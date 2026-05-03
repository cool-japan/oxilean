//! Types for Prolog-style logic programming engine.

use std::collections::HashMap;

// ── Terms ─────────────────────────────────────────────────────────────────────

/// A Prolog-style term.
#[derive(Debug, Clone, PartialEq)]
pub enum LpTerm {
    /// Atom: a ground constant, e.g. `foo`, `[]`, `true`.
    Atom(String),
    /// Variable: begins with uppercase or `_`, e.g. `X`, `_Y`.
    Var(String),
    /// Compound term: `f(t1, ..., tn)`.
    Compound {
        /// The functor name.
        functor: String,
        /// Arguments.
        args: Vec<LpTerm>,
    },
    /// Integer literal.
    Integer(i64),
    /// Float literal.
    Float(f64),
    /// List `[h1, h2, ... | Tail]` — proper list when `tail` is `None`.
    List(Vec<LpTerm>, Option<Box<LpTerm>>),
}

impl LpTerm {
    /// Construct an atom.
    pub fn atom(s: impl Into<String>) -> Self {
        Self::Atom(s.into())
    }

    /// Construct a variable.
    pub fn var(s: impl Into<String>) -> Self {
        Self::Var(s.into())
    }

    /// Construct a compound term.
    pub fn compound(functor: impl Into<String>, args: Vec<LpTerm>) -> Self {
        Self::Compound {
            functor: functor.into(),
            args,
        }
    }

    /// Construct a proper list (no tail variable).
    pub fn list(items: Vec<LpTerm>) -> Self {
        Self::List(items, None)
    }

    /// Construct a partial list with tail variable.
    pub fn list_with_tail(items: Vec<LpTerm>, tail: LpTerm) -> Self {
        Self::List(items, Some(Box::new(tail)))
    }

    /// Return true if this term is ground (contains no variables).
    pub fn is_ground(&self) -> bool {
        match self {
            Self::Var(_) => false,
            Self::Atom(_) | Self::Integer(_) | Self::Float(_) => true,
            Self::Compound { args, .. } => args.iter().all(|a| a.is_ground()),
            Self::List(items, tail) => {
                items.iter().all(|a| a.is_ground()) && tail.as_ref().map_or(true, |t| t.is_ground())
            }
        }
    }

    /// Collect all variable names in this term (without deduplication).
    pub fn variables(&self) -> Vec<String> {
        let mut vars = Vec::new();
        self.collect_vars(&mut vars);
        vars
    }

    fn collect_vars(&self, out: &mut Vec<String>) {
        match self {
            Self::Var(v) => out.push(v.clone()),
            Self::Compound { args, .. } => {
                for a in args {
                    a.collect_vars(out);
                }
            }
            Self::List(items, tail) => {
                for a in items {
                    a.collect_vars(out);
                }
                if let Some(t) = tail {
                    t.collect_vars(out);
                }
            }
            _ => {}
        }
    }
}

// ── Clauses & Database ────────────────────────────────────────────────────────

/// A Horn clause: `head :- body1, body2, ...`  or a fact: `head.` (empty body).
#[derive(Debug, Clone)]
pub struct LpClause {
    /// The head of the clause.
    pub head: LpTerm,
    /// The body (conjunction of goals). Empty for facts.
    pub body: Vec<LpTerm>,
}

impl LpClause {
    /// Construct a fact (no body).
    pub fn fact(head: LpTerm) -> Self {
        Self { head, body: vec![] }
    }

    /// Construct a rule.
    pub fn rule(head: LpTerm, body: Vec<LpTerm>) -> Self {
        Self { head, body }
    }

    /// True if this clause has no body (is a fact).
    pub fn is_fact(&self) -> bool {
        self.body.is_empty()
    }
}

/// A logic programming database of Horn clauses.
#[derive(Debug, Clone, Default)]
pub struct LpDatabase {
    /// All clauses, in order of assertion.
    pub clauses: Vec<LpClause>,
}

impl LpDatabase {
    /// Create an empty database.
    pub fn new() -> Self {
        Self { clauses: vec![] }
    }

    /// Add a fact to the database.
    pub fn add_fact(&mut self, head: LpTerm) {
        self.clauses.push(LpClause::fact(head));
    }

    /// Add a rule to the database.
    pub fn add_rule(&mut self, head: LpTerm, body: Vec<LpTerm>) {
        self.clauses.push(LpClause::rule(head, body));
    }

    /// Return all clauses whose head functor/arity matches the given goal.
    pub fn matching_clauses(&self, goal: &LpTerm) -> Vec<&LpClause> {
        let (goal_f, goal_a) = functor_arity(goal);
        self.clauses
            .iter()
            .filter(|c| {
                let (hf, ha) = functor_arity(&c.head);
                hf == goal_f && ha == goal_a
            })
            .collect()
    }
}

/// Extract (functor_name, arity) for unification index.
pub(super) fn functor_arity(t: &LpTerm) -> (&str, usize) {
    match t {
        LpTerm::Atom(s) => (s.as_str(), 0),
        LpTerm::Compound { functor, args } => (functor.as_str(), args.len()),
        LpTerm::Integer(_) => ("$int", 0),
        LpTerm::Float(_) => ("$float", 0),
        LpTerm::Var(_) => ("$var", 0),
        LpTerm::List(items, None) => ("$list", items.len()),
        LpTerm::List(items, Some(_)) => ("$list_partial", items.len()),
    }
}

// ── Substitution ──────────────────────────────────────────────────────────────

/// A substitution: a finite map from variable names to terms.
#[derive(Debug, Clone, Default)]
pub struct Substitution {
    /// Variable bindings.
    pub bindings: HashMap<String, LpTerm>,
}

impl Substitution {
    /// Create an empty substitution.
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    /// Bind a variable to a term.
    pub fn bind(&mut self, var: impl Into<String>, term: LpTerm) {
        self.bindings.insert(var.into(), term);
    }

    /// Extend this substitution with all bindings from `other`.
    pub fn extend(&self, other: &Substitution) -> Substitution {
        let mut result = self.clone();
        for (k, v) in &other.bindings {
            result.bindings.insert(k.clone(), v.clone());
        }
        result
    }

    /// Look up the current binding of a variable (may need further dereferencing).
    pub fn lookup(&self, var: &str) -> Option<&LpTerm> {
        self.bindings.get(var)
    }

    /// Dereference a variable to its ultimate binding (or back to a variable if unbound).
    pub fn deref(&self, var: &str) -> LpTerm {
        match self.bindings.get(var) {
            None => LpTerm::Var(var.to_string()),
            Some(LpTerm::Var(v2)) if v2 != var => self.deref(v2),
            Some(t) => t.clone(),
        }
    }
}

// ── Query ─────────────────────────────────────────────────────────────────────

/// A conjunctive query: a list of goals to be solved simultaneously.
#[derive(Debug, Clone)]
pub struct Query {
    /// Goals in left-to-right order.
    pub goals: Vec<LpTerm>,
}

impl Query {
    /// Construct a query from a list of goals.
    pub fn new(goals: Vec<LpTerm>) -> Self {
        Self { goals }
    }

    /// Construct a single-goal query.
    pub fn single(goal: LpTerm) -> Self {
        Self { goals: vec![goal] }
    }
}

// ── Resolution result ─────────────────────────────────────────────────────────

/// The outcome of resolving a query.
#[derive(Debug, Clone)]
pub enum ResolutionResult {
    /// A successful substitution answering the query.
    Success(Substitution),
    /// The query has no solution.
    Failure,
    /// An error occurred during resolution (e.g., depth limit exceeded).
    Error(String),
}

// ── Solver configuration ──────────────────────────────────────────────────────

/// Configuration for the SLD resolution engine.
#[derive(Debug, Clone)]
pub struct SolveConfig {
    /// Maximum recursion depth before failing with an error.
    pub max_depth: usize,
    /// Maximum number of solutions to collect.
    pub max_solutions: usize,
    /// Whether to perform the occurs check during unification.
    pub occurs_check: bool,
}

impl Default for SolveConfig {
    fn default() -> Self {
        Self {
            max_depth: 512,
            max_solutions: 1024,
            occurs_check: false,
        }
    }
}

impl SolveConfig {
    /// Create a default configuration.
    pub fn new() -> Self {
        Self::default()
    }
}
