//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Expr, Literal, Name};
use std::collections::HashMap;

use super::functions::TacticResult;
use super::functions::*;
use super::functions_2::{
    beta_reduce, decompose_app_tactic, eval_nat_expr, eval_tactic, extract_eq_sides,
    extract_iff_sides, parse_numeric_comparison, rewrite_in_expr, substitute_bvar_0,
};
use super::types::{Goal, NumCmp, SymLinCon, SymLinExpr, TacticError, TacticRegistry, TacticState};

/// Return `true` iff the numeric comparison `lhs cmp rhs` holds.
pub(super) fn numeric_comparison_holds(lhs: u64, cmp: NumCmp, rhs: u64) -> bool {
    match cmp {
        NumCmp::Eq => lhs == rhs,
        NumCmp::Ne => lhs != rhs,
        NumCmp::Lt => lhs < rhs,
        NumCmp::Le => lhs <= rhs,
        NumCmp::Gt => lhs > rhs,
        NumCmp::Ge => lhs >= rhs,
    }
}
/// Try to close the focused goal by numeric evaluation.
///
/// Succeeds (closes the goal) when the goal is a concrete numeric comparison
/// that evaluates to `true`.  Fails otherwise, allowing callers to fall back.
pub(super) fn try_close_numeric(state: &TacticState) -> TacticResult {
    let goal = get_focused_goal(state)?;
    if let Some((lhs, cmp, rhs)) = parse_numeric_comparison(&goal.target) {
        if numeric_comparison_holds(lhs, cmp, rhs) {
            return replace_focused(state, vec![]);
        }
        return Err(TacticError::TypeMismatch(format!(
            "numeric goal {} {:?} {} is false",
            lhs, cmp, rhs
        )));
    }
    Err(TacticError::TypeMismatch(
        "goal is not a closed numeric comparison".to_string(),
    ))
}
/// Parse a kernel `Expr` into a `SymLinExpr`, if it has linear structure.
fn parse_sym_lin_expr(expr: &Expr) -> Option<SymLinExpr> {
    if let Some(n) = eval_nat_expr(expr) {
        return Some(SymLinExpr::from_const(n as i64));
    }
    let (head, args) = decompose_app_tactic(expr);
    match head {
        Expr::Const(n, _) => {
            let s = n.to_string();
            match s.as_str() {
                "Nat.add" | "HAdd.hAdd" if args.len() >= 2 => {
                    let l = parse_sym_lin_expr(args[args.len() - 2])?;
                    let r = parse_sym_lin_expr(args[args.len() - 1])?;
                    Some(SymLinExpr::add(&l, &r))
                }
                "Nat.sub" | "HSub.hSub" if args.len() >= 2 => {
                    let l = parse_sym_lin_expr(args[args.len() - 2])?;
                    let r = parse_sym_lin_expr(args[args.len() - 1])?;
                    Some(SymLinExpr::add(&l, &r.negate()))
                }
                "Nat.mul" | "HMul.hMul" if args.len() >= 2 => {
                    let a = args[args.len() - 2];
                    let b = args[args.len() - 1];
                    if let Some(k) = eval_nat_expr(a) {
                        Some(parse_sym_lin_expr(b)?.scale(k as i64))
                    } else if let Some(k) = eval_nat_expr(b) {
                        Some(parse_sym_lin_expr(a)?.scale(k as i64))
                    } else {
                        None
                    }
                }
                "Nat.succ" if !args.is_empty() => {
                    let inner = parse_sym_lin_expr(args[args.len() - 1])?;
                    Some(SymLinExpr {
                        terms: inner.terms,
                        constant: inner.constant + 1,
                    })
                }
                _ if args.is_empty() => Some(SymLinExpr::from_var(s)),
                _ => {
                    let key = format!("__app_{}", expr as *const _ as usize);
                    Some(SymLinExpr::from_var(key))
                }
            }
        }
        Expr::FVar(id) => Some(SymLinExpr::from_var(format!("fv_{:?}", id))),
        Expr::BVar(i) => Some(SymLinExpr::from_var(format!("bv_{}", i))),
        _ => None,
    }
}
/// Parse `expr` as one or more `SymLinCon`s (equality gives two constraints).
pub(super) fn parse_sym_lin_cons(expr: &Expr) -> Option<Vec<SymLinCon>> {
    let (head, args) = decompose_app_tactic(expr);
    let name = if let Expr::Const(n, _) = head {
        n.to_string()
    } else {
        return None;
    };
    if args.len() < 2 {
        return None;
    }
    let lhs_expr = args[args.len() - 2];
    let rhs_expr = args[args.len() - 1];
    let l = parse_sym_lin_expr(lhs_expr)?;
    let r = parse_sym_lin_expr(rhs_expr)?;
    let diff = SymLinExpr::add(&l, &r.negate());
    match name.as_str() {
        "Nat.le" | "LE.le" => Some(vec![SymLinCon {
            lhs: diff,
            strict: false,
        }]),
        "Nat.lt" | "LT.lt" => Some(vec![SymLinCon {
            lhs: diff,
            strict: true,
        }]),
        "Nat.ge" | "GE.ge" => Some(vec![SymLinCon {
            lhs: diff.negate(),
            strict: false,
        }]),
        "Nat.gt" | "GT.gt" => Some(vec![SymLinCon {
            lhs: diff.negate(),
            strict: true,
        }]),
        "Eq" => Some(vec![
            SymLinCon {
                lhs: diff.clone(),
                strict: false,
            },
            SymLinCon {
                lhs: diff.negate(),
                strict: false,
            },
        ]),
        _ => None,
    }
}
/// Search for a Farkas certificate: find non-negative integer multipliers (≤ 3)
/// for subsets of constraints such that the weighted sum is contradictory.
pub(super) fn has_farkas_certificate(cons: &[SymLinCon]) -> bool {
    let n = cons.len();
    if n == 0 {
        return false;
    }
    for c in cons {
        if c.is_contradiction() {
            return true;
        }
    }
    let all_sum = cons
        .iter()
        .skip(1)
        .fold(cons[0].clone(), |acc, c| SymLinCon::add(&acc, c));
    if all_sum.is_contradiction() {
        return true;
    }
    for i in 0..n {
        for j in (i + 1)..n {
            if SymLinCon::add(&cons[i], &cons[j]).is_contradiction() {
                return true;
            }
        }
    }
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            for li in 1..=5i64 {
                for lj in 1..=5i64 {
                    if SymLinCon::add(&cons[i].scale(li), &cons[j].scale(lj)).is_contradiction() {
                        return true;
                    }
                }
            }
        }
    }
    if n <= 12 {
        for i in 0..n {
            for j in (i + 1)..n {
                for k in (j + 1)..n {
                    let s = SymLinCon::add(&SymLinCon::add(&cons[i], &cons[j]), &cons[k]);
                    if s.is_contradiction() {
                        return true;
                    }
                    for (li, lj, lk) in [
                        (2, 1, 1),
                        (1, 2, 1),
                        (1, 1, 2),
                        (2, 2, 1),
                        (2, 1, 2),
                        (1, 2, 2),
                    ]
                    .iter()
                    {
                        let s2 = SymLinCon::add(
                            &SymLinCon::add(&cons[i].scale(*li), &cons[j].scale(*lj)),
                            &cons[k].scale(*lk),
                        );
                        if s2.is_contradiction() {
                            return true;
                        }
                    }
                }
            }
        }
    }
    if n >= 2 {
        let all_vars: std::collections::HashSet<String> =
            cons.iter().flat_map(|c| c.vars()).collect();
        for var in &all_vars {
            let mut pos: Vec<&SymLinCon> = vec![];
            let mut neg: Vec<&SymLinCon> = vec![];
            let mut zero: Vec<&SymLinCon> = vec![];
            for c in cons {
                let coeff = c.coeff_of(var);
                if coeff > 0 {
                    pos.push(c);
                } else if coeff < 0 {
                    neg.push(c);
                } else {
                    zero.push(c);
                }
            }
            for p in &pos {
                for q in &neg {
                    let pc = p.coeff_of(var).abs();
                    let qc = q.coeff_of(var).abs();
                    let combined = SymLinCon::add(&p.scale(qc), &q.scale(pc));
                    let mut new_cons: Vec<SymLinCon> = zero.iter().map(|c| (*c).clone()).collect();
                    new_cons.push(combined);
                    for c in &new_cons {
                        if c.is_contradiction() {
                            return true;
                        }
                    }
                    for i in 0..new_cons.len() {
                        for j in (i + 1)..new_cons.len() {
                            if SymLinCon::add(&new_cons[i], &new_cons[j]).is_contradiction() {
                                return true;
                            }
                        }
                    }
                }
            }
        }
    }
    false
}
/// Try to close the focused goal using symbolic linear arithmetic over all hypotheses.
///
/// Negates the goal and collects it together with all hypothesis linear constraints,
/// then searches for a Farkas certificate proving the combined system contradictory.
pub(super) fn try_linarith_with_hyps(state: &TacticState) -> TacticResult {
    let goal = get_focused_goal(state)?;
    let goal_cons = parse_sym_lin_cons(&goal.target)
        .ok_or_else(|| TacticError::TypeMismatch("goal is not a linear comparison".to_string()))?;
    let neg_goal = goal_cons[0].negate();
    let mut all_cons: Vec<SymLinCon> = vec![neg_goal];
    for (_, ty) in &goal.hypotheses {
        if let Some(cs) = parse_sym_lin_cons(ty) {
            all_cons.extend(cs);
        }
    }
    if has_farkas_certificate(&all_cons) {
        replace_focused(state, vec![])
    } else {
        Err(TacticError::TypeMismatch(
            "linarith: no Farkas certificate found".to_string(),
        ))
    }
}
/// Finite case split for `fin_cases` / `interval_cases`.
///
/// Looks through the hypotheses of the focused goal for a bound of the form
/// `h : n < K` (or `n ≤ K-1`) where `K` is a small concrete literal
/// and `n` is a free variable name.  Generates one sub-goal per concrete
/// value `0, 1, …, K-1`.
///
/// Falls back to `sorry` when no usable bound is found.
pub(super) fn tactic_fin_cases_impl(state: &TacticState, subject: Option<&str>) -> TacticResult {
    let goal = get_focused_goal(state)?;
    const MAX_CASES: u64 = 64;
    for (hyp_name, hyp_ty) in &goal.hypotheses {
        let var_name_str = hyp_name.to_string();
        if let Some(s) = subject {
            if var_name_str != s {
                continue;
            }
        }
        let (head, args) = decompose_app_tactic(hyp_ty);
        let head_name = if let Expr::Const(n, _) = head {
            n.to_string()
        } else {
            continue;
        };
        let bound_info: Option<(u64, bool)> = match head_name.as_str() {
            "Nat.lt" | "LT.lt" if args.len() >= 2 => {
                eval_nat_expr(args[args.len() - 1]).map(|rhs| (rhs, true))
            }
            "Nat.le" | "LE.le" if args.len() >= 2 => {
                eval_nat_expr(args[args.len() - 1]).map(|rhs| (rhs, false))
            }
            "Fin" if !args.is_empty() => eval_nat_expr(args[args.len() - 1]).map(|n| (n, true)),
            "Finset.mem" | "Membership.Mem" | "Mem" if args.len() >= 2 => {
                let set_arg = args[args.len() - 1];
                let (set_head, set_args) = decompose_app_tactic(set_arg);
                if let Expr::Const(sn, _) = set_head {
                    if (sn.to_string() == "Finset.range" || sn.to_string() == "List.range")
                        && !set_args.is_empty()
                    {
                        eval_nat_expr(set_args[set_args.len() - 1]).map(|n| (n, true))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        };
        let (bound_val, is_strict) = match bound_info {
            Some(bi) => bi,
            None => continue,
        };
        let upper = if is_strict { bound_val } else { bound_val + 1 };
        if upper == 0 || upper > MAX_CASES {
            continue;
        }
        let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
        let mut sub_goals: Vec<Goal> = Vec::with_capacity(upper as usize);
        for val in 0..upper {
            let lit = Expr::Lit(Literal::Nat(val));
            let mut sub = goal.clone();
            sub.mvar_id = fresh_mvar_id();
            sub.name = Name::str(format!("{}_case_{}", goal.name, val));
            sub.tag = Some(format!("case val = {}", val));
            remove_hypothesis(&mut sub, hyp_name);
            let eq_ty = Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Eq"), vec![])),
                        Box::new(nat_ty.clone()),
                    )),
                    Box::new(Expr::Const(Name::str(&var_name_str), vec![])),
                )),
                Box::new(lit),
            );
            let eq_hyp_name = Name::str(format!("h_{}_eq_{}", var_name_str, val));
            sub.add_hypothesis(eq_hyp_name, eq_ty);
            sub_goals.push(sub);
        }
        return replace_focused(state, sub_goals);
    }
    tactic_sorry(state)
}
/// A monomial is a sorted list of variable keys (multi-set represented as sorted vec).
/// E.g. `a * b^2 = ["a", "b", "b"]`.
type Monomial = Vec<String>;
/// A polynomial is a map from monomial to non-zero integer coefficient.
type Polynomial = HashMap<Monomial, i64>;
fn poly_add(mut a: Polynomial, b: Polynomial) -> Polynomial {
    for (mon, coef) in b {
        *a.entry(mon).or_default() += coef;
    }
    a.retain(|_, c| *c != 0);
    a
}
fn poly_mul(a: &Polynomial, b: &Polynomial) -> Polynomial {
    let mut result: Polynomial = HashMap::new();
    for (mon_a, ca) in a {
        for (mon_b, cb) in b {
            let mut mon = mon_a.clone();
            mon.extend_from_slice(mon_b);
            mon.sort();
            *result.entry(mon).or_default() += ca * cb;
        }
    }
    result.retain(|_, c| *c != 0);
    result
}
fn poly_from_const(k: i64) -> Polynomial {
    let mut p = HashMap::new();
    if k != 0 {
        p.insert(vec![], k);
    }
    p
}
fn poly_from_var(name: String) -> Polynomial {
    let mut p = HashMap::new();
    p.insert(vec![name], 1);
    p
}
fn poly_negate(p: Polynomial) -> Polynomial {
    p.into_iter().map(|(mon, coef)| (mon, -coef)).collect()
}
fn poly_sub(a: Polynomial, b: Polynomial) -> Polynomial {
    poly_add(a, poly_negate(b))
}
#[allow(dead_code)]
fn poly_scale(p: Polynomial, k: i64) -> Polynomial {
    if k == 0 {
        HashMap::new()
    } else {
        p.into_iter()
            .map(|(mon, coef)| (mon, coef * k))
            .filter(|(_, c)| *c != 0)
            .collect()
    }
}
/// Normalize a kernel expression to a polynomial (returns None if non-polynomial structure).
fn normalize_to_poly(expr: &Expr) -> Option<Polynomial> {
    if let Some(n) = eval_nat_expr(expr) {
        return Some(poly_from_const(n as i64));
    }
    let (head, args) = decompose_app_tactic(expr);
    match head {
        Expr::Const(n, _) => {
            let s = n.to_string();
            match s.as_str() {
                "Nat.add" | "HAdd.hAdd" if args.len() >= 2 => {
                    let l = normalize_to_poly(args[args.len() - 2])?;
                    let r = normalize_to_poly(args[args.len() - 1])?;
                    Some(poly_add(l, r))
                }
                "Nat.mul" | "HMul.hMul" if args.len() >= 2 => {
                    let l = normalize_to_poly(args[args.len() - 2])?;
                    let r = normalize_to_poly(args[args.len() - 1])?;
                    Some(poly_mul(&l, &r))
                }
                "Nat.sub" | "HSub.hSub" if args.len() >= 2 => {
                    let l = normalize_to_poly(args[args.len() - 2])?;
                    let r = normalize_to_poly(args[args.len() - 1])?;
                    Some(poly_sub(l, r))
                }
                "Nat.succ" if !args.is_empty() => {
                    let inner = normalize_to_poly(args[args.len() - 1])?;
                    Some(poly_add(inner, poly_from_const(1)))
                }
                "Neg.neg" | "HNeg.hNeg" | "Int.neg" | "Int.negSucc" if !args.is_empty() => {
                    let inner = normalize_to_poly(args[args.len() - 1])?;
                    Some(poly_negate(inner))
                }
                "Int.add" if args.len() >= 2 => {
                    let l = normalize_to_poly(args[args.len() - 2])?;
                    let r = normalize_to_poly(args[args.len() - 1])?;
                    Some(poly_add(l, r))
                }
                "Int.mul" if args.len() >= 2 => {
                    let l = normalize_to_poly(args[args.len() - 2])?;
                    let r = normalize_to_poly(args[args.len() - 1])?;
                    Some(poly_mul(&l, &r))
                }
                "Int.sub" if args.len() >= 2 => {
                    let l = normalize_to_poly(args[args.len() - 2])?;
                    let r = normalize_to_poly(args[args.len() - 1])?;
                    Some(poly_sub(l, r))
                }
                "Nat.pow" | "HPow.hPow" if args.len() >= 2 => {
                    if let Some(exp) = eval_nat_expr(args[args.len() - 1]) {
                        if exp <= 8 {
                            let base = normalize_to_poly(args[args.len() - 2])?;
                            if exp == 0 {
                                return Some(poly_from_const(1));
                            }
                            let mut result = base.clone();
                            for _ in 1..exp {
                                result = poly_mul(&result, &base);
                            }
                            return Some(result);
                        }
                    }
                    Some(poly_from_var(format!(
                        "pow_{}_{}",
                        args.len(),
                        expr as *const _ as usize
                    )))
                }
                _ if args.is_empty() => Some(poly_from_var(s)),
                _ => Some(poly_from_var(format!("app_{}", expr as *const _ as usize))),
            }
        }
        Expr::FVar(id) => Some(poly_from_var(format!("fv_{:?}", id))),
        Expr::BVar(i) => Some(poly_from_var(format!("bv_{}", i))),
        _ => None,
    }
}
/// Try to close an equality goal by polynomial ring normalization.
///
/// Succeeds when both sides normalize to the same polynomial.
pub(super) fn try_ring_norm(state: &TacticState) -> TacticResult {
    let goal = get_focused_goal(state)?;
    let (head, args) = decompose_app_tactic(&goal.target);
    if !matches!(head, Expr::Const(n, _) if n == & Name::str("Eq")) {
        return Err(TacticError::TypeMismatch(
            "ring: goal is not an equality".to_string(),
        ));
    }
    if args.len() < 2 {
        return Err(TacticError::TypeMismatch("ring: malformed Eq".to_string()));
    }
    let lhs_expr = args[args.len() - 2];
    let rhs_expr = args[args.len() - 1];
    let lhs_poly = normalize_to_poly(lhs_expr)
        .ok_or_else(|| TacticError::TypeMismatch("ring: LHS not polynomial".to_string()))?;
    let rhs_poly = normalize_to_poly(rhs_expr)
        .ok_or_else(|| TacticError::TypeMismatch("ring: RHS not polynomial".to_string()))?;
    if lhs_poly == rhs_poly {
        replace_focused(state, vec![])
    } else {
        Err(TacticError::TypeMismatch(
            "ring: LHS and RHS normalize to different polynomials".to_string(),
        ))
    }
}
/// Check if an expression is the constant with the given name.
pub(super) fn is_const_named(e: &Expr, name: &str) -> bool {
    matches!(e, Expr::Const(n, _) if n == & Name::str(name))
}
/// Apply built-in simplification rules to an expression recursively.
///
/// Rules:
/// - `And True P`  → `P`
/// - `And P True`  → `P`
/// - `And False P` → `False`
/// - `And P False` → `False`
/// - `Or False P`  → `P`
/// - `Or P False`  → `P`
/// - `Or True P`   → `True`
/// - `Or P True`   → `True`
/// - `Not False`   → `True`
/// - `Not True`    → `False`
/// - `Pi(_ : True, body)` → `body` (with BVar(0) substituted away)
/// - `Pi(_ : P, True)`    → `True`
/// - `Eq T x x`           → `True`
pub(crate) fn apply_simp_rules(expr: &Expr) -> Expr {
    let simplified = match expr {
        Expr::App(f, a) => {
            let f2 = apply_simp_rules(f);
            let a2 = apply_simp_rules(a);
            Expr::App(Box::new(f2), Box::new(a2))
        }
        Expr::Lam(bi, n, ty, body) => Expr::Lam(
            *bi,
            n.clone(),
            Box::new(apply_simp_rules(ty)),
            Box::new(apply_simp_rules(body)),
        ),
        Expr::Pi(bi, n, ty, body) => Expr::Pi(
            *bi,
            n.clone(),
            Box::new(apply_simp_rules(ty)),
            Box::new(apply_simp_rules(body)),
        ),
        Expr::Let(n, ty, val, body) => Expr::Let(
            n.clone(),
            Box::new(apply_simp_rules(ty)),
            Box::new(apply_simp_rules(val)),
            Box::new(apply_simp_rules(body)),
        ),
        _ => expr.clone(),
    };
    apply_simp_top_level_rule(&simplified)
}
/// Apply one round of top-level simp rules to a (possibly already recursed) expression.
fn apply_simp_top_level_rule(expr: &Expr) -> Expr {
    let true_e = Expr::Const(Name::str("True"), vec![]);
    let false_e = Expr::Const(Name::str("False"), vec![]);
    match expr {
        Expr::App(f, p) => {
            if let Expr::App(head, arg1) = f.as_ref() {
                if is_const_named(head, "And") && is_const_named(arg1, "True") {
                    return *p.clone();
                }
                if is_const_named(head, "And") && is_const_named(arg1, "False") {
                    return false_e.clone();
                }
                if is_const_named(head, "And") && is_const_named(p, "True") {
                    return *arg1.clone();
                }
                if is_const_named(head, "And") && is_const_named(p, "False") {
                    return false_e.clone();
                }
                if is_const_named(head, "Or") && is_const_named(arg1, "False") {
                    return *p.clone();
                }
                if is_const_named(head, "Or") && is_const_named(arg1, "True") {
                    return true_e.clone();
                }
                if is_const_named(head, "Or") && is_const_named(p, "False") {
                    return *arg1.clone();
                }
                if is_const_named(head, "Or") && is_const_named(p, "True") {
                    return true_e.clone();
                }
                if let Expr::App(eq_ty, lhs) = head.as_ref() {
                    if is_const_named(eq_ty, "Eq") && **lhs == *p.as_ref() {
                        return true_e.clone();
                    }
                }
                if is_const_named(head, "Iff") && is_const_named(arg1, "True") {
                    return *p.clone();
                }
                if is_const_named(head, "Iff") && is_const_named(arg1, "False") {
                    return Expr::App(Box::new(Expr::Const(Name::str("Not"), vec![])), p.clone());
                }
                if is_const_named(head, "Iff") && is_const_named(p, "True") {
                    return *arg1.clone();
                }
                if is_const_named(head, "Iff") && is_const_named(p, "False") {
                    return Expr::App(
                        Box::new(Expr::Const(Name::str("Not"), vec![])),
                        arg1.clone(),
                    );
                }
                if is_const_named(head, "Iff") && arg1.as_ref() == p.as_ref() {
                    return true_e.clone();
                }
                if is_const_named(head, "And") && arg1.as_ref() == p.as_ref() {
                    return *p.clone();
                }
                if is_const_named(head, "Or") && arg1.as_ref() == p.as_ref() {
                    return *p.clone();
                }
                if is_const_named(head, "Nat.pow")
                    && matches!(p.as_ref(), Expr::Lit(Literal::Nat(0)))
                {
                    return Expr::Lit(Literal::Nat(1));
                }
                if is_const_named(head, "Nat.pow")
                    && matches!(p.as_ref(), Expr::Lit(Literal::Nat(1)))
                {
                    return *arg1.clone();
                }
                if is_const_named(head, "Nat.pow")
                    && matches!(arg1.as_ref(), Expr::Lit(Literal::Nat(0)))
                {
                    if let Expr::Lit(Literal::Nat(exp)) = p.as_ref() {
                        return Expr::Lit(Literal::Nat(if *exp == 0 { 1 } else { 0 }));
                    }
                }
                if is_const_named(head, "Nat.pow")
                    && matches!(arg1.as_ref(), Expr::Lit(Literal::Nat(1)))
                {
                    return Expr::Lit(Literal::Nat(1));
                }
            }
            if is_const_named(f, "Not") && is_const_named(p, "False") {
                return true_e.clone();
            }
            if is_const_named(f, "Not") && is_const_named(p, "True") {
                return false_e.clone();
            }
            if is_const_named(f, "Not") {
                if let Expr::App(inner_f, inner_p) = p.as_ref() {
                    if is_const_named(inner_f, "Not") {
                        return *inner_p.clone();
                    }
                }
            }
            if let Expr::App(head, arg1) = f.as_ref() {
                let is_nat_add =
                    is_const_named(head, "Nat.add") || is_const_named(head, "HAdd.hAdd");
                let is_nat_mul =
                    is_const_named(head, "Nat.mul") || is_const_named(head, "HMul.hMul");
                let is_nat_sub =
                    is_const_named(head, "Nat.sub") || is_const_named(head, "HSub.hSub");
                let arg1_is_zero = matches!(arg1.as_ref(), Expr::Lit(Literal::Nat(0)));
                let arg2_is_zero = matches!(p.as_ref(), Expr::Lit(Literal::Nat(0)));
                let arg1_is_one = matches!(arg1.as_ref(), Expr::Lit(Literal::Nat(1)));
                let arg2_is_one = matches!(p.as_ref(), Expr::Lit(Literal::Nat(1)));
                if is_nat_add {
                    if arg1_is_zero {
                        return *p.clone();
                    }
                    if arg2_is_zero {
                        return *arg1.clone();
                    }
                }
                if is_nat_mul {
                    if arg1_is_zero || arg2_is_zero {
                        return Expr::Lit(Literal::Nat(0));
                    }
                    if arg1_is_one {
                        return *p.clone();
                    }
                    if arg2_is_one {
                        return *arg1.clone();
                    }
                }
                if is_nat_sub {
                    if arg2_is_zero {
                        return *arg1.clone();
                    }
                    if arg1_is_zero {
                        return Expr::Lit(Literal::Nat(0));
                    }
                    if arg1.as_ref() == p.as_ref() {
                        return Expr::Lit(Literal::Nat(0));
                    }
                }
                let is_nat_pow =
                    is_const_named(head, "Nat.pow") || is_const_named(head, "HPow.hPow");
                let is_nat_div =
                    is_const_named(head, "Nat.div") || is_const_named(head, "HDiv.hDiv");
                let is_nat_mod =
                    is_const_named(head, "Nat.mod") || is_const_named(head, "HMod.hMod");
                let is_nat_max = is_const_named(head, "Nat.max") || is_const_named(head, "Max.max");
                let is_nat_min = is_const_named(head, "Nat.min") || is_const_named(head, "Min.min");
                if (is_nat_max || is_nat_min) && arg1.as_ref() == p.as_ref() {
                    return *p.clone();
                }
                if is_nat_add
                    || is_nat_mul
                    || is_nat_sub
                    || is_nat_pow
                    || is_nat_div
                    || is_nat_mod
                    || is_nat_max
                    || is_nat_min
                {
                    if let (Some(a), Some(b)) = (eval_nat_expr(arg1), eval_nat_expr(p)) {
                        let result: Option<u64> = if is_nat_add {
                            Some(a.saturating_add(b))
                        } else if is_nat_mul {
                            Some(a.saturating_mul(b))
                        } else if is_nat_sub {
                            Some(a.saturating_sub(b))
                        } else if is_nat_pow {
                            if b < 20 {
                                Some(a.saturating_pow(b as u32))
                            } else {
                                None
                            }
                        } else if is_nat_div {
                            if b != 0 {
                                Some(a / b)
                            } else {
                                None
                            }
                        } else if is_nat_mod {
                            if b != 0 {
                                Some(a % b)
                            } else {
                                None
                            }
                        } else if is_nat_max {
                            Some(a.max(b))
                        } else if is_nat_min {
                            Some(a.min(b))
                        } else {
                            None
                        };
                        if let Some(n) = result {
                            return Expr::Lit(Literal::Nat(n));
                        }
                    }
                }
            }
            expr.clone()
        }
        Expr::Pi(_, _, domain, body) => {
            if is_const_named(domain, "True") {
                return substitute_bvar_0(body, &true_e);
            }
            if is_const_named(body, "True") {
                return true_e.clone();
            }
            expr.clone()
        }
        _ => expr.clone(),
    }
}
/// Apply extended simp rules for `ite`, decidability, and comparisons.
///
/// Called after the main simp loop for extra structural rules.
pub(super) fn apply_extended_simp_rules(expr: &Expr) -> Expr {
    let true_e = Expr::Const(Name::str("True"), vec![]);
    let false_e = Expr::Const(Name::str("False"), vec![]);
    let e = match expr {
        Expr::App(f, a) => Expr::App(
            Box::new(apply_extended_simp_rules(f)),
            Box::new(apply_extended_simp_rules(a)),
        ),
        Expr::Lam(bi, n, ty, body) => Expr::Lam(
            *bi,
            n.clone(),
            Box::new(apply_extended_simp_rules(ty)),
            Box::new(apply_extended_simp_rules(body)),
        ),
        Expr::Pi(bi, n, ty, body) => Expr::Pi(
            *bi,
            n.clone(),
            Box::new(apply_extended_simp_rules(ty)),
            Box::new(apply_extended_simp_rules(body)),
        ),
        _ => expr.clone(),
    };
    if let Expr::App(f3, b_else) = &e {
        if let Expr::App(f2, a_then) = f3.as_ref() {
            if let Expr::App(f1, cond) = f2.as_ref() {
                let head_name = if let Expr::Const(n, _) = f1.as_ref() {
                    n.to_string()
                } else {
                    String::new()
                };
                if head_name == "ite" || head_name == "Bool.ite" || head_name == "dite" {
                    if is_const_named(cond, "True") || is_const_named(cond, "true") {
                        return *a_then.clone();
                    }
                    if is_const_named(cond, "False") || is_const_named(cond, "false") {
                        return *b_else.clone();
                    }
                }
            }
        }
    }
    if let Expr::App(f, a) = &e {
        if is_const_named(f, "decide") || is_const_named(f, "Decidable.decide") {
            if is_const_named(a, "True") {
                return true_e.clone();
            }
            if is_const_named(a, "False") {
                return false_e.clone();
            }
        }
        if is_const_named(f, "Nat.succ") {
            if let Some(n) = eval_nat_expr(a) {
                return Expr::Lit(Literal::Nat(n + 1));
            }
        }
    }
    if let Expr::App(f, rhs) = &e {
        if let Expr::App(head, lhs) = f.as_ref() {
            let head_name = if let Expr::Const(n, _) = head.as_ref() {
                n.to_string()
            } else {
                String::new()
            };
            if head_name == "Nat.le" || head_name == "LE.le" {
                if lhs == rhs {
                    return true_e.clone();
                }
                if matches!(lhs.as_ref(), Expr::Lit(Literal::Nat(0))) {
                    return true_e.clone();
                }
            }
            if (head_name == "Nat.lt" || head_name == "LT.lt") && lhs == rhs {
                return false_e.clone();
            }
            if head_name == "Nat.ge" || head_name == "GE.ge" {
                if lhs == rhs {
                    return true_e.clone();
                }
                if matches!(rhs.as_ref(), Expr::Lit(Literal::Nat(0))) {
                    return true_e.clone();
                }
            }
            if (head_name == "Nat.gt" || head_name == "GT.gt") && lhs == rhs {
                return false_e.clone();
            }
        }
    }
    e
}
/// `simp` tactic: apply simplifications then try to close the goal.
///
/// Strategy:
/// 1. Try `trivial` (refl, assumption, True.intro)
/// 2. Apply beta reduction to goal target
/// 3. Apply built-in simp rules (And/Or/Not/Pi/Eq identities)
/// 4. Apply optional named rewrites (for `simp only`)
/// 5. If target became `True` or refl, close the goal
pub fn tactic_simp(state: &TacticState, lemma_names: &[&str]) -> TacticResult {
    if let Ok(s) = tactic_trivial(state) {
        return Ok(s);
    }
    let goal = get_focused_goal(state)?;
    let reduced_target = beta_reduce(&goal.target);
    let rule_simplified = apply_simp_rules(&reduced_target);
    let rule_simplified = apply_extended_simp_rules(&rule_simplified);
    let mut simplified = rule_simplified.clone();
    for hyp_name in lemma_names {
        let hyp_ty = goal.find_hypothesis(&Name::str(*hyp_name)).cloned();
        if let Some(ty) = hyp_ty {
            if let Some((from, to)) = extract_eq_sides(&ty, true) {
                simplified = rewrite_in_expr(&simplified, &from, &to);
            }
            if let Some((a, b)) = extract_iff_sides(&ty) {
                let prev = simplified.clone();
                simplified = rewrite_in_expr(&simplified, &a, &b);
                if simplified == prev {
                    simplified = rewrite_in_expr(&simplified, &b, &a);
                }
            }
        }
    }
    let simplified = apply_simp_rules(&simplified);
    let simplified = apply_extended_simp_rules(&simplified);
    if simplified != goal.target {
        let mut new_state = state.clone();
        new_state.goals[0].target = simplified.clone();
        new_state.goals[0].mvar_id = fresh_mvar_id();
        let is_true = is_const_named(&simplified, "True");
        if is_true || is_refl_target(&simplified) {
            return replace_focused(&new_state, vec![]);
        }
        if let Ok(s) = try_close_numeric(&new_state) {
            return Ok(s);
        }
        return Ok(new_state);
    }
    if let Ok(s) = try_close_numeric(state) {
        return Ok(s);
    }
    tactic_sorry(state)
}
/// Parse a `simp only [lem1, lem2, ...]` argument list.
///
/// Returns the list of lemma name strings from `only [lem1, lem2, ...]`.
pub(super) fn parse_simp_only_lemmas(args: &str) -> Vec<String> {
    let args = args.trim();
    let args = if let Some(rest) = args.strip_prefix("only") {
        rest.trim()
    } else {
        args
    };
    let args = args.trim_start_matches('[').trim_end_matches(']').trim();
    if args.is_empty() {
        return vec![];
    }
    args.split(',')
        .map(|s| {
            let s = s.trim();
            let s = s.strip_prefix('\u{2190}').unwrap_or(s).trim();
            let s = s.strip_prefix("<-").unwrap_or(s).trim();
            s.to_string()
        })
        .collect()
}
/// Evaluate a block of tactics in sequence.
pub fn eval_tactic_block(state: &TacticState, tactics: &[String]) -> TacticResult {
    let mut current = state.clone();
    for tactic_str in tactics {
        current = eval_tactic(&current, tactic_str)?;
    }
    Ok(current)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tactic::*;
    use oxilean_kernel::Level;
    fn mk_prop() -> Expr {
        Expr::Sort(Level::zero())
    }
    fn mk_type() -> Expr {
        Expr::Sort(Level::succ(Level::zero()))
    }
    fn mk_pi(name: &str, domain: Expr, body: Expr) -> Expr {
        Expr::Pi(
            BinderInfo::Default,
            Name::str(name),
            Box::new(domain),
            Box::new(body),
        )
    }
    fn mk_eq(ty: Expr, lhs: Expr, rhs: Expr) -> Expr {
        Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Eq"), vec![])),
                    Box::new(ty),
                )),
                Box::new(lhs),
            )),
            Box::new(rhs),
        )
    }
    fn mk_or(a: Expr, b: Expr) -> Expr {
        Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Or"), vec![])),
                Box::new(a),
            )),
            Box::new(b),
        )
    }
    fn mk_state_with_goal(goal: Goal) -> TacticState {
        let mut state = TacticState::new();
        state.add_goal(goal);
        state
    }
    #[test]
    fn test_error_display() {
        let e = TacticError::NoGoals;
        assert!(e.to_string().contains("no goals"));
        let e = TacticError::GoalNotFound(Name::str("g1"));
        assert!(e.to_string().contains("g1"));
        let e = TacticError::UnknownTactic("foo".to_string());
        assert!(e.to_string().contains("foo"));
    }
    #[test]
    fn test_goal_create() {
        let goal = Goal::new(Name::str("g1"), Expr::Lit(Literal::Nat(42)));
        assert_eq!(goal.name, Name::str("g1"));
        assert_eq!(goal.hypotheses().len(), 0);
        assert!(goal.tag.is_none());
    }
    #[test]
    fn test_goal_add_hypothesis() {
        let mut goal = Goal::new(Name::str("g1"), mk_prop());
        goal.add_hypothesis(Name::str("h"), mk_type());
        assert_eq!(goal.hypotheses().len(), 1);
        assert_eq!(goal.local_ctx.len(), 1);
    }
    #[test]
    fn test_goal_with_hypothesis() {
        let goal = Goal::new(Name::str("g1"), mk_prop());
        let new_goal = goal.with_hypothesis(Name::str("h"), mk_type());
        assert_eq!(new_goal.hypotheses().len(), 1);
        assert_eq!(goal.hypotheses().len(), 0);
    }
    #[test]
    fn test_goal_replace_target() {
        let goal = Goal::new(Name::str("g1"), mk_prop());
        let new_goal = goal.replace_target(mk_type());
        assert_eq!(new_goal.target(), &mk_type());
        assert_ne!(new_goal.mvar_id, goal.mvar_id);
    }
    #[test]
    fn test_goal_has_hypothesis() {
        let mut goal = Goal::new(Name::str("g1"), mk_prop());
        goal.add_hypothesis(Name::str("h"), mk_type());
        assert!(goal.has_hypothesis(&Name::str("h")));
        assert!(!goal.has_hypothesis(&Name::str("x")));
    }
    #[test]
    fn test_goal_find_hypothesis() {
        let mut goal = Goal::new(Name::str("g1"), mk_prop());
        goal.add_hypothesis(Name::str("h"), mk_type());
        assert_eq!(goal.find_hypothesis(&Name::str("h")), Some(&mk_type()));
        assert_eq!(goal.find_hypothesis(&Name::str("x")), None);
    }
    #[test]
    fn test_goal_local_names() {
        let mut goal = Goal::new(Name::str("g1"), mk_prop());
        goal.add_hypothesis(Name::str("h1"), mk_type());
        goal.add_hypothesis(Name::str("h2"), mk_prop());
        let names = goal.local_names();
        assert_eq!(names.len(), 2);
    }
    #[test]
    fn test_tactic_state() {
        let mut state = TacticState::new();
        assert!(state.is_complete());
        let goal = Goal::new(Name::str("g1"), mk_prop());
        state.add_goal(goal);
        assert_eq!(state.num_goals(), 1);
        assert!(!state.is_complete());
        state.solve_goal(&Name::str("g1"));
        assert_eq!(state.num_goals(), 0);
        assert!(state.is_complete());
    }
    #[test]
    fn test_state_focus() {
        let mut state = TacticState::new();
        assert!(state.focus().is_none());
        state.add_goal(Goal::new(Name::str("g1"), mk_prop()));
        state.add_goal(Goal::new(Name::str("g2"), mk_type()));
        assert_eq!(
            state.focus().expect("test operation should succeed").name,
            Name::str("g1")
        );
    }
    #[test]
    fn test_state_rotate() {
        let mut state = TacticState::new();
        state.add_goal(Goal::new(Name::str("g1"), mk_prop()));
        state.add_goal(Goal::new(Name::str("g2"), mk_type()));
        state.add_goal(Goal::new(Name::str("g3"), mk_prop()));
        state.rotate(1);
        assert_eq!(state.goals()[0].name, Name::str("g2"));
        assert_eq!(state.goals()[2].name, Name::str("g1"));
    }
    #[test]
    fn test_state_swap() {
        let mut state = TacticState::new();
        state.add_goal(Goal::new(Name::str("g1"), mk_prop()));
        state.add_goal(Goal::new(Name::str("g2"), mk_type()));
        state.swap();
        assert_eq!(state.goals()[0].name, Name::str("g2"));
        assert_eq!(state.goals()[1].name, Name::str("g1"));
    }
    #[test]
    fn test_state_replace_goal() {
        let mut state = TacticState::new();
        state.add_goal(Goal::new(Name::str("g1"), mk_prop()));
        state.replace_goal(
            &Name::str("g1"),
            vec![
                Goal::new(Name::str("g1a"), mk_prop()),
                Goal::new(Name::str("g1b"), mk_type()),
            ],
        );
        assert_eq!(state.num_goals(), 2);
        assert_eq!(state.goals()[0].name, Name::str("g1a"));
    }
    #[test]
    fn test_state_save_restore() {
        let mut state = TacticState::new();
        state.add_goal(Goal::new(Name::str("g1"), mk_prop()));
        let saved = state.save_state();
        state.solve_goal(&Name::str("g1"));
        assert!(state.is_complete());
        state.restore_state(saved);
        assert_eq!(state.num_goals(), 1);
    }
    #[test]
    fn test_tactic_intro_pi() {
        let target = mk_pi("x", mk_type(), mk_prop());
        let goal = Goal::new(Name::str("g1"), target);
        let state = mk_state_with_goal(goal);
        let result = tactic_intro(&state, Name::str("x")).expect("tactic should succeed");
        assert_eq!(result.num_goals(), 1);
        let new_goal = &result.goals()[0];
        assert!(new_goal.has_hypothesis(&Name::str("x")));
        assert_eq!(new_goal.target(), &mk_prop());
    }
    #[test]
    fn test_tactic_intro_not_pi() {
        let goal = Goal::new(Name::str("g1"), mk_prop());
        let state = mk_state_with_goal(goal);
        let result = tactic_intro(&state, Name::str("x"));
        assert!(result.is_err());
    }
    #[test]
    fn test_tactic_intro_no_goals() {
        let state = TacticState::new();
        let result = tactic_intro(&state, Name::str("x"));
        assert!(matches!(result, Err(TacticError::NoGoals)));
    }
    #[test]
    fn test_tactic_intros() {
        let target = mk_pi("a", mk_type(), mk_pi("b", mk_type(), mk_prop()));
        let goal = Goal::new(Name::str("g1"), target);
        let state = mk_state_with_goal(goal);
        let result = tactic_intros(&state, &[Name::str("x"), Name::str("y")])
            .expect("tactic should succeed");
        assert_eq!(result.num_goals(), 1);
        let new_goal = &result.goals()[0];
        assert!(new_goal.has_hypothesis(&Name::str("x")));
        assert!(new_goal.has_hypothesis(&Name::str("y")));
    }
    #[test]
    fn test_tactic_exact() {
        let goal = Goal::new(Name::str("g1"), mk_prop());
        let state = mk_state_with_goal(goal);
        let result = tactic_exact(&state, mk_prop()).expect("tactic should succeed");
        assert!(result.is_complete());
    }
    #[test]
    fn test_tactic_exact_no_goals() {
        let state = TacticState::new();
        let result = tactic_exact(&state, mk_prop());
        assert!(result.is_err());
    }
    #[test]
    fn test_tactic_assumption_found() {
        let target = mk_prop();
        let mut goal = Goal::new(Name::str("g1"), target.clone());
        goal.add_hypothesis(Name::str("h"), target);
        let state = mk_state_with_goal(goal);
        let result = tactic_assumption(&state).expect("tactic should succeed");
        assert!(result.is_complete());
    }
    #[test]
    fn test_tactic_assumption_not_found() {
        let mut goal = Goal::new(Name::str("g1"), mk_prop());
        goal.add_hypothesis(Name::str("h"), mk_type());
        let state = mk_state_with_goal(goal);
        let result = tactic_assumption(&state);
        assert!(result.is_err());
    }
    #[test]
    fn test_tactic_refl_eq() {
        let nat_const = Expr::Const(Name::str("Nat"), vec![]);
        let zero = Expr::Lit(Literal::Nat(0));
        let target = mk_eq(nat_const, zero.clone(), zero);
        let goal = Goal::new(Name::str("g1"), target);
        let state = mk_state_with_goal(goal);
        let result = tactic_refl(&state).expect("tactic should succeed");
        assert!(result.is_complete());
    }
    #[test]
    fn test_tactic_refl_not_eq() {
        let nat_const = Expr::Const(Name::str("Nat"), vec![]);
        let zero = Expr::Lit(Literal::Nat(0));
        let one = Expr::Lit(Literal::Nat(1));
        let target = mk_eq(nat_const, zero, one);
        let goal = Goal::new(Name::str("g1"), target);
        let state = mk_state_with_goal(goal);
        let result = tactic_refl(&state);
        assert!(result.is_err());
    }
    #[test]
    fn test_tactic_trivial_refl() {
        let goal = Goal::new(Name::str("g1"), mk_prop());
        let state = mk_state_with_goal(goal);
        let result = tactic_trivial(&state).expect("tactic should succeed");
        assert!(result.is_complete());
    }
    #[test]
    fn test_tactic_trivial_assumption() {
        let target = mk_type();
        let mut goal = Goal::new(Name::str("g1"), target.clone());
        goal.add_hypothesis(Name::str("h"), target);
        let state = mk_state_with_goal(goal);
        let result = tactic_trivial(&state).expect("tactic should succeed");
        assert!(result.is_complete());
    }
    #[test]
    fn test_tactic_trivial_true() {
        let target = Expr::Const(Name::str("True"), vec![]);
        let goal = Goal::new(Name::str("g1"), target);
        let state = mk_state_with_goal(goal);
        let result = tactic_trivial(&state).expect("tactic should succeed");
        assert!(result.is_complete());
    }
    #[test]
    fn test_tactic_sorry() {
        let goal = Goal::new(Name::str("g1"), mk_type());
        let state = mk_state_with_goal(goal);
        let result = tactic_sorry(&state).expect("tactic should succeed");
        assert!(result.is_complete());
    }
    #[test]
    fn test_tactic_apply_matching() {
        let target = mk_prop();
        let goal = Goal::new(Name::str("g1"), target.clone());
        let state = mk_state_with_goal(goal);
        let result = tactic_apply(&state, target).expect("tactic should succeed");
        assert!(result.is_complete());
    }
    #[test]
    fn test_tactic_apply_pi() {
        let pi = mk_pi("x", mk_type(), mk_prop());
        let goal = Goal::new(Name::str("g1"), mk_prop());
        let state = mk_state_with_goal(goal);
        let result = tactic_apply(&state, pi).expect("tactic should succeed");
        assert_eq!(result.num_goals(), 1);
    }
    #[test]
    fn test_tactic_constructor_true() {
        let target = Expr::Const(Name::str("True"), vec![]);
        let goal = Goal::new(Name::str("g1"), target);
        let state = mk_state_with_goal(goal);
        let result = tactic_constructor(&state).expect("tactic should succeed");
        assert!(result.is_complete());
    }
    #[test]
    fn test_tactic_left_or() {
        let a = Expr::Const(Name::str("A"), vec![]);
        let b = Expr::Const(Name::str("B"), vec![]);
        let target = mk_or(a.clone(), b);
        let goal = Goal::new(Name::str("g1"), target);
        let state = mk_state_with_goal(goal);
        let result = tactic_left(&state).expect("tactic should succeed");
        assert_eq!(result.num_goals(), 1);
        assert_eq!(result.goals()[0].target(), &a);
    }
    #[test]
    fn test_tactic_right_or() {
        let a = Expr::Const(Name::str("A"), vec![]);
        let b = Expr::Const(Name::str("B"), vec![]);
        let target = mk_or(a, b.clone());
        let goal = Goal::new(Name::str("g1"), target);
        let state = mk_state_with_goal(goal);
        let result = tactic_right(&state).expect("tactic should succeed");
        assert_eq!(result.num_goals(), 1);
        assert_eq!(result.goals()[0].target(), &b);
    }
    #[test]
    fn test_tactic_exfalso() {
        let goal = Goal::new(Name::str("g1"), mk_type());
        let state = mk_state_with_goal(goal);
        let result = tactic_exfalso(&state).expect("tactic should succeed");
        assert_eq!(result.num_goals(), 1);
        assert_eq!(
            result.goals()[0].target(),
            &Expr::Const(Name::str("False"), vec![])
        );
    }
    #[test]
    fn test_tactic_clear() {
        let mut goal = Goal::new(Name::str("g1"), mk_prop());
        goal.add_hypothesis(Name::str("h"), mk_type());
        let state = mk_state_with_goal(goal);
        let result = tactic_clear(&state, &Name::str("h")).expect("tactic should succeed");
        assert!(!result.goals()[0].has_hypothesis(&Name::str("h")));
    }
    #[test]
    fn test_tactic_clear_not_found() {
        let goal = Goal::new(Name::str("g1"), mk_prop());
        let state = mk_state_with_goal(goal);
        let result = tactic_clear(&state, &Name::str("h"));
        assert!(result.is_err());
    }
    #[test]
    fn test_tactic_rename() {
        let mut goal = Goal::new(Name::str("g1"), mk_prop());
        goal.add_hypothesis(Name::str("old"), mk_type());
        let state = mk_state_with_goal(goal);
        let result = tactic_rename(&state, &Name::str("old"), Name::str("new_name"))
            .expect("tactic should succeed");
        assert!(result.goals()[0].has_hypothesis(&Name::str("new_name")));
        assert!(!result.goals()[0].has_hypothesis(&Name::str("old")));
    }
    #[test]
    fn test_tactic_revert() {
        let mut goal = Goal::new(Name::str("g1"), mk_prop());
        goal.add_hypothesis(Name::str("h"), mk_type());
        let state = mk_state_with_goal(goal);
        let result = tactic_revert(&state, &Name::str("h")).expect("tactic should succeed");
        let new_goal = &result.goals()[0];
        assert!(!new_goal.has_hypothesis(&Name::str("h")));
        assert!(new_goal.target().is_pi());
    }
    #[test]
    fn test_tactic_have_with_proof() {
        let goal = Goal::new(Name::str("g1"), mk_prop());
        let state = mk_state_with_goal(goal);
        let result = tactic_have(&state, Name::str("h"), mk_type(), Some(mk_type()))
            .expect("tactic should succeed");
        assert_eq!(result.num_goals(), 1);
        assert!(result.goals()[0].has_hypothesis(&Name::str("h")));
    }
    #[test]
    fn test_tactic_have_without_proof() {
        let goal = Goal::new(Name::str("g1"), mk_prop());
        let state = mk_state_with_goal(goal);
        let result =
            tactic_have(&state, Name::str("h"), mk_type(), None).expect("tactic should succeed");
        assert_eq!(result.num_goals(), 2);
    }
    #[test]
    fn test_tactic_suffices() {
        let goal = Goal::new(Name::str("g1"), mk_prop());
        let state = mk_state_with_goal(goal);
        let result =
            tactic_suffices(&state, Name::str("h"), mk_type()).expect("tactic should succeed");
        assert_eq!(result.num_goals(), 2);
    }
    #[test]
    fn test_eval_tactic_sorry() {
        let goal = Goal::new(Name::str("g1"), mk_type());
        let state = mk_state_with_goal(goal);
        let result = eval_tactic(&state, "sorry").expect("tactic should succeed");
        assert!(result.is_complete());
    }
    #[test]
    fn test_eval_tactic_intro() {
        let target = mk_pi("x", mk_type(), mk_prop());
        let goal = Goal::new(Name::str("g1"), target);
        let state = mk_state_with_goal(goal);
        let result = eval_tactic(&state, "intro x").expect("tactic should succeed");
        assert_eq!(result.num_goals(), 1);
    }
    #[test]
    fn test_eval_tactic_unknown() {
        let goal = Goal::new(Name::str("g1"), mk_prop());
        let state = mk_state_with_goal(goal);
        let result = eval_tactic(&state, "nonexistent");
        assert!(matches!(result, Err(TacticError::UnknownTactic(_))));
    }
    #[test]
    fn test_eval_tactic_block() {
        let target = mk_pi("x", mk_prop(), mk_prop());
        let mut goal = Goal::new(Name::str("g1"), target);
        goal.tag = Some("test".to_string());
        let state = mk_state_with_goal(goal);
        let tactics = vec!["intro h".to_string(), "sorry".to_string()];
        let result = eval_tactic_block(&state, &tactics).expect("tactic should succeed");
        assert!(result.is_complete());
    }
    #[test]
    fn test_eval_tactic_block_empty() {
        let goal = Goal::new(Name::str("g1"), mk_prop());
        let state = mk_state_with_goal(goal);
        let result = eval_tactic_block(&state, &[]).expect("tactic should succeed");
        assert_eq!(result.num_goals(), 1);
    }
    #[test]
    fn test_tactic_registry() {
        let registry = TacticRegistry::default();
        assert!(registry.get(&Name::str("intro")).is_some());
        assert!(registry.get(&Name::str("apply")).is_some());
        assert!(registry.get(&Name::str("exact")).is_some());
        assert!(registry.get(&Name::str("refl")).is_some());
        assert!(registry.get(&Name::str("sorry")).is_some());
        assert!(registry.get(&Name::str("unknown")).is_none());
    }
    #[test]
    fn test_tactic_registry_all() {
        let registry = TacticRegistry::default();
        let all = registry.all_tactics();
        assert!(all.len() >= 15);
    }
    #[test]
    fn test_tactic_registry_execute() {
        let registry = TacticRegistry::default();
        let goal = Goal::new(Name::str("g1"), mk_prop());
        let state = mk_state_with_goal(goal);
        let result = registry
            .execute("sorry", &state, &[])
            .expect("test operation should succeed");
        assert!(result.is_complete());
    }
    #[test]
    fn test_tactic_registry_execute_unknown() {
        let registry = TacticRegistry::default();
        let state = TacticState::new();
        let result = registry.execute("nonexistent", &state, &[]);
        assert!(result.is_err());
    }
    #[test]
    fn test_tactic_registry_arity() {
        let registry = TacticRegistry::default();
        assert_eq!(registry.arity(&Name::str("refl")), Some(Some(0)));
        assert_eq!(registry.arity(&Name::str("intro")), Some(Some(1)));
        assert_eq!(registry.arity(&Name::str("intros")), Some(None));
    }
}
