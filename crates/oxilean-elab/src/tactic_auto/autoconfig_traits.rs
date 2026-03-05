//! # AutoConfig - Trait Implementations
//!
//! This module contains trait implementations for `AutoConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::AutoConfig;
use std::fmt;

impl Default for AutoConfig {
    fn default() -> Self {
        AutoConfig {
            max_depth: 5,
            max_steps: 1000,
            use_assumptions: true,
            use_simp: true,
            use_constructor: true,
            use_apply: true,
            lemma_hints: vec![],
        }
    }
}
