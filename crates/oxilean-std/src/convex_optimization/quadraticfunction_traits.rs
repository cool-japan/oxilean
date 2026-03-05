//! # QuadraticFunction - Trait Implementations
//!
//! This module contains trait implementations for `QuadraticFunction`.
//!
//! ## Implemented Traits
//!
//! - `ConvexFunction`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::ConvexFunction;
use super::types::QuadraticFunction;

impl ConvexFunction for QuadraticFunction {
    /// f(x) = 0.5 * x^T Q x + c^T x + d.
    fn eval(&self, x: &[f64]) -> f64 {
        let n = x.len();
        let mut quad = 0.0_f64;
        for i in 0..n {
            for j in 0..n {
                quad += x[i] * self.coeffs[i][j] * x[j];
            }
        }
        let linear: f64 = self.linear.iter().zip(x).map(|(c, xi)| c * xi).sum();
        0.5 * quad + linear + self.constant
    }
    /// ∇f(x) = Q x + c.
    fn gradient(&self, x: &[f64]) -> Vec<f64> {
        let n = x.len();
        let mut grad = vec![0.0_f64; n];
        for i in 0..n {
            for j in 0..n {
                grad[i] += self.coeffs[i][j] * x[j];
            }
            grad[i] += self.linear[i];
        }
        grad
    }
    /// A quadratic function is strongly convex iff Q is positive definite (approximated
    /// here by checking that all diagonal entries are strictly positive).
    fn is_strongly_convex(&self) -> bool {
        self.coeffs.iter().enumerate().all(|(i, row)| row[i] > 0.0)
    }
}
