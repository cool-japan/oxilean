//! # WhnfReductionOrder - Trait Implementations
//!
//! This module contains trait implementations for `WhnfReductionOrder`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::WhnfReductionOrder;

impl std::fmt::Display for WhnfReductionOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WhnfReductionOrder::BetaFirst => write!(f, "beta-first"),
            WhnfReductionOrder::DeltaFirst => write!(f, "delta-first"),
            WhnfReductionOrder::StructuralOnly => write!(f, "structural-only"),
        }
    }
}
