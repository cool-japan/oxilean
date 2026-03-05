//! # GrindConfig - Trait Implementations
//!
//! This module contains trait implementations for `GrindConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::GrindConfig;

impl Default for GrindConfig {
    fn default() -> Self {
        GrindConfig {
            max_rounds: 100,
            max_instances: 1000,
            max_eclass_size: 500,
            split_cases: true,
            use_simp: true,
            max_splits: 10,
            max_nodes: 10000,
            collect_stats: false,
            fuel: 200,
            reconstruct_proofs: true,
        }
    }
}
