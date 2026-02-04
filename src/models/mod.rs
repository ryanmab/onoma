//! A set of in-memory, and language-agnostic, data models which govern the full end-to-end flow of a symbol.
//!
//! Parsed models represent the period of time _before_ symbols have been indexed. At this point,
//! symbol information has been read out of source files, but is not necessarily available to the
//! end user to query (resolve) yet.
//!
//! Resolved models represent everything _after_ a symbol has been resolved from the index. In
//! other words, this is the period of time where a source file's symbols have been persisted in an
//! index, and have subsequently been queried by the end user and returned by the resolver.

pub mod parsed;
pub mod resolved;
