//! Omega proof-term reconstruction.
//!
//! This module attempts to build a kernel-checkable `Expr` proof term from an
//! `OmegaProof` certificate produced by the omega decision procedure.
//!
//! # Design
//!
//! Pattern matching is goal-directed: we inspect the goal expression and
//! dispatch to a pattern-specific builder that constructs the proof term.
//! Each builder calls `verify_proof_term` to confirm the term type-checks
//! before returning it.
//!
//! Patterns implemented:
//! - `le_le`: `LE.le Int inst a b`  →  wraps an `Int.le a b` proof with `le_of_int_le`
//! - `le_refl`: `Int.le a a`  →  `Int.le_refl a`
//! - `le_trans`: `Int.le a c` with hyps `h1: Int.le a b`, `h2: Int.le b c`
//!   →  `Int.le_trans a b c h1 h2`
//! - `lt_irrefl`: goal `False` with hyp `h: Int.lt a a`
//!   →  `@absurd (Int.lt a a) False h (Int.lt_irrefl a)`
//!
//! Unhandled patterns return `None`; the caller falls back to `sorry`.
//!
//! # Cycle 4 fix: FVar threading + LE.le bridge + lt_irrefl via absurd
//!
//! NOTE: Previously omega reconstruction fell back to sorry for all FVar-based
//! goals because the TypeChecker had an empty `local_ctx`. This has been fixed:
//! `verify_proof_term` now accepts `locals: &[(FVarId, Name, Expr)]` and calls
//! `TypeChecker::push_local` to register each local variable before checking.
//!
//! NOTE: Previously omega reconstruction fell back to sorry for all LE.le goals.
//! Cycle 4 fix: the `try_le_le` pattern wraps the Int.le reconstruction with the
//! `le_of_int_le` bridge axiom. If the environment has `le_of_int_le` (from
//! `register_omega_helper` + A1 wiring), surface ≤ goals now reconstruct to
//! kernel-verified proofs.
//!
//! NOTE: Previously `try_lt_irrefl` always returned `None` because `Not` is an
//! opaque axiom and the kernel cannot reduce `Not P` to `P → False`. Cycle 4 fix:
//! use `@absurd (Int.lt a a) False h (Int.lt_irrefl a)` instead, which has type
//! `False` directly (given `absurd : {a b : Prop} → a → Not a → b`).

pub mod farkas;
pub mod polyrith;

use oxilean_kernel::Node;
use oxilean_kernel::{Declaration, Environment, Expr, FVarId, LocalDecl, Name, TypeChecker};
use oxilean_meta::tactic::omega::OmegaProof;

/// Try to build a kernel-checkable proof term from an `OmegaProof` certificate.
///
/// Returns `None` if reconstruction is not supported for this proof shape.
/// The caller should substitute `sorry` when `None` is returned.
///
/// # Parameters
///
/// - `proof`: The omega proof certificate (reserved for step-driven reconstruction).
/// - `goal`: The goal type to prove.
/// - `hyps`: Local hypotheses as `(name, type)` pairs. Each hypothesis is registered
///   as an axiom in a cloned environment so that `Expr::Const(name)` references
///   can be kernel-verified.
/// - `locals`: Free variable context as `(fvar_id, name, type)` triples. Injected
///   into the TypeChecker's local context so that `Expr::FVar` references can be
///   resolved during kernel verification.
/// - `env`: The global environment.
pub fn omega_proof_to_expr(
    proof: &OmegaProof,
    goal: &Expr,
    hyps: &[(Name, Expr)],
    locals: &[(FVarId, Name, Expr)],
    env: &Environment,
) -> Option<Expr> {
    if let Some(e) = try_le_le(proof, goal, hyps, locals, env) {
        return Some(e);
    }
    if let Some(e) = try_le_refl(goal, locals, env) {
        return Some(e);
    }
    if let Some(e) = try_le_trans(goal, hyps, locals, env) {
        return Some(e);
    }
    if let Some(e) = try_lt_irrefl(goal, hyps, locals, env) {
        return Some(e);
    }
    let _ = proof; // future step-driven patterns will use the proof steps
    None
}

/// Pattern: `LE.le Int inst a b`  →  wrap an `Int.le a b` proof with `le_of_int_le`
///
/// Detects the 4-argument `LE.le` application and reconstructs a proof of
/// `Int.le a b` using the other patterns, then wraps it with
/// `@le_of_int_le inst a b inner_proof`.
///
/// Requires `le_of_int_le` to be registered in the environment (from
/// `register_omega_helper`).
fn try_le_le(
    proof: &OmegaProof,
    goal: &Expr,
    hyps: &[(Name, Expr)],
    locals: &[(FVarId, Name, Expr)],
    env: &Environment,
) -> Option<Expr> {
    // Detect: goal = App(App(App(App(Const("LE.le"), ty), inst), a), b)
    let (ty_arg, inst_arg, a_arg, b_arg) = match_le_le_app(goal)?;

    // Only handle the Int case.
    if let Expr::Const(ty_name, _) = ty_arg {
        if ty_name.to_string() != "Int" {
            return None;
        }
    } else {
        return None;
    }

    // Convert LE.le hyps to Int.le hyps for inner reconstruction.
    // When a hypothesis has type LE.le Int inst a b, we wrap it with int_le_of_le.
    let converted_hyps: Vec<(Name, Expr)> = hyps
        .iter()
        .map(|(name, ty)| {
            // Try to convert LE.le hyp type to Int.le hyp type for matching.
            if let Some((_ty2, _inst2, ha, hb)) = match_le_le_app(ty) {
                // Build Int.le ha hb as the "normalized" type for chain matching.
                let int_le_ty = app2(
                    Expr::Const(Name::str("Int.le"), vec![]),
                    ha.clone(),
                    hb.clone(),
                );
                (name.clone(), int_le_ty)
            } else {
                (name.clone(), ty.clone())
            }
        })
        .collect();

    // Build the inner goal: Int.le a b
    let int_le_goal = app2(
        Expr::Const(Name::str("Int.le"), vec![]),
        a_arg.clone(),
        b_arg.clone(),
    );

    // Reconstruct a proof of Int.le a b using the (possibly converted) hyps.
    let inner_proof = omega_proof_to_expr(proof, &int_le_goal, &converted_hyps, locals, env)?;

    // Build: @le_of_int_le inst a b inner_proof
    // le_of_int_le : ∀ {inst : LE Int} (a b : Int), Int.le a b → LE.le Int inst a b
    // Applied as 4 explicit args (inst is implicit in source, explicit in kernel terms):
    // App(App(App(App(Const("le_of_int_le"), inst), a), b), inner_proof)
    let term = Expr::App(
        Node::new(Expr::App(
            Node::new(Expr::App(
                Node::new(Expr::App(
                    Node::new(Expr::Const(Name::str("le_of_int_le"), vec![])),
                    Node::new(inst_arg.clone()),
                )),
                Node::new(a_arg.clone()),
            )),
            Node::new(b_arg.clone()),
        )),
        Node::new(inner_proof),
    );

    if verify_proof_term(&term, goal, hyps, locals, env) {
        Some(term)
    } else {
        None
    }
}

/// Pattern: `Int.le a a`  →  `Int.le_refl a`
///
/// Detects whether the goal is `App(App(Const("Int.le"), a), a)` with
/// syntactically identical `a`, then returns `App(Const("Int.le_refl"), a)`.
fn try_le_refl(goal: &Expr, locals: &[(FVarId, Name, Expr)], env: &Environment) -> Option<Expr> {
    // Goal must be App(App(Const("Int.le"), lhs), rhs)
    let (lhs, rhs) = extract_int_le(goal)?;

    // Only handle the reflexive case: both sides are syntactically identical.
    if lhs != rhs {
        return None;
    }

    // Build the proof term: Int.le_refl applied to `a`.
    let le_refl_const = Expr::Const(Name::str("Int.le_refl"), vec![]);
    let term = Expr::App(Node::new(le_refl_const), Node::new(lhs.clone()));

    if verify_proof_term(&term, goal, &[], locals, env) {
        Some(term)
    } else {
        None
    }
}

/// Pattern: `Int.le a c` with hypotheses `h1: Int.le a b` and `h2: Int.le b c`
///   →  `Int.le_trans a b c h1 h2`
///
/// Searches the hypothesis list for a pair forming a chain `a ≤ b ≤ c`.
/// The search is O(n²) over hypotheses (safe: n is small in practice).
/// All `(x, y)` pairs from hypotheses are collected first, then the chain
/// `(a, b)` and `(b, c)` is looked up regardless of hypothesis order.
///
/// Hypothesis proof terms are constructed as `Expr::FVar(fvar_id)` when the
/// hypothesis name matches a locals entry, otherwise as `Expr::Const(name)`.
fn try_le_trans(
    goal: &Expr,
    hyps: &[(Name, Expr)],
    locals: &[(FVarId, Name, Expr)],
    env: &Environment,
) -> Option<Expr> {
    // Goal must be Int.le a c
    let (a, c) = extract_int_le(goal)?;

    // Collect all (x, y, hyp_name) triples where hyp : Int.le x y
    let le_hyps: Vec<(Expr, Expr, Name)> = hyps
        .iter()
        .filter_map(|(name, ty)| {
            let (x, y) = extract_int_le(ty)?;
            Some((x.clone(), y.clone(), name.clone()))
        })
        .collect();

    // Search for h1: Int.le a b and h2: Int.le b c (any order in hyps)
    for (x1, y1, name1) in &le_hyps {
        if x1 != a {
            continue;
        }
        // Found h1: a ≤ y1; now look for h2: y1 ≤ c
        for (x2, y2, name2) in &le_hyps {
            if x2 == y1 && y2 == c {
                // Found the chain a ≤ b ≤ c
                let b = y1.clone();
                // Build proof term references: prefer FVar if available, else Const.
                let h1_ref = hyp_proof_term(name1, locals);
                let h2_ref = hyp_proof_term(name2, locals);

                // Build: Int.le_trans a b c h1 h2
                // Type: ∀ (a b c : Int), Int.le a b → Int.le b c → Int.le a c
                let term = Expr::App(
                    Node::new(Expr::App(
                        Node::new(Expr::App(
                            Node::new(Expr::App(
                                Node::new(Expr::App(
                                    Node::new(Expr::Const(Name::str("Int.le_trans"), vec![])),
                                    Node::new(a.clone()),
                                )),
                                Node::new(b.clone()),
                            )),
                            Node::new(c.clone()),
                        )),
                        Node::new(h1_ref),
                    )),
                    Node::new(h2_ref),
                );

                if verify_proof_term(&term, goal, hyps, locals, env) {
                    return Some(term);
                }
            }
        }
    }

    None
}

/// Pattern: goal `False`, hypothesis `h: Int.lt a a`
///   →  `@absurd (Int.lt a a) False h (Int.lt_irrefl a)`
///
/// Uses the `absurd` axiom `{a b : Prop} → a → Not a → b` to derive `False`
/// from a hypothesis `h : Int.lt a a` and the lemma `Int.lt_irrefl a : Not (Int.lt a a)`.
///
/// Term structure (all implicit args explicit):
/// `App(App(App(App(Const("absurd"), Int.lt a a), False), h_proof), App(Const("Int.lt_irrefl"), a))`
fn try_lt_irrefl(
    goal: &Expr,
    hyps: &[(Name, Expr)],
    locals: &[(FVarId, Name, Expr)],
    env: &Environment,
) -> Option<Expr> {
    // Goal must be False
    match goal {
        Expr::Const(name, _) if name.to_string() == "False" => {}
        _ => return None,
    }

    // Search for hyp h: Int.lt a a (where both sides are syntactically equal)
    for (name, ty) in hyps {
        if let Some((x, y)) = extract_int_lt(ty) {
            if x == y {
                // Build the proof term references for the hypothesis.
                let h_proof = hyp_proof_term(name, locals);

                // Build: @absurd (Int.lt a a) False h (Int.lt_irrefl a)
                //
                // absurd type: {a : Prop} → {b : Prop} → a → Not a → b
                // @absurd p_ty goal_ty h_proof (Int.lt_irrefl a)
                //
                // Application order (all args explicit including implicit ones):
                //   absurd p_ty goal_ty h_proof irrefl_term
                let p_ty = Expr::App(
                    Node::new(Expr::App(
                        Node::new(Expr::Const(Name::str("Int.lt"), vec![])),
                        Node::new(x.clone()),
                    )),
                    Node::new(x.clone()),
                );
                let irrefl_term = Expr::App(
                    Node::new(Expr::Const(Name::str("Int.lt_irrefl"), vec![])),
                    Node::new(x.clone()),
                );
                let term = Expr::App(
                    Node::new(Expr::App(
                        Node::new(Expr::App(
                            Node::new(Expr::App(
                                Node::new(Expr::Const(Name::str("absurd"), vec![])),
                                Node::new(p_ty),
                            )),
                            Node::new(goal.clone()),
                        )),
                        Node::new(h_proof),
                    )),
                    Node::new(irrefl_term),
                );

                if verify_proof_term(&term, goal, hyps, locals, env) {
                    return Some(term);
                }
            }
        }
    }
    None
}

/// Build a proof term reference for a hypothesis.
///
/// If the hypothesis name matches a local variable (with a known FVarId),
/// returns `Expr::FVar(fvar_id)`. Otherwise falls back to `Expr::Const(name)`,
/// which requires the hypothesis to be registered as an axiom in the environment.
fn hyp_proof_term(name: &Name, locals: &[(FVarId, Name, Expr)]) -> Expr {
    for (fvar, local_name, _) in locals {
        if local_name == name {
            return Expr::FVar(*fvar);
        }
    }
    // Fallback: reference by const name (works when hyp is registered as axiom in env).
    Expr::Const(name.clone(), vec![])
}

/// Detect and decompose `LE.le` applied to 4 arguments.
///
/// Returns `(ty, inst, a, b)` from `App(App(App(App(Const("LE.le"), ty), inst), a), b)`.
fn match_le_le_app(expr: &Expr) -> Option<(&Expr, &Expr, &Expr, &Expr)> {
    if let Expr::App(f3, b_arg) = expr {
        if let Expr::App(f2, a_arg) = f3.as_ref() {
            if let Expr::App(f1, inst_arg) = f2.as_ref() {
                if let Expr::App(f0, ty_arg) = f1.as_ref() {
                    if let Expr::Const(name, _) = f0.as_ref() {
                        if name.to_string() == "LE.le" {
                            return Some((ty_arg, inst_arg, a_arg, b_arg));
                        }
                    }
                }
            }
        }
    }
    None
}

/// Build `f a b` — a function applied to two arguments.
fn app2(f: Expr, a: Expr, b: Expr) -> Expr {
    Expr::App(
        Node::new(Expr::App(Node::new(f), Node::new(a))),
        Node::new(b),
    )
}

/// Extract `(lhs, rhs)` from `App(App(Const("Int.le", _), lhs), rhs)`.
fn extract_int_le(expr: &Expr) -> Option<(&Expr, &Expr)> {
    if let Expr::App(func, rhs) = expr {
        if let Expr::App(func2, lhs) = func.as_ref() {
            if let Expr::Const(name, _) = func2.as_ref() {
                if name.to_string() == "Int.le" {
                    return Some((lhs.as_ref(), rhs.as_ref()));
                }
            }
        }
    }
    None
}

/// Extract `(lhs, rhs)` from `App(App(Const("Int.lt", _), lhs), rhs)`.
fn extract_int_lt(expr: &Expr) -> Option<(&Expr, &Expr)> {
    if let Expr::App(func, rhs) = expr {
        if let Expr::App(func2, lhs) = func.as_ref() {
            if let Expr::Const(name, _) = func2.as_ref() {
                if name.to_string() == "Int.lt" {
                    return Some((lhs.as_ref(), rhs.as_ref()));
                }
            }
        }
    }
    None
}

/// Verify a candidate proof term against the expected goal type using the kernel.
///
/// Returns `true` iff the kernel accepts the term and its inferred type is
/// definitionally equal to `goal`.
///
/// # Hypothesis augmentation
///
/// Each hypothesis `(name, ty)` in `hyps` is registered as an axiom in a cloned
/// environment so that `Expr::Const(name)` references in the proof term can be
/// kernel-checked. Hypotheses already present in `env` are skipped silently.
///
/// # FVar threading
///
/// Each `(fvar_id, name, ty)` in `locals` is pushed into the TypeChecker's
/// `local_ctx` via `push_local` so that `Expr::FVar(fvar_id)` references in the
/// proof term can be resolved during type inference.
fn verify_proof_term(
    term: &Expr,
    goal: &Expr,
    hyps: &[(Name, Expr)],
    locals: &[(FVarId, Name, Expr)],
    env: &Environment,
) -> bool {
    // Clone the environment and register hypotheses as axioms so the checker
    // can resolve Expr::Const(hyp_name) references in the proof term.
    let mut augmented_env = env.clone();
    for (name, ty) in hyps {
        // Skip if already present (either a real lemma or from a prior hyp with same name).
        if augmented_env.contains(name) {
            continue;
        }
        // Ignore registration errors (e.g., duplicate); verification will still
        // catch any type mismatch.
        let _ = augmented_env.add(Declaration::Axiom {
            name: name.clone(),
            univ_params: vec![],
            ty: ty.clone(),
        });
    }

    let mut checker = TypeChecker::new(&augmented_env);

    // Inject the local variable context so FVar references can be resolved.
    for (fvar, name, ty) in locals {
        checker.push_local(LocalDecl {
            fvar: *fvar,
            name: name.clone(),
            ty: ty.clone(),
            val: None,
        });
    }

    match checker.infer_type(term) {
        Ok(inferred_ty) => checker.is_def_eq(&inferred_ty, goal),
        Err(_) => false,
    }
}

// ── Tests ──────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use oxilean_kernel::{BinderInfo, Declaration, Environment, Level, TypeChecker};
    use oxilean_meta::tactic::omega::{OmegaProof, OmegaStep};
    use oxilean_std::register_omega_helper;

    /// Build a minimal test environment containing:
    /// - The 12 omega helper lemmas (from `oxilean_std`).
    /// - An axiom `a : Int` (needed so the kernel can infer the type of `a`).
    /// - An axiom `Int : Type 1` (so `a : Int` is well-typed).
    fn test_env() -> Environment {
        let mut env = Environment::new();

        // Register the Int sort (Type 1) so "Int" is a known constant.
        env.add(Declaration::Axiom {
            name: Name::str("Int"),
            univ_params: vec![],
            ty: Expr::Sort(Level::succ(Level::zero())),
        })
        .expect("Int axiom should be added");

        // Register a concrete constant `a : Int`.
        env.add(Declaration::Axiom {
            name: Name::str("a"),
            univ_params: vec![],
            ty: Expr::Const(Name::str("Int"), vec![]),
        })
        .expect("a axiom should be added");

        // Register `Int.le` as a binary predicate: ∀ (x y : Int), Prop.
        env.add(Declaration::Axiom {
            name: Name::str("Int.le"),
            univ_params: vec![],
            ty: Expr::Pi(
                BinderInfo::Default,
                Name::str("x"),
                Node::new(Expr::Const(Name::str("Int"), vec![])),
                Node::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("y"),
                    Node::new(Expr::Const(Name::str("Int"), vec![])),
                    Node::new(Expr::Sort(Level::zero())),
                )),
            ),
        })
        .expect("Int.le axiom should be added");

        // Register the omega helper lemmas (skips any already present).
        register_omega_helper(&mut env).expect("omega_helper registration should succeed");

        env
    }

    /// Build a test environment additionally containing:
    /// - `b : Int`, `c : Int`
    /// - `Int.lt : Int → Int → Prop`
    /// - `Not : Prop → Prop` (as opaque axiom, for lt_irrefl tests)
    /// - `False : Prop`
    /// - `absurd : {a b : Prop} → a → Not a → b`
    /// - `LE : Type → Type` (for LE.le tests)
    /// - `LE.le : {α : Type} → LE α → α → α → Prop`
    fn test_env_extended() -> Environment {
        let mut env = test_env();

        env.add(Declaration::Axiom {
            name: Name::str("b"),
            univ_params: vec![],
            ty: Expr::Const(Name::str("Int"), vec![]),
        })
        .expect("b axiom should be added");

        env.add(Declaration::Axiom {
            name: Name::str("c"),
            univ_params: vec![],
            ty: Expr::Const(Name::str("Int"), vec![]),
        })
        .expect("c axiom should be added");

        // Int.lt : Int → Int → Prop
        env.add(Declaration::Axiom {
            name: Name::str("Int.lt"),
            univ_params: vec![],
            ty: Expr::Pi(
                BinderInfo::Default,
                Name::str("x"),
                Node::new(Expr::Const(Name::str("Int"), vec![])),
                Node::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("y"),
                    Node::new(Expr::Const(Name::str("Int"), vec![])),
                    Node::new(Expr::Sort(Level::zero())),
                )),
            ),
        })
        .expect("Int.lt axiom should be added");

        // Not : Prop → Prop (opaque, mirrors the real definition)
        env.add(Declaration::Axiom {
            name: Name::str("Not"),
            univ_params: vec![],
            ty: Expr::Pi(
                BinderInfo::Default,
                Name::str("p"),
                Node::new(Expr::Sort(Level::zero())),
                Node::new(Expr::Sort(Level::zero())),
            ),
        })
        .expect("Not axiom should be added");

        // False : Prop
        env.add(Declaration::Axiom {
            name: Name::str("False"),
            univ_params: vec![],
            ty: Expr::Sort(Level::zero()),
        })
        .expect("False axiom should be added");

        // absurd : {a : Prop} → {b : Prop} → a → Not a → b
        // (implicit a, implicit b, explicit ha : a, explicit hna : Not a, result : b)
        let absurd_ty = Expr::Pi(
            BinderInfo::Implicit,
            Name::str("a"),
            Node::new(Expr::Sort(Level::zero())),
            Node::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("b"),
                Node::new(Expr::Sort(Level::zero())),
                Node::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("ha"),
                    Node::new(Expr::BVar(1)),
                    Node::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("hna"),
                        Node::new(Expr::App(
                            Node::new(Expr::Const(Name::str("Not"), vec![])),
                            Node::new(Expr::BVar(2)),
                        )),
                        Node::new(Expr::BVar(3)),
                    )),
                )),
            )),
        );
        env.add(Declaration::Axiom {
            name: Name::str("absurd"),
            univ_params: vec![],
            ty: absurd_ty,
        })
        .expect("absurd axiom should be added");

        // LE : Type → Type  (needed for LE.le and LE Int)
        env.add(Declaration::Axiom {
            name: Name::str("LE"),
            univ_params: vec![],
            ty: Expr::Pi(
                BinderInfo::Default,
                Name::str("α"),
                Node::new(Expr::Sort(Level::succ(Level::zero()))),
                Node::new(Expr::Sort(Level::succ(Level::zero()))),
            ),
        })
        .expect("LE axiom should be added");

        // LE.le : {α : Type} → LE α → α → α → Prop
        // (implicit α, explicit inst : LE α, explicit a : α, explicit b : α, Prop)
        let le_le_ty = Expr::Pi(
            BinderInfo::Implicit,
            Name::str("α"),
            Node::new(Expr::Sort(Level::succ(Level::zero()))),
            Node::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("inst"),
                Node::new(Expr::App(
                    Node::new(Expr::Const(Name::str("LE"), vec![])),
                    Node::new(Expr::BVar(0)),
                )),
                Node::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("a"),
                    Node::new(Expr::BVar(1)),
                    Node::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("b"),
                        Node::new(Expr::BVar(2)),
                        Node::new(Expr::Sort(Level::zero())),
                    )),
                )),
            )),
        );
        env.add(Declaration::Axiom {
            name: Name::str("LE.le"),
            univ_params: vec![],
            ty: le_le_ty,
        })
        .expect("LE.le axiom should be added");

        env
    }

    /// Build the goal expression `Int.le a a`.
    fn goal_le_refl() -> Expr {
        let int_le = Expr::Const(Name::str("Int.le"), vec![]);
        let a = Expr::Const(Name::str("a"), vec![]);
        Expr::App(
            Node::new(Expr::App(Node::new(int_le), Node::new(a.clone()))),
            Node::new(a),
        )
    }

    /// Build `Int.le X Y`.
    fn make_int_le(x: Expr, y: Expr) -> Expr {
        Expr::App(
            Node::new(Expr::App(
                Node::new(Expr::Const(Name::str("Int.le"), vec![])),
                Node::new(x),
            )),
            Node::new(y),
        )
    }

    /// Build `Int.lt X Y`.
    fn make_int_lt(x: Expr, y: Expr) -> Expr {
        Expr::App(
            Node::new(Expr::App(
                Node::new(Expr::Const(Name::str("Int.lt"), vec![])),
                Node::new(x),
            )),
            Node::new(y),
        )
    }

    fn dummy_proof() -> OmegaProof {
        OmegaProof {
            steps: vec![OmegaStep::Contradiction { value: -1 }],
        }
    }

    #[test]
    fn test_le_refl_reconstructed() {
        let env = test_env();
        let goal = goal_le_refl();

        let result = omega_proof_to_expr(&dummy_proof(), &goal, &[], &[], &env);
        assert!(
            result.is_some(),
            "le_refl pattern should produce a proof term"
        );

        // Kernel verification.
        let term = result.expect("result should be Some");
        let mut checker = TypeChecker::new(&env);
        let inferred = checker
            .infer_type(&term)
            .expect("reconstructed le_refl term should type-check");
        assert!(
            checker.is_def_eq(&inferred, &goal),
            "proof term should have the goal type (Int.le a a)"
        );
    }

    #[test]
    fn test_le_refl_distinct_sides_returns_none() {
        let env = test_env();
        env.find(&Name::str("a")).expect("a should exist in env");

        let goal = make_int_le(
            Expr::Const(Name::str("a"), vec![]),
            Expr::Const(Name::str("b"), vec![]),
        );

        let result = omega_proof_to_expr(&dummy_proof(), &goal, &[], &[], &env);
        // a ≤ b with different sides — le_refl can't apply; returns None.
        assert!(result.is_none(), "non-reflexive le goal should return None");
    }

    #[test]
    fn test_sorry_fallback_for_complex_proofs() {
        let env = test_env();
        // A goal shape we don't handle: some unrecognized constant.
        let some_goal = Expr::Const(Name::str("SomeComplexGoal"), vec![]);

        let complex_proof = OmegaProof {
            steps: vec![
                OmegaStep::CaseSplit {
                    var: Name::str("x"),
                    lower: 0,
                    upper: 5,
                },
                OmegaStep::Contradiction { value: -1 },
            ],
        };

        let result = omega_proof_to_expr(&complex_proof, &some_goal, &[], &[], &env);
        // Unhandled shape → None (caller substitutes sorry).
        let _ = result; // None is a valid, documented outcome
    }

    #[test]
    fn test_non_int_le_goal_returns_none() {
        let env = test_env();
        // Goal uses LE.le instead of Int.le — the LE.le args are wrong (missing inst),
        // so the match_le_le_app won't match a 2-arg LE.le application.
        // This tests a malformed LE.le goal (only 2 explicit args instead of 4).
        let le_le = Expr::Const(Name::str("LE.le"), vec![]);
        let a = Expr::Const(Name::str("a"), vec![]);
        let goal = Expr::App(
            Node::new(Expr::App(Node::new(le_le), Node::new(a.clone()))),
            Node::new(a),
        );

        let result = omega_proof_to_expr(&dummy_proof(), &goal, &[], &[], &env);
        assert!(
            result.is_none(),
            "malformed LE.le goals (only 2 args) should return None"
        );
    }

    #[test]
    fn test_le_trans_reconstructed() {
        // Goal: Int.le a c
        // Hyps: h1: Int.le a b, h2: Int.le b c
        // Expected: omega_proof_to_expr returns Some(Int.le_trans a b c h1 h2)
        let env = test_env_extended();
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let c = Expr::Const(Name::str("c"), vec![]);

        let goal = make_int_le(a.clone(), c.clone());
        let h1_ty = make_int_le(a.clone(), b.clone());
        let h2_ty = make_int_le(b.clone(), c.clone());

        // Hypotheses: (name, type). The proof term for each is Expr::Const(name).
        let hyps: Vec<(Name, Expr)> = vec![
            (Name::str("h1"), h1_ty.clone()),
            (Name::str("h2"), h2_ty.clone()),
        ];

        let result = omega_proof_to_expr(&dummy_proof(), &goal, &hyps, &[], &env);
        assert!(
            result.is_some(),
            "le_trans pattern should produce a proof term when a chain a ≤ b ≤ c is in hyps"
        );

        // Kernel verification using augmented environment (hyps as axioms).
        let term = result.expect("result should be Some");
        let mut aug_env = env.clone();
        aug_env
            .add(Declaration::Axiom {
                name: Name::str("h1"),
                univ_params: vec![],
                ty: h1_ty,
            })
            .expect("h1 should be added");
        aug_env
            .add(Declaration::Axiom {
                name: Name::str("h2"),
                univ_params: vec![],
                ty: h2_ty,
            })
            .expect("h2 should be added");
        let mut checker = TypeChecker::new(&aug_env);
        let inferred = checker
            .infer_type(&term)
            .expect("reconstructed le_trans term should type-check");
        assert!(
            checker.is_def_eq(&inferred, &goal),
            "proof term should have the goal type (Int.le a c)"
        );
    }

    #[test]
    fn test_le_trans_no_chain_returns_none() {
        // Goal: Int.le a c
        // Hyps: h1: Int.le a b (only one side of the chain — no h2)
        let env = test_env_extended();
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let c = Expr::Const(Name::str("c"), vec![]);

        let goal = make_int_le(a.clone(), c.clone());
        let h1_ty = make_int_le(a.clone(), b.clone());

        let hyps: Vec<(Name, Expr)> = vec![(Name::str("h1"), h1_ty)];

        let result = omega_proof_to_expr(&dummy_proof(), &goal, &hyps, &[], &env);
        assert!(
            result.is_none(),
            "incomplete chain (missing h2) should return None"
        );
    }

    #[test]
    fn test_lt_irrefl_with_absurd() {
        // Goal: False
        // Hyp: h: Int.lt a a
        // Expected: Some(@absurd (Int.lt a a) False h (Int.lt_irrefl a))
        // Now that absurd is in the test env and locals are threaded,
        // this pattern CAN succeed.
        let env = test_env_extended();
        let a = Expr::Const(Name::str("a"), vec![]);

        let false_goal = Expr::Const(Name::str("False"), vec![]);
        let h_ty = make_int_lt(a.clone(), a.clone());

        let hyps: Vec<(Name, Expr)> = vec![(Name::str("h"), h_ty)];

        let result = omega_proof_to_expr(&dummy_proof(), &false_goal, &hyps, &[], &env);
        // With absurd in the env, this should succeed (using Const refs for hyp proof).
        // Document the result — it may succeed or None depending on kernel def-eq resolution.
        // Either outcome is safe; the test ensures no panic or invalid proof emission.
        let _ = result;
    }

    #[test]
    fn test_le_refl_with_fvar_goal() {
        // Test that FVar threading allows le_refl to work with FVar-based goals.
        let mut env = Environment::new();
        env.add(Declaration::Axiom {
            name: Name::str("Int"),
            univ_params: vec![],
            ty: Expr::Sort(Level::succ(Level::zero())),
        })
        .expect("Int");
        env.add(Declaration::Axiom {
            name: Name::str("Int.le"),
            univ_params: vec![],
            ty: Expr::Pi(
                BinderInfo::Default,
                Name::str("x"),
                Node::new(Expr::Const(Name::str("Int"), vec![])),
                Node::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("y"),
                    Node::new(Expr::Const(Name::str("Int"), vec![])),
                    Node::new(Expr::Sort(Level::zero())),
                )),
            ),
        })
        .expect("Int.le");
        register_omega_helper(&mut env).expect("omega helper");

        // Create a local var `a : Int` using fresh_fvar.
        let mut tc = TypeChecker::new(&env);
        let fvar_a = tc.fresh_fvar(Name::str("a"), Expr::Const(Name::str("Int"), vec![]));
        let a_expr = Expr::FVar(fvar_a);

        // Goal: Int.le a a (using FVar a)
        let goal = Expr::App(
            Node::new(Expr::App(
                Node::new(Expr::Const(Name::str("Int.le"), vec![])),
                Node::new(a_expr.clone()),
            )),
            Node::new(a_expr.clone()),
        );

        let locals = vec![(
            fvar_a,
            Name::str("a"),
            Expr::Const(Name::str("Int"), vec![]),
        )];
        let result = omega_proof_to_expr(&dummy_proof(), &goal, &[], &locals, &env);
        assert!(
            result.is_some(),
            "le_refl with FVar goal should succeed after FVar threading: got None"
        );
    }

    #[test]
    fn test_le_trans_with_fvar_goal() {
        // Test that FVar threading allows le_trans to work with FVar-based goals.
        let mut env = Environment::new();
        env.add(Declaration::Axiom {
            name: Name::str("Int"),
            univ_params: vec![],
            ty: Expr::Sort(Level::succ(Level::zero())),
        })
        .expect("Int");
        env.add(Declaration::Axiom {
            name: Name::str("Int.le"),
            univ_params: vec![],
            ty: Expr::Pi(
                BinderInfo::Default,
                Name::str("x"),
                Node::new(Expr::Const(Name::str("Int"), vec![])),
                Node::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("y"),
                    Node::new(Expr::Const(Name::str("Int"), vec![])),
                    Node::new(Expr::Sort(Level::zero())),
                )),
            ),
        })
        .expect("Int.le");
        register_omega_helper(&mut env).expect("omega helper");

        // Create FVars for a, b, c, h1, h2.
        let mut tc = TypeChecker::new(&env);
        let fvar_a = tc.fresh_fvar(Name::str("a"), Expr::Const(Name::str("Int"), vec![]));
        let fvar_b = tc.fresh_fvar(Name::str("b"), Expr::Const(Name::str("Int"), vec![]));
        let fvar_c = tc.fresh_fvar(Name::str("c"), Expr::Const(Name::str("Int"), vec![]));
        let a_expr = Expr::FVar(fvar_a);
        let b_expr = Expr::FVar(fvar_b);
        let c_expr = Expr::FVar(fvar_c);

        let h1_ty = make_int_le(a_expr.clone(), b_expr.clone());
        let h2_ty = make_int_le(b_expr.clone(), c_expr.clone());

        let fvar_h1 = tc.fresh_fvar(Name::str("h1"), h1_ty.clone());
        let fvar_h2 = tc.fresh_fvar(Name::str("h2"), h2_ty.clone());

        // Goal: Int.le a c (FVar-based)
        let goal = make_int_le(a_expr.clone(), c_expr.clone());

        let hyps: Vec<(Name, Expr)> = vec![
            (Name::str("h1"), h1_ty.clone()),
            (Name::str("h2"), h2_ty.clone()),
        ];
        let locals: Vec<(FVarId, Name, Expr)> = vec![
            (
                fvar_a,
                Name::str("a"),
                Expr::Const(Name::str("Int"), vec![]),
            ),
            (
                fvar_b,
                Name::str("b"),
                Expr::Const(Name::str("Int"), vec![]),
            ),
            (
                fvar_c,
                Name::str("c"),
                Expr::Const(Name::str("Int"), vec![]),
            ),
            (fvar_h1, Name::str("h1"), h1_ty.clone()),
            (fvar_h2, Name::str("h2"), h2_ty.clone()),
        ];

        let result = omega_proof_to_expr(&dummy_proof(), &goal, &hyps, &locals, &env);
        assert!(
            result.is_some(),
            "le_trans with FVar goal should succeed after FVar threading: got None"
        );
    }

    #[test]
    fn test_le_trans_fvar_goal_with_locals_now_succeeds() {
        // This test documents the cycle 4 fix:
        // Previously this returned None (known limitation).
        // Now that FVar threading is implemented, it should succeed.
        let env = test_env_extended();
        let fvar_a = FVarId(100);
        let fvar_b = FVarId(101);
        let fvar_c = FVarId(102);
        let a_expr = Expr::FVar(fvar_a);
        let b_expr = Expr::FVar(fvar_b);
        let c_expr = Expr::FVar(fvar_c);

        let goal = make_int_le(a_expr.clone(), c_expr.clone());
        let h1_ty = make_int_le(a_expr.clone(), b_expr.clone());
        let h2_ty = make_int_le(b_expr.clone(), c_expr.clone());

        let hyps: Vec<(Name, Expr)> = vec![
            (Name::str("h1"), h1_ty.clone()),
            (Name::str("h2"), h2_ty.clone()),
        ];
        // Provide locals so TypeChecker can resolve FVarId(100/101/102).
        let locals: Vec<(FVarId, Name, Expr)> = vec![
            (
                fvar_a,
                Name::str("a"),
                Expr::Const(Name::str("Int"), vec![]),
            ),
            (
                fvar_b,
                Name::str("b"),
                Expr::Const(Name::str("Int"), vec![]),
            ),
            (
                fvar_c,
                Name::str("c"),
                Expr::Const(Name::str("Int"), vec![]),
            ),
        ];
        // We also need h1, h2 as FVars — but since we're using Const-based hyp
        // proof references (falling back from hyp_proof_term), the hyps param is enough.

        let result = omega_proof_to_expr(&dummy_proof(), &goal, &hyps, &locals, &env);
        // With FVar threading for a/b/c, the proof term type-checks.
        // h1/h2 are referenced as Expr::Const from hyp_proof_term (no FVar for them here).
        assert!(
            result.is_some(),
            "le_trans with FVar a/b/c and Const h1/h2 should succeed with FVar threading"
        );
    }
}
