//! # AppCounter - Trait Implementations
//!
//! This module contains trait implementations for `AppCounter`.
//!
//! ## Implemented Traits
//!
//! - `ExprVisitor`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::ExprVisitor;
use super::types::AppCounter;

impl ExprVisitor for AppCounter {
    fn visit_app(&mut self) {
        self.0 += 1;
    }
}
