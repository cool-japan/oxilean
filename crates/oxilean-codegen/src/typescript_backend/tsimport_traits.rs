//! # TsImport - Trait Implementations
//!
//! This module contains trait implementations for `TsImport`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::{format_ts_stmt, format_ts_stmts};
use super::types::TsImport;
use std::fmt;

impl fmt::Display for TsImport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_type {
            write!(
                f,
                "import type {{ {} }} from \"{}\";",
                self.names.join(", "),
                self.from
            )
        } else {
            write!(
                f,
                "import {{ {} }} from \"{}\";",
                self.names.join(", "),
                self.from
            )
        }
    }
}
