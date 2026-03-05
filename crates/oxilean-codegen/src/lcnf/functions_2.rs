//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};

use super::types::{
    CostModel, LcnfAlt, LcnfArg, LcnfBuilder, LcnfExpr, LcnfFunDecl, LcnfLetValue, LcnfLit,
    LcnfModule, LcnfParam, LcnfType, LcnfVarId, PrettyConfig, Substitution, ValidationError,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lcnf::*;
    #[test]
    pub(super) fn test_lcnf_var_id_display() {
        let id = LcnfVarId(42);
        assert_eq!(id.to_string(), "_x42");
    }
    #[test]
    pub(super) fn test_lcnf_type_display() {
        assert_eq!(LcnfType::Erased.to_string(), "erased");
        assert_eq!(LcnfType::Nat.to_string(), "nat");
        assert_eq!(LcnfType::Unit.to_string(), "unit");
        assert_eq!(LcnfType::Object.to_string(), "object");
    }
    #[test]
    pub(super) fn test_lcnf_lit_display() {
        assert_eq!(LcnfLit::Nat(42).to_string(), "42");
        assert_eq!(LcnfLit::Str("hello".into()).to_string(), "\"hello\"");
    }
    #[test]
    pub(super) fn test_lcnf_arg_display() {
        assert_eq!(LcnfArg::Var(LcnfVarId(0)).to_string(), "_x0");
        assert_eq!(LcnfArg::Lit(LcnfLit::Nat(1)).to_string(), "1");
        assert_eq!(LcnfArg::Erased.to_string(), "erased");
    }
    #[test]
    pub(super) fn test_lcnf_fun_type_display() {
        let ty = LcnfType::Fun(vec![LcnfType::Nat, LcnfType::Nat], Box::new(LcnfType::Nat));
        assert_eq!(ty.to_string(), "(nat, nat) -> nat");
    }
    #[test]
    pub(super) fn test_lcnf_ctor_type_display() {
        let ty = LcnfType::Ctor("List".into(), vec![LcnfType::Nat]);
        assert_eq!(ty.to_string(), "List<nat>");
    }
    #[test]
    pub(super) fn test_lcnf_return_display() {
        let expr = LcnfExpr::Return(LcnfArg::Var(LcnfVarId(0)));
        assert_eq!(expr.to_string(), "return _x0");
    }
    #[test]
    pub(super) fn test_lcnf_module_default() {
        let module = LcnfModule::default();
        assert!(module.fun_decls.is_empty());
        assert!(module.extern_decls.is_empty());
    }
    pub(super) fn make_simple_let_return() -> LcnfExpr {
        LcnfExpr::Let {
            id: LcnfVarId(0),
            name: "a".into(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Lit(LcnfLit::Nat(42)),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(0)))),
        }
    }
    #[test]
    pub(super) fn test_free_vars_no_free() {
        let expr = make_simple_let_return();
        let fv = free_vars(&expr);
        assert!(fv.is_empty());
    }
    #[test]
    pub(super) fn test_free_vars_with_free() {
        let expr = LcnfExpr::Return(LcnfArg::Var(LcnfVarId(5)));
        let fv = free_vars(&expr);
        assert!(fv.contains(&LcnfVarId(5)));
        assert_eq!(fv.len(), 1);
    }
    #[test]
    pub(super) fn test_bound_vars() {
        let expr = make_simple_let_return();
        let bv = bound_vars(&expr);
        assert!(bv.contains(&LcnfVarId(0)));
        assert_eq!(bv.len(), 1);
    }
    #[test]
    pub(super) fn test_all_vars() {
        let expr = LcnfExpr::Let {
            id: LcnfVarId(0),
            name: "a".into(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::FVar(LcnfVarId(5)),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(0)))),
        };
        let av = all_vars(&expr);
        assert!(av.contains(&LcnfVarId(0)));
        assert!(av.contains(&LcnfVarId(5)));
    }
    #[test]
    pub(super) fn test_usage_counts() {
        let expr = LcnfExpr::Let {
            id: LcnfVarId(0),
            name: "a".into(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Lit(LcnfLit::Nat(42)),
            body: Box::new(LcnfExpr::Let {
                id: LcnfVarId(1),
                name: "b".into(),
                ty: LcnfType::Nat,
                value: LcnfLetValue::App(
                    LcnfArg::Var(LcnfVarId(10)),
                    vec![LcnfArg::Var(LcnfVarId(0)), LcnfArg::Var(LcnfVarId(0))],
                ),
                body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(1)))),
            }),
        };
        let counts = usage_counts(&expr);
        assert_eq!(counts.get(&LcnfVarId(0)).copied().unwrap_or(0), 2);
        assert_eq!(counts.get(&LcnfVarId(1)).copied().unwrap_or(0), 1);
        assert_eq!(counts.get(&LcnfVarId(10)).copied().unwrap_or(0), 1);
    }
    #[test]
    pub(super) fn test_is_linear() {
        let expr = make_simple_let_return();
        assert!(is_linear(&expr));
    }
    #[test]
    pub(super) fn test_substitution_basic() {
        let mut subst = Substitution::new();
        assert!(subst.is_empty());
        subst.insert(LcnfVarId(0), LcnfArg::Var(LcnfVarId(1)));
        assert!(!subst.is_empty());
        assert!(subst.contains(&LcnfVarId(0)));
        assert!(!subst.contains(&LcnfVarId(2)));
    }
    #[test]
    pub(super) fn test_substitute_expr_return() {
        let expr = LcnfExpr::Return(LcnfArg::Var(LcnfVarId(0)));
        let mut subst = Substitution::new();
        subst.insert(LcnfVarId(0), LcnfArg::Var(LcnfVarId(99)));
        let result = substitute_expr(&expr, &subst);
        assert_eq!(result, LcnfExpr::Return(LcnfArg::Var(LcnfVarId(99))));
    }
    #[test]
    pub(super) fn test_substitute_shadowed() {
        let expr = make_simple_let_return();
        let mut subst = Substitution::new();
        subst.insert(LcnfVarId(0), LcnfArg::Var(LcnfVarId(99)));
        let result = substitute_expr(&expr, &subst);
        match &result {
            LcnfExpr::Let { body, .. } => {
                assert_eq!(*body.as_ref(), LcnfExpr::Return(LcnfArg::Var(LcnfVarId(0))));
            }
            _ => panic!("Expected Let"),
        }
    }
    #[test]
    pub(super) fn test_alpha_equiv_same() {
        let expr = make_simple_let_return();
        assert!(alpha_equiv(&expr, &expr));
    }
    #[test]
    pub(super) fn test_alpha_equiv_renamed() {
        let e1 = LcnfExpr::Let {
            id: LcnfVarId(0),
            name: "a".into(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Lit(LcnfLit::Nat(42)),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(0)))),
        };
        let e2 = LcnfExpr::Let {
            id: LcnfVarId(7),
            name: "b".into(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Lit(LcnfLit::Nat(42)),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(7)))),
        };
        assert!(alpha_equiv(&e1, &e2));
    }
    #[test]
    pub(super) fn test_alpha_equiv_different() {
        let e1 = LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(1)));
        let e2 = LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(2)));
        assert!(!alpha_equiv(&e1, &e2));
    }
    #[test]
    pub(super) fn test_expr_size_return() {
        let expr = LcnfExpr::Return(LcnfArg::Var(LcnfVarId(0)));
        assert_eq!(expr_size(&expr), 1);
    }
    #[test]
    pub(super) fn test_expr_size_let() {
        let expr = make_simple_let_return();
        assert_eq!(expr_size(&expr), 3);
    }
    #[test]
    pub(super) fn test_expr_depth() {
        let expr = make_simple_let_return();
        assert_eq!(expr_depth(&expr), 2);
    }
    #[test]
    pub(super) fn test_count_branches_none() {
        let expr = make_simple_let_return();
        assert_eq!(count_branches(&expr), 0);
    }
    #[test]
    pub(super) fn test_count_allocations() {
        let expr = LcnfExpr::Let {
            id: LcnfVarId(0),
            name: "n".into(),
            ty: LcnfType::Ctor("List".into(), vec![LcnfType::Nat]),
            value: LcnfLetValue::Ctor("Nil".into(), 0, vec![]),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(0)))),
        };
        assert_eq!(count_allocations(&expr), 0);
        let expr2 = LcnfExpr::Let {
            id: LcnfVarId(1),
            name: "c".into(),
            ty: LcnfType::Ctor("List".into(), vec![LcnfType::Nat]),
            value: LcnfLetValue::Ctor(
                "Cons".into(),
                1,
                vec![LcnfArg::Var(LcnfVarId(0)), LcnfArg::Var(LcnfVarId(0))],
            ),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(1)))),
        };
        assert_eq!(count_allocations(&expr2), 1);
    }
    #[test]
    pub(super) fn test_validate_ok() {
        let expr = make_simple_let_return();
        assert!(validate_expr(&expr, &HashSet::new()).is_ok());
    }
    #[test]
    pub(super) fn test_validate_unbound() {
        let expr = LcnfExpr::Return(LcnfArg::Var(LcnfVarId(99)));
        let result = validate_expr(&expr, &HashSet::new());
        assert_eq!(result, Err(ValidationError::UnboundVariable(LcnfVarId(99))));
    }
    #[test]
    pub(super) fn test_validate_empty_case() {
        let mut bound = HashSet::new();
        bound.insert(LcnfVarId(0));
        let expr = LcnfExpr::Case {
            scrutinee: LcnfVarId(0),
            scrutinee_ty: LcnfType::Nat,
            alts: vec![],
            default: None,
        };
        assert_eq!(
            validate_expr(&expr, &bound),
            Err(ValidationError::EmptyCase)
        );
    }
    #[test]
    pub(super) fn test_validate_fun_decl_ok() {
        let decl = LcnfFunDecl {
            name: "f".into(),
            original_name: None,
            params: vec![LcnfParam {
                id: LcnfVarId(0),
                name: "x".into(),
                ty: LcnfType::Nat,
                erased: false,
                borrowed: false,
            }],
            ret_type: LcnfType::Nat,
            body: LcnfExpr::Return(LcnfArg::Var(LcnfVarId(0))),
            is_recursive: false,
            is_lifted: false,
            inline_cost: 0,
        };
        assert!(validate_fun_decl(&decl).is_ok());
    }
    #[test]
    pub(super) fn test_validate_module_ok() {
        let module = LcnfModule::default();
        assert!(validate_module(&module).is_ok());
    }
    #[test]
    pub(super) fn test_check_anf_invariant() {
        let expr = make_simple_let_return();
        assert!(check_anf_invariant(&expr));
    }
    #[test]
    pub(super) fn test_builder_return() {
        let mut builder = LcnfBuilder::new();
        let x = builder.let_bind("x", LcnfType::Nat, LcnfLetValue::Lit(LcnfLit::Nat(42)));
        let expr = builder.build_return(LcnfArg::Var(x));
        match &expr {
            LcnfExpr::Let { id, body, .. } => {
                assert_eq!(*id, LcnfVarId(0));
                assert_eq!(**body, LcnfExpr::Return(LcnfArg::Var(LcnfVarId(0))));
            }
            _ => panic!("Expected Let"),
        }
        assert!(validate_expr(&expr, &HashSet::new()).is_ok());
    }
    #[test]
    pub(super) fn test_builder_multiple_lets() {
        let mut builder = LcnfBuilder::new();
        let x = builder.let_bind("x", LcnfType::Nat, LcnfLetValue::Lit(LcnfLit::Nat(1)));
        let y = builder.let_app(
            "y",
            LcnfType::Nat,
            LcnfArg::Var(LcnfVarId(100)),
            vec![LcnfArg::Var(x)],
        );
        let expr = builder.build_return(LcnfArg::Var(y));
        assert_eq!(expr_depth(&expr), 3);
    }
    #[test]
    pub(super) fn test_builder_ctor() {
        let mut builder = LcnfBuilder::new();
        let v = builder.let_ctor(
            "nil",
            LcnfType::Ctor("List".into(), vec![]),
            "Nil",
            0,
            vec![],
        );
        let expr = builder.build_return(LcnfArg::Var(v));
        assert!(check_anf_invariant(&expr));
    }
    #[test]
    pub(super) fn test_builder_tail_call() {
        let mut builder = LcnfBuilder::new();
        let x = builder.let_bind("x", LcnfType::Nat, LcnfLetValue::Lit(LcnfLit::Nat(1)));
        let expr = builder.build_tail_call(LcnfArg::Var(LcnfVarId(100)), vec![LcnfArg::Var(x)]);
        match &expr {
            LcnfExpr::Let { body, .. } => {
                assert!(matches!(**body, LcnfExpr::TailCall(..)));
            }
            _ => panic!("Expected Let wrapping TailCall"),
        }
    }
    #[test]
    pub(super) fn test_pretty_print_return() {
        let expr = LcnfExpr::Return(LcnfArg::Var(LcnfVarId(0)));
        let config = PrettyConfig::default();
        let output = pretty_print_expr(&expr, &config);
        assert!(output.contains("return _x0"));
    }
    #[test]
    pub(super) fn test_pretty_print_let() {
        let expr = make_simple_let_return();
        let config = PrettyConfig::default();
        let output = pretty_print_expr(&expr, &config);
        assert!(output.contains("let _x0"));
        assert!(output.contains(": nat"));
        assert!(output.contains("return _x0"));
    }
    #[test]
    pub(super) fn test_pretty_print_fun_decl() {
        let decl = LcnfFunDecl {
            name: "f".into(),
            original_name: None,
            params: vec![LcnfParam {
                id: LcnfVarId(0),
                name: "x".into(),
                ty: LcnfType::Nat,
                erased: false,
                borrowed: false,
            }],
            ret_type: LcnfType::Nat,
            body: LcnfExpr::Return(LcnfArg::Var(LcnfVarId(0))),
            is_recursive: false,
            is_lifted: false,
            inline_cost: 0,
        };
        let config = PrettyConfig::default();
        let output = pretty_print_fun_decl(&decl, &config);
        assert!(output.contains("def f("));
        assert!(output.contains("return _x0"));
    }
    #[test]
    pub(super) fn test_pretty_print_module() {
        let module = LcnfModule {
            name: "test".into(),
            ..Default::default()
        };
        let config = PrettyConfig::default();
        let output = pretty_print_module(&module, &config);
        assert!(output.contains("-- module test"));
    }
    #[test]
    pub(super) fn test_inline_let_literal() {
        let expr = make_simple_let_return();
        let result = inline_let(expr, LcnfVarId(0));
        assert_eq!(result, LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(42))));
    }
    #[test]
    pub(super) fn test_inline_let_non_target() {
        let expr = make_simple_let_return();
        let result = inline_let(expr.clone(), LcnfVarId(99));
        assert_eq!(result, expr);
    }
    #[test]
    pub(super) fn test_remove_unused_lets() {
        let expr = LcnfExpr::Let {
            id: LcnfVarId(0),
            name: "unused".into(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Lit(LcnfLit::Nat(42)),
            body: Box::new(LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0)))),
        };
        let result = remove_unused_lets(expr);
        assert_eq!(result, LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0))));
    }
    #[test]
    pub(super) fn test_remove_unused_keeps_used() {
        let expr = make_simple_let_return();
        let result = remove_unused_lets(expr.clone());
        assert_eq!(result, expr);
    }
    #[test]
    pub(super) fn test_flatten_lets() {
        let expr = make_simple_let_return();
        let result = flatten_lets(expr.clone());
        assert_eq!(result, expr);
    }
    #[test]
    pub(super) fn test_simplify_trivial_case() {
        let expr = LcnfExpr::Case {
            scrutinee: LcnfVarId(0),
            scrutinee_ty: LcnfType::Ctor("Pair".into(), vec![]),
            alts: vec![LcnfAlt {
                ctor_name: "Pair".into(),
                ctor_tag: 0,
                params: vec![
                    LcnfParam {
                        id: LcnfVarId(1),
                        name: "a".into(),
                        ty: LcnfType::Nat,
                        erased: false,
                        borrowed: false,
                    },
                    LcnfParam {
                        id: LcnfVarId(2),
                        name: "b".into(),
                        ty: LcnfType::Nat,
                        erased: false,
                        borrowed: false,
                    },
                ],
                body: LcnfExpr::Return(LcnfArg::Var(LcnfVarId(1))),
            }],
            default: None,
        };
        let result = simplify_trivial_case(expr);
        match &result {
            LcnfExpr::Let {
                id, value, body, ..
            } => {
                assert_eq!(id, &LcnfVarId(1));
                assert!(matches!(value, LcnfLetValue::Proj(n, 0, _) if n == "Pair"));
                match body.as_ref() {
                    LcnfExpr::Let {
                        id: id2,
                        value: val2,
                        body: body2,
                        ..
                    } => {
                        assert_eq!(id2, &LcnfVarId(2));
                        assert!(matches!(val2, LcnfLetValue::Proj(n, 1, _) if n == "Pair"));
                        assert_eq!(
                            body2.as_ref(),
                            &LcnfExpr::Return(LcnfArg::Var(LcnfVarId(1)))
                        );
                    }
                    _ => panic!("Expected inner Let"),
                }
            }
            _ => panic!("Expected Let from simplify_trivial_case"),
        }
    }
    #[test]
    pub(super) fn test_definition_sites() {
        let expr = make_simple_let_return();
        let sites = definition_sites(&expr);
        assert_eq!(sites.len(), 1);
        assert_eq!(sites[0].var, LcnfVarId(0));
        assert_eq!(sites[0].depth, 0);
    }
    #[test]
    pub(super) fn test_estimate_runtime_cost() {
        let expr = make_simple_let_return();
        let model = CostModel::default();
        let cost = estimate_runtime_cost(&expr, &model);
        assert_eq!(cost, 2);
    }
}
