//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    CSSCode, ConcatenatedCode, FaultToleranceThreshold, GKPCodeSimulator, MagicStateDistillation,
    MagicStateProtocol, PauliOp, PauliOperator, PauliString, QECNormChecker, QuantumLDPC, ShorCode,
    StabCode, StabilizerCode, StabilizerSimulator, SurfaceCode, SurfaceCodeDecoder,
    SyndromeDecoder2, ThresholdEstimator,
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
pub fn bvar(n: u32) -> Expr {
    Expr::BVar(n)
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
pub fn list_ty(elem: Expr) -> Expr {
    app(cst("List"), elem)
}
/// `PauliOp : Type` — a single-qubit Pauli operator {I, X, Y, Z}.
pub fn pauli_op_ty() -> Expr {
    type0()
}
/// `PauliGroup : Nat → Type`
/// The n-qubit Pauli group P_n = {±1,±i} × {I,X,Y,Z}^⊗n.
pub fn pauli_group_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `PauliPhase : Type`
/// The phase factor ∈ {1, i, -1, -i}.
pub fn pauli_phase_ty() -> Expr {
    type0()
}
/// `SymplecticRepresentation : Nat → Type`
/// Symplectic representation of an n-qubit Pauli over F₂²ⁿ.
pub fn symplectic_representation_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `CommutatorPauli : PauliGroup n → PauliGroup n → Bool`
/// Test whether two Paulis commute (true) or anti-commute (false).
pub fn commutator_pauli_ty() -> Expr {
    arrow(
        app(cst("PauliGroup"), nat_ty()),
        arrow(app(cst("PauliGroup"), nat_ty()), bool_ty()),
    )
}
/// `WeightPauli : PauliGroup n → Nat`
/// Number of non-identity tensor factors (Hamming weight).
pub fn weight_pauli_ty() -> Expr {
    arrow(app(cst("PauliGroup"), nat_ty()), nat_ty())
}
/// Theorem: `PauliGroupNonAbelian`
/// The n-qubit Pauli group is non-abelian for n ≥ 1.
pub fn pauli_group_non_abelian_ty() -> Expr {
    prop()
}
/// Theorem: `PauliSquareIdentity`
/// σ² = I for σ ∈ {X, Y, Z}.
pub fn pauli_square_identity_ty() -> Expr {
    prop()
}
/// Theorem: `PauliAntiCommute`
/// XY = iZ, YZ = iX, ZX = iY (Pauli anti-commutation relations).
pub fn pauli_anti_commute_ty() -> Expr {
    prop()
}
/// `StabilizerGroup : Nat → Nat → Type`
/// Stabilizer group S ≤ P_n with k = n - |generators| logical qubits.
pub fn stabilizer_group_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `StabilizerCode : Nat → Nat → Nat → Type`
/// [\[n, k, d\]] stabilizer code: n physical, k logical, distance d.
pub fn stabilizer_code_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), type0())))
}
/// `ErrorSyndrome : Nat → Nat → Type`
/// Syndrome vector in F₂^{n-k} from measuring stabilizer generators.
pub fn error_syndrome_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `LogicalOperator : StabilizerCode n k d → PauliGroup n → Type`
/// A logical Pauli operator commuting with all stabilizers but not in S.
pub fn logical_operator_ty() -> Expr {
    arrow(
        app3(cst("StabilizerCode"), nat_ty(), nat_ty(), nat_ty()),
        arrow(app(cst("PauliGroup"), nat_ty()), type0()),
    )
}
/// `DecoderMap : Nat → Nat → Type`
/// A syndrome decoding map σ ↦ correction ∈ P_n.
pub fn decoder_map_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `CodeDistance : StabilizerCode n k d → Nat`
/// d = min weight of a non-trivial logical operator.
pub fn code_distance_ty() -> Expr {
    arrow(
        app3(cst("StabilizerCode"), nat_ty(), nat_ty(), nat_ty()),
        nat_ty(),
    )
}
/// Theorem: `KnillLaflammeConditions`
/// Error set E is correctable iff ⟨ψ_i|E†_a E_b|ψ_j⟩ = C_{ab} δ_{ij}.
pub fn knill_laflamme_conditions_ty() -> Expr {
    prop()
}
/// Theorem: `QuantumSingletonBound`
/// For an [\[n,k,d\]] code: k ≤ n - 4(d-1) (quantum Singleton bound).
pub fn quantum_singleton_bound_ty() -> Expr {
    prop()
}
/// Theorem: `QuantumHammingBound`
/// For a non-degenerate [\[n,k,d\]] code: 2^k ∑_{j=0}^{t} C(n,j) 3^j ≤ 2^n.
pub fn quantum_hamming_bound_ty() -> Expr {
    prop()
}
/// Theorem: `GottesmanKnillTheorem`
/// Clifford circuits (starting from computational basis) can be classically simulated.
pub fn gottesman_knill_theorem_ty() -> Expr {
    prop()
}
/// `ClassicalCode : Nat → Nat → Nat → Type`
/// \[n, k, d\] classical linear code over F₂.
pub fn classical_code_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), type0())))
}
/// `CSSCode : Nat → Nat → Nat → Type`
/// CSS code [\[n, k₁+k₂-n, d\]] from two classical codes C₁⊥ ⊆ C₂.
pub fn css_code_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), type0())))
}
/// `CSSXStabilizer : CSSCode n k d → Type`
/// X-type stabilizers from the parity check matrix H₂.
pub fn css_x_stabilizer_ty() -> Expr {
    arrow(app3(cst("CSSCode"), nat_ty(), nat_ty(), nat_ty()), type0())
}
/// `CSSZStabilizer : CSSCode n k d → Type`
/// Z-type stabilizers from the parity check matrix H₁.
pub fn css_z_stabilizer_ty() -> Expr {
    arrow(app3(cst("CSSCode"), nat_ty(), nat_ty(), nat_ty()), type0())
}
/// Theorem: `CSSConstruction`
/// If C₁⊥ ⊆ C₂ then the CSS construction yields a valid quantum code.
pub fn css_construction_ty() -> Expr {
    prop()
}
/// Theorem: `CSSTransversalCNOT`
/// CSS codes admit a transversal CNOT gate.
pub fn css_transversal_cnot_ty() -> Expr {
    prop()
}
/// `ShorCode : Type`
/// The 9-qubit Shor code [\[9,1,3\]]: first quantum error correcting code.
pub fn shor_code_ty() -> Expr {
    type0()
}
/// `ShorLogicalZero : ShorCode → Type`
/// |0̄⟩ = (|000⟩+|111⟩)^⊗3 / 2^{3/2}.
pub fn shor_logical_zero_ty() -> Expr {
    arrow(cst("ShorCode"), type0())
}
/// `ShorLogicalOne : ShorCode → Type`
/// |1̄⟩ = (|000⟩−|111⟩)^⊗3 / 2^{3/2}.
pub fn shor_logical_one_ty() -> Expr {
    arrow(cst("ShorCode"), type0())
}
/// `ShorBitFlipCode : Type`
/// The 3-qubit bit-flip repetition code (inner code of Shor).
pub fn shor_bit_flip_code_ty() -> Expr {
    type0()
}
/// `ShorPhaseFlipCode : Type`
/// The 3-qubit phase-flip repetition code (outer code of Shor).
pub fn shor_phase_flip_code_ty() -> Expr {
    type0()
}
/// Theorem: `ShorCorrectsSingleErrors`
/// The Shor code corrects any single-qubit error (X, Y, or Z).
pub fn shor_corrects_single_errors_ty() -> Expr {
    prop()
}
/// Theorem: `ShorIsCSS`
/// The Shor code is a CSS code.
pub fn shor_is_css_ty() -> Expr {
    prop()
}
/// `SteaneCode : Type`
/// The [\[7,1,3\]] Steane code from the \[7,4,3\] Hamming code.
pub fn steane_code_ty() -> Expr {
    type0()
}
/// `SteaneHMatrix : Type`
/// Parity check matrix H of the classical \[7,4,3\] Hamming code.
pub fn steane_h_matrix_ty() -> Expr {
    type0()
}
/// `SteaneStabilizer : SteaneCode → Nat → Type`
/// The i-th stabilizer generator (i = 1..6).
pub fn steane_stabilizer_ty() -> Expr {
    arrow(cst("SteaneCode"), arrow(nat_ty(), type0()))
}
/// Theorem: `SteaneCorrectsSingleErrors`
/// The Steane [\[7,1,3\]] code corrects all single-qubit errors.
pub fn steane_corrects_single_errors_ty() -> Expr {
    prop()
}
/// Theorem: `SteaneTransversalClifford`
/// The Steane code has transversal H, S, and CNOT gates.
pub fn steane_transversal_clifford_ty() -> Expr {
    prop()
}
/// Theorem: `SteaneIsCSS`
/// The Steane code is a CSS code derived from the Hamming code.
pub fn steane_is_css_ty() -> Expr {
    prop()
}
/// `FaultTolerantGate : Nat → Nat → Nat → Type`
/// A fault-tolerant implementation of a gate on an [\[n,k,d\]] code.
pub fn fault_tolerant_gate_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), type0())))
}
/// `TransversalGate : Type`
/// A gate applied independently to each physical qubit in the code block.
pub fn transversal_gate_ty() -> Expr {
    type0()
}
/// `MagicStateInjection : Type`
/// Protocol for injecting a T-gate resource state |T⟩ = T|+⟩.
pub fn magic_state_injection_ty() -> Expr {
    type0()
}
/// `TeleportedGate : Type`
/// Gate teleportation using ancilla resource states.
pub fn teleported_gate_ty() -> Expr {
    type0()
}
/// Theorem: `EasttinKnillTheorem`
/// No quantum code can have a universal set of transversal gates.
pub fn easttin_knill_theorem_ty() -> Expr {
    prop()
}
/// Theorem: `TransversalCliffordCSS`
/// CSS codes admit transversal Clifford gates.
pub fn transversal_clifford_css_ty() -> Expr {
    prop()
}
/// Theorem: `MagicStateDistillationWorks`
/// Noisy T-states can be distilled to high-fidelity T-states using Clifford operations.
pub fn magic_state_distillation_works_ty() -> Expr {
    prop()
}
/// `ErrorThreshold : Real`
/// The fault-tolerance threshold p_th below which error rates can be suppressed.
pub fn error_threshold_ty() -> Expr {
    real_ty()
}
/// `ConcatenatedCode : Nat → Nat → Nat → Nat → Type`
/// [\[n,k,d\]] code concatenated to level L.
pub fn concatenated_code_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), type0()))),
    )
}
/// `FaultToleranceOverhead : Nat → Nat → Real`
/// Resource overhead O(polylog(1/ε)) for error rate ε at code distance d.
pub fn fault_tolerance_overhead_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), real_ty()))
}
/// `PseudoThreshold : Nat → Nat → Nat → Real`
/// Pseudo-threshold for an [\[n,k,d\]] code.
pub fn pseudo_threshold_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), real_ty())))
}
/// Theorem: `ThresholdTheorem`
/// If physical error rate p < p_th then logical error rate can be made arbitrarily small.
pub fn threshold_theorem_ty() -> Expr {
    prop()
}
/// Theorem: `ConcatenatedCodeDistance`
/// Level-L concatenation of [\[n,1,d\]] code has distance d^L.
pub fn concatenated_code_distance_ty() -> Expr {
    prop()
}
/// Theorem: `PolytopeBound`
/// The threshold p_th satisfies p_th ≥ 1/(c·(n-k)^2) for some constant c.
pub fn polytope_bound_ty() -> Expr {
    prop()
}
/// `ColorCode : Nat → Type`
/// A 2D color code on a 4-8-8 or 6-6-6 lattice with distance d.
pub fn color_code_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `ColorCodeLattice : Type`
/// The trivalent 3-colorable lattice underlying a color code.
pub fn color_code_lattice_ty() -> Expr {
    type0()
}
/// `ColorCodeSyndrome : Nat → Type`
/// Syndrome pattern (vertex/plaquette check violations) for a color code.
pub fn color_code_syndrome_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// Theorem: `ColorCodeTransversalT`
/// 2D color codes on 4-8-8 lattice admit a transversal T gate.
pub fn color_code_transversal_t_ty() -> Expr {
    prop()
}
/// Theorem: `ColorCodeEquivalent`
/// 2D color codes are equivalent to two copies of toric/surface codes.
pub fn color_code_equivalent_ty() -> Expr {
    prop()
}
/// `QuantumCapacity : Type → Real`
/// Q(N) = max over n,ρ of (1/n) I_c(ρ, N^⊗n) (quantum capacity).
pub fn quantum_capacity_ty() -> Expr {
    arrow(type0(), real_ty())
}
/// `CoherentInformation : Type → Type → Real`
/// I_c(ρ, N) = S(N(ρ)) − S((id⊗N)(|ψ⟩⟨ψ|)) (coherent information).
pub fn coherent_information_ty() -> Expr {
    arrow(type0(), arrow(type0(), real_ty()))
}
/// `HashingBound : Real → Real`
/// Q ≥ 1 - H(p) - p log₂ 3 for depolarizing channel at rate p.
pub fn hashing_bound_ty() -> Expr {
    arrow(real_ty(), real_ty())
}
/// `QuantumErasureCapacity : Real → Real`
/// Q of erasure channel with erasure probability ε is max(0, 1-2ε).
pub fn quantum_erasure_capacity_ty() -> Expr {
    arrow(real_ty(), real_ty())
}
/// Theorem: `QuantumNoiseThreshold`
/// Depolarizing channel has positive quantum capacity for p < 1/4.
pub fn quantum_noise_threshold_ty() -> Expr {
    prop()
}
/// Theorem: `QuantumShannonTheorem`
/// Q(N) = lim_{n→∞} (1/n) max_ρ I_c(ρ, N^⊗n).
pub fn quantum_shannon_theorem_ty() -> Expr {
    prop()
}
/// Theorem: `NoCloning`
/// Q(N) > 0 implies the channel is not entanglement-breaking.
pub fn no_cloning_capacity_ty() -> Expr {
    prop()
}
/// `KLMatrix : Nat → Nat → Type`
/// The matrix C_{ab} in Knill-Laflamme: ⟨ψ_i|E†_a E_b|ψ_j⟩ = C_{ab} δ_{ij}.
pub fn kl_matrix_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `KLErrorSet : Nat → Type`
/// A set of Kraus operators {E_a} for which KL conditions are checked.
pub fn kl_error_set_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `KLCorrectableCode : Nat → Nat → Nat → Type`
/// An [\[n,k,d\]] code satisfying the Knill-Laflamme conditions for a given error set.
pub fn kl_correctable_code_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), type0())))
}
/// `KLNecessaryCondition : KLErrorSet n → StabilizerCode n k d → Prop`
/// Necessary KL condition: E†_a E_b must act as a scalar on the code space.
pub fn kl_necessary_condition_ty() -> Expr {
    arrow(
        app(cst("KLErrorSet"), nat_ty()),
        arrow(
            app3(cst("StabilizerCode"), nat_ty(), nat_ty(), nat_ty()),
            prop(),
        ),
    )
}
/// `KLSufficientCondition : KLErrorSet n → StabilizerCode n k d → Prop`
/// Sufficient KL condition: there exists a recovery channel correcting all errors in E.
pub fn kl_sufficient_condition_ty() -> Expr {
    arrow(
        app(cst("KLErrorSet"), nat_ty()),
        arrow(
            app3(cst("StabilizerCode"), nat_ty(), nat_ty(), nat_ty()),
            prop(),
        ),
    )
}
/// Theorem: `KLEquivalence`
/// A code is error-correcting for E iff KL necessary and sufficient conditions both hold.
pub fn kl_equivalence_ty() -> Expr {
    prop()
}
/// Theorem: `KLDegenerateCode`
/// A degenerate code can correct errors where C_{ab} is non-scalar; KL still applies.
pub fn kl_degenerate_code_ty() -> Expr {
    prop()
}
/// `ReedMullerCode : Nat → Nat → Type`
/// Classical Reed-Muller code R(r, m) with parameters \[2^m, ∑_{i≤r} C(m,i), 2^{m-r}\].
pub fn reed_muller_code_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `QuantumReedMullerCode : Nat → Nat → Type`
/// Quantum Reed-Muller code [\[2^m, k, d\]] from two RM codes.
pub fn quantum_reed_muller_code_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `RMTransversalT : Nat → Nat → Prop`
/// RM code CSS construction admits transversal T gate for appropriate r, m.
pub fn rm_transversal_t_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `RMCodeConcatenation : Nat → Nat → Nat → Type`
/// Concatenated RM code achieving fault-tolerance with transversal gates.
pub fn rm_code_concatenation_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), type0())))
}
/// Theorem: `RMTransversalUniversal`
/// Concatenating two different RM codes yields a transversally universal gate set.
pub fn rm_transversal_universal_ty() -> Expr {
    prop()
}
/// Theorem: `RMCodeDistance`
/// Quantum RM code [\[2^m, 1, 2^{m-r}\]] has distance 2^{m-r}.
pub fn rm_code_distance_ty() -> Expr {
    prop()
}
/// `SurfaceCode : Nat → Type`
/// Distance-d surface code [\[d², 1, d\]] on a d×d planar lattice.
pub fn surface_code_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `ToricCode : Nat → Type`
/// Distance-d toric code [\[2d², 2, d\]] on a torus (Kitaev's toric code).
pub fn toric_code_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `SurfaceCodeVertex : Nat → Type`
/// Vertex (star) operators A_v = ∏_{e∈v} X_e for the surface code.
pub fn surface_code_vertex_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `SurfaceCodePlaquette : Nat → Type`
/// Plaquette operators B_p = ∏_{e∈p} Z_e for the surface code.
pub fn surface_code_plaquette_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `SurfaceCodeLogical : Nat → Type`
/// Logical X and Z operators as homologically non-trivial paths on the lattice.
pub fn surface_code_logical_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// Theorem: `SurfaceCodeDistance`
/// The surface code [\[d², 1, d\]] has code distance d (minimum weight logical).
pub fn surface_code_distance_ty() -> Expr {
    prop()
}
/// Theorem: `ToricCodeAnyons`
/// Excitations of toric code are Abelian anyons (e and m particles).
pub fn toric_code_anyons_ty() -> Expr {
    prop()
}
/// Theorem: `SurfaceCodeThreshold`
/// Surface code has a fault-tolerance threshold ~1% under local noise.
pub fn surface_code_threshold_ty() -> Expr {
    prop()
}
/// `QLDPCCode : Nat → Nat → Nat → Type`
/// Quantum low-density parity-check code [\[n, k, d\]] with constant check weight.
pub fn qldpc_code_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), type0())))
}
/// `FiberBundleCode : Nat → Nat → Type`
/// Fiber bundle LDPC code with linear distance and constant check weight.
pub fn fiber_bundle_code_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `GoodQLDPCCode : Nat → Nat → Nat → Prop`
/// A good qLDPC code with k = Θ(n) logical qubits and d = Θ(n) distance.
pub fn good_qldpc_code_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), prop())))
}
/// `QLDPCTanner : Nat → Type`
/// Tanner graph representation of a qLDPC code.
pub fn qldpc_tanner_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// Theorem: `QLDPCGoodCodes`
/// Good quantum LDPC codes exist with k, d both linear in n.
pub fn qldpc_good_codes_ty() -> Expr {
    prop()
}
/// Theorem: `FiberBundleDistance`
/// Fiber bundle codes achieve distance Ω(n^{3/5}).
pub fn fiber_bundle_distance_ty() -> Expr {
    prop()
}
/// `BosonicCode : Type`
/// A quantum code encoding into bosonic (oscillator) modes.
pub fn bosonic_code_ty() -> Expr {
    type0()
}
/// `CatCode : Nat → Type`
/// Cat code with S-fold symmetry protecting against amplitude damping.
pub fn cat_code_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `GKPCode : Real → Type`
/// Gottesman-Kitaev-Preskill code with lattice spacing Δ in phase space.
pub fn gkp_code_ty() -> Expr {
    arrow(real_ty(), type0())
}
/// `BinomialCode : Nat → Nat → Type`
/// Binomial code [\[N, M, d\]]_b protecting against d-photon loss/gain errors.
pub fn binomial_code_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `GKPDisplacementError : Real → Real → Type`
/// A displacement error D(α) in phase space acting on a GKP codeword.
pub fn gkp_displacement_error_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), type0()))
}
/// Theorem: `GKPCorrectsBoundedDisplacement`
/// GKP code corrects displacement errors with |α| < Δ/2.
pub fn gkp_corrects_bounded_displacement_ty() -> Expr {
    prop()
}
/// Theorem: `CatCodeLossTolerance`
/// S-cat code detects up to S-1 photon losses.
pub fn cat_code_loss_tolerance_ty() -> Expr {
    prop()
}
/// `NoiseModel : Type`
/// A noise model specifying single-qubit, two-qubit, and measurement error rates.
pub fn noise_model_ty() -> Expr {
    type0()
}
/// `DepolarizingNoise : Real → Type`
/// Depolarizing noise channel with uniform error rate p per gate.
pub fn depolarizing_noise_ty() -> Expr {
    arrow(real_ty(), type0())
}
/// `CircuitNoiseThreshold : NoiseModel → Real`
/// The circuit-level noise threshold for a given noise model.
pub fn circuit_noise_threshold_ty() -> Expr {
    arrow(cst("NoiseModel"), real_ty())
}
/// `FaultToleranceGadget : Nat → Nat → Nat → Type`
/// A fault-tolerant implementation gadget satisfying threshold conditions.
pub fn fault_tolerance_gadget_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), type0())))
}
/// Theorem: `CircuitThresholdTheorem`
/// For circuit-level noise below threshold, arbitrarily long computations are possible.
pub fn circuit_threshold_theorem_ty() -> Expr {
    prop()
}
/// Theorem: `ThresholdUpperBound`
/// No fault-tolerance threshold exists above 50% for arbitrary noise models.
pub fn threshold_upper_bound_ty() -> Expr {
    prop()
}
/// `MagicState : Type`
/// A non-stabilizer (magic) resource state enabling non-Clifford gates.
pub fn magic_state_ty() -> Expr {
    type0()
}
/// `TState : Type`
/// The T-gate magic state |T⟩ = T|+⟩ = (|0⟩ + e^{iπ/4}|1⟩)/√2.
pub fn t_state_ty() -> Expr {
    type0()
}
/// `DistillationProtocol : Nat → Nat → Real → Type`
/// A distillation protocol consuming n input magic states to produce k outputs at fidelity F.
pub fn distillation_protocol_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(real_ty(), type0())))
}
/// `MagicStateRobustness : Real`
/// Robustness of magic: minimum overhead for simulating non-Clifford operations.
pub fn magic_state_robustness_ty() -> Expr {
    real_ty()
}
/// Theorem: `BravyiKitaevDistillation`
/// The 15-to-1 distillation protocol asymptotically achieves cubic error suppression.
pub fn bravyi_kitaev_distillation_ty() -> Expr {
    prop()
}
/// Theorem: `MagicStateNonStabilizer`
/// Magic states are not stabilizer states; their Wigner function has negative values.
pub fn magic_state_non_stabilizer_ty() -> Expr {
    prop()
}
/// `MWPMDecoder : Nat → Nat → Nat → Type`
/// Minimum-weight perfect matching decoder for [\[n,k,d\]] stabilizer codes.
pub fn mwpm_decoder_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), type0())))
}
/// `BeliefPropDecoder : Nat → Nat → Nat → Type`
/// Belief propagation decoder for qLDPC codes.
pub fn belief_prop_decoder_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), type0())))
}
/// `UnionFindDecoder : Nat → Type`
/// Union-find decoder achieving near-linear decoding time.
pub fn union_find_decoder_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `MLDecoder : Nat → Nat → Nat → Type`
/// Maximum-likelihood decoder (optimal but exponential complexity).
pub fn ml_decoder_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), type0())))
}
/// `DecodingThreshold : Type → Real`
/// The decoding threshold for a given decoder: physical error rate below which decoding succeeds.
pub fn decoding_threshold_ty() -> Expr {
    arrow(type0(), real_ty())
}
/// Theorem: `MWPMOptimalSurface`
/// MWPM achieves the optimal threshold for the surface code under independent noise.
pub fn mwpm_optimal_surface_ty() -> Expr {
    prop()
}
/// Theorem: `BPConvergenceLDPC`
/// Belief propagation converges for cycle-free factor graphs (tree-like codes).
pub fn bp_convergence_ldpc_ty() -> Expr {
    prop()
}
/// `SubsystemCode : Nat → Nat → Nat → Nat → Type`
/// [\[n, k, r, d\]] subsystem (operator) code with k logical and r gauge qubits.
pub fn subsystem_code_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), type0()))),
    )
}
/// `GaugeGroup : Nat → Nat → Nat → Nat → Type`
/// Gauge group G of a subsystem code (generated by gauge operators).
pub fn gauge_group_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), type0()))),
    )
}
/// `BaconShorCode : Nat → Type`
/// Bacon-Shor code on an m×m grid: [\[m², 1, r, m\]] subsystem code.
pub fn bacon_shor_code_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `OperatorQEC : Nat → Nat → Nat → Nat → Prop`
/// Operator quantum error correction condition for subsystem codes.
pub fn operator_qec_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), prop()))),
    )
}
/// Theorem: `SubsystemCodeCorrection`
/// A subsystem code corrects E iff the operator QEC conditions hold for gauge-reduced errors.
pub fn subsystem_code_correction_ty() -> Expr {
    prop()
}
/// Theorem: `BaconShorSingleFaultTolerant`
/// The Bacon-Shor code is single-fault-tolerant for adversarial noise.
pub fn bacon_shor_single_fault_tolerant_ty() -> Expr {
    prop()
}
/// `CodeSwitching : Nat → Nat → Nat → Nat → Nat → Nat → Type`
/// Protocol for switching between two codes [\[n1,k,d1\]] and [\[n2,k,d2\]].
pub fn code_switching_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(
            nat_ty(),
            arrow(
                nat_ty(),
                arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), type0()))),
            ),
        ),
    )
}
/// `GateSynthesis : Nat → Real → Type`
/// Approximation of a target unitary to precision ε using n gates from a fault-tolerant set.
pub fn gate_synthesis_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), type0()))
}
/// `SolovayKitaevApprox : Real → Nat`
/// Solovay-Kitaev theorem: approximation to ε requires O(log^c(1/ε)) gates.
pub fn solovay_kitaev_approx_ty() -> Expr {
    arrow(real_ty(), nat_ty())
}
/// Theorem: `SolovayKitaevTheorem`
/// Any single-qubit unitary can be approximated to ε using O(log^{3.97}(1/ε)) Clifford+T gates.
pub fn solovay_kitaev_theorem_ty() -> Expr {
    prop()
}
/// Theorem: `TCountOptimality`
/// Minimizing T-count in Clifford+T circuits is #P-hard in general.
pub fn t_count_optimality_ty() -> Expr {
    prop()
}
#[allow(clippy::too_many_arguments)]
pub fn build_quantum_error_correction_env(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        ("PauliOp", pauli_op_ty()),
        ("PauliGroup", pauli_group_ty()),
        ("PauliPhase", pauli_phase_ty()),
        ("SymplecticRepresentation", symplectic_representation_ty()),
        ("CommutatorPauli", commutator_pauli_ty()),
        ("WeightPauli", weight_pauli_ty()),
        ("PauliGroupNonAbelian", pauli_group_non_abelian_ty()),
        ("PauliSquareIdentity", pauli_square_identity_ty()),
        ("PauliAntiCommute", pauli_anti_commute_ty()),
        ("StabilizerGroup", stabilizer_group_ty()),
        ("StabilizerCode", stabilizer_code_ty()),
        ("ErrorSyndrome", error_syndrome_ty()),
        ("LogicalOperator", logical_operator_ty()),
        ("DecoderMap", decoder_map_ty()),
        ("CodeDistance", code_distance_ty()),
        ("KnillLaflammeConditions", knill_laflamme_conditions_ty()),
        ("QuantumSingletonBound", quantum_singleton_bound_ty()),
        ("QuantumHammingBound", quantum_hamming_bound_ty()),
        ("GottesmanKnillTheorem", gottesman_knill_theorem_ty()),
        ("ClassicalCode", classical_code_ty()),
        ("CSSCode", css_code_ty()),
        ("CSSXStabilizer", css_x_stabilizer_ty()),
        ("CSSZStabilizer", css_z_stabilizer_ty()),
        ("CSSConstruction", css_construction_ty()),
        ("CSSTransversalCNOT", css_transversal_cnot_ty()),
        ("ShorCode", shor_code_ty()),
        ("ShorLogicalZero", shor_logical_zero_ty()),
        ("ShorLogicalOne", shor_logical_one_ty()),
        ("ShorBitFlipCode", shor_bit_flip_code_ty()),
        ("ShorPhaseFlipCode", shor_phase_flip_code_ty()),
        ("ShorCorrectsSingleErrors", shor_corrects_single_errors_ty()),
        ("ShorIsCSS", shor_is_css_ty()),
        ("SteaneCode", steane_code_ty()),
        ("SteaneHMatrix", steane_h_matrix_ty()),
        ("SteaneStabilizer", steane_stabilizer_ty()),
        (
            "SteaneCorrectsSingleErrors",
            steane_corrects_single_errors_ty(),
        ),
        (
            "SteaneTransversalClifford",
            steane_transversal_clifford_ty(),
        ),
        ("SteaneIsCSS", steane_is_css_ty()),
        ("FaultTolerantGate", fault_tolerant_gate_ty()),
        ("TransversalGate", transversal_gate_ty()),
        ("MagicStateInjection", magic_state_injection_ty()),
        ("TeleportedGate", teleported_gate_ty()),
        ("EasttinKnillTheorem", easttin_knill_theorem_ty()),
        ("TransversalCliffordCSS", transversal_clifford_css_ty()),
        (
            "MagicStateDistillationWorks",
            magic_state_distillation_works_ty(),
        ),
        ("ErrorThreshold", error_threshold_ty()),
        ("ConcatenatedCode", concatenated_code_ty()),
        ("FaultToleranceOverhead", fault_tolerance_overhead_ty()),
        ("PseudoThreshold", pseudo_threshold_ty()),
        ("ThresholdTheorem", threshold_theorem_ty()),
        ("ConcatenatedCodeDistance", concatenated_code_distance_ty()),
        ("PolytopeBound", polytope_bound_ty()),
        ("ColorCode", color_code_ty()),
        ("ColorCodeLattice", color_code_lattice_ty()),
        ("ColorCodeSyndrome", color_code_syndrome_ty()),
        ("ColorCodeTransversalT", color_code_transversal_t_ty()),
        ("ColorCodeEquivalent", color_code_equivalent_ty()),
        ("QuantumCapacity", quantum_capacity_ty()),
        ("CoherentInformation", coherent_information_ty()),
        ("HashingBound", hashing_bound_ty()),
        ("QuantumErasureCapacity", quantum_erasure_capacity_ty()),
        ("QuantumNoiseThreshold", quantum_noise_threshold_ty()),
        ("QuantumShannonTheorem", quantum_shannon_theorem_ty()),
        ("NoCloning", no_cloning_capacity_ty()),
        ("KLMatrix", kl_matrix_ty()),
        ("KLErrorSet", kl_error_set_ty()),
        ("KLCorrectableCode", kl_correctable_code_ty()),
        ("KLNecessaryCondition", kl_necessary_condition_ty()),
        ("KLSufficientCondition", kl_sufficient_condition_ty()),
        ("KLEquivalence", kl_equivalence_ty()),
        ("KLDegenerateCode", kl_degenerate_code_ty()),
        ("ReedMullerCode", reed_muller_code_ty()),
        ("QuantumReedMullerCode", quantum_reed_muller_code_ty()),
        ("RMTransversalT", rm_transversal_t_ty()),
        ("RMCodeConcatenation", rm_code_concatenation_ty()),
        ("RMTransversalUniversal", rm_transversal_universal_ty()),
        ("RMCodeDistance", rm_code_distance_ty()),
        ("SurfaceCode", surface_code_ty()),
        ("ToricCode", toric_code_ty()),
        ("SurfaceCodeVertex", surface_code_vertex_ty()),
        ("SurfaceCodePlaquette", surface_code_plaquette_ty()),
        ("SurfaceCodeLogical", surface_code_logical_ty()),
        ("SurfaceCodeDistance", surface_code_distance_ty()),
        ("ToricCodeAnyons", toric_code_anyons_ty()),
        ("SurfaceCodeThreshold", surface_code_threshold_ty()),
        ("QLDPCCode", qldpc_code_ty()),
        ("FiberBundleCode", fiber_bundle_code_ty()),
        ("GoodQLDPCCode", good_qldpc_code_ty()),
        ("QLDPCTanner", qldpc_tanner_ty()),
        ("QLDPCGoodCodes", qldpc_good_codes_ty()),
        ("FiberBundleDistance", fiber_bundle_distance_ty()),
        ("BosonicCode", bosonic_code_ty()),
        ("CatCode", cat_code_ty()),
        ("GKPCode", gkp_code_ty()),
        ("BinomialCode", binomial_code_ty()),
        ("GKPDisplacementError", gkp_displacement_error_ty()),
        (
            "GKPCorrectsBoundedDisplacement",
            gkp_corrects_bounded_displacement_ty(),
        ),
        ("CatCodeLossTolerance", cat_code_loss_tolerance_ty()),
        ("NoiseModel", noise_model_ty()),
        ("DepolarizingNoise", depolarizing_noise_ty()),
        ("CircuitNoiseThreshold", circuit_noise_threshold_ty()),
        ("FaultToleranceGadget", fault_tolerance_gadget_ty()),
        ("CircuitThresholdTheorem", circuit_threshold_theorem_ty()),
        ("ThresholdUpperBound", threshold_upper_bound_ty()),
        ("MagicState", magic_state_ty()),
        ("TState", t_state_ty()),
        ("DistillationProtocol", distillation_protocol_ty()),
        ("MagicStateRobustness", magic_state_robustness_ty()),
        ("BravyiKitaevDistillation", bravyi_kitaev_distillation_ty()),
        ("MagicStateNonStabilizer", magic_state_non_stabilizer_ty()),
        ("MWPMDecoder", mwpm_decoder_ty()),
        ("BeliefPropDecoder", belief_prop_decoder_ty()),
        ("UnionFindDecoder", union_find_decoder_ty()),
        ("MLDecoder", ml_decoder_ty()),
        ("DecodingThreshold", decoding_threshold_ty()),
        ("MWPMOptimalSurface", mwpm_optimal_surface_ty()),
        ("BPConvergenceLDPC", bp_convergence_ldpc_ty()),
        ("SubsystemCode", subsystem_code_ty()),
        ("GaugeGroup", gauge_group_ty()),
        ("BaconShorCode", bacon_shor_code_ty()),
        ("OperatorQEC", operator_qec_ty()),
        ("SubsystemCodeCorrection", subsystem_code_correction_ty()),
        (
            "BaconShorSingleFaultTolerant",
            bacon_shor_single_fault_tolerant_ty(),
        ),
        ("CodeSwitching", code_switching_ty()),
        ("GateSynthesis", gate_synthesis_ty()),
        ("SolovayKitaevApprox", solovay_kitaev_approx_ty()),
        ("SolovayKitaevTheorem", solovay_kitaev_theorem_ty()),
        ("TCountOptimality", t_count_optimality_ty()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .map_err(|e| format!("Failed to add '{}': {:?}", name, e))?;
    }
    Ok(())
}
/// Compute the binary entropy H(p) = -p log₂ p - (1-p) log₂(1-p).
pub fn binary_entropy(p: f64) -> f64 {
    if p <= 0.0 || p >= 1.0 {
        return 0.0;
    }
    -p * p.log2() - (1.0 - p) * (1.0 - p).log2()
}
/// Hashing bound for the depolarizing channel with error rate p.
/// Q ≥ max(0, 1 - H(p) - p log₂ 3).
pub fn hashing_bound(p: f64) -> f64 {
    let q = 1.0 - binary_entropy(p) - p * 3.0_f64.log2();
    q.max(0.0)
}
/// Quantum capacity of the erasure channel with erasure probability ε.
/// Q = max(0, 1 - 2ε).
pub fn erasure_channel_capacity(epsilon: f64) -> f64 {
    (1.0 - 2.0 * epsilon).max(0.0)
}
/// Approximate quantum capacity of the amplitude damping channel.
/// Q ≈ 1 - H(γ) for small γ.
pub fn amplitude_damping_capacity(gamma: f64) -> f64 {
    (1.0 - binary_entropy(gamma)).max(0.0)
}
/// T-state fidelity after one round of [\[15,1,3\]] distillation.
/// Input fidelity F → output fidelity F_out ≈ 1 - 35(1-F)³.
pub fn distill_t_state_fidelity(f_in: f64) -> f64 {
    let eps = 1.0 - f_in;
    let f_out = 1.0 - 35.0 * eps.powi(3);
    f_out.min(1.0).max(0.0)
}
/// Number of T-state distillation rounds needed to reach target fidelity.
pub fn distillation_rounds(f_init: f64, f_target: f64) -> usize {
    let mut f = f_init;
    let mut rounds = 0;
    while f < f_target && rounds < 100 {
        f = distill_t_state_fidelity(f);
        rounds += 1;
    }
    rounds
}
/// Overhead (input T-states per output T-state) for k rounds.
/// Each round uses 15 input states to produce 1 output.
pub fn distillation_overhead(rounds: usize) -> u64 {
    15u64.pow(rounds as u32)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pauli_commutation() {
        assert!(!PauliOp::X.commutes_with(PauliOp::Z));
        assert!(!PauliOp::X.commutes_with(PauliOp::Y));
        assert!(PauliOp::X.commutes_with(PauliOp::X));
        for &p in &[PauliOp::I, PauliOp::X, PauliOp::Y, PauliOp::Z] {
            assert!(PauliOp::I.commutes_with(p));
        }
    }
    #[test]
    fn test_pauli_string_commutation() {
        let a = PauliString::new(vec![PauliOp::Z, PauliOp::Z, PauliOp::I]);
        let b = PauliString::new(vec![PauliOp::I, PauliOp::Z, PauliOp::Z]);
        assert!(a.commutes_with(&b));
        let c = PauliString::new(vec![PauliOp::X, PauliOp::X, PauliOp::I]);
        assert!(!c.commutes_with(&b));
    }
    #[test]
    fn test_stabilizer_code_3qubit() {
        let code = StabilizerCode::bit_flip_3();
        assert_eq!(code.n, 3);
        assert_eq!(code.k, 1);
        let x_err = vec![0u8; 3];
        let z_err = vec![0u8; 3];
        let synd = code.syndrome(&x_err, &z_err);
        assert!(synd.iter().all(|&s| s == 0));
        let x_err1 = vec![1u8, 0, 0];
        assert!(code.detects_error(&x_err1, &z_err));
    }
    #[test]
    fn test_steane_7_code() {
        let code = StabilizerCode::steane_7();
        assert_eq!(code.n, 7);
        assert_eq!(code.k, 1);
        assert_eq!(code.d, 3);
        let x_err = vec![0u8; 7];
        let z_err = vec![0u8; 7];
        let synd = code.syndrome(&x_err, &z_err);
        assert!(
            synd.iter().all(|&s| s == 0),
            "No-error syndrome should be zero"
        );
        assert!(code.satisfies_singleton_bound());
        assert!(code.satisfies_hamming_bound());
    }
    #[test]
    fn test_shor_code() {
        let shor = ShorCode::new();
        assert_eq!(shor.n(), 9);
        assert_eq!(shor.k(), 1);
        assert_eq!(shor.d(), 3);
        assert!(shor.corrects_single_qubit_error());
        assert!(shor.satisfies_singleton_bound());
        let x_err = vec![0u8; 9];
        let z_err = vec![0u8; 9];
        let synd = shor.syndrome(&x_err, &z_err);
        assert!(synd.iter().all(|&s| s == 0));
    }
    #[test]
    fn test_css_steane_syndrome() {
        let css = CSSCode::steane();
        let no_err = vec![0u8; 7];
        assert!(!css.detects_x_error(&no_err));
        assert!(!css.detects_z_error(&no_err));
        let mut x_err = vec![0u8; 7];
        x_err[0] = 1;
        assert!(css.detects_x_error(&x_err));
    }
    #[test]
    fn test_hashing_bound() {
        assert!((hashing_bound(0.0) - 1.0).abs() < 1e-9);
        let q = hashing_bound(0.25);
        assert!(q >= 0.0);
        assert!((erasure_channel_capacity(0.0) - 1.0).abs() < 1e-9);
        assert!((erasure_channel_capacity(0.5)).abs() < 1e-9);
        assert!(erasure_channel_capacity(0.3) > 0.0);
    }
    #[test]
    fn test_magic_state_distillation() {
        let f_init = 0.95;
        let f_out = distill_t_state_fidelity(f_init);
        assert!(f_out > f_init, "Distillation should improve fidelity");
        let rounds = distillation_rounds(f_init, 0.9999);
        assert!(rounds >= 1);
        assert_eq!(distillation_overhead(1), 15);
        assert_eq!(distillation_overhead(2), 225);
    }
    #[test]
    fn test_concatenated_code() {
        let cc = ConcatenatedCode::new(7, 3, 2);
        assert_eq!(cc.num_physical_qubits(), 49);
        assert_eq!(cc.distance(), 9);
        let cc1 = ConcatenatedCode::new(7, 3, 1);
        let cc2 = ConcatenatedCode::new(7, 3, 2);
        let p = 0.001;
        let th = 0.01;
        assert!(cc2.logical_error_rate(p, th) < cc1.logical_error_rate(p, th));
    }
    #[test]
    fn test_build_quantum_error_correction_env() {
        let mut env = oxilean_kernel::Environment::new();
        let result = build_quantum_error_correction_env(&mut env);
        assert!(
            result.is_ok(),
            "build_quantum_error_correction_env failed: {:?}",
            result.err()
        );
    }
    #[test]
    fn test_surface_code_decoder() {
        let decoder = SurfaceCodeDecoder::new(3);
        let corrections = decoder.decode(&[]);
        assert!(corrections.is_empty());
        let d3 = SurfaceCodeDecoder::new(3);
        let d5 = SurfaceCodeDecoder::new(5);
        let p = 0.001;
        assert!(d5.logical_error_rate(p) < d3.logical_error_rate(p));
        assert!(d3.logical_error_rate(0.005) < 0.5);
    }
    #[test]
    fn test_surface_code_decoder_syndrome_matching() {
        let decoder = SurfaceCodeDecoder::new(5);
        let syndrome = vec![(1, 1), (1, 2)];
        let corrections = decoder.decode(&syndrome);
        assert!(!corrections.is_empty());
    }
    #[test]
    fn test_stabilizer_simulator_init() {
        let sim = StabilizerSimulator::new(3);
        assert_eq!(sim.measure_z(0), 0);
        assert_eq!(sim.measure_z(1), 0);
        assert_eq!(sim.measure_z(2), 0);
    }
    #[test]
    fn test_stabilizer_simulator_hadamard() {
        let mut sim = StabilizerSimulator::new(2);
        sim.hadamard(0);
        assert_eq!(sim.stab_x[0][0], 1);
        assert_eq!(sim.stab_z[0][0], 0);
    }
    #[test]
    fn test_stabilizer_simulator_phase_gate() {
        let mut sim = StabilizerSimulator::new(1);
        sim.phase_gate(0);
        assert_eq!(sim.stab_z[0][0], 1);
        assert_eq!(sim.stab_x[0][0], 0);
    }
    #[test]
    fn test_fault_tolerance_threshold() {
        let ft = FaultToleranceThreshold::new(7, 1, 3);
        let p_th = ft.estimate_threshold();
        assert!(p_th > 0.0);
        assert!((ft.logical_error_rate(p_th, 1) - 0.5).abs() < 1e-10);
        let p = p_th * 0.5;
        assert!(ft.logical_error_rate(p, 2) < ft.logical_error_rate(p, 1));
        let level = ft.min_level(p, 1e-6);
        assert!(level < u32::MAX);
    }
    #[test]
    fn test_qec_norm_checker_steane() {
        let code = StabilizerCode::steane_7();
        let checker = QECNormChecker::new(code);
        assert!(
            checker.all_single_qubit_errors_correctable(),
            "Steane code should correct all single-qubit errors"
        );
    }
    #[test]
    fn test_qec_norm_checker_kl_condition() {
        let code = StabilizerCode::steane_7();
        let checker = QECNormChecker::new(code);
        assert!(checker.satisfies_kl_conditions(&[0u8; 7], &[0u8; 7]));
        let mut x_err = vec![0u8; 7];
        x_err[0] = 1;
        assert!(checker.satisfies_kl_conditions(&x_err, &[0u8; 7]));
    }
    #[test]
    fn test_gkp_code_simulator_correction() {
        let gkp = GKPCodeSimulator::new();
        let delta = gkp.lattice_spacing;
        assert!(gkp.is_correctable(delta * 0.1, delta * 0.1));
        assert!(!gkp.is_correctable(delta * 0.6, 0.0));
        let (rq, rp) = gkp.correct_displacement(0.1, 0.05);
        assert!(rq.abs() < delta / 2.0);
        assert!(rp.abs() < delta / 2.0);
    }
    #[test]
    fn test_gkp_code_simulator_logical_error_rate() {
        let gkp = GKPCodeSimulator::new();
        let rate_small = gkp.logical_error_rate_gaussian(0.1);
        let rate_large = gkp.logical_error_rate_gaussian(0.5);
        assert!(
            rate_small < rate_large,
            "Larger noise should give larger error rate"
        );
        assert_eq!(gkp.logical_error_rate_gaussian(0.0), 0.0);
    }
    #[test]
    fn test_gkp_simulate_errors() {
        let gkp = GKPCodeSimulator::new();
        let delta = gkp.lattice_spacing;
        let displacements = vec![
            (delta * 0.1, delta * 0.1),
            (delta * 0.4, delta * 0.4),
            (delta * 0.6, 0.0),
        ];
        let uncorrected = gkp.simulate_errors(&displacements);
        assert_eq!(
            uncorrected, 1,
            "Only one displacement should be uncorrectable"
        );
    }
}
#[cfg(test)]
mod tests_qec_extra {
    use super::*;
    #[test]
    fn test_pauli_commutation() {
        let x = PauliOperator::single_x(2, 0);
        let z = PauliOperator::single_z(2, 0);
        assert!(!x.commutes_with(&z), "X and Z anticommute");
        let x1 = PauliOperator::single_x(2, 0);
        let x2 = PauliOperator::single_x(2, 1);
        assert!(x1.commutes_with(&x2), "X on different qubits commute");
    }
    #[test]
    fn test_five_qubit_code() {
        let code = StabCode::five_qubit_code();
        assert_eq!(code.n_physical, 5);
        assert_eq!(code.k_logical, 1);
        assert_eq!(code.generators.len(), 4);
        for i in 0..4 {
            for j in (i + 1)..4 {
                assert!(
                    code.generators[i].commutes_with(&code.generators[j]),
                    "Generator {} must commute with {}",
                    i,
                    j
                );
            }
        }
    }
    #[test]
    fn test_steane_code_structure() {
        let code = StabCode::steane_code();
        assert_eq!(code.n_physical, 7);
        assert_eq!(code.k_logical, 1);
        assert_eq!(code.generators.len(), 6);
    }
    #[test]
    fn test_syndrome_decoder_weight1() {
        let code = StabCode::five_qubit_code();
        let mut decoder = SyndromeDecoder2::new(code);
        decoder.build_weight1_table();
        assert!(decoder.lookup_table.len() <= 15);
    }
    #[test]
    fn test_threshold_estimator() {
        let est = ThresholdEstimator::new(0.001, 3, 10);
        let threshold = 0.01;
        assert!(est.is_below_threshold(threshold));
        let logical_rate = est.logical_error_rate(threshold);
        assert!(
            logical_rate < est.physical_error_rate,
            "Logical rate should improve below threshold"
        );
    }
    #[test]
    fn test_surface_code() {
        let sc = SurfaceCode::new(5);
        assert_eq!(sc.code_distance(), 5);
        assert_eq!(sc.n_data_qubits, 25);
        assert!(sc.total_qubits() > 25);
        assert!(SurfaceCode::depolarizing_threshold() > 0.0);
    }
    #[test]
    fn test_magic_state_distillation() {
        let msd = MagicStateDistillation::new(0.01, 1e-10, MagicStateProtocol::FifteenToOne);
        let out = msd.output_error_rate();
        assert!(
            out < msd.input_error_rate,
            "Distillation should reduce error"
        );
        let rounds = msd.n_rounds_needed();
        assert!(rounds > 0);
    }
    #[test]
    fn test_quantum_ldpc() {
        let code = QuantumLDPC::new(100, 10, 10, 6, 6);
        assert!(code.rate() > 0.0);
        let hp = QuantumLDPC::hypergraph_product(4, 8, 4, 8);
        assert!(hp.n > 0);
    }
}
