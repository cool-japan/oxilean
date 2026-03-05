//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::complex_type::Complex;
use super::functions::*;

/// A Möbius transformation f(z) = (az + b) / (cz + d) with ad - bc ≠ 0.
#[derive(Clone, Copy, Debug)]
pub struct MobiusTransform {
    /// Numerator coefficient of z.
    pub a: Complex,
    /// Numerator constant.
    pub b: Complex,
    /// Denominator coefficient of z.
    pub c: Complex,
    /// Denominator constant.
    pub d: Complex,
}
impl MobiusTransform {
    /// Create a new Möbius transformation. Returns `None` if det = ad - bc = 0.
    pub fn new(a: Complex, b: Complex, c: Complex, d: Complex) -> Option<Self> {
        let det = a.mul(d).sub(b.mul(c));
        if det.abs() < f64::EPSILON {
            return None;
        }
        Some(Self { a, b, c, d })
    }
    /// Identity transformation z ↦ z.
    pub fn identity() -> Self {
        Self {
            a: Complex::one(),
            b: Complex::zero(),
            c: Complex::zero(),
            d: Complex::one(),
        }
    }
    /// Apply the transformation to z. Returns `None` at the pole cz + d = 0.
    pub fn apply(self, z: Complex) -> Option<Complex> {
        let numer = self.a.mul(z).add(self.b);
        let denom = self.c.mul(z).add(self.d);
        numer.div(denom)
    }
    /// Compose two Möbius transformations: (self ∘ other)(z) = self(other(z)).
    pub fn compose(self, other: Self) -> Option<Self> {
        let a = self.a.mul(other.a).add(self.b.mul(other.c));
        let b = self.a.mul(other.b).add(self.b.mul(other.d));
        let c = self.c.mul(other.a).add(self.d.mul(other.c));
        let d = self.c.mul(other.b).add(self.d.mul(other.d));
        Self::new(a, b, c, d)
    }
    /// Invert the transformation: f^{-1}(z) = (dz - b) / (-cz + a).
    pub fn invert(self) -> Option<Self> {
        Self::new(self.d, self.b.neg(), self.c.neg(), self.a)
    }
    /// Fixed points of the transformation: solutions to f(z) = z.
    /// Solves cz² + (d - a)z - b = 0.
    pub fn fixed_points(self) -> Vec<Complex> {
        if self.c.abs() < f64::EPSILON {
            let da = self.d.sub(self.a);
            if da.abs() < f64::EPSILON {
                return vec![];
            }
            return self.b.div(da).map(|z| vec![z]).unwrap_or_default();
        }
        let p = self.d.sub(self.a);
        let disc = p.mul(p).add(self.b.mul(self.c).scale(4.0));
        let disc_sqrt = disc.sqrt();
        let two_c = self.c.scale(2.0);
        let mut pts = vec![];
        if let Some(z1) = p.neg().add(disc_sqrt).div(two_c) {
            pts.push(z1);
        }
        if let Some(z2) = p.neg().sub(disc_sqrt).div(two_c) {
            if pts.is_empty() || !z2.approx_eq(pts[0], 1e-12) {
                pts.push(z2);
            }
        }
        pts
    }
}
/// Conformal mapping: Riemann mapping theorem (abstract).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ConformalMap {
    pub source_domain: String,
    pub target_domain: String,
    pub description: String,
}
#[allow(dead_code)]
impl ConformalMap {
    pub fn new(source: &str, target: &str, desc: &str) -> Self {
        ConformalMap {
            source_domain: source.to_string(),
            target_domain: target.to_string(),
            description: desc.to_string(),
        }
    }
    pub fn riemann_mapping(domain: &str) -> Self {
        ConformalMap::new(domain, "UnitDisk", "Riemann mapping theorem")
    }
    pub fn upper_half_to_disk() -> Self {
        ConformalMap::new("UpperHalfPlane", "UnitDisk", "z → (z-i)/(z+i)")
    }
    pub fn joukowski() -> Self {
        ConformalMap::new("UnitDisk", "Airfoil", "z → z + 1/z")
    }
    pub fn is_biholomorphic(&self) -> bool {
        true
    }
}
/// Möbius transformation: f(z) = (az+b)/(cz+d).
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct MobiusMap {
    pub a: C64,
    pub b: C64,
    pub c: C64,
    pub d: C64,
}
#[allow(dead_code)]
impl MobiusMap {
    pub fn new(a: C64, b: C64, c: C64, d: C64) -> Self {
        MobiusMap { a, b, c, d }
    }
    pub fn identity() -> Self {
        MobiusMap {
            a: C64::one(),
            b: C64::zero(),
            c: C64::zero(),
            d: C64::one(),
        }
    }
    pub fn determinant(&self) -> C64 {
        self.a.mul(&self.d).sub(&self.b.mul(&self.c))
    }
    pub fn apply(&self, z: C64) -> Option<C64> {
        let num = self.a.mul(&z).add(&self.b);
        let den = self.c.mul(&z).add(&self.d);
        num.div(&den)
    }
    pub fn compose(&self, other: &MobiusMap) -> MobiusMap {
        MobiusMap {
            a: self.a.mul(&other.a).add(&self.b.mul(&other.c)),
            b: self.a.mul(&other.b).add(&self.b.mul(&other.d)),
            c: self.c.mul(&other.a).add(&self.d.mul(&other.c)),
            d: self.c.mul(&other.b).add(&self.d.mul(&other.d)),
        }
    }
    pub fn is_parabolic(&self) -> bool {
        let tr = self.a.add(&self.d);
        let tr2 = tr.mul(&tr);
        (tr2.re - 4.0).abs() < 1e-9 && tr2.im.abs() < 1e-9
    }
}
/// Complex number (64-bit floating point).
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct C64 {
    pub re: f64,
    pub im: f64,
}
#[allow(dead_code)]
impl C64 {
    pub fn new(re: f64, im: f64) -> Self {
        C64 { re, im }
    }
    pub fn zero() -> Self {
        C64::new(0.0, 0.0)
    }
    pub fn one() -> Self {
        C64::new(1.0, 0.0)
    }
    pub fn i() -> Self {
        C64::new(0.0, 1.0)
    }
    pub fn from_polar(r: f64, theta: f64) -> Self {
        C64::new(r * theta.cos(), r * theta.sin())
    }
    pub fn modulus(&self) -> f64 {
        (self.re * self.re + self.im * self.im).sqrt()
    }
    pub fn argument(&self) -> f64 {
        self.im.atan2(self.re)
    }
    pub fn conjugate(&self) -> Self {
        C64::new(self.re, -self.im)
    }
    pub fn add(&self, other: &C64) -> C64 {
        C64::new(self.re + other.re, self.im + other.im)
    }
    pub fn sub(&self, other: &C64) -> C64 {
        C64::new(self.re - other.re, self.im - other.im)
    }
    pub fn mul(&self, other: &C64) -> C64 {
        C64::new(
            self.re * other.re - self.im * other.im,
            self.re * other.im + self.im * other.re,
        )
    }
    pub fn div(&self, other: &C64) -> Option<C64> {
        let denom = other.re * other.re + other.im * other.im;
        if denom.abs() < 1e-15 {
            return None;
        }
        Some(C64::new(
            (self.re * other.re + self.im * other.im) / denom,
            (self.im * other.re - self.re * other.im) / denom,
        ))
    }
    pub fn exp(&self) -> C64 {
        let r = self.re.exp();
        C64::new(r * self.im.cos(), r * self.im.sin())
    }
    pub fn log(&self) -> Option<C64> {
        let r = self.modulus();
        if r < 1e-15 {
            return None;
        }
        Some(C64::new(r.ln(), self.argument()))
    }
    pub fn pow_n(&self, n: i32) -> C64 {
        let r = self.modulus().powi(n);
        let theta = self.argument() * n as f64;
        C64::from_polar(r, theta)
    }
    pub fn sqrt_principal(&self) -> C64 {
        let r = self.modulus().sqrt();
        let theta = self.argument() / 2.0;
        C64::from_polar(r, theta)
    }
    pub fn sin(&self) -> C64 {
        C64::new(
            self.re.sin() * self.im.cosh(),
            self.re.cos() * self.im.sinh(),
        )
    }
    pub fn cos(&self) -> C64 {
        C64::new(
            self.re.cos() * self.im.cosh(),
            -self.re.sin() * self.im.sinh(),
        )
    }
}
