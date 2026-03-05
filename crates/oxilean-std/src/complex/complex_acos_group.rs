//! # Complex - acos_group Methods
//!
//! This module contains method implementations for `Complex`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::complex_type::Complex;

impl Complex {
    /// Inverse cosine: arccos(z) = -i * log(z + i*sqrt(1 - z^2)).
    pub fn acos(self) -> Option<Self> {
        let one = Self::one();
        let z2 = self.mul(self);
        let inner = one.sub(z2).sqrt();
        let arg = self.add(Self::i().mul(inner));
        let log = arg.log()?;
        Some(Self::i().neg().mul(log))
    }
}
