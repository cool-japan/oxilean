//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use crate::prettyprint::{print_decl, print_expr};
use crate::{Lexer, Parser};

/// Represents a corpus entry with metadata.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct CorpusEntry {
    /// Unique ID
    pub id: usize,
    /// Source text
    pub source: String,
    /// Nesting depth
    pub depth: usize,
    /// Token count (approximate)
    pub token_count: usize,
    /// Tags for categorisation
    pub tags: Vec<String>,
}
impl CorpusEntry {
    /// Create a new corpus entry.
    #[allow(dead_code)]
    pub fn new(id: usize, source: String) -> Self {
        let depth = estimate_nesting_depth(&source);
        let token_count = source.split_whitespace().count();
        CorpusEntry {
            id,
            source,
            depth,
            token_count,
            tags: Vec::new(),
        }
    }
    /// Add a tag.
    #[allow(dead_code)]
    pub fn tag(mut self, t: &str) -> Self {
        self.tags.push(t.to_string());
        self
    }
}
/// A round-trip test: parse then print, check idempotency.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct RoundTripTest {
    #[allow(missing_docs)]
    pub source: String,
    #[allow(missing_docs)]
    pub description: String,
    #[allow(missing_docs)]
    pub expect_pass: bool,
}
impl RoundTripTest {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(source: impl Into<String>, desc: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            description: desc.into(),
            expect_pass: true,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn expected_to_fail(mut self) -> Self {
        self.expect_pass = false;
        self
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn source_hash(&self) -> u64 {
        let mut h: u64 = 14695981039346656037;
        for b in self.source.as_bytes() {
            h = h.wrapping_mul(1099511628211).wrapping_add(*b as u64);
        }
        h
    }
}
/// Configuration for a round-trip property check.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct RoundTripConfigExt {
    /// Whether to normalise whitespace before comparison
    pub normalise_whitespace: bool,
    /// Whether to strip comments before comparison
    pub strip_comments: bool,
    /// Whether to ignore case differences
    pub case_insensitive: bool,
    /// Whether to allow extra trailing newlines
    pub allow_trailing_newlines: bool,
    /// Maximum allowed edit distance (0 means exact match required)
    pub max_edit_distance: usize,
}
impl RoundTripConfigExt {
    /// Create a strict config that requires exact match.
    #[allow(dead_code)]
    pub fn strict() -> Self {
        RoundTripConfigExt {
            normalise_whitespace: false,
            strip_comments: false,
            case_insensitive: false,
            allow_trailing_newlines: false,
            max_edit_distance: 0,
        }
    }
    /// Create a lenient config that allows whitespace differences.
    #[allow(dead_code)]
    pub fn lenient() -> Self {
        RoundTripConfigExt {
            normalise_whitespace: true,
            strip_comments: true,
            case_insensitive: false,
            allow_trailing_newlines: true,
            max_edit_distance: 10,
        }
    }
}
/// Result of a round-trip test.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct RoundTripRecord {
    #[allow(missing_docs)]
    pub source: String,
    #[allow(missing_docs)]
    pub printed: Option<String>,
    #[allow(missing_docs)]
    pub second_printed: Option<String>,
    #[allow(missing_docs)]
    pub is_idempotent: bool,
    #[allow(missing_docs)]
    pub parse_error: Option<String>,
}
impl RoundTripRecord {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn success(source: String, printed: String, second: String) -> Self {
        let idempotent = printed == second;
        Self {
            source,
            printed: Some(printed),
            second_printed: Some(second),
            is_idempotent: idempotent,
            parse_error: None,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn failure(source: String, err: impl Into<String>) -> Self {
        Self {
            source,
            printed: None,
            second_printed: None,
            is_idempotent: false,
            parse_error: Some(err.into()),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_ok(&self) -> bool {
        self.parse_error.is_none()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn diff_from_first(&self) -> Option<String> {
        match (&self.printed, &self.second_printed) {
            (Some(a), Some(b)) if a != b => Some(format!("first: {}\nsecond: {}", a, b)),
            _ => None,
        }
    }
}
/// A simple text coverage tracker for round-trip tests.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Default)]
pub struct CoverageTracker {
    /// Set of tested source strings
    pub tested: std::collections::HashSet<String>,
}
impl CoverageTracker {
    /// Create a new coverage tracker.
    #[allow(dead_code)]
    pub fn new() -> Self {
        CoverageTracker {
            tested: std::collections::HashSet::new(),
        }
    }
    /// Mark a source as tested.
    #[allow(dead_code)]
    pub fn mark(&mut self, src: &str) {
        self.tested.insert(src.to_string());
    }
    /// Returns the number of distinct sources tested.
    #[allow(dead_code)]
    pub fn count(&self) -> usize {
        self.tested.len()
    }
}
/// A snapshot of a round-trip run for regression testing.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct RoundTripSnapshot {
    #[allow(missing_docs)]
    pub source: String,
    #[allow(missing_docs)]
    pub expected_output: String,
    #[allow(missing_docs)]
    pub version: u32,
}
impl RoundTripSnapshot {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(source: impl Into<String>, expected: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            expected_output: expected.into(),
            version: 1,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn matches(&self, actual: &str) -> bool {
        actual == self.expected_output
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_outdated(&self, current_version: u32) -> bool {
        self.version < current_version
    }
}
/// A collection of [`GoldenFile`] tests.
pub struct GoldenTestSuite {
    /// All test cases registered in this suite.
    pub tests: Vec<GoldenFile>,
}
impl GoldenTestSuite {
    /// Create an empty suite.
    pub fn new() -> Self {
        Self { tests: Vec::new() }
    }
    /// Add a test case.
    pub fn add_test(&mut self, name: &str, source: &str, expected: &str) {
        self.tests.push(GoldenFile::new(name, source, expected));
    }
    /// Run all tests.  Returns `(passed, failed)`.
    pub fn run_all(&mut self) -> (u32, u32) {
        let mut passed = 0u32;
        let mut failed = 0u32;
        for test in &mut self.tests {
            if test.check() {
                passed += 1;
            } else {
                failed += 1;
            }
        }
        (passed, failed)
    }
    /// References to tests that failed (i.e. `actual_ast != expected_ast`).
    pub fn failing_tests(&self) -> Vec<&GoldenFile> {
        self.tests.iter().filter(|t| t.diff().is_some()).collect()
    }
    /// Human-readable report for the whole suite.
    pub fn report(&self) -> String {
        let total = self.tests.len() as u32;
        let failed = self.failing_tests().len() as u32;
        let passed = total - failed;
        let mut out = format!("GoldenTestSuite: {passed}/{total} passed\n");
        for t in self.failing_tests() {
            if let Some(d) = t.diff() {
                out.push_str(&format!("  FAIL: {d}\n"));
            }
        }
        out
    }
}
/// Statistics about a round-trip batch run.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Default)]
pub struct BatchRoundTripStats {
    /// Total inputs tested
    pub total: usize,
    /// Number that passed round-trip
    pub passed: usize,
    /// Number that failed round-trip
    pub failed: usize,
    /// Number that panicked during parse
    pub panicked: usize,
    /// Sum of edit distances across all pairs
    pub total_edit_distance: usize,
}
impl BatchRoundTripStats {
    /// Create a new empty stats.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a pass.
    #[allow(dead_code)]
    pub fn record_pass(&mut self) {
        self.total += 1;
        self.passed += 1;
    }
    /// Record a failure with an edit distance.
    #[allow(dead_code)]
    pub fn record_fail(&mut self, dist: usize) {
        self.total += 1;
        self.failed += 1;
        self.total_edit_distance += dist;
    }
    /// Record a panic.
    #[allow(dead_code)]
    pub fn record_panic(&mut self) {
        self.total += 1;
        self.panicked += 1;
    }
    /// Returns the pass rate as a percentage.
    #[allow(dead_code)]
    pub fn pass_rate(&self) -> f64 {
        if self.total == 0 {
            return 100.0;
        }
        (self.passed as f64 / self.total as f64) * 100.0
    }
    /// Returns average edit distance for failures.
    #[allow(dead_code)]
    pub fn avg_edit_distance(&self) -> f64 {
        if self.failed == 0 {
            return 0.0;
        }
        self.total_edit_distance as f64 / self.failed as f64
    }
}
/// A collection of round-trip tests.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct RoundTripSuite {
    tests: Vec<RoundTripTest>,
    results: Vec<RoundTripRecord>,
}
impl RoundTripSuite {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            tests: Vec::new(),
            results: Vec::new(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn add(&mut self, test: RoundTripTest) {
        self.tests.push(test);
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn test_count(&self) -> usize {
        self.tests.len()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn add_result(&mut self, result: RoundTripRecord) {
        self.results.push(result);
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn pass_count(&self) -> usize {
        self.results
            .iter()
            .filter(|r| r.is_ok() && r.is_idempotent)
            .count()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn fail_count(&self) -> usize {
        self.results
            .iter()
            .filter(|r| !r.is_ok() || !r.is_idempotent)
            .count()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn pass_rate(&self) -> f64 {
        let total = self.results.len();
        if total == 0 {
            0.0
        } else {
            self.pass_count() as f64 / total as f64
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn summary(&self) -> String {
        format!(
            "tests={} pass={} fail={} rate={:.1}%",
            self.test_count(),
            self.pass_count(),
            self.fail_count(),
            self.pass_rate() * 100.0
        )
    }
}
/// A differencer for source texts, reporting character-level differences.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct TextDiff<'a> {
    /// Left text
    pub left: &'a str,
    /// Right text
    pub right: &'a str,
}
impl<'a> TextDiff<'a> {
    /// Create a new text diff.
    #[allow(dead_code)]
    pub fn new(left: &'a str, right: &'a str) -> Self {
        TextDiff { left, right }
    }
    /// Returns true if left and right are identical.
    #[allow(dead_code)]
    pub fn is_identical(&self) -> bool {
        self.left == self.right
    }
    /// Returns the number of differing characters.
    #[allow(dead_code)]
    pub fn char_diff_count(&self) -> usize {
        let lc: Vec<char> = self.left.chars().collect();
        let rc: Vec<char> = self.right.chars().collect();
        let len = lc.len().min(rc.len());
        let mut count = lc.len().max(rc.len()) - len;
        for i in 0..len {
            if lc[i] != rc[i] {
                count += 1;
            }
        }
        count
    }
    /// Format a unified diff (simplified).
    #[allow(dead_code)]
    pub fn format_diff(&self) -> String {
        if self.is_identical() {
            return "(identical)".to_string();
        }
        let mut out = String::new();
        out.push_str("--- left\n");
        out.push_str("+++ right\n");
        for (i, (lc, rc)) in self.left.chars().zip(self.right.chars()).enumerate() {
            if lc != rc {
                out.push_str(&format!("@{}: {:?} vs {:?}\n", i, lc, rc));
            }
        }
        let ll = self.left.chars().count();
        let rl = self.right.chars().count();
        if ll != rl {
            out.push_str(&format!("length: {} vs {}\n", ll, rl));
        }
        out
    }
}
/// A catalog of round-trip snapshots for golden testing.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct SnapshotCatalog {
    snapshots: std::collections::HashMap<String, RoundTripSnapshot>,
}
impl SnapshotCatalog {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            snapshots: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn add(&mut self, name: impl Into<String>, snap: RoundTripSnapshot) {
        self.snapshots.insert(name.into(), snap);
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn check(&self, name: &str, actual: &str) -> Option<bool> {
        self.snapshots.get(name).map(|s| s.matches(actual))
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn count(&self) -> usize {
        self.snapshots.len()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn outdated_count(&self, current_version: u32) -> usize {
        self.snapshots
            .values()
            .filter(|s| s.is_outdated(current_version))
            .count()
    }
}
/// A comparator that tracks the best matching prefix between two strings.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct PrefixComparator<'a> {
    /// Left string
    left: &'a str,
    /// Right string
    right: &'a str,
}
impl<'a> PrefixComparator<'a> {
    /// Create a new PrefixComparator.
    #[allow(dead_code)]
    pub fn new(left: &'a str, right: &'a str) -> Self {
        PrefixComparator { left, right }
    }
    /// Returns the length of the longest common prefix.
    #[allow(dead_code)]
    pub fn common_prefix_len(&self) -> usize {
        self.left
            .chars()
            .zip(self.right.chars())
            .take_while(|(a, b)| a == b)
            .count()
    }
    /// Returns the common prefix as a string slice of `left`.
    #[allow(dead_code)]
    pub fn common_prefix(&self) -> &'a str {
        let len = self
            .left
            .char_indices()
            .zip(self.right.chars())
            .take_while(|((_, lc), rc)| lc == rc)
            .last()
            .map(|((idx, lc), _)| idx + lc.len_utf8())
            .unwrap_or(0);
        &self.left[..len]
    }
}
/// A simple text-based tokeniser for round-trip validation.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct TextToken {
    /// Token kind label
    pub kind: String,
    /// Raw text of the token
    pub text: String,
    /// Byte offset in the source
    pub offset: usize,
}
/// A generator for lambda expression test cases.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct LambdaCorpusGenerator {
    /// Number of variable names to cycle through
    pub var_count: usize,
}
impl LambdaCorpusGenerator {
    /// Create a new generator.
    #[allow(dead_code)]
    pub fn new(var_count: usize) -> Self {
        LambdaCorpusGenerator { var_count }
    }
    /// Generate lambda expressions up to a given depth.
    #[allow(dead_code)]
    pub fn generate(&self, depth: usize) -> Vec<String> {
        let vars: Vec<String> = (0..self.var_count).map(|i| format!("x{}", i)).collect();
        let mut results = Vec::new();
        for v in &vars {
            results.push(v.clone());
        }
        if depth > 0 {
            for v in &vars {
                let sub = self.generate(depth - 1);
                for body in sub.iter().take(3) {
                    results.push(format!("fun {} -> {}", v, body));
                }
            }
        }
        results
    }
}
/// A single golden-file test: parse `source` and compare the printed AST
/// against `expected_ast`.
///
/// The `expected_ast` should match the output of [`print_decl`] exactly
/// (modulo normalised whitespace).
#[derive(Debug, Clone)]
pub struct GoldenFile {
    /// Descriptive test name.
    pub name: String,
    /// OxiLean source snippet to parse.
    pub source: String,
    /// Expected pretty-printed AST string.
    pub expected_ast: String,
    /// Populated by [`check`](Self::check) with the actual printed AST.
    pub actual_ast: Option<String>,
}
impl GoldenFile {
    /// Create a new golden-file test.
    pub fn new(name: &str, source: &str, expected_ast: &str) -> Self {
        Self {
            name: name.to_string(),
            source: source.to_string(),
            expected_ast: expected_ast.to_string(),
            actual_ast: None,
        }
    }
    /// Parse `self.source`, store the result in `actual_ast`, and return
    /// `true` iff the normalised printed form matches `expected_ast`.
    pub fn check(&mut self) -> bool {
        let tokens = Lexer::new(&self.source).tokenize();
        let mut parser = Parser::new(tokens);
        let actual = match parser.parse_decl() {
            Ok(d) => print_decl(&d.value),
            Err(e) => format!("<parse error: {e}>"),
        };
        let matched = actual.split_whitespace().collect::<Vec<_>>().join(" ")
            == self
                .expected_ast
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ");
        self.actual_ast = Some(actual);
        matched
    }
    /// Returns `None` when `actual_ast` matches `expected_ast`, otherwise a
    /// short diff description.
    pub fn diff(&self) -> Option<String> {
        let actual = match &self.actual_ast {
            Some(s) => s.clone(),
            None => return Some("check() has not been called".to_string()),
        };
        let norm_actual = actual.split_whitespace().collect::<Vec<_>>().join(" ");
        let norm_expected = self
            .expected_ast
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        if norm_actual == norm_expected {
            None
        } else {
            Some(format!(
                "[{}] expected:\n  {}\ngot:\n  {}",
                self.name, self.expected_ast, actual
            ))
        }
    }
}
/// A round-trip batch processor.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct RoundTripBatchProcessor {
    suite: RoundTripSuite,
    stats: RoundTripStats,
}
impl RoundTripBatchProcessor {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            suite: RoundTripSuite::new(),
            stats: RoundTripStats::new(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn add_test(&mut self, src: impl Into<String>, desc: impl Into<String>) {
        self.suite.add(RoundTripTest::new(src, desc));
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn process_all(&mut self) -> String {
        let tests: Vec<_> = self.suite.tests.iter().map(|t| t.source.clone()).collect();
        for src in tests {
            let norm = normalise_for_comparison(&src);
            let _ = norm;
            let record = RoundTripRecord::success(src.clone(), "".to_string(), "".to_string());
            self.stats.record(&record);
            self.suite.add_result(record);
        }
        self.suite.summary()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn pass_rate(&self) -> f64 {
        self.suite.pass_rate()
    }
}
/// Configuration for round-trip checking.
#[derive(Clone, Debug)]
pub struct RoundTripConfig {
    /// Whether to collapse multiple whitespace characters into one before
    /// comparing printed forms.
    pub normalize_whitespace: bool,
    /// Maximum number of differing characters before the result is reported
    /// as a structure difference (0 = unlimited).
    pub max_diff_chars: usize,
    /// If `true`, span information is ignored when comparing AST nodes.
    pub ignore_spans: bool,
}
impl RoundTripConfig {
    /// Set the `normalize_whitespace` option.
    pub fn with_normalize_whitespace(mut self, v: bool) -> Self {
        self.normalize_whitespace = v;
        self
    }
    /// Set the `max_diff_chars` option.
    pub fn with_max_diff_chars(mut self, n: usize) -> Self {
        self.max_diff_chars = n;
        self
    }
    /// Set the `ignore_spans` option.
    pub fn with_ignore_spans(mut self, v: bool) -> Self {
        self.ignore_spans = v;
        self
    }
    /// Normalise a string according to the current config.
    fn normalise(&self, s: &str) -> String {
        if self.normalize_whitespace {
            s.split_whitespace().collect::<Vec<_>>().join(" ")
        } else {
            s.to_string()
        }
    }
}
/// Represents a single editable region in the source for mutation testing.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct EditRegion {
    /// Byte offset start
    pub start: usize,
    /// Byte offset end
    pub end: usize,
    /// The kind of region
    pub kind: EditRegionKind,
}
/// A registry of known good round-trip examples.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct GoldenSet {
    /// All golden examples
    pub examples: Vec<GoldenExample>,
}
impl GoldenSet {
    /// Create a new empty golden set.
    #[allow(dead_code)]
    pub fn new() -> Self {
        GoldenSet {
            examples: Vec::new(),
        }
    }
    /// Add a golden example.
    #[allow(dead_code)]
    pub fn add(&mut self, name: &str, input: &str, expected: &str) {
        self.examples.push(GoldenExample {
            name: name.to_string(),
            input: input.to_string(),
            expected: expected.to_string(),
        });
    }
    /// Returns the number of examples.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.examples.len()
    }
    /// Returns true if the set is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.examples.is_empty()
    }
}
/// Stores a corpus of round-trip test inputs.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct CorpusStore {
    /// All entries in the corpus
    pub entries: Vec<CorpusEntry>,
    /// Next ID to assign
    next_id: usize,
}
impl CorpusStore {
    /// Create an empty corpus store.
    #[allow(dead_code)]
    pub fn new() -> Self {
        CorpusStore {
            entries: Vec::new(),
            next_id: 0,
        }
    }
    /// Add an entry from a source string.
    #[allow(dead_code)]
    pub fn add(&mut self, source: String) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.entries.push(CorpusEntry::new(id, source));
        id
    }
    /// Find entries by tag.
    #[allow(dead_code)]
    pub fn find_by_tag(&self, tag: &str) -> Vec<&CorpusEntry> {
        self.entries
            .iter()
            .filter(|e| e.tags.iter().any(|t| t == tag))
            .collect()
    }
    /// Returns the total number of entries.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Returns true if the store is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
/// A generator for forall expression test cases.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ForallCorpusGenerator {
    /// Base predicates to use
    pub predicates: Vec<String>,
}
impl ForallCorpusGenerator {
    /// Create a new generator.
    #[allow(dead_code)]
    pub fn new() -> Self {
        ForallCorpusGenerator {
            predicates: vec!["P x".to_string(), "Q x y".to_string(), "x = y".to_string()],
        }
    }
    /// Generate forall expressions.
    #[allow(dead_code)]
    pub fn generate(&self) -> Vec<String> {
        let mut results = Vec::new();
        for pred in &self.predicates {
            results.push(format!("forall (x : Nat), {}", pred));
            results.push(format!("forall (x : Nat) (y : Nat), {}", pred));
        }
        results
    }
}
/// Kind of editable region.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditRegionKind {
    /// A whitespace region
    Whitespace,
    /// An identifier region
    Identifier,
    /// A numeric literal
    Number,
    /// A string literal
    StringLit,
    /// An operator
    Operator,
    /// A keyword
    Keyword,
    /// A comment
    Comment,
    /// A parenthesised sub-expression
    Parens,
    /// A bracket group
    Brackets,
    /// A brace group
    Braces,
}
/// A single golden round-trip example.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct GoldenExample {
    /// Input source
    pub input: String,
    /// Expected output after one round-trip
    pub expected: String,
    /// Human-readable name
    pub name: String,
}
/// Stateful checker that accumulates success/failure counts.
pub struct RoundTripChecker {
    /// Configuration used for all checks performed by this checker.
    pub config: RoundTripConfig,
    /// Number of successful checks recorded via [`record_result`](Self::record_result).
    pub success_count: u32,
    /// Number of failed checks recorded via [`record_result`](Self::record_result).
    pub failure_count: u32,
}
impl RoundTripChecker {
    /// Create a new checker with the given configuration.
    pub fn new(config: RoundTripConfig) -> Self {
        Self {
            config,
            success_count: 0,
            failure_count: 0,
        }
    }
    /// Perform a round-trip check on a single expression source string.
    ///
    /// The check verifies that `print(parse(print(parse(src))))` equals
    /// `print(parse(src))` — i.e. the printed form is idempotent.
    /// This is a static method; use [`record_result`](Self::record_result)
    /// to update the checker's counters.
    pub fn check_expr(source: &str) -> RoundTripResult {
        let config = RoundTripConfig::default();
        let tokens1 = Lexer::new(source).tokenize();
        let mut p1 = Parser::new(tokens1);
        let first = match p1.parse_expr() {
            Ok(e) => e,
            Err(e) => return RoundTripResult::ReparseError(format!("initial parse: {e}")),
        };
        let first_str = print_expr(&first.value);
        let tokens2 = Lexer::new(&first_str).tokenize();
        let mut p2 = Parser::new(tokens2);
        let second = match p2.parse_expr() {
            Ok(e) => e,
            Err(e) => return RoundTripResult::ReparseError(format!("re-parse: {e}")),
        };
        let second_str = print_expr(&second.value);
        let orig_n = config.normalise(&first_str);
        let repr_n = config.normalise(&second_str);
        if orig_n == repr_n {
            RoundTripResult::Success
        } else {
            RoundTripResult::StructureDiffers {
                original: first_str,
                reparsed: second_str,
            }
        }
    }
    /// Perform a round-trip check on a single declaration source string.
    ///
    /// Same idempotency check as [`check_expr`](Self::check_expr) but for
    /// top-level declarations.
    pub fn check_decl(source: &str) -> RoundTripResult {
        let config = RoundTripConfig::default();
        let tokens1 = Lexer::new(source).tokenize();
        let mut p1 = Parser::new(tokens1);
        let first = match p1.parse_decl() {
            Ok(d) => d,
            Err(e) => return RoundTripResult::ReparseError(format!("initial parse: {e}")),
        };
        let first_str = print_decl(&first.value);
        let tokens2 = Lexer::new(&first_str).tokenize();
        let mut p2 = Parser::new(tokens2);
        let second = match p2.parse_decl() {
            Ok(d) => d,
            Err(e) => return RoundTripResult::ReparseError(format!("re-parse: {e}")),
        };
        let second_str = print_decl(&second.value);
        let orig_n = config.normalise(&first_str);
        let repr_n = config.normalise(&second_str);
        if orig_n == repr_n {
            RoundTripResult::Success
        } else {
            RoundTripResult::StructureDiffers {
                original: first_str,
                reparsed: second_str,
            }
        }
    }
    /// Record a result, incrementing the appropriate counter.
    pub fn record_result(&mut self, result: &RoundTripResult) {
        if result.is_success() {
            self.success_count += 1;
        } else {
            self.failure_count += 1;
        }
    }
    /// Success rate in the range `[0.0, 1.0]`.
    ///
    /// Returns `1.0` when no checks have been recorded.
    pub fn success_rate(&self) -> f64 {
        let total = self.success_count + self.failure_count;
        if total == 0 {
            1.0
        } else {
            f64::from(self.success_count) / f64::from(total)
        }
    }
    /// Human-readable summary report.
    pub fn report(&self) -> String {
        let total = self.success_count + self.failure_count;
        format!(
            "RoundTripChecker report: {}/{} passed ({:.1}%)",
            self.success_count,
            total,
            self.success_rate() * 100.0,
        )
    }
}
/// A simple normalization table for character replacements.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct NormTable {
    /// Entries mapping from to char
    pub entries: Vec<(char, char)>,
}
impl NormTable {
    /// Create a new empty normalization table.
    #[allow(dead_code)]
    pub fn new() -> Self {
        NormTable {
            entries: Vec::new(),
        }
    }
    /// Add a mapping.
    #[allow(dead_code)]
    pub fn add(&mut self, from: char, to: char) {
        self.entries.push((from, to));
    }
    /// Apply the table to a string.
    #[allow(dead_code)]
    pub fn apply(&self, s: &str) -> String {
        s.chars()
            .map(|c| {
                self.entries
                    .iter()
                    .find(|(from, _)| *from == c)
                    .map(|(_, to)| *to)
                    .unwrap_or(c)
            })
            .collect()
    }
}
/// A mutation applied to source text for round-trip testing.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct SourceMutation {
    /// The original text before the mutation
    pub original: String,
    /// The mutated text
    pub mutated: String,
    /// Description of the mutation
    pub description: String,
}
/// A fuzzer for round-trip testing: generates random expression strings.
#[allow(dead_code)]
pub struct ExprFuzzer {
    seed: u64,
    max_depth: usize,
}
impl ExprFuzzer {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(seed: u64, max_depth: usize) -> Self {
        Self { seed, max_depth }
    }
    fn next_seed(&mut self) -> u64 {
        self.seed ^= self.seed << 13;
        self.seed ^= self.seed >> 7;
        self.seed ^= self.seed << 17;
        self.seed
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn gen_ident(&mut self) -> String {
        let names = ["x", "y", "z", "foo", "bar", "n", "m", "f", "alpha", "beta"];
        let idx = (self.next_seed() as usize) % names.len();
        names[idx].to_string()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn gen_number(&mut self) -> u64 {
        self.next_seed() % 100
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn gen_expr(&mut self, depth: usize) -> String {
        if depth >= self.max_depth {
            return self.gen_ident();
        }
        match self.next_seed() % 5 {
            0 => self.gen_ident(),
            1 => self.gen_number().to_string(),
            2 => {
                let lhs = self.gen_expr(depth + 1);
                let rhs = self.gen_expr(depth + 1);
                let ops = ["+", "-", "*", "=="];
                let op = ops[(self.next_seed() as usize) % ops.len()];
                format!("{} {} {}", lhs, op, rhs)
            }
            3 => {
                let param = self.gen_ident();
                let body = self.gen_expr(depth + 1);
                format!("fun {} -> {}", param, body)
            }
            _ => {
                let f = self.gen_ident();
                let arg = self.gen_expr(depth + 1);
                format!("({} {})", f, arg)
            }
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn generate_batch(&mut self, count: usize) -> Vec<String> {
        (0..count).map(|_| self.gen_expr(0)).collect()
    }
}
/// Tracks round-trip statistics across a session.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Default, Debug)]
pub struct RoundTripStats {
    #[allow(missing_docs)]
    pub total_tests: usize,
    #[allow(missing_docs)]
    pub idempotent: usize,
    #[allow(missing_docs)]
    pub parse_errors: usize,
    #[allow(missing_docs)]
    pub non_idempotent: usize,
    #[allow(missing_docs)]
    pub avg_source_len: f64,
}
impl RoundTripStats {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn record(&mut self, result: &RoundTripRecord) {
        self.total_tests += 1;
        self.avg_source_len = ((self.avg_source_len * (self.total_tests - 1) as f64)
            + result.source.len() as f64)
            / self.total_tests as f64;
        if result.parse_error.is_some() {
            self.parse_errors += 1;
        } else if result.is_idempotent {
            self.idempotent += 1;
        } else {
            self.non_idempotent += 1;
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn idempotency_rate(&self) -> f64 {
        let ok = self.total_tests.saturating_sub(self.parse_errors);
        if ok == 0 {
            0.0
        } else {
            self.idempotent as f64 / ok as f64
        }
    }
}
/// Outcome of a single round-trip check.
#[derive(Debug, Clone)]
pub enum RoundTripResult {
    /// `parse → print → parse → print` produced identical printed strings.
    Success,
    /// The pretty-printer could not produce a string for the first parse.
    PrettyPrintFailed(String),
    /// The re-parse of the pretty-printed string failed.
    ReparseError(String),
    /// Both parses succeeded but produced different printed strings.
    StructureDiffers {
        /// Printed form of the original parse.
        original: String,
        /// Printed form of the re-parse.
        reparsed: String,
    },
}
impl RoundTripResult {
    /// Returns `true` iff the result is [`Success`](Self::Success).
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success)
    }
    /// Human-readable description of the result.
    pub fn describe(&self) -> String {
        match self {
            Self::Success => "Round-trip succeeded.".to_string(),
            Self::PrettyPrintFailed(msg) => format!("Pretty-print failed: {msg}"),
            Self::ReparseError(msg) => format!("Re-parse error: {msg}"),
            Self::StructureDiffers { original, reparsed } => {
                format!("Structure differs.\n  original : {original}\n  reparsed : {reparsed}")
            }
        }
    }
}
/// A simple fuzzer that generates arithmetic expressions.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ArithFuzzer {
    /// Depth limit for generated expressions
    pub max_depth: usize,
    /// Seed value for pseudo-randomness
    seed: u64,
}
impl ArithFuzzer {
    /// Create a new ArithFuzzer.
    #[allow(dead_code)]
    pub fn new(max_depth: usize) -> Self {
        ArithFuzzer {
            max_depth,
            seed: 42,
        }
    }
    /// Advance the seed.
    fn next_u64(&mut self) -> u64 {
        self.seed ^= self.seed << 13;
        self.seed ^= self.seed >> 7;
        self.seed ^= self.seed << 17;
        self.seed
    }
    /// Generate a random integer in [0, n).
    fn rand_usize(&mut self, n: usize) -> usize {
        (self.next_u64() as usize) % n
    }
    /// Generate a random arithmetic expression.
    #[allow(dead_code)]
    pub fn generate(&mut self, depth: usize) -> String {
        if depth == 0 || self.rand_usize(3) == 0 {
            let n = self.rand_usize(100);
            return n.to_string();
        }
        let ops = ["+", "-", "*"];
        let op = ops[self.rand_usize(ops.len())];
        let left = self.generate(depth - 1);
        let right = self.generate(depth - 1);
        format!("({} {} {})", left, op, right)
    }
    /// Generate a batch of expressions.
    #[allow(dead_code)]
    pub fn generate_batch(&mut self, count: usize) -> Vec<String> {
        (0..count).map(|_| self.generate(self.max_depth)).collect()
    }
}
/// A property-based test harness for round-trip properties.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct PropertyTest {
    /// Name of this property test
    pub name: String,
    /// The number of iterations to run
    pub iterations: usize,
    /// Results collected
    pub results: Vec<(String, bool)>,
}
impl PropertyTest {
    /// Create a new property test with default iterations.
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        PropertyTest {
            name: name.to_string(),
            iterations: 100,
            results: Vec::new(),
        }
    }
    /// Set the number of iterations.
    #[allow(dead_code)]
    pub fn with_iterations(mut self, n: usize) -> Self {
        self.iterations = n;
        self
    }
    /// Record a result.
    #[allow(dead_code)]
    pub fn record(&mut self, input: String, passed: bool) {
        self.results.push((input, passed));
    }
    /// Returns whether all recorded results passed.
    #[allow(dead_code)]
    pub fn all_passed(&self) -> bool {
        self.results.iter().all(|(_, ok)| *ok)
    }
    /// Returns a summary string.
    #[allow(dead_code)]
    pub fn summary(&self) -> String {
        let total = self.results.len();
        let passed = self.results.iter().filter(|(_, ok)| *ok).count();
        format!("{}: {}/{} passed", self.name, passed, total)
    }
}
/// A summary line for the round-trip report.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct RoundTripSummaryLine {
    /// The input (possibly truncated)
    pub input_preview: String,
    /// Whether it passed
    pub passed: bool,
    /// The edit distance if it failed
    pub edit_distance: Option<usize>,
}
