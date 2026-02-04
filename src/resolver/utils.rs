use std::{ffi::OsStr, path::Path};

/// Check if a given file (i.e. `path/to/some/file/lib.rs`) is in what would
/// traditionally be an entrypoint file in various programming languages.
///
/// Though not a guarantee, these files are _generally_ used to re-export
/// interfaces or act as an entrypoint into other files with business logic,
/// rather than housing the business logic themselves.
pub fn is_entrypoint_file(filename: &str) -> bool {
    matches!(
        filename,
        "mod.rs"
            | "lib.rs"
            | "main.rs"
            | "index.js"
            | "index.jsx"
            | "index.ts"
            | "index.tsx"
            | "index.mjs"
            | "index.cjs"
            | "index.vue"
            | "__init__.py"
            | "__main__.py"
            | "main.go"
            | "main.c"
            | "index.php"
            | "main.rb"
            | "index.rb"
    )
}

/// Check if a given file (i.e. `path/to/some/file/file.test.ts`) is in what would
/// generally be used in a test harness (i.e. Jest, PHP Unit, Rust unit/integration tests,
/// etc.) in various programming languages.
///
/// Though not a guarantee (also, not all cases are testable in this mechanism), these files are
/// _generally_ used for testing, and as such don't house business logic of their own.
pub fn is_part_of_test_harness(path: &Path) -> bool {
    // Common test file patterns
    let test_file_patterns = [
        // JavaScript / TypeScript
        ".test.js",
        ".spec.js",
        ".test.ts",
        ".spec.ts",
        // Python
        "test_",
        "_test.py",
        // PHP
        "test",
        "test.php",
        // Java
        "test",
        "Test.java",
        // Ruby
        "_test.rb",
        "test_",
        // Rust
        "tests.rs",
        "_test.rs",
        "test_",
        // Go
        "_test.go",
        // C / C++
        "_test.c",
        "_test.cpp",
        "_test.cc",
        // C#
        "test",
        "tests",
        "Test.cs",
        // Kotlin
        "Test.kt",
        // Swift
        "Tests.swift",
    ];

    // Check if the filename or directory matches test heuristics
    if let Some(filename) = path.file_name().and_then(OsStr::to_str)
        && test_file_patterns
            .iter()
            .any(|pattern| filename.ends_with(pattern))
    {
        return true;
    }

    // Check if any parent directory is named "tests" (common in Rust, PHP, Python, Go)
    if path
        .ancestors()
        .any(|ancestor| ancestor.file_name().is_some_and(|name| name == "tests"))
    {
        return true;
    }

    false
}

/// Get the number of path's distance between two files.
///
/// It is assumed that the caller has passed in **file paths** for both `a` and `b`.
pub fn get_path_distance(a: &Path, b: &Path) -> usize {
    let a_dir = a
        .parent()
        .map_or_else(String::new, |parent| parent.to_string_lossy().to_string());

    let b_dir = b
        .parent()
        .map_or_else(String::new, |parent| parent.to_string_lossy().to_string());

    if a_dir == b_dir {
        return 0; // Same directory, no penalty
    }

    let a_parts: Vec<&str> = a_dir
        .split(std::path::MAIN_SEPARATOR)
        .filter(|s| !s.is_empty())
        .collect();
    let b_parts: Vec<&str> = b_dir
        .split(std::path::MAIN_SEPARATOR)
        .filter(|s| !s.is_empty())
        .collect();

    let common_parts = a_parts
        .iter()
        .zip(b_parts.iter())
        .take_while(|(a, b)| a == b)
        .count();

    a_parts.len() - common_parts
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    #[test]
    pub fn test_distance_between_files() {
        let distance = super::get_path_distance(
            &PathBuf::from_iter(["some", "path", "here", "1", "mod.rs"]),
            &PathBuf::from_iter(["some", "path", "here", "2", "mod.rs"]),
        );

        assert_eq!(1, distance);
    }

    #[test]
    pub fn test_large_distance_between_files() {
        let distance = super::get_path_distance(
            &PathBuf::from_iter([
                "some", "path", "here", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11",
                "mod.rs",
            ]),
            &PathBuf::from_iter(["some", "path", "here", "2", "mod.rs"]),
        );

        assert_eq!(11, distance);
    }

    #[test]
    pub fn test_no_distance_between_files() {
        let distance = super::get_path_distance(
            &PathBuf::from_iter(["some", "path", "mod.rs"]),
            &PathBuf::from_iter(["some", "path", "something_else.rs"]),
        );

        assert_eq!(0, distance);
    }

    #[test]
    pub fn test_a_path_is_a_dir() {
        let distance = super::get_path_distance(
            &PathBuf::from_iter(["some", "path", "mod.rs"]),
            &PathBuf::from_iter(["some", "path"]),
        );

        assert_eq!(1, distance);
    }

    #[test]
    pub fn test_in_tests_folder() {
        let file = PathBuf::from_iter(["some", "root", "tests", "SomeFileTest.php"]);

        let is_in_test_harness = super::is_part_of_test_harness(file.as_path());

        assert!(is_in_test_harness);
    }

    #[test]
    pub fn test_in_test_harness_file_js() {
        let file = PathBuf::from_iter(["some", "root", "some_file.test.js"]);

        let is_in_test_harness = super::is_part_of_test_harness(file.as_path());

        assert!(is_in_test_harness);
    }

    #[test]
    pub fn test_not_in_test_harness() {
        let file = PathBuf::from_iter(["some", "root", "just_a_normal_file.py"]);

        let is_in_test_harness = super::is_part_of_test_harness(file.as_path());

        assert!(!is_in_test_harness);
    }
}
