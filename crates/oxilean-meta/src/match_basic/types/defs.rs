//! Type definitions for match_basic

use oxilean_kernel::{Expr, Literal, Name};

/// A typed slot for MatchBasic configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum MatchBasicConfigValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}

/// A utility type for MatchBasic (index 12).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MatchBasicUtil12 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}

#[allow(dead_code)]
pub struct MatchBasicExtPass4100 {
    pub name: String,
    pub total_runs: usize,
    pub successes: usize,
    pub errors: usize,
    pub enabled: bool,
    pub results: Vec<MatchBasicExtResult4100>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum MatchBasicExtConfigVal4100 {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}

/// A diagnostic reporter for MatchBasic.
#[allow(dead_code)]
pub struct MatchBasicDiagnostics {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}

/// A configuration store for MatchBasic.
#[allow(dead_code)]
pub struct MatchBasicConfig {
    pub values: std::collections::HashMap<String, MatchBasicConfigValue>,
    pub read_only: bool,
}

/// A utility type for MatchBasic (index 3).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MatchBasicUtil3 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}

/// A pattern in a match expression.
#[derive(Clone, Debug, PartialEq)]
pub enum MetaPattern {
    /// Wildcard pattern: matches anything, binds nothing.
    Wildcard,
    /// Variable pattern: matches anything, binds the value.
    Var(Name),
    /// Constructor pattern: matches if head is this constructor.
    Constructor(Name, Vec<MetaPattern>),
    /// Literal pattern: matches exact literal value.
    Literal(Literal),
    /// As-pattern: `p as x` matches p and also binds x.
    As(Box<MetaPattern>, Name),
    /// Or-pattern: matches if either sub-pattern matches.
    Or(Box<MetaPattern>, Box<MetaPattern>),
    /// Inaccessible pattern (dot pattern): `.e`.
    Inaccessible(Expr),
}

/// A pattern matrix: a list of rows.
#[derive(Clone, Debug, Default)]
pub struct PatternMatrix {
    /// Rows, one per match arm.
    pub rows: Vec<PatternRow>,
    /// Number of discriminants (columns).
    pub num_discriminants: usize,
}

/// A pipeline of MatchBasic analysis passes.
#[allow(dead_code)]
pub struct MatchBasicPipeline {
    pub passes: Vec<MatchBasicAnalysisPass>,
    pub name: String,
    pub total_inputs_processed: usize,
}

/// The result of trying to match a pattern against an expression.
#[derive(Debug, Clone, PartialEq)]
pub enum MatchResult {
    /// The match definitely succeeds; bindings maps variable names to expressions.
    Success(Vec<(oxilean_kernel::Name, Expr)>),
    /// The match definitely fails.
    Failure,
    /// The match is undetermined (e.g., scrutinee not in WHNF yet).
    Undetermined,
}

#[allow(dead_code)]
pub struct MatchBasicExtDiff4100 {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}

/// A utility type for MatchBasic (index 9).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MatchBasicUtil9 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}

/// A priority queue for MatchBasic items.
#[allow(dead_code)]
pub struct MatchBasicPriorityQueue {
    pub items: Vec<(MatchBasicUtil0, i64)>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum MatchBasicExtResult4100 {
    /// Operation completed successfully.
    Ok(String),
    /// Operation encountered an error.
    Err(String),
    /// Operation partially completed.
    Partial { done: usize, total: usize },
    /// Operation was skipped.
    Skipped,
}

#[allow(dead_code)]
pub struct MatchBasicExtDiag4100 {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}

/// A row in a pattern matrix.
#[derive(Clone, Debug)]
pub struct PatternRow {
    /// The patterns in this row (one per discriminant).
    pub patterns: Vec<MetaPattern>,
    /// Index of the match arm this row corresponds to.
    pub arm_index: usize,
    /// Optional guard.
    pub guard: Option<Expr>,
}

/// A utility type for MatchBasic (index 7).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MatchBasicUtil7 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}

/// A utility type for MatchBasic (index 4).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MatchBasicUtil4 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}

/// A simple cache for MatchBasic computations.
#[allow(dead_code)]
pub struct MatchBasicCache {
    pub data: std::collections::HashMap<String, i64>,
    pub hits: usize,
    pub misses: usize,
}

/// A diff for MatchBasic analysis results.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MatchBasicDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}

/// A registry for MatchBasic utilities.
#[allow(dead_code)]
pub struct MatchBasicRegistry {
    pub entries: Vec<MatchBasicUtil0>,
    pub capacity: usize,
}

/// A utility type for MatchBasic (index 0).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MatchBasicUtil0 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}

/// A utility type for MatchBasic (index 6).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MatchBasicUtil6 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}

/// A utility type for MatchBasic (index 11).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MatchBasicUtil11 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}

/// A result type for MatchBasic analysis.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum MatchBasicResult {
    Ok(String),
    Err(String),
    Partial { done: usize, total: usize },
    Skipped,
}

/// A utility type for MatchBasic (index 10).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MatchBasicUtil10 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}

/// A match expression being elaborated.
#[derive(Clone, Debug)]
pub struct MetaMatchExpr {
    /// Discriminant expressions.
    pub discriminants: Vec<Expr>,
    /// Types of discriminants.
    pub discr_types: Vec<Expr>,
    /// Match arms.
    pub arms: Vec<MetaMatchArm>,
    /// Expected result type.
    pub expected_type: Option<Expr>,
}

/// A node in a match decision tree.
///
/// Decision trees are a compiled form of pattern matching that avoid
/// repeatedly testing the same sub-expression.
#[derive(Debug, Clone)]
pub enum DecisionTree {
    /// Leaf: all patterns exhausted; execute this arm index.
    Leaf(usize),
    /// Failure: no arm matches.
    Fail,
    /// Switch on the constructor of a sub-expression.
    Switch {
        /// Index of the discriminant to examine (in the match clause).
        discr_idx: usize,
        /// Cases keyed by constructor name.
        cases: Vec<(oxilean_kernel::Name, Box<DecisionTree>)>,
        /// Default case (wildcard/variable patterns).
        default: Option<Box<DecisionTree>>,
    },
    /// Guard: check a boolean expression before proceeding.
    Guard {
        /// The guard expression to evaluate.
        condition: Expr,
        /// Sub-tree for when the guard holds.
        success: Box<DecisionTree>,
        /// Sub-tree for when the guard fails.
        failure: Box<DecisionTree>,
    },
}

/// A utility type for MatchBasic (index 13).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MatchBasicUtil13 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}

/// An analysis pass for MatchBasic.
#[allow(dead_code)]
pub struct MatchBasicAnalysisPass {
    pub name: String,
    pub enabled: bool,
    pub results: Vec<MatchBasicResult>,
    pub total_runs: usize,
}

/// A match arm (clause) in a match expression.
#[derive(Clone, Debug)]
pub struct MetaMatchArm {
    /// Patterns for each discriminant.
    pub patterns: Vec<MetaPattern>,
    /// Optional guard expression.
    pub guard: Option<Expr>,
    /// Right-hand side (body).
    pub rhs: Expr,
}

/// A utility type for MatchBasic (index 1).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MatchBasicUtil1 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}

/// Statistics for MatchBasic operations.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MatchBasicStats {
    pub total_ops: usize,
    pub successful_ops: usize,
    pub failed_ops: usize,
    pub total_time_ns: u64,
    pub max_time_ns: u64,
}

#[allow(dead_code)]
pub struct MatchBasicExtConfig4100 {
    pub(crate) values: std::collections::HashMap<String, MatchBasicExtConfigVal4100>,
    pub(crate) read_only: bool,
    pub(crate) name: String,
}

/// A utility type for MatchBasic (index 2).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MatchBasicUtil2 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}

/// A utility type for MatchBasic (index 5).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MatchBasicUtil5 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}

/// A logger for MatchBasic operations.
#[allow(dead_code)]
pub struct MatchBasicLogger {
    pub entries: Vec<String>,
    pub max_entries: usize,
    pub verbose: bool,
}

/// A utility type for MatchBasic (index 8).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MatchBasicUtil8 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}

#[allow(dead_code)]
pub struct MatchBasicExtPipeline4100 {
    pub name: String,
    pub passes: Vec<MatchBasicExtPass4100>,
    pub run_count: usize,
}
