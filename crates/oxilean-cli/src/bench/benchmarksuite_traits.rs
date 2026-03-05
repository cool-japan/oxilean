//! # BenchmarkSuite - Trait Implementations
//!
//! This module contains trait implementations for `BenchmarkSuite`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::BenchmarkSuite;
use std::fmt;

impl fmt::Display for BenchmarkSuite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "=== {} ===", self.name)?;
        for r in &self.results {
            writeln!(f, "  {}", r)?;
        }
        Ok(())
    }
}
