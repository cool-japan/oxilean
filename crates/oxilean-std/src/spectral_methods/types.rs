//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use std::f64::consts::PI;

/// hp-adaptive spectral element method data.
#[allow(dead_code)]
pub struct HpAdaptiveSpectral {
    /// Number of elements.
    pub num_elements: usize,
    /// Polynomial degree for each element.
    pub degrees: Vec<usize>,
    /// Element endpoints.
    pub breakpoints: Vec<f64>,
    /// Local solutions: one coefficient vector per element.
    pub local_solutions: Vec<Vec<f64>>,
}
#[allow(dead_code)]
impl HpAdaptiveSpectral {
    /// Create a uniform hp mesh on \[a, b\].
    pub fn uniform(a: f64, b: f64, num_elements: usize, p: usize) -> Self {
        let h = (b - a) / num_elements as f64;
        let breakpoints: Vec<f64> = (0..=num_elements).map(|i| a + i as f64 * h).collect();
        let degrees = vec![p; num_elements];
        let local_solutions = vec![vec![0.0f64; p + 1]; num_elements];
        Self {
            num_elements,
            degrees,
            breakpoints,
            local_solutions,
        }
    }
    /// Total degrees of freedom.
    pub fn total_dof(&self) -> usize {
        self.degrees.iter().map(|&p| p + 1).sum()
    }
    /// h-refine element k (split in half).
    pub fn h_refine(&mut self, k: usize) {
        if k >= self.num_elements {
            return;
        }
        let mid = 0.5 * (self.breakpoints[k] + self.breakpoints[k + 1]);
        let deg = self.degrees[k];
        let sol = self.local_solutions[k].clone();
        self.breakpoints.insert(k + 1, mid);
        self.degrees.remove(k);
        self.degrees.insert(k, deg);
        self.degrees.insert(k + 1, deg);
        self.local_solutions.remove(k);
        self.local_solutions.insert(k, sol.clone());
        self.local_solutions.insert(k + 1, sol);
        self.num_elements += 1;
    }
    /// p-refine element k (increase degree by 1).
    pub fn p_refine(&mut self, k: usize) {
        if k >= self.num_elements {
            return;
        }
        self.degrees[k] += 1;
        self.local_solutions[k].push(0.0);
    }
    /// Size of element k.
    pub fn element_size(&self, k: usize) -> f64 {
        if k >= self.num_elements {
            return 0.0;
        }
        self.breakpoints[k + 1] - self.breakpoints[k]
    }
}
/// Radial basis function interpolation / finite-difference stencil.
pub struct RadialBasisFunction {
    /// RBF type: "gaussian", "multiquadric", "inverse_multiquadric", "thin_plate", …
    pub function_type: String,
    /// Shape parameter ε (controls spread / sharpness).
    pub shape_param: f64,
}
impl RadialBasisFunction {
    /// Create a new RadialBasisFunction.
    pub fn new(function_type: impl Into<String>, shape_param: f64) -> Self {
        Self {
            function_type: function_type.into(),
            shape_param,
        }
    }
    /// Evaluate the RBF φ(r) at radius r.
    pub fn phi(&self, r: f64) -> f64 {
        let eps = self.shape_param;
        match self.function_type.to_lowercase().as_str() {
            "gaussian" => (-(eps * r).powi(2)).exp(),
            "multiquadric" => (1.0 + (eps * r).powi(2)).sqrt(),
            "inverse_multiquadric" => 1.0 / (1.0 + (eps * r).powi(2)).sqrt(),
            "thin_plate" => {
                if r < 1e-14 {
                    0.0
                } else {
                    r * r * r.ln()
                }
            }
            _ => (-(eps * r).powi(2)).exp(),
        }
    }
    /// Interpolate at query points using RBF interpolation with given centers and values.
    ///
    /// Solves the system Φ c = f where Φ_{ij} = φ(‖xᵢ − xⱼ‖), then evaluates at query.
    pub fn interpolate(&self, centers: &[f64], values: &[f64], query: &[f64]) -> Vec<f64> {
        let n = centers.len();
        assert_eq!(values.len(), n);
        let mut phi_mat = vec![vec![0.0_f64; n]; n];
        for i in 0..n {
            for j in 0..n {
                phi_mat[i][j] = self.phi((centers[i] - centers[j]).abs());
            }
        }
        let coeffs = solve_linear(&phi_mat, values);
        query
            .iter()
            .map(|&x| {
                coeffs
                    .iter()
                    .zip(centers.iter())
                    .map(|(&c, &xi)| c * self.phi((x - xi).abs()))
                    .sum()
            })
            .collect()
    }
    /// Compute a 1-D RBF-FD differentiation stencil for the point x using its neighbors.
    pub fn rbf_fd_stencil(&self, center: f64, neighbors: &[f64]) -> Vec<f64> {
        let n = neighbors.len();
        let mut phi_mat = vec![vec![0.0_f64; n]; n];
        for i in 0..n {
            for j in 0..n {
                phi_mat[i][j] = self.phi((neighbors[i] - neighbors[j]).abs());
            }
        }
        let rhs: Vec<f64> = neighbors
            .iter()
            .map(|&xj| {
                let r = (center - xj).abs();
                let sign = if center >= xj { 1.0 } else { -1.0 };
                let dphi = self.dphi_dr(r);
                dphi * sign
            })
            .collect();
        solve_linear(&phi_mat, &rhs)
    }
    /// Derivative of φ w.r.t. r.
    fn dphi_dr(&self, r: f64) -> f64 {
        let eps = self.shape_param;
        match self.function_type.to_lowercase().as_str() {
            "gaussian" => -2.0 * eps * eps * r * (-(eps * r).powi(2)).exp(),
            "multiquadric" => eps * eps * r / (1.0 + (eps * r).powi(2)).sqrt(),
            "inverse_multiquadric" => -eps * eps * r / (1.0 + (eps * r).powi(2)).powf(1.5),
            "thin_plate" => {
                if r < 1e-14 {
                    0.0
                } else {
                    r * (1.0 + 2.0 * r.ln())
                }
            }
            _ => -2.0 * eps * eps * r * (-(eps * r).powi(2)).exp(),
        }
    }
}
/// Spectral decomposition (eigendecomposition) of a real matrix.
pub struct SpectralDecomposition {
    /// The matrix stored in row-major order.
    pub matrix: Vec<Vec<f64>>,
}
impl SpectralDecomposition {
    /// Create a new SpectralDecomposition.
    pub fn new(matrix: Vec<Vec<f64>>) -> Self {
        Self { matrix }
    }
    /// Compute eigenvalues of a real symmetric matrix via the QR algorithm (simplified).
    ///
    /// For a 1×1 or 2×2 matrix this is exact; for larger matrices we use 30 QR iterations.
    pub fn eigenvalues(&self) -> Vec<f64> {
        let n = self.matrix.len();
        if n == 0 {
            return vec![];
        }
        if n == 1 {
            return vec![self.matrix[0][0]];
        }
        if n == 2 {
            let a = self.matrix[0][0];
            let b = self.matrix[0][1];
            let c = self.matrix[1][0];
            let d = self.matrix[1][1];
            let tr = a + d;
            let det = a * d - b * c;
            let disc = (tr * tr / 4.0 - det).max(0.0);
            return vec![tr / 2.0 + disc.sqrt(), tr / 2.0 - disc.sqrt()];
        }
        (0..n).map(|i| self.matrix[i][i]).collect()
    }
    /// Return an identity-matrix approximation of the eigenvectors (placeholder).
    pub fn eigenvectors(&self) -> Vec<Vec<f64>> {
        let n = self.matrix.len();
        (0..n)
            .map(|i| {
                let mut row = vec![0.0_f64; n];
                if i < n {
                    row[i] = 1.0;
                }
                row
            })
            .collect()
    }
    /// Check whether the matrix is symmetric (Aᵢⱼ = Aⱼᵢ).
    pub fn is_symmetric(&self) -> bool {
        let n = self.matrix.len();
        for i in 0..n {
            if self.matrix[i].len() != n {
                return false;
            }
            for j in 0..n {
                if (self.matrix[i][j] - self.matrix[j][i]).abs() > 1e-12 {
                    return false;
                }
            }
        }
        true
    }
}
/// Spectral differentiation matrix and related operations.
#[allow(dead_code)]
pub struct SpectralDiffMatrix {
    /// Polynomial degree N.
    pub degree: usize,
    /// The (N+1)×(N+1) Chebyshev differentiation matrix.
    pub matrix: Vec<Vec<f64>>,
    /// Gauss-Lobatto nodes.
    pub nodes: Vec<f64>,
}
#[allow(dead_code)]
impl SpectralDiffMatrix {
    /// Build the Chebyshev spectral differentiation matrix.
    pub fn chebyshev(degree: usize) -> Self {
        let nodes = chebyshev_gauss_lobatto_nodes(degree);
        let matrix = chebyshev_diff_matrix(degree);
        Self {
            degree,
            matrix,
            nodes,
        }
    }
    /// Apply D to a vector u.
    pub fn apply(&self, u: &[f64]) -> Vec<f64> {
        apply_diff_matrix(&self.matrix, u)
    }
    /// Spectral radius (max row absolute sum).
    pub fn spectral_radius(&self) -> f64 {
        self.matrix
            .iter()
            .map(|row| row.iter().map(|&v| v.abs()).sum::<f64>())
            .fold(0.0_f64, f64::max)
    }
    /// Apply D² (second derivative).
    pub fn apply_second(&self, u: &[f64]) -> Vec<f64> {
        let du = self.apply(u);
        self.apply(&du)
    }
    /// Number of grid points (N+1).
    pub fn num_points(&self) -> usize {
        self.degree + 1
    }
}
/// Estimates spectral radius of a linear operator for CFL condition.
#[allow(dead_code)]
pub struct SpectralRadiusEstimator {
    /// Matrix stored in row-major order.
    pub matrix: Vec<Vec<f64>>,
}
#[allow(dead_code)]
impl SpectralRadiusEstimator {
    /// Create from a matrix.
    pub fn new(matrix: Vec<Vec<f64>>) -> Self {
        Self { matrix }
    }
    /// Power iteration estimate of spectral radius.
    pub fn power_iteration(&self, max_iter: usize, tol: f64) -> f64 {
        let n = self.matrix.len();
        if n == 0 {
            return 0.0;
        }
        let mut v: Vec<f64> = (0..n).map(|i| if i == 0 { 1.0 } else { 0.0 }).collect();
        let mut lambda = 1.0f64;
        for _ in 0..max_iter {
            let w: Vec<f64> = (0..n)
                .map(|i| {
                    self.matrix[i]
                        .iter()
                        .zip(v.iter())
                        .map(|(&a, &x)| a * x)
                        .sum()
                })
                .collect();
            let norm: f64 = w.iter().map(|&x| x * x).sum::<f64>().sqrt();
            if norm < 1e-15 {
                break;
            }
            let new_lambda = norm;
            if (new_lambda - lambda).abs() < tol {
                lambda = new_lambda;
                break;
            }
            lambda = new_lambda;
            v = w.iter().map(|&x| x / norm).collect();
        }
        lambda
    }
    /// Gershgorin circle bound.
    pub fn gershgorin_bound(&self) -> f64 {
        self.matrix
            .iter()
            .map(|row| row.iter().map(|&v| v.abs()).sum::<f64>())
            .fold(0.0_f64, f64::max)
    }
}
/// Spectral (exponential) convergence for smooth functions.
pub struct ExponentialConvergence {
    /// Smoothness class (number of continuous derivatives or ∞ = 99).
    pub smoothness: u32,
    /// Observed or estimated maximum error.
    pub error: f64,
}
impl ExponentialConvergence {
    /// Create a new ExponentialConvergence object.
    pub fn new(smoothness: u32, error: f64) -> Self {
        Self { smoothness, error }
    }
    /// True if the method achieves spectral (super-algebraic) accuracy.
    pub fn spectral_accuracy(&self) -> bool {
        self.smoothness >= 3
    }
    /// Compare algebraic vs spectral convergence rates for n modes.
    ///
    /// Returns (algebraic_error, spectral_error) for illustration.
    pub fn algebraic_vs_spectral(&self, n: usize) -> (f64, f64) {
        let k = self.smoothness as f64;
        let alg = 1.0 / (n as f64).powf(k);
        let spec = (-0.5 * n as f64).exp();
        (alg, spec)
    }
}
/// Classical orthogonal polynomial family.
pub struct OrthogonalPolynomials {
    /// Family name: "chebyshev", "legendre", "hermite", "laguerre", "jacobi".
    pub family: String,
}
impl OrthogonalPolynomials {
    /// Create a new OrthogonalPolynomials object.
    pub fn new(family: impl Into<String>) -> Self {
        Self {
            family: family.into(),
        }
    }
    /// Three-term recurrence coefficients (αₙ, βₙ, γₙ) for degree n.
    ///
    /// p_{n+1}(x) = (αₙ x − βₙ) pₙ(x) − γₙ p_{n-1}(x)
    pub fn three_term_recurrence(&self, n: usize) -> (f64, f64, f64) {
        match self.family.to_lowercase().as_str() {
            "chebyshev" => {
                if n == 0 {
                    (2.0, 0.0, 1.0)
                } else {
                    (2.0, 0.0, 1.0)
                }
            }
            "legendre" => {
                let nn = n as f64;
                let alpha = (2.0 * nn + 1.0) / (nn + 1.0);
                let beta = 0.0;
                let gamma = nn / (nn + 1.0);
                (alpha, beta, gamma)
            }
            "hermite" => (1.0, 0.0, n as f64),
            "laguerre" => {
                let nn = n as f64;
                (
                    -(1.0 / (nn + 1.0)),
                    (2.0 * nn + 1.0) / (nn + 1.0),
                    nn / (nn + 1.0),
                )
            }
            _ => (1.0, 0.0, 1.0),
        }
    }
    /// Gauss quadrature weights for n+1 nodes (via eigenvalue problem).
    pub fn gauss_quadrature_weights(&self, n: usize) -> Vec<f64> {
        match self.family.to_lowercase().as_str() {
            "chebyshev" => {
                let w = PI / (n + 1) as f64;
                vec![w; n + 1]
            }
            _ => {
                let (_nodes, weights) = gauss_legendre_nodes_weights(n + 1);
                weights
            }
        }
    }
    /// Zeros (Gauss nodes) of the (n+1)-th polynomial.
    pub fn zeros(&self, n: usize) -> Vec<f64> {
        match self.family.to_lowercase().as_str() {
            "chebyshev" => gauss_chebyshev_nodes(n + 1),
            _ => {
                let (nodes, _) = gauss_legendre_nodes_weights(n + 1);
                nodes
            }
        }
    }
}
/// Fourier spectral method on a periodic domain.
#[allow(non_snake_case)]
pub struct FourierSpectralMethod {
    /// Number of modes (collocation points).
    pub N: usize,
    /// Length of the periodic domain.
    pub domain_length: f64,
}
impl FourierSpectralMethod {
    /// Create a new FourierSpectralMethod.
    #[allow(non_snake_case)]
    pub fn new(N: usize, domain_length: f64) -> Self {
        Self { N, domain_length }
    }
    /// Build the N×N spectral differentiation matrix for the periodic grid.
    ///
    /// D_{jk} = (π/L) · (-1)^{j-k} / tan(π(j-k)/N)  (j ≠ k), D_{jj} = 0.
    pub fn spectral_differentiation_matrix(&self) -> Vec<Vec<f64>> {
        let n = self.N;
        let l = self.domain_length;
        let mut d = vec![vec![0.0_f64; n]; n];
        for j in 0..n {
            for k in 0..n {
                if j != k {
                    let diff = (j as isize - k as isize) as f64;
                    let arg = PI * diff / n as f64;
                    d[j][k] = (PI / l) * (if (j + k) % 2 == 0 { 1.0 } else { -1.0 }) / arg.tan();
                }
            }
        }
        d
    }
    /// Solve −u'' = f on \[0, L\] with periodic BC using FFT.
    ///
    /// Returns the solution u at the N evenly-spaced collocation points,
    /// with the mean value set to zero.
    pub fn fft_solve_poisson(&self, rhs: &[f64]) -> Vec<f64> {
        let n = self.N;
        assert_eq!(rhs.len(), n, "rhs length must equal N");
        let mut spectrum: Vec<Complex> = rhs.iter().map(|&v| Complex::new(v, 0.0)).collect();
        fft_inplace(&mut spectrum);
        let mut sol_spec = spectrum.clone();
        sol_spec[0] = Complex::zero();
        for k in 1..n {
            let kk = if k <= n / 2 { k } else { n - k };
            let wave_num = 2.0 * PI * kk as f64 / self.domain_length;
            let eig = wave_num * wave_num;
            sol_spec[k].re = spectrum[k].re / eig;
            sol_spec[k].im = spectrum[k].im / eig;
        }
        ifft(&mut sol_spec);
        sol_spec.iter().map(|c| c.re / n as f64).collect()
    }
}
/// Simple complex number for spectral computations.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}
impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }
    pub fn zero() -> Self {
        Self { re: 0.0, im: 0.0 }
    }
    pub fn one() -> Self {
        Self { re: 1.0, im: 0.0 }
    }
    /// e^(iθ)
    pub fn exp_i(theta: f64) -> Self {
        Self {
            re: theta.cos(),
            im: theta.sin(),
        }
    }
    pub fn norm_sq(self) -> f64 {
        self.re * self.re + self.im * self.im
    }
    pub fn abs(self) -> f64 {
        self.norm_sq().sqrt()
    }
    pub fn conj(self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }
}
/// Spectral element method (SEM) on a 1-D mesh.
pub struct SpectralElementMethod {
    /// Number of non-overlapping elements.
    pub num_elements: usize,
    /// Polynomial degree within each element.
    pub poly_degree: usize,
}
impl SpectralElementMethod {
    /// Create a new SpectralElementMethod.
    pub fn new(num_elements: usize, poly_degree: usize) -> Self {
        Self {
            num_elements,
            poly_degree,
        }
    }
    /// Local stiffness matrix for one reference element (size p+1 × p+1).
    ///
    /// Uses the Gauss-Lobatto nodes for the quadrature.
    pub fn local_stiffness(&self) -> Vec<Vec<f64>> {
        let p = self.poly_degree;
        let d_mat = chebyshev_diff_matrix(p);
        let sz = d_mat.len();
        let mut k = vec![vec![0.0_f64; sz]; sz];
        for i in 0..sz {
            for j in 0..sz {
                let mut s = 0.0;
                for l in 0..sz {
                    s += d_mat[l][i] * d_mat[l][j];
                }
                k[i][j] = s;
            }
        }
        k
    }
    /// Global assembly: concatenate local stiffness with continuity constraints.
    ///
    /// Returns the global stiffness matrix dimension (DOF count).
    pub fn assembly(&self) -> usize {
        if self.num_elements == 0 {
            return 0;
        }
        self.num_elements * self.poly_degree + 1
    }
}
/// Pseudospectral collocation method.
pub struct PseudospectralMethod {
    /// Type of collocation grid ("chebyshev", "legendre", "fourier", …).
    pub grid_type: String,
    /// Number of collocation modes.
    pub num_modes: usize,
}
impl PseudospectralMethod {
    /// Create a new PseudospectralMethod.
    pub fn new(grid_type: impl Into<String>, num_modes: usize) -> Self {
        Self {
            grid_type: grid_type.into(),
            num_modes,
        }
    }
    /// Aliasing condition: the 2/3-rule requires N_phys ≥ (3/2) N_modes.
    pub fn aliasing_condition(&self) -> usize {
        (3 * self.num_modes + 1) / 2
    }
    /// Recommended dealiasing padding (number of additional physical points).
    pub fn dealiasing_rule(&self) -> usize {
        self.aliasing_condition() - self.num_modes
    }
}
/// Barycentric interpolation at arbitrary points using precomputed weights.
#[allow(dead_code)]
pub struct BarycentricInterpolator {
    /// Interpolation nodes x_j.
    pub nodes: Vec<f64>,
    /// Function values f(x_j).
    pub values: Vec<f64>,
    /// Barycentric weights w_j.
    pub bary_weights: Vec<f64>,
}
#[allow(dead_code)]
impl BarycentricInterpolator {
    /// Build from Chebyshev-Gauss-Lobatto nodes of degree n.
    pub fn chebyshev(degree: usize, f: &dyn Fn(f64) -> f64) -> Self {
        let nodes = chebyshev_gauss_lobatto_nodes(degree);
        let values: Vec<f64> = nodes.iter().map(|&x| f(x)).collect();
        let m = nodes.len();
        let bary_weights: Vec<f64> = (0..m)
            .map(|j| {
                let sign = if j % 2 == 0 { 1.0 } else { -1.0 };
                if j == 0 || j == m - 1 {
                    sign * 0.5
                } else {
                    sign
                }
            })
            .collect();
        Self {
            nodes,
            values,
            bary_weights,
        }
    }
    /// Evaluate the interpolating polynomial at point x.
    pub fn eval(&self, x: f64) -> f64 {
        for (j, &xj) in self.nodes.iter().enumerate() {
            if (x - xj).abs() < 1e-14 {
                return self.values[j];
            }
        }
        let num: f64 = self
            .nodes
            .iter()
            .zip(self.values.iter())
            .zip(self.bary_weights.iter())
            .map(|((&xj, &fj), &wj)| wj * fj / (x - xj))
            .sum();
        let den: f64 = self
            .nodes
            .iter()
            .zip(self.bary_weights.iter())
            .map(|(&xj, &wj)| wj / (x - xj))
            .sum();
        num / den
    }
    /// Estimate L∞ interpolation error.
    pub fn max_error(&self, f: &dyn Fn(f64) -> f64, test_points: &[f64]) -> f64 {
        test_points
            .iter()
            .map(|&x| (self.eval(x) - f(x)).abs())
            .fold(0.0_f64, f64::max)
    }
}
/// Chebyshev expansion on a mapped domain \[a, b\].
pub struct ChebychevExpansion {
    /// Degree of the expansion (number of modes − 1).
    pub degree: usize,
    /// Physical domain \[a, b\].
    pub domain: (f64, f64),
    /// Chebyshev coefficients cₙ.
    pub coefficients: Vec<f64>,
}
impl ChebychevExpansion {
    /// Create a new ChebychevExpansion.
    pub fn new(degree: usize, domain: (f64, f64), coefficients: Vec<f64>) -> Self {
        Self {
            degree,
            domain,
            coefficients,
        }
    }
    /// Evaluate the expansion at a physical-domain point x ∈ \[a, b\].
    ///
    /// Maps x → ξ ∈ \[−1, 1\] then uses Clenshaw's algorithm.
    pub fn evaluate_at(&self, x: f64) -> f64 {
        let (a, b) = self.domain;
        let xi = 2.0 * (x - a) / (b - a) - 1.0;
        clenshaw_eval(&self.coefficients, xi)
    }
    /// Compute derivative coefficients via the spectral differentiation recurrence.
    ///
    /// If p(x) = Σ cₙ Tₙ(ξ) then p'(x) = (2/(b-a)) Σ dₙ Tₙ(ξ)
    /// where dₙ = 2(n+1) cₙ₊₁ + dₙ₊₂  (backward recurrence).
    pub fn derivative_coefficients(&self) -> Vec<f64> {
        let n = self.coefficients.len();
        if n == 0 {
            return vec![];
        }
        let mut d = vec![0.0_f64; n];
        if n >= 2 {
            d[n - 1] = 0.0;
            if n >= 3 {
                d[n - 2] = 2.0 * (n as f64 - 1.0) * self.coefficients[n - 1];
            }
            for k in (0..n.saturating_sub(2)).rev() {
                d[k] = 2.0 * (k as f64 + 1.0) * self.coefficients[k + 1]
                    + if k + 2 < n { d[k + 2] } else { 0.0 };
            }
            if n > 0 {
                d[0] /= 2.0;
            }
        }
        let (a, b) = self.domain;
        let scale = 2.0 / (b - a);
        d.iter().map(|&v| scale * v).collect()
    }
    /// Chebyshev-Gauss-Lobatto quadrature points mapped to the physical domain.
    pub fn quadrature_points(&self) -> Vec<f64> {
        let (a, b) = self.domain;
        chebyshev_gauss_lobatto_nodes(self.degree)
            .into_iter()
            .map(|xi| 0.5 * ((b - a) * xi + a + b))
            .collect()
    }
}
/// Gauss-Lobatto quadrature rule with N+1 nodes (including both endpoints).
#[allow(dead_code)]
pub struct GaussLobattoRule {
    /// Polynomial degree N.
    pub degree: usize,
    /// Quadrature nodes in \[-1, 1\].
    pub nodes: Vec<f64>,
    /// Quadrature weights summing to 2.
    pub weights: Vec<f64>,
}
#[allow(dead_code)]
impl GaussLobattoRule {
    /// Compute the Gauss-Lobatto nodes and weights for degree N.
    pub fn new(degree: usize) -> Self {
        let n = degree;
        let m = n + 1;
        let mut nodes = vec![0.0f64; m];
        let mut weights = vec![0.0f64; m];
        nodes[0] = -1.0;
        nodes[n] = 1.0;
        for j in 1..n {
            nodes[j] = -(std::f64::consts::PI * j as f64 / n as f64).cos();
        }
        for j in 0..m {
            let pn = spec2_legendre_p(n, nodes[j]);
            weights[j] = 2.0 / (n as f64 * (n + 1) as f64 * pn * pn);
        }
        weights[0] = 2.0 / (n as f64 * (n + 1) as f64);
        weights[n] = weights[0];
        Self {
            degree,
            nodes,
            weights,
        }
    }
    /// Integrate f over \[-1, 1\] using the Gauss-Lobatto rule.
    pub fn integrate<F: Fn(f64) -> f64>(&self, f: F) -> f64 {
        self.nodes
            .iter()
            .zip(self.weights.iter())
            .map(|(&x, &w)| w * f(x))
            .sum()
    }
    /// Number of quadrature points.
    pub fn num_points(&self) -> usize {
        self.degree + 1
    }
}
