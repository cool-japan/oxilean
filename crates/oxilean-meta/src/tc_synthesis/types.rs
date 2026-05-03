//! Types for the type class instance synthesis module.
//!
//! Models Lean4's type class resolution machinery: classes, instances, a database,
//! synthesis goals, and the various outcomes of instance search.

use std::collections::HashMap;
use std::fmt;

// ─── Type class ───────────────────────────────────────────────────────────

/// A Lean4-style type class declaration.
///
/// ```text
/// class Add (α : Type u) where
///   add : α → α → α
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TcClass {
    /// The name of the type class, e.g. `"Add"`.
    pub name: String,
    /// Type parameter names, e.g. `["α"]`.
    pub params: Vec<String>,
    /// Names of superclasses that must be satisfied, e.g. `["BEq"]`.
    pub superclasses: Vec<String>,
    /// Methods declared by this class: `(method_name, type_signature_string)`.
    pub methods: Vec<(String, String)>,
}

impl TcClass {
    /// Create a new type class with no methods or superclasses.
    pub fn new(name: impl Into<String>, params: Vec<String>) -> Self {
        Self {
            name: name.into(),
            params,
            superclasses: Vec::new(),
            methods: Vec::new(),
        }
    }

    /// Add a superclass constraint to this class.
    pub fn with_superclass(mut self, sc: impl Into<String>) -> Self {
        self.superclasses.push(sc.into());
        self
    }

    /// Add a method declaration `(name, type_string)`.
    pub fn with_method(mut self, name: impl Into<String>, ty: impl Into<String>) -> Self {
        self.methods.push((name.into(), ty.into()));
        self
    }
}

impl fmt::Display for TcClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "class {} [{}]", self.name, self.params.join(", "))?;
        if !self.superclasses.is_empty() {
            write!(f, " extends {}", self.superclasses.join(", "))?;
        }
        Ok(())
    }
}

// ─── Type class instance ──────────────────────────────────────────────────

/// A Lean4-style type class instance declaration.
///
/// ```text
/// instance : Add Nat where
///   add := Nat.add
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TcInstance {
    /// The class this instance satisfies, e.g. `"Add"`.
    pub class: String,
    /// Concrete type arguments, e.g. `["Nat"]`.
    pub type_args: Vec<String>,
    /// A unique identifier / name for this instance, e.g. `"instAddNat"`.
    pub name: String,
    /// Method implementations: `method_name → implementation_string`.
    pub impl_body: HashMap<String, String>,
}

impl TcInstance {
    /// Create a new instance with no method body.
    pub fn new(class: impl Into<String>, type_args: Vec<String>, name: impl Into<String>) -> Self {
        Self {
            class: class.into(),
            type_args,
            name: name.into(),
            impl_body: HashMap::new(),
        }
    }

    /// Add a method implementation to this instance.
    pub fn with_impl(mut self, method: impl Into<String>, body: impl Into<String>) -> Self {
        self.impl_body.insert(method.into(), body.into());
        self
    }

    /// Check whether this instance matches a synthesis goal.
    ///
    /// Matching requires:
    /// 1. Class names must be equal.
    /// 2. Arities must be equal.
    /// 3. Each concrete type argument in the goal must equal the corresponding
    ///    argument in the instance (wildcard / meta-variable positions denoted
    ///    by `"_"` or strings starting with `"?"` are treated as universally
    ///    matching).
    pub fn matches_goal(&self, goal: &SynthGoal) -> bool {
        if self.class != goal.class {
            return false;
        }
        if self.type_args.len() != goal.type_args.len() {
            return false;
        }
        // Pairwise match: instance arg must equal goal arg unless the goal arg
        // is a metavariable / wildcard, or the instance arg is a type variable.
        self.type_args
            .iter()
            .zip(goal.type_args.iter())
            .all(|(inst_arg, goal_arg)| {
                // A goal arg that is "_" or starts with "?" is a flex position.
                let goal_flex = goal_arg == "_" || goal_arg.starts_with('?');
                // An instance arg that starts with a lowercase letter is a
                // universally-quantified type variable (e.g., `α`, `β`).
                let inst_flex = inst_arg
                    .chars()
                    .next()
                    .map(|c| c.is_lowercase())
                    .unwrap_or(false);
                goal_flex || inst_flex || inst_arg == goal_arg
            })
    }
}

impl fmt::Display for TcInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "instance {} : {} {}",
            self.name,
            self.class,
            self.type_args.join(" ")
        )
    }
}

// ─── Database ─────────────────────────────────────────────────────────────

/// The type class database: a collection of classes and instances.
#[derive(Debug, Clone)]
pub struct TcDB {
    /// Classes indexed by name.
    pub classes: HashMap<String, TcClass>,
    /// All registered instances (order matters for coherence checks).
    pub instances: Vec<TcInstance>,
}

// ─── Synthesis goal ───────────────────────────────────────────────────────

/// A synthesis goal: "find an instance of `class` for `type_args`".
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SynthGoal {
    /// Name of the class to synthesize.
    pub class: String,
    /// Concrete type arguments.
    pub type_args: Vec<String>,
}

impl SynthGoal {
    /// Convenience constructor.
    pub fn new(class: impl Into<String>, type_args: Vec<String>) -> Self {
        Self {
            class: class.into(),
            type_args,
        }
    }
}

impl fmt::Display for SynthGoal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.type_args.is_empty() {
            write!(f, "{}", self.class)
        } else {
            write!(f, "{} {}", self.class, self.type_args.join(" "))
        }
    }
}

// ─── Synthesis result ─────────────────────────────────────────────────────

/// The outcome of attempting to synthesize a type class instance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SynthResult {
    /// A unique instance was found; `constraints` are the sub-goals that still
    /// need to be discharged (e.g., superclass constraints).
    Found {
        /// The resolved instance.
        instance: TcInstance,
        /// Remaining sub-goals generated by the instance's superclass chain.
        constraints: Vec<SynthGoal>,
    },
    /// No instance for the goal is registered.
    NotFound,
    /// Multiple instances match and no priority distinguishes them.
    Overlapping(Vec<TcInstance>),
    /// Instance resolution entered an infinite loop.
    Cycle(Vec<SynthGoal>),
}

// ─── Configuration ────────────────────────────────────────────────────────

/// Configuration for instance synthesis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SynthConfig {
    /// Maximum recursion depth for instance search.
    pub max_depth: usize,
    /// Whether to backtrack on failure and try alternative instances.
    pub backtrack: bool,
    /// Whether to check for overlapping (incoherent) instances before resolving.
    pub coherence_check: bool,
}

impl Default for SynthConfig {
    fn default() -> Self {
        Self {
            max_depth: 32,
            backtrack: true,
            coherence_check: true,
        }
    }
}

// ─── Synthesis trace ──────────────────────────────────────────────────────

/// A trace of all goals attempted during a synthesis run, for debugging.
#[derive(Debug, Clone, Default)]
pub struct SynthTrace {
    /// `(goal_attempted, result)` pairs in resolution order.
    pub goals: Vec<(SynthGoal, SynthResult)>,
}

impl SynthTrace {
    /// Create an empty trace.
    pub fn new() -> Self {
        Self::default()
    }

    /// Append a goal–result pair.
    pub fn record(&mut self, goal: SynthGoal, result: SynthResult) {
        self.goals.push((goal, result));
    }

    /// How many goals were attempted.
    pub fn len(&self) -> usize {
        self.goals.len()
    }

    /// Whether no goals were attempted.
    pub fn is_empty(&self) -> bool {
        self.goals.is_empty()
    }
}
