//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use crate::tactic::state::{TacticError, TacticResult};
use std::collections::HashMap;

/// Comparison result normalization
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ComparisonOp {
    /// Less than or equal
    Le,
    /// Less than
    Lt,
    /// Equals
    Eq,
    /// Greater than or equal
    Ge,
    /// Greater than
    Gt,
}
impl ComparisonOp {
    /// Compare two numeric values
    pub fn compare(&self, a: &NumericValue, b: &NumericValue) -> bool {
        let af = a.to_float();
        let bf = b.to_float();
        match self {
            ComparisonOp::Le => af <= bf,
            ComparisonOp::Lt => af < bf,
            ComparisonOp::Eq => a == b,
            ComparisonOp::Ge => af >= bf,
            ComparisonOp::Gt => af > bf,
        }
    }
    /// Flip the comparison (< becomes >, etc.)
    pub fn flip(&self) -> ComparisonOp {
        match self {
            ComparisonOp::Le => ComparisonOp::Ge,
            ComparisonOp::Lt => ComparisonOp::Gt,
            ComparisonOp::Eq => ComparisonOp::Eq,
            ComparisonOp::Ge => ComparisonOp::Le,
            ComparisonOp::Gt => ComparisonOp::Lt,
        }
    }
    /// Negate the comparison
    pub fn negate(&self) -> ComparisonOp {
        match self {
            ComparisonOp::Le => ComparisonOp::Gt,
            ComparisonOp::Lt => ComparisonOp::Ge,
            ComparisonOp::Eq => ComparisonOp::Eq,
            ComparisonOp::Ge => ComparisonOp::Lt,
            ComparisonOp::Gt => ComparisonOp::Le,
        }
    }
}
/// A result type for TacticNormNum analysis.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TacticNormNumResult {
    Ok(String),
    Err(String),
    Partial { done: usize, total: usize },
    Skipped,
}
#[allow(dead_code)]
impl TacticNormNumResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, TacticNormNumResult::Ok(_))
    }
    pub fn is_err(&self) -> bool {
        matches!(self, TacticNormNumResult::Err(_))
    }
    pub fn is_partial(&self) -> bool {
        matches!(self, TacticNormNumResult::Partial { .. })
    }
    pub fn is_skipped(&self) -> bool {
        matches!(self, TacticNormNumResult::Skipped)
    }
    pub fn ok_msg(&self) -> Option<&str> {
        match self {
            TacticNormNumResult::Ok(s) => Some(s),
            _ => None,
        }
    }
    pub fn err_msg(&self) -> Option<&str> {
        match self {
            TacticNormNumResult::Err(s) => Some(s),
            _ => None,
        }
    }
    pub fn progress(&self) -> f64 {
        match self {
            TacticNormNumResult::Ok(_) => 1.0,
            TacticNormNumResult::Err(_) => 0.0,
            TacticNormNumResult::Skipped => 0.0,
            TacticNormNumResult::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
        }
    }
}
/// A diff for TacticNormNum analysis results.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TacticNormNumDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
#[allow(dead_code)]
impl TacticNormNumDiff {
    pub fn new() -> Self {
        TacticNormNumDiff {
            added: Vec::new(),
            removed: Vec::new(),
            unchanged: Vec::new(),
        }
    }
    pub fn add(&mut self, s: &str) {
        self.added.push(s.to_string());
    }
    pub fn remove(&mut self, s: &str) {
        self.removed.push(s.to_string());
    }
    pub fn keep(&mut self, s: &str) {
        self.unchanged.push(s.to_string());
    }
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty()
    }
    pub fn total_changes(&self) -> usize {
        self.added.len() + self.removed.len()
    }
    pub fn net_additions(&self) -> i64 {
        self.added.len() as i64 - self.removed.len() as i64
    }
    pub fn summary(&self) -> String {
        format!(
            "+{} -{} =={}",
            self.added.len(),
            self.removed.len(),
            self.unchanged.len()
        )
    }
}
/// A diagnostic reporter for TacticNormNum.
#[allow(dead_code)]
pub struct TacticNormNumDiagnostics {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
#[allow(dead_code)]
impl TacticNormNumDiagnostics {
    pub fn new(max_errors: usize) -> Self {
        TacticNormNumDiagnostics {
            errors: Vec::new(),
            warnings: Vec::new(),
            notes: Vec::new(),
            max_errors,
        }
    }
    pub fn error(&mut self, msg: &str) {
        if self.errors.len() < self.max_errors {
            self.errors.push(msg.to_string());
        }
    }
    pub fn warning(&mut self, msg: &str) {
        self.warnings.push(msg.to_string());
    }
    pub fn note(&mut self, msg: &str) {
        self.notes.push(msg.to_string());
    }
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    pub fn num_errors(&self) -> usize {
        self.errors.len()
    }
    pub fn num_warnings(&self) -> usize {
        self.warnings.len()
    }
    pub fn is_clean(&self) -> bool {
        self.errors.is_empty() && self.warnings.is_empty()
    }
    pub fn at_error_limit(&self) -> bool {
        self.errors.len() >= self.max_errors
    }
    pub fn clear(&mut self) {
        self.errors.clear();
        self.warnings.clear();
        self.notes.clear();
    }
    pub fn summary(&self) -> String {
        format!(
            "{} error(s), {} warning(s)",
            self.errors.len(),
            self.warnings.len()
        )
    }
}
#[allow(dead_code)]
pub struct NormNumExtPipeline2800 {
    pub name: String,
    pub passes: Vec<NormNumExtPass2800>,
    pub run_count: usize,
}
impl NormNumExtPipeline2800 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            passes: Vec::new(),
            run_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_pass(&mut self, pass: NormNumExtPass2800) {
        self.passes.push(pass);
    }
    #[allow(dead_code)]
    pub fn run_all(&mut self, input: &str) -> Vec<NormNumExtResult2800> {
        self.run_count += 1;
        self.passes
            .iter_mut()
            .filter(|p| p.enabled)
            .map(|p| p.run(input))
            .collect()
    }
    #[allow(dead_code)]
    pub fn num_passes(&self) -> usize {
        self.passes.len()
    }
    #[allow(dead_code)]
    pub fn num_enabled_passes(&self) -> usize {
        self.passes.iter().filter(|p| p.enabled).count()
    }
    #[allow(dead_code)]
    pub fn total_success_rate(&self) -> f64 {
        let total: usize = self.passes.iter().map(|p| p.total_runs).sum();
        let ok: usize = self.passes.iter().map(|p| p.successes).sum();
        if total == 0 {
            0.0
        } else {
            ok as f64 / total as f64
        }
    }
}
/// A configuration store for TacticNormNum.
#[allow(dead_code)]
pub struct TacticNormNumConfig {
    pub values: std::collections::HashMap<String, TacticNormNumConfigValue>,
    pub read_only: bool,
}
#[allow(dead_code)]
impl TacticNormNumConfig {
    pub fn new() -> Self {
        TacticNormNumConfig {
            values: std::collections::HashMap::new(),
            read_only: false,
        }
    }
    pub fn set(&mut self, key: &str, value: TacticNormNumConfigValue) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    pub fn get(&self, key: &str) -> Option<&TacticNormNumConfigValue> {
        self.values.get(key)
    }
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.get(key)?.as_bool()
    }
    pub fn get_int(&self, key: &str) -> Option<i64> {
        self.get(key)?.as_int()
    }
    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.get(key)?.as_str()
    }
    pub fn set_bool(&mut self, key: &str, v: bool) -> bool {
        self.set(key, TacticNormNumConfigValue::Bool(v))
    }
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, TacticNormNumConfigValue::Int(v))
    }
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, TacticNormNumConfigValue::Str(v.to_string()))
    }
    pub fn lock(&mut self) {
        self.read_only = true;
    }
    pub fn unlock(&mut self) {
        self.read_only = false;
    }
    pub fn size(&self) -> usize {
        self.values.len()
    }
    pub fn has(&self, key: &str) -> bool {
        self.values.contains_key(key)
    }
    pub fn remove(&mut self, key: &str) -> bool {
        self.values.remove(key).is_some()
    }
}
#[allow(dead_code)]
pub struct NormNumExtDiff2800 {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
impl NormNumExtDiff2800 {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            added: Vec::new(),
            removed: Vec::new(),
            unchanged: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn add(&mut self, s: &str) {
        self.added.push(s.to_string());
    }
    #[allow(dead_code)]
    pub fn remove(&mut self, s: &str) {
        self.removed.push(s.to_string());
    }
    #[allow(dead_code)]
    pub fn keep(&mut self, s: &str) {
        self.unchanged.push(s.to_string());
    }
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty()
    }
    #[allow(dead_code)]
    pub fn total_changes(&self) -> usize {
        self.added.len() + self.removed.len()
    }
    #[allow(dead_code)]
    pub fn net_additions(&self) -> i64 {
        self.added.len() as i64 - self.removed.len() as i64
    }
    #[allow(dead_code)]
    pub fn summary(&self) -> String {
        format!(
            "+{} -{} =={}",
            self.added.len(),
            self.removed.len(),
            self.unchanged.len()
        )
    }
}
/// A linear expression: `c0 + c1*x1 + c2*x2 + ...`
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct LinearExpr {
    /// Constant term.
    pub constant: i64,
    /// Variable coefficients: (variable_name, coefficient).
    pub vars: Vec<(String, i64)>,
}
impl LinearExpr {
    /// Create a constant linear expression.
    pub fn constant(c: i64) -> Self {
        Self {
            constant: c,
            vars: vec![],
        }
    }
    /// Create a single-variable expression `c * x`.
    pub fn var(name: &str, coeff: i64) -> Self {
        Self {
            constant: 0,
            vars: vec![(name.to_string(), coeff)],
        }
    }
    /// Add two linear expressions.
    pub fn add(&self, other: &LinearExpr) -> LinearExpr {
        let mut result = self.clone();
        result.constant = result.constant.saturating_add(other.constant);
        for (name, coeff) in &other.vars {
            if let Some(entry) = result.vars.iter_mut().find(|(n, _)| n == name) {
                entry.1 = entry.1.saturating_add(*coeff);
            } else {
                result.vars.push((name.clone(), *coeff));
            }
        }
        result.vars.retain(|(_, c)| *c != 0);
        result
    }
    /// Negate the expression.
    pub fn negate(&self) -> LinearExpr {
        LinearExpr {
            constant: self.constant.saturating_neg(),
            vars: self
                .vars
                .iter()
                .map(|(n, c)| (n.clone(), c.saturating_neg()))
                .collect(),
        }
    }
    /// Subtract `other` from `self`.
    pub fn sub(&self, other: &LinearExpr) -> LinearExpr {
        self.add(&other.negate())
    }
    /// Scale by an integer constant.
    pub fn scale(&self, c: i64) -> LinearExpr {
        LinearExpr {
            constant: self.constant.saturating_mul(c),
            vars: self
                .vars
                .iter()
                .map(|(n, v)| (n.clone(), v.saturating_mul(c)))
                .collect(),
        }
    }
    /// Evaluate given a variable assignment.
    pub fn eval(&self, assignment: &HashMap<String, i64>) -> i64 {
        let mut result = self.constant;
        for (name, coeff) in &self.vars {
            if let Some(&val) = assignment.get(name) {
                result = result.saturating_add(coeff.saturating_mul(val));
            }
        }
        result
    }
    /// Whether the expression is a constant (no variables).
    pub fn is_constant(&self) -> bool {
        self.vars.iter().all(|(_, c)| *c == 0) || self.vars.is_empty()
    }
    /// Number of variables with non-zero coefficients.
    pub fn num_vars(&self) -> usize {
        self.vars.iter().filter(|(_, c)| *c != 0).count()
    }
}
/// Represents a normalized numeric value
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum NumericValue {
    /// Natural number
    Nat(u64),
    /// Integer
    Int(i64),
    /// Rational: numerator / denominator
    Rat(i64, u64),
}
impl NumericValue {
    /// Create a natural number
    pub fn nat(n: u64) -> Self {
        NumericValue::Nat(n)
    }
    /// Create an integer
    pub fn int(n: i64) -> Self {
        NumericValue::Int(n)
    }
    /// Create a rational number (automatically reduced)
    pub fn rat(num: i64, den: u64) -> Self {
        if den == 0 {
            panic!("zero denominator");
        }
        let g = gcd(num.unsigned_abs(), den);
        let reduced_num = num / g as i64;
        let reduced_den = den / g;
        NumericValue::Rat(reduced_num, reduced_den)
    }
    /// Convert to float for comparisons
    pub fn to_float(&self) -> f64 {
        match self {
            NumericValue::Nat(n) => *n as f64,
            NumericValue::Int(i) => *i as f64,
            NumericValue::Rat(num, den) => *num as f64 / *den as f64,
        }
    }
    /// Check if zero
    pub fn is_zero(&self) -> bool {
        match self {
            NumericValue::Nat(n) => *n == 0,
            NumericValue::Int(i) => *i == 0,
            NumericValue::Rat(num, _) => *num == 0,
        }
    }
    /// Check if one
    pub fn is_one(&self) -> bool {
        match self {
            NumericValue::Nat(n) => *n == 1,
            NumericValue::Int(i) => *i == 1,
            NumericValue::Rat(num, den) => *num == 1 && *den == 1,
        }
    }
    /// Check if negative
    pub fn is_negative(&self) -> bool {
        match self {
            NumericValue::Nat(_) => false,
            NumericValue::Int(i) => *i < 0,
            NumericValue::Rat(num, _) => *num < 0,
        }
    }
    /// Negate the value
    pub fn negate(&self) -> NumericValue {
        match self {
            NumericValue::Nat(n) => NumericValue::Int(-(*n as i64)),
            NumericValue::Int(i) => NumericValue::Int(-i),
            NumericValue::Rat(num, den) => NumericValue::Rat(-num, *den),
        }
    }
    /// Add two numeric values
    pub fn add(&self, other: &NumericValue) -> NumericValue {
        match (self, other) {
            (NumericValue::Nat(a), NumericValue::Nat(b)) => NumericValue::Nat(a + b),
            (NumericValue::Nat(a), NumericValue::Int(b)) => NumericValue::Int(*a as i64 + b),
            (NumericValue::Int(a), NumericValue::Nat(b)) => NumericValue::Int(a + *b as i64),
            (NumericValue::Int(a), NumericValue::Int(b)) => NumericValue::Int(a + b),
            (NumericValue::Nat(a), NumericValue::Rat(num, den)) => {
                NumericValue::Rat(*a as i64 * *den as i64 + num, *den)
            }
            (NumericValue::Rat(num, den), NumericValue::Nat(b)) => {
                NumericValue::Rat(num + *b as i64 * *den as i64, *den)
            }
            (NumericValue::Int(a), NumericValue::Rat(num, den)) => {
                NumericValue::Rat(*a * *den as i64 + num, *den)
            }
            (NumericValue::Rat(num, den), NumericValue::Int(b)) => {
                NumericValue::Rat(num + *b * *den as i64, *den)
            }
            (NumericValue::Rat(num1, den1), NumericValue::Rat(num2, den2)) => {
                let new_den = *den1 * *den2;
                let new_num = num1 * (*den2 as i64) + num2 * (*den1 as i64);
                NumericValue::rat(new_num, new_den)
            }
        }
    }
    /// Subtract two numeric values
    pub fn sub(&self, other: &NumericValue) -> NumericValue {
        self.add(&other.negate())
    }
    /// Multiply two numeric values
    pub fn mul(&self, other: &NumericValue) -> NumericValue {
        match (self, other) {
            (NumericValue::Nat(a), NumericValue::Nat(b)) => NumericValue::Nat(a * b),
            (NumericValue::Nat(a), NumericValue::Int(b)) => NumericValue::Int(*a as i64 * b),
            (NumericValue::Int(a), NumericValue::Nat(b)) => NumericValue::Int(a * *b as i64),
            (NumericValue::Int(a), NumericValue::Int(b)) => NumericValue::Int(a * b),
            (NumericValue::Nat(a), NumericValue::Rat(num, den)) => {
                NumericValue::rat(*a as i64 * num, *den)
            }
            (NumericValue::Rat(num, den), NumericValue::Nat(b)) => {
                NumericValue::rat(num * *b as i64, *den)
            }
            (NumericValue::Int(a), NumericValue::Rat(num, den)) => {
                NumericValue::rat(*a * num, *den)
            }
            (NumericValue::Rat(num, den), NumericValue::Int(b)) => {
                NumericValue::rat(num * *b, *den)
            }
            (NumericValue::Rat(num1, den1), NumericValue::Rat(num2, den2)) => {
                NumericValue::rat(num1 * num2, *den1 * *den2)
            }
        }
    }
    /// Divide two numeric values (returns Rat)
    pub fn div(&self, other: &NumericValue) -> TacticResult<NumericValue> {
        if other.is_zero() {
            return Err(TacticError::Failed("division by zero".into()));
        }
        match (self, other) {
            (NumericValue::Nat(a), NumericValue::Nat(b)) => Ok(NumericValue::rat(*a as i64, *b)),
            (NumericValue::Int(a), NumericValue::Int(b)) => {
                Ok(NumericValue::rat(*a, b.unsigned_abs()))
            }
            (NumericValue::Nat(a), NumericValue::Int(b)) => {
                Ok(NumericValue::rat(*a as i64, b.unsigned_abs()))
            }
            (NumericValue::Int(a), NumericValue::Nat(b)) => Ok(NumericValue::rat(*a, *b)),
            (NumericValue::Rat(num1, den1), NumericValue::Rat(num2, den2)) => Ok(
                NumericValue::rat(num1 * *den2 as i64, *den1 * num2.unsigned_abs()),
            ),
            _ => Err(TacticError::Failed("unsupported division types".into())),
        }
    }
    /// Power: self^n
    pub fn pow(&self, n: u32) -> TacticResult<NumericValue> {
        if n == 0 {
            return Ok(NumericValue::Nat(1));
        }
        let mut result = self.clone();
        for _ in 1..n {
            result = result.mul(self);
        }
        Ok(result)
    }
}
#[allow(dead_code)]
pub struct NormNumExtConfig2800 {
    pub(super) values: std::collections::HashMap<String, NormNumExtConfigVal2800>,
    pub(super) read_only: bool,
    pub(super) name: String,
}
impl NormNumExtConfig2800 {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            values: std::collections::HashMap::new(),
            read_only: false,
            name: String::new(),
        }
    }
    #[allow(dead_code)]
    pub fn named(name: &str) -> Self {
        Self {
            values: std::collections::HashMap::new(),
            read_only: false,
            name: name.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn set(&mut self, key: &str, value: NormNumExtConfigVal2800) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&NormNumExtConfigVal2800> {
        self.values.get(key)
    }
    #[allow(dead_code)]
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.get(key)?.as_bool()
    }
    #[allow(dead_code)]
    pub fn get_int(&self, key: &str) -> Option<i64> {
        self.get(key)?.as_int()
    }
    #[allow(dead_code)]
    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.get(key)?.as_str()
    }
    #[allow(dead_code)]
    pub fn set_bool(&mut self, key: &str, v: bool) -> bool {
        self.set(key, NormNumExtConfigVal2800::Bool(v))
    }
    #[allow(dead_code)]
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, NormNumExtConfigVal2800::Int(v))
    }
    #[allow(dead_code)]
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, NormNumExtConfigVal2800::Str(v.to_string()))
    }
    #[allow(dead_code)]
    pub fn lock(&mut self) {
        self.read_only = true;
    }
    #[allow(dead_code)]
    pub fn unlock(&mut self) {
        self.read_only = false;
    }
    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.values.len()
    }
    #[allow(dead_code)]
    pub fn has(&self, key: &str) -> bool {
        self.values.contains_key(key)
    }
    #[allow(dead_code)]
    pub fn remove(&mut self, key: &str) -> bool {
        self.values.remove(key).is_some()
    }
}
#[allow(dead_code)]
pub struct NormNumExtDiag2800 {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
impl NormNumExtDiag2800 {
    #[allow(dead_code)]
    pub fn new(max_errors: usize) -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
            notes: Vec::new(),
            max_errors,
        }
    }
    #[allow(dead_code)]
    pub fn error(&mut self, msg: &str) {
        if self.errors.len() < self.max_errors {
            self.errors.push(msg.to_string());
        }
    }
    #[allow(dead_code)]
    pub fn warning(&mut self, msg: &str) {
        self.warnings.push(msg.to_string());
    }
    #[allow(dead_code)]
    pub fn note(&mut self, msg: &str) {
        self.notes.push(msg.to_string());
    }
    #[allow(dead_code)]
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    #[allow(dead_code)]
    pub fn num_errors(&self) -> usize {
        self.errors.len()
    }
    #[allow(dead_code)]
    pub fn num_warnings(&self) -> usize {
        self.warnings.len()
    }
    #[allow(dead_code)]
    pub fn is_clean(&self) -> bool {
        self.errors.is_empty() && self.warnings.is_empty()
    }
    #[allow(dead_code)]
    pub fn at_error_limit(&self) -> bool {
        self.errors.len() >= self.max_errors
    }
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.errors.clear();
        self.warnings.clear();
        self.notes.clear();
    }
    #[allow(dead_code)]
    pub fn summary(&self) -> String {
        format!(
            "{} error(s), {} warning(s)",
            self.errors.len(),
            self.warnings.len()
        )
    }
}
/// A univariate polynomial with integer coefficients.
///
/// Represented as a list of coefficients: `coeffs[i]` is the coefficient
/// of `x^i`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Poly {
    /// Coefficients in ascending degree order.
    pub coeffs: Vec<i64>,
}
impl Poly {
    /// The zero polynomial.
    pub fn zero() -> Self {
        Self { coeffs: vec![] }
    }
    /// A constant polynomial.
    pub fn constant(c: i64) -> Self {
        if c == 0 {
            Self::zero()
        } else {
            Self { coeffs: vec![c] }
        }
    }
    /// The identity polynomial `x`.
    pub fn ident() -> Self {
        Self { coeffs: vec![0, 1] }
    }
    /// Degree of the polynomial (-1 for zero polynomial).
    pub fn degree(&self) -> isize {
        let trimmed = self.trim();
        if trimmed.coeffs.is_empty() {
            -1
        } else {
            (trimmed.coeffs.len() - 1) as isize
        }
    }
    /// Remove trailing zero coefficients.
    pub fn trim(&self) -> Self {
        let mut coeffs = self.coeffs.clone();
        while coeffs.last() == Some(&0) {
            coeffs.pop();
        }
        Self { coeffs }
    }
    /// Evaluate the polynomial at `x`.
    pub fn eval(&self, x: i64) -> i64 {
        let mut result = 0i64;
        let mut power = 1i64;
        for &c in &self.coeffs {
            result = result.saturating_add(c.saturating_mul(power));
            power = power.saturating_mul(x);
        }
        result
    }
    /// Add two polynomials.
    pub fn add(&self, other: &Poly) -> Poly {
        let len = self.coeffs.len().max(other.coeffs.len());
        let mut coeffs = vec![0i64; len];
        for (i, &c) in self.coeffs.iter().enumerate() {
            coeffs[i] = coeffs[i].saturating_add(c);
        }
        for (i, &c) in other.coeffs.iter().enumerate() {
            coeffs[i] = coeffs[i].saturating_add(c);
        }
        Self { coeffs }.trim()
    }
    /// Subtract `other` from `self`.
    pub fn sub(&self, other: &Poly) -> Poly {
        let len = self.coeffs.len().max(other.coeffs.len());
        let mut coeffs = vec![0i64; len];
        for (i, &c) in self.coeffs.iter().enumerate() {
            coeffs[i] = coeffs[i].saturating_add(c);
        }
        for (i, &c) in other.coeffs.iter().enumerate() {
            coeffs[i] = coeffs[i].saturating_sub(c);
        }
        Self { coeffs }.trim()
    }
    /// Multiply two polynomials.
    pub fn mul(&self, other: &Poly) -> Poly {
        if self.coeffs.is_empty() || other.coeffs.is_empty() {
            return Poly::zero();
        }
        let len = self.coeffs.len() + other.coeffs.len() - 1;
        let mut coeffs = vec![0i64; len];
        for (i, &a) in self.coeffs.iter().enumerate() {
            for (j, &b) in other.coeffs.iter().enumerate() {
                coeffs[i + j] = coeffs[i + j].saturating_add(a.saturating_mul(b));
            }
        }
        Self { coeffs }.trim()
    }
    /// Scale by a constant.
    pub fn scale(&self, c: i64) -> Poly {
        Self {
            coeffs: self.coeffs.iter().map(|&x| x.saturating_mul(c)).collect(),
        }
        .trim()
    }
    /// Whether this is the zero polynomial.
    pub fn is_zero(&self) -> bool {
        self.trim().coeffs.is_empty()
    }
    /// Leading coefficient.
    pub fn leading_coeff(&self) -> i64 {
        let t = self.trim();
        *t.coeffs.last().unwrap_or(&0)
    }
}
/// A linear inequality constraint.
#[derive(Clone, Debug)]
pub struct LinearConstraint {
    /// Left-hand side.
    pub lhs: LinearExpr,
    /// The comparison operator.
    pub op: ComparisonOp,
    /// Right-hand side.
    pub rhs: LinearExpr,
}
impl LinearConstraint {
    /// Create a constraint `lhs op rhs`.
    pub fn new(lhs: LinearExpr, op: ComparisonOp, rhs: LinearExpr) -> Self {
        Self { lhs, op, rhs }
    }
    /// Move everything to the left: `lhs - rhs op 0`.
    pub fn to_normal_form(&self) -> (LinearExpr, ComparisonOp) {
        (self.lhs.sub(&self.rhs), self.op.clone())
    }
    /// Check whether the constraint is satisfied by a given assignment.
    ///
    /// Note: `ComparisonOp::Ne` is not available, so `Gt` serves as a stand-in.
    pub fn is_satisfied(&self, assignment: &HashMap<String, i64>) -> bool {
        let lval = self.lhs.eval(assignment);
        let rval = self.rhs.eval(assignment);
        match &self.op {
            ComparisonOp::Eq => lval == rval,
            ComparisonOp::Lt => lval < rval,
            ComparisonOp::Le => lval <= rval,
            ComparisonOp::Gt => lval > rval,
            ComparisonOp::Ge => lval >= rval,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum NormNumExtResult2800 {
    /// Operation completed successfully.
    Ok(String),
    /// Operation encountered an error.
    Err(String),
    /// Operation partially completed.
    Partial { done: usize, total: usize },
    /// Operation was skipped.
    Skipped,
}
impl NormNumExtResult2800 {
    #[allow(dead_code)]
    pub fn is_ok(&self) -> bool {
        matches!(self, NormNumExtResult2800::Ok(_))
    }
    #[allow(dead_code)]
    pub fn is_err(&self) -> bool {
        matches!(self, NormNumExtResult2800::Err(_))
    }
    #[allow(dead_code)]
    pub fn is_partial(&self) -> bool {
        matches!(self, NormNumExtResult2800::Partial { .. })
    }
    #[allow(dead_code)]
    pub fn is_skipped(&self) -> bool {
        matches!(self, NormNumExtResult2800::Skipped)
    }
    #[allow(dead_code)]
    pub fn ok_msg(&self) -> Option<&str> {
        if let NormNumExtResult2800::Ok(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn err_msg(&self) -> Option<&str> {
        if let NormNumExtResult2800::Err(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn progress(&self) -> f64 {
        match self {
            NormNumExtResult2800::Ok(_) => 1.0,
            NormNumExtResult2800::Err(_) => 0.0,
            NormNumExtResult2800::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
            NormNumExtResult2800::Skipped => 0.5,
        }
    }
}
#[allow(dead_code)]
pub struct NormNumExtPass2800 {
    pub name: String,
    pub total_runs: usize,
    pub successes: usize,
    pub errors: usize,
    pub enabled: bool,
    pub results: Vec<NormNumExtResult2800>,
}
impl NormNumExtPass2800 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            total_runs: 0,
            successes: 0,
            errors: 0,
            enabled: true,
            results: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn run(&mut self, input: &str) -> NormNumExtResult2800 {
        if !self.enabled {
            return NormNumExtResult2800::Skipped;
        }
        self.total_runs += 1;
        let result = if input.is_empty() {
            self.errors += 1;
            NormNumExtResult2800::Err(format!("empty input in pass '{}'", self.name))
        } else {
            self.successes += 1;
            NormNumExtResult2800::Ok(format!(
                "processed {} chars in pass '{}'",
                input.len(),
                self.name
            ))
        };
        self.results.push(result.clone());
        result
    }
    #[allow(dead_code)]
    pub fn success_count(&self) -> usize {
        self.successes
    }
    #[allow(dead_code)]
    pub fn error_count(&self) -> usize {
        self.errors
    }
    #[allow(dead_code)]
    pub fn success_rate(&self) -> f64 {
        if self.total_runs == 0 {
            0.0
        } else {
            self.successes as f64 / self.total_runs as f64
        }
    }
    #[allow(dead_code)]
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    #[allow(dead_code)]
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    #[allow(dead_code)]
    pub fn clear_results(&mut self) {
        self.results.clear();
    }
}
/// An analysis pass for TacticNormNum.
#[allow(dead_code)]
pub struct TacticNormNumAnalysisPass {
    pub name: String,
    pub enabled: bool,
    pub results: Vec<TacticNormNumResult>,
    pub total_runs: usize,
}
#[allow(dead_code)]
impl TacticNormNumAnalysisPass {
    pub fn new(name: &str) -> Self {
        TacticNormNumAnalysisPass {
            name: name.to_string(),
            enabled: true,
            results: Vec::new(),
            total_runs: 0,
        }
    }
    pub fn run(&mut self, input: &str) -> TacticNormNumResult {
        self.total_runs += 1;
        let result = if input.is_empty() {
            TacticNormNumResult::Err("empty input".to_string())
        } else {
            TacticNormNumResult::Ok(format!("processed: {}", input))
        };
        self.results.push(result.clone());
        result
    }
    pub fn success_count(&self) -> usize {
        self.results.iter().filter(|r| r.is_ok()).count()
    }
    pub fn error_count(&self) -> usize {
        self.results.iter().filter(|r| r.is_err()).count()
    }
    pub fn success_rate(&self) -> f64 {
        if self.total_runs == 0 {
            0.0
        } else {
            self.success_count() as f64 / self.total_runs as f64
        }
    }
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    pub fn clear_results(&mut self) {
        self.results.clear();
    }
}
/// Proof of a numeric equality or inequality
#[derive(Clone, Debug)]
pub struct NumericProof {
    /// The simplified left-hand side
    pub lhs: NumericValue,
    /// The simplified right-hand side
    pub rhs: NumericValue,
    /// The proof term
    pub proof_term: ProofTerm,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum NormNumExtConfigVal2800 {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
impl NormNumExtConfigVal2800 {
    #[allow(dead_code)]
    pub fn as_bool(&self) -> Option<bool> {
        if let NormNumExtConfigVal2800::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_int(&self) -> Option<i64> {
        if let NormNumExtConfigVal2800::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_float(&self) -> Option<f64> {
        if let NormNumExtConfigVal2800::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_str(&self) -> Option<&str> {
        if let NormNumExtConfigVal2800::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_list(&self) -> Option<&[String]> {
        if let NormNumExtConfigVal2800::List(l) = self {
            Some(l)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn type_name(&self) -> &'static str {
        match self {
            NormNumExtConfigVal2800::Bool(_) => "bool",
            NormNumExtConfigVal2800::Int(_) => "int",
            NormNumExtConfigVal2800::Float(_) => "float",
            NormNumExtConfigVal2800::Str(_) => "str",
            NormNumExtConfigVal2800::List(_) => "list",
        }
    }
}
/// Represents a proof term for normalization
#[derive(Clone, Debug)]
pub enum ProofTerm {
    /// Reflexivity: a = a
    Refl,
    /// Transitivity chain
    Trans {
        /// First equality
        eq1: Box<ProofTerm>,
        /// Second equality
        eq2: Box<ProofTerm>,
    },
    /// Congruence of add
    CongAdd {
        /// Left argument proof
        left: Box<ProofTerm>,
        /// Right argument proof
        right: Box<ProofTerm>,
    },
    /// Congruence of mul
    CongMul {
        /// Left argument proof
        left: Box<ProofTerm>,
        /// Right argument proof
        right: Box<ProofTerm>,
    },
    /// Identity: a + 0 = a or 1 * a = a
    Identity {
        /// The side that's the identity
        identity_side: IdentitySide,
    },
    /// Commutivity: a + b = b + a or a * b = b * a
    Comm {
        /// Operation type
        op: CommOp,
    },
    /// Associativity: (a + b) + c = a + (b + c)
    Assoc {
        /// Operation type
        op: AssocOp,
    },
    /// Numeric computation: simplify constants
    Compute {
        /// The computed value
        result: NumericValue,
    },
}
/// Identity side for simplification
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IdentitySide {
    /// Identity on left: 0 + a = a or 1 * a = a
    Left,
    /// Identity on right: a + 0 = a or a * 1 = a
    Right,
}
/// Commutative operations
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CommOp {
    /// Addition
    Add,
    /// Multiplication
    Mul,
}
/// Associative operations
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AssocOp {
    /// Addition
    Add,
    /// Multiplication
    Mul,
}
/// A typed slot for TacticNormNum configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum TacticNormNumConfigValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
#[allow(dead_code)]
impl TacticNormNumConfigValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            TacticNormNumConfigValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            TacticNormNumConfigValue::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            TacticNormNumConfigValue::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            TacticNormNumConfigValue::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&[String]> {
        match self {
            TacticNormNumConfigValue::List(v) => Some(v),
            _ => None,
        }
    }
    pub fn type_name(&self) -> &'static str {
        match self {
            TacticNormNumConfigValue::Bool(_) => "bool",
            TacticNormNumConfigValue::Int(_) => "int",
            TacticNormNumConfigValue::Float(_) => "float",
            TacticNormNumConfigValue::Str(_) => "str",
            TacticNormNumConfigValue::List(_) => "list",
        }
    }
}
/// The canonical normal form of a numeric goal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NumericNormalForm {
    /// A concrete numeric value (fully evaluated).
    Value(NumericValue),
    /// A comparison `lhs op rhs` that could not be fully evaluated.
    Comparison(NumericValue, ComparisonOp, NumericValue),
    /// Could not reduce to a normal form.
    Stuck,
}
impl NumericNormalForm {
    /// Whether the normal form is a concrete value.
    pub fn is_value(&self) -> bool {
        matches!(self, NumericNormalForm::Value(_))
    }
    /// Whether the normal form is stuck.
    pub fn is_stuck(&self) -> bool {
        matches!(self, NumericNormalForm::Stuck)
    }
    /// Extract the value, if any.
    pub fn as_value(&self) -> Option<&NumericValue> {
        match self {
            NumericNormalForm::Value(v) => Some(v),
            _ => None,
        }
    }
    /// Whether a comparison normal form evaluates to true.
    pub fn is_true_comparison(&self) -> Option<bool> {
        match self {
            NumericNormalForm::Comparison(lhs, op, rhs) => Some(op.compare(lhs, rhs)),
            _ => None,
        }
    }
}
/// A pipeline of TacticNormNum analysis passes.
#[allow(dead_code)]
pub struct TacticNormNumPipeline {
    pub passes: Vec<TacticNormNumAnalysisPass>,
    pub name: String,
    pub total_inputs_processed: usize,
}
#[allow(dead_code)]
impl TacticNormNumPipeline {
    pub fn new(name: &str) -> Self {
        TacticNormNumPipeline {
            passes: Vec::new(),
            name: name.to_string(),
            total_inputs_processed: 0,
        }
    }
    pub fn add_pass(&mut self, pass: TacticNormNumAnalysisPass) {
        self.passes.push(pass);
    }
    pub fn run_all(&mut self, input: &str) -> Vec<TacticNormNumResult> {
        self.total_inputs_processed += 1;
        self.passes
            .iter_mut()
            .filter(|p| p.enabled)
            .map(|p| p.run(input))
            .collect()
    }
    pub fn num_passes(&self) -> usize {
        self.passes.len()
    }
    pub fn num_enabled_passes(&self) -> usize {
        self.passes.iter().filter(|p| p.enabled).count()
    }
    pub fn total_success_rate(&self) -> f64 {
        if self.passes.is_empty() {
            0.0
        } else {
            let total_rate: f64 = self.passes.iter().map(|p| p.success_rate()).sum();
            total_rate / self.passes.len() as f64
        }
    }
}
