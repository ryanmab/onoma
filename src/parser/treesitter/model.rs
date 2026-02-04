use crate::models;

/// The context in which parsing has begun in.
///
/// Fields defined in here generally help to better inform the parsing behavior, in order
/// to tailor and improve the output.
#[derive(Debug, Default)]
pub struct Context {
    pub(crate) existing_tree: Option<tree_sitter::Tree>,
}

impl Context {
    /// Set an existing tree for the Treesitter query to incrementally update.
    #[allow(dead_code)]
    #[must_use]
    pub fn with_existing_tree(mut self, tree: tree_sitter::Tree) -> Self {
        self.existing_tree = Some(tree);

        self
    }
}

/// The output of the parsed source file.
#[derive(Debug)]
pub struct Output {
    /// The resulting index.
    pub index: models::parsed::Index,

    /// The resulting Treesitter tree, which can be used in subsequent calls to
    /// [`crate::parser::treesitter::Parser`] to improve parsing performance.
    #[allow(dead_code)]
    pub tree: tree_sitter::Tree,
}
