//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// A Rust-level dense vector.
#[derive(Clone, Debug)]
pub struct DenseVector {
    /// Vector entries.
    pub data: Vec<f64>,
}
impl DenseVector {
    /// Create a zero vector.
    pub fn zero(n: usize) -> Self {
        Self { data: vec![0.0; n] }
    }
    /// Dimension.
    pub fn dim(&self) -> usize {
        self.data.len()
    }
    /// Dot product.
    pub fn dot(&self, other: &DenseVector) -> f64 {
        self.data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a * b)
            .sum()
    }
    /// Norm squared.
    pub fn norm_sq(&self) -> f64 {
        self.dot(self)
    }
    /// Norm.
    pub fn norm(&self) -> f64 {
        self.norm_sq().sqrt()
    }
    /// Normalize (returns None if zero vector).
    pub fn normalize(&self) -> Option<DenseVector> {
        let n = self.norm();
        if n < 1e-12 {
            return None;
        }
        Some(DenseVector {
            data: self.data.iter().map(|x| x / n).collect(),
        })
    }
    /// Add two vectors.
    pub fn add(&self, other: &DenseVector) -> Option<DenseVector> {
        if self.dim() != other.dim() {
            return None;
        }
        Some(DenseVector {
            data: self
                .data
                .iter()
                .zip(other.data.iter())
                .map(|(a, b)| a + b)
                .collect(),
        })
    }
    /// Scale by a scalar.
    pub fn scale(&self, s: f64) -> DenseVector {
        DenseVector {
            data: self.data.iter().map(|x| x * s).collect(),
        }
    }
    /// Apply matrix to vector (A·v).
    pub fn apply(mat: &DenseMatrix, v: &DenseVector) -> Option<DenseVector> {
        if mat.cols != v.dim() {
            return None;
        }
        let mut result = vec![0.0; mat.rows];
        for i in 0..mat.rows {
            for j in 0..mat.cols {
                result[i] += mat.data[i][j] * v.data[j];
            }
        }
        Some(DenseVector { data: result })
    }
}
/// A sparse matrix stored in CSR (Compressed Sparse Row) format.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SparseMatrix {
    /// Number of rows.
    pub rows: usize,
    /// Number of columns.
    pub cols: usize,
    /// Row pointers: row i has non-zeros at col_indices[row_ptr\[i\]..row_ptr[i+1]].
    pub row_ptr: Vec<usize>,
    /// Column indices of non-zero entries.
    pub col_indices: Vec<usize>,
    /// Values of non-zero entries.
    pub values: Vec<f64>,
}
#[allow(dead_code)]
impl SparseMatrix {
    /// Create an empty sparse matrix.
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            row_ptr: vec![0; rows + 1],
            col_indices: vec![],
            values: vec![],
        }
    }
    /// Build from coordinate list (row, col, val). Entries must be sorted by row.
    pub fn from_coo(rows: usize, cols: usize, entries: &[(usize, usize, f64)]) -> Self {
        let mut row_ptr = vec![0usize; rows + 1];
        for &(r, _, _) in entries {
            row_ptr[r + 1] += 1;
        }
        for i in 0..rows {
            row_ptr[i + 1] += row_ptr[i];
        }
        let col_indices: Vec<usize> = entries.iter().map(|&(_, c, _)| c).collect();
        let values: Vec<f64> = entries.iter().map(|&(_, _, v)| v).collect();
        Self {
            rows,
            cols,
            row_ptr,
            col_indices,
            values,
        }
    }
    /// Number of non-zero entries.
    pub fn nnz(&self) -> usize {
        self.values.len()
    }
    /// Get element (O(nnz per row) lookup).
    pub fn get(&self, r: usize, c: usize) -> f64 {
        let start = self.row_ptr[r];
        let end = self.row_ptr[r + 1];
        for idx in start..end {
            if self.col_indices[idx] == c {
                return self.values[idx];
            }
        }
        0.0
    }
    /// Sparse matrix-vector multiply: y = A * x.
    pub fn matvec(&self, x: &[f64]) -> Option<Vec<f64>> {
        if x.len() != self.cols {
            return None;
        }
        let mut y = vec![0.0f64; self.rows];
        for r in 0..self.rows {
            let start = self.row_ptr[r];
            let end = self.row_ptr[r + 1];
            for idx in start..end {
                y[r] += self.values[idx] * x[self.col_indices[idx]];
            }
        }
        Some(y)
    }
    /// Transpose of the sparse matrix.
    pub fn transpose(&self) -> SparseMatrix {
        let mut entries: Vec<(usize, usize, f64)> = Vec::with_capacity(self.nnz());
        for r in 0..self.rows {
            let start = self.row_ptr[r];
            let end = self.row_ptr[r + 1];
            for idx in start..end {
                entries.push((self.col_indices[idx], r, self.values[idx]));
            }
        }
        entries.sort_by_key(|&(r, c, _)| (r, c));
        SparseMatrix::from_coo(self.cols, self.rows, &entries)
    }
    /// Frobenius norm squared.
    pub fn norm_sq(&self) -> f64 {
        self.values.iter().map(|v| v * v).sum()
    }
}
/// Householder QR decomposition for dense matrices.
/// Produces thin Q (m×n) and upper-triangular R (n×n) for m ≥ n.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct QRDecomposition {
    /// The Q factor (m × n orthonormal columns).
    pub q: DenseMatrix,
    /// The R factor (n × n upper-triangular).
    pub r: DenseMatrix,
}
#[allow(dead_code)]
impl QRDecomposition {
    /// Compute QR via Householder reflections.
    pub fn compute(a: &DenseMatrix) -> Option<QRDecomposition> {
        let m = a.rows;
        let n = a.cols;
        if m < n {
            return None;
        }
        let mut mat = a.data.clone();
        let mut vs: Vec<Vec<f64>> = Vec::new();
        for k in 0..n {
            let mut x: Vec<f64> = (k..m).map(|i| mat[i][k]).collect();
            let norm: f64 = x.iter().map(|xi| xi * xi).sum::<f64>().sqrt();
            if norm < 1e-14 {
                vs.push(vec![0.0; m - k]);
                continue;
            }
            x[0] += if x[0] >= 0.0 { norm } else { -norm };
            let v_norm: f64 = x.iter().map(|xi| xi * xi).sum::<f64>().sqrt();
            let v: Vec<f64> = x.iter().map(|xi| xi / v_norm).collect();
            for j in k..n {
                let dot: f64 = v.iter().enumerate().map(|(i, vi)| vi * mat[k + i][j]).sum();
                for i in 0..(m - k) {
                    mat[k + i][j] -= 2.0 * v[i] * dot;
                }
            }
            vs.push(v);
        }
        let mut r_data = vec![vec![0.0; n]; n];
        for i in 0..n {
            for j in i..n {
                r_data[i][j] = mat[i][j];
            }
        }
        let r = DenseMatrix {
            rows: n,
            cols: n,
            data: r_data,
        };
        let mut q_data = vec![vec![0.0; n]; m];
        for j in 0..n {
            q_data[j][j] = 1.0;
        }
        for k in (0..n).rev() {
            let v = &vs[k];
            let len = v.len();
            for j in 0..n {
                let dot: f64 = v
                    .iter()
                    .enumerate()
                    .map(|(i, vi)| vi * q_data[k + i][j])
                    .sum();
                for i in 0..len {
                    q_data[k + i][j] -= 2.0 * v[i] * dot;
                }
            }
        }
        let q = DenseMatrix {
            rows: m,
            cols: n,
            data: q_data,
        };
        Some(QRDecomposition { q, r })
    }
    /// Solve A·x = b using the QR factorization (least squares).
    pub fn solve(&self, b: &[f64]) -> Option<Vec<f64>> {
        let m = self.q.rows;
        let n = self.q.cols;
        if b.len() != m {
            return None;
        }
        let mut qtb = vec![0.0f64; n];
        for j in 0..n {
            qtb[j] = (0..m).map(|i| self.q.data[i][j] * b[i]).sum();
        }
        let mut x = vec![0.0f64; n];
        for i in (0..n).rev() {
            let mut s = qtb[i];
            for j in (i + 1)..n {
                s -= self.r.data[i][j] * x[j];
            }
            if self.r.data[i][i].abs() < 1e-14 {
                return None;
            }
            x[i] = s / self.r.data[i][i];
        }
        Some(x)
    }
}
/// A Rust-level dense matrix over f64 for computational purposes.
#[derive(Clone, Debug)]
pub struct DenseMatrix {
    /// Number of rows.
    pub rows: usize,
    /// Number of columns.
    pub cols: usize,
    /// Row-major storage of matrix entries.
    pub data: Vec<Vec<f64>>,
}
impl DenseMatrix {
    /// Create a new zero matrix.
    pub fn zero(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![vec![0.0; cols]; rows],
        }
    }
    /// Create the identity matrix.
    pub fn identity(n: usize) -> Self {
        let mut m = Self::zero(n, n);
        for i in 0..n {
            m.data[i][i] = 1.0;
        }
        m
    }
    /// Get element.
    pub fn get(&self, r: usize, c: usize) -> f64 {
        self.data[r][c]
    }
    /// Set element.
    pub fn set(&mut self, r: usize, c: usize, v: f64) {
        self.data[r][c] = v;
    }
    /// Matrix addition.
    pub fn add(&self, other: &DenseMatrix) -> Option<DenseMatrix> {
        if self.rows != other.rows || self.cols != other.cols {
            return None;
        }
        let mut result = Self::zero(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        Some(result)
    }
    /// Matrix multiplication.
    pub fn mul(&self, other: &DenseMatrix) -> Option<DenseMatrix> {
        if self.cols != other.rows {
            return None;
        }
        let mut result = Self::zero(self.rows, other.cols);
        for i in 0..self.rows {
            for k in 0..self.cols {
                for j in 0..other.cols {
                    result.data[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }
        Some(result)
    }
    /// Scalar multiplication.
    pub fn scale(&self, s: f64) -> DenseMatrix {
        let mut result = self.clone();
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[i][j] *= s;
            }
        }
        result
    }
    /// Transpose.
    pub fn transpose(&self) -> DenseMatrix {
        let mut result = Self::zero(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[j][i] = self.data[i][j];
            }
        }
        result
    }
    /// Trace (sum of diagonal elements).
    pub fn trace(&self) -> f64 {
        let n = self.rows.min(self.cols);
        (0..n).map(|i| self.data[i][i]).sum()
    }
    /// Frobenius norm squared.
    pub fn norm_sq(&self) -> f64 {
        self.data
            .iter()
            .flat_map(|row| row.iter())
            .map(|x| x * x)
            .sum()
    }
    /// Determinant via Gaussian elimination (for small matrices).
    pub fn det(&self) -> Option<f64> {
        if self.rows != self.cols {
            return None;
        }
        let n = self.rows;
        if n == 0 {
            return Some(1.0);
        }
        if n == 1 {
            return Some(self.data[0][0]);
        }
        if n == 2 {
            return Some(self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0]);
        }
        let mut m = self.data.clone();
        let mut sign = 1.0_f64;
        for col in 0..n {
            let mut pivot_row = col;
            let mut max_val = m[col][col].abs();
            for row in (col + 1)..n {
                if m[row][col].abs() > max_val {
                    max_val = m[row][col].abs();
                    pivot_row = row;
                }
            }
            if max_val < 1e-12 {
                return Some(0.0);
            }
            if pivot_row != col {
                m.swap(col, pivot_row);
                sign = -sign;
            }
            let pivot = m[col][col];
            for row in (col + 1)..n {
                let factor = m[row][col] / pivot;
                for k in col..n {
                    let v = m[col][k] * factor;
                    m[row][k] -= v;
                }
            }
        }
        let diag_prod: f64 = (0..n).map(|i| m[i][i]).product();
        Some(sign * diag_prod)
    }
    /// Gaussian elimination: returns (row echelon form, rank).
    pub fn row_echelon(&self) -> (DenseMatrix, usize) {
        let mut m = self.data.clone();
        let mut rank = 0;
        let mut pivot_col = 0;
        while rank < self.rows && pivot_col < self.cols {
            let mut pivot_row = rank;
            while pivot_row < self.rows && m[pivot_row][pivot_col].abs() < 1e-12 {
                pivot_row += 1;
            }
            if pivot_row == self.rows {
                pivot_col += 1;
                continue;
            }
            if pivot_row != rank {
                m.swap(rank, pivot_row);
            }
            let pivot = m[rank][pivot_col];
            for j in pivot_col..self.cols {
                m[rank][j] /= pivot;
            }
            for i in 0..self.rows {
                if i != rank && m[i][pivot_col].abs() > 1e-12 {
                    let factor = m[i][pivot_col];
                    for j in pivot_col..self.cols {
                        let v = m[rank][j] * factor;
                        m[i][j] -= v;
                    }
                }
            }
            rank += 1;
            pivot_col += 1;
        }
        let result = DenseMatrix {
            rows: self.rows,
            cols: self.cols,
            data: m,
        };
        (result, rank)
    }
    /// Compute rank.
    pub fn rank(&self) -> usize {
        self.row_echelon().1
    }
    /// Check if symmetric.
    pub fn is_symmetric(&self) -> bool {
        if self.rows != self.cols {
            return false;
        }
        for i in 0..self.rows {
            for j in 0..self.cols {
                if (self.data[i][j] - self.data[j][i]).abs() > 1e-12 {
                    return false;
                }
            }
        }
        true
    }
    /// Solve A·x = b via Gaussian elimination (returns None if singular).
    pub fn solve(&self, b: &[f64]) -> Option<Vec<f64>> {
        if self.rows != self.cols || self.rows != b.len() {
            return None;
        }
        let n = self.rows;
        let mut aug: Vec<Vec<f64>> = self
            .data
            .iter()
            .enumerate()
            .map(|(i, row)| {
                let mut r = row.clone();
                r.push(b[i]);
                r
            })
            .collect();
        for col in 0..n {
            let mut pivot_row = col;
            let mut max_val = aug[col][col].abs();
            for row in (col + 1)..n {
                if aug[row][col].abs() > max_val {
                    max_val = aug[row][col].abs();
                    pivot_row = row;
                }
            }
            if max_val < 1e-12 {
                return None;
            }
            if pivot_row != col {
                aug.swap(col, pivot_row);
            }
            let pivot = aug[col][col];
            for j in col..=n {
                aug[col][j] /= pivot;
            }
            for row in 0..n {
                if row != col && aug[row][col].abs() > 1e-12 {
                    let factor = aug[row][col];
                    for j in col..=n {
                        let v = aug[col][j] * factor;
                        aug[row][j] -= v;
                    }
                }
            }
        }
        Some((0..n).map(|i| aug[i][n]).collect())
    }
}
/// A banded matrix stored as an array of diagonals.
/// Diagonals are indexed from -(rows-1) to +(cols-1); offset = k means superdiagonal k.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct BandedMatrix {
    /// Number of rows.
    pub rows: usize,
    /// Number of columns.
    pub cols: usize,
    /// Lower bandwidth (number of sub-diagonals).
    pub kl: usize,
    /// Upper bandwidth (number of super-diagonals).
    pub ku: usize,
    /// Storage: (kl + ku + 1) × cols, band\[k\]\[j\] = A\[j - ku + k\]\[j\].
    pub band: Vec<Vec<f64>>,
}
#[allow(dead_code)]
impl BandedMatrix {
    /// Create a zero banded matrix.
    pub fn zero(rows: usize, cols: usize, kl: usize, ku: usize) -> Self {
        let ndiag = kl + ku + 1;
        Self {
            rows,
            cols,
            kl,
            ku,
            band: vec![vec![0.0; cols]; ndiag],
        }
    }
    /// Get A\[r\]\[c\] (returns 0 if outside band).
    pub fn get(&self, r: usize, c: usize) -> f64 {
        let offset = c as isize - r as isize;
        if offset < -(self.kl as isize) || offset > self.ku as isize {
            return 0.0;
        }
        let k = (offset + self.kl as isize) as usize;
        self.band[k][c]
    }
    /// Set A\[r\]\[c\] (panics if outside band).
    pub fn set(&mut self, r: usize, c: usize, v: f64) {
        let offset = c as isize - r as isize;
        assert!(
            offset >= -(self.kl as isize) && offset <= self.ku as isize,
            "Index outside band"
        );
        let k = (offset + self.kl as isize) as usize;
        self.band[k][c] = v;
    }
    /// Matrix-vector multiply: y = A * x.
    pub fn matvec(&self, x: &[f64]) -> Option<Vec<f64>> {
        if x.len() != self.cols {
            return None;
        }
        let mut y = vec![0.0f64; self.rows];
        for r in 0..self.rows {
            let c_start = r.saturating_sub(self.kl);
            let c_end = (r + self.ku + 1).min(self.cols);
            for c in c_start..c_end {
                y[r] += self.get(r, c) * x[c];
            }
        }
        Some(y)
    }
    /// Diagonal entries (min(rows, cols) elements).
    pub fn diagonal(&self) -> Vec<f64> {
        let n = self.rows.min(self.cols);
        (0..n).map(|i| self.get(i, i)).collect()
    }
}
/// Lanczos iteration for symmetric matrices: computes a tridiagonal projection.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct LanczosResult {
    /// Orthonormal Lanczos vectors (columns of V), stored row-major as `Vec<DenseVector>`.
    pub basis: Vec<DenseVector>,
    /// Diagonal entries of the tridiagonal matrix T.
    pub alpha: Vec<f64>,
    /// Off-diagonal entries of T (length = alpha.len() - 1).
    pub beta: Vec<f64>,
}
#[allow(dead_code)]
impl LanczosResult {
    /// Run k steps of the Lanczos algorithm on symmetric matrix A, starting from v0.
    pub fn compute(a: &DenseMatrix, v0: &DenseVector, k: usize) -> Option<LanczosResult> {
        if a.rows != a.cols || a.rows != v0.dim() || k == 0 {
            return None;
        }
        let n = a.rows;
        let mut basis: Vec<DenseVector> = Vec::with_capacity(k + 1);
        let mut alpha = Vec::with_capacity(k);
        let mut beta: Vec<f64> = Vec::with_capacity(k);
        let v0_norm = v0.norm();
        if v0_norm < 1e-14 {
            return None;
        }
        let v_init = v0.scale(1.0 / v0_norm);
        basis.push(v_init);
        let mut w_prev = DenseVector::zero(n);
        for j in 0..k {
            let vj = &basis[j];
            let av = DenseVector::apply(a, vj)?;
            let aj = vj.dot(&av);
            alpha.push(aj);
            let mut w_data = vec![0.0f64; n];
            for i in 0..n {
                w_data[i] = av.data[i] - aj * vj.data[i];
                if j > 0 {
                    w_data[i] -= beta[j - 1] * w_prev.data[i];
                }
            }
            let w = DenseVector { data: w_data };
            let bj = w.norm();
            if j + 1 < k {
                beta.push(bj);
                if bj < 1e-14 {
                    break;
                }
                let v_next = w.scale(1.0 / bj);
                w_prev = basis[j].clone();
                basis.push(v_next);
            }
        }
        Some(LanczosResult { basis, alpha, beta })
    }
    /// Retrieve the tridiagonal matrix T as a DenseMatrix.
    pub fn tridiagonal(&self) -> DenseMatrix {
        let n = self.alpha.len();
        let mut t = DenseMatrix::zero(n, n);
        for i in 0..n {
            t.data[i][i] = self.alpha[i];
        }
        for i in 0..self.beta.len() {
            t.data[i][i + 1] = self.beta[i];
            t.data[i + 1][i] = self.beta[i];
        }
        t
    }
}
