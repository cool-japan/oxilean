//! # ScalaTrait - Trait Implementations
//!
//! This module contains trait implementations for `ScalaTrait`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::ScalaTrait;
use std::fmt;

impl fmt::Display for ScalaTrait {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "trait {}", self.name)?;
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
        if !self.extends_list.is_empty() {
            write!(f, " extends {}", self.extends_list[0])?;
            for e in &self.extends_list[1..] {
                write!(f, " with {}", e)?;
            }
        }
        write!(f, " {{")?;
        for m in &self.abstract_methods {
            write!(f, "\n  {}", m)?;
        }
        for m in &self.concrete_methods {
            write!(f, "\n  {}", m)?;
        }
        write!(f, "\n}}")
    }
}
