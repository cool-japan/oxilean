//! # LoopNode - Trait Implementations
//!
//! This module contains trait implementations for `LoopNode`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::LoopNode;
use std::fmt;

impl std::fmt::Display for LoopNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tc = self
            .trip_count
            .map(|t| t.to_string())
            .unwrap_or("?".to_string());
        write!(
            f,
            "Loop#{}(header={}, depth={}, trip={}, inner={})",
            self.id.0, self.header, self.depth, tc, self.is_inner_most,
        )
    }
}
