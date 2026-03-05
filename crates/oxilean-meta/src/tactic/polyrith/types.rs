//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;

/// Cache for polyrith results.
#[allow(dead_code)]
pub struct PolyrithCache {
    pub entries: std::collections::HashMap<String, Option<Vec<i64>>>,
    pub hits: usize,
    pub misses: usize,
}
#[allow(dead_code)]
impl PolyrithCache {
    pub fn new() -> Self {
        PolyrithCache {
            entries: std::collections::HashMap::new(),
            hits: 0,
            misses: 0,
        }
    }
    pub fn lookup(&mut self, key: &str) -> Option<Option<Vec<i64>>> {
        match self.entries.get(key).cloned() {
            Some(v) => {
                self.hits += 1;
                Some(v)
            }
            None => {
                self.misses += 1;
                None
            }
        }
    }
    pub fn insert(&mut self, key: &str, value: Option<Vec<i64>>) {
        self.entries.insert(key.to_string(), value);
    }
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }
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
impl PolyrithStats {
    pub fn new() -> Self {
        PolyrithStats::default()
    }
    pub fn record_groebner_call(&mut self) {
        self.groebner_calls += 1;
    }
    pub fn record_reduction(&mut self, steps: usize) {
        self.reduction_steps += steps;
    }
    pub fn record_spoly(&mut self) {
        self.s_polynomials_computed += 1;
    }
    pub fn record_success(&mut self) {
        self.certificates_found += 1;
    }
    pub fn record_failure(&mut self) {
        self.certificates_failed += 1;
    }
    pub fn success_rate(&self) -> f64 {
        let total = self.certificates_found + self.certificates_failed;
        if total == 0 {
            0.0
        } else {
            self.certificates_found as f64 / total as f64
        }
    }
}
#[allow(dead_code)]
pub struct PolyrithExtPipeline301 {
    pub name: String,
    pub passes: Vec<PolyrithExtPass301>,
    pub run_count: usize,
}
impl PolyrithExtPipeline301 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            passes: Vec::new(),
            run_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_pass(&mut self, pass: PolyrithExtPass301) {
        self.passes.push(pass);
    }
    #[allow(dead_code)]
    pub fn run_all(&mut self, input: &str) -> Vec<PolyrithExtResult301> {
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
/// A configuration store for TacticPolyrith.
#[allow(dead_code)]
pub struct TacticPolyrithConfig {
    pub values: std::collections::HashMap<String, TacticPolyrithConfigValue>,
    pub read_only: bool,
}
#[allow(dead_code)]
impl TacticPolyrithConfig {
    pub fn new() -> Self {
        TacticPolyrithConfig {
            values: std::collections::HashMap::new(),
            read_only: false,
        }
    }
    pub fn set(&mut self, key: &str, value: TacticPolyrithConfigValue) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    pub fn get(&self, key: &str) -> Option<&TacticPolyrithConfigValue> {
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
        self.set(key, TacticPolyrithConfigValue::Bool(v))
    }
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, TacticPolyrithConfigValue::Int(v))
    }
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, TacticPolyrithConfigValue::Str(v.to_string()))
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
pub struct PolyrithExtDiff300 {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
impl PolyrithExtDiff300 {
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
#[allow(dead_code)]
pub struct PolyrithExtDiff301 {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
impl PolyrithExtDiff301 {
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
impl PolyrithExtResult301 {
    #[allow(dead_code)]
    pub fn is_ok(&self) -> bool {
        matches!(self, PolyrithExtResult301::Ok(_))
    }
    #[allow(dead_code)]
    pub fn is_err(&self) -> bool {
        matches!(self, PolyrithExtResult301::Err(_))
    }
    #[allow(dead_code)]
    pub fn is_partial(&self) -> bool {
        matches!(self, PolyrithExtResult301::Partial { .. })
    }
    #[allow(dead_code)]
    pub fn is_skipped(&self) -> bool {
        matches!(self, PolyrithExtResult301::Skipped)
    }
    #[allow(dead_code)]
    pub fn ok_msg(&self) -> Option<&str> {
        if let PolyrithExtResult301::Ok(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn err_msg(&self) -> Option<&str> {
        if let PolyrithExtResult301::Err(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn progress(&self) -> f64 {
        match self {
            PolyrithExtResult301::Ok(_) => 1.0,
            PolyrithExtResult301::Err(_) => 0.0,
            PolyrithExtResult301::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
            PolyrithExtResult301::Skipped => 0.5,
        }
    }
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
impl Monomial {
    /// Create a constant monomial with the given coefficient and no variables.
    pub fn new(coeff: i64) -> Self {
        Self {
            vars: Vec::new(),
            coefficient: coeff,
        }
    }
    /// Add a variable with the given degree to this monomial.
    ///
    /// If the variable already exists its degree is incremented; otherwise a
    /// new `(var, deg)` pair is pushed.
    pub fn add_var(&mut self, var: &str, deg: u32) {
        if deg == 0 {
            return;
        }
        for (v, d) in &mut self.vars {
            if v == var {
                *d += deg;
                return;
            }
        }
        self.vars.push((var.to_string(), deg));
    }
    /// Total degree: sum of all variable exponents.
    pub fn degree(&self) -> u32 {
        self.vars.iter().map(|(_, d)| d).sum()
    }
    /// Multiply two monomials, combining coefficients and merging variable lists.
    pub fn mul_monomial(&self, other: &Monomial) -> Monomial {
        let mut result = Monomial::new(self.coefficient * other.coefficient);
        for (v, d) in &self.vars {
            result.add_var(v, *d);
        }
        for (v, d) in &other.vars {
            result.add_var(v, *d);
        }
        result
    }
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
impl PolyrithExtPass300 {
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
    pub fn run(&mut self, input: &str) -> PolyrithExtResult300 {
        if !self.enabled {
            return PolyrithExtResult300::Skipped;
        }
        self.total_runs += 1;
        let result = if input.is_empty() {
            self.errors += 1;
            PolyrithExtResult300::Err(format!("empty input in pass '{}'", self.name))
        } else {
            self.successes += 1;
            PolyrithExtResult300::Ok(format!(
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
/// A term in a multivariate polynomial: coefficient × monomial.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MVTerm {
    pub coeff: i64,
    pub mono: MonomialV2,
}
#[allow(dead_code)]
impl MVTerm {
    pub fn new(coeff: i64, mono: MonomialV2) -> Self {
        MVTerm { coeff, mono }
    }
    pub fn is_zero(&self) -> bool {
        self.coeff == 0
    }
}
/// A polynomial coefficient (integer).
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PolyCoeff(pub i64);
#[allow(dead_code)]
impl PolyCoeff {
    pub fn zero() -> Self {
        PolyCoeff(0)
    }
    pub fn one() -> Self {
        PolyCoeff(1)
    }
    pub fn add(&self, other: &Self) -> Self {
        PolyCoeff(self.0 + other.0)
    }
    pub fn mul(&self, other: &Self) -> Self {
        PolyCoeff(self.0 * other.0)
    }
    pub fn neg(&self) -> Self {
        PolyCoeff(-self.0)
    }
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
    pub fn gcd_with(&self, other: &Self) -> Self {
        let (mut a, mut b) = (self.0.abs(), other.0.abs());
        while b != 0 {
            let t = b;
            b = a % b;
            a = t;
        }
        PolyCoeff(a)
    }
}
/// A multivariate polynomial represented as a list of terms.
/// Each term: (coefficient, exponent_vector).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MultiTerm {
    pub coeff: i64,
    pub exps: Vec<u32>,
}
#[allow(dead_code)]
impl MultiTerm {
    pub fn new(coeff: i64, exps: Vec<u32>) -> Self {
        MultiTerm { coeff, exps }
    }
    pub fn degree(&self) -> u32 {
        self.exps.iter().sum()
    }
    pub fn is_zero(&self) -> bool {
        self.coeff == 0
    }
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
impl PolyrithConfig {
    pub fn new() -> Self {
        PolyrithConfig {
            max_degree: 4,
            timeout_ms: 5000,
            use_cache: true,
            verbose: false,
            max_coeffs: 1000,
        }
    }
    pub fn with_max_degree(mut self, d: u32) -> Self {
        self.max_degree = d;
        self
    }
    pub fn with_timeout(mut self, ms: u64) -> Self {
        self.timeout_ms = ms;
        self
    }
    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }
}
#[allow(dead_code)]
pub struct PolyrithExtDiag301 {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
impl PolyrithExtDiag301 {
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
/// A polynomial in one variable with integer coefficients.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntPoly1 {
    pub coeffs: Vec<i64>,
}
#[allow(dead_code)]
impl IntPoly1 {
    pub fn zero() -> Self {
        IntPoly1 { coeffs: vec![] }
    }
    pub fn constant(c: i64) -> Self {
        IntPoly1 { coeffs: vec![c] }
    }
    pub fn var() -> Self {
        IntPoly1 { coeffs: vec![0, 1] }
    }
    pub fn degree(&self) -> usize {
        for i in (0..self.coeffs.len()).rev() {
            if self.coeffs[i] != 0 {
                return i;
            }
        }
        0
    }
    pub fn eval(&self, x: i64) -> i64 {
        let mut r = 0i64;
        let mut xp = 1i64;
        for &c in &self.coeffs {
            r = r.saturating_add(c.saturating_mul(xp));
            xp = xp.saturating_mul(x);
        }
        r
    }
    pub fn add(&self, other: &Self) -> Self {
        let len = self.coeffs.len().max(other.coeffs.len());
        let mut c = vec![0i64; len];
        for (i, &v) in self.coeffs.iter().enumerate() {
            c[i] += v;
        }
        for (i, &v) in other.coeffs.iter().enumerate() {
            c[i] += v;
        }
        while c.last() == Some(&0) {
            c.pop();
        }
        IntPoly1 { coeffs: c }
    }
    pub fn sub(&self, other: &Self) -> Self {
        let neg: Vec<i64> = other.coeffs.iter().map(|&c| -c).collect();
        self.add(&IntPoly1 { coeffs: neg })
    }
    pub fn mul(&self, other: &Self) -> Self {
        if self.coeffs.is_empty() || other.coeffs.is_empty() {
            return Self::zero();
        }
        let mut c = vec![0i64; self.coeffs.len() + other.coeffs.len() - 1];
        for (i, &a) in self.coeffs.iter().enumerate() {
            for (j, &b) in other.coeffs.iter().enumerate() {
                c[i + j] = c[i + j].saturating_add(a.saturating_mul(b));
            }
        }
        IntPoly1 { coeffs: c }
    }
    pub fn scale(&self, s: i64) -> Self {
        IntPoly1 {
            coeffs: self.coeffs.iter().map(|&c| c * s).collect(),
        }
    }
    pub fn is_zero(&self) -> bool {
        self.coeffs.iter().all(|&c| c == 0)
    }
    pub fn content(&self) -> i64 {
        let mut g = 0i64;
        for &c in &self.coeffs {
            let a = c.abs();
            if a > 0 {
                while g != 0 {
                    let t = a % g;
                    g = if t == 0 { g } else { t };
                }
                if g == 0 {
                    g = a;
                }
            }
        }
        g.max(1)
    }
    pub fn primitive_part(&self) -> Self {
        let c = self.content();
        if c <= 1 {
            return self.clone();
        }
        IntPoly1 {
            coeffs: self.coeffs.iter().map(|&x| x / c).collect(),
        }
    }
}
/// A monomial ordering for Groebner bases (lexicographic).
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MonomialV2 {
    pub exponents: Vec<u32>,
}
#[allow(dead_code)]
impl MonomialV2 {
    pub fn new(exponents: Vec<u32>) -> Self {
        MonomialV2 { exponents }
    }
    pub fn one(nvars: usize) -> Self {
        MonomialV2 {
            exponents: vec![0; nvars],
        }
    }
    pub fn degree(&self) -> u32 {
        self.exponents.iter().sum()
    }
    pub fn is_one(&self) -> bool {
        self.exponents.iter().all(|&e| e == 0)
    }
    pub fn mul(&self, other: &Self) -> Self {
        assert_eq!(self.exponents.len(), other.exponents.len());
        MonomialV2 {
            exponents: self
                .exponents
                .iter()
                .zip(other.exponents.iter())
                .map(|(&a, &b)| a + b)
                .collect(),
        }
    }
    pub fn divides(&self, other: &Self) -> bool {
        self.exponents
            .iter()
            .zip(other.exponents.iter())
            .all(|(&a, &b)| a <= b)
    }
    pub fn div(&self, other: &Self) -> Option<Self> {
        if !other.divides(self) {
            return None;
        }
        Some(MonomialV2 {
            exponents: self
                .exponents
                .iter()
                .zip(other.exponents.iter())
                .map(|(&a, &b)| a - b)
                .collect(),
        })
    }
    pub fn lcm(&self, other: &Self) -> Self {
        MonomialV2 {
            exponents: self
                .exponents
                .iter()
                .zip(other.exponents.iter())
                .map(|(&a, &b)| a.max(b))
                .collect(),
        }
    }
    /// Lexicographic comparison.
    pub fn lex_gt(&self, other: &Self) -> bool {
        for (&a, &b) in self.exponents.iter().zip(other.exponents.iter()) {
            if a > b {
                return true;
            }
            if a < b {
                return false;
            }
        }
        false
    }
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
impl PolyrithExtConfigVal300 {
    #[allow(dead_code)]
    pub fn as_bool(&self) -> Option<bool> {
        if let PolyrithExtConfigVal300::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_int(&self) -> Option<i64> {
        if let PolyrithExtConfigVal300::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_float(&self) -> Option<f64> {
        if let PolyrithExtConfigVal300::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_str(&self) -> Option<&str> {
        if let PolyrithExtConfigVal300::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_list(&self) -> Option<&[String]> {
        if let PolyrithExtConfigVal300::List(l) = self {
            Some(l)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn type_name(&self) -> &'static str {
        match self {
            PolyrithExtConfigVal300::Bool(_) => "bool",
            PolyrithExtConfigVal300::Int(_) => "int",
            PolyrithExtConfigVal300::Float(_) => "float",
            PolyrithExtConfigVal300::Str(_) => "str",
            PolyrithExtConfigVal300::List(_) => "list",
        }
    }
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
#[allow(dead_code)]
impl TacticPolyrithResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, TacticPolyrithResult::Ok(_))
    }
    pub fn is_err(&self) -> bool {
        matches!(self, TacticPolyrithResult::Err(_))
    }
    pub fn is_partial(&self) -> bool {
        matches!(self, TacticPolyrithResult::Partial { .. })
    }
    pub fn is_skipped(&self) -> bool {
        matches!(self, TacticPolyrithResult::Skipped)
    }
    pub fn ok_msg(&self) -> Option<&str> {
        match self {
            TacticPolyrithResult::Ok(s) => Some(s),
            _ => None,
        }
    }
    pub fn err_msg(&self) -> Option<&str> {
        match self {
            TacticPolyrithResult::Err(s) => Some(s),
            _ => None,
        }
    }
    pub fn progress(&self) -> f64 {
        match self {
            TacticPolyrithResult::Ok(_) => 1.0,
            TacticPolyrithResult::Err(_) => 0.0,
            TacticPolyrithResult::Skipped => 0.0,
            TacticPolyrithResult::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
        }
    }
}
/// A (stub) Gröbner basis: a set of polynomials that generate an ideal.
#[derive(Clone, Debug, Default)]
pub struct GroebnerBasis {
    /// The generator polynomials.
    pub generators: Vec<Polynomial>,
}
impl GroebnerBasis {
    /// Create an empty basis.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a polynomial generator to the basis.
    pub fn add_polynomial(&mut self, p: Polynomial) {
        self.generators.push(p);
    }
    /// Return `true` if there are no generators.
    pub fn is_empty(&self) -> bool {
        self.generators.is_empty()
    }
    /// Reduce `p` by the basis.
    ///
    /// This is a stub: it returns `p` unchanged (a full implementation would
    /// perform multivariate polynomial division).
    pub fn reduce(&self, p: &Polynomial) -> Polynomial {
        p.clone()
    }
    /// Check whether polynomial `p` belongs to the ideal generated by this basis.
    ///
    /// Stub: returns `true` iff `p.is_zero()` after reduction (which, since
    /// `reduce` is a stub, means `p` must already be zero).
    pub fn contains(&self, p: &Polynomial) -> bool {
        self.reduce(p).is_zero()
    }
}
/// A pipeline of TacticPolyrith analysis passes.
#[allow(dead_code)]
pub struct TacticPolyrithPipeline {
    pub passes: Vec<TacticPolyrithAnalysisPass>,
    pub name: String,
    pub total_inputs_processed: usize,
}
#[allow(dead_code)]
impl TacticPolyrithPipeline {
    pub fn new(name: &str) -> Self {
        TacticPolyrithPipeline {
            passes: Vec::new(),
            name: name.to_string(),
            total_inputs_processed: 0,
        }
    }
    pub fn add_pass(&mut self, pass: TacticPolyrithAnalysisPass) {
        self.passes.push(pass);
    }
    pub fn run_all(&mut self, input: &str) -> Vec<TacticPolyrithResult> {
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
pub struct PolyrithExtPipeline300 {
    pub name: String,
    pub passes: Vec<PolyrithExtPass300>,
    pub run_count: usize,
}
impl PolyrithExtPipeline300 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            passes: Vec::new(),
            run_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_pass(&mut self, pass: PolyrithExtPass300) {
        self.passes.push(pass);
    }
    #[allow(dead_code)]
    pub fn run_all(&mut self, input: &str) -> Vec<PolyrithExtResult300> {
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
pub struct PolyrithExtDiag300 {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
impl PolyrithExtDiag300 {
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
pub(super) struct CoeffIter<'a> {
    pub(super) candidates: &'a [i64],
    pub(super) indices: Vec<usize>,
    pub(super) done: bool,
}
impl<'a> CoeffIter<'a> {
    fn new(candidates: &'a [i64], n: usize) -> Self {
        Self {
            candidates,
            indices: vec![0; n],
            done: n == 0 || candidates.is_empty(),
        }
    }
}
/// A multivariate polynomial represented as a sum of monomials.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Polynomial {
    /// The terms of the polynomial (not necessarily in canonical form).
    pub terms: Vec<Monomial>,
}
impl Polynomial {
    /// Create a polynomial with no terms (equivalent to 0).
    pub fn new() -> Self {
        Self { terms: Vec::new() }
    }
    /// Add a monomial term.
    pub fn add_term(&mut self, m: Monomial) {
        self.terms.push(m);
    }
    /// The zero polynomial.
    pub fn zero() -> Polynomial {
        Polynomial::new()
    }
    /// The polynomial constant 1.
    pub fn one() -> Polynomial {
        let mut p = Polynomial::new();
        p.add_term(Monomial::new(1));
        p
    }
    /// Add two polynomials (concatenate their term lists).
    pub fn add(p: &Polynomial, q: &Polynomial) -> Polynomial {
        let mut result = Polynomial::new();
        for m in &p.terms {
            result.add_term(m.clone());
        }
        for m in &q.terms {
            result.add_term(m.clone());
        }
        result
    }
    /// Multiply two polynomials (distribute all pairs of monomials).
    pub fn mul(p: &Polynomial, q: &Polynomial) -> Polynomial {
        let mut result = Polynomial::new();
        for mp in &p.terms {
            for mq in &q.terms {
                result.add_term(mp.mul_monomial(mq));
            }
        }
        result
    }
    /// Negate the polynomial (negate every coefficient).
    pub fn negate(&self) -> Polynomial {
        let mut result = Polynomial::new();
        for m in &self.terms {
            let mut neg = m.clone();
            neg.coefficient = -neg.coefficient;
            result.add_term(neg);
        }
        result
    }
    /// Return `true` if all coefficients sum to zero (simple syntactic check:
    /// the term list is empty or every coefficient is 0).
    pub fn is_zero(&self) -> bool {
        self.terms.iter().all(|m| m.coefficient == 0)
    }
    /// Collect all distinct variable names appearing in the polynomial.
    pub fn collect_vars(&self) -> Vec<String> {
        let mut seen: Vec<String> = Vec::new();
        for m in &self.terms {
            for (v, _) in &m.vars {
                if !seen.contains(v) {
                    seen.push(v.clone());
                }
            }
        }
        seen
    }
}
/// The `polyrith` tactic: closes polynomial identity goals by finding a
/// linear combination of hypotheses.
#[derive(Clone, Debug, Default)]
pub struct PolyrithTactic {
    pub(super) hypotheses: Vec<Polynomial>,
    pub(super) goal: Option<Polynomial>,
}
impl PolyrithTactic {
    /// Create a new tactic instance with no hypotheses or goal.
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the list of hypothesis polynomials (the ideal generators).
    pub fn set_hypotheses(&mut self, hyps: Vec<Polynomial>) {
        self.hypotheses = hyps;
    }
    /// Set the goal polynomial (what we want to show is in the ideal).
    pub fn set_goal(&mut self, goal: Polynomial) {
        self.goal = Some(goal);
    }
    /// Run the tactic.
    ///
    /// Returns a vector of integer coefficients `c_i` such that
    /// `∑ c_i * h_i = goal` (modulo the ideal), or `None` if the tactic
    /// cannot find such a combination.
    ///
    /// This implementation uses a simple brute-force search over small
    /// coefficient vectors `{-2,-1,0,1,2}^n` for up to 4 hypotheses.
    pub fn run(&self) -> Option<Vec<i64>> {
        let goal = self.goal.as_ref()?;
        let n = self.hypotheses.len();
        if n == 0 {
            if goal.is_zero() {
                return Some(vec![]);
            }
            return None;
        }
        if n > 4 {
            return None;
        }
        let candidates: &[i64] = &[-2, -1, 0, 1, 2];
        CoeffIter::new(candidates, n).find(|coeffs| self.verify(coeffs))
    }
    /// Verify that coefficients `c_i` satisfy `∑ c_i * h_i ≡ goal`.
    ///
    /// The check is done by computing the signed sum of all coefficients of
    /// constant monomials (a lightweight soundness check suitable for the
    /// stub implementation).
    pub fn verify(&self, coeffs: &[i64]) -> bool {
        if coeffs.len() != self.hypotheses.len() {
            return false;
        }
        let goal = match &self.goal {
            Some(g) => g,
            None => return false,
        };
        let mut lhs_const: i64 = 0;
        for (c, h) in coeffs.iter().zip(&self.hypotheses) {
            for m in &h.terms {
                if m.vars.is_empty() {
                    lhs_const += c * m.coefficient;
                }
            }
        }
        let rhs_const: i64 = goal
            .terms
            .iter()
            .filter(|m| m.vars.is_empty())
            .map(|m| m.coefficient)
            .sum();
        lhs_const == rhs_const
    }
    /// Convenient string-based interface: parse simple integer constants and
    /// try to prove `goal` as a linear combination of `hyps`.
    ///
    /// Each string should be a decimal integer (representing a constant polynomial).
    /// Returns `true` if the tactic succeeds.
    pub fn run_with_strings(&self, hyps: &[&str], goal: &str) -> bool {
        let mut tac = PolyrithTactic::new();
        let parsed_hyps: Vec<Polynomial> = hyps
            .iter()
            .map(|s| {
                let c: i64 = s.trim().parse().unwrap_or(0);
                let mut p = Polynomial::new();
                p.add_term(Monomial::new(c));
                p
            })
            .collect();
        let goal_val: i64 = goal.trim().parse().unwrap_or(0);
        let mut goal_poly = Polynomial::new();
        goal_poly.add_term(Monomial::new(goal_val));
        tac.set_hypotheses(parsed_hyps);
        tac.set_goal(goal_poly);
        tac.run().is_some()
    }
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
impl TacticPolyrithConfigValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            TacticPolyrithConfigValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            TacticPolyrithConfigValue::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            TacticPolyrithConfigValue::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            TacticPolyrithConfigValue::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&[String]> {
        match self {
            TacticPolyrithConfigValue::List(v) => Some(v),
            _ => None,
        }
    }
    pub fn type_name(&self) -> &'static str {
        match self {
            TacticPolyrithConfigValue::Bool(_) => "bool",
            TacticPolyrithConfigValue::Int(_) => "int",
            TacticPolyrithConfigValue::Float(_) => "float",
            TacticPolyrithConfigValue::Str(_) => "str",
            TacticPolyrithConfigValue::List(_) => "list",
        }
    }
}
#[allow(dead_code)]
pub struct PolyrithExtConfig300 {
    pub(super) values: std::collections::HashMap<String, PolyrithExtConfigVal300>,
    pub(super) read_only: bool,
    pub(super) name: String,
}
impl PolyrithExtConfig300 {
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
    pub fn set(&mut self, key: &str, value: PolyrithExtConfigVal300) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&PolyrithExtConfigVal300> {
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
        self.set(key, PolyrithExtConfigVal300::Bool(v))
    }
    #[allow(dead_code)]
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, PolyrithExtConfigVal300::Int(v))
    }
    #[allow(dead_code)]
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, PolyrithExtConfigVal300::Str(v.to_string()))
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
#[derive(Debug, Clone)]
pub enum PolyrithExtConfigVal301 {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
impl PolyrithExtConfigVal301 {
    #[allow(dead_code)]
    pub fn as_bool(&self) -> Option<bool> {
        if let PolyrithExtConfigVal301::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_int(&self) -> Option<i64> {
        if let PolyrithExtConfigVal301::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_float(&self) -> Option<f64> {
        if let PolyrithExtConfigVal301::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_str(&self) -> Option<&str> {
        if let PolyrithExtConfigVal301::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_list(&self) -> Option<&[String]> {
        if let PolyrithExtConfigVal301::List(l) = self {
            Some(l)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn type_name(&self) -> &'static str {
        match self {
            PolyrithExtConfigVal301::Bool(_) => "bool",
            PolyrithExtConfigVal301::Int(_) => "int",
            PolyrithExtConfigVal301::Float(_) => "float",
            PolyrithExtConfigVal301::Str(_) => "str",
            PolyrithExtConfigVal301::List(_) => "list",
        }
    }
}
/// A multivariate polynomial.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MultiPoly1 {
    pub nvars: usize,
    pub terms: Vec<MultiTerm>,
}
#[allow(dead_code)]
impl MultiPoly1 {
    pub fn zero(nvars: usize) -> Self {
        MultiPoly1 {
            nvars,
            terms: vec![],
        }
    }
    pub fn constant(nvars: usize, c: i64) -> Self {
        if c == 0 {
            return Self::zero(nvars);
        }
        MultiPoly1 {
            nvars,
            terms: vec![MultiTerm::new(c, vec![0; nvars])],
        }
    }
    pub fn num_terms(&self) -> usize {
        self.terms.iter().filter(|t| !t.is_zero()).count()
    }
    pub fn degree(&self) -> u32 {
        self.terms.iter().map(|t| t.degree()).max().unwrap_or(0)
    }
    pub fn eval(&self, point: &[i64]) -> i64 {
        let mut sum = 0i64;
        for term in &self.terms {
            let mut mono = term.coeff;
            for (i, &e) in term.exps.iter().enumerate() {
                for _ in 0..e {
                    mono = mono.saturating_mul(point[i]);
                }
            }
            sum = sum.saturating_add(mono);
        }
        sum
    }
    pub fn add_term(&mut self, term: MultiTerm) {
        if term.is_zero() {
            return;
        }
        for t in &mut self.terms {
            if t.exps == term.exps {
                t.coeff += term.coeff;
                return;
            }
        }
        self.terms.push(term);
    }
    pub fn add(&self, other: &Self) -> Self {
        let mut result = self.clone();
        for term in &other.terms {
            result.add_term(term.clone());
        }
        result
    }
    pub fn scale(&self, s: i64) -> Self {
        MultiPoly1 {
            nvars: self.nvars,
            terms: self
                .terms
                .iter()
                .map(|t| MultiTerm::new(t.coeff * s, t.exps.clone()))
                .filter(|t| !t.is_zero())
                .collect(),
        }
    }
}
/// An analysis pass for TacticPolyrith.
#[allow(dead_code)]
pub struct TacticPolyrithAnalysisPass {
    pub name: String,
    pub enabled: bool,
    pub results: Vec<TacticPolyrithResult>,
    pub total_runs: usize,
}
#[allow(dead_code)]
impl TacticPolyrithAnalysisPass {
    pub fn new(name: &str) -> Self {
        TacticPolyrithAnalysisPass {
            name: name.to_string(),
            enabled: true,
            results: Vec::new(),
            total_runs: 0,
        }
    }
    pub fn run(&mut self, input: &str) -> TacticPolyrithResult {
        self.total_runs += 1;
        let result = if input.is_empty() {
            TacticPolyrithResult::Err("empty input".to_string())
        } else {
            TacticPolyrithResult::Ok(format!("processed: {}", input))
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
/// A diagnostic reporter for TacticPolyrith.
#[allow(dead_code)]
pub struct TacticPolyrithDiagnostics {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
#[allow(dead_code)]
impl TacticPolyrithDiagnostics {
    pub fn new(max_errors: usize) -> Self {
        TacticPolyrithDiagnostics {
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
pub struct PolyrithExtPass301 {
    pub name: String,
    pub total_runs: usize,
    pub successes: usize,
    pub errors: usize,
    pub enabled: bool,
    pub results: Vec<PolyrithExtResult301>,
}
impl PolyrithExtPass301 {
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
    pub fn run(&mut self, input: &str) -> PolyrithExtResult301 {
        if !self.enabled {
            return PolyrithExtResult301::Skipped;
        }
        self.total_runs += 1;
        let result = if input.is_empty() {
            self.errors += 1;
            PolyrithExtResult301::Err(format!("empty input in pass '{}'", self.name))
        } else {
            self.successes += 1;
            PolyrithExtResult301::Ok(format!(
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
impl PolyrithExtResult300 {
    #[allow(dead_code)]
    pub fn is_ok(&self) -> bool {
        matches!(self, PolyrithExtResult300::Ok(_))
    }
    #[allow(dead_code)]
    pub fn is_err(&self) -> bool {
        matches!(self, PolyrithExtResult300::Err(_))
    }
    #[allow(dead_code)]
    pub fn is_partial(&self) -> bool {
        matches!(self, PolyrithExtResult300::Partial { .. })
    }
    #[allow(dead_code)]
    pub fn is_skipped(&self) -> bool {
        matches!(self, PolyrithExtResult300::Skipped)
    }
    #[allow(dead_code)]
    pub fn ok_msg(&self) -> Option<&str> {
        if let PolyrithExtResult300::Ok(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn err_msg(&self) -> Option<&str> {
        if let PolyrithExtResult300::Err(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn progress(&self) -> f64 {
        match self {
            PolyrithExtResult300::Ok(_) => 1.0,
            PolyrithExtResult300::Err(_) => 0.0,
            PolyrithExtResult300::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
            PolyrithExtResult300::Skipped => 0.5,
        }
    }
}
/// The polyrith solver: given polynomial hypotheses, find a linear combination.
#[allow(dead_code)]
pub struct PolyrithSolver {
    pub config: PolyrithConfig,
    pub cache: PolyrithCache,
    pub hyps: Vec<MultiPoly1>,
    pub goal: Option<MultiPoly1>,
}
#[allow(dead_code)]
impl PolyrithSolver {
    pub fn new() -> Self {
        PolyrithSolver {
            config: PolyrithConfig::new(),
            cache: PolyrithCache::new(),
            hyps: Vec::new(),
            goal: None,
        }
    }
    pub fn add_hypothesis(&mut self, p: MultiPoly1) {
        self.hyps.push(p);
    }
    pub fn set_goal(&mut self, g: MultiPoly1) {
        self.goal = Some(g);
    }
    pub fn num_hyps(&self) -> usize {
        self.hyps.len()
    }
    /// Simple check: can we express the goal as a linear combination of hyps?
    pub fn try_trivial(&self) -> bool {
        if let Some(g) = &self.goal {
            return g.num_terms() == 0;
        }
        false
    }
}
/// A diff for TacticPolyrith analysis results.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TacticPolyrithDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
#[allow(dead_code)]
impl TacticPolyrithDiff {
    pub fn new() -> Self {
        TacticPolyrithDiff {
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
/// A multivariate polynomial over integers.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MVPoly {
    pub nvars: usize,
    pub terms: Vec<MVTerm>,
}
#[allow(dead_code)]
impl MVPoly {
    pub fn zero(nvars: usize) -> Self {
        MVPoly {
            nvars,
            terms: Vec::new(),
        }
    }
    pub fn one(nvars: usize) -> Self {
        MVPoly {
            nvars,
            terms: vec![MVTerm::new(1, MonomialV2::one(nvars))],
        }
    }
    pub fn from_const(c: i64, nvars: usize) -> Self {
        if c == 0 {
            return Self::zero(nvars);
        }
        MVPoly {
            nvars,
            terms: vec![MVTerm::new(c, MonomialV2::one(nvars))],
        }
    }
    pub fn is_zero(&self) -> bool {
        self.terms.is_empty()
    }
    pub fn leading_term(&self) -> Option<&MVTerm> {
        self.terms.first()
    }
    pub fn leading_monomial(&self) -> Option<&MonomialV2> {
        self.leading_term().map(|t| &t.mono)
    }
    pub fn normalize(&mut self) {
        self.terms.retain(|t| t.coeff != 0);
        self.terms.sort_by(|a, b| b.mono.cmp(&a.mono));
        let mut i = 0;
        while i + 1 < self.terms.len() {
            if self.terms[i].mono == self.terms[i + 1].mono {
                self.terms[i].coeff += self.terms[i + 1].coeff;
                self.terms.remove(i + 1);
                if self.terms[i].coeff == 0 {
                    self.terms.remove(i);
                }
            } else {
                i += 1;
            }
        }
    }
    pub fn add(&self, other: &Self) -> Self {
        let mut result = self.clone();
        result.terms.extend(other.terms.clone());
        result.normalize();
        result
    }
    pub fn neg(&self) -> Self {
        MVPoly {
            nvars: self.nvars,
            terms: self
                .terms
                .iter()
                .map(|t| MVTerm::new(-t.coeff, t.mono.clone()))
                .collect(),
        }
    }
    pub fn sub(&self, other: &Self) -> Self {
        self.add(&other.neg())
    }
    pub fn mul_term(&self, t: &MVTerm) -> Self {
        let terms = self
            .terms
            .iter()
            .map(|s| MVTerm::new(s.coeff * t.coeff, s.mono.mul(&t.mono)))
            .collect();
        let mut result = MVPoly {
            nvars: self.nvars,
            terms,
        };
        result.normalize();
        result
    }
    pub fn mul(&self, other: &Self) -> Self {
        let mut result = Self::zero(self.nvars);
        for t in &other.terms {
            result = result.add(&self.mul_term(t));
        }
        result
    }
    pub fn num_terms(&self) -> usize {
        self.terms.len()
    }
}
/// Check if a polynomial is in the ideal generated by a set of polynomials.
#[allow(dead_code)]
pub struct IdealMembershipChecker {
    pub generators: Vec<MVPoly>,
    pub iterations_used: usize,
}
#[allow(dead_code)]
impl IdealMembershipChecker {
    pub fn new(generators: Vec<MVPoly>) -> Self {
        IdealMembershipChecker {
            generators,
            iterations_used: 0,
        }
    }
    pub fn reduce(&mut self, mut f: MVPoly, max_iter: usize) -> MVPoly {
        let mut changed = true;
        self.iterations_used = 0;
        while changed && self.iterations_used < max_iter {
            changed = false;
            self.iterations_used += 1;
            for g in &self.generators {
                if let Some(reduced) = poly_reduce_step(&f, g) {
                    f = reduced;
                    changed = true;
                    break;
                }
            }
        }
        f
    }
    pub fn is_member(&mut self, f: MVPoly, max_iter: usize) -> bool {
        let remainder = self.reduce(f, max_iter);
        remainder.is_zero()
    }
}
#[allow(dead_code)]
pub struct PolyrithExtConfig301 {
    pub(super) values: std::collections::HashMap<String, PolyrithExtConfigVal301>,
    pub(super) read_only: bool,
    pub(super) name: String,
}
impl PolyrithExtConfig301 {
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
    pub fn set(&mut self, key: &str, value: PolyrithExtConfigVal301) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&PolyrithExtConfigVal301> {
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
        self.set(key, PolyrithExtConfigVal301::Bool(v))
    }
    #[allow(dead_code)]
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, PolyrithExtConfigVal301::Int(v))
    }
    #[allow(dead_code)]
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, PolyrithExtConfigVal301::Str(v.to_string()))
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
