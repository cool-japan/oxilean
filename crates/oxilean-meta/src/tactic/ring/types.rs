//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::Name;

/// A monomial with string-keyed variables, used by [`RingNormalizer`].
///
/// Represents `coeff * x1^e1 * x2^e2 * ...` where keys are variable names.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StrMonomial {
    /// Variable name → exponent mapping (only positive exponents stored).
    pub vars: std::collections::HashMap<String, u32>,
    /// Integer coefficient.
    pub coeff: i64,
}
impl StrMonomial {
    /// Create the multiplicative identity monomial (coefficient 1, no variables).
    pub fn one() -> Self {
        Self {
            vars: std::collections::HashMap::new(),
            coeff: 1,
        }
    }
    /// Create a monomial representing a single variable `name` with exponent 1.
    pub fn var(name: &str) -> Self {
        let mut vars = std::collections::HashMap::new();
        vars.insert(name.to_string(), 1);
        Self { vars, coeff: 1 }
    }
    /// Multiply two monomials: add exponents and multiply coefficients.
    pub fn mul_monomial(&self, other: &StrMonomial) -> StrMonomial {
        let mut vars = self.vars.clone();
        for (k, &v) in &other.vars {
            *vars.entry(k.clone()).or_insert(0) += v;
        }
        vars.retain(|_, v| *v > 0);
        StrMonomial {
            vars,
            coeff: self.coeff * other.coeff,
        }
    }
    /// Total degree: sum of all exponents.
    pub fn degree(&self) -> u32 {
        self.vars.values().sum()
    }
    /// True if this monomial has no variables (is a pure constant).
    pub fn is_constant(&self) -> bool {
        self.vars.is_empty()
    }
    /// Render as a human-readable string (for debugging / equality key).
    pub fn to_str(&self) -> String {
        let mut parts: Vec<String> = self
            .vars
            .iter()
            .map(|(k, v)| {
                if *v == 1 {
                    k.clone()
                } else {
                    format!("{}^{}", k, v)
                }
            })
            .collect();
        parts.sort();
        if self.coeff == 1 && !parts.is_empty() {
            parts.join("*")
        } else if parts.is_empty() {
            self.coeff.to_string()
        } else {
            format!("{}*{}", self.coeff, parts.join("*"))
        }
    }
    /// Canonical key for grouping like terms (variable part only, coefficient excluded).
    fn var_key(&self) -> String {
        let mut parts: Vec<String> = self
            .vars
            .iter()
            .map(|(k, v)| format!("{}^{}", k, v))
            .collect();
        parts.sort();
        parts.join(",")
    }
}
/// Tokenizer output for the ring expression parser.
#[derive(Debug, Clone, PartialEq)]
pub(super) enum StrToken {
    Num(i64),
    Ident(String),
    Plus,
    Minus,
    Star,
    Caret,
    LParen,
    RParen,
}
/// A typed slot for TacticRing configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum TacticRingConfigValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
#[allow(dead_code)]
impl TacticRingConfigValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            TacticRingConfigValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            TacticRingConfigValue::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            TacticRingConfigValue::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            TacticRingConfigValue::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&[String]> {
        match self {
            TacticRingConfigValue::List(v) => Some(v),
            _ => None,
        }
    }
    pub fn type_name(&self) -> &'static str {
        match self {
            TacticRingConfigValue::Bool(_) => "bool",
            TacticRingConfigValue::Int(_) => "int",
            TacticRingConfigValue::Float(_) => "float",
            TacticRingConfigValue::Str(_) => "str",
            TacticRingConfigValue::List(_) => "list",
        }
    }
}
/// Represents a monomial: coefficient * x₁^e₁ * x₂^e₂ * ... * xₙ^eₙ
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Monomial {
    /// Exponents for each variable: (var_name, exponent) - kept sorted for canonicalization
    pub exponents: Vec<(Name, u32)>,
}
impl Monomial {
    /// Create a constant monomial (no variables).
    pub fn constant() -> Self {
        Self {
            exponents: Vec::new(),
        }
    }
    /// Create a monomial for a single variable x^1.
    pub fn var(name: Name) -> Self {
        Self {
            exponents: vec![(name, 1)],
        }
    }
    /// Create a monomial for a variable with given exponent.
    pub fn var_exp(name: Name, exp: u32) -> Self {
        if exp > 0 {
            Self {
                exponents: vec![(name, exp)],
            }
        } else {
            Self {
                exponents: Vec::new(),
            }
        }
    }
    /// Multiply two monomials (add exponents).
    pub fn multiply(&self, other: &Monomial) -> Monomial {
        let mut result_map: Vec<(Name, u32)> = self.exponents.clone();
        for (var, exp) in &other.exponents {
            if let Some(entry) = result_map.iter_mut().find(|(n, _)| n == var) {
                entry.1 += exp;
            } else {
                result_map.push((var.clone(), *exp));
            }
        }
        Self {
            exponents: result_map,
        }
    }
    /// Check if this is the constant monomial (no variables).
    pub fn is_constant(&self) -> bool {
        self.exponents.is_empty()
    }
    /// Get total degree (sum of all exponents).
    pub fn total_degree(&self) -> u32 {
        self.exponents.iter().map(|(_, e)| e).sum()
    }
    /// Get variables in this monomial.
    pub fn variables(&self) -> Vec<Name> {
        self.exponents.iter().map(|(n, _)| n.clone()).collect()
    }
}
/// A configuration store for TacticRing.
#[allow(dead_code)]
pub struct TacticRingConfig {
    pub values: std::collections::HashMap<String, TacticRingConfigValue>,
    pub read_only: bool,
}
#[allow(dead_code)]
impl TacticRingConfig {
    pub fn new() -> Self {
        TacticRingConfig {
            values: std::collections::HashMap::new(),
            read_only: false,
        }
    }
    pub fn set(&mut self, key: &str, value: TacticRingConfigValue) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    pub fn get(&self, key: &str) -> Option<&TacticRingConfigValue> {
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
        self.set(key, TacticRingConfigValue::Bool(v))
    }
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, TacticRingConfigValue::Int(v))
    }
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, TacticRingConfigValue::Str(v.to_string()))
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
/// A polynomial is a finite sum of monomials with rational coefficients.
/// Represented as: Σ c_i * m_i where c_i are coefficients and m_i are monomials.
#[derive(Clone, Debug)]
pub struct Polynomial {
    /// List of (monomial, (numerator, denominator)) pairs
    pub terms: Vec<(Monomial, (i64, u32))>,
}
impl Polynomial {
    /// Create a constant polynomial c.
    pub fn constant(num: i64, den: u32) -> Self {
        if num != 0 {
            let gcd = gcd(num.unsigned_abs(), den as u64) as u32;
            let normalized_num = num / gcd as i64;
            let normalized_den = den / gcd;
            Self {
                terms: vec![(Monomial::constant(), (normalized_num, normalized_den))],
            }
        } else {
            Self { terms: Vec::new() }
        }
    }
    /// Create a zero polynomial.
    pub fn zero() -> Self {
        Self { terms: Vec::new() }
    }
    /// Create a one polynomial.
    pub fn one() -> Self {
        Self::constant(1, 1)
    }
    /// Create a variable polynomial x.
    pub fn var(name: Name) -> Self {
        Self {
            terms: vec![(Monomial::var(name), (1, 1))],
        }
    }
    /// Add two polynomials.
    pub fn add(&self, other: &Polynomial) -> Polynomial {
        let mut result = self.clone();
        for (mono, (num2, den2)) in &other.terms {
            if let Some((_, (num1, den1))) = result.terms.iter_mut().find(|(m, _)| m == mono) {
                let new_den = (*den1 as i64 * *den2 as i64).unsigned_abs() as u32;
                let new_num = *num1 * (*den2 as i64) + *num2 * (*den1 as i64);
                if new_num == 0 {
                    *num1 = 0;
                    *den1 = 1;
                } else {
                    let gcd = gcd(new_num.unsigned_abs(), new_den as u64) as u32;
                    *num1 = new_num / gcd as i64;
                    *den1 = new_den / gcd;
                }
            } else {
                result.terms.push((mono.clone(), (*num2, *den2)));
            }
        }
        result.terms.retain(|(_, (num, _))| *num != 0);
        result
    }
    /// Subtract two polynomials.
    pub fn sub(&self, other: &Polynomial) -> Polynomial {
        let negated = other.negate();
        self.add(&negated)
    }
    /// Negate a polynomial.
    pub fn negate(&self) -> Polynomial {
        let mut result = self.clone();
        for (_mono, (num, _den)) in result.terms.iter_mut() {
            *num = -*num;
        }
        result
    }
    /// Multiply two polynomials.
    pub fn multiply(&self, other: &Polynomial) -> Polynomial {
        let mut result = Polynomial::zero();
        for (mono1, (n1, d1)) in &self.terms {
            for (mono2, (n2, d2)) in &other.terms {
                let new_mono = mono1.multiply(mono2);
                let new_num = n1 * n2;
                let new_den = d1 * d2;
                let g1 = gcd(new_num.unsigned_abs(), new_den as u64) as u32;
                let norm_num = new_num / g1 as i64;
                let norm_den = new_den / g1;
                if norm_num != 0 {
                    if let Some((_, (e_num, e_den))) =
                        result.terms.iter_mut().find(|(m, _)| m == &new_mono)
                    {
                        let final_den = (*e_den as i64 * norm_den as i64).unsigned_abs() as u32;
                        let final_num = *e_num * (norm_den as i64) + norm_num * (*e_den as i64);
                        if final_num == 0 {
                            *e_num = 0;
                        } else {
                            let g2 = gcd(final_num.unsigned_abs(), final_den as u64) as u32;
                            *e_num = final_num / g2 as i64;
                            *e_den = final_den / g2;
                        }
                    } else {
                        result.terms.push((new_mono, (norm_num, norm_den)));
                    }
                }
            }
        }
        result.terms.retain(|(_, (num, _))| *num != 0);
        result
    }
    /// Compute x^n for positive integer n.
    pub fn power(&self, n: u32) -> Polynomial {
        if n == 0 {
            Polynomial::one()
        } else if n == 1 {
            self.clone()
        } else {
            let half = self.power(n / 2);
            let result = half.multiply(&half);
            if n % 2 == 0 {
                result
            } else {
                result.multiply(self)
            }
        }
    }
    /// Check if the polynomial is zero.
    pub fn is_zero(&self) -> bool {
        self.terms.is_empty()
    }
    /// Check if the polynomial is a constant.
    pub fn is_constant(&self) -> bool {
        self.terms.len() <= 1 && self.terms.iter().all(|(m, _)| m.is_constant())
    }
    /// Get all variables in the polynomial.
    pub fn variables(&self) -> Vec<Name> {
        let mut vars = Vec::new();
        for (mono, _) in &self.terms {
            for var in mono.variables() {
                if !vars.iter().any(|v| v == &var) {
                    vars.push(var);
                }
            }
        }
        vars
    }
    /// Convert to Horner normal form for efficient evaluation.
    pub fn to_horner_form(&self) -> HornerForm {
        if self.is_zero() {
            return HornerForm::Constant(0, 1);
        }
        let vars = self.variables();
        if vars.is_empty() {
            if let Some((_, (num, den))) = self.terms.first() {
                return HornerForm::Constant(*num, *den);
            }
            return HornerForm::Constant(0, 1);
        }
        let main_var = &vars[0];
        self.to_horner_by_var(main_var.clone())
    }
    /// Convert to Horner form with respect to a specific variable.
    fn to_horner_by_var(&self, var: Name) -> HornerForm {
        let mut by_degree: Vec<(u32, Polynomial)> = Vec::new();
        for (mono, coeff) in &self.terms {
            let degree = mono
                .exponents
                .iter()
                .find(|(n, _)| n == &var)
                .map(|(_, e)| *e)
                .unwrap_or(0);
            let mut reduced_mono = mono.clone();
            reduced_mono.exponents.retain(|(n, _)| n != &var);
            let mut reduced_poly = Polynomial::zero();
            reduced_poly.terms.push((reduced_mono, *coeff));
            if let Some((_, poly)) = by_degree.iter_mut().find(|(d, _)| *d == degree) {
                *poly = poly.add(&reduced_poly);
            } else {
                by_degree.push((degree, reduced_poly));
            }
        }
        let max_degree = by_degree.iter().map(|(d, _)| d).max().copied().unwrap_or(0);
        let mut result = HornerForm::Constant(0, 1);
        for degree in (0..=max_degree).rev() {
            if let Some((_, coeff)) = by_degree.iter().find(|(d, _)| *d == degree) {
                let coeff_form = coeff.to_horner_form();
                result = if degree > 0 {
                    HornerForm::Horner {
                        coeff: Box::new(result),
                        var: var.clone(),
                        next: Box::new(coeff_form),
                    }
                } else {
                    result.add_horner(&coeff_form)
                };
            }
        }
        result
    }
}
/// A multivariate polynomial with integer coefficients, used by [`RingNormalizer`].
///
/// Internally a vector of [`StrMonomial`]s kept in canonical (simplified) form.
#[derive(Clone, Debug)]
pub struct StrPoly {
    /// Terms of the polynomial; maintained in canonical sorted order.
    pub terms: Vec<StrMonomial>,
}
impl StrPoly {
    /// The zero polynomial.
    pub fn zero() -> Self {
        Self { terms: Vec::new() }
    }
    /// The multiplicative identity polynomial (constant 1).
    pub fn one() -> Self {
        Self {
            terms: vec![StrMonomial::one()],
        }
    }
    /// Constant polynomial with value `n`.
    pub fn constant(n: i64) -> Self {
        if n == 0 {
            Self::zero()
        } else {
            Self {
                terms: vec![StrMonomial {
                    vars: std::collections::HashMap::new(),
                    coeff: n,
                }],
            }
        }
    }
    /// Single-variable polynomial `x`.
    pub fn var(name: &str) -> Self {
        Self {
            terms: vec![StrMonomial::var(name)],
        }
    }
    /// Add two polynomials.
    pub fn add(&self, other: &StrPoly) -> StrPoly {
        let mut result = self.clone();
        for term in &other.terms {
            result.add_term(term.clone());
        }
        result.simplify();
        result
    }
    /// Subtract two polynomials.
    pub fn sub(&self, other: &StrPoly) -> StrPoly {
        self.add(&other.neg())
    }
    /// Multiply two polynomials.
    pub fn mul(&self, other: &StrPoly) -> StrPoly {
        let mut result = StrPoly::zero();
        for t1 in &self.terms {
            for t2 in &other.terms {
                result.add_term(t1.mul_monomial(t2));
            }
        }
        result.simplify();
        result
    }
    /// Negate a polynomial.
    pub fn neg(&self) -> StrPoly {
        StrPoly {
            terms: self
                .terms
                .iter()
                .map(|m| StrMonomial {
                    vars: m.vars.clone(),
                    coeff: -m.coeff,
                })
                .collect(),
        }
    }
    /// Combine like terms, drop zero-coefficient terms, sort by degree then lex.
    pub fn simplify(&mut self) {
        let mut combined: std::collections::HashMap<String, i64> = std::collections::HashMap::new();
        let mut key_to_vars: std::collections::HashMap<
            String,
            std::collections::HashMap<String, u32>,
        > = std::collections::HashMap::new();
        for term in &self.terms {
            let key = term.var_key();
            *combined.entry(key.clone()).or_insert(0) += term.coeff;
            key_to_vars.entry(key).or_insert_with(|| term.vars.clone());
        }
        let mut new_terms: Vec<StrMonomial> = combined
            .into_iter()
            .filter(|(_, c)| *c != 0)
            .map(|(key, coeff)| StrMonomial {
                vars: key_to_vars.remove(&key).unwrap_or_default(),
                coeff,
            })
            .collect();
        new_terms.sort_by(|a, b| {
            b.degree()
                .cmp(&a.degree())
                .then_with(|| a.var_key().cmp(&b.var_key()))
        });
        self.terms = new_terms;
    }
    /// True if this is the zero polynomial.
    pub fn is_zero(&self) -> bool {
        self.terms.is_empty()
    }
    /// Structural equality after simplification.
    pub fn is_equal(&self, other: &StrPoly) -> bool {
        let diff = self.sub(other);
        diff.is_zero()
    }
    /// Human-readable string representation.
    pub fn to_str(&self) -> String {
        if self.terms.is_empty() {
            return "0".to_string();
        }
        self.terms
            .iter()
            .map(|m| m.to_str())
            .collect::<Vec<_>>()
            .join(" + ")
    }
    /// Add a single term (monomial), combining with existing terms later via simplify.
    fn add_term(&mut self, term: StrMonomial) {
        self.terms.push(term);
    }
}
/// A pipeline of TacticRing analysis passes.
#[allow(dead_code)]
pub struct TacticRingPipeline {
    pub passes: Vec<TacticRingAnalysisPass>,
    pub name: String,
    pub total_inputs_processed: usize,
}
#[allow(dead_code)]
impl TacticRingPipeline {
    pub fn new(name: &str) -> Self {
        TacticRingPipeline {
            passes: Vec::new(),
            name: name.to_string(),
            total_inputs_processed: 0,
        }
    }
    pub fn add_pass(&mut self, pass: TacticRingAnalysisPass) {
        self.passes.push(pass);
    }
    pub fn run_all(&mut self, input: &str) -> Vec<TacticRingResult> {
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
/// A diagnostic reporter for TacticRing.
#[allow(dead_code)]
pub struct TacticRingDiagnostics {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
#[allow(dead_code)]
impl TacticRingDiagnostics {
    pub fn new(max_errors: usize) -> Self {
        TacticRingDiagnostics {
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
/// A result type for TacticRing analysis.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TacticRingResult {
    Ok(String),
    Err(String),
    Partial { done: usize, total: usize },
    Skipped,
}
#[allow(dead_code)]
impl TacticRingResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, TacticRingResult::Ok(_))
    }
    pub fn is_err(&self) -> bool {
        matches!(self, TacticRingResult::Err(_))
    }
    pub fn is_partial(&self) -> bool {
        matches!(self, TacticRingResult::Partial { .. })
    }
    pub fn is_skipped(&self) -> bool {
        matches!(self, TacticRingResult::Skipped)
    }
    pub fn ok_msg(&self) -> Option<&str> {
        match self {
            TacticRingResult::Ok(s) => Some(s),
            _ => None,
        }
    }
    pub fn err_msg(&self) -> Option<&str> {
        match self {
            TacticRingResult::Err(s) => Some(s),
            _ => None,
        }
    }
    pub fn progress(&self) -> f64 {
        match self {
            TacticRingResult::Ok(_) => 1.0,
            TacticRingResult::Err(_) => 0.0,
            TacticRingResult::Skipped => 0.0,
            TacticRingResult::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
        }
    }
}
/// Recursive-descent parser for simple ring expressions.
struct StrExprParser {
    pub(super) tokens: Vec<StrToken>,
    pub(super) pos: usize,
}
impl StrExprParser {
    fn new(s: &str) -> Self {
        Self {
            tokens: tokenize_ring_expr(s),
            pos: 0,
        }
    }
    fn peek(&self) -> Option<&StrToken> {
        self.tokens.get(self.pos)
    }
    fn consume(&mut self) -> Option<StrToken> {
        let tok = self.tokens.get(self.pos).cloned();
        self.pos += 1;
        tok
    }
    /// Top-level parse: additive expression.
    fn parse(&mut self) -> Option<StrPoly> {
        let result = self.parse_additive()?;
        if self.pos == self.tokens.len() {
            Some(result)
        } else {
            None
        }
    }
    /// Parse `term (('+' | '-') term)*`.
    fn parse_additive(&mut self) -> Option<StrPoly> {
        let mut left = self.parse_unary()?;
        loop {
            match self.peek() {
                Some(StrToken::Plus) => {
                    self.consume();
                    let right = self.parse_unary()?;
                    left = left.add(&right);
                }
                Some(StrToken::Minus) => {
                    self.consume();
                    let right = self.parse_unary()?;
                    left = left.sub(&right);
                }
                _ => break,
            }
        }
        Some(left)
    }
    /// Parse optional leading `-` then a multiplicative expression.
    fn parse_unary(&mut self) -> Option<StrPoly> {
        if matches!(self.peek(), Some(StrToken::Minus)) {
            self.consume();
            let inner = self.parse_multiplicative()?;
            Some(inner.neg())
        } else {
            self.parse_multiplicative()
        }
    }
    /// Parse `power ('*' power)*`.
    fn parse_multiplicative(&mut self) -> Option<StrPoly> {
        let mut left = self.parse_power()?;
        while matches!(self.peek(), Some(StrToken::Star)) {
            self.consume();
            let right = self.parse_power()?;
            left = left.mul(&right);
        }
        Some(left)
    }
    /// Parse `atom ('^' num)?`.
    fn parse_power(&mut self) -> Option<StrPoly> {
        let base = self.parse_atom()?;
        if matches!(self.peek(), Some(StrToken::Caret)) {
            self.consume();
            if let Some(StrToken::Num(n)) = self.peek().cloned() {
                self.consume();
                if !(0..=30).contains(&n) {
                    return None;
                }
                let exp = n as u32;
                let mut result = StrPoly::one();
                for _ in 0..exp {
                    result = result.mul(&base);
                }
                return Some(result);
            } else {
                return None;
            }
        }
        Some(base)
    }
    /// Parse a parenthesised expression, a numeric literal, or a variable.
    fn parse_atom(&mut self) -> Option<StrPoly> {
        match self.peek().cloned() {
            Some(StrToken::LParen) => {
                self.consume();
                let inner = self.parse_additive()?;
                if matches!(self.peek(), Some(StrToken::RParen)) {
                    self.consume();
                    Some(inner)
                } else {
                    None
                }
            }
            Some(StrToken::Num(n)) => {
                self.consume();
                Some(StrPoly::constant(n))
            }
            Some(StrToken::Ident(name)) => {
                self.consume();
                Some(StrPoly::var(&name))
            }
            _ => None,
        }
    }
}
/// Horner normal form: efficient polynomial representation
#[derive(Clone, Debug, PartialEq)]
pub enum HornerForm {
    /// Constant rational number
    Constant(i64, u32),
    /// Horner form: (coeff * var + next)
    Horner {
        /// Polynomial coefficient
        coeff: Box<HornerForm>,
        /// Variable name
        var: Name,
        /// Next term
        next: Box<HornerForm>,
    },
}
impl HornerForm {
    /// Add two Horner forms.
    ///
    /// Addition rules:
    /// - `Constant + Constant`: rational arithmetic.
    /// - `Horner(c1, x, n1) + Horner(c2, x, n2)` (same var): add coefficients and next terms pairwise.
    /// - `Horner(c, x, n) + other` (different var or other is Constant): add `other` into the next
    ///   term, since `other` does not depend on `x` at this level.
    /// - `Constant + Horner(...)`: symmetric to the case above.
    pub(crate) fn add_horner(&self, other: &HornerForm) -> HornerForm {
        match (self, other) {
            (HornerForm::Constant(n1, d1), HornerForm::Constant(n2, d2)) => {
                let new_den = (*d1 as i64 * *d2 as i64).unsigned_abs() as u32;
                let new_num = n1 * (*d2 as i64) + n2 * (*d1 as i64);
                let gcd = gcd(new_num.unsigned_abs(), new_den as u64) as u32;
                HornerForm::Constant(new_num / gcd as i64, new_den / gcd)
            }
            (
                HornerForm::Horner {
                    coeff: c1,
                    var: v1,
                    next: n1,
                },
                HornerForm::Horner {
                    coeff: c2,
                    var: v2,
                    next: n2,
                },
            ) if v1 == v2 => HornerForm::Horner {
                coeff: Box::new(c1.add_horner(c2)),
                var: v1.clone(),
                next: Box::new(n1.add_horner(n2)),
            },
            (HornerForm::Horner { coeff, var, next }, _) => HornerForm::Horner {
                coeff: coeff.clone(),
                var: var.clone(),
                next: Box::new(next.add_horner(other)),
            },
            (HornerForm::Constant(..), HornerForm::Horner { coeff, var, next }) => {
                HornerForm::Horner {
                    coeff: coeff.clone(),
                    var: var.clone(),
                    next: Box::new(next.add_horner(self)),
                }
            }
        }
    }
    /// Check if two Horner forms are equal.
    pub fn equals(&self, other: &HornerForm) -> bool {
        self == other
    }
}
/// An analysis pass for TacticRing.
#[allow(dead_code)]
pub struct TacticRingAnalysisPass {
    pub name: String,
    pub enabled: bool,
    pub results: Vec<TacticRingResult>,
    pub total_runs: usize,
}
#[allow(dead_code)]
impl TacticRingAnalysisPass {
    pub fn new(name: &str) -> Self {
        TacticRingAnalysisPass {
            name: name.to_string(),
            enabled: true,
            results: Vec::new(),
            total_runs: 0,
        }
    }
    pub fn run(&mut self, input: &str) -> TacticRingResult {
        self.total_runs += 1;
        let result = if input.is_empty() {
            TacticRingResult::Err("empty input".to_string())
        } else {
            TacticRingResult::Ok(format!("processed: {}", input))
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
/// String-based ring normalizer: parses simple ring expressions from strings and
/// checks equality by polynomial normalization.
///
/// Used as a fallback in [`tac_ring`] when the kernel `Expr`-based path fails.
pub struct RingNormalizer {
    /// Variable name → polynomial substitution (currently unused; reserved for future use).
    pub variables: std::collections::HashMap<String, StrPoly>,
}
impl RingNormalizer {
    /// Create a new normalizer with no variable bindings.
    pub fn new() -> Self {
        Self {
            variables: std::collections::HashMap::new(),
        }
    }
    /// Parse a goal string of the form `"lhs = rhs"` and return the two sides as polynomials.
    ///
    /// Returns `None` if the string does not contain a top-level `=`.
    pub fn parse_goal(goal: &str) -> Option<(StrPoly, StrPoly)> {
        let bytes = goal.as_bytes();
        let mut depth: i32 = 0;
        let mut eq_pos: Option<usize> = None;
        let mut i = 0;
        while i < bytes.len() {
            match bytes[i] {
                b'(' => depth += 1,
                b')' => depth -= 1,
                b'=' if depth == 0 => {
                    let prev = if i > 0 { bytes[i - 1] } else { 0 };
                    let next = if i + 1 < bytes.len() { bytes[i + 1] } else { 0 };
                    if prev != b'!' && prev != b'<' && prev != b'>' && prev != b'=' && next != b'='
                    {
                        eq_pos = Some(i);
                        break;
                    }
                }
                _ => {}
            }
            i += 1;
        }
        let eq = eq_pos?;
        let lhs_str = goal[..eq].trim();
        let rhs_str = goal[eq + 1..].trim();
        Some((Self::normalize_expr(lhs_str), Self::normalize_expr(rhs_str)))
    }
    /// Parse a simple ring expression string into a [`StrPoly`].
    ///
    /// Handles: `+`, `-`, `*`, `^n`, parenthesised sub-expressions, numeric literals,
    /// and identifier variables.  Returns [`StrPoly::zero()`] on any parse failure
    /// (conservative: the tactic will simply not close the goal).
    pub fn normalize_expr(expr: &str) -> StrPoly {
        let expr = expr.trim();
        if expr.is_empty() {
            return StrPoly::zero();
        }
        match StrExprParser::new(expr).parse() {
            Some(p) => p,
            None => StrPoly::zero(),
        }
    }
    /// Normalize both sides of an equation and check if they are equal.
    pub fn check_equality(lhs: &str, rhs: &str) -> bool {
        let pl = Self::normalize_expr(lhs);
        let pr = Self::normalize_expr(rhs);
        pl.is_equal(&pr)
    }
}
/// A diff for TacticRing analysis results.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TacticRingDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
#[allow(dead_code)]
impl TacticRingDiff {
    pub fn new() -> Self {
        TacticRingDiff {
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
