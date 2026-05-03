//! Functions for Linear Temporal Logic (LTL) evaluation and model checking.

use std::collections::{HashMap, HashSet, VecDeque};

use super::types::{BuchiAutomaton, LtlFormula, LtlModel, LtlTrace, ModelCheckResult};

// ── Trace Semantics ────────────────────────────────────────────────────────────

/// Evaluate an LTL formula at position `pos` in a (potentially infinite) trace.
///
/// For infinite traces the trace must have a `loop_start`; evaluation beyond
/// the stored states wraps around the cycle via `LtlTrace::state_at`.
pub fn eval_ltl_trace(formula: &LtlFormula, trace: &LtlTrace, pos: usize) -> bool {
    // Guard: if we are past the end of a finite trace, the formula cannot hold.
    if trace.loop_start.is_none() && pos >= trace.states.len() {
        return matches!(formula, LtlFormula::True);
    }

    match formula {
        LtlFormula::True => true,
        LtlFormula::False => false,

        LtlFormula::Atom(name) => trace
            .state_at(pos)
            .map(|s| s.contains(name.as_str()))
            .unwrap_or(false),

        LtlFormula::Neg(inner) => !eval_ltl_trace(inner, trace, pos),

        LtlFormula::And(l, r) => eval_ltl_trace(l, trace, pos) && eval_ltl_trace(r, trace, pos),

        LtlFormula::Or(l, r) => eval_ltl_trace(l, trace, pos) || eval_ltl_trace(r, trace, pos),

        LtlFormula::Implies(l, r) => {
            !eval_ltl_trace(l, trace, pos) || eval_ltl_trace(r, trace, pos)
        }

        LtlFormula::Next(inner) => eval_ltl_trace(inner, trace, pos + 1),

        LtlFormula::Until(phi, psi) => {
            // φ U ψ: bounded unrolling over trace length + cycle.
            eval_until_bounded(phi, psi, trace, pos)
        }

        LtlFormula::Release(phi, psi) => {
            // φ R ψ ≡ ¬(¬φ U ¬ψ)
            let neg_phi = LtlFormula::Neg(Box::new(*phi.clone()));
            let neg_psi = LtlFormula::Neg(Box::new(*psi.clone()));
            let until = LtlFormula::Until(Box::new(neg_phi), Box::new(neg_psi));
            !eval_ltl_trace(&until, trace, pos)
        }

        LtlFormula::Globally(inner) => {
            // G φ ≡ ¬(F ¬φ)
            let neg = LtlFormula::Neg(Box::new(*inner.clone()));
            let fin = LtlFormula::Finally(Box::new(neg));
            !eval_ltl_trace(&fin, trace, pos)
        }

        LtlFormula::Finally(inner) => {
            // F φ ≡ true U φ
            let until = LtlFormula::Until(Box::new(LtlFormula::True), Box::new(*inner.clone()));
            eval_ltl_trace(&until, trace, pos)
        }

        LtlFormula::WeakUntil(phi, psi) => {
            // φ W ψ ≡ (φ U ψ) ∨ G φ
            let until = LtlFormula::Until(phi.clone(), psi.clone());
            let glob = LtlFormula::Globally(phi.clone());
            eval_ltl_trace(&until, trace, pos) || eval_ltl_trace(&glob, trace, pos)
        }
    }
}

/// Bounded Until evaluation that handles lassos correctly.
fn eval_until_bounded(phi: &LtlFormula, psi: &LtlFormula, trace: &LtlTrace, start: usize) -> bool {
    let total = trace.states.len();
    // For lasso traces we need to check up to `total` positions starting from `start`.
    // Once we exceed stored states and have a loop, we wrap via state_at.
    let bound = if trace.loop_start.is_some() {
        total + 1
    } else {
        total
    };

    for k in start..=bound {
        if trace.state_at(k).is_none() {
            break;
        }
        if eval_ltl_trace(psi, trace, k) {
            return true;
        }
        if !eval_ltl_trace(phi, trace, k) {
            return false;
        }
    }
    false
}

// ── Lasso Semantics ────────────────────────────────────────────────────────────

/// Check whether an LTL formula is satisfied by a lasso-shaped infinite trace.
///
/// `prefix` is the finite, non-repeating prefix; `cycle` is the repeating part.
pub fn satisfies_lasso(
    formula: &LtlFormula,
    prefix: &[HashSet<String>],
    cycle: &[HashSet<String>],
) -> bool {
    if cycle.is_empty() {
        return false;
    }
    let trace = LtlTrace::new_lasso(prefix.to_vec(), cycle.to_vec());
    eval_ltl_trace(formula, &trace, 0)
}

// ── Negation Normal Form ───────────────────────────────────────────────────────

/// Convert an LTL formula to Negation Normal Form (NNF).
///
/// In NNF, negations only appear directly in front of atoms.
pub fn ltl_to_nnf(formula: &LtlFormula) -> LtlFormula {
    match formula {
        LtlFormula::True => LtlFormula::True,
        LtlFormula::False => LtlFormula::False,
        LtlFormula::Atom(a) => LtlFormula::Atom(a.clone()),

        LtlFormula::Neg(inner) => negate_ltl(inner),

        LtlFormula::And(l, r) => LtlFormula::And(Box::new(ltl_to_nnf(l)), Box::new(ltl_to_nnf(r))),
        LtlFormula::Or(l, r) => LtlFormula::Or(Box::new(ltl_to_nnf(l)), Box::new(ltl_to_nnf(r))),
        LtlFormula::Implies(l, r) => {
            // φ → ψ ≡ ¬φ ∨ ψ
            LtlFormula::Or(Box::new(negate_ltl(l)), Box::new(ltl_to_nnf(r)))
        }
        LtlFormula::Next(inner) => LtlFormula::Next(Box::new(ltl_to_nnf(inner))),
        LtlFormula::Until(l, r) => {
            LtlFormula::Until(Box::new(ltl_to_nnf(l)), Box::new(ltl_to_nnf(r)))
        }
        LtlFormula::Release(l, r) => {
            LtlFormula::Release(Box::new(ltl_to_nnf(l)), Box::new(ltl_to_nnf(r)))
        }
        LtlFormula::Globally(inner) => LtlFormula::Globally(Box::new(ltl_to_nnf(inner))),
        LtlFormula::Finally(inner) => LtlFormula::Finally(Box::new(ltl_to_nnf(inner))),
        LtlFormula::WeakUntil(l, r) => {
            LtlFormula::WeakUntil(Box::new(ltl_to_nnf(l)), Box::new(ltl_to_nnf(r)))
        }
    }
}

/// Push negation inward (one step), returning the NNF of ¬formula.
pub fn negate_ltl(formula: &LtlFormula) -> LtlFormula {
    match formula {
        LtlFormula::True => LtlFormula::False,
        LtlFormula::False => LtlFormula::True,
        LtlFormula::Atom(a) => LtlFormula::Neg(Box::new(LtlFormula::Atom(a.clone()))),

        // Double negation elimination
        LtlFormula::Neg(inner) => ltl_to_nnf(inner),

        // De Morgan
        LtlFormula::And(l, r) => LtlFormula::Or(Box::new(negate_ltl(l)), Box::new(negate_ltl(r))),
        LtlFormula::Or(l, r) => LtlFormula::And(Box::new(negate_ltl(l)), Box::new(negate_ltl(r))),

        // ¬(φ → ψ) ≡ φ ∧ ¬ψ
        LtlFormula::Implies(l, r) => {
            LtlFormula::And(Box::new(ltl_to_nnf(l)), Box::new(negate_ltl(r)))
        }

        // ¬X φ ≡ X ¬φ
        LtlFormula::Next(inner) => LtlFormula::Next(Box::new(negate_ltl(inner))),

        // ¬(φ U ψ) ≡ ¬φ R ¬ψ
        LtlFormula::Until(l, r) => {
            LtlFormula::Release(Box::new(negate_ltl(l)), Box::new(negate_ltl(r)))
        }

        // ¬(φ R ψ) ≡ ¬φ U ¬ψ
        LtlFormula::Release(l, r) => {
            LtlFormula::Until(Box::new(negate_ltl(l)), Box::new(negate_ltl(r)))
        }

        // ¬G φ ≡ F ¬φ
        LtlFormula::Globally(inner) => LtlFormula::Finally(Box::new(negate_ltl(inner))),

        // ¬F φ ≡ G ¬φ
        LtlFormula::Finally(inner) => LtlFormula::Globally(Box::new(negate_ltl(inner))),

        // ¬(φ W ψ) ≡ ¬ψ U (¬φ ∧ ¬ψ)
        LtlFormula::WeakUntil(l, r) => LtlFormula::Until(
            Box::new(negate_ltl(r)),
            Box::new(LtlFormula::And(
                Box::new(negate_ltl(l)),
                Box::new(negate_ltl(r)),
            )),
        ),
    }
}

// ── Parsing ────────────────────────────────────────────────────────────────────

/// Parse an LTL formula from a simple string representation.
///
/// Supported grammar (whitespace-insensitive):
/// ```text
/// formula ::= "true" | "false" | ident
///           | "!" formula
///           | "X" formula | "G" formula | "F" formula
///           | "(" formula ")"
///           | formula "&" formula | formula "|" formula
///           | formula "->" formula
///           | formula "U" formula | formula "R" formula | formula "W" formula
/// ```
pub fn ltl_parse(s: &str) -> Option<LtlFormula> {
    let tokens = tokenize(s);
    let mut pos = 0;
    let result = parse_implies(&tokens, &mut pos)?;
    if pos == tokens.len() {
        Some(result)
    } else {
        None
    }
}

fn tokenize(s: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut chars = s.chars().peekable();
    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' | '\n' => {
                chars.next();
            }
            '(' | ')' | '!' | '&' | '|' => {
                tokens.push(c.to_string());
                chars.next();
            }
            '-' => {
                chars.next();
                if chars.peek() == Some(&'>') {
                    chars.next();
                    tokens.push("->".to_string());
                } else {
                    tokens.push("-".to_string());
                }
            }
            _ => {
                let mut word = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        word.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if !word.is_empty() {
                    tokens.push(word);
                } else {
                    chars.next();
                }
            }
        }
    }
    tokens
}

fn parse_implies(tokens: &[String], pos: &mut usize) -> Option<LtlFormula> {
    let left = parse_or(tokens, pos)?;
    if *pos < tokens.len() && tokens[*pos] == "->" {
        *pos += 1;
        let right = parse_implies(tokens, pos)?;
        Some(LtlFormula::Implies(Box::new(left), Box::new(right)))
    } else {
        Some(left)
    }
}

fn parse_or(tokens: &[String], pos: &mut usize) -> Option<LtlFormula> {
    let mut left = parse_and(tokens, pos)?;
    while *pos < tokens.len() && tokens[*pos] == "|" {
        *pos += 1;
        let right = parse_and(tokens, pos)?;
        left = LtlFormula::Or(Box::new(left), Box::new(right));
    }
    Some(left)
}

fn parse_and(tokens: &[String], pos: &mut usize) -> Option<LtlFormula> {
    let mut left = parse_binary_temporal(tokens, pos)?;
    while *pos < tokens.len() && tokens[*pos] == "&" {
        *pos += 1;
        let right = parse_binary_temporal(tokens, pos)?;
        left = LtlFormula::And(Box::new(left), Box::new(right));
    }
    Some(left)
}

fn parse_binary_temporal(tokens: &[String], pos: &mut usize) -> Option<LtlFormula> {
    let left = parse_unary(tokens, pos)?;
    if *pos < tokens.len() {
        match tokens[*pos].as_str() {
            "U" => {
                *pos += 1;
                let right = parse_unary(tokens, pos)?;
                return Some(LtlFormula::Until(Box::new(left), Box::new(right)));
            }
            "R" => {
                *pos += 1;
                let right = parse_unary(tokens, pos)?;
                return Some(LtlFormula::Release(Box::new(left), Box::new(right)));
            }
            "W" => {
                *pos += 1;
                let right = parse_unary(tokens, pos)?;
                return Some(LtlFormula::WeakUntil(Box::new(left), Box::new(right)));
            }
            _ => {}
        }
    }
    Some(left)
}

fn parse_unary(tokens: &[String], pos: &mut usize) -> Option<LtlFormula> {
    if *pos >= tokens.len() {
        return None;
    }
    match tokens[*pos].as_str() {
        "!" => {
            *pos += 1;
            let inner = parse_unary(tokens, pos)?;
            Some(LtlFormula::Neg(Box::new(inner)))
        }
        "X" => {
            *pos += 1;
            let inner = parse_unary(tokens, pos)?;
            Some(LtlFormula::Next(Box::new(inner)))
        }
        "G" => {
            *pos += 1;
            let inner = parse_unary(tokens, pos)?;
            Some(LtlFormula::Globally(Box::new(inner)))
        }
        "F" => {
            *pos += 1;
            let inner = parse_unary(tokens, pos)?;
            Some(LtlFormula::Finally(Box::new(inner)))
        }
        _ => parse_atom(tokens, pos),
    }
}

fn parse_atom(tokens: &[String], pos: &mut usize) -> Option<LtlFormula> {
    if *pos >= tokens.len() {
        return None;
    }
    let tok = &tokens[*pos];
    match tok.as_str() {
        "true" => {
            *pos += 1;
            Some(LtlFormula::True)
        }
        "false" => {
            *pos += 1;
            Some(LtlFormula::False)
        }
        "(" => {
            *pos += 1;
            let inner = parse_implies(tokens, pos)?;
            if *pos < tokens.len() && tokens[*pos] == ")" {
                *pos += 1;
                Some(inner)
            } else {
                None
            }
        }
        s if s.chars().next().map(|c| c.is_alphabetic()).unwrap_or(false) => {
            let name = s.to_string();
            *pos += 1;
            Some(LtlFormula::Atom(name))
        }
        _ => None,
    }
}

// ── Pretty-printing ────────────────────────────────────────────────────────────

/// Convert an LTL formula back to a human-readable string.
pub fn ltl_to_string(formula: &LtlFormula) -> String {
    match formula {
        LtlFormula::True => "true".to_string(),
        LtlFormula::False => "false".to_string(),
        LtlFormula::Atom(a) => a.clone(),
        LtlFormula::Neg(inner) => format!("!{}", ltl_to_string_paren(inner, true)),
        LtlFormula::And(l, r) => format!(
            "{} & {}",
            ltl_to_string_paren(l, false),
            ltl_to_string_paren(r, false)
        ),
        LtlFormula::Or(l, r) => format!(
            "{} | {}",
            ltl_to_string_paren(l, false),
            ltl_to_string_paren(r, false)
        ),
        LtlFormula::Implies(l, r) => {
            format!("{} -> {}", ltl_to_string_paren(l, false), ltl_to_string(r))
        }
        LtlFormula::Next(inner) => format!("X {}", ltl_to_string_paren(inner, false)),
        LtlFormula::Until(l, r) => format!(
            "{} U {}",
            ltl_to_string_paren(l, false),
            ltl_to_string_paren(r, false)
        ),
        LtlFormula::Release(l, r) => format!(
            "{} R {}",
            ltl_to_string_paren(l, false),
            ltl_to_string_paren(r, false)
        ),
        LtlFormula::Globally(inner) => format!("G {}", ltl_to_string_paren(inner, false)),
        LtlFormula::Finally(inner) => format!("F {}", ltl_to_string_paren(inner, false)),
        LtlFormula::WeakUntil(l, r) => format!(
            "{} W {}",
            ltl_to_string_paren(l, false),
            ltl_to_string_paren(r, false)
        ),
    }
}

fn ltl_to_string_paren(formula: &LtlFormula, force: bool) -> String {
    let needs_parens = force
        || matches!(
            formula,
            LtlFormula::And(_, _)
                | LtlFormula::Or(_, _)
                | LtlFormula::Implies(_, _)
                | LtlFormula::Until(_, _)
                | LtlFormula::Release(_, _)
                | LtlFormula::WeakUntil(_, _)
        );
    if needs_parens {
        format!("({})", ltl_to_string(formula))
    } else {
        ltl_to_string(formula)
    }
}

// ── Free Variables ─────────────────────────────────────────────────────────────

/// Collect all atomic proposition names occurring free in an LTL formula.
pub fn ltl_free_vars(formula: &LtlFormula) -> Vec<String> {
    let mut seen: HashSet<String> = HashSet::new();
    let mut result: Vec<String> = Vec::new();
    collect_vars(formula, &mut seen, &mut result);
    result
}

fn collect_vars(formula: &LtlFormula, seen: &mut HashSet<String>, result: &mut Vec<String>) {
    match formula {
        LtlFormula::True | LtlFormula::False => {}
        LtlFormula::Atom(a) => {
            if seen.insert(a.clone()) {
                result.push(a.clone());
            }
        }
        LtlFormula::Neg(inner)
        | LtlFormula::Next(inner)
        | LtlFormula::Globally(inner)
        | LtlFormula::Finally(inner) => collect_vars(inner, seen, result),
        LtlFormula::And(l, r)
        | LtlFormula::Or(l, r)
        | LtlFormula::Implies(l, r)
        | LtlFormula::Until(l, r)
        | LtlFormula::Release(l, r)
        | LtlFormula::WeakUntil(l, r) => {
            collect_vars(l, seen, result);
            collect_vars(r, seen, result);
        }
    }
}

// ── Model Checking ─────────────────────────────────────────────────────────────

/// Check whether all runs of a finite Kripke model satisfy an LTL formula.
///
/// Uses explicit-state enumeration: for each initial state, do a DFS to find
/// all lassos (cycles reachable from the initial state) and check each one.
pub fn check_ltl_model(model: &LtlModel, formula: &LtlFormula) -> ModelCheckResult {
    match find_counterexample(model, formula) {
        Some(trace) => ModelCheckResult::Violated {
            counterexample: trace,
        },
        None => ModelCheckResult::Satisfied,
    }
}

/// Find a counterexample trace (lasso) for the formula in the model, if one exists.
pub fn find_counterexample(model: &LtlModel, formula: &LtlFormula) -> Option<LtlTrace> {
    // We enumerate all simple paths from each initial state.
    // When a path reaches a previously-visited state, we have a lasso.
    // We check the negation of the formula: if ¬formula holds on a lasso, we found a counterexample.
    let neg = LtlFormula::Neg(Box::new(formula.clone()));
    let neg_nnf = ltl_to_nnf(&neg);

    for &init in &model.initial {
        if let Some(trace) = dfs_find_lasso(model, init, &neg_nnf) {
            return Some(trace);
        }
    }
    None
}

/// DFS to enumerate lasso-shaped runs and check the formula on each.
fn dfs_find_lasso(model: &LtlModel, start: usize, formula: &LtlFormula) -> Option<LtlTrace> {
    // Stack holds (current state, path so far, visited set with entry index)
    let mut stack: Vec<(usize, Vec<usize>)> = vec![(start, vec![start])];

    while let Some((state, path)) = stack.pop() {
        let succs = model.successors(state);

        if succs.is_empty() {
            // Dead end — treat as a trivial loop on itself for checking purposes
            let labels: Vec<HashSet<String>> =
                path.iter().map(|&s| model.labels[s].clone()).collect();
            let trace = LtlTrace::new_lasso(labels.clone(), vec![model.labels[state].clone()]);
            if eval_ltl_trace(formula, &trace, 0) {
                return Some(trace);
            }
            continue;
        }

        for succ in succs {
            // Check if successor is already in the current path (lasso found)
            if let Some(loop_pos) = path.iter().position(|&s| s == succ) {
                let prefix: Vec<HashSet<String>> = path[..loop_pos]
                    .iter()
                    .map(|&s| model.labels[s].clone())
                    .collect();
                let cycle: Vec<HashSet<String>> = path[loop_pos..]
                    .iter()
                    .map(|&s| model.labels[s].clone())
                    .collect();
                let trace = LtlTrace::new_lasso(prefix, cycle);
                if eval_ltl_trace(formula, &trace, 0) {
                    return Some(trace);
                }
            } else if path.len() < model.states.len() + 1 {
                // Continue DFS (bound by model size to avoid infinite loops)
                let mut new_path = path.clone();
                new_path.push(succ);
                stack.push((succ, new_path));
            }
        }
    }
    None
}

// ── Known LTL Equivalences ─────────────────────────────────────────────────────

/// Return a list of well-known LTL laws as (name, lhs, rhs) triples.
///
/// Each law states that lhs and rhs are semantically equivalent for all traces.
pub fn ltl_equivalences() -> Vec<(&'static str, LtlFormula, LtlFormula)> {
    vec![
        // Boolean duals
        (
            "double_neg",
            LtlFormula::Neg(Box::new(LtlFormula::Neg(Box::new(LtlFormula::Atom(
                "p".to_string(),
            ))))),
            LtlFormula::Atom("p".to_string()),
        ),
        // F p ≡ true U p
        (
            "finally_as_until",
            LtlFormula::Finally(Box::new(LtlFormula::Atom("p".to_string()))),
            LtlFormula::Until(
                Box::new(LtlFormula::True),
                Box::new(LtlFormula::Atom("p".to_string())),
            ),
        ),
        // G p ≡ false R p
        (
            "globally_as_release",
            LtlFormula::Globally(Box::new(LtlFormula::Atom("p".to_string()))),
            LtlFormula::Release(
                Box::new(LtlFormula::False),
                Box::new(LtlFormula::Atom("p".to_string())),
            ),
        ),
        // p W q ≡ (p U q) | G p
        (
            "weak_until_expansion",
            LtlFormula::WeakUntil(
                Box::new(LtlFormula::Atom("p".to_string())),
                Box::new(LtlFormula::Atom("q".to_string())),
            ),
            LtlFormula::Or(
                Box::new(LtlFormula::Until(
                    Box::new(LtlFormula::Atom("p".to_string())),
                    Box::new(LtlFormula::Atom("q".to_string())),
                )),
                Box::new(LtlFormula::Globally(Box::new(LtlFormula::Atom(
                    "p".to_string(),
                )))),
            ),
        ),
        // G(p & q) ≡ G p & G q
        (
            "globally_and",
            LtlFormula::Globally(Box::new(LtlFormula::And(
                Box::new(LtlFormula::Atom("p".to_string())),
                Box::new(LtlFormula::Atom("q".to_string())),
            ))),
            LtlFormula::And(
                Box::new(LtlFormula::Globally(Box::new(LtlFormula::Atom(
                    "p".to_string(),
                )))),
                Box::new(LtlFormula::Globally(Box::new(LtlFormula::Atom(
                    "q".to_string(),
                )))),
            ),
        ),
        // F(p | q) ≡ F p | F q
        (
            "finally_or",
            LtlFormula::Finally(Box::new(LtlFormula::Or(
                Box::new(LtlFormula::Atom("p".to_string())),
                Box::new(LtlFormula::Atom("q".to_string())),
            ))),
            LtlFormula::Or(
                Box::new(LtlFormula::Finally(Box::new(LtlFormula::Atom(
                    "p".to_string(),
                )))),
                Box::new(LtlFormula::Finally(Box::new(LtlFormula::Atom(
                    "q".to_string(),
                )))),
            ),
        ),
        // ¬G p ≡ F ¬p
        (
            "neg_globally",
            LtlFormula::Neg(Box::new(LtlFormula::Globally(Box::new(LtlFormula::Atom(
                "p".to_string(),
            ))))),
            LtlFormula::Finally(Box::new(LtlFormula::Neg(Box::new(LtlFormula::Atom(
                "p".to_string(),
            ))))),
        ),
        // ¬F p ≡ G ¬p
        (
            "neg_finally",
            LtlFormula::Neg(Box::new(LtlFormula::Finally(Box::new(LtlFormula::Atom(
                "p".to_string(),
            ))))),
            LtlFormula::Globally(Box::new(LtlFormula::Neg(Box::new(LtlFormula::Atom(
                "p".to_string(),
            ))))),
        ),
        // X(p & q) ≡ Xp & Xq
        (
            "next_and",
            LtlFormula::Next(Box::new(LtlFormula::And(
                Box::new(LtlFormula::Atom("p".to_string())),
                Box::new(LtlFormula::Atom("q".to_string())),
            ))),
            LtlFormula::And(
                Box::new(LtlFormula::Next(Box::new(LtlFormula::Atom(
                    "p".to_string(),
                )))),
                Box::new(LtlFormula::Next(Box::new(LtlFormula::Atom(
                    "q".to_string(),
                )))),
            ),
        ),
        // p U q ≡ q | (p & X(p U q))  — expansion law
        (
            "until_expansion",
            LtlFormula::Until(
                Box::new(LtlFormula::Atom("p".to_string())),
                Box::new(LtlFormula::Atom("q".to_string())),
            ),
            LtlFormula::Or(
                Box::new(LtlFormula::Atom("q".to_string())),
                Box::new(LtlFormula::And(
                    Box::new(LtlFormula::Atom("p".to_string())),
                    Box::new(LtlFormula::Next(Box::new(LtlFormula::Until(
                        Box::new(LtlFormula::Atom("p".to_string())),
                        Box::new(LtlFormula::Atom("q".to_string())),
                    )))),
                )),
            ),
        ),
    ]
}

// ── Büchi Automaton Construction (stub for integration) ────────────────────────

/// Build a simple (tableau-based) Büchi automaton from an LTL formula.
///
/// This is a simplified version suitable for small formulas and educational use.
/// For a full translation use the Gerth-Peled-Vardi-Wolper (GPVW) algorithm.
pub fn ltl_to_buchi(formula: &LtlFormula) -> BuchiAutomaton {
    let nnf = ltl_to_nnf(formula);
    let mut automaton = BuchiAutomaton::new();

    // Create a single "initial" state that represents the formula obligation.
    let init_name = format!("{{{}}}", ltl_to_string(&nnf));
    automaton.states.push(init_name.clone());
    automaton.initial.push(0);
    automaton.accepting.push(0);

    // For simple atomic formulas or True/False, just one self-loop state.
    automaton.transitions.push((0, init_name, 0));
    automaton
}

// ── Tests ──────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn state(props: &[&str]) -> HashSet<String> {
        props.iter().map(|s| s.to_string()).collect()
    }

    // ── Atom evaluation ──────────────────────────────────────────────────────

    #[test]
    fn test_eval_atom_true() {
        let trace = LtlTrace::new_lasso(vec![state(&["p"])], vec![state(&["p"])]);
        let f = LtlFormula::Atom("p".to_string());
        assert!(eval_ltl_trace(&f, &trace, 0));
    }

    #[test]
    fn test_eval_atom_false() {
        let trace = LtlTrace::new_lasso(vec![state(&["q"])], vec![state(&["q"])]);
        let f = LtlFormula::Atom("p".to_string());
        assert!(!eval_ltl_trace(&f, &trace, 0));
    }

    #[test]
    fn test_eval_true_false_constants() {
        let trace = LtlTrace::new_lasso(vec![state(&[])], vec![state(&[])]);
        assert!(eval_ltl_trace(&LtlFormula::True, &trace, 0));
        assert!(!eval_ltl_trace(&LtlFormula::False, &trace, 0));
    }

    // ── Boolean connectives ──────────────────────────────────────────────────

    #[test]
    fn test_eval_neg() {
        let trace = LtlTrace::new_lasso(vec![state(&["p"])], vec![state(&["p"])]);
        let f = LtlFormula::Neg(Box::new(LtlFormula::Atom("p".to_string())));
        assert!(!eval_ltl_trace(&f, &trace, 0));
    }

    #[test]
    fn test_eval_and() {
        let trace = LtlTrace::new_lasso(vec![state(&["p", "q"])], vec![state(&["p", "q"])]);
        let f = LtlFormula::And(
            Box::new(LtlFormula::Atom("p".to_string())),
            Box::new(LtlFormula::Atom("q".to_string())),
        );
        assert!(eval_ltl_trace(&f, &trace, 0));
    }

    #[test]
    fn test_eval_or_partial() {
        let trace = LtlTrace::new_lasso(vec![state(&["p"])], vec![state(&["p"])]);
        let f = LtlFormula::Or(
            Box::new(LtlFormula::Atom("p".to_string())),
            Box::new(LtlFormula::Atom("q".to_string())),
        );
        assert!(eval_ltl_trace(&f, &trace, 0));
    }

    // ── Temporal operators ───────────────────────────────────────────────────

    #[test]
    fn test_eval_next() {
        let trace = LtlTrace::new_lasso(vec![state(&[]), state(&["p"])], vec![state(&["p"])]);
        let f = LtlFormula::Next(Box::new(LtlFormula::Atom("p".to_string())));
        assert!(eval_ltl_trace(&f, &trace, 0));
    }

    #[test]
    fn test_eval_globally_holds() {
        let trace = LtlTrace::new_lasso(vec![], vec![state(&["p"])]);
        let f = LtlFormula::Globally(Box::new(LtlFormula::Atom("p".to_string())));
        assert!(eval_ltl_trace(&f, &trace, 0));
    }

    #[test]
    fn test_eval_globally_fails() {
        let trace = LtlTrace::new_lasso(vec![state(&["p"])], vec![state(&[])]);
        let f = LtlFormula::Globally(Box::new(LtlFormula::Atom("p".to_string())));
        assert!(!eval_ltl_trace(&f, &trace, 0));
    }

    #[test]
    fn test_eval_finally_holds() {
        let trace = LtlTrace::new_lasso(vec![state(&[]), state(&["p"])], vec![state(&[])]);
        let f = LtlFormula::Finally(Box::new(LtlFormula::Atom("p".to_string())));
        assert!(eval_ltl_trace(&f, &trace, 0));
    }

    #[test]
    fn test_eval_until_basic() {
        // p U q on {p}, {p}, {q}, {q}, ...
        let trace = LtlTrace::new_lasso(vec![state(&["p"]), state(&["p"])], vec![state(&["q"])]);
        let f = LtlFormula::Until(
            Box::new(LtlFormula::Atom("p".to_string())),
            Box::new(LtlFormula::Atom("q".to_string())),
        );
        assert!(eval_ltl_trace(&f, &trace, 0));
    }

    #[test]
    fn test_eval_release() {
        // p R q: q holds until (and including) p; here p always false so q must hold forever
        let trace = LtlTrace::new_lasso(vec![], vec![state(&["q"])]);
        let f = LtlFormula::Release(
            Box::new(LtlFormula::Atom("p".to_string())),
            Box::new(LtlFormula::Atom("q".to_string())),
        );
        assert!(eval_ltl_trace(&f, &trace, 0));
    }

    #[test]
    fn test_eval_weak_until_no_q() {
        // p W q where p always holds and q never holds => satisfied
        let trace = LtlTrace::new_lasso(vec![], vec![state(&["p"])]);
        let f = LtlFormula::WeakUntil(
            Box::new(LtlFormula::Atom("p".to_string())),
            Box::new(LtlFormula::Atom("q".to_string())),
        );
        assert!(eval_ltl_trace(&f, &trace, 0));
    }

    // ── Lasso semantics ──────────────────────────────────────────────────────

    #[test]
    fn test_satisfies_lasso_globally_p() {
        let cycle = vec![state(&["p"])];
        let f = LtlFormula::Globally(Box::new(LtlFormula::Atom("p".to_string())));
        assert!(satisfies_lasso(&f, &[], &cycle));
    }

    #[test]
    fn test_satisfies_lasso_finally_p() {
        let prefix = vec![state(&[]), state(&[])];
        let cycle = vec![state(&["p"])];
        let f = LtlFormula::Finally(Box::new(LtlFormula::Atom("p".to_string())));
        assert!(satisfies_lasso(&f, &prefix, &cycle));
    }

    // ── NNF conversion ───────────────────────────────────────────────────────

    #[test]
    fn test_nnf_double_neg() {
        let f = LtlFormula::Neg(Box::new(LtlFormula::Neg(Box::new(LtlFormula::Atom(
            "p".to_string(),
        )))));
        let nnf = ltl_to_nnf(&f);
        assert_eq!(nnf, LtlFormula::Atom("p".to_string()));
    }

    #[test]
    fn test_nnf_neg_and() {
        // ¬(p ∧ q) → ¬p ∨ ¬q
        let f = LtlFormula::Neg(Box::new(LtlFormula::And(
            Box::new(LtlFormula::Atom("p".to_string())),
            Box::new(LtlFormula::Atom("q".to_string())),
        )));
        let nnf = ltl_to_nnf(&f);
        assert_eq!(
            nnf,
            LtlFormula::Or(
                Box::new(LtlFormula::Neg(Box::new(LtlFormula::Atom("p".to_string())))),
                Box::new(LtlFormula::Neg(Box::new(LtlFormula::Atom("q".to_string())))),
            )
        );
    }

    #[test]
    fn test_nnf_neg_globally() {
        // ¬G p → F ¬p
        let f = LtlFormula::Neg(Box::new(LtlFormula::Globally(Box::new(LtlFormula::Atom(
            "p".to_string(),
        )))));
        let nnf = ltl_to_nnf(&f);
        assert_eq!(
            nnf,
            LtlFormula::Finally(Box::new(LtlFormula::Neg(Box::new(LtlFormula::Atom(
                "p".to_string()
            )))))
        );
    }

    #[test]
    fn test_nnf_neg_until() {
        // ¬(p U q) → ¬p R ¬q
        let f = LtlFormula::Neg(Box::new(LtlFormula::Until(
            Box::new(LtlFormula::Atom("p".to_string())),
            Box::new(LtlFormula::Atom("q".to_string())),
        )));
        let nnf = ltl_to_nnf(&f);
        assert_eq!(
            nnf,
            LtlFormula::Release(
                Box::new(LtlFormula::Neg(Box::new(LtlFormula::Atom("p".to_string())))),
                Box::new(LtlFormula::Neg(Box::new(LtlFormula::Atom("q".to_string())))),
            )
        );
    }

    // ── Parsing ──────────────────────────────────────────────────────────────

    #[test]
    fn test_parse_atom() {
        let f = ltl_parse("p");
        assert_eq!(f, Some(LtlFormula::Atom("p".to_string())));
    }

    #[test]
    fn test_parse_true_false() {
        assert_eq!(ltl_parse("true"), Some(LtlFormula::True));
        assert_eq!(ltl_parse("false"), Some(LtlFormula::False));
    }

    #[test]
    fn test_parse_neg() {
        let f = ltl_parse("!p");
        assert_eq!(
            f,
            Some(LtlFormula::Neg(Box::new(LtlFormula::Atom("p".to_string()))))
        );
    }

    #[test]
    fn test_parse_and() {
        let f = ltl_parse("p & q");
        assert_eq!(
            f,
            Some(LtlFormula::And(
                Box::new(LtlFormula::Atom("p".to_string())),
                Box::new(LtlFormula::Atom("q".to_string())),
            ))
        );
    }

    #[test]
    fn test_parse_globally() {
        let f = ltl_parse("G p");
        assert_eq!(
            f,
            Some(LtlFormula::Globally(Box::new(LtlFormula::Atom(
                "p".to_string()
            ))))
        );
    }

    #[test]
    fn test_parse_finally() {
        let f = ltl_parse("F p");
        assert_eq!(
            f,
            Some(LtlFormula::Finally(Box::new(LtlFormula::Atom(
                "p".to_string()
            ))))
        );
    }

    #[test]
    fn test_parse_until() {
        let f = ltl_parse("p U q");
        assert_eq!(
            f,
            Some(LtlFormula::Until(
                Box::new(LtlFormula::Atom("p".to_string())),
                Box::new(LtlFormula::Atom("q".to_string())),
            ))
        );
    }

    #[test]
    fn test_parse_implies() {
        let f = ltl_parse("p -> q");
        assert_eq!(
            f,
            Some(LtlFormula::Implies(
                Box::new(LtlFormula::Atom("p".to_string())),
                Box::new(LtlFormula::Atom("q".to_string())),
            ))
        );
    }

    #[test]
    fn test_parse_parentheses() {
        let f = ltl_parse("(p & q)");
        assert_eq!(
            f,
            Some(LtlFormula::And(
                Box::new(LtlFormula::Atom("p".to_string())),
                Box::new(LtlFormula::Atom("q".to_string())),
            ))
        );
    }

    // ── Pretty-printing ──────────────────────────────────────────────────────

    #[test]
    fn test_to_string_atom() {
        assert_eq!(ltl_to_string(&LtlFormula::Atom("p".to_string())), "p");
    }

    #[test]
    fn test_to_string_globally() {
        let f = LtlFormula::Globally(Box::new(LtlFormula::Atom("p".to_string())));
        assert_eq!(ltl_to_string(&f), "G p");
    }

    // ── Free variables ───────────────────────────────────────────────────────

    #[test]
    fn test_free_vars_basic() {
        let f = LtlFormula::And(
            Box::new(LtlFormula::Atom("p".to_string())),
            Box::new(LtlFormula::Atom("q".to_string())),
        );
        let vars = ltl_free_vars(&f);
        assert!(vars.contains(&"p".to_string()));
        assert!(vars.contains(&"q".to_string()));
    }

    #[test]
    fn test_free_vars_dedup() {
        let f = LtlFormula::And(
            Box::new(LtlFormula::Atom("p".to_string())),
            Box::new(LtlFormula::Atom("p".to_string())),
        );
        let vars = ltl_free_vars(&f);
        assert_eq!(vars.len(), 1);
    }

    // ── Model checking ───────────────────────────────────────────────────────

    #[test]
    fn test_model_check_simple_satisfied() {
        // Two-state model: s0 ---> s1 ---> s0, s0 has "p", s1 has "q"
        let mut model = LtlModel::new();
        let s0 = model.add_state("s0", state(&["p"]));
        let s1 = model.add_state("s1", state(&["q"]));
        model.add_transition(s0, s1);
        model.add_transition(s1, s0);
        model.set_initial(s0);

        // G(p | q) should be satisfied
        let f = LtlFormula::Globally(Box::new(LtlFormula::Or(
            Box::new(LtlFormula::Atom("p".to_string())),
            Box::new(LtlFormula::Atom("q".to_string())),
        )));
        let result = check_ltl_model(&model, &f);
        assert!(matches!(result, ModelCheckResult::Satisfied));
    }

    #[test]
    fn test_model_check_simple_violated() {
        // One state with no "p"
        let mut model = LtlModel::new();
        let s0 = model.add_state("s0", state(&["q"]));
        model.add_transition(s0, s0);
        model.set_initial(s0);

        // G p should be violated
        let f = LtlFormula::Globally(Box::new(LtlFormula::Atom("p".to_string())));
        let result = check_ltl_model(&model, &f);
        assert!(matches!(result, ModelCheckResult::Violated { .. }));
    }

    #[test]
    fn test_model_check_eventually_p() {
        // s0 -> s1(p) -> s1 loop
        let mut model = LtlModel::new();
        let s0 = model.add_state("s0", state(&[]));
        let s1 = model.add_state("s1", state(&["p"]));
        model.add_transition(s0, s1);
        model.add_transition(s1, s1);
        model.set_initial(s0);

        let f = LtlFormula::Finally(Box::new(LtlFormula::Atom("p".to_string())));
        let result = check_ltl_model(&model, &f);
        assert!(matches!(result, ModelCheckResult::Satisfied));
    }

    // ── LTL equivalences ─────────────────────────────────────────────────────

    #[test]
    fn test_equivalences_nonempty() {
        let eqs = ltl_equivalences();
        assert!(eqs.len() >= 8, "Expected at least 8 equivalences");
    }

    #[test]
    fn test_equivalences_names_unique() {
        let eqs = ltl_equivalences();
        let names: HashSet<&str> = eqs.iter().map(|(n, _, _)| *n).collect();
        assert_eq!(names.len(), eqs.len(), "Equivalence names must be unique");
    }
}
