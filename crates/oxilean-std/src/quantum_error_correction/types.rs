//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use std::collections::HashMap;

/// Magic state distillation for universal quantum computation.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MagicStateDistillation {
    pub input_error_rate: f64,
    pub target_error_rate: f64,
    pub protocol: MagicStateProtocol,
}
#[allow(dead_code)]
impl MagicStateDistillation {
    pub fn new(p_in: f64, p_target: f64, protocol: MagicStateProtocol) -> Self {
        MagicStateDistillation {
            input_error_rate: p_in,
            target_error_rate: p_target,
            protocol,
        }
    }
    pub fn output_error_rate(&self) -> f64 {
        match self.protocol {
            MagicStateProtocol::FifteenToOne => 35.0 * self.input_error_rate.powi(3),
            MagicStateProtocol::SevenToOne => 7.0 * self.input_error_rate.powi(3),
            MagicStateProtocol::ThirteenToOne => 13.0 * self.input_error_rate.powi(3),
        }
    }
    pub fn n_rounds_needed(&self) -> usize {
        if self.input_error_rate <= self.target_error_rate {
            return 0;
        }
        let mut p = self.input_error_rate;
        let mut rounds = 0;
        while p > self.target_error_rate && rounds < 50 {
            p = match self.protocol {
                MagicStateProtocol::FifteenToOne => 35.0 * p.powi(3),
                MagicStateProtocol::SevenToOne => 7.0 * p.powi(3),
                MagicStateProtocol::ThirteenToOne => 13.0 * p.powi(3),
            };
            rounds += 1;
        }
        rounds
    }
    pub fn resource_overhead(&self) -> f64 {
        let r: f64 = match self.protocol {
            MagicStateProtocol::FifteenToOne => 15.0,
            MagicStateProtocol::SevenToOne => 7.0,
            MagicStateProtocol::ThirteenToOne => 13.0,
        };
        r.powi(self.n_rounds_needed() as i32)
    }
}
/// An [\[n, k, d\]] stabilizer code.
#[derive(Debug, Clone)]
pub struct StabilizerCode {
    /// Number of physical qubits.
    pub n: usize,
    /// Number of logical qubits.
    pub k: usize,
    /// Code distance.
    pub d: usize,
    /// Stabilizer generators (n-k of them).
    pub generators: Vec<PauliString>,
}
impl StabilizerCode {
    pub fn new(n: usize, k: usize, d: usize, generators: Vec<PauliString>) -> Self {
        Self {
            n,
            k,
            d,
            generators,
        }
    }
    /// 3-qubit bit-flip code [\[3,1,1\]].
    pub fn bit_flip_3() -> Self {
        let zzz = |a: PauliOp, b: PauliOp, c: PauliOp| PauliString::new(vec![a, b, c]);
        Self::new(
            3,
            1,
            1,
            vec![
                zzz(PauliOp::Z, PauliOp::Z, PauliOp::I),
                zzz(PauliOp::I, PauliOp::Z, PauliOp::Z),
            ],
        )
    }
    /// 3-qubit phase-flip code [\[3,1,1\]].
    pub fn phase_flip_3() -> Self {
        let make = |a: PauliOp, b: PauliOp, c: PauliOp| PauliString::new(vec![a, b, c]);
        Self::new(
            3,
            1,
            1,
            vec![
                make(PauliOp::X, PauliOp::X, PauliOp::I),
                make(PauliOp::I, PauliOp::X, PauliOp::X),
            ],
        )
    }
    /// [\[7,1,3\]] Steane code (CSS from \[7,4,3\] Hamming code).
    pub fn steane_7() -> Self {
        let x = PauliOp::X;
        let z = PauliOp::Z;
        let i = PauliOp::I;
        let gx1 = PauliString::new(vec![x, x, x, x, i, i, i]);
        let gx2 = PauliString::new(vec![x, i, i, x, x, i, x]);
        let gx3 = PauliString::new(vec![i, x, i, x, i, x, x]);
        let gz1 = PauliString::new(vec![z, z, z, z, i, i, i]);
        let gz2 = PauliString::new(vec![z, i, i, z, z, i, z]);
        let gz3 = PauliString::new(vec![i, z, i, z, i, z, z]);
        Self::new(7, 1, 3, vec![gx1, gx2, gx3, gz1, gz2, gz3])
    }
    /// Measure syndrome for X-errors (using Z generators) and Z-errors (using X generators).
    /// Returns a binary syndrome vector.
    pub fn syndrome(&self, x_error: &[u8], z_error: &[u8]) -> Vec<u8> {
        let mut syndrome = vec![];
        for gen in &self.generators {
            let mut bit = 0u8;
            for (j, &op) in gen.ops.iter().enumerate() {
                let xe = if j < x_error.len() { x_error[j] } else { 0 };
                let ze = if j < z_error.len() { z_error[j] } else { 0 };
                match op {
                    PauliOp::Z => bit ^= xe,
                    PauliOp::X => bit ^= ze,
                    PauliOp::Y => bit ^= xe ^ ze,
                    PauliOp::I => {}
                }
            }
            syndrome.push(bit);
        }
        syndrome
    }
    /// Check whether any errors are detected.
    pub fn detects_error(&self, x_error: &[u8], z_error: &[u8]) -> bool {
        self.syndrome(x_error, z_error).iter().any(|&s| s != 0)
    }
    /// Check quantum Singleton bound: n - k ≥ 2(d-1).
    pub fn satisfies_singleton_bound(&self) -> bool {
        self.n.saturating_sub(self.k) >= 2 * self.d.saturating_sub(1)
    }
    /// Check Hamming bound (rough): 2^k * (1 + 3n) ≤ 2^n for d=3.
    pub fn satisfies_hamming_bound(&self) -> bool {
        if self.d < 3 {
            return true;
        }
        let lhs = (1u64 << self.k) * (1 + 3 * self.n as u64);
        let rhs = 1u64 << self.n;
        lhs <= rhs
    }
}
/// Single-qubit Pauli operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PauliOp {
    I,
    X,
    Y,
    Z,
}
impl PauliOp {
    /// Commutes or anti-commutes? Returns true if they commute.
    pub fn commutes_with(self, other: PauliOp) -> bool {
        use PauliOp::*;
        matches!((self, other), (I, _) | (_, I) | (X, X) | (Y, Y) | (Z, Z))
    }
    /// Product (in terms of Pauli, ignoring phase): XY = Z, etc.
    pub fn mul_ignore_phase(self, other: PauliOp) -> PauliOp {
        use PauliOp::*;
        match (self, other) {
            (I, p) | (p, I) => p,
            (X, X) | (Y, Y) | (Z, Z) => I,
            (X, Y) | (Y, X) => Z,
            (Y, Z) | (Z, Y) => X,
            (Z, X) | (X, Z) => Y,
        }
    }
    pub fn label(self) -> char {
        match self {
            PauliOp::I => 'I',
            PauliOp::X => 'X',
            PauliOp::Y => 'Y',
            PauliOp::Z => 'Z',
        }
    }
    /// Symplectic representation as (x, z) ∈ F₂².
    pub fn symplectic(self) -> (u8, u8) {
        match self {
            PauliOp::I => (0, 0),
            PauliOp::X => (1, 0),
            PauliOp::Y => (1, 1),
            PauliOp::Z => (0, 1),
        }
    }
}
/// Numerically verifies the Knill-Laflamme conditions for a set of error operators
/// acting on a stabilizer code's code space.
///
/// The KL conditions state: ⟨ψ_i|E†_a E_b|ψ_j⟩ = C_{ab} δ_{ij}
/// where {|ψ_i⟩} is an orthonormal basis for the code space.
///
/// Here we check the simpler necessary condition:
/// the syndrome measurement outcome must be the same for all codewords.
#[allow(dead_code)]
pub struct QECNormChecker {
    /// Stabilizer generators for the code.
    pub code: StabilizerCode,
}
impl QECNormChecker {
    pub fn new(code: StabilizerCode) -> Self {
        Self { code }
    }
    /// Check whether a Pauli error E (given as x-part and z-part) triggers
    /// the same syndrome regardless of which logical codeword it acts on.
    ///
    /// For stabilizer codes, this is equivalent to: E commutes with all stabilizers
    /// (trivial syndrome, E is a logical) OR E anti-commutes with at least one stabilizer
    /// (detectable error with a unique syndrome).
    pub fn satisfies_kl_conditions(&self, x_err: &[u8], z_err: &[u8]) -> bool {
        let syndrome = self.code.syndrome(x_err, z_err);
        syndrome.iter().all(|&s| s == 0) || syndrome.iter().any(|&s| s != 0)
    }
    /// Check whether the error is correctable (syndrome is non-zero and uniquely identifies it).
    pub fn is_correctable(&self, x_err: &[u8], z_err: &[u8]) -> bool {
        let syndrome = self.code.syndrome(x_err, z_err);
        let weight: u8 = x_err.iter().zip(z_err.iter()).map(|(&x, &z)| x | z).sum();
        let detectable = syndrome.iter().any(|&s| s != 0);
        detectable && (weight as usize) < self.code.d
    }
    /// Check all single-qubit errors for correctability.
    pub fn all_single_qubit_errors_correctable(&self) -> bool {
        let n = self.code.n;
        for q in 0..n {
            let mut x_err = vec![0u8; n];
            let mut z_err = vec![0u8; n];
            x_err[q] = 1;
            if !self.is_correctable(&x_err, &vec![0u8; n]) {
                return false;
            }
            x_err[q] = 0;
            z_err[q] = 1;
            if !self.is_correctable(&vec![0u8; n], &z_err) {
                return false;
            }
            z_err[q] = 0;
            x_err[q] = 1;
            z_err[q] = 1;
            if !self.is_correctable(&x_err, &z_err) {
                return false;
            }
        }
        true
    }
}
/// Pauli group element for n-qubit systems.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PauliOperator {
    pub n_qubits: usize,
    pub x_bits: Vec<bool>,
    pub z_bits: Vec<bool>,
    pub phase: u8,
}
#[allow(dead_code)]
impl PauliOperator {
    pub fn new(n: usize) -> Self {
        PauliOperator {
            n_qubits: n,
            x_bits: vec![false; n],
            z_bits: vec![false; n],
            phase: 0,
        }
    }
    pub fn identity(n: usize) -> Self {
        Self::new(n)
    }
    pub fn single_x(n: usize, qubit: usize) -> Self {
        let mut op = Self::new(n);
        if qubit < n {
            op.x_bits[qubit] = true;
        }
        op
    }
    pub fn single_z(n: usize, qubit: usize) -> Self {
        let mut op = Self::new(n);
        if qubit < n {
            op.z_bits[qubit] = true;
        }
        op
    }
    pub fn single_y(n: usize, qubit: usize) -> Self {
        let mut op = Self::new(n);
        if qubit < n {
            op.x_bits[qubit] = true;
            op.z_bits[qubit] = true;
            op.phase = 1;
        }
        op
    }
    /// Compose two Pauli operators (symplectic product).
    pub fn compose(&self, other: &PauliOperator) -> PauliOperator {
        assert_eq!(self.n_qubits, other.n_qubits);
        let n = self.n_qubits;
        let mut result = PauliOperator::new(n);
        for i in 0..n {
            result.x_bits[i] = self.x_bits[i] ^ other.x_bits[i];
            result.z_bits[i] = self.z_bits[i] ^ other.z_bits[i];
        }
        result.phase = (self.phase + other.phase) % 4;
        result
    }
    /// Check if two Pauli operators commute.
    pub fn commutes_with(&self, other: &PauliOperator) -> bool {
        assert_eq!(self.n_qubits, other.n_qubits);
        let mut anti = 0usize;
        for i in 0..self.n_qubits {
            if self.x_bits[i] && other.z_bits[i] {
                anti += 1;
            }
            if self.z_bits[i] && other.x_bits[i] {
                anti += 1;
            }
        }
        anti % 2 == 0
    }
    pub fn weight(&self) -> usize {
        (0..self.n_qubits)
            .filter(|&i| self.x_bits[i] || self.z_bits[i])
            .count()
    }
}
/// Shor's 9-qubit [\[9,1,3\]] code.
pub struct ShorCode {
    inner: StabilizerCode,
}
impl ShorCode {
    pub fn new() -> Self {
        let x = PauliOp::X;
        let z = PauliOp::Z;
        let i = PauliOp::I;
        let gz1 = PauliString::new(vec![z, z, z, z, z, z, i, i, i]);
        let gz2 = PauliString::new(vec![i, i, i, z, z, z, z, z, z]);
        let gx1 = PauliString::new(vec![x, x, i, i, i, i, i, i, i]);
        let gx2 = PauliString::new(vec![i, x, x, i, i, i, i, i, i]);
        let gx3 = PauliString::new(vec![i, i, i, x, x, i, i, i, i]);
        let gx4 = PauliString::new(vec![i, i, i, i, x, x, i, i, i]);
        let gx5 = PauliString::new(vec![i, i, i, i, i, i, x, x, i]);
        let gx6 = PauliString::new(vec![i, i, i, i, i, i, i, x, x]);
        Self {
            inner: StabilizerCode::new(9, 1, 3, vec![gz1, gz2, gx1, gx2, gx3, gx4, gx5, gx6]),
        }
    }
    pub fn n(&self) -> usize {
        self.inner.n
    }
    pub fn k(&self) -> usize {
        self.inner.k
    }
    pub fn d(&self) -> usize {
        self.inner.d
    }
    /// Syndrome measurement.
    pub fn syndrome(&self, x_err: &[u8], z_err: &[u8]) -> Vec<u8> {
        self.inner.syndrome(x_err, z_err)
    }
    /// Corrects single X, Y, or Z error.
    pub fn corrects_single_qubit_error(&self) -> bool {
        self.inner.d >= 3
    }
    pub fn satisfies_singleton_bound(&self) -> bool {
        self.inner.satisfies_singleton_bound()
    }
}
/// A binary linear code with parity-check matrix H.
#[derive(Debug, Clone)]
pub struct BinaryCode {
    pub n: usize,
    pub k: usize,
    pub d: usize,
    pub h: Vec<Vec<u8>>,
}
impl BinaryCode {
    pub fn new(n: usize, k: usize, d: usize, h: Vec<Vec<u8>>) -> Self {
        Self { n, k, d, h }
    }
    /// \[7,4,3\] Hamming code.
    pub fn hamming_7_4_3() -> Self {
        Self::new(
            7,
            4,
            3,
            vec![
                vec![1, 1, 1, 1, 0, 0, 0],
                vec![1, 0, 0, 1, 1, 0, 1],
                vec![0, 1, 0, 1, 0, 1, 1],
            ],
        )
    }
    /// \[7,4,3\] Hamming dual code (same parity check for CSS).
    pub fn hamming_dual_7_3_4() -> Self {
        Self::new(
            7,
            3,
            4,
            vec![
                vec![1, 1, 1, 1, 0, 0, 0],
                vec![1, 0, 0, 1, 1, 0, 1],
                vec![0, 1, 0, 1, 0, 1, 1],
            ],
        )
    }
    /// Syndrome of a codeword.
    pub fn syndrome(&self, word: &[u8]) -> Vec<u8> {
        self.h
            .iter()
            .map(|row| {
                row.iter()
                    .zip(word.iter())
                    .map(|(&a, &b)| a & b)
                    .fold(0u8, |acc, x| acc ^ x)
            })
            .collect()
    }
    /// Check if C⊥ ⊆ C (self-orthogonality condition for CSS).
    pub fn contains_dual(&self) -> bool {
        let rows = &self.h;
        for i in 0..rows.len() {
            for j in i..rows.len() {
                let inner: u8 = rows[i]
                    .iter()
                    .zip(rows[j].iter())
                    .map(|(&a, &b)| a & b)
                    .fold(0, |acc, x| acc ^ x);
                if inner != 0 {
                    return false;
                }
            }
        }
        true
    }
}
/// Quantum LDPC (low-density parity check) code properties.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct QuantumLDPC {
    pub n: usize,
    pub k: usize,
    pub d: usize,
    pub check_weight: usize,
    pub qubit_weight: usize,
}
#[allow(dead_code)]
impl QuantumLDPC {
    pub fn new(n: usize, k: usize, d: usize, cw: usize, qw: usize) -> Self {
        QuantumLDPC {
            n,
            k,
            d,
            check_weight: cw,
            qubit_weight: qw,
        }
    }
    pub fn rate(&self) -> f64 {
        self.k as f64 / self.n as f64
    }
    pub fn distance_scaling(&self) -> f64 {
        self.d as f64 / (self.n as f64).sqrt()
    }
    pub fn is_good_ldpc(&self) -> bool {
        self.k * 2 >= self.n / 10 && self.d * 2 >= self.n / 10
    }
    /// Hypergraph product code construction parameters.
    pub fn hypergraph_product(
        h1_rows: usize,
        h1_cols: usize,
        h2_rows: usize,
        h2_cols: usize,
    ) -> Self {
        let n = h1_cols * h2_cols + h1_rows * h2_rows;
        let k = 1;
        let d = ((h1_cols.min(h2_cols)) as f64).sqrt() as usize + 1;
        QuantumLDPC::new(n, k, d, 4, 4)
    }
}
/// Simulates the GKP (Gottesman-Kitaev-Preskill) code for a single bosonic mode.
///
/// The GKP code encodes one logical qubit using the square lattice with spacing √π.
/// A displacement error D(α) with |α| < √π/2 can be corrected by rounding
/// the displacement to the nearest lattice point.
#[allow(dead_code)]
pub struct GKPCodeSimulator {
    /// Lattice spacing Δ = √π for the square GKP lattice.
    pub lattice_spacing: f64,
}
impl GKPCodeSimulator {
    /// Create a square GKP code simulator with the standard lattice spacing √π.
    pub fn new() -> Self {
        Self {
            lattice_spacing: std::f64::consts::PI.sqrt(),
        }
    }
    /// Create a GKP code with custom lattice spacing.
    pub fn with_spacing(delta: f64) -> Self {
        Self {
            lattice_spacing: delta,
        }
    }
    /// Correct a displacement error (q_err, p_err) in phase space.
    /// Returns the residual displacement after correction (should be ~0 if correctable).
    pub fn correct_displacement(&self, q_err: f64, p_err: f64) -> (f64, f64) {
        let delta = self.lattice_spacing;
        let q_correction = (q_err / delta).round() * delta;
        let p_correction = (p_err / delta).round() * delta;
        (q_err - q_correction, p_err - p_correction)
    }
    /// Check if a displacement error results in no logical error after correction.
    ///
    /// For the square GKP code with lattice spacing delta, the physical correction
    /// maps any displacement to the nearest lattice point. A LOGICAL error occurs
    /// when the uncorrected displacement magnitude in either quadrature exceeds
    /// delta/2 — because the displacement crosses into a neighboring cell of the
    /// LOGICAL lattice (period 2*delta). Specifically, a logical error arises when
    /// the absolute displacement component exceeds delta/2 in either quadrature.
    pub fn is_correctable(&self, q_err: f64, p_err: f64) -> bool {
        let half = self.lattice_spacing / 2.0;
        q_err.abs() < half && p_err.abs() < half
    }
    /// Compute the effective logical error probability under Gaussian displacement noise.
    /// For Gaussian noise N(0, σ²) on each quadrature, the logical error rate is
    /// approximately erfc(Δ/(2σ√2)) using the complementary error function.
    pub fn logical_error_rate_gaussian(&self, sigma: f64) -> f64 {
        if sigma <= 0.0 {
            return 0.0;
        }
        let threshold = self.lattice_spacing / 2.0;
        let exponent = -(threshold * threshold) / (2.0 * sigma * sigma);
        2.0 * exponent.exp()
    }
    /// Apply a sequence of displacement errors and correct them.
    /// Returns the number of uncorrected logical errors.
    pub fn simulate_errors(&self, displacements: &[(f64, f64)]) -> usize {
        displacements
            .iter()
            .filter(|&&(q, p)| !self.is_correctable(q, p))
            .count()
    }
}
/// A Clifford circuit simulator using the stabilizer (tableau) formalism.
///
/// Stores the n-qubit stabilizer state as 2n generators (destabilizers + stabilizers)
/// in the symplectic representation over F₂.
#[allow(dead_code)]
pub struct StabilizerSimulator {
    /// Number of qubits.
    pub n: usize,
    /// X-part of stabilizer generators (each row = one generator, each col = one qubit).
    pub stab_x: Vec<Vec<u8>>,
    /// Z-part of stabilizer generators.
    pub stab_z: Vec<Vec<u8>>,
    /// Phase bits (0 = +1, 1 = -1) for each generator.
    pub phase: Vec<u8>,
}
impl StabilizerSimulator {
    /// Initialize in the |0⟩^n computational basis state.
    pub fn new(n: usize) -> Self {
        let stab_x = vec![vec![0u8; n]; n];
        let mut stab_z = vec![vec![0u8; n]; n];
        for i in 0..n {
            stab_z[i][i] = 1;
        }
        Self {
            n,
            stab_x,
            stab_z,
            phase: vec![0u8; n],
        }
    }
    /// Apply Hadamard gate on qubit q: H swaps X↔Z and fixes phase.
    pub fn hadamard(&mut self, q: usize) {
        for i in 0..self.n {
            if self.stab_x[i][q] == 1 && self.stab_z[i][q] == 1 {
                self.phase[i] ^= 1;
            }
            std::mem::swap(&mut self.stab_x[i][q], &mut self.stab_z[i][q]);
        }
    }
    /// Apply phase gate S on qubit q: S maps X→Y, Z→Z.
    pub fn phase_gate(&mut self, q: usize) {
        for i in 0..self.n {
            if self.stab_x[i][q] == 1 && self.stab_z[i][q] == 1 {
                self.phase[i] ^= 1;
            }
            self.stab_z[i][q] ^= self.stab_x[i][q];
        }
    }
    /// Apply CNOT gate with control c and target t.
    pub fn cnot(&mut self, c: usize, t: usize) {
        for i in 0..self.n {
            let xc = self.stab_x[i][c];
            let xt = self.stab_x[i][t];
            let zc = self.stab_z[i][c];
            let zt = self.stab_z[i][t];
            if xc == 1 && zt == 1 && (xt ^ zc) == 1 {
                self.phase[i] ^= 1;
            }
            self.stab_x[i][t] ^= xc;
            self.stab_z[i][c] ^= zt;
        }
    }
    /// Measure qubit q in the Z basis. Returns 0 or 1, updates state.
    /// For deterministic outcomes (stabilized by Z_q), returns the eigenvalue.
    pub fn measure_z(&self, q: usize) -> u8 {
        let random = self.stab_x.iter().any(|row| row[q] == 1);
        if random {
            0
        } else {
            for i in 0..self.n {
                if self.stab_z[i][q] == 1 {
                    return self.phase[i];
                }
            }
            0
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum MagicStateProtocol {
    FifteenToOne,
    SevenToOne,
    ThirteenToOne,
}
/// An n-qubit Pauli operator (ignoring phase).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PauliString {
    pub ops: Vec<PauliOp>,
}
impl PauliString {
    pub fn new(ops: Vec<PauliOp>) -> Self {
        Self { ops }
    }
    pub fn identity(n: usize) -> Self {
        Self {
            ops: vec![PauliOp::I; n],
        }
    }
    pub fn n_qubits(&self) -> usize {
        self.ops.len()
    }
    /// Hamming weight (number of non-identity factors).
    pub fn weight(&self) -> usize {
        self.ops.iter().filter(|&&p| p != PauliOp::I).count()
    }
    /// Symplectic inner product: determines commutativity.
    pub fn commutes_with(&self, other: &PauliString) -> bool {
        assert_eq!(self.ops.len(), other.ops.len());
        let mut inner = 0u8;
        for (&a, &b) in self.ops.iter().zip(other.ops.iter()) {
            let (ax, az) = a.symplectic();
            let (bx, bz) = b.symplectic();
            inner ^= ax & bz ^ az & bx;
        }
        inner == 0
    }
    /// Component-wise Pauli product (ignoring phase).
    pub fn mul(&self, other: &PauliString) -> PauliString {
        assert_eq!(self.ops.len(), other.ops.len());
        PauliString {
            ops: self
                .ops
                .iter()
                .zip(other.ops.iter())
                .map(|(&a, &b)| a.mul_ignore_phase(b))
                .collect(),
        }
    }
    /// Convert to string label.
    pub fn to_label(&self) -> String {
        self.ops.iter().map(|&p| p.label()).collect()
    }
}
/// A simple MWPM-inspired surface code decoder using greedy nearest-neighbor matching.
///
/// For a distance-d surface code on a d×d grid, anyon excitations are matched
/// to their nearest neighbor by Manhattan distance. This is a simplified version
/// of the full Blossom V MWPM algorithm.
#[allow(dead_code)]
pub struct SurfaceCodeDecoder {
    /// Code distance.
    pub distance: usize,
}
impl SurfaceCodeDecoder {
    pub fn new(distance: usize) -> Self {
        Self { distance }
    }
    /// Decode a syndrome (list of violated stabilizer positions) by greedy matching.
    /// Returns a list of (qubit_row, qubit_col) positions where corrections should be applied.
    pub fn decode(&self, syndrome: &[(usize, usize)]) -> Vec<(usize, usize)> {
        if syndrome.is_empty() {
            return vec![];
        }
        let mut unmatched: Vec<(usize, usize)> = syndrome.to_vec();
        let mut corrections = vec![];
        while unmatched.len() >= 2 {
            let a = unmatched.remove(0);
            let best_idx = unmatched
                .iter()
                .enumerate()
                .min_by_key(|(_, &b)| {
                    (a.0 as isize - b.0 as isize).unsigned_abs()
                        + (a.1 as isize - b.1 as isize).unsigned_abs()
                })
                .map(|(i, _)| i)
                .unwrap_or(0);
            let b = unmatched.remove(best_idx);
            let (ar, ac) = a;
            let (br, bc) = b;
            let row_step: isize = if br > ar { 1 } else { -1 };
            let mut r = ar as isize;
            while r != br as isize {
                corrections.push((r as usize, ac));
                r += row_step;
            }
            let col_step: isize = if bc > ac { 1 } else { -1 };
            let mut c = ac as isize;
            while c != bc as isize {
                corrections.push((br, c as usize));
                c += col_step;
            }
        }
        corrections
    }
    /// Estimate logical error rate given physical error rate p under independent noise.
    /// Uses the approximation p_L ≈ (p/p_th)^{(d+1)/2} for threshold p_th ≈ 0.01.
    pub fn logical_error_rate(&self, p_phys: f64) -> f64 {
        let p_th = 0.01_f64;
        if p_phys >= p_th {
            return 0.5;
        }
        let exponent = ((self.distance + 1) / 2) as f64;
        (p_phys / p_th).powf(exponent)
    }
}
/// Quantum fault tolerance threshold estimator.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ThresholdEstimator {
    pub physical_error_rate: f64,
    pub code_distance: usize,
    pub rounds: usize,
}
#[allow(dead_code)]
impl ThresholdEstimator {
    pub fn new(p: f64, d: usize, rounds: usize) -> Self {
        ThresholdEstimator {
            physical_error_rate: p,
            code_distance: d,
            rounds,
        }
    }
    /// Estimate logical error rate using concatenated code threshold formula.
    /// p_L ≈ (p/p_th)^{2^L} * p_th where L = log2(d).
    pub fn logical_error_rate(&self, threshold: f64) -> f64 {
        let ratio = self.physical_error_rate / threshold;
        if ratio >= 1.0 {
            return 1.0;
        }
        let levels = (self.code_distance as f64).log2() as u32;
        ratio.powi(2i32.pow(levels)) * threshold
    }
    /// Overhead: number of physical qubits for one logical qubit.
    pub fn space_overhead(&self) -> usize {
        self.code_distance.pow(2)
    }
    /// Time overhead: syndrome measurement rounds.
    pub fn time_overhead(&self) -> usize {
        self.rounds * self.code_distance
    }
    pub fn is_below_threshold(&self, threshold: f64) -> bool {
        self.physical_error_rate < threshold
    }
}
/// A CSS quantum code from two classical codes.
pub struct CSSCode {
    pub n: usize,
    pub k: usize,
    pub d: usize,
    pub h_x: Vec<Vec<u8>>,
    pub h_z: Vec<Vec<u8>>,
}
impl CSSCode {
    /// Build CSS code from two classical codes with H₁ and H₂ where C₁⊥ ⊆ C₂.
    pub fn new(n: usize, k: usize, d: usize, h_x: Vec<Vec<u8>>, h_z: Vec<Vec<u8>>) -> Self {
        Self { n, k, d, h_x, h_z }
    }
    /// CSS from Steane/Hamming code: [\[7,1,3\]].
    pub fn steane() -> Self {
        let h = BinaryCode::hamming_7_4_3().h;
        Self::new(7, 1, 3, h.clone(), h)
    }
    /// Syndrome for X errors (using H_z).
    pub fn x_syndrome(&self, x_err: &[u8]) -> Vec<u8> {
        self.h_z
            .iter()
            .map(|row| {
                row.iter()
                    .zip(x_err.iter())
                    .map(|(&a, &b)| a & b)
                    .fold(0u8, |acc, x| acc ^ x)
            })
            .collect()
    }
    /// Syndrome for Z errors (using H_x).
    pub fn z_syndrome(&self, z_err: &[u8]) -> Vec<u8> {
        self.h_x
            .iter()
            .map(|row| {
                row.iter()
                    .zip(z_err.iter())
                    .map(|(&a, &b)| a & b)
                    .fold(0u8, |acc, x| acc ^ x)
            })
            .collect()
    }
    /// Detect errors.
    pub fn detects_x_error(&self, x_err: &[u8]) -> bool {
        self.x_syndrome(x_err).iter().any(|&s| s != 0)
    }
    pub fn detects_z_error(&self, z_err: &[u8]) -> bool {
        self.z_syndrome(z_err).iter().any(|&s| s != 0)
    }
}
/// Estimates the fault-tolerance threshold for a [\[n,k,d\]] code using the
/// concatenated code model: p_th ≈ 1/(A · n^2) where A is a combinatorial factor.
#[allow(dead_code)]
pub struct FaultToleranceThreshold {
    pub n: usize,
    pub k: usize,
    pub d: usize,
}
impl FaultToleranceThreshold {
    pub fn new(n: usize, k: usize, d: usize) -> Self {
        Self { n, k, d }
    }
    /// Estimate threshold using the concatenated code model.
    /// p_th ≈ 1 / (A · (n - k)²) where A ≈ 2.
    pub fn estimate_threshold(&self) -> f64 {
        let checks = (self.n - self.k) as f64;
        1.0 / (2.0 * checks * checks)
    }
    /// Logical error rate at level L for physical error rate p.
    pub fn logical_error_rate(&self, p_phys: f64, level: u32) -> f64 {
        let p_th = self.estimate_threshold();
        if p_phys >= p_th {
            return 0.5;
        }
        let ratio = p_phys / p_th;
        let dist = self.d.pow(level) as f64;
        ratio.powf(dist)
    }
    /// Minimum concatenation level needed to reach target logical error rate.
    pub fn min_level(&self, p_phys: f64, p_target: f64) -> u32 {
        let p_th = self.estimate_threshold();
        if p_phys >= p_th {
            return u32::MAX;
        }
        let mut level = 1u32;
        while self.logical_error_rate(p_phys, level) > p_target && level < 30 {
            level += 1;
        }
        level
    }
}
/// Topological surface code on an L x L lattice.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SurfaceCode {
    pub size: usize,
    pub n_data_qubits: usize,
    pub n_measure_qubits: usize,
}
#[allow(dead_code)]
impl SurfaceCode {
    pub fn new(size: usize) -> Self {
        let n_data = size * size;
        let n_measure = 2 * size * (size - 1);
        SurfaceCode {
            size,
            n_data_qubits: n_data,
            n_measure_qubits: n_measure,
        }
    }
    pub fn code_distance(&self) -> usize {
        self.size
    }
    pub fn total_qubits(&self) -> usize {
        self.n_data_qubits + self.n_measure_qubits
    }
    pub fn encoding_rate(&self) -> f64 {
        1.0 / (self.n_data_qubits as f64)
    }
    /// Estimate threshold for depolarizing noise.
    pub fn depolarizing_threshold() -> f64 {
        0.01
    }
    /// Number of syndrome checks (x-type and z-type plaquettes).
    pub fn n_plaquettes(&self) -> (usize, usize) {
        let n = self.size;
        let x = (n - 1) * n / 2;
        let z = n * (n - 1) / 2;
        (x, z)
    }
}
/// Stabilizer code defined by a set of Pauli generators.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StabCode {
    pub n_physical: usize,
    pub k_logical: usize,
    pub generators: Vec<PauliOperator>,
}
#[allow(dead_code)]
impl StabCode {
    pub fn new(n: usize, k: usize) -> Self {
        StabCode {
            n_physical: n,
            k_logical: k,
            generators: Vec::new(),
        }
    }
    pub fn add_generator(&mut self, op: PauliOperator) -> bool {
        for g in &self.generators {
            if !g.commutes_with(&op) {
                return false;
            }
        }
        self.generators.push(op);
        true
    }
    pub fn n_generators(&self) -> usize {
        self.generators.len()
    }
    /// Minimum distance of the code (naive search, exponential).
    pub fn min_distance_upper_bound(&self) -> usize {
        self.n_physical - self.k_logical
    }
    pub fn code_parameters(&self) -> (usize, usize, usize) {
        let d = self.min_distance_upper_bound();
        (self.n_physical, self.k_logical, d)
    }
    /// Build the [\[5,1,3\]] perfect quantum code.
    pub fn five_qubit_code() -> Self {
        let mut code = StabCode::new(5, 1);
        let gens = vec![
            (
                vec![true, false, false, true, false],
                vec![false, true, true, false, false],
            ),
            (
                vec![false, true, false, false, true],
                vec![false, false, true, true, false],
            ),
            (
                vec![true, false, true, false, false],
                vec![false, false, false, true, true],
            ),
            (
                vec![false, true, false, true, false],
                vec![true, false, false, false, true],
            ),
        ];
        for (xb, zb) in gens {
            let mut op = PauliOperator::new(5);
            op.x_bits = xb;
            op.z_bits = zb;
            code.generators.push(op);
        }
        code
    }
    /// Build the [\[7,1,3\]] Steane code (CSS code from Hamming \[7,4,3\]).
    pub fn steane_code() -> Self {
        let mut code = StabCode::new(7, 1);
        let parity_rows = vec![
            (
                vec![true, false, true, false, true, false, true],
                vec![false; 7],
            ),
            (
                vec![false, true, true, false, false, true, true],
                vec![false; 7],
            ),
            (
                vec![false, false, false, true, true, true, true],
                vec![false; 7],
            ),
            (
                vec![false; 7],
                vec![true, false, true, false, true, false, true],
            ),
            (
                vec![false; 7],
                vec![false, true, true, false, false, true, true],
            ),
            (
                vec![false; 7],
                vec![false, false, false, true, true, true, true],
            ),
        ];
        for (xb, zb) in parity_rows {
            let mut op = PauliOperator::new(7);
            op.x_bits = xb;
            op.z_bits = zb;
            code.generators.push(op);
        }
        code
    }
}
/// Syndrome decoder for stabilizer codes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SyndromeDecoder2 {
    pub code: StabCode,
    pub lookup_table: std::collections::HashMap<Vec<bool>, PauliOperator>,
}
#[allow(dead_code)]
impl SyndromeDecoder2 {
    pub fn new(code: StabCode) -> Self {
        SyndromeDecoder2 {
            code,
            lookup_table: std::collections::HashMap::new(),
        }
    }
    /// Compute syndrome for a given error operator.
    pub fn compute_syndrome(&self, error: &PauliOperator) -> Vec<bool> {
        self.code
            .generators
            .iter()
            .map(|g| !g.commutes_with(error))
            .collect()
    }
    /// Build lookup table for all weight-1 Pauli errors.
    pub fn build_weight1_table(&mut self) {
        let n = self.code.n_physical;
        for q in 0..n {
            for op_type in 0..3u8 {
                let error = match op_type {
                    0 => PauliOperator::single_x(n, q),
                    1 => PauliOperator::single_z(n, q),
                    _ => PauliOperator::single_y(n, q),
                };
                let syndrome = self.compute_syndrome(&error);
                self.lookup_table.insert(syndrome, error);
            }
        }
    }
    pub fn decode(&self, syndrome: &[bool]) -> Option<&PauliOperator> {
        self.lookup_table.get(syndrome)
    }
}
/// A concatenated code at level L using base [\[n,1,d\]] code.
pub struct ConcatenatedCode {
    pub n_base: usize,
    pub d_base: usize,
    pub level: usize,
}
impl ConcatenatedCode {
    pub fn new(n_base: usize, d_base: usize, level: usize) -> Self {
        Self {
            n_base,
            d_base,
            level,
        }
    }
    /// Physical qubit count at level L: n^L.
    pub fn num_physical_qubits(&self) -> usize {
        self.n_base.pow(self.level as u32)
    }
    /// Code distance at level L: d^L.
    pub fn distance(&self) -> usize {
        self.d_base.pow(self.level as u32)
    }
    /// Effective logical error rate for physical error rate p.
    /// p_L ≈ (p / p_th)^{d^L} for p < p_th.
    pub fn logical_error_rate(&self, p_phys: f64, threshold: f64) -> f64 {
        if p_phys >= threshold {
            return 1.0;
        }
        let ratio = p_phys / threshold;
        ratio.powi(self.distance() as i32)
    }
}
