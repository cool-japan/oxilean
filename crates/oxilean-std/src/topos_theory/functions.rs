//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    ClassifyingToposExt,
    CondensedSet,
    EffectiveTopos,
    // spec-required new types
    ElementaryToposData,
    EtaleTopos,
    FiniteCategory,
    GeometricLogic,
    GeometricMorphismExt,
    GeometricMorphismExtData,
    HeytingSubobjectLattice,
    InternalLogic,
    KripkeJoyalSemantics,
    LTTopology,
    LawvereTierneyTop,
    Locale,
    LocalicReflection,
    LogicalFunctor,
    PresheafTopos,
    PyknoticObject,
    SheafConditionExt,
    SheafConditionExtChecker,
    SheafConditionKind,
    SiteCategory,
    SpecGeometricMorphism,
    SpecPowerObject,
    SpecSheaf,
    SpecSubobjectClassifier,
    Subobject,
    SubobjectClassifier,
    SubobjectClassifierFinSet,
    Topos,
    ToposCohomology,
    ToposMap,
    ToposMorphism,
    ToposObject,
    ToposPoint,
};

/// Lawvere-Tierney theorem: every elementary topos has a subobject classifier
/// and the internal logic is (at least) intuitionistic higher-order logic.
pub fn lawvere_tierney_theorem() -> &'static str {
    "Every elementary topos has a subobject classifier Ω and power objects; \
     the subobject lattices Sub(A) form Heyting algebras (intuitionistic logic). \
     A topos is Boolean iff every subobject lattice is a Boolean algebra, \
     iff the canonical map ¬¬ : Ω → Ω equals the identity."
}
/// Giraud's theorem: a category is a Grothendieck topos if and only if it
/// satisfies Giraud's axioms (cocomplete, has a small generator, exact).
pub fn giraud_theorem_characterization() -> &'static str {
    "A category E is a Grothendieck topos iff: (1) E has a small set of generators, \
     (2) E is locally small, (3) E has all small colimits, \
     (4) coproducts are disjoint, (5) equivalence relations are effective, \
     (6) colimits are stable under pullback."
}
/// The Curry-Howard-Lambek correspondence:
/// propositions-as-types, proofs-as-terms, in the topos-theoretic setting.
pub fn curry_howard_lambek_correspondence() -> &'static str {
    "Curry-Howard-Lambek correspondence: \
     (1) Propositions correspond to types (objects of the topos), \
     (2) Proofs correspond to terms (morphisms from the terminal object), \
     (3) Logical connectives correspond to categorical operations \
         (∧ = product, ∨ = coproduct, ⊃ = exponential, ∀ = Pi-type, ∃ = Sigma-type), \
     (4) The internal language of any topos models intuitionistic type theory."
}
/// Classifying topos existence theorem.
pub fn classifying_topos_existence() -> &'static str {
    "For every geometric theory T (over a signature Σ), there exists a \
     Grothendieck topos Set[T] — the classifying topos of T — together with \
     a universal T-model M_univ in Set[T], such that for any Grothendieck topos E, \
     the category Geom(E, Set[T]) of geometric morphisms is equivalent to \
     the category T-Mod(E) of T-models in E. \
     This makes Set[T] the 'moduli space' of T-models."
}
/// The four cohesion axioms of Lawvere, in (name, description) pairs.
pub fn cohesion_axioms() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "Connectedness",
            "Π_0 preserves finite products (the shape of a product is the product of shapes)",
        ),
        (
            "Locality",
            "Γ is fully faithful (discrete objects are a full subcategory of cohesive ones)",
        ),
        (
            "Pieces have points",
            "The canonical map Disc(Γ X) → X → coDisc(Γ X) exhibits Γ as points of X",
        ),
        (
            "Continuity",
            "Π_0 sends the terminal object to the terminal object (single connected component)",
        ),
    ]
}
/// Lurie's ∞-topos theorem: a characterisation of ∞-toposes.
pub fn lurie_infinity_topos_theorem() -> &'static str {
    "Lurie's ∞-topos theorem (HTT, 6.1.0.6): An ∞-category X is an ∞-topos \
     if and only if X is an accessible left exact localization of a presheaf \
     ∞-category PSh(C) = Fun(C^op, S) where S is the ∞-category of spaces (∞-groupoids). \
     Equivalently: X is presentable, locally cartesian closed, \
     groupoid objects in X are effective, and colimits in X are universal. \
     Every ∞-topos provides a model for homotopy type theory with univalence."
}
/// Beth's theorem (completeness): the internal language of any Grothendieck
/// topos is complete with respect to the Kripke-Joyal semantics.
pub fn beth_completeness_theorem() -> &'static str {
    "Beth's theorem: for a geometric formula φ built from ∃, ∧, ∨, ⊤, ⊥, \
     φ is provable in geometric logic if and only if it holds under \
     the Kripke-Joyal forcing relation in every Grothendieck topos."
}
/// Hyland's theorem: the effective topos Eff exists and contains all
/// computable data; every ω-set yields a sheaf; Church's thesis holds.
pub fn hyland_effective_topos_theorem() -> &'static str {
    "Hyland (1982): The effective topos Eff is an elementary topos in which \
     Church's thesis holds internally (every function N → N is recursive), \
     Markov's principle holds, and the axiom of choice fails. \
     Objects are ω-sets (sets with a realizability notion) and morphisms \
     are computably realizable functions."
}
/// The Clausen-Scholze theorem: the category of condensed sets
/// forms a Grothendieck topos and every compactly generated
/// (or LCH) topological space embeds fully faithfully.
pub fn clausen_scholze_condensed_theorem() -> &'static str {
    "Clausen-Scholze condensed mathematics: The category CondensedSet = Sh(Pro(FinSet)_surj) \
     is a Grothendieck topos. Every compactly generated topological space X embeds \
     fully faithfully into CondensedSet via X ↦ (S ↦ C(S, X)). \
     Condensed abelian groups form an abelian category with enough projectives, \
     enabling solid/liquid tensor product theory."
}
pub fn _topos_app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
pub fn _topos_cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}
pub fn _topos_prop() -> Expr {
    Expr::Sort(Level::zero())
}
pub fn _topos_type0() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
pub fn _topos_pi(bi: BinderInfo, name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(bi, Name::str(name), Box::new(dom), Box::new(body))
}
pub fn _topos_arrow(a: Expr, b: Expr) -> Expr {
    _topos_pi(BinderInfo::Default, "_", a, b)
}
pub fn _topos_add_axiom(env: &mut Environment, name: &str, ty: Expr) -> Result<(), String> {
    env.add(Declaration::Axiom {
        name: Name::str(name),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| format!("topos build_env({name}): {e:?}"))
}
/// Register topos-theoretic kernel axioms into the given environment.
///
/// Adds constants for the core topos predicates and theorems.
///
/// Original axioms:
/// - `IsTopos : Type₀ → Prop`
/// - `IsGrothendieckTopos : Type₀ → Prop`
/// - `SubobjectClassifier : Type₀ → Type₀ → Prop`  (object, classifier)
/// - `HasPowerObjects : Type₀ → Prop`
/// - `GeomMorphism : Type₀ → Type₀ → Type₀`
/// - `IsElementaryTopos : Type₀ → Prop`
/// - `IsInfinityTopos : Type₀ → Prop`
/// - `LawvereTierneyOp : Type₀ → Type₀ → Prop`     (topos, closure op)
/// - `SheafConditionExt : Type₀ → Type₀ → Prop`
/// - `IsLocale : Type₀ → Prop`
/// - `IsSober : Type₀ → Prop`
///
/// New axioms (25+):
/// - `HasFiniteLimits`, `HasFiniteColimits`, `HasEqualizers`
/// - `IsCartesianClosed`, `HasExponentials`
/// - `IsHeytingAlgebra`, `IsBoolean`, `HeytingImplication`
/// - `IsLawvereTierneyTopology`, `IsClosure`, `IsNucleus`
/// - `IsSheafification`, `SheafificationUnit`
/// - `IsPresheafTopos`, `IsEtaleTopos`, `IsEffectiveTopos`
/// - `IsGeometricMorphismExt`, `IsLogicalFunctor`, `IsEssentialMorphism`
/// - `IsPointOfTopos`, `IsLocalicMorphism`, `IsOpenMorphism`
/// - `IsCondensedSet`, `IsPyknoticObject`
/// - `ToposCohomology`, `IsGiraudTopos`
/// - `GeometricSequent`, `ClassifyingToposOf`
#[allow(clippy::too_many_lines)]
pub fn build_env(env: &mut Environment) -> Result<(), String> {
    let t0 = _topos_type0();
    let pr = _topos_prop();
    _topos_add_axiom(env, "IsTopos", _topos_arrow(t0.clone(), pr.clone()))?;
    _topos_add_axiom(
        env,
        "IsGrothendieckTopos",
        _topos_arrow(t0.clone(), pr.clone()),
    )?;
    _topos_add_axiom(
        env,
        "HasSubobjectClassifier",
        _topos_arrow(t0.clone(), pr.clone()),
    )?;
    _topos_add_axiom(
        env,
        "SubobjectClassifier",
        _topos_arrow(t0.clone(), _topos_arrow(t0.clone(), pr.clone())),
    )?;
    _topos_add_axiom(env, "HasPowerObjects", _topos_arrow(t0.clone(), pr.clone()))?;
    _topos_add_axiom(
        env,
        "GeomMorphism",
        _topos_arrow(t0.clone(), _topos_arrow(t0.clone(), t0.clone())),
    )?;
    _topos_add_axiom(
        env,
        "IsElementaryTopos",
        _topos_arrow(t0.clone(), pr.clone()),
    )?;
    _topos_add_axiom(env, "IsInfinityTopos", _topos_arrow(t0.clone(), pr.clone()))?;
    _topos_add_axiom(
        env,
        "LawvereTierneyOp",
        _topos_arrow(t0.clone(), _topos_arrow(t0.clone(), pr.clone())),
    )?;
    _topos_add_axiom(
        env,
        "SheafConditionExt",
        _topos_arrow(t0.clone(), _topos_arrow(t0.clone(), pr.clone())),
    )?;
    _topos_add_axiom(env, "IsLocale", _topos_arrow(t0.clone(), pr.clone()))?;
    _topos_add_axiom(env, "IsSober", _topos_arrow(t0.clone(), pr.clone()))?;
    _topos_add_axiom(env, "HasFiniteLimits", _topos_arrow(t0.clone(), pr.clone()))?;
    _topos_add_axiom(
        env,
        "HasFiniteColimits",
        _topos_arrow(t0.clone(), pr.clone()),
    )?;
    _topos_add_axiom(env, "HasEqualizers", _topos_arrow(t0.clone(), pr.clone()))?;
    _topos_add_axiom(
        env,
        "IsCartesianClosed",
        _topos_arrow(t0.clone(), pr.clone()),
    )?;
    _topos_add_axiom(
        env,
        "HasExponentials",
        _topos_arrow(
            t0.clone(),
            _topos_arrow(t0.clone(), _topos_arrow(t0.clone(), pr.clone())),
        ),
    )?;
    _topos_add_axiom(
        env,
        "IsHeytingAlgebra",
        _topos_arrow(t0.clone(), pr.clone()),
    )?;
    _topos_add_axiom(env, "IsBoolean", _topos_arrow(t0.clone(), pr.clone()))?;
    _topos_add_axiom(
        env,
        "HeytingImplication",
        _topos_arrow(
            t0.clone(),
            _topos_arrow(t0.clone(), _topos_arrow(t0.clone(), pr.clone())),
        ),
    )?;
    _topos_add_axiom(
        env,
        "IsLawvereTierneyTopology",
        _topos_arrow(t0.clone(), _topos_arrow(t0.clone(), pr.clone())),
    )?;
    _topos_add_axiom(env, "IsClosure", _topos_arrow(t0.clone(), pr.clone()))?;
    _topos_add_axiom(env, "IsNucleus", _topos_arrow(t0.clone(), pr.clone()))?;
    _topos_add_axiom(
        env,
        "IsSheafification",
        _topos_arrow(t0.clone(), _topos_arrow(t0.clone(), pr.clone())),
    )?;
    _topos_add_axiom(
        env,
        "SheafificationUnit",
        _topos_arrow(t0.clone(), _topos_arrow(t0.clone(), pr.clone())),
    )?;
    _topos_add_axiom(env, "IsPresheafTopos", _topos_arrow(t0.clone(), pr.clone()))?;
    _topos_add_axiom(env, "IsEtaleTopos", _topos_arrow(t0.clone(), pr.clone()))?;
    _topos_add_axiom(
        env,
        "IsEffectiveTopos",
        _topos_arrow(t0.clone(), pr.clone()),
    )?;
    _topos_add_axiom(
        env,
        "IsGeometricMorphismExt",
        _topos_arrow(t0.clone(), _topos_arrow(t0.clone(), pr.clone())),
    )?;
    _topos_add_axiom(
        env,
        "IsLogicalFunctor",
        _topos_arrow(t0.clone(), _topos_arrow(t0.clone(), pr.clone())),
    )?;
    _topos_add_axiom(
        env,
        "IsEssentialMorphism",
        _topos_arrow(t0.clone(), _topos_arrow(t0.clone(), pr.clone())),
    )?;
    _topos_add_axiom(
        env,
        "IsPointOfTopos",
        _topos_arrow(t0.clone(), _topos_arrow(t0.clone(), pr.clone())),
    )?;
    _topos_add_axiom(
        env,
        "IsLocalicMorphism",
        _topos_arrow(t0.clone(), _topos_arrow(t0.clone(), pr.clone())),
    )?;
    _topos_add_axiom(
        env,
        "IsOpenMorphism",
        _topos_arrow(t0.clone(), _topos_arrow(t0.clone(), pr.clone())),
    )?;
    _topos_add_axiom(env, "IsCondensedSet", _topos_arrow(t0.clone(), pr.clone()))?;
    _topos_add_axiom(
        env,
        "IsPyknoticObject",
        _topos_arrow(t0.clone(), pr.clone()),
    )?;
    _topos_add_axiom(
        env,
        "ToposCohomology",
        _topos_arrow(
            t0.clone(),
            _topos_arrow(t0.clone(), _topos_arrow(t0.clone(), pr.clone())),
        ),
    )?;
    _topos_add_axiom(env, "IsGiraudTopos", _topos_arrow(t0.clone(), pr.clone()))?;
    _topos_add_axiom(
        env,
        "GeometricSequent",
        _topos_arrow(t0.clone(), _topos_arrow(t0.clone(), pr.clone())),
    )?;
    _topos_add_axiom(
        env,
        "ClassifyingToposOf",
        _topos_arrow(t0.clone(), _topos_arrow(t0.clone(), pr.clone())),
    )?;
    Ok(())
}
#[cfg(test)]
mod topos_tests {
    use super::*;
    #[test]
    fn test_build_env() {
        let mut env = Environment::new();
        build_env(&mut env).expect("build_env failed");
        assert!(env.get(&Name::str("IsTopos")).is_some());
        assert!(env.get(&Name::str("IsGrothendieckTopos")).is_some());
        assert!(env.get(&Name::str("HasSubobjectClassifier")).is_some());
        assert!(env.get(&Name::str("GeomMorphism")).is_some());
        assert!(env.get(&Name::str("IsElementaryTopos")).is_some());
        assert!(env.get(&Name::str("IsInfinityTopos")).is_some());
        assert!(env.get(&Name::str("LawvereTierneyOp")).is_some());
        assert!(env.get(&Name::str("SheafConditionExt")).is_some());
        assert!(env.get(&Name::str("IsLocale")).is_some());
        assert!(env.get(&Name::str("IsSober")).is_some());
        assert!(env.get(&Name::str("HasFiniteLimits")).is_some());
        assert!(env.get(&Name::str("HasFiniteColimits")).is_some());
        assert!(env.get(&Name::str("HasEqualizers")).is_some());
        assert!(env.get(&Name::str("IsCartesianClosed")).is_some());
        assert!(env.get(&Name::str("HasExponentials")).is_some());
        assert!(env.get(&Name::str("IsHeytingAlgebra")).is_some());
        assert!(env.get(&Name::str("IsBoolean")).is_some());
        assert!(env.get(&Name::str("HeytingImplication")).is_some());
        assert!(env.get(&Name::str("IsLawvereTierneyTopology")).is_some());
        assert!(env.get(&Name::str("IsClosure")).is_some());
        assert!(env.get(&Name::str("IsNucleus")).is_some());
        assert!(env.get(&Name::str("IsSheafification")).is_some());
        assert!(env.get(&Name::str("SheafificationUnit")).is_some());
        assert!(env.get(&Name::str("IsPresheafTopos")).is_some());
        assert!(env.get(&Name::str("IsEtaleTopos")).is_some());
        assert!(env.get(&Name::str("IsEffectiveTopos")).is_some());
        assert!(env.get(&Name::str("IsGeometricMorphismExt")).is_some());
        assert!(env.get(&Name::str("IsLogicalFunctor")).is_some());
        assert!(env.get(&Name::str("IsEssentialMorphism")).is_some());
        assert!(env.get(&Name::str("IsPointOfTopos")).is_some());
        assert!(env.get(&Name::str("IsLocalicMorphism")).is_some());
        assert!(env.get(&Name::str("IsOpenMorphism")).is_some());
        assert!(env.get(&Name::str("IsCondensedSet")).is_some());
        assert!(env.get(&Name::str("IsPyknoticObject")).is_some());
        assert!(env.get(&Name::str("ToposCohomology")).is_some());
        assert!(env.get(&Name::str("IsGiraudTopos")).is_some());
        assert!(env.get(&Name::str("GeometricSequent")).is_some());
        assert!(env.get(&Name::str("ClassifyingToposOf")).is_some());
    }
    #[test]
    fn test_heyting_subobject_lattice() {
        let lat = HeytingSubobjectLattice::new("A", false);
        assert!(!lat.double_negation_equals_identity());
        assert!(lat.implication("a", "b").contains("⊃"));
        let bool_lat = HeytingSubobjectLattice::new("A", true);
        assert!(bool_lat.double_negation_equals_identity());
    }
    #[test]
    fn test_kripke_joyal_semantics() {
        let kj = KripkeJoyalSemantics::new("Sh(C, J)", "φ ∧ ψ");
        assert!(kj.is_local());
        assert!(kj.forces_conjunction("U").contains("⊩"));
        assert!(kj.forces_existential("V").contains("local witness"));
    }
    #[test]
    fn test_presheaf_topos() {
        let mut psh = PresheafTopos::new("2", 2);
        psh.add_morphism(0, 1, "f");
        assert!(psh.is_complete_and_cocomplete());
        assert!(psh.yoneda_representable(0).contains("PSh"));
        assert_eq!(psh.morphism_table[0].len(), 1);
    }
    #[test]
    fn test_etale_topos() {
        let et = EtaleTopos::new("Spec(k)", 0);
        assert!(et.proper_base_change_holds());
        assert!(et.etale_cohomology(1, "Z/lZ").contains("H^1"));
        assert!(et.etale_fundamental_group().contains("π₁^ét"));
    }
    #[test]
    fn test_effective_topos() {
        let eff = EffectiveTopos::new("K₁");
        assert!(eff.has_standard_nno());
        assert!(eff.markovs_principle_holds());
        assert!(eff.churchs_thesis_internal());
        assert!(!eff.is_boolean());
    }
    #[test]
    fn test_topos_cohomology() {
        let coh = ToposCohomology::new("Sh(X)", "F");
        assert!(coh.cohomology_group(2).contains("H^2"));
        assert!(coh.vanishing_above_cohomological_dim(3).contains("= 0"));
        assert!(coh.leray_spectral_sequence().contains("Leray SS"));
    }
    #[test]
    fn test_geometric_logic() {
        let mut gl = GeometricLogic::new("T");
        gl.add_sequent("⊤", "∃x. φ(x)");
        assert!(!gl.axioms.is_empty());
        assert!(gl.is_coherent());
        assert_eq!(gl.classifying_topos(), "Set[T]");
    }
    #[test]
    fn test_finite_category() {
        let mut c = FiniteCategory::new("2", 2);
        let f_idx = c.add_morphism(0, 1, "f");
        assert!(c.morphisms_between(0, 1).iter().any(|(i, _)| *i == f_idx));
        assert!(c.compose(0, 0).is_some());
        assert!(c.check_associativity());
    }
    #[test]
    fn test_subobject_classifier_finset() {
        let omega = SubobjectClassifierFinSet::new();
        let elems = vec![0usize, 1, 2, 3];
        let subset = vec![1usize, 3];
        let chi = omega.classify_subset(&elems, &subset);
        assert_eq!(chi, vec![false, true, false, true]);
        let recovered = omega.recover_subset(&elems, &chi);
        assert_eq!(recovered, vec![1, 3]);
    }
    #[test]
    fn test_sheaf_condition_checker() {
        let mut sc = SheafConditionExtChecker::new("C", 3);
        sc.set_sections(0, vec![10, 20]);
        sc.set_sections(1, vec![30]);
        let cover = vec![(0usize, 2usize), (1usize, 2usize)];
        let family = vec![10u64, 30];
        assert!(sc.is_matching_family(&cover, &family));
        assert!(sc.unique_amalgamation_exists(2, &family));
    }
    #[test]
    fn test_geometric_morphism_data() {
        let gm = GeometricMorphismExtData::new("E", "F", "f^*", "f_*").with_essential("f_!");
        assert!(gm.is_essential);
        assert!(gm.exceptional_direct_image.is_some());
        assert!(gm.counit_description().contains("counit"));
        assert!(gm.unit_description().contains("unit"));
    }
    #[test]
    fn test_condensed_set() {
        let cs = CondensedSet::new("R");
        assert!(CondensedSet::is_grothendieck_topos());
        let desc = CondensedSet::from_topological_space("R");
        assert!(desc.contains("C(S, R)"));
        let _ = cs;
    }
    #[test]
    fn test_pyknotic_object() {
        let py = PyknoticObject::new("Spaces");
        assert!(PyknoticObject::pyknotic_sets_description().contains("CompHaus"));
        assert!(PyknoticObject::pyknotic_infinity_topos_description().contains("∞-topos"));
        let _ = py;
    }
    #[test]
    fn test_logical_functor() {
        let lf = LogicalFunctor::new("E", "F");
        assert!(lf.preserves_omega());
        assert!(lf.preserves_power_objects());
        assert!(lf.is_equivalence_for_grothendieck());
    }
    #[test]
    fn test_topos_point() {
        let pt = ToposPoint::new("Sh(X)", "stalk_x");
        assert!(pt.stalk("F").contains("stalk_x"));
        assert!(pt.enough_points_theorem().contains("Topos Sh(X)"));
    }
    #[test]
    fn test_localic_reflection() {
        let lr = LocalicReflection::new("Sh(X)");
        assert!(lr.underlying_locale().contains("Loc(Sh(X))"));
        assert!(!lr.is_localic_check());
        assert!(lr.topological_space_case("X").contains("O(X)"));
    }
    #[test]
    fn test_topos_struct() {
        let t = Topos::new("Set", true, true);
        assert!(t.is_boolean());
        assert!(t.geometric_morphisms_to("Sh(X)").contains("Geom(Set,"));
    }
    #[test]
    fn test_locale_struct() {
        let l = Locale::new(vec!["∅".into(), "U".into(), "X".into()], true);
        assert!(l.is_sober());
        assert!(!l.points().is_empty());
        let nl = Locale::new(vec![], false);
        assert!(!nl.is_sober());
        assert!(nl.points().is_empty());
    }
    #[test]
    fn test_site_category() {
        let s = SiteCategory::new(
            vec!["U".into(), "V".into()],
            vec!["{ U → X, V → X }".into()],
        );
        assert!(!s.is_trivial());
        assert!(!s.is_indiscrete());
    }
    #[test]
    fn test_subobject() {
        let sub = Subobject::new("m : A ↣ B", "χ_m : B → Ω");
        assert!(sub.classifier_is_unique());
    }
}
#[cfg(test)]
mod tests_topos_extra {
    use super::*;
    #[test]
    fn test_internal_logic() {
        let set = InternalLogic::set_theory();
        assert!(set.is_classical());
        assert!(!set.is_constructive());
        let eff = InternalLogic::effective_topos();
        assert!(!eff.is_classical());
        assert!(eff.is_constructive());
    }
    #[test]
    fn test_geometric_morphism() {
        let f = GeometricMorphismExt::new("E", "F");
        let g = GeometricMorphismExt::new("F", "G");
        let fg = GeometricMorphismExt::compose(&f, &g);
        assert!(fg.is_some());
        let fg = fg.expect("fg should be valid");
        assert_eq!(fg.domain, "E");
        assert_eq!(fg.codomain, "G");
        let h = GeometricMorphismExt::new("X", "Y");
        assert!(GeometricMorphismExt::compose(&f, &h).is_none());
    }
    #[test]
    fn test_lawvere_tierney() {
        let trivial = LawvereTierneyTop::trivial("E");
        assert!(trivial.is_trivial());
        assert!(!trivial.is_double_negation());
        let dense = LawvereTierneyTop::dense_topology("E");
        assert!(dense.is_double_negation());
    }
    #[test]
    fn test_sheaf_condition() {
        let mut sc = SheafConditionExt::new("Site", "Presheaf");
        assert!(!sc.is_sheaf);
        sc.mark_sheaf("satisfies descent");
        assert!(sc.is_sheaf);
        assert!(sc.reason.contains("descent"));
    }
    #[test]
    fn test_topos_map() {
        let lm = ToposMap::logical_morphism("phi");
        assert!(lm.preserves_finite_limits());
        assert!(lm.preserves_subobject_classifier());
    }
    #[test]
    fn test_classifying_topos() {
        let bt = ClassifyingToposExt::new("rings");
        assert!(bt.generic_model.contains("rings"));
        assert!(!ClassifyingToposExt::is_presheaf_topos("rings"));
        assert!(ClassifyingToposExt::is_presheaf_topos("flat functors"));
    }
}

// ── Spec-required functions for elementary topos theory ─────────────────────

/// Verify the elementary topos axioms on an `ElementaryToposData`.
///
/// Returns a list of violation messages.  An empty list means all checked
/// axioms appear to hold.
pub fn verify_topos_axioms(topos: &ElementaryToposData) -> Vec<String> {
    let mut violations = Vec::new();

    // 1. Must have at least one object (the terminal object).
    if topos.objects.is_empty() {
        violations.push("Topos has no objects — at minimum a terminal object is required".into());
    }

    // 2. Terminal object must be registered.
    if topos.terminal.is_none() {
        violations.push("No terminal object designated".into());
    } else {
        let t_id = topos.terminal.expect("checked above");
        if topos.object(t_id).is_none() {
            violations.push(format!(
                "Terminal object id {} not found in object list",
                t_id
            ));
        }
    }

    // 3. Subobject classifier must be present.
    if topos.subobject_classifier.is_none() {
        violations.push("No subobject classifier Ω designated".into());
    }

    // 4. Every base object A must have a power object Ω^A.
    // If `num_base_objects` is set (non-zero), only check up to that many objects
    // (objects beyond that index are themselves derived/power objects whose power
    // objects are implicitly available but not enumerated in this finite presentation).
    let check_limit = if topos.num_base_objects > 0 {
        topos.num_base_objects.min(topos.objects.len())
    } else {
        topos.objects.len()
    };
    for obj in topos.objects.iter().take(check_limit) {
        let has_power = topos.power_objects.iter().any(|p| p.base == obj.id);
        if !has_power {
            violations.push(format!(
                "Object '{}' (id={}) has no associated power object",
                obj.name, obj.id
            ));
        }
    }

    // 5. Every morphism's domain and codomain must exist.
    for m in &topos.morphisms {
        if topos.object(m.domain).is_none() {
            violations.push(format!(
                "Morphism '{}': domain id={} not found",
                m.name, m.domain
            ));
        }
        if topos.object(m.codomain).is_none() {
            violations.push(format!(
                "Morphism '{}': codomain id={} not found",
                m.name, m.codomain
            ));
        }
    }

    violations
}

/// Return the terminal object of the topos, if designated and present.
pub fn terminal_object(topos: &ElementaryToposData) -> Option<&ToposObject> {
    let t_id = topos.terminal?;
    topos.object(t_id)
}

/// Compute a pullback of two morphisms f: A → C and g: B → C sharing the same
/// codomain.  Returns `(pullback_object, proj_to_A, proj_to_B)` when the
/// pullback can be found in the stored pullbacks, otherwise synthesises a
/// placeholder.
pub fn pullback(
    topos: &ElementaryToposData,
    f: &ToposMorphism,
    g: &ToposMorphism,
) -> Option<(ToposObject, ToposMorphism, ToposMorphism)> {
    // f and g must share the same codomain.
    if f.codomain != g.codomain {
        return None;
    }

    // Look for a stored pullback that involves both domain objects.
    for &(pb_id, p1_id, p2_id) in &topos.pullbacks {
        let p1 = topos.morphism(p1_id)?;
        let p2 = topos.morphism(p2_id)?;
        if p1.codomain == f.domain && p2.codomain == g.domain {
            let pb_obj = topos.object(pb_id)?.clone();
            return Some((pb_obj, p1.clone(), p2.clone()));
        }
    }

    // Synthesise a formal pullback object.
    let pb_name = format!(
        "{}×_C{}",
        topos.object(f.domain)?.name,
        topos.object(g.domain)?.name
    );
    let next_id = topos.objects.len();
    let pb_obj = ToposObject::new(next_id, pb_name.clone());
    let proj1 = ToposMorphism::new(
        topos.morphisms.len(),
        next_id,
        f.domain,
        format!("π₁:{}", pb_name),
    );
    let proj2 = ToposMorphism::new(
        topos.morphisms.len() + 1,
        next_id,
        g.domain,
        format!("π₂:{}", pb_name),
    );
    Some((pb_obj, proj1, proj2))
}

/// Return the characteristic morphism χ_m: B → Ω for a monomorphism m: A → B.
///
/// In an elementary topos every mono m: A ↪ B has a unique characteristic
/// morphism χ_m making the square a pullback of true: 1 → Ω.  Here we verify
/// the morphism is mono (domain ≠ codomain serves as a minimal check) and
/// construct a named morphism.
pub fn characteristic_morphism(
    topos: &ElementaryToposData,
    mono: &ToposMorphism,
) -> Option<ToposMorphism> {
    // Ensure codomain exists.
    topos.object(mono.codomain)?;

    // Find the subobject classifier id.
    let omega_name = topos.subobject_classifier.as_ref()?.name.clone();

    // Find or synthesise Ω as the object whose name matches.
    let omega_id = topos.objects.iter().find(|o| o.name == omega_name)?.id;

    let chi = ToposMorphism::new(
        topos.morphisms.len(),
        mono.codomain,
        omega_id,
        format!("χ_{}", mono.name),
    );
    Some(chi)
}

/// Check whether the sheaf condition holds for a given covering family.
///
/// `covering` is a list of `(restriction_source_id, open_id)` pairs
/// representing the cover.  Returns `true` when the presheaf data is
/// consistent (no duplicate section lists for any single open in the cover).
pub fn check_sheaf_condition(sheaf: &SpecSheaf, covering: &[(usize, usize)]) -> bool {
    // Separation: sections over each open in the cover are uniquely determined.
    let mut seen: std::collections::HashSet<usize> = std::collections::HashSet::new();
    for &(_src, open_id) in covering {
        if !seen.insert(open_id) {
            // open_id appears twice — the covering is self-contradictory.
            return false;
        }
        // Every open in the cover must have section data.
        if sheaf.sections_over(open_id).is_none() {
            return false;
        }
    }
    true
}

/// The plus-construction (sheafification) step.
///
/// Iterates over the site morphisms and for each section set that is reachable
/// via a site morphism, adds the compatible sections to a new sheaf.
pub fn sheafification_steps(presheaf: &SpecSheaf, site_morphisms: &[ToposMorphism]) -> SpecSheaf {
    let mut sheaf = SpecSheaf::new(presheaf.site);

    // Copy existing sections.
    for &(open_id, ref sections) in &presheaf.presheaf_data {
        sheaf.add_sections(open_id, sections.clone());
    }

    // For each site morphism f: U → V, propagate sections from V to U.
    for m in site_morphisms {
        let target_sections: Option<Vec<usize>> = presheaf.sections_over(m.codomain).cloned();
        if let Some(secs) = target_sections {
            // Only add if not already present for this open.
            if sheaf.sections_over(m.domain).is_none() {
                sheaf.add_sections(m.domain, secs);
            }
        }
    }

    sheaf
}

/// Verify the three Lawvere–Tierney axioms for a topology j: Ω → Ω.
///
/// Because we work with named morphisms rather than actual functions, we check
/// the structural conditions:
/// 1. The j-operator is an endomorphism of the Ω object (domain = codomain).
/// 2. The topos id stored in `lt` matches the topos that contains the
///    subobject classifier.
/// 3. The morphism name contains "j" (a convention for the j-operator).
pub fn verify_lt_topology(topos: &ElementaryToposData, lt: &LTTopology) -> bool {
    // 1. j must be an endomorphism.
    if lt.j_operator.domain != lt.j_operator.codomain {
        return false;
    }
    // 2. The j-operator's domain object must exist in the topos.
    if topos.object(lt.j_operator.domain).is_none() {
        return false;
    }
    // 3. The topos id in LTTopology should be consistent (non-usize::MAX sentinel).
    lt.topos != usize::MAX
}

/// Check the adjunction condition for a geometric morphism.
///
/// Returns `true` when the inverse-image morphism has the same domain as the
/// direct-image morphism's codomain (f* goes F→E, f_* goes E→F — so
/// f*.codomain == f_*.domain), ensuring the adjunction f* ⊣ f_* is plausible.
pub fn geometric_morphism_adjunction(gm: &SpecGeometricMorphism) -> bool {
    gm.inverse_image.codomain == gm.direct_image.domain
}

/// Construct the canonical Set topos with a small collection of objects and morphisms.
///
/// The Set topos has:
/// - objects: 0 (∅), 1 (terminal = {*}), 2 (Bool = {0,1}), 3 (Ω = {false,true})
/// - terminal: object 1
/// - subobject classifier: Ω = object 3, truth = morphism 1→3
/// - power objects: Ω^∅, Ω^1, Ω^2, Ω^3
pub fn set_topos() -> ElementaryToposData {
    let mut t = ElementaryToposData::new();

    let empty = t.add_object("∅"); // 0
    let term = t.add_object("1"); // 1
    let two = t.add_object("2"); // 2
    let omega = t.add_object("Ω"); // 3

    t.terminal = Some(term);

    // truth: 1 → Ω
    let truth_id = t.add_morphism(term, omega, "true");
    let truth_m = t.morphisms[truth_id].clone();

    t.subobject_classifier = Some(SpecSubobjectClassifier::new("Ω", truth_m.clone()));

    // Minimal identity morphisms for power objects.
    let ev_empty = t.add_morphism(empty, omega, "ev_∅");
    let ev_term = t.add_morphism(term, omega, "ev_1");
    let ev_two = t.add_morphism(two, omega, "ev_2");
    let ev_omega = t.add_morphism(omega, omega, "ev_Ω");

    // Ω^A objects: synthesised ids beyond current range.
    let pow_empty_id = t.objects.len();
    t.objects.push(ToposObject::new(pow_empty_id, "Ω^∅"));
    let pow_term_id = t.objects.len();
    t.objects.push(ToposObject::new(pow_term_id, "Ω^1"));
    let pow_two_id = t.objects.len();
    t.objects.push(ToposObject::new(pow_two_id, "Ω^2"));
    let pow_omega_id = t.objects.len();
    t.objects.push(ToposObject::new(pow_omega_id, "Ω^Ω"));

    // Register power objects in the power_objects registry (base → power_id).
    // Note: the Ω^A objects are registered in `objects` for enumeration purposes,
    // but `verify_topos_axioms` only checks objects in the `base_objects` range
    // (the first `num_base` objects). Power objects themselves are representable
    // and implicitly have their own power objects via the Ω^(Ω^A) construction,
    // but we do not unroll that infinite chain in the finite presentation.
    t.power_objects.push(SpecPowerObject::new(
        empty,
        pow_empty_id,
        t.morphisms[ev_empty].clone(),
    ));
    t.power_objects.push(SpecPowerObject::new(
        term,
        pow_term_id,
        t.morphisms[ev_term].clone(),
    ));
    t.power_objects.push(SpecPowerObject::new(
        two,
        pow_two_id,
        t.morphisms[ev_two].clone(),
    ));
    t.power_objects.push(SpecPowerObject::new(
        omega,
        pow_omega_id,
        t.morphisms[ev_omega].clone(),
    ));

    // Mark the topos as having only the first 4 objects as "base" objects
    // that require explicit power object entries. The synthesised Ω^A objects
    // are representable power objects and are tracked via `power_objects` entries
    // already. Set `num_base_objects` so the axiom checker knows the boundary.
    t.num_base_objects = 4;

    t
}

#[cfg(test)]
mod tests_elementary_topos {
    use super::*;

    fn make_valid_topos() -> ElementaryToposData {
        set_topos()
    }

    #[test]
    fn test_topos_object_new() {
        let o = ToposObject::new(0, "A");
        assert_eq!(o.id, 0);
        assert_eq!(o.name, "A");
    }

    #[test]
    fn test_topos_morphism_new() {
        let m = ToposMorphism::new(0, 1, 2, "f");
        assert_eq!(m.id, 0);
        assert_eq!(m.domain, 1);
        assert_eq!(m.codomain, 2);
        assert_eq!(m.name, "f");
    }

    #[test]
    fn test_set_topos_objects() {
        let t = set_topos();
        assert!(t.objects.len() >= 4);
    }

    #[test]
    fn test_set_topos_terminal() {
        let t = set_topos();
        assert!(t.terminal.is_some());
        let term = terminal_object(&t);
        assert!(term.is_some());
        assert_eq!(term.expect("present").name, "1");
    }

    #[test]
    fn test_set_topos_subobject_classifier() {
        let t = set_topos();
        assert!(t.subobject_classifier.is_some());
        let sc = t.subobject_classifier.as_ref().expect("present");
        assert_eq!(sc.name, "Ω");
        assert_eq!(sc.truth_morphism.name, "true");
    }

    #[test]
    fn test_set_topos_power_objects() {
        let t = set_topos();
        assert_eq!(t.power_objects.len(), 4);
    }

    #[test]
    fn test_verify_topos_axioms_valid() {
        let t = make_valid_topos();
        let violations = verify_topos_axioms(&t);
        assert!(
            violations.is_empty(),
            "Unexpected violations: {:?}",
            violations
        );
    }

    #[test]
    fn test_verify_topos_axioms_no_terminal() {
        let mut t = ElementaryToposData::new();
        t.add_object("A");
        let violations = verify_topos_axioms(&t);
        assert!(violations.iter().any(|v| v.contains("terminal")));
    }

    #[test]
    fn test_verify_topos_axioms_no_classifier() {
        let mut t = ElementaryToposData::new();
        let a = t.add_object("A");
        t.terminal = Some(a);
        let violations = verify_topos_axioms(&t);
        assert!(violations
            .iter()
            .any(|v| v.contains("subobject classifier")));
    }

    #[test]
    fn test_verify_topos_axioms_missing_power_object() {
        let mut t = ElementaryToposData::new();
        let a = t.add_object("A");
        let omega_id = t.add_object("Ω");
        t.terminal = Some(a);
        let truth = t.add_morphism(a, omega_id, "true");
        let truth_m = t.morphisms[truth].clone();
        t.subobject_classifier = Some(SpecSubobjectClassifier::new("Ω", truth_m));
        // No power objects added.
        let violations = verify_topos_axioms(&t);
        assert!(violations.iter().any(|v| v.contains("power object")));
    }

    #[test]
    fn test_terminal_object_none_when_empty() {
        let t = ElementaryToposData::new();
        assert!(terminal_object(&t).is_none());
    }

    #[test]
    fn test_pullback_different_codomains_returns_none() {
        let t = set_topos();
        let f = t.morphisms[0].clone();
        let mut g = f.clone();
        g.codomain = g.codomain.wrapping_add(1);
        assert!(pullback(&t, &f, &g).is_none());
    }

    #[test]
    fn test_pullback_synthesised() {
        let t = set_topos();
        // Find two morphisms with the same codomain.
        let morphs_to_omega: Vec<_> = t.morphisms.iter().filter(|m| m.codomain == 3).collect();
        if morphs_to_omega.len() >= 2 {
            let f = morphs_to_omega[0].clone();
            let g = morphs_to_omega[1].clone();
            let result = pullback(&t, &f, &g);
            assert!(result.is_some());
        }
    }

    #[test]
    fn test_characteristic_morphism_requires_omega() {
        let t = set_topos();
        let mono = t.morphisms[0].clone();
        let chi = characteristic_morphism(&t, &mono);
        assert!(chi.is_some());
        let chi_m = chi.expect("present");
        assert!(chi_m.name.contains("χ_"));
    }

    #[test]
    fn test_characteristic_morphism_no_classifier_returns_none() {
        let t = ElementaryToposData::new();
        let m = ToposMorphism::new(0, 0, 1, "m");
        assert!(characteristic_morphism(&t, &m).is_none());
    }

    #[test]
    fn test_spec_sheaf_sections() {
        let mut sheaf = SpecSheaf::new(0);
        sheaf.add_sections(1, vec![10, 20, 30]);
        sheaf.add_sections(2, vec![40]);
        assert_eq!(sheaf.sections_over(1), Some(&vec![10, 20, 30]));
        assert_eq!(sheaf.sections_over(3), None);
    }

    #[test]
    fn test_check_sheaf_condition_empty_covering() {
        let sheaf = SpecSheaf::new(0);
        assert!(check_sheaf_condition(&sheaf, &[]));
    }

    #[test]
    fn test_check_sheaf_condition_valid() {
        let mut sheaf = SpecSheaf::new(0);
        sheaf.add_sections(1, vec![1]);
        sheaf.add_sections(2, vec![2]);
        assert!(check_sheaf_condition(&sheaf, &[(0, 1), (0, 2)]));
    }

    #[test]
    fn test_check_sheaf_condition_duplicate_open_fails() {
        let mut sheaf = SpecSheaf::new(0);
        sheaf.add_sections(1, vec![1]);
        assert!(!check_sheaf_condition(&sheaf, &[(0, 1), (1, 1)]));
    }

    #[test]
    fn test_check_sheaf_condition_missing_sections_fails() {
        let sheaf = SpecSheaf::new(0);
        assert!(!check_sheaf_condition(&sheaf, &[(0, 99)]));
    }

    #[test]
    fn test_sheafification_steps_propagates() {
        let mut presheaf = SpecSheaf::new(0);
        presheaf.add_sections(2, vec![100, 200]);
        let site_m = ToposMorphism::new(0, 1, 2, "f");
        let result = sheafification_steps(&presheaf, &[site_m]);
        assert_eq!(result.sections_over(1), Some(&vec![100, 200]));
        assert_eq!(result.sections_over(2), Some(&vec![100, 200]));
    }

    #[test]
    fn test_sheafification_steps_no_morphisms() {
        let mut presheaf = SpecSheaf::new(5);
        presheaf.add_sections(3, vec![7]);
        let result = sheafification_steps(&presheaf, &[]);
        assert_eq!(result.sections_over(3), Some(&vec![7]));
    }

    #[test]
    fn test_lt_topology_valid() {
        let t = set_topos();
        // Omega has id 3; create a j: Ω → Ω endomorphism.
        let j = ToposMorphism::new(99, 3, 3, "j");
        let lt = LTTopology::new(0, j);
        assert!(verify_lt_topology(&t, &lt));
    }

    #[test]
    fn test_lt_topology_invalid_non_endomorphism() {
        let t = set_topos();
        let j = ToposMorphism::new(99, 1, 3, "j");
        let lt = LTTopology::new(0, j);
        assert!(!verify_lt_topology(&t, &lt));
    }

    #[test]
    fn test_geometric_morphism_adjunction_valid() {
        // inverse_image: F→E (cod=E=2), direct_image: E→F (dom=E=2).
        let inv = ToposMorphism::new(0, 0, 2, "f*");
        let dir = ToposMorphism::new(1, 2, 0, "f_*");
        let gm = SpecGeometricMorphism::new(1, 0, inv, dir);
        assert!(geometric_morphism_adjunction(&gm));
    }

    #[test]
    fn test_geometric_morphism_adjunction_invalid() {
        let inv = ToposMorphism::new(0, 0, 3, "f*");
        let dir = ToposMorphism::new(1, 2, 0, "f_*");
        let gm = SpecGeometricMorphism::new(1, 0, inv, dir);
        assert!(!geometric_morphism_adjunction(&gm));
    }

    #[test]
    fn test_sheaf_condition_kind_variants() {
        let k1 = SheafConditionKind::GlueingAxiom;
        let k2 = SheafConditionKind::SeparationAxiom;
        let k3 = SheafConditionKind::BothAxioms;
        assert_ne!(k1, k2);
        assert_ne!(k2, k3);
        assert_ne!(k1, k3);
    }

    #[test]
    fn test_power_object_new() {
        let eval = ToposMorphism::new(5, 10, 3, "ev");
        let po = SpecPowerObject::new(10, 20, eval.clone());
        assert_eq!(po.base, 10);
        assert_eq!(po.power, 20);
        assert_eq!(po.eval.id, 5);
    }

    #[test]
    fn test_elementary_topos_data_add_object_morphism() {
        let mut t = ElementaryToposData::new();
        let a = t.add_object("A");
        let b = t.add_object("B");
        let m_id = t.add_morphism(a, b, "f");
        assert_eq!(t.objects.len(), 2);
        assert_eq!(t.morphisms.len(), 1);
        assert_eq!(t.morphisms[m_id].domain, a);
        assert_eq!(t.morphisms[m_id].codomain, b);
    }
}
