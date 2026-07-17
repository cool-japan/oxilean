//! Regression tests for universe-parameter hygiene and constant-reference
//! universe arity (audit items S7 + S8).
//!
//! * S7 — `infer_const` must reject a level list whose length differs from the
//!   referenced constant's universe-parameter arity (including the previously
//!   silently-accepted empty-vs-nonempty case, which let a polymorphic
//!   constant's `Param(u)` leak uninstantiated).
//! * S8 — both `check_declaration` **and** `check_constant_info` must reject
//!   duplicate universe-parameter names, undeclared `Param`s appearing in the
//!   type/value, and any `Level::MVar`.

#![allow(clippy::all)]

use oxilean_kernel::check::{check_constant_info, check_declaration};
use oxilean_kernel::Node;
use oxilean_kernel::{
    AxiomVal, BinderInfo, ConstantInfo, ConstantVal, Declaration, DefinitionSafety, DefinitionVal,
    Environment, Expr, InductiveVal, Level, LevelMVarId, Name, ReducibilityHint, TheoremVal,
};

fn type_u(u: &Level) -> Expr {
    // Sort (succ u) = "Type u".
    Expr::Sort(Level::succ(u.clone()))
}

/// Declare `axiom Poly.{u} : Sort u` and return the populated environment.
fn env_with_poly_axiom() -> Environment {
    let mut env = Environment::new();
    let u = Level::param(Name::str("u"));
    let ax = Declaration::Axiom {
        name: Name::str("Poly"),
        univ_params: vec![Name::str("u")],
        ty: Expr::Sort(u),
    };
    check_declaration(&mut env, ax).expect("axiom Poly.{u} : Sort u should check");
    env
}

// ── S7: constant-reference universe arity ────────────────────────────────────

#[test]
fn s7_polymorphic_const_referenced_with_empty_levels_is_rejected() {
    // `def Bad : Type 0 := Poly` where `Poly` needs one universe argument but is
    // referenced with an empty level list. Previously the uninstantiated type
    // `Sort u` (with a free `u`) was returned silently; now it must error.
    let mut env = env_with_poly_axiom();
    let bad = Declaration::Definition {
        name: Name::str("Bad"),
        univ_params: vec![],
        ty: Expr::Sort(Level::succ(Level::zero())),
        val: Expr::Const(Name::str("Poly"), vec![]), // <-- missing the level arg
        hint: ReducibilityHint::Regular(1),
    };
    let err = check_declaration(&mut env, bad).expect_err("empty level list must be rejected");
    let msg = format!("{}", err);
    assert!(
        msg.contains("universe parameter count mismatch"),
        "unexpected error: {msg}"
    );
}

#[test]
fn s7_polymorphic_const_referenced_with_too_many_levels_is_rejected() {
    let mut env = env_with_poly_axiom();
    let bad = Declaration::Definition {
        name: Name::str("Bad2"),
        univ_params: vec![Name::str("v")],
        ty: Expr::Sort(Level::succ(Level::zero())),
        val: Expr::Const(
            Name::str("Poly"),
            vec![Level::zero(), Level::param(Name::str("v"))], // 2 levels, arity 1
        ),
        hint: ReducibilityHint::Regular(1),
    };
    let err = check_declaration(&mut env, bad).expect_err("arity 2 vs 1 must be rejected");
    assert!(format!("{}", err).contains("universe parameter count mismatch"));
}

#[test]
fn s7_correct_arity_reference_is_accepted() {
    // `Poly.{u} : Sort u`, so `Poly.{0} : Sort 0`. Declare `def Good : Sort 0 :=
    // Poly.{0}` — exactly one level, matching arity, correct declared type.
    let mut env = env_with_poly_axiom();
    let good = Declaration::Definition {
        name: Name::str("Good"),
        univ_params: vec![],
        ty: Expr::Sort(Level::zero()),
        val: Expr::Const(Name::str("Poly"), vec![Level::zero()]),
        hint: ReducibilityHint::Regular(1),
    };
    check_declaration(&mut env, good).expect("correct-arity reference must check");
    assert!(env.contains(&Name::str("Good")));
}

// ── S8: universe-parameter hygiene ───────────────────────────────────────────

#[test]
fn s8_duplicate_univ_params_rejected() {
    let mut env = Environment::new();
    let u = Level::param(Name::str("u"));
    let decl = Declaration::Axiom {
        name: Name::str("Dup"),
        univ_params: vec![Name::str("u"), Name::str("u")], // duplicate
        ty: Expr::Sort(u),
    };
    let err = check_declaration(&mut env, decl).expect_err("duplicate univ param must be rejected");
    assert!(format!("{}", err).contains("duplicate universe parameter"));
}

#[test]
fn s8_undeclared_param_in_type_rejected() {
    // Type mentions `Sort v` but only `u` is declared.
    let mut env = Environment::new();
    let v = Level::param(Name::str("v"));
    let decl = Declaration::Axiom {
        name: Name::str("Undeclared"),
        univ_params: vec![Name::str("u")],
        ty: Expr::Sort(v),
    };
    let err =
        check_declaration(&mut env, decl).expect_err("undeclared param in type must be rejected");
    assert!(format!("{}", err).contains("undeclared universe parameter"));
}

#[test]
fn s8_undeclared_param_in_value_rejected() {
    let mut env = Environment::new();
    let u = Level::param(Name::str("u"));
    // ty is fine (uses u), but the value mentions `Sort (succ v)` with v undeclared.
    let decl = Declaration::Definition {
        name: Name::str("ValLeak"),
        univ_params: vec![Name::str("u")],
        ty: type_u(&u),
        val: type_u(&Level::param(Name::str("v"))),
        hint: ReducibilityHint::Regular(1),
    };
    let err =
        check_declaration(&mut env, decl).expect_err("undeclared param in value must be rejected");
    assert!(format!("{}", err).contains("undeclared universe parameter"));
}

#[test]
fn s8_level_mvar_rejected() {
    let mut env = Environment::new();
    let decl = Declaration::Axiom {
        name: Name::str("HasMVar"),
        univ_params: vec![],
        ty: Expr::Sort(Level::mvar(LevelMVarId(0))),
    };
    let err = check_declaration(&mut env, decl).expect_err("level mvar must be rejected");
    assert!(format!("{}", err).contains("universe metavariable"));
}

#[test]
fn s8_wellformed_polymorphic_declaration_accepted() {
    // `def Endo.{u} : Sort u -> Sort u := fun a => a -> a` — all params declared,
    // no duplicates, no mvars: hygiene passes and it typechecks end-to-end.
    let mut env = Environment::new();
    let u = Level::param(Name::str("u"));
    let sort_u = Expr::Sort(u);
    let ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("a"),
        Node::new(sort_u.clone()),
        Node::new(sort_u.clone()),
    );
    let val = Expr::Lam(
        BinderInfo::Default,
        Name::str("a"),
        Node::new(sort_u),
        Node::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Node::new(Expr::BVar(0)),
            Node::new(Expr::BVar(1)),
        )),
    );
    let decl = Declaration::Definition {
        name: Name::str("Endo"),
        univ_params: vec![Name::str("u")],
        ty,
        val,
        hint: ReducibilityHint::Regular(1),
    };
    check_declaration(&mut env, decl).expect("well-formed polymorphic def must check");
    assert!(env.contains(&Name::str("Endo")));
}

// ── S8 on the `check_constant_info` path (Wave-3) ────────────────────────────
//
// The Wave-2 S8 check ran only on `check_declaration`; the richer
// `ConstantInfo` path (`Inductive`/`Constructor`/`Recursor`/`Quotient` plus
// `Axiom`/`Definition`/`Theorem`/`Opaque`) must reject the same three
// violations. Hygiene runs before the per-variant checks, so a malformed
// universe-parameter list is caught regardless of the variant's other fields.

fn common(name: &str, level_params: Vec<Name>, ty: Expr) -> ConstantVal {
    ConstantVal {
        name: Name::str(name),
        level_params,
        ty,
    }
}

#[test]
fn s8_ci_axiom_duplicate_univ_params_rejected() {
    let mut env = Environment::new();
    let u = Level::param(Name::str("u"));
    let ci = ConstantInfo::Axiom(AxiomVal {
        common: common("DupAx", vec![Name::str("u"), Name::str("u")], Expr::Sort(u)),
        is_unsafe: false,
    });
    let err = check_constant_info(&mut env, ci)
        .expect_err("duplicate univ param on Axiom must be rejected");
    assert!(format!("{}", err).contains("duplicate universe parameter"));
}

#[test]
fn s8_ci_definition_duplicate_univ_params_rejected() {
    let mut env = Environment::new();
    let u = Level::param(Name::str("u"));
    let ci = ConstantInfo::Definition(DefinitionVal {
        common: common("DupDef", vec![Name::str("u"), Name::str("u")], type_u(&u)),
        value: type_u(&u),
        hints: ReducibilityHint::Regular(1),
        safety: DefinitionSafety::Safe,
        all: vec![],
    });
    let err = check_constant_info(&mut env, ci)
        .expect_err("duplicate univ param on Definition must be rejected");
    assert!(format!("{}", err).contains("duplicate universe parameter"));
}

#[test]
fn s8_ci_theorem_duplicate_univ_params_rejected() {
    let mut env = Environment::new();
    let u = Level::param(Name::str("u"));
    let ci = ConstantInfo::Theorem(TheoremVal {
        common: common(
            "DupThm",
            vec![Name::str("u"), Name::str("u")],
            Expr::Sort(u),
        ),
        value: Expr::Sort(Level::zero()),
        all: vec![],
    });
    let err = check_constant_info(&mut env, ci)
        .expect_err("duplicate univ param on Theorem must be rejected");
    assert!(format!("{}", err).contains("duplicate universe parameter"));
}

#[test]
fn s8_ci_inductive_duplicate_univ_params_rejected() {
    let mut env = Environment::new();
    let u = Level::param(Name::str("u"));
    let ci = ConstantInfo::Inductive(InductiveVal {
        common: common(
            "DupInd",
            vec![Name::str("u"), Name::str("u")],
            Expr::Sort(u),
        ),
        num_params: 0,
        num_indices: 0,
        all: vec![Name::str("DupInd")],
        ctors: vec![],
        num_nested: 0,
        is_rec: false,
        is_unsafe: false,
        is_reflexive: false,
        is_prop: false,
    });
    let err = check_constant_info(&mut env, ci)
        .expect_err("duplicate univ param on Inductive must be rejected");
    assert!(format!("{}", err).contains("duplicate universe parameter"));
}

#[test]
fn s8_ci_undeclared_param_in_type_rejected() {
    // Inductive whose type mentions `Sort v` but only declares `u`.
    let mut env = Environment::new();
    let v = Level::param(Name::str("v"));
    let ci = ConstantInfo::Inductive(InductiveVal {
        common: common("UndeclInd", vec![Name::str("u")], Expr::Sort(v)),
        num_params: 0,
        num_indices: 0,
        all: vec![Name::str("UndeclInd")],
        ctors: vec![],
        num_nested: 0,
        is_rec: false,
        is_unsafe: false,
        is_reflexive: false,
        is_prop: false,
    });
    let err = check_constant_info(&mut env, ci)
        .expect_err("undeclared param in ConstantInfo type must be rejected");
    assert!(format!("{}", err).contains("undeclared universe parameter"));
}

#[test]
fn s8_ci_undeclared_param_in_value_rejected() {
    // Definition whose value leaks an undeclared `v`.
    let mut env = Environment::new();
    let u = Level::param(Name::str("u"));
    let ci = ConstantInfo::Definition(DefinitionVal {
        common: common("ValLeakCi", vec![Name::str("u")], type_u(&u)),
        value: type_u(&Level::param(Name::str("v"))),
        hints: ReducibilityHint::Regular(1),
        safety: DefinitionSafety::Safe,
        all: vec![],
    });
    let err = check_constant_info(&mut env, ci)
        .expect_err("undeclared param in ConstantInfo value must be rejected");
    assert!(format!("{}", err).contains("undeclared universe parameter"));
}

#[test]
fn s8_ci_level_mvar_rejected() {
    let mut env = Environment::new();
    let ci = ConstantInfo::Axiom(AxiomVal {
        common: common("MVarCi", vec![], Expr::Sort(Level::mvar(LevelMVarId(0)))),
        is_unsafe: false,
    });
    let err =
        check_constant_info(&mut env, ci).expect_err("level mvar on ConstantInfo must be rejected");
    assert!(format!("{}", err).contains("universe metavariable"));
}
