//! # ArenaStats - Trait Implementations
//!
//! This module contains trait implementations for `ArenaStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ArenaStats;
use std::fmt;

impl fmt::Display for ArenaStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Arena Statistics:")?;
        writeln!(f, "  Total allocations: {}", self.total_allocations)?;
        writeln!(f, "  Total bytes:       {}", self.total_bytes_allocated)?;
        writeln!(f, "  Total resets:      {}", self.total_resets)?;
        writeln!(f, "  Avg alloc size:    {:.1}", self.avg_alloc_size())
    }
}
