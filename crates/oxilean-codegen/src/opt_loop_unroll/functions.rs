//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::{LcnfArg, LcnfExpr, LcnfFunDecl, LcnfLetValue, LcnfLit, LcnfVarId};
use std::collections::HashMap;

use super::types::{
    LUAnalysisCache, LUConstantFoldingHelper, LUDepGraph, LUDominatorTree, LULivenessInfo,
    LUPassConfig, LUPassPhase, LUPassRegistry, LUPassStats, LUWorklist, LoopInfo, LoopUnrollPass,
    UnrollCandidate, UnrollConfig, UnrollFactor, UnrollReport,
};

/// Count the number of distinct variable references in an expression.
pub fn count_var_refs(expr: &LcnfExpr, target: LcnfVarId) -> usize {
    match expr {
        LcnfExpr::Let {
            id, value, body, ..
        } => {
            let in_value = count_var_refs_in_value(value, target);
            let in_body = if *id == target {
                0
            } else {
                count_var_refs(body, target)
            };
            in_value + in_body
        }
        LcnfExpr::Case {
            scrutinee,
            alts,
            default,
            ..
        } => {
            let scrutinee_count = if *scrutinee == target { 1 } else { 0 };
            let alt_count: usize = alts
                .iter()
                .filter(|a| a.params.iter().all(|p| p.id != target))
                .map(|a| count_var_refs(&a.body, target))
                .sum();
            let default_count = default
                .as_ref()
                .map(|d| count_var_refs(d, target))
                .unwrap_or(0);
            scrutinee_count + alt_count + default_count
        }
        LcnfExpr::Return(arg) | LcnfExpr::TailCall(arg, _) => {
            if let crate::lcnf::LcnfArg::Var(id) = arg {
                if *id == target {
                    1
                } else {
                    0
                }
            } else {
                0
            }
        }
        LcnfExpr::Unreachable => 0,
    }
}
pub(super) fn count_var_refs_in_value(value: &LcnfLetValue, target: LcnfVarId) -> usize {
    let count_arg = |a: &crate::lcnf::LcnfArg| {
        matches!(a, crate ::lcnf::LcnfArg::Var(id) if * id == target) as usize
    };
    match value {
        LcnfLetValue::App(f, args) => count_arg(f) + args.iter().map(count_arg).sum::<usize>(),
        LcnfLetValue::Proj(_, _, v) => {
            if *v == target {
                1
            } else {
                0
            }
        }
        LcnfLetValue::Ctor(_, _, args) => args.iter().map(count_arg).sum(),
        LcnfLetValue::FVar(id) => {
            if *id == target {
                1
            } else {
                0
            }
        }
        LcnfLetValue::Reset(v) => {
            if *v == target {
                1
            } else {
                0
            }
        }
        LcnfLetValue::Reuse(slot, _, _, args) => {
            let s = if *slot == target { 1 } else { 0 };
            s + args.iter().map(count_arg).sum::<usize>()
        }
        LcnfLetValue::Lit(_) | LcnfLetValue::Erased => 0,
    }
}
/// Estimate the abstract instruction count of an LCNF expression.
pub fn estimate_expr_size(expr: &LcnfExpr) -> u64 {
    match expr {
        LcnfExpr::Let { body, .. } => 1 + estimate_expr_size(body),
        LcnfExpr::Case { alts, default, .. } => {
            let alt_sizes: u64 = alts.iter().map(|a| estimate_expr_size(&a.body)).sum();
            let def_size = default.as_ref().map(|d| estimate_expr_size(d)).unwrap_or(0);
            1 + alt_sizes + def_size
        }
        LcnfExpr::Return(_) | LcnfExpr::TailCall(_, _) | LcnfExpr::Unreachable => 1,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::lcnf::{
        LcnfAlt, LcnfArg, LcnfExpr, LcnfFunDecl, LcnfLetValue, LcnfLit, LcnfParam, LcnfType,
        LcnfVarId,
    };
    pub(super) fn make_nat_lit(id: u64, n: u64, body: LcnfExpr) -> LcnfExpr {
        LcnfExpr::Let {
            id: LcnfVarId(id),
            name: format!("v{}", id),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Lit(LcnfLit::Nat(n)),
            body: Box::new(body),
        }
    }
    pub(super) fn make_return_nat(n: u64) -> LcnfExpr {
        LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(n)))
    }
    pub(super) fn make_decl(name: &str, body: LcnfExpr) -> LcnfFunDecl {
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
    #[test]
    pub(super) fn test_unroll_factor_full_has_no_numeric_factor() {
        assert_eq!(UnrollFactor::Full.factor(), None);
    }
    #[test]
    pub(super) fn test_unroll_factor_partial_returns_factor() {
        assert_eq!(UnrollFactor::Partial(4).factor(), Some(4));
    }
    #[test]
    pub(super) fn test_unroll_factor_jamming_has_no_numeric_factor() {
        assert_eq!(UnrollFactor::Jamming.factor(), None);
    }
    #[test]
    pub(super) fn test_unroll_factor_vectorizable_returns_factor() {
        assert_eq!(UnrollFactor::Vectorizable(8).factor(), Some(8));
    }
    #[test]
    pub(super) fn test_unroll_factor_names() {
        assert_eq!(UnrollFactor::Full.name(), "full");
        assert_eq!(UnrollFactor::Partial(2).name(), "partial");
        assert_eq!(UnrollFactor::Jamming.name(), "jamming");
        assert_eq!(UnrollFactor::Vectorizable(4).name(), "vectorizable");
    }
    #[test]
    pub(super) fn test_loop_info_trip_count_basic() {
        let info = LoopInfo::new(LcnfVarId(0), 0, 8, 1, vec![]);
        assert_eq!(info.trip_count, Some(8));
    }
    #[test]
    pub(super) fn test_loop_info_trip_count_step2() {
        let info = LoopInfo::new(LcnfVarId(0), 0, 8, 2, vec![]);
        assert_eq!(info.trip_count, Some(4));
    }
    #[test]
    pub(super) fn test_loop_info_trip_count_non_zero_start() {
        let info = LoopInfo::new(LcnfVarId(0), 3, 15, 3, vec![]);
        assert_eq!(info.trip_count, Some(4));
    }
    #[test]
    pub(super) fn test_loop_info_is_counted_when_trip_known() {
        let info = LoopInfo::new(LcnfVarId(0), 0, 4, 1, vec![]);
        assert!(info.is_counted);
    }
    #[test]
    pub(super) fn test_loop_info_priority_score_innermost_bonus() {
        let mut info = LoopInfo::new(LcnfVarId(0), 0, 8, 1, vec![]);
        info.is_innermost = true;
        let score_inner = info.priority_score();
        let mut info2 = LoopInfo::new(LcnfVarId(0), 0, 8, 1, vec![]);
        info2.is_innermost = false;
        let score_outer = info2.priority_score();
        assert!(score_inner > score_outer);
    }
    #[test]
    pub(super) fn test_default_config_values() {
        let cfg = UnrollConfig::default();
        assert_eq!(cfg.max_unroll_factor, 8);
        assert_eq!(cfg.max_unrolled_size, 256);
        assert_eq!(cfg.unroll_full_threshold, 16);
        assert!(cfg.enable_vectorizable);
    }
    #[test]
    pub(super) fn test_aggressive_config_larger_limits() {
        let agg = UnrollConfig::aggressive();
        let def = UnrollConfig::default();
        assert!(agg.max_unroll_factor >= def.max_unroll_factor);
        assert!(agg.max_unrolled_size >= def.max_unrolled_size);
    }
    #[test]
    pub(super) fn test_conservative_config_smaller_limits() {
        let con = UnrollConfig::conservative();
        let def = UnrollConfig::default();
        assert!(con.max_unroll_factor <= def.max_unroll_factor);
        assert!(!con.enable_vectorizable);
    }
    #[test]
    pub(super) fn test_compute_factor_full_for_small_trip() {
        let pass = LoopUnrollPass::default_pass();
        let info = LoopInfo::new(LcnfVarId(0), 0, 4, 1, vec![]);
        assert_eq!(pass.compute_unroll_factor(&info), UnrollFactor::Full);
    }
    #[test]
    pub(super) fn test_compute_factor_partial_for_medium_trip() {
        let pass = LoopUnrollPass::default_pass();
        let mut info = LoopInfo::new(LcnfVarId(0), 0, 32, 1, vec![]);
        info.estimated_size = 10;
        let factor = pass.compute_unroll_factor(&info);
        assert_ne!(factor, UnrollFactor::Full);
    }
    #[test]
    pub(super) fn test_compute_factor_vectorizable_for_divisible_trip() {
        let mut cfg = UnrollConfig::default();
        cfg.enable_vectorizable = true;
        let pass = LoopUnrollPass::new(cfg);
        let mut info = LoopInfo::new(LcnfVarId(0), 0, 32, 1, vec![]);
        info.estimated_size = 5;
        info.is_innermost = true;
        let factor = pass.compute_unroll_factor(&info);
        assert!(matches!(factor, UnrollFactor::Vectorizable(_)));
    }
    #[test]
    pub(super) fn test_compute_factor_unknown_trip_gives_partial2() {
        let pass = LoopUnrollPass::default_pass();
        let info = LoopInfo {
            loop_var: LcnfVarId(0),
            start: 0,
            end: 0,
            step: 0,
            body: vec![],
            trip_count: None,
            is_innermost: true,
            is_counted: false,
            estimated_size: 10,
        };
        assert_eq!(pass.compute_unroll_factor(&info), UnrollFactor::Partial(2));
    }
    #[test]
    pub(super) fn test_unroll_loop_partial_2_doubles_body() {
        let mut pass = LoopUnrollPass::default_pass();
        let body = vec![make_return_nat(0), make_return_nat(1)];
        let result = pass.unroll_loop(&body, &UnrollFactor::Partial(2));
        assert_eq!(result.len(), body.len() * 2);
    }
    #[test]
    pub(super) fn test_unroll_loop_partial_4_quadruples_body() {
        let mut pass = LoopUnrollPass::default_pass();
        let body = vec![make_return_nat(42)];
        let result = pass.unroll_loop(&body, &UnrollFactor::Partial(4));
        assert_eq!(result.len(), 4);
    }
    #[test]
    pub(super) fn test_unroll_loop_jamming_returns_unchanged() {
        let mut pass = LoopUnrollPass::default_pass();
        let body = vec![make_return_nat(7)];
        let result = pass.unroll_loop(&body, &UnrollFactor::Jamming);
        assert_eq!(result.len(), body.len());
    }
    #[test]
    pub(super) fn test_unroll_loop_vectorizable_replicates() {
        let mut pass = LoopUnrollPass::default_pass();
        let body = vec![make_return_nat(0)];
        let result = pass.unroll_loop(&body, &UnrollFactor::Vectorizable(4));
        assert_eq!(result.len(), 4);
    }
    #[test]
    pub(super) fn test_run_empty_decls() {
        let mut pass = LoopUnrollPass::default_pass();
        let mut decls: Vec<LcnfFunDecl> = vec![];
        pass.run(&mut decls);
        assert_eq!(pass.report().loops_analyzed, 0);
    }
    #[test]
    pub(super) fn test_run_simple_decl_no_loops() {
        let mut pass = LoopUnrollPass::default_pass();
        let decl = make_decl("foo", make_return_nat(0));
        let mut decls = vec![decl];
        pass.run(&mut decls);
        assert_eq!(pass.report().loops_analyzed, 0);
    }
    #[test]
    pub(super) fn test_run_preserves_decl_count() {
        let mut pass = LoopUnrollPass::default_pass();
        let d1 = make_decl("f1", make_return_nat(1));
        let d2 = make_decl("f2", make_return_nat(2));
        let mut decls = vec![d1, d2];
        pass.run(&mut decls);
        assert_eq!(decls.len(), 2);
    }
    #[test]
    pub(super) fn test_report_merge() {
        let mut r1 = UnrollReport {
            loops_analyzed: 3,
            loops_unrolled: 2,
            full_unrolls: 1,
            partial_unrolls: 1,
            jammed_loops: 0,
            vectorizable_loops: 0,
            estimated_speedup: 1.5,
        };
        let r2 = UnrollReport {
            loops_analyzed: 7,
            loops_unrolled: 4,
            full_unrolls: 2,
            partial_unrolls: 2,
            jammed_loops: 2,
            vectorizable_loops: 0,
            estimated_speedup: 2.0,
        };
        r1.merge(&r2);
        assert_eq!(r1.loops_analyzed, 10);
        assert_eq!(r1.loops_unrolled, 6);
        assert_eq!(r1.jammed_loops, 2);
    }
    #[test]
    pub(super) fn test_report_summary_contains_key_fields() {
        let r = UnrollReport {
            loops_analyzed: 5,
            loops_unrolled: 3,
            full_unrolls: 1,
            partial_unrolls: 2,
            jammed_loops: 0,
            vectorizable_loops: 0,
            estimated_speedup: 1.8,
        };
        let s = r.summary();
        assert!(s.contains("analyzed=5"));
        assert!(s.contains("unrolled=3"));
    }
    #[test]
    pub(super) fn test_estimate_size_return_is_1() {
        assert_eq!(estimate_expr_size(&make_return_nat(0)), 1);
    }
    #[test]
    pub(super) fn test_estimate_size_let_adds_1() {
        let e = make_nat_lit(0, 42, make_return_nat(0));
        assert_eq!(estimate_expr_size(&e), 2);
    }
    #[test]
    pub(super) fn test_estimate_size_chain() {
        let e = make_nat_lit(0, 1, make_nat_lit(1, 2, make_return_nat(0)));
        assert_eq!(estimate_expr_size(&e), 3);
    }
    #[test]
    pub(super) fn test_count_var_refs_return() {
        let e = LcnfExpr::Return(LcnfArg::Var(LcnfVarId(5)));
        assert_eq!(count_var_refs(&e, LcnfVarId(5)), 1);
        assert_eq!(count_var_refs(&e, LcnfVarId(6)), 0);
    }
    #[test]
    pub(super) fn test_count_var_refs_in_let_value() {
        let e = LcnfExpr::Let {
            id: LcnfVarId(1),
            name: "x".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::FVar(LcnfVarId(5)),
            body: Box::new(make_return_nat(0)),
        };
        assert_eq!(count_var_refs(&e, LcnfVarId(5)), 1);
    }
    #[test]
    pub(super) fn test_count_var_refs_shadowed() {
        let e = LcnfExpr::Let {
            id: LcnfVarId(5),
            name: "x".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Erased,
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(5)))),
        };
        assert_eq!(count_var_refs(&e, LcnfVarId(5)), 0);
    }
    #[test]
    pub(super) fn test_candidate_is_profitable_positive_savings() {
        let info = LoopInfo::new(LcnfVarId(0), 0, 4, 1, vec![]);
        let c = UnrollCandidate::new("f", info, UnrollFactor::Full, 10);
        assert!(c.is_profitable());
    }
    #[test]
    pub(super) fn test_candidate_is_not_profitable_negative_savings() {
        let info = LoopInfo::new(LcnfVarId(0), 0, 4, 1, vec![]);
        let c = UnrollCandidate::new("f", info, UnrollFactor::Full, -5);
        assert!(!c.is_profitable());
    }
    #[test]
    pub(super) fn test_case_expr_size() {
        let case_expr = LcnfExpr::Case {
            scrutinee: LcnfVarId(0),
            scrutinee_ty: LcnfType::Nat,
            alts: vec![
                LcnfAlt {
                    ctor_name: "zero".to_string(),
                    ctor_tag: 0,
                    params: vec![],
                    body: make_return_nat(0),
                },
                LcnfAlt {
                    ctor_name: "succ".to_string(),
                    ctor_tag: 1,
                    params: vec![LcnfParam {
                        id: LcnfVarId(1),
                        name: "n".to_string(),
                        ty: LcnfType::Nat,
                        erased: false,
                        borrowed: false,
                    }],
                    body: make_return_nat(1),
                },
            ],
            default: None,
        };
        assert_eq!(estimate_expr_size(&case_expr), 3);
    }
}
/// Trait for a single loop optimization pass in the pipeline.
#[allow(dead_code)]
pub trait LoopOptPass {
    /// Name of this pass.
    fn name(&self) -> &str;
    /// Run this pass on the given function declarations.
    fn run_pass(&mut self, decls: &mut [LcnfFunDecl]) -> UnrollReport;
}
#[cfg(test)]
mod LU_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = LUPassConfig::new("test_pass", LUPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = LUPassStats::new();
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
        let mut reg = LUPassRegistry::new();
        reg.register(LUPassConfig::new("pass_a", LUPassPhase::Analysis));
        reg.register(LUPassConfig::new("pass_b", LUPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = LUAnalysisCache::new(10);
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
        let mut wl = LUWorklist::new();
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
        let mut dt = LUDominatorTree::new(5);
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
        let mut liveness = LULivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(LUConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(LUConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(LUConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            LUConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(LUConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = LUDepGraph::new();
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
