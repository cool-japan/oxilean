//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    AnyonFusionTree, AnyonModel, BraidWord, BraidWordNew, Complex, FibAnyon, FibFMatrix,
    FibRMatrix, FibonacciAnyonBraiding, FibonacciBraidGates, KitaevChain, ModularTensorCategory,
    ModularTensorCategoryComputer, PentagonEquationChecker, QuantumDoubleModel, SModularMatrix,
    SurfaceCode, SurfaceCodeQEC, ToricCodeAnyon, ToricCodeKitaev, ToricCodeQEC,
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
/// `AnyonType : Type` — an anyon species (topological charge label).
pub fn anyon_type_ty() -> Expr {
    type0()
}
/// `FusionChannel : AnyonType → AnyonType → AnyonType → Type`
/// `FusionChannel a b c` witnesses that anyons `a` and `b` can fuse to `c`.
pub fn fusion_channel_ty() -> Expr {
    arrow(
        cst("AnyonType"),
        arrow(cst("AnyonType"), arrow(cst("AnyonType"), type0())),
    )
}
/// `FusionMultiplicity : AnyonType → AnyonType → AnyonType → Nat`
/// The fusion multiplicity `N_{ab}^c` counting fusion channels.
pub fn fusion_multiplicity_ty() -> Expr {
    arrow(
        cst("AnyonType"),
        arrow(cst("AnyonType"), arrow(cst("AnyonType"), nat_ty())),
    )
}
/// `TopologicalSpin : AnyonType → Real`
/// The topological spin `θ_a = e^{2πi h_a}` of anyon `a`.
pub fn topological_spin_ty() -> Expr {
    arrow(cst("AnyonType"), real_ty())
}
/// `VacuumCharge : AnyonType` — the vacuum (trivial) topological charge.
pub fn vacuum_charge_ty() -> Expr {
    cst("AnyonType")
}
/// `AntiAnyon : AnyonType → AnyonType`
/// The antiparticle (dual charge) `ā` of anyon `a`.
pub fn anti_anyon_ty() -> Expr {
    arrow(cst("AnyonType"), cst("AnyonType"))
}
/// Theorem: `FusionAssociativity`
/// Fusion is associative up to F-matrices: (a⊗b)⊗c ≅ a⊗(b⊗c).
pub fn fusion_associativity_ty() -> Expr {
    prop()
}
/// Theorem: `FusionCommutativity`
/// For abelian anyons, `a⊗b = b⊗a`.
pub fn fusion_commutativity_ty() -> Expr {
    prop()
}
/// Theorem: `VacuumUnit`
/// The vacuum is a unit for fusion: `a ⊗ 1 = a`.
pub fn vacuum_unit_ty() -> Expr {
    prop()
}
/// `BraidGroup : Nat → Type`
/// `BraidGroup n` is the braid group on `n` strands.
pub fn braid_group_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `BraidElement : Nat → Type`
/// An element of the braid group B_n (a braid word).
pub fn braid_element_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `RMatrix : AnyonType → AnyonType → AnyonType → Real → Real → Type`
/// The R-matrix encoding braiding statistics; R^{ab}_c = e^{iπ(h_c - h_a - h_b)}.
pub fn r_matrix_ty() -> Expr {
    arrow(
        cst("AnyonType"),
        arrow(
            cst("AnyonType"),
            arrow(
                cst("AnyonType"),
                arrow(real_ty(), arrow(real_ty(), type0())),
            ),
        ),
    )
}
/// `FMatrix : AnyonType → AnyonType → AnyonType → AnyonType → Type`
/// The F-matrix (6j-symbol) implementing associativity morphisms.
pub fn f_matrix_ty() -> Expr {
    arrow(
        cst("AnyonType"),
        arrow(
            cst("AnyonType"),
            arrow(cst("AnyonType"), arrow(cst("AnyonType"), type0())),
        ),
    )
}
/// `ExchangePhase : AnyonType → Real`
/// Statistical phase θ acquired when one anyon is exchanged with another.
pub fn exchange_phase_ty() -> Expr {
    arrow(cst("AnyonType"), real_ty())
}
/// Theorem: `BraidRelation`
/// σ_i σ_{i+1} σ_i = σ_{i+1} σ_i σ_{i+1} (Artin braid relation).
pub fn braid_relation_ty() -> Expr {
    prop()
}
/// Theorem: `PentagonEquation`
/// The F-matrices satisfy the pentagon equation (MacLane coherence).
pub fn pentagon_equation_ty() -> Expr {
    prop()
}
/// Theorem: `HexagonEquation`
/// The F- and R-matrices together satisfy the hexagon equation.
pub fn hexagon_equation_ty() -> Expr {
    prop()
}
/// Theorem: `AbelianBraidingIsPhase`
/// For abelian anyons the full braid is a U(1) phase.
pub fn abelian_braiding_is_phase_ty() -> Expr {
    prop()
}
/// `ToricCode : Nat → Type`
/// Kitaev's toric code on an L×L torus.
pub fn toric_code_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `SurfaceCode : Nat → Type`
/// Surface code (planar variant of toric code) with distance d.
pub fn surface_code_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `TopologicalOrder : Type`
/// Abstract topological phase of matter (ground-state degeneracy on torus).
pub fn topological_order_ty() -> Expr {
    type0()
}
/// `GroundStateDegeneracy : ToricCode L → Nat`
/// The GSD of the toric code on a torus: GSD = 4.
pub fn ground_state_degeneracy_ty() -> Expr {
    arrow(app(cst("ToricCode"), nat_ty()), nat_ty())
}
/// `AnyonExcitation : ToricCode L → AnyonType`
/// The four anyon types {1, e, m, ε} of the toric code.
pub fn anyon_excitation_ty() -> Expr {
    arrow(app(cst("ToricCode"), nat_ty()), cst("AnyonType"))
}
/// `LogicalQubit : Nat → Type`
/// A logical qubit encoded in a topological code of distance d.
pub fn logical_qubit_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// Theorem: `ToricCodeGSDFour`
/// The toric code on a torus has ground-state degeneracy exactly 4.
pub fn toric_code_gsd_four_ty() -> Expr {
    prop()
}
/// Theorem: `TopologicalProtection`
/// Local perturbations cannot distinguish degenerate ground states.
pub fn topological_protection_ty() -> Expr {
    prop()
}
/// Theorem: `SurfaceCodeDistance`
/// Surface code of size d has code distance d.
pub fn surface_code_distance_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "d",
        nat_ty(),
        app2(cst("Nat.eq"), app(cst("SurfaceCodeDist"), bvar(0)), bvar(0)),
    )
}
/// `FibonacciAnyon : Type`
/// Fibonacci anyons {1, τ} with τ⊗τ = 1 ⊕ τ.
pub fn fibonacci_anyon_ty() -> Expr {
    type0()
}
/// `GoldenRatio : Real`
/// φ = (1 + √5)/2 ≈ 1.618..., the quantum dimension of τ.
pub fn golden_ratio_ty() -> Expr {
    real_ty()
}
/// `FibFMatrix : Real`
/// The F-matrix for Fibonacci anyons (2×2 real symmetric matrix).
pub fn fib_f_matrix_ty() -> Expr {
    real_ty()
}
/// `FibBraidMatrix : Real`
/// The braiding matrix for Fibonacci anyons.
pub fn fib_braid_matrix_ty() -> Expr {
    real_ty()
}
/// `QuantumDimension : AnyonType → Real`
/// d_a = quantum dimension satisfying d_a d_b = ∑_c N_{ab}^c d_c.
pub fn quantum_dimension_ty() -> Expr {
    arrow(cst("AnyonType"), real_ty())
}
/// `TotalQuantumDimension : Type → Real`
/// D² = ∑_a d_a² (total quantum dimension squared).
pub fn total_quantum_dimension_ty() -> Expr {
    arrow(type0(), real_ty())
}
/// Theorem: `FibonacciUniversal`
/// Fibonacci anyons are universal for quantum computation via braiding alone.
pub fn fibonacci_universal_ty() -> Expr {
    prop()
}
/// Theorem: `GoldenRatioQuantumDim`
/// The quantum dimension of τ is the golden ratio φ.
pub fn golden_ratio_quantum_dim_ty() -> Expr {
    prop()
}
/// Theorem: `FibFusionRule`
/// τ ⊗ τ = 1 ⊕ τ (Fibonacci fusion rule).
pub fn fib_fusion_rule_ty() -> Expr {
    prop()
}
/// `YangBaxterSolution : Type → Type`
/// A solution R ∈ End(V⊗V) to the Yang-Baxter equation.
pub fn yang_baxter_solution_ty() -> Expr {
    arrow(type0(), type0())
}
/// `QuantumGroup : Type`
/// A Hopf algebra deformation of a classical Lie group.
pub fn quantum_group_ty() -> Expr {
    type0()
}
/// `BraidedCategory : Type`
/// A braided monoidal category with natural braiding isomorphisms.
pub fn braided_category_ty() -> Expr {
    type0()
}
/// Theorem: `YangBaxterEquation`
/// R_{12} R_{13} R_{23} = R_{23} R_{13} R_{12}.
pub fn yang_baxter_equation_ty() -> Expr {
    prop()
}
/// Theorem: `BraidGroupRepresentation`
/// Every solution to YBE gives a representation of the braid group.
pub fn braid_group_representation_ty() -> Expr {
    prop()
}
/// Theorem: `QuantumGroupDuality`
/// Every quantum group has a dual quantum group.
pub fn quantum_group_duality_ty() -> Expr {
    prop()
}
/// `ModularTensorCategory : Type`
/// An MTC is a ribbon fusion category with non-degenerate S-matrix.
pub fn modular_tensor_category_ty() -> Expr {
    type0()
}
/// `SMatrix : Type → Type`
/// The modular S-matrix S_{ab} = (1/D) ∑_c N_{ab}^{\bar{c}} d_c θ_c/(θ_a θ_b).
pub fn s_matrix_ty() -> Expr {
    arrow(type0(), type0())
}
/// `TMatrix : Type → Type`
/// The diagonal T-matrix T_{aa} = θ_a (topological spin).
pub fn t_matrix_ty() -> Expr {
    arrow(type0(), type0())
}
/// `VerlindeFormula : Type`
/// N_{ab}^c = ∑_x S_{ax} S_{bx} S_{cx}^* / S_{1x} (Verlinde formula).
pub fn verlinde_formula_ty() -> Expr {
    type0()
}
/// `RibbonElement : Type → Type`
/// The ribbon element of a ribbon Hopf algebra.
pub fn ribbon_element_ty() -> Expr {
    arrow(type0(), type0())
}
/// Theorem: `SMatrixUnitary`
/// The modular S-matrix is unitary: S S† = I.
pub fn s_matrix_unitary_ty() -> Expr {
    prop()
}
/// Theorem: `STRelation`
/// (ST)³ = S² = C (charge conjugation) in the modular group SL(2,ℤ).
pub fn st_relation_ty() -> Expr {
    prop()
}
/// Theorem: `VerlindeFromSMatrix`
/// Fusion multiplicities are determined by the Verlinde formula from S.
pub fn verlinde_from_s_matrix_ty() -> Expr {
    prop()
}
/// Theorem: `FrobeniusSchurzIndicator`
/// ν_n(a) = (1/D²) ∑_{b,c} N_{aa}^c d_b d_c (θ_c/θ_a)^n.
pub fn frobenius_schur_indicator_ty() -> Expr {
    prop()
}
/// `ChernSimonsAction : Nat → Real → Type`
/// CS action S = (k/4π) ∫ Tr(A∧dA + (2/3)A∧A∧A) at level k.
pub fn chern_simons_action_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), type0()))
}
/// `WilsonLoop : AnyonType → Type`
/// A Wilson loop operator W_R(γ) = Tr_R P exp(∮ A).
pub fn wilson_loop_ty() -> Expr {
    arrow(cst("AnyonType"), type0())
}
/// `KnotInvariant : Type`
/// A topological invariant of knots/links from CS theory.
pub fn knot_invariant_ty() -> Expr {
    type0()
}
/// `JonesPolynomial : Type`
/// The Jones polynomial V(t) as a Laurent polynomial.
pub fn jones_polynomial_ty() -> Expr {
    type0()
}
/// `HilbertSpaceCS : Nat → Real → Nat → Nat → Type`
/// Hilbert space of CS theory on Σ_g at level k with gauge group of rank r.
pub fn hilbert_space_cs_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(real_ty(), arrow(nat_ty(), arrow(nat_ty(), type0()))),
    )
}
/// Theorem: `WilsonLoopCorrelator`
/// Wilson loop correlators in CS theory compute knot invariants.
pub fn wilson_loop_correlator_ty() -> Expr {
    prop()
}
/// Theorem: `CSPartitionFunction`
/// The CS partition function on S³ equals 1/D (inverse total quantum dimension).
pub fn cs_partition_function_ty() -> Expr {
    prop()
}
/// Theorem: `CSLevelQuantization`
/// The CS level k must be an integer for gauge invariance.
pub fn cs_level_quantization_ty() -> Expr {
    prop()
}
/// `TQFT : Nat → Type`
/// An n-dimensional topological quantum field theory (Atiyah axioms).
pub fn tqft_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `Cobordism : Type → Type → Type`
/// A cobordism M between manifolds Σ₁ and Σ₂.
pub fn cobordism_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// `StateSpace : TQFT n → Type → Type`
/// The Hilbert space Z(Σ) assigned to a closed (n-1)-manifold Σ.
pub fn state_space_ty() -> Expr {
    arrow(app(cst("TQFT"), nat_ty()), arrow(type0(), type0()))
}
/// `AmplitudeTQFT : TQFT n → Cobordism Σ₁ Σ₂ → Type`
/// The linear map Z(M) : Z(Σ₁) → Z(Σ₂) assigned to a cobordism M.
pub fn amplitude_tqft_ty() -> Expr {
    arrow(
        app(cst("TQFT"), nat_ty()),
        arrow(app2(cst("Cobordism"), type0(), type0()), type0()),
    )
}
/// Theorem: `TQFTFunctoriality`
/// Z(M₁ ∘ M₂) = Z(M₁) ∘ Z(M₂) (composition law).
pub fn tqft_functoriality_ty() -> Expr {
    prop()
}
/// Theorem: `TQFTInvolutivity`
/// Z(Σ̄) = Z(Σ)* (orientation reversal gives conjugate space).
pub fn tqft_involutivity_ty() -> Expr {
    prop()
}
/// Theorem: `AtiyahAxioms`
/// A TQFT is a symmetric monoidal functor from cobordisms to vector spaces.
pub fn atiyah_axioms_ty() -> Expr {
    prop()
}
/// Theorem: `CSIsaTQFT`
/// Chern-Simons theory defines a 3-dimensional TQFT.
pub fn cs_is_tqft_ty() -> Expr {
    prop()
}
/// `IsingAnyon : Type`
/// Ising anyons {1, σ, ψ} realised by Majorana fermions in topological superconductors.
pub fn ising_anyon_ty() -> Expr {
    type0()
}
/// `MajoranaMode : Type`
/// A Majorana zero mode γ = γ† satisfying {γ_i, γ_j} = 2δ_{ij}.
pub fn majorana_mode_ty() -> Expr {
    type0()
}
/// `MajoranaOperator : Nat → Type`
/// The Majorana operator γ_i for site i.
pub fn majorana_operator_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `IsingFusionRule : IsingAnyon → IsingAnyon → IsingAnyon → Prop`
/// σ ⊗ σ = 1 ⊕ ψ, σ ⊗ ψ = σ, ψ ⊗ ψ = 1.
pub fn ising_fusion_rule_ty() -> Expr {
    arrow(
        cst("IsingAnyon"),
        arrow(cst("IsingAnyon"), arrow(cst("IsingAnyon"), prop())),
    )
}
/// `NonAbelianStatistics : Type → Prop`
/// Predicate: the anyon type has non-Abelian braiding statistics.
pub fn non_abelian_statistics_ty() -> Expr {
    arrow(type0(), prop())
}
/// Theorem: `IsingNonAbelian`
/// Ising σ anyons have non-Abelian braiding statistics.
pub fn ising_non_abelian_ty() -> Expr {
    prop()
}
/// Theorem: `MajoranaAnticommutation`
/// Majorana operators satisfy {γ_i, γ_j} = 2δ_{ij}.
pub fn majorana_anticommutation_ty() -> Expr {
    prop()
}
/// Theorem: `MajoranaHermitian`
/// Majorana operators are self-conjugate: γ = γ†.
pub fn majorana_hermitian_ty() -> Expr {
    prop()
}
/// `QuantumDouble : Type → Type`
/// Kitaev's quantum double D(G) of a finite group G.
pub fn quantum_double_ty() -> Expr {
    arrow(type0(), type0())
}
/// `AnyonLabel : Type → Type`
/// Anyon labels of D(G): conjugacy class C and irrep R of centralizer Z(g).
pub fn anyon_label_ty() -> Expr {
    arrow(type0(), type0())
}
/// `ConjugacyClass : Type → Type`
/// A conjugacy class of a finite group G.
pub fn conjugacy_class_ty() -> Expr {
    arrow(type0(), type0())
}
/// `QuantumDoubleS : Type → Type`
/// The S-matrix of the quantum double D(G).
pub fn quantum_double_s_ty() -> Expr {
    arrow(type0(), type0())
}
/// `ToricCodeIsQuantumDouble : Prop`
/// Toric code anyons = quantum double D(ℤ₂) anyons.
pub fn toric_code_is_quantum_double_ty() -> Expr {
    prop()
}
/// Theorem: `QuantumDoubleModular`
/// D(G) is a modular tensor category for any finite group G.
pub fn quantum_double_modular_ty() -> Expr {
    prop()
}
/// Theorem: `QuantumDoubleFusionAbelian`
/// In D(ℤ_n) fusion is abelian (group addition).
pub fn quantum_double_fusion_abelian_ty() -> Expr {
    prop()
}
/// `RibbonCategory : Type`
/// A ribbon category: a braided monoidal category with a twist (ribbon element).
pub fn ribbon_category_ty() -> Expr {
    type0()
}
/// `TwistIsomorphism : Type → Type → Type`
/// The twist θ_V : V → V (a natural automorphism).
pub fn twist_isomorphism_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// `BalancingIsomorphism : Type → Type`
/// Balancing: θ_{V⊗W} = (θ_V ⊗ θ_W) ∘ c_{W,V} ∘ c_{V,W}.
pub fn balancing_isomorphism_ty() -> Expr {
    arrow(type0(), type0())
}
/// `RibbonElementHopf : Type → Type`
/// The ribbon element v of a ribbon Hopf algebra: v = u S(u) where u is the Drinfeld element.
pub fn ribbon_element_hopf_ty() -> Expr {
    arrow(type0(), type0())
}
/// Theorem: `TwistAndBraidConsistency`
/// Twist and braiding satisfy θ_{V⊗W} = c_{W,V} ∘ c_{V,W} ∘ (θ_V ⊗ θ_W).
pub fn twist_and_braid_consistency_ty() -> Expr {
    prop()
}
/// Theorem: `RibbonImpliesSphericalTrace`
/// Every ribbon category has a spherical (pivotal) trace.
pub fn ribbon_implies_spherical_trace_ty() -> Expr {
    prop()
}
/// `PentagonEquationFull : AnyonType → AnyonType → AnyonType → AnyonType → AnyonType → Prop`
/// The full pentagon equation for F-matrices over five anyon labels.
pub fn pentagon_equation_full_ty() -> Expr {
    arrow(
        cst("AnyonType"),
        arrow(
            cst("AnyonType"),
            arrow(
                cst("AnyonType"),
                arrow(cst("AnyonType"), arrow(cst("AnyonType"), prop())),
            ),
        ),
    )
}
/// `HexagonEquationFull : AnyonType → AnyonType → AnyonType → Prop`
/// The full hexagon equation relating F and R matrices.
pub fn hexagon_equation_full_ty() -> Expr {
    arrow(
        cst("AnyonType"),
        arrow(cst("AnyonType"), arrow(cst("AnyonType"), prop())),
    )
}
/// `FrobeniusPerronDim : AnyonType → Real`
/// The Frobenius-Perron dimension d_a (largest real eigenvalue of fusion matrix N_a).
pub fn frobenius_perron_dim_ty() -> Expr {
    arrow(cst("AnyonType"), real_ty())
}
/// `FusionMatrix : AnyonType → Type`
/// The fusion matrix (N_a)_{bc} = N_{ab}^c for fixed a.
pub fn fusion_matrix_ty() -> Expr {
    arrow(cst("AnyonType"), type0())
}
/// Theorem: `FrobeniusPerronPositive`
/// Frobenius-Perron dimensions are strictly positive real numbers.
pub fn frobenius_perron_positive_ty() -> Expr {
    prop()
}
/// Theorem: `QuantumDimEqualsFP`
/// In a unitary fusion category the quantum dim equals the Frobenius-Perron dim.
pub fn quantum_dim_equals_fp_ty() -> Expr {
    prop()
}
/// `TopologicalGate : Type`
/// A quantum gate implemented by topological (braiding) operations.
pub fn topological_gate_ty() -> Expr {
    type0()
}
/// `BraidingGate : AnyonType → AnyonType → Type`
/// A gate realised by braiding anyon a around anyon b.
pub fn braiding_gate_ty() -> Expr {
    arrow(cst("AnyonType"), arrow(cst("AnyonType"), type0()))
}
/// `TopologicalProtectionGate : TopologicalGate → Prop`
/// A gate is topologically protected if it is immune to local perturbations.
pub fn topological_protection_gate_ty() -> Expr {
    arrow(cst("TopologicalGate"), prop())
}
/// `UniversalGateSet : Type`
/// A gate set that is universal for quantum computation.
pub fn universal_gate_set_ty() -> Expr {
    type0()
}
/// `GateApproxError : TopologicalGate → Real → Prop`
/// The approximation error of a topological gate is ε.
pub fn gate_approx_error_ty() -> Expr {
    arrow(cst("TopologicalGate"), arrow(real_ty(), prop()))
}
/// Theorem: `FibonacciBraidingUniversal`
/// The braiding gates of Fibonacci anyons form a universal gate set.
pub fn fibonacci_braiding_universal_ty() -> Expr {
    prop()
}
/// Theorem: `IsingBraidingNotUniversal`
/// Ising anyon braiding alone is NOT universal (requires extra magic states).
pub fn ising_braiding_not_universal_ty() -> Expr {
    prop()
}
/// `QuantumSpinLiquid : Type`
/// A quantum spin liquid: frustrated magnet with long-range entanglement.
pub fn quantum_spin_liquid_ty() -> Expr {
    type0()
}
/// `RVBState : Nat → Type`
/// Anderson's resonating valence bond state on a lattice of size n.
pub fn rvb_state_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `FrustrationIndex : Type → Nat`
/// The frustration index of a spin Hamiltonian (number of unsatisfied bonds).
pub fn frustration_index_ty() -> Expr {
    arrow(type0(), nat_ty())
}
/// `TopologicalEntanglementEntropy : Type → Real`
/// S_topo = γ (topological contribution to entanglement entropy).
pub fn topological_entanglement_entropy_ty() -> Expr {
    arrow(type0(), real_ty())
}
/// Theorem: `KitaevLevinWenEntropy`
/// For a topologically ordered system S = αL - γ where γ = log D.
pub fn kitaev_levin_wen_entropy_ty() -> Expr {
    prop()
}
/// Theorem: `TopologicalOrderFromFrustration`
/// Geometric frustration can stabilize topological order (spin liquid phase).
pub fn topological_order_from_frustration_ty() -> Expr {
    prop()
}
/// `StringNet : Type → Type`
/// Levin-Wen string-net model for a fusion category C.
pub fn string_net_ty() -> Expr {
    arrow(type0(), type0())
}
/// `LevinWenHamiltonian : Type → Type`
/// The Levin-Wen plaquette Hamiltonian realising string-net condensation.
pub fn levin_wen_hamiltonian_ty() -> Expr {
    arrow(type0(), type0())
}
/// `TuraevViroTQFT : Type → Nat → Type`
/// Turaev-Viro TQFT associated to a spherical fusion category at level k.
pub fn turaev_viro_tqft_ty() -> Expr {
    arrow(type0(), arrow(nat_ty(), type0()))
}
/// `StringNetCondensation : Type → Prop`
/// Predicate: the string-net ground state exhibits condensation.
pub fn string_net_condensation_ty() -> Expr {
    arrow(type0(), prop())
}
/// Theorem: `StringNetRealizesAllMTC`
/// Every MTC arises as the anyon content of some string-net model.
pub fn string_net_realizes_all_mtc_ty() -> Expr {
    prop()
}
/// Theorem: `TuraevViroIsStateSum`
/// The Turaev-Viro invariant is a state-sum TQFT.
pub fn turaev_viro_is_state_sum_ty() -> Expr {
    prop()
}
/// `ChernInsulator : Nat → Type`
/// A Chern insulator with Chern number n (integer quantum Hall system).
pub fn chern_insulator_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `ChernNumber : Type → Nat`
/// The first Chern number C₁ = (1/2π) ∫ F of a band structure.
pub fn chern_number_ty() -> Expr {
    arrow(type0(), nat_ty())
}
/// `FractionalQuantumHall : Nat → Nat → Type`
/// FQH state at filling fraction p/q.
pub fn fractional_quantum_hall_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `LaughlinState : Nat → Type`
/// Laughlin wavefunction at filling ν = 1/m.
pub fn laughlin_state_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `HallConductance : Type → Real`
/// The quantised Hall conductance σ_H = ν e²/h.
pub fn hall_conductance_ty() -> Expr {
    arrow(type0(), real_ty())
}
/// Theorem: `ChernNumberIntegerQuantization`
/// The Chern number is an integer (TKNN invariant).
pub fn chern_number_integer_quantization_ty() -> Expr {
    prop()
}
/// Theorem: `LaughlinAnyonCharge`
/// Quasi-particles of Laughlin ν=1/m state carry fractional charge e/m.
pub fn laughlin_anyon_charge_ty() -> Expr {
    prop()
}
/// Theorem: `FQHTopologicalOrder`
/// FQH states exhibit intrinsic topological order.
pub fn fqh_topological_order_ty() -> Expr {
    prop()
}
/// `AbelianAnyon : Type`
/// An abelian anyon type (single-valued exchange phase).
pub fn abelian_anyon_ty() -> Expr {
    type0()
}
/// `ExchangeStatistics : AnyonType → Real`
/// The statistical angle θ: ψ(…b,a…) = e^{iθ} ψ(…a,b…).
pub fn exchange_statistics_ty() -> Expr {
    arrow(cst("AnyonType"), real_ty())
}
/// `BraidingMatrix : AnyonType → AnyonType → Type`
/// The full braiding matrix (unitary) for non-Abelian anyons.
pub fn braiding_matrix_ty() -> Expr {
    arrow(cst("AnyonType"), arrow(cst("AnyonType"), type0()))
}
/// `AnyonicExchangeGroup : AnyonType → Type`
/// The group generated by braiding operations on identical anyons.
pub fn anyonic_exchange_group_ty() -> Expr {
    arrow(cst("AnyonType"), type0())
}
/// Theorem: `AbelianAnyonExchangeIsPhase`
/// For abelian anyons the braiding matrix is a scalar phase.
pub fn abelian_anyon_exchange_is_phase_ty() -> Expr {
    prop()
}
/// Theorem: `NonAbelianAnyonExchangeIsMatrix`
/// For non-Abelian anyons the braiding matrix is a non-trivial unitary matrix.
pub fn non_abelian_anyon_exchange_is_matrix_ty() -> Expr {
    prop()
}
/// `FusionCategory : Type`
/// A fusion category: semisimple monoidal category with finitely many simples.
pub fn fusion_category_ty() -> Expr {
    type0()
}
/// `SimpleObject : FusionCategory → Type`
/// A simple (irreducible) object of a fusion category.
pub fn simple_object_ty() -> Expr {
    arrow(cst("FusionCategory"), type0())
}
/// `CategoricalDimension : FusionCategory → Real`
/// The categorical dimension dim(C) = ∑_i d_i².
pub fn categorical_dimension_ty() -> Expr {
    arrow(cst("FusionCategory"), real_ty())
}
/// `PivotalStructure : FusionCategory → Prop`
/// A pivotal structure (natural isomorphism V ≅ V**).
pub fn pivotal_structure_ty() -> Expr {
    arrow(cst("FusionCategory"), prop())
}
/// `SphericalFusionCategory : Type`
/// A spherical fusion category: pivotal with dim(V) = dim(V*) for all V.
pub fn spherical_fusion_category_ty() -> Expr {
    type0()
}
/// Theorem: `FPDimAdditive`
/// Frobenius-Perron dimension is additive: d_{A⊕B} = d_A + d_B.
pub fn fp_dim_additive_ty() -> Expr {
    prop()
}
/// Theorem: `FPDimMultiplicative`
/// Frobenius-Perron dimension is multiplicative: d_{A⊗B} = d_A · d_B.
pub fn fp_dim_multiplicative_ty() -> Expr {
    prop()
}
/// Theorem: `FusionCategoryFinitelyManySimples`
/// Every fusion category has finitely many isomorphism classes of simple objects.
pub fn fusion_category_finitely_many_simples_ty() -> Expr {
    prop()
}
#[allow(clippy::too_many_arguments)]
pub fn build_topological_quantum_computation_env(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        ("Nat.eq", arrow(nat_ty(), arrow(nat_ty(), prop()))),
        ("SurfaceCodeDist", arrow(nat_ty(), nat_ty())),
        ("AnyonType", anyon_type_ty()),
        ("FusionChannel", fusion_channel_ty()),
        ("FusionMultiplicity", fusion_multiplicity_ty()),
        ("TopologicalSpin", topological_spin_ty()),
        ("VacuumCharge", vacuum_charge_ty()),
        ("AntiAnyon", anti_anyon_ty()),
        ("FusionAssociativity", fusion_associativity_ty()),
        ("FusionCommutativity", fusion_commutativity_ty()),
        ("VacuumUnit", vacuum_unit_ty()),
        ("BraidGroup", braid_group_ty()),
        ("BraidElement", braid_element_ty()),
        ("RMatrix", r_matrix_ty()),
        ("FMatrix", f_matrix_ty()),
        ("ExchangePhase", exchange_phase_ty()),
        ("BraidRelation", braid_relation_ty()),
        ("PentagonEquation", pentagon_equation_ty()),
        ("HexagonEquation", hexagon_equation_ty()),
        ("AbelianBraidingIsPhase", abelian_braiding_is_phase_ty()),
        ("ToricCode", toric_code_ty()),
        ("SurfaceCode", surface_code_ty()),
        ("TopologicalOrder", topological_order_ty()),
        ("GroundStateDegeneracy", ground_state_degeneracy_ty()),
        ("AnyonExcitation", anyon_excitation_ty()),
        ("LogicalQubit", logical_qubit_ty()),
        ("ToricCodeGSDFour", toric_code_gsd_four_ty()),
        ("TopologicalProtection", topological_protection_ty()),
        ("SurfaceCodeDistance", surface_code_distance_ty()),
        ("FibonacciAnyon", fibonacci_anyon_ty()),
        ("GoldenRatio", golden_ratio_ty()),
        ("FibFMatrix", fib_f_matrix_ty()),
        ("FibBraidMatrix", fib_braid_matrix_ty()),
        ("QuantumDimension", quantum_dimension_ty()),
        ("TotalQuantumDimension", total_quantum_dimension_ty()),
        ("FibonacciUniversal", fibonacci_universal_ty()),
        ("GoldenRatioQuantumDim", golden_ratio_quantum_dim_ty()),
        ("FibFusionRule", fib_fusion_rule_ty()),
        ("YangBaxterSolution", yang_baxter_solution_ty()),
        ("QuantumGroup", quantum_group_ty()),
        ("BraidedCategory", braided_category_ty()),
        ("YangBaxterEquation", yang_baxter_equation_ty()),
        ("BraidGroupRepresentation", braid_group_representation_ty()),
        ("QuantumGroupDuality", quantum_group_duality_ty()),
        ("ModularTensorCategory", modular_tensor_category_ty()),
        ("SMatrix", s_matrix_ty()),
        ("TMatrix", t_matrix_ty()),
        ("VerlindeFormula", verlinde_formula_ty()),
        ("RibbonElement", ribbon_element_ty()),
        ("SMatrixUnitary", s_matrix_unitary_ty()),
        ("STRelation", st_relation_ty()),
        ("VerlindeFromSMatrix", verlinde_from_s_matrix_ty()),
        ("FrobeniusSchurIndicator", frobenius_schur_indicator_ty()),
        ("ChernSimonsAction", chern_simons_action_ty()),
        ("WilsonLoop", wilson_loop_ty()),
        ("KnotInvariant", knot_invariant_ty()),
        ("JonesPolynomial", jones_polynomial_ty()),
        ("HilbertSpaceCS", hilbert_space_cs_ty()),
        ("WilsonLoopCorrelator", wilson_loop_correlator_ty()),
        ("CSPartitionFunction", cs_partition_function_ty()),
        ("CSLevelQuantization", cs_level_quantization_ty()),
        ("TQFT", tqft_ty()),
        ("Cobordism", cobordism_ty()),
        ("StateSpace", state_space_ty()),
        ("AmplitudeTQFT", amplitude_tqft_ty()),
        ("TQFTFunctoriality", tqft_functoriality_ty()),
        ("TQFTInvolutivity", tqft_involutivity_ty()),
        ("AtiyahAxioms", atiyah_axioms_ty()),
        ("CSIsaTQFT", cs_is_tqft_ty()),
        ("IsingAnyon", ising_anyon_ty()),
        ("MajoranaMode", majorana_mode_ty()),
        ("MajoranaOperator", majorana_operator_ty()),
        ("IsingFusionRule", ising_fusion_rule_ty()),
        ("NonAbelianStatistics", non_abelian_statistics_ty()),
        ("IsingNonAbelian", ising_non_abelian_ty()),
        ("MajoranaAnticommutation", majorana_anticommutation_ty()),
        ("MajoranaHermitian", majorana_hermitian_ty()),
        ("QuantumDouble", quantum_double_ty()),
        ("AnyonLabel", anyon_label_ty()),
        ("ConjugacyClass", conjugacy_class_ty()),
        ("QuantumDoubleS", quantum_double_s_ty()),
        (
            "ToricCodeIsQuantumDouble",
            toric_code_is_quantum_double_ty(),
        ),
        ("QuantumDoubleModular", quantum_double_modular_ty()),
        (
            "QuantumDoubleFusionAbelian",
            quantum_double_fusion_abelian_ty(),
        ),
        ("RibbonCategory", ribbon_category_ty()),
        ("TwistIsomorphism", twist_isomorphism_ty()),
        ("BalancingIsomorphism", balancing_isomorphism_ty()),
        ("RibbonElementHopf", ribbon_element_hopf_ty()),
        ("TwistAndBraidConsistency", twist_and_braid_consistency_ty()),
        (
            "RibbonImpliesSphericalTrace",
            ribbon_implies_spherical_trace_ty(),
        ),
        ("PentagonEquationFull", pentagon_equation_full_ty()),
        ("HexagonEquationFull", hexagon_equation_full_ty()),
        ("FrobeniusPerronDim", frobenius_perron_dim_ty()),
        ("FusionMatrix", fusion_matrix_ty()),
        ("FrobeniusPerronPositive", frobenius_perron_positive_ty()),
        ("QuantumDimEqualseFP", quantum_dim_equals_fp_ty()),
        ("TopologicalGate", topological_gate_ty()),
        ("BraidingGate", braiding_gate_ty()),
        (
            "TopologicalProtectionGate",
            topological_protection_gate_ty(),
        ),
        ("UniversalGateSet", universal_gate_set_ty()),
        ("GateApproxError", gate_approx_error_ty()),
        (
            "FibonacciBraidingUniversal",
            fibonacci_braiding_universal_ty(),
        ),
        (
            "IsingBraidingNotUniversal",
            ising_braiding_not_universal_ty(),
        ),
        ("QuantumSpinLiquid", quantum_spin_liquid_ty()),
        ("RVBState", rvb_state_ty()),
        ("FrustrationIndex", frustration_index_ty()),
        (
            "TopologicalEntanglementEntropy",
            topological_entanglement_entropy_ty(),
        ),
        ("KitaevLevinWenEntropy", kitaev_levin_wen_entropy_ty()),
        (
            "TopologicalOrderFromFrustration",
            topological_order_from_frustration_ty(),
        ),
        ("StringNet", string_net_ty()),
        ("LevinWenHamiltonian", levin_wen_hamiltonian_ty()),
        ("TuraevViroTQFT", turaev_viro_tqft_ty()),
        ("StringNetCondensation", string_net_condensation_ty()),
        ("StringNetRealizesAllMTC", string_net_realizes_all_mtc_ty()),
        ("TuraevViroIsStateSum", turaev_viro_is_state_sum_ty()),
        ("ChernInsulator", chern_insulator_ty()),
        ("ChernNumber", chern_number_ty()),
        ("FractionalQuantumHall", fractional_quantum_hall_ty()),
        ("LaughlinState", laughlin_state_ty()),
        ("HallConductance", hall_conductance_ty()),
        (
            "ChernNumberIntegerQuantization",
            chern_number_integer_quantization_ty(),
        ),
        ("LaughlinAnyonCharge", laughlin_anyon_charge_ty()),
        ("FQHTopologicalOrder", fqh_topological_order_ty()),
        ("AbelianAnyon", abelian_anyon_ty()),
        ("ExchangeStatistics", exchange_statistics_ty()),
        ("BraidingMatrix", braiding_matrix_ty()),
        ("AnyonicExchangeGroup", anyonic_exchange_group_ty()),
        (
            "AbelianAnyonExchangeIsPhase",
            abelian_anyon_exchange_is_phase_ty(),
        ),
        (
            "NonAbelianAnyonExchangeIsMatrix",
            non_abelian_anyon_exchange_is_matrix_ty(),
        ),
        ("FusionCategory", fusion_category_ty()),
        ("SimpleObject", simple_object_ty()),
        ("CategoricalDimension", categorical_dimension_ty()),
        ("PivotalStructure", pivotal_structure_ty()),
        ("SphericalFusionCategory", spherical_fusion_category_ty()),
        ("FPDimAdditive", fp_dim_additive_ty()),
        ("FPDimMultiplicative", fp_dim_multiplicative_ty()),
        (
            "FusionCategoryFinitelyManySimples",
            fusion_category_finitely_many_simples_ty(),
        ),
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
/// Golden ratio φ = (1 + √5)/2.
pub const GOLDEN_RATIO: f64 = 1.618_033_988_749_895_f64;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_toric_code_anyon_fusion() {
        use ToricCodeAnyon::*;
        assert_eq!(Electric.fuse(Electric), Vacuum);
        assert_eq!(Magnetic.fuse(Magnetic), Vacuum);
        assert_eq!(Electric.fuse(Magnetic), Fermion);
        assert_eq!(Fermion.fuse(Fermion), Vacuum);
        assert_eq!(Vacuum.fuse(Electric), Electric);
    }
    #[test]
    fn test_toric_code_anyon_spin() {
        use ToricCodeAnyon::*;
        assert!((Vacuum.topological_spin() - 1.0).abs() < 1e-10);
        assert!((Electric.topological_spin() - 1.0).abs() < 1e-10);
        assert!((Magnetic.topological_spin() - 1.0).abs() < 1e-10);
        assert!((Fermion.topological_spin() + 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_fibonacci_anyon_fusion() {
        use FibAnyon::*;
        let result = Tau.fuse(Tau);
        assert!(result.contains(&Vacuum));
        assert!(result.contains(&Tau));
        let result2 = Vacuum.fuse(Tau);
        assert_eq!(result2, vec![Tau]);
    }
    #[test]
    fn test_fibonacci_quantum_dim() {
        let phi = GOLDEN_RATIO;
        assert!((phi - (1.0 + 5.0_f64.sqrt()) / 2.0).abs() < 1e-10);
        assert!((FibAnyon::Tau.quantum_dim() - phi).abs() < 1e-10);
        let d = FibAnyon::Tau.quantum_dim();
        assert!((d * d - 1.0 - d).abs() < 1e-10);
    }
    #[test]
    fn test_fib_f_matrix_self_inverse() {
        let f = FibFMatrix::new();
        assert!(f.is_self_inverse(), "Fibonacci F-matrix must satisfy F²=I");
        assert!(f.satisfies_pentagon());
    }
    #[test]
    fn test_fib_r_matrix_unitary() {
        let r = FibRMatrix::new();
        assert!(r.is_unitary(), "Fibonacci R-matrix entries must have |r|=1");
    }
    #[test]
    fn test_braid_word_operations() {
        let b = BraidWord::with_generators(3, vec![1, 2, 1]);
        assert_eq!(b.length(), 3);
        let inv = b.inverse();
        assert_eq!(inv.generators, vec![-1, -2, -1]);
        let composed = b.compose(&inv).simplify();
        assert!(composed.generators.is_empty());
    }
    #[test]
    fn test_toric_code_properties() {
        let tc = ToricCodeQEC::new(5);
        assert_eq!(tc.ground_state_degeneracy(), 4);
        assert_eq!(tc.num_logical_qubits(), 2);
        assert_eq!(tc.distance(), 5);
        assert_eq!(tc.num_physical_qubits(), 50);
    }
    #[test]
    fn test_surface_code_singleton_bound() {
        let sc = SurfaceCode::new(3);
        assert_eq!(sc.distance(), 3);
        assert_eq!(sc.num_logical_qubits(), 1);
        assert!(sc.satisfies_singleton_bound());
    }
    #[test]
    fn test_s_matrix_toric_code_unitary() {
        let s = SModularMatrix::toric_code();
        assert!(s.is_unitary(), "Toric code S-matrix must be unitary");
        let n = s.fusion_multiplicity(0, 1, 1);
        assert!((n - 1.0).abs() < 0.5, "N_{{0,e}}^e should be 1, got {}", n);
    }
    #[test]
    fn test_build_topological_quantum_computation_env() {
        let mut env = oxilean_kernel::Environment::new();
        let result = build_topological_quantum_computation_env(&mut env);
        assert!(
            result.is_ok(),
            "build_topological_quantum_computation_env failed: {:?}",
            result.err()
        );
    }
    #[test]
    fn test_fibonacci_anyon_braiding_norm_preserved() {
        let braiding = FibonacciAnyonBraiding::new(3);
        let init = [Complex::one(), Complex::zero()];
        let braid = BraidWord::with_generators(3, vec![1]);
        let out = braiding.apply_braid(&braid, init);
        let norm_in = FibonacciAnyonBraiding::norm_sq(&init);
        let norm_out = FibonacciAnyonBraiding::norm_sq(&out);
        assert!(
            (norm_in - norm_out).abs() < 1e-9,
            "Braiding should preserve norm: in={:.6} out={:.6}",
            norm_in,
            norm_out
        );
    }
    #[test]
    fn test_fibonacci_anyon_braiding_generator_unitary() {
        let braiding = FibonacciAnyonBraiding::new(4);
        for state in [
            [Complex::one(), Complex::zero()],
            [Complex::zero(), Complex::one()],
        ] {
            let out = braiding.apply_generator(state, 2);
            let norm_in = FibonacciAnyonBraiding::norm_sq(&state);
            let norm_out = FibonacciAnyonBraiding::norm_sq(&out);
            assert!(
                (norm_in - norm_out).abs() < 1e-9,
                "Generator application must be unitary"
            );
        }
    }
    #[test]
    fn test_pentagon_equation_checker_fibonacci() {
        let checker = PentagonEquationChecker::fibonacci();
        assert!(
            checker.check_fibonacci_pentagon(),
            "Fibonacci F-matrix must satisfy pentagon equation"
        );
        assert!(
            checker.check_unitarity(),
            "Fibonacci F-matrix must be unitary"
        );
        assert_eq!(checker.n_anyons(), 2);
    }
    #[test]
    fn test_quantum_double_model_z2() {
        let model = QuantumDoubleModel::new(2);
        assert_eq!(model.n_anyons(), 4);
        assert!(model.fusion_is_abelian());
        assert!(model.vacuum_is_unit());
        assert_eq!(model.fuse((1, 0), (0, 1)), (1, 1));
        assert_eq!(model.fuse((1, 1), (1, 1)), (0, 0));
    }
    #[test]
    fn test_quantum_double_model_z3() {
        let model = QuantumDoubleModel::new(3);
        assert_eq!(model.n_anyons(), 9);
        assert!(model.fusion_is_abelian());
        assert!(model.vacuum_is_unit());
        assert_eq!(model.fuse((2, 1), (1, 2)), (0, 0));
    }
    #[test]
    fn test_quantum_double_model_total_dim() {
        let model = QuantumDoubleModel::new(4);
        let d = model.total_quantum_dimension();
        assert!((d - 4.0).abs() < 1e-10, "Total quantum dim of D(Z_4) = 4");
    }
    #[test]
    fn test_quantum_double_topological_spin() {
        let model = QuantumDoubleModel::new(4);
        let spin = model.topological_spin(0, 2);
        assert!((spin.re - 1.0).abs() < 1e-10 && spin.im.abs() < 1e-10);
    }
    #[test]
    fn test_modular_tensor_category_toric_code() {
        let mtc = ModularTensorCategoryComputer::toric_code();
        assert!(
            mtc.check_s_squared_is_charge_conjugation(),
            "S^2 should equal charge conjugation (identity for toric code)"
        );
        let d = mtc.total_quantum_dimension();
        assert!(
            (d - 2.0).abs() < 1e-9,
            "Total quantum dim of toric code = 2"
        );
    }
    #[test]
    fn test_modular_tensor_category_verlinde() {
        let mtc = ModularTensorCategoryComputer::toric_code();
        let n_em_eps = mtc.fusion_multiplicity(1, 2, 3);
        assert!((n_em_eps - 1.0).abs() < 0.5, "N_{{e,m}}^ε should be 1");
        let n_ee_vac = mtc.fusion_multiplicity(1, 1, 0);
        assert!((n_ee_vac - 1.0).abs() < 0.5, "N_{{e,e}}^1 should be 1");
    }
    #[test]
    fn test_anyon_fusion_tree_shape() {
        let tree = AnyonFusionTree::new(vec![1, 1, 1, 1], vec![1, 0], 0);
        assert!(tree.is_valid_shape());
        assert_eq!(tree.n_internal_edges(), 2);
        let tree3 = AnyonFusionTree::new(vec![1, 1, 1], vec![0], 1);
        assert!(tree3.is_valid_shape());
        assert_eq!(tree3.n_internal_edges(), 1);
    }
    #[test]
    fn test_anyon_fusion_tree_fibonacci_space_dim() {
        assert_eq!(AnyonFusionTree::fibonacci_fusion_space_dim(2), 1);
        assert_eq!(AnyonFusionTree::fibonacci_fusion_space_dim(3), 1);
        assert_eq!(AnyonFusionTree::fibonacci_fusion_space_dim(4), 2);
        assert_eq!(AnyonFusionTree::fibonacci_fusion_space_dim(5), 3);
        assert_eq!(AnyonFusionTree::fibonacci_fusion_space_dim(6), 5);
    }
    #[test]
    fn test_anyon_fusion_tree_all_four_anyon_trees() {
        let trees = AnyonFusionTree::all_four_anyon_trees();
        assert_eq!(trees.len(), 2, "4-anyon fusion space has dimension 2");
        assert!(trees[0].is_valid_shape());
        assert!(trees[1].is_valid_shape());
    }
    #[test]
    fn test_anyon_fusion_tree_f_move() {
        let tree = AnyonFusionTree::new(vec![1, 1, 1], vec![1], 1);
        let f = FibFMatrix::new();
        let new_trees = tree.apply_f_move_fibonacci(0, &f);
        assert!(!new_trees.is_empty());
    }
}
#[cfg(test)]
mod tests_tqc_extended {
    use super::*;
    #[test]
    fn test_kitaev_chain_topological_phase() {
        let chain = KitaevChain::new(10, 1.0, 1.0, 0.5);
        assert!(chain.is_topological());
        assert_eq!(chain.winding_number(), 1);
        assert_eq!(chain.n_majorana_edge_modes(), 2);
    }
    #[test]
    fn test_kitaev_chain_trivial_phase() {
        let chain = KitaevChain::new(10, 1.0, 1.0, 3.0);
        assert!(!chain.is_topological());
        assert_eq!(chain.winding_number(), 0);
        assert_eq!(chain.n_majorana_edge_modes(), 0);
    }
    #[test]
    fn test_kitaev_chain_bulk_gap() {
        let chain = KitaevChain::new(10, 1.0, 1.0, 0.0);
        assert!((chain.bulk_gap() - 2.0).abs() < 1e-10);
    }
    #[test]
    fn test_toric_code_properties() {
        let tc = ToricCodeKitaev::new(4);
        assert_eq!(tc.n_qubits(), 32);
        assert_eq!(tc.n_logical_qubits(), 2);
        assert_eq!(tc.code_distance(), 4);
        assert_eq!(tc.n_independent_stabilizers(), 30);
    }
    #[test]
    fn test_toric_code_anyon_statistics() {
        assert!((ToricCodeKitaev::mutual_statistics("e", "m") + 1.0).abs() < 1e-10);
        assert!((ToricCodeKitaev::mutual_statistics("m", "e") + 1.0).abs() < 1e-10);
        assert!((ToricCodeKitaev::mutual_statistics("e", "e") - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_toric_code_logical_error_rate() {
        let tc = ToricCodeKitaev::new(6);
        let p_l = tc.logical_error_rate(0.01);
        assert!(p_l < 0.1);
        let p_high = tc.logical_error_rate(0.5);
        assert!((p_high - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_surface_code_properties() {
        let sc = SurfaceCodeQEC::new(5);
        assert_eq!(sc.n_logical_qubits(), 1);
        assert_eq!(sc.code_distance(), 5);
        assert!(sc.n_qubits() > 0);
    }
    #[test]
    fn test_fibonacci_braid_universality() {
        let gates = FibonacciBraidGates::new(0.001);
        assert!(gates.is_universal());
        let length = gates.sk_braid_length();
        assert!(length > 0.0 && length.is_finite());
    }
    #[test]
    fn test_topological_error_suppression() {
        let err = FibonacciBraidGates::topological_error_suppression(1.0, 10.0, 1.0);
        assert!(err < 1e-3);
        assert!(err > 0.0);
    }
}
#[cfg(test)]
mod tests_tqc_extra {
    use super::*;
    #[test]
    fn test_fibonacci_anyons() {
        let fib = AnyonModel::fibonacci();
        assert_eq!(fib.n_anyon_types(), 2);
        assert!(!fib.is_abelian);
        assert!(fib.is_universal_for_quantum_computation());
    }
    #[test]
    fn test_ising_anyons() {
        let ising = AnyonModel::ising();
        assert_eq!(ising.n_anyon_types(), 3);
        assert!(!ising.is_universal_for_quantum_computation());
    }
    #[test]
    fn test_braid_word() {
        let mut bw = BraidWordNew::new(3);
        bw.push_gen(1, false);
        bw.push_gen(2, false);
        bw.push_gen(1, true);
        assert_eq!(bw.word_length(), 3);
        assert!(!bw.is_trivial_braid());
        let inv = bw.inverse();
        assert_eq!(inv.word_length(), 3);
    }
    #[test]
    fn test_modular_tensor_category() {
        let mtc = ModularTensorCategory::new("SU(2)_k", 3);
        assert!(mtc.verlinde_formula_applies());
        assert!(mtc.is_anomaly_free());
    }
}
