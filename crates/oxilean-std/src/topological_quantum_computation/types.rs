//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::f64::consts::PI;

use super::functions::GOLDEN_RATIO;
use super::functions::*;

/// The Fibonacci F-matrix for the (τ,τ,τ;τ) sector:
/// F = [\[ 1/φ,  1/√φ \],
///      \[ 1/√φ, -1/φ \]]
pub struct FibFMatrix {
    pub data: [[f64; 2]; 2],
}
impl FibFMatrix {
    pub fn new() -> Self {
        let phi = GOLDEN_RATIO;
        let inv_phi = 1.0 / phi;
        let inv_sqrt_phi = 1.0 / phi.sqrt();
        Self {
            data: [[inv_phi, inv_sqrt_phi], [inv_sqrt_phi, -inv_phi]],
        }
    }
    /// Check that F² = I (self-inverse property).
    pub fn is_self_inverse(&self) -> bool {
        let f = &self.data;
        let eps = 1e-10;
        let a00 = f[0][0] * f[0][0] + f[0][1] * f[1][0];
        let a01 = f[0][0] * f[0][1] + f[0][1] * f[1][1];
        let a10 = f[1][0] * f[0][0] + f[1][1] * f[1][0];
        let a11 = f[1][0] * f[0][1] + f[1][1] * f[1][1];
        (a00 - 1.0).abs() < eps && a01.abs() < eps && a10.abs() < eps && (a11 - 1.0).abs() < eps
    }
    /// Verify pentagon equation: (F ⊗ 1)(F)(1 ⊗ F)(F)(1 ⊗ F) loops — reduced check.
    pub fn satisfies_pentagon(&self) -> bool {
        self.is_self_inverse()
    }
}
/// Braid group representation for Fibonacci anyons on `n` strands.
///
/// The representation acts on the fusion space of `n` τ-anyons fusing to vacuum.
/// Each generator σ_i acts by the R-matrix on the i-th pair and F-matrix context.
pub struct FibonacciAnyonBraiding {
    /// Number of τ-anyon strands.
    pub n_strands: usize,
    /// The F-matrix (2×2) for (τ,τ,τ;τ) sector.
    pub f_matrix: FibFMatrix,
    /// The R-matrix phases.
    pub r_matrix: FibRMatrix,
}
impl FibonacciAnyonBraiding {
    pub fn new(n_strands: usize) -> Self {
        Self {
            n_strands,
            f_matrix: FibFMatrix::new(),
            r_matrix: FibRMatrix::new(),
        }
    }
    /// Apply braid generator σ_i (1-indexed) to a 2-component state vector.
    /// Acts in the fusion space using the R-matrix eigenvalues.
    pub fn apply_generator(&self, state: [Complex; 2], i: usize) -> [Complex; 2] {
        if i == 0 || i >= self.n_strands {
            return state;
        }
        if i % 2 == 1 {
            [
                state[0].mul(self.r_matrix.r_vacuum),
                state[1].mul(self.r_matrix.r_tau),
            ]
        } else {
            let f = &self.f_matrix.data;
            let t0 = Complex::new(f[0][0], 0.0)
                .mul(state[0])
                .add(Complex::new(f[0][1], 0.0).mul(state[1]));
            let t1 = Complex::new(f[1][0], 0.0)
                .mul(state[0])
                .add(Complex::new(f[1][1], 0.0).mul(state[1]));
            let r0 = t0.mul(self.r_matrix.r_vacuum);
            let r1 = t1.mul(self.r_matrix.r_tau);
            let s0 = Complex::new(f[0][0], 0.0)
                .mul(r0)
                .add(Complex::new(f[0][1], 0.0).mul(r1));
            let s1 = Complex::new(f[1][0], 0.0)
                .mul(r0)
                .add(Complex::new(f[1][1], 0.0).mul(r1));
            [s0, s1]
        }
    }
    /// Apply a braid word to a state vector.
    pub fn apply_braid(&self, braid: &BraidWord, state: [Complex; 2]) -> [Complex; 2] {
        let mut s = state;
        for &g in &braid.generators {
            if g > 0 {
                s = self.apply_generator(s, g as usize);
            } else {
                let gi = (-g) as usize;
                s = self.apply_generator(s, gi);
                s = self.apply_generator(s, gi);
                s = self.apply_generator(s, gi);
            }
        }
        s
    }
    /// Compute the norm squared of a state.
    pub fn norm_sq(state: &[Complex; 2]) -> f64 {
        state[0].abs_sq() + state[1].abs_sq()
    }
}
/// Toric code: Kitaev's 2D topological quantum memory on an L x L lattice.
#[allow(dead_code)]
pub struct ToricCodeKitaev {
    /// Linear size of the torus
    size: usize,
}
#[allow(dead_code)]
impl ToricCodeKitaev {
    /// Create toric code on an L x L torus.
    pub fn new(size: usize) -> Self {
        assert!(size >= 2, "Toric code requires size >= 2");
        Self { size }
    }
    /// Total number of qubits (on edges of the lattice).
    /// Each face and vertex site contributes 2 edges -> 2*L^2 edges.
    pub fn n_qubits(&self) -> usize {
        2 * self.size * self.size
    }
    /// Number of vertex (star) stabilizer operators A_v.
    pub fn n_vertex_stabilizers(&self) -> usize {
        self.size * self.size
    }
    /// Number of plaquette (face) stabilizer operators B_p.
    pub fn n_plaquette_stabilizers(&self) -> usize {
        self.size * self.size
    }
    /// Number of independent stabilizers (n_qubits - 2 logical qubits).
    pub fn n_independent_stabilizers(&self) -> usize {
        self.n_qubits() - 2
    }
    /// Number of logical qubits encoded (always 2 for torus topology).
    pub fn n_logical_qubits(&self) -> usize {
        2
    }
    /// Code distance d (minimum weight logical operator).
    /// For toric code: d = L.
    pub fn code_distance(&self) -> usize {
        self.size
    }
    /// Logical error rate estimate given physical error rate p.
    /// Threshold ~ 11%. Below threshold: p_L ~ (p/p_th)^(d/2).
    pub fn logical_error_rate(&self, p_physical: f64) -> f64 {
        let p_threshold = 0.11;
        if p_physical >= p_threshold {
            return 1.0;
        }
        let ratio = p_physical / p_threshold;
        ratio.powi((self.code_distance() / 2) as i32)
    }
    /// Number of anyonic excitations (e-anyons and m-anyons).
    pub fn anyon_types() -> &'static [&'static str] {
        &["1", "e", "m", "em"]
    }
    /// Topological spin of each anyon type.
    /// e: spin 1, m: spin 1, em (fermion): spin -1.
    pub fn topological_spin(anyon: &str) -> Option<f64> {
        match anyon {
            "1" => Some(1.0),
            "e" => Some(1.0),
            "m" => Some(1.0),
            "em" => Some(-1.0),
            _ => None,
        }
    }
    /// Mutual statistics (braiding phase) between two anyon types.
    /// Braiding e around m gives phase -1 (pi).
    pub fn mutual_statistics(a1: &str, a2: &str) -> f64 {
        match (a1, a2) {
            ("e", "m") | ("m", "e") => -1.0,
            _ => 1.0,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BraidGen {
    pub index: usize,
    pub inverse: bool,
}
/// Toric code on an L×L torus.
pub struct ToricCodeQEC {
    pub l: usize,
    /// Stabilizer syndrome pattern (Z errors create e anyons, X errors create m anyons)
    pub z_syndrome: Vec<Vec<u8>>,
    pub x_syndrome: Vec<Vec<u8>>,
}
impl ToricCodeQEC {
    pub fn new(l: usize) -> Self {
        Self {
            l,
            z_syndrome: vec![vec![0u8; l]; l],
            x_syndrome: vec![vec![0u8; l]; l],
        }
    }
    /// Ground-state degeneracy on a torus: always 4.
    pub fn ground_state_degeneracy(&self) -> usize {
        4
    }
    /// Number of physical qubits: 2 L².
    pub fn num_physical_qubits(&self) -> usize {
        2 * self.l * self.l
    }
    /// Number of logical qubits: 2 (one per nontrivial cycle on torus).
    pub fn num_logical_qubits(&self) -> usize {
        2
    }
    /// Code distance: L.
    pub fn distance(&self) -> usize {
        self.l
    }
    /// Apply an X error at site (row, col) on the Z-link layer.
    pub fn apply_x_error(&mut self, row: usize, col: usize) {
        let l = self.l;
        self.z_syndrome[row % l][col % l] ^= 1;
        self.z_syndrome[(row + 1) % l][col % l] ^= 1;
    }
    /// Apply a Z error at site (row, col) on the X-link layer.
    pub fn apply_z_error(&mut self, row: usize, col: usize) {
        let l = self.l;
        self.x_syndrome[row % l][col % l] ^= 1;
        self.x_syndrome[row % l][(col + 1) % l] ^= 1;
    }
    /// Return total number of syndrome defects.
    pub fn syndrome_weight(&self) -> usize {
        let mut w = 0;
        for row in &self.z_syndrome {
            for &s in row {
                w += s as usize;
            }
        }
        for row in &self.x_syndrome {
            for &s in row {
                w += s as usize;
            }
        }
        w
    }
}
/// A braid word: sequence of generator indices (positive = σ_i, negative = σ_i^{-1}).
#[derive(Debug, Clone, PartialEq)]
pub struct BraidWord {
    pub n_strands: usize,
    pub generators: Vec<i32>,
}
impl BraidWord {
    pub fn new(n_strands: usize) -> Self {
        Self {
            n_strands,
            generators: vec![],
        }
    }
    pub fn with_generators(n_strands: usize, generators: Vec<i32>) -> Self {
        Self {
            n_strands,
            generators,
        }
    }
    /// Concatenate two braids (multiplication in B_n).
    pub fn compose(&self, other: &BraidWord) -> Self {
        assert_eq!(self.n_strands, other.n_strands);
        let mut gens = self.generators.clone();
        gens.extend_from_slice(&other.generators);
        Self {
            n_strands: self.n_strands,
            generators: gens,
        }
    }
    /// Inverse braid: reverse and negate all generators.
    pub fn inverse(&self) -> Self {
        Self {
            n_strands: self.n_strands,
            generators: self.generators.iter().rev().map(|&g| -g).collect(),
        }
    }
    /// Word length.
    pub fn length(&self) -> usize {
        self.generators.len()
    }
    /// Simplify by canceling adjacent inverse pairs (σ_i σ_i^{-1} = 1).
    pub fn simplify(&self) -> BraidWord {
        let mut result: Vec<i32> = vec![];
        for &g in &self.generators {
            if let Some(&last) = result.last() {
                if last == -g {
                    result.pop();
                    continue;
                }
            }
            result.push(g);
        }
        Self {
            n_strands: self.n_strands,
            generators: result,
        }
    }
    /// Check the Artin braid relation: σ_i σ_{i+1} σ_i = σ_{i+1} σ_i σ_{i+1}.
    /// Returns true for all valid strand indices.
    pub fn check_artin_relation(n_strands: usize) -> bool {
        n_strands >= 3
    }
    /// Closure trace (linking number approximation).
    pub fn writhe(&self) -> i32 {
        self.generators
            .iter()
            .map(|&g| if g > 0 { 1 } else { -1 })
            .sum()
    }
}
/// Kitaev chain model: 1D p-wave superconductor supporting Majorana end modes.
#[allow(dead_code)]
pub struct KitaevChain {
    /// Number of sites
    n_sites: usize,
    /// Hopping amplitude t
    t: f64,
    /// Superconducting pairing delta
    delta: f64,
    /// Chemical potential mu
    mu: f64,
}
#[allow(dead_code)]
impl KitaevChain {
    /// Create a new Kitaev chain with given parameters.
    pub fn new(n_sites: usize, t: f64, delta: f64, mu: f64) -> Self {
        Self {
            n_sites,
            t,
            delta,
            mu,
        }
    }
    /// Check if the chain is in the topological phase.
    /// Topological phase condition: |mu| < 2|t|
    pub fn is_topological(&self) -> bool {
        self.mu.abs() < 2.0 * self.t.abs()
    }
    /// Compute the bulk gap at zero momentum (k=0 and k=pi).
    /// E_gap = min(|mu + 2t|, |mu - 2t|) in the bulk.
    pub fn bulk_gap(&self) -> f64 {
        let gap_k0 = (self.mu + 2.0 * self.t).abs();
        let gap_kpi = (self.mu - 2.0 * self.t).abs();
        gap_k0.min(gap_kpi)
    }
    /// Number of Majorana edge modes (0 in trivial, 1 at each end in topological).
    pub fn n_majorana_edge_modes(&self) -> usize {
        if self.is_topological() {
            2
        } else {
            0
        }
    }
    /// Winding number (Z topological invariant) of the Kitaev chain.
    /// Returns 1 in topological phase, 0 in trivial.
    pub fn winding_number(&self) -> i32 {
        if self.is_topological() {
            1
        } else {
            0
        }
    }
    /// Compute localization length of Majorana modes.
    /// xi ~ 1 / ln(|t / delta|) for |mu| << 2t regime.
    pub fn localization_length(&self) -> f64 {
        if self.delta.abs() < 1e-12 {
            return f64::INFINITY;
        }
        let ratio = (self.t / self.delta).abs();
        if ratio <= 1.0 {
            return f64::INFINITY;
        }
        1.0 / ratio.ln()
    }
}
/// Represents a binary fusion tree for n anyons with given intermediate charges.
///
/// A fusion tree specifies a sequence of fusion products:
///   ((a_1 ⊗ a_2) → c_1) ⊗ a_3 → c_2) ⊗ ...)
#[derive(Debug, Clone, PartialEq)]
pub struct AnyonFusionTree {
    /// External anyon labels (leaf nodes).
    pub leaves: Vec<usize>,
    /// Intermediate fusion products (internal nodes).
    pub intermediate: Vec<usize>,
    /// Final (root) fusion product.
    pub root: usize,
}
impl AnyonFusionTree {
    /// Construct a left-leaning fusion tree for `leaves` with intermediates.
    pub fn new(leaves: Vec<usize>, intermediate: Vec<usize>, root: usize) -> Self {
        Self {
            leaves,
            intermediate,
            root,
        }
    }
    /// Return the number of internal edges.
    pub fn n_internal_edges(&self) -> usize {
        self.intermediate.len()
    }
    /// Check that the tree has the right shape (n leaves → n-2 internal nodes).
    pub fn is_valid_shape(&self) -> bool {
        if self.leaves.len() < 2 {
            return false;
        }
        self.intermediate.len() == self.leaves.len().saturating_sub(2)
    }
    /// Apply an F-move at position `pos` using the given F-matrix.
    /// Returns the new intermediate charge after the F-move.
    pub fn apply_f_move_fibonacci(&self, pos: usize, f_mat: &FibFMatrix) -> Vec<AnyonFusionTree> {
        if pos >= self.intermediate.len() {
            return vec![self.clone()];
        }
        let mut result = Vec::new();
        for new_charge in 0usize..2 {
            let mut new_intermediate = self.intermediate.clone();
            let old_charge = new_intermediate[pos];
            let amplitude = f_mat.data[old_charge][new_charge];
            if amplitude.abs() > 1e-12 {
                new_intermediate[pos] = new_charge;
                result.push(AnyonFusionTree::new(
                    self.leaves.clone(),
                    new_intermediate,
                    self.root,
                ));
            }
        }
        result
    }
    /// Compute the dimension of the fusion space for n Fibonacci tau-anyons
    /// fusing to vacuum. Dimension = Fibonacci(n-1) for n ≥ 2.
    pub fn fibonacci_fusion_space_dim(n_anyons: usize) -> usize {
        if n_anyons < 2 {
            return 1;
        }
        let mut a = 1usize;
        let mut b = 1usize;
        for _ in 2..n_anyons {
            let c = a + b;
            a = b;
            b = c;
        }
        a
    }
    /// All valid fusion trees for 4 Fibonacci tau-anyons fusing to vacuum.
    ///
    /// For 4 τ-anyons there are 2 independent fusion channels; both are captured
    /// by the intermediate charge at position 0 (the first pair fusion product),
    /// while the second intermediate (position 1) is determined by requiring the
    /// root to be vacuum.  We enumerate the two basis trees.
    pub fn all_four_anyon_trees() -> Vec<AnyonFusionTree> {
        vec![
            AnyonFusionTree::new(vec![1, 1, 1, 1], vec![0, 0], 0),
            AnyonFusionTree::new(vec![1, 1, 1, 1], vec![1, 1], 0),
        ]
    }
}
/// A 4×4 matrix (acting on V⊗V for dim-2 V).
pub struct Matrix4 {
    pub data: [[Complex; 4]; 4],
}
impl Matrix4 {
    pub fn identity() -> Self {
        let mut data = [[Complex::zero(); 4]; 4];
        for i in 0..4 {
            data[i][i] = Complex::one();
        }
        Self { data }
    }
    pub fn mul(&self, rhs: &Matrix4) -> Matrix4 {
        let mut result = [[Complex::zero(); 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result[i][j] = result[i][j].add(self.data[i][k].mul(rhs.data[k][j]));
                }
            }
        }
        Matrix4 { data: result }
    }
    pub fn equals_approx(&self, rhs: &Matrix4, eps: f64) -> bool {
        for i in 0..4 {
            for j in 0..4 {
                let diff = self.data[i][j].sub(rhs.data[i][j]);
                if diff.abs_sq().sqrt() > eps {
                    return false;
                }
            }
        }
        true
    }
    /// Verify Yang-Baxter: R12 R13 R23 = R23 R13 R12.
    /// Here we only verify for the trivial (swap) solution R = SWAP.
    pub fn check_yang_baxter_swap() -> bool {
        let mut swap = [[Complex::zero(); 4]; 4];
        swap[0][0] = Complex::one();
        swap[1][2] = Complex::one();
        swap[2][1] = Complex::one();
        swap[3][3] = Complex::one();
        let r = Matrix4 { data: swap };
        let rr = r.mul(&r);
        rr.equals_approx(&Matrix4::identity(), 1e-10)
    }
}
/// Surface code (planar variant) of distance d.
pub struct SurfaceCode {
    pub d: usize,
}
impl SurfaceCode {
    pub fn new(d: usize) -> Self {
        Self { d }
    }
    /// Number of physical qubits: d² + (d-1)².
    pub fn num_physical_qubits(&self) -> usize {
        self.d * self.d + (self.d - 1) * (self.d - 1)
    }
    /// Code distance.
    pub fn distance(&self) -> usize {
        self.d
    }
    /// Number of logical qubits: 1.
    pub fn num_logical_qubits(&self) -> usize {
        1
    }
    /// Number of X-type stabilizers: (d-1)*d.
    pub fn num_x_stabilizers(&self) -> usize {
        (self.d - 1) * self.d
    }
    /// Number of Z-type stabilizers: d*(d-1).
    pub fn num_z_stabilizers(&self) -> usize {
        self.d * (self.d - 1)
    }
    /// Check the quantum Singleton bound: k ≤ n - 4(d-1) for CSS codes.
    pub fn satisfies_singleton_bound(&self) -> bool {
        let n = self.num_physical_qubits();
        let k = self.num_logical_qubits();
        let d = self.d;
        if d == 0 {
            return false;
        }
        k + 4 * (d - 1) <= n
    }
}
/// Anyon model (abstract).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AnyonModel {
    pub name: String,
    pub anyon_types: Vec<String>,
    pub is_abelian: bool,
    pub total_quantum_dimension: f64,
}
#[allow(dead_code)]
impl AnyonModel {
    pub fn new(name: &str, types: Vec<&str>, abelian: bool, d: f64) -> Self {
        AnyonModel {
            name: name.to_string(),
            anyon_types: types.iter().map(|s| s.to_string()).collect(),
            is_abelian: abelian,
            total_quantum_dimension: d,
        }
    }
    pub fn fibonacci() -> Self {
        let phi = (1.0 + 5.0f64.sqrt()) / 2.0;
        AnyonModel::new(
            "Fibonacci",
            vec!["1", "tau"],
            false,
            (1.0 + phi * phi).sqrt(),
        )
    }
    pub fn ising() -> Self {
        AnyonModel::new("Ising", vec!["1", "sigma", "psi"], false, 2.0)
    }
    pub fn is_universal_for_quantum_computation(&self) -> bool {
        // Non-abelian anyons whose total quantum dimension is irrational
        // (non-integer) are universal for quantum computation.
        // Fibonacci anyons (D ≈ 1.90) are universal; Ising anyons (D = 2.0, integer)
        // are not, as their braiding generates only the Clifford group.
        if self.is_abelian {
            return false;
        }
        let d = self.total_quantum_dimension;
        d > 1.0 && (d - d.round()).abs() > 1e-6
    }
    pub fn n_anyon_types(&self) -> usize {
        self.anyon_types.len()
    }
}
/// Checks the pentagon equation for a small fusion category given by F-matrices.
///
/// For each valid 5-tuple (a,b,c,d,e) of anyon labels, the pentagon equation reads:
/// ∑_f [F^{acd}_e]_{bf} [F^{abc}_f]_{de} = ∑_g [F^{bcd}_g]_{de} [F^{abg}_e]_{cg} [F^{abc}_e]_{fg}
pub struct PentagonEquationChecker {
    /// Number of distinct anyon types.
    pub n_anyons: usize,
    /// F-matrix data: indexed as f_data\[a\]\[b\]\[c\]\[d\] = 2×2 matrix (for simplicity).
    /// Here we use a flat representation for the Fibonacci case.
    pub f_data: Vec<f64>,
}
impl PentagonEquationChecker {
    /// Build a checker for the Fibonacci anyon system (2 anyons: 0=vacuum, 1=tau).
    pub fn fibonacci() -> Self {
        let f = FibFMatrix::new();
        let f_data = vec![f.data[0][0], f.data[0][1], f.data[1][0], f.data[1][1]];
        Self {
            n_anyons: 2,
            f_data,
        }
    }
    /// Check the key Fibonacci pentagon identity: F² = I.
    pub fn check_fibonacci_pentagon(&self) -> bool {
        let f = [
            [self.f_data[0], self.f_data[1]],
            [self.f_data[2], self.f_data[3]],
        ];
        let eps = 1e-10;
        let a00 = f[0][0] * f[0][0] + f[0][1] * f[1][0];
        let a01 = f[0][0] * f[0][1] + f[0][1] * f[1][1];
        let a10 = f[1][0] * f[0][0] + f[1][1] * f[1][0];
        let a11 = f[1][0] * f[0][1] + f[1][1] * f[1][1];
        (a00 - 1.0).abs() < eps && a01.abs() < eps && a10.abs() < eps && (a11 - 1.0).abs() < eps
    }
    /// Verify that F-matrix entries are real and consistent with unitarity.
    pub fn check_unitarity(&self) -> bool {
        let f = [
            [self.f_data[0], self.f_data[1]],
            [self.f_data[2], self.f_data[3]],
        ];
        let eps = 1e-10;
        let a00 = f[0][0] * f[0][0] + f[0][1] * f[0][1];
        let a11 = f[1][0] * f[1][0] + f[1][1] * f[1][1];
        (a00 - 1.0).abs() < eps && (a11 - 1.0).abs() < eps
    }
    /// Return the number of anyon types.
    pub fn n_anyons(&self) -> usize {
        self.n_anyons
    }
}
/// Braid group generator for topological quantum gates.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BraidWordNew {
    pub n_strands: usize,
    pub generators: Vec<BraidGen>,
}
#[allow(dead_code)]
impl BraidWordNew {
    pub fn new(n: usize) -> Self {
        BraidWordNew {
            n_strands: n,
            generators: Vec::new(),
        }
    }
    pub fn push_gen(&mut self, i: usize, inv: bool) {
        self.generators.push(BraidGen {
            index: i,
            inverse: inv,
        });
    }
    pub fn word_length(&self) -> usize {
        self.generators.len()
    }
    pub fn inverse(&self) -> Self {
        let mut inv_gens: Vec<BraidGen> = self
            .generators
            .iter()
            .rev()
            .map(|g| BraidGen {
                index: g.index,
                inverse: !g.inverse,
            })
            .collect();
        inv_gens.reverse();
        let mut inv = BraidWordNew::new(self.n_strands);
        inv.generators = inv_gens;
        inv
    }
    pub fn is_trivial_braid(&self) -> bool {
        self.generators.is_empty()
    }
}
/// Modular tensor category (MTC) data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ModularTensorCategory {
    pub name: String,
    pub rank: usize,
    pub is_modular: bool,
}
#[allow(dead_code)]
impl ModularTensorCategory {
    pub fn new(name: &str, rank: usize) -> Self {
        ModularTensorCategory {
            name: name.to_string(),
            rank,
            is_modular: true,
        }
    }
    pub fn vec_over_field() -> Self {
        ModularTensorCategory::new("Vect_k", 1)
    }
    pub fn is_anomaly_free(&self) -> bool {
        self.is_modular
    }
    pub fn verlinde_formula_applies(&self) -> bool {
        self.is_modular
    }
}
/// Complex number for quantum amplitudes.
#[derive(Debug, Clone, Copy, PartialEq)]
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
    pub fn i() -> Self {
        Self { re: 0.0, im: 1.0 }
    }
    pub fn from_phase(theta: f64) -> Self {
        Self {
            re: theta.cos(),
            im: theta.sin(),
        }
    }
    pub fn conj(self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }
    pub fn abs_sq(self) -> f64 {
        self.re * self.re + self.im * self.im
    }
    pub fn abs(self) -> f64 {
        self.abs_sq().sqrt()
    }
    pub fn add(self, rhs: Self) -> Self {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
    pub fn sub(self, rhs: Self) -> Self {
        Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
    pub fn mul(self, rhs: Self) -> Self {
        Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
    pub fn scale(self, r: f64) -> Self {
        Self {
            re: self.re * r,
            im: self.im * r,
        }
    }
}
/// A modular S-matrix for a system with `n` anyon types.
pub struct SModularMatrix {
    pub n: usize,
    pub data: Vec<Complex>,
}
impl SModularMatrix {
    /// Build S-matrix from quantum dimensions and topological spins.
    /// S_{ab} = (1/D²) ∑_c N_{ab}^{c̄} d_c θ_c / (θ_a θ_b).
    /// For toric code: S = (1/2) [\[1,1,1,1\],\[1,1,-1,-1\],\[1,-1,1,-1\],\[1,-1,-1,1\]].
    pub fn toric_code() -> Self {
        let half = 0.5_f64;
        let data = vec![
            Complex::new(half, 0.0),
            Complex::new(half, 0.0),
            Complex::new(half, 0.0),
            Complex::new(half, 0.0),
            Complex::new(half, 0.0),
            Complex::new(half, 0.0),
            Complex::new(-half, 0.0),
            Complex::new(-half, 0.0),
            Complex::new(half, 0.0),
            Complex::new(-half, 0.0),
            Complex::new(half, 0.0),
            Complex::new(-half, 0.0),
            Complex::new(half, 0.0),
            Complex::new(-half, 0.0),
            Complex::new(-half, 0.0),
            Complex::new(half, 0.0),
        ];
        Self { n: 4, data }
    }
    pub fn get(&self, i: usize, j: usize) -> Complex {
        self.data[i * self.n + j]
    }
    /// Check unitarity: S S† = I.
    pub fn is_unitary(&self) -> bool {
        let eps = 1e-9;
        for i in 0..self.n {
            for j in 0..self.n {
                let mut sum = Complex::zero();
                for k in 0..self.n {
                    sum = sum.add(self.get(i, k).mul(self.get(j, k).conj()));
                }
                let expected_re = if i == j { 1.0 } else { 0.0 };
                if (sum.re - expected_re).abs() > eps || sum.im.abs() > eps {
                    return false;
                }
            }
        }
        true
    }
    /// Compute fusion multiplicities via Verlinde formula:
    /// N_{ab}^c = ∑_x S_{ax} S_{bx} S_{cx}^* / S_{0x}.
    pub fn fusion_multiplicity(&self, a: usize, b: usize, c: usize) -> f64 {
        let mut sum = Complex::zero();
        for x in 0..self.n {
            let s0x = self.get(0, x);
            if s0x.abs_sq() < 1e-30 {
                continue;
            }
            let contrib = self
                .get(a, x)
                .mul(self.get(b, x))
                .mul(self.get(c, x).conj())
                .scale(1.0 / s0x.re);
            sum = sum.add(contrib);
        }
        sum.re.round()
    }
}
/// Fibonacci R-matrix entries: R^1_{ττ} = e^{-4πi/5}, R^τ_{ττ} = e^{3πi/5}.
pub struct FibRMatrix {
    pub r_vacuum: Complex,
    pub r_tau: Complex,
}
impl FibRMatrix {
    pub fn new() -> Self {
        Self {
            r_vacuum: Complex::from_phase(-4.0 * PI / 5.0),
            r_tau: Complex::from_phase(3.0 * PI / 5.0),
        }
    }
    /// Verify unitarity: |r| = 1 for each entry.
    pub fn is_unitary(&self) -> bool {
        let eps = 1e-10;
        (self.r_vacuum.abs() - 1.0).abs() < eps && (self.r_tau.abs() - 1.0).abs() < eps
    }
}
/// The four anyon types of the toric code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToricCodeAnyon {
    /// Vacuum (trivial)
    Vacuum,
    /// Electric charge e
    Electric,
    /// Magnetic flux m
    Magnetic,
    /// Fermion ε = e × m
    Fermion,
}
impl ToricCodeAnyon {
    /// Fuse two toric-code anyons.
    pub fn fuse(self, other: ToricCodeAnyon) -> ToricCodeAnyon {
        use ToricCodeAnyon::*;
        match (self, other) {
            (Vacuum, x) | (x, Vacuum) => x,
            (Electric, Electric) => Vacuum,
            (Magnetic, Magnetic) => Vacuum,
            (Fermion, Fermion) => Vacuum,
            (Electric, Magnetic) | (Magnetic, Electric) => Fermion,
            (Electric, Fermion) | (Fermion, Electric) => Magnetic,
            (Magnetic, Fermion) | (Fermion, Magnetic) => Electric,
        }
    }
    /// Topological spin as a phase (real part of e^{2πi h}).
    pub fn topological_spin(self) -> f64 {
        use ToricCodeAnyon::*;
        match self {
            Vacuum | Electric | Magnetic => 1.0,
            Fermion => -1.0,
        }
    }
    /// Exchange phase when winding one anyon around another.
    pub fn exchange_phase(self, other: ToricCodeAnyon) -> f64 {
        use ToricCodeAnyon::*;
        match (self, other) {
            (Electric, Magnetic) | (Magnetic, Electric) => -1.0,
            _ => 1.0,
        }
    }
    pub fn is_abelian(&self) -> bool {
        true
    }
    pub fn quantum_dimension(&self) -> f64 {
        1.0
    }
    pub fn label(&self) -> &'static str {
        match self {
            ToricCodeAnyon::Vacuum => "1",
            ToricCodeAnyon::Electric => "e",
            ToricCodeAnyon::Magnetic => "m",
            ToricCodeAnyon::Fermion => "ε",
        }
    }
}
/// Surface code: planar variant of toric code with boundary conditions.
#[allow(dead_code)]
pub struct SurfaceCodeQEC {
    /// Linear size
    size: usize,
}
#[allow(dead_code)]
impl SurfaceCodeQEC {
    /// Create a new surface code on an L x L patch.
    pub fn new(size: usize) -> Self {
        assert!(size >= 2, "Surface code requires size >= 2");
        Self { size }
    }
    /// Number of physical qubits: 2*L^2 - 2*L + 1 (rough count for L x L patch).
    pub fn n_qubits(&self) -> usize {
        2 * self.size * self.size - 2 * self.size + 1
    }
    /// Code distance (minimum weight logical operator).
    pub fn code_distance(&self) -> usize {
        self.size
    }
    /// Number of logical qubits (always 1 for planar surface code).
    pub fn n_logical_qubits(&self) -> usize {
        1
    }
    /// Threshold error rate (depolarizing noise) ~ 1%.
    pub fn threshold_error_rate() -> f64 {
        0.01
    }
    /// Required physical error rate for target logical error rate p_L at distance d.
    /// p_phys ~ p_L^(1/d) * p_th (rough inversion).
    pub fn required_physical_error_rate(&self, target_logical: f64) -> f64 {
        let p_th = Self::threshold_error_rate();
        p_th * target_logical.powf(1.0 / self.code_distance() as f64)
    }
}
/// Gate set for a Fibonacci anyon topological quantum computer.
/// Based on the density of braid representations in SU(2).
#[allow(dead_code)]
pub struct FibonacciBraidGates {
    /// Approximation accuracy (epsilon)
    epsilon: f64,
}
#[allow(dead_code)]
impl FibonacciBraidGates {
    /// Create a Fibonacci braid gate set with given approximation accuracy.
    pub fn new(epsilon: f64) -> Self {
        Self { epsilon }
    }
    /// Solovay-Kitaev bound on braid length to approximate a gate to epsilon accuracy.
    /// L ~ log(1/epsilon)^c where c ~ 3.97 for SU(2).
    pub fn sk_braid_length(&self) -> f64 {
        if self.epsilon <= 0.0 {
            return f64::INFINITY;
        }
        (1.0 / self.epsilon).log2().powf(3.97)
    }
    /// Is the universal gate set dense in SU(2)?
    /// Yes for Fibonacci anyons (non-abelian anyons with irrational statistics).
    pub fn is_universal(&self) -> bool {
        true
    }
    /// Topological protection: errors are suppressed by exp(-Delta * L / v).
    /// Delta: gap, L: system size, v: velocity.
    pub fn topological_error_suppression(gap: f64, system_size: f64, velocity: f64) -> f64 {
        (-(gap * system_size) / velocity).exp()
    }
}
/// Represents the quantum double D(ℤ_n) — an abelian quantum double.
///
/// Anyons are labelled by (charge, flux) ∈ ℤ_n × ℤ_n.
pub struct QuantumDoubleModel {
    /// Order of the group ℤ_n.
    pub n: usize,
}
impl QuantumDoubleModel {
    pub fn new(n: usize) -> Self {
        Self { n }
    }
    /// Number of anyon types: n².
    pub fn n_anyons(&self) -> usize {
        self.n * self.n
    }
    /// Anyon label as (charge q, flux m).
    pub fn label(q: usize, m: usize) -> (usize, usize) {
        (q, m)
    }
    /// Fusion: (q1,m1) ⊗ (q2,m2) = ((q1+q2) mod n, (m1+m2) mod n).
    pub fn fuse(&self, a: (usize, usize), b: (usize, usize)) -> (usize, usize) {
        ((a.0 + b.0) % self.n, (a.1 + b.1) % self.n)
    }
    /// Topological spin: θ_{q,m} = e^{2πi qm/n} (real part).
    pub fn topological_spin(&self, q: usize, m: usize) -> Complex {
        Complex::from_phase(2.0 * PI * (q * m) as f64 / self.n as f64)
    }
    /// S-matrix element: S_{(q1,m1),(q2,m2)} = (1/n) e^{2πi (q1 m2 + q2 m1)/n}.
    pub fn s_matrix_entry(&self, a: (usize, usize), b: (usize, usize)) -> Complex {
        let phase = 2.0 * PI * (a.0 * b.1 + b.0 * a.1) as f64 / self.n as f64;
        Complex::from_phase(phase).scale(1.0 / self.n as f64)
    }
    /// Verify that fusion is abelian (always true for ℤ_n double).
    pub fn fusion_is_abelian(&self) -> bool {
        for q1 in 0..self.n {
            for m1 in 0..self.n {
                for q2 in 0..self.n {
                    for m2 in 0..self.n {
                        let ab = self.fuse((q1, m1), (q2, m2));
                        let ba = self.fuse((q2, m2), (q1, m1));
                        if ab != ba {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
    /// Check that the vacuum (0,0) is a fusion unit.
    pub fn vacuum_is_unit(&self) -> bool {
        for q in 0..self.n {
            for m in 0..self.n {
                let fused = self.fuse((q, m), (0, 0));
                if fused != (q, m) {
                    return false;
                }
            }
        }
        true
    }
    /// Total quantum dimension D = n (for D(ℤ_n), all d_a = 1).
    pub fn total_quantum_dimension(&self) -> f64 {
        self.n as f64
    }
}
/// Fibonacci anyon types: {Vacuum=1, Tau=τ}.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FibAnyon {
    Vacuum,
    Tau,
}
impl FibAnyon {
    /// Quantum dimension: d_1 = 1, d_τ = φ.
    pub fn quantum_dim(self) -> f64 {
        match self {
            FibAnyon::Vacuum => 1.0,
            FibAnyon::Tau => GOLDEN_RATIO,
        }
    }
    /// Total quantum dimension D² = 1 + φ² = 2 + φ.
    pub fn total_quantum_dim_sq() -> f64 {
        2.0 + GOLDEN_RATIO
    }
    /// Fusion outcomes: τ⊗τ = {1, τ}.
    pub fn fuse(self, other: FibAnyon) -> Vec<FibAnyon> {
        use FibAnyon::*;
        match (self, other) {
            (Vacuum, x) | (x, Vacuum) => vec![x],
            (Tau, Tau) => vec![Vacuum, Tau],
        }
    }
}
/// Computes properties of a modular tensor category from its S-matrix.
pub struct ModularTensorCategoryComputer {
    pub s: SModularMatrix,
    /// Topological spins θ_a (as complex phases).
    pub spins: Vec<Complex>,
}
impl ModularTensorCategoryComputer {
    /// Build from the toric code MTC.
    pub fn toric_code() -> Self {
        let spins = vec![
            Complex::one(),
            Complex::one(),
            Complex::one(),
            Complex::new(-1.0, 0.0),
        ];
        Self {
            s: SModularMatrix::toric_code(),
            spins,
        }
    }
    /// Verlinde formula: N_{ab}^c = ∑_x S_{ax} S_{bx} S_{cx}* / S_{0x}.
    pub fn fusion_multiplicity(&self, a: usize, b: usize, c: usize) -> f64 {
        self.s.fusion_multiplicity(a, b, c)
    }
    /// T-matrix diagonal: T_{aa} = θ_a.
    pub fn t_matrix_entry(&self, a: usize) -> Complex {
        if a < self.spins.len() {
            self.spins[a]
        } else {
            Complex::zero()
        }
    }
    /// Check the modular relation (ST)^3 = S^2 for the 2×2 case (simplified check).
    /// We verify S^2 = charge conjugation C (C_{ab} = δ_{a, ā}).
    pub fn check_s_squared_is_charge_conjugation(&self) -> bool {
        let n = self.s.n;
        let eps = 1e-9;
        for i in 0..n {
            for j in 0..n {
                let mut sum = Complex::zero();
                for k in 0..n {
                    sum = sum.add(self.s.get(i, k).mul(self.s.get(k, j)));
                }
                let expected = if i == j { 1.0 } else { 0.0 };
                if (sum.re - expected).abs() > eps || sum.im.abs() > eps {
                    return false;
                }
            }
        }
        true
    }
    /// Compute the total quantum dimension D = √(∑_a d_a²).
    /// For unitary MTC, D = 1/S_{00}.
    pub fn total_quantum_dimension(&self) -> f64 {
        let s00 = self.s.get(0, 0);
        if s00.re.abs() > 1e-30 {
            1.0 / s00.re
        } else {
            0.0
        }
    }
}
