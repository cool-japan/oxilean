//! # MutualInductionConfig - Trait Implementations
//!
//! This module contains trait implementations for `MutualInductionConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MutualInductionConfig;

impl Default for MutualInductionConfig {
    fn default() -> Self {
        Self {
            target_configs: Vec::new(),
            target_names: Vec::new(),
            use_combined_recursor: true,
            motive_names: Vec::new(),
            shared_generalizing: Vec::new(),
            max_mutual: 16,
        }
    }
}
