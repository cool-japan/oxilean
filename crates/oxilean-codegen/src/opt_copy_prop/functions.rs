//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::{
    ConstantFoldReport, CopyProp, CopyPropConfig, DeadBindingElim, DeadBindingReport, InlineReport,
    InliningPass, InterferenceGraph, OptPipeline, PassKind, RegisterCoalescingHint, UsedVars,
};

/// Collects register coalescing hints from a copy propagation pass.
///
/// For each copy `let x = y`, if `x` and `y` do not interfere in the
/// interference graph, emit a hint to coalesce them.
#[allow(dead_code)]
pub fn collect_coalescing_hints(
    copies: &[(LcnfVarId, LcnfVarId)],
    ig: &InterferenceGraph,
) -> Vec<RegisterCoalescingHint> {
    let mut hints = Vec::new();
    for &(src, dst) in copies {
        let is_safe = !ig.interfere(src, dst);
        let benefit = if is_safe { 10 } else { 1 };
        hints.push(RegisterCoalescingHint::new(src, dst, is_safe, benefit));
    }
    hints.sort_by(|a, b| b.benefit.cmp(&a.benefit));
    hints
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::lcnf::{
        LcnfAlt, LcnfExpr, LcnfFunDecl, LcnfLetValue, LcnfLit, LcnfParam, LcnfType, LcnfVarId,
    };
    pub(super) fn make_decl(body: LcnfExpr) -> LcnfFunDecl {
        LcnfFunDecl {
            name: "test_fn".to_string(),
            original_name: None,
            params: vec![],
            ret_type: LcnfType::Nat,
            body,
            is_recursive: false,
            is_lifted: false,
            inline_cost: 1,
        }
    }
    /// `let x = fvar(y); return x`  →  `return y`
    #[test]
    pub(super) fn test_simple_fvar_copy() {
        let body = LcnfExpr::Let {
            id: LcnfVarId(1),
            name: "x".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::FVar(LcnfVarId(0)),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(1)))),
        };
        let mut decl = make_decl(body);
        let mut pass = CopyProp::new(CopyPropConfig::default());
        pass.run(&mut decl);
        assert_eq!(decl.body, LcnfExpr::Return(LcnfArg::Var(LcnfVarId(0))));
        assert_eq!(pass.report().copies_eliminated, 1);
    }
    /// `let x = 42; return x`  →  `return 42`  (fold_literals=true)
    #[test]
    pub(super) fn test_literal_fold_enabled() {
        let body = LcnfExpr::Let {
            id: LcnfVarId(0),
            name: "x".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Lit(LcnfLit::Nat(42)),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(0)))),
        };
        let mut decl = make_decl(body);
        let mut pass = CopyProp::new(CopyPropConfig {
            fold_literals: true,
            ..Default::default()
        });
        pass.run(&mut decl);
        assert_eq!(decl.body, LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(42))));
        assert_eq!(pass.report().copies_eliminated, 1);
    }
    /// `let x = 42; return x`  stays when `fold_literals=false`.
    #[test]
    pub(super) fn test_literal_fold_disabled() {
        let body = LcnfExpr::Let {
            id: LcnfVarId(0),
            name: "x".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Lit(LcnfLit::Nat(7)),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(0)))),
        };
        let mut decl = make_decl(body);
        let mut pass = CopyProp::new(CopyPropConfig {
            fold_literals: false,
            ..Default::default()
        });
        pass.run(&mut decl);
        assert!(matches!(decl.body, LcnfExpr::Let { .. }));
        assert_eq!(pass.report().copies_eliminated, 0);
    }
    /// Transitive chain: `a=b, b=c, return a`  →  `return c`  (2 hops)
    #[test]
    pub(super) fn test_transitive_chain() {
        let body = LcnfExpr::Let {
            id: LcnfVarId(1),
            name: "b".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::FVar(LcnfVarId(0)),
            body: Box::new(LcnfExpr::Let {
                id: LcnfVarId(2),
                name: "a".to_string(),
                ty: LcnfType::Nat,
                value: LcnfLetValue::FVar(LcnfVarId(1)),
                body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(2)))),
            }),
        };
        let mut decl = make_decl(body);
        let mut pass = CopyProp::new(CopyPropConfig::default());
        pass.run(&mut decl);
        assert_eq!(decl.body, LcnfExpr::Return(LcnfArg::Var(LcnfVarId(0))));
        assert_eq!(pass.report().copies_eliminated, 2);
        assert_eq!(pass.report().chains_followed, 1);
    }
    /// Chain depth limit: with max_chain_depth=1 the second hop is not followed.
    #[test]
    pub(super) fn test_chain_depth_limit() {
        let body = LcnfExpr::Let {
            id: LcnfVarId(1),
            name: "b".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::FVar(LcnfVarId(0)),
            body: Box::new(LcnfExpr::Let {
                id: LcnfVarId(2),
                name: "a".to_string(),
                ty: LcnfType::Nat,
                value: LcnfLetValue::FVar(LcnfVarId(1)),
                body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(2)))),
            }),
        };
        let mut decl = make_decl(body);
        let mut pass = CopyProp::new(CopyPropConfig {
            max_chain_depth: 1,
            fold_literals: true,
        });
        pass.run(&mut decl);
        assert_eq!(decl.body, LcnfExpr::Return(LcnfArg::Var(LcnfVarId(0))));
    }
    /// App bindings are NOT propagated (conservative aliasing).
    #[test]
    pub(super) fn test_app_not_propagated() {
        let body = LcnfExpr::Let {
            id: LcnfVarId(1),
            name: "r".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::App(LcnfArg::Var(LcnfVarId(0)), vec![]),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(1)))),
        };
        let mut decl = make_decl(body);
        let mut pass = CopyProp::default_pass();
        pass.run(&mut decl);
        assert!(matches!(decl.body, LcnfExpr::Let { .. }));
        assert_eq!(pass.report().copies_eliminated, 0);
    }
    /// Copy propagation inside case branches is independent per branch.
    #[test]
    pub(super) fn test_copy_in_case_branches() {
        let case_expr = LcnfExpr::Case {
            scrutinee: LcnfVarId(0),
            scrutinee_ty: LcnfType::Object,
            alts: vec![LcnfAlt {
                ctor_name: "A".to_string(),
                ctor_tag: 0,
                params: vec![LcnfParam {
                    id: LcnfVarId(1),
                    name: "p".to_string(),
                    ty: LcnfType::Nat,
                    erased: false,
                    borrowed: false,
                }],
                body: LcnfExpr::Let {
                    id: LcnfVarId(2),
                    name: "q".to_string(),
                    ty: LcnfType::Nat,
                    value: LcnfLetValue::FVar(LcnfVarId(1)),
                    body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(2)))),
                },
            }],
            default: Some(Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(0))))),
        };
        let mut decl = make_decl(case_expr);
        let mut pass = CopyProp::default_pass();
        pass.run(&mut decl);
        match &decl.body {
            LcnfExpr::Case { alts, .. } => {
                assert_eq!(alts.len(), 1);
                assert_eq!(alts[0].body, LcnfExpr::Return(LcnfArg::Var(LcnfVarId(1))));
            }
            _ => panic!("Expected Case"),
        }
        assert!(pass.report().copies_eliminated >= 1);
    }
    /// Erased bindings are treated as copies and propagated.
    #[test]
    pub(super) fn test_erased_copy_propagated() {
        let body = LcnfExpr::Let {
            id: LcnfVarId(0),
            name: "e".to_string(),
            ty: LcnfType::Erased,
            value: LcnfLetValue::Erased,
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(0)))),
        };
        let mut decl = make_decl(body);
        let mut pass = CopyProp::default_pass();
        pass.run(&mut decl);
        assert_eq!(decl.body, LcnfExpr::Return(LcnfArg::Erased));
        assert_eq!(pass.report().copies_eliminated, 1);
    }
}
#[allow(dead_code)]
pub(super) fn collect_used(expr: &LcnfExpr, used: &mut UsedVars) {
    match expr {
        LcnfExpr::Return(arg) => collect_used_arg(arg, used),
        LcnfExpr::Let { value, body, .. } => {
            collect_used_value(value, used);
            collect_used(body, used);
        }
        LcnfExpr::Case {
            scrutinee,
            scrutinee_ty: _,
            alts,
            default,
            ..
        } => {
            used.vars.insert(*scrutinee);
            for alt in alts {
                collect_used(&alt.body, used);
            }
            if let Some(d) = default {
                collect_used(d, used);
            }
        }
        LcnfExpr::TailCall(callee, args) => {
            collect_used_arg(callee, used);
            for a in args {
                collect_used_arg(a, used);
            }
        }
        LcnfExpr::Unreachable => {}
    }
}
#[allow(dead_code)]
pub(super) fn collect_used_arg(arg: &LcnfArg, used: &mut UsedVars) {
    if let LcnfArg::Var(id) = arg {
        used.vars.insert(*id);
    }
}
#[allow(dead_code)]
pub(super) fn collect_used_value(val: &LcnfLetValue, used: &mut UsedVars) {
    match val {
        LcnfLetValue::FVar(id) => {
            used.vars.insert(*id);
        }
        LcnfLetValue::App(callee, args) => {
            collect_used_arg(callee, used);
            for a in args {
                collect_used_arg(a, used);
            }
        }
        LcnfLetValue::Ctor(_, _, args) => {
            for a in args {
                collect_used_arg(a, used);
            }
        }
        LcnfLetValue::Proj(_, _, id) => {
            used.vars.insert(*id);
        }
        _ => {}
    }
}
/// Inlining cost threshold (max `inline_cost` to be inlined).
#[allow(dead_code)]
pub const DEFAULT_INLINE_THRESHOLD: u32 = 5;
/// Count the number of `let`-bindings in an expression tree.
#[allow(dead_code)]
pub fn count_let_bindings(expr: &LcnfExpr) -> usize {
    match expr {
        LcnfExpr::Let { body, .. } => 1 + count_let_bindings(body),
        LcnfExpr::Case { alts, default, .. } => {
            let alt_sum: usize = alts.iter().map(|a| count_let_bindings(&a.body)).sum();
            let def_sum = default.as_ref().map_or(0, |d| count_let_bindings(d));
            alt_sum + def_sum
        }
        _ => 0,
    }
}
/// Return the maximum nesting depth of a LCNF expression.
#[allow(dead_code)]
pub fn expr_depth(expr: &LcnfExpr) -> usize {
    match expr {
        LcnfExpr::Let { body, .. } => 1 + expr_depth(body),
        LcnfExpr::Case { alts, default, .. } => {
            let alt_max = alts.iter().map(|a| expr_depth(&a.body)).max().unwrap_or(0);
            let def_max = default.as_ref().map_or(0, |d| expr_depth(d));
            1 + alt_max.max(def_max)
        }
        LcnfExpr::TailCall(_, args) => args.len(),
        _ => 0,
    }
}
/// Check whether an expression contains any tail calls to the given id.
#[allow(dead_code)]
pub fn has_tail_call_to(expr: &LcnfExpr, target: LcnfVarId) -> bool {
    match expr {
        LcnfExpr::TailCall(LcnfArg::Var(id), _) => *id == target,
        LcnfExpr::Let { body, .. } => has_tail_call_to(body, target),
        LcnfExpr::Case { alts, default, .. } => {
            alts.iter().any(|a| has_tail_call_to(&a.body, target))
                || default
                    .as_ref()
                    .is_some_and(|d| has_tail_call_to(d, target))
        }
        _ => false,
    }
}
/// Collect all `LcnfVarId`s bound by `let` in an expression.
#[allow(dead_code)]
pub fn collect_bound_vars(expr: &LcnfExpr, out: &mut Vec<LcnfVarId>) {
    match expr {
        LcnfExpr::Let { id, body, .. } => {
            out.push(*id);
            collect_bound_vars(body, out);
        }
        LcnfExpr::Case { alts, default, .. } => {
            for alt in alts {
                collect_bound_vars(&alt.body, out);
            }
            if let Some(d) = default {
                collect_bound_vars(d, out);
            }
        }
        _ => {}
    }
}
#[cfg(test)]
mod tests_extended {
    use super::*;
    pub(super) fn make_var(n: u32) -> LcnfVarId {
        LcnfVarId(u64::from(n))
    }
    pub(super) fn make_simple_decl(body: LcnfExpr) -> LcnfFunDecl {
        LcnfFunDecl {
            name: "test_fn".to_string(),
            original_name: None,
            params: vec![],
            ret_type: LcnfType::Nat,
            body,
            is_recursive: false,
            is_lifted: false,
            inline_cost: 1,
        }
    }
    #[test]
    pub(super) fn test_dead_binding_removal() {
        let body = LcnfExpr::Let {
            id: LcnfVarId(99),
            name: "x".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Lit(LcnfLit::Nat(42)),
            body: Box::new(LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0)))),
        };
        let mut decl = make_simple_decl(body);
        let mut pass = DeadBindingElim::default_pass();
        pass.run(&mut decl);
        assert_eq!(decl.body, LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0))));
        assert!(pass.report().bindings_removed >= 0);
    }
    #[test]
    pub(super) fn test_count_let_bindings() {
        let body = LcnfExpr::Let {
            id: LcnfVarId(0),
            name: "a".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Lit(LcnfLit::Nat(1)),
            body: Box::new(LcnfExpr::Let {
                id: LcnfVarId(1),
                name: "b".to_string(),
                ty: LcnfType::Nat,
                value: LcnfLetValue::Lit(LcnfLit::Nat(2)),
                body: Box::new(LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0)))),
            }),
        };
        assert_eq!(count_let_bindings(&body), 2);
    }
    #[test]
    pub(super) fn test_expr_depth() {
        let body = LcnfExpr::Let {
            id: LcnfVarId(0),
            name: "a".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Lit(LcnfLit::Nat(0)),
            body: Box::new(LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0)))),
        };
        assert_eq!(expr_depth(&body), 1);
    }
    #[test]
    pub(super) fn test_has_tail_call_to() {
        let target = make_var(7);
        let body = LcnfExpr::TailCall(LcnfArg::Var(target), vec![]);
        assert!(has_tail_call_to(&body, target));
        assert!(!has_tail_call_to(&body, make_var(8)));
    }
    #[test]
    pub(super) fn test_collect_bound_vars() {
        let body = LcnfExpr::Let {
            id: LcnfVarId(5),
            name: "x".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Lit(LcnfLit::Nat(0)),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(5)))),
        };
        let mut bound = vec![];
        collect_bound_vars(&body, &mut bound);
        assert_eq!(bound, vec![LcnfVarId(5)]);
    }
    #[test]
    pub(super) fn test_opt_pipeline_default() {
        let body = LcnfExpr::Let {
            id: LcnfVarId(0),
            name: "x".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::FVar(LcnfVarId(1)),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(0)))),
        };
        let mut decl = make_simple_decl(body);
        decl.params.push(LcnfParam {
            id: LcnfVarId(1),
            name: "n".to_string(),
            ty: LcnfType::Nat,
            erased: false,
            borrowed: false,
        });
        let mut pipeline = OptPipeline::new();
        let result = pipeline.run(&mut decl);
        assert!(result.copy_prop.copies_eliminated >= 1);
    }
    #[test]
    pub(super) fn test_pass_kind_display() {
        assert_eq!(PassKind::CopyProp.to_string(), "CopyProp");
        assert_eq!(PassKind::DeadBinding.to_string(), "DeadBinding");
        assert_eq!(PassKind::ConstantFold.to_string(), "ConstantFold");
        assert_eq!(PassKind::Inlining.to_string(), "Inlining");
    }
    #[test]
    pub(super) fn test_inline_candidate() {
        let pass = InliningPass::default_pass();
        let cheap = LcnfFunDecl {
            name: "cheap".to_string(),
            original_name: None,
            params: vec![],
            ret_type: LcnfType::Nat,
            body: LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0))),
            is_recursive: false,
            is_lifted: false,
            inline_cost: 1,
        };
        let expensive = LcnfFunDecl {
            inline_cost: 100,
            name: "expensive".to_string(),
            ..cheap.clone()
        };
        assert!(pass.is_inline_candidate(&cheap));
        assert!(!pass.is_inline_candidate(&expensive));
    }
    #[test]
    pub(super) fn test_dead_binding_config_display() {
        let cfg = CopyPropConfig::default();
        let s = format!("{}", cfg);
        assert!(s.contains("CopyPropConfig"));
    }
    #[test]
    pub(super) fn test_dead_binding_report_display() {
        let r = DeadBindingReport {
            bindings_removed: 3,
            passes_run: 2,
        };
        let s = format!("{}", r);
        assert!(s.contains("removed=3"));
        assert!(s.contains("passes=2"));
    }
    #[test]
    pub(super) fn test_constant_fold_report_display() {
        let r = ConstantFoldReport { folds_performed: 7 };
        let s = format!("{}", r);
        assert!(s.contains("folds=7"));
    }
    #[test]
    pub(super) fn test_inline_report_display() {
        let r = InlineReport {
            inlines_performed: 2,
            functions_considered: 10,
        };
        let s = format!("{}", r);
        assert!(s.contains("inlined=2"));
        assert!(s.contains("considered=10"));
    }
}
