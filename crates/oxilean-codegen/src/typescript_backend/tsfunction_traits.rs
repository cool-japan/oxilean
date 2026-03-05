//! # TsFunction - Trait Implementations
//!
//! This module contains trait implementations for `TsFunction`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::{format_ts_stmt, format_ts_stmts};
use super::types::TsFunction;
use std::fmt;

impl fmt::Display for TsFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_exported {
            write!(f, "export ")?;
        }
        if self.is_async {
            write!(f, "async ")?;
        }
        write!(f, "function {}", self.name)?;
        if !self.type_params.is_empty() {
            write!(f, "<{}>", self.type_params.join(", "))?;
        }
        write!(f, "(")?;
        for (i, p) in self.params.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            if p.rest {
                write!(f, "...")?;
            }
            write!(f, "{}", p.name)?;
            if p.optional {
                write!(f, "?")?;
            }
            write!(f, ": {}", p.ty)?;
        }
        writeln!(f, "): {} {{", self.return_type)?;
        let body_text = format_ts_stmts(&self.body, 2);
        if !body_text.is_empty() {
            writeln!(f, "{}", body_text)?;
        }
        write!(f, "}}")
    }
}
