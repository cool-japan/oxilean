//! # LocalContextEntry - Trait Implementations
//!
//! This module contains trait implementations for `LocalContextEntry`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LocalContextEntry;
use std::fmt;

impl fmt::Display for LocalContextEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} : {:?}", self.name, self.ty)?;
        if let Some(val) = &self.val {
            write!(f, " := {:?}", val)?;
        }
        Ok(())
    }
}
