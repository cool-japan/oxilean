//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::compute_graph_diameter;
use std::collections::HashMap;

/// A single entry in the build timeline.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TimelineEvent {
    /// Name (usually source file or target).
    pub name: String,
    /// Start timestamp (ms).
    pub start_ms: u64,
    /// End timestamp (ms, 0 if still running).
    pub end_ms: u64,
    /// State of this event.
    pub state: TimelineEventState,
    /// Worker/thread executing this event.
    pub worker_id: u32,
}
#[allow(dead_code)]
impl TimelineEvent {
    /// Create a running event.
    pub fn start(name: &str, start_ms: u64, worker_id: u32) -> Self {
        Self {
            name: name.to_string(),
            start_ms,
            end_ms: 0,
            state: TimelineEventState::Running,
            worker_id,
        }
    }
    /// Mark the event as done.
    pub fn finish(&mut self, end_ms: u64) {
        self.end_ms = end_ms;
        self.state = TimelineEventState::Done;
    }
    /// Mark the event as failed.
    pub fn fail(&mut self, end_ms: u64) {
        self.end_ms = end_ms;
        self.state = TimelineEventState::Failed;
    }
    /// Mark the event as skipped (cached).
    pub fn skip(&mut self) {
        self.state = TimelineEventState::Skipped;
    }
    /// Duration in ms (0 if not yet complete).
    pub fn duration_ms(&self) -> u64 {
        if self.end_ms >= self.start_ms {
            self.end_ms - self.start_ms
        } else {
            0
        }
    }
}
/// Supported analytics export formats.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    /// Comma-separated values.
    Csv,
    /// JSON (hand-written).
    Json,
    /// Plain text human-readable.
    Text,
    /// Markdown table.
    Markdown,
}
/// A profiling span that records a named interval.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ProfilingSpan {
    /// Human-readable name of this span.
    pub name: String,
    /// Start timestamp (monotonic counter or ms).
    pub start_ts: u64,
    /// End timestamp (0 if still open).
    pub end_ts: u64,
    /// Optional parent span index.
    pub parent_idx: Option<usize>,
    /// Metadata key-value pairs.
    pub metadata: HashMap<String, String>,
}
#[allow(dead_code)]
impl ProfilingSpan {
    /// Create a new open span.
    pub fn open(name: &str, start_ts: u64) -> Self {
        ProfilingSpan {
            name: name.to_string(),
            start_ts,
            end_ts: 0,
            parent_idx: None,
            metadata: HashMap::new(),
        }
    }
    /// Close this span with an end timestamp.
    pub fn close(&mut self, end_ts: u64) {
        self.end_ts = end_ts;
    }
    /// Duration in timestamp units (0 if span is still open).
    pub fn duration(&self) -> u64 {
        if self.end_ts >= self.start_ts {
            self.end_ts - self.start_ts
        } else {
            0
        }
    }
    /// Whether this span is still open.
    pub fn is_open(&self) -> bool {
        self.end_ts == 0
    }
    /// Attach a metadata entry.
    pub fn with_meta(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }
    /// Set the parent span index.
    pub fn with_parent(mut self, parent: usize) -> Self {
        self.parent_idx = Some(parent);
        self
    }
}
/// Tracks build statistics across the entire build
pub struct BuildAnalytics {
    pub(crate) events: Vec<(u64, BuildEvent)>,
    start_time: u64,
    pub(crate) total_files: u32,
    pub(crate) cached_files: u32,
}
impl BuildAnalytics {
    pub fn new() -> Self {
        BuildAnalytics {
            events: Vec::new(),
            start_time: 0,
            total_files: 0,
            cached_files: 0,
        }
    }
    pub fn record(&mut self, event: BuildEvent) {
        let ts = self.events.len() as u64;
        match &event {
            BuildEvent::FileEnd { .. } => self.total_files += 1,
            BuildEvent::CacheHit(_) => self.cached_files += 1,
            _ => {}
        }
        self.events.push((ts, event));
    }
    pub fn total_duration_ms(&self) -> u64 {
        let mut total = 0u64;
        for (_, ev) in &self.events {
            if let BuildEvent::FileEnd { duration_ms, .. } = ev {
                total += duration_ms;
            }
        }
        total
    }
    pub fn cache_hit_rate(&self) -> f64 {
        let hits = self.cached_files as f64;
        let total = (self.total_files + self.cached_files) as f64;
        if total == 0.0 {
            0.0
        } else {
            hits / total
        }
    }
    pub fn files_per_second(&self) -> f64 {
        let duration_s = self.total_duration_ms() as f64 / 1000.0;
        if duration_s == 0.0 {
            self.total_files as f64
        } else {
            self.total_files as f64 / duration_s
        }
    }
    /// Return the n slowest files by duration_ms
    pub fn slowest_files(&self, n: usize) -> Vec<(String, u64)> {
        let mut file_durations: HashMap<String, u64> = HashMap::new();
        for (_, ev) in &self.events {
            if let BuildEvent::FileEnd {
                path, duration_ms, ..
            } = ev
            {
                let entry = file_durations.entry(path.clone()).or_insert(0);
                *entry += duration_ms;
            }
        }
        let mut sorted: Vec<(String, u64)> = file_durations.into_iter().collect();
        sorted.sort_by_key(|b| std::cmp::Reverse(b.1));
        sorted.truncate(n);
        sorted
    }
    pub fn total_declarations(&self) -> u32 {
        let mut total = 0u32;
        for (_, ev) in &self.events {
            if let BuildEvent::FileEnd { declarations, .. } = ev {
                total += declarations;
            }
        }
        total
    }
    pub fn error_count(&self) -> u32 {
        self.events
            .iter()
            .filter(|(_, ev)| matches!(ev, BuildEvent::Error { .. }))
            .count() as u32
    }
    pub fn generate_report(&self) -> BuildReport {
        BuildReport {
            total_files: self.total_files,
            cached_files: self.cached_files,
            total_declarations: self.total_declarations(),
            total_duration_ms: self.total_duration_ms(),
            cache_hit_rate: self.cache_hit_rate(),
            errors: self.error_count(),
            slowest_files: self.slowest_files(5),
        }
    }
}
/// Aggregated stats for a single build worker.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct WorkerStats {
    /// Worker ID.
    pub worker_id: u32,
    /// Total compile time attributed to this worker (ms).
    pub total_ms: u64,
    /// Number of compilation units handled.
    pub unit_count: u64,
    /// Number of cache hits on this worker.
    pub cache_hits: u64,
}
#[allow(dead_code)]
impl WorkerStats {
    /// Create a new stats record for a worker.
    pub fn new(worker_id: u32) -> Self {
        Self {
            worker_id,
            ..Self::default()
        }
    }
    /// Average time per unit handled.
    pub fn avg_ms_per_unit(&self) -> f64 {
        if self.unit_count == 0 {
            0.0
        } else {
            self.total_ms as f64 / self.unit_count as f64
        }
    }
    /// Cache hit rate for this worker.
    pub fn cache_hit_rate(&self) -> f64 {
        let total = self.unit_count + self.cache_hits;
        if total == 0 {
            0.0
        } else {
            self.cache_hits as f64 / total as f64
        }
    }
}
/// Collects build timeline events for post-build analysis.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct BuildTimeline {
    events: Vec<TimelineEvent>,
}
#[allow(dead_code)]
impl BuildTimeline {
    /// Create an empty timeline.
    pub fn new() -> Self {
        Self::default()
    }
    /// Push an event onto the timeline.
    pub fn push(&mut self, event: TimelineEvent) {
        self.events.push(event);
    }
    /// Find an event by name and return a mutable reference.
    pub fn find_mut(&mut self, name: &str) -> Option<&mut TimelineEvent> {
        self.events.iter_mut().find(|e| e.name == name)
    }
    /// All events in insertion order.
    pub fn events(&self) -> &[TimelineEvent] {
        &self.events
    }
    /// Events filtered by state.
    pub fn events_in_state(&self, state: &TimelineEventState) -> Vec<&TimelineEvent> {
        self.events.iter().filter(|e| &e.state == state).collect()
    }
    /// Number of events in a particular state.
    pub fn count_in_state(&self, state: &TimelineEventState) -> usize {
        self.events_in_state(state).len()
    }
    /// Compute the overall wall-clock span of all events.
    pub fn total_span_ms(&self) -> u64 {
        let min_start = self.events.iter().map(|e| e.start_ms).min().unwrap_or(0);
        let max_end = self
            .events
            .iter()
            .map(|e| e.end_ms)
            .filter(|&e| e > 0)
            .max()
            .unwrap_or(0);
        max_end.saturating_sub(min_start)
    }
    /// Export a simple Gantt-style text chart (one row per worker).
    pub fn to_gantt_text(&self, scale_ms_per_char: u64) -> String {
        if self.events.is_empty() {
            return String::from("(empty timeline)\n");
        }
        let min_ts = self.events.iter().map(|e| e.start_ms).min().unwrap_or(0);
        let max_ts = self
            .events
            .iter()
            .map(|e| e.end_ms)
            .filter(|&e| e > 0)
            .max()
            .unwrap_or(1);
        let mut workers: Vec<u32> = self.events.iter().map(|e| e.worker_id).collect();
        workers.sort_unstable();
        workers.dedup();
        let mut out = String::new();
        let scale = scale_ms_per_char.max(1);
        for worker in &workers {
            out.push_str(&format!("W{}: ", worker));
            let evs: Vec<&TimelineEvent> = self
                .events
                .iter()
                .filter(|e| &e.worker_id == worker && e.end_ms > 0)
                .collect();
            let total_chars = ((max_ts - min_ts) / scale + 1) as usize;
            let mut line = vec![b' '; total_chars];
            for ev in evs {
                let start_c = ((ev.start_ms - min_ts) / scale) as usize;
                let end_c = (((ev.end_ms - min_ts) / scale) as usize).min(total_chars - 1);
                let ch: u8 = match ev.state {
                    TimelineEventState::Done => b'#',
                    TimelineEventState::Failed => b'!',
                    TimelineEventState::Skipped => b'-',
                    _ => b'?',
                };
                for c in line.iter_mut().take(end_c + 1).skip(start_c) {
                    *c = ch;
                }
            }
            out.push_str(&String::from_utf8_lossy(&line));
            out.push('\n');
        }
        out
    }
    /// Clear all events.
    pub fn clear(&mut self) {
        self.events.clear();
    }
}
/// Estimates time savings from incremental compilation (cache hits).
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct IncrementalSavingsEstimator {
    /// Average time per file in ms (used as baseline for cache hits).
    baseline_ms_per_file: f64,
    /// Total cache hits observed.
    cache_hits: u64,
    /// Total files actually compiled.
    files_compiled: u64,
    /// Sum of actual compilation durations.
    actual_compile_ms: u64,
}
#[allow(dead_code)]
impl IncrementalSavingsEstimator {
    /// Create a new estimator with a baseline per-file compile time.
    pub fn new(baseline_ms_per_file: f64) -> Self {
        Self {
            baseline_ms_per_file,
            ..Self::default()
        }
    }
    /// Record a cache hit (skipped compilation).
    pub fn record_cache_hit(&mut self) {
        self.cache_hits += 1;
    }
    /// Record an actual compilation with its duration.
    pub fn record_compilation(&mut self, duration_ms: u64) {
        self.files_compiled += 1;
        self.actual_compile_ms += duration_ms;
    }
    /// Estimated time saved (hits * baseline).
    pub fn estimated_savings_ms(&self) -> f64 {
        self.cache_hits as f64 * self.baseline_ms_per_file
    }
    /// Actual time spent compiling.
    pub fn actual_compile_ms(&self) -> u64 {
        self.actual_compile_ms
    }
    /// Total files that would have been compiled without incrementality.
    pub fn total_files_hypothetical(&self) -> u64 {
        self.cache_hits + self.files_compiled
    }
    /// Fraction of work avoided by the cache.
    pub fn cache_efficiency(&self) -> f64 {
        let total = self.total_files_hypothetical();
        if total == 0 {
            0.0
        } else {
            self.cache_hits as f64 / total as f64
        }
    }
    /// Speedup factor (hypothetical time / actual time).
    pub fn speedup_factor(&self) -> f64 {
        let hypothetical = self.actual_compile_ms as f64 + self.estimated_savings_ms();
        if self.actual_compile_ms == 0 {
            1.0
        } else {
            hypothetical / self.actual_compile_ms as f64
        }
    }
}
/// A node in the critical path computation.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CriticalPathNode {
    /// Module/file name.
    pub name: String,
    /// Compile duration of this node.
    pub duration_ms: u64,
    /// Earliest start time.
    pub earliest_start: u64,
    /// Earliest finish time.
    pub earliest_finish: u64,
    /// Latest start time (for critical path).
    pub latest_start: u64,
    /// Latest finish time.
    pub latest_finish: u64,
    /// Float (latest_start - earliest_start). 0 = on critical path.
    pub float: u64,
}
#[allow(dead_code)]
impl CriticalPathNode {
    /// Whether this node is on the critical path (zero float).
    pub fn is_critical(&self) -> bool {
        self.float == 0
    }
}
/// The state of a timeline event.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimelineEventState {
    /// Waiting for dependencies.
    Pending,
    /// Currently executing.
    Running,
    /// Successfully completed.
    Done,
    /// Failed.
    Failed,
    /// Skipped (e.g. cached).
    Skipped,
}
/// A summary report from a build
#[derive(Debug)]
pub struct BuildReport {
    pub total_files: u32,
    pub cached_files: u32,
    pub total_declarations: u32,
    pub total_duration_ms: u64,
    pub cache_hit_rate: f64,
    pub errors: u32,
    pub slowest_files: Vec<(String, u64)>,
}
impl BuildReport {
    pub fn to_text(&self) -> String {
        let mut s = String::new();
        s.push_str(
            "Build Report
",
        );
        s.push_str(&format!("  Total files:       {}\n", self.total_files));
        s.push_str(&format!("  Cached files:      {}\n", self.cached_files));
        s.push_str(&format!(
            "  Total declarations:{}\n",
            self.total_declarations
        ));
        s.push_str(&format!(
            "  Total duration:    {} ms\n",
            self.total_duration_ms
        ));
        s.push_str(&format!(
            "  Cache hit rate:    {:.1}%\n",
            self.cache_hit_rate * 100.0
        ));
        s.push_str(&format!("  Errors:            {}\n", self.errors));
        if !self.slowest_files.is_empty() {
            s.push_str("  Slowest files:\n");
            for (path, ms) in &self.slowest_files {
                s.push_str(&format!("    {} ({} ms)\n", path, ms));
            }
        }
        s
    }
    /// Simple hand-written JSON serialization
    pub fn to_json(&self) -> String {
        let slowest_json: Vec<String> = self
            .slowest_files
            .iter()
            .map(|(p, ms)| format!("{{\"path\":\"{}\",\"duration_ms\":{}}}", p, ms))
            .collect();
        format!(
            concat!(
                "{{\n",
                "  \"total_files\": {},\n",
                "  \"cached_files\": {},\n",
                "  \"total_declarations\": {},\n",
                "  \"total_duration_ms\": {},\n",
                "  \"cache_hit_rate\": {:.4},\n",
                "  \"errors\": {},\n",
                "  \"slowest_files\": [{}]\n",
                "}}"
            ),
            self.total_files,
            self.cached_files,
            self.total_declarations,
            self.total_duration_ms,
            self.cache_hit_rate,
            self.errors,
            slowest_json.join(", ")
        )
    }
}
/// A profiler that collects hierarchical spans during a build.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct BuildProfiler {
    spans: Vec<ProfilingSpan>,
    next_ts: u64,
}
#[allow(dead_code)]
impl BuildProfiler {
    /// Create a new empty profiler.
    pub fn new() -> Self {
        Self::default()
    }
    /// Open a new top-level span and return its index.
    pub fn begin(&mut self, name: &str) -> usize {
        let ts = self.next_ts;
        self.next_ts += 1;
        self.spans.push(ProfilingSpan::open(name, ts));
        self.spans.len() - 1
    }
    /// Open a child span under the given parent and return its index.
    pub fn begin_child(&mut self, name: &str, parent: usize) -> usize {
        let ts = self.next_ts;
        self.next_ts += 1;
        let mut span = ProfilingSpan::open(name, ts);
        span.parent_idx = Some(parent);
        self.spans.push(span);
        self.spans.len() - 1
    }
    /// Close the span at `idx`.
    pub fn end(&mut self, idx: usize) {
        let ts = self.next_ts;
        self.next_ts += 1;
        if let Some(span) = self.spans.get_mut(idx) {
            span.close(ts);
        }
    }
    /// Total number of spans recorded.
    pub fn span_count(&self) -> usize {
        self.spans.len()
    }
    /// Return all completed (closed) spans.
    pub fn completed_spans(&self) -> Vec<&ProfilingSpan> {
        self.spans.iter().filter(|s| !s.is_open()).collect()
    }
    /// Return the span with the longest duration.
    pub fn slowest_span(&self) -> Option<&ProfilingSpan> {
        self.spans
            .iter()
            .filter(|s| !s.is_open())
            .max_by_key(|s| s.duration())
    }
    /// Compute total duration of all top-level spans.
    pub fn total_wall_time(&self) -> u64 {
        self.spans
            .iter()
            .filter(|s| s.parent_idx.is_none() && !s.is_open())
            .map(|s| s.duration())
            .sum()
    }
    /// Return spans grouped by name.
    pub fn spans_by_name(&self) -> HashMap<String, Vec<&ProfilingSpan>> {
        let mut map: HashMap<String, Vec<&ProfilingSpan>> = HashMap::new();
        for span in &self.spans {
            map.entry(span.name.clone()).or_default().push(span);
        }
        map
    }
    /// Average duration for spans with a given name.
    pub fn average_duration_for(&self, name: &str) -> Option<f64> {
        let matching: Vec<u64> = self
            .spans
            .iter()
            .filter(|s| s.name == name && !s.is_open())
            .map(|s| s.duration())
            .collect();
        if matching.is_empty() {
            None
        } else {
            let sum: u64 = matching.iter().sum();
            Some(sum as f64 / matching.len() as f64)
        }
    }
    /// Export all spans as a simple CSV string.
    pub fn to_csv(&self) -> String {
        let mut out = String::from("name,start_ts,end_ts,duration,parent\n");
        for s in &self.spans {
            let parent = s
                .parent_idx
                .map(|i| i.to_string())
                .unwrap_or_else(|| "-".to_string());
            out.push_str(&format!(
                "{},{},{},{},{}\n",
                s.name,
                s.start_ts,
                s.end_ts,
                s.duration(),
                parent
            ));
        }
        out
    }
}
/// Accumulates samples and computes rolling statistics.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct MetricsAggregator {
    /// Named series of samples.
    series: HashMap<String, Vec<MetricSample>>,
}
#[allow(dead_code)]
impl MetricsAggregator {
    /// Create a new aggregator.
    pub fn new() -> Self {
        Self::default()
    }
    /// Push a sample into the named series.
    pub fn push(&mut self, series: &str, ts: u64, value: f64) {
        self.series
            .entry(series.to_string())
            .or_default()
            .push(MetricSample::new(ts, value));
    }
    /// Compute the mean of all samples in `series`.
    pub fn mean(&self, series: &str) -> Option<f64> {
        let v = self.series.get(series)?;
        if v.is_empty() {
            return None;
        }
        let sum: f64 = v.iter().map(|s| s.value).sum();
        Some(sum / v.len() as f64)
    }
    /// Compute the min of all samples in `series`.
    pub fn min(&self, series: &str) -> Option<f64> {
        self.series
            .get(series)?
            .iter()
            .map(|s| s.value)
            .reduce(f64::min)
    }
    /// Compute the max of all samples in `series`.
    pub fn max(&self, series: &str) -> Option<f64> {
        self.series
            .get(series)?
            .iter()
            .map(|s| s.value)
            .reduce(f64::max)
    }
    /// Compute the standard deviation of all samples in `series`.
    pub fn std_dev(&self, series: &str) -> Option<f64> {
        let v = self.series.get(series)?;
        if v.len() < 2 {
            return None;
        }
        let mean = self.mean(series)?;
        let variance = v.iter().map(|s| (s.value - mean).powi(2)).sum::<f64>() / v.len() as f64;
        Some(variance.sqrt())
    }
    /// Return the p95 percentile value for `series`.
    pub fn percentile_95(&self, series: &str) -> Option<f64> {
        let v = self.series.get(series)?;
        if v.is_empty() {
            return None;
        }
        let mut vals: Vec<f64> = v.iter().map(|s| s.value).collect();
        vals.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let idx = ((vals.len() as f64 * 0.95) as usize).min(vals.len() - 1);
        Some(vals[idx])
    }
    /// Return samples in the time window `[from_ts, to_ts]`.
    pub fn window(&self, series: &str, from_ts: u64, to_ts: u64) -> Vec<MetricSample> {
        self.series
            .get(series)
            .map(|v| {
                v.iter()
                    .filter(|s| s.ts >= from_ts && s.ts <= to_ts)
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }
    /// Number of samples in `series`.
    pub fn count(&self, series: &str) -> usize {
        self.series.get(series).map(|v| v.len()).unwrap_or(0)
    }
    /// All known series names.
    pub fn series_names(&self) -> Vec<&str> {
        self.series.keys().map(|k| k.as_str()).collect()
    }
    /// Clear a specific series.
    pub fn clear_series(&mut self, series: &str) {
        self.series.remove(series);
    }
    /// Clear all series.
    pub fn clear_all(&mut self) {
        self.series.clear();
    }
}
/// A build event for timing purposes
#[derive(Debug, Clone)]
pub enum BuildEvent {
    FileStart {
        path: String,
        size_bytes: u64,
    },
    FileEnd {
        path: String,
        duration_ms: u64,
        declarations: u32,
    },
    ParseStart(String),
    ParseEnd {
        path: String,
        duration_ms: u64,
    },
    ElabStart(String),
    ElabEnd {
        path: String,
        duration_ms: u64,
    },
    CacheHit(String),
    CacheMiss(String),
    Error {
        path: String,
        message: String,
    },
}
/// Aggregates statistics across multiple watch-mode rebuild cycles.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct WatchModeAnalytics {
    cycles: Vec<WatchCycleStat>,
}
#[allow(dead_code)]
impl WatchModeAnalytics {
    /// Create a new aggregator.
    pub fn new() -> Self {
        Self::default()
    }
    /// Push a completed cycle.
    pub fn push_cycle(&mut self, stat: WatchCycleStat) {
        self.cycles.push(stat);
    }
    /// Total number of cycles.
    pub fn cycle_count(&self) -> usize {
        self.cycles.len()
    }
    /// Average rebuild duration across all cycles.
    pub fn avg_rebuild_ms(&self) -> f64 {
        if self.cycles.is_empty() {
            return 0.0;
        }
        let sum: u64 = self.cycles.iter().map(|c| c.duration_ms).sum();
        sum as f64 / self.cycles.len() as f64
    }
    /// Fastest rebuild cycle.
    pub fn fastest_cycle(&self) -> Option<&WatchCycleStat> {
        self.cycles.iter().min_by_key(|c| c.duration_ms)
    }
    /// Slowest rebuild cycle.
    pub fn slowest_cycle(&self) -> Option<&WatchCycleStat> {
        self.cycles.iter().max_by_key(|c| c.duration_ms)
    }
    /// Success rate across all cycles.
    pub fn success_rate(&self) -> f64 {
        if self.cycles.is_empty() {
            return 0.0;
        }
        let successes = self.cycles.iter().filter(|c| c.success).count();
        successes as f64 / self.cycles.len() as f64
    }
    /// Average number of files recompiled per cycle.
    pub fn avg_files_recompiled(&self) -> f64 {
        if self.cycles.is_empty() {
            return 0.0;
        }
        let sum: usize = self.cycles.iter().map(|c| c.files_recompiled).sum();
        sum as f64 / self.cycles.len() as f64
    }
}
/// A simple tracker for build-time memory usage.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct MemoryUsageTracker {
    current_bytes: u64,
    peak_bytes: u64,
    allocation_count: u64,
    deallocation_count: u64,
}
#[allow(dead_code)]
impl MemoryUsageTracker {
    /// Create a new tracker.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record an allocation of `size` bytes.
    pub fn allocate(&mut self, size: u64) {
        self.current_bytes += size;
        if self.current_bytes > self.peak_bytes {
            self.peak_bytes = self.current_bytes;
        }
        self.allocation_count += 1;
    }
    /// Record a deallocation of `size` bytes.
    pub fn deallocate(&mut self, size: u64) {
        self.current_bytes = self.current_bytes.saturating_sub(size);
        self.deallocation_count += 1;
    }
    /// Current allocated bytes.
    pub fn current_bytes(&self) -> u64 {
        self.current_bytes
    }
    /// Peak allocated bytes observed.
    pub fn peak_bytes(&self) -> u64 {
        self.peak_bytes
    }
    /// Total number of allocation calls.
    pub fn allocation_count(&self) -> u64 {
        self.allocation_count
    }
    /// Total number of deallocation calls.
    pub fn deallocation_count(&self) -> u64 {
        self.deallocation_count
    }
    /// Net allocations still live (allocs - deallocs).
    pub fn live_allocations(&self) -> u64 {
        self.allocation_count
            .saturating_sub(self.deallocation_count)
    }
    /// Reset all counters.
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
/// Tracks build sessions over time to identify trends.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct BuildTrendAnalyzer {
    sessions: Vec<BuildSession>,
}
#[allow(dead_code)]
impl BuildTrendAnalyzer {
    /// Create a new trend analyzer.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a session.
    pub fn push_session(&mut self, session: BuildSession) {
        self.sessions.push(session);
        self.sessions.sort_by_key(|s| s.timestamp_secs);
    }
    /// Number of sessions recorded.
    pub fn session_count(&self) -> usize {
        self.sessions.len()
    }
    /// Average duration over all sessions.
    pub fn avg_duration_ms(&self) -> f64 {
        if self.sessions.is_empty() {
            return 0.0;
        }
        let sum: u64 = self.sessions.iter().map(|s| s.duration_ms).sum();
        sum as f64 / self.sessions.len() as f64
    }
    /// Trend direction for duration: positive = getting slower, negative = getting faster.
    /// Uses a simple linear regression slope over session index.
    pub fn duration_trend_slope(&self) -> f64 {
        let n = self.sessions.len();
        if n < 2 {
            return 0.0;
        }
        let n_f = n as f64;
        let x_mean = (n_f - 1.0) / 2.0;
        let y_mean = self.avg_duration_ms();
        let mut num = 0.0_f64;
        let mut den = 0.0_f64;
        for (i, s) in self.sessions.iter().enumerate() {
            let xi = i as f64 - x_mean;
            let yi = s.duration_ms as f64 - y_mean;
            num += xi * yi;
            den += xi * xi;
        }
        if den.abs() < f64::EPSILON {
            0.0
        } else {
            num / den
        }
    }
    /// Whether the build is regressing (duration trend slope > threshold_ms).
    pub fn is_regressing(&self, threshold_ms: f64) -> bool {
        self.duration_trend_slope() > threshold_ms
    }
    /// Most recent session.
    pub fn latest_session(&self) -> Option<&BuildSession> {
        self.sessions.last()
    }
    /// Sessions with at least one error.
    pub fn failing_sessions(&self) -> Vec<&BuildSession> {
        self.sessions.iter().filter(|s| s.errors > 0).collect()
    }
    /// Average cache hit rate over all sessions.
    pub fn avg_cache_hit_rate(&self) -> f64 {
        if self.sessions.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.sessions.iter().map(|s| s.cache_hit_rate).sum();
        sum / self.sessions.len() as f64
    }
}
/// Analytics specifically about the module dependency graph structure.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct GraphBuildAnalytics {
    /// Number of nodes in the graph.
    pub node_count: usize,
    /// Number of directed edges.
    pub edge_count: usize,
    /// Diameter of the graph (longest shortest path).
    pub diameter: usize,
    /// Average out-degree.
    pub avg_out_degree: f64,
    /// Fraction of nodes that are leaves (no outgoing edges).
    pub leaf_fraction: f64,
    /// Number of isolated nodes (no in or out edges).
    pub isolated_count: usize,
}
#[allow(dead_code)]
impl GraphBuildAnalytics {
    /// Compute analytics from a file-to-imports adjacency map.
    pub fn compute(deps: &HashMap<String, Vec<String>>) -> Self {
        let node_count = deps.len();
        let edge_count: usize = deps.values().map(|v| v.len()).sum();
        let avg_out_degree = if node_count == 0 {
            0.0
        } else {
            edge_count as f64 / node_count as f64
        };
        let leaves = deps.values().filter(|v| v.is_empty()).count();
        let leaf_fraction = if node_count == 0 {
            0.0
        } else {
            leaves as f64 / node_count as f64
        };
        let mut has_in_edge: std::collections::HashSet<&str> = std::collections::HashSet::new();
        for imports in deps.values() {
            for dep in imports {
                has_in_edge.insert(dep.as_str());
            }
        }
        let isolated_count = deps
            .iter()
            .filter(|(k, v)| v.is_empty() && !has_in_edge.contains(k.as_str()))
            .count();
        let diameter = compute_graph_diameter(deps);
        Self {
            node_count,
            edge_count,
            diameter,
            avg_out_degree,
            leaf_fraction,
            isolated_count,
        }
    }
}
/// Metrics about a single source file's complexity.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct FileComplexityMetrics {
    /// Path to the file.
    pub path: String,
    /// Total lines.
    pub line_count: u32,
    /// Number of top-level declarations.
    pub declaration_count: u32,
    /// Number of import statements.
    pub import_count: u32,
    /// Estimated cyclomatic complexity (from tactic branches, match arms, etc.).
    pub cyclomatic_complexity: u32,
    /// Comment line count.
    pub comment_lines: u32,
}
#[allow(dead_code)]
impl FileComplexityMetrics {
    /// Create a new metrics record.
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
            ..Self::default()
        }
    }
    /// Compute the documentation density (comment lines / total lines).
    pub fn doc_density(&self) -> f64 {
        if self.line_count == 0 {
            0.0
        } else {
            self.comment_lines as f64 / self.line_count as f64
        }
    }
    /// Compute the declarations per 100 lines ratio.
    pub fn declaration_density(&self) -> f64 {
        if self.line_count == 0 {
            0.0
        } else {
            self.declaration_count as f64 * 100.0 / self.line_count as f64
        }
    }
    /// Whether this file is considered complex (heuristic).
    pub fn is_complex(&self) -> bool {
        self.cyclomatic_complexity > 10 || self.line_count > 500
    }
}
/// Additional analytics query helpers for per-phase breakdown.
#[allow(dead_code)]
pub struct PhaseBreakdown {
    /// Total parse time across all files (ms).
    pub parse_ms: u64,
    /// Total elaboration time across all files (ms).
    pub elab_ms: u64,
    /// Other event durations (ms).
    pub other_ms: u64,
}
#[allow(dead_code)]
impl PhaseBreakdown {
    /// Compute a phase breakdown from an analytics object.
    pub fn from_analytics(analytics: &BuildAnalytics) -> Self {
        let mut parse_ms = 0u64;
        let mut elab_ms = 0u64;
        for (_, ev) in &analytics.events {
            match ev {
                BuildEvent::ParseEnd { duration_ms, .. } => parse_ms += duration_ms,
                BuildEvent::ElabEnd { duration_ms, .. } => elab_ms += duration_ms,
                _ => {}
            }
        }
        PhaseBreakdown {
            parse_ms,
            elab_ms,
            other_ms: 0,
        }
    }
    /// Total time accounted for in this breakdown.
    pub fn total_ms(&self) -> u64 {
        self.parse_ms + self.elab_ms + self.other_ms
    }
    /// Parse fraction of total.
    pub fn parse_fraction(&self) -> f64 {
        let total = self.total_ms();
        if total == 0 {
            0.0
        } else {
            self.parse_ms as f64 / total as f64
        }
    }
    /// Elab fraction of total.
    pub fn elab_fraction(&self) -> f64 {
        let total = self.total_ms();
        if total == 0 {
            0.0
        } else {
            self.elab_ms as f64 / total as f64
        }
    }
}
/// A recorded build session for trend analysis.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BuildSession {
    /// Session identifier (e.g. commit hash or timestamp).
    pub session_id: String,
    /// When this session occurred (Unix timestamp in seconds).
    pub timestamp_secs: u64,
    /// Number of files compiled.
    pub files_compiled: u32,
    /// Total build duration (ms).
    pub duration_ms: u64,
    /// Error count.
    pub errors: u32,
    /// Cache hit rate [0.0, 1.0].
    pub cache_hit_rate: f64,
}
#[allow(dead_code)]
impl BuildSession {
    /// Create a new session record.
    pub fn new(session_id: &str, timestamp_secs: u64) -> Self {
        Self {
            session_id: session_id.to_string(),
            timestamp_secs,
            files_compiled: 0,
            duration_ms: 0,
            errors: 0,
            cache_hit_rate: 0.0,
        }
    }
}
/// Statistics about the file dependency graph
pub struct DependencyStats {
    pub file_count: usize,
    pub edge_count: usize,
    pub max_depth: usize,
    pub strongly_connected: usize,
    pub most_depended_on: Vec<(String, usize)>,
}
/// Tracks errors over time and computes rolling error rates.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct ErrorRateTracker {
    /// (timestamp_ms, is_error) pairs.
    records: Vec<(u64, bool)>,
}
#[allow(dead_code)]
impl ErrorRateTracker {
    /// Create a new tracker.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a build result.
    pub fn record(&mut self, ts_ms: u64, is_error: bool) {
        self.records.push((ts_ms, is_error));
    }
    /// Error rate in the window `[from_ms, to_ms]`.
    pub fn error_rate_in_window(&self, from_ms: u64, to_ms: u64) -> f64 {
        let window: Vec<bool> = self
            .records
            .iter()
            .filter(|(ts, _)| *ts >= from_ms && *ts <= to_ms)
            .map(|(_, e)| *e)
            .collect();
        if window.is_empty() {
            return 0.0;
        }
        let errors = window.iter().filter(|&&e| e).count();
        errors as f64 / window.len() as f64
    }
    /// Overall error rate across all records.
    pub fn overall_error_rate(&self) -> f64 {
        if self.records.is_empty() {
            return 0.0;
        }
        let errors = self.records.iter().filter(|(_, e)| *e).count();
        errors as f64 / self.records.len() as f64
    }
    /// Total error count.
    pub fn total_errors(&self) -> usize {
        self.records.iter().filter(|(_, e)| *e).count()
    }
    /// Total record count.
    pub fn total_records(&self) -> usize {
        self.records.len()
    }
}
/// Snapshot of concurrently running tasks at a given timestamp.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ParallelismSnapshot {
    /// Timestamp (ms).
    pub ts_ms: u64,
    /// Number of tasks running concurrently at this timestamp.
    pub concurrent_tasks: usize,
}
/// Tracks compiler flag usage across build units.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct CompilerFlagAnalytics {
    flag_counts: HashMap<String, u64>,
}
#[allow(dead_code)]
impl CompilerFlagAnalytics {
    /// Create a new tracker.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record flags used in a single compilation unit.
    pub fn record_flags(&mut self, flags: &[&str]) {
        for flag in flags {
            *self.flag_counts.entry(flag.to_string()).or_insert(0) += 1;
        }
    }
    /// Most commonly used flags, sorted descending.
    pub fn top_flags(&self, n: usize) -> Vec<(&str, u64)> {
        let mut v: Vec<(&str, u64)> = self
            .flag_counts
            .iter()
            .map(|(k, &v)| (k.as_str(), v))
            .collect();
        v.sort_by_key(|b| std::cmp::Reverse(b.1));
        v.truncate(n);
        v
    }
    /// Total number of distinct flags seen.
    pub fn distinct_flag_count(&self) -> usize {
        self.flag_counts.len()
    }
    /// Total flag usage across all compilation units.
    pub fn total_flag_uses(&self) -> u64 {
        self.flag_counts.values().sum()
    }
}
/// Statistics for a single watch-mode rebuild cycle.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WatchCycleStat {
    /// Cycle index (0-based).
    pub cycle: usize,
    /// Number of files changed that triggered the rebuild.
    pub files_changed: usize,
    /// Number of files recompiled.
    pub files_recompiled: usize,
    /// Rebuild duration (ms).
    pub duration_ms: u64,
    /// Whether the rebuild succeeded.
    pub success: bool,
}
/// A sample measurement for aggregation.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct MetricSample {
    /// Timestamp of the sample.
    pub ts: u64,
    /// Value of the measurement.
    pub value: f64,
}
#[allow(dead_code)]
impl MetricSample {
    /// Create a new sample.
    pub fn new(ts: u64, value: f64) -> Self {
        Self { ts, value }
    }
}
/// Exports analytics data in a requested format.
#[allow(dead_code)]
pub struct AnalyticsExporter<'a> {
    /// Reference to the analytics source.
    analytics: &'a BuildAnalytics,
}
#[allow(dead_code)]
impl<'a> AnalyticsExporter<'a> {
    /// Create a new exporter.
    pub fn new(analytics: &'a BuildAnalytics) -> Self {
        Self { analytics }
    }
    /// Export the build report in the given format.
    pub fn export(&self, format: ExportFormat) -> String {
        let report = self.analytics.generate_report();
        match format {
            ExportFormat::Json => report.to_json(),
            ExportFormat::Text => report.to_text(),
            ExportFormat::Csv => self.to_csv(&report),
            ExportFormat::Markdown => self.to_markdown(&report),
        }
    }
    fn to_csv(&self, report: &BuildReport) -> String {
        let mut s = String::from("metric,value\n");
        s.push_str(&format!("total_files,{}\n", report.total_files));
        s.push_str(&format!("cached_files,{}\n", report.cached_files));
        s.push_str(&format!(
            "total_declarations,{}\n",
            report.total_declarations
        ));
        s.push_str(&format!("total_duration_ms,{}\n", report.total_duration_ms));
        s.push_str(&format!("cache_hit_rate,{:.4}\n", report.cache_hit_rate));
        s.push_str(&format!("errors,{}\n", report.errors));
        s
    }
    fn to_markdown(&self, report: &BuildReport) -> String {
        let mut s = String::from("| Metric | Value |\n|---|---|\n");
        s.push_str(&format!("| Total files | {} |\n", report.total_files));
        s.push_str(&format!("| Cached files | {} |\n", report.cached_files));
        s.push_str(&format!(
            "| Total declarations | {} |\n",
            report.total_declarations
        ));
        s.push_str(&format!(
            "| Total duration | {} ms |\n",
            report.total_duration_ms
        ));
        s.push_str(&format!(
            "| Cache hit rate | {:.1}% |\n",
            report.cache_hit_rate * 100.0
        ));
        s.push_str(&format!("| Errors | {} |\n", report.errors));
        s
    }
}
