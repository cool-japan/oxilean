//! # ParallelPattern - Trait Implementations
//!
//! This module contains trait implementations for `ParallelPattern`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ParallelPattern;
use std::fmt;

impl fmt::Display for ParallelPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParallelPattern::Map => write!(f, "map"),
            ParallelPattern::Filter => write!(f, "filter"),
            ParallelPattern::Reduce => write!(f, "reduce"),
            ParallelPattern::Scan => write!(f, "scan"),
            ParallelPattern::Stencil => write!(f, "stencil"),
            ParallelPattern::ParallelFor => write!(f, "parallel-for"),
            ParallelPattern::Scatter => write!(f, "scatter"),
            ParallelPattern::Gather => write!(f, "gather"),
        }
    }
}
