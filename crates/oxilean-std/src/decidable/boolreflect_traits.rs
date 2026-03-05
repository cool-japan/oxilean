//! # BoolReflect - Trait Implementations
//!
//! This module contains trait implementations for `BoolReflect`.
//!
//! ## Implemented Traits
//!
//! - `From`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::BoolReflect;

impl From<bool> for BoolReflect {
    fn from(b: bool) -> Self {
        if b {
            BoolReflect::IsTrue
        } else {
            BoolReflect::IsFalse
        }
    }
}
