//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
/// Jacobi (diagonal) preconditioner: M = diag(A).
#[derive(Debug, Clone)]
pub struct JacobiPreconditioner {
    /// Reciprocals of diagonal entries.
    pub inv_diag: Vec<f64>,
}
impl JacobiPreconditioner {
    /// Construct from matrix A.
    pub fn new(a: &[Vec<f64>]) -> Self {
        let inv_diag = a
            .iter()
            .enumerate()
            .map(|(i, row)| {
                let d = row[i];
                if d.abs() > 1e-14 {
                    1.0 / d
                } else {
                    1.0
                }
            })
            .collect();
        JacobiPreconditioner { inv_diag }
    }
    /// Apply M⁻¹ x in-place (diagonal scaling).
    pub fn apply(&self, x: &[f64]) -> Vec<f64> {
        x.iter()
            .zip(&self.inv_diag)
            .map(|(xi, di)| xi * di)
            .collect()
    }
}
/// Sparse matrix in Compressed Sparse Row (CSR) format.
#[derive(Debug, Clone)]
pub struct CsrMatrix {
    /// Number of rows.
    pub nrows: usize,
    /// Number of columns.
    pub ncols: usize,
    /// Row pointer array (`row_ptr\[i\]..row_ptr[i+1]` gives the range of entries for row i).
    pub row_ptr: Vec<usize>,
    /// Column indices (length nnz).
    pub col_idx: Vec<usize>,
    /// Values (length nnz).
    pub val: Vec<f64>,
}
impl CsrMatrix {
    /// Convert from COO format to CSR.
    pub fn from_coo(coo: &CooMatrix) -> Self {
        let nrows = coo.nrows;
        let ncols = coo.ncols;
        let nnz = coo.nnz();
        let mut row_count = vec![0usize; nrows + 1];
        for &r in &coo.row {
            row_count[r + 1] += 1;
        }
        for i in 0..nrows {
            row_count[i + 1] += row_count[i];
        }
        let mut col_idx = vec![0usize; nnz];
        let mut val = vec![0.0f64; nnz];
        let mut pos = row_count[..nrows].to_vec();
        for k in 0..nnz {
            let r = coo.row[k];
            let idx = pos[r];
            col_idx[idx] = coo.col[k];
            val[idx] = coo.val[k];
            pos[r] += 1;
        }
        CsrMatrix {
            nrows,
            ncols,
            row_ptr: row_count,
            col_idx,
            val,
        }
    }
    /// Sparse matrix-vector product y = Ax.
    pub fn matvec(&self, x: &[f64]) -> Vec<f64> {
        let mut y = vec![0.0; self.nrows];
        for i in 0..self.nrows {
            let start = self.row_ptr[i];
            let end = self.row_ptr[i + 1];
            for k in start..end {
                y[i] += self.val[k] * x[self.col_idx[k]];
            }
        }
        y
    }
    /// Number of nonzeros.
    pub fn nnz(&self) -> usize {
        self.val.len()
    }
}
/// Result of the Arnoldi process.
#[derive(Debug, Clone)]
pub struct KrylovSubspaceResult {
    /// Orthonormal Krylov basis vectors V (each of length n); V has k columns.
    pub v_basis: Vec<Vec<f64>>,
    /// Upper Hessenberg matrix H, stored as (k+1)×k.
    pub h: Vec<Vec<f64>>,
    /// Number of steps actually taken (may be < k on breakdown).
    pub steps: usize,
}
/// Generalized Minimum Residual (GMRES) for general linear systems.
/// Simplified version with restart parameter m.
#[allow(dead_code)]
pub struct GmresSolver {
    /// Restart parameter (Krylov subspace size).
    pub restart: usize,
    /// Maximum outer iterations.
    pub max_outer: usize,
    /// Convergence tolerance.
    pub tol: f64,
}
#[allow(dead_code)]
impl GmresSolver {
    /// Create a new GMRES solver.
    pub fn new(restart: usize, max_outer: usize, tol: f64) -> Self {
        GmresSolver {
            restart,
            max_outer,
            tol,
        }
    }
    /// Arnoldi process: orthonormal Krylov basis via modified Gram-Schmidt.
    /// Returns (V, H) where V has m+1 columns and H is (m+1) x m upper Hessenberg.
    pub fn arnoldi_basis(&self, a: &[Vec<f64>], b: &[f64]) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
        let n = b.len();
        let m = self.restart.min(n);
        let norm_b = b.iter().map(|&x| x * x).sum::<f64>().sqrt();
        if norm_b < 1e-15 {
            return (vec![vec![0.0; n]; m + 1], vec![vec![0.0; m]; m + 1]);
        }
        let mut v: Vec<Vec<f64>> = vec![vec![0.0; n]; m + 1];
        let mut h: Vec<Vec<f64>> = vec![vec![0.0; m]; m + 1];
        v[0] = b.iter().map(|&bi| bi / norm_b).collect();
        for j in 0..m {
            let mut w: Vec<f64> = vec![0.0; n];
            for i in 0..n {
                w[i] = a[i]
                    .iter()
                    .zip(v[j].iter())
                    .map(|(&aij, &vji)| aij * vji)
                    .sum();
            }
            for i in 0..=j {
                h[i][j] = v[i]
                    .iter()
                    .zip(w.iter())
                    .map(|(&vi, &wi)| vi * wi)
                    .sum::<f64>();
                for k in 0..n {
                    w[k] -= h[i][j] * v[i][k];
                }
            }
            let norm_w = w.iter().map(|&x| x * x).sum::<f64>().sqrt();
            h[j + 1][j] = norm_w;
            if norm_w > 1e-15 {
                v[j + 1] = w.iter().map(|&x| x / norm_w).collect();
            }
        }
        (v, h)
    }
    /// Returns the condition number estimate for the Hessenberg matrix.
    pub fn condition_estimate(&self, h: &[Vec<f64>]) -> f64 {
        let m = h.len().saturating_sub(1);
        if m == 0 {
            return 1.0;
        }
        let diag_vals: Vec<f64> = (0..m).map(|i| h[i][i].abs()).collect();
        let max_d = diag_vals.iter().cloned().fold(0.0_f64, f64::max);
        let min_d = diag_vals.iter().cloned().fold(f64::INFINITY, f64::min);
        if min_d < 1e-15 {
            f64::INFINITY
        } else {
            max_d / min_d
        }
    }
}
/// Configuration for the GMRES solver.
#[derive(Debug, Clone)]
pub struct GMRESSolver {
    /// Restart dimension m (Krylov subspace size per restart cycle).
    pub restart_dim: usize,
    /// Relative residual tolerance.
    pub tol: f64,
    /// Maximum number of restart cycles.
    pub max_restarts: usize,
}
impl GMRESSolver {
    /// Create a new GMRES solver configuration.
    pub fn new(restart_dim: usize, tol: f64, max_restarts: usize) -> Self {
        GMRESSolver {
            restart_dim,
            tol,
            max_restarts,
        }
    }
    /// Solve Ax = b starting from x = 0.
    pub fn solve(&self, a: &[Vec<f64>], b: &[f64]) -> GMRESSolution {
        let x0 = vec![0.0; b.len()];
        let b_norm = norm2(b);
        let (x, matvec_count, res_norm) =
            gmres(a, b, &x0, self.restart_dim, self.tol, self.max_restarts);
        let rel_residual = if b_norm > 1e-300 {
            res_norm / b_norm
        } else {
            res_norm
        };
        GMRESSolution {
            x,
            matvec_count,
            rel_residual,
            converged: rel_residual < self.tol,
        }
    }
}
/// Solution result from GMRESSolver.
#[derive(Debug, Clone)]
pub struct GMRESSolution {
    /// Approximate solution vector.
    pub x: Vec<f64>,
    /// Total number of matrix-vector products performed.
    pub matvec_count: usize,
    /// Final relative residual norm.
    pub rel_residual: f64,
    /// Whether the solver converged to tolerance.
    pub converged: bool,
}
/// Result of the randomized SVD.
#[derive(Debug, Clone)]
pub struct RandomizedSVDResult {
    /// Left singular vectors U (m × k).
    pub u: Vec<Vec<f64>>,
    /// Singular values (length k), descending.
    pub sigma: Vec<f64>,
    /// Right singular vectors V^T (k × n).
    pub vt: Vec<Vec<f64>>,
}
/// QR algorithm (unshifted) for computing all eigenvalues of A.
/// Simplified: iterate A = QR, then A = RQ.
#[allow(dead_code)]
pub struct QRAlgorithm {
    /// Maximum iterations.
    pub max_iter: usize,
    /// Tolerance for convergence (subdiagonal elements).
    pub tol: f64,
}
#[allow(dead_code)]
impl QRAlgorithm {
    /// Create a new QR algorithm solver.
    pub fn new(max_iter: usize, tol: f64) -> Self {
        QRAlgorithm { max_iter, tol }
    }
    /// Gram-Schmidt QR decomposition of A.
    /// Returns (Q, R) where A = QR.
    pub fn qr_decompose(a: &[Vec<f64>]) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
        let n = a.len();
        let mut q = vec![vec![0.0f64; n]; n];
        let mut r = vec![vec![0.0f64; n]; n];
        let a_cols: Vec<Vec<f64>> = (0..n)
            .map(|j| a.iter().map(|row| row[j]).collect())
            .collect();
        let mut q_cols: Vec<Vec<f64>> = vec![];
        for j in 0..n {
            let mut u = a_cols[j].clone();
            for k in 0..j {
                let rk_j: f64 = a_cols[j]
                    .iter()
                    .zip(q_cols[k].iter())
                    .map(|(&a, &q)| a * q)
                    .sum();
                r[k][j] = rk_j;
                for i in 0..n {
                    u[i] -= rk_j * q_cols[k][i];
                }
            }
            let norm_u: f64 = u.iter().map(|&x| x * x).sum::<f64>().sqrt();
            r[j][j] = norm_u;
            let q_j = if norm_u > 1e-15 {
                u.iter().map(|&x| x / norm_u).collect()
            } else {
                u
            };
            q_cols.push(q_j.clone());
        }
        for (j, col) in q_cols.iter().enumerate() {
            for i in 0..n {
                q[i][j] = col[i];
            }
        }
        (q, r)
    }
    /// Run QR iteration to find eigenvalues. Returns diagonal of final matrix.
    pub fn run(&self, a: &[Vec<f64>]) -> Vec<f64> {
        let n = a.len();
        let mut ak = a.to_vec();
        for _iter in 0..self.max_iter {
            let (q, r) = Self::qr_decompose(&ak);
            let mut ak_new = vec![vec![0.0; n]; n];
            for i in 0..n {
                for j in 0..n {
                    ak_new[i][j] = (0..n).map(|k| r[i][k] * q[k][j]).sum();
                }
            }
            let off_diag: f64 = (1..n).map(|i| ak_new[i][i - 1].abs()).fold(0.0, f64::max);
            if off_diag < self.tol {
                return (0..n).map(|i| ak_new[i][i]).collect();
            }
            ak = ak_new;
        }
        (0..n).map(|i| ak[i][i]).collect()
    }
}
/// Sparse matrix in coordinate (triplet) format.
#[derive(Debug, Clone)]
pub struct CooMatrix {
    /// Number of rows.
    pub nrows: usize,
    /// Number of columns.
    pub ncols: usize,
    /// Row indices.
    pub row: Vec<usize>,
    /// Column indices.
    pub col: Vec<usize>,
    /// Values.
    pub val: Vec<f64>,
}
impl CooMatrix {
    /// Create an empty sparse matrix.
    pub fn new(nrows: usize, ncols: usize) -> Self {
        CooMatrix {
            nrows,
            ncols,
            row: vec![],
            col: vec![],
            val: vec![],
        }
    }
    /// Push a triplet (i, j, v).
    pub fn push(&mut self, i: usize, j: usize, v: f64) {
        self.row.push(i);
        self.col.push(j);
        self.val.push(v);
    }
    /// Number of stored entries.
    pub fn nnz(&self) -> usize {
        self.val.len()
    }
    /// Sparse density = nnz / (nrows * ncols).
    pub fn density(&self) -> f64 {
        let total = self.nrows * self.ncols;
        if total == 0 {
            0.0
        } else {
            self.nnz() as f64 / total as f64
        }
    }
}
/// Result of LU decomposition with partial pivoting.
///
/// Stores L (lower triangular, unit diagonal), U (upper triangular),
/// and the permutation vector `piv` such that piv\[i\] = j means row i
/// was swapped with row j during factorisation.
#[derive(Debug, Clone)]
pub struct LUResult {
    /// Lower triangular factor (unit diagonal).
    pub l: Vec<Vec<f64>>,
    /// Upper triangular factor.
    pub u: Vec<Vec<f64>>,
    /// Permutation vector.
    pub piv: Vec<usize>,
}
/// Circulant matrix representation.
///
/// A circulant matrix C of size n is fully determined by its first row `c`.
/// Each subsequent row is a cyclic right-shift of the previous one.
/// Multiplication by a vector can be performed in O(n log n) using the DFT.
#[derive(Debug, Clone)]
pub struct CirculantMatrixFFT {
    /// First row of the circulant matrix.
    pub c: Vec<f64>,
    /// Size n of the matrix.
    pub n: usize,
}
impl CirculantMatrixFFT {
    /// Construct a circulant matrix from its first row.
    pub fn new(first_row: Vec<f64>) -> Self {
        let n = first_row.len();
        CirculantMatrixFFT { c: first_row, n }
    }
    /// Naive O(n²) multiplication C x (serves as reference / fallback).
    ///
    /// For production use one would replace this with an FFT-based O(n log n)
    /// implementation. The naive version is provided to keep this crate
    /// dependency-free while still exercising the data structure.
    pub fn matvec_naive(&self, x: &[f64]) -> Vec<f64> {
        let n = self.n;
        let mut y = vec![0.0; n];
        for i in 0..n {
            for j in 0..n {
                let idx = (i + n - j) % n;
                y[i] += self.c[idx] * x[j];
            }
        }
        y
    }
    /// Compute the DFT of `c` (unnormalized) for use in eigenvalue analysis.
    ///
    /// The eigenvalues of a circulant matrix are exactly the DFT of its
    /// first row: λ_k = Σ_j c\[j\] ω^{jk}, ω = e^{-2πi/n}.
    ///
    /// Returns the real and imaginary parts as separate vectors.
    pub fn dft_eigenvalues(&self) -> (Vec<f64>, Vec<f64>) {
        use std::f64::consts::PI;
        let n = self.n;
        let mut re = vec![0.0; n];
        let mut im = vec![0.0; n];
        for k in 0..n {
            for j in 0..n {
                let angle = -2.0 * PI * (k * j) as f64 / n as f64;
                re[k] += self.c[j] * angle.cos();
                im[k] += self.c[j] * angle.sin();
            }
        }
        (re, im)
    }
    /// Matrix-vector product using eigenvalue decomposition (via DFT).
    ///
    /// C x = F⁻¹ (diag(λ) · F x) where F is the DFT matrix.
    /// Uses O(n²) naive DFT — replace with FFT library for O(n log n).
    pub fn matvec(&self, x: &[f64]) -> Vec<f64> {
        use std::f64::consts::PI;
        let n = self.n;
        let mut xr = vec![0.0; n];
        let mut xi = vec![0.0; n];
        for k in 0..n {
            for j in 0..n {
                let angle = -2.0 * PI * (k * j) as f64 / n as f64;
                xr[k] += x[j] * angle.cos();
                xi[k] += x[j] * angle.sin();
            }
        }
        let (lr, li) = self.dft_eigenvalues();
        let yr: Vec<f64> = (0..n).map(|k| lr[k] * xr[k] - li[k] * xi[k]).collect();
        let yi: Vec<f64> = (0..n).map(|k| lr[k] * xi[k] + li[k] * xr[k]).collect();
        let mut y = vec![0.0; n];
        for j in 0..n {
            for k in 0..n {
                let angle = 2.0 * PI * (k * j) as f64 / n as f64;
                y[j] += yr[k] * angle.cos() - yi[k] * angle.sin();
            }
            y[j] /= n as f64;
        }
        y
    }
}
/// Result of QR decomposition.
#[derive(Debug, Clone)]
pub struct QRResult {
    /// Orthogonal factor Q (m×m).
    pub q: Vec<Vec<f64>>,
    /// Upper triangular factor R (m×n).
    pub r: Vec<Vec<f64>>,
}
/// Power iteration for computing the dominant eigenvalue and eigenvector.
#[allow(dead_code)]
pub struct PowerIteration {
    /// Maximum iterations.
    pub max_iter: usize,
    /// Convergence tolerance.
    pub tol: f64,
}
#[allow(dead_code)]
impl PowerIteration {
    /// Create a new power iteration instance.
    pub fn new(max_iter: usize, tol: f64) -> Self {
        PowerIteration { max_iter, tol }
    }
    /// Run power iteration on A. Returns (eigenvalue, eigenvector, iterations).
    pub fn run(&self, a: &[Vec<f64>]) -> (f64, Vec<f64>, usize) {
        let n = a.len();
        if n == 0 {
            return (0.0, vec![], 0);
        }
        let mut v: Vec<f64> = vec![1.0 / (n as f64).sqrt(); n];
        let mut eigenvalue = 0.0_f64;
        for iter in 0..self.max_iter {
            let mut w: Vec<f64> = vec![0.0; n];
            for i in 0..n {
                w[i] = a[i].iter().zip(v.iter()).map(|(&aij, &vj)| aij * vj).sum();
            }
            let norm_w = w.iter().map(|&x| x * x).sum::<f64>().sqrt();
            let new_eigenvalue = w
                .iter()
                .zip(v.iter())
                .map(|(&wi, &vi)| wi * vi)
                .sum::<f64>();
            if (new_eigenvalue - eigenvalue).abs() < self.tol {
                return (new_eigenvalue, v, iter + 1);
            }
            eigenvalue = new_eigenvalue;
            if norm_w > 1e-15 {
                v = w.iter().map(|&x| x / norm_w).collect();
            }
        }
        (eigenvalue, v, self.max_iter)
    }
    /// Inverse iteration for smallest eigenvalue (with shift σ=0).
    /// Requires solving (A - σI) w = v at each step.
    pub fn inverse_iteration_count(&self) -> usize {
        (1.0_f64 / self.tol).log2().ceil() as usize
    }
}
/// Conjugate Gradient (CG) method for symmetric positive definite systems Ax = b.
#[allow(dead_code)]
pub struct ConjugateGradient {
    /// Maximum number of iterations.
    pub max_iter: usize,
    /// Convergence tolerance.
    pub tol: f64,
}
#[allow(dead_code)]
impl ConjugateGradient {
    /// Create a new CG solver.
    pub fn new(max_iter: usize, tol: f64) -> Self {
        ConjugateGradient { max_iter, tol }
    }
    /// Dot product of two vectors.
    fn dot(a: &[f64], b: &[f64]) -> f64 {
        a.iter().zip(b.iter()).map(|(&ai, &bi)| ai * bi).sum()
    }
    /// Matrix-vector product y = A x for dense A (row-major).
    fn matvec(a: &[Vec<f64>], x: &[f64]) -> Vec<f64> {
        a.iter()
            .map(|row| row.iter().zip(x.iter()).map(|(&aij, &xj)| aij * xj).sum())
            .collect()
    }
    /// Solve Ax = b using CG, starting from x = b.
    /// A must be symmetric positive definite.
    /// Returns (solution, residual_norm, iterations).
    pub fn solve(&self, a: &[Vec<f64>], b: &[f64]) -> (Vec<f64>, f64, usize) {
        let n = b.len();
        let mut x = b.to_vec();
        let mut r = {
            let ax = Self::matvec(a, &x);
            b.iter()
                .zip(ax.iter())
                .map(|(&bi, &axi)| bi - axi)
                .collect::<Vec<f64>>()
        };
        let mut p = r.clone();
        let mut r_dot = Self::dot(&r, &r);
        for iter in 0..self.max_iter {
            if r_dot.sqrt() < self.tol {
                return (x, r_dot.sqrt(), iter);
            }
            let ap = Self::matvec(a, &p);
            let pap = Self::dot(&p, &ap);
            if pap.abs() < 1e-15 {
                break;
            }
            let alpha = r_dot / pap;
            for i in 0..n {
                x[i] += alpha * p[i];
                r[i] -= alpha * ap[i];
            }
            let r_dot_new = Self::dot(&r, &r);
            let beta = r_dot_new / r_dot;
            for i in 0..n {
                p[i] = r[i] + beta * p[i];
            }
            r_dot = r_dot_new;
        }
        (x, r_dot.sqrt(), self.max_iter)
    }
    /// Preconditioned CG with Jacobi (diagonal) preconditioner.
    pub fn solve_jacobi_precond(&self, a: &[Vec<f64>], b: &[f64]) -> (Vec<f64>, f64, usize) {
        let n = b.len();
        let m_inv: Vec<f64> = (0..n)
            .map(|i| {
                if a[i][i].abs() > 1e-15 {
                    1.0 / a[i][i]
                } else {
                    1.0
                }
            })
            .collect();
        let b_precond: Vec<f64> = b
            .iter()
            .zip(m_inv.iter())
            .map(|(&bi, &mi)| bi * mi)
            .collect();
        let a_precond: Vec<Vec<f64>> = a
            .iter()
            .enumerate()
            .map(|(i, row)| row.iter().map(|&aij| aij * m_inv[i]).collect())
            .collect();
        self.solve(&a_precond, &b_precond)
    }
}
/// Result of the QR algorithm eigenvalue solver.
#[derive(Debug, Clone)]
pub struct QRAlgorithmResult {
    /// Approximate eigenvalues, sorted ascending.
    pub eigenvalues: Vec<f64>,
    /// Number of QR sweeps performed.
    pub iterations: usize,
    /// Final off-diagonal Frobenius norm (convergence measure).
    pub off_diag_norm: f64,
}
