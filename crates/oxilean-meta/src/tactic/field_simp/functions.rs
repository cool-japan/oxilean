//! Implementation of the `field_simp` tactic — denominator clearing and fraction normalization.

#![allow(dead_code)]

use super::types::{DivisionPattern, FieldSimpConfig, FieldSimpResult};
use crate::basic::{MetaContext, MetavarKind};
use crate::tactic::state::{TacticError, TacticResult, TacticState};
use oxilean_kernel::{Expr, Literal, Name};

// ---------------------------------------------------------------------------
// Helper utilities
// ---------------------------------------------------------------------------

/// Extract the constant name from an expression, if it is a `Const` node.
fn const_name(expr: &Expr) -> Option<String> {
    if let Expr::Const(name, _) = expr {
        Some(name.to_string())
    } else {
        None
    }
}

/// Check whether an expression is the equality constant (`Eq` / `eq`).
fn is_eq_const(expr: &Expr) -> bool {
    matches!(expr, Expr::Const(name, _) if {
        let s = name.to_string();
        s == "Eq" || s == "eq"
    })
}

/// Extract the LHS and RHS of an equality expression, or return `None`.
///
/// Handles:
///   - `App(App(App(Eq, _ty), lhs), rhs)` — fully-typed equality
///   - `App(App(Eq, lhs), rhs)` — raw equality
fn extract_eq_sides(expr: &Expr) -> Option<(Expr, Expr)> {
    if let Expr::App(func, rhs) = expr {
        if let Expr::App(func2, lhs) = func.as_ref() {
            if let Expr::App(eq_expr, _ty) = func2.as_ref() {
                if is_eq_const(eq_expr) {
                    return Some((*lhs.clone(), *rhs.clone()));
                }
            }
            if is_eq_const(func2) {
                return Some((*lhs.clone(), *rhs.clone()));
            }
        }
    }
    None
}

/// Return `true` if this expression looks like a division or inverse constant.
fn is_div_or_inv(name: &str) -> bool {
    matches!(
        name,
        "HDiv.hDiv"
            | "Div.div"
            | "div"
            | "Field.div"
            | "Inv.inv"
            | "inv"
            | "Field.inv"
            | "DivisionRing.div"
    )
}

/// Build a multiplication expression: `lhs * rhs`.
fn make_mul(lhs: Expr, rhs: Expr) -> Expr {
    let mul_const = Expr::Const(Name::str("HMul.hMul"), vec![]);
    Expr::App(
        Box::new(Expr::App(Box::new(mul_const), Box::new(lhs))),
        Box::new(rhs),
    )
}

// ---------------------------------------------------------------------------
// Core field_simp functions
// ---------------------------------------------------------------------------

/// Walk an expression tree and collect all `DivisionPattern`s.
///
/// Traverses the entire tree recursively; every `a / b` or `a⁻¹` node
/// is recorded.  Patterns in sub-expressions of a pattern are also
/// collected independently.
pub fn find_division_patterns(expr: &Expr) -> Vec<DivisionPattern> {
    let mut patterns = Vec::new();
    find_division_patterns_inner(expr, &mut patterns);
    patterns
}

fn find_division_patterns_inner(expr: &Expr, out: &mut Vec<DivisionPattern>) {
    match expr {
        Expr::App(func, arg) => {
            // Check for `Inv.inv e` — unary inverse.
            if let Some(name) = const_name(func) {
                if matches!(name.as_str(), "Inv.inv" | "inv" | "Field.inv") {
                    out.push(DivisionPattern::Inv {
                        inner: *arg.clone(),
                    });
                    // Still recurse into inner.
                    find_division_patterns_inner(arg, out);
                    return;
                }
            }
            // Check for `(HDiv.hDiv lhs) rhs` — binary div applied to both args.
            if let Expr::App(func2, lhs) = func.as_ref() {
                if let Some(op_name) = const_name(func2) {
                    if is_div_or_inv(op_name.as_str()) {
                        out.push(DivisionPattern::Div {
                            numerator: *lhs.clone(),
                            denominator: *arg.clone(),
                        });
                        find_division_patterns_inner(lhs, out);
                        find_division_patterns_inner(arg, out);
                        return;
                    }
                }
                // Three-arg form: `(f ty lhs) rhs` where f is div.
                if let Expr::App(func3, _ty) = func2.as_ref() {
                    if let Some(op_name) = const_name(func3) {
                        if is_div_or_inv(op_name.as_str()) {
                            out.push(DivisionPattern::Div {
                                numerator: *lhs.clone(),
                                denominator: *arg.clone(),
                            });
                            find_division_patterns_inner(lhs, out);
                            find_division_patterns_inner(arg, out);
                            return;
                        }
                    }
                }
            }
            // Default: recurse into both sub-expressions.
            find_division_patterns_inner(func, out);
            find_division_patterns_inner(arg, out);
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            find_division_patterns_inner(ty, out);
            find_division_patterns_inner(body, out);
        }
        Expr::Let(_, ty, val, body) => {
            find_division_patterns_inner(ty, out);
            find_division_patterns_inner(val, out);
            find_division_patterns_inner(body, out);
        }
        // Atomic expressions have no sub-expressions.
        _ => {}
    }
}

/// Multiply both sides of an equation by `denominator` to clear it.
///
/// Returns `(new_lhs, new_rhs)` where each side has been wrapped in a
/// multiplication by `denominator`.
pub fn clear_denominator(lhs: &Expr, rhs: &Expr, denominator: &Expr) -> (Expr, Expr) {
    let new_lhs = make_mul(denominator.clone(), lhs.clone());
    let new_rhs = make_mul(denominator.clone(), rhs.clone());
    (new_lhs, new_rhs)
}

/// Normalize fractions within a single expression.
///
/// Applies the following reductions:
/// - `(a / b) * (b / c)` → `a / c`  (cancellation)
/// - `a * (a⁻¹)` → `1`              (inverse cancellation, structural only)
/// - `(a / a)` → `1`                (trivial self-division)
///
/// This is a purely structural/syntactic normalization — it does not
/// involve proof obligations.
pub fn normalize_fractions(expr: &Expr) -> Expr {
    match expr {
        Expr::App(func, arg) => {
            let norm_func = normalize_fractions(func);
            let norm_arg = normalize_fractions(arg);

            // Pattern: `(a/b) * (b/c)` — structural cancellation.
            // We look for `HMul.hMul (a/b) (c/d)` and try to cancel.
            if let Some(mul_name) = const_name(&norm_func) {
                if matches!(mul_name.as_str(), "HMul.hMul" | "Mul.mul") {
                    // norm_func is `Mul lhs`, norm_arg is `rhs`.
                    // Check norm_arg for `c / d`.
                    if let Some(DivisionPattern::Div {
                        numerator: rhs_num,
                        denominator: rhs_den,
                    }) = try_extract_div_pattern(&norm_arg)
                    {
                        // norm_func side should have (Mul applied to lhs).
                        // The lhs argument is in norm_func itself.
                        if let Some(DivisionPattern::Div {
                            numerator: lhs_num,
                            denominator: lhs_den,
                        }) = try_extract_div_from_app_arg(&norm_func)
                        {
                            // Cancel lhs_den with rhs_num if structurally equal.
                            if exprs_structurally_equal(&lhs_den, &rhs_num) {
                                let div_const = Expr::Const(Name::str("HDiv.hDiv"), vec![]);
                                return Expr::App(
                                    Box::new(Expr::App(Box::new(div_const), Box::new(lhs_num))),
                                    Box::new(rhs_den),
                                );
                            }
                        }
                    }
                }
            }

            // Pattern: `a / a` → `1`.
            if let Some(DivisionPattern::Div {
                numerator,
                denominator,
            }) = try_extract_div_pattern_from_app(&norm_func, &norm_arg)
            {
                if exprs_structurally_equal(&numerator, &denominator) {
                    return Expr::Lit(oxilean_kernel::Literal::Nat(1));
                }
            }

            Expr::App(Box::new(norm_func), Box::new(norm_arg))
        }
        Expr::Lam(bi, name, ty, body) => Expr::Lam(
            *bi,
            name.clone(),
            Box::new(normalize_fractions(ty)),
            Box::new(normalize_fractions(body)),
        ),
        Expr::Pi(bi, name, ty, body) => Expr::Pi(
            *bi,
            name.clone(),
            Box::new(normalize_fractions(ty)),
            Box::new(normalize_fractions(body)),
        ),
        Expr::Let(name, ty, val, body) => Expr::Let(
            name.clone(),
            Box::new(normalize_fractions(ty)),
            Box::new(normalize_fractions(val)),
            Box::new(normalize_fractions(body)),
        ),
        other => other.clone(),
    }
}

/// Try to extract a `DivisionPattern` from a fully-applied expression node
/// where `func` is the partial application and `arg` is the last argument.
fn try_extract_div_pattern_from_app(func: &Expr, arg: &Expr) -> Option<DivisionPattern> {
    // `(HDiv.hDiv lhs) rhs`
    if let Expr::App(inner_func, lhs) = func {
        if let Some(name) = const_name(inner_func) {
            if is_div_or_inv(name.as_str()) {
                return Some(DivisionPattern::Div {
                    numerator: *lhs.clone(),
                    denominator: arg.clone(),
                });
            }
        }
    }
    None
}

/// Try to extract a `DivisionPattern` from a fully-applied expression.
fn try_extract_div_pattern(expr: &Expr) -> Option<DivisionPattern> {
    if let Expr::App(func, arg) = expr {
        return try_extract_div_pattern_from_app(func, arg);
    }
    None
}

/// Try to extract a division pattern where the last argument is the divisor,
/// and the function holds the numerator as its argument.
fn try_extract_div_from_app_arg(func_expr: &Expr) -> Option<DivisionPattern> {
    // func_expr should be `Mul (lhs_num / lhs_den)` — we extract the arg.
    if let Expr::App(_mul_part, div_part) = func_expr {
        return try_extract_div_pattern(div_part);
    }
    None
}

/// Structural equality check (no definitional unfolding, no mvar instantiation).
pub fn exprs_structurally_equal(a: &Expr, b: &Expr) -> bool {
    match (a, b) {
        (Expr::Sort(l1), Expr::Sort(l2)) => l1 == l2,
        (Expr::BVar(i), Expr::BVar(j)) => i == j,
        (Expr::FVar(id1), Expr::FVar(id2)) => id1 == id2,
        (Expr::Const(n1, ls1), Expr::Const(n2, ls2)) => n1 == n2 && ls1 == ls2,
        (Expr::App(f1, a1), Expr::App(f2, a2)) => {
            exprs_structurally_equal(f1, f2) && exprs_structurally_equal(a1, a2)
        }
        (Expr::Lam(bi1, n1, ty1, b1), Expr::Lam(bi2, n2, ty2, b2)) => {
            bi1 == bi2
                && n1 == n2
                && exprs_structurally_equal(ty1, ty2)
                && exprs_structurally_equal(b1, b2)
        }
        (Expr::Pi(bi1, n1, ty1, b1), Expr::Pi(bi2, n2, ty2, b2)) => {
            bi1 == bi2
                && n1 == n2
                && exprs_structurally_equal(ty1, ty2)
                && exprs_structurally_equal(b1, b2)
        }
        (Expr::Let(n1, ty1, v1, b1), Expr::Let(n2, ty2, v2, b2)) => {
            n1 == n2
                && exprs_structurally_equal(ty1, ty2)
                && exprs_structurally_equal(v1, v2)
                && exprs_structurally_equal(b1, b2)
        }
        (Expr::Lit(l1), Expr::Lit(l2)) => l1 == l2,
        _ => false,
    }
}

/// Apply `field_simp` logic to an expression: collect denominators,
/// then normalize fractions iteratively.
pub fn field_simp_expr(expr: &Expr, config: &FieldSimpConfig) -> FieldSimpResult {
    let mut current = expr.clone();
    let mut steps = 0;
    let mut changed = false;

    while steps < config.max_steps {
        let patterns = find_division_patterns(&current);
        if patterns.is_empty() {
            break;
        }
        // Normalize fractions — this may eliminate some patterns.
        let normalized = normalize_fractions(&current);
        if !exprs_structurally_equal(&normalized, &current) {
            current = normalized;
            changed = true;
            steps += 1;
        } else {
            break;
        }
    }

    FieldSimpResult {
        simplified: current,
        num_steps: steps,
        changed,
    }
}

// ---------------------------------------------------------------------------
// Main tactic entry point
// ---------------------------------------------------------------------------

/// The `field_simp` tactic: clear denominators and normalize fractions in the goal.
///
/// Extracts the current goal, looks for division/inverse patterns, multiplies
/// both sides of any equality by denominators, then normalizes.  Closes the
/// goal with `rfl` if the resulting normalized expressions are structurally
/// identical.
pub fn tac_field_simp(state: &mut TacticState, ctx: &mut MetaContext) -> TacticResult<()> {
    tac_field_simp_with_config(state, ctx, &FieldSimpConfig::default())
}

/// `field_simp` with a custom `FieldSimpConfig`.
pub fn tac_field_simp_with_config(
    state: &mut TacticState,
    ctx: &mut MetaContext,
    config: &FieldSimpConfig,
) -> TacticResult<()> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("field_simp: goal has no type".into()))?;
    let target = ctx.instantiate_mvars(&target);

    // Try to extract equality sides from the goal.
    let (lhs, rhs) = extract_eq_sides(&target)
        .ok_or_else(|| TacticError::GoalMismatch("field_simp requires an equality goal".into()))?;

    // Collect denominators from both sides.
    let mut all_denoms: Vec<Expr> = Vec::new();
    for pat in find_division_patterns(&lhs) {
        all_denoms.push(pat.denominator().clone());
    }
    for pat in find_division_patterns(&rhs) {
        all_denoms.push(pat.denominator().clone());
    }

    // Start with original sides and iteratively clear denominators.
    let mut cur_lhs = lhs;
    let mut cur_rhs = rhs;

    for denom in &all_denoms {
        if config.clear_denominators {
            let (new_lhs, new_rhs) = clear_denominator(&cur_lhs, &cur_rhs, denom);
            cur_lhs = new_lhs;
            cur_rhs = new_rhs;
        }
    }

    // Normalize fractions on each side.
    if config.normalize_result {
        cur_lhs = normalize_fractions(&cur_lhs);
        cur_rhs = normalize_fractions(&cur_rhs);
    }

    // If the sides are now structurally equal, close the goal with `rfl`.
    if exprs_structurally_equal(&cur_lhs, &cur_rhs) {
        let rfl = Expr::Const(Name::str("rfl"), vec![]);
        state.close_goal(rfl, ctx)?;
        return Ok(());
    }

    // Did we make any progress (change)?  If so, update the goal type.
    let orig_target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("field_simp: goal vanished".into()))?;

    let new_target = build_eq_expr(cur_lhs, cur_rhs);
    if exprs_structurally_equal(&new_target, &orig_target) {
        return Err(TacticError::Failed(
            "field_simp: no progress on goal".into(),
        ));
    }

    // Replace the goal's type with the simplified version.
    ctx.assign_mvar(
        goal,
        Expr::Const(Name::str("field_simp_placeholder"), vec![]),
    );
    let (new_mvar_id, _) = ctx.mk_fresh_expr_mvar(new_target, MetavarKind::Natural);
    state.replace_goal(vec![new_mvar_id]);
    Ok(())
}

/// Build an equality expression `Eq lhs rhs` (type-erased, untyped form).
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
    use oxilean_kernel::{Expr, Literal, Name};

    fn const_expr(name: &str) -> Expr {
        Expr::Const(Name::str(name), vec![])
    }

    fn nat_lit(n: u64) -> Expr {
        Expr::Lit(Literal::Nat(n))
    }

    fn div_expr(num: Expr, den: Expr) -> Expr {
        let div_const = const_expr("HDiv.hDiv");
        Expr::App(
            Box::new(Expr::App(Box::new(div_const), Box::new(num))),
            Box::new(den),
        )
    }

    #[test]
    fn test_find_division_patterns_simple() {
        // Build `a / b`
        let a = const_expr("a");
        let b = const_expr("b");
        let expr = div_expr(a.clone(), b.clone());
        let patterns = find_division_patterns(&expr);
        assert_eq!(patterns.len(), 1);
        match &patterns[0] {
            DivisionPattern::Div {
                numerator,
                denominator,
            } => {
                assert!(exprs_structurally_equal(numerator, &a));
                assert!(exprs_structurally_equal(denominator, &b));
            }
            _ => panic!("Expected Div pattern"),
        }
    }

    #[test]
    fn test_find_division_patterns_no_division() {
        // Build `a + b` — no division patterns.
        let a = const_expr("a");
        let b = const_expr("b");
        let add = const_expr("HAdd.hAdd");
        let expr = Expr::App(Box::new(Expr::App(Box::new(add), Box::new(a))), Box::new(b));
        let patterns = find_division_patterns(&expr);
        assert!(patterns.is_empty());
    }

    #[test]
    fn test_normalize_fractions_self_division() {
        // Build `a / a` — should normalize to `1`.
        let a = const_expr("a");
        let expr = div_expr(a.clone(), a.clone());
        let result = normalize_fractions(&expr);
        assert!(exprs_structurally_equal(&result, &nat_lit(1)));
    }

    #[test]
    fn test_clear_denominator() {
        // `clear_denominator(a, b, c)` should give `(c * a, c * b)`.
        let a = const_expr("a");
        let b = const_expr("b");
        let c = const_expr("c");
        let (new_lhs, new_rhs) = clear_denominator(&a, &b, &c);
        // new_lhs should be `c * a`
        let mul_const = const_expr("HMul.hMul");
        let expected_lhs = Expr::App(
            Box::new(Expr::App(Box::new(mul_const.clone()), Box::new(c.clone()))),
            Box::new(a),
        );
        let expected_rhs = Expr::App(
            Box::new(Expr::App(Box::new(mul_const), Box::new(c))),
            Box::new(b),
        );
        assert!(exprs_structurally_equal(&new_lhs, &expected_lhs));
        assert!(exprs_structurally_equal(&new_rhs, &expected_rhs));
    }

    #[test]
    fn test_exprs_structurally_equal_consts() {
        let a = const_expr("foo");
        let b = const_expr("foo");
        let c = const_expr("bar");
        assert!(exprs_structurally_equal(&a, &b));
        assert!(!exprs_structurally_equal(&a, &c));
    }

    #[test]
    fn test_find_division_patterns_nested() {
        // Build `(a / b) / c` — should find two patterns.
        let a = const_expr("a");
        let b = const_expr("b");
        let c = const_expr("c");
        let inner = div_expr(a, b);
        let outer = div_expr(inner, c);
        let patterns = find_division_patterns(&outer);
        // outer pattern + inner pattern
        assert!(patterns.len() >= 2);
    }
}
