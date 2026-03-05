//! # RubyLazyEnumerator - Trait Implementations
//!
//! This module contains trait implementations for `RubyLazyEnumerator`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::{fmt_ruby_class, fmt_ruby_method, fmt_ruby_module_stmt, fmt_ruby_stmt};
use super::types::RubyLazyEnumerator;
use std::fmt;

impl std::fmt::Display for RubyLazyEnumerator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chain: Vec<String> = std::iter::once(format!("{}.lazy", self.source))
            .chain(self.transforms.iter().cloned())
            .collect();
        write!(f, "{}", chain.join("."))
    }
}
