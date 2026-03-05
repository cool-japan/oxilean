//! # FfiEnumDef - Trait Implementations
//!
//! This module contains trait implementations for `FfiEnumDef`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::FfiEnumDef;
use std::fmt;

impl std::fmt::Display for FfiEnumDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ty = if let Some(t) = &self.underlying_type {
            format!(" : {}", t)
        } else {
            String::new()
        };
        writeln!(f, "enum {}{} {{", self.name, ty)?;
        for (n, v) in &self.variants {
            if let Some(val) = v {
                writeln!(f, "    {} = {},", n, val)?;
            } else {
                writeln!(f, "    {},", n)?;
            }
        }
        write!(f, "}};")
    }
}
