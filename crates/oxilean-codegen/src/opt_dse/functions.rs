//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::{HashMap, HashSet};

use super::types::{
    DSEAnalysisCache, DSEConstantFoldingHelper, DSEDepGraph, DSEDominatorTree, DSEExtCache,
    DSEExtConstFolder, DSEExtDepGraph, DSEExtDomTree, DSEExtLiveness, DSEExtPassConfig,
    DSEExtPassPhase, DSEExtPassRegistry, DSEExtPassStats, DSEExtWorklist, DSELivenessInfo, DSEPass,
    DSEPassConfig, DSEPassPhase, DSEPassRegistry, DSEPassStats, DSEReport, DSEWorklist, DSEX2Cache,
    DSEX2ConstFolder, DSEX2DepGraph, DSEX2DomTree, DSEX2Liveness, DSEX2PassConfig, DSEX2PassPhase,
    DSEX2PassRegistry, DSEX2PassStats, DSEX2Worklist, DeadStoreConfig, LiveVariableInfo, StoreInfo,
    UseDefChain,
};

/// Alias for `LiveVariableInfo` (used as the return type of `compute_liveness`).
pub type LivenessInfo = LiveVariableInfo;
/// Collect the set of all variables *defined* (introduced by let) in `expr`.
pub(super) fn collect_defs_uses(
    expr: &LcnfExpr,
    defs: &mut HashSet<LcnfVarId>,
    uses: &mut HashSet<LcnfVarId>,
) {
    match expr {
        LcnfExpr::Let {
            id, value, body, ..
        } => {
            defs.insert(*id);
            collect_used_in_let_value_into(value, uses);
            collect_defs_uses(body, defs, uses);
        }
        LcnfExpr::Case {
            scrutinee,
            alts,
            default,
            ..
        } => {
            uses.insert(*scrutinee);
            for alt in alts {
                collect_defs_uses(&alt.body, defs, uses);
            }
            if let Some(d) = default {
                collect_defs_uses(d, defs, uses);
            }
        }
        LcnfExpr::Return(arg) => collect_used_in_arg_into(arg, uses),
        LcnfExpr::TailCall(f, args) => {
            collect_used_in_arg_into(f, uses);
            for a in args {
                collect_used_in_arg_into(a, uses);
            }
        }
        LcnfExpr::Unreachable => {}
    }
}
pub(super) fn collect_used_in_arg_into(arg: &LcnfArg, out: &mut HashSet<LcnfVarId>) {
    if let LcnfArg::Var(v) = arg {
        out.insert(*v);
    }
}
pub(super) fn collect_used_in_let_value_into(val: &LcnfLetValue, out: &mut HashSet<LcnfVarId>) {
    match val {
        LcnfLetValue::App(f, args) => {
            collect_used_in_arg_into(f, out);
            for a in args {
                collect_used_in_arg_into(a, out);
            }
        }
        LcnfLetValue::Proj(_, _, v) => {
            out.insert(*v);
        }
        LcnfLetValue::Ctor(_, _, args) => {
            for a in args {
                collect_used_in_arg_into(a, out);
            }
        }
        LcnfLetValue::FVar(v) => {
            out.insert(*v);
        }
        LcnfLetValue::Reset(v) => {
            out.insert(*v);
        }
        LcnfLetValue::Reuse(slot, _, _, args) => {
            out.insert(*slot);
            for a in args {
                collect_used_in_arg_into(a, out);
            }
        }
        LcnfLetValue::Lit(_) | LcnfLetValue::Erased => {}
    }
}
/// Collect all variables *used* (read) in `expr`.
pub(super) fn collect_used_in_expr(expr: &LcnfExpr, out: &mut HashSet<LcnfVarId>) {
    match expr {
        LcnfExpr::Let { value, body, .. } => {
            collect_used_in_let_value_into(value, out);
            collect_used_in_expr(body, out);
        }
        LcnfExpr::Case {
            scrutinee,
            alts,
            default,
            ..
        } => {
            out.insert(*scrutinee);
            for alt in alts {
                collect_used_in_expr(&alt.body, out);
            }
            if let Some(d) = default {
                collect_used_in_expr(d, out);
            }
        }
        LcnfExpr::Return(arg) => collect_used_in_arg_into(arg, out),
        LcnfExpr::TailCall(f, args) => {
            collect_used_in_arg_into(f, out);
            for a in args {
                collect_used_in_arg_into(a, out);
            }
        }
        LcnfExpr::Unreachable => {}
    }
}
/// Backward liveness pass: walk `expr` from bottom to top, maintaining
/// `live` as the set of variables live *below* the current point.
/// Records `live_after[x]` for each let-binding `x`.
pub(super) fn backward_liveness(
    expr: &LcnfExpr,
    live: &mut HashSet<LcnfVarId>,
    info: &mut LiveVariableInfo,
) {
    match expr {
        LcnfExpr::Let {
            id, value, body, ..
        } => {
            backward_liveness(body, live, info);
            info.live_after.insert(*id, live.clone());
            live.remove(id);
            collect_used_in_let_value_into(value, live);
        }
        LcnfExpr::Case {
            scrutinee,
            alts,
            default,
            ..
        } => {
            let mut branch_live: HashSet<LcnfVarId> = HashSet::new();
            for alt in alts {
                let mut bl = live.clone();
                backward_liveness(&alt.body, &mut bl, info);
                branch_live.extend(bl);
            }
            if let Some(d) = default {
                let mut bl = live.clone();
                backward_liveness(d, &mut bl, info);
                branch_live.extend(bl);
            }
            *live = branch_live;
            live.insert(*scrutinee);
        }
        LcnfExpr::Return(arg) => {
            collect_used_in_arg_into(arg, live);
        }
        LcnfExpr::TailCall(f, args) => {
            collect_used_in_arg_into(f, live);
            for a in args {
                collect_used_in_arg_into(a, live);
            }
        }
        LcnfExpr::Unreachable => {}
    }
}
/// Check if a `LcnfLetValue` is "side-effect free" (no allocation or call).
pub(super) fn is_pure(val: &LcnfLetValue, aggressive: bool) -> bool {
    match val {
        LcnfLetValue::Lit(_) | LcnfLetValue::Erased | LcnfLetValue::FVar(_) => true,
        LcnfLetValue::Proj(..) => true,
        LcnfLetValue::Ctor(..) => aggressive,
        LcnfLetValue::App(..) | LcnfLetValue::Reset(..) | LcnfLetValue::Reuse(..) => false,
    }
}
/// Recursively remove dead let-bindings from `expr`.
pub(super) fn remove_dead_lets(
    expr: &mut LcnfExpr,
    dead: &HashSet<LcnfVarId>,
    cfg: &DeadStoreConfig,
) {
    loop {
        match expr {
            LcnfExpr::Let {
                id, value, body, ..
            } if dead.contains(id) && is_pure(value, cfg.aggressive) => {
                let new_expr = std::mem::replace(body.as_mut(), LcnfExpr::Unreachable);
                *expr = new_expr;
            }
            LcnfExpr::Let { body, .. } => {
                remove_dead_lets(body, dead, cfg);
                break;
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts.iter_mut() {
                    remove_dead_lets(&mut alt.body, dead, cfg);
                }
                if let Some(d) = default {
                    remove_dead_lets(d, dead, cfg);
                }
                break;
            }
            LcnfExpr::Return(_) | LcnfExpr::Unreachable | LcnfExpr::TailCall(..) => break,
        }
    }
}
pub(super) fn var(n: u64) -> LcnfVarId {
    LcnfVarId(n)
}
pub(super) fn lit_val(n: u64) -> LcnfLetValue {
    LcnfLetValue::Lit(LcnfLit::Nat(n))
}
pub(super) fn arg_var(n: u64) -> LcnfArg {
    LcnfArg::Var(LcnfVarId(n))
}
pub(super) fn arg_lit(n: u64) -> LcnfArg {
    LcnfArg::Lit(LcnfLit::Nat(n))
}
pub(super) fn make_decl(name: &str, body: LcnfExpr) -> LcnfFunDecl {
    LcnfFunDecl {
        name: name.to_string(),
        params: vec![],
        ret_type: LcnfType::Nat,
        original_name: None,
        is_recursive: false,
        is_lifted: false,
        inline_cost: 0,
        body,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    pub(super) fn let_nat(id: u64, n: u64, body: LcnfExpr) -> LcnfExpr {
        LcnfExpr::Let {
            id: var(id),
            name: format!("x{}", var(id).0),
            ty: LcnfType::Nat,
            value: lit_val(n),
            body: Box::new(body),
        }
    }
    pub(super) fn let_fvar(id: u64, src: u64, body: LcnfExpr) -> LcnfExpr {
        LcnfExpr::Let {
            id: var(id),
            name: format!("x{}", var(id).0),
            ty: LcnfType::Nat,
            value: LcnfLetValue::FVar(var(src)),
            body: Box::new(body),
        }
    }
    #[test]
    pub(super) fn test_dse_default_config() {
        let cfg = DeadStoreConfig::default();
        assert!(!cfg.check_aliasing);
        assert!(!cfg.aggressive);
    }
    #[test]
    pub(super) fn test_dse_report_default() {
        let r = DSEReport::default();
        assert_eq!(r.stores_analyzed, 0);
        assert_eq!(r.dead_stores_eliminated, 0);
        assert_eq!(r.bytes_saved, 0);
    }
    #[test]
    pub(super) fn test_dse_report_merge() {
        let mut r1 = DSEReport {
            stores_analyzed: 4,
            dead_stores_eliminated: 2,
            bytes_saved: 128,
        };
        let r2 = DSEReport {
            stores_analyzed: 3,
            dead_stores_eliminated: 1,
            bytes_saved: 64,
        };
        r1.merge(&r2);
        assert_eq!(r1.stores_analyzed, 7);
        assert_eq!(r1.dead_stores_eliminated, 3);
        assert_eq!(r1.bytes_saved, 192);
    }
    #[test]
    pub(super) fn test_usedefchain_add_use() {
        let mut udc = UseDefChain::new();
        udc.add_use(var(1), var(2));
        assert!(udc.has_uses(&var(1)));
        assert!(!udc.has_uses(&var(99)));
    }
    #[test]
    pub(super) fn test_usedefchain_add_def() {
        let mut udc = UseDefChain::new();
        udc.add_def(var(5), lit_val(42));
        assert!(udc.defs.contains_key(&var(5)));
    }
    #[test]
    pub(super) fn test_store_info_construction() {
        let si = StoreInfo {
            var: var(3),
            value: lit_val(7),
            overwritten_before_read: false,
        };
        assert_eq!(si.var, var(3));
        assert!(!si.overwritten_before_read);
    }
    #[test]
    pub(super) fn test_live_variable_info_default() {
        let lvi = LiveVariableInfo::new();
        assert!(lvi.live_at_entry.is_empty());
    }
    #[test]
    pub(super) fn test_compute_liveness_return_only() {
        let decl = make_decl("f", LcnfExpr::Return(arg_var(5)));
        let pass = DSEPass::default();
        let liveness = pass.compute_liveness(&decl);
        assert!(liveness.live_at_entry.contains(&var(5)));
    }
    #[test]
    pub(super) fn test_find_dead_stores_unused_lit() {
        let body = let_nat(1, 42, LcnfExpr::Return(arg_lit(0)));
        let decl = make_decl("f", body);
        let mut pass = DSEPass::default();
        let liveness = pass.compute_liveness(&decl);
        let dead = pass.find_dead_stores(&decl, &liveness);
        assert!(dead.contains(&var(1)));
    }
    #[test]
    pub(super) fn test_find_dead_stores_used_var() {
        let body = let_nat(1, 42, LcnfExpr::Return(arg_var(1)));
        let decl = make_decl("f", body);
        let mut pass = DSEPass::default();
        let liveness = pass.compute_liveness(&decl);
        let dead = pass.find_dead_stores(&decl, &liveness);
        assert!(!dead.contains(&var(1)));
    }
    #[test]
    pub(super) fn test_eliminate_dead_stores_removes_unused_let() {
        let body = let_nat(1, 42, LcnfExpr::Return(arg_lit(0)));
        let mut decl = make_decl("f", body);
        let mut pass = DSEPass::default();
        let liveness = pass.compute_liveness(&decl);
        let dead = pass.find_dead_stores(&decl, &liveness);
        pass.eliminate_dead_stores(&mut decl, &dead);
        assert!(matches!(decl.body, LcnfExpr::Return(_)));
    }
    #[test]
    pub(super) fn test_run_removes_dead_stores() {
        let body = let_nat(1, 42, LcnfExpr::Return(arg_lit(0)));
        let mut decls = vec![make_decl("f", body)];
        let mut pass = DSEPass::default();
        pass.run(&mut decls);
        assert!(pass.report().dead_stores_eliminated >= 1);
        assert!(matches!(decls[0].body, LcnfExpr::Return(_)));
    }
    #[test]
    pub(super) fn test_run_keeps_live_stores() {
        let body = let_nat(1, 42, LcnfExpr::Return(arg_var(1)));
        let mut decls = vec![make_decl("f", body)];
        let mut pass = DSEPass::default();
        pass.run(&mut decls);
        assert_eq!(pass.report().dead_stores_eliminated, 0);
        assert!(matches!(decls[0].body, LcnfExpr::Let { .. }));
    }
    #[test]
    pub(super) fn test_run_chain_of_dead_stores() {
        let body = let_nat(1, 1, let_nat(2, 2, LcnfExpr::Return(arg_lit(0))));
        let mut decls = vec![make_decl("f", body)];
        let mut pass = DSEPass::default();
        pass.run(&mut decls);
        assert!(pass.report().dead_stores_eliminated >= 2);
    }
    #[test]
    pub(super) fn test_run_fvar_dead() {
        let body = let_nat(1, 10, let_fvar(2, 1, LcnfExpr::Return(arg_var(1))));
        let mut decls = vec![make_decl("f", body)];
        let mut pass = DSEPass::default();
        pass.run(&mut decls);
        assert!(pass.report().dead_stores_eliminated >= 1);
    }
    #[test]
    pub(super) fn test_is_pure_lit() {
        assert!(is_pure(&lit_val(0), false));
    }
    #[test]
    pub(super) fn test_is_pure_app_not_pure() {
        let val = LcnfLetValue::App(LcnfArg::Lit(LcnfLit::Nat(0)), vec![]);
        assert!(!is_pure(&val, false));
        assert!(!is_pure(&val, true));
    }
    #[test]
    pub(super) fn test_is_pure_ctor_aggressive() {
        let val = LcnfLetValue::Ctor("Foo".to_string(), 0, vec![]);
        assert!(!is_pure(&val, false));
        assert!(is_pure(&val, true));
    }
    #[test]
    pub(super) fn test_run_multiple_decls() {
        let mut decls = vec![
            make_decl("a", let_nat(1, 5, LcnfExpr::Return(arg_lit(0)))),
            make_decl("b", let_nat(2, 6, LcnfExpr::Return(arg_lit(0)))),
        ];
        let mut pass = DSEPass::default();
        pass.run(&mut decls);
        assert!(pass.report().dead_stores_eliminated >= 2);
    }
    #[test]
    pub(super) fn test_bytes_saved_proportional() {
        let body = let_nat(1, 1, LcnfExpr::Return(arg_lit(0)));
        let mut decls = vec![make_decl("f", body)];
        let mut pass = DSEPass::default();
        pass.run(&mut decls);
        let r = pass.report();
        assert_eq!(r.bytes_saved, r.dead_stores_eliminated * 64);
    }
}
#[cfg(test)]
mod DSE_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = DSEPassConfig::new("test_pass", DSEPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = DSEPassStats::new();
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
        let mut reg = DSEPassRegistry::new();
        reg.register(DSEPassConfig::new("pass_a", DSEPassPhase::Analysis));
        reg.register(DSEPassConfig::new("pass_b", DSEPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = DSEAnalysisCache::new(10);
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
        let mut wl = DSEWorklist::new();
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
        let mut dt = DSEDominatorTree::new(5);
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
        let mut liveness = DSELivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(DSEConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(DSEConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(DSEConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            DSEConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(DSEConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = DSEDepGraph::new();
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
mod dseext_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_dseext_phase_order() {
        assert_eq!(DSEExtPassPhase::Early.order(), 0);
        assert_eq!(DSEExtPassPhase::Middle.order(), 1);
        assert_eq!(DSEExtPassPhase::Late.order(), 2);
        assert_eq!(DSEExtPassPhase::Finalize.order(), 3);
        assert!(DSEExtPassPhase::Early.is_early());
        assert!(!DSEExtPassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_dseext_config_builder() {
        let c = DSEExtPassConfig::new("p")
            .with_phase(DSEExtPassPhase::Late)
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
    pub(super) fn test_dseext_stats() {
        let mut s = DSEExtPassStats::new();
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
    pub(super) fn test_dseext_registry() {
        let mut r = DSEExtPassRegistry::new();
        r.register(DSEExtPassConfig::new("a").with_phase(DSEExtPassPhase::Early));
        r.register(DSEExtPassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&DSEExtPassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_dseext_cache() {
        let mut c = DSEExtCache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_dseext_worklist() {
        let mut w = DSEExtWorklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_dseext_dom_tree() {
        let mut dt = DSEExtDomTree::new(5);
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
    pub(super) fn test_dseext_liveness() {
        let mut lv = DSEExtLiveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_dseext_const_folder() {
        let mut cf = DSEExtConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_dseext_dep_graph() {
        let mut g = DSEExtDepGraph::new(4);
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
mod dsex2_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_dsex2_phase_order() {
        assert_eq!(DSEX2PassPhase::Early.order(), 0);
        assert_eq!(DSEX2PassPhase::Middle.order(), 1);
        assert_eq!(DSEX2PassPhase::Late.order(), 2);
        assert_eq!(DSEX2PassPhase::Finalize.order(), 3);
        assert!(DSEX2PassPhase::Early.is_early());
        assert!(!DSEX2PassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_dsex2_config_builder() {
        let c = DSEX2PassConfig::new("p")
            .with_phase(DSEX2PassPhase::Late)
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
    pub(super) fn test_dsex2_stats() {
        let mut s = DSEX2PassStats::new();
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
    pub(super) fn test_dsex2_registry() {
        let mut r = DSEX2PassRegistry::new();
        r.register(DSEX2PassConfig::new("a").with_phase(DSEX2PassPhase::Early));
        r.register(DSEX2PassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&DSEX2PassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_dsex2_cache() {
        let mut c = DSEX2Cache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_dsex2_worklist() {
        let mut w = DSEX2Worklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_dsex2_dom_tree() {
        let mut dt = DSEX2DomTree::new(5);
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
    pub(super) fn test_dsex2_liveness() {
        let mut lv = DSEX2Liveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_dsex2_const_folder() {
        let mut cf = DSEX2ConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_dsex2_dep_graph() {
        let mut g = DSEX2DepGraph::new(4);
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
