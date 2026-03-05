//! # ScalaImport - Trait Implementations
//!
//! This module contains trait implementations for `ScalaImport`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::ScalaImport;
use std::fmt;

impl fmt::Display for ScalaImport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.items.is_empty() || (self.items.len() == 1 && self.items[0] == "*") {
            write!(f, "import {}.*", self.path)
        } else if self.items.len() == 1 {
            write!(f, "import {}.{}", self.path, self.items[0])
        } else {
            write!(f, "import {}.{{", self.path)?;
            for (i, item) in self.items.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", item)?;
            }
            write!(f, "}}")
        }
    }
}
