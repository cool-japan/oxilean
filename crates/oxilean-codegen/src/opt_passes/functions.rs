//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::{LcnfArg, LcnfExpr, LcnfFunDecl, LcnfLetValue, LcnfLit, LcnfVarId};
use std::collections::{HashMap, HashSet};

use super::types::{
    BetaReductionPass, ConstantFoldingPass, CopyPropagationPass, DeadCodeEliminationPass,
    ExprSizeEstimator, IdentityEliminationPass, InlineCostEstimator, OPAnalysisCache,
    OPConstantFoldingHelper, OPDepGraph, OPDominatorTree, OPLivenessInfo, OPPassConfig,
    OPPassPhase, OPPassRegistry, OPPassStats, OPWorklist, PassDependency, PassManager, PassStats,
    PgoHints, StrengthReductionPass, UnreachableCodeEliminationPass,
};
use std::fmt;

/// Trait for optimization passes that operate on LCNF function declarations.
pub trait OptPass: fmt::Debug {
    /// Human-readable name of this pass.
    fn name(&self) -> &str;
    /// Run the pass on a set of declarations, returning the number of changes made.
    fn run_pass(&mut self, decls: &mut [LcnfFunDecl]) -> usize;
    /// Whether this pass is enabled.
    fn is_enabled(&self) -> bool {
        true
    }
    /// Dependencies: names of passes that must run before this one.
    fn dependencies(&self) -> Vec<&str> {
        Vec::new()
    }
}
/// Replace all occurrences of variable `from` with `to` in `expr`.
pub fn substitute_var_in_expr(expr: &mut LcnfExpr, from: LcnfVarId, to: LcnfVarId) {
    let subst_arg = |a: &mut LcnfArg| {
        if let LcnfArg::Var(v) = a {
            if *v == from {
                *v = to;
            }
        }
    };
    let subst_value = |val: &mut LcnfLetValue| match val {
        LcnfLetValue::App(f, args) => {
            subst_arg(f);
            for a in args {
                subst_arg(a);
            }
        }
        LcnfLetValue::FVar(v) => {
            if *v == from {
                *v = to;
            }
        }
        LcnfLetValue::Ctor(_, _, args) | LcnfLetValue::Reuse(_, _, _, args) => {
            for a in args {
                subst_arg(a);
            }
        }
        LcnfLetValue::Proj(_, _, v) => {
            if *v == from {
                *v = to;
            }
        }
        LcnfLetValue::Reset(v) => {
            if *v == from {
                *v = to;
            }
        }
        LcnfLetValue::Lit(_) | LcnfLetValue::Erased => {}
    };
    match expr {
        LcnfExpr::Let { value, body, .. } => {
            subst_value(value);
            substitute_var_in_expr(body, from, to);
        }
        LcnfExpr::Case {
            scrutinee,
            alts,
            default,
            ..
        } => {
            if *scrutinee == from {
                *scrutinee = to;
            }
            for alt in alts.iter_mut() {
                substitute_var_in_expr(&mut alt.body, from, to);
            }
            if let Some(def) = default {
                substitute_var_in_expr(def, from, to);
            }
        }
        LcnfExpr::Return(a) => subst_arg(a),
        LcnfExpr::TailCall(f, args) => {
            subst_arg(f);
            for a in args {
                subst_arg(a);
            }
        }
        LcnfExpr::Unreachable => {}
    }
}
/// Run all optimization passes in sequence.
pub fn run_all_passes(_decls: &mut Vec<LcnfFunDecl>, pgo: Option<&PgoHints>) {
    let mut _dce = DeadCodeEliminationPass::new();
    let mut _cp = CopyPropagationPass::new();
    let mut _cf = ConstantFoldingPass::new();
    let mut _beta = BetaReductionPass::new();
    let mut _identity = IdentityEliminationPass::new();
    let mut _unreachable = UnreachableCodeEliminationPass::new();
    let _ = pgo;
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::lcnf::{LcnfLit, LcnfType};
    pub(super) fn vid(n: u64) -> LcnfVarId {
        LcnfVarId(n)
    }
    pub(super) fn mk_fun_decl(name: &str, body: LcnfExpr) -> LcnfFunDecl {
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
    pub(super) fn mk_let(id: u64, value: LcnfLetValue, body: LcnfExpr) -> LcnfExpr {
        LcnfExpr::Let {
            id: vid(id),
            name: format!("x{}", id),
            ty: LcnfType::Nat,
            value,
            body: Box::new(body),
        }
    }
    #[test]
    pub(super) fn test_constant_folding_pass() {
        let mut pass = ConstantFoldingPass::new();
        assert_eq!(pass.folds_performed, 0);
        assert_eq!(pass.try_fold_nat_op("add", 3, 4), Some(7));
        assert_eq!(pass.try_fold_nat_op("sub", 5, 3), Some(2));
        assert_eq!(pass.try_fold_nat_op("mul", 2, 6), Some(12));
        assert_eq!(pass.try_fold_nat_op("div", 10, 2), Some(5));
        assert_eq!(pass.try_fold_nat_op("div", 10, 0), None);
        assert_eq!(pass.try_fold_nat_op("mod", 10, 3), Some(1));
        assert_eq!(pass.try_fold_nat_op("mod", 10, 0), None);
        assert_eq!(pass.try_fold_nat_op("min", 3, 7), Some(3));
        assert_eq!(pass.try_fold_nat_op("max", 3, 7), Some(7));
        assert_eq!(pass.try_fold_nat_op("pow", 2, 10), Some(1024));
        assert_eq!(pass.try_fold_nat_op("and", 0xFF, 0x0F), Some(0x0F));
        assert_eq!(pass.try_fold_nat_op("or", 0xF0, 0x0F), Some(0xFF));
        assert_eq!(pass.try_fold_nat_op("xor", 0xFF, 0xFF), Some(0));
        assert_eq!(pass.try_fold_nat_op("shl", 1, 3), Some(8));
        assert_eq!(pass.try_fold_nat_op("shr", 16, 2), Some(4));
        assert_eq!(pass.try_fold_nat_op("unknown", 1, 2), None);
    }
    #[test]
    pub(super) fn test_constant_folding_bool_ops() {
        let pass = ConstantFoldingPass::new();
        assert_eq!(pass.try_fold_bool_op("and", true, false), Some(false));
        assert_eq!(pass.try_fold_bool_op("or", true, false), Some(true));
        assert_eq!(pass.try_fold_bool_op("xor", true, true), Some(false));
        assert_eq!(pass.try_fold_bool_op("eq", true, true), Some(true));
        assert_eq!(pass.try_fold_bool_op("ne", true, false), Some(true));
        assert_eq!(pass.try_fold_bool_op("bad", true, false), None);
    }
    #[test]
    pub(super) fn test_constant_folding_cmp_ops() {
        let pass = ConstantFoldingPass::new();
        assert_eq!(pass.try_fold_cmp("eq", 5, 5), Some(true));
        assert_eq!(pass.try_fold_cmp("ne", 5, 5), Some(false));
        assert_eq!(pass.try_fold_cmp("lt", 3, 5), Some(true));
        assert_eq!(pass.try_fold_cmp("le", 5, 5), Some(true));
        assert_eq!(pass.try_fold_cmp("gt", 5, 3), Some(true));
        assert_eq!(pass.try_fold_cmp("ge", 3, 5), Some(false));
        assert_eq!(pass.try_fold_cmp("bad", 1, 2), None);
    }
    #[test]
    pub(super) fn test_constant_folding_run() {
        let mut pass = ConstantFoldingPass::new();
        let body = LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(42)));
        let mut decls = vec![mk_fun_decl("f", body)];
        pass.run(&mut decls);
        assert_eq!(pass.folds_performed, 0);
    }
    #[test]
    pub(super) fn test_constant_folding_debug() {
        let pass = ConstantFoldingPass::new();
        let s = format!("{:?}", pass);
        assert!(s.contains("ConstantFoldingPass"));
    }
    #[test]
    pub(super) fn test_dead_code_elimination_pass() {
        let mut pass = DeadCodeEliminationPass::new();
        assert_eq!(pass.removed, 0);
        let body = mk_let(
            0,
            LcnfLetValue::Lit(LcnfLit::Nat(42)),
            mk_let(
                1,
                LcnfLetValue::Lit(LcnfLit::Nat(99)),
                LcnfExpr::Return(LcnfArg::Var(vid(1))),
            ),
        );
        let mut decls = vec![mk_fun_decl("f", body)];
        pass.run(&mut decls);
        assert!(pass.removed > 0, "expected dead let to be removed");
    }
    #[test]
    pub(super) fn test_dead_code_elimination_debug() {
        let pass = DeadCodeEliminationPass::new();
        let s = format!("{:?}", pass);
        assert!(s.contains("DeadCodeEliminationPass"));
    }
    #[test]
    pub(super) fn test_copy_propagation_pass() {
        let mut pass = CopyPropagationPass::new();
        assert_eq!(pass.substitutions, 0);
        let body = mk_let(
            1,
            LcnfLetValue::FVar(vid(0)),
            LcnfExpr::Return(LcnfArg::Var(vid(1))),
        );
        let mut decls = vec![mk_fun_decl("f", body)];
        pass.run(&mut decls);
        assert!(pass.substitutions > 0, "expected copy to be propagated");
    }
    #[test]
    pub(super) fn test_copy_propagation_debug() {
        let pass = CopyPropagationPass::new();
        let s = format!("{:?}", pass);
        assert!(s.contains("CopyPropagationPass"));
    }
    #[test]
    pub(super) fn test_beta_reduction_pass() {
        let mut pass = BetaReductionPass::new();
        assert_eq!(pass.reductions, 0);
        let body = LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0)));
        let mut decls = vec![mk_fun_decl("f", body)];
        pass.run(&mut decls);
        assert_eq!(pass.reductions, 0);
        let body2 = LcnfExpr::TailCall(LcnfArg::Lit(LcnfLit::Nat(0)), vec![]);
        let mut decls2 = vec![mk_fun_decl("g", body2)];
        pass.run(&mut decls2);
        assert_eq!(pass.reductions, 1);
    }
    #[test]
    pub(super) fn test_beta_reduction_debug() {
        let pass = BetaReductionPass::new();
        let s = format!("{:?}", pass);
        assert!(s.contains("BetaReductionPass"));
    }
    #[test]
    pub(super) fn test_identity_elimination() {
        let mut pass = IdentityEliminationPass::new();
        let body = mk_let(
            0,
            LcnfLetValue::FVar(vid(0)),
            LcnfExpr::Return(LcnfArg::Var(vid(0))),
        );
        let mut decls = vec![mk_fun_decl("f", body)];
        pass.run(&mut decls);
        assert_eq!(pass.eliminated, 1);
        assert!(matches!(decls[0].body, LcnfExpr::Return(_)));
    }
    #[test]
    pub(super) fn test_identity_elimination_no_self_ref() {
        let mut pass = IdentityEliminationPass::new();
        let body = mk_let(
            1,
            LcnfLetValue::FVar(vid(0)),
            LcnfExpr::Return(LcnfArg::Var(vid(1))),
        );
        let mut decls = vec![mk_fun_decl("f", body)];
        pass.run(&mut decls);
        assert_eq!(pass.eliminated, 0);
    }
    #[test]
    pub(super) fn test_strength_reduction_power_of_two() {
        assert!(StrengthReductionPass::is_power_of_two(1));
        assert!(StrengthReductionPass::is_power_of_two(2));
        assert!(StrengthReductionPass::is_power_of_two(4));
        assert!(StrengthReductionPass::is_power_of_two(1024));
        assert!(!StrengthReductionPass::is_power_of_two(0));
        assert!(!StrengthReductionPass::is_power_of_two(3));
        assert!(!StrengthReductionPass::is_power_of_two(6));
    }
    #[test]
    pub(super) fn test_strength_reduction_log2() {
        assert_eq!(StrengthReductionPass::log2_exact(1), Some(0));
        assert_eq!(StrengthReductionPass::log2_exact(2), Some(1));
        assert_eq!(StrengthReductionPass::log2_exact(8), Some(3));
        assert_eq!(StrengthReductionPass::log2_exact(1024), Some(10));
        assert_eq!(StrengthReductionPass::log2_exact(0), None);
        assert_eq!(StrengthReductionPass::log2_exact(3), None);
    }
    #[test]
    pub(super) fn test_strength_reduction_is_mask() {
        assert!(StrengthReductionPass::is_mask(1));
        assert!(StrengthReductionPass::is_mask(3));
        assert!(StrengthReductionPass::is_mask(7));
        assert!(StrengthReductionPass::is_mask(0xFF));
        assert!(!StrengthReductionPass::is_mask(0));
        assert!(!StrengthReductionPass::is_mask(5));
    }
    #[test]
    pub(super) fn test_strength_reduction_bit_ops() {
        assert_eq!(StrengthReductionPass::ctz(8), 3);
        assert_eq!(StrengthReductionPass::ctz(0), 64);
        assert_eq!(StrengthReductionPass::clz(1), 63);
        assert_eq!(StrengthReductionPass::popcount(0xFF), 8);
        assert_eq!(StrengthReductionPass::popcount(0), 0);
    }
    #[test]
    pub(super) fn test_unreachable_code_elimination() {
        let mut pass = UnreachableCodeEliminationPass::new();
        let body = mk_let(
            0,
            LcnfLetValue::Lit(LcnfLit::Nat(42)),
            LcnfExpr::Unreachable,
        );
        let mut decls = vec![mk_fun_decl("f", body)];
        pass.run(&mut decls);
        assert_eq!(pass.eliminated, 1);
        assert!(matches!(decls[0].body, LcnfExpr::Unreachable));
    }
    #[test]
    pub(super) fn test_unreachable_nested() {
        let mut pass = UnreachableCodeEliminationPass::new();
        let body = mk_let(
            0,
            LcnfLetValue::Lit(LcnfLit::Nat(1)),
            mk_let(1, LcnfLetValue::Lit(LcnfLit::Nat(2)), LcnfExpr::Unreachable),
        );
        let mut decls = vec![mk_fun_decl("f", body)];
        pass.run(&mut decls);
        assert!(pass.eliminated >= 2);
    }
    #[test]
    pub(super) fn test_expr_size_count_lets() {
        let body = mk_let(
            0,
            LcnfLetValue::Lit(LcnfLit::Nat(1)),
            mk_let(
                1,
                LcnfLetValue::Lit(LcnfLit::Nat(2)),
                LcnfExpr::Return(LcnfArg::Var(vid(1))),
            ),
        );
        assert_eq!(ExprSizeEstimator::count_lets(&body), 2);
    }
    #[test]
    pub(super) fn test_expr_size_count_cases() {
        let body = LcnfExpr::Case {
            scrutinee: vid(0),
            scrutinee_ty: LcnfType::Nat,
            alts: vec![],
            default: Some(Box::new(LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0))))),
        };
        assert_eq!(ExprSizeEstimator::count_cases(&body), 1);
    }
    #[test]
    pub(super) fn test_expr_size_complexity() {
        let body = mk_let(
            0,
            LcnfLetValue::Lit(LcnfLit::Nat(1)),
            LcnfExpr::Return(LcnfArg::Var(vid(0))),
        );
        assert_eq!(ExprSizeEstimator::complexity(&body), 1);
    }
    #[test]
    pub(super) fn test_expr_size_max_depth() {
        let body = mk_let(
            0,
            LcnfLetValue::Lit(LcnfLit::Nat(1)),
            mk_let(
                1,
                LcnfLetValue::Lit(LcnfLit::Nat(2)),
                LcnfExpr::Return(LcnfArg::Var(vid(1))),
            ),
        );
        assert_eq!(ExprSizeEstimator::max_depth(&body), 2);
    }
    #[test]
    pub(super) fn test_expr_size_is_trivial() {
        assert!(ExprSizeEstimator::is_trivial(&LcnfExpr::Return(
            LcnfArg::Lit(LcnfLit::Nat(0))
        )));
        assert!(ExprSizeEstimator::is_trivial(&LcnfExpr::Unreachable));
        assert!(!ExprSizeEstimator::is_trivial(&mk_let(
            0,
            LcnfLetValue::Lit(LcnfLit::Nat(0)),
            LcnfExpr::Unreachable
        )));
    }
    #[test]
    pub(super) fn test_expr_size_should_inline() {
        let small = LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0)));
        assert!(ExprSizeEstimator::should_inline(&small, 5));
        let big = mk_let(
            0,
            LcnfLetValue::Lit(LcnfLit::Nat(1)),
            mk_let(
                1,
                LcnfLetValue::Lit(LcnfLit::Nat(2)),
                mk_let(
                    2,
                    LcnfLetValue::Lit(LcnfLit::Nat(3)),
                    mk_let(
                        3,
                        LcnfLetValue::Lit(LcnfLit::Nat(4)),
                        mk_let(
                            4,
                            LcnfLetValue::Lit(LcnfLit::Nat(5)),
                            mk_let(
                                5,
                                LcnfLetValue::Lit(LcnfLit::Nat(6)),
                                LcnfExpr::Return(LcnfArg::Var(vid(5))),
                            ),
                        ),
                    ),
                ),
            ),
        );
        assert!(!ExprSizeEstimator::should_inline(&big, 3));
    }
    #[test]
    pub(super) fn test_expr_size_var_refs() {
        let body = mk_let(
            1,
            LcnfLetValue::FVar(vid(0)),
            LcnfExpr::Return(LcnfArg::Var(vid(1))),
        );
        assert_eq!(ExprSizeEstimator::count_var_refs(&body), 2);
    }
    #[test]
    pub(super) fn test_pgo_hints() {
        let mut hints = PgoHints::new();
        assert!(!hints.is_hot("foo"));
        assert!(!hints.should_inline("foo"));
        hints.mark_hot("foo");
        hints.mark_hot("bar");
        hints.mark_hot("foo");
        assert!(hints.is_hot("foo"));
        assert!(hints.is_hot("bar"));
        assert_eq!(hints.hot_functions.len(), 2);
        hints.mark_inline("baz");
        assert!(hints.should_inline("baz"));
        assert!(!hints.should_inline("qux"));
    }
    #[test]
    pub(super) fn test_pgo_hints_cold() {
        let mut hints = PgoHints::new();
        hints.mark_cold("cold_fn");
        assert!(hints.is_cold("cold_fn"));
        assert!(!hints.is_cold("other"));
    }
    #[test]
    pub(super) fn test_pgo_hints_total() {
        let mut hints = PgoHints::new();
        hints.mark_hot("a");
        hints.mark_cold("b");
        hints.mark_inline("c");
        hints.record_call("d", 10);
        assert_eq!(hints.total_hints(), 4);
    }
    #[test]
    pub(super) fn test_pgo_hints_classify() {
        let mut hints = PgoHints::new();
        hints.mark_hot("h");
        hints.mark_cold("c");
        assert_eq!(hints.classify("h"), "hot");
        assert_eq!(hints.classify("c"), "cold");
        assert_eq!(hints.classify("other"), "normal");
    }
    #[test]
    pub(super) fn test_pgo_hints_merge() {
        let mut h1 = PgoHints::new();
        h1.mark_hot("a");
        h1.record_call("f", 5);
        let mut h2 = PgoHints::new();
        h2.mark_hot("b");
        h2.mark_cold("c");
        h2.record_call("f", 3);
        h1.merge(&h2);
        assert!(h1.is_hot("a"));
        assert!(h1.is_hot("b"));
        assert!(h1.is_cold("c"));
        assert_eq!(h1.call_count("f"), 8);
    }
    #[test]
    pub(super) fn test_pgo_hints_call_count() {
        let mut hints = PgoHints::new();
        hints.record_call("f", 10);
        hints.record_call("f", 5);
        assert_eq!(hints.call_count("f"), 15);
        assert_eq!(hints.call_count("g"), 0);
    }
    #[test]
    pub(super) fn test_inline_cost_estimator_trivial() {
        let est = InlineCostEstimator::default();
        let decl = mk_fun_decl("f", LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0))));
        assert!(est.should_inline(&decl, None));
    }
    #[test]
    pub(super) fn test_inline_cost_estimator_with_pgo() {
        let est = InlineCostEstimator::default();
        let body = mk_let(
            0,
            LcnfLetValue::Lit(LcnfLit::Nat(1)),
            mk_let(
                1,
                LcnfLetValue::Lit(LcnfLit::Nat(2)),
                mk_let(
                    2,
                    LcnfLetValue::Lit(LcnfLit::Nat(3)),
                    mk_let(
                        3,
                        LcnfLetValue::Lit(LcnfLit::Nat(4)),
                        LcnfExpr::Return(LcnfArg::Var(vid(3))),
                    ),
                ),
            ),
        );
        let decl = mk_fun_decl("medium", body);
        let mut pgo = PgoHints::new();
        pgo.mark_inline("medium");
        assert!(est.should_inline(&decl, Some(&pgo)));
    }
    #[test]
    pub(super) fn test_inline_cost_recursive_penalty() {
        let est = InlineCostEstimator::default();
        let mut decl = mk_fun_decl("rec", LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0))));
        decl.is_recursive = true;
        let cost = est.cost(&decl);
        assert_eq!(cost, 10);
    }
    #[test]
    pub(super) fn test_pass_manager_new() {
        let pm = PassManager::new();
        assert_eq!(pm.num_passes(), 0);
        assert_eq!(pm.max_iterations, 10);
    }
    #[test]
    pub(super) fn test_pass_manager_add_pass() {
        let mut pm = PassManager::new();
        pm.add_pass("dce");
        pm.add_pass("cp");
        pm.add_pass("dce");
        assert_eq!(pm.num_passes(), 2);
    }
    #[test]
    pub(super) fn test_pass_manager_record_run() {
        let mut pm = PassManager::new();
        pm.add_pass("dce");
        pm.record_run("dce", 5, 100);
        let stats = pm.get_stats("dce").expect("stats should exist");
        assert_eq!(stats.run_count, 1);
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_pass_manager_topological_order() {
        let mut pm = PassManager::new();
        pm.add_pass("beta");
        pm.add_pass("dce");
        pm.add_pass("cp");
        pm.add_dependency("dce", "cp");
        pm.add_dependency("cp", "beta");
        let order = pm.topological_order().expect("no cycle");
        let beta_pos = order
            .iter()
            .position(|n| n == "beta")
            .expect("beta_pos position should exist");
        let cp_pos = order
            .iter()
            .position(|n| n == "cp")
            .expect("cp_pos position should exist");
        let dce_pos = order
            .iter()
            .position(|n| n == "dce")
            .expect("dce_pos position should exist");
        assert!(beta_pos < cp_pos);
        assert!(cp_pos < dce_pos);
    }
    #[test]
    pub(super) fn test_pass_manager_cycle_detection() {
        let mut pm = PassManager::new();
        pm.add_pass("a");
        pm.add_pass("b");
        pm.add_dependency("a", "b");
        pm.add_dependency("b", "a");
        assert!(pm.has_cycle());
        assert!(pm.topological_order().is_none());
    }
    #[test]
    pub(super) fn test_pass_manager_no_cycle() {
        let mut pm = PassManager::new();
        pm.add_pass("a");
        pm.add_pass("b");
        pm.add_dependency("b", "a");
        assert!(!pm.has_cycle());
    }
    #[test]
    pub(super) fn test_pass_manager_total_changes() {
        let mut pm = PassManager::new();
        pm.add_pass("a");
        pm.add_pass("b");
        pm.record_run("a", 3, 0);
        pm.record_run("b", 7, 0);
        assert_eq!(pm.total_changes(), 10);
        assert_eq!(pm.total_runs(), 2);
    }
    #[test]
    pub(super) fn test_pass_stats_display() {
        let mut stats = PassStats::new("test_pass");
        stats.record_run(5, 100);
        stats.record_run(3, 50);
        let s = format!("{}", stats);
        assert!(s.contains("test_pass"));
        assert!(s.contains("runs=2"));
        assert!(s.contains("changes=8"));
    }
    #[test]
    pub(super) fn test_pass_stats_avg() {
        let mut stats = PassStats::new("avg_test");
        stats.record_run(10, 0);
        stats.record_run(20, 0);
        assert!((stats.avg_changes() - 15.0).abs() < 0.001);
    }
    #[test]
    pub(super) fn test_pass_stats_empty_avg() {
        let stats = PassStats::new("empty");
        assert_eq!(stats.avg_changes(), 0.0);
    }
    #[test]
    pub(super) fn test_pass_dependency_display() {
        let dep = PassDependency::new("b", "a");
        assert_eq!(format!("{}", dep), "a -> b");
    }
    #[test]
    pub(super) fn test_substitute_var_in_return() {
        let mut expr = LcnfExpr::Return(LcnfArg::Var(vid(1)));
        substitute_var_in_expr(&mut expr, vid(1), vid(2));
        assert_eq!(expr, LcnfExpr::Return(LcnfArg::Var(vid(2))));
    }
    #[test]
    pub(super) fn test_substitute_var_in_tailcall() {
        let mut expr = LcnfExpr::TailCall(
            LcnfArg::Var(vid(1)),
            vec![LcnfArg::Var(vid(1)), LcnfArg::Lit(LcnfLit::Nat(0))],
        );
        substitute_var_in_expr(&mut expr, vid(1), vid(2));
        if let LcnfExpr::TailCall(f, args) = &expr {
            assert_eq!(*f, LcnfArg::Var(vid(2)));
            assert_eq!(args[0], LcnfArg::Var(vid(2)));
        }
    }
    #[test]
    pub(super) fn test_substitute_var_in_case() {
        let mut expr = LcnfExpr::Case {
            scrutinee: vid(1),
            scrutinee_ty: LcnfType::Nat,
            alts: vec![],
            default: Some(Box::new(LcnfExpr::Return(LcnfArg::Var(vid(1))))),
        };
        substitute_var_in_expr(&mut expr, vid(1), vid(2));
        if let LcnfExpr::Case {
            scrutinee, default, ..
        } = &expr
        {
            assert_eq!(*scrutinee, vid(2));
            assert_eq!(
                **default.as_ref().expect("expected Some/Ok value"),
                LcnfExpr::Return(LcnfArg::Var(vid(2)))
            );
        }
    }
    #[test]
    pub(super) fn test_run_all_passes() {
        let mut hints = PgoHints::new();
        hints.mark_hot("main");
        let body = LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0)));
        let mut decls = vec![mk_fun_decl("main", body)];
        run_all_passes(&mut decls, Some(&hints));
        run_all_passes(&mut decls, None);
    }
    #[test]
    pub(super) fn test_opt_pass_trait_constant_folding() {
        let mut pass = ConstantFoldingPass::new();
        assert_eq!(pass.name(), "constant_folding");
        assert!(pass.is_enabled());
        assert!(pass.dependencies().is_empty());
        let body = LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0)));
        let mut decls = vec![mk_fun_decl("f", body)];
        let changes = pass.run_pass(&mut decls);
        assert_eq!(changes, 0);
    }
    #[test]
    pub(super) fn test_opt_pass_trait_dce() {
        let mut pass = DeadCodeEliminationPass::new();
        assert_eq!(pass.name(), "dead_code_elimination");
    }
    #[test]
    pub(super) fn test_opt_pass_trait_cp() {
        let mut pass = CopyPropagationPass::new();
        assert_eq!(pass.name(), "copy_propagation");
    }
    #[test]
    pub(super) fn test_opt_pass_trait_beta() {
        let mut pass = BetaReductionPass::new();
        assert_eq!(pass.name(), "beta_reduction");
    }
    #[test]
    pub(super) fn test_opt_pass_trait_identity() {
        let mut pass = IdentityEliminationPass::new();
        assert_eq!(pass.name(), "identity_elimination");
    }
    #[test]
    pub(super) fn test_opt_pass_trait_unreachable() {
        let mut pass = UnreachableCodeEliminationPass::new();
        assert_eq!(pass.name(), "unreachable_code_elimination");
    }
    #[test]
    pub(super) fn test_pass_debug_impls() {
        let cf = ConstantFoldingPass::new();
        let dce = DeadCodeEliminationPass::new();
        let cp = CopyPropagationPass::new();
        let beta = BetaReductionPass::new();
        let id = IdentityEliminationPass::new();
        let sr = StrengthReductionPass::new();
        let uce = UnreachableCodeEliminationPass::new();
        assert!(format!("{:?}", cf).contains("ConstantFolding"));
        assert!(format!("{:?}", dce).contains("DeadCode"));
        assert!(format!("{:?}", cp).contains("CopyPropagation"));
        assert!(format!("{:?}", beta).contains("BetaReduction"));
        assert!(format!("{:?}", id).contains("Identity"));
        assert!(format!("{:?}", sr).contains("StrengthReduction"));
        assert!(format!("{:?}", uce).contains("Unreachable"));
    }
}
#[cfg(test)]
mod OP_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = OPPassConfig::new("test_pass", OPPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = OPPassStats::new();
        stats.record_run(10, 100, 3);
        stats.record_run(20, 200, 5);
        assert_eq!(stats.total_runs, 2);
        assert!((stats.average_changes_per_run() - 15.0).abs() < 0.01);
        assert!((stats.success_rate() - 1.0).abs() < 0.01);
        let s = stats.format_summary();
        assert!(s.contains("Runs: 2/2"));
    }
    #[test]
    pub(super) fn test_pass_registry() {
        let mut reg = OPPassRegistry::new();
        reg.register(OPPassConfig::new("pass_a", OPPassPhase::Analysis));
        reg.register(OPPassConfig::new("pass_b", OPPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = OPAnalysisCache::new(10);
        cache.insert("key1".to_string(), vec![1, 2, 3]);
        assert!(cache.get("key1").is_some());
        assert!(cache.get("key2").is_none());
        assert!((cache.hit_rate() - 0.5).abs() < 0.01);
        cache.invalidate("key1");
        assert!(!cache.entries["key1"].valid);
        assert_eq!(cache.size(), 1);
    }
    #[test]
    pub(super) fn test_worklist() {
        let mut wl = OPWorklist::new();
        assert!(wl.push(1));
        assert!(wl.push(2));
        assert!(!wl.push(1));
        assert_eq!(wl.len(), 2);
        assert_eq!(wl.pop(), Some(1));
        assert!(!wl.contains(1));
        assert!(wl.contains(2));
    }
    #[test]
    pub(super) fn test_dominator_tree() {
        let mut dt = OPDominatorTree::new(5);
        dt.set_idom(1, 0);
        dt.set_idom(2, 0);
        dt.set_idom(3, 1);
        assert!(dt.dominates(0, 3));
        assert!(dt.dominates(1, 3));
        assert!(!dt.dominates(2, 3));
        assert!(dt.dominates(3, 3));
    }
    #[test]
    pub(super) fn test_liveness() {
        let mut liveness = OPLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(OPConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(OPConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(OPConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            OPConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(OPConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = OPDepGraph::new();
        g.add_dep(1, 2);
        g.add_dep(2, 3);
        g.add_dep(1, 3);
        assert_eq!(g.dependencies_of(2), vec![1]);
        let topo = g.topological_sort();
        assert_eq!(topo.len(), 3);
        assert!(!g.has_cycle());
        let pos: std::collections::HashMap<u32, usize> =
            topo.iter().enumerate().map(|(i, &n)| (n, i)).collect();
        assert!(pos[&1] < pos[&2]);
        assert!(pos[&1] < pos[&3]);
        assert!(pos[&2] < pos[&3]);
    }
}
