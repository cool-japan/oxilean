//! # Complex - nth_roots_group Methods
//!
//! This module contains method implementations for `Complex`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::f64::consts::PI;

use super::complex_type::Complex;

impl Complex {
    /// n-th roots of z: returns all n roots.
    pub fn nth_roots(self, n: u32) -> Vec<Self> {
        if n == 0 {
            return vec![];
        }
        let (r, theta) = self.to_polar();
        let r_n = r.powf(1.0 / n as f64);
        (0..n)
            .map(|k| {
                let angle = (theta + 2.0 * PI * k as f64) / n as f64;
                Self::from_polar(r_n, angle)
            })
            .collect()
    }
}
