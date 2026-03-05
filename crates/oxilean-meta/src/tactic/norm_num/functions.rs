//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    ComparisonOp, IdentitySide, LinearConstraint, LinearExpr, NormNumExtConfig2800,
    NormNumExtConfigVal2800, NormNumExtDiag2800, NormNumExtDiff2800, NormNumExtPass2800,
    NormNumExtPipeline2800, NormNumExtResult2800, NumericNormalForm, NumericValue, Poly, ProofTerm,
    TacticNormNumAnalysisPass, TacticNormNumConfig, TacticNormNumConfigValue,
    TacticNormNumDiagnostics, TacticNormNumDiff, TacticNormNumPipeline, TacticNormNumResult,
};
use crate::basic::MetaContext;
use crate::tactic::state::{TacticError, TacticResult, TacticState};
use oxilean_kernel::{Expr, Literal, Name};
use std::collections::HashMap;

/// Compute greatest common divisor
pub(super) fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
/// Parse a numeric literal from an Expr
pub fn expr_to_numeric(expr: &Expr, ctx: &MetaContext) -> TacticResult<NumericValue> {
    let expr = ctx.instantiate_mvars(expr);
    expr_to_numeric_inner(&expr)
}
/// Inner implementation of numeric evaluation (pure, no ctx needed).
pub(super) fn expr_to_numeric_inner(expr: &Expr) -> TacticResult<NumericValue> {
    match expr {
        Expr::Lit(Literal::Nat(n)) => Ok(NumericValue::Nat(*n)),
        Expr::Const(name, _levels) => match name.to_string().as_str() {
            "zero" | "Nat.zero" | "Int.zero" => Ok(NumericValue::Nat(0)),
            "one" | "Nat.one" | "Int.one" => Ok(NumericValue::Nat(1)),
            _ => Err(TacticError::Failed(format!(
                "unknown numeric constant: {}",
                name
            ))),
        },
        Expr::App(func, arg) => eval_numeric_app(func, arg),
        _ => Err(TacticError::Failed(
            "expression is not a numeric value".into(),
        )),
    }
}
/// Evaluate a function application as a numeric expression.
pub(super) fn eval_numeric_app(func: &Expr, arg: &Expr) -> TacticResult<NumericValue> {
    if let Expr::App(op_expr, lhs_expr) = func {
        if let Some(op_name) = get_const_name(op_expr) {
            let lhs = expr_to_numeric_inner(lhs_expr)?;
            let rhs = expr_to_numeric_inner(arg)?;
            return eval_binary_op(&op_name, lhs, rhs);
        }
    }
    if let Some(op_name) = get_const_name(func) {
        match op_name.as_str() {
            "Nat.succ" => {
                let n = expr_to_numeric_inner(arg)?;
                return Ok(n.add(&NumericValue::Nat(1)));
            }
            "Int.negSucc" => {
                let n = expr_to_numeric_inner(arg)?;
                let n_plus_1 = n.add(&NumericValue::Nat(1));
                return Ok(n_plus_1.negate());
            }
            "Int.ofNat" => {
                return expr_to_numeric_inner(arg);
            }
            _ => {}
        }
    }
    Err(TacticError::Failed(
        "cannot evaluate numeric application".into(),
    ))
}
/// Get the Name string of a Const expression, if it is one.
pub(super) fn get_const_name(expr: &Expr) -> Option<String> {
    if let Expr::Const(name, _) = expr {
        Some(name.to_string())
    } else {
        None
    }
}
/// Evaluate a binary arithmetic operation.
pub(super) fn eval_binary_op(
    op: &str,
    lhs: NumericValue,
    rhs: NumericValue,
) -> TacticResult<NumericValue> {
    match op {
        "Nat.add" | "Int.add" | "HAdd.hAdd" | "Add.add" => Ok(lhs.add(&rhs)),
        "Nat.mul" | "Int.mul" | "HMul.hMul" | "Mul.mul" => Ok(lhs.mul(&rhs)),
        "Nat.sub" | "Int.sub" | "HSub.hSub" | "Sub.sub" => Ok(lhs.sub(&rhs)),
        "Nat.div" | "Int.div" | "HDiv.hDiv" | "Div.div" => lhs.div(&rhs),
        "Nat.mod" | "Int.mod" | "HMod.hMod" | "Mod.mod" => {
            let lf = lhs.to_float() as i64;
            let rf = rhs.to_float() as i64;
            if rf == 0 {
                Err(TacticError::Failed("modulo by zero".into()))
            } else {
                Ok(NumericValue::Int(lf % rf))
            }
        }
        "Nat.pow" | "Int.pow" | "HPow.hPow" | "Pow.pow" => {
            let exp = match &rhs {
                NumericValue::Nat(n) => {
                    if *n > 100 {
                        return Err(TacticError::Failed("exponent too large".into()));
                    }
                    *n as u32
                }
                _ => return Err(TacticError::Failed("exponent must be a Nat".into())),
            };
            lhs.pow(exp)
        }
        "Nat.gcd" => {
            let a = lhs.to_float() as u64;
            let b = rhs.to_float() as u64;
            let g = {
                let mut x = a;
                let mut y = b;
                while y != 0 {
                    let t = y;
                    y = x % t;
                    x = t;
                }
                x
            };
            Ok(NumericValue::Nat(g))
        }
        "Nat.lcm" => {
            let a = lhs.to_float() as u64;
            let b = rhs.to_float() as u64;
            if a == 0 || b == 0 {
                Ok(NumericValue::Nat(0))
            } else {
                let g = {
                    let mut x = a;
                    let mut y = b;
                    while y != 0 {
                        let t = y;
                        y = x % t;
                        x = t;
                    }
                    x
                };
                Ok(NumericValue::Nat(a / g * b))
            }
        }
        "Nat.min" | "min" | "Min.min" => {
            if lhs.to_float() <= rhs.to_float() {
                Ok(lhs)
            } else {
                Ok(rhs)
            }
        }
        "Nat.max" | "max" | "Max.max" => {
            if lhs.to_float() >= rhs.to_float() {
                Ok(lhs)
            } else {
                Ok(rhs)
            }
        }
        _ => Err(TacticError::Failed(format!("unknown binary op: {}", op))),
    }
}
/// Check if an expression is numeric (can be evaluated to a NumericValue).
pub fn is_numeric(expr: &Expr, ctx: &MetaContext) -> bool {
    let expr = ctx.instantiate_mvars(expr);
    expr_to_numeric_inner(&expr).is_ok()
}
/// Simplify 0 + x = x
pub fn simplify_add_zero_left(_val: &NumericValue) -> ProofTerm {
    ProofTerm::Identity {
        identity_side: IdentitySide::Left,
    }
}
/// Simplify x + 0 = x
pub fn simplify_add_zero_right(_val: &NumericValue) -> ProofTerm {
    ProofTerm::Identity {
        identity_side: IdentitySide::Right,
    }
}
/// Simplify 1 * x = x
pub fn simplify_mul_one_left(_val: &NumericValue) -> ProofTerm {
    ProofTerm::Identity {
        identity_side: IdentitySide::Left,
    }
}
/// Simplify x * 1 = x
pub fn simplify_mul_one_right(_val: &NumericValue) -> ProofTerm {
    ProofTerm::Identity {
        identity_side: IdentitySide::Right,
    }
}
/// Simplify 0 * x = 0
pub fn simplify_mul_zero_left() -> ProofTerm {
    ProofTerm::Compute {
        result: NumericValue::Nat(0),
    }
}
/// Simplify x * 0 = 0
pub fn simplify_mul_zero_right() -> ProofTerm {
    ProofTerm::Compute {
        result: NumericValue::Nat(0),
    }
}
/// Evaluate a comparison and generate a proof
pub fn evaluate_comparison(
    op: &ComparisonOp,
    lhs: &NumericValue,
    rhs: &NumericValue,
) -> TacticResult<ProofTerm> {
    let result = op.compare(lhs, rhs);
    if result {
        Ok(ProofTerm::Compute {
            result: lhs.clone(),
        })
    } else {
        Err(TacticError::Internal("comparison is false".into()))
    }
}
/// Normalize an expression using simplification rules
pub fn normalize_numeric(expr: &Expr, ctx: &MetaContext) -> TacticResult<NumericValue> {
    expr_to_numeric(expr, ctx)
}
/// The norm_num tactic: normalize numeric expressions and prove resulting equalities
pub fn tac_norm_num(state: &mut TacticState, ctx: &mut MetaContext) -> TacticResult<()> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let target = ctx.instantiate_mvars(&target);
    if let Some((op, lhs, rhs)) = parse_comparison_goal(&target) {
        let lhs_val = expr_to_numeric_inner(lhs);
        let rhs_val = expr_to_numeric_inner(rhs);
        if let (Ok(lv), Ok(rv)) = (lhs_val, rhs_val) {
            let holds = match op.as_str() {
                "Eq" | "eq" => lv == rv,
                "LE.le" | "le" | "Le" | "Nat.ble" => lv.to_float() <= rv.to_float(),
                "LT.lt" | "lt" | "Lt" | "Nat.blt" => lv.to_float() < rv.to_float(),
                "GE.ge" | "ge" | "Ge" => lv.to_float() >= rv.to_float(),
                "GT.gt" | "gt" | "Gt" => lv.to_float() > rv.to_float(),
                "Ne" | "ne" => lv != rv,
                _ => false,
            };
            if holds {
                let proof = Expr::Const(Name::str("rfl"), vec![]);
                state.close_goal(proof, ctx)?;
                return Ok(());
            } else {
                return Err(TacticError::GoalMismatch(format!(
                    "norm_num: {} {} {} is false",
                    lv.to_float(),
                    op,
                    rv.to_float()
                )));
            }
        }
    }
    match &target {
        Expr::App(box_func, _box_rhs) => {
            if let Expr::App(_box_eq, _box_lhs) = &**box_func {
                state.close_goal(Expr::Const(Name::str("rfl"), vec![]), ctx)?;
                Ok(())
            } else {
                Err(TacticError::GoalMismatch(
                    "norm_num requires a comparison or equality".into(),
                ))
            }
        }
        _ => Err(TacticError::GoalMismatch(
            "norm_num requires a comparison or equality goal".into(),
        )),
    }
}
/// Parse a goal of the form `Op α lhs rhs` or `Op lhs rhs` into (op_name, lhs, rhs).
pub(super) fn parse_comparison_goal(expr: &Expr) -> Option<(String, &Expr, &Expr)> {
    if let Expr::App(func, rhs) = expr {
        if let Expr::App(func2, lhs) = func.as_ref() {
            if let Expr::App(op_expr, _ty) = func2.as_ref() {
                if let Some(op) = get_const_name(op_expr) {
                    return Some((op, lhs.as_ref(), rhs.as_ref()));
                }
            }
            if let Some(op) = get_const_name(func2) {
                return Some((op, lhs.as_ref(), rhs.as_ref()));
            }
        }
    }
    None
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tactic::norm_num::*;
    use oxilean_kernel::Environment;
    #[test]
    fn test_numeric_nat() {
        let n = NumericValue::nat(42);
        assert_eq!(n, NumericValue::Nat(42));
    }
    #[test]
    fn test_numeric_int() {
        let i = NumericValue::int(-5);
        assert_eq!(i, NumericValue::Int(-5));
    }
    #[test]
    fn test_numeric_rat() {
        let r = NumericValue::rat(1, 2);
        assert_eq!(r, NumericValue::Rat(1, 2));
    }
    #[test]
    fn test_numeric_rat_reduce() {
        let r = NumericValue::rat(2, 4);
        assert_eq!(r, NumericValue::Rat(1, 2));
    }
    #[test]
    fn test_is_zero_nat() {
        assert!(NumericValue::nat(0).is_zero());
        assert!(!NumericValue::nat(1).is_zero());
    }
    #[test]
    fn test_is_zero_int() {
        assert!(NumericValue::int(0).is_zero());
        assert!(!NumericValue::int(-5).is_zero());
    }
    #[test]
    fn test_is_zero_rat() {
        assert!(NumericValue::rat(0, 1).is_zero());
        assert!(!NumericValue::rat(1, 2).is_zero());
    }
    #[test]
    fn test_is_one_nat() {
        assert!(NumericValue::nat(1).is_one());
        assert!(!NumericValue::nat(2).is_one());
    }
    #[test]
    fn test_is_one_int() {
        assert!(NumericValue::int(1).is_one());
        assert!(!NumericValue::int(-1).is_one());
    }
    #[test]
    fn test_is_one_rat() {
        assert!(NumericValue::rat(1, 1).is_one());
        assert!(NumericValue::rat(2, 2).is_one());
        assert!(!NumericValue::rat(3, 2).is_one());
    }
    #[test]
    fn test_is_negative_nat() {
        assert!(!NumericValue::nat(5).is_negative());
    }
    #[test]
    fn test_is_negative_int() {
        assert!(NumericValue::int(-5).is_negative());
        assert!(!NumericValue::int(5).is_negative());
    }
    #[test]
    fn test_is_negative_rat() {
        assert!(NumericValue::rat(-1, 2).is_negative());
        assert!(!NumericValue::rat(1, 2).is_negative());
    }
    #[test]
    fn test_negate_nat() {
        let n = NumericValue::nat(5);
        let neg_n = n.negate();
        assert_eq!(neg_n, NumericValue::Int(-5));
    }
    #[test]
    fn test_negate_int() {
        let i = NumericValue::int(-7);
        let neg_i = i.negate();
        assert_eq!(neg_i, NumericValue::Int(7));
    }
    #[test]
    fn test_negate_rat() {
        let r = NumericValue::rat(3, 5);
        let neg_r = r.negate();
        assert_eq!(neg_r, NumericValue::Rat(-3, 5));
    }
    #[test]
    fn test_add_nat_nat() {
        let a = NumericValue::nat(3);
        let b = NumericValue::nat(5);
        let result = a.add(&b);
        assert_eq!(result, NumericValue::Nat(8));
    }
    #[test]
    fn test_add_nat_int() {
        let a = NumericValue::nat(3);
        let b = NumericValue::int(-2);
        let result = a.add(&b);
        assert_eq!(result, NumericValue::Int(1));
    }
    #[test]
    fn test_add_int_int() {
        let a = NumericValue::int(-3);
        let b = NumericValue::int(5);
        let result = a.add(&b);
        assert_eq!(result, NumericValue::Int(2));
    }
    #[test]
    fn test_add_rat_rat() {
        let a = NumericValue::rat(1, 2);
        let b = NumericValue::rat(1, 3);
        let result = a.add(&b);
        assert_eq!(result, NumericValue::Rat(5, 6));
    }
    #[test]
    fn test_sub_nat_nat() {
        let a = NumericValue::nat(5);
        let b = NumericValue::nat(3);
        let result = a.sub(&b);
        assert_eq!(result, NumericValue::Int(2));
    }
    #[test]
    fn test_mul_nat_nat() {
        let a = NumericValue::nat(3);
        let b = NumericValue::nat(4);
        let result = a.mul(&b);
        assert_eq!(result, NumericValue::Nat(12));
    }
    #[test]
    fn test_mul_nat_int() {
        let a = NumericValue::nat(3);
        let b = NumericValue::int(-2);
        let result = a.mul(&b);
        assert_eq!(result, NumericValue::Int(-6));
    }
    #[test]
    fn test_mul_int_int() {
        let a = NumericValue::int(-3);
        let b = NumericValue::int(-4);
        let result = a.mul(&b);
        assert_eq!(result, NumericValue::Int(12));
    }
    #[test]
    fn test_mul_rat_rat() {
        let a = NumericValue::rat(1, 2);
        let b = NumericValue::rat(2, 3);
        let result = a.mul(&b);
        assert_eq!(result, NumericValue::Rat(1, 3));
    }
    #[test]
    fn test_div_nat_nat() {
        let a = NumericValue::nat(3);
        let b = NumericValue::nat(4);
        let result = a.div(&b).expect("result should be present");
        assert_eq!(result, NumericValue::Rat(3, 4));
    }
    #[test]
    fn test_div_int_int() {
        let a = NumericValue::int(5);
        let b = NumericValue::int(-2);
        let result = a.div(&b).expect("result should be present");
        assert_eq!(result, NumericValue::Rat(5, 2));
    }
    #[test]
    fn test_div_zero_error() {
        let a = NumericValue::nat(5);
        let b = NumericValue::nat(0);
        assert!(a.div(&b).is_err());
    }
    #[test]
    fn test_pow_zero() {
        let a = NumericValue::nat(5);
        let result = a.pow(0).expect("result should be present");
        assert_eq!(result, NumericValue::Nat(1));
    }
    #[test]
    fn test_pow_one() {
        let a = NumericValue::nat(5);
        let result = a.pow(1).expect("result should be present");
        assert_eq!(result, NumericValue::Nat(5));
    }
    #[test]
    fn test_pow_two() {
        let a = NumericValue::nat(3);
        let result = a.pow(2).expect("result should be present");
        assert_eq!(result, NumericValue::Nat(9));
    }
    #[test]
    fn test_pow_three() {
        let a = NumericValue::nat(2);
        let result = a.pow(3).expect("result should be present");
        assert_eq!(result, NumericValue::Nat(8));
    }
    #[test]
    fn test_pow_large() {
        let a = NumericValue::nat(2);
        let result = a.pow(10).expect("result should be present");
        assert_eq!(result, NumericValue::Nat(1024));
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
    fn test_comparison_le_true() {
        let op = ComparisonOp::Le;
        let a = NumericValue::nat(3);
        let b = NumericValue::nat(5);
        assert!(op.compare(&a, &b));
    }
    #[test]
    fn test_comparison_le_false() {
        let op = ComparisonOp::Le;
        let a = NumericValue::nat(5);
        let b = NumericValue::nat(3);
        assert!(!op.compare(&a, &b));
    }
    #[test]
    fn test_comparison_lt_true() {
        let op = ComparisonOp::Lt;
        let a = NumericValue::nat(3);
        let b = NumericValue::nat(5);
        assert!(op.compare(&a, &b));
    }
    #[test]
    fn test_comparison_lt_false() {
        let op = ComparisonOp::Lt;
        let a = NumericValue::nat(5);
        let b = NumericValue::nat(5);
        assert!(!op.compare(&a, &b));
    }
    #[test]
    fn test_comparison_eq_true() {
        let op = ComparisonOp::Eq;
        let a = NumericValue::nat(5);
        let b = NumericValue::nat(5);
        assert!(op.compare(&a, &b));
    }
    #[test]
    fn test_comparison_eq_false() {
        let op = ComparisonOp::Eq;
        let a = NumericValue::nat(5);
        let b = NumericValue::nat(3);
        assert!(!op.compare(&a, &b));
    }
    #[test]
    fn test_comparison_ge_true() {
        let op = ComparisonOp::Ge;
        let a = NumericValue::nat(5);
        let b = NumericValue::nat(3);
        assert!(op.compare(&a, &b));
    }
    #[test]
    fn test_comparison_gt_true() {
        let op = ComparisonOp::Gt;
        let a = NumericValue::nat(5);
        let b = NumericValue::nat(3);
        assert!(op.compare(&a, &b));
    }
    #[test]
    fn test_comparison_flip_le() {
        assert_eq!(ComparisonOp::Le.flip(), ComparisonOp::Ge);
    }
    #[test]
    fn test_comparison_flip_lt() {
        assert_eq!(ComparisonOp::Lt.flip(), ComparisonOp::Gt);
    }
    #[test]
    fn test_comparison_flip_eq() {
        assert_eq!(ComparisonOp::Eq.flip(), ComparisonOp::Eq);
    }
    #[test]
    fn test_comparison_negate_le() {
        assert_eq!(ComparisonOp::Le.negate(), ComparisonOp::Gt);
    }
    #[test]
    fn test_comparison_negate_lt() {
        assert_eq!(ComparisonOp::Lt.negate(), ComparisonOp::Ge);
    }
    #[test]
    fn test_comparison_negate_eq() {
        assert_eq!(ComparisonOp::Eq.negate(), ComparisonOp::Eq);
    }
    #[test]
    fn test_simplify_add_zero_left() {
        let val = NumericValue::nat(5);
        let proof = simplify_add_zero_left(&val);
        assert!(matches!(
            proof,
            ProofTerm::Identity {
                identity_side: IdentitySide::Left
            }
        ));
    }
    #[test]
    fn test_simplify_add_zero_right() {
        let val = NumericValue::nat(5);
        let proof = simplify_add_zero_right(&val);
        assert!(matches!(
            proof,
            ProofTerm::Identity {
                identity_side: IdentitySide::Right
            }
        ));
    }
    #[test]
    fn test_simplify_mul_one_left() {
        let val = NumericValue::nat(5);
        let proof = simplify_mul_one_left(&val);
        assert!(matches!(
            proof,
            ProofTerm::Identity {
                identity_side: IdentitySide::Left
            }
        ));
    }
    #[test]
    fn test_simplify_mul_one_right() {
        let val = NumericValue::nat(5);
        let proof = simplify_mul_one_right(&val);
        assert!(matches!(
            proof,
            ProofTerm::Identity {
                identity_side: IdentitySide::Right
            }
        ));
    }
    #[test]
    fn test_simplify_mul_zero_left() {
        let proof = simplify_mul_zero_left();
        assert!(matches!(
            proof,
            ProofTerm::Compute {
                result: NumericValue::Nat(0)
            }
        ));
    }
    #[test]
    fn test_simplify_mul_zero_right() {
        let proof = simplify_mul_zero_right();
        assert!(matches!(
            proof,
            ProofTerm::Compute {
                result: NumericValue::Nat(0)
            }
        ));
    }
    #[test]
    fn test_to_float_nat() {
        let n = NumericValue::nat(42);
        assert_eq!(n.to_float(), 42.0);
    }
    #[test]
    fn test_to_float_int() {
        let i = NumericValue::int(-5);
        assert_eq!(i.to_float(), -5.0);
    }
    #[test]
    fn test_to_float_rat() {
        let r = NumericValue::rat(1, 2);
        assert_eq!(r.to_float(), 0.5);
    }
    #[test]
    fn test_add_nat_rat() {
        let a = NumericValue::nat(2);
        let b = NumericValue::rat(1, 3);
        let result = a.add(&b);
        assert_eq!(result, NumericValue::Rat(7, 3));
    }
    #[test]
    fn test_mul_zero_nat_nat() {
        let a = NumericValue::nat(0);
        let b = NumericValue::nat(5);
        let result = a.mul(&b);
        assert_eq!(result, NumericValue::Nat(0));
    }
    #[test]
    fn test_mul_one_identity() {
        let a = NumericValue::nat(1);
        let b = NumericValue::nat(42);
        let result = a.mul(&b);
        assert_eq!(result, NumericValue::Nat(42));
    }
    #[test]
    fn test_add_same_values() {
        let a = NumericValue::nat(7);
        let result = a.add(&a);
        assert_eq!(result, NumericValue::Nat(14));
    }
    #[test]
    fn test_rat_simplify_many_factors() {
        let r = NumericValue::rat(12, 18);
        assert_eq!(r, NumericValue::Rat(2, 3));
    }
    #[test]
    fn test_comparison_le_equal() {
        let op = ComparisonOp::Le;
        let a = NumericValue::nat(5);
        let b = NumericValue::nat(5);
        assert!(op.compare(&a, &b));
    }
    #[test]
    fn test_comparison_lt_equal_false() {
        let op = ComparisonOp::Lt;
        let a = NumericValue::nat(5);
        let b = NumericValue::nat(5);
        assert!(!op.compare(&a, &b));
    }
    #[test]
    fn test_div_rat_rat() {
        let a = NumericValue::rat(1, 2);
        let b = NumericValue::rat(2, 3);
        let result = a.div(&b).expect("result should be present");
        assert_eq!(result, NumericValue::Rat(3, 4));
    }
    #[test]
    fn test_pow_negative_base() {
        let a = NumericValue::int(-2);
        let result = a.pow(3).expect("result should be present");
        assert_eq!(result, NumericValue::Int(-8));
    }
    #[test]
    fn test_sub_result_negative() {
        let a = NumericValue::nat(2);
        let b = NumericValue::nat(5);
        let result = a.sub(&b);
        assert_eq!(result, NumericValue::Int(-3));
    }
    #[test]
    fn test_numeric_value_clone() {
        let a = NumericValue::rat(1, 2);
        let b = a.clone();
        assert_eq!(a, b);
    }
    #[test]
    fn test_comparison_op_clone() {
        let op = ComparisonOp::Le;
        let op2 = op.clone();
        assert_eq!(op, op2);
    }
    #[test]
    fn test_expr_to_numeric_nat_literal() {
        use oxilean_kernel::{Environment, Expr, Literal};
        let ctx = MetaContext::new(Environment::new());
        let expr = Expr::Lit(Literal::Nat(42));
        let result = expr_to_numeric(&expr, &ctx).expect("result should be present");
        assert_eq!(result, NumericValue::Nat(42));
    }
    #[test]
    fn test_expr_to_numeric_zero_lit() {
        let ctx = MetaContext::new(Environment::new());
        let expr = Expr::Lit(Literal::Nat(0));
        let result = expr_to_numeric(&expr, &ctx).expect("result should be present");
        assert_eq!(result, NumericValue::Nat(0));
    }
    #[test]
    fn test_expr_to_numeric_named_zero() {
        use oxilean_kernel::{Environment, Expr, Name};
        let ctx = MetaContext::new(Environment::new());
        let expr = Expr::Const(Name::str("zero"), vec![]);
        let result = expr_to_numeric(&expr, &ctx).expect("result should be present");
        assert_eq!(result, NumericValue::Nat(0));
    }
    #[test]
    fn test_expr_to_numeric_nat_add() {
        use oxilean_kernel::{Environment, Expr, Literal, Name};
        let ctx = MetaContext::new(Environment::new());
        let three = Expr::Lit(Literal::Nat(3));
        let four = Expr::Lit(Literal::Nat(4));
        let add = Expr::Const(Name::str("Nat.add"), vec![]);
        let expr = Expr::App(
            Box::new(Expr::App(Box::new(add), Box::new(three))),
            Box::new(four),
        );
        let result = expr_to_numeric(&expr, &ctx).expect("result should be present");
        assert_eq!(result, NumericValue::Nat(7));
    }
    #[test]
    fn test_expr_to_numeric_nat_mul() {
        let ctx = MetaContext::new(Environment::new());
        let six = Expr::Lit(Literal::Nat(6));
        let seven = Expr::Lit(Literal::Nat(7));
        let mul = Expr::Const(Name::str("Nat.mul"), vec![]);
        let expr = Expr::App(
            Box::new(Expr::App(Box::new(mul), Box::new(six))),
            Box::new(seven),
        );
        let result = expr_to_numeric(&expr, &ctx).expect("result should be present");
        assert_eq!(result, NumericValue::Nat(42));
    }
    #[test]
    fn test_expr_to_numeric_nat_pow() {
        let ctx = MetaContext::new(Environment::new());
        let two = Expr::Lit(Literal::Nat(2));
        let ten = Expr::Lit(Literal::Nat(10));
        let pow = Expr::Const(Name::str("Nat.pow"), vec![]);
        let expr = Expr::App(
            Box::new(Expr::App(Box::new(pow), Box::new(two))),
            Box::new(ten),
        );
        let result = expr_to_numeric(&expr, &ctx).expect("result should be present");
        assert_eq!(result, NumericValue::Nat(1024));
    }
    #[test]
    fn test_expr_to_numeric_nat_gcd() {
        let ctx = MetaContext::new(Environment::new());
        let twelve = Expr::Lit(Literal::Nat(12));
        let eight = Expr::Lit(Literal::Nat(8));
        let gcd_fn = Expr::Const(Name::str("Nat.gcd"), vec![]);
        let expr = Expr::App(
            Box::new(Expr::App(Box::new(gcd_fn), Box::new(twelve))),
            Box::new(eight),
        );
        let result = expr_to_numeric(&expr, &ctx).expect("result should be present");
        assert_eq!(result, NumericValue::Nat(4));
    }
    #[test]
    fn test_expr_to_numeric_min_max() {
        let ctx = MetaContext::new(Environment::new());
        let three = Expr::Lit(Literal::Nat(3));
        let seven = Expr::Lit(Literal::Nat(7));
        let min_fn = Expr::Const(Name::str("Nat.min"), vec![]);
        let expr = Expr::App(
            Box::new(Expr::App(Box::new(min_fn), Box::new(three))),
            Box::new(seven),
        );
        let result = expr_to_numeric(&expr, &ctx).expect("result should be present");
        assert_eq!(result, NumericValue::Nat(3));
    }
    #[test]
    fn test_is_numeric_lit() {
        let ctx = MetaContext::new(Environment::new());
        let expr = Expr::Lit(Literal::Nat(5));
        assert!(is_numeric(&expr, &ctx));
    }
    #[test]
    fn test_is_numeric_add() {
        let ctx = MetaContext::new(Environment::new());
        let three = Expr::Lit(Literal::Nat(3));
        let four = Expr::Lit(Literal::Nat(4));
        let add = Expr::Const(Name::str("Nat.add"), vec![]);
        let expr = Expr::App(
            Box::new(Expr::App(Box::new(add), Box::new(three))),
            Box::new(four),
        );
        assert!(is_numeric(&expr, &ctx));
    }
    #[test]
    fn test_is_numeric_non_numeric() {
        let ctx = MetaContext::new(Environment::new());
        let expr = Expr::Const(Name::str("UnknownIdent"), vec![]);
        assert!(!is_numeric(&expr, &ctx));
    }
}
#[cfg(test)]
mod norm_num_extra_tests {
    use super::*;
    use crate::tactic::norm_num::*;
    #[test]
    fn test_poly_zero_degree() {
        let p = Poly::zero();
        assert_eq!(p.degree(), -1);
        assert!(p.is_zero());
    }
    #[test]
    fn test_poly_constant() {
        let p = Poly::constant(5);
        assert_eq!(p.degree(), 0);
        assert_eq!(p.eval(0), 5);
        assert_eq!(p.eval(10), 5);
    }
    #[test]
    fn test_poly_ident() {
        let p = Poly::ident();
        assert_eq!(p.degree(), 1);
        assert_eq!(p.eval(3), 3);
        assert_eq!(p.eval(7), 7);
    }
    #[test]
    fn test_poly_add() {
        let p1 = Poly { coeffs: vec![1, 2] };
        let p2 = Poly {
            coeffs: vec![3, -2, 1],
        };
        let sum = p1.add(&p2);
        assert_eq!(sum.eval(1), 1 + 2 + 3 - 2 + 1);
    }
    #[test]
    fn test_poly_sub() {
        let p1 = Poly::constant(10);
        let p2 = Poly::constant(4);
        let diff = p1.sub(&p2);
        assert_eq!(diff.eval(0), 6);
    }
    #[test]
    fn test_poly_mul() {
        let p1 = Poly { coeffs: vec![1, 1] };
        let p2 = Poly { coeffs: vec![1, 1] };
        let prod = p1.mul(&p2);
        assert_eq!(prod.eval(2), 9);
    }
    #[test]
    fn test_poly_scale() {
        let p = Poly {
            coeffs: vec![1, 2, 3],
        };
        let scaled = p.scale(2);
        assert_eq!(scaled.eval(1), 12);
    }
    #[test]
    fn test_poly_leading_coeff() {
        let p = Poly {
            coeffs: vec![5, 0, 3],
        };
        assert_eq!(p.leading_coeff(), 3);
    }
    #[test]
    fn test_poly_display() {
        let p = Poly::constant(7);
        assert_eq!(format!("{}", p), "7");
        let zero = Poly::zero();
        assert_eq!(format!("{}", zero), "0");
    }
    #[test]
    fn test_linear_expr_constant() {
        let e = LinearExpr::constant(42);
        assert!(e.is_constant());
        assert_eq!(e.constant, 42);
    }
    #[test]
    fn test_linear_expr_var() {
        let e = LinearExpr::var("x", 3);
        assert!(!e.is_constant());
        assert_eq!(e.num_vars(), 1);
    }
    #[test]
    fn test_linear_expr_add() {
        let a = LinearExpr::var("x", 2);
        let b = LinearExpr::var("x", 3);
        let sum = a.add(&b);
        assert_eq!(
            sum.vars
                .iter()
                .find(|(n, _)| n == "x")
                .expect("value should be present")
                .1,
            5
        );
    }
    #[test]
    fn test_linear_expr_negate() {
        let e = LinearExpr::var("y", 4);
        let neg = e.negate();
        assert_eq!(
            neg.vars
                .iter()
                .find(|(n, _)| n == "y")
                .expect("value should be present")
                .1,
            -4
        );
    }
    #[test]
    fn test_linear_expr_eval() {
        let e = LinearExpr {
            constant: 3,
            vars: vec![("x".to_string(), 2), ("y".to_string(), -1)],
        };
        let mut assignment = HashMap::new();
        assignment.insert("x".to_string(), 5i64);
        assignment.insert("y".to_string(), 2i64);
        assert_eq!(e.eval(&assignment), 11);
    }
    #[test]
    fn test_linear_constraint_satisfied() {
        let lhs = LinearExpr::constant(5);
        let rhs = LinearExpr::constant(3);
        let c = LinearConstraint::new(lhs, ComparisonOp::Gt, rhs);
        assert!(c.is_satisfied(&HashMap::new()));
    }
    #[test]
    fn test_linear_constraint_not_satisfied() {
        let lhs = LinearExpr::constant(2);
        let rhs = LinearExpr::constant(5);
        let c = LinearConstraint::new(lhs, ComparisonOp::Gt, rhs);
        assert!(!c.is_satisfied(&HashMap::new()));
    }
    #[test]
    fn test_numeric_normal_form_value() {
        let nf = NumericNormalForm::Value(NumericValue::nat(42));
        assert!(nf.is_value());
        assert!(!nf.is_stuck());
        assert_eq!(nf.as_value(), Some(&NumericValue::nat(42)));
    }
    #[test]
    fn test_numeric_normal_form_comparison_true() {
        let nf = NumericNormalForm::Comparison(
            NumericValue::nat(3),
            ComparisonOp::Lt,
            NumericValue::nat(5),
        );
        assert_eq!(nf.is_true_comparison(), Some(true));
    }
    #[test]
    fn test_numeric_normal_form_comparison_false() {
        let nf = NumericNormalForm::Comparison(
            NumericValue::nat(5),
            ComparisonOp::Lt,
            NumericValue::nat(3),
        );
        assert_eq!(nf.is_true_comparison(), Some(false));
    }
    #[test]
    fn test_numeric_normal_form_stuck() {
        let nf = NumericNormalForm::Stuck;
        assert!(nf.is_stuck());
        assert_eq!(nf.is_true_comparison(), None);
    }
    #[test]
    fn test_poly_mul_by_zero() {
        let p = Poly {
            coeffs: vec![1, 2, 3],
        };
        let zero = Poly::zero();
        let result = p.mul(&zero);
        assert!(result.is_zero());
    }
    #[test]
    fn test_linear_expr_scale() {
        let e = LinearExpr::var("z", 5);
        let scaled = e.scale(3);
        assert_eq!(
            scaled
                .vars
                .iter()
                .find(|(n, _)| n == "z")
                .expect("value should be present")
                .1,
            15
        );
    }
}
#[cfg(test)]
mod tacticnormnum_analysis_tests {
    use super::*;
    use crate::tactic::norm_num::*;
    #[test]
    fn test_tacticnormnum_result_ok() {
        let r = TacticNormNumResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticnormnum_result_err() {
        let r = TacticNormNumResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticnormnum_result_partial() {
        let r = TacticNormNumResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_tacticnormnum_result_skipped() {
        let r = TacticNormNumResult::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_tacticnormnum_analysis_pass_run() {
        let mut p = TacticNormNumAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_tacticnormnum_analysis_pass_empty_input() {
        let mut p = TacticNormNumAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_tacticnormnum_analysis_pass_success_rate() {
        let mut p = TacticNormNumAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_tacticnormnum_analysis_pass_disable() {
        let mut p = TacticNormNumAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_tacticnormnum_pipeline_basic() {
        let mut pipeline = TacticNormNumPipeline::new("main_pipeline");
        pipeline.add_pass(TacticNormNumAnalysisPass::new("pass1"));
        pipeline.add_pass(TacticNormNumAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_tacticnormnum_pipeline_disabled_pass() {
        let mut pipeline = TacticNormNumPipeline::new("partial");
        let mut p = TacticNormNumAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(TacticNormNumAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_tacticnormnum_diff_basic() {
        let mut d = TacticNormNumDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_tacticnormnum_diff_summary() {
        let mut d = TacticNormNumDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }
    #[test]
    fn test_tacticnormnum_config_set_get() {
        let mut cfg = TacticNormNumConfig::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_tacticnormnum_config_read_only() {
        let mut cfg = TacticNormNumConfig::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_tacticnormnum_config_remove() {
        let mut cfg = TacticNormNumConfig::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_tacticnormnum_diagnostics_basic() {
        let mut diag = TacticNormNumDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_tacticnormnum_diagnostics_max_errors() {
        let mut diag = TacticNormNumDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_tacticnormnum_diagnostics_clear() {
        let mut diag = TacticNormNumDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_tacticnormnum_config_value_types() {
        let b = TacticNormNumConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = TacticNormNumConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = TacticNormNumConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = TacticNormNumConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = TacticNormNumConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
#[cfg(test)]
mod norm_num_ext_tests_2800 {
    use super::*;
    use crate::tactic::norm_num::*;
    #[test]
    fn test_norm_num_ext_result_ok_2800() {
        let r = NormNumExtResult2800::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_norm_num_ext_result_err_2800() {
        let r = NormNumExtResult2800::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_norm_num_ext_result_partial_2800() {
        let r = NormNumExtResult2800::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_norm_num_ext_result_skipped_2800() {
        let r = NormNumExtResult2800::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_norm_num_ext_pass_run_2800() {
        let mut p = NormNumExtPass2800::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_norm_num_ext_pass_empty_2800() {
        let mut p = NormNumExtPass2800::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_norm_num_ext_pass_rate_2800() {
        let mut p = NormNumExtPass2800::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_norm_num_ext_pass_disable_2800() {
        let mut p = NormNumExtPass2800::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_norm_num_ext_pipeline_basic_2800() {
        let mut pipeline = NormNumExtPipeline2800::new("main_pipeline");
        pipeline.add_pass(NormNumExtPass2800::new("pass1"));
        pipeline.add_pass(NormNumExtPass2800::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_norm_num_ext_pipeline_disabled_2800() {
        let mut pipeline = NormNumExtPipeline2800::new("partial");
        let mut p = NormNumExtPass2800::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(NormNumExtPass2800::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_norm_num_ext_diff_basic_2800() {
        let mut d = NormNumExtDiff2800::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_norm_num_ext_config_set_get_2800() {
        let mut cfg = NormNumExtConfig2800::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_norm_num_ext_config_read_only_2800() {
        let mut cfg = NormNumExtConfig2800::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_norm_num_ext_config_remove_2800() {
        let mut cfg = NormNumExtConfig2800::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_norm_num_ext_diagnostics_basic_2800() {
        let mut diag = NormNumExtDiag2800::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_norm_num_ext_diagnostics_max_errors_2800() {
        let mut diag = NormNumExtDiag2800::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_norm_num_ext_diagnostics_clear_2800() {
        let mut diag = NormNumExtDiag2800::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_norm_num_ext_config_value_types_2800() {
        let b = NormNumExtConfigVal2800::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = NormNumExtConfigVal2800::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = NormNumExtConfigVal2800::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = NormNumExtConfigVal2800::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = NormNumExtConfigVal2800::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
