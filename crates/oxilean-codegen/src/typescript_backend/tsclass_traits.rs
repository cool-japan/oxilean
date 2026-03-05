//! # TsClass - Trait Implementations
//!
//! This module contains trait implementations for `TsClass`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::{format_ts_stmt, format_ts_stmts};
use super::types::TsClass;
use std::fmt;

impl fmt::Display for TsClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_exported {
            write!(f, "export ")?;
        }
        write!(f, "class {}", self.name)?;
        if !self.type_params.is_empty() {
            write!(f, "<{}>", self.type_params.join(", "))?;
        }
        if let Some(base) = &self.extends {
            write!(f, " extends {}", base)?;
        }
        if !self.implements.is_empty() {
            write!(f, " implements {}", self.implements.join(", "))?;
        }
        writeln!(f, " {{")?;
        for field in &self.fields {
            write!(f, "  ")?;
            if field.is_private {
                write!(f, "private ")?;
            }
            if field.is_static {
                write!(f, "static ")?;
            }
            if field.readonly {
                write!(f, "readonly ")?;
            }
            write!(f, "{}", field.name)?;
            if field.optional {
                write!(f, "?")?;
            }
            writeln!(f, ": {};", field.ty)?;
        }
        for method in &self.methods {
            writeln!(f, "{}", method)?;
        }
        write!(f, "}}")
    }
}
