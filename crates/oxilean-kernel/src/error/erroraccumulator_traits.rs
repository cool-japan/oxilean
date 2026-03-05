//! # ErrorAccumulator - Trait Implementations
//!
//! This module contains trait implementations for `ErrorAccumulator`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ErrorAccumulator;
use std::fmt;

impl fmt::Display for ErrorAccumulator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, err) in self.errors.iter().enumerate() {
            write!(f, "[{}] {}", i + 1, err)?;
            if i + 1 < self.errors.len() {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
