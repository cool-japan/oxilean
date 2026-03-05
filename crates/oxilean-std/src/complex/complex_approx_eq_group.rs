//! # Complex - approx_eq_group Methods
//!
//! This module contains method implementations for `Complex`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::complex_type::Complex;

impl Complex {
    /// Is z approximately equal to w?
    pub fn approx_eq(self, other: Self, eps: f64) -> bool {
        self.sub(other).abs() < eps
    }
}
