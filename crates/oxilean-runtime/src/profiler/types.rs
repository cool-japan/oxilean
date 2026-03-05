//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;

use super::functions::profiler_now_ns;

/// A single entry in the profiling timeline.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct TimelineEntry {
    /// Timestamp in nanoseconds.
    pub timestamp_ns: u64,
    /// Short description.
    pub label: String,
    /// Duration in nanoseconds (0 for instantaneous events).
    pub duration_ns: u64,
    /// Category tag.
    pub category: String,
}
impl TimelineEntry {
    /// Create a new timeline entry.
    #[allow(dead_code)]
    pub fn new(timestamp_ns: u64, label: &str, duration_ns: u64, category: &str) -> Self {
        Self {
            timestamp_ns,
            label: label.to_string(),
            duration_ns,
            category: category.to_string(),
        }
    }
}
/// A comprehensive profiling report combining all profiler outputs.
#[allow(dead_code)]
pub struct ComprehensiveProfilingReport {
    /// Event-based profile report.
    pub event_report: ProfileReport,
    /// Memory profile.
    pub memory_profile: MemoryProfile,
    /// Flat sampling profile.
    pub flat_profile: Vec<(String, usize)>,
    /// Cumulative sampling profile.
    pub cumulative_profile: Vec<(String, usize)>,
    /// GC summary.
    pub gc_summary: String,
}
impl ComprehensiveProfilingReport {
    /// Build a comprehensive report from a session.
    #[allow(dead_code)]
    pub fn build(session: &ProfilingSession) -> Self {
        let event_report = session.profiler.generate_report();
        let memory_profile = session.profiler.memory_profile();
        let flat_profile = session.sampler.flat_profile();
        let cumulative_profile = session.sampler.cumulative_profile();
        let gc_summary = format!(
            "GC cycles: {}, total alloc: {} bytes",
            event_report.gc_cycles, memory_profile.total_allocs,
        );
        Self {
            event_report,
            memory_profile,
            flat_profile,
            cumulative_profile,
            gc_summary,
        }
    }
    /// Format as a text report.
    #[allow(dead_code)]
    pub fn to_text(&self) -> String {
        let mut out = self.event_report.to_text();
        out.push('\n');
        out.push_str(&self.memory_profile.to_text());
        out.push('\n');
        out.push_str(&self.gc_summary);
        out.push('\n');
        if !self.flat_profile.is_empty() {
            out.push_str("\nFlat profile:\n");
            for (name, count) in &self.flat_profile {
                out.push_str(&format!("  {:40} {}\n", name, count));
            }
        }
        out
    }
}
/// Tracks GC activity across a run.
#[allow(dead_code)]
pub struct GcProfiler {
    records: Vec<GcCollectionRecord>,
}
impl GcProfiler {
    /// Create a new GC profiler.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
        }
    }
    /// Record a GC collection.
    #[allow(dead_code)]
    pub fn record(&mut self, collected: usize, live: usize, pause_ns: u64) {
        let ts = profiler_now_ns();
        self.records
            .push(GcCollectionRecord::new(ts, collected, live, pause_ns));
    }
    /// Number of GC collections recorded.
    #[allow(dead_code)]
    pub fn collection_count(&self) -> usize {
        self.records.len()
    }
    /// Total objects collected across all GC cycles.
    #[allow(dead_code)]
    pub fn total_collected(&self) -> usize {
        self.records.iter().map(|r| r.collected).sum()
    }
    /// Average pause time in nanoseconds.
    #[allow(dead_code)]
    pub fn avg_pause_ns(&self) -> f64 {
        if self.records.is_empty() {
            0.0
        } else {
            let total: u64 = self.records.iter().map(|r| r.pause_ns).sum();
            total as f64 / self.records.len() as f64
        }
    }
    /// Maximum pause time seen.
    #[allow(dead_code)]
    pub fn max_pause_ns(&self) -> u64 {
        self.records.iter().map(|r| r.pause_ns).max().unwrap_or(0)
    }
    /// Human-readable summary.
    #[allow(dead_code)]
    pub fn summary(&self) -> String {
        format!(
            "GC: {} collections, {} total collected, avg_pause={:.0}ns, max_pause={}ns",
            self.collection_count(),
            self.total_collected(),
            self.avg_pause_ns(),
            self.max_pause_ns(),
        )
    }
}
/// A log of tactic profiling events.
#[allow(dead_code)]
pub struct TacticProfileLog {
    events: Vec<TacticProfilingEvent>,
}
impl TacticProfileLog {
    /// Create a new log.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }
    /// Record an event.
    #[allow(dead_code)]
    pub fn record(&mut self, event: TacticProfilingEvent) {
        self.events.push(event);
    }
    /// Total duration of all tactic steps.
    #[allow(dead_code)]
    pub fn total_duration_ns(&self) -> u64 {
        self.events.iter().map(|e| e.duration_ns).sum()
    }
    /// Number of successful tactic applications.
    #[allow(dead_code)]
    pub fn success_count(&self) -> usize {
        self.events.iter().filter(|e| e.success).count()
    }
    /// Top N slowest tactics by duration.
    #[allow(dead_code)]
    pub fn top_slow(&self, n: usize) -> Vec<&TacticProfilingEvent> {
        let mut sorted: Vec<&TacticProfilingEvent> = self.events.iter().collect();
        sorted.sort_by(|a, b| b.duration_ns.cmp(&a.duration_ns));
        sorted.truncate(n);
        sorted
    }
    /// Average duration per tactic step.
    #[allow(dead_code)]
    pub fn avg_duration_ns(&self) -> f64 {
        if self.events.is_empty() {
            0.0
        } else {
            self.total_duration_ns() as f64 / self.events.len() as f64
        }
    }
}
/// Statistics for a single GC collection.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct GcCollectionRecord {
    /// When this collection happened.
    pub timestamp_ns: u64,
    /// Objects collected.
    pub collected: usize,
    /// Objects remaining live.
    pub live: usize,
    /// Duration of the pause in nanoseconds.
    pub pause_ns: u64,
}
impl GcCollectionRecord {
    /// Create a new GC collection record.
    #[allow(dead_code)]
    pub fn new(timestamp_ns: u64, collected: usize, live: usize, pause_ns: u64) -> Self {
        Self {
            timestamp_ns,
            collected,
            live,
            pause_ns,
        }
    }
    /// Collection efficiency: fraction of objects collected vs total seen.
    #[allow(dead_code)]
    pub fn efficiency(&self) -> f64 {
        let total = self.collected + self.live;
        if total == 0 {
            0.0
        } else {
            self.collected as f64 / total as f64
        }
    }
}
/// A single profiling event captured by the runtime.
#[derive(Clone, Debug)]
pub enum ProfilingEvent {
    /// A function was called.
    FunctionCall {
        /// Name of the function.
        name: String,
        /// Current call depth at the time of the call.
        depth: u32,
    },
    /// A function returned.
    FunctionReturn {
        /// Name of the function.
        name: String,
        /// Elapsed time in nanoseconds.
        duration_ns: u64,
    },
    /// Memory was allocated.
    Allocation {
        /// Number of bytes allocated.
        size: usize,
        /// Descriptive tag for the allocation.
        tag: String,
    },
    /// Memory was freed.
    Deallocation {
        /// Number of bytes freed.
        size: usize,
        /// Descriptive tag for the allocation.
        tag: String,
    },
    /// A garbage collection cycle completed.
    GcCycle {
        /// Number of objects collected.
        collected: usize,
        /// Number of live objects remaining.
        live: usize,
    },
    /// A tactic step was executed.
    TacticStep {
        /// Name of the tactic.
        tactic_name: String,
        /// Number of open goals after this step.
        goal_count: u32,
    },
}
/// Summary report generated from profiling data.
#[derive(Clone, Debug)]
pub struct ProfileReport {
    /// Total number of function calls recorded.
    pub total_calls: usize,
    /// Total bytes allocated.
    pub total_alloc_bytes: usize,
    /// Top 10 hottest functions sorted by total duration (name, ns).
    pub hot_functions: Vec<(String, u64)>,
    /// Number of GC cycles recorded.
    pub gc_cycles: usize,
}
impl ProfileReport {
    /// Format the report as human-readable text.
    pub fn to_text(&self) -> String {
        let mut out = String::new();
        out.push_str("=== Profile Report ===\n");
        out.push_str(&format!("Total function calls : {}\n", self.total_calls));
        out.push_str(&format!(
            "Total allocations   : {} bytes\n",
            self.total_alloc_bytes
        ));
        out.push_str(&format!("GC cycles           : {}\n", self.gc_cycles));
        if !self.hot_functions.is_empty() {
            out.push_str("\nHot functions (top 10):\n");
            for (i, (name, ns)) in self.hot_functions.iter().enumerate() {
                out.push_str(&format!("  {:2}. {:40} {:>12} ns\n", i + 1, name, ns));
            }
        }
        out
    }
    /// Format the report as a JSON string.
    pub fn to_json(&self) -> String {
        let hot_json: Vec<String> = self
            .hot_functions
            .iter()
            .map(|(name, ns)| format!("{{\"name\":\"{}\",\"duration_ns\":{}}}", name, ns))
            .collect();
        format!(
            "{{\"total_calls\":{},\"total_alloc_bytes\":{},\"gc_cycles\":{},\"hot_functions\":[{}]}}",
            self.total_calls, self.total_alloc_bytes, self.gc_cycles, hot_json.join(",")
        )
    }
}
/// A profiling event specific to tactic execution.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct TacticProfilingEvent {
    /// Name of the tactic.
    pub tactic: String,
    /// Duration in nanoseconds.
    pub duration_ns: u64,
    /// Whether the tactic succeeded.
    pub success: bool,
    /// Number of goals before the tactic.
    pub goals_before: u32,
    /// Number of goals after the tactic.
    pub goals_after: u32,
}
impl TacticProfilingEvent {
    /// Create a new tactic profiling event.
    #[allow(dead_code)]
    pub fn new(
        tactic: &str,
        duration_ns: u64,
        success: bool,
        goals_before: u32,
        goals_after: u32,
    ) -> Self {
        Self {
            tactic: tactic.to_string(),
            duration_ns,
            success,
            goals_before,
            goals_after,
        }
    }
    /// Number of goals eliminated by this tactic step.
    #[allow(dead_code)]
    pub fn goals_eliminated(&self) -> i32 {
        self.goals_before as i32 - self.goals_after as i32
    }
}
/// A heat map showing call density over time.
#[allow(dead_code)]
pub struct HeatMap {
    /// Number of time buckets.
    pub buckets: usize,
    /// Total time span covered in nanoseconds.
    pub span_ns: u64,
    /// Counts per bucket.
    pub counts: Vec<u64>,
}
impl HeatMap {
    /// Create a heat map with `buckets` time slots covering `span_ns`.
    #[allow(dead_code)]
    pub fn new(buckets: usize, span_ns: u64) -> Self {
        Self {
            buckets,
            span_ns,
            counts: vec![0; buckets],
        }
    }
    /// Record an event at the given timestamp.
    #[allow(dead_code)]
    pub fn record(&mut self, timestamp_ns: u64, start_ns: u64) {
        if self.span_ns == 0 || self.buckets == 0 {
            return;
        }
        let offset = timestamp_ns.saturating_sub(start_ns);
        let bucket = ((offset as u128 * self.buckets as u128) / self.span_ns as u128) as usize;
        let bucket = bucket.min(self.buckets - 1);
        self.counts[bucket] += 1;
    }
    /// Return the bucket with the maximum count.
    #[allow(dead_code)]
    pub fn peak_bucket(&self) -> usize {
        self.counts
            .iter()
            .enumerate()
            .max_by_key(|(_, &v)| v)
            .map(|(i, _)| i)
            .unwrap_or(0)
    }
    /// Format as ASCII art.
    #[allow(dead_code)]
    pub fn render_ascii(&self) -> String {
        let max_count = *self.counts.iter().max().unwrap_or(&1).max(&1);
        let height = 8usize;
        let mut rows: Vec<String> = Vec::new();
        for row in (0..height).rev() {
            let threshold = (row as f64 / height as f64 * max_count as f64) as u64;
            let line: String = self
                .counts
                .iter()
                .map(|&c| if c > threshold { '#' } else { ' ' })
                .collect();
            rows.push(format!("|{}", line));
        }
        rows.push(format!("+{}", "-".repeat(self.buckets)));
        rows.join("\n")
    }
}
/// An annotation on a timeline.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct TimelineAnnotation {
    /// Timestamp of the annotation.
    pub timestamp_ns: u64,
    /// Text of the annotation.
    pub text: String,
    /// Category (e.g., "checkpoint", "error").
    pub category: String,
}
impl TimelineAnnotation {
    /// Create a new annotation.
    #[allow(dead_code)]
    pub fn new(timestamp_ns: u64, text: &str, category: &str) -> Self {
        Self {
            timestamp_ns,
            text: text.to_string(),
            category: category.to_string(),
        }
    }
}
/// Simulated hardware performance counter.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct PerfCounter {
    /// Instructions retired (simulated).
    pub instructions_retired: u64,
    /// Cache misses (simulated).
    pub cache_misses: u64,
    /// Branch mispredictions (simulated).
    pub branch_mispredictions: u64,
    /// Context switches (simulated).
    pub context_switches: u64,
    /// Cycles elapsed (simulated).
    pub cycles: u64,
}
impl PerfCounter {
    /// Create zeroed performance counters.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Simulate a number of instructions.
    #[allow(dead_code)]
    pub fn simulate_instructions(&mut self, n: u64) {
        self.instructions_retired += n;
        self.cycles += n;
    }
    /// Simulate a cache miss.
    #[allow(dead_code)]
    pub fn simulate_cache_miss(&mut self) {
        self.cache_misses += 1;
        self.cycles += 200;
    }
    /// Simulate a branch misprediction.
    #[allow(dead_code)]
    pub fn simulate_branch_misprediction(&mut self) {
        self.branch_mispredictions += 1;
        self.cycles += 15;
    }
    /// IPC (instructions per cycle).
    #[allow(dead_code)]
    pub fn ipc(&self) -> f64 {
        if self.cycles == 0 {
            0.0
        } else {
            self.instructions_retired as f64 / self.cycles as f64
        }
    }
    /// Cache miss rate per 1000 instructions.
    #[allow(dead_code)]
    pub fn cache_miss_rate_per_1k(&self) -> f64 {
        if self.instructions_retired == 0 {
            0.0
        } else {
            (self.cache_misses as f64 / self.instructions_retired as f64) * 1000.0
        }
    }
    /// Format a human-readable summary.
    #[allow(dead_code)]
    pub fn summary(&self) -> String {
        format!(
            "PerfCounters: instr={}, cycles={}, IPC={:.2}, cache_misses={}, branch_mispredict={}",
            self.instructions_retired,
            self.cycles,
            self.ipc(),
            self.cache_misses,
            self.branch_mispredictions
        )
    }
}
/// Configuration for the runtime profiler.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ProfilerConfig {
    /// Whether event-based profiling is enabled.
    pub event_profiling: bool,
    /// Whether sampling-based profiling is enabled.
    pub sampling_profiling: bool,
    /// Sampling interval in nanoseconds.
    pub sampling_interval_ns: u64,
    /// Maximum number of events to store before overwriting old ones.
    pub max_events: usize,
    /// Whether to include GC events.
    pub track_gc: bool,
    /// Whether to include allocation events.
    pub track_allocs: bool,
}
impl ProfilerConfig {
    /// Create default configuration.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Enable all profiling.
    #[allow(dead_code)]
    pub fn enable_all(mut self) -> Self {
        self.event_profiling = true;
        self.sampling_profiling = true;
        self
    }
    /// Disable all profiling.
    #[allow(dead_code)]
    pub fn disable_all(mut self) -> Self {
        self.event_profiling = false;
        self.sampling_profiling = false;
        self
    }
}
/// A single sample captured by the sampling profiler.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ProfileSample {
    /// Timestamp in nanoseconds.
    pub timestamp_ns: u64,
    /// Call stack at the time of sampling (most recent first).
    pub call_stack: Vec<String>,
    /// Thread identifier (0 for main thread).
    pub thread_id: u64,
}
impl ProfileSample {
    /// Create a new sample.
    #[allow(dead_code)]
    pub fn new(timestamp_ns: u64, call_stack: Vec<String>, thread_id: u64) -> Self {
        Self {
            timestamp_ns,
            call_stack,
            thread_id,
        }
    }
    /// Returns the function at the top of the call stack, if any.
    #[allow(dead_code)]
    pub fn top_function(&self) -> Option<&str> {
        self.call_stack.first().map(|s| s.as_str())
    }
    /// Returns the depth of the call stack.
    #[allow(dead_code)]
    pub fn depth(&self) -> usize {
        self.call_stack.len()
    }
}
/// A counter step: counts events by variant name.
#[allow(dead_code)]
pub struct CountingStep {
    pub step_name: String,
    pub counts: HashMap<String, u64>,
}
impl CountingStep {
    /// Create a new counting step.
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            step_name: name.to_string(),
            counts: HashMap::new(),
        }
    }
    /// Event variant name for counting.
    pub(super) fn variant_name(event: &ProfilingEvent) -> &'static str {
        match event {
            ProfilingEvent::FunctionCall { .. } => "FunctionCall",
            ProfilingEvent::FunctionReturn { .. } => "FunctionReturn",
            ProfilingEvent::Allocation { .. } => "Allocation",
            ProfilingEvent::Deallocation { .. } => "Deallocation",
            ProfilingEvent::GcCycle { .. } => "GcCycle",
            ProfilingEvent::TacticStep { .. } => "TacticStep",
        }
    }
}
/// A simple real-time monitor that collects snapshots of key metrics.
#[allow(dead_code)]
pub struct RealTimeMonitor {
    /// Name of the monitor.
    pub name: String,
    /// Collected metric snapshots: (timestamp_ns, metric_name, value).
    pub snapshots: Vec<(u64, String, f64)>,
    /// Maximum snapshots to keep.
    pub capacity: usize,
}
impl RealTimeMonitor {
    /// Create a new monitor.
    #[allow(dead_code)]
    pub fn new(name: &str, capacity: usize) -> Self {
        Self {
            name: name.to_string(),
            snapshots: Vec::new(),
            capacity,
        }
    }
    /// Record a metric value.
    #[allow(dead_code)]
    pub fn record(&mut self, metric: &str, value: f64) {
        let ts = profiler_now_ns();
        if self.snapshots.len() >= self.capacity {
            self.snapshots.remove(0);
        }
        self.snapshots.push((ts, metric.to_string(), value));
    }
    /// Get the most recent value for a metric.
    #[allow(dead_code)]
    pub fn latest(&self, metric: &str) -> Option<f64> {
        self.snapshots
            .iter()
            .rev()
            .find(|(_, m, _)| m == metric)
            .map(|(_, _, v)| *v)
    }
    /// Average value for a metric.
    #[allow(dead_code)]
    pub fn avg(&self, metric: &str) -> f64 {
        let values: Vec<f64> = self
            .snapshots
            .iter()
            .filter(|(_, m, _)| m == metric)
            .map(|(_, _, v)| *v)
            .collect();
        if values.is_empty() {
            0.0
        } else {
            values.iter().sum::<f64>() / values.len() as f64
        }
    }
    /// Count of snapshots for a metric.
    #[allow(dead_code)]
    pub fn count(&self, metric: &str) -> usize {
        self.snapshots
            .iter()
            .filter(|(_, m, _)| m == metric)
            .count()
    }
}
/// A histogram for profiling measurements.
#[allow(dead_code)]
pub struct Histogram {
    buckets: Vec<HistogramBucket>,
    /// Total observations.
    pub total: u64,
    /// Sum of all observations (for mean).
    pub sum: f64,
}
impl Histogram {
    /// Create a histogram with `n` equal-width buckets in `[min_val, max_val]`.
    #[allow(dead_code)]
    pub fn new(n: usize, min_val: f64, max_val: f64) -> Self {
        let width = (max_val - min_val) / n as f64;
        let buckets = (0..n)
            .map(|i| HistogramBucket {
                lower: min_val + i as f64 * width,
                upper: min_val + (i + 1) as f64 * width,
                count: 0,
            })
            .collect();
        Self {
            buckets,
            total: 0,
            sum: 0.0,
        }
    }
    /// Record a value.
    #[allow(dead_code)]
    pub fn record(&mut self, value: f64) {
        self.total += 1;
        self.sum += value;
        if let Some(bucket) = self
            .buckets
            .iter_mut()
            .find(|b| value >= b.lower && value < b.upper)
        {
            bucket.count += 1;
        } else if let Some(last) = self.buckets.last_mut() {
            if value >= last.lower {
                last.count += 1;
            }
        }
    }
    /// Mean of all recorded values.
    #[allow(dead_code)]
    pub fn mean(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            self.sum / self.total as f64
        }
    }
    /// Bucket with the most observations (mode bucket).
    #[allow(dead_code)]
    pub fn mode_bucket(&self) -> Option<&HistogramBucket> {
        self.buckets.iter().max_by_key(|b| b.count)
    }
    /// Render a simple ASCII histogram.
    #[allow(dead_code)]
    pub fn render_ascii(&self) -> String {
        let max_count = self
            .buckets
            .iter()
            .map(|b| b.count)
            .max()
            .unwrap_or(1)
            .max(1);
        let bar_width = 40usize;
        let mut out = String::new();
        for bucket in &self.buckets {
            let bar_len = (bucket.count as usize * bar_width) / max_count as usize;
            let bar = "#".repeat(bar_len);
            out.push_str(&format!(
                "[{:.2}, {:.2}): {:6} | {}\n",
                bucket.lower, bucket.upper, bucket.count, bar
            ));
        }
        out
    }
}
/// Tracks allocations grouped by tag.
#[allow(dead_code)]
pub struct AllocationTracker {
    stats: HashMap<String, AllocationStat>,
}
impl AllocationTracker {
    /// Create a new tracker.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            stats: HashMap::new(),
        }
    }
    /// Record an allocation.
    #[allow(dead_code)]
    pub fn record_alloc(&mut self, tag: &str, bytes: u64) {
        let s = self.stats.entry(tag.to_string()).or_default();
        s.total_bytes += bytes;
        s.alloc_count += 1;
        s.live_bytes += bytes;
    }
    /// Record a deallocation.
    #[allow(dead_code)]
    pub fn record_dealloc(&mut self, tag: &str, bytes: u64) {
        let s = self.stats.entry(tag.to_string()).or_default();
        s.dealloc_count += 1;
        s.live_bytes = s.live_bytes.saturating_sub(bytes);
    }
    /// Get stats for a tag.
    #[allow(dead_code)]
    pub fn stats_for(&self, tag: &str) -> Option<&AllocationStat> {
        self.stats.get(tag)
    }
    /// Total live bytes across all tags.
    #[allow(dead_code)]
    pub fn total_live_bytes(&self) -> u64 {
        self.stats.values().map(|s| s.live_bytes).sum()
    }
    /// Total allocated bytes across all tags.
    #[allow(dead_code)]
    pub fn total_allocated_bytes(&self) -> u64 {
        self.stats.values().map(|s| s.total_bytes).sum()
    }
    /// Top N tags by total allocated bytes.
    #[allow(dead_code)]
    pub fn top_allocators(&self, n: usize) -> Vec<(&str, u64)> {
        let mut v: Vec<(&str, u64)> = self
            .stats
            .iter()
            .map(|(k, v)| (k.as_str(), v.total_bytes))
            .collect();
        v.sort_by(|a, b| b.1.cmp(&a.1));
        v.truncate(n);
        v
    }
}
/// A node in a flame graph tree.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct FlameNode {
    /// Function name.
    pub name: String,
    /// Number of samples at or below this node.
    pub count: u64,
    /// Children nodes.
    pub children: Vec<FlameNode>,
}
impl FlameNode {
    /// Create a new node.
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            count: 0,
            children: Vec::new(),
        }
    }
    /// Find or create a child with the given name.
    #[allow(dead_code)]
    pub fn get_or_create_child(&mut self, name: &str) -> &mut FlameNode {
        if let Some(pos) = self.children.iter().position(|c| c.name == name) {
            &mut self.children[pos]
        } else {
            self.children.push(FlameNode::new(name));
            self.children
                .last_mut()
                .expect("just pushed a child so last_mut must return Some")
        }
    }
    /// Total samples in the subtree rooted here.
    #[allow(dead_code)]
    pub fn total(&self) -> u64 {
        self.count + self.children.iter().map(|c| c.total()).sum::<u64>()
    }
    /// Format the flame node as an indented tree.
    #[allow(dead_code)]
    pub fn format(&self, depth: usize) -> String {
        let indent = "  ".repeat(depth);
        let mut out = format!("{}{} ({})\n", indent, self.name, self.count);
        for child in &self.children {
            out.push_str(&child.format(depth + 1));
        }
        out
    }
}
/// A node in a call tree (for inclusive/exclusive timing analysis).
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CallTreeNode {
    /// Function name.
    pub name: String,
    /// Total (inclusive) time in ns.
    pub inclusive_ns: u64,
    /// Self (exclusive) time in ns.
    pub exclusive_ns: u64,
    /// Number of calls.
    pub call_count: u64,
    /// Child nodes.
    pub children: Vec<CallTreeNode>,
}
impl CallTreeNode {
    /// Create a new call tree node.
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            inclusive_ns: 0,
            exclusive_ns: 0,
            call_count: 0,
            children: Vec::new(),
        }
    }
    /// Average self time per call.
    #[allow(dead_code)]
    pub fn avg_exclusive_ns(&self) -> f64 {
        if self.call_count == 0 {
            0.0
        } else {
            self.exclusive_ns as f64 / self.call_count as f64
        }
    }
    /// Average inclusive time per call.
    #[allow(dead_code)]
    pub fn avg_inclusive_ns(&self) -> f64 {
        if self.call_count == 0 {
            0.0
        } else {
            self.inclusive_ns as f64 / self.call_count as f64
        }
    }
    /// Find a child with the given name.
    #[allow(dead_code)]
    pub fn find_child(&self, name: &str) -> Option<&CallTreeNode> {
        self.children.iter().find(|c| c.name == name)
    }
}
/// Lightweight profiler that records events for later analysis.
pub struct Profiler {
    /// Whether profiling is currently active.
    pub enabled: bool,
    /// Recorded events as `(timestamp_ns, event)` pairs.
    pub events: Vec<(u64, ProfilingEvent)>,
    /// Stack of `(function_name, entry_timestamp_ns)` entries.
    pub call_stack: Vec<(String, u64)>,
}
impl Profiler {
    /// Create a new, disabled profiler.
    pub fn new() -> Self {
        Self {
            enabled: false,
            events: Vec::new(),
            call_stack: Vec::new(),
        }
    }
    /// Enable profiling.
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    /// Disable profiling.
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    /// Record an arbitrary profiling event (no-op when disabled).
    pub fn record(&mut self, event: ProfilingEvent) {
        if self.enabled {
            let ts = Self::now_ns();
            self.events.push((ts, event));
        }
    }
    /// Record a function entry and push it onto the call stack.
    pub fn enter_function(&mut self, name: &str) {
        if self.enabled {
            let ts = Self::now_ns();
            let depth = self.call_stack.len() as u32;
            self.call_stack.push((name.to_string(), ts));
            self.events.push((
                ts,
                ProfilingEvent::FunctionCall {
                    name: name.to_string(),
                    depth,
                },
            ));
        }
    }
    /// Record a function exit and pop it from the call stack.
    pub fn exit_function(&mut self, name: &str) {
        if self.enabled {
            let ts = Self::now_ns();
            let duration_ns =
                if let Some(idx) = self.call_stack.iter().rposition(|(n, _)| n == name) {
                    let entry_ts = self.call_stack[idx].1;
                    self.call_stack.remove(idx);
                    ts.saturating_sub(entry_ts)
                } else {
                    0
                };
            self.events.push((
                ts,
                ProfilingEvent::FunctionReturn {
                    name: name.to_string(),
                    duration_ns,
                },
            ));
        }
    }
    /// Record a memory allocation.
    pub fn alloc(&mut self, size: usize, tag: &str) {
        if self.enabled {
            let ts = Self::now_ns();
            self.events.push((
                ts,
                ProfilingEvent::Allocation {
                    size,
                    tag: tag.to_string(),
                },
            ));
        }
    }
    /// Record a memory deallocation.
    pub fn dealloc(&mut self, size: usize, tag: &str) {
        if self.enabled {
            let ts = Self::now_ns();
            self.events.push((
                ts,
                ProfilingEvent::Deallocation {
                    size,
                    tag: tag.to_string(),
                },
            ));
        }
    }
    /// Record a GC cycle.
    pub fn gc_cycle(&mut self, collected: usize, live: usize) {
        if self.enabled {
            let ts = Self::now_ns();
            self.events
                .push((ts, ProfilingEvent::GcCycle { collected, live }));
        }
    }
    /// Generate a report from the recorded events.
    pub fn generate_report(&self) -> ProfileReport {
        let mut total_calls: usize = 0;
        let mut total_alloc_bytes: usize = 0;
        let mut gc_cycles: usize = 0;
        let mut fn_durations: HashMap<String, u64> = HashMap::new();
        for (_, event) in &self.events {
            match event {
                ProfilingEvent::FunctionCall { .. } => {
                    total_calls += 1;
                }
                ProfilingEvent::FunctionReturn { name, duration_ns } => {
                    *fn_durations.entry(name.clone()).or_insert(0) += duration_ns;
                }
                ProfilingEvent::Allocation { size, .. } => {
                    total_alloc_bytes += size;
                }
                ProfilingEvent::GcCycle { .. } => {
                    gc_cycles += 1;
                }
                _ => {}
            }
        }
        let mut hot_functions: Vec<(String, u64)> = fn_durations.into_iter().collect();
        hot_functions.sort_by(|a, b| b.1.cmp(&a.1));
        hot_functions.truncate(10);
        ProfileReport {
            total_calls,
            total_alloc_bytes,
            hot_functions,
            gc_cycles,
        }
    }
    /// Generate a memory profile from the recorded events.
    pub fn memory_profile(&self) -> MemoryProfile {
        let mut current_bytes: usize = 0;
        let mut peak_bytes: usize = 0;
        let mut total_allocs: usize = 0;
        for (_, event) in &self.events {
            match event {
                ProfilingEvent::Allocation { size, .. } => {
                    current_bytes += size;
                    total_allocs += 1;
                    if current_bytes > peak_bytes {
                        peak_bytes = current_bytes;
                    }
                }
                ProfilingEvent::Deallocation { size, .. } => {
                    current_bytes = current_bytes.saturating_sub(*size);
                }
                _ => {}
            }
        }
        MemoryProfile {
            peak_bytes,
            current_bytes,
            total_allocs,
        }
    }
    fn now_ns() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0)
    }
}
/// A simple middleware layer that automatically profiles function calls.
#[allow(dead_code)]
pub struct ProfilingMiddleware {
    /// Inner profiler instance.
    pub profiler: Profiler,
    /// Whether this middleware is active.
    pub active: bool,
}
impl ProfilingMiddleware {
    /// Create a new active middleware.
    #[allow(dead_code)]
    pub fn new() -> Self {
        let mut profiler = Profiler::new();
        profiler.enable();
        Self {
            profiler,
            active: true,
        }
    }
    /// Invoke a closure with profiling.
    #[allow(dead_code)]
    pub fn instrument<F, T>(&mut self, name: &str, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        if self.active {
            self.profiler.enter_function(name);
        }
        let result = f();
        if self.active {
            self.profiler.exit_function(name);
        }
        result
    }
    /// Get a report.
    #[allow(dead_code)]
    pub fn report(&self) -> ProfileReport {
        self.profiler.generate_report()
    }
}
/// A timeline view built from profiling events.
#[allow(dead_code)]
pub struct TimelineView {
    /// Entries in the timeline.
    pub entries: Vec<TimelineEntry>,
}
impl TimelineView {
    /// Create an empty timeline.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    /// Build a timeline view from a profiler's events.
    #[allow(dead_code)]
    pub fn build(profiler: &Profiler) -> Self {
        let mut view = TimelineView::new();
        for (ts, event) in &profiler.events {
            let entry = match event {
                ProfilingEvent::FunctionCall { name, depth } => {
                    TimelineEntry::new(*ts, &format!("CALL {}[d={}]", name, depth), 0, "function")
                }
                ProfilingEvent::FunctionReturn { name, duration_ns } => TimelineEntry::new(
                    *ts,
                    &format!("RET {} ({}ns)", name, duration_ns),
                    *duration_ns,
                    "function",
                ),
                ProfilingEvent::Allocation { size, tag } => {
                    TimelineEntry::new(*ts, &format!("ALLOC {} ({} bytes)", tag, size), 0, "memory")
                }
                ProfilingEvent::Deallocation { size, tag } => {
                    TimelineEntry::new(*ts, &format!("FREE {} ({} bytes)", tag, size), 0, "memory")
                }
                ProfilingEvent::GcCycle { collected, live } => TimelineEntry::new(
                    *ts,
                    &format!("GC: collected={} live={}", collected, live),
                    0,
                    "gc",
                ),
                ProfilingEvent::TacticStep {
                    tactic_name,
                    goal_count,
                } => TimelineEntry::new(
                    *ts,
                    &format!("TACTIC {} goals={}", tactic_name, goal_count),
                    0,
                    "tactic",
                ),
            };
            view.entries.push(entry);
        }
        view
    }
    /// Filter entries by category.
    #[allow(dead_code)]
    pub fn by_category(&self, category: &str) -> Vec<&TimelineEntry> {
        self.entries
            .iter()
            .filter(|e| e.category == category)
            .collect()
    }
    /// Total duration covered by the timeline.
    #[allow(dead_code)]
    pub fn span_ns(&self) -> u64 {
        let min = self
            .entries
            .iter()
            .map(|e| e.timestamp_ns)
            .min()
            .unwrap_or(0);
        let max = self
            .entries
            .iter()
            .map(|e| e.timestamp_ns + e.duration_ns)
            .max()
            .unwrap_or(0);
        max.saturating_sub(min)
    }
}
/// An annotated timeline.
#[allow(dead_code)]
pub struct AnnotatedTimeline {
    /// Profiler events.
    pub entries: Vec<TimelineEntry>,
    /// Annotations overlaid on the timeline.
    pub annotations: Vec<TimelineAnnotation>,
}
impl AnnotatedTimeline {
    /// Create an empty annotated timeline.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            annotations: Vec::new(),
        }
    }
    /// Add an annotation.
    #[allow(dead_code)]
    pub fn annotate(&mut self, annotation: TimelineAnnotation) {
        self.annotations.push(annotation);
    }
    /// Annotations in the given time range.
    #[allow(dead_code)]
    pub fn annotations_in_range(&self, start_ns: u64, end_ns: u64) -> Vec<&TimelineAnnotation> {
        self.annotations
            .iter()
            .filter(|a| a.timestamp_ns >= start_ns && a.timestamp_ns <= end_ns)
            .collect()
    }
}
/// A filter for profiling events.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct EventFilter {
    /// Only include events involving these function names (empty = all).
    pub function_names: Vec<String>,
    /// Only include events with timestamp >= this value.
    pub min_timestamp_ns: u64,
    /// Only include events with timestamp <= this value.
    pub max_timestamp_ns: u64,
    /// Only include allocation events above this size.
    pub min_alloc_bytes: usize,
}
impl EventFilter {
    /// Create an unfiltered event filter (everything passes).
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            function_names: Vec::new(),
            min_timestamp_ns: 0,
            max_timestamp_ns: u64::MAX,
            min_alloc_bytes: 0,
        }
    }
    /// Return `true` if the event passes this filter.
    #[allow(dead_code)]
    pub fn matches(&self, ts: u64, event: &ProfilingEvent) -> bool {
        if ts < self.min_timestamp_ns || ts > self.max_timestamp_ns {
            return false;
        }
        if !self.function_names.is_empty() {
            let name = match event {
                ProfilingEvent::FunctionCall { name, .. } => Some(name.as_str()),
                ProfilingEvent::FunctionReturn { name, .. } => Some(name.as_str()),
                _ => None,
            };
            if let Some(n) = name {
                if !self.function_names.iter().any(|f| f == n) {
                    return false;
                }
            }
        }
        if let ProfilingEvent::Allocation { size, .. } = event {
            if *size < self.min_alloc_bytes {
                return false;
            }
        }
        true
    }
    /// Filter a list of `(ts, event)` pairs.
    #[allow(dead_code)]
    pub fn apply<'a>(&self, events: &'a [(u64, ProfilingEvent)]) -> Vec<&'a (u64, ProfilingEvent)> {
        events
            .iter()
            .filter(|(ts, ev)| self.matches(*ts, ev))
            .collect()
    }
}
/// Memory usage profile.
#[derive(Clone, Debug)]
pub struct MemoryProfile {
    /// Peak memory usage in bytes.
    pub peak_bytes: usize,
    /// Current (live) memory usage in bytes.
    pub current_bytes: usize,
    /// Total number of allocation events.
    pub total_allocs: usize,
}
impl MemoryProfile {
    /// Format as human-readable text.
    pub fn to_text(&self) -> String {
        format!(
            "=== Memory Profile ===\nPeak usage    : {} bytes\nCurrent usage : {} bytes\nTotal allocs  : {}\n",
            self.peak_bytes, self.current_bytes, self.total_allocs
        )
    }
}
/// A flame graph built from sampling profiler data.
#[allow(dead_code)]
pub struct FlameGraph {
    /// Root node (synthetic "all" node).
    pub root: FlameNode,
    /// Total sample count.
    pub total_samples: u64,
}
impl FlameGraph {
    /// Create an empty flame graph.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            root: FlameNode::new("(all)"),
            total_samples: 0,
        }
    }
    /// Add a call stack to the flame graph (stack is bottom-to-top order).
    #[allow(dead_code)]
    pub fn add_stack(&mut self, stack: &[String]) {
        self.total_samples += 1;
        self.root.count += 1;
        let mut node = &mut self.root;
        for frame in stack.iter().rev() {
            node = node.get_or_create_child(frame);
            node.count += 1;
        }
    }
    /// Build a flame graph from a sampling profiler.
    #[allow(dead_code)]
    pub fn from_profiler(profiler: &SamplingProfiler) -> Self {
        let mut fg = FlameGraph::new();
        for sample in &profiler.samples {
            fg.add_stack(&sample.call_stack);
        }
        fg
    }
    /// Render the flame graph as indented text.
    #[allow(dead_code)]
    pub fn render_text(&self) -> String {
        self.root.format(0)
    }
}
/// A single bucket in a histogram.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct HistogramBucket {
    /// Lower bound of the bucket.
    pub lower: f64,
    /// Upper bound of the bucket.
    pub upper: f64,
    /// Number of observations in this bucket.
    pub count: u64,
}
/// Manages the lifecycle of a profiling session.
#[allow(dead_code)]
pub struct ProfilingSession {
    /// Main event profiler.
    pub profiler: Profiler,
    /// Sampling profiler.
    pub sampler: SamplingProfiler,
    /// Allocation tracker.
    pub alloc_tracker: AllocationTracker,
    /// Tactic event log.
    pub tactic_log: TacticProfileLog,
    /// Session name.
    pub name: String,
    /// Whether the session is running.
    pub running: bool,
}
impl ProfilingSession {
    /// Create a new session with the given name.
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            profiler: Profiler::new(),
            sampler: SamplingProfiler::new(1_000_000),
            alloc_tracker: AllocationTracker::new(),
            tactic_log: TacticProfileLog::new(),
            name: name.to_string(),
            running: false,
        }
    }
    /// Start the session.
    #[allow(dead_code)]
    pub fn start(&mut self) {
        self.profiler.enable();
        self.sampler.enable();
        self.running = true;
    }
    /// Stop the session.
    #[allow(dead_code)]
    pub fn stop(&mut self) {
        self.profiler.disable();
        self.sampler.disable();
        self.running = false;
    }
    /// Record a function call (in both profilers).
    #[allow(dead_code)]
    pub fn enter_function(&mut self, name: &str) {
        self.profiler.enter_function(name);
        self.sampler.enter(name);
    }
    /// Record a function return.
    #[allow(dead_code)]
    pub fn exit_function(&mut self, name: &str) {
        self.profiler.exit_function(name);
        self.sampler.leave(name);
    }
    /// Record an allocation.
    #[allow(dead_code)]
    pub fn alloc(&mut self, bytes: usize, tag: &str) {
        self.profiler.alloc(bytes, tag);
        self.alloc_tracker.record_alloc(tag, bytes as u64);
    }
    /// Record a deallocation.
    #[allow(dead_code)]
    pub fn dealloc(&mut self, bytes: usize, tag: &str) {
        self.profiler.dealloc(bytes, tag);
        self.alloc_tracker.record_dealloc(tag, bytes as u64);
    }
    /// Generate a combined report.
    #[allow(dead_code)]
    pub fn combined_report(&self) -> String {
        let profile_report = self.profiler.generate_report();
        let mem_profile = self.profiler.memory_profile();
        format!(
            "=== ProfilingSession: {} ===\n{}\n{}\nTactic steps: {}\nSamples: {}\nLive bytes: {}",
            self.name,
            profile_report.to_text(),
            mem_profile.to_text(),
            self.tactic_log.success_count(),
            self.sampler.sample_count(),
            self.alloc_tracker.total_live_bytes(),
        )
    }
}
/// A sampling-based profiler.
#[allow(dead_code)]
pub struct SamplingProfiler {
    /// Collected samples.
    pub samples: Vec<ProfileSample>,
    /// Whether sampling is enabled.
    pub enabled: bool,
    /// Configured sampling interval in nanoseconds.
    pub interval_ns: u64,
    /// Current simulated call stack.
    pub current_stack: Vec<String>,
}
impl SamplingProfiler {
    /// Create a new sampling profiler with the given interval.
    #[allow(dead_code)]
    pub fn new(interval_ns: u64) -> Self {
        Self {
            samples: Vec::new(),
            enabled: false,
            interval_ns,
            current_stack: Vec::new(),
        }
    }
    /// Enable the profiler.
    #[allow(dead_code)]
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    /// Disable the profiler.
    #[allow(dead_code)]
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    /// Simulate entering a function.
    #[allow(dead_code)]
    pub fn enter(&mut self, function: &str) {
        if self.enabled {
            self.current_stack.insert(0, function.to_string());
        }
    }
    /// Simulate leaving a function.
    #[allow(dead_code)]
    pub fn leave(&mut self, function: &str) {
        if self.enabled {
            if let Some(pos) = self.current_stack.iter().position(|s| s == function) {
                self.current_stack.remove(pos);
            }
        }
    }
    /// Take a sample of the current call stack.
    #[allow(dead_code)]
    pub fn take_sample(&mut self, thread_id: u64) {
        if self.enabled {
            let ts = profiler_now_ns();
            self.samples.push(ProfileSample::new(
                ts,
                self.current_stack.clone(),
                thread_id,
            ));
        }
    }
    /// Compute the flat profile: (function_name, hit_count) sorted by count.
    #[allow(dead_code)]
    pub fn flat_profile(&self) -> Vec<(String, usize)> {
        let mut counts: HashMap<String, usize> = HashMap::new();
        for sample in &self.samples {
            if let Some(top) = sample.top_function() {
                *counts.entry(top.to_string()).or_insert(0) += 1;
            }
        }
        let mut result: Vec<(String, usize)> = counts.into_iter().collect();
        result.sort_by(|a, b| b.1.cmp(&a.1));
        result
    }
    /// Compute the cumulative profile: each function gets credit for every sample
    /// it appears in (at any depth).
    #[allow(dead_code)]
    pub fn cumulative_profile(&self) -> Vec<(String, usize)> {
        let mut counts: HashMap<String, usize> = HashMap::new();
        for sample in &self.samples {
            for func in &sample.call_stack {
                *counts.entry(func.clone()).or_insert(0) += 1;
            }
        }
        let mut result: Vec<(String, usize)> = counts.into_iter().collect();
        result.sort_by(|a, b| b.1.cmp(&a.1));
        result
    }
    /// Total number of samples collected.
    #[allow(dead_code)]
    pub fn sample_count(&self) -> usize {
        self.samples.len()
    }
    /// Average call stack depth across all samples.
    #[allow(dead_code)]
    pub fn avg_stack_depth(&self) -> f64 {
        if self.samples.is_empty() {
            return 0.0;
        }
        let total: usize = self.samples.iter().map(|s| s.depth()).sum();
        total as f64 / self.samples.len() as f64
    }
}
/// Statistics for allocations associated with a single tag.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct AllocationStat {
    /// Total bytes allocated with this tag.
    pub total_bytes: u64,
    /// Number of allocation events.
    pub alloc_count: u64,
    /// Number of deallocation events.
    pub dealloc_count: u64,
    /// Bytes currently live.
    pub live_bytes: u64,
}
/// A snapshot of the call stack at a specific point in time.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct StackSnapshot {
    /// Timestamp when the snapshot was taken.
    pub timestamp_ns: u64,
    /// The call stack frames (most recent first).
    pub frames: Vec<String>,
    /// An optional label for this snapshot.
    pub label: Option<String>,
}
impl StackSnapshot {
    /// Create a new snapshot.
    #[allow(dead_code)]
    pub fn new(timestamp_ns: u64, frames: Vec<String>) -> Self {
        Self {
            timestamp_ns,
            frames,
            label: None,
        }
    }
    /// Attach a label.
    #[allow(dead_code)]
    pub fn with_label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }
    /// Depth of the captured stack.
    #[allow(dead_code)]
    pub fn depth(&self) -> usize {
        self.frames.len()
    }
    /// Format as a string.
    #[allow(dead_code)]
    pub fn format(&self) -> String {
        let label_str = self.label.as_deref().unwrap_or("(no label)");
        let mut out = format!("Stack at {} ns [{}]:\n", self.timestamp_ns, label_str);
        for (i, frame) in self.frames.iter().enumerate() {
            out.push_str(&format!("  {:3}: {}\n", i, frame));
        }
        out
    }
}
