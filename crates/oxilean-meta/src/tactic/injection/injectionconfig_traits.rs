//! # InjectionConfig - Trait Implementations
//!
//! This module contains trait implementations for `InjectionConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::MAX_INJECTION_DEPTH;
use super::types::InjectionConfig;

impl Default for InjectionConfig {
    fn default() -> Self {
        Self {
            with_names: Vec::new(),
            recurse: false,
            clear_hyp: false,
            max_depth: MAX_INJECTION_DEPTH,
            subst: false,
        }
    }
}
