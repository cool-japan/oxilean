//! # VectorInstr - Trait Implementations
//!
//! This module contains trait implementations for `VectorInstr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::VectorInstr;
use std::fmt;

impl fmt::Display for VectorInstr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {} = [", self.width.bits(), self.op, self.dst)?;
        for (i, s) in self.srcs.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", s)?;
        }
        write!(f, "]")
    }
}
