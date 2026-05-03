//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::super::super::functions::*;
use super::super::defs::*;

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
#[allow(dead_code)]
impl MVTerm {
    pub fn new(coeff: i64, mono: MonomialV2) -> Self {
        MVTerm { coeff, mono }
    }
    pub fn is_zero(&self) -> bool {
        self.coeff == 0
    }
}
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
/// Compare two monomials using graded reverse-lex (graded by total degree,
/// then lex on variable names, both descending so that the "largest" term
/// sorts first in a slice sorted by this comparator).
pub(super) fn monomial_cmp(a: &Monomial, b: &Monomial) -> std::cmp::Ordering {
    let deg_a: u32 = a.vars.iter().map(|(_, e)| e).sum();
    let deg_b: u32 = b.vars.iter().map(|(_, e)| e).sum();
    match deg_b.cmp(&deg_a) {
        std::cmp::Ordering::Equal => {}
        other => return other,
    }
    let mut vars: Vec<&str> = Vec::new();
    for (v, _) in &a.vars {
        if !vars.contains(&v.as_str()) {
            vars.push(v.as_str());
        }
    }
    for (v, _) in &b.vars {
        if !vars.contains(&v.as_str()) {
            vars.push(v.as_str());
        }
    }
    vars.sort_unstable();
    vars.reverse();
    for var in vars {
        let ea = a
            .vars
            .iter()
            .find(|(v, _)| v == var)
            .map(|(_, e)| *e)
            .unwrap_or(0);
        let eb = b
            .vars
            .iter()
            .find(|(v, _)| v == var)
            .map(|(_, e)| *e)
            .unwrap_or(0);
        match eb.cmp(&ea) {
            std::cmp::Ordering::Equal => {}
            other => return other,
        }
    }
    std::cmp::Ordering::Equal
}
/// Return the index of the leading monomial (highest by `monomial_cmp`) in `p`.
pub(super) fn leading_monomial(p: &Polynomial) -> Option<usize> {
    if p.terms.is_empty() {
        return None;
    }
    let mut best = 0usize;
    for i in 1..p.terms.len() {
        if monomial_cmp(&p.terms[i], &p.terms[best]) == std::cmp::Ordering::Less {
        } else {
            best = i;
        }
    }
    Some(best)
}
/// Return `true` iff `divisor` (non-zero coefficient) divides `dividend`:
/// every variable exponent in `divisor` is `<=` that in `dividend`, and
/// the coefficient of `dividend` is exactly divisible by that of `divisor`.
pub(super) fn monomial_divides(divisor: &Monomial, dividend: &Monomial) -> bool {
    if divisor.coefficient == 0 {
        return false;
    }
    if dividend.coefficient % divisor.coefficient != 0 {
        return false;
    }
    for (var, exp_d) in &divisor.vars {
        let exp_dd = dividend
            .vars
            .iter()
            .find(|(v, _)| v == var)
            .map(|(_, e)| *e)
            .unwrap_or(0);
        if exp_dd < *exp_d {
            return false;
        }
    }
    true
}
/// Compute `dividend / divisor` (assumes `monomial_divides` is true).
pub(super) fn monomial_quotient(dividend: &Monomial, divisor: &Monomial) -> Monomial {
    let coeff = dividend.coefficient / divisor.coefficient;
    let mut vars: Vec<(String, u32)> = Vec::new();
    for (var, exp) in &dividend.vars {
        let sub = divisor
            .vars
            .iter()
            .find(|(v, _)| v == var)
            .map(|(_, e)| *e)
            .unwrap_or(0);
        let result_exp = exp - sub;
        if result_exp > 0 {
            vars.push((var.clone(), result_exp));
        }
    }
    for (var, _exp) in &divisor.vars {
        if !dividend.vars.iter().any(|(v, _)| v == var) {}
    }
    Monomial {
        vars,
        coefficient: coeff,
    }
}
/// Multiply every term of `p` by monomial `m`.
pub(super) fn poly_mul_monomial(p: &Polynomial, m: &Monomial) -> Polynomial {
    let mut result = Polynomial::new();
    for term in &p.terms {
        result.add_term(term.mul_monomial(m));
    }
    result
}
/// Sort the variable list within a monomial so equality checks are canonical.
pub(super) fn sort_monomial_vars(m: &mut Monomial) {
    m.vars.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));
}
/// Combine like terms, drop zero-coefficient terms, and sort by `monomial_cmp`.
pub(super) fn canonicalize(p: &mut Polynomial) {
    for m in &mut p.terms {
        sort_monomial_vars(m);
    }
    let mut combined: Vec<Monomial> = Vec::new();
    for term in std::mem::take(&mut p.terms) {
        let mut found = false;
        for existing in &mut combined {
            if existing.vars == term.vars {
                existing.coefficient += term.coefficient;
                found = true;
                break;
            }
        }
        if !found {
            combined.push(term);
        }
    }
    combined.retain(|m| m.coefficient != 0);
    combined.sort_unstable_by(monomial_cmp);
    p.terms = combined;
}
/// Compute `a - b` as a new polynomial (fully canonicalized).
pub(super) fn poly_sub(a: &Polynomial, b: &Polynomial) -> Polynomial {
    let mut result = Polynomial::new();
    for m in &a.terms {
        result.add_term(m.clone());
    }
    for m in &b.terms {
        let mut neg = m.clone();
        neg.coefficient = -neg.coefficient;
        result.add_term(neg);
    }
    canonicalize(&mut result);
    result
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
    /// Reduce `p` by the basis using the multivariate polynomial division
    /// algorithm (Cox–Little–O'Shea §2.3).
    ///
    /// Returns the remainder after dividing `p` by all generators in order.
    /// If the result is the zero polynomial, `p` is in the ideal.
    pub fn reduce(&self, p: &Polynomial) -> Polynomial {
        let mut p_remaining = p.clone();
        canonicalize(&mut p_remaining);
        let mut remainder = Polynomial::new();
        loop {
            match leading_monomial(&p_remaining) {
                None => break,
                Some(lead_idx) => {
                    let lead_term = p_remaining.terms[lead_idx].clone();
                    let divisor_found = self.generators.iter().find_map(|g| {
                        let mut g_canon = g.clone();
                        canonicalize(&mut g_canon);
                        let g_lead_idx = leading_monomial(&g_canon)?;
                        let g_lead = &g_canon.terms[g_lead_idx];
                        if monomial_divides(g_lead, &lead_term) {
                            let q = monomial_quotient(&lead_term, g_lead);
                            Some(poly_mul_monomial(&g_canon, &q))
                        } else {
                            None
                        }
                    });
                    match divisor_found {
                        Some(subtrahend) => {
                            p_remaining = poly_sub(&p_remaining, &subtrahend);
                        }
                        None => {
                            remainder.add_term(lead_term);
                            p_remaining.terms.remove(lead_idx);
                            canonicalize(&mut p_remaining);
                        }
                    }
                }
            }
        }
        canonicalize(&mut remainder);
        remainder
    }
    /// Check whether polynomial `p` belongs to the ideal generated by this basis.
    ///
    /// Returns `true` iff `reduce(p)` is the zero polynomial.
    pub fn contains(&self, p: &Polynomial) -> bool {
        self.reduce(p).is_zero()
    }
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
