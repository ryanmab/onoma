use thiserror::Error;

use crate::indexer;

/// Errors that can occur during indexing or filesystem event handling.
///
/// This enum represents failures that can happen either when setting up
/// filesystem event monitoring (via `notify`) or when performing the
/// actual indexing of source code.
///
/// Each variant wraps the underlying error from the respective subsystem.
#[derive(Error, Debug)]
pub enum Error {
    /// An error occurred while setting up the filesystem debouncer.
    ///
    /// This happens when the underlying filesystem watcher could not be started, and as
    /// such the watcher cannot continue.
    #[error("An error occurred when setting up the debouncer for file system events: {0}")]
    NotifySetupFailed(notify::Error),

    /// An error occurred while indexing a changed file.
    ///
    /// This wraps any error returned from underlying indexer, typically
    /// indicating a failure to read files, parse source code, or update
    /// the index.
    #[error("An error occurred when attempting to run indexing: {0}")]
    IndexingFailed(indexer::Error),

    /// An error occurred while de-indexing a file.
    #[error("An error occurred when attempting to deindex a file: {0}")]
    DeindexingFailed(indexer::Error),
}
