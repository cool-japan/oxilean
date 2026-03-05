//! # Closure - Trait Implementations
//!
//! This module contains trait implementations for `Closure`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Closure;
use std::fmt;

impl fmt::Display for Closure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref name) = self.name {
            write!(
                f,
                "<closure {} arity={} env={}>",
                name,
                self.arity,
                self.env.len()
            )
        } else {
            write!(
                f,
                "<closure {} arity={} env={}>",
                self.fn_ptr,
                self.arity,
                self.env.len()
            )
        }
    }
}
