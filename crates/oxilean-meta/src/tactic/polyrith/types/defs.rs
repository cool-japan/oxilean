//! Type definitions for polyrith

use super::super::functions::*;

/// Cache for polyrith results.
#[allow(dead_code)]
pub struct PolyrithCache {
    pub entries: std::collections::HashMap<String, Option<Vec<i64>>>,
    pub hits: usize,
    pub misses: usize,
}

/// Statistics for the polyrith tactic.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct PolyrithStats {
    pub groebner_calls: usize,
    pub reduction_steps: usize,
    pub s_polynomials_computed: usize,
    pub certificates_found: usize,
    pub certificates_failed: usize,
}

#[allow(dead_code)]
pub struct PolyrithExtPipeline301 {
    pub name: String,
    pub passes: Vec<PolyrithExtPass301>,
    pub run_count: usize,
}

/// A configuration store for TacticPolyrith.
#[allow(dead_code)]
pub struct TacticPolyrithConfig {
    pub values: std::collections::HashMap<String, TacticPolyrithConfigValue>,
    pub read_only: bool,
}

#[allow(dead_code)]
pub struct PolyrithExtDiff300 {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}

#[allow(dead_code)]
pub struct PolyrithExtDiff301 {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum PolyrithExtResult301 {
    /// Operation completed successfully.
    Ok(String),
    /// Operation encountered an error.
    Err(String),
    /// Operation partially completed.
    Partial { done: usize, total: usize },
    /// Operation was skipped.
    Skipped,
}

/// A single monomial: a coefficient times a product of variable powers.
///
/// Represents `coefficient * x1^d1 * x2^d2 * ...`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Monomial {
    /// Ordered list of `(variable_name, exponent)` pairs.
    /// Variables with exponent 0 are omitted.
    pub vars: Vec<(String, u32)>,
    /// Integer coefficient.
    pub coefficient: i64,
}

#[allow(dead_code)]
pub struct PolyrithExtPass300 {
    pub name: String,
    pub total_runs: usize,
    pub successes: usize,
    pub errors: usize,
    pub enabled: bool,
    pub results: Vec<PolyrithExtResult300>,
}

/// A term in a multivariate polynomial: coefficient × monomial.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MVTerm {
    pub coeff: i64,
    pub mono: MonomialV2,
}

/// A polynomial coefficient (integer).
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PolyCoeff(pub i64);

/// A multivariate polynomial represented as a list of terms.
/// Each term: (coefficient, exponent_vector).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MultiTerm {
    pub coeff: i64,
    pub exps: Vec<u32>,
}

/// Configuration for the polyrith tactic.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PolyrithConfig {
    pub max_degree: u32,
    pub timeout_ms: u64,
    pub use_cache: bool,
    pub verbose: bool,
    pub max_coeffs: i64,
}

#[allow(dead_code)]
pub struct PolyrithExtDiag301 {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}

/// A polynomial in one variable with integer coefficients.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntPoly1 {
    pub coeffs: Vec<i64>,
}

/// A monomial ordering for Groebner bases (lexicographic).
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MonomialV2 {
    pub exponents: Vec<u32>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum PolyrithExtConfigVal300 {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}

/// A result type for TacticPolyrith analysis.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TacticPolyrithResult {
    Ok(String),
    Err(String),
    Partial { done: usize, total: usize },
    Skipped,
}

/// A (stub) Gröbner basis: a set of polynomials that generate an ideal.
#[derive(Clone, Debug, Default)]
pub struct GroebnerBasis {
    /// The generator polynomials.
    pub generators: Vec<Polynomial>,
}

/// A pipeline of TacticPolyrith analysis passes.
#[allow(dead_code)]
pub struct TacticPolyrithPipeline {
    pub passes: Vec<TacticPolyrithAnalysisPass>,
    pub name: String,
    pub total_inputs_processed: usize,
}

#[allow(dead_code)]
pub struct PolyrithExtPipeline300 {
    pub name: String,
    pub passes: Vec<PolyrithExtPass300>,
    pub run_count: usize,
}

#[allow(dead_code)]
pub struct PolyrithExtDiag300 {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}

pub(crate) struct CoeffIter<'a> {
    pub(crate) candidates: &'a [i64],
    pub(crate) indices: Vec<usize>,
    pub(crate) done: bool,
}

/// A multivariate polynomial represented as a sum of monomials.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Polynomial {
    /// The terms of the polynomial (not necessarily in canonical form).
    pub terms: Vec<Monomial>,
}

/// The `polyrith` tactic: closes polynomial identity goals by finding a
/// linear combination of hypotheses.
#[derive(Clone, Debug, Default)]
pub struct PolyrithTactic {
    pub(crate) hypotheses: Vec<Polynomial>,
    pub(crate) goal: Option<Polynomial>,
}

/// A typed slot for TacticPolyrith configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum TacticPolyrithConfigValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}

#[allow(dead_code)]
pub struct PolyrithExtConfig300 {
    pub(crate) values: std::collections::HashMap<String, PolyrithExtConfigVal300>,
    pub(crate) read_only: bool,
    pub(crate) name: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum PolyrithExtConfigVal301 {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}

/// A multivariate polynomial.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MultiPoly1 {
    pub nvars: usize,
    pub terms: Vec<MultiTerm>,
}

/// An analysis pass for TacticPolyrith.
#[allow(dead_code)]
pub struct TacticPolyrithAnalysisPass {
    pub name: String,
    pub enabled: bool,
    pub results: Vec<TacticPolyrithResult>,
    pub total_runs: usize,
}

/// A diagnostic reporter for TacticPolyrith.
#[allow(dead_code)]
pub struct TacticPolyrithDiagnostics {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}

#[allow(dead_code)]
pub struct PolyrithExtPass301 {
    pub name: String,
    pub total_runs: usize,
    pub successes: usize,
    pub errors: usize,
    pub enabled: bool,
    pub results: Vec<PolyrithExtResult301>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum PolyrithExtResult300 {
    /// Operation completed successfully.
    Ok(String),
    /// Operation encountered an error.
    Err(String),
    /// Operation partially completed.
    Partial { done: usize, total: usize },
    /// Operation was skipped.
    Skipped,
}

/// The polyrith solver: given polynomial hypotheses, find a linear combination.
#[allow(dead_code)]
pub struct PolyrithSolver {
    pub config: PolyrithConfig,
    pub cache: PolyrithCache,
    pub hyps: Vec<MultiPoly1>,
    pub goal: Option<MultiPoly1>,
}

/// A diff for TacticPolyrith analysis results.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TacticPolyrithDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}

/// A multivariate polynomial over integers.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MVPoly {
    pub nvars: usize,
    pub terms: Vec<MVTerm>,
}

/// Check if a polynomial is in the ideal generated by a set of polynomials.
#[allow(dead_code)]
pub struct IdealMembershipChecker {
    pub generators: Vec<MVPoly>,
    pub iterations_used: usize,
}

#[allow(dead_code)]
pub struct PolyrithExtConfig301 {
    pub(crate) values: std::collections::HashMap<String, PolyrithExtConfigVal301>,
    pub(crate) read_only: bool,
    pub(crate) name: String,
}
