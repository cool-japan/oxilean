//! # ScalaCaseClass - Trait Implementations
//!
//! This module contains trait implementations for `ScalaCaseClass`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::ScalaCaseClass;
use std::fmt;

impl fmt::Display for ScalaCaseClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "case class {}", self.name)?;
        if !self.type_params.is_empty() {
            write!(f, "[")?;
            for (i, tp) in self.type_params.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", tp)?;
            }
            write!(f, "]")?;
        }
        write!(f, "(")?;
        for (i, field) in self.fields.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", field)?;
        }
        write!(f, ")")?;
        if !self.extends_list.is_empty() {
            write!(f, " extends {}", self.extends_list[0])?;
            for e in &self.extends_list[1..] {
                write!(f, " with {}", e)?;
            }
        }
        Ok(())
    }
}
