//! # NameUsageCollector - Trait Implementations
//!
//! This module contains trait implementations for `NameUsageCollector`.
//!
//! ## Implemented Traits
//!
//! - `InfoTreeWalker`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Expr, Level, Name};
use std::fmt;

use super::functions::InfoTreeWalker;
use super::functions::{expr_references_name, references_name};
use super::types::{Info, LocalContextEntry, NameUsageCollector};

impl InfoTreeWalker for NameUsageCollector {
    fn visit_node(&mut self, info: &Info, _depth: usize) -> bool {
        if references_name(&info.data, &self.target) {
            self.usages.push(info.stx_range);
        }
        true
    }
    fn visit_hole(&mut self, _expected_type: &Option<Expr>, _range: &Option<(usize, usize)>) {}
    fn enter_context(&mut self, _lctx: &[LocalContextEntry]) {}
    fn leave_context(&mut self) {}
}
