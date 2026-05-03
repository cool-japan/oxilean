//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
/// Tikhonov-regularized least-squares solver: minimizes ||Ax - b||² + λ||x||².
///
/// Solves the normal equations (AᵀA + λI)x = Aᵀb using Gaussian elimination.
#[allow(dead_code)]
pub struct TikhonovSolver {
    pub lambda: f64,
}
#[allow(dead_code)]
impl TikhonovSolver {
    /// Create a new `TikhonovSolver` with regularization parameter `lambda`.
    pub fn new(lambda: f64) -> Self {
        Self { lambda }
    }
    /// Solve the regularized least-squares problem.
    ///
    /// `a` is the m×n coefficient matrix (row-major), `b` is the rhs of length m.
    /// Returns the solution vector of length n, or `None` if the system is singular.
    pub fn solve(&self, a: &[Vec<f64>], b: &[f64]) -> Option<Vec<f64>> {
        let m = a.len();
        let n = if m == 0 { return None } else { a[0].len() };
        let mut ata = vec![vec![0.0f64; n]; n];
        for i in 0..n {
            for j in 0..n {
                let mut s = 0.0;
                for k in 0..m {
                    s += a[k][i] * a[k][j];
                }
                ata[i][j] = s;
            }
            ata[i][i] += self.lambda;
        }
        let mut atb = vec![0.0f64; n];
        for i in 0..n {
            let mut s = 0.0;
            for k in 0..m {
                s += a[k][i] * b[k];
            }
            atb[i] = s;
        }
        gaussian_elimination(ata, atb)
    }
}
/// Bisection root-finder with configurable tolerance and iteration limit.
pub struct BisectionSolver {
    pub tol: f64,
    pub max_iter: u32,
}
impl BisectionSolver {
    /// Create a new `BisectionSolver`.
    pub fn new(tol: f64, max_iter: u32) -> Self {
        Self { tol, max_iter }
    }
    /// Find a root of `f` in `[a, b]`.  Returns `None` if the sign condition fails or
    /// the solver does not converge.
    pub fn solve(&self, f: &dyn Fn(f64) -> f64, mut a: f64, mut b: f64) -> Option<f64> {
        if f(a) * f(b) > 0.0 {
            return None;
        }
        for _ in 0..self.max_iter {
            let mid = (a + b) / 2.0;
            let fm = f(mid);
            if fm.abs() < self.tol || (b - a) / 2.0 < self.tol {
                return Some(mid);
            }
            if f(a) * fm < 0.0 {
                b = mid;
            } else {
                a = mid;
            }
        }
        Some((a + b) / 2.0)
    }
}
/// Newton–Raphson solver that also reports whether convergence was achieved.
pub struct NewtonRaphsonSolver {
    pub tol: f64,
    pub max_iter: u32,
}
impl NewtonRaphsonSolver {
    /// Create a new `NewtonRaphsonSolver`.
    pub fn new(tol: f64, max_iter: u32) -> Self {
        Self { tol, max_iter }
    }
    /// Attempt to find a root of `f` starting from `x0`.
    /// Returns `(root, converged)`.
    pub fn solve(
        &self,
        f: &dyn Fn(f64) -> f64,
        df: &dyn Fn(f64) -> f64,
        mut x: f64,
    ) -> (f64, bool) {
        for _ in 0..self.max_iter {
            let fx = f(x);
            if fx.abs() < self.tol {
                return (x, true);
            }
            let dfx = df(x);
            if dfx.abs() < 1e-15 {
                return (x, false);
            }
            x -= fx / dfx;
        }
        (x, f(x).abs() < self.tol)
    }
}
/// Sparse matrix in Compressed Sparse Row (CSR) format.
///
/// `row_ptr\[i\]..row_ptr[i+1]` gives the range of column indices/values for row `i`.
pub struct SparseMatrix {
    /// Number of rows.
    pub nrows: usize,
    /// Number of columns.
    pub ncols: usize,
    /// Row pointers (length nrows + 1).
    pub row_ptr: Vec<usize>,
    /// Column indices of non-zero entries.
    pub col_idx: Vec<usize>,
    /// Non-zero values.
    pub values: Vec<f64>,
}
impl SparseMatrix {
    /// Build a `SparseMatrix` from a list of `(row, col, value)` triplets.
    pub fn from_triplets(nrows: usize, ncols: usize, triplets: &[(usize, usize, f64)]) -> Self {
        let mut counts = vec![0usize; nrows];
        for &(r, _, _) in triplets {
            counts[r] += 1;
        }
        let mut row_ptr = vec![0usize; nrows + 1];
        for i in 0..nrows {
            row_ptr[i + 1] = row_ptr[i] + counts[i];
        }
        let nnz = triplets.len();
        let mut col_idx = vec![0usize; nnz];
        let mut values = vec![0.0f64; nnz];
        let mut pos = row_ptr.clone();
        for &(r, c, v) in triplets {
            let p = pos[r];
            col_idx[p] = c;
            values[p] = v;
            pos[r] += 1;
        }
        Self {
            nrows,
            ncols,
            row_ptr,
            col_idx,
            values,
        }
    }
    /// Sparse matrix-vector product: y = A * x.
    pub fn matvec(&self, x: &[f64]) -> Vec<f64> {
        assert_eq!(x.len(), self.ncols, "x length must equal ncols");
        let mut y = vec![0.0f64; self.nrows];
        for i in 0..self.nrows {
            let start = self.row_ptr[i];
            let end = self.row_ptr[i + 1];
            for k in start..end {
                y[i] += self.values[k] * x[self.col_idx[k]];
            }
        }
        y
    }
    /// Return the number of non-zero entries.
    pub fn nnz(&self) -> usize {
        self.values.len()
    }
}
/// A closed real interval \[lo, hi\] used for validated/verified computation.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Interval {
    pub lo: f64,
    pub hi: f64,
}
#[allow(dead_code)]
impl Interval {
    /// Construct the interval \[lo, hi\].  Panics if lo > hi.
    pub fn new(lo: f64, hi: f64) -> Self {
        assert!(lo <= hi, "Interval::new: lo ({lo}) must be <= hi ({hi})");
        Self { lo, hi }
    }
    /// Construct a point interval \[x, x\].
    pub fn point(x: f64) -> Self {
        Self { lo: x, hi: x }
    }
    /// Interval addition: \[a,b\] + \[c,d\] = \[a+c, b+d\].
    pub fn add(self, other: Self) -> Self {
        Self {
            lo: self.lo + other.lo,
            hi: self.hi + other.hi,
        }
    }
    /// Interval subtraction: \[a,b\] - \[c,d\] = \[a-d, b-c\].
    pub fn sub(self, other: Self) -> Self {
        Self {
            lo: self.lo - other.hi,
            hi: self.hi - other.lo,
        }
    }
    /// Interval multiplication (all four products, take min/max).
    pub fn mul(self, other: Self) -> Self {
        let p = [
            self.lo * other.lo,
            self.lo * other.hi,
            self.hi * other.lo,
            self.hi * other.hi,
        ];
        let lo = p.iter().cloned().fold(f64::INFINITY, f64::min);
        let hi = p.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        Self { lo, hi }
    }
    /// Interval width hi - lo.
    pub fn width(self) -> f64 {
        self.hi - self.lo
    }
    /// Midpoint of the interval.
    pub fn mid(self) -> f64 {
        (self.lo + self.hi) / 2.0
    }
    /// Check whether a real value is contained in \[lo, hi\].
    pub fn contains(self, x: f64) -> bool {
        self.lo <= x && x <= self.hi
    }
    /// Interval enclosure of `sqrt` (valid for non-negative intervals).
    pub fn sqrt(self) -> Self {
        assert!(
            self.lo >= 0.0,
            "Interval::sqrt requires non-negative interval"
        );
        Self {
            lo: self.lo.sqrt(),
            hi: self.hi.sqrt(),
        }
    }
}
/// Power iteration to find the dominant eigenvalue of a dense matrix.
pub struct PowerIterationSolver {
    pub tol: f64,
    pub max_iter: u32,
}
impl PowerIterationSolver {
    /// Create a new `PowerIterationSolver`.
    pub fn new(tol: f64, max_iter: u32) -> Self {
        Self { tol, max_iter }
    }
    /// Find the dominant eigenvalue and eigenvector of `a` (row-major, n×n).
    ///
    /// Returns `(eigenvalue, eigenvector)` or `None` if not converged.
    pub fn solve(&self, a: &[Vec<f64>]) -> Option<(f64, Vec<f64>)> {
        let n = a.len();
        if n == 0 {
            return None;
        }
        let mut v: Vec<f64> = vec![1.0; n];
        let norm: f64 = v.iter().map(|x| x * x).sum::<f64>().sqrt();
        v.iter_mut().for_each(|x| *x /= norm);
        let mut eigenvalue = 0.0;
        for _ in 0..self.max_iter {
            let w: Vec<f64> = (0..n)
                .map(|i| a[i].iter().zip(v.iter()).map(|(aij, vj)| aij * vj).sum())
                .collect();
            let new_eig: f64 = w.iter().zip(v.iter()).map(|(wi, vi)| wi * vi).sum();
            let w_norm: f64 = w.iter().map(|x| x * x).sum::<f64>().sqrt();
            if w_norm < 1e-15 {
                return None;
            }
            let new_v: Vec<f64> = w.iter().map(|x| x / w_norm).collect();
            if (new_eig - eigenvalue).abs() < self.tol {
                return Some((new_eig, new_v));
            }
            eigenvalue = new_eig;
            v = new_v;
        }
        None
    }
}
/// Classic 4th-order Runge-Kutta ODE stepper.
pub struct RungeKutta4Solver {
    pub h: f64,
}
impl RungeKutta4Solver {
    /// Create a new RK4 solver with step size `h`.
    pub fn new(h: f64) -> Self {
        Self { h }
    }
    /// Advance the solution by one step.
    pub fn step(&self, f: &dyn Fn(f64, f64) -> f64, t: f64, y: f64) -> f64 {
        let h = self.h;
        let k1 = f(t, y);
        let k2 = f(t + h / 2.0, y + h / 2.0 * k1);
        let k3 = f(t + h / 2.0, y + h / 2.0 * k2);
        let k4 = f(t + h, y + h * k3);
        y + h / 6.0 * (k1 + 2.0 * k2 + 2.0 * k3 + k4)
    }
    /// Integrate from `t0` to `t_end`, returning `(t, y)` pairs.
    pub fn integrate(
        &self,
        f: &dyn Fn(f64, f64) -> f64,
        t0: f64,
        y0: f64,
        t_end: f64,
    ) -> Vec<(f64, f64)> {
        let steps = ((t_end - t0) / self.h).ceil() as u64;
        let mut result = Vec::with_capacity(steps as usize + 1);
        let mut t = t0;
        let mut y = y0;
        result.push((t, y));
        for _ in 0..steps {
            let t_next = (t + self.h).min(t_end);
            let h_actual = t_next - t;
            let k1 = f(t, y);
            let k2 = f(t + h_actual / 2.0, y + h_actual / 2.0 * k1);
            let k3 = f(t + h_actual / 2.0, y + h_actual / 2.0 * k2);
            let k4 = f(t + h_actual, y + h_actual * k3);
            y += h_actual / 6.0 * (k1 + 2.0 * k2 + 2.0 * k3 + k4);
            t = t_next;
            result.push((t, y));
            if (t - t_end).abs() < 1e-14 {
                break;
            }
        }
        result
    }
}
/// Gradient-descent minimizer for a smooth function f : Rⁿ → R.
///
/// Uses fixed step size (learning rate) with optional Armijo line-search.
#[allow(dead_code)]
pub struct GradientDescentOptimizer {
    pub learning_rate: f64,
    pub tol: f64,
    pub max_iter: u32,
}
#[allow(dead_code)]
impl GradientDescentOptimizer {
    /// Create a new optimizer.
    pub fn new(learning_rate: f64, tol: f64, max_iter: u32) -> Self {
        Self {
            learning_rate,
            tol,
            max_iter,
        }
    }
    /// Minimize `f` starting from `x0` using `grad` to supply ∇f.
    ///
    /// Returns `(minimizer, num_iters, converged)`.
    pub fn minimize(&self, grad: &dyn Fn(&[f64]) -> Vec<f64>, x0: &[f64]) -> (Vec<f64>, u32, bool) {
        let n = x0.len();
        let mut x = x0.to_vec();
        for iter in 0..self.max_iter {
            let g = grad(&x);
            let g_norm: f64 = g.iter().map(|v| v * v).sum::<f64>().sqrt();
            if g_norm < self.tol {
                return (x, iter, true);
            }
            for i in 0..n {
                x[i] -= self.learning_rate * g[i];
            }
        }
        let g = grad(&x);
        let g_norm: f64 = g.iter().map(|v| v * v).sum::<f64>().sqrt();
        (x, self.max_iter, g_norm < self.tol)
    }
}
/// Crank-Nicolson finite-difference solver for the 1-D heat equation u_t = κ u_xx
/// on the spatial domain \[0, L\] with Dirichlet boundary conditions u(0,t) = u(L,t) = 0.
///
/// The scheme is unconditionally stable (A-stable).
#[allow(dead_code)]
pub struct CrankNicolsonSolver {
    /// Diffusivity coefficient κ.
    pub kappa: f64,
    /// Spatial domain length.
    pub domain_length: f64,
    /// Number of internal spatial nodes.
    pub nx: usize,
    /// Time step.
    pub dt: f64,
}
#[allow(dead_code)]
impl CrankNicolsonSolver {
    /// Create a new `CrankNicolsonSolver`.
    pub fn new(kappa: f64, domain_length: f64, nx: usize, dt: f64) -> Self {
        Self {
            kappa,
            domain_length,
            nx,
            dt,
        }
    }
    /// Advance the solution `u` (length `nx`, interior nodes) by one time step.
    ///
    /// Returns the updated interior values or `None` if the tridiagonal solve fails.
    pub fn step(&self, u: &[f64]) -> Option<Vec<f64>> {
        let n = self.nx;
        assert_eq!(u.len(), n, "u must have length nx");
        let dx = self.domain_length / (n + 1) as f64;
        let r = self.kappa * self.dt / (dx * dx);
        let alpha = -r / 2.0;
        let beta = 1.0 + r;
        let mut rhs = vec![0.0f64; n];
        for i in 0..n {
            let ul = if i > 0 { u[i - 1] } else { 0.0 };
            let uc = u[i];
            let ur = if i < n - 1 { u[i + 1] } else { 0.0 };
            rhs[i] = (r / 2.0) * ul + (1.0 - r) * uc + (r / 2.0) * ur;
        }
        let mut c_prime = vec![0.0f64; n];
        let mut d_prime = vec![0.0f64; n];
        c_prime[0] = alpha / beta;
        d_prime[0] = rhs[0] / beta;
        for i in 1..n {
            let denom = beta - alpha * c_prime[i - 1];
            if denom.abs() < 1e-15 {
                return None;
            }
            c_prime[i] = alpha / denom;
            d_prime[i] = (rhs[i] - alpha * d_prime[i - 1]) / denom;
        }
        let mut u_new = vec![0.0f64; n];
        u_new[n - 1] = d_prime[n - 1];
        for i in (0..n - 1).rev() {
            u_new[i] = d_prime[i] - c_prime[i] * u_new[i + 1];
        }
        Some(u_new)
    }
    /// Integrate from `t=0` to `t=t_end`, returning the solution at each stored step.
    pub fn integrate(&self, u0: &[f64], t_end: f64) -> Vec<Vec<f64>> {
        let steps = (t_end / self.dt).ceil() as u64;
        let mut u = u0.to_vec();
        let mut history = Vec::with_capacity(steps as usize + 1);
        history.push(u.clone());
        for _ in 0..steps {
            if let Some(u_new) = self.step(&u) {
                u = u_new;
            } else {
                break;
            }
            history.push(u.clone());
        }
        history
    }
}
/// Monte Carlo integrator over \[a, b\] using a control variate to reduce variance.
///
/// The control variate `cv` must have a known exact integral `cv_integral` over \[a,b\].
/// Uses a simple pseudo-random sequence via a linear congruential generator (no external deps).
#[allow(dead_code)]
pub struct MonteCarloIntegrator {
    pub n_samples: u64,
    pub seed: u64,
}
#[allow(dead_code)]
impl MonteCarloIntegrator {
    /// Create a new `MonteCarloIntegrator`.
    pub fn new(n_samples: u64, seed: u64) -> Self {
        Self { n_samples, seed }
    }
    /// Generate pseudo-random floats in [0,1) using a LCG.
    fn lcg_samples(&self, n: usize) -> Vec<f64> {
        let mut state = self.seed.wrapping_add(1);
        let mut out = Vec::with_capacity(n);
        let a: u64 = 1664525;
        let c: u64 = 1013904223;
        let m: u64 = 1 << 32;
        for _ in 0..n {
            state = (a.wrapping_mul(state).wrapping_add(c)) % m;
            out.push(state as f64 / m as f64);
        }
        out
    }
    /// Estimate ∫_a^b f(x) dx using crude Monte Carlo.
    pub fn integrate(&self, f: &dyn Fn(f64) -> f64, a: f64, b: f64) -> f64 {
        let samples = self.lcg_samples(self.n_samples as usize);
        let sum: f64 = samples.iter().map(|&u| f(a + u * (b - a))).sum();
        (b - a) * sum / self.n_samples as f64
    }
    /// Estimate ∫_a^b f(x) dx using a control variate `cv` with known integral `cv_integral`.
    ///
    /// Chooses optimal coefficient β = Cov(f, cv) / Var(cv) empirically from the same samples.
    pub fn integrate_with_control_variate(
        &self,
        f: &dyn Fn(f64) -> f64,
        cv: &dyn Fn(f64) -> f64,
        cv_integral: f64,
        a: f64,
        b: f64,
    ) -> f64 {
        let n = self.n_samples as usize;
        let samples = self.lcg_samples(n);
        let xs: Vec<f64> = samples.iter().map(|&u| a + u * (b - a)).collect();
        let fv: Vec<f64> = xs.iter().map(|&x| f(x)).collect();
        let cv_v: Vec<f64> = xs.iter().map(|&x| cv(x)).collect();
        let f_mean = fv.iter().sum::<f64>() / n as f64;
        let cv_mean = cv_v.iter().sum::<f64>() / n as f64;
        let cov: f64 = fv
            .iter()
            .zip(cv_v.iter())
            .map(|(&fi, &ci)| (fi - f_mean) * (ci - cv_mean))
            .sum::<f64>()
            / n as f64;
        let var_cv: f64 = cv_v.iter().map(|&ci| (ci - cv_mean).powi(2)).sum::<f64>() / n as f64;
        let beta = if var_cv.abs() < 1e-15 {
            0.0
        } else {
            cov / var_cv
        };
        let corrected_sum: f64 = fv
            .iter()
            .zip(cv_v.iter())
            .map(|(&fi, &ci)| fi - beta * (ci - cv_integral / (b - a)))
            .sum::<f64>();
        (b - a) * corrected_sum / n as f64
    }
}
