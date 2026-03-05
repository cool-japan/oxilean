//! # FfiStructDef - Trait Implementations
//!
//! This module contains trait implementations for `FfiStructDef`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::FfiStructDef;
use std::fmt;

impl std::fmt::Display for FfiStructDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kw = if self.is_union { "union" } else { "struct" };
        let packed = if self.is_packed {
            " __attribute__((packed))"
        } else {
            ""
        };
        let align = if let Some(a) = self.alignment {
            format!(" __attribute__((aligned({})))", a)
        } else {
            String::new()
        };
        writeln!(f, "{} {}{}{} {{", kw, self.name, packed, align)?;
        for fld in &self.fields {
            writeln!(f, "    {};", fld)?;
        }
        write!(f, "}};")
    }
}
