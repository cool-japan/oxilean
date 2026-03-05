//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::LogicalRelation;

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
pub(super) fn arrow(a: Expr, b: Expr) -> Expr {
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
/// `Rel : Type → Type → Type`
///
/// The type of binary relations between two types.
/// `Rel A B = A → B → Prop`.
pub fn rel_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// `RelId : ∀ (A : Type), Rel A A`
///
/// The identity relation on A: `RelId A a a' ↔ a = a'`.
pub fn rel_id_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        app2(cst("Rel"), bvar(0), bvar(0)),
    )
}
/// `RelComp : ∀ {A B C : Type}, Rel A B → Rel B C → Rel A C`
///
/// Relational composition: (R ∘ S)(a, c) ↔ ∃ b, R(a,b) ∧ S(b,c).
pub fn rel_comp_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "B",
            type0(),
            impl_pi(
                "C",
                type0(),
                arrow(
                    app2(cst("Rel"), bvar(2), bvar(1)),
                    arrow(
                        app2(cst("Rel"), bvar(2), bvar(1)),
                        app2(cst("Rel"), bvar(4), bvar(2)),
                    ),
                ),
            ),
        ),
    )
}
/// `RelConverse : ∀ {A B : Type}, Rel A B → Rel B A`
///
/// The converse relation: R^op(b, a) ↔ R(a, b).
pub fn rel_converse_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "B",
            type0(),
            arrow(
                app2(cst("Rel"), bvar(1), bvar(0)),
                app2(cst("Rel"), bvar(1), bvar(2)),
            ),
        ),
    )
}
/// `RelFun : ∀ {A B C D : Type}, Rel A B → Rel C D → Rel (A → C) (B → D)`
///
/// Lifting of relations to function types: the "function relation" between A→C and B→D
/// induced by relations R : Rel A B and S : Rel C D.
/// `RelFun R S f g ↔ ∀ a b, R a b → S (f a) (g b)`.
pub fn rel_fun_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "B",
            type0(),
            impl_pi(
                "C",
                type0(),
                impl_pi(
                    "D",
                    type0(),
                    arrow(
                        app2(cst("Rel"), bvar(3), bvar(2)),
                        arrow(
                            app2(cst("Rel"), bvar(3), bvar(2)),
                            app2(cst("Rel"), arrow(bvar(5), bvar(3)), arrow(bvar(5), bvar(3))),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `ReynoldsParametricity : Prop`
///
/// The fundamental parametricity theorem (Reynolds 1983):
/// Every polymorphic term f : ∀ α. F(α) satisfies the relation F(R) for every
/// relation R between types A and B.
/// Stated as an axiom at the metatheory level.
pub fn reynolds_parametricity_ty() -> Expr {
    prop()
}
/// `ParametricFunction : ∀ {F : Type → Type}, (∀ α, F α) → Prop`
///
/// Predicate: a polymorphic function is parametric if it preserves all relations.
/// `ParametricFunction f ↔ ∀ A B (R : Rel A B), Lift_F R (f A) (f B)`.
pub fn parametric_function_ty() -> Expr {
    impl_pi(
        "F",
        arrow(type0(), type0()),
        arrow(
            pi(BinderInfo::Default, "α", type0(), app(bvar(1), bvar(0))),
            prop(),
        ),
    )
}
/// `ParametricityCondition : ∀ {A B : Type} (R : Rel A B) {F : Type → Type},
///     (∀ α, F α) → Prop`
///
/// The parametricity condition for a specific relation R and functor F:
/// the polymorphic function maps R-related inputs to F(R)-related outputs.
pub fn parametricity_condition_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "B",
            type0(),
            pi(
                BinderInfo::Default,
                "R",
                app2(cst("Rel"), bvar(1), bvar(0)),
                impl_pi(
                    "F",
                    arrow(type0(), type0()),
                    arrow(
                        pi(BinderInfo::Default, "α", type0(), app(bvar(1), bvar(0))),
                        prop(),
                    ),
                ),
            ),
        ),
    )
}
/// `RelTypeConstructor : (Type → Type) → (Type → Type → Type)`
///
/// Lifting of type constructors to relational type constructors.
/// `RelTypeConstructor F A B = Rel (F A) (F B)`.
pub fn rel_type_constructor_ty() -> Expr {
    arrow(
        arrow(type0(), type0()),
        arrow(type0(), arrow(type0(), type0())),
    )
}
/// `FreeTheorem : ∀ (τ : Type), Prop`
///
/// The free theorem associated to a polymorphic type τ.
/// By Wadler's theorem-for-free, every inhabitant of ∀ α. τ(α) satisfies a
/// non-trivial equational property determined by τ.
pub fn free_theorem_ty() -> Expr {
    arrow(type0(), prop())
}
/// `FreeTheoremId : ∀ {A B : Type} (f : ∀ α, α → α),
///     ∀ (x : A), f A x = x`
///
/// Free theorem for the identity type ∀ α, α → α:
/// Any parametric function of this type must be the identity.
pub fn free_theorem_id_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "B",
            type0(),
            pi(
                BinderInfo::Default,
                "f",
                pi(BinderInfo::Default, "α", type0(), arrow(bvar(0), bvar(0))),
                pi(
                    BinderInfo::Default,
                    "x",
                    bvar(2),
                    app3(cst("Eq"), bvar(3), app(bvar(1), bvar(3)), bvar(0)),
                ),
            ),
        ),
    )
}
/// `FreeTheoremList : ∀ {A B : Type} (f : ∀ α, List α → List α) (g : A → B),
///     map g (f A xs) = f B (map g xs)`
///
/// Free theorem for ∀ α, List α → List α:
/// Every parametric such function commutes with map.
pub fn free_theorem_list_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "B",
            type0(),
            pi(
                BinderInfo::Default,
                "f",
                pi(
                    BinderInfo::Default,
                    "α",
                    type0(),
                    arrow(app(cst("List"), bvar(0)), app(cst("List"), bvar(0))),
                ),
                pi(BinderInfo::Default, "g", arrow(bvar(2), bvar(1)), prop()),
            ),
        ),
    )
}
/// `FreeTheoremFold : ∀ {A B : Type}
///     (f : ∀ α β, (α → β → β) → β → List α → β),
///     ∀ (h : A → B) (op : A → A → A) (z : A),
///     h (f A A op z xs) = f B B (fun a b -> h (op a ...)) (h z) (map h xs)`
///
/// A free theorem for fold-like polymorphic functions.
pub fn free_theorem_fold_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "B",
            type0(),
            pi(
                BinderInfo::Default,
                "f",
                pi(
                    BinderInfo::Default,
                    "α",
                    type0(),
                    pi(
                        BinderInfo::Default,
                        "β",
                        type0(),
                        arrow(
                            arrow(bvar(1), arrow(bvar(0), bvar(0))),
                            arrow(bvar(1), arrow(app(cst("List"), bvar(2)), bvar(2))),
                        ),
                    ),
                ),
                prop(),
            ),
        ),
    )
}
/// `Naturality : ∀ {F G : Type → Type} (η : ∀ α, F α → G α),
///     ∀ {A B : Type} (f : A → B), G f ∘ η A = η B ∘ F f`
///
/// A parametric polymorphic function η is a natural transformation.
pub fn naturality_ty() -> Expr {
    impl_pi(
        "F",
        arrow(type0(), type0()),
        impl_pi(
            "G",
            arrow(type0(), type0()),
            pi(
                BinderInfo::Default,
                "η",
                pi(
                    BinderInfo::Default,
                    "α",
                    type0(),
                    arrow(app(bvar(2), bvar(0)), app(bvar(2), bvar(0))),
                ),
                impl_pi(
                    "A",
                    type0(),
                    impl_pi("B", type0(), arrow(arrow(bvar(1), bvar(0)), prop())),
                ),
            ),
        ),
    )
}
/// `DiNaturality : ∀ {F : Type → Type → Type} (α : ∀ A, F A A),
///     ∀ {A B : Type} (f : A → B), ...`
///
/// Dinaturality: a parametric function of mixed-variance type is a dinatural
/// transformation, meaning it satisfies a hexagon identity.
pub fn dinaturality_ty() -> Expr {
    impl_pi(
        "F",
        arrow(type0(), arrow(type0(), type0())),
        pi(
            BinderInfo::Default,
            "α",
            pi(
                BinderInfo::Default,
                "A",
                type0(),
                app2(bvar(1), bvar(0), bvar(0)),
            ),
            impl_pi(
                "A",
                type0(),
                impl_pi("B", type0(), arrow(arrow(bvar(1), bvar(0)), prop())),
            ),
        ),
    )
}
/// `NaturalTransformation : (Type → Type) → (Type → Type) → Type`
///
/// The type of natural transformations between two functors F and G.
/// `NaturalTransformation F G = { η : ∀ α, F α → G α | Naturality η }`.
pub fn natural_transformation_ty() -> Expr {
    arrow(
        arrow(type0(), type0()),
        arrow(arrow(type0(), type0()), type0()),
    )
}
/// `NaturalIso : (Type → Type) → (Type → Type) → Type`
///
/// A natural isomorphism between functors.
pub fn natural_iso_ty() -> Expr {
    arrow(
        arrow(type0(), type0()),
        arrow(arrow(type0(), type0()), type0()),
    )
}
/// `AbstractType : Type`
///
/// An abstract type is an existential package: `∃ (α : Type), Interface α`.
/// This models information hiding and encapsulation.
pub fn abstract_type_ty() -> Expr {
    type0()
}
/// `AbstractPackage : ∀ (Interface : Type → Prop), Type`
///
/// A package hiding the concrete representation type:
/// `AbstractPackage I = ∃ (α : Type), I α × α`.
pub fn abstract_package_ty() -> Expr {
    arrow(arrow(type0(), prop()), type0())
}
/// `RepresentationIndependence : ∀ {I : Type → Prop}
///     (pkg1 pkg2 : AbstractPackage I), Prop`
///
/// Two abstract packages with the same interface are observationally equivalent:
/// no client program can distinguish them.
pub fn representation_independence_ty() -> Expr {
    impl_pi(
        "I",
        arrow(type0(), prop()),
        arrow(
            app(cst("AbstractPackage"), bvar(0)),
            arrow(app(cst("AbstractPackage"), bvar(1)), prop()),
        ),
    )
}
/// `AbstractionTheorem : ∀ {I : Type → Prop}
///     (pkg1 pkg2 : AbstractPackage I),
///     RepresentationIndependence pkg1 pkg2`
///
/// Reynolds' abstraction theorem: any two implementations of an interface
/// are representationally independent.
pub fn abstraction_theorem_ty() -> Expr {
    impl_pi(
        "I",
        arrow(type0(), prop()),
        pi(
            BinderInfo::Default,
            "pkg1",
            app(cst("AbstractPackage"), bvar(0)),
            pi(
                BinderInfo::Default,
                "pkg2",
                app(cst("AbstractPackage"), bvar(1)),
                app2(cst("RepresentationIndependence"), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// `ExistentialType : (Type → Type) → Type`
///
/// Existential quantification over types: `∃ α. F α`.
/// Models abstract data types in System F.
pub fn existential_type_ty() -> Expr {
    arrow(arrow(type0(), type0()), type0())
}
/// `Pack : ∀ {F : Type → Type} (A : Type), F A → ExistentialType F`
///
/// Packing a concrete implementation into an abstract package.
pub fn pack_ty() -> Expr {
    impl_pi(
        "F",
        arrow(type0(), type0()),
        pi(
            BinderInfo::Default,
            "A",
            type0(),
            arrow(app(bvar(1), bvar(0)), app(cst("ExistentialType"), bvar(2))),
        ),
    )
}
/// `Unpack : ∀ {F : Type → Type} {R : Prop},
///     ExistentialType F → (∀ A, F A → R) → R`
///
/// Unpacking (elimination) for existential types.
pub fn unpack_ty() -> Expr {
    impl_pi(
        "F",
        arrow(type0(), type0()),
        impl_pi(
            "R",
            prop(),
            arrow(
                app(cst("ExistentialType"), bvar(1)),
                arrow(
                    pi(
                        BinderInfo::Default,
                        "A",
                        type0(),
                        arrow(app(bvar(3), bvar(0)), bvar(2)),
                    ),
                    bvar(1),
                ),
            ),
        ),
    )
}
/// `LogicalRelation : ∀ (F : Type → Type), Type`
///
/// A logical relation over a type constructor F:
/// `LogicalRelation F = ∀ A B, Rel A B → Rel (F A) (F B)`.
/// This is the relational interpretation of F in a parametric model.
pub fn logical_relation_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        arrow(type0(), type0()),
        pi(
            BinderInfo::Default,
            "A",
            type0(),
            pi(
                BinderInfo::Default,
                "B",
                type0(),
                arrow(
                    app2(cst("Rel"), bvar(1), bvar(0)),
                    app2(cst("Rel"), app(bvar(3), bvar(2)), app(bvar(3), bvar(1))),
                ),
            ),
        ),
    )
}
/// `FundamentalLemma : ∀ (τ : Type) (f : τ),
///     LogicalRelation_at τ (f, f)`
///
/// The fundamental lemma of logical relations: every typeable term is related
/// to itself by the logical relation of its type.
pub fn fundamental_lemma_ty() -> Expr {
    pi(BinderInfo::Default, "τ", type0(), arrow(bvar(0), prop()))
}
/// `ClosedRelation : ∀ {F : Type → Type}, LogicalRelation F → Prop`
///
/// A logical relation is closed (a.k.a. admissible) if it is preserved by
/// least fixed points (needed for recursive types).
pub fn closed_relation_ty() -> Expr {
    impl_pi(
        "F",
        arrow(type0(), type0()),
        arrow(app(cst("LogicalRelation"), bvar(0)), prop()),
    )
}
/// `RelationalModel : Type`
///
/// A relational model of System F: assigns to each type a carrier set and
/// a logical relation, satisfying the parametricity condition for all terms.
pub fn relational_model_ty() -> Expr {
    type0()
}
/// `ParametricModel : RelationalModel → Prop`
///
/// A relational model is parametric if every definable function in System F
/// is interpreted by a parametric element (one that lives in all logical relations
/// connecting related inputs).
pub fn parametric_model_ty() -> Expr {
    arrow(cst("RelationalModel"), prop())
}
/// `SystemFType : Type`
///
/// Types of System F (second-order polymorphic lambda calculus):
/// ∀ α. τ, τ₁ → τ₂, base types.
pub fn system_f_type_ty() -> Expr {
    type0()
}
/// `SystemFTerm : SystemFType → Type`
///
/// Terms of System F of a given type.
pub fn system_f_term_ty() -> Expr {
    arrow(cst("SystemFType"), type0())
}
/// `PolymorphicId : ∀ (α : Type), α → α`
///
/// The polymorphic identity function: the unique inhabitant of ∀ α, α → α
/// (up to parametricity).
pub fn polymorphic_id_ty() -> Expr {
    pi(BinderInfo::Default, "α", type0(), arrow(bvar(0), bvar(0)))
}
/// `PolymorphicConst : ∀ (α β : Type), α → β → α`
///
/// The polymorphic constant (K combinator): the unique inhabitant of ∀ α β, α → β → α.
pub fn polymorphic_const_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "α",
        type0(),
        pi(
            BinderInfo::Default,
            "β",
            type0(),
            arrow(bvar(1), arrow(bvar(1), bvar(2))),
        ),
    )
}
/// `PolymorphicFlip : ∀ (α β γ : Type), (α → β → γ) → β → α → γ`
///
/// The flip combinator.
pub fn polymorphic_flip_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "α",
        type0(),
        pi(
            BinderInfo::Default,
            "β",
            type0(),
            pi(
                BinderInfo::Default,
                "γ",
                type0(),
                arrow(
                    arrow(bvar(2), arrow(bvar(1), bvar(0))),
                    arrow(bvar(2), arrow(bvar(3), bvar(3))),
                ),
            ),
        ),
    )
}
/// `ChurchNumeral : (∀ α, (α → α) → α → α) → Nat`
///
/// Church numerals in System F: ∀ α. (α → α) → α → α.
/// Parametricity implies these are exactly the natural numbers.
pub fn church_numeral_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "α",
        type0(),
        arrow(arrow(bvar(0), bvar(0)), arrow(bvar(1), bvar(1))),
    )
}
/// `ChurchBool : (∀ α, α → α → α) → Bool`
///
/// Church booleans in System F: ∀ α. α → α → α.
/// Parametricity implies these are exactly true and false.
pub fn church_bool_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "α",
        type0(),
        arrow(bvar(0), arrow(bvar(1), bvar(1))),
    )
}
/// `CoherenceTheorem : ∀ {F : Type → Type}
///     (f g : ∀ α, F α), f = g`
///
/// The coherence theorem via parametricity:
/// In a polymorphic type with sufficient constraints (e.g., functor laws),
/// all parallel morphisms are equal. Parametricity gives coherence for free.
pub fn coherence_theorem_ty() -> Expr {
    impl_pi(
        "F",
        arrow(type0(), type0()),
        pi(
            BinderInfo::Default,
            "f",
            pi(BinderInfo::Default, "α", type0(), app(bvar(1), bvar(0))),
            pi(
                BinderInfo::Default,
                "g",
                pi(BinderInfo::Default, "α", type0(), app(bvar(2), bvar(0))),
                app3(
                    cst("Eq"),
                    pi(BinderInfo::Default, "α", type0(), app(bvar(3), bvar(0))),
                    bvar(1),
                    bvar(0),
                ),
            ),
        ),
    )
}
/// `ParametricityCoherence : ∀ {A B : Type} {F : Type → Type → Type}
///     (f g : ∀ α β, F α β), f = g`
///
/// Coherence for bifunctors: parametricity forces all polymorphic terms
/// of the same type to be equal (up to the relevant equations).
pub fn parametricity_coherence_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "B",
            type0(),
            impl_pi(
                "F",
                arrow(type0(), arrow(type0(), type0())),
                pi(
                    BinderInfo::Default,
                    "f",
                    pi(
                        BinderInfo::Default,
                        "α",
                        type0(),
                        pi(
                            BinderInfo::Default,
                            "β",
                            type0(),
                            app2(bvar(3), bvar(1), bvar(0)),
                        ),
                    ),
                    pi(
                        BinderInfo::Default,
                        "g",
                        pi(
                            BinderInfo::Default,
                            "α",
                            type0(),
                            pi(
                                BinderInfo::Default,
                                "β",
                                type0(),
                                app2(bvar(4), bvar(1), bvar(0)),
                            ),
                        ),
                        prop(),
                    ),
                ),
            ),
        ),
    )
}
/// `UniqueInhabitant : ∀ (τ : ∀ α, F α), Prop`
///
/// Parametricity can imply uniqueness: some polymorphic types have a unique
/// (up to propositional equality) inhabitant by the free theorem.
pub fn unique_inhabitant_ty() -> Expr {
    impl_pi(
        "F",
        arrow(type0(), type0()),
        arrow(
            pi(BinderInfo::Default, "α", type0(), app(bvar(1), bvar(0))),
            prop(),
        ),
    )
}
/// `UniversalProperty : ∀ {F : Type → Type}, (∀ α, F α) → Prop`
///
/// A polymorphic function satisfies a universal property if it is the unique
/// natural transformation of its type — which parametricity guarantees for
/// many common functors.
pub fn universal_property_ty() -> Expr {
    impl_pi(
        "F",
        arrow(type0(), type0()),
        arrow(
            pi(BinderInfo::Default, "α", type0(), app(bvar(1), bvar(0))),
            prop(),
        ),
    )
}
/// `KripkeRelation : ∀ (W : Type) (A B : Type), Type`
///
/// A Kripke relation over a world type W between A and B:
/// `W → A → B → Prop`. Used in step-indexed logical relations.
pub fn kripke_relation_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "W",
        type0(),
        pi(
            BinderInfo::Default,
            "A",
            type0(),
            pi(
                BinderInfo::Default,
                "B",
                type0(),
                arrow(bvar(2), arrow(bvar(2), arrow(bvar(2), prop()))),
            ),
        ),
    )
}
/// `StepIndexedRelation : ∀ (A B : Type), Type`
///
/// A step-indexed (Appel-McAllester) logical relation between A and B,
/// indexed by a step count n : Nat.
/// Used to prove parametricity for recursive types (iso-recursive, equi-recursive).
pub fn step_indexed_relation_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        pi(
            BinderInfo::Default,
            "B",
            type0(),
            arrow(nat_ty(), arrow(bvar(1), arrow(bvar(2), prop()))),
        ),
    )
}
/// `BiorthogonalityRelation : ∀ (A : Type), Type`
///
/// The biorthogonality (⊥⊥) construction used in orthogonality-based
/// logical relations (e.g., Pitts, Bierman-Pitts).
/// `Biorthogonal A = { S : Set A | S = S^⊥⊥ }`.
pub fn biorthogonality_relation_ty() -> Expr {
    arrow(type0(), type0())
}
/// `AdmissibleRelation : ∀ (A B : Type), Rel A B → Prop`
///
/// An admissible relation is one closed under directed limits (needed for
/// domain-theoretic models of parametricity).
pub fn admissible_relation_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        pi(
            BinderInfo::Default,
            "B",
            type0(),
            arrow(app2(cst("Rel"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `PiType : ∀ (A : Type) (B : A → Type), Type`
///
/// Dependent function type (Pi-type): Π (x : A), B x.
/// The fundamental dependent type construct.
pub fn pi_type_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        arrow(arrow(bvar(0), type0()), type0()),
    )
}
/// `SigmaType : ∀ (A : Type) (B : A → Type), Type`
///
/// Dependent pair type (Sigma-type): Σ (x : A), B x.
/// The existential type at the propositions-as-types level.
pub fn sigma_type_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        arrow(arrow(bvar(0), type0()), type0()),
    )
}
/// `SigmaFst : ∀ {A : Type} {B : A → Type}, Sigma A B → A`
///
/// First projection of a dependent pair.
pub fn sigma_fst_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "B",
            arrow(bvar(0), type0()),
            arrow(app2(cst("SigmaType"), bvar(1), bvar(0)), bvar(2)),
        ),
    )
}
/// `SigmaSnd : ∀ {A : Type} {B : A → Type} (p : Sigma A B), B (fst p)`
///
/// Second projection of a dependent pair.
pub fn sigma_snd_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "B",
            arrow(bvar(0), type0()),
            pi(
                BinderInfo::Default,
                "p",
                app2(cst("SigmaType"), bvar(1), bvar(0)),
                app(bvar(1), app(cst("SigmaFst"), bvar(0))),
            ),
        ),
    )
}
/// `PiExt : ∀ {A : Type} {B : A → Type} (f g : Π x, B x),
///     (∀ x, f x = g x) → f = g`
///
/// Eta-extensionality for Pi-types: pointwise equal functions are equal.
pub fn pi_ext_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "B",
            arrow(bvar(0), type0()),
            pi(
                BinderInfo::Default,
                "f",
                app2(cst("PiType"), bvar(1), bvar(0)),
                pi(
                    BinderInfo::Default,
                    "g",
                    app2(cst("PiType"), bvar(2), bvar(1)),
                    arrow(
                        pi(
                            BinderInfo::Default,
                            "x",
                            bvar(3),
                            app3(
                                cst("Eq"),
                                app(bvar(3), bvar(0)),
                                app(bvar(2), bvar(0)),
                                app(bvar(1), bvar(0)),
                            ),
                        ),
                        app3(
                            cst("Eq"),
                            app2(cst("PiType"), bvar(3), bvar(2)),
                            bvar(1),
                            bvar(0),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `IdType : ∀ {A : Type}, A → A → Type`
///
/// Martin-Löf identity type: `Id A a b` is the type of proofs that `a = b`.
pub fn id_type_ty() -> Expr {
    impl_pi("A", type0(), arrow(bvar(0), arrow(bvar(1), type0())))
}
/// `IdRefl : ∀ {A : Type} (a : A), Id A a a`
///
/// Reflexivity: the canonical element of the identity type.
pub fn id_refl_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        pi(
            BinderInfo::Default,
            "a",
            bvar(0),
            app2(cst("IdType"), bvar(1), bvar(0)),
        ),
    )
}
/// `IdElim : ∀ {A : Type} (C : ∀ a b, Id A a b → Type)
///     (refl_case : ∀ a, C a a (IdRefl a))
///     {a b : A} (p : Id A a b), C a b p`
///
/// The J eliminator (path induction).
pub fn id_elim_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        pi(
            BinderInfo::Default,
            "C",
            pi(
                BinderInfo::Default,
                "a",
                bvar(0),
                pi(
                    BinderInfo::Default,
                    "b",
                    bvar(1),
                    arrow(app2(cst("IdType"), bvar(2), bvar(0)), type0()),
                ),
            ),
            pi(
                BinderInfo::Default,
                "refl_case",
                pi(
                    BinderInfo::Default,
                    "a",
                    bvar(1),
                    app3(bvar(2), bvar(0), bvar(0), app(cst("IdRefl"), bvar(0))),
                ),
                impl_pi(
                    "a",
                    bvar(2),
                    impl_pi(
                        "b",
                        bvar(3),
                        pi(
                            BinderInfo::Default,
                            "p",
                            app2(cst("IdType"), bvar(4), bvar(0)),
                            app3(bvar(4), bvar(2), bvar(1), bvar(0)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `IdSymm : ∀ {A : Type} {a b : A}, Id A a b → Id A b a`
///
/// Symmetry of the identity type.
pub fn id_symm_ty() -> Expr {
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
                    app2(cst("IdType"), bvar(2), bvar(0)),
                    app2(cst("IdType"), bvar(1), bvar(3)),
                ),
            ),
        ),
    )
}
/// `IdTrans : ∀ {A : Type} {a b c : A},
///     Id A a b → Id A b c → Id A a c`
///
/// Transitivity (composition) of identity types.
pub fn id_trans_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "a",
            bvar(0),
            impl_pi(
                "b",
                bvar(1),
                impl_pi(
                    "c",
                    bvar(2),
                    arrow(
                        app2(cst("IdType"), bvar(3), bvar(1)),
                        arrow(
                            app2(cst("IdType"), bvar(3), bvar(0)),
                            app2(cst("IdType"), bvar(4), bvar(2)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `TypeEquiv : ∀ (A B : Type), Type`
///
/// Type equivalence: A ≃ B. A pair (f : A → B, g : B → A) with
/// g ∘ f ~ id and f ∘ g ~ id.
pub fn type_equiv_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        pi(BinderInfo::Default, "B", type0(), type0()),
    )
}
/// `UnivalenceAxiom : ∀ (A B : Type), (A ≃ B) ≃ (Id Type A B)`
///
/// The univalence axiom (Voevodsky): equivalence of types is equivalent
/// to identity of types.  Implies function extensionality.
pub fn univalence_axiom_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        pi(
            BinderInfo::Default,
            "B",
            type0(),
            app2(
                cst("TypeEquiv"),
                app2(cst("TypeEquiv"), bvar(1), bvar(0)),
                app2(cst("IdType"), type0(), bvar(1)),
            ),
        ),
    )
}
/// `FunExt : ∀ {A : Type} {B : A → Type} (f g : Π x, B x),
///     (∀ x, f x = g x) → f = g`
///
/// Function extensionality: pointwise equal functions are propositionally equal.
/// A consequence of univalence.
pub fn funext_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "B",
            arrow(bvar(0), type0()),
            pi(
                BinderInfo::Default,
                "f",
                app2(cst("PiType"), bvar(1), bvar(0)),
                pi(
                    BinderInfo::Default,
                    "g",
                    app2(cst("PiType"), bvar(2), bvar(1)),
                    arrow(
                        pi(
                            BinderInfo::Default,
                            "x",
                            bvar(3),
                            app2(cst("IdType"), app(bvar(3), bvar(0)), app(bvar(2), bvar(0))),
                        ),
                        app2(cst("IdType"), bvar(1), bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// `PropExt : ∀ (P Q : Prop), (P ↔ Q) → P = Q`
///
/// Propositional extensionality: logically equivalent propositions are equal.
pub fn prop_ext_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        prop(),
        pi(
            BinderInfo::Default,
            "Q",
            prop(),
            arrow(
                app2(cst("Iff"), bvar(1), bvar(0)),
                app2(cst("IdType"), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// `Trunc : Type → Prop`
///
/// Propositional truncation (squash type): |A| is the proposition that A
/// is inhabited.  The mere existence quantifier.
pub fn trunc_ty() -> Expr {
    arrow(type0(), prop())
}
/// `TruncIn : ∀ {A : Type}, A → Trunc A`
///
/// Introduction rule for propositional truncation.
pub fn trunc_in_ty() -> Expr {
    impl_pi("A", type0(), arrow(bvar(0), app(cst("Trunc"), bvar(1))))
}
/// `TruncElim : ∀ {A : Type} {P : Prop},
///     (A → P) → Trunc A → P`
///
/// Elimination into propositions.
pub fn trunc_elim_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "P",
            prop(),
            arrow(
                arrow(bvar(1), bvar(0)),
                arrow(app(cst("Trunc"), bvar(2)), bvar(1)),
            ),
        ),
    )
}
/// `CircleHIT : Type`
///
/// The circle S¹ as a higher inductive type with one point and one path.
/// `base : S¹` and `loop : Id S¹ base base`.
pub fn circle_hit_ty() -> Expr {
    type0()
}
/// `SuspensionHIT : Type → Type`
///
/// Suspension ΣA of a type A, a HIT with North, South poles and a meridian
/// path for each `a : A`.
pub fn suspension_hit_ty() -> Expr {
    arrow(type0(), type0())
}
/// `IntervalHIT : Type`
///
/// The interval [0,1] as a HIT with two endpoints and a path between them.
pub fn interval_hit_ty() -> Expr {
    type0()
}
/// `PushoutHIT : ∀ {A B C : Type}, (A → B) → (A → C) → Type`
///
/// The pushout of two functions, a fundamental HIT for homotopy type theory.
pub fn pushout_hit_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "B",
            type0(),
            impl_pi(
                "C",
                type0(),
                arrow(
                    arrow(bvar(2), bvar(1)),
                    arrow(arrow(bvar(3), bvar(1)), type0()),
                ),
            ),
        ),
    )
}
/// `ObsEq : ∀ (A : Type) (a b : A), Prop`
///
/// Observational equality: a =_A b.  In OTT this is defined by recursion on
/// the type A, making it definitionally proof-irrelevant.
pub fn obs_eq_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        arrow(bvar(0), arrow(bvar(1), prop())),
    )
}
/// `ObsEqRefl : ∀ {A : Type} (a : A), ObsEq A a a`
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
/// `Coerce : ∀ {A B : Type}, A = B → A → B`
///
/// Coercion / transport along a type equality.
pub fn coerce_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "B",
            type0(),
            arrow(
                app2(cst("IdType"), bvar(1), bvar(0)),
                arrow(bvar(2), bvar(2)),
            ),
        ),
    )
}
/// `Coherence : ∀ {A B : Type} (p q : A = B), p = q`
///
/// Proof irrelevance for type equalities in OTT: all coercions from A to B
/// are propositionally equal.
pub fn coherence_ott_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "B",
            type0(),
            pi(
                BinderInfo::Default,
                "p",
                app2(cst("IdType"), bvar(1), bvar(0)),
                pi(
                    BinderInfo::Default,
                    "q",
                    app2(cst("IdType"), bvar(2), bvar(1)),
                    app2(cst("IdType"), bvar(1), bvar(0)),
                ),
            ),
        ),
    )
}
/// `FibrantType : Type → Prop`
///
/// Fibrant types are those that satisfy the Kan filling condition.
/// In two-level type theory, the inner level consists of fibrant types.
pub fn fibrant_type_ty() -> Expr {
    arrow(type0(), prop())
}
/// `StrictEquality : ∀ (A : Type) (a b : A), Prop`
///
/// Strict (definitional) equality in the outer level of 2LTT.
/// This is a decidable, trivial propositional equality.
pub fn strict_equality_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        arrow(bvar(0), arrow(bvar(1), prop())),
    )
}
/// `InnerToOuter : ∀ {A : Type}, FibrantType A → A → A`
///
/// Embedding from the inner fibrant level to the outer strict level.
pub fn inner_to_outer_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(app(cst("FibrantType"), bvar(0)), arrow(bvar(1), bvar(2))),
    )
}
/// `Setoid : Type`
///
/// A setoid is a type equipped with an equivalence relation.
/// `Setoid = Σ (A : Type), (A → A → Prop) × IsEquiv`.
pub fn setoid_ty() -> Expr {
    type0()
}
/// `SetoidMap : Setoid → Setoid → Type`
///
/// A setoid morphism respects the equivalence relations.
pub fn setoid_map_ty() -> Expr {
    arrow(cst("Setoid"), arrow(cst("Setoid"), type0()))
}
/// `SetoidEquiv : Setoid → Setoid → Prop`
///
/// Two setoids are equivalent if there is a setoid isomorphism between them.
pub fn setoid_equiv_ty() -> Expr {
    arrow(cst("Setoid"), arrow(cst("Setoid"), prop()))
}
/// `SetoidQuotient : ∀ (S : Setoid), Type`
///
/// The quotient type of a setoid: the type of equivalence classes.
pub fn setoid_quotient_ty() -> Expr {
    arrow(cst("Setoid"), type0())
}
/// `PCA : Type`
///
/// A Partial Combinatory Algebra (PCA): the base structure for realizability.
/// Equipped with an application operation that may be partial.
pub fn pca_ty() -> Expr {
    type0()
}
/// `Realizer : ∀ (A : Type), PCA → Prop`
///
/// A realizer for an element of A: a PCA element that witnesses the
/// computational content of an element of A.
pub fn realizer_ty() -> Expr {
    arrow(type0(), arrow(cst("PCA"), prop()))
}
/// `RealizabilityTripos : Type`
///
/// A tripos (a higher-order fibration) arising from realizability.
/// The Kleene realizability tripos is the canonical example.
pub fn realizability_tripos_ty() -> Expr {
    type0()
}
/// `EffectiveTopos : Type`
///
/// The effective topos (Hyland 1982): the realizability topos over
/// Kleene's first algebra. Models constructive mathematics with
/// all functions being computable.
pub fn effective_topos_ty() -> Expr {
    type0()
}
/// `PER : Type → Type`
///
/// A partial equivalence relation on a type A: a symmetric and transitive
/// (but not necessarily reflexive) relation.  The domain of a PER is the
/// set of elements related to themselves.
pub fn per_ty() -> Expr {
    arrow(type0(), type0())
}
/// `PERDomain : ∀ {A : Type}, PER A → A → Prop`
///
/// The domain of a PER: elements a such that R(a, a).
pub fn per_domain_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(app(cst("PER"), bvar(0)), arrow(bvar(1), prop())),
    )
}
/// `PERMap : ∀ {A B : Type}, PER A → PER B → (A → B) → Prop`
///
/// A function respects two PERs: if R(a, a') then S(f a, f a').
pub fn per_map_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "B",
            type0(),
            arrow(
                app(cst("PER"), bvar(1)),
                arrow(
                    app(cst("PER"), bvar(1)),
                    arrow(arrow(bvar(3), bvar(2)), prop()),
                ),
            ),
        ),
    )
}
/// `PERModel : Type`
///
/// The PER model of a type theory: each type is interpreted as a PER,
/// and terms are interpreted as functions respecting the PERs.
pub fn per_model_ty() -> Expr {
    type0()
}
/// `Orthogonal : ∀ {A B C D : Type}, (A → B) → (C → D) → Prop`
///
/// Two morphisms are orthogonal if every lifting problem has a unique solution:
/// `∀ (u : A → C) (v : B → D), ∃! h, h ∘ f = u ∧ g ∘ h = v`.
pub fn orthogonal_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "B",
            type0(),
            impl_pi(
                "C",
                type0(),
                impl_pi(
                    "D",
                    type0(),
                    arrow(
                        arrow(bvar(3), bvar(2)),
                        arrow(arrow(bvar(3), bvar(2)), prop()),
                    ),
                ),
            ),
        ),
    )
}
/// `WFS : Type`
///
/// A weak factorization system (WFS) on a category: a pair (L, R) of
/// morphism classes such that L ⊥ R and every morphism factors as
/// an L-map followed by an R-map.
pub fn wfs_ty() -> Expr {
    type0()
}
/// `SmallObjectArgument : ∀ (I : Type), WFS`
///
/// The small object argument: given a set of generating cofibrations I,
/// produces a WFS (cof(I), inj(I)).
pub fn small_object_argument_ty() -> Expr {
    arrow(type0(), cst("WFS"))
}
/// `DynType : Type`
///
/// The dynamic type: the type of dynamically-typed values.
/// Every type embeds into DynType, and extraction may fail at runtime.
pub fn dyn_type_ty() -> Expr {
    type0()
}
/// `Inject : ∀ {A : Type}, A → DynType`
///
/// Injection of a statically-typed value into the dynamic type.
pub fn inject_ty() -> Expr {
    impl_pi("A", type0(), arrow(bvar(0), cst("DynType")))
}
/// `Project : ∀ {A : Type}, DynType → Option A`
///
/// Projection (cast) from the dynamic type to a static type.
/// May fail if the runtime type does not match.
pub fn project_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(cst("DynType"), app(cst("Option"), bvar(1))),
    )
}
/// `GradualConsistency : ∀ (A B : Type), Prop`
///
/// Type consistency in gradual typing: A ~ B holds when A and B are
/// compatible under the gradualization, i.e., one could be a specialization
/// of the other via the dynamic type.
pub fn gradual_consistency_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        pi(BinderInfo::Default, "B", type0(), prop()),
    )
}
/// `CastEvidence : ∀ {A B : Type}, GradualConsistency A B → A → B`
///
/// A cast with explicit evidence of type consistency.
pub fn cast_evidence_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "B",
            type0(),
            arrow(
                app2(cst("GradualConsistency"), bvar(1), bvar(0)),
                arrow(bvar(2), bvar(2)),
            ),
        ),
    )
}
/// `EffectSig : Type`
///
/// An algebraic effect signature: a set of operations with arities.
/// Example: State sig = { get : Unit → S, put : S → Unit }.
pub fn effect_sig_ty() -> Expr {
    type0()
}
/// `EffectTree : EffectSig → Type → Type`
///
/// The free monad over an effect signature: computation trees.
/// `EffectTree Σ A = Pure A | Op (Σ.op) (Σ.arity op → EffectTree Σ A)`.
pub fn effect_tree_ty() -> Expr {
    arrow(cst("EffectSig"), arrow(type0(), type0()))
}
/// `Handler : ∀ {Σ : EffectSig} {A B : Type},
///     EffectTree Σ A → (A → B) → (∀ op, Σ.arity op → (Σ.result op → B) → B) → B`
///
/// Effect handler: interprets an effect tree using a pure return case
/// and operation cases.
pub fn handler_ty() -> Expr {
    impl_pi(
        "Σ",
        cst("EffectSig"),
        impl_pi(
            "A",
            type0(),
            impl_pi(
                "B",
                type0(),
                arrow(
                    app2(cst("EffectTree"), bvar(2), bvar(1)),
                    arrow(arrow(bvar(2), bvar(1)), bvar(2)),
                ),
            ),
        ),
    )
}
/// `MonadLaw : ∀ (M : Type → Type), Prop`
///
/// The monad laws for M: left unit, right unit, and associativity of bind.
pub fn monad_law_ty() -> Expr {
    pi(BinderInfo::Default, "M", arrow(type0(), type0()), prop())
}
/// `MonadMorphism : ∀ {M N : Type → Type}, (∀ A, M A → N A) → Prop`
///
/// A monad morphism: a natural transformation between monads that commutes
/// with return and bind.
pub fn monad_morphism_ty() -> Expr {
    impl_pi(
        "M",
        arrow(type0(), type0()),
        impl_pi(
            "N",
            arrow(type0(), type0()),
            arrow(
                pi(
                    BinderInfo::Default,
                    "A",
                    type0(),
                    arrow(app(bvar(2), bvar(0)), app(bvar(2), bvar(0))),
                ),
                prop(),
            ),
        ),
    )
}
/// Register all parametricity and free theorem axioms into the kernel environment.
#[allow(clippy::too_many_arguments)]
pub fn register_parametricity(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("Rel", rel_ty()),
        ("RelId", rel_id_ty()),
        ("RelComp", rel_comp_ty()),
        ("RelConverse", rel_converse_ty()),
        ("RelFun", rel_fun_ty()),
        ("ReynoldsParametricity", reynolds_parametricity_ty()),
        ("ParametricFunction", parametric_function_ty()),
        ("ParametricityCondition", parametricity_condition_ty()),
        ("RelTypeConstructor", rel_type_constructor_ty()),
        ("FreeTheorem", free_theorem_ty()),
        ("FreeTheoremId", free_theorem_id_ty()),
        ("FreeTheoremList", free_theorem_list_ty()),
        ("FreeTheoremFold", free_theorem_fold_ty()),
        ("Naturality", naturality_ty()),
        ("DiNaturality", dinaturality_ty()),
        ("NaturalTransformation", natural_transformation_ty()),
        ("NaturalIso", natural_iso_ty()),
        ("AbstractType", abstract_type_ty()),
        ("AbstractPackage", abstract_package_ty()),
        (
            "RepresentationIndependence",
            representation_independence_ty(),
        ),
        ("AbstractionTheorem", abstraction_theorem_ty()),
        ("ExistentialType", existential_type_ty()),
        ("Pack", pack_ty()),
        ("Unpack", unpack_ty()),
        ("LogicalRelation", logical_relation_ty()),
        ("FundamentalLemma", fundamental_lemma_ty()),
        ("ClosedRelation", closed_relation_ty()),
        ("RelationalModel", relational_model_ty()),
        ("ParametricModel", parametric_model_ty()),
        ("SystemFType", system_f_type_ty()),
        ("SystemFTerm", system_f_term_ty()),
        ("PolymorphicId", polymorphic_id_ty()),
        ("PolymorphicConst", polymorphic_const_ty()),
        ("PolymorphicFlip", polymorphic_flip_ty()),
        ("ChurchNumeral", church_numeral_ty()),
        ("ChurchBool", church_bool_ty()),
        ("CoherenceTheorem", coherence_theorem_ty()),
        ("ParametricityCoherence", parametricity_coherence_ty()),
        ("UniqueInhabitant", unique_inhabitant_ty()),
        ("UniversalProperty", universal_property_ty()),
        ("KripkeRelation", kripke_relation_ty()),
        ("StepIndexedRelation", step_indexed_relation_ty()),
        ("BiorthogonalityRelation", biorthogonality_relation_ty()),
        ("AdmissibleRelation", admissible_relation_ty()),
        ("PiType", pi_type_ty()),
        ("SigmaType", sigma_type_ty()),
        ("SigmaFst", sigma_fst_ty()),
        ("SigmaSnd", sigma_snd_ty()),
        ("PiExt", pi_ext_ty()),
        ("IdType", id_type_ty()),
        ("IdRefl", id_refl_ty()),
        ("IdElim", id_elim_ty()),
        ("IdSymm", id_symm_ty()),
        ("IdTrans", id_trans_ty()),
        ("TypeEquiv", type_equiv_ty()),
        ("UnivalenceAxiom", univalence_axiom_ty()),
        ("FunExt", funext_ty()),
        ("PropExt", prop_ext_ty()),
        ("Trunc", trunc_ty()),
        ("TruncIn", trunc_in_ty()),
        ("TruncElim", trunc_elim_ty()),
        ("CircleHIT", circle_hit_ty()),
        ("SuspensionHIT", suspension_hit_ty()),
        ("IntervalHIT", interval_hit_ty()),
        ("PushoutHIT", pushout_hit_ty()),
        ("ObsEq", obs_eq_ty()),
        ("ObsEqRefl", obs_eq_refl_ty()),
        ("Coerce", coerce_ty()),
        ("CoherenceOTT", coherence_ott_ty()),
        ("FibrantType", fibrant_type_ty()),
        ("StrictEquality", strict_equality_ty()),
        ("InnerToOuter", inner_to_outer_ty()),
        ("Setoid", setoid_ty()),
        ("SetoidMap", setoid_map_ty()),
        ("SetoidEquiv", setoid_equiv_ty()),
        ("SetoidQuotient", setoid_quotient_ty()),
        ("PCA", pca_ty()),
        ("Realizer", realizer_ty()),
        ("RealizabilityTripos", realizability_tripos_ty()),
        ("EffectiveTopos", effective_topos_ty()),
        ("PER", per_ty()),
        ("PERDomain", per_domain_ty()),
        ("PERMap", per_map_ty()),
        ("PERModel", per_model_ty()),
        ("Orthogonal", orthogonal_ty()),
        ("WFS", wfs_ty()),
        ("SmallObjectArgument", small_object_argument_ty()),
        ("DynType", dyn_type_ty()),
        ("Inject", inject_ty()),
        ("Project", project_ty()),
        ("GradualConsistency", gradual_consistency_ty()),
        ("CastEvidence", cast_evidence_ty()),
        ("EffectSig", effect_sig_ty()),
        ("EffectTree", effect_tree_ty()),
        ("Handler", handler_ty()),
        ("MonadLaw", monad_law_ty()),
        ("MonadMorphism", monad_morphism_ty()),
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
