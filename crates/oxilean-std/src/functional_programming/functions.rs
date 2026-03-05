//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    AffineTraversal, ApplicativeData, Arrow, ArrowData, Coerce, ComonadExtend, ConsoleEffect,
    ConsoleProg, CpsTransform, DefunctClosure, DependentTypeExample, Effect, EffectHandler,
    EffectSystem, FreeApplicative, FreeMonad, FreeMonadInfo, FreeMonadInterpreter, Futumorphism,
    HList, HMap, Histomorphism, HomotopyEquivalence, Hylomorphism, Iso, Lens, LensComposer,
    Paramorphism, Prism, ProfunctorData, ProfunctorOptic, RecursionSchemeEval, RoseTree, Singleton,
    TraversableData, Traversal, TypeConstructorFunctor, TypeEquality, Zipper,
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
pub fn bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
pub fn fn_ty(dom: Expr, cod: Expr) -> Expr {
    arrow(dom, cod)
}
/// `FreeMonad : (Type → Type) → Type → Type`
///
/// Free monad: Free(F, A) = Pure A | Free (F (Free F A)).
pub fn free_monad_ty() -> Expr {
    arrow(fn_ty(type0(), type0()), fn_ty(type0(), type0()))
}
/// `Cofree : (Type → Type) → Type → Type`
///
/// Cofree comonad: Cofree F A = { extract: A, unwrap: F (Cofree F A) }.
pub fn cofree_ty() -> Expr {
    arrow(fn_ty(type0(), type0()), fn_ty(type0(), type0()))
}
/// `FixPoint : (Type → Type) → Type`
///
/// Least/greatest fixed point Fix F = F (Fix F).
pub fn fixpoint_ty() -> Expr {
    arrow(fn_ty(type0(), type0()), type0())
}
/// `Mu : (Type → Type) → Type`
///
/// Least fixed point (initial algebra / inductive type).
pub fn mu_ty() -> Expr {
    arrow(fn_ty(type0(), type0()), type0())
}
/// `Nu : (Type → Type) → Type`
///
/// Greatest fixed point (terminal coalgebra / coinductive type).
pub fn nu_ty() -> Expr {
    arrow(fn_ty(type0(), type0()), type0())
}
/// `Cata : (F A → A) → Fix F → A`  (catamorphism type template)
///
/// Represented as: (Type → Type) → Type → Type → Type.
pub fn cata_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        fn_ty(type0(), type0()),
        pi(
            BinderInfo::Default,
            "A",
            type0(),
            arrow(
                fn_ty(app(bvar(1), bvar(0)), bvar(0)),
                arrow(app(cst("FixPoint"), bvar(2)), bvar(1)),
            ),
        ),
    )
}
/// `Ana : (A → F A) → A → Fix F`  (anamorphism type template)
pub fn ana_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        fn_ty(type0(), type0()),
        pi(
            BinderInfo::Default,
            "A",
            type0(),
            arrow(
                fn_ty(bvar(0), app(bvar(1), bvar(0))),
                arrow(bvar(0), app(cst("FixPoint"), bvar(2))),
            ),
        ),
    )
}
/// `Hylo : (F B → B) → (A → F A) → A → B`  (hylomorphism)
pub fn hylo_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        fn_ty(type0(), type0()),
        pi(
            BinderInfo::Default,
            "A",
            type0(),
            pi(
                BinderInfo::Default,
                "B",
                type0(),
                arrow(
                    fn_ty(app(bvar(2), bvar(0)), bvar(0)),
                    arrow(
                        fn_ty(bvar(1), app(bvar(2), bvar(1))),
                        arrow(bvar(1), bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// `Catamorphism : (Type → Type) → Type → Type`
///
/// Fold over a recursive data type (Fix F).
pub fn catamorphism_ty() -> Expr {
    arrow(fn_ty(type0(), type0()), fn_ty(type0(), type0()))
}
/// `Paramorphism : (Type → Type) → Type → Type`
///
/// Extended fold with access to the original sub-structure.
pub fn paramorphism_ty() -> Expr {
    arrow(fn_ty(type0(), type0()), fn_ty(type0(), type0()))
}
/// `Histomorphism : (Type → Type) → Type → Type`
///
/// Fold with access to the computation history (course-of-values recursion).
pub fn histomorphism_ty() -> Expr {
    arrow(fn_ty(type0(), type0()), fn_ty(type0(), type0()))
}
/// `Futumorphism : (Type → Type) → Type → Type`
///
/// Unfold with lookahead via the Cofree comonad.
pub fn futumorphism_ty() -> Expr {
    arrow(fn_ty(type0(), type0()), fn_ty(type0(), type0()))
}
/// `Hylomorphism : (Type → Type) → Type → Type → Type`
///
/// Virtual data structure: fold ∘ unfold without materialising the intermediate Fix F.
pub fn hylomorphism_ty() -> Expr {
    arrow(
        fn_ty(type0(), type0()),
        arrow(type0(), fn_ty(type0(), type0())),
    )
}
/// `Chronomorphism : (Type → Type) → Type → Type`
///
/// Combined histomorphism + futumorphism.
pub fn chronomorphism_ty() -> Expr {
    arrow(fn_ty(type0(), type0()), fn_ty(type0(), type0()))
}
/// `Lens : Type → Type → Type`
///
/// (get: S → A, set: A → S → S) — lawful focus on a single A inside S.
pub fn lens_ty() -> Expr {
    arrow(type0(), fn_ty(type0(), type0()))
}
/// `Prism : Type → Type → Type`
///
/// (preview: S → Option A, review: A → S) — partial / sum type focus.
pub fn prism_ty() -> Expr {
    arrow(type0(), fn_ty(type0(), type0()))
}
/// `Traversal : Type → Type → Type`
///
/// Focus on zero or more As inside S.
pub fn traversal_ty() -> Expr {
    arrow(type0(), fn_ty(type0(), type0()))
}
/// `Iso : Type → Type → Type`
///
/// Isomorphism S ≅ A: (to: S → A, from: A → S).
pub fn iso_ty() -> Expr {
    arrow(type0(), fn_ty(type0(), type0()))
}
/// `AffineTraversal : Type → Type → Type`
///
/// Focus on 0 or 1 A inside S (between Lens and Traversal).
pub fn affine_traversal_ty() -> Expr {
    arrow(type0(), fn_ty(type0(), type0()))
}
/// `Effect : Type → Type → Type`
///
/// Computation with effect row E producing A.
pub fn effect_ty() -> Expr {
    arrow(type0(), fn_ty(type0(), type0()))
}
/// `EffectHandler : Type → Type → Type → Type`
///
/// Handles effect E in a computation, delegating remaining effects R.
pub fn effect_handler_ty() -> Expr {
    arrow(type0(), arrow(type0(), fn_ty(type0(), type0())))
}
/// `AlgebraicEffect : Type → Type → Type`
///
/// Effect type + operation + continuation.
pub fn algebraic_effect_ty() -> Expr {
    arrow(type0(), fn_ty(type0(), type0()))
}
/// `FreeSelective : (Type → Type) → Type → Type`
///
/// Free selective applicative functor.
pub fn free_selective_ty() -> Expr {
    arrow(fn_ty(type0(), type0()), fn_ty(type0(), type0()))
}
/// `Arrow : Type → Type → Type`
///
/// Generalized function f a b with arr / >>> / *** / first.
pub fn arrow_ty() -> Expr {
    arrow(type0(), fn_ty(type0(), type0()))
}
/// `HList : Type`
///
/// Heterogeneous list: HNil | HCons H Tail.
pub fn hlist_ty() -> Expr {
    type0()
}
/// `HMap : Type → Type → Type`
///
/// Type-level map from keys to values.
pub fn hmap_ty() -> Expr {
    arrow(type0(), fn_ty(type0(), type0()))
}
/// `Singleton : Type → Type`
///
/// Value reflected at the type level.
pub fn singleton_ty() -> Expr {
    arrow(type0(), type0())
}
/// `TypeEquality : Type → Type → Prop`
///
/// Propositional evidence that S ~ T.
pub fn type_equality_ty() -> Expr {
    arrow(type0(), fn_ty(type0(), prop()))
}
/// `Coerce : Type → Type → Prop`
///
/// Coercion with proof obligation (safe reinterpretation).
pub fn coerce_ty() -> Expr {
    arrow(type0(), fn_ty(type0(), prop()))
}
/// `FreeMonadLeftUnit : ∀ (F : Type → Type) (A : Type), Prop`
///
/// Left unit law: bind (pure a) f = f a.
pub fn free_monad_left_unit_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        fn_ty(type0(), type0()),
        arrow(type0(), prop()),
    )
}
/// `FreeMonadRightUnit : ∀ (F : Type → Type) (A : Type), Prop`
///
/// Right unit law: bind m pure = m.
pub fn free_monad_right_unit_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        fn_ty(type0(), type0()),
        arrow(type0(), prop()),
    )
}
/// `LensGetSet : ∀ (S A : Type), Lens S A → Prop`
///
/// GetSet law: set (get s) s = s.
pub fn lens_get_set_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "S",
        type0(),
        pi(
            BinderInfo::Default,
            "A",
            type0(),
            arrow(app2(cst("Lens"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `LensSetGet : ∀ (S A : Type), Lens S A → Prop`
///
/// SetGet law: get (set a s) = a.
pub fn lens_set_get_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "S",
        type0(),
        pi(
            BinderInfo::Default,
            "A",
            type0(),
            arrow(app2(cst("Lens"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `IsoRoundtrip : ∀ (S A : Type), Iso S A → Prop`
///
/// from (to s) = s  ∧  to (from a) = a.
pub fn iso_roundtrip_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "S",
        type0(),
        pi(
            BinderInfo::Default,
            "A",
            type0(),
            arrow(app2(cst("Iso"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `CatamorphismFusion : ∀ (F : Type → Type) (A B : Type), Prop`
///
/// cata g ∘ cata f = cata (g ∘ fmap (cata f)).
pub fn catamorphism_fusion_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        fn_ty(type0(), type0()),
        pi(BinderInfo::Default, "A", type0(), arrow(type0(), prop())),
    )
}
/// `HyloFusion : ∀ (F : Type → Type) (A B : Type), Prop`
///
/// hylo alg coalg = cata alg ∘ ana coalg (deforestation).
pub fn hylo_fusion_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        fn_ty(type0(), type0()),
        pi(BinderInfo::Default, "A", type0(), arrow(type0(), prop())),
    )
}
/// A catamorphism (fold) over a list as a stand-in for Fix F.
///
/// `cata(alg, list)` = right-fold using `alg`.
pub fn list_cata<A, B>(alg: impl Fn(Option<(A, B)>) -> B, list: Vec<A>) -> B {
    list.into_iter()
        .rev()
        .fold(alg(None), |acc, a| alg(Some((a, acc))))
}
/// An anamorphism (unfold) producing a list from a seed.
pub fn list_ana<A, S>(coalg: impl Fn(S) -> Option<(A, S)>, seed: S) -> Vec<A> {
    let mut result = Vec::new();
    let mut state = seed;
    loop {
        match coalg(state) {
            None => break,
            Some((a, next)) => {
                result.push(a);
                state = next;
            }
        }
    }
    result
}
/// A hylomorphism (fold ∘ unfold) without materialising the intermediate structure.
pub fn list_hylo<A, B, S>(
    alg: &dyn Fn(Option<(A, B)>) -> B,
    coalg: &dyn Fn(S) -> Option<(A, S)>,
    seed: S,
) -> B {
    match coalg(seed) {
        None => alg(None),
        Some((a, next)) => {
            let rec = list_hylo(alg, coalg, next);
            alg(Some((a, rec)))
        }
    }
}
/// A paramorphism over a list (extended fold with original tail access).
pub fn list_para<A: Clone, B>(alg: impl Fn(Option<(A, Vec<A>, B)>) -> B, list: Vec<A>) -> B {
    fn go<A: Clone, B>(alg: &dyn Fn(Option<(A, Vec<A>, B)>) -> B, slice: &[A]) -> B {
        if slice.is_empty() {
            alg(None)
        } else {
            let head = slice[0].clone();
            let tail = slice[1..].to_vec();
            let rec = go(alg, &slice[1..]);
            alg(Some((head, tail, rec)))
        }
    }
    go(&alg, &list)
}
/// `ScottDomain : Type`
///
/// A Scott domain: a dcpo (directed-complete partial order) used as denotational
/// semantic domains for programming languages.
pub fn scott_domain_ty() -> Expr {
    type0()
}
/// `ScottContinuous : Type → Type → Prop`
///
/// Continuous function between Scott domains (preserves directed sups).
pub fn scott_continuous_ty() -> Expr {
    arrow(type0(), fn_ty(type0(), prop()))
}
/// `FixedPointSemantics : (Type → Type) → Type → Prop`
///
/// Kleene fixed-point: the least fixed point of a monotone endofunctor on a
/// Scott domain, providing denotational semantics for recursive definitions.
pub fn fixed_point_semantics_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        fn_ty(type0(), type0()),
        arrow(type0(), prop()),
    )
}
/// `FullAbstraction : Type → Type → Prop`
///
/// Full abstraction: contextual equivalence = denotational equality.
pub fn full_abstraction_ty() -> Expr {
    arrow(type0(), fn_ty(type0(), prop()))
}
/// `LeastUpperBound : Type → Type → Prop`
///
/// Least upper bound (lub) of a directed set in a dcpo.
pub fn least_upper_bound_ty() -> Expr {
    arrow(type0(), fn_ty(type0(), prop()))
}
/// `ApproximationOrder : Type → Type → Prop`
///
/// Information ordering (⊑) in Scott domain theory.
pub fn approximation_order_ty() -> Expr {
    arrow(type0(), fn_ty(type0(), prop()))
}
/// `EffectRow : Type`
///
/// An effect row: a (possibly empty) collection of effect labels and their
/// operation signatures.
pub fn effect_row_ty() -> Expr {
    type0()
}
/// `ScopedEffect : Type → Type → Type`
///
/// A scoped algebraic effect: effects that delimit their own scope (e.g., catch).
pub fn scoped_effect_ty() -> Expr {
    arrow(type0(), fn_ty(type0(), type0()))
}
/// `DeepHandler : Type → Type → Type → Type`
///
/// A deep handler for algebraic effects, handling the entire computation tree.
/// Signature: EffectRow → ResultType → HandledType → Type.
pub fn deep_handler_ty() -> Expr {
    arrow(type0(), arrow(type0(), fn_ty(type0(), type0())))
}
/// `ShallowHandler : Type → Type → Type → Type`
///
/// A shallow handler for algebraic effects, handling only the first effect operation.
pub fn shallow_handler_ty() -> Expr {
    arrow(type0(), arrow(type0(), fn_ty(type0(), type0())))
}
/// `EffectSubrow : Type → Type → Prop`
///
/// Subrow relation: E ⊆ E', used for effect polymorphism.
pub fn effect_subrow_ty() -> Expr {
    arrow(type0(), fn_ty(type0(), prop()))
}
/// `FreerMonad : (Type → Type) → Type → Type`
///
/// Freer monad (extensible effects): Free(IFunctor, A).
/// Avoids the Functor constraint using the Yoneda embedding trick.
pub fn freer_monad_ty() -> Expr {
    arrow(fn_ty(type0(), type0()), fn_ty(type0(), type0()))
}
/// `FreeApplicative : (Type → Type) → Type → Type`
///
/// Free applicative functor: the free construction for applicatives.
pub fn free_applicative_ty() -> Expr {
    arrow(fn_ty(type0(), type0()), fn_ty(type0(), type0()))
}
/// `FreeMonadBind : ∀ (F : Type → Type) (A B : Type), Prop`
///
/// Monadic bind for the free monad satisfies associativity.
pub fn free_monad_bind_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        fn_ty(type0(), type0()),
        pi(BinderInfo::Default, "A", type0(), arrow(type0(), prop())),
    )
}
/// `FreerLift : ∀ (F : Type → Type) (A : Type), F A → FreerMonad F A`
///
/// The canonical lift of a functor effect into the Freer monad.
pub fn freer_lift_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        fn_ty(type0(), type0()),
        pi(
            BinderInfo::Default,
            "A",
            type0(),
            arrow(
                app(bvar(1), bvar(0)),
                app2(cst("FreerMonad"), bvar(2), bvar(1)),
            ),
        ),
    )
}
/// `Profunctor : Type → Type → Type`
///
/// A profunctor P : Type^op × Type → Type with dimap.
pub fn profunctor_ty() -> Expr {
    arrow(type0(), fn_ty(type0(), type0()))
}
/// `StrongProfunctor : Type → Type → Type`
///
/// A strong profunctor: profunctor with first' and second' (for lens encoding).
pub fn strong_profunctor_ty() -> Expr {
    arrow(type0(), fn_ty(type0(), type0()))
}
/// `TambaraModule : Type → Type → Type`
///
/// A Tambara module: a profunctor with an action of monoidal categories,
/// used as the categorical foundation for optics.
pub fn tambara_module_ty() -> Expr {
    arrow(type0(), fn_ty(type0(), type0()))
}
/// `ProfunctorOpticTy : Type → Type → Type → Type → Type`
///
/// Profunctor-encoded optic: ∀ P, P A B → P S T.
/// Type: (S T A B : Type) → Type.
pub fn profunctor_optic_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "S",
        type0(),
        pi(
            BinderInfo::Default,
            "T",
            type0(),
            pi(BinderInfo::Default, "A", type0(), arrow(type0(), type0())),
        ),
    )
}
/// `Dimap : ∀ (P : Type → Type → Type) (A B C D : Type), Prop`
///
/// The dimap law: functorial mapping in both contravariant and covariant positions.
pub fn dimap_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        fn_ty(type0(), fn_ty(type0(), type0())),
        pi(
            BinderInfo::Default,
            "A",
            type0(),
            pi(
                BinderInfo::Default,
                "B",
                type0(),
                pi(BinderInfo::Default, "C", type0(), arrow(type0(), prop())),
            ),
        ),
    )
}
/// `LensSetSet : ∀ (S A : Type), Lens S A → Prop`
///
/// SetSet law: set a' (set a s) = set a' s.
pub fn lens_set_set_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "S",
        type0(),
        pi(
            BinderInfo::Default,
            "A",
            type0(),
            arrow(app2(cst("Lens"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `VanLaarhovenLens : Type → Type → Type`
///
/// van Laarhoven encoding: ∀ (F : Type → Type), Functor F → (A → F A) → S → F S.
pub fn van_laarhoven_lens_ty() -> Expr {
    arrow(type0(), fn_ty(type0(), type0()))
}
/// `PrismLaw : ∀ (S A : Type), Prism S A → Prop`
///
/// Prism law: review (preview s) = s (when preview s = Some a, review a = s).
pub fn prism_law_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "S",
        type0(),
        pi(
            BinderInfo::Default,
            "A",
            type0(),
            arrow(app2(cst("Prism"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `TraversalCompose : ∀ (S A : Type), Traversal S A → Prop`
///
/// Traversal composition law (purity + fusion).
pub fn traversal_compose_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "S",
        type0(),
        pi(
            BinderInfo::Default,
            "A",
            type0(),
            arrow(app2(cst("Traversal"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `Comonad : (Type → Type) → Type`
///
/// A comonad W with extract : W A → A and extend : (W A → B) → W A → W B.
pub fn comonad_ty() -> Expr {
    arrow(fn_ty(type0(), type0()), type0())
}
/// `ComonadLaw : (Type → Type) → Type → Prop`
///
/// Comonad laws: extract . extend f = f, extend extract = id,
/// extend f . extend g = extend (f . extend g).
pub fn comonad_law_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "W",
        fn_ty(type0(), type0()),
        arrow(type0(), prop()),
    )
}
/// `CellularAutomataComonad : Type → Type`
///
/// Comonad for 1D cellular automata: focused streams with neighborhood access.
pub fn cellular_automata_comonad_ty() -> Expr {
    arrow(type0(), type0())
}
/// `StreamComonad : Type → Type`
///
/// The stream comonad: infinite sequences with extract (head) and extend.
pub fn stream_comonad_ty() -> Expr {
    arrow(type0(), type0())
}
/// `CofreeComonadUnfold : ∀ (F : Type → Type) (A : Type), Prop`
///
/// Cofree comonad via terminal coalgebra: Nu (A × F -).
pub fn cofree_comonad_unfold_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        fn_ty(type0(), type0()),
        arrow(type0(), prop()),
    )
}
/// `Applicative : (Type → Type) → Type`
///
/// An applicative functor with pure and <*>.
pub fn applicative_ty() -> Expr {
    arrow(fn_ty(type0(), type0()), type0())
}
/// `ApplicativeLaw : (Type → Type) → Type → Prop`
///
/// Applicative laws: identity, composition, homomorphism, interchange.
pub fn applicative_law_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        fn_ty(type0(), type0()),
        arrow(type0(), prop()),
    )
}
/// `DayConvolution : (Type → Type) → (Type → Type) → Type → Type`
///
/// Day convolution of two functors: (Day F G) A = ∃ X Y, F X × G Y × (X × Y → A).
pub fn day_convolution_ty() -> Expr {
    arrow(
        fn_ty(type0(), type0()),
        arrow(fn_ty(type0(), type0()), fn_ty(type0(), type0())),
    )
}
/// `IdiomBracket : (Type → Type) → Type → Type`
///
/// Idiom bracket notation for applicative expressions: [| f x y |].
pub fn idiom_bracket_ty() -> Expr {
    arrow(fn_ty(type0(), type0()), fn_ty(type0(), type0()))
}
/// `ArrowChoice : Type → Type → Type`
///
/// ArrowChoice: arrows with (+++) and left/right for sum types.
pub fn arrow_choice_ty() -> Expr {
    arrow(type0(), fn_ty(type0(), type0()))
}
/// `ArrowApply : Type → Type → Type`
///
/// ArrowApply (ArrowMonad): arrows that support application (app :: arr (arr a b, a) b).
pub fn arrow_apply_ty() -> Expr {
    arrow(type0(), fn_ty(type0(), type0()))
}
/// `ArrowLaw : Type → Type → Prop`
///
/// Hughes arrow laws: arr id = id, arr (f . g) = arr f >>> arr g, etc.
pub fn arrow_law_ty() -> Expr {
    arrow(type0(), fn_ty(type0(), prop()))
}
/// `ArrowFirst : Type → Type → Type → Type`
///
/// The `first` combinator for arrows: first f = f *** id.
pub fn arrow_first_ty() -> Expr {
    arrow(type0(), arrow(type0(), fn_ty(type0(), type0())))
}
/// `InitialAlgebra : (Type → Type) → Type → Prop`
///
/// Lambek's lemma: the initial algebra of F is a fixed point F(Mu F) ≅ Mu F.
pub fn initial_algebra_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        fn_ty(type0(), type0()),
        arrow(type0(), prop()),
    )
}
/// `FinalCoalgebra : (Type → Type) → Type → Prop`
///
/// The final coalgebra: Nu F is a fixed point with Nu F ≅ F(Nu F).
pub fn final_coalgebra_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        fn_ty(type0(), type0()),
        arrow(type0(), prop()),
    )
}
/// `BanachFixedPoint : Type → Type → Prop`
///
/// Banach's theorem in cpo: any contractive map has a unique fixed point.
pub fn banach_fixed_point_ty() -> Expr {
    arrow(type0(), fn_ty(type0(), prop()))
}
/// `FixpointType : (Type → Type) → Type`
///
/// The generic fixpoint type (equi-recursive types) for both Mu and Nu.
pub fn fixpoint_type_ty() -> Expr {
    arrow(fn_ty(type0(), type0()), type0())
}
/// `ContT : Type → (Type → Type) → Type → Type`
///
/// Continuation monad transformer: ContT r m a = (a → m r) → m r.
pub fn cont_t_ty() -> Expr {
    arrow(
        type0(),
        arrow(fn_ty(type0(), type0()), fn_ty(type0(), type0())),
    )
}
/// `Shift : Type → Type → Type`
///
/// Delimited continuation shift operator: (a → m b) → m a.
pub fn shift_ty() -> Expr {
    arrow(type0(), fn_ty(type0(), type0()))
}
/// `Reset : Type → Type → Type`
///
/// Delimited continuation reset (prompt): delimits the scope of a continuation.
pub fn reset_ty() -> Expr {
    arrow(type0(), fn_ty(type0(), type0()))
}
/// `CpsTransform : Type → Type → Type`
///
/// CPS transform: A → ((A → R) → R).
pub fn cps_transform_ty() -> Expr {
    arrow(type0(), fn_ty(type0(), type0()))
}
/// `DoubleNegationTranslation : Type → Prop`
///
/// Gödel-Gentzen: P is provable classically iff ¬¬P is provable constructively.
pub fn double_negation_translation_ty() -> Expr {
    arrow(type0(), prop())
}
/// `UniqueType : Type → Type`
///
/// A uniqueness type annotation (Clean language): value with guaranteed unique reference.
pub fn unique_type_ty() -> Expr {
    arrow(type0(), type0())
}
/// `LinearType : Type → Type`
///
/// A linear type: must be used exactly once (related to uniqueness types).
pub fn linear_type_ty() -> Expr {
    arrow(type0(), type0())
}
/// `UniquenessLaw : Type → Prop`
///
/// Uniqueness property: a unique value cannot be shared or copied.
pub fn uniqueness_law_ty() -> Expr {
    arrow(type0(), prop())
}
/// `LinearityLaw : Type → Prop`
///
/// Linearity property: a linear resource is consumed exactly once.
pub fn linearity_law_ty() -> Expr {
    arrow(type0(), prop())
}
/// Build the functional programming foundations environment: register all axioms.
pub fn build_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("FreeMonad", free_monad_ty()),
        ("Cofree", cofree_ty()),
        ("FixPoint", fixpoint_ty()),
        ("Mu", mu_ty()),
        ("Nu", nu_ty()),
        ("cata", cata_ty()),
        ("ana", ana_ty()),
        ("hylo", hylo_ty()),
        ("Catamorphism", catamorphism_ty()),
        ("Paramorphism", paramorphism_ty()),
        ("Histomorphism", histomorphism_ty()),
        ("Futumorphism", futumorphism_ty()),
        ("Hylomorphism", hylomorphism_ty()),
        ("Chronomorphism", chronomorphism_ty()),
        ("Lens", lens_ty()),
        ("Prism", prism_ty()),
        ("Traversal", traversal_ty()),
        ("Iso", iso_ty()),
        ("AffineTraversal", affine_traversal_ty()),
        ("Effect", effect_ty()),
        ("EffectHandler", effect_handler_ty()),
        ("AlgebraicEffect", algebraic_effect_ty()),
        ("FreeSelective", free_selective_ty()),
        ("Arrow", arrow_ty()),
        ("HList", hlist_ty()),
        ("HMap", hmap_ty()),
        ("Singleton", singleton_ty()),
        ("TypeEquality", type_equality_ty()),
        ("Coerce", coerce_ty()),
        ("free_monad_left_unit", free_monad_left_unit_ty()),
        ("free_monad_right_unit", free_monad_right_unit_ty()),
        ("lens_get_set", lens_get_set_ty()),
        ("lens_set_get", lens_set_get_ty()),
        ("iso_roundtrip", iso_roundtrip_ty()),
        ("catamorphism_fusion", catamorphism_fusion_ty()),
        ("hylo_fusion", hylo_fusion_ty()),
        ("ScottDomain", scott_domain_ty()),
        ("ScottContinuous", scott_continuous_ty()),
        ("FixedPointSemantics", fixed_point_semantics_ty()),
        ("FullAbstraction", full_abstraction_ty()),
        ("LeastUpperBound", least_upper_bound_ty()),
        ("ApproximationOrder", approximation_order_ty()),
        ("EffectRow", effect_row_ty()),
        ("ScopedEffect", scoped_effect_ty()),
        ("DeepHandler", deep_handler_ty()),
        ("ShallowHandler", shallow_handler_ty()),
        ("EffectSubrow", effect_subrow_ty()),
        ("FreerMonad", freer_monad_ty()),
        ("FreeApplicative", free_applicative_ty()),
        ("free_monad_bind", free_monad_bind_ty()),
        ("freer_lift", freer_lift_ty()),
        ("Profunctor", profunctor_ty()),
        ("StrongProfunctor", strong_profunctor_ty()),
        ("TambaraModule", tambara_module_ty()),
        ("ProfunctorOptic", profunctor_optic_ty()),
        ("dimap", dimap_ty()),
        ("lens_set_set", lens_set_set_ty()),
        ("VanLaarhovenLens", van_laarhoven_lens_ty()),
        ("prism_law", prism_law_ty()),
        ("traversal_compose", traversal_compose_ty()),
        ("Comonad", comonad_ty()),
        ("comonad_law", comonad_law_ty()),
        ("CellularAutomataComonad", cellular_automata_comonad_ty()),
        ("StreamComonad", stream_comonad_ty()),
        ("cofree_comonad_unfold", cofree_comonad_unfold_ty()),
        ("Applicative", applicative_ty()),
        ("applicative_law", applicative_law_ty()),
        ("DayConvolution", day_convolution_ty()),
        ("IdiomBracket", idiom_bracket_ty()),
        ("ArrowChoice", arrow_choice_ty()),
        ("ArrowApply", arrow_apply_ty()),
        ("arrow_law", arrow_law_ty()),
        ("ArrowFirst", arrow_first_ty()),
        ("initial_algebra", initial_algebra_ty()),
        ("final_coalgebra", final_coalgebra_ty()),
        ("BanachFixedPoint", banach_fixed_point_ty()),
        ("FixpointType", fixpoint_type_ty()),
        ("ContT", cont_t_ty()),
        ("Shift", shift_ty()),
        ("Reset", reset_ty()),
        ("CpsTransform", cps_transform_ty()),
        (
            "double_negation_translation",
            double_negation_translation_ty(),
        ),
        ("UniqueType", unique_type_ty()),
        ("LinearType", linear_type_ty()),
        ("uniqueness_law", uniqueness_law_ty()),
        ("linearity_law", linearity_law_ty()),
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
    #[test]
    fn test_free_monad_pure() {
        let m: FreeMonad<i32> = FreeMonad::pure(42);
        let result = m.fold(|a| a * 2, |_| 0);
        assert_eq!(result, 84);
    }
    #[test]
    fn test_list_cata_sum() {
        let sum = list_cata(
            |opt| match opt {
                None => 0,
                Some((a, acc)) => a + acc,
            },
            vec![1, 2, 3, 4, 5],
        );
        assert_eq!(sum, 15);
    }
    #[test]
    fn test_list_ana_range() {
        let list = list_ana(|n: usize| if n < 5 { Some((n, n + 1)) } else { None }, 0);
        assert_eq!(list, vec![0, 1, 2, 3, 4]);
    }
    #[test]
    fn test_list_hylo_factorial() {
        let alg: &dyn Fn(Option<(u64, u64)>) -> u64 = &|opt| match opt {
            None => 1,
            Some((a, acc)) => a * acc,
        };
        let coalg: &dyn Fn(u64) -> Option<(u64, u64)> = &|n| {
            if n == 0 {
                None
            } else {
                Some((n, n - 1))
            }
        };
        let fact5 = list_hylo(alg, coalg, 5u64);
        assert_eq!(fact5, 120);
    }
    #[test]
    fn test_list_para_reverse_list() {
        let result = list_para(
            |opt: Option<(i32, Vec<i32>, usize)>| match opt {
                None => 0,
                Some((_, tail, _)) => tail.len() + 1,
            },
            vec![1, 2, 3],
        );
        assert_eq!(result, 3);
    }
    #[test]
    fn test_lens_view_set() {
        let lens: Lens<(i32, i32), i32> = Lens::new(|(a, _)| *a, |v, (_, b)| (v, b));
        let pair = (10, 20);
        assert_eq!(lens.view(&pair), 10);
        let pair2 = lens.set(99, pair);
        assert_eq!(pair2, (99, 20));
    }
    #[test]
    fn test_lens_over() {
        let lens: Lens<(i32, i32), i32> = Lens::new(|(a, _)| *a, |v, (_, b)| (v, b));
        let pair = (5, 7);
        let pair2 = lens.over(|a| a * 2, pair);
        assert_eq!(pair2, (10, 7));
    }
    #[test]
    fn test_prism_some() {
        let prism: Prism<Option<i32>, i32> = Prism::new(|s| s, |a| Some(a));
        assert_eq!(prism.preview(Some(42)), Some(42));
        assert_eq!(prism.preview(None), None);
        assert_eq!(prism.review(7), Some(7));
    }
    #[test]
    fn test_iso_swap() {
        let iso: Iso<(i32, i32), (i32, i32)> = Iso::new(|(a, b)| (b, a), |(b, a)| (a, b));
        assert_eq!(iso.view((1, 2)), (2, 1));
        assert_eq!(iso.review((2, 1)), (1, 2));
    }
    #[test]
    fn test_traversal_over() {
        let trav: Traversal<Vec<i32>, i32> =
            Traversal::new(|v: &Vec<i32>| v.clone(), |vals, _| vals);
        let v = vec![1, 2, 3];
        let v2 = trav.over(|x| x * 10, v);
        assert_eq!(v2, vec![10, 20, 30]);
    }
    #[test]
    fn test_hlist_len() {
        let list = HList::cons(42i32, HList::cons("hello", HList::nil()));
        assert_eq!(list.len(), 2);
    }
    #[test]
    fn test_hlist_empty() {
        let list = HList::nil();
        assert!(list.is_empty());
    }
    #[test]
    fn test_hmap_insert_get() {
        let mut map = HMap::new();
        map.insert("x", 42i32);
        map.insert("name", "OxiLean");
        assert_eq!(map.get::<i32>("x"), Some(&42));
        assert_eq!(map.get::<&str>("name"), Some(&"OxiLean"));
        assert_eq!(map.len(), 2);
    }
    #[test]
    fn test_singleton() {
        let s = Singleton::new(2.72f64);
        assert!((s.extract() - 2.72).abs() < 1e-15);
    }
    #[test]
    fn test_type_equality_refl() {
        let _eq: TypeEquality<i32, i32> = TypeEquality::refl();
    }
    #[test]
    fn test_arrow_compose() {
        let double = Arrow::arr(|x: i32| x * 2);
        let add_one = Arrow::arr(|x: i32| x + 1);
        let composed = double.compose(add_one);
        assert_eq!(composed.apply(5), 11);
    }
    #[test]
    fn test_build_env() {
        use oxilean_kernel::Environment;
        let mut env = Environment::new();
        build_env(&mut env);
        assert!(env.get(&Name::str("FreeMonad")).is_some());
        assert!(env.get(&Name::str("Lens")).is_some());
        assert!(env.get(&Name::str("HList")).is_some());
        assert!(env.get(&Name::str("Effect")).is_some());
        assert!(env.get(&Name::str("Mu")).is_some());
        assert!(env.get(&Name::str("ScottDomain")).is_some());
        assert!(env.get(&Name::str("EffectRow")).is_some());
        assert!(env.get(&Name::str("FreerMonad")).is_some());
        assert!(env.get(&Name::str("Profunctor")).is_some());
        assert!(env.get(&Name::str("Comonad")).is_some());
        assert!(env.get(&Name::str("Applicative")).is_some());
        assert!(env.get(&Name::str("ArrowChoice")).is_some());
        assert!(env.get(&Name::str("ContT")).is_some());
        assert!(env.get(&Name::str("UniqueType")).is_some());
    }
    #[test]
    fn test_recursion_scheme_cata_sum() {
        let tree = RoseTree::Node(
            1usize,
            vec![RoseTree::Node(2, vec![]), RoseTree::Node(3, vec![])],
        );
        let sum = RecursionSchemeEval::cata(tree, &|v: usize, children: Vec<usize>| {
            v + children.iter().sum::<usize>()
        });
        assert_eq!(sum, 6);
    }
    #[test]
    fn test_recursion_scheme_ana_binary_node_count() {
        let tree = RecursionSchemeEval::ana(2usize, &|d: usize| {
            if d == 0 {
                (d, vec![])
            } else {
                (d, vec![d - 1, d - 1])
            }
        });
        let node_count = RecursionSchemeEval::cata(tree, &|_: usize, children: Vec<usize>| {
            1 + children.iter().sum::<usize>()
        });
        assert_eq!(node_count, 7);
    }
    #[test]
    fn test_recursion_scheme_hylo_path_sum() {
        let result = RecursionSchemeEval::hylo(
            3usize,
            &|d: usize| {
                if d == 0 {
                    (d, vec![])
                } else {
                    (d, vec![d - 1])
                }
            },
            &|v: usize, children: Vec<usize>| v + children.iter().sum::<usize>(),
        );
        assert_eq!(result, 6);
    }
    #[test]
    fn test_lens_composer_get_set_law() {
        let lens: Lens<(i32, i32), i32> = Lens::new(|(a, _)| *a, |v, (_, b)| (v, b));
        assert!(LensComposer::check_get_set(&lens, (42, 7)));
    }
    #[test]
    fn test_lens_composer_set_get_law() {
        let lens: Lens<(i32, i32), i32> = Lens::new(|(a, _)| *a, |v, (_, b)| (v, b));
        assert!(LensComposer::check_set_get(&lens, 99, (1, 2)));
    }
    #[test]
    fn test_lens_composer_set_set_law() {
        let lens: Lens<(i32, i32), i32> = Lens::new(|(a, _)| *a, |v, (_, b)| (v, b));
        assert!(LensComposer::check_set_set(&lens, 10, 20, (1, 2)));
    }
    #[test]
    fn test_lens_composer_compose() {
        let outer: Lens<((i32, i32), i32), (i32, i32)> =
            Lens::new(|(pair, _)| *pair, |v, (_, b)| (v, b));
        let inner: Lens<(i32, i32), i32> = Lens::new(|(a, _)| *a, |v, (_, b)| (v, b));
        let composed = LensComposer::compose(outer, inner);
        let s = ((10, 20), 30);
        assert_eq!(composed(&s), 10);
    }
    #[test]
    fn test_free_monad_interpreter_print() {
        let prog = ConsoleProg::Step(
            ConsoleEffect::Print("hello".to_string()),
            Box::new(|_| ConsoleProg::Done(42i32)),
        );
        let mut output: Vec<String> = Vec::new();
        let result = FreeMonadInterpreter::run(
            prog,
            &mut |msg: &str| output.push(msg.to_string()),
            &mut || String::new(),
        );
        assert_eq!(result, 42);
        assert_eq!(output, vec!["hello"]);
    }
    #[test]
    fn test_free_monad_interpreter_read() {
        let prog = ConsoleProg::Step(
            ConsoleEffect::Read,
            Box::new(|line: String| ConsoleProg::Done(line.len())),
        );
        let result = FreeMonadInterpreter::run(prog, &mut |_| {}, &mut || "world".to_string());
        assert_eq!(result, 5);
    }
    #[test]
    fn test_profunctor_optic_lens() {
        let optic: ProfunctorOptic<(i32, i32), (i32, i32), i32, i32> =
            ProfunctorOptic::lens_optic(|(a, _): &(i32, i32)| *a, |v, (_, b): (i32, i32)| (v, b));
        let transform = optic.apply(|x| x * 10);
        let result = transform((3, 7));
        assert_eq!(result, (30, 7));
    }
    #[test]
    fn test_profunctor_optic_prism_some() {
        let optic: ProfunctorOptic<Option<i32>, Option<i32>, i32, i32> =
            ProfunctorOptic::prism_optic(|s: Option<i32>| s.ok_or(None), |b: i32| Some(b));
        let transform = optic.apply(|x| x + 1);
        assert_eq!(transform(Some(41)), Some(42));
    }
    #[test]
    fn test_profunctor_optic_prism_none() {
        let optic: ProfunctorOptic<Option<i32>, Option<i32>, i32, i32> =
            ProfunctorOptic::prism_optic(|s: Option<i32>| s.ok_or(None), |b: i32| Some(b));
        let transform = optic.apply(|x| x + 1);
        assert_eq!(transform(None), None);
    }
    #[test]
    fn test_zipper_extract() {
        let z = Zipper::new(vec![1, 2, 3, 4, 5], 2).expect("Zipper::new should succeed");
        assert_eq!(*z.extract(), 3);
    }
    #[test]
    fn test_zipper_move_left_right() {
        let z = Zipper::new(vec![10, 20, 30], 1).expect("Zipper::new should succeed");
        let z_left = z.move_left().expect("move_left should succeed");
        assert_eq!(*z_left.extract(), 10);
        let z_right = z.move_right().expect("move_right should succeed");
        assert_eq!(*z_right.extract(), 30);
    }
    #[test]
    fn test_zipper_to_vec() {
        let z = Zipper::new(vec![1, 2, 3], 0).expect("Zipper::new should succeed");
        assert_eq!(z.to_vec(), vec![1, 2, 3]);
    }
    #[test]
    fn test_comonad_extend_sum_neighbors() {
        let z = Zipper::new(vec![1i32, 2, 3, 4, 5], 2).expect("Zipper::new should succeed");
        let extended = ComonadExtend::extend(&z, |sub_z| {
            let left_val: i32 = sub_z.left.last().copied().unwrap_or(0);
            let right_val: i32 = sub_z.right.first().copied().unwrap_or(0);
            left_val + *sub_z.extract() + right_val
        });
        assert_eq!(extended.to_vec(), vec![3, 6, 9, 12, 9]);
        assert_eq!(*extended.extract(), 9);
    }
    #[test]
    fn test_comonad_duplicate_len() {
        let z = Zipper::new(vec![10, 20, 30], 1).expect("Zipper::new should succeed");
        let dup = ComonadExtend::duplicate(&z);
        assert_eq!(dup.len(), 3);
        assert_eq!(dup.extract().to_vec(), vec![10, 20, 30]);
    }
    #[test]
    fn test_new_axioms_registered() {
        use oxilean_kernel::Environment;
        let mut env = Environment::new();
        build_env(&mut env);
        assert!(env.get(&Name::str("FullAbstraction")).is_some());
        assert!(env.get(&Name::str("ApproximationOrder")).is_some());
        assert!(env.get(&Name::str("FixedPointSemantics")).is_some());
        assert!(env.get(&Name::str("DeepHandler")).is_some());
        assert!(env.get(&Name::str("ShallowHandler")).is_some());
        assert!(env.get(&Name::str("EffectSubrow")).is_some());
        assert!(env.get(&Name::str("FreeApplicative")).is_some());
        assert!(env.get(&Name::str("freer_lift")).is_some());
        assert!(env.get(&Name::str("free_monad_bind")).is_some());
        assert!(env.get(&Name::str("TambaraModule")).is_some());
        assert!(env.get(&Name::str("dimap")).is_some());
        assert!(env.get(&Name::str("StrongProfunctor")).is_some());
        assert!(env.get(&Name::str("VanLaarhovenLens")).is_some());
        assert!(env.get(&Name::str("lens_set_set")).is_some());
        assert!(env.get(&Name::str("prism_law")).is_some());
        assert!(env.get(&Name::str("traversal_compose")).is_some());
        assert!(env.get(&Name::str("StreamComonad")).is_some());
        assert!(env.get(&Name::str("CellularAutomataComonad")).is_some());
        assert!(env.get(&Name::str("cofree_comonad_unfold")).is_some());
        assert!(env.get(&Name::str("DayConvolution")).is_some());
        assert!(env.get(&Name::str("IdiomBracket")).is_some());
        assert!(env.get(&Name::str("applicative_law")).is_some());
        assert!(env.get(&Name::str("ArrowApply")).is_some());
        assert!(env.get(&Name::str("ArrowFirst")).is_some());
        assert!(env.get(&Name::str("arrow_law")).is_some());
        assert!(env.get(&Name::str("initial_algebra")).is_some());
        assert!(env.get(&Name::str("BanachFixedPoint")).is_some());
        assert!(env.get(&Name::str("FixpointType")).is_some());
        assert!(env.get(&Name::str("final_coalgebra")).is_some());
        assert!(env.get(&Name::str("Shift")).is_some());
        assert!(env.get(&Name::str("Reset")).is_some());
        assert!(env.get(&Name::str("CpsTransform")).is_some());
        assert!(env.get(&Name::str("double_negation_translation")).is_some());
        assert!(env.get(&Name::str("LinearType")).is_some());
        assert!(env.get(&Name::str("linearity_law")).is_some());
        assert!(env.get(&Name::str("uniqueness_law")).is_some());
    }
}
#[cfg(test)]
mod extended_fp_tests {
    use super::*;
    #[test]
    fn test_free_monad() {
        let fm = FreeMonadInfo::over("F", vec!["op1", "op2", "op3"]);
        assert_eq!(fm.num_operations(), 3);
        assert!(fm.interpreter_description().contains("foldFree"));
    }
    #[test]
    fn test_cps() {
        let cps = CpsTransform::new("Int", "R");
        assert!(cps.transform_description().contains("CPS[Int]"));
    }
    #[test]
    fn test_effect_system() {
        let st = EffectSystem::state("Int");
        assert_eq!(st.num_ops(), 2);
        let exc = EffectSystem::exception("Error");
        assert_eq!(exc.num_ops(), 1);
    }
    #[test]
    fn test_defunctionalized_closure() {
        let cl = DefunctClosure::new("Adder", vec![("n", "Int")], "n + x");
        assert_eq!(cl.arity(), 1);
        assert!(cl.apply_description().contains("apply(Adder)"));
    }
    #[test]
    fn test_type_constructor_functor() {
        let list = TypeConstructorFunctor::list();
        assert_eq!(list.num_laws(), 2);
        assert!(list.fmap_type.contains("List a -> List b"));
    }
}
#[cfg(test)]
mod tests_fp_ext {
    use super::*;
    #[test]
    fn test_applicative() {
        let maybe = ApplicativeData::maybe_applicative();
        assert!(maybe.is_monad);
        let laws = maybe.laws();
        assert_eq!(laws.len(), 4);
        assert!(laws[0].contains("Identity"));
        let paper = maybe.mcbride_paterson_paper();
        assert!(paper.contains("McBride"));
        let val = ApplicativeData::validation_applicative();
        assert!(!val.is_monad);
    }
    #[test]
    fn test_traversable() {
        let list_trav = TraversableData::list_traversable();
        assert!(list_trav.is_foldable);
        let laws = list_trav.laws();
        assert!(laws.len() >= 3);
        let acc = list_trav.efficient_mapaccum();
        assert!(acc.contains("O(n)"));
    }
    #[test]
    fn test_arrow_data() {
        let arr = ArrowData::function_arrow();
        assert!(arr.is_arrowchoice && arr.is_arrowloop);
        let laws = arr.hughes_laws();
        assert_eq!(laws.len(), 4);
        let freyd = arr.freyd_category_connection();
        assert!(freyd.contains("Freyd"));
        let kleisli = ArrowData::kleisli_arrow("Maybe");
        assert!(!kleisli.is_arrowloop);
    }
    #[test]
    fn test_profunctor() {
        let fp = ProfunctorData::function_profunctor();
        assert!(fp.is_cartesian && fp.is_closed);
        let optic = fp.optic_encoding();
        assert!(optic.contains("Profunctor optics"));
        let tambara = fp.tambara_module_connection();
        assert!(tambara.contains("Tambara"));
        let star = ProfunctorData::star_profunctor("Maybe");
        assert!(!star.is_closed);
    }
    #[test]
    fn test_dependent_type() {
        let vec5 = DependentTypeExample::fixed_length_vector("Int", 5);
        let safety = vec5.type_safety_guarantee();
        assert!(safety.contains("length is exactly 5"));
        let fin = DependentTypeExample::fin_type(10);
        assert!(fin.type_name.contains("Fin 10"));
    }
    #[test]
    fn test_homotopy_equivalence() {
        let heq = HomotopyEquivalence::new("A", "B", "f", "g").univalent_equivalence();
        assert!(heq.is_univalent);
        let ua = heq.univalence_axiom();
        assert!(ua.contains("Voevodsky"));
        let cond = heq.contractibility_condition();
        assert!(cond.contains("≃"));
    }
}
