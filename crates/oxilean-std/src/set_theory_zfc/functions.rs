//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    CantorNormalForm, Cardinal, CardinalArithmetic, CardinalComparison, ConstructibleUniverse,
    HeredFiniteSet, Ordinal, OrdinalArithmetic, SetNode, StaticSetMembership, VAlpha, ZFCAxiom,
    ZFCOrdinal,
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
pub fn bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
pub fn set_ty() -> Expr {
    cst("Set")
}
pub fn nat_ty() -> Expr {
    cst("Nat")
}
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn mem(x: Expr, y: Expr) -> Expr {
    app2(cst("Mem"), x, y)
}
pub fn subset(x: Expr, y: Expr) -> Expr {
    app2(cst("Subset"), x, y)
}
pub fn eq_set(x: Expr, y: Expr) -> Expr {
    app3(cst("Eq"), set_ty(), x, y)
}
pub fn and(p: Expr, q: Expr) -> Expr {
    app2(cst("And"), p, q)
}
pub fn or(p: Expr, q: Expr) -> Expr {
    app2(cst("Or"), p, q)
}
pub fn not(p: Expr) -> Expr {
    app(cst("Not"), p)
}
pub fn iff(p: Expr, q: Expr) -> Expr {
    app2(cst("Iff"), p, q)
}
pub fn exists_set(name: &str, body: Expr) -> Expr {
    app(
        cst("Exists"),
        Expr::Lam(
            BinderInfo::Default,
            Name::str(name),
            Box::new(set_ty()),
            Box::new(body),
        ),
    )
}
pub fn forall_set(name: &str, body: Expr) -> Expr {
    pi(BinderInfo::Default, name, set_ty(), body)
}
pub fn empty_set() -> Expr {
    cst("EmptySet")
}
pub(super) fn union(x: Expr, y: Expr) -> Expr {
    app2(cst("Union"), x, y)
}
pub(super) fn inter(x: Expr, y: Expr) -> Expr {
    app2(cst("Inter"), x, y)
}
pub(super) fn singleton(x: Expr) -> Expr {
    app(cst("Singleton"), x)
}
/// Axiom of Extensionality: ∀ X Y, (∀ z, z ∈ X ↔ z ∈ Y) → X = Y
pub fn axiom_extensionality_ty() -> Expr {
    forall_set(
        "X",
        forall_set(
            "Y",
            arrow(
                forall_set("z", iff(mem(bvar(0), bvar(2)), mem(bvar(0), bvar(1)))),
                eq_set(bvar(1), bvar(0)),
            ),
        ),
    )
}
/// Axiom of Empty Set: ∃ X, ∀ y, ¬(y ∈ X)
pub fn axiom_empty_ty() -> Expr {
    exists_set("X", forall_set("y", not(mem(bvar(0), bvar(1)))))
}
/// Axiom of Pairing: ∀ x y, ∃ Z, ∀ w, w ∈ Z ↔ (w = x ∨ w = y)
pub fn axiom_pairing_ty() -> Expr {
    forall_set(
        "x",
        forall_set(
            "y",
            exists_set(
                "Z",
                forall_set(
                    "w",
                    iff(
                        mem(bvar(0), bvar(1)),
                        or(eq_set(bvar(0), bvar(3)), eq_set(bvar(0), bvar(2))),
                    ),
                ),
            ),
        ),
    )
}
/// Axiom of Union: ∀ F, ∃ A, ∀ x Y, (x ∈ Y ∧ Y ∈ F) → x ∈ A
pub fn axiom_union_ty() -> Expr {
    forall_set(
        "F",
        exists_set(
            "A",
            forall_set(
                "x",
                forall_set(
                    "Y",
                    arrow(
                        and(mem(bvar(1), bvar(0)), mem(bvar(0), bvar(3))),
                        mem(bvar(1), bvar(2)),
                    ),
                ),
            ),
        ),
    )
}
/// Axiom of Power Set: ∀ x, ∃ P, ∀ z, z ∈ P ↔ z ⊆ x
pub fn axiom_power_set_ty() -> Expr {
    forall_set(
        "x",
        exists_set(
            "P",
            forall_set("z", iff(mem(bvar(0), bvar(1)), subset(bvar(0), bvar(2)))),
        ),
    )
}
/// Axiom of Infinity: ∃ X, ∅ ∈ X ∧ ∀ y ∈ X, y ∪ {y} ∈ X
pub fn axiom_infinity_ty() -> Expr {
    exists_set(
        "X",
        and(
            mem(empty_set(), bvar(0)),
            forall_set(
                "y",
                arrow(
                    mem(bvar(0), bvar(1)),
                    mem(union(bvar(0), singleton(bvar(0))), bvar(1)),
                ),
            ),
        ),
    )
}
/// Axiom of Replacement: if F is a functional relation, image of any set under F is a set.
/// ∀ A, (∀ x ∈ A, ∃! y, F(x, y)) → ∃ B, ∀ x ∈ A, ∃ y ∈ B, F(x, y)
pub fn axiom_replacement_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        arrow(set_ty(), arrow(set_ty(), prop())),
        forall_set(
            "A",
            arrow(
                forall_set(
                    "x",
                    arrow(
                        mem(bvar(0), bvar(1)),
                        exists_set(
                            "y",
                            and(
                                app2(bvar(4), bvar(1), bvar(0)),
                                forall_set(
                                    "z",
                                    arrow(
                                        app2(bvar(5), bvar(2), bvar(0)),
                                        eq_set(bvar(0), bvar(1)),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                exists_set(
                    "B",
                    forall_set(
                        "x",
                        arrow(
                            mem(bvar(0), bvar(2)),
                            exists_set(
                                "y",
                                and(mem(bvar(0), bvar(2)), app2(bvar(4), bvar(1), bvar(0))),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// Axiom of Regularity: ∀ X, X ≠ ∅ → ∃ y ∈ X, X ∩ y = ∅
pub fn axiom_regularity_ty() -> Expr {
    forall_set(
        "X",
        arrow(
            not(eq_set(bvar(0), empty_set())),
            exists_set(
                "y",
                and(
                    mem(bvar(0), bvar(1)),
                    eq_set(inter(bvar(1), bvar(0)), empty_set()),
                ),
            ),
        ),
    )
}
/// Axiom of Choice: ∀ X, ∅ ∉ X → ∃ f : Set → Set, ∀ A ∈ X, f(A) ∈ A
pub fn axiom_choice_ty() -> Expr {
    forall_set(
        "X",
        arrow(
            not(mem(empty_set(), bvar(0))),
            app(
                cst("Exists"),
                Expr::Lam(
                    BinderInfo::Default,
                    Name::str("f"),
                    Box::new(arrow(set_ty(), set_ty())),
                    Box::new(forall_set(
                        "A",
                        arrow(mem(bvar(0), bvar(2)), mem(app(bvar(1), bvar(0)), bvar(0))),
                    )),
                ),
            ),
        ),
    )
}
/// Zorn's Lemma: every partially ordered set in which every chain has an upper bound
/// contains a maximal element.
/// ∀ (P : Set) (le : Set → Set → Prop), PartialOrder P le
///   → (∀ C, Chain P le C → ∃ ub ∈ P, ∀ x ∈ C, le x ub)
///   → ∃ m ∈ P, ∀ x ∈ P, le m x → m = x
pub fn zorn_lemma_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        set_ty(),
        pi(
            BinderInfo::Default,
            "le",
            arrow(set_ty(), arrow(set_ty(), prop())),
            arrow(
                app2(cst("PartialOrder"), bvar(1), bvar(0)),
                arrow(
                    forall_set(
                        "C",
                        arrow(
                            app3(cst("Chain"), bvar(2), bvar(1), bvar(0)),
                            exists_set(
                                "ub",
                                and(
                                    mem(bvar(0), bvar(3)),
                                    forall_set(
                                        "x",
                                        arrow(
                                            mem(bvar(0), bvar(2)),
                                            app2(bvar(4), bvar(0), bvar(1)),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                    exists_set(
                        "m",
                        and(
                            mem(bvar(0), bvar(2)),
                            forall_set(
                                "x",
                                arrow(
                                    mem(bvar(0), bvar(3)),
                                    arrow(
                                        app2(bvar(3), bvar(1), bvar(0)),
                                        eq_set(bvar(1), bvar(0)),
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
/// Well-Ordering Theorem: every set can be well-ordered.
/// ∀ S : Set, ∃ (R : Set → Set → Prop), WellOrder S R
pub fn well_ordering_ty() -> Expr {
    forall_set(
        "S",
        app(
            cst("Exists"),
            Expr::Lam(
                BinderInfo::Default,
                Name::str("R"),
                Box::new(arrow(set_ty(), arrow(set_ty(), prop()))),
                Box::new(app2(cst("WellOrder"), bvar(1), bvar(0))),
            ),
        ),
    )
}
/// Cantor's Theorem: |X| < |P(X)| for any set X.
/// ∀ X : Set, StrictCardLt (Card X) (Card (PowerSet X))
pub fn cantor_theorem_ty() -> Expr {
    forall_set(
        "X",
        app2(
            cst("StrictCardLt"),
            app(cst("Card"), bvar(0)),
            app(cst("Card"), app(cst("PowerSet"), bvar(0))),
        ),
    )
}
/// Schroeder-Bernstein Theorem: injections both ways imply bijection.
/// ∀ A B : Set, Injection A B → Injection B A → Bijection A B
pub fn schroeder_bernstein_ty() -> Expr {
    forall_set(
        "A",
        forall_set(
            "B",
            arrow(
                app2(cst("Injection"), bvar(1), bvar(0)),
                arrow(
                    app2(cst("Injection"), bvar(0), bvar(1)),
                    app2(cst("Bijection"), bvar(1), bvar(0)),
                ),
            ),
        ),
    )
}
/// Equivalence of Choice, Zorn's Lemma, and Well-Ordering Theorem.
/// AC ↔ (ZL ↔ WO)
pub fn choice_equivalents_ty() -> Expr {
    iff(
        cst("AxiomOfChoice"),
        iff(cst("ZornsLemma"), cst("WellOrderingTheorem")),
    )
}
/// Type of ordinals: transitive sets well-ordered by ∈.
/// Ordinal is a Set that is transitive and well-ordered by membership.
pub fn ordinal_ty() -> Expr {
    arrow(set_ty(), prop())
}
/// Successor ordinal: α ↦ α ∪ {α}
/// ∀ α : Set, IsOrdinal α → IsOrdinal (α ∪ {α})
pub fn ordinal_succ_ty() -> Expr {
    forall_set(
        "alpha",
        arrow(
            app(cst("IsOrdinal"), bvar(0)),
            app(cst("IsOrdinal"), union(bvar(0), singleton(bvar(0)))),
        ),
    )
}
/// First infinite ordinal ω: ∃ ω, IsOrdinal ω ∧ IsLimit ω ∧ ∀ n, IsFinOrdinal n → n ∈ ω
pub fn omega_ordinal_ty() -> Expr {
    exists_set(
        "omega",
        and(
            app(cst("IsOrdinal"), bvar(0)),
            and(
                app(cst("IsLimit"), bvar(0)),
                forall_set(
                    "n",
                    arrow(app(cst("IsFinOrdinal"), bvar(0)), mem(bvar(0), bvar(1))),
                ),
            ),
        ),
    )
}
/// Cardinality type: Card is a function from Set to an ordinal.
/// Card : Set → Ordinal
pub fn cardinal_ty() -> Expr {
    arrow(set_ty(), cst("Ordinal"))
}
/// Aleph function: ℵ_α for ordinals α.
/// Aleph : Ordinal → Cardinal
pub fn aleph_ty() -> Expr {
    arrow(cst("Ordinal"), cst("Cardinal"))
}
/// Beth function: ℶ_α for ordinals α.
/// Beth : Ordinal → Cardinal
pub fn beth_ty() -> Expr {
    arrow(cst("Ordinal"), cst("Cardinal"))
}
/// Generalized Continuum Hypothesis: 2^ℵ_α = ℵ_{α+1}
/// ∀ α : Ordinal, CardPow 2 (Aleph α) = Aleph (OrdSucc α)
pub fn gch_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "alpha",
        cst("Ordinal"),
        app2(
            cst("CardEq"),
            app2(cst("CardPow"), cst("Card2"), app(cst("Aleph"), bvar(0))),
            app(cst("Aleph"), app(cst("OrdSucc"), bvar(0))),
        ),
    )
}
/// Beth-zero is ℵ_0: Beth 0 = Aleph 0
pub fn beth_zero_ty() -> Expr {
    app2(
        cst("CardEq"),
        app(cst("Beth"), cst("OrdZero")),
        app(cst("Aleph"), cst("OrdZero")),
    )
}
/// Beth successor: Beth(α+1) = 2^{Beth α}
/// ∀ α : Ordinal, Beth (OrdSucc α) = CardPow Card2 (Beth α)
pub fn beth_succ_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "alpha",
        cst("Ordinal"),
        app2(
            cst("CardEq"),
            app(cst("Beth"), app(cst("OrdSucc"), bvar(0))),
            app2(cst("CardPow"), cst("Card2"), app(cst("Beth"), bvar(0))),
        ),
    )
}
/// Cardinal addition: CardAdd : Cardinal → Cardinal → Cardinal
pub fn cardinal_add_ty() -> Expr {
    arrow(cst("Cardinal"), arrow(cst("Cardinal"), cst("Cardinal")))
}
/// Cardinal multiplication: CardMul κ λ = max(κ, λ) for infinite cardinals.
/// ∀ κ λ : Cardinal, IsInfinite κ → CardEq (CardMul κ λ) (CardMax κ λ)
pub fn cardinal_mul_infinite_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "kappa",
        cst("Cardinal"),
        pi(
            BinderInfo::Default,
            "lambda",
            cst("Cardinal"),
            arrow(
                app(cst("IsInfiniteCard"), bvar(1)),
                app2(
                    cst("CardEq"),
                    app2(cst("CardMul"), bvar(1), bvar(0)),
                    app2(cst("CardMax"), bvar(1), bvar(0)),
                ),
            ),
        ),
    )
}
/// König's theorem: sum_{i∈I} κ_i < prod_{i∈I} λ_i whenever each κ_i < λ_i.
/// Encoded as: ∀ (f g : Ordinal → Cardinal), (∀ i, CardLt (f i) (g i))
///              → CardLt (CardIndexedSum f) (CardIndexedProd g)
pub fn konig_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(cst("Ordinal"), cst("Cardinal")),
        pi(
            BinderInfo::Default,
            "g",
            arrow(cst("Ordinal"), cst("Cardinal")),
            arrow(
                pi(
                    BinderInfo::Default,
                    "i",
                    cst("Ordinal"),
                    app2(cst("CardLt"), app(bvar(2), bvar(0)), app(bvar(1), bvar(0))),
                ),
                app2(
                    cst("CardLt"),
                    app(cst("CardIndexedSum"), bvar(1)),
                    app(cst("CardIndexedProd"), bvar(0)),
                ),
            ),
        ),
    )
}
/// Cofinality: cf : Ordinal → Ordinal (the cofinality of an ordinal).
pub fn cofinality_ty() -> Expr {
    arrow(cst("Ordinal"), cst("Ordinal"))
}
/// Regular cardinal: κ is regular iff cf(κ) = κ.
/// ∀ κ : Cardinal, IsRegular κ ↔ OrdEq (Cof κ) κ
pub fn regular_cardinal_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "kappa",
        cst("Cardinal"),
        app2(
            cst("Iff"),
            app(cst("IsRegular"), bvar(0)),
            app2(cst("OrdEq"), app(cst("Cof"), bvar(0)), bvar(0)),
        ),
    )
}
/// Singular cardinal: a cardinal that is not regular.
/// ∀ κ : Cardinal, IsSingular κ ↔ ¬ IsRegular κ
pub fn singular_cardinal_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "kappa",
        cst("Cardinal"),
        app2(
            cst("Iff"),
            app(cst("IsSingular"), bvar(0)),
            app(cst("Not"), app(cst("IsRegular"), bvar(0))),
        ),
    )
}
/// Every aleph successor is regular: ∀ α, IsRegular (Aleph (OrdSucc α))
pub fn aleph_succ_regular_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "alpha",
        cst("Ordinal"),
        app(
            cst("IsRegular"),
            app(cst("Aleph"), app(cst("OrdSucc"), bvar(0))),
        ),
    )
}
/// Cofinality of aleph-omega is ω: OrdEq (Cof (Aleph OrdOmega)) OrdOmega
pub fn cof_aleph_omega_ty() -> Expr {
    app2(
        cst("OrdEq"),
        app(cst("Cof"), app(cst("Aleph"), cst("OrdOmega"))),
        cst("OrdOmega"),
    )
}
/// Inaccessible cardinal: a regular limit cardinal greater than ℵ_0.
/// ∀ κ : Cardinal, IsInaccessible κ ↔ (IsRegular κ ∧ IsLimit κ ∧ CardLt Aleph0 κ)
pub fn inaccessible_cardinal_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "kappa",
        cst("Cardinal"),
        app2(
            cst("Iff"),
            app(cst("IsInaccessible"), bvar(0)),
            and(
                app(cst("IsRegular"), bvar(0)),
                and(
                    app(cst("IsLimitCard"), bvar(0)),
                    app2(cst("CardLt"), cst("Aleph0"), bvar(0)),
                ),
            ),
        ),
    )
}
/// Mahlo cardinal: an inaccessible cardinal κ such that the set of inaccessible
/// cardinals below κ is stationary in κ.
/// ∀ κ : Cardinal, IsMahlo κ ↔ (IsInaccessible κ ∧ IsStationary κ (fun λ -> IsInaccessible λ))
pub fn mahlo_cardinal_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "kappa",
        cst("Cardinal"),
        app2(
            cst("Iff"),
            app(cst("IsMahlo"), bvar(0)),
            and(
                app(cst("IsInaccessible"), bvar(0)),
                app2(
                    cst("IsStationary"),
                    bvar(0),
                    Expr::Lam(
                        BinderInfo::Default,
                        Name::str("lambda"),
                        Box::new(cst("Cardinal")),
                        Box::new(app(cst("IsInaccessible"), bvar(0))),
                    ),
                ),
            ),
        ),
    )
}
/// Weakly compact cardinal: a cardinal κ > ω satisfying the tree property.
/// ∀ κ : Cardinal, IsWeaklyCompact κ ↔ (CardLt Aleph0 κ ∧ HasTreeProperty κ)
pub fn weakly_compact_cardinal_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "kappa",
        cst("Cardinal"),
        app2(
            cst("Iff"),
            app(cst("IsWeaklyCompact"), bvar(0)),
            and(
                app2(cst("CardLt"), cst("Aleph0"), bvar(0)),
                app(cst("HasTreeProperty"), bvar(0)),
            ),
        ),
    )
}
/// Measurable cardinal: a cardinal that carries a κ-complete non-principal ultrafilter.
/// ∀ κ : Cardinal, IsMeasurable κ ↔ ∃ U : Filter, IsKappaCompleteUF κ U
pub fn measurable_cardinal_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "kappa",
        cst("Cardinal"),
        app2(
            cst("Iff"),
            app(cst("IsMeasurable"), bvar(0)),
            app(
                cst("Exists"),
                Expr::Lam(
                    BinderInfo::Default,
                    Name::str("U"),
                    Box::new(cst("Filter")),
                    Box::new(app2(cst("IsKappaCompleteUF"), bvar(1), bvar(0))),
                ),
            ),
        ),
    )
}
/// Ulam matrix: ∀ κ, IsMeasurable κ → ∃ (M : UlamMatrix κ), IsUlamMatrix κ M
pub fn ulam_matrix_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "kappa",
        cst("Cardinal"),
        arrow(
            app(cst("IsMeasurable"), bvar(0)),
            app(
                cst("Exists"),
                Expr::Lam(
                    BinderInfo::Default,
                    Name::str("M"),
                    Box::new(app(cst("UlamMatrix"), bvar(1))),
                    Box::new(app2(cst("IsUlamMatrix"), bvar(2), bvar(0))),
                ),
            ),
        ),
    )
}
/// Supercompact cardinal: ∀ κ : Cardinal, IsSupercompact κ ↔
///   ∀ λ ≥ κ, ∃ U : NormalMeasure κ λ, IsNormalMeasure κ λ U
pub fn supercompact_cardinal_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "kappa",
        cst("Cardinal"),
        app2(
            cst("Iff"),
            app(cst("IsSupercompact"), bvar(0)),
            pi(
                BinderInfo::Default,
                "lambda",
                cst("Cardinal"),
                arrow(
                    app2(cst("CardLe"), bvar(1), bvar(0)),
                    app(
                        cst("Exists"),
                        Expr::Lam(
                            BinderInfo::Default,
                            Name::str("U"),
                            Box::new(app2(cst("NormalMeasure"), bvar(2), bvar(1))),
                            Box::new(app3(cst("IsNormalMeasure"), bvar(3), bvar(2), bvar(0))),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// Axiom of Determinacy (AD): every two-player infinite game on ω is determined.
/// AD : Prop  (just a constant, as in Lean/Mathlib)
pub fn ad_axiom_ty() -> Expr {
    prop()
}
/// Projective Determinacy (PD): every projective game is determined.
pub fn pd_axiom_ty() -> Expr {
    prop()
}
/// Σ¹_1 (analytic) sets are determined.
/// ∀ A : Set (Baire), IsSigma11 A → IsDetermined A
pub fn sigma11_det_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        arrow(cst("BaireSpace"), prop()),
        arrow(
            app(cst("IsSigma11"), bvar(0)),
            app(cst("IsDetermined"), bvar(0)),
        ),
    )
}
/// Projective hierarchy level n: IsSigma1n n A — A is Σ¹_n
pub fn projective_sigma_ty() -> Expr {
    arrow(cst("Nat"), arrow(arrow(cst("BaireSpace"), prop()), prop()))
}
/// Projective hierarchy level n: IsPi1n n A — A is Π¹_n
pub fn projective_pi_ty() -> Expr {
    arrow(cst("Nat"), arrow(arrow(cst("BaireSpace"), prop()), prop()))
}
/// PD implies every projective set is Lebesgue measurable.
/// PD → ∀ A : ProjectiveSet, IsLebesgueMeasurable A
pub fn pd_implies_measurability_ty() -> Expr {
    arrow(
        cst("ProjectiveDeterminacy"),
        pi(
            BinderInfo::Default,
            "A",
            cst("ProjectiveSet"),
            app(cst("IsLebesgueMeasurable"), bvar(0)),
        ),
    )
}
/// Borel sets form a σ-algebra: every open set is Borel.
/// ∀ U : OpenSet, IsBorel U
pub fn open_sets_are_borel_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "U",
        cst("OpenSet"),
        app(cst("IsBorel"), bvar(0)),
    )
}
/// Analytic sets (Σ¹_1) are projections of Borel sets.
/// ∀ A, IsSigma11 A ↔ ∃ B : BorelSet, IsProjection B A
pub fn analytic_sets_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        arrow(cst("BaireSpace"), prop()),
        app2(
            cst("Iff"),
            app(cst("IsSigma11"), bvar(0)),
            app(
                cst("Exists"),
                Expr::Lam(
                    BinderInfo::Default,
                    Name::str("B"),
                    Box::new(cst("BorelSet")),
                    Box::new(app2(cst("IsProjection"), bvar(0), bvar(1))),
                ),
            ),
        ),
    )
}
/// Suslin's theorem: a set is Borel iff it is both analytic and co-analytic.
/// ∀ A, IsBorel A ↔ (IsSigma11 A ∧ IsPi11 A)
pub fn suslin_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        arrow(cst("BaireSpace"), prop()),
        app2(
            cst("Iff"),
            app(cst("IsBorel"), bvar(0)),
            and(app(cst("IsSigma11"), bvar(0)), app(cst("IsPi11"), bvar(0))),
        ),
    )
}
/// Axiom of Constructibility (V = L): every set is constructible.
/// VEqualsL : Prop
pub fn v_equals_l_ty() -> Expr {
    prop()
}
/// V = L implies GCH.
pub fn v_equals_l_implies_gch_ty() -> Expr {
    arrow(cst("VEqualsL"), cst("GCH"))
}
/// V = L implies AC.
pub fn v_equals_l_implies_ac_ty() -> Expr {
    arrow(cst("VEqualsL"), cst("AxiomOfChoice"))
}
/// Inner model: L is the smallest model of ZFC containing all ordinals.
/// IsInnerModel L ∧ IsMinimalInnerModel L
pub fn l_is_minimal_inner_model_ty() -> Expr {
    and(
        app(cst("IsInnerModel"), cst("ConstructibleUniverse")),
        app(cst("IsMinimalInnerModel"), cst("ConstructibleUniverse")),
    )
}
/// Generic filter: ∀ P : ForcingPoset, ∃ G, IsGenericFilter P G
pub fn generic_filter_exists_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        cst("ForcingPoset"),
        app(
            cst("Exists"),
            Expr::Lam(
                BinderInfo::Default,
                Name::str("G"),
                Box::new(cst("Filter")),
                Box::new(app2(cst("IsGenericFilter"), bvar(1), bvar(0))),
            ),
        ),
    )
}
/// Forcing relation: p ⊩ φ means condition p forces sentence φ.
pub fn forcing_relation_ty() -> Expr {
    arrow(cst("ForcingCondition"), arrow(cst("Sentence"), prop()))
}
/// Independence of CH from ZFC (Cohen): ¬ProvableFromZFC CH ∧ ¬ProvableFromZFC (¬ CH)
pub fn independence_ch_ty() -> Expr {
    and(
        app(
            cst("Not"),
            app(cst("ProvableFromZFC"), cst("ContinuumHypothesis")),
        ),
        app(
            cst("Not"),
            app(
                cst("ProvableFromZFC"),
                app(cst("Not"), cst("ContinuumHypothesis")),
            ),
        ),
    )
}
/// Independence of AC from ZF: ¬ ProvableFromZF AC
pub fn independence_ac_ty() -> Expr {
    app(cst("Not"), app(cst("ProvableFromZF"), cst("AxiomOfChoice")))
}
/// Ordinal addition: ∀ α β : Ordinal, OrdAdd α β is an ordinal.
/// OrdAdd : Ordinal → Ordinal → Ordinal
pub fn ord_add_ty() -> Expr {
    arrow(cst("Ordinal"), arrow(cst("Ordinal"), cst("Ordinal")))
}
/// Ordinal multiplication: OrdMul : Ordinal → Ordinal → Ordinal
pub fn ord_mul_ty() -> Expr {
    arrow(cst("Ordinal"), arrow(cst("Ordinal"), cst("Ordinal")))
}
/// Ordinal exponentiation: OrdPow : Ordinal → Ordinal → Ordinal
pub fn ord_pow_ty() -> Expr {
    arrow(cst("Ordinal"), arrow(cst("Ordinal"), cst("Ordinal")))
}
/// Cantor normal form: every ordinal α > 0 has a unique representation
/// α = ω^{β_1}·n_1 + ... + ω^{β_k}·n_k with β_1 > ... > β_k and n_i : Nat.
/// ∀ α, α ≠ OrdZero → IsInCantorNF α
pub fn cantor_normal_form_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "alpha",
        cst("Ordinal"),
        arrow(
            app(cst("Not"), app2(cst("OrdEq"), bvar(0), cst("OrdZero"))),
            app(cst("IsInCantorNF"), bvar(0)),
        ),
    )
}
/// Transfinite induction: ∀ P : Ordinal → Prop, (∀ α, (∀ β < α, P β) → P α) → ∀ α, P α
pub fn transfinite_induction_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        arrow(cst("Ordinal"), prop()),
        arrow(
            pi(
                BinderInfo::Default,
                "alpha",
                cst("Ordinal"),
                arrow(
                    pi(
                        BinderInfo::Default,
                        "beta",
                        cst("Ordinal"),
                        arrow(app2(cst("OrdLt"), bvar(0), bvar(1)), app(bvar(2), bvar(0))),
                    ),
                    app(bvar(1), bvar(0)),
                ),
            ),
            pi(
                BinderInfo::Default,
                "alpha",
                cst("Ordinal"),
                app(bvar(1), bvar(0)),
            ),
        ),
    )
}
/// Transfinite recursion: ∀ (F : (Ordinal → Set) → Ordinal → Set), ∃! f : Ordinal → Set,
///   ∀ α, f α = F (f ↾ α) α
pub fn transfinite_recursion_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        arrow(
            arrow(cst("Ordinal"), set_ty()),
            arrow(cst("Ordinal"), set_ty()),
        ),
        app(
            cst("Exists"),
            Expr::Lam(
                BinderInfo::Default,
                Name::str("f"),
                Box::new(arrow(cst("Ordinal"), set_ty())),
                Box::new(pi(
                    BinderInfo::Default,
                    "alpha",
                    cst("Ordinal"),
                    app2(
                        cst("SetEq"),
                        app(bvar(1), bvar(0)),
                        app2(bvar(2), app2(cst("Restrict"), bvar(1), bvar(0)), bvar(0)),
                    ),
                )),
            ),
        ),
    )
}
/// Hartogs number: h(X) is the least ordinal not injectable into X.
/// Hartogs : Set → Ordinal
pub fn hartogs_number_ty() -> Expr {
    arrow(set_ty(), cst("Ordinal"))
}
/// Hartogs' theorem: ∀ X, ¬ Injection (Hartogs X) X
pub fn hartogs_theorem_ty() -> Expr {
    forall_set(
        "X",
        app(
            cst("Not"),
            app2(cst("Injection"), app(cst("Hartogs"), bvar(0)), bvar(0)),
        ),
    )
}
/// Well-ordering from AC: ∀ S, ∃ R, WellOrder S R  (already in well_ordering_ty,
/// this variant gives the explicit ordinal order-type)
/// ∀ S, ∃ α : Ordinal, OrdIso S α
pub fn well_ordering_ord_iso_ty() -> Expr {
    forall_set(
        "S",
        app(
            cst("Exists"),
            Expr::Lam(
                BinderInfo::Default,
                Name::str("alpha"),
                Box::new(cst("Ordinal")),
                Box::new(app2(cst("OrdIso"), bvar(1), bvar(0))),
            ),
        ),
    )
}
/// Club set: a subset C of an ordinal κ is club iff it is closed and unbounded.
/// ∀ κ : Ordinal, ∀ C : Set, IsClub κ C ↔ (IsClosed κ C ∧ IsUnbounded κ C)
pub fn club_set_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "kappa",
        cst("Ordinal"),
        forall_set(
            "C",
            app2(
                cst("Iff"),
                app2(cst("IsClub"), bvar(1), bvar(0)),
                and(
                    app2(cst("IsClosedIn"), bvar(1), bvar(0)),
                    app2(cst("IsUnboundedIn"), bvar(1), bvar(0)),
                ),
            ),
        ),
    )
}
/// Stationary set: S ⊆ κ is stationary iff S meets every club in κ.
/// ∀ κ : Ordinal, ∀ S : Set, IsStationary κ S ↔ ∀ C, IsClub κ C → S ∩ C ≠ ∅
pub fn stationary_set_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "kappa",
        cst("Ordinal"),
        forall_set(
            "S",
            app2(
                cst("Iff"),
                app2(cst("IsStationary"), bvar(1), bvar(0)),
                forall_set(
                    "C",
                    arrow(
                        app2(cst("IsClub"), bvar(2), bvar(0)),
                        app(cst("Not"), eq_set(inter(bvar(1), bvar(0)), empty_set())),
                    ),
                ),
            ),
        ),
    )
}
/// Fodor's lemma (pressing-down lemma): a regressive function on a stationary set is
/// constant on a stationary subset.
/// ∀ κ : RegularCard, ∀ S : StationarySet κ, ∀ f : Regressive S, ∃ α, IsStationary κ (f⁻¹ α)
pub fn fodor_lemma_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "kappa",
        cst("Ordinal"),
        arrow(
            app(
                cst("IsRegular"),
                app(cst("Card"), app(cst("OrdToSet"), bvar(0))),
            ),
            pi(
                BinderInfo::Default,
                "S",
                set_ty(),
                arrow(
                    app2(cst("IsStationary"), bvar(1), bvar(0)),
                    pi(
                        BinderInfo::Default,
                        "f",
                        arrow(set_ty(), cst("Ordinal")),
                        arrow(
                            app2(cst("IsRegressive"), bvar(0), bvar(1)),
                            app(
                                cst("Exists"),
                                Expr::Lam(
                                    BinderInfo::Default,
                                    Name::str("alpha"),
                                    Box::new(cst("Ordinal")),
                                    Box::new(app2(
                                        cst("IsStationary"),
                                        bvar(4),
                                        app2(cst("Preimage"), bvar(2), bvar(0)),
                                    )),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// Build a ZFC set theory environment.
pub fn build_set_theory_zfc_env() -> Environment {
    let mut env = Environment::new();
    let base_types: &[(&str, Expr)] = &[
        ("Set", type0()),
        ("Mem", arrow(set_ty(), arrow(set_ty(), prop()))),
        ("Subset", arrow(set_ty(), arrow(set_ty(), prop()))),
        (
            "Eq",
            pi(
                BinderInfo::Default,
                "T",
                type0(),
                arrow(bvar(0), arrow(bvar(0), prop())),
            ),
        ),
        ("And", arrow(prop(), arrow(prop(), prop()))),
        ("Or", arrow(prop(), arrow(prop(), prop()))),
        ("Not", arrow(prop(), prop())),
        ("Iff", arrow(prop(), arrow(prop(), prop()))),
        ("Exists", arrow(arrow(set_ty(), prop()), prop())),
        ("EmptySet", set_ty()),
        ("Union", arrow(set_ty(), arrow(set_ty(), set_ty()))),
        ("Inter", arrow(set_ty(), arrow(set_ty(), set_ty()))),
        ("Singleton", arrow(set_ty(), set_ty())),
        ("PowerSet", arrow(set_ty(), set_ty())),
        ("Card", arrow(set_ty(), cst("Ordinal"))),
        ("Ordinal", type0()),
        ("Cardinal", type0()),
        ("IsOrdinal", arrow(set_ty(), prop())),
        ("IsLimit", arrow(set_ty(), prop())),
        ("IsFinOrdinal", arrow(set_ty(), prop())),
        (
            "PartialOrder",
            arrow(
                set_ty(),
                arrow(arrow(set_ty(), arrow(set_ty(), prop())), prop()),
            ),
        ),
        (
            "Chain",
            arrow(
                set_ty(),
                arrow(
                    arrow(set_ty(), arrow(set_ty(), prop())),
                    arrow(set_ty(), prop()),
                ),
            ),
        ),
        (
            "WellOrder",
            arrow(
                set_ty(),
                arrow(arrow(set_ty(), arrow(set_ty(), prop())), prop()),
            ),
        ),
        ("Injection", arrow(set_ty(), arrow(set_ty(), prop()))),
        ("Bijection", arrow(set_ty(), arrow(set_ty(), prop()))),
        (
            "StrictCardLt",
            arrow(cst("Ordinal"), arrow(cst("Ordinal"), prop())),
        ),
        ("Aleph", arrow(cst("Ordinal"), cst("Cardinal"))),
        ("Beth", arrow(cst("Ordinal"), cst("Cardinal"))),
        (
            "CardPow",
            arrow(cst("Cardinal"), arrow(cst("Cardinal"), cst("Cardinal"))),
        ),
        ("Card2", cst("Cardinal")),
        (
            "CardEq",
            arrow(cst("Cardinal"), arrow(cst("Cardinal"), prop())),
        ),
        ("OrdSucc", arrow(cst("Ordinal"), cst("Ordinal"))),
        ("AxiomOfChoice", prop()),
        ("ZornsLemma", prop()),
        ("WellOrderingTheorem", prop()),
        ("OrdZero", cst("Ordinal")),
        ("OrdOmega", cst("Ordinal")),
        (
            "CardAdd",
            arrow(cst("Cardinal"), arrow(cst("Cardinal"), cst("Cardinal"))),
        ),
        (
            "CardMul",
            arrow(cst("Cardinal"), arrow(cst("Cardinal"), cst("Cardinal"))),
        ),
        (
            "CardMax",
            arrow(cst("Cardinal"), arrow(cst("Cardinal"), cst("Cardinal"))),
        ),
        (
            "CardLt",
            arrow(cst("Cardinal"), arrow(cst("Cardinal"), prop())),
        ),
        (
            "CardLe",
            arrow(cst("Cardinal"), arrow(cst("Cardinal"), prop())),
        ),
        (
            "CardIndexedSum",
            arrow(arrow(cst("Ordinal"), cst("Cardinal")), cst("Cardinal")),
        ),
        (
            "CardIndexedProd",
            arrow(arrow(cst("Ordinal"), cst("Cardinal")), cst("Cardinal")),
        ),
        ("IsInfiniteCard", arrow(cst("Cardinal"), prop())),
        ("IsLimitCard", arrow(cst("Cardinal"), prop())),
        ("Aleph0", cst("Cardinal")),
        ("Cof", arrow(cst("Cardinal"), cst("Ordinal"))),
        (
            "OrdEq",
            arrow(cst("Ordinal"), arrow(cst("Ordinal"), prop())),
        ),
        ("IsRegular", arrow(cst("Cardinal"), prop())),
        ("IsSingular", arrow(cst("Cardinal"), prop())),
        ("IsInaccessible", arrow(cst("Cardinal"), prop())),
        ("IsMahlo", arrow(cst("Cardinal"), prop())),
        (
            "IsStationary",
            arrow(
                cst("Ordinal"),
                arrow(arrow(cst("Cardinal"), prop()), prop()),
            ),
        ),
        ("IsWeaklyCompact", arrow(cst("Cardinal"), prop())),
        ("HasTreeProperty", arrow(cst("Cardinal"), prop())),
        ("IsMeasurable", arrow(cst("Cardinal"), prop())),
        ("Filter", type0()),
        (
            "IsKappaCompleteUF",
            arrow(cst("Cardinal"), arrow(cst("Filter"), prop())),
        ),
        ("UlamMatrix", arrow(cst("Cardinal"), type0())),
        (
            "IsUlamMatrix",
            arrow(
                cst("Cardinal"),
                arrow(app(cst("UlamMatrix"), cst("Aleph0")), prop()),
            ),
        ),
        ("IsSupercompact", arrow(cst("Cardinal"), prop())),
        (
            "NormalMeasure",
            arrow(cst("Cardinal"), arrow(cst("Cardinal"), type0())),
        ),
        (
            "IsNormalMeasure",
            arrow(
                cst("Cardinal"),
                arrow(
                    cst("Cardinal"),
                    arrow(
                        app2(cst("NormalMeasure"), cst("Aleph0"), cst("Aleph0")),
                        prop(),
                    ),
                ),
            ),
        ),
        ("BaireSpace", type0()),
        ("ProjectiveSet", type0()),
        ("BorelSet", type0()),
        ("OpenSet", type0()),
        ("ProjectiveDeterminacy", prop()),
        ("IsSigma11", arrow(arrow(cst("BaireSpace"), prop()), prop())),
        ("IsPi11", arrow(arrow(cst("BaireSpace"), prop()), prop())),
        (
            "IsDetermined",
            arrow(arrow(cst("BaireSpace"), prop()), prop()),
        ),
        ("IsBorel", arrow(arrow(cst("BaireSpace"), prop()), prop())),
        (
            "IsProjection",
            arrow(
                cst("BorelSet"),
                arrow(arrow(cst("BaireSpace"), prop()), prop()),
            ),
        ),
        ("IsLebesgueMeasurable", arrow(cst("ProjectiveSet"), prop())),
        ("VEqualsL", prop()),
        ("GCH", prop()),
        ("IsInnerModel", arrow(type0(), prop())),
        ("IsMinimalInnerModel", arrow(type0(), prop())),
        ("ConstructibleUniverse", type0()),
        ("ForcingPoset", type0()),
        ("ForcingCondition", type0()),
        ("Sentence", type0()),
        (
            "IsGenericFilter",
            arrow(cst("ForcingPoset"), arrow(cst("Filter"), prop())),
        ),
        ("ProvableFromZFC", arrow(prop(), prop())),
        ("ProvableFromZF", arrow(prop(), prop())),
        ("ContinuumHypothesis", prop()),
        (
            "OrdAdd",
            arrow(cst("Ordinal"), arrow(cst("Ordinal"), cst("Ordinal"))),
        ),
        (
            "OrdMul",
            arrow(cst("Ordinal"), arrow(cst("Ordinal"), cst("Ordinal"))),
        ),
        (
            "OrdPow",
            arrow(cst("Ordinal"), arrow(cst("Ordinal"), cst("Ordinal"))),
        ),
        ("IsInCantorNF", arrow(cst("Ordinal"), prop())),
        (
            "OrdLt",
            arrow(cst("Ordinal"), arrow(cst("Ordinal"), prop())),
        ),
        (
            "Restrict",
            arrow(
                arrow(cst("Ordinal"), set_ty()),
                arrow(cst("Ordinal"), arrow(cst("Ordinal"), set_ty())),
            ),
        ),
        ("SetEq", arrow(set_ty(), arrow(set_ty(), prop()))),
        ("Hartogs", arrow(set_ty(), cst("Ordinal"))),
        ("OrdIso", arrow(set_ty(), arrow(cst("Ordinal"), prop()))),
        ("OrdToSet", arrow(cst("Ordinal"), set_ty())),
        ("IsClub", arrow(cst("Ordinal"), arrow(set_ty(), prop()))),
        ("IsClosedIn", arrow(cst("Ordinal"), arrow(set_ty(), prop()))),
        (
            "IsUnboundedIn",
            arrow(cst("Ordinal"), arrow(set_ty(), prop())),
        ),
        (
            "IsRegressive",
            arrow(arrow(set_ty(), cst("Ordinal")), arrow(set_ty(), prop())),
        ),
        (
            "Preimage",
            arrow(
                arrow(set_ty(), cst("Ordinal")),
                arrow(cst("Ordinal"), set_ty()),
            ),
        ),
    ];
    for (name, ty) in base_types {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    let axioms: &[(&str, fn() -> Expr)] = &[
        ("axiom_extensionality", axiom_extensionality_ty),
        ("axiom_empty", axiom_empty_ty),
        ("axiom_pairing", axiom_pairing_ty),
        ("axiom_union", axiom_union_ty),
        ("axiom_power_set", axiom_power_set_ty),
        ("axiom_infinity", axiom_infinity_ty),
        ("axiom_replacement", axiom_replacement_ty),
        ("axiom_regularity", axiom_regularity_ty),
        ("axiom_choice", axiom_choice_ty),
    ];
    for (name, mk_ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: mk_ty(),
        })
        .ok();
    }
    let theorems: &[(&str, fn() -> Expr)] = &[
        ("zorn_lemma", zorn_lemma_ty),
        ("well_ordering", well_ordering_ty),
        ("cantor_theorem", cantor_theorem_ty),
        ("schroeder_bernstein", schroeder_bernstein_ty),
        ("choice_equivalents", choice_equivalents_ty),
        ("ordinal_succ", ordinal_succ_ty),
        ("omega_ordinal", omega_ordinal_ty),
        ("gch", gch_ty),
        ("beth_zero", beth_zero_ty),
        ("beth_succ", beth_succ_ty),
        ("konig_theorem", konig_theorem_ty),
        ("cardinal_mul_infinite", cardinal_mul_infinite_ty),
        ("regular_cardinal", regular_cardinal_ty),
        ("singular_cardinal", singular_cardinal_ty),
        ("aleph_succ_regular", aleph_succ_regular_ty),
        ("cof_aleph_omega", cof_aleph_omega_ty),
        ("inaccessible_cardinal", inaccessible_cardinal_ty),
        ("mahlo_cardinal", mahlo_cardinal_ty),
        ("weakly_compact_cardinal", weakly_compact_cardinal_ty),
        ("measurable_cardinal", measurable_cardinal_ty),
        ("ulam_matrix", ulam_matrix_ty),
        ("supercompact_cardinal", supercompact_cardinal_ty),
        ("sigma11_det", sigma11_det_ty),
        ("pd_implies_measurability", pd_implies_measurability_ty),
        ("open_sets_are_borel", open_sets_are_borel_ty),
        ("analytic_sets", analytic_sets_ty),
        ("suslin_theorem", suslin_theorem_ty),
        ("v_equals_l_implies_gch", v_equals_l_implies_gch_ty),
        ("v_equals_l_implies_ac", v_equals_l_implies_ac_ty),
        ("l_is_minimal_inner_model", l_is_minimal_inner_model_ty),
        ("generic_filter_exists", generic_filter_exists_ty),
        ("independence_ch", independence_ch_ty),
        ("independence_ac", independence_ac_ty),
        ("cantor_normal_form", cantor_normal_form_ty),
        ("transfinite_induction", transfinite_induction_ty),
        ("transfinite_recursion", transfinite_recursion_ty),
        ("hartogs_theorem", hartogs_theorem_ty),
        ("well_ordering_ord_iso", well_ordering_ord_iso_ty),
        ("club_set", club_set_ty),
        ("stationary_set", stationary_set_ty),
        ("fodor_lemma", fodor_lemma_ty),
    ];
    let const_axioms: &[(&str, Expr)] = &[
        ("ad_axiom", ad_axiom_ty()),
        ("pd_axiom", pd_axiom_ty()),
        ("v_equals_l", v_equals_l_ty()),
        ("forcing_relation", forcing_relation_ty()),
        ("cardinal_add", cardinal_add_ty()),
        ("cofinality", cofinality_ty()),
        ("hartogs_number", hartogs_number_ty()),
        ("ord_add", ord_add_ty()),
        ("ord_mul", ord_mul_ty()),
        ("ord_pow", ord_pow_ty()),
        ("projective_sigma", projective_sigma_ty()),
        ("projective_pi", projective_pi_ty()),
        ("ordinal_type", ordinal_ty()),
    ];
    for (name, ty) in const_axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    for (name, mk_ty) in theorems {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: mk_ty(),
        })
        .ok();
    }
    env
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_set_node_empty() {
        let s = SetNode::Empty;
        assert!(s.is_empty());
        assert_eq!(s.cardinality(), 0);
    }
    #[test]
    fn test_set_node_finite() {
        let s = SetNode::Finite(vec![0, 1, 2]);
        assert!(!s.is_empty());
        assert_eq!(s.cardinality(), 3);
    }
    #[test]
    fn test_set_node_subset() {
        let a = SetNode::Finite(vec![0, 1]);
        let b = SetNode::Finite(vec![0, 1, 2]);
        let c = SetNode::Finite(vec![3]);
        assert!(a.is_subset_of(&b));
        assert!(!c.is_subset_of(&a));
        assert!(SetNode::Empty.is_subset_of(&a));
    }
    #[test]
    fn test_set_node_power() {
        let base = SetNode::Finite(vec![0, 1, 2]);
        let power = SetNode::Power(Box::new(base));
        assert_eq!(power.cardinality(), 8);
    }
    #[test]
    fn test_ordinal_arithmetic() {
        assert_eq!(OrdinalArithmetic::add(3, 5), 8);
        assert_eq!(OrdinalArithmetic::mul(4, 6), 24);
        assert_eq!(OrdinalArithmetic::pow(2, 10), 1024);
        assert_eq!(OrdinalArithmetic::pow(0, 5), 0);
        assert_eq!(OrdinalArithmetic::pow(1, 100), 1);
        assert_eq!(OrdinalArithmetic::pow(3, 0), 1);
    }
    #[test]
    fn test_limit_ordinal() {
        assert!(OrdinalArithmetic::is_limit_ordinal(0));
        assert!(!OrdinalArithmetic::is_limit_ordinal(1));
        assert!(!OrdinalArithmetic::is_limit_ordinal(42));
    }
    #[test]
    fn test_cardinal_display() {
        assert_eq!(CardinalComparison::aleph(0), "ℵ_0");
        assert_eq!(CardinalComparison::aleph(1), "ℵ_1");
        assert_eq!(CardinalComparison::beth(0), "ℶ_0");
        assert_eq!(CardinalComparison::beth(3), "ℶ_3");
        assert!(CardinalComparison::continuum_hypothesis_holds());
    }
    #[test]
    fn test_build_set_theory_zfc_env() {
        let env = build_set_theory_zfc_env();
        assert!(env.get(&Name::str("Set")).is_some());
        assert!(env.get(&Name::str("Mem")).is_some());
        assert!(env.get(&Name::str("EmptySet")).is_some());
        assert!(env.get(&Name::str("Ordinal")).is_some());
        assert!(env.get(&Name::str("Cardinal")).is_some());
        assert!(env.get(&Name::str("axiom_extensionality")).is_some());
        assert!(env.get(&Name::str("axiom_empty")).is_some());
        assert!(env.get(&Name::str("axiom_pairing")).is_some());
        assert!(env.get(&Name::str("axiom_union")).is_some());
        assert!(env.get(&Name::str("axiom_power_set")).is_some());
        assert!(env.get(&Name::str("axiom_infinity")).is_some());
        assert!(env.get(&Name::str("axiom_replacement")).is_some());
        assert!(env.get(&Name::str("axiom_regularity")).is_some());
        assert!(env.get(&Name::str("axiom_choice")).is_some());
        assert!(env.get(&Name::str("zorn_lemma")).is_some());
        assert!(env.get(&Name::str("well_ordering")).is_some());
        assert!(env.get(&Name::str("cantor_theorem")).is_some());
        assert!(env.get(&Name::str("schroeder_bernstein")).is_some());
        assert!(env.get(&Name::str("choice_equivalents")).is_some());
        assert!(env.get(&Name::str("gch")).is_some());
        assert!(env.get(&Name::str("beth_zero")).is_some());
        assert!(env.get(&Name::str("beth_succ")).is_some());
        assert!(env.get(&Name::str("konig_theorem")).is_some());
        assert!(env.get(&Name::str("inaccessible_cardinal")).is_some());
        assert!(env.get(&Name::str("measurable_cardinal")).is_some());
        assert!(env.get(&Name::str("supercompact_cardinal")).is_some());
        assert!(env.get(&Name::str("cantor_normal_form")).is_some());
        assert!(env.get(&Name::str("transfinite_induction")).is_some());
        assert!(env.get(&Name::str("suslin_theorem")).is_some());
        assert!(env.get(&Name::str("independence_ch")).is_some());
        assert!(env.get(&Name::str("fodor_lemma")).is_some());
    }
    #[test]
    fn test_ordinal_symbolic() {
        let zero = Ordinal::Finite(0);
        let three = Ordinal::Finite(3);
        let omega = Ordinal::Omega;
        assert!(zero.is_zero());
        assert!(three.is_finite());
        assert!(!omega.is_finite());
        let eight = three.add(&Ordinal::Finite(5));
        assert_eq!(eight, Ordinal::Finite(8));
        let om_plus_3 = omega.add(&Ordinal::Finite(3));
        assert_eq!(om_plus_3, Ordinal::Omega);
        let two_omega = omega.add(&Ordinal::Omega);
        assert_eq!(two_omega, Ordinal::OmegaMul(2));
        let omega_3 = omega.mul(&Ordinal::Finite(3));
        assert_eq!(omega_3, Ordinal::OmegaMul(3));
        assert_eq!(omega.display(), "ω");
        assert_eq!(Ordinal::OmegaMul(2).display(), "ω·2");
    }
    #[test]
    fn test_cardinal_arithmetic() {
        assert_eq!(CardinalArithmetic::add(3, 5, false, false), 8);
        assert_eq!(CardinalArithmetic::mul(4, 6, false, false), 24);
        assert_eq!(
            CardinalArithmetic::add(100, u64::MAX, false, true),
            u64::MAX
        );
        assert_eq!(
            CardinalArithmetic::mul(100, u64::MAX, false, true),
            u64::MAX
        );
        assert_eq!(CardinalArithmetic::two_pow(10), 1024);
        assert_eq!(CardinalArithmetic::aleph_display(0), "ℵ_0");
        assert_eq!(CardinalArithmetic::beth_display(2), "ℶ_2");
        let seq = CardinalArithmetic::beth_sequence(3);
        assert_eq!(seq.len(), 4);
    }
    #[test]
    fn test_hered_finite_set() {
        let empty = HeredFiniteSet::empty();
        assert!(empty.is_empty());
        assert_eq!(empty.cardinality(), 0);
        let s1 = HeredFiniteSet::singleton(0);
        let s2 = HeredFiniteSet::singleton(1);
        let s3 = HeredFiniteSet::singleton(2);
        let pair01 = s1.union(&s2);
        assert_eq!(pair01.cardinality(), 2);
        assert!(s1.is_subset_of(&pair01));
        assert!(!s3.is_subset_of(&pair01));
        let triple = pair01.union(&s3);
        let inter = triple.inter(&pair01);
        assert_eq!(inter, pair01);
        assert!(pair01.contains_ordinal(0));
        assert!(pair01.contains_ordinal(1));
        assert!(!pair01.contains_ordinal(2));
        let ps = pair01.power_set();
        assert_eq!(ps.len(), 4);
    }
    #[test]
    fn test_static_set_membership() {
        let a = HeredFiniteSet::singleton(0);
        let b = HeredFiniteSet::singleton(0);
        assert!(StaticSetMembership::check_extensionality(&a, &b));
        let pair = StaticSetMembership::check_pairing(0, 1);
        assert_eq!(pair.cardinality(), 2);
        let union = StaticSetMembership::check_union(&a, &HeredFiniteSet::singleton(1));
        assert_eq!(union.cardinality(), 2);
        let ps = StaticSetMembership::check_power_set(&pair);
        assert_eq!(ps.len(), 4);
        assert!(StaticSetMembership::check_regularity(&pair));
    }
    #[test]
    fn test_cantor_normal_form() {
        let zero = CantorNormalForm::zero();
        assert!(zero.is_zero());
        assert_eq!(zero.display(), "0");
        let five = CantorNormalForm::from_finite(5);
        assert!(five.is_finite());
        assert_eq!(five.finite_value(), Some(5));
        assert_eq!(five.display(), "5");
        let omega = CantorNormalForm::omega();
        assert!(!omega.is_finite());
        assert_eq!(omega.display(), "ω");
        let cnf = CantorNormalForm::omega_pow_mul(2, 3);
        assert_eq!(cnf.display(), "ω^2·3");
        let omega_plus_5 = omega.add(&five);
        assert_eq!(omega_plus_5.display(), "ω + 5");
        let five_plus_omega = five.add(&omega);
        assert_eq!(five_plus_omega.display(), "ω");
        let sum = five.add(&zero);
        assert_eq!(sum.finite_value(), Some(5));
    }
}
#[cfg(test)]
mod tests_zfc_extra {
    use super::*;
    #[test]
    fn test_ordinal_arithmetic() {
        let a = ZFCOrdinal(3);
        let b = ZFCOrdinal(4);
        assert_eq!(a.add(b), ZFCOrdinal(7));
        assert_eq!(a.mul(b), ZFCOrdinal(12));
        assert_eq!(ZFCOrdinal::zero().successor(), ZFCOrdinal(1));
        assert!(ZFCOrdinal::zero().is_zero());
        assert!(ZFCOrdinal(3).is_successor());
    }
    #[test]
    fn test_cardinal_arithmetic() {
        let a = Cardinal::finite(3);
        let b = Cardinal::finite(4);
        assert_eq!(a.add(b), Cardinal::finite(7));
        assert_eq!(a.mul(b), Cardinal::finite(12));
        assert_eq!(a.power(b), Cardinal::finite(81));
        let pair = Cardinal::cantor_pairing(2, 3);
        assert!(pair > 0);
    }
    #[test]
    fn test_v_alpha_levels() {
        let v0 = VAlpha::v0();
        let v1 = VAlpha::v1();
        let v2 = VAlpha::v2();
        let v3 = VAlpha::v3();
        let v4 = VAlpha::v4();
        assert_eq!(v0.size, 0);
        assert_eq!(v1.size, 1);
        assert_eq!(v2.size, 2);
        assert_eq!(v3.size, 4);
        assert_eq!(v4.size, 16);
    }
    #[test]
    fn test_zfc_axioms() {
        let all = ZFCAxiom::all();
        assert_eq!(all.len(), 9);
        let zf = ZFCAxiom::zf_without_choice();
        assert_eq!(zf.len(), 8);
        assert!(ZFCAxiom::Choice.independent_of_zf());
        assert!(!ZFCAxiom::Pairing.independent_of_zf());
    }
    #[test]
    fn test_constructible_universe() {
        assert!(ConstructibleUniverse::implies_gch());
        assert!(ConstructibleUniverse::implies_ac());
        assert!(ConstructibleUniverse::satisfies_zfc());
    }
}
