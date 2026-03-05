//! # VyperParam - Trait Implementations
//!
//! This module contains trait implementations for `VyperParam`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::VyperParam;
use std::fmt;

impl fmt::Display for VyperParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.ty)?;
        if let Some(d) = &self.default {
            write!(f, " = {}", d)?;
        }
        Ok(())
    }
}
