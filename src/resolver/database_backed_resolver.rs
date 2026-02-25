use std::{path::Path, time::Duration};

use itertools::Itertools;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use strum::IntoEnumIterator;
use tokio::sync::mpsc::{self, error::SendTimeoutError};
use tokio_stream::StreamExt;
use tokio_stream::wrappers::ReceiverStream;

use crate::{
    models::{self, resolved::ResolvedSymbol},
    resolver::{
        Context, Resolver, constant,
        scoring::{self, fuzzy_match},
    },
    utils::get_database_path,
};

/// Resolver is a wrapper around an existing index, which allows for querying
/// and scoring of indexed symbols.
#[derive(Debug, Clone)]
pub struct DatabaseBackedResolver {
    pool: sqlx::Pool<sqlx::Sqlite>,
}

impl DatabaseBackedResolver {
    /// Initialize a resolver at a given database path, for a set of workspaces.
    ///
    /// If a [`crate::watcher::Watcher`] or [`crate::indexer::Indexer`] are also running,
    /// both should be provided the same storage path and deterministic iterator of workspaces,
    /// as this ensures the resolver and indexer are connecting to the same underlying
    /// database.
    #[must_use]
    pub fn new<'a, 'b>(
        storage_path: &'b Path,
        workspaces: impl IntoIterator<Item = &'a Path>,
    ) -> Self {
        let database_path = get_database_path(storage_path, workspaces);

        if let Err(e) = std::fs::create_dir_all(storage_path) {
            log::error!(
                "Failed to create storage directory for resolver at {}: {e:?}",
                storage_path.display()
            );
        }

        log::info!(
            "Initializing database for resolver at path: {:?}",
            &database_path
        );

        let options = SqliteConnectOptions::new()
            .create_if_missing(false)
            .filename(database_path)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
            .synchronous(sqlx::sqlite::SqliteSynchronous::Normal);

        let pool = SqlitePoolOptions::new().connect_lazy_with(options);

        Self { pool }
    }
}

impl Resolver for DatabaseBackedResolver {
    type QueryContext = Context;

    type QueryResult = ReceiverStream<ResolvedSymbol>;

    /// Run a query against the indexed Symbols.
    ///
    /// The query will immediately yield a stream, consisting of resolved symbols
    /// streamed from the index just-in-time.
    ///
    /// The stream can be dropped at any time, and the resolver will safely cancel
    /// and shut down the query, even if not all symbols have been returned.
    fn query(&self, query: String, ctx: Self::QueryContext) -> Self::QueryResult {
        let (tx, rx) = mpsc::channel::<ResolvedSymbol>(100);

        let pool = self.pool.clone();

        tokio::spawn(async move {
            log::info!("Executing query: {query}");

            let mut supported_symbols = ctx.symbol_kinds.unwrap_or_default();
            if supported_symbols.is_empty() {
                supported_symbols = models::parsed::SymbolKind::iter().collect();
            }

            let sql_query = format!(
                r"
                SELECT
                    symbol.id,
                    symbol.kind,
                    file.path,
                    symbol.name,
                    symbol.start_line,
                    symbol.end_column,
                    symbol.end_line,
                    symbol.start_column
                FROM symbol
                    JOIN file ON symbol.file_id = file.id
                WHERE
                    1=1
                    AND symbol.kind IN ({})
                ",
                supported_symbols
                    .iter()
                    .map(|kind| format!("\"{kind}\""))
                    .join(",")
            );

            let mut results = sqlx::query_as::<_, ResolvedSymbol>(&sql_query).fetch(&pool);

            let mut count = 0;
            let config = frizbee::Config {
                // NOTE: This range must never be below the length of the query, otherwise
                // frizbee will panic
                max_typos: Some(
                    (u16::try_from(query.len())
                        .expect("Query length should always be at most 16 unsigned integer")
                        / 5)
                    .clamp(0, 4),
                ),
                sort: false,
                scoring: frizbee::Scoring::default(),
            };

            while let Some(result) = results.next().await {
                match result {
                    Ok(mut symbol) => {
                        let fuzzy_matches = fuzzy_match(&query, &symbol, &config);

                        if !query.is_empty() && fuzzy_matches.is_empty() {
                            // The symbol didn't fuzzy match the query, meaning we can stop here.
                            continue;
                        }

                        symbol.score = scoring::calculate_score(
                            &symbol,
                            fuzzy_matches.iter(),
                            ctx.current_file.as_deref(),
                        )
                        .into();

                        if *symbol.score < constant::DEFAULT_SCORE {
                            // The symbol's score is less than the score it started with. This
                            // indicates that it incurred more penalties than it did bonuses. As
                            // such, it's likely not a good match.
                            //
                            // NB: There is a tradeoff here - in that, a score with penalties
                            // _might_ still be something a user will want to see. If we find
                            // there's a lot of "missing" symbols, reevaluating the way in which
                            // symbols are filtered out of results here would be a good start.
                            continue;
                        }

                        // Maintaining a timeout here allows for channels to naturally be closed
                        // fairly quickly in times of congestion (when many queries are started
                        // in quick succession). This is important for sqlx, as it has only a small
                        // number of open connections in its pool, and needlessly waiting for a
                        // send to complete here can _easily_ exhaust the available connections, and
                        // starve newer queries.
                        if let Err(e) = tx
                            .send_timeout(
                                symbol,
                                Duration::from_secs(constant::RESOLVER_SEND_TIMEOUT_SECS),
                            )
                            .await
                        {
                            match e {
                                SendTimeoutError::Closed(_) => {
                                    log::warn!(
                                        "Receiving side of the stream is closed (i.e. no longer waiting for additional symbols), stopping task.",
                                    );
                                }
                                SendTimeoutError::Timeout(e) => {
                                    log::error!(
                                        "Receiving side of the stream was full and sender timed out before delivering symbol: {e:?}"
                                    );
                                }
                            }

                            break;
                        }

                        // Symbol returned and the send was successful - we're good to continue
                        // on.
                        count += 1;
                    }
                    Err(e) => {
                        log::error!("Error returned from query listing matching symbols: {e}",);
                    }
                }
            }

            log::info!(
                "Returned {count} symbols (until no other symbols left, or stream no longer open)."
            );
        });

        ReceiverStream::new(rx)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use insta::assert_json_snapshot;
    use tempfile::tempdir;
    use tokio_stream::StreamExt;

    use crate::{
        indexer::{self, Indexer},
        models::{self, parsed::SymbolKind},
        resolver::Resolver,
    };

    #[tokio::test]
    pub async fn test_resolving_symbols_with_query() {
        let storage_path = tempdir()
            .expect("Should never fail when creating a temporary path for testing indexing");

        let fixutes = PathBuf::from("tests/fixtures/");

        let workspaces = vec![fixutes.as_path()];

        let indexer = indexer::DatabaseBackedIndexer::new(storage_path.path(), workspaces.clone())
            .await
            .expect("Should be able to create the empty index");

        let resolver = super::DatabaseBackedResolver::new(storage_path.path(), workspaces.clone());

        assert!(indexer.index_workspaces().await.is_ok());

        let mut resolved_symbols: Vec<models::resolved::ResolvedSymbol> = resolver
            .query(String::from("func"), super::Context::default())
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
    pub async fn test_resolving_symbols_of_specific_type() {
        let storage_path = tempdir()
            .expect("Should never fail when creating a temporary path for testing indexing");

        let fixutes = PathBuf::from("tests/fixtures/");

        let workspaces = vec![fixutes.as_path()];

        let indexer = indexer::DatabaseBackedIndexer::new(storage_path.path(), workspaces.clone())
            .await
            .expect("Should be able to create the empty index");

        let resolver = super::DatabaseBackedResolver::new(storage_path.path(), workspaces.clone());

        assert!(indexer.index_workspaces().await.is_ok());

        let mut resolved_symbols: Vec<models::resolved::ResolvedSymbol> = resolver
            .query(
                String::new(),
                super::Context::default()
                    .with_symbol_kinds(&[SymbolKind::Function, SymbolKind::Method]),
            )
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
