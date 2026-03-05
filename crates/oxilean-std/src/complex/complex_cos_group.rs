//! # Complex - cos_group Methods
//!
//! This module contains method implementations for `Complex`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::complex_type::Complex;

impl Complex {
    /// Complex cosine: cos(z) = (e^(iz) + e^(-iz)) / 2.
    pub fn cos(self) -> Self {
        let iz = Self::i().mul(self);
        let e1 = iz.exp();
        let e2 = iz.neg().exp();
        e1.add(e2).scale(0.5)
    }
    /// Complex tangent: tan(z) = sin(z)/cos(z).
    pub fn tan(self) -> Option<Self> {
        self.sin().div(self.cos())
    }
}
