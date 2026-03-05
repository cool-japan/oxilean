//! # SimpConfig - Trait Implementations
//!
//! This module contains trait implementations for `SimpConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SimpConfig;

impl Default for SimpConfig {
    fn default() -> Self {
        Self {
            max_steps: 100_000,
            use_congr: true,
            beta: true,
            eta: true,
            iota: true,
            zeta: true,
            try_rfl: true,
            use_default_lemmas: true,
            simp_hyps: false,
            discharge_conditions: true,
        }
    }
}
