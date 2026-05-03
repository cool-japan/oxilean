//! Criterion benchmark suite for the `oxilean-kernel` hot paths.
//!
//! Benchmarks cover:
//! - WHNF reduction (beta, delta, let, chained)
//! - Definitional equality checking (reflexivity, beta, delta, structural)
//! - Type inference (sort, literal, lambda, pi, application)
//! - Substitution / instantiation (shallow, deep, wide)
//! - Nat arithmetic via builtin evaluation (add, mul, large operands)
//! - Full normalisation (beta chain, let chain)
//! - Alpha equivalence checking

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use oxilean_kernel::{
    alpha_equiv, beta_normalize, beta_step, init_builtin_env, instantiate, is_def_eq_simple,
    normalize, normalize_env, normalize_whnf, reduce_nat_op, simplify, whnf, BinderInfo,
    Declaration, DefEqChecker, Environment, Expr, Level, Literal, Name, Reducer, ReducibilityHint,
    TypeChecker,
};
use std::hint::black_box;

// ─────────────────────────────────────────────
// Helpers: expression builders
// ─────────────────────────────────────────────

/// Build `(λ x : τ, x)` — the identity function.
fn mk_identity(ty: Expr) -> Expr {
    Expr::Lam(
        BinderInfo::Default,
        Name::str("x"),
        Box::new(ty),
        Box::new(Expr::BVar(0)),
    )
}

/// Build `f arg`.
fn mk_app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}

/// Build a `let x : τ := v in body` expression.
fn mk_let(name: &str, ty: Expr, val: Expr, body: Expr) -> Expr {
    Expr::Let(Name::str(name), Box::new(ty), Box::new(val), Box::new(body))
}

/// Build `Π (x : τ), σ`.
fn mk_pi(name: &str, ty: Expr, body: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str(name),
        Box::new(ty),
        Box::new(body),
    )
}

/// Build a `Nat` literal expression.
fn nat(n: u64) -> Expr {
    Expr::Lit(Literal::Nat(n))
}

/// Build `Nat.add m n` in explicit application form.
fn nat_add_expr(m: u64, n: u64) -> Expr {
    mk_app(
        mk_app(Expr::Const(Name::str("Nat.add"), vec![]), nat(m)),
        nat(n),
    )
}

/// Build `Nat.mul m n` in explicit application form.
fn nat_mul_expr(m: u64, n: u64) -> Expr {
    mk_app(
        mk_app(Expr::Const(Name::str("Nat.mul"), vec![]), nat(m)),
        nat(n),
    )
}

/// Build a left-associated chain of beta redexes:
///   `(λ x, x) ((λ x, x) (… (λ x, x) arg …))`
/// with `depth` nested applications.
fn beta_chain(depth: usize) -> Expr {
    let id = mk_identity(Expr::Sort(Level::zero()));
    let mut e = nat(0);
    for _ in 0..depth {
        e = mk_app(id.clone(), e);
    }
    e
}

/// Build a right-nested `let` chain of depth `n`:
///   `let x0 = 0 in let x1 = x0 in … in x_{n-1}`
fn let_chain(depth: usize) -> Expr {
    // innermost: return the deepest bound variable
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let mut body = Expr::BVar(0);
    for i in 0..depth {
        let name = format!("x{i}");
        body = mk_let(&name, nat_ty.clone(), nat(i as u64), body);
    }
    body
}

/// Build a deeply nested `Π` type of depth `n`:
///   `Π (x0 : Nat), Π (x1 : Nat), … Nat`
fn deep_pi(depth: usize) -> Expr {
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let mut ty = nat_ty.clone();
    for i in 0..depth {
        let name = format!("x{i}");
        ty = mk_pi(&name, nat_ty.clone(), ty);
    }
    ty
}

/// Build an expression for `instantiate` stress tests:
/// a body of depth `width` where BVar(0) appears in every leaf.
/// Body: `(App (App … (App BVar(0) BVar(0)) BVar(0)) … BVar(0))` (width apps).
fn wide_bvar_body(width: usize) -> Expr {
    let mut e = Expr::BVar(0);
    for _ in 0..width {
        e = mk_app(e, Expr::BVar(0));
    }
    e
}

/// Create a fresh environment with the builtin Nat and Bool types loaded.
fn builtin_env() -> Environment {
    let mut env = Environment::new();
    init_builtin_env(&mut env).expect("builtin env should initialise cleanly");
    env
}

/// Add a simple `Nat` constant definition to an environment.
fn env_with_answer(env: &mut Environment, name: &str, val: u64) {
    env.add(Declaration::Definition {
        name: Name::str(name),
        univ_params: vec![],
        ty: Expr::Const(Name::str("Nat"), vec![]),
        val: nat(val),
        hint: ReducibilityHint::Regular(1),
    })
    .expect("definition should be added");
}

// ─────────────────────────────────────────────
// Benchmark group: WHNF reduction
// ─────────────────────────────────────────────

fn bench_whnf(c: &mut Criterion) {
    let mut group = c.benchmark_group("whnf");

    // Single beta redex: `(λ x, x) 42`
    let id_42 = mk_app(mk_identity(Expr::Const(Name::str("Nat"), vec![])), nat(42));
    group.bench_function("beta_single", |b| {
        b.iter(|| whnf(black_box(&id_42)));
    });

    // Chained beta redexes at various depths
    for depth in [4usize, 8, 16, 32] {
        let chain = beta_chain(depth);
        group.bench_with_input(BenchmarkId::new("beta_chain", depth), &chain, |b, expr| {
            b.iter(|| whnf(black_box(expr)));
        });
    }

    // Let reduction: `let x = 42 in x`
    let let_expr = mk_let(
        "x",
        Expr::Const(Name::str("Nat"), vec![]),
        nat(42),
        Expr::BVar(0),
    );
    group.bench_function("let_single", |b| {
        b.iter(|| whnf(black_box(&let_expr)));
    });

    // Let chain at varying depths
    for depth in [4usize, 16, 64] {
        let chain = let_chain(depth);
        group.bench_with_input(BenchmarkId::new("let_chain", depth), &chain, |b, expr| {
            b.iter(|| whnf(black_box(expr)));
        });
    }

    // WHNF with environment delta-expansion
    let env = builtin_env();
    let nat_add = nat_add_expr(1000, 2000);
    group.bench_function("whnf_env_nat_add", |b| {
        b.iter(|| Reducer::new().whnf_env(black_box(&nat_add), black_box(&env)));
    });

    // Already-normalised expression (cache hit path)
    let already_whnf = nat(12345);
    group.bench_function("whnf_already_normal", |b| {
        b.iter(|| whnf(black_box(&already_whnf)));
    });

    // Sort expression (trivially normal)
    let sort_u0 = Expr::Sort(Level::zero());
    group.bench_function("whnf_sort", |b| {
        b.iter(|| whnf(black_box(&sort_u0)));
    });

    // Lambda under application spine — tests spine traversal
    let double_beta = mk_app(
        mk_app(
            Expr::Lam(
                BinderInfo::Default,
                Name::str("f"),
                Expr::Sort(Level::zero()).into(),
                Box::new(Expr::Lam(
                    BinderInfo::Default,
                    Name::str("x"),
                    Expr::Sort(Level::zero()).into(),
                    Box::new(mk_app(Expr::BVar(1), Expr::BVar(0))),
                )),
            ),
            mk_identity(Expr::Sort(Level::zero())),
        ),
        nat(7),
    );
    group.bench_function("whnf_curried_apply", |b| {
        b.iter(|| whnf(black_box(&double_beta)));
    });

    group.finish();
}

// ─────────────────────────────────────────────
// Benchmark group: Definitional equality
// ─────────────────────────────────────────────

fn bench_def_eq(c: &mut Criterion) {
    let mut group = c.benchmark_group("def_eq");

    // Reflexivity: identical literals
    let n42 = nat(42);
    group.bench_function("reflexivity_lit", |b| {
        b.iter(|| is_def_eq_simple(black_box(&n42), black_box(&n42)));
    });

    // Reflexivity: Sort
    let sort0 = Expr::Sort(Level::zero());
    group.bench_function("reflexivity_sort", |b| {
        b.iter(|| is_def_eq_simple(black_box(&sort0), black_box(&sort0)));
    });

    // Beta equality: `(λ x, x) 42  ≡  42`
    let lhs_beta = mk_app(mk_identity(Expr::Const(Name::str("Nat"), vec![])), nat(42));
    group.bench_function("beta_eq", |b| {
        b.iter(|| is_def_eq_simple(black_box(&lhs_beta), black_box(&n42)));
    });

    // Chained beta vs normal form
    for depth in [4usize, 8, 16] {
        let chain = beta_chain(depth);
        group.bench_with_input(
            BenchmarkId::new("beta_chain_eq", depth),
            &chain,
            |b, expr| {
                b.iter(|| is_def_eq_simple(black_box(expr), black_box(&nat(0))));
            },
        );
    }

    // Delta equality: constant vs its definition
    {
        let mut env = builtin_env();
        env_with_answer(&mut env, "myAnswer", 99);
        let answer_const = Expr::Const(Name::str("myAnswer"), vec![]);
        let answer_val = nat(99);
        group.bench_function("delta_eq", |b| {
            b.iter(|| {
                let mut checker = DefEqChecker::new(black_box(&env));
                checker.is_def_eq(black_box(&answer_const), black_box(&answer_val))
            });
        });
    }

    // Inequality: different literals
    let n1 = nat(1);
    let n2 = nat(2);
    group.bench_function("inequality_lit", |b| {
        b.iter(|| is_def_eq_simple(black_box(&n1), black_box(&n2)));
    });

    // Structural equality: deep Pi types
    for depth in [4usize, 8, 16] {
        let pi = deep_pi(depth);
        group.bench_with_input(
            BenchmarkId::new("structural_pi_refl", depth),
            &pi,
            |b, expr| {
                b.iter(|| is_def_eq_simple(black_box(expr), black_box(expr)));
            },
        );
    }

    // Level max commutativity: `max(u, v) ≡ max(v, u)`
    let s1 = Expr::Sort(Level::max(
        Level::param(Name::str("u")),
        Level::param(Name::str("v")),
    ));
    let s2 = Expr::Sort(Level::max(
        Level::param(Name::str("v")),
        Level::param(Name::str("u")),
    ));
    group.bench_function("level_max_comm", |b| {
        b.iter(|| is_def_eq_simple(black_box(&s1), black_box(&s2)));
    });

    group.finish();
}

// ─────────────────────────────────────────────
// Benchmark group: Type inference
// ─────────────────────────────────────────────

fn bench_infer(c: &mut Criterion) {
    let env = builtin_env();
    let mut group = c.benchmark_group("infer");

    // Infer type of `Prop` → `Type 1`
    let prop = Expr::Sort(Level::zero());
    group.bench_function("infer_sort_prop", |b| {
        b.iter(|| {
            let mut tc = TypeChecker::new(black_box(&env));
            tc.infer_type(black_box(&prop)).map_err(Box::new)
        });
    });

    // Infer type of a `Nat` literal → `Nat`
    let lit42 = nat(42);
    group.bench_function("infer_nat_literal", |b| {
        b.iter(|| {
            let mut tc = TypeChecker::new(black_box(&env));
            tc.infer_type(black_box(&lit42)).map_err(Box::new)
        });
    });

    // Infer type of `Bool` constant → `Type 1`
    let bool_const = Expr::Const(Name::str("Bool"), vec![]);
    group.bench_function("infer_bool_const", |b| {
        b.iter(|| {
            let mut tc = TypeChecker::new(black_box(&env));
            tc.infer_type(black_box(&bool_const)).map_err(Box::new)
        });
    });

    // Infer type of identity function `λ x : Prop, x` → `Π (x : Prop), Prop`
    let id_prop = mk_identity(Expr::Sort(Level::zero()));
    group.bench_function("infer_identity_lambda", |b| {
        b.iter(|| {
            let mut tc = TypeChecker::new(black_box(&env));
            tc.infer_type(black_box(&id_prop)).map_err(Box::new)
        });
    });

    // Infer type of a `Π` type → sort
    let pi_ty = mk_pi(
        "x",
        Expr::Const(Name::str("Bool"), vec![]),
        Expr::Const(Name::str("Nat"), vec![]),
    );
    group.bench_function("infer_pi_type", |b| {
        b.iter(|| {
            let mut tc = TypeChecker::new(black_box(&env));
            tc.infer_type(black_box(&pi_ty)).map_err(Box::new)
        });
    });

    // Infer type of `(λ x, x) 42` (application) → `Nat`
    let app_expr = mk_app(
        mk_identity(Expr::Const(Name::str("Nat"), vec![])),
        lit42.clone(),
    );
    group.bench_function("infer_beta_app", |b| {
        b.iter(|| {
            let mut tc = TypeChecker::new(black_box(&env));
            tc.infer_type(black_box(&app_expr)).map_err(Box::new)
        });
    });

    // Infer type for deeply nested lambdas (stress test binder handling)
    for depth in [4usize, 8, 16] {
        let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
        let mut lam = Expr::BVar(0);
        for _ in 0..depth {
            lam = Expr::Lam(
                BinderInfo::Default,
                Name::str("x"),
                Box::new(nat_ty.clone()),
                Box::new(lam),
            );
        }
        group.bench_with_input(
            BenchmarkId::new("infer_nested_lambda", depth),
            &lam,
            |b, expr| {
                b.iter(|| {
                    let mut tc = TypeChecker::new(black_box(&env));
                    tc.infer_type(black_box(expr)).map_err(Box::new)
                });
            },
        );
    }

    group.finish();
}

// ─────────────────────────────────────────────
// Benchmark group: Substitution / instantiation
// ─────────────────────────────────────────────

fn bench_subst(c: &mut Criterion) {
    let mut group = c.benchmark_group("subst");
    let arg = nat(99);

    // Shallow body: BVar(0) at root
    let shallow = Expr::BVar(0);
    group.bench_function("instantiate_shallow", |b| {
        b.iter(|| instantiate(black_box(&shallow), black_box(&arg)));
    });

    // Deep single-path body: `App(App(…BVar(0)…, c), c)` — depth path
    for depth in [8usize, 32, 128] {
        let mut body = Expr::BVar(0);
        for _ in 0..depth {
            body = mk_app(body, nat(0));
        }
        group.bench_with_input(
            BenchmarkId::new("instantiate_deep_app_chain", depth),
            &body,
            |b, expr| {
                b.iter(|| instantiate(black_box(expr), black_box(&arg)));
            },
        );
    }

    // Wide body: many BVar(0) leaves in a flat application spine
    for width in [8usize, 32, 128] {
        let body = wide_bvar_body(width);
        group.bench_with_input(
            BenchmarkId::new("instantiate_wide_bvar", width),
            &body,
            |b, expr| {
                b.iter(|| instantiate(black_box(expr), black_box(&arg)));
            },
        );
    }

    // Instantiate under binders (BVar(0) reference shifts through Pi domains)
    for depth in [4usize, 16, 64] {
        let pi = deep_pi(depth);
        group.bench_with_input(
            BenchmarkId::new("instantiate_under_pi", depth),
            &pi,
            |b, expr| {
                b.iter(|| instantiate(black_box(expr), black_box(&arg)));
            },
        );
    }

    // beta_step: single reduction step benchmark
    let redex = mk_app(
        mk_identity(Expr::Const(Name::str("Nat"), vec![])),
        arg.clone(),
    );
    group.bench_function("beta_step_single", |b| {
        b.iter(|| beta_step(black_box(&redex)));
    });

    // beta_normalize over a chain
    for depth in [4usize, 16, 64] {
        let chain = beta_chain(depth);
        group.bench_with_input(
            BenchmarkId::new("beta_normalize_chain", depth),
            &chain,
            |b, expr| {
                b.iter(|| beta_normalize(black_box(expr)));
            },
        );
    }

    group.finish();
}

// ─────────────────────────────────────────────
// Benchmark group: Nat arithmetic (builtin eval)
// ─────────────────────────────────────────────

fn bench_nat_arith(c: &mut Criterion) {
    let env = builtin_env();
    let mut group = c.benchmark_group("nat_arith");

    // reduce_nat_op is the low-level direct evaluator
    let args_add_small = vec![nat(12), nat(34)];
    group.bench_function("reduce_nat_op_add_small", |b| {
        b.iter(|| reduce_nat_op(black_box("Nat.add"), black_box(&args_add_small)));
    });

    let args_mul_small = vec![nat(7), nat(8)];
    group.bench_function("reduce_nat_op_mul_small", |b| {
        b.iter(|| reduce_nat_op(black_box("Nat.mul"), black_box(&args_mul_small)));
    });

    // Large operands
    for (m, n) in [
        (1_000u64, 2_000),
        (100_000, 200_000),
        (1_000_000, 1_000_000),
    ] {
        let args = vec![nat(m), nat(n)];
        group.bench_with_input(
            BenchmarkId::new("reduce_nat_op_add_large", format!("{m}+{n}")),
            &args,
            |b, a| {
                b.iter(|| reduce_nat_op(black_box("Nat.add"), black_box(a)));
            },
        );
        let args_mul = vec![nat(m), nat(n)];
        group.bench_with_input(
            BenchmarkId::new("reduce_nat_op_mul_large", format!("{m}*{n}")),
            &args_mul,
            |b, a| {
                b.iter(|| reduce_nat_op(black_box("Nat.mul"), black_box(a)));
            },
        );
    }

    // whnf_env on Nat.add application (full reduction pipeline)
    let add_1000_2000 = nat_add_expr(1000, 2000);
    group.bench_function("whnf_env_nat_add_1000_2000", |b| {
        b.iter(|| Reducer::new().whnf_env(black_box(&add_1000_2000), black_box(&env)));
    });

    let mul_100_200 = nat_mul_expr(100, 200);
    group.bench_function("whnf_env_nat_mul_100_200", |b| {
        b.iter(|| Reducer::new().whnf_env(black_box(&mul_100_200), black_box(&env)));
    });

    // simplify path for Nat.succ
    let succ_expr = mk_app(Expr::Const(Name::str("Nat.succ"), vec![]), nat(41));
    group.bench_function("simplify_nat_succ", |b| {
        b.iter(|| simplify(black_box(&succ_expr)));
    });

    group.finish();
}

// ─────────────────────────────────────────────
// Benchmark group: Full normalisation
// ─────────────────────────────────────────────

fn bench_normalize(c: &mut Criterion) {
    let mut group = c.benchmark_group("normalize");

    // Already-normal literal
    group.bench_function("normalize_lit", |b| {
        b.iter(|| normalize(black_box(&nat(42))));
    });

    // Beta chain normalisation
    for depth in [4usize, 8, 16] {
        let chain = beta_chain(depth);
        group.bench_with_input(
            BenchmarkId::new("normalize_beta_chain", depth),
            &chain,
            |b, expr| {
                b.iter(|| normalize(black_box(expr)));
            },
        );
    }

    // Let chain normalisation
    for depth in [4usize, 16, 64] {
        let chain = let_chain(depth);
        group.bench_with_input(
            BenchmarkId::new("normalize_let_chain", depth),
            &chain,
            |b, expr| {
                b.iter(|| normalize(black_box(expr)));
            },
        );
    }

    // normalize_whnf: WHNF then full norm — stress test
    for depth in [4usize, 8, 16] {
        let chain = beta_chain(depth);
        group.bench_with_input(
            BenchmarkId::new("normalize_whnf_chain", depth),
            &chain,
            |b, expr| {
                b.iter(|| normalize_whnf(black_box(expr)));
            },
        );
    }

    // normalize_env: constant unfolding
    {
        let mut env2 = builtin_env();
        env_with_answer(&mut env2, "fortyTwo", 42);
        let const_expr = Expr::Const(Name::str("fortyTwo"), vec![]);
        group.bench_function("normalize_env_const_unfold", |b| {
            b.iter(|| normalize_env(black_box(&const_expr), black_box(&env2)));
        });
    }

    // Deep Pi type normalisation (structural traversal)
    for depth in [4usize, 16, 32] {
        let pi = deep_pi(depth);
        group.bench_with_input(
            BenchmarkId::new("normalize_deep_pi", depth),
            &pi,
            |b, expr| {
                b.iter(|| normalize(black_box(expr)));
            },
        );
    }

    group.finish();
}

// ─────────────────────────────────────────────
// Benchmark group: Alpha equivalence
// ─────────────────────────────────────────────

fn bench_alpha(c: &mut Criterion) {
    let mut group = c.benchmark_group("alpha");

    // Reflexive literals
    let n42 = nat(42);
    group.bench_function("alpha_refl_lit", |b| {
        b.iter(|| alpha_equiv(black_box(&n42), black_box(&n42)));
    });

    // Structurally equal but distinct AST nodes
    let pi1 = deep_pi(8);
    let pi2 = deep_pi(8);
    group.bench_function("alpha_deep_pi_eq", |b| {
        b.iter(|| alpha_equiv(black_box(&pi1), black_box(&pi2)));
    });

    // Lambda: rename check
    let lam1 = mk_identity(Expr::Sort(Level::zero()));
    let lam2 = Expr::Lam(
        BinderInfo::Default,
        Name::str("y"), // different binder name — alpha-equivalent
        Box::new(Expr::Sort(Level::zero())),
        Box::new(Expr::BVar(0)),
    );
    group.bench_function("alpha_lambda_rename", |b| {
        b.iter(|| alpha_equiv(black_box(&lam1), black_box(&lam2)));
    });

    // Non-equivalent pair
    let lam_not_id = Expr::Lam(
        BinderInfo::Default,
        Name::str("x"),
        Box::new(Expr::Sort(Level::zero())),
        Box::new(nat(0)), // constant body, not BVar(0)
    );
    group.bench_function("alpha_not_equiv", |b| {
        b.iter(|| alpha_equiv(black_box(&lam1), black_box(&lam_not_id)));
    });

    group.finish();
}

// ─────────────────────────────────────────────
// Benchmark group: Environment construction
// ─────────────────────────────────────────────

fn bench_env(c: &mut Criterion) {
    let mut group = c.benchmark_group("env");

    // Cost of initialising the builtin environment (Nat, Bool, etc.)
    group.bench_function("init_builtin_env", |b| {
        b.iter(|| {
            let mut env = Environment::new();
            init_builtin_env(black_box(&mut env))
        });
    });

    // Sequential definition additions
    for n in [10usize, 50, 200] {
        group.bench_with_input(BenchmarkId::new("add_definitions", n), &n, |b, &count| {
            b.iter(|| {
                let mut env = Environment::new();
                for i in 0..count {
                    let name = format!("def_{i}");
                    env.add(Declaration::Definition {
                        name: Name::str(&name),
                        univ_params: vec![],
                        ty: Expr::Const(Name::str("Nat"), vec![]),
                        val: nat(i as u64),
                        hint: ReducibilityHint::Regular(1),
                    })
                    .ok();
                }
                env
            });
        });
    }

    group.finish();
}

// ─────────────────────────────────────────────
// Benchmark group: Reducer with caching
// ─────────────────────────────────────────────

fn bench_reducer_cache(c: &mut Criterion) {
    let mut group = c.benchmark_group("reducer_cache");

    // Amortised cost when same expression is reduced many times (cache warm)
    let expr = beta_chain(16);
    group.bench_function("whnf_cache_warm_chain16", |b| {
        let mut reducer = Reducer::new();
        // prime the cache
        let _ = reducer.whnf(&expr);
        b.iter(|| reducer.whnf(black_box(&expr)));
    });

    // Cost with cache cleared between each iter (cold cache)
    group.bench_function("whnf_cache_cold_chain16", |b| {
        b.iter_batched(
            Reducer::new,
            |mut reducer| reducer.whnf(black_box(&expr)),
            criterion::BatchSize::SmallInput,
        );
    });

    // Environment-aware warm cache
    let env = builtin_env();
    let add_expr = nat_add_expr(500, 600);
    group.bench_function("whnf_env_cache_warm_nat_add", |b| {
        let mut reducer = Reducer::new();
        let _ = reducer.whnf_env(&add_expr, &env);
        b.iter(|| reducer.whnf_env(black_box(&add_expr), black_box(&env)));
    });

    group.finish();
}

// ─────────────────────────────────────────────
// Criterion main
// ─────────────────────────────────────────────

criterion_group!(
    benches,
    bench_whnf,
    bench_def_eq,
    bench_infer,
    bench_subst,
    bench_nat_arith,
    bench_normalize,
    bench_alpha,
    bench_env,
    bench_reducer_cache,
);
criterion_main!(benches);
