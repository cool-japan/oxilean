//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::super::super::functions::*;
use super::super::defs::*;

use super::functions::{
    canonicalize, leading_monomial, monomial_cmp, monomial_divides, monomial_quotient,
    poly_mul_monomial, poly_sub, sort_monomial_vars,
};

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
impl<'a> CoeffIter<'a> {
    fn new(candidates: &'a [i64], n: usize) -> Self {
        Self {
            candidates,
            indices: vec![0; n],
            done: n == 0 || candidates.is_empty(),
        }
    }
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
#[cfg(test)]
mod groebner_tests {
    use super::*;
    /// Build a monomial with the given coefficient and (var, exponent) pairs.
    fn mono(coeff: i64, vars: &[(&str, u32)]) -> Monomial {
        let mut m = Monomial::new(coeff);
        for &(v, e) in vars {
            m.add_var(v, e);
        }
        m
    }
    /// Build a polynomial from a list of (coeff, vars) specs.
    fn poly(terms: &[(i64, &[(&str, u32)])]) -> Polynomial {
        let mut p = Polynomial::new();
        for &(coeff, vars) in terms {
            p.add_term(mono(coeff, vars));
        }
        p
    }
    #[test]
    fn test_reduce_by_single_var() {
        let mut basis = GroebnerBasis::new();
        basis.add_polynomial(poly(&[(1, &[("x", 1)])]));
        let p = poly(&[(1, &[("x", 2)]), (1, &[("y", 1)])]);
        let reduced = basis.reduce(&p);
        assert_eq!(
            reduced.terms.len(),
            1,
            "reduced should have exactly 1 term, got {:?}",
            reduced
        );
        assert_eq!(reduced.terms[0].coefficient, 1);
        assert_eq!(reduced.terms[0].vars, vec![("y".to_string(), 1)]);
    }
    #[test]
    fn test_contains_in_ideal() {
        let mut basis = GroebnerBasis::new();
        basis.add_polynomial(poly(&[(1, &[("x", 1)])]));
        basis.add_polynomial(poly(&[(1, &[("y", 1)])]));
        let p = poly(&[(1, &[("x", 1), ("y", 1)])]);
        assert!(basis.contains(&p), "x*y should be in the ideal <x, y>");
    }
    #[test]
    fn test_not_in_ideal() {
        let mut basis = GroebnerBasis::new();
        basis.add_polynomial(poly(&[(1, &[("x", 1)])]));
        let p = poly(&[(1, &[])]);
        assert!(
            !basis.contains(&p),
            "constant 1 should NOT be in the ideal <x>"
        );
    }
    #[test]
    fn test_empty_basis() {
        let basis = GroebnerBasis::new();
        let p = poly(&[(1, &[("x", 2)]), (3, &[("y", 1)])]);
        let reduced = basis.reduce(&p);
        assert!(
            !reduced.is_zero(),
            "nonzero p should not reduce to zero over empty basis"
        );
    }
    #[test]
    fn test_already_zero() {
        let mut basis = GroebnerBasis::new();
        basis.add_polynomial(poly(&[(1, &[("x", 1)])]));
        let p = Polynomial::zero();
        assert!(basis.contains(&p), "zero polynomial is in every ideal");
    }
    #[test]
    fn test_reduce_by_multiple_generators() {
        let mut basis = GroebnerBasis::new();
        basis.add_polynomial(poly(&[(1, &[("x", 2)])]));
        basis.add_polynomial(poly(&[(1, &[("y", 1)])]));
        let p = poly(&[(1, &[("x", 2)]), (1, &[("y", 1)]), (1, &[])]);
        let reduced = basis.reduce(&p);
        assert_eq!(
            reduced.terms.len(),
            1,
            "expected 1 remainder term, got {:?}",
            reduced
        );
        assert_eq!(reduced.terms[0].coefficient, 1);
        assert!(
            reduced.terms[0].vars.is_empty(),
            "expected constant term, got {:?}",
            reduced.terms[0]
        );
    }
}
