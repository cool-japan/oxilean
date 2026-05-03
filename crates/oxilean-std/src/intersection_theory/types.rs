//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

/// A vector bundle with given rank and Chern numbers.
#[derive(Debug, Clone)]
pub struct VectorBundleData {
    /// Rank of the bundle.
    pub rank: usize,
    /// Chern numbers c_1, c_2, …, c_rank (stored as integers, coefficients in A*(X)).
    pub chern_numbers: Vec<i64>,
}
impl VectorBundleData {
    /// Create a trivial bundle of given rank.
    pub fn trivial(rank: usize) -> Self {
        VectorBundleData {
            rank,
            chern_numbers: vec![0; rank],
        }
    }
    /// Create the tautological line bundle O(-1) on P^n (rank 1, c_1 = -1).
    pub fn tautological() -> Self {
        VectorBundleData {
            rank: 1,
            chern_numbers: vec![-1],
        }
    }
    /// Create O(d) on projective space (rank 1, c_1 = d).
    pub fn line_bundle(d: i64) -> Self {
        VectorBundleData {
            rank: 1,
            chern_numbers: vec![d],
        }
    }
    /// The total Chern polynomial evaluated at t: c(E)(t) = ∑_k c_k(E) t^k.
    pub fn total_chern_poly(&self, t: f64) -> f64 {
        let mut result = 1.0_f64;
        for (k, &c_k) in self.chern_numbers.iter().enumerate() {
            result += c_k as f64 * t.powi((k + 1) as i32);
        }
        result
    }
    /// Direct sum E ⊕ F: rank adds, Chern classes multiply (Whitney sum formula).
    pub fn direct_sum(&self, other: &VectorBundleData) -> VectorBundleData {
        let new_rank = self.rank + other.rank;
        let max_k = new_rank;
        let mut chern = vec![0i64; max_k];
        let c_e: Vec<i64> = std::iter::once(1)
            .chain(self.chern_numbers.iter().cloned())
            .collect();
        let c_f: Vec<i64> = std::iter::once(1)
            .chain(other.chern_numbers.iter().cloned())
            .collect();
        for i in 0..c_e.len() {
            for j in 0..c_f.len() {
                if i + j > 0 && i + j <= max_k {
                    chern[i + j - 1] += c_e[i] * c_f[j];
                }
            }
        }
        VectorBundleData {
            rank: new_rank,
            chern_numbers: chern,
        }
    }
    /// Chern character ch(E) in low degrees.
    ///
    /// Always returns at least 3 coefficients:
    /// - ch_0 = rank
    /// - ch_1 = c_1  (0 if no c_1)
    /// - ch_2 = (c_1^2 - 2*c_2) / 2  (uses c_2=0 when bundle is rank 1)
    pub fn chern_character_coeffs(&self) -> Vec<f64> {
        let c1 = self.chern_numbers.first().copied().unwrap_or(0) as f64;
        let c2 = self.chern_numbers.get(1).copied().unwrap_or(0) as f64;
        vec![self.rank as f64, c1, (c1 * c1 - 2.0 * c2) / 2.0]
    }
    /// Euler class = top Chern class c_rank(E).
    pub fn euler_class(&self) -> i64 {
        self.chern_numbers.last().copied().unwrap_or(0)
    }
    /// Todd class coefficients td_0, td_1, td_2 from Chern numbers.
    /// td_0 = 1, td_1 = c_1/2, td_2 = (c_1^2 + c_2)/12.
    pub fn todd_class_coeffs(&self) -> Vec<f64> {
        let c1 = self.chern_numbers.first().copied().unwrap_or(0) as f64;
        let c2 = self.chern_numbers.get(1).copied().unwrap_or(0) as f64;
        vec![1.0, c1 / 2.0, (c1 * c1 + c2) / 12.0]
    }
}
/// Quantum cohomology ring of P^2 (projective plane).
///
/// QH*(P^2) = Z\[H, q\] / (H^3 - q) where q has degree 3.
/// The quantum product satisfies H ★ H ★ H = q · 1.
#[derive(Debug, Clone)]
pub struct QuantumCohomologyP2 {
    /// Novikov parameter q (formal curve class variable).
    pub q: f64,
}
impl QuantumCohomologyP2 {
    /// Create with Novikov parameter q.
    pub fn new(q: f64) -> Self {
        QuantumCohomologyP2 { q }
    }
    /// Quantum product H^a ★ H^b in QH*(P^2), returning coefficient of each generator.
    /// Generators: 1 (deg 0), H (deg 1), H^2 (deg 2), with H^3 = q.
    /// Returns (coeff_1, coeff_H, coeff_H2).
    pub fn quantum_product_h_powers(&self, a: usize, b: usize) -> (f64, f64, f64) {
        let total = a + b;
        match total % 3 {
            0 if total == 0 => (1.0, 0.0, 0.0),
            0 => (self.q.powi((total / 3) as i32), 0.0, 0.0),
            1 => (0.0, self.q.powi(((total - 1) / 3) as i32), 0.0),
            2 => (0.0, 0.0, self.q.powi(((total - 2) / 3) as i32)),
            _ => (0.0, 0.0, 0.0),
        }
    }
}
/// A Chow ring element represented as a polynomial in the hyperplane class H.
/// For P^n, this is Z\[H\]/(H^{n+1}).
#[derive(Debug, Clone, PartialEq)]
pub struct ChowRingElem {
    /// Dimension of the ambient variety.
    pub dim: usize,
    /// Coefficients: coeffs\[k\] = coefficient of H^k.
    pub coeffs: Vec<i64>,
}
impl ChowRingElem {
    /// Create the zero element.
    pub fn zero(dim: usize) -> Self {
        ChowRingElem {
            dim,
            coeffs: vec![0; dim + 1],
        }
    }
    /// Create the unit element 1 = \[X\].
    pub fn one(dim: usize) -> Self {
        let mut c = vec![0i64; dim + 1];
        c[0] = 1;
        ChowRingElem { dim, coeffs: c }
    }
    /// Create the hyperplane class H.
    pub fn hyperplane(dim: usize) -> Self {
        let mut c = vec![0i64; dim + 1];
        if dim >= 1 {
            c[1] = 1;
        }
        ChowRingElem { dim, coeffs: c }
    }
    /// Create from coefficient list, truncating to dim+1 entries.
    pub fn from_coeffs(dim: usize, coeffs: &[i64]) -> Self {
        let mut c = vec![0i64; dim + 1];
        for (i, &v) in coeffs.iter().enumerate().take(dim + 1) {
            c[i] = v;
        }
        ChowRingElem { dim, coeffs: c }
    }
    /// Addition in the Chow ring.
    pub fn add(&self, other: &ChowRingElem) -> Option<ChowRingElem> {
        if self.dim != other.dim {
            return None;
        }
        Some(ChowRingElem {
            dim: self.dim,
            coeffs: self
                .coeffs
                .iter()
                .zip(other.coeffs.iter())
                .map(|(a, b)| a + b)
                .collect(),
        })
    }
    /// Multiplication in Z\[H\]/(H^{n+1}).
    pub fn mul(&self, other: &ChowRingElem) -> Option<ChowRingElem> {
        if self.dim != other.dim {
            return None;
        }
        let n = self.dim;
        let mut result = vec![0i64; n + 1];
        for (i, &a) in self.coeffs.iter().enumerate() {
            for (j, &b) in other.coeffs.iter().enumerate() {
                if i + j <= n {
                    result[i + j] += a * b;
                }
            }
        }
        Some(ChowRingElem {
            dim: n,
            coeffs: result,
        })
    }
    /// Degree (top coefficient, giving the intersection number with the point class).
    pub fn degree(&self) -> i64 {
        self.coeffs.last().copied().unwrap_or(0)
    }
    /// Scale by integer.
    pub fn scale(&self, n: i64) -> ChowRingElem {
        ChowRingElem {
            dim: self.dim,
            coeffs: self.coeffs.iter().map(|&c| c * n).collect(),
        }
    }
}
/// Riemann-Hurwitz data for a branched cover of curves.
///
/// For a degree-d map f: X → Y of smooth curves with ramification data,
/// the formula is: 2g(X) - 2 = d * (2g(Y) - 2) + R
/// where R = ∑_p (e_p - 1) is the total ramification.
#[allow(dead_code)]
pub struct RiemannHurwitzData {
    /// Degree of the map.
    pub degree: usize,
    /// Genus of the target curve Y.
    pub genus_target: i64,
    /// Ramification indices at each branch point: (point_label, ramification_index).
    pub ramification: Vec<(String, usize)>,
}
impl RiemannHurwitzData {
    /// Create a new Riemann-Hurwitz computation.
    pub fn new(degree: usize, genus_target: i64, ramification: Vec<(String, usize)>) -> Self {
        RiemannHurwitzData {
            degree,
            genus_target,
            ramification,
        }
    }
    /// Total ramification R = ∑_p (e_p - 1).
    pub fn total_ramification(&self) -> i64 {
        self.ramification.iter().map(|(_, e)| *e as i64 - 1).sum()
    }
    /// Compute genus of the source curve X.
    /// 2g(X) - 2 = d * (2g(Y) - 2) + R => g(X) = 1 + (d*(g_Y - 1) + R/2)
    pub fn genus_source(&self) -> i64 {
        let r = self.total_ramification();
        let d = self.degree as i64;
        let g_y = self.genus_target;
        1 + d * (g_y - 1) + r / 2
    }
    /// Euler characteristic of X: χ(X) = 2 - 2g(X).
    pub fn euler_char_source(&self) -> i64 {
        2 - 2 * self.genus_source()
    }
    /// Check if the Riemann-Hurwitz formula is consistent (R must be even).
    pub fn is_consistent(&self) -> bool {
        let r = self.total_ramification();
        let d = self.degree as i64;
        let g_y = self.genus_target;
        let rhs = d * (2 * g_y - 2) + r;
        rhs % 2 == 0 && (rhs + 2) / 2 >= 0
    }
}
/// Schubert polynomial evaluator.
///
/// Represents Schubert polynomials S_w in terms of divided differences.
/// For the special case of vexillary permutations, computes via Chern classes.
#[allow(dead_code)]
pub struct SchubertPolynomialEngine {
    /// Number of variables.
    pub n_vars: usize,
}
impl SchubertPolynomialEngine {
    /// Create a new engine for permutations in S_n.
    pub fn new(n_vars: usize) -> Self {
        SchubertPolynomialEngine { n_vars }
    }
    /// Evaluate the Schubert polynomial S_w at a point x = (x_1, …, x_n).
    ///
    /// For the longest permutation w_0 = (n, n-1, …, 1):
    /// S_{w_0}(x) = x_1^{n-1} x_2^{n-2} … x_{n-1}.
    pub fn longest_word_eval(&self, x: &[f64]) -> f64 {
        let n = self.n_vars;
        x.iter()
            .take(n - 1)
            .enumerate()
            .map(|(i, &xi)| xi.powi((n - 1 - i) as i32))
            .product()
    }
    /// Evaluate a single variable Schubert polynomial S_{s_i} = x_1 + … + x_i.
    /// This corresponds to the simple transposition s_i = (i, i+1).
    pub fn simple_transposition_eval(&self, i: usize, x: &[f64]) -> f64 {
        x.iter().take(i).sum()
    }
    /// Grothendieck polynomial β-deformation: G_w = S_w + lower terms.
    /// Here we return the leading term (Schubert polynomial) for illustration.
    pub fn grothendieck_leading(&self, perm: &[usize], x: &[f64]) -> f64 {
        let n = perm.len().min(x.len());
        let mut val = 0.0f64;
        for i in 0..n {
            val += x[i] * (perm[i] as f64);
        }
        val
    }
    /// Number of inversions in a permutation.
    pub fn inversions(perm: &[usize]) -> usize {
        let n = perm.len();
        let mut count = 0;
        for i in 0..n {
            for j in i + 1..n {
                if perm[i] > perm[j] {
                    count += 1;
                }
            }
        }
        count
    }
    /// Degree of the Schubert polynomial S_w = number of inversions.
    pub fn degree(perm: &[usize]) -> usize {
        Self::inversions(perm)
    }
}
/// Hilbert function and polynomial for a graded ring / projective variety.
///
/// For a projective variety X ⊂ P^n of degree d and dimension r,
/// the Hilbert polynomial is H(X, m) = d * C(m + r, r) (asymptotically).
#[allow(dead_code)]
pub struct HilbertPolynomial {
    /// Degree of the variety.
    pub degree: i64,
    /// Dimension of the variety.
    pub dim: usize,
    /// Arithmetic genus (constant term correction).
    pub arithmetic_genus: i64,
}
impl HilbertPolynomial {
    /// Create a new Hilbert polynomial for a variety.
    pub fn new(degree: i64, dim: usize, arithmetic_genus: i64) -> Self {
        HilbertPolynomial {
            degree,
            dim,
            arithmetic_genus,
        }
    }
    /// Create the Hilbert polynomial for P^n: H(P^n, m) = C(m+n, n).
    pub fn projective_space(n: usize) -> Self {
        HilbertPolynomial {
            degree: 1,
            dim: n,
            arithmetic_genus: 0,
        }
    }
    /// Evaluate H(m) = degree * C(m + dim, dim) + arithmetic_genus correction.
    pub fn eval(&self, m: i64) -> i64 {
        if m < 0 {
            return 0;
        }
        let binom = it_binomial(m + self.dim as i64, self.dim as i64);
        self.degree * binom + self.arithmetic_genus
    }
    /// Euler characteristic χ(O_X) = H(0).
    pub fn euler_characteristic(&self) -> i64 {
        self.eval(0)
    }
    /// Leading coefficient (degree / dim!).
    pub fn leading_coefficient(&self) -> f64 {
        let dim_factorial: i64 = (1..=self.dim as i64).product();
        self.degree as f64 / dim_factorial as f64
    }
    /// Compute the first few values of the Hilbert function.
    pub fn values(&self, max_m: usize) -> Vec<i64> {
        (0..=max_m as i64).map(|m| self.eval(m)).collect()
    }
}
/// Compute the excess intersection contribution in codimension.
///
/// When two subvarieties X and Y of Z meet in a component W with
/// excess dimension e = dim(X) + dim(Y) - dim(Z) - dim(W),
/// the contribution of W to X · Y is e(E_W) ∩ \[W\] where E_W is the excess bundle.
#[allow(dead_code)]
pub struct ExcessIntersection {
    /// Dimension of ambient variety Z.
    pub dim_z: usize,
    /// Dimension of X.
    pub dim_x: usize,
    /// Dimension of Y.
    pub dim_y: usize,
    /// Dimension of the intersection component W.
    pub dim_w: usize,
}
impl ExcessIntersection {
    /// Create an excess intersection data.
    pub fn new(dim_z: usize, dim_x: usize, dim_y: usize, dim_w: usize) -> Self {
        ExcessIntersection {
            dim_z,
            dim_x,
            dim_y,
            dim_w,
        }
    }
    /// Expected dimension of the intersection: dim_x + dim_y - dim_z.
    pub fn expected_dim(&self) -> i64 {
        self.dim_x as i64 + self.dim_y as i64 - self.dim_z as i64
    }
    /// Excess dimension: dim_w - expected_dim.
    pub fn excess(&self) -> i64 {
        self.dim_w as i64 - self.expected_dim()
    }
    /// Rank of the excess bundle: excess dimension.
    pub fn excess_bundle_rank(&self) -> usize {
        self.excess().max(0) as usize
    }
    /// Check if the intersection is proper (no excess).
    pub fn is_proper(&self) -> bool {
        self.excess() <= 0
    }
    /// Codimension of W in X: dim_x - dim_w.
    pub fn codim_in_x(&self) -> i64 {
        self.dim_x as i64 - self.dim_w as i64
    }
    /// Codimension of W in Y: dim_y - dim_w.
    pub fn codim_in_y(&self) -> i64 {
        self.dim_y as i64 - self.dim_w as i64
    }
}
/// Compute the Bezout bound for a system of polynomial equations.
///
/// For polynomials f_1, …, f_n of degrees d_1, …, d_n in n variables,
/// the number of common zeros (in projective space, counted with multiplicity)
/// is at most ∏ d_i (Bezout's theorem).
#[derive(Debug, Clone)]
pub struct BezoutBound {
    /// Degrees of the polynomials.
    pub degrees: Vec<u64>,
}
impl BezoutBound {
    /// Create a Bezout bound system.
    pub fn new(degrees: Vec<u64>) -> Self {
        BezoutBound { degrees }
    }
    /// The Bezout bound: product of all degrees.
    pub fn bound(&self) -> u64 {
        self.degrees.iter().product()
    }
    /// Mixed Bezout bound (for a subsystem of equations).
    pub fn mixed_bound(&self, indices: &[usize]) -> u64 {
        indices
            .iter()
            .filter_map(|&i| self.degrees.get(i))
            .product()
    }
    /// Check if the system is underdetermined (fewer equations than variables).
    pub fn is_underdetermined(&self, num_vars: usize) -> bool {
        self.degrees.len() < num_vars
    }
    /// Check if the system is overdetermined.
    pub fn is_overdetermined(&self, num_vars: usize) -> bool {
        self.degrees.len() > num_vars
    }
    /// Compute the multi-homogeneous Bezout bound given variable groups.
    /// `groups\[i\]` = (number of variables in group i, degree vector for each polynomial).
    pub fn multi_homogeneous_bound(groups: &[(usize, Vec<u64>)]) -> u64 {
        groups
            .iter()
            .map(|(_, degs)| degs.iter().max().copied().unwrap_or(1))
            .product()
    }
}
/// Intersection matrix for a set of divisors on a surface.
///
/// Entry (i, j) = intersection number D_i · D_j on a surface (dim=2).
#[derive(Debug, Clone)]
pub struct IntersectionMatrix {
    /// Number of divisors.
    pub size: usize,
    /// Matrix entries (row-major).
    pub entries: Vec<Vec<i64>>,
}
impl IntersectionMatrix {
    /// Create from a symmetric matrix of intersection numbers.
    pub fn new(entries: Vec<Vec<i64>>) -> Self {
        let size = entries.len();
        IntersectionMatrix { size, entries }
    }
    /// Self-intersection numbers along the diagonal.
    pub fn self_intersections(&self) -> Vec<i64> {
        (0..self.size).map(|i| self.entries[i][i]).collect()
    }
    /// Compute the determinant (for 1x1, 2x2, and 3x3 matrices).
    pub fn determinant(&self) -> Option<i64> {
        match self.size {
            1 => Some(self.entries[0][0]),
            2 => {
                let a = self.entries[0][0];
                let b = self.entries[0][1];
                let c = self.entries[1][0];
                let d = self.entries[1][1];
                Some(a * d - b * c)
            }
            3 => {
                let m = &self.entries;
                Some(
                    m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
                        - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
                        + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0]),
                )
            }
            _ => None,
        }
    }
    /// Check if the matrix is negative definite (Cartan matrix condition for ADE singularities).
    pub fn is_negative_definite(&self) -> bool {
        let diag_neg = (0..self.size).all(|i| self.entries[i][i] < 0);
        if !diag_neg {
            return false;
        }
        if let Some(det) = self.determinant() {
            if self.size == 2 {
                return det > 0;
            }
        }
        false
    }
}
/// A cycle class in a Chow group, represented as a degree and multiplicity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CycleClass {
    /// Codimension of the cycle.
    pub codim: usize,
    /// Multiplicity (integer coefficient).
    pub multiplicity: i64,
    /// Name of the supporting variety.
    pub name: String,
}
impl CycleClass {
    /// Create a new cycle class.
    pub fn new(codim: usize, multiplicity: i64, name: impl Into<String>) -> Self {
        CycleClass {
            codim,
            multiplicity,
            name: name.into(),
        }
    }
    /// The fundamental class \[X\] in A^0(X).
    pub fn fundamental(name: impl Into<String>) -> Self {
        CycleClass::new(0, 1, name)
    }
    /// Add two cycle classes (same codimension).
    pub fn add(&self, other: &CycleClass) -> Option<CycleClass> {
        if self.codim != other.codim {
            return None;
        }
        Some(CycleClass {
            codim: self.codim,
            multiplicity: self.multiplicity + other.multiplicity,
            name: format!("{} + {}", self.name, other.name),
        })
    }
    /// Scale by an integer.
    pub fn scale(&self, n: i64) -> CycleClass {
        CycleClass {
            codim: self.codim,
            multiplicity: self.multiplicity * n,
            name: format!("{} · {}", n, self.name),
        }
    }
}
/// Chow ring presentation for the complete flag variety FL(1, 2, …, n; C^n).
///
/// A*(FL(n)) = Z\[x_1, …, x_n\] / (e_1, …, e_n) where e_k are elementary
/// symmetric polynomials (representing Chern classes of the tautological bundles).
#[allow(dead_code)]
pub struct FlagVarietyChowRing {
    /// Dimension n (flag variety of n-dimensional space).
    pub n: usize,
}
impl FlagVarietyChowRing {
    /// Create a Chow ring for FL(1, 2, …, n).
    pub fn new(n: usize) -> Self {
        FlagVarietyChowRing { n }
    }
    /// Dimension of FL(1, 2, …, n) = n*(n-1)/2.
    pub fn dim(&self) -> usize {
        self.n * (self.n - 1) / 2
    }
    /// Number of Schubert cells = n! (indexed by permutations in S_n).
    pub fn num_schubert_cells(&self) -> u64 {
        (1..=self.n as u64).product()
    }
    /// Evaluate elementary symmetric polynomial e_k(x_1, …, x_n) at a given point.
    pub fn elementary_sym(&self, k: usize, x: &[f64]) -> f64 {
        if k == 0 {
            return 1.0;
        }
        if k > x.len() {
            return 0.0;
        }
        let mut sum = 0.0f64;
        self.subsets_sum(x, k, 0, 1.0, &mut sum);
        sum
    }
    fn subsets_sum(&self, x: &[f64], k: usize, start: usize, prod: f64, acc: &mut f64) {
        if k == 0 {
            *acc += prod;
            return;
        }
        if start + k > x.len() {
            return;
        }
        for i in start..=x.len() - k {
            self.subsets_sum(x, k - 1, i + 1, prod * x[i], acc);
        }
    }
    /// Check if a monomial x_1^{a_1} … x_n^{a_n} is zero in A*(FL(n)).
    /// A monomial is zero if the total degree exceeds dim(FL(n)).
    pub fn monomial_is_zero(&self, exponents: &[usize]) -> bool {
        let total: usize = exponents.iter().sum();
        total > self.dim()
    }
    /// Compute the Poincare polynomial ∑_k dim(A^k) t^k.
    /// For FL(n), the Poincare polynomial is the q-factorial \[n\]_q!.
    pub fn poincare_poly_coeffs(&self) -> Vec<u64> {
        let mut result = vec![1u64];
        for k in 2..=self.n {
            let mut new_result = vec![0u64; result.len() + k - 1];
            for (i, &c) in result.iter().enumerate() {
                for j in 0..k {
                    new_result[i + j] += c;
                }
            }
            result = new_result;
        }
        result
    }
}
/// Schubert calculus engine for Grassmannian G(k, n).
///
/// Computes intersection numbers using the Pieri and Giambelli rules.
#[derive(Debug, Clone)]
pub struct SchubertCalc {
    /// Number of planes k in G(k, n).
    pub k: usize,
    /// Ambient dimension n.
    pub n: usize,
}
impl SchubertCalc {
    /// Create a new Schubert calculus engine for G(k, n).
    pub fn new(k: usize, n: usize) -> Self {
        SchubertCalc { k, n }
    }
    /// Dimension of G(k, n) = k * (n - k).
    pub fn dim(&self) -> usize {
        self.k * (self.n - self.k)
    }
    /// Check if a partition λ is valid for G(k, n):
    /// must have at most k parts, each ≤ n - k.
    pub fn is_valid_partition(&self, lambda: &[usize]) -> bool {
        lambda.len() <= self.k && lambda.iter().all(|&p| p <= self.n - self.k)
    }
    /// Size (sum of parts) of a partition.
    pub fn partition_size(lambda: &[usize]) -> usize {
        lambda.iter().sum()
    }
    /// Pieri's formula: σ_p · σ_λ in G(k, n).
    /// Returns all Schubert classes σ_μ that appear (each with coefficient 1).
    pub fn pieri(&self, p: usize, lambda: &[usize]) -> Vec<Vec<usize>> {
        let size_lambda = Self::partition_size(lambda);
        let target_size = size_lambda + p;
        let mut result = Vec::new();
        self.pieri_recurse(lambda, p, 0, &mut Vec::new(), target_size, &mut result);
        result
    }
    fn pieri_recurse(
        &self,
        lambda: &[usize],
        remaining: usize,
        pos: usize,
        current: &mut Vec<usize>,
        target: usize,
        result: &mut Vec<Vec<usize>>,
    ) {
        if Self::partition_size(current) == target {
            if self.is_valid_partition(current) {
                result.push(current.clone());
            }
            return;
        }
        if pos >= self.k {
            return;
        }
        let prev_upper = if pos == 0 {
            self.n - self.k
        } else {
            current[pos - 1]
        };
        let lower = lambda.get(pos).copied().unwrap_or(0);
        let upper = prev_upper.min(lower + remaining);
        for val in lower..=upper {
            current.push(val);
            let new_remaining = remaining.saturating_sub(val - lower);
            self.pieri_recurse(lambda, new_remaining, pos + 1, current, target, result);
            current.pop();
        }
    }
    /// Degree of G(k, n) in the Plücker embedding.
    /// Uses the hook-length formula for the rectangular partition (n-k)^k.
    pub fn degree(&self) -> u64 {
        let d = self.dim();
        let mut hook_product: u64 = 1;
        for i in 0..self.k {
            for j in 0..(self.n - self.k) {
                let hook = (self.k - i - 1) + (self.n - self.k - j - 1) + 1;
                hook_product *= hook as u64;
            }
        }
        let mut factorial = 1u64;
        for i in 1..=(d as u64) {
            factorial = factorial.saturating_mul(i);
        }
        factorial / hook_product
    }
}
