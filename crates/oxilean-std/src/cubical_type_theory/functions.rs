//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
#![allow(clippy::items_after_test_module)]

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{CubicalEquiv, CubicalPath, CubicalSet, HcompBox, IntervalPoint, PathType, HIT};

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
pub fn app5(f: Expr, a: Expr, b: Expr, c: Expr, d: Expr, e: Expr) -> Expr {
    app(app4(f, a, b, c, d), e)
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
/// `IType : Type` — the interval object I = {0,1} with a de Morgan structure.
///
/// In cubical type theory the interval I is a primitive pretype.
/// It is equipped with endpoints `i0 : I`, `i1 : I`, and connection
/// operations `i ∧ j`, `i ∨ j`, `~ i`.
pub fn interval_type_ty() -> Expr {
    type0()
}
/// `i0 : I` — the left endpoint of the interval.
pub fn i0_ty() -> Expr {
    cst("IType")
}
/// `i1 : I` — the right endpoint of the interval.
pub fn i1_ty() -> Expr {
    cst("IType")
}
/// `IMin : I → I → I` — the meet (and / ∧) operation on the interval.
///
/// This gives the de Morgan lattice structure: `i ∧ 0 = 0`, `i ∧ 1 = i`.
pub fn i_min_ty() -> Expr {
    arrow(cst("IType"), arrow(cst("IType"), cst("IType")))
}
/// `IMax : I → I → I` — the join (or / ∨) operation on the interval.
///
/// Satisfies: `i ∨ 0 = i`, `i ∨ 1 = 1`.
pub fn i_max_ty() -> Expr {
    arrow(cst("IType"), arrow(cst("IType"), cst("IType")))
}
/// `INeg : I → I` — the negation (~ / complement) on the interval.
///
/// Satisfies de Morgan laws: `~(i ∧ j) = ~i ∨ ~j`, `~~i = i`.
pub fn i_neg_ty() -> Expr {
    arrow(cst("IType"), cst("IType"))
}
/// `FaceFormula : Type` — a face formula φ built from interval variables.
///
/// Face formulas are disjunctions of conjunctions of literals (i=0) or (i=1).
/// A formula φ represents a face of a cube; when φ=1 the whole cube is present.
pub fn face_formula_ty() -> Expr {
    type0()
}
/// `PartialType : I → Type → Type` — partial elements defined on face formula φ.
///
/// `Partial φ A` is inhabited by elements `a : A` that are "only defined when φ holds".
/// This is the key device for specifying open box fillers.
pub fn partial_type_ty() -> Expr {
    arrow(cst("IType"), arrow(type0(), type1()))
}
/// `PartialElem : ∀ (φ : I) (A : Type), Partial φ A → A` — extract from partial.
///
/// When φ is satisfied, the partial element is an actual element of A.
pub fn partial_elem_ty() -> Expr {
    impl_pi(
        "φ",
        cst("IType"),
        impl_pi(
            "A",
            type0(),
            arrow(app2(cst("PartialType"), bvar(1), bvar(0)), bvar(0)),
        ),
    )
}
/// `PathType : ∀ (A : Type) (a b : A), Type`
///
/// The path type `Path A a b` is the type of paths from a to b in A.
/// Unlike Martin-Löf identity types, paths have a *computational* interpretation:
/// a path is literally a function `I → A` sending 0 to a and 1 to b.
pub fn path_type_ty() -> Expr {
    impl_pi("A", type0(), arrow(bvar(0), arrow(bvar(1), type0())))
}
/// `PathLam : ∀ {A : I → Type} (f : ∀ i : I, A i), Path (A 0) (f 0) (f 1)`
///
/// Path abstraction: given a line `f : I → A`, form a path `<i> f i`.
pub fn path_lam_ty() -> Expr {
    impl_pi(
        "A",
        arrow(cst("IType"), type0()),
        arrow(
            pi(
                BinderInfo::Default,
                "i",
                cst("IType"),
                app(bvar(1), bvar(0)),
            ),
            app3(
                cst("PathType"),
                app(bvar(0), cst("i0")),
                app(bvar(1), cst("i0")),
                app(bvar(1), cst("i1")),
            ),
        ),
    )
}
/// `PathApp : ∀ {A : Type} {a b : A}, Path A a b → I → A`
///
/// Path application: evaluate a path at a point `i : I`.
/// This is the eliminator for path types and gives the computational content.
pub fn path_app_ty() -> Expr {
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
                    app3(cst("PathType"), bvar(2), bvar(1), bvar(0)),
                    arrow(cst("IType"), bvar(3)),
                ),
            ),
        ),
    )
}
/// `Refl : ∀ {A : Type} (a : A), Path A a a`
///
/// The reflexivity path `<i> a` — constant path at a.
pub fn refl_path_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        pi(
            BinderInfo::Default,
            "a",
            bvar(0),
            app3(cst("PathType"), bvar(1), bvar(0), bvar(0)),
        ),
    )
}
/// `Sym : ∀ {A : Type} {a b : A}, Path A a b → Path A b a`
///
/// Path symmetry via interval negation: `<i> p (~ i)`.
pub fn sym_ty() -> Expr {
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
                    app3(cst("PathType"), bvar(2), bvar(1), bvar(0)),
                    app3(cst("PathType"), bvar(3), bvar(1), bvar(2)),
                ),
            ),
        ),
    )
}
/// `Trans : ∀ {A : Type} {a b c : A}, Path A a b → Path A b c → Path A a c`
///
/// Path composition via the connection `<i> comp A (p i) (q i)`.
pub fn trans_ty() -> Expr {
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
                        app3(cst("PathType"), bvar(3), bvar(2), bvar(1)),
                        arrow(
                            app3(cst("PathType"), bvar(4), bvar(2), bvar(1)),
                            app3(cst("PathType"), bvar(5), bvar(4), bvar(2)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `Transport : ∀ (A : I → Type) (i : I), A i0 → A i`
///
/// Transport coerces an element across a line of types.
/// In cubical TT this is *computational*: `transp (<_> A) φ a = a`
/// when A does not depend on the interval variable.
pub fn transport_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        arrow(cst("IType"), type0()),
        pi(
            BinderInfo::Default,
            "i",
            cst("IType"),
            arrow(app(bvar(1), cst("i0")), app(bvar(2), bvar(0))),
        ),
    )
}
/// `Transp : ∀ (A : I → Type) (φ : I) (a : A i0), A i1`
///
/// The `transp` primitive of CCHM: transport with a side condition φ.
/// When φ = i1, transport acts as the identity.
pub fn transp_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        arrow(cst("IType"), type0()),
        pi(
            BinderInfo::Default,
            "φ",
            cst("IType"),
            arrow(app(bvar(1), cst("i0")), app(bvar(2), cst("i1"))),
        ),
    )
}
/// `Coe : ∀ {r s : I} (A : I → Type), A r → A s`
///
/// Coercion along a path of types (variant used in RedTT / cooltt).
pub fn coe_ty() -> Expr {
    impl_pi(
        "r",
        cst("IType"),
        impl_pi(
            "s",
            cst("IType"),
            pi(
                BinderInfo::Default,
                "A",
                arrow(cst("IType"), type0()),
                arrow(app(bvar(0), bvar(2)), app(bvar(1), bvar(2))),
            ),
        ),
    )
}
/// `Hcomp : ∀ (A : Type) (φ : I) (u : I → Partial φ A) (a : A), A`
///
/// Homogeneous composition fills an open box.
/// Given a type A, a face formula φ, a tube `u : I → Partial φ A`,
/// and a base `a : A` with `a = u i0 [φ]`, produces an element of A.
///
/// This is the core of the Kan condition in CCHM cubical type theory.
#[allow(clippy::too_many_arguments)]
pub fn hcomp_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        pi(
            BinderInfo::Default,
            "φ",
            cst("IType"),
            arrow(
                arrow(cst("IType"), app2(cst("PartialType"), bvar(0), bvar(1))),
                arrow(bvar(1), bvar(2)),
            ),
        ),
    )
}
/// `Comp : ∀ (A : I → Type) (φ : I) (u : ∀ i, Partial φ (A i)) (a : A i0), A i1`
///
/// Heterogeneous composition (combines transp + hcomp).
/// This is the full Kan composition operation for dependent types.
#[allow(clippy::too_many_arguments)]
pub fn comp_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        arrow(cst("IType"), type0()),
        pi(
            BinderInfo::Default,
            "φ",
            cst("IType"),
            arrow(
                pi(
                    BinderInfo::Default,
                    "i",
                    cst("IType"),
                    app2(cst("PartialType"), bvar(1), app(bvar(2), bvar(0))),
                ),
                arrow(app(bvar(1), cst("i0")), app(bvar(2), cst("i1"))),
            ),
        ),
    )
}
/// `KanFill : ∀ (A : Type) (φ : I) (u : I → Partial φ A) (a : A) (i : I), A`
///
/// The Kan *filler* produces a path inside A that witnesses the composition.
/// If `hcomp` gives the cap of the filled box, `fill` gives the entire cube.
pub fn kan_fill_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        pi(
            BinderInfo::Default,
            "φ",
            cst("IType"),
            arrow(
                arrow(cst("IType"), app2(cst("PartialType"), bvar(0), bvar(1))),
                arrow(bvar(1), arrow(cst("IType"), bvar(3))),
            ),
        ),
    )
}
/// `KanSquare : ∀ (A : Type), Type`
///
/// An element witnessing that every open square (2-cube with one face removed)
/// can be filled — i.e., the Kan condition for 2-cubes.
pub fn kan_square_ty() -> Expr {
    arrow(type0(), type0())
}
/// `GlueType : ∀ (φ : I) (A : Type) (B : Partial φ Type) (e : Partial φ (B ≃ A)), Type`
///
/// Glue types implement a form of univalence *computationally*.
/// Given an equivalence `e : B ≃ A` valid on face φ, `Glue φ A B e`
/// is a type that "glues" B onto A along the face φ.
///
/// This is the key construction that makes univalence hold *definitionally*
/// (or at least computationally) in cubical type theory.
pub fn glue_type_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "φ",
        cst("IType"),
        arrow(
            type0(),
            arrow(app2(cst("PartialType"), bvar(0), type0()), type0()),
        ),
    )
}
/// `Glue : ∀ {φ : I} {A B : Type} {e : B ≃ A}, B → Glue φ A B e`
///
/// Introduction rule for Glue types: if we have b : B, we can introduce it
/// as an element of the glued type.
pub fn glue_intro_ty() -> Expr {
    impl_pi(
        "φ",
        cst("IType"),
        impl_pi(
            "A",
            type0(),
            impl_pi(
                "B",
                type0(),
                arrow(bvar(0), app3(cst("GlueType"), bvar(2), bvar(1), bvar(0))),
            ),
        ),
    )
}
/// `UnGlue : ∀ {φ : I} {A B : Type} {e : B ≃ A}, Glue φ A B e → A`
///
/// Elimination rule for Glue types: project back to the base type A.
/// When φ = i1, unglue applies the equivalence e to extract the A-value.
pub fn unglue_ty() -> Expr {
    impl_pi(
        "φ",
        cst("IType"),
        impl_pi(
            "A",
            type0(),
            impl_pi(
                "B",
                type0(),
                arrow(app3(cst("GlueType"), bvar(2), bvar(1), bvar(0)), bvar(1)),
            ),
        ),
    )
}
/// `IsEquiv : ∀ {A B : Type} (f : A → B), Type`
///
/// A function f : A → B is an equivalence if it has a contractible fiber:
/// `IsEquiv f = ∀ y : B, IsContr (fiber f y)`.
/// In cubical TT this notion is computational.
pub fn is_equiv_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi("B", type0(), arrow(arrow(bvar(1), bvar(0)), type0())),
    )
}
/// `Equiv : ∀ (A B : Type), Type`
///
/// The type of equivalences: `A ≃ B = Σ (f : A → B), IsEquiv f`.
pub fn equiv_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// `IsContr : ∀ (A : Type), Type`
///
/// A type is contractible if it has a center of contraction:
/// `IsContr A = Σ (a : A), ∀ b : A, Path A a b`.
pub fn is_contr_ty() -> Expr {
    arrow(type0(), type0())
}
/// `IsProp : ∀ (A : Type), Type`
///
/// A type is a proposition ((-1)-type) if any two elements are path-equal.
/// `IsProp A = ∀ (a b : A), Path A a b`.
pub fn is_prop_ty() -> Expr {
    arrow(type0(), type0())
}
/// `IsSet : ∀ (A : Type), Type`
///
/// A type is a set (0-type / discrete type) if all its path spaces are propositions.
/// `IsSet A = ∀ (a b : A), IsProp (Path A a b)`.
pub fn is_set_ty() -> Expr {
    arrow(type0(), type0())
}
/// `Fiber : ∀ {A B : Type} (f : A → B) (y : B), Type`
///
/// The fiber of f over y: `Fiber f y = Σ (x : A), Path B (f x) y`.
pub fn fiber_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "B",
            type0(),
            arrow(arrow(bvar(1), bvar(0)), arrow(bvar(0), type0())),
        ),
    )
}
/// `UA : ∀ {A B : Type} (e : A ≃ B), Path Type A B`
///
/// Univalence: every equivalence is a path between types.
/// In CCHM cubical TT, this is *proved* (not postulated) using Glue types.
/// The construction: `ua e = <i> Glue i B A e`.
pub fn ua_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "B",
            type0(),
            arrow(
                app2(cst("Equiv"), bvar(1), bvar(0)),
                app3(cst("PathType"), type0(), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// `UABeta : ∀ {A B : Type} (e : A ≃ B) (a : A), Path B (transp (ua e) a) (fst e a)`
///
/// The computation rule for ua: transporting along a ua-path computes to
/// applying the underlying function of the equivalence.
pub fn ua_beta_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "B",
            type0(),
            arrow(
                app2(cst("Equiv"), bvar(1), bvar(0)),
                arrow(bvar(1), app3(cst("PathType"), bvar(1), bvar(0), bvar(0))),
            ),
        ),
    )
}
/// `IdToEquiv : ∀ {A B : Type}, Path Type A B → A ≃ B`
///
/// The inverse direction of ua: a path between types gives an equivalence.
pub fn id_to_equiv_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi(
            "B",
            type0(),
            arrow(
                app3(cst("PathType"), type0(), bvar(1), bvar(0)),
                app2(cst("Equiv"), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// `CircleCTT : Type` — the circle S¹ as a HIT in cubical TT.
///
/// Constructors: `base : S¹` and `loop : Path S¹ base base`.
/// In cubical TT, HITs are defined via higher constructors directly.
pub fn circle_ctt_ty() -> Expr {
    type0()
}
/// `CircleBase : CircleCTT` — the basepoint of the circle.
pub fn circle_base_ty() -> Expr {
    cst("CircleCTT")
}
/// `CircleLoop : Path CircleCTT base base` — the generating loop.
pub fn circle_loop_ty() -> Expr {
    app3(
        cst("PathType"),
        cst("CircleCTT"),
        cst("CircleBase"),
        cst("CircleBase"),
    )
}
/// `CircleInd : ∀ (P : CircleCTT → Type) (b : P base) (l : Path (P base) b b), ∀ x, P x`
///
/// The cubical induction principle for the circle. The path `l` must be
/// over the loop, which in cubical TT is stated as a path in P base.
pub fn circle_ind_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        arrow(cst("CircleCTT"), type0()),
        pi(
            BinderInfo::Default,
            "b",
            app(bvar(0), cst("CircleBase")),
            arrow(
                app3(
                    cst("PathType"),
                    app(bvar(1), cst("CircleBase")),
                    bvar(0),
                    bvar(0),
                ),
                pi(
                    BinderInfo::Default,
                    "x",
                    cst("CircleCTT"),
                    app(bvar(3), bvar(0)),
                ),
            ),
        ),
    )
}
/// `IntervalHIT : Type` — the interval as a HIT (different from the primitive I).
///
/// Two constructors `left` and `right` with a path between them:
/// this is contractible, hence a model for 2-HIT theory.
pub fn interval_hit_ty() -> Expr {
    type0()
}
/// `Suspension : Type → Type` — the suspension ΣX of a type X.
///
/// Constructors: `north south : ΣX`, `merid : X → Path (ΣX) north south`.
pub fn suspension_ctt_ty() -> Expr {
    arrow(type0(), type0())
}
/// `SuspMerid : ∀ {X : Type} (x : X), Path (Susp X) north south`
///
/// The meridian path constructor of the suspension.
pub fn susp_merid_ty() -> Expr {
    impl_pi(
        "X",
        type0(),
        arrow(
            bvar(0),
            app3(
                cst("PathType"),
                app(cst("Suspension"), bvar(0)),
                cst("SuspNorth"),
                cst("SuspSouth"),
            ),
        ),
    )
}
/// `Pushout : ∀ {A B C : Type} (f : C → A) (g : C → B), Type`
///
/// The homotopy pushout (amalgamated sum) A ⊔_C B as a cubical HIT.
pub fn pushout_ctt_ty() -> Expr {
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
                    arrow(bvar(0), bvar(2)),
                    arrow(arrow(bvar(1), bvar(2)), type0()),
                ),
            ),
        ),
    )
}
/// `CCHMComp : Type` — marker for CCHM-style composition.
///
/// In CCHM (Cohen-Coquand-Huber-Mörtberg), composition is governed by
/// a Kan condition using De Morgan algebras on the interval.
pub fn cchm_comp_ty() -> Expr {
    type0()
}
/// `CHMFill : Type` — marker for CHM-style filling.
///
/// In CHM (Coquand-Huber-Mörtberg), the interval lacks negation
/// and the theory uses "regular" compositions.
pub fn chm_fill_ty() -> Expr {
    type0()
}
/// `PathOver : ∀ (A : I → Type) (i : I) {a : A i0} {b : A i1}, Type`
///
/// A path over a line of types (heterogeneous path / PathP).
/// `PathOver A i a b = Path (A i) ... ` but this is the CCHM PathP type.
pub fn path_over_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        arrow(cst("IType"), type0()),
        impl_pi(
            "a",
            app(bvar(0), cst("i0")),
            impl_pi(
                "b",
                app(bvar(1), cst("i1")),
                pi(
                    BinderInfo::Default,
                    "i",
                    cst("IType"),
                    app(bvar(3), bvar(0)),
                ),
            ),
        ),
    )
}
/// `SubType : ∀ (A : Type) (φ : I) (a : Partial φ A), Type`
///
/// The sub-type `Sub A φ a` is the type of elements of A that are definitionally
/// equal to the partial element `a` when φ is satisfied.
pub fn sub_type_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        pi(
            BinderInfo::Default,
            "φ",
            cst("IType"),
            arrow(app2(cst("PartialType"), bvar(0), bvar(1)), type0()),
        ),
    )
}
/// Register all cubical type theory axioms into the given environment.
pub fn build_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("IType", interval_type_ty()),
        ("i0", i0_ty()),
        ("i1", i1_ty()),
        ("IMin", i_min_ty()),
        ("IMax", i_max_ty()),
        ("INeg", i_neg_ty()),
        ("FaceFormula", face_formula_ty()),
        ("PartialType", partial_type_ty()),
        ("PartialElem", partial_elem_ty()),
        ("PathType", path_type_ty()),
        ("PathLam", path_lam_ty()),
        ("PathApp", path_app_ty()),
        ("ReflPath", refl_path_ty()),
        ("Sym", sym_ty()),
        ("Trans", trans_ty()),
        ("Transport", transport_ty()),
        ("Transp", transp_ty()),
        ("Coe", coe_ty()),
        ("Hcomp", hcomp_ty()),
        ("Comp", comp_ty()),
        ("KanFill", kan_fill_ty()),
        ("KanSquare", kan_square_ty()),
        ("GlueType", glue_type_ty()),
        ("GlueIntro", glue_intro_ty()),
        ("UnGlue", unglue_ty()),
        ("IsEquiv", is_equiv_ty()),
        ("Equiv", equiv_ty()),
        ("IsContr", is_contr_ty()),
        ("IsProp", is_prop_ty()),
        ("IsSet", is_set_ty()),
        ("Fiber", fiber_ty()),
        ("UA", ua_ty()),
        ("UABeta", ua_beta_ty()),
        ("IdToEquiv", id_to_equiv_ty()),
        ("CircleCTT", circle_ctt_ty()),
        ("CircleBase", circle_base_ty()),
        ("CircleLoop", circle_loop_ty()),
        ("CircleInd", circle_ind_ty()),
        ("IntervalHIT", interval_hit_ty()),
        ("Suspension", suspension_ctt_ty()),
        ("SuspMerid", susp_merid_ty()),
        ("Pushout", pushout_ctt_ty()),
        ("CCHMComp", cchm_comp_ty()),
        ("CHMFill", chm_fill_ty()),
        ("PathOver", path_over_ty()),
        ("SubType", sub_type_ty()),
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
    fn test_interval_registered() {
        let mut env = Environment::new();
        build_env(&mut env);
        assert!(
            env.get(&Name::str("IType")).is_some(),
            "IType must be registered"
        );
        assert!(env.get(&Name::str("i0")).is_some(), "i0 must be registered");
        assert!(env.get(&Name::str("i1")).is_some(), "i1 must be registered");
    }
    #[test]
    fn test_path_types_registered() {
        let mut env = Environment::new();
        build_env(&mut env);
        assert!(
            env.get(&Name::str("PathType")).is_some(),
            "PathType must be registered"
        );
        assert!(
            env.get(&Name::str("ReflPath")).is_some(),
            "ReflPath must be registered"
        );
        assert!(
            env.get(&Name::str("Trans")).is_some(),
            "Trans must be registered"
        );
    }
    #[test]
    fn test_hcomp_registered() {
        let mut env = Environment::new();
        build_env(&mut env);
        assert!(
            env.get(&Name::str("Hcomp")).is_some(),
            "Hcomp must be registered"
        );
        assert!(
            env.get(&Name::str("Comp")).is_some(),
            "Comp must be registered"
        );
        assert!(
            env.get(&Name::str("KanFill")).is_some(),
            "KanFill must be registered"
        );
    }
    #[test]
    fn test_glue_and_ua_registered() {
        let mut env = Environment::new();
        build_env(&mut env);
        assert!(
            env.get(&Name::str("GlueType")).is_some(),
            "GlueType must be registered"
        );
        assert!(env.get(&Name::str("UA")).is_some(), "UA must be registered");
        assert!(
            env.get(&Name::str("IdToEquiv")).is_some(),
            "IdToEquiv must be registered"
        );
    }
    #[test]
    fn test_interval_point_simplification() {
        let i = IntervalPoint::Var("i".to_string());
        let zero = IntervalPoint::Zero;
        let meet = IntervalPoint::Min(Box::new(i.clone()), Box::new(zero));
        assert!(meet.simplify().is_zero(), "i ∧ 0 should simplify to 0");
        let neg_neg_i = IntervalPoint::Neg(Box::new(IntervalPoint::Neg(Box::new(i.clone()))));
        assert_eq!(neg_neg_i.simplify(), i, "~~i should simplify to i");
        let one = IntervalPoint::One;
        let join = IntervalPoint::Max(Box::new(i.clone()), Box::new(one));
        assert!(join.simplify().is_one(), "i ∨ 1 should simplify to 1");
    }
    #[test]
    fn test_cubical_path_operations() {
        let refl = CubicalPath::refl("Nat", "n");
        assert_eq!(refl.left, refl.right, "refl has equal endpoints");
        let p = CubicalPath {
            type_name: "Nat".into(),
            left: "a".into(),
            right: "b".into(),
            name: Some("p".into()),
        };
        let q = CubicalPath {
            type_name: "Nat".into(),
            left: "b".into(),
            right: "c".into(),
            name: Some("q".into()),
        };
        let trans = p
            .trans(&q)
            .expect("trans should succeed when endpoints match");
        assert_eq!(trans.left, "a");
        assert_eq!(trans.right, "c");
        let sym_p = p.sym();
        assert_eq!(sym_p.left, "b");
        assert_eq!(sym_p.right, "a");
    }
    #[test]
    fn test_cubical_equiv_operations() {
        let id = CubicalEquiv::id("Nat");
        assert_eq!(id.domain, id.codomain);
        let inv_id = id.inv();
        assert_eq!(inv_id.domain, "Nat");
        let e1 = CubicalEquiv::new("A", "B", "f", "g");
        let e2 = CubicalEquiv::new("B", "C", "h", "k");
        let composed = e1.compose(&e2).expect("compose should succeed");
        assert_eq!(composed.domain, "A");
        assert_eq!(composed.codomain, "C");
        let e3 = CubicalEquiv::new("X", "Y", "u", "v");
        assert!(
            e1.compose(&e3).is_none(),
            "compose with wrong domain should fail"
        );
    }
    #[test]
    fn test_hcomp_box() {
        let box_ = HcompBox::new("Nat", "zero")
            .add_tube("i=0", "a")
            .add_tube("i=1", "b");
        assert_eq!(box_.num_faces(), 2);
        let filled = box_.fill();
        assert!(
            filled.contains("Nat"),
            "fill output should mention the type"
        );
    }
    #[test]
    fn test_cubical_set_kan() {
        let pt = CubicalSet::point();
        assert!(pt.is_kan(), "point is Kan");
        assert_eq!(pt.cell_count(0), 1);
        let interval = CubicalSet::interval();
        assert!(interval.is_kan(), "interval is Kan");
        assert_eq!(interval.cell_count(1), 2);
        let circle = CubicalSet::circle();
        assert_eq!(circle.name, "Circle");
    }
}
pub fn ctt_ext_app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
pub fn ctt_ext_app2(f: Expr, a: Expr, b: Expr) -> Expr {
    ctt_ext_app(ctt_ext_app(f, a), b)
}
pub fn ctt_ext_app3(f: Expr, a: Expr, b: Expr, c: Expr) -> Expr {
    ctt_ext_app(ctt_ext_app2(f, a, b), c)
}
pub fn ctt_ext_cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}
pub fn ctt_ext_nat() -> Expr {
    ctt_ext_cst("Nat")
}
pub fn ctt_ext_prop() -> Expr {
    Expr::Sort(Level::zero())
}
pub fn ctt_ext_type0() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
pub fn ctt_ext_pi(name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str(name),
        Box::new(dom),
        Box::new(body),
    )
}
pub fn ctt_ext_impl_pi(name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str(name),
        Box::new(dom),
        Box::new(body),
    )
}
pub fn ctt_ext_arrow(a: Expr, b: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(a),
        Box::new(b),
    )
}
pub fn ctt_ext_bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
/// `IDeMorgan1 : ∀ (i j : I), INeg (IMin i j) = IMax (INeg i) (INeg j)`
///
/// First de Morgan law for the cubical interval.
#[allow(dead_code)]
pub fn axiom_i_de_morgan1_ty() -> Expr {
    ctt_ext_arrow(
        ctt_ext_cst("IType"),
        ctt_ext_arrow(ctt_ext_cst("IType"), ctt_ext_prop()),
    )
}
/// `IDeMorgan2 : ∀ (i j : I), INeg (IMax i j) = IMin (INeg i) (INeg j)`
///
/// Second de Morgan law for the cubical interval.
#[allow(dead_code)]
pub fn axiom_i_de_morgan2_ty() -> Expr {
    ctt_ext_arrow(
        ctt_ext_cst("IType"),
        ctt_ext_arrow(ctt_ext_cst("IType"), ctt_ext_prop()),
    )
}
/// `IMinAssoc : ∀ (i j k : I), IMin (IMin i j) k = IMin i (IMin j k)`
///
/// Associativity of the meet operation.
#[allow(dead_code)]
pub fn axiom_i_min_assoc_ty() -> Expr {
    ctt_ext_arrow(
        ctt_ext_cst("IType"),
        ctt_ext_arrow(
            ctt_ext_cst("IType"),
            ctt_ext_arrow(ctt_ext_cst("IType"), ctt_ext_prop()),
        ),
    )
}
/// `IMaxAssoc : ∀ (i j k : I), IMax (IMax i j) k = IMax i (IMax j k)`
///
/// Associativity of the join operation.
#[allow(dead_code)]
pub fn axiom_i_max_assoc_ty() -> Expr {
    ctt_ext_arrow(
        ctt_ext_cst("IType"),
        ctt_ext_arrow(
            ctt_ext_cst("IType"),
            ctt_ext_arrow(ctt_ext_cst("IType"), ctt_ext_prop()),
        ),
    )
}
/// `IAbsorption : ∀ (i j : I), IMin i (IMax i j) = i`
///
/// Absorption law for the de Morgan lattice on I.
#[allow(dead_code)]
pub fn axiom_i_absorption_ty() -> Expr {
    ctt_ext_arrow(
        ctt_ext_cst("IType"),
        ctt_ext_arrow(ctt_ext_cst("IType"), ctt_ext_prop()),
    )
}
/// `INegInvolution : ∀ (i : I), INeg (INeg i) = i`
///
/// Double negation is identity on the interval.
#[allow(dead_code)]
pub fn axiom_i_neg_involution_ty() -> Expr {
    ctt_ext_arrow(ctt_ext_cst("IType"), ctt_ext_prop())
}
/// `KanFillHet : ∀ (A : I → Type) (φ : I) (u : ∀ i, Partial φ (A i)) (a : A i0) (i : I), A i`
///
/// Heterogeneous Kan filling: produces a path inside A(i) witnessing comp.
#[allow(dead_code)]
pub fn axiom_kan_fill_het_ty() -> Expr {
    ctt_ext_pi(
        "A",
        ctt_ext_arrow(ctt_ext_cst("IType"), ctt_ext_type0()),
        ctt_ext_arrow(
            ctt_ext_cst("IType"),
            ctt_ext_arrow(
                ctt_ext_arrow(ctt_ext_cst("IType"), ctt_ext_type0()),
                ctt_ext_arrow(
                    ctt_ext_app(ctt_ext_bvar(2), ctt_ext_cst("i0")),
                    ctt_ext_arrow(
                        ctt_ext_cst("IType"),
                        ctt_ext_app(ctt_ext_bvar(4), ctt_ext_bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// `KanUniq : ∀ (A : Type) (φ : I) (u : I → Partial φ A) (a : A), hcomp A φ u a = hcomp A φ u a`
///
/// Uniqueness (definitional) of Kan composition.
#[allow(dead_code)]
pub fn axiom_kan_uniq_ty() -> Expr {
    ctt_ext_pi(
        "A",
        ctt_ext_type0(),
        ctt_ext_arrow(
            ctt_ext_cst("IType"),
            ctt_ext_arrow(
                ctt_ext_arrow(ctt_ext_cst("IType"), ctt_ext_type0()),
                ctt_ext_arrow(ctt_ext_bvar(2), ctt_ext_prop()),
            ),
        ),
    )
}
/// `KanCube2 : ∀ (A : Type), Type`
///
/// Kan condition for 2-cubes: every open square can be filled.
#[allow(dead_code)]
pub fn axiom_kan_cube2_ty() -> Expr {
    ctt_ext_arrow(ctt_ext_type0(), ctt_ext_type0())
}
/// `KanHomotopy : ∀ {A B : Type} (f g : A → B), Type`
///
/// A homotopy between two maps in the Kan sense.
#[allow(dead_code)]
pub fn axiom_kan_homotopy_ty() -> Expr {
    ctt_ext_impl_pi(
        "A",
        ctt_ext_type0(),
        ctt_ext_impl_pi(
            "B",
            ctt_ext_type0(),
            ctt_ext_arrow(
                ctt_ext_arrow(ctt_ext_bvar(1), ctt_ext_bvar(0)),
                ctt_ext_arrow(
                    ctt_ext_arrow(ctt_ext_bvar(2), ctt_ext_bvar(1)),
                    ctt_ext_type0(),
                ),
            ),
        ),
    )
}
/// `VType : ∀ (φ : I) (A : Type) (T : Partial φ Type) (e : Partial φ (T ≃ A)), Type`
///
/// V-types are the Cartesian cubical analogue of Glue types.
/// They implement univalence in a Cartesian cubical setting.
#[allow(dead_code)]
pub fn axiom_v_type_ty() -> Expr {
    ctt_ext_pi(
        "φ",
        ctt_ext_cst("IType"),
        ctt_ext_arrow(
            ctt_ext_type0(),
            ctt_ext_arrow(
                ctt_ext_app2(ctt_ext_cst("PartialType"), ctt_ext_bvar(0), ctt_ext_type0()),
                ctt_ext_type0(),
            ),
        ),
    )
}
/// `Vin : ∀ {φ : I} {A : Type} {T : Partial φ Type} (t : T) (a : A), V φ A T e`
///
/// Introduction rule for V-types.
#[allow(dead_code)]
pub fn axiom_vin_ty() -> Expr {
    ctt_ext_impl_pi(
        "φ",
        ctt_ext_cst("IType"),
        ctt_ext_impl_pi(
            "A",
            ctt_ext_type0(),
            ctt_ext_arrow(ctt_ext_type0(), ctt_ext_type0()),
        ),
    )
}
/// `Vproj : ∀ {φ : I} {A : Type} {T : Partial φ Type}, V φ A T e → A`
///
/// Elimination rule for V-types: project to the base type.
#[allow(dead_code)]
pub fn axiom_vproj_ty() -> Expr {
    ctt_ext_impl_pi(
        "φ",
        ctt_ext_cst("IType"),
        ctt_ext_impl_pi(
            "A",
            ctt_ext_type0(),
            ctt_ext_arrow(ctt_ext_type0(), ctt_ext_bvar(0)),
        ),
    )
}
/// `CoeCompat : ∀ {r s t : I} (A : I → Type), Coe r s A ∘ Coe s t A = Coe r t A`
///
/// Coercion is transitive: coercing from r to s and then s to t equals coercion from r to t.
#[allow(dead_code)]
pub fn axiom_coe_compat_ty() -> Expr {
    ctt_ext_impl_pi(
        "r",
        ctt_ext_cst("IType"),
        ctt_ext_impl_pi(
            "s",
            ctt_ext_cst("IType"),
            ctt_ext_impl_pi(
                "t",
                ctt_ext_cst("IType"),
                ctt_ext_arrow(
                    ctt_ext_arrow(ctt_ext_cst("IType"), ctt_ext_type0()),
                    ctt_ext_prop(),
                ),
            ),
        ),
    )
}
/// `TranspIdConst : ∀ (A : Type) (φ : I) (a : A), Transp (<_> A) φ a = a`
///
/// Transport along a constant line is the identity.
#[allow(dead_code)]
pub fn axiom_transp_id_const_ty() -> Expr {
    ctt_ext_pi(
        "A",
        ctt_ext_type0(),
        ctt_ext_arrow(
            ctt_ext_cst("IType"),
            ctt_ext_arrow(ctt_ext_bvar(1), ctt_ext_prop()),
        ),
    )
}
/// `TranspSigma : ∀ (A : I → Type) (B : ∀ i, A i → Type) (φ : I) (u : Σ (A i0) (B i0)), Σ (A i1) (B i1)`
///
/// Transport in sigma types: coerce both components.
#[allow(dead_code)]
pub fn axiom_transp_sigma_ty() -> Expr {
    ctt_ext_arrow(
        ctt_ext_arrow(ctt_ext_cst("IType"), ctt_ext_type0()),
        ctt_ext_arrow(
            ctt_ext_cst("IType"),
            ctt_ext_arrow(ctt_ext_type0(), ctt_ext_type0()),
        ),
    )
}
/// `TranspPi : ∀ (A : I → Type) (B : ∀ i, A i → Type), Transp (Π A B) = ...`
///
/// Transport in pi types (function types).
#[allow(dead_code)]
pub fn axiom_transp_pi_ty() -> Expr {
    ctt_ext_arrow(
        ctt_ext_arrow(ctt_ext_cst("IType"), ctt_ext_type0()),
        ctt_ext_arrow(
            ctt_ext_cst("IType"),
            ctt_ext_arrow(ctt_ext_type0(), ctt_ext_type0()),
        ),
    )
}
/// `PathPType : ∀ (A : I → Type) (a : A i0) (b : A i1), Type`
///
/// The heterogeneous path type (PathP) — a path over a line of types.
/// This is the `PathP` of Agda's cubical library.
#[allow(dead_code)]
pub fn axiom_pathp_type_ty() -> Expr {
    ctt_ext_pi(
        "A",
        ctt_ext_arrow(ctt_ext_cst("IType"), ctt_ext_type0()),
        ctt_ext_arrow(
            ctt_ext_app(ctt_ext_bvar(0), ctt_ext_cst("i0")),
            ctt_ext_arrow(
                ctt_ext_app(ctt_ext_bvar(1), ctt_ext_cst("i1")),
                ctt_ext_type0(),
            ),
        ),
    )
}
/// `PathPLam : ∀ (A : I → Type) (f : ∀ i, A i), PathP A (f i0) (f i1)`
///
/// Introduction rule for PathP.
#[allow(dead_code)]
pub fn axiom_pathp_lam_ty() -> Expr {
    ctt_ext_pi(
        "A",
        ctt_ext_arrow(ctt_ext_cst("IType"), ctt_ext_type0()),
        ctt_ext_arrow(
            ctt_ext_pi(
                "i",
                ctt_ext_cst("IType"),
                ctt_ext_app(ctt_ext_bvar(1), ctt_ext_bvar(0)),
            ),
            ctt_ext_app3(
                ctt_ext_cst("PathPType"),
                ctt_ext_bvar(0),
                ctt_ext_app(ctt_ext_bvar(1), ctt_ext_cst("i0")),
                ctt_ext_app(ctt_ext_bvar(1), ctt_ext_cst("i1")),
            ),
        ),
    )
}
/// `PathPApp : ∀ (A : I → Type) {a : A i0} {b : A i1} (p : PathP A a b) (i : I), A i`
///
/// Elimination rule for PathP: evaluate a heterogeneous path at a point.
#[allow(dead_code)]
pub fn axiom_pathp_app_ty() -> Expr {
    ctt_ext_pi(
        "A",
        ctt_ext_arrow(ctt_ext_cst("IType"), ctt_ext_type0()),
        ctt_ext_arrow(
            ctt_ext_app3(
                ctt_ext_cst("PathPType"),
                ctt_ext_bvar(0),
                ctt_ext_app(ctt_ext_bvar(0), ctt_ext_cst("i0")),
                ctt_ext_app(ctt_ext_bvar(0), ctt_ext_cst("i1")),
            ),
            ctt_ext_arrow(
                ctt_ext_cst("IType"),
                ctt_ext_app(ctt_ext_bvar(2), ctt_ext_bvar(0)),
            ),
        ),
    )
}
/// `SuspNorth : ∀ {X : Type}, Susp X`
///
/// The north pole constructor of the suspension.
#[allow(dead_code)]
pub fn axiom_susp_north_ty() -> Expr {
    ctt_ext_impl_pi(
        "X",
        ctt_ext_type0(),
        ctt_ext_app(ctt_ext_cst("Suspension"), ctt_ext_bvar(0)),
    )
}
/// `SuspSouth : ∀ {X : Type}, Susp X`
///
/// The south pole constructor of the suspension.
#[allow(dead_code)]
pub fn axiom_susp_south_ty() -> Expr {
    ctt_ext_impl_pi(
        "X",
        ctt_ext_type0(),
        ctt_ext_app(ctt_ext_cst("Suspension"), ctt_ext_bvar(0)),
    )
}
/// `SuspInd : ∀ {X : Type} (P : Susp X → Type) (n : P north) (s : P south) (m : ∀ x, PathP (P ∘ merid x) n s), ∀ y, P y`
///
/// Induction principle for the suspension.
#[allow(dead_code)]
pub fn axiom_susp_ind_ty() -> Expr {
    ctt_ext_impl_pi(
        "X",
        ctt_ext_type0(),
        ctt_ext_pi(
            "P",
            ctt_ext_arrow(
                ctt_ext_app(ctt_ext_cst("Suspension"), ctt_ext_bvar(0)),
                ctt_ext_type0(),
            ),
            ctt_ext_arrow(
                ctt_ext_app(ctt_ext_bvar(0), ctt_ext_cst("SuspNorth")),
                ctt_ext_arrow(
                    ctt_ext_app(ctt_ext_bvar(1), ctt_ext_cst("SuspSouth")),
                    ctt_ext_arrow(
                        ctt_ext_arrow(ctt_ext_bvar(3), ctt_ext_type0()),
                        ctt_ext_pi(
                            "y",
                            ctt_ext_app(ctt_ext_cst("Suspension"), ctt_ext_bvar(4)),
                            ctt_ext_app(ctt_ext_bvar(4), ctt_ext_bvar(0)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `Join : ∀ (A B : Type), Type`
///
/// The join A * B as a cubical HIT. Constructors: inl, inr, push.
#[allow(dead_code)]
pub fn axiom_join_ty() -> Expr {
    ctt_ext_arrow(
        ctt_ext_type0(),
        ctt_ext_arrow(ctt_ext_type0(), ctt_ext_type0()),
    )
}
/// `JoinInl : ∀ {A B : Type}, A → Join A B`
///
/// Left inclusion into the join.
#[allow(dead_code)]
pub fn axiom_join_inl_ty() -> Expr {
    ctt_ext_impl_pi(
        "A",
        ctt_ext_type0(),
        ctt_ext_impl_pi(
            "B",
            ctt_ext_type0(),
            ctt_ext_arrow(
                ctt_ext_bvar(1),
                ctt_ext_app2(ctt_ext_cst("Join"), ctt_ext_bvar(1), ctt_ext_bvar(0)),
            ),
        ),
    )
}
/// `JoinInr : ∀ {A B : Type}, B → Join A B`
///
/// Right inclusion into the join.
#[allow(dead_code)]
pub fn axiom_join_inr_ty() -> Expr {
    ctt_ext_impl_pi(
        "A",
        ctt_ext_type0(),
        ctt_ext_impl_pi(
            "B",
            ctt_ext_type0(),
            ctt_ext_arrow(
                ctt_ext_bvar(0),
                ctt_ext_app2(ctt_ext_cst("Join"), ctt_ext_bvar(1), ctt_ext_bvar(0)),
            ),
        ),
    )
}
/// `JoinPush : ∀ {A B : Type} (a : A) (b : B), Path (Join A B) (inl a) (inr b)`
///
/// The path constructor connecting left and right in the join.
#[allow(dead_code)]
pub fn axiom_join_push_ty() -> Expr {
    ctt_ext_impl_pi(
        "A",
        ctt_ext_type0(),
        ctt_ext_impl_pi(
            "B",
            ctt_ext_type0(),
            ctt_ext_arrow(
                ctt_ext_bvar(1),
                ctt_ext_arrow(
                    ctt_ext_bvar(1),
                    ctt_ext_app3(
                        ctt_ext_cst("PathType"),
                        ctt_ext_app2(ctt_ext_cst("Join"), ctt_ext_bvar(3), ctt_ext_bvar(2)),
                        ctt_ext_cst("JoinInl"),
                        ctt_ext_cst("JoinInr"),
                    ),
                ),
            ),
        ),
    )
}
/// `PropTrunc : Type → Type`
///
/// Propositional truncation / (-1)-truncation: `‖A‖₋₁`.
/// The set-quotient that collapses all elements to a single one.
#[allow(dead_code)]
pub fn axiom_prop_trunc_ty() -> Expr {
    ctt_ext_arrow(ctt_ext_type0(), ctt_ext_type0())
}
/// `PT_intro : ∀ {A : Type}, A → ‖A‖`
///
/// Introduction rule for propositional truncation.
#[allow(dead_code)]
pub fn axiom_pt_intro_ty() -> Expr {
    ctt_ext_impl_pi(
        "A",
        ctt_ext_type0(),
        ctt_ext_arrow(
            ctt_ext_bvar(0),
            ctt_ext_app(ctt_ext_cst("PropTrunc"), ctt_ext_bvar(1)),
        ),
    )
}
/// `PT_IsProp : ∀ {A : Type}, IsProp ‖A‖`
///
/// The propositional truncation is a proposition.
#[allow(dead_code)]
pub fn axiom_pt_is_prop_ty() -> Expr {
    ctt_ext_impl_pi(
        "A",
        ctt_ext_type0(),
        ctt_ext_app(
            ctt_ext_cst("IsProp"),
            ctt_ext_app(ctt_ext_cst("PropTrunc"), ctt_ext_bvar(0)),
        ),
    )
}
/// `PT_rec : ∀ {A B : Type}, IsProp B → (A → B) → ‖A‖ → B`
///
/// Recursion principle for propositional truncation.
#[allow(dead_code)]
pub fn axiom_pt_rec_ty() -> Expr {
    ctt_ext_impl_pi(
        "A",
        ctt_ext_type0(),
        ctt_ext_impl_pi(
            "B",
            ctt_ext_type0(),
            ctt_ext_arrow(
                ctt_ext_app(ctt_ext_cst("IsProp"), ctt_ext_bvar(0)),
                ctt_ext_arrow(
                    ctt_ext_arrow(ctt_ext_bvar(1), ctt_ext_bvar(1)),
                    ctt_ext_arrow(
                        ctt_ext_app(ctt_ext_cst("PropTrunc"), ctt_ext_bvar(2)),
                        ctt_ext_bvar(2),
                    ),
                ),
            ),
        ),
    )
}
/// `SetTrunc : Type → Type`
///
/// Set truncation / 0-truncation: `‖A‖₀`.
#[allow(dead_code)]
pub fn axiom_set_trunc_ty() -> Expr {
    ctt_ext_arrow(ctt_ext_type0(), ctt_ext_type0())
}
/// `ST_intro : ∀ {A : Type}, A → ‖A‖₀`
///
/// Introduction rule for set truncation.
#[allow(dead_code)]
pub fn axiom_st_intro_ty() -> Expr {
    ctt_ext_impl_pi(
        "A",
        ctt_ext_type0(),
        ctt_ext_arrow(
            ctt_ext_bvar(0),
            ctt_ext_app(ctt_ext_cst("SetTrunc"), ctt_ext_bvar(1)),
        ),
    )
}
/// `ST_IsSet : ∀ {A : Type}, IsSet ‖A‖₀`
///
/// The set truncation is a set.
#[allow(dead_code)]
pub fn axiom_st_is_set_ty() -> Expr {
    ctt_ext_impl_pi(
        "A",
        ctt_ext_type0(),
        ctt_ext_app(
            ctt_ext_cst("IsSet"),
            ctt_ext_app(ctt_ext_cst("SetTrunc"), ctt_ext_bvar(0)),
        ),
    )
}
/// `EMSpace : Type → Nat → Type`
///
/// The Eilenberg-MacLane space K(G,n): the unique space (up to homotopy)
/// with `π_n = G` and `π_k = 0` for `k ≠ n`.
#[allow(dead_code)]
pub fn axiom_em_space_ty() -> Expr {
    ctt_ext_arrow(
        ctt_ext_type0(),
        ctt_ext_arrow(ctt_ext_nat(), ctt_ext_type0()),
    )
}
/// `EMSpace_pi : ∀ (G : Type) (n : Nat), π_n(K(G,n)) = G`
///
/// The fundamental property: homotopy group equals G.
#[allow(dead_code)]
pub fn axiom_em_space_pi_ty() -> Expr {
    ctt_ext_arrow(
        ctt_ext_type0(),
        ctt_ext_arrow(ctt_ext_nat(), ctt_ext_prop()),
    )
}
/// `BG : Type → Type`
///
/// The classifying space BG = K(G, 1) for a group G.
/// `BG = K(G, 1)` has `π_1 = G` and higher homotopy groups trivial.
#[allow(dead_code)]
pub fn axiom_bg_ty() -> Expr {
    ctt_ext_arrow(ctt_ext_type0(), ctt_ext_type0())
}
/// `LoopSpace : Nat → Type → Type → Type`
///
/// The n-fold loop space `Ω^n(A, a)` based at `a`.
#[allow(dead_code)]
pub fn axiom_loop_space_ty() -> Expr {
    ctt_ext_arrow(
        ctt_ext_nat(),
        ctt_ext_arrow(
            ctt_ext_type0(),
            ctt_ext_arrow(ctt_ext_type0(), ctt_ext_type0()),
        ),
    )
}
/// `HomotopyGroup : Nat → Type → Type → Type`
///
/// The n-th homotopy group `π_n(X, x)` of a pointed type (X, x).
#[allow(dead_code)]
pub fn axiom_homotopy_group_ty() -> Expr {
    ctt_ext_arrow(
        ctt_ext_nat(),
        ctt_ext_arrow(
            ctt_ext_type0(),
            ctt_ext_arrow(ctt_ext_type0(), ctt_ext_type0()),
        ),
    )
}
/// `JamesConstruction : Type → Type`
///
/// The James construction J(X): the free A∞-space on X.
/// J(S^n) ≃ Ω Σ S^n (the loops on the suspension of S^n).
#[allow(dead_code)]
pub fn axiom_james_construction_ty() -> Expr {
    ctt_ext_arrow(ctt_ext_type0(), ctt_ext_type0())
}
/// `FreeGroup : Type → Type`
///
/// The free group on a set of generators, defined as a HIT.
/// Constructors: identity, generators, inverses, multiplication,
/// and path constructors for group laws.
#[allow(dead_code)]
pub fn axiom_free_group_ty() -> Expr {
    ctt_ext_arrow(ctt_ext_type0(), ctt_ext_type0())
}
/// `FreeGroupUnit : ∀ {X : Type}, FreeGroup X`
///
/// The identity element of the free group.
#[allow(dead_code)]
pub fn axiom_free_group_unit_ty() -> Expr {
    ctt_ext_impl_pi(
        "X",
        ctt_ext_type0(),
        ctt_ext_app(ctt_ext_cst("FreeGroup"), ctt_ext_bvar(0)),
    )
}
/// `FreeGroupInl : ∀ {X : Type}, X → FreeGroup X`
///
/// Injection of a generator into the free group.
#[allow(dead_code)]
pub fn axiom_free_group_inl_ty() -> Expr {
    ctt_ext_impl_pi(
        "X",
        ctt_ext_type0(),
        ctt_ext_arrow(
            ctt_ext_bvar(0),
            ctt_ext_app(ctt_ext_cst("FreeGroup"), ctt_ext_bvar(1)),
        ),
    )
}
/// `FreeGroupMul : ∀ {X : Type}, FreeGroup X → FreeGroup X → FreeGroup X`
///
/// Group multiplication in the free group.
#[allow(dead_code)]
pub fn axiom_free_group_mul_ty() -> Expr {
    ctt_ext_impl_pi(
        "X",
        ctt_ext_type0(),
        ctt_ext_arrow(
            ctt_ext_app(ctt_ext_cst("FreeGroup"), ctt_ext_bvar(0)),
            ctt_ext_arrow(
                ctt_ext_app(ctt_ext_cst("FreeGroup"), ctt_ext_bvar(1)),
                ctt_ext_app(ctt_ext_cst("FreeGroup"), ctt_ext_bvar(2)),
            ),
        ),
    )
}
/// `QuotType : ∀ (A : Type) (R : A → A → Type), Type`
///
/// The set quotient A/R as a higher inductive type.
#[allow(dead_code)]
pub fn axiom_quot_type_ty() -> Expr {
    ctt_ext_pi(
        "A",
        ctt_ext_type0(),
        ctt_ext_arrow(
            ctt_ext_arrow(
                ctt_ext_bvar(0),
                ctt_ext_arrow(ctt_ext_bvar(1), ctt_ext_type0()),
            ),
            ctt_ext_type0(),
        ),
    )
}
/// `QuotInj : ∀ {A : Type} {R : A → A → Type}, A → Quot A R`
///
/// Injection of elements into the quotient.
#[allow(dead_code)]
pub fn axiom_quot_inj_ty() -> Expr {
    ctt_ext_impl_pi(
        "A",
        ctt_ext_type0(),
        ctt_ext_arrow(
            ctt_ext_bvar(0),
            ctt_ext_app2(ctt_ext_cst("QuotType"), ctt_ext_bvar(1), ctt_ext_cst("R")),
        ),
    )
}
/// `QuotEq : ∀ {A : Type} {R : A → A → Type} {a b : A}, R a b → Path (Quot A R) (inj a) (inj b)`
///
/// The path constructor: related elements become path-equal in the quotient.
#[allow(dead_code)]
pub fn axiom_quot_eq_ty() -> Expr {
    ctt_ext_impl_pi(
        "A",
        ctt_ext_type0(),
        ctt_ext_arrow(
            ctt_ext_bvar(0),
            ctt_ext_arrow(
                ctt_ext_bvar(1),
                ctt_ext_arrow(ctt_ext_type0(), ctt_ext_prop()),
            ),
        ),
    )
}
/// `QuotIsSet : ∀ {A : Type} {R : A → A → Type}, IsSet (Quot A R)`
///
/// The set quotient is a set (0-truncated).
#[allow(dead_code)]
pub fn axiom_quot_is_set_ty() -> Expr {
    ctt_ext_impl_pi(
        "A",
        ctt_ext_type0(),
        ctt_ext_arrow(
            ctt_ext_arrow(
                ctt_ext_bvar(0),
                ctt_ext_arrow(ctt_ext_bvar(1), ctt_ext_type0()),
            ),
            ctt_ext_app(
                ctt_ext_cst("IsSet"),
                ctt_ext_app2(ctt_ext_cst("QuotType"), ctt_ext_bvar(1), ctt_ext_bvar(0)),
            ),
        ),
    )
}
/// `GroupCompletion : Type → Type`
///
/// The group completion of a monoid M: the universal group receiving M.
#[allow(dead_code)]
pub fn axiom_group_completion_ty() -> Expr {
    ctt_ext_arrow(ctt_ext_type0(), ctt_ext_type0())
}
/// `GC_Inject : ∀ {M : Type}, M → GroupCompletion M`
///
/// Injection of the monoid into its group completion.
#[allow(dead_code)]
pub fn axiom_gc_inject_ty() -> Expr {
    ctt_ext_impl_pi(
        "M",
        ctt_ext_type0(),
        ctt_ext_arrow(
            ctt_ext_bvar(0),
            ctt_ext_app(ctt_ext_cst("GroupCompletion"), ctt_ext_bvar(1)),
        ),
    )
}
/// `IsGroupoid : Type → Type`
///
/// A type is a groupoid (1-type) if all path spaces are sets.
#[allow(dead_code)]
pub fn axiom_is_groupoid_ty() -> Expr {
    ctt_ext_arrow(ctt_ext_type0(), ctt_ext_type0())
}
/// `IsNType : Nat → Type → Type`
///
/// A type is an n-type if all (n+1)-dimensional path spaces are contractible.
#[allow(dead_code)]
pub fn axiom_is_n_type_ty() -> Expr {
    ctt_ext_arrow(
        ctt_ext_nat(),
        ctt_ext_arrow(ctt_ext_type0(), ctt_ext_type0()),
    )
}
/// `ConnectedType : Nat → Type → Type`
///
/// An n-connected type: all homotopy groups below n are trivial.
#[allow(dead_code)]
pub fn axiom_connected_type_ty() -> Expr {
    ctt_ext_arrow(
        ctt_ext_nat(),
        ctt_ext_arrow(ctt_ext_type0(), ctt_ext_type0()),
    )
}
/// Register all extended cubical type theory axioms into the given environment.
///
/// This extends `build_env` with de Morgan algebra laws, extended Kan filling,
/// V-types, extended transport/coercion, PathP, HITs (suspension, join, pushout,
/// quotient, truncations), Eilenberg-MacLane spaces, and group-theoretic constructions.
pub fn register_ctt_extended(env: &mut Environment) -> Result<(), String> {
    let decls: &[(&str, Expr)] = &[
        ("IDeMorgan1", axiom_i_de_morgan1_ty()),
        ("IDeMorgan2", axiom_i_de_morgan2_ty()),
        ("IMinAssoc", axiom_i_min_assoc_ty()),
        ("IMaxAssoc", axiom_i_max_assoc_ty()),
        ("IAbsorption", axiom_i_absorption_ty()),
        ("INegInvolution", axiom_i_neg_involution_ty()),
        ("KanFillHet", axiom_kan_fill_het_ty()),
        ("KanUniq", axiom_kan_uniq_ty()),
        ("KanCube2", axiom_kan_cube2_ty()),
        ("KanHomotopy", axiom_kan_homotopy_ty()),
        ("VType", axiom_v_type_ty()),
        ("Vin", axiom_vin_ty()),
        ("Vproj", axiom_vproj_ty()),
        ("CoeCompat", axiom_coe_compat_ty()),
        ("TranspIdConst", axiom_transp_id_const_ty()),
        ("TranspSigma", axiom_transp_sigma_ty()),
        ("TranspPi", axiom_transp_pi_ty()),
        ("PathPType", axiom_pathp_type_ty()),
        ("PathPLam", axiom_pathp_lam_ty()),
        ("PathPApp", axiom_pathp_app_ty()),
        ("SuspNorth", axiom_susp_north_ty()),
        ("SuspSouth", axiom_susp_south_ty()),
        ("SuspInd", axiom_susp_ind_ty()),
        ("Join", axiom_join_ty()),
        ("JoinInl", axiom_join_inl_ty()),
        ("JoinInr", axiom_join_inr_ty()),
        ("JoinPush", axiom_join_push_ty()),
        ("PropTrunc", axiom_prop_trunc_ty()),
        ("PT_intro", axiom_pt_intro_ty()),
        ("PT_IsProp", axiom_pt_is_prop_ty()),
        ("PT_rec", axiom_pt_rec_ty()),
        ("SetTrunc", axiom_set_trunc_ty()),
        ("ST_intro", axiom_st_intro_ty()),
        ("ST_IsSet", axiom_st_is_set_ty()),
        ("EMSpace", axiom_em_space_ty()),
        ("EMSpace_pi", axiom_em_space_pi_ty()),
        ("BG", axiom_bg_ty()),
        ("LoopSpace", axiom_loop_space_ty()),
        ("HomotopyGroup", axiom_homotopy_group_ty()),
        ("JamesConstruction", axiom_james_construction_ty()),
        ("FreeGroup", axiom_free_group_ty()),
        ("FreeGroupUnit", axiom_free_group_unit_ty()),
        ("FreeGroupInl", axiom_free_group_inl_ty()),
        ("FreeGroupMul", axiom_free_group_mul_ty()),
        ("QuotType", axiom_quot_type_ty()),
        ("QuotInj", axiom_quot_inj_ty()),
        ("QuotEq", axiom_quot_eq_ty()),
        ("QuotIsSet", axiom_quot_is_set_ty()),
        ("GroupCompletion", axiom_group_completion_ty()),
        ("GC_Inject", axiom_gc_inject_ty()),
        ("IsGroupoid", axiom_is_groupoid_ty()),
        ("IsNType", axiom_is_n_type_ty()),
        ("ConnectedType", axiom_connected_type_ty()),
    ];
    for (name, ty) in decls {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}
