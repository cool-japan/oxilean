//! # CountingVisitorExt - Trait Implementations
//!
//! This module contains trait implementations for `CountingVisitorExt`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `AstNodeVisitorExt`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::AstNodeVisitorExt;
use super::types::CountingVisitorExt;

impl Default for CountingVisitorExt {
    fn default() -> Self {
        Self::new()
    }
}

impl AstNodeVisitorExt for CountingVisitorExt {
    fn visit_node(&mut self, kind: &str, _depth: usize) {
        *self.counts.entry(kind.to_string()).or_insert(0) += 1;
    }
}
