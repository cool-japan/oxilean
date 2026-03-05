//! # SwiftParam - Trait Implementations
//!
//! This module contains trait implementations for `SwiftParam`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::SwiftParam;
use std::fmt;

impl fmt::Display for SwiftParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.label.is_empty() && self.label != self.name {
            write!(f, "{} ", self.label)?;
        }
        if self.inout {
            write!(f, "inout ")?;
        }
        write!(f, "{}: {}", self.name, self.ty)?;
        if self.variadic {
            write!(f, "...")?;
        }
        if let Some(ref default) = self.default {
            write!(f, " = {}", default)?;
        }
        Ok(())
    }
}
