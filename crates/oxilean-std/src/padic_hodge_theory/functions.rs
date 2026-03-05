//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Declaration, Environment, Expr, Name};

use super::types::{
    AbsolutelyIrreducible, BreuilKisin, CrystallineLiftingRing, CrystallineRepresentation,
    DeRhamRepresentation, DieudonneModule, EtaleLocalSystem, FilteredPhiModule, FiltrationOnBdR,
    FontaineDieudonne, FrobeniusOnBcrys, GaloisRepresentationOfEllipticCurve,
    HodgeTateDecomposition, HodgeTateDecompositionComputer, HodgeTateWeights, IwasawaTheory,
    LAdicSheaf, MonodromyOperator, PAdicLanglands, PAdicPeriodRings, PAdicRepresentation,
    PadicLFunctionInterpolation, PadicNumber, PerfectoidAlgebra, PerfectoidChar, PeriodRing,
    PhiModuleComputation, PrismaticCohomology, SemiStableRepresentation, SyntonicComplex,
    WachModuleCheck, WeaklyAdmissible,
};

pub fn cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}
pub fn prop() -> Expr {
    Expr::Sort(oxilean_kernel::Level::zero())
}
pub fn type0() -> Expr {
    Expr::Sort(oxilean_kernel::Level::succ(oxilean_kernel::Level::zero()))
}
pub fn nat_ty() -> Expr {
    cst("Nat")
}
pub fn int_ty() -> Expr {
    cst("Int")
}
pub fn real_ty() -> Expr {
    cst("Real")
}
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn arrow(a: Expr, b: Expr) -> Expr {
    Expr::Pi(
        oxilean_kernel::BinderInfo::Default,
        Name::str("_"),
        Box::new(a),
        Box::new(b),
    )
}
/// `PAdicPeriodRingsTy : Nat → Type` — the period rings B_crys, B_st, B_dR, B_HT.
pub fn padic_period_rings_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `FiltrationOnBdRTy : Nat → Type` — Fil^i B_dR = t^i B_dR^+.
pub fn filtration_on_bdr_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `FrobeniusOnBcrysTy : Nat → Type` — the Frobenius φ on B_crys.
pub fn frobenius_on_bcrys_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `MonodromyOperatorTy : Nat → Type` — monodromy N with N φ = p φ N.
pub fn monodromy_operator_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `PAdicRepresentationTy : Nat → Nat → Type` — G_K → GL_n(ℚ_p).
pub fn padic_representation_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `HodgeTateDecompositionTy : Nat → Nat → Prop` — V ⊗ C_p ≅ ⊕ C_p(i)^{h_i}.
pub fn hodge_tate_decomposition_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `HodgeTateWeightsTy : Nat → Type` — the multiset of HT weights.
pub fn hodge_tate_weights_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `IsHodgeTateTy : Prop` — V is Hodge–Tate.
pub fn is_hodge_tate_ty() -> Expr {
    prop()
}
/// `IsDeRhamTy : Prop` — V is de Rham.
pub fn is_de_rham_ty() -> Expr {
    prop()
}
/// `IsCrystallineTy : Prop` — V is crystalline.
pub fn is_crystalline_ty() -> Expr {
    prop()
}
/// `IsSemiStableTy : Prop` — V is semi-stable.
pub fn is_semi_stable_ty() -> Expr {
    prop()
}
/// `CrystallineRepresentationTy : Nat → Nat → Type` — D_crys(V).
pub fn crystalline_representation_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `SemiStableRepresentationTy : Nat → Nat → Type` — D_st(V).
pub fn semi_stable_representation_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `DeRhamRepresentationTy : Nat → Nat → Type` — D_dR(V).
pub fn de_rham_representation_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `WeaklyAdmissibleTy : Prop` — Newton polygon ≥ Hodge polygon.
pub fn weakly_admissible_ty() -> Expr {
    prop()
}
/// `FontaineTheoremTy : Prop` — weakly admissible ↔ admissible.
pub fn fontaine_theorem_ty() -> Expr {
    prop()
}
/// `FilteredPhiModuleTy : Nat → Nat → Type` — finite K₀-vs with φ and filtration.
pub fn filtered_phi_module_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `FontaineDieudonneTy : Nat → Nat → Type` — Dieudonné module of a formal group.
pub fn fontaine_dieudonne_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `BreuilKisinTy : Nat → Nat → Type` — Kisin module over S = W(k)[[u]].
pub fn breuil_kisin_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `KisinEquivalenceTy : Prop` — free ℤ_p-lattices ↔ Kisin modules.
pub fn kisin_equivalence_ty() -> Expr {
    prop()
}
/// `TateModuleTy : Nat → Type` — T_p(E) = lim← E[p^n].
pub fn tate_module_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `GaloisRepresentationOfEllipticCurveTy : Nat → Type` — ρ_{E,p}: G_K → GL_2(ℚ_p).
pub fn galois_representation_of_elliptic_curve_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `AbsolutelyIrreducibleTy : Prop` — irreducibility condition.
pub fn absolutely_irreducible_ty() -> Expr {
    prop()
}
/// `CrystallineLiftingRingTy : Nat → Type` — Kisin's framed deformation ring.
pub fn crystalline_lifting_ring_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `PAdicComparisonTheoremTy : Prop` — étale cohomology ≅ de Rham via B_dR.
pub fn padic_comparison_theorem_ty() -> Expr {
    prop()
}
/// `SenTheoremTy : Prop` — Sen's theorem on Hodge–Tate decomposition.
pub fn sen_theorem_ty() -> Expr {
    prop()
}
/// `BergerTheoremTy : Prop` — B-admissible ↔ filtered (φ,N)-module conditions.
pub fn berger_theorem_ty() -> Expr {
    prop()
}
/// `ColmezFontaineTy : Prop` — weakly admissible = admissible (Colmez–Fontaine 2000).
pub fn colmez_fontaine_ty() -> Expr {
    prop()
}
/// `TateTwistTy : Int → Nat → Type` — Tate twist V(n) = V ⊗ ℚ_p(n).
pub fn tate_twist_ty() -> Expr {
    arrow(int_ty(), arrow(nat_ty(), type0()))
}
/// `CyclotomicCharacterTy : Nat → Type` — the p-adic cyclotomic character χ_p.
pub fn cyclotomic_character_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `NewtonPolygonTy : Nat → Type` — Newton polygon of a filtered φ-module.
pub fn newton_polygon_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `HodgePolygonTy : Nat → Type` — Hodge polygon of a filtered module.
pub fn hodge_polygon_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `BdRRingTy : Nat → Type` — the de Rham period ring B_dR as a complete DVR.
pub fn bdr_ring_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `BcrysRingTy : Nat → Type` — the crystalline period ring B_crys with φ.
pub fn bcrys_ring_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `BstRingTy : Nat → Type` — the semi-stable period ring B_st with (φ, N).
pub fn bst_ring_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `BHTRingTy : Nat → Type` — the Hodge–Tate period ring B_HT = gr*(B_dR).
pub fn bht_ring_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `BdRFiltrationTy : Nat → Nat → Type` — Fil^i B_dR = t^i B_dR^+.
pub fn bdr_filtration_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `CrystallineComparisonTy : Prop` — H*_et ⊗ B_crys ≅ H*_crys ⊗ B_crys (Fontaine–Faltings).
pub fn crystalline_comparison_ty() -> Expr {
    prop()
}
/// `DeRhamComparisonTy : Prop` — H*_et ⊗ B_dR ≅ H*_dR ⊗ B_dR (p-adic comparison).
pub fn de_rham_comparison_ty() -> Expr {
    prop()
}
/// `EtaleComparisonTy : Prop` — étale cohomology comparison via period rings.
pub fn etale_comparison_ty() -> Expr {
    prop()
}
/// `CrystallineDeRhamComparisonTy : Prop` — crys comparison implies de Rham comparison.
pub fn crystalline_de_rham_comparison_ty() -> Expr {
    prop()
}
/// `PDivisibleGroupTy : Nat → Nat → Type` — p-divisible group G of height h, dimension d.
pub fn p_divisible_group_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `DieudonneModuleTy : Nat → Nat → Type` — covariant Dieudonné module D(G).
pub fn dieudonne_module_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `FormalGroupTy : Nat → Type` — formal group of dimension d over W(k).
pub fn formal_group_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `PDivisibleGroupEquivalenceTy : Prop` — equivalence of p-div groups and Dieudonné modules.
pub fn p_divisible_group_equivalence_ty() -> Expr {
    prop()
}
/// `PerfectoidSpaceTy : Nat → Type` — a perfectoid space over a perfectoid field.
pub fn perfectoid_space_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `TiltingFunctorTy : Nat → Type` — the tilting equivalence X ↦ X♭.
pub fn tilting_functor_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `TiltingEquivalenceTy : Prop` — Scholze's tilting equivalence for perfectoid spaces.
pub fn tilting_equivalence_ty() -> Expr {
    prop()
}
/// `AlmostMathematicsTy : Nat → Type` — Faltings' almost mathematics / almost ring theory.
pub fn almost_mathematics_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `PerfectoidFieldTy : Nat → Type` — a perfectoid field K with |K*| dense in ℝ_{>0}.
pub fn perfectoid_field_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `PrismTy : Nat → Type` — a prism (A, I) with A a δ-ring, I a Cartier divisor.
pub fn prism_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `PrismaticSiteTy : Nat → Type` — the prismatic site (X/A)_Δ.
pub fn prismatic_site_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `PrismaticCohomologyTy : Nat → Nat → Type` — Δ*(X/A), the prismatic cohomology.
pub fn prismatic_cohomology_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `NygaardFiltrationTy : Nat → Nat → Type` — the Nygaard filtration N^≥i Δ*(X).
pub fn nygaard_filtration_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `BKPrismTy : Nat → Type` — the Breuil–Kisin prism (S, (E(u))) where S = W[[u]].
pub fn bk_prism_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `AinfPrismTy : Nat → Type` — the Ainf-prism (A_inf, (ξ)) of Fontaine.
pub fn ainf_prism_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `SyntomicComplexTy : Nat → Nat → Type` — the syntomic complex Syn(X, n).
pub fn syntomic_complex_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `PAdicRegulatoryTy : Prop` — the p-adic regulator on K-theory.
pub fn padic_regulator_ty() -> Expr {
    prop()
}
/// `SyntomicCohomologyTy : Nat → Nat → Type` — H^i_syn(X, Z_p(n)).
pub fn syntomic_cohomology_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `WachModuleTy : Nat → Nat → Type` — a Wach module over Λ_A(Γ) for positive crys reps.
pub fn wach_module_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `WachModuleEquivalenceTy : Prop` — Wach module equivalence for positive crys G_K-reps.
pub fn wach_module_equivalence_ty() -> Expr {
    prop()
}
/// `PositiveCrystallineRepTy : Nat → Type` — G_K-rep with non-negative Hodge–Tate weights.
pub fn positive_crystalline_rep_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `PAdicLFunctionTy : Nat → Type` — a p-adic L-function L_p(s) interpolating L-values.
pub fn padic_l_function_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `IwasawaMainConjectureTy : Prop` — the Iwasawa main conjecture char(X) = (L_p).
pub fn iwasawa_main_conjecture_ty() -> Expr {
    prop()
}
/// `IwasawaAlgebraTy : Nat → Type` — the Iwasawa algebra Λ = Z_p[[Γ]] ≅ Z_p[[T]].
pub fn iwasawa_algebra_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `PAdicLFunctionInterpolationTy : Prop` — interpolation property at classical characters.
pub fn padic_l_function_interpolation_ty() -> Expr {
    prop()
}
/// `IwasawaInvariantsTy : Nat → Type` — μ and λ invariants of an Iwasawa module.
pub fn iwasawa_invariants_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `SenOperatorTy : Nat → Type` — the Sen operator Θ on C_p-representations.
pub fn sen_operator_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `SenTheoryTy : Prop` — Sen's decompletion and Hodge–Tate decomposition theorem.
pub fn sen_theory_ty() -> Expr {
    prop()
}
/// `HodgeTateWeightMultiplicityTy : Nat → Nat → Prop` — multiplicity h_i of weight i.
pub fn hodge_tate_weight_multiplicity_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `AlmostPurityTheoremTy : Prop` — Faltings' almost purity / Scholze's perfectoid version.
pub fn almost_purity_theorem_ty() -> Expr {
    prop()
}
/// `FaltingsAlmostEtaleTy : Prop` — Faltings' almost étale extensions theorem.
pub fn faltings_almost_etale_ty() -> Expr {
    prop()
}
/// `PerfectoidAlmostPurityTy : Prop` — perfectoid almost purity (Scholze 2012).
pub fn perfectoid_almost_purity_ty() -> Expr {
    prop()
}
/// `FilteredPhiNModuleTy : Nat → Nat → Type` — filtered (φ, N)-module; target of D_st.
pub fn filtered_phi_n_module_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `AdmissibleFilteredModuleTy : Prop` — D is admissible (arises from a semi-stable rep).
pub fn admissible_filtered_module_ty() -> Expr {
    prop()
}
/// `WeaklyAdmissibleEqualsAdmissibleTy : Prop` — Colmez–Fontaine: w.a. ↔ admissible.
pub fn weakly_admissible_equals_admissible_ty() -> Expr {
    prop()
}
/// `BreuilKisinModuleTy : Nat → Nat → Type` — Kisin module M over S = W(k)[[u]].
pub fn breuil_kisin_module_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `BreuilKisinGModuleTy : Nat → Nat → Type` — Kisin module with G_K∞-action.
pub fn breuil_kisin_g_module_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `IntegralCrystallineRepTy : Nat → Type` — free Z_p-lattice in a crys representation.
pub fn integral_crystalline_rep_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `WittVectorsTy : Nat → Type` — the ring of Witt vectors W(R).
pub fn witt_vectors_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `AinfRingTy : Nat → Type` — Fontaine's A_inf = W(O_C_p^flat) with θ: A_inf → O_C_p.
pub fn ainf_ring_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `ThetaMapTy : Prop` — Fontaine's map θ: A_inf → O_{C_p}.
pub fn theta_map_ty() -> Expr {
    prop()
}
/// `CrystallineComparisonIsomTy : Nat → Prop` — crys comparison isom at degree i.
pub fn crystalline_comparison_isom_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `DeRhamComparisonIsomTy : Nat → Prop` — de Rham comparison isom at degree i.
pub fn de_rham_comparison_isom_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `PerfectoidTiltTy : Nat → Type` — the tilt X♭ of a perfectoid space X.
pub fn perfectoid_tilt_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `DeltaRingTy : Nat → Type` — a δ-ring (ring with a Witt-vector lift of Frobenius).
pub fn delta_ring_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `PrismaticFrobeniusTy : Nat → Type` — the absolute Frobenius on a δ-ring.
pub fn prismatic_frobenius_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `KisinFarguesTy : Prop` — Kisin–Fargues classification of p-divisible groups.
pub fn kisin_fargues_ty() -> Expr {
    prop()
}
/// `OverconvergentPhiModuleTy : Nat → Type` — overconvergent (φ, Γ)-module.
pub fn overconvergent_phi_module_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `EtalPhiModuleTy : Nat → Type` — étale (φ, Γ)-module (p-adic rep ↔ étale φ-mod).
pub fn etal_phi_module_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `FontaineEquivalenceTy : Prop` — Fontaine: ét. φ-mods ↔ p-adic representations of G_K.
pub fn fontaine_equivalence_ty() -> Expr {
    prop()
}
/// `ColmezFunctorTy : Nat → Type` — the Colmez functor V ↦ D_rig(V).
pub fn colmez_functor_ty() -> Expr {
    arrow(nat_ty(), type0())
}
type AxiomEntry = (&'static str, fn() -> Expr);
pub fn build_env(env: &mut Environment) {
    let axioms: &[AxiomEntry] = &[
        ("PAdicPeriodRings", padic_period_rings_ty),
        ("FiltrationOnBdR", filtration_on_bdr_ty),
        ("FrobeniusOnBcrys", frobenius_on_bcrys_ty),
        ("MonodromyOperator", monodromy_operator_ty),
        ("PAdicRepresentation", padic_representation_ty),
        ("HodgeTateDecomposition", hodge_tate_decomposition_ty),
        ("HodgeTateWeights", hodge_tate_weights_ty),
        ("IsHodgeTate", is_hodge_tate_ty),
        ("IsDeRham", is_de_rham_ty),
        ("IsCrystalline", is_crystalline_ty),
        ("IsSemiStable", is_semi_stable_ty),
        ("CrystallineRepresentation", crystalline_representation_ty),
        ("SemiStableRepresentation", semi_stable_representation_ty),
        ("DeRhamRepresentation", de_rham_representation_ty),
        ("WeaklyAdmissible", weakly_admissible_ty),
        ("FontaineTheorem", fontaine_theorem_ty),
        ("FilteredPhiModule", filtered_phi_module_ty),
        ("FontaineDieudonne", fontaine_dieudonne_ty),
        ("BreuilKisin", breuil_kisin_ty),
        ("KisinEquivalence", kisin_equivalence_ty),
        ("TateModule", tate_module_ty),
        (
            "GaloisRepresentationOfEllipticCurve",
            galois_representation_of_elliptic_curve_ty,
        ),
        ("AbsolutelyIrreducible", absolutely_irreducible_ty),
        ("CrystallineLiftingRing", crystalline_lifting_ring_ty),
        ("PAdicComparisonTheorem", padic_comparison_theorem_ty),
        ("SenTheorem", sen_theorem_ty),
        ("BergerTheorem", berger_theorem_ty),
        ("ColmezFontaine", colmez_fontaine_ty),
        ("TateTwist", tate_twist_ty),
        ("CyclotomicCharacter", cyclotomic_character_ty),
        ("NewtonPolygon", newton_polygon_ty),
        ("HodgePolygon", hodge_polygon_ty),
        ("BdRRing", bdr_ring_ty),
        ("BcrysRing", bcrys_ring_ty),
        ("BstRing", bst_ring_ty),
        ("BHTRing", bht_ring_ty),
        ("BdRFiltration", bdr_filtration_ty),
        ("CrystallineComparison", crystalline_comparison_ty),
        ("DeRhamComparison", de_rham_comparison_ty),
        ("EtaleComparison", etale_comparison_ty),
        (
            "CrystallineDeRhamComparison",
            crystalline_de_rham_comparison_ty,
        ),
        ("PDivisibleGroup", p_divisible_group_ty),
        ("DieudonneModule", dieudonne_module_ty),
        ("FormalGroup", formal_group_ty),
        (
            "PDivisibleGroupEquivalence",
            p_divisible_group_equivalence_ty,
        ),
        ("PerfectoidSpace", perfectoid_space_ty),
        ("TiltingFunctor", tilting_functor_ty),
        ("TiltingEquivalence", tilting_equivalence_ty),
        ("AlmostMathematics", almost_mathematics_ty),
        ("PerfectoidField", perfectoid_field_ty),
        ("Prism", prism_ty),
        ("PrismaticSite", prismatic_site_ty),
        ("PrismaticCohomology", prismatic_cohomology_ty),
        ("NygaardFiltration", nygaard_filtration_ty),
        ("BKPrism", bk_prism_ty),
        ("AinfPrism", ainf_prism_ty),
        ("SyntomicComplex", syntomic_complex_ty),
        ("PAdicRegulator", padic_regulator_ty),
        ("SyntomicCohomology", syntomic_cohomology_ty),
        ("WachModule", wach_module_ty),
        ("WachModuleEquivalence", wach_module_equivalence_ty),
        ("PositiveCrystallineRep", positive_crystalline_rep_ty),
        ("PAdicLFunction", padic_l_function_ty),
        ("IwasawaMainConjecture", iwasawa_main_conjecture_ty),
        ("IwasawaAlgebra", iwasawa_algebra_ty),
        (
            "PAdicLFunctionInterpolation",
            padic_l_function_interpolation_ty,
        ),
        ("IwasawaInvariants", iwasawa_invariants_ty),
        ("SenOperator", sen_operator_ty),
        ("SenTheory", sen_theory_ty),
        (
            "HodgeTateWeightMultiplicity",
            hodge_tate_weight_multiplicity_ty,
        ),
        ("AlmostPurityTheorem", almost_purity_theorem_ty),
        ("FaltingsAlmostEtale", faltings_almost_etale_ty),
        ("PerfectoidAlmostPurity", perfectoid_almost_purity_ty),
        ("FilteredPhiNModule", filtered_phi_n_module_ty),
        ("AdmissibleFilteredModule", admissible_filtered_module_ty),
        (
            "WeaklyAdmissibleEqualsAdmissible",
            weakly_admissible_equals_admissible_ty,
        ),
        ("BreuilKisinModule", breuil_kisin_module_ty),
        ("BreuilKisinGModule", breuil_kisin_g_module_ty),
        ("IntegralCrystallineRep", integral_crystalline_rep_ty),
        ("WittVectors", witt_vectors_ty),
        ("AinfRing", ainf_ring_ty),
        ("ThetaMap", theta_map_ty),
        ("CrystallineComparisonIsom", crystalline_comparison_isom_ty),
        ("DeRhamComparisonIsom", de_rham_comparison_isom_ty),
        ("PerfectoidTilt", perfectoid_tilt_ty),
        ("DeltaRing", delta_ring_ty),
        ("PrismaticFrobenius", prismatic_frobenius_ty),
        ("KisinFargues", kisin_fargues_ty),
        ("OverconvergentPhiModule", overconvergent_phi_module_ty),
        ("EtalPhiModule", etal_phi_module_ty),
        ("FontaineEquivalence", fontaine_equivalence_ty),
        ("ColmezFunctor", colmez_functor_ty),
    ];
    for (name, ty_fn) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty_fn(),
        })
        .ok();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_padic_number_from_integer() {
        let x = PadicNumber::from_integer(5, 13);
        assert_eq!(x.digits, vec![3, 2]);
        assert_eq!(x.padic_valuation(), Some(0));
    }
    #[test]
    fn test_padic_number_norm() {
        let x = PadicNumber::from_integer(5, 25);
        assert_eq!(x.padic_valuation(), Some(2));
        let norm = x.norm();
        assert!((norm - 0.04).abs() < 1e-10);
    }
    #[test]
    fn test_padic_number_add() {
        let a = PadicNumber::from_integer(5, 3);
        let b = PadicNumber::from_integer(5, 4);
        let sum = a.add(&b);
        assert_eq!(sum.digits[0], 2);
        assert_eq!(sum.digits[1], 1);
    }
    #[test]
    fn test_hodge_tate_decomposition_computer() {
        let comp = HodgeTateDecompositionComputer::new(vec![0, 0, 1, 1, 2]);
        let decomp = comp.compute();
        assert_eq!(decomp, vec![(0, 2), (1, 2), (2, 1)]);
        assert_eq!(comp.dimension(), 5);
    }
    #[test]
    fn test_hodge_tate_format() {
        let comp = HodgeTateDecompositionComputer::new(vec![0, 1]);
        let s = comp.format_decomposition();
        assert!(s.contains("C_p(0)^1"));
        assert!(s.contains("C_p(1)^1"));
    }
    #[test]
    fn test_phi_module_trace_det() {
        let m = PhiModuleComputation::new(5, vec![vec![2, 1], vec![0, 3]]);
        assert_eq!(m.trace(), 5);
        assert_eq!(m.determinant(), Some(6));
    }
    #[test]
    fn test_phi_module_newton_slope() {
        let m = PhiModuleComputation::new(5, vec![vec![5, 0], vec![0, 1]]);
        let slope = m.newton_slope().expect("newton_slope should succeed");
        assert!((slope - 0.5).abs() < 1e-10);
    }
    #[test]
    fn test_wach_module_check_valid() {
        let mut w = WachModuleCheck::new(2, 5, 1);
        assert!(!w.is_valid_wach_module());
        w.set_all_ok();
        assert!(w.is_valid_wach_module());
    }
    #[test]
    fn test_wach_module_berger_string() {
        let w = WachModuleCheck::new(2, 5, 1);
        let s = w.berger_equivalence();
        assert!(s.contains("Wach modules"));
        assert!(s.contains("positive crys"));
    }
    #[test]
    fn test_padic_l_function_interpolation() {
        let mut lp = PadicLFunctionInterpolation::new(5, 1);
        lp.add_value(1, -0.5);
        lp.add_value(2, 0.25);
        assert_eq!(lp.query(1), Some(-0.5));
        assert_eq!(lp.query(3), None);
    }
    #[test]
    fn test_padic_l_function_mu_invariant() {
        let mut lp = PadicLFunctionInterpolation::new(5, 1);
        lp.add_value(1, 1.0);
        lp.add_value(2, 0.5);
        assert_eq!(lp.mu_invariant(), 0);
    }
    #[test]
    fn test_build_env_has_new_axioms() {
        let mut env = Environment::new();
        build_env(&mut env);
        let names = [
            "BdRRing",
            "PrismaticCohomology",
            "WachModule",
            "PAdicLFunction",
            "TiltingEquivalence",
            "AlmostPurityTheorem",
            "FontaineEquivalence",
            "ColmezFunctor",
            "WittVectors",
            "AinfRing",
        ];
        for name in &names {
            assert!(
                env.get(&Name::str(*name)).is_some(),
                "Missing axiom: {name}"
            );
        }
    }
    #[test]
    fn test_period_ring_contains() {
        assert!(PeriodRing::BdR.contains_ring(&PeriodRing::Bcrys));
        assert!(PeriodRing::Bst.contains_ring(&PeriodRing::Bcrys));
        assert!(!PeriodRing::Bcrys.contains_ring(&PeriodRing::BdR));
        assert!(!PeriodRing::Bht.contains_ring(&PeriodRing::BdR));
    }
    #[test]
    fn test_padic_number_display() {
        let x = PadicNumber::from_integer(5, 0);
        let d = x.display();
        assert!(d.is_empty() || d == "0");
    }
}
#[cfg(test)]
mod tests_padic_hodge_ext {
    use super::*;
    #[test]
    fn test_ladic_sheaf() {
        let sheaf = LAdicSheaf::constant_sheaf("X", 5);
        assert_eq!(sheaf.rank, 1);
        assert!(sheaf.is_lisse);
        let weil = sheaf.weil_conjecture_reference();
        assert!(weil.contains("Deligne"));
        let trace = sheaf.grothendieck_trace_formula();
        assert!(trace.contains("Frob"));
    }
    #[test]
    fn test_etale_local_system() {
        let els = EtaleLocalSystem::new("ρ_E", vec![0, 1]).crystalline();
        assert!(els.is_crystalline);
        let font = els.fontaine_correspondence();
        assert!(font.contains("Fontaine"));
    }
    #[test]
    fn test_prismatic_cohomology() {
        let ainf = PrismaticCohomology::ainf_prism();
        assert!(ainf.perfect_prism);
        let bms = ainf.bms_comparison_theorem();
        assert!(bms.contains("BMS"));
        assert!(ainf.is_universal_cohomology_theory());
        let ht = ainf.hodge_tate_comparison();
        assert!(ht.contains("HT"));
    }
    #[test]
    fn test_perfectoid_algebra() {
        let pa = PerfectoidAlgebra::new("C_p", PerfectoidChar::CharZero);
        assert!(!pa.is_tilted_char_p());
        let tilting = pa.scholze_tilting_equivalence();
        assert!(tilting.contains("Scholze"));
        let witt = pa.witt_vector_description();
        assert!(witt.contains("W("));
    }
    #[test]
    fn test_padic_langlands() {
        let pl = PAdicLanglands::gl2_qp(7);
        assert_eq!(pl.prime_p, 7);
        let colmez = pl.colmez_description();
        assert!(colmez.contains("Colmez"));
        let bm = pl.breuil_mézard_conjecture();
        assert!(bm.contains("Breuil"));
    }
    #[test]
    fn test_iwasawa_theory() {
        let iw = IwasawaTheory::cyclotomic("Q");
        let mc = iw.main_conjecture_iwasawa();
        assert!(mc.contains("Iwasawa"));
        let st = iw.structure_theorem();
        assert!(st.contains("Λ"));
    }
}
#[cfg(test)]
mod tests_padic_hodge_ext2 {
    use super::*;
    #[test]
    fn test_syntomic_complex() {
        let sc = SyntonicComplex {
            scheme: "X".to_string(),
            torsion_bound: 4,
            is_quasi_syntomic: true,
        };
        assert!(sc.is_quasi_syntomic);
        let cmp = sc.aq_crys_comparison();
        assert!(cmp.contains("syntomic"));
    }
}
