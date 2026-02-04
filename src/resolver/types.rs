use std::{fmt::Debug, path::PathBuf};

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg(test)]
use tokio_stream::wrappers::ReceiverStream;

#[cfg(test)]
use crate::models::resolved::ResolvedSymbol;

use crate::models::{self};

/// The Resolver trait defines the core functionality required for resolving
/// semantic symbols from indexed source code, within registered workspaces.
#[cfg_attr(
    test,
    automock(type QueryContext=super::types::Context; type QueryResult=ReceiverStream<ResolvedSymbol>;)
)]
pub trait Resolver: Send + Sync + Debug {
    #[allow(missing_docs)]
    type QueryContext;

    #[allow(missing_docs)]
    type QueryResult;

    /// Run a query against the indexed Symbols.
    ///
    /// It is generally advised that a stream be yielded, backed by an asynchronous
    /// task which resolves symbols from the index just-in-time. But in practice,
    /// the implementation details are left up to the resolver.
    fn query(&self, query: String, ctx: Self::QueryContext) -> Self::QueryResult;
}

/// The context in which a query was executed from.
///
/// Fields defined in here generally help to better inform the scoring logic, in order to more
/// accurately score symbols which are likely to be relevant for a given query.
#[derive(Default, Debug, Clone)]
pub struct Context {
    /// The currently focused file, when the query began.
    ///
    /// This helps influence scoring to favor symbols which are closer to
    /// the current file.
    pub current_file: Option<PathBuf>,

    /// The kinds of symbols which should be returned.
    ///
    /// Queries where the context provides [`Option::None`] or an empty [`Vec`] will return symbols of all kinds.
    pub symbol_kinds: Option<Vec<models::parsed::SymbolKind>>,
}

impl Context {
    /// Set the current file.
    #[must_use]
    pub fn with_current_file(mut self, current_file: PathBuf) -> Self {
        self.current_file = Some(current_file);

        self
    }

    /// Set the symbol kinds.
    #[must_use]
    pub fn with_symbol_kinds(mut self, symbol_kinds: &[models::parsed::SymbolKind]) -> Self {
        self.symbol_kinds = Some(symbol_kinds.to_vec());

        self
    }
}
