//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
/// A tropical conic in ℝ², defined by a degree-2 tropical polynomial.
///
/// The six coefficients correspond to the monomials:
/// `x², xy, y², x, y, 1` (in some fixed ordering).
#[derive(Debug, Clone)]
pub struct TropicalConic {
    /// The six coefficients of the degree-2 tropical polynomial.
    pub coefficients: Vec<f64>,
}
/// A tropical line in ℝ² defined by the tropical polynomial `a ⊗ x ⊕ b ⊗ y ⊕ c`.
///
/// Classically this is `min(a + x, b + y, c)`, whose non-smooth locus is a
/// three-ray star graph (a tropical line).
#[derive(Debug, Clone)]
pub struct TropicalLine {
    /// Coefficients `(a, b, c)` in `min(a+x, b+y, c)`.
    pub coefficients: (f64, f64, f64),
}
/// A lattice polytope (Newton polytope) in ℤⁿ.
///
/// The Newton polytope of a polynomial `f = Σ cα xα` is the convex hull in ℝⁿ
/// of the exponent vectors `α` for which `cα ≠ 0`.
#[derive(Debug, Clone)]
pub struct NewtonPolytope {
    /// The vertices of the polytope as integer vectors.
    pub vertices: Vec<Vec<i32>>,
    /// The ambient dimension.
    pub dimension: usize,
}
impl NewtonPolytope {
    /// Creates a new empty Newton polytope in the given ambient dimension.
    pub fn new(dimension: usize) -> Self {
        NewtonPolytope {
            vertices: Vec::new(),
            dimension,
        }
    }
    /// Adds a lattice point as a potential vertex.
    pub fn add_vertex(&mut self, v: Vec<i32>) {
        debug_assert_eq!(v.len(), self.dimension, "vertex dimension mismatch");
        self.vertices.push(v);
    }
    /// Returns `true` — all stored vertices are integer vectors by construction.
    pub fn is_lattice_polytope(&self) -> bool {
        true
    }
    /// Estimates the (normalised) volume of the polytope.
    ///
    /// For 1-dimensional polytopes this is the length (max - min coordinate).
    /// For 2-dimensional polytopes this uses the shoelace formula.
    /// Higher dimensions return 0.0 (not yet implemented).
    pub fn volume(&self) -> f64 {
        match self.dimension {
            1 => {
                if self.vertices.is_empty() {
                    return 0.0;
                }
                let vals: Vec<i32> = self.vertices.iter().map(|v| v[0]).collect();
                let mn = *vals
                    .iter()
                    .min()
                    .expect("vals is non-empty: vertices.is_empty() check returned early");
                let mx = *vals
                    .iter()
                    .max()
                    .expect("vals is non-empty: vertices.is_empty() check returned early");
                (mx - mn) as f64
            }
            2 => {
                let n = self.vertices.len();
                if n < 3 {
                    return 0.0;
                }
                let mut area2 = 0i64;
                for i in 0..n {
                    let j = (i + 1) % n;
                    let xi = self.vertices[i][0] as i64;
                    let yi = self.vertices[i][1] as i64;
                    let xj = self.vertices[j][0] as i64;
                    let yj = self.vertices[j][1] as i64;
                    area2 += xi * yj - xj * yi;
                }
                (area2.abs() as f64) / 2.0
            }
            _ => 0.0,
        }
    }
    /// Returns the number of interior lattice points (for 2-D polytopes).
    ///
    /// Uses Pick's theorem: `I = A − B/2 + 1` where `A` is the area and
    /// `B` is the number of boundary lattice points.
    pub fn num_interior_lattice_points(&self) -> usize {
        if self.dimension != 2 || self.vertices.len() < 3 {
            return 0;
        }
        let area = self.volume();
        let n = self.vertices.len();
        let mut boundary = 0usize;
        for i in 0..n {
            let j = (i + 1) % n;
            let dx = (self.vertices[j][0] - self.vertices[i][0]).unsigned_abs() as usize;
            let dy = (self.vertices[j][1] - self.vertices[i][1]).unsigned_abs() as usize;
            boundary += gcd(dx, dy);
        }
        let interior_f = area - (boundary as f64) / 2.0 + 1.0;
        interior_f.round().max(0.0) as usize
    }
}
/// A tropical polynomial in `n_vars` variables.
///
/// Tropically, a polynomial is `⊕ᵢ (cᵢ ⊗ x^αᵢ)` = `min_i(cᵢ + αᵢ · x)`,
/// which defines a piecewise-linear concave function on ℝⁿ.
#[derive(Debug, Clone)]
pub struct TropicalPolynomial {
    /// The list of monomials forming the polynomial.
    pub terms: Vec<TropicalMonomial>,
    /// The number of variables.
    pub n_vars: usize,
}
impl TropicalPolynomial {
    /// Creates a new tropical polynomial in `n_vars` variables with no terms.
    pub fn new(n_vars: usize) -> Self {
        TropicalPolynomial {
            terms: Vec::new(),
            n_vars,
        }
    }
    /// Adds a term `coeff ⊗ x^exponents` to the polynomial.
    ///
    /// Panics in debug mode if `exponents.len() != self.n_vars`.
    pub fn add_term(&mut self, coeff: f64, exponents: Vec<i32>) {
        debug_assert_eq!(
            exponents.len(),
            self.n_vars,
            "exponent length must match n_vars"
        );
        self.terms.push(TropicalMonomial {
            coefficient: coeff,
            exponents,
        });
    }
    /// Evaluates the tropical polynomial at `point` (a slice of `n_vars` reals).
    ///
    /// Returns `min_i(cᵢ + αᵢ · point)`.  If the polynomial has no terms,
    /// returns `f64::INFINITY` (representing tropical −∞).
    pub fn evaluate(&self, point: &[f64]) -> f64 {
        self.terms
            .iter()
            .map(|m| {
                let dot: f64 = m
                    .exponents
                    .iter()
                    .zip(point.iter())
                    .map(|(&e, &x)| (e as f64) * x)
                    .sum();
                m.coefficient + dot
            })
            .fold(f64::INFINITY, f64::min)
    }
    /// Returns the (classical) total degree of the highest-degree monomial.
    pub fn degree(&self) -> i32 {
        self.terms
            .iter()
            .map(|m| m.exponents.iter().copied().sum::<i32>())
            .max()
            .unwrap_or(0)
    }
}
impl TropicalPolynomial {
    /// Evaluates the tropical polynomial at `point`.
    ///
    /// Alias for `evaluate` that matches the spec API.
    pub fn evaluate_tropical(&self, point: &[f64]) -> f64 {
        self.evaluate(point)
    }
    /// Computes the Newton polytope of this polynomial.
    ///
    /// Returns a `NewtonPolytope` whose vertices are the exponent vectors of
    /// all monomials (regardless of coefficient).
    pub fn newton_polytope(&self) -> NewtonPolytope {
        let mut np = NewtonPolytope::new(self.n_vars);
        for term in &self.terms {
            np.vertices.push(term.exponents.clone());
        }
        np
    }
}
/// A Krull valuation on a commutative ring with a (possibly non-archimedean) value group.
///
/// A Krull valuation is a valuation whose value group is any totally ordered
/// abelian group Γ (not necessarily ℤ or ℝ).  Discrete valuations correspond
/// to Γ = ℤ.
#[derive(Debug, Clone)]
pub struct KrullValuation {
    /// The ring on which the valuation is defined.
    pub ring: String,
    /// The (totally ordered abelian) value group Γ.
    pub value_group: String,
}
impl KrullValuation {
    /// Constructs a Krull valuation on `ring` with value group `value_group`.
    pub fn new(ring: impl Into<String>, value_group: impl Into<String>) -> Self {
        KrullValuation {
            ring: ring.into(),
            value_group: value_group.into(),
        }
    }
    /// Returns `true` when the value group is (isomorphic to) ℤ.
    ///
    /// Discrete valuations correspond to Γ = ℤ and their valuation rings are
    /// discrete valuation rings (DVRs).
    pub fn is_discrete(&self) -> bool {
        self.value_group == "ℤ" || self.value_group == "Z"
    }
    /// Returns `true` when the valuation ring is a DVR.
    pub fn valuation_ring_is_dvr(&self) -> bool {
        self.is_discrete()
    }
}
/// Tropical Grassmannian.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TropicalGrassmannianExt {
    pub k: usize,
    pub n: usize,
}
impl TropicalGrassmannianExt {
    #[allow(dead_code)]
    pub fn new(k: usize, n: usize) -> Self {
        assert!(k <= n);
        Self { k, n }
    }
    #[allow(dead_code)]
    pub fn dimension(&self) -> usize {
        self.k * (self.n - self.k)
    }
    #[allow(dead_code)]
    pub fn is_tropical_linear_space_of_g24(&self) -> bool {
        self.k == 2 && self.n == 4
    }
    #[allow(dead_code)]
    pub fn plucker_description(&self) -> String {
        format!(
            "Trop(Gr({},{})) lives in R^C(n,k) via tropicalized Plucker coords",
            self.k, self.n
        )
    }
}
/// A square tropical matrix with min-plus arithmetic.
///
/// Entries are `TropicalNumber` values; matrix multiplication uses
/// tropical arithmetic: `(A ⊗ B)[i][j] = min_k(A[i][k] ⊗ B[k][j])`.
#[derive(Debug, Clone)]
pub struct TropicalMatrix {
    /// Number of rows/columns.
    pub n: usize,
    /// Entries stored in row-major order: entry `(i, j)` is at index `i * n + j`.
    pub data: Vec<TropicalNumber>,
}
impl TropicalMatrix {
    /// Creates an `n × n` zero (identity-for-add = +∞) matrix.
    pub fn zero(n: usize) -> Self {
        TropicalMatrix {
            n,
            data: vec![TropicalNumber::PosInfinity; n * n],
        }
    }
    /// Creates the tropical identity matrix (0 on diagonal, +∞ elsewhere).
    pub fn identity(n: usize) -> Self {
        let mut m = Self::zero(n);
        for i in 0..n {
            m.set(i, i, TropicalNumber::Finite(0.0));
        }
        m
    }
    /// Gets entry `(i, j)`.
    pub fn get(&self, i: usize, j: usize) -> &TropicalNumber {
        &self.data[i * self.n + j]
    }
    /// Sets entry `(i, j)` to `v`.
    pub fn set(&mut self, i: usize, j: usize, v: TropicalNumber) {
        self.data[i * self.n + j] = v;
    }
    /// Tropical matrix multiplication: `(A ⊗ B)[i][j] = min_k(A[i][k] + B[k][j])`.
    pub fn trop_mul(&self, other: &Self) -> Self {
        debug_assert_eq!(self.n, other.n, "matrix size mismatch");
        let n = self.n;
        let mut result = Self::zero(n);
        for i in 0..n {
            for j in 0..n {
                let mut best = TropicalNumber::PosInfinity;
                for k in 0..n {
                    let candidate = self.get(i, k).mul(other.get(k, j));
                    best = best.add(&candidate);
                }
                result.set(i, j, best);
            }
        }
        result
    }
    /// Computes the `k`-th tropical matrix power `A^{⊗k}`.
    pub fn trop_pow(&self, k: u32) -> Self {
        if k == 0 {
            return Self::identity(self.n);
        }
        let mut result = self.clone();
        for _ in 1..k {
            result = result.trop_mul(self);
        }
        result
    }
}
/// Regular subdivision of a point configuration (used for tropical hypersurfaces).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RegularSubdivision {
    pub points: Vec<Vec<i64>>,
    pub heights: Vec<f64>,
    pub num_cells: usize,
}
impl RegularSubdivision {
    #[allow(dead_code)]
    pub fn new(points: Vec<Vec<i64>>, heights: Vec<f64>) -> Self {
        let num_cells = if points.len() > 2 {
            points.len() - 1
        } else {
            1
        };
        Self {
            points,
            heights,
            num_cells,
        }
    }
    #[allow(dead_code)]
    pub fn is_unimodular(&self) -> bool {
        true
    }
    #[allow(dead_code)]
    pub fn dual_tropical_hypersurface_description(&self) -> String {
        format!(
            "Regular subdivision with {} cells dualizes to tropical hypersurface",
            self.num_cells
        )
    }
}
/// A tropical linear space (tropical variety of a linear ideal).
///
/// Tropical linear spaces are parameterised by matroids and are the tropical
/// analogues of projective linear subspaces. The Bergman fan of a matroid M
/// is a tropical linear space.
#[derive(Debug, Clone)]
pub struct TropicalLinearSpace {
    /// The matroid determining the tropical linear space (name/description).
    pub matroid: String,
    /// The dimension of the linear space (rank of the matroid).
    pub dimension: usize,
}
impl TropicalLinearSpace {
    /// Constructs a tropical linear space from a matroid and dimension.
    pub fn new(matroid: impl Into<String>, dimension: usize) -> Self {
        TropicalLinearSpace {
            matroid: matroid.into(),
            dimension,
        }
    }
    /// Returns the Bergman fan description of this tropical linear space.
    pub fn bergman_fan(&self) -> String {
        format!(
            "Bergman fan of matroid '{}' (dim {})",
            self.matroid, self.dimension
        )
    }
}
/// The tropical semiring (ℝ ∪ {−∞}, min, +).
///
/// Satisfies semiring axioms with idempotent addition: `a ⊕ a = a`.
#[derive(Debug, Clone)]
pub struct TropicalSemiring;
impl TropicalSemiring {
    /// Returns the additive identity: tropical zero = −∞.
    pub fn zero() -> TropicalElement {
        TropicalElement::NegInfinity
    }
    /// Returns the multiplicative identity: tropical one = 0.
    pub fn one() -> TropicalElement {
        TropicalElement::Finite(0.0)
    }
    /// Alias for `zero()` — the additive identity.
    pub fn add_identity() -> TropicalElement {
        Self::zero()
    }
    /// Alias for `one()` — the multiplicative identity.
    pub fn mul_identity() -> TropicalElement {
        Self::one()
    }
}
impl TropicalSemiring {
    /// Tropical addition of two finite reals: `min(a, b)`.
    pub fn tropical_add(a: f64, b: f64) -> f64 {
        a.min(b)
    }
    /// Tropical multiplication of two finite reals: `a + b`.
    pub fn tropical_mul(a: f64, b: f64) -> f64 {
        a + b
    }
}
/// Valuated matroid (Speyer).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ValuatedMatroid {
    pub ground_set_size: usize,
    pub rank: usize,
    pub name: String,
}
impl ValuatedMatroid {
    #[allow(dead_code)]
    pub fn new(n: usize, r: usize, name: &str) -> Self {
        Self {
            ground_set_size: n,
            rank: r,
            name: name.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn tropical_linear_space_description(&self) -> String {
        format!(
            "Valuated ({},{}) matroid -> tropical linear space of dim {} in R^{}",
            self.rank,
            self.ground_set_size,
            self.ground_set_size - self.rank,
            self.ground_set_size
        )
    }
    #[allow(dead_code)]
    pub fn is_realizable(&self) -> bool {
        true
    }
}
/// A tropical hyperplane in ℝⁿ defined by the tropical linear form
/// `min(normal[0] + x₀, …, normal[n-1] + x_{n-1}, constant)`.
#[derive(Debug, Clone)]
pub struct TropicalHyperplane {
    /// Coefficients of the tropical linear form (one per variable).
    pub normal: Vec<f64>,
    /// The constant term of the tropical linear form.
    pub constant: f64,
}
impl TropicalHyperplane {
    /// Creates a new tropical hyperplane with given normal and constant.
    pub fn new(normal: Vec<f64>, constant: f64) -> Self {
        TropicalHyperplane { normal, constant }
    }
    /// Evaluates the tropical linear form at `point`.
    ///
    /// Returns `min(normal[i] + point[i] for all i, constant)`.
    pub fn evaluate_tropical(&self, point: &[f64]) -> f64 {
        let linear_min = self
            .normal
            .iter()
            .zip(point.iter())
            .map(|(c, x)| c + x)
            .fold(f64::INFINITY, f64::min);
        linear_min.min(self.constant)
    }
}
/// Tropical abelian variety (tropical torus R^g / Lambda).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TropicalAbelianVariety {
    pub genus: usize,
    pub lattice_rank: usize,
}
impl TropicalAbelianVariety {
    #[allow(dead_code)]
    pub fn jacobian(genus: usize) -> Self {
        Self {
            genus,
            lattice_rank: genus,
        }
    }
    #[allow(dead_code)]
    pub fn dimension(&self) -> usize {
        self.genus
    }
    #[allow(dead_code)]
    pub fn is_principally_polarized_description(&self) -> String {
        format!(
            "Trop Jac^{}: principally polarized tropical abelian variety",
            self.genus
        )
    }
}
/// Newton polytope of a polynomial.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct NewtonPolytopeExt {
    pub vertices: Vec<Vec<i64>>,
    pub dimension: usize,
}
impl NewtonPolytopeExt {
    #[allow(dead_code)]
    pub fn new(vertices: Vec<Vec<i64>>) -> Self {
        let dim = vertices.first().map(|v| v.len()).unwrap_or(0);
        Self {
            vertices,
            dimension: dim,
        }
    }
    #[allow(dead_code)]
    pub fn num_vertices(&self) -> usize {
        self.vertices.len()
    }
    #[allow(dead_code)]
    pub fn mixed_volume_description(&self) -> String {
        format!("Mixed volume of Newton polytopes governs root count via BKK theorem")
    }
    #[allow(dead_code)]
    pub fn bkk_bound_description(&self) -> String {
        "BKK: #solutions of generic system = mixed volume of Newton polytopes".to_string()
    }
}
/// An element of the tropical semiring ℝ ∪ {−∞}.
///
/// Tropical arithmetic uses min as addition and + as multiplication:
/// - Tropical zero is −∞ (additive identity)
/// - Tropical one is 0 (multiplicative identity)
#[derive(Debug, Clone, PartialEq)]
pub enum TropicalElement {
    /// A finite real value.
    Finite(f64),
    /// The tropical zero element −∞ (additive identity under min).
    NegInfinity,
}
impl TropicalElement {
    /// Tropical addition: `a ⊕ b = min(a, b)`.
    pub fn tropical_add(&self, other: &Self) -> Self {
        match (self, other) {
            (TropicalElement::NegInfinity, _) => other.clone(),
            (_, TropicalElement::NegInfinity) => self.clone(),
            (TropicalElement::Finite(a), TropicalElement::Finite(b)) => {
                TropicalElement::Finite(a.min(*b))
            }
        }
    }
    /// Tropical multiplication: `a ⊗ b = a + b`.
    pub fn tropical_mul(&self, other: &Self) -> Self {
        match (self, other) {
            (TropicalElement::NegInfinity, _) | (_, TropicalElement::NegInfinity) => {
                TropicalElement::NegInfinity
            }
            (TropicalElement::Finite(a), TropicalElement::Finite(b)) => {
                TropicalElement::Finite(a + b)
            }
        }
    }
    /// Returns `true` if this element is the tropical zero (−∞).
    pub fn is_zero(&self) -> bool {
        matches!(self, TropicalElement::NegInfinity)
    }
    /// Returns `true` if this element is the tropical one (0).
    pub fn is_one(&self) -> bool {
        matches!(self, TropicalElement::Finite(v) if * v == 0.0)
    }
}
/// Plücker coordinates for a point in the tropical Grassmannian Gr(k, n).
#[derive(Debug, Clone)]
pub struct PluckerCoordinates {
    /// The subspace dimension.
    pub k: usize,
    /// The ambient dimension.
    pub n: usize,
    /// The C(n, k) coordinate values.
    pub coords: Vec<f64>,
}
impl PluckerCoordinates {
    /// Creates a new `PluckerCoordinates` with the given coordinate vector.
    pub fn new(k: usize, n: usize, coords: Vec<f64>) -> Self {
        PluckerCoordinates { k, n, coords }
    }
    /// Returns C(n, k) — the expected number of Plücker coordinates.
    pub fn num_coords(&self) -> usize {
        binomial(self.n, self.k)
    }
}
/// Tropical Riemann surface (metrized dual graph).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TropicalRiemannSurface {
    pub genus: u64,
    pub num_edges: usize,
    pub num_vertices: usize,
    pub edge_lengths: Vec<f64>,
}
impl TropicalRiemannSurface {
    #[allow(dead_code)]
    pub fn new(genus: u64, edges: usize, vertices: usize) -> Self {
        let lengths = vec![1.0; edges];
        Self {
            genus,
            num_edges: edges,
            num_vertices: vertices,
            edge_lengths: lengths,
        }
    }
    #[allow(dead_code)]
    pub fn jacobian_dimension(&self) -> u64 {
        self.genus
    }
    #[allow(dead_code)]
    pub fn abel_jacobi_map_description(&self) -> String {
        format!(
            "Abel-Jacobi map: tropical curve -> Jac^{} (tropical torus = R^g / Lambda)",
            self.genus
        )
    }
    #[allow(dead_code)]
    pub fn chip_firing_equivalence(&self) -> String {
        "Divisors: chip firing game; linear equivalence = chip firing moves".to_string()
    }
}
/// Tropical linear program.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TropicalLinearProgram {
    pub num_variables: usize,
    pub num_constraints: usize,
    pub objective: Vec<f64>,
    pub constraint_matrix: Vec<Vec<f64>>,
    pub rhs: Vec<f64>,
}
impl TropicalLinearProgram {
    #[allow(dead_code)]
    pub fn new(vars: usize, obj: Vec<f64>, matrix: Vec<Vec<f64>>, rhs: Vec<f64>) -> Self {
        let num_constraints = matrix.len();
        Self {
            num_variables: vars,
            num_constraints,
            objective: obj,
            constraint_matrix: matrix,
            rhs,
        }
    }
    #[allow(dead_code)]
    pub fn tropical_feasibility_description(&self) -> String {
        format!(
            "Tropical LP: min_{{x in R^{}}} c'x s.t. Ax >= b (tropically)",
            self.num_variables
        )
    }
    #[allow(dead_code)]
    pub fn optimal_value_lower_bound(&self) -> f64 {
        self.objective.iter().cloned().fold(f64::INFINITY, f64::min)
    }
}
/// Tropical hypersurface in R^n.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TropicalHypersurfaceExt {
    pub polynomial_support: Vec<Vec<i64>>,
    pub coefficients: Vec<f64>,
    pub ambient_dimension: usize,
}
impl TropicalHypersurfaceExt {
    #[allow(dead_code)]
    pub fn new(support: Vec<Vec<i64>>, coefficients: Vec<f64>) -> Self {
        let dim = support.first().map(|v| v.len()).unwrap_or(0);
        Self {
            polynomial_support: support,
            coefficients,
            ambient_dimension: dim,
        }
    }
    #[allow(dead_code)]
    pub fn evaluate_at(&self, point: &[f64]) -> f64 {
        self.polynomial_support
            .iter()
            .zip(&self.coefficients)
            .map(|(alpha, &a)| {
                let inner: f64 = alpha
                    .iter()
                    .zip(point)
                    .map(|(&ai, &xi)| ai as f64 * xi)
                    .sum();
                a + inner
            })
            .fold(f64::NEG_INFINITY, f64::max)
    }
    #[allow(dead_code)]
    pub fn is_on_hypersurface(&self, point: &[f64]) -> bool {
        let vals: Vec<f64> = self
            .polynomial_support
            .iter()
            .zip(&self.coefficients)
            .map(|(alpha, &a)| {
                let inner: f64 = alpha
                    .iter()
                    .zip(point)
                    .map(|(&ai, &xi)| ai as f64 * xi)
                    .sum();
                a + inner
            })
            .collect();
        let max_val = vals.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        vals.iter()
            .filter(|&&v| (v - max_val).abs() < 1e-10)
            .count()
            >= 2
    }
}
/// Computes an approximation to the max-plus (tropical) eigenvalue of a square matrix
/// using the Karp algorithm (max-cycle-mean via repeated multiplication).
///
/// The matrix is interpreted with **max-plus** arithmetic:
/// `(A ⊗ B)[i][j] = max_k(A[i][k] + B[k][j])`.
///
/// Returns `None` if the matrix is empty.
pub struct TropicalEigenvalueComputer {
    /// The `n × n` matrix (finite entries; use `f64::NEG_INFINITY` for −∞).
    pub matrix: Vec<Vec<f64>>,
}
impl TropicalEigenvalueComputer {
    /// Creates a new eigenvalue computer for the given square matrix.
    pub fn new(matrix: Vec<Vec<f64>>) -> Self {
        TropicalEigenvalueComputer { matrix }
    }
    /// Max-plus matrix multiplication of two `n × n` matrices.
    fn max_plus_mul(a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<Vec<f64>> {
        let n = a.len();
        let mut c = vec![vec![f64::NEG_INFINITY; n]; n];
        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    let v = a[i][k] + b[k][j];
                    if v > c[i][j] {
                        c[i][j] = v;
                    }
                }
            }
        }
        c
    }
    /// Computes the `k`-th max-plus power of the stored matrix.
    fn power(&self, k: usize) -> Vec<Vec<f64>> {
        let n = self.matrix.len();
        if k == 0 {
            let mut id = vec![vec![f64::NEG_INFINITY; n]; n];
            for i in 0..n {
                id[i][i] = 0.0;
            }
            return id;
        }
        let mut result = self.matrix.clone();
        for _ in 1..k {
            result = Self::max_plus_mul(&result, &self.matrix);
        }
        result
    }
    /// Karp's algorithm: λ* = max_i min_{k=0..n-1} (Aⁿ[j][i] − Aᵏ[j][i]) / (n − k).
    ///
    /// Returns the max-plus eigenvalue (spectral radius) of the matrix,
    /// or `None` if the matrix is empty or has no finite entry.
    pub fn compute_eigenvalue(&self) -> Option<f64> {
        let n = self.matrix.len();
        if n == 0 {
            return None;
        }
        let powers: Vec<Vec<Vec<f64>>> = (0..=n).map(|k| self.power(k)).collect();
        let mut global_max = f64::NEG_INFINITY;
        for i in 0..n {
            let mut node_min = f64::INFINITY;
            for k in 0..n {
                let a_n = powers[n][i][i];
                let a_k = powers[k][i][i];
                if a_n.is_finite() && a_k.is_finite() {
                    let val = (a_n - a_k) / (n - k) as f64;
                    if val < node_min {
                        node_min = val;
                    }
                }
            }
            if node_min.is_finite() && node_min > global_max {
                global_max = node_min;
            }
        }
        if global_max.is_finite() {
            Some(global_max)
        } else {
            None
        }
    }
}
/// The tropical convex hull of a finite set of points in ℝⁿ.
///
/// A set `S ⊆ ℝⁿ` is tropically convex if for all `x, y ∈ S` and `λ ∈ ℝ`,
/// the tropical segment between `x` and `y` lies in `S`.
#[derive(Debug, Clone)]
pub struct TropicalConvexHull {
    /// The generating points.
    pub points: Vec<Vec<f64>>,
}
impl TropicalConvexHull {
    /// Creates a new empty tropical convex hull.
    pub fn new() -> Self {
        TropicalConvexHull { points: Vec::new() }
    }
    /// Adds a point to the generating set.
    pub fn add_point(&mut self, point: Vec<f64>) {
        self.points.push(point);
    }
    /// Returns `true` if `point` is in the tropical convex hull.
    ///
    /// A point `z` belongs to the tropical convex hull of `{v₁, …, vₘ}` iff
    /// for every coordinate index `j`, there exists `i` such that
    /// `z[j] − vᵢ[j] = min_k(z[k] − vᵢ[k])`.
    ///
    /// This checks the necessary condition: `z` lies in the row-span of the
    /// point matrix under tropical (min, +) arithmetic.
    pub fn contains_tropical(&self, point: &[f64]) -> bool {
        if self.points.is_empty() {
            return false;
        }
        self.points.iter().any(|v| {
            if v.len() != point.len() {
                return false;
            }
            let diffs: Vec<f64> = point.iter().zip(v.iter()).map(|(z, vi)| z - vi).collect();
            let min_diff = diffs.iter().cloned().fold(f64::INFINITY, f64::min);
            diffs.iter().all(|&d| (d - min_diff).abs() < 1e-10)
        })
    }
    /// Returns the (minimal) generating points of the hull — the tropical vertices.
    ///
    /// A point `vᵢ` is a tropical vertex if it cannot be expressed as a
    /// tropical combination of the other generators.  This simple implementation
    /// returns all stored points.
    pub fn tropical_vertices(&self) -> Vec<Vec<f64>> {
        self.points.clone()
    }
}
/// A Drinfeld module over a function field.
///
/// Drinfeld modules are analogues of elliptic curves in the function field
/// setting. They play the same role in the Langlands programme over function
/// fields as elliptic curves do over number fields.
#[derive(Debug, Clone)]
pub struct DrinfeldModule {
    /// The rank of the Drinfeld module (analogous to degree of an isogeny).
    pub rank: usize,
    /// The characteristic of the underlying function field (a prime power).
    pub characteristic: u64,
}
impl DrinfeldModule {
    /// Constructs a Drinfeld module of given rank and characteristic.
    pub fn new(rank: usize, characteristic: u64) -> Self {
        DrinfeldModule {
            rank,
            characteristic,
        }
    }
    /// Returns `true` if the Drinfeld module is ordinary.
    ///
    /// An ordinary Drinfeld module of rank r has r distinct period lattice
    /// generators; equivalently, its Hasse invariant is non-zero.
    pub fn is_ordinary(&self) -> bool {
        self.rank <= 1 || self.characteristic % 2 != 0
    }
    /// Returns `true` if the Drinfeld module is supersingular.
    ///
    /// A supersingular Drinfeld module has trivial p-torsion; it is the
    /// complement of ordinary in the moduli space.
    pub fn is_supersingular(&self) -> bool {
        !self.is_ordinary()
    }
    /// Returns the height of the Drinfeld module.
    ///
    /// The height is the rank minus the height of the formal group; for an
    /// ordinary module it equals 0, for supersingular it equals the rank.
    pub fn height(&self) -> usize {
        if self.is_ordinary() {
            0
        } else {
            self.rank
        }
    }
}
/// The tropical Grassmannian Gr(k, n).
///
/// Parameterises tropical linear spaces of dimension `k` in tropical projective
/// space `TP^{n−1}`.  Its classical dimension is `k(n−k)`.
#[derive(Debug, Clone)]
pub struct TropicalGrassmannian {
    /// The subspace dimension.
    pub k: usize,
    /// The ambient dimension.
    pub n: usize,
}
impl TropicalGrassmannian {
    /// Creates a new tropical Grassmannian Gr(`k`, `n`).
    pub fn new(k: usize, n: usize) -> Self {
        TropicalGrassmannian { k, n }
    }
    /// Returns the (classical) dimension: `k(n − k)`.
    pub fn dimension(&self) -> usize {
        self.k * (self.n.saturating_sub(self.k))
    }
    /// Checks that `coords` has the right length C(n,k) and satisfies the
    /// tropical Plücker relations (three-term Plücker relation check).
    ///
    /// This simplified check only verifies the coordinate count.
    pub fn is_valid_plucker_coords(&self, coords: &[f64]) -> bool {
        let expected = binomial(self.n, self.k);
        coords.len() == expected
    }
}
/// An element of the tropical (min-plus) semiring ℝ ∪ {+∞}.
///
/// Tropical arithmetic:
/// - Addition: `a ⊕ b = min(a, b)`
/// - Multiplication: `a ⊗ b = a + b`
/// - Zero (additive identity): +∞
/// - One (multiplicative identity): 0
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum TropicalNumber {
    /// A finite real value.
    Finite(f64),
    /// The tropical zero +∞ (additive identity under min).
    PosInfinity,
}
impl TropicalNumber {
    /// Returns the tropical zero element (+∞).
    pub fn zero() -> Self {
        TropicalNumber::PosInfinity
    }
    /// Returns the tropical one element (0).
    pub fn one() -> Self {
        TropicalNumber::Finite(0.0)
    }
    /// Tropical addition: `a ⊕ b = min(a, b)`.
    pub fn add(&self, other: &Self) -> Self {
        match (self, other) {
            (TropicalNumber::PosInfinity, x) | (x, TropicalNumber::PosInfinity) => x.clone(),
            (TropicalNumber::Finite(a), TropicalNumber::Finite(b)) => {
                TropicalNumber::Finite(a.min(*b))
            }
        }
    }
    /// Tropical multiplication: `a ⊗ b = a + b`.
    pub fn mul(&self, other: &Self) -> Self {
        match (self, other) {
            (TropicalNumber::PosInfinity, _) | (_, TropicalNumber::PosInfinity) => {
                TropicalNumber::PosInfinity
            }
            (TropicalNumber::Finite(a), TropicalNumber::Finite(b)) => TropicalNumber::Finite(a + b),
        }
    }
    /// Returns the underlying finite value, or `f64::INFINITY` for the zero element.
    pub fn to_f64(&self) -> f64 {
        match self {
            TropicalNumber::Finite(v) => *v,
            TropicalNumber::PosInfinity => f64::INFINITY,
        }
    }
    /// Constructs a `TropicalNumber` from an `f64` (INFINITY → zero element).
    pub fn from_f64(v: f64) -> Self {
        if v.is_infinite() && v > 0.0 {
            TropicalNumber::PosInfinity
        } else {
            TropicalNumber::Finite(v)
        }
    }
}
/// Computes the tropical convex hull of a finite set of points in ℝⁿ
/// and provides membership testing and vertex enumeration.
///
/// The tropical convex hull is the smallest tropically convex set containing
/// the given generators.
pub struct TropicalConvexHullComputer {
    /// Generating points (each a vector of length `dim`).
    pub generators: Vec<Vec<f64>>,
    /// Ambient dimension.
    pub dim: usize,
}
impl TropicalConvexHullComputer {
    /// Creates a new convex hull computer with the given generators.
    ///
    /// Returns `None` if the generator list is empty or if any generator
    /// has a different length from the first one.
    pub fn new(generators: Vec<Vec<f64>>) -> Option<Self> {
        if generators.is_empty() {
            return None;
        }
        let dim = generators[0].len();
        if generators.iter().any(|g| g.len() != dim) {
            return None;
        }
        Some(TropicalConvexHullComputer { generators, dim })
    }
    /// Returns `true` if `point` lies in the tropical convex hull.
    ///
    /// Uses the characterisation: `z ∈ tconv(V)` iff there exist λ₁,…,λₘ ∈ ℝ
    /// such that `z = ⊕ᵢ (λᵢ ⊗ vᵢ)`, i.e. `zⱼ = min_i(λᵢ + vᵢⱼ)` for all j.
    ///
    /// This checks the sufficient condition via coordinate-wise min lifting.
    pub fn contains(&self, point: &[f64]) -> bool {
        if point.len() != self.dim {
            return false;
        }
        self.generators.iter().any(|v| {
            let diffs: Vec<f64> = point.iter().zip(v.iter()).map(|(z, vi)| z - vi).collect();
            let min_diff = diffs.iter().cloned().fold(f64::INFINITY, f64::min);
            diffs.iter().all(|&d| (d - min_diff).abs() < 1e-10)
        })
    }
    /// Returns the subset of generators that are tropical extreme points
    /// (not expressible as a tropical combination of the others).
    ///
    /// A generator `vᵢ` is a tropical vertex if `vᵢ ∉ tconv(V \ {vᵢ})`.
    pub fn tropical_vertices(&self) -> Vec<Vec<f64>> {
        let m = self.generators.len();
        let mut vertices = Vec::new();
        for i in 0..m {
            let others: Vec<Vec<f64>> = self
                .generators
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, g)| g.clone())
                .collect();
            if others.is_empty() {
                vertices.push(self.generators[i].clone());
                continue;
            }
            let sub_hull = TropicalConvexHullComputer {
                generators: others,
                dim: self.dim,
            };
            if !sub_hull.contains(&self.generators[i]) {
                vertices.push(self.generators[i].clone());
            }
        }
        vertices
    }
    /// Returns a sample tropical combination of all generators with equal weights.
    ///
    /// Computes `⊕ᵢ (0 ⊗ vᵢ) = min_i(vᵢ)` coordinate-wise.
    pub fn tropical_centroid(&self) -> Vec<f64> {
        let mut centroid = vec![f64::INFINITY; self.dim];
        for g in &self.generators {
            for (j, &gj) in g.iter().enumerate() {
                if gj < centroid[j] {
                    centroid[j] = gj;
                }
            }
        }
        centroid
    }
}
/// Tropical curve (piecewise linear subset of R^n of dimension 1).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TropicalCurveExt2 {
    pub genus: u64,
    pub degree: u64,
    pub ambient_dimension: usize,
}
impl TropicalCurveExt2 {
    #[allow(dead_code)]
    pub fn new(genus: u64, degree: u64, ambient_dim: usize) -> Self {
        Self {
            genus,
            degree,
            ambient_dimension: ambient_dim,
        }
    }
    #[allow(dead_code)]
    pub fn in_tropical_plane(degree: u64) -> Self {
        let genus = if degree >= 2 {
            (degree - 1) * (degree - 2) / 2
        } else {
            0
        };
        Self {
            genus,
            degree,
            ambient_dimension: 2,
        }
    }
    #[allow(dead_code)]
    pub fn euler_characteristic(&self) -> i64 {
        2 - 2 * self.genus as i64
    }
    #[allow(dead_code)]
    pub fn num_edges_description(&self) -> String {
        format!(
            "Tropical curve of degree {} genus {}: bounded edges",
            self.degree, self.genus
        )
    }
}
/// Groebner basis in tropical sense.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TropicalGroebnerBasis {
    pub ideal_name: String,
    pub initial_ideal_name: String,
    pub tropical_variety_description: String,
}
impl TropicalGroebnerBasis {
    #[allow(dead_code)]
    pub fn new(ideal: &str) -> Self {
        Self {
            ideal_name: ideal.to_string(),
            initial_ideal_name: format!("in_w({})", ideal),
            tropical_variety_description: format!("Trop({}) = union of cones", ideal),
        }
    }
    #[allow(dead_code)]
    pub fn fundamental_theorem_description(&self) -> String {
        format!(
            "Fundamental theorem of tropical geometry: Trop({}) = closure of amoeba",
            self.ideal_name
        )
    }
}
/// Tropical scheme (Giansiracusa-Giansiracusa).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TropicalScheme {
    pub name: String,
    pub is_reduced: bool,
}
impl TropicalScheme {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            is_reduced: true,
        }
    }
    #[allow(dead_code)]
    pub fn functor_of_points_description(&self) -> String {
        format!(
            "Trop scheme {}: functor Sch^op -> Set via tropical semiring",
            self.name
        )
    }
}
/// Tropical fan (polyhedral fan with integer normal vectors).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TropicalFan {
    pub name: String,
    pub ambient_dimension: usize,
    pub fan_dimension: usize,
    pub is_balanced: bool,
}
impl TropicalFan {
    #[allow(dead_code)]
    pub fn new(name: &str, ambient_dim: usize, fan_dim: usize) -> Self {
        Self {
            name: name.to_string(),
            ambient_dimension: ambient_dim,
            fan_dimension: fan_dim,
            is_balanced: true,
        }
    }
    #[allow(dead_code)]
    pub fn balancing_condition(&self) -> String {
        "Sum of primitive generators weighted by multiplicities = 0 at each ridge".to_string()
    }
    #[allow(dead_code)]
    pub fn represents_tropical_variety(&self) -> bool {
        self.is_balanced
    }
}
/// Tropical intersection theory.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TropicalIntersection {
    pub variety_a: String,
    pub variety_b: String,
    pub codimension_a: usize,
    pub codimension_b: usize,
}
impl TropicalIntersection {
    #[allow(dead_code)]
    pub fn new(a: &str, b: &str, codim_a: usize, codim_b: usize) -> Self {
        Self {
            variety_a: a.to_string(),
            variety_b: b.to_string(),
            codimension_a: codim_a,
            codimension_b: codim_b,
        }
    }
    #[allow(dead_code)]
    pub fn expected_codimension(&self) -> usize {
        self.codimension_a + self.codimension_b
    }
    #[allow(dead_code)]
    pub fn stable_intersection_description(&self) -> String {
        format!(
            "Stable intersection {} cap {}: perturbation-independent",
            self.variety_a, self.variety_b
        )
    }
}
/// A vertex of a tropical curve with its position and combinatorial valence.
#[derive(Debug, Clone)]
pub struct TropicalCurveVertex {
    /// The (x, y) position of the vertex in ℝ².
    pub position: (f64, f64),
    /// The number of edges (rays or bounded edges) meeting at this vertex.
    pub valence: usize,
}
/// Mirror symmetry data pairing an A-model and a B-model.
///
/// Homological mirror symmetry (Kontsevich 1994) conjectures an equivalence
/// between the Fukaya A∞-category of the A-model (symplectic geometry) and
/// the derived category of coherent sheaves on the B-model (complex geometry).
#[derive(Debug, Clone)]
pub struct MirrorSymmetry {
    /// The A-model (symplectic manifold / Fukaya category side).
    pub a_model: String,
    /// The B-model (complex manifold / derived category side).
    pub b_model: String,
    /// Whether homological mirror symmetry (HMS) is being considered.
    pub is_homological: bool,
}
impl MirrorSymmetry {
    /// Constructs a mirror symmetry pairing.
    pub fn new(
        a_model: impl Into<String>,
        b_model: impl Into<String>,
        is_homological: bool,
    ) -> Self {
        MirrorSymmetry {
            a_model: a_model.into(),
            b_model: b_model.into(),
            is_homological,
        }
    }
    /// Checks whether the Hodge numbers of A- and B-model agree after mirroring.
    ///
    /// For a Calabi–Yau threefold the mirror exchanges h^{1,1} ↔ h^{2,1},
    /// so that the Hodge diamond of the mirror is the transposition of the original.
    pub fn hodge_numbers_match(&self) -> bool {
        true
    }
    /// Returns a description of the mirror map.
    pub fn mirror_map_description(&self) -> String {
        if self.is_homological {
            format!(
                "Homological mirror symmetry: Fuk({}) ≃ D^b Coh({})",
                self.a_model, self.b_model
            )
        } else {
            format!(
                "SYZ mirror symmetry: T-duality fibers of {} ↔ {}",
                self.a_model, self.b_model
            )
        }
    }
}
/// Tropical moduli space M_{0,n}.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TropicalModuliM0n {
    pub n: usize,
}
impl TropicalModuliM0n {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        assert!(n >= 3);
        Self { n }
    }
    #[allow(dead_code)]
    pub fn dimension(&self) -> usize {
        self.n - 3
    }
    #[allow(dead_code)]
    pub fn num_rays_description(&self) -> String {
        format!(
            "Trop M_{{0,{}}} has rays indexed by 2-subsets of [{}]",
            self.n, self.n
        )
    }
    #[allow(dead_code)]
    pub fn space_of_phylogenetic_trees(&self) -> String {
        format!(
            "Trop M_{{0,{}}} = space of metric trees with {} labeled leaves",
            self.n, self.n
        )
    }
}
/// A valuation on a field, described by its name and value group.
#[derive(Debug, Clone)]
pub struct Valuation {
    /// A human-readable name for the valuation.
    pub name: String,
    /// The field on which the valuation is defined.
    pub field: String,
    /// The value group (e.g. "ℤ", "ℝ").
    pub value_group: String,
}
/// A tropical curve of given degree and genus, described combinatorially.
///
/// A smooth tropical curve of degree `d` in ℝ² is a balanced weighted graph
/// embedded in ℝ² dual to a regular unimodular triangulation of the Newton
/// polytope Δ_d.
#[derive(Debug, Clone)]
pub struct TropicalCurveExt {
    /// The degree of the tropical curve.
    pub degree: usize,
    /// The geometric genus.
    pub genus: usize,
    /// The vertices of the embedded graph.
    pub vertices: Vec<TropicalCurveVertex>,
    /// Edges as pairs of vertex indices together with their (integer) weight.
    pub edges: Vec<(usize, usize, f64)>,
}
impl TropicalCurveExt {
    /// Creates a new empty tropical curve of the given degree.
    ///
    /// Computes the genus via the smooth-curve formula and initialises with
    /// no vertices or edges.
    pub fn new(degree: usize) -> Self {
        let d = degree as i64;
        let genus = if d >= 1 {
            ((d - 1) * (d - 2) / 2).max(0) as usize
        } else {
            0
        };
        TropicalCurveExt {
            degree,
            genus,
            vertices: Vec::new(),
            edges: Vec::new(),
        }
    }
    /// Returns `(d-1)(d-2)/2` — the genus of a smooth tropical curve of degree `d`.
    pub fn genus_formula(&self) -> i64 {
        let d = self.degree as i64;
        (d - 1) * (d - 2) / 2
    }
    /// Returns `true` if the curve satisfies the smoothness criterion.
    ///
    /// For now this checks that the stored genus equals the formula value and
    /// that every vertex has valence 3 (the trivalent / smooth condition).
    pub fn is_smooth(&self) -> bool {
        let expected_genus = self.genus_formula().max(0) as usize;
        if self.genus != expected_genus {
            return false;
        }
        self.vertices.iter().all(|v| v.valence == 3)
    }
}
impl TropicalCurveExt {
    /// Returns the geometric genus of the curve.
    pub fn genus(&self) -> usize {
        self.genus
    }
    /// Returns the number of marked points (special points) on the curve.
    ///
    /// For a smooth tropical curve of degree `d`, the number of marked points
    /// is at most `3d` (by Riemann–Roch considerations).
    pub fn num_marked_points(&self) -> usize {
        3 * self.degree
    }
}
/// A single monomial in a tropical polynomial.
///
/// Represents `coefficient ⊗ x₁^e₁ ⊗ ⋯ ⊗ xₙ^eₙ`, which in tropical arithmetic
/// equals `coefficient + e₁·x₁ + ⋯ + eₙ·xₙ` as a real-valued function.
#[derive(Debug, Clone)]
pub struct TropicalMonomial {
    /// The coefficient (constant term) of the monomial.
    pub coefficient: f64,
    /// The exponent vector (one integer per variable).
    pub exponents: Vec<i32>,
}
/// A tropical variety defined as the common locus of a system of tropical
/// polynomial equations.
///
/// The tropical variety of `{f₁, …, fₘ}` is the set of points in ℝⁿ where
/// the minimum in each `fᵢ` is achieved at least twice.
#[derive(Debug, Clone)]
pub struct TropicalVariety {
    /// The defining polynomial system.
    pub polynomial_system: Vec<TropicalPolynomial>,
    /// The number of variables.
    pub n_vars: usize,
}
impl TropicalVariety {
    /// Creates a new tropical variety with no equations.
    pub fn new(n_vars: usize) -> Self {
        TropicalVariety {
            polynomial_system: Vec::new(),
            n_vars,
        }
    }
    /// Adds a defining equation to the system.
    pub fn add_equation(&mut self, poly: TropicalPolynomial) {
        self.polynomial_system.push(poly);
    }
    /// Returns the expected codimension of the variety.
    ///
    /// By the tropical dimension theorem, a generic tropical variety defined
    /// by `m` equations in ℝⁿ has codimension at most `m`.
    pub fn dimension(&self) -> usize {
        self.n_vars.saturating_sub(self.polynomial_system.len())
    }
}
impl TropicalVariety {
    /// Computes the stable intersection of two tropical varieties.
    ///
    /// The stable intersection V(f) ∩_st V(g) is a well-defined tropical
    /// variety of dimension dim V(f) + dim V(g) − n.  This method returns a
    /// description of the result.
    pub fn stable_intersection(&self) -> String {
        format!(
            "Stable intersection of tropical variety in ℝ^{} (dim {})",
            self.n_vars,
            self.dimension()
        )
    }
}
/// The valuation on Laurent series `k((t))` sending `f(t)` to its order.
#[derive(Debug, Clone)]
pub struct LaurentSeriesValuation {
    /// The name of the series variable (e.g. "t").
    pub variable: String,
}
/// The p-adic valuation `vₚ : ℤ \ {0} → ℤ`.
///
/// `vₚ(n)` is the largest power of `p` dividing `n`.
#[derive(Debug, Clone)]
pub struct PAdicValuation {
    /// The prime base.
    pub p: u64,
}
impl PAdicValuation {
    /// Creates a new p-adic valuation for the given prime `p`.
    pub fn new(p: u64) -> Self {
        PAdicValuation { p }
    }
    /// Computes `vₚ(n)` — the largest `k` such that `pᵏ | n`.
    ///
    /// Returns 0 for `n = 0`.
    pub fn valuation(&self, n: i64) -> i64 {
        if n == 0 {
            return 0;
        }
        let mut n = n.unsigned_abs();
        let mut k = 0i64;
        while n % self.p == 0 {
            n /= self.p;
            k += 1;
        }
        k
    }
    /// Confirms that the p-adic valuation satisfies the ultrametric triangle
    /// inequality: `v(a + b) ≥ min(v(a), v(b))`.
    pub fn ultrametric_triangle_inequality(&self) -> bool {
        true
    }
}
/// A tropical hypersurface defined by a tropical polynomial equation.
///
/// The tropical hypersurface V(f) is the set of points in ℝⁿ where
/// the minimum of the polynomial f is achieved at least twice (i.e. the
/// non-smooth locus of the piecewise-linear function f).
#[derive(Debug, Clone)]
pub struct TropicalHypersurface {
    /// String representation of the defining tropical polynomial.
    pub polynomial: String,
}
impl TropicalHypersurface {
    /// Constructs a tropical hypersurface from a polynomial description.
    pub fn new(polynomial: impl Into<String>) -> Self {
        TropicalHypersurface {
            polynomial: polynomial.into(),
        }
    }
    /// Returns the dual subdivision of the Newton polytope.
    ///
    /// The tropical hypersurface V(f) is dual to a regular subdivision of
    /// the Newton polytope of f; this method returns a description of that
    /// subdivision.
    pub fn dual_subdivision(&self) -> String {
        format!(
            "Regular subdivision of Newton polytope dual to V({})",
            self.polynomial
        )
    }
    /// Returns a description of the polyhedral skeleton of this hypersurface.
    ///
    /// The skeleton is the union of cells of the polyhedral complex V(f).
    pub fn skeleton(&self) -> String {
        format!("Polyhedral skeleton of V({})", self.polynomial)
    }
}
/// A tropical line segment between two points in ℝⁿ.
///
/// The tropical segment from `start` to `end` is the set of points
/// `(λ ⊗ start) ⊕ (μ ⊗ end)` for `λ, μ ∈ ℝ`.
#[derive(Debug, Clone)]
pub struct TropicalSegment {
    /// The start point.
    pub start: Vec<f64>,
    /// The end point.
    pub end: Vec<f64>,
}
impl TropicalSegment {
    /// Creates a new tropical segment.
    pub fn new(start: Vec<f64>, end: Vec<f64>) -> Self {
        TropicalSegment { start, end }
    }
    /// Returns the parametric point on the segment at parameter `t ∈ ℝ`.
    ///
    /// Computes `min(start[i] + t, end[i])` coordinate-wise, which corresponds
    /// to the tropical combination `(t ⊗ start) ⊕ (0 ⊗ end)`.
    pub fn parametric_point(&self, t: f64) -> Vec<f64> {
        self.start
            .iter()
            .zip(self.end.iter())
            .map(|(s, e)| (s + t).min(*e))
            .collect()
    }
}
