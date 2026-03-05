//! # FlatClosure - Trait Implementations
//!
//! This module contains trait implementations for `FlatClosure`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FlatClosure;
use std::fmt;

impl fmt::Display for FlatClosure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FlatClosure(fn={}, arity={}, env={})",
            self.fn_index,
            self.arity,
            self.env.len()
        )
    }
}
