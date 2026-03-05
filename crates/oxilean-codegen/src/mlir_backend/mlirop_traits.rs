//! # MlirOp - Trait Implementations
//!
//! This module contains trait implementations for `MlirOp`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MlirOp;
use std::fmt;

impl fmt::Display for MlirOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.results.is_empty() {
            for (i, r) in self.results.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", r)?;
            }
            write!(f, " = ")?;
        }
        write!(f, "{}", self.op_name)?;
        if !self.operands.is_empty() {
            write!(f, " ")?;
            for (i, op) in self.operands.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", op)?;
            }
        }
        if !self.regions.is_empty() {
            write!(f, " ")?;
            for region in &self.regions {
                write!(f, "{}", region)?;
            }
        }
        if !self.successors.is_empty() {
            write!(f, " [")?;
            for (i, s) in self.successors.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "^{}", s)?;
            }
            write!(f, "]")?;
        }
        if !self.attributes.is_empty() {
            write!(f, " {{")?;
            for (i, (k, v)) in self.attributes.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{} = {}", k, v)?;
            }
            write!(f, "}}")?;
        }
        if !self.type_annotations.is_empty() {
            write!(f, " : ")?;
            if self.type_annotations.len() == 1 {
                write!(f, "{}", self.type_annotations[0])?;
            } else {
                write!(f, "(")?;
                for (i, t) in self.type_annotations.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", t)?;
                }
                write!(f, ")")?;
            }
        }
        Ok(())
    }
}
