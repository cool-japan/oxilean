//! # Complex - asin_group Methods
//!
//! This module contains method implementations for `Complex`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::complex_type::Complex;

impl Complex {
    /// Inverse sine: arcsin(z) = -i * log(iz + sqrt(1 - z^2)).
    pub fn asin(self) -> Option<Self> {
        let one = Self::one();
        let z2 = self.mul(self);
        let inner = one.sub(z2).sqrt();
        let iz = Self::i().mul(self);
        let arg = iz.add(inner);
        let log = arg.log()?;
        Some(Self::i().neg().mul(log))
    }
}
