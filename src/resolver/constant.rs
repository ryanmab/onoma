/// The default score for a resolved symbol, before any bonuses of penalties are applied.
///
/// Symbols who's calculated score is below this can be presumed to have incurred more penalties
/// than bonuses, and thus likely not a good match for a given query.
///
/// See [`resolver::utils::calculate_score`].
pub const DEFAULT_SCORE: i64 = 1000;

/// The number of seconds the Resolver thread will attempt to send a resolved
/// symbol back to the caller, before timing out and shutting down.
///
/// This number should be long enough to not prematurely shut down the resolving
/// thread while the caller is still alive and processing messages. But also not
/// too long that the Resolving thread is holding a connection in the sqlx pool
/// and starving future queries from being processed.
pub const RESOLVER_SEND_TIMEOUT_SECS: u64 = 2;
