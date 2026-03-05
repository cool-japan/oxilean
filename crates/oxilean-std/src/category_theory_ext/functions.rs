//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    Adjunction, Category, DerivedCategoryComplex, EInfinityAlgebraOps, Functor, Monad, NatTrans,
    QuasiCategoryHorn, SegalConditionChecker, SixFunctorComputations,
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
pub fn app4(f: Expr, a: Expr, b: Expr, c: Expr, d: Expr) -> Expr {
    app(app3(f, a, b, c), d)
}
pub fn pi(bi: BinderInfo, name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(bi, Name::str(name), Box::new(dom), Box::new(body))
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
/// `category_ty()` — type of a (large) category; lives in Type 1.
pub fn category_ty() -> Expr {
    type1()
}
/// `functor_ty()` — type of a functor F : C → D between two categories.
///
/// Encoded as: `Category → Category → Type₀`.
pub fn functor_ty() -> Expr {
    arrow(category_ty(), arrow(category_ty(), type0()))
}
/// `nat_trans_ty()` — type of a natural transformation η : F → G.
pub fn nat_trans_ty() -> Expr {
    type0()
}
/// `adjunction_ty()` — type of an adjunction F ⊣ G (F left, G right adjoint).
pub fn adjunction_ty() -> Expr {
    type0()
}
/// `monad_ty()` — type of a monad on a category C.
///
/// Encoded as: `Category → Type₀`.
pub fn monad_ty() -> Expr {
    arrow(category_ty(), type0())
}
/// `comonad_ty()` — type of a comonad on a category C.
pub fn comonad_ty() -> Expr {
    arrow(category_ty(), type0())
}
/// `limit_ty()` — type of the limit of a diagram (cone apex).
pub fn limit_ty() -> Expr {
    type0()
}
/// `colimit_ty()` — type of the colimit of a diagram (cocone apex).
pub fn colimit_ty() -> Expr {
    type0()
}
/// `monoidal_category_ty()` — type of a monoidal category.
///
/// A monoidal category extends a category with a tensor product ⊗ and unit I.
pub fn monoidal_category_ty() -> Expr {
    type0()
}
/// `enriched_category_ty()` — type of a category enriched over a monoidal category V.
///
/// Encoded as: `MonoidalCategory → Type₀`.
pub fn enriched_category_ty() -> Expr {
    arrow(monoidal_category_ty(), type0())
}
/// `two_category_ty()` — type of a 2-category (objects, 1-cells, 2-cells).
pub fn two_category_ty() -> Expr {
    type0()
}
/// `topos_ty()` — type of an elementary topos (has subobject classifier Ω).
pub fn topos_ty() -> Expr {
    type0()
}
/// `yoneda_lemma_ty()` — the Yoneda lemma: Nat(Hom(A, -), F) ≅ F(A).
///
/// For any locally small category C, object A, and functor F : C → Set,
/// the natural transformations from the hom-functor Hom(A, -) to F are
/// in natural bijection with F(A).
pub fn yoneda_lemma_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "C",
        category_ty(),
        pi(
            BinderInfo::Default,
            "A",
            bvar(0),
            pi(
                BinderInfo::Default,
                "F",
                app2(cst("Functor"), bvar(1), cst("Set")),
                app2(
                    cst("Iso"),
                    app2(cst("NatTrans"), app(cst("Hom"), bvar(1)), bvar(0)),
                    app(bvar(0), bvar(1)),
                ),
            ),
        ),
    )
}
/// `adjunction_unit_counit_ty()` — unit-counit triangle identities for F ⊣ G.
///
/// Given F ⊣ G with unit η : Id → G∘F and counit ε : F∘G → Id, the triangles
/// (εF)∘(Fη) = id_F and (Gε)∘(ηG) = id_G must hold.
pub fn adjunction_unit_counit_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        nat_trans_ty(),
        pi(
            BinderInfo::Default,
            "G",
            nat_trans_ty(),
            arrow(
                app2(cst("Adjunction"), bvar(1), bvar(0)),
                app2(cst("TriangleIdentities"), bvar(2), bvar(1)),
            ),
        ),
    )
}
/// `monad_from_adjunction_ty()` — every adjunction F ⊣ G gives a monad G∘F.
///
/// The composite G∘F : C → C carries a monad structure with
/// unit η : Id → G∘F and multiplication G(ε_F) : (G∘F)∘(G∘F) → G∘F.
pub fn monad_from_adjunction_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        nat_trans_ty(),
        pi(
            BinderInfo::Default,
            "G",
            nat_trans_ty(),
            arrow(
                app2(cst("Adjunction"), bvar(1), bvar(0)),
                app(cst("Monad"), app2(cst("Compose"), bvar(1), bvar(2))),
            ),
        ),
    )
}
/// `limits_from_products_equalizers_ty()` — all limits exist given products and equalizers.
///
/// A category has all small limits iff it has all small products and equalizers.
pub fn limits_from_products_equalizers_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "C",
        category_ty(),
        arrow(
            app(cst("HasProducts"), bvar(0)),
            arrow(
                app(cst("HasEqualizers"), bvar(1)),
                app(cst("HasLimits"), bvar(2)),
            ),
        ),
    )
}
/// `eckmann_hilton_ty()` — the Eckmann-Hilton argument.
///
/// If a set has two monoid structures that interchange, they are equal and commutative.
/// Used to show π₂ is abelian.
pub fn eckmann_hilton_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        pi(
            BinderInfo::Default,
            "op1",
            arrow(bvar(0), arrow(bvar(1), bvar(2))),
            pi(
                BinderInfo::Default,
                "op2",
                arrow(bvar(1), arrow(bvar(2), bvar(3))),
                arrow(
                    app2(cst("Interchange"), bvar(1), bvar(0)),
                    app2(
                        cst("And"),
                        app2(cst("Eq"), bvar(2), bvar(1)),
                        app(cst("Commutative"), bvar(2)),
                    ),
                ),
            ),
        ),
    )
}
/// `representable_functor_ty()` — a functor is representable iff it has a universal element.
///
/// F : C → Set is representable iff there exists A and η ∈ F(A) that is universal.
pub fn representable_functor_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "C",
        category_ty(),
        pi(
            BinderInfo::Default,
            "F",
            app2(cst("Functor"), bvar(0), cst("Set")),
            app2(
                cst("Iff"),
                app(cst("Representable"), bvar(0)),
                app(cst("HasUniversalElement"), bvar(0)),
            ),
        ),
    )
}
/// `free_forgetful_adjunction_ty()` — free and forgetful functors form an adjunction.
///
/// For algebraic structures (groups, rings, etc.), the free functor F : Set → Alg
/// is left adjoint to the forgetful functor U : Alg → Set.
pub fn free_forgetful_adjunction_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "Alg",
        category_ty(),
        app2(
            cst("Adjunction"),
            app(cst("Free"), bvar(0)),
            app(cst("Forget"), bvar(0)),
        ),
    )
}
/// `beck_monadicity_ty()` — Beck's monadicity theorem.
///
/// A functor G : D → C is monadic iff it has a left adjoint F and creates
/// coequalizers of G-split pairs.
pub fn beck_monadicity_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        nat_trans_ty(),
        pi(
            BinderInfo::Default,
            "G",
            nat_trans_ty(),
            arrow(
                app2(cst("Adjunction"), bvar(1), bvar(0)),
                arrow(
                    app(cst("CreatesCoequalizersSplitPairs"), bvar(1)),
                    app(cst("IsMonadic"), bvar(1)),
                ),
            ),
        ),
    )
}
/// `kan_extension_ty()` — Kan extensions exist in complete categories.
///
/// If C has all small limits, then for any functor K : A → B and F : A → C,
/// the right Kan extension Ran_K F exists.
pub fn kan_extension_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "C",
        category_ty(),
        arrow(
            app(cst("HasLimits"), bvar(0)),
            pi(
                BinderInfo::Default,
                "K",
                nat_trans_ty(),
                pi(
                    BinderInfo::Default,
                    "F",
                    nat_trans_ty(),
                    app(cst("Exists"), app2(cst("RanKF"), bvar(1), bvar(0))),
                ),
            ),
        ),
    )
}
/// `quasi_category_ty()` — type of a quasi-category (weak Kan complex).
///
/// A quasi-category is a simplicial set satisfying horn filling for all inner horns.
pub fn quasi_category_ty() -> Expr {
    type0()
}
/// `segal_condition_ty()` — the Segal condition for a simplicial space X.
///
/// X is a Segal space iff X_n ≅ X_1 ×_{X_0} ··· ×_{X_0} X_1 (n-fold fiber product).
pub fn segal_condition_ty() -> Expr {
    arrow(type0(), prop())
}
/// `joyal_model_structure_ty()` — type asserting sSet has Joyal's model structure.
///
/// The Joyal model structure on simplicial sets presents (∞,1)-categories.
pub fn joyal_model_structure_ty() -> Expr {
    prop()
}
/// `complete_segal_space_ty()` — Rezk's complete Segal spaces.
///
/// A Segal space X where the space of equivalences is X_0 (completeness).
pub fn complete_segal_space_ty() -> Expr {
    arrow(type0(), prop())
}
/// `lurie_infty_category_ty()` — Lurie's ∞-category (quasi-category).
///
/// As in HTT: an ∞-category is a simplicial set with inner horn extensions.
pub fn lurie_infty_category_ty() -> Expr {
    type1()
}
/// `theta_n_space_ty()` — Θ_n-space for (∞,n)-categories.
///
/// Spaces over Joyal's disk category Θ_n satisfying Segal conditions.
pub fn theta_n_space_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `barwick_n_fold_segal_ty()` — Barwick's n-fold Segal spaces.
///
/// Multisimplicial spaces satisfying Segal conditions in each direction.
pub fn barwick_n_fold_segal_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `gray_tensor_product_ty()` — the Gray tensor product of 2-categories.
///
/// A non-symmetric monoidal structure on 2-Cat capturing lax naturality.
pub fn gray_tensor_product_ty() -> Expr {
    arrow(
        two_category_ty(),
        arrow(two_category_ty(), two_category_ty()),
    )
}
/// `e_n_operad_ty()` — type of an E_n operad (little n-disks operad).
///
/// E_n-algebras are algebras over the little n-cubes operad.
pub fn e_n_operad_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `e_infinity_ring_spectrum_ty()` — type of an E_∞ ring spectrum.
///
/// Commutative monoids in the stable ∞-category of spectra.
pub fn e_infinity_ring_spectrum_ty() -> Expr {
    type0()
}
/// `monoidal_infty_category_ty()` — type of a monoidal ∞-category.
///
/// An ∞-category with a coherent monoidal structure (associative up to all higher homotopies).
pub fn monoidal_infty_category_ty() -> Expr {
    type1()
}
/// `day_convolution_ty()` — Day convolution monoidal structure.
///
/// For a monoidal category C, the functor category [C, Set] is monoidal via Day convolution.
pub fn day_convolution_ty() -> Expr {
    arrow(monoidal_category_ty(), monoidal_category_ty())
}
/// `lurie_infty_topos_ty()` — type of an ∞-topos (Lurie's definition).
///
/// An ∞-category that satisfies Giraud's axioms up to homotopy.
pub fn lurie_infty_topos_ty() -> Expr {
    type1()
}
/// `descent_condition_ty()` — effective descent in an ∞-topos.
///
/// Colimits in an ∞-topos are stable under base change (descent = van Kampen).
pub fn descent_condition_ty() -> Expr {
    arrow(lurie_infty_topos_ty(), prop())
}
/// `cohesive_infty_topos_ty()` — cohesive ∞-topos (Lawvere's cohesion axioms).
///
/// An ∞-topos with an adjoint quadruple relating discrete and codiscrete objects.
pub fn cohesive_infty_topos_ty() -> Expr {
    arrow(lurie_infty_topos_ty(), prop())
}
/// `factorization_homology_ty()` — factorization homology ∫_M A.
///
/// For an E_n-algebra A and framed n-manifold M, ∫_M A is a derived invariant.
pub fn factorization_homology_ty() -> Expr {
    arrow(e_n_operad_ty(), arrow(type0(), type0()))
}
/// `ran_space_ty()` — the Ran space of a manifold.
///
/// Ran(M) = colim of M^I over finite sets I, used in factorization homology.
pub fn ran_space_ty() -> Expr {
    type0()
}
/// `excision_property_ty()` — excision for factorization homology.
///
/// ∫_{M₁ ∪_N M₂} A ≃ ∫_{M₁} A ⊗_{∫_N A} ∫_{M₂} A.
pub fn excision_property_ty() -> Expr {
    arrow(factorization_homology_ty(), prop())
}
/// `six_functor_ty()` — the six-functor formalism (f*, f_!, f^!, Rf_*, ⊗, Hom).
///
/// For a morphism f : X → Y between spaces, there are six related functors.
pub fn six_functor_ty() -> Expr {
    type0()
}
/// `derived_scheme_ty()` — type of a derived scheme.
///
/// A derived scheme is a spectral space locally modeled on E_∞ ring spectra.
pub fn derived_scheme_ty() -> Expr {
    type0()
}
/// `structured_space_ty()` — structured space in the sense of Lurie.
///
/// A topological space equipped with a sheaf of E_∞ ring spectra.
pub fn structured_space_ty() -> Expr {
    arrow(e_infinity_ring_spectrum_ty(), type0())
}
/// `spectral_stack_ty()` — type of a spectral stack.
///
/// A sheaf of ∞-groupoids on the ∞-site of E_∞ ring spectra.
pub fn spectral_stack_ty() -> Expr {
    type0()
}
/// `condensed_set_ty()` — type of a condensed set (Clausen-Scholze).
///
/// A sheaf on the category of profinite sets (pyknotic sets / condensed sets).
pub fn condensed_set_ty() -> Expr {
    type0()
}
/// `liquid_module_ty()` — type of a liquid module over a condensed ring.
///
/// Used in Clausen-Scholze's theory of analytic geometry.
pub fn liquid_module_ty() -> Expr {
    arrow(condensed_set_ty(), type0())
}
/// `analytic_geometry_ty()` — type of an analytic space (condensed/pyknotic).
///
/// Analytic spaces generalize rigid analytic spaces and p-adic geometry.
pub fn analytic_geometry_ty() -> Expr {
    type0()
}
/// `perfectoid_space_ty()` — type of a perfectoid space.
///
/// Perfectoid spaces are used in Scholze's proof of the weight-monodromy conjecture.
pub fn perfectoid_space_ty() -> Expr {
    type0()
}
/// `tilting_equivalence_ty()` — the tilting equivalence as an ∞-categorical equivalence.
///
/// For a perfectoid field K with tilt K^♭, the ∞-category of étale K-spaces ≃ K^♭-spaces.
pub fn tilting_equivalence_ty() -> Expr {
    arrow(
        perfectoid_space_ty(),
        app2(cst("Equiv"), perfectoid_space_ty(), perfectoid_space_ty()),
    )
}
/// `decategorification_ty()` — decategorification: passing from categories to sets.
///
/// The Grothendieck group K₀ is a decategorification of the derived category.
pub fn decategorification_ty() -> Expr {
    arrow(category_ty(), type0())
}
/// `additive_category_ty()` — type of an additive category.
///
/// An Ab-enriched category with finite products and coproducts.
pub fn additive_category_ty() -> Expr {
    type0()
}
/// `triangulated_category_ty()` — type of a triangulated category.
///
/// Has a shift functor [1] and distinguished triangles.
pub fn triangulated_category_ty() -> Expr {
    type0()
}
/// `stable_infty_category_ty()` — type of a stable ∞-category.
///
/// An ∞-category that is pointed and has all pullbacks = pushouts.
pub fn stable_infty_category_ty() -> Expr {
    type1()
}
/// `tmf_ty()` — topological modular forms (tmf spectrum).
///
/// The E_∞ ring spectrum of topological modular forms.
pub fn tmf_ty() -> Expr {
    e_infinity_ring_spectrum_ty()
}
/// `spectra_category_ty()` — the stable ∞-category of spectra.
pub fn spectra_category_ty() -> Expr {
    stable_infty_category_ty()
}
/// `infty_groupoid_ty()` — type of an ∞-groupoid (Kan complex).
///
/// An ∞-groupoid is a Kan complex; all morphisms are invertible.
pub fn infty_groupoid_ty() -> Expr {
    type0()
}
/// `higher_morita_ty()` — Morita theory for E_n-algebras.
///
/// Two E_n-algebras A, B are Morita equivalent iff their ∞-categories of modules are equivalent.
pub fn higher_morita_ty() -> Expr {
    arrow(e_n_operad_ty(), arrow(type0(), arrow(type0(), prop())))
}
/// `quasi_category_horn_filling_ty()` — inner horn filling for quasi-categories.
///
/// X is a quasi-category iff every inner horn Λ^n_k → X (0 < k < n) has a filler.
pub fn quasi_category_horn_filling_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        app2(
            cst("Iff"),
            app(cst("IsQuasiCategory"), bvar(0)),
            app(cst("HasInnerHornFillers"), bvar(0)),
        ),
    )
}
/// `complete_segal_space_characterization_ty()` — CSS = (∞,1)-categories.
///
/// Complete Segal spaces and quasi-categories present the same homotopy theory.
pub fn complete_segal_space_characterization_ty() -> Expr {
    app2(
        cst("Equiv"),
        app(cst("CompletesSegalSpaces"), cst("sSet")),
        app(cst("QuasiCategories"), cst("sSet")),
    )
}
/// `lurie_adjoint_functor_theorem_ty()` — adjoint functor theorem for ∞-categories.
///
/// An accessible functor between presentable ∞-categories has a right adjoint
/// iff it preserves small colimits.
pub fn lurie_adjoint_functor_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "C",
        lurie_infty_category_ty(),
        pi(
            BinderInfo::Default,
            "D",
            lurie_infty_category_ty(),
            pi(
                BinderInfo::Default,
                "F",
                arrow(bvar(1), bvar(1)),
                arrow(
                    app(cst("PreservesSmallColimits"), bvar(0)),
                    app(cst("HasRightAdjoint"), bvar(0)),
                ),
            ),
        ),
    )
}
/// `barr_beck_lurie_ty()` — the Barr-Beck-Lurie monadicity theorem.
///
/// An ∞-functor G is monadic iff it is conservative and creates colimits of G-split simplicial objects.
pub fn barr_beck_lurie_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        nat_trans_ty(),
        arrow(
            app(cst("IsConservative"), bvar(0)),
            arrow(
                app(cst("CreatesGSplitSimplicial"), bvar(0)),
                app(cst("IsMonadic"), bvar(0)),
            ),
        ),
    )
}
/// `infty_topos_descent_ty()` — descent in ∞-topoi.
///
/// In an ∞-topos, all colimits are van Kampen (universal and effective).
pub fn infty_topos_descent_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "T",
        lurie_infty_topos_ty(),
        app2(
            cst("And"),
            app(cst("AllColimitsUniversal"), bvar(0)),
            app(cst("AllColimitsEffective"), bvar(0)),
        ),
    )
}
/// `six_functor_base_change_ty()` — proper base change in the six-functor formalism.
///
/// For a Cartesian square, f^! ∘ g_* ≃ g'_* ∘ f'^!.
pub fn six_functor_base_change_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "S",
        six_functor_ty(),
        arrow(
            app(cst("IsCartesianSquare"), bvar(0)),
            app(cst("SatisfiesProperBaseChange"), bvar(0)),
        ),
    )
}
/// `poincare_duality_six_functor_ty()` — Poincaré duality via six functors.
///
/// For a smooth proper morphism f of relative dimension d, f^! ≃ f^* ⊗ ω_{X/S}[d].
pub fn poincare_duality_six_functor_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        six_functor_ty(),
        pi(
            BinderInfo::Default,
            "d",
            nat_ty(),
            arrow(
                app(cst("IsSmoothProper"), bvar(1)),
                app2(cst("SixFunctorDuality"), bvar(2), bvar(1)),
            ),
        ),
    )
}
/// `excision_theorem_ty()` — excision for factorization homology.
///
/// ∫_{M₁ ∪_N M₂} A ≃ ∫_{M₁} A ⊗_{∫_N A} ∫_{M₂} A.
pub fn excision_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(
                app2(cst("IsEn"), bvar(1), bvar(0)),
                app(cst("SatisfiesExcision"), bvar(2)),
            ),
        ),
    )
}
/// `condensed_sets_topos_ty()` — condensed sets form an ∞-topos.
pub fn condensed_sets_topos_ty() -> Expr {
    app(cst("IsInfinityTopos"), cst("CondensedSets"))
}
/// `tilting_infty_equiv_ty()` — tilting as an ∞-categorical equivalence.
///
/// For perfectoid field K, Perf_K ≃ Perf_{K^♭} as ∞-categories.
pub fn tilting_infty_equiv_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "K",
        perfectoid_space_ty(),
        app2(
            cst("InftyEquiv"),
            app(cst("PerfectoidCategory"), bvar(0)),
            app(cst("PerfectoidCategory"), app(cst("Tilt"), bvar(0))),
        ),
    )
}
/// `e_infinity_algebra_rectification_ty()` — rectification of E_∞ algebras.
///
/// Every E_∞ algebra is equivalent (as an ∞-category object) to a strictly commutative one.
pub fn e_infinity_algebra_rectification_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        e_infinity_ring_spectrum_ty(),
        app2(
            cst("Equiv"),
            app(cst("AsEInfinity"), bvar(0)),
            app(cst("AsStrictlyCommutative"), bvar(0)),
        ),
    )
}
/// `presentable_infty_category_ty()` — type of a presentable ∞-category.
///
/// An accessible ∞-category that is cocomplete.
pub fn presentable_infty_category_ty() -> Expr {
    type1()
}
/// `lurie_tensor_product_ty()` — Lurie's tensor product of presentable ∞-categories.
///
/// Pr^L is symmetric monoidal under Lurie's tensor product.
pub fn lurie_tensor_product_ty() -> Expr {
    arrow(
        presentable_infty_category_ty(),
        arrow(
            presentable_infty_category_ty(),
            presentable_infty_category_ty(),
        ),
    )
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
    .map_err(|e| format!("Failed to add axiom '{}': {:?}", name, e))
}
/// Build the extended category theory environment with all axioms and theorems.
///
/// Registers adjunctions, monads, Yoneda, limits/colimits, monoidal categories,
/// enriched categories, 2-categories, and toposes into the given environment.
pub fn build_category_theory_ext_env(env: &mut Environment) -> Result<(), String> {
    add_axiom(
        env,
        "Adjunction",
        vec![],
        arrow(nat_trans_ty(), arrow(nat_trans_ty(), type0())),
    )?;
    add_axiom(
        env,
        "NatTrans",
        vec![],
        arrow(nat_trans_ty(), arrow(nat_trans_ty(), type0())),
    )?;
    add_axiom(env, "Monad", vec![], arrow(nat_trans_ty(), type0()))?;
    add_axiom(env, "Comonad", vec![], arrow(nat_trans_ty(), type0()))?;
    add_axiom(env, "MonoidalCategory", vec![], type0())?;
    add_axiom(
        env,
        "EnrichedCategory",
        vec![],
        arrow(monoidal_category_ty(), type0()),
    )?;
    add_axiom(env, "TwoCategory", vec![], type0())?;
    add_axiom(env, "Topos", vec![], type0())?;
    add_axiom(
        env,
        "Adjunction.unit",
        vec![],
        pi(
            BinderInfo::Default,
            "F",
            nat_trans_ty(),
            pi(
                BinderInfo::Default,
                "G",
                nat_trans_ty(),
                arrow(
                    app2(cst("Adjunction"), bvar(1), bvar(0)),
                    app2(
                        cst("NatTrans"),
                        cst("Id"),
                        app2(cst("Compose"), bvar(1), bvar(2)),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Adjunction.counit",
        vec![],
        pi(
            BinderInfo::Default,
            "F",
            nat_trans_ty(),
            pi(
                BinderInfo::Default,
                "G",
                nat_trans_ty(),
                arrow(
                    app2(cst("Adjunction"), bvar(1), bvar(0)),
                    app2(
                        cst("NatTrans"),
                        app2(cst("Compose"), bvar(2), bvar(1)),
                        cst("Id"),
                    ),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "TriangleIdentities",
        vec![],
        arrow(nat_trans_ty(), arrow(nat_trans_ty(), prop())),
    )?;
    add_axiom(
        env,
        "Adjunction.triangleIdentities",
        vec![],
        adjunction_unit_counit_ty(),
    )?;
    add_axiom(
        env,
        "HomSetBijection",
        vec![],
        arrow(nat_trans_ty(), arrow(nat_trans_ty(), prop())),
    )?;
    add_axiom(
        env,
        "Adjunction.homSetBijection",
        vec![],
        pi(
            BinderInfo::Default,
            "F",
            nat_trans_ty(),
            pi(
                BinderInfo::Default,
                "G",
                nat_trans_ty(),
                arrow(
                    app2(cst("Adjunction"), bvar(1), bvar(0)),
                    app2(cst("HomSetBijection"), bvar(2), bvar(1)),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Monad.unit",
        vec![],
        pi(
            BinderInfo::Default,
            "T",
            nat_trans_ty(),
            arrow(
                app(cst("Monad"), bvar(0)),
                app2(cst("NatTrans"), cst("Id"), bvar(1)),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Monad.multiplication",
        vec![],
        pi(
            BinderInfo::Default,
            "T",
            nat_trans_ty(),
            arrow(
                app(cst("Monad"), bvar(0)),
                app2(
                    cst("NatTrans"),
                    app2(cst("Compose"), bvar(1), bvar(1)),
                    bvar(1),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Monad.fromAdjunction",
        vec![],
        monad_from_adjunction_ty(),
    )?;
    add_axiom(
        env,
        "Hom",
        vec![],
        pi(
            BinderInfo::Default,
            "C",
            category_ty(),
            arrow(bvar(0), app2(cst("Functor"), bvar(1), cst("Set"))),
        ),
    )?;
    add_axiom(env, "Representable", vec![], arrow(nat_trans_ty(), prop()))?;
    add_axiom(
        env,
        "HasUniversalElement",
        vec![],
        arrow(nat_trans_ty(), prop()),
    )?;
    add_axiom(env, "Yoneda.lemma", vec![], yoneda_lemma_ty())?;
    add_axiom(
        env,
        "Yoneda.embedding",
        vec![],
        pi(
            BinderInfo::Default,
            "C",
            category_ty(),
            app(cst("IsFullyFaithful"), app(cst("Hom"), bvar(0))),
        ),
    )?;
    add_axiom(env, "Representable.iff", vec![], representable_functor_ty())?;
    add_axiom(env, "HasProducts", vec![], arrow(category_ty(), prop()))?;
    add_axiom(env, "HasCoproducts", vec![], arrow(category_ty(), prop()))?;
    add_axiom(env, "HasEqualizers", vec![], arrow(category_ty(), prop()))?;
    add_axiom(env, "HasCoequalizers", vec![], arrow(category_ty(), prop()))?;
    add_axiom(env, "HasLimits", vec![], arrow(category_ty(), prop()))?;
    add_axiom(env, "HasColimits", vec![], arrow(category_ty(), prop()))?;
    add_axiom(
        env,
        "HasLimits.fromProductsEqualizers",
        vec![],
        limits_from_products_equalizers_ty(),
    )?;
    add_axiom(
        env,
        "HasColimits.fromCoproductsCoequalizers",
        vec![],
        pi(
            BinderInfo::Default,
            "C",
            category_ty(),
            arrow(
                app(cst("HasCoproducts"), bvar(0)),
                arrow(
                    app(cst("HasCoequalizers"), bvar(1)),
                    app(cst("HasColimits"), bvar(2)),
                ),
            ),
        ),
    )?;
    add_axiom(
        env,
        "RanKF",
        vec![],
        arrow(nat_trans_ty(), arrow(nat_trans_ty(), prop())),
    )?;
    add_axiom(
        env,
        "LanKF",
        vec![],
        arrow(nat_trans_ty(), arrow(nat_trans_ty(), prop())),
    )?;
    add_axiom(env, "Kan.rightExtension", vec![], kan_extension_ty())?;
    add_axiom(
        env,
        "MonoidalCategory.tensor",
        vec![],
        arrow(
            monoidal_category_ty(),
            arrow(type0(), arrow(type0(), type0())),
        ),
    )?;
    add_axiom(
        env,
        "MonoidalCategory.unit",
        vec![],
        arrow(monoidal_category_ty(), type0()),
    )?;
    add_axiom(
        env,
        "MonoidalCategory.associator",
        vec![],
        arrow(monoidal_category_ty(), prop()),
    )?;
    add_axiom(
        env,
        "MonoidalCategory.leftUnitor",
        vec![],
        arrow(monoidal_category_ty(), prop()),
    )?;
    add_axiom(
        env,
        "MonoidalCategory.rightUnitor",
        vec![],
        arrow(monoidal_category_ty(), prop()),
    )?;
    add_axiom(
        env,
        "MonoidalCategory.coherence",
        vec![],
        arrow(monoidal_category_ty(), prop()),
    )?;
    add_axiom(
        env,
        "SymmetricMonoidalCategory",
        vec![],
        arrow(monoidal_category_ty(), prop()),
    )?;
    add_axiom(
        env,
        "BraidedMonoidalCategory",
        vec![],
        arrow(monoidal_category_ty(), prop()),
    )?;
    add_axiom(
        env,
        "EnrichedCategory.homObj",
        vec![],
        pi(
            BinderInfo::Default,
            "V",
            monoidal_category_ty(),
            arrow(
                app(cst("EnrichedCategory"), bvar(0)),
                arrow(type0(), arrow(type0(), type0())),
            ),
        ),
    )?;
    add_axiom(
        env,
        "EnrichedCategory.composition",
        vec![],
        arrow(monoidal_category_ty(), prop()),
    )?;
    add_axiom(env, "OrdEnrichedCategory", vec![], type0())?;
    add_axiom(env, "AbEnrichedCategory", vec![], type0())?;
    add_axiom(
        env,
        "TwoCategory.oneCells",
        vec![],
        arrow(two_category_ty(), type0()),
    )?;
    add_axiom(
        env,
        "TwoCategory.twoCells",
        vec![],
        pi(
            BinderInfo::Default,
            "K",
            two_category_ty(),
            arrow(
                app(cst("TwoCategory.oneCells"), bvar(0)),
                arrow(app(cst("TwoCategory.oneCells"), bvar(1)), prop()),
            ),
        ),
    )?;
    add_axiom(
        env,
        "TwoCategory.verticalComp",
        vec![],
        arrow(two_category_ty(), prop()),
    )?;
    add_axiom(
        env,
        "TwoCategory.horizontalComp",
        vec![],
        arrow(two_category_ty(), prop()),
    )?;
    add_axiom(
        env,
        "TwoCategory.interchangeLaw",
        vec![],
        arrow(two_category_ty(), prop()),
    )?;
    add_axiom(env, "Bicategory", vec![], type0())?;
    add_axiom(env, "EckmannHilton", vec![], eckmann_hilton_ty())?;
    add_axiom(
        env,
        "Topos.subobjectClassifier",
        vec![],
        arrow(topos_ty(), type0()),
    )?;
    add_axiom(
        env,
        "Topos.powerObject",
        vec![],
        arrow(topos_ty(), arrow(type0(), type0())),
    )?;
    add_axiom(
        env,
        "Topos.truthMorphism",
        vec![],
        arrow(topos_ty(), prop()),
    )?;
    add_axiom(
        env,
        "IsElementaryTopos",
        vec![],
        arrow(category_ty(), prop()),
    )?;
    add_axiom(
        env,
        "IsGrothendieckTopos",
        vec![],
        arrow(category_ty(), prop()),
    )?;
    add_axiom(
        env,
        "LawvereTierneyTopology",
        vec![],
        arrow(topos_ty(), prop()),
    )?;
    add_axiom(env, "IsMonadic", vec![], arrow(nat_trans_ty(), prop()))?;
    add_axiom(
        env,
        "CreatesCoequalizersSplitPairs",
        vec![],
        arrow(nat_trans_ty(), prop()),
    )?;
    add_axiom(env, "Beck.monadicity", vec![], beck_monadicity_ty())?;
    add_axiom(env, "Free", vec![], arrow(category_ty(), nat_trans_ty()))?;
    add_axiom(env, "Forget", vec![], arrow(category_ty(), nat_trans_ty()))?;
    add_axiom(
        env,
        "FreeForgetAdjunction",
        vec![],
        free_forgetful_adjunction_ty(),
    )?;
    add_axiom(
        env,
        "Interchange",
        vec![],
        pi(
            BinderInfo::Default,
            "A",
            type0(),
            arrow(
                arrow(bvar(0), arrow(bvar(1), bvar(2))),
                arrow(arrow(bvar(1), arrow(bvar(2), bvar(3))), prop()),
            ),
        ),
    )?;
    add_axiom(
        env,
        "Commutative",
        vec![],
        pi(
            BinderInfo::Default,
            "A",
            type0(),
            arrow(arrow(bvar(0), arrow(bvar(1), bvar(2))), prop()),
        ),
    )?;
    add_axiom(env, "IsQuasiCategory", vec![], arrow(type0(), prop()))?;
    add_axiom(env, "HasInnerHornFillers", vec![], arrow(type0(), prop()))?;
    add_axiom(
        env,
        "QuasiCategory.hornFilling",
        vec![],
        quasi_category_horn_filling_ty(),
    )?;
    add_axiom(env, "IsSegalSpace", vec![], segal_condition_ty())?;
    add_axiom(
        env,
        "IsCompleteSegalSpace",
        vec![],
        complete_segal_space_ty(),
    )?;
    add_axiom(env, "CompletesSegalSpaces", vec![], arrow(type0(), type0()))?;
    add_axiom(env, "QuasiCategories", vec![], arrow(type0(), type0()))?;
    add_axiom(env, "sSet", vec![], type0())?;
    add_axiom(
        env,
        "JoyalModelStructure",
        vec![],
        joyal_model_structure_ty(),
    )?;
    add_axiom(env, "ThetaNSpace", vec![], theta_n_space_ty())?;
    add_axiom(
        env,
        "BarwickNFoldSegalSpace",
        vec![],
        barwick_n_fold_segal_ty(),
    )?;
    add_axiom(env, "GrayTensorProduct", vec![], gray_tensor_product_ty())?;
    add_axiom(env, "EnOperad", vec![], e_n_operad_ty())?;
    add_axiom(
        env,
        "EInfinityRingSpectrum",
        vec![],
        e_infinity_ring_spectrum_ty(),
    )?;
    add_axiom(
        env,
        "MonoidalInfinityCategory",
        vec![],
        monoidal_infty_category_ty(),
    )?;
    add_axiom(env, "DayConvolution", vec![], day_convolution_ty())?;
    add_axiom(env, "IsEn", vec![], arrow(type0(), arrow(nat_ty(), prop())))?;
    add_axiom(env, "SatisfiesExcision", vec![], arrow(type0(), prop()))?;
    add_axiom(env, "Excision.theorem", vec![], excision_theorem_ty())?;
    add_axiom(env, "IsInfinityTopos", vec![], arrow(type1(), prop()))?;
    add_axiom(env, "AllColimitsUniversal", vec![], descent_condition_ty())?;
    add_axiom(env, "AllColimitsEffective", vec![], descent_condition_ty())?;
    add_axiom(
        env,
        "InfinityTopos.descent",
        vec![],
        infty_topos_descent_ty(),
    )?;
    add_axiom(env, "IsCohesive", vec![], cohesive_infty_topos_ty())?;
    add_axiom(env, "CondensedSets", vec![], type1())?;
    add_axiom(
        env,
        "CondensedSets.isTopos",
        vec![],
        condensed_sets_topos_ty(),
    )?;
    add_axiom(
        env,
        "FactorizationHomology",
        vec![],
        factorization_homology_ty(),
    )?;
    add_axiom(env, "RanSpace", vec![], ran_space_ty())?;
    add_axiom(env, "SixFunctor", vec![], six_functor_ty())?;
    add_axiom(
        env,
        "IsCartesianSquare",
        vec![],
        arrow(six_functor_ty(), prop()),
    )?;
    add_axiom(
        env,
        "SatisfiesProperBaseChange",
        vec![],
        arrow(six_functor_ty(), prop()),
    )?;
    add_axiom(
        env,
        "SixFunctor.baseChange",
        vec![],
        six_functor_base_change_ty(),
    )?;
    add_axiom(
        env,
        "IsSmoothProper",
        vec![],
        arrow(six_functor_ty(), prop()),
    )?;
    add_axiom(
        env,
        "SixFunctorDuality",
        vec![],
        arrow(six_functor_ty(), arrow(nat_ty(), prop())),
    )?;
    add_axiom(
        env,
        "SixFunctor.poincareDuality",
        vec![],
        poincare_duality_six_functor_ty(),
    )?;
    add_axiom(env, "DerivedScheme", vec![], derived_scheme_ty())?;
    add_axiom(env, "StructuredSpace", vec![], structured_space_ty())?;
    add_axiom(env, "SpectralStack", vec![], spectral_stack_ty())?;
    add_axiom(env, "TopologicalModularForms", vec![], tmf_ty())?;
    add_axiom(
        env,
        "StableInfinityCategory",
        vec![],
        stable_infty_category_ty(),
    )?;
    add_axiom(env, "SpectraCategory", vec![], spectra_category_ty())?;
    add_axiom(env, "CondensedSet", vec![], condensed_set_ty())?;
    add_axiom(env, "LiquidModule", vec![], liquid_module_ty())?;
    add_axiom(env, "AnalyticSpace", vec![], analytic_geometry_ty())?;
    add_axiom(env, "PerfectoidSpace", vec![], perfectoid_space_ty())?;
    add_axiom(
        env,
        "Tilt",
        vec![],
        arrow(perfectoid_space_ty(), perfectoid_space_ty()),
    )?;
    add_axiom(
        env,
        "PerfectoidCategory",
        vec![],
        arrow(perfectoid_space_ty(), lurie_infty_category_ty()),
    )?;
    add_axiom(
        env,
        "InftyEquiv",
        vec![],
        arrow(type1(), arrow(type1(), prop())),
    )?;
    add_axiom(env, "TiltingEquivalence", vec![], tilting_infty_equiv_ty())?;
    add_axiom(env, "Decategorification", vec![], decategorification_ty())?;
    add_axiom(env, "AdditiveCategory", vec![], additive_category_ty())?;
    add_axiom(
        env,
        "TriangulatedCategory",
        vec![],
        triangulated_category_ty(),
    )?;
    add_axiom(env, "InfinityGroupoid", vec![], infty_groupoid_ty())?;
    add_axiom(env, "HigherMorita", vec![], higher_morita_ty())?;
    add_axiom(
        env,
        "PresentableInfinityCategory",
        vec![],
        presentable_infty_category_ty(),
    )?;
    add_axiom(env, "LurieTensorProduct", vec![], lurie_tensor_product_ty())?;
    add_axiom(
        env,
        "PreservesSmallColimits",
        vec![],
        arrow(nat_trans_ty(), prop()),
    )?;
    add_axiom(
        env,
        "HasRightAdjoint",
        vec![],
        arrow(nat_trans_ty(), prop()),
    )?;
    add_axiom(env, "IsConservative", vec![], arrow(nat_trans_ty(), prop()))?;
    add_axiom(
        env,
        "CreatesGSplitSimplicial",
        vec![],
        arrow(nat_trans_ty(), prop()),
    )?;
    add_axiom(
        env,
        "Lurie.adjointFunctorTheorem",
        vec![],
        lurie_adjoint_functor_theorem_ty(),
    )?;
    add_axiom(
        env,
        "Lurie.barrBeckMonadicity",
        vec![],
        barr_beck_lurie_ty(),
    )?;
    add_axiom(
        env,
        "AsEInfinity",
        vec![],
        arrow(e_infinity_ring_spectrum_ty(), type0()),
    )?;
    add_axiom(
        env,
        "AsStrictlyCommutative",
        vec![],
        arrow(e_infinity_ring_spectrum_ty(), type0()),
    )?;
    add_axiom(
        env,
        "EInfinityAlgebra.rectification",
        vec![],
        e_infinity_algebra_rectification_ty(),
    )?;
    add_axiom(
        env,
        "Equiv",
        vec![],
        arrow(type0(), arrow(type0(), type0())),
    )?;
    Ok(())
}
/// Kleisli composition for the List monad.
///
/// Given `f : S → Vec<S>` and `g : S → Vec<S>`, returns `(g >=> f)(x) = f(x).flat_map(g)`.
pub fn kleisli_compose<S: Clone>(
    f: impl Fn(S) -> Vec<S>,
    g: impl Fn(S) -> Vec<S>,
) -> impl Fn(S) -> Vec<S> {
    move |x| f(x).into_iter().flat_map(&g).collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    use oxilean_kernel::Environment;
    /// Build a small category with 3 objects and test composition associativity.
    ///
    /// Objects: 0, 1, 2
    /// Morphisms:
    ///   0: id_0 (0→0), 1: id_1 (1→1), 2: id_2 (2→2)
    ///   3: f (0→1), 4: g (1→2), 5: h=g∘f (0→2)
    #[test]
    fn test_category_composition() {
        let cat: Category<&str, &str> = Category {
            objects: vec!["A", "B", "C"],
            morphisms: vec![
                (0, 0, "id_A"),
                (1, 1, "id_B"),
                (2, 2, "id_C"),
                (0, 1, "f"),
                (1, 2, "g"),
                (0, 2, "h"),
            ],
            identity: vec![0, 1, 2],
        };
        let fg = cat.compose(3, 4);
        assert_eq!(fg, Some(5), "f then g should compose to h");
        let id_f = cat.compose(0, 3);
        assert_eq!(id_f, Some(3), "id_A ∘ f = f");
        let f_id = cat.compose(3, 1);
        assert_eq!(f_id, Some(3), "f ∘ id_B = f");
        assert!(
            cat.is_associative(0, 3, 1),
            "id_A ∘ f ∘ id_B is associative"
        );
    }
    /// Test that an Option-like monad satisfies left and right unit laws.
    #[test]
    fn test_monad_laws() {
        let identity_monad: Monad<i32> = Monad {
            unit: |x| x,
            bind: |m, f| f(m),
        };
        let double: fn(i32) -> i32 = |x| x * 2;
        assert!(identity_monad.left_unit_law(5, double), "left unit law");
        assert!(identity_monad.right_unit_law(42), "right unit law");
        assert!(identity_monad.right_unit_law(-1), "right unit law negative");
    }
    /// Test Kleisli composition for the List monad.
    #[test]
    fn test_kleisli() {
        let f = |x: i32| vec![x, x + 1];
        let g = |x: i32| vec![x * 3];
        let fg = kleisli_compose(f, g);
        let result = fg(2);
        assert_eq!(result, vec![6, 9], "kleisli_compose(f,g)(2) = [6,9]");
        let fg2 = kleisli_compose(|x: i32| vec![x, -x], |x: i32| vec![x + 100]);
        assert_eq!(fg2(1), vec![101, 99], "kleisli_compose handles negatives");
    }
    /// Test naturality square for a trivial natural transformation.
    #[test]
    fn test_nat_trans() {
        let cat: Category<&str, &str> = Category {
            objects: vec!["A", "B"],
            morphisms: vec![(0, 0, "id_A"), (1, 1, "id_B"), (0, 1, "f")],
            identity: vec![0, 1],
        };
        let eta = NatTrans {
            components: vec![0, 1],
        };
        let natural = eta.is_natural(&cat, &cat, 0, 1, 2, 2);
        assert!(natural, "identity nat trans should be natural");
    }
    /// Test QuasiCategoryHorn: inner horn creation and filling.
    #[test]
    fn test_quasi_category_horn() {
        let horn = QuasiCategoryHorn::new_inner(2, 1).expect("inner horn 2,1 should exist");
        assert!(horn.is_inner(), "Λ^2_1 is an inner horn");
        assert_eq!(horn.n, 2);
        assert_eq!(horn.k, 1);
        let filler = horn.fill();
        assert!(filler.is_some(), "inner horn should have a filler");
        let outer = QuasiCategoryHorn::new_inner(2, 0);
        assert!(outer.is_none(), "Λ^2_0 is not an inner horn");
        let horn3 = QuasiCategoryHorn::new_inner(3, 2).expect("inner horn 3,2 should exist");
        assert!(horn3.is_inner(), "Λ^3_2 is an inner horn");
    }
    /// Test SegalConditionChecker on a small simplicial set.
    #[test]
    fn test_segal_condition_checker() {
        let levels = vec![
            vec![vec![0u64], vec![1]],
            vec![vec![0, 1], vec![1, 2]],
            vec![vec![0, 1, 2]],
        ];
        let checker = SegalConditionChecker::new(levels);
        assert!(checker.check_level(2), "simple Segal check at level 2");
        assert!(checker.check_all(), "all Segal conditions satisfied");
    }
    /// Test EInfinityAlgebraOps with Z/2Z (boolean algebra).
    #[test]
    fn test_e_infinity_algebra() {
        let algebra: EInfinityAlgebraOps<u32> = EInfinityAlgebraOps {
            elements: vec![0, 1],
            unit_idx: 0,
            multiply: vec![vec![0, 1], vec![1, 0]],
        };
        assert!(algebra.is_commutative(), "Z/2Z is commutative");
        assert!(algebra.is_associative(), "Z/2Z is associative");
        assert!(algebra.has_unit(), "Z/2Z has unit 0");
        assert!(algebra.verify_e_infinity(), "Z/2Z is an E_∞ algebra");
    }
    /// Test DerivedCategoryComplex: chain complex with d^2 = 0.
    #[test]
    fn test_derived_category_complex() {
        let cx = DerivedCategoryComplex {
            dimensions: vec![2, 1],
            lo_degree: 0,
            differentials: vec![vec![vec![1, -1]]],
        };
        assert!(cx.is_chain_complex(), "d^2 = 0 for this complex");
        assert_eq!(cx.euler_characteristic(), 2 - 1, "chi = 1");
        let cx2 = DerivedCategoryComplex {
            dimensions: vec![1, 2, 1],
            lo_degree: 0,
            differentials: vec![vec![vec![0, 1]], vec![vec![1], vec![0]]],
        };
        assert!(cx2.is_chain_complex(), "d1 * d2 = 0");
    }
    /// Test SixFunctorComputations: pullback and pushforward.
    #[test]
    fn test_six_functor_computations() {
        let six = SixFunctorComputations {
            source_points: 3,
            target_points: 2,
            f_map: vec![0, 0, 1],
        };
        let pb = six.pullback(&[3.0, 5.0]);
        assert_eq!(pb, vec![3.0, 3.0, 5.0], "pullback");
        let pf = six.pushforward(&[1.0, 2.0, 4.0]);
        assert_eq!(pf, vec![3.0, 4.0], "pushforward");
        let tensor = six.tensor(&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]);
        assert_eq!(tensor, vec![4.0, 10.0, 18.0], "tensor product");
        let hom = six.internal_hom(&[2.0, 4.0, 0.0], &[6.0, 8.0, 9.0]);
        assert_eq!(hom, vec![3.0, 2.0, 0.0], "internal hom");
    }
    /// Test that `build_category_theory_ext_env` builds without error.
    #[test]
    fn test_build_env() {
        let mut env = Environment::new();
        let helpers: &[(&str, Expr)] = &[
            (
                "Eq",
                pi(
                    BinderInfo::Implicit,
                    "_",
                    type0(),
                    pi(
                        BinderInfo::Default,
                        "_",
                        bvar(0),
                        pi(BinderInfo::Default, "_", bvar(1), prop()),
                    ),
                ),
            ),
            ("And", arrow(prop(), arrow(prop(), prop()))),
            ("Or", arrow(prop(), arrow(prop(), prop()))),
            ("Iff", arrow(prop(), arrow(prop(), prop()))),
            (
                "Exists",
                pi(
                    BinderInfo::Implicit,
                    "a",
                    type0(),
                    arrow(arrow(bvar(0), prop()), prop()),
                ),
            ),
            ("Iso", arrow(type0(), arrow(type0(), type0()))),
            ("Set", type0()),
            (
                "Functor",
                arrow(category_ty(), arrow(category_ty(), type0())),
            ),
            (
                "Compose",
                arrow(nat_trans_ty(), arrow(nat_trans_ty(), nat_trans_ty())),
            ),
            ("Id", nat_trans_ty()),
            ("IsFullyFaithful", arrow(nat_trans_ty(), prop())),
            ("Category", arrow(type0(), prop())),
        ];
        for (name, ty) in helpers {
            let _ = env.add(Declaration::Axiom {
                name: Name::str(*name),
                univ_params: vec![],
                ty: ty.clone(),
            });
        }
        let result = build_category_theory_ext_env(&mut env);
        assert!(
            result.is_ok(),
            "build_category_theory_ext_env failed: {:?}",
            result
        );
    }
}
