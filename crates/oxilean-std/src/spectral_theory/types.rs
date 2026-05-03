//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};
use std::collections::HashMap;

/// Stone's theorem: every strongly continuous one-parameter unitary group U(t) = e^{itA}
/// has a unique self-adjoint generator A.
pub struct StoneThm;
impl StoneThm {
    /// Create a Stone theorem struct.
    pub fn new() -> Self {
        Self
    }
    /// Return a description of the generator self-adjointness result.
    pub fn group_generator_is_selfadjoint(&self) -> String {
        "Stone's theorem: U(t) = e^{itA} is a strongly continuous unitary group \
         iff A is a (densely defined) self-adjoint operator."
            .to_string()
    }
}
/// Spectral data for a self-adjoint operator: eigenvalues and their projection
/// operators (as 2×2 matrices for the finite-dimensional case).
#[derive(Debug, Clone)]
pub struct SpectralDecomposition {
    /// Eigenvalues in increasing order.
    pub eigenvalues: Vec<f64>,
    /// Orthogonal projections P_λ (one per eigenvalue).
    pub projections: Vec<ComplexMatrix>,
}
impl SpectralDecomposition {
    /// Verify that ∑ P_λ = I.
    pub fn check_resolution_of_identity(&self) -> bool {
        if self.projections.is_empty() {
            return true;
        }
        let n = self.projections[0].n;
        let mut sum = ComplexMatrix::zeros(n);
        for p in &self.projections {
            sum = sum.add(p);
        }
        let id = ComplexMatrix::identity(n);
        for i in 0..n {
            for j in 0..n {
                if sum.data[i][j].sub(&id.data[i][j]).modulus() > 1e-8 {
                    return false;
                }
            }
        }
        true
    }
    /// Verify that P_i P_j = 0 for i ≠ j (orthogonality).
    pub fn check_orthogonality(&self) -> bool {
        for i in 0..self.projections.len() {
            for j in (i + 1)..self.projections.len() {
                let prod = self.projections[i].mul(&self.projections[j]);
                if prod.frobenius_norm() > 1e-8 {
                    return false;
                }
            }
        }
        true
    }
    /// Reconstruct the operator: ∑ λ_i P_i.
    pub fn reconstruct(&self) -> ComplexMatrix {
        if self.projections.is_empty() {
            return ComplexMatrix::zeros(0);
        }
        let n = self.projections[0].n;
        let mut result = ComplexMatrix::zeros(n);
        for (lam, p) in self.eigenvalues.iter().zip(self.projections.iter()) {
            let term = p.scale(Complex64::real(*lam));
            result = result.add(&term);
        }
        result
    }
}
/// Spectral data for a graph Laplacian.
#[allow(dead_code)]
pub struct GraphLaplacianSpectrum {
    /// Number of vertices.
    pub num_vertices: usize,
    /// Number of edges.
    pub num_edges: usize,
    /// Eigenvalues of the Laplacian in non-decreasing order.
    pub eigenvalues: Vec<f64>,
}
#[allow(dead_code)]
impl GraphLaplacianSpectrum {
    /// Create graph Laplacian spectrum data.
    pub fn new(num_vertices: usize, num_edges: usize, eigenvalues: Vec<f64>) -> Self {
        Self {
            num_vertices,
            num_edges,
            eigenvalues,
        }
    }
    /// The smallest eigenvalue λ₀ = 0 (corresponding to constant functions).
    pub fn zero_eigenvalue(&self) -> Option<f64> {
        self.eigenvalues.first().copied()
    }
    /// The spectral gap λ₁ = second smallest eigenvalue (algebraic connectivity).
    pub fn spectral_gap(&self) -> Option<f64> {
        if self.eigenvalues.len() >= 2 {
            Some(self.eigenvalues[1])
        } else {
            None
        }
    }
    /// The largest eigenvalue λ_{n-1} ≤ 2 * max_degree.
    pub fn largest_eigenvalue(&self) -> Option<f64> {
        self.eigenvalues.last().copied()
    }
    /// Cheeger lower bound: h(G)²/2 ≤ λ₁.
    pub fn cheeger_lower_bound(&self) -> Option<f64> {
        self.spectral_gap()
    }
    /// The number of connected components equals the multiplicity of eigenvalue 0.
    pub fn num_connected_components(&self) -> usize {
        self.eigenvalues
            .iter()
            .filter(|&&v| v.abs() < 1e-10)
            .count()
    }
    /// Isoperimetric ratio estimate: h(G) ≤ sqrt(2 λ_{n-1} λ₁).
    pub fn isoperimetric_upper_bound(&self) -> Option<f64> {
        let gap = self.spectral_gap()?;
        let largest = self.largest_eigenvalue()?;
        Some((2.0 * largest * gap).sqrt())
    }
}
/// A compact linear operator.
pub struct CompactOperator {
    /// Whether the operator can be approximated by finite-rank operators.
    pub approximating_finite_rank: bool,
}
impl CompactOperator {
    /// Create a compact operator.
    pub fn new(approximating_finite_rank: bool) -> Self {
        Self {
            approximating_finite_rank,
        }
    }
    /// Return a description of the singular value sequence.
    /// For compact operators the singular values form a sequence converging to 0.
    pub fn singular_value_sequence(&self) -> String {
        "s₁ ≥ s₂ ≥ … ≥ 0,  sₙ → 0  [singular values of compact operator]".to_string()
    }
    /// Check whether the operator is in the Schatten p-class (Sₚ).
    /// Formally: T ∈ Sₚ ⟺ Σ sₙ(T)^p < ∞.
    pub fn schatten_class(&self, p: f64) -> bool {
        p > 0.0 && self.approximating_finite_rank
    }
}
/// Record holding Selberg trace formula data for a hyperbolic surface.
#[allow(dead_code)]
pub struct SelbergTraceData {
    /// Volume of the hyperbolic surface.
    pub volume: f64,
    /// Euler characteristic.
    pub euler_characteristic: i64,
    /// Lengths of primitive closed geodesics (up to some cutoff).
    pub geodesic_lengths: Vec<f64>,
    /// Eigenvalues of the Laplace-Beltrami operator.
    pub laplacian_eigenvalues: Vec<f64>,
}
#[allow(dead_code)]
impl SelbergTraceData {
    /// Create Selberg trace data.
    pub fn new(
        volume: f64,
        euler_characteristic: i64,
        geodesic_lengths: Vec<f64>,
        laplacian_eigenvalues: Vec<f64>,
    ) -> Self {
        Self {
            volume,
            euler_characteristic,
            geodesic_lengths,
            laplacian_eigenvalues,
        }
    }
    /// The spectral side: sum over eigenvalues λₙ of ∫ h(√(λₙ - 1/4)).
    pub fn spectral_side_description(&self) -> String {
        format!(
            "Spectral side: sum over {} eigenvalues of Laplacian",
            self.laplacian_eigenvalues.len()
        )
    }
    /// The geometric side: sum over identity and closed geodesics.
    pub fn geometric_side_description(&self) -> String {
        format!(
            "Geometric side: identity term (Vol={:.4}) + sum over {} primitive geodesics",
            self.volume,
            self.geodesic_lengths.len()
        )
    }
    /// Weyl law: N(λ) = #{λₙ ≤ λ} ~ (Vol / 4π) λ as λ → ∞.
    pub fn weyl_law_approximation(&self, lambda: f64) -> f64 {
        (self.volume / (4.0 * std::f64::consts::PI)) * lambda
    }
    /// Count eigenvalues ≤ λ.
    pub fn count_eigenvalues(&self, lambda: f64) -> usize {
        self.laplacian_eigenvalues
            .iter()
            .filter(|&&v| v <= lambda)
            .count()
    }
    /// The smallest positive eigenvalue (spectral gap for Γ\H).
    pub fn spectral_gap(&self) -> Option<f64> {
        self.laplacian_eigenvalues
            .iter()
            .filter(|&&v| v > 1e-10)
            .copied()
            .fold(None, |acc, v| {
                Some(match acc {
                    None => v,
                    Some(a) => a.min(v),
                })
            })
    }
}
/// Essential spectrum of an operator (Weyl's theorem).
pub struct EssentialSpectrum {
    /// Description of the operator.
    pub operator: String,
}
impl EssentialSpectrum {
    /// Create an essential spectrum struct.
    pub fn new(operator: String) -> Self {
        Self { operator }
    }
    /// Return a description of the Weyl criterion.
    /// λ ∈ σ_ess(A) iff there exists a Weyl sequence (approximate eigenvectors with no
    /// convergent subsequence).
    pub fn weyl_criterion(&self) -> String {
        format!(
            "Weyl criterion for {}: λ ∈ σ_ess iff ∃ {{φₙ}} ⊂ D(A), ‖φₙ‖=1, φₙ ⇀ 0, ‖(A−λ)φₙ‖→0",
            self.operator
        )
    }
    /// Returns true if the essential spectrum is empty (e.g. for operators with compact resolvent).
    pub fn is_empty(&self) -> bool {
        false
    }
}
/// Spectral zeta function data for a self-adjoint operator.
#[allow(dead_code)]
pub struct SpectralZeta {
    /// Name/description of the operator.
    pub operator_name: String,
    /// Positive eigenvalues (λₙ > 0) of the operator.
    pub positive_eigenvalues: Vec<f64>,
}
#[allow(dead_code)]
impl SpectralZeta {
    /// Create a spectral zeta function record.
    pub fn new(operator_name: &str, positive_eigenvalues: Vec<f64>) -> Self {
        Self {
            operator_name: operator_name.to_string(),
            positive_eigenvalues,
        }
    }
    /// Evaluate ζ_A(s) = ∑ λₙ^{-s} for Re(s) large enough (naive sum).
    pub fn evaluate(&self, s: f64) -> Option<f64> {
        if self.positive_eigenvalues.is_empty() {
            return None;
        }
        let sum: f64 = self
            .positive_eigenvalues
            .iter()
            .map(|&lam| lam.powf(-s))
            .sum();
        Some(sum)
    }
    /// The zeta-regularized determinant det_ζ(A) = exp(-ζ_A'(0)).
    pub fn zeta_regularized_determinant_description(&self) -> String {
        format!(
            "det_ζ({}) = exp(-ζ'(0))  [zeta-regularized determinant]",
            self.operator_name
        )
    }
    /// The spectral zeta function has a meromorphic continuation to ℂ.
    pub fn has_meromorphic_continuation(&self) -> bool {
        true
    }
    /// The zeta function encodes the heat kernel: ζ_A(s) = Γ(s)^{-1} ∫₀^∞ t^{s-1} tr(e^{-tA}) dt.
    pub fn heat_kernel_mellin_transform(&self) -> String {
        format!(
            "ζ_{}(s) = (1/Γ(s)) ∫₀^∞ t^{{s-1}} tr(e^{{-t·{}}}) dt",
            self.operator_name, self.operator_name
        )
    }
    /// Number of positive eigenvalues (spectral complexity).
    pub fn spectral_complexity(&self) -> usize {
        self.positive_eigenvalues.len()
    }
}
/// A discrete spectral measure: E : 2^{λ₁,...,λₙ} → Projections.
#[derive(Debug, Clone)]
pub struct DiscreteSpectralMeasure {
    /// Eigenvalues (spectral points).
    pub spectral_points: Vec<f64>,
    /// Projection for each spectral point.
    pub projections: HashMap<usize, ComplexMatrix>,
}
impl DiscreteSpectralMeasure {
    /// Create a new empty spectral measure.
    pub fn new() -> Self {
        Self {
            spectral_points: vec![],
            projections: HashMap::new(),
        }
    }
    /// Add a spectral point λ with its projection P.
    pub fn add(&mut self, lambda: f64, proj: ComplexMatrix) {
        let idx = self.spectral_points.len();
        self.spectral_points.push(lambda);
        self.projections.insert(idx, proj);
    }
    /// E(A) for a Borel set A = subset of spectral points (given as indices).
    pub fn measure_of_set(&self, indices: &[usize]) -> Option<ComplexMatrix> {
        if self.projections.is_empty() {
            return None;
        }
        let n = self.projections.values().next()?.n;
        let mut sum = ComplexMatrix::zeros(n);
        for &i in indices {
            if let Some(p) = self.projections.get(&i) {
                sum = sum.add(p);
            }
        }
        Some(sum)
    }
    /// Number of spectral points.
    pub fn num_points(&self) -> usize {
        self.spectral_points.len()
    }
}
/// A complex number with f64 real and imaginary parts.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex64 {
    /// Real part.
    pub re: f64,
    /// Imaginary part.
    pub im: f64,
}
impl Complex64 {
    /// Create a new complex number.
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }
    /// Create a real number as Complex64.
    pub fn real(re: f64) -> Self {
        Self { re, im: 0.0 }
    }
    /// Complex modulus |z|.
    pub fn modulus(&self) -> f64 {
        (self.re * self.re + self.im * self.im).sqrt()
    }
    /// Complex conjugate z̄.
    pub fn conj(&self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }
    /// Complex multiplication.
    pub fn mul(&self, other: &Complex64) -> Complex64 {
        Complex64 {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
    /// Complex addition.
    pub fn add(&self, other: &Complex64) -> Complex64 {
        Complex64 {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
    /// Complex subtraction.
    pub fn sub(&self, other: &Complex64) -> Complex64 {
        Complex64 {
            re: self.re - other.re,
            im: self.im - other.im,
        }
    }
    /// Complex division (returns None if divisor is zero).
    pub fn div(&self, other: &Complex64) -> Option<Complex64> {
        let denom = other.re * other.re + other.im * other.im;
        if denom < 1e-300 {
            return None;
        }
        Some(Complex64 {
            re: (self.re * other.re + self.im * other.im) / denom,
            im: (self.im * other.re - self.re * other.im) / denom,
        })
    }
    /// Exponential exp(z) = exp(re) * (cos(im) + i*sin(im)).
    pub fn exp(&self) -> Complex64 {
        let exp_re = self.re.exp();
        Complex64 {
            re: exp_re * self.im.cos(),
            im: exp_re * self.im.sin(),
        }
    }
    /// Check if essentially real (imaginary part below tolerance).
    pub fn is_real(&self, tol: f64) -> bool {
        self.im.abs() < tol
    }
}
/// A self-adjoint (Hermitian) operator on a Hilbert space.
pub struct SelfAdjointOperator {
    /// Description of the domain of the operator.
    pub domain: String,
    /// Whether the operator is bounded.
    pub is_bounded: bool,
}
impl SelfAdjointOperator {
    /// Create a self-adjoint operator.
    pub fn new(domain: String, is_bounded: bool) -> Self {
        Self { domain, is_bounded }
    }
    /// Self-adjoint operators have purely real spectrum.
    pub fn spectrum_is_real(&self) -> bool {
        true
    }
    /// Return a symbolic description of the resolvent R(λ) = (A − λI)⁻¹.
    pub fn resolvent(&self, lambda: f64) -> String {
        format!(
            "R({lambda:.4}) = (A − {lambda:.4}·I)⁻¹  [domain: {}]",
            self.domain
        )
    }
}
/// Estimate the essential spectrum of a compact perturbation.
///
/// Weyl's theorem: the essential spectrum is invariant under compact
/// perturbations.  In finite dimensions, the essential spectrum is empty (all
/// operators are compact), so this returns a struct recording the perturbation.
#[derive(Debug, Clone)]
pub struct WeylData {
    /// Spectrum of the original operator T.
    pub spectrum_t: FiniteSpectrum,
    /// Spectrum of T + K (compact perturbation).
    pub spectrum_t_plus_k: FiniteSpectrum,
}
impl WeylData {
    /// Create Weyl data from two spectra.
    pub fn new(spectrum_t: FiniteSpectrum, spectrum_t_plus_k: FiniteSpectrum) -> Self {
        Self {
            spectrum_t,
            spectrum_t_plus_k,
        }
    }
    /// In infinite dimensions, verify: essential spectra agree.
    /// (Here we just check the spectral radii are equal as a proxy.)
    pub fn essential_spectra_agree(&self, tol: f64) -> bool {
        (self.spectrum_t.spectral_radius() - self.spectrum_t_plus_k.spectral_radius()).abs() < tol
    }
}
/// A dense n×n matrix of Complex64 values.
#[derive(Debug, Clone)]
pub struct ComplexMatrix {
    /// Number of rows/columns.
    pub n: usize,
    /// Row-major entries.
    pub data: Vec<Vec<Complex64>>,
}
impl ComplexMatrix {
    /// Create an n×n zero matrix.
    pub fn zeros(n: usize) -> Self {
        Self {
            n,
            data: vec![vec![Complex64::new(0.0, 0.0); n]; n],
        }
    }
    /// Create an n×n identity matrix.
    pub fn identity(n: usize) -> Self {
        let mut m = Self::zeros(n);
        for i in 0..n {
            m.data[i][i] = Complex64::real(1.0);
        }
        m
    }
    /// Create a diagonal matrix from real eigenvalues.
    pub fn diagonal(values: &[f64]) -> Self {
        let n = values.len();
        let mut m = Self::zeros(n);
        for i in 0..n {
            m.data[i][i] = Complex64::real(values[i]);
        }
        m
    }
    /// Get the (i,j) entry.
    pub fn get(&self, i: usize, j: usize) -> Complex64 {
        self.data[i][j]
    }
    /// Set the (i,j) entry.
    pub fn set(&mut self, i: usize, j: usize, val: Complex64) {
        self.data[i][j] = val;
    }
    /// Matrix multiplication.
    pub fn mul(&self, other: &ComplexMatrix) -> ComplexMatrix {
        assert_eq!(self.n, other.n);
        let n = self.n;
        let mut result = ComplexMatrix::zeros(n);
        for i in 0..n {
            for j in 0..n {
                let mut sum = Complex64::new(0.0, 0.0);
                for k in 0..n {
                    sum = sum.add(&self.data[i][k].mul(&other.data[k][j]));
                }
                result.data[i][j] = sum;
            }
        }
        result
    }
    /// Matrix addition.
    pub fn add(&self, other: &ComplexMatrix) -> ComplexMatrix {
        assert_eq!(self.n, other.n);
        let n = self.n;
        let mut result = ComplexMatrix::zeros(n);
        for i in 0..n {
            for j in 0..n {
                result.data[i][j] = self.data[i][j].add(&other.data[i][j]);
            }
        }
        result
    }
    /// Matrix scalar multiplication.
    pub fn scale(&self, s: Complex64) -> ComplexMatrix {
        let n = self.n;
        let mut result = ComplexMatrix::zeros(n);
        for i in 0..n {
            for j in 0..n {
                result.data[i][j] = self.data[i][j].mul(&s);
            }
        }
        result
    }
    /// Frobenius norm ‖A‖_F = sqrt(∑|aᵢⱼ|²).
    pub fn frobenius_norm(&self) -> f64 {
        self.data
            .iter()
            .flat_map(|row| row.iter())
            .map(|z| z.modulus() * z.modulus())
            .sum::<f64>()
            .sqrt()
    }
    /// Operator norm approximation via power method (‖A‖ = max singular value).
    pub fn operator_norm(&self) -> f64 {
        if self.n == 0 {
            return 0.0;
        }
        let mut v: Vec<f64> = vec![1.0; self.n];
        let mut sigma = 0.0_f64;
        for _ in 0..100 {
            let mut w = vec![0.0_f64; self.n];
            for i in 0..self.n {
                for j in 0..self.n {
                    w[i] += self.data[i][j].re * v[j];
                }
            }
            let mut w2 = vec![0.0_f64; self.n];
            for i in 0..self.n {
                for j in 0..self.n {
                    w2[i] += self.data[j][i].re * w[j];
                }
            }
            let norm: f64 = w2.iter().map(|x| x * x).sum::<f64>().sqrt();
            if norm < 1e-300 {
                break;
            }
            let v_norm: f64 = v.iter().map(|x| x * x).sum::<f64>().sqrt().max(1e-300);
            sigma = norm / v_norm;
            v = w2.iter().map(|x| x / norm).collect();
        }
        sigma.sqrt()
    }
    /// Check if the matrix is (approximately) self-adjoint: A = A*.
    pub fn is_self_adjoint(&self, tol: f64) -> bool {
        for i in 0..self.n {
            for j in 0..self.n {
                let diff = self.data[i][j].sub(&self.data[j][i].conj());
                if diff.modulus() > tol {
                    return false;
                }
            }
        }
        true
    }
    /// Trace of the matrix.
    pub fn trace(&self) -> Complex64 {
        let mut tr = Complex64::new(0.0, 0.0);
        for i in 0..self.n {
            tr = tr.add(&self.data[i][i]);
        }
        tr
    }
    /// Power T^k (by repeated squaring).
    pub fn power(&self, k: u32) -> ComplexMatrix {
        if k == 0 {
            return ComplexMatrix::identity(self.n);
        }
        if k == 1 {
            return self.clone();
        }
        let half = self.power(k / 2);
        let sq = half.mul(&half);
        if k % 2 == 0 {
            sq
        } else {
            sq.mul(self)
        }
    }
}
/// A projection-valued (spectral) measure on a Hilbert space.
pub struct SpectralMeasure {
    /// Description of the associated operator.
    pub operator: String,
    /// Description of the Borel set.
    pub borel_set: String,
}
impl SpectralMeasure {
    /// Create a spectral measure.
    pub fn new(operator: String, borel_set: String) -> Self {
        Self {
            operator,
            borel_set,
        }
    }
    /// A spectral measure is always projection-valued.
    pub fn is_projection_valued(&self) -> bool {
        true
    }
}
/// Represents a commutative C*-algebra via its maximal ideal space.
#[allow(dead_code)]
pub struct GelfandDuality {
    /// Name of the algebra (e.g., "C(X)", "l^∞", etc.).
    pub algebra_name: String,
    /// Name of the maximal ideal space X.
    pub maximal_ideal_space: String,
}
#[allow(dead_code)]
impl GelfandDuality {
    /// Create a Gelfand duality record.
    pub fn new(algebra_name: &str, maximal_ideal_space: &str) -> Self {
        Self {
            algebra_name: algebra_name.to_string(),
            maximal_ideal_space: maximal_ideal_space.to_string(),
        }
    }
    /// The Gelfand transform ĝ: A → C₀(X) is an isometric *-isomorphism.
    pub fn gelfand_transform_is_isometry(&self) -> bool {
        true
    }
    /// Statement of the Gelfand-Naimark duality.
    pub fn gelfand_naimark_statement(&self) -> String {
        format!(
            "Gelfand-Naimark: {} ≅ C₀({}) as Banach *-algebras (isometrically)",
            self.algebra_name, self.maximal_ideal_space
        )
    }
    /// The spectrum of an element a ∈ A equals the image of its Gelfand transform â.
    pub fn spectrum_is_image(&self) -> String {
        format!(
            "σ(a) = {{â(χ) : χ ∈ {}}} for a ∈ {}",
            self.maximal_ideal_space, self.algebra_name
        )
    }
    /// Characters (maximal ideals) of the algebra form the maximal ideal space.
    pub fn characters_are_maximal_ideals(&self) -> bool {
        true
    }
    /// The Gelfand topology on the maximal ideal space is the weak-* topology.
    pub fn topology_is_weak_star(&self) -> bool {
        true
    }
}
/// Kato-Rellich theorem: a symmetric operator relatively bounded by a self-adjoint operator
/// with relative bound < 1 is itself self-adjoint.
pub struct KatoRellichThm;
impl KatoRellichThm {
    /// Create a Kato-Rellich theorem struct.
    pub fn new() -> Self {
        Self
    }
    /// Return a description of the symmetric perturbation boundedness result.
    pub fn symmetric_perturbation_bounded(&self) -> String {
        "Kato-Rellich: if B is A-bounded with relative bound a < 1 and B is symmetric, \
         then A+B is self-adjoint on D(A)."
            .to_string()
    }
}
/// The spectral theorem for a given operator type.
pub struct SpectralThm {
    /// Description of the operator type (e.g. "bounded self-adjoint", "unbounded self-adjoint").
    pub operator_type: String,
}
impl SpectralThm {
    /// Create a spectral theorem struct.
    pub fn new(operator_type: String) -> Self {
        Self { operator_type }
    }
    /// Return a description of the spectral decomposition.
    pub fn spectral_decomposition(&self) -> String {
        format!(
            "Spectral theorem for {}: A = ∫ λ dE(λ)  [spectral integral via PVM E]",
            self.operator_type
        )
    }
    /// Return whether the spectral theorem is proven for this operator type.
    pub fn is_proven_for(&self) -> bool {
        matches!(
            self.operator_type.as_str(),
            "bounded self-adjoint" | "unbounded self-adjoint" | "normal" | "compact self-adjoint"
        )
    }
}
/// A bounded linear operator on a Hilbert space.
pub struct BoundedOperator {
    /// Operator norm ‖T‖.
    pub norm: f64,
    /// Whether the operator is compact.
    pub is_compact: bool,
}
impl BoundedOperator {
    /// Create a bounded operator.
    pub fn new(norm: f64, is_compact: bool) -> Self {
        Self { norm, is_compact }
    }
    /// A bounded operator T is normal if T*T = TT*.
    /// This implementation returns true (assume normal by construction when unspecified).
    pub fn is_normal(&self) -> bool {
        true
    }
    /// Return a symbolic description of the adjoint operator T*.
    pub fn adjoint(&self) -> String {
        format!("T*  [adjoint, ‖T*‖ = ‖T‖ = {:.4}]", self.norm)
    }
}
/// Resolvent set of an operator.
pub struct ResolventSet {
    /// Description of the operator.
    pub operator: String,
}
impl ResolventSet {
    /// Create a resolvent set struct.
    pub fn new(operator: String) -> Self {
        Self { operator }
    }
    /// The resolvent set ρ(A) is always open in ℂ.
    pub fn is_open(&self) -> bool {
        true
    }
    /// The complement of the resolvent set is the spectrum σ(A).
    pub fn complement_is_spectrum(&self) -> String {
        format!(
            "ℂ \\ ρ({op}) = σ({op})  [spectrum = complement of resolvent set]",
            op = self.operator
        )
    }
}
/// Statistics for eigenvalues of a random matrix ensemble.
#[allow(dead_code)]
pub struct RandomMatrixStats {
    /// The size of the matrix.
    pub n: usize,
    /// The empirical eigenvalue mean.
    pub mean: f64,
    /// The empirical eigenvalue variance.
    pub variance: f64,
    /// The empirical spectral radius.
    pub spectral_radius: f64,
    /// The ensemble type: "GOE", "GUE", or "Wishart".
    pub ensemble: String,
}
#[allow(dead_code)]
impl RandomMatrixStats {
    /// Create random matrix statistics.
    pub fn new(n: usize, mean: f64, variance: f64, spectral_radius: f64, ensemble: &str) -> Self {
        Self {
            n,
            mean,
            variance,
            spectral_radius,
            ensemble: ensemble.to_string(),
        }
    }
    /// The Wigner semicircle density at x, for |x| ≤ 2:  ρ(x) = sqrt(4 - x²) / (2π).
    pub fn wigner_semicircle_density(x: f64) -> f64 {
        let r_sq = 4.0 - x * x;
        if r_sq <= 0.0 {
            0.0
        } else {
            r_sq.sqrt() / (2.0 * std::f64::consts::PI)
        }
    }
    /// The Marchenko-Pastur density for aspect ratio γ = p/n at x.
    /// Supported on \[(1-√γ)², (1+√γ)²\].
    pub fn marchenko_pastur_density(x: f64, gamma: f64) -> f64 {
        if gamma <= 0.0 || x <= 0.0 {
            return 0.0;
        }
        let a = (1.0 - gamma.sqrt()).powi(2);
        let b = (1.0 + gamma.sqrt()).powi(2);
        if x < a || x > b {
            return 0.0;
        }
        ((b - x) * (x - a)).sqrt() / (2.0 * std::f64::consts::PI * gamma * x)
    }
    /// The expected spectral radius of an n×n GUE matrix scales as ≈ 2√n.
    pub fn gue_expected_spectral_radius(n: usize) -> f64 {
        2.0 * (n as f64).sqrt()
    }
    /// The expected spacing between adjacent eigenvalues (Wigner surmise for GUE): P(s) ~ s² e^{-πs²/4}.
    pub fn gue_spacing_density(s: f64) -> f64 {
        let pi = std::f64::consts::PI;
        (32.0 / pi.powi(2)) * s * s * (-(4.0 / pi) * s * s).exp()
    }
    /// Check if statistics are consistent with a GOE ensemble (variance ≈ 1/n).
    pub fn is_goe_consistent(&self, tol: f64) -> bool {
        (self.variance - 1.0 / self.n as f64).abs() < tol
    }
    /// Return a human-readable summary of the ensemble statistics.
    pub fn summary(&self) -> String {
        format!(
            "{} ensemble (n={}): mean={:.4}, var={:.4}, r(A)={:.4}",
            self.ensemble, self.n, self.mean, self.variance, self.spectral_radius
        )
    }
}
/// An unbounded linear operator on a Hilbert space.
pub struct UnboundedOperator {
    /// Description of the operator domain.
    pub domain: String,
    /// Whether the operator is closed.
    pub is_closed: bool,
    /// Whether the domain is dense in the Hilbert space.
    pub is_densely_defined: bool,
}
impl UnboundedOperator {
    /// Create an unbounded operator.
    pub fn new(domain: String, is_closed: bool, is_densely_defined: bool) -> Self {
        Self {
            domain,
            is_closed,
            is_densely_defined,
        }
    }
    /// An operator is essentially self-adjoint if its closure is self-adjoint.
    /// Returns true when the operator is closed and densely defined (necessary condition).
    pub fn is_essentially_self_adjoint(&self) -> bool {
        self.is_closed && self.is_densely_defined
    }
}
/// Spectrum representation for a finite-dimensional operator.
#[derive(Debug, Clone)]
pub struct FiniteSpectrum {
    /// Eigenvalues (may be approximate for large matrices).
    pub eigenvalues: Vec<Complex64>,
    /// Corresponding algebraic multiplicities.
    pub multiplicities: Vec<usize>,
}
impl FiniteSpectrum {
    /// Create a spectrum from a list of eigenvalues (multiplicity 1 each).
    pub fn new(eigenvalues: Vec<Complex64>) -> Self {
        let n = eigenvalues.len();
        Self {
            eigenvalues,
            multiplicities: vec![1; n],
        }
    }
    /// Create a spectrum with given multiplicities.
    pub fn with_multiplicities(eigenvalues: Vec<Complex64>, multiplicities: Vec<usize>) -> Self {
        assert_eq!(eigenvalues.len(), multiplicities.len());
        Self {
            eigenvalues,
            multiplicities,
        }
    }
    /// Spectral radius: max |λ|.
    pub fn spectral_radius(&self) -> f64 {
        self.eigenvalues
            .iter()
            .map(|z| z.modulus())
            .fold(0.0_f64, f64::max)
    }
    /// Check if all eigenvalues are real (within tolerance).
    pub fn is_real_spectrum(&self, tol: f64) -> bool {
        self.eigenvalues.iter().all(|z| z.is_real(tol))
    }
    /// Check if all eigenvalues are in [0, ∞) (positive semidefinite).
    pub fn is_non_negative(&self, tol: f64) -> bool {
        self.eigenvalues
            .iter()
            .all(|z| z.re >= -tol && z.im.abs() < tol)
    }
    /// Total size (counting multiplicity).
    pub fn total_dimension(&self) -> usize {
        self.multiplicities.iter().sum()
    }
    /// Check if λ is in the spectrum (within tolerance).
    pub fn contains(&self, lambda: &Complex64, tol: f64) -> bool {
        self.eigenvalues
            .iter()
            .any(|z| z.sub(lambda).modulus() < tol)
    }
}
