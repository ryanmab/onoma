use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::resolver::constant;

/// A score for a symbol.
#[derive(Debug, sqlx::FromRow, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Score(i64);

impl Default for Score {
    fn default() -> Self {
        Self(constant::DEFAULT_SCORE)
    }
}

impl Deref for Score {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<i64> for Score {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
