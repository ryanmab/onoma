use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::models;

/// A resolved symbol is a symbol which can been indexed previously (by [`crate::indexer::Indexer`])
/// and has now been matched to a given query by the Resolver.
#[derive(Debug, sqlx::FromRow, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResolvedSymbol {
    /// The ID of the symbol.
    ///
    /// This is a unique ID for the symbol in the index, at the point it was indexed.
    ///
    /// However, it _cannot_ be guarantee that when re-indexed the symbol will continue
    /// to be allocated the same ID.
    pub id: i64,

    /// The name of the symbol.
    ///
    /// The contents of this field will depend on the kind of symbol.
    ///
    /// I.e. for a function, this value will be its name. For a self parameter of a function will
    /// be the language-specific syntax for the self field (i.e. for Rust: `&mut self`, etc.).
    pub name: String,

    /// The kind of symbol.
    ///
    /// The symbol kinds is long (and non-exhaustive), but in practice these depend on the language
    /// the symbol was indexed from.
    ///
    /// For example, it's unlikely a [`models::parsed::SymbolKind::Lemma`] will be returned when indexing a
    /// Rust project.
    pub kind: models::parsed::SymbolKind,

    /// The path to the file which contains the symbol.
    #[sqlx[try_from = "String"]]
    pub path: PathBuf,

    /// The score is calculated just-in-time by the Resolver and represents a numerical value how
    /// good a match the resolved symbol is for query.
    ///
    /// For information on how the score is calculated, see [`crate::resolver::Resolver::query`].
    #[sqlx(default)]
    #[sqlx[try_from = "i64"]]
    pub score: models::resolved::Score,

    /// The start line for the definition of the symbol.
    ///
    /// This matches how editors generally refer to lines, and so starts from 1.
    pub start_line: i64,

    /// The end line for the definition of the symbol.
    ///
    /// For symbols which are defined on only a single line, this will be equal to
    /// [`ResolvedSymbol::start_line`].
    ///
    /// This matches how editors generally refer to lines, and so starts from 1.
    pub end_line: i64,

    /// The start column (character) for the definition of the symbol, relative to the
    /// [`ResolvedSymbol::start_line`].
    ///
    /// This matches how editors generally refer to columns (characters), and so starts from 1.
    pub start_column: i64,

    /// The end column (character) for the definition of the symbol, relative to the
    /// [`ResolvedSymbol::end_line`].
    ///
    /// This matches how editors generally refer to columns (characters), and so starts from 1.
    pub end_column: i64,
}

impl PartialOrd for ResolvedSymbol {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ResolvedSymbol {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.path.cmp(&other.path) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        match self.name.cmp(&other.name) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.start_line.cmp(&other.start_line)
    }
}
