//! # MonoConfig - Trait Implementations
//!
//! This module contains trait implementations for `MonoConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::DEFAULT_MONO_MAX_DEPTH;
use super::types::MonoConfig;

impl Default for MonoConfig {
    fn default() -> Self {
        Self {
            max_depth: DEFAULT_MONO_MAX_DEPTH,
            use_defaults: true,
            custom_rules: Vec::new(),
            try_refl: true,
            try_assumption: true,
            trace: false,
        }
    }
}
