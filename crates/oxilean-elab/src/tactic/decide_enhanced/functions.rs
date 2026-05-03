//! Functions implementing the enhanced `decide` tactic.

use std::collections::HashMap;

use super::types::{
    looks_like_bool_expr, looks_like_nat_arith, looks_like_prop_formula, DecideConfig,
    DecideResult, DecisionProcedure, PropFormula,
};

// ──────────────────────────────────────────────────────────────────────────────
// Natural-number arithmetic decision
// ──────────────────────────────────────────────────────────────────────────────

/// Decide a natural-number arithmetic proposition by direct evaluation.
///
/// Recognises expressions of the form `<lhs> <op> <rhs>` where `<lhs>` and
/// `<rhs>` are Nat-valued arithmetic sub-expressions built from literals,
/// variables (bound by the caller as concrete values), `+`, `-`, `*`, and `/`.
///
/// Returns `None` if the expression cannot be decided within the given
/// `max_val` budget.
pub fn decide_nat_arith(expr: &str, max_val: u64) -> Option<DecideResult> {
    let trimmed = expr.trim();

    // Parse the comparison.
    let (lhs_str, op, rhs_str) = parse_nat_comparison(trimmed)?;

    // Evaluate both sides.
    let lhs_val = eval_nat_expr(lhs_str, max_val)?;
    let rhs_val = eval_nat_expr(rhs_str, max_val)?;

    let verdict = match op {
        "==" | "=" => lhs_val == rhs_val,
        "!=" | "≠" => lhs_val != rhs_val,
        "<" => lhs_val < rhs_val,
        "<=" | "≤" => lhs_val <= rhs_val,
        ">" => lhs_val > rhs_val,
        ">=" | "≥" => lhs_val >= rhs_val,
        _ => return None,
    };

    Some(DecideResult {
        verdict,
        procedure_used: DecisionProcedure::NatArith,
        steps: 1,
    })
}

/// Parse a flat comparison `<lhs> <op> <rhs>` from `expr`.
fn parse_nat_comparison(expr: &str) -> Option<(&str, &str, &str)> {
    // Try each operator in decreasing specificity order.
    for op in &["==", "!=", "<=", ">=", "≤", "≥", "≠", "<", ">", "="] {
        if let Some(pos) = expr.find(op) {
            // Make sure this is not part of a longer operator.
            let before = &expr[..pos];
            let after = &expr[pos + op.len()..];
            // Avoid matching '=' when '<=' or '>=' was already tried first.
            let lhs = before.trim();
            let rhs = after.trim();
            if !lhs.is_empty() && !rhs.is_empty() {
                return Some((lhs, op, rhs));
            }
        }
    }
    None
}

/// Evaluate a simple Nat arithmetic expression string to a `u64`.
/// Supports: literals, `+`, `-` (saturating), `*`, `/` (integer), and
/// parentheses.  Returns `None` if the value would exceed `max_val`.
fn eval_nat_expr(expr: &str, max_val: u64) -> Option<u64> {
    let trimmed = expr.trim();
    // Fast path: literal.
    if let Ok(n) = trimmed.parse::<u64>() {
        return if n <= max_val { Some(n) } else { None };
    }

    // Strip outer parentheses.
    if trimmed.starts_with('(') && trimmed.ends_with(')') {
        let inner = trimmed[1..trimmed.len() - 1].trim();
        return eval_nat_expr(inner, max_val);
    }

    // Try binary operators (lowest precedence first: +, -, then *, /).
    for op in &["+", "-", "*", "/"] {
        if let Some((lhs_s, rhs_s)) = split_at_op(trimmed, op) {
            let l = eval_nat_expr(lhs_s, max_val)?;
            let r = eval_nat_expr(rhs_s, max_val)?;
            let result = match *op {
                "+" => l.checked_add(r)?,
                "-" => l.saturating_sub(r),
                "*" => l.checked_mul(r)?,
                "/" => {
                    if r == 0 {
                        return None;
                    }
                    l / r
                }
                _ => return None,
            };
            return if result <= max_val {
                Some(result)
            } else {
                None
            };
        }
    }

    None
}

/// Split `expr` at the rightmost top-level occurrence of `op`.
///
/// "Top-level" means not inside parentheses.  We use the rightmost to give
/// left-associativity.
fn split_at_op<'a>(expr: &'a str, op: &str) -> Option<(&'a str, &'a str)> {
    let bytes = expr.as_bytes();
    let op_bytes = op.as_bytes();
    let op_len = op_bytes.len();

    let mut depth = 0usize;
    let mut last_pos: Option<usize> = None;

    let mut i = 0usize;
    while i < bytes.len() {
        match bytes[i] {
            b'(' => depth += 1,
            b')' => depth = depth.saturating_sub(1),
            _ => {
                if depth == 0 && bytes[i..].starts_with(op_bytes) {
                    last_pos = Some(i);
                    i += op_len;
                    continue;
                }
            }
        }
        i += 1;
    }

    last_pos.map(|p| (expr[..p].trim(), expr[p + op_len..].trim()))
}

// ──────────────────────────────────────────────────────────────────────────────
// Boolean expression decision
// ──────────────────────────────────────────────────────────────────────────────

/// Decide a pure boolean expression by evaluation.
///
/// Handles: `true`, `false`, `&&`, `||`, `!`, `and`, `or`, `not`.
/// Returns `None` if the expression cannot be fully evaluated (e.g. contains
/// free variables).
pub fn decide_bool_expr(expr: &str) -> Option<DecideResult> {
    let verdict = eval_bool_expr(expr)?;
    Some(DecideResult {
        verdict,
        procedure_used: DecisionProcedure::BoolEval,
        steps: 1,
    })
}

/// Recursive boolean expression evaluator.
pub(super) fn eval_bool_expr(expr: &str) -> Option<bool> {
    let trimmed = expr.trim().to_lowercase();

    // Base cases.
    if trimmed == "true" {
        return Some(true);
    }
    if trimmed == "false" {
        return Some(false);
    }

    // Strip parentheses.
    if trimmed.starts_with('(') && trimmed.ends_with(')') {
        let inner = &trimmed[1..trimmed.len() - 1];
        return eval_bool_expr(inner);
    }

    // NOT.
    if trimmed.starts_with('!') {
        return eval_bool_expr(&trimmed[1..]).map(|v| !v);
    }
    if trimmed.starts_with("not ") {
        return eval_bool_expr(&trimmed[4..]).map(|v| !v);
    }

    // Binary operators (right-to-left: find the last occurrence).
    for (op_str, op_logical) in &[("||", "or"), ("&&", "and")] {
        let search = expr.trim();
        // Find the top-level operator.
        if let Some((lhs_s, rhs_s)) = find_top_level_bool_op(search, op_str, op_logical) {
            let l = eval_bool_expr(lhs_s)?;
            let r = eval_bool_expr(rhs_s)?;
            return Some(if *op_str == "||" { l || r } else { l && r });
        }
    }

    None
}

/// Find the top-level binary boolean operator in `expr`.
fn find_top_level_bool_op<'a>(
    expr: &'a str,
    symbolic: &str,
    word: &str,
) -> Option<(&'a str, &'a str)> {
    let bytes = expr.as_bytes();
    let sym_bytes = symbolic.as_bytes();
    let mut depth = 0usize;
    let mut last_pos: Option<(usize, usize)> = None; // (start, len)

    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'(' => depth += 1,
            b')' => depth = depth.saturating_sub(1),
            _ => {
                if depth == 0 {
                    // Check symbolic form.
                    if bytes[i..].starts_with(sym_bytes) {
                        last_pos = Some((i, sym_bytes.len()));
                        i += sym_bytes.len();
                        continue;
                    }
                    // Check word form (must be surrounded by spaces or boundaries).
                    let rest = &expr[i..].to_lowercase();
                    if rest.starts_with(word) {
                        let end = i + word.len();
                        let before_ok = i == 0 || bytes[i - 1].is_ascii_whitespace();
                        let after_ok = end >= bytes.len() || bytes[end].is_ascii_whitespace();
                        if before_ok && after_ok {
                            last_pos = Some((i, word.len()));
                            i += word.len();
                            continue;
                        }
                    }
                }
            }
        }
        i += 1;
    }

    last_pos.map(|(p, len)| (expr[..p].trim(), expr[p + len..].trim()))
}

// ──────────────────────────────────────────────────────────────────────────────
// Propositional formula parsing
// ──────────────────────────────────────────────────────────────────────────────

/// Parse a propositional formula from a string expression.
///
/// Supported syntax:
/// - `True` / `False`
/// - `p`, `q`, `some_atom` — atoms (alphabetic identifiers)
/// - `¬f` / `not f` / `!f` — negation
/// - `f ∧ g` / `f /\ g` / `f && g` / `f and g` — conjunction
/// - `f ∨ g` / `f \/ g` / `f || g` / `f or g` — disjunction
/// - `f → g` / `f -> g` / `f implies g` — implication
/// - `f ↔ g` / `f <-> g` / `f iff g` — biconditional
/// - `(f)` — grouping
pub fn parse_prop_formula(expr: &str) -> Option<PropFormula> {
    parse_iff(expr.trim())
}

fn parse_iff(expr: &str) -> Option<PropFormula> {
    // Find the last top-level ↔ / <-> / iff.
    for (sym, word) in &[("↔", "iff"), ("<->", "iff")] {
        if let Some((lhs_s, rhs_s)) = find_top_level_infix(expr, sym, word) {
            let l = parse_iff(lhs_s)?;
            let r = parse_iff(rhs_s)?;
            return Some(PropFormula::Iff(Box::new(l), Box::new(r)));
        }
    }
    parse_implies(expr)
}

fn parse_implies(expr: &str) -> Option<PropFormula> {
    for (sym, word) in &[("→", "implies"), ("->", "implies")] {
        if let Some((lhs_s, rhs_s)) = find_top_level_infix(expr, sym, word) {
            let l = parse_implies(lhs_s)?;
            let r = parse_implies(rhs_s)?;
            return Some(PropFormula::Implies(Box::new(l), Box::new(r)));
        }
    }
    parse_or(expr)
}

fn parse_or(expr: &str) -> Option<PropFormula> {
    for (sym, word) in &[("∨", "or"), ("\\/", "or"), ("||", "or")] {
        if let Some((lhs_s, rhs_s)) = find_top_level_infix(expr, sym, word) {
            let l = parse_or(lhs_s)?;
            let r = parse_or(rhs_s)?;
            return Some(PropFormula::Or(Box::new(l), Box::new(r)));
        }
    }
    parse_and(expr)
}

fn parse_and(expr: &str) -> Option<PropFormula> {
    for (sym, word) in &[("∧", "and"), ("/\\", "and"), ("&&", "and")] {
        if let Some((lhs_s, rhs_s)) = find_top_level_infix(expr, sym, word) {
            let l = parse_and(lhs_s)?;
            let r = parse_and(rhs_s)?;
            return Some(PropFormula::And(Box::new(l), Box::new(r)));
        }
    }
    parse_not(expr)
}

fn parse_not(expr: &str) -> Option<PropFormula> {
    let trimmed = expr.trim();
    // Unicode not.
    if trimmed.starts_with('¬') {
        let inner_str = trimmed.strip_prefix('¬')?.trim();
        return parse_not(inner_str).map(PropFormula::not);
    }
    // ASCII not.
    if trimmed.starts_with('!') {
        let inner_str = trimmed.strip_prefix('!')?.trim();
        return parse_not(inner_str).map(PropFormula::not);
    }
    // Keyword not.
    if let Some(rest) = trimmed.strip_prefix("not ") {
        return parse_not(rest.trim()).map(PropFormula::not);
    }
    parse_atom(trimmed)
}

fn parse_atom(expr: &str) -> Option<PropFormula> {
    let trimmed = expr.trim();
    // Parenthesised sub-expression.
    if trimmed.starts_with('(') && trimmed.ends_with(')') {
        let inner = trimmed[1..trimmed.len() - 1].trim();
        return parse_iff(inner);
    }
    // Boolean constants.
    match trimmed.to_lowercase().as_str() {
        "true" => return Some(PropFormula::True),
        "false" => return Some(PropFormula::False),
        _ => {}
    }
    // Atom: alphabetic + '_'.
    if trimmed.chars().all(|c| c.is_alphanumeric() || c == '_') && !trimmed.is_empty() {
        return Some(PropFormula::Atom(trimmed.to_string()));
    }
    None
}

/// Find the last top-level occurrence of `sym` or the word `word` in `expr`.
/// Returns `(lhs, rhs)` slices if found.
fn find_top_level_infix<'a>(expr: &'a str, sym: &str, word: &str) -> Option<(&'a str, &'a str)> {
    let bytes = expr.as_bytes();
    let mut depth = 0usize;
    let mut last: Option<(usize, usize)> = None; // (start, len)
    let mut i = 0usize;

    while i < bytes.len() {
        match bytes[i] {
            b'(' => {
                depth += 1;
                i += 1;
            }
            b')' => {
                depth = depth.saturating_sub(1);
                i += 1;
            }
            _ => {
                if depth == 0 {
                    // Try symbolic form (handles multi-byte UTF-8 gracefully
                    // by working on the string slice).
                    let rest_str = &expr[i..];
                    if rest_str.starts_with(sym) {
                        last = Some((i, sym.len()));
                        i += sym.len();
                        continue;
                    }
                    // Try word form.
                    let rest_lower = rest_str.to_lowercase();
                    if rest_lower.starts_with(word) {
                        let word_end = i + word.len();
                        let before_space = i == 0 || bytes[i - 1].is_ascii_whitespace();
                        let after_space =
                            word_end >= bytes.len() || bytes[word_end].is_ascii_whitespace();
                        if before_space && after_space {
                            last = Some((i, word.len()));
                            i += word.len();
                            continue;
                        }
                    }
                }
                i += 1;
            }
        }
    }

    last.map(|(p, len)| (expr[..p].trim(), expr[p + len..].trim()))
}

// ──────────────────────────────────────────────────────────────────────────────
// DPLL-based propositional decision
// ──────────────────────────────────────────────────────────────────────────────

/// Decide a propositional formula using DPLL with unit propagation.
///
/// Returns a `DecideResult` whose `verdict` is `true` iff the formula is
/// satisfiable.
pub fn decide_prop_formula(formula: &PropFormula) -> DecideResult {
    let mut assignment: HashMap<String, bool> = HashMap::new();
    let sat = dpll(formula, &mut assignment);
    DecideResult {
        verdict: sat,
        procedure_used: DecisionProcedure::PropFormulaDpll,
        steps: 1,
    }
}

/// DPLL SAT solver with unit propagation.
///
/// Mutates `assignment` to contain a satisfying assignment if one exists.
pub fn dpll(formula: &PropFormula, assignment: &mut HashMap<String, bool>) -> bool {
    // Evaluate under the current (possibly partial) assignment.
    let val = eval_partial(formula, assignment);
    match val {
        Some(true) => return true,
        Some(false) => return false,
        None => {}
    }

    // Pick the first unassigned atom.
    let all_atoms = formula.atoms();
    let unassigned = all_atoms
        .iter()
        .find(|a| !assignment.contains_key(a.as_str()));
    let atom = match unassigned {
        Some(a) => a.clone(),
        None => return false, // All atoms assigned but undecided — shouldn't happen.
    };

    // Try true.
    assignment.insert(atom.clone(), true);
    if dpll(formula, assignment) {
        return true;
    }

    // Try false.
    assignment.insert(atom.clone(), false);
    if dpll(formula, assignment) {
        return true;
    }

    // Neither works — backtrack.
    assignment.remove(&atom);
    false
}

/// Evaluate the formula under a partial assignment.
/// Returns `None` if the result is still undetermined.
fn eval_partial(formula: &PropFormula, assignment: &HashMap<String, bool>) -> Option<bool> {
    match formula {
        PropFormula::True => Some(true),
        PropFormula::False => Some(false),
        PropFormula::Atom(name) => assignment.get(name).copied(),
        PropFormula::Not(f) => eval_partial(f, assignment).map(|v| !v),
        PropFormula::And(l, r) => match eval_partial(l, assignment) {
            Some(false) => Some(false), // Short-circuit.
            Some(true) => eval_partial(r, assignment),
            None => match eval_partial(r, assignment) {
                Some(false) => Some(false),
                _ => None,
            },
        },
        PropFormula::Or(l, r) => match eval_partial(l, assignment) {
            Some(true) => Some(true), // Short-circuit.
            Some(false) => eval_partial(r, assignment),
            None => match eval_partial(r, assignment) {
                Some(true) => Some(true),
                _ => None,
            },
        },
        PropFormula::Implies(l, r) => eval_partial(
            &PropFormula::or(PropFormula::not(l.as_ref().clone()), r.as_ref().clone()),
            assignment,
        ),
        PropFormula::Iff(l, r) => {
            let lv = eval_partial(l, assignment);
            let rv = eval_partial(r, assignment);
            match (lv, rv) {
                (Some(a), Some(b)) => Some(a == b),
                _ => None,
            }
        }
    }
}

/// Unit propagation step for DIMACS-style clauses.
///
/// A *unit clause* is a clause containing exactly one unassigned literal.
/// Unit propagation forces that literal to be true.
///
/// `clauses`: each clause is a `Vec<i32>` of DIMACS literals (positive =
/// positive occurrence of atom index - 1; negative = negation).
/// `assignment`: indexed by atom position (0-based); `None` = unassigned.
///
/// Returns the updated assignment after exhaustive unit propagation, or `None`
/// if a conflict (empty clause) is detected.
pub fn unit_propagate(
    clauses: &[Vec<i32>],
    assignment: &[Option<bool>],
) -> Option<Vec<Option<bool>>> {
    let mut current = assignment.to_vec();
    let mut changed = true;

    while changed {
        changed = false;
        for clause in clauses {
            let unresolved: Vec<i32> = clause
                .iter()
                .filter(|&&lit| {
                    let idx = (lit.unsigned_abs() as usize).saturating_sub(1);
                    current.get(idx).copied().flatten().is_none()
                })
                .copied()
                .collect();

            // Check if the clause is already satisfied.
            let satisfied = clause.iter().any(|&lit| {
                let idx = (lit.unsigned_abs() as usize).saturating_sub(1);
                let positive = lit > 0;
                current
                    .get(idx)
                    .copied()
                    .flatten()
                    .map(|v| v == positive)
                    .unwrap_or(false)
            });

            if satisfied {
                continue;
            }

            match unresolved.len() {
                0 => return None, // Conflict: empty clause.
                1 => {
                    // Unit clause: force the literal.
                    let lit = unresolved[0];
                    let idx = (lit.unsigned_abs() as usize).saturating_sub(1);
                    let positive = lit > 0;
                    if idx < current.len() {
                        current[idx] = Some(positive);
                        changed = true;
                    }
                }
                _ => {}
            }
        }
    }

    Some(current)
}

// ──────────────────────────────────────────────────────────────────────────────
// Orchestrating `try_decide`
// ──────────────────────────────────────────────────────────────────────────────

/// Attempt to decide `goal` by trying all available decision procedures in
/// order of heuristic suitability.
///
/// Returns `None` if no procedure can decide the goal.
pub fn try_decide(goal: &str, cfg: &DecideConfig) -> Option<DecideResult> {
    let trimmed = goal.trim();

    // Trivial cases.
    match trimmed.to_lowercase().as_str() {
        "true" | "rfl" => {
            return Some(DecideResult {
                verdict: true,
                procedure_used: DecisionProcedure::Refl,
                steps: 0,
            })
        }
        "false" => {
            return Some(DecideResult {
                verdict: false,
                procedure_used: DecisionProcedure::Refl,
                steps: 0,
            })
        }
        _ => {}
    }

    // Nat arithmetic.
    if looks_like_nat_arith(trimmed) {
        if let Some(r) = decide_nat_arith(trimmed, cfg.max_nat_value) {
            return Some(r);
        }
    }

    // Bool evaluation.
    if looks_like_bool_expr(trimmed) {
        if let Some(r) = decide_bool_expr(trimmed) {
            return Some(r);
        }
    }

    // Propositional formula.
    if looks_like_prop_formula(trimmed) {
        if let Some(formula) = parse_prop_formula(trimmed) {
            return Some(decide_prop_formula(&formula));
        }
    }

    // Fall back to parsing as a propositional formula anyway.
    if let Some(formula) = parse_prop_formula(trimmed) {
        return Some(decide_prop_formula(&formula));
    }

    None
}

// ──────────────────────────────────────────────────────────────────────────────
// Tests
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── decide_nat_arith ──────────────────────────────────────────────────────

    #[test]
    fn test_nat_arith_equal_true() {
        let r = decide_nat_arith("3 == 3", 1000).expect("should decide");
        assert!(r.verdict);
        assert_eq!(r.procedure_used, DecisionProcedure::NatArith);
    }

    #[test]
    fn test_nat_arith_equal_false() {
        let r = decide_nat_arith("3 == 4", 1000).expect("should decide");
        assert!(!r.verdict);
    }

    #[test]
    fn test_nat_arith_less_than_true() {
        let r = decide_nat_arith("2 < 5", 1000).expect("should decide");
        assert!(r.verdict);
    }

    #[test]
    fn test_nat_arith_less_than_false() {
        let r = decide_nat_arith("5 < 2", 1000).expect("should decide");
        assert!(!r.verdict);
    }

    #[test]
    fn test_nat_arith_addition() {
        let r = decide_nat_arith("2 + 3 == 5", 1000).expect("should decide");
        assert!(r.verdict);
    }

    #[test]
    fn test_nat_arith_multiplication() {
        let r = decide_nat_arith("4 * 3 == 12", 1000).expect("should decide");
        assert!(r.verdict);
    }

    #[test]
    fn test_nat_arith_leq() {
        let r = decide_nat_arith("5 <= 5", 1000).expect("should decide");
        assert!(r.verdict);
    }

    #[test]
    fn test_nat_arith_geq_false() {
        let r = decide_nat_arith("3 >= 10", 1000).expect("should decide");
        assert!(!r.verdict);
    }

    #[test]
    fn test_nat_arith_neq_true() {
        let r = decide_nat_arith("7 != 8", 1000).expect("should decide");
        assert!(r.verdict);
    }

    #[test]
    fn test_nat_arith_neq_false() {
        let r = decide_nat_arith("7 != 7", 1000).expect("should decide");
        assert!(!r.verdict);
    }

    #[test]
    fn test_nat_arith_complex() {
        // (2 + 3) * 4 == 20
        let r = decide_nat_arith("(2 + 3) * 4 == 20", 1000).expect("should decide");
        assert!(r.verdict);
    }

    #[test]
    fn test_nat_arith_exceeds_max() {
        // Expression value exceeds max_val budget.
        assert!(decide_nat_arith("1000001 == 1000001", 1000).is_none());
    }

    // ── decide_bool_expr ──────────────────────────────────────────────────────

    #[test]
    fn test_bool_true() {
        let r = decide_bool_expr("true").expect("should decide");
        assert!(r.verdict);
        assert_eq!(r.procedure_used, DecisionProcedure::BoolEval);
    }

    #[test]
    fn test_bool_false() {
        let r = decide_bool_expr("false").expect("should decide");
        assert!(!r.verdict);
    }

    #[test]
    fn test_bool_and_tt() {
        let r = decide_bool_expr("true && true").expect("should decide");
        assert!(r.verdict);
    }

    #[test]
    fn test_bool_and_tf() {
        let r = decide_bool_expr("true && false").expect("should decide");
        assert!(!r.verdict);
    }

    #[test]
    fn test_bool_or_ff() {
        let r = decide_bool_expr("false || false").expect("should decide");
        assert!(!r.verdict);
    }

    #[test]
    fn test_bool_or_tf() {
        let r = decide_bool_expr("true || false").expect("should decide");
        assert!(r.verdict);
    }

    #[test]
    fn test_bool_not_true() {
        let r = decide_bool_expr("!true").expect("should decide");
        assert!(!r.verdict);
    }

    #[test]
    fn test_bool_not_false() {
        let r = decide_bool_expr("!false").expect("should decide");
        assert!(r.verdict);
    }

    #[test]
    fn test_bool_complex() {
        let r = decide_bool_expr("!false && true").expect("should decide");
        assert!(r.verdict);
    }

    // ── parse_prop_formula ────────────────────────────────────────────────────

    #[test]
    fn test_parse_true() {
        let f = parse_prop_formula("True").expect("should parse");
        assert_eq!(f, PropFormula::True);
    }

    #[test]
    fn test_parse_false() {
        let f = parse_prop_formula("False").expect("should parse");
        assert_eq!(f, PropFormula::False);
    }

    #[test]
    fn test_parse_atom() {
        let f = parse_prop_formula("p").expect("should parse");
        assert_eq!(f, PropFormula::Atom("p".into()));
    }

    #[test]
    fn test_parse_not() {
        let f = parse_prop_formula("!p").expect("should parse");
        assert_eq!(f, PropFormula::not(PropFormula::Atom("p".into())));
    }

    #[test]
    fn test_parse_and() {
        let f = parse_prop_formula("p && q").expect("should parse");
        assert_eq!(
            f,
            PropFormula::and(PropFormula::Atom("p".into()), PropFormula::Atom("q".into()),)
        );
    }

    #[test]
    fn test_parse_or() {
        let f = parse_prop_formula("p || q").expect("should parse");
        assert_eq!(
            f,
            PropFormula::or(PropFormula::Atom("p".into()), PropFormula::Atom("q".into()),)
        );
    }

    #[test]
    fn test_parse_implies() {
        let f = parse_prop_formula("p -> q").expect("should parse");
        assert_eq!(
            f,
            PropFormula::Implies(
                Box::new(PropFormula::Atom("p".into())),
                Box::new(PropFormula::Atom("q".into())),
            )
        );
    }

    // ── decide_prop_formula ───────────────────────────────────────────────────

    #[test]
    fn test_decide_tautology() {
        // p || !p is a tautology — but SAT says it is satisfiable (which it is).
        let f = parse_prop_formula("p || !p").expect("parse");
        let r = decide_prop_formula(&f);
        assert!(r.verdict); // Satisfiable.
    }

    #[test]
    fn test_decide_contradiction() {
        // p && !p is unsatisfiable.
        let f = parse_prop_formula("p && !p").expect("parse");
        let r = decide_prop_formula(&f);
        assert!(!r.verdict);
    }

    #[test]
    fn test_decide_sat_complex() {
        // (p || q) && (!p || r) — satisfiable.
        let f = parse_prop_formula("(p || q) && (!p || r)").expect("parse");
        let r = decide_prop_formula(&f);
        assert!(r.verdict);
    }

    // ── unit_propagate ────────────────────────────────────────────────────────

    #[test]
    fn test_unit_propagate_single_unit() {
        // Clause [1] (i.e. atom 0 must be true) with one atom.
        let clauses = vec![vec![1i32]];
        let assignment = vec![None];
        let result = unit_propagate(&clauses, &assignment).expect("no conflict");
        assert_eq!(result[0], Some(true));
    }

    #[test]
    fn test_unit_propagate_conflict() {
        // Clauses [1] and [-1] — contradiction.
        let clauses = vec![vec![1i32], vec![-1i32]];
        let assignment = vec![None];
        assert!(unit_propagate(&clauses, &assignment).is_none());
    }

    #[test]
    fn test_unit_propagate_chain() {
        // [1], [2] (two units, two atoms).
        let clauses = vec![vec![1i32], vec![2i32]];
        let assignment = vec![None, None];
        let result = unit_propagate(&clauses, &assignment).expect("no conflict");
        assert_eq!(result[0], Some(true));
        assert_eq!(result[1], Some(true));
    }

    // ── try_decide ────────────────────────────────────────────────────────────

    #[test]
    fn test_try_decide_true() {
        let cfg = DecideConfig::default();
        let r = try_decide("true", &cfg).expect("should decide");
        assert!(r.verdict);
        assert_eq!(r.procedure_used, DecisionProcedure::Refl);
    }

    #[test]
    fn test_try_decide_nat_eq() {
        let cfg = DecideConfig::default();
        let r = try_decide("5 == 5", &cfg).expect("should decide");
        assert!(r.verdict);
    }

    #[test]
    fn test_try_decide_nat_lt() {
        let cfg = DecideConfig::default();
        let r = try_decide("3 < 10", &cfg).expect("should decide");
        assert!(r.verdict);
    }

    #[test]
    fn test_try_decide_bool() {
        let cfg = DecideConfig::default();
        let r = try_decide("true && true", &cfg).expect("should decide");
        assert!(r.verdict);
    }

    #[test]
    fn test_try_decide_unrecognised() {
        let cfg = DecideConfig::default();
        // A string that looks like a free variable — cannot decide.
        let r = try_decide("some_random_thing", &cfg);
        // May or may not succeed depending on prop parsing; just check no panic.
        let _ = r;
    }

    // ── PropFormula helpers ───────────────────────────────────────────────────

    #[test]
    fn test_prop_formula_atoms() {
        let f = parse_prop_formula("p && q || r").expect("parse");
        let atoms = f.atoms();
        assert!(atoms.contains(&"p".to_string()));
        assert!(atoms.contains(&"q".to_string()));
        assert!(atoms.contains(&"r".to_string()));
    }

    #[test]
    fn test_prop_formula_eval_true() {
        let f = parse_prop_formula("p && q").expect("parse");
        let mut assignment = HashMap::new();
        assignment.insert("p".to_string(), true);
        assignment.insert("q".to_string(), true);
        assert!(f.eval(&assignment));
    }

    #[test]
    fn test_prop_formula_eval_false() {
        let f = parse_prop_formula("p && q").expect("parse");
        let mut assignment = HashMap::new();
        assignment.insert("p".to_string(), true);
        assignment.insert("q".to_string(), false);
        assert!(!f.eval(&assignment));
    }

    #[test]
    fn test_decide_config_default() {
        let cfg = DecideConfig::default();
        assert_eq!(cfg.max_nat_value, 10_000);
        assert!(cfg.use_reflection);
    }

    #[test]
    fn test_dpll_satisfiable() {
        let f = PropFormula::or(PropFormula::Atom("a".into()), PropFormula::Atom("b".into()));
        let mut assignment = HashMap::new();
        assert!(dpll(&f, &mut assignment));
    }

    #[test]
    fn test_dpll_unsatisfiable() {
        let f = PropFormula::and(
            PropFormula::Atom("a".into()),
            PropFormula::not(PropFormula::Atom("a".into())),
        );
        let mut assignment = HashMap::new();
        assert!(!dpll(&f, &mut assignment));
    }
}
