//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, InductiveEnv, Level, Name};

/// A simple Kleene three-valued logic evaluator.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kleene3Val {
    /// Definitely false.
    False,
    /// Unknown/undecided.
    Unknown,
    /// Definitely true.
    True,
}
impl Kleene3Val {
    /// Kleene NOT operation.
    #[allow(dead_code)]
    pub fn not(self) -> Self {
        match self {
            Kleene3Val::True => Kleene3Val::False,
            Kleene3Val::False => Kleene3Val::True,
            Kleene3Val::Unknown => Kleene3Val::Unknown,
        }
    }
    /// Kleene AND operation: min of values.
    #[allow(dead_code)]
    pub fn and(self, other: Self) -> Self {
        match (self, other) {
            (Kleene3Val::False, _) | (_, Kleene3Val::False) => Kleene3Val::False,
            (Kleene3Val::True, Kleene3Val::True) => Kleene3Val::True,
            _ => Kleene3Val::Unknown,
        }
    }
    /// Kleene OR operation: max of values.
    #[allow(dead_code)]
    pub fn or(self, other: Self) -> Self {
        match (self, other) {
            (Kleene3Val::True, _) | (_, Kleene3Val::True) => Kleene3Val::True,
            (Kleene3Val::False, Kleene3Val::False) => Kleene3Val::False,
            _ => Kleene3Val::Unknown,
        }
    }
    /// Convert from Bool to Kleene3.
    #[allow(dead_code)]
    pub fn from_bool(b: bool) -> Self {
        if b {
            Kleene3Val::True
        } else {
            Kleene3Val::False
        }
    }
}
/// A complete truth table for a 2-input boolean function.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TruthTable {
    /// All four entries (FF, FT, TF, TT).
    pub entries: [TruthTableEntry; 4],
    /// Name of the operation.
    pub name: &'static str,
}
impl TruthTable {
    /// Compute the truth table for a given 2-input function.
    #[allow(dead_code)]
    pub fn compute(name: &'static str, f: fn(bool, bool) -> bool) -> Self {
        Self {
            name,
            entries: [
                TruthTableEntry {
                    a: false,
                    b: false,
                    output: f(false, false),
                },
                TruthTableEntry {
                    a: false,
                    b: true,
                    output: f(false, true),
                },
                TruthTableEntry {
                    a: true,
                    b: false,
                    output: f(true, false),
                },
                TruthTableEntry {
                    a: true,
                    b: true,
                    output: f(true, true),
                },
            ],
        }
    }
    /// Look up the output for given inputs.
    #[allow(dead_code)]
    pub fn lookup(&self, a: bool, b: bool) -> bool {
        self.entries
            .iter()
            .find(|e| e.a == a && e.b == b)
            .map(|e| e.output)
            .expect("Truth table must have all 4 entries")
    }
    /// Check if the function is commutative: f(a,b) == f(b,a) for all inputs.
    #[allow(dead_code)]
    pub fn is_commutative(&self) -> bool {
        self.lookup(true, false) == self.lookup(false, true)
    }
    /// Check if the function always returns true (tautology).
    #[allow(dead_code)]
    pub fn is_tautology(&self) -> bool {
        self.entries.iter().all(|e| e.output)
    }
    /// Check if the function always returns false (contradiction).
    #[allow(dead_code)]
    pub fn is_contradiction(&self) -> bool {
        self.entries.iter().all(|e| !e.output)
    }
    /// Count the number of true outputs.
    #[allow(dead_code)]
    pub fn true_count(&self) -> usize {
        self.entries.iter().filter(|e| e.output).count()
    }
}
/// Decidable predicate: a predicate P: T -> Bool computable at runtime.
#[allow(dead_code)]
pub struct DecidablePred<T> {
    /// The underlying decision procedure.
    pub decide: Box<dyn Fn(&T) -> bool>,
    /// Name of this predicate for display.
    pub name: String,
}
/// SAT instance: a propositional formula in conjunctive normal form.
#[allow(dead_code)]
pub struct SATInstance {
    /// Number of variables.
    pub num_vars: usize,
    /// Clauses as disjunctions of literals (positive = var index, negative = -(var index+1)).
    pub clauses: Vec<Vec<i32>>,
}
impl SATInstance {
    /// Create a new empty SAT instance with given number of variables.
    #[allow(dead_code)]
    pub fn new(num_vars: usize) -> Self {
        SATInstance {
            num_vars,
            clauses: Vec::new(),
        }
    }
    /// Add a clause to the SAT instance.
    #[allow(dead_code)]
    pub fn add_clause(&mut self, clause: Vec<i32>) {
        self.clauses.push(clause);
    }
    /// Naive SAT solver by exhaustive enumeration.
    #[allow(dead_code)]
    pub fn solve(&self) -> Option<Vec<bool>> {
        let n = self.num_vars;
        for mask in 0..(1u64 << n) {
            let assignment: Vec<bool> = (0..n).map(|i| (mask >> i) & 1 == 1).collect();
            if self.evaluate(&assignment) {
                return Some(assignment);
            }
        }
        None
    }
    /// Evaluate all clauses under a given assignment.
    #[allow(dead_code)]
    pub fn evaluate(&self, assignment: &[bool]) -> bool {
        self.clauses.iter().all(|clause| {
            clause.iter().any(|&lit| {
                if lit > 0 {
                    assignment.get((lit - 1) as usize).copied().unwrap_or(false)
                } else {
                    !assignment.get((-lit - 1) as usize).copied().unwrap_or(true)
                }
            })
        })
    }
}
/// A boolean expression tree used to represent propositional formulas.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum BoolExpr {
    /// A constant boolean value.
    Const(bool),
    /// A named variable.
    Var(String),
    /// Logical NOT.
    Not(Box<BoolExpr>),
    /// Logical AND.
    And(Box<BoolExpr>, Box<BoolExpr>),
    /// Logical OR.
    Or(Box<BoolExpr>, Box<BoolExpr>),
    /// Logical XOR.
    Xor(Box<BoolExpr>, Box<BoolExpr>),
    /// Logical implication (a → b = ¬a ∨ b).
    Implies(Box<BoolExpr>, Box<BoolExpr>),
    /// Logical IFF (biconditional).
    Iff(Box<BoolExpr>, Box<BoolExpr>),
}
impl BoolExpr {
    /// Evaluate the expression under a variable assignment.
    ///
    /// Returns `None` if any variable is unassigned.
    #[allow(dead_code)]
    pub fn eval(&self, env: &std::collections::HashMap<&str, bool>) -> Option<bool> {
        match self {
            BoolExpr::Const(b) => Some(*b),
            BoolExpr::Var(name) => env.get(name.as_str()).copied(),
            BoolExpr::Not(e) => e.eval(env).map(|b| !b),
            BoolExpr::And(l, r) => Some(l.eval(env)? && r.eval(env)?),
            BoolExpr::Or(l, r) => Some(l.eval(env)? || r.eval(env)?),
            BoolExpr::Xor(l, r) => Some(l.eval(env)? ^ r.eval(env)?),
            BoolExpr::Implies(l, r) => Some(!l.eval(env)? || r.eval(env)?),
            BoolExpr::Iff(l, r) => Some(l.eval(env)? == r.eval(env)?),
        }
    }
    /// Collect all variable names in this expression.
    #[allow(dead_code)]
    pub fn variables(&self) -> Vec<String> {
        let mut vars = Vec::new();
        self.collect_vars(&mut vars);
        vars
    }
    fn collect_vars(&self, vars: &mut Vec<String>) {
        match self {
            BoolExpr::Var(name) => {
                if !vars.contains(name) {
                    vars.push(name.clone());
                }
            }
            BoolExpr::Not(e) => e.collect_vars(vars),
            BoolExpr::And(l, r)
            | BoolExpr::Or(l, r)
            | BoolExpr::Xor(l, r)
            | BoolExpr::Implies(l, r)
            | BoolExpr::Iff(l, r) => {
                l.collect_vars(vars);
                r.collect_vars(vars);
            }
            BoolExpr::Const(_) => {}
        }
    }
    /// Check if the expression is a tautology by enumerating all assignments.
    #[allow(dead_code)]
    pub fn is_tautology(&self) -> bool {
        let vars = self.variables();
        let n = vars.len();
        for mask in 0..(1u64 << n) {
            let mut env = std::collections::HashMap::new();
            for (i, var) in vars.iter().enumerate() {
                env.insert(var.as_str(), (mask >> i) & 1 == 1);
            }
            if self.eval(&env) != Some(true) {
                return false;
            }
        }
        true
    }
}
/// XOR monoid structure: (Bool, XOR, false) is a commutative group.
#[allow(dead_code)]
pub struct XorMonoid {
    /// Identity element is false.
    pub identity: bool,
    /// XOR is associative and commutative.
    pub associative: bool,
    /// Every element is its own inverse: a XOR a = false.
    pub self_inverse: bool,
}
/// Boolean algebra structure witnessing the laws of a Boolean algebra.
#[allow(dead_code)]
pub struct BoolAlgebra {
    /// The carrier set size (2 for the standard Boolean algebra B2).
    pub carrier_size: usize,
    /// Whether complementation is involutive: ¬¬a = a.
    pub involutive: bool,
    /// Whether the structure satisfies De Morgan laws.
    pub de_morgan: bool,
}
/// Boolean lattice structure: (Bool, OR=join, AND=meet, false=bot, true=top).
#[allow(dead_code)]
pub struct BoolLattice {
    /// Bottom element (false).
    pub bottom: bool,
    /// Top element (true).
    pub top: bool,
    /// Is this a distributive lattice?
    pub distributive: bool,
    /// Is this a complemented lattice?
    pub complemented: bool,
}
/// A truth table entry for a 2-input boolean function.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TruthTableEntry {
    /// Input a.
    pub a: bool,
    /// Input b.
    pub b: bool,
    /// Output value.
    pub output: bool,
}
