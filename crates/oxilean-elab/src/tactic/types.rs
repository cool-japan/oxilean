//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Expr, Literal, Name};
use std::collections::HashMap;

use super::functions::*;
use super::functions_2::eval_tactic;

/// A tactic that transforms proof goals.
#[derive(Debug, Clone)]
pub struct Tactic {
    /// Tactic name
    pub name: Name,
    /// Tactic description
    pub description: String,
}
/// Errors that can occur during tactic execution.
#[derive(Clone, Debug)]
pub enum TacticError {
    /// Goal with the given name was not found
    GoalNotFound(Name),
    /// No goals remain to work on
    NoGoals,
    /// Expected exactly one goal, but found more
    TooManyGoals,
    /// Type mismatch during tactic application
    TypeMismatch(String),
    /// Unknown tactic name
    UnknownTactic(String),
    /// Invalid argument to a tactic
    InvalidArg(String),
    /// Internal error in the tactic engine
    InternalError(String),
}
/// Registry for built-in tactics.
pub struct TacticRegistry {
    /// Registered tactics
    tactics: HashMap<Name, TacticInfo>,
}
impl TacticRegistry {
    /// Create a new tactic registry.
    pub fn new() -> Self {
        Self {
            tactics: HashMap::new(),
        }
    }
    /// Register a tactic with optional arity.
    #[allow(dead_code)]
    pub fn register_with_arity(&mut self, tactic: Tactic, arity: Option<usize>) {
        let name = tactic.name.clone();
        self.tactics.insert(name, TacticInfo { tactic, arity });
    }
    /// Register a tactic.
    pub fn register(&mut self, tactic: Tactic) {
        let name = tactic.name.clone();
        self.tactics.insert(
            name,
            TacticInfo {
                tactic,
                arity: None,
            },
        );
    }
    /// Get a tactic by name.
    pub fn get(&self, name: &Name) -> Option<&Tactic> {
        self.tactics.get(name).map(|info| &info.tactic)
    }
    /// Get the arity of a tactic.
    #[allow(dead_code)]
    pub fn arity(&self, name: &Name) -> Option<Option<usize>> {
        self.tactics.get(name).map(|info| info.arity)
    }
    /// Get all registered tactics.
    pub fn all_tactics(&self) -> Vec<&Tactic> {
        self.tactics.values().map(|info| &info.tactic).collect()
    }
    /// Execute a tactic by name.
    #[allow(dead_code)]
    pub fn execute(
        &self,
        name: &str,
        state: &TacticState,
        _args: &[Expr],
        env: &oxilean_kernel::Environment,
    ) -> TacticResult {
        let tactic_name = Name::str(name);
        if self.tactics.contains_key(&tactic_name) {
            eval_tactic(state, name, env)
        } else {
            Err(TacticError::UnknownTactic(name.to_string()))
        }
    }
}
/// Describes the high-level shape of a type for tactic dispatch.
pub enum TypeShape {
    /// `False`
    False,
    /// `And A B`
    And(Expr, Expr),
    /// `Or A B`
    Or(Expr, Expr),
    /// `Iff A B`
    Iff(Expr, Expr),
    /// `Nat`
    Nat,
    /// `Exists A P`
    Exists(Expr, Expr),
    /// `Bool`
    Bool,
    /// `Option A`
    Option(Expr),
    /// `List A`
    List(Expr),
    /// `Prod A B`
    Prod(Expr, Expr),
    /// `Sum A B`
    Sum(Expr, Expr),
    /// `Eq A a b`
    Eq(Expr, Expr, Expr),
    /// Unrecognised shape
    Other,
}
/// A proof goal to be solved.
#[derive(Debug, Clone)]
pub struct Goal {
    /// Goal name
    pub name: Name,
    /// Metavariable ID this goal is solving
    pub mvar_id: u64,
    /// Hypotheses (assumptions): (name, type)
    pub hypotheses: Vec<(Name, Expr)>,
    /// Local context with optional values: (name, type, optional_value)
    pub local_ctx: Vec<(Name, Expr, Option<Expr>)>,
    /// Target type to prove
    pub target: Expr,
    /// Optional goal tag/label
    pub tag: Option<String>,
}
impl Goal {
    /// Create a new goal.
    pub fn new(name: Name, target: Expr) -> Self {
        Self {
            name,
            mvar_id: fresh_mvar_id(),
            hypotheses: Vec::new(),
            local_ctx: Vec::new(),
            target,
            tag: None,
        }
    }
    /// Create a new goal with a specific mvar_id.
    #[allow(dead_code)]
    pub fn with_mvar_id(name: Name, mvar_id: u64, target: Expr) -> Self {
        Self {
            name,
            mvar_id,
            hypotheses: Vec::new(),
            local_ctx: Vec::new(),
            target,
            tag: None,
        }
    }
    /// Add a hypothesis.
    pub fn add_hypothesis(&mut self, name: Name, ty: Expr) {
        self.hypotheses.push((name.clone(), ty.clone()));
        self.local_ctx.push((name, ty, None));
    }
    /// Add a hypothesis with a value (a let-bound hypothesis).
    #[allow(dead_code)]
    pub fn add_hypothesis_with_value(&mut self, name: Name, ty: Expr, val: Expr) {
        self.hypotheses.push((name.clone(), ty.clone()));
        self.local_ctx.push((name, ty, Some(val)));
    }
    /// Get all hypotheses.
    pub fn hypotheses(&self) -> &[(Name, Expr)] {
        &self.hypotheses
    }
    /// Get the target.
    pub fn target(&self) -> &Expr {
        &self.target
    }
    /// Create a new goal with an additional hypothesis.
    #[allow(dead_code)]
    pub fn with_hypothesis(&self, name: Name, ty: Expr) -> Self {
        let mut new_goal = self.clone();
        new_goal.add_hypothesis(name, ty);
        new_goal
    }
    /// Create a new goal with a replaced target.
    #[allow(dead_code)]
    pub fn replace_target(&self, new_target: Expr) -> Self {
        let mut new_goal = self.clone();
        new_goal.target = new_target;
        new_goal.mvar_id = fresh_mvar_id();
        new_goal
    }
    /// Check if a hypothesis with the given name exists.
    #[allow(dead_code)]
    pub fn has_hypothesis(&self, name: &Name) -> bool {
        self.hypotheses.iter().any(|(n, _)| n == name)
    }
    /// Find a hypothesis by name.
    #[allow(dead_code)]
    pub fn find_hypothesis(&self, name: &Name) -> Option<&Expr> {
        self.hypotheses
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, ty)| ty)
    }
    /// Get all local names in this goal.
    #[allow(dead_code)]
    pub fn local_names(&self) -> Vec<&Name> {
        self.hypotheses.iter().map(|(n, _)| n).collect()
    }
}
/// A snapshot of the tactic state that can be restored for backtracking.
#[derive(Clone, Debug)]
pub struct SavedState {
    /// Saved goals
    goals: Vec<Goal>,
    /// Saved solved goal names
    pub solved: Vec<Name>,
}
/// A normalised linear expression: `Σᵢ coeffs[i].1 * coeffs[i].0 + constant`.
/// Variable names are sorted; zero-coefficient terms are omitted.
#[derive(Clone, Debug, Default)]
pub struct SymLinExpr {
    /// (variable_key, coefficient), sorted by variable_key, non-zero coefficients only.
    pub terms: Vec<(String, i64)>,
    pub constant: i64,
}
impl SymLinExpr {
    pub fn from_const(k: i64) -> Self {
        Self {
            terms: vec![],
            constant: k,
        }
    }
    pub fn from_var(name: String) -> Self {
        Self {
            terms: vec![(name, 1)],
            constant: 0,
        }
    }
    pub fn negate(&self) -> Self {
        Self {
            terms: self.terms.iter().map(|(v, c)| (v.clone(), -c)).collect(),
            constant: -self.constant,
        }
    }
    pub fn scale(&self, factor: i64) -> Self {
        if factor == 0 {
            return Self::default();
        }
        Self {
            terms: self
                .terms
                .iter()
                .map(|(v, c)| (v.clone(), c * factor))
                .collect(),
            constant: self.constant * factor,
        }
    }
    pub fn add(a: &Self, b: &Self) -> Self {
        let mut map: HashMap<String, i64> = HashMap::new();
        for (v, c) in &a.terms {
            *map.entry(v.clone()).or_default() += c;
        }
        for (v, c) in &b.terms {
            *map.entry(v.clone()).or_default() += c;
        }
        let mut terms: Vec<_> = map.into_iter().filter(|(_, c)| *c != 0).collect();
        terms.sort_by(|(a, _), (b, _)| a.cmp(b));
        Self {
            terms,
            constant: a.constant + b.constant,
        }
    }
}
/// A linear constraint `lhs OP 0` where OP is ≤ (strict=false) or < (strict=true).
#[derive(Clone, Debug)]
pub struct SymLinCon {
    pub lhs: SymLinExpr,
    pub strict: bool,
}
impl SymLinCon {
    pub fn add(a: &Self, b: &Self) -> Self {
        Self {
            lhs: SymLinExpr::add(&a.lhs, &b.lhs),
            strict: a.strict || b.strict,
        }
    }
    pub fn scale(&self, factor: i64) -> Self {
        assert!(factor > 0, "scale factor must be positive");
        Self {
            lhs: self.lhs.scale(factor),
            strict: self.strict,
        }
    }
    /// True iff this constraint is contradictory (0 OP positive_number for ≤, or 0 OP non-neg for <).
    pub fn is_contradiction(&self) -> bool {
        if !self.lhs.terms.is_empty() {
            return false;
        }
        let k = self.lhs.constant;
        if self.strict {
            k >= 0
        } else {
            k > 0
        }
    }
    /// Return all variable names appearing in this constraint.
    pub fn vars(&self) -> Vec<String> {
        self.lhs.terms.iter().map(|(v, _)| v.clone()).collect()
    }
    /// Return the coefficient of a variable in this constraint (0 if absent).
    pub fn coeff_of(&self, var: &str) -> i64 {
        self.lhs
            .terms
            .iter()
            .find(|(v, _)| v == var)
            .map(|(_, c)| *c)
            .unwrap_or(0)
    }
    /// Negate: not (lhs ≤ 0) → -lhs + 1 ≤ 0; not (lhs < 0) → -lhs ≤ 0.
    pub fn negate(&self) -> Self {
        let neg = self.lhs.negate();
        if self.strict {
            Self {
                lhs: neg,
                strict: false,
            }
        } else {
            let mut lhs = neg;
            lhs.constant += 1;
            Self { lhs, strict: false }
        }
    }
}
/// Numeric comparison kinds extracted from a goal expression.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumCmp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}
/// Tactic state tracking proof progress.
#[derive(Clone)]
pub struct TacticState {
    /// Current goals
    pub(super) goals: Vec<Goal>,
    /// Solved goals
    pub solved: Vec<Name>,
}
impl TacticState {
    /// Create a new tactic state.
    pub fn new() -> Self {
        Self {
            goals: Vec::new(),
            solved: Vec::new(),
        }
    }
    /// Add a goal.
    pub fn add_goal(&mut self, goal: Goal) {
        self.goals.push(goal);
    }
    /// Get the current goals.
    pub fn goals(&self) -> &[Goal] {
        &self.goals
    }
    /// Get the current goals mutably.
    #[allow(dead_code)]
    pub fn goals_mut(&mut self) -> &mut Vec<Goal> {
        &mut self.goals
    }
    /// Mark a goal as solved.
    pub fn solve_goal(&mut self, name: &Name) {
        self.goals.retain(|g| &g.name != name);
        self.solved.push(name.clone());
    }
    /// Check if all goals are solved.
    pub fn is_complete(&self) -> bool {
        self.goals.is_empty()
    }
    /// Get the number of remaining goals.
    pub fn num_goals(&self) -> usize {
        self.goals.len()
    }
    /// Get the focused (first) goal.
    #[allow(dead_code)]
    pub fn focus(&self) -> Option<&Goal> {
        self.goals.first()
    }
    /// Get the focused (first) goal mutably.
    #[allow(dead_code)]
    pub fn focus_mut(&mut self) -> Option<&mut Goal> {
        self.goals.first_mut()
    }
    /// Rotate goals by n positions (move first n goals to the end).
    #[allow(dead_code)]
    pub fn rotate(&mut self, n: usize) {
        if !self.goals.is_empty() {
            let n = n % self.goals.len();
            self.goals.rotate_left(n);
        }
    }
    /// Swap the first two goals.
    #[allow(dead_code)]
    pub fn swap(&mut self) {
        if self.goals.len() >= 2 {
            self.goals.swap(0, 1);
        }
    }
    /// Replace a goal (by name) with a set of new sub-goals.
    #[allow(dead_code)]
    pub fn replace_goal(&mut self, name: &Name, new_goals: Vec<Goal>) {
        if let Some(idx) = self.goals.iter().position(|g| &g.name == name) {
            self.goals.remove(idx);
            for (i, g) in new_goals.into_iter().enumerate() {
                self.goals.insert(idx + i, g);
            }
            self.solved.push(name.clone());
        }
    }
    /// Apply a function to all goals, collecting results.
    #[allow(dead_code)]
    pub fn all_goals_with<F>(&mut self, f: F) -> Result<(), TacticError>
    where
        F: Fn(&mut Goal) -> Result<(), TacticError>,
    {
        for goal in &mut self.goals {
            f(goal)?;
        }
        Ok(())
    }
    /// Save the current state for backtracking.
    #[allow(dead_code)]
    pub fn save_state(&self) -> SavedState {
        SavedState {
            goals: self.goals.clone(),
            solved: self.solved.clone(),
        }
    }
    /// Restore a previously saved state.
    #[allow(dead_code)]
    pub fn restore_state(&mut self, saved: SavedState) {
        self.goals = saved.goals;
        self.solved = saved.solved;
    }
}
/// Information about a registered tactic.
#[derive(Debug, Clone)]
struct TacticInfo {
    /// The tactic definition
    tactic: Tactic,
    /// Number of expected arguments (0 = no args, None = variable)
    arity: Option<usize>,
}
