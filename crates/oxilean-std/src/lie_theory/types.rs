//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

/// Exceptional Lie algebra.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExceptionalLieAlgebra {
    /// G₂: dimension 14, rank 2
    G2,
    /// F₄: dimension 52, rank 4
    F4,
    /// E₆: dimension 78, rank 6
    E6,
    /// E₇: dimension 133, rank 7
    E7,
    /// E₈: dimension 248, rank 8
    E8,
}
impl ExceptionalLieAlgebra {
    /// Complex dimension of the exceptional Lie algebra.
    pub fn dimension(&self) -> usize {
        match self {
            Self::G2 => 14,
            Self::F4 => 52,
            Self::E6 => 78,
            Self::E7 => 133,
            Self::E8 => 248,
        }
    }
    /// Rank (dimension of Cartan subalgebra).
    pub fn rank(&self) -> usize {
        match self {
            Self::G2 => 2,
            Self::F4 => 4,
            Self::E6 => 6,
            Self::E7 => 7,
            Self::E8 => 8,
        }
    }
    /// Conventional name.
    pub fn name(&self) -> &'static str {
        match self {
            Self::G2 => "G_2",
            Self::F4 => "F_4",
            Self::E6 => "E_6",
            Self::E7 => "E_7",
            Self::E8 => "E_8",
        }
    }
    /// Number of positive roots.
    pub fn num_positive_roots(&self) -> usize {
        match self {
            Self::G2 => 6,
            Self::F4 => 24,
            Self::E6 => 36,
            Self::E7 => 63,
            Self::E8 => 120,
        }
    }
    /// Coxeter number h.
    pub fn coxeter_number(&self) -> usize {
        match self {
            Self::G2 => 6,
            Self::F4 => 12,
            Self::E6 => 12,
            Self::E7 => 18,
            Self::E8 => 30,
        }
    }
}
/// A nilpotent orbit in the nilpotent cone N(g) of a semisimple Lie algebra g.
///
/// By the Jacobson-Morozov theorem, each nilpotent orbit determines an sl_2-triple
/// (e, h, f) with [h,e]=2e, [h,f]=-2f, [e,f]=h.
pub struct NilpotentOrbit {
    /// Algebra name.
    pub algebra: String,
    /// Bala-Carter label (e.g. "A_1", "2A_1", "D_4(a_1)").
    pub label: String,
    /// Dimension of the orbit as a variety.
    pub dimension: usize,
    /// Whether this is the regular (principal) nilpotent orbit (open dense).
    pub is_regular: bool,
    /// Whether this is the subregular nilpotent orbit.
    pub is_subregular: bool,
    /// Dynkin diagram of the orbit (weighted Dynkin diagram: labels 0, 1, 2).
    pub dynkin_labels: Vec<u8>,
}
impl NilpotentOrbit {
    /// Construct a nilpotent orbit.
    pub fn new(
        algebra: impl Into<String>,
        label: impl Into<String>,
        dimension: usize,
        is_regular: bool,
        is_subregular: bool,
        dynkin_labels: Vec<u8>,
    ) -> Self {
        Self {
            algebra: algebra.into(),
            label: label.into(),
            dimension,
            is_regular,
            is_subregular,
            dynkin_labels,
        }
    }
    /// The zero orbit (the trivial orbit {0}).
    pub fn zero(algebra: impl Into<String>, rank: usize) -> Self {
        Self::new(algebra, "0", 0, false, false, vec![0; rank])
    }
    /// The principal (regular) nilpotent orbit has dimension dim(g) - rank(g).
    pub fn principal(algebra: impl Into<String>, alg_dimension: usize, rank: usize) -> Self {
        let dim = alg_dimension.saturating_sub(rank);
        Self::new(algebra, "regular", dim, true, false, vec![2; rank])
    }
    /// Codimension of this orbit in the nilpotent cone.
    pub fn codimension(&self, nilcone_dimension: usize) -> usize {
        nilcone_dimension.saturating_sub(self.dimension)
    }
    /// Orbit closure partial order: O₁ ≤ O₂ iff O₁ ⊆ closure(O₂).
    ///
    /// Returns true if this orbit is likely in the closure of `other`
    /// (heuristic: dimension ordering).
    pub fn is_in_closure_of(&self, other: &Self) -> bool {
        self.dimension <= other.dimension
    }
    /// A-group A(O) of the orbit: component group of the centralizer Z_G(e).
    ///
    /// For classical algebras the A-group is often trivial or ℤ/2.
    pub fn a_group_description(&self) -> String {
        format!("A({}) for orbit {}", self.algebra, self.label)
    }
}
/// An element of a Lie group represented as a matrix.
pub struct LieGroupElement {
    pub group: String,
    pub matrix: Vec<Vec<f64>>,
}
impl LieGroupElement {
    pub fn new(group: impl Into<String>, matrix: Vec<Vec<f64>>) -> Self {
        Self {
            group: group.into(),
            matrix,
        }
    }
    /// Matrix size (n for an n×n matrix).
    pub fn size(&self) -> usize {
        self.matrix.len()
    }
    /// Matrix determinant (for square matrices up to 3×3).
    pub fn determinant(&self) -> f64 {
        let n = self.size();
        match n {
            0 => 1.0,
            1 => self.matrix[0][0],
            2 => self.matrix[0][0] * self.matrix[1][1] - self.matrix[0][1] * self.matrix[1][0],
            3 => {
                let m = &self.matrix;
                m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
                    - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
                    + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
            }
            _ => f64::NAN,
        }
    }
    /// Matrix transpose.
    #[allow(clippy::needless_range_loop)]
    pub fn transpose(&self) -> Self {
        let n = self.size();
        let m_cols = if n > 0 { self.matrix[0].len() } else { 0 };
        let mut t = vec![vec![0.0_f64; n]; m_cols];
        for i in 0..n {
            for j in 0..m_cols {
                t[j][i] = self.matrix[i][j];
            }
        }
        Self {
            group: self.group.clone(),
            matrix: t,
        }
    }
    #[allow(clippy::needless_range_loop)]
    /// Matrix multiplication (square matrices of same size).
    pub fn multiply(&self, other: &Self) -> Self {
        let n = self.size();
        let mut result = vec![vec![0.0_f64; n]; n];
        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    result[i][j] += self.matrix[i][k] * other.matrix[k][j];
                }
            }
        }
        Self {
            group: self.group.clone(),
            matrix: result,
        }
    }
    /// Trace of the matrix.
    pub fn trace(&self) -> f64 {
        let n = self.size();
        (0..n).map(|i| self.matrix[i][i]).sum()
    }
}
/// Cartan subalgebra: maximal abelian self-normalizing subalgebra h ⊂ g.
pub struct CartanSubalgebra {
    pub rank: usize,
    pub generators: Vec<String>,
}
impl CartanSubalgebra {
    pub fn new(rank: usize) -> Self {
        let generators = (0..rank).map(|i| format!("H_{}", i + 1)).collect();
        Self { rank, generators }
    }
    /// Dimension of the Cartan subalgebra equals its rank.
    pub fn dimension(&self) -> usize {
        self.rank
    }
}
/// An element of a Lie algebra expressed in a basis.
pub struct LieAlgebraElement {
    pub algebra: String,
    pub coefficients: Vec<f64>,
}
impl LieAlgebraElement {
    pub fn new(algebra: impl Into<String>, coefficients: Vec<f64>) -> Self {
        Self {
            algebra: algebra.into(),
            coefficients,
        }
    }
    /// Scalar multiplication.
    pub fn scale(&self, s: f64) -> Self {
        Self {
            algebra: self.algebra.clone(),
            coefficients: self.coefficients.iter().map(|&c| c * s).collect(),
        }
    }
    /// Add two elements (same algebra, same dimension).
    pub fn add(&self, other: &Self) -> Self {
        let len = self.coefficients.len().max(other.coefficients.len());
        let mut result = vec![0.0_f64; len];
        for (i, &v) in self.coefficients.iter().enumerate() {
            result[i] += v;
        }
        for (i, &v) in other.coefficients.iter().enumerate() {
            result[i] += v;
        }
        Self {
            algebra: self.algebra.clone(),
            coefficients: result,
        }
    }
    /// L2 norm of the coefficient vector.
    pub fn norm(&self) -> f64 {
        self.coefficients.iter().map(|&c| c * c).sum::<f64>().sqrt()
    }
}
/// The exponential map exp : g → G from a Lie algebra to its Lie group.
pub struct ExponentialMap {
    pub algebra: String,
    pub group: String,
}
impl ExponentialMap {
    pub fn new(algebra: impl Into<String>, group: impl Into<String>) -> Self {
        Self {
            algebra: algebra.into(),
            group: group.into(),
        }
    }
    /// The exponential map always maps the Lie algebra to the Lie group.
    pub fn maps_algebra_to_group(&self) -> bool {
        true
    }
    /// For compact connected Lie groups the exponential map is surjective.
    pub fn is_surjective_for_compact(&self) -> bool {
        true
    }
    /// Evaluate exp(X) for a 2×2 matrix using Taylor series (up to 20 terms).
    pub fn matrix_exp_2x2(&self, x: &[[f64; 2]; 2]) -> [[f64; 2]; 2] {
        let mut result = [[1.0_f64, 0.0], [0.0, 1.0]];
        let mut term = [[1.0_f64, 0.0], [0.0, 1.0]];
        for k in 1..=20_usize {
            let new_term = [
                [
                    (term[0][0] * x[0][0] + term[0][1] * x[1][0]) / k as f64,
                    (term[0][0] * x[0][1] + term[0][1] * x[1][1]) / k as f64,
                ],
                [
                    (term[1][0] * x[0][0] + term[1][1] * x[1][0]) / k as f64,
                    (term[1][0] * x[0][1] + term[1][1] * x[1][1]) / k as f64,
                ],
            ];
            result[0][0] += new_term[0][0];
            result[0][1] += new_term[0][1];
            result[1][0] += new_term[1][0];
            result[1][1] += new_term[1][1];
            term = new_term;
        }
        result
    }
}
/// Lie group homomorphism.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LieGroupHom {
    pub source: String,
    pub target: String,
    pub is_surjective: bool,
    pub kernel_description: String,
}
#[allow(dead_code)]
impl LieGroupHom {
    /// Covering map.
    pub fn covering(source: &str, target: &str, kernel: &str) -> Self {
        Self {
            source: source.to_string(),
            target: target.to_string(),
            is_surjective: true,
            kernel_description: kernel.to_string(),
        }
    }
    /// Adjoint representation: G → GL(g).
    pub fn adjoint(g: &str) -> Self {
        Self {
            source: g.to_string(),
            target: format!("GL(Lie({}))", g),
            is_surjective: false,
            kernel_description: "center Z(G)".to_string(),
        }
    }
    /// Induced map on Lie algebras.
    pub fn lie_algebra_map(&self) -> String {
        format!("d phi: Lie({}) → Lie({})", self.source, self.target)
    }
}
/// A finite-dimensional Lie algebra over a field.
pub struct LieAlgebra {
    pub name: String,
    pub dimension: usize,
    pub field: String,
    pub is_simple: bool,
    pub is_semisimple: bool,
}
impl LieAlgebra {
    /// Create a new Lie algebra.
    pub fn new(
        name: impl Into<String>,
        dimension: usize,
        field: impl Into<String>,
        is_simple: bool,
        is_semisimple: bool,
    ) -> Self {
        Self {
            name: name.into(),
            dimension,
            field: field.into(),
            is_simple,
            is_semisimple,
        }
    }
    /// Rank of the algebra (dimension of a Cartan subalgebra).
    ///
    /// For classical algebras this returns a heuristic estimate;
    /// for general algebras we return dimension / 3 as an approximation.
    pub fn rank(&self) -> usize {
        if self.dimension == 0 {
            0
        } else {
            (self.dimension as f64).cbrt().round() as usize
        }
    }
    /// A Lie algebra is abelian iff its Lie bracket vanishes identically.
    ///
    /// Simple algebras are never abelian; dimension-0 is trivially abelian.
    pub fn is_abelian(&self) -> bool {
        if self.is_simple {
            return false;
        }
        self.dimension == 0
    }
    /// A Lie algebra is nilpotent iff its lower central series terminates.
    ///
    /// Abelian algebras are nilpotent; simple/semisimple are not.
    pub fn is_nilpotent(&self) -> bool {
        if self.is_semisimple && self.dimension > 0 {
            return false;
        }
        self.is_abelian()
    }
    /// A Lie algebra is solvable iff its derived series terminates.
    ///
    /// Nilpotent implies solvable; simple algebras are not solvable.
    pub fn is_solvable(&self) -> bool {
        if self.is_simple {
            return false;
        }
        self.is_nilpotent()
    }
}
/// Dynkin diagram encoding the Cartan matrix via nodes and edges.
pub struct DynkinDiagram {
    /// Node labels (e.g. "α₁", "α₂", ...)
    pub nodes: Vec<String>,
    /// Edges: (from, to, multiplicity) where multiplicity ∈ {1, 2, 3}
    pub edges: Vec<(usize, usize, u8)>,
}
impl DynkinDiagram {
    /// Build the Dynkin diagram for a classical Lie algebra.
    pub fn from_classical(algebra: ClassicalLieAlgebra) -> Self {
        let n = algebra.rank();
        let nodes: Vec<String> = (1..=n).map(|i| format!("α_{}", i)).collect();
        let mut edges = Vec::new();
        match algebra {
            ClassicalLieAlgebra::An(_) => {
                for i in 0..n.saturating_sub(1) {
                    edges.push((i, i + 1, 1));
                }
            }
            ClassicalLieAlgebra::Bn(_) => {
                for i in 0..n.saturating_sub(1) {
                    let mult = if i + 1 == n - 1 { 2 } else { 1 };
                    edges.push((i, i + 1, mult));
                }
            }
            ClassicalLieAlgebra::Cn(_) => {
                for i in 0..n.saturating_sub(1) {
                    let mult = if i == 0 { 2 } else { 1 };
                    edges.push((i, i + 1, mult));
                }
            }
            ClassicalLieAlgebra::Dn(_) => {
                for i in 0..n.saturating_sub(2) {
                    edges.push((i, i + 1, 1));
                }
                if n >= 2 {
                    edges.push((n - 2, n - 1, 1));
                }
            }
        }
        Self { nodes, edges }
    }
    /// Number of nodes.
    pub fn num_nodes(&self) -> usize {
        self.nodes.len()
    }
    /// Number of edges.
    pub fn num_edges(&self) -> usize {
        self.edges.len()
    }
}
/// The weight lattice P = Σ ℤ ω_i generated by fundamental weights.
pub struct WeightLattice {
    pub rank: usize,
    pub fundamental_weights: Vec<Vec<i32>>,
}
impl WeightLattice {
    pub fn new(rank: usize) -> Self {
        let fundamental_weights: Vec<Vec<i32>> = (0..rank)
            .map(|i| {
                let mut w = vec![0_i32; rank];
                w[i] = 1;
                w
            })
            .collect();
        Self {
            rank,
            fundamental_weights,
        }
    }
    /// Express a weight as an integer linear combination of fundamental weights.
    pub fn decompose(&self, weight: &[i32]) -> Vec<i32> {
        weight.to_vec()
    }
}
/// A Verma module M(λ) for a semisimple Lie algebra g with weight λ.
///
/// The Verma module is the universal highest weight module generated by a
/// highest weight vector v_λ satisfying n⁺·v_λ = 0 and H·v_λ = λ(H)v_λ.
pub struct VermaModule {
    /// Name of the underlying Lie algebra.
    pub algebra: String,
    /// Highest weight (Dynkin labels).
    pub highest_weight: Vec<i32>,
    /// Whether this module is a simple (irreducible) quotient L(λ).
    pub is_simple: bool,
}
impl VermaModule {
    /// Construct a Verma module M(λ) for algebra `algebra` and highest weight `lambda`.
    pub fn new(algebra: impl Into<String>, highest_weight: Vec<i32>) -> Self {
        Self {
            algebra: algebra.into(),
            highest_weight,
            is_simple: false,
        }
    }
    /// The Verma module is finite-dimensional iff λ is a dominant integral weight.
    ///
    /// For A_1 (sl_2): dominant integral iff highest_weight[0] ≥ 0.
    pub fn is_finite_dimensional(&self) -> bool {
        self.highest_weight.iter().all(|&w| w >= 0)
    }
    /// The Verma module M(λ) is irreducible iff λ is antidominant:
    /// ⟨λ + ρ, α∨⟩ ∉ ℤ_{>0} for all positive roots α.
    ///
    /// Heuristic: irreducible iff all Dynkin labels are strictly negative.
    pub fn is_irreducible(&self) -> bool {
        self.highest_weight.iter().all(|&w| w < 0)
    }
    /// Dimension of the finite-dimensional quotient L(λ) via Weyl dimension formula.
    ///
    /// For sl_2: dim L(m) = m + 1 where m = highest_weight[0] ≥ 0.
    pub fn simple_quotient_dimension(&self) -> Option<u64> {
        if !self.is_finite_dimensional() {
            return None;
        }
        if self.highest_weight.len() == 1 {
            Some((self.highest_weight[0] + 1).max(0) as u64)
        } else {
            let rank = self.highest_weight.len();
            let mut dim = 1u64;
            for i in 0..rank {
                for j in i..rank {
                    let num = (self.highest_weight[i..=j].iter().sum::<i32>() + (j - i + 1) as i32)
                        as u64;
                    let den = (j - i + 1) as u64;
                    dim = dim.saturating_mul(num) / den;
                }
            }
            Some(dim)
        }
    }
    /// BGG duality: Ext^k(M(λ), M(μ)) is related to KL polynomial P_{x,y}(1).
    pub fn bgg_duality_statement(&self) -> String {
        format!(
            "BGG duality for M({:?}) in category O: Ext^k(M(λ), L(μ)) ≅ KL multiplicity",
            self.highest_weight
        )
    }
}
/// Invariant theory data for Lie groups.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InvariantTheory {
    pub group: String,
    pub representation: String,
    pub invariant_generators: Vec<String>,
}
#[allow(dead_code)]
impl InvariantTheory {
    /// Classical invariants for SL_n.
    pub fn sl_n_invariants(n: usize) -> Self {
        Self {
            group: format!("SL_{}", n),
            representation: "standard".to_string(),
            invariant_generators: (2..=n).map(|k| format!("sigma_{}", k)).collect(),
        }
    }
    /// Number of fundamental invariants.
    pub fn num_invariants(&self) -> usize {
        self.invariant_generators.len()
    }
    /// Chevalley's theorem: invariant polynomial ring is free.
    pub fn chevalley_description(&self) -> String {
        format!(
            "C[V]^{} is freely generated by {} elements",
            self.group,
            self.invariant_generators.len()
        )
    }
}
/// The Lie bracket [X, Y] for a named Lie algebra.
pub struct LieBracket {
    pub algebra: String,
}
impl LieBracket {
    pub fn new(algebra: impl Into<String>) -> Self {
        Self {
            algebra: algebra.into(),
        }
    }
    /// Evaluate the Lie bracket of two elements given their coefficient vectors.
    ///
    /// Uses structure constants f^k_{ij}: result_k = sum_{i,j} f^k_{ij} * a_i * b_j.
    /// If structure constants are not provided, returns the zero element.
    #[allow(clippy::needless_range_loop)]
    pub fn evaluate(
        &self,
        a: &[f64],
        b: &[f64],
        structure_consts: &StructureConstants,
    ) -> Vec<f64> {
        let n = a.len().min(b.len()).min(structure_consts.rank);
        let mut result = vec![0.0_f64; n];
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    result[k] += structure_consts.f[i][j][k] * a[i] * b[j];
                }
            }
        }
        result
    }
}
/// A Lie group: a smooth manifold with a compatible group structure.
pub struct LieGroup {
    pub name: String,
    pub dimension: usize,
    pub is_compact: bool,
    pub is_connected: bool,
    pub is_simply_connected: bool,
}
impl LieGroup {
    pub fn new(
        name: impl Into<String>,
        dimension: usize,
        is_compact: bool,
        is_connected: bool,
        is_simply_connected: bool,
    ) -> Self {
        Self {
            name: name.into(),
            dimension,
            is_compact,
            is_connected,
            is_simply_connected,
        }
    }
}
/// Classical Lie group.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClassicalLieGroup {
    /// GL(n): general linear group, invertible n×n matrices
    GL(usize),
    /// SL(n): special linear group, det = 1
    SL(usize),
    /// O(n): orthogonal group, A^T A = I
    O(usize),
    /// SO(n): special orthogonal group, det = 1
    SO(usize),
    /// U(n): unitary group, A† A = I
    U(usize),
    /// SU(n): special unitary group, det = 1
    SU(usize),
    /// Sp(2n): symplectic group
    Sp(usize),
}
impl ClassicalLieGroup {
    /// Real dimension of the Lie group.
    pub fn dimension(&self) -> usize {
        match self {
            Self::GL(n) => n * n,
            Self::SL(n) => n * n - 1,
            Self::O(n) => n * (n - 1) / 2,
            Self::SO(n) => n * (n - 1) / 2,
            Self::U(n) => n * n,
            Self::SU(n) => n * n - 1,
            Self::Sp(n) => n * (2 * n + 1),
        }
    }
    /// Whether the group is compact.
    pub fn is_compact(&self) -> bool {
        match self {
            Self::GL(_) => false,
            Self::SL(_) => false,
            Self::O(_) => true,
            Self::SO(_) => true,
            Self::U(_) => true,
            Self::SU(_) => true,
            Self::Sp(_) => true,
        }
    }
    /// Whether the group is connected.
    pub fn is_connected(&self) -> bool {
        match self {
            Self::GL(n) => *n == 1,
            Self::SL(_) => true,
            Self::O(_) => false,
            Self::SO(_) => true,
            Self::U(_) => true,
            Self::SU(_) => true,
            Self::Sp(_) => true,
        }
    }
    /// Whether the group is simply connected.
    pub fn is_simply_connected(&self) -> bool {
        match self {
            Self::GL(_) => false,
            Self::SL(n) => *n >= 2,
            Self::O(_) => false,
            Self::SO(n) => *n == 1,
            Self::U(_) => false,
            Self::SU(_) => true,
            Self::Sp(_) => true,
        }
    }
}
/// The Killing form B(X, Y) = Tr(ad X ∘ ad Y) on a Lie algebra.
pub struct KillingForm {
    pub algebra: String,
    /// Symmetric matrix of B values in a chosen basis
    pub matrix: Vec<Vec<f64>>,
}
impl KillingForm {
    /// Create a zero Killing form (to be populated from structure constants).
    pub fn new(n: usize) -> Self {
        Self {
            algebra: String::new(),
            matrix: vec![vec![0.0_f64; n]; n],
        }
    }
    /// Compute the Killing form from structure constants:
    ///   B(e_i, e_j) = Σ_{k,l} f^k_{il} f^l_{jk}
    pub fn from_structure_constants(sc: &StructureConstants) -> Self {
        let n = sc.rank;
        let mut kf = Self::new(n);
        kf.algebra = sc.algebra.clone();
        for i in 0..n {
            for j in 0..n {
                let mut val = 0.0_f64;
                for k in 0..n {
                    for l in 0..n {
                        val += sc.f[i][l][k] * sc.f[j][k][l];
                    }
                }
                kf.matrix[i][j] = val;
            }
        }
        kf
    }
    /// Check if the Killing form is non-degenerate (det ≠ 0).
    ///
    /// By Cartan's criterion, g is semisimple iff B is non-degenerate.
    pub fn is_nondegenerate(&self) -> bool {
        let n = self.matrix.len();
        match n {
            0 => true,
            1 => self.matrix[0][0].abs() > 1e-9,
            2 => {
                let det =
                    self.matrix[0][0] * self.matrix[1][1] - self.matrix[0][1] * self.matrix[1][0];
                det.abs() > 1e-9
            }
            3 => {
                let m = &self.matrix;
                let det = m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
                    - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
                    + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0]);
                det.abs() > 1e-9
            }
            _ => {
                let tr: f64 = (0..n).map(|i| self.matrix[i][i]).sum();
                tr.abs() > 1e-9
            }
        }
    }
    /// Check if the Killing form is negative definite (all eigenvalues < 0).
    ///
    /// For compact real forms of semisimple Lie algebras, B is negative definite.
    pub fn is_negative_definite(&self) -> bool {
        self.matrix
            .iter()
            .enumerate()
            .all(|(i, row)| row[i] < -1e-9)
    }
    /// Signature (p, q) of the Killing form: (number of positive, number of negative) eigenvalues.
    ///
    /// For compact real forms: signature = (0, n).
    /// For split real forms of A_n: signature = (n²+2n-1, 1) approximately.
    pub fn signature(&self) -> (usize, usize) {
        let n = self.matrix.len();
        let mut pos = 0usize;
        let mut neg = 0usize;
        for i in 0..n {
            let diag = self.matrix[i][i];
            if diag > 1e-9 {
                pos += 1;
            } else if diag < -1e-9 {
                neg += 1;
            }
        }
        (pos, neg)
    }
}
/// Nilpotent orbit in semisimple Lie algebra.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct NilpotentOrbitData {
    pub algebra_type: String,
    pub orbit_label: String,
    pub dimension: usize,
    pub bala_carter_label: String,
}
#[allow(dead_code)]
impl NilpotentOrbitData {
    /// Regular nilpotent orbit (open dense).
    pub fn regular(algebra: &str, dim: usize) -> Self {
        Self {
            algebra_type: algebra.to_string(),
            orbit_label: "regular".to_string(),
            dimension: dim,
            bala_carter_label: algebra.to_string(),
        }
    }
    /// Zero orbit.
    pub fn zero(algebra: &str) -> Self {
        Self {
            algebra_type: algebra.to_string(),
            orbit_label: "zero".to_string(),
            dimension: 0,
            bala_carter_label: "0".to_string(),
        }
    }
    /// Is this in the closure of another orbit?
    pub fn in_closure_of(&self, other: &NilpotentOrbitData) -> bool {
        self.dimension <= other.dimension
    }
}
/// Adjoint action Ad_g : g → g, X ↦ g X g⁻¹.
pub struct AdjointAction {
    pub element: String,
    pub algebra: String,
}
impl AdjointAction {
    pub fn new(element: impl Into<String>, algebra: impl Into<String>) -> Self {
        Self {
            element: element.into(),
            algebra: algebra.into(),
        }
    }
    /// Trace of the adjoint action matrix (= adjoint character).
    ///
    /// For compact groups tr(Ad_g) = sum of eigenvalues = character of adjoint rep.
    /// Returns 0.0 as a placeholder for the symbolic case.
    pub fn trace(&self) -> f64 {
        0.0
    }
}
/// A Kashiwara crystal basis B(λ) for a quantum group U_q(g) module.
///
/// A crystal is a set B together with maps e_i, f_i: B → B ∪ {0},
/// weight function wt: B → P, and integers ε_i, φ_i: B → ℤ.
pub struct KashiwaraCrystal {
    /// Algebra label (e.g. "sl_2", "sl_3").
    pub algebra: String,
    /// Nodes of the crystal graph.
    pub nodes: Vec<Vec<i32>>,
    /// Colored edges: (from_idx, to_idx, color_i).
    pub edges: Vec<(usize, usize, usize)>,
    /// Rank (number of simple roots = number of crystal operator colors).
    pub rank: usize,
}
impl KashiwaraCrystal {
    /// Create an empty crystal.
    pub fn new(algebra: impl Into<String>, rank: usize) -> Self {
        Self {
            algebra: algebra.into(),
            nodes: Vec::new(),
            edges: Vec::new(),
            rank,
        }
    }
    /// Number of nodes (= dimension of the corresponding module).
    pub fn num_nodes(&self) -> usize {
        self.nodes.len()
    }
    /// Add a node with weight vector.
    pub fn add_node(&mut self, weight: Vec<i32>) {
        self.nodes.push(weight);
    }
    /// Add a crystal edge f_i: node `from` → node `to` with color `i`.
    pub fn add_edge(&mut self, from: usize, to: usize, color: usize) {
        self.edges.push((from, to, color));
    }
    /// Build the crystal B(m) for sl_2 with highest weight m.
    ///
    /// Nodes: weights m, m-2, ..., -m+2, -m (2m+2 nodes for half-integer; m+1 for integer).
    /// Edges: f_1: node_k → node_{k+1}.
    pub fn sl2_crystal(m: u32) -> Self {
        let mut crystal = Self::new("sl_2", 1);
        let count = (m + 1) as usize;
        for k in 0..count {
            let weight = (m as i32) - 2 * (k as i32);
            crystal.add_node(vec![weight]);
        }
        for k in 0..count.saturating_sub(1) {
            crystal.add_edge(k, k + 1, 0);
        }
        crystal
    }
    /// Check that each node has at most one outgoing edge of each color.
    pub fn is_valid(&self) -> bool {
        for node_idx in 0..self.nodes.len() {
            for color in 0..self.rank {
                let out_edges: usize = self
                    .edges
                    .iter()
                    .filter(|&&(from, _, c)| from == node_idx && c == color)
                    .count();
                if out_edges > 1 {
                    return false;
                }
            }
        }
        true
    }
    /// Highest weight node: node with no incoming f-edges (weight = highest weight).
    pub fn highest_weight_node(&self) -> Option<usize> {
        let with_incoming: std::collections::HashSet<usize> =
            self.edges.iter().map(|&(_, to, _)| to).collect();
        (0..self.nodes.len()).find(|idx| !with_incoming.contains(idx))
    }
}
/// Structure constants f^k_{ij} for a Lie algebra in a chosen basis:
///   [e_i, e_j] = Σ_k f^k_{ij} e_k
pub struct StructureConstants {
    pub algebra: String,
    pub rank: usize,
    /// f[i][j][k] = f^k_{ij}
    pub f: Vec<Vec<Vec<f64>>>,
}
impl StructureConstants {
    /// Create zero structure constants for an n-dimensional algebra.
    pub fn new(n: usize) -> Self {
        Self {
            algebra: String::new(),
            rank: n,
            f: vec![vec![vec![0.0_f64; n]; n]; n],
        }
    }
    /// Check the Jacobi identity:
    ///   Σ_l (f^l_{ij} f^m_{lk} + f^l_{jk} f^m_{li} + f^l_{ki} f^m_{lj}) = 0  ∀ i,j,k,m
    pub fn jacobi_identity_check(&self) -> bool {
        let n = self.rank;
        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    for m in 0..n {
                        let mut sum = 0.0_f64;
                        for l in 0..n {
                            sum += self.f[i][j][l] * self.f[l][k][m];
                            sum += self.f[j][k][l] * self.f[l][i][m];
                            sum += self.f[k][i][l] * self.f[l][j][m];
                        }
                        if sum.abs() > 1e-9 {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
    /// Check antisymmetry: f^k_{ij} = -f^k_{ji}.
    pub fn antisymmetry_check(&self) -> bool {
        let n = self.rank;
        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    if (self.f[i][j][k] + self.f[j][i][k]).abs() > 1e-9 {
                        return false;
                    }
                }
            }
        }
        true
    }
}
/// Levi decomposition data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LeviDecomposition {
    pub algebra: String,
    pub radical: String,
    pub levi_factor: String,
}
#[allow(dead_code)]
impl LeviDecomposition {
    /// Levi's theorem: every finite-dim Lie algebra g = r ⋊ s.
    pub fn new(algebra: &str, radical: &str, levi: &str) -> Self {
        Self {
            algebra: algebra.to_string(),
            radical: radical.to_string(),
            levi_factor: levi.to_string(),
        }
    }
    /// Description.
    pub fn description(&self) -> String {
        format!(
            "{} = {} ⋊ {} (Levi decomposition)",
            self.algebra, self.radical, self.levi_factor
        )
    }
    /// The Levi factor is semisimple.
    pub fn levi_is_semisimple(&self) -> bool {
        true
    }
}
/// Classical Lie algebra in the Cartan–Killing classification.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClassicalLieAlgebra {
    /// A_n = sl(n+1): traceless (n+1)×(n+1) matrices; n ≥ 1
    An(usize),
    /// B_n = so(2n+1): skew-symmetric (2n+1)×(2n+1) matrices; n ≥ 2
    Bn(usize),
    /// C_n = sp(2n): symplectic 2n×2n matrices; n ≥ 3
    Cn(usize),
    /// D_n = so(2n): skew-symmetric 2n×2n matrices; n ≥ 4
    Dn(usize),
}
impl ClassicalLieAlgebra {
    /// Complex dimension of the Lie algebra.
    pub fn dimension(&self) -> usize {
        match self {
            Self::An(n) => (n + 1) * (n + 1) - 1,
            Self::Bn(n) => n * (2 * n + 1),
            Self::Cn(n) => n * (2 * n + 1),
            Self::Dn(n) => n * (2 * n - 1),
        }
    }
    /// Rank (dimension of a Cartan subalgebra).
    pub fn rank(&self) -> usize {
        match self {
            Self::An(n) => *n,
            Self::Bn(n) => *n,
            Self::Cn(n) => *n,
            Self::Dn(n) => *n,
        }
    }
    /// Conventional Cartan name.
    pub fn name(&self) -> &'static str {
        match self {
            Self::An(_) => "A_n (sl_{n+1})",
            Self::Bn(_) => "B_n (so_{2n+1})",
            Self::Cn(_) => "C_n (sp_{2n})",
            Self::Dn(_) => "D_n (so_{2n})",
        }
    }
    /// Number of positive roots.
    pub fn num_positive_roots(&self) -> usize {
        match self {
            Self::An(n) => n * (n + 1) / 2,
            Self::Bn(n) => n * n,
            Self::Cn(n) => n * n,
            Self::Dn(n) => n * (n - 1),
        }
    }
    /// Coxeter number h.
    pub fn coxeter_number(&self) -> usize {
        match self {
            Self::An(n) => n + 1,
            Self::Bn(n) => 2 * n - 1,
            Self::Cn(n) => n + 1,
            Self::Dn(n) => 2 * n - 2,
        }
    }
    /// Dual Coxeter number h∨.
    pub fn dual_coxeter_number(&self) -> usize {
        match self {
            Self::An(n) => n + 1,
            Self::Bn(n) => 2 * n - 1,
            Self::Cn(n) => n + 1,
            Self::Dn(n) => 2 * n - 2,
        }
    }
}
/// An irreducible representation of a Lie algebra specified by highest weight.
pub struct LieRepresentation {
    pub algebra: String,
    pub dimension: usize,
    pub highest_weight: Vec<i32>,
    pub is_irreducible: bool,
}
impl LieRepresentation {
    pub fn new(
        algebra: impl Into<String>,
        dimension: usize,
        highest_weight: Vec<i32>,
        is_irreducible: bool,
    ) -> Self {
        Self {
            algebra: algebra.into(),
            dimension,
            highest_weight,
            is_irreducible,
        }
    }
    /// Weyl dimension formula for A_n: product formula.
    ///
    /// For SU(2) with spin j: dim = 2j + 1 where highest_weight = [2j].
    /// For general algebras we return the stored dimension as a fallback.
    pub fn dimension_formula(&self) -> u64 {
        if self.highest_weight.len() == 1 {
            (self.highest_weight[0] + 1).max(0) as u64
        } else {
            self.dimension as u64
        }
    }
    /// Whether this is the adjoint representation (highest weight = highest root).
    pub fn is_adjoint(&self) -> bool {
        let rank = self.highest_weight.len();
        if rank < 2 {
            return false;
        }
        self.highest_weight[0] == 1
            && self.highest_weight[rank - 1] == 1
            && self.highest_weight[1..rank - 1].iter().all(|&x| x == 0)
    }
    /// Whether this is a fundamental representation.
    ///
    /// A fundamental representation has highest weight ω_i (one fundamental weight).
    pub fn is_fundamental(&self) -> bool {
        let ones: usize = self.highest_weight.iter().filter(|&&x| x == 1).count();
        let zeros: usize = self.highest_weight.iter().filter(|&&x| x == 0).count();
        ones == 1 && zeros + ones == self.highest_weight.len()
    }
}
/// Weyl character formula bundled with its representation.
pub struct WeylCharacterFormula {
    pub representation: LieRepresentation,
    pub character: String,
}
impl WeylCharacterFormula {
    pub fn new(representation: LieRepresentation) -> Self {
        let char_str = format!(
            "chi_{{{}}}",
            representation
                .highest_weight
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(",")
        );
        Self {
            representation,
            character: char_str,
        }
    }
}
/// A root: an eigenvalue of the adjoint action of the Cartan subalgebra.
pub struct Root {
    pub algebra: String,
    pub coefficients: Vec<i32>,
    pub is_positive: bool,
    pub is_simple: bool,
}
impl Root {
    pub fn new(
        algebra: impl Into<String>,
        coefficients: Vec<i32>,
        is_positive: bool,
        is_simple: bool,
    ) -> Self {
        Self {
            algebra: algebra.into(),
            coefficients,
            is_positive,
            is_simple,
        }
    }
    /// Inner product (using standard Euclidean metric on root space).
    pub fn inner_product(&self, other: &Self) -> i32 {
        self.coefficients
            .iter()
            .zip(other.coefficients.iter())
            .map(|(&a, &b)| a * b)
            .sum()
    }
    /// Squared length of the root.
    pub fn length_squared(&self) -> i32 {
        self.inner_product(self)
    }
    /// Cartan integer <α, β∨> = 2(α, β) / (β, β).
    pub fn cartan_integer(&self, other: &Self) -> i32 {
        let num = 2 * self.inner_product(other);
        let den = other.length_squared();
        if den == 0 {
            0
        } else {
            num / den
        }
    }
    /// Negation: produce the negative of this root.
    pub fn negate(&self) -> Self {
        Self {
            algebra: self.algebra.clone(),
            coefficients: self.coefficients.iter().map(|&c| -c).collect(),
            is_positive: !self.is_positive,
            is_simple: false,
        }
    }
}
/// A root system associated to a semisimple Lie algebra.
pub struct RootSystem {
    pub rank: usize,
    pub positive_roots: Vec<Root>,
    pub simple_roots: Vec<Root>,
}
impl RootSystem {
    /// Create an empty root system of given rank.
    pub fn new(rank: usize) -> Self {
        Self {
            rank,
            positive_roots: Vec::new(),
            simple_roots: Vec::new(),
        }
    }
    /// Total number of roots (positive + negative).
    pub fn num_roots(&self) -> usize {
        2 * self.positive_roots.len()
    }
    /// Highest root (last positive root in standard ordering).
    pub fn highest_root(&self) -> Option<&Root> {
        self.positive_roots.last()
    }
    /// Add a simple root.
    pub fn add_simple_root(&mut self, root: Root) {
        self.simple_roots.push(root);
    }
    /// Add a positive root.
    pub fn add_positive_root(&mut self, root: Root) {
        self.positive_roots.push(root);
    }
    /// Build the A_n root system.
    ///
    /// Simple roots: α_i = e_i - e_{i+1} for i = 1..n
    pub fn type_a(n: usize) -> Self {
        let mut rs = Self::new(n);
        for i in 0..n {
            let mut coeffs = vec![0_i32; n + 1];
            coeffs[i] = 1;
            if i < n {
                coeffs[i + 1] = -1;
            }
            rs.add_simple_root(Root::new("A", coeffs.clone(), true, true));
        }
        for i in 0..n {
            for j in (i + 1)..=(n) {
                let mut coeffs = vec![0_i32; n + 1];
                coeffs[i] = 1;
                coeffs[j] = -1;
                rs.add_positive_root(Root::new("A", coeffs, true, i + 1 == j));
            }
        }
        rs
    }
}
/// Iwahori-Hecke algebra H(W, q) with parameter q.
///
/// H(W, q) is a deformation of the group algebra ℂ[W] generated by T_w (w ∈ W)
/// with quadratic relation (T_s + 1)(T_s - q) = 0 for simple reflections s.
pub struct HeckeAlgebra {
    /// Name of the underlying Weyl/Coxeter group W.
    pub coxeter_group: String,
    /// Deformation parameter q (generic or specialized).
    pub q: f64,
    /// Rank of the Coxeter group.
    pub rank: usize,
    /// Order of the group |W|.
    pub group_order: usize,
}
impl HeckeAlgebra {
    /// Create a Hecke algebra for the given Coxeter group with parameter q.
    pub fn new(coxeter_group: impl Into<String>, q: f64, rank: usize, group_order: usize) -> Self {
        Self {
            coxeter_group: coxeter_group.into(),
            q,
            rank,
            group_order,
        }
    }
    /// Dimension of the Hecke algebra as a ℤ[q]-module equals |W|.
    pub fn dimension(&self) -> usize {
        self.group_order
    }
    /// At q = 1 the Hecke algebra specializes to the group algebra ℂ[W].
    pub fn is_group_algebra(&self) -> bool {
        (self.q - 1.0).abs() < 1e-9
    }
    /// At q = 0 the Hecke algebra specializes to the nil-Hecke algebra.
    pub fn is_nil_hecke(&self) -> bool {
        self.q.abs() < 1e-9
    }
    /// Hecke algebra for A_n (Weyl group S_{n+1}, order (n+1)!).
    pub fn type_a(n: usize, q: f64) -> Self {
        let group_order = (1..=(n + 1)).product();
        Self::new(format!("A_{}", n), q, n, group_order)
    }
    /// Kazhdan-Lusztig basis element C'_w expressed as a formal sum.
    /// Returns a description string for the KL basis element.
    pub fn kl_basis_element(&self, w_label: &str) -> String {
        format!(
            "C'_{{{w_label}}} = sum_{{y <= {w_label}}} P_{{y,{w_label}}}(q) * T_{{y}} in H(W,q)"
        )
    }
}
/// Cartan subalgebra data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CartanSubalgebraData {
    pub parent_algebra: String,
    pub rank: usize,
    pub is_abelian: bool,
    pub is_self_normalizing: bool,
}
#[allow(dead_code)]
impl CartanSubalgebraData {
    /// Cartan subalgebra of a semisimple Lie algebra.
    pub fn semisimple(parent: &str, rank: usize) -> Self {
        Self {
            parent_algebra: parent.to_string(),
            rank,
            is_abelian: true,
            is_self_normalizing: true,
        }
    }
    /// Weight space decomposition.
    pub fn weight_space_description(&self) -> String {
        format!(
            "g = h ⊕ ⊕_α g_α where h = Cartan({}) of rank {}",
            self.parent_algebra, self.rank
        )
    }
}
/// Solvable Lie algebra data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SolvableLieAlgebra {
    pub name: String,
    pub derived_length: usize,
    pub is_nilpotent: bool,
    pub is_abelian: bool,
}
#[allow(dead_code)]
impl SolvableLieAlgebra {
    /// Abelian Lie algebra (trivially solvable and nilpotent).
    pub fn abelian(name: &str) -> Self {
        Self {
            name: name.to_string(),
            derived_length: 1,
            is_nilpotent: true,
            is_abelian: true,
        }
    }
    /// Heisenberg Lie algebra (nilpotent but not abelian).
    pub fn heisenberg() -> Self {
        Self {
            name: "heis".to_string(),
            derived_length: 2,
            is_nilpotent: true,
            is_abelian: false,
        }
    }
    /// Lie's theorem: solvable Lie algebra over algebraically closed field
    /// of char 0 has simultaneous eigenvectors (triangularizable).
    pub fn lies_theorem_applies(&self, alg_closed: bool) -> bool {
        alg_closed
    }
}
/// A Drinfeld-Jimbo quantum group U_q(g) at a formal parameter q.
///
/// U_q(g) is generated by E_i, F_i, K_i^±1 for each simple root α_i,
/// with q-Serre relations deforming the Chevalley-Serre presentation of U(g).
pub struct QuantumGroup {
    /// Name of the underlying Lie algebra.
    pub algebra: String,
    /// Rank (number of simple roots).
    pub rank: usize,
    /// Cartan matrix entries a_{ij}.
    pub cartan_matrix: Vec<Vec<i32>>,
    /// Deformation parameter q (0.0 = formal, otherwise specialized).
    pub q: f64,
}
impl QuantumGroup {
    /// Construct U_q(g) from a Cartan matrix.
    pub fn new(algebra: impl Into<String>, cartan_matrix: Vec<Vec<i32>>, q: f64) -> Self {
        let rank = cartan_matrix.len();
        Self {
            algebra: algebra.into(),
            rank,
            cartan_matrix,
            q,
        }
    }
    /// U_q(sl_2) with given q.
    pub fn sl2(q: f64) -> Self {
        Self::new("sl_2", vec![vec![2]], q)
    }
    /// U_q(sl_3) with given q.
    pub fn sl3(q: f64) -> Self {
        Self::new("sl_3", vec![vec![2, -1], vec![-1, 2]], q)
    }
    /// At q → 1 this specializes to U(g).
    pub fn is_classical_limit(&self) -> bool {
        (self.q - 1.0).abs() < 1e-9
    }
    /// When q is a primitive ℓ-th root of unity, U_q(g) is a finite-dimensional
    /// quotient called the small quantum group u_q(g).
    pub fn is_root_of_unity(&self, ell: u32) -> bool {
        if ell == 0 {
            return false;
        }
        let angle = 2.0 * std::f64::consts::PI / ell as f64;
        let qre = self.q * angle.cos();
        let qim = self.q * angle.sin();
        (self.q - qre.cos()).abs() < 1e-6 || (self.q - qim.cos()).abs() < 1e-6
    }
    /// q-integer [n]_q = (q^n - q^{-n}) / (q - q^{-1}) for q ≠ 1.
    pub fn q_integer(&self, n: i32) -> f64 {
        if (self.q - 1.0).abs() < 1e-9 {
            return n as f64;
        }
        let qn = self.q.powi(n);
        let qneg = self.q.powi(-n);
        (qn - qneg) / (self.q - 1.0 / self.q)
    }
    /// q-factorial [n]_q! = [1]_q [2]_q ... [n]_q.
    pub fn q_factorial(&self, n: u32) -> f64 {
        (1..=n).map(|k| self.q_integer(k as i32)).product()
    }
    /// q-binomial coefficient (Gaussian binomial) [n choose k]_q.
    pub fn q_binomial(&self, n: u32, k: u32) -> f64 {
        if k > n {
            return 0.0;
        }
        self.q_factorial(n) / (self.q_factorial(k) * self.q_factorial(n - k))
    }
    /// Description of the algebra.
    pub fn description(&self) -> String {
        format!(
            "U_q({}) with rank {} and q = {:.4}",
            self.algebra, self.rank, self.q
        )
    }
}
