//! Low-level tooling for managing in-memory indexes using a persistent database.
//!
//! This _does not_ handle incremental updates, such as when files change. For that
//! capability, refer to [`crate::watcher`].

mod database_backed_indexer;
mod error;
mod types;

pub use database_backed_indexer::DatabaseBackedIndexer;
pub use error::Error;
pub use types::*;
