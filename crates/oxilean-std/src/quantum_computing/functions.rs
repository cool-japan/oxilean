//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};
use std::f64::consts::PI;

use super::types::{
    CliffordGroup, Complex, Gate2x2, GroverOracle, Pauli, PauliString, QFTSimulator,
    QuantumChannel, QuantumCircuit, QuantumCircuitData, QuantumErrorCode, QuantumGate,
    QuantumRegister, QuantumRegisterData, QuantumStatevector, Qubit, StabilizerState,
};

pub fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
pub fn app2(f: Expr, a: Expr, b: Expr) -> Expr {
    app(app(f, a), b)
}
pub fn app3(f: Expr, a: Expr, b: Expr, c: Expr) -> Expr {
    app(app2(f, a, b), c)
}
pub fn cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}
pub fn prop() -> Expr {
    Expr::Sort(Level::zero())
}
pub fn type0() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
pub fn pi(bi: BinderInfo, name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(bi, Name::str(name), Box::new(dom), Box::new(body))
}
pub fn arrow(a: Expr, b: Expr) -> Expr {
    pi(BinderInfo::Default, "_", a, b)
}
pub fn nat_ty() -> Expr {
    cst("Nat")
}
pub fn real_ty() -> Expr {
    cst("Real")
}
pub fn bool_ty() -> Expr {
    cst("Bool")
}
/// `Qubit : Type` — qubit state α|0⟩ + β|1⟩ in ℂ².
pub fn qubit_ty() -> Expr {
    type0()
}
/// `QuantumGate : Nat → Type` — unitary operator on n qubits.
pub fn quantum_gate_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `QuantumCircuit : Type` — sequence of quantum gates applied to a register.
pub fn quantum_circuit_ty() -> Expr {
    type0()
}
/// `Measurement : Type` — quantum measurement (observable).
pub fn measurement_ty() -> Expr {
    type0()
}
/// `Entanglement : Type` — entangled quantum state (e.g., Bell state).
pub fn entanglement_ty() -> Expr {
    type0()
}
/// No-Cloning Theorem: it is impossible to create an identical copy of an
/// arbitrary unknown quantum state.
///
/// `no_cloning : Prop`
pub fn no_cloning_ty() -> Expr {
    prop()
}
/// No-Deleting Theorem: it is impossible to delete a copy of an arbitrary
/// unknown quantum state given two identical copies.
///
/// `no_deleting : Prop`
pub fn no_deleting_ty() -> Expr {
    prop()
}
/// Quantum Teleportation Theorem: the quantum teleportation protocol
/// faithfully transmits an arbitrary unknown qubit state using one shared
/// Bell pair and two classical bits.
///
/// `quantum_teleportation : Prop`
pub fn quantum_teleportation_ty() -> Expr {
    prop()
}
/// Grover's Quadratic Speedup: Grover's algorithm solves unstructured search
/// on N items in O(√N) queries, a quadratic speedup over classical O(N).
///
/// `grover_speedup : ∀ n : Nat, GroverComplexity n ≤ Real.sqrt (Nat.pow 2 n)`
pub fn grover_speedup_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        app2(
            cst("Real.le"),
            app(cst("GroverComplexity"), cst("n")),
            app(
                cst("Real.sqrt"),
                app2(cst("Nat.pow"), cst("Nat.two"), cst("n")),
            ),
        ),
    )
}
/// Shor's Exponential Speedup: Shor's algorithm factors an n-bit integer in
/// polynomial time O(n³), an exponential speedup over the best classical algorithm.
///
/// `shor_exponential : Prop`
pub fn shor_exponential_ty() -> Expr {
    prop()
}
/// `GateSet : Type` — a finite set of quantum gates.
pub fn gate_set_ty() -> Expr {
    type0()
}
/// `UniversalGateSet : GateSet → Prop`
/// A gate set G is universal if any unitary can be approximated arbitrarily
/// well by a circuit from G.
pub fn universal_gate_set_ty() -> Expr {
    arrow(cst("GateSet"), prop())
}
/// `SolovayKitaev : ∀ (g : GateSet), UniversalGateSet g → Prop`
/// The Solovay-Kitaev theorem: any universal gate set can approximate any
/// single-qubit unitary to precision ε in O(log^c(1/ε)) gates.
pub fn solovay_kitaev_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "g",
        cst("GateSet"),
        arrow(app(cst("UniversalGateSet"), cst("g")), prop()),
    )
}
/// `HTCliffordUniversal : Prop`
/// The gate set {H, T, CNOT} is universal for quantum computation.
pub fn ht_clifford_universal_ty() -> Expr {
    prop()
}
/// `QFTState : Nat → Type`
/// The state produced by the quantum Fourier transform on n qubits.
pub fn qft_state_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `QFTCorrectness : ∀ n : Nat, Prop`
/// The quantum Fourier transform on n qubits correctly computes the DFT
/// of the amplitude vector.
pub fn qft_correctness_ty() -> Expr {
    pi(BinderInfo::Default, "n", nat_ty(), prop())
}
/// `QFTComplexity : ∀ n : Nat, QFTGateCount n ≤ Nat.pow n 2`
/// The QFT on n qubits requires O(n²) gates.
pub fn qft_complexity_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        app2(
            cst("Nat.le"),
            app(cst("QFTGateCount"), cst("n")),
            app2(cst("Nat.pow"), cst("n"), cst("Nat.two")),
        ),
    )
}
/// `PeriodFinding : ∀ (N a : Nat), Prop`
/// Shor's period-finding subroutine: for coprime a, N, the quantum period
/// finding algorithm finds the order r of a mod N in polynomial time.
pub fn period_finding_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "N",
        nat_ty(),
        pi(BinderInfo::Default, "a", nat_ty(), prop()),
    )
}
/// `ShorFactoring : ∀ N : Nat, Prop`
/// Given access to the period-finding subroutine, Shor's algorithm factors N
/// in O(log³ N) quantum gate operations with high probability.
pub fn shor_factoring_ty() -> Expr {
    pi(BinderInfo::Default, "N", nat_ty(), prop())
}
/// `AmplitudeAmplification : ∀ n : Nat, ∀ k : Nat, Prop`
/// General amplitude amplification: if a state has success probability p,
/// then after O(1/√p) oracle calls the success probability is amplified to
/// nearly 1.
pub fn amplitude_amplification_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        pi(BinderInfo::Default, "k", nat_ty(), prop()),
    )
}
/// `GroverOptimality : Prop`
/// Grover's algorithm is optimal: any quantum algorithm requires Ω(√N)
/// oracle queries for unstructured search.
pub fn grover_optimality_ty() -> Expr {
    prop()
}
/// `PhaseEstimation : ∀ (n : Nat), Prop`
/// Quantum phase estimation approximates the eigenvalue e^{2πiφ} of a
/// unitary U to n bits of precision using n ancilla qubits and O(2^n)
/// controlled-U applications.
pub fn phase_estimation_ty() -> Expr {
    pi(BinderInfo::Default, "n", nat_ty(), prop())
}
/// `PhaseEstimationPrecision : ∀ n : Nat, PhaseError n ≤ Real.pow 2 n`
/// The phase estimation error is bounded by 2^{-n}.
pub fn phase_estimation_precision_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        app2(
            cst("Real.le"),
            app(cst("PhaseError"), cst("n")),
            app2(cst("Real.pow"), cst("Real.two"), cst("n")),
        ),
    )
}
/// `VQEState : Type`
/// The parameterized ansatz state used in the variational quantum eigensolver.
pub fn vqe_state_ty() -> Expr {
    type0()
}
/// `VQEVariationalPrinciple : Prop`
/// The variational principle underlying VQE: the expectation value of any
/// Hamiltonian H with any normalized state |ψ⟩ satisfies ⟨ψ|H|ψ⟩ ≥ E_0,
/// where E_0 is the ground-state energy.
pub fn vqe_variational_principle_ty() -> Expr {
    prop()
}
/// `QAOAState : Nat → Type`
/// The variational state produced by the Quantum Approximate Optimization
/// Algorithm (QAOA) at depth p.
pub fn qaoa_state_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `QAOAApproximation : ∀ p : Nat, Prop`
/// As the QAOA depth p → ∞, the algorithm converges to the exact solution
/// of the combinatorial optimization problem.
pub fn qaoa_approximation_ty() -> Expr {
    pi(BinderInfo::Default, "p", nat_ty(), prop())
}
/// `StabilizerCode : Nat → Nat → Type`
/// An [\[n, k\]] stabilizer code encodes k logical qubits into n physical qubits.
pub fn stabilizer_code_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `StabilizerCodeCorrectsErrors : ∀ (n k d : Nat), Prop`
/// An [\[n, k, d\]] stabilizer code can correct ⌊(d−1)/2⌋ arbitrary qubit errors.
pub fn stabilizer_code_corrects_errors_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "k",
            nat_ty(),
            pi(BinderInfo::Default, "d", nat_ty(), prop()),
        ),
    )
}
/// `QuantumHammingBound : ∀ (n k t : Nat), Prop`
/// The quantum Hamming (Singleton) bound: n − k ≥ 4t for a code correcting
/// t errors.
pub fn quantum_hamming_bound_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "k",
            nat_ty(),
            pi(BinderInfo::Default, "t", nat_ty(), prop()),
        ),
    )
}
/// `SurfaceCode : Nat → Type`
/// A surface code of linear size d (code distance d, n ≈ 2d² physical qubits,
/// 1 logical qubit).
pub fn surface_code_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `SurfaceCodeDistance : ∀ d : Nat, Prop`
/// The surface code with parameter d has code distance d, i.e., the minimum
/// weight of any undetectable logical error is d.
pub fn surface_code_distance_ty() -> Expr {
    pi(BinderInfo::Default, "d", nat_ty(), prop())
}
/// `FaultTolerantThreshold : Prop`
/// The threshold theorem: if the physical error rate p is below a constant
/// threshold p_th ≈ 10⁻², arbitrarily long quantum computations can be
/// performed with exponentially suppressed logical error rates.
pub fn fault_tolerant_threshold_ty() -> Expr {
    prop()
}
/// `DenseCodingProtocol : Prop`
/// Super-dense coding: using one shared Bell pair, Alice can send 2 classical
/// bits to Bob using only a single qubit transmission.
pub fn dense_coding_protocol_ty() -> Expr {
    prop()
}
/// `BB84Security : Prop`
/// Information-theoretic security of the BB84 protocol: any eavesdropper
/// introduces detectable disturbance, and the secret key rate is positive
/// below the QBER threshold ≈ 11%.
pub fn bb84_security_ty() -> Expr {
    prop()
}
/// `BB84KeyRate : ∀ e : Real, Prop`
/// The BB84 asymptotic key rate r(e) = 1 − 2H(e), where H is the binary
/// entropy function and e is the quantum bit-error rate.
pub fn bb84_key_rate_ty() -> Expr {
    pi(BinderInfo::Default, "e", real_ty(), prop())
}
/// `BQPContainsBPP : Prop`
/// The complexity class BQP (bounded-error quantum polynomial time) contains
/// BPP (bounded-error probabilistic polynomial time): BPP ⊆ BQP.
pub fn bqp_contains_bpp_ty() -> Expr {
    prop()
}
/// `BQPInPSHARPP : Prop`
/// BQP is contained in P^#P (and hence in PSPACE): BQP ⊆ P^#P.
pub fn bqp_in_psharp_p_ty() -> Expr {
    prop()
}
/// `QMADefinition : Prop`
/// QMA (Quantum Merlin-Arthur) is the quantum analogue of NP: a problem is
/// in QMA if there exists a polynomial-time quantum verifier and a quantum
/// witness state.
pub fn qma_definition_ty() -> Expr {
    prop()
}
/// `LocalHamiltonianQMAComplete : Prop`
/// The k-local Hamiltonian problem is QMA-complete for k ≥ 2.
pub fn local_hamiltonian_qma_complete_ty() -> Expr {
    prop()
}
/// `HamiltonianSimulation : ∀ (t : Real), Prop`
/// For any local Hamiltonian H, the time-evolution operator e^{−iHt} can be
/// approximated to precision ε in poly(n, t, 1/ε) gate complexity.
pub fn hamiltonian_simulation_ty() -> Expr {
    pi(BinderInfo::Default, "t", real_ty(), prop())
}
/// `TrotterSuzukiError : ∀ (t : Real) (r : Nat), Prop`
/// The first-order Trotter-Suzuki product formula has error O(t²/r) for r
/// time steps.
pub fn trotter_suzuki_error_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "t",
        real_ty(),
        pi(BinderInfo::Default, "r", nat_ty(), prop()),
    )
}
/// `AdiabaticTheorem : ∀ (gap : Real), Prop`
/// The adiabatic theorem: if a Hamiltonian is evolved slowly enough (relative
/// to the inverse square of the spectral gap), the system remains in its
/// instantaneous ground state.
pub fn adiabatic_theorem_ty() -> Expr {
    pi(BinderInfo::Default, "gap", real_ty(), prop())
}
/// `AdiabaticQCEquivalence : Prop`
/// Adiabatic quantum computation is polynomially equivalent to the standard
/// gate model of quantum computation.
pub fn adiabatic_qc_equivalence_ty() -> Expr {
    prop()
}
/// `Anyon : Type`
/// An anyon: a quasi-particle in (2+1)D that obeys fractional (non-Abelian)
/// statistics under exchange.
pub fn anyon_ty() -> Expr {
    type0()
}
/// `BraidingOperator : Anyon → Anyon → Type`
/// The unitary braiding operator R_{ij} arising from exchanging two anyons.
pub fn braiding_operator_ty() -> Expr {
    arrow(cst("Anyon"), arrow(cst("Anyon"), type0()))
}
/// `NonAbelianStatistics : Prop`
/// Non-Abelian anyons: the braiding operators do not commute in general,
/// enabling topologically protected quantum gates.
pub fn non_abelian_statistics_ty() -> Expr {
    prop()
}
/// `TopologicalProtection : Prop`
/// Topological quantum computation is inherently fault-tolerant: logical
/// operations correspond to global topological properties immune to local
/// perturbations.
pub fn topological_protection_ty() -> Expr {
    prop()
}
/// `FibonacciAnyon : Prop`
/// Fibonacci anyons are universal for topological quantum computation: any
/// unitary can be approximated by braiding Fibonacci anyons.
pub fn fibonacci_anyon_ty() -> Expr {
    prop()
}
/// `BosonSampling : Nat → Type`
/// The boson sampling problem on n photons: sample from the output
/// distribution of a linear optical network acting on Fock states.
pub fn boson_sampling_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `BosonSamplingHardness : Prop`
/// Boson sampling is classically hard to simulate (under plausible complexity
/// theoretic conjectures): efficient classical simulation would imply
/// P^#P = BPP^NP.
pub fn boson_sampling_hardness_ty() -> Expr {
    prop()
}
/// `QuantumVolume : Nat → Nat`
/// The quantum volume V_Q = 2^n where n is the largest square quantum circuit
/// that a device can implement with at-least-2/3 probability of success.
pub fn quantum_volume_ty() -> Expr {
    arrow(nat_ty(), nat_ty())
}
/// `QuantumVolumeMonotone : ∀ (n m : Nat), Prop`
/// Quantum volume is monotone: improvements in gate fidelity and connectivity
/// cannot decrease the quantum volume.
pub fn quantum_volume_monotone_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        pi(BinderInfo::Default, "m", nat_ty(), prop()),
    )
}
pub fn build_quantum_computing_env(
    env: &mut Environment,
) -> Result<(), Box<dyn std::error::Error>> {
    let axioms: &[(&str, Expr)] = &[
        ("Qubit", qubit_ty()),
        ("QuantumGate", quantum_gate_ty()),
        ("QuantumCircuit", quantum_circuit_ty()),
        ("Measurement", measurement_ty()),
        ("Entanglement", entanglement_ty()),
        ("GroverComplexity", arrow(nat_ty(), real_ty())),
        ("Nat.two", nat_ty()),
        ("Nat.pow", arrow(nat_ty(), arrow(nat_ty(), nat_ty()))),
        ("no_cloning", no_cloning_ty()),
        ("no_deleting", no_deleting_ty()),
        ("quantum_teleportation", quantum_teleportation_ty()),
        ("grover_speedup", grover_speedup_ty()),
        ("shor_exponential", shor_exponential_ty()),
        ("GateSet", gate_set_ty()),
        ("UniversalGateSet", universal_gate_set_ty()),
        ("solovay_kitaev", solovay_kitaev_ty()),
        ("ht_clifford_universal", ht_clifford_universal_ty()),
        ("QFTState", qft_state_ty()),
        ("QFTGateCount", arrow(nat_ty(), nat_ty())),
        ("qft_correctness", qft_correctness_ty()),
        ("qft_complexity", qft_complexity_ty()),
        ("period_finding", period_finding_ty()),
        ("shor_factoring", shor_factoring_ty()),
        ("amplitude_amplification", amplitude_amplification_ty()),
        ("grover_optimality", grover_optimality_ty()),
        ("phase_estimation", phase_estimation_ty()),
        ("PhaseError", arrow(nat_ty(), real_ty())),
        ("Real.two", real_ty()),
        ("Real.pow", arrow(real_ty(), arrow(nat_ty(), real_ty()))),
        (
            "phase_estimation_precision",
            phase_estimation_precision_ty(),
        ),
        ("VQEState", vqe_state_ty()),
        ("vqe_variational_principle", vqe_variational_principle_ty()),
        ("QAOAState", qaoa_state_ty()),
        ("qaoa_approximation", qaoa_approximation_ty()),
        ("StabilizerCode", stabilizer_code_ty()),
        (
            "stabilizer_code_corrects_errors",
            stabilizer_code_corrects_errors_ty(),
        ),
        ("quantum_hamming_bound", quantum_hamming_bound_ty()),
        ("SurfaceCode", surface_code_ty()),
        ("surface_code_distance", surface_code_distance_ty()),
        ("fault_tolerant_threshold", fault_tolerant_threshold_ty()),
        ("dense_coding_protocol", dense_coding_protocol_ty()),
        ("bb84_security", bb84_security_ty()),
        ("bb84_key_rate", bb84_key_rate_ty()),
        ("bqp_contains_bpp", bqp_contains_bpp_ty()),
        ("bqp_in_psharp_p", bqp_in_psharp_p_ty()),
        ("qma_definition", qma_definition_ty()),
        (
            "local_hamiltonian_qma_complete",
            local_hamiltonian_qma_complete_ty(),
        ),
        ("hamiltonian_simulation", hamiltonian_simulation_ty()),
        ("trotter_suzuki_error", trotter_suzuki_error_ty()),
        ("adiabatic_theorem", adiabatic_theorem_ty()),
        ("adiabatic_qc_equivalence", adiabatic_qc_equivalence_ty()),
        ("Anyon", anyon_ty()),
        ("BraidingOperator", braiding_operator_ty()),
        ("non_abelian_statistics", non_abelian_statistics_ty()),
        ("topological_protection", topological_protection_ty()),
        ("fibonacci_anyon", fibonacci_anyon_ty()),
        ("BosonSampling", boson_sampling_ty()),
        ("boson_sampling_hardness", boson_sampling_hardness_ty()),
        ("QuantumVolume", quantum_volume_ty()),
        ("quantum_volume_monotone", quantum_volume_monotone_ty()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    Ok(())
}
/// Optimal number of Grover iterations for `n_qubits` qubits.
///
/// Approximately ⌊π/4 · √(2^n)⌋.
pub fn grover_iterations(n_qubits: u32) -> u32 {
    let n = 1u64 << n_qubits;
    let iters = (PI / 4.0 * (n as f64).sqrt()).floor() as u32;
    iters.max(1)
}
/// Probability of success after `iterations` Grover iterations on `n_qubits` qubits.
///
/// P_success = sin²((2k + 1) · arcsin(1 / √N))  where N = 2^n, k = iterations.
pub fn grover_success_prob(n_qubits: u32, iterations: u32) -> f64 {
    let n = (1u64 << n_qubits) as f64;
    let theta = (1.0 / n.sqrt()).asin();
    let angle = (2 * iterations + 1) as f64 * theta;
    angle.sin().powi(2)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_complex_mul() {
        let i = Complex::i();
        let result = i.mul(&i);
        assert!((result.re - (-1.0)).abs() < 1e-10);
        assert!(result.im.abs() < 1e-10);
        let a = Complex::new(1.0, 1.0);
        let b = Complex::new(1.0, -1.0);
        let r = a.mul(&b);
        assert!((r.re - 2.0).abs() < 1e-10);
        assert!(r.im.abs() < 1e-10);
    }
    #[test]
    fn test_qubit_zero_normalized() {
        let q = Qubit::zero();
        assert!(q.is_normalized());
        assert!((q.prob_zero() - 1.0).abs() < 1e-10);
        assert!(q.prob_one().abs() < 1e-10);
    }
    #[test]
    fn test_qubit_plus_equal_prob() {
        let q = Qubit::plus();
        assert!(q.is_normalized(), "|+⟩ must be normalized");
        assert!((q.prob_zero() - 0.5).abs() < 1e-10, "prob_0 of |+⟩ = 0.5");
        assert!((q.prob_one() - 0.5).abs() < 1e-10, "prob_1 of |+⟩ = 0.5");
    }
    #[test]
    fn test_hadamard_maps_zero_to_plus() {
        let h = Gate2x2::hadamard();
        let zero = Qubit::zero();
        let out = h.apply(&zero);
        let inv_sqrt2 = 1.0 / 2.0f64.sqrt();
        assert!((out.alpha.re - inv_sqrt2).abs() < 1e-10, "H|0⟩ α.re");
        assert!((out.beta.re - inv_sqrt2).abs() < 1e-10, "H|0⟩ β.re");
        assert!(out.is_normalized());
    }
    #[test]
    fn test_pauli_x_maps_zero_to_one() {
        let x = Gate2x2::pauli_x();
        let zero = Qubit::zero();
        let out = x.apply(&zero);
        assert!(out.alpha.abs() < 1e-10, "X|0⟩ α should be 0");
        assert!((out.beta.abs() - 1.0).abs() < 1e-10, "X|0⟩ β should be 1");
    }
    #[test]
    fn test_hadamard_unitary() {
        let h = Gate2x2::hadamard();
        assert!(h.is_unitary(), "Hadamard should be unitary");
        let hh = h.compose(&h);
        let zero = Qubit::zero();
        let out = hh.apply(&zero);
        assert!(
            (out.alpha.re - 1.0).abs() < 1e-9,
            "HH|0⟩ should be |0⟩; got α={}",
            out.alpha
        );
        assert!(
            out.beta.abs() < 1e-9,
            "HH|0⟩ should be |0⟩; got β={}",
            out.beta
        );
    }
    #[test]
    fn test_statevector_new_normalized() {
        for n in 1..=4 {
            let sv = QuantumStatevector::new(n);
            assert!(sv.is_normalized(), "{n}-qubit state should be normalized");
            assert_eq!(sv.n_amplitudes(), 1 << n);
            assert!((sv.prob(0) - 1.0).abs() < 1e-10);
            for i in 1..(1 << n) {
                assert!(sv.prob(i).abs() < 1e-10);
            }
        }
    }
    #[test]
    fn test_grover_iterations_grows_sqrt() {
        let k4 = grover_iterations(4);
        let k8 = grover_iterations(8);
        assert!(k8 > k4, "more qubits → more iterations");
        let p = grover_success_prob(4, k4);
        assert!(p > 0.9, "Grover success prob should be > 90%; got {p:.3}");
    }
    #[test]
    fn test_quantum_register_uniform_superposition() {
        let mut reg = QuantumRegister::new(3);
        reg.prepare_uniform_superposition();
        assert!(
            reg.is_normalized(),
            "uniform superposition must be normalized"
        );
        for i in 0..8 {
            let p = reg.prob(i);
            assert!(
                (p - 0.125).abs() < 1e-9,
                "basis state {i} prob = {p}, expected 0.125"
            );
        }
    }
    #[test]
    fn test_quantum_circuit_hadamard_chain() {
        let mut circuit = QuantumCircuit::new(2);
        circuit.add_gate(Gate2x2::hadamard(), 0);
        circuit.add_gate(Gate2x2::hadamard(), 1);
        let reg = circuit.run();
        assert!(reg.is_normalized());
        for i in 0..4 {
            assert!((reg.prob(i) - 0.25).abs() < 1e-9);
        }
    }
    #[test]
    fn test_quantum_circuit_bell_state() {
        let mut circuit = QuantumCircuit::new(2);
        circuit.add_gate(Gate2x2::hadamard(), 0);
        circuit.add_cnot(0, 1);
        let reg = circuit.run();
        assert!(reg.is_normalized(), "Bell state must be normalized");
        let inv_sqrt2 = 1.0 / 2.0f64.sqrt();
        assert!((reg.amplitude(0).re - inv_sqrt2).abs() < 1e-9);
        assert!((reg.amplitude(3).re - inv_sqrt2).abs() < 1e-9);
        assert!(reg.amplitude(1).abs() < 1e-9);
        assert!(reg.amplitude(2).abs() < 1e-9);
    }
    #[test]
    fn test_grover_oracle_amplifies_target() {
        let n_qubits = 4;
        let target = 5;
        let oracle = GroverOracle::new(target);
        let k = grover_iterations(n_qubits as u32);
        let reg = oracle.run_grover(n_qubits, k);
        assert!(reg.is_normalized(), "Grover register must be normalized");
        let p_target = reg.prob(target);
        let max_non_target = (0..reg.size())
            .filter(|&i| i != target)
            .map(|i| reg.prob(i))
            .fold(0.0f64, f64::max);
        assert!(
            p_target > max_non_target,
            "target prob {p_target:.4} should exceed max non-target {max_non_target:.4}"
        );
    }
    #[test]
    fn test_qft_preserves_norm() {
        let n_qubits = 3;
        let qft = QFTSimulator::new(n_qubits);
        let mut reg = QuantumRegister::new(n_qubits);
        reg.prepare_uniform_superposition();
        qft.apply(&mut reg);
        assert!(
            reg.is_normalized(),
            "QFT must preserve normalization; total prob = {}",
            reg.amplitudes().iter().map(|a| a.abs_sq()).sum::<f64>()
        );
    }
    #[test]
    fn test_qft_transform_of_uniform() {
        let n = 4;
        let qft = QFTSimulator::new(n);
        let uniform = vec![Complex::new(1.0 / (n as f64).sqrt(), 0.0); n];
        let out = qft.transform(&uniform);
        assert!(
            (out[0].abs() - 1.0).abs() < 1e-9,
            "DFT of uniform → delta at 0"
        );
        for k in 1..n {
            assert!(
                out[k].abs() < 1e-9,
                "DFT of uniform: output[{k}] = {} (expected ~0)",
                out[k].abs()
            );
        }
    }
    #[test]
    fn test_pauli_commutation() {
        assert!(!Pauli::X.commutes_with(Pauli::Y));
        assert!(!Pauli::Y.commutes_with(Pauli::Z));
        assert!(!Pauli::Z.commutes_with(Pauli::X));
        assert!(Pauli::X.commutes_with(Pauli::X));
        assert!(Pauli::Y.commutes_with(Pauli::Y));
        assert!(Pauli::Z.commutes_with(Pauli::Z));
        assert!(Pauli::I.commutes_with(Pauli::X));
        assert!(Pauli::I.commutes_with(Pauli::Y));
        assert!(Pauli::I.commutes_with(Pauli::Z));
    }
    #[test]
    fn test_pauli_string_commutation() {
        let zz = PauliString::new(1, vec![Pauli::Z, Pauli::Z]);
        let xx = PauliString::new(1, vec![Pauli::X, Pauli::X]);
        assert!(zz.commutes_with(&xx), "ZZ and XX should commute");
        let zi = PauliString::new(1, vec![Pauli::Z, Pauli::I]);
        let ix = PauliString::new(1, vec![Pauli::I, Pauli::X]);
        assert!(zi.commutes_with(&ix));
    }
    #[test]
    fn test_stabilizer_state_zero_consistent() {
        let stab = StabilizerState::computational_zero(4);
        assert!(
            stab.is_consistent(),
            "|0000⟩ stabilizer generators must commute"
        );
    }
    #[test]
    fn test_stabilizer_state_plus_consistent() {
        let stab = StabilizerState::plus_state(3);
        assert!(
            stab.is_consistent(),
            "|+++⟩ stabilizer generators must commute"
        );
    }
    #[test]
    fn test_build_quantum_computing_env() {
        let mut env = Environment::new();
        build_quantum_computing_env(&mut env).expect("env build should succeed");
        let names: &[&str] = &[
            "GateSet",
            "UniversalGateSet",
            "solovay_kitaev",
            "ht_clifford_universal",
            "qft_correctness",
            "qft_complexity",
            "period_finding",
            "shor_factoring",
            "amplitude_amplification",
            "grover_optimality",
            "phase_estimation",
            "phase_estimation_precision",
            "VQEState",
            "vqe_variational_principle",
            "QAOAState",
            "qaoa_approximation",
            "StabilizerCode",
            "stabilizer_code_corrects_errors",
            "quantum_hamming_bound",
            "SurfaceCode",
            "surface_code_distance",
            "fault_tolerant_threshold",
            "dense_coding_protocol",
            "bb84_security",
            "bb84_key_rate",
            "bqp_contains_bpp",
            "bqp_in_psharp_p",
            "qma_definition",
            "local_hamiltonian_qma_complete",
            "hamiltonian_simulation",
            "trotter_suzuki_error",
            "adiabatic_theorem",
            "adiabatic_qc_equivalence",
            "Anyon",
            "BraidingOperator",
            "non_abelian_statistics",
            "topological_protection",
            "fibonacci_anyon",
            "BosonSampling",
            "boson_sampling_hardness",
            "QuantumVolume",
            "quantum_volume_monotone",
        ];
        for name in names {
            assert!(
                env.get(&Name::str(*name)).is_some(),
                "axiom '{name}' not found in env"
            );
        }
    }
}
/// Standard quantum algorithms summary.
#[allow(dead_code)]
pub fn standard_quantum_algorithms() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        ("Shor", "Integer factorization", "O((log N)^3)"),
        ("Grover", "Unstructured search", "O(sqrt(N))"),
        ("HHL", "Linear systems", "O(log(N) kappa^2 / epsilon)"),
        ("QFT", "Quantum Fourier transform", "O(n^2)"),
        ("QPE", "Phase estimation", "O(n / epsilon)"),
        ("QAOA", "Combinatorial optimization", "variational"),
        ("VQE", "Ground state energy", "variational"),
        ("QRAM", "Quantum random access", "O(log N)"),
        (
            "Amplitude Amplification",
            "Generalized Grover",
            "O(1/sqrt(p))",
        ),
        ("Quantum Walk", "Graph problems", "quadratic speedup"),
    ]
}
#[cfg(test)]
mod qc_ext_tests {
    use super::*;
    #[test]
    fn test_quantum_register() {
        let r = QuantumRegisterData::new(3, "q");
        assert_eq!(r.hilbert_space_dim(), 8);
    }
    #[test]
    fn test_quantum_circuit() {
        let mut c = QuantumCircuitData::new(2);
        c.apply(QuantumGate::hadamard(), vec![0]);
        c.apply(QuantumGate::cnot(), vec![0, 1]);
        assert_eq!(c.gate_count(), 2);
        assert!(c.is_clifford());
    }
    #[test]
    fn test_error_code() {
        let steane = QuantumErrorCode::steane_7();
        assert_eq!(steane.n, 7);
        assert_eq!(steane.corrects_errors_up_to(), 1);
        let sc = QuantumErrorCode::surface_code(3);
        assert_eq!(sc.d, 3);
    }
    #[test]
    fn test_clifford_gottesman_knill() {
        let c = CliffordGroup::new(5);
        assert!(c.is_efficiently_simulable());
        assert!(c.universal_with_t_gate());
    }
    #[test]
    fn test_quantum_channel() {
        let ch = QuantumChannel::depolarizing(2, 0.1);
        assert!(ch.is_unital());
        let ad = QuantumChannel::amplitude_damping(0.3);
        assert!(ad.is_degradable());
    }
    #[test]
    fn test_algorithms_nonempty() {
        let algs = standard_quantum_algorithms();
        assert!(!algs.is_empty());
    }
}
