//! End-to-end tests for kernel quotient support (`#QUOT`).
//!
//! These drive the *real* kernel path: quotients are installed via
//! `Environment::add_quot` (with kernel-constructed canonical types), and
//! reduction / def-eq run through `Reducer::whnf_env` / `DefEqChecker` — the
//! same machinery the `TypeChecker` uses.
//!
//! Coverage (matching the verify-gap audit §7):
//! (a) `Quot.lift f h (Quot.mk r a) ≡ f a` with hierarchical names;
//! (b) `Quot.ind m q` iota at exact arity 5 (pins the off-by-one fix);
//! (c) over-application `(Quot.lift f h (Quot.mk r a)) x` reduces to `f a x`;
//! (d) major premise needing WHNF first;
//! (e) negatives: partial application / non-mk major stay stuck;
//! (f) env hygiene: second `add_quot`, `add_quot` before `Eq`, non-canonical
//!     `ConstantInfo::Quotient` all rejected;
//! (g) full typecheck of a hand-built setoid-style quotient + lifted function.

use oxilean_kernel::Node;
use oxilean_kernel::{
    check_constant_info, init_builtin_env, BinderInfo, DefEqChecker, Environment, Expr, Level,
    Name, Reducer, TypeChecker,
};

// ---------------------------------------------------------------------------
// Small term-building helpers (hierarchical names throughout).
// ---------------------------------------------------------------------------

fn env() -> Environment {
    let mut env = Environment::new();
    match init_builtin_env(&mut env) {
        Ok(()) => env,
        Err(e) => unreachable!("builtin env must initialize: {e}"),
    }
}

fn n(s: &str) -> Name {
    Name::from_str(s)
}

fn c(name: &str, levels: Vec<Level>) -> Expr {
    Expr::Const(n(name), levels)
}

fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Node::new(f), Node::new(a))
}

fn apps(f: Expr, args: Vec<Expr>) -> Expr {
    args.into_iter().fold(f, app)
}

fn lam(name: &str, ty: Expr, body: Expr) -> Expr {
    Expr::Lam(
        BinderInfo::Default,
        Name::str(name),
        Node::new(ty),
        Node::new(body),
    )
}

fn u0() -> Level {
    Level::zero()
}

fn u1() -> Level {
    Level::succ(Level::zero())
}

/// `Sort 0` = `Prop`.
fn prop() -> Expr {
    Expr::Sort(u0())
}

/// `@Quot.mk α r a`.
fn quot_mk(alpha: Expr, r: Expr, a: Expr, lvl: Level) -> Expr {
    apps(c("Quot.mk", vec![lvl]), vec![alpha, r, a])
}

/// `@Quot.lift α r β f h q` (universe levels u, v).
#[allow(clippy::too_many_arguments)]
fn quot_lift(
    alpha: Expr,
    r: Expr,
    beta: Expr,
    f: Expr,
    h: Expr,
    q: Expr,
    lvl_u: Level,
    lvl_v: Level,
) -> Expr {
    apps(
        c("Quot.lift", vec![lvl_u, lvl_v]),
        vec![alpha, r, beta, f, h, q],
    )
}

/// `@Quot.ind α r β m q`.
#[allow(clippy::too_many_arguments)]
fn quot_ind(alpha: Expr, r: Expr, beta: Expr, m: Expr, q: Expr, lvl_u: Level) -> Expr {
    apps(c("Quot.ind", vec![lvl_u]), vec![alpha, r, beta, m, q])
}

// ---------------------------------------------------------------------------
// (0) The kernel-constructed canonical types must themselves typecheck.
// ---------------------------------------------------------------------------

#[test]
fn canonical_quot_types_are_well_sorted() {
    let env = env();
    // Each of the four primitives, having been installed by `add_quot`, has a
    // well-formed type (`TypeChecker::ensure_sort` succeeds). This is the
    // strongest single check that the de Bruijn plumbing is correct.
    for name in ["Quot", "Quot.mk", "Quot.lift", "Quot.ind"] {
        let ty = match env.get_type(&n(name)) {
            Some(t) => t.clone(),
            None => unreachable!("{name} must be installed by add_quot"),
        };
        let mut tc = TypeChecker::new(&env);
        tc.ensure_sort(&ty)
            .unwrap_or_else(|e| unreachable!("canonical type of {name} must be a sort: {e:?}"));
    }
}

#[test]
fn canonical_quot_sound_type_is_well_sorted_prop() {
    let env = env();
    // `Quot.sound` is not installed by `add_quot` (it is an ordinary axiom in
    // Lean), but the kernel owns its canonical type so replayers can verify the
    // exported axiom. The type must be a well-formed proposition in a
    // quot-initialized environment.
    let ty = oxilean_kernel::env::canonical_quot_sound_type();
    assert_eq!(
        oxilean_kernel::env::canonical_quot_sound_level_params(),
        vec![Name::str("u")]
    );
    let mut tc = TypeChecker::new(&env);
    let sort = tc
        .ensure_sort(&ty)
        .unwrap_or_else(|e| unreachable!("canonical Quot.sound type must be a sort: {e:?}"));
    assert!(
        sort.is_zero(),
        "Quot.sound must be a proposition, got Sort {sort:?}"
    );
}

// ---------------------------------------------------------------------------
// (a) Quot.lift computation rule.
// ---------------------------------------------------------------------------

#[test]
fn quot_lift_reduces_to_f_a() {
    let e = env();
    // α = Nat, r some relation const, β = Nat, f = (fun x => x), a = 7.
    let alpha = c("Nat", vec![]);
    let r = c("R", vec![]); // uninterpreted relation
    let beta = c("Nat", vec![]);
    let f = lam("x", c("Nat", vec![]), Expr::BVar(0));
    let h = c("H", vec![]); // uninterpreted proof
    let a = c("A", vec![]);
    let mk = quot_mk(alpha.clone(), r.clone(), a.clone(), u1());
    let lift = quot_lift(alpha, r, beta, f.clone(), h, mk, u1(), u1());

    let mut reducer = Reducer::new();
    let whnf = reducer.whnf_env(&lift, &e);
    // `(fun x => x) A` β-reduces to `A`.
    assert_eq!(whnf, a, "Quot.lift f h (Quot.mk r a) must reduce to f a");
}

#[test]
fn quot_lift_defeq_f_a_with_hierarchical_names() {
    let e = env();
    let alpha = c("Nat", vec![]);
    let r = c("R", vec![]);
    let beta = c("Nat", vec![]);
    let f = c("F", vec![]); // an opaque function symbol
    let h = c("H", vec![]);
    let a = c("A", vec![]);
    let mk = quot_mk(alpha.clone(), r.clone(), a.clone(), u1());
    let lift = quot_lift(alpha, r, beta, f.clone(), h, mk, u1(), u1());
    let expected = app(f, a); // f a

    let mut deq = DefEqChecker::new(&e);
    assert!(
        deq.is_def_eq(&lift, &expected),
        "Quot.lift f h (Quot.mk r a) ≡ f a on the def-eq path"
    );
}

// ---------------------------------------------------------------------------
// (b) Quot.ind iota at exact arity 5 (pins the off-by-one fix).
// ---------------------------------------------------------------------------

#[test]
fn quot_ind_reduces_at_exact_arity_five() {
    let e = env();
    let alpha = c("Nat", vec![]);
    let r = c("R", vec![]);
    let beta = c("B", vec![]); // motive
    let m = c("M", vec![]); // minor premise (opaque function)
    let a = c("A", vec![]);
    let mk = quot_mk(alpha.clone(), r.clone(), a.clone(), u1());
    // Exactly 5 args: α, r, β, m, q — this is where the old `< 6` guard bailed.
    let ind = quot_ind(alpha, r, beta, m.clone(), mk, u1());
    assert_eq!(get_app_num_args(&ind), 5, "must be exactly arity 5");
    let expected = app(m, a); // m a

    let mut reducer = Reducer::new();
    let whnf = reducer.whnf_env(&ind, &e);
    assert_eq!(
        whnf, expected,
        "Quot.ind m (Quot.mk r a) must reduce to m a"
    );
}

// ---------------------------------------------------------------------------
// (c) Over-application: args after the major premise must be re-applied.
// ---------------------------------------------------------------------------

#[test]
fn quot_lift_over_application_keeps_extra_arg() {
    let e = env();
    let alpha = c("Nat", vec![]);
    let r = c("R", vec![]);
    let beta = c("Nat", vec![]);
    let f = c("F", vec![]);
    let h = c("H", vec![]);
    let a = c("A", vec![]);
    let x = c("X", vec![]);
    let mk = quot_mk(alpha.clone(), r.clone(), a.clone(), u1());
    let lift = quot_lift(alpha, r, beta, f.clone(), h, mk, u1(), u1());
    // `(Quot.lift f h (Quot.mk r a)) x` — 7 args total.
    let over = app(lift, x.clone());
    let expected = app(app(f, a), x); // f a x

    let mut reducer = Reducer::new();
    let whnf = reducer.whnf_env(&over, &e);
    assert_eq!(
        whnf, expected,
        "over-application must re-apply the trailing arg: (f a) x"
    );
}

#[test]
fn quot_ind_over_application_keeps_extra_arg() {
    let e = env();
    let alpha = c("Nat", vec![]);
    let r = c("R", vec![]);
    let beta = c("B", vec![]);
    let m = c("M", vec![]);
    let a = c("A", vec![]);
    let x = c("X", vec![]);
    let mk = quot_mk(alpha.clone(), r.clone(), a.clone(), u1());
    let ind = quot_ind(alpha, r, beta, m.clone(), mk, u1());
    let over = app(ind, x.clone()); // 6 args
    let expected = app(app(m, a), x);

    let mut reducer = Reducer::new();
    let whnf = reducer.whnf_env(&over, &e);
    assert_eq!(
        whnf, expected,
        "Quot.ind over-application must keep the extra arg"
    );
}

// ---------------------------------------------------------------------------
// (d) Major premise needing WHNF first.
// ---------------------------------------------------------------------------

#[test]
fn quot_lift_whnfs_major_premise() {
    let e = env();
    let alpha = c("Nat", vec![]);
    let r = c("R", vec![]);
    let beta = c("Nat", vec![]);
    let f = c("F", vec![]);
    let h = c("H", vec![]);
    let a = c("A", vec![]);
    let mk = quot_mk(alpha.clone(), r.clone(), a.clone(), u1());
    // Major premise is `(fun q => q) (Quot.mk r a)` — must WHNF to `Quot.mk`.
    let id_q = app(
        lam(
            "q",
            app(app(c("Quot", vec![u1()]), alpha.clone()), r.clone()),
            Expr::BVar(0),
        ),
        mk,
    );
    let lift = quot_lift(alpha, r, beta, f.clone(), h, id_q, u1(), u1());
    let expected = app(f, a);

    let mut reducer = Reducer::new();
    let whnf = reducer.whnf_env(&lift, &e);
    assert_eq!(
        whnf, expected,
        "major premise that only reduces to Quot.mk must still fire the rule"
    );
}

// ---------------------------------------------------------------------------
// (e) Negatives: partial application / non-mk major premise stay stuck.
// ---------------------------------------------------------------------------

#[test]
fn quot_lift_partial_application_is_stuck() {
    let e = env();
    let alpha = c("Nat", vec![]);
    let r = c("R", vec![]);
    let beta = c("Nat", vec![]);
    let f = c("F", vec![]);
    let h = c("H", vec![]);
    // Only 5 args (no major premise). Must not reduce.
    let partial = apps(c("Quot.lift", vec![u1(), u1()]), vec![alpha, r, beta, f, h]);
    let mut reducer = Reducer::new();
    let whnf = reducer.whnf_env(&partial, &e);
    assert_eq!(whnf, partial, "partial Quot.lift must stay stuck");
}

#[test]
fn quot_lift_non_mk_major_is_stuck() {
    let e = env();
    let alpha = c("Nat", vec![]);
    let r = c("R", vec![]);
    let beta = c("Nat", vec![]);
    let f = c("F", vec![]);
    let h = c("H", vec![]);
    // Major premise is an opaque const `Q`, not headed by Quot.mk.
    let q = c("Q", vec![]);
    let lift = quot_lift(alpha, r, beta, f, h, q, u1(), u1());
    let mut reducer = Reducer::new();
    let whnf = reducer.whnf_env(&lift, &e);
    assert_eq!(
        whnf, lift,
        "Quot.lift with non-mk major premise must stay stuck"
    );
}

#[test]
fn quot_ind_partial_application_is_stuck() {
    let e = env();
    let alpha = c("Nat", vec![]);
    let r = c("R", vec![]);
    let beta = c("B", vec![]);
    let m = c("M", vec![]);
    // Only 4 args (no major premise).
    let partial = apps(c("Quot.ind", vec![u1()]), vec![alpha, r, beta, m]);
    let mut reducer = Reducer::new();
    let whnf = reducer.whnf_env(&partial, &e);
    assert_eq!(whnf, partial, "partial Quot.ind must stay stuck");
}

// ---------------------------------------------------------------------------
// (f) Environment hygiene.
// ---------------------------------------------------------------------------

#[test]
fn second_add_quot_is_rejected() {
    let mut env = env(); // init_builtin_env already called add_quot once
    assert!(env.quot_initialized());
    assert!(
        env.add_quot().is_err(),
        "a second add_quot must be a typed error"
    );
}

#[test]
fn add_quot_before_eq_is_rejected() {
    // Fresh env with no `Eq`: add_quot must fail its precondition.
    let mut env = Environment::new();
    assert!(!env.quot_initialized());
    let err = env.add_quot();
    assert!(err.is_err(), "add_quot before Eq must be rejected");
    assert!(!env.quot_initialized(), "flag must not be set on failure");
    assert!(!env.contains(&n("Quot")), "no partial install on failure");
}

#[test]
fn add_quot_installs_all_four() {
    let env = env();
    for name in ["Quot", "Quot.mk", "Quot.lift", "Quot.ind"] {
        assert!(env.contains(&n(name)), "{name} must be installed");
        assert!(
            env.is_quotient(&n(name)),
            "{name} must be a Quotient constant"
        );
    }
}

#[test]
fn direct_quotient_constant_addition_is_rejected() {
    use oxilean_kernel::declaration::{ConstantInfo, ConstantVal, QuotKind, QuotVal};
    // Even a *canonical-looking* Quotient added via add_constant is rejected:
    // quotients may only be installed via add_quot.
    let mut env = Environment::new();
    let qv = ConstantInfo::Quotient(QuotVal {
        common: ConstantVal {
            name: n("Quot"),
            level_params: vec![Name::str("u")],
            ty: prop(),
        },
        kind: QuotKind::Type,
    });
    assert!(
        env.add_constant(qv).is_err(),
        "add_constant must reject any Quotient entry"
    );
}

#[test]
fn noncanonical_quotient_via_check_is_rejected() {
    use oxilean_kernel::declaration::{ConstantInfo, ConstantVal, QuotKind, QuotVal};
    let mut env = env();
    // A `Quot.lift` with a bogus type (`Prop`) must be rejected by the checker
    // (defense-in-depth), independent of the add_constant guard.
    let qv = ConstantInfo::Quotient(QuotVal {
        common: ConstantVal {
            name: n("MyQuot.lift"),
            level_params: vec![Name::str("u"), Name::str("v")],
            ty: prop(),
        },
        kind: QuotKind::Lift,
    });
    assert!(
        check_constant_info(&mut env, qv).is_err(),
        "non-canonical Quot.lift type must be rejected"
    );
}

// ---------------------------------------------------------------------------
// (g) Full typecheck of a hand-built setoid-style quotient.
// ---------------------------------------------------------------------------

/// Build a small setoid-style scenario entirely through the kernel API and
/// typecheck a lifted function application.
///
/// We use `α := Nat`, an opaque relation `R : Nat → Nat → Prop`, form
/// `Q := @Quot Nat R : Type`, then check that `@Quot.mk Nat R a : Q` for a
/// concrete `a : Nat`, and that lifting `f : Nat → Nat` over `Quot.mk R a`
/// both typechecks and reduces to `f a`.
#[test]
fn setoid_style_quotient_typechecks_and_lifts() {
    use oxilean_kernel::declaration::{AxiomVal, ConstantInfo, ConstantVal};

    let mut env = env();

    // R : Nat → Nat → Prop
    let r_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("a"),
        Node::new(c("Nat", vec![])),
        Node::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("b"),
            Node::new(c("Nat", vec![])),
            Node::new(prop()),
        )),
    );
    let add_axiom = |env: &mut Environment, name: &str, ty: Expr| {
        let ci = ConstantInfo::Axiom(AxiomVal {
            common: ConstantVal {
                name: n(name),
                level_params: vec![],
                ty,
            },
            is_unsafe: false,
        });
        check_constant_info(env, ci).unwrap_or_else(|e| unreachable!("axiom {name}: {e:?}"));
    };
    add_axiom(&mut env, "R", r_ty);

    // a : Nat  (an opaque element of the carrier type)
    add_axiom(&mut env, "A", c("Nat", vec![]));
    let a = c("A", vec![]);
    let alpha = c("Nat", vec![]);
    let r = c("R", vec![]);

    // The quotient type `Q := @Quot Nat R`.
    let quot_ty = apps(c("Quot", vec![u1()]), vec![alpha.clone(), r.clone()]);
    // `@Quot.mk Nat R a` should typecheck against `Q`.
    let mk = quot_mk(alpha.clone(), r.clone(), a.clone(), u1());
    {
        let mut tc = TypeChecker::new(&env);
        let inferred = tc
            .infer_type(&mk)
            .unwrap_or_else(|e| unreachable!("Quot.mk must typecheck: {e:?}"));
        assert!(
            tc.is_def_eq(&inferred, &quot_ty),
            "@Quot.mk Nat R a : @Quot Nat R"
        );
    }

    // f : Nat → Nat (opaque), h : the soundness hypothesis (opaque).
    add_axiom(
        &mut env,
        "F",
        Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Node::new(c("Nat", vec![])),
            Node::new(c("Nat", vec![])),
        ),
    );
    // Lift f over the quotient; the *type* of the lift application is Nat, and
    // it reduces to `f a`.
    let f = c("F", vec![]);
    // h has type `∀ a b, R a b → F a = F b`; we make it opaque with that type.
    let h_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("a"),
        Node::new(c("Nat", vec![])),
        Node::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("b"),
            Node::new(c("Nat", vec![])),
            Node::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("_"),
                Node::new(apps(r.clone(), vec![Expr::BVar(1), Expr::BVar(0)])),
                Node::new(apps(
                    c("Eq", vec![u1()]),
                    vec![
                        c("Nat", vec![]),
                        app(f.clone(), Expr::BVar(2)),
                        app(f.clone(), Expr::BVar(1)),
                    ],
                )),
            )),
        )),
    );
    add_axiom(&mut env, "H", h_ty);
    let h = c("H", vec![]);

    let lift = quot_lift(alpha, r, c("Nat", vec![]), f.clone(), h, mk, u1(), u1());
    // Typecheck: the lift application has type `Nat`.
    {
        let mut tc = TypeChecker::new(&env);
        let inferred = tc
            .infer_type(&lift)
            .unwrap_or_else(|e| unreachable!("Quot.lift application must typecheck: {e:?}"));
        assert!(
            tc.is_def_eq(&inferred, &c("Nat", vec![])),
            "the lifted application has type Nat"
        );
    }
    // Reduce: it computes to `F A`.
    let mut reducer = Reducer::new();
    let whnf = reducer.whnf_env(&lift, &env);
    assert_eq!(whnf, app(f, a), "lifted application reduces to F A");
}

// ---------------------------------------------------------------------------
// Local helper (avoids depending on a crate-private util).
// ---------------------------------------------------------------------------

fn get_app_num_args(e: &Expr) -> usize {
    match e {
        Expr::App(f, _) => 1 + get_app_num_args(f),
        _ => 0,
    }
}
