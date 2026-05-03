//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    AdSCFTDictionary, CalabiyauHodge, Compactification, KnownCY3, MTheory, StrComplex,
    StringConfiguration, Superstring, TopologicalString, VenezianoAmplitudeCalc,
    VirasoroAlgebraExt, BPS,
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
/// Worldsheet : Type — the 2D worldsheet of the string (a Riemann surface Σ).
pub fn worldsheet_ty() -> Expr {
    type0()
}
/// TargetManifold : Type — the D-dimensional spacetime target manifold M.
pub fn target_manifold_ty() -> Expr {
    type0()
}
/// SigmaModelMap : Worldsheet → TargetManifold — an embedding X : Σ → M.
pub fn sigma_model_map_ty() -> Expr {
    arrow(cst("Worldsheet"), cst("TargetManifold"))
}
/// SigmaModelAction : SigmaModelMap → ℝ
/// S\[X\] = 1/(4πα') ∫_Σ d²σ √h h^{ab} ∂_a X^μ ∂_b X_μ
pub fn sigma_model_action_ty() -> Expr {
    arrow(arrow(cst("Worldsheet"), cst("TargetManifold")), real_ty())
}
/// MetricTensor : TargetManifold → (TangentVector → TangentVector → ℝ)
pub fn metric_tensor_ty() -> Expr {
    arrow(
        cst("TargetManifold"),
        arrow(cst("TangentVector"), arrow(cst("TangentVector"), real_ty())),
    )
}
/// WorldsheetMetric : Worldsheet → (2×2 symmetric tensor)
pub fn worldsheet_metric_ty() -> Expr {
    arrow(cst("Worldsheet"), cst("SymmetricTensor2"))
}
/// NambuGotoAction : SigmaModelMap → ℝ
/// S_NG = -T ∫_Σ d²σ √(-det(∂_a X^μ ∂_b X_μ))
/// where T = 1/(2πα') is the string tension.
pub fn nambu_goto_action_ty() -> Expr {
    arrow(arrow(cst("Worldsheet"), cst("TargetManifold")), real_ty())
}
/// StringTension : ℝ — T = 1/(2πα'), the fundamental string tension.
pub fn string_tension_ty() -> Expr {
    real_ty()
}
/// InducedMetric : SigmaModelMap → WorldsheetMetric
/// γ_{ab} = G_{μν} ∂_a X^μ ∂_b X^ν
pub fn induced_metric_ty() -> Expr {
    arrow(
        arrow(cst("Worldsheet"), cst("TargetManifold")),
        arrow(cst("Worldsheet"), cst("SymmetricTensor2")),
    )
}
/// NambuGotoEquivalence : Prop
/// Nambu-Goto action ≡ Polyakov action (classically)
pub fn nambu_goto_equivalence_ty() -> Expr {
    prop()
}
/// PolyakovAction : SigmaModelMap → WorldsheetMetric → ℝ
/// S_P = -T/2 ∫_Σ d²σ √h h^{ab} G_{μν} ∂_a X^μ ∂_b X^ν
pub fn polyakov_action_ty() -> Expr {
    arrow(
        arrow(cst("Worldsheet"), cst("TargetManifold")),
        arrow(arrow(cst("Worldsheet"), cst("SymmetricTensor2")), real_ty()),
    )
}
/// WeylInvariance : PolyakovAction → Prop
/// S_P is invariant under Weyl rescaling h_{ab} → e^{2φ} h_{ab}.
pub fn weyl_invariance_ty() -> Expr {
    arrow(
        arrow(
            arrow(cst("Worldsheet"), cst("TargetManifold")),
            arrow(arrow(cst("Worldsheet"), cst("SymmetricTensor2")), real_ty()),
        ),
        prop(),
    )
}
/// DiffInvariance : PolyakovAction → Prop
/// S_P is invariant under worldsheet diffeomorphisms.
pub fn diff_invariance_ty() -> Expr {
    arrow(
        arrow(
            arrow(cst("Worldsheet"), cst("TargetManifold")),
            arrow(arrow(cst("Worldsheet"), cst("SymmetricTensor2")), real_ty()),
        ),
        prop(),
    )
}
/// ConformalGauge : WorldsheetMetric — fix h_{ab} = e^{2φ} η_{ab}.
pub fn conformal_gauge_ty() -> Expr {
    arrow(real_ty(), arrow(cst("Worldsheet"), cst("SymmetricTensor2")))
}
/// CFTState : Type — a state in the CFT Hilbert space (radial quantization).
pub fn cft_state_ty() -> Expr {
    type0()
}
/// PrimaryField : ConformalWeight × ConformalWeight → (ℂ → CFTState)
/// A primary operator O(z, z̄) with weights (h, h̄).
pub fn primary_field_ty() -> Expr {
    arrow(
        real_ty(),
        arrow(real_ty(), arrow(complex_ty(), cst("CFTState"))),
    )
}
/// ConformalWeight : (h : ℝ) × (h̄ : ℝ) — the holomorphic and antiholomorphic weights.
pub fn conformal_weight_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), type0()))
}
/// ConformalTransformation : ℂ → ℂ — a holomorphic map z ↦ f(z).
pub fn conformal_transformation_ty() -> Expr {
    arrow(complex_ty(), complex_ty())
}
/// StressEnergyTensor : ℂ → CFTState — T(z) in CFT.
pub fn stress_energy_tensor_ty() -> Expr {
    arrow(complex_ty(), cst("CFTState"))
}
/// CentralCharge : ℝ — the Virasoro central charge c.
pub fn central_charge_ty() -> Expr {
    real_ty()
}
/// CriticalDimension : ℕ — the dimension where the Weyl anomaly vanishes.
/// For bosonic strings: D = 26; for superstrings: D = 10.
pub fn critical_dimension_ty() -> Expr {
    nat_ty()
}
/// OPECoefficient : PrimaryField → PrimaryField → PrimaryField → ℝ
/// C_{ijk} in the OPE: O_i(z) O_j(0) = Σ_k C_{ijk} z^{h_k - h_i - h_j} O_k(0)
pub fn ope_coefficient_ty() -> Expr {
    arrow(
        arrow(complex_ty(), cst("CFTState")),
        arrow(
            arrow(complex_ty(), cst("CFTState")),
            arrow(arrow(complex_ty(), cst("CFTState")), real_ty()),
        ),
    )
}
/// OPEExpansion : PrimaryField → PrimaryField → (ℂ → ℂ → CFTState)
/// O_i(z) O_j(w) = Σ_k C^k_{ij} (z-w)^{...} O_k(w)
pub fn ope_expansion_ty() -> Expr {
    arrow(
        arrow(complex_ty(), cst("CFTState")),
        arrow(
            arrow(complex_ty(), cst("CFTState")),
            arrow(complex_ty(), arrow(complex_ty(), cst("CFTState"))),
        ),
    )
}
/// TTbar_OPE : Prop
/// T(z) T(w) = c/2 / (z-w)^4 + 2T(w)/(z-w)^2 + ∂T(w)/(z-w) + ...
pub fn ttbar_ope_ty() -> Expr {
    prop()
}
/// OPEAssociativity : Prop — OPE is associative (bootstrap equation).
pub fn ope_associativity_ty() -> Expr {
    prop()
}
/// VirasoroGenerator : ℤ → (CFTState → CFTState) — L_n operators.
pub fn virasoro_generator_ty() -> Expr {
    arrow(int_ty(), arrow(cst("CFTState"), cst("CFTState")))
}
/// VirasoroCommutator : Prop
/// \[L_m, L_n\] = (m-n) L_{m+n} + c/12 m(m²-1) δ_{m+n,0}
pub fn virasoro_commutator_ty() -> Expr {
    prop()
}
/// L0Eigenvalue : CFTState → ℝ — the conformal dimension h is the L_0 eigenvalue.
pub fn l0_eigenvalue_ty() -> Expr {
    arrow(cst("CFTState"), real_ty())
}
/// PhysicalStateCondition : CFTState → Prop
/// (L_0 - 1)|phys⟩ = 0, L_n|phys⟩ = 0 for n > 0
pub fn physical_state_condition_ty() -> Expr {
    arrow(cst("CFTState"), prop())
}
/// NullState : CFTState → Prop — a state that decouples from all correlators.
pub fn null_state_ty() -> Expr {
    arrow(cst("CFTState"), prop())
}
/// CharacterFunction : ℝ → ℂ
/// χ(q) = Tr_{H}(q^{L_0 - c/24})
pub fn character_function_ty() -> Expr {
    arrow(real_ty(), complex_ty())
}
/// ModularInvariance : CharacterFunction → Prop
/// Z(τ) = Z(-1/τ) = Z(τ+1)
pub fn modular_invariance_ty() -> Expr {
    arrow(arrow(real_ty(), complex_ty()), prop())
}
/// VertexAlgebra : Type — a vertex algebra (V, Y, |0⟩, T).
pub fn vertex_algebra_ty() -> Expr {
    type0()
}
/// VertexOperator : VertexAlgebra → (ℂ → End(VertexAlgebra))
/// Y(a, z) = Σ_n a_{(n)} z^{-n-1}
pub fn vertex_operator_ty() -> Expr {
    arrow(
        cst("VertexAlgebra"),
        arrow(
            complex_ty(),
            arrow(cst("VertexAlgebra"), cst("VertexAlgebra")),
        ),
    )
}
/// TranslationOperator : VertexAlgebra → VertexAlgebra — T: the translation operator ∂.
pub fn translation_operator_ty() -> Expr {
    arrow(cst("VertexAlgebra"), cst("VertexAlgebra"))
}
/// VacuumVector : VertexAlgebra — the vacuum |0⟩ ∈ V.
pub fn vacuum_vector_ty() -> Expr {
    cst("VertexAlgebra")
}
/// LocalityAxiom : VertexAlgebra → Prop
/// (z-w)^N \[Y(a,z), Y(b,w)\] = 0 for N large enough.
pub fn locality_axiom_ty() -> Expr {
    arrow(cst("VertexAlgebra"), prop())
}
/// VertexAlgebraCreativity : VertexAlgebra → Prop
/// Y(a, z)|0⟩|_{z=0} = a  (state-field correspondence)
pub fn va_creativity_ty() -> Expr {
    arrow(cst("VertexAlgebra"), prop())
}
/// ConformalVertexAlgebra : VertexAlgebra → ℝ → Prop
/// A vertex algebra with a Virasoro element ω of central charge c.
pub fn conformal_vertex_algebra_ty() -> Expr {
    arrow(cst("VertexAlgebra"), arrow(real_ty(), prop()))
}
/// WorldsheetCFT : Type — the 2D CFT living on the string worldsheet.
pub fn worldsheet_cft_ty() -> Expr {
    type0()
}
/// ModeExpansion : SigmaModelMap → (ℤ → ℂ)
/// X^μ(σ,τ) = x^μ + α' p^μ τ + i√(α'/2) Σ_{n≠0} (1/n) α^μ_n e^{-in(τ±σ)}
pub fn mode_expansion_ty() -> Expr {
    arrow(
        arrow(cst("Worldsheet"), cst("TargetManifold")),
        arrow(int_ty(), complex_ty()),
    )
}
/// OscillatorMode : ℤ → (CFTState → CFTState) — α^μ_n oscillator.
pub fn oscillator_mode_ty() -> Expr {
    arrow(int_ty(), arrow(cst("CFTState"), cst("CFTState")))
}
/// OscillatorCommutator : Prop
/// \[α^μ_m, α^ν_n\] = m η^μν δ_{m+n,0}
pub fn oscillator_commutator_ty() -> Expr {
    prop()
}
/// MassShellCondition : CFTState → ℝ → Prop
/// M² = (2/α')(N - 1) for bosonic open string (N = number operator).
pub fn mass_shell_condition_ty() -> Expr {
    arrow(cst("CFTState"), arrow(real_ty(), prop()))
}
/// TachyonicGroundState : CFTState — the tachyonic ground state M² = -1/α'.
pub fn tachyonic_ground_state_ty() -> Expr {
    cst("CFTState")
}
/// MasslessVectorState : CFTState — the massless vector (graviton multiplet) state.
pub fn massless_vector_state_ty() -> Expr {
    cst("CFTState")
}
/// TDuality : TargetManifold → TargetManifold
/// T-duality along a circle: R ↦ α'/R.
pub fn t_duality_ty() -> Expr {
    arrow(cst("TargetManifold"), cst("TargetManifold"))
}
/// TDualRadius : ℝ → ℝ
/// R̃ = α' / R
pub fn t_dual_radius_ty() -> Expr {
    arrow(real_ty(), real_ty())
}
/// TDualityInvolution : Prop
/// (T-duality)² = identity
pub fn t_duality_involution_ty() -> Expr {
    prop()
}
/// WindingNumber : SigmaModelMap → ℤ
/// w ∈ ℤ: number of times the string winds around a compact direction.
pub fn winding_number_ty() -> Expr {
    arrow(arrow(cst("Worldsheet"), cst("TargetManifold")), int_ty())
}
/// MomentumWindingDuality : Prop
/// T-duality exchanges momentum and winding quantum numbers: n ↔ w, R ↔ α'/R.
pub fn momentum_winding_duality_ty() -> Expr {
    prop()
}
/// TDualityMassSpectrum : ℝ → ℤ → ℤ → ℝ
/// M² = (n/R)² + (wR/α')² + (2/α')(N_L + N_R - 2)
pub fn t_duality_mass_spectrum_ty() -> Expr {
    arrow(real_ty(), arrow(int_ty(), arrow(int_ty(), real_ty())))
}
/// DBrane : ℕ → Type — a Dp-brane: a p-dimensional hypersurface.
pub fn d_brane_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// BraneEmbedding : DBrane → TargetManifold — how the D-brane sits in spacetime.
pub fn brane_embedding_ty() -> Expr {
    arrow(arrow(nat_ty(), type0()), cst("TargetManifold"))
}
/// DirichletBoundaryCondition : SigmaModelMap → DBrane → Prop
/// ∂_n X^i = 0 for directions transverse to the brane.
pub fn dirichlet_bc_ty() -> Expr {
    arrow(
        arrow(cst("Worldsheet"), cst("TargetManifold")),
        arrow(arrow(nat_ty(), type0()), prop()),
    )
}
/// NeumannBoundaryCondition : SigmaModelMap → DBrane → Prop
/// ∂_σ X^i = 0 for directions along the brane.
pub fn neumann_bc_ty() -> Expr {
    arrow(
        arrow(cst("Worldsheet"), cst("TargetManifold")),
        arrow(arrow(nat_ty(), type0()), prop()),
    )
}
/// DBIAction : DBrane → ℝ
/// S_DBI = -T_p ∫ d^{p+1}ξ e^{-φ} √(-det(g + B + 2πα' F))
pub fn dbi_action_ty() -> Expr {
    arrow(arrow(nat_ty(), type0()), real_ty())
}
/// WorldvolumeGaugeField : DBrane → (Worldvolume → LieAlgebra)
/// The U(1) (or U(N)) gauge field living on the brane worldvolume.
pub fn worldvolume_gauge_field_ty() -> Expr {
    arrow(
        arrow(nat_ty(), type0()),
        arrow(cst("Worldvolume"), cst("LieAlgebra")),
    )
}
/// BraneCharge : DBrane → ℤ — the RR charge of a Dp-brane.
pub fn brane_charge_ty() -> Expr {
    arrow(arrow(nat_ty(), type0()), int_ty())
}
/// BraneIntersection : DBrane → DBrane → ℕ → Prop
/// Two branes intersect on a p-dimensional subspace.
pub fn brane_intersection_ty() -> Expr {
    arrow(
        arrow(nat_ty(), type0()),
        arrow(arrow(nat_ty(), type0()), arrow(nat_ty(), prop())),
    )
}
/// CalabiYauManifold : ℕ → Type — a complex n-fold with SU(n) holonomy.
pub fn calabi_yau_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// HolomorphicForm : CalabiYauManifold → Type
/// Ω ∈ H^{n,0}(CY) — the unique holomorphic n-form.
pub fn holomorphic_form_ty() -> Expr {
    arrow(arrow(nat_ty(), type0()), type0())
}
/// KahlerForm : CalabiYauManifold → Type
/// J ∈ H^{1,1}(CY) — the Kähler (1,1)-form.
pub fn kahler_form_ty() -> Expr {
    arrow(arrow(nat_ty(), type0()), type0())
}
/// HodgeDiamond : CalabiYauManifold → (ℕ → ℕ → ℕ)
/// h^{p,q}(CY) — the Hodge numbers.
pub fn hodge_diamond_ty() -> Expr {
    arrow(
        arrow(nat_ty(), type0()),
        arrow(nat_ty(), arrow(nat_ty(), nat_ty())),
    )
}
/// RicciFlat : CalabiYauManifold → Prop — Ric(g) = 0.
pub fn ricci_flat_ty() -> Expr {
    arrow(arrow(nat_ty(), type0()), prop())
}
/// CYTopologicalData : CalabiYauManifold → (ℕ × ℕ) — (h^{1,1}, h^{2,1}).
pub fn cy_topological_data_ty() -> Expr {
    arrow(arrow(nat_ty(), type0()), arrow(nat_ty(), nat_ty()))
}
/// EulerCharacteristic : CalabiYauManifold → ℤ
/// χ = 2(h^{1,1} - h^{2,1}) for a CY3.
pub fn euler_characteristic_cy_ty() -> Expr {
    arrow(arrow(nat_ty(), type0()), int_ty())
}
/// MirrorManifold : CalabiYauManifold → CalabiYauManifold
/// The mirror CY: h^{1,1}(M) = h^{2,1}(W), h^{2,1}(M) = h^{1,1}(W).
pub fn mirror_manifold_ty() -> Expr {
    arrow(arrow(nat_ty(), type0()), arrow(nat_ty(), type0()))
}
/// MirrorSymmetry : CalabiYauManifold → CalabiYauManifold → Prop
pub fn mirror_symmetry_ty() -> Expr {
    arrow(
        arrow(nat_ty(), type0()),
        arrow(arrow(nat_ty(), type0()), prop()),
    )
}
/// HodgeNumberExchange : Prop
/// Mirror symmetry exchanges h^{1,1} ↔ h^{2,1}.
pub fn hodge_number_exchange_ty() -> Expr {
    prop()
}
/// GromovWittenInvariant : CalabiYauManifold → ℕ → ℤ
/// GW invariant counting rational curves of degree d on CY.
pub fn gromov_witten_ty() -> Expr {
    arrow(arrow(nat_ty(), type0()), arrow(nat_ty(), int_ty()))
}
/// PeriodIntegrals : CalabiYauManifold → (Cycle → Complex)
/// ∫_γ Ω — period integrals of the holomorphic form.
pub fn period_integrals_ty() -> Expr {
    arrow(arrow(nat_ty(), type0()), arrow(cst("Cycle"), complex_ty()))
}
/// PicardFuchsEquation : CalabiYauManifold → Prop
/// The differential equation satisfied by period integrals.
pub fn picard_fuchs_ty() -> Expr {
    arrow(arrow(nat_ty(), type0()), prop())
}
/// StringDuality : TargetManifold → TargetManifold → Prop
/// Two string theories are dual if they are equivalent at the quantum level.
pub fn string_duality_ty() -> Expr {
    arrow(cst("TargetManifold"), arrow(cst("TargetManifold"), prop()))
}
/// Register all string theory axioms into the environment.
pub fn build_string_theory_env() -> Environment {
    let mut env = Environment::new();
    let axioms: &[(&str, Expr)] = &[
        ("Worldsheet", worldsheet_ty()),
        ("TargetManifold", target_manifold_ty()),
        ("TangentVector", type0()),
        ("SymmetricTensor2", type0()),
        ("CFTState", cft_state_ty()),
        ("VertexAlgebra", vertex_algebra_ty()),
        ("Worldvolume", type0()),
        ("LieAlgebra", type0()),
        ("Cycle", type0()),
        ("SigmaModelMap", sigma_model_map_ty()),
        ("SigmaModelAction", sigma_model_action_ty()),
        ("MetricTensor", metric_tensor_ty()),
        ("WorldsheetMetric", worldsheet_metric_ty()),
        ("NambuGotoAction", nambu_goto_action_ty()),
        ("StringTension", string_tension_ty()),
        ("InducedMetric", induced_metric_ty()),
        ("nambu_goto_equivalence", nambu_goto_equivalence_ty()),
        ("PolyakovAction", polyakov_action_ty()),
        ("weyl_invariance", weyl_invariance_ty()),
        ("diff_invariance", diff_invariance_ty()),
        ("ConformalGauge", conformal_gauge_ty()),
        ("PrimaryField", primary_field_ty()),
        ("ConformalWeight", conformal_weight_ty()),
        ("ConformalTransformation", conformal_transformation_ty()),
        ("StressEnergyTensor", stress_energy_tensor_ty()),
        ("CentralCharge", central_charge_ty()),
        ("CriticalDimension", critical_dimension_ty()),
        ("OPECoefficient", ope_coefficient_ty()),
        ("OPEExpansion", ope_expansion_ty()),
        ("ttbar_ope", ttbar_ope_ty()),
        ("ope_associativity", ope_associativity_ty()),
        ("VirasoroGenerator", virasoro_generator_ty()),
        ("virasoro_commutator", virasoro_commutator_ty()),
        ("L0Eigenvalue", l0_eigenvalue_ty()),
        ("PhysicalStateCondition", physical_state_condition_ty()),
        ("NullState", null_state_ty()),
        ("CharacterFunction", character_function_ty()),
        ("modular_invariance", modular_invariance_ty()),
        ("VertexOperator", vertex_operator_ty()),
        ("TranslationOperator", translation_operator_ty()),
        ("VacuumVector", vacuum_vector_ty()),
        ("locality_axiom", locality_axiom_ty()),
        ("va_creativity", va_creativity_ty()),
        ("ConformalVertexAlgebra", conformal_vertex_algebra_ty()),
        ("WorldsheetCFT", worldsheet_cft_ty()),
        ("ModeExpansion", mode_expansion_ty()),
        ("OscillatorMode", oscillator_mode_ty()),
        ("oscillator_commutator", oscillator_commutator_ty()),
        ("MassShellCondition", mass_shell_condition_ty()),
        ("TachyonicGroundState", tachyonic_ground_state_ty()),
        ("MasslessVectorState", massless_vector_state_ty()),
        ("TDuality", t_duality_ty()),
        ("TDualRadius", t_dual_radius_ty()),
        ("t_duality_involution", t_duality_involution_ty()),
        ("WindingNumber", winding_number_ty()),
        ("momentum_winding_duality", momentum_winding_duality_ty()),
        ("TDualityMassSpectrum", t_duality_mass_spectrum_ty()),
        ("DBrane", d_brane_ty()),
        ("BraneEmbedding", brane_embedding_ty()),
        ("DirichletBC", dirichlet_bc_ty()),
        ("NeumannBC", neumann_bc_ty()),
        ("DBIAction", dbi_action_ty()),
        ("WorldvolumeGaugeField", worldvolume_gauge_field_ty()),
        ("BraneCharge", brane_charge_ty()),
        ("BraneIntersection", brane_intersection_ty()),
        ("CalabiYauManifold", calabi_yau_ty()),
        ("HolomorphicForm", holomorphic_form_ty()),
        ("KahlerForm", kahler_form_ty()),
        ("HodgeDiamond", hodge_diamond_ty()),
        ("ricci_flat", ricci_flat_ty()),
        ("CYTopologicalData", cy_topological_data_ty()),
        ("EulerCharacteristicCY", euler_characteristic_cy_ty()),
        ("MirrorManifold", mirror_manifold_ty()),
        ("mirror_symmetry", mirror_symmetry_ty()),
        ("hodge_number_exchange", hodge_number_exchange_ty()),
        ("GromovWittenInvariant", gromov_witten_ty()),
        ("PeriodIntegrals", period_integrals_ty()),
        ("picard_fuchs_equation", picard_fuchs_ty()),
        ("string_duality", string_duality_ty()),
        ("AlphaPrime", real_ty()),
        ("StringLength", real_ty()),
        ("PlanckLength", real_ty()),
        ("StringCoupling", real_ty()),
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
/// Check the Virasoro algebra relation for integer modes.
/// Returns the value of \[L_m, L_n\] - ((m-n)L_{m+n} + c/12 m(m²-1) δ_{m+n,0})
/// acting on a single mode, modeled as real numbers (eigenvalue sector).
pub fn virasoro_commutator_residual(m: i64, n: i64, lm: f64, ln: f64, l_mpn: f64, c: f64) -> f64 {
    // For commuting real-valued eigenvalue scalars, [L_m, L_n] = 0
    #[allow(clippy::eq_op)]
    let commutator = lm * ln - ln * lm;
    let rhs_algebraic = ((m - n) as f64) * l_mpn;
    let central = if m + n == 0 {
        c / 12.0 * (m as f64) * ((m * m - 1) as f64)
    } else {
        0.0
    };
    commutator - rhs_algebraic - central
}
/// Compute the T-dual radius: R̃ = α' / R.
pub fn t_dual_radius(alpha_prime: f64, radius: f64) -> f64 {
    alpha_prime / radius
}
/// Compute the string mass spectrum for a string on a circle.
/// M² = (n/R)² + (wR/α')² + (2/α')(N - 1)
/// where n = momentum, w = winding, N = oscillator level.
pub fn string_mass_sq(n: i64, w: i64, n_level: u64, radius: f64, alpha_prime: f64) -> f64 {
    let momentum_term = (n as f64 / radius).powi(2);
    let winding_term = (w as f64 * radius / alpha_prime).powi(2);
    let oscillator_term = 2.0 / alpha_prime * (n_level as f64 - 1.0);
    momentum_term + winding_term + oscillator_term
}
/// Euler characteristic of a Calabi-Yau threefold: χ = 2(h^{1,1} - h^{2,1}).
pub fn cy3_euler_characteristic(h11: i64, h21: i64) -> i64 {
    2 * (h11 - h21)
}
/// Check that a pair (h11, h21) satisfies mirror symmetry (exchanges the two numbers).
pub fn is_mirror_pair(h11_a: u64, h21_a: u64, h11_b: u64, h21_b: u64) -> bool {
    h11_a == h21_b && h21_a == h11_b
}
/// Compute the OPE coefficient for two primary fields in a free CFT.
/// For free boson: ∂X(z) ∂X(w) ~ -α'/(2(z-w)²).
pub fn free_boson_ope_coefficient(z: StrComplex, w: StrComplex, alpha_prime: f64) -> StrComplex {
    let dz = StrComplex {
        re: z.re - w.re,
        im: z.im - w.im,
    };
    let dz_sq = dz.mul(&dz);
    let norm_sq = dz_sq.abs_sq();
    if norm_sq < 1e-30 {
        return StrComplex::new(f64::INFINITY, 0.0);
    }
    let coeff = -alpha_prime / 2.0;
    StrComplex {
        re: coeff * dz_sq.re / norm_sq,
        im: coeff * (-dz_sq.im) / norm_sq,
    }
}
/// Stress tensor OPE: T(z) T(w) ~ c/2/(z-w)^4 + 2T(w)/(z-w)^2 + ∂T(w)/(z-w).
/// Returns the coefficient of 1/(z-w)^4 term (= c/2).
pub fn stress_tensor_ope_leading(c: f64) -> f64 {
    c / 2.0
}
/// Map from cylinder (σ, τ) to complex plane: z = e^{τ + iσ}.
pub fn cylinder_to_plane(sigma: f64, tau: f64) -> StrComplex {
    let r = tau.exp();
    StrComplex {
        re: r * sigma.cos(),
        im: r * sigma.sin(),
    }
}
/// Map from complex plane back to cylinder: τ = ln|z|, σ = arg(z).
pub fn plane_to_cylinder(z: StrComplex) -> (f64, f64) {
    let tau = z.abs().ln();
    let sigma = z.im.atan2(z.re);
    (sigma, tau)
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;
    #[test]
    fn test_build_env_succeeds() {
        let env = build_string_theory_env();
        let _ = env;
    }
    #[test]
    fn test_t_dual_radius_involution() {
        let alpha_prime = 1.0;
        let r = 2.0;
        let r_dual = t_dual_radius(alpha_prime, r);
        let r_double_dual = t_dual_radius(alpha_prime, r_dual);
        assert!(
            (r_double_dual - r).abs() < 1e-14,
            "T-duality should be an involution: R̃̃ = R"
        );
    }
    #[test]
    fn test_t_duality_self_dual_radius() {
        let alpha_prime: f64 = 2.0;
        let r_self_dual = alpha_prime.sqrt();
        let r_dual = t_dual_radius(alpha_prime, r_self_dual);
        assert!(
            (r_dual - r_self_dual).abs() < 1e-14,
            "Self-dual radius should satisfy R = √α'"
        );
    }
    #[test]
    fn test_string_mass_spectrum_tachyon() {
        let alpha_prime = 1.0;
        let m_sq = string_mass_sq(0, 0, 0, 1.0, alpha_prime);
        assert!(
            (m_sq - (-2.0)).abs() < 1e-12,
            "Tachyon M² should be -2/α' = -2; got {m_sq}"
        );
    }
    #[test]
    fn test_string_mass_massless_state() {
        let alpha_prime = 1.0;
        let m_sq = string_mass_sq(0, 0, 1, 1.0, alpha_prime);
        assert!(
            m_sq.abs() < 1e-12,
            "Massless state should have M² = 0; got {m_sq}"
        );
    }
    #[test]
    fn test_cy3_euler_characteristic_quintic() {
        let quintic = KnownCY3::quintic();
        let chi = quintic.euler_characteristic();
        assert_eq!(chi, -200, "Quintic CY3 should have χ = -200");
    }
    #[test]
    fn test_mirror_symmetry_quintic() {
        let quintic = KnownCY3::quintic();
        let mirror = KnownCY3::mirror_quintic();
        assert!(
            is_mirror_pair(quintic.h11, quintic.h21, mirror.h11, mirror.h21),
            "Quintic and mirror quintic should be a mirror pair"
        );
        assert_eq!(
            quintic.euler_characteristic(),
            -mirror.euler_characteristic()
        );
    }
    #[test]
    fn test_cylinder_plane_round_trip() {
        let sigma = 1.2;
        let tau = 0.5;
        let z = cylinder_to_plane(sigma, tau);
        let (s2, t2) = plane_to_cylinder(z);
        let ds = (s2 - sigma).abs();
        let dt = (t2 - tau).abs();
        assert!(
            dt < 1e-12,
            "tau round-trip failed: got {t2}, expected {tau}"
        );
        assert!(
            ds < 1e-12 || (ds - 2.0 * PI).abs() < 1e-12,
            "sigma round-trip failed: got {s2}, expected {sigma}"
        );
    }
    #[test]
    fn test_nambu_goto_action_straight_string() {
        let tension = 1.0 / (2.0 * PI);
        let n = 100;
        let d_sigma = 2.0 * PI / n as f64;
        let string = StringConfiguration::straight(4, n, d_sigma);
        let action = string.nambu_goto_action(tension);
        let expected_length = (n - 1) as f64 * d_sigma;
        assert!(
            (action - (-tension * expected_length)).abs() < 1e-10,
            "Straight string action incorrect: got {action}"
        );
    }
}
/// WorldsheetCFT_CentralCharge : ℝ — the central charge c of the worldsheet CFT.
/// For bosonic strings: c = D = 26; for superstrings: c = 3D/2 = 15.
pub fn worldsheet_cft_central_charge_ty() -> Expr {
    real_ty()
}
/// OPEAlgebra : WorldsheetCFT → Type
/// The operator product expansion algebra of a 2D CFT on the worldsheet.
pub fn ope_algebra_ty() -> Expr {
    arrow(cst("WorldsheetCFT"), type0())
}
/// StressTensorTT_OPE : WorldsheetCFT → Prop
/// T(z) T(w) = c/2/(z-w)^4 + 2T(w)/(z-w)^2 + ∂T(w)/(z-w)
/// The fundamental OPE relation for the stress tensor.
pub fn stress_tensor_tt_ope_ty() -> Expr {
    arrow(cst("WorldsheetCFT"), prop())
}
/// PrimaryFieldOPE : WorldsheetCFT → ℝ → Prop
/// T(z) O(w,w̄) = h O(w,w̄)/(z-w)^2 + ∂_w O(w,w̄)/(z-w) for primary of weight h.
pub fn primary_field_ope_ty() -> Expr {
    arrow(cst("WorldsheetCFT"), arrow(real_ty(), prop()))
}
/// DescendantField : CFTState → ℕ → CFTState
/// L_{-n₁}...L_{-nk}|h⟩: a descendant field at level N above a primary.
pub fn descendant_field_ty() -> Expr {
    arrow(cst("CFTState"), arrow(nat_ty(), cst("CFTState")))
}
/// ConformalBlock : CFTState → CFTState → CFTState → CFTState → ℂ → ℝ
/// A conformal block F_{h_p}(z) for the 4-point function of primaries.
pub fn conformal_block_ty() -> Expr {
    arrow(
        cst("CFTState"),
        arrow(
            cst("CFTState"),
            arrow(
                cst("CFTState"),
                arrow(cst("CFTState"), arrow(complex_ty(), real_ty())),
            ),
        ),
    )
}
/// CrossingSymmetry : WorldsheetCFT → Prop
/// Consistency of OPE in different channels: bootstrap equation.
pub fn crossing_symmetry_ty() -> Expr {
    arrow(cst("WorldsheetCFT"), prop())
}
/// VenezianoAmplitude : ℝ → ℝ → ℝ → ℝ
/// A(s,t,u) = Γ(-α(s)) Γ(-α(t)) / Γ(-α(s)-α(t))
/// where α(s) = 1 + α's/2 is the Regge trajectory.
pub fn veneziano_amplitude_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), real_ty())))
}
/// VirasoroShapiroAmplitude : ℝ → ℝ → ℝ
/// The closed-string analog of the Veneziano amplitude.
/// A(s,t) = (Γ(-α's/4) Γ(-α't/4) Γ(-α'u/4)) / (Γ(1+α's/4) Γ(1+α't/4) Γ(1+α'u/4))
pub fn virasoro_shapiro_amplitude_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
/// SuperstringVertexOperator : Superstring → ℤ → (ℂ → CFTState)
/// A picture-number q vertex operator V_q(z) in superstring theory.
pub fn superstring_vertex_operator_ty() -> Expr {
    arrow(
        cst("SuperstringType"),
        arrow(int_ty(), arrow(complex_ty(), cst("CFTState"))),
    )
}
/// SuperstringAmplitude : Superstring → ℕ → ℕ → ℝ
/// An n-point amplitude at genus g in the given superstring theory.
pub fn superstring_amplitude_ty() -> Expr {
    arrow(
        cst("SuperstringType"),
        arrow(nat_ty(), arrow(nat_ty(), real_ty())),
    )
}
/// ReggeTrajectory : ℝ → ℝ → ℝ
/// α(s) = α₀ + α' s — the linear Regge trajectory.
pub fn regge_trajectory_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
/// SuperstringMassSpectrum : Superstring → ℕ → ℝ
/// M² = (2/α') (N - a) where a = 1 (bosonic) or 1/2 (superstring NS).
pub fn superstring_mass_spectrum_ty() -> Expr {
    arrow(cst("SuperstringType"), arrow(nat_ty(), real_ty()))
}
/// RRCharge : DBrane → ℝ
/// The Ramond-Ramond charge carried by a Dp-brane couples to the (p+1)-form C_{p+1}.
pub fn rr_charge_ty() -> Expr {
    arrow(arrow(nat_ty(), type0()), real_ty())
}
/// DBraneWorldvolume : DBrane → ℕ → TargetManifold
/// The (p+1)-dimensional worldvolume of a Dp-brane.
pub fn d_brane_worldvolume_ty() -> Expr {
    arrow(
        arrow(nat_ty(), type0()),
        arrow(nat_ty(), cst("TargetManifold")),
    )
}
/// OpenStringEndpoint : DBrane → Worldsheet → Prop
/// Dirichlet boundary condition: open string endpoint lies on the brane.
pub fn open_string_endpoint_ty() -> Expr {
    arrow(arrow(nat_ty(), type0()), arrow(cst("Worldsheet"), prop()))
}
/// BraneAntibraneAnnihilation : DBrane → DBrane → Prop
/// A Dp-brane and anti-Dp-brane annihilate and release energy.
pub fn brane_antibrane_annihilation_ty() -> Expr {
    arrow(
        arrow(nat_ty(), type0()),
        arrow(arrow(nat_ty(), type0()), prop()),
    )
}
/// KTheoryCharge : DBrane → ℤ
/// D-brane charges classified by K-theory: K(X) for type IIB, K^1(X) for IIA.
pub fn k_theory_charge_ty() -> Expr {
    arrow(arrow(nat_ty(), type0()), int_ty())
}
/// TDualityGroup : ℕ → Type
/// O(d,d;ℤ): the T-duality group for compactification on T^d.
pub fn t_duality_group_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// WindingMode : SigmaModelMap → ℤ → ℝ
/// Winding mode contribution to the energy: (wR/α')².
pub fn winding_mode_ty() -> Expr {
    arrow(
        arrow(cst("Worldsheet"), cst("TargetManifold")),
        arrow(int_ty(), real_ty()),
    )
}
/// NaivenessOfMomentum : ℤ → ℝ → ℝ
/// Momentum mode energy: (n/R)².
pub fn momentum_mode_ty() -> Expr {
    arrow(int_ty(), arrow(real_ty(), real_ty()))
}
/// TDualityInvariantSpectrum : Prop
/// The full string spectrum (including winding modes) is invariant under T-duality.
pub fn t_duality_invariant_spectrum_ty() -> Expr {
    prop()
}
/// BusherRules : Prop
/// Buscher transformation rules for the metric, B-field, and dilaton under T-duality.
pub fn buscher_rules_ty() -> Expr {
    prop()
}
/// MirrorSymmetryTA : CalabiYauManifold → CalabiYauManifold → Prop
/// Mirror symmetry as T-duality along the fibers of a special Lagrangian fibration (SYZ).
pub fn mirror_syz_ty() -> Expr {
    arrow(
        arrow(nat_ty(), type0()),
        arrow(arrow(nat_ty(), type0()), prop()),
    )
}
/// SDuality : SuperstringType → SuperstringType → Prop
/// Strong-weak coupling duality g_s ↦ 1/g_s.
pub fn s_duality_ty() -> Expr {
    arrow(
        cst("SuperstringType"),
        arrow(cst("SuperstringType"), prop()),
    )
}
/// MontonenOliveDuality : Prop
/// Electric-magnetic duality in N=4 SYM: g ↦ 4π/g.
pub fn montonen_olive_duality_ty() -> Expr {
    prop()
}
/// SL2Z_Action : ℝ → ℂ → ℂ
/// The SL(2,ℤ) action on the axio-dilaton τ = χ + ie^{-φ}.
pub fn sl2z_action_ty() -> Expr {
    arrow(real_ty(), arrow(complex_ty(), complex_ty()))
}
/// SL2Z_Duality : Prop
/// Type IIB has an exact SL(2,ℤ) symmetry acting on (F₁, D1) and (NS5, D5) branes.
pub fn sl2z_duality_ty() -> Expr {
    prop()
}
/// SDualityBPSSpectrum : Prop
/// S-duality maps BPS particles to BPS particles (protects spectrum).
pub fn s_duality_bps_spectrum_ty() -> Expr {
    prop()
}
/// HolographicPrinciple : Prop
/// The bulk physics in AdS_{d+1} is encoded in the CFT_d on the boundary.
pub fn holographic_principle_ty() -> Expr {
    prop()
}
/// MaldacenaConjecture : Prop
/// AdS₅/CFT₄: N=4 SYM is dual to Type IIB on AdS₅×S⁵.
pub fn maldacena_conjecture_ty() -> Expr {
    prop()
}
/// HolographicDictionary : ℝ → ℝ → ℝ
/// Δ(m) = d/2 + √((d/2)² + m²L²): bulk mass ↦ boundary conformal dimension.
pub fn holographic_dictionary_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
/// WilsonLoop_AdS : ℝ → ℝ
/// Holographic Wilson loop: W\[C\] = exp(-S_string\[C\]) for a string ending on C.
pub fn wilson_loop_ads_ty() -> Expr {
    arrow(real_ty(), real_ty())
}
/// HolographicEntanglement : ℝ → ℝ
/// Ryu-Takayanagi formula: S_EE = Area(γ)/(4G_N).
pub fn holographic_entanglement_ty() -> Expr {
    arrow(real_ty(), real_ty())
}
/// AdSRadius_Duality : ℝ → ℝ → ℝ
/// L⁴ = 4π g_s N α'^2: AdS radius in terms of string coupling and flux.
pub fn ads_radius_duality_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
/// CYModuliSpace : CalabiYauManifold → Type
/// The moduli space M = M_K × M_cs of Kähler and complex structure deformations.
pub fn cy_moduli_space_ty() -> Expr {
    arrow(arrow(nat_ty(), type0()), type0())
}
/// KahlerModuli : CalabiYauManifold → ℕ → ℝ
/// The h^{1,1} Kähler moduli t^a parameterizing J = t^a J_a.
pub fn kahler_moduli_ty() -> Expr {
    arrow(arrow(nat_ty(), type0()), arrow(nat_ty(), real_ty()))
}
/// ComplexStructureModuli : CalabiYauManifold → ℕ → ℂ
/// The h^{2,1} complex structure moduli z^i.
pub fn complex_structure_moduli_ty() -> Expr {
    arrow(arrow(nat_ty(), type0()), arrow(nat_ty(), complex_ty()))
}
/// SpecialGeometry : CYModuliSpace → Prop
/// The Kähler potential on complex structure moduli is determined by prepotential F.
pub fn special_geometry_ty() -> Expr {
    arrow(type0(), prop())
}
/// GVInvariant : CalabiYauManifold → ℕ → ℤ
/// Gopakumar-Vafa BPS invariants n_g^β counting BPS states of genus g and curve class β.
pub fn gopakumar_vafa_ty() -> Expr {
    arrow(arrow(nat_ty(), type0()), arrow(nat_ty(), int_ty()))
}
/// MTheoryDimension : ℕ — 11 dimensions.
pub fn m_theory_dimension_ty() -> Expr {
    nat_ty()
}
/// MTheorySupergravity : Type — 11D supergravity: the low-energy limit of M-theory.
pub fn m_theory_supergravity_ty() -> Expr {
    type0()
}
/// M2Brane : ℕ → Type — the M2-brane: a fundamental 2+1 dimensional object in M-theory.
pub fn m2_brane_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// M5Brane : ℕ → Type — the M5-brane: a 5+1 dimensional object dual to M2.
pub fn m5_brane_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// MTheoryCompactification : MTheorySupergravity → ℕ → TargetManifold
/// Compactification of M-theory on a (11-d)-dimensional internal manifold.
pub fn m_theory_compactification_ty() -> Expr {
    arrow(type0(), arrow(nat_ty(), cst("TargetManifold")))
}
/// MTheoryDualityWeb : Prop
/// All five 10D superstring theories are limits of M-theory.
pub fn m_theory_duality_web_ty() -> Expr {
    prop()
}
/// HoravaMolten : Prop
/// M-theory on S¹/ℤ₂ gives 11D Horava-Witten theory (E₈×E₈ at the boundaries).
pub fn horava_molten_ty() -> Expr {
    prop()
}
/// StringFieldTheory : Type — the field-theoretic formulation of string theory.
pub fn string_field_theory_ty() -> Expr {
    type0()
}
/// BRSTCohomology : StringFieldTheory → Type
/// Q_B²=0: the BRST charge Q_B generates the gauge symmetry of SFT.
pub fn brst_cohomology_ty() -> Expr {
    arrow(cst("StringFieldTheory"), type0())
}
/// StringVertex : StringField → StringField → StringField → ℝ
/// The cubic (and higher) interaction vertex ⟨V₃|⟩ in Witten's open SFT.
pub fn string_vertex_ty() -> Expr {
    arrow(
        cst("StringField"),
        arrow(cst("StringField"), arrow(cst("StringField"), real_ty())),
    )
}
/// StarProduct : StringField → StringField → StringField
/// Midpoint product: Ψ * Φ in Witten's open string field theory.
pub fn star_product_ty() -> Expr {
    arrow(
        cst("StringField"),
        arrow(cst("StringField"), cst("StringField")),
    )
}
/// WittenSFT_Action : StringField → ℝ
/// S = -(1/g²)(1/2⟨Ψ,Q_BΨ⟩ + 1/3⟨Ψ,Ψ*Ψ⟩)
pub fn witten_sft_action_ty() -> Expr {
    arrow(cst("StringField"), real_ty())
}
/// TachyonCondensation : StringField → Prop
/// The tachyon potential minimum: Sen's conjecture (open string vacuum = nothing).
pub fn tachyon_condensation_ty() -> Expr {
    arrow(cst("StringField"), prop())
}
/// AModelTwist : WorldsheetCFT → Type — the A-twist giving the topological A-model.
pub fn a_model_twist_ty() -> Expr {
    arrow(cst("WorldsheetCFT"), type0())
}
/// BModelTwist : WorldsheetCFT → Type — the B-twist giving the topological B-model.
pub fn b_model_twist_ty() -> Expr {
    arrow(cst("WorldsheetCFT"), type0())
}
/// GromovWittenPotential : CalabiYauManifold → ℝ
/// The generating function F_g(t) of genus-g GW invariants (A-model free energy).
pub fn gromov_witten_potential_ty() -> Expr {
    arrow(arrow(nat_ty(), type0()), real_ty())
}
/// BModelFreeEnergy : CalabiYauManifold → ℝ
/// The holomorphic anomaly equation governs B-model free energies F_g.
pub fn b_model_free_energy_ty() -> Expr {
    arrow(arrow(nat_ty(), type0()), real_ty())
}
/// HolomorphicAnomalyEquation : Prop
/// BCOV holomorphic anomaly equation: ∂̄_ī F_g = (1/2) C̄ī^{jk}(D_j D_k F_{g-1} + ...).
pub fn holomorphic_anomaly_equation_ty() -> Expr {
    prop()
}
/// TopologicalAmplitude : ℕ → ℝ
/// F_g: the genus-g topological string amplitude.
pub fn topological_amplitude_ty() -> Expr {
    arrow(nat_ty(), real_ty())
}
/// SwamplandDistanceConjecture : Prop
/// As φ → ∞ in moduli space, an infinite tower of states becomes exponentially light:
/// m ~ e^{-α·d(φ₀,φ)}.
pub fn swampland_distance_conjecture_ty() -> Expr {
    prop()
}
/// DesSitterConjecture : Prop
/// |∇V| ≥ c·V in any consistent quantum gravity theory (no stable de Sitter vacua).
pub fn de_sitter_conjecture_ty() -> Expr {
    prop()
}
/// WeakGravityConjecture : Prop
/// For every U(1) gauge field, there exists a particle with q/m ≥ 1 (in Planck units).
pub fn weak_gravity_conjecture_ty() -> Expr {
    prop()
}
/// SwamplandConjecturesConsistency : Prop
/// All three swampland conjectures are mutually consistent.
pub fn swampland_consistency_ty() -> Expr {
    prop()
}
/// LandscapeVsSwampland : Prop
/// A string vacuum lies in the landscape iff it satisfies all swampland criteria.
pub fn landscape_vs_swampland_ty() -> Expr {
    prop()
}
/// MatrixModel : ℕ → Type — a matrix model description of string theory.
/// BFSS matrix model: M-theory in the DLCQ limit.
pub fn matrix_model_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// BFSSModel : Type — the Banks-Fischler-Shenker-Susskind matrix model.
pub fn bfss_model_ty() -> Expr {
    type0()
}
/// NonPerturbativeCorrection : ℝ → ℝ
/// e^{-1/g_s}: non-perturbative contributions from D-instantons.
pub fn non_perturbative_correction_ty() -> Expr {
    arrow(real_ty(), real_ty())
}
/// DInstanton : ℕ → Type — a D(-1)-brane (D-instanton) contributing non-perturbatively.
pub fn d_instanton_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// StringDualityBoundState : DBrane → DBrane → ℕ → Type
/// A bound state of D-branes (p,q)-strings or (p,q) 5-branes.
pub fn string_duality_bound_state_ty() -> Expr {
    arrow(
        arrow(nat_ty(), type0()),
        arrow(arrow(nat_ty(), type0()), arrow(nat_ty(), type0())),
    )
}
/// MatrixStringTheory : Prop
/// BFSS matrix quantum mechanics at large N describes M-theory.
pub fn matrix_string_theory_ty() -> Expr {
    prop()
}
/// Register all *extended* string theory axioms into the given environment.
pub fn build_string_theory_ext_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("SuperstringType", type0()),
        ("StringField", type0()),
        ("StringFieldTheory", type0()),
        (
            "WorldsheetCFT_CentralCharge",
            worldsheet_cft_central_charge_ty(),
        ),
        ("OPEAlgebra", ope_algebra_ty()),
        ("stress_tensor_tt_ope", stress_tensor_tt_ope_ty()),
        ("primary_field_ope", primary_field_ope_ty()),
        ("DescendantField", descendant_field_ty()),
        ("ConformalBlock", conformal_block_ty()),
        ("crossing_symmetry", crossing_symmetry_ty()),
        ("VenezianoAmplitude", veneziano_amplitude_ty()),
        ("VirasoroShapiroAmplitude", virasoro_shapiro_amplitude_ty()),
        (
            "SuperstringVertexOperator",
            superstring_vertex_operator_ty(),
        ),
        ("SuperstringAmplitude", superstring_amplitude_ty()),
        ("ReggeTrajectory", regge_trajectory_ty()),
        ("SuperstringMassSpectrum", superstring_mass_spectrum_ty()),
        ("RRCharge", rr_charge_ty()),
        ("DBraneWorldvolume", d_brane_worldvolume_ty()),
        ("OpenStringEndpoint", open_string_endpoint_ty()),
        (
            "brane_antibrane_annihilation",
            brane_antibrane_annihilation_ty(),
        ),
        ("KTheoryCharge", k_theory_charge_ty()),
        ("TDualityGroup", t_duality_group_ty()),
        ("WindingMode", winding_mode_ty()),
        ("MomentumMode", momentum_mode_ty()),
        (
            "t_duality_invariant_spectrum",
            t_duality_invariant_spectrum_ty(),
        ),
        ("buscher_rules", buscher_rules_ty()),
        ("mirror_syz", mirror_syz_ty()),
        ("s_duality", s_duality_ty()),
        ("montonen_olive_duality", montonen_olive_duality_ty()),
        ("SL2Z_Action", sl2z_action_ty()),
        ("sl2z_duality", sl2z_duality_ty()),
        ("s_duality_bps_spectrum", s_duality_bps_spectrum_ty()),
        ("holographic_principle", holographic_principle_ty()),
        ("maldacena_conjecture", maldacena_conjecture_ty()),
        ("HolographicDictionary", holographic_dictionary_ty()),
        ("WilsonLoopAdS", wilson_loop_ads_ty()),
        ("HolographicEntanglement", holographic_entanglement_ty()),
        ("AdSRadiusDuality", ads_radius_duality_ty()),
        ("CYModuliSpace", cy_moduli_space_ty()),
        ("KahlerModuli", kahler_moduli_ty()),
        ("ComplexStructureModuli", complex_structure_moduli_ty()),
        ("special_geometry", special_geometry_ty()),
        ("GopakumarVafa", gopakumar_vafa_ty()),
        ("MTheoryDimension", m_theory_dimension_ty()),
        ("MTheorySupergravity", m_theory_supergravity_ty()),
        ("M2Brane", m2_brane_ty()),
        ("M5Brane", m5_brane_ty()),
        ("MTheoryCompactification", m_theory_compactification_ty()),
        ("m_theory_duality_web", m_theory_duality_web_ty()),
        ("horava_molten", horava_molten_ty()),
        ("BRSTCohomology", brst_cohomology_ty()),
        ("StringVertex", string_vertex_ty()),
        ("StarProduct", star_product_ty()),
        ("WittenSFTAction", witten_sft_action_ty()),
        ("tachyon_condensation", tachyon_condensation_ty()),
        ("AModelTwist", a_model_twist_ty()),
        ("BModelTwist", b_model_twist_ty()),
        ("GromovWittenPotential", gromov_witten_potential_ty()),
        ("BModelFreeEnergy", b_model_free_energy_ty()),
        (
            "holomorphic_anomaly_equation",
            holomorphic_anomaly_equation_ty(),
        ),
        ("TopologicalAmplitude", topological_amplitude_ty()),
        (
            "swampland_distance_conjecture",
            swampland_distance_conjecture_ty(),
        ),
        ("de_sitter_conjecture", de_sitter_conjecture_ty()),
        ("weak_gravity_conjecture", weak_gravity_conjecture_ty()),
        ("swampland_consistency", swampland_consistency_ty()),
        ("landscape_vs_swampland", landscape_vs_swampland_ty()),
        ("MatrixModel", matrix_model_ty()),
        ("BFSSModel", bfss_model_ty()),
        (
            "NonPerturbativeCorrection",
            non_perturbative_correction_ty(),
        ),
        ("DInstanton", d_instanton_ty()),
        ("StringDualityBoundState", string_duality_bound_state_ty()),
        ("matrix_string_theory", matrix_string_theory_ty()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
}
/// Log-Gamma approximation for positive arguments via Stirling's series.
pub(super) fn lgamma_approx(x: f64) -> f64 {
    if x <= 0.0 {
        return f64::NAN;
    }
    (x - 0.5) * x.ln() - x + 0.5 * (2.0 * std::f64::consts::PI).ln()
}
#[cfg(test)]
mod ext_tests {
    use super::*;
    #[test]
    fn test_build_ext_env() {
        let mut env = build_string_theory_env();
        build_string_theory_ext_env(&mut env);
        assert!(env.get(&Name::str("VenezianoAmplitude")).is_some());
        assert!(env.get(&Name::str("maldacena_conjecture")).is_some());
        assert!(env.get(&Name::str("M2Brane")).is_some());
        assert!(env
            .get(&Name::str("swampland_distance_conjecture"))
            .is_some());
    }
    #[test]
    fn test_virasoro_ext_partition_numbers() {
        let vir = VirasoroAlgebraExt::new(26.0, 0.0, 5);
        assert_eq!(vir.partition_number(0), 1);
        assert_eq!(vir.partition_number(1), 1);
        assert_eq!(vir.partition_number(2), 2);
        assert_eq!(vir.partition_number(3), 3);
        assert_eq!(vir.partition_number(4), 5);
        assert_eq!(vir.partition_number(5), 7);
    }
    #[test]
    fn test_virasoro_ext_central_term() {
        let vir = VirasoroAlgebraExt::new(26.0, 1.0, 3);
        let ct = vir.central_term(2);
        assert!(
            (ct - 13.0).abs() < 1e-10,
            "Central term for m=2 should be 13, got {ct}"
        );
    }
    #[test]
    fn test_virasoro_ext_character() {
        let vir = VirasoroAlgebraExt::new(26.0, 0.0, 3);
        let chi = vir.character_truncated(0.5);
        assert!(
            chi > 0.0 && chi.is_finite(),
            "Character should be positive, got {chi}"
        );
    }
    #[test]
    fn test_veneziano_regge_trajectory() {
        let amp = VenezianoAmplitudeCalc::new(1.0);
        assert!((amp.regge_trajectory(0.0) - 1.0).abs() < 1e-12);
        assert!((amp.regge_trajectory(2.0) - 2.0).abs() < 1e-12);
    }
    #[test]
    fn test_veneziano_low_energy() {
        let amp = VenezianoAmplitudeCalc::new(1.0);
        let val = amp.low_energy_amplitude(-1.0, -2.0);
        assert!(
            (val - 1.5).abs() < 1e-10,
            "Low energy amplitude should be 1.5, got {val}"
        );
    }
    #[test]
    fn test_calabiyau_hodge_quintic() {
        let cy = CalabiyauHodge::quintic();
        assert_eq!(cy.h11, 1);
        assert_eq!(cy.h21, 101);
        assert_eq!(cy.euler_characteristic(), -200);
        assert_eq!(cy.total_moduli(), 102);
        assert_eq!(cy.num_generations(), 100);
    }
    #[test]
    fn test_calabiyau_hodge_mirror() {
        let quintic = CalabiyauHodge::quintic();
        let mirror = CalabiyauHodge::mirror_quintic();
        assert!(quintic.is_mirror_of(&mirror));
        assert!(mirror.is_mirror_of(&quintic));
        assert_eq!(
            quintic.euler_characteristic(),
            -mirror.euler_characteristic()
        );
    }
    #[test]
    fn test_ads_cft_dictionary_conformal_dim() {
        let dict = AdSCFTDictionary::ads5_cft4(100);
        let delta = dict.conformal_dimension(0.0);
        // For massless scalar in AdS5/CFT4: Δ = d/2 + d/2 = d = 4
        assert!(
            (delta - 4.0).abs() < 1e-10,
            "Massless scalar: Δ should be d=4, got {delta}"
        );
    }
    #[test]
    fn test_ads_cft_bf_bound() {
        let dict = AdSCFTDictionary::new(4, 1.0, 1.0, 10, 0.1);
        let bf = dict.bf_bound();
        assert!(
            (bf - (-4.0)).abs() < 1e-10,
            "BF bound should be -4 for d=4,L=1, got {bf}"
        );
    }
    #[test]
    fn test_ads_cft_t_hooft_coupling() {
        let dict = AdSCFTDictionary::new(4, 1.0, 0.01, 100, 0.1);
        let lambda = dict.t_hooft_coupling();
        let expected = 4.0 * std::f64::consts::PI * 0.1 * 100.0;
        assert!(
            (lambda - expected).abs() < 1e-10,
            "t'Hooft coupling mismatch"
        );
    }
    #[test]
    fn test_m_theory_branes() {
        let m = MTheory::new(11);
        assert!(m.eleven_dim_supergravity());
        let (m2, m5) = m.m2_m5_branes();
        assert!(m2 > 0.0);
        assert!(m5 > 0.0);
        assert!(m2 > m5, "M2 tension should exceed M5 in natural units");
    }
    #[test]
    fn test_topological_string_mirror() {
        let ts = TopologicalString::new(true, true);
        assert!(ts.mirror_symmetry());
        assert!(ts.gromov_witten());
        assert!(ts.kodaira_spencer());
    }
}
