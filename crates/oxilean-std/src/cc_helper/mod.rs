//! # CC (Congruence Closure) Helper Lemma Bundle
//!
//! This module provides axiom-backed declarations for equality and congruence lemmas
//! that the congruence-closure proof reconstruction compiler uses when building proof terms.
//!
//! The bundle registers core equality lemmas (which may already exist in the environment
//! from `build_eq_env`) plus cc-specific additional ones, all guarded against duplicates
//! (idempotent via `env.contains` guards). This allows a single `register_cc_helper(env)`
//! call to make all congruence-closure axioms available in any environment.
//!
//! The 9 lemmas cover:
//! - `Eq.symm`, `Eq.trans`, `congrArg` (may already be in env from build_eq_env)
//! - `Ne.symm` (new — negated-equality symmetry)
//! - `eq_of_heq` (bridge from heterogeneous to homogeneous equality)
//! - `Eq.subst` (rewriting via equality)
//! - `congr` (congruence for both function and argument simultaneously)
//! - `eq_true_intro`, `eq_false_intro` (Prop-level Bool-reflection)

use oxilean_kernel::Node;
use oxilean_kernel::{BinderInfo, Declaration, EnvError, Environment, Expr, Level, Name};

// ── Local expression-builder helpers ─────────────────────────────────────────

fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Node::new(f), Node::new(a))
}

fn app2(f: Expr, a: Expr, b: Expr) -> Expr {
    app(app(f, a), b)
}

fn app3(f: Expr, a: Expr, b: Expr, c: Expr) -> Expr {
    app(app2(f, a, b), c)
}

fn pi_default(name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str(name),
        Node::new(dom),
        Node::new(body),
    )
}

fn pi_implicit(name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Implicit,
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

/// `Prop` = `Sort 0`
fn prop() -> Expr {
    Expr::Sort(Level::zero())
}

/// `Type` = `Sort 1`
fn sort1() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}

/// `@Eq` at universe level 1 (for `α : Type`).
fn eq_u1() -> Expr {
    Expr::Const(Name::str("Eq"), vec![Level::succ(Level::zero())])
}

/// `@Eq α a b` where α lives in `Sort 1`.
/// Usage: pass the α, a, b expressions directly.
fn eq_app(alpha: Expr, a: Expr, b: Expr) -> Expr {
    app3(eq_u1(), alpha, a, b)
}

/// `@HEq` at universe level 1 (for heterogeneous equality between values of same type).
fn heq_u1() -> Expr {
    Expr::Const(Name::str("HEq"), vec![Level::succ(Level::zero())])
}

/// `@Eq` at universe level 0 (for `p : Prop`).
fn eq_prop() -> Expr {
    Expr::Const(Name::str("Eq"), vec![Level::zero()])
}

/// `@Eq Prop p True`
fn eq_true_expr(p: Expr) -> Expr {
    app3(eq_prop(), prop(), p, cst("True"))
}

/// `@Eq Prop p False`
fn eq_false_expr(p: Expr) -> Expr {
    app3(eq_prop(), prop(), p, cst("False"))
}

// ── Private type-builder functions ───────────────────────────────────────────

/// `Eq.symm : ∀ {α : Sort 1} {a b : α}, @Eq α a b → @Eq α b a`
///
/// De Bruijn trace:
/// - After `{α : Sort 1}` (Implicit): stack=[α]; α=bvar(0)
/// - After `{a : α}` (Implicit): stack=[a,α]; a=bvar(0), α=bvar(1)
/// - After `{b : α}` (Implicit): stack=[b,a,α]; b=bvar(0), a=bvar(1), α=bvar(2)
/// - h domain: `@Eq α a b` → eq_app(bvar(2), bvar(1), bvar(0))
/// - After h (Default): stack=[h,b,a,α]
///   h=bvar(0), b=bvar(1), a=bvar(2), α=bvar(3)
/// - conclusion: `@Eq α b a` → eq_app(bvar(3), bvar(1), bvar(2))
fn ty_eq_symm() -> Expr {
    pi_implicit(
        "α",
        sort1(),
        pi_implicit(
            "a",
            bvar(0),
            pi_implicit(
                "b",
                bvar(1),
                pi_default(
                    "h",
                    eq_app(bvar(2), bvar(1), bvar(0)),
                    eq_app(bvar(3), bvar(1), bvar(2)),
                ),
            ),
        ),
    )
}

/// `Eq.trans : ∀ {α : Sort 1} {a b c : α}, @Eq α a b → @Eq α b c → @Eq α a c`
///
/// De Bruijn trace:
/// - After {α} (Implicit): α=bvar(0)
/// - After {a : α} (Implicit): a=bvar(0), α=bvar(1)
/// - After {b : α} (Implicit): b=bvar(0), a=bvar(1), α=bvar(2)
/// - After {c : α} (Implicit): c=bvar(0), b=bvar(1), a=bvar(2), α=bvar(3)
/// - h1 domain: `@Eq α a b` → eq_app(bvar(3), bvar(2), bvar(1))
/// - After h1: c=bvar(1), b=bvar(2), a=bvar(3), α=bvar(4)
///   (all 4 type-level binders shift by 1 past h1)
/// - h2 domain: `@Eq α b c` → eq_app(bvar(4), bvar(2), bvar(1))
/// - After h2: c=bvar(2), b=bvar(3), a=bvar(4), α=bvar(5)
/// - conclusion: `@Eq α a c` → eq_app(bvar(5), bvar(4), bvar(2))
fn ty_eq_trans() -> Expr {
    pi_implicit(
        "α",
        sort1(),
        pi_implicit(
            "a",
            bvar(0),
            pi_implicit(
                "b",
                bvar(1),
                pi_implicit(
                    "c",
                    bvar(2),
                    pi_default(
                        "h1",
                        eq_app(bvar(3), bvar(2), bvar(1)),
                        pi_default(
                            "h2",
                            eq_app(bvar(4), bvar(2), bvar(1)),
                            eq_app(bvar(5), bvar(4), bvar(2)),
                        ),
                    ),
                ),
            ),
        ),
    )
}

/// `congrArg : ∀ {α β : Sort 1} {a b : α} (f : α → β), @Eq α a b → @Eq β (f a) (f b)`
///
/// De Bruijn trace:
/// - After {α} (Implicit): α=bvar(0)
/// - After {β} (Implicit): β=bvar(0), α=bvar(1)
/// - After {a : α} (Implicit): a=bvar(0), β=bvar(1), α=bvar(2)
/// - After {b : α} (Implicit): b=bvar(0), a=bvar(1), β=bvar(2), α=bvar(3)
/// - After (f : α → β) (Default): f=bvar(0), b=bvar(1), a=bvar(2), β=bvar(3), α=bvar(4)
///   f's type = Pi(Default, "_", bvar(4 — but measured BEFORE f is bound) = α_at_that_depth)
///   At the point where f's type is written: stack=[b,a,β,α] → α=bvar(3), β=bvar(2)
///   f : α → β = Pi(Default, "_", bvar(3), bvar(3))  ...but bvar(3) for return β would be...
///   Actually: Pi("_", α, β) in context [b,a,β,α] → α=bvar(3), β=bvar(2)
///   So f : bvar(3) → bvar(2)
/// - h domain (in context [f,b,a,β,α]): `@Eq α a b` → α=bvar(4), a=bvar(3), b=bvar(2)
///   eq_app(bvar(4), bvar(3), bvar(2))
/// - After h: stack=[h,f,b,a,β,α]; h=bvar(0), f=bvar(1), b=bvar(2), a=bvar(3), β=bvar(4), α=bvar(5)
/// - conclusion: `@Eq β (f a) (f b)` → β=bvar(4), f=bvar(1), a=bvar(3), b=bvar(2)
///   eq_app(bvar(4), app(bvar(1), bvar(3)), app(bvar(1), bvar(2)))
fn ty_congr_arg() -> Expr {
    pi_implicit(
        "α",
        sort1(),
        pi_implicit(
            "β",
            sort1(),
            pi_implicit(
                "a",
                bvar(1),
                pi_implicit(
                    "b",
                    bvar(2),
                    pi_default(
                        "f",
                        // f : α → β; in context [b,a,β,α]: α=bvar(3), β=bvar(2)
                        pi_default("_", bvar(3), bvar(3)),
                        pi_default(
                            "h",
                            // @Eq α a b; in context [f,b,a,β,α]: α=bvar(4), a=bvar(3), b=bvar(2)
                            eq_app(bvar(4), bvar(3), bvar(2)),
                            // @Eq β (f a) (f b); after h: β=bvar(4), f=bvar(1), a=bvar(3), b=bvar(2)
                            eq_app(bvar(4), app(bvar(1), bvar(3)), app(bvar(1), bvar(2))),
                        ),
                    ),
                ),
            ),
        ),
    )
}

/// `Ne.symm : ∀ {α : Sort 1} {a b : α}, ¬(@Eq α a b) → ¬(@Eq α b a)`
///
/// Where `¬P = P → False`.
///
/// De Bruijn trace:
/// - After {α} (Implicit): α=bvar(0)
/// - After {a : α} (Implicit): a=bvar(0), α=bvar(1)
/// - After {b : α} (Implicit): b=bvar(0), a=bvar(1), α=bvar(2)
/// - h domain (Default): `¬(@Eq α a b)` = Pi("_", @Eq α a b, False)
///   In context [b,a,α]: eq_app(bvar(2), bvar(1), bvar(0)) → False
///   Written: pi_default("_", eq_app(bvar(2), bvar(1), bvar(0)), cst("False"))
/// - After h: stack=[h,b,a,α]; h=bvar(0), b=bvar(1), a=bvar(2), α=bvar(3)
/// - conclusion: `¬(@Eq α b a)` = Pi("_", @Eq α b a, False)
///   eq_app(bvar(3), bvar(1), bvar(2)) → False
fn ty_ne_symm() -> Expr {
    pi_implicit(
        "α",
        sort1(),
        pi_implicit(
            "a",
            bvar(0),
            pi_implicit(
                "b",
                bvar(1),
                pi_default(
                    "h",
                    // ¬(Eq α a b) = Eq α a b → False
                    pi_default("_", eq_app(bvar(2), bvar(1), bvar(0)), cst("False")),
                    // ¬(Eq α b a) = Eq α b a → False
                    pi_default("_", eq_app(bvar(3), bvar(1), bvar(2)), cst("False")),
                ),
            ),
        ),
    )
}

/// `eq_of_heq : ∀ {α : Sort 1} {a b : α}, @HEq α a α b → @Eq α a b`
///
/// Bridge from heterogeneous equality to homogeneous equality (same type).
/// When both sides have the same type α, `HEq a b` implies `Eq a b`.
///
/// De Bruijn trace:
/// - After {α} (Implicit): α=bvar(0)
/// - After {a : α} (Implicit): a=bvar(0), α=bvar(1)
/// - After {b : α} (Implicit): b=bvar(0), a=bvar(1), α=bvar(2)
/// - h domain: `@HEq α a α b` = app(app(app(app(heq_u1(), α, a), α), b))
///   In context [b,a,α]: α=bvar(2), a=bvar(1), b=bvar(0)
/// - After h: h=bvar(0), b=bvar(1), a=bvar(2), α=bvar(3)
/// - conclusion: `@Eq α a b` = eq_app(bvar(3), bvar(2), bvar(1))
fn ty_eq_of_heq() -> Expr {
    pi_implicit(
        "α",
        sort1(),
        pi_implicit(
            "a",
            bvar(0),
            pi_implicit(
                "b",
                bvar(1),
                pi_default(
                    "h",
                    // @HEq α a α b; in [b,a,α]: α=bvar(2), a=bvar(1), b=bvar(0)
                    app(app(app(app(heq_u1(), bvar(2)), bvar(1)), bvar(2)), bvar(0)),
                    // @Eq α a b; after h: α=bvar(3), a=bvar(2), b=bvar(1)
                    eq_app(bvar(3), bvar(2), bvar(1)),
                ),
            ),
        ),
    )
}

/// `Eq.subst : ∀ {α : Sort 1} {a b : α} (p : α → Prop), @Eq α a b → p a → p b`
///
/// Substitution principle: rewrite `p a` to `p b` via `a = b`.
///
/// De Bruijn trace:
/// - After {α} (Implicit): α=bvar(0)
/// - After {a : α} (Implicit): a=bvar(0), α=bvar(1)
/// - After {b : α} (Implicit): b=bvar(0), a=bvar(1), α=bvar(2)
/// - After (p : α → Prop) (Default): p=bvar(0), b=bvar(1), a=bvar(2), α=bvar(3)
///   p's type at [b,a,α]: Pi("_", α, Prop) = Pi("_", bvar(2), prop())
///   p's type at [p,b,a,α]: p maps bvar(3) to Prop (but we write the type before p is bound)
///   In context [b,a,α]: p : bvar(2) → prop()
/// - After p: stack=[p,b,a,α]; p=bvar(0), b=bvar(1), a=bvar(2), α=bvar(3)
/// - h domain (Default): `@Eq α a b` = eq_app(bvar(3), bvar(2), bvar(1))
/// - After h: stack=[h,p,b,a,α]; h=bvar(0), p=bvar(1), b=bvar(2), a=bvar(3), α=bvar(4)
/// - ha domain (Default): `p a` = app(bvar(1), bvar(3))
/// - After ha: h=bvar(1), p=bvar(2), b=bvar(3), a=bvar(4), α=bvar(5)
/// - conclusion: `p b` = app(bvar(2), bvar(3))
fn ty_eq_subst() -> Expr {
    pi_implicit(
        "α",
        sort1(),
        pi_implicit(
            "a",
            bvar(0),
            pi_implicit(
                "b",
                bvar(1),
                pi_default(
                    "p",
                    // p : α → Prop; in context [b,a,α]: α=bvar(2)
                    pi_default("_", bvar(2), prop()),
                    pi_default(
                        "h",
                        // @Eq α a b; in [p,b,a,α]: α=bvar(3), a=bvar(2), b=bvar(1)
                        eq_app(bvar(3), bvar(2), bvar(1)),
                        pi_default(
                            "ha",
                            // p a; in [h,p,b,a,α]: p=bvar(1), a=bvar(3)
                            app(bvar(1), bvar(3)),
                            // p b; in [ha,h,p,b,a,α]: p=bvar(2), b=bvar(3)
                            app(bvar(2), bvar(3)),
                        ),
                    ),
                ),
            ),
        ),
    )
}

/// `congr : ∀ {α β : Sort 1} {f g : α → β} {a b : α}, @Eq (α → β) f g → @Eq α a b → @Eq β (f a) (g b)`
///
/// Full congruence: simultaneous rewriting of both function and argument.
///
/// De Bruijn trace:
/// - After {α} (Implicit): α=bvar(0)
/// - After {β} (Implicit): β=bvar(0), α=bvar(1)
/// - After {f : α → β} (Implicit): f=bvar(0), β=bvar(1), α=bvar(2)
///   f's type in [β,α]: Pi("_", bvar(1), bvar(1)) — α=bvar(1), β=bvar(1)?
///   Wait: in context [β,α]: α=bvar(1), β=bvar(0). So f : α→β = Pi("_", bvar(1), bvar(1)).
///   That's wrong — both bvars would be the same. Need to track carefully:
///   In context [β,α]: α=bvar(1), β=bvar(0).
///   f : α → β = pi_default("_", bvar(1), bvar(1)) is wrong because β=bvar(0) not bvar(1).
///   Correct: f : α → β = pi_default("_", bvar(1), bvar(1))... no: α=bvar(1), β=bvar(0).
///   So f : pi_default("_", bvar(1), bvar(1)) would make it α → α. Wrong.
///   Correct f : pi_default("_", bvar(1), bvar(0)) = α → β. Yes!
/// - After {f} (Implicit): f=bvar(0), β=bvar(1), α=bvar(2)
/// - After {g : α → β} (Implicit): g=bvar(0), f=bvar(1), β=bvar(2), α=bvar(3)
///   g's type in [f,β,α]: pi_default("_", bvar(2), bvar(1)) = α→β where α=bvar(2), β=bvar(1)
/// - After {a : α} (Implicit): a=bvar(0), g=bvar(1), f=bvar(2), β=bvar(3), α=bvar(4)
///   a's type in [g,f,β,α]: bvar(3) = α
/// - After {b : α} (Implicit): b=bvar(0), a=bvar(1), g=bvar(2), f=bvar(3), β=bvar(4), α=bvar(5)
///   b's type in [a,g,f,β,α]: bvar(4) = α
/// - hf domain (Default): `@Eq (α→β) f g`
///   In context [b,a,g,f,β,α]: α=bvar(5), β=bvar(4), f=bvar(3), g=bvar(2)
///   (α→β) = pi_default("_", bvar(5), bvar(4))
///   eq_app(pi_default("_", bvar(5), bvar(4)), bvar(3), bvar(2))
/// - After hf: hf=bvar(0), b=bvar(1), a=bvar(2), g=bvar(3), f=bvar(4), β=bvar(5), α=bvar(6)
/// - ha domain: `@Eq α a b`
///   In context [hf,b,a,g,f,β,α]: α=bvar(6), a=bvar(2), b=bvar(1)
///   eq_app(bvar(6), bvar(2), bvar(1))
/// - After ha: ha=bvar(0), hf=bvar(1), b=bvar(2), a=bvar(3), g=bvar(4), f=bvar(5), β=bvar(6), α=bvar(7)
/// - conclusion: `@Eq β (f a) (g b)`
///   β=bvar(6), f=bvar(5), a=bvar(3), g=bvar(4), b=bvar(2)
///   eq_app(bvar(6), app(bvar(5), bvar(3)), app(bvar(4), bvar(2)))
fn ty_congr() -> Expr {
    // Corrected De Bruijn indices (see detailed comment above ty_congr).
    // Key principle: inside Pi("_", dom, cod), the cod has one extra binder at
    // index 0 for the anonymous argument, so all outer-context indices shift +1.
    //
    // f binder context [β,α]: α=bvar(1),β=bvar(0). f: dom=bvar(1)=α, cod in [_,β,α]=bvar(1)=β.
    //   → pi_default("_", bvar(1), bvar(1)). ✓
    // g binder context [f,β,α]: α=bvar(2),β=bvar(1),f=bvar(0). g: dom=bvar(2)=α, cod in [_,f,β,α]=bvar(2)=β.
    //   → pi_default("_", bvar(2), bvar(2)). ✓
    // hf's (α→β) in context [b,a,g,f,β,α]: α=bvar(5). In [_,...]: β=bvar(5).
    //   → pi_default("_", bvar(5), bvar(5)). ✓
    pi_implicit(
        "α",
        sort1(),
        pi_implicit(
            "β",
            sort1(),
            pi_implicit(
                "f",
                // f : α→β; dom=bvar(1)=α (outer [β,α]), cod=bvar(1)=β (inner [_,β,α])
                pi_default("_", bvar(1), bvar(1)),
                pi_implicit(
                    "g",
                    // g : α→β; dom=bvar(2)=α (outer [f,β,α]), cod=bvar(2)=β (inner [_,f,β,α])
                    pi_default("_", bvar(2), bvar(2)),
                    pi_implicit(
                        "a",
                        // a : α; in context [g,f,β,α]: α=bvar(3)
                        bvar(3),
                        pi_implicit(
                            "b",
                            // b : α; in context [a,g,f,β,α]: α=bvar(4)
                            bvar(4),
                            pi_default(
                                "hf",
                                // @Eq (α→β) f g; in [b,a,g,f,β,α]: α=bvar(5), f=bvar(3), g=bvar(2)
                                // α→β: dom=bvar(5)=α; cod in [_,b,a,g,f,β,α]=bvar(5)=β.
                                eq_app(pi_default("_", bvar(5), bvar(5)), bvar(3), bvar(2)),
                                pi_default(
                                    "ha",
                                    // @Eq α a b; in [hf,b,a,g,f,β,α]: α=bvar(6), a=bvar(2), b=bvar(1)
                                    eq_app(bvar(6), bvar(2), bvar(1)),
                                    // @Eq β (f a) (g b); β=bvar(6), f=bvar(5), a=bvar(3), g=bvar(4), b=bvar(2)
                                    eq_app(bvar(6), app(bvar(5), bvar(3)), app(bvar(4), bvar(2))),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}

/// `eq_true_intro : ∀ {p : Prop}, p → p = True`
///
/// Prop-level Bool-reflection: a proof of `p` gives `p = True`.
///
/// De Bruijn trace:
/// - After {p : Prop} (Implicit): p=bvar(0)
/// - h domain: p = bvar(0)
/// - After h: p=bvar(1)
/// - conclusion: `p = True` = eq_true_expr(bvar(1))
fn ty_eq_true_intro() -> Expr {
    pi_implicit("p", prop(), pi_default("h", bvar(0), eq_true_expr(bvar(1))))
}

/// `eq_false_intro : ∀ {p : Prop}, ¬p → p = False`
///
/// Prop-level Bool-reflection: a proof of `¬p` gives `p = False`.
/// `¬p` = `p → False` = `pi_default("_", bvar(0), cst("False"))`.
///
/// De Bruijn trace:
/// - After {p : Prop} (Implicit): p=bvar(0)
/// - h domain: `¬p` = pi_default("_", bvar(0), cst("False"))
/// - After h: p=bvar(1)
/// - conclusion: `p = False` = eq_false_expr(bvar(1))
fn ty_eq_false_intro() -> Expr {
    pi_implicit(
        "p",
        prop(),
        pi_default(
            "h",
            pi_default("_", bvar(0), cst("False")),
            eq_false_expr(bvar(1)),
        ),
    )
}

// ── Public registration function ─────────────────────────────────────────────

/// Register the congruence-closure helper lemma bundle into `env`.
///
/// Adds axiom-backed declarations for the 9 equality/congruence lemmas that the
/// cc proof reconstruction compiler uses. Each lemma is guarded: if already
/// present in `env` (e.g., because `build_eq_env` was called first), it is
/// silently skipped. This makes the function idempotent.
///
/// The bundle includes:
/// - `Eq.symm`: `∀ {α : Sort 1} {a b : α}, a = b → b = a`
/// - `Eq.trans`: `∀ {α : Sort 1} {a b c : α}, a = b → b = c → a = c`
/// - `congrArg`: `∀ {α β : Sort 1} {a b : α} (f : α → β), a = b → f a = f b`
/// - `Ne.symm` (new): `∀ {α : Sort 1} {a b : α}, ¬(a=b) → ¬(b=a)`
/// - `eq_of_heq` (new): `∀ {α : Sort 1} {a b : α}, HEq a b → a = b`
/// - `Eq.subst`: `∀ {α : Sort 1} {a b : α} (p : α → Prop), a = b → p a → p b`
/// - `congr`: `∀ {α β : Sort 1} {f g : α→β} {a b : α}, f=g → a=b → f a = g b`
/// - `eq_true_intro`: `∀ {p : Prop}, p → p = True`
/// - `eq_false_intro`: `∀ {p : Prop}, ¬p → p = False`
///
/// # Errors
///
/// Returns `EnvError` if an unexpected name collision occurs (should not happen
/// in normal usage due to the `contains` guards).
pub fn register_cc_helper(env: &mut Environment) -> Result<(), EnvError> {
    let lemmas: &[(&str, Expr)] = &[
        ("Eq.symm", ty_eq_symm()),
        ("Eq.trans", ty_eq_trans()),
        ("congrArg", ty_congr_arg()),
        ("Ne.symm", ty_ne_symm()),
        ("eq_of_heq", ty_eq_of_heq()),
        ("Eq.subst", ty_eq_subst()),
        ("congr", ty_congr()),
        ("eq_true_intro", ty_eq_true_intro()),
        ("eq_false_intro", ty_eq_false_intro()),
    ];

    for (name, ty) in lemmas {
        let n = Name::str(*name);
        if env.contains(&n) {
            continue;
        }
        env.add(Declaration::Axiom {
            name: n,
            univ_params: vec![],
            ty: ty.clone(),
        })?;
    }
    Ok(())
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    const CC_LEMMAS: &[&str] = &[
        "Eq.symm",
        "Eq.trans",
        "congrArg",
        "Ne.symm",
        "eq_of_heq",
        "Eq.subst",
        "congr",
        "eq_true_intro",
        "eq_false_intro",
    ];

    #[test]
    fn test_register_cc_helper_success() {
        let mut env = Environment::new();
        register_cc_helper(&mut env).expect("registration should succeed");
    }

    #[test]
    fn test_all_lemmas_present() {
        let mut env = Environment::new();
        register_cc_helper(&mut env).expect("registration should succeed");

        for name in CC_LEMMAS {
            assert!(
                env.contains(&Name::str(*name)),
                "lemma {} should be registered",
                name
            );
        }
    }

    #[test]
    fn test_idempotent_registration() {
        let mut env = Environment::new();
        register_cc_helper(&mut env).expect("first registration");
        register_cc_helper(&mut env).expect("second registration should succeed");

        for name in CC_LEMMAS {
            assert!(
                env.contains(&Name::str(*name)),
                "lemma {} should still be registered after second call",
                name
            );
        }
    }

    #[test]
    fn test_lemma_count_in_fresh_env() {
        let mut env = Environment::new();
        register_cc_helper(&mut env).expect("registration should succeed");
        assert_eq!(env.len(), CC_LEMMAS.len(), "expected 9 cc lemmas");
    }

    #[test]
    fn test_lemma_count_is_9() {
        assert_eq!(CC_LEMMAS.len(), 9, "CC_LEMMAS should list 9 entries");
    }

    #[test]
    fn test_eq_symm_present() {
        let mut env = Environment::new();
        register_cc_helper(&mut env).expect("registration should succeed");
        assert!(
            env.contains(&Name::str("Eq.symm")),
            "Eq.symm should be registered"
        );
    }

    #[test]
    fn test_ne_symm_present() {
        let mut env = Environment::new();
        register_cc_helper(&mut env).expect("registration should succeed");
        assert!(
            env.contains(&Name::str("Ne.symm")),
            "Ne.symm should be registered"
        );
    }

    #[test]
    fn test_eq_of_heq_present() {
        let mut env = Environment::new();
        register_cc_helper(&mut env).expect("registration should succeed");
        assert!(
            env.contains(&Name::str("eq_of_heq")),
            "eq_of_heq should be registered"
        );
    }

    #[test]
    fn test_eq_symm_type_starts_with_implicit_pi() {
        let ty = ty_eq_symm();
        assert!(
            matches!(ty, Expr::Pi(BinderInfo::Implicit, _, _, _)),
            "Eq.symm type should start with an implicit Pi binder ({{α : Sort 1}})"
        );
    }

    #[test]
    fn test_ne_symm_type_starts_with_implicit_pi() {
        let ty = ty_ne_symm();
        assert!(
            matches!(ty, Expr::Pi(BinderInfo::Implicit, _, _, _)),
            "Ne.symm type should start with an implicit Pi binder ({{α : Sort 1}})"
        );
    }

    #[test]
    fn test_eq_of_heq_type_starts_with_implicit_pi() {
        let ty = ty_eq_of_heq();
        assert!(
            matches!(ty, Expr::Pi(BinderInfo::Implicit, _, _, _)),
            "eq_of_heq type should start with an implicit Pi binder ({{α : Sort 1}})"
        );
    }

    #[test]
    fn test_eq_true_intro_type_starts_with_implicit_pi() {
        let ty = ty_eq_true_intro();
        assert!(
            matches!(ty, Expr::Pi(BinderInfo::Implicit, _, _, _)),
            "eq_true_intro type should start with an implicit Pi binder ({{p : Prop}})"
        );
    }

    #[test]
    fn test_eq_false_intro_type_starts_with_implicit_pi() {
        let ty = ty_eq_false_intro();
        assert!(
            matches!(ty, Expr::Pi(BinderInfo::Implicit, _, _, _)),
            "eq_false_intro type should start with an implicit Pi binder ({{p : Prop}})"
        );
    }

    #[test]
    fn test_idempotent_count_stable() {
        let mut env = Environment::new();
        register_cc_helper(&mut env).expect("first registration");
        let count_after_first = env.len();
        register_cc_helper(&mut env).expect("second registration should succeed");
        assert_eq!(
            env.len(),
            count_after_first,
            "second registration must not add any new entries"
        );
    }

    #[test]
    fn test_coexistence_with_preregistered() {
        let mut env = Environment::new();
        // Simulate pre-registration of lemmas that build_eq_env might add.
        let pre_existing = &[
            ("Eq.symm", ty_eq_symm()),
            ("Eq.trans", ty_eq_trans()),
            ("congrArg", ty_congr_arg()),
        ];
        for (name, ty) in pre_existing {
            env.add(Declaration::Axiom {
                name: Name::str(*name),
                univ_params: vec![],
                ty: ty.clone(),
            })
            .expect("pre-registration should succeed");
        }
        // register_cc_helper should skip the 3 pre-existing and add the remaining 6.
        register_cc_helper(&mut env).expect("registration with pre-existing should succeed");
        for name in CC_LEMMAS {
            assert!(
                env.contains(&Name::str(*name)),
                "lemma {} should be present after coexistence test",
                name
            );
        }
        assert_eq!(env.len(), 9, "total should be 9 after coexistence test");
    }

    #[test]
    fn test_env_find_after_registration() {
        let mut env = Environment::new();
        register_cc_helper(&mut env).expect("registration should succeed");
        assert!(env.find(&Name::str("Eq.symm")).is_some());
        assert!(env.find(&Name::str("Ne.symm")).is_some());
        assert!(env.find(&Name::str("eq_of_heq")).is_some());
        assert!(env.find(&Name::str("congr")).is_some());
        assert!(env.find(&Name::str("eq_true_intro")).is_some());
        assert!(env.find(&Name::str("eq_false_intro")).is_some());
    }
}
