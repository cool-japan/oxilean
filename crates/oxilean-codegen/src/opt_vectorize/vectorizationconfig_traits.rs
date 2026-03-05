//! # VectorizationConfig - Trait Implementations
//!
//! This module contains trait implementations for `VectorizationConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{SIMDTarget, VectorWidth, VectorizationConfig};
use std::fmt;

impl Default for VectorizationConfig {
    fn default() -> Self {
        VectorizationConfig {
            min_trip_count: 8,
            preferred_width: VectorWidth::W256,
            enable_fma: true,
            vectorize_reductions: true,
            target: SIMDTarget::X86AVX,
        }
    }
}

impl fmt::Display for VectorizationConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "VectorizationConfig {{ min_trip={}, width={}, fma={}, reductions={}, target={} }}",
            self.min_trip_count,
            self.preferred_width,
            self.enable_fma,
            self.vectorize_reductions,
            self.target,
        )
    }
}
