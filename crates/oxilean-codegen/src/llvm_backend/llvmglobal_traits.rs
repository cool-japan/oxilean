//! # LlvmGlobal - Trait Implementations
//!
//! This module contains trait implementations for `LlvmGlobal`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::{LlvmGlobal, LlvmLinkage};
use std::fmt;

impl fmt::Display for LlvmGlobal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "@{} = ", self.name)?;
        if self.linkage != LlvmLinkage::External {
            write!(f, "{} ", self.linkage)?;
        }
        if self.is_constant {
            write!(f, "constant")?;
        } else {
            write!(f, "global")?;
        }
        write!(f, " {}", self.ty)?;
        if let Some(init) = &self.init {
            write!(f, " {}", init)?;
        }
        if let Some(a) = self.align {
            write!(f, ", align {}", a)?;
        }
        Ok(())
    }
}
