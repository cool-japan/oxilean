//! # Pap - Trait Implementations
//!
//! This module contains trait implementations for `Pap`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Pap;
use std::fmt;

impl fmt::Display for Pap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<pap {} ({}/{})",
            self.closure.fn_ptr,
            self.args.len(),
            self.closure.arity
        )?;
        write!(f, ">")
    }
}
