//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    AcyclicMorphism, AdamsNovikovSS, AdamsSpectralSequence, BPAdamsSpectralSequence,
    BPCohomologyData, BousfieldLocalization, BrownPetersonBP, ChromaticComplexityData,
    ChromaticConvergence, ChromaticFiltration, ChromaticSS, ChromaticTowerLevel, DescentData,
    EllipticCohomologyTheory, EllipticCurveOverRing, FglArithmetic, FormalGroupDeformation,
    FormalGroupLaw, HondaFormalGroup, LambdaRingElement, LandweberExactFunctor, LazardRing,
    LevelStructure, LocalSpectra, LocalizationUnit, LubinTateSpaceData, ModularFormSpectrum,
    MoravaETheory, MoravaKData, MoravaKGroup, MoravaKTheory, MoravaStabilizerGroupData,
    NilpotenceData, OrientationData, PeriodicityThm, SpectralScheme, ThickSubcategoryData,
    TopologicalCyclicHomologyData, TopologicalHochschildHomologyData, TopologicalModularForms,
    VnSelfMapData, WittVectorRing,
};

pub fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
pub fn app2(f: Expr, a: Expr, b: Expr) -> Expr {
    app(app(f, a), b)
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
pub fn impl_pi(name: &str, dom: Expr, body: Expr) -> Expr {
    pi(BinderInfo::Implicit, name, dom, body)
}
pub fn bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
pub fn nat_ty() -> Expr {
    cst("Nat")
}
pub fn add_axiom(
    env: &mut Environment,
    name: &str,
    univ_params: Vec<Name>,
    ty: Expr,
) -> Result<(), String> {
    env.add(Declaration::Axiom {
        name: Name::str(name),
        univ_params,
        ty,
    })
    .map_err(|e| format!("add_axiom({name}): {e:?}"))
}
/// `FormalGroupLaw : Type → Type`
///
/// A formal group law over a ring R: a power series F(x,y) ∈ R[[x,y]]
/// satisfying associativity, commutativity, and identity axioms.
pub fn formal_group_law_ty() -> Expr {
    arrow(type0(), type0())
}
/// `LazardRing : Type`
///
/// The Lazard ring L: the universal ring for formal group laws.
/// There is a bijection between ring homomorphisms L → R and formal group laws over R.
pub fn lazard_ring_ty() -> Expr {
    type0()
}
/// `MoravaETheory : Nat → Type → Type`
///
/// Morava E-theory E(k, Γ) at height k with formal group Γ.
/// This is the Lubin-Tate spectrum associated to a formal group of height k.
pub fn morava_e_theory_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), type0()))
}
/// `MoravaKTheory : Nat → Nat → Type`
///
/// Morava K-theory K(n) at height n and prime p.
/// The residue field of Morava E-theory; has a unique formal group of height n.
pub fn morava_k_theory_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `ChromaticFiltration : Nat → Type → Type`
///
/// The chromatic filtration: L_n = E(n)-localization giving the n-th chromatic
/// approximation, and M_n = monochromatic layer (fiber of L_n → L_{n-1}).
pub fn chromatic_filtration_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), type0()))
}
/// `ChromaticConvergence : Type → Prop`
///
/// The chromatic convergence theorem: π_*(X_p^) ≅ lim_n π_*(L_n X_p^)
/// for p-complete spectra X of finite type.
pub fn chromatic_convergence_ty() -> Expr {
    arrow(type0(), prop())
}
/// `PeriodicityThm : Nat → Nat → Type → Prop`
///
/// v_n-periodicity: a finite CW-complex of type n carries a v_n self-map
/// (Devinatz-Hopkins-Smith nilpotence and periodicity theorems).
pub fn periodicity_thm_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(type0(), prop())))
}
/// `Height : Type → Nat`
///
/// The chromatic height of a spectrum (or formal group law).
pub fn height_ty() -> Expr {
    arrow(type0(), nat_ty())
}
/// `VnPeriodicityClass : Nat → Type → Type`
///
/// The v_n-periodicity class in π_*(X) for a type-n spectrum X.
pub fn vn_periodicity_class_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), type0()))
}
/// `NilpotentElement : Type → Type → Prop`
///
/// Nilpotence: an element α ∈ π_*(X) is nilpotent in the sense of
/// Devinatz-Hopkins-Smith (α^N = 0 in MU_*(X) implies it is nilpotent).
pub fn nilpotent_element_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// `BousfieldLocalization : Type → Type → Type`
///
/// Bousfield localization L_E X: the E-localization of a spectrum X.
pub fn bousfield_localization_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// `LocalizationUnit : ∀ (E X : Type), BousfieldLocalization E X → Prop`
///
/// The localization unit η: X → L_E X, the universal map to an E-local spectrum.
pub fn localization_unit_ty() -> Expr {
    impl_pi(
        "E",
        type0(),
        impl_pi(
            "X",
            type0(),
            arrow(app2(cst("BousfieldLocalization"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `AcyclicMorphism : Type → Type → Type → Prop`
///
/// An E-acyclic morphism f: X → Y for which E_*(f) is an isomorphism.
pub fn acyclic_morphism_ty() -> Expr {
    arrow(type0(), arrow(type0(), arrow(type0(), prop())))
}
/// `LocalSpectra : Type → Type`
///
/// The full subcategory of E-local spectra.
pub fn local_spectra_ty() -> Expr {
    arrow(type0(), type0())
}
/// `EllipticCohomologyTheory : Type`
///
/// An elliptic cohomology theory: a complex-oriented cohomology theory E
/// whose associated formal group is (isomorphic to) the formal group of an elliptic curve.
pub fn elliptic_cohomology_theory_ty() -> Expr {
    type0()
}
/// `EllipticCurveOverRing : Type → Type`
///
/// An elliptic curve over a ring R, given in Weierstrass form with its formal group.
pub fn elliptic_curve_over_ring_ty() -> Expr {
    arrow(type0(), type0())
}
/// `OrientationData : Type → Type → Prop`
///
/// An orientation A: MU → E_*(BU): a complex orientation of E compatible with the
/// formal group structure coming from the elliptic curve.
pub fn orientation_data_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// `LevelStructure : Nat → Type → Type`
///
/// A level structure Γ₀(n) or Γ₁(n) on an elliptic curve over a ring R.
pub fn level_structure_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), type0()))
}
/// `TopologicalModularForms : Type`
///
/// The spectrum tmf of topological modular forms,
/// with π_*(tmf) ≅ M_* (modular forms graded ring).
pub fn topological_modular_forms_ty() -> Expr {
    type0()
}
/// `ModularFormSpectrum : Type → Type`
///
/// The moduli of elliptic curves Ell(R) over a ring R, as a derived stack.
pub fn modular_form_spectrum_ty() -> Expr {
    arrow(type0(), type0())
}
/// `DescentData : Type → Type`
///
/// Descent data for the Galois action Gal(ℂ/ℝ), yielding the real spectrum tmf_ℝ
/// as homotopy fixed points tmf^{hGal(ℂ/ℝ)}.
pub fn descent_data_ty() -> Expr {
    arrow(type0(), type0())
}
/// `ModularFormDegree : Type → Nat`
///
/// The degree of a modular form (or element in π_*(tmf)).
pub fn modular_form_degree_ty() -> Expr {
    arrow(type0(), nat_ty())
}
/// `ModularFormWeight : Type → Nat`
///
/// The weight of a modular form (weight k means f(aτ+b/cτ+d) = (cτ+d)^k f(τ)).
pub fn modular_form_weight_ty() -> Expr {
    arrow(type0(), nat_ty())
}
/// `CuspForms : Nat → Nat → Type`
///
/// The space of cusp forms S_k(Γ) of weight k and level Γ (encoded as a Nat).
pub fn cusp_forms_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `IsInvertible : Type → Type → Prop`
///
/// Whether an element of π_*(tmf) is invertible.
pub fn is_invertible_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// `AdamsNovikovSS : Type → Type`
///
/// The Adams-Novikov spectral sequence:
/// E_2^{s,t} = Ext^{s,t}_{MU_*(MU)}(MU_*, MU_*(X)) ⇒ π_{t-s}(X_p^).
pub fn adams_novikov_ss_ty() -> Expr {
    arrow(type0(), type0())
}
/// `ChromaticSS : Type → Type`
///
/// The chromatic spectral sequence:
/// E_1^{n,*} = π_*(M_n X) ⇒ π_*(X_p^).
pub fn chromatic_ss_ty() -> Expr {
    arrow(type0(), type0())
}
/// `BPAdamsSpectralSequence : Type → Type`
///
/// The Brown-Peterson Adams spectral sequence using BP cohomology.
pub fn bp_adams_ss_ty() -> Expr {
    arrow(type0(), type0())
}
/// Register all chromatic homotopy theory axioms into the given kernel environment.
pub fn build_env(env: &mut Environment) -> Result<(), String> {
    add_axiom(env, "FormalGroupLaw", vec![], formal_group_law_ty())?;
    add_axiom(env, "LazardRing", vec![], lazard_ring_ty())?;
    add_axiom(env, "MoravaETheory", vec![], morava_e_theory_ty())?;
    add_axiom(env, "MoravaKTheory", vec![], morava_k_theory_ty())?;
    add_axiom(
        env,
        "ChromaticFiltration",
        vec![],
        chromatic_filtration_ty(),
    )?;
    add_axiom(
        env,
        "ChromaticConvergence",
        vec![],
        chromatic_convergence_ty(),
    )?;
    add_axiom(env, "PeriodicityThm", vec![], periodicity_thm_ty())?;
    add_axiom(env, "ChromaticHeight", vec![], height_ty())?;
    add_axiom(env, "VnPeriodicityClass", vec![], vn_periodicity_class_ty())?;
    add_axiom(env, "NilpotentElement", vec![], nilpotent_element_ty())?;
    add_axiom(
        env,
        "BousfieldLocalization",
        vec![],
        bousfield_localization_ty(),
    )?;
    add_axiom(env, "LocalizationUnit", vec![], localization_unit_ty())?;
    add_axiom(env, "AcyclicMorphism", vec![], acyclic_morphism_ty())?;
    add_axiom(env, "LocalSpectra", vec![], local_spectra_ty())?;
    add_axiom(
        env,
        "EllipticCohomologyTheory",
        vec![],
        elliptic_cohomology_theory_ty(),
    )?;
    add_axiom(
        env,
        "EllipticCurveOverRing",
        vec![],
        elliptic_curve_over_ring_ty(),
    )?;
    add_axiom(env, "OrientationData", vec![], orientation_data_ty())?;
    add_axiom(env, "LevelStructure", vec![], level_structure_ty())?;
    add_axiom(
        env,
        "TopologicalModularForms",
        vec![],
        topological_modular_forms_ty(),
    )?;
    add_axiom(
        env,
        "ModularFormSpectrum",
        vec![],
        modular_form_spectrum_ty(),
    )?;
    add_axiom(env, "DescentData", vec![], descent_data_ty())?;
    add_axiom(env, "ModularFormDegree", vec![], modular_form_degree_ty())?;
    add_axiom(env, "ModularFormWeight", vec![], modular_form_weight_ty())?;
    add_axiom(env, "CuspForms", vec![], cusp_forms_ty())?;
    add_axiom(env, "IsInvertible", vec![], is_invertible_ty())?;
    add_axiom(env, "AdamsNovikovSS", vec![], adams_novikov_ss_ty())?;
    add_axiom(env, "ChromaticSS", vec![], chromatic_ss_ty())?;
    add_axiom(env, "BPAdamsSpectralSequence", vec![], bp_adams_ss_ty())?;
    add_axiom(env, "ComplexCobordismMU", vec![], complex_cobordism_mu_ty())?;
    add_axiom(env, "BrownPetersonBP", vec![], brown_peterson_bp_ty())?;
    add_axiom(
        env,
        "LandweberExactFunctor",
        vec![],
        landweber_exact_functor_ty(),
    )?;
    add_axiom(env, "HondaFormalGroup", vec![], honda_formal_group_ty())?;
    add_axiom(env, "NilpotenceThm", vec![], nilpotence_thm_ty())?;
    add_axiom(env, "ThickSubcategory", vec![], thick_subcategory_ty())?;
    add_axiom(env, "TypeNComplex", vec![], type_n_complex_ty())?;
    add_axiom(env, "VnSelfMap", vec![], vn_self_map_ty())?;
    add_axiom(
        env,
        "TelescopeConjecture",
        vec![],
        telescope_conjecture_ty(),
    )?;
    add_axiom(env, "LubinTateSpace", vec![], lubin_tate_space_ty())?;
    add_axiom(
        env,
        "MoravaStabilizerGroup",
        vec![],
        morava_stabilizer_group_ty(),
    )?;
    add_axiom(
        env,
        "GrossHopkinsDuality",
        vec![],
        gross_hopkins_duality_ty(),
    )?;
    add_axiom(env, "LnLocalization", vec![], ln_localization_ty())?;
    add_axiom(env, "MonochromaticLayer", vec![], monochromatic_layer_ty())?;
    add_axiom(
        env,
        "ChromaticLocalization",
        vec![],
        chromatic_localization_ty(),
    )?;
    add_axiom(
        env,
        "TopologicalHochschildHomology",
        vec![],
        topological_hochschild_homology_ty(),
    )?;
    add_axiom(
        env,
        "TopologicalCyclicHomology",
        vec![],
        topological_cyclic_homology_ty(),
    )?;
    add_axiom(env, "TateConstruction", vec![], tate_construction_ty())?;
    add_axiom(env, "NormMap", vec![], norm_map_ty())?;
    add_axiom(
        env,
        "HomotopyFixedPoints",
        vec![],
        homotopy_fixed_points_ty(),
    )?;
    add_axiom(
        env,
        "HomotopyOrbitSpectrum",
        vec![],
        homotopy_orbit_spectrum_ty(),
    )?;
    add_axiom(env, "AmbidexterityThm", vec![], ambidexterity_thm_ty())?;
    add_axiom(env, "RedshiftConjecture", vec![], redshift_conjecture_ty())?;
    add_axiom(
        env,
        "EInfinityRingSpectrum",
        vec![],
        e_infinity_ring_spectrum_ty(),
    )?;
    add_axiom(env, "SphericalSpectrum", vec![], spherical_spectrum_ty())?;
    add_axiom(
        env,
        "ChromaticComplexity",
        vec![],
        chromatic_complexity_ty(),
    )?;
    add_axiom(env, "WittVectors", vec![], witt_vectors_ty())?;
    add_axiom(
        env,
        "ChromaticDescentData",
        vec![],
        chromatic_descent_data_ty(),
    )?;
    Ok(())
}
/// `ComplexCobordismMU : Type`
///
/// The complex cobordism spectrum MU: the universal complex-oriented cohomology
/// theory. π_*(MU) ≅ L (the Lazard ring).
pub fn complex_cobordism_mu_ty() -> Expr {
    type0()
}
/// `BrownPetersonBP : Nat → Type`
///
/// The Brown-Peterson spectrum BP at prime p (encoded as a Nat index).
/// BP is the direct summand of MU localized at p; π_*(BP) ≅ ℤ_(p)[v_1, v_2, ...].
pub fn brown_peterson_bp_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `LandweberExactFunctor : Type → Type → Prop`
///
/// The Landweber exact functor theorem: a flat map L → R of rings gives a
/// complex-oriented cohomology theory R_*(−) = R ⊗_L MU_*(−).
pub fn landweber_exact_functor_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// `HondaFormalGroup : Nat → Nat → Type`
///
/// The Honda formal group H_n at height n over F_{p^n}: the unique (up to
/// isomorphism) height-n formal group over the algebraic closure of F_p,
/// with [p]-series x^{p^n}.
pub fn honda_formal_group_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `NilpotenceThm : Type → Type → Prop`
///
/// The Devinatz-Hopkins-Smith nilpotence theorem: a ring map R → MU inducing
/// zero on homotopy groups implies R is nilpotent.
pub fn nilpotence_thm_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// `ThickSubcategory : Nat → Type`
///
/// A thick subcategory C(n) of finite spectra; C(n) = {X | K(m)_*(X) = 0 for m < n}.
pub fn thick_subcategory_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `TypeNComplex : Nat → Type → Prop`
///
/// A finite CW-complex X is of type n if K(n-1)_*(X) = 0 and K(n)_*(X) ≠ 0.
pub fn type_n_complex_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), prop()))
}
/// `VnSelfMap : Nat → Type → Type`
///
/// A v_n self-map on a type-n finite complex X.
pub fn vn_self_map_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), type0()))
}
/// `TelescopeConjecture : Nat → Type → Prop`
///
/// Ravenel's telescope conjecture: v_n^{-1} X ≃ L_{K(n)} X.
/// Open at heights n ≥ 2; proved at n = 1.
pub fn telescope_conjecture_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), prop()))
}
/// `LubinTateSpace : Nat → Nat → Type`
///
/// The Lubin-Tate deformation space at height n and prime p.
pub fn lubin_tate_space_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `MoravaStabilizerGroup : Nat → Nat → Type`
///
/// The Morava stabilizer group S_n = Aut(H_n) at height n, prime p.
pub fn morava_stabilizer_group_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `GrossHopkinsDuality : Nat → Type → Prop`
///
/// Gross-Hopkins duality in the K(n)-local category.
pub fn gross_hopkins_duality_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), prop()))
}
/// `LnLocalization : Nat → Type → Type`
///
/// The L_n = L_{E(n)}-localization functor.
pub fn ln_localization_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), type0()))
}
/// `MonochromaticLayer : Nat → Type → Type`
///
/// The monochromatic layer M_n X = fiber(L_n X → L_{n-1} X).
pub fn monochromatic_layer_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), type0()))
}
/// `ChromaticLocalization : Type → Type → Type`
///
/// Chromatic localization L_{K(n)} X at Morava K-theory K(n).
pub fn chromatic_localization_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// `TopologicalHochschildHomology : Type → Type`
///
/// THH(A): topological Hochschild homology of a ring spectrum A.
pub fn topological_hochschild_homology_ty() -> Expr {
    arrow(type0(), type0())
}
/// `TopologicalCyclicHomology : Type → Type`
///
/// TC(A): topological cyclic homology built from THH(A)^{C_{p^n}}.
pub fn topological_cyclic_homology_ty() -> Expr {
    arrow(type0(), type0())
}
/// `TateConstruction : Type → Type → Type`
///
/// The Tate construction X^{tG}.
pub fn tate_construction_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// `NormMap : Type → Type → Type`
///
/// The norm map Nm: X_{hG} → X^{hG}.
pub fn norm_map_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// `HomotopyFixedPoints : Type → Type → Type`
///
/// The homotopy fixed point spectrum X^{hG}.
pub fn homotopy_fixed_points_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// `HomotopyOrbitSpectrum : Type → Type → Type`
///
/// The homotopy orbit spectrum X_{hG}.
pub fn homotopy_orbit_spectrum_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// `AmbidexterityThm : Nat → Type → Prop`
///
/// The Hopkins-Lurie ambidexterity theorem.
pub fn ambidexterity_thm_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), prop()))
}
/// `RedshiftConjecture : Type → Prop`
///
/// The Ausoni-Rognes redshift conjecture.
pub fn redshift_conjecture_ty() -> Expr {
    arrow(type0(), prop())
}
/// `EInfinityRingSpectrum : Type → Prop`
///
/// A spectrum carries an E_∞-ring structure.
pub fn e_infinity_ring_spectrum_ty() -> Expr {
    arrow(type0(), prop())
}
/// `SphericalSpectrum : Type → Prop`
///
/// A spectrum is spherical (retract of a finite CW-spectrum).
pub fn spherical_spectrum_ty() -> Expr {
    arrow(type0(), prop())
}
/// `ChromaticComplexity : Type → Nat`
///
/// The chromatic complexity of a spectrum: smallest n with X ≃ L_n X.
pub fn chromatic_complexity_ty() -> Expr {
    arrow(type0(), nat_ty())
}
/// `WittVectors : Nat → Type → Type`
///
/// The p-typical Witt vectors W_n(R).
pub fn witt_vectors_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), type0()))
}
/// `ChromaticDescentData : Nat → Type → Prop`
///
/// Chromatic descent: S_{K(n)} recoverable from E_n via S_n-action.
pub fn chromatic_descent_data_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), prop()))
}
#[cfg(test)]
mod tests {
    use super::*;
    fn test_env() -> Environment {
        let mut env = Environment::new();
        build_env(&mut env).expect("build_env failed");
        env
    }
    #[test]
    fn test_formal_group_laws_registered() {
        let env = test_env();
        assert!(env.get(&Name::str("FormalGroupLaw")).is_some());
        assert!(env.get(&Name::str("LazardRing")).is_some());
        assert!(env.get(&Name::str("MoravaETheory")).is_some());
        assert!(env.get(&Name::str("MoravaKTheory")).is_some());
    }
    #[test]
    fn test_chromatic_filtration_registered() {
        let env = test_env();
        assert!(env.get(&Name::str("ChromaticFiltration")).is_some());
        assert!(env.get(&Name::str("ChromaticConvergence")).is_some());
        assert!(env.get(&Name::str("PeriodicityThm")).is_some());
        assert!(env.get(&Name::str("VnPeriodicityClass")).is_some());
        assert!(env.get(&Name::str("NilpotentElement")).is_some());
    }
    #[test]
    fn test_bousfield_localization_registered() {
        let env = test_env();
        assert!(env.get(&Name::str("BousfieldLocalization")).is_some());
        assert!(env.get(&Name::str("LocalizationUnit")).is_some());
        assert!(env.get(&Name::str("AcyclicMorphism")).is_some());
        assert!(env.get(&Name::str("LocalSpectra")).is_some());
    }
    #[test]
    fn test_elliptic_cohomology_registered() {
        let env = test_env();
        assert!(env.get(&Name::str("EllipticCohomologyTheory")).is_some());
        assert!(env.get(&Name::str("EllipticCurveOverRing")).is_some());
        assert!(env.get(&Name::str("OrientationData")).is_some());
        assert!(env.get(&Name::str("LevelStructure")).is_some());
    }
    #[test]
    fn test_tmf_registered() {
        let env = test_env();
        assert!(env.get(&Name::str("TopologicalModularForms")).is_some());
        assert!(env.get(&Name::str("ModularFormSpectrum")).is_some());
        assert!(env.get(&Name::str("DescentData")).is_some());
        assert!(env.get(&Name::str("CuspForms")).is_some());
    }
    #[test]
    fn test_spectral_sequences_registered() {
        let env = test_env();
        assert!(env.get(&Name::str("AdamsNovikovSS")).is_some());
        assert!(env.get(&Name::str("ChromaticSS")).is_some());
        assert!(env.get(&Name::str("BPAdamsSpectralSequence")).is_some());
    }
    #[test]
    fn test_formal_group_law_additive() {
        let fgl = FormalGroupLaw::additive(3);
        assert!(fgl.satisfies_identity());
        assert_eq!(fgl.height_at_prime(2), None);
    }
    #[test]
    fn test_formal_group_law_multiplicative() {
        let fgl = FormalGroupLaw::multiplicative(3);
        assert!(fgl.satisfies_identity());
        assert_eq!(fgl.height_at_prime(2), Some(1));
    }
    #[test]
    fn test_morava_e_theory_periodicity() {
        let e = MoravaETheory::new(1, 2);
        assert_eq!(e.periodicity(), 2);
        let e2 = MoravaETheory::new(2, 3);
        assert_eq!(e2.periodicity(), 16);
    }
    #[test]
    fn test_morava_k_theory() {
        let k = MoravaKTheory::new(1, 2);
        assert!(k.is_field_spectrum());
        assert_eq!(k.periodicity(), 2);
    }
    #[test]
    fn test_elliptic_curve_nonsingular() {
        let e = EllipticCurveOverRing {
            ring: "Z".to_string(),
            a: -1,
            b: 0,
        };
        assert!(e.is_nonsingular());
        let s = EllipticCurveOverRing {
            ring: "Z".to_string(),
            a: 0,
            b: 0,
        };
        assert!(!s.is_nonsingular());
    }
    #[test]
    fn test_modular_form_degree() {
        let f = ModularFormSpectrum {
            weight: 4,
            level: 1,
            is_cusp_form: false,
        };
        assert_eq!(f.degree(), 8);
        assert_eq!(f.weight(), 4);
    }
    #[test]
    fn test_lazard_ring_generators() {
        let l = LazardRing::new(6);
        assert_eq!(l.generators_of_degree(2).len(), 1);
        assert_eq!(l.generators_of_degree(4).len(), 1);
        assert_eq!(l.generators_of_degree(6).len(), 1);
        assert_eq!(l.generators_of_degree(3).len(), 0);
    }
    #[test]
    fn test_adams_novikov_stem() {
        let ss = AdamsNovikovSS {
            spectrum: "S".to_string(),
            prime: 2,
            e2_page: vec![(0, 0, 1), (1, 3, 1)],
        };
        assert_eq!(ss.stem(4, 1), Some(3));
        assert_eq!(ss.e2_rank(1, 3), 1);
        assert_eq!(ss.e2_rank(0, 0), 1);
    }
    #[test]
    fn test_level_structures() {
        let g0 = LevelStructure::gamma_0(4);
        assert_eq!(g0.level, 4);
        assert_eq!(g0.structure_type, "Gamma_0");
        let g1 = LevelStructure::gamma_1(5);
        assert_eq!(g1.level, 5);
        assert_eq!(g1.structure_type, "Gamma_1");
    }
    #[test]
    fn test_periodicity_theorem() {
        let p = PeriodicityThm::new(1, 2);
        assert_eq!(p.vn_degree(), 2);
        let p2 = PeriodicityThm::new(2, 3);
        assert_eq!(p2.vn_degree(), 16);
    }
    #[test]
    fn test_complex_cobordism_bp_registered() {
        let env = test_env();
        assert!(env.get(&Name::str("ComplexCobordismMU")).is_some());
        assert!(env.get(&Name::str("BrownPetersonBP")).is_some());
        assert!(env.get(&Name::str("LandweberExactFunctor")).is_some());
        assert!(env.get(&Name::str("HondaFormalGroup")).is_some());
    }
    #[test]
    fn test_nilpotence_thick_telescope_registered() {
        let env = test_env();
        assert!(env.get(&Name::str("NilpotenceThm")).is_some());
        assert!(env.get(&Name::str("ThickSubcategory")).is_some());
        assert!(env.get(&Name::str("TypeNComplex")).is_some());
        assert!(env.get(&Name::str("VnSelfMap")).is_some());
        assert!(env.get(&Name::str("TelescopeConjecture")).is_some());
    }
    #[test]
    fn test_lubin_tate_stabilizer_registered() {
        let env = test_env();
        assert!(env.get(&Name::str("LubinTateSpace")).is_some());
        assert!(env.get(&Name::str("MoravaStabilizerGroup")).is_some());
        assert!(env.get(&Name::str("GrossHopkinsDuality")).is_some());
        assert!(env.get(&Name::str("LnLocalization")).is_some());
        assert!(env.get(&Name::str("MonochromaticLayer")).is_some());
        assert!(env.get(&Name::str("ChromaticLocalization")).is_some());
    }
    #[test]
    fn test_thh_tc_registered() {
        let env = test_env();
        assert!(env
            .get(&Name::str("TopologicalHochschildHomology"))
            .is_some());
        assert!(env.get(&Name::str("TopologicalCyclicHomology")).is_some());
        assert!(env.get(&Name::str("TateConstruction")).is_some());
        assert!(env.get(&Name::str("NormMap")).is_some());
        assert!(env.get(&Name::str("HomotopyFixedPoints")).is_some());
        assert!(env.get(&Name::str("HomotopyOrbitSpectrum")).is_some());
    }
    #[test]
    fn test_ambidexterity_redshift_registered() {
        let env = test_env();
        assert!(env.get(&Name::str("AmbidexterityThm")).is_some());
        assert!(env.get(&Name::str("RedshiftConjecture")).is_some());
        assert!(env.get(&Name::str("EInfinityRingSpectrum")).is_some());
        assert!(env.get(&Name::str("SphericalSpectrum")).is_some());
        assert!(env.get(&Name::str("ChromaticComplexity")).is_some());
        assert!(env.get(&Name::str("WittVectors")).is_some());
        assert!(env.get(&Name::str("ChromaticDescentData")).is_some());
    }
    #[test]
    fn test_honda_formal_group() {
        let h2 = HondaFormalGroup::new(2, 3);
        assert_eq!(h2.p_series_degree(), 9);
        let h2b = HondaFormalGroup::new(2, 3);
        assert!(h2.isomorphic_to(&h2b));
        let h3 = HondaFormalGroup::new(3, 3);
        assert!(!h2.isomorphic_to(&h3));
    }
    #[test]
    fn test_brown_peterson_bp() {
        let bp = BrownPetersonBP::new(2, 3);
        assert_eq!(bp.vn_degree(1), 2);
        assert_eq!(bp.vn_degree(2), 6);
        assert_eq!(bp.vn_degree(3), 14);
        let gens = bp.generators();
        assert_eq!(gens.len(), 3);
        assert_eq!(gens[0].0, "v_1");
    }
    #[test]
    fn test_landweber_exact_functor() {
        let lef = LandweberExactFunctor::new("K(u)_*", 2);
        assert!(lef.produces_cohomology_theory());
        let not_exact = LandweberExactFunctor::new("BadRing", 0);
        assert!(!not_exact.produces_cohomology_theory());
    }
    #[test]
    fn test_thick_subcategory_data() {
        let mut c1 = ThickSubcategoryData::new(1, 2);
        c1.add_member("V(0)");
        c1.add_member("M(1)");
        assert!(c1.contains("V(0)"));
        assert!(!c1.contains("S^0"));
    }
    #[test]
    fn test_vn_self_map_data() {
        let v = VnSelfMapData::new(1, 2, 1);
        assert_eq!(v.period, 2);
        assert_eq!(v.telescope_name("V(0)"), "v_1^{-1} V(0)");
        let v2 = VnSelfMapData::new(2, 2, 2);
        assert_eq!(v2.period, 12);
    }
    #[test]
    fn test_lubin_tate_space_data() {
        let lt = LubinTateSpaceData::new(3, 2);
        assert_eq!(lt.num_deformation_params(), 2);
        let lt0 = LubinTateSpaceData::new(0, 2);
        assert_eq!(lt0.num_deformation_params(), 0);
    }
    #[test]
    fn test_morava_stabilizer_group_data() {
        let s1 = MoravaStabilizerGroupData::new(1, 2);
        assert_eq!(s1.center_index(), 1);
        let s2 = MoravaStabilizerGroupData::new(2, 3);
        assert_eq!(s2.center_index(), 8);
    }
    #[test]
    fn test_thh_data_struct() {
        let mut thh = TopologicalHochschildHomologyData::new("ku", true);
        thh.add_homotopy_group(0, 1);
        thh.add_homotopy_group(2, 1);
        assert_eq!(thh.pi_rank(0), 1);
        assert_eq!(thh.pi_rank(2), 1);
        assert_eq!(thh.pi_rank(1), 0);
        assert!(thh.is_e_infty);
    }
    #[test]
    fn test_tc_data_struct() {
        let tc = TopologicalCyclicHomologyData::new("HZ", 2);
        assert!(tc.cyclotomic_trace_exists());
        assert_eq!(tc.prime, 2);
    }
    #[test]
    fn test_witt_vector_ring() {
        let w = WittVectorRing::new("F_p", 2, 4);
        let result = w.add(&[1, 0, 0, 0], &[1, 0, 0, 0]);
        assert_eq!(result[0], 0);
        assert_eq!(result[1], 1);
        let shifted = WittVectorRing::frobenius_shift(&[1, 2, 3, 4]);
        assert_eq!(shifted, vec![2, 3, 4]);
        let vers = WittVectorRing::verschiebung(&[1, 2, 3]);
        assert_eq!(vers, vec![0, 1, 2, 3]);
    }
    #[test]
    fn test_adams_spectral_sequence() {
        let ss = AdamsSpectralSequence::sphere_at_2();
        assert_eq!(ss.rank_at(0, 0), 1);
        assert_eq!(ss.rank_at(1, 1), 1);
        assert_eq!(ss.rank_at(1, 7), 1);
        assert_eq!(ss.total_rank_in_stem(3), 2);
        assert_eq!(ss.rank_at(0, 5), 0);
    }
    #[test]
    fn test_morava_k_group() {
        let mut kg = MoravaKGroup::new(1, 2, "V(0)");
        kg.set_rank(0, 1);
        kg.set_rank(1, 1);
        assert_eq!(kg.rank_in_degree(0), 1);
        assert_eq!(kg.euler_characteristic(), 0);
        assert!(!kg.is_acyclic());
        let acyclic = MoravaKGroup::new(2, 2, "acyclic");
        assert!(acyclic.is_acyclic());
    }
    #[test]
    fn test_chromatic_complexity_data() {
        let mut cc = ChromaticComplexityData::new("V(1)");
        cc.set_acyclic_at(0);
        assert_eq!(cc.type_estimate(), Some(1));
        assert_eq!(cc.complexity(), 1);
        let empty = ChromaticComplexityData::new("unknown");
        assert_eq!(empty.type_estimate(), None);
        assert_eq!(empty.complexity(), 0);
        let mut cc2 = ChromaticComplexityData::new("sphere");
        cc2.set_acyclic_at(0);
        cc2.set_acyclic_at(1);
        cc2.set_acyclic_at(2);
        assert_eq!(cc2.type_estimate(), Some(3));
    }
}
#[cfg(test)]
mod tests_chromatic_ext {
    use super::*;
    #[test]
    fn test_formal_group_law_additive() {
        let fgl = FglArithmetic::additive(3);
        let val = fgl.evaluate(2.0, 3.0);
        assert!((val - 5.0).abs() < 1e-10, "Additive FGL: 2+3=5, got {val}");
    }
    #[test]
    fn test_formal_group_law_multiplicative() {
        let fgl = FglArithmetic::multiplicative(3);
        let val = fgl.evaluate(1.0, 2.0);
        assert!(
            (val - 5.0).abs() < 1e-10,
            "Multiplicative FGL: 1+2+2=5, got {val}"
        );
    }
    #[test]
    fn test_formal_group_law_commutativity() {
        let fgl = FglArithmetic::additive(4);
        assert!(
            fgl.is_commutative_approx(1e-10),
            "Additive FGL should be commutative"
        );
        let fgl2 = FglArithmetic::multiplicative(4);
        assert!(
            fgl2.is_commutative_approx(1e-10),
            "Multiplicative FGL should be commutative"
        );
    }
    #[test]
    fn test_lambda_ring_element() {
        let x = LambdaRingElement::new(1.0, 5);
        assert!((x.adams_op(1).expect("adams_op should succeed") - 1.0).abs() < 1e-10);
        assert!((x.adams_op(3).expect("adams_op should succeed") - 3.0).abs() < 1e-10);
        assert!(x.check_composition(2, 3));
    }
    #[test]
    fn test_chromatic_tower_level() {
        let mut lvl = ChromaticTowerLevel::new(2, 2);
        lvl.add_homotopy_group(0, "Z".to_string());
        assert_eq!(lvl.bousfield_class(), "<E(2,2)>");
        assert!(lvl.monochromatic_nontrivial());
        assert_eq!(lvl.periodicity_degree(), Some(6));
    }
    #[test]
    fn test_morava_k_theory() {
        let kn = MoravaKData::new(2, 3);
        assert_eq!(kn.vn_degree(), 16);
        assert!(kn.satisfies_kunneth());
        assert!(kn.coeff_of_point_is_fp());
        assert_eq!(kn.bcp_dimension(), 6);
    }
    #[test]
    fn test_nilpotence_data() {
        let mut nd = NilpotenceData::new("eta", 2);
        assert!(!nd.is_nilpotent);
        nd.set_nilpotent(4);
        assert!(nd.satisfies_nishida());
        assert_eq!(nd.nilpotency_exponent, Some(4));
    }
    #[test]
    fn test_formal_group_deformation() {
        let deform = FormalGroupDeformation::universal(3, 2);
        assert_eq!(deform.deformation_ring_dim(), 2);
        assert!(deform.is_lubin_tate());
        assert!(deform.hasse_invariant_vanishes());
    }
    #[test]
    fn test_spectral_scheme() {
        let ss = SpectralScheme::new("tmf", "M_{1,1}")
            .with_cotangent_dim(1)
            .as_dci();
        assert_eq!(ss.tor_amplitude(), "[0, 1]");
        assert!(!ss.is_spectral_dm_stack());
        assert!(ss.is_dci);
    }
    #[test]
    fn test_bp_cohomology_data() {
        let bp = BPCohomologyData::new(2, 3);
        assert_eq!(bp.vn_degree(0), Some(2));
        assert_eq!(bp.vn_degree(2), Some(6));
        assert_eq!(bp.vn_degree(3), Some(14));
    }
    #[test]
    fn test_p_series_additive() {
        let fgl = FglArithmetic::additive(5);
        let x = 0.5;
        let ps = fgl.p_series(3, x);
        assert!((ps - 3.0 * x).abs() < 1e-9, "Additive [3](x)=3x, got {ps}");
    }
    #[test]
    fn test_bp_rank_query() {
        let mut bp = BPCohomologyData::new(2, 2);
        bp.add_bp_rank(0, 1);
        bp.add_bp_rank(2, 1);
        bp.add_bp_rank(4, 2);
        assert_eq!(bp.rank_in_degree(0), 1);
        assert_eq!(bp.rank_in_degree(4), 2);
        assert_eq!(bp.rank_in_degree(99), 0);
    }
}
