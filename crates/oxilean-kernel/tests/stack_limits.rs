//! Two limits for a caller whose stack is small.
//!
//! The kernel is recursive: a reduction nested inside another holds a frame
//! of the native stack until it is done, and so does every level of a term
//! while the term is walked. A caller that runs the kernel on a deep stack —
//! `oxilean-verify` gives it 512 MiB — needs neither limit, and there is none
//! by default. A caller that cannot (a WebAssembly module runs on its host's
//! stack) sets them on its thread:
//!
//! * `fuel::set_reduction_nesting_limit` — a reduction that would nest deeper
//!   is not started. Its term comes back unreduced, which is sound, and the
//!   event is recorded so that a failure that follows is not read as a
//!   verdict.
//! * `fuel::set_term_depth_limit` — a declaration with a deeper term is
//!   refused before the term is walked, and building a deeper one while
//!   checking spends the budget. Effective in a kernel built to track the
//!   depth of its terms (the `term-depth` feature); these tests pin both
//!   builds.
//!
//! Neither limit may ever turn into a verdict: what is over a limit is not
//! verified and not rejected.

use oxilean_kernel::{
    check_constant_info, fuel, init_builtin_env, AxiomVal, BinderInfo, ConstantInfo, ConstantVal,
    DefinitionSafety, DefinitionVal, Environment, Expr, KernelError, Level, Literal, Name, Node,
    Reducer, ReducibilityHint,
};

fn env() -> Environment {
    let mut env = Environment::new();
    match init_builtin_env(&mut env) {
        Ok(()) => env,
        Err(e) => unreachable!("builtin env must initialize: {e}"),
    }
}

fn nat(n: u64) -> Expr {
    Expr::Lit(Literal::nat(n))
}

fn cnst(name: &str) -> Expr {
    Expr::Const(Name::str(name), vec![])
}

fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Node::new(f), Node::new(a))
}

/// `Nat.add (Nat.add (… (Nat.add 0 1) …) 1) 1` with `k` additions. The
/// kernel adds two literals at once, and reduces an argument that is not a
/// literal yet first — so evaluating the sum is a reduction nested `k` deep,
/// one level for each addition still waiting for its left argument.
fn sum(k: u64) -> Expr {
    let mut term = nat(0);
    for _ in 0..k {
        term = app(app(cnst("Nat.add"), term), nat(1));
    }
    term
}

/// Run `work` on a thread with room for a few thousand nested reductions in
/// an unoptimised build, with no limit set when it starts.
fn on_a_deep_stack<T: Send + 'static>(work: impl FnOnce() -> T + Send + 'static) -> T {
    std::thread::Builder::new()
        .stack_size(256 << 20)
        .spawn(work)
        .unwrap_or_else(|e| panic!("cannot spawn the test thread: {e}"))
        .join()
        .unwrap_or_else(|_| panic!("the test thread panicked"))
}

#[test]
fn a_reduction_nested_within_the_limit_runs_to_its_value() {
    on_a_deep_stack(|| {
        let env = env();
        fuel::set_budget(None);
        fuel::set_reduction_nesting_limit(Some(64));
        let value = Reducer::new().whnf_env(&sum(40), &env);
        assert_eq!(value, nat(40));
        assert!(!fuel::reduction_limit_hit());
        fuel::set_reduction_nesting_limit(None);
    });
}

#[test]
fn a_reduction_nested_deeper_than_the_limit_is_not_started_and_is_recorded() {
    on_a_deep_stack(|| {
        let env = env();
        let term = sum(500);
        // No limit: five hundred levels are a matter of stack, and this
        // thread has it.
        fuel::set_budget(None);
        assert_eq!(Reducer::new().whnf_env(&term, &env), nat(500));
        assert!(!fuel::reduction_limit_hit());

        // A limit of 64: the reduction stops going down at level 64. What
        // comes back is not a value — and it is recorded that a limit, not
        // the term, ended the reduction.
        fuel::set_budget(None);
        fuel::set_reduction_nesting_limit(Some(64));
        let stuck = Reducer::new().whnf_env(&term, &env);
        assert!(
            !matches!(stuck, Expr::Lit(_)),
            "a reduction 500 deep cannot finish 64 deep"
        );
        assert!(fuel::reduction_limit_hit());
        // Sound: the term returned is the term given (nothing was reduced
        // at the top, since its argument could not be).
        assert_eq!(stuck, term);

        // The limit is the thread's; the next unit of work is under it too,
        // with a clean record.
        fuel::set_budget(None);
        assert!(!fuel::reduction_limit_hit());
        assert_eq!(fuel::reduction_nesting_limit(), Some(64));
        assert_eq!(Reducer::new().whnf_env(&sum(40), &env), nat(40));
        assert!(!fuel::reduction_limit_hit());

        // Lifted, the same term reduces again.
        fuel::set_reduction_nesting_limit(None);
        assert_eq!(Reducer::new().whnf_env(&term, &env), nat(500));
    });
}

#[test]
fn the_limit_counts_reductions_in_progress_not_reductions_made() {
    // Many shallow reductions one after another never come near a limit
    // that a single deep one exceeds.
    on_a_deep_stack(|| {
        let env = env();
        fuel::set_budget(None);
        fuel::set_reduction_nesting_limit(Some(16));
        let mut reducer = Reducer::new();
        for k in 0..2_000u64 {
            let value = reducer.whnf_env(&app(app(cnst("Nat.add"), sum(8)), nat(k)), &env);
            assert_eq!(value, nat(8 + k));
        }
        assert!(!fuel::reduction_limit_hit());
        fuel::set_reduction_nesting_limit(None);
    });
}

/// `N : Type`, `g : N → N`, `a : N`.
fn base_env() -> Environment {
    let mut env = Environment::new();
    let axiom = |env: &mut Environment, name: &str, ty: Expr| {
        let ci = ConstantInfo::Axiom(AxiomVal {
            common: ConstantVal {
                name: Name::str(name),
                level_params: vec![],
                ty,
            },
            is_unsafe: false,
        });
        check_constant_info(env, ci).unwrap_or_else(|e| panic!("axiom {name}: {e:?}"));
    };
    axiom(&mut env, "N", Expr::Sort(Level::succ(Level::zero())));
    axiom(
        &mut env,
        "g",
        Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Node::new(cnst("N")),
            Node::new(cnst("N")),
        ),
    );
    axiom(&mut env, "a", cnst("N"));
    env
}

/// `def name : N := g (g (… (g a)))`, `k` applications: a value `k + 1`
/// levels deep.
fn chain_definition(name: &str, k: u32) -> ConstantInfo {
    let mut value = cnst("a");
    for _ in 0..k {
        value = app(cnst("g"), value);
    }
    ConstantInfo::Definition(DefinitionVal {
        common: ConstantVal {
            name: Name::str(name),
            level_params: vec![],
            ty: cnst("N"),
        },
        value,
        hints: ReducibilityHint::Regular(1),
        safety: DefinitionSafety::Safe,
        all: vec![Name::str(name)],
    })
}

#[test]
fn a_declaration_with_a_term_over_the_depth_limit_is_refused_where_depth_is_tracked() {
    on_a_deep_stack(|| {
        let mut env = base_env();
        // No limit: a value a thousand levels deep is checked.
        fuel::set_budget(None);
        check_constant_info(&mut env, chain_definition("deep", 1_000))
            .unwrap_or_else(|e| panic!("{e:?}"));

        fuel::set_term_depth_limit(Some(100));
        // Within the limit (99 applications are 100 levels): checked, in
        // either build.
        fuel::set_budget(None);
        check_constant_info(&mut env, chain_definition("within", 99))
            .unwrap_or_else(|e| panic!("{e:?}"));
        assert!(!fuel::term_depth_limit_hit());

        // One level over.
        fuel::set_budget(None);
        let result = check_constant_info(&mut env, chain_definition("over", 100));
        if fuel::TERM_DEPTH_TRACKED {
            match result {
                Err(KernelError::Other(message)) => assert!(
                    message.contains("term-depth limit"),
                    "expected a refusal for the depth, got: {message}"
                ),
                other => panic!("expected a refusal for the depth, got {other:?}"),
            }
            // Recorded as a resource limit — the caller must not report a
            // rejection — and nothing was added.
            assert!(fuel::term_depth_limit_hit() && fuel::is_exhausted());
            assert!(env.find(&Name::str("over")).is_none());
        } else {
            // A kernel that does not track the depth keeps the limit and
            // never reaches it.
            result.unwrap_or_else(|e| panic!("{e:?}"));
            assert!(!fuel::term_depth_limit_hit());
        }

        // The record is the declaration's: the next one starts clean, and a
        // declaration within the limit still checks.
        fuel::set_budget(None);
        assert!(!fuel::term_depth_limit_hit() && !fuel::is_exhausted());
        check_constant_info(&mut env, chain_definition("within_again", 50))
            .unwrap_or_else(|e| panic!("{e:?}"));
        fuel::set_term_depth_limit(None);
    });
}
