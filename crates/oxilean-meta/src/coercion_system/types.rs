//! Types for the coercion system module.
//!
//! Provides automatic coercion (type casting) infrastructure: when Lean4 cannot
//! unify two types directly, it searches for a registered coercion function
//! `coe : A → B` and inserts it automatically.

use std::fmt;

// ─── Core coercion data ────────────────────────────────────────────────────

/// A single registered coercion: a function that converts values of `from_type`
/// to values of `to_type`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CoercionDef {
    /// The source type (domain of the coercion function).
    pub from_type: String,
    /// The target type (codomain of the coercion function).
    pub to_type: String,
    /// The name of the coercion function in scope.
    pub fn_name: String,
    /// Lower priority values are tried first (0 = highest priority).
    pub priority: i32,
}

impl CoercionDef {
    /// Create a new coercion definition.
    pub fn new(
        from_type: impl Into<String>,
        to_type: impl Into<String>,
        fn_name: impl Into<String>,
        priority: i32,
    ) -> Self {
        Self {
            from_type: from_type.into(),
            to_type: to_type.into(),
            fn_name: fn_name.into(),
            priority,
        }
    }
}

impl fmt::Display for CoercionDef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} : {} → {} (priority {})",
            self.fn_name, self.from_type, self.to_type, self.priority
        )
    }
}

/// The coercion database: a sorted collection of `CoercionDef`s.
///
/// Coercions are kept sorted by priority (ascending) so that lower-priority
/// values (= higher conceptual priority) are tried first during search.
#[derive(Debug, Clone)]
pub struct CoercionDB {
    /// All registered coercions, sorted by `priority` ascending.
    pub coercions: Vec<CoercionDef>,
}

/// A chain of coercions A → B → … → Z, representing a multi-step coercion path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoercionPath {
    /// Ordered sequence of coercion steps. Must be non-empty and form a chain:
    /// `steps[i].to_type == steps[i+1].from_type`.
    pub steps: Vec<CoercionDef>,
}

impl CoercionPath {
    /// The source type of this path (domain of the first step).
    pub fn from_type(&self) -> Option<&str> {
        self.steps.first().map(|s| s.from_type.as_str())
    }

    /// The target type of this path (codomain of the last step).
    pub fn to_type(&self) -> Option<&str> {
        self.steps.last().map(|s| s.to_type.as_str())
    }

    /// Number of coercion steps in this path.
    pub fn len(&self) -> usize {
        self.steps.len()
    }

    /// Whether the path contains no steps (invariant: always false for valid paths).
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }
}

// ─── Result / error types ─────────────────────────────────────────────────

/// The outcome of a coercion search.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoercionResult {
    /// A direct single-step coercion was found.
    Direct(CoercionDef),
    /// A multi-step coercion chain was found.
    Chain(CoercionPath),
    /// No coercion from the requested source to target type exists.
    NotFound,
    /// Multiple distinct coercion paths exist (ambiguous).
    Ambiguous(Vec<CoercionPath>),
}

/// Errors that can occur when managing or traversing the coercion graph.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoercionError {
    /// The proposed coercion would introduce a cycle in the graph.
    Cycle {
        /// The sequence of types forming the cycle, e.g. `["A", "B", "A"]`.
        path: Vec<String>,
    },
    /// The found (or requested) coercion chain exceeds the configured maximum length.
    TooLong {
        /// Actual length of the chain.
        length: usize,
        /// Configured maximum length.
        max: usize,
    },
    /// Multiple equally-ranked coercion paths exist with the same (from, to) pair.
    Ambiguous,
}

impl fmt::Display for CoercionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CoercionError::Cycle { path } => {
                write!(f, "coercion cycle detected: {}", path.join(" → "))
            }
            CoercionError::TooLong { length, max } => write!(
                f,
                "coercion chain too long: length {} exceeds max {}",
                length, max
            ),
            CoercionError::Ambiguous => write!(f, "ambiguous coercion: multiple paths exist"),
        }
    }
}

impl std::error::Error for CoercionError {}

/// The result of applying a coercion path to an expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoercedExpr {
    /// String representation of the original (un-coerced) expression.
    pub original: String,
    /// The type of the original expression.
    pub original_type: String,
    /// String representation of the coerced expression (after wrapping with coercion fns).
    pub coerced: String,
    /// The type of the coerced expression (= the path's target type).
    pub target_type: String,
    /// The function names applied, in order, during coercion.
    pub steps_applied: Vec<String>,
}

impl fmt::Display for CoercedExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} : {} ~~({})~~> {} : {}",
            self.original,
            self.original_type,
            self.steps_applied.join(" ∘ "),
            self.coerced,
            self.target_type,
        )
    }
}
