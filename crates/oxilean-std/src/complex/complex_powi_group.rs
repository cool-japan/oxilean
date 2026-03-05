//! # Complex - powi_group Methods
//!
//! This module contains method implementations for `Complex`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::complex_type::Complex;

impl Complex {
    /// Complex power z^n for integer n (fast exponentiation).
    pub fn powi(self, n: i32) -> Self {
        if n == 0 {
            return Self::one();
        }
        if n < 0 {
            let inv = self.div(Self::one()).unwrap_or(Self::zero());
            return inv.powi(-n);
        }
        let mut result = Self::one();
        let mut base = self;
        let mut exp = n as u32;
        while exp > 0 {
            if exp % 2 == 1 {
                result = result.mul(base);
            }
            base = base.mul(base);
            exp /= 2;
        }
        result
    }
}
