//! # Complex - atan_group Methods
//!
//! This module contains method implementations for `Complex`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::complex_type::Complex;

impl Complex {
    /// Inverse tangent: arctan(z) = (i/2) * log((i+z)/(i-z)).
    pub fn atan(self) -> Option<Self> {
        let i = Self::i();
        let numer = i.add(self);
        let denom = i.sub(self);
        let ratio = numer.div(denom)?;
        let log = ratio.log()?;
        Some(i.scale(0.5).mul(log))
    }
}
