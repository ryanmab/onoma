use std::hash::Hash;

use crate::models::{self, parsed::SymbolRole};

/// A symbol which has been parsed out of a source file.
///
/// Generally speaking, parsing will have likely happened through a parser
/// ([`crate::parser`]).
#[derive(Debug, Eq, PartialOrd, Ord)]
pub struct Symbol {
    /// The kind of symbol.
    pub kind: models::parsed::SymbolKind,

    /// The name of the symbol.
    ///
    /// Exactly what the name is will depend on what kind of symbol it is.
    pub name: String,

    /// The occurrence of the definition of this symbol in the source files.
    pub definition: Option<models::parsed::Occurrence>,

    /// The occurrences in different source files of this symbol.
    pub occurrences: Vec<models::parsed::Occurrence>,
}

impl Symbol {
    /// Create a new symbol, of a particular kind.
    ///
    /// This symbol will not yet have any occurrences.
    #[must_use]
    pub fn new(kind: models::parsed::SymbolKind, name: &str) -> Self {
        Self {
            kind,
            name: name.to_string(),
            occurrences: Vec::default(),
            definition: None,
        }
    }

    /// Append one of more occurrences from different source files of the symbol.
    pub fn add_occurrence(&mut self, occurrence: models::parsed::Occurrence) {
        if self.definition.is_none() && occurrence.roles.contains(&SymbolRole::Definition) {
            let _ = self.definition.insert(occurrence);

            return;
        }

        self.occurrences.push(occurrence);
    }
}

impl Hash for Symbol {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Don't check the symbol kind here, as its possible we'll see duplicate symbols of
        // different kinds (i.e. method vs function)

        self.name.hash(state);

        if let Some(definition) = &self.definition {
            definition.hash(state);
        } else {
            self.occurrences.hash(state);
        }
    }
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        // Don't check the symbol kind here, as its possible we'll see duplicate symbols of
        // different kinds (i.e. method vs function)

        if let Some(self_definition) = &self.definition
            && let Some(other_definition) = &other.definition
        {
            self.name == other.name && self_definition == other_definition
        } else {
            self.name == other.name && self.occurrences == other.occurrences
        }
    }
}
