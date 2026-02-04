//! A language-agnostic set of models to represent a symbol, which has previously been indexed, and now has been
//! resolved as part of a query.

mod resolved_symbol;
mod score;

pub use resolved_symbol::*;
pub use score::*;
