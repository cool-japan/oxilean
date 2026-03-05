//! # KotlinDataClass - Trait Implementations
//!
//! This module contains trait implementations for `KotlinDataClass`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::{fmt_stmt, fmt_stmts};
use super::types::KotlinDataClass;
use std::fmt;

impl fmt::Display for KotlinDataClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "data class {}(", self.name)?;
        for (i, (name, ty)) in self.fields.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "val {}: {}", name, ty)?;
        }
        writeln!(f, ")")
    }
}
