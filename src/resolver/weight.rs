use crate::resolver::constant::{self, DEFAULT_SCORE};

/// 3.5% bonus for common symbol kinds.
pub const COMMON_SYMBOL_KINDS_SCORE_BONUS: i64 = (constant::DEFAULT_SCORE * 35) / 1000;

/// 0,5% bonus for infrequent symbol kinds.
pub const INFREQUENT_SYMBOL_KINDS_SCORE_BONUS: i64 = (constant::DEFAULT_SCORE * 5) / 1000;

/// -1.5% penalty for uncommon symbol kinds.
pub const UNCOMMON_SYMBOL_KINDS_SCORE_PENALTY: i64 = -((constant::DEFAULT_SCORE * 15) / 1000);

/// 0.5% penalty for symbols which are part of a test harness (i.e. it's likely a test
/// case, part of a test file, etc.).
pub const TEST_HARNESS_SCORE_PENALTY: i64 = -((constant::DEFAULT_SCORE * 5) / 1000);

/// 1% penalty for symbols defined in an entrypoint - this helps to
/// filter out re-exports.
pub const ENTRYPOINT_FILE_SCORE_PENALTY: i64 = -(constant::DEFAULT_SCORE / 100);

/// 1% penalty for each directory distance from the current focused file (up to max of
/// 8 directories - aka a 8% penalty)
pub fn calculate_distance_score_penalty(distance: usize) -> i64 {
    const MAX_DISTANCE: i64 = 8;

    if distance == 0 {
        return 0;
    }

    let distance = i64::try_from(distance)
        .unwrap_or(MAX_DISTANCE)
        .min(MAX_DISTANCE);

    -((constant::DEFAULT_SCORE * distance) / 1000)
}

/// A small bonus for fuzzy match scores, and a higher bonus for exact match scores.
///
/// Broadly these are arbitrary, but the bonus should be enough that exact (and similar) matches
/// are scored higher than those which are only loosely matched.
pub fn calculate_fuzzy_match_bonus(fuzzy_match: &frizbee::Match) -> i64 {
    match fuzzy_match {
        fuzzy_match if fuzzy_match.exact => {
            let score = i64::from(fuzzy_match.score);

            (score / 5) * 2
        }
        fuzzy_match => {
            let score = i64::from(fuzzy_match.score);

            // Prevent non-exact matches going above 4% of the score to prevent arbitrary inflation
            // of symbols from generic queries, which aren't exact matches.
            (score / 4).min((DEFAULT_SCORE * 40) / 1000)
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case(0, 0)]
    #[case(1, -1)]
    #[case(2, -2)]
    #[case(3, -3)]
    #[case(4, -4)]
    #[case(5, -5)]
    #[case(6, -6)]
    #[case(7, -7)]
    #[case(8, -8)]
    #[case(9, -8)]
    #[case(10, -8)]
    pub fn test_distance_weighting(#[case] distance: usize, #[case] expected_penalty: i64) {
        assert_eq!(
            expected_penalty,
            super::calculate_distance_score_penalty(distance)
        );
    }
}
