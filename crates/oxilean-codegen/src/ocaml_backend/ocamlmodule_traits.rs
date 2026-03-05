//! # OcamlModule - Trait Implementations
//!
//! This module contains trait implementations for `OcamlModule`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::format_ocaml_expr;
use super::types::OcamlModule;
use std::fmt;

impl fmt::Display for OcamlModule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.emit())
    }
}
