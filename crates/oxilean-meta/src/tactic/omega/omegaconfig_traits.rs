//! # OmegaConfig - Trait Implementations
//!
//! This module contains trait implementations for `OmegaConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::OmegaConfig;

impl Default for OmegaConfig {
    fn default() -> Self {
        OmegaConfig {
            max_steps: 1000,
            use_preprocessing: true,
            use_dark_gray_shadow: true,
            allow_case_splits: true,
            max_case_splits: 10,
            nat_mode: true,
        }
    }
}
