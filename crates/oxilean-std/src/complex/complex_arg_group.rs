//! # Complex - arg_group Methods
//!
//! This module contains method implementations for `Complex`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::complex_type::Complex;

impl Complex {
    /// Argument: arg(z) ∈ (-π, π].
    pub fn arg(self) -> f64 {
        self.im.atan2(self.re)
    }
    /// Convert to polar form (r, θ).
    pub fn to_polar(self) -> (f64, f64) {
        (self.abs(), self.arg())
    }
}
