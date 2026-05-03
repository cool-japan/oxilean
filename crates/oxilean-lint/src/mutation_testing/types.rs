//! Mutation testing types for oxilean-lint.
//!
//! This module defines the core data structures used by the mutation testing
//! framework. Mutation testing works by introducing small syntactic changes
//! ("mutations") to source code and verifying that the existing test suite
//! detects each change (i.e., the mutation is "killed"). A mutation that
//! causes no test failure "survives" and signals a gap in test coverage.

use std::collections::HashMap;
use std::fmt;

// ============================================================
// MutationOperator
// ============================================================

/// Every supported mutation operator.
///
/// Each variant describes one kind of syntactic transformation that the
/// framework can apply automatically to a source file.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MutationOperator {
    /// Replace `true` with `false` and vice-versa.
    ReplaceBoolLiteral,
    /// Wrap a condition in a logical-NOT: `cond` → `!cond`.
    NegateCondition,
    /// Swap arithmetic operators: `+` → `-`, `*` → `/`.
    ReplaceArithmetic,
    /// Replace the expression in a `return` with the type's default value.
    RemoveReturn,
    /// Swap comparison operators: `<` → `<=`, `==` → `!=`.
    ReplaceComparison,
    /// Swap logical operators: `&&` → `||` and `||` → `&&`.
    ReplaceLogical,
    /// Increment an integer literal by one: `n` → `n + 1`.
    IncrementLiteral,
    /// Decrement an integer literal by one: `n` → `n - 1`.
    DecrementLiteral,
}

impl fmt::Display for MutationOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            MutationOperator::ReplaceBoolLiteral => "ReplaceBoolLiteral",
            MutationOperator::NegateCondition => "NegateCondition",
            MutationOperator::ReplaceArithmetic => "ReplaceArithmetic",
            MutationOperator::RemoveReturn => "RemoveReturn",
            MutationOperator::ReplaceComparison => "ReplaceComparison",
            MutationOperator::ReplaceLogical => "ReplaceLogical",
            MutationOperator::IncrementLiteral => "IncrementLiteral",
            MutationOperator::DecrementLiteral => "DecrementLiteral",
        };
        write!(f, "{}", name)
    }
}

// ============================================================
// Mutation
// ============================================================

/// A single mutation that can be applied to a source file.
///
/// Mutations are purely positional: they record the byte range of the original
/// text and the replacement text so that `apply_mutation` can perform a simple
/// string substitution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mutation {
    /// The operator that produced this mutation.
    pub operator: MutationOperator,
    /// Path to the file in which the mutation lives.
    pub file: String,
    /// 1-based line number of the mutated token.
    pub line: u32,
    /// 1-based column number of the mutated token.
    pub col: u32,
    /// The original source text that will be replaced.
    pub original: String,
    /// The replacement text that the mutation introduces.
    pub mutated: String,
}

impl Mutation {
    /// Construct a new [`Mutation`].
    pub fn new(
        operator: MutationOperator,
        file: impl Into<String>,
        line: u32,
        col: u32,
        original: impl Into<String>,
        mutated: impl Into<String>,
    ) -> Self {
        Self {
            operator,
            file: file.into(),
            line,
            col,
            original: original.into(),
            mutated: mutated.into(),
        }
    }
}

impl fmt::Display for Mutation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {}:{}:{} `{}` → `{}`",
            self.operator, self.file, self.line, self.col, self.original, self.mutated
        )
    }
}

// ============================================================
// MutationResult
// ============================================================

/// The outcome of running the test suite against a mutated program.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MutationResult {
    /// At least one test failed — the mutation was detected.
    Killed,
    /// All tests passed — the mutation went undetected.
    Survived,
    /// The test suite did not finish within the configured timeout.
    Timeout,
    /// The mutated source did not compile.
    CompileError,
}

impl fmt::Display for MutationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            MutationResult::Killed => "Killed",
            MutationResult::Survived => "Survived",
            MutationResult::Timeout => "Timeout",
            MutationResult::CompileError => "CompileError",
        };
        write!(f, "{}", label)
    }
}

// ============================================================
// MutationReport
// ============================================================

/// Aggregated results from a mutation-testing run.
#[derive(Debug, Clone)]
pub struct MutationReport {
    /// Every mutation together with its test-suite outcome.
    pub mutations: Vec<(Mutation, MutationResult)>,
    /// Fraction of mutations that were killed (killed / total).
    pub kill_rate: f64,
    /// Total number of mutations attempted.
    pub total: usize,
    /// Number of mutations killed by the test suite.
    pub killed: usize,
    /// Number of mutations that survived (tests did not detect them).
    pub survived: usize,
}

impl MutationReport {
    /// Build a [`MutationReport`] by inspecting a list of `(Mutation, MutationResult)` pairs.
    pub fn from_results(mutations: Vec<(Mutation, MutationResult)>) -> Self {
        let total = mutations.len();
        let killed = mutations
            .iter()
            .filter(|(_, r)| *r == MutationResult::Killed)
            .count();
        let survived = mutations
            .iter()
            .filter(|(_, r)| *r == MutationResult::Survived)
            .count();
        let kill_rate = if total == 0 {
            0.0
        } else {
            killed as f64 / total as f64
        };
        Self {
            mutations,
            kill_rate,
            total,
            killed,
            survived,
        }
    }

    /// Returns `true` if no mutations were attempted.
    pub fn is_empty(&self) -> bool {
        self.total == 0
    }
}

impl Default for MutationReport {
    fn default() -> Self {
        Self::from_results(Vec::new())
    }
}

impl fmt::Display for MutationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MutationReport {{ total: {}, killed: {}, survived: {}, kill_rate: {:.2}% }}",
            self.total,
            self.killed,
            self.survived,
            self.kill_rate * 100.0
        )
    }
}

// ============================================================
// MutationConfig
// ============================================================

/// Configuration for a mutation-testing run.
#[derive(Debug, Clone)]
pub struct MutationConfig {
    /// Which mutation operators to apply.
    pub operators: Vec<MutationOperator>,
    /// Maximum number of mutations to generate (0 = unlimited).
    pub max_mutations: usize,
    /// Milliseconds before a single mutant's test run is considered timed-out.
    pub timeout_ms: u64,
    /// File paths (or glob patterns) to target. Empty = all files.
    pub target_files: Vec<String>,
}

impl MutationConfig {
    /// Create a configuration with sensible defaults: all operators enabled,
    /// at most 1 000 mutations, 30-second timeout, no file filter.
    pub fn default_config() -> Self {
        Self {
            operators: vec![
                MutationOperator::ReplaceBoolLiteral,
                MutationOperator::NegateCondition,
                MutationOperator::ReplaceArithmetic,
                MutationOperator::RemoveReturn,
                MutationOperator::ReplaceComparison,
                MutationOperator::ReplaceLogical,
                MutationOperator::IncrementLiteral,
                MutationOperator::DecrementLiteral,
            ],
            max_mutations: 1_000,
            timeout_ms: 30_000,
            target_files: Vec::new(),
        }
    }

    /// Return a builder for constructing a [`MutationConfig`] step by step.
    pub fn builder() -> MutationConfigBuilder {
        MutationConfigBuilder::new()
    }
}

impl Default for MutationConfig {
    fn default() -> Self {
        Self::default_config()
    }
}

// ============================================================
// MutationConfigBuilder
// ============================================================

/// Fluent builder for [`MutationConfig`].
#[derive(Debug, Default)]
pub struct MutationConfigBuilder {
    operators: Vec<MutationOperator>,
    max_mutations: Option<usize>,
    timeout_ms: Option<u64>,
    target_files: Vec<String>,
}

impl MutationConfigBuilder {
    /// Create a new builder with no operators selected.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a mutation operator.
    pub fn with_operator(mut self, op: MutationOperator) -> Self {
        self.operators.push(op);
        self
    }

    /// Add multiple mutation operators at once.
    pub fn with_operators(mut self, ops: impl IntoIterator<Item = MutationOperator>) -> Self {
        self.operators.extend(ops);
        self
    }

    /// Set the maximum number of mutations to generate.
    pub fn max_mutations(mut self, n: usize) -> Self {
        self.max_mutations = Some(n);
        self
    }

    /// Set the per-mutant timeout in milliseconds.
    pub fn timeout_ms(mut self, ms: u64) -> Self {
        self.timeout_ms = Some(ms);
        self
    }

    /// Add a target file path or glob pattern.
    pub fn target_file(mut self, path: impl Into<String>) -> Self {
        self.target_files.push(path.into());
        self
    }

    /// Consume the builder and produce a [`MutationConfig`].
    pub fn build(self) -> MutationConfig {
        let base = MutationConfig::default_config();
        MutationConfig {
            operators: if self.operators.is_empty() {
                base.operators
            } else {
                self.operators
            },
            max_mutations: self.max_mutations.unwrap_or(base.max_mutations),
            timeout_ms: self.timeout_ms.unwrap_or(base.timeout_ms),
            target_files: self.target_files,
        }
    }
}

// ============================================================
// MutationStats
// ============================================================

/// Statistical summary of a mutation-testing run.
///
/// Unlike [`MutationReport`], which owns the full mutation list, `MutationStats`
/// is a lightweight value-type intended for quick inspections and reporting.
#[derive(Debug, Clone, PartialEq)]
pub struct MutationStats {
    /// Total number of mutations attempted.
    pub total: usize,
    /// Number of mutations killed by the test suite.
    pub killed: usize,
    /// Number of mutations that were not detected.
    pub survived: usize,
    /// Number of mutations that timed-out.
    pub timed_out: usize,
    /// Number of mutations that failed to compile.
    pub compile_errors: usize,
    /// Per-operator breakdown: operator name → (killed, total).
    pub by_operator: HashMap<String, (usize, usize)>,
}

impl MutationStats {
    /// Compute [`MutationStats`] from a [`MutationReport`].
    pub fn from_report(report: &MutationReport) -> Self {
        let total = report.total;
        let killed = report.killed;
        let survived = report.survived;
        let timed_out = report
            .mutations
            .iter()
            .filter(|(_, r)| *r == MutationResult::Timeout)
            .count();
        let compile_errors = report
            .mutations
            .iter()
            .filter(|(_, r)| *r == MutationResult::CompileError)
            .count();

        let mut by_operator: HashMap<String, (usize, usize)> = HashMap::new();
        for (mutation, result) in &report.mutations {
            let key = mutation.operator.to_string();
            let entry = by_operator.entry(key).or_insert((0, 0));
            entry.1 += 1;
            if *result == MutationResult::Killed {
                entry.0 += 1;
            }
        }

        Self {
            total,
            killed,
            survived,
            timed_out,
            compile_errors,
            by_operator,
        }
    }

    /// Fraction of mutations killed (killed / total). Returns 0.0 when total == 0.
    pub fn kill_rate(&self) -> f64 {
        if self.total == 0 {
            return 0.0;
        }
        self.killed as f64 / self.total as f64
    }

    /// Number of mutations that survived the test suite.
    pub fn survived_count(&self) -> usize {
        self.survived
    }

    /// Number of mutations killed by the test suite.
    pub fn killed_count(&self) -> usize {
        self.killed
    }

    /// Returns `true` when every mutation was killed (perfect test coverage).
    pub fn is_perfect(&self) -> bool {
        self.total > 0 && self.survived == 0 && self.timed_out == 0 && self.compile_errors == 0
    }

    /// Returns `true` when the kill rate meets or exceeds `threshold` (0.0–1.0).
    pub fn meets_threshold(&self, threshold: f64) -> bool {
        self.kill_rate() >= threshold
    }
}

impl fmt::Display for MutationStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MutationStats {{ total: {}, killed: {}, survived: {}, timed_out: {}, compile_errors: {}, kill_rate: {:.2}% }}",
            self.total,
            self.killed,
            self.survived,
            self.timed_out,
            self.compile_errors,
            self.kill_rate() * 100.0
        )
    }
}

// ============================================================
// MutationScanContext
// ============================================================

/// Context passed along during a source-text scan.
///
/// Tracks the current 1-based line and column as the scanner walks bytes
/// of a source file.
#[derive(Debug, Clone)]
pub struct MutationScanContext {
    /// Current 1-based line number.
    pub line: u32,
    /// Current 1-based column number.
    pub col: u32,
    /// File path being scanned.
    pub file: String,
}

impl MutationScanContext {
    /// Create a context positioned at the start of `file`.
    pub fn new(file: impl Into<String>) -> Self {
        Self {
            line: 1,
            col: 1,
            file: file.into(),
        }
    }

    /// Advance the context past `text`, updating line and column numbers.
    pub fn advance(&mut self, text: &str) {
        for ch in text.chars() {
            if ch == '\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
        }
    }
}

// ============================================================
// MutationFilter
// ============================================================

/// A predicate used to exclude certain mutations from a run.
#[derive(Debug, Clone)]
pub struct MutationFilter {
    /// Only include mutations from these files (empty = include all).
    pub allowed_files: Vec<String>,
    /// Exclude mutations produced by these operators.
    pub excluded_operators: Vec<MutationOperator>,
    /// Only include mutations on or after this 1-based line number.
    pub min_line: Option<u32>,
    /// Only include mutations on or before this 1-based line number.
    pub max_line: Option<u32>,
}

impl MutationFilter {
    /// A filter that accepts all mutations.
    pub fn accept_all() -> Self {
        Self {
            allowed_files: Vec::new(),
            excluded_operators: Vec::new(),
            min_line: None,
            max_line: None,
        }
    }

    /// Return `true` if `mutation` passes this filter.
    pub fn accepts(&self, mutation: &Mutation) -> bool {
        if !self.allowed_files.is_empty()
            && !self
                .allowed_files
                .iter()
                .any(|f| mutation.file.contains(f.as_str()))
        {
            return false;
        }
        if self.excluded_operators.contains(&mutation.operator) {
            return false;
        }
        if let Some(min) = self.min_line {
            if mutation.line < min {
                return false;
            }
        }
        if let Some(max) = self.max_line {
            if mutation.line > max {
                return false;
            }
        }
        true
    }
}

impl Default for MutationFilter {
    fn default() -> Self {
        Self::accept_all()
    }
}
