//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::cell::RefCell;

use super::types::{
    AllocationTracker, AnnotatedTimeline, CallTreeNode, ComprehensiveProfilingReport, CountingStep,
    EventFilter, FlameGraph, FlameNode, GcCollectionRecord, GcProfiler, HeatMap, Histogram,
    PerfCounter, ProfileReport, ProfileSample, Profiler, ProfilerConfig, ProfilingEvent,
    ProfilingMiddleware, ProfilingSession, RealTimeMonitor, SamplingProfiler, StackSnapshot,
    TacticProfileLog, TacticProfilingEvent, TimelineAnnotation, TimelineView,
};

thread_local! {
    #[doc = " Per-thread profiler instance."] static THREAD_PROFILER : RefCell < Profiler
    > = RefCell::new(Profiler::new());
}
/// Enable the thread-local profiler.
pub fn profiler_enable() {
    THREAD_PROFILER.with(|p| p.borrow_mut().enable());
}
/// Disable the thread-local profiler.
pub fn profiler_disable() {
    THREAD_PROFILER.with(|p| p.borrow_mut().disable());
}
/// Generate a report from the thread-local profiler.
pub fn profiler_report() -> ProfileReport {
    THREAD_PROFILER.with(|p| p.borrow().generate_report())
}
/// Record a function entry in the thread-local profiler.
pub fn profiler_enter(name: &str) {
    THREAD_PROFILER.with(|p| p.borrow_mut().enter_function(name));
}
/// Record a function exit in the thread-local profiler.
pub fn profiler_exit(name: &str) {
    THREAD_PROFILER.with(|p| p.borrow_mut().exit_function(name));
}
/// Record an allocation in the thread-local profiler.
pub fn profiler_alloc(size: usize, tag: &str) {
    THREAD_PROFILER.with(|p| p.borrow_mut().alloc(size, tag));
}
/// Record a deallocation in the thread-local profiler.
pub fn profiler_dealloc(size: usize, tag: &str) {
    THREAD_PROFILER.with(|p| p.borrow_mut().dealloc(size, tag));
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_profiler_basic() {
        let mut p = Profiler::new();
        p.enter_function("foo");
        p.exit_function("foo");
        assert!(p.events.is_empty());
        p.enable();
        p.enter_function("bar");
        p.exit_function("bar");
        assert_eq!(p.events.len(), 2);
    }
    #[test]
    fn test_profiler_report() {
        let mut p = Profiler::new();
        p.enable();
        p.enter_function("compute");
        p.exit_function("compute");
        p.enter_function("compute");
        p.exit_function("compute");
        p.alloc(1024, "heap");
        p.alloc(512, "stack");
        let report = p.generate_report();
        assert_eq!(report.total_calls, 2);
        assert_eq!(report.total_alloc_bytes, 1536);
        assert_eq!(report.gc_cycles, 0);
        assert!(!report.hot_functions.is_empty());
        assert_eq!(report.hot_functions[0].0, "compute");
    }
    #[test]
    fn test_memory_profile() {
        let mut p = Profiler::new();
        p.enable();
        p.alloc(100, "a");
        p.alloc(200, "b");
        p.dealloc(100, "a");
        let mem = p.memory_profile();
        assert_eq!(mem.total_allocs, 2);
        assert_eq!(mem.peak_bytes, 300);
        assert_eq!(mem.current_bytes, 200);
        let text = mem.to_text();
        assert!(text.contains("Memory Profile"));
        assert!(text.contains("300"));
    }
    #[test]
    fn test_profiler_disabled() {
        let mut p = Profiler::new();
        p.alloc(1000, "test");
        p.enter_function("ignored");
        p.gc_cycle(10, 50);
        assert!(p.events.is_empty());
        assert!(p.call_stack.is_empty());
        p.enable();
        p.alloc(42, "enabled");
        assert_eq!(p.events.len(), 1);
    }
    #[test]
    fn test_profiler_json() {
        let mut p = Profiler::new();
        p.enable();
        p.enter_function("alpha");
        p.exit_function("alpha");
        p.alloc(256, "buf");
        let report = p.generate_report();
        let json = report.to_json();
        assert!(json.contains("total_calls"));
        assert!(json.contains("total_alloc_bytes"));
        assert!(json.contains("gc_cycles"));
        assert!(json.contains("hot_functions"));
        assert!(json.starts_with('{'));
        assert!(json.ends_with('}'));
    }
    #[test]
    fn test_gc_cycle() {
        let mut p = Profiler::new();
        p.enable();
        p.gc_cycle(50, 200);
        p.gc_cycle(30, 170);
        let report = p.generate_report();
        assert_eq!(report.gc_cycles, 2);
        let text = report.to_text();
        assert!(text.contains("Profile Report"));
        assert!(text.contains("GC cycles"));
    }
}
pub(super) fn profiler_now_ns() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0)
}
#[cfg(test)]
mod profiler_extended_tests {
    use super::*;
    #[test]
    fn test_sampling_profiler_basic() {
        let mut p = SamplingProfiler::new(1_000_000);
        p.enable();
        p.enter("foo");
        p.enter("bar");
        p.take_sample(0);
        assert_eq!(p.sample_count(), 1);
        let sample = &p.samples[0];
        assert_eq!(sample.top_function(), Some("bar"));
        assert_eq!(sample.depth(), 2);
    }
    #[test]
    fn test_flat_profile() {
        let mut p = SamplingProfiler::new(1_000_000);
        p.enable();
        for _ in 0..3 {
            p.enter("foo");
            p.take_sample(0);
            p.leave("foo");
        }
        p.enter("bar");
        p.take_sample(0);
        p.leave("bar");
        let flat = p.flat_profile();
        assert_eq!(flat[0].0, "foo");
        assert_eq!(flat[0].1, 3);
        assert_eq!(flat[1].0, "bar");
        assert_eq!(flat[1].1, 1);
    }
    #[test]
    fn test_cumulative_profile() {
        let mut p = SamplingProfiler::new(1_000_000);
        p.enable();
        p.current_stack = vec!["inner".to_string(), "outer".to_string()];
        p.take_sample(0);
        p.take_sample(0);
        let cum = p.cumulative_profile();
        let has_inner = cum.iter().any(|(n, _)| n == "inner");
        let has_outer = cum.iter().any(|(n, _)| n == "outer");
        assert!(has_inner);
        assert!(has_outer);
    }
    #[test]
    fn test_flame_graph_basic() {
        let mut fg = FlameGraph::new();
        let stack = vec![
            "main".to_string(),
            "compute".to_string(),
            "inner".to_string(),
        ];
        fg.add_stack(&stack);
        fg.add_stack(&stack);
        assert_eq!(fg.total_samples, 2);
        let text = fg.render_text();
        assert!(text.contains("(all)"));
    }
    #[test]
    fn test_flame_node_get_or_create() {
        let mut node = FlameNode::new("root");
        {
            let child = node.get_or_create_child("child1");
            child.count += 1;
        }
        {
            let child = node.get_or_create_child("child1");
            child.count += 1;
        }
        {
            node.get_or_create_child("child2");
        }
        assert_eq!(node.children.len(), 2);
        assert_eq!(node.children[0].count, 2);
    }
    #[test]
    fn test_perf_counter() {
        let mut pc = PerfCounter::new();
        pc.simulate_instructions(1000);
        pc.simulate_cache_miss();
        pc.simulate_branch_misprediction();
        assert_eq!(pc.instructions_retired, 1000);
        assert_eq!(pc.cache_misses, 1);
        assert_eq!(pc.branch_mispredictions, 1);
        assert!(pc.ipc() > 0.0);
        let summary = pc.summary();
        assert!(summary.contains("PerfCounters"));
        assert!(summary.contains("IPC"));
    }
    #[test]
    fn test_allocation_tracker() {
        let mut tracker = AllocationTracker::new();
        tracker.record_alloc("heap", 1024);
        tracker.record_alloc("heap", 512);
        tracker.record_alloc("stack", 256);
        tracker.record_dealloc("heap", 512);
        let heap_stats = tracker
            .stats_for("heap")
            .expect("test operation should succeed");
        assert_eq!(heap_stats.alloc_count, 2);
        assert_eq!(heap_stats.total_bytes, 1536);
        assert_eq!(heap_stats.live_bytes, 1024);
        assert_eq!(tracker.total_live_bytes(), 1024 + 256);
        let top = tracker.top_allocators(1);
        assert_eq!(top[0].0, "heap");
    }
    #[test]
    fn test_tactic_profile_log() {
        let mut log = TacticProfileLog::new();
        log.record(TacticProfilingEvent::new("intro", 500, true, 1, 1));
        log.record(TacticProfilingEvent::new("apply", 1500, true, 1, 0));
        log.record(TacticProfilingEvent::new("rw", 300, false, 1, 1));
        assert_eq!(log.total_duration_ns(), 2300);
        assert_eq!(log.success_count(), 2);
        let top = log.top_slow(1);
        assert_eq!(top[0].tactic, "apply");
        assert!((log.avg_duration_ns() - 2300.0 / 3.0).abs() < 1.0);
    }
    #[test]
    fn test_tactic_profiling_event_goals_eliminated() {
        let event = TacticProfilingEvent::new("exact", 100, true, 3, 1);
        assert_eq!(event.goals_eliminated(), 2);
    }
    #[test]
    fn test_stack_snapshot() {
        let frames = vec!["main".to_string(), "foo".to_string(), "bar".to_string()];
        let snap = StackSnapshot::new(12345, frames.clone()).with_label("checkpoint");
        assert_eq!(snap.depth(), 3);
        assert_eq!(snap.label.as_deref(), Some("checkpoint"));
        let formatted = snap.format();
        assert!(formatted.contains("checkpoint"));
        assert!(formatted.contains("main"));
        assert!(formatted.contains("bar"));
    }
    #[test]
    fn test_profiler_config_builder() {
        let cfg = ProfilerConfig::new().enable_all();
        assert!(cfg.event_profiling);
        assert!(cfg.sampling_profiling);
        let cfg2 = ProfilerConfig::default().disable_all();
        assert!(!cfg2.event_profiling);
        assert!(!cfg2.sampling_profiling);
    }
    #[test]
    fn test_call_tree_node() {
        let mut root = CallTreeNode::new("main");
        root.inclusive_ns = 10_000;
        root.exclusive_ns = 1_000;
        root.call_count = 2;
        let mut child = CallTreeNode::new("compute");
        child.inclusive_ns = 9_000;
        child.exclusive_ns = 9_000;
        child.call_count = 5;
        root.children.push(child);
        assert!((root.avg_exclusive_ns() - 500.0).abs() < 1.0);
        assert!((root.avg_inclusive_ns() - 5000.0).abs() < 1.0);
        assert!(root.find_child("compute").is_some());
        assert!(root.find_child("missing").is_none());
    }
    #[test]
    fn test_sampling_profiler_avg_stack_depth() {
        let mut p = SamplingProfiler::new(1_000_000);
        p.enabled = true;
        p.samples
            .push(ProfileSample::new(0, vec!["a".into(), "b".into()], 0));
        p.samples.push(ProfileSample::new(
            1,
            vec!["a".into(), "b".into(), "c".into()],
            0,
        ));
        assert!((p.avg_stack_depth() - 2.5).abs() < 1e-9);
    }
}
#[cfg(test)]
mod profiler_extended_tests_2 {
    use super::*;
    #[test]
    fn test_event_filter_timestamp() {
        let filter = EventFilter {
            min_timestamp_ns: 100,
            max_timestamp_ns: 200,
            ..EventFilter::new()
        };
        let e1 = ProfilingEvent::FunctionCall {
            name: "f".to_string(),
            depth: 0,
        };
        let e2 = ProfilingEvent::FunctionCall {
            name: "g".to_string(),
            depth: 0,
        };
        assert!(filter.matches(150, &e1));
        assert!(!filter.matches(50, &e2));
        assert!(!filter.matches(250, &e2));
    }
    #[test]
    fn test_event_filter_function_name() {
        let filter = EventFilter {
            function_names: vec!["foo".to_string()],
            ..EventFilter::new()
        };
        let e_foo = ProfilingEvent::FunctionCall {
            name: "foo".to_string(),
            depth: 0,
        };
        let e_bar = ProfilingEvent::FunctionCall {
            name: "bar".to_string(),
            depth: 0,
        };
        assert!(filter.matches(0, &e_foo));
        assert!(!filter.matches(0, &e_bar));
    }
    #[test]
    fn test_event_filter_alloc_size() {
        let filter = EventFilter {
            min_alloc_bytes: 100,
            ..EventFilter::new()
        };
        let small = ProfilingEvent::Allocation {
            size: 50,
            tag: "x".to_string(),
        };
        let large = ProfilingEvent::Allocation {
            size: 200,
            tag: "y".to_string(),
        };
        assert!(!filter.matches(0, &small));
        assert!(filter.matches(0, &large));
    }
    #[test]
    fn test_timeline_view_build() {
        let mut p = Profiler::new();
        p.enable();
        p.enter_function("foo");
        p.exit_function("foo");
        p.alloc(1024, "heap");
        p.gc_cycle(10, 90);
        let view = TimelineView::build(&p);
        assert!(!view.entries.is_empty());
        let func_entries = view.by_category("function");
        assert!(!func_entries.is_empty());
    }
    #[test]
    fn test_heat_map() {
        let mut hm = HeatMap::new(10, 1_000_000_000);
        let base = 0u64;
        hm.record(0, base);
        hm.record(100_000_000, base);
        hm.record(900_000_000, base);
        assert_eq!(hm.counts[0], 1);
        let ascii = hm.render_ascii();
        assert!(ascii.contains('|'));
    }
    #[test]
    fn test_profiling_session() {
        let mut session = ProfilingSession::new("test_session");
        assert!(!session.running);
        session.start();
        assert!(session.running);
        session.enter_function("main");
        session.alloc(512, "buf");
        session.dealloc(512, "buf");
        session.exit_function("main");
        session.stop();
        assert!(!session.running);
        let report = session.combined_report();
        assert!(report.contains("test_session"));
    }
    #[test]
    fn test_real_time_monitor() {
        let mut mon = RealTimeMonitor::new("metrics", 100);
        mon.record("cpu", 0.5);
        mon.record("cpu", 0.7);
        mon.record("mem", 1024.0);
        assert!((mon.latest("cpu").expect("test operation should succeed") - 0.7).abs() < 1e-9);
        assert!((mon.avg("cpu") - 0.6).abs() < 1e-9);
        assert_eq!(mon.count("cpu"), 2);
        assert_eq!(mon.count("mem"), 1);
        assert!(mon.latest("missing").is_none());
    }
    #[test]
    fn test_real_time_monitor_capacity() {
        let mut mon = RealTimeMonitor::new("capacity_test", 3);
        for i in 0..5 {
            mon.record("x", i as f64);
        }
        assert_eq!(mon.snapshots.len(), 3);
    }
    #[test]
    fn test_histogram() {
        let mut h = Histogram::new(5, 0.0, 100.0);
        h.record(10.0);
        h.record(10.0);
        h.record(50.0);
        h.record(90.0);
        assert_eq!(h.total, 4);
        assert!((h.mean() - 40.0).abs() < 1e-9);
        let mode = h.mode_bucket().expect("test operation should succeed");
        assert!(mode.count >= 1);
        let ascii = h.render_ascii();
        assert!(ascii.contains('['));
    }
    #[test]
    fn test_histogram_edge_case() {
        let mut h = Histogram::new(3, 0.0, 10.0);
        h.record(0.0);
        h.record(5.0);
        h.record(9.99);
        h.record(100.0);
        assert_eq!(h.total, 4);
    }
}
/// A step in the profiling pipeline.
#[allow(dead_code)]
pub trait ProfilingStep {
    /// Process a batch of events.
    fn process(&mut self, events: &[(u64, ProfilingEvent)]);
    /// Name of this step.
    fn name(&self) -> &str;
}
#[cfg(test)]
mod profiler_extended_tests_3 {
    use super::*;
    #[test]
    fn test_gc_collection_record() {
        let r = GcCollectionRecord::new(1000, 50, 150, 500_000);
        assert!((r.efficiency() - 50.0 / 200.0).abs() < 1e-9);
    }
    #[test]
    fn test_gc_profiler() {
        let mut gcp = GcProfiler::new();
        gcp.record(30, 100, 1_000_000);
        gcp.record(20, 80, 2_000_000);
        assert_eq!(gcp.collection_count(), 2);
        assert_eq!(gcp.total_collected(), 50);
        assert!((gcp.avg_pause_ns() - 1_500_000.0).abs() < 1.0);
        assert_eq!(gcp.max_pause_ns(), 2_000_000);
        let summary = gcp.summary();
        assert!(summary.contains("2 collections"));
    }
    #[test]
    fn test_counting_step() {
        let mut step = CountingStep::new("counter");
        assert_eq!(step.name(), "counter");
        let events = vec![
            (
                0u64,
                ProfilingEvent::FunctionCall {
                    name: "f".to_string(),
                    depth: 0,
                },
            ),
            (
                1u64,
                ProfilingEvent::Allocation {
                    size: 100,
                    tag: "t".to_string(),
                },
            ),
            (
                2u64,
                ProfilingEvent::FunctionCall {
                    name: "g".to_string(),
                    depth: 0,
                },
            ),
        ];
        step.process(&events);
        assert_eq!(step.counts.get("FunctionCall").copied().unwrap_or(0), 2);
        assert_eq!(step.counts.get("Allocation").copied().unwrap_or(0), 1);
    }
    #[test]
    fn test_annotated_timeline() {
        let mut tl = AnnotatedTimeline::new();
        tl.annotate(TimelineAnnotation::new(100, "start", "checkpoint"));
        tl.annotate(TimelineAnnotation::new(500, "mid", "checkpoint"));
        tl.annotate(TimelineAnnotation::new(1000, "end", "checkpoint"));
        let in_range = tl.annotations_in_range(200, 700);
        assert_eq!(in_range.len(), 1);
        assert_eq!(in_range[0].text, "mid");
    }
    #[test]
    fn test_comprehensive_report() {
        let mut session = ProfilingSession::new("test");
        session.start();
        session.enter_function("main");
        session.alloc(1024, "heap");
        session.sampler.take_sample(0);
        session.exit_function("main");
        session.stop();
        let report = ComprehensiveProfilingReport::build(&session);
        let text = report.to_text();
        assert!(text.contains("Profile Report"));
    }
    #[test]
    fn test_gc_profiler_empty() {
        let gcp = GcProfiler::new();
        assert_eq!(gcp.collection_count(), 0);
        assert_eq!(gcp.total_collected(), 0);
        assert!((gcp.avg_pause_ns() - 0.0).abs() < 1e-9);
        assert_eq!(gcp.max_pause_ns(), 0);
    }
    #[test]
    fn test_timeline_annotation_categories() {
        let mut tl = AnnotatedTimeline::new();
        tl.annotate(TimelineAnnotation::new(0, "start error", "error"));
        tl.annotate(TimelineAnnotation::new(0, "start ok", "checkpoint"));
        let errors: Vec<_> = tl
            .annotations
            .iter()
            .filter(|a| a.category == "error")
            .collect();
        assert_eq!(errors.len(), 1);
    }
    #[test]
    fn test_flame_graph_from_profiler() {
        let mut p = SamplingProfiler::new(1_000_000);
        p.enabled = true;
        p.current_stack = vec!["main".into(), "foo".into()];
        p.take_sample(0);
        p.take_sample(0);
        let fg = FlameGraph::from_profiler(&p);
        assert_eq!(fg.total_samples, 2);
        let text = fg.render_text();
        assert!(text.contains("(all)"));
    }
}
#[cfg(test)]
mod middleware_tests {
    use super::*;
    #[test]
    fn test_middleware_instrument() {
        let mut mw = ProfilingMiddleware::new();
        let result = mw.instrument("compute", || 6 * 7);
        assert_eq!(result, 42);
        let report = mw.report();
        assert_eq!(report.total_calls, 1);
    }
    #[test]
    fn test_middleware_inactive() {
        let mut mw = ProfilingMiddleware::new();
        mw.active = false;
        mw.instrument("ignored", || ());
        let report = mw.report();
        assert_eq!(report.total_calls, 0);
    }
}
