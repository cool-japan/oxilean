//! # OcamlExpr - Trait Implementations
//!
//! This module contains trait implementations for `OcamlExpr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::format_ocaml_expr;
use super::types::OcamlExpr;
use std::fmt;

impl fmt::Display for OcamlExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format_ocaml_expr(self, 0))
    }
}
