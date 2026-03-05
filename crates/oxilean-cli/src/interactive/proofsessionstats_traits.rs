//! # ProofSessionStats - Trait Implementations
//!
//! This module contains trait implementations for `ProofSessionStats`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::fmt;
use std::time::Instant;

use super::types::ProofSessionStats;

impl Default for ProofSessionStats {
    fn default() -> Self {
        Self {
            tactics_applied: 0,
            undos: 0,
            goals_created: 0,
            goals_solved: 0,
            total_time_ms: 0,
            start_time: Instant::now(),
        }
    }
}
