//! The `simp_rw` tactic: directed rewriting combined with simp cleanup.
//!
//! `simp_rw lemmas` alternates between:
//!   1. A rewriting pass — apply each listed lemma as an equational rewrite
//!      (left-to-right) anywhere in the goal expression.
//!   2. A simp cleanup pass — apply the full simp set to normalize.
//!
//! It iterates at most 10 times (configurable) and reports success if any
//! progress was made.  This mirrors Lean 4's `simp_rw` which is useful when
//! `simp only` would fail because the lemmas need to be applied under binders.

#![allow(dead_code)]

use crate::basic::{MetaContext, MetavarKind};
use crate::tactic::simp::main::simp;
use crate::tactic::simp::types::{SimpConfig, SimpLemma, SimpResult, SimpTheorems};
use crate::tactic::state::{TacticError, TacticResult, TacticState};
use oxilean_kernel::{Expr, Name};

// ---------------------------------------------------------------------------
// Rewriting helpers
// ---------------------------------------------------------------------------

/// A single equational rewrite rule `lhs → rhs`.
#[derive(Debug, Clone)]
pub struct RwRule {
    /// Left-hand side pattern.
    pub lhs: Expr,
    /// Right-hand side to substitute.
    pub rhs: Expr,
    /// Originating lemma name (for error reporting).
    pub name: Name,
}

/// Apply `rule` to `expr` if the root of `expr` matches `rule.lhs`.
///
/// Returns `Some(rhs)` on a match, `None` otherwise.
/// This is a pure syntactic match (no unification).
fn try_rewrite_root(rule: &RwRule, expr: &Expr) -> Option<Expr> {
    if exprs_equal(&rule.lhs, expr) {
        Some(rule.rhs.clone())
    } else {
        None
    }
}

/// Apply all `rules` to the entire expression tree, recursively.
///
/// Returns `Some(new_expr)` if any rewrite was applied, `None` if the
/// expression is unchanged.
pub fn apply_rw_rules(rules: &[RwRule], expr: &Expr) -> Option<Expr> {
    // Try root-level rewrites first.
    for rule in rules {
        if let Some(result) = try_rewrite_root(rule, expr) {
            return Some(result);
        }
    }
    // Recurse into sub-expressions.
    match expr {
        Expr::App(f, a) => {
            let new_f = apply_rw_rules(rules, f);
            let new_a = apply_rw_rules(rules, a);
            if new_f.is_some() || new_a.is_some() {
                let f_out = new_f.unwrap_or_else(|| *f.clone());
                let a_out = new_a.unwrap_or_else(|| *a.clone());
                Some(Expr::App(Box::new(f_out), Box::new(a_out)))
            } else {
                None
            }
        }
        Expr::Lam(bi, name, ty, body) => {
            let new_ty = apply_rw_rules(rules, ty);
            let new_body = apply_rw_rules(rules, body);
            if new_ty.is_some() || new_body.is_some() {
                Some(Expr::Lam(
                    *bi,
                    name.clone(),
                    Box::new(new_ty.unwrap_or_else(|| *ty.clone())),
                    Box::new(new_body.unwrap_or_else(|| *body.clone())),
                ))
            } else {
                None
            }
        }
        Expr::Pi(bi, name, ty, body) => {
            let new_ty = apply_rw_rules(rules, ty);
            let new_body = apply_rw_rules(rules, body);
            if new_ty.is_some() || new_body.is_some() {
                Some(Expr::Pi(
                    *bi,
                    name.clone(),
                    Box::new(new_ty.unwrap_or_else(|| *ty.clone())),
                    Box::new(new_body.unwrap_or_else(|| *body.clone())),
                ))
            } else {
                None
            }
        }
        Expr::Let(name, ty, val, body) => {
            let new_ty = apply_rw_rules(rules, ty);
            let new_val = apply_rw_rules(rules, val);
            let new_body = apply_rw_rules(rules, body);
            if new_ty.is_some() || new_val.is_some() || new_body.is_some() {
                Some(Expr::Let(
                    name.clone(),
                    Box::new(new_ty.unwrap_or_else(|| *ty.clone())),
                    Box::new(new_val.unwrap_or_else(|| *val.clone())),
                    Box::new(new_body.unwrap_or_else(|| *body.clone())),
                ))
            } else {
                None
            }
        }
        // Atomic nodes cannot be rewritten further (unless matched at root).
        _ => None,
    }
}

/// Structural equality check — no unification, no definitional reduction.
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
// Rule construction from lemma names
// ---------------------------------------------------------------------------

/// Build a `RwRule` from a lemma name.
///
/// In a full implementation, this would look up the lemma's statement in the
/// environment and extract `lhs = rhs`.  Here we create a structural
/// placeholder rule `Const(name) → Const(name)_simp_rw_result` to make the
/// machinery testable.
fn rule_for_lemma(name: &Name) -> RwRule {
    let lhs = Expr::Const(name.clone(), vec![]);
    let simp_name = Name::str(format!("{}_simp_rw_result", name));
    let rhs = Expr::Const(simp_name, vec![]);
    RwRule {
        lhs,
        rhs,
        name: name.clone(),
    }
}

// ---------------------------------------------------------------------------
// Main tactic
// ---------------------------------------------------------------------------

/// The `simp_rw` tactic.
///
/// Alternates between rewriting passes (using the given lemma names) and
/// simp cleanup passes, for at most `max_iters` iterations.
/// Returns `Ok(())` if any progress was made; `Err` otherwise.
pub fn tac_simp_rw(
    state: &mut TacticState,
    ctx: &mut MetaContext,
    lemma_names: &[Name],
) -> TacticResult<()> {
    tac_simp_rw_with_iters(state, ctx, lemma_names, 10)
}

/// `simp_rw` with a configurable maximum number of iterations.
pub fn tac_simp_rw_with_iters(
    state: &mut TacticState,
    ctx: &mut MetaContext,
    lemma_names: &[Name],
    max_iters: usize,
) -> TacticResult<()> {
    let goal = state.current_goal()?;
    let initial_target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("simp_rw: goal has no type".into()))?;
    let initial_target = ctx.instantiate_mvars(&initial_target);

    // Build rewrite rules from the provided lemma names.
    let rules: Vec<RwRule> = lemma_names.iter().map(rule_for_lemma).collect();

    // Build a default simp theorem set for cleanup passes.
    let theorems = SimpTheorems::new();
    let simp_config = SimpConfig::default();

    let mut current = initial_target.clone();
    let mut any_progress = false;

    for _iter in 0..max_iters {
        // Rewriting pass.
        let after_rw = apply_rw_rules(&rules, &current);
        let rw_changed = after_rw.is_some();
        let after_rw_expr = after_rw.unwrap_or_else(|| current.clone());

        // Simp cleanup pass.
        let after_simp = simp(&after_rw_expr, &theorems, &simp_config, ctx);
        let (simp_changed, after_simp_expr) = match after_simp {
            SimpResult::Simplified { new_expr, .. } => (true, new_expr),
            SimpResult::Proved(_proof) => {
                // Goal is now trivially true — close it.
                let rfl = Expr::Const(Name::str("rfl"), vec![]);
                state.close_goal(rfl, ctx)?;
                return Ok(());
            }
            SimpResult::Unchanged => (false, after_rw_expr),
        };

        let iter_changed = rw_changed || simp_changed;
        if iter_changed {
            any_progress = true;
            current = after_simp_expr;
        } else {
            break;
        }
    }

    if !any_progress {
        return Err(TacticError::Failed("simp_rw: no progress made".into()));
    }

    // Update the goal's type to the simplified expression, if it changed.
    if !exprs_equal(&current, &initial_target) {
        ctx.assign_mvar(goal, Expr::Const(Name::str("simp_rw_placeholder"), vec![]));
        let (new_mvar_id, _) = ctx.mk_fresh_expr_mvar(current, MetavarKind::Natural);
        state.replace_goal(vec![new_mvar_id]);
    }

    Ok(())
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
    fn test_apply_rw_rules_root_match() {
        let rule = RwRule {
            lhs: var("x"),
            rhs: var("y"),
            name: Name::str("test_rule"),
        };
        let expr = var("x");
        let result = apply_rw_rules(&[rule], &expr);
        assert!(result.is_some());
        let r = result.unwrap();
        assert!(matches!(r, Expr::Const(ref n, _) if n.to_string() == "y"));
    }

    #[test]
    fn test_apply_rw_rules_no_match() {
        let rule = RwRule {
            lhs: var("x"),
            rhs: var("y"),
            name: Name::str("test_rule"),
        };
        let expr = var("z");
        let result = apply_rw_rules(&[rule], &expr);
        assert!(result.is_none());
    }

    #[test]
    fn test_apply_rw_rules_under_app() {
        // `f(x)` with rule `x → y` should give `f(y)`.
        let rule = RwRule {
            lhs: var("x"),
            rhs: var("y"),
            name: Name::str("r"),
        };
        let expr = app(var("f"), var("x"));
        let result = apply_rw_rules(&[rule], &expr);
        assert!(result.is_some());
        let r = result.unwrap();
        assert!(
            matches!(r, Expr::App(_, a) if matches!(a.as_ref(), Expr::Const(ref n, _) if n.to_string() == "y"))
        );
    }

    #[test]
    fn test_exprs_equal_reflexive() {
        let e = app(var("f"), var("g"));
        assert!(exprs_equal(&e, &e));
    }

    #[test]
    fn test_exprs_equal_different() {
        let e1 = var("a");
        let e2 = var("b");
        assert!(!exprs_equal(&e1, &e2));
    }
}
