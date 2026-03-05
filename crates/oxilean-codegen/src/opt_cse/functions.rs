//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::HashMap;

use super::types::{
    AvailableSet, CSEAnalysisCache, CSEConstantFoldingHelper, CSEDepGraph, CSEDominatorTree,
    CSEExtCache, CSEExtConstFolder, CSEExtDepGraph, CSEExtDomTree, CSEExtLiveness,
    CSEExtPassConfig, CSEExtPassPhase, CSEExtPassRegistry, CSEExtPassStats, CSEExtWorklist,
    CSELivenessInfo, CSEPass, CSEPassConfig, CSEPassPhase, CSEPassRegistry, CSEPassStats,
    CSEWorklist, CSEX2Cache, CSEX2ConstFolder, CSEX2DepGraph, CSEX2DomTree, CSEX2Liveness,
    CSEX2PassConfig, CSEX2PassPhase, CSEX2PassRegistry, CSEX2PassStats, CSEX2Worklist, CseConfig,
    CseReport, DominatorTree, ExprKey, GvnTable,
};

/// Normalize a let-bound value into an `ExprKey`, or `None` if the value
/// is not a candidate for CSE (e.g. impure, reset, reuse).
pub fn let_value_to_key(value: &LcnfLetValue, pure_fns: &[String]) -> Option<ExprKey> {
    match value {
        LcnfLetValue::Lit(l) => Some(ExprKey::Lit(l.clone())),
        LcnfLetValue::FVar(v) => Some(ExprKey::Var(*v)),
        LcnfLetValue::Proj(name, idx, var) => Some(ExprKey::Proj(name.clone(), *idx, *var)),
        LcnfLetValue::Ctor(name, tag, args) => {
            Some(ExprKey::Ctor(name.clone(), *tag, args.clone()))
        }
        LcnfLetValue::App(func, args) => {
            let is_pure = match func {
                LcnfArg::Var(_) => false,
                LcnfArg::Lit(_) => false,
                LcnfArg::Erased => false,
                LcnfArg::Type(_) => false,
            };
            let is_named_pure = pure_fns
                .iter()
                .any(|name| matches!(func, LcnfArg::Lit(LcnfLit::Str(s)) if s == name));
            if is_pure || is_named_pure {
                Some(ExprKey::App(func.clone(), args.clone()))
            } else {
                None
            }
        }
        LcnfLetValue::Erased => None,
        LcnfLetValue::Reset(_) => None,
        LcnfLetValue::Reuse(_, _, _, _) => None,
    }
}
/// Run CSE on a vector of function declarations with default configuration.
pub fn optimize_cse(decls: &mut [LcnfFunDecl]) {
    CSEPass::default().run(decls);
}
#[cfg(test)]
mod tests {
    use super::*;
    pub(super) fn make_var(id: u64) -> LcnfVarId {
        LcnfVarId(id)
    }
    pub(super) fn make_lit_nat(n: u64) -> LcnfLetValue {
        LcnfLetValue::Lit(LcnfLit::Nat(n))
    }
    pub(super) fn make_fvar(id: u64) -> LcnfLetValue {
        LcnfLetValue::FVar(make_var(id))
    }
    pub(super) fn make_proj(field: &str, idx: u32, var: u64) -> LcnfLetValue {
        LcnfLetValue::Proj(field.to_string(), idx, make_var(var))
    }
    pub(super) fn make_ctor(name: &str, tag: u32, args: Vec<LcnfArg>) -> LcnfLetValue {
        LcnfLetValue::Ctor(name.to_string(), tag, args)
    }
    pub(super) fn ret(v: u64) -> LcnfExpr {
        LcnfExpr::Return(LcnfArg::Var(make_var(v)))
    }
    pub(super) fn let_binding(id: u64, value: LcnfLetValue, body: LcnfExpr) -> LcnfExpr {
        LcnfExpr::Let {
            id: make_var(id),
            name: format!("x{}", id),
            ty: LcnfType::Nat,
            value,
            body: Box::new(body),
        }
    }
    #[test]
    pub(super) fn test_key_lit() {
        let v = make_lit_nat(42);
        let key = let_value_to_key(&v, &[]);
        assert_eq!(key, Some(ExprKey::Lit(LcnfLit::Nat(42))));
    }
    #[test]
    pub(super) fn test_key_fvar() {
        let v = make_fvar(7);
        let key = let_value_to_key(&v, &[]);
        assert_eq!(key, Some(ExprKey::Var(make_var(7))));
    }
    #[test]
    pub(super) fn test_key_proj() {
        let v = make_proj("foo", 1, 3);
        let key = let_value_to_key(&v, &[]);
        assert_eq!(key, Some(ExprKey::Proj("foo".into(), 1, make_var(3))));
    }
    #[test]
    pub(super) fn test_key_ctor() {
        let v = make_ctor("Pair", 0, vec![LcnfArg::Var(make_var(1))]);
        let key = let_value_to_key(&v, &[]);
        assert_eq!(
            key,
            Some(ExprKey::Ctor(
                "Pair".into(),
                0,
                vec![LcnfArg::Var(make_var(1))]
            ))
        );
    }
    #[test]
    pub(super) fn test_key_reset_is_none() {
        let v = LcnfLetValue::Reset(make_var(5));
        assert_eq!(let_value_to_key(&v, &[]), None);
    }
    #[test]
    pub(super) fn test_key_erased_is_none() {
        let v = LcnfLetValue::Erased;
        assert_eq!(let_value_to_key(&v, &[]), None);
    }
    #[test]
    pub(super) fn test_avail_insert_and_find() {
        let mut avail = AvailableSet::new();
        let key = ExprKey::Lit(LcnfLit::Nat(1));
        avail.insert(key.clone(), make_var(10), "x10".into(), 0);
        assert_eq!(avail.find(&key), Some(make_var(10)));
    }
    #[test]
    pub(super) fn test_avail_miss() {
        let avail = AvailableSet::new();
        let key = ExprKey::Lit(LcnfLit::Nat(99));
        assert_eq!(avail.find(&key), None);
    }
    #[test]
    pub(super) fn test_avail_kill_above_depth() {
        let mut avail = AvailableSet::new();
        let k0 = ExprKey::Lit(LcnfLit::Nat(0));
        let k1 = ExprKey::Lit(LcnfLit::Nat(1));
        avail.insert(k0.clone(), make_var(1), "a".into(), 0);
        avail.insert(k1.clone(), make_var(2), "b".into(), 3);
        avail.kill_above_depth(2);
        assert_eq!(avail.find(&k0), Some(make_var(1)));
        assert_eq!(avail.find(&k1), None);
    }
    #[test]
    pub(super) fn test_avail_len() {
        let mut avail = AvailableSet::new();
        assert_eq!(avail.len(), 0);
        avail.insert(ExprKey::Lit(LcnfLit::Nat(1)), make_var(1), "a".into(), 0);
        assert_eq!(avail.len(), 1);
    }
    #[test]
    pub(super) fn test_gvn_insert_and_lookup() {
        let mut gvn = GvnTable::new();
        let key = ExprKey::Lit(LcnfLit::Nat(7));
        let rep = gvn.insert(key.clone(), make_var(42));
        assert_eq!(rep, make_var(42));
        assert_eq!(gvn.lookup(&key), Some(make_var(42)));
    }
    #[test]
    pub(super) fn test_gvn_duplicate_keeps_first() {
        let mut gvn = GvnTable::new();
        let key = ExprKey::Lit(LcnfLit::Nat(7));
        gvn.insert(key.clone(), make_var(1));
        let rep2 = gvn.insert(key.clone(), make_var(2));
        assert_eq!(rep2, make_var(1));
    }
    #[test]
    pub(super) fn test_dominator_tree() {
        let mut dom = DominatorTree::new();
        dom.insert(make_var(1), 0);
        dom.insert(make_var(2), 2);
        assert!(dom.dominates(make_var(1), make_var(2)));
        assert!(!dom.dominates(make_var(2), make_var(1)));
    }
    #[test]
    pub(super) fn test_local_cse_eliminates_duplicate_lit() {
        let expr = let_binding(
            1,
            make_lit_nat(42),
            let_binding(2, make_lit_nat(42), ret(2)),
        );
        let mut pass = CSEPass::default();
        let result = pass.local_cse(&expr);
        if let LcnfExpr::Let { body, .. } = &result {
            if let LcnfExpr::Let { value, .. } = body.as_ref() {
                assert_eq!(*value, LcnfLetValue::FVar(make_var(1)));
            } else {
                panic!("expected inner Let");
            }
        } else {
            panic!("expected outer Let");
        }
        assert_eq!(pass.report().expressions_eliminated, 1);
    }
    #[test]
    pub(super) fn test_local_cse_no_false_positive_different_lit() {
        let expr = let_binding(1, make_lit_nat(1), let_binding(2, make_lit_nat(2), ret(2)));
        let mut pass = CSEPass::default();
        let result = pass.local_cse(&expr);
        if let LcnfExpr::Let { body, .. } = &result {
            if let LcnfExpr::Let { value, .. } = body.as_ref() {
                assert_eq!(*value, make_lit_nat(2));
            } else {
                panic!("expected inner Let");
            }
        }
        assert_eq!(pass.report().expressions_eliminated, 0);
    }
    #[test]
    pub(super) fn test_local_cse_duplicate_proj() {
        let proj = make_proj("record", 0, 5);
        let expr = let_binding(10, proj.clone(), let_binding(11, proj.clone(), ret(11)));
        let mut pass = CSEPass::default();
        let result = pass.local_cse(&expr);
        if let LcnfExpr::Let { body, .. } = &result {
            if let LcnfExpr::Let { value, .. } = body.as_ref() {
                assert_eq!(*value, LcnfLetValue::FVar(make_var(10)));
            } else {
                panic!("expected inner Let");
            }
        }
        assert_eq!(pass.report().expressions_eliminated, 1);
    }
    #[test]
    pub(super) fn test_local_cse_duplicate_ctor() {
        let ctor = make_ctor("Some", 1, vec![LcnfArg::Var(make_var(3))]);
        let expr = let_binding(20, ctor.clone(), let_binding(21, ctor.clone(), ret(21)));
        let mut pass = CSEPass::default();
        pass.local_cse(&expr);
        assert_eq!(pass.report().expressions_eliminated, 1);
    }
    #[test]
    pub(super) fn test_cse_return_unchanged() {
        let expr = LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0)));
        let mut pass = CSEPass::default();
        let result = pass.local_cse(&expr);
        assert_eq!(result, expr);
    }
    #[test]
    pub(super) fn test_cse_unreachable_unchanged() {
        let expr = LcnfExpr::Unreachable;
        let mut pass = CSEPass::default();
        let result = pass.local_cse(&expr);
        assert_eq!(result, expr);
    }
    #[test]
    pub(super) fn test_cse_report_display() {
        let r = CseReport {
            expressions_found: 5,
            expressions_eliminated: 3,
            lets_hoisted: 1,
        };
        let s = format!("{}", r);
        assert!(s.contains("found=5"));
        assert!(s.contains("eliminated=3"));
    }
    #[test]
    pub(super) fn test_cse_config_display() {
        let c = CseConfig::default();
        let s = format!("{}", c);
        assert!(s.contains("max_expr_size=20"));
    }
    #[test]
    pub(super) fn test_global_cse_run() {
        let mut decls: Vec<LcnfFunDecl> = vec![];
        CSEPass::default().run(&mut decls);
    }
    #[test]
    pub(super) fn test_cse_triple_dup() {
        let expr = let_binding(
            1,
            make_lit_nat(7),
            let_binding(2, make_lit_nat(7), let_binding(3, make_lit_nat(7), ret(3))),
        );
        let mut pass = CSEPass::default();
        pass.local_cse(&expr);
        assert_eq!(pass.report().expressions_eliminated, 2);
    }
    #[test]
    pub(super) fn test_cse_case_branches_independent() {
        let inner_lit = make_lit_nat(99);
        let alt_body = let_binding(
            10,
            inner_lit.clone(),
            let_binding(11, inner_lit.clone(), ret(11)),
        );
        let case_expr = LcnfExpr::Case {
            scrutinee: make_var(0),
            scrutinee_ty: LcnfType::Nat,
            alts: vec![LcnfAlt {
                ctor_name: "Zero".into(),
                ctor_tag: 0,
                params: vec![],
                body: alt_body,
            }],
            default: None,
        };
        let mut pass = CSEPass::default();
        pass.local_cse(&case_expr);
        assert_eq!(pass.report().expressions_eliminated, 1);
    }
    #[test]
    pub(super) fn test_find_available() {
        let pass = CSEPass::default();
        let mut avail = AvailableSet::new();
        let proj = make_proj("hd", 0, 1);
        let key = let_value_to_key(&proj, &[]).expect("key key extraction should succeed");
        avail.insert(key, make_var(5), "hd".into(), 0);
        let result = pass.find_available(&avail, &proj);
        assert_eq!(result, Some(make_var(5)));
    }
    #[test]
    pub(super) fn test_hash_expr_is_commutative() {
        let pass = CSEPass::default();
        let va = LcnfArg::Lit(LcnfLit::Nat(1));
        let vb = LcnfArg::Lit(LcnfLit::Nat(2));
        let v1 = LcnfLetValue::App(
            LcnfArg::Lit(LcnfLit::Str("add".into())),
            vec![va.clone(), vb.clone()],
        );
        let v2 = LcnfLetValue::App(
            LcnfArg::Lit(LcnfLit::Str("add".into())),
            vec![vb.clone(), va.clone()],
        );
        let k1 = pass.hash_expr(&v1);
        let k2 = pass.hash_expr(&v2);
        assert_eq!(k1, k2);
    }
    #[test]
    pub(super) fn test_optimize_cse_convenience() {
        let mut decls: Vec<LcnfFunDecl> = vec![];
        optimize_cse(&mut decls);
    }
}
#[cfg(test)]
mod CSE_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = CSEPassConfig::new("test_pass", CSEPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = CSEPassStats::new();
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
        let mut reg = CSEPassRegistry::new();
        reg.register(CSEPassConfig::new("pass_a", CSEPassPhase::Analysis));
        reg.register(CSEPassConfig::new("pass_b", CSEPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = CSEAnalysisCache::new(10);
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
        let mut wl = CSEWorklist::new();
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
        let mut dt = CSEDominatorTree::new(5);
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
        let mut liveness = CSELivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(CSEConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(CSEConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(CSEConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            CSEConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(CSEConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = CSEDepGraph::new();
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
mod cseext_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_cseext_phase_order() {
        assert_eq!(CSEExtPassPhase::Early.order(), 0);
        assert_eq!(CSEExtPassPhase::Middle.order(), 1);
        assert_eq!(CSEExtPassPhase::Late.order(), 2);
        assert_eq!(CSEExtPassPhase::Finalize.order(), 3);
        assert!(CSEExtPassPhase::Early.is_early());
        assert!(!CSEExtPassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_cseext_config_builder() {
        let c = CSEExtPassConfig::new("p")
            .with_phase(CSEExtPassPhase::Late)
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
    pub(super) fn test_cseext_stats() {
        let mut s = CSEExtPassStats::new();
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
    pub(super) fn test_cseext_registry() {
        let mut r = CSEExtPassRegistry::new();
        r.register(CSEExtPassConfig::new("a").with_phase(CSEExtPassPhase::Early));
        r.register(CSEExtPassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&CSEExtPassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_cseext_cache() {
        let mut c = CSEExtCache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_cseext_worklist() {
        let mut w = CSEExtWorklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_cseext_dom_tree() {
        let mut dt = CSEExtDomTree::new(5);
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
    pub(super) fn test_cseext_liveness() {
        let mut lv = CSEExtLiveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_cseext_const_folder() {
        let mut cf = CSEExtConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_cseext_dep_graph() {
        let mut g = CSEExtDepGraph::new(4);
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
mod csex2_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_csex2_phase_order() {
        assert_eq!(CSEX2PassPhase::Early.order(), 0);
        assert_eq!(CSEX2PassPhase::Middle.order(), 1);
        assert_eq!(CSEX2PassPhase::Late.order(), 2);
        assert_eq!(CSEX2PassPhase::Finalize.order(), 3);
        assert!(CSEX2PassPhase::Early.is_early());
        assert!(!CSEX2PassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_csex2_config_builder() {
        let c = CSEX2PassConfig::new("p")
            .with_phase(CSEX2PassPhase::Late)
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
    pub(super) fn test_csex2_stats() {
        let mut s = CSEX2PassStats::new();
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
    pub(super) fn test_csex2_registry() {
        let mut r = CSEX2PassRegistry::new();
        r.register(CSEX2PassConfig::new("a").with_phase(CSEX2PassPhase::Early));
        r.register(CSEX2PassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&CSEX2PassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_csex2_cache() {
        let mut c = CSEX2Cache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_csex2_worklist() {
        let mut w = CSEX2Worklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_csex2_dom_tree() {
        let mut dt = CSEX2DomTree::new(5);
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
    pub(super) fn test_csex2_liveness() {
        let mut lv = CSEX2Liveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_csex2_const_folder() {
        let mut cf = CSEX2ConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_csex2_dep_graph() {
        let mut g = CSEX2DepGraph::new(4);
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
