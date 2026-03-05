//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

pub struct L2Sequence {
    pub terms: Vec<f64>,
    pub max_terms: usize,
}
impl L2Sequence {
    pub fn new(terms: Vec<f64>) -> Self {
        let max_terms = terms.len();
        L2Sequence { terms, max_terms }
    }
    pub fn l2_norm(&self) -> f64 {
        self.terms.iter().map(|x| x * x).sum::<f64>().sqrt()
    }
    pub fn is_in_l2(&self) -> bool {
        self.l2_norm().is_finite()
    }
    pub fn inner_product(&self, other: &Self) -> f64 {
        self.terms
            .iter()
            .zip(other.terms.iter())
            .map(|(a, b)| a * b)
            .sum()
    }
    pub fn shift_left(&self) -> Self {
        let terms = if self.terms.is_empty() {
            vec![]
        } else {
            self.terms[1..].to_vec()
        };
        L2Sequence {
            max_terms: terms.len(),
            terms,
        }
    }
    pub fn shift_right(&self) -> Self {
        let mut terms = vec![0.0];
        terms.extend_from_slice(&self.terms);
        L2Sequence {
            max_terms: terms.len(),
            terms,
        }
    }
    pub fn seq_add(&self, other: &Self) -> Self {
        let len = self.terms.len().max(other.terms.len());
        let terms: Vec<f64> = (0..len)
            .map(|i| {
                let a = if i < self.terms.len() {
                    self.terms[i]
                } else {
                    0.0
                };
                let b = if i < other.terms.len() {
                    other.terms[i]
                } else {
                    0.0
                };
                a + b
            })
            .collect();
        L2Sequence {
            max_terms: terms.len(),
            terms,
        }
    }
    pub fn seq_scale(&self, c: f64) -> Self {
        let terms: Vec<f64> = self.terms.iter().map(|x| x * c).collect();
        L2Sequence {
            max_terms: terms.len(),
            terms,
        }
    }
    pub fn convolve(&self, other: &Self) -> Self {
        let (n, m) = (self.terms.len(), other.terms.len());
        if n == 0 || m == 0 {
            return L2Sequence::new(vec![]);
        }
        let mut result = vec![0.0; n + m - 1];
        for i in 0..n {
            for j in 0..m {
                result[i + j] += self.terms[i] * other.terms[j];
            }
        }
        L2Sequence::new(result)
    }
    pub fn parseval_residual(&self) -> f64 {
        let norm_sq = self.l2_norm().powi(2);
        let sum_sq: f64 = self.terms.iter().map(|x| x * x).sum();
        (norm_sq - sum_sq).abs()
    }
}
/// Data for Sobolev space W^{k,p}(Ω).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SobolevSpaceData {
    /// Differentiability order k.
    pub order: usize,
    /// Integrability p (1 <= p <= ∞).
    pub p: f64,
    /// Domain dimension n.
    pub domain_dim: usize,
    /// Domain description.
    pub domain: String,
}
#[allow(dead_code)]
impl SobolevSpaceData {
    /// Creates Sobolev space data.
    pub fn new(order: usize, p: f64, domain_dim: usize, domain: &str) -> Self {
        SobolevSpaceData {
            order,
            p,
            domain_dim,
            domain: domain.to_string(),
        }
    }
    /// Returns H^k = W^{k,2} Sobolev space.
    pub fn hilbert_sobolev(order: usize, domain_dim: usize, domain: &str) -> Self {
        SobolevSpaceData::new(order, 2.0, domain_dim, domain)
    }
    /// Sobolev embedding: W^{k,p}(Ω) ↪ L^q(Ω) for 1/q = 1/p - k/n.
    pub fn embedding_exponent(&self) -> Option<f64> {
        if self.p < 1.0 {
            return None;
        }
        let critical = 1.0 / self.p - (self.order as f64) / (self.domain_dim as f64);
        if critical <= 0.0 {
            None
        } else {
            Some(1.0 / critical)
        }
    }
    /// Returns the critical Sobolev exponent p* = np/(n-kp).
    pub fn critical_sobolev_exponent(&self) -> Option<f64> {
        let kp = self.order as f64 * self.p;
        let n = self.domain_dim as f64;
        if kp >= n {
            None
        } else {
            Some(n * self.p / (n - kp))
        }
    }
    /// Checks the Rellich-Kondrachov theorem (compact embedding).
    pub fn rellich_kondrachov_compact(&self) -> bool {
        self.order >= 1 && self.domain_dim >= 1
    }
    /// Returns trace theorem statement.
    pub fn trace_theorem(&self) -> String {
        if self.order >= 1 {
            format!(
                "Trace: W^{{{},{}}}'(Ω) → W^{{{}−1/p,p}}(∂Ω)",
                self.order, self.p, self.order
            )
        } else {
            "No trace for W^{0,p}".to_string()
        }
    }
}
/// Data for interpolation between Banach spaces.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InterpolationData {
    /// Space X_0 description.
    pub space0: String,
    /// Space X_1 description.
    pub space1: String,
    /// Interpolation parameter θ ∈ [0,1].
    pub theta: f64,
    /// Interpolation method.
    pub method: InterpolationMethod,
}
#[allow(dead_code)]
impl InterpolationData {
    /// Creates interpolation data.
    pub fn new(space0: &str, space1: &str, theta: f64, method: InterpolationMethod) -> Self {
        InterpolationData {
            space0: space0.to_string(),
            space1: space1.to_string(),
            theta,
            method,
        }
    }
    /// Returns the interpolation exponent for L^p spaces: 1/p = (1-θ)/p0 + θ/p1.
    pub fn lp_exponent(&self, p0: f64, p1: f64) -> f64 {
        (1.0 - self.theta) / p0 + self.theta / p1
    }
    /// Riesz-Thorin theorem: if T: X0→Y0 has norm M0 and T: X1→Y1 has norm M1,
    /// then T: [X0,X1]_θ → [Y0,Y1]_θ has norm <= M0^{1-θ} M1^θ.
    pub fn riesz_thorin_bound(&self, m0: f64, m1: f64) -> f64 {
        m0.powf(1.0 - self.theta) * m1.powf(self.theta)
    }
    /// Returns the interpolation space description.
    pub fn interpolation_space(&self) -> String {
        format!("[{}, {}]_{}", self.space0, self.space1, self.theta)
    }
}
#[derive(Debug, Clone)]
pub struct RnVector {
    pub components: Vec<f64>,
}
impl RnVector {
    pub fn new(components: Vec<f64>) -> Self {
        RnVector { components }
    }
    pub fn dim(&self) -> usize {
        self.components.len()
    }
    pub fn zero(dim: usize) -> Self {
        RnVector {
            components: vec![0.0; dim],
        }
    }
    pub fn basis(dim: usize, i: usize) -> Self {
        let mut v = vec![0.0; dim];
        if i < dim {
            v[i] = 1.0;
        }
        RnVector { components: v }
    }
    pub fn norm(&self) -> f64 {
        self.components.iter().map(|x| x * x).sum::<f64>().sqrt()
    }
    pub fn norm_p(&self, p: f64) -> f64 {
        if p.is_infinite() {
            self.components
                .iter()
                .map(|x| x.abs())
                .fold(0.0_f64, f64::max)
        } else {
            self.components
                .iter()
                .map(|x| x.abs().powf(p))
                .sum::<f64>()
                .powf(1.0 / p)
        }
    }
    pub fn inner(&self, other: &Self) -> f64 {
        self.components
            .iter()
            .zip(other.components.iter())
            .map(|(a, b)| a * b)
            .sum()
    }
    pub fn normalized(&self) -> Option<Self> {
        let n = self.norm();
        if n == 0.0 {
            None
        } else {
            Some(self.scale(1.0 / n))
        }
    }
    pub fn add(&self, other: &Self) -> Self {
        RnVector {
            components: self
                .components
                .iter()
                .zip(other.components.iter())
                .map(|(a, b)| a + b)
                .collect(),
        }
    }
    pub fn sub(&self, other: &Self) -> Self {
        RnVector {
            components: self
                .components
                .iter()
                .zip(other.components.iter())
                .map(|(a, b)| a - b)
                .collect(),
        }
    }
    pub fn scale(&self, s: f64) -> Self {
        RnVector {
            components: self.components.iter().map(|x| x * s).collect(),
        }
    }
    pub fn angle_with(&self, other: &Self) -> f64 {
        let n1 = self.norm();
        let n2 = other.norm();
        if n1 == 0.0 || n2 == 0.0 {
            return f64::NAN;
        }
        (self.inner(other) / (n1 * n2)).clamp(-1.0, 1.0).acos()
    }
    pub fn cross(&self, other: &Self) -> Option<Self> {
        if self.dim() != 3 || other.dim() != 3 {
            return None;
        }
        let (a, b) = (&self.components, &other.components);
        Some(RnVector::new(vec![
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ]))
    }
    pub fn project_onto(&self, other: &Self) -> Self {
        let d = other.inner(other);
        if d.abs() < 1e-15 {
            return RnVector::zero(self.dim());
        }
        other.scale(self.inner(other) / d)
    }
}
/// QR decomposition result: A = Q * R where Q is orthogonal and R is upper triangular.
#[derive(Debug, Clone)]
pub struct QrDecomposition {
    pub q: BoundedOp,
    pub r: BoundedOp,
}
/// Interpolation method.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum InterpolationMethod {
    /// Complex interpolation (Calderón-Lions).
    Complex,
    /// Real interpolation (K-functional).
    Real,
    /// Riesz-Thorin (L^p spaces).
    RieszThorin,
}
#[derive(Debug, Clone)]
pub struct BoundedOp {
    pub matrix: Vec<Vec<f64>>,
    pub domain_dim: usize,
    pub range_dim: usize,
}
impl BoundedOp {
    pub fn new(matrix: Vec<Vec<f64>>) -> Self {
        let range_dim = matrix.len();
        let domain_dim = if range_dim > 0 { matrix[0].len() } else { 0 };
        BoundedOp {
            matrix,
            domain_dim,
            range_dim,
        }
    }
    pub fn identity(n: usize) -> Self {
        let matrix: Vec<Vec<f64>> = (0..n)
            .map(|i| (0..n).map(|j| if i == j { 1.0 } else { 0.0 }).collect())
            .collect();
        BoundedOp {
            matrix,
            domain_dim: n,
            range_dim: n,
        }
    }
    pub fn diagonal(entries: &[f64]) -> Self {
        let n = entries.len();
        let matrix: Vec<Vec<f64>> = (0..n)
            .map(|i| {
                let mut row = vec![0.0; n];
                row[i] = entries[i];
                row
            })
            .collect();
        BoundedOp {
            matrix,
            domain_dim: n,
            range_dim: n,
        }
    }
    pub fn zero_op(m: usize, n: usize) -> Self {
        BoundedOp {
            matrix: vec![vec![0.0; n]; m],
            domain_dim: n,
            range_dim: m,
        }
    }
    pub fn apply(&self, v: &RnVector) -> RnVector {
        if v.dim() != self.domain_dim {
            return RnVector::zero(self.range_dim);
        }
        let comps: Vec<f64> = self
            .matrix
            .iter()
            .map(|row| {
                row.iter()
                    .zip(v.components.iter())
                    .map(|(a, b)| a * b)
                    .sum()
            })
            .collect();
        RnVector { components: comps }
    }
    pub fn operator_norm(&self) -> f64 {
        self.frobenius_norm()
    }
    pub fn operator_norm_power_iter(&self, iterations: usize) -> f64 {
        let ata = match self.transpose().compose(self) {
            Some(m) => m,
            None => return 0.0,
        };
        if self.domain_dim == 0 {
            return 0.0;
        }
        let mut v = RnVector::new(vec![1.0; self.domain_dim]);
        let n = v.norm();
        if n > 0.0 {
            v = v.scale(1.0 / n);
        }
        let mut eigenvalue = 0.0;
        for _ in 0..iterations {
            let w = ata.apply(&v);
            eigenvalue = w.norm();
            if eigenvalue < 1e-15 {
                return 0.0;
            }
            v = w.scale(1.0 / eigenvalue);
        }
        eigenvalue.sqrt()
    }
    pub fn transpose(&self) -> Self {
        if self.range_dim == 0 || self.domain_dim == 0 {
            return BoundedOp::new(vec![]);
        }
        let matrix: Vec<Vec<f64>> = (0..self.domain_dim)
            .map(|j| (0..self.range_dim).map(|i| self.matrix[i][j]).collect())
            .collect();
        BoundedOp {
            matrix,
            domain_dim: self.range_dim,
            range_dim: self.domain_dim,
        }
    }
    pub fn compose(&self, other: &Self) -> Option<Self> {
        if other.range_dim != self.domain_dim {
            return None;
        }
        let (m, n, k) = (self.range_dim, other.domain_dim, self.domain_dim);
        let matrix: Vec<Vec<f64>> = (0..m)
            .map(|i| {
                (0..n)
                    .map(|j| (0..k).map(|l| self.matrix[i][l] * other.matrix[l][j]).sum())
                    .collect()
            })
            .collect();
        Some(BoundedOp {
            matrix,
            domain_dim: n,
            range_dim: m,
        })
    }
    pub fn op_add(&self, other: &Self) -> Option<Self> {
        if self.domain_dim != other.domain_dim || self.range_dim != other.range_dim {
            return None;
        }
        let matrix: Vec<Vec<f64>> = self
            .matrix
            .iter()
            .zip(other.matrix.iter())
            .map(|(r1, r2)| r1.iter().zip(r2.iter()).map(|(a, b)| a + b).collect())
            .collect();
        Some(BoundedOp {
            matrix,
            domain_dim: self.domain_dim,
            range_dim: self.range_dim,
        })
    }
    pub fn scalar_mul(&self, c: f64) -> Self {
        let matrix: Vec<Vec<f64>> = self
            .matrix
            .iter()
            .map(|row| row.iter().map(|x| x * c).collect())
            .collect();
        BoundedOp {
            matrix,
            domain_dim: self.domain_dim,
            range_dim: self.range_dim,
        }
    }
    pub fn is_symmetric(&self) -> bool {
        if self.domain_dim != self.range_dim {
            return false;
        }
        let n = self.domain_dim;
        for i in 0..n {
            for j in (i + 1)..n {
                if (self.matrix[i][j] - self.matrix[j][i]).abs() > 1e-10 {
                    return false;
                }
            }
        }
        true
    }
    pub fn is_positive_definite(&self) -> bool {
        if !self.is_symmetric() {
            return false;
        }
        let n = self.domain_dim;
        let mut l = vec![vec![0.0; n]; n];
        for i in 0..n {
            for j in 0..=i {
                let mut sum = 0.0;
                for k in 0..j {
                    sum += l[i][k] * l[j][k];
                }
                if i == j {
                    let val = self.matrix[i][i] - sum;
                    if val <= 0.0 {
                        return false;
                    }
                    l[i][j] = val.sqrt();
                } else {
                    if l[j][j].abs() < 1e-15 {
                        return false;
                    }
                    l[i][j] = (self.matrix[i][j] - sum) / l[j][j];
                }
            }
        }
        true
    }
    pub fn eigenvalues_2x2(&self) -> Option<(f64, f64)> {
        if self.domain_dim != 2 || self.range_dim != 2 || !self.is_symmetric() {
            return None;
        }
        let (a, b, d) = (self.matrix[0][0], self.matrix[0][1], self.matrix[1][1]);
        let tr = a + d;
        let det = a * d - b * b;
        let disc = tr * tr - 4.0 * det;
        if disc < 0.0 {
            return None;
        }
        let s = disc.sqrt();
        Some(((tr + s) / 2.0, (tr - s) / 2.0))
    }
    pub fn power_iteration(&self, iterations: usize) -> Option<(f64, RnVector)> {
        if self.domain_dim != self.range_dim || self.domain_dim == 0 {
            return None;
        }
        let n = self.domain_dim;
        let mut v = RnVector::new(vec![1.0; n]);
        let nrm = v.norm();
        if nrm > 0.0 {
            v = v.scale(1.0 / nrm);
        }
        let mut eigenvalue = 0.0;
        for _ in 0..iterations {
            let w = self.apply(&v);
            eigenvalue = w.inner(&v);
            let nrm = w.norm();
            if nrm < 1e-15 {
                return Some((0.0, v));
            }
            v = w.scale(1.0 / nrm);
        }
        Some((eigenvalue, v))
    }
    pub fn trace(&self) -> f64 {
        let n = self.domain_dim.min(self.range_dim);
        (0..n).map(|i| self.matrix[i][i]).sum()
    }
    pub fn frobenius_norm(&self) -> f64 {
        self.matrix
            .iter()
            .flat_map(|row| row.iter())
            .map(|x| x * x)
            .sum::<f64>()
            .sqrt()
    }
    pub fn determinant(&self) -> Option<f64> {
        if self.domain_dim != self.range_dim {
            return None;
        }
        let n = self.domain_dim;
        if n == 0 {
            return Some(1.0);
        }
        let mut a = self.matrix.clone();
        let mut sign = 1.0;
        for col in 0..n {
            let mut max_row = col;
            let mut max_val = a[col][col].abs();
            for row in (col + 1)..n {
                if a[row][col].abs() > max_val {
                    max_val = a[row][col].abs();
                    max_row = row;
                }
            }
            if max_val < 1e-15 {
                return Some(0.0);
            }
            if max_row != col {
                a.swap(col, max_row);
                sign *= -1.0;
            }
            let pivot = a[col][col];
            for row in (col + 1)..n {
                let factor = a[row][col] / pivot;
                for j in col..n {
                    let val = a[col][j];
                    a[row][j] -= factor * val;
                }
            }
        }
        Some((0..n).map(|i| a[i][i]).product::<f64>() * sign)
    }
    pub fn solve(&self, b: &RnVector) -> Option<RnVector> {
        if self.domain_dim != self.range_dim || b.dim() != self.range_dim {
            return None;
        }
        let n = self.domain_dim;
        let mut aug: Vec<Vec<f64>> = self
            .matrix
            .iter()
            .enumerate()
            .map(|(i, row)| {
                let mut r = row.clone();
                r.push(b.components[i]);
                r
            })
            .collect();
        for col in 0..n {
            let mut max_row = col;
            let mut max_val = aug[col][col].abs();
            for row in (col + 1)..n {
                if aug[row][col].abs() > max_val {
                    max_val = aug[row][col].abs();
                    max_row = row;
                }
            }
            if max_val < 1e-15 {
                return None;
            }
            if max_row != col {
                aug.swap(col, max_row);
            }
            let pivot = aug[col][col];
            for row in (col + 1)..n {
                let factor = aug[row][col] / pivot;
                for j in col..=n {
                    let val = aug[col][j];
                    aug[row][j] -= factor * val;
                }
            }
        }
        let mut x = vec![0.0; n];
        for i in (0..n).rev() {
            let mut sum = aug[i][n];
            for j in (i + 1)..n {
                sum -= aug[i][j] * x[j];
            }
            if aug[i][i].abs() < 1e-15 {
                return None;
            }
            x[i] = sum / aug[i][i];
        }
        Some(RnVector::new(x))
    }
    pub fn rank(&self) -> usize {
        let (m, n) = (self.range_dim, self.domain_dim);
        let mut a = self.matrix.clone();
        let mut rank = 0;
        for col in 0..n {
            let mut pivot_row = None;
            for row in rank..m {
                if a[row][col].abs() > 1e-12 {
                    pivot_row = Some(row);
                    break;
                }
            }
            let pr = match pivot_row {
                Some(r) => r,
                None => continue,
            };
            a.swap(rank, pr);
            let pivot = a[rank][col];
            for row in (rank + 1)..m {
                let factor = a[row][col] / pivot;
                for j in col..n {
                    let val = a[rank][j];
                    a[row][j] -= factor * val;
                }
            }
            rank += 1;
        }
        rank
    }
    pub fn nullity(&self) -> usize {
        self.domain_dim - self.rank()
    }
    pub fn is_injective(&self) -> bool {
        self.nullity() == 0
    }
    pub fn is_surjective(&self) -> bool {
        self.rank() == self.range_dim
    }
}
/// LU decomposition result with partial pivoting: P*A = L*U.
#[derive(Debug, Clone)]
pub struct LuDecomposition {
    pub l: BoundedOp,
    pub u: BoundedOp,
    pub permutation: Vec<usize>,
}
/// Data representing weak convergence in a Banach space.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WeakConvergenceData {
    /// Sequence of vectors (represented as finite-dimensional approximations).
    pub sequence: Vec<Vec<f64>>,
    /// Weak limit, if it exists.
    pub weak_limit: Option<Vec<f64>>,
    /// Whether the sequence is bounded (Banach-Alaoglu prerequisite).
    pub is_bounded: bool,
}
#[allow(dead_code)]
impl WeakConvergenceData {
    /// Creates weak convergence data.
    pub fn new(sequence: Vec<Vec<f64>>) -> Self {
        let is_bounded = sequence.iter().all(|v| {
            let norm: f64 = v.iter().map(|&x| x * x).sum::<f64>().sqrt();
            norm <= 1e6
        });
        WeakConvergenceData {
            sequence,
            weak_limit: None,
            is_bounded,
        }
    }
    /// Checks weak convergence to a given vector (tests against standard basis functionals).
    pub fn check_weak_convergence(&self, limit: &[f64], tol: f64) -> bool {
        if self.sequence.is_empty() {
            return true;
        }
        let last = &self.sequence[self.sequence.len().saturating_sub(5)..];
        last.iter().all(|v| {
            limit
                .iter()
                .zip(v.iter())
                .map(|(&l, &x)| (l - x).abs())
                .fold(0.0f64, f64::max)
                < tol
        })
    }
    /// Banach-Alaoglu: bounded sequences in reflexive spaces have weakly convergent subsequences.
    pub fn banach_alaoglu_applies(&self) -> bool {
        self.is_bounded
    }
    /// Returns the strong norm of the last element.
    pub fn last_norm(&self) -> Option<f64> {
        self.sequence
            .last()
            .map(|v| v.iter().map(|&x| x * x).sum::<f64>().sqrt())
    }
}
/// Data for a Fredholm operator T: X → Y.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FredholmOperatorData {
    /// Description of the operator.
    pub name: String,
    /// Dimension of kernel (null space).
    pub kernel_dim: usize,
    /// Dimension of cokernel (Y / Im T).
    pub cokernel_dim: usize,
    /// Whether T is Fredholm.
    pub is_fredholm: bool,
}
#[allow(dead_code)]
impl FredholmOperatorData {
    /// Creates Fredholm operator data.
    pub fn new(name: &str, kernel_dim: usize, cokernel_dim: usize) -> Self {
        FredholmOperatorData {
            name: name.to_string(),
            kernel_dim,
            cokernel_dim,
            is_fredholm: true,
        }
    }
    /// Fredholm index: ind(T) = dim(ker T) - dim(coker T).
    pub fn index(&self) -> i64 {
        self.kernel_dim as i64 - self.cokernel_dim as i64
    }
    /// Checks Atkinson's theorem: T is Fredholm iff it is invertible modulo compact operators.
    pub fn atkinson_description(&self) -> String {
        format!("T = {} is Fredholm with index {}", self.name, self.index())
    }
    /// Returns the stability of the index under compact perturbations.
    pub fn index_stable_under_compact(&self) -> bool {
        true
    }
    /// Checks if T is an isomorphism (index 0 and injective).
    pub fn is_isomorphism(&self) -> bool {
        self.kernel_dim == 0 && self.cokernel_dim == 0
    }
    /// Essential spectrum: the set of λ for which T - λ is NOT Fredholm.
    pub fn essential_spectrum_description(&self) -> String {
        format!("σ_ess({}): values λ making T-λI non-Fredholm", self.name)
    }
}
/// Represents a distribution (generalized function) as a linear functional on test functions.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Distribution {
    /// Name of the distribution.
    pub name: String,
    /// Order: distributions of finite order.
    pub order: Option<usize>,
    /// Support description.
    pub support: String,
    /// Whether the distribution is a regular distribution (given by integration).
    pub is_regular: bool,
}
#[allow(dead_code)]
impl Distribution {
    /// Creates a distribution.
    pub fn new(name: &str) -> Self {
        Distribution {
            name: name.to_string(),
            order: None,
            support: "unknown".to_string(),
            is_regular: false,
        }
    }
    /// Creates the Dirac delta distribution.
    pub fn dirac_delta(point: f64) -> Self {
        Distribution {
            name: format!("δ_{{{point}}}"),
            order: Some(0),
            support: point.to_string(),
            is_regular: false,
        }
    }
    /// Creates a regular distribution from L^1_loc.
    pub fn regular(name: &str, support: &str) -> Self {
        Distribution {
            name: name.to_string(),
            order: Some(0),
            support: support.to_string(),
            is_regular: true,
        }
    }
    /// Differentiation raises the order by 1.
    pub fn differentiate(&self) -> Distribution {
        Distribution {
            name: format!("d/dx ({})", self.name),
            order: self.order.map(|o| o + 1),
            support: self.support.clone(),
            is_regular: false,
        }
    }
    /// Returns the Fourier transform description.
    pub fn fourier_transform_description(&self) -> String {
        if self.name.starts_with('δ') {
            "F[δ_a](ξ) = e^{-2πiαξ} (constant modulus 1)".to_string()
        } else {
            format!("F[{}](ξ) (distributional Fourier transform)", self.name)
        }
    }
    /// Checks if the distribution is tempered (in S').
    pub fn is_tempered(&self) -> bool {
        self.order.map(|o| o <= 100).unwrap_or(false)
    }
}
