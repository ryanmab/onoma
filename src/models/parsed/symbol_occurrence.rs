use std::path::{Path, PathBuf};

use crate::models::{self};

/// An occurrence of a particular symbol in a given file.
///
/// This represents a place in which a symbol was used, what it's
/// role in the larger file was (i.e. a definition, reference, etc.),
/// and the range of characters which enclose the occurrence.
///
/// This method of modelling can be used to represent a particular symbol which
/// was defined in one file (that would be an occurrence), and then referenced
/// in two other places (that would be two more occurrences, with particular roles).
///
/// This model was inspired by the SCIP (SCIP Indexing Protocol) `Occurrence` enum: <https://github.com/sourcegraph/scip/blob/main/scip.proto#L633>
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Occurrence {
    /// The language the occurrence was defined in.
    pub language: models::parsed::Language,

    /// The path to the file which contains the occurrence of the symbol.
    pub absolute_path: PathBuf,

    /// The range which encloses the occurrence.
    pub range: models::parsed::Range,

    /// The roles this occurrence acts in. For example, a definition, reference, etc.
    pub roles: models::parsed::Roles,
}

impl Occurrence {
    /// Create a new occurrence of a Symbol in a particular source file.
    #[must_use]
    pub fn new(
        language: models::parsed::Language,
        absolute_path: &Path,
        range: models::parsed::Range,
        symbol_roles: models::parsed::Roles,
    ) -> Self {
        Self {
            language,
            absolute_path: absolute_path.to_path_buf(),
            range,
            roles: symbol_roles,
        }
    }
}
