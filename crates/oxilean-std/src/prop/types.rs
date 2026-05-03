//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
/// A simple truth table checker for propositional formulas over a fixed number of variables.
///
/// Checks if a formula is a tautology, satisfiable, or contradictory by exhaustive
/// enumeration of truth assignments.
#[allow(dead_code)]
pub struct TruthTableChecker {
    /// Number of propositional variables.
    pub num_vars: usize,
}
#[allow(dead_code)]
impl TruthTableChecker {
    /// Create a checker for `n` variables.
    pub fn new(n: usize) -> Self {
        Self { num_vars: n }
    }
    /// Enumerate all truth assignments.
    pub fn all_assignments(&self) -> Vec<Vec<bool>> {
        let n = 1usize << self.num_vars;
        (0..n)
            .map(|mask| (0..self.num_vars).map(|i| (mask >> i) & 1 == 1).collect())
            .collect()
    }
    /// Check if a formula (given as evaluator) is a tautology.
    pub fn is_tautology(&self, formula: &NnfFormula) -> bool {
        self.all_assignments().iter().all(|a| formula.eval(a))
    }
    /// Check if a formula is satisfiable.
    pub fn is_satisfiable(&self, formula: &NnfFormula) -> bool {
        self.all_assignments().iter().any(|a| formula.eval(a))
    }
    /// Check if a formula is a contradiction.
    pub fn is_contradiction(&self, formula: &NnfFormula) -> bool {
        !self.is_satisfiable(formula)
    }
    /// Find a satisfying assignment, if one exists.
    pub fn find_satisfying_assignment(&self, formula: &NnfFormula) -> Option<Vec<bool>> {
        self.all_assignments().into_iter().find(|a| formula.eval(a))
    }
    /// Count the number of satisfying assignments.
    pub fn count_satisfying(&self, formula: &NnfFormula) -> usize {
        self.all_assignments()
            .iter()
            .filter(|a| formula.eval(a))
            .count()
    }
}
/// A propositional formula in NNF (Negation Normal Form).
///
/// In NNF, negations appear only at atoms (literals). This simplifies
/// many proof-theoretic manipulations.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NnfFormula {
    /// A positive atom (propositional variable index).
    Atom(usize),
    /// A negated atom.
    NegAtom(usize),
    /// Conjunction.
    And(Box<NnfFormula>, Box<NnfFormula>),
    /// Disjunction.
    Or(Box<NnfFormula>, Box<NnfFormula>),
    /// The constant True.
    Top,
    /// The constant False.
    Bot,
}
#[allow(dead_code)]
impl NnfFormula {
    /// Check if the formula is a literal (atom or negated atom).
    pub fn is_literal(&self) -> bool {
        matches!(self, NnfFormula::Atom(_) | NnfFormula::NegAtom(_))
    }
    /// Count the number of atoms in the formula.
    pub fn atom_count(&self) -> usize {
        match self {
            NnfFormula::Atom(_) | NnfFormula::NegAtom(_) => 1,
            NnfFormula::And(a, b) | NnfFormula::Or(a, b) => a.atom_count() + b.atom_count(),
            NnfFormula::Top | NnfFormula::Bot => 0,
        }
    }
    /// Count the number of connectives in the formula.
    pub fn connective_count(&self) -> usize {
        match self {
            NnfFormula::Atom(_) | NnfFormula::NegAtom(_) | NnfFormula::Top | NnfFormula::Bot => 0,
            NnfFormula::And(a, b) | NnfFormula::Or(a, b) => {
                1 + a.connective_count() + b.connective_count()
            }
        }
    }
    /// Collect all variable indices appearing in the formula.
    pub fn variables(&self) -> Vec<usize> {
        let mut vars = Vec::new();
        self.collect_vars(&mut vars);
        vars.sort();
        vars.dedup();
        vars
    }
    fn collect_vars(&self, acc: &mut Vec<usize>) {
        match self {
            NnfFormula::Atom(i) | NnfFormula::NegAtom(i) => acc.push(*i),
            NnfFormula::And(a, b) | NnfFormula::Or(a, b) => {
                a.collect_vars(acc);
                b.collect_vars(acc);
            }
            _ => {}
        }
    }
    /// Evaluate the formula under a truth assignment (variable index → bool).
    pub fn eval(&self, assignment: &[bool]) -> bool {
        match self {
            NnfFormula::Atom(i) => assignment.get(*i).copied().unwrap_or(false),
            NnfFormula::NegAtom(i) => !assignment.get(*i).copied().unwrap_or(false),
            NnfFormula::And(a, b) => a.eval(assignment) && b.eval(assignment),
            NnfFormula::Or(a, b) => a.eval(assignment) || b.eval(assignment),
            NnfFormula::Top => true,
            NnfFormula::Bot => false,
        }
    }
    /// Simplify by eliminating Top/Bot.
    pub fn simplify(self) -> NnfFormula {
        match self {
            NnfFormula::And(a, b) => {
                let a = a.simplify();
                let b = b.simplify();
                match (&a, &b) {
                    (NnfFormula::Bot, _) | (_, NnfFormula::Bot) => NnfFormula::Bot,
                    (NnfFormula::Top, _) => b,
                    (_, NnfFormula::Top) => a,
                    _ => NnfFormula::And(Box::new(a), Box::new(b)),
                }
            }
            NnfFormula::Or(a, b) => {
                let a = a.simplify();
                let b = b.simplify();
                match (&a, &b) {
                    (NnfFormula::Top, _) | (_, NnfFormula::Top) => NnfFormula::Top,
                    (NnfFormula::Bot, _) => b,
                    (_, NnfFormula::Bot) => a,
                    _ => NnfFormula::Or(Box::new(a), Box::new(b)),
                }
            }
            other => other,
        }
    }
}
/// A sequent in propositional sequent calculus (LK/LJ).
///
/// A sequent `Γ ⊢ Δ` consists of a context (antecedent) and a conclusion (succedent).
/// For LJ (intuitionistic), `Δ` has at most one formula.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Sequent {
    /// Antecedent: list of assumption formulas.
    pub antecedent: Vec<NnfFormula>,
    /// Succedent: list of conclusion formulas (at most one for intuitionistic).
    pub succedent: Vec<NnfFormula>,
}
#[allow(dead_code)]
impl Sequent {
    /// Create an empty sequent ⊢ .
    pub fn empty() -> Self {
        Self {
            antecedent: vec![],
            succedent: vec![],
        }
    }
    /// Create a sequent from antecedent and succedent.
    pub fn new(antecedent: Vec<NnfFormula>, succedent: Vec<NnfFormula>) -> Self {
        Self {
            antecedent,
            succedent,
        }
    }
    /// Create an axiom sequent: p ⊢ p.
    pub fn axiom(p: NnfFormula) -> Self {
        Self {
            antecedent: vec![p.clone()],
            succedent: vec![p],
        }
    }
    /// Check if this is an initial sequent (antecedent and succedent share a formula).
    pub fn is_initial(&self) -> bool {
        for a in &self.antecedent {
            if self.succedent.contains(a) {
                return true;
            }
        }
        false
    }
    /// Check if this sequent is classically valid (all atoms satisfied under any assignment).
    pub fn is_classically_valid(&self, num_vars: usize) -> bool {
        let checker = TruthTableChecker::new(num_vars);
        checker.all_assignments().iter().all(|assign| {
            let ant_false = self.antecedent.iter().any(|f| !f.eval(assign));
            let suc_true = self.succedent.iter().any(|f| f.eval(assign));
            ant_false || suc_true
        })
    }
    /// Number of formulas in the sequent.
    pub fn size(&self) -> usize {
        self.antecedent.len() + self.succedent.len()
    }
    /// Add a formula to the antecedent.
    pub fn with_assumption(mut self, f: NnfFormula) -> Self {
        self.antecedent.push(f);
        self
    }
    /// Add a formula to the succedent.
    pub fn with_conclusion(mut self, f: NnfFormula) -> Self {
        self.succedent.push(f);
        self
    }
    /// Check if the antecedent contains Bot (False), making the sequent trivially valid.
    pub fn has_false_in_antecedent(&self) -> bool {
        self.antecedent.contains(&NnfFormula::Bot)
    }
    /// Check if the succedent contains Top (True), making the sequent trivially valid.
    pub fn has_true_in_succedent(&self) -> bool {
        self.succedent.contains(&NnfFormula::Top)
    }
}
/// A Kripke model for modal propositional logic.
///
/// A Kripke model M = (W, R, V) consists of:
/// - W: a set of worlds (indexed 0..n)
/// - R: an accessibility relation (as adjacency matrix)
/// - V: a valuation assigning truth values to atoms at each world
#[allow(dead_code)]
pub struct KripkeModel {
    /// Number of worlds.
    pub num_worlds: usize,
    /// Number of propositional variables.
    pub num_vars: usize,
    /// Accessibility relation: access\[i\]\[j\] = world i can see world j.
    pub access: Vec<Vec<bool>>,
    /// Valuation: val\[w\]\[p\] = atom p is true at world w.
    pub val: Vec<Vec<bool>>,
}
#[allow(dead_code)]
impl KripkeModel {
    /// Create a Kripke model with reflexive accessibility (for T axiom).
    pub fn reflexive(num_worlds: usize, num_vars: usize) -> Self {
        let mut access = vec![vec![false; num_worlds]; num_worlds];
        for i in 0..num_worlds {
            access[i][i] = true;
        }
        let val = vec![vec![false; num_vars]; num_worlds];
        Self {
            num_worlds,
            num_vars,
            access,
            val,
        }
    }
    /// Set the valuation of variable `var` at world `world`.
    pub fn set_val(&mut self, world: usize, var: usize, truth: bool) {
        if world < self.num_worlds && var < self.num_vars {
            self.val[world][var] = truth;
        }
    }
    /// Set the accessibility from world `from` to world `to`.
    pub fn set_access(&mut self, from: usize, to: usize) {
        if from < self.num_worlds && to < self.num_worlds {
            self.access[from][to] = true;
        }
    }
    /// Evaluate an NnfFormula at a given world (treating Box as necessity over accessible worlds).
    pub fn satisfies_at(&self, formula: &NnfFormula, world: usize) -> bool {
        formula.eval(&self.val[world])
    }
    /// Check if the model is reflexive (for T axiom validity).
    pub fn is_reflexive(&self) -> bool {
        (0..self.num_worlds).all(|i| self.access[i][i])
    }
    /// Check if the model is transitive (for S4 axiom validity).
    pub fn is_transitive(&self) -> bool {
        for i in 0..self.num_worlds {
            for j in 0..self.num_worlds {
                for k in 0..self.num_worlds {
                    if self.access[i][j] && self.access[j][k] && !self.access[i][k] {
                        return false;
                    }
                }
            }
        }
        true
    }
    /// Check if the model is symmetric (for B axiom validity).
    pub fn is_symmetric(&self) -> bool {
        for i in 0..self.num_worlds {
            for j in 0..self.num_worlds {
                if self.access[i][j] && !self.access[j][i] {
                    return false;
                }
            }
        }
        true
    }
    /// Check if the model is Euclidean (for S5 axiom validity).
    pub fn is_euclidean(&self) -> bool {
        for i in 0..self.num_worlds {
            for j in 0..self.num_worlds {
                for k in 0..self.num_worlds {
                    if self.access[i][j] && self.access[i][k] && !self.access[j][k] {
                        return false;
                    }
                }
            }
        }
        true
    }
    /// Number of worlds accessible from a given world.
    pub fn accessible_count(&self, world: usize) -> usize {
        if world < self.num_worlds {
            self.access[world].iter().filter(|&&b| b).count()
        } else {
            0
        }
    }
}
