//! # FutharkUnsafeCoerce - Trait Implementations
//!
//! This module contains trait implementations for `FutharkUnsafeCoerce`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FutharkUnsafeCoerce;
use std::fmt;

impl std::fmt::Display for FutharkUnsafeCoerce {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#[unsafe] ({} :> {})", self.value, self.to_type)
    }
}
