//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    BRSTData, BetaFunctionRG, BrstComplex, ConformalFieldTheory2D, CorrelationFunctionLattice,
    FeynmanDiagram, FeynmanDiagramEvaluator, FiniteFockSpace, KleinGordonField, OpeTable,
    PathIntegralData, QftComplex, ScatteringAmplitude, VirasoroCommutator, YangMillsTheory,
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
pub fn real_ty() -> Expr {
    cst("Real")
}
pub fn nat_ty() -> Expr {
    cst("Nat")
}
pub fn int_ty() -> Expr {
    cst("Int")
}
pub fn complex_ty() -> Expr {
    cst("Complex")
}
pub fn list_ty(t: Expr) -> Expr {
    app(cst("List"), t)
}
/// FockSpace : Type — the Hilbert space of variable-particle-number states.
/// FockSpace = ⊕_{n=0}^∞  H^⊗n
pub fn fock_space_ty() -> Expr {
    type0()
}
/// FockState : FockSpace → Type — a specific state in the Fock space.
pub fn fock_state_ty() -> Expr {
    arrow(cst("FockSpace"), type0())
}
/// OccupationNumber : Mode → ℕ — occupation number for a given mode.
pub fn occupation_number_ty() -> Expr {
    arrow(cst("Mode"), nat_ty())
}
/// CreationOp : Mode → FockSpace → FockSpace
/// a†_k : raises occupation number of mode k by 1.
pub fn creation_op_ty() -> Expr {
    arrow(cst("Mode"), arrow(cst("FockSpace"), cst("FockSpace")))
}
/// AnnihilationOp : Mode → FockSpace → FockSpace
/// a_k : lowers occupation number of mode k by 1.
pub fn annihilation_op_ty() -> Expr {
    arrow(cst("Mode"), arrow(cst("FockSpace"), cst("FockSpace")))
}
/// NumberOp : Mode → FockSpace → FockSpace
/// N_k = a†_k a_k : counts particles in mode k.
pub fn number_op_ty() -> Expr {
    arrow(cst("Mode"), arrow(cst("FockSpace"), cst("FockSpace")))
}
/// CanonicalCommutationRelation : [a_i, a†_j] = δ_{ij}
/// CCR : Mode → Mode → Prop
pub fn ccr_ty() -> Expr {
    arrow(cst("Mode"), arrow(cst("Mode"), prop()))
}
/// CanonicalAnticommutationRelation : {a_i, a†_j} = δ_{ij}  (fermions)
/// CAR : Mode → Mode → Prop
pub fn car_ty() -> Expr {
    arrow(cst("Mode"), arrow(cst("Mode"), prop()))
}
/// VacuumState : FockSpace — the state with no particles |0⟩.
pub fn vacuum_state_ty() -> Expr {
    cst("FockSpace")
}
/// AnnihilatesVacuum : a_k |0⟩ = 0
pub fn annihilates_vacuum_ty() -> Expr {
    arrow(cst("Mode"), prop())
}
/// ScalarField : Spacetime → ℝ — a real scalar field φ(x).
pub fn scalar_field_ty() -> Expr {
    arrow(cst("Spacetime"), real_ty())
}
/// KleinGordonEq : ScalarField → ℝ → Prop
/// (□ + m²) φ = 0, where m is the mass.
pub fn klein_gordon_eq_ty() -> Expr {
    arrow(arrow(cst("Spacetime"), real_ty()), arrow(real_ty(), prop()))
}
/// ScalarQFTLagrangian : ScalarField → ℝ
/// L = ½(∂μφ)² − ½m²φ²
pub fn scalar_qft_lagrangian_ty() -> Expr {
    arrow(arrow(cst("Spacetime"), real_ty()), real_ty())
}
/// MomentumMode : (4-momentum) → Mode — plane-wave decomposition.
pub fn momentum_mode_ty() -> Expr {
    arrow(cst("FourMomentum"), cst("Mode"))
}
/// FieldExpansion : ScalarField → Integral(Mode × (CreationOp + AnnihilationOp))
/// φ(x) = ∫ d³p/(2π)³ 1/√(2ωp) [a_p e^{ipx} + a†_p e^{-ipx}]
pub fn field_expansion_ty() -> Expr {
    arrow(arrow(cst("Spacetime"), real_ty()), prop())
}
/// Propagator : Spacetime → Spacetime → Complex
/// Δ_F(x-y) = ⟨0|T{φ(x)φ(y)}|0⟩
pub fn propagator_ty() -> Expr {
    arrow(cst("Spacetime"), arrow(cst("Spacetime"), complex_ty()))
}
/// TimeOrdering : FockState → FockState → FockState
/// T{A(x)B(y)} = θ(x⁰-y⁰)A(x)B(y) + θ(y⁰-x⁰)B(y)A(x)
pub fn time_ordering_ty() -> Expr {
    arrow(cst("FockSpace"), arrow(cst("FockSpace"), cst("FockSpace")))
}
/// PathIntegral : Action → Observable → Complex
/// Z[J] = ∫ Dφ exp(i S[φ] + i ∫J φ)
pub fn path_integral_ty() -> Expr {
    arrow(
        arrow(arrow(cst("Spacetime"), real_ty()), real_ty()),
        complex_ty(),
    )
}
/// GeneratingFunctional : (Spacetime → ℝ) → Complex
/// Z[J] = ⟨0|0⟩_J
pub fn generating_functional_ty() -> Expr {
    arrow(arrow(cst("Spacetime"), real_ty()), complex_ty())
}
/// ConnectedGreenFunction : ℕ → (Spacetime^n → Complex)
/// G^(n)(x₁,...,xₙ) — n-point connected Green's function
pub fn connected_green_fn_ty() -> Expr {
    arrow(nat_ty(), arrow(list_ty(cst("Spacetime")), complex_ty()))
}
/// LatticePathIntegral : lattice discretization of Z[J].
pub fn lattice_path_integral_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), complex_ty()))
}
/// WickRotation : Spacetime → Spacetime
/// Wick rotation t → -iτ: Minkowski → Euclidean
pub fn wick_rotation_ty() -> Expr {
    arrow(cst("Spacetime"), cst("Spacetime"))
}
/// EuclideanAction : ScalarField → ℝ
/// S_E[φ] = ∫ dτ d³x [½(∂φ)² + ½m²φ² + V(φ)]
pub fn euclidean_action_ty() -> Expr {
    arrow(arrow(cst("Spacetime"), real_ty()), real_ty())
}
/// FeynmanDiagram : a combinatorial representation of a perturbation theory diagram.
pub fn feynman_diagram_ty() -> Expr {
    type0()
}
/// ExternalLeg : FeynmanDiagram → Mode → Prop
pub fn external_leg_ty() -> Expr {
    arrow(cst("FeynmanDiagram"), arrow(cst("Mode"), prop()))
}
/// InternalVertex : FeynmanDiagram → Spacetime → Prop
pub fn internal_vertex_ty() -> Expr {
    arrow(cst("FeynmanDiagram"), arrow(cst("Spacetime"), prop()))
}
/// Propagator edge connects two vertices.
/// PropagatorEdge : FeynmanDiagram → Spacetime → Spacetime → Prop
pub fn propagator_edge_ty() -> Expr {
    arrow(
        cst("FeynmanDiagram"),
        arrow(cst("Spacetime"), arrow(cst("Spacetime"), prop())),
    )
}
/// SymmetryFactor : FeynmanDiagram → ℕ — the symmetry factor of a diagram.
pub fn symmetry_factor_ty() -> Expr {
    arrow(cst("FeynmanDiagram"), nat_ty())
}
/// DiagramAmplitude : FeynmanDiagram → Complex
pub fn diagram_amplitude_ty() -> Expr {
    arrow(cst("FeynmanDiagram"), complex_ty())
}
/// LoopOrder : FeynmanDiagram → ℕ — the loop order L = I - V + 1.
pub fn loop_order_ty() -> Expr {
    arrow(cst("FeynmanDiagram"), nat_ty())
}
/// UVDivergence : FeynmanDiagram → Prop — the diagram has a UV divergence.
pub fn uv_divergence_ty() -> Expr {
    arrow(cst("FeynmanDiagram"), prop())
}
/// RegularizedAmplitude : FeynmanDiagram → ℝ → Complex
/// A_ε(D) — amplitude with dimensional regulator ε.
pub fn regularized_amplitude_ty() -> Expr {
    arrow(cst("FeynmanDiagram"), arrow(real_ty(), complex_ty()))
}
/// CounterTerm : name → type → Expr — a counterterm in the Lagrangian.
pub fn counter_term_ty() -> Expr {
    arrow(
        arrow(cst("Spacetime"), real_ty()),
        arrow(cst("Spacetime"), real_ty()),
    )
}
/// RenormalizationGroup : ℝ → ℝ — RG flow of a coupling constant.
/// g(μ) — running coupling as a function of scale μ.
pub fn renormalization_group_ty() -> Expr {
    arrow(real_ty(), real_ty())
}
/// BetaFunction : ℝ → ℝ
/// β(g) = μ dg/dμ
pub fn beta_function_ty() -> Expr {
    arrow(real_ty(), real_ty())
}
/// AsymptoticFreedom : BetaFunction → Prop — β(g) < 0 for small g.
pub fn asymptotic_freedom_ty() -> Expr {
    arrow(arrow(real_ty(), real_ty()), prop())
}
/// WilsonianEFT : ℝ → Environment — Wilsonian EFT at scale Λ.
pub fn wilsonian_eft_ty() -> Expr {
    arrow(real_ty(), type0())
}
/// GhostField : Spacetime → GrassmannAlgebra — the Faddeev-Popov ghost field.
pub fn ghost_field_ty() -> Expr {
    arrow(cst("Spacetime"), cst("GrassmannAlgebra"))
}
/// BRSTOperator : (Field → Field) — the nilpotent BRST operator s.
/// s² = 0
pub fn brst_operator_ty() -> Expr {
    arrow(cst("FockSpace"), cst("FockSpace"))
}
/// BRSTNilpotent : BRSTOperator → Prop — s(s(|ψ⟩)) = 0.
pub fn brst_nilpotent_ty() -> Expr {
    arrow(arrow(cst("FockSpace"), cst("FockSpace")), prop())
}
/// BRSTCohomology : BRSTOperator → Type — physical states = ker(s)/im(s).
pub fn brst_cohomology_ty() -> Expr {
    arrow(arrow(cst("FockSpace"), cst("FockSpace")), type0())
}
/// GaugeFixing : GaugeField → FeynmanDiagram → Prop — gauge-fixing condition.
pub fn gauge_fixing_ty() -> Expr {
    arrow(cst("GaugeField"), arrow(cst("FeynmanDiagram"), prop()))
}
/// FaddeevPopovDeterminant : GaugeField → Complex
pub fn faddeev_popov_det_ty() -> Expr {
    arrow(cst("GaugeField"), complex_ty())
}
/// GaugeGroup : Type — a Lie group G acting as the gauge symmetry group.
pub fn gauge_group_ty() -> Expr {
    type0()
}
/// GaugeField : Spacetime → LieAlgebra — the gauge connection A_μ(x).
pub fn gauge_field_ty() -> Expr {
    arrow(cst("Spacetime"), cst("LieAlgebra"))
}
/// FieldStrength : GaugeField → (Spacetime → Spacetime → LieAlgebra)
/// F_μν = ∂_μ A_ν − ∂_ν A_μ + [A_μ, A_ν]
pub fn field_strength_ty() -> Expr {
    arrow(
        arrow(cst("Spacetime"), cst("LieAlgebra")),
        arrow(cst("Spacetime"), arrow(cst("Spacetime"), cst("LieAlgebra"))),
    )
}
/// YangMillsLagrangian : GaugeField → ℝ
/// L = -¼ Tr(F_μν F^μν)
pub fn yang_mills_lagrangian_ty() -> Expr {
    arrow(arrow(cst("Spacetime"), cst("LieAlgebra")), real_ty())
}
/// GaugeTransformation : GaugeGroup → GaugeField → GaugeField
pub fn gauge_transformation_ty() -> Expr {
    arrow(
        cst("GaugeGroup"),
        arrow(
            arrow(cst("Spacetime"), cst("LieAlgebra")),
            arrow(cst("Spacetime"), cst("LieAlgebra")),
        ),
    )
}
/// GaugeInvariant : (GaugeField → Prop) → Prop
/// A predicate is gauge-invariant if it commutes with gauge transformations.
pub fn gauge_invariant_ty() -> Expr {
    arrow(
        arrow(arrow(cst("Spacetime"), cst("LieAlgebra")), prop()),
        prop(),
    )
}
/// WilsonLine : GaugeField → Path → Complex
/// W(C) = Tr P exp(i ∮_C A_μ dx^μ)
pub fn wilson_line_ty() -> Expr {
    arrow(
        arrow(cst("Spacetime"), cst("LieAlgebra")),
        arrow(cst("Path"), complex_ty()),
    )
}
/// FieldSymmetry : (ScalarField → ScalarField) — an infinitesimal symmetry of the action.
pub fn field_symmetry_ty() -> Expr {
    arrow(
        arrow(cst("Spacetime"), real_ty()),
        arrow(cst("Spacetime"), real_ty()),
    )
}
/// ConservedCurrent : FieldSymmetry → (Spacetime → ℝ^4)
/// j^μ — the Noether current associated to a symmetry.
pub fn conserved_current_ty() -> Expr {
    arrow(
        arrow(
            arrow(cst("Spacetime"), real_ty()),
            arrow(cst("Spacetime"), real_ty()),
        ),
        arrow(cst("Spacetime"), cst("FourVector")),
    )
}
/// CurrentConservation : ConservedCurrent → Prop
/// ∂_μ j^μ = 0
pub fn current_conservation_ty() -> Expr {
    arrow(
        arrow(
            arrow(
                arrow(cst("Spacetime"), real_ty()),
                arrow(cst("Spacetime"), real_ty()),
            ),
            arrow(cst("Spacetime"), cst("FourVector")),
        ),
        prop(),
    )
}
/// NoetherCharge : ConservedCurrent → ℝ
/// Q = ∫ d³x j⁰(x)
pub fn noether_charge_ty() -> Expr {
    arrow(arrow(cst("Spacetime"), cst("FourVector")), real_ty())
}
/// NoetherTheorem : FieldSymmetry → ConservedCurrent → Prop
pub fn noether_theorem_ty() -> Expr {
    arrow(
        arrow(
            arrow(cst("Spacetime"), real_ty()),
            arrow(cst("Spacetime"), real_ty()),
        ),
        arrow(arrow(cst("Spacetime"), cst("FourVector")), prop()),
    )
}
/// EnergyMomentumTensor : ScalarField → (Spacetime → Spacetime → ℝ)
/// T^μν = ∂L/∂(∂_μφ) ∂^ν φ − g^μν L
pub fn energy_momentum_tensor_ty() -> Expr {
    arrow(
        arrow(cst("Spacetime"), real_ty()),
        arrow(cst("Spacetime"), arrow(cst("Spacetime"), real_ty())),
    )
}
/// WardIdentity : GaugeField → Prop
/// The Ward-Takahashi identity relating Green's functions.
pub fn ward_identity_ty() -> Expr {
    arrow(arrow(cst("Spacetime"), cst("LieAlgebra")), prop())
}
/// VertexFunction : ℕ → (List Spacetime → Complex)
/// Γ^(n)(x₁,...,xₙ) — the 1PI n-point vertex function.
pub fn vertex_function_ty() -> Expr {
    arrow(nat_ty(), arrow(list_ty(cst("Spacetime")), complex_ty()))
}
/// SlavnovTaylorIdentity : GaugeField → BRSTOperator → Prop
pub fn slavnov_taylor_identity_ty() -> Expr {
    arrow(
        arrow(cst("Spacetime"), cst("LieAlgebra")),
        arrow(arrow(cst("FockSpace"), cst("FockSpace")), prop()),
    )
}
/// LSZReductionFormula : VertexFunction → SMatrix → Prop
/// Relates Green's functions to S-matrix elements.
pub fn lsz_reduction_ty() -> Expr {
    arrow(
        arrow(nat_ty(), arrow(list_ty(cst("Spacetime")), complex_ty())),
        arrow(cst("SMatrix"), prop()),
    )
}
/// ChargeConjugation : FockSpace → FockSpace — C operator.
pub fn charge_conjugation_ty() -> Expr {
    arrow(cst("FockSpace"), cst("FockSpace"))
}
/// ParityTransformation : FockSpace → FockSpace — P operator.
pub fn parity_transformation_ty() -> Expr {
    arrow(cst("FockSpace"), cst("FockSpace"))
}
/// TimeReversal : FockSpace → FockSpace — T operator (antiunitary).
pub fn time_reversal_ty() -> Expr {
    arrow(cst("FockSpace"), cst("FockSpace"))
}
/// CPTOperator : FockSpace → FockSpace — the combined CPT operator.
pub fn cpt_operator_ty() -> Expr {
    arrow(cst("FockSpace"), cst("FockSpace"))
}
/// CPTTheorem : CPTOperator → Prop
/// Every Lorentz-invariant local QFT is CPT invariant.
pub fn cpt_theorem_ty() -> Expr {
    arrow(arrow(cst("FockSpace"), cst("FockSpace")), prop())
}
/// SpinStatisticsTheorem : ℕ → Prop
/// Integer spin → Bose-Einstein statistics; half-integer → Fermi-Dirac.
pub fn spin_statistics_theorem_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// LorentzCovariance : (FockSpace → FockSpace) → Prop
pub fn lorentz_covariance_ty() -> Expr {
    arrow(arrow(cst("FockSpace"), cst("FockSpace")), prop())
}
/// WightmanField : (Spacetime → Operator) — an operator-valued distribution.
/// The fundamental object in Wightman's axiomatic QFT.
pub fn wightman_field_ty() -> Expr {
    arrow(cst("Spacetime"), cst("HilbertOp"))
}
/// PoincareCovariance : WightmanField → Prop
/// U(a,Λ) φ(x) U(a,Λ)⁻¹ = φ(Λx + a)
pub fn poincare_covariance_ty() -> Expr {
    arrow(arrow(cst("Spacetime"), cst("HilbertOp")), prop())
}
/// WightmanLocality : WightmanField → WightmanField → Prop
/// [φ(x), φ(y)] = 0 for spacelike separated x,y.
pub fn wightman_locality_ty() -> Expr {
    arrow(
        arrow(cst("Spacetime"), cst("HilbertOp")),
        arrow(arrow(cst("Spacetime"), cst("HilbertOp")), prop()),
    )
}
/// SpectrumCondition : Prop — eigenvalues of P^μ lie in the forward light cone.
pub fn spectrum_condition_ty() -> Expr {
    prop()
}
/// WightmanPositivity : WightmanField → Prop
/// ⟨ψ|A†A|ψ⟩ ≥ 0 for all states |ψ⟩.
pub fn wightman_positivity_ty() -> Expr {
    arrow(arrow(cst("Spacetime"), cst("HilbertOp")), prop())
}
/// WightmanDistribution : (List Spacetime → Complex) — Wightman n-point function.
/// W(x₁,...,xₙ) = ⟨0|φ(x₁)···φ(xₙ)|0⟩
pub fn wightman_distribution_ty() -> Expr {
    arrow(list_ty(cst("Spacetime")), complex_ty())
}
/// ReconstructionTheorem : WightmanDistribution → WightmanField → Prop
/// Wightman's reconstruction: axioms ↔ Wightman functions.
pub fn reconstruction_theorem_ty() -> Expr {
    arrow(
        arrow(list_ty(cst("Spacetime")), complex_ty()),
        arrow(arrow(cst("Spacetime"), cst("HilbertOp")), prop()),
    )
}
/// LocalAlgebra : Region → CStarAlgebra — net of local C*-algebras O ↦ A(O).
pub fn local_algebra_ty() -> Expr {
    arrow(cst("SpacetimeRegion"), cst("CStarAlgebra"))
}
/// Isotony : LocalAlgebra → Prop
/// O₁ ⊆ O₂ ⇒ A(O₁) ⊆ A(O₂).
pub fn isotony_ty() -> Expr {
    arrow(arrow(cst("SpacetimeRegion"), cst("CStarAlgebra")), prop())
}
/// HaagKastlerCausality : LocalAlgebra → Prop
/// O₁ ⊥ O₂ (spacelike) ⇒ [A(O₁), A(O₂)] = 0.
pub fn haag_kastler_causality_ty() -> Expr {
    arrow(arrow(cst("SpacetimeRegion"), cst("CStarAlgebra")), prop())
}
/// HaagDuality : LocalAlgebra → Prop
/// A(O') = A(O)' (commutant equality for causal complement O').
pub fn haag_duality_ty() -> Expr {
    arrow(arrow(cst("SpacetimeRegion"), cst("CStarAlgebra")), prop())
}
/// Reeh_SchliederTheorem : LocalAlgebra → Prop
/// Local algebras act cyclically on the vacuum.
pub fn reeh_schlieder_theorem_ty() -> Expr {
    arrow(arrow(cst("SpacetimeRegion"), cst("CStarAlgebra")), prop())
}
/// ForestFormula : FeynmanDiagram → Complex
/// BPHZ forest formula: R(Γ) = Σ_{forests F} (-T_F) Γ.
pub fn forest_formula_ty() -> Expr {
    arrow(cst("FeynmanDiagram"), complex_ty())
}
/// SubtractionOperator : FeynmanDiagram → FeynmanDiagram → Prop
/// T: subtracts the UV-divergent part of a subdiagram.
pub fn subtraction_operator_ty() -> Expr {
    arrow(cst("FeynmanDiagram"), arrow(cst("FeynmanDiagram"), prop()))
}
/// BphzRenormalized : FeynmanDiagram → Complex
/// The fully BPHZ-renormalized amplitude.
pub fn bphz_renormalized_ty() -> Expr {
    arrow(cst("FeynmanDiagram"), complex_ty())
}
/// DimRegAmplitude : FeynmanDiagram → ℝ → Complex
/// Amplitude in d = 4 - 2ε dimensions.
pub fn dim_reg_amplitude_ty() -> Expr {
    arrow(cst("FeynmanDiagram"), arrow(real_ty(), complex_ty()))
}
/// EpsilonPole : FeynmanDiagram → ℕ → Complex
/// The coefficient of 1/ε^n in the Laurent expansion.
pub fn epsilon_pole_ty() -> Expr {
    arrow(cst("FeynmanDiagram"), arrow(nat_ty(), complex_ty()))
}
/// MSbarScheme : FeynmanDiagram → Complex
/// The MS-bar renormalized amplitude (subtracts poles + ln(4π) - γ_E).
pub fn msbar_scheme_ty() -> Expr {
    arrow(cst("FeynmanDiagram"), complex_ty())
}
/// RGFixedPoint : BetaFunction → ℝ → Prop
/// g* is a fixed point: β(g*) = 0.
pub fn rg_fixed_point_ty() -> Expr {
    arrow(arrow(real_ty(), real_ty()), arrow(real_ty(), prop()))
}
/// UniversalityClass : BetaFunction → Type
/// Theories sharing a fixed point belong to the same universality class.
pub fn universality_class_ty() -> Expr {
    arrow(arrow(real_ty(), real_ty()), type0())
}
/// CriticalExponent : UniversalityClass → ℝ
/// The anomalous dimension η, correlation length exponent ν, etc.
pub fn critical_exponent_ty() -> Expr {
    arrow(type0(), real_ty())
}
/// SchwingerDysonEq : VertexFunction → Prop
/// The functional differential equation δZ/δJ = ⟨φ⟩_J.
pub fn schwinger_dyson_eq_ty() -> Expr {
    arrow(
        arrow(nat_ty(), arrow(list_ty(cst("Spacetime")), complex_ty())),
        prop(),
    )
}
/// SelfEnergyFunction : Spacetime → Complex
/// Σ(p²) — the 1PI self-energy insertion.
pub fn self_energy_fn_ty() -> Expr {
    arrow(cst("Spacetime"), complex_ty())
}
/// DressedPropagator : SelfEnergyFunction → (Spacetime → Spacetime → Complex)
/// G(p) = 1/(p² - m² - Σ(p²)).
pub fn dressed_propagator_ty() -> Expr {
    arrow(
        arrow(cst("Spacetime"), complex_ty()),
        arrow(cst("Spacetime"), arrow(cst("Spacetime"), complex_ty())),
    )
}
/// InLSZState : Mode → FockSpace → Prop
/// Asymptotic in-state |k₁ k₂ ...⟩_in used in LSZ.
pub fn in_lsz_state_ty() -> Expr {
    arrow(cst("Mode"), arrow(cst("FockSpace"), prop()))
}
/// OutLSZState : Mode → FockSpace → Prop
/// Asymptotic out-state ⟨k₁ k₂ ...|_out used in LSZ.
pub fn out_lsz_state_ty() -> Expr {
    arrow(cst("Mode"), arrow(cst("FockSpace"), prop()))
}
/// FactorizationTheorem : FeynmanDiagram → Prop
/// Collinear and soft limits factorize into universal splitting functions.
pub fn factorization_theorem_ty() -> Expr {
    arrow(cst("FeynmanDiagram"), prop())
}
/// InfraredSafe : Observable → Prop
/// Observable is insensitive to soft/collinear emissions.
pub fn infrared_safe_ty() -> Expr {
    arrow(cst("Observable"), prop())
}
/// OPECoefficient : WightmanField → WightmanField → WightmanField → ℝ → Complex
/// C_{AB}^C(x-y): OPE coefficient.
pub fn ope_coefficient_ty() -> Expr {
    arrow(
        arrow(cst("Spacetime"), cst("HilbertOp")),
        arrow(
            arrow(cst("Spacetime"), cst("HilbertOp")),
            arrow(
                arrow(cst("Spacetime"), cst("HilbertOp")),
                arrow(real_ty(), complex_ty()),
            ),
        ),
    )
}
/// OperatorProductExpansion : WightmanField → WightmanField → Prop
/// φ(x)φ(y) ~ Σ_k C_k(x-y) O_k(y) as x → y.
pub fn operator_product_expansion_ty() -> Expr {
    arrow(
        arrow(cst("Spacetime"), cst("HilbertOp")),
        arrow(arrow(cst("Spacetime"), cst("HilbertOp")), prop()),
    )
}
/// ConformalPrimary : WightmanField → ℝ → Prop
/// A conformal primary operator of dimension Δ.
pub fn conformal_primary_ty() -> Expr {
    arrow(
        arrow(cst("Spacetime"), cst("HilbertOp")),
        arrow(real_ty(), prop()),
    )
}
/// VirasoroGenerator : ℤ → (FockSpace → FockSpace)
/// L_n — Virasoro generator as an operator.
pub fn virasoro_generator_ty() -> Expr {
    arrow(int_ty(), arrow(cst("FockSpace"), cst("FockSpace")))
}
/// VirasoroAlgebraRelation : ℤ → ℤ → ℝ → Prop
/// [L_m, L_n] = (m-n)L_{m+n} + c/12 m(m²-1) δ_{m+n,0}.
pub fn virasoro_algebra_relation_ty() -> Expr {
    arrow(int_ty(), arrow(int_ty(), arrow(real_ty(), prop())))
}
/// CentralCharge : ℝ — the Virasoro central charge c.
pub fn central_charge_ty() -> Expr {
    real_ty()
}
/// CTheorem : ℝ → ℝ → Prop
/// Zamolodchikov's c-theorem: c decreases along RG flow.
pub fn c_theorem_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), prop()))
}
/// ModularInvariance : (Complex → Complex) → Prop
/// The partition function Z(τ) is invariant under SL(2,ℤ).
pub fn modular_invariance_ty() -> Expr {
    arrow(arrow(complex_ty(), complex_ty()), prop())
}
/// ConformalBootstrap : ℝ → Prop
/// Bootstrap consistency of OPE coefficients and conformal dimensions.
pub fn conformal_bootstrap_ty() -> Expr {
    arrow(real_ty(), prop())
}
/// ChernSimonsAction : GaugeField → ℝ → ℝ
/// S_CS[A] = (k/4π) ∫ Tr(A ∧ dA + 2/3 A ∧ A ∧ A).
pub fn chern_simons_action_ty() -> Expr {
    arrow(
        arrow(cst("Spacetime"), cst("LieAlgebra")),
        arrow(real_ty(), real_ty()),
    )
}
/// BFAction : GaugeField → (Spacetime → LieAlgebra) → ℝ
/// S_BF[A,B] = ∫ Tr(B ∧ F[A]).
pub fn bf_action_ty() -> Expr {
    arrow(
        arrow(cst("Spacetime"), cst("LieAlgebra")),
        arrow(arrow(cst("Spacetime"), cst("LieAlgebra")), real_ty()),
    )
}
/// TQFTFunctor : SpacetimeRegion → CStarAlgebra → Prop
/// Atiyah's TQFT axioms: functorial assignment of vector spaces to manifolds.
pub fn tqft_functor_ty() -> Expr {
    arrow(arrow(cst("SpacetimeRegion"), cst("CStarAlgebra")), prop())
}
/// LinkingNumber : Path → Path → ℤ
/// Topological invariant of two closed curves in 3-space.
pub fn linking_number_ty() -> Expr {
    arrow(cst("Path"), arrow(cst("Path"), int_ty()))
}
/// JonesPolynomial : Path → ℤ → Complex
/// The Jones polynomial V_L(t) of a knot/link L.
pub fn jones_polynomial_ty() -> Expr {
    arrow(cst("Path"), arrow(int_ty(), complex_ty()))
}
/// LatticeAction : ℕ → ℝ → ℝ
/// Wilson action S_lat[U] on a lattice of size N with coupling β.
pub fn lattice_action_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), real_ty()))
}
/// LatticeGreenFunction : ℕ → ℕ → ℝ → Complex
/// The lattice propagator G(x,y;β).
pub fn lattice_green_fn_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(real_ty(), complex_ty())))
}
/// InstantonSolution : GaugeField → ℕ → Prop
/// A self-dual gauge field with instanton number k.
pub fn instanton_solution_ty() -> Expr {
    arrow(
        arrow(cst("Spacetime"), cst("LieAlgebra")),
        arrow(nat_ty(), prop()),
    )
}
/// InstantonAction : ℕ → ℝ
/// S_inst(k) = 8π²k/g² — the instanton action.
pub fn instanton_action_ty() -> Expr {
    arrow(nat_ty(), real_ty())
}
/// SeibergWittenCurve : ℝ → Complex → Prop
/// The Seiberg-Witten curve encoding the exact low-energy effective action.
pub fn seiberg_witten_curve_ty() -> Expr {
    arrow(real_ty(), arrow(complex_ty(), prop()))
}
/// PrepotentialSW : ℝ → Complex
/// F(a) — the prepotential of the Seiberg-Witten effective theory.
pub fn prepotential_sw_ty() -> Expr {
    arrow(real_ty(), complex_ty())
}
/// Register all quantum field theory axioms into the environment.
pub fn build_quantum_field_theory_env() -> Environment {
    let mut env = Environment::new();
    let axioms: &[(&str, Expr)] = &[
        ("Mode", type0()),
        ("Spacetime", type0()),
        ("FourMomentum", type0()),
        ("FourVector", type0()),
        ("LieAlgebra", type0()),
        ("GaugeField", type0()),
        ("GaugeGroup", type0()),
        ("GrassmannAlgebra", type0()),
        ("SMatrix", type0()),
        ("Path", type0()),
        ("HilbertOp", type0()),
        ("SpacetimeRegion", type0()),
        ("CStarAlgebra", type0()),
        ("Observable", type0()),
        ("FockSpace", fock_space_ty()),
        ("FockState", fock_state_ty()),
        ("OccupationNumber", occupation_number_ty()),
        ("CreationOp", creation_op_ty()),
        ("AnnihilationOp", annihilation_op_ty()),
        ("NumberOp", number_op_ty()),
        ("CanonicalCommutationRelation", ccr_ty()),
        ("CanonicalAnticommutationRelation", car_ty()),
        ("VacuumState", vacuum_state_ty()),
        ("AnnihilatesVacuum", annihilates_vacuum_ty()),
        ("ScalarField", scalar_field_ty()),
        ("KleinGordonEq", klein_gordon_eq_ty()),
        ("ScalarQFTLagrangian", scalar_qft_lagrangian_ty()),
        ("MomentumMode", momentum_mode_ty()),
        ("FieldExpansion", field_expansion_ty()),
        ("FeynmanPropagator", propagator_ty()),
        ("TimeOrdering", time_ordering_ty()),
        ("PathIntegral", path_integral_ty()),
        ("GeneratingFunctional", generating_functional_ty()),
        ("ConnectedGreenFn", connected_green_fn_ty()),
        ("LatticePathIntegral", lattice_path_integral_ty()),
        ("WickRotation", wick_rotation_ty()),
        ("EuclideanAction", euclidean_action_ty()),
        ("FeynmanDiagram", feynman_diagram_ty()),
        ("ExternalLeg", external_leg_ty()),
        ("InternalVertex", internal_vertex_ty()),
        ("PropagatorEdge", propagator_edge_ty()),
        ("SymmetryFactor", symmetry_factor_ty()),
        ("DiagramAmplitude", diagram_amplitude_ty()),
        ("LoopOrder", loop_order_ty()),
        ("UVDivergence", uv_divergence_ty()),
        ("RegularizedAmplitude", regularized_amplitude_ty()),
        ("CounterTerm", counter_term_ty()),
        ("RenormalizationGroup", renormalization_group_ty()),
        ("BetaFunction", beta_function_ty()),
        ("AsymptoticFreedom", asymptotic_freedom_ty()),
        ("WilsonianEFT", wilsonian_eft_ty()),
        ("GhostField", ghost_field_ty()),
        ("BRSTOperator", brst_operator_ty()),
        ("BRSTNilpotent", brst_nilpotent_ty()),
        ("BRSTCohomology", brst_cohomology_ty()),
        ("GaugeFixing", gauge_fixing_ty()),
        ("FaddeevPopovDet", faddeev_popov_det_ty()),
        ("GaugeGroup_t", gauge_group_ty()),
        ("GaugeField_t", gauge_field_ty()),
        ("FieldStrength", field_strength_ty()),
        ("YangMillsLagrangian", yang_mills_lagrangian_ty()),
        ("GaugeTransformation", gauge_transformation_ty()),
        ("GaugeInvariant", gauge_invariant_ty()),
        ("WilsonLine", wilson_line_ty()),
        ("FieldSymmetry", field_symmetry_ty()),
        ("ConservedCurrent", conserved_current_ty()),
        ("CurrentConservation", current_conservation_ty()),
        ("NoetherCharge", noether_charge_ty()),
        ("noether_theorem", noether_theorem_ty()),
        ("EnergyMomentumTensor", energy_momentum_tensor_ty()),
        ("WardIdentity", ward_identity_ty()),
        ("VertexFunction", vertex_function_ty()),
        ("SlavnovTaylorIdentity", slavnov_taylor_identity_ty()),
        ("LSZReductionFormula", lsz_reduction_ty()),
        ("ChargeConjugation", charge_conjugation_ty()),
        ("ParityTransformation", parity_transformation_ty()),
        ("TimeReversal", time_reversal_ty()),
        ("CPTOperator", cpt_operator_ty()),
        ("cpt_theorem", cpt_theorem_ty()),
        ("spin_statistics_theorem", spin_statistics_theorem_ty()),
        ("LorentzCovariance", lorentz_covariance_ty()),
        ("HbarConstant", real_ty()),
        ("SpeedOfLight", real_ty()),
        ("ElementaryCharge", real_ty()),
        ("CouplingConstantQED", real_ty()),
        ("CouplingConstantQCD", real_ty()),
        ("WightmanField", wightman_field_ty()),
        ("PoincareCovariance", poincare_covariance_ty()),
        ("WightmanLocality", wightman_locality_ty()),
        ("SpectrumCondition", spectrum_condition_ty()),
        ("WightmanPositivity", wightman_positivity_ty()),
        ("WightmanDistribution", wightman_distribution_ty()),
        ("ReconstructionTheorem", reconstruction_theorem_ty()),
        ("LocalAlgebra", local_algebra_ty()),
        ("Isotony", isotony_ty()),
        ("HaagKastlerCausality", haag_kastler_causality_ty()),
        ("HaagDuality", haag_duality_ty()),
        ("ReehSchliederTheorem", reeh_schlieder_theorem_ty()),
        ("ForestFormula", forest_formula_ty()),
        ("SubtractionOperator", subtraction_operator_ty()),
        ("BphzRenormalized", bphz_renormalized_ty()),
        ("DimRegAmplitude", dim_reg_amplitude_ty()),
        ("EpsilonPole", epsilon_pole_ty()),
        ("MSbarScheme", msbar_scheme_ty()),
        ("RGFixedPoint", rg_fixed_point_ty()),
        ("UniversalityClass", universality_class_ty()),
        ("CriticalExponent", critical_exponent_ty()),
        ("SchwingerDysonEq", schwinger_dyson_eq_ty()),
        ("SelfEnergyFunction", self_energy_fn_ty()),
        ("DressedPropagator", dressed_propagator_ty()),
        ("InLSZState", in_lsz_state_ty()),
        ("OutLSZState", out_lsz_state_ty()),
        ("FactorizationTheorem", factorization_theorem_ty()),
        ("InfraredSafe", infrared_safe_ty()),
        ("OPECoefficient", ope_coefficient_ty()),
        ("OperatorProductExpansion", operator_product_expansion_ty()),
        ("ConformalPrimary", conformal_primary_ty()),
        ("VirasoroGenerator", virasoro_generator_ty()),
        ("VirasoroAlgebraRelation", virasoro_algebra_relation_ty()),
        ("CentralCharge", central_charge_ty()),
        ("cTheorem", c_theorem_ty()),
        ("ModularInvariance", modular_invariance_ty()),
        ("ConformalBootstrap", conformal_bootstrap_ty()),
        ("ChernSimonsAction", chern_simons_action_ty()),
        ("BFAction", bf_action_ty()),
        ("TQFTFunctor", tqft_functor_ty()),
        ("LinkingNumber", linking_number_ty()),
        ("JonesPolynomial", jones_polynomial_ty()),
        ("LatticeAction", lattice_action_ty()),
        ("LatticeGreenFn", lattice_green_fn_ty()),
        ("InstantonSolution", instanton_solution_ty()),
        ("InstantonAction", instanton_action_ty()),
        ("SeibergWittenCurve", seiberg_witten_curve_ty()),
        ("PrepotentialSW", prepotential_sw_ty()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    env
}
/// Running coupling in φ⁴ theory at one loop.
/// g(μ) = g_0 / (1 - (3 g_0)/(16π²) ln(μ/μ_0))
pub fn running_coupling_phi4(g0: f64, mu: f64, mu0: f64) -> f64 {
    use std::f64::consts::PI;
    let log_ratio = (mu / mu0).ln();
    let denominator = 1.0 - (3.0 * g0) / (16.0 * PI * PI) * log_ratio;
    if denominator.abs() < 1e-15 {
        f64::INFINITY
    } else {
        g0 / denominator
    }
}
/// One-loop beta function for φ⁴ theory: β(g) = 3g²/(16π²).
pub fn beta_function_phi4(g: f64) -> f64 {
    use std::f64::consts::PI;
    3.0 * g * g / (16.0 * PI * PI)
}
/// Running coupling in QED: α(μ) with one-loop correction.
/// α(μ) = α_0 / (1 - (α_0/(3π)) ln(μ²/m_e²))
pub fn running_coupling_qed(alpha0: f64, mu: f64, m_electron: f64) -> f64 {
    use std::f64::consts::PI;
    let log_ratio = (mu * mu / (m_electron * m_electron)).ln();
    let denominator = 1.0 - (alpha0 / (3.0 * PI)) * log_ratio;
    if denominator.abs() < 1e-15 {
        f64::INFINITY
    } else {
        alpha0 / denominator
    }
}
/// Check the Ward identity k_μ M^μ = 0 for a photon amplitude.
/// Here M is represented as a 4-vector of complex amplitudes.
pub fn check_ward_identity(k: &[f64; 4], amplitude: &[QftComplex; 4]) -> f64 {
    let k_mu = k[0] * amplitude[0].re
        - k[1] * amplitude[1].re
        - k[2] * amplitude[2].re
        - k[3] * amplitude[3].re;
    k_mu.abs()
}
/// Check if a Hamiltonian matrix commutes with a discrete symmetry (CPT check).
/// H = 2×2 complex matrix represented as [[a,b],[c,d]].
pub fn check_cpt_invariance(h: &[[QftComplex; 2]; 2], theta: &[[QftComplex; 2]; 2]) -> bool {
    let mut th = [[QftComplex::zero(); 2]; 2];
    let mut ht = [[QftComplex::zero(); 2]; 2];
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                th[i][j] = th[i][j].add(&theta[i][k].mul(&h[k][j]));
                ht[i][j] = ht[i][j].add(&h[i][k].mul(&theta[k][j]));
            }
        }
    }
    for i in 0..2 {
        for j in 0..2 {
            if (th[i][j].re - ht[i][j].re).abs() > 1e-10 {
                return false;
            }
            if (th[i][j].im - ht[i][j].im).abs() > 1e-10 {
                return false;
            }
        }
    }
    true
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_build_env_succeeds() {
        let env = build_quantum_field_theory_env();
        let _ = env;
    }
    #[test]
    fn test_fock_vacuum_creation_annihilation() {
        let vac = FiniteFockSpace::vacuum(3, false);
        assert_eq!(vac.total_number(), 0);
        let (state1, norm1) = vac.create(1).expect("create should succeed");
        assert_eq!(state1.occupations[1], 1);
        assert!((norm1 - 1.0).abs() < 1e-12);
        let (state2, norm2) = state1.create(1).expect("create should succeed");
        assert_eq!(state2.occupations[1], 2);
        assert!((norm2 - 2.0f64.sqrt()).abs() < 1e-12);
        let (state3, norm3) = state2.annihilate(1).expect("annihilate should succeed");
        assert_eq!(state3.occupations[1], 1);
        assert!((norm3 - 2.0f64.sqrt()).abs() < 1e-12);
    }
    #[test]
    fn test_fermionic_pauli_exclusion() {
        let vac = FiniteFockSpace::vacuum(2, true);
        let (state1, _) = vac.create(0).expect("create should succeed");
        assert_eq!(state1.occupations[0], 1);
        assert!(state1.create(0).is_none());
    }
    #[test]
    fn test_vacuum_annihilation_gives_none() {
        let vac = FiniteFockSpace::vacuum(3, false);
        assert!(vac.annihilate(0).is_none());
        assert!(vac.annihilate(2).is_none());
    }
    #[test]
    fn test_klein_gordon_hamiltonian_zero_field() {
        let field = KleinGordonField::new(8, 0.25, 1.0);
        let h = field.hamiltonian();
        assert!(
            h.abs() < 1e-15,
            "Zero field should have zero Hamiltonian: {h}"
        );
    }
    #[test]
    fn test_klein_gordon_energy_conservation() {
        let mut field = KleinGordonField::new(16, 0.1, 0.5);
        field.phi[0] = 0.1;
        field.pi[0] = 0.0;
        let h0 = field.hamiltonian();
        for _ in 0..100 {
            field.step(0.01);
        }
        let h1 = field.hamiltonian();
        assert!(
            (h1 - h0).abs() / (h0 + 1e-30) < 5e-3,
            "Energy should be approximately conserved: h0={h0}, h1={h1}"
        );
    }
    #[test]
    fn test_feynman_diagram_loop_number() {
        let tree = FeynmanDiagram::new(4, 1, 0);
        assert_eq!(tree.loop_number(), 0);
        let one_loop = FeynmanDiagram::new(4, 2, 3);
        assert_eq!(one_loop.loop_number(), 2);
    }
    #[test]
    fn test_running_coupling_phi4_increases() {
        let g0 = 0.1;
        let mu0 = 1.0;
        let g_low = running_coupling_phi4(g0, 0.5, mu0);
        let g_high = running_coupling_phi4(g0, 2.0, mu0);
        assert!(g_high > g0, "φ⁴ coupling should grow with scale");
        assert!(g_low < g0, "φ⁴ coupling should decrease at lower scale");
    }
    #[test]
    fn test_brst_nilpotency_trivial() {
        let complex = BrstComplex::new(vec![2, 3, 2]);
        let v = vec![1.0, 0.5];
        assert!(complex.check_nilpotency(0, &v));
    }
    #[test]
    fn test_build_env_extended_axioms() {
        let env = build_quantum_field_theory_env();
        let _ = env;
    }
    #[test]
    fn test_feynman_evaluator_propagator_on_shell() {
        let eval = FeynmanDiagramEvaluator::new(0.1, 1.0);
        let prop = eval.propagator(2.0, 1e-3);
        let denom_sq = 1.0 + 1e-6;
        let expected_re = eval.propagator(2.0, 1e-3).re;
        assert!((expected_re - 1e-3 / denom_sq).abs() < 1e-10);
        assert!(prop.abs() > 0.0);
    }
    #[test]
    fn test_feynman_evaluator_vertex() {
        let eval = FeynmanDiagramEvaluator::new(0.5, 1.0);
        let v = eval.vertex();
        assert!((v.re).abs() < 1e-15);
        assert!((v.im + 0.5).abs() < 1e-15);
    }
    #[test]
    fn test_feynman_evaluator_tree_amplitude() {
        let eval = FeynmanDiagramEvaluator::new(0.1, 1.0);
        let m = eval.tree_amplitude_2to2();
        assert!((m.re).abs() < 1e-15);
        assert!((m.im + 0.1).abs() < 1e-15);
    }
    #[test]
    fn test_beta_function_rg_qcd() {
        let rg = BetaFunctionRG::new(3, 6, 0);
        assert!(rg.is_asymptotically_free());
        let b0 = rg.b0_coefficient();
        assert!((b0 - 7.0).abs() < 1e-10);
    }
    #[test]
    fn test_beta_function_rg_not_af() {
        let rg = BetaFunctionRG::new(3, 20, 0);
        assert!(!rg.is_asymptotically_free());
    }
    #[test]
    fn test_beta_function_rg_running_coupling_af() {
        let rg = BetaFunctionRG::new(3, 3, 0);
        assert!(rg.is_asymptotically_free());
        let g0 = 1.0;
        let g_high = rg.running_coupling(g0, 10.0, 1.0);
        let g_low = rg.running_coupling(g0, 0.1, 1.0);
        assert!(g_high < g0, "AF: coupling decreases at higher scale");
        assert!(g_low > g0, "AF: coupling increases at lower scale");
    }
    #[test]
    fn test_ising_lattice_ferromagnetic_energy() {
        let lat = CorrelationFunctionLattice::new_ferromagnetic(4, 0.5);
        let e = lat.energy();
        assert!((e + 32.0).abs() < 1e-10);
    }
    #[test]
    fn test_ising_lattice_magnetization() {
        let lat = CorrelationFunctionLattice::new_ferromagnetic(4, 0.5);
        let m = lat.magnetization();
        assert!((m - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_ising_lattice_two_point_correlation() {
        let lat = CorrelationFunctionLattice::new_ferromagnetic(4, 0.5);
        let corr = lat.two_point_correlation(2);
        assert!((corr - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_ising_lattice_metropolis_step() {
        let mut lat = CorrelationFunctionLattice::new_ferromagnetic(8, 10.0);
        let mut rng = 12345u64;
        for _ in 0..100 {
            lat.metropolis_step(&mut rng);
        }
        let m = lat.magnetization().abs();
        assert!(m > 0.5, "Expected high magnetization at low T: {m}");
    }
    #[test]
    fn test_ope_table_basic() {
        let mut ope = OpeTable::new();
        let identity = ope.add_operator("I", 0.0);
        let sigma = ope.add_operator("σ", 0.125);
        let epsilon = ope.add_operator("ε", 1.0);
        ope.set_coefficient(sigma, sigma, identity, 1.0);
        ope.set_coefficient(sigma, sigma, epsilon, 1.0);
        assert!((ope.get_coefficient(sigma, sigma, identity) - 1.0).abs() < 1e-15);
        assert!((ope.get_coefficient(sigma, sigma, epsilon) - 1.0).abs() < 1e-15);
        assert!((ope.get_coefficient(epsilon, epsilon, sigma)).abs() < 1e-15);
        assert!((ope.dimensions[identity]).abs() < 1e-15);
        assert!((ope.dimensions[sigma] - 0.125).abs() < 1e-15);
    }
    #[test]
    fn test_ope_crossing_residual_consistent() {
        let mut ope = OpeTable::new();
        let i = ope.add_operator("I", 0.0);
        let s = ope.add_operator("sigma", 0.125);
        let e = ope.add_operator("eps", 1.0);
        ope.set_coefficient(s, s, i, 1.0);
        ope.set_coefficient(s, s, e, 0.5);
        ope.set_coefficient(i, i, i, 1.0);
        let residual = ope.crossing_residual(s, s, i, i);
        assert!((residual - 1.0).abs() < 1e-12);
    }
    #[test]
    fn test_virasoro_commutator_basic() {
        let vir = VirasoroCommutator::new(1.0);
        let (coeff, idx, anomaly) = vir.commutator(2, -2);
        assert!((coeff - 4.0).abs() < 1e-12);
        assert_eq!(idx, 0);
        assert!((anomaly - 0.5).abs() < 1e-12);
    }
    #[test]
    fn test_virasoro_commutator_no_anomaly_offdiag() {
        let vir = VirasoroCommutator::new(26.0);
        let (coeff, idx, anomaly) = vir.commutator(1, 2);
        assert!((coeff - (-1.0)).abs() < 1e-12);
        assert_eq!(idx, 3);
        assert!(anomaly.abs() < 1e-15);
    }
    #[test]
    fn test_virasoro_jacobi_identity() {
        let vir = VirasoroCommutator::new(1.0);
        assert!(vir.check_jacobi(1, 2, -3));
        assert!(vir.check_jacobi(3, -1, -2));
        assert!(vir.check_jacobi(0, 1, -1));
    }
    #[test]
    fn test_virasoro_witt_limit() {
        let (coeff, idx) = VirasoroCommutator::witt_commutator(3, -1);
        assert!((coeff - 4.0).abs() < 1e-12);
        assert_eq!(idx, 2);
    }
    #[test]
    fn test_one_loop_bubble_approx() {
        let eval = FeynmanDiagramEvaluator::new(0.1, 1.0);
        let correction = eval.one_loop_bubble_approx(100.0);
        assert!(correction.abs() > 0.0);
    }
}
#[cfg(test)]
mod tests_qft_ext {
    use super::*;
    #[test]
    fn test_path_integral() {
        let mut pi = PathIntegralData::new("phi^4", 4);
        pi.add_coupling("lambda", 0.1);
        assert!(pi.power_counting_renormalizable(0.0));
        assert_eq!(pi.superficial_divergence(1, 2), 0);
        assert!(pi.partition_function_description().contains("phi^4"));
    }
    #[test]
    fn test_yang_mills_asymptotic_freedom() {
        let qcd = YangMillsTheory::new("SU(3)", 3, 1.0).with_flavors(6);
        assert!(qcd.asymptotically_free);
        assert!((qcd.beta0() - 7.0).abs() < 1e-10);
        assert_eq!(qcd.dual_coxeter_number(), 3);
    }
    #[test]
    fn test_yang_mills_not_af() {
        let qcd = YangMillsTheory::new("SU(2)", 2, 1.0).with_flavors(12);
        assert!(!qcd.asymptotically_free);
    }
    #[test]
    fn test_cft_ising() {
        let ising = ConformalFieldTheory2D::ising();
        assert!((ising.central_charge - 0.5).abs() < 1e-10);
        assert!(ising.is_rational);
        assert_eq!(ising.primary_operators.len(), 3);
        let exp = ising.character_exponent(0.0);
        assert!((exp + 0.5 / 24.0).abs() < 1e-10);
    }
}
#[cfg(test)]
mod tests_qft_ext2 {
    use super::*;
    #[test]
    fn test_brst_data() {
        let mut brst = BRSTData::new();
        brst.add_state("|photon, +>", 0);
        brst.add_state("|photon, ->", 0);
        brst.add_state("|ghost>", 1);
        brst.add_state("|antighost>", -1);
        assert!(brst.is_nilpotent());
        assert_eq!(brst.physical_ghost_zero().len(), 2);
        assert!(brst.cohomology_description().contains("2 physical"));
    }
    #[test]
    fn test_running_coupling() {
        let ym = YangMillsTheory::new("SU(3)", 3, 1.0).with_flavors(0);
        let g2 = ym.running_coupling_squared(10.0, 1.0);
        assert!(g2 < 1.0, "Running coupling should decrease: {g2}");
    }
}
#[cfg(test)]
mod tests_qft_ext3 {
    use super::*;
    #[test]
    fn test_scattering_amplitude() {
        let amp = ScatteringAmplitude::new("2→2 massless", 4)
            .with_mandelstam(100.0, -50.0, -50.0)
            .with_tree_amplitude(1.5);
        assert!(amp.crossing_satisfied(0.0, 1e-10));
        assert!(amp.parke_taylor_description().contains("4 gluons"));
    }
}
