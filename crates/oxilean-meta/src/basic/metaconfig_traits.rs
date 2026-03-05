//! # MetaConfig - Trait Implementations
//!
//! This module contains trait implementations for `MetaConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MetaConfig;

impl Default for MetaConfig {
    fn default() -> Self {
        Self {
            fo_approx: true,
            const_approx: false,
            ctx_approx: true,
            track_assignments: false,
            max_recursion_depth: 512,
            proof_irrelevance: true,
            eta_struct: true,
            unfold_reducible: true,
        }
    }
}
