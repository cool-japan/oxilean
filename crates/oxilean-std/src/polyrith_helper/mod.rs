//! # Polyrith Helper Lemma Bundle
//!
//! This module provides axiom-backed declarations for commutative ring lemmas
//! that the polyrith proof reconstruction compiler uses when building proof terms
//! for polynomial ring equality goals.
//!
//! All 15 lemmas are declared here with guards to skip duplicates if any of them
//! have already been added to the environment.
//!
//! The 15 lemmas comprise:
//! - 2 additive commutativity/associativity axioms (`Int.add_comm`, `Int.add_assoc`)
//! - 2 multiplicative commutativity/associativity axioms (`Int.mul_comm`, `Int.mul_assoc`)
//! - 2 distributivity axioms (`Int.left_distrib`, `Int.right_distrib`)
//! - 4 identity axioms (`Int.add_zero`, `Int.zero_add`, `Int.mul_one`, `Int.one_mul`)
//! - 2 absorption axioms (`Int.mul_zero`, `Int.zero_mul`)
//! - 2 additive inverse axioms (`Int.neg_add_cancel`, `Int.add_neg_cancel`)
//! - 1 negation-multiplication axiom (`Int.mul_neg`)

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

/// `@Eq Int a b` (propositional equality on Int).
///
/// De Bruijn encoding: uses `Level::succ(Level::zero())` for the universe
/// argument of `Eq`, since `Eq` lives in `Type 1` when applied to `Int : Type 1`.
fn int_eq_expr(a: Expr, b: Expr) -> Expr {
    let eq_const = Expr::Const(Name::str("Eq"), vec![Level::succ(Level::zero())]);
    app(app(app(eq_const, int_const()), a), b)
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

/// `Int.add_comm : ∀ (a b : Int), @Eq Int (Int.add a b) (Int.add b a)`
///
/// De Bruijn trace:
/// - `forall2_int` binds: after `a`: a=bvar(0); after `b`: b=bvar(0), a=bvar(1)
/// - body: `@Eq Int (Int.add a b) (Int.add b a)` → a=bvar(1), b=bvar(0)
fn ty_add_comm() -> Expr {
    forall2_int(int_eq_expr(
        app2(cst("Int.add"), bvar(1), bvar(0)),
        app2(cst("Int.add"), bvar(0), bvar(1)),
    ))
}

/// `Int.add_assoc : ∀ (a b c : Int), @Eq Int (Int.add (Int.add a b) c) (Int.add a (Int.add b c))`
///
/// De Bruijn trace:
/// - `forall3_int` binds: after `a`: a=bvar(0); after `b`: b=bvar(0),a=bvar(1);
///   after `c`: c=bvar(0), b=bvar(1), a=bvar(2)
/// - body: `@Eq Int ((a+b)+c) (a+(b+c))` → a=bvar(2), b=bvar(1), c=bvar(0)
fn ty_add_assoc() -> Expr {
    forall3_int(int_eq_expr(
        app2(
            cst("Int.add"),
            app2(cst("Int.add"), bvar(2), bvar(1)),
            bvar(0),
        ),
        app2(
            cst("Int.add"),
            bvar(2),
            app2(cst("Int.add"), bvar(1), bvar(0)),
        ),
    ))
}

/// `Int.mul_comm : ∀ (a b : Int), @Eq Int (Int.mul a b) (Int.mul b a)`
fn ty_mul_comm() -> Expr {
    forall2_int(int_eq_expr(
        app2(cst("Int.mul"), bvar(1), bvar(0)),
        app2(cst("Int.mul"), bvar(0), bvar(1)),
    ))
}

/// `Int.mul_assoc : ∀ (a b c : Int), @Eq Int (Int.mul (Int.mul a b) c) (Int.mul a (Int.mul b c))`
fn ty_mul_assoc() -> Expr {
    forall3_int(int_eq_expr(
        app2(
            cst("Int.mul"),
            app2(cst("Int.mul"), bvar(2), bvar(1)),
            bvar(0),
        ),
        app2(
            cst("Int.mul"),
            bvar(2),
            app2(cst("Int.mul"), bvar(1), bvar(0)),
        ),
    ))
}

/// `Int.left_distrib : ∀ (a b c : Int), @Eq Int (Int.mul a (Int.add b c)) (Int.add (Int.mul a b) (Int.mul a c))`
fn ty_left_distrib() -> Expr {
    forall3_int(int_eq_expr(
        app2(
            cst("Int.mul"),
            bvar(2),
            app2(cst("Int.add"), bvar(1), bvar(0)),
        ),
        app2(
            cst("Int.add"),
            app2(cst("Int.mul"), bvar(2), bvar(1)),
            app2(cst("Int.mul"), bvar(2), bvar(0)),
        ),
    ))
}

/// `Int.right_distrib : ∀ (a b c : Int), @Eq Int (Int.mul (Int.add a b) c) (Int.add (Int.mul a c) (Int.mul b c))`
fn ty_right_distrib() -> Expr {
    forall3_int(int_eq_expr(
        app2(
            cst("Int.mul"),
            app2(cst("Int.add"), bvar(2), bvar(1)),
            bvar(0),
        ),
        app2(
            cst("Int.add"),
            app2(cst("Int.mul"), bvar(2), bvar(0)),
            app2(cst("Int.mul"), bvar(1), bvar(0)),
        ),
    ))
}

/// `Int.add_zero : ∀ (a : Int), @Eq Int (Int.add a (Int.ofNat 0)) a`
///
/// De Bruijn trace:
/// - after `a` binder: a = bvar(0)
/// - body: `@Eq Int (Int.add a 0) a` → a=bvar(0), 0=int_zero()
fn ty_add_zero() -> Expr {
    forall1_int(int_eq_expr(
        app2(cst("Int.add"), bvar(0), int_zero()),
        bvar(0),
    ))
}

/// `Int.zero_add : ∀ (a : Int), @Eq Int (Int.add (Int.ofNat 0) a) a`
fn ty_zero_add() -> Expr {
    forall1_int(int_eq_expr(
        app2(cst("Int.add"), int_zero(), bvar(0)),
        bvar(0),
    ))
}

/// `Int.mul_one : ∀ (a : Int), @Eq Int (Int.mul a (Int.ofNat 1)) a`
fn ty_mul_one() -> Expr {
    forall1_int(int_eq_expr(
        app2(cst("Int.mul"), bvar(0), int_one()),
        bvar(0),
    ))
}

/// `Int.one_mul : ∀ (a : Int), @Eq Int (Int.mul (Int.ofNat 1) a) a`
fn ty_one_mul() -> Expr {
    forall1_int(int_eq_expr(
        app2(cst("Int.mul"), int_one(), bvar(0)),
        bvar(0),
    ))
}

/// `Int.mul_zero : ∀ (a : Int), @Eq Int (Int.mul a (Int.ofNat 0)) (Int.ofNat 0)`
fn ty_mul_zero() -> Expr {
    forall1_int(int_eq_expr(
        app2(cst("Int.mul"), bvar(0), int_zero()),
        int_zero(),
    ))
}

/// `Int.zero_mul : ∀ (a : Int), @Eq Int (Int.mul (Int.ofNat 0) a) (Int.ofNat 0)`
fn ty_zero_mul() -> Expr {
    forall1_int(int_eq_expr(
        app2(cst("Int.mul"), int_zero(), bvar(0)),
        int_zero(),
    ))
}

/// `Int.neg_add_cancel : ∀ (a : Int), @Eq Int (Int.add (Int.neg a) a) (Int.ofNat 0)`
///
/// De Bruijn trace:
/// - after `a` binder: a = bvar(0)
/// - body: `@Eq Int (Int.add (Int.neg a) a) 0` → a=bvar(0)
fn ty_neg_add_cancel() -> Expr {
    forall1_int(int_eq_expr(
        app2(cst("Int.add"), app(cst("Int.neg"), bvar(0)), bvar(0)),
        int_zero(),
    ))
}

/// `Int.add_neg_cancel : ∀ (a : Int), @Eq Int (Int.add a (Int.neg a)) (Int.ofNat 0)`
fn ty_add_neg_cancel() -> Expr {
    forall1_int(int_eq_expr(
        app2(cst("Int.add"), bvar(0), app(cst("Int.neg"), bvar(0))),
        int_zero(),
    ))
}

/// `Int.mul_neg : ∀ (a b : Int), @Eq Int (Int.mul a (Int.neg b)) (Int.neg (Int.mul a b))`
///
/// De Bruijn trace:
/// - `forall2_int` binds: after `a`: a=bvar(0); after `b`: b=bvar(0), a=bvar(1)
/// - body: `@Eq Int (a * (-b)) (-(a * b))` → a=bvar(1), b=bvar(0)
fn ty_mul_neg() -> Expr {
    forall2_int(int_eq_expr(
        app2(cst("Int.mul"), bvar(1), app(cst("Int.neg"), bvar(0))),
        app(cst("Int.neg"), app2(cst("Int.mul"), bvar(1), bvar(0))),
    ))
}

// ── Public registration function ─────────────────────────────────────────────

/// Register the polyrith helper lemma bundle into `env`.
///
/// Adds axiom-backed declarations for the 15 commutative ring axioms that the
/// polyrith proof reconstruction compiler uses when building proof terms for
/// polynomial ring equality goals. Each lemma is guarded: if already present
/// in `env`, it is silently skipped. This makes the function idempotent.
///
/// The bundle includes:
/// - 2 additive axioms: `Int.add_comm`, `Int.add_assoc`
/// - 2 multiplicative axioms: `Int.mul_comm`, `Int.mul_assoc`
/// - 2 distributivity axioms: `Int.left_distrib`, `Int.right_distrib`
/// - 4 identity axioms: `Int.add_zero`, `Int.zero_add`, `Int.mul_one`, `Int.one_mul`
/// - 2 absorption axioms: `Int.mul_zero`, `Int.zero_mul`
/// - 2 additive inverse axioms: `Int.neg_add_cancel`, `Int.add_neg_cancel`
/// - 1 negation-multiplication axiom: `Int.mul_neg`
///
/// # Errors
///
/// Returns `EnvError::DuplicateDeclaration` if an unexpected name collision
/// occurs (should not happen in normal usage due to the `contains` guards).
pub fn register_polyrith_helper(env: &mut Environment) -> Result<(), EnvError> {
    let lemmas: &[(&str, Expr)] = &[
        ("Int.add_comm", ty_add_comm()),
        ("Int.add_assoc", ty_add_assoc()),
        ("Int.mul_comm", ty_mul_comm()),
        ("Int.mul_assoc", ty_mul_assoc()),
        ("Int.left_distrib", ty_left_distrib()),
        ("Int.right_distrib", ty_right_distrib()),
        ("Int.add_zero", ty_add_zero()),
        ("Int.zero_add", ty_zero_add()),
        ("Int.mul_one", ty_mul_one()),
        ("Int.one_mul", ty_one_mul()),
        ("Int.mul_zero", ty_mul_zero()),
        ("Int.zero_mul", ty_zero_mul()),
        ("Int.neg_add_cancel", ty_neg_add_cancel()),
        ("Int.add_neg_cancel", ty_add_neg_cancel()),
        ("Int.mul_neg", ty_mul_neg()),
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

    const POLYRITH_LEMMAS: &[&str] = &[
        "Int.add_comm",
        "Int.add_assoc",
        "Int.mul_comm",
        "Int.mul_assoc",
        "Int.left_distrib",
        "Int.right_distrib",
        "Int.add_zero",
        "Int.zero_add",
        "Int.mul_one",
        "Int.one_mul",
        "Int.mul_zero",
        "Int.zero_mul",
        "Int.neg_add_cancel",
        "Int.add_neg_cancel",
        "Int.mul_neg",
    ];

    #[test]
    fn test_register_polyrith_helper_success() {
        let mut env = Environment::new();
        register_polyrith_helper(&mut env).expect("registration should succeed");
    }

    #[test]
    fn test_all_polyrith_lemmas_present() {
        let mut env = Environment::new();
        register_polyrith_helper(&mut env).expect("registration should succeed");

        for name in POLYRITH_LEMMAS {
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
        register_polyrith_helper(&mut env).expect("first registration");
        // Second call should skip all due to contains() guards.
        register_polyrith_helper(&mut env).expect("second registration should succeed");
        // All lemmas still present.
        for name in POLYRITH_LEMMAS {
            assert!(
                env.contains(&Name::str(*name)),
                "lemma {} should still be registered after second call",
                name
            );
        }
    }

    #[test]
    fn test_polyrith_lemma_count_exact() {
        let mut env = Environment::new();
        register_polyrith_helper(&mut env).expect("registration should succeed");
        assert_eq!(
            env.len(),
            POLYRITH_LEMMAS.len(),
            "exactly {} polyrith lemmas expected",
            POLYRITH_LEMMAS.len()
        );
    }

    #[test]
    fn test_add_comm_is_pi() {
        let ty = ty_add_comm();
        assert!(
            matches!(ty, Expr::Pi(_, _, _, _)),
            "add_comm type should be a Pi"
        );
    }

    #[test]
    fn test_mul_comm_is_pi() {
        let ty = ty_mul_comm();
        assert!(
            matches!(ty, Expr::Pi(_, _, _, _)),
            "mul_comm type should be a Pi"
        );
    }

    #[test]
    fn test_add_assoc_is_pi() {
        let ty = ty_add_assoc();
        assert!(
            matches!(ty, Expr::Pi(_, _, _, _)),
            "add_assoc type should be a Pi"
        );
    }

    #[test]
    fn test_mul_assoc_is_pi() {
        let ty = ty_mul_assoc();
        assert!(
            matches!(ty, Expr::Pi(_, _, _, _)),
            "mul_assoc type should be a Pi"
        );
    }

    #[test]
    fn test_left_distrib_is_pi() {
        let ty = ty_left_distrib();
        assert!(
            matches!(ty, Expr::Pi(_, _, _, _)),
            "left_distrib type should be a Pi"
        );
    }

    #[test]
    fn test_right_distrib_is_pi() {
        let ty = ty_right_distrib();
        assert!(
            matches!(ty, Expr::Pi(_, _, _, _)),
            "right_distrib type should be a Pi"
        );
    }

    #[test]
    fn test_add_zero_is_pi() {
        let ty = ty_add_zero();
        assert!(
            matches!(ty, Expr::Pi(_, _, _, _)),
            "add_zero type should be a Pi"
        );
    }

    #[test]
    fn test_mul_one_is_pi() {
        let ty = ty_mul_one();
        assert!(
            matches!(ty, Expr::Pi(_, _, _, _)),
            "mul_one type should be a Pi"
        );
    }

    #[test]
    fn test_neg_add_cancel_registered() {
        let mut env = Environment::new();
        register_polyrith_helper(&mut env).expect("registration should succeed");
        assert!(
            env.contains(&Name::str("Int.neg_add_cancel")),
            "Int.neg_add_cancel should be registered"
        );
    }

    #[test]
    fn test_add_neg_cancel_registered() {
        let mut env = Environment::new();
        register_polyrith_helper(&mut env).expect("registration should succeed");
        assert!(
            env.contains(&Name::str("Int.add_neg_cancel")),
            "Int.add_neg_cancel should be registered"
        );
    }

    #[test]
    fn test_mul_neg_registered() {
        let mut env = Environment::new();
        register_polyrith_helper(&mut env).expect("registration should succeed");
        assert!(
            env.contains(&Name::str("Int.mul_neg")),
            "Int.mul_neg should be registered"
        );
    }

    #[test]
    fn test_coexistence_with_preregistered_lemmas() {
        let mut env = Environment::new();
        // Simulate a pre-registered lemma (e.g., Int.add_comm might be added by omega).
        env.add(Declaration::Axiom {
            name: Name::str("Int.add_comm"),
            univ_params: vec![],
            ty: ty_add_comm(),
        })
        .expect("pre-registration should succeed");
        // register_polyrith_helper should skip Int.add_comm and add the remaining 14.
        register_polyrith_helper(&mut env).expect("registration with pre-existing should succeed");
        for name in POLYRITH_LEMMAS {
            assert!(
                env.contains(&Name::str(*name)),
                "lemma {} should be present after coexistence test",
                name
            );
        }
    }

    #[test]
    fn test_env_find_after_registration() {
        let mut env = Environment::new();
        register_polyrith_helper(&mut env).expect("registration should succeed");
        assert!(env.find(&Name::str("Int.add_comm")).is_some());
        assert!(env.find(&Name::str("Int.mul_assoc")).is_some());
        assert!(env.find(&Name::str("Int.left_distrib")).is_some());
        assert!(env.find(&Name::str("Int.mul_neg")).is_some());
    }

    #[test]
    fn test_idempotent_count_stays_15() {
        let mut env = Environment::new();
        register_polyrith_helper(&mut env).expect("first registration");
        register_polyrith_helper(&mut env).expect("second registration");
        assert_eq!(env.len(), 15, "expected exactly 15 polyrith lemmas");
    }
}
