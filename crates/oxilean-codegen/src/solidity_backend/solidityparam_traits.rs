//! # SolidityParam - Trait Implementations
//!
//! This module contains trait implementations for `SolidityParam`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SolidityParam;
use std::fmt;

impl fmt::Display for SolidityParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ty)?;
        if let Some(loc) = &self.location {
            write!(f, " {}", loc)?;
        }
        if !self.name.is_empty() {
            write!(f, " {}", self.name)?;
        }
        Ok(())
    }
}
