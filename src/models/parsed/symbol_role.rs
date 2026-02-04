use std::ops::Deref;

/// A particular role an [`crate::models::parsed::Occurrence`] has in a particular
/// [`crate::models::parsed::Symbol`].
///
/// For example, a particular occurrence might represent the definition of a
/// symbol, a reference, or something else.
#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum SymbolRole {
    /// The occurrence is where the Symbol was defined.
    Definition,

    /// A catch-all for any roles not yet promoted to first-class roles.
    Other(String),
}

/// Zero or more roles that an occurrence has for a particular symbol.
#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct Roles(pub Vec<SymbolRole>);

impl Deref for Roles {
    type Target = Vec<SymbolRole>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
