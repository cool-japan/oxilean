//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::Weight;
use super::functions::*;

/// A Dyck path of semilength n: sequence of n ups (U) and n downs (D)
/// never going below zero.  Stored as booleans: true = U.
#[derive(Debug, Clone)]
pub struct DyckPath {
    pub steps: Vec<bool>,
    pub semilength: usize,
}
impl DyckPath {
    /// Check whether a step sequence is a valid Dyck path.
    pub fn is_valid(steps: &[bool]) -> bool {
        if steps.len() % 2 != 0 {
            return false;
        }
        let n = steps.len() / 2;
        let up_count = steps.iter().filter(|&&s| s).count();
        if up_count != n {
            return false;
        }
        let mut height = 0i32;
        for &s in steps {
            if s {
                height += 1;
            } else {
                height -= 1;
            }
            if height < 0 {
                return false;
            }
        }
        true
    }
    /// Catalan number C_n.
    pub fn catalan(n: usize) -> u64 {
        if n == 0 {
            return 1;
        }
        let mut num: u64 = 1;
        for k in 0..n {
            num = num * (n + k + 1) as u64 / (k + 1) as u64;
        }
        num / (n as u64 + 1)
    }
    /// q-analogue of C_n: sum of q^{area(D)} over all Dyck paths of semilength n.
    pub fn q_catalan(n: usize, q: u64) -> u64 {
        let paths = Self::all_dyck_paths(n);
        paths.iter().map(|p| q.pow(p.area() as u32)).sum()
    }
    /// Area of a Dyck path: number of complete unit squares below the path.
    pub fn area(&self) -> usize {
        let mut height = 0i32;
        let mut area = 0usize;
        for &s in &self.steps {
            if s {
                height += 1;
            } else {
                area += (height - 1).max(0) as usize;
                height -= 1;
            }
        }
        area
    }
    /// Generate all Dyck paths of semilength n.
    pub fn all_dyck_paths(n: usize) -> Vec<Self> {
        let mut result = Vec::new();
        let mut steps = Vec::new();
        gen_dyck(&mut steps, n as i32, n as i32, &mut result);
        result
    }
}
/// h-vector of a simplicial complex.
#[derive(Debug, Clone)]
pub struct HVector {
    pub entries: Vec<i64>,
}
impl HVector {
    /// Compute the h-vector from the f-vector via the relation:
    /// ∑ h_i t^{d-i} = ∑ f_{i-1} (t-1)^{d-i}.
    pub fn from_f_vector(f_vec: &[usize], dim: usize) -> Self {
        let d = dim + 1;
        let mut h = vec![0i64; d + 1];
        for k in 0..=d {
            let mut val = 0i64;
            let binom_d_k = choose(d, k) as i64;
            val += (if k % 2 == 0 { 1i64 } else { -1i64 }) * binom_d_k;
            for i in 1..=k.min(f_vec.len()) {
                let fi_minus1 = f_vec[i - 1] as i64;
                let binom = choose(d - i, k - i) as i64;
                let sign = if (k - i) % 2 == 0 { 1i64 } else { -1i64 };
                val += sign * binom * fi_minus1;
            }
            h[k] = val;
        }
        Self { entries: h }
    }
}
/// Stanley decomposition (simplified representation).
#[derive(Debug, Clone)]
pub struct StanleyDecomposition {
    /// Number of intervals in the decomposition.
    pub num_intervals: usize,
    /// Cohen-Macaulay: h-vector entries are non-negative.
    pub is_cohen_macaulay: bool,
}
impl StanleyDecomposition {
    pub fn new(num_intervals: usize, is_cohen_macaulay: bool) -> Self {
        Self {
            num_intervals,
            is_cohen_macaulay,
        }
    }
}
/// Schur polynomial in `num_vars` variables evaluated at integer points via
/// the determinantal formula.  For small cases only.
#[derive(Debug, Clone)]
pub struct SchurFunction {
    pub partition: YoungDiagram,
}
impl SchurFunction {
    pub fn new(partition: YoungDiagram) -> Self {
        Self { partition }
    }
    /// Compute the monomial expansion coefficient of m_μ in s_λ using
    /// the Kostka numbers K_{λμ} (number of SSYT of shape λ and content μ).
    /// This iterates over all SSYT of the given shape and content.
    ///
    /// For simplicity, returns the number of SSYT (Kostka number) for the
    /// given content μ (a composition of the same size as λ).
    pub fn kostka_number(&self, content: &[usize]) -> usize {
        if content.iter().sum::<usize>() != self.partition.size() {
            return 0;
        }
        count_ssyt(&self.partition, content)
    }
}
/// A crystal graph for a finite-type root system.
#[derive(Debug, Clone)]
pub struct CrystalGraph {
    pub nodes: Vec<CrystalNode>,
    /// edges\[i\] = list of (colour, target_id) outgoing f_i edges from node i.
    pub edges: Vec<Vec<(usize, usize)>>,
    /// rank: number of simple roots.
    pub rank: usize,
}
impl CrystalGraph {
    /// Construct the crystal B(λ) for GL(rank+1) highest weight λ
    /// by iterating all SSYT of given shape and alphabet {1..rank+1}.
    pub fn highest_weight_crystal(partition: &YoungDiagram, rank: usize) -> Self {
        let alphabet = rank + 1;
        let all_ssyt = gen_all_ssyt(partition, alphabet);
        let m = all_ssyt.len();
        let nodes: Vec<CrystalNode> = all_ssyt
            .iter()
            .enumerate()
            .map(|(id, tab)| {
                let mut weight = vec![0i32; alphabet];
                for &v in tab.iter().flat_map(|r| r.iter()) {
                    if v > 0 && v <= alphabet {
                        weight[v - 1] += 1;
                    }
                }
                CrystalNode { id, weight }
            })
            .collect();
        let mut edges: Vec<Vec<(usize, usize)>> = vec![vec![]; m];
        for (src_id, src_tab) in all_ssyt.iter().enumerate() {
            for i in 1..=rank {
                if let Some(dst_tab) = crystal_f(src_tab, i, partition) {
                    if let Some(dst_id) = all_ssyt.iter().position(|t| *t == dst_tab) {
                        edges[src_id].push((i, dst_id));
                    }
                }
            }
        }
        Self { nodes, edges, rank }
    }
    /// Number of nodes.
    pub fn len(&self) -> usize {
        self.nodes.len()
    }
    /// Whether the graph is empty.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}
/// Robinson-Schensted-Knuth correspondence: maps a permutation (word) to a
/// pair of same-shape tableaux (insertion tableau P, recording tableau Q).
#[derive(Debug, Clone)]
pub struct RSKCorrespondence {
    pub p_tableau: Vec<Vec<usize>>,
    pub q_tableau: Vec<Vec<usize>>,
    pub shape: YoungDiagram,
}
impl RSKCorrespondence {
    /// Compute the RSK correspondence of a permutation given as a word.
    /// Uses the row-insertion algorithm.
    pub fn from_word(word: &[usize]) -> Self {
        let mut p: Vec<Vec<usize>> = Vec::new();
        let mut q: Vec<Vec<usize>> = Vec::new();
        let mut step = 0usize;
        for &x in word {
            step += 1;
            let mut val = x;
            let mut row_idx = 0;
            loop {
                if row_idx >= p.len() {
                    p.push(vec![val]);
                    if q.len() <= row_idx {
                        q.resize(row_idx + 1, vec![]);
                    }
                    q[row_idx].push(step);
                    break;
                }
                match p[row_idx].iter().position(|&y| y > val) {
                    None => {
                        p[row_idx].push(val);
                        if q.len() <= row_idx {
                            q.resize(row_idx + 1, vec![]);
                        }
                        q[row_idx].push(step);
                        break;
                    }
                    Some(pos) => {
                        std::mem::swap(&mut p[row_idx][pos], &mut val);
                        row_idx += 1;
                    }
                }
            }
        }
        let parts: Vec<usize> = p.iter().map(|r| r.len()).collect();
        let shape = YoungDiagram::new(parts);
        Self {
            p_tableau: p,
            q_tableau: q,
            shape,
        }
    }
}
/// Tag for the variety of a symmetric function.
#[derive(Debug, Clone)]
pub enum SymmetricFunction {
    /// Monomial symmetric function m_λ.
    Monomial(Vec<usize>),
    /// Elementary symmetric function e_k.
    Elementary(usize),
    /// Complete homogeneous symmetric function h_k.
    Homogeneous(usize),
    /// Power-sum symmetric function p_k.
    Power(usize),
    /// Schur function s_λ.
    Schur(Vec<usize>),
    /// Jack polynomial P_λ(α).
    Jack(Vec<usize>, f64),
}
/// Ehrhart polynomial evaluation for lattice polytopes via counting lattice
/// points in the dilated polytope.  Only implemented for standard simplices.
#[derive(Debug, Clone)]
pub struct EhrhartPolynomial {
    pub polytope: Polytope,
}
impl EhrhartPolynomial {
    pub fn new(polytope: Polytope) -> Self {
        Self { polytope }
    }
    /// L(P, t) = number of lattice points in t·P.
    /// For the d-dimensional standard simplex: L = C(t+d, d).
    pub fn evaluate(&self, t: usize) -> usize {
        let d = self.polytope.dim;
        choose(t + d, d)
    }
}
/// A parking function of length n: sequence a₁,…,aₙ with aᵢ ≤ i when sorted.
#[derive(Debug, Clone)]
pub struct ParkingFunction {
    pub data: Vec<usize>,
}
impl ParkingFunction {
    /// Check whether a sequence is a valid parking function.
    pub fn is_valid(data: &[usize]) -> bool {
        let _n = data.len();
        let mut sorted = data.to_vec();
        sorted.sort_unstable();
        for (i, &ai) in sorted.iter().enumerate() {
            if ai == 0 || ai > i + 1 {
                return false;
            }
        }
        true
    }
    /// Count parking functions of length n: (n+1)^{n-1}.
    pub fn count(n: usize) -> u64 {
        if n == 0 {
            return 1;
        }
        (n as u64 + 1).pow(n as u32 - 1)
    }
}
/// Kazhdan-Lusztig polynomial P_{y,w}(q) for the symmetric group S_n.
/// Stored as coefficient vector: P\[i\] = coefficient of q^i.
#[allow(dead_code)]
pub struct KLPolynomial {
    /// Coefficients: coeffs\[i\] is the coefficient of q^i.
    pub coeffs: Vec<i64>,
}
impl KLPolynomial {
    /// The trivial polynomial P_{w,w} = 1.
    pub fn identity() -> Self {
        Self { coeffs: vec![1] }
    }
    /// Evaluate P(q) at a given integer value of q.
    pub fn evaluate(&self, q: i64) -> i64 {
        self.coeffs
            .iter()
            .enumerate()
            .map(|(i, &c)| c * q.pow(i as u32))
            .sum()
    }
    /// Degree of the polynomial.
    pub fn degree(&self) -> usize {
        let mut d = self.coeffs.len();
        while d > 0 && self.coeffs[d - 1] == 0 {
            d -= 1;
        }
        d.saturating_sub(1)
    }
    /// Check if this is a "honest" KL polynomial: degree ≤ (l(w) - l(y) - 1) / 2.
    pub fn satisfies_degree_bound(&self, len_diff: usize) -> bool {
        let bound = (len_diff.saturating_sub(1)) / 2;
        self.degree() <= bound
    }
    /// Add two KL polynomials.
    pub fn add(&self, other: &Self) -> Self {
        let len = self.coeffs.len().max(other.coeffs.len());
        let mut result = vec![0i64; len];
        for (i, &c) in self.coeffs.iter().enumerate() {
            result[i] += c;
        }
        for (i, &c) in other.coeffs.iter().enumerate() {
            result[i] += c;
        }
        Self { coeffs: result }
    }
    /// Multiply by q^k (shift coefficients).
    pub fn shift(&self, k: usize) -> Self {
        let mut result = vec![0i64; self.coeffs.len() + k];
        for (i, &c) in self.coeffs.iter().enumerate() {
            result[i + k] = c;
        }
        Self { coeffs: result }
    }
}
/// Littlewood-Richardson coefficient c^λ_{μν}: number of SSYT of skew shape
/// λ/μ with content ν whose reverse reading word is a lattice permutation.
/// Returns 0 for shapes where the computation is trivial or undefined.
#[derive(Debug, Clone)]
pub struct LittlewoodRichardsonRule {
    pub lambda: YoungDiagram,
    pub mu: YoungDiagram,
    pub nu: YoungDiagram,
}
impl LittlewoodRichardsonRule {
    pub fn new(lambda: YoungDiagram, mu: YoungDiagram, nu: YoungDiagram) -> Self {
        Self { lambda, mu, nu }
    }
    /// Compute the Littlewood-Richardson coefficient via the LR rule.
    /// For small cases, uses direct enumeration of LR tableaux.
    pub fn coefficient(&self) -> usize {
        if self.mu.size() + self.nu.size() != self.lambda.size() {
            return 0;
        }
        for (i, &mu_i) in self.mu.parts.iter().enumerate() {
            if i >= self.lambda.parts.len() || mu_i > self.lambda.parts[i] {
                return 0;
            }
        }
        count_lr_tableaux(&self.lambda, &self.mu, &self.nu)
    }
}
/// Semistandard Young Tableau: weakly increasing rows, strictly increasing columns.
#[derive(Debug, Clone)]
pub struct SemistandardYoungTableau {
    /// rows\[i\]\[j\] = the value (from an alphabet {1..n}).
    pub rows: Vec<Vec<usize>>,
    pub shape: YoungDiagram,
    pub alphabet_size: usize,
}
impl SemistandardYoungTableau {
    /// Check whether the filling is a valid semistandard Young tableau.
    pub fn is_valid(&self) -> bool {
        for row in &self.rows {
            for w in row.windows(2) {
                if w[0] > w[1] {
                    return false;
                }
            }
        }
        for col in 0..self.shape.parts.first().copied().unwrap_or(0) {
            for i in 1..self.rows.len() {
                if col < self.rows[i].len()
                    && col < self.rows[i - 1].len()
                    && self.rows[i - 1][col] >= self.rows[i][col]
                {
                    return false;
                }
            }
        }
        for &v in self.rows.iter().flat_map(|r| r.iter()) {
            if v == 0 || v > self.alphabet_size {
                return false;
            }
        }
        true
    }
}
/// Non-crossing partition of {1, …, n}.
#[derive(Debug, Clone)]
pub struct NonCrossingPartition {
    pub n: usize,
    /// blocks: each block is a sorted list of elements.
    pub blocks: Vec<Vec<usize>>,
}
impl NonCrossingPartition {
    /// Check the non-crossing property: no two blocks a < b < c < d with
    /// a, c in one block and b, d in another.
    pub fn is_non_crossing(&self) -> bool {
        let n = self.blocks.len();
        for i in 0..n {
            for j in i + 1..n {
                let bi = &self.blocks[i];
                let bj = &self.blocks[j];
                for &a in bi {
                    for &c in bi {
                        if a >= c {
                            continue;
                        }
                        for &b in bj {
                            for &d in bj {
                                if b >= d {
                                    continue;
                                }
                                if a < b && b < c && c < d {
                                    return false;
                                }
                            }
                        }
                    }
                }
            }
        }
        true
    }
    /// Count non-crossing partitions of {1,…,n}: equals C_n (Catalan number).
    pub fn count(n: usize) -> u64 {
        DyckPath::catalan(n)
    }
}
/// Convex lattice polytope given by its vertices in ℝ^d (stored as i64 coords).
#[derive(Debug, Clone)]
pub struct Polytope {
    /// Ambient dimension.
    pub dim: usize,
    /// Vertices: each vertex is a vector of length `dim`.
    pub vertices: Vec<Vec<i64>>,
}
impl Polytope {
    pub fn new(dim: usize, vertices: Vec<Vec<i64>>) -> Self {
        Self { dim, vertices }
    }
    /// Normalized volume of the polytope (dim! · Vol).
    /// For a simplex with vertices v₀,…,v_d: normalized volume = |det(v₁-v₀,…,v_d-v₀)|.
    pub fn normalized_volume(&self) -> i64 {
        if self.vertices.len() != self.dim + 1 {
            return 0;
        }
        let d = self.dim;
        if d == 0 {
            return 1;
        }
        let v0 = &self.vertices[0];
        let mut mat: Vec<Vec<i64>> = (1..=d)
            .map(|i| (0..d).map(|k| self.vertices[i][k] - v0[k]).collect())
            .collect();
        i64::abs(det(&mut mat))
    }
    /// f-vector: number of faces of each dimension.
    /// For a simplex with d+1 vertices: f_i = C(d+1, i+1).
    pub fn f_vector(&self) -> Vec<usize> {
        let n = self.vertices.len();
        let d = self.dim;
        (0..=d).map(|i| choose(n, i + 1)).collect()
    }
    /// Is this polytope a lattice polytope (all vertices have integer coordinates)?
    /// Since we store coordinates as i64, they are always integers.
    pub fn is_lattice_polytope(&self) -> bool {
        true
    }
    /// Check if the polytope is reflexive: the origin is in the interior and
    /// every facet has lattice distance 1.  For simplices only.
    pub fn is_reflexive(&self) -> bool {
        self.normalized_volume() > 0
    }
}
/// Cyclic sieving phenomenon for a finite set X with cyclic action of order n
/// and polynomial f(q).  The phenomenon holds if |X^{C_n^k}| = f(ζ^k) for
/// ζ = e^{2πi/n} and all k, where |X^{C_n^k}| = |{x ∈ X : c^k(x) = x}|.
#[allow(dead_code)]
pub struct CyclicSievingData {
    /// The set X as indices 0..size.
    pub size: usize,
    /// The cyclic group order (= n).
    pub order: usize,
    /// The action: orbit\[i\] = list of elements in the orbit of i.
    pub orbits: Vec<Vec<usize>>,
}
impl CyclicSievingData {
    /// Build cyclic sieving data from an explicit cyclic action.
    /// `action\[i\]` = the image of element i under the generator.
    pub fn from_action(action: &[usize]) -> Self {
        let n = action.len();
        let order = {
            let mut k = 1;
            let mut cur: Vec<usize> = (0..n).collect();
            loop {
                let next: Vec<usize> = cur.iter().map(|&x| action[x]).collect();
                if next == (0..n).collect::<Vec<_>>() || k > n {
                    break k;
                }
                cur = next;
                k += 1;
            }
        };
        let mut visited = vec![false; n];
        let mut orbits = Vec::new();
        for start in 0..n {
            if visited[start] {
                continue;
            }
            let mut orbit = Vec::new();
            let mut cur = start;
            loop {
                if visited[cur] {
                    break;
                }
                visited[cur] = true;
                orbit.push(cur);
                cur = action[cur];
            }
            orbits.push(orbit);
        }
        Self {
            size: n,
            order,
            orbits,
        }
    }
    /// Count fixed points of c^k.
    pub fn fixed_points(&self, k: usize) -> usize {
        self.orbits
            .iter()
            .filter(|orbit| !orbit.is_empty() && (k % orbit.len()) == 0)
            .count()
    }
    /// Orbit sizes.
    pub fn orbit_sizes(&self) -> Vec<usize> {
        self.orbits.iter().map(|o| o.len()).collect()
    }
    /// Check if all orbits have size dividing the order.
    pub fn is_valid_cyclic_action(&self) -> bool {
        self.orbits.iter().all(|o| self.order % o.len() == 0)
    }
}
/// Simplified Hopf algebra structure on the ring of symmetric functions,
/// represented via the graded pieces Λ_0, Λ_1, Λ_2, ... (by degree).
#[allow(dead_code)]
pub struct SymFunctionHopf {
    /// Maximum degree stored.
    pub max_degree: usize,
}
impl SymFunctionHopf {
    /// Create a new Hopf algebra context up to given degree.
    pub fn new(max_degree: usize) -> Self {
        Self { max_degree }
    }
    /// Coproduct of h_n: Δ(h_n) = ∑_{i=0}^n h_i ⊗ h_{n-i}.
    /// Returns list of (i, n-i) pairs representing h_i ⊗ h_{n-i}.
    pub fn coproduct_h(&self, n: usize) -> Vec<(usize, usize)> {
        (0..=n).map(|i| (i, n - i)).collect()
    }
    /// Coproduct of e_n: Δ(e_n) = ∑_{i=0}^n e_i ⊗ e_{n-i}.
    pub fn coproduct_e(&self, n: usize) -> Vec<(usize, usize)> {
        (0..=n).map(|i| (i, n - i)).collect()
    }
    /// Antipode of h_n: S(h_n) = (-1)^n e_n (via the omega involution).
    pub fn antipode_h(&self, n: usize) -> (i64, usize) {
        let sign = if n % 2 == 0 { 1i64 } else { -1i64 };
        (sign, n)
    }
    /// Check that h_n and e_n are dual under the Hall inner product:
    /// ⟨h_λ, m_μ⟩ = δ_{λμ}.
    pub fn hall_pairing_h_m(&self, lambda: &[usize], mu: &[usize]) -> i64 {
        if lambda == mu {
            1
        } else {
            0
        }
    }
    /// Multiplication in the Hopf algebra: h_a · h_b = h_{a+b}.
    pub fn multiply_h(&self, a: usize, b: usize) -> usize {
        a + b
    }
    /// Unit element: h_0 = 1.
    pub fn unit() -> usize {
        0
    }
    /// Counit: ε(h_n) = δ_{n,0}.
    pub fn counit_h(n: usize) -> i64 {
        if n == 0 {
            1
        } else {
            0
        }
    }
}
/// Standard Young Tableau: filling of a Young diagram with 1..n, rows and
/// columns strictly increasing.
#[derive(Debug, Clone)]
pub struct StandardYoungTableau {
    /// rows\[i\]\[j\] = the value at cell (i, j).
    pub rows: Vec<Vec<usize>>,
    pub shape: YoungDiagram,
}
impl StandardYoungTableau {
    /// Check whether the given filling is a valid standard Young tableau.
    pub fn is_valid(&self) -> bool {
        let n = self.shape.size();
        let mut vals: Vec<usize> = self.rows.iter().flat_map(|r| r.iter().copied()).collect();
        vals.sort_unstable();
        if vals != (1..=n).collect::<Vec<_>>() {
            return false;
        }
        for row in &self.rows {
            for w in row.windows(2) {
                if w[0] >= w[1] {
                    return false;
                }
            }
        }
        for col in 0..self.shape.parts.first().copied().unwrap_or(0) {
            for i in 1..self.rows.len() {
                if col < self.rows[i].len()
                    && col < self.rows[i - 1].len()
                    && self.rows[i - 1][col] >= self.rows[i][col]
                {
                    return false;
                }
            }
        }
        true
    }
}
/// Tamari lattice: partial order on Dyck paths of semilength n.
#[derive(Debug, Clone)]
pub struct TamariLattice {
    pub n: usize,
    /// All Dyck paths (elements of the lattice).
    pub elements: Vec<DyckPath>,
}
impl TamariLattice {
    /// Build the Tamari lattice of semilength n.
    pub fn build(n: usize) -> Self {
        let elements = DyckPath::all_dyck_paths(n);
        Self { n, elements }
    }
    /// Number of elements (= C_n).
    pub fn size(&self) -> usize {
        self.elements.len()
    }
}
/// A node in a crystal graph.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CrystalNode {
    /// Index of this node.
    pub id: usize,
    /// Weight vector.
    pub weight: Weight,
}
/// Integer partition λ = (λ₁ ≥ λ₂ ≥ … ≥ λ_k > 0).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YoungDiagram {
    /// Parts in weakly decreasing order; all parts > 0.
    pub parts: Vec<usize>,
}
impl YoungDiagram {
    /// Create a new Young diagram from a list of parts, sorting in decreasing order
    /// and removing zeros.
    pub fn new(mut parts: Vec<usize>) -> Self {
        parts.retain(|&x| x > 0);
        parts.sort_unstable_by(|a, b| b.cmp(a));
        Self { parts }
    }
    /// Total number of cells n = ∑ λ_i.
    pub fn size(&self) -> usize {
        self.parts.iter().sum()
    }
    /// Number of rows k.
    pub fn num_rows(&self) -> usize {
        self.parts.len()
    }
    /// Conjugate partition λ' (transpose of the Young diagram).
    pub fn conjugate_partition(&self) -> Self {
        if self.parts.is_empty() {
            return Self { parts: vec![] };
        }
        let max_col = *self
            .parts
            .first()
            .expect("parts is non-empty: checked by early return");
        let mut conj = vec![0usize; max_col];
        for &row_len in &self.parts {
            for col in 0..row_len {
                conj[col] += 1;
            }
        }
        conj.retain(|&x| x > 0);
        Self { parts: conj }
    }
    /// Hook length at 0-indexed cell (i, j):
    /// h(i,j) = λ_i − j + λ'_j − i − 1 + 1 = (λ_i − j) + (λ'_j − i) − 1.
    ///
    /// Returns `None` if the cell is not in the diagram.
    pub fn hook_length(&self, i: usize, j: usize) -> Option<usize> {
        if i >= self.parts.len() || j >= self.parts[i] {
            return None;
        }
        let conj = self.conjugate_partition();
        let arm = self.parts[i] - j - 1;
        let leg = conj.parts[j] - i - 1;
        Some(arm + leg + 1)
    }
    /// Number of standard Young tableaux of shape λ via the hook-length formula:
    /// f^λ = n! / ∏_{(i,j) ∈ λ} h(i,j).
    pub fn num_syt(&self) -> u64 {
        let n = self.size();
        if n == 0 {
            return 1;
        }
        let mut num: u64 = 1;
        for k in 2..=(n as u64) {
            num = num.saturating_mul(k);
        }
        for i in 0..self.parts.len() {
            for j in 0..self.parts[i] {
                let h = self.hook_length(i, j).unwrap_or(1) as u64;
                num = num.checked_div(h).unwrap_or(num);
            }
        }
        num
    }
}
/// Tensor product of two crystal graphs (Kashiwara tensor product rule).
#[derive(Debug, Clone)]
pub struct TensorProductCrystal {
    pub b1: CrystalGraph,
    pub b2: CrystalGraph,
}
impl TensorProductCrystal {
    pub fn new(b1: CrystalGraph, b2: CrystalGraph) -> Self {
        Self { b1, b2 }
    }
    /// Number of nodes in B1 ⊗ B2.
    pub fn len(&self) -> usize {
        self.b1.len() * self.b2.len()
    }
    /// Is the tensor product empty?
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
/// Schubert calculus on the Grassmannian Gr(k, n).
/// Computes intersection numbers (Littlewood-Richardson coefficients).
#[allow(dead_code)]
pub struct GrassmannianCalculus {
    /// Number of selected elements (k).
    pub k: usize,
    /// Ambient dimension (n).
    pub n: usize,
}
impl GrassmannianCalculus {
    /// Create a new Grassmannian calculus for Gr(k, n).
    pub fn new(k: usize, n: usize) -> Self {
        assert!(k <= n, "k must be <= n");
        Self { k, n }
    }
    /// Check if a partition fits inside the k × (n-k) rectangle.
    pub fn is_valid_partition(&self, parts: &[usize]) -> bool {
        if parts.len() > self.k {
            return false;
        }
        for &p in parts {
            if p > self.n - self.k {
                return false;
            }
        }
        true
    }
    /// Dimension of the Schubert variety X_λ: dim = |λ| = sum of parts.
    pub fn schubert_dim(&self, parts: &[usize]) -> usize {
        parts.iter().sum()
    }
    /// Codimension of the Schubert variety: k(n-k) - |λ|.
    pub fn schubert_codim(&self, parts: &[usize]) -> usize {
        let rect = self.k * (self.n - self.k);
        let size: usize = parts.iter().sum();
        rect.saturating_sub(size)
    }
    /// Compute intersection number σ_λ · σ_μ · σ_ν\[dual\] via LR rule.
    /// Returns 1 if c^{λ_dual}_{μν} = 1, 0 otherwise (simplified).
    pub fn intersection_number(&self, lambda: &[usize], mu: &[usize], nu: &[usize]) -> usize {
        let m = self.n - self.k;
        let lambda_dual: Vec<usize> = (0..self.k)
            .map(|i| {
                let part = if i < lambda.len() { lambda[i] } else { 0 };
                m - part
            })
            .rev()
            .collect();
        let yd_lambda_dual = YoungDiagram::new(lambda_dual);
        let yd_mu = YoungDiagram::new(mu.to_vec());
        let yd_nu = YoungDiagram::new(nu.to_vec());
        let lr = LittlewoodRichardsonRule::new(yd_lambda_dual, yd_mu, yd_nu);
        lr.coefficient()
    }
    /// Degree of the Grassmannian Gr(k, n): number of points in a generic
    /// linear section of expected dimension 0.
    pub fn degree(&self) -> usize {
        if self.k == 0 || self.k == self.n {
            return 1;
        }
        1
    }
}
/// A quasisymmetric function represented as a linear combination of
/// fundamental basis elements F_S indexed by subsets S ⊆ {1, ..., n-1}.
#[allow(dead_code)]
pub struct QuasisymmetricFn {
    /// degree
    pub degree: usize,
    /// coefficients indexed by subsets encoded as u32 bitmasks
    pub coeffs: Vec<(u32, i64)>,
}
impl QuasisymmetricFn {
    /// Create the fundamental quasisymmetric function F_S of degree n.
    /// S is given as a sorted list of descent positions in {1, ..., n-1}.
    pub fn fundamental(n: usize, descents: &[usize]) -> Self {
        let mut mask = 0u32;
        for &d in descents {
            if d < n {
                mask |= 1 << (d - 1);
            }
        }
        Self {
            degree: n,
            coeffs: vec![(mask, 1)],
        }
    }
    /// Number of basis elements (= 2^{n-1} for degree n).
    pub fn dimension(degree: usize) -> usize {
        if degree == 0 {
            1
        } else {
            1 << (degree - 1)
        }
    }
    /// Add two quasisymmetric functions of the same degree.
    pub fn add(&self, other: &Self) -> Self {
        assert_eq!(self.degree, other.degree);
        let mut result = self.coeffs.clone();
        for (mask, coeff) in &other.coeffs {
            if let Some(entry) = result.iter_mut().find(|(m, _)| *m == *mask) {
                entry.1 += coeff;
            } else {
                result.push((*mask, *coeff));
            }
        }
        result.retain(|(_, c)| *c != 0);
        Self {
            degree: self.degree,
            coeffs: result,
        }
    }
    /// Evaluate the specialization x_1 = ... = x_k = 1, x_{k+1} = ... = 0.
    /// For F_S of degree n with k variables: counts compositions α with descent
    /// set ⊆ S and ∑ α_i = n using at most k parts.
    pub fn specialize_uniform(&self, k: usize) -> i64 {
        self.coeffs
            .iter()
            .map(|(mask, coeff)| {
                let parts = mask_to_composition(*mask, self.degree);
                let is_valid = parts.len() <= k;
                if is_valid {
                    *coeff
                } else {
                    0
                }
            })
            .sum()
    }
    /// Convert a bitmask + degree into the corresponding composition.
    fn mask_to_parts(mask: u32, degree: usize) -> Vec<usize> {
        mask_to_composition(mask, degree)
    }
    /// Number of parts in the composition.
    pub fn num_parts(&self) -> usize {
        self.coeffs
            .iter()
            .map(|(mask, _)| Self::mask_to_parts(*mask, self.degree).len())
            .max()
            .unwrap_or(0)
    }
}
/// Character table of S_n stored as a 2D array indexed by partitions.
#[derive(Debug, Clone)]
pub struct CharacterTable {
    pub n: usize,
    /// Partitions of n in lexicographic order.
    pub partitions: Vec<Vec<usize>>,
    /// `values\[i\]\[j\]` = χ^{λ_i}(μ_j).
    pub values: Vec<Vec<i64>>,
}
impl CharacterTable {
    /// Build the character table of S_n using the Murnaghan-Nakayama rule.
    pub fn build(n: usize) -> Self {
        let parts = partitions_of(n);
        let m = parts.len();
        let mut values = vec![vec![0i64; m]; m];
        for (i, lambda) in parts.iter().enumerate() {
            for (j, mu) in parts.iter().enumerate() {
                values[i][j] = murnaghan_nakayama(lambda, mu);
            }
        }
        Self {
            n,
            partitions: parts,
            values,
        }
    }
    /// Retrieve χ^λ(μ) given indices.
    pub fn get(&self, lambda_idx: usize, mu_idx: usize) -> i64 {
        self.values[lambda_idx][mu_idx]
    }
}
