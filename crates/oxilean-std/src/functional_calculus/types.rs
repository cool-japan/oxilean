//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

/// Represents the numerical range (field of values) of an operator.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct NumericalRange {
    /// Boundary points sampled at regular angles.
    pub boundary_samples: Vec<(f64, f64)>,
    /// Whether the operator is normal (numerical range = convex hull of spectrum).
    pub is_normal: bool,
}
#[allow(dead_code)]
impl NumericalRange {
    /// Creates a numerical range from eigenvalues (normal operator case).
    pub fn from_eigenvalues(eigenvalues: &[f64]) -> Self {
        if eigenvalues.is_empty() {
            return NumericalRange {
                boundary_samples: Vec::new(),
                is_normal: true,
            };
        }
        let λ_min = eigenvalues.iter().copied().fold(f64::INFINITY, f64::min);
        let λ_max = eigenvalues
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max);
        let boundary_samples = vec![(λ_min, 0.0), (λ_max, 0.0)];
        NumericalRange {
            boundary_samples,
            is_normal: true,
        }
    }
    /// Returns the numerical radius: max{|z| : z in W(A)}.
    pub fn numerical_radius(&self) -> f64 {
        self.boundary_samples
            .iter()
            .map(|&(x, y)| (x * x + y * y).sqrt())
            .fold(0.0f64, f64::max)
    }
    /// Checks if 0 is in the numerical range (relevant for invertibility).
    pub fn contains_zero(&self, tol: f64) -> bool {
        if self.boundary_samples.len() >= 2 {
            let xmin = self
                .boundary_samples
                .iter()
                .map(|&(x, _)| x)
                .fold(f64::INFINITY, f64::min);
            let xmax = self
                .boundary_samples
                .iter()
                .map(|&(x, _)| x)
                .fold(f64::NEG_INFINITY, f64::max);
            xmin - tol <= 0.0 && xmax + tol >= 0.0
        } else {
            false
        }
    }
}
/// Bounded perturbation data for Miyadera-Voigt theorem.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BoundedPerturbation {
    /// Norm of the perturbation B.
    pub perturbation_norm: f64,
    /// Growth bound of the unperturbed semigroup.
    pub base_growth_bound: f64,
    /// Constant M for the unperturbed semigroup.
    pub base_constant: f64,
}
#[allow(dead_code)]
impl BoundedPerturbation {
    /// Creates perturbation data.
    pub fn new(b_norm: f64, omega: f64, m: f64) -> Self {
        BoundedPerturbation {
            perturbation_norm: b_norm,
            base_growth_bound: omega,
            base_constant: m,
        }
    }
    /// New growth bound after perturbation: ω + M * ||B||.
    pub fn perturbed_growth_bound(&self) -> f64 {
        self.base_growth_bound + self.base_constant * self.perturbation_norm
    }
    /// Checks if the perturbation preserves contractivity (||B|| small enough).
    pub fn preserves_contractivity(&self) -> bool {
        self.base_growth_bound + self.base_constant * self.perturbation_norm <= 0.0
    }
}
/// Represents a function in a function algebra, stored as a lookup table.
///
/// This models C(K) -- the algebra of continuous functions on a compact set K,
/// approximated by sampling at discrete points.
#[derive(Debug, Clone)]
pub struct FunctionAlgebraElement {
    /// Sampled function values at equally-spaced points in \[0, 1\].
    pub values: Vec<f64>,
}
impl FunctionAlgebraElement {
    /// Create from a Rust closure, sampling at `n` equally-spaced points in \[0,1\].
    pub fn from_fn<F: Fn(f64) -> f64>(f: F, n: usize) -> Self {
        let values = (0..n)
            .map(|i| {
                let t = if n <= 1 {
                    0.0
                } else {
                    i as f64 / (n - 1) as f64
                };
                f(t)
            })
            .collect();
        FunctionAlgebraElement { values }
    }
    /// Pointwise addition: (f + g)(x) = f(x) + g(x).
    pub fn add(&self, other: &FunctionAlgebraElement) -> FunctionAlgebraElement {
        let values = self
            .values
            .iter()
            .zip(other.values.iter())
            .map(|(a, b)| a + b)
            .collect();
        FunctionAlgebraElement { values }
    }
    /// Pointwise multiplication: (f * g)(x) = f(x) * g(x).
    pub fn multiply(&self, other: &FunctionAlgebraElement) -> FunctionAlgebraElement {
        let values = self
            .values
            .iter()
            .zip(other.values.iter())
            .map(|(a, b)| a * b)
            .collect();
        FunctionAlgebraElement { values }
    }
    /// Scalar multiplication: (c * f)(x) = c * f(x).
    pub fn scale(&self, c: f64) -> FunctionAlgebraElement {
        FunctionAlgebraElement {
            values: self.values.iter().map(|v| v * c).collect(),
        }
    }
    /// The supremum norm: ||f||_inf = max |f(x)|.
    pub fn sup_norm(&self) -> f64 {
        self.values.iter().map(|v| v.abs()).fold(0.0_f64, f64::max)
    }
    /// The L2 norm (approximated by the trapezoidal rule on the samples).
    pub fn l2_norm(&self) -> f64 {
        if self.values.len() <= 1 {
            return self.values.first().map(|v| v.abs()).unwrap_or(0.0);
        }
        let n = self.values.len();
        let h = 1.0 / (n - 1) as f64;
        let mut integral = 0.0;
        for i in 0..n - 1 {
            let f0 = self.values[i] * self.values[i];
            let f1 = self.values[i + 1] * self.values[i + 1];
            integral += (f0 + f1) * h / 2.0;
        }
        integral.sqrt()
    }
    /// Composition: (f o g)(x) = f(g(x)).
    ///
    /// Since we store sampled values, we interpolate g's output to find f's value.
    /// Clamps g's output to \[0, 1\] for lookups.
    pub fn compose(&self, g: &FunctionAlgebraElement) -> FunctionAlgebraElement {
        let n = self.values.len();
        if n <= 1 {
            return self.clone();
        }
        let values = g
            .values
            .iter()
            .map(|&gx| {
                let t = gx.clamp(0.0, 1.0) * (n - 1) as f64;
                let lo = (t.floor() as usize).min(n - 2);
                let hi = lo + 1;
                let frac = t - lo as f64;
                self.values[lo] * (1.0 - frac) + self.values[hi] * frac
            })
            .collect();
        FunctionAlgebraElement { values }
    }
    /// The multiplicative identity in C(K): the constant function 1.
    pub fn one(n: usize) -> Self {
        FunctionAlgebraElement {
            values: vec![1.0; n],
        }
    }
    /// The zero element in C(K): the constant function 0.
    pub fn zero_fn(n: usize) -> Self {
        FunctionAlgebraElement {
            values: vec![0.0; n],
        }
    }
}
/// A polynomial p(z) = coeffs\[0\] + coeffs\[1\]*z + coeffs\[2\]*z^2 + ...
///
/// Used to represent continuous functions on the spectrum for the continuous
/// functional calculus (by density of polynomials in C(K)).
#[derive(Debug, Clone)]
pub struct Polynomial {
    /// Coefficients in ascending degree order: `coeffs\[i\]` is the coefficient of z^i.
    pub coeffs: Vec<f64>,
}
impl Polynomial {
    /// Create a polynomial from coefficients in ascending degree order.
    pub fn new(coeffs: Vec<f64>) -> Self {
        Polynomial { coeffs }
    }
    /// The zero polynomial.
    pub fn zero() -> Self {
        Polynomial { coeffs: vec![0.0] }
    }
    /// A constant polynomial p(z) = c.
    pub fn constant(c: f64) -> Self {
        Polynomial { coeffs: vec![c] }
    }
    /// The identity polynomial p(z) = z.
    pub fn identity() -> Self {
        Polynomial {
            coeffs: vec![0.0, 1.0],
        }
    }
    /// The degree of the polynomial (0 for the zero polynomial).
    pub fn degree(&self) -> usize {
        if self.coeffs.is_empty() {
            return 0;
        }
        for i in (0..self.coeffs.len()).rev() {
            if self.coeffs[i].abs() > 1e-15 {
                return i;
            }
        }
        0
    }
    /// Evaluate the polynomial at a scalar point x using Horner's method.
    pub fn eval(&self, x: f64) -> f64 {
        if self.coeffs.is_empty() {
            return 0.0;
        }
        let mut result = 0.0;
        for c in self.coeffs.iter().rev() {
            result = result * x + c;
        }
        result
    }
    /// Add two polynomials.
    pub fn add(&self, other: &Polynomial) -> Polynomial {
        let max_len = self.coeffs.len().max(other.coeffs.len());
        let mut result = vec![0.0; max_len];
        for (i, c) in self.coeffs.iter().enumerate() {
            result[i] += c;
        }
        for (i, c) in other.coeffs.iter().enumerate() {
            result[i] += c;
        }
        Polynomial { coeffs: result }
    }
    /// Multiply two polynomials via convolution.
    pub fn multiply(&self, other: &Polynomial) -> Polynomial {
        if self.coeffs.is_empty() || other.coeffs.is_empty() {
            return Polynomial::zero();
        }
        let n = self.coeffs.len() + other.coeffs.len() - 1;
        let mut result = vec![0.0; n];
        for (i, a) in self.coeffs.iter().enumerate() {
            for (j, b) in other.coeffs.iter().enumerate() {
                result[i + j] += a * b;
            }
        }
        Polynomial { coeffs: result }
    }
    /// Scale the polynomial by a constant factor.
    pub fn scale(&self, s: f64) -> Polynomial {
        Polynomial {
            coeffs: self.coeffs.iter().map(|c| c * s).collect(),
        }
    }
    /// Compose two polynomials: compute (self o other)(z) = self(other(z)).
    pub fn compose(&self, other: &Polynomial) -> Polynomial {
        if self.coeffs.is_empty() {
            return Polynomial::zero();
        }
        let mut result = Polynomial::constant(*self.coeffs.last().unwrap_or(&0.0));
        for c in self.coeffs.iter().rev().skip(1) {
            result = result.multiply(other).add(&Polynomial::constant(*c));
        }
        result
    }
}
/// Computes the spectral radius of a square matrix via the power method.
///
/// The power method applied to ||A^k||^{1/k} converges monotonically down
/// to r(A) by the Gelfand formula.
#[derive(Debug, Clone)]
pub struct SpectralRadiusComputer {
    /// Maximum number of iterations for the power iteration.
    pub max_iters: u32,
    /// Convergence tolerance: stop when successive estimates differ by less
    /// than `tol`.
    pub tol: f64,
}
impl SpectralRadiusComputer {
    /// Create a new computer with given iteration count and tolerance.
    pub fn new(max_iters: u32, tol: f64) -> Self {
        SpectralRadiusComputer { max_iters, tol }
    }
    /// Default: 30 iterations, tolerance 1e-8.
    pub fn default() -> Self {
        SpectralRadiusComputer {
            max_iters: 30,
            tol: 1e-8,
        }
    }
    /// Compute the spectral radius of `mat` using the Gelfand formula
    /// r(A) = inf_k ||A^k||^{1/k}.
    ///
    /// Returns the best estimate found within `max_iters` steps.
    pub fn compute(&self, mat: &SquareMatrix) -> f64 {
        let mut best = f64::INFINITY;
        let mut prev = f64::INFINITY;
        for k in 1..=self.max_iters {
            let nk = mat.pow(k).frobenius_norm();
            let rk = if nk == 0.0 {
                0.0
            } else {
                nk.powf(1.0 / k as f64)
            };
            if rk < best {
                best = rk;
            }
            if (rk - prev).abs() < self.tol {
                break;
            }
            prev = rk;
        }
        best
    }
    /// Use the power-vector method: iterate v_{k+1} = A v_k / ||A v_k||
    /// and track ||A v_k|| to estimate the dominant eigenvalue magnitude.
    pub fn power_vector_method(&self, mat: &SquareMatrix, init: &[f64]) -> f64 {
        assert_eq!(init.len(), mat.dim, "init must have length mat.dim");
        let n = mat.dim;
        let mut v: Vec<f64> = init.to_vec();
        let norm0: f64 = v.iter().map(|x| x * x).sum::<f64>().sqrt();
        if norm0 > 1e-15 {
            for x in &mut v {
                *x /= norm0;
            }
        }
        let mut lambda = 0.0;
        for _ in 0..self.max_iters {
            let mut w = vec![0.0; n];
            for i in 0..n {
                for j in 0..n {
                    w[i] += mat.get(i, j) * v[j];
                }
            }
            let dot_wv: f64 = w.iter().zip(v.iter()).map(|(a, b)| a * b).sum();
            lambda = dot_wv.abs();
            let nw: f64 = w.iter().map(|x| x * x).sum::<f64>().sqrt();
            if nw < 1e-15 {
                return 0.0;
            }
            for x in &mut w {
                *x /= nw;
            }
            v = w;
        }
        lambda
    }
}
/// Computes the Fredholm index of a linear map A : R^m -> R^n represented
/// as a non-square (or square) matrix.
///
/// For a finite-dimensional matrix A in R^{n x m},
/// ind(A) = dim ker(A) - dim coker(A) = (m - rank A) - (n - rank A) = m - n.
///
/// For a square matrix ind(A) = 0 always; the interesting case is detecting
/// whether A is Fredholm (finite-dimensional kernel and cokernel).
#[derive(Debug, Clone)]
pub struct FredholmIndexCalculator {
    /// Row dimension (codomain dimension n).
    pub rows: usize,
    /// Column dimension (domain dimension m).
    pub cols: usize,
    /// Matrix entries in row-major order (rows * cols entries).
    pub entries: Vec<f64>,
}
impl FredholmIndexCalculator {
    /// Construct from row-major entries.
    ///
    /// Panics if `entries.len() != rows * cols`.
    pub fn new(rows: usize, cols: usize, entries: Vec<f64>) -> Self {
        assert_eq!(entries.len(), rows * cols, "entries must be rows*cols");
        FredholmIndexCalculator {
            rows,
            cols,
            entries,
        }
    }
    fn get(&self, r: usize, c: usize) -> f64 {
        self.entries[r * self.cols + c]
    }
    /// Estimate the numerical rank via Gaussian elimination with partial pivoting.
    pub fn numerical_rank(&self, tol: f64) -> usize {
        let mut mat: Vec<Vec<f64>> = (0..self.rows)
            .map(|r| (0..self.cols).map(|c| self.get(r, c)).collect())
            .collect();
        let mut rank = 0;
        let mut col = 0;
        let mut row = 0;
        while row < self.rows && col < self.cols {
            let mut max_val = 0.0;
            let mut max_row = row;
            for r in row..self.rows {
                if mat[r][col].abs() > max_val {
                    max_val = mat[r][col].abs();
                    max_row = r;
                }
            }
            if max_val < tol {
                col += 1;
                continue;
            }
            mat.swap(row, max_row);
            let pivot = mat[row][col];
            for c in col..self.cols {
                mat[row][c] /= pivot;
            }
            for r in 0..self.rows {
                if r != row {
                    let factor = mat[r][col];
                    for c in col..self.cols {
                        mat[r][c] -= factor * mat[row][c];
                    }
                }
            }
            rank += 1;
            row += 1;
            col += 1;
        }
        rank
    }
    /// Dimension of the (approximate) kernel: dim ker(A) = cols - rank(A).
    pub fn kernel_dim(&self, tol: f64) -> usize {
        self.cols - self.numerical_rank(tol)
    }
    /// Dimension of the (approximate) cokernel: dim coker(A) = rows - rank(A).
    pub fn cokernel_dim(&self, tol: f64) -> usize {
        self.rows - self.numerical_rank(tol)
    }
    /// Fredholm index: ind(A) = dim ker(A) - dim coker(A).
    ///
    /// For a matrix this equals `cols - rows` (independent of rank, as long
    /// as the map is Fredholm, i.e., both dimensions are finite).
    pub fn fredholm_index(&self, tol: f64) -> i64 {
        let k = self.kernel_dim(tol) as i64;
        let c = self.cokernel_dim(tol) as i64;
        k - c
    }
}
/// A simplified spectral measure (discrete version for finite matrices).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SpectralMeasure {
    /// Eigenvalues (the support of the spectral measure).
    pub eigenvalues: Vec<f64>,
    /// Spectral projections as rank-1 projectors (encoded as outer products of eigenvectors).
    pub projector_vectors: Vec<Vec<f64>>,
    /// Dimension of the space.
    pub dim: usize,
}
#[allow(dead_code)]
impl SpectralMeasure {
    /// Creates a spectral measure from eigenvalues (diagonal matrix case).
    pub fn diagonal(eigenvalues: Vec<f64>) -> Self {
        let dim = eigenvalues.len();
        let mut projector_vectors = Vec::new();
        for i in 0..dim {
            let mut v = vec![0.0f64; dim];
            v[i] = 1.0;
            projector_vectors.push(v);
        }
        SpectralMeasure {
            eigenvalues,
            projector_vectors,
            dim,
        }
    }
    /// Applies a Borel function f to the operator via functional calculus.
    /// Returns the eigenvalues of f(A).
    pub fn apply_function<F: Fn(f64) -> f64>(&self, f: F) -> Vec<f64> {
        self.eigenvalues.iter().map(|&λ| f(λ)).collect()
    }
    /// Computes trace(f(A)) = sum f(λ_i).
    pub fn trace_of_function<F: Fn(f64) -> f64>(&self, f: F) -> f64 {
        self.eigenvalues.iter().map(|&λ| f(λ)).sum()
    }
    /// Computes the spectral radius.
    pub fn spectral_radius(&self) -> f64 {
        self.eigenvalues
            .iter()
            .map(|&λ| λ.abs())
            .fold(0.0f64, f64::max)
    }
    /// Computes the operator norm (largest |λ|).
    pub fn operator_norm(&self) -> f64 {
        self.spectral_radius()
    }
    /// Checks if the operator is positive (all eigenvalues >= 0).
    pub fn is_positive(&self) -> bool {
        self.eigenvalues.iter().all(|&λ| λ >= 0.0)
    }
    /// Checks if the operator is positive definite (all eigenvalues > 0).
    pub fn is_positive_definite(&self) -> bool {
        self.eigenvalues.iter().all(|&λ| λ > 0.0)
    }
    /// Computes the square root of a positive operator.
    pub fn sqrt_eigenvalues(&self) -> Option<Vec<f64>> {
        if !self.is_positive() {
            return None;
        }
        Some(self.eigenvalues.iter().map(|&λ| λ.sqrt()).collect())
    }
    /// Computes the exponential exp(A) eigenvalues.
    pub fn exp_eigenvalues(&self) -> Vec<f64> {
        self.eigenvalues.iter().map(|&λ| λ.exp()).collect()
    }
    /// Computes the logarithm log(A) for positive definite A.
    pub fn log_eigenvalues(&self) -> Option<Vec<f64>> {
        if !self.is_positive_definite() {
            return None;
        }
        Some(self.eigenvalues.iter().map(|&λ| λ.ln()).collect())
    }
}
/// An element of a finite-dimensional Banach algebra, represented as a square
/// matrix together with a name for the algebra.
///
/// Provides an approximate spectrum via the characteristic polynomial roots
/// (for small dimensions) and the Gelfand spectral radius estimate.
#[derive(Debug, Clone)]
pub struct BanachAlgebraElem {
    /// The matrix representation of the element.
    pub matrix: SquareMatrix,
    /// Human-readable label for the algebra (e.g. "M_2(R)").
    pub algebra_name: String,
}
impl BanachAlgebraElem {
    /// Construct a new element in the algebra `algebra_name`.
    pub fn new(matrix: SquareMatrix, algebra_name: impl Into<String>) -> Self {
        BanachAlgebraElem {
            matrix,
            algebra_name: algebra_name.into(),
        }
    }
    /// Operator (Frobenius) norm as a proxy for the Banach algebra norm.
    pub fn norm(&self) -> f64 {
        self.matrix.frobenius_norm()
    }
    /// Estimate the spectral radius r(a) = lim ||a^n||^{1/n}.
    ///
    /// Uses power iteration over the first `iters` powers.
    pub fn spectral_radius_estimate(&self, iters: u32) -> f64 {
        self.matrix.spectral_radius(iters)
    }
    /// Check whether the element is approximately invertible (det != 0 for 2x2).
    pub fn is_invertible_2x2(&self) -> Option<bool> {
        if self.matrix.dim != 2 {
            return None;
        }
        let m = &self.matrix;
        let det = m.get(0, 0) * m.get(1, 1) - m.get(0, 1) * m.get(1, 0);
        Some(det.abs() > 1e-12)
    }
    /// For a 2x2 matrix, check membership of a scalar in the spectrum by
    /// testing non-invertibility of (a - lambda * I).
    pub fn is_in_spectrum_2x2(&self, lambda: f64) -> Option<bool> {
        if self.matrix.dim != 2 {
            return None;
        }
        let shifted = self.matrix.sub(&SquareMatrix::identity(2).scale(lambda));
        let det = shifted.get(0, 0) * shifted.get(1, 1) - shifted.get(0, 1) * shifted.get(1, 0);
        Some(det.abs() <= 1e-10)
    }
    /// Approximate the spectrum of a 2x2 matrix analytically.
    ///
    /// Returns the two eigenvalues (possibly complex — here returned as pairs
    /// (real_part, imag_part)) via the characteristic polynomial.
    pub fn spectrum_2x2(&self) -> Option<[(f64, f64); 2]> {
        if self.matrix.dim != 2 {
            return None;
        }
        let m = &self.matrix;
        let tr = m.get(0, 0) + m.get(1, 1);
        let det = m.get(0, 0) * m.get(1, 1) - m.get(0, 1) * m.get(1, 0);
        let disc = tr * tr - 4.0 * det;
        if disc >= 0.0 {
            let s = disc.sqrt();
            Some([((tr + s) / 2.0, 0.0), ((tr - s) / 2.0, 0.0)])
        } else {
            let s = (-disc).sqrt();
            Some([(tr / 2.0, s / 2.0), (tr / 2.0, -s / 2.0)])
        }
    }
}
/// A discretised simulation of the C_0-semigroup {T(t)} generated by a bounded
/// operator A via the explicit Euler approximation T(t) ≈ (I + (t/N) A)^N.
///
/// This is the Euler product formula (Trotter approximation) and converges to
/// exp(tA) as N -> infty for bounded A.
#[derive(Debug, Clone)]
pub struct OperatorSemigroup {
    /// The generator matrix A (bounded approximation).
    pub generator: SquareMatrix,
    /// Number of Euler steps used in the product approximation.
    pub num_steps: u32,
}
impl OperatorSemigroup {
    /// Construct with a given generator and number of discretisation steps.
    pub fn new(generator: SquareMatrix, num_steps: u32) -> Self {
        OperatorSemigroup {
            generator,
            num_steps,
        }
    }
    /// Evaluate the semigroup T(t) ≈ (I + (t/N) A)^N.
    ///
    /// For small t and large N this approximates exp(tA).
    pub fn eval(&self, t: f64) -> SquareMatrix {
        let n = self.num_steps.max(1);
        let dt = t / n as f64;
        let dim = self.generator.dim;
        let step = SquareMatrix::identity(dim).add(&self.generator.scale(dt));
        step.pow(n)
    }
    /// Apply T(t) to an initial vector x_0, returning the evolved state T(t) x_0.
    pub fn apply(&self, t: f64, x0: &[f64]) -> Vec<f64> {
        let tt = self.eval(t);
        assert_eq!(x0.len(), tt.dim);
        let n = tt.dim;
        let mut result = vec![0.0; n];
        for i in 0..n {
            for j in 0..n {
                result[i] += tt.get(i, j) * x0[j];
            }
        }
        result
    }
    /// Check the semigroup property T(s+t) ≈ T(s) T(t) by comparing Frobenius norms.
    ///
    /// Returns the relative error ||T(s+t) - T(s)T(t)||_F / ||T(s+t)||_F.
    pub fn check_semigroup_property(&self, s: f64, t: f64) -> f64 {
        let t_sum = self.eval(s + t);
        let t_s = self.eval(s);
        let t_t = self.eval(t);
        let product = t_s.mul(&t_t);
        let diff = t_sum.sub(&product);
        let err = diff.frobenius_norm();
        let denom = t_sum.frobenius_norm();
        if denom < 1e-15 {
            err
        } else {
            err / denom
        }
    }
}
/// Estimates the trace-class (nuclear) norm ||T||_1 = sum of singular values
/// of a square matrix, using power iteration to find singular values.
///
/// For an n x n matrix the singular values are the square roots of the
/// eigenvalues of T^T T (or T* T for complex operators).
#[derive(Debug, Clone)]
pub struct TraceClassNorm {
    /// The matrix whose trace norm we estimate.
    pub matrix: SquareMatrix,
}
impl TraceClassNorm {
    /// Construct from a `SquareMatrix`.
    pub fn new(matrix: SquareMatrix) -> Self {
        TraceClassNorm { matrix }
    }
    /// Compute T^T (transpose) for a square matrix.
    fn transpose(&self) -> SquareMatrix {
        let n = self.matrix.dim;
        let mut entries = vec![0.0; n * n];
        for i in 0..n {
            for j in 0..n {
                entries[j * n + i] = self.matrix.get(i, j);
            }
        }
        SquareMatrix { entries, dim: n }
    }
    /// Compute T^T T.
    fn gram_matrix(&self) -> SquareMatrix {
        self.transpose().mul(&self.matrix)
    }
    /// Estimate the largest singular value of `self.matrix` via power
    /// iteration on the Gram matrix T^T T.
    pub fn largest_singular_value(&self, iters: u32) -> f64 {
        let gram = self.gram_matrix();
        let computer = SpectralRadiusComputer::new(iters, 1e-10);
        let init: Vec<f64> = (0..self.matrix.dim)
            .map(|i| if i == 0 { 1.0 } else { 0.0 })
            .collect();
        let lambda_sq = computer.power_vector_method(&gram, &init);
        lambda_sq.sqrt()
    }
    /// For a 2x2 matrix, compute both singular values exactly and return
    /// the trace norm ||T||_1 = sigma_1 + sigma_2.
    pub fn trace_norm_2x2(&self) -> Option<f64> {
        if self.matrix.dim != 2 {
            return None;
        }
        let gram = self.gram_matrix();
        let tr = gram.get(0, 0) + gram.get(1, 1);
        let det = gram.get(0, 0) * gram.get(1, 1) - gram.get(0, 1) * gram.get(1, 0);
        let disc = (tr * tr - 4.0 * det).max(0.0);
        let ev1 = ((tr + disc.sqrt()) / 2.0).max(0.0);
        let ev2 = ((tr - disc.sqrt()) / 2.0).max(0.0);
        Some(ev1.sqrt() + ev2.sqrt())
    }
    /// Check if the operator is (approximately) Hilbert-Schmidt:
    /// ||T||_HS = sqrt(sum sigma_i^2) = ||T||_F (Frobenius norm).
    pub fn hilbert_schmidt_norm(&self) -> f64 {
        self.matrix.frobenius_norm()
    }
    /// Estimate the trace norm for a general n x n matrix as the Frobenius
    /// norm (upper bound, exact when T is normal).
    pub fn trace_norm_estimate(&self) -> f64 {
        self.matrix.frobenius_norm()
    }
    /// Compute the trace of T (sum of diagonal entries).
    pub fn trace(&self) -> f64 {
        self.matrix.trace()
    }
    /// Check the Lidskii approximation: for a normal matrix, trace(T) ≈ sum eigenvalues.
    /// Returns |tr(T) - (lambda_1 + lambda_2)| for 2x2 matrices.
    pub fn lidskii_error_2x2(&self) -> Option<f64> {
        if self.matrix.dim != 2 {
            return None;
        }
        let elem = BanachAlgebraElem::new(self.matrix.clone(), "M_2");
        let evs = elem.spectrum_2x2()?;
        let sum_ev = evs[0].0 + evs[1].0;
        Some((self.trace() - sum_ev).abs())
    }
}
/// Represents the resolvent data of a linear operator.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ResolventData {
    /// Spectrum (eigenvalues for finite-dimensional).
    pub spectrum: Vec<f64>,
    /// Operator norm bound.
    pub norm_bound: f64,
    /// Whether the operator is compact.
    pub is_compact: bool,
    /// Whether the operator is self-adjoint.
    pub is_self_adjoint: bool,
}
#[allow(dead_code)]
impl ResolventData {
    /// Creates resolvent data.
    pub fn new(spectrum: Vec<f64>, norm_bound: f64) -> Self {
        ResolventData {
            spectrum,
            norm_bound,
            is_compact: false,
            is_self_adjoint: false,
        }
    }
    /// Returns the resolvent set (complement of spectrum) membership check.
    pub fn in_resolvent_set(&self, lambda: f64, tol: f64) -> bool {
        self.spectrum.iter().all(|&sv| (lambda - sv).abs() > tol)
    }
    /// Estimates ||(λ - A)^{-1}|| using spectral theorem for self-adjoint.
    pub fn resolvent_norm_estimate(&self, lambda: f64) -> Option<f64> {
        if !self.is_self_adjoint {
            return None;
        }
        let min_dist = self
            .spectrum
            .iter()
            .map(|&spec_val| (lambda - spec_val).abs())
            .fold(f64::INFINITY, f64::min);
        if min_dist < 1e-14 {
            None
        } else {
            Some(1.0 / min_dist)
        }
    }
    /// Returns the spectral gap (distance between consecutive eigenvalues, min).
    pub fn spectral_gap(&self) -> Option<f64> {
        if self.spectrum.len() < 2 {
            return None;
        }
        let mut sorted = self.spectrum.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let gap = sorted
            .windows(2)
            .map(|w| w[1] - w[0])
            .fold(f64::INFINITY, f64::min);
        Some(gap)
    }
    /// Checks if the operator is invertible (0 not in spectrum).
    pub fn is_invertible(&self) -> bool {
        self.in_resolvent_set(0.0, 1e-14)
    }
}
/// A square matrix operator represented as a flat row-major array.
#[derive(Debug, Clone)]
pub struct SquareMatrix {
    /// Row-major entries.
    pub entries: Vec<f64>,
    /// Dimension (n x n).
    pub dim: usize,
}
impl SquareMatrix {
    /// Create a new square matrix from row-major entries.
    ///
    /// Panics if `entries.len() != dim * dim`.
    pub fn new(entries: Vec<f64>, dim: usize) -> Self {
        assert_eq!(
            entries.len(),
            dim * dim,
            "entries must have dim*dim elements"
        );
        SquareMatrix { entries, dim }
    }
    /// The n x n identity matrix.
    pub fn identity(dim: usize) -> Self {
        let mut entries = vec![0.0; dim * dim];
        for i in 0..dim {
            entries[i * dim + i] = 1.0;
        }
        SquareMatrix { entries, dim }
    }
    /// The n x n zero matrix.
    pub fn zero(dim: usize) -> Self {
        SquareMatrix {
            entries: vec![0.0; dim * dim],
            dim,
        }
    }
    /// Get entry at (row, col).
    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.entries[row * self.dim + col]
    }
    /// Set entry at (row, col).
    pub fn set(&mut self, row: usize, col: usize, val: f64) {
        self.entries[row * self.dim + col] = val;
    }
    /// Matrix addition.
    pub fn add(&self, other: &SquareMatrix) -> SquareMatrix {
        assert_eq!(self.dim, other.dim);
        let entries: Vec<f64> = self
            .entries
            .iter()
            .zip(other.entries.iter())
            .map(|(a, b)| a + b)
            .collect();
        SquareMatrix {
            entries,
            dim: self.dim,
        }
    }
    /// Matrix subtraction.
    pub fn sub(&self, other: &SquareMatrix) -> SquareMatrix {
        assert_eq!(self.dim, other.dim);
        let entries: Vec<f64> = self
            .entries
            .iter()
            .zip(other.entries.iter())
            .map(|(a, b)| a - b)
            .collect();
        SquareMatrix {
            entries,
            dim: self.dim,
        }
    }
    /// Scalar multiplication.
    pub fn scale(&self, s: f64) -> SquareMatrix {
        SquareMatrix {
            entries: self.entries.iter().map(|x| x * s).collect(),
            dim: self.dim,
        }
    }
    /// Matrix multiplication.
    pub fn mul(&self, other: &SquareMatrix) -> SquareMatrix {
        assert_eq!(self.dim, other.dim);
        let n = self.dim;
        let mut entries = vec![0.0; n * n];
        for i in 0..n {
            for j in 0..n {
                let mut s = 0.0;
                for k in 0..n {
                    s += self.entries[i * n + k] * other.entries[k * n + j];
                }
                entries[i * n + j] = s;
            }
        }
        SquareMatrix { entries, dim: n }
    }
    /// Compute A^k (matrix exponentiation by repeated squaring).
    pub fn pow(&self, k: u32) -> SquareMatrix {
        if k == 0 {
            return SquareMatrix::identity(self.dim);
        }
        if k == 1 {
            return self.clone();
        }
        let half = self.pow(k / 2);
        let sq = half.mul(&half);
        if k % 2 == 0 {
            sq
        } else {
            sq.mul(self)
        }
    }
    /// The operator norm (approximated by the Frobenius norm).
    pub fn operator_norm(&self) -> f64 {
        self.frobenius_norm()
    }
    /// The Frobenius norm: ||A||_F = sqrt(sum a_ij^2).
    pub fn frobenius_norm(&self) -> f64 {
        self.entries.iter().map(|x| x * x).sum::<f64>().sqrt()
    }
    /// The trace: sum of diagonal entries.
    pub fn trace(&self) -> f64 {
        (0..self.dim).map(|i| self.entries[i * self.dim + i]).sum()
    }
    /// Evaluate a polynomial at a matrix: p(A) = c_0 I + c_1 A + c_2 A^2 + ...
    ///
    /// Uses Horner's method for efficiency: p(A) = (... ((c_n A + c_{n-1}) A + c_{n-2}) A + ...) + c_0.
    pub fn poly_eval(&self, poly: &Polynomial) -> SquareMatrix {
        if poly.coeffs.is_empty() {
            return SquareMatrix::zero(self.dim);
        }
        let n = poly.coeffs.len();
        let mut result = SquareMatrix::identity(self.dim).scale(poly.coeffs[n - 1]);
        for i in (0..n - 1).rev() {
            result = result
                .mul(self)
                .add(&SquareMatrix::identity(self.dim).scale(poly.coeffs[i]));
        }
        result
    }
    /// Approximate spectral radius: r(A) = lim ||A^n||^{1/n}.
    ///
    /// Computed by evaluating ||A^n||^{1/n} for increasing n and returning the
    /// value at `max_iter`.
    pub fn spectral_radius(&self, max_iter: u32) -> f64 {
        let mut best = f64::INFINITY;
        for k in 1..=max_iter {
            let norm_k = self.pow(k).operator_norm();
            let r_k = norm_k.powf(1.0 / k as f64);
            if r_k < best {
                best = r_k;
            }
        }
        best
    }
    /// Compute the resolvent (lambda I - A)^{-1} for a 2x2 matrix using
    /// the explicit inverse formula.
    ///
    /// Returns `None` if the matrix is not 2x2 or the determinant is zero
    /// (i.e., lambda is in the spectrum).
    pub fn resolvent_2x2(&self, lambda: f64) -> Option<SquareMatrix> {
        if self.dim != 2 {
            return None;
        }
        let a = lambda - self.get(0, 0);
        let b = -self.get(0, 1);
        let c = -self.get(1, 0);
        let d = lambda - self.get(1, 1);
        let det = a * d - b * c;
        if det.abs() < 1e-12 {
            return None;
        }
        let inv_det = 1.0 / det;
        Some(SquareMatrix::new(
            vec![d * inv_det, -b * inv_det, -c * inv_det, a * inv_det],
            2,
        ))
    }
}
/// Represents a finite-dimensional C*-algebra as a product of matrix algebras.
/// A = M_{n_1} × M_{n_2} × ... × M_{n_k}.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FiniteDimCStarAlgebra {
    /// Block sizes n_i.
    pub blocks: Vec<usize>,
    /// Name.
    pub name: String,
}
#[allow(dead_code)]
impl FiniteDimCStarAlgebra {
    /// Creates a C*-algebra.
    pub fn new(name: &str, blocks: Vec<usize>) -> Self {
        FiniteDimCStarAlgebra {
            blocks,
            name: name.to_string(),
        }
    }
    /// Creates the algebra C^n (n commutative factors).
    pub fn commutative(n: usize) -> Self {
        FiniteDimCStarAlgebra::new(&format!("C^{n}"), vec![1; n])
    }
    /// Creates M_n(C).
    pub fn matrix_algebra(n: usize) -> Self {
        FiniteDimCStarAlgebra::new(&format!("M_{n}(C)"), vec![n])
    }
    /// Returns the total dimension as a vector space.
    pub fn dimension(&self) -> usize {
        self.blocks.iter().map(|&n| n * n).sum()
    }
    /// Returns the number of irreducible representations.
    pub fn num_irreps(&self) -> usize {
        self.blocks.len()
    }
    /// Checks if the algebra is commutative.
    pub fn is_commutative(&self) -> bool {
        self.blocks.iter().all(|&n| n == 1)
    }
    /// Returns the K_0 group (free abelian on generators of irreps).
    pub fn k0_rank(&self) -> usize {
        self.blocks.len()
    }
    /// Checks if this algebra is simple (single block).
    pub fn is_simple(&self) -> bool {
        self.blocks.len() == 1
    }
    /// Computes the trace (normalized to 1 on identity) of an element given as eigenvalues.
    pub fn normalized_trace(&self, block_idx: usize, value: f64) -> f64 {
        if block_idx < self.blocks.len() {
            let n = self.blocks[block_idx] as f64;
            value / n
        } else {
            0.0
        }
    }
}
/// Data for a strongly continuous semigroup {T(t)}_{t>=0}.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StrongContSemigroup {
    /// Generator A (represented as spectral data).
    pub generator_spectrum: Vec<f64>,
    /// Growth bound ω: ||T(t)|| <= M e^{ωt}.
    pub growth_bound: f64,
    /// Constant M in the bound.
    pub bound_constant: f64,
    /// Whether the semigroup is analytic.
    pub is_analytic: bool,
    /// Whether the semigroup is contractive (ω <= 0, M = 1).
    pub is_contractive: bool,
}
#[allow(dead_code)]
impl StrongContSemigroup {
    /// Creates a strongly continuous semigroup.
    pub fn new(generator_spectrum: Vec<f64>, growth_bound: f64) -> Self {
        let is_contractive = growth_bound <= 0.0;
        StrongContSemigroup {
            generator_spectrum,
            growth_bound,
            bound_constant: 1.0,
            is_analytic: false,
            is_contractive,
        }
    }
    /// Estimates ||T(t)|| <= M e^{ωt}.
    pub fn norm_bound(&self, t: f64) -> f64 {
        self.bound_constant * (self.growth_bound * t).exp()
    }
    /// Computes T(t)v for a diagonal generator (eigenvalue decomposition).
    pub fn apply_at_time(&self, t: f64, initial: &[f64]) -> Vec<f64> {
        initial
            .iter()
            .zip(self.generator_spectrum.iter())
            .map(|(&v, &λ)| v * (λ * t).exp())
            .collect()
    }
    /// Checks Hille-Yosida condition: (ω, ∞) ⊂ resolvent set.
    pub fn check_hille_yosida(&self) -> bool {
        self.generator_spectrum
            .iter()
            .all(|&λ| λ <= self.growth_bound + 1e-10)
    }
    /// Returns the spectral bound s(A) = sup{Re(λ) : λ ∈ σ(A)}.
    pub fn spectral_bound(&self) -> f64 {
        self.generator_spectrum
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max)
    }
}
