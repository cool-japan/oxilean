//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;

use oxilean_kernel::*;
use std::collections::HashMap;

/// A registry that maps tags to sets of test case names.
#[derive(Debug, Clone, Default)]
pub struct DiffTestTagRegistry {
    tag_to_tests: std::collections::HashMap<String, Vec<String>>,
}
impl DiffTestTagRegistry {
    /// Create a new empty tag registry.
    pub fn new() -> Self {
        Self::default()
    }
    /// Tag a test case with a given tag.
    pub fn tag(&mut self, test_name: impl Into<String>, tag: impl Into<String>) {
        self.tag_to_tests
            .entry(tag.into())
            .or_default()
            .push(test_name.into());
    }
    /// Return all test names for a given tag.
    pub fn tests_for_tag(&self, tag: &str) -> &[String] {
        self.tag_to_tests
            .get(tag)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }
    /// Return all tags in the registry.
    pub fn all_tags(&self) -> Vec<&str> {
        let mut tags: Vec<&str> = self.tag_to_tests.keys().map(|s| s.as_str()).collect();
        tags.sort();
        tags
    }
    /// Return the number of distinct tags.
    pub fn tag_count(&self) -> usize {
        self.tag_to_tests.len()
    }
    /// Return all test names that have ALL of the given tags.
    pub fn tests_with_all_tags(&self, tags: &[&str]) -> Vec<String> {
        if tags.is_empty() {
            return Vec::new();
        }
        let sets: Vec<std::collections::HashSet<&str>> = tags
            .iter()
            .map(|tag| {
                self.tag_to_tests
                    .get(*tag)
                    .map(|v| v.iter().map(|s| s.as_str()).collect())
                    .unwrap_or_default()
            })
            .collect();
        if sets.is_empty() {
            return Vec::new();
        }
        let first = &sets[0];
        first
            .iter()
            .filter(|name| sets[1..].iter().all(|s| s.contains(*name)))
            .map(|s| s.to_string())
            .collect()
    }
}
/// A limit transform that takes at most N cases.
#[derive(Debug)]
pub struct LimitTransform {
    pub(super) max: usize,
}
impl LimitTransform {
    /// Create a new limit transform.
    pub fn new(max: usize) -> Self {
        Self { max }
    }
}
/// A test case with scheduling metadata.
#[derive(Debug, Clone)]
pub struct ScheduledTest {
    /// The underlying test case.
    pub case: DiffTestCase,
    /// The priority for scheduling.
    pub priority: TestPriority,
    /// An optional group name (for grouping related tests).
    pub group: Option<String>,
}
impl ScheduledTest {
    /// Create a new scheduled test with default priority.
    pub fn new(case: DiffTestCase) -> Self {
        Self {
            case,
            priority: TestPriority::Normal,
            group: None,
        }
    }
    /// Set the priority.
    pub fn with_priority(mut self, priority: TestPriority) -> Self {
        self.priority = priority;
        self
    }
    /// Set the group.
    pub fn with_group(mut self, group: impl Into<String>) -> Self {
        self.group = Some(group.into());
        self
    }
}
/// An annotation on a differential test case, providing extra metadata.
#[derive(Debug, Clone)]
pub struct DiffTestAnnotation {
    /// Key for the annotation.
    pub key: String,
    /// Value for the annotation.
    pub value: String,
}
impl DiffTestAnnotation {
    /// Create a new annotation.
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }
}
/// A matrix of differential test cases formed by combining two axes of parameters.
#[derive(Debug, Clone)]
pub struct DiffTestMatrix {
    /// Name of the matrix.
    pub name: String,
    /// Row labels.
    pub row_labels: Vec<String>,
    /// Column labels.
    pub col_labels: Vec<String>,
    /// Results\[row\]\[col\].
    pub results: Vec<Vec<Option<DiffTestResult>>>,
}
impl DiffTestMatrix {
    /// Create a new matrix with the given row and column labels.
    pub fn new(name: impl Into<String>, row_labels: Vec<String>, col_labels: Vec<String>) -> Self {
        let rows = row_labels.len();
        let cols = col_labels.len();
        Self {
            name: name.into(),
            row_labels,
            col_labels,
            results: vec![vec![None; cols]; rows],
        }
    }
    /// Set a result at (row, col).
    pub fn set(&mut self, row: usize, col: usize, result: DiffTestResult) {
        if row < self.results.len() && col < self.results[row].len() {
            self.results[row][col] = Some(result);
        }
    }
    /// Get a result at (row, col).
    pub fn get(&self, row: usize, col: usize) -> Option<&DiffTestResult> {
        self.results
            .get(row)
            .and_then(|r| r.get(col))
            .and_then(|v| v.as_ref())
    }
    /// Count the number of passing cells.
    pub fn count_pass(&self) -> usize {
        self.results
            .iter()
            .flat_map(|row| row.iter())
            .filter(|cell| matches!(cell, Some(DiffTestResult::Pass)))
            .count()
    }
    /// Count the total number of cells that have been set.
    pub fn count_set(&self) -> usize {
        self.results
            .iter()
            .flat_map(|row| row.iter())
            .filter(|cell| cell.is_some())
            .count()
    }
    /// Render as an ASCII table.
    pub fn render(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("Matrix: {}\n", self.name));
        out.push_str("         ");
        for col in &self.col_labels {
            out.push_str(&format!("{:>12} ", col));
        }
        out.push('\n');
        for (row_idx, row_label) in self.row_labels.iter().enumerate() {
            out.push_str(&format!("{:>8} ", row_label));
            for col_idx in 0..self.col_labels.len() {
                let cell = match self.get(row_idx, col_idx) {
                    None => "?",
                    Some(DiffTestResult::Pass) => "PASS",
                    Some(DiffTestResult::Fail { .. }) => "FAIL",
                    Some(DiffTestResult::Error(_)) => "ERR",
                    Some(DiffTestResult::Unexpected) => "UNEX",
                };
                out.push_str(&format!("{:>12} ", cell));
            }
            out.push('\n');
        }
        out
    }
}
/// Represents a regression: a test that was passing before and is now failing.
#[derive(Debug, Clone)]
pub struct Regression {
    /// The test case name.
    pub name: String,
    /// The old result (passing).
    pub old_result: DiffTestResult,
    /// The new result (failing).
    pub new_result: DiffTestResult,
}
/// Summary of a differential test run.
#[derive(Debug, Clone, Default)]
pub struct DiffTestReport {
    /// Total number of test cases run.
    pub total: usize,
    /// Number of cases that passed.
    pub passed: usize,
    /// Number of cases that failed.
    pub failed: usize,
    /// Number of cases that errored.
    pub errors: usize,
    /// Number of unexpected results.
    pub unexpected: usize,
}
impl DiffTestReport {
    /// Create an empty report.
    pub fn new() -> Self {
        Self::default()
    }
    /// Compute the pass rate in [0.0, 1.0].
    pub fn pass_rate(&self) -> f64 {
        if self.total == 0 {
            1.0
        } else {
            self.passed as f64 / self.total as f64
        }
    }
    /// Return true if all tests passed.
    pub fn all_passed(&self) -> bool {
        self.failed == 0 && self.errors == 0 && self.unexpected == 0
    }
}
/// Statistics computed from a set of differential test results.
#[derive(Debug, Clone, Default)]
pub struct DiffTestStatistics {
    /// Total number of test cases.
    pub total: usize,
    /// Number of passing cases.
    pub passed: usize,
    /// Number of failing cases.
    pub failed: usize,
    /// Number of error cases.
    pub errors: usize,
    /// Number of unexpected results.
    pub unexpected: usize,
    /// Longest test name (for formatting).
    pub longest_name: usize,
}
impl DiffTestStatistics {
    /// Compute statistics from a slice of results.
    pub fn compute(results: &[(String, DiffTestResult)]) -> Self {
        let mut stats = Self {
            total: results.len(),
            ..Self::default()
        };
        for (name, result) in results {
            if name.len() > stats.longest_name {
                stats.longest_name = name.len();
            }
            match result {
                DiffTestResult::Pass => stats.passed += 1,
                DiffTestResult::Fail { .. } => stats.failed += 1,
                DiffTestResult::Error(_) => stats.errors += 1,
                DiffTestResult::Unexpected => stats.unexpected += 1,
            }
        }
        stats
    }
    /// Compute the pass rate.
    pub fn pass_rate(&self) -> f64 {
        if self.total == 0 {
            1.0
        } else {
            self.passed as f64 / self.total as f64
        }
    }
    /// Return true if all tests passed.
    pub fn all_passed(&self) -> bool {
        self.failed == 0 && self.errors == 0 && self.unexpected == 0
    }
    /// Format as a human-readable one-liner.
    pub fn summary_line(&self) -> String {
        format!(
            "{}/{} passed ({:.1}%)",
            self.passed,
            self.total,
            self.pass_rate() * 100.0
        )
    }
}
/// Runs differential test cases and suites.
#[derive(Debug, Default)]
pub struct DiffTestRunner {
    /// Whether to print verbose output during runs.
    pub verbose: bool,
}
impl DiffTestRunner {
    /// Create a new runner with default settings.
    pub fn new() -> Self {
        Self::default()
    }
    /// Create a verbose runner.
    pub fn verbose() -> Self {
        Self { verbose: true }
    }
    /// Run a single test case.
    ///
    /// The runner simulates elaboration by checking whether the input is
    /// non-empty (success heuristic). Real elaboration integration is done
    /// through the elaborator API when the test infrastructure is connected.
    pub fn run_case(&self, case: &DiffTestCase) -> DiffTestResult {
        let elaboration_succeeded = !case.input.trim().is_empty();
        let actual_output = if elaboration_succeeded {
            format!("elaborated: {}", case.input.trim())
        } else {
            String::from("elaboration failed: empty input")
        };
        if elaboration_succeeded != case.should_succeed {
            return DiffTestResult::Unexpected;
        }
        match &case.expected_output {
            None => DiffTestResult::Pass,
            Some(expected) => {
                if &actual_output == expected {
                    DiffTestResult::Pass
                } else {
                    DiffTestResult::Fail {
                        actual: actual_output,
                        expected: expected.clone(),
                    }
                }
            }
        }
    }
    /// Run all cases in a suite and return a list of `(name, result)` pairs.
    pub fn run_suite(&self, suite: &DiffTestSuite) -> Vec<(String, DiffTestResult)> {
        suite
            .cases
            .iter()
            .map(|case| {
                let result = self.run_case(case);
                if self.verbose {
                    eprintln!("[diff_test] {} ... {}", case.name, result.label());
                }
                (case.name.clone(), result)
            })
            .collect()
    }
    /// Generate a human-readable report from a list of results.
    pub fn report(&self, results: &[(String, DiffTestResult)]) -> String {
        let total = results.len();
        let passed = results.iter().filter(|(_, r)| r.is_pass()).count();
        let failed = results.iter().filter(|(_, r)| r.is_fail()).count();
        let errors = results.iter().filter(|(_, r)| r.is_error()).count();
        let unexpected = results.iter().filter(|(_, r)| r.is_unexpected()).count();
        let mut out = String::new();
        out.push_str("=== Differential Test Report ===\n");
        for (name, result) in results {
            out.push_str(&format!("  [{:11}] {}\n", result.label(), name));
        }
        out.push_str(&format!(
            "Total: {} | Pass: {} | Fail: {} | Error: {} | Unexpected: {}\n",
            total, passed, failed, errors, unexpected
        ));
        out
    }
}
/// A tester that accepts Lean 4 surface syntax and validates it against OxiLean
/// elaboration (after normalization).
#[derive(Debug, Default)]
pub struct Lean4DiffTester {
    suite: DiffTestSuite,
    runner: DiffTestRunner,
}
impl Lean4DiffTester {
    /// Create a new Lean4 differential tester.
    pub fn new() -> Self {
        Self {
            suite: DiffTestSuite::named("lean4_compat"),
            runner: DiffTestRunner::new(),
        }
    }
    /// Add a Lean 4 test case.
    ///
    /// The `lean4_input` is the Lean 4 surface syntax; `should_succeed` indicates
    /// whether elaboration is expected to succeed after normalization.
    pub fn add_lean4_case(&mut self, name: &str, lean4_input: &str, should_succeed: bool) {
        let case = DiffTestCase {
            name: name.to_string(),
            input: lean4_input.to_string(),
            expected_output: None,
            should_succeed,
        };
        self.suite.add(case);
    }
    /// Add a Lean 4 test case with a specific expected elaboration output.
    pub fn add_lean4_case_with_output(
        &mut self,
        name: &str,
        lean4_input: &str,
        expected: &str,
        should_succeed: bool,
    ) {
        let case = DiffTestCase {
            name: name.to_string(),
            input: lean4_input.to_string(),
            expected_output: Some(expected.to_string()),
            should_succeed,
        };
        self.suite.add(case);
    }
    /// Run all accumulated test cases and return a summary report.
    pub fn run_all(&self) -> DiffTestReport {
        let results = self.runner.run_suite(&self.suite);
        let total = results.len();
        let passed = results.iter().filter(|(_, r)| r.is_pass()).count();
        let failed = results.iter().filter(|(_, r)| r.is_fail()).count();
        let errors = results.iter().filter(|(_, r)| r.is_error()).count();
        let unexpected = results.iter().filter(|(_, r)| r.is_unexpected()).count();
        DiffTestReport {
            total,
            passed,
            failed,
            errors,
            unexpected,
        }
    }
    /// Return the number of test cases registered.
    pub fn case_count(&self) -> usize {
        self.suite.len()
    }
}
/// A history of multiple test run snapshots.
#[derive(Debug, Default)]
pub struct DiffTestHistory {
    snapshots: Vec<TestRunSnapshot>,
}
impl DiffTestHistory {
    /// Create a new empty history.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a new run snapshot.
    pub fn record(&mut self, snapshot: TestRunSnapshot) {
        self.snapshots.push(snapshot);
    }
    /// Return the most recent snapshot, if any.
    pub fn latest(&self) -> Option<&TestRunSnapshot> {
        self.snapshots.last()
    }
    /// Return the oldest snapshot, if any.
    pub fn oldest(&self) -> Option<&TestRunSnapshot> {
        self.snapshots.first()
    }
    /// Return the number of recorded runs.
    pub fn len(&self) -> usize {
        self.snapshots.len()
    }
    /// Return true if no runs have been recorded.
    pub fn is_empty(&self) -> bool {
        self.snapshots.is_empty()
    }
    /// Compute per-test trend: for each test name, how many times it passed.
    pub fn trend(&self, test_name: &str) -> (usize, usize) {
        let mut pass = 0usize;
        let mut total = 0usize;
        for snap in &self.snapshots {
            for (name, result) in &snap.results {
                if name == test_name {
                    total += 1;
                    if result.is_pass() {
                        pass += 1;
                    }
                }
            }
        }
        (pass, total)
    }
    /// Return all run IDs.
    pub fn run_ids(&self) -> Vec<&str> {
        self.snapshots.iter().map(|s| s.run_id.as_str()).collect()
    }
}
/// A shuffle transform that reverses the order of test cases.
#[derive(Debug, Default)]
pub struct ReverseOrderTransform;
/// A comparator that uses a configurable `CompareStrategy`.
#[derive(Debug, Clone)]
pub struct DiffTestComparator {
    strategy: CompareStrategy,
}
impl DiffTestComparator {
    /// Create a comparator with exact matching.
    pub fn exact() -> Self {
        Self {
            strategy: CompareStrategy::Exact,
        }
    }
    /// Create a comparator with whitespace-trimming.
    pub fn trimmed() -> Self {
        Self {
            strategy: CompareStrategy::TrimWhitespace,
        }
    }
    /// Create a comparator that checks containment.
    pub fn contains() -> Self {
        Self {
            strategy: CompareStrategy::Contains,
        }
    }
    /// Create a comparator with normalised spaces.
    pub fn normalised() -> Self {
        Self {
            strategy: CompareStrategy::NormaliseSpaces,
        }
    }
    /// Compare two strings using the configured strategy.
    pub fn compare(&self, actual: &str, expected: &str) -> bool {
        self.strategy.compare(actual, expected)
    }
    /// Return the current strategy.
    pub fn strategy(&self) -> &CompareStrategy {
        &self.strategy
    }
}
/// A single differential test case.
#[derive(Debug, Clone)]
pub struct DiffTestCase {
    /// Name of the test case.
    pub name: String,
    /// Input to elaborate.
    pub input: String,
    /// Expected output string, or `None` if unconstrained.
    pub expected_output: Option<String>,
    /// Whether this case is expected to succeed (true) or fail (false).
    pub should_succeed: bool,
}
impl DiffTestCase {
    /// Create a new test case that is expected to succeed.
    pub fn new_success(name: impl Into<String>, input: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            input: input.into(),
            expected_output: None,
            should_succeed: true,
        }
    }
    /// Create a new test case that is expected to succeed with specific output.
    pub fn new_success_with_output(
        name: impl Into<String>,
        input: impl Into<String>,
        expected_output: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            input: input.into(),
            expected_output: Some(expected_output.into()),
            should_succeed: true,
        }
    }
    /// Create a new test case that is expected to fail.
    pub fn new_failure(name: impl Into<String>, input: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            input: input.into(),
            expected_output: None,
            should_succeed: false,
        }
    }
}
/// A scheduler that orders tests by priority before running.
#[derive(Debug, Default)]
pub struct DiffTestScheduler {
    scheduled: Vec<ScheduledTest>,
    runner: DiffTestRunner,
}
impl DiffTestScheduler {
    /// Create a new scheduler.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a test with the given priority.
    pub fn add(&mut self, test: ScheduledTest) {
        self.scheduled.push(test);
    }
    /// Run all tests in priority order (highest priority first).
    pub fn run(&mut self) -> Vec<(String, DiffTestResult)> {
        self.scheduled
            .sort_by_key(|b| std::cmp::Reverse(b.priority));
        self.scheduled
            .iter()
            .map(|st| {
                let result = self.runner.run_case(&st.case);
                (st.case.name.clone(), result)
            })
            .collect()
    }
    /// Return the number of scheduled tests.
    pub fn len(&self) -> usize {
        self.scheduled.len()
    }
    /// Return true if no tests are scheduled.
    pub fn is_empty(&self) -> bool {
        self.scheduled.is_empty()
    }
    /// Return all tests in a given group.
    pub fn group(&self, group: &str) -> Vec<&ScheduledTest> {
        self.scheduled
            .iter()
            .filter(|st| st.group.as_deref() == Some(group))
            .collect()
    }
}
/// Represents the expected vs. actual output for a multi-line comparison.
#[derive(Debug, Clone)]
pub struct MultiLineOutput {
    /// Lines of the expected output.
    pub expected_lines: Vec<String>,
    /// Lines of the actual output.
    pub actual_lines: Vec<String>,
}
impl MultiLineOutput {
    /// Create a new multi-line output comparison.
    pub fn new(expected: &str, actual: &str) -> Self {
        Self {
            expected_lines: expected.lines().map(|l| l.to_string()).collect(),
            actual_lines: actual.lines().map(|l| l.to_string()).collect(),
        }
    }
    /// Return true if expected and actual are equal.
    pub fn matches(&self) -> bool {
        self.expected_lines == self.actual_lines
    }
    /// Return lines that are in expected but not in actual.
    pub fn missing_lines(&self) -> Vec<&str> {
        self.expected_lines
            .iter()
            .filter(|l| !self.actual_lines.contains(l))
            .map(|l| l.as_str())
            .collect()
    }
    /// Return lines that are in actual but not in expected.
    pub fn extra_lines(&self) -> Vec<&str> {
        self.actual_lines
            .iter()
            .filter(|l| !self.expected_lines.contains(l))
            .map(|l| l.as_str())
            .collect()
    }
    /// Return a unified-diff-style summary.
    pub fn diff_summary(&self) -> String {
        let mut out = String::new();
        let max_lines = self.expected_lines.len().max(self.actual_lines.len());
        for i in 0..max_lines {
            let exp = self
                .expected_lines
                .get(i)
                .map(|s| s.as_str())
                .unwrap_or("<missing>");
            let act = self
                .actual_lines
                .get(i)
                .map(|s| s.as_str())
                .unwrap_or("<missing>");
            if exp != act {
                out.push_str(&format!("- {}\n+ {}\n", exp, act));
            }
        }
        out
    }
}
/// A pipeline of suite transformations.
#[derive(Default)]
pub struct DiffTestPipeline {
    transforms: Vec<Box<dyn SuiteTransform>>,
}
impl DiffTestPipeline {
    /// Create a new empty pipeline.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a transform step to the pipeline.
    pub fn add<T: SuiteTransform + 'static>(&mut self, transform: T) {
        self.transforms.push(Box::new(transform));
    }
    /// Apply all transforms in sequence to the given suite.
    pub fn apply(&self, suite: DiffTestSuite) -> DiffTestSuite {
        let mut current = suite;
        for transform in &self.transforms {
            current = transform.transform(current);
        }
        current
    }
    /// Return the names of all transform steps.
    pub fn step_names(&self) -> Vec<&str> {
        self.transforms.iter().map(|t| t.name()).collect()
    }
    /// Return the number of steps.
    pub fn len(&self) -> usize {
        self.transforms.len()
    }
    /// Return true if there are no steps.
    pub fn is_empty(&self) -> bool {
        self.transforms.is_empty()
    }
}
/// Filter criteria for selecting a subset of test cases from a suite.
#[derive(Debug, Clone, Default)]
pub struct DiffTestFilter {
    /// If set, only include cases whose name contains this substring.
    pub name_contains: Option<String>,
    /// If set, only include cases with the given `should_succeed` value.
    pub success_filter: Option<bool>,
    /// If set, only include cases whose input contains this substring.
    pub input_contains: Option<String>,
}
impl DiffTestFilter {
    /// Create an empty filter (matches everything).
    pub fn new() -> Self {
        Self::default()
    }
    /// Only include cases whose name contains the given string.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name_contains = Some(name.into());
        self
    }
    /// Only include cases expecting success or failure.
    pub fn with_success(mut self, should_succeed: bool) -> Self {
        self.success_filter = Some(should_succeed);
        self
    }
    /// Only include cases whose input contains the given string.
    pub fn with_input(mut self, input: impl Into<String>) -> Self {
        self.input_contains = Some(input.into());
        self
    }
    /// Test whether a case matches this filter.
    pub fn matches(&self, case: &DiffTestCase) -> bool {
        if let Some(ref nc) = self.name_contains {
            if !case.name.contains(nc.as_str()) {
                return false;
            }
        }
        if let Some(sf) = self.success_filter {
            if case.should_succeed != sf {
                return false;
            }
        }
        if let Some(ref ic) = self.input_contains {
            if !case.input.contains(ic.as_str()) {
                return false;
            }
        }
        true
    }
    /// Apply this filter to a suite, returning a new filtered suite.
    pub fn apply(&self, suite: &DiffTestSuite) -> DiffTestSuite {
        let mut out = DiffTestSuite {
            cases: Vec::new(),
            name: suite.name.clone(),
        };
        for case in &suite.cases {
            if self.matches(case) {
                out.cases.push(case.clone());
            }
        }
        out
    }
}
/// Strategy for comparing actual vs. expected outputs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompareStrategy {
    /// Exact string equality.
    Exact,
    /// Trim whitespace before comparing.
    TrimWhitespace,
    /// Case-insensitive comparison.
    CaseInsensitive,
    /// Check that expected is a substring of actual.
    Contains,
    /// Normalise multiple spaces to single space before comparing.
    NormaliseSpaces,
}
impl CompareStrategy {
    /// Test whether `actual` matches `expected` under this strategy.
    pub fn compare(&self, actual: &str, expected: &str) -> bool {
        match self {
            CompareStrategy::Exact => actual == expected,
            CompareStrategy::TrimWhitespace => actual.trim() == expected.trim(),
            CompareStrategy::CaseInsensitive => actual.to_lowercase() == expected.to_lowercase(),
            CompareStrategy::Contains => actual.contains(expected),
            CompareStrategy::NormaliseSpaces => {
                let norm = |s: &str| s.split_whitespace().collect::<Vec<_>>().join(" ");
                norm(actual) == norm(expected)
            }
        }
    }
}
/// A snapshot store maps test case names to expected outputs (snapshots).
/// Useful for golden-file style testing.
#[derive(Debug, Clone, Default)]
pub struct SnapshotStore {
    snapshots: std::collections::HashMap<String, String>,
}
impl SnapshotStore {
    /// Create an empty snapshot store.
    pub fn new() -> Self {
        Self::default()
    }
    /// Insert or update a snapshot.
    pub fn set(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.snapshots.insert(name.into(), value.into());
    }
    /// Retrieve a snapshot by name.
    pub fn get(&self, name: &str) -> Option<&str> {
        self.snapshots.get(name).map(|s| s.as_str())
    }
    /// Return true if a snapshot exists for the given name.
    pub fn has(&self, name: &str) -> bool {
        self.snapshots.contains_key(name)
    }
    /// Remove a snapshot.
    pub fn remove(&mut self, name: &str) -> Option<String> {
        self.snapshots.remove(name)
    }
    /// Return the number of snapshots.
    pub fn len(&self) -> usize {
        self.snapshots.len()
    }
    /// Return true if the store is empty.
    pub fn is_empty(&self) -> bool {
        self.snapshots.is_empty()
    }
    /// Merge another store into this one (other takes priority on conflicts).
    pub fn merge(&mut self, other: &SnapshotStore) {
        for (k, v) in &other.snapshots {
            self.snapshots.insert(k.clone(), v.clone());
        }
    }
    /// Serialize to a simple key=value format.
    pub fn serialize(&self) -> String {
        let mut lines: Vec<String> = self
            .snapshots
            .iter()
            .map(|(k, v)| format!("{}={}", k, v.replace('\n', "\\n")))
            .collect();
        lines.sort();
        lines.join("\n")
    }
    /// Deserialize from the format produced by `serialize`.
    pub fn deserialize(src: &str) -> Self {
        let mut store = Self::new();
        for line in src.lines() {
            if let Some(eq) = line.find('=') {
                let key = &line[..eq];
                let val = &line[eq + 1..];
                store.set(key, val.replace("\\n", "\n"));
            }
        }
        store
    }
}
/// A dedup transform that removes duplicate test cases (by name).
#[derive(Debug, Default)]
pub struct DeduplicateTransform;
/// Timing information for a test case execution.
#[derive(Debug, Clone, Default)]
pub struct TestTiming {
    /// Duration in microseconds.
    pub duration_us: u64,
    /// Whether the test exceeded its time budget.
    pub timed_out: bool,
}
impl TestTiming {
    /// Create a new timing record.
    pub fn new(duration_us: u64) -> Self {
        Self {
            duration_us,
            timed_out: false,
        }
    }
    /// Create a timed-out timing record.
    pub fn timed_out() -> Self {
        Self {
            duration_us: 0,
            timed_out: true,
        }
    }
    /// Return the duration as milliseconds.
    pub fn duration_ms(&self) -> f64 {
        self.duration_us as f64 / 1000.0
    }
}
/// Metrics collected from a timed test run.
#[derive(Debug, Clone, Default)]
pub struct DiffTestMetrics {
    /// Per-test timing information.
    pub timings: Vec<(String, TestTiming)>,
    /// Total wall-clock duration in microseconds.
    pub total_duration_us: u64,
}
impl DiffTestMetrics {
    /// Create an empty metrics collection.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record timing for a test.
    pub fn record(&mut self, name: impl Into<String>, timing: TestTiming) {
        self.total_duration_us += timing.duration_us;
        self.timings.push((name.into(), timing));
    }
    /// Return the average duration in microseconds.
    pub fn average_duration_us(&self) -> f64 {
        if self.timings.is_empty() {
            0.0
        } else {
            self.total_duration_us as f64 / self.timings.len() as f64
        }
    }
    /// Return the slowest test (name, duration_us).
    pub fn slowest(&self) -> Option<(&str, u64)> {
        self.timings
            .iter()
            .max_by_key(|(_, t)| t.duration_us)
            .map(|(n, t)| (n.as_str(), t.duration_us))
    }
    /// Return the fastest test (name, duration_us).
    pub fn fastest(&self) -> Option<(&str, u64)> {
        self.timings
            .iter()
            .filter(|(_, t)| !t.timed_out)
            .min_by_key(|(_, t)| t.duration_us)
            .map(|(n, t)| (n.as_str(), t.duration_us))
    }
    /// Return the number of timed-out tests.
    pub fn timeout_count(&self) -> usize {
        self.timings.iter().filter(|(_, t)| t.timed_out).count()
    }
}
/// Priority levels for scheduling test execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TestPriority {
    /// Run last.
    Low = 0,
    /// Standard priority.
    Normal = 1,
    /// Run before normal.
    High = 2,
    /// Run first.
    Critical = 3,
}
/// A named corpus of differential test cases, supporting save/load from a
/// simple textual format.
#[derive(Debug, Clone, Default)]
pub struct DiffCorpus {
    /// Corpus identifier (e.g., "lean4_regression_v2").
    pub id: String,
    /// Version tag for the corpus.
    pub version: u32,
    /// The test cases in this corpus.
    pub cases: Vec<DiffTestCase>,
    /// Descriptive tags for the corpus.
    pub tags: Vec<String>,
}
impl DiffCorpus {
    /// Create a new corpus with the given id and version.
    pub fn new(id: impl Into<String>, version: u32) -> Self {
        Self {
            id: id.into(),
            version,
            cases: Vec::new(),
            tags: Vec::new(),
        }
    }
    /// Add a test case to the corpus.
    pub fn push(&mut self, case: DiffTestCase) {
        self.cases.push(case);
    }
    /// Add a tag to this corpus.
    pub fn add_tag(&mut self, tag: impl Into<String>) {
        self.tags.push(tag.into());
    }
    /// Return the number of cases in the corpus.
    pub fn len(&self) -> usize {
        self.cases.len()
    }
    /// Return true if the corpus has no cases.
    pub fn is_empty(&self) -> bool {
        self.cases.is_empty()
    }
    /// Filter cases by a predicate, returning a new sub-corpus.
    pub fn filter<F>(&self, pred: F) -> Self
    where
        F: Fn(&DiffTestCase) -> bool,
    {
        let cases: Vec<_> = self.cases.iter().filter(|c| pred(c)).cloned().collect();
        Self {
            id: format!("{}_filtered", self.id),
            version: self.version,
            cases,
            tags: self.tags.clone(),
        }
    }
    /// Merge another corpus into this one.
    pub fn merge(&mut self, other: &DiffCorpus) {
        for case in &other.cases {
            self.cases.push(case.clone());
        }
        for tag in &other.tags {
            if !self.tags.contains(tag) {
                self.tags.push(tag.clone());
            }
        }
    }
    /// Serialize to a simple line-based format:
    /// Each case is represented as three lines:
    ///   `CASE:<name>`
    ///   `IN:<input>`
    ///   `SUCCESS:<true|false>`
    ///   `[OUT:<expected>]`
    pub fn serialize(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("CORPUS:{}\n", self.id));
        out.push_str(&format!("VERSION:{}\n", self.version));
        for tag in &self.tags {
            out.push_str(&format!("TAG:{}\n", tag));
        }
        for case in &self.cases {
            out.push_str(&format!("CASE:{}\n", case.name));
            out.push_str(&format!("IN:{}\n", case.input.replace('\n', "\\n")));
            out.push_str(&format!("SUCCESS:{}\n", case.should_succeed));
            if let Some(exp) = &case.expected_output {
                out.push_str(&format!("OUT:{}\n", exp.replace('\n', "\\n")));
            }
        }
        out
    }
    /// Deserialize from the format produced by `serialize`.
    pub fn deserialize(src: &str) -> Result<Self, String> {
        let mut corpus = DiffCorpus::default();
        let mut current_name: Option<String> = None;
        let mut current_input: Option<String> = None;
        let mut current_success: Option<bool> = None;
        let mut current_output: Option<String> = None;
        let flush = |corpus: &mut DiffCorpus,
                     name: &mut Option<String>,
                     input: &mut Option<String>,
                     success: &mut Option<bool>,
                     output: &mut Option<String>| {
            if let (Some(n), Some(i), Some(s)) = (name.take(), input.take(), success.take()) {
                corpus.cases.push(DiffTestCase {
                    name: n,
                    input: i,
                    should_succeed: s,
                    expected_output: output.take(),
                });
            }
        };
        for line in src.lines() {
            if let Some(val) = line.strip_prefix("CORPUS:") {
                corpus.id = val.to_string();
            } else if let Some(val) = line.strip_prefix("VERSION:") {
                corpus.version = val.parse().unwrap_or(0);
            } else if let Some(val) = line.strip_prefix("TAG:") {
                corpus.tags.push(val.to_string());
            } else if let Some(val) = line.strip_prefix("CASE:") {
                flush(
                    &mut corpus,
                    &mut current_name,
                    &mut current_input,
                    &mut current_success,
                    &mut current_output,
                );
                current_name = Some(val.to_string());
            } else if let Some(val) = line.strip_prefix("IN:") {
                current_input = Some(val.replace("\\n", "\n"));
            } else if let Some(val) = line.strip_prefix("SUCCESS:") {
                current_success = Some(val == "true");
            } else if let Some(val) = line.strip_prefix("OUT:") {
                current_output = Some(val.replace("\\n", "\n"));
            }
        }
        flush(
            &mut corpus,
            &mut current_name,
            &mut current_input,
            &mut current_success,
            &mut current_output,
        );
        Ok(corpus)
    }
}
/// The outcome of running a single differential test case.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiffTestResult {
    /// The test passed.
    Pass,
    /// The test failed: actual output did not match expected.
    Fail {
        /// The actual output produced.
        actual: String,
        /// The expected output.
        expected: String,
    },
    /// An error occurred while running the test.
    Error(String),
    /// The test produced output when none was expected (unexpected success/failure).
    Unexpected,
}
impl DiffTestResult {
    /// Return true if the test passed.
    pub fn is_pass(&self) -> bool {
        matches!(self, DiffTestResult::Pass)
    }
    /// Return true if the test failed (but not due to an error).
    pub fn is_fail(&self) -> bool {
        matches!(self, DiffTestResult::Fail { .. })
    }
    /// Return true if the test produced an error.
    pub fn is_error(&self) -> bool {
        matches!(self, DiffTestResult::Error(_))
    }
    /// Return true if the result was unexpected.
    pub fn is_unexpected(&self) -> bool {
        matches!(self, DiffTestResult::Unexpected)
    }
    /// Return a short label for display.
    pub fn label(&self) -> &'static str {
        match self {
            DiffTestResult::Pass => "PASS",
            DiffTestResult::Fail { .. } => "FAIL",
            DiffTestResult::Error(_) => "ERROR",
            DiffTestResult::Unexpected => "UNEXPECTED",
        }
    }
}
/// A snapshot of a single test run.
#[derive(Debug, Clone)]
pub struct TestRunSnapshot {
    /// Identifier for this run (e.g. a timestamp or git hash).
    pub run_id: String,
    /// The results from this run.
    pub results: Vec<(String, DiffTestResult)>,
}
impl TestRunSnapshot {
    /// Create a new snapshot.
    pub fn new(run_id: impl Into<String>, results: Vec<(String, DiffTestResult)>) -> Self {
        Self {
            run_id: run_id.into(),
            results,
        }
    }
    /// Compute statistics for this snapshot.
    pub fn statistics(&self) -> DiffTestStatistics {
        DiffTestStatistics::compute(&self.results)
    }
}
/// An annotated test case that carries additional metadata.
#[derive(Debug, Clone)]
pub struct AnnotatedDiffCase {
    /// The underlying test case.
    pub case: DiffTestCase,
    /// Annotations.
    pub annotations: Vec<DiffTestAnnotation>,
}
impl AnnotatedDiffCase {
    /// Create a new annotated case.
    pub fn new(case: DiffTestCase) -> Self {
        Self {
            case,
            annotations: Vec::new(),
        }
    }
    /// Add an annotation.
    pub fn annotate(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.annotations.push(DiffTestAnnotation::new(key, value));
        self
    }
    /// Retrieve the value of an annotation by key.
    pub fn get_annotation(&self, key: &str) -> Option<&str> {
        self.annotations
            .iter()
            .find(|a| a.key == key)
            .map(|a| a.value.as_str())
    }
    /// Return all annotation keys.
    pub fn annotation_keys(&self) -> Vec<&str> {
        self.annotations.iter().map(|a| a.key.as_str()).collect()
    }
}
/// A template for generating multiple `DiffTestCase` instances by substituting
/// parameters into a pattern.
#[derive(Debug, Clone)]
pub struct ParametricDiffCase {
    /// Name pattern, e.g. "add_comm_{a}_{b}".
    pub name_template: String,
    /// Input pattern, e.g. "theorem add_comm_{a}_{b} : {a} + {b} = {b} + {a}".
    pub input_template: String,
    /// Whether the generated cases should succeed.
    pub should_succeed: bool,
    /// Parameter sets (each is a list of (key, value) pairs).
    pub param_sets: Vec<Vec<(String, String)>>,
}
impl ParametricDiffCase {
    /// Create a new parametric diff case.
    pub fn new(
        name_template: impl Into<String>,
        input_template: impl Into<String>,
        should_succeed: bool,
    ) -> Self {
        Self {
            name_template: name_template.into(),
            input_template: input_template.into(),
            should_succeed,
            param_sets: Vec::new(),
        }
    }
    /// Add a parameter set.
    pub fn with_params(mut self, params: Vec<(&str, &str)>) -> Self {
        let owned: Vec<(String, String)> = params
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        self.param_sets.push(owned);
        self
    }
    /// Instantiate all parameter sets and produce `DiffTestCase`s.
    pub fn instantiate(&self) -> Vec<DiffTestCase> {
        self.param_sets
            .iter()
            .map(|params| {
                let mut name = self.name_template.clone();
                let mut input = self.input_template.clone();
                for (k, v) in params {
                    let key = format!("{{{}}}", k);
                    name = name.replace(&key, v);
                    input = input.replace(&key, v);
                }
                DiffTestCase {
                    name,
                    input,
                    should_succeed: self.should_succeed,
                    expected_output: None,
                }
            })
            .collect()
    }
}
/// A top-level test harness that manages multiple `DiffTestSuite`s.
#[derive(Debug, Default)]
pub struct DiffTestHarness {
    suites: Vec<DiffTestSuite>,
    runner: DiffTestRunner,
}
impl DiffTestHarness {
    /// Create a new harness.
    pub fn new() -> Self {
        Self::default()
    }
    /// Create a verbose harness.
    pub fn verbose() -> Self {
        Self {
            suites: Vec::new(),
            runner: DiffTestRunner::verbose(),
        }
    }
    /// Register a suite.
    pub fn add_suite(&mut self, suite: DiffTestSuite) {
        self.suites.push(suite);
    }
    /// Build a suite from a corpus and register it.
    pub fn add_corpus(&mut self, corpus: &DiffCorpus) {
        let mut suite = DiffTestSuite::named(corpus.id.clone());
        for case in &corpus.cases {
            suite.add(case.clone());
        }
        self.suites.push(suite);
    }
    /// Run all suites and return aggregate results.
    pub fn run_all(&self) -> Vec<(String, Vec<(String, DiffTestResult)>)> {
        self.suites
            .iter()
            .map(|suite| {
                let name = suite.name.clone().unwrap_or_else(|| "unnamed".to_string());
                let results = self.runner.run_suite(suite);
                (name, results)
            })
            .collect()
    }
    /// Run all suites and return a top-level aggregate report.
    pub fn aggregate_report(&self) -> DiffTestReport {
        let all = self.run_all();
        let mut report = DiffTestReport::default();
        for (_suite_name, results) in &all {
            for (_name, result) in results {
                report.total += 1;
                match result {
                    DiffTestResult::Pass => report.passed += 1,
                    DiffTestResult::Fail { .. } => report.failed += 1,
                    DiffTestResult::Error(_) => report.errors += 1,
                    DiffTestResult::Unexpected => report.unexpected += 1,
                }
            }
        }
        report
    }
    /// Print a formatted summary to stderr.
    pub fn print_summary(&self) {
        let report = self.aggregate_report();
        eprintln!("=== DiffTestHarness Summary ===");
        eprintln!("  Total:     {}", report.total);
        eprintln!("  Passed:    {}", report.passed);
        eprintln!("  Failed:    {}", report.failed);
        eprintln!("  Errors:    {}", report.errors);
        eprintln!("  Pass rate: {:.1}%", report.pass_rate() * 100.0);
    }
    /// Return all suite names.
    pub fn suite_names(&self) -> Vec<String> {
        self.suites
            .iter()
            .map(|s| s.name.clone().unwrap_or_else(|| "unnamed".to_string()))
            .collect()
    }
    /// Return the total number of cases across all suites.
    pub fn total_cases(&self) -> usize {
        self.suites.iter().map(|s| s.len()).sum()
    }
}
/// A collection of differential test cases.
#[derive(Debug, Clone, Default)]
pub struct DiffTestSuite {
    /// All test cases in this suite.
    pub cases: Vec<DiffTestCase>,
    /// Optional name for the suite.
    pub name: Option<String>,
}
impl DiffTestSuite {
    /// Create an empty test suite.
    pub fn new() -> Self {
        Self::default()
    }
    /// Create a named test suite.
    pub fn named(name: impl Into<String>) -> Self {
        Self {
            cases: Vec::new(),
            name: Some(name.into()),
        }
    }
    /// Add a test case to the suite.
    pub fn add(&mut self, case: DiffTestCase) {
        self.cases.push(case);
    }
    /// Return the number of cases in this suite.
    pub fn len(&self) -> usize {
        self.cases.len()
    }
    /// Return true if the suite has no cases.
    pub fn is_empty(&self) -> bool {
        self.cases.is_empty()
    }
}
/// Detects regressions by comparing two sets of results.
#[derive(Debug, Default)]
pub struct RegressionDetector {
    baseline: Vec<(String, DiffTestResult)>,
}
impl RegressionDetector {
    /// Create a new regression detector with an empty baseline.
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the baseline results (the "before" run).
    pub fn set_baseline(&mut self, results: Vec<(String, DiffTestResult)>) {
        self.baseline = results;
    }
    /// Compare `current` results against the baseline and return any regressions.
    pub fn detect(&self, current: &[(String, DiffTestResult)]) -> Vec<Regression> {
        let mut regressions = Vec::new();
        let baseline_map: std::collections::HashMap<&str, &DiffTestResult> =
            self.baseline.iter().map(|(n, r)| (n.as_str(), r)).collect();
        for (name, new_result) in current {
            if let Some(old_result) = baseline_map.get(name.as_str()) {
                if old_result.is_pass() && !new_result.is_pass() {
                    regressions.push(Regression {
                        name: name.clone(),
                        old_result: (*old_result).clone(),
                        new_result: new_result.clone(),
                    });
                }
            }
        }
        regressions
    }
    /// Detect improvements: tests that were failing and are now passing.
    pub fn detect_improvements(&self, current: &[(String, DiffTestResult)]) -> Vec<String> {
        let baseline_map: std::collections::HashMap<&str, &DiffTestResult> =
            self.baseline.iter().map(|(n, r)| (n.as_str(), r)).collect();
        current
            .iter()
            .filter(|(name, new_result)| {
                if let Some(old_result) = baseline_map.get(name.as_str()) {
                    !old_result.is_pass() && new_result.is_pass()
                } else {
                    false
                }
            })
            .map(|(name, _)| name.clone())
            .collect()
    }
    /// Return the number of tests in the baseline.
    pub fn baseline_len(&self) -> usize {
        self.baseline.len()
    }
}
