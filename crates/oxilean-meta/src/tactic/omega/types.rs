//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::Name;
use std::collections::HashMap;

/// A linear constraint.
#[derive(Clone, Debug)]
pub enum LinearConstraint {
    /// `expr ≤ 0`
    Le(LinearExpr),
    /// `expr = 0`
    Eq(LinearExpr),
    /// `expr ≥ 0` (stored as `-expr ≤ 0`)
    Ge(LinearExpr),
}
impl LinearConstraint {
    /// Convert `expr ≤ 0` constraint (canonical form).
    pub fn le(expr: LinearExpr) -> Self {
        LinearConstraint::Le(expr)
    }
    /// Convert `expr ≥ 0` constraint.
    pub fn ge(expr: LinearExpr) -> Self {
        LinearConstraint::Ge(expr)
    }
    /// Convert `expr = 0` constraint.
    pub fn eq(expr: LinearExpr) -> Self {
        LinearConstraint::Eq(expr)
    }
    /// `a ≤ b` → `a - b ≤ 0`.
    pub fn a_le_b(a: LinearExpr, b: LinearExpr) -> Self {
        LinearConstraint::Le(a.sub(&b))
    }
    /// `a ≥ b` → `b - a ≤ 0`.
    pub fn a_ge_b(a: LinearExpr, b: LinearExpr) -> Self {
        LinearConstraint::Le(b.sub(&a))
    }
    /// `a = b` → `a - b = 0`.
    pub fn a_eq_b(a: LinearExpr, b: LinearExpr) -> Self {
        LinearConstraint::Eq(a.sub(&b))
    }
    /// `a < b` → `a - b + 1 ≤ 0` (integer strict inequality).
    pub fn a_lt_b(a: LinearExpr, b: LinearExpr) -> Self {
        let mut expr = a.sub(&b);
        expr.constant += 1;
        LinearConstraint::Le(expr)
    }
    /// Get the underlying expression.
    pub fn expr(&self) -> &LinearExpr {
        match self {
            LinearConstraint::Le(e) | LinearConstraint::Eq(e) | LinearConstraint::Ge(e) => e,
        }
    }
    /// Normalize to canonical form (Le).
    pub fn to_le(&self) -> LinearConstraint {
        match self {
            LinearConstraint::Le(_) => self.clone(),
            LinearConstraint::Ge(e) => LinearConstraint::Le(e.negate()),
            LinearConstraint::Eq(e) => LinearConstraint::Le(e.clone()),
        }
    }
    /// Check if constraint is trivially true.
    pub fn is_trivially_true(&self) -> bool {
        let e = self.expr();
        if !e.is_constant() {
            return false;
        }
        match self {
            LinearConstraint::Le(_) => e.constant <= 0,
            LinearConstraint::Ge(_) => e.constant >= 0,
            LinearConstraint::Eq(_) => e.constant == 0,
        }
    }
    /// Check if constraint is trivially false.
    pub fn is_trivially_false(&self) -> bool {
        let e = self.expr();
        if !e.is_constant() {
            return false;
        }
        match self {
            LinearConstraint::Le(_) => e.constant > 0,
            LinearConstraint::Ge(_) => e.constant < 0,
            LinearConstraint::Eq(_) => e.constant != 0,
        }
    }
    /// Get the coefficient of a given variable.
    pub fn coeff_of(&self, var: &Name) -> i64 {
        self.expr().coeff_of(var)
    }
    /// Normalize: divide all coefficients by GCD.
    pub fn normalize(&self) -> LinearConstraint {
        match self {
            LinearConstraint::Le(e) => LinearConstraint::Le(e.normalize()),
            LinearConstraint::Eq(e) => LinearConstraint::Eq(e.normalize()),
            LinearConstraint::Ge(e) => LinearConstraint::Ge(e.normalize()),
        }
    }
    /// Substitute a variable.
    pub fn substitute(&self, var: &Name, replacement: &LinearExpr) -> LinearConstraint {
        match self {
            LinearConstraint::Le(e) => LinearConstraint::Le(e.substitute(var, replacement)),
            LinearConstraint::Eq(e) => LinearConstraint::Eq(e.substitute(var, replacement)),
            LinearConstraint::Ge(e) => LinearConstraint::Ge(e.substitute(var, replacement)),
        }
    }
}
/// A diagnostic reporter for TacticOmega.
#[allow(dead_code)]
pub struct TacticOmegaDiagnostics {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
#[allow(dead_code)]
impl TacticOmegaDiagnostics {
    pub fn new(max_errors: usize) -> Self {
        TacticOmegaDiagnostics {
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
pub struct OmegaExtPass4200 {
    pub name: String,
    pub total_runs: usize,
    pub successes: usize,
    pub errors: usize,
    pub enabled: bool,
    pub results: Vec<OmegaExtResult4200>,
}
impl OmegaExtPass4200 {
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
    pub fn run(&mut self, input: &str) -> OmegaExtResult4200 {
        if !self.enabled {
            return OmegaExtResult4200::Skipped;
        }
        self.total_runs += 1;
        let result = if input.is_empty() {
            self.errors += 1;
            OmegaExtResult4200::Err(format!("empty input in pass '{}'", self.name))
        } else {
            self.successes += 1;
            OmegaExtResult4200::Ok(format!(
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
pub struct OmegaExtDiag4200 {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
impl OmegaExtDiag4200 {
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
pub struct OmegaExtDiff4200 {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
impl OmegaExtDiff4200 {
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
/// The Omega solver implementing Pugh's Omega test.
pub struct OmegaSolver {
    pub(super) config: OmegaConfig,
    pub(super) steps: usize,
}
impl OmegaSolver {
    /// Create a new solver with default config.
    pub fn new() -> Self {
        OmegaSolver {
            config: OmegaConfig::default(),
            steps: 0,
        }
    }
    /// Create a new solver with custom config.
    pub fn with_config(config: OmegaConfig) -> Self {
        OmegaSolver { config, steps: 0 }
    }
    /// Main entry: solve a system of linear constraints.
    ///
    /// Returns `Unsatisfiable` if the system has no solution (goal is proved),
    /// or `Satisfiable` if a solution might exist.
    pub fn solve(&mut self, constraints: &[LinearConstraint]) -> OmegaResult {
        self.steps = 0;
        let mut system: Vec<LinearConstraint> = constraints.to_vec();
        if self.config.nat_mode {
            system = self.add_nat_constraints(system);
        }
        self.solve_system(system, 0)
    }
    /// Solve the constraint system recursively.
    fn solve_system(&mut self, system: Vec<LinearConstraint>, depth: usize) -> OmegaResult {
        if self.steps >= self.config.max_steps {
            return OmegaResult::Satisfiable;
        }
        self.steps += 1;
        for c in &system {
            if c.is_trivially_false() {
                let val = c.expr().constant;
                return OmegaResult::Unsatisfiable(OmegaProof::contradiction(val));
            }
        }
        let system: Vec<_> = system
            .into_iter()
            .filter(|c| !c.is_trivially_true())
            .collect();
        if system.is_empty() {
            return OmegaResult::Satisfiable;
        }
        if self.config.use_preprocessing {
            let pre = self.preprocess(system);
            if pre.is_unsat {
                return OmegaResult::Unsatisfiable(OmegaProof::contradiction(1));
            }
            if pre.constraints.is_empty() {
                return OmegaResult::Satisfiable;
            }
            return self.fourier_motzkin(pre.constraints, depth);
        }
        self.fourier_motzkin(system, depth)
    }
    /// Add non-negativity constraints for variables in Nat mode.
    fn add_nat_constraints(&self, mut system: Vec<LinearConstraint>) -> Vec<LinearConstraint> {
        let mut nat_vars: Vec<Name> = Vec::new();
        for c in &system {
            for var in c.expr().variables() {
                if !nat_vars.contains(&var) {
                    nat_vars.push(var);
                }
            }
        }
        for var in nat_vars {
            let neg_var = LinearExpr::scaled_var(var, -1);
            system.push(LinearConstraint::Le(neg_var));
        }
        system
    }
    /// Preprocess: GCD reduction, equality elimination.
    fn preprocess(&self, system: Vec<LinearConstraint>) -> PreprocessResult {
        let mut constraints = system;
        let mut eliminated = Vec::new();
        constraints = constraints.iter().map(|c| c.normalize()).collect();
        let mut changed = true;
        while changed {
            changed = false;
            let eq_idx = constraints
                .iter()
                .position(|c| matches!(c, LinearConstraint::Eq(_)));
            if let Some(idx) = eq_idx {
                let eq_constraint = constraints.remove(idx);
                let expr = eq_constraint.expr().clone();
                if let Some((var, coeff)) = expr.terms.iter().find(|(_, c)| c.abs() == 1).cloned() {
                    let rest = expr.eliminate_var(&var);
                    let var_expr = rest.scale(-1).div_by(coeff.signum());
                    eliminated.push((var.clone(), var_expr.clone()));
                    constraints = constraints
                        .into_iter()
                        .map(|c| c.substitute(&var, &var_expr))
                        .collect();
                    changed = true;
                } else if !expr.is_constant() {
                    constraints.insert(idx, LinearConstraint::Eq(expr));
                } else if expr.constant != 0 {
                    return PreprocessResult {
                        constraints: vec![],
                        eliminated,
                        is_unsat: true,
                    };
                }
            } else {
                break;
            }
        }
        for c in &constraints {
            if c.is_trivially_false() {
                return PreprocessResult {
                    constraints: vec![],
                    eliminated,
                    is_unsat: true,
                };
            }
        }
        let constraints: Vec<_> = constraints
            .into_iter()
            .filter(|c| !c.is_trivially_true())
            .collect();
        PreprocessResult {
            constraints,
            eliminated,
            is_unsat: false,
        }
    }
    /// Fourier-Motzkin variable elimination.
    fn fourier_motzkin(&mut self, system: Vec<LinearConstraint>, depth: usize) -> OmegaResult {
        if system.is_empty() {
            return OmegaResult::Satisfiable;
        }
        let var = match self.choose_variable(&system) {
            Some(v) => v,
            None => {
                for c in &system {
                    if c.is_trivially_false() {
                        return OmegaResult::Unsatisfiable(OmegaProof::contradiction(
                            c.expr().constant,
                        ));
                    }
                }
                return OmegaResult::Satisfiable;
            }
        };
        let mut lower_bounds: Vec<LinearConstraint> = Vec::new();
        let mut upper_bounds: Vec<LinearConstraint> = Vec::new();
        let mut independent: Vec<LinearConstraint> = Vec::new();
        for c in &system {
            let c_norm = match c {
                LinearConstraint::Ge(e) => LinearConstraint::Le(e.negate()),
                LinearConstraint::Eq(e) => {
                    let le1 = LinearConstraint::Le(e.clone());
                    let le2 = LinearConstraint::Le(e.negate());
                    let coeff = e.coeff_of(&var);
                    if coeff > 0 {
                        upper_bounds.push(le1);
                        lower_bounds.push(le2);
                    } else if coeff < 0 {
                        lower_bounds.push(le1);
                        upper_bounds.push(le2);
                    } else {
                        independent.push(le1);
                        if !le2.is_trivially_true() {
                            independent.push(le2);
                        }
                    }
                    continue;
                }
                other => other.clone(),
            };
            let coeff = c_norm.expr().coeff_of(&var);
            if coeff > 0 {
                upper_bounds.push(c_norm);
            } else if coeff < 0 {
                lower_bounds.push(c_norm);
            } else {
                independent.push(c_norm);
            }
        }
        if upper_bounds.is_empty() || lower_bounds.is_empty() {
            return self.solve_system(independent, depth + 1);
        }
        let mut new_system = independent;
        if self.config.use_dark_gray_shadow {
            match self.dark_shadow(&var, &lower_bounds, &upper_bounds) {
                DarkShadowResult::Unsat => OmegaResult::Unsatisfiable(OmegaProof {
                    steps: vec![OmegaStep::DarkShadow {
                        upper_idx: 0,
                        lower_idx: 0,
                    }],
                }),
                DarkShadowResult::NewConstraints(cs) => {
                    new_system.extend(cs);
                    self.solve_system(new_system, depth + 1)
                }
                DarkShadowResult::NeedGrayShadow => {
                    self.gray_shadow(&var, &lower_bounds, &upper_bounds, new_system, depth)
                }
            }
        } else {
            for lb in &lower_bounds {
                for ub in &upper_bounds {
                    if let Some(combined) = fm_combine(&var, lb, ub) {
                        new_system.push(combined);
                    }
                }
            }
            self.solve_system(new_system, depth + 1)
        }
    }
    /// Choose a variable to eliminate (heuristic: fewest occurrences).
    fn choose_variable(&self, system: &[LinearConstraint]) -> Option<Name> {
        let mut counts: HashMap<Name, usize> = HashMap::new();
        for c in system {
            for var in c.expr().variables() {
                *counts.entry(var).or_insert(0) += 1;
            }
        }
        counts
            .into_iter()
            .min_by_key(|(_, count)| *count)
            .map(|(var, _)| var)
    }
}
impl OmegaSolver {
    /// The dark shadow procedure.
    ///
    /// For each pair of lower bound `a·x ≥ l` and upper bound `b·x ≤ u`,
    /// the dark shadow constraint is: `b·l - a·u + (a-1)·(b-1) ≤ 0`.
    ///
    /// If the dark shadow is infeasible, we know there's no integer solution.
    /// If the dark shadow is feasible, the original system might be feasible.
    /// If neither is decisive, we use the gray shadow (case analysis).
    fn dark_shadow(
        &mut self,
        var: &Name,
        lower_bounds: &[LinearConstraint],
        upper_bounds: &[LinearConstraint],
    ) -> DarkShadowResult {
        let mut new_constraints = Vec::new();
        let mut need_gray = false;
        for lb in lower_bounds {
            let lb_expr = lb.expr();
            let a = (-lb_expr.coeff_of(var)).abs();
            let rest_lb = lb_expr.eliminate_var(var);
            for ub in upper_bounds {
                let ub_expr = ub.expr();
                let b = ub_expr.coeff_of(var);
                let rest_ub = ub_expr.eliminate_var(var);
                let dark = rest_lb
                    .scale(b)
                    .add(&rest_ub.scale(a).negate())
                    .add(&LinearExpr::constant(-((a - 1) * (b - 1))));
                let fm = rest_lb.scale(b).add(&rest_ub.scale(a));
                let fm_constraint = LinearConstraint::Le(fm);
                if fm_constraint.is_trivially_false() {
                    return DarkShadowResult::Unsat;
                }
                let dark_constraint = LinearConstraint::Le(dark);
                if !dark_constraint.is_trivially_true() {
                    if a > 1 && b > 1 {
                        need_gray = true;
                    }
                    new_constraints.push(fm_constraint);
                }
            }
        }
        if need_gray && self.config.allow_case_splits {
            DarkShadowResult::NeedGrayShadow
        } else {
            DarkShadowResult::NewConstraints(new_constraints)
        }
    }
    /// The gray shadow procedure (case analysis on integer gaps).
    ///
    /// When the dark shadow is not sufficient, we do a case analysis:
    /// for each possible value of `var` in [lb, ub], substitute and check.
    fn gray_shadow(
        &mut self,
        var: &Name,
        lower_bounds: &[LinearConstraint],
        upper_bounds: &[LinearConstraint],
        independent: Vec<LinearConstraint>,
        depth: usize,
    ) -> OmegaResult {
        let (lb_val, ub_val) = self.find_integer_bounds(var, lower_bounds, upper_bounds);
        let lb_val = match lb_val {
            Some(v) => v,
            None => return OmegaResult::Satisfiable,
        };
        let ub_val = match ub_val {
            Some(v) => v,
            None => return OmegaResult::Satisfiable,
        };
        if lb_val > ub_val {
            return OmegaResult::Unsatisfiable(OmegaProof {
                steps: vec![OmegaStep::GrayShadow {
                    var: var.clone(),
                    lower: lb_val,
                    upper: ub_val,
                }],
            });
        }
        let range = ub_val - lb_val;
        if range > self.config.max_case_splits as i64 {
            let mut new_system = independent;
            for lb in lower_bounds {
                for ub in upper_bounds {
                    if let Some(combined) = fm_combine(var, lb, ub) {
                        new_system.push(combined);
                    }
                }
            }
            return self.solve_system(new_system, depth + 1);
        }
        for val in lb_val..=ub_val {
            let replacement = LinearExpr::constant(val);
            let mut subst_system = independent.clone();
            for lb in lower_bounds {
                subst_system.push(lb.substitute(var, &replacement));
            }
            for ub in upper_bounds {
                subst_system.push(ub.substitute(var, &replacement));
            }
            match self.solve_system(subst_system, depth + 1) {
                OmegaResult::Satisfiable => return OmegaResult::Satisfiable,
                OmegaResult::Unsatisfiable(_) => {}
            }
        }
        OmegaResult::Unsatisfiable(OmegaProof {
            steps: vec![OmegaStep::GrayShadow {
                var: var.clone(),
                lower: lb_val,
                upper: ub_val,
            }],
        })
    }
    /// Find the tightest integer bounds on a variable.
    fn find_integer_bounds(
        &self,
        var: &Name,
        lower_bounds: &[LinearConstraint],
        upper_bounds: &[LinearConstraint],
    ) -> (Option<i64>, Option<i64>) {
        let mut lb = None::<i64>;
        let mut ub = None::<i64>;
        for c in lower_bounds {
            let e = c.expr();
            let coeff = e.coeff_of(var);
            if coeff == 0 {
                continue;
            }
            let rest = e.eliminate_var(var);
            if rest.is_constant() {
                let threshold = div_ceil(-rest.constant, -coeff);
                lb = Some(match lb {
                    None => threshold,
                    Some(prev) => prev.max(threshold),
                });
            }
        }
        for c in upper_bounds {
            let e = c.expr();
            let coeff = e.coeff_of(var);
            if coeff == 0 {
                continue;
            }
            let rest = e.eliminate_var(var);
            if rest.is_constant() {
                let threshold = div_floor(-rest.constant, coeff);
                ub = Some(match ub {
                    None => threshold,
                    Some(prev) => prev.min(threshold),
                });
            }
        }
        (lb, ub)
    }
}
/// A result type for TacticOmega analysis.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TacticOmegaResult {
    Ok(String),
    Err(String),
    Partial { done: usize, total: usize },
    Skipped,
}
#[allow(dead_code)]
impl TacticOmegaResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, TacticOmegaResult::Ok(_))
    }
    pub fn is_err(&self) -> bool {
        matches!(self, TacticOmegaResult::Err(_))
    }
    pub fn is_partial(&self) -> bool {
        matches!(self, TacticOmegaResult::Partial { .. })
    }
    pub fn is_skipped(&self) -> bool {
        matches!(self, TacticOmegaResult::Skipped)
    }
    pub fn ok_msg(&self) -> Option<&str> {
        match self {
            TacticOmegaResult::Ok(s) => Some(s),
            _ => None,
        }
    }
    pub fn err_msg(&self) -> Option<&str> {
        match self {
            TacticOmegaResult::Err(s) => Some(s),
            _ => None,
        }
    }
    pub fn progress(&self) -> f64 {
        match self {
            TacticOmegaResult::Ok(_) => 1.0,
            TacticOmegaResult::Err(_) => 0.0,
            TacticOmegaResult::Skipped => 0.0,
            TacticOmegaResult::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
        }
    }
}
/// A linear expression: `c₀ + c₁·x₁ + c₂·x₂ + ...`
#[derive(Clone, Debug, PartialEq)]
pub struct LinearExpr {
    /// Constant term.
    pub constant: i64,
    /// Variable coefficients: (variable_name, coefficient).
    pub terms: Vec<(Name, i64)>,
}
impl LinearExpr {
    /// Create a constant linear expression.
    pub fn constant(n: i64) -> Self {
        Self {
            constant: n,
            terms: Vec::new(),
        }
    }
    /// Create a single variable (with coefficient 1).
    pub fn var(name: Name) -> Self {
        Self {
            constant: 0,
            terms: vec![(name, 1)],
        }
    }
    /// Create a scaled variable.
    pub fn scaled_var(name: Name, coeff: i64) -> Self {
        if coeff == 0 {
            Self::constant(0)
        } else {
            Self {
                constant: 0,
                terms: vec![(name, coeff)],
            }
        }
    }
    /// Add two linear expressions.
    pub fn add(&self, other: &LinearExpr) -> LinearExpr {
        let mut result = self.clone();
        result.constant += other.constant;
        for (name, coeff) in &other.terms {
            if let Some(entry) = result.terms.iter_mut().find(|(n, _)| n == name) {
                entry.1 += coeff;
            } else {
                result.terms.push((name.clone(), *coeff));
            }
        }
        result.terms.retain(|(_, c)| *c != 0);
        result
    }
    /// Subtract another linear expression.
    pub fn sub(&self, other: &LinearExpr) -> LinearExpr {
        let neg = other.negate();
        self.add(&neg)
    }
    /// Negate a linear expression.
    pub fn negate(&self) -> LinearExpr {
        LinearExpr {
            constant: -self.constant,
            terms: self.terms.iter().map(|(n, c)| (n.clone(), -c)).collect(),
        }
    }
    /// Multiply by a scalar.
    pub fn scale(&self, factor: i64) -> LinearExpr {
        if factor == 0 {
            return LinearExpr::constant(0);
        }
        LinearExpr {
            constant: self.constant * factor,
            terms: self
                .terms
                .iter()
                .map(|(n, c)| (n.clone(), c * factor))
                .collect(),
        }
    }
    /// Check if this is a constant expression.
    pub fn is_constant(&self) -> bool {
        self.terms.is_empty()
    }
    /// Get all variable names.
    pub fn variables(&self) -> Vec<Name> {
        self.terms.iter().map(|(n, _)| n.clone()).collect()
    }
    /// Get the coefficient of a variable (0 if not present).
    pub fn coeff_of(&self, var: &Name) -> i64 {
        self.terms
            .iter()
            .find(|(n, _)| n == var)
            .map(|(_, c)| *c)
            .unwrap_or(0)
    }
    /// Compute GCD of all coefficients and the constant.
    pub fn gcd(&self) -> i64 {
        let mut g = self.constant.abs();
        for (_, c) in &self.terms {
            g = gcd(g, c.abs());
        }
        if g == 0 {
            1
        } else {
            g
        }
    }
    /// Divide all coefficients by d (must divide evenly).
    pub fn div_by(&self, d: i64) -> LinearExpr {
        debug_assert!(d != 0);
        LinearExpr {
            constant: self.constant / d,
            terms: self.terms.iter().map(|(n, c)| (n.clone(), c / d)).collect(),
        }
    }
    /// Normalize: divide all coefficients by their GCD.
    pub fn normalize(&self) -> LinearExpr {
        let g = self.gcd();
        self.div_by(g)
    }
    /// Evaluate on an integer assignment.
    pub fn eval(&self, assignment: &HashMap<Name, i64>) -> i64 {
        let mut val = self.constant;
        for (n, c) in &self.terms {
            val += c * assignment.get(n).copied().unwrap_or(0);
        }
        val
    }
    /// Remove a variable from the expression (set its coefficient to 0).
    pub fn eliminate_var(&self, var: &Name) -> LinearExpr {
        let mut result = self.clone();
        result.terms.retain(|(n, _)| n != var);
        result
    }
    /// Substitute a variable with a linear expression.
    pub fn substitute(&self, var: &Name, replacement: &LinearExpr) -> LinearExpr {
        let coeff = self.coeff_of(var);
        if coeff == 0 {
            return self.clone();
        }
        let base = self.eliminate_var(var);
        base.add(&replacement.scale(coeff))
    }
}
/// Preprocessing result: constraints after simplification.
#[derive(Clone, Debug)]
struct PreprocessResult {
    pub(super) constraints: Vec<LinearConstraint>,
    pub(super) eliminated: Vec<(Name, LinearExpr)>,
    pub(super) is_unsat: bool,
}
/// A diff for TacticOmega analysis results.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TacticOmegaDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
#[allow(dead_code)]
impl TacticOmegaDiff {
    pub fn new() -> Self {
        TacticOmegaDiff {
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
/// A proof term produced by the omega procedure.
#[derive(Clone, Debug)]
pub struct OmegaProof {
    /// Sequence of deduction steps.
    pub steps: Vec<OmegaStep>,
}
impl OmegaProof {
    /// Create an empty proof.
    pub fn empty() -> Self {
        OmegaProof { steps: Vec::new() }
    }
    /// Create a proof with a single contradiction step.
    pub fn contradiction(value: i64) -> Self {
        OmegaProof {
            steps: vec![OmegaStep::Contradiction { value }],
        }
    }
}
#[allow(dead_code)]
pub struct OmegaExtPipeline4200 {
    pub name: String,
    pub passes: Vec<OmegaExtPass4200>,
    pub run_count: usize,
}
impl OmegaExtPipeline4200 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            passes: Vec::new(),
            run_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_pass(&mut self, pass: OmegaExtPass4200) {
        self.passes.push(pass);
    }
    #[allow(dead_code)]
    pub fn run_all(&mut self, input: &str) -> Vec<OmegaExtResult4200> {
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
/// Result of the omega decision procedure.
#[derive(Clone, Debug)]
pub enum OmegaResult {
    /// The constraints are satisfiable (cannot prove the negation).
    Satisfiable,
    /// The constraints are unsatisfiable (the negated goal is proved).
    Unsatisfiable(OmegaProof),
}
impl OmegaResult {
    /// Returns true if the system is unsatisfiable.
    pub fn is_unsat(&self) -> bool {
        matches!(self, OmegaResult::Unsatisfiable(_))
    }
    /// Returns true if the system is satisfiable.
    pub fn is_sat(&self) -> bool {
        matches!(self, OmegaResult::Satisfiable)
    }
}
/// A typed slot for TacticOmega configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum TacticOmegaConfigValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
#[allow(dead_code)]
impl TacticOmegaConfigValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            TacticOmegaConfigValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            TacticOmegaConfigValue::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            TacticOmegaConfigValue::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            TacticOmegaConfigValue::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&[String]> {
        match self {
            TacticOmegaConfigValue::List(v) => Some(v),
            _ => None,
        }
    }
    pub fn type_name(&self) -> &'static str {
        match self {
            TacticOmegaConfigValue::Bool(_) => "bool",
            TacticOmegaConfigValue::Int(_) => "int",
            TacticOmegaConfigValue::Float(_) => "float",
            TacticOmegaConfigValue::Str(_) => "str",
            TacticOmegaConfigValue::List(_) => "list",
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum OmegaExtConfigVal4200 {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
impl OmegaExtConfigVal4200 {
    #[allow(dead_code)]
    pub fn as_bool(&self) -> Option<bool> {
        if let OmegaExtConfigVal4200::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_int(&self) -> Option<i64> {
        if let OmegaExtConfigVal4200::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_float(&self) -> Option<f64> {
        if let OmegaExtConfigVal4200::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_str(&self) -> Option<&str> {
        if let OmegaExtConfigVal4200::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_list(&self) -> Option<&[String]> {
        if let OmegaExtConfigVal4200::List(l) = self {
            Some(l)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn type_name(&self) -> &'static str {
        match self {
            OmegaExtConfigVal4200::Bool(_) => "bool",
            OmegaExtConfigVal4200::Int(_) => "int",
            OmegaExtConfigVal4200::Float(_) => "float",
            OmegaExtConfigVal4200::Str(_) => "str",
            OmegaExtConfigVal4200::List(_) => "list",
        }
    }
}
/// String-based omega goal parser.
///
/// Parses a goal expressed as a string into linear constraints for `OmegaSolver`.
///
/// # Example
/// ```ignore
/// let cs = OmegaGoalParser::parse_goal("x + 1 <= y");
/// // Returns Some([a_le_b(x+1, y)])
/// ```
pub struct OmegaGoalParser;
impl OmegaGoalParser {
    /// Parse a goal string into linear constraints.
    ///
    /// Handles:
    /// - Comparisons: `a <= b`, `a < b`, `a = b`, `a >= b`, `a > b`, `a != b`
    /// - Unicode: `≤`, `≥`, `≠`
    /// - Conjunction: `A /\ B` or `A ∧ B`
    ///
    /// Returns `None` if the goal cannot be parsed as linear arithmetic.
    pub fn parse_goal(goal_str: &str) -> Option<Vec<LinearConstraint>> {
        let goal_str = goal_str.trim();
        let parts: Vec<&str> = if goal_str.contains(" /\\ ") {
            goal_str.split(" /\\ ").collect()
        } else if goal_str.contains(" \u{2227} ") {
            goal_str.split(" \u{2227} ").collect()
        } else {
            vec![goal_str]
        };
        if parts.len() > 1 {
            let mut all = Vec::new();
            for part in &parts {
                match Self::parse_goal(part.trim()) {
                    Some(cs) => all.extend(cs),
                    None => return None,
                }
            }
            return Some(all);
        }
        let constraint = parse_constraint_str(goal_str)?;
        Some(vec![constraint])
    }
}
/// A step in an omega proof.
#[derive(Clone, Debug)]
pub enum OmegaStep {
    /// A linear combination of constraints yields a contradiction.
    LinearCombination {
        /// Coefficients for each input constraint.
        coeffs: Vec<i64>,
        /// Resulting expression (should be a positive constant ≤ 0).
        result: LinearExpr,
    },
    /// Case split on a variable's value.
    CaseSplit {
        /// The variable being split on.
        var: Name,
        /// Lower bound.
        lower: i64,
        /// Upper bound.
        upper: i64,
    },
    /// Equality elimination: express var in terms of others.
    EqualityElim {
        /// The variable being eliminated.
        var: Name,
        /// The expression it equals.
        expr: LinearExpr,
    },
    /// Dark shadow application.
    DarkShadow {
        /// Upper bound constraint index.
        upper_idx: usize,
        /// Lower bound constraint index.
        lower_idx: usize,
    },
    /// Gray shadow case analysis.
    GrayShadow {
        /// The variable.
        var: Name,
        /// Lower bound value.
        lower: i64,
        /// Upper bound value.
        upper: i64,
    },
    /// Direct contradiction: constant ≤ 0 where constant > 0.
    Contradiction {
        /// The contradictory constant.
        value: i64,
    },
}
/// A single linear term: coefficient * variable name.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinearTerm {
    /// The variable name.
    pub var: Name,
    /// The coefficient.
    pub coeff: i64,
}
impl LinearTerm {
    /// Create a new linear term.
    pub fn new(var: Name, coeff: i64) -> Self {
        LinearTerm { var, coeff }
    }
}
/// A pipeline of TacticOmega analysis passes.
#[allow(dead_code)]
pub struct TacticOmegaPipeline {
    pub passes: Vec<TacticOmegaAnalysisPass>,
    pub name: String,
    pub total_inputs_processed: usize,
}
#[allow(dead_code)]
impl TacticOmegaPipeline {
    pub fn new(name: &str) -> Self {
        TacticOmegaPipeline {
            passes: Vec::new(),
            name: name.to_string(),
            total_inputs_processed: 0,
        }
    }
    pub fn add_pass(&mut self, pass: TacticOmegaAnalysisPass) {
        self.passes.push(pass);
    }
    pub fn run_all(&mut self, input: &str) -> Vec<TacticOmegaResult> {
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
pub struct OmegaExtConfig4200 {
    pub(super) values: std::collections::HashMap<String, OmegaExtConfigVal4200>,
    pub(super) read_only: bool,
    pub(super) name: String,
}
impl OmegaExtConfig4200 {
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
    pub fn set(&mut self, key: &str, value: OmegaExtConfigVal4200) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&OmegaExtConfigVal4200> {
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
        self.set(key, OmegaExtConfigVal4200::Bool(v))
    }
    #[allow(dead_code)]
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, OmegaExtConfigVal4200::Int(v))
    }
    #[allow(dead_code)]
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, OmegaExtConfigVal4200::Str(v.to_string()))
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
pub enum OmegaExtResult4200 {
    /// Operation completed successfully.
    Ok(String),
    /// Operation encountered an error.
    Err(String),
    /// Operation partially completed.
    Partial { done: usize, total: usize },
    /// Operation was skipped.
    Skipped,
}
impl OmegaExtResult4200 {
    #[allow(dead_code)]
    pub fn is_ok(&self) -> bool {
        matches!(self, OmegaExtResult4200::Ok(_))
    }
    #[allow(dead_code)]
    pub fn is_err(&self) -> bool {
        matches!(self, OmegaExtResult4200::Err(_))
    }
    #[allow(dead_code)]
    pub fn is_partial(&self) -> bool {
        matches!(self, OmegaExtResult4200::Partial { .. })
    }
    #[allow(dead_code)]
    pub fn is_skipped(&self) -> bool {
        matches!(self, OmegaExtResult4200::Skipped)
    }
    #[allow(dead_code)]
    pub fn ok_msg(&self) -> Option<&str> {
        if let OmegaExtResult4200::Ok(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn err_msg(&self) -> Option<&str> {
        if let OmegaExtResult4200::Err(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn progress(&self) -> f64 {
        match self {
            OmegaExtResult4200::Ok(_) => 1.0,
            OmegaExtResult4200::Err(_) => 0.0,
            OmegaExtResult4200::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
            OmegaExtResult4200::Skipped => 0.5,
        }
    }
}
/// A configuration store for TacticOmega.
#[allow(dead_code)]
pub struct TacticOmegaConfig {
    pub values: std::collections::HashMap<String, TacticOmegaConfigValue>,
    pub read_only: bool,
}
#[allow(dead_code)]
impl TacticOmegaConfig {
    pub fn new() -> Self {
        TacticOmegaConfig {
            values: std::collections::HashMap::new(),
            read_only: false,
        }
    }
    pub fn set(&mut self, key: &str, value: TacticOmegaConfigValue) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    pub fn get(&self, key: &str) -> Option<&TacticOmegaConfigValue> {
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
        self.set(key, TacticOmegaConfigValue::Bool(v))
    }
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, TacticOmegaConfigValue::Int(v))
    }
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, TacticOmegaConfigValue::Str(v.to_string()))
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
/// Configuration for the Omega solver.
#[derive(Clone, Debug)]
pub struct OmegaConfig {
    /// Maximum number of elimination steps.
    pub max_steps: usize,
    /// Whether to apply preprocessing (GCD reduction, equality elimination).
    pub use_preprocessing: bool,
    /// Whether to apply the dark and gray shadow procedures.
    pub use_dark_gray_shadow: bool,
    /// Whether to allow case splits.
    pub allow_case_splits: bool,
    /// Maximum number of case splits.
    pub max_case_splits: usize,
    /// Whether to normalize Nat constraints (add x >= 0 for Nat vars).
    pub nat_mode: bool,
}
/// An analysis pass for TacticOmega.
#[allow(dead_code)]
pub struct TacticOmegaAnalysisPass {
    pub name: String,
    pub enabled: bool,
    pub results: Vec<TacticOmegaResult>,
    pub total_runs: usize,
}
#[allow(dead_code)]
impl TacticOmegaAnalysisPass {
    pub fn new(name: &str) -> Self {
        TacticOmegaAnalysisPass {
            name: name.to_string(),
            enabled: true,
            results: Vec::new(),
            total_runs: 0,
        }
    }
    pub fn run(&mut self, input: &str) -> TacticOmegaResult {
        self.total_runs += 1;
        let result = if input.is_empty() {
            TacticOmegaResult::Err("empty input".to_string())
        } else {
            TacticOmegaResult::Ok(format!("processed: {}", input))
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
/// Result of the dark shadow test.
enum DarkShadowResult {
    /// The system is unsatisfiable.
    Unsat,
    /// New constraints for the projected system.
    NewConstraints(Vec<LinearConstraint>),
    /// Need to apply gray shadow (case analysis).
    NeedGrayShadow,
}
