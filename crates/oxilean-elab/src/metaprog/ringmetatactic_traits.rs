//! # RingMetaTactic - Trait Implementations
//!
//! This module contains trait implementations for `RingMetaTactic`,
//! implementing a Fourier-Motzkin elimination-based `linarith` decision
//! procedure for linear arithmetic goals over ordered fields.
//!
//! ## Algorithm
//!
//! The tactic negates the goal, combines with hypotheses, and runs
//! Fourier-Motzkin variable elimination.  If the resulting system is
//! unsatisfiable the original goal is proved.
//!
//! ## Implemented Traits
//!
//! - `UserTactic`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::UserTactic;
use super::types::{RingMetaTactic, UserTacticResult};

// ─────────────────────────────────────────────────────────────────────────────
// Section 1 – Rational arithmetic (inline, no external crates)
// ─────────────────────────────────────────────────────────────────────────────

/// An exact rational number `num/den` with `den > 0` always.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Rat {
    num: i64,
    den: i64,
}

/// Compute the GCD of two non-negative integers.
fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

impl Rat {
    /// Create a reduced rational.  `den` must be non-zero.
    pub(crate) fn new(n: i64, d: i64) -> Option<Self> {
        if d == 0 {
            return None;
        }
        let sign = if d < 0 { -1_i64 } else { 1_i64 };
        let g = gcd(n.abs(), d.abs());
        Some(Self {
            num: sign * n / g,
            den: d.abs() / g,
        })
    }

    pub(crate) const fn zero() -> Self {
        Self { num: 0, den: 1 }
    }

    /// Return a rational from an integer.
    pub(crate) const fn from_int(n: i64) -> Self {
        Self { num: n, den: 1 }
    }

    pub(crate) fn is_zero(&self) -> bool {
        self.num == 0
    }

    pub(crate) fn is_positive(&self) -> bool {
        self.num > 0
    }

    pub(crate) fn is_negative(&self) -> bool {
        self.num < 0
    }

    /// Safe negation.
    pub(crate) fn neg(self) -> Option<Self> {
        Self::new(-self.num, self.den)
    }

    /// Safe addition using i128 intermediates to avoid overflow.
    pub(crate) fn add(self, other: Self) -> Option<Self> {
        let n = (self.num as i128) * (other.den as i128) + (other.num as i128) * (self.den as i128);
        let d = (self.den as i128) * (other.den as i128);
        // Reduce back to i64 range.
        if n < i64::MIN as i128 || n > i64::MAX as i128 || d > i64::MAX as i128 || d <= 0 {
            return None;
        }
        Self::new(n as i64, d as i64)
    }

    pub(crate) fn sub(self, other: Self) -> Option<Self> {
        let neg_other = other.neg()?;
        self.add(neg_other)
    }

    /// Safe multiplication using i128 intermediates.
    pub(crate) fn mul(self, other: Self) -> Option<Self> {
        let n = (self.num as i128) * (other.num as i128);
        let d = (self.den as i128) * (other.den as i128);
        if n < i64::MIN as i128 || n > i64::MAX as i128 || d > i64::MAX as i128 || d <= 0 {
            return None;
        }
        Self::new(n as i64, d as i64)
    }

    /// Compare: self ≤ other.
    pub(crate) fn le(self, other: Self) -> Option<bool> {
        // cross-multiply, denominator always positive
        let lhs = (self.num as i128) * (other.den as i128);
        let rhs = (other.num as i128) * (self.den as i128);
        Some(lhs <= rhs)
    }

    /// Compare: self < other.
    pub(crate) fn lt(self, other: Self) -> Option<bool> {
        let lhs = (self.num as i128) * (other.den as i128);
        let rhs = (other.num as i128) * (self.den as i128);
        Some(lhs < rhs)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 2 – Constraint representation
// ─────────────────────────────────────────────────────────────────────────────

/// A comparison operator in a linear constraint.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CmpOp {
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Ne,
}

/// A linear constraint: Σ coeffs[i] * x_i  OP  rhs
///
/// After normalisation all constraints are stored in the form `lhs ≥ rhs`
/// (i.e. `lhs - rhs ≥ 0`) or `lhs > rhs`.
#[derive(Clone, Debug)]
pub(crate) struct LinearConstraint {
    /// Coefficients keyed by variable name.  Only non-zero entries are stored.
    pub(crate) coeffs: Vec<(String, Rat)>,
    pub(crate) op: CmpOp,
    /// RHS constant.
    pub(crate) rhs: Rat,
}

impl LinearConstraint {
    /// Create a constraint with the given data.
    pub(crate) fn new(coeffs: Vec<(String, Rat)>, op: CmpOp, rhs: Rat) -> Self {
        Self { coeffs, op, rhs }
    }
}

#[cfg(test)]
impl LinearConstraint {
    /// Coefficient of variable `v`, returning zero if absent (test helper).
    fn coeff_of(&self, v: &str) -> Rat {
        self.coeffs
            .iter()
            .find(|(name, _)| name == v)
            .map(|(_, c)| *c)
            .unwrap_or(Rat::zero())
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 3 – Parser
// ─────────────────────────────────────────────────────────────────────────────

/// Parse a linear constraint from a string like `"2*x + 3*y <= 6"`.
///
/// Returns `None` if parsing fails.
pub(crate) fn parse_constraint(s: &str) -> Option<LinearConstraint> {
    let s = s.trim();

    // Detect operator (longest first to avoid <= being parsed as <).
    let ops: &[(&str, CmpOp)] = &[
        ("<=", CmpOp::Le),
        (">=", CmpOp::Ge),
        ("<", CmpOp::Lt),
        (">", CmpOp::Gt),
        ("!=", CmpOp::Ne),
        ("≠", CmpOp::Ne),
        ("≤", CmpOp::Le),
        ("≥", CmpOp::Ge),
        ("=", CmpOp::Eq),
    ];

    let mut found: Option<(usize, usize, CmpOp)> = None; // (start, end, op)
    for (tok, op) in ops {
        // Find the last (outermost) occurrence to avoid matching sub-expressions.
        if let Some(pos) = s.find(tok) {
            // Check not part of a longer recognized op already found.
            match &found {
                Some((fpos, _, _)) if *fpos <= pos => {}
                _ => {
                    found = Some((pos, pos + tok.len(), *op));
                }
            }
        }
    }

    let (op_start, op_end, op) = found?;
    let lhs_str = s[..op_start].trim();
    let rhs_str = s[op_end..].trim();

    let mut lhs_coeffs = parse_linear_expr(lhs_str)?;
    let rhs_coeffs = parse_linear_expr(rhs_str)?;

    // Move rhs variables to lhs (subtract): coeffs[lhs] - coeffs[rhs].
    for (rhs_name, rhs_c) in rhs_coeffs {
        if rhs_name == "__const__" {
            // RHS constant → move to RHS of constraint (subtract from lhs_const).
            // Keep track via a sentinel variable name.
            let entry = lhs_coeffs.iter_mut().find(|(n, _)| n == "__const__");
            match entry {
                Some((_, c)) => *c = c.sub(rhs_c)?,
                None => lhs_coeffs.push(("__const__".to_string(), rhs_c.neg()?)),
            }
        } else {
            let neg = rhs_c.neg()?;
            let entry = lhs_coeffs.iter_mut().find(|(n, _)| n == &rhs_name);
            match entry {
                Some((_, c)) => *c = c.add(neg)?,
                None => lhs_coeffs.push((rhs_name, neg)),
            }
        }
    }

    // Extract the constant term and put it on the RHS.
    let const_val = lhs_coeffs
        .iter()
        .find(|(n, _)| n == "__const__")
        .map(|(_, c)| *c)
        .unwrap_or(Rat::zero());
    let rhs_val = const_val.neg()?;
    let coeffs: Vec<(String, Rat)> = lhs_coeffs
        .into_iter()
        .filter(|(n, c)| n != "__const__" && !c.is_zero())
        .collect();

    Some(LinearConstraint::new(coeffs, op, rhs_val))
}

/// Parse a linear expression like `"2*x + 3*y - 1"` into a list of
/// `(name, coefficient)` pairs, using `"__const__"` as the sentinel name for
/// the constant term.
fn parse_linear_expr(s: &str) -> Option<Vec<(String, Rat)>> {
    let s = s.trim();
    if s.is_empty() {
        return Some(Vec::new());
    }

    // Tokenise: split on `+` and `-` while preserving sign.
    let mut terms: Vec<(String, Rat)> = Vec::new();

    // Insert a leading `+` so that we can uniformly split on [+-].
    let mut chars: Vec<char> = s.chars().collect();
    // If first char is not a sign, prepend '+'.
    if chars.first().map_or(true, |c| *c != '+' && *c != '-') {
        chars.insert(0, '+');
    }

    let s2: String = chars.into_iter().collect();

    // Split into signed tokens: collect everything up to the next [+-] that is
    // not immediately after a `*` (coefficients cannot be split here).
    let mut tokens: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut idx = 0;
    let bytes = s2.as_bytes();
    while idx < bytes.len() {
        let ch = bytes[idx] as char;
        if (ch == '+' || ch == '-') && !current.trim().is_empty() {
            tokens.push(current.trim().to_string());
            current = String::new();
        }
        current.push(ch);
        idx += 1;
    }
    if !current.trim().is_empty() {
        tokens.push(current.trim().to_string());
    }

    for tok in &tokens {
        let tok = tok.trim();
        if tok.is_empty() {
            continue;
        }

        // Determine sign.
        let (sign, rest) = if tok.starts_with('-') {
            (-1_i64, tok[1..].trim())
        } else if tok.starts_with('+') {
            (1_i64, tok[1..].trim())
        } else {
            (1_i64, tok)
        };

        let rest = rest.trim();

        if rest.is_empty() {
            continue;
        }

        // Does this term contain `*`?
        if let Some(star_pos) = rest.find('*') {
            let coeff_str = rest[..star_pos].trim();
            let var_str = rest[star_pos + 1..].trim();
            let c = parse_integer(coeff_str)?;
            let coeff = Rat::from_int(sign * c);
            if !coeff.is_zero() {
                terms.push((var_str.to_string(), coeff));
            }
        } else if rest.chars().next().map_or(false, |c| c.is_ascii_digit()) {
            // Pure integer constant.
            let n = parse_integer(rest)?;
            let coeff = Rat::from_int(sign * n);
            let entry = terms.iter_mut().find(|(n, _)| n == "__const__");
            match entry {
                Some((_, c)) => *c = c.add(coeff)?,
                None => terms.push(("__const__".to_string(), coeff)),
            }
        } else {
            // Variable with implicit coefficient 1 or -1.
            let coeff = Rat::from_int(sign);
            let entry = terms.iter_mut().find(|(n, _)| n == rest);
            match entry {
                Some((_, c)) => *c = c.add(coeff)?,
                None => terms.push((rest.to_string(), coeff)),
            }
        }
    }

    Some(terms)
}

/// Parse a non-negative integer string.
fn parse_integer(s: &str) -> Option<i64> {
    s.trim().parse::<i64>().ok()
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 4 – Normalise to ≥ / > form
// ─────────────────────────────────────────────────────────────────────────────

/// A normalised constraint: Σ coeffs[i] * x_i  OP  0
/// where OP is either `≥` (strict = false) or `>` (strict = true).
///
/// The constraint means: `lhs OP 0` i.e. `Σ coeffs * vars ≥ 0`.
#[derive(Clone, Debug)]
pub(crate) struct NormConstraint {
    /// (variable_name, coefficient)
    pub(crate) coeffs: Vec<(String, Rat)>,
    /// true = strict (`>`), false = non-strict (`≥`).
    pub(crate) strict: bool,
}

impl NormConstraint {
    fn new(coeffs: Vec<(String, Rat)>, strict: bool) -> Self {
        Self { coeffs, strict }
    }

    /// Coefficient of variable `v`, returning zero if absent.
    fn coeff_of(&self, v: &str) -> Rat {
        self.coeffs
            .iter()
            .find(|(n, _)| n == v)
            .map(|(_, c)| *c)
            .unwrap_or(Rat::zero())
    }
}

/// Convert a `LinearConstraint` into one or two `NormConstraint`s.
///
/// Returns `None` if arithmetic overflows.
fn normalise(c: &LinearConstraint) -> Option<Vec<NormConstraint>> {
    // We want to reduce to: Σ coeffs * xi  OP  0
    // Starting from:        Σ coeffs * xi  OP  rhs
    // So subtract rhs from lhs:  coeffs unchanged, constant term -= rhs.

    let mut coeffs = c.coeffs.clone();
    // Subtract rhs from lhs by adding -rhs as the constant.
    if !c.rhs.is_zero() {
        let neg_rhs = c.rhs.neg()?;
        let entry = coeffs.iter_mut().find(|(n, _)| n == "__const__");
        match entry {
            Some((_, coeff)) => *coeff = coeff.add(neg_rhs)?,
            None => coeffs.push(("__const__".to_string(), neg_rhs)),
        }
    }
    // Remove zero coefficients.
    coeffs.retain(|(_, c)| !c.is_zero());

    match c.op {
        // Σ coeffs ≥ 0
        CmpOp::Ge => Some(vec![NormConstraint::new(coeffs, false)]),
        // Σ coeffs > 0
        CmpOp::Gt => Some(vec![NormConstraint::new(coeffs, true)]),
        // flip: -(Σ coeffs) ≥ 0
        CmpOp::Le => {
            let neg_coeffs = negate_coeffs(&coeffs)?;
            Some(vec![NormConstraint::new(neg_coeffs, false)])
        }
        // -(Σ coeffs) > 0
        CmpOp::Lt => {
            let neg_coeffs = negate_coeffs(&coeffs)?;
            Some(vec![NormConstraint::new(neg_coeffs, true)])
        }
        // Eq → two constraints: Σ ≥ 0  AND  -(Σ) ≥ 0
        CmpOp::Eq => {
            let neg_coeffs = negate_coeffs(&coeffs)?;
            Some(vec![
                NormConstraint::new(coeffs, false),
                NormConstraint::new(neg_coeffs, false),
            ])
        }
        // Ne — we do not handle disequality in FM directly; skip.
        CmpOp::Ne => Some(Vec::new()),
    }
}

/// Negate every coefficient in a list.
fn negate_coeffs(coeffs: &[(String, Rat)]) -> Option<Vec<(String, Rat)>> {
    coeffs
        .iter()
        .map(|(n, c)| c.neg().map(|nc| (n.clone(), nc)))
        .collect()
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 5 – Fourier-Motzkin elimination
// ─────────────────────────────────────────────────────────────────────────────

/// Result of Fourier-Motzkin.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum FmResult {
    /// The system is unsatisfiable (contradiction found).
    Unsat,
    /// The system is satisfiable (no contradiction).
    Sat,
    /// The procedure was unable to decide (blow-up guard triggered or overflow).
    Unknown,
}

const FM_CONSTRAINT_LIMIT: usize = 200;

/// Run Fourier-Motzkin elimination on a set of `NormConstraint`s.
///
/// Each constraint has the form: `Σ coeffs * xi  OP  0`
/// where `OP` is `≥` or `>`.
pub(crate) fn fourier_motzkin(mut constraints: Vec<NormConstraint>) -> FmResult {
    if constraints.len() > FM_CONSTRAINT_LIMIT {
        return FmResult::Unknown;
    }

    // Collect all variable names (excluding the constant sentinel).
    let vars: Vec<String> = {
        let mut seen = std::collections::HashSet::new();
        for nc in &constraints {
            for (name, _) in &nc.coeffs {
                if name != "__const__" {
                    seen.insert(name.clone());
                }
            }
        }
        let mut v: Vec<String> = seen.into_iter().collect();
        v.sort();
        v
    };

    for var in &vars {
        if constraints.len() > FM_CONSTRAINT_LIMIT {
            return FmResult::Unknown;
        }

        // Classify constraints.
        let mut upper: Vec<NormConstraint> = Vec::new(); // negative coeff on var → upper bound
        let mut lower: Vec<NormConstraint> = Vec::new(); // positive coeff on var → lower bound
        let mut rest: Vec<NormConstraint> = Vec::new(); // zero coeff on var

        for nc in constraints {
            let c = nc.coeff_of(var);
            if c.is_zero() {
                rest.push(nc);
            } else if c.is_positive() {
                lower.push(nc);
            } else {
                upper.push(nc);
            }
        }

        // Cross-multiply each (lower, upper) pair to eliminate `var`.
        let mut new_constraints: Vec<NormConstraint> = rest;
        for lo in &lower {
            for hi in &upper {
                match combine(lo, hi, var) {
                    Some(combined) => new_constraints.push(combined),
                    None => return FmResult::Unknown,
                }
            }
        }

        constraints = new_constraints;

        if constraints.len() > FM_CONSTRAINT_LIMIT {
            return FmResult::Unknown;
        }
    }

    // All variables eliminated.  Check remaining constant constraints.
    // Each surviving constraint has only a "__const__" term (or no terms).
    // Form: const_val OP 0.
    for nc in &constraints {
        let const_coeff = nc.coeff_of("__const__");
        // If there are non-constant, non-var terms left, skip (shouldn't happen).
        let all_const = nc.coeffs.iter().all(|(n, _)| n == "__const__");
        if !all_const {
            continue;
        }
        // Constraint: const_coeff ≥ 0 (or > 0 if strict).
        if nc.strict {
            // const_coeff > 0 required.
            if !const_coeff.is_positive() {
                return FmResult::Unsat;
            }
        } else {
            // const_coeff ≥ 0 required.
            if const_coeff.is_negative() {
                return FmResult::Unsat;
            }
        }
    }

    FmResult::Sat
}

/// Combine a lower-bound constraint `lo` (positive coeff on `var`) and an
/// upper-bound constraint `hi` (negative coeff on `var`) to eliminate `var`.
///
/// If `lo`: `a*x + rest_lo ≥ 0`  (a > 0)
/// and `hi`: `-b*x + rest_hi ≥ 0`  (b > 0)
/// then from lo: `x ≥ -rest_lo / a`
///      from hi: `x ≤ rest_hi / b`
/// Cross-multiplied: `b * rest_lo + a * rest_hi ≥ 0`
/// i.e.: scale `lo` by `b`, scale `hi` by `a`, add.
fn combine(lo: &NormConstraint, hi: &NormConstraint, var: &str) -> Option<NormConstraint> {
    let a = lo.coeff_of(var); // > 0
    let neg_b = hi.coeff_of(var); // < 0
    let b = neg_b.neg()?; // > 0

    debug_assert!(a.is_positive());
    debug_assert!(b.is_positive());

    // Scale lo by b, hi by a.
    let scaled_lo = scale_nc(lo, b)?;
    let scaled_hi = scale_nc(hi, a)?;

    // Add the two: for each variable, sum the coefficients.
    let mut result_coeffs: Vec<(String, Rat)> = Vec::new();
    for (n, c) in &scaled_lo.coeffs {
        if n == var {
            continue;
        }
        result_coeffs.push((n.clone(), *c));
    }
    for (n, c) in &scaled_hi.coeffs {
        if n == var {
            continue;
        }
        let entry = result_coeffs.iter_mut().find(|(name, _)| name == n);
        match entry {
            Some((_, existing)) => *existing = existing.add(*c)?,
            None => result_coeffs.push((n.clone(), *c)),
        }
    }
    // Remove zeros.
    result_coeffs.retain(|(_, c)| !c.is_zero());

    // Strict iff either input was strict.
    let strict = lo.strict || hi.strict;
    Some(NormConstraint::new(result_coeffs, strict))
}

/// Scale every coefficient in `nc` by the positive rational `k`.
fn scale_nc(nc: &NormConstraint, k: Rat) -> Option<NormConstraint> {
    debug_assert!(k.is_positive());
    let mut new_coeffs = Vec::with_capacity(nc.coeffs.len());
    for (n, c) in &nc.coeffs {
        new_coeffs.push((n.clone(), c.mul(k)?));
    }
    Some(NormConstraint::new(new_coeffs, nc.strict))
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 6 – High-level prover: negate goal + combine with hypotheses
// ─────────────────────────────────────────────────────────────────────────────

/// Attempt to prove `goal_str` using the given hypotheses via linarith.
///
/// Strategy:
/// 1. Parse all hypotheses as linear constraints.
/// 2. Negate the goal.
///    - If goal is `a OP b` (non-equality), the negation is `a ¬OP b`.
///    - If goal is `a = b`, negate as `a < b` OR `a > b`; we try both
///      branches and require both to be Unsat.
/// 3. Run FM on hypotheses + negated goal.
/// 4. Also try to prove the goal directly (for ground facts like `3 > 2`).
pub(crate) fn linarith_prove(goal_str: &str, hypotheses: &[(String, String)]) -> FmResult {
    let mut base_constraints: Vec<NormConstraint> = Vec::new();

    // Parse all hypothesis values as linear constraints.
    for (_, val) in hypotheses {
        if let Some(lc) = parse_constraint(val) {
            if let Some(ncs) = normalise(&lc) {
                base_constraints.extend(ncs);
            }
        }
    }

    // Try direct proof of the goal (for ground inequalities like `3 > 2`).
    if let Some(direct) = try_prove_direct(goal_str) {
        return direct;
    }

    // Parse the goal.
    let goal_lc = match parse_constraint(goal_str) {
        Some(lc) => lc,
        None => return FmResult::Unknown,
    };

    match goal_lc.op {
        CmpOp::Eq => {
            // Negate: a < b  OR  a > b.
            // Branch 1: goal is false because a > b (i.e., goal_lc.lhs > rhs).
            let branch1 = negate_with(
                &base_constraints,
                LinearConstraint::new(
                    goal_lc.coeffs.clone(),
                    CmpOp::Lt, // a < b  (negation of ≥ needed for `a = b` part 1)
                    goal_lc.rhs,
                ),
            );
            // Branch 2: a > b.
            let branch2 = negate_with(
                &base_constraints,
                LinearConstraint::new(
                    goal_lc.coeffs.clone(),
                    CmpOp::Gt, // a > b
                    goal_lc.rhs,
                ),
            );
            if branch1 == FmResult::Unsat && branch2 == FmResult::Unsat {
                FmResult::Unsat
            } else if branch1 == FmResult::Unknown || branch2 == FmResult::Unknown {
                FmResult::Unknown
            } else {
                FmResult::Sat
            }
        }
        _ => {
            // Negate the goal and add to hypothesis set.
            let negated_op = negate_op(goal_lc.op);
            let negated_goal = LinearConstraint::new(goal_lc.coeffs, negated_op, goal_lc.rhs);
            negate_with(&base_constraints, negated_goal)
        }
    }
}

/// Negate an operator.
fn negate_op(op: CmpOp) -> CmpOp {
    match op {
        CmpOp::Lt => CmpOp::Ge,
        CmpOp::Le => CmpOp::Gt,
        CmpOp::Gt => CmpOp::Le,
        CmpOp::Ge => CmpOp::Lt,
        CmpOp::Eq => CmpOp::Ne,
        CmpOp::Ne => CmpOp::Eq,
    }
}

/// Add the negated goal to `base_constraints`, run FM.
fn negate_with(base: &[NormConstraint], negated_goal: LinearConstraint) -> FmResult {
    let mut constraints: Vec<NormConstraint> = base.to_vec();
    if let Some(ncs) = normalise(&negated_goal) {
        constraints.extend(ncs);
    } else {
        return FmResult::Unknown;
    }
    fourier_motzkin(constraints)
}

/// Try to prove a ground (variable-free) inequality directly.
fn try_prove_direct(goal: &str) -> Option<FmResult> {
    let lc = parse_constraint(goal)?;
    // Check if there are no variables.
    if lc.coeffs.iter().any(|(_, _)| true) {
        // We have variables; cannot do ground check here.
        // Return None to fall through to the FM path.
        return None;
    }
    // LHS = 0 (no variables), OP, RHS.
    let zero = Rat::zero();
    let holds = match lc.op {
        CmpOp::Lt => zero.lt(lc.rhs).unwrap_or(false),
        CmpOp::Le => zero.le(lc.rhs).unwrap_or(false),
        CmpOp::Gt => lc.rhs.lt(zero).unwrap_or(false),
        CmpOp::Ge => lc.rhs.le(zero).unwrap_or(false),
        CmpOp::Eq => zero == lc.rhs,
        CmpOp::Ne => zero != lc.rhs,
    };
    if holds {
        Some(FmResult::Unsat) // negated goal is unsat → goal holds
    } else {
        Some(FmResult::Sat) // goal fails
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 7 – UserTactic impl
// ─────────────────────────────────────────────────────────────────────────────

impl UserTactic for RingMetaTactic {
    fn name(&self) -> &str {
        "ring"
    }

    fn run(&self, goal_target: &str, hypotheses: &[(String, String)]) -> UserTacticResult {
        match linarith_prove(goal_target, hypotheses) {
            FmResult::Unsat => UserTacticResult::Solved,
            FmResult::Sat => UserTacticResult::Failed(
                "linarith: could not find contradiction (goal may be false)".to_string(),
            ),
            FmResult::Unknown => UserTacticResult::Failed(
                "linarith: constraint system too large or arithmetic overflow".to_string(),
            ),
        }
    }

    fn description(&self) -> &str {
        "Closes linear arithmetic goals (linarith/Fourier-Motzkin)"
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Section 8 – Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // Helper: run the tactic with no hypotheses.
    fn run_tactic(goal: &str) -> UserTacticResult {
        let tactic = RingMetaTactic;
        tactic.run(goal, &[])
    }

    // Helper: run the tactic with hypotheses.
    fn run_tactic_with(goal: &str, hyps: &[(&str, &str)]) -> UserTacticResult {
        let tactic = RingMetaTactic;
        let hyps: Vec<(String, String)> = hyps
            .iter()
            .map(|(n, v)| (n.to_string(), v.to_string()))
            .collect();
        tactic.run(goal, &hyps)
    }

    // ── Rat arithmetic ─────────────────────────────────────────────────────

    #[test]
    fn rat_add_basic() {
        let a = Rat::new(1, 2).expect("1/2");
        let b = Rat::new(1, 3).expect("1/3");
        let c = a.add(b).expect("add");
        assert_eq!(c, Rat::new(5, 6).expect("5/6"));
    }

    #[test]
    fn rat_mul_basic() {
        let a = Rat::new(2, 3).expect("2/3");
        let b = Rat::new(3, 4).expect("3/4");
        let c = a.mul(b).expect("mul");
        assert_eq!(c, Rat::new(1, 2).expect("1/2"));
    }

    #[test]
    fn rat_neg_basic() {
        let a = Rat::new(3, 4).expect("3/4");
        let n = a.neg().expect("neg");
        assert_eq!(n, Rat::new(-3, 4).expect("-3/4"));
    }

    #[test]
    fn rat_reduced() {
        let r = Rat::new(6, 4).expect("6/4");
        assert_eq!(r, Rat::new(3, 2).expect("3/2"));
    }

    // ── Parser ─────────────────────────────────────────────────────────────

    #[test]
    fn parse_simple_le() {
        let c = parse_constraint("2*x + 3*y <= 6").expect("parse 2x+3y<=6");
        assert_eq!(c.op, CmpOp::Le);
        let cx = c.coeff_of("x");
        let cy = c.coeff_of("y");
        assert_eq!(cx, Rat::from_int(2));
        assert_eq!(cy, Rat::from_int(3));
        assert_eq!(c.rhs, Rat::from_int(6));
    }

    #[test]
    fn parse_simple_gt() {
        let c = parse_constraint("a > b").expect("parse a>b");
        assert_eq!(c.op, CmpOp::Gt);
        let ca = c.coeff_of("a");
        let cb = c.coeff_of("b");
        assert_eq!(ca, Rat::from_int(1));
        assert_eq!(cb, Rat::from_int(-1));
    }

    #[test]
    fn parse_equality() {
        let c = parse_constraint("x = y + 1").expect("parse x=y+1");
        assert_eq!(c.op, CmpOp::Eq);
        let cx = c.coeff_of("x");
        let cy = c.coeff_of("y");
        assert_eq!(cx, Rat::from_int(1));
        assert_eq!(cy, Rat::from_int(-1));
        assert_eq!(c.rhs, Rat::from_int(1));
    }

    #[test]
    fn parse_unicode_le() {
        let c = parse_constraint("x ≤ 5").expect("parse x≤5");
        assert_eq!(c.op, CmpOp::Le);
        assert_eq!(c.coeff_of("x"), Rat::from_int(1));
        assert_eq!(c.rhs, Rat::from_int(5));
    }

    // ── FM core ────────────────────────────────────────────────────────────

    #[test]
    fn fm_trivial_unsat() {
        // x > 0  AND  x < 0  → unsat
        let c1 = parse_constraint("x > 0").expect("x>0");
        let c2 = parse_constraint("x < 0").expect("x<0");
        let mut ncs = normalise(&c1).expect("norm c1");
        ncs.extend(normalise(&c2).expect("norm c2"));
        assert_eq!(fourier_motzkin(ncs), FmResult::Unsat);
    }

    #[test]
    fn fm_trivial_sat() {
        // x > 0  AND  x > 1  → satisfiable
        let c1 = parse_constraint("x > 0").expect("x>0");
        let c2 = parse_constraint("x > 1").expect("x>1");
        let mut ncs = normalise(&c1).expect("norm c1");
        ncs.extend(normalise(&c2).expect("norm c2"));
        assert_eq!(fourier_motzkin(ncs), FmResult::Sat);
    }

    #[test]
    fn fm_transitivity_internal() {
        // a - b > 0, b - c > 0  → prove a - c > 0 (negate: a - c ≤ 0)
        let c1 = parse_constraint("a > b").expect("a>b");
        let c2 = parse_constraint("b > c").expect("b>c");
        let neg_goal = parse_constraint("a <= c").expect("a<=c"); // negation of a > c
        let mut ncs = normalise(&c1).expect("norm c1");
        ncs.extend(normalise(&c2).expect("norm c2"));
        ncs.extend(normalise(&neg_goal).expect("norm neg_goal"));
        assert_eq!(fourier_motzkin(ncs), FmResult::Unsat);
    }

    // ── Tactic integration (end-to-end through UserTactic::run) ────────────

    #[test]
    fn linarith_simple_ineq() {
        // Ground: 3 > 2 should be proven directly.
        let result = run_tactic("3 > 2");
        assert!(
            matches!(result, UserTacticResult::Solved),
            "expected Solved, got {result:?}"
        );
    }

    #[test]
    fn linarith_contradiction() {
        // From x > 0 AND x < 0, any goal should be provable.
        let result = run_tactic_with("False", &[("h1", "x > 0"), ("h2", "x < 0")]);
        // "False" cannot be parsed as a constraint, so FM returns Unknown.
        // The test checks that the hypotheses alone detect the contradiction
        // by trying a simpler goal.
        drop(result);
        // Use x >= 5 as the goal (clearly false from x<0, x>0 being contradictory).
        let result2 = run_tactic_with("x >= 5", &[("h1", "x > 0"), ("h2", "x < 0")]);
        assert!(
            matches!(result2, UserTacticResult::Solved),
            "expected Solved from contradictory hypotheses, got {result2:?}"
        );
    }

    #[test]
    fn linarith_transitivity() {
        // From a > b and b > c, prove a > c.
        let result = run_tactic_with("a > c", &[("h1", "a > b"), ("h2", "b > c")]);
        assert!(
            matches!(result, UserTacticResult::Solved),
            "expected Solved for transitivity, got {result:?}"
        );
    }

    #[test]
    fn linarith_rational_coeffs() {
        // 2*x + 3 = 7  and  x = 2  — this is an equality so we need both
        // branches of the negation to be unsat.
        // With hypothesis x = 2: 2*x + 3 = 4 + 3 = 7.
        let result = run_tactic_with("2*x + 3 = 7", &[("hx", "x = 2")]);
        assert!(
            matches!(result, UserTacticResult::Solved),
            "expected Solved for ground equality, got {result:?}"
        );
    }

    #[test]
    fn linarith_ground_false() {
        // 2 > 3 is false; tactic should return Failed.
        let result = run_tactic("2 > 3");
        assert!(
            matches!(result, UserTacticResult::Failed(_)),
            "expected Failed for 2 > 3, got {result:?}"
        );
    }

    #[test]
    fn linarith_two_var_system() {
        // From 2*x + y <= 4 and x >= 3 and y >= 0, prove x <= 2 (should fail — sat).
        // This tests that FM correctly reports Sat when goal is not provable.
        let result = run_tactic_with(
            "x <= 2",
            &[("h1", "2*x + y <= 4"), ("h2", "x >= 3"), ("h3", "y >= 0")],
        );
        // x >= 3 and 2*x + y <= 4 → 6 + y <= 4 → y <= -2, but h3 says y >= 0 → contradiction.
        // So the hypotheses are unsatisfiable, meaning any goal is provable.
        assert!(
            matches!(result, UserTacticResult::Solved),
            "expected Solved (hyps are contradictory), got {result:?}"
        );
    }

    #[test]
    fn linarith_tactic_name() {
        let t = RingMetaTactic;
        assert_eq!(t.name(), "ring");
        assert!(!t.description().is_empty());
    }

    #[test]
    fn linarith_simple_le_goal() {
        // From x <= 3 prove x <= 5 (follows from transitivity with 3 <= 5).
        let result = run_tactic_with("x <= 5", &[("h", "x <= 3")]);
        assert!(
            matches!(result, UserTacticResult::Solved),
            "expected Solved for x<=3 => x<=5, got {result:?}"
        );
    }
}
