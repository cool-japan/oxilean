//! # SimpStats - Trait Implementations
//!
//! This module contains trait implementations for `SimpStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::simp_types::SimpStats;

impl std::fmt::Display for SimpStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SimpStats {{ rewrites: {}, lemmas_tried: {}, beta: {}, iota: {} }}",
            self.rewrites_applied, self.lemmas_tried, self.beta_steps, self.iota_steps
        )
    }
}
