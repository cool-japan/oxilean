//! # ScalaObject - Trait Implementations
//!
//! This module contains trait implementations for `ScalaObject`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::ScalaObject;
use std::fmt;

impl fmt::Display for ScalaObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "object {}", self.name)?;
        if !self.extends_list.is_empty() {
            write!(f, " extends {}", self.extends_list[0])?;
            for e in &self.extends_list[1..] {
                write!(f, " with {}", e)?;
            }
        }
        write!(f, " {{")?;
        for (name, ty, expr) in &self.constants {
            write!(f, "\n  val {}: {} = {}", name, ty, expr)?;
        }
        for m in &self.methods {
            write!(f, "\n  {}", m)?;
        }
        write!(f, "\n}}")
    }
}
