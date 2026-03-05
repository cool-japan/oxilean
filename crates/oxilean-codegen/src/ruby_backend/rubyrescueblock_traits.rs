//! # RubyRescueBlock - Trait Implementations
//!
//! This module contains trait implementations for `RubyRescueBlock`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::{fmt_ruby_class, fmt_ruby_method, fmt_ruby_module_stmt, fmt_ruby_stmt};
use super::types::RubyRescueBlock;
use std::fmt;

impl std::fmt::Display for RubyRescueBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "begin")?;
        writeln!(f, "  {}", self.body)?;
        for (types, var, body) in &self.rescues {
            let types_str = if types.is_empty() {
                String::new()
            } else {
                format!(" {}", types.join(", "))
            };
            let var_str = if let Some(v) = var {
                format!(" => {}", v)
            } else {
                String::new()
            };
            writeln!(f, "rescue{}{}", types_str, var_str)?;
            writeln!(f, "  {}", body)?;
        }
        if let Some(ens) = &self.ensure {
            writeln!(f, "ensure")?;
            writeln!(f, "  {}", ens)?;
        }
        write!(f, "end")
    }
}
