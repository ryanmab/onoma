use std::collections::HashSet;

use crate::models;

/// A type of parser which can create indexes.
#[derive(Debug)]
pub enum Type {
    /// Treesitter (see [`crate::parser::treesitter::Parser`]) was
    /// used to construct the index.
    TreeSitter,
}

/// An index of symbols parsed from source files which represent all the
/// possible items which can be queried and resolved.
#[derive(Debug)]
pub struct Index {
    /// The type of parser used to create the index.
    pub r#type: models::parsed::Type,

    /// The symbols contained in the index.
    pub symbols: HashSet<models::parsed::Symbol>,
}

impl Index {
    /// Create an empty index.
    #[must_use]
    pub fn new(r#type: models::parsed::Type) -> Self {
        Self {
            r#type,
            symbols: HashSet::new(),
        }
    }

    /// Append a symbol to an index.
    pub fn append_symbol(&mut self, symbol: models::parsed::Symbol) {
        self.symbols.insert(symbol);
    }
}
