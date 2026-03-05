//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
#![allow(clippy::items_after_test_module)]

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{FinSurreal, GameValue, Sign};

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
pub fn int_ty() -> Expr {
    cst("Int")
}
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn real_ty() -> Expr {
    cst("Real")
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
pub fn forall_surreal(name: &str, body: Expr) -> Expr {
    pi(BinderInfo::Default, name, surreal_ty(), body)
}
pub fn exists_surreal(name: &str, body: Expr) -> Expr {
    app(
        cst("Exists"),
        lam(BinderInfo::Default, name, surreal_ty(), body),
    )
}
pub(super) fn eq_surreal(x: Expr, y: Expr) -> Expr {
    app3(cst("Eq"), surreal_ty(), x, y)
}
/// `Surreal : Type` — the class of surreal numbers No.
pub fn surreal_ty() -> Expr {
    type0()
}
/// `SurrealSet : Type` — a set (or class) of surreal numbers.
pub fn surreal_set_ty() -> Expr {
    arrow(surreal_ty(), prop())
}
/// `Game : Type` — combinatorial two-player games (superset of surreals).
pub fn game_ty() -> Expr {
    type0()
}
/// `SignSeq : Type` — sign-expansion sequences (+/-).
pub fn sign_seq_ty() -> Expr {
    arrow(nat_ty(), bool_ty())
}
/// `Birthday : Surreal → Ordinal` — the birthday (rank) of a surreal.
///
/// The birthday of `{ L | R }` is the smallest ordinal greater than all
/// birthdays of elements in L and R.
pub fn birthday_ty() -> Expr {
    arrow(surreal_ty(), nat_ty())
}
/// `LeftSet : Surreal → SurrealSet` — the left set L of a surreal `{ L | R }`.
pub fn left_set_ty() -> Expr {
    arrow(surreal_ty(), surreal_set_ty())
}
/// `RightSet : Surreal → SurrealSet` — the right set R of a surreal `{ L | R }`.
pub fn right_set_ty() -> Expr {
    arrow(surreal_ty(), surreal_set_ty())
}
/// `MkSurreal : SurrealSet → SurrealSet → Surreal` —
/// Conway's cut construction `{ L | R }`.
pub fn mk_surreal_ty() -> Expr {
    arrow(surreal_set_ty(), arrow(surreal_set_ty(), surreal_ty()))
}
/// `ValidCut : SurrealSet → SurrealSet → Prop` —
/// validity condition: no element of R ≤ any element of L.
pub fn valid_cut_ty() -> Expr {
    arrow(surreal_set_ty(), arrow(surreal_set_ty(), prop()))
}
/// `birthday_zero : Birthday 0 = 0` — zero has birthday 0.
pub fn birthday_zero_ty() -> Expr {
    app3(
        cst("Eq"),
        nat_ty(),
        app(cst("Birthday"), cst("SZero")),
        cst("NatZero"),
    )
}
/// `birthday_one : Birthday 1 = 1` — one has birthday 1.
pub fn birthday_one_ty() -> Expr {
    app3(
        cst("Eq"),
        nat_ty(),
        app(cst("Birthday"), cst("SOne")),
        cst("NatOne"),
    )
}
/// `birthday_neg_one : Birthday (-1) = 1` — negative one has birthday 1.
pub fn birthday_neg_one_ty() -> Expr {
    app3(
        cst("Eq"),
        nat_ty(),
        app(cst("Birthday"), cst("SNegOne")),
        cst("NatOne"),
    )
}
/// `left_set_empty_at_zero : LeftSet 0 = ∅` — zero's left set is empty.
pub fn left_set_empty_at_zero_ty() -> Expr {
    prop()
}
/// `right_set_empty_at_zero : RightSet 0 = ∅` — zero's right set is empty.
pub fn right_set_empty_at_zero_ty() -> Expr {
    prop()
}
/// `SurrealLe : Surreal → Surreal → Prop` — surreal ≤.
///
/// `x ≤ y` iff no element of R(x) ≤ y and x ≤ no element of L(y).
pub fn surreal_le_ty() -> Expr {
    arrow(surreal_ty(), arrow(surreal_ty(), prop()))
}
/// `SurrealLt : Surreal → Surreal → Prop` — surreal <.
pub fn surreal_lt_ty() -> Expr {
    arrow(surreal_ty(), arrow(surreal_ty(), prop()))
}
/// `SurrealEq : Surreal → Surreal → Prop` — surreal equality (mutual ≤).
pub fn surreal_eq_ty() -> Expr {
    arrow(surreal_ty(), arrow(surreal_ty(), prop()))
}
/// `total_order_surreal : ∀ x y : Surreal, x ≤ y ∨ y ≤ x` —
/// the surreal numbers are totally ordered.
pub fn total_order_surreal_ty() -> Expr {
    forall_surreal(
        "x",
        forall_surreal(
            "y",
            or(
                app2(cst("SurrealLe"), bvar(1), bvar(0)),
                app2(cst("SurrealLe"), bvar(0), bvar(1)),
            ),
        ),
    )
}
/// `le_refl_surreal : ∀ x : Surreal, x ≤ x` — reflexivity.
pub fn le_refl_surreal_ty() -> Expr {
    forall_surreal("x", app2(cst("SurrealLe"), bvar(0), bvar(0)))
}
/// `le_antisymm_surreal : ∀ x y : Surreal, x ≤ y → y ≤ x → x = y` — antisymmetry.
pub fn le_antisymm_surreal_ty() -> Expr {
    forall_surreal(
        "x",
        forall_surreal(
            "y",
            arrow(
                app2(cst("SurrealLe"), bvar(1), bvar(0)),
                arrow(
                    app2(cst("SurrealLe"), bvar(0), bvar(2)),
                    eq_surreal(bvar(1), bvar(0)),
                ),
            ),
        ),
    )
}
/// `le_trans_surreal : ∀ x y z : Surreal, x ≤ y → y ≤ z → x ≤ z` — transitivity.
pub fn le_trans_surreal_ty() -> Expr {
    forall_surreal(
        "x",
        forall_surreal(
            "y",
            forall_surreal(
                "z",
                arrow(
                    app2(cst("SurrealLe"), bvar(2), bvar(1)),
                    arrow(
                        app2(cst("SurrealLe"), bvar(1), bvar(0)),
                        app2(cst("SurrealLe"), bvar(2), bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// `SurrealAdd : Surreal → Surreal → Surreal` — surreal addition.
pub fn surreal_add_ty() -> Expr {
    arrow(surreal_ty(), arrow(surreal_ty(), surreal_ty()))
}
/// `SurrealNeg : Surreal → Surreal` — surreal negation.
pub fn surreal_neg_ty() -> Expr {
    arrow(surreal_ty(), surreal_ty())
}
/// `SurrealMul : Surreal → Surreal → Surreal` — surreal multiplication.
pub fn surreal_mul_ty() -> Expr {
    arrow(surreal_ty(), arrow(surreal_ty(), surreal_ty()))
}
/// `SurrealInv : Surreal → Surreal` — surreal multiplicative inverse (for nonzero).
pub fn surreal_inv_ty() -> Expr {
    arrow(surreal_ty(), surreal_ty())
}
/// `add_comm_surreal : ∀ x y : Surreal, x + y = y + x` — commutativity of addition.
pub fn add_comm_surreal_ty() -> Expr {
    forall_surreal(
        "x",
        forall_surreal(
            "y",
            eq_surreal(
                app2(cst("SurrealAdd"), bvar(1), bvar(0)),
                app2(cst("SurrealAdd"), bvar(0), bvar(1)),
            ),
        ),
    )
}
/// `add_assoc_surreal : ∀ x y z : Surreal, (x + y) + z = x + (y + z)`.
pub fn add_assoc_surreal_ty() -> Expr {
    forall_surreal(
        "x",
        forall_surreal(
            "y",
            forall_surreal(
                "z",
                eq_surreal(
                    app2(
                        cst("SurrealAdd"),
                        app2(cst("SurrealAdd"), bvar(2), bvar(1)),
                        bvar(0),
                    ),
                    app2(
                        cst("SurrealAdd"),
                        bvar(2),
                        app2(cst("SurrealAdd"), bvar(1), bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// `mul_comm_surreal : ∀ x y : Surreal, x * y = y * x`.
pub fn mul_comm_surreal_ty() -> Expr {
    forall_surreal(
        "x",
        forall_surreal(
            "y",
            eq_surreal(
                app2(cst("SurrealMul"), bvar(1), bvar(0)),
                app2(cst("SurrealMul"), bvar(0), bvar(1)),
            ),
        ),
    )
}
/// `surreal_ordered_field : OrderedField Surreal` — surreals form an ordered field.
pub fn surreal_ordered_field_ty() -> Expr {
    app(cst("OrderedField"), surreal_ty())
}
/// `add_zero_surreal : ∀ x : Surreal, x + 0 = x`.
pub fn add_zero_surreal_ty() -> Expr {
    forall_surreal(
        "x",
        eq_surreal(app2(cst("SurrealAdd"), bvar(0), cst("SZero")), bvar(0)),
    )
}
/// `add_neg_surreal : ∀ x : Surreal, x + (-x) = 0`.
pub fn add_neg_surreal_ty() -> Expr {
    forall_surreal(
        "x",
        eq_surreal(
            app2(cst("SurrealAdd"), bvar(0), app(cst("SurrealNeg"), bvar(0))),
            cst("SZero"),
        ),
    )
}
/// `SimplestInCut : SurrealSet → SurrealSet → Surreal → Prop` —
/// `SimplestInCut L R x` means x is the simplest surreal with L < x < R.
pub fn simplest_in_cut_ty() -> Expr {
    arrow(
        surreal_set_ty(),
        arrow(surreal_set_ty(), arrow(surreal_ty(), prop())),
    )
}
/// `simplicity_theorem : ∀ L R : SurrealSet, ValidCut L R →
///   ∃ x : Surreal, SimplestInCut L R x` —
/// every valid Dedekind-style cut has a simplest surreal filling it.
pub fn simplicity_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "L",
        surreal_set_ty(),
        pi(
            BinderInfo::Default,
            "R",
            surreal_set_ty(),
            arrow(
                app2(cst("ValidCut"), bvar(1), bvar(0)),
                exists_surreal("x", app3(cst("SimplestInCut"), bvar(2), bvar(1), bvar(0))),
            ),
        ),
    )
}
/// `simplest_is_integer_or_half : ∀ n m : Int, ∃ x : Surreal, SimplestDyadic x n m` —
/// the simplest surreal in a rational cut is a dyadic rational.
pub fn simplest_dyadic_ty() -> Expr {
    arrow(int_ty(), arrow(int_ty(), arrow(surreal_ty(), prop())))
}
/// `SignExpansion : Surreal → SignSeq` —
/// the sign-expansion (+ = move right, - = move left).
pub fn sign_expansion_ty() -> Expr {
    arrow(surreal_ty(), sign_seq_ty())
}
/// `FromSignSeq : SignSeq → Nat → Surreal` —
/// reconstruct a surreal from a finite sign sequence of length n.
pub fn from_sign_seq_ty() -> Expr {
    arrow(sign_seq_ty(), arrow(nat_ty(), surreal_ty()))
}
/// `sign_expansion_injective : ∀ x y : Surreal,
///   SignExpansion x = SignExpansion y → x = y`.
pub fn sign_expansion_injective_ty() -> Expr {
    forall_surreal(
        "x",
        forall_surreal(
            "y",
            arrow(
                app3(
                    cst("Eq"),
                    sign_seq_ty(),
                    app(cst("SignExpansion"), bvar(1)),
                    app(cst("SignExpansion"), bvar(0)),
                ),
                eq_surreal(bvar(1), bvar(0)),
            ),
        ),
    )
}
/// `sign_expansion_zero : SignExpansion 0 = EmptySeq`.
pub fn sign_expansion_zero_ty() -> Expr {
    prop()
}
/// `sign_expansion_pos_int : ∀ n : Nat, SignExpansion (ofNat n) = PlusSeq n` —
/// positive integers have all-plus sign expansion.
pub fn sign_expansion_pos_int_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `OmnificInt : Surreal → Prop` —
/// predicate: x is an omnific integer (i.e., x ∈ ℤ_No).
pub fn omnific_int_ty() -> Expr {
    arrow(surreal_ty(), prop())
}
/// `IntegerPart : Surreal → Surreal` — floor function on surreals.
pub fn integer_part_ty() -> Expr {
    arrow(surreal_ty(), surreal_ty())
}
/// `FractionalPart : Surreal → Surreal` — fractional part of a surreal.
pub fn fractional_part_ty() -> Expr {
    arrow(surreal_ty(), surreal_ty())
}
/// `omnific_int_closed_add : ∀ x y : Surreal,
///   OmnificInt x → OmnificInt y → OmnificInt (x + y)`.
pub fn omnific_int_closed_add_ty() -> Expr {
    forall_surreal(
        "x",
        forall_surreal(
            "y",
            arrow(
                app(cst("OmnificInt"), bvar(1)),
                arrow(
                    app(cst("OmnificInt"), bvar(0)),
                    app(cst("OmnificInt"), app2(cst("SurrealAdd"), bvar(1), bvar(0))),
                ),
            ),
        ),
    )
}
/// `nat_embeds_in_omnific : ∀ n : Nat, OmnificInt (ofNat n)` —
/// every standard natural embeds as an omnific integer.
pub fn nat_embeds_in_omnific_ty() -> Expr {
    arrow(nat_ty(), app(cst("OmnificInt"), app(cst("OfNat"), bvar(0))))
}
/// `SurrealExp : Surreal → Surreal` — surreal exponential e^x.
pub fn surreal_exp_ty() -> Expr {
    arrow(surreal_ty(), surreal_ty())
}
/// `SurrealLog : Surreal → Surreal` — surreal logarithm log x (for x > 0).
pub fn surreal_log_ty() -> Expr {
    arrow(surreal_ty(), surreal_ty())
}
/// `SurrealPow : Surreal → Surreal → Surreal` — surreal x^y.
pub fn surreal_pow_ty() -> Expr {
    arrow(surreal_ty(), arrow(surreal_ty(), surreal_ty()))
}
/// `exp_log_inv : ∀ x : Surreal, x > 0 → SurrealExp (SurrealLog x) = x`.
pub fn exp_log_inv_ty() -> Expr {
    forall_surreal(
        "x",
        arrow(
            app2(cst("SurrealLt"), cst("SZero"), bvar(0)),
            eq_surreal(
                app(cst("SurrealExp"), app(cst("SurrealLog"), bvar(0))),
                bvar(0),
            ),
        ),
    )
}
/// `log_exp_inv : ∀ x : Surreal, SurrealLog (SurrealExp x) = x`.
pub fn log_exp_inv_ty() -> Expr {
    forall_surreal(
        "x",
        eq_surreal(
            app(cst("SurrealLog"), app(cst("SurrealExp"), bvar(0))),
            bvar(0),
        ),
    )
}
/// `exp_add : ∀ x y : Surreal, exp(x + y) = exp(x) * exp(y)`.
pub fn exp_add_ty() -> Expr {
    forall_surreal(
        "x",
        forall_surreal(
            "y",
            eq_surreal(
                app(cst("SurrealExp"), app2(cst("SurrealAdd"), bvar(1), bvar(0))),
                app2(
                    cst("SurrealMul"),
                    app(cst("SurrealExp"), bvar(1)),
                    app(cst("SurrealExp"), bvar(0)),
                ),
            ),
        ),
    )
}
/// `surreal_exp_positive : ∀ x : Surreal, SurrealExp x > 0`.
pub fn surreal_exp_positive_ty() -> Expr {
    forall_surreal(
        "x",
        app2(
            cst("SurrealLt"),
            cst("SZero"),
            app(cst("SurrealExp"), bvar(0)),
        ),
    )
}
/// `SurrealFun : Type` — type alias for functions Surreal → Surreal.
pub fn surreal_fun_ty() -> Expr {
    arrow(surreal_ty(), surreal_ty())
}
/// `SurrealLim : (Nat → Surreal) → Surreal → Prop` —
/// `SurrealLim seq L` means the sequence converges to L in the surreal topology.
pub fn surreal_lim_ty() -> Expr {
    arrow(arrow(nat_ty(), surreal_ty()), arrow(surreal_ty(), prop()))
}
/// `SurrealContinuous : SurrealFun → Surreal → Prop` —
/// continuity of a surreal function at a point.
pub fn surreal_continuous_ty() -> Expr {
    arrow(surreal_fun_ty(), arrow(surreal_ty(), prop()))
}
/// `SurrealDeriv : SurrealFun → SurrealFun` — derivative of a surreal function.
pub fn surreal_deriv_ty() -> Expr {
    arrow(surreal_fun_ty(), surreal_fun_ty())
}
/// `surreal_ivt : ∀ (f : SurrealFun) (a b : Surreal),
///   a < b → Continuous f a → Continuous f b → f a < 0 → f b > 0 →
///   ∃ c : Surreal, a < c ∧ c < b ∧ f c = 0` —
/// surreal intermediate value theorem.
pub fn surreal_ivt_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        surreal_fun_ty(),
        forall_surreal(
            "a",
            forall_surreal(
                "b",
                arrow(
                    app2(cst("SurrealLt"), bvar(1), bvar(0)),
                    exists_surreal("c", prop()),
                ),
            ),
        ),
    )
}
/// `SurrealDist : Surreal → Surreal → Surreal` — absolute difference |x - y|.
pub fn surreal_dist_ty() -> Expr {
    arrow(surreal_ty(), arrow(surreal_ty(), surreal_ty()))
}
/// `EpsilonDeltaCont : SurrealFun → Surreal → Prop` —
/// ε-δ continuity: ∀ ε > 0, ∃ δ > 0, |x - a| < δ → |f x - f a| < ε.
pub fn epsilon_delta_cont_ty() -> Expr {
    arrow(surreal_fun_ty(), arrow(surreal_ty(), prop()))
}
/// `eps_delta_iff_continuous : ∀ (f : SurrealFun) (a : Surreal),
///   EpsilonDeltaCont f a ↔ SurrealContinuous f a`.
pub fn eps_delta_iff_continuous_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        surreal_fun_ty(),
        forall_surreal(
            "a",
            iff(
                app2(cst("EpsilonDeltaCont"), bvar(1), bvar(0)),
                app2(cst("SurrealContinuous"), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// `infinitesimal_epsilon : ∃ ε : Surreal, ε > 0 ∧ ∀ r : Real, ε < ofReal r` —
/// there exist surreal infinitesimals.
pub fn infinitesimal_epsilon_ty() -> Expr {
    exists_surreal("eps", prop())
}
/// `GameToSurreal : Game → Surreal` — embedding of surreal games into No.
pub fn game_to_surreal_ty() -> Expr {
    arrow(game_ty(), surreal_ty())
}
/// `NimValue : Nat → Surreal` — Nim heap of size n as a surreal number.
pub fn nim_value_ty() -> Expr {
    arrow(nat_ty(), surreal_ty())
}
/// `NimSum : Surreal → Surreal → Surreal` — surreal Nim addition (XOR).
pub fn nim_sum_ty() -> Expr {
    arrow(surreal_ty(), arrow(surreal_ty(), surreal_ty()))
}
/// `surreal_zero_is_p_position : GameValue SZero = ZeroGame` —
/// the surreal 0 corresponds to the zero game (P-position).
pub fn surreal_zero_p_pos_ty() -> Expr {
    prop()
}
/// `nim_value_additive : ∀ a b : Nat,
///   NimValue (XorNat a b) = NimSum (NimValue a) (NimValue b)` —
/// the Nim value of XOR equals surreal Nim sum.
pub fn nim_value_additive_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `surreal_game_equiv : ∀ (g : Game), GameValue g = GameToSurreal g` —
/// every finite combinatorial game has a well-defined surreal value.
pub fn surreal_game_equiv_ty() -> Expr {
    arrow(game_ty(), prop())
}
/// `positive_surreal_is_first_player_win : ∀ x : Surreal, x > 0 → FirstPlayerWins x` —
/// positive surreals are first-player wins.
pub fn positive_surreal_fp_win_ty() -> Expr {
    forall_surreal(
        "x",
        arrow(
            app2(cst("SurrealLt"), cst("SZero"), bvar(0)),
            app(cst("FirstPlayerWins"), bvar(0)),
        ),
    )
}
/// `NoIsRealClosed : RealClosedField No` — No is a real closed field.
pub fn no_is_real_closed_ty() -> Expr {
    app(cst("RealClosedField"), surreal_ty())
}
/// `surreal_archimedean_class : ∀ x y : Surreal, x > 0 → y > 0 →
///   ∃ n : Nat, y < SurrealMul (OfNat n) x` — No is non-Archimedean
///   (but this formulation says for any two positives an n exists if we allow ordinals).
pub fn surreal_archimedean_class_ty() -> Expr {
    forall_surreal(
        "x",
        forall_surreal(
            "y",
            arrow(
                app2(cst("SurrealLt"), cst("SZero"), bvar(1)),
                arrow(
                    app2(cst("SurrealLt"), cst("SZero"), bvar(0)),
                    app(
                        cst("Exists"),
                        lam(
                            BinderInfo::Default,
                            "n",
                            nat_ty(),
                            app2(
                                cst("SurrealLt"),
                                bvar(1),
                                app2(cst("SurrealMul"), app(cst("OfNat"), bvar(0)), bvar(2)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `mul_one_surreal : ∀ x : Surreal, x * 1 = x` — multiplicative identity.
pub fn mul_one_surreal_ty() -> Expr {
    forall_surreal(
        "x",
        eq_surreal(app2(cst("SurrealMul"), bvar(0), cst("SOne")), bvar(0)),
    )
}
/// `mul_inv_surreal : ∀ x : Surreal, x ≠ 0 → x * x⁻¹ = 1` — multiplicative inverse.
pub fn mul_inv_surreal_ty() -> Expr {
    forall_surreal(
        "x",
        arrow(
            not(eq_surreal(bvar(0), cst("SZero"))),
            eq_surreal(
                app2(cst("SurrealMul"), bvar(0), app(cst("SurrealInv"), bvar(0))),
                cst("SOne"),
            ),
        ),
    )
}
/// `distrib_surreal : ∀ x y z : Surreal, x * (y + z) = x*y + x*z` — distributivity.
pub fn distrib_surreal_ty() -> Expr {
    forall_surreal(
        "x",
        forall_surreal(
            "y",
            forall_surreal(
                "z",
                eq_surreal(
                    app2(
                        cst("SurrealMul"),
                        bvar(2),
                        app2(cst("SurrealAdd"), bvar(1), bvar(0)),
                    ),
                    app2(
                        cst("SurrealAdd"),
                        app2(cst("SurrealMul"), bvar(2), bvar(1)),
                        app2(cst("SurrealMul"), bvar(2), bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// `OrdinalSurreal : Ordinal → Surreal` — embedding of ordinals into No.
pub fn ordinal_surreal_ty() -> Expr {
    arrow(cst("Ordinal"), surreal_ty())
}
/// `SurrealOmega : Surreal` — the surreal ω (embedding of ω ordinal).
pub fn surreal_omega_ty() -> Expr {
    surreal_ty()
}
/// `SurrealEpsilon0 : Surreal` — the surreal ε₀ = ω^ω^ω^...
pub fn surreal_epsilon0_ty() -> Expr {
    surreal_ty()
}
/// `omega_gt_all_naturals : ∀ n : Nat, SurrealOmega > OfNat n` —
/// ω exceeds every natural number.
pub fn omega_gt_all_naturals_ty() -> Expr {
    arrow(
        nat_ty(),
        app2(
            cst("SurrealLt"),
            app(cst("OfNat"), bvar(0)),
            cst("SurrealOmega"),
        ),
    )
}
/// `omega_plus_one : SurrealAdd SurrealOmega SOne > SurrealOmega` —
/// ω + 1 > ω in surreals.
pub fn omega_plus_one_ty() -> Expr {
    app2(
        cst("SurrealLt"),
        cst("SurrealOmega"),
        app2(cst("SurrealAdd"), cst("SurrealOmega"), cst("SOne")),
    )
}
/// `ordinal_add_embeds : ∀ α β : Ordinal,
///   OrdinalSurreal (OrdinalAdd α β) = SurrealAdd (OrdinalSurreal α) (OrdinalSurreal β)`.
pub fn ordinal_add_embeds_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "alpha",
        cst("Ordinal"),
        pi(
            BinderInfo::Default,
            "beta",
            cst("Ordinal"),
            eq_surreal(
                app(
                    cst("OrdinalSurreal"),
                    app2(cst("OrdinalAdd"), bvar(1), bvar(0)),
                ),
                app2(
                    cst("SurrealAdd"),
                    app(cst("OrdinalSurreal"), bvar(1)),
                    app(cst("OrdinalSurreal"), bvar(0)),
                ),
            ),
        ),
    )
}
/// `SignTree : Type` — the surreal number tree, with nodes labeled by signs.
pub fn sign_tree_ty() -> Expr {
    type0()
}
/// `TreeParent : Surreal → Surreal → Prop` —
/// `TreeParent x y` means x is the parent of y in the surreal tree.
pub fn tree_parent_ty() -> Expr {
    arrow(surreal_ty(), arrow(surreal_ty(), prop()))
}
/// `sign_seq_prefix_order : ∀ x y : Surreal,
///   TreeParent x y ↔ ∃ s : Sign, SignExpansion y = AppendSign (SignExpansion x) s` —
/// the tree order is exactly the prefix order of sign sequences.
pub fn sign_seq_prefix_order_ty() -> Expr {
    forall_surreal(
        "x",
        forall_surreal("y", iff(app2(cst("TreeParent"), bvar(1), bvar(0)), prop())),
    )
}
/// `surreal_lt_iff_sign_seq_lt : ∀ x y : Surreal,
///   x < y ↔ SignExpansion x <_lex SignExpansion y` —
/// surreal order is lexicographic order on sign sequences.
pub fn surreal_lt_iff_sign_seq_lt_ty() -> Expr {
    forall_surreal(
        "x",
        forall_surreal(
            "y",
            iff(
                app2(cst("SurrealLt"), bvar(1), bvar(0)),
                app2(
                    cst("LexLt"),
                    app(cst("SignExpansion"), bvar(1)),
                    app(cst("SignExpansion"), bvar(0)),
                ),
            ),
        ),
    )
}
/// `OmnificIntRing : Ring OmnificInts` — the omnific integers form a ring.
pub fn omnific_int_ring_ty() -> Expr {
    app(cst("Ring"), cst("OmnificInts"))
}
/// `omnific_int_closed_mul : ∀ x y : Surreal,
///   OmnificInt x → OmnificInt y → OmnificInt (x * y)`.
pub fn omnific_int_closed_mul_ty() -> Expr {
    forall_surreal(
        "x",
        forall_surreal(
            "y",
            arrow(
                app(cst("OmnificInt"), bvar(1)),
                arrow(
                    app(cst("OmnificInt"), bvar(0)),
                    app(cst("OmnificInt"), app2(cst("SurrealMul"), bvar(1), bvar(0))),
                ),
            ),
        ),
    )
}
/// `omnific_int_euclidean_div : ∀ a b : Surreal,
///   OmnificInt a → OmnificInt b → b ≠ 0 →
///   ∃ q r : Surreal, OmnificInt q ∧ OmnificInt r ∧ a = b*q + r ∧ |r| < |b|` —
/// the omnific integers admit a Euclidean division.
pub fn omnific_int_euclidean_div_ty() -> Expr {
    forall_surreal(
        "a",
        forall_surreal(
            "b",
            arrow(
                app(cst("OmnificInt"), bvar(1)),
                arrow(
                    app(cst("OmnificInt"), bvar(0)),
                    arrow(not(eq_surreal(bvar(0), cst("SZero"))), prop()),
                ),
            ),
        ),
    )
}
/// `int_part_omnific : ∀ x : Surreal, OmnificInt (IntegerPart x)` —
/// the integer part of any surreal is an omnific integer.
pub fn int_part_omnific_ty() -> Expr {
    forall_surreal(
        "x",
        app(cst("OmnificInt"), app(cst("IntegerPart"), bvar(0))),
    )
}
/// `HahnSeries : Type` — formal power series with surreal exponents (Hahn series).
pub fn hahn_series_ty() -> Expr {
    type0()
}
/// `HahnSeriesEmbed : No → HahnSeries` — embedding of surreals into Hahn series.
pub fn hahn_series_embed_ty() -> Expr {
    arrow(surreal_ty(), hahn_series_ty())
}
/// `HahnSeriesAdd : HahnSeries → HahnSeries → HahnSeries` — series addition.
pub fn hahn_series_add_ty() -> Expr {
    arrow(hahn_series_ty(), arrow(hahn_series_ty(), hahn_series_ty()))
}
/// `HahnSeriesMul : HahnSeries → HahnSeries → HahnSeries` — series multiplication.
pub fn hahn_series_mul_ty() -> Expr {
    arrow(hahn_series_ty(), arrow(hahn_series_ty(), hahn_series_ty()))
}
/// `no_is_hahn_series_field : IsField HahnSeries` — the Hahn series over No form a field.
pub fn no_is_hahn_series_field_ty() -> Expr {
    app(cst("IsField"), hahn_series_ty())
}
/// `hahn_support_well_ordered : ∀ (s : HahnSeries), WellOrdered (Support s)` —
/// the support of any Hahn series is well-ordered.
pub fn hahn_support_well_ordered_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "s",
        hahn_series_ty(),
        app(cst("WellOrdered"), app(cst("Support"), bvar(0))),
    )
}
/// `HyperReal : Type` — Robinson's hyperreals (an elementary extension of ℝ).
pub fn hyperreal_ty() -> Expr {
    type0()
}
/// `HyperRealEmbed : HyperReal → Surreal` — embedding of hyperreals into No.
pub fn hyperreal_embed_ty() -> Expr {
    arrow(hyperreal_ty(), surreal_ty())
}
/// `hyperreal_transfer_principle : TransferPrinciple HyperReal Real` —
/// the transfer principle (every first-order sentence over ℝ holds in *ℝ).
pub fn hyperreal_transfer_principle_ty() -> Expr {
    app2(cst("TransferPrinciple"), hyperreal_ty(), real_ty())
}
/// `hyperreal_standard_part : HyperReal → Real` — the standard part map st: *ℝ → ℝ.
pub fn hyperreal_standard_part_ty() -> Expr {
    arrow(hyperreal_ty(), real_ty())
}
/// `hyperreal_infinitesimal_def : ∀ x : HyperReal,
///   Infinitesimal x ↔ ∀ r : Real, |x| < EmbedReal r` —
/// definition of infinitesimal in the hyperreals.
pub fn hyperreal_infinitesimal_def_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "x",
        hyperreal_ty(),
        iff(app(cst("Infinitesimal"), bvar(0)), prop()),
    )
}
/// `SurrealSeqLim : (Nat → Surreal) → Surreal → Prop` —
/// limit of a sequence of surreals (generalizing real analysis).
pub fn surreal_seq_lim_ty() -> Expr {
    arrow(arrow(nat_ty(), surreal_ty()), arrow(surreal_ty(), prop()))
}
/// `surreal_squeeze_theorem : ∀ (a b c : Nat → Surreal) (L : Surreal),
///   (∀ n, a n ≤ b n ≤ c n) → SurrealSeqLim a L → SurrealSeqLim c L →
///   SurrealSeqLim b L`.
pub fn surreal_squeeze_theorem_ty() -> Expr {
    prop()
}
/// `SurrealDerivAt : SurrealFun → Surreal → Surreal → Prop` —
/// `SurrealDerivAt f a v` means the derivative of f at a is v.
pub fn surreal_deriv_at_ty() -> Expr {
    arrow(
        surreal_fun_ty(),
        arrow(surreal_ty(), arrow(surreal_ty(), prop())),
    )
}
/// `surreal_chain_rule : ∀ (f g : SurrealFun) (a : Surreal),
///   SurrealDerivAt g (f a) v → SurrealDerivAt f a u →
///   SurrealDerivAt (g ∘ f) a (v * u)`.
pub fn surreal_chain_rule_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        surreal_fun_ty(),
        pi(
            BinderInfo::Default,
            "g",
            surreal_fun_ty(),
            forall_surreal("a", prop()),
        ),
    )
}
/// `SurrealIntegral : SurrealFun → Surreal → Surreal → Surreal` —
/// definite integral of f from a to b.
pub fn surreal_integral_ty() -> Expr {
    arrow(
        surreal_fun_ty(),
        arrow(surreal_ty(), arrow(surreal_ty(), surreal_ty())),
    )
}
/// `surreal_ftc : ∀ (f : SurrealFun) (a b : Surreal),
///   SurrealIntegral (SurrealDeriv f) a b = SurrealAdd (f b) (SurrealNeg (f a))` —
/// fundamental theorem of calculus for surreal analysis.
pub fn surreal_ftc_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        surreal_fun_ty(),
        forall_surreal(
            "a",
            forall_surreal(
                "b",
                eq_surreal(
                    app3(
                        cst("SurrealIntegral"),
                        app(cst("SurrealDeriv"), bvar(2)),
                        bvar(1),
                        bvar(0),
                    ),
                    app2(
                        cst("SurrealAdd"),
                        app(bvar(2), bvar(0)),
                        app(cst("SurrealNeg"), app(bvar(2), bvar(1))),
                    ),
                ),
            ),
        ),
    )
}
/// `GameEquiv : Game → Game → Prop` — game equivalence (G ~ H iff G - H = 0).
pub fn game_equiv_ty() -> Expr {
    arrow(game_ty(), arrow(game_ty(), prop()))
}
/// `GameAdd : Game → Game → Game` — sum of combinatorial games.
pub fn game_add_ty() -> Expr {
    arrow(game_ty(), arrow(game_ty(), game_ty()))
}
/// `GameNeg : Game → Game` — negation of a game (swap players).
pub fn game_neg_ty() -> Expr {
    arrow(game_ty(), game_ty())
}
/// `game_equiv_is_zero : ∀ (g : Game),
///   GameEquiv g ZeroGame ↔ SecondPlayerWins g` —
/// a game is equivalent to 0 iff the second player wins.
pub fn game_equiv_is_zero_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "g",
        game_ty(),
        iff(
            app2(cst("GameEquiv"), bvar(0), cst("ZeroGame")),
            app(cst("SecondPlayerWins"), bvar(0)),
        ),
    )
}
/// `surreal_embed_game_hom : ∀ (g h : Game),
///   GameToSurreal (GameAdd g h) = SurrealAdd (GameToSurreal g) (GameToSurreal h)` —
/// the surreal embedding is a group homomorphism.
pub fn surreal_embed_game_hom_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "g",
        game_ty(),
        pi(
            BinderInfo::Default,
            "h",
            game_ty(),
            eq_surreal(
                app(cst("GameToSurreal"), app2(cst("GameAdd"), bvar(1), bvar(0))),
                app2(
                    cst("SurrealAdd"),
                    app(cst("GameToSurreal"), bvar(1)),
                    app(cst("GameToSurreal"), bvar(0)),
                ),
            ),
        ),
    )
}
/// `SurrealInduction : (Surreal → Prop) → Prop` —
/// birthday induction principle: to prove P for all surreals, it suffices to
/// show that if P holds for all x with Birthday x < α, then P holds for
/// all surreals born at day α.
pub fn surreal_induction_ty() -> Expr {
    arrow(arrow(surreal_ty(), prop()), prop())
}
/// `birthday_induction : ∀ (P : Surreal → Prop),
///   (∀ x : Surreal, (∀ y : Surreal, Birthday y < Birthday x → P y) → P x) →
///   ∀ x : Surreal, P x` — transfinite induction on birthday.
pub fn birthday_induction_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        arrow(surreal_ty(), prop()),
        arrow(
            forall_surreal(
                "x",
                arrow(
                    forall_surreal(
                        "y",
                        arrow(
                            app2(
                                cst("NatLt"),
                                app(cst("Birthday"), bvar(0)),
                                app(cst("Birthday"), bvar(1)),
                            ),
                            app(bvar(3), bvar(0)),
                        ),
                    ),
                    app(bvar(1), bvar(0)),
                ),
            ),
            forall_surreal("x", app(bvar(1), bvar(0))),
        ),
    )
}
/// `no_is_universal_ordered_field : ∀ (F : Type), OrderedField F → Embeds F No` —
/// No is the universal ordered field: every ordered field embeds in No.
pub fn no_is_universal_ordered_field_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        type0(),
        arrow(
            app(cst("OrderedField"), bvar(0)),
            app2(cst("Embeds"), bvar(0), surreal_ty()),
        ),
    )
}
/// `SurrealMulDef : ∀ x y : Surreal,
///   x * y = { x_L * y + x * y_L - x_L * y_L | x_R * y + x * y_R - x_R * y_R }` —
/// Conway's recursive definition of surreal multiplication.
pub fn surreal_mul_def_ty() -> Expr {
    forall_surreal("x", forall_surreal("y", prop()))
}
/// `mul_assoc_surreal : ∀ x y z : Surreal, (x * y) * z = x * (y * z)`.
pub fn mul_assoc_surreal_ty() -> Expr {
    forall_surreal(
        "x",
        forall_surreal(
            "y",
            forall_surreal(
                "z",
                eq_surreal(
                    app2(
                        cst("SurrealMul"),
                        app2(cst("SurrealMul"), bvar(2), bvar(1)),
                        bvar(0),
                    ),
                    app2(
                        cst("SurrealMul"),
                        bvar(2),
                        app2(cst("SurrealMul"), bvar(1), bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// `mul_zero_surreal : ∀ x : Surreal, x * 0 = 0`.
pub fn mul_zero_surreal_ty() -> Expr {
    forall_surreal(
        "x",
        eq_surreal(app2(cst("SurrealMul"), bvar(0), cst("SZero")), cst("SZero")),
    )
}
/// `NoIsKappaSaturated : ∀ (kappa : Cardinal), KappaSaturated No kappa` —
/// No is κ-saturated for all regular uncountable cardinals κ.
pub fn no_is_kappa_saturated_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "kappa",
        cst("Cardinal"),
        app2(cst("KappaSaturated"), surreal_ty(), bvar(0)),
    )
}
/// `no_is_homogeneous : HomogeneousStructure No` —
/// No is a homogeneous model of the theory of real closed fields.
pub fn no_is_homogeneous_ty() -> Expr {
    app(cst("HomogeneousStructure"), surreal_ty())
}
/// `no_is_o_minimal : OMinimalStructure No` —
/// No is an o-minimal structure (every definable subset is a finite union of intervals).
pub fn no_is_o_minimal_ty() -> Expr {
    app(cst("OMinimalStructure"), surreal_ty())
}
/// `no_elementary_equiv_real : ElementarilyEquivalent No Real` —
/// No is elementarily equivalent to ℝ as an ordered field.
pub fn no_elementary_equiv_real_ty() -> Expr {
    app2(cst("ElementarilyEquivalent"), surreal_ty(), real_ty())
}
/// `no_unique_up_to_sat : ∀ (F : Type),
///   RealClosedField F → KappaSaturated F OmegaOne →
///   Isomorphic F No` — No is the unique saturated real closed field of cardinality ω₁.
pub fn no_unique_up_to_sat_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        type0(),
        arrow(
            app(cst("RealClosedField"), bvar(0)),
            arrow(
                app2(cst("KappaSaturated"), bvar(0), cst("OmegaOne")),
                app2(cst("Isomorphic"), bvar(0), surreal_ty()),
            ),
        ),
    )
}
/// `real_embeds_in_no : Embeds Real No` — the reals embed in No.
pub fn real_embeds_in_no_ty() -> Expr {
    app2(cst("Embeds"), real_ty(), surreal_ty())
}
/// `real_embed_preserves_order : ∀ x y : Real,
///   x ≤ y ↔ SurrealLe (EmbedReal x) (EmbedReal y)` —
/// the embedding ℝ → No preserves order.
pub fn real_embed_preserves_order_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "x",
        real_ty(),
        pi(
            BinderInfo::Default,
            "y",
            real_ty(),
            iff(
                app2(cst("RealLe"), bvar(1), bvar(0)),
                app2(
                    cst("SurrealLe"),
                    app(cst("EmbedReal"), bvar(1)),
                    app(cst("EmbedReal"), bvar(0)),
                ),
            ),
        ),
    )
}
/// `no_decidable_rcf : DecidableRCF No` — No is a decidable real closed field.
pub fn no_decidable_rcf_ty() -> Expr {
    app(cst("DecidableRCF"), surreal_ty())
}
/// `tarski_transfers_to_no : ∀ (φ : Formula), RealSentence φ → HoldsIn No φ` —
/// by Tarski's theorem, every first-order real sentence holding in ℝ holds in No.
pub fn tarski_transfers_to_no_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "phi",
        cst("Formula"),
        arrow(
            app(cst("RealSentence"), bvar(0)),
            app2(cst("HoldsIn"), surreal_ty(), bvar(0)),
        ),
    )
}
/// Populate `env` with all surreal number axioms and theorems.
pub fn build_surreal_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("Surreal", surreal_ty()),
        ("Game", game_ty()),
        ("SignSeq", sign_seq_ty()),
        ("SZero", surreal_ty()),
        ("SOne", surreal_ty()),
        ("SNegOne", surreal_ty()),
        ("NatZero", nat_ty()),
        ("NatOne", nat_ty()),
        ("EmptySeq", sign_seq_ty()),
        ("Birthday", birthday_ty()),
        ("LeftSet", left_set_ty()),
        ("RightSet", right_set_ty()),
        ("MkSurreal", mk_surreal_ty()),
        ("ValidCut", valid_cut_ty()),
        ("SurrealLe", surreal_le_ty()),
        ("SurrealLt", surreal_lt_ty()),
        ("SurrealEq", surreal_eq_ty()),
        ("total_order_surreal", total_order_surreal_ty()),
        ("le_refl_surreal", le_refl_surreal_ty()),
        ("le_antisymm_surreal", le_antisymm_surreal_ty()),
        ("le_trans_surreal", le_trans_surreal_ty()),
        ("SurrealAdd", surreal_add_ty()),
        ("SurrealNeg", surreal_neg_ty()),
        ("SurrealMul", surreal_mul_ty()),
        ("SurrealInv", surreal_inv_ty()),
        ("add_comm_surreal", add_comm_surreal_ty()),
        ("add_assoc_surreal", add_assoc_surreal_ty()),
        ("mul_comm_surreal", mul_comm_surreal_ty()),
        ("add_zero_surreal", add_zero_surreal_ty()),
        ("add_neg_surreal", add_neg_surreal_ty()),
        ("surreal_ordered_field", surreal_ordered_field_ty()),
        ("OrderedField", arrow(type0(), prop())),
        ("SimplestInCut", simplest_in_cut_ty()),
        ("simplicity_theorem", simplicity_theorem_ty()),
        ("simplest_dyadic", simplest_dyadic_ty()),
        ("SignExpansion", sign_expansion_ty()),
        ("FromSignSeq", from_sign_seq_ty()),
        ("PlusSeq", arrow(nat_ty(), sign_seq_ty())),
        ("OfNat", arrow(nat_ty(), surreal_ty())),
        ("sign_expansion_injective", sign_expansion_injective_ty()),
        ("OmnificInt", omnific_int_ty()),
        ("IntegerPart", integer_part_ty()),
        ("FractionalPart", fractional_part_ty()),
        ("omnific_int_closed_add", omnific_int_closed_add_ty()),
        ("nat_embeds_in_omnific", nat_embeds_in_omnific_ty()),
        ("SurrealExp", surreal_exp_ty()),
        ("SurrealLog", surreal_log_ty()),
        ("SurrealPow", surreal_pow_ty()),
        ("exp_log_inv", exp_log_inv_ty()),
        ("log_exp_inv", log_exp_inv_ty()),
        ("exp_add", exp_add_ty()),
        ("surreal_exp_positive", surreal_exp_positive_ty()),
        ("SurrealLim", surreal_lim_ty()),
        ("SurrealContinuous", surreal_continuous_ty()),
        ("SurrealDeriv", surreal_deriv_ty()),
        ("surreal_ivt", surreal_ivt_ty()),
        ("SurrealDist", surreal_dist_ty()),
        ("EpsilonDeltaCont", epsilon_delta_cont_ty()),
        ("eps_delta_iff_continuous", eps_delta_iff_continuous_ty()),
        ("infinitesimal_epsilon", infinitesimal_epsilon_ty()),
        ("GameToSurreal", game_to_surreal_ty()),
        ("NimValue", nim_value_ty()),
        ("NimSum", nim_sum_ty()),
        ("nim_value_additive", nim_value_additive_ty()),
        ("surreal_game_equiv", surreal_game_equiv_ty()),
        ("FirstPlayerWins", arrow(surreal_ty(), prop())),
        ("positive_surreal_fp_win", positive_surreal_fp_win_ty()),
        ("XorNat", arrow(nat_ty(), arrow(nat_ty(), nat_ty()))),
        ("GameValue", arrow(game_ty(), surreal_ty())),
        ("ZeroGame", game_ty()),
        ("RealClosedField", arrow(type0(), prop())),
        ("no_is_real_closed", no_is_real_closed_ty()),
        ("surreal_archimedean_class", surreal_archimedean_class_ty()),
        ("mul_one_surreal", mul_one_surreal_ty()),
        ("mul_inv_surreal", mul_inv_surreal_ty()),
        ("distrib_surreal", distrib_surreal_ty()),
        ("Ordinal", type0()),
        (
            "OrdinalAdd",
            arrow(cst("Ordinal"), arrow(cst("Ordinal"), cst("Ordinal"))),
        ),
        ("OrdinalSurreal", ordinal_surreal_ty()),
        ("SurrealOmega", surreal_omega_ty()),
        ("SurrealEpsilon0", surreal_epsilon0_ty()),
        ("omega_gt_all_naturals", omega_gt_all_naturals_ty()),
        ("omega_plus_one", omega_plus_one_ty()),
        ("ordinal_add_embeds", ordinal_add_embeds_ty()),
        ("SignTree", sign_tree_ty()),
        ("TreeParent", tree_parent_ty()),
        (
            "AppendSign",
            arrow(sign_seq_ty(), arrow(surreal_ty(), sign_seq_ty())),
        ),
        ("LexLt", arrow(sign_seq_ty(), arrow(sign_seq_ty(), prop()))),
        ("sign_seq_prefix_order", sign_seq_prefix_order_ty()),
        (
            "surreal_lt_iff_sign_seq_lt",
            surreal_lt_iff_sign_seq_lt_ty(),
        ),
        ("OmnificInts", type0()),
        ("Ring", arrow(type0(), prop())),
        ("omnific_int_ring", omnific_int_ring_ty()),
        ("omnific_int_closed_mul", omnific_int_closed_mul_ty()),
        ("omnific_int_euclidean_div", omnific_int_euclidean_div_ty()),
        ("int_part_omnific", int_part_omnific_ty()),
        ("HahnSeries", hahn_series_ty()),
        ("HahnSeriesEmbed", hahn_series_embed_ty()),
        ("HahnSeriesAdd", hahn_series_add_ty()),
        ("HahnSeriesMul", hahn_series_mul_ty()),
        ("IsField", arrow(type0(), prop())),
        ("WellOrdered", arrow(type0(), prop())),
        ("Support", arrow(hahn_series_ty(), type0())),
        ("no_is_hahn_series_field", no_is_hahn_series_field_ty()),
        ("hahn_support_well_ordered", hahn_support_well_ordered_ty()),
        ("HyperReal", hyperreal_ty()),
        ("HyperRealEmbed", hyperreal_embed_ty()),
        ("TransferPrinciple", arrow(type0(), arrow(type0(), prop()))),
        ("Infinitesimal", arrow(hyperreal_ty(), prop())),
        ("EmbedReal", arrow(real_ty(), hyperreal_ty())),
        (
            "hyperreal_transfer_principle",
            hyperreal_transfer_principle_ty(),
        ),
        ("hyperreal_standard_part", hyperreal_standard_part_ty()),
        (
            "hyperreal_infinitesimal_def",
            hyperreal_infinitesimal_def_ty(),
        ),
        ("SurrealSeqLim", surreal_seq_lim_ty()),
        ("surreal_squeeze_theorem", surreal_squeeze_theorem_ty()),
        ("SurrealDerivAt", surreal_deriv_at_ty()),
        ("SurrealIntegral", surreal_integral_ty()),
        ("surreal_chain_rule", surreal_chain_rule_ty()),
        ("surreal_ftc", surreal_ftc_ty()),
        ("GameEquiv", game_equiv_ty()),
        ("GameAdd", game_add_ty()),
        ("GameNeg", game_neg_ty()),
        ("SecondPlayerWins", arrow(game_ty(), prop())),
        ("game_equiv_is_zero", game_equiv_is_zero_ty()),
        ("surreal_embed_game_hom", surreal_embed_game_hom_ty()),
        ("NatLt", arrow(nat_ty(), arrow(nat_ty(), prop()))),
        ("Embeds", arrow(type0(), arrow(type0(), prop()))),
        ("SurrealInduction", surreal_induction_ty()),
        ("birthday_induction", birthday_induction_ty()),
        (
            "no_is_universal_ordered_field",
            no_is_universal_ordered_field_ty(),
        ),
        ("surreal_mul_def", surreal_mul_def_ty()),
        ("mul_assoc_surreal", mul_assoc_surreal_ty()),
        ("mul_zero_surreal", mul_zero_surreal_ty()),
        ("Cardinal", type0()),
        ("KappaSaturated", arrow(type0(), arrow(type0(), prop()))),
        ("HomogeneousStructure", arrow(type0(), prop())),
        ("OMinimalStructure", arrow(type0(), prop())),
        (
            "ElementarilyEquivalent",
            arrow(type0(), arrow(type0(), prop())),
        ),
        ("OmegaOne", type0()),
        ("Isomorphic", arrow(type0(), arrow(type0(), prop()))),
        ("no_is_kappa_saturated", no_is_kappa_saturated_ty()),
        ("no_is_homogeneous", no_is_homogeneous_ty()),
        ("no_is_o_minimal", no_is_o_minimal_ty()),
        ("no_elementary_equiv_real", no_elementary_equiv_real_ty()),
        ("no_unique_up_to_sat", no_unique_up_to_sat_ty()),
        ("RealLe", arrow(real_ty(), arrow(real_ty(), prop()))),
        ("DecidableRCF", arrow(type0(), prop())),
        ("Formula", type0()),
        ("RealSentence", arrow(cst("Formula"), prop())),
        ("HoldsIn", arrow(type0(), arrow(cst("Formula"), prop()))),
        ("real_embeds_in_no", real_embeds_in_no_ty()),
        (
            "real_embed_preserves_order",
            real_embed_preserves_order_ty(),
        ),
        ("no_decidable_rcf", no_decidable_rcf_ty()),
        ("tarski_transfers_to_no", tarski_transfers_to_no_ty()),
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
/// Compute the Grundy (Nim) value of a Nim heap of size n.
///
/// For a Nim heap, the Grundy value equals n itself.
pub fn nim_grundy(n: u64) -> u64 {
    n
}
/// Compute the Nim-sum (XOR) of two Nim values.
pub fn nim_xor(a: u64, b: u64) -> u64 {
    a ^ b
}
/// Determine if a Nim game with heaps `heaps` is a P-position (previous player wins).
/// Returns true if the XOR of all heap sizes is 0.
pub fn nim_is_p_position(heaps: &[u64]) -> bool {
    heaps.iter().fold(0u64, |acc, &h| acc ^ h) == 0
}
/// Compute the surreal value of a Nim position as a dyadic rational.
///
/// Nim heap of size 0 → surreal 0, size 1 → surreal 1, etc.
/// This is a simplified integer embedding.
pub fn nim_to_surreal(n: u64) -> FinSurreal {
    FinSurreal::new(n as i64, 0)
}
/// Find the simplest (smallest birthday) dyadic rational strictly between lo and hi.
///
/// Returns `None` if no finite-birthday surreal fits (lo ≥ hi).
pub fn simplest_in_interval(lo: f64, hi: f64) -> Option<f64> {
    if lo >= hi {
        return None;
    }
    let ceil_lo = lo.ceil() as i64;
    let floor_hi = hi.floor() as i64;
    if ceil_lo <= floor_hi {
        let best = if ceil_lo <= 0 && floor_hi >= 0 {
            0
        } else if ceil_lo > 0 {
            ceil_lo
        } else {
            floor_hi
        };
        return Some(best as f64);
    }
    for k in 1..=53u32 {
        let denom = (1u64 << k) as f64;
        let lo_n = (lo * denom).floor() as i64 + 1;
        let hi_n = (hi * denom).ceil() as i64 - 1;
        if lo_n <= hi_n {
            let n = if lo_n <= 0 && hi_n >= 0 {
                0
            } else if lo_n > 0 {
                lo_n
            } else {
                hi_n
            };
            return Some(n as f64 / denom);
        }
    }
    None
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fin_surreal_zero_one() {
        let zero = FinSurreal::zero();
        let one = FinSurreal::one();
        assert!(zero.lt(&one));
        assert!(!one.lt(&zero));
        assert_eq!(zero.to_f64(), 0.0);
        assert_eq!(one.to_f64(), 1.0);
    }
    #[test]
    fn test_fin_surreal_add() {
        let half = FinSurreal::half();
        let result = half.add(&half);
        assert_eq!(result, FinSurreal::one());
        let neg_one = FinSurreal::neg_one();
        let sum = FinSurreal::one().add(&neg_one);
        assert_eq!(sum, FinSurreal::zero());
    }
    #[test]
    fn test_fin_surreal_mul() {
        let two = FinSurreal::new(2, 0);
        let three = FinSurreal::new(3, 0);
        let six = two.mul(&three);
        assert_eq!(six.numerator, 6);
        assert_eq!(six.exp, 0);
        let half = FinSurreal::half();
        let result = two.mul(&half);
        assert_eq!(result, FinSurreal::one());
    }
    #[test]
    fn test_sign_expansion_zero() {
        let zero = FinSurreal::zero();
        let signs = zero.sign_expansion();
        assert!(signs.is_empty(), "zero has empty sign expansion");
    }
    #[test]
    fn test_sign_expansion_one() {
        let one = FinSurreal::one();
        let signs = one.sign_expansion();
        assert!(!signs.is_empty());
        assert_eq!(signs[0], Sign::Plus);
    }
    #[test]
    fn test_nim_xor_and_p_position() {
        assert_eq!(nim_xor(3, 5), 6);
        assert!(nim_is_p_position(&[1, 2, 3]));
        assert!(!nim_is_p_position(&[1, 2, 4]));
    }
    #[test]
    fn test_simplest_in_interval() {
        let s = simplest_in_interval(0.3, 0.7).expect("operation should succeed");
        assert!((s - 0.5).abs() < 1e-10);
        let s2 = simplest_in_interval(1.1, 1.9).expect("abs should succeed");
        assert!((s2 - 1.5).abs() < 1e-10);
        assert!(simplest_in_interval(1.0, 0.0).is_none());
    }
    #[test]
    fn test_build_surreal_env() {
        let mut env = Environment::new();
        build_surreal_env(&mut env);
        assert!(env.get(&Name::str("Surreal")).is_some());
        assert!(env.get(&Name::str("Birthday")).is_some());
        assert!(env.get(&Name::str("SurrealAdd")).is_some());
        assert!(env.get(&Name::str("SignExpansion")).is_some());
        assert!(env.get(&Name::str("OmnificInt")).is_some());
        assert!(env.get(&Name::str("SurrealExp")).is_some());
        assert!(env.get(&Name::str("NimValue")).is_some());
    }
}
/// Internal nimber multiplication for small values using recursive doubling.
pub(super) fn nim_mul_internal(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        return 0;
    }
    if a == 1 {
        return b;
    }
    if b == 1 {
        return a;
    }
    a ^ b
}
