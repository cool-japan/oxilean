//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use std::collections::HashMap as BenchHashMap;
use std::time::{Duration, Instant};

use oxilean_kernel::Name;
use std::collections::HashMap;

/// Runs benchmarks according to a configuration.
#[derive(Clone, Debug)]
pub struct BenchRunner {
    /// Benchmark configuration.
    config: BenchmarkConfig,
}
impl BenchRunner {
    /// Create a runner with the given configuration.
    pub fn new(config: BenchmarkConfig) -> Self {
        Self { config }
    }
    /// Create a runner with default configuration.
    pub fn with_defaults() -> Self {
        Self::new(BenchmarkConfig::default())
    }
    /// Run a single benchmark.
    ///
    /// The provided closure is called `warmup_iterations + measure_iterations`
    /// times; only the last `measure_iterations` are measured.
    pub fn run_benchmark<F>(&self, name: &str, mut f: F) -> BenchmarkResult
    where
        F: FnMut(),
    {
        self.warmup(&mut f);
        let timings = self.measure(&mut f);
        compute_statistics(name, &timings)
    }
    /// Run an entire suite of benchmarks.
    pub fn run_suite<F>(&self, suite_name: &str, benchmarks: Vec<(&str, F)>) -> BenchmarkSuite
    where
        F: FnMut(),
    {
        let mut suite = BenchmarkSuite::new(suite_name);
        for (name, mut bench_fn) in benchmarks {
            let result = self.run_benchmark(name, &mut bench_fn);
            suite.add(result);
        }
        suite
    }
    /// Perform warmup iterations.
    fn warmup<F: FnMut()>(&self, f: &mut F) {
        for _ in 0..self.config.warmup_iterations {
            f();
        }
    }
    /// Measure individual iterations and return their durations.
    fn measure<F: FnMut()>(&self, f: &mut F) -> Vec<Duration> {
        let overall_start = Instant::now();
        let mut timings = Vec::with_capacity(self.config.measure_iterations as usize);
        for _ in 0..self.config.measure_iterations {
            if overall_start.elapsed() > self.config.timeout {
                break;
            }
            let start = Instant::now();
            f();
            timings.push(start.elapsed());
        }
        timings
    }
}
#[allow(dead_code)]
pub struct BenchCapabilities {
    pub supports_warmup: bool,
    pub supports_parallel: bool,
    pub supports_cpu_pinning: bool,
    pub supports_memory_profiling: bool,
    pub supports_flamegraph: bool,
    pub max_duration_secs: u64,
}
/// A single entry in benchmark history.
#[derive(Debug, Clone)]
pub struct BenchHistoryEntry {
    /// Label for this run (e.g., commit hash or date).
    pub run_label: String,
    /// Mean time in nanoseconds.
    pub mean_nanos: u64,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BaselineEntry {
    pub name: String,
    pub mean_ns: f64,
    pub stddev_ns: f64,
    pub sample_count: usize,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TaggedResult {
    pub name: String,
    pub tags: Vec<BenchTag>,
    pub mean_ns: f64,
    pub stddev_ns: f64,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BenchRun {
    pub name: String,
    pub samples_ns: Vec<u64>,
    pub warmup_count: usize,
}
#[allow(dead_code)]
impl BenchRun {
    pub fn new(name: &str, warmup_count: usize) -> Self {
        Self {
            name: name.to_string(),
            samples_ns: Vec::new(),
            warmup_count,
        }
    }
    pub fn add_sample(&mut self, ns: u64) {
        self.samples_ns.push(ns);
    }
    pub fn mean_ns(&self) -> f64 {
        if self.samples_ns.is_empty() {
            return 0.0;
        }
        self.samples_ns.iter().map(|&s| s as f64).sum::<f64>() / self.samples_ns.len() as f64
    }
    pub fn to_report_entry(&self) -> BenchReportEntry {
        let mut sorted = self.samples_ns.clone();
        sorted.sort_unstable();
        let min = sorted.first().copied().unwrap_or(0) as f64;
        let max = sorted.last().copied().unwrap_or(0) as f64;
        let mean = self.mean_ns();
        let n = self.samples_ns.len() as f64;
        let variance = self
            .samples_ns
            .iter()
            .map(|&s| {
                let d = s as f64 - mean;
                d * d
            })
            .sum::<f64>()
            / n.max(1.0);
        BenchReportEntry {
            name: self.name.clone(),
            mean_ns: mean,
            stddev_ns: variance.sqrt(),
            min_ns: min,
            max_ns: max,
            sample_count: self.samples_ns.len(),
            baseline_diff_pct: None,
        }
    }
}
#[allow(dead_code)]
pub struct SampleCollector {
    samples: Vec<f64>,
}
#[allow(dead_code)]
impl SampleCollector {
    pub fn new() -> Self {
        Self {
            samples: Vec::new(),
        }
    }
    pub fn add(&mut self, sample: f64) {
        self.samples.push(sample);
    }
    pub fn remove_outliers_iqr(&mut self) {
        if self.samples.len() < 4 {
            return;
        }
        let mut sorted = self.samples.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let n = sorted.len();
        let q1 = sorted[n / 4];
        let q3 = sorted[n * 3 / 4];
        let iqr = q3 - q1;
        let lo = q1 - 1.5 * iqr;
        let hi = q3 + 1.5 * iqr;
        self.samples.retain(|&x| x >= lo && x <= hi);
    }
    pub fn mean(&self) -> f64 {
        if self.samples.is_empty() {
            return 0.0;
        }
        self.samples.iter().sum::<f64>() / self.samples.len() as f64
    }
    pub fn count(&self) -> usize {
        self.samples.len()
    }
    pub fn min(&self) -> f64 {
        self.samples.iter().cloned().fold(f64::INFINITY, f64::min)
    }
    pub fn max(&self) -> f64 {
        self.samples
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max)
    }
}
#[allow(dead_code)]
pub struct BenchSpecRegistry {
    specs: std::collections::HashMap<String, BenchSpec>,
}
#[allow(dead_code)]
impl BenchSpecRegistry {
    pub fn new() -> Self {
        Self {
            specs: std::collections::HashMap::new(),
        }
    }
    pub fn register(&mut self, spec: BenchSpec) {
        self.specs.insert(spec.name.clone(), spec);
    }
    pub fn get(&self, name: &str) -> Option<&BenchSpec> {
        self.specs.get(name)
    }
    pub fn all_names(&self) -> Vec<&str> {
        self.specs.keys().map(|s| s.as_str()).collect()
    }
    pub fn count(&self) -> usize {
        self.specs.len()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BenchEnv {
    pub os: String,
    pub arch: String,
    pub cpu_count: usize,
    pub rustc_version: String,
    pub timestamp: String,
}
#[allow(dead_code)]
impl BenchEnv {
    pub fn current() -> Self {
        Self {
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            cpu_count: num_cpus_estimate(),
            rustc_version: "unknown".to_string(),
            timestamp: "2026-02-28".to_string(),
        }
    }
    pub fn to_json(&self) -> String {
        format!(
            "{{\"os\":\"{}\",\"arch\":\"{}\",\"cpus\":{},\"rustc\":\"{}\",\"ts\":\"{}\"}}",
            self.os, self.arch, self.cpu_count, self.rustc_version, self.timestamp
        )
    }
}
/// Result of running a single benchmark.
#[derive(Clone, Debug)]
pub struct BenchmarkResult {
    /// Human-readable name of the benchmark.
    pub name: String,
    /// Number of measured iterations.
    pub iterations: u64,
    /// Total wall-clock time across all iterations.
    pub total_time: Duration,
    /// Fastest single iteration.
    pub min_time: Duration,
    /// Slowest single iteration.
    pub max_time: Duration,
    /// Arithmetic mean time per iteration.
    pub mean_time: Duration,
    /// Standard deviation (in nanoseconds, stored as Duration).
    pub std_dev: Duration,
}
impl BenchmarkResult {
    /// Throughput in iterations per second.
    pub fn throughput(&self) -> f64 {
        let secs = self.total_time.as_secs_f64();
        if secs == 0.0 {
            return 0.0;
        }
        self.iterations as f64 / secs
    }
    /// Coefficient of variation (std_dev / mean) as a percentage.
    pub fn cv_percent(&self) -> f64 {
        let mean_ns = self.mean_time.as_nanos() as f64;
        if mean_ns == 0.0 {
            return 0.0;
        }
        let std_ns = self.std_dev.as_nanos() as f64;
        (std_ns / mean_ns) * 100.0
    }
}
#[allow(dead_code)]
pub struct OpsCounter {
    ops: u64,
    elapsed: std::time::Duration,
}
#[allow(dead_code)]
impl OpsCounter {
    pub fn new() -> Self {
        Self {
            ops: 0,
            elapsed: std::time::Duration::ZERO,
        }
    }
    pub fn add_ops(&mut self, count: u64, elapsed: std::time::Duration) {
        self.ops += count;
        self.elapsed += elapsed;
    }
    pub fn ops_per_sec(&self) -> f64 {
        let secs = self.elapsed.as_secs_f64();
        if secs == 0.0 {
            return f64::INFINITY;
        }
        self.ops as f64 / secs
    }
    pub fn ns_per_op(&self) -> f64 {
        if self.ops == 0 {
            return f64::INFINITY;
        }
        self.elapsed.as_nanos() as f64 / self.ops as f64
    }
}
/// Persists benchmark history across multiple runs for trend analysis.
#[derive(Debug, Clone, Default)]
pub struct BenchHistory {
    entries: BenchHashMap<String, Vec<BenchHistoryEntry>>,
}
impl BenchHistory {
    /// Create a new empty history.
    pub fn new() -> Self {
        BenchHistory {
            entries: BenchHashMap::new(),
        }
    }
    /// Record a result for a benchmark.
    pub fn record(&mut self, bench_name: &str, run_label: &str, mean_nanos: u64) {
        self.entries
            .entry(bench_name.to_string())
            .or_default()
            .push(BenchHistoryEntry {
                run_label: run_label.to_string(),
                mean_nanos,
            });
    }
    /// Get all history entries for a named benchmark.
    pub fn entries_for(&self, bench_name: &str) -> &[BenchHistoryEntry] {
        self.entries
            .get(bench_name)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }
    /// Compute the linear trend slope (nanoseconds per entry) for a benchmark.
    ///
    /// Returns `None` if fewer than 2 data points are available.
    pub fn trend_slope(&self, bench_name: &str) -> Option<f64> {
        let entries = self.entries_for(bench_name);
        if entries.len() < 2 {
            return None;
        }
        let n = entries.len() as f64;
        let xs: Vec<f64> = (0..entries.len()).map(|i| i as f64).collect();
        let ys: Vec<f64> = entries.iter().map(|e| e.mean_nanos as f64).collect();
        let sum_x: f64 = xs.iter().sum();
        let sum_y: f64 = ys.iter().sum();
        let sum_xy: f64 = xs.iter().zip(ys.iter()).map(|(x, y)| x * y).sum();
        let sum_xx: f64 = xs.iter().map(|x| x * x).sum();
        let denom = n * sum_xx - sum_x * sum_x;
        if denom.abs() < f64::EPSILON {
            return None;
        }
        Some((n * sum_xy - sum_x * sum_y) / denom)
    }
    /// Return all benchmark names that have history entries.
    pub fn bench_names(&self) -> Vec<&str> {
        self.entries.keys().map(String::as_str).collect()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BenchTag(pub String);
#[allow(dead_code)]
impl BenchTag {
    pub fn new(s: &str) -> Self {
        Self(s.to_string())
    }
}
/// Builder for `BenchmarkConfig`.
#[derive(Debug, Default)]
pub struct BenchConfigBuilder {
    warmup_iters: Option<usize>,
    measurement_iters: Option<usize>,
    time_limit: Option<Duration>,
    verbose: Option<bool>,
    filter: Option<String>,
}
impl BenchConfigBuilder {
    /// Create a new builder with all defaults.
    pub fn new() -> Self {
        BenchConfigBuilder::default()
    }
    /// Set the number of warmup iterations.
    pub fn warmup_iters(mut self, n: usize) -> Self {
        self.warmup_iters = Some(n);
        self
    }
    /// Set the number of measurement iterations.
    pub fn measurement_iters(mut self, n: usize) -> Self {
        self.measurement_iters = Some(n);
        self
    }
    /// Set the time limit per benchmark.
    pub fn time_limit(mut self, d: Duration) -> Self {
        self.time_limit = Some(d);
        self
    }
    /// Enable or disable verbose output.
    pub fn verbose(mut self, v: bool) -> Self {
        self.verbose = Some(v);
        self
    }
    /// Set a filter string for benchmark names.
    pub fn filter(mut self, f: impl Into<String>) -> Self {
        self.filter = Some(f.into());
        self
    }
    /// Build the `BenchmarkConfig`.
    pub fn build(self) -> BenchmarkConfig {
        let wi = self.warmup_iters.unwrap_or(3);
        let mi = self.measurement_iters.unwrap_or(10);
        let tl = self.time_limit.unwrap_or(Duration::from_secs(30));
        let v = self.verbose.unwrap_or(false);
        BenchmarkConfig {
            warmup_iterations: wi as u64,
            measure_iterations: mi as u64,
            timeout: tl,
            warmup_iters: wi,
            measurement_iters: mi,
            verbose: v,
            filter: self.filter,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SingleComparison {
    pub bench_name: String,
    pub baseline_mean: f64,
    pub candidate_mean: f64,
    pub ratio: f64,
    pub is_regression: bool,
}
#[allow(dead_code)]
pub struct BenchComparisonV2 {
    pub baseline_name: String,
    pub candidate_name: String,
    pub comparisons: Vec<SingleComparison>,
}
#[allow(dead_code)]
impl BenchComparisonV2 {
    pub fn new(baseline: &str, candidate: &str) -> Self {
        Self {
            baseline_name: baseline.to_string(),
            candidate_name: candidate.to_string(),
            comparisons: Vec::new(),
        }
    }
    pub fn add(
        &mut self,
        bench_name: &str,
        baseline_mean: f64,
        candidate_mean: f64,
        threshold_pct: f64,
    ) {
        let ratio = if baseline_mean == 0.0 {
            1.0
        } else {
            candidate_mean / baseline_mean
        };
        let is_regression = ratio > 1.0 + threshold_pct / 100.0;
        self.comparisons.push(SingleComparison {
            bench_name: bench_name.to_string(),
            baseline_mean,
            candidate_mean,
            ratio,
            is_regression,
        });
    }
    pub fn regressions(&self) -> Vec<&SingleComparison> {
        self.comparisons
            .iter()
            .filter(|c| c.is_regression)
            .collect()
    }
    pub fn improvements(&self) -> Vec<&SingleComparison> {
        self.comparisons.iter().filter(|c| c.ratio < 1.0).collect()
    }
    pub fn to_table(&self) -> String {
        let mut out = format!(
            "{:<30} {:>12} {:>12} {:>8}\n",
            "Benchmark", self.baseline_name, self.candidate_name, "Ratio"
        );
        out.push_str(&"-".repeat(66));
        out.push('\n');
        for c in &self.comparisons {
            let marker = if c.is_regression {
                "regression"
            } else if c.ratio < 1.0 {
                "improvement"
            } else {
                "stable"
            };
            out.push_str(&format!(
                "{:<30} {:>12.1} {:>12.1} {:>7.2}x {}\n",
                c.bench_name, c.baseline_mean, c.candidate_mean, c.ratio, marker
            ));
        }
        out
    }
}
#[allow(dead_code)]
pub struct BenchSpec {
    pub name: String,
    pub description: String,
    pub warmup_iters: usize,
    pub measure_iters: usize,
    pub timeout_ms: Option<u64>,
}
#[allow(dead_code)]
impl BenchSpec {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            warmup_iters: 3,
            measure_iters: 10,
            timeout_ms: None,
        }
    }
    pub fn with_warmup(mut self, n: usize) -> Self {
        self.warmup_iters = n;
        self
    }
    pub fn with_iters(mut self, n: usize) -> Self {
        self.measure_iters = n;
        self
    }
    pub fn with_timeout_ms(mut self, ms: u64) -> Self {
        self.timeout_ms = Some(ms);
        self
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BenchAnnotation {
    pub bench_name: String,
    pub note: String,
    pub severity: AnnotationSeverity,
}
#[allow(dead_code)]
impl BenchAnnotation {
    pub fn new(bench_name: &str, note: &str, severity: AnnotationSeverity) -> Self {
        Self {
            bench_name: bench_name.to_string(),
            note: note.to_string(),
            severity,
        }
    }
    pub fn info(bench_name: &str, note: &str) -> Self {
        Self::new(bench_name, note, AnnotationSeverity::Info)
    }
    pub fn warning(bench_name: &str, note: &str) -> Self {
        Self::new(bench_name, note, AnnotationSeverity::Warning)
    }
    pub fn critical(bench_name: &str, note: &str) -> Self {
        Self::new(bench_name, note, AnnotationSeverity::Critical)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BenchRunConfig {
    pub output_dir: std::path::PathBuf,
    pub save_results: bool,
    pub compare_with_baseline: bool,
    pub regression_threshold_pct: f64,
    pub verbose: bool,
    pub filter: Option<String>,
}
/// Aggregate result across multiple benchmark suites.
#[derive(Debug, Clone)]
pub struct AggregateResult {
    /// Total number of benchmarks across all suites.
    pub total_benchmarks: usize,
    /// Total combined time across all benchmarks.
    pub total_time: Duration,
    /// Mean time per benchmark.
    pub mean_per_benchmark: Duration,
    /// Names of benchmarks that may be regressions.
    pub potential_regressions: Vec<String>,
}
#[allow(dead_code)]
pub struct NsHistogram {
    buckets: Vec<usize>,
    bucket_size_ns: u64,
    min_ns: u64,
}
#[allow(dead_code)]
impl NsHistogram {
    pub fn new(min_ns: u64, max_ns: u64, num_buckets: usize) -> Self {
        let bucket_size_ns = ((max_ns - min_ns) / num_buckets as u64).max(1);
        Self {
            buckets: vec![0; num_buckets],
            bucket_size_ns,
            min_ns,
        }
    }
    pub fn add(&mut self, ns: u64) {
        if ns < self.min_ns {
            return;
        }
        let idx = ((ns - self.min_ns) / self.bucket_size_ns) as usize;
        let idx = idx.min(self.buckets.len() - 1);
        self.buckets[idx] += 1;
    }
    pub fn render(&self) -> String {
        let max_count = *self.buckets.iter().max().unwrap_or(&1);
        let max_count = max_count.max(1);
        let mut out = String::new();
        for (i, &count) in self.buckets.iter().enumerate() {
            let bar_len = count * 40 / max_count;
            let lo = self.min_ns + i as u64 * self.bucket_size_ns;
            let hi = lo + self.bucket_size_ns;
            out.push_str(&format!(
                "{:6}ns-{:6}ns |{} {}\n",
                lo,
                hi,
                "#".repeat(bar_len),
                count
            ));
        }
        out
    }
    pub fn total_count(&self) -> usize {
        self.buckets.iter().sum()
    }
}
#[allow(dead_code)]
pub struct MultiRunAggregator {
    runs: Vec<Vec<f64>>,
    labels: Vec<String>,
}
#[allow(dead_code)]
impl MultiRunAggregator {
    pub fn new() -> Self {
        Self {
            runs: Vec::new(),
            labels: Vec::new(),
        }
    }
    pub fn add_run(&mut self, label: &str, samples: Vec<f64>) {
        self.labels.push(label.to_string());
        self.runs.push(samples);
    }
    pub fn mean_per_run(&self) -> Vec<f64> {
        self.runs
            .iter()
            .map(|r| {
                if r.is_empty() {
                    return 0.0;
                }
                r.iter().sum::<f64>() / r.len() as f64
            })
            .collect()
    }
    pub fn best_run_idx(&self) -> Option<usize> {
        let means = self.mean_per_run();
        means
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
    }
    pub fn worst_run_idx(&self) -> Option<usize> {
        let means = self.mean_per_run();
        means
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
    }
    pub fn overall_mean(&self) -> f64 {
        let all: Vec<f64> = self.runs.iter().flat_map(|r| r.iter().copied()).collect();
        if all.is_empty() {
            return 0.0;
        }
        all.iter().sum::<f64>() / all.len() as f64
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BenchReportEntry {
    pub name: String,
    pub mean_ns: f64,
    pub stddev_ns: f64,
    pub min_ns: f64,
    pub max_ns: f64,
    pub sample_count: usize,
    pub baseline_diff_pct: Option<f64>,
}
#[allow(dead_code)]
pub struct BenchReport {
    pub entries: Vec<BenchReportEntry>,
}
#[allow(dead_code)]
impl BenchReport {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    pub fn add(&mut self, entry: BenchReportEntry) {
        self.entries.push(entry);
    }
    pub fn to_table(&self) -> String {
        let mut out = format!(
            "{:<30} {:>10} {:>10} {:>10} {:>10} {:>8}\n",
            "Name", "Mean(ns)", "Stddev", "Min(ns)", "Max(ns)", "Samples"
        );
        out.push_str(&"-".repeat(82));
        out.push('\n');
        for e in &self.entries {
            let diff = e
                .baseline_diff_pct
                .map(|d| format!("{:+.1}%", d))
                .unwrap_or_else(|| "N/A".to_string());
            out.push_str(&format!(
                "{:<30} {:>10.1} {:>10.1} {:>10.1} {:>10.1} {:>8} {}\n",
                e.name, e.mean_ns, e.stddev_ns, e.min_ns, e.max_ns, e.sample_count, diff
            ));
        }
        out
    }
    pub fn to_csv(&self) -> String {
        let mut out =
            String::from("name,mean_ns,stddev_ns,min_ns,max_ns,samples,baseline_diff_pct\n");
        for e in &self.entries {
            let diff = e
                .baseline_diff_pct
                .map(|d| format!("{:.2}", d))
                .unwrap_or_default();
            out.push_str(&format!(
                "{},{:.1},{:.1},{:.1},{:.1},{},{}\n",
                e.name, e.mean_ns, e.stddev_ns, e.min_ns, e.max_ns, e.sample_count, diff
            ));
        }
        out
    }
    pub fn regressions(&self, threshold_pct: f64) -> Vec<&BenchReportEntry> {
        self.entries
            .iter()
            .filter(|e| {
                e.baseline_diff_pct
                    .map(|d| d > threshold_pct)
                    .unwrap_or(false)
            })
            .collect()
    }
}
#[allow(dead_code)]
pub struct BenchPercentileTracker {
    samples: Vec<u64>,
    sorted: bool,
}
#[allow(dead_code)]
impl BenchPercentileTracker {
    pub fn new() -> Self {
        Self {
            samples: Vec::new(),
            sorted: false,
        }
    }
    pub fn add(&mut self, sample: u64) {
        self.samples.push(sample);
        self.sorted = false;
    }
    fn ensure_sorted(&mut self) {
        if !self.sorted {
            self.samples.sort_unstable();
            self.sorted = true;
        }
    }
    pub fn p50(&mut self) -> Option<u64> {
        self.ensure_sorted();
        let n = self.samples.len();
        if n == 0 {
            return None;
        }
        Some(self.samples[n / 2])
    }
    pub fn p90(&mut self) -> Option<u64> {
        self.ensure_sorted();
        let n = self.samples.len();
        if n == 0 {
            return None;
        }
        Some(self.samples[n * 9 / 10])
    }
    pub fn p95(&mut self) -> Option<u64> {
        self.ensure_sorted();
        let n = self.samples.len();
        if n == 0 {
            return None;
        }
        Some(self.samples[n * 95 / 100])
    }
    pub fn p99(&mut self) -> Option<u64> {
        self.ensure_sorted();
        let n = self.samples.len();
        if n == 0 {
            return None;
        }
        Some(self.samples[(n * 99 / 100).min(n - 1)])
    }
    pub fn min(&self) -> Option<u64> {
        self.samples.iter().copied().min()
    }
    pub fn max(&self) -> Option<u64> {
        self.samples.iter().copied().max()
    }
    pub fn mean(&self) -> f64 {
        if self.samples.is_empty() {
            return 0.0;
        }
        self.samples.iter().map(|&s| s as f64).sum::<f64>() / self.samples.len() as f64
    }
    pub fn count(&self) -> usize {
        self.samples.len()
    }
}
#[allow(dead_code)]
pub struct FlameStack {
    frames: Vec<String>,
    counts: std::collections::HashMap<String, u64>,
}
#[allow(dead_code)]
impl FlameStack {
    pub fn new() -> Self {
        Self {
            frames: Vec::new(),
            counts: std::collections::HashMap::new(),
        }
    }
    pub fn push_frame(&mut self, frame: &str) {
        self.frames.push(frame.to_string());
    }
    pub fn pop_frame(&mut self) {
        self.frames.pop();
    }
    pub fn sample(&mut self) {
        let stack = self.frames.join(";");
        *self.counts.entry(stack).or_default() += 1;
    }
    pub fn to_flamegraph_lines(&self) -> Vec<String> {
        self.counts
            .iter()
            .map(|(stack, count)| format!("{} {}", stack, count))
            .collect()
    }
    pub fn total_samples(&self) -> u64 {
        self.counts.values().sum()
    }
}
/// A named collection of benchmark results.
#[derive(Clone, Debug)]
pub struct BenchmarkSuite {
    /// Suite name.
    pub name: String,
    /// Individual results.
    pub results: Vec<BenchmarkResult>,
}
impl BenchmarkSuite {
    /// Create an empty suite.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            results: Vec::new(),
        }
    }
    /// Add a result.
    pub fn add(&mut self, result: BenchmarkResult) {
        self.results.push(result);
    }
    /// Total wall-clock time for the entire suite.
    pub fn total_time(&self) -> Duration {
        self.results.iter().map(|r| r.total_time).sum()
    }
}
#[allow(dead_code)]
pub struct SuiteRunRecord {
    pub suite_name: String,
    pub started_at: std::time::Instant,
    pub results: Vec<BenchReportEntry>,
    pub env: BenchEnv,
}
#[allow(dead_code)]
impl SuiteRunRecord {
    pub fn new(suite_name: &str) -> Self {
        Self {
            suite_name: suite_name.to_string(),
            started_at: std::time::Instant::now(),
            results: Vec::new(),
            env: BenchEnv::current(),
        }
    }
    pub fn add_result(&mut self, entry: BenchReportEntry) {
        self.results.push(entry);
    }
    pub fn elapsed_secs(&self) -> f64 {
        self.started_at.elapsed().as_secs_f64()
    }
    pub fn to_summary(&self) -> String {
        format!(
            "Suite: {} | {} benchmarks in {:.2}s",
            self.suite_name,
            self.results.len(),
            self.elapsed_secs()
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum AnnotationSeverity {
    Info,
    Warning,
    Critical,
}
/// Comparison between two benchmark runs.
#[derive(Clone, Debug)]
pub struct BenchComparison {
    /// Baseline results keyed by name.
    pub baseline: Vec<BenchmarkResult>,
    /// Current results keyed by name.
    pub current: Vec<BenchmarkResult>,
    /// Names of benchmarks that regressed.
    pub regressions: Vec<String>,
    /// Names of benchmarks that improved.
    pub improvements: Vec<String>,
}
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct PipelineTiming {
    pub lex_ns: u64,
    pub parse_ns: u64,
    pub elab_ns: u64,
    pub check_ns: u64,
    pub codegen_ns: u64,
}
#[allow(dead_code)]
impl PipelineTiming {
    pub fn total_ns(&self) -> u64 {
        self.lex_ns + self.parse_ns + self.elab_ns + self.check_ns + self.codegen_ns
    }
    pub fn dominant_stage(&self) -> &'static str {
        let stages = [
            ("lex", self.lex_ns),
            ("parse", self.parse_ns),
            ("elab", self.elab_ns),
            ("check", self.check_ns),
            ("codegen", self.codegen_ns),
        ];
        stages
            .iter()
            .max_by_key(|(_, ns)| ns)
            .map(|(s, _)| *s)
            .unwrap_or("none")
    }
    pub fn to_csv_row(&self) -> String {
        format!(
            "{},{},{},{},{},{}",
            self.lex_ns,
            self.parse_ns,
            self.elab_ns,
            self.check_ns,
            self.codegen_ns,
            self.total_ns()
        )
    }
}
#[allow(dead_code)]
pub struct BaselineStore {
    entries: std::collections::HashMap<String, BaselineEntry>,
}
#[allow(dead_code)]
impl BaselineStore {
    pub fn new() -> Self {
        Self {
            entries: std::collections::HashMap::new(),
        }
    }
    pub fn record(&mut self, name: &str, mean_ns: f64, stddev_ns: f64, sample_count: usize) {
        self.entries.insert(
            name.to_string(),
            BaselineEntry {
                name: name.to_string(),
                mean_ns,
                stddev_ns,
                sample_count,
            },
        );
    }
    pub fn get(&self, name: &str) -> Option<&BaselineEntry> {
        self.entries.get(name)
    }
    pub fn compare(&self, name: &str, new_mean_ns: f64) -> Option<f64> {
        self.get(name)
            .map(|b| (new_mean_ns - b.mean_ns) / b.mean_ns * 100.0)
    }
    pub fn is_regression(&self, name: &str, new_mean_ns: f64, threshold_pct: f64) -> bool {
        self.compare(name, new_mean_ns)
            .map(|pct| pct > threshold_pct)
            .unwrap_or(false)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum BenchBudgetResult {
    Pass(std::time::Duration, std::time::Duration),
    Fail(std::time::Duration, std::time::Duration),
    NoBudget,
}
#[allow(dead_code)]
impl BenchBudgetResult {
    pub fn is_pass(&self) -> bool {
        matches!(self, Self::Pass(_, _))
    }
    pub fn is_fail(&self) -> bool {
        matches!(self, Self::Fail(_, _))
    }
}
#[allow(dead_code)]
pub struct OnlineRollingAvg {
    n: u64,
    mean: f64,
    m2: f64,
}
#[allow(dead_code)]
impl OnlineRollingAvg {
    pub fn new() -> Self {
        Self {
            n: 0,
            mean: 0.0,
            m2: 0.0,
        }
    }
    pub fn update(&mut self, x: f64) {
        self.n += 1;
        let delta = x - self.mean;
        self.mean += delta / self.n as f64;
        let delta2 = x - self.mean;
        self.m2 += delta * delta2;
    }
    pub fn mean(&self) -> f64 {
        self.mean
    }
    pub fn variance(&self) -> f64 {
        if self.n < 2 {
            return 0.0;
        }
        self.m2 / (self.n - 1) as f64
    }
    pub fn stddev(&self) -> f64 {
        self.variance().sqrt()
    }
    pub fn count(&self) -> u64 {
        self.n
    }
}
/// Configuration for running benchmarks.
#[derive(Clone, Debug)]
pub struct BenchmarkConfig {
    /// Number of warmup iterations (not counted).
    pub warmup_iterations: u64,
    /// Number of measured iterations.
    pub measure_iterations: u64,
    /// Maximum wall-clock time for the entire benchmark.
    pub timeout: Duration,
    /// Short alias: warmup iteration count (same as warmup_iterations).
    pub warmup_iters: usize,
    /// Short alias: measurement iteration count (same as measure_iterations).
    pub measurement_iters: usize,
    /// Whether to print verbose output during benchmarking.
    pub verbose: bool,
    /// Optional filter: only run benchmarks whose names contain this string.
    pub filter: Option<String>,
}
impl BenchmarkConfig {
    /// A fast configuration for CI / smoke-test usage.
    pub fn fast() -> Self {
        Self {
            warmup_iterations: 1,
            measure_iterations: 10,
            timeout: Duration::from_secs(10),
            warmup_iters: 1,
            measurement_iters: 10,
            verbose: false,
            filter: None,
        }
    }
}
#[allow(dead_code)]
pub struct BenchBudget {
    budgets: std::collections::HashMap<String, std::time::Duration>,
}
#[allow(dead_code)]
impl BenchBudget {
    pub fn new() -> Self {
        Self {
            budgets: std::collections::HashMap::new(),
        }
    }
    pub fn set(&mut self, name: &str, budget: std::time::Duration) {
        self.budgets.insert(name.to_string(), budget);
    }
    pub fn check(&self, name: &str, actual: std::time::Duration) -> BenchBudgetResult {
        match self.budgets.get(name) {
            None => BenchBudgetResult::NoBudget,
            Some(budget) if actual <= *budget => BenchBudgetResult::Pass(actual, *budget),
            Some(budget) => BenchBudgetResult::Fail(actual, *budget),
        }
    }
}
