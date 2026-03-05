//! # ElabOptions - Trait Implementations
//!
//! This module contains trait implementations for `ElabOptions`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ElabOptions;
use std::fmt;

impl Default for ElabOptions {
    fn default() -> Self {
        Self {
            allow_sorry: false,
            auto_bound_implicit: true,
            max_universe: 100,
            warn_unused_hyps: false,
            strict_def_eq: true,
        }
    }
}
