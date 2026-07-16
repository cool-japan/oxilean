//! # Omega Helper Lemma Bundle
//!
//! This module provides axiom-backed declarations for integer arithmetic lemmas
//! that the omega proof reconstruction compiler uses when building proof terms.
//!
//! All 20 lemmas are declared here with guards to skip duplicates if any of them
//! have already been added to the environment (e.g., `Int.le_refl` is also
//! declared by `build_int_env`).
//!
//! The 20 lemmas comprise:
//! - 10 core ordering/arithmetic lemmas
//! - 2 bridge axioms (`le_of_int_le`, `int_le_of_le`)
//! - 2 closing axioms for Farkas proof reconstruction:
//!   `Int.absurd_le_zero` and `Int.zero_lt_one`
//! - 3 strict-inequality transitivity lemmas for nlinarith Farkas (cycle 6):
//!   `Int.lt_of_le_of_lt`, `Int.lt_of_lt_of_le`, `Int.lt_trans`
//! - 1 direct Farkas contradiction closer:
//!   `Int.lt_irrefl'` (`∀ a, Int.lt a a → False`)
//! - 2 Bool-reflection contradiction closers (cycle 8):
//!   `Int.not_le_of_ble_false` and `Int.not_lt_of_blt_false`

use oxilean_kernel::Node;
use oxilean_kernel::{BinderInfo, Declaration, EnvError, Environment, Expr, Level, Literal, Name};

// ── Local expression-builder helpers ─────────────────────────────────────────

fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Node::new(f), Node::new(a))
}

fn app2(f: Expr, a: Expr, b: Expr) -> Expr {
    app(app(f, a), b)
}

fn pi(name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str(name),
        Node::new(dom),
        Node::new(body),
    )
}

fn cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}

fn bvar(n: u32) -> Expr {
    Expr::BVar(n)
}

fn prop() -> Expr {
    Expr::Sort(Level::zero())
}

fn type1() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}

fn int_const() -> Expr {
    cst("Int")
}

fn nat_const() -> Expr {
    cst("Nat")
}

/// Wraps a body in `∀ (a : Int), body`.
fn forall1_int(body: Expr) -> Expr {
    pi("a", int_const(), body)
}

/// Wraps a body in `∀ (a b : Int), body`.
fn forall2_int(body: Expr) -> Expr {
    pi("a", int_const(), pi("b", int_const(), body))
}

/// Wraps a body in `∀ (a b c : Int), body`.
fn forall3_int(body: Expr) -> Expr {
    pi(
        "a",
        int_const(),
        pi("b", int_const(), pi("c", int_const(), body)),
    )
}

/// `Eq @Int a b` (propositional equality on Int)
fn int_eq_expr(a: Expr, b: Expr) -> Expr {
    let eq_const = Expr::Const(Name::str("Eq"), vec![Level::succ(Level::zero())]);
    app(app(app(eq_const, int_const()), a), b)
}

/// `@Eq Bool lhs Bool.false` (propositional equality of a Bool expression to false)
fn bool_eq_false_expr(lhs: Expr) -> Expr {
    let eq_const = Expr::Const(Name::str("Eq"), vec![Level::succ(Level::zero())]);
    app(app(app(eq_const, cst("Bool")), lhs), cst("Bool.false"))
}

/// `Int.ofNat 0` — the integer zero.
fn int_zero() -> Expr {
    app(cst("Int.ofNat"), Expr::Lit(Literal::nat(0)))
}

/// `Int.ofNat 1` — the integer one.
fn int_one() -> Expr {
    app(cst("Int.ofNat"), Expr::Lit(Literal::nat(1)))
}

// ── Private type-builder functions ───────────────────────────────────────────

/// `Int.le_refl : ∀ (a : Int), Int.le a a`
fn ty_le_refl() -> Expr {
    forall1_int(app2(cst("Int.le"), bvar(0), bvar(0)))
}

/// `Int.le_trans : ∀ (a b c : Int), Int.le a b → Int.le b c → Int.le a c`
fn ty_le_trans() -> Expr {
    forall3_int(pi(
        "h1",
        app2(cst("Int.le"), bvar(2), bvar(1)),
        pi(
            "h2",
            app2(cst("Int.le"), bvar(2), bvar(1)),
            app2(cst("Int.le"), bvar(4), bvar(2)),
        ),
    ))
}

/// `Int.le_antisymm : ∀ (a b : Int), Int.le a b → Int.le b a → Eq a b`
fn ty_le_antisymm() -> Expr {
    forall2_int(pi(
        "h1",
        app2(cst("Int.le"), bvar(1), bvar(0)),
        pi(
            "h2",
            app2(cst("Int.le"), bvar(1), bvar(2)),
            int_eq_expr(bvar(3), bvar(2)),
        ),
    ))
}

/// `Int.lt_irrefl : ∀ (a : Int), Not (Int.lt a a)`
fn ty_lt_irrefl() -> Expr {
    forall1_int(app(cst("Not"), app2(cst("Int.lt"), bvar(0), bvar(0))))
}

/// `Int.lt_iff_add_one_le : ∀ (a b : Int), Iff (Int.lt a b) (Int.le (Int.add a 1) b)`
fn ty_lt_iff_add_one_le() -> Expr {
    forall2_int(app2(
        cst("Iff"),
        app2(cst("Int.lt"), bvar(1), bvar(0)),
        app2(
            cst("Int.le"),
            app2(cst("Int.add"), bvar(1), int_one()),
            bvar(0),
        ),
    ))
}

/// `Int.le_of_lt : ∀ (a b : Int), Int.lt a b → Int.le a b`
fn ty_le_of_lt() -> Expr {
    forall2_int(pi(
        "h",
        app2(cst("Int.lt"), bvar(1), bvar(0)),
        app2(cst("Int.le"), bvar(2), bvar(1)),
    ))
}

/// `Int.add_le_add : ∀ (a b c d : Int), Int.le a b → Int.le c d → Int.le (Int.add a c) (Int.add b d)`
fn ty_add_le_add() -> Expr {
    // Bind a b c d in order — each pi shifts outer bvars by 1.
    // After all 4 binders: a=bvar(3), b=bvar(2), c=bvar(1), d=bvar(0).
    pi(
        "a",
        int_const(),
        pi(
            "b",
            int_const(),
            pi(
                "c",
                int_const(),
                pi(
                    "d",
                    int_const(),
                    pi(
                        "h1",
                        // h1 : Int.le a b  →  a=bvar(3), b=bvar(2)
                        app2(cst("Int.le"), bvar(3), bvar(2)),
                        pi(
                            "h2",
                            // h2 : Int.le c d  →  c=bvar(2), d=bvar(1)  (shifted by the h1 binder)
                            app2(cst("Int.le"), bvar(2), bvar(1)),
                            // conclusion: Int.le (a+c) (b+d)  →  a=bvar(5), b=bvar(4), c=bvar(3), d=bvar(2)
                            app2(
                                cst("Int.le"),
                                app2(cst("Int.add"), bvar(5), bvar(3)),
                                app2(cst("Int.add"), bvar(4), bvar(2)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}

/// `Int.mul_le_mul_of_nonneg_left : ∀ (a b c : Int), Int.le a b → Int.le 0 c → Int.le (Int.mul c a) (Int.mul c b)`
fn ty_mul_le_mul_of_nonneg_left() -> Expr {
    // After binding a b c: a=bvar(2), b=bvar(1), c=bvar(0)
    forall3_int(pi(
        "h1",
        // h1 : a ≤ b  →  a=bvar(2), b=bvar(1)
        app2(cst("Int.le"), bvar(2), bvar(1)),
        pi(
            "h2",
            // h2 : 0 ≤ c  →  c=bvar(1)  (shifted by h1 binder)
            app2(cst("Int.le"), int_zero(), bvar(1)),
            // conclusion: c*a ≤ c*b  →  a=bvar(4), b=bvar(3), c=bvar(2)
            app2(
                cst("Int.le"),
                app2(cst("Int.mul"), bvar(2), bvar(4)),
                app2(cst("Int.mul"), bvar(2), bvar(3)),
            ),
        ),
    ))
}

/// `Int.le_of_eq : ∀ (a b : Int), Eq a b → Int.le a b`
fn ty_le_of_eq() -> Expr {
    // After binding a b: a=bvar(1), b=bvar(0)
    forall2_int(pi(
        "h",
        int_eq_expr(bvar(1), bvar(0)),
        // conclusion: a ≤ b  →  a=bvar(2), b=bvar(1)
        app2(cst("Int.le"), bvar(2), bvar(1)),
    ))
}

/// `Int.le_total : ∀ (a b : Int), Or (Int.le a b) (Int.le b a)`
fn ty_le_total() -> Expr {
    forall2_int(app2(
        cst("Or"),
        app2(cst("Int.le"), bvar(1), bvar(0)),
        app2(cst("Int.le"), bvar(0), bvar(1)),
    ))
}

/// `le_of_int_le : ∀ {inst : LE Int} (a b : Int), Int.le a b → LE.le Int inst a b`
///
/// Bridge axiom: converts an `Int.le` proof into a `LE.le` proof for a given instance.
///
/// Binder stack trace (innermost first):
/// - After `inst` (Implicit): [inst] → inst = bvar(0)
/// - After `a` (Default): [a, inst] → a = bvar(0), inst = bvar(1)
/// - After `b` (Default): [b, a, inst] → b = bvar(0), a = bvar(1), inst = bvar(2)
/// - h-domain (in [b,a,inst]): `Int.le a b` → a=bvar(1), b=bvar(0)
/// - After `h` (Default): [h, b, a, inst] → h=bvar(0), b=bvar(1), a=bvar(2), inst=bvar(3)
/// - conclusion: `LE.le Int inst a b` → inst=bvar(3), a=bvar(2), b=bvar(1)
fn ty_le_of_int_le() -> Expr {
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("inst"),
        Node::new(app(cst("LE"), int_const())),
        Node::new(pi(
            "a",
            int_const(),
            pi(
                "b",
                int_const(),
                pi(
                    "h",
                    app2(cst("Int.le"), bvar(1), bvar(0)),
                    // conclusion: LE.le Int inst a b
                    // after h binder: inst=bvar(3), a=bvar(2), b=bvar(1)
                    app(
                        app(app(app(cst("LE.le"), int_const()), bvar(3)), bvar(2)),
                        bvar(1),
                    ),
                ),
            ),
        )),
    )
}

/// `int_le_of_le : ∀ {inst : LE Int} (a b : Int), LE.le Int inst a b → Int.le a b`
///
/// Bridge axiom: converts a `LE.le` proof into an `Int.le` proof.
///
/// Binder stack trace (innermost first):
/// - After `inst` (Implicit): [inst] → inst = bvar(0)
/// - After `a` (Default): [a, inst] → a = bvar(0), inst = bvar(1)
/// - After `b` (Default): [b, a, inst] → b = bvar(0), a = bvar(1), inst = bvar(2)
/// - h-domain (in [b,a,inst]): `LE.le Int inst a b` → inst=bvar(2), a=bvar(1), b=bvar(0)
/// - After `h` (Default): [h, b, a, inst] → h=bvar(0), b=bvar(1), a=bvar(2), inst=bvar(3)
/// - conclusion: `Int.le a b` → a=bvar(2), b=bvar(1)
fn ty_int_le_of_le() -> Expr {
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("inst"),
        Node::new(app(cst("LE"), int_const())),
        Node::new(pi(
            "a",
            int_const(),
            pi(
                "b",
                int_const(),
                pi(
                    "h",
                    // LE.le Int inst a b — in [b, a, inst]: inst=bvar(2), a=bvar(1), b=bvar(0)
                    app(
                        app(app(app(cst("LE.le"), int_const()), bvar(2)), bvar(1)),
                        bvar(0),
                    ),
                    // conclusion: Int.le a b — after h binder: a=bvar(2), b=bvar(1)
                    app2(cst("Int.le"), bvar(2), bvar(1)),
                ),
            ),
        )),
    )
}

/// `Int.absurd_le_zero : ∀ (k : Int), Int.lt 0 k → Int.le k 0 → False`
///
/// Contradiction closer: given a proof that `0 < k` and a proof that `k ≤ 0`,
/// derives `False`. Used by Farkas proof reconstruction to close the sum
/// after scaling and adding linear constraints.
///
/// Binder stack trace:
/// - After `k` (Default): [k] → k = bvar(0)
/// - h1-domain (in [k]): `Int.lt 0 k` → 0=int_zero(), k=bvar(0)
/// - After `h1` (Default): [h1, k] → h1=bvar(0), k=bvar(1)
/// - h2-domain (in [h1, k]): `Int.le k 0` → k=bvar(1), 0=int_zero()
/// - After `h2` (Default): [h2, h1, k] → h2=bvar(0), h1=bvar(1), k=bvar(2)
/// - conclusion: `False`
fn ty_absurd_le_zero() -> Expr {
    pi(
        "k",
        int_const(),
        pi(
            "h1",
            // Int.lt 0 k — in [k]: 0=int_zero(), k=bvar(0)
            app2(cst("Int.lt"), int_zero(), bvar(0)),
            pi(
                "h2",
                // Int.le k 0 — in [h1, k]: k=bvar(1), 0=int_zero()
                app2(cst("Int.le"), bvar(1), int_zero()),
                // conclusion: False
                cst("False"),
            ),
        ),
    )
}

/// `Int.zero_lt_one : Int.lt 0 1`
///
/// Ground positivity axiom: `0 < 1` for integers.
/// Represented as `Int.lt (Int.ofNat 0) (Int.ofNat 1)`.
/// Used by Farkas proof reconstruction to supply the positivity side-condition
/// needed when invoking `Int.mul_le_mul_of_nonneg_left` with multiplier 1,
/// or when closing a constant contradiction at `k = 1`.
fn ty_zero_lt_one() -> Expr {
    app2(cst("Int.lt"), int_zero(), int_one())
}

/// `Int.lt_of_le_of_lt : ∀ (a b c : Int), Int.le a b → Int.lt b c → Int.lt a c`
///
/// Strict-inequality transitivity: combines a non-strict lower bound with a
/// strict upper bound to yield a strict bound on the original term.
///
/// De Bruijn trace (same shape as `ty_le_trans` — only h2 domain and body change):
/// - After forall3_int: a=bvar(2), b=bvar(1), c=bvar(0)
/// - h1 domain: `Int.le a b` → bvar(2)=a, bvar(1)=b
/// - After h1: a=bvar(3), b=bvar(2), c=bvar(1)
/// - h2 domain: `Int.lt b c` → bvar(2)=b, bvar(1)=c
/// - After h2: a=bvar(4), b=bvar(3), c=bvar(2)
/// - body: `Int.lt a c` → bvar(4)=a, bvar(2)=c
fn ty_lt_of_le_of_lt() -> Expr {
    forall3_int(pi(
        "h1",
        app2(cst("Int.le"), bvar(2), bvar(1)),
        pi(
            "h2",
            app2(cst("Int.lt"), bvar(2), bvar(1)),
            app2(cst("Int.lt"), bvar(4), bvar(2)),
        ),
    ))
}

/// `Int.lt_of_lt_of_le : ∀ (a b c : Int), Int.lt a b → Int.le b c → Int.lt a c`
///
/// Strict-inequality transitivity: combines a strict lower bound with a
/// non-strict upper bound to yield a strict bound on the original term.
///
/// De Bruijn trace:
/// - After forall3_int: a=bvar(2), b=bvar(1), c=bvar(0)
/// - h1 domain: `Int.lt a b` → bvar(2)=a, bvar(1)=b
/// - After h1: a=bvar(3), b=bvar(2), c=bvar(1)
/// - h2 domain: `Int.le b c` → bvar(2)=b, bvar(1)=c
/// - After h2: a=bvar(4), b=bvar(3), c=bvar(2)
/// - body: `Int.lt a c` → bvar(4)=a, bvar(2)=c
fn ty_lt_of_lt_of_le() -> Expr {
    forall3_int(pi(
        "h1",
        app2(cst("Int.lt"), bvar(2), bvar(1)),
        pi(
            "h2",
            app2(cst("Int.le"), bvar(2), bvar(1)),
            app2(cst("Int.lt"), bvar(4), bvar(2)),
        ),
    ))
}

/// `Int.lt_trans : ∀ (a b c : Int), Int.lt a b → Int.lt b c → Int.lt a c`
///
/// Full strict-inequality transitivity.
///
/// De Bruijn trace:
/// - After forall3_int: a=bvar(2), b=bvar(1), c=bvar(0)
/// - h1 domain: `Int.lt a b` → bvar(2)=a, bvar(1)=b
/// - After h1: a=bvar(3), b=bvar(2), c=bvar(1)
/// - h2 domain: `Int.lt b c` → bvar(2)=b, bvar(1)=c
/// - After h2: a=bvar(4), b=bvar(3), c=bvar(2)
/// - body: `Int.lt a c` → bvar(4)=a, bvar(2)=c
fn ty_lt_trans() -> Expr {
    forall3_int(pi(
        "h1",
        app2(cst("Int.lt"), bvar(2), bvar(1)),
        pi(
            "h2",
            app2(cst("Int.lt"), bvar(2), bvar(1)),
            app2(cst("Int.lt"), bvar(4), bvar(2)),
        ),
    ))
}

/// `Int.lt_irrefl' : ∀ (a : Int), Int.lt a a → False`
///
/// Direct Farkas contradiction closer: given `h : Int.lt a a`, produces `False`.
/// Unlike `Int.lt_irrefl` (which has type `∀ a, Not (Int.lt a a)` where `Not` may
/// be opaque), this variant is the direct function form that the Farkas proof
/// builder can apply to a proof of `Int.lt a a` without reducing through `Not`.
///
/// De Bruijn trace:
/// - After `a` binder: a=bvar(0)
/// - h domain: `Int.lt a a` → bvar(0)=a, bvar(0)=a
/// - After h: a=bvar(1), h=bvar(0)
/// - body: `False`
fn ty_lt_irrefl_false() -> Expr {
    forall1_int(pi("h", app2(cst("Int.lt"), bvar(0), bvar(0)), cst("False")))
}

/// `Int.not_le_of_ble_false : ∀ (a b : Int), @Eq Bool (Int.ble a b) Bool.false → Int.le a b → False`
///
/// Bool-reflection contradiction closer (cycle 8): given a proof that
/// `Int.ble a b = Bool.false` and a proof that `Int.le a b`, derives `False`.
/// Used by the arithmetic chain close strategy in `farkas.rs` to close chains
/// `a ≤ ... ≤ b` where `a > b` as ground integer literals (since `Int.ble a b`
/// reduces to `Bool.false` under `whnf` when `a > b`).
///
/// De Bruijn trace:
/// - `forall2_int` binds: after `a` binder: a=bvar(0); after `b` binder: b=bvar(0), a=bvar(1)
/// - h1 Pi (in [b,a]): domain = `@Eq Bool (Int.ble a b) Bool.false`
///   → a=bvar(1), b=bvar(0)
/// - After h1 (in [h1,b,a]): h1=bvar(0), b=bvar(1), a=bvar(2)
/// - h2 Pi (in [h1,b,a]): domain = `Int.le a b` → a=bvar(2), b=bvar(1)
/// - body: `False`
fn ty_not_le_of_ble_false() -> Expr {
    forall2_int(pi(
        "h1",
        bool_eq_false_expr(app2(cst("Int.ble"), bvar(1), bvar(0))),
        pi("h2", app2(cst("Int.le"), bvar(2), bvar(1)), cst("False")),
    ))
}

/// `Int.not_lt_of_blt_false : ∀ (a b : Int), @Eq Bool (Int.blt a b) Bool.false → Int.lt a b → False`
///
/// Bool-reflection contradiction closer (cycle 8): given a proof that
/// `Int.blt a b = Bool.false` and a proof that `Int.lt a b`, derives `False`.
/// Used by the arithmetic chain close strategy in `farkas.rs` to close chains
/// `a < ... < b` where `a ≥ b` as ground integer literals (since `Int.blt a b`
/// reduces to `Bool.false` under `whnf` when `a >= b`).
///
/// De Bruijn trace (same shape as `ty_not_le_of_ble_false` but with `blt`/`lt`):
/// - `forall2_int` binds: after `a` binder: a=bvar(0); after `b` binder: b=bvar(0), a=bvar(1)
/// - h1 Pi (in [b,a]): domain = `@Eq Bool (Int.blt a b) Bool.false`
///   → a=bvar(1), b=bvar(0)
/// - After h1 (in [h1,b,a]): h1=bvar(0), b=bvar(1), a=bvar(2)
/// - h2 Pi (in [h1,b,a]): domain = `Int.lt a b` → a=bvar(2), b=bvar(1)
/// - body: `False`
fn ty_not_lt_of_blt_false() -> Expr {
    forall2_int(pi(
        "h1",
        bool_eq_false_expr(app2(cst("Int.blt"), bvar(1), bvar(0))),
        pi("h2", app2(cst("Int.lt"), bvar(2), bvar(1)), cst("False")),
    ))
}

// ── Public registration function ─────────────────────────────────────────────

/// Register the omega helper lemma bundle into `env`.
///
/// Adds axiom-backed declarations for the 20 integer arithmetic lemmas that the
/// omega proof reconstruction compiler uses. Each lemma is guarded: if already
/// present in `env` (e.g., because `build_int_env` was called first), it is
/// silently skipped. This makes the function idempotent.
///
/// The bundle includes:
/// - 10 core ordering/arithmetic lemmas (`Int.le_refl`, `Int.le_trans`, etc.)
/// - 2 bridge axioms (`le_of_int_le`, `int_le_of_le`) for `LE.le ↔ Int.le`
/// - 2 closing axioms for Farkas proof reconstruction:
///   - `Int.absurd_le_zero`: `∀ k, 0 < k → k ≤ 0 → False`
///   - `Int.zero_lt_one`: `0 < 1` (ground positivity fact for multiplier 1)
/// - 3 strict-inequality transitivity lemmas for nlinarith Farkas (cycle 6):
///   - `Int.lt_of_le_of_lt`: `∀ a b c, a ≤ b → b < c → a < c`
///   - `Int.lt_of_lt_of_le`: `∀ a b c, a < b → b ≤ c → a < c`
///   - `Int.lt_trans`: `∀ a b c, a < b → b < c → a < c`
/// - 1 direct Farkas contradiction closer:
///   - `Int.lt_irrefl'`: `∀ a, Int.lt a a → False`
/// - 2 Bool-reflection contradiction closers (cycle 8):
///   - `Int.not_le_of_ble_false`: `∀ a b, @Eq Bool (Int.ble a b) Bool.false → Int.le a b → False`
///   - `Int.not_lt_of_blt_false`: `∀ a b, @Eq Bool (Int.blt a b) Bool.false → Int.lt a b → False`
///
/// # Errors
///
/// Returns `EnvError::DuplicateDeclaration` if an unexpected name collision
/// occurs (should not happen in normal usage due to the `contains` guards).
pub fn register_omega_helper(env: &mut Environment) -> Result<(), EnvError> {
    let lemmas: &[(&str, Vec<Name>, Expr)] = &[
        ("Int.le_refl", vec![], ty_le_refl()),
        ("Int.le_trans", vec![], ty_le_trans()),
        ("Int.le_antisymm", vec![], ty_le_antisymm()),
        ("Int.lt_irrefl", vec![], ty_lt_irrefl()),
        ("Int.lt_iff_add_one_le", vec![], ty_lt_iff_add_one_le()),
        ("Int.le_of_lt", vec![], ty_le_of_lt()),
        ("Int.add_le_add", vec![], ty_add_le_add()),
        (
            "Int.mul_le_mul_of_nonneg_left",
            vec![],
            ty_mul_le_mul_of_nonneg_left(),
        ),
        ("Int.le_of_eq", vec![], ty_le_of_eq()),
        ("Int.le_total", vec![], ty_le_total()),
        ("le_of_int_le", vec![], ty_le_of_int_le()),
        ("int_le_of_le", vec![], ty_int_le_of_le()),
        ("Int.absurd_le_zero", vec![], ty_absurd_le_zero()),
        ("Int.zero_lt_one", vec![], ty_zero_lt_one()),
        ("Int.lt_of_le_of_lt", vec![], ty_lt_of_le_of_lt()),
        ("Int.lt_of_lt_of_le", vec![], ty_lt_of_lt_of_le()),
        ("Int.lt_trans", vec![], ty_lt_trans()),
        ("Int.lt_irrefl'", vec![], ty_lt_irrefl_false()),
        ("Int.not_le_of_ble_false", vec![], ty_not_le_of_ble_false()),
        ("Int.not_lt_of_blt_false", vec![], ty_not_lt_of_blt_false()),
    ];

    for (name, univ_params, ty) in lemmas {
        let n = Name::str(*name);
        if env.contains(&n) {
            continue;
        }
        env.add(Declaration::Axiom {
            name: n,
            univ_params: univ_params.clone(),
            ty: ty.clone(),
        })?;
    }
    Ok(())
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    const OMEGA_LEMMAS: &[&str] = &[
        "Int.le_refl",
        "Int.le_trans",
        "Int.le_antisymm",
        "Int.lt_irrefl",
        "Int.lt_iff_add_one_le",
        "Int.le_of_lt",
        "Int.add_le_add",
        "Int.mul_le_mul_of_nonneg_left",
        "Int.le_of_eq",
        "Int.le_total",
        "le_of_int_le",
        "int_le_of_le",
        "Int.absurd_le_zero",
        "Int.zero_lt_one",
        "Int.lt_of_le_of_lt",
        "Int.lt_of_lt_of_le",
        "Int.lt_trans",
        "Int.lt_irrefl'",
        "Int.not_le_of_ble_false",
        "Int.not_lt_of_blt_false",
    ];

    #[test]
    fn test_register_omega_helper_success() {
        let mut env = Environment::new();
        register_omega_helper(&mut env).expect("registration should succeed");
    }

    #[test]
    fn test_all_lemmas_present() {
        let mut env = Environment::new();
        register_omega_helper(&mut env).expect("registration should succeed");

        for name in OMEGA_LEMMAS {
            assert!(
                env.contains(&Name::str(*name)),
                "lemma {} should be registered",
                name
            );
        }
    }

    #[test]
    fn test_duplicate_registration_is_idempotent() {
        let mut env = Environment::new();
        register_omega_helper(&mut env).expect("first registration");
        // Second call should skip all due to contains() guards.
        register_omega_helper(&mut env).expect("second registration should succeed");
        // All lemmas still present exactly once.
        for name in OMEGA_LEMMAS {
            assert!(
                env.contains(&Name::str(*name)),
                "lemma {} should still be registered after second call",
                name
            );
        }
    }

    #[test]
    fn test_coexistence_with_preregistered_lemmas() {
        let mut env = Environment::new();
        // Simulate what build_int_env does: pre-register the three that overlap.
        let pre_existing = &[
            ("Int.le_refl", ty_le_refl()),
            ("Int.le_trans", ty_le_trans()),
            ("Int.le_antisymm", ty_le_antisymm()),
        ];
        for (name, ty) in pre_existing {
            env.add(Declaration::Axiom {
                name: Name::str(*name),
                univ_params: vec![],
                ty: ty.clone(),
            })
            .expect("pre-registration should succeed");
        }
        // Now register_omega_helper should skip those and add the remaining 7.
        register_omega_helper(&mut env).expect("registration with pre-existing should succeed");
        for name in OMEGA_LEMMAS {
            assert!(
                env.contains(&Name::str(*name)),
                "lemma {} should be present after coexistence test",
                name
            );
        }
    }

    #[test]
    fn test_le_refl_type_structure() {
        let ty = ty_le_refl();
        assert!(
            matches!(ty, Expr::Pi(_, _, _, _)),
            "le_refl type should be a Pi"
        );
    }

    #[test]
    fn test_lt_irrefl_type_structure() {
        let ty = ty_lt_irrefl();
        assert!(
            matches!(ty, Expr::Pi(_, _, _, _)),
            "lt_irrefl type should be a Pi"
        );
    }

    #[test]
    fn test_add_le_add_type_structure() {
        let ty = ty_add_le_add();
        // Should be a deeply nested Pi (4 variables + 2 hypothesis binders).
        assert!(
            matches!(ty, Expr::Pi(_, _, _, _)),
            "add_le_add type should be a Pi"
        );
    }

    #[test]
    fn test_le_total_type_structure() {
        let ty = ty_le_total();
        assert!(
            matches!(ty, Expr::Pi(_, _, _, _)),
            "le_total type should be a Pi"
        );
    }

    #[test]
    fn test_env_contains_after_registration() {
        let mut env = Environment::new();
        register_omega_helper(&mut env).expect("registration should succeed");
        assert!(env.find(&Name::str("Int.le_refl")).is_some());
        assert!(env.find(&Name::str("Int.le_total")).is_some());
        assert!(env.find(&Name::str("Int.add_le_add")).is_some());
        assert!(env
            .find(&Name::str("Int.mul_le_mul_of_nonneg_left"))
            .is_some());
    }

    #[test]
    fn test_lemma_count_in_fresh_env() {
        let mut env = Environment::new();
        register_omega_helper(&mut env).expect("registration should succeed");
        // After registration, the env should contain exactly the 14 lemmas.
        assert_eq!(env.len(), OMEGA_LEMMAS.len());
    }

    #[test]
    fn test_bridge_axioms_present() {
        let mut env = Environment::new();
        register_omega_helper(&mut env).expect("registration should succeed");
        assert!(env.contains(&Name::str("le_of_int_le")));
        assert!(env.contains(&Name::str("int_le_of_le")));
    }

    #[test]
    fn test_le_of_int_le_type_structure() {
        let ty = ty_le_of_int_le();
        assert!(
            matches!(ty, Expr::Pi(BinderInfo::Implicit, _, _, _)),
            "le_of_int_le should start with an implicit Pi (inst binder)"
        );
    }

    #[test]
    fn test_int_le_of_le_type_structure() {
        let ty = ty_int_le_of_le();
        assert!(
            matches!(ty, Expr::Pi(BinderInfo::Implicit, _, _, _)),
            "int_le_of_le should start with an implicit Pi (inst binder)"
        );
    }

    #[test]
    fn test_bridge_axioms_idempotent() {
        let mut env = Environment::new();
        register_omega_helper(&mut env).expect("first registration");
        // Second call should skip all due to contains() guards.
        register_omega_helper(&mut env).expect("second registration should succeed");
        assert!(env.contains(&Name::str("le_of_int_le")));
        assert!(env.contains(&Name::str("int_le_of_le")));
    }

    #[test]
    fn test_absurd_le_zero_registered() {
        let mut env = Environment::new();
        register_omega_helper(&mut env).expect("registration should succeed");
        assert!(
            env.contains(&Name::str("Int.absurd_le_zero")),
            "Int.absurd_le_zero should be registered"
        );
    }

    #[test]
    fn test_zero_lt_one_registered() {
        let mut env = Environment::new();
        register_omega_helper(&mut env).expect("registration should succeed");
        assert!(
            env.contains(&Name::str("Int.zero_lt_one")),
            "Int.zero_lt_one should be registered"
        );
    }

    #[test]
    fn test_absurd_le_zero_type_is_pi() {
        let ty = ty_absurd_le_zero();
        // Should be ∀ (k : Int), ... — outermost binder is a Pi over Int.
        assert!(
            matches!(ty, Expr::Pi(BinderInfo::Default, _, _, _)),
            "absurd_le_zero type should start with a default Pi binder (k : Int)"
        );
        // Unwrap to verify it's at least 3 levels deep (k, h1, h2).
        if let Expr::Pi(_, _, _, body1) = ty {
            assert!(
                matches!(*body1, Expr::Pi(BinderInfo::Default, _, _, _)),
                "second binder (h1) should be a default Pi"
            );
            if let Expr::Pi(_, _, _, body2) = (*body1).clone() {
                assert!(
                    matches!(*body2, Expr::Pi(BinderInfo::Default, _, _, _)),
                    "third binder (h2) should be a default Pi"
                );
                if let Expr::Pi(_, _, _, concl) = (*body2).clone() {
                    // Conclusion should be False (a Const).
                    assert!(
                        matches!(*concl, Expr::Const(_, _)),
                        "conclusion of absurd_le_zero should be Const(\"False\")"
                    );
                }
            }
        }
    }

    #[test]
    fn test_zero_lt_one_type_is_ground_app() {
        let ty = ty_zero_lt_one();
        // Should be a fully-applied App (no Pi binders) — ground proposition.
        assert!(
            matches!(ty, Expr::App(_, _)),
            "zero_lt_one type should be a ground App (Int.lt 0 1), no binders"
        );
    }

    #[test]
    fn test_closing_axioms_idempotent() {
        let mut env = Environment::new();
        register_omega_helper(&mut env).expect("first registration");
        register_omega_helper(&mut env).expect("second registration should succeed (idempotent)");
        assert!(env.contains(&Name::str("Int.absurd_le_zero")));
        assert!(env.contains(&Name::str("Int.zero_lt_one")));
        // Total lemma count should still be 20 (no duplicates).
        assert_eq!(env.len(), OMEGA_LEMMAS.len());
    }

    #[test]
    fn test_strict_transitivity_axioms_present() {
        let mut env = Environment::new();
        register_omega_helper(&mut env).expect("registration should succeed");
        assert!(
            env.contains(&Name::str("Int.lt_of_le_of_lt")),
            "Int.lt_of_le_of_lt should be registered"
        );
        assert!(
            env.contains(&Name::str("Int.lt_of_lt_of_le")),
            "Int.lt_of_lt_of_le should be registered"
        );
        assert!(
            env.contains(&Name::str("Int.lt_trans")),
            "Int.lt_trans should be registered"
        );
    }

    #[test]
    fn test_lt_irrefl_false_registered() {
        let mut env = Environment::new();
        register_omega_helper(&mut env).expect("registration should succeed");
        assert!(
            env.contains(&Name::str("Int.lt_irrefl'")),
            "Int.lt_irrefl' should be registered"
        );
    }

    #[test]
    fn test_lt_of_le_of_lt_type_structure() {
        let ty = ty_lt_of_le_of_lt();
        // Should be ∀ (a : Int), ... — outermost binder is a Pi over Int.
        assert!(
            matches!(ty, Expr::Pi(BinderInfo::Default, _, _, _)),
            "lt_of_le_of_lt type should start with a default Pi binder"
        );
    }

    #[test]
    fn test_lt_of_lt_of_le_type_structure() {
        let ty = ty_lt_of_lt_of_le();
        assert!(
            matches!(ty, Expr::Pi(BinderInfo::Default, _, _, _)),
            "lt_of_lt_of_le type should start with a default Pi binder"
        );
    }

    #[test]
    fn test_lt_trans_type_structure() {
        let ty = ty_lt_trans();
        assert!(
            matches!(ty, Expr::Pi(BinderInfo::Default, _, _, _)),
            "lt_trans type should start with a default Pi binder"
        );
    }

    #[test]
    fn test_lt_irrefl_false_type_structure() {
        let ty = ty_lt_irrefl_false();
        // Should be ∀ (a : Int), Int.lt a a → False — two binders.
        assert!(
            matches!(ty, Expr::Pi(BinderInfo::Default, _, _, _)),
            "lt_irrefl' type should start with a default Pi binder (a : Int)"
        );
        if let Expr::Pi(_, _, _, body) = ty {
            assert!(
                matches!(*body, Expr::Pi(BinderInfo::Default, _, _, _)),
                "second binder (h) should be a default Pi"
            );
            if let Expr::Pi(_, _, _, concl) = (*body).clone() {
                assert!(
                    matches!(*concl, Expr::Const(_, _)),
                    "conclusion of lt_irrefl' should be Const(\"False\")"
                );
            }
        }
    }

    #[test]
    fn test_lemma_count_is_20() {
        let mut env = Environment::new();
        register_omega_helper(&mut env).expect("registration should succeed");
        // After registration, the env should contain exactly the 20 lemmas.
        assert_eq!(env.len(), 20, "expected 20 omega lemmas");
        assert_eq!(
            OMEGA_LEMMAS.len(),
            20,
            "OMEGA_LEMMAS slice should list 20 entries"
        );
    }

    #[test]
    fn test_strict_transitivity_axioms_idempotent() {
        let mut env = Environment::new();
        register_omega_helper(&mut env).expect("first registration");
        register_omega_helper(&mut env).expect("second registration should succeed");
        assert!(env.contains(&Name::str("Int.lt_of_le_of_lt")));
        assert!(env.contains(&Name::str("Int.lt_of_lt_of_le")));
        assert!(env.contains(&Name::str("Int.lt_trans")));
        assert!(env.contains(&Name::str("Int.lt_irrefl'")));
        // Total should still be 20.
        assert_eq!(env.len(), 20);
    }

    #[test]
    fn test_bool_reflection_axioms_present() {
        let mut env = Environment::new();
        register_omega_helper(&mut env).expect("registration should succeed");
        assert!(
            env.contains(&Name::str("Int.not_le_of_ble_false")),
            "Int.not_le_of_ble_false should be registered"
        );
        assert!(
            env.contains(&Name::str("Int.not_lt_of_blt_false")),
            "Int.not_lt_of_blt_false should be registered"
        );
    }

    #[test]
    fn test_not_le_of_ble_false_type_structure() {
        let ty = ty_not_le_of_ble_false();
        // Should be ∀ (a : Int), ... — outermost binder is a Pi over Int.
        assert!(
            matches!(ty, Expr::Pi(BinderInfo::Default, _, _, _)),
            "not_le_of_ble_false type should start with a default Pi binder (a : Int)"
        );
        // Unwrap to verify: ∀ a b, h1 → h2 → False (4 binders total).
        if let Expr::Pi(_, _, _, body1) = ty {
            assert!(
                matches!(*body1, Expr::Pi(BinderInfo::Default, _, _, _)),
                "second binder (b) should be a default Pi"
            );
            if let Expr::Pi(_, _, _, body2) = (*body1).clone() {
                assert!(
                    matches!(*body2, Expr::Pi(BinderInfo::Default, _, _, _)),
                    "third binder (h1: Eq Bool ...) should be a default Pi"
                );
                if let Expr::Pi(_, _, _, body3) = (*body2).clone() {
                    assert!(
                        matches!(*body3, Expr::Pi(BinderInfo::Default, _, _, _)),
                        "fourth binder (h2: Int.le ...) should be a default Pi"
                    );
                    if let Expr::Pi(_, _, _, concl) = (*body3).clone() {
                        assert!(
                            matches!(*concl, Expr::Const(_, _)),
                            "conclusion should be Const(\"False\")"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn test_not_lt_of_blt_false_type_structure() {
        let ty = ty_not_lt_of_blt_false();
        // Same shape as not_le_of_ble_false: ∀ a b h1 h2, False (4 binders).
        assert!(
            matches!(ty, Expr::Pi(BinderInfo::Default, _, _, _)),
            "not_lt_of_blt_false type should start with a default Pi binder (a : Int)"
        );
        if let Expr::Pi(_, _, _, body1) = ty {
            if let Expr::Pi(_, _, _, body2) = (*body1).clone() {
                if let Expr::Pi(_, _, _, body3) = (*body2).clone() {
                    if let Expr::Pi(_, _, _, concl) = (*body3).clone() {
                        assert!(
                            matches!(*concl, Expr::Const(_, _)),
                            "conclusion of not_lt_of_blt_false should be Const(\"False\")"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn test_bool_reflection_axioms_idempotent() {
        let mut env = Environment::new();
        register_omega_helper(&mut env).expect("first registration");
        register_omega_helper(&mut env).expect("second registration should succeed");
        assert!(env.contains(&Name::str("Int.not_le_of_ble_false")));
        assert!(env.contains(&Name::str("Int.not_lt_of_blt_false")));
        // Total should still be 20.
        assert_eq!(env.len(), 20);
    }
}
