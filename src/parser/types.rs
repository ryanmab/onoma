use crate::parser;

use std::{future::Future, path::Path};

/// A parser for reading source code and constructing symbol indexes ([`crate::models::parsed::Index`]).
pub trait Parser: Default + Send + Sync + Clone {
    #[allow(missing_docs)]
    type ParseContext;

    #[allow(missing_docs)]
    type ParseOutput;

    /// Create an index from a source file.
    ///
    /// The way in which parsing occurs, and the context required to perform parsing efficiently
    /// will depend on the underlying parser which is used.
    fn parse(
        &self,
        file: &Path,
        ctx: &Self::ParseContext,
    ) -> impl Future<Output = Result<Self::ParseOutput>> + Send;
}

#[allow(missing_docs)]
#[doc(hidden)]
pub type Result<T> = std::result::Result<T, parser::Error>;
