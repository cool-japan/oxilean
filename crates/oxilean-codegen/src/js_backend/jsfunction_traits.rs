//! # JsFunction - Trait Implementations
//!
//! This module contains trait implementations for `JsFunction`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::*;
use super::types::JsFunction;
use std::fmt;

impl fmt::Display for JsFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_export {
            write!(f, "export ")?;
        }
        if self.is_async {
            write!(f, "async ")?;
        }
        write!(f, "function {}(", self.name)?;
        for (i, p) in self.params.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", p)?;
        }
        writeln!(f, ") {{")?;
        let body_text = display_indented(&self.body, 2);
        if !body_text.is_empty() {
            writeln!(f, "{}", body_text)?;
        }
        write!(f, "}}")
    }
}
