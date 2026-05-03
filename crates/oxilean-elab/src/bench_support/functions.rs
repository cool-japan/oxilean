//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::time::{Duration, Instant};

use super::types::{
    BenchCompareRow, BenchCompareTable, BenchConfig, BenchMeta, BenchResult, BenchResultBuilder,
    BenchSuite, BenchmarkSet, Comparison, ElabBenchmark, ElabMicroBench, ElabMicroBenchRegistry,
    FlamegraphHook, HotPathAnalyzer, HotPathEntry, LatencyHistogram, MovingAverage,
    PartialEvalBenchConfig, PartialEvalBenchResult, RegressionReport, RegressionSeverity,
    RetryPolicy, SolverBenchStats, ThroughputResult, Timer, WarmupStrategy,
};

/// Compute the arithmetic mean of a slice of `f64` values.
/// Returns `0.0` for an empty slice.
pub fn mean(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    values.iter().sum::<f64>() / values.len() as f64
}
/// Compute the median of a slice (non-destructive; clones internally).
/// Returns `0.0` for an empty slice.
pub fn median(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let n = sorted.len();
    if n % 2 == 0 {
        (sorted[n / 2 - 1] + sorted[n / 2]) / 2.0
    } else {
        sorted[n / 2]
    }
}
/// Compute the population standard deviation.
/// Returns `0.0` for slices with fewer than 2 elements.
pub fn stddev(values: &[f64]) -> f64 {
    if values.len() < 2 {
        return 0.0;
    }
    let m = mean(values);
    let variance = values.iter().map(|v| (v - m).powi(2)).sum::<f64>() / values.len() as f64;
    variance.sqrt()
}
/// Compute a percentile (0..=100) of a slice.
/// Uses nearest-rank method. Returns `0.0` for empty slices.
pub fn percentile(values: &[f64], p: f64) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let p = p.clamp(0.0, 100.0);
    let rank = (p / 100.0 * (sorted.len() as f64 - 1.0)).round() as usize;
    let rank = rank.min(sorted.len() - 1);
    sorted[rank]
}
/// Format a `BenchResult` as a human-readable text block.
pub fn format_text(result: &BenchResult) -> String {
    let mut out = String::new();
    out.push_str(&format!("Benchmark: {}\n", result.name));
    out.push_str(&format!("  Iterations: {}\n", result.iterations));
    out.push_str(&format!("  Avg:    {:.1} ns\n", result.avg_ns));
    out.push_str(&format!("  Min:    {:.1} ns\n", result.min_ns));
    out.push_str(&format!("  Max:    {:.1} ns\n", result.max_ns));
    out.push_str(&format!("  Median: {:.1} ns\n", result.median_ns()));
    out.push_str(&format!("  Stddev: {:.1} ns\n", result.stddev_ns));
    out.push_str(&format!("  CV:     {:.4}\n", result.cv()));
    out.push_str(&format!("  P95:    {:.1} ns\n", result.percentile_ns(95.0)));
    out.push_str(&format!("  P99:    {:.1} ns\n", result.percentile_ns(99.0)));
    out
}
/// Format a `BenchResult` as a CSV row.
/// Header: name,iterations,avg_ns,min_ns,max_ns,stddev_ns,median_ns,p95_ns,p99_ns
pub fn format_csv_header() -> &'static str {
    "name,iterations,avg_ns,min_ns,max_ns,stddev_ns,median_ns,p95_ns,p99_ns"
}
/// Format a single result as a CSV data row.
pub fn format_csv_row(result: &BenchResult) -> String {
    format!(
        "{},{},{:.1},{:.1},{:.1},{:.1},{:.1},{:.1},{:.1}",
        result.name,
        result.iterations,
        result.avg_ns,
        result.min_ns,
        result.max_ns,
        result.stddev_ns,
        result.median_ns(),
        result.percentile_ns(95.0),
        result.percentile_ns(99.0),
    )
}
/// Format a `BenchResult` as a JSON-like string (no external JSON crate).
pub fn format_json(result: &BenchResult) -> String {
    format!(
        concat!(
            "{{\n",
            "  \"name\": \"{}\",\n",
            "  \"iterations\": {},\n",
            "  \"avg_ns\": {:.1},\n",
            "  \"min_ns\": {:.1},\n",
            "  \"max_ns\": {:.1},\n",
            "  \"stddev_ns\": {:.1},\n",
            "  \"median_ns\": {:.1},\n",
            "  \"cv\": {:.4},\n",
            "  \"p95_ns\": {:.1},\n",
            "  \"p99_ns\": {:.1}\n",
            "}}"
        ),
        result.name,
        result.iterations,
        result.avg_ns,
        result.min_ns,
        result.max_ns,
        result.stddev_ns,
        result.median_ns(),
        result.cv(),
        result.percentile_ns(95.0),
        result.percentile_ns(99.0),
    )
}
/// Format an entire `BenchSuite` as a JSON-like array string.
pub fn format_suite_json(suite: &BenchSuite) -> String {
    let mut out = String::new();
    out.push_str("{\n");
    out.push_str(&format!("  \"suite\": \"{}\",\n", suite.name));
    out.push_str("  \"benchmarks\": [\n");
    for (i, r) in suite.iter().enumerate() {
        let json = format_json(r);
        for (j, line) in json.lines().enumerate() {
            out.push_str("    ");
            out.push_str(line);
            if j < json.lines().count() - 1 {
                out.push('\n');
            }
        }
        if i + 1 < suite.len() {
            out.push(',');
        }
        out.push('\n');
    }
    out.push_str("  ]\n");
    out.push_str("}\n");
    out
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::bench_support::*;
    #[test]
    fn test_bench_config_defaults() {
        let cfg = BenchConfig::default();
        assert_eq!(cfg.iterations, 100);
        assert_eq!(cfg.warmup_rounds, 5);
        assert!(cfg.time_limit_ms.is_none());
    }
    #[test]
    fn test_bench_config_builder() {
        let cfg = BenchConfig::default()
            .with_iterations(500)
            .with_warmup(10)
            .with_time_limit_ms(5000);
        assert_eq!(cfg.iterations, 500);
        assert_eq!(cfg.warmup_rounds, 10);
        assert_eq!(cfg.time_limit_ms, Some(5000));
    }
    #[test]
    fn test_timer_start_stop() {
        let mut t = Timer::new();
        assert!(!t.is_running());
        t.start();
        assert!(t.is_running());
        std::thread::sleep(Duration::from_micros(100));
        t.stop();
        assert!(!t.is_running());
        assert!(t.elapsed_ns() > 0);
    }
    #[test]
    fn test_timer_reset() {
        let mut t = Timer::new();
        t.start();
        std::hint::black_box(42);
        t.stop();
        t.reset();
        assert_eq!(t.elapsed(), Duration::ZERO);
        assert!(!t.is_running());
    }
    #[test]
    fn test_statistics_mean() {
        assert_eq!(mean(&[]), 0.0);
        assert!((mean(&[10.0, 20.0, 30.0]) - 20.0).abs() < 1e-10);
        assert!((mean(&[5.0]) - 5.0).abs() < 1e-10);
    }
    #[test]
    fn test_statistics_median() {
        assert_eq!(median(&[]), 0.0);
        assert!((median(&[3.0, 1.0, 2.0]) - 2.0).abs() < 1e-10);
        assert!((median(&[4.0, 1.0, 3.0, 2.0]) - 2.5).abs() < 1e-10);
    }
    #[test]
    fn test_statistics_stddev() {
        assert_eq!(stddev(&[]), 0.0);
        assert_eq!(stddev(&[5.0]), 0.0);
        let vals = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        assert!((stddev(&vals) - 2.0).abs() < 1e-10);
    }
    #[test]
    fn test_statistics_percentile() {
        assert_eq!(percentile(&[], 50.0), 0.0);
        let vals: Vec<f64> = (1..=100).map(|i| i as f64).collect();
        assert!((percentile(&vals, 0.0) - 1.0).abs() < 1e-10);
        assert!((percentile(&vals, 100.0) - 100.0).abs() < 1e-10);
        assert!((percentile(&vals, 50.0) - 50.0).abs() < 1.5);
    }
    #[test]
    fn test_bench_result_from_samples() {
        let samples = vec![100.0, 200.0, 300.0, 400.0, 500.0];
        let r = BenchResult::from_samples("test_bench", samples);
        assert_eq!(r.name, "test_bench");
        assert!((r.avg_ns - 300.0).abs() < 1e-10);
        assert!((r.min_ns - 100.0).abs() < 1e-10);
        assert!((r.max_ns - 500.0).abs() < 1e-10);
        assert_eq!(r.iterations, 5);
        assert!((r.median_ns() - 300.0).abs() < 1e-10);
        assert!(r.cv() > 0.0);
    }
    #[test]
    fn test_bench_suite_fastest_slowest() {
        let mut suite = BenchSuite::new("test_suite");
        suite.add_result(BenchResult::from_samples("fast", vec![10.0, 20.0, 15.0]));
        suite.add_result(BenchResult::from_samples("slow", vec![100.0, 200.0, 150.0]));
        assert_eq!(suite.len(), 2);
        assert!(!suite.is_empty());
        assert_eq!(
            suite.fastest().expect("test operation should succeed").name,
            "fast"
        );
        assert_eq!(
            suite.slowest().expect("test operation should succeed").name,
            "slow"
        );
    }
    #[test]
    fn test_comparison() {
        let baseline = BenchResult::from_samples("old", vec![200.0, 200.0, 200.0]);
        let candidate = BenchResult::from_samples("new", vec![100.0, 100.0, 100.0]);
        let cmp = Comparison::compare(&baseline, &candidate);
        assert!(cmp.is_improvement());
        assert!(!cmp.is_regression());
        assert!((cmp.speedup - 2.0).abs() < 1e-10);
        assert!((cmp.diff_ns - 100.0).abs() < 1e-10);
        assert!((cmp.pct_change - 50.0).abs() < 1e-10);
        let display = format!("{}", cmp);
        assert!(display.contains("faster"));
    }
    #[test]
    fn test_comparison_regression() {
        let baseline = BenchResult::from_samples("old", vec![100.0; 5]);
        let candidate = BenchResult::from_samples("new", vec![200.0; 5]);
        let cmp = Comparison::compare(&baseline, &candidate);
        assert!(cmp.is_regression());
        assert!(!cmp.is_improvement());
        assert!((cmp.speedup - 0.5).abs() < 1e-10);
    }
    #[test]
    fn test_format_text() {
        let r = BenchResult::from_samples("my_bench", vec![50.0, 60.0, 70.0]);
        let txt = format_text(&r);
        assert!(txt.contains("my_bench"));
        assert!(txt.contains("Avg:"));
        assert!(txt.contains("Min:"));
        assert!(txt.contains("Max:"));
        assert!(txt.contains("Median:"));
        assert!(txt.contains("Stddev:"));
    }
    #[test]
    fn test_format_csv() {
        let r = BenchResult::from_samples("csv_bench", vec![100.0, 200.0]);
        let header = format_csv_header();
        assert!(header.contains("name"));
        assert!(header.contains("avg_ns"));
        let row = format_csv_row(&r);
        assert!(row.starts_with("csv_bench,"));
    }
    #[test]
    fn test_format_json() {
        let r = BenchResult::from_samples("json_bench", vec![10.0, 20.0, 30.0]);
        let json = format_json(&r);
        assert!(json.contains("\"name\": \"json_bench\""));
        assert!(json.contains("\"iterations\": 3"));
        assert!(json.contains("\"avg_ns\":"));
    }
    #[test]
    fn test_elab_benchmark_run() {
        let cfg = BenchConfig::quick();
        let bench = ElabBenchmark::new("add_loop", cfg);
        let result = bench.run(|| {
            let mut s = 0u64;
            for i in 0..100 {
                s = s.wrapping_add(i);
            }
            std::hint::black_box(s);
        });
        assert_eq!(result.name, "add_loop");
        assert!(result.iterations > 0);
        assert!(result.avg_ns > 0.0);
    }
    #[test]
    fn test_elab_benchmark_run_with_result() {
        let cfg = BenchConfig::default().with_iterations(5).with_warmup(1);
        let bench = ElabBenchmark::new("fib", cfg);
        let result = bench.run_with_result(|| {
            let mut a = 0u64;
            let mut b = 1u64;
            for _ in 0..20 {
                let c = a.wrapping_add(b);
                a = b;
                b = c;
            }
            b
        });
        assert_eq!(result.iterations, 5);
    }
    #[test]
    fn test_suite_summary_text() {
        let mut suite = BenchSuite::new("elaboration");
        suite.add_result(BenchResult::from_samples("expr_elab", vec![500.0, 600.0]));
        suite.add_result(BenchResult::from_samples("decl_elab", vec![1000.0, 1200.0]));
        let text = suite.summary_text();
        assert!(text.contains("elaboration"));
        assert!(text.contains("expr_elab"));
        assert!(text.contains("decl_elab"));
    }
    #[test]
    fn test_format_suite_json() {
        let mut suite = BenchSuite::new("kernel");
        suite.add_result(BenchResult::from_samples("alpha_eq", vec![30.0, 40.0]));
        let json = format_suite_json(&suite);
        assert!(json.contains("\"suite\": \"kernel\""));
        assert!(json.contains("alpha_eq"));
    }
    #[test]
    fn test_timer_accumulates() {
        let mut t = Timer::new();
        t.start();
        std::hint::black_box(0u64);
        t.stop();
        let first = t.elapsed();
        t.start();
        std::hint::black_box(0u64);
        t.stop();
        assert!(t.elapsed() >= first);
    }
    #[test]
    fn test_bench_config_quick_and_thorough() {
        let q = BenchConfig::quick();
        assert_eq!(q.iterations, 10);
        assert_eq!(q.warmup_rounds, 1);
        let th = BenchConfig::thorough();
        assert_eq!(th.iterations, 1000);
        assert_eq!(th.warmup_rounds, 20);
        assert_eq!(th.time_limit_ms, Some(30_000));
    }
}
/// Compute the interquartile range (IQR) of a set of samples.
#[allow(dead_code)]
pub fn iqr(samples: &[f64]) -> f64 {
    if samples.len() < 2 {
        return 0.0;
    }
    let mut sorted = samples.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let q1 = percentile(&sorted, 25.0);
    let q3 = percentile(&sorted, 75.0);
    q3 - q1
}
/// Compute the geometric mean of a set of positive samples.
#[allow(dead_code)]
pub fn geometric_mean(samples: &[f64]) -> f64 {
    if samples.is_empty() {
        return 0.0;
    }
    let log_sum: f64 = samples.iter().map(|&x| x.max(1e-300).ln()).sum();
    (log_sum / samples.len() as f64).exp()
}
/// Compute the harmonic mean of a set of positive samples.
#[allow(dead_code)]
pub fn harmonic_mean(samples: &[f64]) -> f64 {
    if samples.is_empty() {
        return 0.0;
    }
    let inv_sum: f64 = samples.iter().map(|&x| 1.0 / x.max(1e-300)).sum();
    samples.len() as f64 / inv_sum
}
/// Remove outliers using the Tukey fence method (1.5 * IQR).
#[allow(dead_code)]
pub fn remove_outliers(samples: &[f64]) -> Vec<f64> {
    if samples.len() < 4 {
        return samples.to_vec();
    }
    let mut sorted = samples.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let q1 = percentile(&sorted, 25.0);
    let q3 = percentile(&sorted, 75.0);
    let fence = 1.5 * (q3 - q1);
    samples
        .iter()
        .copied()
        .filter(|&x| x >= q1 - fence && x <= q3 + fence)
        .collect()
}
/// Compute a confidence interval estimate (mean ± z * stddev / sqrt(n)).
#[allow(dead_code)]
pub fn confidence_interval(samples: &[f64], z: f64) -> (f64, f64) {
    let n = samples.len();
    if n == 0 {
        return (0.0, 0.0);
    }
    let m = mean(samples);
    let s = stddev(samples);
    let margin = z * s / (n as f64).sqrt();
    (m - margin, m + margin)
}
#[cfg(test)]
mod extended_bench_tests {
    use super::*;
    use crate::bench_support::*;
    #[test]
    fn test_moving_average_basic() {
        let mut ma = MovingAverage::new(3);
        assert!((ma.average() - 0.0).abs() < 1e-10);
        ma.push(10.0);
        assert!((ma.average() - 10.0).abs() < 1e-10);
        ma.push(20.0);
        assert!((ma.average() - 15.0).abs() < 1e-10);
        ma.push(30.0);
        assert!((ma.average() - 20.0).abs() < 1e-10);
        ma.push(40.0);
        assert!((ma.average() - 30.0).abs() < 1e-10);
    }
    #[test]
    fn test_moving_average_reset() {
        let mut ma = MovingAverage::new(5);
        ma.push(100.0);
        ma.push(200.0);
        ma.reset();
        assert_eq!(ma.count(), 0);
        assert!((ma.average() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_throughput_result() {
        let tr = ThroughputResult::compute("parse", 1_000_000, 1_000_000_000);
        assert!((tr.items_per_sec - 1_000_000.0).abs() < 1.0);
        assert!((tr.ns_per_item - 1_000.0).abs() < 1e-6);
        let s = tr.format();
        assert!(s.contains("parse"));
    }
    #[test]
    fn test_latency_histogram_record() {
        let mut h = LatencyHistogram::new(8, 100.0, 100_000.0);
        h.record(500.0);
        h.record(1000.0);
        h.record(50000.0);
        assert_eq!(h.total(), 3);
        let fracs = h.fractions();
        let sum: f64 = fracs.iter().sum();
        assert!((sum - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_latency_histogram_ascii() {
        let mut h = LatencyHistogram::new(4, 1.0, 1000.0);
        for ns in [10.0, 100.0, 500.0] {
            h.record(ns);
        }
        let ascii = h.format_ascii();
        assert!(!ascii.is_empty());
    }
    #[test]
    fn test_regression_severity_ordering() {
        assert!(RegressionSeverity::Improvement < RegressionSeverity::Minor);
        assert!(RegressionSeverity::Minor < RegressionSeverity::Moderate);
        assert!(RegressionSeverity::Moderate < RegressionSeverity::Major);
    }
    #[test]
    fn test_regression_report_improvement() {
        let baseline = BenchResult::from_samples("base", vec![100.0; 5]);
        let candidate = BenchResult::from_samples("cand", vec![50.0; 5]);
        let cmp = Comparison::compare(&baseline, &candidate);
        let report = RegressionReport::from_comparison(cmp);
        assert!(!report.is_regression());
        assert_eq!(report.severity, RegressionSeverity::Improvement);
    }
    #[test]
    fn test_regression_report_major() {
        let baseline = BenchResult::from_samples("base", vec![100.0; 5]);
        let candidate = BenchResult::from_samples("cand", vec![300.0; 5]);
        let cmp = Comparison::compare(&baseline, &candidate);
        let report = RegressionReport::from_comparison(cmp);
        assert!(report.is_regression());
        assert_eq!(report.severity, RegressionSeverity::Major);
    }
    #[test]
    fn test_iqr_basic() {
        let samples = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let v = iqr(&samples);
        assert!(v > 0.0);
    }
    #[test]
    fn test_geometric_mean() {
        let samples = vec![1.0, 10.0, 100.0];
        let gm = geometric_mean(&samples);
        assert!((gm - 10.0).abs() < 1e-6);
    }
    #[test]
    fn test_harmonic_mean() {
        let samples = vec![1.0, 2.0, 4.0];
        let hm = harmonic_mean(&samples);
        assert!((hm - 12.0 / 7.0).abs() < 1e-6);
    }
    #[test]
    fn test_remove_outliers() {
        let samples = vec![10.0, 11.0, 12.0, 10.5, 11.5, 1000.0];
        let cleaned = remove_outliers(&samples);
        assert!(!cleaned.contains(&1000.0));
    }
    #[test]
    fn test_confidence_interval() {
        let samples: Vec<f64> = (1..=100).map(|i| i as f64).collect();
        let (lo, hi) = confidence_interval(&samples, 1.96);
        assert!(lo < hi);
        assert!(lo > 0.0);
    }
    #[test]
    fn test_bench_result_builder() {
        let r = BenchResultBuilder::new("builder_test")
            .sample(100.0)
            .sample(200.0)
            .sample(300.0)
            .build();
        assert_eq!(r.name, "builder_test");
        assert_eq!(r.iterations, 3);
    }
    #[test]
    fn test_warmup_strategy_none() {
        let mut count = 0usize;
        WarmupStrategy::None.apply(|| count += 1);
        assert_eq!(count, 0);
    }
    #[test]
    fn test_warmup_strategy_iterations() {
        let mut count = 0usize;
        WarmupStrategy::Iterations(7).apply(|| count += 1);
        assert_eq!(count, 7);
    }
    #[test]
    fn test_bench_meta_tags() {
        let meta = BenchMeta::new("Elaboration latency")
            .with_tag("elab")
            .with_tag("kernel")
            .mark_stable();
        assert!(meta.has_tag("elab"));
        assert!(meta.has_tag("kernel"));
        assert!(!meta.has_tag("parse"));
        assert!(meta.expect_stable);
    }
    #[test]
    fn test_retry_policy_lenient() {
        let p = RetryPolicy::lenient();
        assert_eq!(p.max_retries, 3);
        assert!((p.cv_threshold - 0.2).abs() < 1e-10);
    }
    #[test]
    fn test_retry_policy_strict() {
        let p = RetryPolicy::strict();
        assert_eq!(p.max_retries, 10);
        assert!((p.cv_threshold - 0.05).abs() < 1e-10);
    }
    #[test]
    fn test_benchmark_set_basic() {
        let mut set = BenchmarkSet::new("test_set", BenchConfig::quick());
        set.bench("increment", || {});
        assert_eq!(set.len(), 1);
        let suite = set.into_suite();
        assert_eq!(suite.len(), 1);
    }
    #[test]
    fn test_benchmark_set_empty() {
        let set = BenchmarkSet::new("empty", BenchConfig::quick());
        assert!(set.is_empty());
    }
    #[test]
    fn test_regression_severity_display() {
        assert_eq!(
            format!("{}", RegressionSeverity::Improvement),
            "improvement"
        );
        assert_eq!(format!("{}", RegressionSeverity::Major), "major regression");
    }
}
#[cfg(test)]
mod bench_ext_tests {
    use super::*;
    use crate::bench_support::*;
    #[test]
    fn test_flamegraph_hook_record() {
        let mut hook = FlamegraphHook::new(100);
        hook.start();
        hook.record_frame(0, "elab");
        hook.record_frame(1, "infer");
        assert_eq!(hook.sample_count(), 2);
        let collapsed = hook.collapsed_stacks();
        assert_eq!(collapsed.len(), 2);
        assert!(collapsed[1].contains("infer"));
    }
    #[test]
    fn test_flamegraph_hook_inactive() {
        let mut hook = FlamegraphHook::new(100);
        hook.record_frame(0, "elab");
        assert_eq!(hook.sample_count(), 0);
    }
    #[test]
    fn test_flamegraph_hook_clear() {
        let mut hook = FlamegraphHook::new(100);
        hook.start();
        hook.record_frame(0, "elab");
        hook.clear();
        assert_eq!(hook.sample_count(), 0);
    }
    #[test]
    fn test_elab_micro_bench_median() {
        let mut b = ElabMicroBench::new("unify", "Unification benchmark");
        for ns in [10, 20, 30, 40, 50] {
            b.record(ns);
        }
        assert_eq!(b.median_ns(), Some(30));
    }
    #[test]
    fn test_elab_micro_bench_mean_stddev() {
        let mut b = ElabMicroBench::new("simp", "Simp benchmark");
        b.record(10);
        b.record(20);
        b.record(30);
        let mean = b.mean_ns().expect("test operation should succeed");
        assert!((mean - 20.0).abs() < 1e-6);
        let sd = b.stddev_ns().expect("test operation should succeed");
        assert!(sd > 0.0);
    }
    #[test]
    fn test_elab_micro_bench_regression() {
        let mut b = ElabMicroBench::new("check", "Check benchmark").with_expected_ns(100);
        b.record(200);
        assert!(b.is_regression(10.0));
        assert!(!b.is_regression(500.0));
    }
    #[test]
    fn test_elab_micro_bench_no_data() {
        let b = ElabMicroBench::new("empty", "empty");
        assert_eq!(b.median_ns(), None);
        assert_eq!(b.mean_ns(), None);
        assert!(b.summary().contains("no data"));
    }
    #[test]
    fn test_micro_bench_registry_register_get() {
        let mut reg = ElabMicroBenchRegistry::new();
        let b = ElabMicroBench::new("test_bench", "a test");
        reg.register(b);
        assert!(reg.get("test_bench").is_some());
        assert!(reg.get("missing").is_none());
        assert_eq!(reg.len(), 1);
    }
    #[test]
    fn test_micro_bench_registry_regressions() {
        let mut reg = ElabMicroBenchRegistry::new();
        let mut b = ElabMicroBench::new("slow", "slow bench").with_expected_ns(50);
        b.record(500);
        reg.register(b);
        let b2 = ElabMicroBench::new("fast", "fast bench").with_expected_ns(50);
        reg.register(b2);
        let regs = reg.regressions(10.0);
        assert_eq!(regs.len(), 1);
        assert_eq!(regs[0].name, "slow");
    }
    #[test]
    fn test_solver_bench_stats_record() {
        let mut s = SolverBenchStats::new();
        s.record_invocation(5, true, 1000);
        s.record_invocation(3, false, 500);
        assert_eq!(s.invocations, 2);
        assert_eq!(s.constraints_solved, 5);
        assert_eq!(s.failures, 1);
        assert!((s.success_rate() - 0.5).abs() < 1e-10);
    }
    #[test]
    fn test_solver_bench_stats_merge() {
        let mut a = SolverBenchStats::new();
        a.record_invocation(10, true, 2000);
        a.record_depth(5);
        let mut b = SolverBenchStats::new();
        b.record_invocation(20, true, 3000);
        b.record_depth(8);
        a.merge(&b);
        assert_eq!(a.invocations, 2);
        assert_eq!(a.max_depth, 8);
    }
    #[test]
    fn test_partial_eval_bench_result_avg() {
        let r = PartialEvalBenchResult {
            steps_performed: 100,
            reached_nf: true,
            elapsed_ns: 1_000_000,
            step_times_ns: vec![],
        };
        assert!((r.avg_ns_per_step() - 10_000.0).abs() < 1e-6);
        assert!(r.summary().contains("steps=100"));
    }
    #[test]
    fn test_hot_path_analyzer_top_n() {
        let mut a = HotPathAnalyzer::new();
        a.record("check", 5000, 3000);
        a.record("infer", 2000, 1500);
        a.record("unify", 8000, 4000);
        let top = a.top_n(2);
        assert_eq!(top[0].name, "unify");
        assert_eq!(top[1].name, "check");
    }
    #[test]
    fn test_hot_path_analyzer_report() {
        let mut a = HotPathAnalyzer::new();
        a.record("elab", 10_000, 5_000);
        a.record("reduce", 3_000, 2_000);
        let report = a.report(5);
        assert!(report.contains("elab"));
        assert!(report.contains("reduce"));
    }
    #[test]
    fn test_hot_path_entry_self_ratio() {
        let mut e = HotPathEntry::new("f");
        e.record(1000, 600);
        assert!((e.self_ratio() - 0.6).abs() < 1e-10);
    }
    #[test]
    fn test_partial_eval_bench_config_builder() {
        let cfg = PartialEvalBenchConfig::new()
            .with_max_steps(500)
            .with_step_times()
            .nf_only();
        assert_eq!(cfg.max_steps, 500);
        assert!(cfg.record_step_times);
        assert!(cfg.nf_only);
    }
}
#[cfg(test)]
mod bench_compare_tests {
    use super::*;
    use crate::bench_support::*;
    #[test]
    fn test_bench_compare_row_ratio() {
        let row = BenchCompareRow::new("f", 100.0, 120.0);
        assert!((row.ratio() - 1.2).abs() < 1e-10);
        assert!((row.pct_change() - 20.0).abs() < 1e-10);
    }
    #[test]
    fn test_bench_compare_row_improvement() {
        let row = BenchCompareRow::new("g", 200.0, 100.0);
        assert!((row.pct_change() - (-50.0)).abs() < 1e-10);
    }
    #[test]
    fn test_bench_compare_table_regressions() {
        let mut table = BenchCompareTable::new();
        table.add(BenchCompareRow::new("slow", 100.0, 200.0));
        table.add(BenchCompareRow::new("fast", 100.0, 90.0));
        table.add(BenchCompareRow::new("same", 100.0, 101.0));
        let regs = table.regressions(10.0);
        assert_eq!(regs.len(), 1);
        assert_eq!(regs[0].name, "slow");
    }
    #[test]
    fn test_bench_compare_table_improvements() {
        let mut table = BenchCompareTable::new();
        table.add(BenchCompareRow::new("fast", 200.0, 100.0));
        table.add(BenchCompareRow::new("same", 100.0, 99.0));
        let imps = table.improvements(10.0);
        assert_eq!(imps.len(), 1);
        assert_eq!(imps[0].name, "fast");
    }
    #[test]
    fn test_bench_compare_table_format() {
        let mut table = BenchCompareTable::new();
        table.add(BenchCompareRow::new("check", 500.0, 600.0));
        let report = table.format_report();
        assert!(report.contains("check"));
        assert!(report.contains("+20.0%"));
    }
    #[test]
    fn test_bench_compare_table_empty() {
        let table = BenchCompareTable::new();
        assert!(table.is_empty());
        assert_eq!(table.len(), 0);
    }
    #[test]
    fn test_bench_compare_row_zero_baseline() {
        let row = BenchCompareRow::new("zero", 0.0, 100.0);
        assert!((row.ratio() - 1.0).abs() < 1e-10);
    }
}
