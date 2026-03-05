//! # YulFunction - Trait Implementations
//!
//! This module contains trait implementations for `YulFunction`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::YulFunction;
use std::fmt;

impl std::fmt::Display for YulFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params = self.params.join(", ");
        let returns = if self.returns.is_empty() {
            String::new()
        } else {
            format!(" -> {}", self.returns.join(", "))
        };
        write!(f, "function {}({}){}", self.name, params, returns)?;
        write!(f, " {{")?;
        for stmt in &self.body {
            write!(f, "\n  {}", stmt)?;
        }
        write!(f, "\n}}")
    }
}
