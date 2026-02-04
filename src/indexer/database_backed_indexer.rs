use crate::{
    indexer::{self, Error, Indexer, types},
    models::parsed::{FileExtension, Language},
    parser::{self, Parser},
    utils::get_database_path,
};
use itertools::Itertools;
use sqlx::sqlite::SqliteConnectOptions;
use std::{
    iter,
    path::{Path, PathBuf},
    sync::Arc,
};
use strum::IntoEnumIterator;
use tokio::task::JoinSet;
use types::Result;

/// Indexer acts as the layer around the language-agnostic models ([`crate::models`]),
/// and stores resulting data in an underlying data store.
///
/// This database can then be consumed by a [`crate::resolver::Resolver`] to run queries
/// and scoring on.
///
/// In all likelihood, an indexer _should not_ be used or called directly. Instead,
/// a [`crate::watcher::Watcher`] should be used to orchestrate incremental updates to an
/// index using an indexer automatically using filesystem events.
#[derive(Debug, Clone)]
pub struct DatabaseBackedIndexer {
    #[allow(dead_code)]
    database_path: PathBuf,
    workspaces: Vec<Arc<PathBuf>>,
    pool: sqlx::Pool<sqlx::Sqlite>,
    parser: parser::treesitter::Parser,
}

impl DatabaseBackedIndexer {
    /// Initialize an indexer at a given database path, for a set of workspaces.
    ///
    /// If a [`crate::resolver::Resolver`] is also running, both should be provided
    /// the same storage path and deterministic iterator of workspaces, as this ensures
    /// the resolver and indexer are connecting to the same underlying database.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying database cannot be initialized successfully,
    /// usually as the result of being unable to setup the tables correctly.
    pub async fn new<'a, 'b>(
        storage_path: &'b Path,
        workspaces: impl IntoIterator<Item = &'a Path> + Clone,
    ) -> Result<Self> {
        let (database_path, pool) =
            Self::initialise_database(storage_path, workspaces.clone()).await?;

        let indexer = Self {
            database_path: PathBuf::from(&database_path),
            pool,
            workspaces: workspaces
                .into_iter()
                .map(Path::to_path_buf)
                .map(Arc::new)
                .collect_vec(),
            parser: parser::treesitter::Parser::default(),
        };

        Ok(indexer)
    }

    /// Initialize the database for the given workspaces, in a particular path.
    ///
    /// This will create the database (if it does not already exist), as well as
    /// running any migrations necessary to put the database into the correct state,
    /// before indexing begins.
    async fn initialise_database<'a, 'b>(
        storage_path: &'b Path,
        workspaces: impl IntoIterator<Item = &'a Path>,
    ) -> Result<(PathBuf, sqlx::Pool<sqlx::Sqlite>)> {
        let database_path = get_database_path(storage_path, workspaces);

        if let Err(e) = std::fs::create_dir_all(storage_path) {
            return Err(indexer::Error::DatabaseFileError(
                storage_path.to_path_buf(),
                e,
            ));
        }

        log::info!(
            "Initializing database for indexer at path: {:?}",
            &database_path
        );

        let options = SqliteConnectOptions::new()
            .create_if_missing(true)
            .filename(&database_path)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
            .synchronous(sqlx::sqlite::SqliteSynchronous::Normal);

        let pool = sqlx::Pool::connect_lazy_with(options);

        sqlx::migrate!()
            .run(&pool)
            .await
            .map_err(indexer::Error::MigrationFailed)?;

        Ok((PathBuf::from(database_path), pool))
    }

    /// Index a particular file in a workspace.
    ///
    /// # Errors
    ///
    /// Returns an error if the file could not be indexed successfully.
    async fn index_file(&self, path: &Path) -> Result<()> {
        if !path.exists() {
            return Err(Error::InvalidPath(
                path.to_path_buf(),
                "File does not exist".into(),
            ));
        }

        if !path.is_file() {
            return Err(Error::InvalidPath(
                path.to_path_buf(),
                "Path is not a file".into(),
            ));
        }

        if !self.is_inside_workspace(path) {
            return Err(Error::InvalidPath(
                path.to_path_buf(),
                "File is not inside any registered workspace".into(),
            ));
        }

        let parser::treesitter::Output { index, .. } = self
            .parser
            .parse(path, &parser::treesitter::Context::default())
            .await
            .map_err(Error::ParsingFailed)?;

        log::info!("Parsed file: {}", path.display());
        let now = chrono::Utc::now();

        let mut transaction = self
            .pool
            .begin()
            .await
            .map_err(indexer::Error::QueryFailed)?;

        let file_id: i64 = {
            let path = path.to_string_lossy();

            sqlx::query_scalar!(
                r#"
                    INSERT INTO file (
                        path,
                        indexed_at
                    )
                    VALUES (?, ?)
                    ON CONFLICT(path) DO UPDATE SET indexed_at = excluded.indexed_at
                    RETURNING id
                    "#,
                path,
                now
            )
            .fetch_one(&mut *transaction)
            .await
            .map_err(Error::QueryFailed)?
        };

        // Remove all the old symbols, before persisting all the current symbols
        sqlx::query!(
            r#"
                DELETE FROM symbol WHERE file_id = ?
                "#,
            file_id
        )
        .execute(&mut *transaction)
        .await
        .map_err(indexer::Error::QueryFailed)?;

        for symbol in index.symbols {
            let Some(definition) = symbol.definition else {
                log::warn!("Symbol {} has no definition, skipping", symbol.name);

                continue;
            };

            if path != definition.absolute_path {
                log::warn!(
                    "Symbol {} was defined in {}, but indexing only occurring for {}, skipping",
                    symbol.name,
                    definition.absolute_path.display(),
                    path.display()
                );
            }

            log::info!(
                "Persisting {} found in {}.",
                symbol.name,
                definition.absolute_path.display(),
            );

            let range = &definition.range;

            let start_line: i32 = i32::try_from(range.start_line)
                .map_err(|_| indexer::Error::InvalidRange(range.clone()))?;
            let start_column: i32 = i32::try_from(range.start_column)
                .map_err(|_| indexer::Error::InvalidRange(range.clone()))?;

            let end_line: i32 = i32::try_from(range.end_line)
                .map_err(|_| indexer::Error::InvalidRange(range.clone()))?;
            let end_column: i32 = i32::try_from(range.end_column)
                .map_err(|_| indexer::Error::InvalidRange(range.clone()))?;

            // Create new symbols
            sqlx::query!(
                r#"
                    INSERT INTO symbol (
                        kind,
                        name,
                        file_id,
                        start_line,
                        start_column,
                        end_line,
                        end_column,
                        indexed_at
                    )
                    VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                    "#,
                symbol.kind,
                symbol.name,
                file_id,
                start_line,
                start_column,
                end_line,
                end_column,
                now
            )
            .execute(&mut *transaction)
            .await
            .map_err(Error::QueryFailed)?;
        }

        // TODO: File bloom filter here?
        transaction
            .commit()
            .await
            .map_err(indexer::Error::QueryFailed)?;

        Ok(())
    }
}

impl Indexer for DatabaseBackedIndexer {
    /// Get the list of workspaces currently being managed by the indexer.
    fn get_workspaces(&self) -> Vec<Arc<PathBuf>> {
        self.workspaces.clone()
    }

    fn is_inside_workspace(&self, path: &Path) -> bool {
        self.workspaces
            .iter()
            .any(|workspace| path.starts_with(workspace.as_ref()))
    }

    /// Run indexing on all relevant files in all workspaces.
    ///
    /// # Errors
    ///
    /// Returns a list of errors for each workspace which could not be successfully indexed.
    async fn index_workspaces(&self) -> std::result::Result<(), Vec<indexer::Error>> {
        let mut errors = vec![];
        for workspace in &*self.workspaces {
            // TODO: For indexes that already exist this will prove to be inefficient. We should
            // hash the file content and only the parts of the workspace which have not changed.
            // Currently, this will fully re-index the workspace even if no files have changed.
            if let Err(e) = self.index(workspace.as_path()).await {
                errors.push(e);
            }
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(())
    }

    /// Index a particular file, or folder, inside a workspace.
    ///
    /// # Errors
    ///
    /// Returns an error if the folder could not be successfully indexed.
    async fn index(&self, path: &Path) -> Result<()> {
        if !path.exists() {
            return Err(Error::InvalidPath(
                path.to_path_buf(),
                "Path does not exist".into(),
            ));
        }

        if !self.is_inside_workspace(path) {
            return Err(Error::InvalidPath(
                path.to_path_buf(),
                "Path is not inside any registered workspace".into(),
            ));
        }

        let files: Box<dyn Iterator<Item = std::result::Result<PathBuf, _>> + Send> =
            if path.is_dir() {
                // If it's a directory, we need to walk the directory and find all relevant files to
                // index, based on the supported file extensions
                let mut types = ignore::types::TypesBuilder::new();
                for language in Language::iter() {
                    let file_extension = &*FileExtension::from(language);

                    if let Err(e) = types.add(file_extension, &format!("*.{file_extension}")) {
                        log::error!(
                            "File extension ({file_extension}) could not be added to indexer: {e}"
                        );

                        continue;
                    }

                    types.select(file_extension);
                }
                let types = types.build().expect("Failed to build ignore types");

                let walker = ignore::WalkBuilder::new(path)
                    .types(types)
                    .git_ignore(true)
                    .git_exclude(true)
                    .build();

                Box::new(walker.into_iter().filter_map(|entry| match entry {
                    Ok(entry) => {
                        if entry.metadata().map(|m| m.is_file()).unwrap_or(false) {
                            Some(Ok(entry.into_path()))
                        } else {
                            None
                        }
                    }
                    Err(e) => Some(Err(e)),
                }))
            } else {
                // If it's a file, we can short-circuit and just index that single file
                Box::new(iter::once(Ok(path.to_path_buf())))
            };

        let mut tasks = JoinSet::<()>::new();

        for result in files {
            match result {
                Ok(entry) => {
                    let indexer = self.clone();

                    tasks.spawn(async move {
                        if let Err(e) = indexer.index_file(entry.as_path()).await {
                            log::error!("Error indexing file {}: {e:?}", entry.display());
                        }
                    });
                }
                Err(e) => {
                    log::error!("Error while walking project directory: {e:?}");
                }
            }
        }

        tasks.join_all().await;

        Ok(())
    }

    /// De-index a particular file, or folder, in a workspace.
    ///
    /// Usually, this is necessary when a previously indexed file is deleted.
    ///
    /// # Errors
    ///
    /// Returns an error if the file could not be de-indexed successfully.
    async fn deindex(&self, path: &Path) -> Result<()> {
        let path_pattern = format!("{}%", path.display());

        // Removing the file will trigger a removal of any associated symbols as the FK
        // is set to cascade delete
        sqlx::query!(r#"DELETE FROM file WHERE path LIKE ?"#, path_pattern)
            .execute(&self.pool)
            .await
            .map_err(indexer::Error::QueryFailed)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use insta::assert_json_snapshot;
    use tempfile::tempdir;
    use tokio_stream::StreamExt;

    use crate::{
        indexer::Indexer,
        models,
        resolver::{self, Resolver},
    };

    #[tokio::test]
    pub async fn test_indexing_project() {
        let storage_path = tempdir()
            .expect("Should never fail when creating a temporary path for testing indexing");

        let fixtures = PathBuf::from("tests/fixtures/");

        let workspaces = vec![fixtures.as_path()];

        let indexer = super::DatabaseBackedIndexer::new(storage_path.path(), workspaces.clone())
            .await
            .expect("Should be able to create the empty index");

        let resolver =
            resolver::DatabaseBackedResolver::new(storage_path.path(), workspaces.clone());

        assert!(indexer.index_workspaces().await.is_ok());

        let mut resolved_symbols: Vec<models::resolved::ResolvedSymbol> = resolver
            .query(String::new(), resolver::Context::default())
            .collect()
            .await;

        // The order of symbols is not guaranteed, so we need the sort symbols to keep the
        // snapshot predictable
        resolved_symbols.sort_unstable();

        assert_json_snapshot!(
            resolved_symbols,
            {"[].id" => 0} // IDs are non-deterministic, so just blank them out
        );
    }

    #[tokio::test]
    pub async fn test_deindexing_files_in_project() {
        let storage_path = tempdir()
            .expect("Should never fail when creating a temporary path for testing indexing");

        let fixtures = PathBuf::from("tests/fixtures/");

        let workspaces = vec![fixtures.as_path()];

        let indexer = super::DatabaseBackedIndexer::new(storage_path.path(), workspaces.clone())
            .await
            .expect("Should be able to create the empty index");

        let resolver =
            resolver::DatabaseBackedResolver::new(storage_path.path(), workspaces.clone());

        assert!(indexer.index_workspaces().await.is_ok());

        // Remove the Go symbols
        assert!(
            indexer
                .deindex(fixtures.join("go.go").as_path())
                .await
                .is_ok()
        );

        let mut resolved_symbols: Vec<models::resolved::ResolvedSymbol> = resolver
            .query(String::new(), resolver::Context::default())
            .collect()
            .await;

        // The order of symbols is not guaranteed, so we need the sort symbols to keep the
        // snapshot predictable
        resolved_symbols.sort_unstable();

        assert_json_snapshot!(
            resolved_symbols,
            {"[].id" => 0} // IDs are non-deterministic, so just blank them out
        );
    }
}
