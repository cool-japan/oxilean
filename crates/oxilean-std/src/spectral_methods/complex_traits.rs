//! # Complex - Trait Implementations
//!
//! This module contains trait implementations for `Complex`.
//!
//! ## Implemented Traits
//!
//! - `Add`
//! - `Sub`
//! - `Mul`
//! - `Mul`
//! - `Div`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Complex;

impl std::ops::Add for Complex {
    type Output = Complex;
    fn add(self, rhs: Complex) -> Complex {
        Complex::new(self.re + rhs.re, self.im + rhs.im)
    }
}

impl std::ops::Sub for Complex {
    type Output = Complex;
    fn sub(self, rhs: Complex) -> Complex {
        Complex::new(self.re - rhs.re, self.im - rhs.im)
    }
}

impl std::ops::Mul for Complex {
    type Output = Complex;
    fn mul(self, rhs: Complex) -> Complex {
        Complex::new(
            self.re * rhs.re - self.im * rhs.im,
            self.re * rhs.im + self.im * rhs.re,
        )
    }
}

impl std::ops::Mul<f64> for Complex {
    type Output = Complex;
    fn mul(self, s: f64) -> Complex {
        Complex::new(self.re * s, self.im * s)
    }
}

impl std::ops::Div<f64> for Complex {
    type Output = Complex;
    fn div(self, s: f64) -> Complex {
        Complex::new(self.re / s, self.im / s)
    }
}
