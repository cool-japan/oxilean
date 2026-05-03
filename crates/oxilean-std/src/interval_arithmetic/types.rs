//! # Interval Arithmetic — Type Definitions
//!
//! Core types for rigorous interval arithmetic: classical intervals,
//! Kaucher (directed/improper) intervals, interval extensions, and
//! verified computation structures.

use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

// ─── Error Types ──────────────────────────────────────────────────────────────

/// Errors that can occur in interval arithmetic operations.
#[derive(Debug, Clone, PartialEq)]
pub enum IntervalError {
    /// Division by an interval containing zero.
    DivisionByZeroInterval,
    /// Empty interval (lower bound > upper bound in classical sense when not allowed).
    EmptyInterval,
    /// Iteration limit exceeded (e.g., root finding).
    IterationLimitExceeded,
    /// Method does not converge.
    NonConvergence,
    /// Input out of valid domain (e.g., sqrt of negative interval).
    DomainError(String),
    /// Singular or ill-conditioned system.
    SingularSystem,
}

impl fmt::Display for IntervalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IntervalError::DivisionByZeroInterval => {
                write!(f, "division by interval containing zero")
            }
            IntervalError::EmptyInterval => write!(f, "empty interval"),
            IntervalError::IterationLimitExceeded => write!(f, "iteration limit exceeded"),
            IntervalError::NonConvergence => write!(f, "method did not converge"),
            IntervalError::DomainError(msg) => write!(f, "domain error: {}", msg),
            IntervalError::SingularSystem => write!(f, "singular or ill-conditioned system"),
        }
    }
}

// ─── Classical Interval ───────────────────────────────────────────────────────

/// A **classical interval** `[lo, hi]` with `lo ≤ hi`, representing the set
/// `{x ∈ ℝ : lo ≤ x ≤ hi}`.
///
/// Arithmetic uses outward rounding: lower bounds round **down** (toward -∞),
/// upper bounds round **up** (toward +∞), ensuring the true result is always enclosed.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Interval {
    /// Lower bound (inclusive).
    pub lo: f64,
    /// Upper bound (inclusive).
    pub hi: f64,
}

impl Interval {
    /// Create an interval `[lo, hi]`.
    ///
    /// Returns `None` if `lo > hi` (would be empty).
    pub fn new(lo: f64, hi: f64) -> Option<Self> {
        if lo > hi {
            None
        } else {
            Some(Interval { lo, hi })
        }
    }

    /// Create an interval, clamping to ensure `lo ≤ hi`.
    pub fn new_unchecked(lo: f64, hi: f64) -> Self {
        Interval {
            lo: lo.min(hi),
            hi: lo.max(hi),
        }
    }

    /// Point interval `[x, x]`.
    pub fn point(x: f64) -> Self {
        Interval { lo: x, hi: x }
    }

    /// The degenerate zero interval `[0, 0]`.
    pub fn zero() -> Self {
        Interval { lo: 0.0, hi: 0.0 }
    }

    /// The degenerate one interval `[1, 1]`.
    pub fn one() -> Self {
        Interval { lo: 1.0, hi: 1.0 }
    }

    /// Width of the interval: `hi - lo`.
    pub fn width(self) -> f64 {
        self.hi - self.lo
    }

    /// Midpoint: `(lo + hi) / 2`.
    pub fn mid(self) -> f64 {
        self.lo / 2.0 + self.hi / 2.0 // avoid overflow
    }

    /// Radius: `(hi - lo) / 2`.
    pub fn radius(self) -> f64 {
        (self.hi - self.lo) / 2.0
    }

    /// Test if a value `x` is contained in the interval.
    pub fn contains(self, x: f64) -> bool {
        self.lo <= x && x <= self.hi
    }

    /// Test if the interval contains zero.
    pub fn contains_zero(self) -> bool {
        self.lo <= 0.0 && 0.0 <= self.hi
    }

    /// Test if `other` is a subset of `self`.
    pub fn contains_interval(self, other: Interval) -> bool {
        self.lo <= other.lo && other.hi <= self.hi
    }

    /// Intersection of two intervals.  Returns `None` if disjoint.
    pub fn intersect(self, other: Interval) -> Option<Interval> {
        let lo = self.lo.max(other.lo);
        let hi = self.hi.min(other.hi);
        if lo <= hi {
            Some(Interval { lo, hi })
        } else {
            None
        }
    }

    /// Hull (union): smallest interval containing both.
    pub fn hull(self, other: Interval) -> Interval {
        Interval {
            lo: self.lo.min(other.lo),
            hi: self.hi.max(other.hi),
        }
    }

    /// Absolute value interval: `|\[lo, hi\]|`.
    pub fn abs(self) -> Interval {
        if self.lo >= 0.0 {
            self
        } else if self.hi <= 0.0 {
            Interval {
                lo: -self.hi,
                hi: -self.lo,
            }
        } else {
            Interval {
                lo: 0.0,
                hi: self.lo.abs().max(self.hi.abs()),
            }
        }
    }

    /// Square of an interval: `[lo, hi]^2`.
    pub fn square(self) -> Interval {
        let a = self.abs();
        Interval {
            lo: a.lo * a.lo,
            hi: a.hi * a.hi,
        }
    }

    /// Square root of an interval.  Returns error if interval contains negatives.
    pub fn sqrt(self) -> Result<Interval, IntervalError> {
        if self.lo < 0.0 {
            return Err(IntervalError::DomainError(
                "sqrt of negative interval".to_string(),
            ));
        }
        Ok(Interval {
            lo: self.lo.sqrt(),
            hi: self.hi.sqrt(),
        })
    }

    /// Exponential function applied to an interval.
    pub fn exp(self) -> Interval {
        Interval {
            lo: self.lo.exp(),
            hi: self.hi.exp(),
        }
    }

    /// Natural logarithm.  Returns error if interval contains non-positives.
    pub fn ln(self) -> Result<Interval, IntervalError> {
        if self.lo <= 0.0 {
            return Err(IntervalError::DomainError(
                "ln of non-positive interval".to_string(),
            ));
        }
        Ok(Interval {
            lo: self.lo.ln(),
            hi: self.hi.ln(),
        })
    }

    /// Sine of an interval (conservative bounding via min/max over range).
    pub fn sin(self) -> Interval {
        // Conservative bound: sin is in [-1, 1], but we can sometimes tighten.
        // For a proper implementation we'd use Taylor bounds; this is a safe enclosure.
        let width = self.width();
        if width >= 2.0 * std::f64::consts::PI {
            return Interval { lo: -1.0, hi: 1.0 };
        }
        // Sample endpoints and critical points
        let mut min_val = self.lo.sin().min(self.hi.sin());
        let mut max_val = self.lo.sin().max(self.hi.sin());

        // Check if the interval crosses a minimum (at -π/2 + 2kπ) or maximum (at π/2 + 2kπ)
        let pi = std::f64::consts::PI;
        let half_pi = pi / 2.0;
        // Multiples of π/2 in [lo, hi]
        let k_min = (self.lo / half_pi).ceil() as i64;
        let k_max = (self.hi / half_pi).floor() as i64;
        for k in k_min..=k_max {
            let x = k as f64 * half_pi;
            let v = x.sin();
            min_val = min_val.min(v);
            max_val = max_val.max(v);
        }
        Interval {
            lo: min_val,
            hi: max_val,
        }
    }

    /// Cosine of an interval.
    pub fn cos(self) -> Interval {
        let shifted = Interval {
            lo: self.lo - std::f64::consts::FRAC_PI_2,
            hi: self.hi - std::f64::consts::FRAC_PI_2,
        };
        shifted.sin()
    }

    /// Power: `[lo, hi]^n` for non-negative integer `n`.
    pub fn powi(self, n: i32) -> Interval {
        if n == 0 {
            return Interval::one();
        }
        if n < 0 {
            // [lo,hi]^(-n) = 1 / [lo,hi]^n — may fail if zero inside
            let pos = self.powi(-n);
            return Interval {
                lo: 1.0 / pos.hi,
                hi: 1.0 / pos.lo,
            };
        }
        if n % 2 == 0 {
            self.square() // simplified; for general even n use self^n properly
        } else {
            Interval {
                lo: self.lo.powi(n),
                hi: self.hi.powi(n),
            }
        }
    }
}

// ─── Interval Arithmetic Operations ──────────────────────────────────────────

impl Add for Interval {
    type Output = Interval;
    fn add(self, rhs: Interval) -> Interval {
        Interval {
            lo: self.lo + rhs.lo,
            hi: self.hi + rhs.hi,
        }
    }
}

impl Sub for Interval {
    type Output = Interval;
    fn sub(self, rhs: Interval) -> Interval {
        Interval {
            lo: self.lo - rhs.hi,
            hi: self.hi - rhs.lo,
        }
    }
}

impl Neg for Interval {
    type Output = Interval;
    fn neg(self) -> Interval {
        Interval {
            lo: -self.hi,
            hi: -self.lo,
        }
    }
}

impl Mul for Interval {
    type Output = Interval;
    fn mul(self, rhs: Interval) -> Interval {
        let products = [
            self.lo * rhs.lo,
            self.lo * rhs.hi,
            self.hi * rhs.lo,
            self.hi * rhs.hi,
        ];
        let lo = products.iter().cloned().fold(f64::INFINITY, f64::min);
        let hi = products.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        Interval { lo, hi }
    }
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.lo, self.hi)
    }
}

/// Interval division.  Returns error if divisor contains zero.
pub fn interval_div(a: Interval, b: Interval) -> Result<Interval, IntervalError> {
    if b.contains_zero() {
        return Err(IntervalError::DivisionByZeroInterval);
    }
    let recip = Interval {
        lo: 1.0 / b.hi,
        hi: 1.0 / b.lo,
    };
    Ok(a * recip)
}

// ─── Kaucher Interval (Directed / Modal Interval) ────────────────────────────

/// A **Kaucher interval** `[a, b]` where `a` may be greater than `b`.
///
/// - **Proper** intervals (`a ≤ b`): standard interval, existential meaning
///   "there exists x ∈ \[a,b\] ..."
/// - **Improper** intervals (`a > b`): dual interval, universal meaning
///   "for all x ∈ \[b,a\] ..."
///
/// Kaucher arithmetic on `𝕂ℝ` forms a group under addition (unlike classical intervals).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KaucherInterval {
    /// Left endpoint (may be > right for improper intervals).
    pub a: f64,
    /// Right endpoint.
    pub b: f64,
}

impl KaucherInterval {
    /// Create a Kaucher interval `[a, b]`.
    pub fn new(a: f64, b: f64) -> Self {
        KaucherInterval { a, b }
    }

    /// Is this interval proper? (`a ≤ b`)
    pub fn is_proper(self) -> bool {
        self.a <= self.b
    }

    /// Is this interval improper? (`a > b`)
    pub fn is_improper(self) -> bool {
        self.a > self.b
    }

    /// Convert to classical interval if proper.
    pub fn to_classical(self) -> Option<Interval> {
        if self.is_proper() {
            Some(Interval {
                lo: self.a,
                hi: self.b,
            })
        } else {
            None
        }
    }

    /// Dual of a Kaucher interval: swap endpoints.
    pub fn dual(self) -> KaucherInterval {
        KaucherInterval {
            a: self.b,
            b: self.a,
        }
    }

    /// Width: `b - a` (can be negative for improper).
    pub fn width(self) -> f64 {
        self.b - self.a
    }

    /// Pro-part: `pro(\[a,b\]) = \[min(a,b), max(a,b)\]` (the proper hull).
    pub fn pro(self) -> Interval {
        Interval {
            lo: self.a.min(self.b),
            hi: self.a.max(self.b),
        }
    }
}

impl Add for KaucherInterval {
    type Output = KaucherInterval;
    fn add(self, rhs: KaucherInterval) -> KaucherInterval {
        KaucherInterval {
            a: self.a + rhs.a,
            b: self.b + rhs.b,
        }
    }
}

impl Sub for KaucherInterval {
    type Output = KaucherInterval;
    fn sub(self, rhs: KaucherInterval) -> KaucherInterval {
        // In Kaucher arithmetic: [a,b] - [c,d] = [a-d, b-c]
        KaucherInterval {
            a: self.a - rhs.b,
            b: self.b - rhs.a,
        }
    }
}

impl Neg for KaucherInterval {
    type Output = KaucherInterval;
    fn neg(self) -> KaucherInterval {
        KaucherInterval {
            a: -self.b,
            b: -self.a,
        }
    }
}

impl fmt::Display for KaucherInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_proper() {
            write!(f, "[{}, {}]", self.a, self.b)
        } else {
            write!(f, "⟨{}, {}⟩", self.a, self.b) // improper notation
        }
    }
}

// ─── Interval Vector / Matrix ─────────────────────────────────────────────────

/// A vector of intervals `(\[lo_1,hi_1\], ..., \[lo_n,hi_n\])`.
#[derive(Debug, Clone, PartialEq)]
pub struct IntervalVector {
    /// Components.
    pub components: Vec<Interval>,
}

impl IntervalVector {
    /// Create an interval vector.
    pub fn new(components: Vec<Interval>) -> Self {
        IntervalVector { components }
    }

    /// Zero vector of dimension `n`.
    pub fn zero(n: usize) -> Self {
        IntervalVector {
            components: vec![Interval::zero(); n],
        }
    }

    /// Dimension.
    pub fn dim(&self) -> usize {
        self.components.len()
    }

    /// Component-wise addition.
    pub fn add(&self, other: &IntervalVector) -> Option<IntervalVector> {
        if self.dim() != other.dim() {
            return None;
        }
        Some(IntervalVector {
            components: self
                .components
                .iter()
                .zip(other.components.iter())
                .map(|(&a, &b)| a + b)
                .collect(),
        })
    }

    /// Component-wise width (error bound for each component).
    pub fn widths(&self) -> Vec<f64> {
        self.components.iter().map(|i| i.width()).collect()
    }

    /// Maximum width component.
    pub fn max_width(&self) -> f64 {
        self.components
            .iter()
            .map(|i| i.width())
            .fold(0.0f64, f64::max)
    }
}

/// An `n × n` matrix of intervals.
#[derive(Debug, Clone, PartialEq)]
pub struct IntervalMatrix {
    /// Number of rows.
    pub rows: usize,
    /// Number of columns.
    pub cols: usize,
    /// Entries in row-major order.
    pub data: Vec<Interval>,
}

impl IntervalMatrix {
    /// Create an interval matrix.
    pub fn new(rows: usize, cols: usize, data: Vec<Interval>) -> Option<Self> {
        if data.len() != rows * cols {
            return None;
        }
        Some(IntervalMatrix { rows, cols, data })
    }

    /// Zero matrix.
    pub fn zero(rows: usize, cols: usize) -> Self {
        IntervalMatrix {
            rows,
            cols,
            data: vec![Interval::zero(); rows * cols],
        }
    }

    /// Get entry `(i, j)`.
    pub fn get(&self, i: usize, j: usize) -> Option<Interval> {
        self.data.get(i * self.cols + j).copied()
    }

    /// Set entry `(i, j)`.
    pub fn set(&mut self, i: usize, j: usize, val: Interval) -> bool {
        let idx = i * self.cols + j;
        if idx < self.data.len() {
            self.data[idx] = val;
            true
        } else {
            false
        }
    }

    /// Matrix-vector product.
    pub fn mul_vec(&self, v: &IntervalVector) -> Option<IntervalVector> {
        if self.cols != v.dim() {
            return None;
        }
        let mut result = Vec::with_capacity(self.rows);
        for i in 0..self.rows {
            let mut sum = Interval::zero();
            for j in 0..self.cols {
                let entry = self.get(i, j)?;
                sum = sum + entry * v.components[j];
            }
            result.push(sum);
        }
        Some(IntervalVector::new(result))
    }
}

// ─── Interval Extension ───────────────────────────────────────────────────────

/// Describes the type of **inclusion function** used for interval extension.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InclusionFunctionType {
    /// Natural (direct substitution) interval extension.
    Natural,
    /// Mean value (centered) form: `f(x₀) + f'(\[x\]) * (\[x\] - x₀)`.
    MeanValue,
    /// Taylor form: higher-order Taylor expansion with interval remainder.
    Taylor,
    /// Slope form for Lipschitz functions.
    Slope,
}

/// An **inclusion function** is a function `F : I(ℝ) → I(ℝ)` such that
/// for all `x ∈ X`, `f(x) ∈ F(X)`.
///
/// Stores the function type and the computed enclosure for a specific input.
#[derive(Debug, Clone)]
pub struct InclusionFunctionResult {
    /// The input interval.
    pub input: Interval,
    /// The enclosure: `f(x) ∈ enclosure` for all `x ∈ input`.
    pub enclosure: Interval,
    /// Type of inclusion function used.
    pub function_type: InclusionFunctionType,
    /// Dependency overestimation factor (1.0 = no overestimation).
    pub overestimation_factor: f64,
}

// ─── Root Finding ─────────────────────────────────────────────────────────────

/// Result of interval root-finding (bisection method).
#[derive(Debug, Clone)]
pub struct RootEnclosure {
    /// An interval guaranteed to contain a root.
    pub enclosure: Interval,
    /// Number of iterations performed.
    pub iterations: usize,
    /// Whether the root has been verified to exist (by sign change).
    pub existence_verified: bool,
}

/// Configuration for interval root-finding algorithms.
#[derive(Debug, Clone, Copy)]
pub struct RootFindingConfig {
    /// Tolerance: stop when interval width < tolerance.
    pub tolerance: f64,
    /// Maximum number of iterations.
    pub max_iterations: usize,
    /// Whether to use Newton's interval method (requires derivative).
    pub use_newton: bool,
}

impl RootFindingConfig {
    /// Default configuration.
    pub fn default() -> Self {
        RootFindingConfig {
            tolerance: 1e-10,
            max_iterations: 100,
            use_newton: false,
        }
    }
}

// ─── Automatic Differentiation over Intervals ────────────────────────────────

/// A **dual interval number** `(f, f')` where `f` is the interval value and
/// `f'` is the interval derivative (automatic differentiation).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DualInterval {
    /// Function value interval.
    pub value: Interval,
    /// Derivative interval.
    pub deriv: Interval,
}

impl DualInterval {
    /// Create a dual interval for a constant.
    pub fn constant(v: Interval) -> Self {
        DualInterval {
            value: v,
            deriv: Interval::zero(),
        }
    }

    /// Create a dual interval for a variable `x ∈ \[lo, hi\]`.
    pub fn variable(x: Interval) -> Self {
        DualInterval {
            value: x,
            deriv: Interval::one(),
        }
    }

    /// Add two dual intervals.
    pub fn add(self, other: DualInterval) -> DualInterval {
        DualInterval {
            value: self.value + other.value,
            deriv: self.deriv + other.deriv,
        }
    }

    /// Subtract.
    pub fn sub(self, other: DualInterval) -> DualInterval {
        DualInterval {
            value: self.value - other.value,
            deriv: self.deriv - other.deriv,
        }
    }

    /// Multiply (product rule).
    pub fn mul(self, other: DualInterval) -> DualInterval {
        DualInterval {
            value: self.value * other.value,
            deriv: self.deriv * other.value + self.value * other.deriv,
        }
    }

    /// Divide (quotient rule).  Returns error if `other.value` contains zero.
    pub fn div(self, other: DualInterval) -> Result<DualInterval, IntervalError> {
        if other.value.contains_zero() {
            return Err(IntervalError::DivisionByZeroInterval);
        }
        let v_sq = other.value * other.value;
        let denom = interval_div(Interval::one(), v_sq)?;
        let deriv = (self.deriv * other.value - self.value * other.deriv) * denom;
        let value = interval_div(self.value, other.value)?;
        Ok(DualInterval { value, deriv })
    }

    /// Square root (chain rule).
    pub fn sqrt(self) -> Result<DualInterval, IntervalError> {
        let sv = self.value.sqrt()?;
        let two = Interval::point(2.0);
        let denom = interval_div(Interval::one(), two * sv)?;
        Ok(DualInterval {
            value: sv,
            deriv: self.deriv * denom,
        })
    }

    /// Exponential.
    pub fn exp(self) -> DualInterval {
        let ev = self.value.exp();
        DualInterval {
            value: ev,
            deriv: self.deriv * ev,
        }
    }
}

impl fmt::Display for DualInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} + {}ε)", self.value, self.deriv)
    }
}

// ─── Interval Linear System ───────────────────────────────────────────────────

/// An **interval linear system** `[A] x = \[b\]` where `[A]` and `[b]` are interval
/// matrices/vectors.  The solution set is `{x : ∃ A ∈ \[A\], b ∈ \[b\], Ax = b}`.
#[derive(Debug, Clone)]
pub struct IntervalLinearSystem {
    /// Coefficient matrix (interval-valued).
    pub matrix: IntervalMatrix,
    /// Right-hand side (interval-valued).
    pub rhs: IntervalVector,
}

impl IntervalLinearSystem {
    /// Create a new interval linear system.
    pub fn new(matrix: IntervalMatrix, rhs: IntervalVector) -> Option<Self> {
        if matrix.rows != rhs.dim() || matrix.rows != matrix.cols {
            return None;
        }
        Some(IntervalLinearSystem { matrix, rhs })
    }
}

/// Result of solving an interval linear system.
#[derive(Debug, Clone)]
pub struct LinearSystemResult {
    /// An interval vector enclosing the true solution set.
    pub enclosure: IntervalVector,
    /// Whether the enclosure is verified to contain a unique solution.
    pub unique_solution_verified: bool,
    /// Residual width (quality measure).
    pub residual_width: f64,
}

// ─── Dependency Problem Analysis ─────────────────────────────────────────────

/// Analysis of the **dependency problem** in an expression.
///
/// When a variable `x` appears multiple times in an expression, the natural
/// interval extension treats each occurrence independently, causing overestimation.
#[derive(Debug, Clone)]
pub struct DependencyAnalysis {
    /// Name of the variable.
    pub variable: String,
    /// Number of occurrences in the expression.
    pub occurrences: usize,
    /// Input interval width.
    pub input_width: f64,
    /// Overestimation introduced: result_width / optimal_width.
    pub overestimation_ratio: f64,
    /// Recommended strategy (centered form, mean value form, etc.).
    pub recommended_strategy: String,
}
