use std::path::PathBuf;

use thiserror::Error;

use crate::{
    models,
    parser::{self},
};

/// Errors that can occur during file indexing, parsing, and database operations.
///
/// This enum represents failures encountered when creating index files,
/// validating workspace paths, parsing source code, or interacting with the
/// database. Each variant wraps the relevant context or underlying error.
#[derive(Error, Debug)]
pub enum Error {
    /// Failed to create the parent directory for the index file.
    ///
    /// This error occurs when the specified parent path could not be created
    /// due to filesystem errors, such as permission issues or missing directories.
    ///
    /// - `PathBuf` contains the parent path that could not be created.
    /// - `std::io::Error` provides the underlying I/O error.
    #[error("Unable to create parent path ({0}) for index file: {1}")]
    DatabaseFileError(PathBuf, std::io::Error),

    /// The provided workspace path is not valid.
    ///
    /// This occurs when a path intended as a workspace root is malformed,
    /// does not exist, or fails application-specific validation rules.
    ///
    /// - `PathBuf` contains the invalid path.
    /// - `String` describes why the path is invalid.
    #[error("Provided file path ({0}) was not valid: {1}")]
    InvalidPath(PathBuf, String),

    /// Parsing failed while indexing a file.
    ///
    /// This occurs when the parser encounters a syntax error or other
    /// failure while processing the source file.
    ///
    /// The wrapped `parser::Error` provides detailed diagnostic information.
    #[error("Parsing error occurred while indexing file: {0:?}")]
    ParsingFailed(parser::Error),

    /// A database error occurred during indexing.
    ///
    /// This can happen while inserting, updating, or querying the index
    /// database. The wrapped `sqlx::Error` contains the underlying SQL error.
    #[error("Database error occurred during indexing: {0}")]
    QueryFailed(#[from] sqlx::Error),

    /// Database migration failed.
    ///
    /// This occurs when applying migrations to the index database fails.
    /// The wrapped `sqlx::migrate::MigrateError` provides the failure details.
    #[error("Database migration failed: {0}")]
    MigrationFailed(#[from] sqlx::migrate::MigrateError),

    /// The provided range is invalid.
    ///
    /// This occurs when a `models::parsed::Range` object does not satisfy
    /// expected constraints, such as start > end or out-of-bounds positions.
    #[error("The provided range is invalid: {0:?}")]
    InvalidRange(models::parsed::Range),
}
