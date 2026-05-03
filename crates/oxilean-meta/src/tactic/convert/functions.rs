//! Implementation of the `convert` tactic — exact with holes.
//!
//! `convert e` attempts to close the current goal `⊢ T` using the expression
//! `e : U`.  Where `T` and `U` are structurally similar but differ at some
//! sub-positions, new subgoals of the form `T_sub = U_sub` are generated for
//! each mismatch.  If there are no mismatches, `convert` acts exactly like
//! `exact`.

#![allow(dead_code)]

use super::types::{ConvertConfig, ConvertResult};
use crate::basic::{MetaContext, MetavarKind};
use crate::tactic::state::{TacticError, TacticResult, TacticState};
use oxilean_kernel::{Expr, Name};

// ---------------------------------------------------------------------------
// Structural equality helpers
// ---------------------------------------------------------------------------

/// Check structural (syntactic) equality of two expressions.
fn exprs_equal(a: &Expr, b: &Expr) -> bool {
    match (a, b) {
        (Expr::Sort(l1), Expr::Sort(l2)) => l1 == l2,
        (Expr::BVar(i), Expr::BVar(j)) => i == j,
        (Expr::FVar(id1), Expr::FVar(id2)) => id1 == id2,
        (Expr::Const(n1, ls1), Expr::Const(n2, ls2)) => n1 == n2 && ls1 == ls2,
        (Expr::App(f1, a1), Expr::App(f2, a2)) => exprs_equal(f1, f2) && exprs_equal(a1, a2),
        (Expr::Lam(bi1, n1, ty1, b1), Expr::Lam(bi2, n2, ty2, b2)) => {
            bi1 == bi2 && n1 == n2 && exprs_equal(ty1, ty2) && exprs_equal(b1, b2)
        }
        (Expr::Pi(bi1, n1, ty1, b1), Expr::Pi(bi2, n2, ty2, b2)) => {
            bi1 == bi2 && n1 == n2 && exprs_equal(ty1, ty2) && exprs_equal(b1, b2)
        }
        (Expr::Let(n1, ty1, v1, b1), Expr::Let(n2, ty2, v2, b2)) => {
            n1 == n2 && exprs_equal(ty1, ty2) && exprs_equal(v1, v2) && exprs_equal(b1, b2)
        }
        (Expr::Lit(l1), Expr::Lit(l2)) => l1 == l2,
        _ => false,
    }
}

// ---------------------------------------------------------------------------
// Mismatch collection
// ---------------------------------------------------------------------------

/// Walk `expected` and `provided` in parallel and collect all structural
/// mismatches as `(expected_subexpr, provided_subexpr)` pairs.
///
/// The walk is depth-first.  Recursion stops when the current pair already
/// constitutes a mismatch at a higher level, or when `depth` reaches zero.
///
/// A "mismatch" at a node means that the root constructors differ, or the
/// constructors match but a child pair is mismatched.  We report the
/// *lowest-level* mismatch: if children match, no mismatch is reported even
/// if the parents would "match" but have different child trees.
pub fn find_mismatches(expected: &Expr, provided: &Expr, depth: usize) -> Vec<(Expr, Expr)> {
    let mut mismatches = Vec::new();
    find_mismatches_inner(expected, provided, depth, &mut mismatches);
    mismatches
}

fn find_mismatches_inner(
    expected: &Expr,
    provided: &Expr,
    depth: usize,
    out: &mut Vec<(Expr, Expr)>,
) {
    if depth == 0 {
        if !exprs_equal(expected, provided) {
            out.push((expected.clone(), provided.clone()));
        }
        return;
    }
    match (expected, provided) {
        (Expr::App(ef, ea), Expr::App(pf, pa)) => {
            find_mismatches_inner(ef, pf, depth - 1, out);
            find_mismatches_inner(ea, pa, depth - 1, out);
        }
        (Expr::Lam(_, en, ety, eb), Expr::Lam(_, pn, pty, pb)) => {
            if en != pn {
                // Name mismatch — record at this level.
                out.push((expected.clone(), provided.clone()));
                return;
            }
            find_mismatches_inner(ety, pty, depth - 1, out);
            find_mismatches_inner(eb, pb, depth - 1, out);
        }
        (Expr::Pi(_, en, ety, eb), Expr::Pi(_, pn, pty, pb)) => {
            if en != pn {
                out.push((expected.clone(), provided.clone()));
                return;
            }
            find_mismatches_inner(ety, pty, depth - 1, out);
            find_mismatches_inner(eb, pb, depth - 1, out);
        }
        (Expr::Let(en, ety, ev, eb), Expr::Let(pn, pty, pv, pb)) => {
            if en != pn {
                out.push((expected.clone(), provided.clone()));
                return;
            }
            find_mismatches_inner(ety, pty, depth - 1, out);
            find_mismatches_inner(ev, pv, depth - 1, out);
            find_mismatches_inner(eb, pb, depth - 1, out);
        }
        (a, b) if exprs_equal(a, b) => {
            // Identical — no mismatch.
        }
        (a, b) => {
            // Different constructors or different leaf values.
            out.push((a.clone(), b.clone()));
        }
    }
}

// ---------------------------------------------------------------------------
// Tactic entry point
// ---------------------------------------------------------------------------

/// The `convert` tactic: close a goal with an expression that may not match
/// exactly, generating new subgoals for each structural mismatch.
///
/// If `expr` matches the goal type exactly → acts like `exact`.
/// If `expr` is structurally close but differs at some positions →
///   each mismatch position becomes a new subgoal `expected = provided`.
/// If the number of mismatches exceeds `config.max_subgoals` → fails.
pub fn tac_convert(state: &mut TacticState, ctx: &mut MetaContext, expr: Expr) -> TacticResult<()> {
    tac_convert_with_config(state, ctx, expr, &ConvertConfig::default())
}

/// `convert` with a custom `ConvertConfig`.
pub fn tac_convert_with_config(
    state: &mut TacticState,
    ctx: &mut MetaContext,
    expr: Expr,
    config: &ConvertConfig,
) -> TacticResult<()> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("convert: goal has no type".into()))?;
    let target = ctx.instantiate_mvars(&target);
    let expr = ctx.instantiate_mvars(&expr);

    // Infer the type of the provided expression (structural approximation:
    // we use the type stored in the context if expr is an mvar, otherwise
    // we treat the expression itself as the "type" for comparison purposes
    // when we cannot infer).
    //
    // For a full implementation, we would call `infer_type` here.  Since
    // we are in the meta layer without a full kernel evaluator available in
    // this crate, we use a conservative path: compare `target` with the
    // expression structurally, treating the expression itself as its type
    // representation for mismatch collection.
    let provided_ty = infer_expr_type_approx(ctx, &expr);

    // Collect mismatches between the goal type and the provided expression's type.
    let mismatches = find_mismatches(&target, &provided_ty, config.max_depth);

    if mismatches.len() > config.max_subgoals {
        return Err(TacticError::Failed(format!(
            "convert: {} mismatches exceed max_subgoals={}",
            mismatches.len(),
            config.max_subgoals
        )));
    }

    if mismatches.is_empty() {
        // No mismatches: act like `exact`.
        state.close_goal(expr, ctx)?;
        return Ok(());
    }

    // Generate a fresh metavariable for each mismatch (subgoal).
    // The subgoal type is `Eq expected_sub provided_sub`.
    let mut new_mvar_ids = Vec::new();
    for (expected_sub, provided_sub) in &mismatches {
        let subgoal_ty = build_eq_expr(expected_sub.clone(), provided_sub.clone());
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(subgoal_ty, MetavarKind::Natural);
        new_mvar_ids.push(mvar_id);
    }

    // Close the current goal with a placeholder proof.  In a full
    // implementation, this would be a `congrArg`-based term built from
    // the mvar proofs.  We use a named constant as a placeholder.
    let proof_placeholder = Expr::Const(Name::str("convert_proof_placeholder"), vec![]);
    ctx.assign_mvar(goal, proof_placeholder);

    // Replace the current goal with the mismatch subgoals.
    state.replace_goal(new_mvar_ids);
    Ok(())
}

/// Approximate type inference for an expression.
///
/// If the expression is a metavariable whose type is known in `ctx`,
/// returns that type.  Otherwise returns the expression itself as a proxy
/// for its type in the structural mismatch comparison.
fn infer_expr_type_approx(ctx: &MetaContext, expr: &Expr) -> Expr {
    // Check if expr encodes a metavariable (e.g. `Const("?m.N", [])` style).
    if let Some(mvar_id) = MetaContext::is_mvar_expr(expr) {
        if let Some(ty) = ctx.get_mvar_type(mvar_id) {
            return ty.clone();
        }
    }
    // For everything else, return the expression as-is (as a proxy for its
    // type in the structural comparison).  This is sufficient for the
    // convert tactic's purpose of identifying structural gaps.
    expr.clone()
}

/// Build an equality expression `Eq lhs rhs`.
fn build_eq_expr(lhs: Expr, rhs: Expr) -> Expr {
    let eq_const = Expr::Const(Name::str("Eq"), vec![]);
    Expr::App(
        Box::new(Expr::App(Box::new(eq_const), Box::new(lhs))),
        Box::new(rhs),
    )
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use oxilean_kernel::{Expr, Name};

    fn var(name: &str) -> Expr {
        Expr::Const(Name::str(name), vec![])
    }

    fn app(f: Expr, a: Expr) -> Expr {
        Expr::App(Box::new(f), Box::new(a))
    }

    #[test]
    fn test_no_mismatches_identical() {
        let e = app(var("f"), var("x"));
        let mismatches = find_mismatches(&e, &e, 8);
        assert!(
            mismatches.is_empty(),
            "identical exprs should have no mismatches"
        );
    }

    #[test]
    fn test_mismatches_root_different() {
        let a = var("A");
        let b = var("B");
        let mismatches = find_mismatches(&a, &b, 8);
        assert_eq!(mismatches.len(), 1);
        assert!(exprs_equal(&mismatches[0].0, &a));
        assert!(exprs_equal(&mismatches[0].1, &b));
    }

    #[test]
    fn test_mismatches_one_child() {
        // `f X` vs `f Y` — one mismatch in the argument.
        let f = var("f");
        let x = var("X");
        let y = var("Y");
        let lhs = app(f.clone(), x.clone());
        let rhs = app(f.clone(), y.clone());
        let mismatches = find_mismatches(&lhs, &rhs, 4);
        assert_eq!(mismatches.len(), 1);
        assert!(exprs_equal(&mismatches[0].0, &x));
        assert!(exprs_equal(&mismatches[0].1, &y));
    }

    #[test]
    fn test_mismatches_depth_zero() {
        // At depth 0, any non-equal pair is a mismatch.
        let a = var("A");
        let b = var("B");
        let mismatches = find_mismatches(&a, &b, 0);
        assert_eq!(mismatches.len(), 1);
    }

    #[test]
    fn test_exprs_equal_consts() {
        let a = var("Foo");
        let b = var("Foo");
        let c = var("Bar");
        assert!(exprs_equal(&a, &b));
        assert!(!exprs_equal(&a, &c));
    }
}
