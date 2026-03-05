//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::*;
use std::collections::HashMap;

/// A span (start/end offset in source) associated with an elaboration event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElabSpan {
    /// Byte offset of the start of the span in the source.
    pub start: usize,
    /// Byte offset just past the end of the span in the source.
    pub end: usize,
    /// Optional source file name.
    pub file: Option<String>,
}
impl ElabSpan {
    /// Create a new span.
    pub fn new(start: usize, end: usize) -> Self {
        Self {
            start,
            end,
            file: None,
        }
    }
    /// Create a span with a file label.
    pub fn with_file(start: usize, end: usize, file: impl Into<String>) -> Self {
        Self {
            start,
            end,
            file: Some(file.into()),
        }
    }
    /// Length of this span in bytes.
    pub fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }
    /// Whether the span has zero length.
    pub fn is_empty(&self) -> bool {
        self.end <= self.start
    }
    /// Whether this span contains the given byte offset.
    pub fn contains(&self, offset: usize) -> bool {
        offset >= self.start && offset < self.end
    }
    /// Whether this span overlaps with another.
    pub fn overlaps(&self, other: &ElabSpan) -> bool {
        self.start < other.end && other.start < self.end
    }
}
/// A memory snapshot capturing heap allocation state.
#[derive(Debug, Clone, Default)]
pub struct MemSnapshot {
    /// Approximate number of live heap objects (simulated).
    pub live_objects: usize,
    /// Approximate total bytes allocated (simulated).
    pub allocated_bytes: usize,
    /// Approximate bytes in the free list (simulated).
    pub free_bytes: usize,
    /// Label for this snapshot.
    pub label: String,
}
impl MemSnapshot {
    /// Create a labeled snapshot.
    pub fn new(label: impl Into<String>, live_objects: usize, allocated_bytes: usize) -> Self {
        Self {
            live_objects,
            allocated_bytes,
            free_bytes: 0,
            label: label.into(),
        }
    }
}
/// A node in the elaboration call graph.
#[derive(Debug, Clone)]
pub struct CallNode {
    /// Name of the function/elaborator being called.
    pub name: String,
    /// Total time spent in this node (inclusive) in nanoseconds.
    pub inclusive_ns: u64,
    /// Time spent in this node excluding children in nanoseconds.
    pub exclusive_ns: u64,
    /// Call count.
    pub call_count: usize,
    /// Indices of child nodes in the call graph.
    pub children: Vec<usize>,
}
impl CallNode {
    /// Create a new call node.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            inclusive_ns: 0,
            exclusive_ns: 0,
            call_count: 0,
            children: Vec::new(),
        }
    }
    /// Record a call with the given inclusive time.
    pub fn record_call(&mut self, inclusive_ns: u64, exclusive_ns: u64) {
        self.inclusive_ns += inclusive_ns;
        self.exclusive_ns += exclusive_ns;
        self.call_count += 1;
    }
    /// Inclusive time in milliseconds.
    pub fn inclusive_ms(&self) -> f64 {
        self.inclusive_ns as f64 / 1_000_000.0
    }
    /// Exclusive time in milliseconds.
    pub fn exclusive_ms(&self) -> f64 {
        self.exclusive_ns as f64 / 1_000_000.0
    }
}
/// Builder that converts a `CallGraph` into flamegraph frames.
#[derive(Debug, Default)]
pub struct FlamegraphBuilder {
    /// Collected frames.
    pub frames: Vec<FlameFrame>,
}
impl FlamegraphBuilder {
    /// Create a new builder.
    pub fn new() -> Self {
        Self::default()
    }
    /// Build frames from a call graph starting at the given root node.
    pub fn build_from_graph(&mut self, graph: &CallGraph, root: usize) {
        self.frames.clear();
        self.collect_frames(graph, root, 0, None);
    }
    fn collect_frames(
        &mut self,
        graph: &CallGraph,
        idx: usize,
        depth: usize,
        parent: Option<usize>,
    ) {
        if let Some(node) = graph.nodes.get(idx) {
            let frame_idx = self.frames.len();
            let mut frame = FlameFrame::new(
                node.name.clone(),
                node.exclusive_ns,
                node.inclusive_ns,
                depth,
            );
            frame.parent = parent;
            self.frames.push(frame);
            let children = node.children.clone();
            for child_idx in children {
                self.collect_frames(graph, child_idx, depth + 1, Some(frame_idx));
            }
        }
    }
    /// Generate a simple collapsed flamegraph text format (name;name;... count).
    pub fn to_collapsed_text(&self) -> String {
        let mut lines = Vec::new();
        for frame in &self.frames {
            let mut stack = vec![frame.name.clone()];
            let mut cur_parent = frame.parent;
            while let Some(pidx) = cur_parent {
                if let Some(pf) = self.frames.get(pidx) {
                    stack.push(pf.name.clone());
                    cur_parent = pf.parent;
                } else {
                    break;
                }
            }
            stack.reverse();
            lines.push(format!("{} {}", stack.join(";"), frame.self_ns));
        }
        lines.join("\n")
    }
    /// Total number of frames.
    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }
}
/// Call graph for elaboration, stored as a flat array of nodes.
#[derive(Debug, Default)]
pub struct CallGraph {
    /// All nodes in the graph.
    pub nodes: Vec<CallNode>,
    /// Index of the root node (if any).
    pub root: Option<usize>,
}
impl CallGraph {
    /// Create an empty call graph.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a node and return its index.
    pub fn add_node(&mut self, name: impl Into<String>) -> usize {
        let idx = self.nodes.len();
        self.nodes.push(CallNode::new(name));
        idx
    }
    /// Add a child edge from `parent` to `child`.
    pub fn add_edge(&mut self, parent: usize, child: usize) {
        if let Some(node) = self.nodes.get_mut(parent) {
            if !node.children.contains(&child) {
                node.children.push(child);
            }
        }
    }
    /// Find the node index by name.
    pub fn find_node(&self, name: &str) -> Option<usize> {
        self.nodes.iter().position(|n| n.name == name)
    }
    /// Total number of nodes.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
    /// DFS visitor: calls `visitor` on each reachable node index from `start`.
    pub fn dfs_visit(&self, start: usize, visitor: &mut dyn FnMut(usize)) {
        fn dfs_inner(
            graph: &CallGraph,
            idx: usize,
            visited: &mut Vec<bool>,
            visitor: &mut dyn FnMut(usize),
        ) {
            if idx >= visited.len() || visited[idx] {
                return;
            }
            visited[idx] = true;
            visitor(idx);
            if let Some(node) = graph.nodes.get(idx) {
                for &child in &node.children {
                    dfs_inner(graph, child, visited, visitor);
                }
            }
        }
        let mut visited = vec![false; self.nodes.len()];
        dfs_inner(self, start, &mut visited, visitor);
    }
    /// All node indices reachable from `start` via DFS.
    pub fn reachable_from(&self, start: usize) -> Vec<usize> {
        let mut result = Vec::new();
        self.dfs_visit(start, &mut |idx| result.push(idx));
        result
    }
    /// Nodes sorted by inclusive time, descending.
    pub fn top_by_inclusive(&self, n: usize) -> Vec<&CallNode> {
        let mut sorted: Vec<&CallNode> = self.nodes.iter().collect();
        sorted.sort_by(|a, b| b.inclusive_ns.cmp(&a.inclusive_ns));
        sorted.into_iter().take(n).collect()
    }
}
/// Comparison between two profile reports (baseline vs. candidate).
#[derive(Debug, Clone)]
pub struct ProfileComparison {
    /// Name of the baseline run.
    pub baseline_label: String,
    /// Name of the candidate run.
    pub candidate_label: String,
    /// Speedup factor (baseline_total / candidate_total). >1 means candidate is faster.
    pub speedup: f64,
    /// Per-phase speedup factors.
    pub phase_speedups: HashMap<String, f64>,
    /// Whether this is an overall improvement.
    pub is_improvement: bool,
}
impl ProfileComparison {
    /// Compare two reports.
    pub fn compare(
        baseline_label: impl Into<String>,
        baseline: &ProfileReport,
        candidate_label: impl Into<String>,
        candidate: &ProfileReport,
    ) -> Self {
        let speedup = if candidate.total_time_ms == 0.0 {
            f64::INFINITY
        } else {
            baseline.total_time_ms / candidate.total_time_ms
        };
        let mut phase_speedups = HashMap::new();
        for (phase, &base_ms) in &baseline.phase_totals {
            if let Some(&cand_ms) = candidate.phase_totals.get(phase) {
                let ps = if cand_ms == 0.0 {
                    f64::INFINITY
                } else {
                    base_ms / cand_ms
                };
                phase_speedups.insert(phase.clone(), ps);
            }
        }
        Self {
            baseline_label: baseline_label.into(),
            candidate_label: candidate_label.into(),
            speedup,
            phase_speedups,
            is_improvement: speedup > 1.0,
        }
    }
    /// Phases where candidate is slower than baseline (speedup < 1.0).
    pub fn regressions(&self) -> Vec<(&str, f64)> {
        self.phase_speedups
            .iter()
            .filter(|(_, &s)| s < 1.0)
            .map(|(name, &s)| (name.as_str(), s))
            .collect()
    }
    /// Phases where candidate is faster than baseline (speedup > 1.0).
    pub fn improvements(&self) -> Vec<(&str, f64)> {
        self.phase_speedups
            .iter()
            .filter(|(_, &s)| s > 1.0)
            .map(|(name, &s)| (name.as_str(), s))
            .collect()
    }
    /// Format as a text summary.
    pub fn summary(&self) -> String {
        let dir = if self.is_improvement {
            "improvement"
        } else {
            "regression"
        };
        format!(
            "{} vs {}: {:.2}x overall {} | {} phase regressions, {} phase improvements",
            self.baseline_label,
            self.candidate_label,
            self.speedup,
            dir,
            self.regressions().len(),
            self.improvements().len()
        )
    }
}
/// A record of a single unification attempt.
#[derive(Debug, Clone)]
pub struct UnificationRecord {
    /// Kind of constraint.
    pub kind: UnifKind,
    /// LHS description (short string).
    pub lhs: String,
    /// RHS description (short string).
    pub rhs: String,
    /// Time taken in nanoseconds.
    pub duration_ns: u64,
    /// Whether unification succeeded.
    pub succeeded: bool,
    /// Number of metavariables assigned during this unification.
    pub metavars_assigned: usize,
}
impl UnificationRecord {
    /// Create a new record.
    pub fn new(
        kind: UnifKind,
        lhs: impl Into<String>,
        rhs: impl Into<String>,
        duration_ns: u64,
        succeeded: bool,
    ) -> Self {
        Self {
            kind,
            lhs: lhs.into(),
            rhs: rhs.into(),
            duration_ns,
            succeeded,
            metavars_assigned: 0,
        }
    }
}
/// Difference between two memory snapshots.
#[derive(Debug, Clone)]
pub struct MemSnapshotDiff {
    /// Change in live objects (positive = more objects).
    pub delta_objects: isize,
    /// Change in allocated bytes (positive = more memory).
    pub delta_bytes: isize,
    /// Label of the 'before' snapshot.
    pub from_label: String,
    /// Label of the 'after' snapshot.
    pub to_label: String,
}
impl MemSnapshotDiff {
    /// Compute the diff from `before` to `after`.
    pub fn compute(before: &MemSnapshot, after: &MemSnapshot) -> Self {
        Self {
            delta_objects: after.live_objects as isize - before.live_objects as isize,
            delta_bytes: after.allocated_bytes as isize - before.allocated_bytes as isize,
            from_label: before.label.clone(),
            to_label: after.label.clone(),
        }
    }
    /// Whether memory grew.
    pub fn is_growth(&self) -> bool {
        self.delta_bytes > 0
    }
    /// Whether memory shrank.
    pub fn is_shrinkage(&self) -> bool {
        self.delta_bytes < 0
    }
}
/// Profile for an entire tactic block (a `by ...` proof).
#[derive(Debug, Clone, Default)]
pub struct TacticBlockProfile {
    /// Individual tactic profiles in execution order.
    pub steps: Vec<TacticProfile>,
    /// Total time for the entire block in nanoseconds.
    pub total_ns: u64,
    /// Whether the block completed successfully.
    pub completed: bool,
}
impl TacticBlockProfile {
    /// Create a new empty tactic block profile.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a tactic step.
    pub fn add_step(&mut self, step: TacticProfile) {
        self.total_ns += step.duration_ns;
        self.steps.push(step);
    }
    /// Number of tactic steps.
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }
    /// Number of successful steps.
    pub fn successful_steps(&self) -> usize {
        self.steps.iter().filter(|s| s.succeeded).count()
    }
    /// The slowest tactic step.
    pub fn slowest_step(&self) -> Option<&TacticProfile> {
        self.steps.iter().max_by_key(|s| s.duration_ns)
    }
    /// Average time per step in nanoseconds.
    pub fn avg_step_ns(&self) -> f64 {
        if self.steps.is_empty() {
            0.0
        } else {
            self.total_ns as f64 / self.steps.len() as f64
        }
    }
    /// Total goals closed across all steps.
    pub fn total_goals_closed(&self) -> isize {
        self.steps.iter().map(|s| s.goals_closed()).sum()
    }
    /// Aggregate time per tactic name.
    pub fn time_by_tactic(&self) -> HashMap<String, u64> {
        let mut map: HashMap<String, u64> = HashMap::new();
        for step in &self.steps {
            *map.entry(step.tactic_name.clone()).or_insert(0) += step.duration_ns;
        }
        map
    }
    /// Count per tactic name.
    pub fn count_by_tactic(&self) -> HashMap<String, usize> {
        let mut map: HashMap<String, usize> = HashMap::new();
        for step in &self.steps {
            *map.entry(step.tactic_name.clone()).or_insert(0) += 1;
        }
        map
    }
}
/// Timing data for a single elaboration phase.
#[derive(Debug, Clone)]
pub struct PhaseTimer {
    /// Which phase was timed.
    pub phase: ElabPhase,
    /// Total elapsed time in nanoseconds.
    pub duration_ns: u64,
    /// Number of times this phase was entered.
    pub call_count: usize,
}
impl PhaseTimer {
    /// Creates a new PhaseTimer for the given phase.
    pub fn new(phase: ElabPhase, duration_ns: u64, call_count: usize) -> Self {
        Self {
            phase,
            duration_ns,
            call_count,
        }
    }
    /// Returns the duration in milliseconds.
    pub fn duration_ms(&self) -> f64 {
        self.duration_ns as f64 / 1_000_000.0
    }
}
/// Aggregated profiling report across all elaborated declarations.
#[derive(Debug, Clone)]
pub struct ProfileReport {
    /// Total number of declarations profiled.
    pub total_decls: usize,
    /// Total elaboration time in milliseconds.
    pub total_time_ms: f64,
    /// Average elaboration time per declaration in milliseconds.
    pub avg_time_ms: f64,
    /// Maximum single-declaration elaboration time in milliseconds.
    pub max_time_ms: f64,
    /// Total time per phase name in milliseconds.
    pub phase_totals: HashMap<String, f64>,
}
impl ProfileReport {
    /// Creates an empty ProfileReport.
    pub fn new() -> Self {
        Self {
            total_decls: 0,
            total_time_ms: 0.0,
            avg_time_ms: 0.0,
            max_time_ms: 0.0,
            phase_totals: HashMap::new(),
        }
    }
    /// Returns a formatted text table summarising the report.
    pub fn format_table(&self) -> String {
        let mut lines = vec![
            format!("Declarations : {}", self.total_decls),
            format!("Total time   : {:.3}ms", self.total_time_ms),
            format!("Average time : {:.3}ms", self.avg_time_ms),
            format!("Max time     : {:.3}ms", self.max_time_ms),
            String::from("--- Phase Totals ---"),
        ];
        let mut phases: Vec<(&String, &f64)> = self.phase_totals.iter().collect();
        phases.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap_or(std::cmp::Ordering::Equal));
        for (name, ms) in phases {
            lines.push(format!("  {:<20} {:.3}ms", name, ms));
        }
        lines.join("\n")
    }
}
/// Simple memory allocation tracker that records snapshots.
#[derive(Debug, Default)]
pub struct MemTracker {
    /// Snapshots in chronological order.
    pub snapshots: Vec<MemSnapshot>,
}
impl MemTracker {
    /// Create a new tracker.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a snapshot.
    pub fn snapshot(&mut self, snap: MemSnapshot) {
        self.snapshots.push(snap);
    }
    /// Return all consecutive diffs.
    pub fn diffs(&self) -> Vec<MemSnapshotDiff> {
        self.snapshots
            .windows(2)
            .map(|w| MemSnapshotDiff::compute(&w[0], &w[1]))
            .collect()
    }
    /// Total bytes allocated across all snapshots (sum of deltas that grew).
    pub fn total_growth_bytes(&self) -> isize {
        self.diffs().iter().map(|d| d.delta_bytes.max(0)).sum()
    }
    /// Number of snapshots recorded.
    pub fn len(&self) -> usize {
        self.snapshots.len()
    }
    /// Whether no snapshots have been recorded.
    pub fn is_empty(&self) -> bool {
        self.snapshots.is_empty()
    }
}
/// A detected hotspot in the elaboration profile.
#[derive(Debug, Clone)]
pub struct Hotspot {
    /// Name of the hotspot location.
    pub name: String,
    /// Total time attributed to this hotspot in nanoseconds.
    pub total_ns: u64,
    /// Percentage of overall time (0.0..=100.0).
    pub pct: f64,
    /// Recommendation for optimization.
    pub recommendation: String,
}
impl Hotspot {
    /// Create a new hotspot.
    pub fn new(
        name: impl Into<String>,
        total_ns: u64,
        pct: f64,
        recommendation: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            total_ns,
            pct,
            recommendation: recommendation.into(),
        }
    }
}
/// A hot-path report derived from profiling data.
#[derive(Clone, Debug, Default)]
pub struct ElabHotPathReport {
    /// Top declarations by total time.
    pub top_decls: Vec<(String, u64)>,
    /// Top tactics by total time.
    pub top_tactics: Vec<(String, u64)>,
    /// Top unification kinds by count.
    pub top_unif_kinds: Vec<(String, u64)>,
    /// Threshold used to define "hot".
    pub threshold_ns: u64,
}
impl ElabHotPathReport {
    /// Create an empty report with the given threshold.
    pub fn new(threshold_ns: u64) -> Self {
        Self {
            threshold_ns,
            ..Default::default()
        }
    }
    /// Add a declaration to the report.
    pub fn add_decl(&mut self, name: impl Into<String>, total_ns: u64) {
        if total_ns >= self.threshold_ns {
            self.top_decls.push((name.into(), total_ns));
            self.top_decls.sort_by(|a, b| b.1.cmp(&a.1));
        }
    }
    /// Add a tactic to the report.
    pub fn add_tactic(&mut self, name: impl Into<String>, total_ns: u64) {
        if total_ns >= self.threshold_ns {
            self.top_tactics.push((name.into(), total_ns));
            self.top_tactics.sort_by(|a, b| b.1.cmp(&a.1));
        }
    }
    /// Return the total time accounted for in declarations.
    pub fn total_decl_time_ns(&self) -> u64 {
        self.top_decls.iter().map(|(_, t)| t).sum()
    }
    /// Return the total time accounted for in tactics.
    pub fn total_tactic_time_ns(&self) -> u64 {
        self.top_tactics.iter().map(|(_, t)| t).sum()
    }
    /// Produce a formatted report string.
    pub fn format(&self) -> String {
        let mut out = String::new();
        out.push_str("=== Hot-Path Report ===\n");
        out.push_str("Top declarations:\n");
        for (name, ns) in &self.top_decls {
            out.push_str(&format!("  {:40} {:>10}ns\n", name, ns));
        }
        out.push_str("Top tactics:\n");
        for (name, ns) in &self.top_tactics {
            out.push_str(&format!("  {:40} {:>10}ns\n", name, ns));
        }
        out
    }
}
/// A log of profiling events for a single elaboration session.
#[derive(Clone, Debug, Default)]
pub struct ProfilingEventLog {
    events: Vec<ProfilingEvent>,
}
impl ProfilingEventLog {
    /// Create an empty log.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record an event.
    pub fn record(&mut self, event: ProfilingEvent) {
        self.events.push(event);
    }
    /// Return the number of recorded events.
    pub fn len(&self) -> usize {
        self.events.len()
    }
    /// Return `true` if no events have been recorded.
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
    /// Return all events for a given phase.
    pub fn for_phase(&self, phase: ElabPhase) -> Vec<&ProfilingEvent> {
        self.events.iter().filter(|e| e.phase == phase).collect()
    }
    /// Total duration of all events (nanoseconds).
    pub fn total_duration_ns(&self) -> u64 {
        self.events.iter().map(|e| e.duration_ns).sum()
    }
    /// Average duration per event.
    pub fn avg_duration_ns(&self) -> f64 {
        if self.events.is_empty() {
            0.0
        } else {
            self.total_duration_ns() as f64 / self.events.len() as f64
        }
    }
    /// Return the slowest event.
    pub fn slowest(&self) -> Option<&ProfilingEvent> {
        self.events.iter().max_by_key(|e| e.duration_ns)
    }
}
/// Profile for a single tactic invocation.
#[derive(Debug, Clone)]
pub struct TacticProfile {
    /// Name of the tactic (e.g. "intro", "apply", "simp").
    pub tactic_name: String,
    /// Time taken in nanoseconds.
    pub duration_ns: u64,
    /// Number of goals before the tactic.
    pub goals_before: usize,
    /// Number of goals after the tactic.
    pub goals_after: usize,
    /// Whether the tactic succeeded.
    pub succeeded: bool,
}
impl TacticProfile {
    /// Create a new tactic profile.
    pub fn new(
        tactic_name: impl Into<String>,
        duration_ns: u64,
        goals_before: usize,
        goals_after: usize,
        succeeded: bool,
    ) -> Self {
        Self {
            tactic_name: tactic_name.into(),
            duration_ns,
            goals_before,
            goals_after,
            succeeded,
        }
    }
    /// Goals closed by this tactic (positive means goals were eliminated).
    pub fn goals_closed(&self) -> isize {
        self.goals_before as isize - self.goals_after as isize
    }
    /// Efficiency: goals closed per millisecond. Returns 0.0 for zero time.
    pub fn efficiency(&self) -> f64 {
        let ms = self.duration_ns as f64 / 1_000_000.0;
        if ms == 0.0 {
            0.0
        } else {
            self.goals_closed().max(0) as f64 / ms
        }
    }
}
/// Main profiler that accumulates profiles across multiple declarations.
#[derive(Debug, Default)]
pub struct ElabProfiler {
    /// Profiles collected, one per declaration.
    pub decl_profiles: Vec<DeclProfile>,
    /// Cumulative time across all declarations in nanoseconds.
    pub total_time_ns: u64,
    /// Whether profiling is currently active.
    pub enabled: bool,
}
impl ElabProfiler {
    /// Creates a new, disabled profiler.
    pub fn new() -> Self {
        Self {
            decl_profiles: Vec::new(),
            total_time_ns: 0,
            enabled: false,
        }
    }
    /// Enables profiling.
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    /// Disables profiling.
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    /// Starts profiling a new declaration. Returns the index into `decl_profiles`.
    pub fn start_decl(&mut self, name: &str) -> usize {
        let idx = self.decl_profiles.len();
        self.decl_profiles.push(DeclProfile::new(name));
        idx
    }
    /// Records a phase timing for the declaration at `idx`.
    pub fn record_phase(&mut self, idx: usize, phase: ElabPhase, duration_ns: u64) {
        if !self.enabled {
            return;
        }
        if let Some(profile) = self.decl_profiles.get_mut(idx) {
            profile.add_phase(phase, duration_ns);
        }
    }
    /// Finalises the declaration profile at `idx`, accumulating into the global total.
    pub fn finish_decl(&mut self, idx: usize) {
        if let Some(profile) = self.decl_profiles.get(idx) {
            self.total_time_ns += profile.total_ns;
        }
    }
    /// Returns the `n` slowest declaration profiles, sorted descending by total time.
    pub fn top_slow(&self, n: usize) -> Vec<&DeclProfile> {
        let mut sorted: Vec<&DeclProfile> = self.decl_profiles.iter().collect();
        sorted.sort_by(|a, b| b.total_ns.cmp(&a.total_ns));
        sorted.into_iter().take(n).collect()
    }
    /// Generates a full ProfileReport from accumulated data.
    pub fn report(&self) -> ProfileReport {
        let total_decls = self.decl_profiles.len();
        let total_time_ms = self.total_time_ns as f64 / 1_000_000.0;
        let avg_time_ms = if total_decls > 0 {
            total_time_ms / total_decls as f64
        } else {
            0.0
        };
        let max_time_ms = self
            .decl_profiles
            .iter()
            .map(|p| p.total_ms())
            .fold(0.0_f64, f64::max);
        let mut phase_totals: HashMap<String, f64> = HashMap::new();
        for profile in &self.decl_profiles {
            for pt in &profile.phases {
                *phase_totals
                    .entry(pt.phase.name().to_string())
                    .or_insert(0.0) += pt.duration_ms();
            }
        }
        ProfileReport {
            total_decls,
            total_time_ms,
            avg_time_ms,
            max_time_ms,
            phase_totals,
        }
    }
    /// Clears all collected profiles and resets counters.
    pub fn reset(&mut self) {
        self.decl_profiles.clear();
        self.total_time_ns = 0;
    }
}
/// Detects hotspots in an elaboration profile.
#[derive(Debug, Default)]
pub struct HotspotDetector {
    /// Threshold percentage above which something is considered a hotspot.
    pub threshold_pct: f64,
}
impl HotspotDetector {
    /// Create a detector with the given percentage threshold.
    pub fn new(threshold_pct: f64) -> Self {
        Self { threshold_pct }
    }
    /// Detect hotspots from a list of (name, ns) measurements.
    pub fn detect(&self, measurements: &[(String, u64)]) -> Vec<Hotspot> {
        let total_ns: u64 = measurements.iter().map(|(_, ns)| *ns).sum();
        if total_ns == 0 {
            return Vec::new();
        }
        let mut hotspots: Vec<Hotspot> = measurements
            .iter()
            .filter_map(|(name, ns)| {
                let ns = *ns;
                let pct = ns as f64 / total_ns as f64 * 100.0;
                if pct >= self.threshold_pct {
                    let recommendation = if pct > 50.0 {
                        format!(
                            "Critical: {} dominates elaboration time ({:.1}%)",
                            name, pct
                        )
                    } else {
                        format!("Consider optimising {} ({:.1}% of total)", name, pct)
                    };
                    Some(Hotspot::new(name.clone(), ns, pct, recommendation))
                } else {
                    None
                }
            })
            .collect();
        hotspots.sort_by(|a, b| b.total_ns.cmp(&a.total_ns));
        hotspots
    }
    /// Detect hotspots directly from a `ProfileReport`.
    pub fn detect_from_report(&self, report: &ProfileReport) -> Vec<Hotspot> {
        let measurements: Vec<(String, u64)> = report
            .phase_totals
            .iter()
            .map(|(name, &ms)| (name.clone(), (ms * 1_000_000.0) as u64))
            .collect();
        self.detect(&measurements)
    }
}
/// Kind of unification constraint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnifKind {
    /// Type equality constraint.
    TypeEq,
    /// Term equality constraint.
    TermEq,
    /// Subtype constraint.
    Subtype,
    /// Instance constraint.
    Instance,
}
impl UnifKind {
    /// Human-readable label.
    pub fn label(&self) -> &'static str {
        match self {
            UnifKind::TypeEq => "TypeEq",
            UnifKind::TermEq => "TermEq",
            UnifKind::Subtype => "Subtype",
            UnifKind::Instance => "Instance",
        }
    }
}
/// Extended per-declaration profile that includes span information.
#[derive(Debug, Clone)]
pub struct DeclSpanProfile {
    /// Declaration name.
    pub name: String,
    /// The source span of this declaration.
    pub span: ElabSpan,
    /// Total elaboration time in nanoseconds.
    pub total_ns: u64,
    /// Per-phase timings (phase name → ns).
    pub phase_ns: HashMap<String, u64>,
    /// Number of tactic steps in this declaration's proof.
    pub tactic_steps: usize,
    /// Number of type annotations inserted by the elaborator.
    pub annotations_inserted: usize,
}
impl DeclSpanProfile {
    /// Create a new profile for a declaration with the given span.
    pub fn new(name: impl Into<String>, span: ElabSpan) -> Self {
        Self {
            name: name.into(),
            span,
            total_ns: 0,
            phase_ns: HashMap::new(),
            tactic_steps: 0,
            annotations_inserted: 0,
        }
    }
    /// Record time for a named phase.
    pub fn record_phase_ns(&mut self, phase: &str, ns: u64) {
        *self.phase_ns.entry(phase.to_string()).or_insert(0) += ns;
        self.total_ns += ns;
    }
    /// Total time in milliseconds.
    pub fn total_ms(&self) -> f64 {
        self.total_ns as f64 / 1_000_000.0
    }
    /// Nanoseconds per source byte (throughput metric).
    pub fn ns_per_byte(&self) -> f64 {
        let bytes = self.span.len();
        if bytes == 0 {
            0.0
        } else {
            self.total_ns as f64 / bytes as f64
        }
    }
    /// Dominant phase by time.
    pub fn dominant_phase(&self) -> Option<(&str, u64)> {
        self.phase_ns
            .iter()
            .max_by_key(|(_, &ns)| ns)
            .map(|(name, &ns)| (name.as_str(), ns))
    }
}
/// Phases of elaboration that can be individually profiled.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ElabPhase {
    /// Parsing source text into an AST.
    Parsing,
    /// Type inference for expressions.
    TypeInference,
    /// Unification of types and terms.
    Unification,
    /// Typeclass instance synthesis.
    InstanceSynthesis,
    /// Tactic block evaluation.
    TacticEval,
    /// Pattern matching compilation.
    PatternMatch,
    /// Kernel type checking.
    TermCheck,
    /// Type checking phase (alias).
    TypeCheck,
    /// General elaboration phase.
    Elaboration,
}
impl ElabPhase {
    /// Returns a human-readable name for the phase.
    pub fn name(&self) -> &'static str {
        match self {
            ElabPhase::Parsing => "Parsing",
            ElabPhase::TypeInference => "TypeInference",
            ElabPhase::Unification => "Unification",
            ElabPhase::InstanceSynthesis => "InstanceSynthesis",
            ElabPhase::TacticEval => "TacticEval",
            ElabPhase::PatternMatch => "PatternMatch",
            ElabPhase::TermCheck => "TermCheck",
            ElabPhase::TypeCheck => "TypeCheck",
            ElabPhase::Elaboration => "Elaboration",
        }
    }
}
/// Lightweight statistical (sampling) profiler.
///
/// Instead of measuring every call, it decides probabilistically whether to
/// record a sample based on a target sampling rate.
pub struct SamplingProfiler {
    /// Target number of samples to collect per second (informational only here;
    /// `should_sample` uses a simple counter-based heuristic).
    sample_rate_hz: f64,
    /// Counts per phase name.
    samples: HashMap<String, usize>,
    /// Total samples recorded.
    pub(crate) total_samples: usize,
    /// Internal counter used by `should_sample`.
    counter: usize,
}
impl SamplingProfiler {
    /// Creates a new SamplingProfiler with the given target rate.
    pub fn new(sample_rate_hz: f64) -> Self {
        Self {
            sample_rate_hz,
            samples: HashMap::new(),
            total_samples: 0,
            counter: 0,
        }
    }
    /// Returns true if the profiler should record a sample this call.
    ///
    /// Uses a simple counter-based decision: sample every
    /// `max(1, 1000 / rate)` calls so that low rates skip most calls.
    pub fn should_sample(&mut self) -> bool {
        let skip = (1000.0 / self.sample_rate_hz.max(1.0)).round() as usize;
        let skip = skip.max(1);
        self.counter += 1;
        if self.counter >= skip {
            self.counter = 0;
            true
        } else {
            false
        }
    }
    /// Records a sample for the named phase.
    pub fn record_sample(&mut self, phase: &str) {
        *self.samples.entry(phase.to_string()).or_insert(0) += 1;
        self.total_samples += 1;
    }
    /// Returns phase names sorted descending by percentage of total samples.
    pub fn hotspots(&self) -> Vec<(String, f64)> {
        if self.total_samples == 0 {
            return Vec::new();
        }
        let mut result: Vec<(String, f64)> = self
            .samples
            .iter()
            .map(|(name, &count)| {
                let pct = count as f64 / self.total_samples as f64 * 100.0;
                (name.clone(), pct)
            })
            .collect();
        result.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        result
    }
}
/// An extended profile report with more detail than `ProfileReport`.
#[derive(Debug, Clone)]
pub struct ExtendedProfileReport {
    /// Base report.
    pub base: ProfileReport,
    /// Per-declaration profiles (summaries).
    pub decl_summaries: Vec<String>,
    /// Hotspots detected.
    pub hotspots: Vec<String>,
    /// Memory growth in bytes.
    pub memory_growth_bytes: isize,
    /// Number of tactic steps across all proofs.
    pub total_tactic_steps: usize,
    /// Number of unification attempts.
    pub total_unif_attempts: usize,
    /// Unification success rate.
    pub unif_success_rate: f64,
}
impl Default for ExtendedProfileReport {
    fn default() -> Self {
        Self {
            base: ProfileReport::default(),
            decl_summaries: Vec::new(),
            hotspots: Vec::new(),
            memory_growth_bytes: 0,
            total_tactic_steps: 0,
            total_unif_attempts: 0,
            unif_success_rate: 1.0,
        }
    }
}
impl ExtendedProfileReport {
    /// Create from a profiler and optional extra data.
    pub fn from_profiler(profiler: &ElabProfiler) -> Self {
        let base = profiler.report();
        let decl_summaries = profiler.decl_profiles.iter().map(|p| p.summary()).collect();
        let detector = HotspotDetector::new(10.0);
        let hs = detector.detect_from_report(&base);
        let hotspots = hs.iter().map(|h| h.recommendation.clone()).collect();
        Self {
            base,
            decl_summaries,
            hotspots,
            memory_growth_bytes: 0,
            total_tactic_steps: 0,
            total_unif_attempts: 0,
            unif_success_rate: 1.0,
        }
    }
    /// Format as plain text.
    pub fn format_text(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.base.format_table());
        out.push_str("\n--- Declaration Summaries ---\n");
        for s in &self.decl_summaries {
            out.push_str("  ");
            out.push_str(s);
            out.push('\n');
        }
        out.push_str("--- Hotspots ---\n");
        if self.hotspots.is_empty() {
            out.push_str("  (none)\n");
        } else {
            for h in &self.hotspots {
                out.push_str("  ! ");
                out.push_str(h);
                out.push('\n');
            }
        }
        out.push_str(&format!(
            "Memory growth: {} bytes\nTactic steps: {}\nUnification attempts: {} (rate: {:.1}%)\n",
            self.memory_growth_bytes,
            self.total_tactic_steps,
            self.total_unif_attempts,
            self.unif_success_rate * 100.0,
        ));
        out
    }
    /// Format as a JSON-like string (no external dependencies).
    pub fn format_json(&self) -> String {
        let mut out = String::new();
        out.push_str("{\n");
        out.push_str(&format!(
            "  \"total_decls\": {},\n  \"total_time_ms\": {:.3},\n  \"avg_time_ms\": {:.3},\n",
            self.base.total_decls, self.base.total_time_ms, self.base.avg_time_ms
        ));
        out.push_str(&format!(
            "  \"memory_growth_bytes\": {},\n  \"total_tactic_steps\": {},\n",
            self.memory_growth_bytes, self.total_tactic_steps
        ));
        out.push_str(&format!(
            "  \"unif_attempts\": {},\n  \"unif_success_rate\": {:.4},\n",
            self.total_unif_attempts, self.unif_success_rate
        ));
        out.push_str("  \"hotspots\": [\n");
        for (i, h) in self.hotspots.iter().enumerate() {
            let comma = if i + 1 < self.hotspots.len() { "," } else { "" };
            out.push_str(&format!("    \"{}\"{}\n", h.replace('"', "'"), comma));
        }
        out.push_str("  ]\n}\n");
        out
    }
}
/// A single frame in a flamegraph stack.
#[derive(Debug, Clone)]
pub struct FlameFrame {
    /// Function/node name.
    pub name: String,
    /// Self time (exclusive) in nanoseconds.
    pub self_ns: u64,
    /// Total time (inclusive) in nanoseconds.
    pub total_ns: u64,
    /// Depth in the call stack.
    pub depth: usize,
    /// Parent frame index (None for roots).
    pub parent: Option<usize>,
}
impl FlameFrame {
    /// Create a new frame.
    pub fn new(name: impl Into<String>, self_ns: u64, total_ns: u64, depth: usize) -> Self {
        Self {
            name: name.into(),
            self_ns,
            total_ns,
            depth,
            parent: None,
        }
    }
    /// Fraction of total time (0.0..=1.0) relative to a root total.
    pub fn fraction(&self, root_total_ns: u64) -> f64 {
        if root_total_ns == 0 {
            0.0
        } else {
            self.total_ns as f64 / root_total_ns as f64
        }
    }
}
/// Profiling data collected for a single declaration elaboration.
#[derive(Debug, Clone)]
pub struct DeclProfile {
    /// Name of the declaration being profiled.
    pub name: String,
    /// Total elapsed time in nanoseconds.
    pub total_ns: u64,
    /// Per-phase timing breakdown.
    pub phases: Vec<PhaseTimer>,
    /// Number of metavariables created during elaboration.
    pub num_metavars: usize,
    /// Number of tactic goals created during elaboration.
    pub num_goals_created: usize,
}
impl DeclProfile {
    /// Creates a new empty DeclProfile for the named declaration.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            total_ns: 0,
            phases: Vec::new(),
            num_metavars: 0,
            num_goals_created: 0,
        }
    }
    /// Records time spent in a given phase (accumulates if phase already present).
    pub fn add_phase(&mut self, phase: ElabPhase, duration_ns: u64) {
        self.total_ns += duration_ns;
        if let Some(existing) = self.phases.iter_mut().find(|p| p.phase == phase) {
            existing.duration_ns += duration_ns;
            existing.call_count += 1;
        } else {
            self.phases.push(PhaseTimer::new(phase, duration_ns, 1));
        }
    }
    /// Returns total elaboration time in milliseconds.
    pub fn total_ms(&self) -> f64 {
        self.total_ns as f64 / 1_000_000.0
    }
    /// Returns a reference to the slowest phase, if any phases were recorded.
    pub fn slowest_phase(&self) -> Option<&PhaseTimer> {
        self.phases.iter().max_by_key(|p| p.duration_ns)
    }
    /// Returns a human-readable summary line for this declaration.
    pub fn summary(&self) -> String {
        let slowest = self
            .slowest_phase()
            .map(|p| format!(" (slowest: {} {:.3}ms)", p.phase.name(), p.duration_ms()))
            .unwrap_or_default();
        format!(
            "{}: {:.3}ms, {} metavars, {} goals{}",
            self.name,
            self.total_ms(),
            self.num_metavars,
            self.num_goals_created,
            slowest,
        )
    }
}
/// A single profiling event.
#[derive(Clone, Debug)]
pub struct ProfilingEvent {
    /// Human-readable label for the event.
    pub label: String,
    /// Phase in which the event occurred.
    pub phase: ElabPhase,
    /// Timestamp (nanoseconds since profiling start).
    pub timestamp_ns: u64,
    /// Duration of the event in nanoseconds.
    pub duration_ns: u64,
}
impl ProfilingEvent {
    /// Create a new event.
    pub fn new(
        label: impl Into<String>,
        phase: ElabPhase,
        timestamp_ns: u64,
        duration_ns: u64,
    ) -> Self {
        Self {
            label: label.into(),
            phase,
            timestamp_ns,
            duration_ns,
        }
    }
}
/// Additional timing breakdown fields for the extended report.
#[derive(Clone, Debug, Default)]
pub struct TimingBreakdown {
    /// Time spent in type-checking (nanoseconds).
    pub type_check_ns: u64,
    /// Time spent in unification (nanoseconds).
    pub unification_ns: u64,
    /// Time spent in tactic evaluation (nanoseconds).
    pub tactic_eval_ns: u64,
    /// Time spent in simp (nanoseconds).
    pub simp_ns: u64,
    /// Time spent in instance synthesis (nanoseconds).
    pub instance_synth_ns: u64,
}
impl TimingBreakdown {
    /// Create zeroed breakdown.
    pub fn new() -> Self {
        Self::default()
    }
    /// Total time across all categories.
    pub fn total_ns(&self) -> u64 {
        self.type_check_ns
            + self.unification_ns
            + self.tactic_eval_ns
            + self.simp_ns
            + self.instance_synth_ns
    }
    /// Fraction of time spent in type-checking.
    pub fn type_check_fraction(&self) -> f64 {
        let total = self.total_ns();
        if total == 0 {
            0.0
        } else {
            self.type_check_ns as f64 / total as f64
        }
    }
    /// Summary string.
    pub fn summary(&self) -> String {
        format!(
            "typecheck={}ns unif={}ns tactics={}ns simp={}ns synth={}ns",
            self.type_check_ns,
            self.unification_ns,
            self.tactic_eval_ns,
            self.simp_ns,
            self.instance_synth_ns,
        )
    }
}
/// Profiler that collects unification records.
#[derive(Debug, Default)]
pub struct UnificationProfiler {
    /// All recorded unification attempts.
    pub records: Vec<UnificationRecord>,
    /// Total time spent in unification in nanoseconds.
    pub total_ns: u64,
}
impl UnificationProfiler {
    /// Create a new profiler.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a unification record.
    pub fn add(&mut self, record: UnificationRecord) {
        self.total_ns += record.duration_ns;
        self.records.push(record);
    }
    /// Number of recorded unifications.
    pub fn count(&self) -> usize {
        self.records.len()
    }
    /// Number of successful unifications.
    pub fn successful(&self) -> usize {
        self.records.iter().filter(|r| r.succeeded).count()
    }
    /// Number of failed unifications.
    pub fn failed(&self) -> usize {
        self.records.iter().filter(|r| !r.succeeded).count()
    }
    /// Success rate in [0.0, 1.0].
    pub fn success_rate(&self) -> f64 {
        if self.records.is_empty() {
            1.0
        } else {
            self.successful() as f64 / self.records.len() as f64
        }
    }
    /// Total time in milliseconds.
    pub fn total_ms(&self) -> f64 {
        self.total_ns as f64 / 1_000_000.0
    }
    /// Top-N slowest unification records.
    pub fn top_slow(&self, n: usize) -> Vec<&UnificationRecord> {
        let mut sorted: Vec<&UnificationRecord> = self.records.iter().collect();
        sorted.sort_by(|a, b| b.duration_ns.cmp(&a.duration_ns));
        sorted.into_iter().take(n).collect()
    }
    /// Time breakdown by unification kind.
    pub fn time_by_kind(&self) -> HashMap<String, u64> {
        let mut map: HashMap<String, u64> = HashMap::new();
        for r in &self.records {
            *map.entry(r.kind.label().to_string()).or_insert(0) += r.duration_ns;
        }
        map
    }
    /// Count breakdown by unification kind.
    pub fn count_by_kind(&self) -> HashMap<String, usize> {
        let mut map: HashMap<String, usize> = HashMap::new();
        for r in &self.records {
            *map.entry(r.kind.label().to_string()).or_insert(0) += 1;
        }
        map
    }
}
