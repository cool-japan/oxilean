//! # IoStats - Trait Implementations
//!
//! This module contains trait implementations for `IoStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::IoStats;
use std::fmt;

impl fmt::Display for IoStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "I/O Statistics:")?;
        writeln!(f, "  File reads:       {}", self.file_reads)?;
        writeln!(f, "  File writes:      {}", self.file_writes)?;
        writeln!(f, "  Console outputs:  {}", self.console_outputs)?;
        writeln!(f, "  Console inputs:   {}", self.console_inputs)?;
        writeln!(f, "  Exceptions thrown: {}", self.exceptions_thrown)?;
        writeln!(f, "  Exceptions caught: {}", self.exceptions_caught)?;
        writeln!(f, "  Refs created:     {}", self.refs_created)?;
        writeln!(f, "  Bytes read:       {}", self.bytes_read)?;
        writeln!(f, "  Bytes written:    {}", self.bytes_written)
    }
}
