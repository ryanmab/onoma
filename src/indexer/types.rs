use std::{
    fmt::Debug,
    path::{Path, PathBuf},
    sync::Arc,
};

#[cfg(test)]
use mockall::{automock, predicate::*};

use crate::indexer;

#[allow(missing_docs)]
pub type Result<T> = std::result::Result<T, indexer::Error>;

/// The Indexer trait defines the core functionality required for indexing
/// source code files within registered workspaces.
#[cfg_attr(test, automock)]
pub trait Indexer: Send + Sync + Debug {
    /// Get the list of workspaces currently registered with the indexer.
    fn get_workspaces(&self) -> Vec<Arc<PathBuf>>;

    /// Check if a given path is inside one of the indexer's registered
    /// workspaces.
    fn is_inside_workspace(&self, path: &Path) -> bool;

    /// Index all workspaces registered with the indexer.
    fn index_workspaces(
        &self,
    ) -> impl Future<Output = std::result::Result<(), Vec<indexer::Error>>> + Send;

    /// Index a particular file, or folder, inside a workspace.
    ///
    /// # Errors
    ///
    /// Returns an error if the folder could not be successfully indexed.
    fn index(&self, path: &Path) -> impl Future<Output = Result<()>> + Send;

    /// De-index a particular file, or folder, in a workspace.
    ///
    /// Usually, this is necessary when a previously indexed file is deleted.
    ///
    /// # Errors
    ///
    /// Returns an error if the file could not be de-indexed successfully.
    fn deindex(&self, path: &Path) -> impl Future<Output = Result<()>> + Send;
}
