//! # Linarith Helper Lemma Bundle
//!
//! This module provides axiom-backed declarations for integer arithmetic lemmas
//! that the linarith proof reconstruction compiler uses when building proof terms.
//!
//! The 12 lemmas cover strict and mixed inequality reasoning needed when the
//! proof strategy uses multiply+add rather than pure transitivity cycles:
//!
//! - 3 `add_lt_add` variants (strict + mixed strict/le)
//! - 2 `add_le_add` left/right scale lemmas
//! - 2 negation-flips (`neg_le_neg`, `neg_lt_neg`)
//! - 2 integer-discreteness bridges (`lt_add_one_of_le`, `le_of_lt_add_one`)
//! - 1 nonnegativity closure (`mul_nonneg`)
//! - 2 iff/disjunctive strict lemmas (`lt_of_le_of_ne`, `lt_iff_le_and_ne`)

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

fn int_const() -> Expr {
    cst("Int")
}

/// `Int.ofNat 0` — the integer zero.
fn int_zero() -> Expr {
    app(cst("Int.ofNat"), Expr::Lit(Literal::nat(0)))
}

/// `Int.ofNat 1` — the integer one.
fn int_one() -> Expr {
    app(cst("Int.ofNat"), Expr::Lit(Literal::nat(1)))
}

/// Propositional Int equality: `@Eq Int a b`
fn int_eq_expr(a: Expr, b: Expr) -> Expr {
    let eq_const = Expr::Const(Name::str("Eq"), vec![Level::succ(Level::zero())]);
    app(app(app(eq_const, int_const()), a), b)
}

/// `¬P` expressed as `Not P` (opaque negation)
fn not_expr(p: Expr) -> Expr {
    app(cst("Not"), p)
}

/// Wrap body in `∀ (a : Int), body`.
fn forall1_int(body: Expr) -> Expr {
    pi("a", int_const(), body)
}

/// Wrap body in `∀ (a b : Int), body`.
fn forall2_int(body: Expr) -> Expr {
    pi("a", int_const(), pi("b", int_const(), body))
}

/// Wrap body in `∀ (a b c d : Int), body`.
fn forall4_int(body: Expr) -> Expr {
    pi(
        "a",
        int_const(),
        pi(
            "b",
            int_const(),
            pi("c", int_const(), pi("d", int_const(), body)),
        ),
    )
}

// ── Private type-builder functions ───────────────────────────────────────────

/// `Int.add_lt_add : ∀ (a b c d : Int), Int.lt a b → Int.lt c d → Int.lt (Int.add a c) (Int.add b d)`
///
/// De Bruijn trace:
/// - forall4_int binds a b c d: after all 4: a=bvar(3), b=bvar(2), c=bvar(1), d=bvar(0)
/// - h1 domain: `Int.lt a b` → a=bvar(3), b=bvar(2)
/// - after h1 binder: a=bvar(4), b=bvar(3), c=bvar(2), d=bvar(1)
/// - h2 domain: `Int.lt c d` → c=bvar(2), d=bvar(1)
/// - after h2 binder: a=bvar(5), b=bvar(4), c=bvar(3), d=bvar(2)
/// - conclusion: `Int.lt (a+c) (b+d)` → a=bvar(5), b=bvar(4), c=bvar(3), d=bvar(2)
fn ty_add_lt_add() -> Expr {
    forall4_int(pi(
        "h1",
        app2(cst("Int.lt"), bvar(3), bvar(2)),
        pi(
            "h2",
            app2(cst("Int.lt"), bvar(2), bvar(1)),
            app2(
                cst("Int.lt"),
                app2(cst("Int.add"), bvar(5), bvar(3)),
                app2(cst("Int.add"), bvar(4), bvar(2)),
            ),
        ),
    ))
}

/// `Int.add_lt_add_of_le_of_lt : ∀ (a b c d : Int), Int.le a b → Int.lt c d → Int.lt (Int.add a c) (Int.add b d)`
///
/// De Bruijn trace: identical shape to `add_lt_add` but h1 uses `Int.le`.
/// - after forall4_int: a=bvar(3), b=bvar(2), c=bvar(1), d=bvar(0)
/// - h1 domain (le): a=bvar(3), b=bvar(2)
/// - after h1: a=bvar(4), b=bvar(3), c=bvar(2), d=bvar(1)
/// - h2 domain (lt): c=bvar(2), d=bvar(1)
/// - after h2: a=bvar(5), b=bvar(4), c=bvar(3), d=bvar(2)
/// - conclusion: a=bvar(5), b=bvar(4), c=bvar(3), d=bvar(2)
fn ty_add_lt_add_of_le_of_lt() -> Expr {
    forall4_int(pi(
        "h1",
        app2(cst("Int.le"), bvar(3), bvar(2)),
        pi(
            "h2",
            app2(cst("Int.lt"), bvar(2), bvar(1)),
            app2(
                cst("Int.lt"),
                app2(cst("Int.add"), bvar(5), bvar(3)),
                app2(cst("Int.add"), bvar(4), bvar(2)),
            ),
        ),
    ))
}

/// `Int.add_lt_add_of_lt_of_le : ∀ (a b c d : Int), Int.lt a b → Int.le c d → Int.lt (Int.add a c) (Int.add b d)`
///
/// De Bruijn trace: identical shape but h1 uses `Int.lt`, h2 uses `Int.le`.
/// - after forall4_int: a=bvar(3), b=bvar(2), c=bvar(1), d=bvar(0)
/// - h1 domain (lt): a=bvar(3), b=bvar(2)
/// - after h1: a=bvar(4), b=bvar(3), c=bvar(2), d=bvar(1)
/// - h2 domain (le): c=bvar(2), d=bvar(1)
/// - after h2: a=bvar(5), b=bvar(4), c=bvar(3), d=bvar(2)
fn ty_add_lt_add_of_lt_of_le() -> Expr {
    forall4_int(pi(
        "h1",
        app2(cst("Int.lt"), bvar(3), bvar(2)),
        pi(
            "h2",
            app2(cst("Int.le"), bvar(2), bvar(1)),
            app2(
                cst("Int.lt"),
                app2(cst("Int.add"), bvar(5), bvar(3)),
                app2(cst("Int.add"), bvar(4), bvar(2)),
            ),
        ),
    ))
}

/// `Int.add_le_add_left : ∀ (a b c : Int), Int.le a b → Int.le (Int.add c a) (Int.add c b)`
///
/// Scale-left: add a common left summand `c` to both sides of `a ≤ b`.
///
/// De Bruijn trace:
/// - Bind a b c: after all 3: a=bvar(2), b=bvar(1), c=bvar(0)
/// - h domain: `Int.le a b` → a=bvar(2), b=bvar(1)
/// - after h: a=bvar(3), b=bvar(2), c=bvar(1)
/// - conclusion: `Int.le (c+a) (c+b)` → c=bvar(1), a=bvar(3), b=bvar(2)
fn ty_add_le_add_left() -> Expr {
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
                    "h",
                    app2(cst("Int.le"), bvar(2), bvar(1)),
                    app2(
                        cst("Int.le"),
                        app2(cst("Int.add"), bvar(1), bvar(3)),
                        app2(cst("Int.add"), bvar(1), bvar(2)),
                    ),
                ),
            ),
        ),
    )
}

/// `Int.add_le_add_right : ∀ (a b c : Int), Int.le a b → Int.le (Int.add a c) (Int.add b c)`
///
/// Scale-right: add a common right summand `c` to both sides of `a ≤ b`.
///
/// De Bruijn trace:
/// - Bind a b c: after all 3: a=bvar(2), b=bvar(1), c=bvar(0)
/// - h domain: `Int.le a b` → a=bvar(2), b=bvar(1)
/// - after h: a=bvar(3), b=bvar(2), c=bvar(1)
/// - conclusion: `Int.le (a+c) (b+c)` → a=bvar(3), b=bvar(2), c=bvar(1)
fn ty_add_le_add_right() -> Expr {
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
                    "h",
                    app2(cst("Int.le"), bvar(2), bvar(1)),
                    app2(
                        cst("Int.le"),
                        app2(cst("Int.add"), bvar(3), bvar(1)),
                        app2(cst("Int.add"), bvar(2), bvar(1)),
                    ),
                ),
            ),
        ),
    )
}

/// `Int.neg_le_neg : ∀ (a b : Int), Int.le a b → Int.le (Int.neg b) (Int.neg a)`
///
/// Negation flips the direction of ≤.
///
/// De Bruijn trace:
/// - forall2_int: after a b: a=bvar(1), b=bvar(0)
/// - h domain: `Int.le a b` → a=bvar(1), b=bvar(0)
/// - after h: a=bvar(2), b=bvar(1)
/// - conclusion: `Int.le (neg b) (neg a)` → b=bvar(1), a=bvar(2)
fn ty_neg_le_neg() -> Expr {
    forall2_int(pi(
        "h",
        app2(cst("Int.le"), bvar(1), bvar(0)),
        app2(
            cst("Int.le"),
            app(cst("Int.neg"), bvar(1)),
            app(cst("Int.neg"), bvar(2)),
        ),
    ))
}

/// `Int.neg_lt_neg : ∀ (a b : Int), Int.lt a b → Int.lt (Int.neg b) (Int.neg a)`
///
/// Negation flips the direction of <.
///
/// De Bruijn trace (identical shape to `neg_le_neg` but with `Int.lt`):
/// - forall2_int: after a b: a=bvar(1), b=bvar(0)
/// - h domain: `Int.lt a b` → a=bvar(1), b=bvar(0)
/// - after h: a=bvar(2), b=bvar(1)
/// - conclusion: `Int.lt (neg b) (neg a)` → b=bvar(1), a=bvar(2)
fn ty_neg_lt_neg() -> Expr {
    forall2_int(pi(
        "h",
        app2(cst("Int.lt"), bvar(1), bvar(0)),
        app2(
            cst("Int.lt"),
            app(cst("Int.neg"), bvar(1)),
            app(cst("Int.neg"), bvar(2)),
        ),
    ))
}

/// `Int.lt_add_one_of_le : ∀ (a b : Int), Int.le a b → Int.lt a (Int.add b (Int.ofNat 1))`
///
/// Integer discreteness bridge: `a ≤ b` implies `a < b + 1`.
///
/// De Bruijn trace:
/// - forall2_int: after a b: a=bvar(1), b=bvar(0)
/// - h domain: `Int.le a b` → a=bvar(1), b=bvar(0)
/// - after h: a=bvar(2), b=bvar(1)
/// - conclusion: `Int.lt a (b+1)` → a=bvar(2), b=bvar(1)
fn ty_lt_add_one_of_le() -> Expr {
    forall2_int(pi(
        "h",
        app2(cst("Int.le"), bvar(1), bvar(0)),
        app2(
            cst("Int.lt"),
            bvar(2),
            app2(cst("Int.add"), bvar(1), int_one()),
        ),
    ))
}

/// `Int.le_of_lt_add_one : ∀ (a b : Int), Int.lt a (Int.add b (Int.ofNat 1)) → Int.le a b`
///
/// Integer discreteness bridge (converse): `a < b + 1` implies `a ≤ b`.
///
/// De Bruijn trace:
/// - forall2_int: after a b: a=bvar(1), b=bvar(0)
/// - h domain: `Int.lt a (b+1)` → a=bvar(1), b=bvar(0)
/// - after h: a=bvar(2), b=bvar(1)
/// - conclusion: `Int.le a b` → a=bvar(2), b=bvar(1)
fn ty_le_of_lt_add_one() -> Expr {
    forall2_int(pi(
        "h",
        app2(
            cst("Int.lt"),
            bvar(1),
            app2(cst("Int.add"), bvar(0), int_one()),
        ),
        app2(cst("Int.le"), bvar(2), bvar(1)),
    ))
}

/// `Int.mul_nonneg : ∀ (a b : Int), Int.le (Int.ofNat 0) a → Int.le (Int.ofNat 0) b → Int.le (Int.ofNat 0) (Int.mul a b)`
///
/// Product of two non-negative integers is non-negative.
///
/// De Bruijn trace:
/// - forall2_int: after a b: a=bvar(1), b=bvar(0)
/// - h1 domain: `0 ≤ a` → a=bvar(1)
/// - after h1: a=bvar(2), b=bvar(1)
/// - h2 domain: `0 ≤ b` → b=bvar(1)
/// - after h2: a=bvar(3), b=bvar(2)
/// - conclusion: `0 ≤ (a*b)` → a=bvar(3), b=bvar(2)
fn ty_mul_nonneg() -> Expr {
    forall2_int(pi(
        "h1",
        app2(cst("Int.le"), int_zero(), bvar(1)),
        pi(
            "h2",
            app2(cst("Int.le"), int_zero(), bvar(1)),
            app2(
                cst("Int.le"),
                int_zero(),
                app2(cst("Int.mul"), bvar(3), bvar(2)),
            ),
        ),
    ))
}

/// `Int.lt_of_le_of_ne : ∀ (a b : Int), Int.le a b → ¬(Int.eq a b) → Int.lt a b`
///
/// From `a ≤ b` and `a ≠ b`, conclude `a < b`.
///
/// De Bruijn trace:
/// - forall2_int: after a b: a=bvar(1), b=bvar(0)
/// - h1 domain: `Int.le a b` → a=bvar(1), b=bvar(0)
/// - after h1: a=bvar(2), b=bvar(1)
/// - h2 domain: `¬(Int.eq a b)` = `Not (Int.eq a b)` → a=bvar(2), b=bvar(1)
/// - after h2: a=bvar(3), b=bvar(2)
/// - conclusion: `Int.lt a b` → a=bvar(3), b=bvar(2)
fn ty_lt_of_le_of_ne() -> Expr {
    forall2_int(pi(
        "h1",
        app2(cst("Int.le"), bvar(1), bvar(0)),
        pi(
            "h2",
            not_expr(app2(cst("Int.eq"), bvar(2), bvar(1))),
            app2(cst("Int.lt"), bvar(3), bvar(2)),
        ),
    ))
}

/// `Int.lt_iff_le_and_ne : ∀ (a b : Int), Iff (Int.lt a b) (And (Int.le a b) (¬Int.eq a b))`
///
/// Characterises strict inequality as the conjunction of weak inequality and non-equality.
///
/// De Bruijn trace:
/// - forall2_int: after a b: a=bvar(1), b=bvar(0)
/// - Body is a ground Iff (no additional binders):
///   LHS: `Int.lt a b` → a=bvar(1), b=bvar(0)
///   RHS: `And (Int.le a b) (Not (Int.eq a b))` → a=bvar(1), b=bvar(0)
fn ty_lt_iff_le_and_ne() -> Expr {
    forall2_int(app2(
        cst("Iff"),
        app2(cst("Int.lt"), bvar(1), bvar(0)),
        app2(
            cst("And"),
            app2(cst("Int.le"), bvar(1), bvar(0)),
            not_expr(app2(cst("Int.eq"), bvar(1), bvar(0))),
        ),
    ))
}

// ── Public registration function ─────────────────────────────────────────────

/// Register the linarith helper lemma bundle into `env`.
///
/// Adds axiom-backed declarations for the 12 integer arithmetic lemmas that the
/// linarith proof reconstruction compiler uses. Each lemma is guarded: if already
/// present in `env`, it is silently skipped. This makes the function idempotent.
///
/// The bundle includes:
/// - 3 `add_lt_add` variants: strict+strict, le+lt, lt+le
/// - 2 `add_le_add` left/right constant-offset scale lemmas
/// - 2 negation monotonicity lemmas: `neg_le_neg`, `neg_lt_neg`
/// - 2 integer-discreteness bridges: `lt_add_one_of_le`, `le_of_lt_add_one`
/// - 1 nonnegativity closure: `mul_nonneg`
/// - 1 lt-from-le-and-ne: `lt_of_le_of_ne`
/// - 1 iff characterisation: `lt_iff_le_and_ne`
///
/// # Errors
///
/// Returns `EnvError` if an unexpected name collision occurs (should not happen
/// in normal usage due to the `contains` guards).
pub fn register_linarith_helper(env: &mut Environment) -> Result<(), EnvError> {
    let lemmas: &[(&str, Expr)] = &[
        ("Int.add_lt_add", ty_add_lt_add()),
        ("Int.add_lt_add_of_le_of_lt", ty_add_lt_add_of_le_of_lt()),
        ("Int.add_lt_add_of_lt_of_le", ty_add_lt_add_of_lt_of_le()),
        ("Int.add_le_add_left", ty_add_le_add_left()),
        ("Int.add_le_add_right", ty_add_le_add_right()),
        ("Int.neg_le_neg", ty_neg_le_neg()),
        ("Int.neg_lt_neg", ty_neg_lt_neg()),
        ("Int.lt_add_one_of_le", ty_lt_add_one_of_le()),
        ("Int.le_of_lt_add_one", ty_le_of_lt_add_one()),
        ("Int.mul_nonneg", ty_mul_nonneg()),
        ("Int.lt_of_le_of_ne", ty_lt_of_le_of_ne()),
        ("Int.lt_iff_le_and_ne", ty_lt_iff_le_and_ne()),
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

    const LINARITH_LEMMAS: &[&str] = &[
        "Int.add_lt_add",
        "Int.add_lt_add_of_le_of_lt",
        "Int.add_lt_add_of_lt_of_le",
        "Int.add_le_add_left",
        "Int.add_le_add_right",
        "Int.neg_le_neg",
        "Int.neg_lt_neg",
        "Int.lt_add_one_of_le",
        "Int.le_of_lt_add_one",
        "Int.mul_nonneg",
        "Int.lt_of_le_of_ne",
        "Int.lt_iff_le_and_ne",
    ];

    #[test]
    fn test_register_linarith_helper_success() {
        let mut env = Environment::new();
        register_linarith_helper(&mut env).expect("registration should succeed");
    }

    #[test]
    fn test_all_lemmas_present() {
        let mut env = Environment::new();
        register_linarith_helper(&mut env).expect("registration should succeed");

        for name in LINARITH_LEMMAS {
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
        register_linarith_helper(&mut env).expect("first registration");
        register_linarith_helper(&mut env).expect("second registration should succeed");

        for name in LINARITH_LEMMAS {
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
        register_linarith_helper(&mut env).expect("registration should succeed");
        assert_eq!(
            env.len(),
            LINARITH_LEMMAS.len(),
            "expected 12 linarith lemmas"
        );
    }

    #[test]
    fn test_lemma_count_is_12() {
        assert_eq!(
            LINARITH_LEMMAS.len(),
            12,
            "LINARITH_LEMMAS should list 12 entries"
        );
    }

    #[test]
    fn test_add_lt_add_type_is_pi() {
        let ty = ty_add_lt_add();
        assert!(
            matches!(ty, Expr::Pi(BinderInfo::Default, _, _, _)),
            "add_lt_add type should start with a default Pi binder (a : Int)"
        );
    }

    #[test]
    fn test_add_lt_add_of_le_of_lt_type_is_pi() {
        let ty = ty_add_lt_add_of_le_of_lt();
        assert!(
            matches!(ty, Expr::Pi(BinderInfo::Default, _, _, _)),
            "add_lt_add_of_le_of_lt type should start with a default Pi binder"
        );
    }

    #[test]
    fn test_neg_le_neg_type_is_pi() {
        let ty = ty_neg_le_neg();
        assert!(
            matches!(ty, Expr::Pi(BinderInfo::Default, _, _, _)),
            "neg_le_neg type should start with a default Pi binder"
        );
    }

    #[test]
    fn test_neg_lt_neg_type_is_pi() {
        let ty = ty_neg_lt_neg();
        assert!(
            matches!(ty, Expr::Pi(BinderInfo::Default, _, _, _)),
            "neg_lt_neg type should start with a default Pi binder"
        );
    }

    #[test]
    fn test_mul_nonneg_type_is_pi() {
        let ty = ty_mul_nonneg();
        assert!(
            matches!(ty, Expr::Pi(BinderInfo::Default, _, _, _)),
            "mul_nonneg type should start with a default Pi binder (a : Int)"
        );
    }

    #[test]
    fn test_lt_iff_le_and_ne_type_is_pi() {
        let ty = ty_lt_iff_le_and_ne();
        assert!(
            matches!(ty, Expr::Pi(BinderInfo::Default, _, _, _)),
            "lt_iff_le_and_ne type should start with a default Pi binder"
        );
    }

    #[test]
    fn test_lt_iff_le_and_ne_body_is_app() {
        let ty = ty_lt_iff_le_and_ne();
        // Unwrap the two forall binders to reach the Iff body.
        if let Expr::Pi(_, _, _, body1) = ty {
            if let Expr::Pi(_, _, _, body2) = (*body1).clone() {
                // body2 should be App(App(Iff, ...), ...) — a ground Iff
                assert!(
                    matches!(*body2, Expr::App(_, _)),
                    "lt_iff_le_and_ne inner body should be an App (Iff applied)"
                );
            }
        }
    }

    #[test]
    fn test_idempotent_count_stable() {
        let mut env = Environment::new();
        register_linarith_helper(&mut env).expect("first registration");
        let count_after_first = env.len();
        register_linarith_helper(&mut env).expect("second registration should succeed");
        assert_eq!(
            env.len(),
            count_after_first,
            "second registration must not add any new entries"
        );
    }

    #[test]
    fn test_coexistence_with_preregistered() {
        let mut env = Environment::new();
        // Simulate pre-registering two lemmas.
        let pre_existing = &[
            ("Int.add_lt_add", ty_add_lt_add()),
            ("Int.neg_le_neg", ty_neg_le_neg()),
        ];
        for (name, ty) in pre_existing {
            env.add(Declaration::Axiom {
                name: Name::str(*name),
                univ_params: vec![],
                ty: ty.clone(),
            })
            .expect("pre-registration should succeed");
        }
        // register_linarith_helper should skip those two and add the remaining 10.
        register_linarith_helper(&mut env).expect("registration with pre-existing should succeed");
        for name in LINARITH_LEMMAS {
            assert!(
                env.contains(&Name::str(*name)),
                "lemma {} should be present after coexistence test",
                name
            );
        }
        assert_eq!(env.len(), 12, "total should be 12 after coexistence test");
    }

    #[test]
    fn test_discreteness_bridges_present() {
        let mut env = Environment::new();
        register_linarith_helper(&mut env).expect("registration should succeed");
        assert!(
            env.contains(&Name::str("Int.lt_add_one_of_le")),
            "Int.lt_add_one_of_le should be registered"
        );
        assert!(
            env.contains(&Name::str("Int.le_of_lt_add_one")),
            "Int.le_of_lt_add_one should be registered"
        );
    }

    #[test]
    fn test_scale_lemmas_present() {
        let mut env = Environment::new();
        register_linarith_helper(&mut env).expect("registration should succeed");
        assert!(env.contains(&Name::str("Int.add_le_add_left")));
        assert!(env.contains(&Name::str("Int.add_le_add_right")));
    }

    #[test]
    fn test_env_find_after_registration() {
        let mut env = Environment::new();
        register_linarith_helper(&mut env).expect("registration should succeed");
        assert!(env.find(&Name::str("Int.add_lt_add")).is_some());
        assert!(env.find(&Name::str("Int.mul_nonneg")).is_some());
        assert!(env.find(&Name::str("Int.lt_iff_le_and_ne")).is_some());
        assert!(env.find(&Name::str("Int.lt_of_le_of_ne")).is_some());
    }
}
