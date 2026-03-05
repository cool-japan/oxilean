//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::time::{Duration, Instant};

use super::types::{
    AggregateResult, AnnotationSeverity, BaselineStore, BenchAnnotation, BenchBudget,
    BenchCapabilities, BenchComparison, BenchComparisonV2, BenchConfigBuilder, BenchEnv,
    BenchHistory, BenchPercentileTracker, BenchReport, BenchReportEntry, BenchRun, BenchRunConfig,
    BenchRunner, BenchSpec, BenchSpecRegistry, BenchTag, BenchmarkConfig, BenchmarkResult,
    BenchmarkSuite, FlameStack, MultiRunAggregator, NsHistogram, OnlineRollingAvg, OpsCounter,
    PipelineTiming, SampleCollector, SuiteRunRecord, TaggedResult,
};

/// Compute descriptive statistics from a list of timings.
pub fn compute_statistics(name: &str, timings: &[Duration]) -> BenchmarkResult {
    if timings.is_empty() {
        return BenchmarkResult {
            name: name.to_string(),
            iterations: 0,
            total_time: Duration::ZERO,
            min_time: Duration::ZERO,
            max_time: Duration::ZERO,
            mean_time: Duration::ZERO,
            std_dev: Duration::ZERO,
        };
    }
    let n = timings.len() as u64;
    let total: Duration = timings.iter().sum();
    let min = *timings
        .iter()
        .min()
        .expect("timings is non-empty: checked by early return");
    let max = *timings
        .iter()
        .max()
        .expect("timings is non-empty: checked by early return");
    let mean_ns = total.as_nanos() / n as u128;
    let mean = Duration::from_nanos(mean_ns as u64);
    let variance: f64 = timings
        .iter()
        .map(|t| {
            let diff = t.as_nanos() as f64 - mean_ns as f64;
            diff * diff
        })
        .sum::<f64>()
        / n as f64;
    let std_dev_ns = variance.sqrt();
    let std_dev = Duration::from_nanos(std_dev_ns as u64);
    BenchmarkResult {
        name: name.to_string(),
        iterations: n,
        total_time: total,
        min_time: min,
        max_time: max,
        mean_time: mean,
        std_dev,
    }
}
/// Benchmark the lexer on a source string.
pub fn bench_lexer(runner: &BenchRunner, source: &str) -> BenchmarkResult {
    let src = source.to_string();
    runner.run_benchmark("lexer", || {
        let mut lexer = oxilean_parse::Lexer::new(&src);
        let _tokens = lexer.tokenize();
    })
}
/// Benchmark the parser on a source string.
pub fn bench_parser(runner: &BenchRunner, source: &str) -> BenchmarkResult {
    let src = source.to_string();
    runner.run_benchmark("parser", || {
        let mut lexer = oxilean_parse::Lexer::new(&src);
        let tokens = lexer.tokenize();
        let mut parser = oxilean_parse::Parser::new(tokens);
        while parser.parse_decl().is_ok() {}
    })
}
/// Benchmark type checking on a simple expression.
pub fn bench_type_check(runner: &BenchRunner) -> BenchmarkResult {
    runner.run_benchmark("type_check", || {
        let env = oxilean_kernel::Environment::new();
        let _tc = oxilean_kernel::TypeChecker::new(&env);
    })
}
/// Benchmark elaboration context creation.
pub fn bench_elaboration(runner: &BenchRunner) -> BenchmarkResult {
    runner.run_benchmark("elaboration", || {
        let env = oxilean_kernel::Environment::new();
        let _ctx = oxilean_elab::ElabContext::new(&env);
    })
}
/// Benchmark WHNF reduction on a simple expression.
pub fn bench_whnf(runner: &BenchRunner) -> BenchmarkResult {
    runner.run_benchmark("whnf", || {
        let expr = oxilean_kernel::Expr::Sort(oxilean_kernel::Level::Zero);
        let _ = oxilean_kernel::whnf(&expr);
    })
}
/// Format benchmark results as a table.
pub fn format_bench_results(suite: &BenchmarkSuite) -> String {
    let mut lines = Vec::new();
    let header = format!(
        "{:<30} {:>10} {:>12} {:>12} {:>12} {:>12}",
        "Benchmark", "Iters", "Mean", "Min", "Max", "Std Dev"
    );
    let sep = "-".repeat(header.len());
    lines.push(format!("Suite: {}", suite.name));
    lines.push(sep.clone());
    lines.push(header);
    lines.push(sep.clone());
    for r in &suite.results {
        lines.push(format!(
            "{:<30} {:>10} {:>12} {:>12} {:>12} {:>12}",
            r.name,
            r.iterations,
            format_ns(r.mean_time),
            format_ns(r.min_time),
            format_ns(r.max_time),
            format_ns(r.std_dev),
        ));
    }
    lines.push(sep);
    lines.push(format!("Total: {}", format_ns(suite.total_time()),));
    lines.join("\n")
}
/// Compare two benchmark suites and identify regressions / improvements.
///
/// A regression is defined as a >10% increase in mean time;
/// an improvement is a >10% decrease.
pub fn compare_results(baseline: &BenchmarkSuite, current: &BenchmarkSuite) -> BenchComparison {
    let baseline_map: std::collections::HashMap<&str, &BenchmarkResult> = baseline
        .results
        .iter()
        .map(|r| (r.name.as_str(), r))
        .collect();
    let mut regressions = Vec::new();
    let mut improvements = Vec::new();
    for cur in &current.results {
        if let Some(base) = baseline_map.get(cur.name.as_str()) {
            let ratio = detect_regression(base, cur);
            if ratio > 1.10 {
                regressions.push(cur.name.clone());
            } else if ratio < 0.90 {
                improvements.push(cur.name.clone());
            }
        }
    }
    BenchComparison {
        baseline: baseline.results.clone(),
        current: current.results.clone(),
        regressions,
        improvements,
    }
}
/// Return the ratio `current_mean / baseline_mean`.
///
/// Values > 1.0 indicate a regression, < 1.0 an improvement.
pub fn detect_regression(baseline: &BenchmarkResult, current: &BenchmarkResult) -> f64 {
    let base_ns = baseline.mean_time.as_nanos() as f64;
    if base_ns == 0.0 {
        return 1.0;
    }
    current.mean_time.as_nanos() as f64 / base_ns
}
/// Format a comparison table.
pub fn format_comparison(comparison: &BenchComparison) -> String {
    let baseline_map: std::collections::HashMap<&str, &BenchmarkResult> = comparison
        .baseline
        .iter()
        .map(|r| (r.name.as_str(), r))
        .collect();
    let mut lines = Vec::new();
    let header = format!(
        "{:<30} {:>12} {:>12} {:>10}",
        "Benchmark", "Baseline", "Current", "Change"
    );
    let sep = "-".repeat(header.len());
    lines.push(header);
    lines.push(sep.clone());
    for cur in &comparison.current {
        if let Some(base) = baseline_map.get(cur.name.as_str()) {
            let ratio = detect_regression(base, cur);
            let change = format!("{:+.1}%", (ratio - 1.0) * 100.0);
            let marker = if ratio > 1.10 {
                " REGRESSION"
            } else if ratio < 0.90 {
                " IMPROVED"
            } else {
                ""
            };
            lines.push(format!(
                "{:<30} {:>12} {:>12} {:>10}{}",
                cur.name,
                format_ns(base.mean_time),
                format_ns(cur.mean_time),
                change,
                marker,
            ));
        }
    }
    lines.push(sep);
    if comparison.regressions.is_empty() && comparison.improvements.is_empty() {
        lines.push("No significant changes.".to_string());
    } else {
        if !comparison.regressions.is_empty() {
            lines.push(format!(
                "Regressions: {}",
                comparison.regressions.join(", ")
            ));
        }
        if !comparison.improvements.is_empty() {
            lines.push(format!(
                "Improvements: {}",
                comparison.improvements.join(", ")
            ));
        }
    }
    lines.join("\n")
}
/// Format a duration in a compact human-readable form (ns / us / ms / s).
pub fn format_ns(d: Duration) -> String {
    let ns = d.as_nanos();
    if ns >= 1_000_000_000 {
        format!("{:.3}s", d.as_secs_f64())
    } else if ns >= 1_000_000 {
        format!("{:.3}ms", ns as f64 / 1_000_000.0)
    } else if ns >= 1_000 {
        format!("{:.3}us", ns as f64 / 1_000.0)
    } else {
        format!("{}ns", ns)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_benchmark_config_default() {
        let cfg = BenchmarkConfig::default();
        assert_eq!(cfg.warmup_iterations, 5);
        assert_eq!(cfg.measure_iterations, 100);
        assert_eq!(cfg.timeout, Duration::from_secs(60));
    }
    #[test]
    fn test_benchmark_config_fast() {
        let cfg = BenchmarkConfig::fast();
        assert_eq!(cfg.warmup_iterations, 1);
        assert_eq!(cfg.measure_iterations, 10);
    }
    #[test]
    fn test_compute_statistics_empty() {
        let result = compute_statistics("empty", &[]);
        assert_eq!(result.iterations, 0);
        assert_eq!(result.total_time, Duration::ZERO);
    }
    #[test]
    fn test_compute_statistics_single() {
        let timings = vec![Duration::from_millis(10)];
        let result = compute_statistics("single", &timings);
        assert_eq!(result.iterations, 1);
        assert_eq!(result.min_time, Duration::from_millis(10));
        assert_eq!(result.max_time, Duration::from_millis(10));
        assert_eq!(result.mean_time, Duration::from_millis(10));
    }
    #[test]
    fn test_compute_statistics_multiple() {
        let timings = vec![
            Duration::from_millis(10),
            Duration::from_millis(20),
            Duration::from_millis(30),
        ];
        let result = compute_statistics("multi", &timings);
        assert_eq!(result.iterations, 3);
        assert_eq!(result.min_time, Duration::from_millis(10));
        assert_eq!(result.max_time, Duration::from_millis(30));
        assert_eq!(result.mean_time, Duration::from_millis(20));
    }
    #[test]
    fn test_benchmark_result_throughput() {
        let result = BenchmarkResult {
            name: "test".into(),
            iterations: 100,
            total_time: Duration::from_secs(1),
            min_time: Duration::from_millis(5),
            max_time: Duration::from_millis(15),
            mean_time: Duration::from_millis(10),
            std_dev: Duration::from_millis(2),
        };
        assert!((result.throughput() - 100.0).abs() < 0.1);
    }
    #[test]
    fn test_benchmark_result_cv() {
        let result = BenchmarkResult {
            name: "test".into(),
            iterations: 100,
            total_time: Duration::from_secs(1),
            min_time: Duration::from_millis(5),
            max_time: Duration::from_millis(15),
            mean_time: Duration::from_millis(10),
            std_dev: Duration::from_millis(2),
        };
        assert!((result.cv_percent() - 20.0).abs() < 0.1);
    }
    #[test]
    fn test_benchmark_result_display() {
        let result = BenchmarkResult {
            name: "test_bench".into(),
            iterations: 50,
            total_time: Duration::from_millis(500),
            min_time: Duration::from_millis(8),
            max_time: Duration::from_millis(12),
            mean_time: Duration::from_millis(10),
            std_dev: Duration::from_millis(1),
        };
        let s = result.to_string();
        assert!(s.contains("test_bench"));
        assert!(s.contains("50"));
    }
    #[test]
    fn test_benchmark_suite_empty() {
        let suite = BenchmarkSuite::new("empty");
        assert!(suite.results.is_empty());
        assert_eq!(suite.total_time(), Duration::ZERO);
    }
    #[test]
    fn test_benchmark_suite_total_time() {
        let mut suite = BenchmarkSuite::new("suite");
        suite.add(BenchmarkResult {
            name: "a".into(),
            iterations: 10,
            total_time: Duration::from_millis(100),
            min_time: Duration::ZERO,
            max_time: Duration::ZERO,
            mean_time: Duration::ZERO,
            std_dev: Duration::ZERO,
        });
        suite.add(BenchmarkResult {
            name: "b".into(),
            iterations: 10,
            total_time: Duration::from_millis(200),
            min_time: Duration::ZERO,
            max_time: Duration::ZERO,
            mean_time: Duration::ZERO,
            std_dev: Duration::ZERO,
        });
        assert_eq!(suite.total_time(), Duration::from_millis(300));
    }
    #[test]
    fn test_bench_runner_simple() {
        let runner = BenchRunner::new(BenchmarkConfig::fast());
        let mut counter = 0u64;
        let result = runner.run_benchmark("counter", || {
            counter += 1;
        });
        assert_eq!(result.iterations, 10);
        assert_eq!(counter, 11);
    }
    #[test]
    fn test_detect_regression() {
        let base = BenchmarkResult {
            name: "x".into(),
            iterations: 100,
            total_time: Duration::from_secs(1),
            min_time: Duration::ZERO,
            max_time: Duration::ZERO,
            mean_time: Duration::from_millis(10),
            std_dev: Duration::ZERO,
        };
        let fast = BenchmarkResult {
            name: "x".into(),
            iterations: 100,
            total_time: Duration::from_secs(1),
            min_time: Duration::ZERO,
            max_time: Duration::ZERO,
            mean_time: Duration::from_millis(5),
            std_dev: Duration::ZERO,
        };
        let slow = BenchmarkResult {
            name: "x".into(),
            iterations: 100,
            total_time: Duration::from_secs(1),
            min_time: Duration::ZERO,
            max_time: Duration::ZERO,
            mean_time: Duration::from_millis(20),
            std_dev: Duration::ZERO,
        };
        let ratio_fast = detect_regression(&base, &fast);
        assert!(ratio_fast < 1.0);
        let ratio_slow = detect_regression(&base, &slow);
        assert!(ratio_slow > 1.0);
    }
    #[test]
    fn test_compare_results() {
        let mut baseline = BenchmarkSuite::new("base");
        baseline.add(BenchmarkResult {
            name: "test".into(),
            iterations: 100,
            total_time: Duration::from_secs(1),
            min_time: Duration::ZERO,
            max_time: Duration::ZERO,
            mean_time: Duration::from_millis(10),
            std_dev: Duration::ZERO,
        });
        let mut current = BenchmarkSuite::new("current");
        current.add(BenchmarkResult {
            name: "test".into(),
            iterations: 100,
            total_time: Duration::from_secs(1),
            min_time: Duration::ZERO,
            max_time: Duration::ZERO,
            mean_time: Duration::from_millis(15),
            std_dev: Duration::ZERO,
        });
        let cmp = compare_results(&baseline, &current);
        assert_eq!(cmp.regressions.len(), 1);
        assert!(cmp.improvements.is_empty());
    }
    #[test]
    fn test_format_ns() {
        assert_eq!(format_ns(Duration::from_nanos(500)), "500ns");
        assert!(format_ns(Duration::from_micros(500)).contains("us"));
        assert!(format_ns(Duration::from_millis(1500)).contains("s"));
    }
    #[test]
    fn test_format_bench_results() {
        let mut suite = BenchmarkSuite::new("demo");
        suite.add(BenchmarkResult {
            name: "alpha".into(),
            iterations: 10,
            total_time: Duration::from_millis(100),
            min_time: Duration::from_millis(8),
            max_time: Duration::from_millis(12),
            mean_time: Duration::from_millis(10),
            std_dev: Duration::from_millis(1),
        });
        let text = format_bench_results(&suite);
        assert!(text.contains("alpha"));
        assert!(text.contains("Suite: demo"));
    }
    #[test]
    fn test_format_comparison() {
        let mut baseline = BenchmarkSuite::new("base");
        baseline.add(BenchmarkResult {
            name: "bench".into(),
            iterations: 10,
            total_time: Duration::from_millis(100),
            min_time: Duration::ZERO,
            max_time: Duration::ZERO,
            mean_time: Duration::from_millis(10),
            std_dev: Duration::ZERO,
        });
        let mut current = BenchmarkSuite::new("current");
        current.add(BenchmarkResult {
            name: "bench".into(),
            iterations: 10,
            total_time: Duration::from_millis(100),
            min_time: Duration::ZERO,
            max_time: Duration::ZERO,
            mean_time: Duration::from_millis(10),
            std_dev: Duration::ZERO,
        });
        let cmp = compare_results(&baseline, &current);
        let text = format_comparison(&cmp);
        assert!(text.contains("bench"));
        assert!(text.contains("No significant changes"));
    }
}
/// Aggregate results from multiple benchmark suites into one summary.
pub fn aggregate_results(suites: &[&BenchmarkSuite]) -> AggregateResult {
    let all_results: Vec<&BenchmarkResult> = suites.iter().flat_map(|s| s.results.iter()).collect();
    let total_benchmarks = all_results.len();
    let total_time: Duration = all_results.iter().map(|r| r.total_time).sum();
    let mean_per_benchmark = if total_benchmarks > 0 {
        total_time / total_benchmarks as u32
    } else {
        Duration::ZERO
    };
    AggregateResult {
        total_benchmarks,
        total_time,
        mean_per_benchmark,
        potential_regressions: vec![],
    }
}
/// Detect trend regressions in benchmark history.
///
/// Returns names of benchmarks with a positive (worsening) slope
/// above the given threshold (nanoseconds per run).
pub fn detect_trend_regressions(history: &BenchHistory, threshold_nanos: f64) -> Vec<String> {
    let mut regressions = Vec::new();
    for name in history.bench_names() {
        if let Some(slope) = history.trend_slope(name) {
            if slope > threshold_nanos {
                regressions.push(name.to_string());
            }
        }
    }
    regressions.sort();
    regressions
}
#[cfg(test)]
mod extended_tests {
    use super::*;
    #[test]
    fn test_bench_history_record_and_entries() {
        let mut h = BenchHistory::new();
        h.record("kernel", "run1", 1000);
        h.record("kernel", "run2", 1100);
        let entries = h.entries_for("kernel");
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].mean_nanos, 1000);
    }
    #[test]
    fn test_bench_history_entries_for_missing() {
        let h = BenchHistory::new();
        assert!(h.entries_for("missing").is_empty());
    }
    #[test]
    fn test_trend_slope_increasing() {
        let mut h = BenchHistory::new();
        h.record("b", "r1", 100);
        h.record("b", "r2", 200);
        h.record("b", "r3", 300);
        let slope = h.trend_slope("b").expect("test operation should succeed");
        assert!(slope > 0.0, "slope should be positive (worsening)");
    }
    #[test]
    fn test_trend_slope_insufficient_data() {
        let mut h = BenchHistory::new();
        h.record("b", "r1", 100);
        assert!(h.trend_slope("b").is_none());
    }
    #[test]
    fn test_trend_slope_flat() {
        let mut h = BenchHistory::new();
        h.record("b", "r1", 500);
        h.record("b", "r2", 500);
        h.record("b", "r3", 500);
        let slope = h.trend_slope("b").expect("test operation should succeed");
        assert!(slope.abs() < 1.0, "flat trend should have near-zero slope");
    }
    #[test]
    fn test_bench_config_builder_defaults() {
        let cfg = BenchConfigBuilder::new().build();
        assert_eq!(cfg.warmup_iters, 3);
        assert_eq!(cfg.measurement_iters, 10);
        assert!(!cfg.verbose);
    }
    #[test]
    fn test_bench_config_builder_custom() {
        let cfg = BenchConfigBuilder::new()
            .warmup_iters(5)
            .measurement_iters(20)
            .verbose(true)
            .filter("kernel")
            .build();
        assert_eq!(cfg.warmup_iters, 5);
        assert_eq!(cfg.measurement_iters, 20);
        assert!(cfg.verbose);
        assert_eq!(cfg.filter, Some("kernel".to_string()));
    }
    #[test]
    fn test_aggregate_results_empty() {
        let agg = aggregate_results(&[]);
        assert_eq!(agg.total_benchmarks, 0);
        assert_eq!(agg.total_time, Duration::ZERO);
    }
    #[test]
    fn test_aggregate_results_single_suite() {
        let mut suite = BenchmarkSuite::new("test");
        suite.add(BenchmarkResult {
            name: "bench1".into(),
            iterations: 10,
            total_time: Duration::from_millis(100),
            min_time: Duration::ZERO,
            max_time: Duration::ZERO,
            mean_time: Duration::from_millis(10),
            std_dev: Duration::ZERO,
        });
        suite.add(BenchmarkResult {
            name: "bench2".into(),
            iterations: 10,
            total_time: Duration::from_millis(200),
            min_time: Duration::ZERO,
            max_time: Duration::ZERO,
            mean_time: Duration::from_millis(20),
            std_dev: Duration::ZERO,
        });
        let agg = aggregate_results(&[&suite]);
        assert_eq!(agg.total_benchmarks, 2);
        assert_eq!(agg.total_time, Duration::from_millis(300));
    }
    #[test]
    fn test_detect_trend_regressions_no_regression() {
        let mut h = BenchHistory::new();
        h.record("b", "r1", 500);
        h.record("b", "r2", 500);
        let regressions = detect_trend_regressions(&h, 10.0);
        assert!(regressions.is_empty());
    }
    #[test]
    fn test_detect_trend_regressions_detected() {
        let mut h = BenchHistory::new();
        h.record("slow_bench", "r1", 100);
        h.record("slow_bench", "r2", 10000);
        let regressions = detect_trend_regressions(&h, 10.0);
        assert!(regressions.contains(&"slow_bench".to_string()));
    }
    #[test]
    fn test_bench_names() {
        let mut h = BenchHistory::new();
        h.record("alpha", "r1", 100);
        h.record("beta", "r1", 200);
        let mut names = h.bench_names();
        names.sort();
        assert!(names.contains(&"alpha"));
        assert!(names.contains(&"beta"));
    }
}
#[allow(dead_code)]
pub fn filter_by_tag<'a>(results: &'a [TaggedResult], tag: &BenchTag) -> Vec<&'a TaggedResult> {
    results.iter().filter(|r| r.tags.contains(tag)).collect()
}
#[allow(dead_code)]
pub fn group_results_by_prefix(
    results: &[(String, f64)],
    sep: char,
) -> std::collections::HashMap<String, Vec<(String, f64)>> {
    let mut groups: std::collections::HashMap<String, Vec<(String, f64)>> =
        std::collections::HashMap::new();
    for (name, val) in results {
        let group = name.splitn(2, sep).next().unwrap_or(name).to_string();
        groups.entry(group).or_default().push((name.clone(), *val));
    }
    groups
}
#[allow(dead_code)]
pub fn summarize_group(group: &[(String, f64)]) -> (f64, f64, f64) {
    if group.is_empty() {
        return (0.0, 0.0, 0.0);
    }
    let vals: Vec<f64> = group.iter().map(|(_, v)| *v).collect();
    let mean = vals.iter().sum::<f64>() / vals.len() as f64;
    let min = vals.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = vals.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    (mean, min, max)
}
#[allow(dead_code)]
pub fn compute_z_score(value: f64, mean: f64, stddev: f64) -> f64 {
    if stddev == 0.0 {
        return 0.0;
    }
    (value - mean) / stddev
}
#[allow(dead_code)]
pub fn is_stat_regression(
    new_mean: f64,
    baseline_mean: f64,
    baseline_stddev: f64,
    z_threshold: f64,
) -> bool {
    compute_z_score(new_mean, baseline_mean, baseline_stddev) > z_threshold
}
#[allow(dead_code)]
pub fn format_ns_f64(ns: f64) -> String {
    if ns < 1_000.0 {
        format!("{:.1}ns", ns)
    } else if ns < 1_000_000.0 {
        format!("{:.2}µs", ns / 1_000.0)
    } else if ns < 1_000_000_000.0 {
        format!("{:.2}ms", ns / 1_000_000.0)
    } else {
        format!("{:.2}s", ns / 1_000_000_000.0)
    }
}
#[allow(dead_code)]
pub fn format_throughput(bytes: u64, elapsed_ns: f64) -> String {
    let secs = elapsed_ns / 1e9;
    if secs == 0.0 {
        return "inf".to_string();
    }
    let bps = bytes as f64 / secs;
    if bps < 1024.0 {
        format!("{:.0} B/s", bps)
    } else if bps < 1024.0 * 1024.0 {
        format!("{:.1} KB/s", bps / 1024.0)
    } else if bps < 1024.0 * 1024.0 * 1024.0 {
        format!("{:.1} MB/s", bps / (1024.0 * 1024.0))
    } else {
        format!("{:.1} GB/s", bps / (1024.0 * 1024.0 * 1024.0))
    }
}
#[cfg(test)]
mod bench_extended_tests {
    use super::*;
    #[test]
    fn test_percentile_tracker() {
        let mut t = BenchPercentileTracker::new();
        for i in 1..=100 {
            t.add(i as u64);
        }
        assert_eq!(t.p50(), Some(51));
        assert_eq!(t.count(), 100);
    }
    #[test]
    fn test_baseline_store() {
        let mut store = BaselineStore::new();
        store.record("lex", 1000.0, 50.0, 100);
        assert!(!store.is_regression("lex", 1001.0, 10.0));
        assert!(store.is_regression("lex", 1200.0, 10.0));
    }
    #[test]
    fn test_online_rolling_avg() {
        let mut avg = OnlineRollingAvg::new();
        for i in 1..=10 {
            avg.update(i as f64);
        }
        assert!((avg.mean() - 5.5).abs() < 0.01);
        assert!(avg.stddev() > 0.0);
    }
    #[test]
    fn test_ns_histogram_add_and_count() {
        let mut hist = NsHistogram::new(0, 1000, 10);
        hist.add(100);
        hist.add(500);
        hist.add(900);
        assert_eq!(hist.total_count(), 3);
    }
    #[test]
    fn test_ns_histogram_render() {
        let mut hist = NsHistogram::new(0, 1000, 5);
        hist.add(100);
        hist.add(200);
        let rendered = hist.render();
        assert!(!rendered.is_empty());
        assert!(rendered.contains("ns"));
    }
    #[test]
    fn test_bench_budget_pass() {
        let mut budget = BenchBudget::new();
        budget.set("parse", std::time::Duration::from_millis(10));
        let result = budget.check("parse", std::time::Duration::from_millis(5));
        assert!(result.is_pass());
    }
    #[test]
    fn test_bench_budget_fail() {
        let mut budget = BenchBudget::new();
        budget.set("elab", std::time::Duration::from_millis(5));
        let result = budget.check("elab", std::time::Duration::from_millis(100));
        assert!(result.is_fail());
    }
    #[test]
    fn test_ops_counter() {
        let mut counter = OpsCounter::new();
        counter.add_ops(1000, std::time::Duration::from_secs(1));
        assert!((counter.ops_per_sec() - 1000.0).abs() < 0.1);
        assert!((counter.ns_per_op() - 1_000_000.0).abs() < 100.0);
    }
    #[test]
    fn test_sample_collector_outlier_removal() {
        let mut sc = SampleCollector::new();
        for i in 1..=10 {
            sc.add(i as f64);
        }
        sc.add(10000.0);
        sc.remove_outliers_iqr();
        assert!(sc.max() < 1000.0);
    }
    #[test]
    fn test_bench_report_to_table() {
        let mut report = BenchReport::new();
        report.add(BenchReportEntry {
            name: "lex".to_string(),
            mean_ns: 1000.0,
            stddev_ns: 50.0,
            min_ns: 900.0,
            max_ns: 1100.0,
            sample_count: 100,
            baseline_diff_pct: Some(5.0),
        });
        let table = report.to_table();
        assert!(table.contains("lex"));
        assert!(table.contains("1000"));
    }
    #[test]
    fn test_bench_report_regressions() {
        let mut report = BenchReport::new();
        report.add(BenchReportEntry {
            name: "slow".to_string(),
            mean_ns: 5000.0,
            stddev_ns: 100.0,
            min_ns: 4800.0,
            max_ns: 5200.0,
            sample_count: 50,
            baseline_diff_pct: Some(50.0),
        });
        report.add(BenchReportEntry {
            name: "fast".to_string(),
            mean_ns: 100.0,
            stddev_ns: 5.0,
            min_ns: 90.0,
            max_ns: 110.0,
            sample_count: 50,
            baseline_diff_pct: Some(-2.0),
        });
        let regressions = report.regressions(10.0);
        assert_eq!(regressions.len(), 1);
        assert_eq!(regressions[0].name, "slow");
    }
    #[test]
    fn test_format_ns_f64() {
        assert!(format_ns_f64(500.0).contains("ns"));
        assert!(format_ns_f64(1500.0).contains("µs"));
        assert!(format_ns_f64(1_500_000.0).contains("ms"));
        assert!(format_ns_f64(1_500_000_000.0).contains("s"));
    }
    #[test]
    fn test_z_score_regression() {
        assert!(is_stat_regression(1500.0, 1000.0, 100.0, 2.0));
        assert!(!is_stat_regression(1050.0, 1000.0, 100.0, 2.0));
    }
    #[test]
    fn test_group_results_by_prefix() {
        let results = vec![
            ("lex/simple".to_string(), 100.0),
            ("lex/complex".to_string(), 200.0),
            ("parse/basic".to_string(), 300.0),
        ];
        let groups = group_results_by_prefix(&results, '/');
        assert!(groups.contains_key("lex"));
        assert_eq!(groups["lex"].len(), 2);
    }
    #[test]
    fn test_bench_tag_filter() {
        let results = vec![
            TaggedResult {
                name: "a".to_string(),
                tags: vec![BenchTag::new("slow")],
                mean_ns: 1000.0,
                stddev_ns: 0.0,
            },
            TaggedResult {
                name: "b".to_string(),
                tags: vec![BenchTag::new("fast")],
                mean_ns: 100.0,
                stddev_ns: 0.0,
            },
        ];
        let slow = filter_by_tag(&results, &BenchTag::new("slow"));
        assert_eq!(slow.len(), 1);
        assert_eq!(slow[0].name, "a");
    }
}
#[allow(dead_code)]
pub fn num_cpus_estimate() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
}
#[allow(dead_code)]
pub fn adaptive_warmup_iters(target_duration_ms: u64, single_iter_ns: u64) -> usize {
    let target_ns = target_duration_ms * 1_000_000;
    if single_iter_ns == 0 {
        return 3;
    }
    let n = target_ns / single_iter_ns;
    (n as usize).clamp(1, 100)
}
#[cfg(test)]
mod bench_final_tests {
    use super::*;
    #[test]
    fn test_bench_spec_builder() {
        let spec = BenchSpec::new("lex_test", "Lexer benchmark")
            .with_warmup(5)
            .with_iters(20)
            .with_timeout_ms(1000);
        assert_eq!(spec.name, "lex_test");
        assert_eq!(spec.warmup_iters, 5);
        assert_eq!(spec.measure_iters, 20);
        assert_eq!(spec.timeout_ms, Some(1000));
    }
    #[test]
    fn test_spec_registry() {
        let mut reg = BenchSpecRegistry::new();
        reg.register(BenchSpec::new("a", "desc a"));
        reg.register(BenchSpec::new("b", "desc b"));
        assert_eq!(reg.count(), 2);
        assert!(reg.get("a").is_some());
        assert!(reg.get("c").is_none());
    }
    #[test]
    fn test_pipeline_timing_total() {
        let t = PipelineTiming {
            lex_ns: 100,
            parse_ns: 200,
            elab_ns: 300,
            check_ns: 50,
            codegen_ns: 150,
        };
        assert_eq!(t.total_ns(), 800);
        assert_eq!(t.dominant_stage(), "elab");
    }
    #[test]
    fn test_pipeline_timing_csv() {
        let t = PipelineTiming {
            lex_ns: 1,
            parse_ns: 2,
            elab_ns: 3,
            check_ns: 4,
            codegen_ns: 5,
        };
        let row = t.to_csv_row();
        assert!(row.contains("15"));
    }
    #[test]
    fn test_multi_run_aggregator() {
        let mut agg = MultiRunAggregator::new();
        agg.add_run("run1", vec![10.0, 12.0, 11.0]);
        agg.add_run("run2", vec![20.0, 22.0, 21.0]);
        let means = agg.mean_per_run();
        assert!((means[0] - 11.0).abs() < 0.1);
        assert_eq!(agg.best_run_idx(), Some(0));
        assert_eq!(agg.worst_run_idx(), Some(1));
    }
    #[test]
    fn test_bench_env_current() {
        let env = BenchEnv::current();
        assert!(!env.os.is_empty());
        assert!(!env.arch.is_empty());
        let json = env.to_json();
        assert!(json.contains("os"));
    }
    #[test]
    fn test_adaptive_warmup() {
        let n = adaptive_warmup_iters(100, 10_000_000);
        assert!(n >= 1 && n <= 100);
    }
    #[test]
    fn test_flame_stack_sample() {
        let mut fs = FlameStack::new();
        fs.push_frame("main");
        fs.push_frame("lex");
        fs.sample();
        fs.sample();
        fs.pop_frame();
        fs.sample();
        assert_eq!(fs.total_samples(), 3);
    }
    #[test]
    fn test_suite_run_record() {
        let mut rec = SuiteRunRecord::new("my_suite");
        rec.add_result(BenchReportEntry {
            name: "a".to_string(),
            mean_ns: 100.0,
            stddev_ns: 5.0,
            min_ns: 90.0,
            max_ns: 110.0,
            sample_count: 10,
            baseline_diff_pct: None,
        });
        let summary = rec.to_summary();
        assert!(summary.contains("my_suite"));
        assert!(summary.contains("1 benchmarks"));
    }
    #[test]
    fn test_format_throughput() {
        let s = format_throughput(1024 * 1024, 1_000_000_000.0);
        assert!(s.contains("MB/s") || s.contains("KB/s"));
    }
    #[test]
    fn test_summarize_group() {
        let g = vec![
            ("a".to_string(), 10.0),
            ("b".to_string(), 20.0),
            ("c".to_string(), 30.0),
        ];
        let (mean, min, max) = summarize_group(&g);
        assert!((mean - 20.0).abs() < 0.1);
        assert!((min - 10.0).abs() < 0.1);
        assert!((max - 30.0).abs() < 0.1);
    }
    #[test]
    fn test_percentile_tracker_empty() {
        let mut t = BenchPercentileTracker::new();
        assert_eq!(t.p50(), None);
        assert_eq!(t.p99(), None);
    }
}
#[cfg(test)]
mod bench_comparison_tests {
    use super::*;
    #[test]
    fn test_bench_comparison_regression() {
        let mut cmp = BenchComparisonV2::new("v1", "v2");
        cmp.add("lex", 1000.0, 1200.0, 5.0);
        cmp.add("parse", 2000.0, 1900.0, 5.0);
        let regressions = cmp.regressions();
        assert_eq!(regressions.len(), 1);
        assert_eq!(regressions[0].bench_name, "lex");
    }
    #[test]
    fn test_bench_comparison_improvements() {
        let mut cmp = BenchComparisonV2::new("old", "new");
        cmp.add("elab", 5000.0, 4000.0, 5.0);
        let improvements = cmp.improvements();
        assert_eq!(improvements.len(), 1);
    }
    #[test]
    fn test_bench_comparison_table() {
        let mut cmp = BenchComparisonV2::new("baseline", "candidate");
        cmp.add("test", 100.0, 110.0, 5.0);
        let table = cmp.to_table();
        assert!(table.contains("Benchmark"));
        assert!(table.contains("test"));
    }
    #[test]
    fn test_bench_annotation_severity() {
        let ann = BenchAnnotation::critical("elab", "3x regression!");
        assert_eq!(ann.severity, AnnotationSeverity::Critical);
        let warn = BenchAnnotation::warning("lex", "slight slowdown");
        assert_eq!(warn.severity, AnnotationSeverity::Warning);
    }
    #[test]
    fn test_bench_capabilities_default() {
        let caps = BenchCapabilities::default();
        assert!(caps.supports_warmup);
        assert!(caps.supports_flamegraph);
        assert!(!caps.supports_parallel);
    }
    #[test]
    fn test_bench_run_config_default() {
        let cfg = BenchRunConfig::default();
        assert!(cfg.save_results);
        assert!(!cfg.compare_with_baseline);
        assert!((cfg.regression_threshold_pct - 5.0).abs() < 0.01);
    }
}
#[allow(dead_code)]
pub fn bench_version() -> &'static str {
    "2.0.0"
}
#[allow(dead_code)]
pub fn bench_min_iters() -> usize {
    1
}
#[allow(dead_code)]
pub fn bench_max_iters() -> usize {
    1_000_000
}
#[allow(dead_code)]
pub fn bench_default_warmup() -> usize {
    3
}
#[allow(dead_code)]
pub fn bench_supports_html_output() -> bool {
    true
}
#[allow(dead_code)]
pub fn bench_supports_json_output() -> bool {
    true
}
#[allow(dead_code)]
pub fn bench_supports_csv_output() -> bool {
    true
}
#[allow(dead_code)]
pub fn bench_estimate_iters_for_ms(target_ms: u64, single_iter_ns: u64) -> usize {
    let target_ns = target_ms * 1_000_000;
    if single_iter_ns == 0 {
        return 100;
    }
    (target_ns / single_iter_ns).clamp(1, 1_000_000) as usize
}
#[allow(dead_code)]
pub fn bench_do_not_optimize<T>(val: T) -> T {
    std::hint::black_box(val)
}
#[cfg(test)]
mod bench_utility_tests {
    use super::*;
    #[test]
    fn test_bench_version() {
        assert!(!bench_version().is_empty());
    }
    #[test]
    fn test_estimate_iters() {
        let n = bench_estimate_iters_for_ms(100, 1_000_000);
        assert_eq!(n, 100);
    }
    #[test]
    fn test_estimate_iters_zero_single() {
        let n = bench_estimate_iters_for_ms(100, 0);
        assert_eq!(n, 100);
    }
    #[test]
    fn test_do_not_optimize() {
        let x = bench_do_not_optimize(42u64);
        assert_eq!(x, 42);
    }
}
#[cfg(test)]
mod bench_run_tests {
    use super::*;
    #[test]
    fn test_bench_run_mean() {
        let mut run = BenchRun::new("test", 3);
        run.add_sample(100);
        run.add_sample(200);
        run.add_sample(300);
        assert!((run.mean_ns() - 200.0).abs() < 0.1);
    }
    #[test]
    fn test_bench_run_to_report() {
        let mut run = BenchRun::new("mytest", 0);
        for i in 1..=10 {
            run.add_sample(i * 100);
        }
        let entry = run.to_report_entry();
        assert_eq!(entry.name, "mytest");
        assert_eq!(entry.sample_count, 10);
        assert!(entry.stddev_ns >= 0.0);
    }
    #[test]
    fn test_bench_run_empty() {
        let run = BenchRun::new("empty", 0);
        assert_eq!(run.mean_ns(), 0.0);
    }
}
