//! # LlvmFunc - Trait Implementations
//!
//! This module contains trait implementations for `LlvmFunc`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::{LlvmFunc, LlvmLinkage};
use std::fmt;

impl fmt::Display for LlvmFunc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_declare {
            write!(f, "declare")?;
        } else {
            write!(f, "define")?;
        }
        if self.linkage != LlvmLinkage::External {
            write!(f, " {}", self.linkage)?;
        }
        write!(f, " {} @{}(", self.ret_ty, self.name)?;
        for (i, (ty, name)) in self.params.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            if self.is_declare {
                write!(f, "{}", ty)?;
            } else {
                write!(f, "{} %{}", ty, name)?;
            }
        }
        write!(f, ")")?;
        for attr in &self.attrs {
            write!(f, " {}", attr)?;
        }
        if self.is_declare {
            writeln!(f)?;
        } else {
            writeln!(f, " {{")?;
            for instr in &self.body {
                writeln!(f, "{}", instr)?;
            }
            writeln!(f, "}}")?;
        }
        Ok(())
    }
}
