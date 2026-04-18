use std::{ffi::OsStr, path::Path};

use crate::{
    models::{self},
    resolver::{constant::DEFAULT_SCORE, utils, weight},
};

/// Get the fuzzy matching config for a particular query.
///
/// This factors in smart casing and typos to produce a best effort fuzzy match
/// behaviour which behaves as you'd expect when searching.
pub fn get_fuzzy_config(query: &str) -> frizbee::Config {
    let has_uppercase = query.chars().any(char::is_uppercase);

    frizbee::Config {
        // Scale the number of typos with the length of the query.
        //
        // In other words, allow for more misplaced letters in fuzzy matching
        // as more characters are typed.
        //
        // This is a trade off as symbols become less relevant the more typos you allow (as
        // in, you'll see more symbols which are further from the original query).
        //
        // NOTE: This must never be below the length of the query, otherwise
        // frizbee will panic
        max_typos: Some(u16::try_from(query.len().div_euclid(3).min(4)).unwrap_or(0)),
        sort: false,
        scoring: frizbee::Scoring {
            // Make the fuzzy matching act more like smart case grep in Vim, in that if the query
            // is all lowercase, the query is treated as case insensitive (i.e. no favoring to
            // matched casing).
            capitalization_bonus: if has_uppercase {
                weight::CASE_SENSITIVE_MATCHING_CAPITALISATION_BONUS
            } else {
                0
            },
            matching_case_bonus: if has_uppercase {
                weight::CASE_SENSITIVE_MATCHING_CASE_BONUS
            } else {
                0
            },
            ..Default::default()
        },
    }
}

/// Run fuzzy matching on a given symbol, for a query, using a set of configuration.
///
/// No matches returned means the query didn't match any elements of the Symbol, and
/// therefore the symbol can be completely ignored.
pub fn fuzzy_match(
    query: &str,
    symbol: &models::resolved::ResolvedSymbol,
    config: &frizbee::Config,
) -> Vec<frizbee::Match> {
    // TODO: Should we batch the results from SQLite and perform matching on a
    // larger list? It'll be a tradeoff between Time to First Result (TTFR)
    // and SIMD performance. But, is the difference that much? Is it in fact
    // faster because SIMD is more efficient and the bridges aren't having to
    // continually repaint for every drip-fed result?

    frizbee::match_list(
        query,
        &[
            symbol.name.as_str(),
            // NB: Include the lowercase name here in order to favour exact matches on symbols
            // who contain upper case characters. Frizbee by default only does case-sensitive exact
            // matching meaning `watcher` and `Watcher` will not be treated as an exact match
            // unless we include an explicit lowercase haystack element.
            symbol.name.to_lowercase().as_str(),
        ],
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
    query: &str,
    symbol: &models::resolved::ResolvedSymbol,
    fuzzy_matches: impl Iterator<Item = &'a frizbee::Match>,
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
        // Penalty for symbols defined in an entrypoint - this helps to
        // filter out re-exports
        weight::ENTRYPOINT_FILE_SCORE_PENALTY
    } else {
        0
    };

    let fuzzy_match_bonus: i64 = fuzzy_matches.map(weight::calculate_fuzzy_match_bonus).sum();

    let clear_intent_bonus: i64 = calculate_clear_intent_bonus(query, symbol);

    let symbol_kind_bonus = match symbol.kind {
        // Bonus for the most commonly jumped to symbol kinds
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

        // Bonus for less frequently used, but helpful, symbol kinds
        models::parsed::SymbolKind::Variable => weight::INFREQUENT_SYMBOL_KINDS_SCORE_BONUS,

        // Penalty for uncommon symbols
        models::parsed::SymbolKind::Package
        | models::parsed::SymbolKind::Module
        | models::parsed::SymbolKind::SelfParameter => weight::UNCOMMON_SYMBOL_KINDS_SCORE_PENALTY,

        // No bonus for any other kinds
        _ => 0,
    };

    let test_harness_penalty = if utils::is_part_of_test_harness(symbol.path.as_path()) {
        // Penalty for symbols which are part of a test harness (i.e. it's likely a test
        // case, part of a test file, etc.)
        weight::TEST_HARNESS_SCORE_PENALTY
    } else {
        0
    };

    // Penalty for each directory distance from the current focused file (up to max of 8 directories - or 8%)
    let distance_penalty = current_file.map_or(0, |current_file| {
        if current_file == symbol.path {
            // Apply a penalty to symbols inside the current file. The idea is that it's likely that the
            // intent of a workspace-wide search is to find symbols which are within close proximity
            // ([`calculate_distance_score_penalty`]), but also not in a place where it's more convenient
            // to just grep/navigate to.
            return weight::SAME_FILE_PENALTY;
        }

        weight::calculate_distance_score_penalty(utils::get_path_distance(
            current_file,
            symbol.path.as_path(),
        ))
    });

    DEFAULT_SCORE
        .saturating_add(entrypoint_file_penalty)
        .saturating_add(fuzzy_match_bonus)
        .saturating_add(clear_intent_bonus)
        .saturating_add(symbol_kind_bonus)
        .saturating_add(test_harness_penalty)
        .saturating_add(distance_penalty)
}

/// Apply a bonus to symbols who's [`models::resolved::SymbolKind`] matches the intent
/// displayed by a query string.
///
/// This is largely a heuristic using common style conventions - i.e. constants in all caps,
/// functions in snake or pascal case, etc.
pub fn calculate_clear_intent_bonus(query: &str, symbol: &models::resolved::ResolvedSymbol) -> i64 {
    let has_uppercase = query.chars().any(char::is_uppercase);
    let has_lowercase = query.chars().any(char::is_lowercase);
    let has_underscores = query.chars().any(|c| c == '_');

    let is_length_for_clear_intent = query.len() >= 3;
    let is_upper_and_lower_mix = has_uppercase && has_lowercase && !has_underscores;
    let is_snake_case = has_underscores && !has_uppercase;
    let is_screaming_case = has_uppercase && !has_lowercase;

    // Bonus for symbols where a particular query indicates clear intent to that symbol
    // kind. For example, queries in all uppercase prioritise constants, and queries which
    // could be pascal case prioritise structs/classes/etc.
    match symbol.kind {
        // i.e. `SOME_CONSTANT` or `WEIGHT`
        models::parsed::SymbolKind::Constant
        | models::parsed::SymbolKind::StaticField
        | models::parsed::SymbolKind::StaticVariable
        | models::parsed::SymbolKind::StaticDataMember
            if is_length_for_clear_intent && is_screaming_case =>
        {
            weight::CLEAR_QUERY_INTENT_SYMBOL_KINDS_SCORE_BONUS
        }

        // i.e. `SpecialClass` or `NewClassInterface`
        models::parsed::SymbolKind::Struct
        | models::parsed::SymbolKind::Type
        | models::parsed::SymbolKind::TypeAlias
        | models::parsed::SymbolKind::Class
        | models::parsed::SymbolKind::Enum
        | models::parsed::SymbolKind::EnumMember
        | models::parsed::SymbolKind::Interface
        | models::parsed::SymbolKind::Trait
        | models::parsed::SymbolKind::Protocol
        | models::parsed::SymbolKind::Union
        | models::parsed::SymbolKind::Variable // Components as Arrow functions in TypeScript
            if is_length_for_clear_intent && is_upper_and_lower_mix =>
        {
            weight::CLEAR_QUERY_INTENT_SYMBOL_KINDS_SCORE_BONUS
        }

        // i.e. `is_ready` or `isReady` or `IsReady`
        models::parsed::SymbolKind::Function
        | models::parsed::SymbolKind::Method
        | models::parsed::SymbolKind::Predicate
        | models::parsed::SymbolKind::TraitMethod
        | models::parsed::SymbolKind::ProtocolMethod
        | models::parsed::SymbolKind::AbstractMethod
        | models::parsed::SymbolKind::Getter
            if is_length_for_clear_intent
                && (is_snake_case || is_upper_and_lower_mix ) =>
        {
            weight::CLEAR_QUERY_INTENT_SYMBOL_KINDS_SCORE_BONUS
        }

        // No bonus for anything without clear intent
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use std::path::PathBuf;

    use crate::{
        models::{
            parsed::{Language, SymbolKind},
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
            language: Language::Rust,
            path: PathBuf::from("/some/file/mod.rs"),
            score: Score::default(),
            start_line: 1,
            start_column: 1,
            end_line: 1,
            end_column: 14,
        };

        let score = super::calculate_score("", &symbol, Vec::new().iter(), None);

        let mut target_score = DEFAULT_SCORE;

        target_score += 35; // Increase the score by 3.5%, because it is a struct
        target_score -= 2; // Reduce the default score by 0.25% because the Symbol is in a module file

        assert_eq!(target_score, score);
    }

    #[test]
    pub fn test_scoring_struct_where_path_has_no_filename() {
        let symbol = ResolvedSymbol {
            id: 1,
            name: "ResolvedSymbol".to_string(),
            kind: SymbolKind::Struct,
            language: Language::Rust,
            path: PathBuf::from("/some/file"),
            score: Score::default(),
            start_line: 1,
            start_column: 1,
            end_line: 1,
            end_column: 14,
        };

        let score = super::calculate_score("", &symbol, Vec::new().iter(), None);

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
            language: Language::Rust,
            path: PathBuf::from_iter(["", "some", "file", "over", "here", "file.rs"]),
            score: Score::default(),
            start_line: 1,
            start_column: 1,
            end_line: 1,
            end_column: 14,
        };

        let score = super::calculate_score(
            "",
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

        target_score += 15; // Increase the score by 1.5%, because it is a variable
        target_score -= 12; // Reduce the default score by 12% because the symbol is 6 directories apart

        assert_eq!(target_score, score);
    }

    #[test]
    pub fn test_scoring_variable_in_same_file() {
        let symbol = ResolvedSymbol {
            id: 1,
            name: "ResolvedSymbol".to_string(),
            kind: SymbolKind::Variable,
            language: Language::Rust,
            path: PathBuf::from_iter(["", "some", "file", "over", "here", "file.rs"]),
            score: Score::default(),
            start_line: 1,
            start_column: 1,
            end_line: 1,
            end_column: 14,
        };

        let score = super::calculate_score(
            "",
            &symbol,
            Vec::new().iter(),
            Some(&PathBuf::from_iter([
                "", "some", "file", "over", "here", "file.rs",
            ])),
        );

        let mut target_score = DEFAULT_SCORE;

        target_score += 15; // Increase the score by 1.5%, because it is a variable
        target_score -= 10; // Reduce the default score by 1%

        assert_eq!(target_score, score);
    }

    #[test]
    pub fn test_scoring_module_symbol() {
        let symbol = ResolvedSymbol {
            id: 1,
            name: "tests".to_string(),
            kind: SymbolKind::Module,
            language: Language::Rust,
            path: PathBuf::from("some_module.rs"),
            score: Score::default(),
            start_line: 1,
            start_column: 1,
            end_line: 1,
            end_column: 14,
        };

        let score = super::calculate_score("", &symbol, Vec::new().iter(), None);

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
            language: Language::TypeScript,
            path: PathBuf::from("some_file.test.ts"),
            score: Score::default(),
            start_line: 1,
            start_column: 1,
            end_line: 1,
            end_column: 9,
        };

        let score = super::calculate_score("", &symbol, Vec::new().iter(), None);

        let mut target_score = DEFAULT_SCORE;

        target_score += 35; // Increase the score by 3.5%, because it is a Class
        target_score -= 10; // Decrease the score by 1.0%, because its in a test file

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
            language: Language::TypeScript,
            path: path.clone(),
            score: Score::default(),
            start_line: 1,
            start_column: 1,
            end_line: 1,
            end_column: 9,
        };

        let config = frizbee::Config {
            max_typos: Some(1),
            sort: false,
            scoring: frizbee::Scoring::default(),
        };

        // Broadly matches the behavior defined in scoring.rs, though not a requirement,
        // this test just confirms we _are_ factoring in the fuzzy matches, and that the
        // results are deterministic
        let fuzzy_matches = frizbee::match_list(
            query,
            &[
                format!("{}:{name}", path.to_str().unwrap()).as_str(),
                name.as_str(),
            ],
            &config,
        );

        let score = super::calculate_score(&query, &symbol, fuzzy_matches.iter(), None);

        let mut target_score = DEFAULT_SCORE;

        target_score += 26; // Increase the score by 2.6% for the fuzzy matches (with matching case)

        assert_eq!(target_score, score);
    }

    #[test]
    pub fn test_scoring_fuzzy_matched_symbol_smartcase_insensitive() {
        let query = "lem";

        let name = "TestLemma".to_string();
        let path = PathBuf::from_iter(["some", "file", "over", "there.ts"]);

        let symbol = ResolvedSymbol {
            id: 1,
            name: name.clone(),
            kind: SymbolKind::Lemma,
            language: Language::Clojure,
            path: path.clone(),
            score: Score::default(),
            start_line: 1,
            start_column: 1,
            end_line: 1,
            end_column: 9,
        };

        let config = frizbee::Config {
            max_typos: Some(1),
            sort: false,
            scoring: frizbee::Scoring::default(),
        };

        // Broadly matches the behavior defined in scoring.rs, though not a requirement,
        // this test just confirms we _are_ factoring in the fuzzy matches, and that the
        // results are deterministic
        let fuzzy_matches = frizbee::match_list(
            query,
            &[
                format!("{}:{name}", path.to_str().unwrap()).as_str(),
                name.as_str(),
            ],
            &config,
        );

        let score = super::calculate_score(&query, &symbol, fuzzy_matches.iter(), None);

        let mut target_score = DEFAULT_SCORE;

        target_score += 24; // Increase the score by 2.4% for the fuzzy matches (as the case does not match)

        assert_eq!(target_score, score);
    }

    // =========================================================
    // CONSTANT / SCREAMING CASE INTENT
    // =========================================================
    #[rstest]
    #[case(SymbolKind::Constant)]
    #[case(SymbolKind::StaticField)]
    #[case(SymbolKind::StaticVariable)]
    #[case(SymbolKind::StaticDataMember)]
    fn test_constant_screaming_case_intent(#[case] kind: SymbolKind) {
        let query = "MAXSIZE";

        let sym = ResolvedSymbol {
            id: 1,
            name: "MAXSIZE".to_string(),
            kind,
            language: Language::Rust,
            path: PathBuf::from("src/lib.rs"),
            score: Score::default(),
            start_line: 1,
            start_column: 1,
            end_line: 1,
            end_column: 1,
        };

        let score = calculate_clear_intent_bonus(query, &sym);

        assert_eq!(score, weight::CLEAR_QUERY_INTENT_SYMBOL_KINDS_SCORE_BONUS);
    }

    #[test]
    fn test_constant_non_screaming_case_no_bonus() {
        let query = "maxSize";

        let sym = ResolvedSymbol {
            id: 1,
            name: "MAXSIZE".to_string(),
            kind: SymbolKind::Constant,
            language: Language::Rust,
            path: PathBuf::from("src/lib.rs"),
            score: Score::default(),
            start_line: 1,
            start_column: 1,
            end_line: 1,
            end_column: 1,
        };

        let score = calculate_clear_intent_bonus(query, &sym);
        assert_eq!(score, 0);
    }

    // =========================================================
    // TYPE / STRUCT / CLASS INTENT
    // =========================================================
    #[rstest]
    #[case(SymbolKind::Struct)]
    #[case(SymbolKind::Type)]
    #[case(SymbolKind::TypeAlias)]
    #[case(SymbolKind::Class)]
    #[case(SymbolKind::Enum)]
    #[case(SymbolKind::EnumMember)]
    #[case(SymbolKind::Interface)]
    #[case(SymbolKind::Trait)]
    #[case(SymbolKind::Protocol)]
    #[case(SymbolKind::Union)]
    fn test_type_like_upper_lower_mix(#[case] kind: SymbolKind) {
        let query = "UserProfile";

        let sym = ResolvedSymbol {
            id: 1,
            name: "UserProfile".to_string(),
            kind,
            language: Language::Rust,
            path: PathBuf::from("src/lib.rs"),
            score: Score::default(),
            start_line: 1,
            start_column: 1,
            end_line: 1,
            end_column: 1,
        };

        let score = calculate_clear_intent_bonus(query, &sym);

        assert_eq!(score, weight::CLEAR_QUERY_INTENT_SYMBOL_KINDS_SCORE_BONUS);
    }

    #[test]
    fn test_type_like_snake_case_no_bonus() {
        let query = "user_profile";

        let sym = ResolvedSymbol {
            id: 1,
            name: "UserProfile".to_string(),
            kind: SymbolKind::Struct,
            language: Language::Rust,
            path: PathBuf::from("src/lib.rs"),
            score: Score::default(),
            start_line: 1,
            start_column: 1,
            end_line: 1,
            end_column: 1,
        };

        let score = calculate_clear_intent_bonus(query, &sym);
        assert_eq!(score, 0);
    }

    #[rstest]
    #[case("is_ready")]
    #[case("isReady")]
    fn test_function_intent(#[case] query: &str) {
        let sym = ResolvedSymbol {
            id: 1,
            name: query.to_string(),
            kind: SymbolKind::Function,
            language: Language::Rust,
            path: PathBuf::from("src/lib.rs"),
            score: Score::default(),
            start_line: 1,
            start_column: 1,
            end_line: 1,
            end_column: 1,
        };

        let score = calculate_clear_intent_bonus(query, &sym);

        assert_eq!(score, weight::CLEAR_QUERY_INTENT_SYMBOL_KINDS_SCORE_BONUS);
    }

    #[rstest]
    #[case(SymbolKind::Getter)]
    #[case(SymbolKind::Function)]
    #[case(SymbolKind::Method)]
    #[case(SymbolKind::Predicate)]
    fn test_predicate_intent(#[case] kind: SymbolKind) {
        let query = "is_ready";

        let sym = ResolvedSymbol {
            id: 1,
            name: "is_ready".to_string(),
            kind,
            language: Language::Rust,
            path: PathBuf::from("src/lib.rs"),
            score: Score::default(),
            start_line: 1,
            start_column: 1,
            end_line: 1,
            end_column: 1,
        };

        let score = calculate_clear_intent_bonus(query, &sym);

        assert_eq!(score, weight::CLEAR_QUERY_INTENT_SYMBOL_KINDS_SCORE_BONUS);
    }

    #[rstest]
    #[case("is", SymbolKind::Function)]
    #[case("maxSize", SymbolKind::Constant)]
    #[case("is_ready", SymbolKind::Struct)]
    fn test_no_bonus_cases(#[case] query: &str, #[case] kind: SymbolKind) {
        let sym = ResolvedSymbol {
            id: 1,
            name: "irrelevant".to_string(),
            kind,
            language: Language::Rust,
            path: PathBuf::from("src/lib.rs"),
            score: Score::default(),
            start_line: 1,
            start_column: 1,
            end_line: 1,
            end_column: 1,
        };

        let score = calculate_clear_intent_bonus(query, &sym);
        assert_eq!(score, 0);
    }

    #[test]
    fn test_short_query_no_bonus() {
        let query = "is";

        let sym = ResolvedSymbol {
            id: 1,
            name: "is_ready".to_string(),
            kind: SymbolKind::Function,
            language: Language::Rust,
            path: PathBuf::from("src/lib.rs"),
            score: Score::default(),
            start_line: 1,
            start_column: 1,
            end_line: 1,
            end_column: 1,
        };

        let score = calculate_clear_intent_bonus(query, &sym);
        assert_eq!(score, 0);
    }
}
