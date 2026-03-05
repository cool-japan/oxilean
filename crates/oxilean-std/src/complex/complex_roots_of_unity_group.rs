//! # Complex - roots_of_unity_group Methods
//!
//! This module contains method implementations for `Complex`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::f64::consts::PI;

use super::complex_type::Complex;

impl Complex {
    /// n-th roots of unity: e^(2πi·k/n) for k = 0..n-1.
    pub fn roots_of_unity(n: u32) -> Vec<Self> {
        if n == 0 {
            return vec![];
        }
        (0..n)
            .map(|k| Self::from_polar(1.0, 2.0 * PI * k as f64 / n as f64))
            .collect()
    }
}
