//! # FutharkIota - Trait Implementations
//!
//! This module contains trait implementations for `FutharkIota`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FutharkIota;
use std::fmt;

impl std::fmt::Display for FutharkIota {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.start, &self.step) {
            (None, None) => write!(f, "iota {}", self.n),
            (Some(s), None) => write!(f, "iota {} {} 1", self.n, s),
            (Some(s), Some(st)) => write!(f, "iota {} {} {}", self.n, s, st),
            (None, Some(st)) => write!(f, "iota {} 0 {}", self.n, st),
        }
    }
}
