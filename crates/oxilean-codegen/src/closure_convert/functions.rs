//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::{HashMap, HashSet};

use super::types::{
    CCAnalysisCache, CCConstantFoldingHelper, CCDepGraph, CCDominatorTree, CCExtCache,
    CCExtConstFolder, CCExtDepGraph, CCExtDomTree, CCExtLiveness, CCExtPassConfig, CCExtPassPhase,
    CCExtPassRegistry, CCExtPassStats, CCExtWorklist, CCLivenessInfo, CCPassConfig, CCPassPhase,
    CCPassRegistry, CCPassStats, CCWorklist, CCX2Cache, CCX2ConstFolder, CCX2DepGraph, CCX2DomTree,
    CCX2Liveness, CCX2PassConfig, CCX2PassPhase, CCX2PassRegistry, CCX2PassStats, CCX2Worklist,
    ClosureConvertConfig, ClosureConvertStats, ClosureConverter, ClosureInfo, EscapeAnalysis,
    EscapeInfo,
};

/// Compute the set of free variables in an LCNF expression.
///
/// A variable is "free" if it is referenced but not bound within the expression.
pub fn compute_free_vars(expr: &LcnfExpr, bound: &HashSet<LcnfVarId>) -> HashSet<LcnfVarId> {
    let mut free = HashSet::new();
    collect_free_vars(expr, bound, &mut free);
    free
}
/// Recursive helper for free variable collection.
pub(super) fn collect_free_vars(
    expr: &LcnfExpr,
    bound: &HashSet<LcnfVarId>,
    free: &mut HashSet<LcnfVarId>,
) {
    match expr {
        LcnfExpr::Let {
            id, value, body, ..
        } => {
            collect_free_vars_value(value, bound, free);
            let mut new_bound = bound.clone();
            new_bound.insert(*id);
            collect_free_vars(body, &new_bound, free);
        }
        LcnfExpr::Case {
            scrutinee,
            alts,
            default,
            ..
        } => {
            if !bound.contains(scrutinee) {
                free.insert(*scrutinee);
            }
            for alt in alts {
                let mut alt_bound = bound.clone();
                for param in &alt.params {
                    alt_bound.insert(param.id);
                }
                collect_free_vars(&alt.body, &alt_bound, free);
            }
            if let Some(def) = default {
                collect_free_vars(def, bound, free);
            }
        }
        LcnfExpr::Return(arg) => {
            collect_free_vars_arg(arg, bound, free);
        }
        LcnfExpr::TailCall(func, args) => {
            collect_free_vars_arg(func, bound, free);
            for arg in args {
                collect_free_vars_arg(arg, bound, free);
            }
        }
        LcnfExpr::Unreachable => {}
    }
}
/// Collect free variables from a let-value.
pub(super) fn collect_free_vars_value(
    value: &LcnfLetValue,
    bound: &HashSet<LcnfVarId>,
    free: &mut HashSet<LcnfVarId>,
) {
    match value {
        LcnfLetValue::App(func, args) => {
            collect_free_vars_arg(func, bound, free);
            for arg in args {
                collect_free_vars_arg(arg, bound, free);
            }
        }
        LcnfLetValue::Ctor(_, _, args) => {
            for arg in args {
                collect_free_vars_arg(arg, bound, free);
            }
        }
        LcnfLetValue::Proj(_, _, var) => {
            if !bound.contains(var) {
                free.insert(*var);
            }
        }
        LcnfLetValue::FVar(var) => {
            if !bound.contains(var) {
                free.insert(*var);
            }
        }
        LcnfLetValue::Lit(_)
        | LcnfLetValue::Erased
        | LcnfLetValue::Reset(_)
        | LcnfLetValue::Reuse(_, _, _, _) => {}
    }
}
/// Collect free variables from an argument.
pub(super) fn collect_free_vars_arg(
    arg: &LcnfArg,
    bound: &HashSet<LcnfVarId>,
    free: &mut HashSet<LcnfVarId>,
) {
    if let LcnfArg::Var(v) = arg {
        if !bound.contains(v) {
            free.insert(*v);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
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
    pub(super) fn mk_fun_decl(name: &str, body: LcnfExpr) -> LcnfFunDecl {
        LcnfFunDecl {
            name: name.to_string(),
            original_name: None,
            params: vec![mk_param(0, "x")],
            ret_type: LcnfType::Nat,
            body,
            is_recursive: false,
            is_lifted: false,
            inline_cost: 1,
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
    pub(super) fn mk_module(decls: Vec<LcnfFunDecl>) -> LcnfModule {
        LcnfModule {
            fun_decls: decls,
            extern_decls: vec![],
            name: "test_mod".to_string(),
            metadata: LcnfModuleMetadata::default(),
        }
    }
    #[test]
    pub(super) fn test_escape_info_merge() {
        assert_eq!(
            EscapeInfo::NoEscape.merge(EscapeInfo::NoEscape),
            EscapeInfo::NoEscape
        );
        assert_eq!(
            EscapeInfo::NoEscape.merge(EscapeInfo::LocalEscape),
            EscapeInfo::LocalEscape
        );
        assert_eq!(
            EscapeInfo::LocalEscape.merge(EscapeInfo::GlobalEscape),
            EscapeInfo::GlobalEscape
        );
        assert_eq!(
            EscapeInfo::GlobalEscape.merge(EscapeInfo::NoEscape),
            EscapeInfo::GlobalEscape
        );
    }
    #[test]
    pub(super) fn test_escape_info_requires_heap() {
        assert!(!EscapeInfo::NoEscape.requires_heap());
        assert!(!EscapeInfo::LocalEscape.requires_heap());
        assert!(EscapeInfo::GlobalEscape.requires_heap());
    }
    #[test]
    pub(super) fn test_escape_info_display() {
        assert_eq!(EscapeInfo::NoEscape.to_string(), "no-escape");
        assert_eq!(EscapeInfo::LocalEscape.to_string(), "local-escape");
        assert_eq!(EscapeInfo::GlobalEscape.to_string(), "global-escape");
    }
    #[test]
    pub(super) fn test_closure_info_can_stack_allocate() {
        let info = ClosureInfo {
            free_vars: vec![vid(1), vid(2)],
            captured_types: vec![LcnfType::Nat, LcnfType::Nat],
            arity: 1,
            is_escaping: false,
            has_side_effects: false,
            original_name: None,
        };
        assert!(info.can_stack_allocate());
        let escaping = ClosureInfo {
            is_escaping: true,
            ..info.clone()
        };
        assert!(!escaping.can_stack_allocate());
    }
    #[test]
    pub(super) fn test_closure_info_total_fields() {
        let info = ClosureInfo {
            free_vars: vec![vid(1), vid(2), vid(3)],
            captured_types: vec![],
            arity: 2,
            is_escaping: false,
            has_side_effects: false,
            original_name: None,
        };
        assert_eq!(info.total_fields(), 4);
    }
    #[test]
    pub(super) fn test_closure_info_display() {
        let info = ClosureInfo {
            free_vars: vec![vid(1)],
            captured_types: vec![LcnfType::Nat],
            arity: 2,
            is_escaping: false,
            has_side_effects: true,
            original_name: None,
        };
        let s = info.to_string();
        assert!(s.contains("arity=2"));
        assert!(s.contains("captured=1"));
        assert!(s.contains("side_effects=true"));
    }
    #[test]
    pub(super) fn test_escape_analysis_simple_return() {
        let body = LcnfExpr::Return(LcnfArg::Var(vid(0)));
        let decl = mk_fun_decl("f", body);
        let module = mk_module(vec![decl]);
        let mut analysis = EscapeAnalysis::new();
        analysis.analyze(&module);
        assert_eq!(analysis.get_var_escape(vid(0)), EscapeInfo::GlobalEscape);
    }
    #[test]
    pub(super) fn test_escape_analysis_let_no_escape() {
        let body = mk_let(
            1,
            LcnfLetValue::Lit(LcnfLit::Nat(42)),
            LcnfExpr::Return(LcnfArg::Var(vid(0))),
        );
        let decl = mk_fun_decl("f", body);
        let module = mk_module(vec![decl]);
        let mut analysis = EscapeAnalysis::new();
        analysis.analyze(&module);
        assert_eq!(analysis.get_var_escape(vid(1)), EscapeInfo::NoEscape);
    }
    #[test]
    pub(super) fn test_escape_analysis_ctor_escape() {
        let body = mk_let(
            1,
            LcnfLetValue::Ctor(
                "Pair".into(),
                0,
                vec![LcnfArg::Var(vid(0)), LcnfArg::Var(vid(0))],
            ),
            LcnfExpr::Return(LcnfArg::Var(vid(1))),
        );
        let decl = mk_fun_decl("f", body);
        let module = mk_module(vec![decl]);
        let mut analysis = EscapeAnalysis::new();
        analysis.analyze(&module);
        assert_eq!(analysis.get_var_escape(vid(0)), EscapeInfo::GlobalEscape);
    }
    #[test]
    pub(super) fn test_free_vars_return() {
        let expr = LcnfExpr::Return(LcnfArg::Var(vid(5)));
        let bound = HashSet::new();
        let free = compute_free_vars(&expr, &bound);
        assert!(free.contains(&vid(5)));
    }
    #[test]
    pub(super) fn test_free_vars_let_binds() {
        let expr = mk_let(
            1,
            LcnfLetValue::Lit(LcnfLit::Nat(42)),
            LcnfExpr::Return(LcnfArg::Var(vid(1))),
        );
        let bound = HashSet::new();
        let free = compute_free_vars(&expr, &bound);
        assert!(!free.contains(&vid(1)));
    }
    #[test]
    pub(super) fn test_free_vars_with_bound() {
        let expr = LcnfExpr::Return(LcnfArg::Var(vid(0)));
        let mut bound = HashSet::new();
        bound.insert(vid(0));
        let free = compute_free_vars(&expr, &bound);
        assert!(!free.contains(&vid(0)));
    }
    #[test]
    pub(super) fn test_free_vars_case() {
        let expr = LcnfExpr::Case {
            scrutinee: vid(0),
            scrutinee_ty: LcnfType::Nat,
            alts: vec![
                LcnfAlt {
                    ctor_name: "A".into(),
                    ctor_tag: 0,
                    params: vec![mk_param(1, "a")],
                    body: LcnfExpr::Return(LcnfArg::Var(vid(1))),
                },
                LcnfAlt {
                    ctor_name: "B".into(),
                    ctor_tag: 1,
                    params: vec![],
                    body: LcnfExpr::Return(LcnfArg::Var(vid(5))),
                },
            ],
            default: None,
        };
        let bound = HashSet::new();
        let free = compute_free_vars(&expr, &bound);
        assert!(free.contains(&vid(0)));
        assert!(!free.contains(&vid(1)));
        assert!(free.contains(&vid(5)));
    }
    #[test]
    pub(super) fn test_free_vars_tail_call() {
        let expr = LcnfExpr::TailCall(
            LcnfArg::Var(vid(10)),
            vec![LcnfArg::Var(vid(0)), LcnfArg::Var(vid(1))],
        );
        let bound = HashSet::new();
        let free = compute_free_vars(&expr, &bound);
        assert!(free.contains(&vid(10)));
        assert!(free.contains(&vid(0)));
        assert!(free.contains(&vid(1)));
    }
    #[test]
    pub(super) fn test_closure_convert_identity() {
        let body = LcnfExpr::Return(LcnfArg::Var(vid(0)));
        let decl = mk_fun_decl("identity", body.clone());
        let mut module = mk_module(vec![decl]);
        let mut converter = ClosureConverter::default_converter();
        converter.convert_module(&mut module);
        assert_eq!(module.fun_decls.len(), 1);
        assert_eq!(module.fun_decls[0].name, "identity");
    }
    #[test]
    pub(super) fn test_closure_convert_let_chain() {
        let body = mk_let(
            1,
            LcnfLetValue::Lit(LcnfLit::Nat(42)),
            mk_let(
                2,
                LcnfLetValue::FVar(vid(1)),
                LcnfExpr::Return(LcnfArg::Var(vid(2))),
            ),
        );
        let decl = mk_fun_decl("chain", body);
        let mut module = mk_module(vec![decl]);
        let mut converter = ClosureConverter::default_converter();
        converter.convert_module(&mut module);
        assert!(!module.fun_decls.is_empty());
    }
    #[test]
    pub(super) fn test_defunctionalize() {
        let mut converter = ClosureConverter::new(ClosureConvertConfig {
            defunctionalize: true,
            ..Default::default()
        });
        let result = converter.defunctionalize(vid(5), &["add".to_string(), "sub".to_string()]);
        assert!(result.is_some());
        if let Some(LcnfExpr::Case { alts, .. }) = &result {
            assert_eq!(alts.len(), 2);
            assert_eq!(alts[0].ctor_name, "add");
            assert_eq!(alts[1].ctor_name, "sub");
        } else {
            panic!("expected Case");
        }
    }
    #[test]
    pub(super) fn test_defunctionalize_disabled() {
        let mut converter = ClosureConverter::new(ClosureConvertConfig {
            defunctionalize: false,
            ..Default::default()
        });
        let result = converter.defunctionalize(vid(5), &["f".to_string()]);
        assert!(result.is_none());
    }
    #[test]
    pub(super) fn test_stack_allocate_closure() {
        let converter = ClosureConverter::default_converter();
        let non_escaping = ClosureInfo {
            free_vars: vec![vid(1), vid(2)],
            captured_types: vec![],
            arity: 1,
            is_escaping: false,
            has_side_effects: false,
            original_name: None,
        };
        assert!(converter.stack_allocate_closure(&non_escaping));
        let escaping = ClosureInfo {
            is_escaping: true,
            ..non_escaping.clone()
        };
        assert!(!converter.stack_allocate_closure(&escaping));
        let too_many = ClosureInfo {
            free_vars: vec![vid(1), vid(2), vid(3), vid(4), vid(5)],
            ..non_escaping.clone()
        };
        assert!(!converter.stack_allocate_closure(&too_many));
    }
    #[test]
    pub(super) fn test_closure_convert_stats_display() {
        let stats = ClosureConvertStats {
            closures_converted: 3,
            helpers_lifted: 2,
            defunctionalized: 1,
            stack_allocated: 2,
            heap_allocated: 1,
            closures_merged: 0,
        };
        let s = stats.to_string();
        assert!(s.contains("converted=3"));
        assert!(s.contains("lifted=2"));
    }
    #[test]
    pub(super) fn test_closure_convert_config_default() {
        let cfg = ClosureConvertConfig::default();
        assert!(cfg.defunctionalize);
        assert!(cfg.stack_alloc_non_escaping);
        assert_eq!(cfg.max_inline_captures, 4);
    }
    #[test]
    pub(super) fn test_escape_analysis_fn_escape() {
        let body = LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0)));
        let decl = mk_fun_decl("pure_fn", body);
        let module = mk_module(vec![decl]);
        let mut analysis = EscapeAnalysis::new();
        let fn_info = analysis.analyze(&module);
        assert!(fn_info.contains_key("pure_fn"));
    }
}
#[cfg(test)]
mod CC_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = CCPassConfig::new("test_pass", CCPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = CCPassStats::new();
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
        let mut reg = CCPassRegistry::new();
        reg.register(CCPassConfig::new("pass_a", CCPassPhase::Analysis));
        reg.register(CCPassConfig::new("pass_b", CCPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = CCAnalysisCache::new(10);
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
        let mut wl = CCWorklist::new();
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
        let mut dt = CCDominatorTree::new(5);
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
        let mut liveness = CCLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(CCConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(CCConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(CCConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            CCConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(CCConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = CCDepGraph::new();
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
#[cfg(test)]
mod ccext_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_ccext_phase_order() {
        assert_eq!(CCExtPassPhase::Early.order(), 0);
        assert_eq!(CCExtPassPhase::Middle.order(), 1);
        assert_eq!(CCExtPassPhase::Late.order(), 2);
        assert_eq!(CCExtPassPhase::Finalize.order(), 3);
        assert!(CCExtPassPhase::Early.is_early());
        assert!(!CCExtPassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_ccext_config_builder() {
        let c = CCExtPassConfig::new("p")
            .with_phase(CCExtPassPhase::Late)
            .with_max_iter(50)
            .with_debug(1);
        assert_eq!(c.name, "p");
        assert_eq!(c.max_iterations, 50);
        assert!(c.is_debug_enabled());
        assert!(c.enabled);
        let c2 = c.disabled();
        assert!(!c2.enabled);
    }
    #[test]
    pub(super) fn test_ccext_stats() {
        let mut s = CCExtPassStats::new();
        s.visit();
        s.visit();
        s.modify();
        s.iterate();
        assert_eq!(s.nodes_visited, 2);
        assert_eq!(s.nodes_modified, 1);
        assert!(s.changed);
        assert_eq!(s.iterations, 1);
        let e = s.efficiency();
        assert!((e - 0.5).abs() < 1e-9);
    }
    #[test]
    pub(super) fn test_ccext_registry() {
        let mut r = CCExtPassRegistry::new();
        r.register(CCExtPassConfig::new("a").with_phase(CCExtPassPhase::Early));
        r.register(CCExtPassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&CCExtPassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_ccext_cache() {
        let mut c = CCExtCache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_ccext_worklist() {
        let mut w = CCExtWorklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_ccext_dom_tree() {
        let mut dt = CCExtDomTree::new(5);
        dt.set_idom(1, 0);
        dt.set_idom(2, 0);
        dt.set_idom(3, 1);
        dt.set_idom(4, 1);
        assert!(dt.dominates(0, 3));
        assert!(dt.dominates(1, 4));
        assert!(!dt.dominates(2, 3));
        assert_eq!(dt.depth_of(3), 2);
    }
    #[test]
    pub(super) fn test_ccext_liveness() {
        let mut lv = CCExtLiveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_ccext_const_folder() {
        let mut cf = CCExtConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_ccext_dep_graph() {
        let mut g = CCExtDepGraph::new(4);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        assert!(!g.has_cycle());
        assert_eq!(g.topo_sort(), Some(vec![0, 1, 2, 3]));
        assert_eq!(g.reachable(0).len(), 4);
        let sccs = g.scc();
        assert_eq!(sccs.len(), 4);
    }
}
#[cfg(test)]
mod ccx2_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_ccx2_phase_order() {
        assert_eq!(CCX2PassPhase::Early.order(), 0);
        assert_eq!(CCX2PassPhase::Middle.order(), 1);
        assert_eq!(CCX2PassPhase::Late.order(), 2);
        assert_eq!(CCX2PassPhase::Finalize.order(), 3);
        assert!(CCX2PassPhase::Early.is_early());
        assert!(!CCX2PassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_ccx2_config_builder() {
        let c = CCX2PassConfig::new("p")
            .with_phase(CCX2PassPhase::Late)
            .with_max_iter(50)
            .with_debug(1);
        assert_eq!(c.name, "p");
        assert_eq!(c.max_iterations, 50);
        assert!(c.is_debug_enabled());
        assert!(c.enabled);
        let c2 = c.disabled();
        assert!(!c2.enabled);
    }
    #[test]
    pub(super) fn test_ccx2_stats() {
        let mut s = CCX2PassStats::new();
        s.visit();
        s.visit();
        s.modify();
        s.iterate();
        assert_eq!(s.nodes_visited, 2);
        assert_eq!(s.nodes_modified, 1);
        assert!(s.changed);
        assert_eq!(s.iterations, 1);
        let e = s.efficiency();
        assert!((e - 0.5).abs() < 1e-9);
    }
    #[test]
    pub(super) fn test_ccx2_registry() {
        let mut r = CCX2PassRegistry::new();
        r.register(CCX2PassConfig::new("a").with_phase(CCX2PassPhase::Early));
        r.register(CCX2PassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&CCX2PassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_ccx2_cache() {
        let mut c = CCX2Cache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_ccx2_worklist() {
        let mut w = CCX2Worklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_ccx2_dom_tree() {
        let mut dt = CCX2DomTree::new(5);
        dt.set_idom(1, 0);
        dt.set_idom(2, 0);
        dt.set_idom(3, 1);
        dt.set_idom(4, 1);
        assert!(dt.dominates(0, 3));
        assert!(dt.dominates(1, 4));
        assert!(!dt.dominates(2, 3));
        assert_eq!(dt.depth_of(3), 2);
    }
    #[test]
    pub(super) fn test_ccx2_liveness() {
        let mut lv = CCX2Liveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_ccx2_const_folder() {
        let mut cf = CCX2ConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_ccx2_dep_graph() {
        let mut g = CCX2DepGraph::new(4);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        assert!(!g.has_cycle());
        assert_eq!(g.topo_sort(), Some(vec![0, 1, 2, 3]));
        assert_eq!(g.reachable(0).len(), 4);
        let sccs = g.scc();
        assert_eq!(sccs.len(), 4);
    }
}
