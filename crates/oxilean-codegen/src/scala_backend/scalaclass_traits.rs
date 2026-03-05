//! # ScalaClass - Trait Implementations
//!
//! This module contains trait implementations for `ScalaClass`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::ScalaClass;
use std::fmt;

impl fmt::Display for ScalaClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for m in &self.modifiers {
            write!(f, "{} ", m)?;
        }
        write!(f, "class {}", self.name)?;
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
        if !self.constructor_params.is_empty() {
            write!(f, "(")?;
            for (i, p) in self.constructor_params.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", p)?;
            }
            write!(f, ")")?;
        }
        if !self.extends_list.is_empty() {
            write!(f, " extends {}", self.extends_list[0])?;
            for e in &self.extends_list[1..] {
                write!(f, " with {}", e)?;
            }
        }
        write!(f, " {{")?;
        for m in &self.methods {
            write!(f, "\n  {}", m)?;
        }
        write!(f, "\n}}")
    }
}
