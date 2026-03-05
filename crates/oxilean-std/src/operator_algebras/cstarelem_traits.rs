//! # CStarElem - Trait Implementations
//!
//! This module contains trait implementations for `CStarElem`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CStarElem;
use std::fmt;

impl std::fmt::Display for CStarElem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CStarElem[{}](norm={:.4}, sa={}, normal={}, unitary={}, proj={})",
            self.label,
            self.norm,
            self.is_self_adjoint,
            self.is_normal,
            self.is_unitary,
            self.is_projection
        )
    }
}
