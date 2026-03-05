//! # MemSnapshotDiff - Trait Implementations
//!
//! This module contains trait implementations for `MemSnapshotDiff`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MemSnapshotDiff;
use std::fmt;

impl std::fmt::Display for MemSnapshotDiff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} → {}: {:+} objects, {:+} bytes",
            self.from_label, self.to_label, self.delta_objects, self.delta_bytes
        )
    }
}
