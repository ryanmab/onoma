/// A start and end position in a source file.
///
/// A range may span one or more lines.
#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct Range {
    /// The start line for the definition of the symbol.
    ///
    /// This matches how editors generally refer to lines, and so starts from 1.
    pub start_line: usize,

    /// The end line for the definition of the symbol.
    ///
    /// For symbols which are defined on only a single line, this will be equal to
    /// [`Range::start_line`].
    ///
    /// This matches how editors generally refer to lines, and so starts from 1.
    pub end_line: usize,

    /// The start column (character) for the definition of the symbol, relative to the
    /// [`Range::start_line`].
    ///
    /// This matches how editors generally refer to columns (characters), and so starts from 1.
    pub start_column: usize,

    /// The end column (character) for the definition of the symbol, relative to the
    /// [`Range::end_line`].
    ///
    /// This matches how editors generally refer to columns (characters), and so starts from 1.
    pub end_column: usize,
}

impl Range {
    /// Create a new range.
    #[must_use]
    pub const fn new(
        start_line: usize,
        end_line: usize,
        start_column: usize,
        end_column: usize,
    ) -> Self {
        Self {
            start_line,
            end_line,
            start_column,
            end_column,
        }
    }
}
