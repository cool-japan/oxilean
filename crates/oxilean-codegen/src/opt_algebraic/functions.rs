//! Functions for the Algebraic Simplification optimisation pass.

use std::collections::HashMap;

use super::types::{AlgExpr, AlgSimplConfig, SimplResult, SimplStats};

// ── Expression utilities ──────────────────────────────────────────────────────

/// Count the number of nodes in an expression tree.
pub fn expr_size(expr: &AlgExpr) -> usize {
    match expr {
        AlgExpr::Const(_) | AlgExpr::Var(_) => 1,
        AlgExpr::Neg(e) => 1 + expr_size(e),
        AlgExpr::Add(l, r)
        | AlgExpr::Sub(l, r)
        | AlgExpr::Mul(l, r)
        | AlgExpr::Div(l, r)
        | AlgExpr::Pow(l, r)
        | AlgExpr::Mod(l, r) => 1 + expr_size(l) + expr_size(r),
    }
}

/// Count the occurrences of each variable name in `expr`.
pub fn count_vars(expr: &AlgExpr) -> HashMap<String, usize> {
    let mut map: HashMap<String, usize> = HashMap::new();
    count_vars_impl(expr, &mut map);
    map
}

fn count_vars_impl(expr: &AlgExpr, map: &mut HashMap<String, usize>) {
    match expr {
        AlgExpr::Const(_) => {}
        AlgExpr::Var(v) => *map.entry(v.clone()).or_insert(0) += 1,
        AlgExpr::Neg(e) => count_vars_impl(e, map),
        AlgExpr::Add(l, r)
        | AlgExpr::Sub(l, r)
        | AlgExpr::Mul(l, r)
        | AlgExpr::Div(l, r)
        | AlgExpr::Pow(l, r)
        | AlgExpr::Mod(l, r) => {
            count_vars_impl(l, map);
            count_vars_impl(r, map);
        }
    }
}

/// Substitute variables in `expr` using `subs`.  Variables not present in
/// `subs` are left unchanged.
pub fn substitute_vars(expr: &AlgExpr, subs: &HashMap<String, AlgExpr>) -> AlgExpr {
    match expr {
        AlgExpr::Const(n) => AlgExpr::Const(*n),
        AlgExpr::Var(v) => subs
            .get(v)
            .cloned()
            .unwrap_or_else(|| AlgExpr::Var(v.clone())),
        AlgExpr::Neg(e) => AlgExpr::Neg(Box::new(substitute_vars(e, subs))),
        AlgExpr::Add(l, r) => AlgExpr::Add(
            Box::new(substitute_vars(l, subs)),
            Box::new(substitute_vars(r, subs)),
        ),
        AlgExpr::Sub(l, r) => AlgExpr::Sub(
            Box::new(substitute_vars(l, subs)),
            Box::new(substitute_vars(r, subs)),
        ),
        AlgExpr::Mul(l, r) => AlgExpr::Mul(
            Box::new(substitute_vars(l, subs)),
            Box::new(substitute_vars(r, subs)),
        ),
        AlgExpr::Div(l, r) => AlgExpr::Div(
            Box::new(substitute_vars(l, subs)),
            Box::new(substitute_vars(r, subs)),
        ),
        AlgExpr::Pow(l, r) => AlgExpr::Pow(
            Box::new(substitute_vars(l, subs)),
            Box::new(substitute_vars(r, subs)),
        ),
        AlgExpr::Mod(l, r) => AlgExpr::Mod(
            Box::new(substitute_vars(l, subs)),
            Box::new(substitute_vars(r, subs)),
        ),
    }
}

/// Format an expression as a human-readable string.
pub fn alg_expr_to_string(expr: &AlgExpr) -> String {
    match expr {
        AlgExpr::Const(n) => n.to_string(),
        AlgExpr::Var(v) => v.clone(),
        AlgExpr::Neg(e) => format!("(-{})", alg_expr_to_string(e)),
        AlgExpr::Add(l, r) => format!("({} + {})", alg_expr_to_string(l), alg_expr_to_string(r)),
        AlgExpr::Sub(l, r) => format!("({} - {})", alg_expr_to_string(l), alg_expr_to_string(r)),
        AlgExpr::Mul(l, r) => format!("({} * {})", alg_expr_to_string(l), alg_expr_to_string(r)),
        AlgExpr::Div(l, r) => format!("({} / {})", alg_expr_to_string(l), alg_expr_to_string(r)),
        AlgExpr::Pow(l, r) => format!("({} ^ {})", alg_expr_to_string(l), alg_expr_to_string(r)),
        AlgExpr::Mod(l, r) => format!("({} % {})", alg_expr_to_string(l), alg_expr_to_string(r)),
    }
}

// ── Core simplification steps ─────────────────────────────────────────────────

/// Try to evaluate `expr` to a constant if both children are constants.
///
/// Returns `Some(simplified)` if folding was possible, `None` otherwise.
pub fn fold_constants(expr: &AlgExpr) -> Option<AlgExpr> {
    match expr {
        AlgExpr::Add(l, r) => {
            if let (AlgExpr::Const(a), AlgExpr::Const(b)) = (l.as_ref(), r.as_ref()) {
                return a.checked_add(*b).map(AlgExpr::Const);
            }
        }
        AlgExpr::Sub(l, r) => {
            if let (AlgExpr::Const(a), AlgExpr::Const(b)) = (l.as_ref(), r.as_ref()) {
                return a.checked_sub(*b).map(AlgExpr::Const);
            }
        }
        AlgExpr::Mul(l, r) => {
            if let (AlgExpr::Const(a), AlgExpr::Const(b)) = (l.as_ref(), r.as_ref()) {
                return a.checked_mul(*b).map(AlgExpr::Const);
            }
        }
        AlgExpr::Div(l, r) => {
            if let (AlgExpr::Const(a), AlgExpr::Const(b)) = (l.as_ref(), r.as_ref()) {
                if *b != 0 {
                    return a.checked_div(*b).map(AlgExpr::Const);
                }
            }
        }
        AlgExpr::Mod(l, r) => {
            if let (AlgExpr::Const(a), AlgExpr::Const(b)) = (l.as_ref(), r.as_ref()) {
                if *b != 0 {
                    return a.checked_rem(*b).map(AlgExpr::Const);
                }
            }
        }
        AlgExpr::Neg(e) => {
            if let AlgExpr::Const(n) = e.as_ref() {
                return n.checked_neg().map(AlgExpr::Const);
            }
        }
        AlgExpr::Pow(base, exp) => {
            if let (AlgExpr::Const(b), AlgExpr::Const(e)) = (base.as_ref(), exp.as_ref()) {
                if *e >= 0 {
                    let e_u32 = *e as u32;
                    return b.checked_pow(e_u32).map(AlgExpr::Const);
                }
            }
        }
        _ => {}
    }
    None
}

/// Try to apply a single algebraic identity to the top-level node of `expr`.
///
/// Returns `Some((simplified_expr, rule_name))` when a rule fires.
///
/// Identities applied:
/// - `x + 0 = x`, `0 + x = x`
/// - `x * 1 = x`, `1 * x = x`
/// - `x * 0 = 0`, `0 * x = 0`
/// - `x - 0 = x`
/// - `x + x = 2 * x`
/// - `x - x = 0`
/// - `x / x = 1` (when x is a non-zero constant or a variable)
/// - `0 - x = -x`
/// - `-(-x) = x`
/// - `x ^ 0 = 1`
/// - `x ^ 1 = x`
/// - `0 ^ x = 0` (x must be a positive constant)
pub fn apply_identity(expr: &AlgExpr) -> Option<(AlgExpr, String)> {
    match expr {
        // x + 0 = x
        AlgExpr::Add(_, r) if *r.as_ref() == AlgExpr::Const(0) => {
            Some((expr_left(expr).clone(), "add_zero_right".to_string()))
        }
        // 0 + x = x
        AlgExpr::Add(l, _) if *l.as_ref() == AlgExpr::Const(0) => {
            Some((expr_right(expr).clone(), "add_zero_left".to_string()))
        }
        // x - 0 = x
        AlgExpr::Sub(_, r) if *r.as_ref() == AlgExpr::Const(0) => {
            Some((expr_left(expr).clone(), "sub_zero".to_string()))
        }
        // 0 - x = -x
        AlgExpr::Sub(l, _) if *l.as_ref() == AlgExpr::Const(0) => Some((
            AlgExpr::Neg(Box::new(expr_right(expr).clone())),
            "neg_zero".to_string(),
        )),
        // x * 1 = x
        AlgExpr::Mul(_, r) if *r.as_ref() == AlgExpr::Const(1) => {
            Some((expr_left(expr).clone(), "mul_one_right".to_string()))
        }
        // 1 * x = x
        AlgExpr::Mul(l, _) if *l.as_ref() == AlgExpr::Const(1) => {
            Some((expr_right(expr).clone(), "mul_one_left".to_string()))
        }
        // x * 0 = 0
        AlgExpr::Mul(_, r) if *r.as_ref() == AlgExpr::Const(0) => {
            Some((AlgExpr::Const(0), "mul_zero_right".to_string()))
        }
        // 0 * x = 0
        AlgExpr::Mul(l, _) if *l.as_ref() == AlgExpr::Const(0) => {
            Some((AlgExpr::Const(0), "mul_zero_left".to_string()))
        }
        // x + x = 2 * x
        AlgExpr::Add(l, r) if l == r => Some((
            AlgExpr::Mul(Box::new(AlgExpr::Const(2)), Box::new(l.as_ref().clone())),
            "add_self".to_string(),
        )),
        // x - x = 0
        AlgExpr::Sub(l, r) if l == r => Some((AlgExpr::Const(0), "sub_self".to_string())),
        // x / x = 1 (when x ≠ 0; we check constants and variables)
        AlgExpr::Div(l, r) if l == r => match l.as_ref() {
            AlgExpr::Const(n) if *n != 0 => Some((AlgExpr::Const(1), "div_self".to_string())),
            AlgExpr::Var(_) => Some((AlgExpr::Const(1), "div_self".to_string())),
            _ => None,
        },
        // -(-x) = x
        AlgExpr::Neg(inner) => match inner.as_ref() {
            AlgExpr::Neg(inner2) => Some((inner2.as_ref().clone(), "double_neg".to_string())),
            _ => None,
        },
        // x ^ 0 = 1
        AlgExpr::Pow(_, r) if *r.as_ref() == AlgExpr::Const(0) => {
            Some((AlgExpr::Const(1), "pow_zero".to_string()))
        }
        // x ^ 1 = x
        AlgExpr::Pow(_, r) if *r.as_ref() == AlgExpr::Const(1) => {
            Some((expr_left(expr).clone(), "pow_one".to_string()))
        }
        // 0 ^ x = 0 (x > 0)
        AlgExpr::Pow(l, r) if *l.as_ref() == AlgExpr::Const(0) => match r.as_ref() {
            AlgExpr::Const(e) if *e > 0 => Some((AlgExpr::Const(0), "zero_pow".to_string())),
            _ => None,
        },
        _ => None,
    }
}

/// Helper: extract left child of a binary expression.
fn expr_left(expr: &AlgExpr) -> &AlgExpr {
    match expr {
        AlgExpr::Add(l, _)
        | AlgExpr::Sub(l, _)
        | AlgExpr::Mul(l, _)
        | AlgExpr::Div(l, _)
        | AlgExpr::Pow(l, _)
        | AlgExpr::Mod(l, _) => l,
        _ => expr,
    }
}

/// Helper: extract right child of a binary expression.
fn expr_right(expr: &AlgExpr) -> &AlgExpr {
    match expr {
        AlgExpr::Add(_, r)
        | AlgExpr::Sub(_, r)
        | AlgExpr::Mul(_, r)
        | AlgExpr::Div(_, r)
        | AlgExpr::Pow(_, r)
        | AlgExpr::Mod(_, r) => r,
        _ => expr,
    }
}

/// Produce a canonical form for `expr`:
/// - For commutative `Add` and `Mul`, sort the operand string representations
///   so that `a + b` and `b + a` both become the same canonical form.
/// - Flatten nested `Add(Add(a,b),c)` → `Add(a, Add(b,c))`.
/// - Flatten nested `Mul(Mul(a,b),c)` → `Mul(a, Mul(b,c))`.
pub fn normalize(expr: &AlgExpr) -> AlgExpr {
    match expr {
        AlgExpr::Const(_) | AlgExpr::Var(_) => expr.clone(),
        AlgExpr::Neg(e) => AlgExpr::Neg(Box::new(normalize(e))),
        AlgExpr::Add(l, r) => {
            let nl = normalize(l);
            let nr = normalize(r);
            // Flatten left-nested add: (a+b)+c => a+(b+c)
            if let AlgExpr::Add(ll, lr) = nl.clone() {
                return normalize(&AlgExpr::Add(ll, Box::new(AlgExpr::Add(lr, Box::new(nr)))));
            }
            // Sort for commutativity
            let ls = alg_expr_to_string(&nl);
            let rs = alg_expr_to_string(&nr);
            if ls <= rs {
                AlgExpr::Add(Box::new(nl), Box::new(nr))
            } else {
                AlgExpr::Add(Box::new(nr), Box::new(nl))
            }
        }
        AlgExpr::Mul(l, r) => {
            let nl = normalize(l);
            let nr = normalize(r);
            // Flatten left-nested mul: (a*b)*c => a*(b*c)
            if let AlgExpr::Mul(ll, lr) = nl.clone() {
                return normalize(&AlgExpr::Mul(ll, Box::new(AlgExpr::Mul(lr, Box::new(nr)))));
            }
            // Sort for commutativity
            let ls = alg_expr_to_string(&nl);
            let rs = alg_expr_to_string(&nr);
            if ls <= rs {
                AlgExpr::Mul(Box::new(nl), Box::new(nr))
            } else {
                AlgExpr::Mul(Box::new(nr), Box::new(nl))
            }
        }
        AlgExpr::Sub(l, r) => AlgExpr::Sub(Box::new(normalize(l)), Box::new(normalize(r))),
        AlgExpr::Div(l, r) => AlgExpr::Div(Box::new(normalize(l)), Box::new(normalize(r))),
        AlgExpr::Pow(l, r) => AlgExpr::Pow(Box::new(normalize(l)), Box::new(normalize(r))),
        AlgExpr::Mod(l, r) => AlgExpr::Mod(Box::new(normalize(l)), Box::new(normalize(r))),
    }
}

/// Apply one simplification step (fold_constants or apply_identity) to the
/// top-level node of `expr`.  Recurse into children first.
///
/// Returns `(simplified, changed, rule_name)`.
fn simplify_step(expr: AlgExpr, fold: bool) -> (AlgExpr, bool, Option<String>) {
    // Recurse into children first.
    let expr = match expr {
        AlgExpr::Neg(e) => {
            let (se, _, _) = simplify_step(*e, fold);
            AlgExpr::Neg(Box::new(se))
        }
        AlgExpr::Add(l, r) => {
            let (sl, _, _) = simplify_step(*l, fold);
            let (sr, _, _) = simplify_step(*r, fold);
            AlgExpr::Add(Box::new(sl), Box::new(sr))
        }
        AlgExpr::Sub(l, r) => {
            let (sl, _, _) = simplify_step(*l, fold);
            let (sr, _, _) = simplify_step(*r, fold);
            AlgExpr::Sub(Box::new(sl), Box::new(sr))
        }
        AlgExpr::Mul(l, r) => {
            let (sl, _, _) = simplify_step(*l, fold);
            let (sr, _, _) = simplify_step(*r, fold);
            AlgExpr::Mul(Box::new(sl), Box::new(sr))
        }
        AlgExpr::Div(l, r) => {
            let (sl, _, _) = simplify_step(*l, fold);
            let (sr, _, _) = simplify_step(*r, fold);
            AlgExpr::Div(Box::new(sl), Box::new(sr))
        }
        AlgExpr::Pow(l, r) => {
            let (sl, _, _) = simplify_step(*l, fold);
            let (sr, _, _) = simplify_step(*r, fold);
            AlgExpr::Pow(Box::new(sl), Box::new(sr))
        }
        AlgExpr::Mod(l, r) => {
            let (sl, _, _) = simplify_step(*l, fold);
            let (sr, _, _) = simplify_step(*r, fold);
            AlgExpr::Mod(Box::new(sl), Box::new(sr))
        }
        other => other,
    };

    // Try constant folding.
    if fold {
        if let Some(folded) = fold_constants(&expr) {
            return (folded, true, Some("fold_constants".to_string()));
        }
    }

    // Try identity rules.
    if let Some((simplified, rule)) = apply_identity(&expr) {
        return (simplified, true, Some(rule));
    }

    (expr, false, None)
}

/// Simplify `expr` according to `cfg`, returning a `SimplResult` with the
/// simplified expression, a trace of applied rules, and statistics.
pub fn simplify(expr: AlgExpr, cfg: &AlgSimplConfig) -> SimplResult {
    let size_before = expr_size(&expr);
    let mut current = expr;
    let mut steps: Vec<String> = Vec::new();
    let mut passes = 0usize;
    for _pass in 0..cfg.max_passes {
        passes += 1;
        let (next, changed, rule) = simplify_step(current.clone(), cfg.fold_constants);
        if changed {
            if let Some(r) = rule {
                steps.push(format!(
                    "pass {}: {} => {}",
                    passes,
                    r,
                    alg_expr_to_string(&next)
                ));
            }
            current = normalize(&next);
        } else {
            break;
        }
    }

    let size_after = expr_size(&current);
    let reduced = size_after < size_before || !steps.is_empty();
    SimplResult {
        expr: current,
        steps,
        reduced,
    }
}

/// Return a `SimplStats` for simplifying `expr` with `cfg`.
pub fn simplify_with_stats(expr: AlgExpr, cfg: &AlgSimplConfig) -> (SimplResult, SimplStats) {
    let size_before = expr_size(&expr);
    let mut current = expr;
    let mut steps: Vec<String> = Vec::new();
    let mut passes_completed = 0usize;
    let mut rules_applied = 0usize;

    for _pass in 0..cfg.max_passes {
        passes_completed += 1;
        let (next, changed, rule) = simplify_step(current.clone(), cfg.fold_constants);
        if changed {
            if let Some(r) = rule {
                steps.push(format!(
                    "pass {}: {} => {}",
                    passes_completed,
                    r,
                    alg_expr_to_string(&next)
                ));
            }
            rules_applied += 1;
            current = normalize(&next);
        } else {
            break;
        }
    }

    let size_after = expr_size(&current);
    let reduced = size_after < size_before || !steps.is_empty();
    let result = SimplResult {
        expr: current,
        steps,
        reduced,
    };
    let stats = SimplStats {
        rules_applied,
        passes_completed,
        size_before,
        size_after,
    };
    (result, stats)
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::super::types::{AlgExpr, AlgSimplConfig};
    use super::*;

    fn c(n: i64) -> AlgExpr {
        AlgExpr::Const(n)
    }
    fn v(s: &str) -> AlgExpr {
        AlgExpr::Var(s.to_string())
    }
    fn add(l: AlgExpr, r: AlgExpr) -> AlgExpr {
        AlgExpr::Add(Box::new(l), Box::new(r))
    }
    fn sub(l: AlgExpr, r: AlgExpr) -> AlgExpr {
        AlgExpr::Sub(Box::new(l), Box::new(r))
    }
    fn mul(l: AlgExpr, r: AlgExpr) -> AlgExpr {
        AlgExpr::Mul(Box::new(l), Box::new(r))
    }
    fn div(l: AlgExpr, r: AlgExpr) -> AlgExpr {
        AlgExpr::Div(Box::new(l), Box::new(r))
    }
    fn neg(e: AlgExpr) -> AlgExpr {
        AlgExpr::Neg(Box::new(e))
    }
    fn pow(b: AlgExpr, e: AlgExpr) -> AlgExpr {
        AlgExpr::Pow(Box::new(b), Box::new(e))
    }
    fn cfg_default() -> AlgSimplConfig {
        AlgSimplConfig::default()
    }

    // ── fold_constants ────────────────────────────────────────────────────────

    #[test]
    fn test_fold_add_constants() {
        assert_eq!(fold_constants(&add(c(3), c(4))), Some(c(7)));
    }

    #[test]
    fn test_fold_sub_constants() {
        assert_eq!(fold_constants(&sub(c(10), c(3))), Some(c(7)));
    }

    #[test]
    fn test_fold_mul_constants() {
        assert_eq!(fold_constants(&mul(c(6), c(7))), Some(c(42)));
    }

    #[test]
    fn test_fold_div_constants() {
        assert_eq!(fold_constants(&div(c(12), c(4))), Some(c(3)));
    }

    #[test]
    fn test_fold_div_by_zero_returns_none() {
        assert_eq!(fold_constants(&div(c(5), c(0))), None);
    }

    #[test]
    fn test_fold_neg_constant() {
        assert_eq!(fold_constants(&neg(c(7))), Some(c(-7)));
    }

    #[test]
    fn test_fold_pow_constants() {
        assert_eq!(fold_constants(&pow(c(2), c(10))), Some(c(1024)));
    }

    #[test]
    fn test_fold_not_applicable_for_vars() {
        assert_eq!(fold_constants(&add(v("x"), c(1))), None);
    }

    // ── apply_identity ────────────────────────────────────────────────────────

    #[test]
    fn test_identity_add_zero_right() {
        let (e, rule) = apply_identity(&add(v("x"), c(0))).unwrap();
        assert_eq!(e, v("x"));
        assert_eq!(rule, "add_zero_right");
    }

    #[test]
    fn test_identity_add_zero_left() {
        let (e, rule) = apply_identity(&add(c(0), v("x"))).unwrap();
        assert_eq!(e, v("x"));
        assert_eq!(rule, "add_zero_left");
    }

    #[test]
    fn test_identity_mul_one_right() {
        let (e, rule) = apply_identity(&mul(v("x"), c(1))).unwrap();
        assert_eq!(e, v("x"));
        assert_eq!(rule, "mul_one_right");
    }

    #[test]
    fn test_identity_mul_zero_right() {
        let (e, rule) = apply_identity(&mul(v("x"), c(0))).unwrap();
        assert_eq!(e, c(0));
        assert_eq!(rule, "mul_zero_right");
    }

    #[test]
    fn test_identity_sub_zero() {
        let (e, rule) = apply_identity(&sub(v("x"), c(0))).unwrap();
        assert_eq!(e, v("x"));
        assert_eq!(rule, "sub_zero");
    }

    #[test]
    fn test_identity_add_self() {
        let (e, rule) = apply_identity(&add(v("x"), v("x"))).unwrap();
        assert_eq!(e, mul(c(2), v("x")));
        assert_eq!(rule, "add_self");
    }

    #[test]
    fn test_identity_sub_self() {
        let (e, rule) = apply_identity(&sub(v("x"), v("x"))).unwrap();
        assert_eq!(e, c(0));
        assert_eq!(rule, "sub_self");
    }

    #[test]
    fn test_identity_div_self_var() {
        let (e, rule) = apply_identity(&div(v("x"), v("x"))).unwrap();
        assert_eq!(e, c(1));
        assert_eq!(rule, "div_self");
    }

    #[test]
    fn test_identity_div_self_nonzero_const() {
        let (e, rule) = apply_identity(&div(c(5), c(5))).unwrap();
        assert_eq!(e, c(1));
        assert_eq!(rule, "div_self");
    }

    #[test]
    fn test_identity_neg_zero() {
        let (e, rule) = apply_identity(&sub(c(0), v("x"))).unwrap();
        assert_eq!(e, neg(v("x")));
        assert_eq!(rule, "neg_zero");
    }

    #[test]
    fn test_identity_double_neg() {
        let (e, rule) = apply_identity(&neg(neg(v("x")))).unwrap();
        assert_eq!(e, v("x"));
        assert_eq!(rule, "double_neg");
    }

    #[test]
    fn test_identity_pow_zero() {
        let (e, rule) = apply_identity(&pow(v("x"), c(0))).unwrap();
        assert_eq!(e, c(1));
        assert_eq!(rule, "pow_zero");
    }

    #[test]
    fn test_identity_pow_one() {
        let (e, rule) = apply_identity(&pow(v("x"), c(1))).unwrap();
        assert_eq!(e, v("x"));
        assert_eq!(rule, "pow_one");
    }

    #[test]
    fn test_identity_zero_pow() {
        let (e, rule) = apply_identity(&pow(c(0), c(3))).unwrap();
        assert_eq!(e, c(0));
        assert_eq!(rule, "zero_pow");
    }

    #[test]
    fn test_identity_no_match() {
        assert!(apply_identity(&add(v("x"), v("y"))).is_none());
    }

    // ── simplify ──────────────────────────────────────────────────────────────

    #[test]
    fn test_simplify_constant_fold_through() {
        let expr = add(c(3), c(4));
        let result = simplify(expr, &cfg_default());
        assert_eq!(result.expr, c(7));
        assert!(result.reduced);
    }

    #[test]
    fn test_simplify_add_zero() {
        let expr = add(v("x"), c(0));
        let result = simplify(expr, &cfg_default());
        assert_eq!(result.expr, v("x"));
    }

    #[test]
    fn test_simplify_mul_one() {
        let expr = mul(v("x"), c(1));
        let result = simplify(expr, &cfg_default());
        assert_eq!(result.expr, v("x"));
    }

    #[test]
    fn test_simplify_no_change() {
        let expr = add(v("x"), v("y"));
        let result = simplify(expr.clone(), &cfg_default());
        // normalize may reorder but no identity fires
        assert!(!result.steps.iter().any(|s| s.contains("fold")));
    }

    #[test]
    fn test_simplify_nested() {
        // (x + 0) * 1  =>  x
        let expr = mul(add(v("x"), c(0)), c(1));
        let result = simplify(expr, &cfg_default());
        assert_eq!(result.expr, v("x"));
    }

    // ── expr_size ─────────────────────────────────────────────────────────────

    #[test]
    fn test_expr_size_leaf() {
        assert_eq!(expr_size(&c(5)), 1);
        assert_eq!(expr_size(&v("x")), 1);
    }

    #[test]
    fn test_expr_size_add() {
        assert_eq!(expr_size(&add(c(1), c(2))), 3);
    }

    #[test]
    fn test_expr_size_nested() {
        assert_eq!(expr_size(&add(mul(v("a"), v("b")), c(1))), 5);
    }

    // ── count_vars ────────────────────────────────────────────────────────────

    #[test]
    fn test_count_vars_single() {
        let m = count_vars(&v("x"));
        assert_eq!(m.get("x"), Some(&1));
    }

    #[test]
    fn test_count_vars_repeated() {
        let m = count_vars(&add(v("x"), v("x")));
        assert_eq!(m.get("x"), Some(&2));
    }

    #[test]
    fn test_count_vars_multiple() {
        let m = count_vars(&add(v("x"), v("y")));
        assert_eq!(m.get("x"), Some(&1));
        assert_eq!(m.get("y"), Some(&1));
    }

    // ── substitute_vars ───────────────────────────────────────────────────────

    #[test]
    fn test_substitute_simple() {
        let mut subs = HashMap::new();
        subs.insert("x".to_string(), c(5));
        let result = substitute_vars(&v("x"), &subs);
        assert_eq!(result, c(5));
    }

    #[test]
    fn test_substitute_partial() {
        let mut subs = HashMap::new();
        subs.insert("x".to_string(), c(3));
        let result = substitute_vars(&add(v("x"), v("y")), &subs);
        assert_eq!(result, add(c(3), v("y")));
    }

    #[test]
    fn test_substitute_then_simplify() {
        let mut subs = HashMap::new();
        subs.insert("x".to_string(), c(0));
        let expr = add(v("x"), v("y"));
        let subbed = substitute_vars(&expr, &subs);
        let result = simplify(subbed, &cfg_default());
        assert_eq!(result.expr, v("y"));
    }

    // ── alg_expr_to_string ────────────────────────────────────────────────────

    #[test]
    fn test_to_string_const() {
        assert_eq!(alg_expr_to_string(&c(42)), "42");
    }

    #[test]
    fn test_to_string_var() {
        assert_eq!(alg_expr_to_string(&v("x")), "x");
    }

    #[test]
    fn test_to_string_add() {
        assert_eq!(alg_expr_to_string(&add(v("a"), c(1))), "(a + 1)");
    }

    // ── normalize ─────────────────────────────────────────────────────────────

    #[test]
    fn test_normalize_commutes_add() {
        let e1 = normalize(&add(v("z"), v("a")));
        let e2 = normalize(&add(v("a"), v("z")));
        assert_eq!(e1, e2, "normalize should produce same form for a+z and z+a");
    }

    #[test]
    fn test_normalize_commutes_mul() {
        let e1 = normalize(&mul(v("z"), v("a")));
        let e2 = normalize(&mul(v("a"), v("z")));
        assert_eq!(e1, e2);
    }
}
