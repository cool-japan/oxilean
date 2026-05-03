//! Types for universe-level management and polymorphism.
//!
//! Universe levels form the backbone of Lean's hierarchy of sorts:
//! `Prop : Sort 0`, `Type 0 : Sort 1`, `Type 1 : Sort 2`, etc.

use std::collections::HashMap;

// ─── UniverseLevel ───────────────────────────────────────────────────────────

/// A universe level expression.
///
/// This mirrors Lean 4's internal representation:
/// - `zero`  corresponds to `Prop` / `Sort 0`
/// - `succ u` corresponds to `u + 1`
/// - `max u v` = the maximum of two levels
/// - `imax u v` = `0` if `v = 0`, else `max u v` (used for Pi-types in `Prop`)
/// - `param name` = a universe polymorphism parameter
/// - `metavar id` = an unsolved metavariable (used during inference)
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum UniverseLevel {
    /// The zero universe level (corresponds to `Prop`).
    Zero,
    /// Successor of a universe level.
    Succ(Box<UniverseLevel>),
    /// Pointwise maximum of two universe levels.
    Max(Box<UniverseLevel>, Box<UniverseLevel>),
    /// Impredicative maximum: `IMax(u, v) = 0` if `v = 0`, else `Max(u, v)`.
    IMax(Box<UniverseLevel>, Box<UniverseLevel>),
    /// A named universe-polymorphism parameter.
    Param(String),
    /// An unsolved universe metavariable identified by its ID.
    Metavar(u64),
}

impl UniverseLevel {
    /// Construct `Succ(self)`.
    pub fn succ(self) -> Self {
        UniverseLevel::Succ(Box::new(self))
    }

    /// Construct `Max(self, other)`.
    pub fn max_with(self, other: UniverseLevel) -> Self {
        UniverseLevel::Max(Box::new(self), Box::new(other))
    }

    /// Construct `IMax(self, other)`.
    pub fn imax_with(self, other: UniverseLevel) -> Self {
        UniverseLevel::IMax(Box::new(self), Box::new(other))
    }

    /// Whether this level contains any metavariables.
    pub fn has_metavars(&self) -> bool {
        match self {
            UniverseLevel::Zero => false,
            UniverseLevel::Succ(inner) => inner.has_metavars(),
            UniverseLevel::Max(l, r) | UniverseLevel::IMax(l, r) => {
                l.has_metavars() || r.has_metavars()
            }
            UniverseLevel::Param(_) => false,
            UniverseLevel::Metavar(_) => true,
        }
    }

    /// Whether this level contains any named parameters.
    pub fn has_params(&self) -> bool {
        match self {
            UniverseLevel::Zero => false,
            UniverseLevel::Succ(inner) => inner.has_params(),
            UniverseLevel::Max(l, r) | UniverseLevel::IMax(l, r) => {
                l.has_params() || r.has_params()
            }
            UniverseLevel::Param(_) => true,
            UniverseLevel::Metavar(_) => false,
        }
    }

    /// Collect all metavariable IDs that appear in this level.
    pub fn collect_metavars(&self) -> Vec<u64> {
        let mut ids = Vec::new();
        self.collect_metavars_into(&mut ids);
        ids
    }

    fn collect_metavars_into(&self, acc: &mut Vec<u64>) {
        match self {
            UniverseLevel::Zero => {}
            UniverseLevel::Succ(inner) => inner.collect_metavars_into(acc),
            UniverseLevel::Max(l, r) | UniverseLevel::IMax(l, r) => {
                l.collect_metavars_into(acc);
                r.collect_metavars_into(acc);
            }
            UniverseLevel::Param(_) => {}
            UniverseLevel::Metavar(id) => {
                if !acc.contains(id) {
                    acc.push(*id);
                }
            }
        }
    }

    /// Collect all named parameters that appear in this level.
    pub fn collect_params(&self) -> Vec<String> {
        let mut params = Vec::new();
        self.collect_params_into(&mut params);
        params
    }

    fn collect_params_into(&self, acc: &mut Vec<String>) {
        match self {
            UniverseLevel::Zero => {}
            UniverseLevel::Succ(inner) => inner.collect_params_into(acc),
            UniverseLevel::Max(l, r) | UniverseLevel::IMax(l, r) => {
                l.collect_params_into(acc);
                r.collect_params_into(acc);
            }
            UniverseLevel::Param(name) => {
                if !acc.contains(name) {
                    acc.push(name.clone());
                }
            }
            UniverseLevel::Metavar(_) => {}
        }
    }
}

impl std::fmt::Display for UniverseLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use super::functions::level_to_string;
        write!(f, "{}", level_to_string(self))
    }
}

// ─── UniverseConstraint ──────────────────────────────────────────────────────

/// A constraint relating two universe levels.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UniverseConstraint {
    /// `l ≤ r`
    Leq(UniverseLevel, UniverseLevel),
    /// `l = r`
    Eq(UniverseLevel, UniverseLevel),
    /// `l < r`  (strict: `l + 1 ≤ r`)
    Lt(UniverseLevel, UniverseLevel),
}

impl UniverseConstraint {
    /// Return the left-hand side.
    pub fn lhs(&self) -> &UniverseLevel {
        match self {
            UniverseConstraint::Leq(l, _)
            | UniverseConstraint::Eq(l, _)
            | UniverseConstraint::Lt(l, _) => l,
        }
    }

    /// Return the right-hand side.
    pub fn rhs(&self) -> &UniverseLevel {
        match self {
            UniverseConstraint::Leq(_, r)
            | UniverseConstraint::Eq(_, r)
            | UniverseConstraint::Lt(_, r) => r,
        }
    }
}

impl std::fmt::Display for UniverseConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use super::functions::level_to_string;
        match self {
            UniverseConstraint::Leq(l, r) => {
                write!(f, "{} ≤ {}", level_to_string(l), level_to_string(r))
            }
            UniverseConstraint::Eq(l, r) => {
                write!(f, "{} = {}", level_to_string(l), level_to_string(r))
            }
            UniverseConstraint::Lt(l, r) => {
                write!(f, "{} < {}", level_to_string(l), level_to_string(r))
            }
        }
    }
}

// ─── UniverseCtx ─────────────────────────────────────────────────────────────

/// Elaboration context for universe levels.
///
/// Tracks the universe polymorphism parameters, accumulated constraints, and
/// current metavariable assignments.
#[derive(Clone, Debug, Default)]
pub struct UniverseCtx {
    /// Named universe-polymorphism parameters in scope.
    pub params: Vec<String>,
    /// Accumulated constraints to be solved.
    pub constraints: Vec<UniverseConstraint>,
    /// Current assignments from metavariable ID → level.
    pub assignments: HashMap<u64, UniverseLevel>,
    /// Counter for fresh metavariable IDs.
    pub(crate) next_meta_id: u64,
}

impl UniverseCtx {
    /// Create an empty context.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a context with the given universe parameters.
    pub fn with_params(params: Vec<String>) -> Self {
        Self {
            params,
            ..Self::default()
        }
    }

    /// Add a constraint to the context.
    pub fn add_constraint(&mut self, c: UniverseConstraint) {
        self.constraints.push(c);
    }

    /// Assign a metavariable.
    pub fn assign(&mut self, id: u64, level: UniverseLevel) {
        self.assignments.insert(id, level);
    }

    /// Look up the current assignment for a metavariable.
    pub fn lookup(&self, id: u64) -> Option<&UniverseLevel> {
        self.assignments.get(&id)
    }

    /// Whether `param_name` is a known universe parameter.
    pub fn has_param(&self, param_name: &str) -> bool {
        self.params.iter().any(|p| p == param_name)
    }

    /// Number of unsolved constraints remaining.
    pub fn constraint_count(&self) -> usize {
        self.constraints.len()
    }
}

// ─── UniverseError ───────────────────────────────────────────────────────────

/// Errors arising from universe-level solving.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UniverseError {
    /// A cyclic dependency was detected among universe levels.
    Cycle,
    /// A constraint is inconsistent (cannot be satisfied).
    Inconsistent(UniverseConstraint),
    /// A metavariable has no unique solution.
    Underdetermined(u64),
}

impl std::fmt::Display for UniverseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UniverseError::Cycle => write!(f, "cyclic universe constraint detected"),
            UniverseError::Inconsistent(c) => {
                write!(f, "inconsistent universe constraint: {c}")
            }
            UniverseError::Underdetermined(id) => {
                write!(f, "universe metavariable ?u{id} has no unique solution")
            }
        }
    }
}

impl std::error::Error for UniverseError {}

// ─── UniverseSolution ────────────────────────────────────────────────────────

/// A solved assignment from universe parameter names to universe levels.
#[derive(Clone, Debug, Default)]
pub struct UniverseSolution {
    /// Assignment from named parameter → solved level.
    pub assignments: HashMap<String, UniverseLevel>,
    /// Whether this is the minimal solution (all levels as small as possible).
    pub is_minimal: bool,
}

impl UniverseSolution {
    /// Create an empty solution.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a minimal solution from an explicit map.
    pub fn minimal(assignments: HashMap<String, UniverseLevel>) -> Self {
        Self {
            assignments,
            is_minimal: true,
        }
    }

    /// Look up the solved level for a parameter name.
    pub fn get(&self, name: &str) -> Option<&UniverseLevel> {
        self.assignments.get(name)
    }

    /// Number of solved parameters.
    pub fn len(&self) -> usize {
        self.assignments.len()
    }

    /// Whether the solution is empty.
    pub fn is_empty(&self) -> bool {
        self.assignments.is_empty()
    }
}
