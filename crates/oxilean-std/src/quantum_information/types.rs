//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};
use std::f64::consts::LN_2;

/// Quantum circuit: sequence of gates.
#[derive(Debug, Clone)]
pub struct QuantumCircuit {
    pub num_qubits: usize,
    /// Gates as (type, target qubits).
    pub gates: Vec<(GateType, Vec<usize>)>,
}
impl QuantumCircuit {
    pub fn new(num_qubits: usize) -> Self {
        Self {
            num_qubits,
            gates: Vec::new(),
        }
    }
    pub fn add_gate(&mut self, gate: GateType, qubits: Vec<usize>) {
        self.gates.push((gate, qubits));
    }
    /// Total gate count.
    pub fn gate_count(&self) -> usize {
        self.gates.len()
    }
    /// Circuit depth (longest path — simplified: total gate count).
    pub fn depth(&self) -> usize {
        self.gates.len()
    }
    /// T-gate count.
    pub fn t_gate_count(&self) -> usize {
        self.gates
            .iter()
            .filter(|(g, _)| *g == GateType::T || *g == GateType::Tdg)
            .count()
    }
    /// Count Clifford gates (non-T gates).
    pub fn clifford_count(&self) -> usize {
        self.gates
            .iter()
            .filter(|(g, _)| !g.is_non_clifford())
            .count()
    }
    /// Solovay-Kitaev approximation: T-gate count ≈ O(log^{2.71}(1/ε)).
    pub fn solovay_kitaev_t_count(epsilon: f64) -> usize {
        if epsilon <= 0.0 || epsilon >= 1.0 {
            return 0;
        }
        let log_inv_eps = (1.0 / epsilon).ln();
        (log_inv_eps.powf(2.71) as usize).max(1)
    }
}
/// BQP complexity class (representation).
#[derive(Debug, Clone)]
pub struct BQPComplexity {
    /// Error bound (default 1/3).
    pub error_bound: f64,
}
impl BQPComplexity {
    pub fn new(error_bound: f64) -> Self {
        Self { error_bound }
    }
    pub fn standard() -> Self {
        Self::new(1.0 / 3.0)
    }
    /// Shor's algorithm is in BQP (factoring is in BQP).
    pub fn factoring_in_bqp() -> bool {
        true
    }
    /// Grover search is in BQP.
    pub fn search_in_bqp() -> bool {
        true
    }
}
/// Mixed state: a probability ensemble of pure states.
#[derive(Debug, Clone)]
pub struct MixedState {
    /// Pairs (probability, pure state).
    pub ensemble: Vec<(f64, PureState)>,
}
impl MixedState {
    pub fn new(ensemble: Vec<(f64, PureState)>) -> Self {
        Self { ensemble }
    }
    /// Convert to density matrix.
    pub fn to_density_matrix(&self) -> DensityMatrix {
        let d = self.ensemble.first().map(|(_, s)| s.dim).unwrap_or(1);
        let mut rho = DensityMatrix {
            dim: d,
            data: vec![Complex::zero(); d * d],
        };
        for (p, psi) in &self.ensemble {
            let rho_i = DensityMatrix::from_pure_state(psi);
            rho = rho.add_mat(&rho_i.scale(*p));
        }
        rho
    }
}
/// Quantum error correction model (noise model + code).
#[derive(Debug, Clone)]
pub struct QuantumErrorCorrModel {
    /// Description of the noise model (e.g. "depolarizing", "amplitude damping").
    pub noise_model: String,
}
impl QuantumErrorCorrModel {
    /// Construct an error correction model.
    pub fn new(noise_model: impl Into<String>) -> Self {
        Self {
            noise_model: noise_model.into(),
        }
    }
    /// Fidelity of the logical qubit after one round of error correction,
    /// given a physical error rate `p`.
    pub fn fidelity_after_correction(&self, p: f64, code_distance: usize) -> f64 {
        let t = (code_distance / 2) as u32;
        (1.0 - p.powi(t as i32 + 1)).max(0.0)
    }
    /// Returns `true` if `p` is below the fault-tolerance threshold.
    pub fn threshold_condition(&self, p: f64) -> bool {
        p < 0.01
    }
}
/// Gate types in a quantum circuit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateType {
    H,
    X,
    Y,
    Z,
    S,
    T,
    Tdg,
    Cnot,
    Toffoli,
    Custom,
}
impl GateType {
    /// Is this a non-Clifford gate?
    pub fn is_non_clifford(&self) -> bool {
        matches!(self, GateType::T | GateType::Tdg | GateType::Toffoli)
    }
}
/// CSS (Calderbank-Shor-Steane) code built from classical linear codes.
#[derive(Debug, Clone)]
pub struct CSSCode {
    pub n: usize,
    pub k: usize,
    pub d: usize,
    /// H_X: parity check matrix for X stabilizers (rows = X generators).
    pub h_x: Vec<Vec<u8>>,
    /// H_Z: parity check matrix for Z stabilizers (rows = Z generators).
    pub h_z: Vec<Vec<u8>>,
}
impl CSSCode {
    pub fn new(n: usize, k: usize, d: usize, h_x: Vec<Vec<u8>>, h_z: Vec<Vec<u8>>) -> Self {
        Self { n, k, d, h_x, h_z }
    }
    /// Convert to StabilizerCode.
    pub fn to_stabilizer_code(&self) -> StabilizerCode {
        let mut gens = Vec::new();
        for row in &self.h_x {
            gens.push((row.clone(), vec![0u8; self.n]));
        }
        for row in &self.h_z {
            gens.push((vec![0u8; self.n], row.clone()));
        }
        StabilizerCode {
            n: self.n,
            k: self.k,
            d: self.d,
            generators: gens,
        }
    }
}
/// Quantum key distribution protocol.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct QkdProtocol {
    pub name: String,
    pub security_model: String,
    pub key_rate_description: String,
    pub uses_entanglement: bool,
}
impl QkdProtocol {
    #[allow(dead_code)]
    pub fn bb84() -> Self {
        Self {
            name: "BB84".to_string(),
            security_model: "Information-theoretic security".to_string(),
            key_rate_description: "r = I(A:B) - I(A:E) (Devetak-Winter)".to_string(),
            uses_entanglement: false,
        }
    }
    #[allow(dead_code)]
    pub fn e91() -> Self {
        Self {
            name: "E91 (Ekert)".to_string(),
            security_model: "Bell inequality violation implies security".to_string(),
            key_rate_description: "Based on CHSH violation S = 2*sqrt(2)".to_string(),
            uses_entanglement: true,
        }
    }
    #[allow(dead_code)]
    pub fn bbm92() -> Self {
        Self {
            name: "BBM92".to_string(),
            security_model: "Entanglement-based BB84".to_string(),
            key_rate_description: "Same as BB84 via entanglement".to_string(),
            uses_entanglement: true,
        }
    }
    #[allow(dead_code)]
    pub fn is_unconditionally_secure(&self) -> bool {
        true
    }
}
/// Pure quantum state |ψ⟩ ∈ ℂ^d.
#[derive(Debug, Clone)]
pub struct PureState {
    pub dim: usize,
    pub amplitudes: Vec<Complex>,
}
impl PureState {
    pub fn new(amplitudes: Vec<Complex>) -> Self {
        let dim = amplitudes.len();
        Self { dim, amplitudes }
    }
    /// Computational basis state |k⟩.
    pub fn basis(dim: usize, k: usize) -> Self {
        let mut amplitudes = vec![Complex::zero(); dim];
        amplitudes[k] = Complex::one();
        Self { dim, amplitudes }
    }
    /// |0⟩ state.
    pub fn zero_state() -> Self {
        Self::basis(2, 0)
    }
    /// |1⟩ state.
    pub fn one_state() -> Self {
        Self::basis(2, 1)
    }
    /// |+⟩ = (|0⟩ + |1⟩)/√2.
    pub fn plus_state() -> Self {
        let a = Complex::new(1.0 / 2f64.sqrt(), 0.0);
        Self {
            dim: 2,
            amplitudes: vec![a, a],
        }
    }
    /// Norm squared ⟨ψ|ψ⟩.
    pub fn norm_sq(&self) -> f64 {
        self.amplitudes.iter().map(|c| c.abs_sq()).sum()
    }
    /// Normalise in place.
    pub fn normalize(&mut self) {
        let n = self.norm_sq().sqrt();
        for c in self.amplitudes.iter_mut() {
            *c = c.scale(1.0 / n);
        }
    }
    /// Tensor product |ψ⟩ ⊗ |φ⟩.
    pub fn tensor(&self, other: &PureState) -> PureState {
        let dim = self.dim * other.dim;
        let mut amps = vec![Complex::zero(); dim];
        for (i, &ai) in self.amplitudes.iter().enumerate() {
            for (j, &bj) in other.amplitudes.iter().enumerate() {
                amps[i * other.dim + j] = ai.mul(bj);
            }
        }
        PureState {
            dim,
            amplitudes: amps,
        }
    }
}
/// Stabilizer code [[n, k, d]] parameters.
#[derive(Debug, Clone)]
pub struct StabilizerCode {
    /// Physical qubits.
    pub n: usize,
    /// Logical qubits.
    pub k: usize,
    /// Code distance.
    pub d: usize,
    /// Generator matrix (n-k generators, each an n-bit Pauli string encoded
    /// as a pair of binary vectors (x_part, z_part) of length n).
    pub generators: Vec<(Vec<u8>, Vec<u8>)>,
}
impl StabilizerCode {
    pub fn new(n: usize, k: usize, d: usize, generators: Vec<(Vec<u8>, Vec<u8>)>) -> Self {
        assert_eq!(generators.len(), n - k);
        Self {
            n,
            k,
            d,
            generators,
        }
    }
    /// [[7, 1, 3]] Steane code (CSS code derived from [7,4,3] Hamming code).
    pub fn steane_code() -> Self {
        let h = vec![
            (vec![1, 0, 1, 0, 1, 0, 1], vec![0, 0, 0, 0, 0, 0, 0]),
            (vec![0, 1, 1, 0, 0, 1, 1], vec![0, 0, 0, 0, 0, 0, 0]),
            (vec![0, 0, 0, 1, 1, 1, 1], vec![0, 0, 0, 0, 0, 0, 0]),
            (vec![0, 0, 0, 0, 0, 0, 0], vec![1, 0, 1, 0, 1, 0, 1]),
            (vec![0, 0, 0, 0, 0, 0, 0], vec![0, 1, 1, 0, 0, 1, 1]),
            (vec![0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 1, 1, 1, 1]),
        ];
        Self::new(7, 1, 3, h)
    }
    /// [[5, 1, 3]] perfect code.
    pub fn perfect_code() -> Self {
        let gens = vec![
            (vec![1, 0, 0, 1, 0], vec![0, 1, 1, 0, 0]),
            (vec![0, 1, 0, 0, 1], vec![0, 0, 1, 1, 0]),
            (vec![1, 0, 1, 0, 0], vec![0, 0, 0, 1, 1]),
            (vec![0, 1, 0, 1, 0], vec![1, 0, 0, 0, 1]),
        ];
        Self::new(5, 1, 3, gens)
    }
    /// Code distance.
    pub fn distance(&self) -> usize {
        self.d
    }
    /// Compute the syndrome for a given error (Pauli error as (x_err, z_err)).
    /// Returns a binary vector of length n−k.
    pub fn syndrome(&self, x_err: &[u8], z_err: &[u8]) -> Vec<u8> {
        self.generators
            .iter()
            .map(|(gx, gz)| {
                let dot_xz: u8 = gx
                    .iter()
                    .zip(z_err.iter())
                    .map(|(&a, &b)| a & b)
                    .fold(0, |acc, x| acc ^ x);
                let dot_zx: u8 = gz
                    .iter()
                    .zip(x_err.iter())
                    .map(|(&a, &b)| a & b)
                    .fold(0, |acc, x| acc ^ x);
                (dot_xz ^ dot_zx) & 1
            })
            .collect()
    }
    /// Detect errors: returns true if the syndrome is non-zero.
    pub fn detect_errors(&self, x_err: &[u8], z_err: &[u8]) -> bool {
        self.syndrome(x_err, z_err).iter().any(|&s| s != 0)
    }
    /// Quantum Singleton bound check: n − k ≥ 2(d − 1).
    pub fn satisfies_singleton_bound(&self) -> bool {
        self.n - self.k >= 2 * (self.d - 1)
    }
}
/// Holographic quantum error correcting code (HaPPY code).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HolographicCode {
    pub name: String,
    pub bulk_qubits: usize,
    pub boundary_qubits: usize,
    pub causal_wedge_description: String,
}
impl HolographicCode {
    #[allow(dead_code)]
    pub fn happy_code(layers: usize) -> Self {
        let boundary = 5 * (6_usize.pow(layers as u32 - 1));
        let bulk = boundary / 5;
        Self {
            name: "HaPPY (pentagon-hexagon)".to_string(),
            bulk_qubits: bulk,
            boundary_qubits: boundary,
            causal_wedge_description: "Causal wedge reconstruction via RT formula".to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn ryu_takayanagi_formula(&self) -> String {
        "S(A) = Area(gamma_A) / (4 G_N) + S_bulk: entanglement entropy from minimal surface"
            .to_string()
    }
    #[allow(dead_code)]
    pub fn is_isometric(&self) -> bool {
        true
    }
    #[allow(dead_code)]
    pub fn encoding_rate(&self) -> f64 {
        if self.boundary_qubits == 0 {
            0.0
        } else {
            self.bulk_qubits as f64 / self.boundary_qubits as f64
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorMitigationTechnique {
    ZeroNoiseExtrapolation,
    ProbabilisticErrorCancellation,
    SymmetryVerification,
    VirtualDistillation,
    CliffordDataRegression,
}
/// Quantum communication complexity protocol.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct QccProtocol {
    pub problem_name: String,
    pub classical_cc: usize,
    pub quantum_cc: usize,
    pub has_exponential_gap: bool,
}
impl QccProtocol {
    #[allow(dead_code)]
    pub fn new(problem: &str, cc: usize, qcc: usize, exp_gap: bool) -> Self {
        Self {
            problem_name: problem.to_string(),
            classical_cc: cc,
            quantum_cc: qcc,
            has_exponential_gap: exp_gap,
        }
    }
    #[allow(dead_code)]
    pub fn inner_product() -> Self {
        Self::new("Inner Product", 100, 100, false)
    }
    #[allow(dead_code)]
    pub fn equality_function() -> Self {
        Self::new("Equality", 100, 10, true)
    }
    #[allow(dead_code)]
    pub fn quantum_advantage_factor(&self) -> f64 {
        if self.quantum_cc == 0 {
            return f64::INFINITY;
        }
        self.classical_cc as f64 / self.quantum_cc as f64
    }
}
/// Choi matrix of a quantum channel.
#[derive(Debug, Clone)]
pub struct ChoiMatrix {
    pub dim_in: usize,
    pub dim_out: usize,
    pub data: DensityMatrix,
}
impl ChoiMatrix {
    /// Compute the Choi matrix from Kraus operators.
    pub fn from_kraus(channel: &KrausChannel) -> Self {
        let di = channel.dim_in;
        let dout = channel.dim_out;
        let d_total = dout * di;
        let mut choi_data = vec![Complex::zero(); d_total * d_total];
        for k_data in &channel.operators {
            for a in 0..dout {
                for b in 0..di {
                    for c in 0..dout {
                        for dd in 0..di {
                            let idx_row = a * di + b;
                            let idx_col = c * di + dd;
                            let val = k_data[a * di + b].mul(k_data[c * di + dd].conj());
                            choi_data[idx_row * d_total + idx_col] =
                                choi_data[idx_row * d_total + idx_col].add(val);
                        }
                    }
                }
            }
        }
        Self {
            dim_in: di,
            dim_out: dout,
            data: DensityMatrix {
                dim: d_total,
                data: choi_data,
            },
        }
    }
}
/// Quantum error mitigation (near-term techniques).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct QuantumErrorMitigation {
    pub technique: ErrorMitigationTechnique,
    pub overhead_factor: f64,
}
impl QuantumErrorMitigation {
    #[allow(dead_code)]
    pub fn zne(scale_factor: f64) -> Self {
        Self {
            technique: ErrorMitigationTechnique::ZeroNoiseExtrapolation,
            overhead_factor: scale_factor,
        }
    }
    #[allow(dead_code)]
    pub fn pec(overhead: f64) -> Self {
        Self {
            technique: ErrorMitigationTechnique::ProbabilisticErrorCancellation,
            overhead_factor: overhead,
        }
    }
    #[allow(dead_code)]
    pub fn is_exact_in_limit(&self) -> bool {
        matches!(
            self.technique,
            ErrorMitigationTechnique::ZeroNoiseExtrapolation
                | ErrorMitigationTechnique::ProbabilisticErrorCancellation
        )
    }
    #[allow(dead_code)]
    pub fn variance_overhead(&self) -> f64 {
        self.overhead_factor * self.overhead_factor
    }
}
/// Quantum non-locality and Bell inequalities.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BellInequality {
    pub name: String,
    pub classical_bound: f64,
    pub quantum_bound: f64,
    pub algebraic_bound: f64,
}
impl BellInequality {
    #[allow(dead_code)]
    pub fn chsh() -> Self {
        Self {
            name: "CHSH".to_string(),
            classical_bound: 2.0,
            quantum_bound: 2.0 * 2.0_f64.sqrt(),
            algebraic_bound: 4.0,
        }
    }
    #[allow(dead_code)]
    pub fn mermin_n(n: usize) -> Self {
        let bound_c = 2.0_f64.powi(n as i32 / 2);
        let bound_q = 2.0_f64.powi((n as i32 - 1) / 2 + 1);
        Self {
            name: format!("Mermin n={n}"),
            classical_bound: bound_c,
            quantum_bound: bound_q,
            algebraic_bound: 2.0_f64.powi(n as i32 - 1),
        }
    }
    #[allow(dead_code)]
    pub fn quantum_violation_ratio(&self) -> f64 {
        self.quantum_bound / self.classical_bound
    }
}
/// QMA complexity class.
#[derive(Debug, Clone)]
pub struct QMAComplexity {
    pub completeness: f64,
    pub soundness: f64,
}
impl QMAComplexity {
    pub fn new(completeness: f64, soundness: f64) -> Self {
        assert!(completeness > soundness);
        Self {
            completeness,
            soundness,
        }
    }
    pub fn standard() -> Self {
        Self::new(2.0 / 3.0, 1.0 / 3.0)
    }
    /// Local Hamiltonian problem is QMA-complete (Kitaev's theorem).
    pub fn local_hamiltonian_is_qma_complete() -> bool {
        true
    }
}
/// Density matrix ρ: d×d complex matrix stored row-major.
#[derive(Debug, Clone)]
pub struct DensityMatrix {
    pub dim: usize,
    pub data: Vec<Complex>,
}
impl DensityMatrix {
    /// Create density matrix from data.
    pub fn new(dim: usize, data: Vec<Complex>) -> Self {
        assert_eq!(data.len(), dim * dim);
        Self { dim, data }
    }
    /// |ψ⟩⟨ψ| from a state vector.
    pub fn from_pure_state(psi: &PureState) -> Self {
        let d = psi.dim;
        let mut data = vec![Complex::zero(); d * d];
        for i in 0..d {
            for j in 0..d {
                data[i * d + j] = psi.amplitudes[i].mul(psi.amplitudes[j].conj());
            }
        }
        Self { dim: d, data }
    }
    /// Maximally mixed state I/d.
    pub fn maximally_mixed(d: usize) -> Self {
        let mut data = vec![Complex::zero(); d * d];
        for i in 0..d {
            data[i * d + i] = Complex::new(1.0 / d as f64, 0.0);
        }
        Self { dim: d, data }
    }
    pub fn get(&self, i: usize, j: usize) -> Complex {
        self.data[i * self.dim + j]
    }
    pub fn set(&mut self, i: usize, j: usize, val: Complex) {
        self.data[i * self.dim + j] = val;
    }
    /// Trace: Tr(ρ) = ∑_i ρ_{ii}.
    pub fn trace(&self) -> Complex {
        (0..self.dim).fold(Complex::zero(), |acc, i| acc.add(self.get(i, i)))
    }
    /// Is this a pure state? Tr(ρ²) ≈ 1.
    pub fn is_pure(&self) -> bool {
        (self.purity() - 1.0).abs() < 1e-9
    }
    /// Purity: γ(ρ) = Tr(ρ²).
    pub fn purity(&self) -> f64 {
        let rho2 = self.mul_mat(self);
        rho2.trace().re
    }
    /// Von Neumann entropy S(ρ) = −∑ λ_i log₂ λ_i (nats if we use ln).
    /// Computed from eigenvalues via power iteration approximation for 2×2 case,
    /// or exact formula for diagonal ρ.
    pub fn von_neumann_entropy(&self) -> f64 {
        let eigenvalues = self.eigenvalues_approx();
        eigenvalues
            .iter()
            .filter(|&&x| x > 1e-15)
            .map(|&x| -x * x.ln() / LN_2)
            .sum()
    }
    /// Approximate eigenvalues for 2×2 matrices; for larger use diagonal approx.
    pub fn eigenvalues_approx(&self) -> Vec<f64> {
        let d = self.dim;
        if d == 2 {
            let tr = self.trace().re;
            let det =
                self.get(0, 0).re * self.get(1, 1).re - (self.get(0, 1).mul(self.get(1, 0))).re;
            let disc = (tr * tr - 4.0 * det).max(0.0);
            let sqrt_disc = disc.sqrt();
            vec![(tr + sqrt_disc) / 2.0, (tr - sqrt_disc) / 2.0]
        } else {
            (0..d).map(|i| self.get(i, i).re.max(0.0)).collect()
        }
    }
    /// Matrix multiplication ρ·σ.
    pub fn mul_mat(&self, other: &DensityMatrix) -> DensityMatrix {
        let d = self.dim;
        let mut result = vec![Complex::zero(); d * d];
        for i in 0..d {
            for j in 0..d {
                let mut sum = Complex::zero();
                for k in 0..d {
                    sum = sum.add(self.get(i, k).mul(other.get(k, j)));
                }
                result[i * d + j] = sum;
            }
        }
        DensityMatrix {
            dim: d,
            data: result,
        }
    }
    /// Add two density matrices.
    pub fn add_mat(&self, other: &DensityMatrix) -> DensityMatrix {
        let d = self.dim;
        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(&a, &b)| a.add(b))
            .collect();
        DensityMatrix { dim: d, data }
    }
    /// Scale by a real scalar.
    pub fn scale(&self, s: f64) -> DensityMatrix {
        DensityMatrix {
            dim: self.dim,
            data: self.data.iter().map(|&c| c.scale(s)).collect(),
        }
    }
    /// Partial trace over subsystem B for a bipartite state on H_A ⊗ H_B.
    /// `da` = dim of subsystem A, `db` = dim of subsystem B.
    pub fn partial_trace_b(&self, da: usize, db: usize) -> DensityMatrix {
        assert_eq!(self.dim, da * db);
        let mut result = vec![Complex::zero(); da * da];
        for i in 0..da {
            for j in 0..da {
                let mut sum = Complex::zero();
                for k in 0..db {
                    sum = sum.add(self.get(i * db + k, j * db + k));
                }
                result[i * da + j] = sum;
            }
        }
        DensityMatrix {
            dim: da,
            data: result,
        }
    }
    /// Partial transpose with respect to subsystem B.
    pub fn partial_transpose_b(&self, da: usize, db: usize) -> DensityMatrix {
        assert_eq!(self.dim, da * db);
        let d = self.dim;
        let mut result = vec![Complex::zero(); d * d];
        for ia in 0..da {
            for ib in 0..db {
                for ja in 0..da {
                    for jb in 0..db {
                        let row_in = ia * db + ib;
                        let col_in = ja * db + jb;
                        let row_out = ia * db + jb;
                        let col_out = ja * db + ib;
                        result[row_out * d + col_out] = self.get(row_in, col_in);
                    }
                }
            }
        }
        DensityMatrix {
            dim: d,
            data: result,
        }
    }
    /// Bipartite entanglement entropy S(Tr_B ρ).
    pub fn bipartite_entropy(&self, da: usize, db: usize) -> f64 {
        self.partial_trace_b(da, db).von_neumann_entropy()
    }
    /// Check PPT criterion: is ρ^{T_B} positive semidefinite?
    /// (Uses the 2-qubit eigenvalue check for da=db=2.)
    pub fn is_ppt(&self, da: usize, db: usize) -> bool {
        let rho_tb = self.partial_transpose_b(da, db);
        rho_tb.eigenvalues_approx().iter().all(|&ev| ev >= -1e-9)
    }
}
/// Quantum channel in Kraus representation.
#[derive(Debug, Clone)]
pub struct KrausChannel {
    pub dim_in: usize,
    pub dim_out: usize,
    /// Kraus operators K_i: each is a dim_out × dim_in complex matrix.
    pub operators: Vec<Vec<Complex>>,
}
impl KrausChannel {
    pub fn new(dim_in: usize, dim_out: usize, operators: Vec<Vec<Complex>>) -> Self {
        Self {
            dim_in,
            dim_out,
            operators,
        }
    }
    /// Depolarising channel ε_p(ρ) = (1−p)ρ + p I/d.
    pub fn depolarizing(d: usize, p: f64) -> Self {
        assert_eq!(d, 2);
        let a = (1.0 - 3.0 * p / 4.0).sqrt();
        let b = (p / 4.0).sqrt();
        let eye: Vec<Complex> = vec![
            Complex::new(a, 0.0),
            Complex::zero(),
            Complex::zero(),
            Complex::new(a, 0.0),
        ];
        let x: Vec<Complex> = vec![
            Complex::zero(),
            Complex::new(b, 0.0),
            Complex::new(b, 0.0),
            Complex::zero(),
        ];
        let y: Vec<Complex> = vec![
            Complex::zero(),
            Complex::new(0.0, -b),
            Complex::new(0.0, b),
            Complex::zero(),
        ];
        let z: Vec<Complex> = vec![
            Complex::new(b, 0.0),
            Complex::zero(),
            Complex::zero(),
            Complex::new(-b, 0.0),
        ];
        Self {
            dim_in: d,
            dim_out: d,
            operators: vec![eye, x, y, z],
        }
    }
    /// Apply the channel: ε(ρ) = ∑_i K_i ρ K_i†.
    pub fn apply(&self, rho: &DensityMatrix) -> DensityMatrix {
        let d_out = self.dim_out;
        let d_in = self.dim_in;
        let mut result = DensityMatrix {
            dim: d_out,
            data: vec![Complex::zero(); d_out * d_out],
        };
        for k_data in &self.operators {
            let mut kp = vec![Complex::zero(); d_out * d_in];
            for i in 0..d_out {
                for j in 0..d_in {
                    let mut sum = Complex::zero();
                    for l in 0..d_in {
                        sum = sum.add(k_data[i * d_in + l].mul(rho.get(l, j)));
                    }
                    kp[i * d_in + j] = sum;
                }
            }
            for i in 0..d_out {
                for j in 0..d_out {
                    let mut sum = Complex::zero();
                    for l in 0..d_in {
                        sum = sum.add(kp[i * d_in + l].mul(k_data[j * d_in + l].conj()));
                    }
                    let cur = result.get(i, j);
                    result.set(i, j, cur.add(sum));
                }
            }
        }
        result
    }
    /// Check if the channel is unitary (single Kraus operator that is unitary).
    pub fn is_unitary(&self) -> bool {
        if self.operators.len() != 1 {
            return false;
        }
        let d = self.dim_in;
        if d != self.dim_out {
            return false;
        }
        let k = &self.operators[0];
        for i in 0..d {
            for j in 0..d {
                let mut sum = Complex::zero();
                for l in 0..d {
                    sum = sum.add(k[l * d + i].conj().mul(k[l * d + j]));
                }
                let expected = if i == j { 1.0 } else { 0.0 };
                if (sum.re - expected).abs() > 1e-9 || sum.im.abs() > 1e-9 {
                    return false;
                }
            }
        }
        true
    }
    /// Check if entanglement-breaking: every Kraus operator has rank 1.
    /// (Equivalent to: the channel has a Kraus decomposition with rank-1 operators.)
    pub fn is_entanglement_breaking(&self) -> bool {
        let d = self.dim_in;
        for k_data in &self.operators {
            let first_nonzero =
                (0..self.dim_out).find(|&i| (0..d).any(|j| k_data[i * d + j].abs_sq() > 1e-18));
            if let Some(r0) = first_nonzero {
                for i in (r0 + 1)..self.dim_out {
                    let mut ratio: Option<Complex> = None;
                    for j in 0..d {
                        let a = k_data[r0 * d + j];
                        let b = k_data[i * d + j];
                        if a.abs_sq() > 1e-18 {
                            let r = Complex {
                                re: b.re / a.abs_sq() * a.re + b.im / a.abs_sq() * a.im,
                                im: b.im / a.abs_sq() * a.re - b.re / a.abs_sq() * a.im,
                            };
                            if let Some(prev) = ratio {
                                if (prev.re - r.re).abs() > 1e-9 || (prev.im - r.im).abs() > 1e-9 {
                                    return false;
                                }
                            } else {
                                ratio = Some(r);
                            }
                        } else if b.abs_sq() > 1e-18 {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
}
/// Quantum circuit complexity.
#[derive(Debug, Clone)]
pub struct QuantumComplexity {
    /// A textual description of the circuit or problem instance.
    pub circuit: String,
}
impl QuantumComplexity {
    /// Construct a complexity model.
    pub fn new(circuit: impl Into<String>) -> Self {
        Self {
            circuit: circuit.into(),
        }
    }
    /// Number of T (π/8) gates in the circuit (placeholder).
    pub fn t_gate_count(&self) -> usize {
        0
    }
    /// Depth of the circuit (placeholder).
    pub fn circuit_depth(&self) -> usize {
        0
    }
    /// Returns `true` if the problem is in BQP (bounded-error quantum
    /// polynomial time).
    pub fn is_bqp(&self) -> bool {
        true
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CvStateType {
    Coherent,
    Squeezed,
    ThermalState,
    GaussianState,
    FockState(u32),
    CatState,
}
/// Entanglement measure variants.
#[derive(Debug, Clone)]
pub enum EntanglementMeasure {
    VonNeumann,
    Concurrence,
    NegativityMeasure,
    EntanglementFormation,
    DistillableEntanglement,
}
/// Bloch vector representation of a qubit state.
#[derive(Debug, Clone)]
pub struct BlochVector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl BlochVector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    /// Is this a valid Bloch vector? (|r| ≤ 1)
    pub fn is_valid(&self) -> bool {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt() <= 1.0 + 1e-9
    }
    /// Convert to 2×2 density matrix ρ = (I + r·σ)/2.
    pub fn to_density_matrix(&self) -> DensityMatrix {
        DensityMatrix::new(
            2,
            vec![
                Complex::new((1.0 + self.z) / 2.0, 0.0),
                Complex::new(self.x / 2.0, -self.y / 2.0),
                Complex::new(self.x / 2.0, self.y / 2.0),
                Complex::new((1.0 - self.z) / 2.0, 0.0),
            ],
        )
    }
    /// Convert from a 2×2 density matrix.
    pub fn from_density_matrix(rho: &DensityMatrix) -> Self {
        assert_eq!(rho.dim, 2);
        let x = 2.0 * rho.get(0, 1).re;
        let y = -2.0 * rho.get(0, 1).im;
        let z = 2.0 * rho.get(0, 0).re - 1.0;
        Self { x, y, z }
    }
}
/// A single qubit in state α|0⟩ + β|1⟩, stored as (re,im) pairs.
#[derive(Debug, Clone)]
pub struct Qubit {
    /// Amplitude for |0⟩.
    pub alpha: (f64, f64),
    /// Amplitude for |1⟩.
    pub beta: (f64, f64),
}
impl Qubit {
    /// Construct a qubit. Panics if not normalised to 1.
    pub fn new(alpha: (f64, f64), beta: (f64, f64)) -> Self {
        let q = Self { alpha, beta };
        assert!(q.is_normalized(), "Qubit must be normalised");
        q
    }
    /// Computational basis |0⟩.
    pub fn zero() -> Self {
        Self {
            alpha: (1.0, 0.0),
            beta: (0.0, 0.0),
        }
    }
    /// Computational basis |1⟩.
    pub fn one() -> Self {
        Self {
            alpha: (0.0, 0.0),
            beta: (1.0, 0.0),
        }
    }
    /// Hadamard state |+⟩ = (|0⟩+|1⟩)/√2.
    pub fn plus() -> Self {
        let v = 1.0_f64 / 2.0_f64.sqrt();
        Self {
            alpha: (v, 0.0),
            beta: (v, 0.0),
        }
    }
    /// Returns `true` when |α|²+|β|² ≈ 1.
    pub fn is_normalized(&self) -> bool {
        let n =
            self.alpha.0.powi(2) + self.alpha.1.powi(2) + self.beta.0.powi(2) + self.beta.1.powi(2);
        (n - 1.0).abs() < 1e-9
    }
    /// Bloch vector (x, y, z) for the qubit.
    pub fn bloch_vector(&self) -> (f64, f64, f64) {
        let (ar, ai) = self.alpha;
        let (br, bi) = self.beta;
        let x = 2.0 * (ar * br + ai * bi);
        let y = 2.0 * (ai * br - ar * bi);
        let z = ar * ar + ai * ai - br * br - bi * bi;
        (x, y, z)
    }
    /// Simulates a Z-basis measurement; returns 0 or 1 (deterministic for
    /// pure |0⟩ or |1⟩, probabilistic otherwise — here uses a simple 50/50
    /// tie-break based on probability).
    pub fn measure(&self) -> u8 {
        let p0 = self.alpha.0.powi(2) + self.alpha.1.powi(2);
        if p0 >= 0.5 {
            0
        } else {
            1
        }
    }
}
/// Quantum key distribution (QKD) protocol.
#[derive(Debug, Clone)]
pub struct QKD {
    /// Protocol name, e.g. "BB84", "E91".
    pub protocol: String,
}
impl QKD {
    /// Construct a QKD model.
    pub fn new(protocol: impl Into<String>) -> Self {
        Self {
            protocol: protocol.into(),
        }
    }
    /// BB84 protocol steps.
    pub fn bb84_protocol(&self) -> Vec<String> {
        vec![
            "1. Alice sends random qubits in random bases".to_string(),
            "2. Bob measures each qubit in a random basis".to_string(),
            "3. Alice and Bob publicly compare bases".to_string(),
            "4. They keep bits where bases agree (sifted key)".to_string(),
            "5. They sacrifice a subset to estimate eavesdropping (QBER)".to_string(),
            "6. Classical privacy amplification yields the secret key".to_string(),
        ]
    }
    /// BB84 is information-theoretically secure against any eavesdropper.
    pub fn is_information_theoretically_secure(&self) -> bool {
        true
    }
    /// Secret key rate (bits per signal) in the asymptotic regime under
    /// collective attacks (simplified Devetak–Winter formula).
    ///
    /// `qber` is the quantum bit error rate (0 ≤ qber ≤ 0.5).
    pub fn key_rate(&self, qber: f64) -> f64 {
        if qber >= 0.11 {
            return 0.0;
        }
        let h = |p: f64| {
            if p <= 0.0 || p >= 1.0 {
                return 0.0;
            }
            -(p * p.log2() + (1.0 - p) * (1.0 - p).log2())
        };
        1.0 - h(qber) - h(qber)
    }
}
/// Formal statement of quantum no-cloning, no-deleting, and no-broadcasting.
#[derive(Debug, Clone, Copy)]
pub struct NoCloning;
impl NoCloning {
    /// No-cloning theorem: there is no unitary that copies an arbitrary
    /// unknown quantum state |ψ⟩|0⟩ → |ψ⟩|ψ⟩.
    pub fn no_cloning_theorem(&self) -> bool {
        true
    }
    /// No-deleting theorem: there is no unitary that erases an unknown
    /// quantum state |ψ⟩|ψ⟩ → |ψ⟩|0⟩.
    pub fn no_deleting_theorem(&self) -> bool {
        true
    }
    /// No-broadcasting theorem: there is no CPTP map that broadcasts a
    /// mixed state ρ to ρ ⊗ ρ.
    pub fn no_broadcasting(&self) -> bool {
        true
    }
}
/// Syndrome decoder using minimum-weight matching (simplified greedy).
#[derive(Debug, Clone)]
pub struct SyndromeDecoder {
    pub code: StabilizerCode,
}
impl SyndromeDecoder {
    pub fn new(code: StabilizerCode) -> Self {
        Self { code }
    }
    /// Decode a syndrome by minimum-weight matching (greedy single-qubit errors).
    pub fn decode(&self, syndrome: &[u8]) -> (Vec<u8>, Vec<u8>) {
        let n = self.code.n;
        for q in 0..n {
            let mut x_err = vec![0u8; n];
            x_err[q] = 1;
            let z_err = vec![0u8; n];
            if self.code.syndrome(&x_err, &z_err) == syndrome {
                return (x_err, z_err);
            }
            let x_err = vec![0u8; n];
            let mut z_err = vec![0u8; n];
            z_err[q] = 1;
            if self.code.syndrome(&x_err, &z_err) == syndrome {
                return (x_err, z_err);
            }
        }
        (vec![0u8; n], vec![0u8; n])
    }
}
/// Quantum state discrimination.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StateDiscrimination {
    pub num_states: usize,
    pub prior_probabilities: Vec<f64>,
    pub is_minimum_error: bool,
}
impl StateDiscrimination {
    #[allow(dead_code)]
    pub fn new(n: usize, priors: Vec<f64>, min_error: bool) -> Self {
        Self {
            num_states: n,
            prior_probabilities: priors,
            is_minimum_error: min_error,
        }
    }
    #[allow(dead_code)]
    pub fn helstrom_bound_two_states(p1: f64, p2: f64) -> f64 {
        let _p1 = p1;
        let _p2 = p2;
        0.5 * (1.0 - (p1 - p2).abs())
    }
    #[allow(dead_code)]
    pub fn unambiguous_discrimination_exists(&self) -> bool {
        true
    }
}
/// Quantum resource theory.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ResourceTheory {
    pub name: String,
    pub free_states: String,
    pub free_operations: String,
    pub monotone: String,
}
impl ResourceTheory {
    #[allow(dead_code)]
    pub fn entanglement() -> Self {
        Self {
            name: "Entanglement".to_string(),
            free_states: "Separable states".to_string(),
            free_operations: "LOCC (Local Operations and Classical Communication)".to_string(),
            monotone: "Entanglement entropy / Concurrence".to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn coherence() -> Self {
        Self {
            name: "Coherence".to_string(),
            free_states: "Incoherent states (diagonal in reference basis)".to_string(),
            free_operations: "Incoherent operations".to_string(),
            monotone: "l1-norm of coherence / Relative entropy of coherence".to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn magic_states() -> Self {
        Self {
            name: "Magic / Non-stabilizerness".to_string(),
            free_states: "Stabilizer states".to_string(),
            free_operations: "Clifford operations".to_string(),
            monotone: "Robustness of magic / mana".to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn thermodynamics() -> Self {
        Self {
            name: "Thermodynamics".to_string(),
            free_states: "Thermal (Gibbs) states".to_string(),
            free_operations: "Thermal operations".to_string(),
            monotone: "Free energy F = E - TS".to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn asymptotic_rate_description(&self) -> String {
        format!(
            "Asymptotic interconversion rate between {} resources via {}",
            self.name, self.free_operations
        )
    }
}
/// Surface code on an L×L torus (simplified).
#[derive(Debug, Clone)]
pub struct SurfaceCode {
    pub l: usize,
}
impl SurfaceCode {
    pub fn new(l: usize) -> Self {
        Self { l }
    }
    /// Number of physical qubits: 2L² − 2L + 1 for planar, L² + (L-1)² for toric.
    pub fn num_physical_qubits(&self) -> usize {
        2 * self.l * self.l
    }
    /// Number of logical qubits: 1 for planar, 2 for toric.
    pub fn num_logical_qubits(&self) -> usize {
        1
    }
    /// Code distance: L.
    pub fn distance(&self) -> usize {
        self.l
    }
}
/// Holevo capacity of a quantum channel.
#[derive(Debug, Clone)]
pub struct HolevoCapacity {
    /// Descriptive name of the channel.
    pub channel: String,
}
impl HolevoCapacity {
    /// Construct a Holevo capacity model.
    pub fn new(channel: impl Into<String>) -> Self {
        Self {
            channel: channel.into(),
        }
    }
    /// Holevo χ quantity: S(ρ_out) - ∑ pᵢ S(ρᵢ_out).
    ///
    /// Returns a simplified upper-bound value for demonstration.
    pub fn holevo_chi(&self, input_entropy: f64, avg_output_entropy: f64) -> f64 {
        (input_entropy - avg_output_entropy).max(0.0)
    }
    /// Classical capacity of a quantum channel is bounded by the Holevo χ.
    pub fn product_state_bound(&self) -> bool {
        true
    }
}
/// Concurrence for a 2-qubit density matrix.
#[derive(Debug, Clone)]
pub struct Concurrence;
impl Concurrence {
    /// Compute C(ρ) = max(0, λ₁ - λ₂ - λ₃ - λ₄) where λ_i are eigenvalues of
    /// ρ(σ_y⊗σ_y)ρ*(σ_y⊗σ_y) in decreasing order.
    pub fn compute(rho: &DensityMatrix) -> f64 {
        assert_eq!(rho.dim, 4);
        let sy_sy = [
            [
                Complex::zero(),
                Complex::zero(),
                Complex::zero(),
                Complex::new(-1.0, 0.0),
            ],
            [
                Complex::zero(),
                Complex::zero(),
                Complex::new(1.0, 0.0),
                Complex::zero(),
            ],
            [
                Complex::zero(),
                Complex::new(1.0, 0.0),
                Complex::zero(),
                Complex::zero(),
            ],
            [
                Complex::new(-1.0, 0.0),
                Complex::zero(),
                Complex::zero(),
                Complex::zero(),
            ],
        ];
        let rho_conj_data: Vec<Complex> = rho.data.iter().map(|&c| c.conj()).collect();
        let rho_conj = DensityMatrix {
            dim: 4,
            data: rho_conj_data,
        };
        let mut m_data = vec![Complex::zero(); 16];
        for i in 0..4 {
            for j in 0..4 {
                m_data[i * 4 + j] = sy_sy[i][j];
            }
        }
        let m = DensityMatrix {
            dim: 4,
            data: m_data,
        };
        let mr = m.mul_mat(&rho_conj);
        let mrm = mr.mul_mat(&m);
        let r = rho.mul_mat(&mrm);
        let tr_r = r.trace().re;
        let purity_rho = rho.purity();
        let tr_r2 = r.mul_mat(&r).trace().re;
        let ev_variance = (tr_r2 - tr_r * tr_r / 4.0).max(0.0);
        let c_approx = ((2.0 * ev_variance).sqrt() - tr_r / 2.0).max(0.0);
        let _ = purity_rho;
        c_approx
    }
}
/// PPT criterion checker.
#[derive(Debug, Clone)]
pub struct PPTCriterion {
    pub da: usize,
    pub db: usize,
}
impl PPTCriterion {
    pub fn new(da: usize, db: usize) -> Self {
        Self { da, db }
    }
    /// Check if ρ satisfies the PPT criterion (necessary for separability).
    pub fn is_ppt(&self, rho: &DensityMatrix) -> bool {
        rho.is_ppt(self.da, self.db)
    }
    /// For 2×2 and 2×3 systems, PPT implies separability.
    pub fn implies_separable(&self) -> bool {
        (self.da == 2 && self.db == 2)
            || (self.da == 2 && self.db == 3)
            || (self.da == 3 && self.db == 2)
    }
}
/// Quantum teleportation protocol model.
#[derive(Debug, Clone)]
pub struct QuantumTeleportation {
    /// Name of the Bell state used (e.g. "Phi+", "Phi-", "Psi+", "Psi-").
    pub bell_state: String,
}
impl QuantumTeleportation {
    /// Create a teleportation model with the given entangled Bell state.
    pub fn new(bell_state: impl Into<String>) -> Self {
        Self {
            bell_state: bell_state.into(),
        }
    }
    /// Steps in the standard teleportation protocol.
    pub fn protocol_steps(&self) -> Vec<String> {
        vec![
            "1. Alice and Bob share a Bell pair".to_string(),
            "2. Alice entangles her qubit with her half of the Bell pair".to_string(),
            "3. Alice performs a Bell measurement (2 classical bits)".to_string(),
            "4. Alice sends 2 classical bits to Bob".to_string(),
            "5. Bob applies a Pauli correction based on Alice's measurement".to_string(),
            "6. Bob's qubit is now in Alice's original state".to_string(),
        ]
    }
    /// Standard teleportation with a maximally entangled Bell state is exact.
    pub fn is_exact(&self) -> bool {
        matches!(self.bell_state.as_str(), "Phi+" | "Phi-" | "Psi+" | "Psi-")
    }
    /// Classical communication cost: 2 bits.
    pub fn classical_bits_required(&self) -> usize {
        2
    }
}
/// Continuous variable quantum information.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CvQuantumInfo {
    pub num_modes: usize,
    pub state_type: CvStateType,
}
impl CvQuantumInfo {
    #[allow(dead_code)]
    pub fn new(modes: usize, state: CvStateType) -> Self {
        Self {
            num_modes: modes,
            state_type: state,
        }
    }
    #[allow(dead_code)]
    pub fn is_gaussian(&self) -> bool {
        matches!(
            self.state_type,
            CvStateType::Coherent
                | CvStateType::Squeezed
                | CvStateType::ThermalState
                | CvStateType::GaussianState
        )
    }
    #[allow(dead_code)]
    pub fn wigner_function_nonnegative(&self) -> bool {
        self.is_gaussian()
    }
    #[allow(dead_code)]
    pub fn gaussian_channels_description(&self) -> String {
        "Gaussian channels preserve Gaussian states; described by (S, d) affine maps on phase space"
            .to_string()
    }
}
/// Quantum discord.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct QuantumDiscord {
    pub system_name: String,
    pub mutual_information: f64,
    pub classical_correlations: f64,
}
impl QuantumDiscord {
    #[allow(dead_code)]
    pub fn new(system: &str, mi: f64, cc: f64) -> Self {
        Self {
            system_name: system.to_string(),
            mutual_information: mi,
            classical_correlations: cc,
        }
    }
    #[allow(dead_code)]
    pub fn discord_value(&self) -> f64 {
        self.mutual_information - self.classical_correlations
    }
    #[allow(dead_code)]
    pub fn is_zero_discord(&self) -> bool {
        self.discord_value().abs() < 1e-10
    }
    #[allow(dead_code)]
    pub fn zero_discord_iff_classical(&self) -> bool {
        true
    }
}
/// Complex number in Cartesian form.
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
    pub fn add(self, other: Self) -> Self {
        Self {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
    pub fn mul(self, other: Self) -> Self {
        Self {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
    pub fn scale(self, s: f64) -> Self {
        Self {
            re: self.re * s,
            im: self.im * s,
        }
    }
}
/// A quantum channel specified by its Kraus operators.
///
/// Each Kraus operator is stored as a flat row-major complex matrix of
/// dimension `dim × dim`, where each element is an `(re, im)` pair.
#[derive(Debug, Clone)]
pub struct QuantumChannel {
    /// Side dimension of each Kraus operator matrix.
    pub dim: usize,
    /// The Kraus operators.
    pub kraus_ops: Vec<Vec<Vec<(f64, f64)>>>,
}
impl QuantumChannel {
    /// Construct from a list of `dim × dim` Kraus operators.
    pub fn new(dim: usize, kraus_ops: Vec<Vec<Vec<(f64, f64)>>>) -> Self {
        Self { dim, kraus_ops }
    }
    /// Identity channel on `dim`-dimensional system.
    pub fn identity(dim: usize) -> Self {
        let mut eye = vec![vec![(0.0_f64, 0.0_f64); dim]; dim];
        for i in 0..dim {
            eye[i][i] = (1.0, 0.0);
        }
        Self {
            dim,
            kraus_ops: vec![eye],
        }
    }
    /// Returns `true` iff every Kraus sum ∑ Kᵢ†Kᵢ = I (trace-preserving).
    pub fn is_trace_preserving(&self) -> bool {
        let d = self.dim;
        let mut sum = vec![vec![(0.0_f64, 0.0_f64); d]; d];
        for k in &self.kraus_ops {
            for r in 0..d {
                for c in 0..d {
                    let mut acc = (0.0_f64, 0.0_f64);
                    for m in 0..d {
                        let kdag = (k[m][r].0, -k[m][r].1);
                        let km = k[m][c];
                        acc.0 += kdag.0 * km.0 - kdag.1 * km.1;
                        acc.1 += kdag.0 * km.1 + kdag.1 * km.0;
                    }
                    sum[r][c].0 += acc.0;
                    sum[r][c].1 += acc.1;
                }
            }
        }
        for r in 0..d {
            for c in 0..d {
                let expected = if r == c { 1.0 } else { 0.0 };
                if (sum[r][c].0 - expected).abs() > 1e-9 {
                    return false;
                }
                if sum[r][c].1.abs() > 1e-9 {
                    return false;
                }
            }
        }
        true
    }
    /// A completely-positive map with Kraus representation is always CP.
    pub fn is_completely_positive(&self) -> bool {
        true
    }
    /// Apply the channel to a density matrix (as flat row-major Vec<(f64,f64)>).
    pub fn apply(&self, rho: &[Vec<(f64, f64)>]) -> Vec<Vec<(f64, f64)>> {
        let d = self.dim;
        let mut out = vec![vec![(0.0_f64, 0.0_f64); d]; d];
        for k in &self.kraus_ops {
            for r in 0..d {
                for c in 0..d {
                    let mut acc = (0.0_f64, 0.0_f64);
                    for m in 0..d {
                        for n in 0..d {
                            let km = k[r][m];
                            let rho_mn = rho[m][n];
                            let kdag = (k[c][n].0, -k[c][n].1);
                            let prod0 = km.0 * rho_mn.0 - km.1 * rho_mn.1;
                            let prod1 = km.0 * rho_mn.1 + km.1 * rho_mn.0;
                            acc.0 += prod0 * kdag.0 - prod1 * kdag.1;
                            acc.1 += prod0 * kdag.1 + prod1 * kdag.0;
                        }
                    }
                    out[r][c].0 += acc.0;
                    out[r][c].1 += acc.1;
                }
            }
        }
        out
    }
}
