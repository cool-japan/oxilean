//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    AccessPattern, CacheLevel, CacheOptPass, CacheOptReport, LoopTile, MemoryAccess,
    OCAnalysisCache, OCConstantFoldingHelper, OCDepGraph, OCDominatorTree, OCLivenessInfo,
    OCPassConfig, OCPassPhase, OCPassRegistry, OCPassStats, OCWorklist, OCacheExtCache,
    OCacheExtConstFolder, OCacheExtDepGraph, OCacheExtDomTree, OCacheExtLiveness,
    OCacheExtPassConfig, OCacheExtPassPhase, OCacheExtPassRegistry, OCacheExtPassStats,
    OCacheExtWorklist, OCacheX2Cache, OCacheX2ConstFolder, OCacheX2DepGraph, OCacheX2DomTree,
    OCacheX2Liveness, OCacheX2PassConfig, OCacheX2PassPhase, OCacheX2PassRegistry,
    OCacheX2PassStats, OCacheX2Worklist, PrefetchHint, PrefetchType, StructLayout,
};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub(super) fn cache_level_capacity_ordering() {
        assert!(CacheLevel::L1.capacity_bytes() < CacheLevel::L2.capacity_bytes());
        assert!(CacheLevel::L2.capacity_bytes() < CacheLevel::L3.capacity_bytes());
    }
    #[test]
    pub(super) fn cache_level_latency_ordering() {
        assert!(CacheLevel::L1.latency_cycles() < CacheLevel::L2.latency_cycles());
        assert!(CacheLevel::L2.latency_cycles() < CacheLevel::L3.latency_cycles());
        assert!(CacheLevel::L3.latency_cycles() < CacheLevel::Ram.latency_cycles());
    }
    #[test]
    pub(super) fn access_pattern_friendliness() {
        assert!(AccessPattern::Sequential.is_cache_friendly());
        assert!(AccessPattern::Broadcast.is_cache_friendly());
        assert!(AccessPattern::Strided(8).is_cache_friendly());
        assert!(!AccessPattern::Strided(128).is_cache_friendly());
        assert!(!AccessPattern::Random.is_cache_friendly());
    }
    #[test]
    pub(super) fn struct_layout_metrics() {
        let layout = StructLayout {
            struct_name: "Foo".into(),
            fields: vec![("a".into(), 8), ("b".into(), 8)],
            total_size: 16,
            alignment: 8,
        };
        assert_eq!(layout.padding_bytes(), 0);
        assert_eq!(layout.cache_lines_used(), 1);
        assert!(!layout.is_cache_aligned());
    }
    #[test]
    pub(super) fn struct_layout_cache_aligned() {
        let layout = StructLayout {
            struct_name: "Aligned".into(),
            fields: vec![("x".into(), 64)],
            total_size: 64,
            alignment: 64,
        };
        assert!(layout.is_cache_aligned());
        assert_eq!(layout.cache_lines_used(), 1);
    }
    #[test]
    pub(super) fn loop_tile_names() {
        let tile = LoopTile::new("i", 64);
        assert_eq!(tile.tile_var, "i_tile");
        assert_eq!(tile.intra_var, "i_intra");
        assert_eq!(tile.tile_size, 64);
    }
    #[test]
    pub(super) fn cache_opt_report_summary() {
        let report = CacheOptReport {
            num_loops_tiled: 3,
            num_prefetches_inserted: 12,
            estimated_cache_miss_reduction: 0.25,
        };
        let s = report.summary();
        assert!(s.contains("3 loops tiled"));
        assert!(s.contains("12 prefetches"));
        assert!(s.contains("25.0%"));
    }
    #[test]
    pub(super) fn memory_access_cache_friendly() {
        let seq = MemoryAccess::new("arr", 0, AccessPattern::Sequential, Some(8), 100);
        assert!(seq.is_cache_friendly());
        let rnd = MemoryAccess::new("arr", 0, AccessPattern::Random, None, 10);
        assert!(!rnd.is_cache_friendly());
    }
    #[test]
    pub(super) fn prefetch_hint_construction() {
        let hint = PrefetchHint::new("&arr[8]", 8, PrefetchType::Read);
        assert_eq!(hint.distance, 8);
        assert_eq!(hint.hint_type, PrefetchType::Read);
    }
    #[test]
    pub(super) fn cache_opt_pass_default() {
        let pass = CacheOptPass::default();
        assert_eq!(pass.config.cache_line_size, 64);
        assert_eq!(pass.config.prefetch_distance, 8);
        assert!(pass.config.enable_prefetch);
    }
}
#[cfg(test)]
mod OC_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = OCPassConfig::new("test_pass", OCPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = OCPassStats::new();
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
        let mut reg = OCPassRegistry::new();
        reg.register(OCPassConfig::new("pass_a", OCPassPhase::Analysis));
        reg.register(OCPassConfig::new("pass_b", OCPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = OCAnalysisCache::new(10);
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
        let mut wl = OCWorklist::new();
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
        let mut dt = OCDominatorTree::new(5);
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
        let mut liveness = OCLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(OCConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(OCConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(OCConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            OCConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(OCConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = OCDepGraph::new();
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
mod ocacheext_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_ocacheext_phase_order() {
        assert_eq!(OCacheExtPassPhase::Early.order(), 0);
        assert_eq!(OCacheExtPassPhase::Middle.order(), 1);
        assert_eq!(OCacheExtPassPhase::Late.order(), 2);
        assert_eq!(OCacheExtPassPhase::Finalize.order(), 3);
        assert!(OCacheExtPassPhase::Early.is_early());
        assert!(!OCacheExtPassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_ocacheext_config_builder() {
        let c = OCacheExtPassConfig::new("p")
            .with_phase(OCacheExtPassPhase::Late)
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
    pub(super) fn test_ocacheext_stats() {
        let mut s = OCacheExtPassStats::new();
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
    pub(super) fn test_ocacheext_registry() {
        let mut r = OCacheExtPassRegistry::new();
        r.register(OCacheExtPassConfig::new("a").with_phase(OCacheExtPassPhase::Early));
        r.register(OCacheExtPassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&OCacheExtPassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_ocacheext_cache() {
        let mut c = OCacheExtCache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_ocacheext_worklist() {
        let mut w = OCacheExtWorklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_ocacheext_dom_tree() {
        let mut dt = OCacheExtDomTree::new(5);
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
    pub(super) fn test_ocacheext_liveness() {
        let mut lv = OCacheExtLiveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_ocacheext_const_folder() {
        let mut cf = OCacheExtConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_ocacheext_dep_graph() {
        let mut g = OCacheExtDepGraph::new(4);
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
mod ocachex2_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_ocachex2_phase_order() {
        assert_eq!(OCacheX2PassPhase::Early.order(), 0);
        assert_eq!(OCacheX2PassPhase::Middle.order(), 1);
        assert_eq!(OCacheX2PassPhase::Late.order(), 2);
        assert_eq!(OCacheX2PassPhase::Finalize.order(), 3);
        assert!(OCacheX2PassPhase::Early.is_early());
        assert!(!OCacheX2PassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_ocachex2_config_builder() {
        let c = OCacheX2PassConfig::new("p")
            .with_phase(OCacheX2PassPhase::Late)
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
    pub(super) fn test_ocachex2_stats() {
        let mut s = OCacheX2PassStats::new();
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
    pub(super) fn test_ocachex2_registry() {
        let mut r = OCacheX2PassRegistry::new();
        r.register(OCacheX2PassConfig::new("a").with_phase(OCacheX2PassPhase::Early));
        r.register(OCacheX2PassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&OCacheX2PassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_ocachex2_cache() {
        let mut c = OCacheX2Cache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_ocachex2_worklist() {
        let mut w = OCacheX2Worklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_ocachex2_dom_tree() {
        let mut dt = OCacheX2DomTree::new(5);
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
    pub(super) fn test_ocachex2_liveness() {
        let mut lv = OCacheX2Liveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_ocachex2_const_folder() {
        let mut cf = OCacheX2ConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_ocachex2_dep_graph() {
        let mut g = OCacheX2DepGraph::new(4);
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
