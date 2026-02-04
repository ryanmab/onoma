use std::path::{MAIN_SEPARATOR, Path};

use sha2::{Digest, Sha256};

/// Generate a unique database name for a given list of workspaces.
///
/// In practice, this will return a unique SHA256 hash comprised of all the workspaces, in the
/// order they are yielded from the iterator.
///
/// This means:
/// 1. The order in which workspaces are yielded _MUST_ be deterministic (i.e. the same every
///    time), otherwise a different name will be returned on each call.
/// 2. Generating a name using a subset of workspaces will yield a different result to that of
///    the larger set.
#[must_use]
pub fn generate_unique_db_name<'a>(workspaces: impl IntoIterator<Item = &'a Path>) -> String {
    let mut hasher = Sha256::new();

    for workspace in workspaces {
        hasher.update(workspace.to_str().unwrap_or_default().as_bytes());
    }

    hex::encode(hasher.finalize())
}

/// Allocate a database path, in a specific location, for a given set of workspaces.
///
/// In practice, the database name will be a unique SHA256 hash comprised of all the workspaces, in the
/// order they are yielded from the iterator.
///
/// This means:
/// 1. The order in which workspaces are yielded _MUST_ be deterministic (i.e. the same every
///    time), otherwise a different name will be returned on each call.
/// 2. Generating a name using a subset of workspaces will yield a different result to that of
///    the larger set.
#[must_use]
pub fn get_database_path<'a, 'b>(
    storage_path: &'b Path,
    workspaces: impl IntoIterator<Item = &'a Path>,
) -> String {
    format!(
        "{}{}{}.db",
        storage_path
            .to_string_lossy()
            .trim_end_matches(MAIN_SEPARATOR),
        MAIN_SEPARATOR,
        generate_unique_db_name(workspaces)
    )
}

/// A helper to normalise symbol names during parsing.
///
/// This is particularly useful for normalising between Windows and Unix systems for snapshot
/// testing.
#[must_use]
pub fn normalise_symbol_name(name: &str) -> String {
    name
        // Treesitter parsed symbols commonly include whitespace, so that can all be trimmed
        // out.
        .trim()
        // This helps us standardise between Windows and Unix by matching
        // all carriage returns into a single format. Particularly good for snapshot
        // testing
        .replace("\r\n", "\n")
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use std::path::{MAIN_SEPARATOR, PathBuf};

    #[test]
    pub fn test_database_path_handles_trailing_slashes() {
        let path_1 = PathBuf::from("/some/workspace/1".to_string());
        let path_2 = PathBuf::from("/some/workspace/2".to_string());

        let workspaces = [path_1, path_2];

        let sep = MAIN_SEPARATOR;
        let expected_output = format!(
            "{sep}some{sep}path{sep}trailing{sep}slash{sep}06932fd30d07d2130b10e019489a6609900ab13bf91b87b7db5b1da256b3c75c.db"
        );
        assert_eq!(
            expected_output,
            super::get_database_path(
                PathBuf::from(format!("{sep}some{sep}path{sep}trailing{sep}slash{sep}")).as_path(),
                workspaces.iter().map(PathBuf::as_path)
            )
        );
        assert_eq!(
            expected_output,
            super::get_database_path(
                PathBuf::from(format!("{sep}some{sep}path{sep}trailing{sep}slash")).as_path(),
                workspaces.iter().map(PathBuf::as_path)
            )
        );
    }

    #[test]
    pub fn test_check_database_name_is_deterministic() {
        let path_1 = PathBuf::from("/some/workspace/1".to_string());
        let path_2 = PathBuf::from("/some/workspace/2".to_string());

        assert_eq!(
            "06932fd30d07d2130b10e019489a6609900ab13bf91b87b7db5b1da256b3c75c",
            super::generate_unique_db_name(
                [path_1.clone(), path_2.clone()]
                    .iter()
                    .map(PathBuf::as_path)
            )
        );

        // Notice, different workspaces result in different hashes
        assert_eq!(
            "d01760e68b77ce0e0332006ba446e60c790b2b4b7d36c58f20f131b41ed1a05c",
            super::generate_unique_db_name(std::iter::once(&path_1).map(PathBuf::as_path))
        );
        assert_eq!(
            "cc15b1ffd2f2ef48f3350a5d5de6f9b1c3cf88ea5f28e53f5087115054b870ce",
            super::generate_unique_db_name(std::iter::once(&path_2).map(PathBuf::as_path))
        );
    }

    #[rstest]
    #[case("   SomeEnum   ", "SomeEnum")]
    #[case("   SomeEnum\n", "SomeEnum")]
    #[case("  \n  SomeEnum   ", "SomeEnum")]
    #[case("SomeEnum   \n   ", "SomeEnum")]
    #[case(
        "pub fn some_function(\r\nvar_1: &str,\r\nvar_2:&str): String {\r\n\r\n}",
        "pub fn some_function(\nvar_1: &str,\nvar_2:&str): String {\n\n}"
    )]
    #[case("    \n\r\n\r    ", "")]
    pub fn test_normalising_symbols(#[case] name: &str, #[case] expected_normalised_name: &str) {
        assert_eq!(super::normalise_symbol_name(name), expected_normalised_name);
    }
}
