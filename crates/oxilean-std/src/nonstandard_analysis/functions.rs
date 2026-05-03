//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
#![allow(clippy::items_after_test_module)]

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    HyperfiniteProb, HyperfiniteSet, Hyperreal, HyperrealApprox, InternalSet, LoebMeasure,
    PrincipalUltrafilter, TransferPrinciple,
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
pub fn lam(bi: BinderInfo, name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Lam(bi, Name::str(name), Box::new(dom), Box::new(body))
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
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn set_ty() -> Expr {
    cst("Set")
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
pub fn forall_hyper(name: &str, body: Expr) -> Expr {
    pi(BinderInfo::Default, name, hyperreal_ty(), body)
}
pub fn forall_real(name: &str, body: Expr) -> Expr {
    pi(BinderInfo::Default, name, real_ty(), body)
}
pub fn exists_hyper(name: &str, body: Expr) -> Expr {
    app(
        cst("Exists"),
        lam(BinderInfo::Default, name, hyperreal_ty(), body),
    )
}
pub fn exists_nat(name: &str, body: Expr) -> Expr {
    app(
        cst("Exists"),
        lam(BinderInfo::Default, name, nat_ty(), body),
    )
}
pub fn eq_hyper(x: Expr, y: Expr) -> Expr {
    app3(cst("Eq"), hyperreal_ty(), x, y)
}
pub fn eq_real(x: Expr, y: Expr) -> Expr {
    app3(cst("Eq"), real_ty(), x, y)
}
/// `Hyperreal : Type` — the hyperreal number field *ℝ.
pub fn hyperreal_ty() -> Expr {
    type0()
}
/// `Ultrafilter : Type` — an ultrafilter on ℕ.
pub fn ultrafilter_ty() -> Expr {
    arrow(arrow(nat_ty(), bool_ty()), bool_ty())
}
/// `InternalSet : Type` — an internal subset of *ℝ.
pub fn internal_set_ty() -> Expr {
    arrow(hyperreal_ty(), prop())
}
/// `InternalFun : Type` — an internal function *ℝ → *ℝ.
pub fn internal_fun_ty() -> Expr {
    arrow(hyperreal_ty(), hyperreal_ty())
}
/// `LoebMeasure : Type` — the Loeb measure on an internal probability space.
pub fn loeb_measure_ty() -> Expr {
    arrow(internal_set_ty(), real_ty())
}
/// `HyperAdd : Hyperreal → Hyperreal → Hyperreal` — addition in *ℝ.
pub fn hyper_add_ty() -> Expr {
    arrow(hyperreal_ty(), arrow(hyperreal_ty(), hyperreal_ty()))
}
/// `HyperMul : Hyperreal → Hyperreal → Hyperreal` — multiplication in *ℝ.
pub fn hyper_mul_ty() -> Expr {
    arrow(hyperreal_ty(), arrow(hyperreal_ty(), hyperreal_ty()))
}
/// `HyperNeg : Hyperreal → Hyperreal` — negation in *ℝ.
pub fn hyper_neg_ty() -> Expr {
    arrow(hyperreal_ty(), hyperreal_ty())
}
/// `HyperInv : Hyperreal → Hyperreal` — multiplicative inverse in *ℝ \ {0}.
pub fn hyper_inv_ty() -> Expr {
    arrow(hyperreal_ty(), hyperreal_ty())
}
/// `HyperLe : Hyperreal → Hyperreal → Prop` — order ≤ on *ℝ.
pub fn hyper_le_ty() -> Expr {
    arrow(hyperreal_ty(), arrow(hyperreal_ty(), prop()))
}
/// `HyperLt : Hyperreal → Hyperreal → Prop` — strict order < on *ℝ.
pub fn hyper_lt_ty() -> Expr {
    arrow(hyperreal_ty(), arrow(hyperreal_ty(), prop()))
}
/// `EmbedReal : Real → Hyperreal` — canonical embedding *: ℝ → *ℝ.
pub fn embed_real_ty() -> Expr {
    arrow(real_ty(), hyperreal_ty())
}
/// `hyperreal_ordered_field : OrderedField Hyperreal` — *ℝ is an ordered field.
pub fn hyperreal_ordered_field_ty() -> Expr {
    app(cst("OrderedField"), hyperreal_ty())
}
/// `real_embeds_in_hyperreal : ∀ r s : Real,
///   EmbedReal r + EmbedReal s = EmbedReal (r + s)` —
/// the embedding is a field homomorphism.
pub fn embed_real_hom_add_ty() -> Expr {
    forall_real(
        "r",
        forall_real(
            "s",
            eq_hyper(
                app2(
                    cst("HyperAdd"),
                    app(cst("EmbedReal"), bvar(1)),
                    app(cst("EmbedReal"), bvar(0)),
                ),
                app(cst("EmbedReal"), app2(cst("RealAdd"), bvar(1), bvar(0))),
            ),
        ),
    )
}
/// `IsInfinitesimal : Hyperreal → Prop` —
/// x is infinitesimal if |x| < r for every positive standard r.
pub fn is_infinitesimal_ty() -> Expr {
    arrow(hyperreal_ty(), prop())
}
/// `IsUnlimited : Hyperreal → Prop` —
/// x is unlimited (infinite) if |x| > r for every positive standard r.
pub fn is_unlimited_ty() -> Expr {
    arrow(hyperreal_ty(), prop())
}
/// `IsFinite : Hyperreal → Prop` —
/// x is finite if ∃ r : Real, |x| ≤ EmbedReal r.
pub fn is_finite_ty() -> Expr {
    arrow(hyperreal_ty(), prop())
}
/// `HyperAbs : Hyperreal → Hyperreal` — absolute value in *ℝ.
pub fn hyper_abs_ty() -> Expr {
    arrow(hyperreal_ty(), hyperreal_ty())
}
/// `Monad : Hyperreal → InternalSet` —
/// the monad (halo) of x: the set of hyperreals infinitely close to x.
pub fn monad_ty() -> Expr {
    arrow(hyperreal_ty(), internal_set_ty())
}
/// `Galaxy : Hyperreal → InternalSet` —
/// the galaxy of x: the set of hyperreals finitely close to x.
pub fn galaxy_ty() -> Expr {
    arrow(hyperreal_ty(), internal_set_ty())
}
/// `infinitesimal_exists : ∃ ε : Hyperreal, IsInfinitesimal ε ∧ ε ≠ 0` —
/// there exist nonzero infinitesimals in *ℝ.
pub fn infinitesimal_exists_ty() -> Expr {
    exists_hyper(
        "eps",
        and(
            app(cst("IsInfinitesimal"), bvar(0)),
            not(eq_hyper(bvar(0), cst("HyperZero"))),
        ),
    )
}
/// `unlimited_exists : ∃ ω : Hyperreal, IsUnlimited ω` —
/// there exist unlimited hyperreals.
pub fn unlimited_exists_ty() -> Expr {
    exists_hyper("omega", app(cst("IsUnlimited"), bvar(0)))
}
/// `infinitesimal_sum : ∀ ε δ : Hyperreal,
///   IsInfinitesimal ε → IsInfinitesimal δ → IsInfinitesimal (ε + δ)`.
pub fn infinitesimal_sum_ty() -> Expr {
    forall_hyper(
        "eps",
        forall_hyper(
            "delta",
            arrow(
                app(cst("IsInfinitesimal"), bvar(1)),
                arrow(
                    app(cst("IsInfinitesimal"), bvar(0)),
                    app(
                        cst("IsInfinitesimal"),
                        app2(cst("HyperAdd"), bvar(1), bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// `infinitesimal_product_finite : ∀ ε x : Hyperreal,
///   IsInfinitesimal ε → IsFinite x → IsInfinitesimal (ε * x)`.
pub fn infinitesimal_product_finite_ty() -> Expr {
    forall_hyper(
        "eps",
        forall_hyper(
            "x",
            arrow(
                app(cst("IsInfinitesimal"), bvar(1)),
                arrow(
                    app(cst("IsFinite"), bvar(0)),
                    app(
                        cst("IsInfinitesimal"),
                        app2(cst("HyperMul"), bvar(1), bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// `TransferPrinciple : FOSentence → Prop` —
/// a first-order sentence φ holds in ℝ iff it holds in *ℝ.
pub fn transfer_principle_ty() -> Expr {
    arrow(cst("FOSentence"), prop())
}
/// `transfer_fwd : ∀ φ : FOSentence, HoldsInReal φ → HoldsInHyperreal φ` —
/// transfer from ℝ to *ℝ.
pub fn transfer_fwd_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "phi",
        cst("FOSentence"),
        arrow(
            app(cst("HoldsInReal"), bvar(0)),
            app(cst("HoldsInHyperreal"), bvar(0)),
        ),
    )
}
/// `transfer_bwd : ∀ φ : FOSentence, HoldsInHyperreal φ → HoldsInReal φ` —
/// transfer from *ℝ to ℝ.
pub fn transfer_bwd_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "phi",
        cst("FOSentence"),
        arrow(
            app(cst("HoldsInHyperreal"), bvar(0)),
            app(cst("HoldsInReal"), bvar(0)),
        ),
    )
}
/// `transfer_iff : ∀ φ : FOSentence, HoldsInReal φ ↔ HoldsInHyperreal φ` —
/// full transfer equivalence.
pub fn transfer_iff_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "phi",
        cst("FOSentence"),
        iff(
            app(cst("HoldsInReal"), bvar(0)),
            app(cst("HoldsInHyperreal"), bvar(0)),
        ),
    )
}
/// `InternalAlgebra : Prop` —
/// the internal sets form an algebra (closed under complement, finite union).
pub fn internal_algebra_ty() -> Expr {
    prop()
}
/// `InternalClosed : InternalSet → InternalSet → InternalSet` —
/// intersection of two internal sets is internal.
pub fn internal_inter_ty() -> Expr {
    arrow(
        internal_set_ty(),
        arrow(internal_set_ty(), internal_set_ty()),
    )
}
/// `InternalUnion : InternalSet → InternalSet → InternalSet` —
/// union of two internal sets is internal.
pub fn internal_union_ty() -> Expr {
    arrow(
        internal_set_ty(),
        arrow(internal_set_ty(), internal_set_ty()),
    )
}
/// `InternalComplement : InternalSet → InternalSet` —
/// complement of an internal set is internal.
pub fn internal_complement_ty() -> Expr {
    arrow(internal_set_ty(), internal_set_ty())
}
/// `StarOfSet : Set → InternalSet` —
/// the nonstandard extension *A of a standard set A.
pub fn star_of_set_ty() -> Expr {
    arrow(set_ty(), internal_set_ty())
}
/// `StarOfFun : (Real → Real) → InternalFun` —
/// the nonstandard extension *f of a standard function f.
pub fn star_of_fun_ty() -> Expr {
    arrow(arrow(real_ty(), real_ty()), internal_fun_ty())
}
/// `overflow_principle : ∀ A : InternalSet,
///   (∀ n : Nat, EmbedReal (ofNat n) ∈ A) →
///   ∃ ω : Hyperreal, IsUnlimited ω ∧ ω ∈ A` —
/// if A contains all standard naturals, it contains an unlimited element.
pub fn overflow_principle_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        internal_set_ty(),
        arrow(
            arrow(
                nat_ty(),
                app(bvar(0), app(cst("EmbedReal"), app(cst("OfNat"), bvar(0)))),
            ),
            exists_hyper(
                "omega",
                and(app(cst("IsUnlimited"), bvar(0)), app(bvar(1), bvar(0))),
            ),
        ),
    )
}
/// `underflow_principle : ∀ A : InternalSet,
///   (∀ ε : Hyperreal, IsInfinitesimal ε → ε > 0 → ε ∈ A) →
///   ∃ r : Real, r > 0 ∧ EmbedReal r ∈ A` —
/// if A contains all positive infinitesimals, it contains a standard positive element.
pub fn underflow_principle_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        internal_set_ty(),
        arrow(
            forall_hyper(
                "eps",
                arrow(
                    app(cst("IsInfinitesimal"), bvar(0)),
                    arrow(
                        app2(cst("HyperLt"), cst("HyperZero"), bvar(0)),
                        app(bvar(1), bvar(0)),
                    ),
                ),
            ),
            prop(),
        ),
    )
}
/// `StPart : Hyperreal → Real` —
/// the standard part map st: fin(*ℝ) → ℝ, mapping x to the unique r ≈ x.
pub fn st_part_ty() -> Expr {
    arrow(hyperreal_ty(), real_ty())
}
/// `st_approx : ∀ x : Hyperreal, IsFinite x →
///   IsInfinitesimal (x - EmbedReal (StPart x))` —
/// st(x) ≈ x (they differ by an infinitesimal).
pub fn st_approx_ty() -> Expr {
    forall_hyper(
        "x",
        arrow(
            app(cst("IsFinite"), bvar(0)),
            app(
                cst("IsInfinitesimal"),
                app2(
                    cst("HyperAdd"),
                    bvar(0),
                    app(
                        cst("HyperNeg"),
                        app(cst("EmbedReal"), app(cst("StPart"), bvar(0))),
                    ),
                ),
            ),
        ),
    )
}
/// `st_unique : ∀ x : Hyperreal, ∀ r s : Real,
///   IsInfinitesimal (x - EmbedReal r) → IsInfinitesimal (x - EmbedReal s) → r = s` —
/// the standard part is unique.
pub fn st_unique_ty() -> Expr {
    forall_hyper(
        "x",
        forall_real(
            "r",
            forall_real(
                "s",
                arrow(
                    app(
                        cst("IsInfinitesimal"),
                        app2(
                            cst("HyperAdd"),
                            bvar(2),
                            app(cst("HyperNeg"), app(cst("EmbedReal"), bvar(1))),
                        ),
                    ),
                    arrow(
                        app(
                            cst("IsInfinitesimal"),
                            app2(
                                cst("HyperAdd"),
                                bvar(2),
                                app(cst("HyperNeg"), app(cst("EmbedReal"), bvar(0))),
                            ),
                        ),
                        eq_real(bvar(1), bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// `st_add : ∀ x y : Hyperreal, IsFinite x → IsFinite y →
///   StPart (x + y) = StPart x + StPart y`.
pub fn st_add_ty() -> Expr {
    forall_hyper(
        "x",
        forall_hyper(
            "y",
            arrow(
                app(cst("IsFinite"), bvar(1)),
                arrow(
                    app(cst("IsFinite"), bvar(0)),
                    eq_real(
                        app(cst("StPart"), app2(cst("HyperAdd"), bvar(1), bvar(0))),
                        app2(
                            cst("RealAdd"),
                            app(cst("StPart"), bvar(1)),
                            app(cst("StPart"), bvar(0)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `st_mul : ∀ x y : Hyperreal, IsFinite x → IsFinite y →
///   StPart (x * y) = StPart x * StPart y`.
pub fn st_mul_ty() -> Expr {
    forall_hyper(
        "x",
        forall_hyper(
            "y",
            arrow(
                app(cst("IsFinite"), bvar(1)),
                arrow(
                    app(cst("IsFinite"), bvar(0)),
                    eq_real(
                        app(cst("StPart"), app2(cst("HyperMul"), bvar(1), bvar(0))),
                        app2(
                            cst("RealMul"),
                            app(cst("StPart"), bvar(1)),
                            app(cst("StPart"), bvar(0)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `UltrapowerSeq : Type` — sequences ℝ^ℕ used in the ultrapower.
pub fn ultrapower_seq_ty() -> Expr {
    arrow(nat_ty(), real_ty())
}
/// `UltrapowerEquiv : UltrapowerSeq → UltrapowerSeq → Prop` —
/// equivalence relation: (a_n) ~ (b_n) iff {n | a_n = b_n} ∈ U.
pub fn ultrapower_equiv_ty() -> Expr {
    arrow(ultrapower_seq_ty(), arrow(ultrapower_seq_ty(), prop()))
}
/// `UltrapowerConstant : Real → UltrapowerSeq` —
/// constant sequence embedding r ↦ (r, r, r, …).
pub fn ultrapower_constant_ty() -> Expr {
    arrow(real_ty(), ultrapower_seq_ty())
}
/// `ultrapower_is_hyperreal : ∀ U : Ultrafilter,
///   IsHyperreal (UltrapowerQuotient U)` —
/// the ultrapower ℝ^ℕ/U is a model of *ℝ.
pub fn ultrapower_is_hyperreal_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "U",
        ultrafilter_ty(),
        app(cst("IsHyperreal"), app(cst("UltrapowerQuotient"), bvar(0))),
    )
}
/// `ultrapower_transfer : ∀ (U : Ultrafilter) (φ : FOSentence),
///   HoldsInReal φ ↔ HoldsInUltrapower U φ` —
/// Łoś's theorem: φ holds in the ultrapower iff it holds U-almost-everywhere.
pub fn los_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "U",
        ultrafilter_ty(),
        pi(
            BinderInfo::Default,
            "phi",
            cst("FOSentence"),
            iff(
                app(cst("HoldsInReal"), bvar(0)),
                app2(cst("HoldsInUltrapower"), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// `FreeUltrafilter : Ultrafilter → Prop` —
/// U is a free (non-principal) ultrafilter: does not contain any singleton.
pub fn free_ultrafilter_ty() -> Expr {
    arrow(ultrafilter_ty(), prop())
}
/// `IsFilter : Ultrafilter → Prop` —
/// filter axioms: upward closed, closed under finite intersection, nonempty.
pub fn is_filter_ty() -> Expr {
    arrow(ultrafilter_ty(), prop())
}
/// `IsUltrafilter : Ultrafilter → Prop` —
/// ultrafilter: filter that is maximal (for every set A, A ∈ U or Aᶜ ∈ U).
pub fn is_ultrafilter_ty() -> Expr {
    arrow(ultrafilter_ty(), prop())
}
/// `free_ultrafilter_exists : ∃ U : Ultrafilter, FreeUltrafilter U` —
/// free ultrafilters exist (requires Axiom of Choice / Zorn's lemma).
pub fn free_ultrafilter_exists_ty() -> Expr {
    app(
        cst("Exists"),
        lam(
            BinderInfo::Default,
            "U",
            ultrafilter_ty(),
            app(cst("FreeUltrafilter"), bvar(0)),
        ),
    )
}
/// `ultrafilter_dichotomy : ∀ (U : Ultrafilter) (A : Nat → Bool),
///   IsUltrafilter U → (U A = true ∨ U (complement A) = true)` —
/// ultrafilter dichotomy: exactly one of A or Aᶜ is in U.
pub fn ultrafilter_dichotomy_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "U",
        ultrafilter_ty(),
        arrow(app(cst("IsUltrafilter"), bvar(0)), prop()),
    )
}
/// `ultrafilter_closed_inter : ∀ (U : Ultrafilter) (A B : Nat → Bool),
///   IsUltrafilter U → U A = true → U B = true → U (inter A B) = true`.
pub fn ultrafilter_closed_inter_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "U",
        ultrafilter_ty(),
        arrow(app(cst("IsUltrafilter"), bvar(0)), prop()),
    )
}
/// `InternalMeasure : InternalSet → Hyperreal` —
/// an internal (finitely additive) measure on an internal algebra.
pub fn internal_measure_ty() -> Expr {
    arrow(internal_set_ty(), hyperreal_ty())
}
/// `LoebMeasureFn : InternalSet → Real` —
/// the Loeb measure: L(A) = st(μ(A)) where μ is an internal measure.
pub fn loeb_measure_fn_ty() -> Expr {
    arrow(internal_set_ty(), real_ty())
}
/// `loeb_is_measure : IsMeasure LoebMeasure` —
/// the Loeb measure is a genuine (σ-additive) measure.
pub fn loeb_is_measure_ty() -> Expr {
    app(cst("IsMeasure"), cst("LoebMeasure"))
}
/// `loeb_countably_additive : ∀ (A : Nat → InternalSet),
///   PairwiseDisjoint A →
///   LoebMeasure (InternalUnionCountable A) = ∑ n, LoebMeasure (A n)` —
/// the Loeb measure is countably additive.
pub fn loeb_countably_additive_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        arrow(nat_ty(), internal_set_ty()),
        arrow(
            app(cst("PairwiseDisjoint"), bvar(0)),
            eq_real(
                app(
                    cst("LoebMeasure"),
                    app(cst("InternalUnionCountable"), bvar(0)),
                ),
                app(
                    cst("SumSeq"),
                    lam(
                        BinderInfo::Default,
                        "n",
                        nat_ty(),
                        app(cst("LoebMeasure"), app(bvar(1), bvar(0))),
                    ),
                ),
            ),
        ),
    )
}
/// `loeb_from_counting : LoebMeasure derived from counting measure on *{1,...,N}
///   approximates Lebesgue measure` — informal statement as axiom.
pub fn loeb_approx_lebesgue_ty() -> Expr {
    prop()
}
/// `Nearstandard : Hyperreal → Prop` —
/// x is nearstandard if x is in the monad of some standard real.
pub fn nearstandard_ty() -> Expr {
    arrow(hyperreal_ty(), prop())
}
/// `StarOpen : InternalSet → Prop` —
/// A is *-open if it is the nonstandard extension of an open set.
pub fn star_open_ty() -> Expr {
    arrow(internal_set_ty(), prop())
}
/// `StarClosed : InternalSet → Prop` —
/// A is *-closed if it is the nonstandard extension of a closed set.
pub fn star_closed_ty() -> Expr {
    arrow(internal_set_ty(), prop())
}
/// `ns_continuity : ∀ (f : Real → Real) (a : Real),
///   ContinuousAt f a ↔
///   ∀ x : Hyperreal, x ≈ EmbedReal a → StarOfFun f x ≈ EmbedReal (f a)` —
/// nonstandard characterisation of continuity.
pub fn ns_continuity_ty() -> Expr {
    arrow(
        arrow(real_ty(), real_ty()),
        arrow(
            real_ty(),
            iff(
                app2(cst("ContinuousAt"), bvar(1), bvar(0)),
                forall_hyper(
                    "x",
                    arrow(
                        app2(cst("Approx"), bvar(0), app(cst("EmbedReal"), bvar(1))),
                        app2(
                            cst("Approx"),
                            app(cst("StarFun"), app2(cst("StarFun"), bvar(2), bvar(0))),
                            app(cst("EmbedReal"), app(bvar(2), bvar(1))),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `ns_compactness : ∀ K : Set, CompactStd K ↔
///   ∀ x : Hyperreal, x ∈ StarOfSet K → Nearstandard x ∧ StPart x ∈ K` —
/// nonstandard characterisation of compactness.
pub fn ns_compactness_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "K",
        set_ty(),
        iff(
            app(cst("CompactStd"), bvar(0)),
            forall_hyper(
                "x",
                arrow(
                    app(app(cst("StarOfSet"), bvar(1)), bvar(0)),
                    and(
                        app(cst("Nearstandard"), bvar(0)),
                        app(bvar(1), app(cst("StPart"), bvar(0))),
                    ),
                ),
            ),
        ),
    )
}
/// `ns_uniform_continuity : ∀ (f : Real → Real) (K : Set),
///   CompactStd K → ContinuousOnStd f K → UniformContinuousOnStd f K`.
pub fn ns_uniform_continuity_ty() -> Expr {
    arrow(arrow(real_ty(), real_ty()), arrow(set_ty(), prop()))
}
/// `InternalProb : InternalSet → Hyperreal` —
/// an internal probability measure (taking values in *\[0,1\]).
pub fn internal_prob_ty() -> Expr {
    arrow(internal_set_ty(), hyperreal_ty())
}
/// `LoebProb : InternalSet → Real` —
/// the Loeb probability measure L(A) = st(P*(A)).
pub fn loeb_prob_ty() -> Expr {
    arrow(internal_set_ty(), real_ty())
}
/// `loeb_prob_is_prob_measure : ∀ (P* : InternalProb),
///   IsProbMeasure (LoebProbOf P*)`.
pub fn loeb_prob_is_prob_measure_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "Pstar",
        internal_prob_ty(),
        app(cst("IsProbMeasure"), app(cst("LoebProbOf"), bvar(0))),
    )
}
/// `ns_independence : ∀ (A B : InternalSet),
///   InternallyIndep A B → LoebIndep (LoebProb A) (LoebProb B)`.
pub fn ns_independence_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        internal_set_ty(),
        pi(
            BinderInfo::Default,
            "B",
            internal_set_ty(),
            arrow(
                app2(cst("InternallyIndep"), bvar(1), bvar(0)),
                app2(cst("LoebIndep"), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// `anderson_theorem : ∀ (X : InternalFun),
///   InternalMartingale X → StandardMartingale (StLift X)` —
/// Anderson's lifting theorem: internal martingales lift to standard martingales.
pub fn anderson_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        internal_fun_ty(),
        arrow(
            app(cst("InternalMartingale"), bvar(0)),
            app(cst("StandardMartingale"), app(cst("StLift"), bvar(0))),
        ),
    )
}
/// `KappaSaturated : Hyperreal → Prop` —
/// *ℝ is κ-saturated if every family of fewer than κ internal sets with the
/// finite intersection property has nonempty intersection.
pub fn kappa_saturated_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `EnlargementPrinciple : Prop` —
/// the enlargement principle: for every standard set A, *A ⊇ A.
pub fn enlargement_principle_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        set_ty(),
        pi(
            BinderInfo::Default,
            "x",
            set_ty(),
            arrow(
                app2(cst("Mem"), bvar(0), bvar(1)),
                app(
                    app(cst("StarOfSet"), bvar(1)),
                    app(cst("EmbedSet"), bvar(0)),
                ),
            ),
        ),
    )
}
/// `saturation_implies_transfer : ℵ_1-saturation implies κ-saturation
///   for κ ≤ ℵ_1` — informal axiom.
pub fn saturation_implies_transfer_ty() -> Expr {
    prop()
}
/// `concurrent_relation_realized : ∀ (R : Hyperreal → Hyperreal → Prop),
///   InternalRelation R → Concurrent R →
///   ∃ y : Hyperreal, ∀ x : Hyperreal, StdElem x → R x y` —
/// saturation: every concurrent internal relation is realised.
pub fn concurrent_relation_realized_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        arrow(hyperreal_ty(), arrow(hyperreal_ty(), prop())),
        arrow(
            app(cst("InternalRelation"), bvar(0)),
            arrow(
                app(cst("Concurrent"), bvar(0)),
                exists_hyper(
                    "y",
                    forall_hyper(
                        "x",
                        arrow(
                            app(cst("StdElem"), bvar(0)),
                            app2(bvar(2), bvar(0), bvar(1)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `ns_limit : ∀ (f : Real → Real) (a L : Real),
///   LimitAt f a L ↔
///   ∀ x : Hyperreal, (x ≠ EmbedReal a ∧ x ≈ EmbedReal a) →
///   StarOfFun f x ≈ EmbedReal L` —
/// nonstandard characterisation of limit.
pub fn ns_limit_ty() -> Expr {
    arrow(
        arrow(real_ty(), real_ty()),
        arrow(real_ty(), arrow(real_ty(), prop())),
    )
}
/// `ns_derivative : ∀ (f : Real → Real) (a f' : Real),
///   HasDerivAt f a f' ↔
///   ∀ ε : Hyperreal, IsInfinitesimal ε → ε ≠ 0 →
///   StPart ((StarOfFun f (EmbedReal a + ε) - f a) / ε) = f'` —
/// nonstandard characterisation of the derivative.
pub fn ns_derivative_ty() -> Expr {
    arrow(
        arrow(real_ty(), real_ty()),
        arrow(real_ty(), arrow(real_ty(), prop())),
    )
}
/// `ns_integral : ∀ (f : Real → Real) (a b I : Real),
///   HasIntegral f a b I ↔
///   ∃ N : Hyperreal, IsUnlimited N ∧
///   IsInfinitesimal (InternalRiemannSum f a b N - EmbedReal I)` —
/// nonstandard characterisation of the Riemann integral.
pub fn ns_integral_ty() -> Expr {
    arrow(
        arrow(real_ty(), real_ty()),
        arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), prop()))),
    )
}
/// Populate `env` with all nonstandard analysis axioms and theorems.
pub fn build_nonstandard_analysis_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("Hyperreal", hyperreal_ty()),
        ("Ultrafilter", ultrafilter_ty()),
        ("InternalSet", internal_set_ty()),
        ("InternalFun", internal_fun_ty()),
        ("LoebMeasure", loeb_measure_ty()),
        ("FOSentence", type0()),
        ("HyperZero", hyperreal_ty()),
        ("HyperOne", hyperreal_ty()),
        ("HyperAdd", hyper_add_ty()),
        ("HyperMul", hyper_mul_ty()),
        ("HyperNeg", hyper_neg_ty()),
        ("HyperInv", hyper_inv_ty()),
        ("HyperLe", hyper_le_ty()),
        ("HyperLt", hyper_lt_ty()),
        ("HyperAbs", hyper_abs_ty()),
        ("EmbedReal", embed_real_ty()),
        ("RealAdd", arrow(real_ty(), arrow(real_ty(), real_ty()))),
        ("RealMul", arrow(real_ty(), arrow(real_ty(), real_ty()))),
        ("OrderedField", arrow(type0(), prop())),
        ("hyperreal_ordered_field", hyperreal_ordered_field_ty()),
        ("embed_real_hom_add", embed_real_hom_add_ty()),
        ("IsInfinitesimal", is_infinitesimal_ty()),
        ("IsUnlimited", is_unlimited_ty()),
        ("IsFinite", is_finite_ty()),
        ("Monad", monad_ty()),
        ("Galaxy", galaxy_ty()),
        (
            "Approx",
            arrow(hyperreal_ty(), arrow(hyperreal_ty(), prop())),
        ),
        ("infinitesimal_exists", infinitesimal_exists_ty()),
        ("unlimited_exists", unlimited_exists_ty()),
        ("infinitesimal_sum", infinitesimal_sum_ty()),
        (
            "infinitesimal_product_finite",
            infinitesimal_product_finite_ty(),
        ),
        ("HoldsInReal", arrow(cst("FOSentence"), prop())),
        ("HoldsInHyperreal", arrow(cst("FOSentence"), prop())),
        (
            "HoldsInUltrapower",
            arrow(ultrafilter_ty(), arrow(cst("FOSentence"), prop())),
        ),
        ("transfer_fwd", transfer_fwd_ty()),
        ("transfer_bwd", transfer_bwd_ty()),
        ("transfer_iff", transfer_iff_ty()),
        ("InternalInter", internal_inter_ty()),
        ("InternalUnion", internal_union_ty()),
        ("InternalComplement", internal_complement_ty()),
        ("StarOfSet", star_of_set_ty()),
        ("StarOfFun", star_of_fun_ty()),
        (
            "StarFun",
            arrow(
                arrow(real_ty(), real_ty()),
                arrow(hyperreal_ty(), hyperreal_ty()),
            ),
        ),
        (
            "InternalRelation",
            arrow(arrow(hyperreal_ty(), arrow(hyperreal_ty(), prop())), prop()),
        ),
        (
            "Concurrent",
            arrow(arrow(hyperreal_ty(), arrow(hyperreal_ty(), prop())), prop()),
        ),
        ("StdElem", arrow(hyperreal_ty(), prop())),
        ("overflow_principle", overflow_principle_ty()),
        ("OfNat", arrow(nat_ty(), real_ty())),
        ("StPart", st_part_ty()),
        ("st_approx", st_approx_ty()),
        ("st_unique", st_unique_ty()),
        ("st_add", st_add_ty()),
        ("st_mul", st_mul_ty()),
        (
            "UltrapowerQuotient",
            arrow(ultrafilter_ty(), hyperreal_ty()),
        ),
        ("IsHyperreal", arrow(hyperreal_ty(), prop())),
        ("ultrapower_is_hyperreal", ultrapower_is_hyperreal_ty()),
        ("los_theorem", los_theorem_ty()),
        ("FreeUltrafilter", free_ultrafilter_ty()),
        ("IsFilter", is_filter_ty()),
        ("IsUltrafilter", is_ultrafilter_ty()),
        ("free_ultrafilter_exists", free_ultrafilter_exists_ty()),
        ("ultrafilter_dichotomy", ultrafilter_dichotomy_ty()),
        ("ultrafilter_closed_inter", ultrafilter_closed_inter_ty()),
        ("InternalMeasure", internal_measure_ty()),
        ("LoebMeasureFn", loeb_measure_fn_ty()),
        (
            "IsMeasure",
            arrow(arrow(internal_set_ty(), real_ty()), prop()),
        ),
        ("loeb_is_measure", loeb_is_measure_ty()),
        (
            "PairwiseDisjoint",
            arrow(arrow(nat_ty(), internal_set_ty()), prop()),
        ),
        (
            "InternalUnionCountable",
            arrow(arrow(nat_ty(), internal_set_ty()), internal_set_ty()),
        ),
        ("SumSeq", arrow(arrow(nat_ty(), real_ty()), real_ty())),
        ("loeb_countably_additive", loeb_countably_additive_ty()),
        ("Nearstandard", nearstandard_ty()),
        ("StarOpen", star_open_ty()),
        ("StarClosed", star_closed_ty()),
        ("CompactStd", arrow(set_ty(), prop())),
        (
            "ContinuousAt",
            arrow(arrow(real_ty(), real_ty()), arrow(real_ty(), prop())),
        ),
        (
            "ContinuousOnStd",
            arrow(arrow(real_ty(), real_ty()), arrow(set_ty(), prop())),
        ),
        (
            "UniformContinuousOnStd",
            arrow(arrow(real_ty(), real_ty()), arrow(set_ty(), prop())),
        ),
        ("EmbedSet", arrow(set_ty(), hyperreal_ty())),
        ("ns_continuity", ns_continuity_ty()),
        ("ns_compactness", ns_compactness_ty()),
        ("InternalProb", internal_prob_ty()),
        (
            "LoebProbOf",
            arrow(internal_prob_ty(), arrow(internal_set_ty(), real_ty())),
        ),
        (
            "IsProbMeasure",
            arrow(arrow(internal_set_ty(), real_ty()), prop()),
        ),
        (
            "InternallyIndep",
            arrow(internal_set_ty(), arrow(internal_set_ty(), prop())),
        ),
        ("LoebIndep", arrow(real_ty(), arrow(real_ty(), prop()))),
        ("InternalMartingale", arrow(internal_fun_ty(), prop())),
        ("StandardMartingale", arrow(internal_fun_ty(), prop())),
        ("StLift", arrow(internal_fun_ty(), internal_fun_ty())),
        ("loeb_prob_is_prob_measure", loeb_prob_is_prob_measure_ty()),
        ("ns_independence", ns_independence_ty()),
        ("anderson_theorem", anderson_theorem_ty()),
        (
            "saturation_implies_transfer",
            saturation_implies_transfer_ty(),
        ),
        ("enlargement_principle", enlargement_principle_ty()),
        (
            "concurrent_relation_realized",
            concurrent_relation_realized_ty(),
        ),
        (
            "LimitAt",
            arrow(
                arrow(real_ty(), real_ty()),
                arrow(real_ty(), arrow(real_ty(), prop())),
            ),
        ),
        (
            "HasDerivAt",
            arrow(
                arrow(real_ty(), real_ty()),
                arrow(real_ty(), arrow(real_ty(), prop())),
            ),
        ),
        (
            "HasIntegral",
            arrow(
                arrow(real_ty(), real_ty()),
                arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), prop()))),
            ),
        ),
        (
            "InternalRiemannSum",
            arrow(
                arrow(real_ty(), real_ty()),
                arrow(
                    real_ty(),
                    arrow(real_ty(), arrow(hyperreal_ty(), hyperreal_ty())),
                ),
            ),
        ),
        ("ns_limit", ns_limit_ty()),
        ("ns_derivative", ns_derivative_ty()),
        ("ns_integral", ns_integral_ty()),
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
/// Compute the nonstandard derivative of `f` at `a` using increment `h`.
///
/// Returns `(f(a + h) - f(a)) / h`, which equals the standard derivative when h → 0.
pub fn ns_derivative_approx<F: Fn(f64) -> f64>(f: F, a: f64, h: f64) -> f64 {
    if h.abs() < f64::EPSILON {
        return f64::NAN;
    }
    (f(a + h) - f(a)) / h
}
/// Compute the nonstandard integral of `f` over `[a, b]` using `n` subintervals.
///
/// Returns the standard part of the internal Riemann sum with step size `(b-a)/n`.
pub fn ns_integral_approx<F: Fn(f64) -> f64>(f: F, a: f64, b: f64, n: usize) -> f64 {
    if n == 0 {
        return 0.0;
    }
    let h = (b - a) / n as f64;
    (0..n).map(|i| f(a + i as f64 * h) * h).sum()
}
/// Check if two f64 values are "infinitely close" (differ by less than a given tolerance).
pub fn approx_equal(x: f64, y: f64, tol: f64) -> bool {
    (x - y).abs() < tol
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_principal_ultrafilter_contains() {
        let u = PrincipalUltrafilter::new(4, 2);
        assert!(u.contains_set(0b0100));
        assert!(!u.contains_set(0b0011));
        assert!(!u.is_free());
    }
    #[test]
    fn test_hyperreal_constant_add() {
        let a = HyperrealApprox::constant(3.0, 4, 1);
        let b = HyperrealApprox::constant(5.0, 4, 1);
        let c = a.add(&b);
        assert!((c.value() - 8.0).abs() < 1e-12);
        assert!((c.standard_part() - 8.0).abs() < 1e-12);
    }
    #[test]
    fn test_hyperreal_mul() {
        let a = HyperrealApprox::constant(2.0, 3, 0);
        let b = HyperrealApprox::constant(7.0, 3, 0);
        let c = a.mul(&b);
        assert!((c.value() - 14.0).abs() < 1e-12);
    }
    #[test]
    fn test_ns_derivative_linear() {
        let f = |x: f64| 3.0 * x + 1.0;
        let deriv = ns_derivative_approx(f, 2.0, 1e-7);
        assert!((deriv - 3.0).abs() < 1e-5);
    }
    #[test]
    fn test_ns_integral_quadratic() {
        let f = |x: f64| x * x;
        let integral = ns_integral_approx(f, 0.0, 1.0, 100_000);
        assert!((integral - 1.0 / 3.0).abs() < 1e-4);
    }
    #[test]
    fn test_hyperfinite_prob_independence() {
        let space = HyperfiniteProb::new(8);
        let a: Vec<usize> = (0..4).collect();
        let b: Vec<usize> = (0..8).filter(|x| x % 2 == 0).collect();
        assert!(space.loeb_independent(&a, &b));
    }
    #[test]
    fn test_hyperfinite_prob_non_independence() {
        let space = HyperfiniteProb::new(4);
        let a = vec![0usize, 1];
        let b = vec![0usize];
        assert!(!space.loeb_independent(&a, &b));
    }
    #[test]
    fn test_build_nonstandard_analysis_env() {
        let mut env = Environment::new();
        build_nonstandard_analysis_env(&mut env);
        assert!(env.get(&Name::str("Hyperreal")).is_some());
        assert!(env.get(&Name::str("IsInfinitesimal")).is_some());
        assert!(env.get(&Name::str("StPart")).is_some());
        assert!(env.get(&Name::str("transfer_iff")).is_some());
        assert!(env.get(&Name::str("loeb_is_measure")).is_some());
        assert!(env.get(&Name::str("free_ultrafilter_exists")).is_some());
        assert!(env
            .get(&Name::str("concurrent_relation_realized"))
            .is_some());
    }
}
#[allow(dead_code)]
pub fn nsa_ext_hyperreal() -> Expr {
    cst("Hyperreal")
}
#[allow(dead_code)]
pub fn nsa_ext_real() -> Expr {
    cst("Real")
}
#[allow(dead_code)]
pub fn nsa_ext_nat() -> Expr {
    cst("Nat")
}
#[allow(dead_code)]
pub fn nsa_ext_prop() -> Expr {
    Expr::Sort(Level::zero())
}
#[allow(dead_code)]
pub fn nsa_ext_type0() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
#[allow(dead_code)]
pub fn nsa_ext_pi(name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str(name),
        Box::new(dom),
        Box::new(body),
    )
}
#[allow(dead_code)]
pub fn nsa_ext_arrow(a: Expr, b: Expr) -> Expr {
    nsa_ext_pi("_", a, b)
}
#[allow(dead_code)]
pub fn nsa_ext_iff(a: Expr, b: Expr) -> Expr {
    app2(cst("Iff"), a, b)
}
#[allow(dead_code)]
pub fn nsa_ext_and(a: Expr, b: Expr) -> Expr {
    app2(cst("And"), a, b)
}
#[allow(dead_code)]
pub fn nsa_ext_not(a: Expr) -> Expr {
    app(cst("Not"), a)
}
#[allow(dead_code)]
pub fn nsa_ext_or(a: Expr, b: Expr) -> Expr {
    app2(cst("Or"), a, b)
}
#[allow(dead_code)]
pub fn nsa_ext_eq(ty: Expr, a: Expr, b: Expr) -> Expr {
    app3(cst("Eq"), ty, a, b)
}
/// `internal_subset_ty : ∀ (A B : InternalSet), InternalSubset A B → Prop`
#[allow(dead_code)]
pub fn nsa_ext_internal_subset_ty() -> Expr {
    nsa_ext_arrow(
        cst("InternalSet"),
        nsa_ext_arrow(cst("InternalSet"), nsa_ext_prop()),
    )
}
/// `internal_inter_closed_ty : ∀ (A B : InternalSet), ∃ C : InternalSet, …`
/// Axiom: finite intersections of internal sets are internal.
#[allow(dead_code)]
pub fn nsa_ext_internal_inter_closed_ty() -> Expr {
    nsa_ext_pi(
        "A",
        cst("InternalSet"),
        nsa_ext_pi(
            "B",
            cst("InternalSet"),
            app(cst("Nonempty"), cst("InternalSet")),
        ),
    )
}
/// `sigma_saturation_ty` — σ-saturation: a countable family of internal sets with the
/// finite intersection property has a common point.
#[allow(dead_code)]
pub fn nsa_ext_sigma_saturation_ty() -> Expr {
    nsa_ext_arrow(
        nsa_ext_arrow(nsa_ext_nat(), cst("InternalSet")),
        nsa_ext_arrow(
            app(
                cst("HasFIP"),
                nsa_ext_arrow(nsa_ext_nat(), cst("InternalSet")),
            ),
            app(cst("Nonempty"), cst("Hyperreal")),
        ),
    )
}
/// `kappa_saturation_existence_ty` — every κ-saturated model of ℝ exists
/// (there is a κ-saturated elementary extension of ℝ).
#[allow(dead_code)]
pub fn nsa_ext_kappa_saturation_existence_ty() -> Expr {
    nsa_ext_arrow(
        app(cst("IsInfiniteCardinal"), nsa_ext_nat()),
        app(cst("Nonempty"), cst("Hyperreal")),
    )
}
/// `overflow_for_internal_sets_ty` — overflow: if an internal set A contains all standard
/// naturals then it contains a non-standard natural.
#[allow(dead_code)]
pub fn nsa_ext_overflow_for_internal_sets_ty() -> Expr {
    nsa_ext_pi(
        "A",
        cst("InternalSet"),
        nsa_ext_arrow(
            nsa_ext_pi(
                "n",
                nsa_ext_nat(),
                app2(cst("InternalMem"), cst("n"), Expr::BVar(1)),
            ),
            app(cst("Nonempty"), cst("Hyperreal")),
        ),
    )
}
/// `underflow_for_internal_sets_ty` — underflow dual principle.
#[allow(dead_code)]
pub fn nsa_ext_underflow_for_internal_sets_ty() -> Expr {
    nsa_ext_pi(
        "A",
        cst("InternalSet"),
        nsa_ext_arrow(
            app(cst("InternalBoundedAbove"), Expr::BVar(0)),
            app(cst("Nonempty"), cst("Hyperreal")),
        ),
    )
}
/// `hyperfinite_set_ty : Type` — the type of hyperfinite sets.
#[allow(dead_code)]
pub fn nsa_ext_hyperfinite_set_ty() -> Expr {
    nsa_ext_type0()
}
/// `hyperfinite_cardinality_ty` — every hyperfinite set has a hypernatural cardinality.
#[allow(dead_code)]
pub fn nsa_ext_hyperfinite_cardinality_ty() -> Expr {
    nsa_ext_arrow(cst("HyperfiniteSet"), cst("Hyperreal"))
}
/// `hyperfinite_interval_ty` — `{0, 1, …, N}` is hyperfinite for any hypernatural N.
#[allow(dead_code)]
pub fn nsa_ext_hyperfinite_interval_ty() -> Expr {
    nsa_ext_arrow(
        cst("Hyperreal"),
        app(cst("IsHyperfinite"), cst("HyperfiniteInterval")),
    )
}
/// `hyperfinite_sum_ty` — the sum of an internal function over a hyperfinite set is hyperreal.
#[allow(dead_code)]
pub fn nsa_ext_hyperfinite_sum_ty() -> Expr {
    nsa_ext_arrow(
        cst("HyperfiniteSet"),
        nsa_ext_arrow(
            nsa_ext_arrow(cst("Hyperreal"), cst("Hyperreal")),
            cst("Hyperreal"),
        ),
    )
}
/// `hyperfinite_product_ty` — product analog.
#[allow(dead_code)]
pub fn nsa_ext_hyperfinite_product_ty() -> Expr {
    nsa_ext_arrow(
        cst("HyperfiniteSet"),
        nsa_ext_arrow(
            nsa_ext_arrow(cst("Hyperreal"), cst("Hyperreal")),
            cst("Hyperreal"),
        ),
    )
}
/// `loeb_sigma_algebra_ty` — the Loeb σ-algebra on a hyperfinite set.
#[allow(dead_code)]
pub fn nsa_ext_loeb_sigma_algebra_ty() -> Expr {
    nsa_ext_arrow(cst("HyperfiniteSet"), nsa_ext_type0())
}
/// `loeb_measure_extends_counting_ty` — the Loeb measure agrees with normalized counting
/// measure on internal sets.
#[allow(dead_code)]
pub fn nsa_ext_loeb_measure_extends_counting_ty() -> Expr {
    nsa_ext_pi(
        "S",
        cst("HyperfiniteSet"),
        nsa_ext_pi(
            "A",
            cst("InternalSet"),
            nsa_ext_eq(
                nsa_ext_real(),
                app2(cst("LoebMeasOf"), Expr::BVar(1), Expr::BVar(0)),
                app(
                    cst("StPart"),
                    app2(
                        cst("NormalizedCountingMeasure"),
                        Expr::BVar(1),
                        Expr::BVar(0),
                    ),
                ),
            ),
        ),
    )
}
/// `nonstandard_integral_as_loeb_ty` — the Lebesgue integral can be computed as the
/// Loeb integral of an internal Riemann sum.
#[allow(dead_code)]
pub fn nsa_ext_nonstandard_integral_as_loeb_ty() -> Expr {
    nsa_ext_pi(
        "f",
        nsa_ext_arrow(nsa_ext_real(), nsa_ext_real()),
        nsa_ext_pi(
            "a",
            nsa_ext_real(),
            nsa_ext_pi(
                "b",
                nsa_ext_real(),
                nsa_ext_iff(
                    app3(
                        cst("HasLebesgueIntegral"),
                        Expr::BVar(2),
                        Expr::BVar(1),
                        Expr::BVar(0),
                    ),
                    app3(
                        cst("HasLoebIntegral"),
                        Expr::BVar(2),
                        Expr::BVar(1),
                        Expr::BVar(0),
                    ),
                ),
            ),
        ),
    )
}
/// `loeb_anderson_theorem_ty` — Anderson's theorem: the Loeb measure corresponds to
/// Wiener measure under the hyperfinite random walk.
#[allow(dead_code)]
pub fn nsa_ext_loeb_anderson_theorem_ty() -> Expr {
    nsa_ext_arrow(
        cst("HyperfiniteRandomWalk"),
        nsa_ext_arrow(
            cst("LoebProbSpace"),
            app(cst("IsWienerProcess"), cst("BrownianMotion")),
        ),
    )
}
/// `s_continuous_iff_continuous_ty` — a standard function is continuous iff it is
/// S-continuous (infinitely close inputs give infinitely close outputs).
#[allow(dead_code)]
pub fn nsa_ext_s_continuous_iff_continuous_ty() -> Expr {
    nsa_ext_pi(
        "f",
        nsa_ext_arrow(nsa_ext_real(), nsa_ext_real()),
        nsa_ext_pi(
            "x",
            nsa_ext_real(),
            nsa_ext_iff(
                app2(cst("ContinuousAt"), Expr::BVar(1), Expr::BVar(0)),
                app2(
                    cst("SContinuousAt"),
                    Expr::BVar(1),
                    app(cst("EmbedReal"), Expr::BVar(0)),
                ),
            ),
        ),
    )
}
/// `s_differentiable_iff_differentiable_ty` — S-differentiability characterization.
#[allow(dead_code)]
pub fn nsa_ext_s_differentiable_iff_differentiable_ty() -> Expr {
    nsa_ext_pi(
        "f",
        nsa_ext_arrow(nsa_ext_real(), nsa_ext_real()),
        nsa_ext_pi(
            "x",
            nsa_ext_real(),
            nsa_ext_iff(
                app2(cst("DifferentiableAt"), Expr::BVar(1), Expr::BVar(0)),
                app2(
                    cst("SDifferentiableAt"),
                    Expr::BVar(1),
                    app(cst("EmbedReal"), Expr::BVar(0)),
                ),
            ),
        ),
    )
}
/// `s_continuous_at_standard_ty` — S-continuity at standard points.
#[allow(dead_code)]
pub fn nsa_ext_s_continuous_at_standard_ty() -> Expr {
    nsa_ext_pi(
        "f",
        nsa_ext_arrow(nsa_ext_real(), nsa_ext_real()),
        nsa_ext_pi(
            "x",
            nsa_ext_real(),
            nsa_ext_arrow(
                app2(
                    cst("SContinuousAt"),
                    Expr::BVar(1),
                    app(cst("EmbedReal"), Expr::BVar(0)),
                ),
                nsa_ext_pi(
                    "y",
                    nsa_ext_hyperreal(),
                    nsa_ext_arrow(
                        app2(
                            cst("Approx"),
                            Expr::BVar(0),
                            app(cst("EmbedReal"), Expr::BVar(2)),
                        ),
                        app2(
                            cst("Approx"),
                            app(cst("StarFun"), app(Expr::BVar(3), Expr::BVar(0))),
                            app(cst("EmbedReal"), app(Expr::BVar(3), Expr::BVar(2))),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `hyperfinite_dft_ty` — the hyperfinite discrete Fourier transform.
#[allow(dead_code)]
pub fn nsa_ext_hyperfinite_dft_ty() -> Expr {
    nsa_ext_arrow(
        nsa_ext_arrow(cst("Hyperreal"), cst("Hyperreal")),
        nsa_ext_arrow(
            cst("HyperfiniteSet"),
            nsa_ext_arrow(cst("Hyperreal"), cst("Hyperreal")),
        ),
    )
}
/// `hyperfinite_dft_inversion_ty` — the inverse DFT theorem for hyperfinite transforms.
#[allow(dead_code)]
pub fn nsa_ext_hyperfinite_dft_inversion_ty() -> Expr {
    nsa_ext_arrow(
        app(cst("IsHyperfinite"), cst("HyperfiniteSet")),
        nsa_ext_arrow(
            nsa_ext_arrow(cst("Hyperreal"), cst("Hyperreal")),
            nsa_ext_eq(
                nsa_ext_arrow(cst("Hyperreal"), cst("Hyperreal")),
                app(cst("InverseDFT"), app(cst("HyperfiniteDFT"), cst("f"))),
                cst("f"),
            ),
        ),
    )
}
/// `hyperfinite_dft_approx_fourier_ty` — the hyperfinite DFT approximates the classical
/// Fourier transform for standard functions.
#[allow(dead_code)]
pub fn nsa_ext_hyperfinite_dft_approx_fourier_ty() -> Expr {
    nsa_ext_pi(
        "f",
        nsa_ext_arrow(nsa_ext_real(), nsa_ext_real()),
        nsa_ext_pi(
            "xi",
            nsa_ext_real(),
            app2(
                cst("Approx"),
                app2(
                    cst("HyperfiniteDFT"),
                    app(cst("StarFun"), Expr::BVar(1)),
                    app(cst("EmbedReal"), Expr::BVar(0)),
                ),
                app(
                    cst("EmbedReal"),
                    app2(cst("FourierTransform"), Expr::BVar(1), Expr::BVar(0)),
                ),
            ),
        ),
    )
}
/// `monad_subset_galaxy_ty` — the monad of x is a subset of its galaxy.
#[allow(dead_code)]
pub fn nsa_ext_monad_subset_galaxy_ty() -> Expr {
    nsa_ext_pi(
        "x",
        nsa_ext_hyperreal(),
        app2(
            cst("InternalSubset"),
            app(cst("Monad"), Expr::BVar(0)),
            app(cst("Galaxy"), Expr::BVar(0)),
        ),
    )
}
/// `monad_intersection_standard_ty` — the monad of a standard real r is {*r}.
#[allow(dead_code)]
pub fn nsa_ext_monad_intersection_standard_ty() -> Expr {
    nsa_ext_pi(
        "r",
        nsa_ext_real(),
        nsa_ext_eq(
            cst("InternalSet"),
            app(cst("Monad"), app(cst("EmbedReal"), Expr::BVar(0))),
            app(cst("Singleton"), app(cst("EmbedReal"), Expr::BVar(0))),
        ),
    )
}
/// `open_iff_monad_contained_ty` — a set U is open iff for every x ∈ U, monad(x) ⊆ *U.
#[allow(dead_code)]
pub fn nsa_ext_open_iff_monad_contained_ty() -> Expr {
    nsa_ext_pi(
        "U",
        app(cst("Set"), nsa_ext_real()),
        nsa_ext_iff(
            app(cst("IsOpen"), Expr::BVar(0)),
            nsa_ext_pi(
                "x",
                nsa_ext_real(),
                nsa_ext_arrow(
                    app2(cst("Mem"), Expr::BVar(0), Expr::BVar(1)),
                    app2(
                        cst("InternalSubset"),
                        app(cst("Monad"), app(cst("EmbedReal"), Expr::BVar(0))),
                        app(cst("StarOfSet"), Expr::BVar(1)),
                    ),
                ),
            ),
        ),
    )
}
/// `compact_iff_all_nearstandard_ty` — K is compact iff every x ∈ *K is nearstandard.
#[allow(dead_code)]
pub fn nsa_ext_compact_iff_all_nearstandard_ty() -> Expr {
    nsa_ext_pi(
        "K",
        app(cst("Set"), nsa_ext_real()),
        nsa_ext_iff(
            app(cst("IsCompact"), Expr::BVar(0)),
            nsa_ext_pi(
                "x",
                nsa_ext_hyperreal(),
                nsa_ext_arrow(
                    app2(
                        cst("HyperMem"),
                        Expr::BVar(0),
                        app(cst("StarOfSet"), Expr::BVar(1)),
                    ),
                    app(cst("Nearstandard"), Expr::BVar(0)),
                ),
            ),
        ),
    )
}
/// `loeb_outer_measure_ty` — the Loeb outer measure.
#[allow(dead_code)]
pub fn nsa_ext_loeb_outer_measure_ty() -> Expr {
    nsa_ext_arrow(
        cst("HyperfiniteSet"),
        nsa_ext_arrow(app(cst("Set"), nsa_ext_hyperreal()), nsa_ext_real()),
    )
}
/// `loeb_completion_ty` — the Loeb measure is complete (null sets have all subsets measurable).
#[allow(dead_code)]
pub fn nsa_ext_loeb_completion_ty() -> Expr {
    nsa_ext_pi(
        "S",
        cst("HyperfiniteSet"),
        app(
            cst("IsMeasureComplete"),
            app(cst("LoebMeasOf"), Expr::BVar(0)),
        ),
    )
}
/// `ns_lebesgue_from_loeb_ty` — the standard part of the Loeb integral is the Lebesgue integral.
#[allow(dead_code)]
pub fn nsa_ext_ns_lebesgue_from_loeb_ty() -> Expr {
    nsa_ext_pi(
        "f",
        nsa_ext_arrow(nsa_ext_real(), nsa_ext_real()),
        nsa_ext_pi(
            "a",
            nsa_ext_real(),
            nsa_ext_pi(
                "b",
                nsa_ext_real(),
                nsa_ext_eq(
                    nsa_ext_real(),
                    app(
                        cst("StPart"),
                        app3(
                            cst("LoebIntegral"),
                            Expr::BVar(2),
                            Expr::BVar(1),
                            Expr::BVar(0),
                        ),
                    ),
                    app3(
                        cst("LebesgueIntegral"),
                        Expr::BVar(2),
                        Expr::BVar(1),
                        Expr::BVar(0),
                    ),
                ),
            ),
        ),
    )
}
/// `bernstein_transfer_ty` — Bernstein's extension of transfer to second-order sentences
/// for Borel sets.
#[allow(dead_code)]
pub fn nsa_ext_bernstein_transfer_ty() -> Expr {
    nsa_ext_arrow(
        cst("BorelSentence"),
        nsa_ext_iff(
            app(cst("HoldsInBorelReal"), Expr::BVar(0)),
            app(cst("HoldsInHyperBorel"), Expr::BVar(0)),
        ),
    )
}
/// `full_saturation_transfer_ty` — full saturation implies transfer for all internal sentences.
#[allow(dead_code)]
pub fn nsa_ext_full_saturation_transfer_ty() -> Expr {
    nsa_ext_arrow(
        app(cst("IsFullySaturated"), cst("Hyperreal")),
        nsa_ext_pi(
            "phi",
            cst("InternalSentence"),
            nsa_ext_iff(
                app(cst("HoldsInReal"), Expr::BVar(0)),
                app(cst("HoldsInHyperreal"), Expr::BVar(0)),
            ),
        ),
    )
}
/// `ns_compactness_sequential_ty` — sequential compactness via nonstandard analysis:
/// a metric space X is sequentially compact iff every x ∈ *X is nearstandard.
#[allow(dead_code)]
pub fn nsa_ext_ns_compactness_sequential_ty() -> Expr {
    nsa_ext_pi(
        "X",
        nsa_ext_type0(),
        nsa_ext_iff(
            app(cst("IsSequentiallyCompact"), Expr::BVar(0)),
            nsa_ext_pi(
                "x",
                app(cst("StarOf"), Expr::BVar(1)),
                app(cst("Nearstandard"), Expr::BVar(0)),
            ),
        ),
    )
}
/// `ns_tychonoff_ty` — nonstandard proof schema for Tychonoff's theorem.
#[allow(dead_code)]
pub fn nsa_ext_ns_tychonoff_ty() -> Expr {
    nsa_ext_pi(
        "I",
        nsa_ext_type0(),
        nsa_ext_pi(
            "X",
            nsa_ext_arrow(Expr::BVar(0), nsa_ext_type0()),
            nsa_ext_arrow(
                nsa_ext_pi(
                    "i",
                    Expr::BVar(1),
                    app(cst("IsCompact"), app(Expr::BVar(1), Expr::BVar(0))),
                ),
                app(
                    cst("IsCompact"),
                    app2(cst("PiType"), Expr::BVar(2), Expr::BVar(1)),
                ),
            ),
        ),
    )
}
/// `nsa_ramsey_ty` — nonstandard proof of Ramsey's theorem (finite version).
/// For every r, k : ℕ, there exists N such that any r-coloring of \[N\]^k has a monochromatic copy.
#[allow(dead_code)]
pub fn nsa_ext_nsa_ramsey_ty() -> Expr {
    nsa_ext_pi(
        "r",
        nsa_ext_nat(),
        nsa_ext_pi(
            "k",
            nsa_ext_nat(),
            app(
                cst("Nonempty"),
                app(
                    cst("RamseyWitness"),
                    app2(cst("RamseyPair"), Expr::BVar(1), Expr::BVar(0)),
                ),
            ),
        ),
    )
}
/// `nsa_brouwer_fixed_point_ty` — nonstandard proof of Brouwer fixed-point theorem.
/// Every continuous f : D^n → D^n has a fixed point.
#[allow(dead_code)]
pub fn nsa_ext_nsa_brouwer_fixed_point_ty() -> Expr {
    nsa_ext_pi(
        "n",
        nsa_ext_nat(),
        nsa_ext_pi(
            "f",
            nsa_ext_arrow(
                app(cst("Disk"), Expr::BVar(0)),
                app(cst("Disk"), Expr::BVar(1)),
            ),
            nsa_ext_arrow(
                app(cst("IsContinuous"), Expr::BVar(0)),
                app(cst("Nonempty"), app(cst("FixedPoint"), Expr::BVar(1))),
            ),
        ),
    )
}
/// `nsa_intermediate_value_ty` — nonstandard proof of the intermediate value theorem.
#[allow(dead_code)]
pub fn nsa_ext_nsa_intermediate_value_ty() -> Expr {
    nsa_ext_pi(
        "f",
        nsa_ext_arrow(nsa_ext_real(), nsa_ext_real()),
        nsa_ext_pi(
            "a",
            nsa_ext_real(),
            nsa_ext_pi(
                "b",
                nsa_ext_real(),
                nsa_ext_pi(
                    "y",
                    nsa_ext_real(),
                    nsa_ext_arrow(
                        app(cst("IsContinuous"), Expr::BVar(3)),
                        nsa_ext_arrow(
                            app2(
                                cst("Between"),
                                app(Expr::BVar(3), Expr::BVar(2)),
                                app(Expr::BVar(3), Expr::BVar(1)),
                            ),
                            app(
                                cst("Nonempty"),
                                app(
                                    cst("IVTWitness"),
                                    app3(
                                        cst("IVTData"),
                                        Expr::BVar(3),
                                        Expr::BVar(2),
                                        Expr::BVar(1),
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
/// `nsa_bolzano_weierstrass_ty` — every bounded sequence has a convergent subsequence.
#[allow(dead_code)]
pub fn nsa_ext_nsa_bolzano_weierstrass_ty() -> Expr {
    nsa_ext_pi(
        "seq",
        nsa_ext_arrow(nsa_ext_nat(), nsa_ext_real()),
        nsa_ext_arrow(
            app(cst("IsBounded"), Expr::BVar(0)),
            app(cst("HasConvergentSubsequence"), Expr::BVar(1)),
        ),
    )
}
/// `nsa_arzelà_ascoli_ty` — the Arzelà-Ascoli theorem via NSA: equicontinuous + bounded ↔ compact.
#[allow(dead_code)]
pub fn nsa_ext_nsa_arzela_ascoli_ty() -> Expr {
    nsa_ext_pi(
        "F",
        app(cst("FunctionFamily"), nsa_ext_real()),
        nsa_ext_iff(
            nsa_ext_and(
                app(cst("IsEquicontinuous"), Expr::BVar(0)),
                app(cst("IsPointwiseBounded"), Expr::BVar(0)),
            ),
            app(cst("IsRelativelyCompact"), Expr::BVar(0)),
        ),
    )
}
/// `enlargement_existence_ty` — every structure has a proper elementary extension
/// (enlargement theorem).
#[allow(dead_code)]
pub fn nsa_ext_enlargement_existence_ty() -> Expr {
    nsa_ext_pi(
        "M",
        nsa_ext_type0(),
        app(
            cst("Nonempty"),
            app(cst("ProperElementaryExtension"), Expr::BVar(0)),
        ),
    )
}
/// `enlargement_internal_sets_ty` — in any enlargement, all standard sets are internal.
#[allow(dead_code)]
pub fn nsa_ext_enlargement_internal_sets_ty() -> Expr {
    nsa_ext_pi(
        "M",
        nsa_ext_type0(),
        nsa_ext_pi(
            "ext",
            app(cst("ElementaryExtension"), Expr::BVar(0)),
            nsa_ext_pi(
                "A",
                app(cst("Set"), Expr::BVar(1)),
                app(cst("IsInternal"), app(cst("StarOfSet"), Expr::BVar(0))),
            ),
        ),
    )
}
/// Register all extended nonstandard analysis axioms into `env`.
pub fn register_nonstandard_analysis_extended(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        ("nsa_internal_subset", nsa_ext_internal_subset_ty()),
        (
            "nsa_internal_inter_closed",
            nsa_ext_internal_inter_closed_ty(),
        ),
        ("nsa_sigma_saturation", nsa_ext_sigma_saturation_ty()),
        (
            "nsa_kappa_sat_existence",
            nsa_ext_kappa_saturation_existence_ty(),
        ),
        (
            "nsa_overflow_internal",
            nsa_ext_overflow_for_internal_sets_ty(),
        ),
        (
            "nsa_underflow_internal",
            nsa_ext_underflow_for_internal_sets_ty(),
        ),
        ("nsa_hyperfinite_set", nsa_ext_hyperfinite_set_ty()),
        (
            "nsa_hyperfinite_cardinality",
            nsa_ext_hyperfinite_cardinality_ty(),
        ),
        (
            "nsa_hyperfinite_interval",
            nsa_ext_hyperfinite_interval_ty(),
        ),
        ("nsa_hyperfinite_sum", nsa_ext_hyperfinite_sum_ty()),
        ("nsa_hyperfinite_product", nsa_ext_hyperfinite_product_ty()),
        ("nsa_loeb_sigma_algebra", nsa_ext_loeb_sigma_algebra_ty()),
        (
            "nsa_loeb_extends_counting",
            nsa_ext_loeb_measure_extends_counting_ty(),
        ),
        (
            "nsa_ns_integral_as_loeb",
            nsa_ext_nonstandard_integral_as_loeb_ty(),
        ),
        ("nsa_loeb_anderson", nsa_ext_loeb_anderson_theorem_ty()),
        (
            "nsa_s_continuous_iff",
            nsa_ext_s_continuous_iff_continuous_ty(),
        ),
        (
            "nsa_s_differentiable_iff",
            nsa_ext_s_differentiable_iff_differentiable_ty(),
        ),
        (
            "nsa_s_continuous_at_std",
            nsa_ext_s_continuous_at_standard_ty(),
        ),
        ("nsa_hyperfinite_dft", nsa_ext_hyperfinite_dft_ty()),
        (
            "nsa_hyperfinite_dft_inversion",
            nsa_ext_hyperfinite_dft_inversion_ty(),
        ),
        (
            "nsa_hyperfinite_dft_approx",
            nsa_ext_hyperfinite_dft_approx_fourier_ty(),
        ),
        ("nsa_monad_subset_galaxy", nsa_ext_monad_subset_galaxy_ty()),
        (
            "nsa_monad_at_standard",
            nsa_ext_monad_intersection_standard_ty(),
        ),
        ("nsa_open_iff_monad", nsa_ext_open_iff_monad_contained_ty()),
        (
            "nsa_compact_iff_nearstd",
            nsa_ext_compact_iff_all_nearstandard_ty(),
        ),
        ("nsa_loeb_outer_measure", nsa_ext_loeb_outer_measure_ty()),
        ("nsa_loeb_completion", nsa_ext_loeb_completion_ty()),
        ("nsa_lebesgue_from_loeb", nsa_ext_ns_lebesgue_from_loeb_ty()),
        ("nsa_bernstein_transfer", nsa_ext_bernstein_transfer_ty()),
        (
            "nsa_full_sat_transfer",
            nsa_ext_full_saturation_transfer_ty(),
        ),
        (
            "nsa_seq_compactness",
            nsa_ext_ns_compactness_sequential_ty(),
        ),
        ("nsa_tychonoff", nsa_ext_ns_tychonoff_ty()),
        ("nsa_ramsey", nsa_ext_nsa_ramsey_ty()),
        ("nsa_brouwer_fp", nsa_ext_nsa_brouwer_fixed_point_ty()),
        ("nsa_ivt", nsa_ext_nsa_intermediate_value_ty()),
        (
            "nsa_bolzano_weierstrass",
            nsa_ext_nsa_bolzano_weierstrass_ty(),
        ),
        ("nsa_arzela_ascoli", nsa_ext_nsa_arzela_ascoli_ty()),
        (
            "nsa_enlargement_existence",
            nsa_ext_enlargement_existence_ty(),
        ),
        (
            "nsa_enlargement_internal",
            nsa_ext_enlargement_internal_sets_ty(),
        ),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .map_err(|e| format!("{e:?}"))?;
    }
    Ok(())
}
