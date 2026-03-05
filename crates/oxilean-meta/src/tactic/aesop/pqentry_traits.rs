//! # PQEntry - Trait Implementations
//!
//! This module contains trait implementations for `PQEntry`.
//!
//! ## Implemented Traits
//!
//! - `PartialEq`
//! - `Eq`
//! - `PartialOrd`
//! - `Ord`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::cmp::Ordering;

use super::types::PQEntry;

impl PartialEq for PQEntry {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.node_id == other.node_id
    }
}

impl Eq for PQEntry {}

impl PartialOrd for PQEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PQEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .partial_cmp(&self.cost)
            .unwrap_or(Ordering::Equal)
            .then_with(|| other.node_id.0.cmp(&self.node_id.0))
    }
}
