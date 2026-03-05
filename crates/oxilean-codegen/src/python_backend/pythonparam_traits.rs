//! # PythonParam - Trait Implementations
//!
//! This module contains trait implementations for `PythonParam`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::PythonParam;
use std::fmt;

impl fmt::Display for PythonParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_vararg {
            write!(f, "*")?;
        } else if self.is_kwarg {
            write!(f, "**")?;
        }
        write!(f, "{}", self.name)?;
        if let Some(ann) = &self.annotation {
            write!(f, ": {}", ann)?;
        }
        if let Some(default) = &self.default {
            write!(f, " = {}", default)?;
        }
        Ok(())
    }
}
