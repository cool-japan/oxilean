//! # SurfaceIdent - Trait Implementations
//!
//! This module contains trait implementations for `SurfaceIdent`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SurfaceIdent;
use std::fmt;

impl fmt::Display for SurfaceIdent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
