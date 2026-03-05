//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    AdaptiveWarmup, BatchTimer, BenchAnnotationSet, BenchConfig, BenchEventLog, BenchFilter,
    BenchGroup, BenchHarnessExt, BenchHarnessV2, BenchHistogram, BenchMatrix, BenchPlan,
    BenchProfiler, BenchRegistry, BenchReporter, BenchResult, BenchResultExt, BenchSuite,
    BenchSummary, BenchTimer, ColdCacheSimulator, CompareResult, ConfidenceInterval, CpuPinner,
    FuzzInput, HdrHistogram, IterationPolicy, LatencyPercentile, MetricSet, MovingAverage,
    MultiArmBandit, OlsRegression, ProgressBar, RegressionTest, SampleBuffer, ScalingTest,
    StabilityChecker, ThroughputTracker, ThroughputUnit, TimeSlice,
};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bench_timer_start() {
        let timer = BenchTimer::start();
        let _ = timer;
    }
    #[test]
    fn test_bench_timer_elapsed() {
        let timer = BenchTimer::start();
        let mut sum = 0u64;
        for i in 0..1000u64 {
            sum = sum.wrapping_add(i);
        }
        let _ = sum;
        let ms = timer.elapsed_ms();
        let us = timer.elapsed_us();
        assert!(ms >= 0.0, "elapsed_ms should be non-negative");
        assert!(us >= 0.0, "elapsed_us should be non-negative");
        assert!(
            us >= ms - 1.0,
            "elapsed_us should be >= elapsed_ms (roughly)"
        );
    }
    #[test]
    fn test_bench_result_creation() {
        let r = BenchResult::new("my_bench", 100.0, 1000);
        assert_eq!(r.name, "my_bench");
        assert_eq!(r.duration_ms, 100.0);
        assert_eq!(r.iterations, 1000);
        assert!((r.avg_ms() - 0.1).abs() < 1e-10);
        assert!((r.avg_us() - 100.0).abs() < 1e-10);
        assert!(r.iters_per_sec() > 0.0);
        let zero_iters = BenchResult::new("zero", 50.0, 0);
        assert_eq!(zero_iters.avg_ms(), 0.0);
        assert_eq!(zero_iters.avg_us(), 0.0);
        let s = format!("{}", r);
        assert!(s.contains("my_bench"));
    }
    #[test]
    fn test_bench_suite_new() {
        let suite = BenchSuite::new();
        assert!(suite.is_empty());
        assert_eq!(suite.len(), 0);
        assert!(suite.results().is_empty());
    }
    #[test]
    fn test_bench_suite_run() {
        let mut suite = BenchSuite::new();
        suite.run("noop", 10, || {});
        suite.run("sum", 100, || {
            let mut x = 0u64;
            for i in 0..10u64 {
                x = x.wrapping_add(i);
            }
            let _ = x;
        });
        assert_eq!(suite.len(), 2);
        assert_eq!(suite.results()[0].name, "noop");
        assert_eq!(suite.results()[0].iterations, 10);
        assert_eq!(suite.results()[1].name, "sum");
        assert_eq!(suite.results()[1].iterations, 100);
    }
    #[test]
    fn test_bench_suite_report() {
        let mut suite = BenchSuite::new();
        suite.run("alpha", 5, || {});
        let report = suite.report();
        assert!(report.contains("alpha"));
        assert!(report.contains("Benchmark Suite Report"));
    }
    #[test]
    fn test_bench_harness_new() {
        let harness = BenchHarnessV2::new(5, 50);
        assert_eq!(harness.warmup_count(), 5);
        assert_eq!(harness.iteration_count(), 50);
        let default_harness = BenchHarnessV2::default();
        assert_eq!(default_harness.warmup_count(), 10);
        assert_eq!(default_harness.iteration_count(), 100);
        let result = harness.bench("simple", || {});
        assert_eq!(result.name, "simple");
        assert_eq!(result.iterations, 50);
        assert!(result.duration_ms >= 0.0);
    }
    #[test]
    fn test_bench_harness_throughput() {
        let harness = BenchHarnessV2::new(0, 1000);
        let result = harness.bench("throughput_test", || {
            let mut x = 0u64;
            for i in 0..100u64 {
                x = x.wrapping_add(i);
            }
            let _ = x;
        });
        let tp = harness.throughput(&result, 100);
        assert!(tp > 0.0, "throughput should be positive, got {}", tp);
        let zero_dur = BenchResult::new("instant", 0.0, 100);
        let inf_tp = harness.throughput(&zero_dur, 10);
        assert!(inf_tp.is_infinite());
    }
}
#[cfg(test)]
mod tests_bench_extra {
    use super::*;
    #[test]
    fn test_histogram() {
        let mut h = BenchHistogram::new(0.0, 1000.0, 10);
        h.record(50.0);
        h.record(150.0);
        h.record(50.0);
        assert_eq!(h.total_samples(), 3);
        h.record(2000.0);
        assert_eq!(h.overflow_count(), 1);
    }
    #[test]
    fn test_moving_average() {
        let mut ma = MovingAverage::new(0.5);
        assert!(ma.current().is_none());
        ma.update(100.0);
        assert!((ma.current().expect("current should succeed") - 100.0).abs() < 1e-9);
        ma.update(200.0);
        assert!((ma.current().expect("current should succeed") - 150.0).abs() < 1e-9);
    }
    #[test]
    fn test_adaptive_warmup() {
        let mut w = AdaptiveWarmup::new(5, 0.10);
        for _ in 0..10 {
            w.record(100.0);
        }
        assert!(w.is_warmed());
    }
    #[test]
    fn test_throughput_tracker() {
        let mut t = ThroughputTracker::new(1000.0);
        t.record(100);
        t.record(100);
        let _ips = t.items_per_sec();
    }
    #[test]
    fn test_bench_matrix() {
        let mut m = BenchMatrix::new();
        let r0 = m.add_row("case_a");
        let c0 = m.add_col("metric_1");
        let c1 = m.add_col("metric_2");
        m.set(r0, c0, 1.23);
        m.set(r0, c1, 4.56);
        assert!((m.get(r0, c0).expect("element at r0, c0 should exist") - 1.23).abs() < 1e-9);
        let md = m.to_markdown();
        assert!(md.contains("case_a"));
        assert!(md.contains("metric_1"));
    }
    #[test]
    fn test_latency_percentile() {
        let mut lp = LatencyPercentile::new();
        for i in 1..=100 {
            lp.record(i as f64);
        }
        let (p50, p90, p99) = lp.summary().expect("summary should succeed");
        assert!(p50 <= p90 && p90 <= p99);
    }
    #[test]
    fn test_regression_test() {
        let rt = RegressionTest::new("foo", 100.0, 1.10);
        assert!(!rt.is_regression(105.0));
        assert!(rt.is_regression(115.0));
        assert_eq!(rt.verdict(105.0), "PASS");
        assert_eq!(rt.verdict(115.0), "REGRESSION");
    }
    #[test]
    fn test_bench_summary() {
        let mut s = BenchSummary::new("suite_a");
        s.passed = 5;
        s.regressions = 1;
        s.skipped = 2;
        s.total_ms = 500.0;
        assert_eq!(s.total(), 8);
        assert!(!s.all_passed());
        let line = s.result_line();
        assert!(line.contains("REGRESSION") || line.contains("regression"));
    }
    #[test]
    fn test_cold_cache_simulator() {
        let mut sim = ColdCacheSimulator::new(1024 * 1024);
        assert_eq!(sim.buffer_size(), 1024 * 1024);
        sim.flush();
    }
    #[test]
    fn test_bench_annotation_set() {
        let mut s = BenchAnnotationSet::new();
        s.add("os", "linux");
        s.add("arch", "x86_64");
        assert_eq!(s.get("os"), Some("linux"));
        assert_eq!(s.get("arch"), Some("x86_64"));
        assert_eq!(s.get("cpu"), None);
        assert_eq!(s.len(), 2);
    }
    #[test]
    fn test_iteration_policy() {
        assert_eq!(IterationPolicy::Fixed(10).min_iters(), 10);
        assert!(IterationPolicy::TimeBounded(100).is_time_bounded());
        let adaptive = IterationPolicy::Adaptive { min: 5, max: 100 };
        assert_eq!(adaptive.min_iters(), 5);
    }
    #[test]
    fn test_bench_harness_fixed() {
        let mut harness = BenchHarnessExt::new("noop", IterationPolicy::Fixed(10));
        harness.run(|| {
            std::hint::black_box(42u64);
        });
        assert_eq!(harness.num_samples(), 10);
        assert!(harness.median_us().is_some());
    }
}
#[cfg(test)]
mod tests_bench_metric {
    use super::*;
    #[test]
    fn test_metric_set() {
        let mut ms = MetricSet::new();
        ms.add("latency", 12.5, "µs");
        ms.add("throughput", 1e6, "op/s");
        assert_eq!(ms.len(), 2);
        assert!(
            (ms.get("latency")
                .expect("element at \'latency\' should exist")
                - 12.5)
                .abs()
                < 1e-9
        );
        let disp = ms.display_all();
        assert!(disp.contains("latency"));
    }
}
/// Compares `new_us` against `baseline_us` with a given threshold fraction.
#[allow(dead_code)]
pub fn compare_timings(baseline_us: f64, new_us: f64, threshold: f64) -> CompareResult {
    if baseline_us < f64::EPSILON {
        return CompareResult::Neutral;
    }
    let ratio = new_us / baseline_us;
    if ratio < 1.0 - threshold {
        CompareResult::Improvement
    } else if ratio > 1.0 + threshold {
        CompareResult::Regression
    } else {
        CompareResult::Neutral
    }
}
#[cfg(test)]
mod tests_bench_extra2 {
    use super::*;
    #[test]
    fn test_bench_event_log() {
        let mut log = BenchEventLog::new();
        log.record("start");
        log.record("end");
        assert_eq!(log.count(), 2);
        assert!(log.since_last_ms() >= 0.0);
    }
    #[test]
    fn test_scaling_test() {
        let mut st = ScalingTest::new();
        st.add_point(100, 100.0);
        st.add_point(1000, 1000.0);
        let exp = st.scaling_exponent().expect("exp should be present");
        assert!((exp - 1.0).abs() < 0.1, "expected ~1.0, got {}", exp);
        assert!(st.is_at_most_order(1.1));
    }
    #[test]
    fn test_bench_filter() {
        let mut f = BenchFilter::new();
        f.include("fast");
        f.exclude("slow");
        assert!(f.accepts("fast_sort"));
        assert!(!f.accepts("slow_sort"));
        assert!(!f.accepts("bubble_sort"));
    }
    #[test]
    fn test_bench_registry() {
        let mut reg = BenchRegistry::new();
        let mut g = BenchGroup::new("group_a");
        g.add("bench_1");
        g.add("bench_2");
        reg.add_group(g);
        assert_eq!(reg.total_benchmarks(), 2);
        let names = reg.all_benchmark_names();
        assert!(names.contains(&"bench_1"));
        let found = reg.find_group("bench_2");
        assert!(found.is_some());
        assert_eq!(found.expect("found should be valid").name, "group_a");
    }
    #[test]
    fn test_sample_buffer() {
        let mut buf = SampleBuffer::new(5);
        for i in 1..=7 {
            buf.push(i as f64);
        }
        assert_eq!(buf.len(), 5);
        let mean = buf.mean().expect("mean should be present");
        assert!(mean > 0.0);
    }
    #[test]
    fn test_progress_bar() {
        let mut pb = ProgressBar::new(10, 20);
        assert_eq!(pb.fraction(), 0.0);
        for _ in 0..5 {
            pb.step();
        }
        assert!((pb.fraction() - 0.5).abs() < 1e-9);
        let r = pb.render();
        assert!(r.contains("5/10"));
        for _ in 0..5 {
            pb.step();
        }
        assert!(pb.is_complete());
    }
    #[test]
    fn test_compare_timings() {
        assert_eq!(
            compare_timings(100.0, 85.0, 0.10),
            CompareResult::Improvement
        );
        assert_eq!(compare_timings(100.0, 100.0, 0.10), CompareResult::Neutral);
        assert_eq!(
            compare_timings(100.0, 115.0, 0.10),
            CompareResult::Regression
        );
    }
    #[test]
    fn test_cpu_pinner() {
        let pinner = CpuPinner::new(0);
        assert_eq!(pinner.cpu(), 0);
        assert!(pinner.pin());
    }
}
#[cfg(test)]
mod tests_bench_result {
    use super::*;
    #[test]
    fn test_bench_result_from_samples() {
        let samples = vec![10.0, 12.0, 11.0, 10.5, 11.5];
        let r = BenchResultExt::from_samples("my_bench", &samples).expect("r should be present");
        assert!((r.mean_us - 11.0).abs() < 1.0);
        assert_eq!(r.iterations, 5);
        let csv = r.to_csv();
        assert!(csv.starts_with("my_bench,"));
        assert!(r.is_stable(0.5));
    }
    #[test]
    fn test_bench_result_empty() {
        let r = BenchResultExt::from_samples("empty", &[]);
        assert!(r.is_none());
    }
}
#[cfg(test)]
mod tests_bench_profiler {
    use super::*;
    #[test]
    fn test_bench_profiler() {
        let p = BenchProfiler::start("my_op");
        assert_eq!(p.label(), "my_op");
        let elapsed = p.stop();
        assert!(elapsed >= 0.0);
    }
}
#[cfg(test)]
mod tests_bench_final {
    use super::*;
    #[test]
    fn test_stability_checker() {
        let mut sc = StabilityChecker::new(5, 0.02);
        for _ in 0..4 {
            assert!(!sc.push(100.0));
        }
        assert!(sc.push(100.0));
        assert_eq!(sc.count(), 5);
    }
    #[test]
    fn test_multi_arm_bandit() {
        let mut bandit = MultiArmBandit::new(3, 0.1);
        for _ in 0..30 {
            bandit.update(0, 1.0);
            bandit.update(1, 0.5);
            bandit.update(2, 0.2);
        }
        assert_eq!(bandit.best_arm(), 0);
        assert!((bandit.estimate(0) - 1.0).abs() < 0.01);
    }
    #[test]
    fn test_time_slice() {
        let mut ts = TimeSlice::new("phase_1", 100.0);
        assert!(!ts.consume(30.0));
        assert!(!ts.consume(50.0));
        assert!(ts.consume(30.0));
        assert!(ts.remaining_ms() == 0.0);
        let util = ts.utilisation();
        assert!(util > 1.0);
    }
    #[test]
    fn test_bench_reporter() {
        let mut rep = BenchReporter::new("suite");
        rep.add(
            BenchResultExt::from_samples("alpha", &[10.0, 11.0, 10.5])
                .expect("value should be present"),
        );
        rep.add(
            BenchResultExt::from_samples("beta", &[20.0, 21.0, 20.5])
                .expect("value should be present"),
        );
        assert_eq!(rep.count(), 2);
        assert_eq!(rep.fastest().expect("fastest should succeed").name, "alpha");
        assert_eq!(rep.slowest().expect("slowest should succeed").name, "beta");
        let csv = rep.to_csv();
        assert!(csv.contains("alpha"));
    }
    #[test]
    fn test_fuzz_input() {
        let mut fi = FuzzInput::new(12345);
        let v1 = fi.next_u64();
        let v2 = fi.next_u64();
        assert_ne!(v1, v2);
        let idx = fi.next_usize(10);
        assert!(idx < 10);
        let f = fi.next_f64();
        assert!((0.0..1.0).contains(&f));
        let mut buf = [0u8; 16];
        fi.fill_bytes(&mut buf);
        assert!(buf.iter().any(|&b| b != 0));
    }
    #[test]
    fn test_bench_plan() {
        let mut plan = BenchPlan::default_plan();
        plan.add("bench_a");
        plan.add("bench_b");
        plan.add("bench_c");
        assert_eq!(plan.len(), 3);
        plan.reverse();
        assert_eq!(plan.order[0], "bench_c");
        assert_eq!(plan.warmup_iters, 3);
        assert_eq!(plan.measure_iters, 10);
    }
}
#[cfg(test)]
mod tests_bench_final3 {
    use super::*;
    #[test]
    fn test_confidence_interval() {
        let samples: Vec<f64> = (1..=100).map(|i| i as f64).collect();
        let ci = ConfidenceInterval::compute_95(&samples).expect("ci should be present");
        assert!((ci.estimate - 50.5).abs() < 1.0);
        assert!(ci.upper > ci.lower);
        assert!(ci.half_width() > 0.0);
        let disp = ci.display();
        assert!(disp.contains("CI"));
    }
    #[test]
    fn test_ols_regression() {
        let mut reg = OlsRegression::new();
        for i in 0..=10 {
            reg.add(i as f64, 2.0 * i as f64 + 1.0);
        }
        let (a, b) = reg.fit().expect("fit should succeed");
        assert!((a - 1.0).abs() < 1e-9, "intercept: {}", a);
        assert!((b - 2.0).abs() < 1e-9, "slope: {}", b);
        let pred = reg.predict(5.0).expect("pred should be present");
        assert!((pred - 11.0).abs() < 1e-9);
    }
    #[test]
    fn test_hdr_histogram() {
        let mut h = HdrHistogram::new(1000.0, 100);
        for i in 0..100 {
            h.record(i as f64 * 10.0);
        }
        assert_eq!(h.total_count(), 100);
        let p50 = h.value_at_percentile(50.0);
        let p99 = h.value_at_percentile(99.0);
        assert!(p50 <= p99);
    }
    #[test]
    fn test_batch_timer() {
        let timer = BatchTimer::start(1000);
        let _elapsed = timer.total_us();
        assert!(timer.ns_per_item() >= 0.0);
    }
}
/// Calculates throughput given byte count and elapsed microseconds.
#[allow(dead_code)]
pub fn calc_throughput(bytes: u64, elapsed_us: f64, unit: ThroughputUnit) -> f64 {
    if elapsed_us < f64::EPSILON {
        return 0.0;
    }
    let bps = bytes as f64 / (elapsed_us * 1e-6);
    unit.from_bytes_per_sec(bps)
}
#[cfg(test)]
mod tests_throughput_unit {
    use super::*;
    #[test]
    fn test_calc_throughput() {
        let tp = calc_throughput(1_000_000_000, 1_000_000.0, ThroughputUnit::GBPerSec);
        assert!((tp - 1.0).abs() < 0.01, "expected 1.0 GB/s, got {}", tp);
    }
    #[test]
    fn test_throughput_unit_label() {
        assert_eq!(ThroughputUnit::MBPerSec.label(), "MB/s");
        assert_eq!(ThroughputUnit::OpsPerSec.label(), "ops/s");
    }
}
#[cfg(test)]
mod tests_bench_config {
    use super::*;
    #[test]
    fn test_default_config() {
        let cfg = BenchConfig::default_config();
        assert!(!cfg.verbose);
        assert_eq!(cfg.warmup_iters, 3);
        assert_eq!(cfg.measure_iters, 10);
        assert!(!cfg.has_fuzz());
    }
    #[test]
    fn test_ci_config() {
        let cfg = BenchConfig::ci_config();
        assert!(cfg.json_output);
        assert!(cfg.has_fuzz());
        assert_eq!(cfg.fuzz_seed, Some(42));
    }
}
