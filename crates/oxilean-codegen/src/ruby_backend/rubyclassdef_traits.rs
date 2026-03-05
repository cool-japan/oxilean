//! # RubyClassDef - Trait Implementations
//!
//! This module contains trait implementations for `RubyClassDef`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::{fmt_ruby_class, fmt_ruby_method, fmt_ruby_module_stmt, fmt_ruby_stmt};
use super::types::RubyClassDef;
use std::fmt;

impl std::fmt::Display for RubyClassDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let super_str = if let Some(s) = &self.superclass {
            format!(" < {}", s)
        } else {
            String::new()
        };
        writeln!(f, "class {}{}", self.name, super_str)?;
        for inc in &self.includes {
            writeln!(f, "  include {}", inc)?;
        }
        for ext in &self.extends {
            writeln!(f, "  extend {}", ext)?;
        }
        for prep in &self.prepends {
            writeln!(f, "  prepend {}", prep)?;
        }
        for (n, r, w) in &self.attrs {
            if *r && *w {
                writeln!(f, "  attr_accessor :{}", n)?;
            } else if *r {
                writeln!(f, "  attr_reader :{}", n)?;
            } else if *w {
                writeln!(f, "  attr_writer :{}", n)?;
            }
        }
        for (n, v) in &self.constants {
            writeln!(f, "  {} = {}", n, v)?;
        }
        for m in &self.methods {
            writeln!(f, "  {}", m)?;
        }
        write!(f, "end")
    }
}
