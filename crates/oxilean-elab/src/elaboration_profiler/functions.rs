//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    CallGraph, CallNode, DeclProfile, DeclSpanProfile, ElabHotPathReport, ElabPhase, ElabProfiler,
    ElabSpan, ExtendedProfileReport, FlameFrame, FlamegraphBuilder, Hotspot, HotspotDetector,
    MemSnapshot, MemSnapshotDiff, MemTracker, PhaseTimer, ProfileComparison, ProfileReport,
    ProfilingEvent, ProfilingEventLog, SamplingProfiler, TacticBlockProfile, TacticProfile,
    TimingBreakdown, UnifKind, UnificationProfiler, UnificationRecord,
};
use oxilean_kernel::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elaboration_profiler::*;
    #[test]
    fn test_phase_timer() {
        let pt = PhaseTimer::new(ElabPhase::Parsing, 2_000_000, 3);
        assert_eq!(pt.phase, ElabPhase::Parsing);
        assert_eq!(pt.duration_ns, 2_000_000);
        assert_eq!(pt.call_count, 3);
        assert!((pt.duration_ms() - 2.0).abs() < 1e-9);
    }
    #[test]
    fn test_decl_profile_new() {
        let dp = DeclProfile::new("MyThm");
        assert_eq!(dp.name, "MyThm");
        assert_eq!(dp.total_ns, 0);
        assert!(dp.phases.is_empty());
        assert_eq!(dp.num_metavars, 0);
        assert_eq!(dp.num_goals_created, 0);
    }
    #[test]
    fn test_decl_profile_summary() {
        let mut dp = DeclProfile::new("Foo");
        dp.add_phase(ElabPhase::TypeInference, 5_000_000);
        dp.add_phase(ElabPhase::Unification, 1_000_000);
        dp.num_metavars = 4;
        dp.num_goals_created = 2;
        let s = dp.summary();
        assert!(s.contains("Foo"));
        assert!(s.contains("TypeInference"));
    }
    #[test]
    fn test_elab_profiler_new() {
        let ep = ElabProfiler::new();
        assert!(!ep.enabled);
        assert!(ep.decl_profiles.is_empty());
        assert_eq!(ep.total_time_ns, 0);
    }
    #[test]
    fn test_elab_profiler_record() {
        let mut ep = ElabProfiler::new();
        ep.enable();
        let idx = ep.start_decl("Bar");
        ep.record_phase(idx, ElabPhase::TacticEval, 3_000_000);
        ep.finish_decl(idx);
        assert_eq!(ep.total_time_ns, 3_000_000);
        assert_eq!(ep.decl_profiles[idx].phases.len(), 1);
    }
    #[test]
    fn test_profile_report() {
        let mut ep = ElabProfiler::new();
        ep.enable();
        let i1 = ep.start_decl("A");
        ep.record_phase(i1, ElabPhase::TermCheck, 10_000_000);
        ep.finish_decl(i1);
        let i2 = ep.start_decl("B");
        ep.record_phase(i2, ElabPhase::TermCheck, 20_000_000);
        ep.finish_decl(i2);
        let report = ep.report();
        assert_eq!(report.total_decls, 2);
        assert!((report.total_time_ms - 30.0).abs() < 1e-6);
        assert!((report.avg_time_ms - 15.0).abs() < 1e-6);
        assert!((report.max_time_ms - 20.0).abs() < 1e-6);
        let table = report.format_table();
        assert!(table.contains("TermCheck"));
    }
    #[test]
    fn test_sampling_profiler() {
        let mut sp = SamplingProfiler::new(100.0);
        for _ in 0..10 {
            if sp.should_sample() {
                sp.record_sample("TypeInference");
            }
        }
        let hs = sp.hotspots();
        assert!(!hs.is_empty());
        assert_eq!(hs[0].0, "TypeInference");
        assert!((hs[0].1 - 100.0).abs() < 1e-9);
    }
    #[test]
    fn test_top_slow() {
        let mut ep = ElabProfiler::new();
        ep.enable();
        for (name, ns) in &[("A", 1_000), ("B", 5_000), ("C", 2_000)] {
            let i = ep.start_decl(name);
            ep.record_phase(i, ElabPhase::Parsing, *ns);
            ep.finish_decl(i);
        }
        let top2 = ep.top_slow(2);
        assert_eq!(top2.len(), 2);
        assert_eq!(top2[0].name, "B");
        assert_eq!(top2[1].name, "C");
    }
}
#[cfg(test)]
mod extra_tests {
    use super::*;
    use crate::elaboration_profiler::*;
    #[test]
    fn test_elab_phase_names_all() {
        assert_eq!(ElabPhase::Parsing.name(), "Parsing");
        assert_eq!(ElabPhase::TypeInference.name(), "TypeInference");
        assert_eq!(ElabPhase::Unification.name(), "Unification");
        assert_eq!(ElabPhase::InstanceSynthesis.name(), "InstanceSynthesis");
        assert_eq!(ElabPhase::TacticEval.name(), "TacticEval");
        assert_eq!(ElabPhase::PatternMatch.name(), "PatternMatch");
        assert_eq!(ElabPhase::TermCheck.name(), "TermCheck");
    }
    #[test]
    fn test_elab_phase_clone_eq() {
        let p = ElabPhase::TacticEval;
        let p2 = p.clone();
        assert_eq!(p, p2);
        assert_ne!(p, ElabPhase::Parsing);
    }
    #[test]
    fn test_decl_profile_add_phase_accumulation() {
        let mut dp = DeclProfile::new("Thm");
        dp.add_phase(ElabPhase::Parsing, 1_000);
        dp.add_phase(ElabPhase::Parsing, 2_000);
        assert_eq!(dp.phases.len(), 1);
        assert_eq!(dp.phases[0].duration_ns, 3_000);
        assert_eq!(dp.phases[0].call_count, 2);
        assert_eq!(dp.total_ns, 3_000);
    }
    #[test]
    fn test_decl_profile_slowest_phase() {
        let mut dp = DeclProfile::new("X");
        dp.add_phase(ElabPhase::Parsing, 100);
        dp.add_phase(ElabPhase::TermCheck, 9_000);
        dp.add_phase(ElabPhase::Unification, 500);
        let slowest = dp.slowest_phase().expect("test operation should succeed");
        assert_eq!(slowest.phase, ElabPhase::TermCheck);
    }
    #[test]
    fn test_decl_profile_no_phases_slowest_none() {
        let dp = DeclProfile::new("Empty");
        assert!(dp.slowest_phase().is_none());
        let s = dp.summary();
        assert!(s.contains("Empty"));
    }
    #[test]
    fn test_decl_profile_total_ms() {
        let mut dp = DeclProfile::new("X");
        dp.add_phase(ElabPhase::Parsing, 2_000_000);
        dp.add_phase(ElabPhase::Unification, 3_000_000);
        assert!((dp.total_ms() - 5.0).abs() < 1e-9);
    }
    #[test]
    fn test_phase_timer_clone() {
        let pt = PhaseTimer::new(ElabPhase::Unification, 5_000, 2);
        let pt2 = pt.clone();
        assert_eq!(pt2.duration_ns, 5_000);
        assert_eq!(pt2.call_count, 2);
    }
    #[test]
    fn test_phase_timer_duration_ms_precision() {
        let pt = PhaseTimer::new(ElabPhase::TermCheck, 1_500_000, 1);
        assert!((pt.duration_ms() - 1.5).abs() < 1e-9);
    }
    #[test]
    fn test_profiler_disabled_does_not_record() {
        let mut ep = ElabProfiler::new();
        let idx = ep.start_decl("NoRecord");
        ep.record_phase(idx, ElabPhase::Parsing, 999_000);
        ep.finish_decl(idx);
        assert_eq!(ep.total_time_ns, 0);
        assert!(ep.decl_profiles[idx].phases.is_empty());
    }
    #[test]
    fn test_profiler_enable_disable() {
        let mut ep = ElabProfiler::new();
        ep.enable();
        assert!(ep.enabled);
        ep.disable();
        assert!(!ep.enabled);
    }
    #[test]
    fn test_profiler_reset() {
        let mut ep = ElabProfiler::new();
        ep.enable();
        let i = ep.start_decl("A");
        ep.record_phase(i, ElabPhase::Parsing, 1_000_000);
        ep.finish_decl(i);
        ep.reset();
        assert!(ep.decl_profiles.is_empty());
        assert_eq!(ep.total_time_ns, 0);
    }
    #[test]
    fn test_profiler_record_out_of_bounds_no_panic() {
        let mut ep = ElabProfiler::new();
        ep.enable();
        ep.record_phase(99, ElabPhase::Parsing, 1_000);
    }
    #[test]
    fn test_profile_report_phase_totals() {
        let mut ep = ElabProfiler::new();
        ep.enable();
        let i1 = ep.start_decl("X");
        ep.record_phase(i1, ElabPhase::Parsing, 1_000_000);
        ep.record_phase(i1, ElabPhase::Unification, 2_000_000);
        ep.finish_decl(i1);
        let i2 = ep.start_decl("Y");
        ep.record_phase(i2, ElabPhase::Parsing, 3_000_000);
        ep.finish_decl(i2);
        let report = ep.report();
        let parsing_total = report.phase_totals.get("Parsing").copied().unwrap_or(0.0);
        assert!((parsing_total - 4.0).abs() < 1e-6);
        let table = report.format_table();
        assert!(table.contains("Parsing"));
        assert!(table.contains("Unification"));
    }
    #[test]
    fn test_profile_report_default() {
        let report = ProfileReport::default();
        assert_eq!(report.total_decls, 0);
        let table = report.format_table();
        assert!(table.contains("Declarations"));
    }
    #[test]
    fn test_profiler_top_slow_fewer_than_n() {
        let mut ep = ElabProfiler::new();
        ep.enable();
        let i = ep.start_decl("OnlyOne");
        ep.record_phase(i, ElabPhase::Parsing, 1_000_000);
        ep.finish_decl(i);
        let top = ep.top_slow(5);
        assert_eq!(top.len(), 1);
    }
    #[test]
    fn test_sampling_profiler_no_samples() {
        let sp = SamplingProfiler::new(10.0);
        let hs = sp.hotspots();
        assert!(hs.is_empty());
    }
    #[test]
    fn test_sampling_profiler_multiple_phases() {
        let mut sp = SamplingProfiler::new(1000.0);
        for _ in 0..3 {
            sp.record_sample("TypeInference");
        }
        for _ in 0..1 {
            sp.record_sample("Parsing");
        }
        let hs = sp.hotspots();
        assert_eq!(hs[0].0, "TypeInference");
        assert!((hs[0].1 - 75.0).abs() < 1e-9);
    }
    #[test]
    fn test_sampling_profiler_record_increments_total() {
        let mut sp = SamplingProfiler::new(1000.0);
        sp.record_sample("Parsing");
        sp.record_sample("Parsing");
        sp.record_sample("TypeInference");
        assert_eq!(sp.total_samples, 3);
    }
}
#[cfg(test)]
mod profiler_extended_tests {
    use super::*;
    use crate::elaboration_profiler::*;
    #[test]
    fn test_elab_span_basic() {
        let s = ElabSpan::new(10, 20);
        assert_eq!(s.start, 10);
        assert_eq!(s.end, 20);
        assert_eq!(s.len(), 10);
        assert!(!s.is_empty());
        assert!(s.file.is_none());
    }
    #[test]
    fn test_elab_span_with_file() {
        let s = ElabSpan::with_file(0, 5, "Main.lean");
        assert_eq!(s.file.as_deref(), Some("Main.lean"));
    }
    #[test]
    fn test_elab_span_contains() {
        let s = ElabSpan::new(5, 15);
        assert!(s.contains(5));
        assert!(s.contains(14));
        assert!(!s.contains(15));
        assert!(!s.contains(4));
    }
    #[test]
    fn test_elab_span_overlaps() {
        let a = ElabSpan::new(0, 10);
        let b = ElabSpan::new(5, 20);
        let c = ElabSpan::new(10, 20);
        assert!(a.overlaps(&b));
        assert!(!a.overlaps(&c));
    }
    #[test]
    fn test_elab_span_empty() {
        let s = ElabSpan::new(5, 5);
        assert!(s.is_empty());
        assert_eq!(s.len(), 0);
    }
    #[test]
    fn test_elab_span_display() {
        let s = ElabSpan::new(0, 10);
        let d = format!("{}", s);
        assert!(d.contains("0"));
        assert!(d.contains("10"));
        let sf = ElabSpan::with_file(0, 10, "foo.lean");
        let df = format!("{}", sf);
        assert!(df.contains("foo.lean"));
    }
    #[test]
    fn test_elab_span_clone_eq() {
        let s = ElabSpan::new(1, 2);
        let s2 = s.clone();
        assert_eq!(s, s2);
    }
    #[test]
    fn test_decl_span_profile_basic() {
        let span = ElabSpan::new(0, 100);
        let mut p = DeclSpanProfile::new("Foo", span);
        assert_eq!(p.name, "Foo");
        assert_eq!(p.total_ns, 0);
        p.record_phase_ns("Parsing", 1_000_000);
        p.record_phase_ns("Unification", 2_000_000);
        assert_eq!(p.total_ns, 3_000_000);
        assert!((p.total_ms() - 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_decl_span_profile_ns_per_byte() {
        let span = ElabSpan::new(0, 100);
        let mut p = DeclSpanProfile::new("Bar", span);
        p.record_phase_ns("X", 1_000);
        assert!((p.ns_per_byte() - 10.0).abs() < 1e-9);
    }
    #[test]
    fn test_decl_span_profile_ns_per_byte_zero_span() {
        let span = ElabSpan::new(5, 5);
        let mut p = DeclSpanProfile::new("Z", span);
        p.record_phase_ns("X", 1_000);
        assert_eq!(p.ns_per_byte(), 0.0);
    }
    #[test]
    fn test_decl_span_profile_dominant_phase() {
        let span = ElabSpan::new(0, 50);
        let mut p = DeclSpanProfile::new("T", span);
        p.record_phase_ns("Parsing", 100);
        p.record_phase_ns("TermCheck", 9_000);
        let dom = p.dominant_phase().expect("test operation should succeed");
        assert_eq!(dom.0, "TermCheck");
    }
    #[test]
    fn test_tactic_profile_goals_closed() {
        let tp = TacticProfile::new("intro", 1_000, 3, 2, true);
        assert_eq!(tp.goals_closed(), 1);
    }
    #[test]
    fn test_tactic_profile_goals_opened() {
        let tp = TacticProfile::new("apply", 2_000, 1, 3, true);
        assert_eq!(tp.goals_closed(), -2);
    }
    #[test]
    fn test_tactic_profile_efficiency_zero_time() {
        let tp = TacticProfile::new("refl", 0, 1, 0, true);
        assert_eq!(tp.efficiency(), 0.0);
    }
    #[test]
    fn test_tactic_profile_efficiency_positive() {
        let tp = TacticProfile::new("exact", 1_000_000, 2, 1, true);
        assert!((tp.efficiency() - 1.0).abs() < 1e-9);
    }
    #[test]
    fn test_tactic_block_profile_empty() {
        let bp = TacticBlockProfile::new();
        assert_eq!(bp.step_count(), 0);
        assert_eq!(bp.total_ns, 0);
        assert!(!bp.completed);
        assert_eq!(bp.avg_step_ns(), 0.0);
        assert!(bp.slowest_step().is_none());
    }
    #[test]
    fn test_tactic_block_profile_add_steps() {
        let mut bp = TacticBlockProfile::new();
        bp.add_step(TacticProfile::new("intro", 1_000, 1, 0, true));
        bp.add_step(TacticProfile::new("exact", 3_000, 1, 0, true));
        assert_eq!(bp.step_count(), 2);
        assert_eq!(bp.total_ns, 4_000);
        assert_eq!(bp.successful_steps(), 2);
        let slowest = bp.slowest_step().expect("test operation should succeed");
        assert_eq!(slowest.tactic_name, "exact");
    }
    #[test]
    fn test_tactic_block_profile_time_by_tactic() {
        let mut bp = TacticBlockProfile::new();
        bp.add_step(TacticProfile::new("intro", 100, 1, 0, true));
        bp.add_step(TacticProfile::new("intro", 200, 1, 0, true));
        bp.add_step(TacticProfile::new("simp", 500, 1, 0, true));
        let by_tac = bp.time_by_tactic();
        assert_eq!(*by_tac.get("intro").expect("key should exist"), 300);
        assert_eq!(*by_tac.get("simp").expect("key should exist"), 500);
    }
    #[test]
    fn test_tactic_block_profile_count_by_tactic() {
        let mut bp = TacticBlockProfile::new();
        bp.add_step(TacticProfile::new("rw", 100, 1, 1, true));
        bp.add_step(TacticProfile::new("rw", 200, 1, 1, true));
        bp.add_step(TacticProfile::new("exact", 50, 1, 0, true));
        let counts = bp.count_by_tactic();
        assert_eq!(*counts.get("rw").expect("key should exist"), 2);
        assert_eq!(*counts.get("exact").expect("key should exist"), 1);
    }
    #[test]
    fn test_tactic_block_total_goals_closed() {
        let mut bp = TacticBlockProfile::new();
        bp.add_step(TacticProfile::new("constructor", 1_000, 1, 2, true));
        bp.add_step(TacticProfile::new("refl", 500, 1, 0, true));
        bp.add_step(TacticProfile::new("refl", 500, 1, 0, true));
        assert_eq!(bp.total_goals_closed(), 1);
    }
    #[test]
    fn test_unif_kind_labels() {
        assert_eq!(UnifKind::TypeEq.label(), "TypeEq");
        assert_eq!(UnifKind::TermEq.label(), "TermEq");
        assert_eq!(UnifKind::Subtype.label(), "Subtype");
        assert_eq!(UnifKind::Instance.label(), "Instance");
    }
    #[test]
    fn test_unif_kind_clone_eq() {
        let k = UnifKind::TypeEq;
        assert_eq!(k.clone(), UnifKind::TypeEq);
        assert_ne!(k, UnifKind::TermEq);
    }
    #[test]
    fn test_unification_profiler_empty() {
        let p = UnificationProfiler::new();
        assert_eq!(p.count(), 0);
        assert_eq!(p.successful(), 0);
        assert_eq!(p.failed(), 0);
        assert_eq!(p.success_rate(), 1.0);
        assert_eq!(p.total_ms(), 0.0);
    }
    #[test]
    fn test_unification_profiler_add() {
        let mut p = UnificationProfiler::new();
        p.add(UnificationRecord::new(
            UnifKind::TypeEq,
            "A",
            "B",
            1_000,
            true,
        ));
        p.add(UnificationRecord::new(
            UnifKind::TermEq,
            "C",
            "D",
            2_000,
            false,
        ));
        assert_eq!(p.count(), 2);
        assert_eq!(p.successful(), 1);
        assert_eq!(p.failed(), 1);
        assert!((p.success_rate() - 0.5).abs() < 1e-9);
        assert!((p.total_ms() - 0.003).abs() < 1e-6);
    }
    #[test]
    fn test_unification_profiler_top_slow() {
        let mut p = UnificationProfiler::new();
        p.add(UnificationRecord::new(
            UnifKind::TypeEq,
            "X",
            "Y",
            100,
            true,
        ));
        p.add(UnificationRecord::new(
            UnifKind::TypeEq,
            "A",
            "B",
            9_000,
            true,
        ));
        p.add(UnificationRecord::new(
            UnifKind::TypeEq,
            "C",
            "D",
            500,
            true,
        ));
        let top1 = p.top_slow(1);
        assert_eq!(top1[0].lhs, "A");
    }
    #[test]
    fn test_unification_profiler_time_by_kind() {
        let mut p = UnificationProfiler::new();
        p.add(UnificationRecord::new(
            UnifKind::TypeEq,
            "A",
            "B",
            1_000,
            true,
        ));
        p.add(UnificationRecord::new(
            UnifKind::TypeEq,
            "C",
            "D",
            2_000,
            true,
        ));
        p.add(UnificationRecord::new(
            UnifKind::Instance,
            "E",
            "F",
            500,
            true,
        ));
        let by_kind = p.time_by_kind();
        assert_eq!(*by_kind.get("TypeEq").expect("key should exist"), 3_000);
        assert_eq!(*by_kind.get("Instance").expect("key should exist"), 500);
    }
    #[test]
    fn test_unification_profiler_count_by_kind() {
        let mut p = UnificationProfiler::new();
        p.add(UnificationRecord::new(
            UnifKind::Subtype,
            "A",
            "B",
            100,
            true,
        ));
        p.add(UnificationRecord::new(
            UnifKind::Subtype,
            "C",
            "D",
            200,
            true,
        ));
        let by_kind = p.count_by_kind();
        assert_eq!(*by_kind.get("Subtype").expect("key should exist"), 2);
    }
    #[test]
    fn test_mem_snapshot_new() {
        let s = MemSnapshot::new("start", 100, 4096);
        assert_eq!(s.live_objects, 100);
        assert_eq!(s.allocated_bytes, 4096);
        assert_eq!(s.label, "start");
    }
    #[test]
    fn test_mem_snapshot_diff_compute() {
        let a = MemSnapshot::new("before", 100, 4096);
        let b = MemSnapshot::new("after", 150, 8192);
        let diff = MemSnapshotDiff::compute(&a, &b);
        assert_eq!(diff.delta_objects, 50);
        assert_eq!(diff.delta_bytes, 4096);
        assert!(diff.is_growth());
        assert!(!diff.is_shrinkage());
    }
    #[test]
    fn test_mem_snapshot_diff_shrinkage() {
        let a = MemSnapshot::new("a", 200, 8192);
        let b = MemSnapshot::new("b", 100, 4096);
        let diff = MemSnapshotDiff::compute(&a, &b);
        assert!(diff.is_shrinkage());
        assert!(!diff.is_growth());
    }
    #[test]
    fn test_mem_snapshot_diff_display() {
        let a = MemSnapshot::new("before", 10, 100);
        let b = MemSnapshot::new("after", 20, 200);
        let diff = MemSnapshotDiff::compute(&a, &b);
        let d = format!("{}", diff);
        assert!(d.contains("before"));
        assert!(d.contains("after"));
        assert!(d.contains("+100"));
    }
    #[test]
    fn test_mem_tracker_empty() {
        let t = MemTracker::new();
        assert!(t.is_empty());
        assert_eq!(t.len(), 0);
        assert!(t.diffs().is_empty());
    }
    #[test]
    fn test_mem_tracker_snapshots_and_diffs() {
        let mut t = MemTracker::new();
        t.snapshot(MemSnapshot::new("s0", 10, 100));
        t.snapshot(MemSnapshot::new("s1", 20, 200));
        t.snapshot(MemSnapshot::new("s2", 15, 150));
        assert_eq!(t.len(), 3);
        let diffs = t.diffs();
        assert_eq!(diffs.len(), 2);
        assert_eq!(diffs[0].delta_bytes, 100);
        assert_eq!(diffs[1].delta_bytes, -50);
        assert_eq!(t.total_growth_bytes(), 100);
    }
    #[test]
    fn test_call_node_record_call() {
        let mut n = CallNode::new("elaborate");
        n.record_call(5_000_000, 2_000_000);
        assert_eq!(n.call_count, 1);
        assert_eq!(n.inclusive_ns, 5_000_000);
        assert_eq!(n.exclusive_ns, 2_000_000);
        assert!((n.inclusive_ms() - 5.0).abs() < 1e-9);
    }
    #[test]
    fn test_call_graph_add_node_and_edge() {
        let mut g = CallGraph::new();
        let a = g.add_node("A");
        let b = g.add_node("B");
        let c = g.add_node("C");
        g.add_edge(a, b);
        g.add_edge(a, c);
        assert_eq!(g.node_count(), 3);
        assert_eq!(g.nodes[a].children, vec![b, c]);
        assert!(g.nodes[b].children.is_empty());
    }
    #[test]
    fn test_call_graph_find_node() {
        let mut g = CallGraph::new();
        g.add_node("foo");
        g.add_node("bar");
        assert_eq!(g.find_node("foo"), Some(0));
        assert_eq!(g.find_node("bar"), Some(1));
        assert_eq!(g.find_node("baz"), None);
    }
    #[test]
    fn test_call_graph_dfs_visit() {
        let mut g = CallGraph::new();
        let a = g.add_node("A");
        let b = g.add_node("B");
        let c = g.add_node("C");
        g.add_edge(a, b);
        g.add_edge(b, c);
        let mut visited = Vec::new();
        g.dfs_visit(a, &mut |idx| visited.push(idx));
        assert_eq!(visited, vec![0, 1, 2]);
    }
    #[test]
    fn test_call_graph_reachable_from() {
        let mut g = CallGraph::new();
        let a = g.add_node("A");
        let b = g.add_node("B");
        let c = g.add_node("C");
        let d = g.add_node("D");
        g.add_edge(a, b);
        g.add_edge(a, c);
        let reachable = g.reachable_from(a);
        assert!(reachable.contains(&a));
        assert!(reachable.contains(&b));
        assert!(reachable.contains(&c));
        assert!(!reachable.contains(&d));
    }
    #[test]
    fn test_call_graph_top_by_inclusive() {
        let mut g = CallGraph::new();
        let a = g.add_node("A");
        let b = g.add_node("B");
        g.nodes[a].inclusive_ns = 100;
        g.nodes[b].inclusive_ns = 500;
        let top = g.top_by_inclusive(1);
        assert_eq!(top[0].name, "B");
    }
    #[test]
    fn test_flame_frame_fraction() {
        let f = FlameFrame::new("foo", 100, 500, 0);
        assert!((f.fraction(1000) - 0.5).abs() < 1e-9);
        assert_eq!(f.fraction(0), 0.0);
    }
    #[test]
    fn test_flamegraph_builder_empty_graph() {
        let mut b = FlamegraphBuilder::new();
        let g = CallGraph::new();
        b.build_from_graph(&g, 0);
        assert_eq!(b.frame_count(), 0);
    }
    #[test]
    fn test_flamegraph_builder_simple() {
        let mut g = CallGraph::new();
        let a = g.add_node("root");
        let b = g.add_node("child");
        g.add_edge(a, b);
        g.nodes[a].inclusive_ns = 1_000;
        g.nodes[a].exclusive_ns = 500;
        g.nodes[b].inclusive_ns = 500;
        g.nodes[b].exclusive_ns = 500;
        let mut fb = FlamegraphBuilder::new();
        fb.build_from_graph(&g, a);
        assert_eq!(fb.frame_count(), 2);
    }
    #[test]
    fn test_flamegraph_collapsed_text() {
        let mut g = CallGraph::new();
        let a = g.add_node("root");
        let b = g.add_node("child");
        g.add_edge(a, b);
        g.nodes[a].inclusive_ns = 1_000;
        g.nodes[a].exclusive_ns = 400;
        g.nodes[b].inclusive_ns = 600;
        g.nodes[b].exclusive_ns = 600;
        let mut fb = FlamegraphBuilder::new();
        fb.build_from_graph(&g, a);
        let text = fb.to_collapsed_text();
        assert!(text.contains("root"));
        assert!(text.contains("child"));
    }
    #[test]
    fn test_hotspot_detector_no_data() {
        let det = HotspotDetector::new(10.0);
        let hs = det.detect(&[]);
        assert!(hs.is_empty());
    }
    #[test]
    fn test_hotspot_detector_below_threshold() {
        let det = HotspotDetector::new(50.0);
        let measurements = vec![
            ("Parsing".to_string(), 40u64),
            ("TermCheck".to_string(), 60u64),
        ];
        let hs = det.detect(&measurements);
        assert_eq!(hs.len(), 1);
        assert_eq!(hs[0].name, "TermCheck");
    }
    #[test]
    fn test_hotspot_detector_critical_recommendation() {
        let det = HotspotDetector::new(10.0);
        let measurements = vec![
            ("TermCheck".to_string(), 900u64),
            ("Parsing".to_string(), 100u64),
        ];
        let hs = det.detect(&measurements);
        assert!(!hs.is_empty());
        let critical = hs
            .iter()
            .find(|h| h.name == "TermCheck")
            .expect("find should succeed");
        assert!(critical.recommendation.contains("Critical"));
    }
    #[test]
    fn test_hotspot_detector_zero_total() {
        let det = HotspotDetector::new(10.0);
        let measurements = vec![("X".to_string(), 0u64)];
        let hs = det.detect(&measurements);
        assert!(hs.is_empty());
    }
    #[test]
    fn test_profile_comparison_improvement() {
        let mut baseline = ProfileReport::new();
        baseline.total_time_ms = 100.0;
        baseline.phase_totals.insert("Parsing".to_string(), 60.0);
        baseline.phase_totals.insert("TermCheck".to_string(), 40.0);
        let mut candidate = ProfileReport::new();
        candidate.total_time_ms = 50.0;
        candidate.phase_totals.insert("Parsing".to_string(), 30.0);
        candidate.phase_totals.insert("TermCheck".to_string(), 20.0);
        let cmp = ProfileComparison::compare("v1", &baseline, "v2", &candidate);
        assert!(cmp.is_improvement);
        assert!((cmp.speedup - 2.0).abs() < 1e-9);
        assert!(cmp.regressions().is_empty());
        assert_eq!(cmp.improvements().len(), 2);
    }
    #[test]
    fn test_profile_comparison_regression() {
        let mut baseline = ProfileReport::new();
        baseline.total_time_ms = 50.0;
        let mut candidate = ProfileReport::new();
        candidate.total_time_ms = 100.0;
        let cmp = ProfileComparison::compare("v1", &baseline, "v2", &candidate);
        assert!(!cmp.is_improvement);
        assert!((cmp.speedup - 0.5).abs() < 1e-9);
    }
    #[test]
    fn test_profile_comparison_summary() {
        let baseline = ProfileReport::new();
        let candidate = ProfileReport::new();
        let cmp = ProfileComparison::compare("base", &baseline, "cand", &candidate);
        let s = cmp.summary();
        assert!(s.contains("base"));
        assert!(s.contains("cand"));
    }
    #[test]
    fn test_profile_comparison_phase_regressions() {
        let mut baseline = ProfileReport::new();
        baseline.total_time_ms = 100.0;
        baseline.phase_totals.insert("Parsing".to_string(), 10.0);
        let mut candidate = ProfileReport::new();
        candidate.total_time_ms = 90.0;
        candidate.phase_totals.insert("Parsing".to_string(), 20.0);
        let cmp = ProfileComparison::compare("v1", &baseline, "v2", &candidate);
        let regs = cmp.regressions();
        assert!(regs.iter().any(|(name, _)| *name == "Parsing"));
    }
    #[test]
    fn test_extended_profile_report_from_profiler() {
        let mut ep = ElabProfiler::new();
        ep.enable();
        let i = ep.start_decl("TestThm");
        ep.record_phase(i, ElabPhase::TermCheck, 800_000_000);
        ep.record_phase(i, ElabPhase::Parsing, 100_000_000);
        ep.finish_decl(i);
        let ext = ExtendedProfileReport::from_profiler(&ep);
        assert_eq!(ext.base.total_decls, 1);
        assert!(!ext.decl_summaries.is_empty());
    }
    #[test]
    fn test_extended_profile_report_format_text() {
        let mut ep = ElabProfiler::new();
        ep.enable();
        let i = ep.start_decl("Foo");
        ep.record_phase(i, ElabPhase::Parsing, 1_000_000);
        ep.finish_decl(i);
        let ext = ExtendedProfileReport::from_profiler(&ep);
        let text = ext.format_text();
        assert!(text.contains("Declaration"));
        assert!(text.contains("Hotspot") || text.contains("(none)"));
    }
    #[test]
    fn test_extended_profile_report_format_json() {
        let ep = ElabProfiler::new();
        let mut ext = ExtendedProfileReport::from_profiler(&ep);
        ext.total_tactic_steps = 42;
        ext.unif_success_rate = 0.95;
        let json = ext.format_json();
        assert!(json.contains("\"total_tactic_steps\": 42"));
        assert!(json.contains("0.9500"));
    }
    #[test]
    fn test_extended_profile_report_default() {
        let r = ExtendedProfileReport::default();
        assert_eq!(r.base.total_decls, 0);
        assert_eq!(r.total_tactic_steps, 0);
        assert!((r.unif_success_rate - 1.0).abs() < 1e-9);
    }
}
#[cfg(test)]
mod elaboration_profiler_ext_tests {
    use super::*;
    use crate::elaboration_profiler::*;
    #[test]
    fn test_hot_path_report_add_decl_above_threshold() {
        let mut r = ElabHotPathReport::new(1000);
        r.add_decl("expensive", 5000);
        assert_eq!(r.top_decls.len(), 1);
        assert_eq!(r.total_decl_time_ns(), 5000);
    }
    #[test]
    fn test_hot_path_report_add_decl_below_threshold() {
        let mut r = ElabHotPathReport::new(10_000);
        r.add_decl("cheap", 100);
        assert!(r.top_decls.is_empty());
    }
    #[test]
    fn test_hot_path_report_sorted() {
        let mut r = ElabHotPathReport::new(0);
        r.add_decl("slow", 5000);
        r.add_decl("fast", 100);
        r.add_decl("medium", 2000);
        assert_eq!(r.top_decls[0].0, "slow");
        assert_eq!(r.top_decls[1].0, "medium");
    }
    #[test]
    fn test_hot_path_report_format() {
        let mut r = ElabHotPathReport::new(0);
        r.add_decl("check", 1000);
        r.add_tactic("simp", 500);
        let out = r.format();
        assert!(out.contains("check"));
        assert!(out.contains("simp"));
    }
    #[test]
    fn test_profiling_event_new() {
        let e = ProfilingEvent::new("check", ElabPhase::TypeCheck, 1000, 500);
        assert_eq!(e.label, "check");
        assert_eq!(e.duration_ns, 500);
    }
    #[test]
    fn test_profiling_event_log_record() {
        let mut log = ProfilingEventLog::new();
        log.record(ProfilingEvent::new("a", ElabPhase::Elaboration, 0, 100));
        log.record(ProfilingEvent::new("b", ElabPhase::TypeCheck, 100, 200));
        assert_eq!(log.len(), 2);
        assert_eq!(log.total_duration_ns(), 300);
    }
    #[test]
    fn test_profiling_event_log_for_phase() {
        let mut log = ProfilingEventLog::new();
        log.record(ProfilingEvent::new("a", ElabPhase::Elaboration, 0, 100));
        log.record(ProfilingEvent::new("b", ElabPhase::TypeCheck, 100, 200));
        log.record(ProfilingEvent::new("c", ElabPhase::Elaboration, 300, 50));
        let elab = log.for_phase(ElabPhase::Elaboration);
        assert_eq!(elab.len(), 2);
    }
    #[test]
    fn test_profiling_event_log_slowest() {
        let mut log = ProfilingEventLog::new();
        log.record(ProfilingEvent::new("fast", ElabPhase::Elaboration, 0, 10));
        log.record(ProfilingEvent::new("slow", ElabPhase::TypeCheck, 10, 1000));
        let slowest = log.slowest().expect("test operation should succeed");
        assert_eq!(slowest.label, "slow");
    }
    #[test]
    fn test_profiling_event_log_avg() {
        let mut log = ProfilingEventLog::new();
        log.record(ProfilingEvent::new("a", ElabPhase::Elaboration, 0, 100));
        log.record(ProfilingEvent::new("b", ElabPhase::Elaboration, 100, 200));
        assert!((log.avg_duration_ns() - 150.0).abs() < 1e-6);
    }
    #[test]
    fn test_profiling_event_log_empty() {
        let log = ProfilingEventLog::new();
        assert!(log.is_empty());
        assert_eq!(log.total_duration_ns(), 0);
        assert!((log.avg_duration_ns() - 0.0).abs() < 1e-10);
        assert!(log.slowest().is_none());
    }
    #[test]
    fn test_timing_breakdown_total() {
        let t = TimingBreakdown {
            type_check_ns: 1000,
            unification_ns: 500,
            tactic_eval_ns: 200,
            simp_ns: 100,
            instance_synth_ns: 200,
        };
        assert_eq!(t.total_ns(), 2000);
    }
    #[test]
    fn test_timing_breakdown_fraction() {
        let t = TimingBreakdown {
            type_check_ns: 500,
            unification_ns: 500,
            ..Default::default()
        };
        assert!((t.type_check_fraction() - 0.5).abs() < 1e-10);
    }
    #[test]
    fn test_timing_breakdown_zero_total() {
        let t = TimingBreakdown::new();
        assert!((t.type_check_fraction() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_timing_breakdown_summary() {
        let t = TimingBreakdown {
            type_check_ns: 100,
            ..Default::default()
        };
        let s = t.summary();
        assert!(s.contains("typecheck=100ns"));
    }
}
