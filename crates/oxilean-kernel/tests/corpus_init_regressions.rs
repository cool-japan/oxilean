//! Regression tests for the three kernel completeness bugs exposed by the
//! first Init.ndjson (Lean core) corpus replay in Wave 3b:
//!
//! 1. **Literal acceleration must fire before head delta-unfold.** Real
//!    exports declare `Nat.ble`/`Nat.add`/… as ordinary definitions
//!    (structural recursion over `Nat`). `whnf` used to WHNF the application
//!    head first, delta-unfolding the operator and bypassing the Nat/String
//!    literal extension entirely — sending e.g. `Nat.ble 55297 4294967296`
//!    into ~55K layers of unary `Nat.rec` (stack overflow at the default
//!    8 MiB stack, multi-GiB `Nat.below` towers, then a stuck term and a
//!    wrong REJECTION of Lean core's `isValidChar_UInt32`). The Lean 4
//!    kernel tries the extension first; now we do too.
//! 2. **Bool-valued results must be spelled for the env.** The comparison
//!    ops return `Bool.true`/`Bool.false`; the flat single-atom spelling the
//!    builtin env uses does not resolve in an env replayed from lean4export
//!    (hierarchical names), leaving `Bool.rec` stuck on the folded result
//!    (wrong REJECTION of `noConfusion_of_Nat`).
//! 3. **One-sided eta expansion must be able to type FVars.** The def-eq
//!    checker's `quick_infer_type` had no `FVar` arm, so a Pi-bound `f`
//!    could not eta-expand against `fun x => … f … x` (wrong REJECTION of
//!    Lean core's `funext`, cascading into 130+ downstream rejections).
//! 4. **Binders must be opened with typed locals; K-like iota must see the
//!    local context during WHNF** (`Std.IterStep.noConfusion`, Init decl
//!    #1975 — the first Wave-4 frontier). Lean ≥ 4.32 builds `noConfusion`
//!    from `ctorIdx`/`ctorElimType`/`ctorElim`; checking it needs
//!    `(Eq.rec … (Eq.symm … : 0 = ctorIdx (yield it out)) …).PULift.0 it out`
//!    to reduce, where the `Eq.rec` K-major mentions the Pi-bound
//!    `it`/`out` and the redex sits *under a `Proj`*. With bodies compared
//!    as loose BVars the major is untypeable at any depth, and a top-level
//!    K rescue can never reach a buried redex — so the def-eq checker now
//!    opens binders with typed free variables (Lean's `isDefEqBinding`) and
//!    the `Reducer` mirrors the local context for `to_ctor_when_k`.

use oxilean_kernel::Node;
use oxilean_kernel::{
    add_inductive_family, check_constant_info, init_builtin_env, AxiomVal, BigNat, BinderInfo,
    ConstantInfo, ConstantVal, DefinitionSafety, DefinitionVal, Environment, Expr, InductiveSpec,
    Level, Literal, Name, Reducer, ReducibilityHint, TypeChecker,
};

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

fn lit(n: u64) -> Expr {
    Expr::Lit(Literal::Nat(BigNat::from(n)))
}

fn cnst(name: Name) -> Expr {
    Expr::Const(name, vec![])
}

fn axiom(env: &mut Environment, name: Name, ty: Expr) {
    let ci = ConstantInfo::Axiom(AxiomVal {
        common: ConstantVal {
            name: name.clone(),
            level_params: vec![],
            ty,
        },
        is_unsafe: false,
    });
    check_constant_info(env, ci).unwrap_or_else(|e| panic!("axiom {name}: {e:?}"));
}

/// A minimal environment in the HIERARCHICAL naming style of a lean4export
/// replay: `Nat`, `Bool`, `Bool.true`, `Bool.false` (as stand-in axioms) and
/// a *defined* `Nat.ble` whose body is deliberately NOT the real structural
/// recursion (a constant `Bool.false`), so any observation of `Bool.true`
/// proves the literal extension fired instead of the definition unfolding.
fn hierarchical_env_with_defined_ble() -> Environment {
    let mut env = Environment::new();
    let nat = Name::str("Nat");
    let bool_n = Name::str("Bool");
    let bool_true = Name::str("Bool").append_str("true");
    let bool_false = Name::str("Bool").append_str("false");
    let nat_ble = Name::str("Nat").append_str("ble");
    axiom(
        &mut env,
        nat.clone(),
        Expr::Sort(Level::succ(Level::zero())),
    );
    axiom(
        &mut env,
        bool_n.clone(),
        Expr::Sort(Level::succ(Level::zero())),
    );
    axiom(&mut env, bool_true, cnst(bool_n.clone()));
    axiom(&mut env, bool_false.clone(), cnst(bool_n.clone()));
    let ble_ty = pi(
        "a",
        cnst(nat.clone()),
        pi("b", cnst(nat.clone()), cnst(bool_n.clone())),
    );
    let ble_val = lam(
        "a",
        cnst(nat.clone()),
        lam("b", cnst(nat), cnst(bool_false)),
    );
    let ci = ConstantInfo::Definition(DefinitionVal {
        common: ConstantVal {
            name: nat_ble.clone(),
            level_params: vec![],
            ty: ble_ty,
        },
        value: ble_val,
        hints: ReducibilityHint::Regular(1),
        safety: DefinitionSafety::Safe,
        all: vec![nat_ble],
    });
    check_constant_info(&mut env, ci).unwrap_or_else(|e| panic!("Nat.ble def: {e:?}"));
    env
}

// ---------------------------------------------------------------------------
// 1 + 2. Acceleration ordering and result spelling.
// ---------------------------------------------------------------------------

#[test]
fn lit_acceleration_fires_before_definition_unfold() {
    // `Nat.ble 2 3` where `Nat.ble` is a DEFINED constant returning
    // `Bool.false`: the extension must decide `true` before the definition
    // is unfolded — exactly what the Lean 4 kernel does. (The extension's
    // semantics are the real `Nat.ble` semantics; a mismatching definition
    // in the env cannot override the kernel extension there either.)
    let env = hierarchical_env_with_defined_ble();
    let ble = cnst(Name::str("Nat").append_str("ble"));
    let e = apps(ble, vec![lit(2), lit(3)]);
    let result = Reducer::new().whnf_env(&e, &env);
    let Expr::Const(name, _) = &result else {
        panic!("Nat.ble 2 3 must fold to a Bool constant, got {result:?}");
    };
    assert_eq!(
        name.to_string(),
        "Bool.true",
        "literal acceleration must fire before the head unfolds"
    );
}

#[test]
fn folded_bool_result_uses_env_spelling() {
    // In the hierarchical env the folded result must be the hierarchical
    // `Bool.true` (`Bool` ++ `true`), NOT the flat single-atom `"Bool.true"`,
    // so `Bool.rec`-style iota downstream can resolve the constructor.
    let env = hierarchical_env_with_defined_ble();
    let ble = cnst(Name::str("Nat").append_str("ble"));
    let e = apps(ble, vec![lit(0), lit(0)]);
    let result = Reducer::new().whnf_env(&e, &env);
    assert_eq!(
        result,
        cnst(Name::str("Bool").append_str("true")),
        "folded Bool result must use the env's hierarchical spelling"
    );
}

#[test]
fn lit_acceleration_declines_on_open_args() {
    // `Nat.ble a b` with non-literal arguments must NOT be decided by the
    // extension: the definition unfolds as before (here to `Bool.false`).
    let env = hierarchical_env_with_defined_ble();
    let nat = cnst(Name::str("Nat"));
    let ble = cnst(Name::str("Nat").append_str("ble"));
    // fun a b : Nat => Nat.ble a b — whnf of the applied form with opaque
    // axiom args.
    axiom_pair_env_check(&env, &nat, &ble);
}

fn axiom_pair_env_check(env: &Environment, _nat: &Expr, ble: &Expr) {
    let mut env = env.clone();
    let nat_name = Name::str("Nat");
    axiom(&mut env, Name::str("someA"), cnst(nat_name.clone()));
    axiom(&mut env, Name::str("someB"), cnst(nat_name));
    let e = apps(
        ble.clone(),
        vec![cnst(Name::str("someA")), cnst(Name::str("someB"))],
    );
    let result = Reducer::new().whnf_env(&e, &env);
    assert_eq!(
        result,
        cnst(Name::str("Bool").append_str("false")),
        "non-literal args must fall through to the definition"
    );
}

// ---------------------------------------------------------------------------
// 3. FVar typing for one-sided eta (the funext shape).
// ---------------------------------------------------------------------------

#[test]
fn pi_bound_function_eta_expands_against_lambda() {
    // f : Nat → Nat (an opened FVar) must be def-eq to `fun x : Nat => f x`.
    // Before the fix `quick_infer_type` could not type the FVar, so
    // `eta_expand_one` failed and the comparison was wrongly rejected —
    // the exact shape of Lean core's `funext` proof.
    let mut env = Environment::new();
    init_builtin_env(&mut env).unwrap_or_else(|e| panic!("builtin env: {e}"));
    let nat = cnst(Name::str("Nat"));
    let f_ty = pi("x", nat.clone(), nat.clone());
    let mut tc = TypeChecker::new(&env);
    let f = tc.fresh_fvar(Name::str("f"), f_ty);
    let f_expr = Expr::FVar(f);
    let eta_expanded = lam("x", nat, app(f_expr.clone(), Expr::BVar(0)));
    assert!(
        tc.is_def_eq(&f_expr, &eta_expanded),
        "f must be def-eq to fun x => f x (one-sided eta with an FVar)"
    );
    assert!(
        tc.is_def_eq(&eta_expanded, &f_expr),
        "eta must also fire with the lambda on the left"
    );
}

// ---------------------------------------------------------------------------
// 4. K-like iota buried under a Proj, with a binder-bound major
//    (the `Std.IterStep.noConfusion` shape, Init decl #1975).
// ---------------------------------------------------------------------------

/// Builtin env + a one-field structure `Box : Type → Type` (the `PULift`
/// stand-in) + the axioms of the distilled `noConfusion` shape:
/// `T`, `R : Type`, `f : T → R`, `bf bf2 : T → R`, `Q : R → Prop`, and a
/// stuck-but-typeable equality proof `g : ∀ x : T, Eq R (f x) (f x)`.
fn noconfusion_shape_env() -> Environment {
    let mut env = Environment::new();
    init_builtin_env(&mut env).unwrap_or_else(|e| panic!("builtin env: {e}"));
    let type0 = Expr::Sort(Level::succ(Level::zero()));
    // Box (α : Type) : Type, ctor Box.mk (α : Type) (v : α) : Box α.
    let box_ty = pi("α", type0.clone(), type0.clone());
    let mk_ty = pi(
        "α",
        type0.clone(),
        pi(
            "v",
            Expr::BVar(0),
            app(cnst(Name::str("Box")), Expr::BVar(1)),
        ),
    );
    let spec = InductiveSpec::new(Name::str("Box"), box_ty, vec![(Name::str("Box.mk"), mk_ty)]);
    add_inductive_family(&mut env, vec![], 1, vec![spec])
        .unwrap_or_else(|e| panic!("Box family: {e:?}"));
    let t_ty = cnst(Name::str("T"));
    let r_ty = cnst(Name::str("R"));
    axiom(&mut env, Name::str("T"), type0.clone());
    axiom(&mut env, Name::str("R"), type0);
    let fn_ty = pi("x", t_ty.clone(), r_ty.clone());
    axiom(&mut env, Name::str("f"), fn_ty.clone());
    axiom(&mut env, Name::str("bf"), fn_ty.clone());
    axiom(&mut env, Name::str("bf2"), fn_ty);
    axiom(
        &mut env,
        Name::str("Q"),
        pi("r", r_ty.clone(), Expr::Sort(Level::zero())),
    );
    // g : ∀ x : T, Eq R (f x) (f x) — a stuck (non-refl) proof whose type
    // mentions the bound variable, like `Eq.symm … : 0 = ctorIdx (yield it out)`.
    let fx = |x: Expr| app(cnst(Name::str("f")), x);
    let g_ty = pi(
        "x",
        t_ty,
        apps(
            Expr::Const(Name::str("Eq"), vec![Level::succ(Level::zero())]),
            vec![r_ty, fx(Expr::BVar(0)), fx(Expr::BVar(0))],
        ),
    );
    axiom(&mut env, Name::str("g"), g_ty);
    env
}

/// `(Eq.rec R (f x) (fun b h => Box (T → R)) (Box.mk (T → R) bf) (f x) (g x))`
/// — a K-reducible `Eq.rec` whose major `g x` mentions `x`.
fn buried_k_redex(x: Expr) -> Expr {
    let u1 = Level::succ(Level::zero());
    let r_ty = cnst(Name::str("R"));
    let fn_ty = pi("x", cnst(Name::str("T")), r_ty.clone());
    let fx = app(cnst(Name::str("f")), x.clone());
    let box_fn = app(cnst(Name::str("Box")), fn_ty.clone());
    let motive = lam(
        "b",
        r_ty.clone(),
        lam(
            "h",
            apps(
                Expr::Const(Name::str("Eq"), vec![u1.clone()]),
                vec![r_ty.clone(), fx.clone(), Expr::BVar(0)],
            ),
            box_fn,
        ),
    );
    let minor = apps(
        cnst(Name::str("Box.mk")),
        vec![fn_ty, cnst(Name::str("bf"))],
    );
    apps(
        Expr::Const(Name::str("Eq.rec"), vec![u1.clone(), u1]),
        vec![
            r_ty,
            fx.clone(),
            motive,
            minor,
            fx,
            app(cnst(Name::str("g")), x),
        ],
    )
}

#[test]
fn k_reduction_fires_under_proj_with_pi_bound_major() {
    // Π x : T, Q ((Eq.rec … (g x)).Box.0 x)  ≡  Π x : T, Q (bf x)
    //
    // The lhs body's head is `App(Proj(Box, 0, <stuck Eq.rec>), x)` — the
    // K-redex is *inside* the Proj, and its major `g x` mentions the
    // Pi-bound `x`. Before the fix the redex was unreachable (whnf could
    // not type the major; the def-eq K rescue only inspected the top-level
    // head) and the comparison was wrongly rejected — the exact shape that
    // rejected Lean core's `Std.IterStep.noConfusion` and its 171
    // dependents.
    let env = noconfusion_shape_env();
    let q = |e: Expr| app(cnst(Name::str("Q")), e);
    let t = pi(
        "x",
        cnst(Name::str("T")),
        q(app(
            Expr::Proj(
                Name::str("Box"),
                0,
                Node::new(buried_k_redex(Expr::BVar(0))),
            ),
            Expr::BVar(0),
        )),
    );
    let s = pi(
        "x",
        cnst(Name::str("T")),
        q(app(cnst(Name::str("bf")), Expr::BVar(0))),
    );
    let mut tc = TypeChecker::new(&env);
    assert!(
        tc.is_def_eq(&t, &s),
        "K-reduction must fire inside a Proj with a Pi-bound major"
    );
    // Soundness control: a different field must NOT become equal.
    let s_bad = pi(
        "x",
        cnst(Name::str("T")),
        q(app(cnst(Name::str("bf2")), Expr::BVar(0))),
    );
    let mut tc2 = TypeChecker::new(&env);
    assert!(
        !tc2.is_def_eq(&t, &s_bad),
        "projecting bf must not equal bf2"
    );
}

#[test]
fn reducer_k_path_types_fvar_majors_via_mirrored_locals() {
    // The Reducer-level half of the fix: `whnf` of the buried redex with an
    // FVar-typed major must reduce all the way to `bf` (K fires because the
    // TypeChecker mirrors its local context into the Reducer, which seeds
    // `to_ctor_when_k`).
    let env = noconfusion_shape_env();
    let mut tc = TypeChecker::new(&env);
    let x = Expr::FVar(tc.fresh_fvar(Name::str("x"), cnst(Name::str("T"))));
    let projected = Expr::Proj(Name::str("Box"), 0, Node::new(buried_k_redex(x)));
    let reduced = tc.whnf(&projected);
    assert_eq!(
        reduced,
        cnst(Name::str("bf")),
        "whnf must K-reduce the Eq.rec under the Proj and project the field"
    );
}

#[test]
fn distinct_fvars_do_not_become_equal_via_eta() {
    // Soundness control: `f` vs `fun x => g x` with g a DIFFERENT fvar must
    // still be rejected.
    let mut env = Environment::new();
    init_builtin_env(&mut env).unwrap_or_else(|e| panic!("builtin env: {e}"));
    let nat = cnst(Name::str("Nat"));
    let f_ty = pi("x", nat.clone(), nat.clone());
    let mut tc = TypeChecker::new(&env);
    let f = tc.fresh_fvar(Name::str("f"), f_ty.clone());
    let g = tc.fresh_fvar(Name::str("g"), f_ty);
    let eta_g = lam("x", nat, app(Expr::FVar(g), Expr::BVar(0)));
    assert!(
        !tc.is_def_eq(&Expr::FVar(f), &eta_g),
        "distinct functions must stay distinct"
    );
}

// ---------------------------------------------------------------------------
// 6. Structure-eta of stuck recursor majors (Lean's `toCtorWhenStruct`).
//
// Init corpus root: `Nat.Linear.Poly.denote_reverse` (and
// `Nat.Linear.ExprCnstr.denote_toNormPoly`, + 37 cascading) were wrongly
// REJECTED because `Poly.denote.match_1` performs the nested pair match
// `| (k, v) :: p => …` as an inner `Prod.casesOn` whose major is the
// Pi/Lam-bound HEAD of the list — a free variable after the def-eq checker
// opens binders. Structure eta is definitional in Lean 4, so the kernel must
// rewrite the stuck major `p` to `Prod.mk _ _ p.0 p.1` and let iota fire.
// ---------------------------------------------------------------------------

/// Env with a parametric structure `Pair (α : Type) : Type`,
/// ctor `Pair.mk (α : Type) (a b : α) : Pair α`, and a Prop-sorted control
/// structure `PP : Prop`, ctor `PP.mk (h₁ h₂ : Q) : PP`.
fn pair_env() -> Environment {
    let mut env = Environment::new();
    init_builtin_env(&mut env).unwrap_or_else(|e| panic!("builtin env: {e}"));
    let type0 = Expr::Sort(Level::succ(Level::zero()));
    let pair = Name::str("Pair");
    let pair_ty = pi("α", type0.clone(), type0.clone());
    let mk_ty = pi(
        "α",
        type0.clone(),
        pi(
            "a",
            Expr::BVar(0),
            pi("b", Expr::BVar(1), app(cnst(pair.clone()), Expr::BVar(2))),
        ),
    );
    let spec = InductiveSpec::new(pair.clone(), pair_ty, vec![(pair.append_str("mk"), mk_ty)]);
    add_inductive_family(&mut env, vec![], 1, vec![spec])
        .unwrap_or_else(|e| panic!("Pair family: {e:?}"));
    // Prop control: PP : Prop with two proof fields of axiom Q : Prop.
    axiom(&mut env, Name::str("Q"), Expr::Sort(Level::zero()));
    let pp = Name::str("PP");
    let q = cnst(Name::str("Q"));
    let pp_mk_ty = pi("h1", q.clone(), pi("h2", q, cnst(pp.clone())));
    let pp_spec = InductiveSpec::new(
        pp.clone(),
        Expr::Sort(Level::zero()),
        vec![(pp.append_str("mk"), pp_mk_ty)],
    );
    add_inductive_family(&mut env, vec![], 0, vec![pp_spec])
        .unwrap_or_else(|e| panic!("PP family: {e:?}"));
    env
}

/// `Pair.rec.{1} Nat (fun t => Nat) (fun a b => a) p` with `p` a free
/// variable: iota must fire via structure eta of the major (`p ↝
/// Pair.mk Nat p.0 p.1`) and project the first field.
#[test]
fn struct_eta_major_reduces_stuck_cases_on_fvar() {
    let env = pair_env();
    let mut tc = TypeChecker::new(&env);
    let nat = cnst(Name::str("Nat"));
    let pair = Name::str("Pair");
    let pair_nat = app(cnst(pair.clone()), nat.clone());
    let p = Expr::FVar(tc.fresh_fvar(Name::str("p"), pair_nat.clone()));
    let motive = lam("t", pair_nat.clone(), nat.clone());
    let first = lam("a", nat.clone(), lam("b", nat.clone(), Expr::BVar(1)));
    let e = apps(
        Expr::Const(
            pair.clone().append_str("rec"),
            vec![Level::succ(Level::zero())],
        ),
        vec![nat, motive, first, p.clone()],
    );
    let reduced = tc.whnf(&e);
    assert_eq!(
        reduced,
        Expr::Proj(pair, 0, Node::new(p)),
        "stuck structure major must eta-expand so iota projects the field"
    );
}

/// The def-eq shape the Init corpus actually needs: the major is BOUND by a
/// lambda, so the redex is only reachable after the checker opens the binder
/// with a typed local.
#[test]
fn struct_eta_major_decides_def_eq_under_binder() {
    let env = pair_env();
    let mut tc = TypeChecker::new(&env);
    let nat = cnst(Name::str("Nat"));
    let pair = Name::str("Pair");
    let pair_nat = app(cnst(pair.clone()), nat.clone());
    let motive = lam("t", pair_nat.clone(), nat.clone());
    let first = lam("a", nat.clone(), lam("b", nat.clone(), Expr::BVar(1)));
    let lhs = lam(
        "p",
        pair_nat.clone(),
        apps(
            Expr::Const(
                pair.clone().append_str("rec"),
                vec![Level::succ(Level::zero())],
            ),
            vec![nat.clone(), motive, first, Expr::BVar(0)],
        ),
    );
    let rhs = lam(
        "p",
        pair_nat.clone(),
        Expr::Proj(pair.clone(), 0, Node::new(Expr::BVar(0))),
    );
    assert!(
        tc.is_def_eq(&lhs, &rhs),
        "cases-on-first-field must be def-eq to the projection under a binder"
    );
    // Soundness control: the SECOND field is a different function.
    let second = lam("a", nat.clone(), lam("b", nat.clone(), Expr::BVar(0)));
    let motive2 = lam("t", pair_nat.clone(), nat.clone());
    let lhs2 = lam(
        "p",
        pair_nat.clone(),
        apps(
            Expr::Const(
                pair.clone().append_str("rec"),
                vec![Level::succ(Level::zero())],
            ),
            vec![nat, motive2, second, Expr::BVar(0)],
        ),
    );
    let mut tc2 = TypeChecker::new(&env);
    assert!(
        !tc2.is_def_eq(&lhs2, &rhs),
        "projecting the second field must not equal the first projection"
    );
}

/// Prop-sorted structures are excluded from major eta (Lean's
/// `== .sort .zero` guard): the recursor application must stay stuck.
#[test]
fn struct_eta_major_skips_prop_sorted_structures() {
    let env = pair_env();
    let mut tc = TypeChecker::new(&env);
    let pp = Name::str("PP");
    let q = cnst(Name::str("Q"));
    let h = Expr::FVar(tc.fresh_fvar(Name::str("h"), cnst(pp.clone())));
    // PP.rec.{1} (fun t => Q) (fun h1 h2 => h1) h  — Prop major, must stay
    // stuck (no wrong reduction, no eta on proofs outside the K path).
    let motive = lam("t", cnst(pp.clone()), q.clone());
    let minor = lam("h1", q.clone(), lam("h2", q, Expr::BVar(1)));
    let e = apps(
        Expr::Const(
            pp.clone().append_str("rec"),
            vec![Level::succ(Level::zero())],
        ),
        vec![motive, minor, h],
    );
    let reduced = tc.whnf(&e);
    assert_eq!(
        get_head_const_name(&reduced),
        Some(pp.append_str("rec")),
        "Prop-sorted structure major must stay stuck, got: {reduced:?}"
    );
}

fn get_head_const_name(e: &Expr) -> Option<Name> {
    let mut cur = e;
    loop {
        match cur {
            Expr::App(f, _) => cur = f,
            Expr::Const(n, _) => return Some(n.clone()),
            _ => return None,
        }
    }
}

// ---------------------------------------------------------------------------
// 5. C16b (Init decl #3864, `Array.extract_append_extract._proof_1_1`): a
//    `let`-tower duplicates its value at every level, so full instantiation
//    is exponential in the tower height. Substitution rebuilds spine nodes
//    WITHOUT passing through `Expr::clone`, so the per-declaration fuel used
//    to stay uncharged and `infer_type` allocated unboundedly (>12 GiB on a
//    14 GiB machine) with the budget long exhausted. Now every node visited
//    by the substitution builders is charged, and `infer_type` observes the
//    latch: a typed error, never an OOM — and never a truncated term.
// ---------------------------------------------------------------------------

#[test]
fn let_tower_exhausts_fuel_with_typed_error_never_oom() {
    let mut env = Environment::new();
    let n = Name::str("N");
    axiom(&mut env, n.clone(), Expr::Sort(Level::succ(Level::zero())));
    axiom(
        &mut env,
        Name::str("P"),
        pi(
            "x",
            cnst(n.clone()),
            pi("y", cnst(n.clone()), cnst(n.clone())),
        ),
    );
    axiom(&mut env, Name::str("a"), cnst(n.clone()));

    // let x0 = P a a in let x1 = P x0 x0 in … in let x59 = P x58 x58 in x59
    // Full instantiation is ~2^60 nodes: unrepresentable, must be cut off.
    let mut body = Expr::BVar(0);
    for i in (0..60).rev() {
        let val = if i == 0 {
            apps(
                cnst(Name::str("P")),
                vec![cnst(Name::str("a")), cnst(Name::str("a"))],
            )
        } else {
            apps(cnst(Name::str("P")), vec![Expr::BVar(0), Expr::BVar(0)])
        };
        body = Expr::Let(
            Name::str("x"),
            Node::new(cnst(n.clone())),
            Node::new(val),
            Node::new(body),
        );
    }

    oxilean_kernel::fuel::set_budget(Some(1_000_000));
    let mut tc = TypeChecker::new(&env);
    let res = tc.infer_type(&body);
    let exhausted = oxilean_kernel::fuel::is_exhausted();
    oxilean_kernel::fuel::set_budget(None);
    assert!(
        res.is_err(),
        "exponential let-tower must abort with a typed error"
    );
    assert!(
        exhausted,
        "the abort must be attributable to the fuel latch (named resource \
         limit downstream), not a genuine kernel verdict"
    );
}
