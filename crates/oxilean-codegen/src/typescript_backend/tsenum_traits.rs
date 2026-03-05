//! # TsEnum - Trait Implementations
//!
//! This module contains trait implementations for `TsEnum`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::{format_ts_stmt, format_ts_stmts};
use super::types::TsEnum;
use std::fmt;

impl fmt::Display for TsEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_const {
            write!(f, "const ")?;
        }
        writeln!(f, "enum {} {{", self.name)?;
        for m in &self.members {
            if let Some(val) = &m.value {
                writeln!(f, "  {} = {},", m.name, val)?;
            } else {
                writeln!(f, "  {},", m.name)?;
            }
        }
        write!(f, "}}")
    }
}
