//! The kernel's work follows a term's SHARED structure, not its size as a
//! tree, and the def-eq checker compares the arguments of two applications
//! of one definition before it unfolds the definition.
//!
//! Lean exports are DAGs: a subterm with many parents is stored once, and
//! the terms `omega` and `simp` build are a few thousand shared nodes
//! standing for millions of tree nodes. Through 0.1.4's first performance
//! pass the kernel typed, reduced and compared such a term once per PATH,
//! and charged its per-declaration budget for the tree — which put 632
//! declarations of Lean core `Init` over the budget (and 9,512 more behind
//! them). These tests pin what replaced that, on terms small enough to read:
//!
//! 1. type inference remembers what it has typed (`TypeChecker::infer_type`);
//! 2. definitional equality remembers pairs of any size, and compares two
//!    equal terms in separate allocations without walking their trees;
//! 3. `f a ≡ f b` follows from `a ≡ b` without unfolding `f` — and a failed
//!    comparison of the arguments decides nothing;
//! 4. the universe of a telescope is built simplified, so asking whether it
//!    is `Prop` does not take time exponential in the telescope's length;
//! 5. a declaration must be a closed term: no free variable, no loose bound
//!    variable.

use oxilean_kernel::Node;
use oxilean_kernel::{
    check_and_derive_family, check_constant_info, AxiomVal, BinderInfo, ConstantInfo, ConstantVal,
    DefinitionSafety, DefinitionVal, Environment, Expr, FVarId, InductiveSpec, KernelError, Level,
    LocalDecl, Name, ReducibilityHint, TheoremVal, TypeChecker,
};

fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Node::new(f), Node::new(a))
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

fn cnst(name: &str) -> Expr {
    Expr::Const(Name::str(name), vec![])
}

fn axiom(env: &mut Environment, name: &str, ty: Expr) {
    let ci = ConstantInfo::Axiom(AxiomVal {
        common: ConstantVal {
            name: Name::str(name),
            level_params: vec![],
            ty,
        },
        is_unsafe: false,
    });
    check_constant_info(env, ci).unwrap_or_else(|e| panic!("axiom {name}: {e:?}"));
}

fn definition(env: &mut Environment, name: &str, level_params: &[&str], ty: Expr, value: Expr) {
    let ci = ConstantInfo::Definition(DefinitionVal {
        common: ConstantVal {
            name: Name::str(name),
            level_params: level_params.iter().map(|p| Name::str(*p)).collect(),
            ty,
        },
        value,
        hints: ReducibilityHint::Regular(1),
        safety: DefinitionSafety::Safe,
        all: vec![Name::str(name)],
    });
    check_constant_info(env, ci).unwrap_or_else(|e| panic!("definition {name}: {e:?}"));
}

/// `N : Type`, `M : Type`, `a b : N`, `P : N → N → N`, `g : N → N`,
/// `F : Prop`.
fn base_env() -> Environment {
    let mut env = Environment::new();
    let ty = Expr::Sort(Level::succ(Level::zero()));
    axiom(&mut env, "N", ty.clone());
    axiom(&mut env, "M", ty);
    axiom(&mut env, "a", cnst("N"));
    axiom(&mut env, "b", cnst("N"));
    axiom(
        &mut env,
        "P",
        pi("x", cnst("N"), pi("y", cnst("N"), cnst("N"))),
    );
    axiom(&mut env, "g", pi("x", cnst("N"), cnst("N")));
    axiom(&mut env, "F", Expr::Sort(Level::zero()));
    env
}

/// `xₖ₊₁ = P xₖ xₖ` over `leaf`, with `xₖ` ONE allocation used twice:
/// `3 * height` allocations standing for a tree of `2^height` leaves.
fn tower(leaf: Expr, height: u32) -> Expr {
    let p = Node::new(cnst("P"));
    let mut term = leaf;
    for _ in 0..height {
        let shared = Node::new(term);
        term = Expr::App(Node::new(Expr::App(p.clone(), shared.clone())), shared);
    }
    term
}

/// Run `work` under a budget of `budget` units; its result, the units it
/// charged, and whether the budget ran out.
fn with_budget<T>(budget: u64, work: impl FnOnce() -> T) -> (T, u64, bool) {
    oxilean_kernel::fuel::set_budget(Some(budget));
    let result = work();
    let used = oxilean_kernel::fuel::used();
    let exhausted = oxilean_kernel::fuel::is_exhausted();
    oxilean_kernel::fuel::set_budget(None);
    (result, used, exhausted)
}

// ---------------------------------------------------------------------------
// 1. Type inference remembers what it has typed.
// ---------------------------------------------------------------------------

#[test]
fn a_shared_subterm_is_typed_once() {
    let env = base_env();
    // 2^50 leaves as a tree. Typing it once per path would not finish.
    let term = tower(cnst("a"), 50);
    let (ty, used, exhausted) = with_budget(1_000_000, || TypeChecker::new(&env).infer_type(&term));
    assert_eq!(ty.unwrap_or_else(|e| panic!("{e:?}")), cnst("N"));
    assert!(!exhausted);
    assert!(used < 5_000, "a 50-level tower charged {used} units");
}

#[test]
fn inference_out_of_budget_is_a_typed_error_and_is_not_remembered() {
    // The budget still bounds the work: with too little of it inference
    // stops with an error attributable to the budget (a named resource limit
    // downstream — never a rejection, never a truncated term) …
    let env = base_env();
    let term = tower(cnst("a"), 50);
    let mut tc = TypeChecker::new(&env);
    let (res, _, exhausted) = with_budget(40, || tc.infer_type(&term));
    assert!(res.is_err(), "40 units cannot type a 50-level tower");
    assert!(exhausted, "the failure must be the budget's");
    // … and that failure is not a fact about the term: the same checker,
    // given a budget, types it.
    let (res, _, exhausted) = with_budget(1_000_000, || tc.infer_type(&term));
    assert_eq!(res.unwrap_or_else(|e| panic!("{e:?}")), cnst("N"));
    assert!(!exhausted);
}

#[test]
fn a_type_is_looked_up_the_second_time() {
    let env = base_env();
    let term = tower(cnst("a"), 30);
    let mut tc = TypeChecker::new(&env);
    let (first, first_used, _) = with_budget(1_000_000, || tc.infer_type(&term));
    let (second, second_used, _) = with_budget(1_000_000, || tc.infer_type(&term));
    assert_eq!(first.ok(), second.ok());
    assert!(
        first_used > 100,
        "the first inference did the work: {first_used}"
    );
    assert!(
        second_used <= 2,
        "the second is a lookup: {second_used} units"
    );
    // An equal term in other allocations is the same key.
    let again = tower(cnst("a"), 30);
    let (third, third_used, _) = with_budget(1_000_000, || tc.infer_type(&again));
    assert_eq!(third.ok(), Some(cnst("N")));
    assert!(
        third_used <= 2,
        "an equal term is a lookup: {third_used} units"
    );
}

#[test]
fn rebinding_a_variable_drops_what_was_inferred_with_its_old_type() {
    // The checker's own variables are never bound twice. A caller of
    // `push_local` can do it; the types inferred before must not survive.
    let env = base_env();
    let mut tc = TypeChecker::new(&env);
    let x = FVarId(7);
    let g_x = app(cnst("g"), Expr::FVar(x));
    let decl = |ty: &str| LocalDecl {
        fvar: x,
        name: Name::str("x"),
        ty: cnst(ty),
        val: None,
    };
    tc.push_local(decl("N"));
    assert_eq!(tc.infer_type(&Expr::FVar(x)).ok(), Some(cnst("N")));
    assert_eq!(tc.infer_type(&g_x).ok(), Some(cnst("N")));
    tc.pop_local();
    // Bound again, at another type: `g x` no longer checks, and `x` has the
    // new type.
    tc.push_local(decl("M"));
    assert_eq!(tc.infer_type(&Expr::FVar(x)).ok(), Some(cnst("M")));
    assert!(
        tc.infer_type(&g_x).is_err(),
        "`g x` with `x : M` must not be typed from a remembered `x : N`"
    );
}

#[test]
fn changing_the_transparency_mode_drops_what_was_inferred() {
    let env = base_env();
    let term = tower(cnst("a"), 30);
    let mut tc = TypeChecker::new(&env);
    let (_, first_used, _) = with_budget(1_000_000, || tc.infer_type(&term));
    tc.set_transparency(oxilean_kernel::reduce::TransparencyMode::Reducible);
    let (ty, used, _) = with_budget(1_000_000, || tc.infer_type(&term));
    assert_eq!(ty.ok(), Some(cnst("N")));
    assert!(
        used * 2 > first_used,
        "inferred again, not looked up: {used} units against {first_used}"
    );
}

// ---------------------------------------------------------------------------
// 2. Definitional equality on big shared terms.
// ---------------------------------------------------------------------------

#[test]
fn two_equal_towers_in_separate_allocations_are_equal_at_once() {
    // 34 levels: 2^34 leaves each. `==` — what the checker's syntactic
    // test used — walks both trees, which takes a minute or more; comparing
    // each shared pair once takes microseconds. The time bound is far from
    // both, so a regression fails here instead of stalling the test.
    let env = base_env();
    let mut tc = TypeChecker::new(&env);
    let start = std::time::Instant::now();
    let (equal, used, exhausted) = with_budget(100_000, || {
        tc.is_def_eq(&tower(cnst("a"), 34), &tower(cnst("a"), 34))
    });
    assert!(equal);
    assert!(!exhausted);
    assert!(used < 1_000, "charged {used} units");
    assert!(
        start.elapsed().as_secs() < 10,
        "comparing two equal 34-level towers took {:?}",
        start.elapsed()
    );
}

#[test]
fn two_different_towers_are_told_apart_without_walking_their_trees() {
    // `P x x` against `P y y`, fifty levels deep, differing at the one leaf.
    // `P` is an axiom, so both sides are stuck applications and are compared
    // argument by argument: each level's pair comes up twice, and the second
    // time it must be found, not compared again — 2^50 comparisons otherwise.
    let env = base_env();
    let mut tc = TypeChecker::new(&env);
    let (equal, used, exhausted) = with_budget(1_000_000, || {
        tc.is_def_eq(&tower(cnst("a"), 50), &tower(cnst("b"), 50))
    });
    assert!(!equal, "`a` and `b` are different axioms");
    assert!(!exhausted, "the comparison must finish, not run out");
    assert!(used < 50_000, "charged {used} units");
    // And the verdict is the same asked again, and the other way round.
    assert!(!tc.is_def_eq(&tower(cnst("a"), 50), &tower(cnst("b"), 50)));
    assert!(!tc.is_def_eq(&tower(cnst("b"), 50), &tower(cnst("a"), 50)));
    assert!(tc.is_def_eq(&tower(cnst("b"), 50), &tower(cnst("b"), 50)));
}

// ---------------------------------------------------------------------------
// 3. Congruence before unfolding.
// ---------------------------------------------------------------------------

/// `base_env` plus
/// * `idN (x : N) : N := x`, so `idN a ≡ a`;
/// * `big (x : N) : N`, whose value is a complete binary tree of `P`
///   applications of depth `depth` with `g x` at every leaf — built with NO
///   sharing, so unfolding `big` and comparing two results visits all of it;
/// * `konst (x : N) : N := a`, which ignores its argument;
/// * `poly.{u} (x : N) : N := x`, which ignores its universe.
fn congruence_env(depth: u32) -> Environment {
    fn tree(depth: u32) -> Expr {
        if depth == 0 {
            app(cnst("g"), Expr::BVar(0))
        } else {
            app(app(cnst("P"), tree(depth - 1)), tree(depth - 1))
        }
    }
    let mut env = base_env();
    let n_to_n = pi("x", cnst("N"), cnst("N"));
    definition(
        &mut env,
        "idN",
        &[],
        n_to_n.clone(),
        lam("x", cnst("N"), Expr::BVar(0)),
    );
    definition(
        &mut env,
        "big",
        &[],
        n_to_n.clone(),
        lam("x", cnst("N"), tree(depth)),
    );
    definition(
        &mut env,
        "konst",
        &[],
        n_to_n.clone(),
        lam("x", cnst("N"), cnst("a")),
    );
    definition(
        &mut env,
        "poly",
        &["u"],
        n_to_n,
        lam("x", cnst("N"), Expr::BVar(0)),
    );
    env
}

#[test]
fn one_definition_applied_to_equal_arguments_is_equal_without_unfolding_it() {
    // `big (idN a)` against `big a`. The arguments are equal after one
    // unfolding of `idN`; unfolding `big` on both sides instead builds two
    // trees of 2^12 leaves and compares them leaf by leaf.
    let env = congruence_env(12);
    let lhs = app(cnst("big"), app(cnst("idN"), cnst("a")));
    let rhs = app(cnst("big"), cnst("a"));
    let mut tc = TypeChecker::new(&env);
    let (equal, used, exhausted) = with_budget(1_000_000, || tc.is_def_eq(&lhs, &rhs));
    assert!(equal);
    assert!(!exhausted);
    assert!(
        used < 500,
        "charged {used} units: `big` was unfolded (its value is thousands of nodes)"
    );
}

#[test]
fn unequal_arguments_do_not_make_two_applications_equal() {
    let env = congruence_env(3);
    let mut tc = TypeChecker::new(&env);
    // `big a` against `big b`: the arguments differ and so do the values.
    assert!(!tc.is_def_eq(&app(cnst("big"), cnst("a")), &app(cnst("big"), cnst("b"))));
    // `g` is an axiom: `g a` and `g b` are different stuck terms.
    assert!(!tc.is_def_eq(&app(cnst("g"), cnst("a")), &app(cnst("g"), cnst("b"))));
    // Two different definitions are compared by what they unfold to:
    // `idN a` and `poly a` are both `a`; `idN a` and `konst b` are both `a`;
    // `idN b` and `konst b` are `b` and `a`.
    let poly = Expr::Const(Name::str("poly"), vec![Level::zero()]);
    assert!(tc.is_def_eq(&app(cnst("idN"), cnst("a")), &app(poly, cnst("a"))));
    assert!(tc.is_def_eq(&app(cnst("idN"), cnst("a")), &app(cnst("konst"), cnst("b"))));
    assert!(!tc.is_def_eq(&app(cnst("idN"), cnst("b")), &app(cnst("konst"), cnst("b"))));
    // A different number of arguments is never equal by congruence.
    assert!(!tc.is_def_eq(
        &app(app(cnst("P"), cnst("a")), cnst("a")),
        &app(cnst("g"), cnst("a"))
    ));
}

#[test]
fn a_failed_comparison_of_the_arguments_decides_nothing() {
    // `konst a` against `konst b`: the arguments are different, but `konst`
    // ignores its argument — both sides are `a`. Comparing arguments first
    // is a shortcut to `true`, never an answer of `false`.
    let env = congruence_env(3);
    let mut tc = TypeChecker::new(&env);
    assert!(!tc.is_def_eq(&cnst("a"), &cnst("b")));
    assert!(tc.is_def_eq(
        &app(cnst("konst"), cnst("a")),
        &app(cnst("konst"), cnst("b"))
    ));
    // The same for universe levels: `poly.{0} a` and `poly.{1} a` are not at
    // equivalent levels, and both unfold to `a`.
    let poly = |level: Level| app(Expr::Const(Name::str("poly"), vec![level]), cnst("a"));
    assert!(tc.is_def_eq(&poly(Level::zero()), &poly(Level::succ(Level::zero()))));
    // Equivalent levels that are not written the same are equal by
    // congruence: `max u u` is `u`.
    let u = Level::param(Name::str("u"));
    assert!(tc.is_def_eq(&poly(Level::max(u.clone(), u.clone())), &poly(u)));
}

// ---------------------------------------------------------------------------
// 4. The universe of a telescope.
// ---------------------------------------------------------------------------

#[test]
fn proof_irrelevance_on_a_long_telescope_does_not_take_exponential_time() {
    // `h1 h2 : ∀ (x1 … x40 : N), F`. Two proofs of one proposition are
    // definitionally equal; to see that the type is a proposition the
    // def-eq checker types the telescope, and its universe is an `imax` per
    // binder. Built unsimplified (`imax 1 (imax 1 (… 0))`), deciding that
    // level is `0` distributed over every nesting — 2^40 steps here, which
    // is what made two inductive types of Std (`Std.DTreeMap.Internal.Impl.WF`,
    // `Std.DHashMap.Raw.WF`) take 18 s and more than 30 s to check. Built
    // with the simplifying constructor it is `0` at every level.
    let mut env = base_env();
    let mut telescope = cnst("F");
    for _ in 0..40 {
        telescope = pi("x", cnst("N"), telescope);
    }
    axiom(&mut env, "h1", telescope.clone());
    axiom(&mut env, "h2", telescope);
    // A regression fails this assertion when the deadline passes, instead
    // of hanging the test.
    oxilean_kernel::deadline::set_deadline(Some(std::time::Duration::from_secs(20)));
    let mut tc = TypeChecker::new(&env);
    let equal = tc.is_def_eq(&cnst("h1"), &cnst("h2"));
    let expired = oxilean_kernel::deadline::is_expired();
    oxilean_kernel::deadline::set_deadline(None);
    assert!(!expired, "deciding the telescope's universe took over 20 s");
    assert!(equal, "two proofs of one proposition are equal");
    // Not a proposition: no proof irrelevance, and the two are different.
    let mut env = base_env();
    let mut telescope = cnst("N");
    for _ in 0..40 {
        telescope = pi("x", cnst("N"), telescope);
    }
    axiom(&mut env, "f1", telescope.clone());
    axiom(&mut env, "f2", telescope);
    let mut tc = TypeChecker::new(&env);
    assert!(!tc.is_def_eq(&cnst("f1"), &cnst("f2")));
}

// ---------------------------------------------------------------------------
// 5. A declaration is a closed term.
// ---------------------------------------------------------------------------

fn theorem(name: &str, ty: Expr, value: Expr) -> ConstantInfo {
    ConstantInfo::Theorem(TheoremVal {
        common: ConstantVal {
            name: Name::str(name),
            level_params: vec![],
            ty,
        },
        value,
        all: vec![Name::str(name)],
    })
}

fn refused_for(result: Result<(), KernelError>, what: &str) {
    match result {
        Err(KernelError::Other(message)) => assert!(
            message.contains(what),
            "expected a refusal for a {what}, got: {message}"
        ),
        other => panic!("expected a refusal for a {what}, got {other:?}"),
    }
}

#[test]
fn a_declaration_with_a_free_variable_is_refused() {
    // `(fun (x : F) => x) (FVar 0)`, offered as a proof of `F`.
    //
    // The checker opens the binder of `x` with its own free variable — the
    // first one it mints, number 0 — and types the body, the variable `x`,
    // against it: `FVar 0 : F`. It remembers the types it has inferred. The
    // ARGUMENT is a free variable the declaration brought along, with the
    // same number; without the check below it would be given the type
    // remembered for `x`, and the application would be a proof of `F` from
    // nothing. No export can spell a free variable; this is the Rust API.
    let mut env = base_env();
    let identity = lam("x", cnst("F"), Expr::BVar(0));
    let bad = theorem("bad", cnst("F"), app(identity, Expr::FVar(FVarId(0))));
    refused_for(check_constant_info(&mut env, bad), "free variable");
    assert!(env.find(&Name::str("bad")).is_none(), "nothing was added");

    // The shape that got past the kernel BEFORE it remembered types (and is
    // in the published 0.1.3): `fun (x : F) => FVar 1`, offered as a proof
    // of `F → F`. The statement's own binder is opened with number 0, the
    // lambda's with number 1 — so under the lambda `FVar 1` passed for `x`,
    // and the theorem went into the environment with the free variable in
    // its value, to be unfolded later under some other binder numbered 1.
    let alias = theorem(
        "alias",
        pi("h", cnst("F"), cnst("F")),
        lam("x", cnst("F"), Expr::FVar(FVarId(1))),
    );
    refused_for(check_constant_info(&mut env, alias), "free variable");
    assert!(env.find(&Name::str("alias")).is_none(), "nothing was added");

    // In the type as well, and for a definition, and whatever the number.
    let bad = theorem("bad", Expr::FVar(FVarId(41)), cnst("a"));
    refused_for(check_constant_info(&mut env, bad), "free variable");
    let bad = ConstantInfo::Definition(DefinitionVal {
        common: ConstantVal {
            name: Name::str("bad"),
            level_params: vec![],
            ty: cnst("N"),
        },
        value: app(cnst("g"), Expr::FVar(FVarId(1 << 62))),
        hints: ReducibilityHint::Regular(1),
        safety: DefinitionSafety::Safe,
        all: vec![Name::str("bad")],
    });
    refused_for(check_constant_info(&mut env, bad), "free variable");
    assert!(env.find(&Name::str("bad")).is_none());

    // The same proof with nothing free checks: the check is not in the way.
    let good = theorem(
        "good",
        pi("h", cnst("F"), cnst("F")),
        lam("h", cnst("F"), Expr::BVar(0)),
    );
    check_constant_info(&mut env, good).unwrap_or_else(|e| panic!("{e:?}"));
}

#[test]
fn a_declaration_with_a_loose_bound_variable_is_refused() {
    let mut env = base_env();
    let bad = theorem("bad", cnst("F"), Expr::BVar(0));
    refused_for(check_constant_info(&mut env, bad), "loose bound variable");
    // One binder is not enough for index 1.
    let bad = theorem(
        "bad",
        pi("h", cnst("F"), cnst("F")),
        lam("h", cnst("F"), Expr::BVar(1)),
    );
    refused_for(check_constant_info(&mut env, bad), "loose bound variable");
    let bad = theorem("bad", app(cnst("g"), Expr::BVar(3)), cnst("a"));
    refused_for(check_constant_info(&mut env, bad), "loose bound variable");
    assert!(env.find(&Name::str("bad")).is_none());
}

#[test]
fn an_inductive_family_with_a_free_variable_is_refused() {
    let env = base_env();
    let sort = Expr::Sort(Level::succ(Level::zero()));
    // `inductive T : Type | mk : FVar 0 → T`
    let spec = InductiveSpec::new(
        Name::str("T"),
        sort.clone(),
        vec![(Name::str("T.mk"), pi("x", Expr::FVar(FVarId(0)), cnst("T")))],
    );
    let result = check_and_derive_family(&env, &[], 0, &[spec]).map(|_| ());
    refused_for(result, "free variable");
    // In the type of the family itself.
    let spec = InductiveSpec::new(Name::str("T"), pi("x", cnst("N"), Expr::BVar(1)), vec![]);
    let result = check_and_derive_family(&env, &[], 0, &[spec]).map(|_| ());
    refused_for(result, "loose bound variable");
    // The control: the same family, closed, derives.
    let spec = InductiveSpec::new(
        Name::str("T"),
        sort,
        vec![(Name::str("T.mk"), pi("x", cnst("N"), cnst("T")))],
    );
    check_and_derive_family(&env, &[], 0, &[spec]).unwrap_or_else(|e| panic!("{e:?}"));
}
