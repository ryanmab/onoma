use std::{ffi::OsStr, path::Path};

use crate::{
    models::{self},
    resolver::{constant::DEFAULT_SCORE, utils, weight},
};

/// Run fuzzy matching on a given symbol, for a query, using a set of configuration.
///
/// In practice, this fuzzy matches the symbols path (if available) and the symbols
/// name to provide zero or more fuzzy matches.
///
/// No matches returned means the query didn't match any elements of the Symbol, and
/// therefore the symbol can be completely ignored.
pub fn fuzzy_match(
    query: &str,
    symbol: &models::resolved::ResolvedSymbol,
    config: &neo_frizbee::Config,
) -> Vec<neo_frizbee::Match> {
    let path = symbol.path.to_str().unwrap_or_default();

    let path_symbol_prefix = format!("{path}:{}", symbol.name.as_str());

    // TODO: Should we batch the results from SQLite and perform matching on a
    // larger list? It'll be a tradeoff between Time to First Result (TTFR)
    // and SIMD performance. But, is the difference that much? Is it in fact
    // faster because SIMD is more efficient and the bridges aren't having to
    // continually repaint for every drip-fed result?
    neo_frizbee::match_list(
        query,
        &[path_symbol_prefix.as_str(), symbol.name.as_str()],
        config,
    )
}

/// Calculate a score for a given symbol, using a set of results from fuzzy matching ([`fuzzy_match`]),
/// the provided query, and the current file which is open (if available).
///
/// In practice, this weights all of these elements, along with derived heuristics like
/// whether the symbol looks to be part of a test harness ([`utils::is_part_of_test_harness`]),
/// or entrypoint file ([`utils::is_entrypoint_file`]) to provide a final sortable score.
///
/// The default score, if no bonuses or penalties are applied is defined as [`constant::DEFAULT_SCORE`].
/// Any score returned which is _below_ the default can be assumed to have occurred more penalties
/// than bonuses, and thus not a good match.
pub fn calculate_score<'a, 'b>(
    symbol: &models::resolved::ResolvedSymbol,
    fuzzy_matches: impl Iterator<Item = &'a neo_frizbee::Match>,
    current_file: Option<&'b Path>,
) -> i64 {
    let filename = if let Some(Some(filename)) = symbol.path.file_name().map(OsStr::to_str) {
        Some(filename)
    } else {
        None
    };

    let entrypoint_file_penalty = if let Some(filename) = filename
        && utils::is_entrypoint_file(filename)
    {
        // 1% penalty for symbols defined in an entrypoint - this helps to
        // filter out re-exports
        weight::ENTRYPOINT_FILE_SCORE_PENALTY
    } else {
        0
    };

    let fuzzy_match_bonus: i64 = fuzzy_matches.map(weight::calculate_fuzzy_match_bonus).sum();

    let symbol_kind_bonus = match symbol.kind {
        // 3.5% bonus for the most common symbol kinds
        models::parsed::SymbolKind::Function
        | models::parsed::SymbolKind::Method
        | models::parsed::SymbolKind::Struct
        | models::parsed::SymbolKind::Type
        | models::parsed::SymbolKind::TypeAlias
        | models::parsed::SymbolKind::Class
        | models::parsed::SymbolKind::Constant
        | models::parsed::SymbolKind::Enum
        | models::parsed::SymbolKind::EnumMember
        | models::parsed::SymbolKind::Interface => weight::COMMON_SYMBOL_KINDS_SCORE_BONUS,

        // 0.5% bonus for less frequently but helpful symbol kinds
        models::parsed::SymbolKind::Variable => weight::INFREQUENT_SYMBOL_KINDS_SCORE_BONUS,

        // 1.5% PENALTY for uncommon symbols
        models::parsed::SymbolKind::Package
        | models::parsed::SymbolKind::Module
        | models::parsed::SymbolKind::SelfParameter => weight::UNCOMMON_SYMBOL_KINDS_SCORE_PENALTY,

        // No bonus for any other kinds
        _ => 0,
    };

    let test_harness_penalty = if utils::is_part_of_test_harness(symbol.path.as_path()) {
        // 0.5% penalty for symbols which are part of a test harness (i.e. it's likely a test
        // case, part of a test file, etc.)
        weight::TEST_HARNESS_SCORE_PENALTY
    } else {
        0
    };

    // 1% penalty for each directory distance from the current focused file (up to max of 8 directories - or 8%)
    let distance_penalty = current_file.map_or(0, |current_file| {
        weight::calculate_distance_score_penalty(utils::get_path_distance(
            current_file,
            symbol.path.as_path(),
        ))
    });

    DEFAULT_SCORE
        .saturating_add(entrypoint_file_penalty)
        .saturating_add(fuzzy_match_bonus)
        .saturating_add(symbol_kind_bonus)
        .saturating_add(test_harness_penalty)
        .saturating_add(distance_penalty)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        models::{
            parsed::SymbolKind,
            resolved::{ResolvedSymbol, Score},
        },
        resolver::scoring::DEFAULT_SCORE,
    };

    #[test]
    pub fn test_scoring_struct_in_entrypoint_file() {
        let symbol = ResolvedSymbol {
            id: 1,
            name: "ResolvedSymbol".to_string(),
            kind: SymbolKind::Struct,
            path: PathBuf::from("/some/file/mod.rs"),
            score: Score::default(),
            start_line: 1,
            start_column: 1,
            end_line: 1,
            end_column: 14,
        };

        let score = super::calculate_score(&symbol, Vec::new().iter(), None);

        let mut target_score = DEFAULT_SCORE;

        target_score += 35; // Increase the score by 3.5%, because it is a struct
        target_score -= 10; // Reduce the default score by 1% because the Symbol is in a module file

        assert_eq!(target_score, score);
    }

    #[test]
    pub fn test_scoring_struct_where_path_has_no_filename() {
        let symbol = ResolvedSymbol {
            id: 1,
            name: "ResolvedSymbol".to_string(),
            kind: SymbolKind::Struct,
            path: PathBuf::from("/some/file"),
            score: Score::default(),
            start_line: 1,
            start_column: 1,
            end_line: 1,
            end_column: 14,
        };

        let score = super::calculate_score(&symbol, Vec::new().iter(), None);

        let mut target_score = DEFAULT_SCORE;

        target_score += 35; // Increase the score by 3.5%, because it is a struct
        // Notice, no decrement for being defined in an entrypoint file - because the filename is not
        // available. Arguably this should be an invariant, and caught with a panic/assert.

        assert_eq!(target_score, score);
    }

    #[test]
    pub fn test_scoring_variable_in_far_away_file() {
        let symbol = ResolvedSymbol {
            id: 1,
            name: "ResolvedSymbol".to_string(),
            kind: SymbolKind::Variable,
            path: PathBuf::from_iter(["", "some", "file", "over", "here", "file.rs"]),
            score: Score::default(),
            start_line: 1,
            start_column: 1,
            end_line: 1,
            end_column: 14,
        };

        let score = super::calculate_score(
            &symbol,
            Vec::new().iter(),
            Some(&PathBuf::from_iter([
                "a",
                "totally",
                "different",
                "file",
                "over",
                "there",
                "file.ts",
            ])),
        );

        let mut target_score = DEFAULT_SCORE;

        target_score += 5; // Increase the score by 0.5%, because it is a variable
        target_score -= 6; // Reduce the default score by 6% because the symbol is 6 directories apart

        assert_eq!(target_score, score);
    }

    #[test]
    pub fn test_scoring_module_symbol() {
        let symbol = ResolvedSymbol {
            id: 1,
            name: "tests".to_string(),
            kind: SymbolKind::Module,
            path: PathBuf::from("some_module.rs"),
            score: Score::default(),
            start_line: 1,
            start_column: 1,
            end_line: 1,
            end_column: 14,
        };

        let score = super::calculate_score(&symbol, Vec::new().iter(), None);

        let mut target_score = DEFAULT_SCORE;

        target_score -= 15; // Decrease the score by 0.5%, because it is a variable

        assert_eq!(target_score, score);
    }

    #[test]
    pub fn test_scoring_class_in_test_file() {
        let symbol = ResolvedSymbol {
            id: 1,
            name: "TestClass".to_string(),
            kind: SymbolKind::Class,
            path: PathBuf::from("some_file.test.ts"),
            score: Score::default(),
            start_line: 1,
            start_column: 1,
            end_line: 1,
            end_column: 9,
        };

        let score = super::calculate_score(&symbol, Vec::new().iter(), None);

        let mut target_score = DEFAULT_SCORE;

        target_score += 35; // Increase the score by 3.5%, because it is a Class
        target_score -= 5; // Decrease the score by 0.5%, because its in a test file

        assert_eq!(target_score, score);
    }

    #[test]
    pub fn test_scoring_fuzzy_matched_symbol() {
        let query = "Lem";

        let name = "TestLemma".to_string();
        let path = PathBuf::from_iter(["some", "file", "over", "there.ts"]);

        let symbol = ResolvedSymbol {
            id: 1,
            name: name.clone(),
            kind: SymbolKind::Lemma,
            path: path.clone(),
            score: Score::default(),
            start_line: 1,
            start_column: 1,
            end_line: 1,
            end_column: 9,
        };

        let config = neo_frizbee::Config {
            prefilter: true,
            max_typos: Some(1),
            sort: false,
            scoring: neo_frizbee::Scoring::default(),
        };

        // Broadly matches the behavior defined in scoring.rs, though not a requirement,
        // this test just confirms we _are_ factoring in the fuzzy matches, and that the
        // results are deterministic
        let fuzzy_matches = neo_frizbee::match_list(
            query,
            &[
                format!("{}:{name}", path.to_str().unwrap()).as_str(),
                name.as_str(),
            ],
            &config,
        );

        let score = super::calculate_score(&symbol, fuzzy_matches.iter(), None);

        let mut target_score = DEFAULT_SCORE;

        target_score += 26; // Increase the score by 2.6% for the fuzzy matches

        assert_eq!(target_score, score);
    }
}
