//! # ProofOptions - Trait Implementations
//!
//! This module contains trait implementations for `ProofOptions`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ProofOptions;
use std::fmt;

impl Default for ProofOptions {
    fn default() -> Self {
        Self {
            sorry_is_error: false,
            max_depth: 1024,
            trace: false,
            verbose: false,
        }
    }
}
