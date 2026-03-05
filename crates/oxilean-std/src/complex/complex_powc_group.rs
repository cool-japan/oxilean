//! # Complex - powc_group Methods
//!
//! This module contains method implementations for `Complex`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::complex_type::Complex;

impl Complex {
    /// Complex power z^w = exp(w * log(z)).
    pub fn powc(self, w: Self) -> Option<Self> {
        Some(w.mul(self.log()?).exp())
    }
}
