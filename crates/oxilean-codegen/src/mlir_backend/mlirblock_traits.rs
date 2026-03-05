//! # MlirBlock - Trait Implementations
//!
//! This module contains trait implementations for `MlirBlock`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MlirBlock;
use std::fmt;

impl fmt::Display for MlirBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(lbl) = &self.label {
            write!(f, "^{}", lbl)?;
            if !self.arguments.is_empty() {
                write!(f, "(")?;
                for (i, arg) in self.arguments.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", arg, arg.ty)?;
                }
                write!(f, ")")?;
            }
            writeln!(f, ":")?;
        } else if !self.arguments.is_empty() {
        }
        for op in &self.body {
            writeln!(f, "    {}", op)?;
        }
        Ok(())
    }
}
