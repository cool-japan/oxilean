//! # MonomialV2 - Trait Implementations
//!
//! This module contains trait implementations for `MonomialV2`.
//!
//! ## Implemented Traits
//!
//! - `PartialOrd`
//! - `Ord`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MonomialV2;

impl PartialOrd for MonomialV2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MonomialV2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        for (&a, &b) in self.exponents.iter().zip(other.exponents.iter()) {
            match a.cmp(&b) {
                std::cmp::Ordering::Equal => continue,
                o => return o,
            }
        }
        std::cmp::Ordering::Equal
    }
}
