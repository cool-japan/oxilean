//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::{LcnfArg, LcnfExpr, LcnfFunDecl, LcnfLetValue, LcnfLit, LcnfVarId};

use super::types::{
    BinOp, CtfeConfig, CtfeContext, CtfeError, CtfeFeatureFlags, CtfeFuncEntry, CtfeInterpreter,
    CtfePass, CtfeReport, CtfeType, CtfeValue, CtfeValueExt,
};

/// Convenient result type for CTFE operations.
pub type CtfeResult = Result<CtfeValue, CtfeError>;
pub(super) fn ctfe_value_to_arg(val: &CtfeValue) -> LcnfArg {
    match val {
        CtfeValue::Int(n) => {
            if *n >= 0 {
                LcnfArg::Lit(LcnfLit::Nat(*n as u64))
            } else {
                LcnfArg::Lit(LcnfLit::Nat(n.unsigned_abs()))
            }
        }
        CtfeValue::String(s) => LcnfArg::Lit(LcnfLit::Str(s.clone())),
        _ => LcnfArg::Erased,
    }
}
pub(super) fn ctfe_value_to_let_value(val: CtfeValue) -> LcnfLetValue {
    match val {
        CtfeValue::Int(n) => LcnfLetValue::Lit(LcnfLit::Nat(if n >= 0 {
            n as u64
        } else {
            n.unsigned_abs()
        })),
        CtfeValue::String(s) => LcnfLetValue::Lit(LcnfLit::Str(s)),
        CtfeValue::Constructor(name, fields) => {
            let args: Vec<LcnfArg> = fields.iter().map(ctfe_value_to_arg).collect();
            LcnfLetValue::Ctor(name, 0, args)
        }
        _ => LcnfLetValue::Erased,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::lcnf::{LcnfFunDecl, LcnfParam, LcnfType, LcnfVarId};
    pub(super) fn vid(n: u64) -> LcnfVarId {
        LcnfVarId(n)
    }
    pub(super) fn mk_param(n: u64, name: &str) -> LcnfParam {
        LcnfParam {
            id: vid(n),
            name: name.to_string(),
            ty: LcnfType::Nat,
            erased: false,
            borrowed: false,
        }
    }
    pub(super) fn mk_const_decl(name: &str, body: LcnfExpr) -> LcnfFunDecl {
        LcnfFunDecl {
            name: name.to_string(),
            original_name: None,
            params: vec![],
            ret_type: LcnfType::Nat,
            body,
            is_recursive: false,
            is_lifted: false,
            inline_cost: 0,
        }
    }
    pub(super) fn mk_decl_with_params(
        name: &str,
        params: Vec<LcnfParam>,
        body: LcnfExpr,
    ) -> LcnfFunDecl {
        LcnfFunDecl {
            name: name.to_string(),
            original_name: None,
            params,
            ret_type: LcnfType::Nat,
            body,
            is_recursive: false,
            is_lifted: false,
            inline_cost: 1,
        }
    }
    pub(super) fn ret_nat(n: u64) -> LcnfExpr {
        LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(n)))
    }
    pub(super) fn ret_str(s: &str) -> LcnfExpr {
        LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Str(s.to_string())))
    }
    #[test]
    pub(super) fn ctfe_value_int_display() {
        assert_eq!(CtfeValue::Int(42).to_string(), "42");
    }
    #[test]
    pub(super) fn ctfe_value_bool_display() {
        assert_eq!(CtfeValue::Bool(true).to_string(), "true");
        assert_eq!(CtfeValue::Bool(false).to_string(), "false");
    }
    #[test]
    pub(super) fn ctfe_value_string_display() {
        assert_eq!(CtfeValue::String("hi".to_string()).to_string(), "\"hi\"");
    }
    #[test]
    pub(super) fn ctfe_value_undef_display() {
        assert_eq!(CtfeValue::Undef.to_string(), "undef");
    }
    #[test]
    pub(super) fn ctfe_value_list_display() {
        let v = CtfeValue::List(vec![CtfeValue::Int(1), CtfeValue::Int(2)]);
        assert_eq!(v.to_string(), "[1, 2]");
    }
    #[test]
    pub(super) fn ctfe_value_tuple_display() {
        let v = CtfeValue::Tuple(vec![CtfeValue::Int(3), CtfeValue::Bool(true)]);
        assert_eq!(v.to_string(), "(3, true)");
    }
    #[test]
    pub(super) fn ctfe_value_constructor_display() {
        let v = CtfeValue::Constructor(
            "Pair".to_string(),
            vec![CtfeValue::Int(1), CtfeValue::Int(2)],
        );
        assert_eq!(v.to_string(), "Pair(1, 2)");
    }
    #[test]
    pub(super) fn ctfe_value_is_concrete() {
        assert!(CtfeValue::Int(0).is_concrete());
        assert!(!CtfeValue::Undef.is_concrete());
        let v = CtfeValue::List(vec![CtfeValue::Int(1), CtfeValue::Undef]);
        assert!(!v.is_concrete());
    }
    #[test]
    pub(super) fn ctfe_value_as_int() {
        assert_eq!(CtfeValue::Int(7).as_int(), Some(7));
        assert_eq!(CtfeValue::Bool(true).as_int(), None);
    }
    #[test]
    pub(super) fn ctfe_value_as_bool() {
        assert_eq!(CtfeValue::Bool(false).as_bool(), Some(false));
        assert_eq!(CtfeValue::Int(1).as_bool(), None);
    }
    #[test]
    pub(super) fn ctfe_value_as_str() {
        let s = CtfeValue::String("hello".to_string());
        assert_eq!(s.as_str(), Some("hello"));
        assert_eq!(CtfeValue::Int(0).as_str(), None);
    }
    #[test]
    pub(super) fn ctfe_error_display_division_by_zero() {
        assert_eq!(CtfeError::DivisionByZero.to_string(), "division by zero");
    }
    #[test]
    pub(super) fn ctfe_error_display_index_out_of_bounds() {
        let e = CtfeError::IndexOutOfBounds {
            index: 5,
            length: 3,
        };
        let s = e.to_string();
        assert!(s.contains("5") && s.contains("3"));
    }
    #[test]
    pub(super) fn ctfe_error_display_stack_overflow() {
        let e = CtfeError::StackOverflow { depth: 256 };
        assert!(e.to_string().contains("256"));
    }
    #[test]
    pub(super) fn ctfe_error_display_timeout() {
        let e = CtfeError::Timeout { fuel_used: 10000 };
        assert!(e.to_string().contains("10000"));
    }
    #[test]
    pub(super) fn binop_from_name_recognized() {
        assert_eq!(BinOp::from_name("add"), Some(BinOp::Add));
        assert_eq!(BinOp::from_name("+"), Some(BinOp::Add));
        assert_eq!(BinOp::from_name("Nat.add"), Some(BinOp::Add));
        assert_eq!(BinOp::from_name("eq"), Some(BinOp::Eq));
        assert_eq!(BinOp::from_name("=="), Some(BinOp::Eq));
        assert_eq!(BinOp::from_name("&&"), Some(BinOp::And));
    }
    #[test]
    pub(super) fn binop_from_name_unknown() {
        assert_eq!(BinOp::from_name("foo"), None);
    }
    #[test]
    pub(super) fn eval_lit_nat() {
        let interp = CtfeInterpreter::new(&[]);
        assert_eq!(interp.eval_lit(&LcnfLit::Nat(42)), CtfeValue::Int(42));
    }
    #[test]
    pub(super) fn eval_lit_str() {
        let interp = CtfeInterpreter::new(&[]);
        let v = interp.eval_lit(&LcnfLit::Str("hello".to_string()));
        assert_eq!(v, CtfeValue::String("hello".to_string()));
    }
    #[test]
    pub(super) fn eval_binop_add() {
        let interp = CtfeInterpreter::new(&[]);
        let r = interp.eval_binop(BinOp::Add, &CtfeValue::Int(3), &CtfeValue::Int(4));
        assert_eq!(r, Ok(CtfeValue::Int(7)));
    }
    #[test]
    pub(super) fn eval_binop_div_by_zero() {
        let interp = CtfeInterpreter::new(&[]);
        let r = interp.eval_binop(BinOp::Div, &CtfeValue::Int(5), &CtfeValue::Int(0));
        assert_eq!(r, Err(CtfeError::DivisionByZero));
    }
    #[test]
    pub(super) fn eval_binop_mod_by_zero() {
        let interp = CtfeInterpreter::new(&[]);
        let r = interp.eval_binop(BinOp::Mod, &CtfeValue::Int(5), &CtfeValue::Int(0));
        assert_eq!(r, Err(CtfeError::DivisionByZero));
    }
    #[test]
    pub(super) fn eval_binop_bool_and() {
        let interp = CtfeInterpreter::new(&[]);
        let r = interp.eval_binop(BinOp::And, &CtfeValue::Bool(true), &CtfeValue::Bool(false));
        assert_eq!(r, Ok(CtfeValue::Bool(false)));
    }
    #[test]
    pub(super) fn eval_binop_comparison_lt() {
        let interp = CtfeInterpreter::new(&[]);
        let r = interp.eval_binop(BinOp::Lt, &CtfeValue::Int(2), &CtfeValue::Int(5));
        assert_eq!(r, Ok(CtfeValue::Bool(true)));
    }
    #[test]
    pub(super) fn eval_binop_string_eq() {
        let interp = CtfeInterpreter::new(&[]);
        let r = interp.eval_binop(
            BinOp::Eq,
            &CtfeValue::String("abc".to_string()),
            &CtfeValue::String("abc".to_string()),
        );
        assert_eq!(r, Ok(CtfeValue::Bool(true)));
    }
    #[test]
    pub(super) fn eval_expr_return_literal() {
        let decl = mk_const_decl("forty_two", ret_nat(42));
        let mut interp = CtfeInterpreter::new(&[decl]);
        let mut ctx = CtfeContext::new();
        let r = interp.eval_expr(&mk_const_decl("", ret_nat(42)).body, &mut ctx);
        assert_eq!(r, Ok(CtfeValue::Int(42)));
    }
    #[test]
    pub(super) fn eval_expr_return_string() {
        let mut interp = CtfeInterpreter::new(&[]);
        let mut ctx = CtfeContext::new();
        let body = ret_str("world");
        let r = interp.eval_expr(&body, &mut ctx);
        assert_eq!(r, Ok(CtfeValue::String("world".to_string())));
    }
    #[test]
    pub(super) fn eval_expr_let_binding() {
        let body = LcnfExpr::Let {
            id: vid(1),
            name: "x".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Lit(LcnfLit::Nat(10)),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(vid(1)))),
        };
        let mut interp = CtfeInterpreter::new(&[]);
        let mut ctx = CtfeContext::new();
        assert_eq!(interp.eval_expr(&body, &mut ctx), Ok(CtfeValue::Int(10)));
    }
    #[test]
    pub(super) fn eval_expr_constructor() {
        let body = LcnfExpr::Let {
            id: vid(5),
            name: "p".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Ctor(
                "Pair".to_string(),
                0,
                vec![LcnfArg::Lit(LcnfLit::Nat(1)), LcnfArg::Lit(LcnfLit::Nat(2))],
            ),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(vid(5)))),
        };
        let mut interp = CtfeInterpreter::new(&[]);
        let mut ctx = CtfeContext::new();
        let r = interp
            .eval_expr(&body, &mut ctx)
            .expect("r evaluation should succeed");
        assert_eq!(
            r,
            CtfeValue::Constructor(
                "Pair".to_string(),
                vec![CtfeValue::Int(1), CtfeValue::Int(2)]
            )
        );
    }
    #[test]
    pub(super) fn ctfe_context_fuel_consumption() {
        let mut ctx = CtfeContext::with_fuel(3);
        assert!(ctx.consume_fuel().is_ok());
        assert!(ctx.consume_fuel().is_ok());
        assert!(ctx.consume_fuel().is_ok());
        assert!(ctx.consume_fuel().is_err());
    }
    #[test]
    pub(super) fn ctfe_context_stack_overflow() {
        let mut ctx = CtfeContext::new();
        ctx.max_depth = 2;
        assert!(ctx.push_frame().is_ok());
        assert!(ctx.push_frame().is_ok());
        assert!(ctx.push_frame().is_err());
    }
    #[test]
    pub(super) fn ctfe_context_bind_lookup() {
        let mut ctx = CtfeContext::new();
        ctx.bind_local(vid(3), CtfeValue::Int(99));
        assert_eq!(ctx.lookup_local(vid(3)), Some(&CtfeValue::Int(99)));
        assert_eq!(ctx.lookup_local(vid(4)), None);
    }
    #[test]
    pub(super) fn ctfe_pass_empty_module() {
        let mut pass = CtfePass::default_pass();
        let mut decls: Vec<LcnfFunDecl> = vec![];
        pass.run(&mut decls);
        let r = pass.report();
        assert_eq!(r.functions_evaluated, 0);
        assert_eq!(r.calls_replaced, 0);
    }
    #[test]
    pub(super) fn ctfe_pass_evaluates_constant_function() {
        let decl = mk_const_decl("answer", ret_nat(42));
        let mut pass = CtfePass::default_pass();
        let mut decls = vec![decl];
        pass.run(&mut decls);
        let r = pass.report();
        assert_eq!(r.functions_evaluated, 1);
        assert!(pass.known_constants.contains_key("answer"));
        assert_eq!(pass.known_constants["answer"], CtfeValue::Int(42));
    }
    #[test]
    pub(super) fn ctfe_pass_skips_parameterised_function() {
        let decl = mk_decl_with_params(
            "id_fn",
            vec![mk_param(0, "x")],
            LcnfExpr::Return(LcnfArg::Var(vid(0))),
        );
        let mut pass = CtfePass::default_pass();
        let mut decls = vec![decl];
        pass.run(&mut decls);
        assert_eq!(pass.report().functions_evaluated, 0);
    }
    #[test]
    pub(super) fn ctfe_pass_report_display() {
        let r = CtfeReport {
            functions_evaluated: 5,
            calls_replaced: 12,
            constants_propagated: 8,
            fuel_exhausted_count: 1,
        };
        let s = r.to_string();
        assert!(s.contains("evaluated=5"));
        assert!(s.contains("replaced=12"));
    }
    #[test]
    pub(super) fn ctfe_config_default() {
        let cfg = CtfeConfig::default();
        assert_eq!(cfg.fuel, 10_000);
        assert_eq!(cfg.max_depth, 256);
        assert!(cfg.replace_calls);
    }
}
/// CTFE arithmetic operations
#[allow(dead_code)]
pub fn ctfe_arith_int(op: &str, a: i64, b: i64) -> Option<CtfeValueExt> {
    match op {
        "add" => a.checked_add(b).map(CtfeValueExt::Int),
        "sub" => a.checked_sub(b).map(CtfeValueExt::Int),
        "mul" => a.checked_mul(b).map(CtfeValueExt::Int),
        "div" => {
            if b != 0 {
                Some(CtfeValueExt::Int(a / b))
            } else {
                None
            }
        }
        "rem" => {
            if b != 0 {
                Some(CtfeValueExt::Int(a % b))
            } else {
                None
            }
        }
        "eq" => Some(CtfeValueExt::Bool(a == b)),
        "ne" => Some(CtfeValueExt::Bool(a != b)),
        "lt" => Some(CtfeValueExt::Bool(a < b)),
        "le" => Some(CtfeValueExt::Bool(a <= b)),
        "gt" => Some(CtfeValueExt::Bool(a > b)),
        "ge" => Some(CtfeValueExt::Bool(a >= b)),
        "and" => Some(CtfeValueExt::Int(a & b)),
        "or" => Some(CtfeValueExt::Int(a | b)),
        "xor" => Some(CtfeValueExt::Int(a ^ b)),
        "shl" => Some(CtfeValueExt::Int(a << (b & 63))),
        "shr" => Some(CtfeValueExt::Int(a >> (b & 63))),
        _ => None,
    }
}
#[allow(dead_code)]
pub fn ctfe_arith_bool(op: &str, a: bool, b: bool) -> Option<CtfeValueExt> {
    match op {
        "and" | "&&" => Some(CtfeValueExt::Bool(a && b)),
        "or" | "||" => Some(CtfeValueExt::Bool(a || b)),
        "eq" => Some(CtfeValueExt::Bool(a == b)),
        "ne" => Some(CtfeValueExt::Bool(a != b)),
        "xor" => Some(CtfeValueExt::Bool(a ^ b)),
        _ => None,
    }
}
/// CTFE value comparison (for equality / ordering)
#[allow(dead_code)]
pub fn ctfe_values_equal(a: &CtfeValueExt, b: &CtfeValueExt) -> bool {
    match (a, b) {
        (CtfeValueExt::Unit, CtfeValueExt::Unit) => true,
        (CtfeValueExt::Bool(x), CtfeValueExt::Bool(y)) => x == y,
        (CtfeValueExt::Int(x), CtfeValueExt::Int(y)) => x == y,
        (CtfeValueExt::Uint(x), CtfeValueExt::Uint(y)) => x == y,
        (CtfeValueExt::Str(x), CtfeValueExt::Str(y)) => x == y,
        (CtfeValueExt::Tuple(xs), CtfeValueExt::Tuple(ys)) => {
            xs.len() == ys.len()
                && xs
                    .iter()
                    .zip(ys.iter())
                    .all(|(a, b)| ctfe_values_equal(a, b))
        }
        (CtfeValueExt::List(xs), CtfeValueExt::List(ys)) => {
            xs.len() == ys.len()
                && xs
                    .iter()
                    .zip(ys.iter())
                    .all(|(a, b)| ctfe_values_equal(a, b))
        }
        (CtfeValueExt::Constructor(na, va), CtfeValueExt::Constructor(nb, vb)) => {
            na == nb
                && va.len() == vb.len()
                && va
                    .iter()
                    .zip(vb.iter())
                    .all(|(a, b)| ctfe_values_equal(a, b))
        }
        _ => false,
    }
}
#[allow(dead_code)]
pub fn ctfe_value_type(v: &CtfeValueExt) -> CtfeType {
    match v {
        CtfeValueExt::Unit => CtfeType::Unit,
        CtfeValueExt::Bool(_) => CtfeType::Bool,
        CtfeValueExt::Int(_) => CtfeType::Int,
        CtfeValueExt::Uint(_) => CtfeType::Uint,
        CtfeValueExt::Float(_) => CtfeType::Float,
        CtfeValueExt::Str(_) => CtfeType::Str,
        CtfeValueExt::Tuple(vs) => CtfeType::Tuple(vs.iter().map(ctfe_value_type).collect()),
        CtfeValueExt::List(vs) => {
            if vs.is_empty() {
                CtfeType::List(Box::new(CtfeType::Unknown))
            } else {
                CtfeType::List(Box::new(ctfe_value_type(&vs[0])))
            }
        }
        CtfeValueExt::Constructor(n, _) => CtfeType::Named(n.clone()),
        CtfeValueExt::Closure { .. } => CtfeType::Named("Closure".to_string()),
        CtfeValueExt::Opaque => CtfeType::Unknown,
    }
}
/// CTFE inline heuristic
#[allow(dead_code)]
pub fn ctfe_should_inline(entry: &CtfeFuncEntry, depth: usize, fuel: u64) -> bool {
    if !entry.is_pure {
        return false;
    }
    if entry.is_recursive && depth > 3 {
        return false;
    }
    if fuel < 100 {
        return false;
    }
    true
}
/// CTFE default feature flags
#[allow(dead_code)]
pub fn ctfe_default_features() -> CtfeFeatureFlags {
    CtfeFeatureFlags {
        fold_arithmetic: true,
        fold_boolean: true,
        fold_string: true,
        partial_eval: false,
        memoize: true,
    }
}
/// CTFE version string
#[allow(dead_code)]
pub const CTFE_PASS_VERSION: &str = "1.0.0";
/// CTFE max inline depth
#[allow(dead_code)]
pub const CTFE_MAX_INLINE_DEPTH: usize = 32;
/// CTFE default strategy
#[allow(dead_code)]
pub const CTFE_DEFAULT_STRATEGY: &str = "cbv";
