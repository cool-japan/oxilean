//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};

use super::types::{
    AllocationSinkKind, ConnectionGraph, EscapeAnnotation, EscapeAnnotationPass,
    EscapeBasedRefCountOpt, EscapeEdgeKind, EscapeFlowGraph2, EscapeOptimizationPass,
    EscapeOptimizationResult, EscapeSummary, FieldEscapeInfo, InterproceduralEscapeAnalysis,
    OEAnalysisCache, OEConstantFoldingHelper, OEDepGraph, OEDominatorTree, OEExtCache,
    OEExtConstFolder, OEExtDepGraph, OEExtDomTree, OEExtLiveness, OEExtPassConfig, OEExtPassPhase,
    OEExtPassRegistry, OEExtPassStats, OEExtWorklist, OELivenessInfo, OEPassConfig, OEPassPhase,
    OEPassRegistry, OEPassStats, OEWorklist, OEX2Cache, OEX2ConstFolder, OEX2DepGraph, OEX2DomTree,
    OEX2Liveness, OEX2PassConfig, OEX2PassPhase, OEX2PassRegistry, OEX2PassStats, OEX2Worklist,
    PointsToSet2, PointsToTarget, StructFieldEscapeAnalyzer,
};

#[cfg(test)]
mod opt_escape_extended_tests {
    use super::*;
    #[test]
    pub(super) fn test_points_to_set() {
        let mut pts = PointsToSet2::new();
        pts.add(PointsToTarget::HeapObject(1));
        pts.add(PointsToTarget::HeapObject(2));
        assert_eq!(pts.len(), 2);
        let mut pts2 = PointsToSet2::new();
        pts2.add(PointsToTarget::HeapObject(2));
        assert!(pts.may_alias(&pts2));
        let pts3 = PointsToSet2::new();
        assert!(!pts.may_alias(&pts3));
    }
    #[test]
    pub(super) fn test_escape_flow_graph() {
        let mut g = EscapeFlowGraph2::new();
        g.add_node(1);
        g.add_node(2);
        g.add_edge(1, 2, EscapeEdgeKind::DirectAssign);
        g.add_edge(2, 3, EscapeEdgeKind::Return);
        assert_eq!(g.node_count(), 3);
        assert_eq!(g.edge_count(), 2);
        assert_eq!(g.successors(1), vec![2]);
        assert_eq!(g.predecessors(2), vec![1]);
    }
    #[test]
    pub(super) fn test_allocation_sink_kind() {
        assert!(AllocationSinkKind::Stack.is_stack_eligible());
        assert!(AllocationSinkKind::ArenaAllocated.is_stack_eligible());
        assert!(!AllocationSinkKind::HeapLongLived.is_stack_eligible());
    }
    #[test]
    pub(super) fn test_optimization_result() {
        let result =
            EscapeOptimizationResult::new(42, AllocationSinkKind::Stack, "no escape detected")
                .with_confidence(0.95);
        assert!(result.is_high_confidence());
        assert_eq!(result.allocation_id, 42);
    }
    #[test]
    pub(super) fn test_escape_opt_pass() {
        let mut pass = EscapeOptimizationPass::new();
        pass.add_result(
            EscapeOptimizationResult::new(1, AllocationSinkKind::Stack, "short-lived")
                .with_confidence(0.95),
        );
        pass.add_result(
            EscapeOptimizationResult::new(2, AllocationSinkKind::HeapLongLived, "long-lived")
                .with_confidence(0.6),
        );
        assert_eq!(pass.total_optimizations(), 2);
        let promotable = pass.stack_promotable();
        assert_eq!(promotable.len(), 1);
        let report = pass.emit_report();
        assert!(report.contains("Stack-promotable allocations: 1"));
    }
    #[test]
    pub(super) fn test_struct_field_escape() {
        let mut analyzer = StructFieldEscapeAnalyzer::new();
        analyzer.add_field(FieldEscapeInfo::new("Foo", "x"));
        let mut f2 = FieldEscapeInfo::new("Foo", "y");
        f2.escapes_via_return = true;
        analyzer.add_field(f2);
        assert_eq!(analyzer.escaping_fields().len(), 1);
        assert_eq!(analyzer.non_escaping_fields().len(), 1);
        assert!(!analyzer.can_scalar_replace_struct("Foo"));
    }
    #[test]
    pub(super) fn test_connection_graph() {
        let mut cg = ConnectionGraph::new();
        cg.add_node(1, "allocation");
        cg.add_node(2, "local_var");
        cg.add_deferred_edge(1, 2);
        cg.propagate_escape();
        let non_escaping = cg.non_escaping_allocations();
        assert_eq!(non_escaping.len(), 2);
    }
    #[test]
    pub(super) fn test_interprocedural_escape() {
        let mut ipa = InterproceduralEscapeAnalysis::new();
        let mut summary = EscapeSummary::default();
        summary.escaping_params.push(0);
        ipa.register_summary("foo", summary);
        assert!(ipa.param_escapes("foo", 0));
        assert!(!ipa.param_escapes("foo", 1));
        assert!(ipa.param_escapes("unknown", 0));
    }
    #[test]
    pub(super) fn test_refcount_opt() {
        let mut opt = EscapeBasedRefCountOpt::new();
        opt.record_elimination();
        opt.record_elimination();
        opt.record_stack_replace(10);
        assert_eq!(opt.total_eliminated(), 4);
        let report = opt.savings_report();
        assert!(report.contains("2 retain/release pairs"));
        assert!(report.contains("1 stack promotions"));
    }
    #[test]
    pub(super) fn test_annotation_pass() {
        let mut pass = EscapeAnnotationPass::new();
        pass.annotate(1, EscapeAnnotation::StackAlloc);
        pass.annotate(2, EscapeAnnotation::HeapAlloc);
        let candidates = pass.stack_promote_candidates();
        assert_eq!(candidates, vec![1]);
        assert!(matches!(
            pass.get_annotation(2),
            Some(EscapeAnnotation::HeapAlloc)
        ));
    }
}
#[cfg(test)]
mod OE_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = OEPassConfig::new("test_pass", OEPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = OEPassStats::new();
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
        let mut reg = OEPassRegistry::new();
        reg.register(OEPassConfig::new("pass_a", OEPassPhase::Analysis));
        reg.register(OEPassConfig::new("pass_b", OEPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = OEAnalysisCache::new(10);
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
        let mut wl = OEWorklist::new();
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
        let mut dt = OEDominatorTree::new(5);
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
        let mut liveness = OELivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(OEConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(OEConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(OEConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            OEConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(OEConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = OEDepGraph::new();
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
mod oeext_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_oeext_phase_order() {
        assert_eq!(OEExtPassPhase::Early.order(), 0);
        assert_eq!(OEExtPassPhase::Middle.order(), 1);
        assert_eq!(OEExtPassPhase::Late.order(), 2);
        assert_eq!(OEExtPassPhase::Finalize.order(), 3);
        assert!(OEExtPassPhase::Early.is_early());
        assert!(!OEExtPassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_oeext_config_builder() {
        let c = OEExtPassConfig::new("p")
            .with_phase(OEExtPassPhase::Late)
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
    pub(super) fn test_oeext_stats() {
        let mut s = OEExtPassStats::new();
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
    pub(super) fn test_oeext_registry() {
        let mut r = OEExtPassRegistry::new();
        r.register(OEExtPassConfig::new("a").with_phase(OEExtPassPhase::Early));
        r.register(OEExtPassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&OEExtPassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_oeext_cache() {
        let mut c = OEExtCache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_oeext_worklist() {
        let mut w = OEExtWorklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_oeext_dom_tree() {
        let mut dt = OEExtDomTree::new(5);
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
    pub(super) fn test_oeext_liveness() {
        let mut lv = OEExtLiveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_oeext_const_folder() {
        let mut cf = OEExtConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_oeext_dep_graph() {
        let mut g = OEExtDepGraph::new(4);
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
mod oex2_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_oex2_phase_order() {
        assert_eq!(OEX2PassPhase::Early.order(), 0);
        assert_eq!(OEX2PassPhase::Middle.order(), 1);
        assert_eq!(OEX2PassPhase::Late.order(), 2);
        assert_eq!(OEX2PassPhase::Finalize.order(), 3);
        assert!(OEX2PassPhase::Early.is_early());
        assert!(!OEX2PassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_oex2_config_builder() {
        let c = OEX2PassConfig::new("p")
            .with_phase(OEX2PassPhase::Late)
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
    pub(super) fn test_oex2_stats() {
        let mut s = OEX2PassStats::new();
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
    pub(super) fn test_oex2_registry() {
        let mut r = OEX2PassRegistry::new();
        r.register(OEX2PassConfig::new("a").with_phase(OEX2PassPhase::Early));
        r.register(OEX2PassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&OEX2PassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_oex2_cache() {
        let mut c = OEX2Cache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_oex2_worklist() {
        let mut w = OEX2Worklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_oex2_dom_tree() {
        let mut dt = OEX2DomTree::new(5);
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
    pub(super) fn test_oex2_liveness() {
        let mut lv = OEX2Liveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_oex2_const_folder() {
        let mut cf = OEX2ConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_oex2_dep_graph() {
        let mut g = OEX2DepGraph::new(4);
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
