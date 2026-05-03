//! Type definitions for linear_combination

use std::collections::HashMap;

/// An analysis pass for TacticLinearCombination.
#[allow(dead_code)]
pub struct TacticLinearCombinationAnalysisPass {
    pub name: String,
    pub enabled: bool,
    pub results: Vec<TacticLinearCombinationResult>,
    pub total_runs: usize,
}

/// A utility type for LinearComb (index 12).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LinearCombUtil12 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}

#[allow(dead_code)]
pub struct LinearCombinationExtPass2900 {
    pub name: String,
    pub total_runs: usize,
    pub successes: usize,
    pub errors: usize,
    pub enabled: bool,
    pub results: Vec<LinearCombinationExtResult2900>,
}

/// A priority queue for LinearComb items.
#[allow(dead_code)]
pub struct LinearCombPriorityQueue {
    pub items: Vec<(LinearCombUtil0, i64)>,
}

#[allow(dead_code)]
pub struct LinearCombinationExtConfig2900 {
    pub(crate) values: std::collections::HashMap<String, LinearCombinationExtConfigVal2900>,
    pub(crate) read_only: bool,
    pub(crate) name: String,
}

/// A utility type for LinearComb (index 14).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LinearCombUtil14 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}

/// A rational number p/q in lowest terms.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Rat {
    pub numer: i64,
    pub denom: i64,
}

/// A utility type for LinearComb (index 9).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LinearCombUtil9 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}

#[allow(dead_code)]
pub struct LinearCombinationExtDiag2900 {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}

/// A utility type for LinearComb (index 8).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LinearCombUtil8 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}

/// The linear_combination tactic.
#[derive(Clone, Debug)]
pub struct LinearCombinationTactic {
    /// Maximum coefficient magnitude to try.
    pub(crate) max_coeff: i64,
}

/// A pipeline of TacticLinearCombination analysis passes.
#[allow(dead_code)]
pub struct TacticLinearCombinationPipeline {
    pub passes: Vec<TacticLinearCombinationAnalysisPass>,
    pub name: String,
    pub total_inputs_processed: usize,
}

/// A diff for TacticLinearCombination analysis results.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TacticLinearCombinationDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}

/// A linear combination problem: given hypotheses prove the goal.
#[derive(Clone, Debug)]
pub struct LinearCombination {
    /// Named linear hypotheses.
    pub hypotheses: Vec<(String, LinCombExpr)>,
    /// The goal expression (we try to prove `goal = 0`).
    pub goal: LinCombExpr,
}

/// A utility type for LinearComb (index 7).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LinearCombUtil7 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}

/// A logger for LinearComb operations.
#[allow(dead_code)]
pub struct LinearCombLogger {
    pub entries: Vec<String>,
    pub max_entries: usize,
    pub verbose: bool,
}

/// Statistics for LinearComb operations.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LinearCombStats {
    pub total_ops: usize,
    pub successful_ops: usize,
    pub failed_ops: usize,
    pub total_time_ns: u64,
    pub max_time_ns: u64,
}

/// A registry for LinearComb utilities.
#[allow(dead_code)]
pub struct LinearCombRegistry {
    pub entries: Vec<LinearCombUtil0>,
    pub capacity: usize,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum LinearCombinationExtResult2900 {
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
pub struct LinearCombinationExtDiff2900 {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum LinearCombinationExtConfigVal2900 {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}

#[allow(dead_code)]
pub struct LinearCombinationExtPipeline2900 {
    pub name: String,
    pub passes: Vec<LinearCombinationExtPass2900>,
    pub run_count: usize,
}

/// A univariate polynomial with i64 coefficients.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UniPoly {
    pub coeffs: Vec<i64>,
}

/// A utility type for LinearComb (index 11).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LinearCombUtil11 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}

/// A utility type for LinearComb (index 10).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LinearCombUtil10 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}

/// A utility type for LinearComb (index 13).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LinearCombUtil13 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}

/// A matrix with rational entries.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RatMatrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<Rat>>,
}

/// A Farkas certificate for infeasibility of a linear system.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FarkasCert {
    /// Non-negative multipliers for each inequality.
    pub multipliers: Vec<Rat>,
    /// The resulting contradiction value (should be < 0 for infeasibility).
    pub combined_rhs: Rat,
}

/// A configuration store for TacticLinearCombination.
#[allow(dead_code)]
pub struct TacticLinearCombinationConfig {
    pub values: std::collections::HashMap<String, TacticLinearCombinationConfigValue>,
    pub read_only: bool,
}

/// A single term in a linear expression: `coefficient * variable`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinCombTerm {
    /// The scalar coefficient.
    pub coefficient: i64,
    /// The variable name.
    pub variable: String,
}

/// A utility type for LinearComb (index 2).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LinearCombUtil2 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}

/// A utility type for LinearComb (index 0).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LinearCombUtil0 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}

/// A simple 2-variable LP solver over integers (bounded domain).
#[allow(dead_code)]
pub struct SimpleLp {
    pub constraints: Vec<([i64; 2], i64)>,
    pub objective: [i64; 2],
    pub bounds: i64,
}

/// Result of an LP solve.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LpSolveResult {
    Optimal(i64, i64),
    Infeasible,
    Unbounded,
}

/// A linear combination: map from variable → coefficient + constant term.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LinCombMap {
    pub terms: std::collections::HashMap<String, i64>,
    pub constant: i64,
}

/// A utility type for LinearComb (index 4).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LinearCombUtil4 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}

/// A utility type for LinearComb (index 5).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LinearCombUtil5 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}

/// A utility type for LinearComb (index 3).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LinearCombUtil3 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}

/// A utility type for LinearComb (index 6).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LinearCombUtil6 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}

/// A linear expression: `constant + c₁·x₁ + c₂·x₂ + ...`
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinCombExpr {
    /// The variable terms.
    pub terms: Vec<LinCombTerm>,
    /// The constant part.
    pub constant: i64,
}

/// A result type for TacticLinearCombination analysis.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TacticLinearCombinationResult {
    Ok(String),
    Err(String),
    Partial { done: usize, total: usize },
    Skipped,
}

/// A simple cache for LinearComb computations.
#[allow(dead_code)]
pub struct LinearCombCache {
    pub data: std::collections::HashMap<String, i64>,
    pub hits: usize,
    pub misses: usize,
}

/// A diagnostic reporter for TacticLinearCombination.
#[allow(dead_code)]
pub struct TacticLinearCombinationDiagnostics {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}

/// A utility type for LinearComb (index 1).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LinearCombUtil1 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}

/// A typed slot for TacticLinearCombination configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum TacticLinearCombinationConfigValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
