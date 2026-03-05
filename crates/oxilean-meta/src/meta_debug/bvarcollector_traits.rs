//! # BVarCollector - Trait Implementations
//!
//! This module contains trait implementations for `BVarCollector`.
//!
//! ## Implemented Traits
//!
//! - `ExprVisitor`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::ExprVisitor;
use super::types::BVarCollector;

impl ExprVisitor for BVarCollector {
    fn visit_bvar(&mut self, idx: u32) {
        self.0.push(idx);
    }
}
