//! # BasicBlock - Trait Implementations
//!
//! This module contains trait implementations for `BasicBlock`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::BasicBlock;
use std::fmt;

impl fmt::Display for BasicBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:", self.label)?;
        if !self.params.is_empty() {
            write!(f, "(")?;
            for (i, (r, ty)) in self.params.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}: {}", r, ty)?;
            }
            write!(f, ")")?;
        }
        writeln!(f)?;
        for inst in &self.instructions {
            writeln!(f, "  {:?}", inst)?;
        }
        if let Some(term) = &self.terminator {
            writeln!(f, "  {:?}", term)?;
        }
        Ok(())
    }
}
