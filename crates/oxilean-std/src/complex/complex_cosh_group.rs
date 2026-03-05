//! # Complex - cosh_group Methods
//!
//! This module contains method implementations for `Complex`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::complex_type::Complex;

impl Complex {
    /// Hyperbolic cosine: cosh(z) = (e^z + e^(-z)) / 2.
    pub fn cosh(self) -> Self {
        let ep = self.exp();
        let em = self.neg().exp();
        ep.add(em).scale(0.5)
    }
}
