//! # OcamlSignature - Trait Implementations
//!
//! This module contains trait implementations for `OcamlSignature`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::format_ocaml_expr;
use super::types::OcamlSignature;
use std::fmt;

impl fmt::Display for OcamlSignature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "module type {} = sig", self.name)?;
        for item in &self.items {
            writeln!(f, "  {}", item)?;
        }
        write!(f, "end")
    }
}
