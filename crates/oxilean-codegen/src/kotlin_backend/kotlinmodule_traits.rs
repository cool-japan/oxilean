//! # KotlinModule - Trait Implementations
//!
//! This module contains trait implementations for `KotlinModule`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::KOTLIN_RUNTIME;
use super::functions::{fmt_stmt, fmt_stmts};
use super::types::KotlinModule;
use std::fmt;

impl fmt::Display for KotlinModule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.package.is_empty() {
            writeln!(f, "package {}", self.package)?;
            writeln!(f)?;
        }
        for import in &self.imports {
            writeln!(f, "import {}", import)?;
        }
        if !self.imports.is_empty() {
            writeln!(f)?;
        }
        writeln!(f, "{}", KOTLIN_RUNTIME)?;
        for dc in &self.data_classes {
            writeln!(f, "{}", dc)?;
        }
        for func in &self.funs {
            writeln!(f, "{}", func)?;
        }
        Ok(())
    }
}
