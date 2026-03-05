//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    LinearConstraint, LinearExpr, LinearTerm, OmegaConfig, OmegaExtConfig4200,
    OmegaExtConfigVal4200, OmegaExtDiag4200, OmegaExtDiff4200, OmegaExtPass4200,
    OmegaExtPipeline4200, OmegaExtResult4200, OmegaGoalParser, OmegaProof, OmegaResult,
    OmegaSolver, OmegaStep, TacticOmegaAnalysisPass, TacticOmegaConfig, TacticOmegaConfigValue,
    TacticOmegaDiagnostics, TacticOmegaDiff, TacticOmegaPipeline, TacticOmegaResult,
};
use crate::basic::MetaContext;
use crate::tactic::state::{TacticError, TacticResult, TacticState};
use oxilean_kernel::{Expr, Level, Literal, Name};

/// Fourier-Motzkin combination of a lower and upper bound constraint.
///
/// Given `a_lb * var + rest_lb ≤ 0` (a_lb < 0) and `a_ub * var + rest_ub ≤ 0` (a_ub > 0),
/// multiplies: `(-a_lb) * (ub constraint) + a_ub * (lb constraint)`.
pub(super) fn fm_combine(
    var: &Name,
    lb: &LinearConstraint,
    ub: &LinearConstraint,
) -> Option<LinearConstraint> {
    let lb_expr = lb.expr();
    let ub_expr = ub.expr();
    let lb_coeff = lb_expr.coeff_of(var);
    let ub_coeff = ub_expr.coeff_of(var);
    if lb_coeff >= 0 || ub_coeff <= 0 {
        return None;
    }
    let a = -lb_coeff;
    let b = ub_coeff;
    let combined = lb_expr.scale(b).add(&ub_expr.scale(a));
    let g = gcd(a, b);
    let simplified = combined.div_by(g);
    Some(LinearConstraint::Le(simplified))
}
/// Compute the greatest common divisor of two non-negative integers.
pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    if a == 0 {
        1
    } else {
        a
    }
}
/// Compute the least common multiple.
pub fn lcm(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a / gcd(a, b)) * b
    }
}
/// Integer floor division.
pub fn div_floor(a: i64, b: i64) -> i64 {
    debug_assert!(b != 0);
    let d = a / b;
    let r = a % b;
    if (r != 0) && ((r < 0) != (b < 0)) {
        d - 1
    } else {
        d
    }
}
/// Integer ceiling division.
pub fn div_ceil(a: i64, b: i64) -> i64 {
    debug_assert!(b != 0);
    let d = a / b;
    let r = a % b;
    if (r != 0) && ((r < 0) == (b < 0)) {
        d + 1
    } else {
        d
    }
}
/// Parse a goal expression into linear constraints.
pub(super) fn parse_omega_goal(expr: &Expr) -> TacticResult<Vec<LinearConstraint>> {
    match parse_comparison(expr) {
        Some(constraint) => Ok(vec![constraint]),
        None => Err(TacticError::GoalMismatch(
            "omega: goal is not a linear arithmetic statement".into(),
        )),
    }
}
/// Try to parse a comparison expression into a linear constraint.
pub(super) fn parse_comparison(expr: &Expr) -> Option<LinearConstraint> {
    match expr {
        Expr::App(func, rhs) => {
            if let Expr::App(func2, lhs) = func.as_ref() {
                if let Some(head) = get_const_name(func2) {
                    let a = lhs.as_ref().clone();
                    let b = rhs.as_ref().clone();
                    let la = expr_to_linear(&a)?;
                    let lb = expr_to_linear(&b)?;
                    let head_str = head.to_string();
                    return match head_str.as_str() {
                        "LE.le" | "Nat.ble" => Some(LinearConstraint::a_le_b(la, lb)),
                        "LT.lt" => Some(LinearConstraint::a_lt_b(la, lb)),
                        "GE.ge" => Some(LinearConstraint::a_ge_b(la, lb)),
                        "GT.gt" => Some(LinearConstraint::a_lt_b(lb, la)),
                        _ => None,
                    };
                }
                if let Expr::App(func3, _ty) = func2.as_ref() {
                    if let Some(head) = get_const_name(func3) {
                        if head.to_string() == "Eq" {
                            let a = lhs.as_ref().clone();
                            let b = rhs.as_ref().clone();
                            let la = expr_to_linear(&a)?;
                            let lb = expr_to_linear(&b)?;
                            return Some(LinearConstraint::a_eq_b(la, lb));
                        }
                    }
                }
            }
            None
        }
        _ => None,
    }
}
/// Convert an expression to a linear expression.
///
/// Handles:
/// - Numeric literals: `Lit(Literal::Nat(n))`
/// - Variables: `Const(name)`, `FVar(id)`, `BVar(i)` (as named vars)
/// - Arithmetic: `HAdd.hAdd`, `HSub.hSub`, `HMul.hMul`, `Nat.add`, `Nat.sub`, `Nat.mul`
/// - Successor: `Nat.succ x` -> `x + 1`
/// - Negation: `Neg.neg x` -> `-x`
pub(super) fn expr_to_linear(expr: &Expr) -> Option<LinearExpr> {
    match expr {
        Expr::Lit(Literal::Nat(n)) => Some(LinearExpr::constant(*n as i64)),
        Expr::Const(name, _) => Some(LinearExpr::var(name.clone())),
        Expr::FVar(fvar_id) => Some(LinearExpr::var(Name::str(format!("fvar_{}", fvar_id.0)))),
        Expr::BVar(i) => Some(LinearExpr::var(Name::str(format!("bvar_{i}")))),
        Expr::App(func, arg) => expr_to_linear_app(func, arg),
        _ => None,
    }
}
/// Parse an application node as a linear arithmetic expression.
pub(super) fn expr_to_linear_app(func: &Expr, arg: &Expr) -> Option<LinearExpr> {
    if let Some(fname) = get_app_head_name(func) {
        let fname_str = fname.to_string();
        if let Expr::Const(ref cname, _) = *func {
            match cname.to_string().as_str() {
                "Nat.succ" | "Nat.successor" => {
                    let x = expr_to_linear(arg)?;
                    return Some(x.add(&LinearExpr::constant(1)));
                }
                "Neg.neg" | "Int.neg" => {
                    let x = expr_to_linear(arg)?;
                    return Some(x.negate());
                }
                _ => {}
            }
        }
        let _ = fname_str;
    }
    if let Expr::App(inner_func, lhs_expr) = func {
        if let Some(op_name) = extract_binary_op_name(inner_func) {
            let lhs = expr_to_linear(lhs_expr)?;
            let rhs = expr_to_linear(arg)?;
            return match op_name.as_str() {
                "add" => Some(lhs.add(&rhs)),
                "sub" => Some(lhs.sub(&rhs)),
                "mul" => linear_mul(&lhs, &rhs),
                _ => None,
            };
        }
        if let Expr::App(inner2, lhs2) = inner_func.as_ref() {
            if let Some(op_name) = extract_binary_op_name(inner2) {
                let lhs = expr_to_linear(lhs2)?;
                let rhs = expr_to_linear(arg)?;
                return match op_name.as_str() {
                    "add" => Some(lhs.add(&rhs)),
                    "sub" => Some(lhs.sub(&rhs)),
                    "mul" => linear_mul(&lhs, &rhs),
                    _ => None,
                };
            }
        }
    }
    None
}
/// Get the head constant name of a (possibly partially-applied) function expression.
pub(super) fn get_app_head_name(expr: &Expr) -> Option<Name> {
    match expr {
        Expr::Const(name, _) => Some(name.clone()),
        Expr::App(f, _) => get_app_head_name(f),
        _ => None,
    }
}
/// Extract a canonical binary operator name from a (possibly applied) expression.
///
/// Returns `"add"`, `"sub"`, or `"mul"` for known arithmetic operations.
pub(super) fn extract_binary_op_name(expr: &Expr) -> Option<String> {
    let head = get_app_head_name(expr)?;
    match head.to_string().as_str() {
        "HAdd.hAdd" | "Nat.add" | "Int.add" | "Add.add" => Some("add".into()),
        "HSub.hSub" | "Nat.sub" | "Int.sub" | "Sub.sub" => Some("sub".into()),
        "HMul.hMul" | "Nat.mul" | "Int.mul" | "Mul.mul" => Some("mul".into()),
        _ => None,
    }
}
/// Multiply two linear expressions; only linear if at most one has variables.
pub(super) fn linear_mul(lhs: &LinearExpr, rhs: &LinearExpr) -> Option<LinearExpr> {
    if lhs.is_constant() {
        Some(rhs.scale(lhs.constant))
    } else if rhs.is_constant() {
        Some(lhs.scale(rhs.constant))
    } else {
        None
    }
}
/// Get the constant name from an expression.
pub(super) fn get_const_name(expr: &Expr) -> Option<Name> {
    match expr {
        Expr::Const(name, _) => Some(name.clone()),
        _ => None,
    }
}
/// Build a proof term from an omega proof.
pub(super) fn build_omega_proof(_proof: &OmegaProof, _goal: &Expr) -> Expr {
    Expr::Const(Name::str("omega_proof"), vec![Level::zero()])
}
/// Run the omega tactic on the current goal.
pub fn tac_omega(state: &mut TacticState, ctx: &mut MetaContext) -> TacticResult<()> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let target = ctx.instantiate_mvars(&target);
    let constraints = parse_omega_goal(&target)?;
    let mut solver = OmegaSolver::new();
    match solver.solve(&constraints) {
        OmegaResult::Unsatisfiable(proof) => {
            let proof_term = build_omega_proof(&proof, &target);
            state.close_goal(proof_term, ctx)?;
            Ok(())
        }
        OmegaResult::Satisfiable => Err(TacticError::Failed(
            "omega: goal is not provable by linear arithmetic".into(),
        )),
    }
}
/// Tokenize a goal string into words, treating operators as separate tokens.
pub(super) fn tokenize_goal(s: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut cur = String::new();
    for c in s.chars() {
        if c == ' ' || c == '\t' || c == '\n' {
            if !cur.is_empty() {
                tokens.push(std::mem::take(&mut cur));
            }
        } else if c == '+' || c == '*' || c == '(' || c == ')' {
            if !cur.is_empty() {
                tokens.push(std::mem::take(&mut cur));
            }
            tokens.push(c.to_string());
        } else if c == '-' {
            if !cur.is_empty() {
                tokens.push(std::mem::take(&mut cur));
            }
            tokens.push("-".to_string());
        } else {
            cur.push(c);
        }
    }
    if !cur.is_empty() {
        tokens.push(cur);
    }
    tokens
}
/// Parse a linear expression from a tokenized slice (owned strings).
///
/// Handles: integers, identifiers, `+`, `-`, `*` (constant scaling only).
pub(super) fn parse_linear_str(tokens: &[String]) -> Option<LinearExpr> {
    if tokens.first().map(|s| s.as_str()) == Some("(")
        && tokens.last().map(|s| s.as_str()) == Some(")")
    {
        return parse_linear_str(&tokens[1..tokens.len() - 1]);
    }
    let mut depth: i32 = 0;
    let mut split_pos: Option<usize> = None;
    let mut split_op = '+';
    for i in (0..tokens.len()).rev() {
        match tokens[i].as_str() {
            ")" => depth += 1,
            "(" => depth -= 1,
            "+" | "-" if depth == 0 && i > 0 => {
                split_pos = Some(i);
                split_op = tokens[i]
                    .chars()
                    .next()
                    .expect("matched '+' or '-' literal so it is non-empty");
                break;
            }
            _ => {}
        }
    }
    if let Some(pos) = split_pos {
        let lhs = parse_linear_str(&tokens[..pos])?;
        let rhs = parse_linear_str(&tokens[pos + 1..])?;
        return Some(if split_op == '+' {
            lhs.add(&rhs)
        } else {
            lhs.sub(&rhs)
        });
    }
    depth = 0;
    let mut mul_pos: Option<usize> = None;
    for (i, tok) in tokens.iter().enumerate() {
        match tok.as_str() {
            "(" => depth += 1,
            ")" => depth -= 1,
            "*" if depth == 0 => {
                mul_pos = Some(i);
                break;
            }
            _ => {}
        }
    }
    if let Some(pos) = mul_pos {
        let lhs = parse_linear_str(&tokens[..pos])?;
        let rhs = parse_linear_str(&tokens[pos + 1..])?;
        return linear_mul(&lhs, &rhs);
    }
    if tokens.len() == 1 {
        let t = &tokens[0];
        if let Ok(n) = t.parse::<i64>() {
            return Some(LinearExpr::constant(n));
        }
        if !t.is_empty()
            && t.chars()
                .all(|c| c.is_alphanumeric() || c == '_' || c == '\'')
        {
            return Some(LinearExpr::var(Name::str(t.as_str())));
        }
    }
    None
}
/// Parse a single constraint from a goal string like `"x + y <= z"` or `"a = b + 2"`.
///
/// Supported relation operators: `<=`, `<`, `>=`, `>`, `=`, `!=`, `≤`, `≥`, `≠`.
pub(super) fn parse_constraint_str(goal: &str) -> Option<LinearConstraint> {
    let goal = goal
        .replace('\u{2264}', "<=")
        .replace('\u{2265}', ">=")
        .replace('\u{2260}', "!=")
        .replace('\u{2261}', "=");
    let operators: &[(&str, &str)] = &[
        ("<=", "le"),
        ("!=", "ne"),
        (">=", "ge"),
        ("<", "lt"),
        (">", "gt"),
        ("=", "eq"),
    ];
    for &(op_str, op_kind) in operators {
        if let Some(pos) = goal.find(op_str) {
            let lhs_str = goal[..pos].trim();
            let rhs_str = goal[pos + op_str.len()..].trim();
            if lhs_str.is_empty() || rhs_str.is_empty() {
                continue;
            }
            let lhs_toks = tokenize_goal(lhs_str);
            let rhs_toks = tokenize_goal(rhs_str);
            let lhs = parse_linear_str(&lhs_toks)?;
            let rhs = parse_linear_str(&rhs_toks)?;
            return Some(match op_kind {
                "le" => LinearConstraint::a_le_b(lhs, rhs),
                "lt" => LinearConstraint::a_lt_b(lhs, rhs),
                "ge" => LinearConstraint::a_ge_b(lhs, rhs),
                "gt" => LinearConstraint::a_lt_b(rhs, lhs),
                "eq" => LinearConstraint::a_eq_b(lhs, rhs),
                "ne" => LinearConstraint::a_eq_b(lhs, rhs),
                _ => return None,
            });
        }
    }
    None
}
/// Negate a single linear constraint (for omega provability checking).
///
/// - `¬(e ≤ 0)` → `e ≥ 1` → represented as `Le(-e + 1 ≤ 0)` ... actually `-e - 1 ≤ 0`?
///   Wait: `¬(e ≤ 0)` ↔ `e > 0` ↔ `e ≥ 1` (integers) ↔ `-e ≤ -1` ↔ `-e + (-1) ≤ 0`
///   So: `Le(negate(e).add(constant(-1)))`.
/// - `¬(e ≥ 0)` → `e < 0` ↔ `e ≤ -1` ↔ `e + 1 ≤ 0`.
/// - `¬(e = 0)` → `e ≠ 0`: cannot represent as single linear constraint.
pub(super) fn negate_constraint(c: &LinearConstraint) -> Option<LinearConstraint> {
    match c {
        LinearConstraint::Le(e) => {
            let mut neg_e = e.negate();
            neg_e.constant -= 1;
            Some(LinearConstraint::Le(neg_e))
        }
        LinearConstraint::Ge(e) => {
            let mut shifted = e.clone();
            shifted.constant += 1;
            Some(LinearConstraint::Le(shifted))
        }
        LinearConstraint::Eq(_) => None,
    }
}
/// Run the omega decision procedure on a string-formatted goal.
///
/// Returns `true` if the goal is provable by linear arithmetic (the negation is
/// unsatisfiable), `false` otherwise (including on parse failure).
///
/// # Examples
/// ```
/// use oxilean_meta::tactic::omega::run_omega;
/// assert!(run_omega("1 <= 2"));
/// assert!(run_omega("0 = 0"));
/// assert!(!run_omega("2 <= 1"));
/// ```
pub fn run_omega(goal_str: &str) -> bool {
    let constraints = match OmegaGoalParser::parse_goal(goal_str) {
        Some(cs) => cs,
        None => return false,
    };
    if constraints.iter().all(|c| c.expr().is_constant()) {
        return constraints.iter().all(|c| c.is_trivially_true());
    }
    if constraints.len() == 1 {
        if let Some(neg) = negate_constraint(&constraints[0]) {
            return solve_omega(&[neg]).is_unsat();
        }
        return constraints[0].is_trivially_true();
    }
    let neg_constraints: Vec<LinearConstraint> =
        constraints.iter().filter_map(negate_constraint).collect();
    if neg_constraints.len() != constraints.len() {
        return constraints.iter().all(|c| c.is_trivially_true());
    }
    neg_constraints
        .iter()
        .all(|nc| solve_omega(std::slice::from_ref(nc)).is_unsat())
}
/// Solve a system of linear constraints.
pub fn solve_omega(constraints: &[LinearConstraint]) -> OmegaResult {
    OmegaSolver::new().solve(constraints)
}
/// Check if a system of constraints is satisfiable.
pub fn is_satisfiable(constraints: &[LinearConstraint]) -> bool {
    solve_omega(constraints).is_sat()
}
/// Check if a system of constraints is unsatisfiable.
pub fn is_unsatisfiable(constraints: &[LinearConstraint]) -> bool {
    solve_omega(constraints).is_unsat()
}
/// Solve using a specific config.
pub fn solve_omega_with_config(
    constraints: &[LinearConstraint],
    config: OmegaConfig,
) -> OmegaResult {
    OmegaSolver::with_config(config).solve(constraints)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tactic::omega::*;
    fn name(s: &str) -> Name {
        Name::str(s)
    }
    fn var(s: &str) -> LinearExpr {
        LinearExpr::var(name(s))
    }
    fn cst(n: i64) -> LinearExpr {
        LinearExpr::constant(n)
    }
    #[test]
    fn test_linear_expr_constant() {
        let e = cst(42);
        assert!(e.is_constant());
        assert_eq!(e.constant, 42);
        assert!(e.terms.is_empty());
    }
    #[test]
    fn test_linear_expr_var() {
        let e = var("x");
        assert!(!e.is_constant());
        assert_eq!(e.terms.len(), 1);
        assert_eq!(e.coeff_of(&name("x")), 1);
    }
    #[test]
    fn test_linear_expr_add_constants() {
        let a = cst(3);
        let b = cst(7);
        let sum = a.add(&b);
        assert!(sum.is_constant());
        assert_eq!(sum.constant, 10);
    }
    #[test]
    fn test_linear_expr_add_same_var() {
        let a = var("x");
        let b = var("x");
        let sum = a.add(&b);
        assert_eq!(sum.terms.len(), 1);
        assert_eq!(sum.terms[0].1, 2);
    }
    #[test]
    fn test_linear_expr_add_cancels() {
        let a = var("x");
        let b = var("x").negate();
        let sum = a.add(&b);
        assert!(sum.is_constant());
        assert_eq!(sum.constant, 0);
    }
    #[test]
    fn test_linear_expr_sub() {
        let a = var("x");
        let b = var("x");
        let diff = a.sub(&b);
        assert!(diff.is_constant());
        assert_eq!(diff.constant, 0);
    }
    #[test]
    fn test_linear_expr_negate() {
        let e = LinearExpr {
            constant: 3,
            terms: vec![(name("x"), 2)],
        };
        let neg = e.negate();
        assert_eq!(neg.constant, -3);
        assert_eq!(neg.terms[0].1, -2);
    }
    #[test]
    fn test_linear_expr_scale() {
        let e = var("x");
        let scaled = e.scale(3);
        assert_eq!(scaled.terms[0].1, 3);
    }
    #[test]
    fn test_linear_expr_scale_zero() {
        let e = var("x");
        let scaled = e.scale(0);
        assert!(scaled.is_constant());
        assert_eq!(scaled.constant, 0);
    }
    #[test]
    fn test_linear_expr_variables() {
        let e = var("x").add(&var("y")).add(&cst(5));
        let vars = e.variables();
        assert_eq!(vars.len(), 2);
    }
    #[test]
    fn test_linear_expr_gcd() {
        let e = LinearExpr {
            constant: 6,
            terms: vec![(name("x"), 4), (name("y"), 8)],
        };
        assert_eq!(e.gcd(), 2);
    }
    #[test]
    fn test_linear_expr_normalize() {
        let e = LinearExpr {
            constant: 6,
            terms: vec![(name("x"), 4)],
        };
        let n = e.normalize();
        assert_eq!(n.constant, 3);
        assert_eq!(n.terms[0].1, 2);
    }
    #[test]
    fn test_linear_expr_substitute() {
        let e = var("x").add(&cst(1));
        let replacement = var("y").add(&cst(2));
        let result = e.substitute(&name("x"), &replacement);
        assert_eq!(result.coeff_of(&name("y")), 1);
        assert_eq!(result.constant, 3);
    }
    #[test]
    fn test_linear_expr_display() {
        let e = cst(5);
        assert_eq!(format!("{e}"), "5");
        let e2 = var("x");
        assert!(format!("{e2}").contains('x'));
    }
    #[test]
    fn test_constraint_trivially_true_le() {
        let c = LinearConstraint::Le(cst(-1));
        assert!(c.is_trivially_true());
    }
    #[test]
    fn test_constraint_trivially_false_le() {
        let c = LinearConstraint::Le(cst(1));
        assert!(c.is_trivially_false());
    }
    #[test]
    fn test_constraint_trivially_true_eq() {
        let c = LinearConstraint::Eq(cst(0));
        assert!(c.is_trivially_true());
    }
    #[test]
    fn test_constraint_trivially_false_eq() {
        let c = LinearConstraint::Eq(cst(3));
        assert!(c.is_trivially_false());
    }
    #[test]
    fn test_constraint_a_le_b() {
        let a = cst(3);
        let b = cst(5);
        let c = LinearConstraint::a_le_b(a, b);
        assert!(c.is_trivially_true());
    }
    #[test]
    fn test_constraint_a_lt_b() {
        let c = LinearConstraint::a_lt_b(cst(3), cst(5));
        assert!(c.is_trivially_true());
    }
    #[test]
    fn test_constraint_a_lt_b_false() {
        let c = LinearConstraint::a_lt_b(cst(5), cst(3));
        assert!(c.is_trivially_false());
    }
    #[test]
    fn test_constraint_eq_a_eq_b() {
        let c = LinearConstraint::a_eq_b(cst(3), cst(3));
        assert!(c.is_trivially_true());
    }
    #[test]
    fn test_gcd_basic() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(7, 3), 1);
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(6, 0), 6);
    }
    #[test]
    fn test_lcm_basic() {
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(3, 7), 21);
        assert_eq!(lcm(0, 5), 0);
    }
    #[test]
    fn test_div_floor() {
        assert_eq!(div_floor(7, 2), 3);
        assert_eq!(div_floor(-7, 2), -4);
        assert_eq!(div_floor(7, -2), -4);
        assert_eq!(div_floor(-7, -2), 3);
    }
    #[test]
    fn test_div_ceil() {
        assert_eq!(div_ceil(7, 2), 4);
        assert_eq!(div_ceil(-7, 2), -3);
        assert_eq!(div_ceil(6, 2), 3);
    }
    #[test]
    fn test_solve_omega_empty() {
        let result = solve_omega(&[]);
        assert!(result.is_sat());
    }
    #[test]
    fn test_solve_omega_contradiction_le() {
        let constraints = vec![LinearConstraint::Le(cst(5))];
        let result = solve_omega(&constraints);
        assert!(result.is_unsat());
    }
    #[test]
    fn test_solve_omega_contradiction_eq() {
        let constraints = vec![LinearConstraint::Eq(cst(3))];
        let result = solve_omega(&constraints);
        assert!(result.is_unsat());
    }
    #[test]
    fn test_solve_omega_valid_le() {
        let constraints = vec![LinearConstraint::Le(cst(-1))];
        let result = solve_omega(&constraints);
        assert!(result.is_sat());
    }
    #[test]
    fn test_solve_omega_valid_eq() {
        let constraints = vec![LinearConstraint::Eq(cst(0))];
        let result = solve_omega(&constraints);
        assert!(result.is_sat());
    }
    #[test]
    fn test_solve_omega_variable_contradiction() {
        let constraints = vec![
            LinearConstraint::Le(var("x")),
            LinearConstraint::Le(var("x").negate().add(&cst(1))),
        ];
        let result = solve_omega(&constraints);
        assert!(result.is_unsat(), "x ≤ 0 ∧ x ≥ 1 should be unsatisfiable");
    }
    #[test]
    fn test_solve_omega_equality_elimination() {
        let constraints = [
            LinearConstraint::Eq(var("x").sub(&cst(5))),
            LinearConstraint::Le(var("x").negate().add(&cst(7)).negate()),
        ];
        let result = solve_omega(&constraints[..1]);
        assert!(result.is_sat());
    }
    #[test]
    fn test_preprocess_gcd_normalization() {
        let e = LinearExpr {
            constant: 4,
            terms: vec![(name("x"), 2)],
        };
        let c = LinearConstraint::Le(e);
        let normalized = c.normalize();
        match normalized {
            LinearConstraint::Le(e) => {
                assert_eq!(e.constant, 2);
                assert_eq!(e.coeff_of(&name("x")), 1);
            }
            _ => panic!("expected Le"),
        }
    }
    #[test]
    fn test_omega_config_default() {
        let config = OmegaConfig::default();
        assert!(config.use_preprocessing);
        assert!(config.nat_mode);
        assert_eq!(config.max_steps, 1000);
    }
    #[test]
    fn test_omega_solver_new() {
        let solver = OmegaSolver::new();
        assert_eq!(solver.steps, 0);
    }
    #[test]
    fn test_omega_proof_contradiction() {
        let proof = OmegaProof::contradiction(5);
        assert_eq!(proof.steps.len(), 1);
        assert!(matches!(
            proof.steps[0],
            OmegaStep::Contradiction { value: 5 }
        ));
    }
    #[test]
    fn test_linear_term_new() {
        let term = LinearTerm::new(name("x"), 3);
        assert_eq!(term.coeff, 3);
        assert_eq!(term.var, name("x"));
    }
    #[test]
    fn test_is_satisfiable_trivial() {
        assert!(is_satisfiable(&[]));
        assert!(!is_unsatisfiable(&[]));
    }
    #[test]
    fn test_is_unsatisfiable_trivial() {
        let cs = vec![LinearConstraint::Le(cst(1))];
        assert!(is_unsatisfiable(&cs));
        assert!(!is_satisfiable(&cs));
    }
    #[test]
    fn test_omega_result_methods() {
        let sat = OmegaResult::Satisfiable;
        assert!(sat.is_sat());
        assert!(!sat.is_unsat());
        let unsat = OmegaResult::Unsatisfiable(OmegaProof::empty());
        assert!(unsat.is_unsat());
        assert!(!unsat.is_sat());
    }
    #[test]
    fn test_solve_omega_with_config_no_nat_mode() {
        let config = OmegaConfig {
            nat_mode: false,
            ..OmegaConfig::default()
        };
        let constraints = vec![LinearConstraint::Le(cst(5))];
        let result = solve_omega_with_config(&constraints, config);
        assert!(result.is_unsat());
    }
    #[test]
    fn test_constraint_display() {
        let c = LinearConstraint::Le(cst(3));
        let s = format!("{c}");
        assert!(s.contains('3'));
        assert!(s.contains('0'));
    }
    #[test]
    fn test_fm_combine_basic() {
        let lb = LinearConstraint::Le(LinearExpr::scaled_var(name("x"), -1));
        let ub = LinearConstraint::Le(var("x").sub(&cst(5)));
        let result = fm_combine(&name("x"), &lb, &ub);
        assert!(result.is_some());
    }
}
#[cfg(test)]
mod tacticomega_analysis_tests {
    use super::*;
    use crate::tactic::omega::*;
    #[test]
    fn test_tacticomega_result_ok() {
        let r = TacticOmegaResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticomega_result_err() {
        let r = TacticOmegaResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticomega_result_partial() {
        let r = TacticOmegaResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_tacticomega_result_skipped() {
        let r = TacticOmegaResult::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_tacticomega_analysis_pass_run() {
        let mut p = TacticOmegaAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_tacticomega_analysis_pass_empty_input() {
        let mut p = TacticOmegaAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_tacticomega_analysis_pass_success_rate() {
        let mut p = TacticOmegaAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_tacticomega_analysis_pass_disable() {
        let mut p = TacticOmegaAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_tacticomega_pipeline_basic() {
        let mut pipeline = TacticOmegaPipeline::new("main_pipeline");
        pipeline.add_pass(TacticOmegaAnalysisPass::new("pass1"));
        pipeline.add_pass(TacticOmegaAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_tacticomega_pipeline_disabled_pass() {
        let mut pipeline = TacticOmegaPipeline::new("partial");
        let mut p = TacticOmegaAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(TacticOmegaAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_tacticomega_diff_basic() {
        let mut d = TacticOmegaDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_tacticomega_diff_summary() {
        let mut d = TacticOmegaDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }
    #[test]
    fn test_tacticomega_config_set_get() {
        let mut cfg = TacticOmegaConfig::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_tacticomega_config_read_only() {
        let mut cfg = TacticOmegaConfig::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_tacticomega_config_remove() {
        let mut cfg = TacticOmegaConfig::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_tacticomega_diagnostics_basic() {
        let mut diag = TacticOmegaDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_tacticomega_diagnostics_max_errors() {
        let mut diag = TacticOmegaDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_tacticomega_diagnostics_clear() {
        let mut diag = TacticOmegaDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_tacticomega_config_value_types() {
        let b = TacticOmegaConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = TacticOmegaConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = TacticOmegaConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = TacticOmegaConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = TacticOmegaConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
#[cfg(test)]
mod omega_ext_tests_4200 {
    use super::*;
    use crate::tactic::omega::*;
    #[test]
    fn test_omega_ext_result_ok_4200() {
        let r = OmegaExtResult4200::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_omega_ext_result_err_4200() {
        let r = OmegaExtResult4200::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_omega_ext_result_partial_4200() {
        let r = OmegaExtResult4200::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_omega_ext_result_skipped_4200() {
        let r = OmegaExtResult4200::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_omega_ext_pass_run_4200() {
        let mut p = OmegaExtPass4200::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_omega_ext_pass_empty_4200() {
        let mut p = OmegaExtPass4200::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_omega_ext_pass_rate_4200() {
        let mut p = OmegaExtPass4200::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_omega_ext_pass_disable_4200() {
        let mut p = OmegaExtPass4200::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_omega_ext_pipeline_basic_4200() {
        let mut pipeline = OmegaExtPipeline4200::new("main_pipeline");
        pipeline.add_pass(OmegaExtPass4200::new("pass1"));
        pipeline.add_pass(OmegaExtPass4200::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_omega_ext_pipeline_disabled_4200() {
        let mut pipeline = OmegaExtPipeline4200::new("partial");
        let mut p = OmegaExtPass4200::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(OmegaExtPass4200::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_omega_ext_diff_basic_4200() {
        let mut d = OmegaExtDiff4200::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_omega_ext_config_set_get_4200() {
        let mut cfg = OmegaExtConfig4200::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_omega_ext_config_read_only_4200() {
        let mut cfg = OmegaExtConfig4200::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_omega_ext_config_remove_4200() {
        let mut cfg = OmegaExtConfig4200::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_omega_ext_diagnostics_basic_4200() {
        let mut diag = OmegaExtDiag4200::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_omega_ext_diagnostics_max_errors_4200() {
        let mut diag = OmegaExtDiag4200::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_omega_ext_diagnostics_clear_4200() {
        let mut diag = OmegaExtDiag4200::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_omega_ext_config_value_types_4200() {
        let b = OmegaExtConfigVal4200::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = OmegaExtConfigVal4200::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = OmegaExtConfigVal4200::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = OmegaExtConfigVal4200::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = OmegaExtConfigVal4200::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
