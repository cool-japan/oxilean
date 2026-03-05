//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
#![allow(clippy::items_after_test_module)]

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    DisplayedCategory, ObsEq, ObsEqJustification, ObsEqProof, QuotientType, Setoid, TwoLevelType,
    UIPStatus,
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
pub fn cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}
pub(super) fn prop() -> Expr {
    Expr::Sort(Level::zero())
}
pub(super) fn type0() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
pub fn type1() -> Expr {
    Expr::Sort(Level::succ(Level::succ(Level::zero())))
}
pub(super) fn pi(bi: BinderInfo, name: &str, dom: Expr, body: Expr) -> Expr {
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
/// `Setoid : Type 1` — the type of setoids (a type with a proof-irrelevant equivalence relation).
///
/// A setoid is a pair `(A, ~)` where `A : Type` and `~ : A → A → Prop` is an
/// equivalence relation. In the setoid model, every "type" is really a setoid.
pub fn setoid_ty() -> Expr {
    type1()
}
/// `Setoid.mk : ∀ (A : Type) (R : A → A → Prop), IsEquivRel R → Setoid`
///
/// Constructor for setoids: pair a type with an equivalence relation.
/// The `IsEquivRel R` witness carries reflexivity, symmetry, and transitivity proofs.
pub fn setoid_mk_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        pi(
            BinderInfo::Default,
            "R",
            arrow(bvar(0), arrow(bvar(1), prop())),
            arrow(app(cst("IsEquivRel"), bvar(0)), cst("Setoid")),
        ),
    )
}
/// `Setoid.carrier : Setoid → Type`
///
/// Project the underlying carrier type of a setoid.
pub fn setoid_carrier_ty() -> Expr {
    arrow(cst("Setoid"), type0())
}
/// `Setoid.rel : ∀ (s : Setoid) (a b : s.carrier), Prop`
///
/// Project the equivalence relation of a setoid.
pub fn setoid_rel_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "s",
        cst("Setoid"),
        arrow(
            app(cst("Setoid.carrier"), bvar(0)),
            arrow(app(cst("Setoid.carrier"), bvar(1)), prop()),
        ),
    )
}
/// `IsEquivRel : ∀ {A : Type} (R : A → A → Prop), Prop`
///
/// The predicate that R is an equivalence relation on A:
/// reflexive, symmetric, and transitive.
pub fn is_equiv_rel_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(arrow(bvar(0), arrow(bvar(1), prop())), prop()),
    )
}
/// `SetoidMorphism : Setoid → Setoid → Type`
///
/// A setoid morphism (setoid-respecting function) from setoid S to setoid T:
/// a function `f : S.carrier → T.carrier` that preserves the equivalence relation.
pub fn setoid_morphism_ty() -> Expr {
    arrow(cst("Setoid"), arrow(cst("Setoid"), type0()))
}
/// `SetoidIso : Setoid → Setoid → Type`
///
/// A setoid isomorphism: a pair of inverse setoid morphisms.
pub fn setoid_iso_ty() -> Expr {
    arrow(cst("Setoid"), arrow(cst("Setoid"), type0()))
}
/// `IsProp : ∀ (A : Type), Type`
///
/// A type is proof-irrelevant (a proposition / h-proposition) if
/// any two elements are equal: `IsProp A = ∀ (a b : A), a = b`.
/// This is a fundamental concept of both HoTT and OTT.
pub fn is_prop_ty() -> Expr {
    arrow(type0(), type0())
}
/// `PropSquash : Type → Prop`
///
/// The propositional truncation / squash type `‖A‖ = ∥A∥`:
/// collapses all elements of A into a proof-irrelevant proposition.
/// Key to the setoid model: `‖A‖` is used to hide the computational content.
pub fn prop_squash_ty() -> Expr {
    arrow(type0(), prop())
}
/// `PropSquash.intro : ∀ {A : Type}, A → ‖A‖`
///
/// Introduction rule: any element of A gives an element of ‖A‖.
pub fn prop_squash_intro_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(bvar(0), app(cst("PropSquash"), bvar(0))),
    )
}
/// `PropSquash.elim : ∀ {A : Type} (P : Prop) (h : IsProp P) (f : A → P), ‖A‖ → P`
///
/// Elimination rule: to map out of ‖A‖ into a proposition P, it suffices to
/// provide a function A → P.
pub fn prop_squash_elim_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        pi(
            BinderInfo::Default,
            "P",
            prop(),
            arrow(
                app(cst("IsProp"), bvar(0)),
                arrow(
                    arrow(bvar(1), bvar(1)),
                    arrow(app(cst("PropSquash"), bvar(2)), bvar(1)),
                ),
            ),
        ),
    )
}
/// `PropIrrelevance : ∀ (P : Prop) (h k : P), h = k`
///
/// Proof irrelevance axiom: all proofs of a proposition P are equal.
/// This holds in the setoid model and in OTT.
pub fn prop_irrelevance_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        prop(),
        pi(
            BinderInfo::Default,
            "h",
            bvar(0),
            pi(
                BinderInfo::Default,
                "k",
                bvar(1),
                app3(cst("Eq"), bvar(2), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// `ObsEq : ∀ (A : Type) (a b : A), Prop`
///
/// Observational equality: `a ≅_A b` is defined by structural induction on A.
/// - For `Nat`: `ObsEq Nat n m ↔ n = m` (standard equality)
/// - For `A → B`: `ObsEq (A→B) f g ↔ ∀ a, ObsEq B (f a) (g a)` (extensional)
/// - For `Σ A B`: pointwise observational equality
/// Unlike identity types, observational equality is *definitionally proof-irrelevant*.
pub fn obs_eq_ty() -> Expr {
    impl_pi("A", type0(), arrow(bvar(0), arrow(bvar(1), prop())))
}
/// `ObsEq.refl : ∀ {A : Type} (a : A), ObsEq A a a`
///
/// Reflexivity of observational equality.
pub fn obs_eq_refl_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        pi(
            BinderInfo::Default,
            "a",
            bvar(0),
            app3(cst("ObsEq"), bvar(1), bvar(0), bvar(0)),
        ),
    )
}
/// `ObsEq.sym : ∀ {A : Type} {a b : A}, ObsEq A a b → ObsEq A b a`
///
/// Symmetry of observational equality.
pub fn obs_eq_sym_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "a",
            bvar(0),
            impl_pi(
                "b",
                bvar(1),
                arrow(
                    app3(cst("ObsEq"), bvar(2), bvar(1), bvar(0)),
                    app3(cst("ObsEq"), bvar(3), bvar(1), bvar(2)),
                ),
            ),
        ),
    )
}
/// `ObsEq.trans : ∀ {A : Type} {a b c : A}, ObsEq A a b → ObsEq A b c → ObsEq A a c`
///
/// Transitivity of observational equality.
pub fn obs_eq_trans_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(
            bvar(0),
            arrow(
                bvar(1),
                arrow(
                    bvar(2),
                    arrow(
                        app3(cst("ObsEq"), bvar(3), bvar(2), bvar(1)),
                        arrow(
                            app3(cst("ObsEq"), bvar(4), bvar(2), bvar(1)),
                            app3(cst("ObsEq"), bvar(5), bvar(4), bvar(2)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `ObsEq.funext : ∀ {A B : Type} {f g : A → B}, (∀ x, ObsEq B (f x) (g x)) → ObsEq (A→B) f g`
///
/// Function extensionality for observational equality:
/// two functions are observationally equal iff they agree on all inputs.
/// This holds *by definition* in OTT — it is not an axiom but a consequence of
/// the type-directed definition of equality.
pub fn obs_eq_funext_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "B",
            type0(),
            impl_pi(
                "f",
                arrow(bvar(1), bvar(0)),
                impl_pi(
                    "g",
                    arrow(bvar(2), bvar(1)),
                    arrow(
                        pi(
                            BinderInfo::Default,
                            "x",
                            bvar(3),
                            app3(
                                cst("ObsEq"),
                                bvar(3),
                                app(bvar(2), bvar(0)),
                                app(bvar(1), bvar(0)),
                            ),
                        ),
                        app3(cst("ObsEq"), arrow(bvar(4), bvar(3)), bvar(1), bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// `Coerce : ∀ {A B : Type}, ObsEq Type A B → A → B`
///
/// Coercion: observational equality of types allows transport.
/// In OTT, if two types are observationally equal they must be isomorphic.
pub fn coerce_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "B",
            type0(),
            arrow(
                app3(cst("ObsEq"), type0(), bvar(1), bvar(0)),
                arrow(bvar(1), bvar(0)),
            ),
        ),
    )
}
/// `Quot : ∀ {A : Type} (R : A → A → Prop), Type`
///
/// The quotient of A by an (arbitrary) relation R.
/// In OTT, quotients are well-behaved because equality is proof-irrelevant.
pub fn quot_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(arrow(bvar(0), arrow(bvar(1), prop())), type0()),
    )
}
/// `Quot.mk : ∀ {A : Type} {R : A → A → Prop}, A → Quot R`
///
/// Introduction rule: embed an element into the quotient type.
pub fn quot_mk_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "R",
            arrow(bvar(0), arrow(bvar(1), prop())),
            arrow(bvar(1), app2(cst("Quot"), bvar(1), bvar(0))),
        ),
    )
}
/// `Quot.lift : ∀ {A B : Type} {R : A → A → Prop} (f : A → B),
///     (∀ a b, R a b → f a = f b) → Quot R → B`
///
/// Lift a function f : A → B to the quotient, given that f respects R.
/// This is the main elimination principle for quotients.
pub fn quot_lift_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "B",
            type0(),
            impl_pi(
                "R",
                arrow(bvar(1), arrow(bvar(2), prop())),
                pi(
                    BinderInfo::Default,
                    "f",
                    arrow(bvar(2), bvar(1)),
                    arrow(
                        pi(
                            BinderInfo::Default,
                            "a",
                            bvar(3),
                            pi(
                                BinderInfo::Default,
                                "b",
                                bvar(4),
                                arrow(
                                    app2(bvar(2), bvar(1), bvar(0)),
                                    app3(
                                        cst("Eq"),
                                        bvar(3),
                                        app(bvar(4), bvar(1)),
                                        app(bvar(4), bvar(0)),
                                    ),
                                ),
                            ),
                        ),
                        arrow(app2(cst("Quot"), bvar(3), bvar(2)), bvar(2)),
                    ),
                ),
            ),
        ),
    )
}
/// `Quot.sound : ∀ {A : Type} {R : A → A → Prop} {a b : A}, R a b → Quot.mk a = Quot.mk b`
///
/// Soundness: related elements have equal images in the quotient.
pub fn quot_sound_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "R",
            arrow(bvar(0), arrow(bvar(1), prop())),
            impl_pi(
                "a",
                bvar(1),
                impl_pi(
                    "b",
                    bvar(2),
                    arrow(
                        app2(bvar(2), bvar(1), bvar(0)),
                        app3(
                            cst("Eq"),
                            app2(cst("Quot"), bvar(3), bvar(2)),
                            app3(cst("Quot.mk"), bvar(3), bvar(2), bvar(1)),
                            app3(cst("Quot.mk"), bvar(4), bvar(3), bvar(0)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `UIP : ∀ {A : Type} {a b : A} (p q : a = b), p = q`
///
/// Uniqueness of Identity Proofs (Streicher's K axiom equivalent):
/// all proofs of the same equality are themselves equal.
/// This holds in the setoid model and is consistent with classical logic
/// but *inconsistent* with univalence.
pub fn uip_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "a",
            bvar(0),
            impl_pi(
                "b",
                bvar(1),
                pi(
                    BinderInfo::Default,
                    "p",
                    app3(cst("Eq"), bvar(2), bvar(1), bvar(0)),
                    pi(
                        BinderInfo::Default,
                        "q",
                        app3(cst("Eq"), bvar(3), bvar(2), bvar(1)),
                        app3(
                            cst("Eq"),
                            app3(cst("Eq"), bvar(4), bvar(3), bvar(2)),
                            bvar(1),
                            bvar(0),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `AxiomK : ∀ {A : Type} {a : A} (P : a = a → Prop), P refl → ∀ (p : a = a), P p`
///
/// Streicher's Axiom K: the only proof of `a = a` is `refl`.
/// Equivalent to UIP. Consistent in the setoid model.
pub fn axiom_k_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "a",
            bvar(0),
            pi(
                BinderInfo::Default,
                "P",
                arrow(app3(cst("Eq"), bvar(1), bvar(0), bvar(0)), prop()),
                arrow(
                    app(bvar(0), app2(cst("Eq.refl"), bvar(1), bvar(0))),
                    pi(
                        BinderInfo::Default,
                        "p",
                        app3(cst("Eq"), bvar(2), bvar(1), bvar(1)),
                        app(bvar(2), bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// `Decidable : ∀ (P : Prop), Type`
///
/// The type of decidable propositions: `Decidable P = P ⊕ ¬P`.
/// In the setoid model and OTT, decidability plays a role in computational content.
pub fn decidable_ty() -> Expr {
    arrow(prop(), type0())
}
/// `DecEq : ∀ (A : Type), Type`
///
/// Types with decidable equality — required for the setoid model to be effective.
/// `DecEq A = ∀ (a b : A), Decidable (a = b)`.
pub fn dec_eq_ty() -> Expr {
    arrow(type0(), type0())
}
/// `FibrantType : Type 1` — marker for fibrant (homotopy-coherent) types.
///
/// In two-level type theory (Altenkirch-Capriotti-Kraus-Scherer), the universe
/// is stratified into a fibrant level (types with an interpretation in ∞-groupoids)
/// and a strict/exact level (types with uniqueness of identity proofs).
pub fn fibrant_type_ty() -> Expr {
    type1()
}
/// `StrictType : Type 1` — marker for strict (exact) types in 2LTT.
///
/// Strict types have definitional UIP: all proofs of equality are definitionally
/// equal. They form the "outer layer" of 2LTT.
pub fn strict_type_ty() -> Expr {
    type1()
}
/// `Lift : StrictType → FibrantType`
///
/// Coerce a strict type into a fibrant type (inclusion of the strict universe).
/// In 2LTT, every strict type has an associated fibrant counterpart.
pub fn lift_ty() -> Expr {
    arrow(cst("StrictType"), cst("FibrantType"))
}
/// `FibrantUniverse : FibrantType`
///
/// The fibrant universe — the type of fibrant types.
/// This is the universe used by homotopy type theory (HoTT) within 2LTT.
pub fn fibrant_universe_ty() -> Expr {
    cst("FibrantType")
}
/// `ExactEquality : ∀ {A : StrictType} (a b : A), Prop`
///
/// The strict/exact equality in 2LTT — definitionally proof-irrelevant
/// because strict types satisfy UIP. Distinct from propositional identity.
pub fn exact_equality_ty() -> Expr {
    impl_pi(
        "A",
        cst("StrictType"),
        arrow(
            app(cst("Lift"), bvar(0)),
            arrow(app(cst("Lift"), bvar(1)), prop()),
        ),
    )
}
/// `2LTTCoherenceAxiom : ∀ (A : FibrantType), ExactEq (Lift (forget A)) A`
///
/// The coherence axiom of 2LTT: every fibrant type is "essentially strict"
/// in the sense that we can round-trip through the strict universe.
pub fn two_ltt_coherence_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        cst("FibrantType"),
        app3(
            cst("ExactEquality"),
            app(cst("Lift"), bvar(0)),
            bvar(0),
            bvar(0),
        ),
    )
}
/// `DisplayedCat : ∀ (C : Category), Type 1`
///
/// A displayed category (Ahrens-Lumsdaine) over C is the categorical analogue
/// of a dependent type over C. It assigns to each object/morphism of C
/// a set of "lying over" objects/morphisms.
pub fn displayed_cat_ty() -> Expr {
    arrow(cst("Category"), type1())
}
/// `TotalCat : ∀ {C : Category} (D : DisplayedCat C), Category`
///
/// The total category of a displayed category: objects are pairs (c, d)
/// with d lying over c.
pub fn total_cat_ty() -> Expr {
    impl_pi(
        "C",
        cst("Category"),
        arrow(app(cst("DisplayedCat"), bvar(0)), cst("Category")),
    )
}
/// `Section : ∀ {C : Category} (D : DisplayedCat C), Type`
///
/// A section of a displayed category: for every object c of C, a choice of
/// displayed object over c, functorially in c. Sections model dependent functions.
pub fn section_ty() -> Expr {
    impl_pi(
        "C",
        cst("Category"),
        arrow(app(cst("DisplayedCat"), bvar(0)), type0()),
    )
}
/// `FiberedFunctor : ∀ {C D : Category} (F : Functor C D)
///     (E : DisplayedCat C) (G : DisplayedCat D), Type`
///
/// A functor between displayed categories over a base functor F.
pub fn fibered_functor_ty() -> Expr {
    impl_pi(
        "C",
        cst("Category"),
        impl_pi(
            "D",
            cst("Category"),
            arrow(
                app2(cst("Functor"), bvar(1), bvar(0)),
                arrow(
                    app(cst("DisplayedCat"), bvar(1)),
                    arrow(app(cst("DisplayedCat"), bvar(1)), type0()),
                ),
            ),
        ),
    )
}
/// `Category : Type 1` — a (small) category.
pub fn category_ty() -> Expr {
    type1()
}
/// `Functor : Category → Category → Type`
pub fn functor_ty() -> Expr {
    arrow(cst("Category"), arrow(cst("Category"), type0()))
}
/// `Modality : Type 1` — a (left exact) modality / modal operator on types.
///
/// A modality ○ on a type theory is an endofunctor on types with:
/// - A unit `η_A : A → ○A`
/// - ○ preserves dependent products
/// - The modalized unit is an equivalence: `η_{○A} : ○A ≃ ○(○A)` is an iso.
pub fn modality_ty() -> Expr {
    type1()
}
/// `Modal : Modality → Type → Type`
///
/// Apply a modality to a type: `○ A` is the modalization of A.
pub fn modal_ty() -> Expr {
    arrow(cst("Modality"), arrow(type0(), type0()))
}
/// `ModalUnit : ∀ (m : Modality) (A : Type), A → Modal m A`
///
/// The unit of the modality: every element of A gives a modal element.
pub fn modal_unit_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "m",
        cst("Modality"),
        pi(
            BinderInfo::Default,
            "A",
            type0(),
            arrow(bvar(0), app2(cst("Modal"), bvar(1), bvar(0))),
        ),
    )
}
/// `ModalElim : ∀ (m : Modality) (A : Type) (P : Modal m A → Type)
///     (h : ∀ a : A, P (unit m A a)) (x : Modal m A), P x`
///
/// Elimination principle for modal types: given a predicate P on Modal m A,
/// it suffices to prove P for all `unit a`.
#[allow(clippy::too_many_arguments)]
pub fn modal_elim_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "m",
        cst("Modality"),
        pi(
            BinderInfo::Default,
            "A",
            type0(),
            pi(
                BinderInfo::Default,
                "P",
                arrow(app2(cst("Modal"), bvar(1), bvar(0)), type0()),
                arrow(
                    pi(
                        BinderInfo::Default,
                        "a",
                        bvar(1),
                        app(bvar(1), app3(cst("ModalUnit"), bvar(3), bvar(2), bvar(0))),
                    ),
                    pi(
                        BinderInfo::Default,
                        "x",
                        app2(cst("Modal"), bvar(2), bvar(1)),
                        app(bvar(2), bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// `IsModal : ∀ (m : Modality) (A : Type), Prop`
///
/// A type A is m-modal if the modal unit `η_A : A → Modal m A` is an equivalence.
pub fn is_modal_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "m",
        cst("Modality"),
        arrow(type0(), prop()),
    )
}
/// `ModalSeperation : ∀ (m : Modality) (A B : Type),
///     IsModal m B → (A → B) → Modal m A → B`
///
/// Separation / modal adjunction: functions into modal types factor through Modal m.
pub fn modal_separation_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "m",
        cst("Modality"),
        pi(
            BinderInfo::Default,
            "A",
            type0(),
            pi(
                BinderInfo::Default,
                "B",
                type0(),
                arrow(
                    app2(cst("IsModal"), bvar(2), bvar(0)),
                    arrow(
                        arrow(bvar(2), bvar(1)),
                        arrow(app2(cst("Modal"), bvar(3), bvar(2)), bvar(1)),
                    ),
                ),
            ),
        ),
    )
}
/// `SetoidCategory : Type 1`
///
/// The category of setoids: objects are setoids, morphisms are
/// setoid morphisms (equivalence-preserving functions), and
/// morphism equality is pointwise observational equality.
pub fn setoid_category_ty() -> Expr {
    type1()
}
/// `SetoidFunctor : Setoid → Setoid → Type`
///
/// A functor in the category of setoids: a setoid morphism.
pub fn setoid_functor_ty() -> Expr {
    arrow(cst("Setoid"), arrow(cst("Setoid"), type0()))
}
/// `SetoidComprehension : ∀ (s : Setoid) (P : s.carrier → Prop), Setoid`
///
/// Setoid comprehension: given a setoid s and a predicate P on its carrier,
/// form the sub-setoid `{ a ∈ s | P a }` with the induced equivalence relation.
pub fn setoid_comprehension_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "s",
        cst("Setoid"),
        arrow(
            arrow(app(cst("Setoid.carrier"), bvar(0)), prop()),
            cst("Setoid"),
        ),
    )
}
/// `SetoidProduct : Setoid → Setoid → Setoid`
///
/// The product of two setoids: carrier is the cartesian product,
/// equivalence is componentwise.
pub fn setoid_product_ty() -> Expr {
    arrow(cst("Setoid"), arrow(cst("Setoid"), cst("Setoid")))
}
/// `SetoidCoproduct : Setoid → Setoid → Setoid`
///
/// The coproduct (disjoint union) of two setoids.
pub fn setoid_coproduct_ty() -> Expr {
    arrow(cst("Setoid"), arrow(cst("Setoid"), cst("Setoid")))
}
/// `SetoidExponential : Setoid → Setoid → Setoid`
///
/// The internal hom / exponential setoid: morphisms between setoids,
/// with pointwise equivalence.
pub fn setoid_exponential_ty() -> Expr {
    arrow(cst("Setoid"), arrow(cst("Setoid"), cst("Setoid")))
}
/// Register all observational type theory and setoid axioms into the given environment.
pub fn build_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("Setoid", setoid_ty()),
        ("Setoid.mk", setoid_mk_ty()),
        ("Setoid.carrier", setoid_carrier_ty()),
        ("Setoid.rel", setoid_rel_ty()),
        ("IsEquivRel", is_equiv_rel_ty()),
        ("SetoidMorphism", setoid_morphism_ty()),
        ("SetoidIso", setoid_iso_ty()),
        ("IsProp", is_prop_ty()),
        ("PropSquash", prop_squash_ty()),
        ("PropSquash.intro", prop_squash_intro_ty()),
        ("PropSquash.elim", prop_squash_elim_ty()),
        ("PropIrrelevance", prop_irrelevance_ty()),
        ("ObsEq", obs_eq_ty()),
        ("ObsEq.refl", obs_eq_refl_ty()),
        ("ObsEq.sym", obs_eq_sym_ty()),
        ("ObsEq.trans", obs_eq_trans_ty()),
        ("ObsEq.funext", obs_eq_funext_ty()),
        ("Coerce", coerce_ty()),
        ("Quot", quot_ty()),
        ("Quot.mk", quot_mk_ty()),
        ("Quot.lift", quot_lift_ty()),
        ("Quot.sound", quot_sound_ty()),
        ("UIP", uip_ty()),
        ("AxiomK", axiom_k_ty()),
        ("Decidable", decidable_ty()),
        ("DecEq", dec_eq_ty()),
        ("FibrantType", fibrant_type_ty()),
        ("StrictType", strict_type_ty()),
        ("Lift", lift_ty()),
        ("FibrantUniverse", fibrant_universe_ty()),
        ("ExactEquality", exact_equality_ty()),
        ("TwoLTTCoherence", two_ltt_coherence_ty()),
        ("Category", category_ty()),
        ("Functor", functor_ty()),
        ("DisplayedCat", displayed_cat_ty()),
        ("TotalCat", total_cat_ty()),
        ("Section", section_ty()),
        ("FiberedFunctor", fibered_functor_ty()),
        ("Modality", modality_ty()),
        ("Modal", modal_ty()),
        ("ModalUnit", modal_unit_ty()),
        ("ModalElim", modal_elim_ty()),
        ("IsModal", is_modal_ty()),
        ("ModalSeparation", modal_separation_ty()),
        ("SetoidCategory", setoid_category_ty()),
        ("SetoidFunctor", setoid_functor_ty()),
        ("SetoidComprehension", setoid_comprehension_ty()),
        ("SetoidProduct", setoid_product_ty()),
        ("SetoidCoproduct", setoid_coproduct_ty()),
        ("SetoidExponential", setoid_exponential_ty()),
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
#[cfg(test)]
mod tests {
    use super::*;
    use oxilean_kernel::Environment;
    #[test]
    fn test_setoid_axioms_registered() {
        let mut env = Environment::new();
        build_env(&mut env);
        assert!(
            env.get(&Name::str("Setoid")).is_some(),
            "Setoid must be registered"
        );
        assert!(
            env.get(&Name::str("Setoid.mk")).is_some(),
            "Setoid.mk must be registered"
        );
        assert!(
            env.get(&Name::str("IsEquivRel")).is_some(),
            "IsEquivRel must be registered"
        );
    }
    #[test]
    fn test_obs_eq_axioms_registered() {
        let mut env = Environment::new();
        build_env(&mut env);
        assert!(
            env.get(&Name::str("ObsEq")).is_some(),
            "ObsEq must be registered"
        );
        assert!(
            env.get(&Name::str("ObsEq.refl")).is_some(),
            "ObsEq.refl must be registered"
        );
        assert!(
            env.get(&Name::str("ObsEq.funext")).is_some(),
            "ObsEq.funext must be registered"
        );
        assert!(
            env.get(&Name::str("UIP")).is_some(),
            "UIP must be registered"
        );
    }
    #[test]
    fn test_quotient_axioms_registered() {
        let mut env = Environment::new();
        build_env(&mut env);
        assert!(
            env.get(&Name::str("Quot")).is_some(),
            "Quot must be registered"
        );
        assert!(
            env.get(&Name::str("Quot.mk")).is_some(),
            "Quot.mk must be registered"
        );
        assert!(
            env.get(&Name::str("Quot.lift")).is_some(),
            "Quot.lift must be registered"
        );
        assert!(
            env.get(&Name::str("Quot.sound")).is_some(),
            "Quot.sound must be registered"
        );
    }
    #[test]
    fn test_two_ltt_axioms_registered() {
        let mut env = Environment::new();
        build_env(&mut env);
        assert!(
            env.get(&Name::str("FibrantType")).is_some(),
            "FibrantType must be registered"
        );
        assert!(
            env.get(&Name::str("StrictType")).is_some(),
            "StrictType must be registered"
        );
        assert!(
            env.get(&Name::str("ExactEquality")).is_some(),
            "ExactEquality must be registered"
        );
    }
    #[test]
    fn test_setoid_rust_level() {
        let s = Setoid::discrete("Nat");
        assert!(s.is_valid(), "discrete setoid is valid");
        let t = Setoid::discrete("Bool");
        let prod = s.product(&t);
        assert!(prod.is_valid(), "product of valid setoids is valid");
        assert!(prod.carrier.contains("Nat") && prod.carrier.contains("Bool"));
        let exp = s.exponential(&t);
        assert!(exp.is_valid(), "exponential of valid setoids is valid");
    }
    #[test]
    fn test_obs_eq_rust_level() {
        let refl = ObsEqProof::refl("Nat", "n");
        assert_eq!(refl.lhs, refl.rhs, "refl proof has equal sides");
        let p = ObsEqProof {
            type_name: "Nat".into(),
            lhs: "a".into(),
            rhs: "b".into(),
            justification: ObsEqJustification::Refl,
        };
        let sym_p = p.clone().sym();
        assert_eq!(sym_p.lhs, "b");
        assert_eq!(sym_p.rhs, "a");
        let q = ObsEqProof {
            type_name: "Nat".into(),
            lhs: "b".into(),
            rhs: "c".into(),
            justification: ObsEqJustification::Refl,
        };
        let trans = p
            .trans(q)
            .expect("trans should succeed when endpoints match");
        assert_eq!(trans.lhs, "a");
        assert_eq!(trans.rhs, "c");
    }
    #[test]
    fn test_quotient_rust_level() {
        let q = QuotientType::new("Int", "EvenEq");
        assert!(q.name().contains("Int") && q.name().contains("EvenEq"));
        let elem = q.mk("3");
        assert!(elem.contains("3") && elem.contains("EvenEq"));
    }
    #[test]
    fn test_two_level_type() {
        let strict = TwoLevelType::strict("StrictNat");
        assert_eq!(strict.uip_status(), UIPStatus::Definitional);
        assert!(!strict.admits_univalence());
        let fibrant = TwoLevelType::fibrant("FibrantNat");
        assert_eq!(fibrant.uip_status(), UIPStatus::Unknown);
        assert!(fibrant.admits_univalence());
        let lifted = strict.lift();
        assert_eq!(lifted.uip_status(), UIPStatus::Propositional);
        assert!(lifted.name().contains("StrictNat"));
    }
    #[test]
    fn test_displayed_category_rust_level() {
        let disp = DisplayedCategory::new("Type", "FamilyOver", 10);
        assert_eq!(disp.slice_count(), 10);
        assert!(disp.total_category_name().contains("FamilyOver"));
        assert!(disp.section_name().contains("FamilyOver"));
    }
}
pub(super) fn ott_ext_cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}
pub(super) fn ott_ext_app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
pub(super) fn ott_ext_app2(f: Expr, a: Expr, b: Expr) -> Expr {
    ott_ext_app(ott_ext_app(f, a), b)
}
pub(super) fn ott_ext_app3(f: Expr, a: Expr, b: Expr, c: Expr) -> Expr {
    ott_ext_app(ott_ext_app2(f, a, b), c)
}
pub(super) fn ott_ext_app4(f: Expr, a: Expr, b: Expr, c: Expr, d: Expr) -> Expr {
    ott_ext_app(ott_ext_app3(f, a, b, c), d)
}
pub(super) fn ott_ext_pi(name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str(name),
        Box::new(dom),
        Box::new(body),
    )
}
pub(super) fn ott_ext_ipi(name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str(name),
        Box::new(dom),
        Box::new(body),
    )
}
pub(super) fn ott_ext_arrow(a: Expr, b: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::Anonymous,
        Box::new(a),
        Box::new(b),
    )
}
pub(super) fn ott_ext_prop() -> Expr {
    Expr::Sort(Level::zero())
}
pub(super) fn ott_ext_type0() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
pub fn ott_ext_type1() -> Expr {
    Expr::Sort(Level::succ(Level::succ(Level::zero())))
}
pub(super) fn ott_ext_bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
/// `HEq : ∀ (A : Type) (a : A) (B : Type) (b : B), Prop`
///
/// Heterogeneous (John-Major) equality: `a ≅ b` asserts that `a` and `b`
/// are equal even if they have different types `A` and `B`.
/// This is used in OTT to state equality of elements at distinct but
/// observationally-equal types without requiring explicit coercions.
pub fn heq_ty() -> Expr {
    ott_ext_ipi(
        "A",
        ott_ext_type0(),
        ott_ext_pi(
            "a",
            ott_ext_bvar(0),
            ott_ext_ipi(
                "B",
                ott_ext_type0(),
                ott_ext_arrow(ott_ext_bvar(0), ott_ext_prop()),
            ),
        ),
    )
}
/// `HEq.refl : ∀ {A : Type} (a : A), HEq a a`
///
/// Reflexivity of heterogeneous equality: every element is heterogeneously
/// equal to itself.
pub fn heq_refl_ty() -> Expr {
    ott_ext_ipi(
        "A",
        ott_ext_type0(),
        ott_ext_pi(
            "a",
            ott_ext_bvar(0),
            ott_ext_app4(
                ott_ext_cst("HEq"),
                ott_ext_bvar(1),
                ott_ext_bvar(0),
                ott_ext_bvar(1),
                ott_ext_bvar(0),
            ),
        ),
    )
}
/// `HEq.sym : ∀ {A B : Type} {a : A} {b : B}, HEq a b → HEq b a`
///
/// Symmetry of heterogeneous equality.
pub fn heq_sym_ty() -> Expr {
    ott_ext_ipi(
        "A",
        ott_ext_type0(),
        ott_ext_ipi(
            "B",
            ott_ext_type0(),
            ott_ext_ipi(
                "a",
                ott_ext_bvar(1),
                ott_ext_ipi(
                    "b",
                    ott_ext_bvar(1),
                    ott_ext_arrow(
                        ott_ext_app4(
                            ott_ext_cst("HEq"),
                            ott_ext_bvar(3),
                            ott_ext_bvar(1),
                            ott_ext_bvar(2),
                            ott_ext_bvar(0),
                        ),
                        ott_ext_app4(
                            ott_ext_cst("HEq"),
                            ott_ext_bvar(2),
                            ott_ext_bvar(0),
                            ott_ext_bvar(3),
                            ott_ext_bvar(1),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `HEq.trans : ∀ {A B C : Type} {a : A} {b : B} {c : C}, HEq a b → HEq b c → HEq a c`
///
/// Transitivity of heterogeneous equality.
pub fn heq_trans_ty() -> Expr {
    ott_ext_ipi(
        "A",
        ott_ext_type0(),
        ott_ext_ipi(
            "B",
            ott_ext_type0(),
            ott_ext_ipi(
                "C",
                ott_ext_type0(),
                ott_ext_arrow(
                    ott_ext_bvar(2),
                    ott_ext_arrow(
                        ott_ext_bvar(2),
                        ott_ext_arrow(
                            ott_ext_bvar(2),
                            ott_ext_arrow(
                                ott_ext_app4(
                                    ott_ext_cst("HEq"),
                                    ott_ext_bvar(5),
                                    ott_ext_bvar(2),
                                    ott_ext_bvar(4),
                                    ott_ext_bvar(1),
                                ),
                                ott_ext_arrow(
                                    ott_ext_app4(
                                        ott_ext_cst("HEq"),
                                        ott_ext_bvar(5),
                                        ott_ext_bvar(2),
                                        ott_ext_bvar(4),
                                        ott_ext_bvar(1),
                                    ),
                                    ott_ext_app4(
                                        ott_ext_cst("HEq"),
                                        ott_ext_bvar(7),
                                        ott_ext_bvar(4),
                                        ott_ext_bvar(5),
                                        ott_ext_bvar(2),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `HEq.fromEq : ∀ {A : Type} {a b : A}, a = b → HEq a b`
///
/// Every homogeneous equality implies heterogeneous equality.
pub fn heq_from_eq_ty() -> Expr {
    ott_ext_ipi(
        "A",
        ott_ext_type0(),
        ott_ext_ipi(
            "a",
            ott_ext_bvar(0),
            ott_ext_ipi(
                "b",
                ott_ext_bvar(1),
                ott_ext_arrow(
                    ott_ext_app3(
                        ott_ext_cst("Eq"),
                        ott_ext_bvar(2),
                        ott_ext_bvar(1),
                        ott_ext_bvar(0),
                    ),
                    ott_ext_app4(
                        ott_ext_cst("HEq"),
                        ott_ext_bvar(3),
                        ott_ext_bvar(2),
                        ott_ext_bvar(3),
                        ott_ext_bvar(1),
                    ),
                ),
            ),
        ),
    )
}
/// `HEq.toEq : ∀ {A : Type} {a b : A}, HEq a b → a = b`
///
/// Heterogeneous equality at the same type implies homogeneous equality.
pub fn heq_to_eq_ty() -> Expr {
    ott_ext_ipi(
        "A",
        ott_ext_type0(),
        ott_ext_ipi(
            "a",
            ott_ext_bvar(0),
            ott_ext_ipi(
                "b",
                ott_ext_bvar(1),
                ott_ext_arrow(
                    ott_ext_app4(
                        ott_ext_cst("HEq"),
                        ott_ext_bvar(2),
                        ott_ext_bvar(1),
                        ott_ext_bvar(2),
                        ott_ext_bvar(0),
                    ),
                    ott_ext_app3(
                        ott_ext_cst("Eq"),
                        ott_ext_bvar(3),
                        ott_ext_bvar(2),
                        ott_ext_bvar(1),
                    ),
                ),
            ),
        ),
    )
}
/// `ObsEq.transport : ∀ {A B : Type} (e : ObsEq Type A B) (P : A → Prop), P → B → Prop`
///
/// Transport of predicates along observational equality of types.
/// Given evidence that `A` and `B` are observationally equal types and
/// a predicate `P` on `A`, one can "transport" `P` along `e` to `B`.
pub fn obs_eq_transport_ty() -> Expr {
    ott_ext_ipi(
        "A",
        ott_ext_type0(),
        ott_ext_ipi(
            "B",
            ott_ext_type0(),
            ott_ext_pi(
                "e",
                ott_ext_app3(
                    ott_ext_cst("ObsEq"),
                    ott_ext_type0(),
                    ott_ext_bvar(1),
                    ott_ext_bvar(0),
                ),
                ott_ext_pi(
                    "P",
                    ott_ext_arrow(ott_ext_bvar(1), ott_ext_prop()),
                    ott_ext_arrow(
                        ott_ext_app(ott_ext_cst("PropSquash"), ott_ext_bvar(0)),
                        ott_ext_arrow(ott_ext_bvar(1), ott_ext_prop()),
                    ),
                ),
            ),
        ),
    )
}
/// `CoerceCoherence : ∀ {A : Type} (e : ObsEq Type A A) (a : A), Coerce e a = a`
///
/// Coercion coherence: coercing along a reflexivity proof is the identity.
/// This ensures that coercions do not change computational behavior.
pub fn coerce_coherence_ty() -> Expr {
    ott_ext_ipi(
        "A",
        ott_ext_type0(),
        ott_ext_pi(
            "e",
            ott_ext_app3(
                ott_ext_cst("ObsEq"),
                ott_ext_type0(),
                ott_ext_bvar(0),
                ott_ext_bvar(0),
            ),
            ott_ext_pi(
                "a",
                ott_ext_bvar(1),
                ott_ext_app3(
                    ott_ext_cst("Eq"),
                    ott_ext_bvar(2),
                    ott_ext_app3(
                        ott_ext_cst("Coerce"),
                        ott_ext_bvar(2),
                        ott_ext_bvar(2),
                        ott_ext_bvar(0),
                    ),
                    ott_ext_bvar(0),
                ),
            ),
        ),
    )
}
/// `ObsEq.sigma_ext : ∀ {A : Type} {B : A → Type} {p q : Σ x : A, B x},
///     ObsEq A p.1 q.1 → ObsEq (B q.1) (coerce p.2) q.2 → ObsEq (Σ A B) p q`
///
/// Sigma type extensionality for OTT: two dependent pairs are observationally
/// equal iff their first components are equal and second components are equal
/// after transporting.
pub fn obs_eq_sigma_ext_ty() -> Expr {
    ott_ext_ipi(
        "A",
        ott_ext_type0(),
        ott_ext_ipi(
            "B",
            ott_ext_arrow(ott_ext_bvar(0), ott_ext_type0()),
            ott_ext_arrow(
                ott_ext_app(ott_ext_cst("Sigma"), ott_ext_bvar(1)),
                ott_ext_arrow(
                    ott_ext_app(ott_ext_cst("Sigma"), ott_ext_bvar(2)),
                    ott_ext_arrow(
                        ott_ext_prop(),
                        ott_ext_arrow(
                            ott_ext_prop(),
                            ott_ext_app3(
                                ott_ext_cst("ObsEq"),
                                ott_ext_app(ott_ext_cst("Sigma"), ott_ext_bvar(4)),
                                ott_ext_bvar(3),
                                ott_ext_bvar(2),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `ObsEq.nat_char : ∀ (n m : Nat), ObsEq Nat n m ↔ n = m`
///
/// Characterisation of observational equality on Nat: it coincides with
/// propositional equality. This is the base case of the type-directed
/// definition of ObsEq.
pub fn obs_eq_nat_char_ty() -> Expr {
    ott_ext_pi(
        "n",
        ott_ext_cst("Nat"),
        ott_ext_pi(
            "m",
            ott_ext_cst("Nat"),
            ott_ext_app2(
                ott_ext_cst("Iff"),
                ott_ext_app3(
                    ott_ext_cst("ObsEq"),
                    ott_ext_cst("Nat"),
                    ott_ext_bvar(1),
                    ott_ext_bvar(0),
                ),
                ott_ext_app3(
                    ott_ext_cst("Eq"),
                    ott_ext_cst("Nat"),
                    ott_ext_bvar(1),
                    ott_ext_bvar(0),
                ),
            ),
        ),
    )
}
/// `OTTPropTrunc : ∀ (A : Type), Type`
///
/// OTT propositional truncation type former: `‖A‖` or `Trunc₋₁ A`.
/// Unlike HoTT truncation, in OTT this is definitionally proof-irrelevant.
pub fn ott_prop_trunc_ty() -> Expr {
    ott_ext_arrow(ott_ext_type0(), ott_ext_type0())
}
/// `OTTPropTrunc.intro : ∀ {A : Type}, A → ‖A‖`
///
/// Introduction: any element witnesses the truncation.
pub fn ott_prop_trunc_intro_ty() -> Expr {
    ott_ext_ipi(
        "A",
        ott_ext_type0(),
        ott_ext_arrow(
            ott_ext_bvar(0),
            ott_ext_app(ott_ext_cst("OTTPropTrunc"), ott_ext_bvar(0)),
        ),
    )
}
/// `OTTPropTrunc.elim : ∀ {A : Type} (P : Prop) (f : A → P), ‖A‖ → P`
///
/// OTT truncation elimination: to prove a proposition from the truncation,
/// it suffices to have an A-valued proof (the truncation discards the witness).
pub fn ott_prop_trunc_elim_ty() -> Expr {
    ott_ext_ipi(
        "A",
        ott_ext_type0(),
        ott_ext_pi(
            "P",
            ott_ext_prop(),
            ott_ext_arrow(
                ott_ext_arrow(ott_ext_bvar(1), ott_ext_bvar(0)),
                ott_ext_arrow(
                    ott_ext_app(ott_ext_cst("OTTPropTrunc"), ott_ext_bvar(2)),
                    ott_ext_bvar(1),
                ),
            ),
        ),
    )
}
/// `OTTPropTrunc.pi : ∀ {A : Type}, IsProp (‖A‖)`
///
/// The truncation type is definitionally a proposition.
pub fn ott_prop_trunc_is_prop_ty() -> Expr {
    ott_ext_ipi(
        "A",
        ott_ext_type0(),
        ott_ext_app(
            ott_ext_cst("IsProp"),
            ott_ext_app(ott_ext_cst("OTTPropTrunc"), ott_ext_bvar(0)),
        ),
    )
}
/// `OTTProofIrrelevance : ∀ (P : Prop) (h k : P), ObsEq P h k`
///
/// Proof irrelevance for OTT: all proofs of a proposition are observationally
/// equal. This is definitional in OTT because ObsEq on propositions collapses.
pub fn ott_proof_irrel_ty() -> Expr {
    ott_ext_pi(
        "P",
        ott_ext_prop(),
        ott_ext_pi(
            "h",
            ott_ext_bvar(0),
            ott_ext_pi(
                "k",
                ott_ext_bvar(1),
                ott_ext_app3(
                    ott_ext_cst("ObsEq"),
                    ott_ext_bvar(2),
                    ott_ext_bvar(1),
                    ott_ext_bvar(0),
                ),
            ),
        ),
    )
}
/// `OTTSingletonElim : ∀ {A : Type} {a : A} (P : ∀ (b : A), ObsEq A a b → Type)
///     (d : P a refl), ∀ (b : A) (e : ObsEq A a b), P b e`
///
/// Singleton elimination (J-rule) for observational equality.
/// This is the fundamental recursion principle for ObsEq.
pub fn ott_singleton_elim_ty() -> Expr {
    ott_ext_ipi(
        "A",
        ott_ext_type0(),
        ott_ext_ipi(
            "a",
            ott_ext_bvar(0),
            ott_ext_pi(
                "P",
                ott_ext_pi(
                    "b",
                    ott_ext_bvar(1),
                    ott_ext_arrow(
                        ott_ext_app3(
                            ott_ext_cst("ObsEq"),
                            ott_ext_bvar(2),
                            ott_ext_bvar(1),
                            ott_ext_bvar(0),
                        ),
                        ott_ext_type0(),
                    ),
                ),
                ott_ext_arrow(
                    ott_ext_app(ott_ext_bvar(0), ott_ext_bvar(1)),
                    ott_ext_pi(
                        "b",
                        ott_ext_bvar(3),
                        ott_ext_pi(
                            "e",
                            ott_ext_app3(
                                ott_ext_cst("ObsEq"),
                                ott_ext_bvar(4),
                                ott_ext_bvar(2),
                                ott_ext_bvar(0),
                            ),
                            ott_ext_app(ott_ext_bvar(3), ott_ext_bvar(1)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `ObsEq.eta_fun : ∀ {A B : Type} (f : A → B), ObsEq (A→B) f (fun x => f x)`
///
/// Eta-rule for functions: a function is observationally equal to its eta expansion.
pub fn obs_eq_eta_fun_ty() -> Expr {
    ott_ext_ipi(
        "A",
        ott_ext_type0(),
        ott_ext_ipi(
            "B",
            ott_ext_type0(),
            ott_ext_pi(
                "f",
                ott_ext_arrow(ott_ext_bvar(1), ott_ext_bvar(0)),
                ott_ext_app3(
                    ott_ext_cst("ObsEq"),
                    ott_ext_arrow(ott_ext_bvar(2), ott_ext_bvar(1)),
                    ott_ext_bvar(0),
                    ott_ext_bvar(0),
                ),
            ),
        ),
    )
}
/// `ObsEq.eta_sigma : ∀ {A : Type} {B : A → Type} (p : Σ x : A, B x),
///     ObsEq (Σ A B) p ⟨p.1, p.2⟩`
///
/// Eta-rule for dependent pairs: every pair is equal to its own eta expansion.
pub fn obs_eq_eta_sigma_ty() -> Expr {
    ott_ext_ipi(
        "A",
        ott_ext_type0(),
        ott_ext_ipi(
            "B",
            ott_ext_arrow(ott_ext_bvar(0), ott_ext_type0()),
            ott_ext_pi(
                "p",
                ott_ext_app(ott_ext_cst("Sigma"), ott_ext_bvar(1)),
                ott_ext_app3(
                    ott_ext_cst("ObsEq"),
                    ott_ext_app(ott_ext_cst("Sigma"), ott_ext_bvar(2)),
                    ott_ext_bvar(0),
                    ott_ext_bvar(0),
                ),
            ),
        ),
    )
}
/// `ObsEq.definitional_eta : ∀ {A B : Type} {f g : A → B},
///     (∀ x, f x ≡ g x) → f ≡ g`
///
/// Definitional eta for function types in OTT: observational equality
/// on `A → B` reduces to pointwise equality, which is definitional (not axiomatic).
pub fn obs_eq_def_eta_ty() -> Expr {
    ott_ext_ipi(
        "A",
        ott_ext_type0(),
        ott_ext_ipi(
            "B",
            ott_ext_type0(),
            ott_ext_pi(
                "f",
                ott_ext_arrow(ott_ext_bvar(1), ott_ext_bvar(0)),
                ott_ext_pi(
                    "g",
                    ott_ext_arrow(ott_ext_bvar(2), ott_ext_bvar(1)),
                    ott_ext_arrow(
                        ott_ext_pi(
                            "x",
                            ott_ext_bvar(3),
                            ott_ext_app3(
                                ott_ext_cst("ObsEq"),
                                ott_ext_bvar(3),
                                ott_ext_app(ott_ext_bvar(2), ott_ext_bvar(0)),
                                ott_ext_app(ott_ext_bvar(1), ott_ext_bvar(0)),
                            ),
                        ),
                        ott_ext_app3(
                            ott_ext_cst("ObsEq"),
                            ott_ext_arrow(ott_ext_bvar(4), ott_ext_bvar(3)),
                            ott_ext_bvar(1),
                            ott_ext_bvar(0),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `ObsQuot : ∀ (A : Type) (R : A → A → Prop), Type`
///
/// Observational quotient type: in OTT quotients are built-in because
/// the equality of `Quot A R` is defined observationally.
pub fn obs_quot_ty() -> Expr {
    ott_ext_pi(
        "A",
        ott_ext_type0(),
        ott_ext_arrow(
            ott_ext_arrow(
                ott_ext_bvar(0),
                ott_ext_arrow(ott_ext_bvar(1), ott_ext_prop()),
            ),
            ott_ext_type0(),
        ),
    )
}
/// `ObsQuot.eq : ∀ {A : Type} {R : A → A → Prop} {a b : A},
///     R a b ↔ ObsEq (ObsQuot R) (ObsQuot.mk a) (ObsQuot.mk b)`
///
/// In OTT, equality of elements of the quotient type is *definitionally*
/// equivalent to the relation R.
pub fn obs_quot_eq_ty() -> Expr {
    ott_ext_ipi(
        "A",
        ott_ext_type0(),
        ott_ext_ipi(
            "R",
            ott_ext_arrow(
                ott_ext_bvar(0),
                ott_ext_arrow(ott_ext_bvar(1), ott_ext_prop()),
            ),
            ott_ext_ipi(
                "a",
                ott_ext_bvar(1),
                ott_ext_ipi(
                    "b",
                    ott_ext_bvar(2),
                    ott_ext_app2(
                        ott_ext_cst("Iff"),
                        ott_ext_app2(ott_ext_bvar(2), ott_ext_bvar(1), ott_ext_bvar(0)),
                        ott_ext_app3(
                            ott_ext_cst("ObsEq"),
                            ott_ext_app2(ott_ext_cst("ObsQuot"), ott_ext_bvar(3), ott_ext_bvar(2)),
                            ott_ext_app(ott_ext_cst("ObsQuot.mk"), ott_ext_bvar(1)),
                            ott_ext_app(ott_ext_cst("ObsQuot.mk"), ott_ext_bvar(0)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `IsPER : ∀ {A : Type} (R : A → A → Prop), Prop`
///
/// The predicate that R is a partial equivalence relation (symmetric and transitive
/// but not necessarily reflexive). PER models are used in realizability semantics
/// and denotational semantics of polymorphic type theory.
pub fn is_per_ty() -> Expr {
    ott_ext_ipi(
        "A",
        ott_ext_type0(),
        ott_ext_arrow(
            ott_ext_arrow(
                ott_ext_bvar(0),
                ott_ext_arrow(ott_ext_bvar(1), ott_ext_prop()),
            ),
            ott_ext_prop(),
        ),
    )
}
/// `PERModel : Type → Type 1`
///
/// The type of PER models for a given type: assigns a PER to each element
/// of the type.
pub fn per_model_ty() -> Expr {
    ott_ext_arrow(ott_ext_type0(), ott_ext_type1())
}
/// `PERModel.carrier : ∀ {A : Type} (M : PERModel A), Type`
///
/// Project the carrier of a PER model.
pub fn per_model_carrier_ty() -> Expr {
    ott_ext_ipi(
        "A",
        ott_ext_type0(),
        ott_ext_arrow(
            ott_ext_app(ott_ext_cst("PERModel"), ott_ext_bvar(0)),
            ott_ext_type0(),
        ),
    )
}
/// `PERModel.rel : ∀ {A : Type} (M : PERModel A), PERModel.carrier M → PERModel.carrier M → Prop`
///
/// Project the partial equivalence relation from a PER model.
pub fn per_model_rel_ty() -> Expr {
    ott_ext_ipi(
        "A",
        ott_ext_type0(),
        ott_ext_pi(
            "M",
            ott_ext_app(ott_ext_cst("PERModel"), ott_ext_bvar(0)),
            ott_ext_arrow(
                ott_ext_app(ott_ext_cst("PERModel.carrier"), ott_ext_bvar(0)),
                ott_ext_arrow(
                    ott_ext_app(ott_ext_cst("PERModel.carrier"), ott_ext_bvar(1)),
                    ott_ext_prop(),
                ),
            ),
        ),
    )
}
/// `SetoidInterp : ∀ (A : Type), Setoid`
///
/// The setoid interpretation of a type: every type A yields a setoid
/// with carrier A and observational equality as the relation.
pub fn setoid_interp_ty() -> Expr {
    ott_ext_arrow(ott_ext_type0(), ott_ext_cst("Setoid"))
}
/// `SetoidInterp.pi : ∀ {A : Type} {B : A → Type},
///     SetoidInterp (Π x : A, B x) = SetoidExponential (SetoidInterp A) (SetoidInterp B_?)`
///
/// The setoid interpretation commutes with Pi types.
pub fn setoid_interp_pi_ty() -> Expr {
    ott_ext_ipi(
        "A",
        ott_ext_type0(),
        ott_ext_ipi(
            "B",
            ott_ext_arrow(ott_ext_bvar(0), ott_ext_type0()),
            ott_ext_app3(
                ott_ext_cst("Eq"),
                ott_ext_cst("Setoid"),
                ott_ext_app(
                    ott_ext_cst("SetoidInterp"),
                    ott_ext_pi(
                        "x",
                        ott_ext_bvar(1),
                        ott_ext_app(ott_ext_bvar(1), ott_ext_bvar(0)),
                    ),
                ),
                ott_ext_app2(
                    ott_ext_cst("SetoidExponential"),
                    ott_ext_app(ott_ext_cst("SetoidInterp"), ott_ext_bvar(1)),
                    ott_ext_app(
                        ott_ext_cst("SetoidInterp"),
                        ott_ext_app(ott_ext_bvar(0), ott_ext_bvar(1)),
                    ),
                ),
            ),
        ),
    )
}
/// `UniverseRussell : ∀ (n : Nat), Type (n+1)`
///
/// Russell-style universe: `Type n` contains types that can be mentioned
/// without explicit decoding. Each `A : Type n` is itself a type.
/// In OTT, the equality of `Type n` elements is given by equivalence of
/// the corresponding setoids.
pub fn universe_russell_ty() -> Expr {
    ott_ext_arrow(ott_ext_cst("Nat"), ott_ext_type1())
}
/// `UniverseTarski : ∀ (n : Nat), Type (n+1)`
///
/// Tarski-style universe: a code type `U n` with an explicit decoding
/// function `El : U n → Type`. The OTT equality of codes in `U n` is
/// independent of the decoding.
pub fn universe_tarski_ty() -> Expr {
    ott_ext_arrow(ott_ext_cst("Nat"), ott_ext_type1())
}
/// `UniverseTarski.El : ∀ (n : Nat), UniverseTarski n → Type`
///
/// The decoding function for Tarski-style universes.
pub fn universe_tarski_el_ty() -> Expr {
    ott_ext_pi(
        "n",
        ott_ext_cst("Nat"),
        ott_ext_arrow(
            ott_ext_app(ott_ext_cst("UniverseTarski"), ott_ext_bvar(0)),
            ott_ext_type0(),
        ),
    )
}
/// `UnivPolyOTT : ∀ {u : Level} (A : Type u), Prop`
///
/// Universe polymorphism compatibility in OTT: every universe-polymorphic
/// type admits an observational equality that respects level constraints.
pub fn univ_poly_ott_ty() -> Expr {
    ott_ext_ipi(
        "u",
        ott_ext_cst("Level"),
        ott_ext_arrow(ott_ext_cst("Type"), ott_ext_prop()),
    )
}
/// `Realizability.valid : ∀ (P : Prop), Type`
///
/// A proposition `P` is realizable if there exists a realizer (a computable
/// proof term) witnessing it. This is the realizability valuation of P.
pub fn realizability_valid_ty() -> Expr {
    ott_ext_arrow(ott_ext_prop(), ott_ext_type0())
}
/// `Realizability.completeness : ∀ (P : Prop), P → Realizability.valid P`
///
/// Completeness: every proof of P gives a realizer of P.
pub fn realizability_completeness_ty() -> Expr {
    ott_ext_pi(
        "P",
        ott_ext_prop(),
        ott_ext_arrow(
            ott_ext_bvar(0),
            ott_ext_app(ott_ext_cst("Realizability.valid"), ott_ext_bvar(0)),
        ),
    )
}
/// `Realizability.soundness : ∀ (P : Prop), Realizability.valid P → P`
///
/// Soundness: every realizer of P gives an actual proof of P.
pub fn realizability_soundness_ty() -> Expr {
    ott_ext_pi(
        "P",
        ott_ext_prop(),
        ott_ext_arrow(
            ott_ext_app(ott_ext_cst("Realizability.valid"), ott_ext_bvar(0)),
            ott_ext_bvar(0),
        ),
    )
}
/// `Realizability.uniformity : ∀ {A : Type} (P : A → Prop),
///     (∀ a, Realizability.valid (P a)) → Realizability.valid (∀ a, P a)`
///
/// Uniformity of realizability: a family of realizable predicates can be
/// uniformly realised.
pub fn realizability_uniformity_ty() -> Expr {
    ott_ext_ipi(
        "A",
        ott_ext_type0(),
        ott_ext_pi(
            "P",
            ott_ext_arrow(ott_ext_bvar(0), ott_ext_prop()),
            ott_ext_arrow(
                ott_ext_pi(
                    "a",
                    ott_ext_bvar(1),
                    ott_ext_app(
                        ott_ext_cst("Realizability.valid"),
                        ott_ext_app(ott_ext_bvar(1), ott_ext_bvar(0)),
                    ),
                ),
                ott_ext_app(
                    ott_ext_cst("Realizability.valid"),
                    ott_ext_pi(
                        "a",
                        ott_ext_bvar(2),
                        ott_ext_app(ott_ext_bvar(2), ott_ext_bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// `ObsEq.isProp : ∀ {A : Type} {a b : A}, IsProp (ObsEq A a b)`
///
/// Observational equality proofs are unique (prop-irrelevant).
/// This is the key property that makes OTT work: equality is a proposition.
pub fn obs_eq_is_prop_ty() -> Expr {
    ott_ext_ipi(
        "A",
        ott_ext_type0(),
        ott_ext_ipi(
            "a",
            ott_ext_bvar(0),
            ott_ext_ipi(
                "b",
                ott_ext_bvar(1),
                ott_ext_app(
                    ott_ext_cst("IsProp"),
                    ott_ext_app3(
                        ott_ext_cst("ObsEq"),
                        ott_ext_bvar(2),
                        ott_ext_bvar(1),
                        ott_ext_bvar(0),
                    ),
                ),
            ),
        ),
    )
}
