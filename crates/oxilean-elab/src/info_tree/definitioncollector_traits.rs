//! # DefinitionCollector - Trait Implementations
//!
//! This module contains trait implementations for `DefinitionCollector`.
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
use super::types::{DefinitionCollector, Info, InfoData, LocalContextEntry};

impl Default for DefinitionCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl InfoTreeWalker for DefinitionCollector {
    fn visit_node(&mut self, info: &Info, _depth: usize) -> bool {
        if let InfoData::CommandInfo {
            decl_name: Some(name),
            ..
        } = &info.data
        {
            self.definitions.push((name.clone(), info.stx_range));
        }
        true
    }
    fn visit_hole(&mut self, _expected_type: &Option<Expr>, _range: &Option<(usize, usize)>) {}
    fn enter_context(&mut self, _lctx: &[LocalContextEntry]) {}
    fn leave_context(&mut self) {}
}
