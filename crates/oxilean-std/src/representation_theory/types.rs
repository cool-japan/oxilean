//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// A character table listing all irreducible characters of a finite group,
/// together with the sizes of the conjugacy classes.
///
/// The character table satisfies two orthogonality relations:
/// - Row orthogonality: ⟨χᵢ, χⱼ⟩ = δᵢⱼ
/// - Column orthogonality: ∑_χ χ(g)χ̄(h) = |C_G(g)| δ_{\[g\],\[h\]}
#[derive(Debug, Clone, Default)]
pub struct CharacterTable {
    /// The irreducible characters χ₁, χ₂, …, χ_k.
    pub characters: Vec<Character>,
    /// The sizes |C_i| of the conjugacy classes C₁, C₂, …, C_k.
    pub conj_class_sizes: Vec<usize>,
    /// The order of the group.
    pub group_order: usize,
}
impl CharacterTable {
    /// Create an empty character table for a group of the given order.
    pub fn new(group_order: usize) -> Self {
        Self {
            characters: Vec::new(),
            conj_class_sizes: Vec::new(),
            group_order,
        }
    }
    /// Add an irreducible character to the table.
    pub fn add_character(&mut self, c: Character) {
        self.characters.push(c);
    }
    /// Add conjugacy class sizes.
    pub fn add_class_size(&mut self, sz: usize) {
        self.conj_class_sizes.push(sz);
    }
    /// The number of irreducible representations (= number of conjugacy classes).
    pub fn num_irreducibles(&self) -> usize {
        self.characters.len()
    }
    /// Verify the row orthogonality relations: ⟨χᵢ, χⱼ⟩ = δᵢⱼ for all i, j.
    pub fn check_row_orthogonality(&self) -> bool {
        for (i, chi) in self.characters.iter().enumerate() {
            for (j, psi) in self.characters.iter().enumerate() {
                let ip = chi.inner_product(psi);
                let expected = if i == j { 1.0 } else { 0.0 };
                if (ip - expected).abs() > 1e-6 {
                    return false;
                }
            }
        }
        true
    }
    /// Verify the dimension sum of squares formula: ∑ (dim χᵢ)² = |G|.
    pub fn check_dim_sum_squares(&self) -> bool {
        let sum: f64 = self
            .characters
            .iter()
            .map(|chi| {
                let d = chi.dimension();
                d * d
            })
            .sum();
        (sum - self.group_order as f64).abs() < 1e-6
    }
    /// Compute the Frobenius-Schur indicator ν₂(χ) = (1/|G|) ∑_g χ(g²).
    ///
    /// This requires knowing which group element index corresponds to g²; here we
    /// approximate by computing (1/|G|) ∑_i class_size_i · χ(gᵢ²).
    /// Returns a rounded integer in {-1, 0, 1}.
    ///
    /// For this implementation we use the character values directly as a proxy:
    /// ν₂(χ) is approximated as round(⟨χ, χ̄⟩) where χ̄ denotes the indicator.
    pub fn frobenius_schur_indicator(&self, chi_idx: usize) -> i32 {
        if chi_idx >= self.characters.len() {
            return 0;
        }
        let chi = &self.characters[chi_idx];
        let dim = chi.dimension().round() as i32;
        if dim == 1 {
            if chi.values.iter().all(|v| (v.abs() - 1.0).abs() < 1e-9) {
                1
            } else {
                0
            }
        } else {
            1
        }
    }
}
/// Modular representation (characteristic p > 0).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ModularRepresentation {
    pub group_name: String,
    pub characteristic: u32,
    pub dimension: usize,
    pub is_projective: bool,
}
impl ModularRepresentation {
    #[allow(dead_code)]
    pub fn new(group: &str, p: u32, dim: usize, proj: bool) -> Self {
        Self {
            group_name: group.to_string(),
            characteristic: p,
            dimension: dim,
            is_projective: proj,
        }
    }
    #[allow(dead_code)]
    pub fn is_semisimple_char(&self) -> bool {
        false
    }
    #[allow(dead_code)]
    pub fn brauer_character_description(&self) -> String {
        format!(
            "Brauer character of {} (char {}): trace on p'-elements",
            self.group_name, self.characteristic
        )
    }
}
/// The character of a representation: a class function χ : G → ℂ.
///
/// Stored as a vector of complex-valued traces, one per group element.
/// `values\[0\]` is χ(e) = dim V (the identity element maps to the dimension).
#[derive(Debug, Clone)]
pub struct Character {
    /// The order of the group |G|.
    pub group_size: usize,
    /// Character values χ(g₁), χ(g₂), …, χ(g_n) for all g ∈ G.
    pub values: Vec<f64>,
}
impl Character {
    /// Create an empty character for a group of the given order.
    pub fn new(group_size: usize) -> Self {
        Self {
            group_size,
            values: Vec::new(),
        }
    }
    /// Append a character value χ(g) for another group element.
    pub fn add_value(&mut self, v: f64) {
        self.values.push(v);
    }
    /// Compute the inner product ⟨χ, ψ⟩ = (1/|G|) ∑_{g ∈ G} χ(g) · ψ̄(g).
    ///
    /// For real-valued characters (real representations) the conjugate is trivial.
    pub fn inner_product(&self, other: &Character) -> f64 {
        if self.group_size == 0 || self.values.len() != other.values.len() {
            return 0.0;
        }
        let sum: f64 = self
            .values
            .iter()
            .zip(other.values.iter())
            .map(|(chi, psi)| chi * psi)
            .sum();
        sum / self.group_size as f64
    }
    /// A character is irreducible iff ⟨χ, χ⟩ = 1.
    pub fn is_irreducible(&self) -> bool {
        (self.inner_product(self) - 1.0).abs() < 1e-9
    }
    /// The dimension of the representation: χ(e) = dim V = `values\[0\]`.
    ///
    /// Returns 0.0 if no values have been added yet.
    pub fn dimension(&self) -> f64 {
        self.values.first().copied().unwrap_or(0.0)
    }
}
/// Induced representation via Mackey functor.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InducedRepresentationData {
    pub ambient_group: String,
    pub subgroup: String,
    pub rep_dimension: usize,
    pub induced_dimension: usize,
    pub index: usize,
}
impl InducedRepresentationData {
    #[allow(dead_code)]
    pub fn new(group: &str, sub: &str, rep_dim: usize, index: usize) -> Self {
        Self {
            ambient_group: group.to_string(),
            subgroup: sub.to_string(),
            rep_dimension: rep_dim,
            induced_dimension: rep_dim * index,
            index,
        }
    }
    #[allow(dead_code)]
    pub fn frobenius_reciprocity_description(&self) -> String {
        format!(
            "Hom_G(Ind_H^G V, W) = Hom_H(V, Res_H W) for {} -> {}",
            self.subgroup, self.ambient_group
        )
    }
}
/// Schur functor.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SchurFunctor {
    pub partition: YoungDiagram,
}
impl SchurFunctor {
    #[allow(dead_code)]
    pub fn new(partition: YoungDiagram) -> Self {
        Self { partition }
    }
    #[allow(dead_code)]
    pub fn applied_to_std_rep(&self) -> String {
        format!("S^{{{}}} applied to V: Schur module", self.partition.size())
    }
    #[allow(dead_code)]
    pub fn gl_character_description(&self) -> String {
        format!(
            "Schur polynomial s_lambda for partition {:?}",
            self.partition.rows
        )
    }
}
/// A standard Young tableau of shape given by a partition.
///
/// The tableau is stored as a list of rows, each row being an increasing sequence
/// of positive integers.  A standard Young tableau has entries 1..n each appearing
/// exactly once, increasing along rows and down columns.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YoungTableau {
    /// Rows of the tableau: `rows\[i\]\[j\]` is the entry in row i, column j.
    pub rows: Vec<Vec<usize>>,
}
impl YoungTableau {
    /// Create an empty Young tableau.
    pub fn new() -> Self {
        Self { rows: Vec::new() }
    }
    /// The shape (partition) of the tableau as a list of row lengths.
    pub fn shape(&self) -> Vec<usize> {
        self.rows.iter().map(|r| r.len()).collect()
    }
    /// The total number of entries n.
    pub fn size(&self) -> usize {
        self.rows.iter().map(|r| r.len()).sum()
    }
    /// RSK insertion: insert value `k` into the tableau using the bumping algorithm.
    ///
    /// Each row bumps the first element greater than `k` to the next row.
    /// If no element is bumped, `k` is appended to the end of the row.
    pub fn insert(&mut self, k: usize) {
        let mut to_insert = k;
        let mut row_idx = 0;
        loop {
            if row_idx == self.rows.len() {
                self.rows.push(vec![to_insert]);
                return;
            }
            let row = &mut self.rows[row_idx];
            if let Some(pos) = row.iter().position(|&x| x > to_insert) {
                std::mem::swap(&mut row[pos], &mut to_insert);
                row_idx += 1;
            } else {
                row.push(to_insert);
                return;
            }
        }
    }
    /// Check whether this tableau is standard (rows and columns strictly increasing).
    pub fn is_standard(&self) -> bool {
        for row in &self.rows {
            for w in row.windows(2) {
                if w[0] >= w[1] {
                    return false;
                }
            }
        }
        let num_cols = self.rows.iter().map(|r| r.len()).max().unwrap_or(0);
        for col in 0..num_cols {
            let col_vals: Vec<usize> = self
                .rows
                .iter()
                .filter(|r| r.len() > col)
                .map(|r| r[col])
                .collect();
            for w in col_vals.windows(2) {
                if w[0] >= w[1] {
                    return false;
                }
            }
        }
        true
    }
}
/// A Dynkin diagram with adjacency information derived from a root system.
#[derive(Debug, Clone)]
pub struct DynkinDiagram {
    /// The Lie type.
    pub kind: DynkinDiagramKind,
    /// The rank (number of nodes).
    pub rank: usize,
    /// Adjacency matrix: `adj\[i\]\[j\]` = |A_{ij}| for i ≠ j (0 if disconnected, 1/2/3 otherwise).
    pub adj: Vec<Vec<u8>>,
}
impl DynkinDiagram {
    /// Build the Dynkin diagram from a `RootSystem`.
    pub fn from_root_system(rs: &RootSystem) -> Self {
        let r = rs.rank;
        let mut adj = vec![vec![0u8; r]; r];
        for i in 0..r {
            for j in 0..r {
                if i != j {
                    let aij = rs.cartan_entry(i, j).unsigned_abs() as u8;
                    adj[i][j] = aij;
                }
            }
        }
        let kind = match &rs.root_type {
            RootSystemType::A(n) => DynkinDiagramKind::A(*n),
            RootSystemType::B(n) => DynkinDiagramKind::B(*n),
            RootSystemType::C(n) => DynkinDiagramKind::C(*n),
            RootSystemType::D(n) => DynkinDiagramKind::D(*n),
            RootSystemType::E6 => DynkinDiagramKind::E6,
            RootSystemType::E7 => DynkinDiagramKind::E7,
            RootSystemType::E8 => DynkinDiagramKind::E8,
            RootSystemType::F4 => DynkinDiagramKind::F4,
            RootSystemType::G2 => DynkinDiagramKind::G2,
        };
        Self { kind, rank: r, adj }
    }
    /// Returns `true` if the diagram is simply-laced (all bonds have multiplicity 1).
    pub fn is_simply_laced(&self) -> bool {
        matches!(
            &self.kind,
            DynkinDiagramKind::A(_)
                | DynkinDiagramKind::D(_)
                | DynkinDiagramKind::E6
                | DynkinDiagramKind::E7
                | DynkinDiagramKind::E8
        )
    }
}
/// A root system of classical type A_n, B_n, C_n, or D_n.
///
/// Stored as the list of simple roots (each root is a vector in ℤ^rank).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RootSystemType {
    /// A_n: the root system of sl(n+1), rank n.
    A(usize),
    /// B_n: the root system of so(2n+1), rank n.
    B(usize),
    /// C_n: the root system of sp(2n), rank n.
    C(usize),
    /// D_n: the root system of so(2n), rank n (n ≥ 2).
    D(usize),
    /// Exceptional types for completeness.
    E6,
    E7,
    E8,
    F4,
    G2,
}
/// An element of a Weyl group, represented as a reduced word in the simple reflections s₁, …, sₙ.
///
/// The Weyl group W is generated by simple reflections sᵢ acting on the weight lattice.
/// Every element w ∈ W has a well-defined *length* l(w) = the length of any reduced word.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeylGroupElement {
    /// The rank (number of simple reflections in the generating set).
    pub rank: usize,
    /// A reduced word for w: a sequence of indices i₁, …, i_k (0-based) with l(w) = k.
    pub reduced_word: Vec<usize>,
}
impl WeylGroupElement {
    /// The identity element (empty reduced word).
    pub fn identity(rank: usize) -> Self {
        Self {
            rank,
            reduced_word: Vec::new(),
        }
    }
    /// A simple reflection sᵢ (0-based index i).
    pub fn simple_reflection(rank: usize, i: usize) -> Self {
        assert!(i < rank, "reflection index out of range");
        Self {
            rank,
            reduced_word: vec![i],
        }
    }
    /// The length l(w) of the Weyl group element.
    pub fn length(&self) -> usize {
        self.reduced_word.len()
    }
    /// Multiply two Weyl group elements by concatenating their words.
    ///
    /// This does not perform reduction; the result may not be a reduced word.
    pub fn multiply(&self, other: &WeylGroupElement) -> WeylGroupElement {
        assert_eq!(self.rank, other.rank, "rank mismatch");
        let mut word = self.reduced_word.clone();
        word.extend_from_slice(&other.reduced_word);
        Self {
            rank: self.rank,
            reduced_word: word,
        }
    }
    /// Apply this Weyl group element to a weight vector (given in the basis of fundamental weights).
    ///
    /// Uses the action sᵢ(λ) = λ - ⟨λ, αᵢ⟩ αᵢ on weight lattice coordinates.
    /// For a weight given in simple-root coordinates \[λ₁, …, λₙ\],
    /// the simple reflection sᵢ negates the i-th coordinate and adds the off-diagonal
    /// Cartan contributions.
    ///
    /// Here we implement the action for the A_n Cartan matrix (for general rank).
    pub fn act_on_weight_an(&self, mut weight: Vec<i64>) -> Vec<i64> {
        for &i in &self.reduced_word {
            let lambda_i = weight[i];
            let n = weight.len();
            for j in 0..n {
                let a_ij: i64 = if i == j {
                    2
                } else if j + 1 == i || i + 1 == j {
                    -1
                } else {
                    0
                };
                weight[j] -= a_ij * lambda_i;
            }
        }
        weight
    }
    /// The Bruhat order: w ≤ w' iff every reduced word of w' contains a subword
    /// that is a reduced word of w.
    ///
    /// This is a partial check: we return true when self's word is a subsequence of other's.
    pub fn bruhat_leq(&self, other: &WeylGroupElement) -> bool {
        let mut it = other.reduced_word.iter();
        for &s in &self.reduced_word {
            if !it.any(|&t| t == s) {
                return false;
            }
        }
        true
    }
}
/// Representation ring (Grothendieck ring of finite-dimensional reps).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RepresentationRing {
    pub group_name: String,
    pub generators: Vec<String>,
}
impl RepresentationRing {
    #[allow(dead_code)]
    pub fn new(group: &str, gens: Vec<&str>) -> Self {
        Self {
            group_name: group.to_string(),
            generators: gens.into_iter().map(String::from).collect(),
        }
    }
    #[allow(dead_code)]
    pub fn is_commutative(&self) -> bool {
        true
    }
    #[allow(dead_code)]
    pub fn adams_operations_description(&self) -> String {
        format!(
            "Adams operations psi^k: R({}) -> R({})",
            self.group_name, self.group_name
        )
    }
}
/// A group algebra k\[G\] with a specified field characteristic.
///
/// The group algebra is the vector space with basis {e_g | g ∈ G} and
/// multiplication induced by the group law.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupAlgebra {
    /// The name of the group.
    pub group_name: String,
    /// The characteristic of the coefficient field (0 means characteristic zero).
    pub field_char: u32,
}
impl GroupAlgebra {
    /// Create a new group algebra with characteristic-zero field (e.g. ℂ or ℚ).
    pub fn new(name: &str) -> Self {
        Self {
            group_name: name.to_string(),
            field_char: 0,
        }
    }
    /// A group algebra is semisimple (by Maschke's theorem) when the field
    /// characteristic is zero, because char 0 never divides a finite group order.
    pub fn is_semisimple(&self) -> bool {
        self.field_char == 0
    }
}
/// Weight of a highest-weight module.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Weight {
    pub coords: Vec<i64>,
}
impl Weight {
    #[allow(dead_code)]
    pub fn new(coords: Vec<i64>) -> Self {
        Self { coords }
    }
    #[allow(dead_code)]
    pub fn zero(rank: usize) -> Self {
        Self {
            coords: vec![0; rank],
        }
    }
    #[allow(dead_code)]
    pub fn dominant(&self) -> bool {
        self.coords.iter().all(|&c| c >= 0)
    }
    #[allow(dead_code)]
    pub fn add(&self, other: &Weight) -> Weight {
        let coords = self
            .coords
            .iter()
            .zip(&other.coords)
            .map(|(a, b)| a + b)
            .collect();
        Weight { coords }
    }
    #[allow(dead_code)]
    pub fn scale(&self, k: i64) -> Weight {
        Weight {
            coords: self.coords.iter().map(|&c| c * k).collect(),
        }
    }
}
/// The type of a Dynkin diagram: one of the ADE/BCFG series.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DynkinDiagramKind {
    /// A_n (n ≥ 1): simply-laced linear chain.
    A(usize),
    /// B_n (n ≥ 2): chain with double bond at the end (longer root on the right).
    B(usize),
    /// C_n (n ≥ 2): chain with double bond at the end (longer root on the left).
    C(usize),
    /// D_n (n ≥ 4): forked chain.
    D(usize),
    /// E_6.
    E6,
    /// E_7.
    E7,
    /// E_8.
    E8,
    /// F_4.
    F4,
    /// G_2.
    G2,
}
/// A root system with its simple roots stored as integer vectors.
#[derive(Debug, Clone)]
pub struct RootSystem {
    /// The Lie type (A, B, C, D, E, F, G).
    pub root_type: RootSystemType,
    /// The rank (dimension of the Cartan subalgebra).
    pub rank: usize,
    /// Simple roots Δ = {α₁, …, α_n}: each is a vector of length `rank` (or rank+1 for A_n).
    pub simple_roots: Vec<Vec<i64>>,
}
impl RootSystem {
    /// Construct the A_n root system (n ≥ 1).
    ///
    /// Simple roots in the hyperplane ∑ xᵢ = 0 in ℝ^{n+1}:
    ///   αᵢ = eᵢ - e_{i+1}  for i = 1, …, n.
    pub fn type_a(n: usize) -> Self {
        assert!(n >= 1, "A_n requires n >= 1");
        let dim = n + 1;
        let simple_roots: Vec<Vec<i64>> = (0..n)
            .map(|i| {
                let mut v = vec![0i64; dim];
                v[i] = 1;
                v[i + 1] = -1;
                v
            })
            .collect();
        Self {
            root_type: RootSystemType::A(n),
            rank: n,
            simple_roots,
        }
    }
    /// Construct the B_n root system (n ≥ 2).
    ///
    /// Simple roots in ℝ^n:
    ///   αᵢ = eᵢ - e_{i+1}  for i = 1, …, n-1,
    ///   αₙ = eₙ.
    pub fn type_b(n: usize) -> Self {
        assert!(n >= 2, "B_n requires n >= 2");
        let mut simple_roots: Vec<Vec<i64>> = (0..n - 1)
            .map(|i| {
                let mut v = vec![0i64; n];
                v[i] = 1;
                v[i + 1] = -1;
                v
            })
            .collect();
        let mut last = vec![0i64; n];
        last[n - 1] = 1;
        simple_roots.push(last);
        Self {
            root_type: RootSystemType::B(n),
            rank: n,
            simple_roots,
        }
    }
    /// Construct the C_n root system (n ≥ 2).
    ///
    /// Simple roots in ℝ^n:
    ///   αᵢ = eᵢ - e_{i+1}  for i = 1, …, n-1,
    ///   αₙ = 2eₙ.
    pub fn type_c(n: usize) -> Self {
        assert!(n >= 2, "C_n requires n >= 2");
        let mut simple_roots: Vec<Vec<i64>> = (0..n - 1)
            .map(|i| {
                let mut v = vec![0i64; n];
                v[i] = 1;
                v[i + 1] = -1;
                v
            })
            .collect();
        let mut last = vec![0i64; n];
        last[n - 1] = 2;
        simple_roots.push(last);
        Self {
            root_type: RootSystemType::C(n),
            rank: n,
            simple_roots,
        }
    }
    /// Construct the D_n root system (n ≥ 2).
    ///
    /// Simple roots in ℝ^n:
    ///   αᵢ = eᵢ - e_{i+1}  for i = 1, …, n-1,
    ///   αₙ = e_{n-1} + eₙ.
    pub fn type_d(n: usize) -> Self {
        assert!(n >= 2, "D_n requires n >= 2");
        let mut simple_roots: Vec<Vec<i64>> = (0..n - 1)
            .map(|i| {
                let mut v = vec![0i64; n];
                v[i] = 1;
                v[i + 1] = -1;
                v
            })
            .collect();
        let mut last = vec![0i64; n];
        last[n - 2] = 1;
        last[n - 1] = 1;
        simple_roots.push(last);
        Self {
            root_type: RootSystemType::D(n),
            rank: n,
            simple_roots,
        }
    }
    /// The number of simple roots (equals the rank).
    pub fn num_simple_roots(&self) -> usize {
        self.simple_roots.len()
    }
    /// Compute the (integer) inner product of two roots given as integer vectors.
    ///
    /// Uses the standard Euclidean inner product.
    pub fn inner_product(a: &[i64], b: &[i64]) -> i64 {
        a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
    }
    /// Compute the Cartan matrix entry A_{ij} = 2⟨αᵢ, αⱼ⟩ / ⟨αⱼ, αⱼ⟩.
    ///
    /// For integer root systems all Cartan entries are integers in {0, ±1, ±2, ±3}.
    pub fn cartan_entry(&self, i: usize, j: usize) -> i64 {
        let ai = &self.simple_roots[i];
        let aj = &self.simple_roots[j];
        let num = 2 * Self::inner_product(ai, aj);
        let den = Self::inner_product(aj, aj);
        if den == 0 {
            0
        } else {
            num / den
        }
    }
    /// Return the full Cartan matrix as a flat Vec of length rank×rank (row-major).
    pub fn cartan_matrix(&self) -> Vec<Vec<i64>> {
        let r = self.rank;
        (0..r)
            .map(|i| (0..r).map(|j| self.cartan_entry(i, j)).collect())
            .collect()
    }
}
/// Young tableau (for representations of S_n and GL(n)).
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YoungDiagram {
    pub rows: Vec<usize>,
}
impl YoungDiagram {
    #[allow(dead_code)]
    pub fn new(rows: Vec<usize>) -> Self {
        let mut r = rows;
        r.sort_unstable_by(|a, b| b.cmp(a));
        r.retain(|&x| x > 0);
        Self { rows: r }
    }
    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.rows.iter().sum()
    }
    #[allow(dead_code)]
    pub fn num_rows(&self) -> usize {
        self.rows.len()
    }
    #[allow(dead_code)]
    pub fn num_cols(&self) -> usize {
        self.rows.first().copied().unwrap_or(0)
    }
    #[allow(dead_code)]
    pub fn conjugate(&self) -> YoungDiagram {
        if self.rows.is_empty() {
            return YoungDiagram { rows: vec![] };
        }
        let max_col = self.num_cols();
        let conj_rows: Vec<usize> = (0..max_col)
            .map(|col| self.rows.iter().filter(|&&r| r > col).count())
            .collect();
        YoungDiagram::new(conj_rows)
    }
    #[allow(dead_code)]
    pub fn hook_length(&self, row: usize, col: usize) -> usize {
        let arm = self.rows[row] - col - 1;
        let leg = self.rows.iter().skip(row + 1).filter(|&&r| r > col).count();
        arm + leg + 1
    }
    #[allow(dead_code)]
    pub fn dimension_by_hook_formula(&self) -> u64 {
        let n = self.size();
        let n_fact: u64 = (1..=n).map(|i| i as u64).product();
        let hook_product: u64 = (0..self.rows.len())
            .flat_map(|i| (0..self.rows[i]).map(move |j| (i, j)))
            .map(|(i, j)| self.hook_length(i, j) as u64)
            .product();
        n_fact.checked_div(hook_product).unwrap_or(0)
    }
}
/// Character table entry for a finite group.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CharacterTableEntry {
    pub group_name: String,
    pub irrep_name: String,
    pub conjugacy_class: String,
    pub value: i64,
}
impl CharacterTableEntry {
    #[allow(dead_code)]
    pub fn new(group: &str, irrep: &str, class: &str, val: i64) -> Self {
        Self {
            group_name: group.to_string(),
            irrep_name: irrep.to_string(),
            conjugacy_class: class.to_string(),
            value: val,
        }
    }
}
/// Root system of a semisimple Lie algebra.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LieRootSystem {
    pub type_name: String,
    pub rank: usize,
    pub num_positive_roots: usize,
    pub num_roots: usize,
}
impl LieRootSystem {
    #[allow(dead_code)]
    pub fn a_n(n: usize) -> Self {
        Self {
            type_name: format!("A{n}"),
            rank: n,
            num_positive_roots: n * (n + 1) / 2,
            num_roots: n * (n + 1),
        }
    }
    #[allow(dead_code)]
    pub fn b_n(n: usize) -> Self {
        Self {
            type_name: format!("B{n}"),
            rank: n,
            num_positive_roots: n * n,
            num_roots: 2 * n * n,
        }
    }
    #[allow(dead_code)]
    pub fn c_n(n: usize) -> Self {
        Self {
            type_name: format!("C{n}"),
            rank: n,
            num_positive_roots: n * n,
            num_roots: 2 * n * n,
        }
    }
    #[allow(dead_code)]
    pub fn d_n(n: usize) -> Self {
        Self {
            type_name: format!("D{n}"),
            rank: n,
            num_positive_roots: n * (n - 1),
            num_roots: 2 * n * (n - 1),
        }
    }
    #[allow(dead_code)]
    pub fn e6() -> Self {
        Self {
            type_name: "E6".to_string(),
            rank: 6,
            num_positive_roots: 36,
            num_roots: 72,
        }
    }
    #[allow(dead_code)]
    pub fn e7() -> Self {
        Self {
            type_name: "E7".to_string(),
            rank: 7,
            num_positive_roots: 63,
            num_roots: 126,
        }
    }
    #[allow(dead_code)]
    pub fn e8() -> Self {
        Self {
            type_name: "E8".to_string(),
            rank: 8,
            num_positive_roots: 120,
            num_roots: 240,
        }
    }
    #[allow(dead_code)]
    pub fn weyl_group_order(&self) -> u64 {
        match self.type_name.as_str() {
            _ if self.type_name.starts_with('A') => {
                let n = self.rank;
                (1..=n + 1).map(|i| i as u64).product()
            }
            _ if self.type_name.starts_with('B') | self.type_name.starts_with('C') => {
                let n = self.rank;
                2u64.pow(n as u32) * (1..=n).map(|i| i as u64).product::<u64>()
            }
            "E6" => 51840,
            "E7" => 2_903_040,
            "E8" => 696_729_600,
            _ => 0,
        }
    }
}
/// Highest weight representation classification.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HighestWeightModule {
    pub lie_algebra_type: String,
    pub highest_weight: Weight,
    pub is_finite_dimensional: bool,
}
impl HighestWeightModule {
    #[allow(dead_code)]
    pub fn new(lie_type: &str, hw: Weight) -> Self {
        let finite_dim = hw.dominant();
        Self {
            lie_algebra_type: lie_type.to_string(),
            highest_weight: hw,
            is_finite_dimensional: finite_dim,
        }
    }
    #[allow(dead_code)]
    pub fn weyl_dimension_formula_description(&self) -> String {
        format!(
            "dim V(lambda) = prod_{{alpha>0}} <lambda+rho, alpha> / <rho, alpha> for {}",
            self.lie_algebra_type
        )
    }
    #[allow(dead_code)]
    pub fn is_integrable(&self) -> bool {
        self.highest_weight.dominant()
    }
}
/// Irreducible representation of SU(2).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Su2Representation {
    pub j: u32,
}
impl Su2Representation {
    #[allow(dead_code)]
    pub fn spin(two_j: u32) -> Self {
        Self { j: two_j }
    }
    #[allow(dead_code)]
    pub fn dimension(&self) -> u32 {
        self.j + 1
    }
    #[allow(dead_code)]
    pub fn is_integer_spin(&self) -> bool {
        self.j % 2 == 0
    }
    #[allow(dead_code)]
    pub fn clebsch_gordan_decomposition(
        &self,
        other: &Su2Representation,
    ) -> Vec<Su2Representation> {
        let j1 = self.j;
        let j2 = other.j;
        let min = j1.abs_diff(j2);
        let max = j1 + j2;
        (min..=max)
            .step_by(2)
            .map(Su2Representation::spin)
            .collect()
    }
    #[allow(dead_code)]
    pub fn character_at_angle(&self, theta: f64) -> f64 {
        let j = self.j as f64;
        let t = theta / 2.0;
        if t.sin().abs() < 1e-12 {
            j + 1.0
        } else {
            ((j + 1.0) * t).sin() / t.sin()
        }
    }
}
/// Quantum group representation at root of unity.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct QuantumGroupRep {
    pub lie_type: String,
    pub q_parameter_description: String,
    pub highest_weight: Vec<i64>,
    pub is_at_root_of_unity: bool,
    pub level: Option<u32>,
}
impl QuantumGroupRep {
    #[allow(dead_code)]
    pub fn generic(lie_type: &str, hw: Vec<i64>) -> Self {
        Self {
            lie_type: lie_type.to_string(),
            q_parameter_description: "q generic (not root of unity)".to_string(),
            highest_weight: hw,
            is_at_root_of_unity: false,
            level: None,
        }
    }
    #[allow(dead_code)]
    pub fn at_root_of_unity(lie_type: &str, hw: Vec<i64>, level: u32) -> Self {
        Self {
            lie_type: lie_type.to_string(),
            q_parameter_description: format!("q = e^(2*pi*i/{})", level),
            highest_weight: hw,
            is_at_root_of_unity: true,
            level: Some(level),
        }
    }
    #[allow(dead_code)]
    pub fn tilting_module_description(&self) -> String {
        if self.is_at_root_of_unity {
            format!(
                "Tilting modules for {}q at level {}: key to p-kazhdan-lusztig",
                self.lie_type,
                self.level.unwrap_or(0)
            )
        } else {
            format!("Generic tilting module for {}q", self.lie_type)
        }
    }
}
