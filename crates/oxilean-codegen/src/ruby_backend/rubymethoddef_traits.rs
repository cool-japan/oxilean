//! # RubyMethodDef - Trait Implementations
//!
//! This module contains trait implementations for `RubyMethodDef`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::{fmt_ruby_class, fmt_ruby_method, fmt_ruby_module_stmt, fmt_ruby_stmt};
use super::types::{RubyMethodDef, RubyVisibility};
use std::fmt;

impl std::fmt::Display for RubyMethodDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<String> = self
            .params
            .iter()
            .map(|(n, t)| {
                if let Some(t) = t {
                    format!("{}: T.cast(nil, {})", n, t)
                } else {
                    n.clone()
                }
            })
            .collect();
        let self_prefix = if self.is_class_method { "self." } else { "" };
        write!(
            f,
            "{}def {}{}({})\n  {}\nend",
            if self.visibility != RubyVisibility::Public {
                format!("{}\n", self.visibility)
            } else {
                String::new()
            },
            self_prefix,
            self.name,
            params.join(", "),
            self.body,
        )
    }
}
