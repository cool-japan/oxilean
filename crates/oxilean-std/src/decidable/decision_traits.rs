//! # Decision - Trait Implementations
//!
//! This module contains trait implementations for `Decision`.
//!
//! ## Implemented Traits
//!
//! - `From`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Decision;

impl From<bool> for Decision<bool> {
    fn from(b: bool) -> Self {
        if b {
            Decision::IsTrue(true)
        } else {
            Decision::IsFalse("false".to_string())
        }
    }
}
