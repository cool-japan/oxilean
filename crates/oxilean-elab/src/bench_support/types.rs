//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use std::time::{Duration, Instant};

use oxilean_kernel::*;
use std::collections::HashMap;

/// Result of a partial-evaluation benchmark run.
#[derive(Debug, Clone, Default)]
pub struct PartialEvalBenchResult {
    /// Number of reduction steps performed.
    pub steps_performed: u64,
    /// Whether the normal form was reached.
    pub reached_nf: bool,
    /// Wall-clock time in nanoseconds.
    pub elapsed_ns: u64,
    /// Per-step times (populated only when `record_step_times` is true).
    pub step_times_ns: Vec<u64>,
}
impl PartialEvalBenchResult {
    /// Average time per step in nanoseconds.
    pub fn avg_ns_per_step(&self) -> f64 {
        if self.steps_performed == 0 {
            0.0
        } else {
            self.elapsed_ns as f64 / self.steps_performed as f64
        }
    }
    /// One-line summary.
    pub fn summary(&self) -> String {
        format!(
            "steps={} nf={} elapsed={}ns avg={:.1}ns/step",
            self.steps_performed,
            self.reached_nf,
            self.elapsed_ns,
            self.avg_ns_per_step()
        )
    }
}
/// A comparison table row for two benchmark runs.
#[derive(Debug, Clone)]
pub struct BenchCompareRow {
    /// Name of the benchmark.
    pub name: String,
    /// Baseline mean in nanoseconds.
    pub baseline_ns: f64,
    /// Candidate mean in nanoseconds.
    pub candidate_ns: f64,
}
impl BenchCompareRow {
    /// Create a new comparison row.
    pub fn new(name: impl Into<String>, baseline_ns: f64, candidate_ns: f64) -> Self {
        Self {
            name: name.into(),
            baseline_ns,
            candidate_ns,
        }
    }
    /// Ratio of candidate to baseline (> 1 means regression).
    pub fn ratio(&self) -> f64 {
        if self.baseline_ns == 0.0 {
            1.0
        } else {
            self.candidate_ns / self.baseline_ns
        }
    }
    /// Percentage change (positive = slower, negative = faster).
    pub fn pct_change(&self) -> f64 {
        (self.ratio() - 1.0) * 100.0
    }
    /// Format as a table row string.
    pub fn format_row(&self) -> String {
        format!(
            "{:40} baseline={:8.0}ns candidate={:8.0}ns change={:+.1}%",
            self.name,
            self.baseline_ns,
            self.candidate_ns,
            self.pct_change()
        )
    }
}
/// The result of comparing two benchmark runs.
#[derive(Debug, Clone)]
pub struct Comparison {
    /// Name of the baseline benchmark.
    pub baseline_name: String,
    /// Name of the candidate benchmark.
    pub candidate_name: String,
    /// Speedup factor (baseline_avg / candidate_avg). Values > 1.0 mean the
    /// candidate is faster.
    pub speedup: f64,
    /// Absolute difference in average ns (baseline - candidate). Positive
    /// means the candidate is faster.
    pub diff_ns: f64,
    /// Percentage change ((baseline - candidate) / baseline * 100).
    pub pct_change: f64,
}
impl Comparison {
    /// Compare two benchmark results.
    pub fn compare(baseline: &BenchResult, candidate: &BenchResult) -> Self {
        let speedup = if candidate.avg_ns == 0.0 {
            f64::INFINITY
        } else {
            baseline.avg_ns / candidate.avg_ns
        };
        let diff_ns = baseline.avg_ns - candidate.avg_ns;
        let pct_change = if baseline.avg_ns == 0.0 {
            0.0
        } else {
            diff_ns / baseline.avg_ns * 100.0
        };
        Self {
            baseline_name: baseline.name.clone(),
            candidate_name: candidate.name.clone(),
            speedup,
            diff_ns,
            pct_change,
        }
    }
    /// Whether the candidate is faster than the baseline.
    pub fn is_improvement(&self) -> bool {
        self.speedup > 1.0
    }
    /// Whether the candidate is slower than the baseline.
    pub fn is_regression(&self) -> bool {
        self.speedup < 1.0
    }
}
/// Represents a single hot-path entry: a function call site that accounts for
/// a significant fraction of total elaboration time.
#[derive(Debug, Clone)]
pub struct HotPathEntry {
    /// Qualified name of the function.
    pub name: String,
    /// Number of times the function was called.
    pub call_count: u64,
    /// Total time spent in the function (nanoseconds).
    pub total_ns: u64,
    /// Self time (exclusive of callees) in nanoseconds.
    pub self_ns: u64,
}
impl HotPathEntry {
    /// Create a new entry.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            call_count: 0,
            total_ns: 0,
            self_ns: 0,
        }
    }
    /// Record one call with the given total and self time.
    pub fn record(&mut self, total_ns: u64, self_ns: u64) {
        self.call_count += 1;
        self.total_ns += total_ns;
        self.self_ns += self_ns;
    }
    /// Average total time per call.
    pub fn avg_total_ns(&self) -> f64 {
        if self.call_count == 0 {
            0.0
        } else {
            self.total_ns as f64 / self.call_count as f64
        }
    }
    /// Overhead ratio: self / total.
    pub fn self_ratio(&self) -> f64 {
        if self.total_ns == 0 {
            0.0
        } else {
            self.self_ns as f64 / self.total_ns as f64
        }
    }
}
/// Strategy for warming up the CPU/cache before a benchmark run.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WarmupStrategy {
    /// No warmup at all.
    None,
    /// Run the benchmark `n` times before measuring.
    Iterations(u64),
    /// Run for at least `ms` milliseconds before measuring.
    Duration(u64),
}
impl WarmupStrategy {
    /// Apply the warmup strategy by calling `f` the appropriate number of times.
    #[allow(dead_code)]
    pub fn apply<F: FnMut()>(&self, mut f: F) {
        match self {
            WarmupStrategy::None => {}
            WarmupStrategy::Iterations(n) => {
                for _ in 0..*n {
                    f();
                }
            }
            WarmupStrategy::Duration(ms) => {
                let deadline = Instant::now() + Duration::from_millis(*ms);
                while Instant::now() < deadline {
                    f();
                }
            }
        }
    }
}
/// Severity of a detected regression.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum RegressionSeverity {
    /// Improvement (not a regression).
    Improvement,
    /// Minor regression (< 10% slower).
    Minor,
    /// Moderate regression (10-25% slower).
    Moderate,
    /// Major regression (> 25% slower).
    Major,
}
/// Represents a throughput measurement: items processed per second.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ThroughputResult {
    /// Name of the benchmark.
    pub name: String,
    /// Total items processed.
    pub items: u64,
    /// Total elapsed time in nanoseconds.
    pub elapsed_ns: u64,
    /// Items per second.
    pub items_per_sec: f64,
    /// Nanoseconds per item.
    pub ns_per_item: f64,
}
impl ThroughputResult {
    /// Compute a throughput result from item count and elapsed nanoseconds.
    #[allow(dead_code)]
    pub fn compute(name: impl Into<String>, items: u64, elapsed_ns: u64) -> Self {
        let secs = elapsed_ns as f64 / 1e9;
        let items_per_sec = if secs > 0.0 { items as f64 / secs } else { 0.0 };
        let ns_per_item = if items > 0 {
            elapsed_ns as f64 / items as f64
        } else {
            0.0
        };
        Self {
            name: name.into(),
            items,
            elapsed_ns,
            items_per_sec,
            ns_per_item,
        }
    }
    /// Format as a human-readable string.
    #[allow(dead_code)]
    pub fn format(&self) -> String {
        format!(
            "{}: {:.0} items/sec  ({:.1} ns/item)",
            self.name, self.items_per_sec, self.ns_per_item
        )
    }
}
/// Builder for constructing a BenchResult from raw fields.
#[allow(dead_code)]
pub struct BenchResultBuilder {
    name: String,
    samples: Vec<f64>,
}
impl BenchResultBuilder {
    /// Create a new builder with the given name.
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            samples: Vec::new(),
        }
    }
    /// Add a single sample (nanoseconds).
    #[allow(dead_code)]
    pub fn sample(mut self, ns: f64) -> Self {
        self.samples.push(ns);
        self
    }
    /// Add multiple samples.
    #[allow(dead_code)]
    pub fn samples(mut self, ns: impl IntoIterator<Item = f64>) -> Self {
        self.samples.extend(ns);
        self
    }
    /// Build the BenchResult.
    #[allow(dead_code)]
    pub fn build(self) -> BenchResult {
        BenchResult::from_samples(&self.name, self.samples)
    }
}
/// The result of a single benchmark run.
#[derive(Debug, Clone)]
pub struct BenchResult {
    /// Name/label of the benchmark.
    pub name: String,
    /// Average time per iteration in nanoseconds.
    pub avg_ns: f64,
    /// Minimum observed time per iteration in nanoseconds.
    pub min_ns: f64,
    /// Maximum observed time per iteration in nanoseconds.
    pub max_ns: f64,
    /// Standard deviation in nanoseconds.
    pub stddev_ns: f64,
    /// Number of measured iterations actually executed.
    pub iterations: u64,
    /// All individual sample durations in nanoseconds.
    samples: Vec<f64>,
}
impl BenchResult {
    /// Build a `BenchResult` from raw sample durations (in nanoseconds).
    pub fn from_samples(name: impl Into<String>, samples: Vec<f64>) -> Self {
        let avg_ns = mean(&samples);
        let min_ns = samples.iter().copied().fold(f64::INFINITY, f64::min);
        let max_ns = samples.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let stddev_ns = stddev(&samples);
        let iterations = samples.len() as u64;
        Self {
            name: name.into(),
            avg_ns,
            min_ns,
            max_ns,
            stddev_ns,
            iterations,
            samples,
        }
    }
    /// Return the median duration in nanoseconds.
    pub fn median_ns(&self) -> f64 {
        median(&self.samples)
    }
    /// Return a given percentile of the sample durations.
    pub fn percentile_ns(&self, p: f64) -> f64 {
        percentile(&self.samples, p)
    }
    /// Return the coefficient of variation (stddev / mean).
    pub fn cv(&self) -> f64 {
        if self.avg_ns == 0.0 {
            0.0
        } else {
            self.stddev_ns / self.avg_ns
        }
    }
    /// Return the raw samples.
    pub fn samples(&self) -> &[f64] {
        &self.samples
    }
}
/// Configuration for a benchmark run.
#[derive(Debug, Clone)]
pub struct BenchConfig {
    /// Number of measured iterations to execute.
    pub iterations: u64,
    /// Number of warm-up rounds (results discarded).
    pub warmup_rounds: u64,
    /// Optional wall-clock time limit in milliseconds. If set, the benchmark
    /// stops collecting samples once this limit is exceeded.
    pub time_limit_ms: Option<u64>,
}
impl BenchConfig {
    /// Create a quick configuration (few iterations, no warmup).
    pub fn quick() -> Self {
        Self {
            iterations: 10,
            warmup_rounds: 1,
            time_limit_ms: None,
        }
    }
    /// Create a thorough configuration (many iterations, warmup).
    pub fn thorough() -> Self {
        Self {
            iterations: 1000,
            warmup_rounds: 20,
            time_limit_ms: Some(30_000),
        }
    }
    /// Builder: set iterations.
    pub fn with_iterations(mut self, n: u64) -> Self {
        self.iterations = n;
        self
    }
    /// Builder: set warmup rounds.
    pub fn with_warmup(mut self, n: u64) -> Self {
        self.warmup_rounds = n;
        self
    }
    /// Builder: set time limit in milliseconds.
    pub fn with_time_limit_ms(mut self, ms: u64) -> Self {
        self.time_limit_ms = Some(ms);
        self
    }
}
/// Registry of all registered elaboration micro-benchmarks.
#[derive(Debug, Default)]
pub struct ElabMicroBenchRegistry {
    benches: Vec<ElabMicroBench>,
}
impl ElabMicroBenchRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self::default()
    }
    /// Register a new benchmark.
    pub fn register(&mut self, bench: ElabMicroBench) {
        self.benches.push(bench);
    }
    /// Look up a benchmark by name.
    pub fn get(&self, name: &str) -> Option<&ElabMicroBench> {
        self.benches.iter().find(|b| b.name == name)
    }
    /// Look up a benchmark mutably.
    pub fn get_mut(&mut self, name: &str) -> Option<&mut ElabMicroBench> {
        self.benches.iter_mut().find(|b| b.name == name)
    }
    /// Return all regressions (median exceeds expected by `threshold_pct`%).
    pub fn regressions(&self, threshold_pct: f64) -> Vec<&ElabMicroBench> {
        self.benches
            .iter()
            .filter(|b| b.is_regression(threshold_pct))
            .collect()
    }
    /// Produce a summary report for all registered benchmarks.
    pub fn report(&self) -> String {
        self.benches
            .iter()
            .map(|b| b.summary())
            .collect::<Vec<_>>()
            .join("\n")
    }
    /// Number of registered benchmarks.
    pub fn len(&self) -> usize {
        self.benches.len()
    }
    /// Return `true` if no benchmarks are registered.
    pub fn is_empty(&self) -> bool {
        self.benches.is_empty()
    }
    /// Iterate over all benchmarks.
    pub fn iter(&self) -> impl Iterator<Item = &ElabMicroBench> {
        self.benches.iter()
    }
}
/// A benchmark runner for elaboration tasks.
///
/// Wraps a closure that performs the work to be measured, runs it according
/// to a `BenchConfig`, and produces a `BenchResult`.
pub struct ElabBenchmark {
    name: String,
    config: BenchConfig,
}
impl ElabBenchmark {
    /// Create a new benchmark with the given name and configuration.
    pub fn new(name: impl Into<String>, config: BenchConfig) -> Self {
        Self {
            name: name.into(),
            config,
        }
    }
    /// Run the benchmark, executing `f` repeatedly and collecting timings.
    ///
    /// The closure `f` is called once per iteration. Any setup that should
    /// not be timed must happen outside `f`.
    pub fn run<F: FnMut()>(&self, mut f: F) -> BenchResult {
        for _ in 0..self.config.warmup_rounds {
            f();
        }
        let deadline = self
            .config
            .time_limit_ms
            .map(|ms| Instant::now() + Duration::from_millis(ms));
        let mut samples = Vec::with_capacity(self.config.iterations as usize);
        for _ in 0..self.config.iterations {
            if let Some(dl) = deadline {
                if Instant::now() >= dl {
                    break;
                }
            }
            let start = Instant::now();
            f();
            let elapsed = start.elapsed();
            samples.push(elapsed.as_nanos() as f64);
        }
        BenchResult::from_samples(&self.name, samples)
    }
    /// Run a benchmark whose work function returns a value. The value is
    /// consumed via `std::hint::black_box` to prevent the optimizer from
    /// eliminating the work.
    pub fn run_with_result<T, F: FnMut() -> T>(&self, mut f: F) -> BenchResult {
        self.run(move || {
            std::hint::black_box(f());
        })
    }
}
/// A simple start/stop timer based on `std::time::Instant`.
#[derive(Debug, Clone)]
pub struct Timer {
    start: Option<Instant>,
    accumulated: Duration,
    running: bool,
}
impl Timer {
    /// Create a new timer (not yet started).
    pub fn new() -> Self {
        Self {
            start: None,
            accumulated: Duration::ZERO,
            running: false,
        }
    }
    /// Start (or resume) the timer.
    pub fn start(&mut self) {
        if !self.running {
            self.start = Some(Instant::now());
            self.running = true;
        }
    }
    /// Stop the timer and accumulate elapsed time.
    pub fn stop(&mut self) {
        if self.running {
            if let Some(s) = self.start.take() {
                self.accumulated += s.elapsed();
            }
            self.running = false;
        }
    }
    /// Return the total accumulated duration. If the timer is still running,
    /// includes the time since the last `start`.
    pub fn elapsed(&self) -> Duration {
        let extra = if self.running {
            self.start.map(|s| s.elapsed()).unwrap_or(Duration::ZERO)
        } else {
            Duration::ZERO
        };
        self.accumulated + extra
    }
    /// Return elapsed time in nanoseconds.
    pub fn elapsed_ns(&self) -> u128 {
        self.elapsed().as_nanos()
    }
    /// Reset the timer to zero.
    pub fn reset(&mut self) {
        self.start = None;
        self.accumulated = Duration::ZERO;
        self.running = false;
    }
    /// Whether the timer is currently running.
    pub fn is_running(&self) -> bool {
        self.running
    }
}
/// Runs a collection of named benchmarks with a shared configuration.
#[allow(dead_code)]
pub struct BenchmarkSet {
    config: BenchConfig,
    suite: BenchSuite,
}
impl BenchmarkSet {
    /// Create a new BenchmarkSet with the given configuration.
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, config: BenchConfig) -> Self {
        Self {
            config,
            suite: BenchSuite::new(name),
        }
    }
    /// Add and run a benchmark with the given name.
    #[allow(dead_code)]
    pub fn bench<F: FnMut()>(&mut self, name: impl Into<String>, f: F) {
        let bench = ElabBenchmark::new(name, self.config.clone());
        let result = bench.run(f);
        self.suite.add_result(result);
    }
    /// Add and run a benchmark that returns a value (black-boxed).
    #[allow(dead_code)]
    pub fn bench_with_result<T, F: FnMut() -> T>(&mut self, name: impl Into<String>, f: F) {
        let bench = ElabBenchmark::new(name, self.config.clone());
        let result = bench.run_with_result(f);
        self.suite.add_result(result);
    }
    /// Consume the set and return the resulting suite.
    #[allow(dead_code)]
    pub fn into_suite(self) -> BenchSuite {
        self.suite
    }
    /// Return the current number of benchmarks in the set.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.suite.len()
    }
    /// Return true if no benchmarks have been run.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.suite.is_empty()
    }
}
/// A simple moving average over a fixed window of samples.
#[allow(dead_code)]
pub struct MovingAverage {
    window: Vec<f64>,
    capacity: usize,
    head: usize,
    count: usize,
    sum: f64,
}
impl MovingAverage {
    /// Create a new MovingAverage with the given window size.
    #[allow(dead_code)]
    pub fn new(capacity: usize) -> Self {
        let cap = capacity.max(1);
        Self {
            window: vec![0.0; cap],
            capacity: cap,
            head: 0,
            count: 0,
            sum: 0.0,
        }
    }
    /// Push a new sample into the moving average.
    #[allow(dead_code)]
    pub fn push(&mut self, value: f64) {
        if self.count == self.capacity {
            self.sum -= self.window[self.head];
        } else {
            self.count += 1;
        }
        self.window[self.head] = value;
        self.sum += value;
        self.head = (self.head + 1) % self.capacity;
    }
    /// Current moving average, or 0.0 if no samples.
    #[allow(dead_code)]
    pub fn average(&self) -> f64 {
        if self.count == 0 {
            0.0
        } else {
            self.sum / self.count as f64
        }
    }
    /// Number of samples currently in the window.
    #[allow(dead_code)]
    pub fn count(&self) -> usize {
        self.count
    }
    /// Reset the moving average.
    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.window.fill(0.0);
        self.head = 0;
        self.count = 0;
        self.sum = 0.0;
    }
}
/// Controls a lightweight flamegraph sampler that records the call stack at
/// fixed intervals.  The implementation here is a stub; a real production
/// implementation would integrate with `inferno` or `pprof`.
#[derive(Debug, Default)]
pub struct FlamegraphHook {
    /// Whether sampling is currently active.
    active: bool,
    /// Collected stack frames: each entry is a `(depth, label)` pair.
    frames: Vec<(usize, String)>,
    /// Sampling interval in microseconds.
    #[allow(dead_code)]
    interval_us: u64,
    /// Total samples collected.
    sample_count: u64,
}
impl FlamegraphHook {
    /// Create a new flamegraph hook with the given sampling interval.
    pub fn new(interval_us: u64) -> Self {
        Self {
            active: false,
            frames: Vec::new(),
            interval_us,
            sample_count: 0,
        }
    }
    /// Start sampling.
    pub fn start(&mut self) {
        self.active = true;
    }
    /// Stop sampling.
    pub fn stop(&mut self) {
        self.active = false;
    }
    /// Record a simulated stack frame at `depth` with the given `label`.
    pub fn record_frame(&mut self, depth: usize, label: impl Into<String>) {
        if self.active {
            self.frames.push((depth, label.into()));
            self.sample_count += 1;
        }
    }
    /// Return the number of samples recorded.
    pub fn sample_count(&self) -> u64 {
        self.sample_count
    }
    /// Produce a collapsed-stack representation suitable for `inferno`.
    ///
    /// Each line is `frame1;frame2;...;frameN count`.
    pub fn collapsed_stacks(&self) -> Vec<String> {
        let mut out = Vec::new();
        let mut current_stack: Vec<String> = Vec::new();
        for (depth, label) in &self.frames {
            current_stack.truncate(*depth);
            current_stack.push(label.clone());
            out.push(format!("{} 1", current_stack.join(";")));
        }
        out
    }
    /// Clear all recorded frames.
    pub fn clear(&mut self) {
        self.frames.clear();
        self.sample_count = 0;
    }
    /// Return whether the hook is active.
    pub fn is_active(&self) -> bool {
        self.active
    }
}
/// A single elaboration micro-benchmark entry.
#[derive(Debug, Clone)]
pub struct ElabMicroBench {
    /// Unique name of this benchmark.
    pub name: String,
    /// Human-readable description.
    pub description: String,
    /// Expected median time in nanoseconds (for regression detection).
    pub expected_ns: Option<u64>,
    /// Measured results from the last run (in nanoseconds per iteration).
    pub measurements: Vec<u64>,
}
impl ElabMicroBench {
    /// Create a new micro-benchmark entry.
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            expected_ns: None,
            measurements: Vec::new(),
        }
    }
    /// Set an expected median time in nanoseconds.
    pub fn with_expected_ns(mut self, ns: u64) -> Self {
        self.expected_ns = Some(ns);
        self
    }
    /// Record a measurement in nanoseconds.
    pub fn record(&mut self, ns: u64) {
        self.measurements.push(ns);
    }
    /// Compute the median of recorded measurements.
    pub fn median_ns(&self) -> Option<u64> {
        if self.measurements.is_empty() {
            return None;
        }
        let mut sorted = self.measurements.clone();
        sorted.sort_unstable();
        let mid = sorted.len() / 2;
        Some(if sorted.len() % 2 == 1 {
            sorted[mid]
        } else {
            (sorted[mid - 1] + sorted[mid]) / 2
        })
    }
    /// Compute the mean of recorded measurements.
    pub fn mean_ns(&self) -> Option<f64> {
        if self.measurements.is_empty() {
            return None;
        }
        let sum: u64 = self.measurements.iter().sum();
        Some(sum as f64 / self.measurements.len() as f64)
    }
    /// Compute the standard deviation of the measurements.
    pub fn stddev_ns(&self) -> Option<f64> {
        let mean = self.mean_ns()?;
        let variance = self.measurements.iter().fold(0.0_f64, |acc, &x| {
            let diff = x as f64 - mean;
            acc + diff * diff
        }) / self.measurements.len() as f64;
        Some(variance.sqrt())
    }
    /// Return `true` if the median exceeds the expected value by more than
    /// `threshold_pct` percent.
    pub fn is_regression(&self, threshold_pct: f64) -> bool {
        if let (Some(expected), Some(median)) = (self.expected_ns, self.median_ns()) {
            let ratio = (median as f64 - expected as f64) / expected as f64 * 100.0;
            ratio > threshold_pct
        } else {
            false
        }
    }
    /// Produce a one-line summary of this benchmark.
    pub fn summary(&self) -> String {
        match (self.mean_ns(), self.stddev_ns()) {
            (Some(mean), Some(sd)) => {
                format!(
                    "{}: mean={:.0}ns std={:.0}ns n={}",
                    self.name,
                    mean,
                    sd,
                    self.measurements.len()
                )
            }
            _ => format!("{}: no data", self.name),
        }
    }
}
/// Collects and ranks hot-path entries by total time.
#[derive(Debug, Default)]
pub struct HotPathAnalyzer {
    entries: std::collections::HashMap<String, HotPathEntry>,
}
impl HotPathAnalyzer {
    /// Create an empty analyzer.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a call to function `name`.
    pub fn record(&mut self, name: impl Into<String>, total_ns: u64, self_ns: u64) {
        let name = name.into();
        self.entries
            .entry(name.clone())
            .or_insert_with(|| HotPathEntry::new(name))
            .record(total_ns, self_ns);
    }
    /// Return the top-N hottest functions by total time.
    pub fn top_n(&self, n: usize) -> Vec<&HotPathEntry> {
        let mut entries: Vec<&HotPathEntry> = self.entries.values().collect();
        entries.sort_by(|a, b| b.total_ns.cmp(&a.total_ns));
        entries.truncate(n);
        entries
    }
    /// Total time accounted for across all entries.
    pub fn total_time_ns(&self) -> u64 {
        self.entries.values().map(|e| e.total_ns).sum()
    }
    /// Produce a report of the top-N functions.
    pub fn report(&self, n: usize) -> String {
        let top = self.top_n(n);
        let total = self.total_time_ns();
        top.iter()
            .map(|e| {
                let pct = if total == 0 {
                    0.0
                } else {
                    e.total_ns as f64 / total as f64 * 100.0
                };
                format!(
                    "{:40} calls={:6} total={:10}ns self%={:.0}% share={:.1}%",
                    e.name,
                    e.call_count,
                    e.total_ns,
                    e.self_ratio() * 100.0,
                    pct
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}
/// Controls retry behaviour when a benchmark run exceeds a variance threshold.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    /// Maximum number of retry attempts.
    pub max_retries: u32,
    /// Maximum allowed coefficient of variation before retrying.
    pub cv_threshold: f64,
}
impl RetryPolicy {
    /// Create a lenient retry policy.
    #[allow(dead_code)]
    pub fn lenient() -> Self {
        Self {
            max_retries: 3,
            cv_threshold: 0.2,
        }
    }
    /// Create a strict retry policy.
    #[allow(dead_code)]
    pub fn strict() -> Self {
        Self {
            max_retries: 10,
            cv_threshold: 0.05,
        }
    }
}
/// Metadata associated with a benchmark for reporting.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BenchMeta {
    /// Short description of what the benchmark measures.
    pub description: String,
    /// Tags for grouping (e.g., "elab", "kernel", "parse").
    pub tags: Vec<String>,
    /// Whether this benchmark is expected to be stable (low variance).
    pub expect_stable: bool,
}
impl BenchMeta {
    /// Create a new BenchMeta.
    #[allow(dead_code)]
    pub fn new(description: impl Into<String>) -> Self {
        Self {
            description: description.into(),
            tags: Vec::new(),
            expect_stable: false,
        }
    }
    /// Add a tag.
    #[allow(dead_code)]
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }
    /// Mark as expected to be stable.
    #[allow(dead_code)]
    pub fn mark_stable(mut self) -> Self {
        self.expect_stable = true;
        self
    }
    /// Return true if the tag is present.
    #[allow(dead_code)]
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}
/// A fixed-size histogram for latency values (nanoseconds).
#[allow(dead_code)]
pub struct LatencyHistogram {
    boundaries: Vec<f64>,
    counts: Vec<u64>,
    total: u64,
}
impl LatencyHistogram {
    /// Create a new histogram with logarithmically spaced buckets.
    #[allow(dead_code)]
    pub fn new(num_buckets: usize, min_ns: f64, max_ns: f64) -> Self {
        let n = num_buckets.max(2);
        let log_min = min_ns.max(1.0).ln();
        let log_max = max_ns.max(min_ns + 1.0).ln();
        let step = (log_max - log_min) / n as f64;
        let mut boundaries = Vec::with_capacity(n + 1);
        for i in 0..=n {
            boundaries.push((log_min + i as f64 * step).exp());
        }
        let counts = vec![0u64; n];
        Self {
            boundaries,
            counts,
            total: 0,
        }
    }
    /// Record a sample (in nanoseconds).
    #[allow(dead_code)]
    pub fn record(&mut self, ns: f64) {
        self.total += 1;
        let idx = self
            .boundaries
            .windows(2)
            .position(|w| ns < w[1])
            .unwrap_or(self.counts.len() - 1);
        self.counts[idx] += 1;
    }
    /// Returns the fraction of samples in each bucket.
    #[allow(dead_code)]
    pub fn fractions(&self) -> Vec<f64> {
        if self.total == 0 {
            return vec![0.0; self.counts.len()];
        }
        self.counts
            .iter()
            .map(|&c| c as f64 / self.total as f64)
            .collect()
    }
    /// Total number of samples recorded.
    #[allow(dead_code)]
    pub fn total(&self) -> u64 {
        self.total
    }
    /// Number of buckets.
    #[allow(dead_code)]
    pub fn num_buckets(&self) -> usize {
        self.counts.len()
    }
    /// Format a text histogram.
    #[allow(dead_code)]
    pub fn format_ascii(&self) -> String {
        let fracs = self.fractions();
        let mut out = String::new();
        for (i, frac) in fracs.iter().enumerate() {
            let lo = self.boundaries[i];
            let hi = self.boundaries[i + 1];
            let bar_len = (frac * 40.0).round() as usize;
            let bar = "#".repeat(bar_len);
            out.push_str(&format!(
                "[{:>8.0}, {:>8.0}) {:5.1}% |{}\n",
                lo,
                hi,
                frac * 100.0,
                bar
            ));
        }
        out
    }
}
/// A scheduler that retries benchmark runs until results are stable.
#[allow(dead_code)]
pub struct BenchScheduler {
    config: BenchConfig,
    policy: RetryPolicy,
}
impl BenchScheduler {
    /// Create a new scheduler.
    #[allow(dead_code)]
    pub fn new(config: BenchConfig, policy: RetryPolicy) -> Self {
        Self { config, policy }
    }
    /// Run a benchmark, retrying if the CV exceeds the threshold.
    #[allow(dead_code)]
    pub fn run_stable<F: FnMut() + Clone>(&self, name: impl Into<String>, mut f: F) -> BenchResult {
        let name_str: String = name.into();
        let bench = ElabBenchmark::new(name_str.clone(), self.config.clone());
        let mut best = bench.run(f.clone());
        for _ in 0..self.policy.max_retries {
            if best.cv() <= self.policy.cv_threshold {
                break;
            }
            let attempt = ElabBenchmark::new(name_str.clone(), self.config.clone()).run(f.clone());
            if attempt.cv() < best.cv() {
                best = attempt;
            }
            f = f.clone();
        }
        best
    }
}
/// Cumulative statistics from a constraint-solver run, used for benchmarking
/// the integration with the elaborator's constraint engine.
#[derive(Debug, Clone, Default)]
pub struct SolverBenchStats {
    /// Total number of constraint-solving invocations.
    pub invocations: u64,
    /// Total constraints solved (across all invocations).
    pub constraints_solved: u64,
    /// Total failed constraint-solving attempts.
    pub failures: u64,
    /// Wall-clock time spent in the solver (nanoseconds).
    pub total_solver_ns: u64,
    /// Maximum depth reached by the solver.
    pub max_depth: u32,
    /// Number of times the occurs-check was triggered.
    pub occurs_check_hits: u64,
}
impl SolverBenchStats {
    /// Create zeroed statistics.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record one solver invocation.
    pub fn record_invocation(&mut self, constraints: u64, _ok: bool, elapsed_ns: u64) {
        self.invocations += 1;
        if _ok {
            self.constraints_solved += constraints;
        } else {
            self.failures += 1;
        }
        self.total_solver_ns += elapsed_ns;
    }
    /// Record the depth reached in one solver call.
    pub fn record_depth(&mut self, depth: u32) {
        if depth > self.max_depth {
            self.max_depth = depth;
        }
    }
    /// Record an occurs-check hit.
    pub fn record_occurs_check(&mut self) {
        self.occurs_check_hits += 1;
    }
    /// Average solver time per invocation in nanoseconds.
    pub fn avg_ns_per_invocation(&self) -> f64 {
        if self.invocations == 0 {
            0.0
        } else {
            self.total_solver_ns as f64 / self.invocations as f64
        }
    }
    /// Success rate in [0, 1].
    pub fn success_rate(&self) -> f64 {
        let total = self.invocations;
        if total == 0 {
            1.0
        } else {
            (total - self.failures) as f64 / total as f64
        }
    }
    /// Merge another set of stats into this one.
    pub fn merge(&mut self, other: &SolverBenchStats) {
        self.invocations += other.invocations;
        self.constraints_solved += other.constraints_solved;
        self.failures += other.failures;
        self.total_solver_ns += other.total_solver_ns;
        if other.max_depth > self.max_depth {
            self.max_depth = other.max_depth;
        }
        self.occurs_check_hits += other.occurs_check_hits;
    }
    /// One-line summary string.
    pub fn summary(&self) -> String {
        format!(
            "invocations={} solved={} failures={} avg={:.0}ns max_depth={} oc_hits={}",
            self.invocations,
            self.constraints_solved,
            self.failures,
            self.avg_ns_per_invocation(),
            self.max_depth,
            self.occurs_check_hits,
        )
    }
}
/// Configuration for a partial-evaluation benchmark pass.
#[derive(Debug, Clone)]
pub struct PartialEvalBenchConfig {
    /// Maximum number of reduction steps before aborting.
    pub max_steps: u64,
    /// Whether to record per-step timing.
    pub record_step_times: bool,
    /// Whether to count normal-form reductions only.
    pub nf_only: bool,
}
impl PartialEvalBenchConfig {
    /// Create a default config.
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the step limit.
    pub fn with_max_steps(mut self, n: u64) -> Self {
        self.max_steps = n;
        self
    }
    /// Enable per-step timing recording.
    pub fn with_step_times(mut self) -> Self {
        self.record_step_times = true;
        self
    }
    /// Restrict to normal-form reductions only.
    pub fn nf_only(mut self) -> Self {
        self.nf_only = true;
        self
    }
}
/// A full comparison table between two benchmark runs.
#[derive(Debug, Default)]
pub struct BenchCompareTable {
    rows: Vec<BenchCompareRow>,
}
impl BenchCompareTable {
    /// Create an empty comparison table.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a row to the table.
    pub fn add(&mut self, row: BenchCompareRow) {
        self.rows.push(row);
    }
    /// Return all rows where the candidate is slower by more than `threshold_pct`.
    pub fn regressions(&self, threshold_pct: f64) -> Vec<&BenchCompareRow> {
        self.rows
            .iter()
            .filter(|r| r.pct_change() > threshold_pct)
            .collect()
    }
    /// Return all rows where the candidate is faster by more than `threshold_pct`.
    pub fn improvements(&self, threshold_pct: f64) -> Vec<&BenchCompareRow> {
        self.rows
            .iter()
            .filter(|r| r.pct_change() < -threshold_pct)
            .collect()
    }
    /// Produce a full formatted report.
    pub fn format_report(&self) -> String {
        self.rows
            .iter()
            .map(|r| r.format_row())
            .collect::<Vec<_>>()
            .join("\n")
    }
    /// Number of rows in the table.
    pub fn len(&self) -> usize {
        self.rows.len()
    }
    /// Return `true` if the table is empty.
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}
/// A detected performance regression.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RegressionReport {
    /// The comparison that triggered this report.
    pub comparison: Comparison,
    /// Severity of the regression.
    pub severity: RegressionSeverity,
    /// Human-readable description.
    pub message: String,
}
impl RegressionReport {
    /// Analyse a comparison and produce a regression report.
    #[allow(dead_code)]
    pub fn from_comparison(cmp: Comparison) -> Self {
        let pct = -cmp.pct_change;
        let severity = if pct <= 0.0 {
            RegressionSeverity::Improvement
        } else if pct < 10.0 {
            RegressionSeverity::Minor
        } else if pct < 25.0 {
            RegressionSeverity::Moderate
        } else {
            RegressionSeverity::Major
        };
        let message = format!(
            "{}: {:.1}% change ({:.2}x speedup)",
            severity, cmp.pct_change, cmp.speedup,
        );
        Self {
            comparison: cmp,
            severity,
            message,
        }
    }
    /// Returns true if this report represents an actual regression.
    #[allow(dead_code)]
    pub fn is_regression(&self) -> bool {
        self.severity != RegressionSeverity::Improvement
    }
}
/// A collection of related benchmarks.
#[derive(Debug, Clone)]
pub struct BenchSuite {
    /// Suite name.
    pub name: String,
    /// Collected results.
    results: Vec<BenchResult>,
}
impl BenchSuite {
    /// Create a new empty suite.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            results: Vec::new(),
        }
    }
    /// Add a benchmark result.
    pub fn add_result(&mut self, result: BenchResult) {
        self.results.push(result);
    }
    /// Number of benchmarks in the suite.
    pub fn len(&self) -> usize {
        self.results.len()
    }
    /// Whether the suite is empty.
    pub fn is_empty(&self) -> bool {
        self.results.is_empty()
    }
    /// Iterate over results.
    pub fn iter(&self) -> impl Iterator<Item = &BenchResult> {
        self.results.iter()
    }
    /// Find the fastest benchmark by average time.
    pub fn fastest(&self) -> Option<&BenchResult> {
        self.results.iter().min_by(|a, b| {
            a.avg_ns
                .partial_cmp(&b.avg_ns)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }
    /// Find the slowest benchmark by average time.
    pub fn slowest(&self) -> Option<&BenchResult> {
        self.results.iter().max_by(|a, b| {
            a.avg_ns
                .partial_cmp(&b.avg_ns)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }
    /// Generate a summary report as a plain-text table.
    pub fn summary_text(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("Suite: {}\n", self.name));
        out.push_str(&format!(
            "{:<30} {:>12} {:>12} {:>12} {:>12} {:>8}\n",
            "Benchmark", "Avg (ns)", "Min (ns)", "Max (ns)", "Stddev", "Iters"
        ));
        out.push_str(&"-".repeat(88));
        out.push('\n');
        for r in &self.results {
            out.push_str(&format!(
                "{:<30} {:>12.1} {:>12.1} {:>12.1} {:>12.1} {:>8}\n",
                r.name, r.avg_ns, r.min_ns, r.max_ns, r.stddev_ns, r.iterations
            ));
        }
        out
    }
}
