//! # IoSessionStats - Trait Implementations
//!
//! This module contains trait implementations for `IoSessionStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::IoSessionStats;
use std::fmt;

impl fmt::Display for IoSessionStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "IoSessionStats {{ reads: {}, writes: {}, bytes_read: {}, bytes_written: {} }}",
            self.reads, self.writes, self.bytes_read, self.bytes_written
        )
    }
}
