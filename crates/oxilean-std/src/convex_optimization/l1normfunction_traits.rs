//! # L1NormFunction - Trait Implementations
//!
//! This module contains trait implementations for `L1NormFunction`.
//!
//! ## Implemented Traits
//!
//! - `ConvexFunction`
//! - `ProxableFunction`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::{ConvexFunction, ProxableFunction};
use super::types::L1NormFunction;

impl ConvexFunction for L1NormFunction {
    fn eval(&self, x: &[f64]) -> f64 {
        self.lambda * x.iter().map(|xi| xi.abs()).sum::<f64>()
    }
    /// Subgradient: sign(x_i) · λ (0 at x_i = 0 is replaced by 0.0).
    fn gradient(&self, x: &[f64]) -> Vec<f64> {
        x.iter().map(|xi| self.lambda * xi.signum()).collect()
    }
    fn is_strongly_convex(&self) -> bool {
        false
    }
}

impl ProxableFunction for L1NormFunction {
    /// Soft thresholding: prox_{t·λ‖·‖₁}(v)_i = sign(v_i)·max(|v_i| - t·λ, 0).
    fn prox(&self, v: &[f64], t: f64) -> Vec<f64> {
        let threshold = t * self.lambda;
        v.iter()
            .map(|vi| vi.signum() * (vi.abs() - threshold).max(0.0))
            .collect()
    }
}
