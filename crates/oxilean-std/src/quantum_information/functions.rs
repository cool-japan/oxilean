//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    BQPComplexity, BellInequality, BlochVector, CSSCode, ChoiMatrix, Complex, Concurrence,
    CvQuantumInfo, CvStateType, DensityMatrix, EntanglementMeasure, GateType, HolographicCode,
    KrausChannel, MixedState, PPTCriterion, PureState, QMAComplexity, QccProtocol, QkdProtocol,
    QuantumChannel, QuantumCircuit, QuantumDiscord, QuantumErrorMitigation, ResourceTheory,
    StabilizerCode, StateDiscrimination, SurfaceCode, SyndromeDecoder,
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
/// `DensityMatrix : Nat → Type`
/// ρ ∈ L(ℂ^d): d×d positive semidefinite matrix with Tr(ρ) = 1.
pub fn density_matrix_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `PureState : Nat → Type`
/// |ψ⟩ ∈ ℂ^d: unit vector (state vector).
pub fn pure_state_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `MixedState : Nat → Type`
/// Mixture ρ = ∑ p_i |ψ_i⟩⟨ψ_i| with p_i ≥ 0, ∑ p_i = 1.
pub fn mixed_state_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `BlochVector : Type`
/// r ∈ ℝ³ with |r| ≤ 1; the qubit ρ = (I + r·σ)/2.
pub fn bloch_vector_ty() -> Expr {
    type0()
}
/// `VonNeumannEntropy : DensityMatrix d → Real`
/// S(ρ) = −Tr(ρ log ρ) = −∑ λ_i log λ_i.
pub fn von_neumann_entropy_ty() -> Expr {
    arrow(app(cst("DensityMatrix"), nat_ty()), real_ty())
}
/// `Purity : DensityMatrix d → Real`
/// γ(ρ) = Tr(ρ²) ∈ [1/d, 1]; equal to 1 iff ρ is pure.
pub fn purity_ty() -> Expr {
    arrow(app(cst("DensityMatrix"), nat_ty()), real_ty())
}
/// Theorem: S(ρ) = 0 iff ρ is a pure state.
pub fn entropy_zero_iff_pure_ty() -> Expr {
    prop()
}
/// Theorem: ρ is pure iff Tr(ρ²) = 1.
pub fn purity_one_iff_pure_ty() -> Expr {
    prop()
}
/// `QuantumChannel : Nat → Nat → Type`
/// Completely positive trace-preserving (CPTP) map ε : L(ℂ^m) → L(ℂ^n).
pub fn quantum_channel_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `KrausRepresentation : Nat → Nat → Nat → Type`
/// ε(ρ) = ∑_{i<r} K_i ρ K_i†, where ∑ K_i†K_i = I.
pub fn kraus_representation_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), type0())))
}
/// `ChoiMatrix : Nat → Nat → Type`
/// J(ε) = (ε ⊗ id)(|Ω⟩⟨Ω|) ∈ L(ℂ^{mn}).
pub fn choi_matrix_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `DepolarizingChannel : Nat → Real → QuantumChannel`
/// ε_p(ρ) = (1−p)ρ + p I/d for p ∈ [0,1].
pub fn depolarizing_channel_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(real_ty(), app2(cst("QuantumChannel"), nat_ty(), nat_ty())),
    )
}
/// `DiamondNorm : QuantumChannel m n → Real`
/// ‖ε‖_◇ = sup_ρ ‖(ε ⊗ id)(ρ)‖₁.
pub fn diamond_norm_ty() -> Expr {
    arrow(app2(cst("QuantumChannel"), nat_ty(), nat_ty()), real_ty())
}
/// Choi-Kraus isomorphism: ε is CPTP iff J(ε) ≥ 0 and Tr_1(J(ε)) = I.
pub fn choi_kraus_isomorphism_ty() -> Expr {
    prop()
}
/// Stinespring dilation: every CPTP map has a unitary dilation.
pub fn stinespring_dilation_ty() -> Expr {
    prop()
}
/// `EntanglementMeasure : Nat → Nat → Type`
/// Entanglement measure for bipartite systems ℂ^m ⊗ ℂ^n.
pub fn entanglement_measure_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `Concurrence : DensityMatrix 4 → Real`
/// C(ρ) for two-qubit states; C = 0 iff separable.
pub fn concurrence_ty() -> Expr {
    arrow(app(cst("DensityMatrix"), cst("Nat.four")), real_ty())
}
/// `PartialTranspose : Nat → Nat → DensityMatrix (m*n) → DensityMatrix (m*n)`
/// ρ^{T_B}: partial transpose with respect to subsystem B.
pub fn partial_transpose_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(
            nat_ty(),
            arrow(
                app(
                    cst("DensityMatrix"),
                    app2(cst("Nat.mul"), nat_ty(), nat_ty()),
                ),
                app(
                    cst("DensityMatrix"),
                    app2(cst("Nat.mul"), nat_ty(), nat_ty()),
                ),
            ),
        ),
    )
}
/// `PPTCriterion : DensityMatrix (m*n) → Prop`
/// Peres-Horodecki: ρ separable ⟹ ρ^{T_B} ≥ 0.
pub fn ppt_criterion_ty() -> Expr {
    arrow(app(cst("DensityMatrix"), nat_ty()), prop())
}
/// `EntanglementOfFormation : DensityMatrix 4 → Real`
/// E_F(ρ) = min_{p_i,ψ_i} ∑ p_i S(Tr_A |ψ_i⟩⟨ψ_i|).
pub fn entanglement_of_formation_ty() -> Expr {
    arrow(app(cst("DensityMatrix"), cst("Nat.four")), real_ty())
}
/// PPT is necessary for separability (Peres-Horodecki theorem).
pub fn ppt_necessary_ty() -> Expr {
    prop()
}
/// For 2×2 and 2×3 systems, PPT is also sufficient (Horodecki theorem).
pub fn ppt_sufficient_low_dim_ty() -> Expr {
    prop()
}
/// `StabilizerCode : Nat → Nat → Nat → Type`
/// [[n, k, d]] stabilizer code with n physical, k logical qubits and distance d.
pub fn stabilizer_code_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), type0())))
}
/// `CSSCode : Nat → Nat → Type`
/// Calderbank-Shor-Steane code built from two classical linear codes C₁ ⊇ C₂.
pub fn css_code_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `SurfaceCode : Nat → Type`
/// Topological surface code on an L×L lattice with [[L²+(L-1)², 1, L]] parameters.
pub fn surface_code_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `SyndromeDecoder : Nat → Nat → Nat → Type`
/// Minimum-weight matching decoder for an [[n,k,d]] stabilizer code.
pub fn syndrome_decoder_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), type0())))
}
/// `StabilizerDistance : Nat → Nat → Nat → Nat`
/// Code distance d = min weight of a non-trivial logical operator.
pub fn stabilizer_distance_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), nat_ty())))
}
/// Quantum Singleton bound: [[n, k, d]] code requires n − k ≥ 2(d − 1).
pub fn quantum_singleton_bound_ty() -> Expr {
    prop()
}
/// Knill-Laflamme conditions: necessary and sufficient conditions for error
/// correction by a quantum code.
pub fn knill_laflamme_ty() -> Expr {
    prop()
}
/// `BQPComplexity : Type`
/// Class of decision problems solvable by a uniform quantum circuit family in
/// polynomial time with bounded error ≤ 1/3.
pub fn bqp_complexity_ty() -> Expr {
    type0()
}
/// `QMAComplexity : Type`
/// Quantum Merlin-Arthur: problems verifiable in BQP with a quantum witness.
pub fn qma_complexity_ty() -> Expr {
    type0()
}
/// `QuantumCircuitComplexity : Nat → Type`
/// Circuit complexity of an n-qubit unitary U.
pub fn quantum_circuit_complexity_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `TGateCount : Nat → Nat → Nat`
/// Number of T (π/8) gates in an n-qubit circuit with m total gates.
pub fn t_gate_count_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), nat_ty()))
}
/// `CliffordPlusTDecomposition : Type`
/// Decomposition of a unitary into Clifford + T gates (Solovay-Kitaev).
pub fn clifford_plus_t_ty() -> Expr {
    type0()
}
/// BQP ⊆ PSPACE.
pub fn bqp_in_pspace_ty() -> Expr {
    prop()
}
/// P ⊆ BQP.
pub fn p_in_bqp_ty() -> Expr {
    prop()
}
/// Solovay-Kitaev theorem: any single-qubit unitary can be approximated to
/// precision ε using O(log^c(1/ε)) gates from the Clifford+T gate set.
pub fn solovay_kitaev_ty() -> Expr {
    prop()
}
/// Populate an `Environment` with all quantum-information axioms.
pub fn build_quantum_information_env(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        ("DensityMatrix", density_matrix_ty()),
        ("PureState", pure_state_ty()),
        ("MixedState", mixed_state_ty()),
        ("BlochVector", bloch_vector_ty()),
        ("VonNeumannEntropy", von_neumann_entropy_ty()),
        ("Purity", purity_ty()),
        ("EntropyZeroIffPure", entropy_zero_iff_pure_ty()),
        ("PurityOneIffPure", purity_one_iff_pure_ty()),
        ("Nat.four", nat_ty()),
        ("Nat.mul", arrow(nat_ty(), arrow(nat_ty(), nat_ty()))),
        ("QuantumChannel", quantum_channel_ty()),
        ("KrausRepresentation", kraus_representation_ty()),
        ("ChoiMatrix", choi_matrix_ty()),
        ("DepolarizingChannel", depolarizing_channel_ty()),
        ("DiamondNorm", diamond_norm_ty()),
        ("ChoiKrausIsomorphism", choi_kraus_isomorphism_ty()),
        ("StinespringDilation", stinespring_dilation_ty()),
        ("EntanglementMeasure", entanglement_measure_ty()),
        ("Concurrence", concurrence_ty()),
        ("PartialTranspose", partial_transpose_ty()),
        ("PPTCriterion", ppt_criterion_ty()),
        ("EntanglementOfFormation", entanglement_of_formation_ty()),
        ("PPTNecessary", ppt_necessary_ty()),
        ("PPTSufficientLowDim", ppt_sufficient_low_dim_ty()),
        ("StabilizerCode", stabilizer_code_ty()),
        ("CSSCode", css_code_ty()),
        ("SurfaceCode", surface_code_ty()),
        ("SyndromeDecoder", syndrome_decoder_ty()),
        ("StabilizerDistance", stabilizer_distance_ty()),
        ("QuantumSingletonBound", quantum_singleton_bound_ty()),
        ("KnillLaflamme", knill_laflamme_ty()),
        ("BQPComplexity", bqp_complexity_ty()),
        ("QMAComplexity", qma_complexity_ty()),
        ("QuantumCircuitComplexity", quantum_circuit_complexity_ty()),
        ("TGateCount", t_gate_count_ty()),
        ("CliffordPlusT", clifford_plus_t_ty()),
        ("BQPInPSPACE", bqp_in_pspace_ty()),
        ("PInBQP", p_in_bqp_ty()),
        ("SolovayKitaev", solovay_kitaev_ty()),
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
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_density_matrix_pure() {
        let psi = PureState::zero_state();
        let rho = DensityMatrix::from_pure_state(&psi);
        assert!(rho.is_pure());
        assert!((rho.purity() - 1.0).abs() < 1e-9);
    }
    #[test]
    fn test_density_matrix_mixed() {
        let rho = DensityMatrix::maximally_mixed(2);
        assert!(!rho.is_pure());
        assert!((rho.purity() - 0.5).abs() < 1e-9);
    }
    #[test]
    fn test_von_neumann_entropy() {
        let psi = PureState::zero_state();
        let rho_pure = DensityMatrix::from_pure_state(&psi);
        assert!(rho_pure.von_neumann_entropy() < 1e-9);
        let rho_mix = DensityMatrix::maximally_mixed(2);
        assert!((rho_mix.von_neumann_entropy() - 1.0).abs() < 1e-9);
    }
    #[test]
    fn test_bloch_vector() {
        let psi = PureState::zero_state();
        let rho = DensityMatrix::from_pure_state(&psi);
        let bv = BlochVector::from_density_matrix(&rho);
        assert!((bv.z - 1.0).abs() < 1e-9);
        assert!(bv.x.abs() < 1e-9);
        assert!(bv.y.abs() < 1e-9);
        assert!(bv.is_valid());
        let rho_mix = DensityMatrix::maximally_mixed(2);
        let bv_mix = BlochVector::from_density_matrix(&rho_mix);
        assert!(bv_mix.x.abs() < 1e-9);
        assert!(bv_mix.y.abs() < 1e-9);
        assert!(bv_mix.z.abs() < 1e-9);
    }
    #[test]
    fn test_depolarizing_channel() {
        let ch = KrausChannel::depolarizing(2, 0.0);
        let psi = PureState::zero_state();
        let rho = DensityMatrix::from_pure_state(&psi);
        let out = ch.apply(&rho);
        assert!((out.get(0, 0).re - 1.0).abs() < 1e-9);
        assert!(out.get(1, 1).re.abs() < 1e-9);
    }
    #[test]
    fn test_channel_unitary() {
        let eye: Vec<Complex> = vec![
            Complex::one(),
            Complex::zero(),
            Complex::zero(),
            Complex::one(),
        ];
        let ch = KrausChannel::new(2, 2, vec![eye]);
        assert!(ch.is_unitary());
        let dep = KrausChannel::depolarizing(2, 0.1);
        assert!(!dep.is_unitary());
    }
    #[test]
    fn test_partial_transpose_separable() {
        let psi = PureState::basis(2, 0);
        let rho_a = DensityMatrix::from_pure_state(&psi);
        let rho_b = DensityMatrix::maximally_mixed(2);
        let mut data = vec![Complex::zero(); 16];
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    for l in 0..2 {
                        let idx = (i * 2 + k) * 4 + (j * 2 + l);
                        data[idx] = rho_a.get(i, j).mul(rho_b.get(k, l));
                    }
                }
            }
        }
        let rho_prod = DensityMatrix { dim: 4, data };
        assert!(rho_prod.is_ppt(2, 2));
    }
    #[test]
    fn test_stabilizer_code_steane() {
        let code = StabilizerCode::steane_code();
        assert_eq!(code.n, 7);
        assert_eq!(code.k, 1);
        assert_eq!(code.d, 3);
        let x_err = vec![0u8; 7];
        let z_err = vec![0u8; 7];
        let synd = code.syndrome(&x_err, &z_err);
        assert!(synd.iter().all(|&s| s == 0));
        let mut x_err1 = vec![0u8; 7];
        x_err1[0] = 1;
        assert!(code.detect_errors(&x_err1, &[0u8; 7]));
        assert!(code.satisfies_singleton_bound());
    }
    #[test]
    fn test_stabilizer_decoder() {
        let code = StabilizerCode::steane_code();
        let decoder = SyndromeDecoder::new(code);
        let mut x_err = vec![0u8; 7];
        x_err[1] = 1;
        let z_err = vec![0u8; 7];
        let synd = decoder.code.syndrome(&x_err, &z_err);
        let (rec_x, _rec_z) = decoder.decode(&synd);
        assert_eq!(rec_x, x_err);
    }
    #[test]
    fn test_surface_code() {
        let sc = SurfaceCode::new(3);
        assert_eq!(sc.distance(), 3);
        assert_eq!(sc.num_logical_qubits(), 1);
    }
    #[test]
    fn test_quantum_circuit_complexity() {
        let mut circ = QuantumCircuit::new(2);
        circ.add_gate(GateType::H, vec![0]);
        circ.add_gate(GateType::T, vec![1]);
        circ.add_gate(GateType::Cnot, vec![0, 1]);
        circ.add_gate(GateType::T, vec![0]);
        assert_eq!(circ.gate_count(), 4);
        assert_eq!(circ.t_gate_count(), 2);
        assert_eq!(circ.clifford_count(), 2);
    }
    #[test]
    fn test_bqp_qma() {
        assert!(BQPComplexity::factoring_in_bqp());
        assert!(BQPComplexity::search_in_bqp());
        assert!(QMAComplexity::local_hamiltonian_is_qma_complete());
        let qma = QMAComplexity::standard();
        assert!(qma.completeness > qma.soundness);
    }
    #[test]
    fn test_build_quantum_information_env() {
        let mut env = oxilean_kernel::Environment::new();
        let result = build_quantum_information_env(&mut env);
        assert!(
            result.is_ok(),
            "build_quantum_information_env failed: {:?}",
            result.err()
        );
    }
}
/// Entanglement measures comparison.
#[allow(dead_code)]
pub fn entanglement_measures_comparison() -> Vec<(&'static str, &'static str, bool)> {
    vec![
        (
            "Entanglement entropy",
            "S(rho_A) = -Tr(rho_A log rho_A) for pure states",
            false,
        ),
        ("Entanglement of formation", "E_F = min convex hull S", true),
        (
            "Concurrence",
            "C = max(0, lambda1-lambda2-lambda3-lambda4) for 2 qubits",
            true,
        ),
        ("Squashed entanglement", "Esq = inf over extensions", false),
        (
            "Entanglement cost",
            "Rate for preparing rho from singlets",
            false,
        ),
        (
            "Distillable entanglement",
            "Rate for extracting singlets from rho",
            false,
        ),
        (
            "Relative entropy of entanglement",
            "min over sep states D(rho||sigma)",
            true,
        ),
        ("Negativity", "N = (||rho^T_A||_1 - 1)/2", true),
        (
            "Geometric measure",
            "E_G = -log max|<psi|phi_prod>|^2",
            false,
        ),
        (
            "Robustness of entanglement",
            "min s: (rho+s*sigma)/(1+s) sep",
            true,
        ),
    ]
}
#[cfg(test)]
mod qi_ext_tests {
    use super::*;
    #[test]
    fn test_state_discrimination() {
        let p_e = StateDiscrimination::helstrom_bound_two_states(0.5, 0.5);
        assert!((p_e - 0.5).abs() < 1e-10);
    }
    #[test]
    fn test_quantum_discord() {
        let qd = QuantumDiscord::new("Bell state", 1.0, 0.0);
        assert!((qd.discord_value() - 1.0).abs() < 1e-10);
        assert!(!qd.is_zero_discord());
    }
    #[test]
    fn test_cv_gaussian() {
        let cv = CvQuantumInfo::new(1, CvStateType::Coherent);
        assert!(cv.is_gaussian());
        assert!(cv.wigner_function_nonnegative());
    }
    #[test]
    fn test_qkd_protocols() {
        let bb84 = QkdProtocol::bb84();
        assert!(!bb84.uses_entanglement);
        assert!(bb84.is_unconditionally_secure());
        let e91 = QkdProtocol::e91();
        assert!(e91.uses_entanglement);
    }
    #[test]
    fn test_error_mitigation() {
        let zne = QuantumErrorMitigation::zne(3.0);
        assert!(zne.is_exact_in_limit());
    }
    #[test]
    fn test_entanglement_measures_nonempty() {
        let measures = entanglement_measures_comparison();
        assert!(!measures.is_empty());
    }
}
#[cfg(test)]
mod qi_comm_tests {
    use super::*;
    #[test]
    fn test_bell_chsh() {
        let chsh = BellInequality::chsh();
        assert!((chsh.classical_bound - 2.0).abs() < 1e-10);
        assert!((chsh.quantum_bound - 2.0 * 2.0_f64.sqrt()).abs() < 1e-10);
        assert!(chsh.quantum_violation_ratio() > 1.0);
    }
    #[test]
    fn test_qcc_equality() {
        let eq = QccProtocol::equality_function();
        assert!(eq.quantum_advantage_factor() > 1.0);
        assert!(eq.has_exponential_gap);
    }
}
#[cfg(test)]
mod resource_theory_tests {
    use super::*;
    #[test]
    fn test_resource_theories() {
        let ent = ResourceTheory::entanglement();
        assert!(!ent.free_states.is_empty());
        let coh = ResourceTheory::coherence();
        assert!(!coh.monotone.is_empty());
        let magic = ResourceTheory::magic_states();
        assert!(!magic.asymptotic_rate_description().is_empty());
    }
}
#[cfg(test)]
mod holographic_qec_tests {
    use super::*;
    #[test]
    fn test_holographic_code() {
        let hc = HolographicCode::happy_code(2);
        assert!(hc.encoding_rate() > 0.0);
        assert!(hc.is_isometric());
        assert!(!hc.ryu_takayanagi_formula().is_empty());
    }
}
