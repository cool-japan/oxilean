//! # Complex - new_group Methods
//!
//! This module contains method implementations for `Complex`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::complex_type::Complex;

impl Complex {
    /// Create a new complex number.
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }
    /// Zero.
    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }
    /// One.
    pub fn one() -> Self {
        Self::new(1.0, 0.0)
    }
    /// Imaginary unit i.
    pub fn i() -> Self {
        Self::new(0.0, 1.0)
    }
    /// Complex conjugate: a + bi → a - bi.
    pub fn conj(self) -> Self {
        Self::new(self.re, -self.im)
    }
    /// Addition.
    pub fn add(self, other: Self) -> Self {
        Self::new(self.re + other.re, self.im + other.im)
    }
    /// Subtraction.
    pub fn sub(self, other: Self) -> Self {
        Self::new(self.re - other.re, self.im - other.im)
    }
    /// Multiplication: (a+bi)(c+di) = (ac-bd) + (ad+bc)i.
    pub fn mul(self, other: Self) -> Self {
        Self::new(
            self.re * other.re - self.im * other.im,
            self.re * other.im + self.im * other.re,
        )
    }
    /// Scalar multiplication.
    pub fn scale(self, s: f64) -> Self {
        Self::new(self.re * s, self.im * s)
    }
    /// Negation.
    pub fn neg(self) -> Self {
        Self::new(-self.re, -self.im)
    }
    /// Complex exponential: exp(a+bi) = e^a * (cos b + i sin b).
    pub fn exp(self) -> Self {
        let ea = self.re.exp();
        Self::new(ea * self.im.cos(), ea * self.im.sin())
    }
    /// Principal complex logarithm: log(z) = ln|z| + i·arg(z).
    pub fn log(self) -> Option<Self> {
        if self.norm_sq() < f64::EPSILON {
            return None;
        }
        Some(Self::new(self.abs().ln(), self.arg()))
    }
    /// Principal square root.
    pub fn sqrt(self) -> Self {
        if self.im == 0.0 && self.re >= 0.0 {
            return Self::new(self.re.sqrt(), 0.0);
        }
        let r = self.abs();
        let theta = self.arg() / 2.0;
        Self::new(r.sqrt() * theta.cos(), r.sqrt() * theta.sin())
    }
    /// Complex sine: sin(z) = (e^(iz) - e^(-iz)) / (2i).
    pub fn sin(self) -> Self {
        let iz = Self::i().mul(self);
        let e1 = iz.exp();
        let e2 = iz.neg().exp();
        e1.sub(e2).div(Self::new(0.0, 2.0)).unwrap_or(Self::zero())
    }
    /// Polar form: (r, θ) → r·(cos θ + i·sin θ).
    pub fn from_polar(r: f64, theta: f64) -> Self {
        Self::new(r * theta.cos(), r * theta.sin())
    }
}
