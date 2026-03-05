//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// Continued fraction expansion of an integer ratio p/q.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IntExtContinuedFraction {
    /// The partial quotients a_0, a_1, a_2, ...
    pub coeffs: Vec<i64>,
}
#[allow(dead_code)]
impl IntExtContinuedFraction {
    /// Compute the continued fraction expansion of p/q.
    pub fn from_ratio(mut p: i64, mut q: i64) -> Self {
        let mut coeffs = Vec::new();
        while q != 0 {
            coeffs.push(p / q);
            let r = p % q;
            p = q;
            q = r;
        }
        IntExtContinuedFraction { coeffs }
    }
    /// Evaluate back to a rational (p, q) pair.
    pub fn to_ratio(&self) -> (i64, i64) {
        if self.coeffs.is_empty() {
            return (0, 1);
        }
        let mut num = 1i64;
        let mut den = 0i64;
        for &a in self.coeffs.iter().rev() {
            let new_num = a * num + den;
            den = num;
            num = new_num;
        }
        (num, den)
    }
    /// Return the n-th convergent as (p_n, q_n).
    pub fn convergent(&self, n: usize) -> Option<(i64, i64)> {
        if n >= self.coeffs.len() {
            return None;
        }
        let sub = IntExtContinuedFraction {
            coeffs: self.coeffs[..=n].to_vec(),
        };
        Some(sub.to_ratio())
    }
    /// Return the number of partial quotients.
    pub fn depth(&self) -> usize {
        self.coeffs.len()
    }
}
/// Representation of a Gaussian integer a + b*i.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntExtGaussian {
    /// Real part.
    pub re: i64,
    /// Imaginary part.
    pub im: i64,
}
#[allow(dead_code)]
impl IntExtGaussian {
    /// Create a new Gaussian integer.
    pub fn new(re: i64, im: i64) -> Self {
        IntExtGaussian { re, im }
    }
    /// The zero Gaussian integer.
    pub fn zero() -> Self {
        IntExtGaussian { re: 0, im: 0 }
    }
    /// The unit Gaussian integer.
    pub fn one() -> Self {
        IntExtGaussian { re: 1, im: 0 }
    }
    /// The imaginary unit i.
    pub fn i_unit() -> Self {
        IntExtGaussian { re: 0, im: 1 }
    }
    /// Add two Gaussian integers.
    pub fn add(self, other: Self) -> Self {
        IntExtGaussian {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
    /// Multiply two Gaussian integers.
    pub fn mul(self, other: Self) -> Self {
        IntExtGaussian {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
    /// The complex conjugate.
    pub fn conj(self) -> Self {
        IntExtGaussian {
            re: self.re,
            im: -self.im,
        }
    }
    /// The norm: N(a+bi) = a^2 + b^2.
    pub fn norm(self) -> i64 {
        self.re * self.re + self.im * self.im
    }
    /// Returns the four units of the Gaussian integers.
    pub fn units() -> [Self; 4] {
        [
            IntExtGaussian::one(),
            IntExtGaussian::new(-1, 0),
            IntExtGaussian::i_unit(),
            IntExtGaussian::new(0, -1),
        ]
    }
    /// Check if this is a unit (norm = 1).
    pub fn is_unit(self) -> bool {
        self.norm() == 1
    }
    /// Check if this divides other in the Gaussian integers.
    pub fn divides(self, other: Self) -> bool {
        if self.norm() == 0 {
            return other.re == 0 && other.im == 0;
        }
        let n = self.norm();
        let prod = other.mul(self.conj());
        prod.re % n == 0 && prod.im % n == 0
    }
}
/// p-adic valuation: the largest power of p dividing n.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntExtPadicVal {
    /// The prime p.
    pub prime: i64,
}
#[allow(dead_code)]
impl IntExtPadicVal {
    /// Create a p-adic valuation for prime p.
    pub fn new(prime: i64) -> Self {
        IntExtPadicVal { prime }
    }
    /// Compute v_p(n): the p-adic valuation of n.
    /// Returns None if n = 0 (undefined) or prime <= 1.
    pub fn val(&self, n: i64) -> Option<u32> {
        if n == 0 || self.prime <= 1 {
            return None;
        }
        let mut count = 0u32;
        let mut current = n.abs();
        while current % self.prime == 0 {
            count += 1;
            current /= self.prime;
        }
        Some(count)
    }
    /// Returns p^k.
    pub fn power(&self, k: u32) -> i64 {
        self.prime.pow(k)
    }
    /// Check if v_p(a) >= v_p(b), i.e. whether p^v_p(b) divides a.
    pub fn divides_to_val(&self, a: i64, b: i64) -> bool {
        match (self.val(a), self.val(b)) {
            (Some(va), Some(vb)) => va >= vb,
            (None, _) => false,
            (_, None) => true,
        }
    }
    /// Return the unit part: n / p^v_p(n).
    pub fn unit_part(&self, n: i64) -> Option<i64> {
        let v = self.val(n)?;
        Some(n / self.power(v))
    }
}
/// Extended Euclidean algorithm result: gcd and Bezout coefficients.
///
/// Given integers a and b, stores gcd, x, y such that a*x + b*y = gcd.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntExtEuclidResult {
    /// The greatest common divisor (always non-negative).
    pub gcd: i64,
    /// Bezout coefficient for a.
    pub x: i64,
    /// Bezout coefficient for b.
    pub y: i64,
}
#[allow(dead_code)]
impl IntExtEuclidResult {
    /// Run the extended Euclidean algorithm on (a, b).
    pub fn compute(a: i64, b: i64) -> Self {
        fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
            if b == 0 {
                (a.abs(), if a >= 0 { 1 } else { -1 }, 0)
            } else {
                let (g, x1, y1) = extended_gcd(b, a % b);
                (g, y1, x1 - (a / b) * y1)
            }
        }
        let (gcd, x, y) = extended_gcd(a, b);
        IntExtEuclidResult { gcd, x, y }
    }
    /// Verify the Bezout identity: a*x + b*y = gcd.
    pub fn verify(&self, a: i64, b: i64) -> bool {
        a * self.x + b * self.y == self.gcd
    }
    /// Returns true if a and b are coprime (gcd = 1).
    pub fn is_coprime(&self) -> bool {
        self.gcd == 1
    }
    /// Returns a modular inverse of a modulo b, if it exists.
    pub fn modular_inverse(&self, _a: i64, b: i64) -> Option<i64> {
        if self.gcd != 1 {
            return None;
        }
        let inv = self.x % b;
        Some(if inv < 0 { inv + b } else { inv })
    }
}
/// Chinese Remainder Theorem solver for a system of congruences.
///
/// Solves: x ≡ r_i (mod m_i) for pairwise coprime moduli.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IntCrtSolver {
    /// Pairs of (remainder, modulus).
    congruences: Vec<(i64, i64)>,
}
#[allow(dead_code)]
impl IntCrtSolver {
    /// Create a new CRT solver with no congruences.
    pub fn new() -> Self {
        IntCrtSolver {
            congruences: Vec::new(),
        }
    }
    /// Add a congruence x ≡ remainder (mod modulus).
    pub fn add_congruence(&mut self, remainder: i64, modulus: i64) {
        self.congruences.push((remainder, modulus));
    }
    /// Solve the system and return the smallest non-negative x, plus the combined modulus M.
    /// Returns None if the moduli are not pairwise coprime.
    pub fn solve(&self) -> Option<(i64, i64)> {
        if self.congruences.is_empty() {
            return Some((0, 1));
        }
        let (mut x, mut m) = self.congruences[0];
        x = ((x % m) + m) % m;
        for &(r, mi) in &self.congruences[1..] {
            let ext = IntExtEuclidResult::compute(m, mi);
            if ext.gcd != 1 {
                return None;
            }
            let diff = r - x;
            let new_m = m * mi;
            let step = (diff % mi * ext.x % mi + mi) % mi;
            x = ((x + m * step) % new_m + new_m) % new_m;
            m = new_m;
        }
        Some((x, m))
    }
    /// Number of congruences registered.
    pub fn len(&self) -> usize {
        self.congruences.len()
    }
    /// Returns true if no congruences have been added.
    pub fn is_empty(&self) -> bool {
        self.congruences.is_empty()
    }
}
/// Stern-Brocot tree node: a rational number p/q in lowest terms.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntExtSternBrocot {
    /// Numerator.
    pub p: i64,
    /// Denominator (positive).
    pub q: i64,
}
#[allow(dead_code)]
impl IntExtSternBrocot {
    /// The root of the Stern-Brocot tree (1/1).
    pub fn root() -> Self {
        IntExtSternBrocot { p: 1, q: 1 }
    }
    /// The left child (mediant with 0/1).
    pub fn left_child(self) -> Self {
        IntExtSternBrocot {
            p: self.p,
            q: self.p + self.q,
        }
    }
    /// The right child (mediant with 1/0 ancestor).
    pub fn right_child(self) -> Self {
        IntExtSternBrocot {
            p: self.p + self.q,
            q: self.q,
        }
    }
    /// The mediant of two fractions.
    pub fn mediant(self, other: Self) -> Self {
        IntExtSternBrocot {
            p: self.p + other.p,
            q: self.q + other.q,
        }
    }
    /// Approximate pi/4 using the Stern-Brocot tree (returns p/q < target or p/q > target).
    /// Steps from root, going left if current > target_num/target_den, right otherwise.
    pub fn approximate(target_num: i64, target_den: i64, steps: usize) -> Self {
        let mut lo_p = 0i64;
        let mut lo_q = 1i64;
        let mut hi_p = 1i64;
        let mut hi_q = 0i64;
        let mut current = IntExtSternBrocot { p: 1, q: 1 };
        for _ in 0..steps {
            if current.p * target_den < target_num * current.q {
                lo_p = current.p;
                lo_q = current.q;
            } else if current.p * target_den > target_num * current.q {
                hi_p = current.p;
                hi_q = current.q;
            } else {
                break;
            }
            current = IntExtSternBrocot {
                p: lo_p + hi_p,
                q: lo_q + hi_q,
            };
        }
        current
    }
    /// Convert to a floating-point value.
    pub fn to_f64(self) -> f64 {
        self.p as f64 / self.q as f64
    }
}
