//! # ScoringCriteria - Trait Implementations
//!
//! This module contains trait implementations for `ScoringCriteria`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ScoringCriteria;

impl Default for ScoringCriteria {
    fn default() -> Self {
        Self {
            specificity: 0.0,
            remaining_goals: 0,
            edit_distance: 0,
            is_local: false,
            num_universe_params: 0,
            num_synth_args: 0,
            total_args: 0,
        }
    }
}
