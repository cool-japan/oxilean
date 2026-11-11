//! Regression tests for definitional equality on OPEN terms (loose bound
//! variables under binders), exposed by Wave-3b export replay:
//!
//! 1. `subst::instantiate` must lift a substituted open argument past the
//!    binders it is inserted under (de Bruijn capture avoidance) — without
//!    this, iota/delta reduction inside `Pi` bodies rewrites wrong variables
//!    (seen as a wrong REJECTION of `Nat.brecOn.go` in `simple_add.ndjson`).
//! 2. `Nat` literal ↔ constructor bridging: `Lit 0 ≡ Nat.zero`,
//!    `Lit (n+1) ≡ Nat.succ t` when `Lit n ≡ t` (audit §5.2; seen in
//!    `Parity.isEven` via `OfNat.ofNat Nat 0 (instOfNatNat 0)`).
//! 3. K-like reduction with a Pi-bound major premise: the def-eq checker can
//!    type loose BVars from its binder stack and fire `Eq.rec` K-reduction
//!    where the Reducer's fresh `TypeChecker` cannot (seen as a wrong
//!    REJECTION of `eq_of_heq` in `Parity.isEven`).
//! 4. A checker seeded with the locals of a surrounding checker
//!    (`TypeChecker::with_fvar_types`) opens binders with free variables
//!    distinct from every seeded local (a wrong ACCEPT otherwise: the bound
//!    variable is identified with the local that carries its id).

use oxilean_kernel::Node;
use oxilean_kernel::{
    check_constant_info, init_builtin_env, instantiate, AxiomVal, BigNat, BinderInfo, ConstantInfo,
    ConstantVal, Environment, Expr, Level, Literal, Name, TypeChecker,
};

fn env_with(axioms: &[(&str, Expr)]) -> Environment {
    let mut env = Environment::new();
    init_builtin_env(&mut env).unwrap_or_else(|e| panic!("builtin env: {e}"));
    for (name, ty) in axioms {
        let ci = ConstantInfo::Axiom(AxiomVal {
            common: ConstantVal {
                name: Name::str(*name),
                level_params: vec![],
                ty: ty.clone(),
            },
            is_unsafe: false,
        });
        check_constant_info(&mut env, ci).unwrap_or_else(|e| panic!("axiom {name}: {e:?}"));
    }
    env
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

fn pi(name: &str, ty: Expr, body: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str(name),
        Node::new(ty),
        Node::new(body),
    )
}

// ---------------------------------------------------------------------------
// 1. Capture avoidance in `instantiate`.
// ---------------------------------------------------------------------------

#[test]
fn instantiate_lifts_open_arg_under_binder() {
    // body = λ y : Prop. x   (x = BVar(1) inside the lambda)
    // instantiate x := #0 (an OPEN variable of the surrounding context):
    // the argument must be lifted past the `y` binder → λ y : Prop. #1.
    let body = lam("y", Expr::Sort(Level::zero()), Expr::BVar(1));
    let result = instantiate(&body, &Expr::BVar(0));
    let expected = lam("y", Expr::Sort(Level::zero()), Expr::BVar(1));
    assert_eq!(result, expected, "open arg must not be captured by `y`");
}

#[test]
fn instantiate_closed_arg_behavior_unchanged() {
    // Closed arguments are unaffected by lifting.
    let body = lam("y", Expr::Sort(Level::zero()), Expr::BVar(1));
    let arg = Expr::Const(Name::str("Nat"), vec![]);
    let result = instantiate(&body, &arg);
    let expected = lam("y", Expr::Sort(Level::zero()), arg);
    assert_eq!(result, expected);
}

// ---------------------------------------------------------------------------
// 2. Nat literal ↔ constructor bridging.
// ---------------------------------------------------------------------------

#[test]
fn nat_literal_constructor_bridge() {
    let env = env_with(&[]);
    let mut tc = TypeChecker::new(&env);
    let lit = |n: u64| Expr::Lit(Literal::Nat(BigNat::from(n)));
    let zero = Expr::Const(Name::str("Nat.zero"), vec![]);
    let succ = |e: Expr| app(Expr::Const(Name::str("Nat.succ"), vec![]), e);

    assert!(tc.is_def_eq(&lit(0), &zero), "0 ≡ Nat.zero");
    assert!(tc.is_def_eq(&zero, &lit(0)), "Nat.zero ≡ 0 (symmetric)");
    assert!(
        tc.is_def_eq(&lit(2), &succ(succ(zero.clone()))),
        "2 ≡ Nat.succ (Nat.succ Nat.zero)"
    );
    assert!(
        tc.is_def_eq(&succ(lit(1)), &lit(2)),
        "Nat.succ 1 ≡ 2 (mixed)"
    );
    assert!(!tc.is_def_eq(&lit(1), &zero), "1 ≢ Nat.zero");
    assert!(
        !tc.is_def_eq(&lit(3), &succ(succ(zero))),
        "3 ≢ Nat.succ (Nat.succ Nat.zero)"
    );
}

#[test]
fn nat_literal_bridge_under_stuck_congruence() {
    // F 0 ≡ F Nat.zero for an opaque F : Nat → Prop (argument congruence
    // must bridge the literal).
    let nat = Expr::Const(Name::str("Nat"), vec![]);
    let env = env_with(&[("F", pi("n", nat, Expr::Sort(Level::zero())))]);
    let mut tc = TypeChecker::new(&env);
    let f = Expr::Const(Name::str("F"), vec![]);
    let t = app(f.clone(), Expr::Lit(Literal::Nat(BigNat::from(0u64))));
    let s = app(f, Expr::Const(Name::str("Nat.zero"), vec![]));
    assert!(tc.is_def_eq(&t, &s), "F 0 ≡ F Nat.zero");
}

// ---------------------------------------------------------------------------
// 3. K-reduction with a Pi-bound major premise.
// ---------------------------------------------------------------------------

#[test]
fn k_reduction_fires_on_pi_bound_major() {
    // A : Type, a : A, P : A → Prop.
    let type0 = Expr::Sort(Level::succ(Level::zero()));
    let a_ty = Expr::Const(Name::str("A"), vec![]);
    let env = env_with(&[
        ("A", type0),
        ("a", Expr::Const(Name::str("A"), vec![])),
        (
            "P",
            pi(
                "x",
                Expr::Const(Name::str("A"), vec![]),
                Expr::Sort(Level::zero()),
            ),
        ),
    ]);
    let u1 = Level::succ(Level::zero());
    let a = Expr::Const(Name::str("a"), vec![]);
    let p = Expr::Const(Name::str("P"), vec![]);
    let eq_a_a = apps(
        Expr::Const(Name::str("Eq"), vec![u1.clone()]),
        vec![a_ty.clone(), a.clone(), a.clone()],
    );
    // motive : (b : A) → Eq A a b → Sort 1  :=  λ b h, A
    let motive = lam(
        "b",
        a_ty.clone(),
        lam(
            "h",
            apps(
                Expr::Const(Name::str("Eq"), vec![u1.clone()]),
                vec![a_ty.clone(), a.clone(), Expr::BVar(0)],
            ),
            a_ty.clone(),
        ),
    );
    // @Eq.rec.{1, 1} A a motive (minor := a) (b := a) (h := #0, Pi-bound).
    let rec_app = apps(
        Expr::Const(Name::str("Eq.rec"), vec![u1.clone(), u1]),
        vec![a_ty, a.clone(), motive, a.clone(), a.clone(), Expr::BVar(0)],
    );
    // Π h : Eq A a a, P (Eq.rec … h)  ≡  Π h : Eq A a a, P a
    // requires K-reduction on the STUCK major `h` (typed from the binder).
    let t = pi("h", eq_a_a.clone(), app(p.clone(), rec_app));
    let s = pi("h", eq_a_a, app(p, a));
    let mut tc = TypeChecker::new(&env);
    assert!(
        tc.is_def_eq(&t, &s),
        "K-reduction must fire under the binder"
    );
}

// ---------------------------------------------------------------------------
// 4. A seeded checker's fresh variables are distinct from its seeded locals.
// ---------------------------------------------------------------------------

/// The first id a def-eq checker gives a binder it opens
/// (`DEF_EQ_OPENED_FVAR_BASE`, private to the kernel).
const OPENED_BASE: u64 = 1 << 62;

#[test]
fn seeded_checker_opens_binders_with_ids_distinct_from_seeded_locals() {
    use oxilean_kernel::FVarId;
    use std::collections::HashMap;

    let a = Expr::Const(Name::str("A"), vec![]);
    let env = env_with(&[
        ("A", Expr::Sort(Level::succ(Level::zero()))),
        ("g", pi("_", a.clone(), pi("_", a.clone(), a.clone()))),
    ]);
    let g = Expr::Const(Name::str("g"), vec![]);

    // The surrounding def-eq checker has opened binders `c₀ c₁ c₂ : A`; they
    // carry the first three ids of the opened range, as they do in a real
    // run. `K`-like reduction hands exactly such a context to a nested
    // checker.
    for seeded in 0..3u64 {
        let mut locals: HashMap<u64, Expr> = HashMap::new();
        locals.insert(0, a.clone());
        for k in 0..3u64 {
            locals.insert(OPENED_BASE + k, a.clone());
        }
        let c = Expr::FVar(FVarId(OPENED_BASE + seeded));

        // `fun y => g y c` and `fun y => g c y` differ: `g` is opaque. Opening
        // `y` with the id of `c` would turn both bodies into `g c c`.
        let lhs = lam(
            "y",
            a.clone(),
            apps(g.clone(), vec![Expr::BVar(0), c.clone()]),
        );
        let rhs = lam(
            "y",
            a.clone(),
            apps(g.clone(), vec![c.clone(), Expr::BVar(0)]),
        );
        let mut tc = TypeChecker::with_fvar_types(&env, &locals);
        assert!(
            !tc.is_def_eq(&lhs, &rhs),
            "`fun y => g y c` ≢ `fun y => g c y` for the seeded local c = opened #{seeded}"
        );

        // Three nested binders: whichever of them is opened first, second or
        // third, none may be identified with a seeded local.
        let nest = |body: Expr| {
            lam(
                "y1",
                a.clone(),
                lam("y2", a.clone(), lam("y3", a.clone(), body)),
            )
        };
        for bound in 0..3u32 {
            let mut tc = TypeChecker::with_fvar_types(&env, &locals);
            assert!(
                !tc.is_def_eq(&nest(c.clone()), &nest(Expr::BVar(bound))),
                "`fun y1 y2 y3 => c` ≢ `fun y1 y2 y3 => #{bound}` for c = opened #{seeded}"
            );
        }

        // Control: the same shapes with equal bodies ARE definitionally equal.
        let mut tc = TypeChecker::with_fvar_types(&env, &locals);
        assert!(tc.is_def_eq(&lhs, &lhs.clone()));
        assert!(tc.is_def_eq(&nest(c.clone()), &nest(c.clone())));
    }
}

#[test]
fn seeded_checker_keeps_its_own_locals_below_the_opened_range() {
    use oxilean_kernel::FVarId;
    use std::collections::HashMap;

    let a = Expr::Const(Name::str("A"), vec![]);
    let env = env_with(&[("A", Expr::Sort(Level::succ(Level::zero())))]);

    // Seeded with own-range locals 0 and 7 and opened-range locals
    // base+0 … base+4.
    let mut locals: HashMap<u64, Expr> = HashMap::new();
    locals.insert(0, a.clone());
    locals.insert(7, a.clone());
    for k in 0..5u64 {
        locals.insert(OPENED_BASE + k, a.clone());
    }
    let mut tc = TypeChecker::with_fvar_types(&env, &locals);
    // The checker's own fresh local follows the largest seeded OWN id, and so
    // stays out of the range its def-eq checker opens binders from.
    let fresh = tc.fresh_fvar(Name::str("z"), a.clone());
    assert_eq!(fresh, FVarId(8));
    assert!(!locals.contains_key(&fresh.0));
    // Every seeded local is still typeable.
    for id in locals.keys() {
        let ty = tc
            .infer_type(&Expr::FVar(FVarId(*id)))
            .unwrap_or_else(|e| panic!("seeded local {id} must be typeable: {e:?}"));
        assert_eq!(ty, a);
    }
}

// ---------------------------------------------------------------------------
// 5. A variable bound again after its binder closed.
// ---------------------------------------------------------------------------

/// The reducer and the def-eq checker forget a local's type when its binder
/// closes, and their caches may still hold results that mention it. Those
/// results were computed under the old binding: if a caller binds the same
/// id again — possible through the public `push_local` — they must not be
/// reused, whatever type the id had before.
///
/// `Eq.rec m h` reduces to `m` by K exactly when `h : a = a`. The same term,
/// with the same variable id, is asked about under `h : a = a`, then under
/// `h : a = b`, then under `h : a = a` again; a verdict carried over from the
/// binding before would be wrong each time, and the first carry-over would be
/// a wrong ACCEPT.
#[test]
fn a_variable_bound_again_after_its_binder_closed_gets_fresh_verdicts() {
    use oxilean_kernel::{FVarId, LocalDecl};

    let type0 = Expr::Sort(Level::succ(Level::zero()));
    let a_ty = Expr::Const(Name::str("A"), vec![]);
    let env = env_with(&[("A", type0), ("a", a_ty.clone()), ("b", a_ty.clone())]);
    let u1 = Level::succ(Level::zero());
    let a = Expr::Const(Name::str("a"), vec![]);
    let b = Expr::Const(Name::str("b"), vec![]);
    let eq_a = |rhs: Expr| {
        apps(
            Expr::Const(Name::str("Eq"), vec![u1.clone()]),
            vec![a_ty.clone(), a.clone(), rhs],
        )
    };
    // motive : (b : A) → Eq A a b → Sort 1  :=  λ b h, A
    let motive = lam(
        "b",
        a_ty.clone(),
        lam("h", eq_a(Expr::BVar(0)), a_ty.clone()),
    );
    let h = FVarId(5);
    // @Eq.rec.{1, 1} A a motive (minor := a) (b := a) (h := the local).
    let rec_app = apps(
        Expr::Const(Name::str("Eq.rec"), vec![u1.clone(), u1.clone()]),
        vec![
            a_ty.clone(),
            a.clone(),
            motive,
            a.clone(),
            a.clone(),
            Expr::FVar(h),
        ],
    );
    let declare = |ty: Expr| LocalDecl {
        fvar: h,
        name: Name::str("h"),
        ty,
        val: None,
    };

    let mut tc = TypeChecker::new(&env);
    for round in 0..3 {
        // h : a = a — K applies, and the recursor reduces to its minor.
        tc.push_local(declare(eq_a(a.clone())));
        assert!(
            tc.is_def_eq(&rec_app, &a),
            "round {round}: `Eq.rec a h ≡ a` for `h : a = a`"
        );
        assert_eq!(tc.whnf(&rec_app), a, "round {round}");
        tc.pop_local();

        // h : a = b — the same id, another hypothesis: K does not apply.
        tc.push_local(declare(eq_a(b.clone())));
        assert!(
            !tc.is_def_eq(&rec_app, &a),
            "round {round}: `Eq.rec a h ≢ a` for `h : a = b` — a verdict about the \
             closed binding of `h` was reused"
        );
        assert_ne!(tc.whnf(&rec_app), a, "round {round}");
        tc.pop_local();
    }

    // With the binder closed and the id not bound again, the major cannot be
    // typed at all: the term stays stuck (never a wrong reduction).
    assert!(!tc.is_def_eq(&rec_app, &a));
    assert_ne!(tc.whnf(&rec_app), a);
}
