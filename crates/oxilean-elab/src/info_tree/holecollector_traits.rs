//! # HoleCollector - Trait Implementations
//!
//! This module contains trait implementations for `HoleCollector`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `InfoTreeWalker`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Expr, Level, Name};
use std::fmt;

use super::functions::InfoTreeWalker;
use super::functions::{expr_references_name, references_name};
use super::types::{HoleCollector, Info, LocalContextEntry};

impl Default for HoleCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl InfoTreeWalker for HoleCollector {
    fn visit_node(&mut self, _info: &Info, _depth: usize) -> bool {
        true
    }
    fn visit_hole(&mut self, expected_type: &Option<Expr>, range: &Option<(usize, usize)>) {
        self.holes.push((expected_type.clone(), *range));
    }
    fn enter_context(&mut self, _lctx: &[LocalContextEntry]) {}
    fn leave_context(&mut self) {}
}
