//! # ExtConfig - Trait Implementations
//!
//! This module contains trait implementations for `ExtConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::DEFAULT_EXT_DEPTH;
use super::types::ExtConfig;

impl Default for ExtConfig {
    fn default() -> Self {
        Self {
            max_depth: DEFAULT_EXT_DEPTH,
            use_default_lemmas: true,
            extra_lemmas: Vec::new(),
            with_names: Vec::new(),
        }
    }
}
