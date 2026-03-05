//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    ClassifyingToposExt, CondensedSet, EffectiveTopos, EtaleTopos, FiniteCategory, GeometricLogic,
    GeometricMorphismExt, GeometricMorphismExtData, HeytingSubobjectLattice, InternalLogic,
    KripkeJoyalSemantics, LawvereTierneyTop, Locale, LocalicReflection, LogicalFunctor,
    PresheafTopos, PyknoticObject, SheafConditionExt, SheafConditionExtChecker, SiteCategory,
    Subobject, SubobjectClassifier, SubobjectClassifierFinSet, Topos, ToposCohomology, ToposMap,
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
