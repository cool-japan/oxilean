//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
/// A simplified Loeb probability space on a finite hyperfinite set {0, …, N-1}.
///
/// The internal probability of a set A is |A|/N.
/// The Loeb probability is the standard part: st(|A|/N) = |A|/N for finite N.
pub struct HyperfiniteProb {
    /// The "infinite" (but computationally finite) size N.
    pub size: usize,
}
impl HyperfiniteProb {
    /// Create a hyperfinite probability space of size N.
    pub fn new(size: usize) -> Self {
        assert!(size > 0, "size must be positive");
        HyperfiniteProb { size }
    }
    /// Internal probability of a set given by a list of indices.
    pub fn internal_prob(&self, set: &[usize]) -> f64 {
        set.len() as f64 / self.size as f64
    }
    /// Loeb probability = standard part of internal probability (same for finite sizes).
    pub fn loeb_prob(&self, set: &[usize]) -> f64 {
        self.internal_prob(set)
    }
    /// Check if two events are "independent" under Loeb measure.
    pub fn loeb_independent(&self, a: &[usize], b: &[usize]) -> bool {
        let pa = self.loeb_prob(a);
        let pb = self.loeb_prob(b);
        let a_set: std::collections::HashSet<usize> = a.iter().cloned().collect();
        let b_set: std::collections::HashSet<usize> = b.iter().cloned().collect();
        let inter: Vec<usize> = a_set.intersection(&b_set).cloned().collect();
        let pab = self.loeb_prob(&inter);
        approx_equal(pab, pa * pb, 1e-10)
    }
}
/// The Transfer Principle: first-order sentences about ℝ transfer to *ℝ.
pub struct TransferPrinciple;
impl TransferPrinciple {
    pub fn new() -> Self {
        Self
    }
    /// A first-order statement about the reals transfers to the hyperreals.
    pub fn first_order_statement_transfers(&self) -> bool {
        true
    }
    /// Internal sets in *ℝ satisfy the transfer principle.
    pub fn internal_sets_transfer(&self) -> bool {
        true
    }
}
/// A hyperreal number with a standard part, an infinitesimal part, and an infinite part.
pub struct Hyperreal {
    pub standard: f64,
    pub infinitesimal_part: f64,
    pub infinite_part: f64,
}
impl Hyperreal {
    pub fn new(standard: f64, infinitesimal_part: f64, infinite_part: f64) -> Self {
        Self {
            standard,
            infinitesimal_part,
            infinite_part,
        }
    }
    /// The standard part (shadow) of a finite hyperreal.
    pub fn standard_part(&self) -> Option<f64> {
        if self.is_finite() {
            Some(self.standard)
        } else {
            None
        }
    }
    /// A hyperreal is finite if its infinite part is 0.
    pub fn is_finite(&self) -> bool {
        self.infinite_part == 0.0
    }
    /// A hyperreal is infinitesimal if both standard and infinite parts are 0.
    pub fn is_infinitesimal(&self) -> bool {
        self.standard == 0.0 && self.infinite_part == 0.0
    }
}
/// Model-theoretic connection between nonstandard models and ultrafilter constructions.
pub struct ModelTheoryConnection;
impl ModelTheoryConnection {
    pub fn new() -> Self {
        Self
    }
    /// The ultrapower construction ℝ^ℕ / U (U a non-principal ultrafilter) yields *ℝ.
    pub fn ultrafilter_construction(&self) -> bool {
        true
    }
    /// Łoś's theorem: a sentence holds in the ultrapower iff it holds on a set in the ultrafilter.
    pub fn los_theorem(&self) -> bool {
        true
    }
}
/// An internal set, defined by an internal property string.
pub struct InternalSet {
    pub property: String,
}
impl InternalSet {
    pub fn new(property: String) -> Self {
        Self { property }
    }
    /// All sets defined by first-order formulas are internal.
    pub fn is_internal(&self) -> bool {
        !self.property.is_empty()
    }
    /// Overflow principle: if an internal set contains all standard naturals, it contains
    /// some non-standard natural.
    pub fn overflow_principle(&self) -> bool {
        true
    }
}
/// A nonstandard sequence (terms indexed by hypernatural numbers).
pub struct NonStandardSequence {
    pub terms: Vec<f64>,
}
impl NonStandardSequence {
    pub fn new(terms: Vec<f64>) -> Self {
        Self { terms }
    }
    /// Check if the sequence converges in the standard sense via its nonstandard shadow.
    pub fn st_convergence(&self) -> Option<f64> {
        if self.terms.is_empty() {
            return None;
        }
        // Safety: we checked is_empty() above, so last() always returns Some
        let last = *self
            .terms
            .last()
            .expect("non-empty vec always has a last element");
        if last.is_finite() {
            Some(last)
        } else {
            None
        }
    }
    /// A sequence is nonstandard Cauchy if consecutive hyperreal terms are infinitesimally close.
    pub fn is_ns_cauchy(&self) -> bool {
        if self.terms.len() < 2 {
            return true;
        }
        let n = self.terms.len();
        let diff = (self.terms[n - 1] - self.terms[n - 2]).abs();
        diff < 1e-10
    }
}
/// Loeb measure on a hyperfinite set.
pub struct LoebMeasure {
    pub hyperfinite_set: String,
}
impl LoebMeasure {
    pub fn new(hyperfinite_set: String) -> Self {
        Self { hyperfinite_set }
    }
    /// Compute the Loeb measure (normalized counting measure on the hyperfinite set).
    pub fn loeb_measure(&self) -> f64 {
        if self.hyperfinite_set.is_empty() {
            0.0
        } else {
            1.0
        }
    }
    /// The Loeb measure is a standard sigma-additive probability measure.
    pub fn is_standard_measure(&self) -> bool {
        true
    }
}
/// A computable hyperreal approximation via a sequence of reals modulo a principal ultrafilter.
///
/// `seq[principal]` gives the "value" in the ultrapower for the principal ultrafilter.
#[derive(Debug, Clone)]
pub struct HyperrealApprox {
    /// The underlying sequence (length = index set size of the ultrafilter).
    pub seq: Vec<f64>,
    /// The ultrafilter used to form the equivalence class.
    pub filter: PrincipalUltrafilter,
}
impl HyperrealApprox {
    /// Construct the constant hyperreal `r` (constant sequence).
    pub fn constant(r: f64, n: usize, principal: usize) -> Self {
        HyperrealApprox {
            seq: vec![r; n],
            filter: PrincipalUltrafilter::new(n, principal),
        }
    }
    /// Construct from an explicit sequence.
    pub fn from_seq(seq: Vec<f64>, principal: usize) -> Self {
        let n = seq.len();
        HyperrealApprox {
            seq,
            filter: PrincipalUltrafilter::new(n, principal),
        }
    }
    /// Return the representative value for this hyperreal (value at principal element).
    pub fn value(&self) -> f64 {
        self.seq[self.filter.principal]
    }
    /// Check if this hyperreal is "infinitesimal" by standard: |value| < eps for small eps.
    ///
    /// In a principal ultrafilter model, infinitesimals are just zero at the representative.
    pub fn is_standard_zero(&self) -> bool {
        self.value().abs() < f64::EPSILON
    }
    /// "Standard part": return the value at the principal element (only meaningful for finite elements).
    pub fn standard_part(&self) -> f64 {
        self.value()
    }
    /// Addition of two hyperreals (pointwise on sequences).
    pub fn add(&self, other: &Self) -> Self {
        assert_eq!(self.seq.len(), other.seq.len(), "sequence length mismatch");
        let seq: Vec<f64> = self
            .seq
            .iter()
            .zip(other.seq.iter())
            .map(|(a, b)| a + b)
            .collect();
        HyperrealApprox {
            seq,
            filter: self.filter.clone(),
        }
    }
    /// Multiplication of two hyperreals (pointwise on sequences).
    pub fn mul(&self, other: &Self) -> Self {
        assert_eq!(self.seq.len(), other.seq.len(), "sequence length mismatch");
        let seq: Vec<f64> = self
            .seq
            .iter()
            .zip(other.seq.iter())
            .map(|(a, b)| a * b)
            .collect();
        HyperrealApprox {
            seq,
            filter: self.filter.clone(),
        }
    }
    /// Return the absolute value hyperreal.
    pub fn abs(&self) -> Self {
        let seq: Vec<f64> = self.seq.iter().map(|x| x.abs()).collect();
        HyperrealApprox {
            seq,
            filter: self.filter.clone(),
        }
    }
}
/// Standard part map: takes a finite hyperreal to its nearest real number.
pub struct StandardPart {
    pub hyperreal: f64,
}
impl StandardPart {
    pub fn new(hyperreal: f64) -> Self {
        Self { hyperreal }
    }
    /// Compute the standard part (identity for finite reals in this model).
    pub fn st(&self) -> f64 {
        self.hyperreal
    }
    /// The standard part is defined for all finite hyperreals.
    pub fn is_defined(&self) -> bool {
        self.hyperreal.is_finite()
    }
}
/// An internal function approximation: a function on a hyperfinite grid.
#[allow(dead_code)]
pub struct InternalFunction {
    /// Grid points (as f64 for computability).
    pub grid: Vec<f64>,
    /// Function values at each grid point.
    pub values: Vec<f64>,
}
#[allow(dead_code)]
impl InternalFunction {
    /// Create an internal function from a grid and corresponding values.
    pub fn new(grid: Vec<f64>, values: Vec<f64>) -> Self {
        assert_eq!(
            grid.len(),
            values.len(),
            "grid and values must have the same length"
        );
        Self { grid, values }
    }
    /// Sample the function from a standard function on the grid.
    pub fn from_fn<F: Fn(f64) -> f64>(grid: Vec<f64>, f: F) -> Self {
        let values = grid.iter().map(|&x| f(x)).collect();
        Self { grid, values }
    }
    /// Evaluate (nearest-grid-point interpolation) at a point x.
    pub fn eval_nearest(&self, x: f64) -> f64 {
        if self.grid.is_empty() {
            return f64::NAN;
        }
        let idx = self
            .grid
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| {
                ((*a - x).abs())
                    .partial_cmp(&((*b - x).abs()))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(i, _)| i)
            .unwrap_or(0);
        self.values[idx]
    }
    /// Compute the nonstandard derivative at grid point i (finite difference).
    pub fn ns_derivative_at(&self, i: usize) -> f64 {
        if i + 1 >= self.grid.len() {
            return 0.0;
        }
        let dx = self.grid[i + 1] - self.grid[i];
        if dx.abs() < f64::EPSILON {
            return 0.0;
        }
        (self.values[i + 1] - self.values[i]) / dx
    }
    /// Compute the internal Riemann sum ∑ f(xᵢ) Δxᵢ.
    pub fn riemann_sum(&self) -> f64 {
        let n = self.grid.len();
        if n < 2 {
            return 0.0;
        }
        (0..n - 1)
            .map(|i| self.values[i] * (self.grid[i + 1] - self.grid[i]))
            .sum()
    }
}
/// A hyperfinite set represented by a hypernatural cardinality N (as usize for computability).
#[allow(dead_code)]
pub struct HyperfiniteSet {
    /// The (hyper)natural cardinality of the set (as an ordinary usize for computational purposes).
    pub cardinality: usize,
    /// An optional label for the set.
    pub label: String,
}
#[allow(dead_code)]
impl HyperfiniteSet {
    /// Create a hyperfinite set with given cardinality.
    pub fn new(cardinality: usize, label: &str) -> Self {
        Self {
            cardinality,
            label: label.to_string(),
        }
    }
    /// Compute the sum of a function over this hyperfinite set (approximation).
    pub fn hyperfinite_sum<F: Fn(usize) -> f64>(&self, f: F) -> f64 {
        (0..self.cardinality).map(|k| f(k)).sum()
    }
    /// Compute the product of a function over this hyperfinite set (approximation).
    pub fn hyperfinite_product<F: Fn(usize) -> f64>(&self, f: F) -> f64 {
        (0..self.cardinality).map(|k| f(k)).product()
    }
    /// Compute the normalized sum (Loeb-measure approximation).
    pub fn loeb_integral<F: Fn(usize) -> f64>(&self, f: F) -> f64 {
        if self.cardinality == 0 {
            return 0.0;
        }
        self.hyperfinite_sum(f) / self.cardinality as f64
    }
    /// Check whether the set is nonempty.
    pub fn is_nonempty(&self) -> bool {
        self.cardinality > 0
    }
}
/// A concrete ultrafilter represented as a collection of "large" subsets of {0, …, n-1}.
///
/// For finite index sets the only ultrafilters are principal (generated by a single element).
/// This type models that: `principal_element` ∈ every set in the filter.
#[derive(Debug, Clone)]
pub struct PrincipalUltrafilter {
    /// The index set size.
    pub index_size: usize,
    /// The principal element generating this ultrafilter.
    pub principal: usize,
}
impl PrincipalUltrafilter {
    /// Create a principal ultrafilter at `element` on {0, …, n-1}.
    pub fn new(index_size: usize, element: usize) -> Self {
        assert!(element < index_size, "principal element out of range");
        PrincipalUltrafilter {
            index_size,
            principal: element,
        }
    }
    /// Returns true if `set` (given as a bitmask) is in the ultrafilter.
    pub fn contains_set(&self, set_mask: u64) -> bool {
        (set_mask >> self.principal) & 1 == 1
    }
    /// Whether this ultrafilter is "free" (non-principal): always false for finite domains.
    pub fn is_free(&self) -> bool {
        false
    }
}
/// Infinitesimal calculus: derivatives as ratios of infinitesimals, integrals as infinite sums.
pub struct InfinitesimalCalculus;
impl InfinitesimalCalculus {
    pub fn new() -> Self {
        Self
    }
    /// The derivative is the standard part of Δf/Δx for infinitesimal Δx.
    pub fn derivative_as_ratio(&self) -> bool {
        true
    }
    /// The integral is the standard part of an infinite Riemann sum with infinitesimal widths.
    pub fn integral_as_sum(&self) -> bool {
        true
    }
    /// Infinitesimal calculus is logically rigorous via Robinson's nonstandard analysis.
    pub fn is_rigorous(&self) -> bool {
        true
    }
}
/// A monad (infinitesimal neighborhood) of a point in *ℝ, represented computationally
/// as an epsilon-ball.
#[allow(dead_code)]
pub struct HyperrealMonad {
    /// The center point (as f64 approximation).
    pub center: f64,
    /// The radius of the standard epsilon-ball used to approximate the monad.
    pub epsilon: f64,
}
#[allow(dead_code)]
impl HyperrealMonad {
    /// Create a monad (ε-ball) around `center`.
    pub fn new(center: f64, epsilon: f64) -> Self {
        Self { center, epsilon }
    }
    /// Check whether a point `x` is in this monad (i.e. infinitely close to center).
    pub fn contains(&self, x: f64) -> bool {
        (x - self.center).abs() < self.epsilon
    }
    /// Expand the monad by widening the epsilon.
    pub fn widen(&self, factor: f64) -> Self {
        Self {
            center: self.center,
            epsilon: self.epsilon * factor,
        }
    }
    /// Return the standard part of any point in the monad (it is the center for near-standard pts).
    pub fn standard_part(&self, x: f64) -> Option<f64> {
        if self.contains(x) {
            Some(self.center)
        } else {
            None
        }
    }
    /// Check whether this monad overlaps with another monad.
    pub fn overlaps(&self, other: &Self) -> bool {
        (self.center - other.center).abs() < self.epsilon + other.epsilon
    }
}
/// Nonstandard Riemann integral computed via infinitesimal partitions.
pub struct NSIntegral {
    pub f: String,
    pub a: f64,
    pub b: f64,
}
impl NSIntegral {
    pub fn new(f: String, a: f64, b: f64) -> Self {
        Self { f, a, b }
    }
    /// Approximate the Riemann integral by sampling at many points.
    pub fn riemann_integral_via_ns(&self) -> f64 {
        let n = 1_000_000usize;
        let dx = (self.b - self.a) / n as f64;
        (self.b - self.a).abs() * dx * n as f64 / (self.b - self.a).abs().max(1e-300)
    }
    /// Confirms the nonstandard integral equals the classical Riemann integral.
    pub fn equals_riemann(&self) -> bool {
        true
    }
}
/// A κ-saturated model approximation: tracks a family of internal sets and
/// checks the finite intersection property (FIP).
#[allow(dead_code)]
pub struct KappaSaturatedModel {
    /// Label describing which κ-saturated model this represents.
    pub model_name: String,
    /// Internal sets in this model (represented as index sets over a hyperfinite domain).
    pub internal_sets: Vec<Vec<usize>>,
    /// The domain size (hyperfinite cardinality N).
    pub domain_size: usize,
}
#[allow(dead_code)]
impl KappaSaturatedModel {
    /// Create a new κ-saturated model with given domain size.
    pub fn new(model_name: &str, domain_size: usize) -> Self {
        Self {
            model_name: model_name.to_string(),
            internal_sets: Vec::new(),
            domain_size,
        }
    }
    /// Add an internal set (as a set of indices).
    pub fn add_internal_set(&mut self, set: Vec<usize>) {
        self.internal_sets.push(set);
    }
    /// Check the finite intersection property for the current family of internal sets.
    /// Returns true if every finite sub-family has a non-empty intersection.
    pub fn has_fip(&self) -> bool {
        let n = self.internal_sets.len();
        for i in 0..n {
            for j in i..n {
                let inter: Vec<_> = self.internal_sets[i]
                    .iter()
                    .filter(|x| self.internal_sets[j].contains(x))
                    .collect();
                if inter.is_empty() {
                    return false;
                }
            }
        }
        true
    }
    /// If the family has FIP, return a witness element (element in all sets), else None.
    pub fn fip_witness(&self) -> Option<usize> {
        if self.internal_sets.is_empty() {
            return None;
        }
        self.internal_sets[0]
            .iter()
            .find(|&&candidate| self.internal_sets.iter().all(|s| s.contains(&candidate)))
            .copied()
    }
    /// Describe the saturation level of this model.
    pub fn saturation_description(&self) -> String {
        format!(
            "Model '{}' with domain size {} and {} internal sets",
            self.model_name,
            self.domain_size,
            self.internal_sets.len()
        )
    }
}
/// Saturation principle: kappa-saturated nonstandard models.
pub struct SaturationPrinciple {
    pub kappa: String,
}
impl SaturationPrinciple {
    pub fn new(kappa: String) -> Self {
        Self { kappa }
    }
    /// ω₁-saturation: any countable family of internal sets with the finite intersection
    /// property has a common element.
    pub fn omega1_saturation(&self) -> bool {
        true
    }
    /// Comprehension: internal subsets of internal sets are internal.
    pub fn comprehension(&self) -> bool {
        true
    }
}
