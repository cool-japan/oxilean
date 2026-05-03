//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

/// Simplified Nevanlinna characteristic computation for rational functions.
///
/// For a meromorphic function f(z) = p(z)/q(z), the characteristic growth
/// is determined by max(deg p, deg q).
#[derive(Debug, Clone)]
pub struct NevanlinnaData {
    /// Degree of numerator polynomial.
    pub numerator_degree: usize,
    /// Degree of denominator polynomial (pole order at infinity).
    pub denominator_degree: usize,
}
impl NevanlinnaData {
    /// Create `NevanlinnaData` for a rational function.
    pub fn new(numerator_degree: usize, denominator_degree: usize) -> Self {
        NevanlinnaData {
            numerator_degree,
            denominator_degree,
        }
    }
    /// The order of growth: max(deg p, deg q) for rational functions.
    pub fn order_of_growth(&self) -> usize {
        self.numerator_degree.max(self.denominator_degree)
    }
    /// Number of distinct values omitted (Picard: entire functions omit at most 1).
    pub fn picard_omitted_values(&self) -> usize {
        if self.denominator_degree == 0 {
            1
        } else {
            0
        }
    }
    /// Deficiency sum bound: ∑ δ(a, f) ≤ 2 (Nevanlinna deficiency relation).
    pub fn deficiency_sum_bound(&self) -> f64 {
        2.0
    }
}
/// A smooth projective curve over a number field, tracked by genus.
#[derive(Debug, Clone)]
pub struct ProjectiveCurve {
    /// The geometric genus g ≥ 0.
    pub genus: usize,
    /// Description string (for display).
    pub description: String,
}
impl ProjectiveCurve {
    /// Create a new `ProjectiveCurve` with the given genus.
    pub fn new(genus: usize, description: &str) -> Self {
        ProjectiveCurve {
            genus,
            description: description.to_string(),
        }
    }
    /// Faltings' theorem: the curve has finitely many rational points iff genus ≥ 2.
    pub fn has_finitely_many_rational_points(&self) -> bool {
        self.genus >= 2
    }
    /// Whether the curve is rational (genus 0).
    pub fn is_rational(&self) -> bool {
        self.genus == 0
    }
    /// Whether the curve is elliptic (genus 1).
    pub fn is_elliptic(&self) -> bool {
        self.genus == 1
    }
    /// Riemann-Roch: dimension of L(D) for a divisor of degree d ≥ g.
    pub fn riemann_roch_dim(&self, degree: i64) -> i64 {
        if degree >= self.genus as i64 {
            degree - self.genus as i64 + 1
        } else {
            0
        }
    }
}
/// Data for Mordell-Weil theorem on abelian varieties.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MordellWeilData {
    /// Variety description.
    pub variety: String,
    /// Base field.
    pub field: String,
    /// Rank of the free part.
    pub rank: usize,
    /// Torsion subgroup description.
    pub torsion: String,
    /// Generators of the free part.
    pub generators: Vec<String>,
}
#[allow(dead_code)]
impl MordellWeilData {
    /// Creates Mordell-Weil data.
    pub fn new(variety: &str, field: &str) -> Self {
        MordellWeilData {
            variety: variety.to_string(),
            field: field.to_string(),
            rank: 0,
            torsion: "trivial".to_string(),
            generators: Vec::new(),
        }
    }
    /// Sets rank.
    pub fn with_rank(mut self, r: usize) -> Self {
        self.rank = r;
        self
    }
    /// Sets torsion.
    pub fn with_torsion(mut self, t: &str) -> Self {
        self.torsion = t.to_string();
        self
    }
    /// Adds a generator.
    pub fn add_generator(&mut self, gen: &str) {
        self.generators.push(gen.to_string());
    }
    /// Returns the structure theorem statement.
    pub fn structure_theorem(&self) -> String {
        format!(
            "{}({}) ≅ Z^{} ⊕ ({})",
            self.variety, self.field, self.rank, self.torsion
        )
    }
    /// Returns the Mordell-Weil rank.
    pub fn mw_rank(&self) -> usize {
        self.rank
    }
}
/// Computes the naive Weil height for a projective point given as integer coords.
#[derive(Debug, Clone)]
pub struct NaiveHeightComputer {
    /// Homogeneous integer coordinates.
    pub coords: Vec<i64>,
}
#[allow(dead_code)]
impl NaiveHeightComputer {
    /// Create a new `NaiveHeightComputer` with the given coordinates.
    pub fn new(coords: Vec<i64>) -> Self {
        NaiveHeightComputer { coords }
    }
    /// Naive height H(P) = max |x_i|.
    pub fn naive_height(&self) -> i64 {
        self.coords.iter().map(|x| x.abs()).max().unwrap_or(0)
    }
    /// Logarithmic height h(P) = log H(P).
    pub fn log_height(&self) -> f64 {
        let h = self.naive_height();
        if h == 0 {
            0.0
        } else {
            (h as f64).ln()
        }
    }
    /// Number of coordinates.
    pub fn number_of_coords(&self) -> usize {
        self.coords.len()
    }
    /// Check if all coordinates are zero (degenerate point).
    pub fn is_zero(&self) -> bool {
        self.coords.iter().all(|&x| x == 0)
    }
}
/// Represents a height function on algebraic varieties.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HeightFunction {
    /// Name of the height function.
    pub name: String,
    /// Whether the Northcott property holds (finite points of bounded height and degree).
    pub northcott: bool,
    /// Logarithmic Weil height on the base field.
    pub base_field: String,
}
#[allow(dead_code)]
impl HeightFunction {
    /// Creates a height function.
    pub fn new(name: &str, base_field: &str) -> Self {
        HeightFunction {
            name: name.to_string(),
            northcott: false,
            base_field: base_field.to_string(),
        }
    }
    /// Sets Northcott property.
    pub fn with_northcott(mut self) -> Self {
        self.northcott = true;
        self
    }
    /// Weil height of a rational number p/q (in lowest terms): h(p/q) = log(max(|p|, |q|)).
    pub fn weil_height_rational(p: i64, q: i64) -> f64 {
        if q == 0 {
            return f64::INFINITY;
        }
        (p.abs().max(q.abs()) as f64).ln()
    }
    /// Naive height of (x_0 : ... : x_n) in projective space: max |x_i|.
    pub fn projective_height(coords: &[i64]) -> f64 {
        coords.iter().map(|&x| x.unsigned_abs()).max().unwrap_or(0) as f64
    }
    /// Logarithmic height.
    pub fn log_height(coords: &[i64]) -> f64 {
        Self::projective_height(coords).max(1.0).ln()
    }
    /// Checks Northcott's theorem consequence: there are finitely many points h(P) <= B.
    pub fn northcott_bound(&self, bound: f64) -> String {
        if self.northcott {
            format!(
                "Finitely many points P over {} with h(P) <= {bound:.2}",
                self.base_field
            )
        } else {
            format!("Northcott property not established for {}", self.name)
        }
    }
}
/// Represents a line bundle on a smooth projective variety by its numerical data.
#[derive(Debug, Clone)]
pub struct LineBundleData {
    /// Self-intersection number (for surfaces: L²).
    pub self_intersection: i64,
    /// Intersection with a curve class (for ampleness checks).
    pub curve_intersections: Vec<i64>,
}
impl LineBundleData {
    /// Create a `LineBundleData` with given self-intersection and curve intersections.
    pub fn new(self_intersection: i64, curve_intersections: Vec<i64>) -> Self {
        LineBundleData {
            self_intersection,
            curve_intersections,
        }
    }
    /// Check ampleness via Nakai-Moishezon on a surface: L² > 0 and L·C > 0 for all curves.
    pub fn is_ample_nakai_moishezon(&self) -> bool {
        self.self_intersection > 0 && self.curve_intersections.iter().all(|&c| c > 0)
    }
    /// Check the nef property: L·C ≥ 0 for all curves.
    pub fn is_nef(&self) -> bool {
        self.curve_intersections.iter().all(|&c| c >= 0)
    }
    /// Compute the Seshadri constant (simplified: min ratio L·C / mult_x(C)).
    pub fn seshadri_constant(&self, multiplicities: &[u64]) -> f64 {
        self.curve_intersections
            .iter()
            .zip(multiplicities.iter())
            .filter(|(_, &m)| m > 0)
            .map(|(&lc, &m)| lc as f64 / m as f64)
            .fold(f64::INFINITY, f64::min)
    }
}
/// An abc triple: coprime positive integers a, b, c with a + b = c.
#[derive(Debug, Clone)]
pub struct ABCTriple {
    pub a: u64,
    pub b: u64,
    pub c: u64,
}
impl ABCTriple {
    /// Construct an ABCTriple, verifying a + b = c.
    pub fn new(a: u64, b: u64, c: u64) -> Option<Self> {
        if a + b == c && gcd(a, b) == 1 && gcd(b, c) == 1 {
            Some(ABCTriple { a, b, c })
        } else {
            None
        }
    }
    /// Compute the radical rad(abc) = product of distinct prime factors.
    pub fn radical(&self) -> u64 {
        radical(self.a) / gcd(radical(self.a), 1)
            * (radical(self.b) / gcd(radical(self.b), 1))
            * (radical(self.c) / gcd(radical(self.c), 1))
    }
    /// Compute the quality q = log(c) / log(rad(abc)).
    pub fn quality(&self) -> f64 {
        let r = self.radical();
        if r <= 1 {
            return 0.0;
        }
        (self.c as f64).ln() / (r as f64).ln()
    }
}
/// Data related to Faltings' theorem (Mordell conjecture).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FaltingsData {
    /// Curve description.
    pub curve: String,
    /// Genus.
    pub genus: usize,
    /// Number of rational points (if finite and known).
    pub rational_points_count: Option<usize>,
    /// Whether finitely many rational points are expected (genus >= 2).
    pub finitely_many: bool,
}
#[allow(dead_code)]
impl FaltingsData {
    /// Creates Faltings data.
    pub fn new(curve: &str, genus: usize) -> Self {
        FaltingsData {
            curve: curve.to_string(),
            genus,
            rational_points_count: None,
            finitely_many: genus >= 2,
        }
    }
    /// Sets the number of rational points.
    pub fn with_point_count(mut self, n: usize) -> Self {
        self.rational_points_count = Some(n);
        self
    }
    /// Returns the Faltings theorem statement.
    pub fn faltings_statement(&self) -> String {
        if self.genus >= 2 {
            format!(
                "C = {} has only finitely many rational points (Faltings 1983)",
                self.curve
            )
        } else {
            format!(
                "C = {} genus {} - Faltings does not apply",
                self.curve, self.genus
            )
        }
    }
    /// Checks the genus condition.
    pub fn applies(&self) -> bool {
        self.genus >= 2
    }
}
/// Estimates the rank of a Selmer group via a 2-descent-style bound.
///
/// In practice, the Selmer rank equals the Mordell-Weil rank plus the
/// number of independent Sha elements, but here we give an upper bound.
#[derive(Debug, Clone)]
pub struct SelmerGroupEstimator {
    /// Number of bad primes (primes of bad reduction).
    pub num_bad_primes: usize,
    /// The 2-rank of the torsion subgroup.
    pub two_torsion_rank: usize,
}
#[allow(dead_code)]
impl SelmerGroupEstimator {
    /// Create a new `SelmerGroupEstimator`.
    pub fn new(num_bad_primes: usize, two_torsion_rank: usize) -> Self {
        SelmerGroupEstimator {
            num_bad_primes,
            two_torsion_rank,
        }
    }
    /// Upper bound on the 2-Selmer rank: 2-torsion rank + num_bad_primes + 1.
    pub fn rank_upper_bound(&self) -> usize {
        self.two_torsion_rank + self.num_bad_primes + 1
    }
    /// Whether the Selmer group is provably trivial (both params are zero).
    pub fn is_trivially_trivial(&self) -> bool {
        self.num_bad_primes == 0 && self.two_torsion_rank == 0
    }
    /// A Birch-Swinnerton-Dyer style parity conjecture: if analytic rank is odd,
    /// the algebraic rank is odd (rank upper bound is at least 1).
    pub fn parity_conjecture_rank_lower_bound(&self) -> usize {
        0
    }
}
/// Data for Arakelov geometry on an arithmetic surface.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ArakelovData {
    /// Arithmetic surface description.
    pub surface: String,
    /// Arithmetic degree of a divisor.
    pub arithmetic_degree: f64,
    /// Green's function data (metric on Hermitian line bundle).
    pub green_function_values: Vec<(f64, f64)>,
}
#[allow(dead_code)]
impl ArakelovData {
    /// Creates Arakelov data.
    pub fn new(surface: &str) -> Self {
        ArakelovData {
            surface: surface.to_string(),
            arithmetic_degree: 0.0,
            green_function_values: Vec::new(),
        }
    }
    /// Sets arithmetic degree.
    pub fn with_arith_degree(mut self, d: f64) -> Self {
        self.arithmetic_degree = d;
        self
    }
    /// Adds a Green's function value at (x, y).
    pub fn add_green_value(&mut self, x: f64, g: f64) {
        self.green_function_values.push((x, g));
    }
    /// Computes the Faltings height from Arakelov data.
    pub fn faltings_height_estimate(&self) -> f64 {
        self.arithmetic_degree / 2.0
    }
    /// Noether formula: χ(O_X) = (K^2 + e(X))/12, returns description.
    pub fn noether_formula(&self) -> String {
        format!(
            "χ(O_{{{}}}) = (K² + e(X))/12 (Noether formula)",
            self.surface
        )
    }
}
/// Represents an elliptic curve E: y^2 = x^3 + ax + b over Q with arithmetic data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EllipticCurveArithmetic {
    /// Coefficient a.
    pub a: i64,
    /// Coefficient b.
    pub b: i64,
    /// Rational points (x, y) as pairs.
    pub rational_points: Vec<(i64, i64)>,
    /// Mordell-Weil rank.
    pub rank: usize,
    /// Torsion subgroup order.
    pub torsion_order: usize,
    /// Conductor N.
    pub conductor: Option<u64>,
}
#[allow(dead_code)]
impl EllipticCurveArithmetic {
    /// Creates an elliptic curve.
    pub fn new(a: i64, b: i64) -> Self {
        EllipticCurveArithmetic {
            a,
            b,
            rational_points: Vec::new(),
            rank: 0,
            torsion_order: 1,
            conductor: None,
        }
    }
    /// Adds a rational point.
    pub fn add_rational_point(&mut self, x: i64, y: i64) {
        if y * y == x * x * x + self.a * x + self.b {
            self.rational_points.push((x, y));
        }
    }
    /// Discriminant Δ = -16(4a^3 + 27b^2).
    pub fn discriminant(&self) -> i64 {
        -16 * (4 * self.a.pow(3) + 27 * self.b.pow(2))
    }
    /// j-invariant: j = 1728 * (4a)^3 / Δ.
    pub fn j_invariant(&self) -> Option<f64> {
        let disc = self.discriminant();
        if disc == 0 {
            return None;
        }
        let num = 1728.0 * (4.0 * self.a as f64).powi(3);
        Some(num / disc as f64)
    }
    /// Returns true if E is a smooth (non-singular) curve.
    pub fn is_smooth(&self) -> bool {
        self.discriminant() != 0
    }
    /// Checks if a point (x, y) is on the curve.
    pub fn is_on_curve(&self, x: i64, y: i64) -> bool {
        y * y == x * x * x + self.a * x + self.b
    }
    /// Mordell's theorem: E(Q) is finitely generated.
    pub fn mordell_theorem(&self) -> String {
        format!("E(Q) ≅ Z^{} ⊕ Z/{}", self.rank, self.torsion_order)
    }
    /// Point doubling formula (for rational points): returns 2P.
    pub fn double_point(&self, x: i64, y: i64) -> Option<(f64, f64)> {
        if y == 0 {
            return None;
        }
        let xf = x as f64;
        let yf = y as f64;
        let af = self.a as f64;
        let lambda = (3.0 * xf * xf + af) / (2.0 * yf);
        let x3 = lambda * lambda - 2.0 * xf;
        let y3 = lambda * (xf - x3) - yf;
        Some((x3, y3))
    }
}
/// A simple Thue equation solver for F(x,y) = m where F is a degree-d polynomial.
///
/// Stores the polynomial by its coefficient vector: a_0 x^d + a_1 x^{d-1} y + ... + a_d y^d.
#[derive(Debug, Clone)]
pub struct ThueSolver {
    /// Coefficients of the homogeneous polynomial F, from degree d down to 0.
    pub coeffs: Vec<i64>,
    /// Right-hand side m.
    pub rhs: i64,
}
#[allow(dead_code)]
impl ThueSolver {
    /// Create a new `ThueSolver` for F(x,y) = rhs.
    pub fn new(coeffs: Vec<i64>, rhs: i64) -> Self {
        ThueSolver { coeffs, rhs }
    }
    /// Degree of the Thue form.
    pub fn degree(&self) -> usize {
        self.coeffs.len().saturating_sub(1)
    }
    /// Evaluate F(x, y): F = sum_{i=0}^{d} coeffs\[i\] * x^{d-i} * y^i.
    pub fn evaluate(&self, x: i64, y: i64) -> i64 {
        let d = self.degree();
        let mut result = 0i64;
        for (i, &c) in self.coeffs.iter().enumerate() {
            let x_pow = (d - i) as u32;
            let y_pow = i as u32;
            let term = c
                .saturating_mul(x.saturating_pow(x_pow))
                .saturating_mul(y.saturating_pow(y_pow));
            result = result.saturating_add(term);
        }
        result
    }
    /// Search for small solutions (x,y) with |x|,|y| ≤ bound.
    pub fn small_solutions(&self, bound: i64) -> Vec<(i64, i64)> {
        let mut solutions = Vec::new();
        for x in -bound..=bound {
            for y in -bound..=bound {
                if self.evaluate(x, y) == self.rhs {
                    solutions.push((x, y));
                }
            }
        }
        solutions
    }
}
/// A Weil height function on projective space P^n(Q).
///
/// For P = \[x₀ : x₁ : ... : xₙ\] with integer coordinates (no common factor),
/// the naive height is H(P) = max |xᵢ|, and the logarithmic height is h(P) = log H(P).
#[derive(Debug, Clone)]
pub struct WeilHeight {
    /// Coordinates of the projective point (as integers).
    pub coords: Vec<i64>,
}
impl WeilHeight {
    /// Create a new `WeilHeight` for a projective point with given coordinates.
    pub fn new(coords: Vec<i64>) -> Self {
        WeilHeight { coords }
    }
    /// Compute the naive Weil height H(P) = max |xᵢ|.
    pub fn naive_height(&self) -> i64 {
        self.coords.iter().map(|x| x.abs()).max().unwrap_or(0)
    }
    /// Compute the logarithmic Weil height h(P) = log H(P).
    pub fn logarithmic_height(&self) -> f64 {
        let h = self.naive_height();
        if h == 0 {
            0.0
        } else {
            (h as f64).ln()
        }
    }
    /// Check the Northcott property: are there finitely many points with bounded height?
    ///
    /// Returns `true` (the Northcott property holds for standard Weil heights).
    pub fn northcott_property_holds(&self) -> bool {
        true
    }
}
/// A simplified model of the Mordell-Weil group of an elliptic curve.
///
/// We store only the rank and the orders of torsion generators (for illustration).
#[derive(Debug, Clone)]
pub struct MordellWeilGroup {
    /// The rank r of the free part Z^r.
    pub rank: usize,
    /// Orders of cyclic factors in the torsion subgroup (e.g., \[2, 6\] for Z/2 × Z/6).
    pub torsion_orders: Vec<u32>,
}
impl MordellWeilGroup {
    /// Create a new `MordellWeilGroup` with given rank and torsion.
    pub fn new(rank: usize, torsion_orders: Vec<u32>) -> Self {
        MordellWeilGroup {
            rank,
            torsion_orders,
        }
    }
    /// The total size of the torsion subgroup.
    pub fn torsion_size(&self) -> u64 {
        self.torsion_orders.iter().map(|&n| n as u64).product()
    }
    /// Check if the Mordell-Weil group is finite (rank = 0).
    pub fn is_finite(&self) -> bool {
        self.rank == 0
    }
    /// The minimum number of generators needed.
    pub fn num_generators(&self) -> usize {
        self.rank + self.torsion_orders.len()
    }
}
/// Implements the Chabauty-Coleman bound on rational points of a curve.
///
/// For a curve C of genus g over Q with Mordell-Weil rank r < g,
/// the Coleman bound gives |C(Q)| ≤ 2g - 2 + (p - 1) for good prime p > 2g.
#[derive(Debug, Clone)]
pub struct ChabautyBound {
    /// Genus of the curve.
    pub genus: usize,
    /// Mordell-Weil rank (must be < genus for applicability).
    pub rank: usize,
    /// Prime p used for p-adic integration (p > 2g, p good reduction).
    pub prime: u64,
}
#[allow(dead_code)]
impl ChabautyBound {
    /// Create a new `ChabautyBound`.
    pub fn new(genus: usize, rank: usize, prime: u64) -> Self {
        ChabautyBound { genus, rank, prime }
    }
    /// The Chabauty method applies when rank < genus.
    pub fn is_applicable(&self) -> bool {
        self.rank < self.genus
    }
    /// The Coleman bound: |C(Q)| ≤ 2g - 2 + (p - 1) for p > 2g good prime.
    pub fn point_bound(&self) -> u64 {
        if !self.is_applicable() {
            return u64::MAX;
        }
        let g = self.genus as u64;
        let p = self.prime;
        if p == 0 {
            return 2 * g;
        }
        2 * g - 2 + (p - 1)
    }
    /// Improved bound from Stoll: accounts for p-adic residue disks.
    pub fn stoll_bound(&self) -> u64 {
        if !self.is_applicable() {
            return u64::MAX;
        }
        let g = self.genus as u64;
        2 * g - 2 + (self.prime - 1) / 2
    }
}
