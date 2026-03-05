//! # HaskellNewtype - Trait Implementations
//!
//! This module contains trait implementations for `HaskellNewtype`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::HaskellNewtype;
use std::fmt;

impl fmt::Display for HaskellNewtype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "newtype {}", self.name)?;
        if let Some(p) = &self.type_param {
            write!(f, " {}", p)?;
        }
        write!(
            f,
            " = {} {{ {} :: {} }}",
            self.constructor, self.field.0, self.field.1
        )?;
        if !self.deriving_clauses.is_empty() {
            write!(f, " deriving (")?;
            for (i, d) in self.deriving_clauses.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", d)?;
            }
            write!(f, ")")?;
        }
        Ok(())
    }
}
