//! # Complex - sinh_group Methods
//!
//! This module contains method implementations for `Complex`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::complex_type::Complex;

impl Complex {
    /// Hyperbolic sine: sinh(z) = (e^z - e^(-z)) / 2.
    pub fn sinh(self) -> Self {
        let ep = self.exp();
        let em = self.neg().exp();
        ep.sub(em).scale(0.5)
    }
    /// Hyperbolic tangent: tanh(z) = sinh(z)/cosh(z).
    pub fn tanh(self) -> Option<Self> {
        self.sinh().div(self.cosh())
    }
}
