use std::{
    collections::HashMap,
    fmt::Debug,
    path::{Path, PathBuf},
    sync::Arc,
};

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg(test)]
use tokio_stream::wrappers::ReceiverStream;

#[cfg(test)]
use crate::models::resolved::ResolvedSymbol;

use crate::models::{self};

/// Specifies which symbol kinds should be included when executing a query.
///
/// This acts as a filter over indexed symbols, allowing queries to either:
///
/// - restrict to a fixed set applied globally, or
/// - restrict per language.
#[derive(Debug)]
pub enum SymbolKindFilter {
    /// Restrict results to the given symbol kinds, regardless of language.
    ///
    /// This applies the same filter across all indexed files.
    Global(Vec<models::parsed::SymbolKind>),

    /// Restrict results to symbol kinds based on the language the symbol
    /// is defined in.
    ///
    /// For symbols defined in a language not present in the map, no
    /// language-specific filtering is applied.
    PerLanguage(HashMap<models::parsed::Language, Vec<models::parsed::SymbolKind>>),
}

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
    pub current_file: Arc<Option<PathBuf>>,

    /// The kinds of symbols which should be returned from a query.
    ///
    /// Queries where the context provides [`Option::None`] will return symbols of any kind.
    pub symbol_kinds: Arc<Option<SymbolKindFilter>>,
}

impl Context {
    /// Set the current file.
    #[must_use]
    pub fn with_current_file(mut self, current_file: impl AsRef<Path>) -> Self {
        self.current_file = Arc::new(Some(current_file.as_ref().into()));

        self
    }

    /// Set the symbol kinds.
    #[must_use]
    pub fn with_symbol_kinds(mut self, symbol_kinds: SymbolKindFilter) -> Self {
        self.symbol_kinds = Arc::new(Some(symbol_kinds));

        self
    }
}
