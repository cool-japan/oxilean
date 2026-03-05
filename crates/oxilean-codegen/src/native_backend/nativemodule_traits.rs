//! # NativeModule - Trait Implementations
//!
//! This module contains trait implementations for `NativeModule`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::NativeModule;
use std::fmt;

impl fmt::Display for NativeModule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "; module: {}", self.name)?;
        for (name, params, ret) in &self.extern_fns {
            write!(f, "declare @{}(", name)?;
            for (i, ty) in params.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", ty)?;
            }
            writeln!(f, ") -> {}", ret)?;
        }
        writeln!(f)?;
        for func in &self.functions {
            write!(f, "{}", func)?;
        }
        Ok(())
    }
}
