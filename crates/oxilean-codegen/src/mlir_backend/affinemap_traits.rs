//! # AffineMap - Trait Implementations
//!
//! This module contains trait implementations for `AffineMap`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::AffineMap;
use std::fmt;

impl fmt::Display for AffineMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AffineMap::Identity(n) => {
                let dims: Vec<String> = (0..*n).map(|i| format!("d{}", i)).collect();
                let dims_str = dims.join(", ");
                write!(f, "affine_map<({}) -> ({})>", dims_str, dims_str)
            }
            AffineMap::Constant => write!(f, "affine_map<() -> ()>"),
            AffineMap::Custom(s) => write!(f, "affine_map<{}>", s),
        }
    }
}
