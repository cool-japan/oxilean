//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use std::collections::HashMap;

/// Craig interpolant: given unsatisfiable (A ∧ B), find I such that A ⊢ I and I ∧ B is unsat,
/// and I only uses common atoms of A and B.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Interpolant {
    pub formula: PropFormula,
    pub common_atoms: Vec<String>,
}
#[allow(dead_code)]
impl Interpolant {
    /// Simple case: if A = (p) and B = (¬p) and they share p, interpolant is p.
    pub fn trivial(atom: &str) -> Self {
        Interpolant {
            formula: PropFormula::Atom(atom.to_string()),
            common_atoms: vec![atom.to_string()],
        }
    }
    pub fn true_interpolant() -> Self {
        Interpolant {
            formula: PropFormula::True,
            common_atoms: vec![],
        }
    }
    pub fn false_interpolant() -> Self {
        Interpolant {
            formula: PropFormula::False,
            common_atoms: vec![],
        }
    }
    pub fn is_trivial(&self) -> bool {
        matches!(&self.formula, PropFormula::True | PropFormula::False)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum PropProof {
    Hyp(String),
    ImplIntro(String, Box<PropProof>),
    ImplElim(Box<PropProof>, Box<PropProof>),
    AndIntro(Box<PropProof>, Box<PropProof>),
    AndLeft(Box<PropProof>),
    AndRight(Box<PropProof>),
    OrLeft(Box<PropProof>),
    OrRight(Box<PropProof>),
    OrElim(
        Box<PropProof>,
        String,
        Box<PropProof>,
        String,
        Box<PropProof>,
    ),
    Dne(Box<PropProof>),
    Absurd(Box<PropProof>),
    TrueIntro,
}
#[allow(dead_code)]
impl PropProof {
    pub fn depth(&self) -> usize {
        match self {
            PropProof::Hyp(_) | PropProof::TrueIntro => 0,
            PropProof::ImplIntro(_, p)
            | PropProof::AndLeft(p)
            | PropProof::AndRight(p)
            | PropProof::OrLeft(p)
            | PropProof::OrRight(p)
            | PropProof::Dne(p)
            | PropProof::Absurd(p) => 1 + p.depth(),
            PropProof::ImplElim(p, q) | PropProof::AndIntro(p, q) => 1 + p.depth().max(q.depth()),
            PropProof::OrElim(p, _, q, _, r) => 1 + p.depth().max(q.depth()).max(r.depth()),
        }
    }
    pub fn size(&self) -> usize {
        match self {
            PropProof::Hyp(_) | PropProof::TrueIntro => 1,
            PropProof::ImplIntro(_, p)
            | PropProof::AndLeft(p)
            | PropProof::AndRight(p)
            | PropProof::OrLeft(p)
            | PropProof::OrRight(p)
            | PropProof::Dne(p)
            | PropProof::Absurd(p) => 1 + p.size(),
            PropProof::ImplElim(p, q) | PropProof::AndIntro(p, q) => 1 + p.size() + q.size(),
            PropProof::OrElim(p, _, q, _, r) => 1 + p.size() + q.size() + r.size(),
        }
    }
}
/// A simple CDCL skeleton structure.
#[allow(dead_code)]
pub struct CdclSolver {
    pub num_vars: usize,
    pub clauses: Vec<Vec<(usize, bool)>>,
    pub learned_clauses: Vec<Vec<(usize, bool)>>,
    pub decision_level: usize,
}
#[allow(dead_code)]
impl CdclSolver {
    pub fn new(num_vars: usize) -> Self {
        CdclSolver {
            num_vars,
            clauses: Vec::new(),
            learned_clauses: Vec::new(),
            decision_level: 0,
        }
    }
    pub fn add_clause(&mut self, lits: Vec<(usize, bool)>) {
        self.clauses.push(lits);
    }
    pub fn learn_clause(&mut self, lits: Vec<(usize, bool)>) {
        self.learned_clauses.push(lits);
    }
    pub fn num_clauses(&self) -> usize {
        self.clauses.len() + self.learned_clauses.len()
    }
    /// Count literals across all clauses.
    pub fn total_literals(&self) -> usize {
        self.clauses
            .iter()
            .chain(self.learned_clauses.iter())
            .map(|c| c.len())
            .sum()
    }
    /// Check if a unit clause exists.
    pub fn has_unit_clause(&self, assignment: &[Option<bool>]) -> Option<(usize, bool)> {
        for clause in self.clauses.iter().chain(self.learned_clauses.iter()) {
            let mut unassigned = None;
            let mut falsified_count = 0;
            for &(var, pol) in clause {
                match assignment[var] {
                    Some(v) if v == pol => {
                        unassigned = None;
                        break;
                    }
                    Some(_) => {
                        falsified_count += 1;
                    }
                    None => {
                        if unassigned.is_some() {
                            unassigned = None;
                            break;
                        }
                        unassigned = Some((var, pol));
                    }
                }
            }
            if falsified_count == clause.len() - 1 {
                if let Some(u) = unassigned {
                    return Some(u);
                }
            }
        }
        None
    }
}
/// A new DPLL solver with instance methods.
#[allow(dead_code)]
pub struct ExtDpllSolver {
    pub num_vars: usize,
    pub clauses: Vec<Clause>,
}
#[allow(dead_code)]
impl ExtDpllSolver {
    pub fn new(num_vars: usize) -> Self {
        ExtDpllSolver {
            num_vars,
            clauses: Vec::new(),
        }
    }
    pub fn add_clause(&mut self, clause: Clause) {
        self.clauses.push(clause);
    }
    pub fn solve(&self) -> Option<Vec<bool>> {
        let mut assignment = vec![None; self.num_vars];
        if self.dpll_ext(&mut assignment) {
            Some(assignment.iter().map(|v| v.unwrap_or(false)).collect())
        } else {
            None
        }
    }
    fn dpll_ext(&self, assignment: &mut Vec<Option<bool>>) -> bool {
        loop {
            let mut propagated = false;
            for clause in &self.clauses {
                if clause.is_falsified_ext(assignment) {
                    return false;
                }
                if let Some((var, pol)) = clause.is_unit_ext(assignment) {
                    if assignment[var].is_none() {
                        assignment[var] = Some(pol);
                        propagated = true;
                    }
                }
            }
            if !propagated {
                break;
            }
        }
        if self.clauses.iter().all(|c| c.is_satisfied_ext(assignment)) {
            return true;
        }
        if self.clauses.iter().any(|c| c.is_falsified_ext(assignment)) {
            return false;
        }
        if let Some(idx) = assignment.iter().position(|v| v.is_none()) {
            for &val in &[true, false] {
                assignment[idx] = Some(val);
                if self.dpll_ext(assignment) {
                    return true;
                }
                assignment[idx] = None;
            }
        }
        false
    }
}
/// Detailed statistics about a propositional formula.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FormulaStats {
    pub atom_count: usize,
    pub unique_atoms: usize,
    pub connective_count: usize,
    pub depth: usize,
    pub and_count: usize,
    pub or_count: usize,
    pub not_count: usize,
    pub implies_count: usize,
    pub iff_count: usize,
}
#[allow(dead_code)]
impl FormulaStats {
    pub fn compute(f: &PropFormula) -> Self {
        let unique_atoms = f.atoms().len();
        let atom_count = count_total_atoms(f);
        let connective_count = count_connectives_ext(f);
        let depth = formula_depth_ext(f);
        let (and_c, or_c, not_c, impl_c, iff_c) = count_by_type(f);
        FormulaStats {
            atom_count,
            unique_atoms,
            connective_count,
            depth,
            and_count: and_c,
            or_count: or_c,
            not_count: not_c,
            implies_count: impl_c,
            iff_count: iff_c,
        }
    }
}
/// A counterexample to a formula: an assignment where the formula is false.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Counterexample {
    pub assignment: Vec<(String, bool)>,
    pub formula: String,
}
#[allow(dead_code)]
impl Counterexample {
    pub fn new(assignment: Vec<(String, bool)>, formula: String) -> Self {
        Counterexample {
            assignment,
            formula,
        }
    }
    pub fn num_vars(&self) -> usize {
        self.assignment.len()
    }
    pub fn get_value(&self, var: &str) -> Option<bool> {
        self.assignment
            .iter()
            .find(|(k, _)| k == var)
            .map(|(_, v)| *v)
    }
}
/// A proof tree node.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ProofTreeNode {
    pub formula: String,
    pub rule: String,
    pub children: Vec<ProofTreeNode>,
}
#[allow(dead_code)]
impl ProofTreeNode {
    pub fn leaf(formula: &str, rule: &str) -> Self {
        ProofTreeNode {
            formula: formula.to_string(),
            rule: rule.to_string(),
            children: vec![],
        }
    }
    pub fn inner(formula: &str, rule: &str, children: Vec<ProofTreeNode>) -> Self {
        ProofTreeNode {
            formula: formula.to_string(),
            rule: rule.to_string(),
            children,
        }
    }
    pub fn depth(&self) -> usize {
        if self.children.is_empty() {
            0
        } else {
            1 + self.children.iter().map(|c| c.depth()).max().unwrap_or(0)
        }
    }
    pub fn size(&self) -> usize {
        1 + self.children.iter().map(|c| c.size()).sum::<usize>()
    }
    pub fn leaves(&self) -> usize {
        if self.children.is_empty() {
            1
        } else {
            self.children.iter().map(|c| c.leaves()).sum()
        }
    }
}
/// Intuitionistic tautology prover using a simplified Dyckhoff / G4ip algorithm.
pub struct IntuitTactic;
impl IntuitTactic {
    pub fn new() -> Self {
        Self
    }
    /// Attempt to prove the formula intuitionistically.
    ///
    /// Uses a simple contraction-free sequent calculus (subset of G4ip).
    pub fn prove_intuitionist(&self, formula: &PropFormula) -> bool {
        self.provable(&[], formula, 30)
    }
    /// `ctx ⊢ goal` with a depth limit.
    fn provable(&self, ctx: &[PropFormula], goal: &PropFormula, depth: usize) -> bool {
        if depth == 0 {
            return false;
        }
        if matches!(goal, PropFormula::True) {
            return true;
        }
        if ctx.contains(goal) {
            return true;
        }
        if ctx.contains(&PropFormula::False) {
            return true;
        }
        for i in 0..ctx.len() {
            if let PropFormula::And(l, r) = &ctx[i] {
                let mut new_ctx: Vec<PropFormula> = ctx
                    .iter()
                    .enumerate()
                    .filter(|&(j, _)| j != i)
                    .map(|(_, f)| f.clone())
                    .collect();
                new_ctx.push(*l.clone());
                new_ctx.push(*r.clone());
                return self.provable(&new_ctx, goal, depth - 1);
            }
        }
        for i in 0..ctx.len() {
            if let PropFormula::Or(l, r) = &ctx[i] {
                let base: Vec<PropFormula> = ctx
                    .iter()
                    .enumerate()
                    .filter(|&(j, _)| j != i)
                    .map(|(_, f)| f.clone())
                    .collect();
                let mut left_ctx = base.clone();
                left_ctx.push(*l.clone());
                let mut right_ctx = base;
                right_ctx.push(*r.clone());
                return self.provable(&left_ctx, goal, depth - 1)
                    && self.provable(&right_ctx, goal, depth - 1);
            }
        }
        for i in 0..ctx.len() {
            if let PropFormula::Iff(a, b) = &ctx[i] {
                let mut new_ctx: Vec<PropFormula> = ctx
                    .iter()
                    .enumerate()
                    .filter(|&(j, _)| j != i)
                    .map(|(_, f)| f.clone())
                    .collect();
                new_ctx.push(PropFormula::Implies(a.clone(), b.clone()));
                new_ctx.push(PropFormula::Implies(b.clone(), a.clone()));
                return self.provable(&new_ctx, goal, depth - 1);
            }
        }
        match goal {
            PropFormula::And(l, r) => {
                self.provable(ctx, l, depth - 1) && self.provable(ctx, r, depth - 1)
            }
            PropFormula::Or(l, r) => {
                self.provable(ctx, l, depth - 1) || self.provable(ctx, r, depth - 1)
            }
            PropFormula::Implies(a, b) => {
                let mut new_ctx = ctx.to_vec();
                new_ctx.push(*a.clone());
                self.provable(&new_ctx, b, depth - 1)
            }
            PropFormula::Iff(a, b) => {
                let fwd = PropFormula::Implies(a.clone(), b.clone());
                let bwd = PropFormula::Implies(b.clone(), a.clone());
                self.provable(ctx, &fwd, depth - 1) && self.provable(ctx, &bwd, depth - 1)
            }
            PropFormula::Not(a) => {
                let mut new_ctx = ctx.to_vec();
                new_ctx.push(*a.clone());
                self.provable(&new_ctx, &PropFormula::False, depth - 1)
            }
            _ => {
                for i in 0..ctx.len() {
                    if let PropFormula::Implies(ant, cons) = &ctx[i] {
                        if self.provable(ctx, ant, depth - 1) {
                            let mut new_ctx = ctx.to_vec();
                            new_ctx.push(*cons.clone());
                            if self.provable(&new_ctx, goal, depth - 1) {
                                return true;
                            }
                        }
                    }
                }
                false
            }
        }
    }
}
/// A natural deduction derivation step.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum NdStep {
    /// Assumption: use a hypothesis.
    Assumption(String),
    /// Apply modus ponens: proof of A→B and proof of A to get B.
    ModusPonens(Box<NdStep>, Box<NdStep>),
    /// Introduce implication.
    ImplIntro(String, PropFormula, Box<NdStep>),
    /// Introduce conjunction.
    ConjIntro(Box<NdStep>, Box<NdStep>),
    /// Eliminate conjunction (left).
    ConjElimL(Box<NdStep>),
    /// Eliminate conjunction (right).
    ConjElimR(Box<NdStep>),
    /// Introduce disjunction (left).
    DisjIntroL(Box<NdStep>, PropFormula),
    /// Introduce disjunction (right).
    DisjIntroR(PropFormula, Box<NdStep>),
}
#[allow(dead_code)]
impl NdStep {
    pub fn depth(&self) -> usize {
        match self {
            NdStep::Assumption(_) => 0,
            NdStep::ImplIntro(_, _, p)
            | NdStep::ConjElimL(p)
            | NdStep::ConjElimR(p)
            | NdStep::DisjIntroL(p, _)
            | NdStep::DisjIntroR(_, p) => 1 + p.depth(),
            NdStep::ModusPonens(p, q) | NdStep::ConjIntro(p, q) => 1 + p.depth().max(q.depth()),
        }
    }
}
/// Truth-table based tautology checker.
pub struct TruthTable;
impl TruthTable {
    /// Evaluate the formula under a variable assignment.
    pub fn evaluate(formula: &PropFormula, assignment: &HashMap<String, bool>) -> bool {
        match formula {
            PropFormula::True => true,
            PropFormula::False => false,
            PropFormula::Atom(name) => *assignment.get(name).unwrap_or(&false),
            PropFormula::Not(inner) => !Self::evaluate(inner, assignment),
            PropFormula::And(l, r) => {
                Self::evaluate(l, assignment) && Self::evaluate(r, assignment)
            }
            PropFormula::Or(l, r) => Self::evaluate(l, assignment) || Self::evaluate(r, assignment),
            PropFormula::Implies(l, r) => {
                !Self::evaluate(l, assignment) || Self::evaluate(r, assignment)
            }
            PropFormula::Iff(l, r) => {
                Self::evaluate(l, assignment) == Self::evaluate(r, assignment)
            }
        }
    }
    /// Check whether the formula is a tautology by exhaustive truth table.
    pub fn is_tautology(formula: &PropFormula) -> bool {
        let atoms = formula.atoms();
        let n = atoms.len();
        for mask in 0u64..(1u64 << n) {
            let mut assignment = HashMap::new();
            for (i, atom) in atoms.iter().enumerate() {
                assignment.insert(atom.clone(), (mask >> i) & 1 == 1);
            }
            if !Self::evaluate(formula, &assignment) {
                return false;
            }
        }
        true
    }
}
/// Forward chaining (unit propagation) for Horn clauses.
#[allow(dead_code)]
pub struct HornSolver {
    pub clauses: Vec<HornClause>,
}
#[allow(dead_code)]
impl HornSolver {
    pub fn new() -> Self {
        HornSolver {
            clauses: Vec::new(),
        }
    }
    pub fn add_clause(&mut self, clause: HornClause) {
        self.clauses.push(clause);
    }
    /// Forward chaining: compute all derivable atoms.
    pub fn derive_all(&self) -> std::collections::HashSet<String> {
        let mut derived: std::collections::HashSet<String> = std::collections::HashSet::new();
        for c in &self.clauses {
            if c.is_fact() {
                if let Some(h) = &c.head {
                    derived.insert(h.clone());
                }
            }
        }
        let mut changed = true;
        while changed {
            changed = false;
            for c in &self.clauses {
                if let Some(h) = &c.head {
                    if !derived.contains(h) && c.body.iter().all(|b| derived.contains(b)) {
                        derived.insert(h.clone());
                        changed = true;
                    }
                }
            }
        }
        derived
    }
    /// Check if a goal atom is derivable.
    pub fn is_derivable(&self, goal: &str) -> bool {
        self.derive_all().contains(goal)
    }
}
/// A proof context for natural deduction.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct NdContext {
    pub hypotheses: Vec<(String, PropFormula)>,
}
#[allow(dead_code)]
impl NdContext {
    pub fn new() -> Self {
        NdContext {
            hypotheses: Vec::new(),
        }
    }
    pub fn add_hyp(&mut self, name: String, formula: PropFormula) {
        self.hypotheses.push((name, formula));
    }
    pub fn lookup(&self, name: &str) -> Option<&PropFormula> {
        self.hypotheses
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, f)| f)
    }
    pub fn contains_formula(&self, f: &PropFormula) -> bool {
        self.hypotheses.iter().any(|(_, h)| formula_eq_ext(h, f))
    }
    pub fn remove_hyp(&mut self, name: &str) {
        self.hypotheses.retain(|(n, _)| n != name);
    }
    pub fn size(&self) -> usize {
        self.hypotheses.len()
    }
}
/// Tautology prover using truth tables (for small formulas) or DPLL.
pub struct TautoTactic {
    /// Maximum number of atoms before switching to DPLL-based approach.
    pub(super) max_atoms: usize,
}
impl TautoTactic {
    pub fn new() -> Self {
        Self::default()
    }
    /// Return the atom limit for truth-table mode.
    pub fn max_atoms(&self) -> usize {
        self.max_atoms
    }
    /// Check if the given formula is a tautology.
    pub fn is_tautology(&self, formula: &PropFormula) -> bool {
        let atoms = formula.atoms();
        if atoms.len() <= self.max_atoms {
            TruthTable::is_tautology(formula)
        } else {
            let negated = formula.negate();
            let mut solver = DpllSolver::new();
            let clauses = solver.to_cnf(&negated);
            let num_vars = solver.next_var as usize;
            !DpllSolver::solve(clauses, num_vars)
        }
    }
    /// Parse and prove a simple formula string.
    ///
    /// Supported syntax: `A`, `¬A`, `A ∧ B`, `A ∨ B`, `A → B`, `A ↔ B`, `True`, `False`.
    /// Parentheses not yet supported in the mini-parser.
    pub fn prove(&self, formula_str: &str) -> bool {
        match Self::parse_simple(formula_str.trim()) {
            Some(f) => self.is_tautology(&f),
            None => false,
        }
    }
    fn parse_simple(s: &str) -> Option<PropFormula> {
        for sep in &["↔", "<->"] {
            if let Some(idx) = s.find(sep) {
                let l = Self::parse_simple(s[..idx].trim())?;
                let r = Self::parse_simple(s[idx + sep.len()..].trim())?;
                return Some(PropFormula::Iff(Box::new(l), Box::new(r)));
            }
        }
        for sep in &["→", "->"] {
            if let Some(idx) = s.find(sep) {
                let l = Self::parse_simple(s[..idx].trim())?;
                let r = Self::parse_simple(s[idx + sep.len()..].trim())?;
                return Some(PropFormula::Implies(Box::new(l), Box::new(r)));
            }
        }
        for sep in &["∨", "||", "or"] {
            if let Some(idx) = s.find(sep) {
                let l = Self::parse_simple(s[..idx].trim())?;
                let r = Self::parse_simple(s[idx + sep.len()..].trim())?;
                return Some(PropFormula::Or(Box::new(l), Box::new(r)));
            }
        }
        for sep in &["∧", "&&", "and"] {
            if let Some(idx) = s.find(sep) {
                let l = Self::parse_simple(s[..idx].trim())?;
                let r = Self::parse_simple(s[idx + sep.len()..].trim())?;
                return Some(PropFormula::And(Box::new(l), Box::new(r)));
            }
        }
        if s.starts_with('¬') {
            let inner = Self::parse_simple(s['¬'.len_utf8()..].trim())?;
            return Some(PropFormula::Not(Box::new(inner)));
        }
        if s.starts_with("not ") {
            let inner = Self::parse_simple(s.strip_prefix("not ").unwrap_or(s).trim())?;
            return Some(PropFormula::Not(Box::new(inner)));
        }
        match s {
            "True" | "true" => Some(PropFormula::True),
            "False" | "false" => Some(PropFormula::False),
            name if !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_') => {
                Some(PropFormula::Atom(name.to_string()))
            }
            _ => None,
        }
    }
}
/// A basic LTL formula over propositional atoms.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum LtlFormula {
    Atom(String),
    True,
    False,
    Not(Box<LtlFormula>),
    And(Box<LtlFormula>, Box<LtlFormula>),
    Or(Box<LtlFormula>, Box<LtlFormula>),
    Implies(Box<LtlFormula>, Box<LtlFormula>),
    /// Next: X f
    Next(Box<LtlFormula>),
    /// Globally: G f
    Globally(Box<LtlFormula>),
    /// Finally: F f
    Finally(Box<LtlFormula>),
    /// Until: f U g
    Until(Box<LtlFormula>, Box<LtlFormula>),
}
#[allow(dead_code)]
impl LtlFormula {
    /// Evaluate LTL formula on a finite trace (sequence of assignments).
    pub fn eval_on_trace(
        &self,
        trace: &[std::collections::HashMap<String, bool>],
        pos: usize,
    ) -> bool {
        if pos >= trace.len() {
            return false;
        }
        match self {
            LtlFormula::Atom(a) => *trace[pos].get(a).unwrap_or(&false),
            LtlFormula::True => true,
            LtlFormula::False => false,
            LtlFormula::Not(f) => !f.eval_on_trace(trace, pos),
            LtlFormula::And(f, g) => f.eval_on_trace(trace, pos) && g.eval_on_trace(trace, pos),
            LtlFormula::Or(f, g) => f.eval_on_trace(trace, pos) || g.eval_on_trace(trace, pos),
            LtlFormula::Implies(f, g) => {
                !f.eval_on_trace(trace, pos) || g.eval_on_trace(trace, pos)
            }
            LtlFormula::Next(f) => f.eval_on_trace(trace, pos + 1),
            LtlFormula::Globally(f) => (pos..trace.len()).all(|i| f.eval_on_trace(trace, i)),
            LtlFormula::Finally(f) => (pos..trace.len()).any(|i| f.eval_on_trace(trace, i)),
            LtlFormula::Until(f, g) => {
                for i in pos..trace.len() {
                    if g.eval_on_trace(trace, i) {
                        return true;
                    }
                    if !f.eval_on_trace(trace, i) {
                        return false;
                    }
                }
                false
            }
        }
    }
    pub fn depth(&self) -> usize {
        match self {
            LtlFormula::Atom(_) | LtlFormula::True | LtlFormula::False => 0,
            LtlFormula::Not(f)
            | LtlFormula::Next(f)
            | LtlFormula::Globally(f)
            | LtlFormula::Finally(f) => 1 + f.depth(),
            LtlFormula::And(f, g)
            | LtlFormula::Or(f, g)
            | LtlFormula::Implies(f, g)
            | LtlFormula::Until(f, g) => 1 + f.depth().max(g.depth()),
        }
    }
}
/// A clause in the new external DPLL solver style: a disjunction of literals.
/// A literal is (var_idx, polarity).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Clause {
    pub literals: Vec<(usize, bool)>,
}
#[allow(dead_code)]
impl Clause {
    pub fn new(lits: Vec<(usize, bool)>) -> Self {
        Clause { literals: lits }
    }
    pub fn is_unit_ext(&self, assignment: &[Option<bool>]) -> Option<(usize, bool)> {
        let mut unassigned = None;
        for &(var, pol) in &self.literals {
            match assignment[var] {
                Some(v) if v == pol => return None,
                Some(_) => {}
                None => {
                    if unassigned.is_some() {
                        return None;
                    }
                    unassigned = Some((var, pol));
                }
            }
        }
        unassigned
    }
    pub fn is_satisfied_ext(&self, assignment: &[Option<bool>]) -> bool {
        self.literals
            .iter()
            .any(|&(var, pol)| assignment[var] == Some(pol))
    }
    pub fn is_falsified_ext(&self, assignment: &[Option<bool>]) -> bool {
        self.literals
            .iter()
            .all(|&(var, pol)| assignment[var] == Some(!pol))
    }
}
/// A propositional formula.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PropFormula {
    True,
    False,
    Atom(String),
    Not(Box<PropFormula>),
    And(Box<PropFormula>, Box<PropFormula>),
    Or(Box<PropFormula>, Box<PropFormula>),
    Implies(Box<PropFormula>, Box<PropFormula>),
    Iff(Box<PropFormula>, Box<PropFormula>),
}
impl PropFormula {
    /// Negate this formula (pushes negation in one level).
    pub fn negate(&self) -> PropFormula {
        PropFormula::Not(Box::new(self.clone()))
    }
    /// Return `true` if the formula is an atom or negated atom.
    pub fn is_literal(&self) -> bool {
        match self {
            PropFormula::Atom(_) => true,
            PropFormula::Not(inner) => matches!(inner.as_ref(), PropFormula::Atom(_)),
            PropFormula::True | PropFormula::False => true,
            _ => false,
        }
    }
    /// Collect all atom names appearing in the formula (deduplication preserved via Vec order).
    pub fn atoms(&self) -> Vec<String> {
        let mut result = Vec::new();
        self.collect_atoms(&mut result);
        result
    }
    fn collect_atoms(&self, acc: &mut Vec<String>) {
        match self {
            PropFormula::Atom(name) => {
                if !acc.contains(name) {
                    acc.push(name.clone());
                }
            }
            PropFormula::Not(inner) => inner.collect_atoms(acc),
            PropFormula::And(l, r)
            | PropFormula::Or(l, r)
            | PropFormula::Implies(l, r)
            | PropFormula::Iff(l, r) => {
                l.collect_atoms(acc);
                r.collect_atoms(acc);
            }
            PropFormula::True | PropFormula::False => {}
        }
    }
}
/// Watch literal structure for efficient unit propagation.
#[allow(dead_code)]
pub struct WatchedClauses {
    /// For each literal (var * 2 + polarity), list of clause indices watching it.
    pub watches: Vec<Vec<usize>>,
    pub clauses: Vec<Vec<(usize, bool)>>,
}
#[allow(dead_code)]
impl WatchedClauses {
    pub fn new(num_vars: usize) -> Self {
        WatchedClauses {
            watches: vec![Vec::new(); num_vars * 2],
            clauses: Vec::new(),
        }
    }
    fn lit_idx(var: usize, pol: bool) -> usize {
        var * 2 + if pol { 1 } else { 0 }
    }
    pub fn add_clause(&mut self, lits: Vec<(usize, bool)>) {
        let idx = self.clauses.len();
        if !lits.is_empty() {
            let (v, p) = lits[0];
            self.watches[Self::lit_idx(v, p)].push(idx);
        }
        if lits.len() >= 2 {
            let (v, p) = lits[1];
            self.watches[Self::lit_idx(v, p)].push(idx);
        }
        self.clauses.push(lits);
    }
    pub fn num_watched_by(&self, var: usize, pol: bool) -> usize {
        self.watches[Self::lit_idx(var, pol)].len()
    }
}
/// A sequent: ante ⊢ suc.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Sequent {
    pub ante: Vec<PropFormula>,
    pub suc: Vec<PropFormula>,
}
#[allow(dead_code)]
impl Sequent {
    pub fn new(ante: Vec<PropFormula>, suc: Vec<PropFormula>) -> Self {
        Sequent { ante, suc }
    }
    pub fn is_axiom(&self) -> bool {
        for a in &self.ante {
            if self.suc.iter().any(|s| formula_eq_ext(a, s)) {
                return true;
            }
        }
        false
    }
    pub fn atoms_in_ante(&self) -> Vec<String> {
        let mut atoms = Vec::new();
        for f in &self.ante {
            for a in f.atoms() {
                if !atoms.contains(&a) {
                    atoms.push(a);
                }
            }
        }
        atoms
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormulaClass {
    Tautology,
    Contradiction,
    Satisfiable,
    Unknown,
}
/// A Horn clause: head :- body1, body2, ...
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HornClause {
    /// Head of the clause (always a positive atom, or None for integrity constraints).
    pub head: Option<String>,
    /// Body: a list of atoms (all positive for definite clauses).
    pub body: Vec<String>,
}
#[allow(dead_code)]
impl HornClause {
    pub fn fact(head: &str) -> Self {
        HornClause {
            head: Some(head.to_string()),
            body: vec![],
        }
    }
    pub fn rule(head: &str, body: Vec<&str>) -> Self {
        HornClause {
            head: Some(head.to_string()),
            body: body.into_iter().map(|s| s.to_string()).collect(),
        }
    }
    pub fn is_fact(&self) -> bool {
        self.body.is_empty()
    }
    pub fn is_definite(&self) -> bool {
        self.head.is_some()
    }
}
/// DPLL-based SAT solver (used to check unsatisfiability of negation).
pub struct DpllSolver {
    /// Next fresh variable index (beyond named atoms).
    pub(super) next_var: i32,
}
impl DpllSolver {
    pub fn new() -> Self {
        Self::default()
    }
    fn fresh(&mut self) -> i32 {
        let v = self.next_var;
        self.next_var += 1;
        v
    }
    /// Convert a `PropFormula` to CNF clauses.  Returns `(clauses, num_vars)`.
    pub fn to_cnf(&mut self, formula: &PropFormula) -> Vec<Vec<i32>> {
        let mut clauses: Vec<Vec<i32>> = Vec::new();
        let top = self.tseitin(formula, &mut clauses);
        clauses.push(vec![top]);
        clauses
    }
    fn tseitin(&mut self, formula: &PropFormula, clauses: &mut Vec<Vec<i32>>) -> i32 {
        match formula {
            PropFormula::True => {
                let v = self.fresh();
                clauses.push(vec![v]);
                v
            }
            PropFormula::False => {
                let v = self.fresh();
                clauses.push(vec![-v]);
                v
            }
            PropFormula::Atom(name) => {
                let mut h: i32 = 0;
                for b in name.bytes() {
                    h = h.wrapping_mul(31).wrapping_add(b as i32).abs();
                }
                let v = self.fresh();
                let _ = h;
                v
            }
            PropFormula::Not(inner) => {
                let inner_v = self.tseitin(inner, clauses);
                let v = self.fresh();
                clauses.push(vec![v, inner_v]);
                clauses.push(vec![-v, -inner_v]);
                v
            }
            PropFormula::And(l, r) => {
                let lv = self.tseitin(l, clauses);
                let rv = self.tseitin(r, clauses);
                let v = self.fresh();
                clauses.push(vec![-v, lv]);
                clauses.push(vec![-v, rv]);
                clauses.push(vec![v, -lv, -rv]);
                v
            }
            PropFormula::Or(l, r) => {
                let lv = self.tseitin(l, clauses);
                let rv = self.tseitin(r, clauses);
                let v = self.fresh();
                clauses.push(vec![-v, lv, rv]);
                clauses.push(vec![v, -lv]);
                clauses.push(vec![v, -rv]);
                v
            }
            PropFormula::Implies(l, r) => {
                let lv = self.tseitin(l, clauses);
                let rv = self.tseitin(r, clauses);
                let v = self.fresh();
                clauses.push(vec![-v, -lv, rv]);
                clauses.push(vec![v, lv]);
                clauses.push(vec![v, -rv]);
                v
            }
            PropFormula::Iff(l, r) => {
                let lv = self.tseitin(l, clauses);
                let rv = self.tseitin(r, clauses);
                let v = self.fresh();
                clauses.push(vec![-v, -lv, rv]);
                clauses.push(vec![-v, lv, -rv]);
                clauses.push(vec![v, -lv, -rv]);
                clauses.push(vec![v, lv, rv]);
                v
            }
        }
    }
    /// DPLL SAT solver.  Returns `true` if the clauses are satisfiable.
    pub fn solve(mut clauses: Vec<Vec<i32>>, _num_vars: usize) -> bool {
        Self::dpll(&mut clauses)
    }
    fn dpll(clauses: &mut Vec<Vec<i32>>) -> bool {
        loop {
            let before = clauses.len();
            if !Self::unit_propagate(clauses) {
                return false;
            }
            if clauses.len() == before {
                break;
            }
        }
        if clauses.is_empty() {
            return true;
        }
        let lit = match clauses.iter().flat_map(|c| c.iter()).next() {
            Some(&l) => l.abs(),
            None => return true,
        };
        let mut branch_true = clauses
            .iter()
            .filter(|c| !c.contains(&lit))
            .map(|c| {
                c.iter()
                    .filter(|&&l| l != -lit)
                    .cloned()
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        if Self::dpll(&mut branch_true) {
            return true;
        }
        let neg = -lit;
        let mut branch_false = clauses
            .iter()
            .filter(|c| !c.contains(&neg))
            .map(|c| c.iter().filter(|&&l| l != lit).cloned().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Self::dpll(&mut branch_false)
    }
    /// Perform one round of unit propagation.  Returns `false` on conflict.
    pub fn unit_propagate(clauses: &mut Vec<Vec<i32>>) -> bool {
        loop {
            let unit_lit = clauses.iter().find(|c| c.len() == 1).map(|c| c[0]);
            match unit_lit {
                None => return true,
                Some(lit) => {
                    if clauses.iter().any(|c| c == &[-lit]) {
                        return false;
                    }
                    clauses.retain(|c| !c.contains(&lit));
                    for clause in clauses.iter_mut() {
                        clause.retain(|&l| l != -lit);
                    }
                    if clauses.iter().any(|c| c.is_empty()) {
                        return false;
                    }
                }
            }
        }
    }
}
