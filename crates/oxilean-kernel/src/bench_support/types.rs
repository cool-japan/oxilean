//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::time::Instant;

use std::collections::VecDeque;

/// A simple wall-clock timer backed by [`std::time::Instant`].
#[derive(Debug, Clone)]
pub struct BenchTimer {
    start: Instant,
}
impl BenchTimer {
    /// Start the timer (records the current instant).
    pub fn start() -> Self {
        Self {
            start: Instant::now(),
        }
    }
    /// Elapsed time in milliseconds since [`BenchTimer::start`] was called.
    pub fn elapsed_ms(&self) -> f64 {
        self.start.elapsed().as_secs_f64() * 1_000.0
    }
    /// Elapsed time in microseconds since [`BenchTimer::start`] was called.
    pub fn elapsed_us(&self) -> f64 {
        self.start.elapsed().as_secs_f64() * 1_000_000.0
    }
}
/// Stub for CPU affinity pinning (platform-specific in production).
#[allow(dead_code)]
pub struct CpuPinner {
    preferred_cpu: usize,
}
#[allow(dead_code)]
impl CpuPinner {
    /// Creates a CPU pinner that requests `cpu`.
    pub fn new(cpu: usize) -> Self {
        Self { preferred_cpu: cpu }
    }
    /// Attempts to pin the current thread to the preferred CPU.
    ///
    /// Always succeeds in this stub (real impl would call `sched_setaffinity`).
    pub fn pin(&self) -> bool {
        true
    }
    /// Returns the preferred CPU index.
    pub fn cpu(&self) -> usize {
        self.preferred_cpu
    }
}
/// An ε-greedy multi-arm bandit for adaptive benchmark resource allocation.
#[allow(dead_code)]
pub struct MultiArmBandit {
    arms: Vec<f64>,
    counts: Vec<u64>,
    epsilon: f64,
    total_pulls: u64,
}
#[allow(dead_code)]
impl MultiArmBandit {
    /// Creates a new bandit with `n_arms` arms and exploration rate `epsilon`.
    pub fn new(n_arms: usize, epsilon: f64) -> Self {
        Self {
            arms: vec![0.0; n_arms],
            counts: vec![0; n_arms],
            epsilon,
            total_pulls: 0,
        }
    }
    /// Selects an arm using the ε-greedy policy.
    ///
    /// Returns the arm index (0-based).
    pub fn select(&self) -> usize {
        let explore = (self.total_pulls % 100) < (self.epsilon * 100.0) as u64;
        if explore {
            (self.total_pulls as usize) % self.arms.len()
        } else {
            self.arms
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(i, _)| i)
                .unwrap_or(0)
        }
    }
    /// Updates the reward estimate for `arm` with `reward`.
    pub fn update(&mut self, arm: usize, reward: f64) {
        if arm >= self.arms.len() {
            return;
        }
        self.counts[arm] += 1;
        let n = self.counts[arm] as f64;
        self.arms[arm] += (reward - self.arms[arm]) / n;
        self.total_pulls += 1;
    }
    /// Returns the estimated reward for `arm`.
    pub fn estimate(&self, arm: usize) -> f64 {
        self.arms.get(arm).copied().unwrap_or(0.0)
    }
    /// Returns the arm with the highest estimated reward.
    pub fn best_arm(&self) -> usize {
        self.arms
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
            .unwrap_or(0)
    }
}
/// An exponential moving average over timing samples.
#[allow(dead_code)]
pub struct MovingAverage {
    alpha: f64,
    value: Option<f64>,
    count: usize,
}
#[allow(dead_code)]
impl MovingAverage {
    /// Creates a moving average with smoothing factor `alpha` (0 < alpha <= 1).
    ///
    /// Larger `alpha` gives more weight to recent samples.
    pub fn new(alpha: f64) -> Self {
        assert!(alpha > 0.0 && alpha <= 1.0);
        Self {
            alpha,
            value: None,
            count: 0,
        }
    }
    /// Updates the average with a new sample.
    pub fn update(&mut self, sample: f64) {
        self.value = Some(match self.value {
            None => sample,
            Some(v) => self.alpha * sample + (1.0 - self.alpha) * v,
        });
        self.count += 1;
    }
    /// Returns the current moving average, or `None` if no samples.
    pub fn current(&self) -> Option<f64> {
        self.value
    }
    /// Returns the number of samples seen so far.
    pub fn count(&self) -> usize {
        self.count
    }
}
/// A fixed-capacity circular buffer of timing samples.
#[allow(dead_code)]
pub struct SampleBuffer {
    data: Vec<f64>,
    capacity: usize,
    head: usize,
    count: usize,
}
#[allow(dead_code)]
impl SampleBuffer {
    /// Creates a new circular buffer with the given capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            data: vec![0.0; capacity],
            capacity,
            head: 0,
            count: 0,
        }
    }
    /// Pushes a sample, overwriting the oldest if full.
    pub fn push(&mut self, val: f64) {
        self.data[self.head] = val;
        self.head = (self.head + 1) % self.capacity;
        if self.count < self.capacity {
            self.count += 1;
        }
    }
    /// Returns the number of valid samples.
    pub fn len(&self) -> usize {
        self.count
    }
    /// Returns `true` if the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
    /// Returns the mean of all samples, or `None` if empty.
    pub fn mean(&self) -> Option<f64> {
        if self.count == 0 {
            return None;
        }
        Some(self.data[..self.count].iter().sum::<f64>() / self.count as f64)
    }
    /// Returns the minimum sample, or `None` if empty.
    pub fn min(&self) -> Option<f64> {
        if self.count == 0 {
            return None;
        }
        self.data[..self.count].iter().copied().reduce(f64::min)
    }
    /// Returns the maximum sample, or `None` if empty.
    pub fn max(&self) -> Option<f64> {
        if self.count == 0 {
            return None;
        }
        self.data[..self.count].iter().copied().reduce(f64::max)
    }
}
/// A single bin in a histogram of timing samples.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct HistogramBin {
    /// The inclusive lower bound of this bin in microseconds.
    pub lower_us: f64,
    /// The exclusive upper bound of this bin in microseconds.
    pub upper_us: f64,
    /// The number of samples that fell in this bin.
    pub count: u64,
}
#[allow(dead_code)]
impl HistogramBin {
    /// Creates a new histogram bin.
    pub fn new(lower_us: f64, upper_us: f64) -> Self {
        Self {
            lower_us,
            upper_us,
            count: 0,
        }
    }
    /// Returns the midpoint of the bin.
    pub fn mid(&self) -> f64 {
        (self.lower_us + self.upper_us) / 2.0
    }
    /// Returns `true` if `sample_us` falls within this bin.
    pub fn contains(&self, sample_us: f64) -> bool {
        sample_us >= self.lower_us && sample_us < self.upper_us
    }
}
/// Computes percentiles from a collected set of latency samples.
#[allow(dead_code)]
pub struct LatencyPercentile {
    samples: Vec<f64>,
    sorted: bool,
}
#[allow(dead_code)]
impl LatencyPercentile {
    /// Creates a new empty percentile tracker.
    pub fn new() -> Self {
        Self {
            samples: Vec::new(),
            sorted: false,
        }
    }
    /// Adds a sample (in microseconds).
    pub fn record(&mut self, us: f64) {
        self.samples.push(us);
        self.sorted = false;
    }
    fn ensure_sorted(&mut self) {
        if !self.sorted {
            self.samples
                .sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            self.sorted = true;
        }
    }
    /// Returns the p-th percentile (0.0 – 100.0), or `None` if empty.
    pub fn percentile(&mut self, p: f64) -> Option<f64> {
        if self.samples.is_empty() {
            return None;
        }
        self.ensure_sorted();
        let idx = ((p / 100.0) * (self.samples.len() - 1) as f64).round() as usize;
        Some(self.samples[idx.min(self.samples.len() - 1)])
    }
    /// Returns (p50, p90, p99) in microseconds, or `None` if empty.
    pub fn summary(&mut self) -> Option<(f64, f64, f64)> {
        Some((
            self.percentile(50.0)?,
            self.percentile(90.0)?,
            self.percentile(99.0)?,
        ))
    }
    /// Returns the total number of samples.
    pub fn count(&self) -> usize {
        self.samples.len()
    }
}
/// Checks whether a sequence of measurements has stabilised.
#[allow(dead_code)]
pub struct StabilityChecker {
    window: usize,
    threshold: f64,
    history: Vec<f64>,
}
#[allow(dead_code)]
impl StabilityChecker {
    /// Creates a checker requiring `window` consecutive measurements within
    /// `threshold` relative variation.
    pub fn new(window: usize, threshold: f64) -> Self {
        Self {
            window,
            threshold,
            history: Vec::new(),
        }
    }
    /// Adds a measurement.  Returns `true` when stability is detected.
    pub fn push(&mut self, val: f64) -> bool {
        self.history.push(val);
        if self.history.len() < self.window {
            return false;
        }
        let recent = &self.history[self.history.len() - self.window..];
        let mean = recent.iter().sum::<f64>() / recent.len() as f64;
        if mean.abs() < f64::EPSILON {
            return true;
        }
        let max_dev = recent
            .iter()
            .map(|&x| (x - mean).abs())
            .fold(0.0f64, f64::max);
        max_dev / mean < self.threshold
    }
    /// Returns the number of measurements seen so far.
    pub fn count(&self) -> usize {
        self.history.len()
    }
}
/// A set of annotations for a single benchmark result.
#[allow(dead_code)]
pub struct BenchAnnotationSet {
    entries: Vec<BenchAnnotation>,
}
#[allow(dead_code)]
impl BenchAnnotationSet {
    /// Creates an empty annotation set.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    /// Adds an annotation.
    pub fn add(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.entries.push(BenchAnnotation::new(key, value));
    }
    /// Returns the value for `key`, or `None`.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries
            .iter()
            .find(|e| e.key == key)
            .map(|e| e.value.as_str())
    }
    /// Returns the number of annotations.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Returns `true` if there are no annotations.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
/// A time slice reservation for a benchmark run.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct TimeSlice {
    /// Label for this time slice.
    pub label: String,
    /// Allocated duration in milliseconds.
    pub budget_ms: f64,
    /// Used duration in milliseconds.
    pub used_ms: f64,
}
#[allow(dead_code)]
impl TimeSlice {
    /// Creates a time slice with `budget_ms` milliseconds.
    pub fn new(label: impl Into<String>, budget_ms: f64) -> Self {
        Self {
            label: label.into(),
            budget_ms,
            used_ms: 0.0,
        }
    }
    /// Records `ms` milliseconds of usage.  Returns `true` if over budget.
    pub fn consume(&mut self, ms: f64) -> bool {
        self.used_ms += ms;
        self.used_ms > self.budget_ms
    }
    /// Returns remaining budget in milliseconds.
    pub fn remaining_ms(&self) -> f64 {
        (self.budget_ms - self.used_ms).max(0.0)
    }
    /// Returns the fraction of budget used (0.0–).
    pub fn utilisation(&self) -> f64 {
        if self.budget_ms < f64::EPSILON {
            return 0.0;
        }
        self.used_ms / self.budget_ms
    }
}
/// Collects `BenchResultExt` objects and formats a final report.
#[allow(dead_code)]
pub struct BenchReporter {
    results: Vec<BenchResultExt>,
    suite_name: String,
}
#[allow(dead_code)]
impl BenchReporter {
    /// Creates a new reporter for the given suite.
    pub fn new(suite_name: impl Into<String>) -> Self {
        Self {
            results: Vec::new(),
            suite_name: suite_name.into(),
        }
    }
    /// Adds a result to the reporter.
    pub fn add(&mut self, result: BenchResultExt) {
        self.results.push(result);
    }
    /// Returns the number of results.
    pub fn count(&self) -> usize {
        self.results.len()
    }
    /// Returns a CSV report of all results.
    pub fn to_csv(&self) -> String {
        let header = "name,median_us,mean_us,stddev_us,iterations\n".to_string();
        header
            + &self
                .results
                .iter()
                .map(|r| r.to_csv())
                .collect::<Vec<_>>()
                .join("\n")
    }
    /// Returns the result with the lowest median, or `None`.
    pub fn fastest(&self) -> Option<&BenchResultExt> {
        self.results.iter().min_by(|a, b| {
            a.median_us
                .partial_cmp(&b.median_us)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }
    /// Returns the result with the highest median, or `None`.
    pub fn slowest(&self) -> Option<&BenchResultExt> {
        self.results.iter().max_by(|a, b| {
            a.median_us
                .partial_cmp(&b.median_us)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }
    /// Returns the suite name.
    pub fn suite(&self) -> &str {
        &self.suite_name
    }
}
/// A simplified HDR (High Dynamic Range) histogram.
///
/// Supports sub-microsecond resolution up to 1 second (1e6 µs).
#[allow(dead_code)]
pub struct HdrHistogram {
    buckets: Vec<u64>,
    max_us: f64,
}
#[allow(dead_code)]
impl HdrHistogram {
    /// Creates a new HDR histogram with `n_buckets` linear buckets from 0 to `max_us`.
    pub fn new(max_us: f64, n_buckets: usize) -> Self {
        Self {
            buckets: vec![0; n_buckets.max(1)],
            max_us,
        }
    }
    /// Records a sample in microseconds.
    pub fn record(&mut self, us: f64) {
        let n = self.buckets.len();
        let idx = if us >= self.max_us {
            n - 1
        } else {
            ((us / self.max_us) * n as f64) as usize
        };
        self.buckets[idx.min(n - 1)] += 1;
    }
    /// Returns the value at the given percentile (0–100).
    pub fn value_at_percentile(&self, pct: f64) -> f64 {
        let total: u64 = self.buckets.iter().sum();
        if total == 0 {
            return 0.0;
        }
        let target = ((pct / 100.0) * total as f64).ceil() as u64;
        let n = self.buckets.len();
        let mut cumulative = 0u64;
        for (i, &count) in self.buckets.iter().enumerate() {
            cumulative += count;
            if cumulative >= target {
                return (i as f64 + 0.5) / n as f64 * self.max_us;
            }
        }
        self.max_us
    }
    /// Returns the total number of samples recorded.
    pub fn total_count(&self) -> u64 {
        self.buckets.iter().sum()
    }
}
/// Top-level configuration for a benchmark run.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct BenchConfig {
    /// Whether to print progress during the run.
    pub verbose: bool,
    /// Whether to output results as JSON.
    pub json_output: bool,
    /// Directory to write result files, if any.
    pub output_dir: Option<String>,
    /// Number of warmup iterations.
    pub warmup_iters: u64,
    /// Number of measurement iterations.
    pub measure_iters: u64,
    /// Whether to simulate cold-cache conditions.
    pub cold_cache: bool,
    /// Optional seed for deterministic pseudo-random inputs.
    pub fuzz_seed: Option<u64>,
}
#[allow(dead_code)]
impl BenchConfig {
    /// Creates a default configuration.
    pub fn default_config() -> Self {
        Self {
            verbose: false,
            json_output: false,
            output_dir: None,
            warmup_iters: 3,
            measure_iters: 10,
            cold_cache: false,
            fuzz_seed: None,
        }
    }
    /// Creates a fast configuration for CI pipelines.
    pub fn ci_config() -> Self {
        Self {
            verbose: false,
            json_output: true,
            output_dir: None,
            warmup_iters: 1,
            measure_iters: 5,
            cold_cache: false,
            fuzz_seed: Some(42),
        }
    }
    /// Returns `true` if random-input generation is enabled.
    pub fn has_fuzz(&self) -> bool {
        self.fuzz_seed.is_some()
    }
}
/// Tracks throughput (items per second) over a rolling window.
#[allow(dead_code)]
pub struct ThroughputTracker {
    window_ms: f64,
    events: std::collections::VecDeque<(std::time::Instant, u64)>,
}
#[allow(dead_code)]
impl ThroughputTracker {
    /// Creates a tracker with the given rolling window in milliseconds.
    pub fn new(window_ms: f64) -> Self {
        Self {
            window_ms,
            events: std::collections::VecDeque::new(),
        }
    }
    /// Records that `count` items were processed at this instant.
    pub fn record(&mut self, count: u64) {
        let now = std::time::Instant::now();
        self.events.push_back((now, count));
        let cutoff = now - std::time::Duration::from_secs_f64(self.window_ms / 1000.0);
        while self.events.front().is_some_and(|(t, _)| *t < cutoff) {
            self.events.pop_front();
        }
    }
    /// Returns the estimated throughput in items per second.
    pub fn items_per_sec(&self) -> f64 {
        if self.events.len() < 2 {
            return 0.0;
        }
        let total_items: u64 = self.events.iter().map(|(_, c)| c).sum();
        let duration = self
            .events
            .back()
            .expect("events non-empty: checked len >= 2 above")
            .0
            .duration_since(
                self.events
                    .front()
                    .expect("events non-empty: checked len >= 2 above")
                    .0,
            )
            .as_secs_f64();
        if duration < f64::EPSILON {
            return 0.0;
        }
        total_items as f64 / duration
    }
}
/// A statistical confidence interval for a timing measurement.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ConfidenceInterval {
    /// The point estimate (e.g. mean latency in µs).
    pub estimate: f64,
    /// The lower bound of the confidence interval.
    pub lower: f64,
    /// The upper bound of the confidence interval.
    pub upper: f64,
    /// Confidence level (e.g. 0.95 for 95 %).
    pub level: f64,
}
#[allow(dead_code)]
impl ConfidenceInterval {
    /// Computes a 95 % confidence interval using a normal approximation.
    ///
    /// Requires at least one sample; returns `None` otherwise.
    pub fn compute_95(samples: &[f64]) -> Option<Self> {
        if samples.is_empty() {
            return None;
        }
        let n = samples.len() as f64;
        let mean = samples.iter().sum::<f64>() / n;
        let var = samples.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / n;
        let sem = var.sqrt() / n.sqrt();
        let z = 1.96;
        Some(Self {
            estimate: mean,
            lower: mean - z * sem,
            upper: mean + z * sem,
            level: 0.95,
        })
    }
    /// Returns the half-width of the interval.
    pub fn half_width(&self) -> f64 {
        (self.upper - self.lower) / 2.0
    }
    /// Returns `true` if the interval is entirely positive.
    pub fn is_positive(&self) -> bool {
        self.lower > 0.0
    }
    /// Formats as `estimate ± half_width`.
    pub fn display(&self) -> String {
        format!(
            "{:.3} ± {:.3} ({}% CI)",
            self.estimate,
            self.half_width(),
            (self.level * 100.0) as u32
        )
    }
}
/// A registry of benchmark groups.
#[allow(dead_code)]
pub struct BenchRegistry {
    groups: Vec<BenchGroup>,
}
#[allow(dead_code)]
impl BenchRegistry {
    /// Creates an empty registry.
    pub fn new() -> Self {
        Self { groups: Vec::new() }
    }
    /// Adds a group to the registry.
    pub fn add_group(&mut self, group: BenchGroup) {
        self.groups.push(group);
    }
    /// Returns the total number of benchmarks across all groups.
    pub fn total_benchmarks(&self) -> usize {
        self.groups.iter().map(|g| g.size()).sum()
    }
    /// Returns all benchmark names across all groups.
    pub fn all_benchmark_names(&self) -> Vec<&str> {
        self.groups
            .iter()
            .flat_map(|g| g.benchmarks.iter().map(|s| s.as_str()))
            .collect()
    }
    /// Finds the group containing `bench_name`, if any.
    pub fn find_group(&self, bench_name: &str) -> Option<&BenchGroup> {
        self.groups
            .iter()
            .find(|g| g.benchmarks.iter().any(|b| b == bench_name))
    }
}
/// A compact summary produced at the end of a benchmark run.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct BenchSummary {
    /// Name of the benchmark suite.
    pub suite: String,
    /// Total wall time for the entire run.
    pub total_ms: f64,
    /// Number of benchmarks that passed.
    pub passed: usize,
    /// Number of benchmarks that detected regressions.
    pub regressions: usize,
    /// Number of benchmarks that were skipped.
    pub skipped: usize,
}
#[allow(dead_code)]
impl BenchSummary {
    /// Creates a new empty summary.
    pub fn new(suite: impl Into<String>) -> Self {
        Self {
            suite: suite.into(),
            total_ms: 0.0,
            passed: 0,
            regressions: 0,
            skipped: 0,
        }
    }
    /// Returns the total number of benchmarks (passed + regressions + skipped).
    pub fn total(&self) -> usize {
        self.passed + self.regressions + self.skipped
    }
    /// Returns `true` if all non-skipped benchmarks passed.
    pub fn all_passed(&self) -> bool {
        self.regressions == 0
    }
    /// Returns a one-line result string.
    pub fn result_line(&self) -> String {
        format!(
            "[{}] {}/{} passed, {} regressions, {:.1} ms total",
            self.suite,
            self.passed,
            self.total(),
            self.regressions,
            self.total_ms
        )
    }
}
/// A named floating-point metric value.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct Metric {
    /// Name of the metric.
    pub name: String,
    /// Numeric value.
    pub value: f64,
    /// Unit string (e.g. `"µs"`, `"MB/s"`).
    pub unit: String,
}
#[allow(dead_code)]
impl Metric {
    /// Creates a new metric.
    pub fn new(name: impl Into<String>, value: f64, unit: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value,
            unit: unit.into(),
        }
    }
    /// Formats as `name: value unit`.
    pub fn display(&self) -> String {
        format!("{}: {} {}", self.name, self.value, self.unit)
    }
}
/// Compares a measured value against a baseline and flags regressions.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct RegressionTest {
    /// Human-readable name of the benchmark.
    pub name: String,
    /// Baseline value (e.g. median latency in µs).
    pub baseline: f64,
    /// Maximum allowed regression factor (e.g. 1.10 = 10 % slower is OK).
    pub threshold: f64,
}
#[allow(dead_code)]
impl RegressionTest {
    /// Creates a new regression test.
    pub fn new(name: impl Into<String>, baseline: f64, threshold: f64) -> Self {
        Self {
            name: name.into(),
            baseline,
            threshold,
        }
    }
    /// Returns `true` if `measured` represents a regression.
    pub fn is_regression(&self, measured: f64) -> bool {
        measured > self.baseline * self.threshold
    }
    /// Returns the ratio `measured / baseline`.
    pub fn ratio(&self, measured: f64) -> f64 {
        if self.baseline.abs() < f64::EPSILON {
            return 1.0;
        }
        measured / self.baseline
    }
    /// Returns a human-readable verdict.
    pub fn verdict(&self, measured: f64) -> &'static str {
        if self.is_regression(measured) {
            "REGRESSION"
        } else {
            "PASS"
        }
    }
}
/// A fixed-width histogram over microsecond timing samples.
#[allow(dead_code)]
pub struct BenchHistogram {
    bins: Vec<HistogramBin>,
    underflow: u64,
    overflow: u64,
}
#[allow(dead_code)]
impl BenchHistogram {
    /// Creates a histogram with `n_bins` equal-width bins from
    /// `min_us` to `max_us`.
    pub fn new(min_us: f64, max_us: f64, n_bins: usize) -> Self {
        assert!(max_us > min_us && n_bins > 0);
        let width = (max_us - min_us) / n_bins as f64;
        let bins = (0..n_bins)
            .map(|i| HistogramBin::new(min_us + i as f64 * width, min_us + (i + 1) as f64 * width))
            .collect();
        Self {
            bins,
            underflow: 0,
            overflow: 0,
        }
    }
    /// Records a sample (in microseconds).
    pub fn record(&mut self, sample_us: f64) {
        if let Some(bin) = self.bins.iter_mut().find(|b| b.contains(sample_us)) {
            bin.count += 1;
        } else if sample_us < self.bins.first().map(|b| b.lower_us).unwrap_or(0.0) {
            self.underflow += 1;
        } else {
            self.overflow += 1;
        }
    }
    /// Returns the bin with the highest count.
    pub fn mode_bin(&self) -> Option<&HistogramBin> {
        self.bins.iter().max_by_key(|b| b.count)
    }
    /// Returns the total number of samples recorded.
    pub fn total_samples(&self) -> u64 {
        self.bins.iter().map(|b| b.count).sum::<u64>() + self.underflow + self.overflow
    }
    /// Returns the number of overflow samples.
    pub fn overflow_count(&self) -> u64 {
        self.overflow
    }
    /// Returns the number of underflow samples.
    pub fn underflow_count(&self) -> u64 {
        self.underflow
    }
}
/// The result of a single benchmark run.
#[derive(Debug, Clone)]
pub struct BenchResult {
    /// Name of the benchmark.
    pub name: String,
    /// Total wall-clock duration in milliseconds.
    pub duration_ms: f64,
    /// Number of iterations executed.
    pub iterations: usize,
}
impl BenchResult {
    /// Create a new bench result.
    pub fn new(name: impl Into<String>, duration_ms: f64, iterations: usize) -> Self {
        Self {
            name: name.into(),
            duration_ms,
            iterations,
        }
    }
    /// Average duration per iteration in milliseconds.
    pub fn avg_ms(&self) -> f64 {
        if self.iterations == 0 {
            0.0
        } else {
            self.duration_ms / self.iterations as f64
        }
    }
    /// Average duration per iteration in microseconds.
    pub fn avg_us(&self) -> f64 {
        self.avg_ms() * 1_000.0
    }
    /// Iterations per second.
    pub fn iters_per_sec(&self) -> f64 {
        if self.duration_ms == 0.0 {
            f64::INFINITY
        } else {
            (self.iterations as f64) / (self.duration_ms / 1_000.0)
        }
    }
}
/// Determines when a benchmark has warmed up by detecting stabilisation.
#[allow(dead_code)]
pub struct AdaptiveWarmup {
    window: Vec<f64>,
    window_sz: usize,
    threshold: f64,
    warmed: bool,
}
#[allow(dead_code)]
impl AdaptiveWarmup {
    /// Creates a warmup detector with the given window size and
    /// coefficient-of-variation threshold (e.g., 0.05 = 5 %).
    pub fn new(window_sz: usize, threshold: f64) -> Self {
        Self {
            window: Vec::new(),
            window_sz,
            threshold,
            warmed: false,
        }
    }
    /// Records a sample.  Returns `true` once warmup is detected.
    pub fn record(&mut self, sample: f64) -> bool {
        if self.warmed {
            return true;
        }
        self.window.push(sample);
        if self.window.len() > self.window_sz {
            self.window.remove(0);
        }
        if self.window.len() == self.window_sz {
            let mean = self.window.iter().sum::<f64>() / self.window_sz as f64;
            if mean.abs() > f64::EPSILON {
                let var = self.window.iter().map(|&x| (x - mean).powi(2)).sum::<f64>()
                    / self.window_sz as f64;
                let cv = var.sqrt() / mean;
                if cv < self.threshold {
                    self.warmed = true;
                }
            }
        }
        self.warmed
    }
    /// Returns `true` if the warmup phase is complete.
    pub fn is_warmed(&self) -> bool {
        self.warmed
    }
}
/// Tests how benchmark performance scales with input size.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ScalingTest {
    /// List of (input_size, measured_time_us) pairs.
    pub data_points: Vec<(usize, f64)>,
}
#[allow(dead_code)]
impl ScalingTest {
    /// Creates a new empty scaling test.
    pub fn new() -> Self {
        Self {
            data_points: Vec::new(),
        }
    }
    /// Adds a data point.
    pub fn add_point(&mut self, size: usize, time_us: f64) {
        self.data_points.push((size, time_us));
    }
    /// Estimates the scaling exponent using log-log linear regression.
    ///
    /// Returns `None` if there are fewer than two data points.
    pub fn scaling_exponent(&self) -> Option<f64> {
        if self.data_points.len() < 2 {
            return None;
        }
        let n = self.data_points.len() as f64;
        let xs: Vec<f64> = self
            .data_points
            .iter()
            .map(|(s, _)| (*s as f64).ln())
            .collect();
        let ys: Vec<f64> = self.data_points.iter().map(|(_, t)| t.ln()).collect();
        let xmean = xs.iter().sum::<f64>() / n;
        let ymean = ys.iter().sum::<f64>() / n;
        let num = xs
            .iter()
            .zip(ys.iter())
            .map(|(x, y)| (x - xmean) * (y - ymean))
            .sum::<f64>();
        let den = xs.iter().map(|x| (x - xmean).powi(2)).sum::<f64>();
        if den.abs() < f64::EPSILON {
            return None;
        }
        Some(num / den)
    }
    /// Returns `true` if the estimated scaling exponent is at most `max_exp`.
    pub fn is_at_most_order(&self, max_exp: f64) -> bool {
        self.scaling_exponent().is_some_and(|e| e <= max_exp)
    }
}
/// A lightweight harness that runs a closure and collects timing samples.
#[allow(dead_code)]
pub struct BenchHarnessExt {
    name: String,
    policy: IterationPolicy,
    samples_us: Vec<f64>,
}
#[allow(dead_code)]
impl BenchHarnessExt {
    /// Creates a new harness.
    pub fn new(name: impl Into<String>, policy: IterationPolicy) -> Self {
        Self {
            name: name.into(),
            policy,
            samples_us: Vec::new(),
        }
    }
    /// Runs `f` according to the iteration policy and records samples.
    pub fn run<F: FnMut()>(&mut self, mut f: F) {
        match self.policy {
            IterationPolicy::Fixed(n) => {
                for _ in 0..n {
                    let t = std::time::Instant::now();
                    f();
                    self.samples_us.push(t.elapsed().as_secs_f64() * 1e6);
                }
            }
            IterationPolicy::TimeBounded(ms) => {
                let deadline = std::time::Instant::now() + std::time::Duration::from_millis(ms);
                while std::time::Instant::now() < deadline {
                    let t = std::time::Instant::now();
                    f();
                    self.samples_us.push(t.elapsed().as_secs_f64() * 1e6);
                }
            }
            IterationPolicy::Adaptive { min, max } => {
                let mut count = 0u64;
                while count < max {
                    let t = std::time::Instant::now();
                    f();
                    let elapsed = t.elapsed().as_secs_f64() * 1e6;
                    self.samples_us.push(elapsed);
                    count += 1;
                    if count >= min && elapsed > 1000.0 {
                        break;
                    }
                }
            }
        }
    }
    /// Returns the median sample in microseconds, or `None`.
    pub fn median_us(&self) -> Option<f64> {
        if self.samples_us.is_empty() {
            return None;
        }
        let mut v = self.samples_us.clone();
        v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        Some(v[v.len() / 2])
    }
    /// Returns the number of samples collected.
    pub fn num_samples(&self) -> usize {
        self.samples_us.len()
    }
    /// Returns the name of the benchmark.
    pub fn name(&self) -> &str {
        &self.name
    }
}
/// A serialisable benchmark result suitable for writing to a file.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct BenchResultExt {
    /// Benchmark name.
    pub name: String,
    /// Median latency in microseconds.
    pub median_us: f64,
    /// Mean latency in microseconds.
    pub mean_us: f64,
    /// Standard deviation in microseconds.
    pub stddev_us: f64,
    /// Number of iterations.
    pub iterations: u64,
}
#[allow(dead_code)]
impl BenchResultExt {
    /// Creates a `BenchResultExt` from raw samples.
    pub fn from_samples(name: impl Into<String>, samples: &[f64]) -> Option<Self> {
        if samples.is_empty() {
            return None;
        }
        let n = samples.len() as f64;
        let mean_us = samples.iter().sum::<f64>() / n;
        let var = samples.iter().map(|&x| (x - mean_us).powi(2)).sum::<f64>() / n;
        let stddev_us = var.sqrt();
        let mut sorted = samples.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let median_us = sorted[sorted.len() / 2];
        Some(Self {
            name: name.into(),
            median_us,
            mean_us,
            stddev_us,
            iterations: samples.len() as u64,
        })
    }
    /// Formats the result as a CSV row.
    pub fn to_csv(&self) -> String {
        format!(
            "{},{:.3},{:.3},{:.3},{}",
            self.name, self.median_us, self.mean_us, self.stddev_us, self.iterations
        )
    }
    /// Returns `true` if the coefficient of variation is below `threshold`.
    pub fn is_stable(&self, threshold: f64) -> bool {
        if self.mean_us.abs() < f64::EPSILON {
            return true;
        }
        (self.stddev_us / self.mean_us) < threshold
    }
}
/// Generates deterministic pseudo-random input for fuzz-style benchmarks.
#[allow(dead_code)]
pub struct FuzzInput {
    seed: u64,
}
#[allow(dead_code)]
impl FuzzInput {
    /// Creates a new generator with the given seed.
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }
    /// Returns the next pseudo-random `u64`.
    pub fn next_u64(&mut self) -> u64 {
        self.seed ^= self.seed << 13;
        self.seed ^= self.seed >> 7;
        self.seed ^= self.seed << 17;
        self.seed
    }
    /// Returns the next pseudo-random `usize` in `[0, n)`.
    pub fn next_usize(&mut self, n: usize) -> usize {
        if n == 0 {
            return 0;
        }
        (self.next_u64() as usize) % n
    }
    /// Returns the next pseudo-random `f64` in `[0.0, 1.0)`.
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
    /// Fills `buf` with pseudo-random bytes.
    pub fn fill_bytes(&mut self, buf: &mut [u8]) {
        for chunk in buf.chunks_mut(8) {
            let v = self.next_u64().to_le_bytes();
            let len = chunk.len();
            chunk.copy_from_slice(&v[..len]);
        }
    }
}
/// Unit for expressing throughput measurements.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThroughputUnit {
    /// Operations per second.
    OpsPerSec,
    /// Bytes per second.
    BytesPerSec,
    /// Megabytes per second.
    MBPerSec,
    /// Gigabytes per second.
    GBPerSec,
}
#[allow(dead_code)]
impl ThroughputUnit {
    /// Returns the display label for this unit.
    pub fn label(self) -> &'static str {
        match self {
            ThroughputUnit::OpsPerSec => "ops/s",
            ThroughputUnit::BytesPerSec => "B/s",
            ThroughputUnit::MBPerSec => "MB/s",
            ThroughputUnit::GBPerSec => "GB/s",
        }
    }
    /// Converts `bytes_per_sec` to this unit.
    pub fn from_bytes_per_sec(self, bps: f64) -> f64 {
        match self {
            ThroughputUnit::OpsPerSec => bps,
            ThroughputUnit::BytesPerSec => bps,
            ThroughputUnit::MBPerSec => bps / 1e6,
            ThroughputUnit::GBPerSec => bps / 1e9,
        }
    }
}
/// Comparison result between two benchmark measurements.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CompareResult {
    /// The new measurement is faster by more than the threshold.
    Improvement,
    /// The new measurement is within the threshold of the baseline.
    Neutral,
    /// The new measurement is slower by more than the threshold.
    Regression,
}
/// Groups related benchmarks and runs them together.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct BenchGroup {
    /// Group name.
    pub name: String,
    /// Names of benchmarks in this group.
    pub benchmarks: Vec<String>,
}
#[allow(dead_code)]
impl BenchGroup {
    /// Creates a new empty group.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            benchmarks: Vec::new(),
        }
    }
    /// Adds a benchmark to the group.
    pub fn add(&mut self, bench: impl Into<String>) {
        self.benchmarks.push(bench.into());
    }
    /// Returns the number of benchmarks in the group.
    pub fn size(&self) -> usize {
        self.benchmarks.len()
    }
}
/// Simulates cold-cache conditions by writing random data to flush the cache.
#[allow(dead_code)]
pub struct ColdCacheSimulator {
    buf: Vec<u8>,
}
#[allow(dead_code)]
impl ColdCacheSimulator {
    /// Creates a simulator that allocates a buffer of `size` bytes.
    pub fn new(size: usize) -> Self {
        Self {
            buf: vec![0xABu8; size],
        }
    }
    /// Writes to every cache line to flush them.
    pub fn flush(&mut self) {
        let step = 64;
        let mut sink: u64 = 0;
        for i in (0..self.buf.len()).step_by(step) {
            self.buf[i] = self.buf[i].wrapping_add(1);
            sink = sink.wrapping_add(self.buf[i] as u64);
        }
        std::hint::black_box(sink);
    }
    /// Returns the size of the flush buffer.
    pub fn buffer_size(&self) -> usize {
        self.buf.len()
    }
}
/// A collection of named benchmarks that can be run in sequence.
#[derive(Debug, Default)]
pub struct BenchSuite {
    results: Vec<BenchResult>,
}
impl BenchSuite {
    /// Create a new empty suite.
    pub fn new() -> Self {
        Self::default()
    }
    /// Run a named benchmark for `iterations` iterations and record the result.
    ///
    /// The closure `f` is called `iterations` times; total wall-clock time is measured.
    pub fn run<F: Fn()>(&mut self, name: &str, iterations: usize, f: F) {
        let timer = BenchTimer::start();
        for _ in 0..iterations {
            f();
        }
        let duration_ms = timer.elapsed_ms();
        self.results
            .push(BenchResult::new(name, duration_ms, iterations));
    }
    /// Generate a human-readable report of all recorded results.
    pub fn report(&self) -> String {
        let mut out = String::from("=== Benchmark Suite Report ===\n");
        for r in &self.results {
            out.push_str(&format!("  {}\n", r));
        }
        out.push_str(&format!("Total benchmarks: {}\n", self.results.len()));
        out
    }
    /// Return a slice of all recorded results.
    pub fn results(&self) -> &[BenchResult] {
        &self.results
    }
    /// Clear all recorded results.
    pub fn clear(&mut self) {
        self.results.clear();
    }
    /// Return the number of recorded results.
    pub fn len(&self) -> usize {
        self.results.len()
    }
    /// Return true if no results have been recorded.
    pub fn is_empty(&self) -> bool {
        self.results.is_empty()
    }
}
/// A reusable harness for running benchmarks with configurable warmup and
/// iteration counts.
#[derive(Debug, Clone)]
pub struct BenchHarnessV2 {
    /// Number of warmup iterations (not measured).
    warmup: usize,
    /// Number of measured iterations.
    iterations: usize,
}
impl BenchHarnessV2 {
    /// Create a new harness with the given warmup and iteration counts.
    pub fn new(warmup: usize, iterations: usize) -> Self {
        Self { warmup, iterations }
    }
    /// Run a benchmark, discarding warmup iterations, and return the result.
    pub fn bench<F: Fn()>(&self, name: &str, f: F) -> BenchResult {
        for _ in 0..self.warmup {
            f();
        }
        let timer = BenchTimer::start();
        for _ in 0..self.iterations {
            f();
        }
        let duration_ms = timer.elapsed_ms();
        BenchResult::new(name, duration_ms, self.iterations)
    }
    /// Compute throughput in items per second given a benchmark result and
    /// the number of items processed per iteration.
    ///
    /// Returns `items_per_iter * iters_per_sec`.
    pub fn throughput(&self, result: &BenchResult, items: usize) -> f64 {
        result.iters_per_sec() * items as f64
    }
    /// Return the number of warmup iterations.
    pub fn warmup_count(&self) -> usize {
        self.warmup
    }
    /// Return the number of measured iterations.
    pub fn iteration_count(&self) -> usize {
        self.iterations
    }
}
/// A key-value annotation attached to a benchmark result.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct BenchAnnotation {
    /// The annotation key.
    pub key: String,
    /// The annotation value.
    pub value: String,
}
#[allow(dead_code)]
impl BenchAnnotation {
    /// Creates a new annotation.
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }
}
/// Decides how many iterations to run for a benchmark.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IterationPolicy {
    /// Run a fixed number of iterations.
    Fixed(u64),
    /// Run until the elapsed time exceeds the given number of milliseconds.
    TimeBounded(u64),
    /// Run at least `min` and at most `max` iterations.
    Adaptive {
        /// Minimum number of iterations.
        min: u64,
        /// Maximum number of iterations.
        max: u64,
    },
}
#[allow(dead_code)]
impl IterationPolicy {
    /// Returns the minimum number of iterations for this policy.
    pub fn min_iters(self) -> u64 {
        match self {
            IterationPolicy::Fixed(n) => n,
            IterationPolicy::TimeBounded(_) => 1,
            IterationPolicy::Adaptive { min, .. } => min,
        }
    }
    /// Returns `true` if time-bounded stopping should be used.
    pub fn is_time_bounded(self) -> bool {
        matches!(self, IterationPolicy::TimeBounded(_))
    }
}
/// A matrix of benchmark results indexed by (row_label, col_label).
#[allow(dead_code)]
pub struct BenchMatrix {
    row_labels: Vec<String>,
    col_labels: Vec<String>,
    data: Vec<Vec<Option<f64>>>,
}
#[allow(dead_code)]
impl BenchMatrix {
    /// Creates an empty benchmark matrix.
    pub fn new() -> Self {
        Self {
            row_labels: Vec::new(),
            col_labels: Vec::new(),
            data: Vec::new(),
        }
    }
    /// Adds a row label and returns its index.
    pub fn add_row(&mut self, label: impl Into<String>) -> usize {
        let idx = self.row_labels.len();
        self.row_labels.push(label.into());
        self.data.push(vec![None; self.col_labels.len()]);
        idx
    }
    /// Adds a column label and returns its index.
    pub fn add_col(&mut self, label: impl Into<String>) -> usize {
        let idx = self.col_labels.len();
        self.col_labels.push(label.into());
        for row in self.data.iter_mut() {
            row.push(None);
        }
        idx
    }
    /// Sets the value at `(row, col)`.
    pub fn set(&mut self, row: usize, col: usize, val: f64) {
        if let Some(r) = self.data.get_mut(row) {
            if let Some(slot) = r.get_mut(col) {
                *slot = Some(val);
            }
        }
    }
    /// Gets the value at `(row, col)`, or `None`.
    pub fn get(&self, row: usize, col: usize) -> Option<f64> {
        self.data.get(row)?.get(col)?.as_ref().copied()
    }
    /// Returns the number of rows.
    pub fn num_rows(&self) -> usize {
        self.row_labels.len()
    }
    /// Returns the number of columns.
    pub fn num_cols(&self) -> usize {
        self.col_labels.len()
    }
    /// Formats the matrix as a Markdown table.
    pub fn to_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("| |");
        for c in &self.col_labels {
            out.push_str(&format!(" {} |", c));
        }
        out.push('\n');
        out.push_str("|---|");
        for _ in &self.col_labels {
            out.push_str("---|");
        }
        out.push('\n');
        for (ri, rl) in self.row_labels.iter().enumerate() {
            out.push_str(&format!("| {} |", rl));
            for ci in 0..self.col_labels.len() {
                match self.get(ri, ci) {
                    Some(v) => out.push_str(&format!(" {:.3} |", v)),
                    None => out.push_str(" - |"),
                }
            }
            out.push('\n');
        }
        out
    }
}
/// A minimal profiler that measures CPU instructions and cache misses (stub).
#[allow(dead_code)]
pub struct BenchProfiler {
    label: String,
    start_time: std::time::Instant,
}
#[allow(dead_code)]
impl BenchProfiler {
    /// Creates and starts a new profiler.
    pub fn start(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            start_time: std::time::Instant::now(),
        }
    }
    /// Stops the profiler and returns elapsed microseconds.
    pub fn stop(self) -> f64 {
        self.start_time.elapsed().as_secs_f64() * 1e6
    }
    /// Returns the profiler label.
    pub fn label(&self) -> &str {
        &self.label
    }
}
/// A plan that specifies which benchmarks to run and in what order.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct BenchPlan {
    /// Ordered list of benchmark names.
    pub order: Vec<String>,
    /// Whether to shuffle the order.
    pub shuffle: bool,
    /// Number of warmup iterations per benchmark.
    pub warmup_iters: u64,
    /// Number of measurement iterations per benchmark.
    pub measure_iters: u64,
}
#[allow(dead_code)]
impl BenchPlan {
    /// Creates a default plan (no shuffle, 3 warmup, 10 measure).
    pub fn default_plan() -> Self {
        Self {
            order: Vec::new(),
            shuffle: false,
            warmup_iters: 3,
            measure_iters: 10,
        }
    }
    /// Adds a benchmark to the plan.
    pub fn add(&mut self, name: impl Into<String>) {
        self.order.push(name.into());
    }
    /// Returns the number of benchmarks in the plan.
    pub fn len(&self) -> usize {
        self.order.len()
    }
    /// Returns `true` if the plan is empty.
    pub fn is_empty(&self) -> bool {
        self.order.is_empty()
    }
    /// Reverses the benchmark order.
    pub fn reverse(&mut self) {
        self.order.reverse();
    }
}
/// Measures the average time per item when processing a batch.
#[allow(dead_code)]
pub struct BatchTimer {
    start: std::time::Instant,
    batch_size: u64,
}
#[allow(dead_code)]
impl BatchTimer {
    /// Starts a batch timer for `batch_size` items.
    pub fn start(batch_size: u64) -> Self {
        Self {
            start: std::time::Instant::now(),
            batch_size,
        }
    }
    /// Returns elapsed nanoseconds per item.
    pub fn ns_per_item(&self) -> f64 {
        if self.batch_size == 0 {
            return 0.0;
        }
        self.start.elapsed().as_nanos() as f64 / self.batch_size as f64
    }
    /// Returns total elapsed microseconds.
    pub fn total_us(&self) -> f64 {
        self.start.elapsed().as_secs_f64() * 1e6
    }
}
/// Ordinary least squares regression for benchmark trend analysis.
#[allow(dead_code)]
pub struct OlsRegression {
    data: Vec<(f64, f64)>,
}
#[allow(dead_code)]
impl OlsRegression {
    /// Creates a new regression with no data points.
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
    /// Adds a data point.
    pub fn add(&mut self, x: f64, y: f64) {
        self.data.push((x, y));
    }
    /// Fits a line `y = a + b*x` and returns `(intercept, slope)`, or `None`.
    pub fn fit(&self) -> Option<(f64, f64)> {
        let n = self.data.len() as f64;
        if n < 2.0 {
            return None;
        }
        let sx = self.data.iter().map(|(x, _)| x).sum::<f64>();
        let sy = self.data.iter().map(|(_, y)| y).sum::<f64>();
        let sxx = self.data.iter().map(|(x, _)| x * x).sum::<f64>();
        let sxy = self.data.iter().map(|(x, y)| x * y).sum::<f64>();
        let denom = n * sxx - sx * sx;
        if denom.abs() < f64::EPSILON {
            return None;
        }
        let slope = (n * sxy - sx * sy) / denom;
        let intercept = (sy - slope * sx) / n;
        Some((intercept, slope))
    }
    /// Predicts `y` for a given `x`, or `None` if the fit fails.
    pub fn predict(&self, x: f64) -> Option<f64> {
        let (a, b) = self.fit()?;
        Some(a + b * x)
    }
    /// Returns the number of data points.
    pub fn len(&self) -> usize {
        self.data.len()
    }
    /// Returns `true` if there are no data points.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
/// A simple text-mode progress bar for bench output.
#[allow(dead_code)]
pub struct ProgressBar {
    total: usize,
    done: usize,
    width: usize,
}
#[allow(dead_code)]
impl ProgressBar {
    /// Creates a progress bar with `total` steps and `width` characters.
    pub fn new(total: usize, width: usize) -> Self {
        Self {
            total,
            done: 0,
            width,
        }
    }
    /// Advances the progress by one step.
    pub fn step(&mut self) {
        if self.done < self.total {
            self.done += 1;
        }
    }
    /// Returns the progress fraction (0.0 – 1.0).
    pub fn fraction(&self) -> f64 {
        if self.total == 0 {
            return 1.0;
        }
        self.done as f64 / self.total as f64
    }
    /// Renders the progress bar as a string.
    pub fn render(&self) -> String {
        let filled = ((self.fraction() * self.width as f64) as usize).min(self.width);
        let bar: String = "#".repeat(filled) + &"-".repeat(self.width - filled);
        format!("[{}] {}/{}", bar, self.done, self.total)
    }
    /// Returns `true` if all steps are done.
    pub fn is_complete(&self) -> bool {
        self.done >= self.total
    }
}
/// A collection of named metrics from a single benchmark result.
#[allow(dead_code)]
pub struct MetricSet {
    metrics: Vec<Metric>,
}
#[allow(dead_code)]
impl MetricSet {
    /// Creates an empty metric set.
    pub fn new() -> Self {
        Self {
            metrics: Vec::new(),
        }
    }
    /// Adds a metric.
    pub fn add(&mut self, name: impl Into<String>, value: f64, unit: impl Into<String>) {
        self.metrics.push(Metric::new(name, value, unit));
    }
    /// Returns the value of the named metric, or `None`.
    pub fn get(&self, name: &str) -> Option<f64> {
        self.metrics
            .iter()
            .find(|m| m.name == name)
            .map(|m| m.value)
    }
    /// Returns all metrics as a formatted string.
    pub fn display_all(&self) -> String {
        self.metrics
            .iter()
            .map(|m| m.display())
            .collect::<Vec<_>>()
            .join(", ")
    }
    /// Returns the number of metrics.
    pub fn len(&self) -> usize {
        self.metrics.len()
    }
    /// Returns `true` if there are no metrics.
    pub fn is_empty(&self) -> bool {
        self.metrics.is_empty()
    }
}
/// A time-stamped event log for a benchmark run.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct BenchEventLog {
    /// List of (elapsed_ms, event_name) pairs.
    pub events: Vec<(f64, String)>,
    start: std::time::Instant,
}
#[allow(dead_code)]
impl BenchEventLog {
    /// Creates a new event log, starting the clock.
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            start: std::time::Instant::now(),
        }
    }
    /// Records an event with the current elapsed time.
    pub fn record(&mut self, name: impl Into<String>) {
        let elapsed = self.start.elapsed().as_secs_f64() * 1000.0;
        self.events.push((elapsed, name.into()));
    }
    /// Returns the number of recorded events.
    pub fn count(&self) -> usize {
        self.events.len()
    }
    /// Returns the elapsed time since the last recorded event, or since start.
    pub fn since_last_ms(&self) -> f64 {
        let now = self.start.elapsed().as_secs_f64() * 1000.0;
        self.events.last().map_or(now, |(t, _)| now - t)
    }
}
/// A filter that selects benchmarks by name pattern.
#[allow(dead_code)]
pub struct BenchFilter {
    includes: Vec<String>,
    excludes: Vec<String>,
}
#[allow(dead_code)]
impl BenchFilter {
    /// Creates a new filter with no rules (accepts everything).
    pub fn new() -> Self {
        Self {
            includes: Vec::new(),
            excludes: Vec::new(),
        }
    }
    /// Adds an include pattern.
    pub fn include(&mut self, pat: impl Into<String>) {
        self.includes.push(pat.into());
    }
    /// Adds an exclude pattern.
    pub fn exclude(&mut self, pat: impl Into<String>) {
        self.excludes.push(pat.into());
    }
    /// Returns `true` if `name` passes the filter.
    pub fn accepts(&self, name: &str) -> bool {
        if !self.includes.is_empty() && !self.includes.iter().any(|p| name.contains(p.as_str())) {
            return false;
        }
        !self.excludes.iter().any(|p| name.contains(p.as_str()))
    }
}
