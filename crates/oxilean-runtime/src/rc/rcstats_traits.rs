//! # RcStats - Trait Implementations
//!
//! This module contains trait implementations for `RcStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::RcStats;
use std::fmt;

impl fmt::Display for RcStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "RC Statistics:")?;
        writeln!(f, "  Increments:      {}", self.increments)?;
        writeln!(f, "  Decrements:      {}", self.decrements)?;
        writeln!(f, "  Deallocations:   {}", self.deallocations)?;
        writeln!(f, "  Elided inc:      {}", self.elided_increments)?;
        writeln!(f, "  Elided dec:      {}", self.elided_decrements)?;
        writeln!(f, "  In-place mut:    {}", self.inplace_mutations)?;
        writeln!(f, "  Copy-on-write:   {}", self.copy_on_write)?;
        writeln!(f, "  Peak RC:         {}", self.peak_rc)?;
        writeln!(f, "  Elision ratio:   {:.1}%", self.elision_ratio() * 100.0)
    }
}
