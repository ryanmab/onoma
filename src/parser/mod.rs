//! Low-level tooling for reading source code files and parsing symbols into in-memory models.
//!
//! It is generally not advised to use any parsers directly. Higher level structs are provided in
//! [`crate::indexer`] and [`crate::watcher`] which handle persistence and incremental indexing
//! respectively.
//!
//! ## Supported Languages
//!
//! Supported languages are defined by [`crate::models::parsed::Language`].

mod error;
mod types;

pub mod treesitter;

pub use error::Error;
pub use types::*;
