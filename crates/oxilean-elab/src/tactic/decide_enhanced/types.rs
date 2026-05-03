//! Types for the enhanced `decide` tactic.
//!
//! The enhanced tactic extends the trivial `decide` with:
//! - Natural-number arithmetic evaluation,
//! - Boolean expression evaluation,
//! - DPLL-based SAT solving for propositional formulae,
//! - Finite-enumeration checking,
//! - Reflection-based procedures.

use std::collections::HashMap;

// ──────────────────────────────────────────────────────────────────────────────
// Decision procedures
// ──────────────────────────────────────────────────────────────────────────────

/// The decision procedure that produced a `DecideResult`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecisionProcedure {
    /// Decided by definitional equality / reflexivity.
    Refl,
    /// Decided by evaluating natural-number arithmetic.
    NatArith,
    /// Decided by evaluating a boolean expression.
    BoolEval,
    /// Decided by exhaustive enumeration over a finite type.
    FiniteEnum,
    /// Decided by DPLL SAT solving a propositional formula.
    PropFormulaDpll,
    /// Decision timed out; the result is inconclusive.
    Timeout,
}

// ──────────────────────────────────────────────────────────────────────────────
// Configuration
// ──────────────────────────────────────────────────────────────────────────────

/// Configuration for the enhanced `decide` tactic.
#[derive(Debug, Clone)]
pub struct DecideConfig {
    /// Maximum natural-number value to evaluate (prevents unbounded search).
    pub max_nat_value: u64,
    /// Timeout in milliseconds (used as a step budget, not wall-clock time).
    pub timeout_ms: u64,
    /// Whether to attempt reflection-based procedures before others.
    pub use_reflection: bool,
}

impl Default for DecideConfig {
    fn default() -> Self {
        Self {
            max_nat_value: 10_000,
            timeout_ms: 5_000,
            use_reflection: true,
        }
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Result
// ──────────────────────────────────────────────────────────────────────────────

/// The result produced by a decision procedure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecideResult {
    /// `true` = the proposition holds; `false` = it does not.
    pub verdict: bool,
    /// The procedure that produced this verdict.
    pub procedure_used: DecisionProcedure,
    /// Number of internal evaluation steps performed.
    pub steps: u32,
}

// ──────────────────────────────────────────────────────────────────────────────
// Decidable proposition
// ──────────────────────────────────────────────────────────────────────────────

/// A proposition that can, in principle, be decided by one of the built-in
/// procedures.
#[derive(Debug, Clone)]
pub struct DecidableProposition {
    /// The raw goal expression string.
    pub expr: String,
    /// The procedure preferred for this proposition (determined heuristically).
    pub preferred_procedure: DecisionProcedure,
}

impl DecidableProposition {
    /// Create a decidable proposition from a raw expression string.
    pub fn new(expr: impl Into<String>) -> Self {
        let expr = expr.into();
        let preferred_procedure = infer_procedure(&expr);
        Self {
            expr,
            preferred_procedure,
        }
    }
}

/// Infer which decision procedure is most likely to apply to `expr`.
fn infer_procedure(expr: &str) -> DecisionProcedure {
    let e = expr.trim().to_lowercase();
    if e == "true" || e == "false" || e == "rfl" {
        return DecisionProcedure::Refl;
    }
    if looks_like_nat_arith(expr) {
        return DecisionProcedure::NatArith;
    }
    if looks_like_bool_expr(expr) {
        return DecisionProcedure::BoolEval;
    }
    if looks_like_prop_formula(expr) {
        return DecisionProcedure::PropFormulaDpll;
    }
    DecisionProcedure::FiniteEnum
}

/// Heuristic: the expression looks like a Nat arithmetic comparison.
pub(super) fn looks_like_nat_arith(expr: &str) -> bool {
    // Contains a comparison operator and only Nat-like tokens.
    let has_cmp = expr.contains("==")
        || expr.contains("!=")
        || expr.contains("<=")
        || expr.contains(">=")
        || (expr.contains('<') && !expr.contains("<<"))
        || (expr.contains('>') && !expr.contains(">>"));
    let has_nat = expr.bytes().any(|b| b.is_ascii_digit());
    has_cmp && has_nat
}

/// Heuristic: the expression looks like a boolean expression.
pub(super) fn looks_like_bool_expr(expr: &str) -> bool {
    let e = expr.trim().to_lowercase();
    e == "true"
        || e == "false"
        || e.contains("&&")
        || e.contains("||")
        || e.starts_with("!")
        || e.contains(" and ")
        || e.contains(" or ")
        || e.contains(" not ")
}

/// Heuristic: the expression looks like a propositional formula.
pub(super) fn looks_like_prop_formula(expr: &str) -> bool {
    let e = expr.to_lowercase();
    e.contains(" ∧ ")
        || e.contains(" ∨ ")
        || e.contains("¬")
        || e.contains("/\\")
        || e.contains("\\/")
        || e.contains(" and ")
        || e.contains(" or ")
}

// ──────────────────────────────────────────────────────────────────────────────
// Propositional formula AST
// ──────────────────────────────────────────────────────────────────────────────

/// A simple propositional formula used for SAT/UNSAT checking.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropFormula {
    /// The constant `True`.
    True,
    /// The constant `False`.
    False,
    /// A propositional atom identified by name.
    Atom(String),
    /// Logical negation.
    Not(Box<PropFormula>),
    /// Logical conjunction.
    And(Box<PropFormula>, Box<PropFormula>),
    /// Logical disjunction.
    Or(Box<PropFormula>, Box<PropFormula>),
    /// Logical implication.
    Implies(Box<PropFormula>, Box<PropFormula>),
    /// Logical biconditional.
    Iff(Box<PropFormula>, Box<PropFormula>),
}

impl PropFormula {
    /// Convenience constructor for `Not`.
    pub fn not(f: PropFormula) -> Self {
        PropFormula::Not(Box::new(f))
    }

    /// Convenience constructor for `And`.
    pub fn and(l: PropFormula, r: PropFormula) -> Self {
        PropFormula::And(Box::new(l), Box::new(r))
    }

    /// Convenience constructor for `Or`.
    pub fn or(l: PropFormula, r: PropFormula) -> Self {
        PropFormula::Or(Box::new(l), Box::new(r))
    }

    /// Convenience constructor for `Implies`.
    pub fn implies(l: PropFormula, r: PropFormula) -> Self {
        PropFormula::Implies(Box::new(l), Box::new(r))
    }

    /// Collect all atom names that appear in the formula.
    pub fn atoms(&self) -> Vec<String> {
        let mut atoms = Vec::new();
        self.collect_atoms(&mut atoms);
        atoms.sort();
        atoms.dedup();
        atoms
    }

    fn collect_atoms(&self, out: &mut Vec<String>) {
        match self {
            PropFormula::Atom(name) => out.push(name.clone()),
            PropFormula::Not(f) => f.collect_atoms(out),
            PropFormula::And(l, r)
            | PropFormula::Or(l, r)
            | PropFormula::Implies(l, r)
            | PropFormula::Iff(l, r) => {
                l.collect_atoms(out);
                r.collect_atoms(out);
            }
            PropFormula::True | PropFormula::False => {}
        }
    }

    /// Evaluate the formula under the given variable assignment.
    /// Variables not in the map are treated as `false`.
    pub fn eval(&self, assignment: &HashMap<String, bool>) -> bool {
        match self {
            PropFormula::True => true,
            PropFormula::False => false,
            PropFormula::Atom(name) => *assignment.get(name).unwrap_or(&false),
            PropFormula::Not(f) => !f.eval(assignment),
            PropFormula::And(l, r) => l.eval(assignment) && r.eval(assignment),
            PropFormula::Or(l, r) => l.eval(assignment) || r.eval(assignment),
            PropFormula::Implies(l, r) => !l.eval(assignment) || r.eval(assignment),
            PropFormula::Iff(l, r) => l.eval(assignment) == r.eval(assignment),
        }
    }

    /// Convert this formula to CNF clause representation.
    /// Each clause is a list of literals; a positive integer `i` means atom
    /// `atoms[i-1]` is positive, negative means negated.
    pub fn to_cnf_clauses(&self, atoms: &[String]) -> Vec<Vec<i32>> {
        // We use Tseitin transformation lite: convert to NNF first, then CNF.
        let nnf = self.to_nnf();
        nnf.cnf_clauses_nnf(atoms)
    }

    /// Push negations inward (Negation Normal Form).
    pub fn to_nnf(&self) -> PropFormula {
        match self {
            PropFormula::True => PropFormula::True,
            PropFormula::False => PropFormula::False,
            PropFormula::Atom(s) => PropFormula::Atom(s.clone()),
            PropFormula::Not(inner) => match inner.as_ref() {
                PropFormula::True => PropFormula::False,
                PropFormula::False => PropFormula::True,
                PropFormula::Atom(s) => PropFormula::Not(Box::new(PropFormula::Atom(s.clone()))),
                PropFormula::Not(inner2) => inner2.to_nnf(),
                PropFormula::And(l, r) => PropFormula::or(
                    PropFormula::not(l.as_ref().clone()).to_nnf(),
                    PropFormula::not(r.as_ref().clone()).to_nnf(),
                ),
                PropFormula::Or(l, r) => PropFormula::and(
                    PropFormula::not(l.as_ref().clone()).to_nnf(),
                    PropFormula::not(r.as_ref().clone()).to_nnf(),
                ),
                PropFormula::Implies(l, r) => {
                    // ¬(A → B) = A ∧ ¬B
                    PropFormula::and(l.to_nnf(), PropFormula::not(r.as_ref().clone()).to_nnf())
                }
                PropFormula::Iff(l, r) => {
                    // ¬(A ↔ B) = (A ∧ ¬B) ∨ (¬A ∧ B)
                    PropFormula::or(
                        PropFormula::and(l.to_nnf(), PropFormula::not(r.as_ref().clone()).to_nnf()),
                        PropFormula::and(PropFormula::not(l.as_ref().clone()).to_nnf(), r.to_nnf()),
                    )
                }
            },
            PropFormula::And(l, r) => PropFormula::and(l.to_nnf(), r.to_nnf()),
            PropFormula::Or(l, r) => PropFormula::or(l.to_nnf(), r.to_nnf()),
            PropFormula::Implies(l, r) => {
                // A → B = ¬A ∨ B
                PropFormula::or(PropFormula::not(l.as_ref().clone()).to_nnf(), r.to_nnf())
            }
            PropFormula::Iff(l, r) => {
                // A ↔ B = (A → B) ∧ (B → A)
                PropFormula::and(
                    PropFormula::implies(l.as_ref().clone(), r.as_ref().clone()).to_nnf(),
                    PropFormula::implies(r.as_ref().clone(), l.as_ref().clone()).to_nnf(),
                )
            }
        }
    }

    /// Produce CNF clauses from a formula already in NNF.
    fn cnf_clauses_nnf(&self, atoms: &[String]) -> Vec<Vec<i32>> {
        match self {
            PropFormula::True => vec![],        // No constraint (always true).
            PropFormula::False => vec![vec![]], // Empty clause = contradiction.
            PropFormula::Atom(name) => {
                let idx = atoms.iter().position(|a| a == name).map(|i| i as i32 + 1);
                vec![vec![idx.unwrap_or(0)]]
            }
            PropFormula::Not(inner) => {
                if let PropFormula::Atom(name) = inner.as_ref() {
                    let idx = atoms
                        .iter()
                        .position(|a| a == name)
                        .map(|i| -(i as i32 + 1));
                    vec![vec![idx.unwrap_or(0)]]
                } else {
                    vec![] // NNF guarantees Not only wraps atoms.
                }
            }
            PropFormula::And(l, r) => {
                let mut clauses = l.cnf_clauses_nnf(atoms);
                clauses.extend(r.cnf_clauses_nnf(atoms));
                clauses
            }
            PropFormula::Or(l, r) => {
                // Distribute Or over And via Cartesian product.
                let left_clauses = l.cnf_clauses_nnf(atoms);
                let right_clauses = r.cnf_clauses_nnf(atoms);
                if left_clauses.is_empty() || right_clauses.is_empty() {
                    return vec![]; // One side is True.
                }
                let mut result = Vec::new();
                for lc in &left_clauses {
                    for rc in &right_clauses {
                        let mut combined = lc.clone();
                        combined.extend_from_slice(rc);
                        // Deduplicate and sort literals.
                        combined.sort_unstable();
                        combined.dedup();
                        result.push(combined);
                    }
                }
                result
            }
            // Implies / Iff should not appear in NNF.
            PropFormula::Implies(l, r) => {
                // Treat as ¬l ∨ r (already NNF equivalent).
                PropFormula::or(PropFormula::not(l.as_ref().clone()), r.as_ref().clone())
                    .cnf_clauses_nnf(atoms)
            }
            PropFormula::Iff(l, r) => PropFormula::and(
                PropFormula::implies(l.as_ref().clone(), r.as_ref().clone()),
                PropFormula::implies(r.as_ref().clone(), l.as_ref().clone()),
            )
            .cnf_clauses_nnf(atoms),
        }
    }
}
