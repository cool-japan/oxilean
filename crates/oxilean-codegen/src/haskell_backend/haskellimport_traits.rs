//! # HaskellImport - Trait Implementations
//!
//! This module contains trait implementations for `HaskellImport`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::HaskellImport;
use std::fmt;

impl fmt::Display for HaskellImport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "import")?;
        if self.qualified {
            write!(f, " qualified")?;
        }
        write!(f, " {}", self.module)?;
        if let Some(alias) = &self.alias {
            write!(f, " as {}", alias)?;
        }
        if !self.hiding.is_empty() {
            write!(f, " hiding (")?;
            for (i, h) in self.hiding.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", h)?;
            }
            write!(f, ")")?;
        } else if !self.items.is_empty() {
            write!(f, " (")?;
            for (i, item) in self.items.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", item)?;
            }
            write!(f, ")")?;
        }
        Ok(())
    }
}
