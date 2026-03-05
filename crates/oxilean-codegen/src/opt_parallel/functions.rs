//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::{LcnfArg, LcnfExpr, LcnfFunDecl, LcnfLetValue, LcnfVarId};
use std::collections::{HashMap, HashSet};

use super::types::{
    AffineAccess, DepEdge, DependenceAnalyser, DependenceInfo, OParConfig, OParDiagCollector,
    OParDiagMsg, OParEmitStats, OParEventLog, OParExtCache, OParExtConstFolder, OParExtDepGraph,
    OParExtDomTree, OParExtLiveness, OParExtPassConfig, OParExtPassPhase, OParExtPassRegistry,
    OParExtPassStats, OParExtWorklist, OParFeatures, OParIdGen, OParIncrKey, OParNameScope,
    OParPassTiming, OParProfiler, OParSourceBuffer, OParVersion, ParAnalysisCache,
    ParConstantFoldingHelper, ParDepGraph, ParDominatorTree, ParLivenessInfo, ParPassConfig,
    ParPassPhase, ParPassRegistry, ParPassStats, ParWorklist, ParallelConfig, ParallelKind,
    ParallelPass, ParallelPattern, ParallelRegion, ParallelReport, PatternDetector,
    ThreadSafetyInfo,
};

pub(super) fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
/// Amdahl-law speed-up estimate.
///
/// `p` is the parallel fraction (0..1), `n` is the number of cores.
pub(super) fn amdahl_speedup(p: f64, n: f64) -> f64 {
    let p = p.clamp(0.0, 1.0);
    let n = n.max(1.0);
    1.0 / ((1.0 - p) + p / n)
}
pub(super) fn estimate_speedup_for_pattern(
    pattern: ParallelPattern,
    trip_count: Option<u64>,
) -> f64 {
    let n_cores = 8.0_f64;
    let trip = trip_count.unwrap_or(1024) as f64;
    let parallel_fraction = match pattern {
        ParallelPattern::Map => 0.98,
        ParallelPattern::Reduce => 0.85,
        ParallelPattern::Scan => 0.75,
        ParallelPattern::Filter => 0.90,
        ParallelPattern::Stencil => 0.88,
        ParallelPattern::ParallelFor => 0.95,
        ParallelPattern::Scatter => 0.70,
        ParallelPattern::Gather => 0.72,
    };
    let effective_p = parallel_fraction * (trip / (trip + 64.0));
    amdahl_speedup(effective_p, n_cores)
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
    pub(super) fn mk_decl(name: &str, body: LcnfExpr, recursive: bool) -> LcnfFunDecl {
        LcnfFunDecl {
            name: name.to_string(),
            original_name: None,
            params: vec![
                mk_param(0, "arr_in"),
                mk_param(1, "arr_out"),
                mk_param(2, "n"),
            ],
            ret_type: LcnfType::Nat,
            body,
            is_recursive: recursive,
            is_lifted: false,
            inline_cost: 1,
        }
    }
    pub(super) fn simple_return() -> LcnfExpr {
        LcnfExpr::Return(LcnfArg::Lit(crate::lcnf::LcnfLit::Nat(0)))
    }
    pub(super) fn tail_call_body() -> LcnfExpr {
        LcnfExpr::TailCall(LcnfArg::Var(vid(0)), vec![LcnfArg::Var(vid(1))])
    }
    pub(super) fn reduce_body() -> LcnfExpr {
        LcnfExpr::Let {
            id: vid(10),
            name: "acc".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::App(
                LcnfArg::Var(vid(2)),
                vec![LcnfArg::Var(vid(0)), LcnfArg::Var(vid(1))],
            ),
            body: Box::new(tail_call_body()),
        }
    }
    #[test]
    pub(super) fn parallel_kind_display() {
        assert_eq!(ParallelKind::DataParallel.to_string(), "data-parallel");
        assert_eq!(ParallelKind::TaskParallel.to_string(), "task-parallel");
        assert_eq!(
            ParallelKind::PipelineParallel.to_string(),
            "pipeline-parallel"
        );
        assert_eq!(
            ParallelKind::SpeculativeParallel.to_string(),
            "speculative-parallel"
        );
    }
    #[test]
    pub(super) fn parallel_kind_eq() {
        assert_eq!(ParallelKind::DataParallel, ParallelKind::DataParallel);
        assert_ne!(ParallelKind::DataParallel, ParallelKind::TaskParallel);
    }
    #[test]
    pub(super) fn parallel_pattern_display() {
        assert_eq!(ParallelPattern::Map.to_string(), "map");
        assert_eq!(ParallelPattern::Reduce.to_string(), "reduce");
        assert_eq!(ParallelPattern::Scan.to_string(), "scan");
        assert_eq!(ParallelPattern::Filter.to_string(), "filter");
        assert_eq!(ParallelPattern::Stencil.to_string(), "stencil");
        assert_eq!(ParallelPattern::ParallelFor.to_string(), "parallel-for");
        assert_eq!(ParallelPattern::Scatter.to_string(), "scatter");
        assert_eq!(ParallelPattern::Gather.to_string(), "gather");
    }
    #[test]
    pub(super) fn dependence_info_empty_is_parallelizable() {
        let info = DependenceInfo::default();
        assert!(info.is_parallelizable());
        assert_eq!(info.total_deps(), 0);
    }
    #[test]
    pub(super) fn dependence_info_with_loop_carried_not_parallelizable() {
        let mut info = DependenceInfo::default();
        info.loop_carried_deps.push(DepEdge {
            from: "a".to_string(),
            to: "b".to_string(),
            distance: 1,
        });
        assert!(!info.is_parallelizable());
    }
    #[test]
    pub(super) fn dependence_info_total_counts() {
        let mut info = DependenceInfo::default();
        info.true_deps.push(DepEdge {
            from: "a".to_string(),
            to: "b".to_string(),
            distance: 0,
        });
        info.anti_deps.push(DepEdge {
            from: "c".to_string(),
            to: "d".to_string(),
            distance: 0,
        });
        info.output_deps.push(DepEdge {
            from: "e".to_string(),
            to: "f".to_string(),
            distance: 0,
        });
        assert_eq!(info.total_deps(), 3);
        assert!(info.is_parallelizable());
    }
    #[test]
    pub(super) fn dependence_info_display() {
        let info = DependenceInfo::default();
        let s = info.to_string();
        assert!(s.contains("parallelizable=true"));
    }
    #[test]
    pub(super) fn affine_access_different_bases_are_independent() {
        let a = AffineAccess {
            base: "x".to_string(),
            coeff: 1,
            offset: 0,
            is_write: true,
        };
        let b = AffineAccess {
            base: "y".to_string(),
            coeff: 1,
            offset: 0,
            is_write: false,
        };
        assert!(a.independent_from(&b));
    }
    #[test]
    pub(super) fn affine_access_two_reads_are_independent() {
        let a = AffineAccess {
            base: "x".to_string(),
            coeff: 1,
            offset: 0,
            is_write: false,
        };
        let b = AffineAccess {
            base: "x".to_string(),
            coeff: 1,
            offset: 0,
            is_write: false,
        };
        assert!(a.independent_from(&b));
    }
    #[test]
    pub(super) fn affine_access_aliased_write_read_not_independent() {
        let a = AffineAccess {
            base: "a".to_string(),
            coeff: 1,
            offset: 0,
            is_write: true,
        };
        let b = AffineAccess {
            base: "a".to_string(),
            coeff: 1,
            offset: 0,
            is_write: false,
        };
        assert!(!a.independent_from(&b));
    }
    #[test]
    pub(super) fn affine_access_non_overlapping_offsets() {
        let a = AffineAccess {
            base: "a".to_string(),
            coeff: 2,
            offset: 0,
            is_write: true,
        };
        let b = AffineAccess {
            base: "a".to_string(),
            coeff: 2,
            offset: 1,
            is_write: false,
        };
        assert!(a.independent_from(&b));
    }
    #[test]
    pub(super) fn thread_safety_safe_constructor() {
        let info = ThreadSafetyInfo::safe();
        assert!(info.is_thread_safe);
        assert!(info.race_conditions.is_empty());
        assert!(info.atomic_ops_needed.is_empty());
    }
    #[test]
    pub(super) fn thread_safety_unsafe_race_constructor() {
        let info = ThreadSafetyInfo::unsafe_race("w", "r");
        assert!(!info.is_thread_safe);
        assert_eq!(info.race_conditions.len(), 1);
        assert_eq!(info.race_conditions[0], ("w".to_string(), "r".to_string()));
    }
    #[test]
    pub(super) fn thread_safety_display() {
        let info = ThreadSafetyInfo::safe();
        let s = info.to_string();
        assert!(s.contains("safe=true"));
    }
    #[test]
    pub(super) fn parallel_region_new_defaults() {
        let r = ParallelRegion::new("foo", ParallelKind::DataParallel, ParallelPattern::Map);
        assert_eq!(r.func_name, "foo");
        assert_eq!(r.estimated_speedup, 1.0);
        assert!(r.shared_vars.is_empty());
    }
    #[test]
    pub(super) fn parallel_region_not_profitable_at_1x() {
        let r = ParallelRegion::new("foo", ParallelKind::DataParallel, ParallelPattern::Map);
        assert!(!r.is_profitable(1.5));
    }
    #[test]
    pub(super) fn parallel_region_profitable_with_high_speedup() {
        let mut r = ParallelRegion::new("foo", ParallelKind::DataParallel, ParallelPattern::Map);
        r.estimated_speedup = 4.0;
        assert!(r.is_profitable(1.5));
    }
    #[test]
    pub(super) fn parallel_region_not_profitable_with_loop_carried_dep() {
        let mut r = ParallelRegion::new("foo", ParallelKind::DataParallel, ParallelPattern::Map);
        r.estimated_speedup = 4.0;
        r.dependence_info.loop_carried_deps.push(DepEdge {
            from: "a".to_string(),
            to: "b".to_string(),
            distance: 1,
        });
        assert!(!r.is_profitable(1.5));
    }
    #[test]
    pub(super) fn parallel_region_display() {
        let r = ParallelRegion::new("bar", ParallelKind::DataParallel, ParallelPattern::Reduce);
        let s = r.to_string();
        assert!(s.contains("bar"));
        assert!(s.contains("reduce"));
    }
    #[test]
    pub(super) fn amdahl_all_serial() {
        assert!((amdahl_speedup(0.0, 8.0) - 1.0).abs() < 1e-9);
    }
    #[test]
    pub(super) fn amdahl_all_parallel() {
        let s = amdahl_speedup(1.0, 8.0);
        assert!((s - 8.0).abs() < 1e-9);
    }
    #[test]
    pub(super) fn amdahl_clamps_fraction() {
        let s1 = amdahl_speedup(1.0, 4.0);
        let s2 = amdahl_speedup(1.5, 4.0);
        assert!((s1 - s2).abs() < 1e-9);
    }
    #[test]
    pub(super) fn estimate_speedup_map_large_trip() {
        let s = estimate_speedup_for_pattern(ParallelPattern::Map, Some(8192));
        assert!(s > 1.0, "map speedup={}", s);
    }
    #[test]
    pub(super) fn estimate_speedup_reduce_less_than_map() {
        let s_map = estimate_speedup_for_pattern(ParallelPattern::Map, Some(4096));
        let s_red = estimate_speedup_for_pattern(ParallelPattern::Reduce, Some(4096));
        assert!(s_map > s_red, "map={} reduce={}", s_map, s_red);
    }
    #[test]
    pub(super) fn pattern_detector_reduce_body() {
        let decl = mk_decl("reduce_loop", reduce_body(), true);
        let pat = PatternDetector::new(&decl).detect();
        assert_eq!(pat, Some(ParallelPattern::Reduce));
    }
    #[test]
    pub(super) fn pattern_detector_non_recursive_is_none() {
        let decl = mk_decl("just_ret", simple_return(), false);
        let pat = PatternDetector::new(&decl).detect();
        assert!(pat.is_none());
    }
    #[test]
    pub(super) fn pattern_detector_parallel_for_fallback() {
        let decl = mk_decl("generic_loop", tail_call_body(), true);
        let pat = PatternDetector::new(&decl).detect();
        assert!(pat.is_some());
    }
    #[test]
    pub(super) fn dependence_analyser_empty_body_no_deps() {
        let decl = mk_decl("simple", simple_return(), false);
        let info = DependenceAnalyser::new(&decl).analyse();
        assert!(info.is_parallelizable());
    }
    #[test]
    pub(super) fn dependence_analyser_reduce_body() {
        let decl = mk_decl("reduce", reduce_body(), true);
        let info = DependenceAnalyser::new(&decl).analyse();
        let _ = info.is_parallelizable();
    }
    #[test]
    pub(super) fn parallel_pass_empty_decls() {
        let mut pass = ParallelPass::default_pass();
        let mut decls: Vec<LcnfFunDecl> = vec![];
        pass.run(&mut decls);
        let report = pass.report();
        assert_eq!(report.regions_found, 0);
        assert_eq!(report.regions_transformed, 0);
    }
    #[test]
    pub(super) fn parallel_pass_report_default_speedup() {
        let pass = ParallelPass::default_pass();
        let report = pass.report();
        assert_eq!(report.estimated_total_speedup, 1.0);
    }
    #[test]
    pub(super) fn parallel_pass_finds_reduce_region() {
        let mut pass = ParallelPass::default_pass();
        let decl = mk_decl("sum_loop", reduce_body(), true);
        let mut decls = vec![decl];
        pass.analyze_parallelism(&decls.clone());
        let _ = pass.report();
        pass.transform_to_parallel(&mut decls);
    }
    #[test]
    pub(super) fn parallel_report_display() {
        let r = ParallelReport {
            regions_found: 3,
            regions_transformed: 2,
            estimated_total_speedup: 5.5,
            race_conditions_detected: 0,
        };
        let s = r.to_string();
        assert!(s.contains("found=3"));
        assert!(s.contains("transformed=2"));
    }
    #[test]
    pub(super) fn parallel_config_default() {
        let cfg = ParallelConfig::default();
        assert!((cfg.min_speedup_threshold - 1.5).abs() < 1e-9);
        assert_eq!(cfg.hardware_threads, 8);
        assert!(!cfg.allow_speculative);
    }
    #[test]
    pub(super) fn gcd_basic() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(7, 3), 1);
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(5, 0), 5);
    }
}
#[cfg(test)]
mod tests_opar_extra {
    use super::*;
    #[test]
    pub(super) fn test_opar_config() {
        let mut cfg = OParConfig::new();
        cfg.set("mode", "release");
        cfg.set("verbose", "true");
        assert_eq!(cfg.get("mode"), Some("release"));
        assert!(cfg.get_bool("verbose"));
        assert!(cfg.get_int("mode").is_none());
        assert_eq!(cfg.len(), 2);
    }
    #[test]
    pub(super) fn test_opar_source_buffer() {
        let mut buf = OParSourceBuffer::new();
        buf.push_line("fn main() {");
        buf.indent();
        buf.push_line("println!(\"hello\");");
        buf.dedent();
        buf.push_line("}");
        assert!(buf.as_str().contains("fn main()"));
        assert!(buf.as_str().contains("    println!"));
        assert_eq!(buf.line_count(), 3);
        buf.reset();
        assert!(buf.is_empty());
    }
    #[test]
    pub(super) fn test_opar_name_scope() {
        let mut scope = OParNameScope::new();
        assert!(scope.declare("x"));
        assert!(!scope.declare("x"));
        assert!(scope.is_declared("x"));
        let scope = scope.push_scope();
        assert_eq!(scope.depth(), 1);
        let mut scope = scope.pop_scope();
        assert_eq!(scope.depth(), 0);
        scope.declare("y");
        assert_eq!(scope.len(), 2);
    }
    #[test]
    pub(super) fn test_opar_diag_collector() {
        let mut col = OParDiagCollector::new();
        col.emit(OParDiagMsg::warning("pass_a", "slow"));
        col.emit(OParDiagMsg::error("pass_b", "fatal"));
        assert!(col.has_errors());
        assert_eq!(col.errors().len(), 1);
        assert_eq!(col.warnings().len(), 1);
        col.clear();
        assert!(col.is_empty());
    }
    #[test]
    pub(super) fn test_opar_id_gen() {
        let mut gen = OParIdGen::new();
        assert_eq!(gen.next_id(), 0);
        assert_eq!(gen.next_id(), 1);
        gen.skip(10);
        assert_eq!(gen.next_id(), 12);
        gen.reset();
        assert_eq!(gen.peek_next(), 0);
    }
    #[test]
    pub(super) fn test_opar_incr_key() {
        let k1 = OParIncrKey::new(100, 200);
        let k2 = OParIncrKey::new(100, 200);
        let k3 = OParIncrKey::new(999, 200);
        assert!(k1.matches(&k2));
        assert!(!k1.matches(&k3));
    }
    #[test]
    pub(super) fn test_opar_profiler() {
        let mut p = OParProfiler::new();
        p.record(OParPassTiming::new("pass_a", 1000, 50, 200, 100));
        p.record(OParPassTiming::new("pass_b", 500, 30, 100, 200));
        assert_eq!(p.total_elapsed_us(), 1500);
        assert_eq!(
            p.slowest_pass()
                .expect("slowest pass should exist")
                .pass_name,
            "pass_a"
        );
        assert_eq!(p.profitable_passes().len(), 1);
    }
    #[test]
    pub(super) fn test_opar_event_log() {
        let mut log = OParEventLog::new(3);
        log.push("event1");
        log.push("event2");
        log.push("event3");
        assert_eq!(log.len(), 3);
        log.push("event4");
        assert_eq!(log.len(), 3);
        assert_eq!(
            log.iter()
                .next()
                .expect("iterator should have next element"),
            "event2"
        );
    }
    #[test]
    pub(super) fn test_opar_version() {
        let v = OParVersion::new(1, 2, 3).with_pre("alpha");
        assert!(!v.is_stable());
        assert_eq!(format!("{}", v), "1.2.3-alpha");
        let stable = OParVersion::new(2, 0, 0);
        assert!(stable.is_stable());
        assert!(stable.is_compatible_with(&OParVersion::new(2, 0, 0)));
        assert!(!stable.is_compatible_with(&OParVersion::new(3, 0, 0)));
    }
    #[test]
    pub(super) fn test_opar_features() {
        let mut f = OParFeatures::new();
        f.enable("sse2");
        f.enable("avx2");
        assert!(f.is_enabled("sse2"));
        assert!(!f.is_enabled("avx512"));
        f.disable("avx2");
        assert!(!f.is_enabled("avx2"));
        let mut g = OParFeatures::new();
        g.enable("sse2");
        g.enable("neon");
        let union = f.union(&g);
        assert!(union.is_enabled("sse2") && union.is_enabled("neon"));
        let inter = f.intersection(&g);
        assert!(inter.is_enabled("sse2"));
    }
    #[test]
    pub(super) fn test_opar_emit_stats() {
        let mut s = OParEmitStats::new();
        s.bytes_emitted = 50_000;
        s.items_emitted = 500;
        s.elapsed_ms = 100;
        assert!(s.is_clean());
        assert!((s.throughput_bps() - 500_000.0).abs() < 1.0);
        let disp = format!("{}", s);
        assert!(disp.contains("bytes=50000"));
    }
}
#[cfg(test)]
mod Par_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = ParPassConfig::new("test_pass", ParPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = ParPassStats::new();
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
        let mut reg = ParPassRegistry::new();
        reg.register(ParPassConfig::new("pass_a", ParPassPhase::Analysis));
        reg.register(ParPassConfig::new("pass_b", ParPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = ParAnalysisCache::new(10);
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
        let mut wl = ParWorklist::new();
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
        let mut dt = ParDominatorTree::new(5);
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
        let mut liveness = ParLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(ParConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(ParConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(ParConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            ParConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(ParConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = ParDepGraph::new();
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
mod oparext_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_oparext_phase_order() {
        assert_eq!(OParExtPassPhase::Early.order(), 0);
        assert_eq!(OParExtPassPhase::Middle.order(), 1);
        assert_eq!(OParExtPassPhase::Late.order(), 2);
        assert_eq!(OParExtPassPhase::Finalize.order(), 3);
        assert!(OParExtPassPhase::Early.is_early());
        assert!(!OParExtPassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_oparext_config_builder() {
        let c = OParExtPassConfig::new("p")
            .with_phase(OParExtPassPhase::Late)
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
    pub(super) fn test_oparext_stats() {
        let mut s = OParExtPassStats::new();
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
    pub(super) fn test_oparext_registry() {
        let mut r = OParExtPassRegistry::new();
        r.register(OParExtPassConfig::new("a").with_phase(OParExtPassPhase::Early));
        r.register(OParExtPassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&OParExtPassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_oparext_cache() {
        let mut c = OParExtCache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_oparext_worklist() {
        let mut w = OParExtWorklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_oparext_dom_tree() {
        let mut dt = OParExtDomTree::new(5);
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
    pub(super) fn test_oparext_liveness() {
        let mut lv = OParExtLiveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_oparext_const_folder() {
        let mut cf = OParExtConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_oparext_dep_graph() {
        let mut g = OParExtDepGraph::new(4);
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
