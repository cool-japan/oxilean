//! Helper functions for formal proof test execution.
//!
//! `run_proof_test` is the main entry point: it parses the source declaration
//! with `oxilean-parse` and then elaborates it with `oxilean-elab`. A `true`
//! return value means both stages succeeded (equivalent to "type-checked OK").
//!
//! The test environment is pre-populated with all standard propositional
//! logic constants (`And`, `Or`, `Not`, `True`, `False`, `Exists`, etc.)
//! as well as the kernel builtins (`Nat`, `Bool`, `Eq`, etc.).

use oxilean_elab::elaborate_decl;
use oxilean_kernel::{init_builtin_env, BinderInfo, Declaration, Environment, Expr, Level, Name};
use oxilean_parse::{Lexer, Parser, TokenKind};

use super::types::{ProofOutcome, ProofSuiteStats, ProofTestCase};

// ──────────────────────────────────────────────────────────────────────────────
// Environment construction
// ──────────────────────────────────────────────────────────────────────────────

/// Convenience: `Type 0` (= Prop in Lean 4).
fn prop() -> Expr {
    Expr::Sort(Level::zero())
}

/// Convenience: `Type 1`.
fn type1() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}

/// Build a Pi type: `(name : dom) -> cod`.
fn mk_pi(name: &str, dom: Expr, cod: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str(name),
        Box::new(dom),
        Box::new(cod),
    )
}

/// Build an implicit Pi type: `{name : dom} -> cod`.
fn mk_ipi(name: &str, dom: Expr, cod: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str(name),
        Box::new(dom),
        Box::new(cod),
    )
}

/// `BVar(0)` — the innermost bound variable.
fn bv0() -> Expr {
    Expr::BVar(0)
}

/// `BVar(1)` — the second innermost bound variable.
fn bv1() -> Expr {
    Expr::BVar(1)
}

/// `BVar(2)`.
fn bv2() -> Expr {
    Expr::BVar(2)
}

/// Build the proof-relevant environment containing:
/// 1. Kernel builtins: `Bool`, `Unit`, `Empty`, `Nat`, `String`, `Eq`, `Prod`, `List`.
/// 2. Propositional logic constants: `True`, `False`, `And`, `Or`, `Not`, `Exists`.
/// 3. Basic inequality symbols reachable via `¬`.
///
/// These are added as axioms where their exact inductive structure is not
/// required for elaboration purposes — the elaborator only needs to resolve
/// the names to types.
pub fn build_proof_env() -> Environment {
    let mut env = Environment::new();

    // Add kernel builtins (Nat, Bool, Eq, Prod, List, String, Unit, Empty).
    if let Err(e) = init_builtin_env(&mut env) {
        // If builtins fail to load, tests will fail with informative messages.
        eprintln!("Warning: init_builtin_env failed: {e}");
    }

    // ── True : Prop ──────────────────────────────────────────────────────────
    let _ = env.add(Declaration::Axiom {
        name: Name::str("True"),
        univ_params: vec![],
        ty: prop(),
    });
    // True.intro : True
    let _ = env.add(Declaration::Axiom {
        name: Name::str("True.intro"),
        univ_params: vec![],
        ty: Expr::Const(Name::str("True"), vec![]),
    });

    // ── False : Prop ─────────────────────────────────────────────────────────
    let _ = env.add(Declaration::Axiom {
        name: Name::str("False"),
        univ_params: vec![],
        ty: prop(),
    });
    // False.elim : {C : Prop} -> False -> C
    let false_elim_ty = mk_ipi(
        "C",
        prop(),
        mk_pi("_", Expr::Const(Name::str("False"), vec![]), bv1()),
    );
    let _ = env.add(Declaration::Axiom {
        name: Name::str("False.elim"),
        univ_params: vec![],
        ty: false_elim_ty,
    });

    // ── Not : Prop -> Prop ───────────────────────────────────────────────────
    // Not p = p -> False
    let not_ty = mk_pi("p", prop(), prop());
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Not"),
        univ_params: vec![],
        ty: not_ty,
    });

    // ── And : Prop -> Prop -> Prop ───────────────────────────────────────────
    let and_ty = mk_pi("p", prop(), mk_pi("q", prop(), prop()));
    let _ = env.add(Declaration::Axiom {
        name: Name::str("And"),
        univ_params: vec![],
        ty: and_ty,
    });
    // And.intro : {p q : Prop} -> p -> q -> And p q
    let and_p = Expr::Const(Name::str("And"), vec![]);
    let and_intro_ty = mk_ipi(
        "p",
        prop(),
        mk_ipi(
            "q",
            prop(),
            mk_pi(
                "hp",
                bv1(),
                mk_pi(
                    "hq",
                    bv1(),
                    Expr::App(
                        Box::new(Expr::App(Box::new(and_p.clone()), Box::new(bv3()))),
                        Box::new(bv2()),
                    ),
                ),
            ),
        ),
    );
    let _ = env.add(Declaration::Axiom {
        name: Name::str("And.intro"),
        univ_params: vec![],
        ty: and_intro_ty,
    });
    // And.left : {p q : Prop} -> And p q -> p
    let and_left_ty = mk_ipi(
        "p",
        prop(),
        mk_ipi(
            "q",
            prop(),
            mk_pi(
                "h",
                Expr::App(
                    Box::new(Expr::App(Box::new(and_p.clone()), Box::new(bv1()))),
                    Box::new(bv0()),
                ),
                bv2(),
            ),
        ),
    );
    let _ = env.add(Declaration::Axiom {
        name: Name::str("And.left"),
        univ_params: vec![],
        ty: and_left_ty,
    });
    // And.right : {p q : Prop} -> And p q -> q
    let and_right_ty = mk_ipi(
        "p",
        prop(),
        mk_ipi(
            "q",
            prop(),
            mk_pi(
                "h",
                Expr::App(
                    Box::new(Expr::App(Box::new(and_p.clone()), Box::new(bv1()))),
                    Box::new(bv0()),
                ),
                bv1(),
            ),
        ),
    );
    let _ = env.add(Declaration::Axiom {
        name: Name::str("And.right"),
        univ_params: vec![],
        ty: and_right_ty,
    });

    // ── Or : Prop -> Prop -> Prop ────────────────────────────────────────────
    let or_ty = mk_pi("p", prop(), mk_pi("q", prop(), prop()));
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Or"),
        univ_params: vec![],
        ty: or_ty,
    });
    // Or.inl : {p q : Prop} -> p -> Or p q
    let or_c = Expr::Const(Name::str("Or"), vec![]);
    let or_inl_ty = mk_ipi(
        "p",
        prop(),
        mk_ipi(
            "q",
            prop(),
            mk_pi(
                "h",
                bv1(),
                Expr::App(
                    Box::new(Expr::App(Box::new(or_c.clone()), Box::new(bv2()))),
                    Box::new(bv1()),
                ),
            ),
        ),
    );
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Or.inl"),
        univ_params: vec![],
        ty: or_inl_ty,
    });
    // Or.inr : {p q : Prop} -> q -> Or p q
    let or_inr_ty = mk_ipi(
        "p",
        prop(),
        mk_ipi(
            "q",
            prop(),
            mk_pi(
                "h",
                bv0(),
                Expr::App(
                    Box::new(Expr::App(Box::new(or_c.clone()), Box::new(bv2()))),
                    Box::new(bv1()),
                ),
            ),
        ),
    );
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Or.inr"),
        univ_params: vec![],
        ty: or_inr_ty,
    });
    // Or.elim : {p q r : Prop} -> Or p q -> (p -> r) -> (q -> r) -> r
    let or_elim_ty = mk_ipi(
        "p",
        prop(),
        mk_ipi(
            "q",
            prop(),
            mk_ipi(
                "r",
                prop(),
                mk_pi(
                    "h",
                    Expr::App(
                        Box::new(Expr::App(Box::new(or_c.clone()), Box::new(bv2()))),
                        Box::new(bv1()),
                    ),
                    mk_pi(
                        "hl",
                        mk_pi("_", bv3(), bv3()),
                        mk_pi("hr", mk_pi("_", bv2(), bv3()), bv3()),
                    ),
                ),
            ),
        ),
    );
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Or.elim"),
        univ_params: vec![],
        ty: or_elim_ty,
    });

    // ── Exists : {α : Type} -> (α -> Prop) -> Prop ───────────────────────────
    // Exists.{u} : {α : Sort u} -> (α -> Prop) -> Prop
    let exists_ty = mk_ipi(
        "alpha",
        type1(),
        mk_pi("p", mk_pi("_", bv0(), prop()), prop()),
    );
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Exists"),
        univ_params: vec![],
        ty: exists_ty,
    });

    // ── Iff : Prop -> Prop -> Prop ───────────────────────────────────────────
    let iff_ty = mk_pi("p", prop(), mk_pi("q", prop(), prop()));
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Iff"),
        univ_params: vec![],
        ty: iff_ty,
    });

    // ── Classical.em : (p : Prop) -> Or p (Not p) ───────────────────────────
    let not_c = Expr::Const(Name::str("Not"), vec![]);
    let or_c2 = Expr::Const(Name::str("Or"), vec![]);
    let em_ty = mk_pi(
        "p",
        prop(),
        Expr::App(
            Box::new(Expr::App(Box::new(or_c2), Box::new(bv0()))),
            Box::new(Expr::App(Box::new(not_c), Box::new(bv0()))),
        ),
    );
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Classical.em"),
        univ_params: vec![],
        ty: em_ty,
    });

    // ── sorry : {α : Sort u} -> α ────────────────────────────────────────────
    // `sorry` is the "placeholder proof" axiom. We register it as an axiom
    // of type `Prop` so the elaborator can resolve the name during proof
    // elaboration.  The precise type of `sorry` is universe-polymorphic in
    // Lean 4, but registering it at `Prop` is sufficient for our tests since
    // all our `sorry` uses are as proofs of Prop-kinded goals.
    let sorry_ty = mk_ipi("alpha", prop(), bv0());
    let _ = env.add(Declaration::Axiom {
        name: Name::str("sorry"),
        univ_params: vec![],
        ty: sorry_ty,
    });

    env
}

/// `BVar(3)`.
fn bv3() -> Expr {
    Expr::BVar(3)
}

// ──────────────────────────────────────────────────────────────────────────────
// Parsing helper
// ──────────────────────────────────────────────────────────────────────────────

/// Parse a single OxiLean surface-syntax declaration.
///
/// Returns `Ok(located_decl)` on success, `Err(error_string)` on failure.
fn parse_source(src: &str) -> Result<oxilean_parse::Located<oxilean_parse::Decl>, String> {
    let mut lexer = Lexer::new(src);
    let tokens = lexer.tokenize();
    // Check for lex errors first.
    for tok in &tokens {
        if let TokenKind::Error(ref msg) = tok.kind {
            return Err(format!("lex error: {msg}"));
        }
    }
    let mut parser = Parser::new(tokens);
    parser
        .parse_decl()
        .map_err(|e| format!("parse error: {e:?}"))
}

// ──────────────────────────────────────────────────────────────────────────────
// Core test runner
// ──────────────────────────────────────────────────────────────────────────────

/// Run a single proof test case.
///
/// The function attempts to:
/// 1. Lex and parse the source declaration.
/// 2. Elaborate the parsed declaration in a pre-populated environment.
///
/// Returns a `ProofOutcome` describing the result.
pub fn run_proof_test(case: &ProofTestCase) -> ProofOutcome {
    match parse_source(case.source) {
        Err(msg) => ProofOutcome {
            case_name: case.name,
            parse_ok: false,
            elab_ok: false,
            error: Some(msg),
        },
        Ok(located) => {
            let env = build_proof_env();
            match elaborate_decl(&env, &located.value) {
                Ok(_pending) => ProofOutcome {
                    case_name: case.name,
                    parse_ok: true,
                    elab_ok: true,
                    error: None,
                },
                Err(e) => ProofOutcome {
                    case_name: case.name,
                    parse_ok: true,
                    elab_ok: false,
                    error: Some(format!("elab error: {e:?}")),
                },
            }
        }
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Suite helpers
// ──────────────────────────────────────────────────────────────────────────────

/// Run an entire suite of proof test cases, collecting statistics.
///
/// Each case is run independently with a fresh environment.
pub fn run_proof_suite(cases: &[ProofTestCase]) -> ProofSuiteStats {
    let mut stats = ProofSuiteStats::default();
    for case in cases {
        let outcome = run_proof_test(case);
        stats.total += 1;
        if outcome.parse_ok {
            stats.parse_ok += 1;
        }
        if outcome.elab_ok {
            stats.elab_ok += 1;
        }
        if !outcome.success() {
            let err = outcome.error.unwrap_or_else(|| "unknown".to_string());
            stats.failures.push((case.name.to_string(), err));
        }
    }
    stats
}

/// Print a suite statistics report to stdout.
pub fn print_suite_stats(suite_name: &str, stats: &ProofSuiteStats) {
    println!(
        "\n=== Formal Proofs Suite: {suite_name} ===\n\
         Total: {}\n\
         Parse OK: {} ({:.1}%)\n\
         Elab  OK: {} ({:.1}%)\n\
         Failures: {}",
        stats.total,
        stats.parse_ok,
        stats.parse_rate(),
        stats.elab_ok,
        stats.elab_rate(),
        stats.failures.len(),
    );
    if !stats.failures.is_empty() {
        println!("  Failed cases:");
        for (name, err) in &stats.failures {
            let short_err: String = err.chars().take(120).collect();
            println!("    [{name}] {short_err}");
        }
    }
}

/// Assert that a suite of proof test cases all pass, printing stats on failure.
///
/// This function collects all failures and panics with a summary rather than
/// failing on the first case, providing better diagnostics.
pub fn assert_suite_passes(suite_name: &str, cases: &[ProofTestCase]) {
    let stats = run_proof_suite(cases);
    print_suite_stats(suite_name, &stats);
    if !stats.failures.is_empty() {
        panic!(
            "Formal proofs suite '{suite_name}' had {}/{} failures",
            stats.failures.len(),
            stats.total,
        );
    }
}

/// Assert that at least `min_pass` of the cases pass.
///
/// Useful for suites that test features which may be partially implemented.
pub fn assert_suite_passes_at_least(suite_name: &str, cases: &[ProofTestCase], min_pass: usize) {
    let stats = run_proof_suite(cases);
    print_suite_stats(suite_name, &stats);
    assert!(
        stats.elab_ok >= min_pass,
        "Suite '{suite_name}': expected at least {min_pass} proofs to pass, got {}",
        stats.elab_ok,
    );
}
