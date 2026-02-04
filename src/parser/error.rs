use thiserror::Error;
use tree_sitter::LanguageError;

/// Errors that can occur during parsing.
///
/// This enum represents failures encountered when validating input data,
/// setting up parsers, or interpreting symbol aliases in the indexing system.
///
/// Each variant wraps the relevant context or underlying error where applicable.
#[derive(Error, Debug)]
pub enum Error {
    /// The provided URI is invalid.
    ///
    /// This error occurs when a string intended to represent a URI
    /// does not conform to the expected format.
    ///
    /// The wrapped `String` contains the invalid URI value.
    #[error("The provided URI is invalid: {0}")]
    InvalidUri(String),

    /// The provided file could not be opened.
    ///
    /// This error occurs when attempting to read a file from disk
    /// and the operation fails, for example due to the file not existing
    /// or insufficient permissions.
    ///
    /// The wrapped `tokio::io::Error` contains the original I/O failure reason.
    #[error("The provided file could not be opened: {0}")]
    InvalidFile(tokio::io::Error),

    /// Setting the parser failed because the specified language was not valid.
    ///
    /// This occurs when attempting to configure a parser with a language
    /// that is either unsupported or could not be loaded.
    ///
    /// The wrapped `Option<LanguageError>` provides additional diagnostic details
    /// if available.
    #[error("Setting the parser failed as the language was not valid: {0:?}")]
    InvalidLanguage(Option<LanguageError>),

    /// An invalid query was provided.
    ///
    /// This occurs when a query, such as a Tree-sitter query, cannot
    /// be parsed or contains invalid syntax.
    ///
    /// The wrapped `tree_sitter::QueryError` contains details about the parsing failure.
    #[error("Invalid query: {0}")]
    InvalidQuery(tree_sitter::QueryError),
}
