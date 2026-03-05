//! # TsTypeAlias - Trait Implementations
//!
//! This module contains trait implementations for `TsTypeAlias`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::{format_ts_stmt, format_ts_stmts};
use super::types::TsTypeAlias;
use std::fmt;

impl fmt::Display for TsTypeAlias {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "type {}", self.name)?;
        if !self.type_params.is_empty() {
            write!(f, "<{}>", self.type_params.join(", "))?;
        }
        write!(f, " = {};", self.definition)
    }
}
