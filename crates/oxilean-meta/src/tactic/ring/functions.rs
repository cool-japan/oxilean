//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    HornerForm, Monomial, Polynomial, RingNormalizer, StrToken, TacticRingAnalysisPass,
    TacticRingConfig, TacticRingConfigValue, TacticRingDiagnostics, TacticRingDiff,
    TacticRingPipeline, TacticRingResult,
};
use crate::basic::MetaContext;
use crate::tactic::state::{TacticError, TacticResult, TacticState};
use oxilean_kernel::{Expr, FVarId, Literal, Name};

/// Tokenize a ring expression string.
pub(super) fn tokenize_ring_expr(s: &str) -> Vec<StrToken> {
    let chars: Vec<char> = s.chars().collect();
    let mut tokens = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            ' ' | '\t' | '\n' => {
                i += 1;
            }
            '+' => {
                tokens.push(StrToken::Plus);
                i += 1;
            }
            '-' => {
                tokens.push(StrToken::Minus);
                i += 1;
            }
            '*' => {
                tokens.push(StrToken::Star);
                i += 1;
            }
            '^' => {
                tokens.push(StrToken::Caret);
                i += 1;
            }
            '(' => {
                tokens.push(StrToken::LParen);
                i += 1;
            }
            ')' => {
                tokens.push(StrToken::RParen);
                i += 1;
            }
            c if c.is_ascii_digit() => {
                let mut num_str = String::new();
                while i < chars.len() && chars[i].is_ascii_digit() {
                    num_str.push(chars[i]);
                    i += 1;
                }
                if let Ok(n) = num_str.parse::<i64>() {
                    tokens.push(StrToken::Num(n));
                }
            }
            c if c.is_alphabetic() || c == '_' => {
                let mut ident = String::new();
                while i < chars.len()
                    && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '\'')
                {
                    ident.push(chars[i]);
                    i += 1;
                }
                tokens.push(StrToken::Ident(ident));
            }
            _ => {
                i += 1;
            }
        }
    }
    tokens
}
/// Compute the greatest common divisor using Euclidean algorithm.
pub(super) fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
/// Parse an Expr into a Polynomial if possible.
pub fn expr_to_polynomial(expr: &Expr, ctx: &MetaContext) -> TacticResult<Polynomial> {
    let expr = ctx.instantiate_mvars(expr);
    expr_to_polynomial_inner(&expr)
}
/// Inner polynomial conversion (no ctx needed, pure).
pub(super) fn expr_to_polynomial_inner(expr: &Expr) -> TacticResult<Polynomial> {
    match expr {
        Expr::Lit(Literal::Nat(n)) => {
            if *n > i64::MAX as u64 {
                Err(TacticError::Failed("literal too large for ring".into()))
            } else {
                Ok(Polynomial::constant(*n as i64, 1))
            }
        }
        Expr::Const(name, _levels) => match name.to_string().as_str() {
            "zero" | "Nat.zero" | "Int.zero" => Ok(Polynomial::zero()),
            "one" | "Nat.one" | "Int.one" => Ok(Polynomial::one()),
            _ => Ok(Polynomial::var(name.clone())),
        },
        Expr::FVar(FVarId(id)) => {
            let var_name = Name::str(format!("fvar_{}", id));
            Ok(Polynomial::var(var_name))
        }
        Expr::App(func, arg) => eval_poly_app(func, arg),
        _ => Err(TacticError::Failed(
            "expression type not supported in ring".into(),
        )),
    }
}
/// Get the name string from a Const expression.
pub(super) fn poly_get_const_name(expr: &Expr) -> Option<String> {
    if let Expr::Const(name, _) = expr {
        Some(name.to_string())
    } else {
        None
    }
}
/// Evaluate a function application as a polynomial.
pub(super) fn eval_poly_app(func: &Expr, arg: &Expr) -> TacticResult<Polynomial> {
    if let Expr::App(op_expr, lhs_expr) = func {
        if let Some(op_name) = poly_get_const_name(op_expr) {
            let lhs = expr_to_polynomial_inner(lhs_expr)?;
            let rhs = expr_to_polynomial_inner(arg)?;
            return match op_name.as_str() {
                "Nat.add" | "Int.add" | "HAdd.hAdd" | "Add.add" => Ok(lhs.add(&rhs)),
                "Nat.sub" | "Int.sub" | "HSub.hSub" | "Sub.sub" => Ok(lhs.sub(&rhs)),
                "Nat.mul" | "Int.mul" | "HMul.hMul" | "Mul.mul" => Ok(lhs.multiply(&rhs)),
                "HPow.hPow" | "Pow.pow" | "Nat.pow" | "Int.pow" => match arg {
                    Expr::Lit(Literal::Nat(n)) => {
                        if *n > 20 {
                            Err(TacticError::Failed("exponent too large for ring".into()))
                        } else {
                            Ok(lhs.power(*n as u32))
                        }
                    }
                    _ => Err(TacticError::Failed(
                        "ring: exponent must be a literal Nat".into(),
                    )),
                },
                _ => Err(TacticError::Failed(format!(
                    "ring: unknown binary op: {}",
                    op_name
                ))),
            };
        }
    }
    if let Some(op_name) = poly_get_const_name(func) {
        match op_name.as_str() {
            "Nat.succ" => {
                let p = expr_to_polynomial_inner(arg)?;
                let one = Polynomial::one();
                return Ok(p.add(&one));
            }
            "Neg.neg" | "Int.neg" => {
                let p = expr_to_polynomial_inner(arg)?;
                return Ok(p.negate());
            }
            "Int.ofNat" => {
                return expr_to_polynomial_inner(arg);
            }
            _ => {}
        }
    }
    if let Expr::App(func2, lhs_expr) = func {
        if let Expr::App(op_expr, _ty) = func2.as_ref() {
            if let Some(op_name) = poly_get_const_name(op_expr) {
                let lhs = expr_to_polynomial_inner(lhs_expr)?;
                let rhs = expr_to_polynomial_inner(arg)?;
                return match op_name.as_str() {
                    "Nat.add" | "Int.add" | "HAdd.hAdd" | "Add.add" => Ok(lhs.add(&rhs)),
                    "Nat.sub" | "Int.sub" | "HSub.hSub" | "Sub.sub" => Ok(lhs.sub(&rhs)),
                    "Nat.mul" | "Int.mul" | "HMul.hMul" | "Mul.mul" => Ok(lhs.multiply(&rhs)),
                    _ => Err(TacticError::Failed(format!(
                        "ring: unknown typed op: {}",
                        op_name
                    ))),
                };
            }
        }
    }
    Err(TacticError::Failed(
        "ring: cannot convert expression to polynomial".into(),
    ))
}
/// Simplify polynomial using zero/one laws.
pub fn simplify_zero_one(poly: &Polynomial) -> Polynomial {
    if poly.is_zero() {
        Polynomial::zero()
    } else if poly.is_constant() {
        poly.clone()
    } else {
        let mut result = poly.clone();
        result.terms.retain(|(_mono, (num, _))| *num != 0);
        result
    }
}
/// Check if two polynomials are equal after normalization.
pub fn polynomials_equal(p1: &Polynomial, p2: &Polynomial) -> bool {
    if p1.terms.len() != p2.terms.len() {
        return false;
    }
    for (mono1, (n1, d1)) in &p1.terms {
        if let Some((n2, d2)) = p2.terms.iter().find(|(m, _)| m == mono1).map(|(_, c)| c) {
            if n1 * (*d2 as i64) != n2 * (*d1 as i64) {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}
/// Canonical key type for monomial grouping: sorted (variable, exponent) pairs with rational coeff.
type MonomialEntry = (Vec<(Name, u32)>, (i64, u32));
/// Simplify a polynomial using add/mul/pow algebraic laws:
///
///   - Remove zero-exponent factors from monomials (x^0 = 1, so drop them).
///   - Sort each monomial's variables canonically (for grouping).
///   - Combine like monomials by adding their rational coefficients.
///   - Remove terms with zero combined coefficient.
///   - Normalize each surviving coefficient by dividing by gcd(|num|, den).
pub fn simplify_add_mul_pow(poly: &Polynomial) -> Polynomial {
    let mut combined: Vec<MonomialEntry> = Vec::new();
    for (mono, (num, den)) in &poly.terms {
        if *num == 0 {
            continue;
        }
        let mut canon: Vec<(Name, u32)> = mono
            .exponents
            .iter()
            .filter(|(_, exp)| *exp > 0)
            .cloned()
            .collect();
        canon.sort_by_key(|(a, _)| a.to_string());
        if let Some(entry) = combined.iter_mut().find(|(key, _)| key == &canon) {
            let (en, ed) = entry.1;
            let (cn, cd) = (*num, *den);
            let new_num = en * cd as i64 + cn * ed as i64;
            let new_den = ed * cd;
            if new_den == 0 {
                entry.1 = (0, 1);
            } else {
                let g = gcd(new_num.unsigned_abs(), new_den as u64) as u32;
                let g = g.max(1);
                entry.1 = (new_num / g as i64, new_den / g);
            }
        } else {
            let g = gcd((*num).unsigned_abs(), *den as u64) as u32;
            let g = g.max(1);
            combined.push((canon, (*num / g as i64, *den / g)));
        }
    }
    let terms: Vec<(Monomial, (i64, u32))> = combined
        .into_iter()
        .filter(|(_, (n, _))| *n != 0)
        .map(|(canon, coeff)| (Monomial { exponents: canon }, coeff))
        .collect();
    Polynomial { terms }
}
/// The ring tactic: prove equality by polynomial normalization.
pub fn tac_ring(state: &mut TacticState, ctx: &mut MetaContext) -> TacticResult<()> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let target = ctx.instantiate_mvars(&target);
    let (lhs, rhs) = extract_eq_sides(&target)
        .ok_or_else(|| TacticError::GoalMismatch("ring requires an equality goal".into()))?;
    let poly_lhs = expr_to_polynomial(&lhs, ctx)?;
    let poly_rhs = expr_to_polynomial(&rhs, ctx)?;
    let norm_lhs = simplify_add_mul_pow(&poly_lhs);
    let norm_rhs = simplify_add_mul_pow(&poly_rhs);
    if polynomials_equal(&norm_lhs, &norm_rhs) {
        state.close_goal(Expr::Const(Name::str("rfl"), vec![]), ctx)?;
        return Ok(());
    }
    if let (Some(lhs_str), Some(rhs_str)) = (expr_to_ring_str(&lhs), expr_to_ring_str(&rhs)) {
        if RingNormalizer::check_equality(&lhs_str, &rhs_str) {
            state.close_goal(Expr::Const(Name::str("rfl"), vec![]), ctx)?;
            return Ok(());
        }
    }
    Err(TacticError::GoalMismatch(
        "ring: sides are not equal as polynomials".into(),
    ))
}
/// Convert a kernel `Expr` to a simple ring expression string for [`RingNormalizer`].
///
/// Returns `None` if the expression cannot be represented as a simple ring string.
pub(super) fn expr_to_ring_str(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Lit(Literal::Nat(n)) => Some(n.to_string()),
        Expr::Const(name, _) => Some(name.to_string()),
        Expr::FVar(FVarId(id)) => Some(format!("fvar_{}", id)),
        Expr::App(func, arg) => {
            if let Expr::App(op_expr, lhs_expr) = func.as_ref() {
                if let Some(op) = get_ring_op(op_expr) {
                    let l = expr_to_ring_str(lhs_expr)?;
                    let r = expr_to_ring_str(arg)?;
                    return Some(format!("({} {} {})", l, op, r));
                }
                if let Expr::App(op_expr2, _ty) = op_expr.as_ref() {
                    if let Some(op) = get_ring_op(op_expr2) {
                        let l = expr_to_ring_str(lhs_expr)?;
                        let r = expr_to_ring_str(arg)?;
                        return Some(format!("({} {} {})", l, op, r));
                    }
                }
            }
            if let Some(op_name) = const_name(func) {
                match op_name.as_str() {
                    "Nat.succ" => {
                        let inner = expr_to_ring_str(arg)?;
                        return Some(format!("({} + 1)", inner));
                    }
                    "Neg.neg" | "Int.neg" => {
                        let inner = expr_to_ring_str(arg)?;
                        return Some(format!("(0 - {})", inner));
                    }
                    "Int.ofNat" => return expr_to_ring_str(arg),
                    _ => {}
                }
            }
            None
        }
        _ => None,
    }
}
/// Return the ring operator symbol (+, -, *, ^) for a known binary op Const, or None.
pub(super) fn get_ring_op(expr: &Expr) -> Option<&'static str> {
    let name = const_name(expr)?;
    match name.as_str() {
        "Nat.add" | "Int.add" | "HAdd.hAdd" | "Add.add" => Some("+"),
        "Nat.sub" | "Int.sub" | "HSub.hSub" | "Sub.sub" => Some("-"),
        "Nat.mul" | "Int.mul" | "HMul.hMul" | "Mul.mul" => Some("*"),
        "HPow.hPow" | "Pow.pow" | "Nat.pow" | "Int.pow" => Some("^"),
        _ => None,
    }
}
/// Get the name string from a Const expression (helper).
pub(super) fn const_name(expr: &Expr) -> Option<String> {
    if let Expr::Const(name, _) = expr {
        Some(name.to_string())
    } else {
        None
    }
}
/// Extract the two sides of an equality goal.
///
/// Handles:
///   - `App(App(App(Eq, _ty), lhs), rhs)` — fully applied typed equality
///   - `App(App(Eq, lhs), rhs)` — untyped equality
pub(super) fn extract_eq_sides(expr: &Expr) -> Option<(Expr, Expr)> {
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
/// Check if an expression is the equality constant.
pub(super) fn is_eq_const(expr: &Expr) -> bool {
    matches!(
        expr, Expr::Const(name, _) if { let s = name.to_string(); s == "Eq" || s == "eq"
        }
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tactic::ring::*;
    #[test]
    fn test_monomial_constant() {
        let m = Monomial::constant();
        assert!(m.is_constant());
        assert_eq!(m.total_degree(), 0);
    }
    #[test]
    fn test_monomial_var() {
        let x = Monomial::var(Name::str("x"));
        assert!(!x.is_constant());
        assert_eq!(x.total_degree(), 1);
    }
    #[test]
    fn test_monomial_var_exp() {
        let x2 = Monomial::var_exp(Name::str("x"), 2);
        assert_eq!(x2.total_degree(), 2);
    }
    #[test]
    fn test_monomial_multiply() {
        let x = Monomial::var(Name::str("x"));
        let y = Monomial::var(Name::str("y"));
        let xy = x.multiply(&y);
        assert_eq!(xy.total_degree(), 2);
    }
    #[test]
    fn test_polynomial_constant() {
        let p = Polynomial::constant(5, 1);
        assert!(p.is_constant());
        assert!(!p.is_zero());
    }
    #[test]
    fn test_polynomial_zero() {
        let p = Polynomial::zero();
        assert!(p.is_zero());
        assert!(p.is_constant());
    }
    #[test]
    fn test_polynomial_one() {
        let p = Polynomial::one();
        assert!(!p.is_zero());
        assert!(p.is_constant());
    }
    #[test]
    fn test_polynomial_var() {
        let p = Polynomial::var(Name::str("x"));
        assert!(!p.is_constant());
        assert!(!p.is_zero());
    }
    #[test]
    fn test_polynomial_add_constants() {
        let p1 = Polynomial::constant(3, 1);
        let p2 = Polynomial::constant(2, 1);
        let p3 = p1.add(&p2);
        let p5 = Polynomial::constant(5, 1);
        assert!(polynomials_equal(&p3, &p5));
    }
    #[test]
    fn test_polynomial_add_zero() {
        let p = Polynomial::constant(7, 1);
        let z = Polynomial::zero();
        let result = p.add(&z);
        assert!(polynomials_equal(&result, &p));
    }
    #[test]
    fn test_polynomial_sub_constants() {
        let p1 = Polynomial::constant(5, 1);
        let p2 = Polynomial::constant(3, 1);
        let p3 = p1.sub(&p2);
        let p2_result = Polynomial::constant(2, 1);
        assert!(polynomials_equal(&p3, &p2_result));
    }
    #[test]
    fn test_polynomial_negate() {
        let p = Polynomial::constant(5, 1);
        let neg_p = p.negate();
        let neg5 = Polynomial::constant(-5, 1);
        assert!(polynomials_equal(&neg_p, &neg5));
    }
    #[test]
    fn test_polynomial_multiply_constants() {
        let p1 = Polynomial::constant(3, 1);
        let p2 = Polynomial::constant(4, 1);
        let p3 = p1.multiply(&p2);
        let p12 = Polynomial::constant(12, 1);
        assert!(polynomials_equal(&p3, &p12));
    }
    #[test]
    fn test_polynomial_multiply_by_zero() {
        let p = Polynomial::constant(5, 1);
        let z = Polynomial::zero();
        let result = p.multiply(&z);
        assert!(result.is_zero());
    }
    #[test]
    fn test_polynomial_multiply_by_one() {
        let p = Polynomial::constant(7, 1);
        let one = Polynomial::one();
        let result = p.multiply(&one);
        assert!(polynomials_equal(&result, &p));
    }
    #[test]
    fn test_polynomial_power_zero() {
        let p = Polynomial::var(Name::str("x"));
        let p0 = p.power(0);
        assert!(polynomials_equal(&p0, &Polynomial::one()));
    }
    #[test]
    fn test_polynomial_power_one() {
        let p = Polynomial::var(Name::str("x"));
        let p1 = p.power(1);
        assert!(polynomials_equal(&p1, &p));
    }
    #[test]
    fn test_polynomial_power_two() {
        let p = Polynomial::constant(2, 1);
        let p2 = p.power(2);
        let four = Polynomial::constant(4, 1);
        assert!(polynomials_equal(&p2, &four));
    }
    #[test]
    fn test_polynomial_variables() {
        let x = Polynomial::var(Name::str("x"));
        let vars = x.variables();
        assert_eq!(vars.len(), 1);
        assert_eq!(vars[0], Name::str("x"));
    }
    #[test]
    fn test_polynomial_simplify_zero_one() {
        let p = Polynomial::constant(5, 1);
        let simplified = simplify_zero_one(&p);
        assert!(polynomials_equal(&simplified, &p));
    }
    #[test]
    fn test_polynomial_simplify_add_mul_pow() {
        let p = Polynomial::constant(3, 1);
        let simplified = simplify_add_mul_pow(&p);
        assert!(polynomials_equal(&simplified, &p));
    }
    #[test]
    fn test_polynomial_fraction_add() {
        let p1 = Polynomial::constant(1, 2);
        let p2 = Polynomial::constant(1, 3);
        let _result = p1.add(&p2);
    }
    #[test]
    fn test_horner_form_constant() {
        let h = HornerForm::Constant(5, 1);
        assert!(h.equals(&HornerForm::Constant(5, 1)));
    }
    #[test]
    fn test_gcd_basic() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(17, 19), 1);
        assert_eq!(gcd(100, 50), 50);
    }
    #[test]
    fn test_gcd_one() {
        assert_eq!(gcd(1, 5), 1);
        assert_eq!(gcd(5, 1), 1);
    }
    #[test]
    fn test_gcd_zero() {
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(5, 0), 5);
    }
    #[test]
    fn test_polynomial_cancel_coefficients() {
        let p1 = Polynomial::constant(2, 4);
        assert!(!p1.is_zero());
    }
    #[test]
    fn test_polynomial_is_constant_single_term() {
        let p = Polynomial::constant(42, 1);
        assert!(p.is_constant());
    }
    #[test]
    fn test_polynomial_is_constant_with_var() {
        let p = Polynomial::var(Name::str("x"));
        assert!(!p.is_constant());
    }
    #[test]
    fn test_monomial_multiply_same_var() {
        let x = Monomial::var(Name::str("x"));
        let result = x.multiply(&x);
        assert_eq!(result.total_degree(), 2);
    }
    #[test]
    fn test_polynomial_add_fractions_same_denom() {
        let p1 = Polynomial::constant(1, 5);
        let p2 = Polynomial::constant(2, 5);
        let p3 = p1.add(&p2);
        let p3_expected = Polynomial::constant(3, 5);
        assert!(polynomials_equal(&p3, &p3_expected));
    }
    #[test]
    fn test_polynomial_add_fractions_different_denom() {
        let p1 = Polynomial::constant(1, 2);
        let p2 = Polynomial::constant(1, 3);
        let _result = p1.add(&p2);
    }
    #[test]
    fn test_polynomial_multiply_fractions() {
        let p1 = Polynomial::constant(1, 2);
        let p2 = Polynomial::constant(1, 3);
        let p3 = p1.multiply(&p2);
        let expected = Polynomial::constant(1, 6);
        assert!(polynomials_equal(&p3, &expected));
    }
    #[test]
    fn test_polynomial_negate_fraction() {
        let p = Polynomial::constant(3, 5);
        let neg_p = p.negate();
        let neg5_thirds = Polynomial::constant(-3, 5);
        assert!(polynomials_equal(&neg_p, &neg5_thirds));
    }
    #[test]
    fn test_polynomial_power_three() {
        let p = Polynomial::constant(2, 1);
        let p3 = p.power(3);
        let eight = Polynomial::constant(8, 1);
        assert!(polynomials_equal(&p3, &eight));
    }
    #[test]
    fn test_polynomial_power_large() {
        let p = Polynomial::constant(2, 1);
        let p10 = p.power(10);
        let thousand24 = Polynomial::constant(1024, 1);
        assert!(polynomials_equal(&p10, &thousand24));
    }
    #[test]
    fn test_monomial_variables_multiple() {
        let mut m = Monomial::constant();
        m.exponents.push((Name::str("x"), 1));
        m.exponents.push((Name::str("y"), 2));
        m.exponents.push((Name::str("z"), 1));
        let vars = m.variables();
        assert_eq!(vars.len(), 3);
    }
    #[test]
    fn test_polynomial_variables_deduplicated() {
        let mut p = Polynomial::zero();
        let mut m1 = Monomial::constant();
        m1.exponents.push((Name::str("x"), 1));
        let mut m2 = Monomial::constant();
        m2.exponents.push((Name::str("x"), 2));
        p.terms.push((m1, (1, 1)));
        p.terms.push((m2, (1, 1)));
        let vars = p.variables();
        assert_eq!(vars.len(), 1);
    }
    #[test]
    fn test_polynomial_sub_self() {
        let p = Polynomial::constant(5, 1);
        let result = p.sub(&p);
        assert!(result.is_zero());
    }
    #[test]
    fn test_polynomial_triple_multiply() {
        let p1 = Polynomial::constant(2, 1);
        let p2 = Polynomial::constant(3, 1);
        let p3 = Polynomial::constant(4, 1);
        let result = p1.multiply(&p2).multiply(&p3);
        let expected = Polynomial::constant(24, 1);
        assert!(polynomials_equal(&result, &expected));
    }
    #[test]
    fn test_simplify_zero_one_identity() {
        let z = Polynomial::zero();
        let simplified = simplify_zero_one(&z);
        assert!(simplified.is_zero());
    }
    #[test]
    fn test_horner_form_add_constants() {
        let h1 = HornerForm::Constant(3, 1);
        let h2 = HornerForm::Constant(2, 1);
        let result = h1.add_horner(&h2);
        assert!(result.equals(&HornerForm::Constant(5, 1)));
    }
    #[test]
    fn test_polynomial_fraction_reduce() {
        let p = Polynomial::constant(10, 20);
        assert!(!p.is_zero());
    }
    #[test]
    fn test_monomial_multiply_multiple_vars() {
        let mut m1 = Monomial::constant();
        m1.exponents.push((Name::str("x"), 2));
        m1.exponents.push((Name::str("y"), 1));
        let mut m2 = Monomial::constant();
        m2.exponents.push((Name::str("y"), 1));
        m2.exponents.push((Name::str("z"), 3));
        let result = m1.multiply(&m2);
        assert_eq!(result.total_degree(), 7);
    }
    #[test]
    fn test_polynomial_zero_handling() {
        let p1 = Polynomial::constant(0, 1);
        let p2 = Polynomial::constant(5, 1);
        let result = p1.add(&p2);
        assert!(polynomials_equal(&result, &p2));
    }
}
/// Degree of a polynomial in a single variable.
pub fn degree_in_var(poly: &Polynomial, var: &Name) -> u32 {
    poly.terms
        .iter()
        .filter_map(|(m, _)| m.exponents.iter().find(|(n, _)| n == var).map(|(_, e)| *e))
        .max()
        .unwrap_or(0)
}
/// Total degree of a polynomial (maximum total degree over all monomials).
pub fn total_degree(poly: &Polynomial) -> u32 {
    poly.terms
        .iter()
        .map(|(m, _)| m.total_degree())
        .max()
        .unwrap_or(0)
}
/// Evaluate a polynomial at given variable assignments.
///
/// Returns `None` if any variable is not assigned.
pub fn eval_polynomial(
    poly: &Polynomial,
    env: &std::collections::HashMap<String, i64>,
) -> Option<i64> {
    let mut result: i64 = 0;
    for (mono, (num, den)) in &poly.terms {
        let mut mono_val: i64 = 1;
        for (var, exp) in &mono.exponents {
            let v = *env.get(&var.to_string())?;
            mono_val *= i64::pow(v, *exp);
        }
        result += mono_val * num / (*den as i64);
    }
    Some(result)
}
/// Check if a polynomial has no negative coefficients.
pub fn is_nonnegative_coefficients(poly: &Polynomial) -> bool {
    poly.terms.iter().all(|(_, (num, _))| *num >= 0)
}
/// Extract the constant term of a polynomial.
pub fn constant_term(poly: &Polynomial) -> (i64, u32) {
    poly.terms
        .iter()
        .find(|(m, _)| m.is_constant())
        .map(|(_, c)| *c)
        .unwrap_or((0, 1))
}
/// Remove the constant term from a polynomial, returning it and the rest.
pub fn split_constant(poly: &Polynomial) -> ((i64, u32), Polynomial) {
    let ct = constant_term(poly);
    let mut rest = poly.clone();
    rest.terms.retain(|(m, _)| !m.is_constant());
    (ct, rest)
}
/// Check if two polynomials are equal after normalization.
///
/// This is a stronger check than `polynomials_equal` because it first sorts
/// the term lists so the order of terms does not matter.
pub fn polynomials_equal_sorted(p1: &Polynomial, p2: &Polynomial) -> bool {
    let key = |terms: &[(Monomial, (i64, u32))]| -> Vec<String> {
        let mut keys: Vec<String> = terms
            .iter()
            .map(|(m, (n, d))| format!("{:?}:{}/{}", m.exponents, n, d))
            .collect();
        keys.sort();
        keys
    };
    key(&p1.terms) == key(&p2.terms)
}
/// Monomial ordering (graded lexicographic).
pub fn monomial_lt(a: &Monomial, b: &Monomial) -> bool {
    let da = a.total_degree();
    let db = b.total_degree();
    if da != db {
        return da < db;
    }
    let va = a.variables();
    let vb = b.variables();
    let va_str: Vec<String> = va.iter().map(|n| n.to_string()).collect();
    let vb_str: Vec<String> = vb.iter().map(|n| n.to_string()).collect();
    va_str < vb_str
}
/// Sort the terms of a polynomial in graded lex order.
pub fn sort_polynomial(poly: &mut Polynomial) {
    poly.terms.sort_by(|(ma, _), (mb, _)| {
        if monomial_lt(ma, mb) {
            std::cmp::Ordering::Less
        } else if monomial_lt(mb, ma) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });
}
/// Scale a polynomial by a rational factor.
pub fn scale_polynomial(poly: &Polynomial, num: i64, den: u32) -> Polynomial {
    let scaler = Polynomial::constant(num, den);
    poly.multiply(&scaler)
}
/// Compute the sum of a list of polynomials.
pub fn sum_polynomials(polys: &[Polynomial]) -> Polynomial {
    polys.iter().fold(Polynomial::zero(), |acc, p| acc.add(p))
}
/// Compute the product of a list of polynomials.
pub fn product_polynomials(polys: &[Polynomial]) -> Polynomial {
    polys
        .iter()
        .fold(Polynomial::one(), |acc, p| acc.multiply(p))
}
/// Differentiate a polynomial with respect to a variable (formal derivative).
pub fn differentiate(poly: &Polynomial, var: &Name) -> Polynomial {
    let mut result = Polynomial::zero();
    for (mono, (num, den)) in &poly.terms {
        if let Some((_, exp)) = mono.exponents.iter().find(|(n, _)| n == var) {
            if *exp > 0 {
                let new_num = (*exp as i64) * num;
                let g = gcd(new_num.unsigned_abs(), *den as u64) as u32;
                let norm_num = new_num / g as i64;
                let norm_den = *den / g;
                let mut new_mono = mono.clone();
                for entry in new_mono.exponents.iter_mut() {
                    if &entry.0 == var {
                        entry.1 -= 1;
                    }
                }
                new_mono.exponents.retain(|(_, e)| *e > 0);
                result.terms.push((new_mono, (norm_num, norm_den)));
            }
        }
    }
    result
}
#[cfg(test)]
mod ring_extended_tests {
    use super::*;
    use crate::tactic::ring::*;
    use oxilean_kernel::Environment;
    #[test]
    fn test_degree_in_var() {
        let mut p = Polynomial::zero();
        let mut m = Monomial::constant();
        m.exponents.push((Name::str("x"), 3));
        p.terms.push((m, (1, 1)));
        assert_eq!(degree_in_var(&p, &Name::str("x")), 3);
    }
    #[test]
    fn test_total_degree_zero() {
        assert_eq!(total_degree(&Polynomial::zero()), 0);
    }
    #[test]
    fn test_total_degree_constant() {
        let p = Polynomial::constant(5, 1);
        assert_eq!(total_degree(&p), 0);
    }
    #[test]
    fn test_eval_polynomial_constant() {
        let p = Polynomial::constant(7, 1);
        let env = std::collections::HashMap::new();
        assert_eq!(eval_polynomial(&p, &env), Some(7));
    }
    #[test]
    fn test_is_nonnegative_coefficients() {
        assert!(is_nonnegative_coefficients(&Polynomial::constant(3, 1)));
        assert!(!is_nonnegative_coefficients(&Polynomial::constant(-1, 1)));
    }
    #[test]
    fn test_constant_term() {
        let p = Polynomial::constant(4, 1);
        assert_eq!(constant_term(&p), (4, 1));
    }
    #[test]
    fn test_constant_term_zero_poly() {
        let p = Polynomial::zero();
        assert_eq!(constant_term(&p), (0, 1));
    }
    #[test]
    fn test_split_constant() {
        let p = Polynomial::constant(5, 1);
        let (ct, rest) = split_constant(&p);
        assert_eq!(ct, (5, 1));
        assert!(rest.is_zero());
    }
    #[test]
    fn test_scale_polynomial() {
        let p = Polynomial::constant(3, 1);
        let scaled = scale_polynomial(&p, 2, 1);
        assert!(polynomials_equal(&scaled, &Polynomial::constant(6, 1)));
    }
    #[test]
    fn test_sum_polynomials() {
        let polys = vec![
            Polynomial::constant(1, 1),
            Polynomial::constant(2, 1),
            Polynomial::constant(3, 1),
        ];
        let sum = sum_polynomials(&polys);
        assert!(polynomials_equal(&sum, &Polynomial::constant(6, 1)));
    }
    #[test]
    fn test_product_polynomials() {
        let polys = vec![
            Polynomial::constant(2, 1),
            Polynomial::constant(3, 1),
            Polynomial::constant(4, 1),
        ];
        let prod = product_polynomials(&polys);
        assert!(polynomials_equal(&prod, &Polynomial::constant(24, 1)));
    }
    #[test]
    fn test_differentiate_constant_is_zero() {
        let p = Polynomial::constant(5, 1);
        let dp = differentiate(&p, &Name::str("x"));
        assert!(dp.is_zero());
    }
    #[test]
    fn test_differentiate_linear() {
        let p = Polynomial::var(Name::str("x"));
        let scaled = scale_polynomial(&p, 3, 1);
        let dp = differentiate(&scaled, &Name::str("x"));
        assert!(polynomials_equal(&dp, &Polynomial::constant(3, 1)));
    }
    #[test]
    fn test_polynomials_equal_sorted() {
        let p1 = Polynomial::constant(5, 1);
        let p2 = Polynomial::constant(5, 1);
        assert!(polynomials_equal_sorted(&p1, &p2));
    }
    #[test]
    fn test_sort_polynomial_does_not_crash() {
        let mut p = Polynomial::constant(3, 1);
        sort_polynomial(&mut p);
        assert!(!p.is_zero());
    }
    #[test]
    fn test_monomial_lt_by_degree() {
        let m1 = Monomial::var_exp(Name::str("x"), 1);
        let m2 = Monomial::var_exp(Name::str("x"), 2);
        assert!(monomial_lt(&m1, &m2));
        assert!(!monomial_lt(&m2, &m1));
    }
    #[test]
    fn test_expr_to_poly_nat_literal() {
        use oxilean_kernel::{Environment, Expr, Literal};
        let ctx = MetaContext::new(Environment::new());
        let expr = Expr::Lit(Literal::Nat(5));
        let poly = expr_to_polynomial(&expr, &ctx).expect("poly should be present");
        assert!(poly.is_constant());
        assert!(!poly.is_zero());
    }
    #[test]
    fn test_expr_to_poly_zero_lit() {
        let ctx = MetaContext::new(Environment::new());
        let expr = Expr::Lit(Literal::Nat(0));
        let poly = expr_to_polynomial(&expr, &ctx).expect("poly should be present");
        assert!(poly.is_zero());
    }
    #[test]
    fn test_expr_to_poly_named_zero() {
        use oxilean_kernel::{Environment, Expr, Name};
        let ctx = MetaContext::new(Environment::new());
        let expr = Expr::Const(Name::str("zero"), vec![]);
        let poly = expr_to_polynomial(&expr, &ctx).expect("poly should be present");
        assert!(poly.is_zero());
    }
    #[test]
    fn test_expr_to_poly_named_one() {
        let ctx = MetaContext::new(Environment::new());
        let expr = Expr::Const(Name::str("one"), vec![]);
        let poly = expr_to_polynomial(&expr, &ctx).expect("poly should be present");
        assert!(polynomials_equal(&poly, &Polynomial::one()));
    }
    #[test]
    fn test_expr_to_poly_add() {
        use oxilean_kernel::{Environment, Expr, Literal, Name};
        let ctx = MetaContext::new(Environment::new());
        let three = Expr::Lit(Literal::Nat(3));
        let four = Expr::Lit(Literal::Nat(4));
        let add = Expr::Const(Name::str("Nat.add"), vec![]);
        let expr = Expr::App(
            Box::new(Expr::App(Box::new(add), Box::new(three))),
            Box::new(four),
        );
        let poly = expr_to_polynomial(&expr, &ctx).expect("poly should be present");
        let expected = Polynomial::constant(7, 1);
        assert!(polynomials_equal(&poly, &expected));
    }
    #[test]
    fn test_expr_to_poly_mul() {
        let ctx = MetaContext::new(Environment::new());
        let three = Expr::Lit(Literal::Nat(3));
        let four = Expr::Lit(Literal::Nat(4));
        let mul = Expr::Const(Name::str("Nat.mul"), vec![]);
        let expr = Expr::App(
            Box::new(Expr::App(Box::new(mul), Box::new(three))),
            Box::new(four),
        );
        let poly = expr_to_polynomial(&expr, &ctx).expect("poly should be present");
        let expected = Polynomial::constant(12, 1);
        assert!(polynomials_equal(&poly, &expected));
    }
    #[test]
    fn test_expr_to_poly_variable() {
        let ctx = MetaContext::new(Environment::new());
        let expr = Expr::Const(Name::str("x"), vec![]);
        let poly = expr_to_polynomial(&expr, &ctx).expect("poly should be present");
        assert!(!poly.is_constant());
        let vars = poly.variables();
        assert!(vars.contains(&Name::str("x")));
    }
    #[test]
    fn test_expr_to_poly_succ() {
        let ctx = MetaContext::new(Environment::new());
        let four = Expr::Lit(Literal::Nat(4));
        let succ = Expr::Const(Name::str("Nat.succ"), vec![]);
        let expr = Expr::App(Box::new(succ), Box::new(four));
        let poly = expr_to_polynomial(&expr, &ctx).expect("poly should be present");
        let expected = Polynomial::constant(5, 1);
        assert!(polynomials_equal(&poly, &expected));
    }
    #[test]
    fn test_extract_eq_sides() {
        use oxilean_kernel::{Expr, Level, Name};
        let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let eq_goal = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Eq"), vec![Level::zero()])),
                    Box::new(nat_ty),
                )),
                Box::new(a.clone()),
            )),
            Box::new(b.clone()),
        );
        let (lhs, rhs) = extract_eq_sides(&eq_goal).expect("value should be present");
        assert_eq!(lhs, a);
        assert_eq!(rhs, b);
    }
    #[test]
    fn test_extract_eq_sides_not_eq() {
        use oxilean_kernel::{Expr, Name};
        let expr = Expr::Const(Name::str("Nat"), vec![]);
        assert!(extract_eq_sides(&expr).is_none());
    }
}
#[cfg(test)]
mod tacticring_analysis_tests {
    use super::*;
    use crate::tactic::ring::*;
    #[test]
    fn test_tacticring_result_ok() {
        let r = TacticRingResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticring_result_err() {
        let r = TacticRingResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticring_result_partial() {
        let r = TacticRingResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_tacticring_result_skipped() {
        let r = TacticRingResult::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_tacticring_analysis_pass_run() {
        let mut p = TacticRingAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_tacticring_analysis_pass_empty_input() {
        let mut p = TacticRingAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_tacticring_analysis_pass_success_rate() {
        let mut p = TacticRingAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_tacticring_analysis_pass_disable() {
        let mut p = TacticRingAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_tacticring_pipeline_basic() {
        let mut pipeline = TacticRingPipeline::new("main_pipeline");
        pipeline.add_pass(TacticRingAnalysisPass::new("pass1"));
        pipeline.add_pass(TacticRingAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_tacticring_pipeline_disabled_pass() {
        let mut pipeline = TacticRingPipeline::new("partial");
        let mut p = TacticRingAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(TacticRingAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_tacticring_diff_basic() {
        let mut d = TacticRingDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_tacticring_diff_summary() {
        let mut d = TacticRingDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }
    #[test]
    fn test_tacticring_config_set_get() {
        let mut cfg = TacticRingConfig::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_tacticring_config_read_only() {
        let mut cfg = TacticRingConfig::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_tacticring_config_remove() {
        let mut cfg = TacticRingConfig::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_tacticring_diagnostics_basic() {
        let mut diag = TacticRingDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_tacticring_diagnostics_max_errors() {
        let mut diag = TacticRingDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_tacticring_diagnostics_clear() {
        let mut diag = TacticRingDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_tacticring_config_value_types() {
        let b = TacticRingConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = TacticRingConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = TacticRingConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = TacticRingConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = TacticRingConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
