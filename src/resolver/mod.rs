//! Tooling for fuzzy matching and scoring symbols from indexes in real-time.

pub(crate) mod constant;
mod database_backed_resolver;
mod scoring;
mod types;
mod utils;
mod weight;

pub use database_backed_resolver::DatabaseBackedResolver;

pub use types::{Context, Resolver};
