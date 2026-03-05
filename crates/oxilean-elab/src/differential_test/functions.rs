//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    AnnotatedDiffCase, CompareStrategy, DeduplicateTransform, DiffCorpus, DiffTestCase,
    DiffTestComparator, DiffTestFilter, DiffTestHarness, DiffTestHistory, DiffTestMatrix,
    DiffTestMetrics, DiffTestPipeline, DiffTestReport, DiffTestResult, DiffTestRunner,
    DiffTestScheduler, DiffTestStatistics, DiffTestSuite, DiffTestTagRegistry, Lean4DiffTester,
    LimitTransform, MultiLineOutput, ParametricDiffCase, RegressionDetector, ReverseOrderTransform,
    ScheduledTest, SnapshotStore, TestPriority, TestRunSnapshot, TestTiming,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::differential_test::*;
    #[test]
    fn test_diff_test_case_creation() {
        let case = DiffTestCase::new_success("my_test", "def x := 42");
        assert_eq!(case.name, "my_test");
        assert_eq!(case.input, "def x := 42");
        assert!(case.should_succeed);
        assert!(case.expected_output.is_none());
        let fail_case = DiffTestCase::new_failure("bad_test", "");
        assert!(!fail_case.should_succeed);
        let with_out = DiffTestCase::new_success_with_output("out_test", "input", "output");
        assert_eq!(with_out.expected_output, Some("output".to_string()));
    }
    #[test]
    fn test_diff_test_runner_pass() {
        let runner = DiffTestRunner::new();
        let case = DiffTestCase::new_success("pass_case", "theorem foo : True := trivial");
        let result = runner.run_case(&case);
        assert!(result.is_pass(), "Expected Pass, got {:?}", result);
        assert_eq!(result.label(), "PASS");
    }
    #[test]
    fn test_diff_test_runner_fail() {
        let runner = DiffTestRunner::new();
        let case =
            DiffTestCase::new_success_with_output("fail_case", "some input", "wrong expected");
        let result = runner.run_case(&case);
        assert!(result.is_fail(), "Expected Fail, got {:?}", result);
        assert_eq!(result.label(), "FAIL");
        if let DiffTestResult::Fail { actual, expected } = &result {
            assert_eq!(expected, "wrong expected");
            assert!(actual.contains("some input"));
        }
    }
    #[test]
    fn test_diff_test_suite_empty() {
        let suite = DiffTestSuite::new();
        assert!(suite.is_empty());
        assert_eq!(suite.len(), 0);
        assert!(suite.name.is_none());
        let named = DiffTestSuite::named("my_suite");
        assert_eq!(named.name.as_deref(), Some("my_suite"));
    }
    #[test]
    fn test_diff_test_suite_run() {
        let mut suite = DiffTestSuite::new();
        suite.add(DiffTestCase::new_success("s1", "valid input"));
        suite.add(DiffTestCase::new_failure("s2", ""));
        assert_eq!(suite.len(), 2);
        let runner = DiffTestRunner::new();
        let results = runner.run_suite(&suite);
        assert_eq!(results.len(), 2);
        assert!(results[0].1.is_pass());
        assert!(results[1].1.is_pass());
    }
    #[test]
    fn test_lean4_diff_tester_new() {
        let tester = Lean4DiffTester::new();
        assert_eq!(tester.case_count(), 0);
    }
    #[test]
    fn test_lean4_diff_tester_add_case() {
        let mut tester = Lean4DiffTester::new();
        tester.add_lean4_case("case1", "theorem foo : True := trivial", true);
        tester.add_lean4_case("case2", "def x : Nat := 0", true);
        tester.add_lean4_case("bad", "", false);
        assert_eq!(tester.case_count(), 3);
        let report = tester.run_all();
        assert_eq!(report.total, 3);
        assert!(report.passed > 0);
    }
    #[test]
    fn test_diff_test_report() {
        let report = DiffTestReport {
            total: 10,
            passed: 8,
            failed: 1,
            errors: 1,
            unexpected: 0,
        };
        assert_eq!(report.total, 10);
        assert_eq!(report.passed, 8);
        assert!(!report.all_passed());
        let rate = report.pass_rate();
        assert!((rate - 0.8).abs() < 1e-10);
        let empty = DiffTestReport::new();
        assert_eq!(empty.pass_rate(), 1.0);
        assert!(empty.all_passed());
        let s = format!("{}", report);
        assert!(s.contains("10"));
        assert!(s.contains("80.0"));
    }
}
#[cfg(test)]
mod extended_tests {
    use super::*;
    use crate::differential_test::*;
    #[test]
    fn test_diff_corpus_serialize_deserialize() {
        let mut corpus = DiffCorpus::new("test_corpus", 1);
        corpus.add_tag("lean4");
        corpus.push(DiffTestCase::new_success("c1", "def x := 1"));
        corpus.push(DiffTestCase::new_failure("c2", ""));
        corpus.push(DiffTestCase::new_success_with_output(
            "c3", "input", "output",
        ));
        let serialized = corpus.serialize();
        let deserialized =
            DiffCorpus::deserialize(&serialized).expect("test operation should succeed");
        assert_eq!(deserialized.id, "test_corpus");
        assert_eq!(deserialized.version, 1);
        assert_eq!(deserialized.cases.len(), 3);
        assert_eq!(deserialized.tags, vec!["lean4".to_string()]);
        assert_eq!(deserialized.cases[0].name, "c1");
        assert!(deserialized.cases[0].should_succeed);
        assert!(!deserialized.cases[1].should_succeed);
        assert_eq!(
            deserialized.cases[2].expected_output,
            Some("output".to_string())
        );
    }
    #[test]
    fn test_diff_corpus_filter() {
        let mut corpus = DiffCorpus::new("all", 1);
        corpus.push(DiffTestCase::new_success("ok1", "valid"));
        corpus.push(DiffTestCase::new_success("ok2", "also valid"));
        corpus.push(DiffTestCase::new_failure("bad1", ""));
        let success_only = corpus.filter(|c| c.should_succeed);
        assert_eq!(success_only.len(), 2);
        let fail_only = corpus.filter(|c| !c.should_succeed);
        assert_eq!(fail_only.len(), 1);
    }
    #[test]
    fn test_diff_corpus_merge() {
        let mut a = DiffCorpus::new("a", 1);
        a.push(DiffTestCase::new_success("a1", "input"));
        a.add_tag("tagA");
        let mut b = DiffCorpus::new("b", 1);
        b.push(DiffTestCase::new_success("b1", "input2"));
        b.add_tag("tagA");
        b.add_tag("tagB");
        a.merge(&b);
        assert_eq!(a.len(), 2);
        assert_eq!(a.tags.len(), 2);
    }
    #[test]
    fn test_parametric_diff_case() {
        let template =
            ParametricDiffCase::new("add_{a}_{b}", "theorem add : {a} + {b} = {b} + {a}", true)
                .with_params(vec![("a", "0"), ("b", "1")])
                .with_params(vec![("a", "x"), ("b", "y")]);
        let cases = template.instantiate();
        assert_eq!(cases.len(), 2);
        assert_eq!(cases[0].name, "add_0_1");
        assert!(cases[0].input.contains("0 + 1 = 1 + 0"));
        assert_eq!(cases[1].name, "add_x_y");
        assert!(cases[1].input.contains("x + y = y + x"));
    }
    #[test]
    fn test_diff_test_harness_aggregate() {
        let mut harness = DiffTestHarness::new();
        let mut suite1 = DiffTestSuite::named("s1");
        suite1.add(DiffTestCase::new_success("t1", "valid"));
        suite1.add(DiffTestCase::new_success("t2", "valid2"));
        let mut suite2 = DiffTestSuite::named("s2");
        suite2.add(DiffTestCase::new_failure("t3", ""));
        harness.add_suite(suite1);
        harness.add_suite(suite2);
        assert_eq!(harness.total_cases(), 3);
        let report = harness.aggregate_report();
        assert_eq!(report.total, 3);
        assert!(report.passed > 0);
        assert_eq!(harness.suite_names(), vec!["s1", "s2"]);
    }
    #[test]
    fn test_regression_detector() {
        let baseline = vec![
            ("test_a".to_string(), DiffTestResult::Pass),
            ("test_b".to_string(), DiffTestResult::Pass),
            (
                "test_c".to_string(),
                DiffTestResult::Fail {
                    actual: "x".to_string(),
                    expected: "y".to_string(),
                },
            ),
        ];
        let current = vec![
            ("test_a".to_string(), DiffTestResult::Pass),
            (
                "test_b".to_string(),
                DiffTestResult::Fail {
                    actual: "wrong".to_string(),
                    expected: "right".to_string(),
                },
            ),
            ("test_c".to_string(), DiffTestResult::Pass),
        ];
        let mut detector = RegressionDetector::new();
        detector.set_baseline(baseline);
        let regressions = detector.detect(&current);
        assert_eq!(regressions.len(), 1);
        assert_eq!(regressions[0].name, "test_b");
        let improvements = detector.detect_improvements(&current);
        assert_eq!(improvements.len(), 1);
        assert_eq!(improvements[0], "test_c");
        assert_eq!(detector.baseline_len(), 3);
    }
    #[test]
    fn test_diff_test_statistics() {
        let results = vec![
            ("t1".to_string(), DiffTestResult::Pass),
            ("t2".to_string(), DiffTestResult::Pass),
            (
                "t3".to_string(),
                DiffTestResult::Fail {
                    actual: "a".to_string(),
                    expected: "b".to_string(),
                },
            ),
            ("t4".to_string(), DiffTestResult::Error("oops".to_string())),
        ];
        let stats = DiffTestStatistics::compute(&results);
        assert_eq!(stats.total, 4);
        assert_eq!(stats.passed, 2);
        assert_eq!(stats.failed, 1);
        assert_eq!(stats.errors, 1);
        assert!(!stats.all_passed());
        assert!((stats.pass_rate() - 0.5).abs() < 1e-10);
        let summary = stats.summary_line();
        assert!(summary.contains("2/4"));
    }
    #[test]
    fn test_diff_test_filter() {
        let mut suite = DiffTestSuite::named("suite");
        suite.add(DiffTestCase::new_success("lean_add", "def add := ..."));
        suite.add(DiffTestCase::new_success("lean_mul", "def mul := ..."));
        suite.add(DiffTestCase::new_failure("bad_empty", ""));
        let name_filter = DiffTestFilter::new().with_name("lean");
        let filtered = name_filter.apply(&suite);
        assert_eq!(filtered.len(), 2);
        let success_filter = DiffTestFilter::new().with_success(false);
        let fail_only = success_filter.apply(&suite);
        assert_eq!(fail_only.len(), 1);
        assert_eq!(fail_only.cases[0].name, "bad_empty");
        let input_filter = DiffTestFilter::new().with_input("def mul");
        let mul_only = input_filter.apply(&suite);
        assert_eq!(mul_only.len(), 1);
    }
    #[test]
    fn test_compare_strategy() {
        assert!(CompareStrategy::Exact.compare("hello", "hello"));
        assert!(!CompareStrategy::Exact.compare("hello ", "hello"));
        assert!(CompareStrategy::TrimWhitespace.compare("  hello  ", "hello"));
        assert!(!CompareStrategy::TrimWhitespace.compare("hi", "hello"));
        assert!(CompareStrategy::CaseInsensitive.compare("Hello", "hello"));
        assert!(!CompareStrategy::CaseInsensitive.compare("hi", "hello"));
        assert!(CompareStrategy::Contains.compare("hello world", "world"));
        assert!(!CompareStrategy::Contains.compare("hello", "world"));
        assert!(CompareStrategy::NormaliseSpaces.compare("a  b  c", "a b c"));
        assert!(!CompareStrategy::NormaliseSpaces.compare("a b", "a b c"));
    }
    #[test]
    fn test_diff_test_comparator() {
        let exact = DiffTestComparator::exact();
        assert!(exact.compare("foo", "foo"));
        assert!(!exact.compare("foo ", "foo"));
        assert_eq!(exact.strategy(), &CompareStrategy::Exact);
        let trimmed = DiffTestComparator::trimmed();
        assert!(trimmed.compare("  foo  ", "foo"));
        let contains = DiffTestComparator::contains();
        assert!(contains.compare("hello world", "world"));
        let normalised = DiffTestComparator::normalised();
        assert!(normalised.compare("a  b", "a b"));
    }
    #[test]
    fn test_snapshot_store_serialize_deserialize() {
        let mut store = SnapshotStore::new();
        store.set("test1", "output1");
        store.set("test2", "output2");
        assert_eq!(store.get("test1"), Some("output1"));
        assert!(store.has("test2"));
        assert_eq!(store.len(), 2);
        let serialized = store.serialize();
        let deserialized = SnapshotStore::deserialize(&serialized);
        assert_eq!(deserialized.get("test1"), Some("output1"));
        assert_eq!(deserialized.get("test2"), Some("output2"));
    }
    #[test]
    fn test_snapshot_store_merge() {
        let mut a = SnapshotStore::new();
        a.set("k1", "v1");
        let mut b = SnapshotStore::new();
        b.set("k1", "v1_new");
        b.set("k2", "v2");
        a.merge(&b);
        assert_eq!(a.get("k1"), Some("v1_new"));
        assert_eq!(a.get("k2"), Some("v2"));
        assert_eq!(a.len(), 2);
    }
    #[test]
    fn test_snapshot_store_remove() {
        let mut store = SnapshotStore::new();
        store.set("key", "val");
        assert!(store.has("key"));
        let removed = store.remove("key");
        assert_eq!(removed, Some("val".to_string()));
        assert!(!store.has("key"));
        assert!(store.is_empty());
    }
    #[test]
    fn test_harness_from_corpus() {
        let mut corpus = DiffCorpus::new("corpus1", 1);
        corpus.push(DiffTestCase::new_success("t1", "input1"));
        corpus.push(DiffTestCase::new_success("t2", "input2"));
        let mut harness = DiffTestHarness::new();
        harness.add_corpus(&corpus);
        assert_eq!(harness.total_cases(), 2);
        let report = harness.aggregate_report();
        assert_eq!(report.total, 2);
    }
}
#[cfg(test)]
mod matrix_and_scheduler_tests {
    use super::*;
    use crate::differential_test::*;
    #[test]
    fn test_diff_test_matrix_basic() {
        let rows = vec!["row1".to_string(), "row2".to_string()];
        let cols = vec!["col1".to_string(), "col2".to_string()];
        let mut matrix = DiffTestMatrix::new("test_matrix", rows, cols);
        matrix.set(0, 0, DiffTestResult::Pass);
        matrix.set(
            0,
            1,
            DiffTestResult::Fail {
                actual: "a".to_string(),
                expected: "b".to_string(),
            },
        );
        matrix.set(1, 0, DiffTestResult::Error("err".to_string()));
        assert!(matches!(matrix.get(0, 0), Some(DiffTestResult::Pass)));
        assert!(matches!(
            matrix.get(0, 1),
            Some(DiffTestResult::Fail { .. })
        ));
        assert!(matrix.get(1, 1).is_none());
        assert_eq!(matrix.count_pass(), 1);
        assert_eq!(matrix.count_set(), 3);
        let rendered = matrix.render();
        assert!(rendered.contains("test_matrix"));
        assert!(rendered.contains("PASS"));
        assert!(rendered.contains("FAIL"));
    }
    #[test]
    fn test_diff_test_scheduler_priority() {
        let mut scheduler = DiffTestScheduler::new();
        scheduler.add(
            ScheduledTest::new(DiffTestCase::new_success("low", "valid"))
                .with_priority(TestPriority::Low),
        );
        scheduler.add(
            ScheduledTest::new(DiffTestCase::new_success("critical", "valid"))
                .with_priority(TestPriority::Critical),
        );
        scheduler.add(
            ScheduledTest::new(DiffTestCase::new_success("normal", "valid"))
                .with_priority(TestPriority::Normal),
        );
        assert_eq!(scheduler.len(), 3);
        let results = scheduler.run();
        assert_eq!(results[0].0, "critical");
        assert_eq!(results[2].0, "low");
    }
    #[test]
    fn test_diff_test_scheduler_group() {
        let mut scheduler = DiffTestScheduler::new();
        scheduler
            .add(ScheduledTest::new(DiffTestCase::new_success("t1", "v")).with_group("algebra"));
        scheduler
            .add(ScheduledTest::new(DiffTestCase::new_success("t2", "v")).with_group("algebra"));
        scheduler.add(ScheduledTest::new(DiffTestCase::new_success("t3", "v")).with_group("logic"));
        let algebra = scheduler.group("algebra");
        assert_eq!(algebra.len(), 2);
        let logic = scheduler.group("logic");
        assert_eq!(logic.len(), 1);
    }
    #[test]
    fn test_annotated_diff_case() {
        let case = DiffTestCase::new_success("theorem_foo", "theorem foo : True");
        let annotated = AnnotatedDiffCase::new(case)
            .annotate("category", "logic")
            .annotate("source", "mathlib4")
            .annotate("difficulty", "easy");
        assert_eq!(annotated.get_annotation("category"), Some("logic"));
        assert_eq!(annotated.get_annotation("source"), Some("mathlib4"));
        assert!(annotated.get_annotation("nonexistent").is_none());
        let keys = annotated.annotation_keys();
        assert!(keys.contains(&"category"));
        assert!(keys.contains(&"difficulty"));
    }
    #[test]
    fn test_test_priority_ordering() {
        assert!(TestPriority::Critical > TestPriority::High);
        assert!(TestPriority::High > TestPriority::Normal);
        assert!(TestPriority::Normal > TestPriority::Low);
    }
    #[test]
    fn test_scheduler_empty() {
        let mut scheduler = DiffTestScheduler::new();
        assert!(scheduler.is_empty());
        let results = scheduler.run();
        assert!(results.is_empty());
    }
    #[test]
    fn test_matrix_out_of_bounds() {
        let rows = vec!["r1".to_string()];
        let cols = vec!["c1".to_string()];
        let mut matrix = DiffTestMatrix::new("tiny", rows, cols);
        matrix.set(10, 10, DiffTestResult::Pass);
        assert_eq!(matrix.count_set(), 0);
        assert!(matrix.get(10, 10).is_none());
    }
}
/// A transformation step applied to a `DiffTestSuite` before running.
pub trait SuiteTransform {
    /// Apply the transformation to the given suite, returning a new suite.
    fn transform(&self, suite: DiffTestSuite) -> DiffTestSuite;
    /// A name for this transformation step.
    fn name(&self) -> &str;
}
#[cfg(test)]
mod pipeline_and_history_tests {
    use super::*;
    use crate::differential_test::*;
    #[test]
    fn test_suite_pipeline_reverse() {
        let mut suite = DiffTestSuite::named("original");
        suite.add(DiffTestCase::new_success("first", "v1"));
        suite.add(DiffTestCase::new_success("second", "v2"));
        suite.add(DiffTestCase::new_success("third", "v3"));
        let mut pipeline = DiffTestPipeline::new();
        pipeline.add(ReverseOrderTransform);
        let result = pipeline.apply(suite);
        assert_eq!(result.cases[0].name, "third");
        assert_eq!(result.cases[2].name, "first");
    }
    #[test]
    fn test_suite_pipeline_dedup() {
        let mut suite = DiffTestSuite::named("with_dups");
        suite.add(DiffTestCase::new_success("t1", "v1"));
        suite.add(DiffTestCase::new_success("t1", "v1_dup"));
        suite.add(DiffTestCase::new_success("t2", "v2"));
        let mut pipeline = DiffTestPipeline::new();
        pipeline.add(DeduplicateTransform);
        let result = pipeline.apply(suite);
        assert_eq!(result.cases.len(), 2);
        assert_eq!(result.cases[0].name, "t1");
        assert_eq!(result.cases[0].input, "v1");
    }
    #[test]
    fn test_suite_pipeline_limit() {
        let mut suite = DiffTestSuite::named("big");
        for i in 0..10 {
            suite.add(DiffTestCase::new_success(format!("t{}", i), "v"));
        }
        let mut pipeline = DiffTestPipeline::new();
        pipeline.add(LimitTransform::new(3));
        let result = pipeline.apply(suite);
        assert_eq!(result.cases.len(), 3);
    }
    #[test]
    fn test_suite_pipeline_chained() {
        let mut suite = DiffTestSuite::named("chain");
        for i in 0..5 {
            suite.add(DiffTestCase::new_success(format!("t{}", i), "v"));
        }
        suite.add(DiffTestCase::new_success("t0", "v_dup"));
        let mut pipeline = DiffTestPipeline::new();
        pipeline.add(DeduplicateTransform);
        pipeline.add(ReverseOrderTransform);
        pipeline.add(LimitTransform::new(3));
        let result = pipeline.apply(suite);
        assert_eq!(result.cases.len(), 3);
        assert_eq!(result.cases[0].name, "t4");
        let names = pipeline.step_names();
        assert_eq!(names, vec!["deduplicate", "reverse_order", "limit"]);
    }
    #[test]
    fn test_diff_test_history_basic() {
        let mut history = DiffTestHistory::new();
        assert!(history.is_empty());
        let snap1 = TestRunSnapshot::new(
            "run_001",
            vec![
                ("t1".to_string(), DiffTestResult::Pass),
                (
                    "t2".to_string(),
                    DiffTestResult::Fail {
                        actual: "a".to_string(),
                        expected: "b".to_string(),
                    },
                ),
            ],
        );
        let snap2 = TestRunSnapshot::new(
            "run_002",
            vec![
                ("t1".to_string(), DiffTestResult::Pass),
                ("t2".to_string(), DiffTestResult::Pass),
            ],
        );
        history.record(snap1);
        history.record(snap2);
        assert_eq!(history.len(), 2);
        assert_eq!(
            history
                .latest()
                .expect("test operation should succeed")
                .run_id,
            "run_002"
        );
        assert_eq!(
            history
                .oldest()
                .expect("test operation should succeed")
                .run_id,
            "run_001"
        );
        let (pass, total) = history.trend("t1");
        assert_eq!(pass, 2);
        assert_eq!(total, 2);
        let (pass2, total2) = history.trend("t2");
        assert_eq!(pass2, 1);
        assert_eq!(total2, 2);
        let ids = history.run_ids();
        assert_eq!(ids, vec!["run_001", "run_002"]);
    }
    #[test]
    fn test_test_run_snapshot_statistics() {
        let snap = TestRunSnapshot::new(
            "snap",
            vec![
                ("t1".to_string(), DiffTestResult::Pass),
                ("t2".to_string(), DiffTestResult::Pass),
                ("t3".to_string(), DiffTestResult::Error("err".to_string())),
            ],
        );
        let stats = snap.statistics();
        assert_eq!(stats.total, 3);
        assert_eq!(stats.passed, 2);
        assert_eq!(stats.errors, 1);
        assert!(!stats.all_passed());
    }
    #[test]
    fn test_pipeline_empty() {
        let pipeline = DiffTestPipeline::new();
        assert!(pipeline.is_empty());
        assert_eq!(pipeline.len(), 0);
        assert!(pipeline.step_names().is_empty());
        let mut suite = DiffTestSuite::named("passthrough");
        suite.add(DiffTestCase::new_success("t", "v"));
        let result = pipeline.apply(suite);
        assert_eq!(result.cases.len(), 1);
    }
}
#[cfg(test)]
mod metrics_tests {
    use super::*;
    use crate::differential_test::*;
    #[test]
    fn test_test_timing() {
        let t = TestTiming::new(5000);
        assert_eq!(t.duration_us, 5000);
        assert!(!t.timed_out);
        assert!((t.duration_ms() - 5.0).abs() < 1e-9);
        let to = TestTiming::timed_out();
        assert!(to.timed_out);
    }
    #[test]
    fn test_diff_test_metrics() {
        let mut metrics = DiffTestMetrics::new();
        metrics.record("fast_test", TestTiming::new(100));
        metrics.record("slow_test", TestTiming::new(5000));
        metrics.record("timeout_test", TestTiming::timed_out());
        assert_eq!(metrics.timeout_count(), 1);
        assert_eq!(metrics.total_duration_us, 5100);
        let avg = metrics.average_duration_us();
        assert!((avg - 5100.0 / 3.0).abs() < 1.0);
        let slowest = metrics.slowest().expect("test operation should succeed");
        assert_eq!(slowest.0, "slow_test");
        let fastest = metrics.fastest().expect("test operation should succeed");
        assert_eq!(fastest.0, "fast_test");
    }
    #[test]
    fn test_metrics_empty() {
        let metrics = DiffTestMetrics::new();
        assert_eq!(metrics.average_duration_us(), 0.0);
        assert!(metrics.slowest().is_none());
        assert!(metrics.fastest().is_none());
        assert_eq!(metrics.timeout_count(), 0);
    }
    #[test]
    fn test_multi_line_output_match() {
        let mlo = MultiLineOutput::new("line1\nline2\nline3", "line1\nline2\nline3");
        assert!(mlo.matches());
        assert!(mlo.missing_lines().is_empty());
        assert!(mlo.extra_lines().is_empty());
    }
    #[test]
    fn test_multi_line_output_diff() {
        let mlo = MultiLineOutput::new("line1\nline2", "line1\nline3");
        assert!(!mlo.matches());
        assert_eq!(mlo.missing_lines(), vec!["line2"]);
        assert_eq!(mlo.extra_lines(), vec!["line3"]);
        let summary = mlo.diff_summary();
        assert!(summary.contains("- line2"));
        assert!(summary.contains("+ line3"));
    }
    #[test]
    fn test_multi_line_output_different_lengths() {
        let mlo = MultiLineOutput::new("a\nb\nc", "a\nb");
        assert!(!mlo.matches());
        let missing = mlo.missing_lines();
        assert!(missing.contains(&"c"));
    }
}
#[cfg(test)]
mod tag_registry_tests {
    use super::*;
    use crate::differential_test::*;
    #[test]
    fn test_tag_registry_basic() {
        let mut reg = DiffTestTagRegistry::new();
        reg.tag("test_add", "algebra");
        reg.tag("test_mul", "algebra");
        reg.tag("test_add", "arithmetic");
        let algebra = reg.tests_for_tag("algebra");
        assert_eq!(algebra.len(), 2);
        let arithmetic = reg.tests_for_tag("arithmetic");
        assert_eq!(arithmetic.len(), 1);
        assert_eq!(arithmetic[0], "test_add");
        assert_eq!(reg.tag_count(), 2);
        let tags = reg.all_tags();
        assert_eq!(tags, vec!["algebra", "arithmetic"]);
    }
    #[test]
    fn test_tag_registry_intersection() {
        let mut reg = DiffTestTagRegistry::new();
        reg.tag("t1", "algebra");
        reg.tag("t1", "logic");
        reg.tag("t2", "algebra");
        reg.tag("t3", "logic");
        let both = reg.tests_with_all_tags(&["algebra", "logic"]);
        assert_eq!(both.len(), 1);
        assert!(both.contains(&"t1".to_string()));
        let empty = reg.tests_with_all_tags(&[]);
        assert!(empty.is_empty());
    }
    #[test]
    fn test_tag_registry_missing_tag() {
        let reg = DiffTestTagRegistry::new();
        let result = reg.tests_for_tag("nonexistent");
        assert!(result.is_empty());
    }
}
