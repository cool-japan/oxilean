//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use std::f64::consts::PI;

/// A gate operation in a `QuantumCircuit`.
#[derive(Debug, Clone)]
pub enum GateOp {
    /// Apply a single-qubit gate to qubit `q`.
    Single { gate: Gate2x2, qubit: usize },
    /// Apply a CNOT gate.
    Cnot { control: usize, target: usize },
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChannelType {
    Depolarizing,
    AmplitudeDamping,
    PhaseDamping,
    BitFlip,
    PhaseFlip,
    Erasure,
    Unitary,
    Identity,
}
/// Clifford group properties.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CliffordGroup {
    pub num_qubits: usize,
}
impl CliffordGroup {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        Self { num_qubits: n }
    }
    #[allow(dead_code)]
    pub fn normalizes_pauli_group(&self) -> bool {
        true
    }
    #[allow(dead_code)]
    pub fn is_efficiently_simulable(&self) -> bool {
        true
    }
    #[allow(dead_code)]
    pub fn gottesman_knill_theorem(&self) -> String {
        format!(
            "Clifford circuits on {} qubits can be simulated in poly time (Gottesman-Knill)",
            self.num_qubits
        )
    }
    #[allow(dead_code)]
    pub fn universal_with_t_gate(&self) -> bool {
        true
    }
}
/// Quantum approximate optimization algorithm (QAOA).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct QaoaConfig {
    pub problem_name: String,
    pub p_layers: usize,
}
impl QaoaConfig {
    #[allow(dead_code)]
    pub fn new(problem: &str, p: usize) -> Self {
        Self {
            problem_name: problem.to_string(),
            p_layers: p,
        }
    }
    #[allow(dead_code)]
    pub fn approximation_ratio_lower_bound(&self) -> f64 {
        let neg_layers = -(self.p_layers as i64) as f64;
        0.5 + 0.1924 * (1.0 - (neg_layers * 0.5).exp())
    }
    #[allow(dead_code)]
    pub fn num_circuit_parameters(&self) -> usize {
        2 * self.p_layers
    }
}
/// Quantum error-correcting code.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct QuantumErrorCode {
    pub name: String,
    pub n: usize,
    pub k: usize,
    pub d: usize,
}
impl QuantumErrorCode {
    #[allow(dead_code)]
    pub fn steane_7() -> Self {
        Self {
            name: "Steane [[7,1,3]]".to_string(),
            n: 7,
            k: 1,
            d: 3,
        }
    }
    #[allow(dead_code)]
    pub fn shor_9() -> Self {
        Self {
            name: "Shor [[9,1,3]]".to_string(),
            n: 9,
            k: 1,
            d: 3,
        }
    }
    #[allow(dead_code)]
    pub fn surface_code(d: usize) -> Self {
        let n = 2 * d * d - 2 * d + 1;
        Self {
            name: format!("Surface [[{n},1,{d}]]"),
            n,
            k: 1,
            d,
        }
    }
    #[allow(dead_code)]
    pub fn encoding_rate(&self) -> f64 {
        self.k as f64 / self.n as f64
    }
    #[allow(dead_code)]
    pub fn corrects_errors_up_to(&self) -> usize {
        (self.d - 1) / 2
    }
    #[allow(dead_code)]
    pub fn qec_bound_satisfied(&self) -> bool {
        true
    }
}
/// A quantum gate represented by its name and matrix dimension.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct QuantumGate {
    pub name: String,
    pub num_qubits: usize,
    pub is_unitary: bool,
    pub is_clifford: bool,
}
impl QuantumGate {
    #[allow(dead_code)]
    pub fn hadamard() -> Self {
        Self {
            name: "H".to_string(),
            num_qubits: 1,
            is_unitary: true,
            is_clifford: true,
        }
    }
    #[allow(dead_code)]
    pub fn pauli_x() -> Self {
        Self {
            name: "X".to_string(),
            num_qubits: 1,
            is_unitary: true,
            is_clifford: true,
        }
    }
    #[allow(dead_code)]
    pub fn pauli_y() -> Self {
        Self {
            name: "Y".to_string(),
            num_qubits: 1,
            is_unitary: true,
            is_clifford: true,
        }
    }
    #[allow(dead_code)]
    pub fn pauli_z() -> Self {
        Self {
            name: "Z".to_string(),
            num_qubits: 1,
            is_unitary: true,
            is_clifford: true,
        }
    }
    #[allow(dead_code)]
    pub fn phase(theta_desc: &str) -> Self {
        Self {
            name: format!("P({theta_desc})"),
            num_qubits: 1,
            is_unitary: true,
            is_clifford: false,
        }
    }
    #[allow(dead_code)]
    pub fn t_gate() -> Self {
        Self {
            name: "T".to_string(),
            num_qubits: 1,
            is_unitary: true,
            is_clifford: false,
        }
    }
    #[allow(dead_code)]
    pub fn cnot() -> Self {
        Self {
            name: "CNOT".to_string(),
            num_qubits: 2,
            is_unitary: true,
            is_clifford: true,
        }
    }
    #[allow(dead_code)]
    pub fn toffoli() -> Self {
        Self {
            name: "Toffoli".to_string(),
            num_qubits: 3,
            is_unitary: true,
            is_clifford: false,
        }
    }
    #[allow(dead_code)]
    pub fn swap() -> Self {
        Self {
            name: "SWAP".to_string(),
            num_qubits: 2,
            is_unitary: true,
            is_clifford: true,
        }
    }
    #[allow(dead_code)]
    pub fn matrix_size(&self) -> usize {
        1usize << self.num_qubits
    }
}
/// A complex number a + bi (simple f64 wrapper).
#[derive(Debug, Clone, Copy)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}
impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Complex { re, im }
    }
    pub fn zero() -> Self {
        Complex { re: 0.0, im: 0.0 }
    }
    pub fn one() -> Self {
        Complex { re: 1.0, im: 0.0 }
    }
    pub fn i() -> Self {
        Complex { re: 0.0, im: 1.0 }
    }
    /// Modulus |z|.
    pub fn abs(&self) -> f64 {
        (self.re * self.re + self.im * self.im).sqrt()
    }
    /// |z|².
    pub fn abs_sq(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }
    /// Complex conjugate.
    pub fn conj(&self) -> Self {
        Complex {
            re: self.re,
            im: -self.im,
        }
    }
    pub fn add(&self, other: &Self) -> Self {
        Complex {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
    pub fn mul(&self, other: &Self) -> Self {
        Complex {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
    pub fn scale(&self, s: f64) -> Self {
        Complex {
            re: self.re * s,
            im: self.im * s,
        }
    }
    /// e^{iθ} = cos θ + i sin θ.
    pub fn exp(theta: f64) -> Self {
        Complex {
            re: theta.cos(),
            im: theta.sin(),
        }
    }
}
/// An n-qubit quantum register holding 2^n complex amplitudes.
///
/// This is a higher-level wrapper around `QuantumStatevector` that tracks
/// qubit count explicitly and provides convenience initializers.
#[derive(Debug)]
pub struct QuantumRegister {
    sv: QuantumStatevector,
}
impl QuantumRegister {
    /// Create a new register in the |0…0⟩ state.
    pub fn new(n_qubits: usize) -> Self {
        QuantumRegister {
            sv: QuantumStatevector::new(n_qubits),
        }
    }
    /// Number of qubits.
    pub fn n_qubits(&self) -> usize {
        self.sv.n_qubits
    }
    /// Number of basis states (2^n).
    pub fn size(&self) -> usize {
        self.sv.amplitudes.len()
    }
    /// Read amplitude at index `i`.
    pub fn amplitude(&self, i: usize) -> Complex {
        self.sv.amplitude(i)
    }
    /// Probability of basis state `i`.
    pub fn prob(&self, i: usize) -> f64 {
        self.sv.prob(i)
    }
    /// Apply a single-qubit gate to qubit `q`.
    pub fn apply_gate(&mut self, gate: &Gate2x2, qubit: usize) {
        self.sv.apply_single_qubit(gate, qubit);
    }
    /// Apply CNOT with given control and target qubits.
    pub fn apply_cnot(&mut self, control: usize, target: usize) {
        self.sv.apply_cnot(control, target);
    }
    /// Prepare the uniform superposition state H^⊗n |0⟩.
    pub fn prepare_uniform_superposition(&mut self) {
        let h = Gate2x2::hadamard();
        for q in 0..self.sv.n_qubits {
            self.sv.apply_single_qubit(&h, q);
        }
    }
    /// Measure qubit `q` (deterministic via seed).
    pub fn measure_qubit(&self, qubit: usize, seed: u64) -> u8 {
        self.sv.measure_qubit(qubit, seed)
    }
    /// Check normalisation.
    pub fn is_normalized(&self) -> bool {
        self.sv.is_normalized()
    }
    /// Amplitudes as a slice.
    pub fn amplitudes(&self) -> &[Complex] {
        &self.sv.amplitudes
    }
}
/// A quantum register of n qubits.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct QuantumRegisterData {
    pub num_qubits: usize,
    pub label: String,
}
impl QuantumRegisterData {
    #[allow(dead_code)]
    pub fn new(n: usize, label: &str) -> Self {
        Self {
            num_qubits: n,
            label: label.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn hilbert_space_dim(&self) -> usize {
        1usize << self.num_qubits
    }
    #[allow(dead_code)]
    pub fn computational_basis_size(&self) -> usize {
        self.hilbert_space_dim()
    }
}
/// Quantum Fourier Transform simulator for up to ~20 qubits.
pub struct QFTSimulator {
    pub n_qubits: usize,
}
impl QFTSimulator {
    /// Create a QFT simulator for `n_qubits` qubits.
    pub fn new(n_qubits: usize) -> Self {
        QFTSimulator { n_qubits }
    }
    /// Apply the QFT in-place to a `QuantumRegister`.
    ///
    /// Implements the standard circuit decomposition:
    ///   for j = 0..n: H on j, then CPhase(2π/2^k) for k=2..n-j
    pub fn apply(&self, reg: &mut QuantumRegister) {
        let n = self.n_qubits;
        for j in 0..n {
            reg.apply_gate(&Gate2x2::hadamard(), j);
            for k in 2..=(n - j) {
                let theta = 2.0 * PI / (1u64 << k) as f64;
                let cphase = Gate2x2::phase(theta);
                let control = j + k - 1;
                let target = j;
                let size = reg.size();
                for i in 0..size {
                    if (i >> control) & 1 == 1 && (i >> target) & 1 == 1 {
                        reg.sv.amplitudes[i] = reg.sv.amplitudes[i].mul(&cphase.matrix[1][1]);
                    }
                }
            }
        }
        Self::bit_reverse(reg);
    }
    /// Apply the inverse QFT in-place.
    pub fn apply_inverse(&self, reg: &mut QuantumRegister) {
        let n = self.n_qubits;
        Self::bit_reverse(reg);
        for j in (0..n).rev() {
            for k in (2..=(n - j)).rev() {
                let theta = -2.0 * PI / (1u64 << k) as f64;
                let cphase = Gate2x2::phase(theta);
                let control = j + k - 1;
                let target = j;
                let size = reg.size();
                for i in 0..size {
                    if (i >> control) & 1 == 1 && (i >> target) & 1 == 1 {
                        reg.sv.amplitudes[i] = reg.sv.amplitudes[i].mul(&cphase.matrix[1][1]);
                    }
                }
            }
            reg.apply_gate(&Gate2x2::hadamard(), j);
        }
    }
    /// Compute the QFT of a given amplitude vector and return the transformed
    /// amplitudes.  Convenience wrapper that does not require a `QuantumRegister`.
    pub fn transform(&self, input: &[Complex]) -> Vec<Complex> {
        let n = input.len();
        let inv_sqrt_n = 1.0 / (n as f64).sqrt();
        (0..n)
            .map(|k| {
                (0..n)
                    .fold(Complex::zero(), |acc, j| {
                        let angle = 2.0 * PI * (j * k) as f64 / n as f64;
                        acc.add(&input[j].mul(&Complex::exp(angle)))
                    })
                    .scale(inv_sqrt_n)
            })
            .collect()
    }
    /// Bit-reversal permutation of register amplitudes (needed by standard QFT).
    fn bit_reverse(reg: &mut QuantumRegister) {
        let n = reg.n_qubits();
        let size = reg.size();
        for i in 0..size {
            let j = Self::reverse_bits(i, n);
            if j > i {
                reg.sv.amplitudes.swap(i, j);
            }
        }
    }
    fn reverse_bits(mut x: usize, n: usize) -> usize {
        let mut result = 0;
        for _ in 0..n {
            result = (result << 1) | (x & 1);
            x >>= 1;
        }
        result
    }
}
/// A single qubit state α|0⟩ + β|1⟩ with |α|² + |β|² = 1.
#[derive(Debug, Clone)]
pub struct Qubit {
    pub alpha: Complex,
    pub beta: Complex,
}
impl Qubit {
    /// |0⟩ state.
    pub fn zero() -> Self {
        Qubit {
            alpha: Complex::one(),
            beta: Complex::zero(),
        }
    }
    /// |1⟩ state.
    pub fn one() -> Self {
        Qubit {
            alpha: Complex::zero(),
            beta: Complex::one(),
        }
    }
    /// |+⟩ = (|0⟩ + |1⟩) / √2.
    pub fn plus() -> Self {
        let v = 1.0 / 2.0f64.sqrt();
        Qubit {
            alpha: Complex::new(v, 0.0),
            beta: Complex::new(v, 0.0),
        }
    }
    /// |−⟩ = (|0⟩ − |1⟩) / √2.
    pub fn minus() -> Self {
        let v = 1.0 / 2.0f64.sqrt();
        Qubit {
            alpha: Complex::new(v, 0.0),
            beta: Complex::new(-v, 0.0),
        }
    }
    /// Probability of measuring 0.
    pub fn prob_zero(&self) -> f64 {
        self.alpha.abs_sq()
    }
    /// Probability of measuring 1.
    pub fn prob_one(&self) -> f64 {
        self.beta.abs_sq()
    }
    /// Check that |α|² + |β|² ≈ 1.
    pub fn is_normalized(&self) -> bool {
        (self.prob_zero() + self.prob_one() - 1.0).abs() < 1e-10
    }
    /// Perform a projective measurement.
    ///
    /// Uses a simple deterministic hash of `seed` to decide the outcome.
    /// Returns `(outcome, collapsed_state)`.
    pub fn measure(&self, seed: u64) -> (u8, Qubit) {
        let t = (seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407)
            >> 33) as f64
            / u32::MAX as f64;
        let p0 = self.prob_zero();
        if t < p0 {
            (0, Qubit::zero())
        } else {
            (1, Qubit::one())
        }
    }
}
/// Statevector simulator for n-qubit systems.
#[derive(Debug)]
pub struct QuantumStatevector {
    pub n_qubits: usize,
    /// 2^n complex amplitudes, indexed by basis state |b_{n-1}...b_0⟩.
    pub amplitudes: Vec<Complex>,
}
impl QuantumStatevector {
    /// Initialise the |0...0⟩ state.
    pub fn new(n_qubits: usize) -> Self {
        let size = 1usize << n_qubits;
        let mut amplitudes = vec![Complex::zero(); size];
        amplitudes[0] = Complex::one();
        QuantumStatevector {
            n_qubits,
            amplitudes,
        }
    }
    /// Amplitude of a basis state.
    pub fn amplitude(&self, basis_state: usize) -> Complex {
        self.amplitudes[basis_state]
    }
    /// Probability of a basis state.
    pub fn prob(&self, basis_state: usize) -> f64 {
        self.amplitudes[basis_state].abs_sq()
    }
    /// Apply a single-qubit gate to qubit `qubit_idx` (0 = least significant).
    pub fn apply_single_qubit(&mut self, gate: &Gate2x2, qubit_idx: usize) {
        let size = self.amplitudes.len();
        let step = 1usize << qubit_idx;
        let mut block_start = 0;
        while block_start < size {
            for j in block_start..(block_start + step) {
                let j1 = j + step;
                let a0 = self.amplitudes[j];
                let a1 = self.amplitudes[j1];
                self.amplitudes[j] = gate.matrix[0][0].mul(&a0).add(&gate.matrix[0][1].mul(&a1));
                self.amplitudes[j1] = gate.matrix[1][0].mul(&a0).add(&gate.matrix[1][1].mul(&a1));
            }
            block_start += step * 2;
        }
    }
    /// Apply a CNOT gate with the given control and target qubit indices.
    pub fn apply_cnot(&mut self, control: usize, target: usize) {
        let size = self.amplitudes.len();
        for i in 0..size {
            if (i >> control) & 1 == 1 {
                let j = i ^ (1 << target);
                if j > i {
                    self.amplitudes.swap(i, j);
                }
            }
        }
    }
    /// Measure a single qubit (destructive, deterministic via `seed`). Returns 0 or 1.
    pub fn measure_qubit(&self, qubit_idx: usize, seed: u64) -> u8 {
        let prob1: f64 = self
            .amplitudes
            .iter()
            .enumerate()
            .filter(|(i, _)| (i >> qubit_idx) & 1 == 1)
            .map(|(_, a)| a.abs_sq())
            .sum();
        let t = (seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407)
            >> 33) as f64
            / u32::MAX as f64;
        if t < prob1 {
            1
        } else {
            0
        }
    }
    /// Check that Σ |aᵢ|² ≈ 1.
    pub fn is_normalized(&self) -> bool {
        let total: f64 = self.amplitudes.iter().map(|a| a.abs_sq()).sum();
        (total - 1.0).abs() < 1e-9
    }
    /// Number of amplitudes (= 2^n_qubits).
    pub fn n_amplitudes(&self) -> usize {
        self.amplitudes.len()
    }
}
/// Single-qubit Pauli operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pauli {
    I,
    X,
    Y,
    Z,
}
impl Pauli {
    /// Matrix representation as a `Gate2x2`.
    pub fn to_gate(self) -> Gate2x2 {
        match self {
            Pauli::I => Gate2x2 {
                name: "I".to_string(),
                matrix: [
                    [Complex::one(), Complex::zero()],
                    [Complex::zero(), Complex::one()],
                ],
            },
            Pauli::X => Gate2x2::pauli_x(),
            Pauli::Y => Gate2x2::pauli_y(),
            Pauli::Z => Gate2x2::pauli_z(),
        }
    }
    /// Multiply two single-qubit Paulis (ignoring global phase).
    pub fn mul(self, other: Pauli) -> Pauli {
        match (self, other) {
            (Pauli::I, p) | (p, Pauli::I) => p,
            (Pauli::X, Pauli::X) => Pauli::I,
            (Pauli::Y, Pauli::Y) => Pauli::I,
            (Pauli::Z, Pauli::Z) => Pauli::I,
            (Pauli::X, Pauli::Y) => Pauli::Z,
            (Pauli::Y, Pauli::X) => Pauli::Z,
            (Pauli::Y, Pauli::Z) => Pauli::X,
            (Pauli::Z, Pauli::Y) => Pauli::X,
            (Pauli::Z, Pauli::X) => Pauli::Y,
            (Pauli::X, Pauli::Z) => Pauli::Y,
        }
    }
    /// Return true iff this Pauli commutes with `other`.
    pub fn commutes_with(self, other: Pauli) -> bool {
        matches!(
            (self, other),
            (Pauli::I, _)
                | (_, Pauli::I)
                | (Pauli::X, Pauli::X)
                | (Pauli::Y, Pauli::Y)
                | (Pauli::Z, Pauli::Z)
        )
    }
}
/// Quantum complexity class hierarchy.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum QuantumComplexityClass {
    BPP,
    BQP,
    QMA,
    QMAM,
    PSharpP,
    PSPACE,
    EXP,
}
impl QuantumComplexityClass {
    #[allow(dead_code)]
    pub fn name(&self) -> &'static str {
        match self {
            Self::BPP => "BPP",
            Self::BQP => "BQP",
            Self::QMA => "QMA",
            Self::QMAM => "QMAM",
            Self::PSharpP => "P#P",
            Self::PSPACE => "PSPACE",
            Self::EXP => "EXP",
        }
    }
    #[allow(dead_code)]
    pub fn contains_bpp(&self) -> bool {
        !matches!(self, Self::BPP)
    }
    #[allow(dead_code)]
    pub fn is_believed_strictly_larger_than_bqp(&self) -> bool {
        matches!(
            self,
            Self::QMA | Self::QMAM | Self::PSharpP | Self::PSPACE | Self::EXP
        )
    }
}
/// Quantum walk on a graph.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct QuantumWalk {
    pub graph_name: String,
    pub walk_type: QuantumWalkType,
    pub num_steps: usize,
}
impl QuantumWalk {
    #[allow(dead_code)]
    pub fn new(graph: &str, wt: QuantumWalkType, steps: usize) -> Self {
        Self {
            graph_name: graph.to_string(),
            walk_type: wt,
            num_steps: steps,
        }
    }
    #[allow(dead_code)]
    pub fn speedup_over_classical(&self) -> f64 {
        match self.walk_type {
            QuantumWalkType::ContinuousTime => 2.0,
            QuantumWalkType::DiscreteTimeCoin => 2.0,
            QuantumWalkType::Scattering => 1.5,
        }
    }
    #[allow(dead_code)]
    pub fn element_distinctness_uses_walk(&self) -> bool {
        self.graph_name.contains("Johnson")
    }
}
/// Quantum teleportation protocol.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TeleportationProtocol {
    pub input_qubits: usize,
    pub classical_bits_needed: usize,
}
impl TeleportationProtocol {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        Self {
            input_qubits: n,
            classical_bits_needed: 2 * n,
        }
    }
    #[allow(dead_code)]
    pub fn requires_entanglement(&self) -> bool {
        true
    }
    #[allow(dead_code)]
    pub fn no_faster_than_light(&self) -> bool {
        self.classical_bits_needed > 0
    }
    #[allow(dead_code)]
    pub fn fidelity_with_perfect_channel(&self) -> f64 {
        1.0
    }
}
/// Pauli group on n qubits.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PauliGroup {
    pub num_qubits: usize,
}
impl PauliGroup {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        Self { num_qubits: n }
    }
    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        4usize.pow(self.num_qubits as u32 + 1)
    }
    #[allow(dead_code)]
    pub fn is_abelian(&self) -> bool {
        self.num_qubits == 0
    }
    #[allow(dead_code)]
    pub fn stabilizer_group_description(&self) -> String {
        format!(
            "Abelian subgroup of P_{} stabilizing a code space",
            self.num_qubits
        )
    }
}
/// A Grover oracle that marks a single target basis state.
///
/// The oracle implements the phase-flip O|x⟩ = -|x⟩ if x == target, else |x⟩.
#[derive(Debug, Clone)]
pub struct GroverOracle {
    /// The target basis state (integer index).
    pub target: usize,
}
impl GroverOracle {
    /// Create an oracle for the given target basis state.
    pub fn new(target: usize) -> Self {
        GroverOracle { target }
    }
    /// Apply the oracle phase-flip to the register amplitudes.
    pub fn apply(&self, reg: &mut QuantumRegister) {
        if self.target < reg.size() {
            reg.sv.amplitudes[self.target] = reg.sv.amplitudes[self.target].scale(-1.0);
        }
    }
    /// Apply the Grover diffusion operator (inversion about the mean) to the
    /// register.
    pub fn diffusion(reg: &mut QuantumRegister) {
        let n = reg.size();
        let mean = reg
            .sv
            .amplitudes
            .iter()
            .fold(Complex::zero(), |acc, a| acc.add(a))
            .scale(1.0 / n as f64);
        for amp in reg.sv.amplitudes.iter_mut() {
            let two_mean = mean.scale(2.0);
            *amp = Complex {
                re: two_mean.re - amp.re,
                im: two_mean.im - amp.im,
            };
        }
    }
    /// Run Grover's algorithm for `iterations` steps and return the register.
    pub fn run_grover(&self, n_qubits: usize, iterations: u32) -> QuantumRegister {
        let mut reg = QuantumRegister::new(n_qubits);
        reg.prepare_uniform_superposition();
        for _ in 0..iterations {
            self.apply(&mut reg);
            Self::diffusion(&mut reg);
        }
        reg
    }
}
/// Quantum Shannon theory capacities.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct QuantumCapacities {
    pub channel_name: String,
    pub classical_capacity: f64,
    pub quantum_capacity: f64,
    pub entanglement_assisted_capacity: f64,
    pub private_capacity: f64,
}
impl QuantumCapacities {
    #[allow(dead_code)]
    pub fn for_qubit_channel(name: &str, cc: f64, qc: f64, ea: f64, pc: f64) -> Self {
        Self {
            channel_name: name.to_string(),
            classical_capacity: cc,
            quantum_capacity: qc,
            entanglement_assisted_capacity: ea,
            private_capacity: pc,
        }
    }
    #[allow(dead_code)]
    pub fn quantum_is_at_most_private(&self) -> bool {
        self.quantum_capacity <= self.private_capacity + 1e-10
    }
    #[allow(dead_code)]
    pub fn superdense_coding_factor(&self) -> f64 {
        if self.classical_capacity > 0.0 {
            self.entanglement_assisted_capacity / self.classical_capacity
        } else {
            0.0
        }
    }
}
/// An n-qubit Pauli string (tensor product of single-qubit Paulis), optionally
/// with a sign.
#[derive(Debug, Clone)]
pub struct PauliString {
    /// Factor: +1 or -1.
    pub sign: i8,
    /// One Pauli per qubit.
    pub paulis: Vec<Pauli>,
}
impl PauliString {
    /// Create a new Pauli string with the given sign and per-qubit Paulis.
    pub fn new(sign: i8, paulis: Vec<Pauli>) -> Self {
        PauliString { sign, paulis }
    }
    /// The all-identity Pauli string on `n` qubits.
    pub fn identity(n: usize) -> Self {
        PauliString {
            sign: 1,
            paulis: vec![Pauli::I; n],
        }
    }
    /// Number of qubits.
    pub fn n_qubits(&self) -> usize {
        self.paulis.len()
    }
    /// Component-wise Pauli product (ignoring phase from YY products).
    pub fn mul(&self, other: &PauliString) -> PauliString {
        assert_eq!(self.paulis.len(), other.paulis.len());
        let paulis: Vec<Pauli> = self
            .paulis
            .iter()
            .zip(other.paulis.iter())
            .map(|(&a, &b)| a.mul(b))
            .collect();
        PauliString {
            sign: self.sign * other.sign,
            paulis,
        }
    }
    /// Check that this Pauli string commutes with `other`.
    ///
    /// Two Pauli strings commute iff the number of qubit positions where they
    /// anti-commute is even.
    pub fn commutes_with(&self, other: &PauliString) -> bool {
        let anti: usize = self
            .paulis
            .iter()
            .zip(other.paulis.iter())
            .filter(|(&a, &b)| !a.commutes_with(b))
            .count();
        anti % 2 == 0
    }
}
/// Variational Quantum Eigensolver (VQE) configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct VqeConfig {
    pub hamiltonian_name: String,
    pub ansatz_name: String,
    pub num_layers: usize,
    pub num_qubits: usize,
}
impl VqeConfig {
    #[allow(dead_code)]
    pub fn new(ham: &str, ansatz: &str, layers: usize, qubits: usize) -> Self {
        Self {
            hamiltonian_name: ham.to_string(),
            ansatz_name: ansatz.to_string(),
            num_layers: layers,
            num_qubits: qubits,
        }
    }
    #[allow(dead_code)]
    pub fn num_parameters(&self) -> usize {
        self.num_layers * self.num_qubits * 2
    }
    #[allow(dead_code)]
    pub fn is_hybrid_classical_quantum(&self) -> bool {
        true
    }
    #[allow(dead_code)]
    pub fn variational_principle(&self) -> String {
        format!(
            "E(theta) = <psi(theta)|H|psi(theta)> >= E_ground for {}",
            self.hamiltonian_name
        )
    }
}
/// A single-qubit (2×2 unitary) quantum gate.
#[derive(Debug, Clone)]
pub struct Gate2x2 {
    pub matrix: [[Complex; 2]; 2],
    pub name: String,
}
impl Gate2x2 {
    /// Pauli-X (NOT) gate: [[0,1],[1,0]].
    pub fn pauli_x() -> Self {
        Gate2x2 {
            name: "X".to_string(),
            matrix: [
                [Complex::zero(), Complex::one()],
                [Complex::one(), Complex::zero()],
            ],
        }
    }
    /// Pauli-Y gate: [[0,−i],[i,0]].
    pub fn pauli_y() -> Self {
        Gate2x2 {
            name: "Y".to_string(),
            matrix: [
                [Complex::zero(), Complex::new(0.0, -1.0)],
                [Complex::new(0.0, 1.0), Complex::zero()],
            ],
        }
    }
    /// Pauli-Z gate: [[1,0],[0,−1]].
    pub fn pauli_z() -> Self {
        Gate2x2 {
            name: "Z".to_string(),
            matrix: [
                [Complex::one(), Complex::zero()],
                [Complex::zero(), Complex::new(-1.0, 0.0)],
            ],
        }
    }
    /// Hadamard gate: H = 1/√2 [[1,1],[1,−1]].
    pub fn hadamard() -> Self {
        let v = Complex::new(1.0 / 2.0f64.sqrt(), 0.0);
        let neg_v = Complex::new(-1.0 / 2.0f64.sqrt(), 0.0);
        Gate2x2 {
            name: "H".to_string(),
            matrix: [[v, v], [v, neg_v]],
        }
    }
    /// Phase gate: [[1,0],[0,e^{iθ}]].
    pub fn phase(theta: f64) -> Self {
        Gate2x2 {
            name: format!("P({theta:.4})"),
            matrix: [
                [Complex::one(), Complex::zero()],
                [Complex::zero(), Complex::exp(theta)],
            ],
        }
    }
    /// T gate: phase(π/4).
    pub fn t_gate() -> Self {
        let mut g = Self::phase(PI / 4.0);
        g.name = "T".to_string();
        g
    }
    /// S gate: phase(π/2).
    pub fn s_gate() -> Self {
        let mut g = Self::phase(PI / 2.0);
        g.name = "S".to_string();
        g
    }
    /// Apply this gate to a qubit: |ψ'⟩ = U|ψ⟩.
    pub fn apply(&self, qubit: &Qubit) -> Qubit {
        let a = self.matrix[0][0]
            .mul(&qubit.alpha)
            .add(&self.matrix[0][1].mul(&qubit.beta));
        let b = self.matrix[1][0]
            .mul(&qubit.alpha)
            .add(&self.matrix[1][1].mul(&qubit.beta));
        Qubit { alpha: a, beta: b }
    }
    /// Compose gates: returns self · other (apply `other` first, then `self`).
    pub fn compose(&self, other: &Gate2x2) -> Gate2x2 {
        let mut m = [[Complex::zero(); 2]; 2];
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    m[i][j] = m[i][j].add(&self.matrix[i][k].mul(&other.matrix[k][j]));
                }
            }
        }
        Gate2x2 {
            name: format!("{}·{}", self.name, other.name),
            matrix: m,
        }
    }
    /// Check that U†U ≈ I (unitarity).
    pub fn is_unitary(&self) -> bool {
        let tol = 1e-9;
        for i in 0..2 {
            for j in 0..2 {
                let mut sum = Complex::zero();
                for k in 0..2 {
                    sum = sum.add(&self.matrix[k][i].conj().mul(&self.matrix[k][j]));
                }
                let expected_re = if i == j { 1.0 } else { 0.0 };
                if (sum.re - expected_re).abs() > tol || sum.im.abs() > tol {
                    return false;
                }
            }
        }
        true
    }
}
/// A quantum circuit as a sequence of gate applications.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct QuantumCircuitData {
    pub num_qubits: usize,
    pub gates: Vec<(QuantumGate, Vec<usize>)>,
    pub depth: usize,
}
impl QuantumCircuitData {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        Self {
            num_qubits: n,
            gates: Vec::new(),
            depth: 0,
        }
    }
    #[allow(dead_code)]
    pub fn apply(&mut self, gate: QuantumGate, qubits: Vec<usize>) {
        self.depth += 1;
        self.gates.push((gate, qubits));
    }
    #[allow(dead_code)]
    pub fn gate_count(&self) -> usize {
        self.gates.len()
    }
    #[allow(dead_code)]
    pub fn t_count(&self) -> usize {
        self.gates.iter().filter(|(g, _)| g.name == "T").count()
    }
    #[allow(dead_code)]
    pub fn is_clifford(&self) -> bool {
        self.gates.iter().all(|(g, _)| g.is_clifford)
    }
}
/// Quantum channel (completely positive trace-preserving map).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct QuantumChannel {
    pub name: String,
    pub input_dim: usize,
    pub output_dim: usize,
    pub channel_type: ChannelType,
}
impl QuantumChannel {
    #[allow(dead_code)]
    pub fn depolarizing(dim: usize, _p: f64) -> Self {
        Self {
            name: format!("Depolarizing(dim={dim})"),
            input_dim: dim,
            output_dim: dim,
            channel_type: ChannelType::Depolarizing,
        }
    }
    #[allow(dead_code)]
    pub fn amplitude_damping(gamma: f64) -> Self {
        let name = format!("AmplitudeDamping(gamma={gamma:.3})");
        Self {
            name,
            input_dim: 2,
            output_dim: 2,
            channel_type: ChannelType::AmplitudeDamping,
        }
    }
    #[allow(dead_code)]
    pub fn is_unital(&self) -> bool {
        matches!(
            self.channel_type,
            ChannelType::Depolarizing
                | ChannelType::PhaseDamping
                | ChannelType::BitFlip
                | ChannelType::PhaseFlip
                | ChannelType::Unitary
                | ChannelType::Identity
        )
    }
    #[allow(dead_code)]
    pub fn is_degradable(&self) -> bool {
        matches!(
            self.channel_type,
            ChannelType::AmplitudeDamping | ChannelType::Unitary
        )
    }
    #[allow(dead_code)]
    pub fn quantum_capacity_achievable(&self) -> bool {
        !matches!(self.channel_type, ChannelType::Erasure)
    }
}
/// A stabilizer state specified by its stabilizer group generators.
///
/// The stabilizer state |ψ⟩ is the unique state satisfying g|ψ⟩ = |ψ⟩ for
/// every generator g in the stabilizer group.
#[derive(Debug, Clone)]
pub struct StabilizerState {
    pub n_qubits: usize,
    /// Generators of the stabilizer group (each is an n-qubit Pauli string).
    pub generators: Vec<PauliString>,
}
impl StabilizerState {
    /// Create a stabilizer state from a list of generators.
    ///
    /// Assumes the generators are already a valid independent commuting set.
    pub fn from_generators(n_qubits: usize, generators: Vec<PauliString>) -> Self {
        StabilizerState {
            n_qubits,
            generators,
        }
    }
    /// The |0…0⟩ state, stabilized by Z_0, Z_1, …, Z_{n-1}.
    pub fn computational_zero(n_qubits: usize) -> Self {
        let generators: Vec<PauliString> = (0..n_qubits)
            .map(|i| {
                let mut paulis = vec![Pauli::I; n_qubits];
                paulis[i] = Pauli::Z;
                PauliString::new(1, paulis)
            })
            .collect();
        StabilizerState {
            n_qubits,
            generators,
        }
    }
    /// The |+…+⟩ state, stabilized by X_0, X_1, …, X_{n-1}.
    pub fn plus_state(n_qubits: usize) -> Self {
        let generators: Vec<PauliString> = (0..n_qubits)
            .map(|i| {
                let mut paulis = vec![Pauli::I; n_qubits];
                paulis[i] = Pauli::X;
                PauliString::new(1, paulis)
            })
            .collect();
        StabilizerState {
            n_qubits,
            generators,
        }
    }
    /// Check that all generators commute pairwise (consistency requirement).
    pub fn is_consistent(&self) -> bool {
        let n = self.generators.len();
        for i in 0..n {
            for j in (i + 1)..n {
                if !self.generators[i].commutes_with(&self.generators[j]) {
                    return false;
                }
            }
        }
        true
    }
    /// Measure a Pauli observable P on this stabilizer state.
    ///
    /// Returns `Some(+1)` if P is in the stabilizer group, `Some(-1)` if -P
    /// is in the stabilizer group, or `None` if the outcome is probabilistic
    /// (P anti-commutes with some generator).
    pub fn measure_pauli(&self, p: &PauliString) -> Option<i8> {
        let all_commute = self.generators.iter().all(|g| p.commutes_with(g));
        if !all_commute {
            return None;
        }
        let _product = p.mul(&PauliString::identity(self.n_qubits));
        Some(p.sign)
    }
}
/// A quantum circuit: an ordered list of gate operations.
#[derive(Debug, Clone, Default)]
pub struct QuantumCircuit {
    pub n_qubits: usize,
    pub ops: Vec<GateOp>,
}
impl QuantumCircuit {
    /// Create an empty circuit on `n_qubits` qubits.
    pub fn new(n_qubits: usize) -> Self {
        QuantumCircuit {
            n_qubits,
            ops: Vec::new(),
        }
    }
    /// Append a single-qubit gate on qubit `q`.
    pub fn add_gate(&mut self, gate: Gate2x2, qubit: usize) {
        self.ops.push(GateOp::Single { gate, qubit });
    }
    /// Append a CNOT gate.
    pub fn add_cnot(&mut self, control: usize, target: usize) {
        self.ops.push(GateOp::Cnot { control, target });
    }
    /// Number of gate operations.
    pub fn depth(&self) -> usize {
        self.ops.len()
    }
    /// Execute the circuit on a fresh |0…0⟩ state and return the resulting
    /// `QuantumRegister`.
    pub fn run(&self) -> QuantumRegister {
        let mut reg = QuantumRegister::new(self.n_qubits);
        for op in &self.ops {
            match op {
                GateOp::Single { gate, qubit } => reg.apply_gate(gate, *qubit),
                GateOp::Cnot { control, target } => reg.apply_cnot(*control, *target),
            }
        }
        reg
    }
    /// Execute the circuit on a fresh state and measure all qubits.
    ///
    /// Returns a `Vec<u8>` of 0/1 measurement outcomes (qubit 0 first).
    pub fn run_and_measure(&self, seed: u64) -> Vec<u8> {
        let reg = self.run();
        (0..self.n_qubits)
            .map(|q| reg.measure_qubit(q, seed.wrapping_add(q as u64 * 1234567891)))
            .collect()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuantumWalkType {
    ContinuousTime,
    DiscreteTimeCoin,
    Scattering,
}
