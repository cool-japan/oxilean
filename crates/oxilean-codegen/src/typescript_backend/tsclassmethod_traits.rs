//! # TsClassMethod - Trait Implementations
//!
//! This module contains trait implementations for `TsClassMethod`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::{format_ts_stmt, format_ts_stmts};
use super::types::TsClassMethod;
use std::fmt;

impl fmt::Display for TsClassMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "  ")?;
        if self.is_private {
            write!(f, "private ")?;
        }
        if self.is_static {
            write!(f, "static ")?;
        }
        if self.is_async {
            write!(f, "async ")?;
        }
        if self.is_getter {
            write!(f, "get ")?;
        } else if self.is_setter {
            write!(f, "set ")?;
        }
        write!(f, "{}(", self.name)?;
        for (i, p) in self.params.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", p.name, p.ty)?;
        }
        writeln!(f, "): {} {{", self.return_type)?;
        let body_text = format_ts_stmts(&self.body, 4);
        if !body_text.is_empty() {
            writeln!(f, "{}", body_text)?;
        }
        write!(f, "  }}")
    }
}
