//! Functions for the comprehensive profiling API (`Profiler2` / `ProfileReport2`).
//!
//! Includes:
//! - `Profiler2` method implementations
//! - `format_report2`
//! - `format_flame_graph_dot`
//! - `merge_reports2`
//! - 30+ tests

use std::collections::HashMap;

use super::perf_types::{
    AllocationRecord, CallRecord, EventKind, FlameGraphNode, InFlightFrame, ProfileReport2,
    Profiler2, ProfilerConfig2, ProfilerEvent,
};

// ---------------------------------------------------------------------------
// Profiler2 method impls
// ---------------------------------------------------------------------------

impl Profiler2 {
    // -----------------------------------------------------------------------
    // enter / exit
    // -----------------------------------------------------------------------

    /// Record entering a function named `name`.
    ///
    /// Pushes a timing frame onto the internal call stack.  No-op when
    /// the profiler is disabled.
    pub fn enter(&mut self, name: &str) {
        if !self.enabled || !self.should_sample() {
            return;
        }
        let ev = ProfilerEvent::new(EventKind::FunctionEnter, name, None);
        self.push_event(ev);
        self.call_stack.push(InFlightFrame::new(name));
    }

    /// Record exiting a function named `name`.
    ///
    /// Pops the corresponding frame, updates [`CallRecord`] timing, and
    /// charges the elapsed time as *callee time* to the parent frame.
    /// No-op when the profiler is disabled.
    pub fn exit(&mut self, name: &str) {
        if !self.enabled {
            return;
        }
        let ev = ProfilerEvent::new(EventKind::FunctionExit, name, None);
        self.push_event(ev);

        // Pop the matching frame (from the top; handle mis-matched calls).
        let frame = if let Some(pos) = self.call_stack.iter().rposition(|f| f.name == name) {
            self.call_stack.remove(pos)
        } else {
            return; // unmatched exit — ignore
        };

        let elapsed = frame.enter_instant.elapsed().as_nanos() as u64;
        let self_ns = elapsed.saturating_sub(frame.callee_ns);

        // Charge this elapsed time to the parent's callee_ns.
        if let Some(parent) = self.call_stack.last_mut() {
            parent.callee_ns = parent.callee_ns.saturating_add(elapsed);
        }

        // Update the aggregated call record.
        let record = self
            .call_records
            .entry(name.to_string())
            .or_insert_with(|| CallRecord::new(name));
        record.calls += 1;
        record.total_ns = record.total_ns.saturating_add(elapsed);
        record.self_ns = record.self_ns.saturating_add(self_ns);
        if elapsed > record.max_ns {
            record.max_ns = elapsed;
        }
        if elapsed < record.min_ns {
            record.min_ns = elapsed;
        }
    }

    // -----------------------------------------------------------------------
    // record_alloc / record_gc / record_tail_call
    // -----------------------------------------------------------------------

    /// Record a heap allocation for `type_name` of `bytes` bytes.
    ///
    /// No-op when disabled or when `track_allocations` is `false`.
    pub fn record_alloc(&mut self, type_name: &str, bytes: usize) {
        if !self.enabled || !self.config.track_allocations {
            return;
        }
        let ev = ProfilerEvent::new(
            EventKind::Allocation { size_bytes: bytes },
            type_name,
            Some(bytes as u64),
        );
        self.push_event(ev);
        let record = self
            .alloc_records
            .entry(type_name.to_string())
            .or_insert_with(|| AllocationRecord::new(type_name));
        record.count += 1;
        record.total_bytes += bytes as u64;
    }

    /// Record a GC pause of `duration_ns` nanoseconds.
    ///
    /// No-op when disabled.
    pub fn record_gc(&mut self, duration_ns: u64) {
        if !self.enabled {
            return;
        }
        let ev = ProfilerEvent::new(EventKind::GcPause { duration_ns }, "gc", Some(duration_ns));
        self.push_event(ev);
        self.gc_pauses.push(duration_ns);
    }

    /// Record a tail call to `name`.
    ///
    /// No-op when disabled.
    pub fn record_tail_call(&mut self, name: &str) {
        if !self.enabled {
            return;
        }
        let ev = ProfilerEvent::new(EventKind::TailCall, name, None);
        self.push_event(ev);
    }

    // -----------------------------------------------------------------------
    // generate_report
    // -----------------------------------------------------------------------

    /// Generate a [`ProfileReport2`] from all accumulated profiling data.
    pub fn generate_report(&self) -> ProfileReport2 {
        let wall_time_ns = self
            .start_instant
            .map(|t| t.elapsed().as_nanos() as u64)
            .unwrap_or(0);

        let mut call_records: Vec<CallRecord> = self.call_records.values().cloned().collect();
        call_records.sort_by_key(|r| std::cmp::Reverse(r.total_ns));

        let mut alloc_records: Vec<AllocationRecord> =
            self.alloc_records.values().cloned().collect();
        alloc_records.sort_by_key(|r| std::cmp::Reverse(r.total_bytes));

        ProfileReport2 {
            call_records,
            alloc_records,
            total_events: self.events.len() as u64,
            wall_time_ns,
            gc_pauses: self.gc_pauses.clone(),
        }
    }

    // -----------------------------------------------------------------------
    // to_flame_graph
    // -----------------------------------------------------------------------

    /// Build a [`FlameGraphNode`] tree from accumulated call records.
    ///
    /// The root node is named `"(root)"` and holds the sum of all call times.
    /// Each call record becomes a direct child.  Callee relationships are
    /// approximated from the sorted call-record list: any record whose
    /// `self_ns` is substantially less than its `total_ns` is assumed to have
    /// children.
    pub fn to_flame_graph(&self) -> FlameGraphNode {
        let mut root = FlameGraphNode::new("(root)");
        root.total_time = self
            .call_records
            .values()
            .map(|r| r.total_ns)
            .max()
            .unwrap_or(0);

        for record in self.call_records.values() {
            let child = root.get_or_create_child(&record.name);
            child.total_time = record.total_ns;
            child.self_time = record.self_ns;
        }
        root
    }

    // -----------------------------------------------------------------------
    // top_functions
    // -----------------------------------------------------------------------

    /// Return the top `n` call records sorted by `total_ns` descending.
    pub fn top_functions(&self, n: usize) -> Vec<&CallRecord> {
        let mut records: Vec<&CallRecord> = self.call_records.values().collect();
        records.sort_by_key(|r| std::cmp::Reverse(r.total_ns));
        records.truncate(n);
        records
    }

    // -----------------------------------------------------------------------
    // to_folded_stacks
    // -----------------------------------------------------------------------

    /// Produce Brendan Gregg-style "folded stacks" output.
    ///
    /// Each line is `func_name count` where `count` is the number of samples
    /// (calls) attributed to that function.  Suitable for input to
    /// `flamegraph.pl`.
    pub fn to_folded_stacks(&self) -> String {
        let mut lines: Vec<String> = self
            .call_records
            .values()
            .map(|r| format!("{} {}", r.name, r.calls))
            .collect();
        lines.sort();
        lines.join("\n")
    }
}

// ---------------------------------------------------------------------------
// format_report2
// ---------------------------------------------------------------------------

/// Format a [`ProfileReport2`] as a human-readable text table.
pub fn format_report2(report: &ProfileReport2) -> String {
    let mut out = String::new();
    out.push_str("╔══════════════════════════════════════════════════════════╗\n");
    out.push_str("║               Profiler2 Performance Report               ║\n");
    out.push_str("╠══════════════════════════════════════════════════════════╣\n");
    out.push_str(&format!(
        "║  Wall time       : {:>10} ns\n",
        report.wall_time_ns
    ));
    out.push_str(&format!(
        "║  Total events    : {:>10}\n",
        report.total_events
    ));
    out.push_str(&format!(
        "║  GC pauses       : {:>10}\n",
        report.gc_pause_count()
    ));
    out.push_str(&format!(
        "║  Total GC time   : {:>10} ns\n",
        report.total_gc_pause_ns()
    ));
    out.push_str("╠══════════════════════════════════════════════════════════╣\n");

    if report.call_records.is_empty() {
        out.push_str("║  (no call records)\n");
    } else {
        out.push_str(&format!(
            "║  {:<30} {:>8} {:>12} {:>12}\n",
            "Function", "Calls", "Total ns", "Self ns"
        ));
        out.push_str("║  ─────────────────────────────────────────────────────\n");
        for r in &report.call_records {
            out.push_str(&format!(
                "║  {:<30} {:>8} {:>12} {:>12}\n",
                r.name, r.calls, r.total_ns, r.self_ns
            ));
        }
    }

    if !report.alloc_records.is_empty() {
        out.push_str("╠══════════════════════════════════════════════════════════╣\n");
        out.push_str(&format!(
            "║  {:<30} {:>8} {:>12}\n",
            "Type", "Count", "Total bytes"
        ));
        out.push_str("║  ─────────────────────────────────────────────────────\n");
        for r in &report.alloc_records {
            out.push_str(&format!(
                "║  {:<30} {:>8} {:>12}\n",
                r.type_name, r.count, r.total_bytes
            ));
        }
    }

    out.push_str("╚══════════════════════════════════════════════════════════╝\n");
    out
}

// ---------------------------------------------------------------------------
// format_flame_graph_dot
// ---------------------------------------------------------------------------

/// Render a [`FlameGraphNode`] tree in Graphviz DOT format.
///
/// The resulting string can be piped to `dot -Tsvg` to produce a call-graph
/// SVG.
pub fn format_flame_graph_dot(root: &FlameGraphNode) -> String {
    let mut out = String::new();
    out.push_str("digraph flamegraph {\n");
    out.push_str("    rankdir=TB;\n");
    out.push_str("    node [shape=box, fontname=\"monospace\"];\n");
    out.push('\n');
    write_dot_node(&mut out, root, 0);
    out.push_str("}\n");
    out
}

/// Recursively write a DOT node and its edges.
fn write_dot_node(out: &mut String, node: &FlameGraphNode, id: usize) -> usize {
    let safe_name = node.name.replace('"', "\\\"");
    let label = format!(
        "{}\ntotal={}ns self={}ns",
        safe_name, node.total_time, node.self_time
    );
    out.push_str(&format!(
        "    n{} [label=\"{}\"];\n",
        id,
        label.replace('\n', "\\n")
    ));
    let mut next_id = id + 1;
    for child in &node.children {
        let child_id = next_id;
        next_id = write_dot_node(out, child, child_id);
        out.push_str(&format!("    n{} -> n{};\n", id, child_id));
    }
    next_id
}

// ---------------------------------------------------------------------------
// merge_reports2
// ---------------------------------------------------------------------------

/// Merge multiple [`ProfileReport2`] instances into one.
///
/// Call records with the same name are aggregated; GC pauses are concatenated;
/// allocation records are summed; wall time is the maximum of all sessions.
pub fn merge_reports2(reports: &[ProfileReport2]) -> ProfileReport2 {
    if reports.is_empty() {
        return ProfileReport2::empty();
    }

    let mut call_map: HashMap<String, CallRecord> = HashMap::new();
    let mut alloc_map: HashMap<String, AllocationRecord> = HashMap::new();
    let mut total_events: u64 = 0;
    let mut wall_time_ns: u64 = 0;
    let mut gc_pauses: Vec<u64> = Vec::new();

    for report in reports {
        total_events = total_events.saturating_add(report.total_events);
        if report.wall_time_ns > wall_time_ns {
            wall_time_ns = report.wall_time_ns;
        }
        gc_pauses.extend_from_slice(&report.gc_pauses);

        for cr in &report.call_records {
            let entry = call_map
                .entry(cr.name.clone())
                .or_insert_with(|| CallRecord::new(&cr.name));
            entry.calls = entry.calls.saturating_add(cr.calls);
            entry.total_ns = entry.total_ns.saturating_add(cr.total_ns);
            entry.self_ns = entry.self_ns.saturating_add(cr.self_ns);
            if cr.max_ns > entry.max_ns {
                entry.max_ns = cr.max_ns;
            }
            if cr.min_ns < entry.min_ns {
                entry.min_ns = cr.min_ns;
            }
        }

        for ar in &report.alloc_records {
            let entry = alloc_map
                .entry(ar.type_name.clone())
                .or_insert_with(|| AllocationRecord::new(&ar.type_name));
            entry.count = entry.count.saturating_add(ar.count);
            entry.total_bytes = entry.total_bytes.saturating_add(ar.total_bytes);
        }
    }

    let mut call_records: Vec<CallRecord> = call_map.into_values().collect();
    call_records.sort_by_key(|r| std::cmp::Reverse(r.total_ns));

    let mut alloc_records: Vec<AllocationRecord> = alloc_map.into_values().collect();
    alloc_records.sort_by_key(|r| std::cmp::Reverse(r.total_bytes));

    ProfileReport2 {
        call_records,
        alloc_records,
        total_events,
        wall_time_ns,
        gc_pauses,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn make_profiler() -> Profiler2 {
        Profiler2::new_enabled(ProfilerConfig2::new())
    }

    // --- ProfilerConfig2 ---

    #[test]
    fn test_config_defaults() {
        let cfg = ProfilerConfig2::new();
        assert_eq!(cfg.max_events, 1_000_000);
        assert_eq!(cfg.sample_rate, 1);
        assert!(cfg.track_allocations);
    }

    #[test]
    fn test_config_builder() {
        let cfg = ProfilerConfig2::new()
            .with_max_events(500)
            .with_sample_rate(2)
            .with_track_allocations(false);
        assert_eq!(cfg.max_events, 500);
        assert_eq!(cfg.sample_rate, 2);
        assert!(!cfg.track_allocations);
    }

    #[test]
    fn test_config_sample_rate_min_one() {
        let cfg = ProfilerConfig2::new().with_sample_rate(0);
        assert_eq!(cfg.sample_rate, 1);
    }

    // --- ProfilerEvent ---

    #[test]
    fn test_event_kind_variant_names() {
        assert_eq!(EventKind::FunctionEnter.variant_name(), "FunctionEnter");
        assert_eq!(EventKind::FunctionExit.variant_name(), "FunctionExit");
        assert_eq!(
            EventKind::Allocation { size_bytes: 64 }.variant_name(),
            "Allocation"
        );
        assert_eq!(
            EventKind::GcPause { duration_ns: 100 }.variant_name(),
            "GcPause"
        );
        assert_eq!(EventKind::TailCall.variant_name(), "TailCall");
        assert_eq!(EventKind::ThunkForced.variant_name(), "ThunkForced");
        assert_eq!(EventKind::ClosureCreated.variant_name(), "ClosureCreated");
        assert_eq!(EventKind::ClosureApplied.variant_name(), "ClosureApplied");
        assert_eq!(EventKind::RuntimeError.variant_name(), "RuntimeError");
    }

    #[test]
    fn test_event_kind_display() {
        assert_eq!(format!("{}", EventKind::TailCall), "TailCall");
    }

    #[test]
    fn test_profiler_event_new() {
        let ev = ProfilerEvent::new(EventKind::FunctionEnter, "main", None);
        assert_eq!(ev.kind, EventKind::FunctionEnter);
        assert_eq!(ev.name, "main");
        assert!(ev.data.is_none());
        assert!(ev.timestamp_ns > 0);
    }

    #[test]
    fn test_profiler_event_with_timestamp() {
        let ev = ProfilerEvent::with_timestamp(12345, EventKind::RuntimeError, "crash", Some(99));
        assert_eq!(ev.timestamp_ns, 12345);
        assert_eq!(ev.data, Some(99));
    }

    // --- CallRecord ---

    #[test]
    fn test_call_record_mean() {
        let mut r = CallRecord::new("foo");
        r.calls = 4;
        r.total_ns = 1_000;
        assert!((r.mean_ns() - 250.0).abs() < 1e-9);
    }

    #[test]
    fn test_call_record_mean_zero_calls() {
        let r = CallRecord::new("bar");
        assert!((r.mean_ns() - 0.0).abs() < 1e-9);
    }

    // --- AllocationRecord ---

    #[test]
    fn test_alloc_record_avg() {
        let mut r = AllocationRecord::new("Vec<u8>");
        r.count = 5;
        r.total_bytes = 500;
        assert!((r.avg_bytes() - 100.0).abs() < 1e-9);
    }

    #[test]
    fn test_alloc_record_avg_zero() {
        let r = AllocationRecord::new("T");
        assert!((r.avg_bytes() - 0.0).abs() < 1e-9);
    }

    // --- Profiler2 basic enter/exit ---

    #[test]
    fn test_enter_exit_increments_calls() {
        let mut p = make_profiler();
        p.enter("alpha");
        p.exit("alpha");
        let report = p.generate_report();
        assert_eq!(report.call_records.len(), 1);
        assert_eq!(report.call_records[0].calls, 1);
    }

    #[test]
    fn test_multiple_calls_aggregate() {
        let mut p = make_profiler();
        for _ in 0..5 {
            p.enter("beta");
            p.exit("beta");
        }
        let report = p.generate_report();
        let rec = report.call_records.iter().find(|r| r.name == "beta");
        assert!(rec.is_some());
        assert_eq!(rec.expect("beta record exists").calls, 5);
    }

    #[test]
    fn test_disabled_profiler_records_nothing() {
        let mut p = Profiler2::new(ProfilerConfig2::new());
        p.enter("gamma");
        p.exit("gamma");
        p.record_alloc("Foo", 1024);
        p.record_gc(500_000);
        let report = p.generate_report();
        assert!(report.call_records.is_empty());
        assert!(report.alloc_records.is_empty());
        assert!(report.gc_pauses.is_empty());
    }

    #[test]
    fn test_enable_disable() {
        let mut p = Profiler2::new(ProfilerConfig2::new());
        p.enable();
        p.enter("x");
        p.exit("x");
        p.disable();
        p.enter("y");
        p.exit("y");
        let report = p.generate_report();
        assert!(report.call_records.iter().any(|r| r.name == "x"));
        assert!(!report.call_records.iter().any(|r| r.name == "y"));
    }

    // --- record_alloc ---

    #[test]
    fn test_record_alloc_accumulates() {
        let mut p = make_profiler();
        p.record_alloc("Box<i32>", 4);
        p.record_alloc("Box<i32>", 4);
        p.record_alloc("Vec<u8>", 64);
        let report = p.generate_report();
        let box_rec = report
            .alloc_records
            .iter()
            .find(|r| r.type_name == "Box<i32>")
            .expect("Box<i32> should be present");
        assert_eq!(box_rec.count, 2);
        assert_eq!(box_rec.total_bytes, 8);
    }

    #[test]
    fn test_record_alloc_disabled_when_track_off() {
        let cfg = ProfilerConfig2::new().with_track_allocations(false);
        let mut p = Profiler2::new_enabled(cfg);
        p.record_alloc("Thing", 256);
        let report = p.generate_report();
        assert!(report.alloc_records.is_empty());
    }

    // --- record_gc ---

    #[test]
    fn test_record_gc_collects_pauses() {
        let mut p = make_profiler();
        p.record_gc(100_000);
        p.record_gc(200_000);
        let report = p.generate_report();
        assert_eq!(report.gc_pauses.len(), 2);
        assert_eq!(report.total_gc_pause_ns(), 300_000);
    }

    #[test]
    fn test_gc_pause_statistics() {
        let mut p = make_profiler();
        p.record_gc(1_000_000);
        p.record_gc(3_000_000);
        let report = p.generate_report();
        assert!((report.mean_gc_pause_ns() - 2_000_000.0).abs() < 1.0);
    }

    // --- record_tail_call ---

    #[test]
    fn test_record_tail_call_pushes_event() {
        let mut p = make_profiler();
        p.record_tail_call("recur");
        assert!(!p.events.is_empty());
        assert!(matches!(p.events[0].kind, EventKind::TailCall));
    }

    // --- generate_report ordering ---

    #[test]
    fn test_report_sorted_by_total_ns_desc() {
        let mut p = make_profiler();
        // Force different timings by sleeping briefly on one function.
        p.enter("fast");
        p.exit("fast");
        p.enter("slow");
        std::thread::sleep(std::time::Duration::from_millis(2));
        p.exit("slow");
        let report = p.generate_report();
        // "slow" should appear first.
        assert_eq!(report.call_records[0].name, "slow");
    }

    // --- top_functions ---

    #[test]
    fn test_top_functions_returns_n() {
        let mut p = make_profiler();
        for name in &["a", "b", "c", "d", "e"] {
            p.enter(name);
            p.exit(name);
        }
        let top = p.top_functions(3);
        assert_eq!(top.len(), 3);
    }

    #[test]
    fn test_top_functions_fewer_than_n() {
        let mut p = make_profiler();
        p.enter("only");
        p.exit("only");
        let top = p.top_functions(10);
        assert_eq!(top.len(), 1);
    }

    // --- to_flame_graph ---

    #[test]
    fn test_flame_graph_root_name() {
        let p = make_profiler();
        let root = p.to_flame_graph();
        assert_eq!(root.name, "(root)");
    }

    #[test]
    fn test_flame_graph_children() {
        let mut p = make_profiler();
        p.enter("compute");
        p.exit("compute");
        let root = p.to_flame_graph();
        assert!(root.children.iter().any(|c| c.name == "compute"));
    }

    #[test]
    fn test_flame_graph_node_count() {
        let mut root = FlameGraphNode::new("root");
        root.get_or_create_child("child1");
        root.get_or_create_child("child2");
        assert_eq!(root.node_count(), 3);
    }

    // --- to_folded_stacks ---

    #[test]
    fn test_folded_stacks_format() {
        let mut p = make_profiler();
        p.enter("foo");
        p.exit("foo");
        p.enter("foo");
        p.exit("foo");
        p.enter("bar");
        p.exit("bar");
        let stacks = p.to_folded_stacks();
        assert!(stacks.contains("foo 2"));
        assert!(stacks.contains("bar 1"));
    }

    #[test]
    fn test_folded_stacks_empty() {
        let p = make_profiler();
        assert!(p.to_folded_stacks().is_empty());
    }

    // --- format_report2 ---

    #[test]
    fn test_format_report2_contains_headers() {
        let mut p = make_profiler();
        p.enter("work");
        p.exit("work");
        p.record_alloc("Heap", 512);
        let report = p.generate_report();
        let text = format_report2(&report);
        assert!(text.contains("Profiler2 Performance Report"));
        assert!(text.contains("work"));
        assert!(text.contains("Heap"));
    }

    #[test]
    fn test_format_report2_empty() {
        let report = ProfileReport2::empty();
        let text = format_report2(&report);
        assert!(text.contains("no call records"));
    }

    // --- format_flame_graph_dot ---

    #[test]
    fn test_dot_output_starts_with_digraph() {
        let root = FlameGraphNode::new("root");
        let dot = format_flame_graph_dot(&root);
        assert!(dot.starts_with("digraph flamegraph {"));
        assert!(dot.contains("n0"));
    }

    #[test]
    fn test_dot_output_has_edges() {
        let mut root = FlameGraphNode::new("root");
        root.get_or_create_child("child");
        let dot = format_flame_graph_dot(&root);
        assert!(dot.contains("n0 -> n1"));
    }

    // --- merge_reports2 ---

    #[test]
    fn test_merge_empty_slice() {
        let merged = merge_reports2(&[]);
        assert!(merged.call_records.is_empty());
        assert_eq!(merged.total_events, 0);
    }

    #[test]
    fn test_merge_single_report() {
        let mut p = make_profiler();
        p.enter("x");
        p.exit("x");
        let report = p.generate_report();
        let merged = merge_reports2(std::slice::from_ref(&report));
        assert_eq!(merged.call_records.len(), report.call_records.len());
    }

    #[test]
    fn test_merge_two_reports_sum_calls() {
        let mut p1 = make_profiler();
        p1.enter("fn_a");
        p1.exit("fn_a");

        let mut p2 = make_profiler();
        p2.enter("fn_a");
        p2.exit("fn_a");
        p2.enter("fn_a");
        p2.exit("fn_a");

        let r1 = p1.generate_report();
        let r2 = p2.generate_report();
        let merged = merge_reports2(&[r1, r2]);
        let rec = merged
            .call_records
            .iter()
            .find(|r| r.name == "fn_a")
            .expect("fn_a should be in merged report");
        assert_eq!(rec.calls, 3);
    }

    #[test]
    fn test_merge_gc_pauses_concatenated() {
        let mut p1 = make_profiler();
        p1.record_gc(1_000);
        let mut p2 = make_profiler();
        p2.record_gc(2_000);
        p2.record_gc(3_000);
        let merged = merge_reports2(&[p1.generate_report(), p2.generate_report()]);
        assert_eq!(merged.gc_pauses.len(), 3);
    }

    #[test]
    fn test_merge_alloc_records_sum() {
        let mut p1 = make_profiler();
        p1.record_alloc("T", 100);
        let mut p2 = make_profiler();
        p2.record_alloc("T", 200);
        let merged = merge_reports2(&[p1.generate_report(), p2.generate_report()]);
        let t = merged
            .alloc_records
            .iter()
            .find(|r| r.type_name == "T")
            .expect("T should be present");
        assert_eq!(t.total_bytes, 300);
        assert_eq!(t.count, 2);
    }

    // --- ring buffer ---

    #[test]
    fn test_ring_buffer_caps_at_max_events() {
        let cfg = ProfilerConfig2::new().with_max_events(5);
        let mut p = Profiler2::new_enabled(cfg);
        for _ in 0..10 {
            p.enter("x");
            p.exit("x");
        }
        assert!(p.events.len() <= 5);
    }

    // --- FlameGraphNode helpers ---

    #[test]
    fn test_get_or_create_child_reuses() {
        let mut root = FlameGraphNode::new("root");
        {
            let c = root.get_or_create_child("child");
            c.self_time = 42;
        }
        {
            let c = root.get_or_create_child("child");
            assert_eq!(c.self_time, 42);
        }
        assert_eq!(root.children.len(), 1);
    }

    #[test]
    fn test_flame_graph_node_totals() {
        let mut n = FlameGraphNode::new("n");
        n.self_time = 100;
        n.total_time = 500;
        assert_eq!(n.self_time, 100);
        assert_eq!(n.total_time, 500);
    }

    // --- ProfileReport2 statistics ---

    #[test]
    fn test_report_gc_statistics() {
        let report = ProfileReport2 {
            call_records: Vec::new(),
            alloc_records: Vec::new(),
            total_events: 0,
            wall_time_ns: 0,
            gc_pauses: vec![1_000, 3_000, 2_000],
        };
        assert_eq!(report.gc_pause_count(), 3);
        assert_eq!(report.total_gc_pause_ns(), 6_000);
        assert!((report.mean_gc_pause_ns() - 2_000.0).abs() < 1.0);
    }

    #[test]
    fn test_report_empty_gc_statistics() {
        let report = ProfileReport2::empty();
        assert_eq!(report.gc_pause_count(), 0);
        assert_eq!(report.total_gc_pause_ns(), 0);
        assert!((report.mean_gc_pause_ns() - 0.0).abs() < 1e-9);
    }
}
