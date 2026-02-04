//! Incremental indexing using [`crate::indexer::Indexer`] and filesystem events.

use std::{path::Path, sync::Arc, time::Duration};

use itertools::Itertools;
use notify::{RecommendedWatcher, RecursiveMode};
use notify_debouncer_mini::{DebouncedEvent, Debouncer, new_debouncer_opt};

mod constant;
mod error;
mod types;

pub use error::Error;
use tokio::{
    sync::{
        Mutex,
        mpsc::{self, Receiver},
    },
    task::JoinHandle,
};
pub use types::Result;

use crate::{
    indexer::Indexer,
    watcher::{self, types::Event},
};

/// Watcher acts as the bridge between the [`Indexer`] and the filesystem.
///
/// More specifically, it watches for file system events and notifies the Indexer to re-index
/// any relevant files, when they have changed.
///
/// This may include:
/// 1. New files
/// 2. Changed files
#[derive(Debug)]
pub struct Watcher<I>
where
    I: Indexer + Send + Sync + 'static,
{
    debouncer: Arc<Mutex<Option<Debouncer<RecommendedWatcher>>>>,
    handle: Arc<Mutex<Option<JoinHandle<()>>>>,
    indexer: Arc<Mutex<I>>,
}

impl<I> Watcher<I>
where
    I: Indexer + Send + Sync + 'static,
{
    /// Initialize a new Watcher for an existing Indexer.
    ///
    /// The watcher _will not_ begin watching for file changes in the indexers
    /// workspaces until [`Watcher::start`] is called.
    #[must_use]
    pub fn new(indexer: I) -> Self {
        Self {
            debouncer: Arc::default(),
            handle: Arc::default(),
            indexer: Arc::new(Mutex::new(indexer)),
        }
    }

    /// Run a full index on all files in the indexer's workspace.
    ///
    /// This generally precedes a call to [`Watcher::start`], which will incrementally
    /// update the index when files change.
    ///
    /// # Errors
    ///
    /// Returns a list of errors for each workspace which could not be successfully indexed.
    pub async fn run_full_index(&self) -> std::result::Result<(), Vec<watcher::Error>> {
        self.indexer
            .lock()
            .await
            .index_workspaces()
            .await
            .map_err(|errors| {
                errors
                    .into_iter()
                    .map(watcher::Error::IndexingFailed)
                    .collect::<Vec<_>>()
            })?;

        Ok(())
    }

    /// Begin watching for file changes in the indexer's workspaces, and trigger a re-index of
    /// any relevant files which have changed.
    ///
    /// # Errors
    ///
    /// Returns an error if the Watcher could not be started. Generally this occurs if the
    /// underlying filesystem event debouncer fails to start.
    pub async fn start(&self) -> Result<()> {
        let (mut rx, debouncer) = self.setup_debouncer().await?;

        *self.debouncer.lock().await = Some(debouncer);

        log::debug!("Watching: {:?}", self.indexer.lock().await.get_workspaces());

        let indexer = Arc::clone(&self.indexer);

        let handle = tokio::spawn(async move {
            while let Some(res) = rx.recv().await {
                match res {
                    Ok(events) => {
                        log::trace!(
                            "New debounced filesystem event received for files: {:?}",
                            events
                                .iter()
                                .map(|event| event.path.as_path())
                                .dedup()
                                .collect::<Vec<&Path>>()
                        );

                        if let Err(e) =
                            Self::on_event(Arc::clone(&indexer), events.into_iter()).await
                        {
                            log::error!("Indexing error: {e:?}");
                        }
                    }
                    Err(e) => log::error!("Watch error: {e:?}"),
                }
            }
        });

        *self.handle.lock().await = Some(handle);

        Ok(())
    }

    /// Stop watching for file changes in the indexer's workspaces.
    ///
    /// At any point, the watcher can be restarted by calling [`Watcher::start`].
    pub async fn stop(&self) {
        let debouncer = self.debouncer.lock().await.take();
        let handle = self.handle.lock().await.take();

        // They'll both be dropped and safely shut down when they go
        // out of scope, but just for verbosity, drop them explicitly
        drop(handle);
        drop(debouncer);

        log::debug!("Watcher stopped");
    }

    /// Process any events received from the debouncer, by triggering the indexer for
    /// all files.
    ///
    /// It is the responsibility of the Indexer to ensure the file is relevant for its
    /// index (i.e. it's a supported programming language, etc.).
    async fn on_event(
        indexer: Arc<Mutex<I>>,
        events: impl IntoIterator<Item = DebouncedEvent> + Send,
    ) -> Result<()> {
        for path in events.into_iter().map(|event| event.path).dedup() {
            match path {
                path if path.exists() && path.is_file() => {
                    log::debug!("Indexing file change: {}", path.display());

                    indexer
                        .lock()
                        .await
                        .index(&path)
                        .await
                        .map_err(watcher::Error::IndexingFailed)?;
                }
                path if !path.exists() => {
                    log::debug!(
                        "Deindexing as the file no longer exists: {}",
                        path.display()
                    );

                    indexer
                        .lock()
                        .await
                        .deindex(&path)
                        .await
                        .map_err(watcher::Error::IndexingFailed)?;
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Setup a debouncer, and configure a channel to receive the debounced events in real-time from the
    /// filesystem.
    #[must_use = "Watcher has no purpose if events are not picked up from the receiver"]
    async fn setup_debouncer(&self) -> Result<(Receiver<Event>, Debouncer<RecommendedWatcher>)> {
        let (tx, rx) = mpsc::channel(1);

        let config = notify_debouncer_mini::Config::with_timeout(
            notify_debouncer_mini::Config::default(),
            Duration::from_secs(constant::DEBOUNCED_EVENT_TIMEOUT_SECS),
        );

        let mut debouncer: Debouncer<RecommendedWatcher> = new_debouncer_opt(config, move |res| {
            if let Err(err) = tx.blocking_send(res) {
                log::error!("Error when trying to notify subscriber of file system event: {err}");
            }
        })
        .map_err(watcher::Error::NotifySetupFailed)?;

        let watcher = debouncer.watcher();

        let workspaces = self.indexer.lock().await.get_workspaces();

        for path in workspaces {
            watcher
                .watch(path.as_ref(), RecursiveMode::Recursive)
                .map_err(watcher::Error::NotifySetupFailed)?;
        }

        Ok((rx, debouncer))
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::{self, File},
        future,
        sync::Arc,
    };

    use tempfile::tempdir;

    use crate::indexer::MockIndexer;

    #[tokio::test]
    async fn test_watcher_new_file_event() {
        let mut mock_indexer = MockIndexer::default();

        let workspace = Arc::new(tempdir().unwrap().keep());
        let workspaces = vec![workspace.clone()];

        mock_indexer
            .expect_get_workspaces()
            .returning(move || workspaces.clone());

        mock_indexer.expect_is_inside_workspace().return_const(true);

        // Should be called to index the foo.txt file only
        mock_indexer
            .expect_index()
            .times(1..)
            .withf(|path| path.ends_with("foo.txt"))
            .returning(|_| Box::pin(future::ready(Ok(()))));

        let watcher = super::Watcher::new(mock_indexer);

        watcher.start().await.expect("Watcher to start");

        fs::create_dir_all(workspace.clone().join("src"))
            .expect("Should always be able to create a test directory");

        File::create(workspace.clone().join("src").join("foo.txt"))
            .expect("Should always be able to create a test file");

        tokio::time::sleep(std::time::Duration::from_secs(3)).await;

        watcher.stop().await;
    }

    #[tokio::test]
    async fn test_watcher_deleting_file_event() {
        let mut mock_indexer = MockIndexer::default();

        let workspace = Arc::new(tempdir().unwrap().keep());
        let workspaces = vec![workspace.clone()];

        mock_indexer
            .expect_get_workspaces()
            .returning(move || workspaces.clone());

        mock_indexer.expect_is_inside_workspace().return_const(true);

        let path = workspace.clone().join("foo.txt");

        File::create(&path).expect("Should always be able to create a test file");

        // Should be called to index the foo.txt file only
        mock_indexer
            .expect_deindex()
            .times(1)
            .withf(|path| path.ends_with("foo.txt"))
            .returning(|_| Box::pin(future::ready(Ok(()))));

        let watcher = super::Watcher::new(mock_indexer);

        watcher.start().await.expect("Watcher to start");

        fs::remove_file(&path).expect("Should always be able to delete a test file");

        tokio::time::sleep(std::time::Duration::from_secs(3)).await;

        watcher.stop().await;
    }
}
