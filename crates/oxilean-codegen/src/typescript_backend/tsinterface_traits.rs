//! # TsInterface - Trait Implementations
//!
//! This module contains trait implementations for `TsInterface`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::{format_ts_stmt, format_ts_stmts};
use super::types::TsInterface;
use std::fmt;

impl fmt::Display for TsInterface {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "interface {}", self.name)?;
        if !self.type_params.is_empty() {
            write!(f, "<{}>", self.type_params.join(", "))?;
        }
        if !self.extends.is_empty() {
            write!(f, " extends {}", self.extends.join(", "))?;
        }
        writeln!(f, " {{")?;
        for m in &self.members {
            if m.readonly {
                write!(f, "  readonly ")?;
            } else {
                write!(f, "  ")?;
            }
            write!(f, "{}", m.name)?;
            if m.optional {
                write!(f, "?")?;
            }
            writeln!(f, ": {};", m.ty)?;
        }
        write!(f, "}}")
    }
}
