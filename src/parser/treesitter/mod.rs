//! # Treesitter Parser
//!
//! A low-level wrapper around Treesitter designed to parse symbols into an
//! [`crate::models::parsed::Index`].
//!
//! The parser _does not_ handle persistence (i.e. building an index). For that capability, refer
//! to [`crate::indexer`].

mod model;
mod parser;

pub use model::*;
pub use parser::*;
