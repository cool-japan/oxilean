//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;

#[allow(dead_code)]
pub struct PositivityExtPass101 {
    pub name: String,
    pub total_runs: usize,
    pub successes: usize,
    pub errors: usize,
    pub enabled: bool,
    pub results: Vec<PositivityExtResult101>,
}
impl PositivityExtPass101 {
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
    pub fn run(&mut self, input: &str) -> PositivityExtResult101 {
        if !self.enabled {
            return PositivityExtResult101::Skipped;
        }
        self.total_runs += 1;
        let result = if input.is_empty() {
            self.errors += 1;
            PositivityExtResult101::Err(format!("empty input in pass '{}'", self.name))
        } else {
            self.successes += 1;
            PositivityExtResult101::Ok(format!(
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
#[allow(dead_code)]
pub struct PositivityExtDiag101 {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
impl PositivityExtDiag101 {
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
#[allow(dead_code)]
pub struct PositivityExtDiag100 {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
impl PositivityExtDiag100 {
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
#[allow(dead_code)]
pub struct PositivityExtConfig100 {
    pub(super) values: std::collections::HashMap<String, PositivityExtConfigVal100>,
    pub(super) read_only: bool,
    pub(super) name: String,
}
impl PositivityExtConfig100 {
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
    pub fn set(&mut self, key: &str, value: PositivityExtConfigVal100) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&PositivityExtConfigVal100> {
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
        self.set(key, PositivityExtConfigVal100::Bool(v))
    }
    #[allow(dead_code)]
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, PositivityExtConfigVal100::Int(v))
    }
    #[allow(dead_code)]
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, PositivityExtConfigVal100::Str(v.to_string()))
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
/// A result type for TacticPositivity analysis.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TacticPositivityResult {
    Ok(String),
    Err(String),
    Partial { done: usize, total: usize },
    Skipped,
}
#[allow(dead_code)]
impl TacticPositivityResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, TacticPositivityResult::Ok(_))
    }
    pub fn is_err(&self) -> bool {
        matches!(self, TacticPositivityResult::Err(_))
    }
    pub fn is_partial(&self) -> bool {
        matches!(self, TacticPositivityResult::Partial { .. })
    }
    pub fn is_skipped(&self) -> bool {
        matches!(self, TacticPositivityResult::Skipped)
    }
    pub fn ok_msg(&self) -> Option<&str> {
        match self {
            TacticPositivityResult::Ok(s) => Some(s),
            _ => None,
        }
    }
    pub fn err_msg(&self) -> Option<&str> {
        match self {
            TacticPositivityResult::Err(s) => Some(s),
            _ => None,
        }
    }
    pub fn progress(&self) -> f64 {
        match self {
            TacticPositivityResult::Ok(_) => 1.0,
            TacticPositivityResult::Err(_) => 0.0,
            TacticPositivityResult::Skipped => 0.0,
            TacticPositivityResult::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
        }
    }
}
#[allow(dead_code)]
pub struct PositivityExtDiff100 {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
impl PositivityExtDiff100 {
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
/// A sum-of-squares certificate.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SosCertificate {
    pub squares: Vec<PolyF64>,
    pub verified: bool,
}
#[allow(dead_code)]
impl SosCertificate {
    pub fn new(squares: Vec<PolyF64>) -> Self {
        SosCertificate {
            squares,
            verified: false,
        }
    }
    pub fn num_squares(&self) -> usize {
        self.squares.len()
    }
    pub fn verify(&self, poly: &PolyF64, tolerance: f64) -> bool {
        let samples = 50;
        for i in 0..=samples {
            let x = -5.0 + 10.0 * i as f64 / samples as f64;
            let lhs = poly.poly_eval(x);
            let rhs: f64 = self
                .squares
                .iter()
                .map(|p| {
                    let v = p.poly_eval(x);
                    v * v
                })
                .sum();
            if (lhs - rhs).abs() > tolerance {
                return false;
            }
        }
        true
    }
}
/// A diff for TacticPositivity analysis results.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TacticPositivityDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
#[allow(dead_code)]
impl TacticPositivityDiff {
    pub fn new() -> Self {
        TacticPositivityDiff {
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
/// The sign of an arithmetic expression.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Sign {
    /// Strictly positive: `e > 0`.
    Pos,
    /// Non-negative: `e ≥ 0`.
    Nonneg,
    /// Zero: `e = 0`.
    Zero,
    /// Strictly negative: `e < 0`.
    Neg,
    /// Non-positive: `e ≤ 0`.
    Nonpos,
    /// Sign could not be determined.
    Unknown,
}
impl Sign {
    /// Return `true` if the sign implies `e > 0`.
    pub fn is_positive(&self) -> bool {
        matches!(self, Sign::Pos)
    }
    /// Return `true` if the sign implies `e ≥ 0`.
    pub fn is_nonneg(&self) -> bool {
        matches!(self, Sign::Pos | Sign::Nonneg | Sign::Zero)
    }
    /// Return `true` if the sign implies `e ≤ 0`.
    pub fn is_nonpos(&self) -> bool {
        matches!(self, Sign::Neg | Sign::Nonpos | Sign::Zero)
    }
    /// Return `true` if the sign is definitively known.
    pub fn is_known(&self) -> bool {
        !matches!(self, Sign::Unknown)
    }
}
#[allow(dead_code)]
pub struct PositivityExtDiff101 {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
impl PositivityExtDiff101 {
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
/// A typed slot for TacticPositivity configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum TacticPositivityConfigValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
#[allow(dead_code)]
impl TacticPositivityConfigValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            TacticPositivityConfigValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            TacticPositivityConfigValue::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            TacticPositivityConfigValue::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            TacticPositivityConfigValue::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&[String]> {
        match self {
            TacticPositivityConfigValue::List(v) => Some(v),
            _ => None,
        }
    }
    pub fn type_name(&self) -> &'static str {
        match self {
            TacticPositivityConfigValue::Bool(_) => "bool",
            TacticPositivityConfigValue::Int(_) => "int",
            TacticPositivityConfigValue::Float(_) => "float",
            TacticPositivityConfigValue::Str(_) => "str",
            TacticPositivityConfigValue::List(_) => "list",
        }
    }
}
/// A configuration store for TacticPositivity.
#[allow(dead_code)]
pub struct TacticPositivityConfig {
    pub values: std::collections::HashMap<String, TacticPositivityConfigValue>,
    pub read_only: bool,
}
#[allow(dead_code)]
impl TacticPositivityConfig {
    pub fn new() -> Self {
        TacticPositivityConfig {
            values: std::collections::HashMap::new(),
            read_only: false,
        }
    }
    pub fn set(&mut self, key: &str, value: TacticPositivityConfigValue) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    pub fn get(&self, key: &str) -> Option<&TacticPositivityConfigValue> {
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
        self.set(key, TacticPositivityConfigValue::Bool(v))
    }
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, TacticPositivityConfigValue::Int(v))
    }
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, TacticPositivityConfigValue::Str(v.to_string()))
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
#[derive(Debug, Clone)]
pub enum PositivityExtResult101 {
    /// Operation completed successfully.
    Ok(String),
    /// Operation encountered an error.
    Err(String),
    /// Operation partially completed.
    Partial { done: usize, total: usize },
    /// Operation was skipped.
    Skipped,
}
impl PositivityExtResult101 {
    #[allow(dead_code)]
    pub fn is_ok(&self) -> bool {
        matches!(self, PositivityExtResult101::Ok(_))
    }
    #[allow(dead_code)]
    pub fn is_err(&self) -> bool {
        matches!(self, PositivityExtResult101::Err(_))
    }
    #[allow(dead_code)]
    pub fn is_partial(&self) -> bool {
        matches!(self, PositivityExtResult101::Partial { .. })
    }
    #[allow(dead_code)]
    pub fn is_skipped(&self) -> bool {
        matches!(self, PositivityExtResult101::Skipped)
    }
    #[allow(dead_code)]
    pub fn ok_msg(&self) -> Option<&str> {
        if let PositivityExtResult101::Ok(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn err_msg(&self) -> Option<&str> {
        if let PositivityExtResult101::Err(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn progress(&self) -> f64 {
        match self {
            PositivityExtResult101::Ok(_) => 1.0,
            PositivityExtResult101::Err(_) => 0.0,
            PositivityExtResult101::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
            PositivityExtResult101::Skipped => 0.5,
        }
    }
}
/// A tree structure for non-negativity proofs.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum NnfTree {
    Atom {
        expr: String,
        reason: String,
    },
    NnfSum {
        children: Vec<NnfTree>,
    },
    NnfProduct {
        children: Vec<NnfTree>,
    },
    NnfSquare {
        child: Box<NnfTree>,
    },
    NnfComposition {
        outer: Box<NnfTree>,
        inner: Box<NnfTree>,
    },
    NnfConstant(f64),
}
#[allow(dead_code)]
impl NnfTree {
    pub fn atom(expr: &str, reason: &str) -> Self {
        NnfTree::Atom {
            expr: expr.to_string(),
            reason: reason.to_string(),
        }
    }
    pub fn nnf_const(c: f64) -> Self {
        NnfTree::NnfConstant(c)
    }
    pub fn nnf_square(child: NnfTree) -> Self {
        NnfTree::NnfSquare {
            child: Box::new(child),
        }
    }
    pub fn nnf_depth(&self) -> usize {
        match self {
            NnfTree::Atom { .. } | NnfTree::NnfConstant(_) => 0,
            NnfTree::NnfSquare { child } => 1 + child.nnf_depth(),
            NnfTree::NnfComposition { outer, inner } => {
                1 + outer.nnf_depth().max(inner.nnf_depth())
            }
            NnfTree::NnfSum { children } | NnfTree::NnfProduct { children } => {
                1 + children.iter().map(|c| c.nnf_depth()).max().unwrap_or(0)
            }
        }
    }
    pub fn num_leaves(&self) -> usize {
        match self {
            NnfTree::Atom { .. } | NnfTree::NnfConstant(_) => 1,
            NnfTree::NnfSquare { child } => child.num_leaves(),
            NnfTree::NnfComposition { outer, inner } => outer.num_leaves() + inner.num_leaves(),
            NnfTree::NnfSum { children } | NnfTree::NnfProduct { children } => {
                children.iter().map(|c| c.num_leaves()).sum()
            }
        }
    }
}
#[allow(dead_code)]
pub struct PositivityExtPipeline101 {
    pub name: String,
    pub passes: Vec<PositivityExtPass101>,
    pub run_count: usize,
}
impl PositivityExtPipeline101 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            passes: Vec::new(),
            run_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_pass(&mut self, pass: PositivityExtPass101) {
        self.passes.push(pass);
    }
    #[allow(dead_code)]
    pub fn run_all(&mut self, input: &str) -> Vec<PositivityExtResult101> {
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
/// A diagnostic reporter for TacticPositivity.
#[allow(dead_code)]
pub struct TacticPositivityDiagnostics {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
#[allow(dead_code)]
impl TacticPositivityDiagnostics {
    pub fn new(max_errors: usize) -> Self {
        TacticPositivityDiagnostics {
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
/// A proof step in the positivity tactic.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum PosProofStep {
    /// Use a known hypothesis.
    FromHyp(String),
    /// The expression is a positive literal.
    PosLit(i64),
    /// The expression is a non-negative literal.
    NonnegLit(i64),
    /// Sum of positive terms.
    AddPos(Box<PosProofStep>, Box<PosProofStep>),
    /// Product of positive terms.
    MulPos(Box<PosProofStep>, Box<PosProofStep>),
    /// Square is non-negative.
    Squared,
    /// Abs is non-negative.
    AbsNonneg,
    /// Exp is positive.
    ExpPos,
    /// Unknown/sorry.
    Sorry,
}
#[allow(dead_code)]
impl PosProofStep {
    pub fn depth(&self) -> usize {
        match self {
            PosProofStep::FromHyp(_)
            | PosProofStep::PosLit(_)
            | PosProofStep::NonnegLit(_)
            | PosProofStep::Squared
            | PosProofStep::AbsNonneg
            | PosProofStep::ExpPos
            | PosProofStep::Sorry => 0,
            PosProofStep::AddPos(a, b) | PosProofStep::MulPos(a, b) => 1 + a.depth().max(b.depth()),
        }
    }
    pub fn size(&self) -> usize {
        match self {
            PosProofStep::AddPos(a, b) | PosProofStep::MulPos(a, b) => 1 + a.size() + b.size(),
            _ => 1,
        }
    }
    pub fn is_sorry(&self) -> bool {
        matches!(self, PosProofStep::Sorry)
    }
}
/// Polynomial arithmetic for positivity: verify sum-of-squares decompositions.
#[derive(Clone, Debug)]
pub struct PolyrithTactic {
    pub(super) max_degree: usize,
}
impl PolyrithTactic {
    /// Create a new `PolyrithTactic`.
    pub fn new() -> Self {
        PolyrithTactic { max_degree: 4 }
    }
    /// Verify whether `expr` is syntactically a sum of squares.
    ///
    /// Checks for patterns like `a^2`, `a^2 + b^2`, etc.
    pub fn verify_sum_of_squares(&self, expr: &str) -> bool {
        let trimmed = expr.trim();
        if is_square_term(trimmed) {
            return true;
        }
        if let Some(pos) = find_op_at_depth0(trimmed, '+') {
            let l = trimmed[..pos].trim();
            let r = trimmed[pos + 1..].trim();
            return is_square_term(l) && self.verify_sum_of_squares(r);
        }
        false
    }
}
/// A real-valued interval [lo, hi] (using f64 for simplicity).
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct SignInterval {
    pub lo: f64,
    pub hi: f64,
}
#[allow(dead_code)]
impl SignInterval {
    pub fn new(lo: f64, hi: f64) -> Self {
        SignInterval { lo, hi }
    }
    pub fn point(v: f64) -> Self {
        SignInterval { lo: v, hi: v }
    }
    pub fn pos() -> Self {
        SignInterval {
            lo: f64::EPSILON,
            hi: f64::INFINITY,
        }
    }
    pub fn nonneg() -> Self {
        SignInterval {
            lo: 0.0,
            hi: f64::INFINITY,
        }
    }
    pub fn neg() -> Self {
        SignInterval {
            lo: f64::NEG_INFINITY,
            hi: -f64::EPSILON,
        }
    }
    pub fn nonpos() -> Self {
        SignInterval {
            lo: f64::NEG_INFINITY,
            hi: 0.0,
        }
    }
    pub fn zero() -> Self {
        SignInterval { lo: 0.0, hi: 0.0 }
    }
    pub fn top() -> Self {
        SignInterval {
            lo: f64::NEG_INFINITY,
            hi: f64::INFINITY,
        }
    }
    pub fn contains_zero(&self) -> bool {
        self.lo <= 0.0 && self.hi >= 0.0
    }
    pub fn is_pos(&self) -> bool {
        self.lo > 0.0
    }
    pub fn is_nonneg(&self) -> bool {
        self.lo >= 0.0
    }
    pub fn is_neg(&self) -> bool {
        self.hi < 0.0
    }
    pub fn is_nonpos(&self) -> bool {
        self.hi <= 0.0
    }
    pub fn is_zero(&self) -> bool {
        self.lo == 0.0 && self.hi == 0.0
    }
    pub fn sign(&self) -> Sign {
        if self.is_pos() {
            Sign::Pos
        } else if self.is_neg() {
            Sign::Neg
        } else if self.is_zero() {
            Sign::Zero
        } else if self.is_nonneg() {
            Sign::Nonneg
        } else if self.is_nonpos() {
            Sign::Nonpos
        } else {
            Sign::Unknown
        }
    }
    pub fn add(&self, other: &Self) -> Self {
        SignInterval::new(self.lo + other.lo, self.hi + other.hi)
    }
    pub fn mul(&self, other: &Self) -> Self {
        let products = [
            self.lo * other.lo,
            self.lo * other.hi,
            self.hi * other.lo,
            self.hi * other.hi,
        ];
        let lo = products.iter().cloned().fold(f64::INFINITY, f64::min);
        let hi = products.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        SignInterval::new(lo, hi)
    }
    pub fn neg_interval(&self) -> Self {
        SignInterval::new(-self.hi, -self.lo)
    }
    pub fn abs_interval(&self) -> Self {
        if self.lo >= 0.0 {
            *self
        } else if self.hi <= 0.0 {
            self.neg_interval()
        } else {
            SignInterval::new(0.0, self.lo.abs().max(self.hi))
        }
    }
    pub fn meet(&self, other: &Self) -> Self {
        SignInterval::new(self.lo.max(other.lo), self.hi.min(other.hi))
    }
    pub fn join(&self, other: &Self) -> Self {
        SignInterval::new(self.lo.min(other.lo), self.hi.max(other.hi))
    }
    pub fn width(&self) -> f64 {
        self.hi - self.lo
    }
    pub fn midpoint(&self) -> f64 {
        (self.lo + self.hi) / 2.0
    }
}
#[allow(dead_code)]
pub struct PositivityExtConfig101 {
    pub(super) values: std::collections::HashMap<String, PositivityExtConfigVal101>,
    pub(super) read_only: bool,
    pub(super) name: String,
}
impl PositivityExtConfig101 {
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
    pub fn set(&mut self, key: &str, value: PositivityExtConfigVal101) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&PositivityExtConfigVal101> {
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
        self.set(key, PositivityExtConfigVal101::Bool(v))
    }
    #[allow(dead_code)]
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, PositivityExtConfigVal101::Int(v))
    }
    #[allow(dead_code)]
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, PositivityExtConfigVal101::Str(v.to_string()))
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
/// A pipeline of TacticPositivity analysis passes.
#[allow(dead_code)]
pub struct TacticPositivityPipeline {
    pub passes: Vec<TacticPositivityAnalysisPass>,
    pub name: String,
    pub total_inputs_processed: usize,
}
#[allow(dead_code)]
impl TacticPositivityPipeline {
    pub fn new(name: &str) -> Self {
        TacticPositivityPipeline {
            passes: Vec::new(),
            name: name.to_string(),
            total_inputs_processed: 0,
        }
    }
    pub fn add_pass(&mut self, pass: TacticPositivityAnalysisPass) {
        self.passes.push(pass);
    }
    pub fn run_all(&mut self, input: &str) -> Vec<TacticPositivityResult> {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum PositivityExtConfigVal101 {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
impl PositivityExtConfigVal101 {
    #[allow(dead_code)]
    pub fn as_bool(&self) -> Option<bool> {
        if let PositivityExtConfigVal101::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_int(&self) -> Option<i64> {
        if let PositivityExtConfigVal101::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_float(&self) -> Option<f64> {
        if let PositivityExtConfigVal101::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_str(&self) -> Option<&str> {
        if let PositivityExtConfigVal101::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_list(&self) -> Option<&[String]> {
        if let PositivityExtConfigVal101::List(l) = self {
            Some(l)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn type_name(&self) -> &'static str {
        match self {
            PositivityExtConfigVal101::Bool(_) => "bool",
            PositivityExtConfigVal101::Int(_) => "int",
            PositivityExtConfigVal101::Float(_) => "float",
            PositivityExtConfigVal101::Str(_) => "str",
            PositivityExtConfigVal101::List(_) => "list",
        }
    }
}
/// The positivity tactic: prove expressions are positive or non-negative.
#[derive(Clone, Debug)]
pub struct PositivityTactic {
    pub(super) config: PositivityConfig,
}
impl PositivityTactic {
    /// Create a new `PositivityTactic` with default configuration.
    pub fn new() -> Self {
        PositivityTactic {
            config: PositivityConfig::default(),
        }
    }
    /// Create a new `PositivityTactic` that only proves strict positivity.
    pub fn with_strict(strict: bool) -> Self {
        PositivityTactic {
            config: PositivityConfig {
                strict,
                ..Default::default()
            },
        }
    }
    /// Determine the sign of a natural number literal.
    pub fn sign_of_nat(n: u64) -> Sign {
        if n == 0 {
            Sign::Zero
        } else {
            Sign::Pos
        }
    }
    /// Determine the sign of a sum given the signs of the summands.
    pub fn sign_of_sum(s1: Sign, s2: Sign) -> Sign {
        match (s1, s2) {
            (Sign::Pos, Sign::Pos) => Sign::Pos,
            (Sign::Pos, Sign::Nonneg) | (Sign::Nonneg, Sign::Pos) => Sign::Pos,
            (Sign::Pos, Sign::Zero) | (Sign::Zero, Sign::Pos) => Sign::Pos,
            (Sign::Nonneg, Sign::Nonneg) => Sign::Nonneg,
            (Sign::Nonneg, Sign::Zero) | (Sign::Zero, Sign::Nonneg) => Sign::Nonneg,
            (Sign::Zero, Sign::Zero) => Sign::Zero,
            (Sign::Neg, Sign::Neg) => Sign::Neg,
            (Sign::Neg, Sign::Nonpos) | (Sign::Nonpos, Sign::Neg) => Sign::Neg,
            (Sign::Neg, Sign::Zero) | (Sign::Zero, Sign::Neg) => Sign::Neg,
            (Sign::Nonpos, Sign::Nonpos) => Sign::Nonpos,
            (Sign::Nonpos, Sign::Zero) | (Sign::Zero, Sign::Nonpos) => Sign::Nonpos,
            _ => Sign::Unknown,
        }
    }
    /// Determine the sign of a product given the signs of the factors.
    pub fn sign_of_product(s1: Sign, s2: Sign) -> Sign {
        match (s1, s2) {
            (Sign::Zero, _) | (_, Sign::Zero) => Sign::Zero,
            (Sign::Pos, Sign::Pos) => Sign::Pos,
            (Sign::Neg, Sign::Neg) => Sign::Pos,
            (Sign::Pos, Sign::Neg) | (Sign::Neg, Sign::Pos) => Sign::Neg,
            (Sign::Nonneg, Sign::Nonneg) => Sign::Nonneg,
            (Sign::Nonneg, Sign::Pos) | (Sign::Pos, Sign::Nonneg) => Sign::Nonneg,
            (Sign::Nonpos, Sign::Nonpos) => Sign::Nonneg,
            (Sign::Neg, Sign::Nonneg) | (Sign::Nonneg, Sign::Neg) => Sign::Nonpos,
            _ => Sign::Unknown,
        }
    }
    /// Determine the sign of a power expression `base ^ exp`.
    pub fn sign_of_power(base: Sign, exp: u64) -> Sign {
        if exp == 0 {
            return Sign::Pos;
        }
        match base {
            Sign::Pos => Sign::Pos,
            Sign::Zero => Sign::Zero,
            Sign::Neg => {
                if exp % 2 == 0 {
                    Sign::Pos
                } else {
                    Sign::Neg
                }
            }
            Sign::Nonneg => Sign::Nonneg,
            Sign::Nonpos => {
                if exp % 2 == 0 {
                    Sign::Nonneg
                } else {
                    Sign::Nonpos
                }
            }
            Sign::Unknown => Sign::Unknown,
        }
    }
    /// Determine the sign of an absolute value expression.
    ///
    /// `|e|` is always non-negative, and positive unless `e = 0`.
    pub fn sign_of_abs(s: Sign) -> Sign {
        match s {
            Sign::Zero => Sign::Zero,
            Sign::Unknown => Sign::Nonneg,
            _ => Sign::Nonneg,
        }
    }
    /// Analyze the sign of an expression given as a string.
    ///
    /// Handles numeric literals, `abs(...)`, `sq(...)`, and arithmetic.
    pub fn analyze_expr(&self, expr: &str) -> Sign {
        let trimmed = expr.trim();
        if let Ok(n) = trimmed.parse::<u64>() {
            return Self::sign_of_nat(n);
        }
        if let Ok(n) = trimmed.parse::<i64>() {
            if n > 0 {
                return Sign::Pos;
            } else if n == 0 {
                return Sign::Zero;
            } else {
                return Sign::Neg;
            }
        }
        if let Some(inner) = trimmed
            .strip_prefix("abs(")
            .and_then(|s| s.strip_suffix(')'))
        {
            let inner_sign = self.analyze_expr(inner);
            return Self::sign_of_abs(inner_sign);
        }
        if let Some(inner) = trimmed
            .strip_prefix("sq(")
            .and_then(|s| s.strip_suffix(')'))
        {
            let inner_sign = self.analyze_expr(inner);
            return Self::sign_of_power(inner_sign, 2);
        }
        if let Some(base_str) = trimmed.strip_suffix("^2") {
            let base_sign = self.analyze_expr(base_str.trim());
            return Self::sign_of_power(base_sign, 2);
        }
        if let Some(pos) = find_op_at_depth0(trimmed, '+') {
            let l = self.analyze_expr(&trimmed[..pos]);
            let r = self.analyze_expr(&trimmed[pos + 1..]);
            return Self::sign_of_sum(l, r);
        }
        if let Some(pos) = find_op_at_depth0(trimmed, '*') {
            let l = self.analyze_expr(&trimmed[..pos]);
            let r = self.analyze_expr(&trimmed[pos + 1..]);
            return Self::sign_of_product(l, r);
        }
        Sign::Unknown
    }
    /// Return `true` if the tactic can prove `0 < expr`.
    pub fn can_prove_pos(&self, expr: &str) -> bool {
        self.analyze_expr(expr).is_positive()
    }
    /// Return `true` if the tactic can prove `0 ≤ expr`.
    pub fn can_prove_nonneg(&self, expr: &str) -> bool {
        self.analyze_expr(expr).is_nonneg()
    }
}
#[allow(dead_code)]
pub struct PositivityExtPipeline100 {
    pub name: String,
    pub passes: Vec<PositivityExtPass100>,
    pub run_count: usize,
}
impl PositivityExtPipeline100 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            passes: Vec::new(),
            run_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_pass(&mut self, pass: PositivityExtPass100) {
        self.passes.push(pass);
    }
    #[allow(dead_code)]
    pub fn run_all(&mut self, input: &str) -> Vec<PositivityExtResult100> {
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
#[allow(dead_code)]
pub struct PositivityExtPass100 {
    pub name: String,
    pub total_runs: usize,
    pub successes: usize,
    pub errors: usize,
    pub enabled: bool,
    pub results: Vec<PositivityExtResult100>,
}
impl PositivityExtPass100 {
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
    pub fn run(&mut self, input: &str) -> PositivityExtResult100 {
        if !self.enabled {
            return PositivityExtResult100::Skipped;
        }
        self.total_runs += 1;
        let result = if input.is_empty() {
            self.errors += 1;
            PositivityExtResult100::Err(format!("empty input in pass '{}'", self.name))
        } else {
            self.successes += 1;
            PositivityExtResult100::Ok(format!(
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
/// A univariate polynomial with f64 coefficients.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PolyF64 {
    pub coeffs: Vec<f64>,
}
#[allow(dead_code)]
impl PolyF64 {
    pub fn new(coeffs: Vec<f64>) -> Self {
        PolyF64 { coeffs }
    }
    pub fn poly_constant(c: f64) -> Self {
        PolyF64 { coeffs: vec![c] }
    }
    pub fn poly_degree(&self) -> usize {
        let mut d = self.coeffs.len();
        while d > 1 && self.coeffs[d - 1].abs() < 1e-15 {
            d -= 1;
        }
        d - 1
    }
    pub fn poly_eval(&self, x: f64) -> f64 {
        let mut result = 0.0;
        let mut xpow = 1.0;
        for &c in &self.coeffs {
            result += c * xpow;
            xpow *= x;
        }
        result
    }
    pub fn is_nonneg_on_reals(&self) -> Option<bool> {
        let d = self.poly_degree();
        if d == 0 {
            return Some(self.coeffs[0] >= 0.0);
        }
        if d % 2 == 1 {
            return Some(false);
        }
        if self.coeffs[d] < 0.0 {
            return Some(false);
        }
        let samples = 1000;
        for i in 0..=samples {
            let x = -10.0 + 20.0 * i as f64 / samples as f64;
            if self.poly_eval(x) < -1e-10 {
                return Some(false);
            }
        }
        Some(true)
    }
    pub fn poly_add(&self, other: &Self) -> Self {
        let n = self.coeffs.len().max(other.coeffs.len());
        let mut result = vec![0.0; n];
        for (i, &c) in self.coeffs.iter().enumerate() {
            result[i] += c;
        }
        for (i, &c) in other.coeffs.iter().enumerate() {
            result[i] += c;
        }
        PolyF64 { coeffs: result }
    }
    pub fn poly_mul_scalar(&self, s: f64) -> Self {
        PolyF64 {
            coeffs: self.coeffs.iter().map(|&c| c * s).collect(),
        }
    }
    pub fn leading_coeff(&self) -> f64 {
        let d = self.poly_degree();
        self.coeffs[d]
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum PositivityExtResult100 {
    /// Operation completed successfully.
    Ok(String),
    /// Operation encountered an error.
    Err(String),
    /// Operation partially completed.
    Partial { done: usize, total: usize },
    /// Operation was skipped.
    Skipped,
}
impl PositivityExtResult100 {
    #[allow(dead_code)]
    pub fn is_ok(&self) -> bool {
        matches!(self, PositivityExtResult100::Ok(_))
    }
    #[allow(dead_code)]
    pub fn is_err(&self) -> bool {
        matches!(self, PositivityExtResult100::Err(_))
    }
    #[allow(dead_code)]
    pub fn is_partial(&self) -> bool {
        matches!(self, PositivityExtResult100::Partial { .. })
    }
    #[allow(dead_code)]
    pub fn is_skipped(&self) -> bool {
        matches!(self, PositivityExtResult100::Skipped)
    }
    #[allow(dead_code)]
    pub fn ok_msg(&self) -> Option<&str> {
        if let PositivityExtResult100::Ok(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn err_msg(&self) -> Option<&str> {
        if let PositivityExtResult100::Err(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn progress(&self) -> f64 {
        match self {
            PositivityExtResult100::Ok(_) => 1.0,
            PositivityExtResult100::Err(_) => 0.0,
            PositivityExtResult100::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
            PositivityExtResult100::Skipped => 0.5,
        }
    }
}
/// Configuration for the positivity tactic.
#[derive(Clone, Debug)]
pub struct PositivityConfig {
    /// If `true`, the tactic only succeeds for strict positivity (`> 0`).
    pub strict: bool,
    /// Maximum recursion depth for sign analysis.
    pub max_depth: usize,
}
/// An analysis pass for TacticPositivity.
#[allow(dead_code)]
pub struct TacticPositivityAnalysisPass {
    pub name: String,
    pub enabled: bool,
    pub results: Vec<TacticPositivityResult>,
    pub total_runs: usize,
}
#[allow(dead_code)]
impl TacticPositivityAnalysisPass {
    pub fn new(name: &str) -> Self {
        TacticPositivityAnalysisPass {
            name: name.to_string(),
            enabled: true,
            results: Vec::new(),
            total_runs: 0,
        }
    }
    pub fn run(&mut self, input: &str) -> TacticPositivityResult {
        self.total_runs += 1;
        let result = if input.is_empty() {
            TacticPositivityResult::Err("empty input".to_string())
        } else {
            TacticPositivityResult::Ok(format!("processed: {}", input))
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum PositivityExtConfigVal100 {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
impl PositivityExtConfigVal100 {
    #[allow(dead_code)]
    pub fn as_bool(&self) -> Option<bool> {
        if let PositivityExtConfigVal100::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_int(&self) -> Option<i64> {
        if let PositivityExtConfigVal100::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_float(&self) -> Option<f64> {
        if let PositivityExtConfigVal100::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_str(&self) -> Option<&str> {
        if let PositivityExtConfigVal100::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_list(&self) -> Option<&[String]> {
        if let PositivityExtConfigVal100::List(l) = self {
            Some(l)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn type_name(&self) -> &'static str {
        match self {
            PositivityExtConfigVal100::Bool(_) => "bool",
            PositivityExtConfigVal100::Int(_) => "int",
            PositivityExtConfigVal100::Float(_) => "float",
            PositivityExtConfigVal100::Str(_) => "str",
            PositivityExtConfigVal100::List(_) => "list",
        }
    }
}
/// A Positivstellensatz certificate for polynomial inequality.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PosstellensatzCert {
    pub polynomial: String,
    pub inequalities: Vec<String>,
    pub certificate_type: String,
}
#[allow(dead_code)]
impl PosstellensatzCert {
    pub fn new(poly: &str, cert_type: &str) -> Self {
        PosstellensatzCert {
            polynomial: poly.to_string(),
            inequalities: Vec::new(),
            certificate_type: cert_type.to_string(),
        }
    }
    pub fn add_inequality(&mut self, ineq: &str) {
        self.inequalities.push(ineq.to_string());
    }
    pub fn is_sos_cert(&self) -> bool {
        self.certificate_type == "sos"
    }
    pub fn is_farkas_cert(&self) -> bool {
        self.certificate_type == "farkas"
    }
}
/// A system for verifying positivity of complex expressions.
#[allow(dead_code)]
pub struct PositivitySystem {
    pub checker: PositivityCheckerExt,
    pub sos_attempts: usize,
    pub direct_proofs: usize,
    pub failed_checks: usize,
}
#[allow(dead_code)]
impl PositivitySystem {
    pub fn new() -> Self {
        PositivitySystem {
            checker: PositivityCheckerExt::new(),
            sos_attempts: 0,
            direct_proofs: 0,
            failed_checks: 0,
        }
    }
    pub fn prove_nonneg_poly(&mut self, poly: &PolyF64) -> Option<SosCertificate> {
        self.sos_attempts += 1;
        let cert = find_sos_decomposition(poly);
        if cert.is_some() {
            self.direct_proofs += 1;
        } else {
            self.failed_checks += 1;
        }
        cert
    }
    pub fn total_checks(&self) -> usize {
        self.sos_attempts
    }
    pub fn success_rate(&self) -> f64 {
        if self.sos_attempts == 0 {
            0.0
        } else {
            self.direct_proofs as f64 / self.sos_attempts as f64
        }
    }
}
/// A mapping from variable names to their known sign intervals.
#[allow(dead_code)]
pub struct SignContext {
    pub bindings: std::collections::HashMap<String, SignInterval>,
}
#[allow(dead_code)]
impl SignContext {
    pub fn new() -> Self {
        SignContext {
            bindings: std::collections::HashMap::new(),
        }
    }
    pub fn bind(&mut self, name: &str, interval: SignInterval) {
        self.bindings.insert(name.to_string(), interval);
    }
    pub fn lookup(&self, name: &str) -> Option<SignInterval> {
        self.bindings.get(name).copied()
    }
    pub fn known_pos(&mut self, name: &str) {
        self.bind(name, SignInterval::pos());
    }
    pub fn known_nonneg(&mut self, name: &str) {
        self.bind(name, SignInterval::nonneg());
    }
    pub fn known_neg(&mut self, name: &str) {
        self.bind(name, SignInterval::neg());
    }
    pub fn known_zero(&mut self, name: &str) {
        self.bind(name, SignInterval::zero());
    }
    pub fn size(&self) -> usize {
        self.bindings.len()
    }
}
/// Extended positivity checker with sign context.
#[allow(dead_code)]
pub struct PositivityCheckerExt {
    pub ctx: SignContext,
    pub verbose: bool,
}
#[allow(dead_code)]
impl PositivityCheckerExt {
    pub fn new() -> Self {
        PositivityCheckerExt {
            ctx: SignContext::new(),
            verbose: false,
        }
    }
    pub fn with_context(ctx: SignContext) -> Self {
        PositivityCheckerExt {
            ctx,
            verbose: false,
        }
    }
    pub fn check_pos(&self, name: &str) -> Option<Sign> {
        self.ctx.lookup(name).map(|iv| iv.sign())
    }
    pub fn check_sum_pos(&self, names: &[&str]) -> Sign {
        names.iter().fold(Sign::Zero, |acc, name| {
            let s = self
                .ctx
                .lookup(name)
                .map(|iv| iv.sign())
                .unwrap_or(Sign::Unknown);
            sign_of_sum(&acc, &s)
        })
    }
    pub fn check_product_pos(&self, names: &[&str]) -> Sign {
        names.iter().fold(Sign::Pos, |acc, name| {
            let s = self
                .ctx
                .lookup(name)
                .map(|iv| iv.sign())
                .unwrap_or(Sign::Unknown);
            sign_of_product(&acc, &s)
        })
    }
}
/// Sign information for arithmetic expressions.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum SignInfo {
    Positive,
    NonNegative,
    Negative,
    NonPositive,
    SignZero,
    SignUnknown,
}
#[allow(dead_code)]
impl SignInfo {
    pub fn is_nonneg(&self) -> bool {
        matches!(
            self,
            SignInfo::Positive | SignInfo::NonNegative | SignInfo::SignZero
        )
    }
    pub fn is_positive(&self) -> bool {
        matches!(self, SignInfo::Positive)
    }
    pub fn sign_negate(&self) -> Self {
        match self {
            SignInfo::Positive => SignInfo::Negative,
            SignInfo::Negative => SignInfo::Positive,
            SignInfo::NonNegative => SignInfo::NonPositive,
            SignInfo::NonPositive => SignInfo::NonNegative,
            SignInfo::SignZero => SignInfo::SignZero,
            SignInfo::SignUnknown => SignInfo::SignUnknown,
        }
    }
    pub fn sign_add(&self, other: &Self) -> Self {
        match (self, other) {
            (SignInfo::Positive, SignInfo::Positive) => SignInfo::Positive,
            (SignInfo::Positive, SignInfo::NonNegative) => SignInfo::Positive,
            (SignInfo::NonNegative, SignInfo::Positive) => SignInfo::Positive,
            (SignInfo::NonNegative, SignInfo::NonNegative) => SignInfo::NonNegative,
            (SignInfo::Negative, SignInfo::Negative) => SignInfo::Negative,
            (SignInfo::SignZero, x) | (x, SignInfo::SignZero) => x.clone(),
            _ => SignInfo::SignUnknown,
        }
    }
    pub fn sign_mul(&self, other: &Self) -> Self {
        match (self, other) {
            (SignInfo::Positive, SignInfo::Positive) => SignInfo::Positive,
            (SignInfo::Negative, SignInfo::Negative) => SignInfo::Positive,
            (SignInfo::Positive, SignInfo::Negative) => SignInfo::Negative,
            (SignInfo::Negative, SignInfo::Positive) => SignInfo::Negative,
            (SignInfo::SignZero, _) | (_, SignInfo::SignZero) => SignInfo::SignZero,
            (SignInfo::NonNegative, SignInfo::NonNegative) => SignInfo::NonNegative,
            _ => SignInfo::SignUnknown,
        }
    }
}
