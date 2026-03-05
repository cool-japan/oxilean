//! # EvalErrorChain - Trait Implementations
//!
//! This module contains trait implementations for `EvalErrorChain`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::EvalErrorChain;
use std::fmt;

impl fmt::Display for EvalErrorChain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}
