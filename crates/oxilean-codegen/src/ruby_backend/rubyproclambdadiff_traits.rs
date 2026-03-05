//! # RubyProcLambdaDiff - Trait Implementations
//!
//! This module contains trait implementations for `RubyProcLambdaDiff`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::RubyProcLambdaDiff;

impl Default for RubyProcLambdaDiff {
    fn default() -> Self {
        Self {
            arity_strict: true,
            return_behavior: "returns from lambda".to_string(),
        }
    }
}
