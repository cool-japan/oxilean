//! # Complex - norm_sq_group Methods
//!
//! This module contains method implementations for `Complex`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::complex_type::Complex;

impl Complex {
    /// Modulus squared: |z|² = re² + im².
    pub fn norm_sq(self) -> f64 {
        self.re * self.re + self.im * self.im
    }
    /// Modulus: |z|.
    pub fn abs(self) -> f64 {
        self.norm_sq().sqrt()
    }
    /// Division: z/w = z * conj(w) / |w|².
    pub fn div(self, other: Self) -> Option<Self> {
        let denom = other.norm_sq();
        if denom < f64::EPSILON {
            return None;
        }
        Some(self.mul(other.conj()).scale(1.0 / denom))
    }
}
