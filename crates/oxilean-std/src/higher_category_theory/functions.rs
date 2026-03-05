//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    AccessibleCategory, AssociativeOperad, CartesianFibration, CoCartesianFibration,
    CommutativeOperad, CompactlyGenerated, ComputadData, EnAlgebra, EnrichedCategory,
    EtaleMorphism, ExcisiveFunctor, FactorizationAlgebra, GlobularSetData,
    GrothendieckConstruction, HomogeneousFunctor, HypercoverDescent, InfinityNCatData,
    InfinityNCategory, InftyColimit, InftyFunctor, InftyLimit, InftyNaturalTransformation,
    InftyOperad, InftyTopos, InnerHorn, KanExtension, LocalizationFunctor, MonoidalFunctor,
    ObjectClassifier, OmegaCategory, OperadNew, OperadV2, PresentableInftyCategory, QuasiCategory,
    QuasiCategoryNew, SegalSpaceData, StraighteningEquivalence, TaylorTower, TwoCategoryData,
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
pub fn type1() -> Expr {
    Expr::Sort(Level::succ(Level::succ(Level::zero())))
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
/// `QuasiCategory : Type`
///
/// A quasi-category is a simplicial set satisfying the inner horn filling
/// condition: every inner horn Λ^n_k (0 < k < n) admits a filler.
pub fn quasi_category_ty() -> Expr {
    type0()
}
/// `InnerHorn : Nat → Nat → QuasiCategory → Type`
///
/// The inner horn Λ^n_k for 0 < k < n inside a simplicial set X.
/// An element is a partial n-simplex missing the k-th face.
pub fn inner_horn_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(nat_ty(), arrow(cst("QuasiCategory"), type0())),
    )
}
/// `InnerHornFiller : ∀ (n k : Nat) (X : QuasiCategory), InnerHorn n k X → Type`
///
/// A filler for an inner horn: an n-simplex extending the partial data.
pub fn inner_horn_filler_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(
            nat_ty(),
            arrow(
                cst("QuasiCategory"),
                arrow(app3(cst("InnerHorn"), bvar(2), bvar(1), bvar(0)), type0()),
            ),
        ),
    )
}
/// `InftyFunctor : QuasiCategory → QuasiCategory → Type`
///
/// A morphism of simplicial sets (simplicial map) between quasi-categories.
pub fn infty_functor_ty() -> Expr {
    arrow(cst("QuasiCategory"), arrow(cst("QuasiCategory"), type0()))
}
/// `InftyNaturalTransformation : ∀ (C D : QuasiCategory), InftyFunctor C D → InftyFunctor C D → Type`
///
/// A homotopy between two ∞-functors: an element of the functor ∞-category Fun(C, D).
pub fn infty_natural_transformation_ty() -> Expr {
    impl_pi(
        "C",
        cst("QuasiCategory"),
        impl_pi(
            "D",
            cst("QuasiCategory"),
            arrow(
                app2(cst("InftyFunctor"), bvar(1), bvar(0)),
                arrow(app2(cst("InftyFunctor"), bvar(2), bvar(1)), type0()),
            ),
        ),
    )
}
/// `HasInnerHornFillings : QuasiCategory → Prop`
///
/// The inner horn filling condition: every inner horn Λ^n_k admits a filler.
pub fn has_inner_horn_fillings_ty() -> Expr {
    arrow(cst("QuasiCategory"), prop())
}
/// `IsKanComplex : QuasiCategory → Prop`
///
/// A Kan complex: ALL horns (including outer) admit fillers — the ∞-groupoid condition.
pub fn is_kan_complex_ty() -> Expr {
    arrow(cst("QuasiCategory"), prop())
}
/// `IsEquivalence : ∀ (C D : QuasiCategory), InftyFunctor C D → Prop`
///
/// An ∞-functor is an equivalence of quasi-categories.
pub fn is_equivalence_ty() -> Expr {
    impl_pi(
        "C",
        cst("QuasiCategory"),
        impl_pi(
            "D",
            cst("QuasiCategory"),
            arrow(app2(cst("InftyFunctor"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `InftyLimit : ∀ (C D : QuasiCategory), InftyFunctor C D → Type`
///
/// The ∞-categorical limit of a diagram F: C → D.
/// This is the terminal object in the slice ∞-category D_{/F}.
pub fn infty_limit_ty() -> Expr {
    impl_pi(
        "C",
        cst("QuasiCategory"),
        impl_pi(
            "D",
            cst("QuasiCategory"),
            arrow(app2(cst("InftyFunctor"), bvar(1), bvar(0)), type0()),
        ),
    )
}
/// `InftyColimit : ∀ (C D : QuasiCategory), InftyFunctor C D → Type`
///
/// The ∞-categorical colimit of a diagram F: C → D.
/// This is the initial object in the co-slice ∞-category D_{F/}.
pub fn infty_colimit_ty() -> Expr {
    impl_pi(
        "C",
        cst("QuasiCategory"),
        impl_pi(
            "D",
            cst("QuasiCategory"),
            arrow(app2(cst("InftyFunctor"), bvar(1), bvar(0)), type0()),
        ),
    )
}
/// `InftyAdjunction : ∀ (C D : QuasiCategory), InftyFunctor C D → InftyFunctor D C → Prop`
///
/// A homotopy coherent adjunction (F ⊣ G) between quasi-categories.
pub fn infty_adjunction_ty() -> Expr {
    impl_pi(
        "C",
        cst("QuasiCategory"),
        impl_pi(
            "D",
            cst("QuasiCategory"),
            arrow(
                app2(cst("InftyFunctor"), bvar(1), bvar(0)),
                arrow(app2(cst("InftyFunctor"), bvar(1), bvar(2)), prop()),
            ),
        ),
    )
}
/// `KanExtension : ∀ (C D E : QuasiCategory), InftyFunctor C D → InftyFunctor C E → Type`
///
/// Left/right Kan extension in ∞-Cat: Lan_F G for F: C → D and G: C → E.
pub fn kan_extension_ty() -> Expr {
    impl_pi(
        "C",
        cst("QuasiCategory"),
        impl_pi(
            "D",
            cst("QuasiCategory"),
            impl_pi(
                "E",
                cst("QuasiCategory"),
                arrow(
                    app2(cst("InftyFunctor"), bvar(2), bvar(1)),
                    arrow(app2(cst("InftyFunctor"), bvar(3), bvar(1)), type0()),
                ),
            ),
        ),
    )
}
/// `PresentableInftyCategory : Type`
///
/// A presentable ∞-category: accessible and cocomplete (has all small colimits).
pub fn presentable_infty_category_ty() -> Expr {
    type0()
}
/// `AccessibleCategory : Nat → QuasiCategory → Prop`
///
/// A κ-accessible category: closed under κ-filtered colimits, generated by κ-compact objects.
pub fn accessible_category_ty() -> Expr {
    arrow(nat_ty(), arrow(cst("QuasiCategory"), prop()))
}
/// `LocalizationFunctor : ∀ (C : QuasiCategory), Type → InftyFunctor C C`
///
/// The localization L_S: C → C[S⁻¹] inverting a set S of morphisms.
pub fn localization_functor_ty() -> Expr {
    impl_pi(
        "C",
        cst("QuasiCategory"),
        arrow(type0(), app2(cst("InftyFunctor"), bvar(0), bvar(0))),
    )
}
/// `CompactlyGenerated : QuasiCategory → Prop`
///
/// A compactly generated ∞-category: generated by its compact objects under colimits.
pub fn compactly_generated_ty() -> Expr {
    arrow(cst("QuasiCategory"), prop())
}
/// `InftyTopos : Type`
///
/// An ∞-topos: a presentable ∞-category with universal colimits and an object classifier.
pub fn infty_topos_ty() -> Expr {
    type0()
}
/// `ObjectClassifier : InftyTopos → Type`
///
/// The object classifier U in an ∞-topos: a univalent universe with Univ(X) ≃ Map(X, U).
pub fn object_classifier_ty() -> Expr {
    arrow(cst("InftyTopos"), type0())
}
/// `EtaleMorphism : ∀ (T : InftyTopos), T → T → Prop`
///
/// A formally étale morphism in an ∞-topos: the square X → T, X → Y is a pullback.
pub fn etale_morphism_ty() -> Expr {
    impl_pi(
        "T",
        cst("InftyTopos"),
        arrow(type0(), arrow(type0(), prop())),
    )
}
/// `HypercoverDescent : InftyTopos → Prop`
///
/// The sheaf condition for ∞-toposes: descent along hypercoverings.
pub fn hypercover_descent_ty() -> Expr {
    arrow(cst("InftyTopos"), prop())
}
/// `CartesianFibration : ∀ (S T : QuasiCategory), InftyFunctor S T → Prop`
///
/// A cartesian fibration p: S → T: a map with enough cartesian edges.
pub fn cartesian_fibration_ty() -> Expr {
    impl_pi(
        "S",
        cst("QuasiCategory"),
        impl_pi(
            "T",
            cst("QuasiCategory"),
            arrow(app2(cst("InftyFunctor"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `CoCartesianFibration : ∀ (S T : QuasiCategory), InftyFunctor S T → Prop`
///
/// A coCartesian fibration p: S → T: the dual notion with coCartesian edges.
pub fn cocartesian_fibration_ty() -> Expr {
    impl_pi(
        "S",
        cst("QuasiCategory"),
        impl_pi(
            "T",
            cst("QuasiCategory"),
            arrow(app2(cst("InftyFunctor"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `StraighteningEquivalence : ∀ (S : QuasiCategory), Prop`
///
/// The straightening/unstraightening equivalence: CartFib/S ≃ Fun(S^op, ∞-Cat).
pub fn straightening_equivalence_ty() -> Expr {
    impl_pi("S", cst("QuasiCategory"), prop())
}
/// `GrothendieckConstruction : ∀ (S : QuasiCategory), InftyFunctor S (cst("InftyCat")) → QuasiCategory`
///
/// The ∞-categorical Grothendieck construction: produces a coCartesian fibration from a functor.
pub fn grothendieck_construction_ty() -> Expr {
    impl_pi(
        "S",
        cst("QuasiCategory"),
        arrow(
            app2(cst("InftyFunctor"), bvar(0), cst("InftyCat")),
            cst("QuasiCategory"),
        ),
    )
}
/// `InftyOperad : Type`
///
/// A multi-colored ∞-operad: a coCartesian fibration over Fin_* (pointed finite sets).
pub fn infty_operad_ty() -> Expr {
    type0()
}
/// `AssociativeOperad : InftyOperad`
///
/// The associative operad Assoc: the ∞-operad underlying A_∞-algebra theory.
pub fn associative_operad_ty() -> Expr {
    cst("InftyOperad")
}
/// `CommutativeOperad : InftyOperad`
///
/// The commutative operad Comm: the ∞-operad underlying E_∞-algebra theory.
pub fn commutative_operad_ty() -> Expr {
    cst("InftyOperad")
}
/// `EnAlgebra : Nat → InftyOperad → Type → Type`
///
/// An algebra over the E_n-operad (little n-disks operad) with n levels of commutativity.
pub fn en_algebra_ty() -> Expr {
    arrow(nat_ty(), arrow(cst("InftyOperad"), arrow(type0(), type0())))
}
/// `ExcisiveFunctor : Nat → (QuasiCategory → QuasiCategory) → Prop`
///
/// The n-excisive approximation P_n F: the best n-excisive approximation to F.
pub fn excisive_functor_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(arrow(cst("QuasiCategory"), cst("QuasiCategory")), prop()),
    )
}
/// `TaylorTower : (QuasiCategory → QuasiCategory) → Nat → QuasiCategory → QuasiCategory`
///
/// The Taylor tower of F: the sequence P_0 F → P_1 F → P_2 F → ... of excisive approximations.
pub fn taylor_tower_ty() -> Expr {
    arrow(
        arrow(cst("QuasiCategory"), cst("QuasiCategory")),
        arrow(nat_ty(), arrow(cst("QuasiCategory"), cst("QuasiCategory"))),
    )
}
/// `HomogeneousFunctor : Nat → (QuasiCategory → QuasiCategory) → Prop`
///
/// The n-th Goodwillie derivative D_n F: the n-th layer of the Taylor tower.
pub fn homogeneous_functor_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(arrow(cst("QuasiCategory"), cst("QuasiCategory")), prop()),
    )
}
/// `SpanierWhiteheadDuality : QuasiCategory → Prop`
///
/// Spanier-Whitehead duality in a stable ∞-category: the S-dual functor D.
pub fn spanier_whitehead_ty() -> Expr {
    arrow(cst("QuasiCategory"), prop())
}
/// Register all higher category theory axioms into the given kernel environment.
pub fn build_env(env: &mut Environment) -> Result<(), String> {
    add_axiom(env, "QuasiCategory", vec![], quasi_category_ty())?;
    add_axiom(env, "InnerHorn", vec![], inner_horn_ty())?;
    add_axiom(env, "InnerHornFiller", vec![], inner_horn_filler_ty())?;
    add_axiom(env, "InftyFunctor", vec![], infty_functor_ty())?;
    add_axiom(
        env,
        "InftyNaturalTransformation",
        vec![],
        infty_natural_transformation_ty(),
    )?;
    add_axiom(
        env,
        "HasInnerHornFillings",
        vec![],
        has_inner_horn_fillings_ty(),
    )?;
    add_axiom(env, "IsKanComplex", vec![], is_kan_complex_ty())?;
    add_axiom(env, "IsEquivalence", vec![], is_equivalence_ty())?;
    add_axiom(env, "InftyLimit", vec![], infty_limit_ty())?;
    add_axiom(env, "InftyColimit", vec![], infty_colimit_ty())?;
    add_axiom(env, "InftyAdjunction", vec![], infty_adjunction_ty())?;
    add_axiom(env, "KanExtension", vec![], kan_extension_ty())?;
    add_axiom(
        env,
        "PresentableInftyCategory",
        vec![],
        presentable_infty_category_ty(),
    )?;
    add_axiom(env, "AccessibleCategory", vec![], accessible_category_ty())?;
    add_axiom(
        env,
        "LocalizationFunctor",
        vec![],
        localization_functor_ty(),
    )?;
    add_axiom(env, "CompactlyGenerated", vec![], compactly_generated_ty())?;
    add_axiom(env, "InftyTopos", vec![], infty_topos_ty())?;
    add_axiom(env, "ObjectClassifier", vec![], object_classifier_ty())?;
    add_axiom(env, "EtaleMorphism", vec![], etale_morphism_ty())?;
    add_axiom(env, "HypercoverDescent", vec![], hypercover_descent_ty())?;
    add_axiom(env, "CartesianFibration", vec![], cartesian_fibration_ty())?;
    add_axiom(
        env,
        "CoCartesianFibration",
        vec![],
        cocartesian_fibration_ty(),
    )?;
    add_axiom(
        env,
        "StraighteningEquivalence",
        vec![],
        straightening_equivalence_ty(),
    )?;
    add_axiom(env, "InftyCat", vec![], type1())?;
    add_axiom(
        env,
        "GrothendieckConstruction",
        vec![],
        grothendieck_construction_ty(),
    )?;
    add_axiom(env, "InftyOperad", vec![], infty_operad_ty())?;
    add_axiom(env, "AssociativeOperad", vec![], associative_operad_ty())?;
    add_axiom(env, "CommutativeOperad", vec![], commutative_operad_ty())?;
    add_axiom(env, "EnAlgebra", vec![], en_algebra_ty())?;
    add_axiom(env, "ExcisiveFunctor", vec![], excisive_functor_ty())?;
    add_axiom(env, "TaylorTower", vec![], taylor_tower_ty())?;
    add_axiom(env, "HomogeneousFunctor", vec![], homogeneous_functor_ty())?;
    add_axiom(
        env,
        "SpanierWhiteheadDuality",
        vec![],
        spanier_whitehead_ty(),
    )?;
    Ok(())
}
/// Register all higher category theory axioms (base + extended) into the environment.
pub fn build_env_all(env: &mut Environment) -> Result<(), String> {
    build_env(env)?;
    build_env_extended(env)?;
    Ok(())
}
/// `InfinityNCat : Nat → Type`
///
/// An (∞,n)-category: an ∞-category where all k-morphisms for k > n are
/// invertible. Parametrized by the truncation level n.
pub fn infinity_n_cat_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `SegalSpace : Type`
///
/// A Segal space: a simplicial space X satisfying the Segal condition
/// X_n ≃ X_1 ×_{X_0} ⋯ ×_{X_0} X_1 (n-fold iterated fiber product).
pub fn segal_space_ty() -> Expr {
    type0()
}
/// `SegalCondition : SegalSpace → Prop`
///
/// The Segal condition: the spine inclusions Sp[n] ↪ Δ[n] induce
/// equivalences X(Δ[n]) ≃ X(Sp[n]).
pub fn segal_condition_ty() -> Expr {
    arrow(cst("SegalSpace"), prop())
}
/// `CompleteSegalSpace : Type`
///
/// A complete Segal space (CSS): a Segal space where the space of objects
/// X_0 is equivalent to the sub-space of equivalences in X_1.
pub fn complete_segal_space_ty() -> Expr {
    type0()
}
/// `CompletenessCondition : SegalSpace → Prop`
///
/// The completeness condition: the degeneracy map X_0 → X_1^{equiv} is
/// an equivalence of spaces.
pub fn completeness_condition_ty() -> Expr {
    arrow(cst("SegalSpace"), prop())
}
/// `RezkCompletion : SegalSpace → CompleteSegalSpace`
///
/// The Rezk completion: the universal CSS approximation of a Segal space.
pub fn rezk_completion_ty() -> Expr {
    arrow(cst("SegalSpace"), cst("CompleteSegalSpace"))
}
/// `GlobularSet : Type`
///
/// A globular set: a presheaf on the globe category G, providing source
/// and target maps s_n, t_n : G_n → G_{n-1} with globularity axioms.
pub fn globular_set_ty() -> Expr {
    type0()
}
/// `GlobularCell : GlobularSet → Nat → Type`
///
/// The set of n-cells in a globular set.
pub fn globular_cell_ty() -> Expr {
    arrow(cst("GlobularSet"), arrow(nat_ty(), type0()))
}
/// `ThetaNCategory : Nat → Type`
///
/// A Θ_n-category (Berger): a presheaf on the Θ_n category satisfying
/// Segal-type horn filling conditions indexed by Θ_n.
pub fn theta_n_category_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `ThetaNFunctor : ∀ (n : Nat), ThetaNCategory n → ThetaNCategory n → Type`
///
/// A morphism of Θ_n-categories: a map of presheaves.
pub fn theta_n_functor_ty() -> Expr {
    impl_pi(
        "n",
        nat_ty(),
        arrow(
            app(cst("ThetaNCategory"), bvar(0)),
            arrow(app(cst("ThetaNCategory"), bvar(1)), type0()),
        ),
    )
}
/// `GlobularComposition : GlobularSet → Prop`
///
/// Composition in a globular higher category: associative composition at
/// each level satisfying the interchange law.
pub fn globular_composition_ty() -> Expr {
    arrow(cst("GlobularSet"), prop())
}
/// `TwoCategory : Type`
///
/// A strict 2-category: objects, 1-morphisms, 2-morphisms with strict
/// functoriality and interchange law.
pub fn two_category_ty() -> Expr {
    type0()
}
/// `TwoCategoryFunctor : TwoCategory → TwoCategory → Type`
///
/// A strict 2-functor between 2-categories.
pub fn two_category_functor_ty() -> Expr {
    arrow(cst("TwoCategory"), arrow(cst("TwoCategory"), type0()))
}
/// `GrayTensorProduct : TwoCategory → TwoCategory → TwoCategory`
///
/// The Gray tensor product C ⊗_G D of two 2-categories: the lax tensor
/// product in 2-Cat where interchange holds only up to a 2-cell.
pub fn gray_tensor_product_ty() -> Expr {
    arrow(
        cst("TwoCategory"),
        arrow(cst("TwoCategory"), cst("TwoCategory")),
    )
}
/// `PseudoFunctor : TwoCategory → TwoCategory → Type`
///
/// A pseudofunctor (homomorphism of bicategories): a 2-functor where
/// functoriality holds only up to coherent invertible 2-cells.
pub fn pseudo_functor_ty() -> Expr {
    arrow(cst("TwoCategory"), arrow(cst("TwoCategory"), type0()))
}
/// `Tricategory : Type`
///
/// A tricategory (Gordon-Power-Street): a 3-dimensional analog of a
/// bicategory with objects, 1-, 2-, and 3-morphisms satisfying coherence
/// conditions up to invertible 3-cells.
pub fn tricategory_ty() -> Expr {
    type0()
}
/// `TricategoryCoherence : Tricategory → Prop`
///
/// The coherence theorem for tricategories: every tricategory is
/// triequivalent to a Gray-category (a 2-Cat-enriched category).
pub fn tricategory_coherence_ty() -> Expr {
    arrow(cst("Tricategory"), prop())
}
/// `Oriental : Nat → Type`
///
/// Street's n-th oriental O_n: the free strict ω-category on a single
/// n-dimensional cell, yielding the standard pasting scheme.
pub fn oriental_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `GlobularComplex : Type`
///
/// A globular complex (Batanin): a collection of globular pasting diagrams
/// forming the basis of weak ω-category theory.
pub fn globular_complex_ty() -> Expr {
    type0()
}
/// `Computad : Type`
///
/// A computad (Schanuel-Street): a presentation of a strict ω-category by
/// generators and relations at each dimension.
pub fn computad_ty() -> Expr {
    type0()
}
/// `Polygraph : Type`
///
/// A polygraph (Burroni): equivalent to a computad; a rewriting system
/// for higher-dimensional categories.
pub fn polygraph_ty() -> Expr {
    type0()
}
/// `FreeOmegaCategory : Computad → Type`
///
/// The free strict ω-category generated by a computad.
pub fn free_omega_category_ty() -> Expr {
    arrow(cst("Computad"), type0())
}
/// `PastingDiagram : Nat → GlobularSet → Type`
///
/// An n-dimensional pasting diagram in a globular set: a combinatorial
/// description of a composite n-cell.
pub fn pasting_diagram_ty() -> Expr {
    arrow(nat_ty(), arrow(cst("GlobularSet"), type0()))
}
/// `Bilimit : ∀ (C D : TwoCategory), TwoCategoryFunctor C D → Type`
///
/// A bilimit (biadjoint limit) in a 2-category: a limit that is defined
/// only up to equivalence, not strict isomorphism.
pub fn bilimit_ty() -> Expr {
    impl_pi(
        "C",
        cst("TwoCategory"),
        impl_pi(
            "D",
            cst("TwoCategory"),
            arrow(app2(cst("TwoCategoryFunctor"), bvar(1), bvar(0)), type0()),
        ),
    )
}
/// `Pseudolimit : ∀ (C D : TwoCategory), TwoCategoryFunctor C D → Type`
///
/// A pseudolimit: a limit in the 2-categorical sense, defined up to
/// pseudo-natural equivalence.
pub fn pseudolimit_ty() -> Expr {
    impl_pi(
        "C",
        cst("TwoCategory"),
        impl_pi(
            "D",
            cst("TwoCategory"),
            arrow(app2(cst("TwoCategoryFunctor"), bvar(1), bvar(0)), type0()),
        ),
    )
}
/// `Strict2Limit : ∀ (C D : TwoCategory), TwoCategoryFunctor C D → Type`
///
/// A strict 2-limit: a limit in the strict 2-categorical sense, where
/// the universal property holds on the nose.
pub fn strict_2_limit_ty() -> Expr {
    impl_pi(
        "C",
        cst("TwoCategory"),
        impl_pi(
            "D",
            cst("TwoCategory"),
            arrow(app2(cst("TwoCategoryFunctor"), bvar(1), bvar(0)), type0()),
        ),
    )
}
/// `Bicolimit : ∀ (C D : TwoCategory), TwoCategoryFunctor C D → Type`
///
/// A bicolimit: the bi-colimit dual of bilimit.
pub fn bicolimit_ty() -> Expr {
    impl_pi(
        "C",
        cst("TwoCategory"),
        impl_pi(
            "D",
            cst("TwoCategory"),
            arrow(app2(cst("TwoCategoryFunctor"), bvar(1), bvar(0)), type0()),
        ),
    )
}
/// `EnrichedCategory : TwoCategory → Type`
///
/// A category enriched over a monoidal category V: hom-sets are replaced
/// by objects of V.
pub fn enriched_category_ty() -> Expr {
    arrow(cst("TwoCategory"), type0())
}
/// `EnrichedFunctor : ∀ (V : TwoCategory), EnrichedCategory V → EnrichedCategory V → Type`
///
/// A V-enriched functor between V-enriched categories.
pub fn enriched_functor_ty() -> Expr {
    impl_pi(
        "V",
        cst("TwoCategory"),
        arrow(
            app(cst("EnrichedCategory"), bvar(0)),
            arrow(app(cst("EnrichedCategory"), bvar(1)), type0()),
        ),
    )
}
/// `LaxNaturalTransformation : ∀ (C D : TwoCategory), PseudoFunctor C D → PseudoFunctor C D → Type`
///
/// A lax natural transformation between pseudofunctors: components with
/// 2-cells that need not be invertible.
pub fn lax_natural_transformation_ty() -> Expr {
    impl_pi(
        "C",
        cst("TwoCategory"),
        impl_pi(
            "D",
            cst("TwoCategory"),
            arrow(
                app2(cst("PseudoFunctor"), bvar(1), bvar(0)),
                arrow(app2(cst("PseudoFunctor"), bvar(2), bvar(1)), type0()),
            ),
        ),
    )
}
/// `Modification : ∀ (C D : TwoCategory) (F G : PseudoFunctor C D),
///    LaxNaturalTransformation F G → LaxNaturalTransformation F G → Type`
///
/// A modification between lax natural transformations: a 3-cell in the
/// tricategory of bicategories.
pub fn modification_ty() -> Expr {
    impl_pi(
        "C",
        cst("TwoCategory"),
        impl_pi(
            "D",
            cst("TwoCategory"),
            arrow(
                app2(cst("PseudoFunctor"), bvar(1), bvar(0)),
                arrow(
                    app2(cst("PseudoFunctor"), bvar(2), bvar(1)),
                    arrow(
                        app2(cst("LaxNaturalTransformation"), bvar(1), bvar(0)),
                        arrow(
                            app2(cst("LaxNaturalTransformation"), bvar(2), bvar(1)),
                            type0(),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `OplaxNaturalTransformation : ∀ (C D : TwoCategory), PseudoFunctor C D → PseudoFunctor C D → Type`
///
/// An oplax natural transformation: the dual of a lax transformation with
/// 2-cells reversed.
pub fn oplax_natural_transformation_ty() -> Expr {
    impl_pi(
        "C",
        cst("TwoCategory"),
        impl_pi(
            "D",
            cst("TwoCategory"),
            arrow(
                app2(cst("PseudoFunctor"), bvar(1), bvar(0)),
                arrow(app2(cst("PseudoFunctor"), bvar(2), bvar(1)), type0()),
            ),
        ),
    )
}
/// `StrictOmegaCategory : Type`
///
/// A strict ω-category: globular set with strictly associative and unital
/// composition at every dimension, satisfying interchange.
pub fn strict_omega_category_ty() -> Expr {
    type0()
}
/// `WeakOmegaCategory : Type`
///
/// A weak ω-category (Batanin-Leinster): globular set with composition
/// operations governed by a globular operad, coherent up to higher cells.
pub fn weak_omega_category_ty() -> Expr {
    type0()
}
/// `BataninOperad : Type`
///
/// A globular operad in Batanin's sense: the governing structure for weak
/// ω-categories via contractible globular collections.
pub fn batanin_operad_ty() -> Expr {
    type0()
}
/// `BataninAlgebra : BataninOperad → GlobularSet → Prop`
///
/// An algebra over a Batanin globular operad: a globular set with
/// composition operations satisfying the operad axioms.
pub fn batanin_algebra_ty() -> Expr {
    arrow(cst("BataninOperad"), arrow(cst("GlobularSet"), prop()))
}
/// `ContractibleGlobularCollection : GlobularSet → Prop`
///
/// Contractibility of a globular collection: the key condition in
/// Batanin's definition that ensures coherence of weak ω-categories.
pub fn contractible_globular_collection_ty() -> Expr {
    arrow(cst("GlobularSet"), prop())
}
/// `TamsamaniWeakNCat : Nat → Type`
///
/// A Tamsamani-Simpson weak n-category: a simplicial object in (n-1)-Cat
/// satisfying the Segal condition, defined inductively.
pub fn tamsamani_weak_n_cat_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `SimpsonConjecture : Prop`
///
/// Simpson's conjecture: every weak ∞-groupoid (in the sense of Tamsamani)
/// is equivalent to a strict ∞-groupoid.
pub fn simpson_conjecture_ty() -> Expr {
    prop()
}
/// `NerveFunctor : StrictOmegaCategory → SegalSpace`
///
/// The Street-Roberts nerve: embeds strict ω-categories into Segal spaces
/// (or quasi-categories).
pub fn nerve_functor_ty() -> Expr {
    arrow(cst("StrictOmegaCategory"), cst("SegalSpace"))
}
/// `OmegaCategoryEquivalence : StrictOmegaCategory → StrictOmegaCategory → Prop`
///
/// An equivalence of strict ω-categories at the ω-categorical level.
pub fn omega_category_equivalence_ty() -> Expr {
    arrow(
        cst("StrictOmegaCategory"),
        arrow(cst("StrictOmegaCategory"), prop()),
    )
}
/// `FormalCategoryTheory : TwoCategory → Prop`
///
/// Formal category theory in a 2-category K (Street): the axioms for
/// Yoneda lemma, adjunctions, monads, and limits internal to K.
pub fn formal_category_theory_ty() -> Expr {
    arrow(cst("TwoCategory"), prop())
}
/// Register all §8–§14 higher category theory axioms.
pub fn build_env_extended(env: &mut Environment) -> Result<(), String> {
    add_axiom(env, "InfinityNCat", vec![], infinity_n_cat_ty())?;
    add_axiom(env, "SegalSpace", vec![], segal_space_ty())?;
    add_axiom(env, "SegalCondition", vec![], segal_condition_ty())?;
    add_axiom(env, "CompleteSegalSpace", vec![], complete_segal_space_ty())?;
    add_axiom(
        env,
        "CompletenessCondition",
        vec![],
        completeness_condition_ty(),
    )?;
    add_axiom(env, "RezkCompletion", vec![], rezk_completion_ty())?;
    add_axiom(env, "GlobularSet", vec![], globular_set_ty())?;
    add_axiom(env, "GlobularCell", vec![], globular_cell_ty())?;
    add_axiom(env, "ThetaNCategory", vec![], theta_n_category_ty())?;
    add_axiom(env, "ThetaNFunctor", vec![], theta_n_functor_ty())?;
    add_axiom(
        env,
        "GlobularComposition",
        vec![],
        globular_composition_ty(),
    )?;
    add_axiom(env, "TwoCategory", vec![], two_category_ty())?;
    add_axiom(env, "TwoCategoryFunctor", vec![], two_category_functor_ty())?;
    add_axiom(env, "GrayTensorProduct", vec![], gray_tensor_product_ty())?;
    add_axiom(env, "PseudoFunctor", vec![], pseudo_functor_ty())?;
    add_axiom(env, "Tricategory", vec![], tricategory_ty())?;
    add_axiom(
        env,
        "TricategoryCoherence",
        vec![],
        tricategory_coherence_ty(),
    )?;
    add_axiom(env, "Oriental", vec![], oriental_ty())?;
    add_axiom(env, "GlobularComplex", vec![], globular_complex_ty())?;
    add_axiom(env, "Computad", vec![], computad_ty())?;
    add_axiom(env, "Polygraph", vec![], polygraph_ty())?;
    add_axiom(env, "FreeOmegaCategory", vec![], free_omega_category_ty())?;
    add_axiom(env, "PastingDiagram", vec![], pasting_diagram_ty())?;
    add_axiom(env, "Bilimit", vec![], bilimit_ty())?;
    add_axiom(env, "Pseudolimit", vec![], pseudolimit_ty())?;
    add_axiom(env, "Strict2Limit", vec![], strict_2_limit_ty())?;
    add_axiom(env, "Bicolimit", vec![], bicolimit_ty())?;
    add_axiom(env, "EnrichedCategory", vec![], enriched_category_ty())?;
    add_axiom(env, "EnrichedFunctor", vec![], enriched_functor_ty())?;
    add_axiom(
        env,
        "LaxNaturalTransformation",
        vec![],
        lax_natural_transformation_ty(),
    )?;
    add_axiom(env, "Modification", vec![], modification_ty())?;
    add_axiom(
        env,
        "OplaxNaturalTransformation",
        vec![],
        oplax_natural_transformation_ty(),
    )?;
    add_axiom(
        env,
        "StrictOmegaCategory",
        vec![],
        strict_omega_category_ty(),
    )?;
    add_axiom(env, "WeakOmegaCategory", vec![], weak_omega_category_ty())?;
    add_axiom(env, "BataninOperad", vec![], batanin_operad_ty())?;
    add_axiom(env, "BataninAlgebra", vec![], batanin_algebra_ty())?;
    add_axiom(
        env,
        "ContractibleGlobularCollection",
        vec![],
        contractible_globular_collection_ty(),
    )?;
    add_axiom(env, "TamsamaniWeakNCat", vec![], tamsamani_weak_n_cat_ty())?;
    add_axiom(env, "SimpsonConjecture", vec![], simpson_conjecture_ty())?;
    add_axiom(env, "NerveFunctor", vec![], nerve_functor_ty())?;
    add_axiom(
        env,
        "OmegaCategoryEquivalence",
        vec![],
        omega_category_equivalence_ty(),
    )?;
    add_axiom(
        env,
        "FormalCategoryTheory",
        vec![],
        formal_category_theory_ty(),
    )?;
    Ok(())
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
    fn test_quasi_category_registered() {
        let env = test_env();
        assert!(env.get(&Name::str("QuasiCategory")).is_some());
        assert!(env.get(&Name::str("InnerHorn")).is_some());
        assert!(env.get(&Name::str("InftyFunctor")).is_some());
    }
    #[test]
    fn test_natural_transformation_registered() {
        let env = test_env();
        assert!(env.get(&Name::str("InftyNaturalTransformation")).is_some());
        assert!(env.get(&Name::str("HasInnerHornFillings")).is_some());
        assert!(env.get(&Name::str("IsKanComplex")).is_some());
        assert!(env.get(&Name::str("IsEquivalence")).is_some());
    }
    #[test]
    fn test_limits_colimits_registered() {
        let env = test_env();
        assert!(env.get(&Name::str("InftyLimit")).is_some());
        assert!(env.get(&Name::str("InftyColimit")).is_some());
        assert!(env.get(&Name::str("InftyAdjunction")).is_some());
        assert!(env.get(&Name::str("KanExtension")).is_some());
    }
    #[test]
    fn test_presentable_registered() {
        let env = test_env();
        assert!(env.get(&Name::str("PresentableInftyCategory")).is_some());
        assert!(env.get(&Name::str("AccessibleCategory")).is_some());
        assert!(env.get(&Name::str("LocalizationFunctor")).is_some());
        assert!(env.get(&Name::str("CompactlyGenerated")).is_some());
    }
    #[test]
    fn test_topos_registered() {
        let env = test_env();
        assert!(env.get(&Name::str("InftyTopos")).is_some());
        assert!(env.get(&Name::str("ObjectClassifier")).is_some());
        assert!(env.get(&Name::str("EtaleMorphism")).is_some());
        assert!(env.get(&Name::str("HypercoverDescent")).is_some());
    }
    #[test]
    fn test_fibrations_registered() {
        let env = test_env();
        assert!(env.get(&Name::str("CartesianFibration")).is_some());
        assert!(env.get(&Name::str("CoCartesianFibration")).is_some());
        assert!(env.get(&Name::str("StraighteningEquivalence")).is_some());
        assert!(env.get(&Name::str("GrothendieckConstruction")).is_some());
    }
    #[test]
    fn test_operads_registered() {
        let env = test_env();
        assert!(env.get(&Name::str("InftyOperad")).is_some());
        assert!(env.get(&Name::str("AssociativeOperad")).is_some());
        assert!(env.get(&Name::str("CommutativeOperad")).is_some());
        assert!(env.get(&Name::str("EnAlgebra")).is_some());
    }
    #[test]
    fn test_goodwillie_registered() {
        let env = test_env();
        assert!(env.get(&Name::str("ExcisiveFunctor")).is_some());
        assert!(env.get(&Name::str("TaylorTower")).is_some());
        assert!(env.get(&Name::str("HomogeneousFunctor")).is_some());
        assert!(env.get(&Name::str("SpanierWhiteheadDuality")).is_some());
    }
    #[test]
    fn test_inner_horn_validity() {
        let h = InnerHorn::new(3, 1);
        assert!(h.is_valid());
        assert_eq!(h.present_faces, vec![0, 2, 3]);
    }
    #[test]
    fn test_en_algebra() {
        let a = EnAlgebra::new(3, "R");
        assert!(a.is_at_least_e_m(2));
        assert!(a.is_at_least_e_m(3));
        assert!(!a.is_at_least_e_m(4));
    }
    #[test]
    fn test_taylor_tower() {
        let tower = TaylorTower {
            approximations: vec![
                ExcisiveFunctor {
                    degree: 0,
                    verified: true,
                },
                ExcisiveFunctor {
                    degree: 1,
                    verified: true,
                },
                ExcisiveFunctor {
                    degree: 2,
                    verified: true,
                },
            ],
        };
        assert!(tower.converges_at(2));
        assert!(tower.get(1).is_some_and(|f| f.is_n_excisive(1)));
    }
    #[test]
    fn test_infty_functor_equivalence() {
        let f = InftyFunctor {
            obj_map: vec![0, 1, 2],
            mor_map: vec![0, 1, 2],
        };
        assert!(f.is_equivalence());
        let g = InftyFunctor {
            obj_map: vec![0, 0, 1],
            mor_map: vec![0, 1, 2],
        };
        assert!(!g.is_equivalence());
    }
    fn test_env_extended() -> Environment {
        let mut env = Environment::new();
        build_env_all(&mut env).expect("build_env_all failed");
        env
    }
    #[test]
    fn test_segal_spaces_registered() {
        let env = test_env_extended();
        assert!(env.get(&Name::str("SegalSpace")).is_some());
        assert!(env.get(&Name::str("SegalCondition")).is_some());
        assert!(env.get(&Name::str("CompleteSegalSpace")).is_some());
        assert!(env.get(&Name::str("CompletenessCondition")).is_some());
        assert!(env.get(&Name::str("RezkCompletion")).is_some());
    }
    #[test]
    fn test_infinity_n_cat_registered() {
        let env = test_env_extended();
        assert!(env.get(&Name::str("InfinityNCat")).is_some());
    }
    #[test]
    fn test_globular_registered() {
        let env = test_env_extended();
        assert!(env.get(&Name::str("GlobularSet")).is_some());
        assert!(env.get(&Name::str("GlobularCell")).is_some());
        assert!(env.get(&Name::str("ThetaNCategory")).is_some());
        assert!(env.get(&Name::str("ThetaNFunctor")).is_some());
        assert!(env.get(&Name::str("GlobularComposition")).is_some());
    }
    #[test]
    fn test_two_category_registered() {
        let env = test_env_extended();
        assert!(env.get(&Name::str("TwoCategory")).is_some());
        assert!(env.get(&Name::str("TwoCategoryFunctor")).is_some());
        assert!(env.get(&Name::str("GrayTensorProduct")).is_some());
        assert!(env.get(&Name::str("PseudoFunctor")).is_some());
        assert!(env.get(&Name::str("Tricategory")).is_some());
        assert!(env.get(&Name::str("TricategoryCoherence")).is_some());
    }
    #[test]
    fn test_computads_registered() {
        let env = test_env_extended();
        assert!(env.get(&Name::str("Oriental")).is_some());
        assert!(env.get(&Name::str("GlobularComplex")).is_some());
        assert!(env.get(&Name::str("Computad")).is_some());
        assert!(env.get(&Name::str("Polygraph")).is_some());
        assert!(env.get(&Name::str("FreeOmegaCategory")).is_some());
        assert!(env.get(&Name::str("PastingDiagram")).is_some());
    }
    #[test]
    fn test_two_limits_registered() {
        let env = test_env_extended();
        assert!(env.get(&Name::str("Bilimit")).is_some());
        assert!(env.get(&Name::str("Pseudolimit")).is_some());
        assert!(env.get(&Name::str("Strict2Limit")).is_some());
        assert!(env.get(&Name::str("Bicolimit")).is_some());
    }
    #[test]
    fn test_enriched_lax_registered() {
        let env = test_env_extended();
        assert!(env.get(&Name::str("EnrichedCategory")).is_some());
        assert!(env.get(&Name::str("EnrichedFunctor")).is_some());
        assert!(env.get(&Name::str("LaxNaturalTransformation")).is_some());
        assert!(env.get(&Name::str("Modification")).is_some());
        assert!(env.get(&Name::str("OplaxNaturalTransformation")).is_some());
    }
    #[test]
    fn test_omega_categories_registered() {
        let env = test_env_extended();
        assert!(env.get(&Name::str("StrictOmegaCategory")).is_some());
        assert!(env.get(&Name::str("WeakOmegaCategory")).is_some());
        assert!(env.get(&Name::str("BataninOperad")).is_some());
        assert!(env.get(&Name::str("BataninAlgebra")).is_some());
        assert!(env
            .get(&Name::str("ContractibleGlobularCollection"))
            .is_some());
        assert!(env.get(&Name::str("TamsamaniWeakNCat")).is_some());
        assert!(env.get(&Name::str("SimpsonConjecture")).is_some());
        assert!(env.get(&Name::str("NerveFunctor")).is_some());
        assert!(env.get(&Name::str("OmegaCategoryEquivalence")).is_some());
        assert!(env.get(&Name::str("FormalCategoryTheory")).is_some());
    }
    #[test]
    fn test_globular_set_data() {
        let gs = GlobularSetData::new(vec![
            vec!["x".into(), "y".into()],
            vec!["f".into(), "g".into()],
            vec!["alpha".into()],
        ]);
        assert_eq!(gs.cell_count(0), 2);
        assert_eq!(gs.cell_count(1), 2);
        assert_eq!(gs.cell_count(2), 1);
        assert!(gs.source(1, 0).is_some());
        assert!(gs.target(1, 0).is_some());
        assert!(gs.source(0, 0).is_none());
        assert!(gs.satisfies_globularity());
    }
    #[test]
    fn test_segal_space_data() {
        let ss = SegalSpaceData::new(vec![
            vec!["pt".into()],
            vec!["pt".into()],
            vec!["pt".into()],
        ]);
        assert!(ss.check_segal_condition(2));
        assert_eq!(ss.equivalences_count(), 1);
    }
    #[test]
    fn test_two_category_data() {
        let mut cat = TwoCategoryData::new();
        let a = cat.add_object("A");
        let b = cat.add_object("B");
        let c = cat.add_object("C");
        let f = cat.add_one_morphism(a, b, "f");
        let g = cat.add_one_morphism(b, c, "g");
        let _alpha = cat.add_two_morphism(f, f, "id_f");
        assert_eq!(cat.objects.len(), 3);
        assert_eq!(cat.one_morphisms.len(), 2);
        assert_eq!(cat.two_morphisms.len(), 1);
        assert!(cat.compose_1morph(f, g).is_some());
        assert!(cat.compose_1morph(g, f).is_none());
    }
    #[test]
    fn test_computad_data() {
        let mut c = ComputadData::new();
        let _x = c.add_generator(0, vec![], vec![], "x");
        let _y = c.add_generator(0, vec![], vec![], "y");
        let _f = c.add_generator(1, vec![0], vec![1], "f");
        assert_eq!(c.generator_count(0), 2);
        assert_eq!(c.generator_count(1), 1);
        assert!(c.is_well_typed());
    }
    #[test]
    fn test_infinity_n_cat_data() {
        let ss = SegalSpaceData::new(vec![vec!["a".into(), "b".into()], vec!["f".into()]]);
        let mut cat = InfinityNCatData::new(1, ss);
        assert!(cat.is_quasi_category());
        assert!(!cat.is_infinity_groupoid());
        assert!(!cat.invertibility_verified);
        cat.verify_invertibility();
        assert!(cat.invertibility_verified);
    }
    #[test]
    fn test_omega_category_nerve() {
        let c = OmegaCategory::new(Some(2));
        assert!(c.is_strict());
        let nerve = c.street_roberts_nerve();
        assert!(nerve.contains("2"));
        let inf_c = OmegaCategory::new(None);
        let inf_nerve = inf_c.street_roberts_nerve();
        assert!(inf_nerve.contains("ω"));
    }
}
#[cfg(test)]
mod tests_higher_cat_extended {
    use super::*;
    #[test]
    fn test_infinity_n_category_basic() {
        let cat = InfinityNCategory::new(1, "Spaces");
        assert!(cat.is_infinity_one_category());
        assert!(!cat.is_infinity_groupoid());
        assert!(cat.is_cobordism_hypothesis_target());
    }
    #[test]
    fn test_infinity_zero_is_groupoid() {
        let cat = InfinityNCategory::new(0, "Groupoid");
        assert!(cat.is_infinity_groupoid());
    }
    #[test]
    fn test_truncation() {
        let cat = InfinityNCategory::new(3, "3-category");
        let trunc = cat.truncate_to(2);
        assert_eq!(trunc.n, 2);
    }
    #[test]
    fn test_operad_koszul() {
        let lie = OperadNew::lie();
        assert!(lie.is_koszul());
        assert!(lie.koszul_dual.is_some());
    }
    #[test]
    fn test_little_disks_operad() {
        let e2 = OperadNew::little_disks(2);
        assert_eq!(e2.name, "E_2");
    }
    #[test]
    fn test_factorization_algebra_en() {
        let mut fa = FactorizationAlgebra::new(3, "ChernSimons");
        fa.is_locally_constant = true;
        let desc = fa.corresponding_en_algebra();
        assert!(desc.contains("E_3"));
    }
    #[test]
    fn test_factorization_algebra_vertex() {
        let mut fa = FactorizationAlgebra::new(1, "VirasAlgebra");
        fa.is_holomorphic = true;
        let va = fa.corresponding_vertex_algebra();
        assert!(va.is_some());
    }
    #[test]
    fn test_factorization_homology() {
        let fa = FactorizationAlgebra::new(2, "E2-algebra");
        let fh = fa.factorization_homology("Torus");
        assert!(fh.contains("Torus"));
    }
}
#[cfg(test)]
mod tests_hct_extra {
    use super::*;
    #[test]
    fn test_enriched_category() {
        let ab_cat = EnrichedCategory::ab_enriched("Mod_R");
        assert!(ab_cat.is_preadditive());
        assert!(ab_cat.is_symmetric);
    }
    #[test]
    fn test_monoidal_functor() {
        let f = MonoidalFunctor::strict("C", "D");
        assert!(f.is_strict && f.is_lax && f.is_strong);
        let g = MonoidalFunctor::new("C", "D");
        assert!(!g.is_strict);
    }
    #[test]
    fn test_operad() {
        let assoc = OperadV2::assoc();
        assert!(!assoc.is_symmetric);
        let comm = OperadV2::comm();
        assert!(comm.is_symmetric);
    }
    #[test]
    fn test_quasi_category() {
        let kan = QuasiCategoryNew::infinity_groupoid("Spaces");
        assert!(kan.all_morphisms_invertible());
        let nerve = QuasiCategoryNew::nerve_of_ordinary_cat("C");
        assert!(!nerve.is_kan_complex);
    }
}
