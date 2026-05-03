//! Types for auto-bound implicit variable handling.
//!
//! Lean4 has "auto-bound implicit" variables: if a name appears free in a
//! declaration type but isn't declared, it is automatically added as an
//! implicit parameter.

/// A single auto-bound implicit variable discovered in a declaration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AutoBoundVar {
    /// The variable name as it appears in the source.
    pub name: String,
    /// The inferred type for this variable (if determinable).
    pub inferred_type: Option<String>,
    /// How many times this variable is referenced in the declaration.
    pub usage_count: usize,
    /// The (line, column) of the first reference to this variable.
    pub first_use: (u32, u32),
}

impl AutoBoundVar {
    /// Create a new auto-bound variable with a given name.
    pub fn new(name: impl Into<String>, first_use: (u32, u32)) -> Self {
        Self {
            name: name.into(),
            inferred_type: None,
            usage_count: 1,
            first_use,
        }
    }

    /// Create an auto-bound variable with a known inferred type.
    pub fn with_type(
        name: impl Into<String>,
        inferred_type: impl Into<String>,
        first_use: (u32, u32),
    ) -> Self {
        Self {
            name: name.into(),
            inferred_type: Some(inferred_type.into()),
            usage_count: 1,
            first_use,
        }
    }

    /// Record one more usage of this variable.
    pub fn increment_usage(&mut self) {
        self.usage_count += 1;
    }

    /// Whether a type has been inferred for this variable.
    pub fn has_inferred_type(&self) -> bool {
        self.inferred_type.is_some()
    }
}

impl std::fmt::Display for AutoBoundVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.inferred_type {
            Some(ty) => write!(f, "{{{} : {}}}", self.name, ty),
            None => write!(f, "{{{}}}", self.name),
        }
    }
}

/// The result of processing auto-bound implicit variables for a declaration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AutoBoundResult {
    /// The implicit variables that were added.
    pub added_implicits: Vec<AutoBoundVar>,
    /// The modified declaration type string (with implicits prepended).
    pub modified_type: String,
    /// The modified declaration body (if any transformation was needed).
    pub modified_body: Option<String>,
}

impl AutoBoundResult {
    /// Create a result with no modifications.
    pub fn unchanged(original_type: impl Into<String>) -> Self {
        Self {
            added_implicits: Vec::new(),
            modified_type: original_type.into(),
            modified_body: None,
        }
    }

    /// How many implicit parameters were automatically added.
    pub fn num_added(&self) -> usize {
        self.added_implicits.len()
    }

    /// Whether any implicit variables were added.
    pub fn has_additions(&self) -> bool {
        !self.added_implicits.is_empty()
    }
}

/// Configuration controlling auto-bound implicit variable behaviour.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AutoBoundConfig {
    /// Whether auto-bound implicit processing is enabled at all.
    pub enable: bool,
    /// Sort auto-bound variables alphabetically (otherwise: universe vars
    /// first, then type vars, then term vars).
    pub sort_alphabetically: bool,
    /// Maximum number of auto-bound variables allowed in a single declaration.
    pub max_auto_vars: usize,
    /// Names that should never be treated as auto-bound (e.g. reserved names).
    pub ignore_names: Vec<String>,
}

impl AutoBoundConfig {
    /// Return the default configuration (enabled, max 32 vars).
    pub fn default_config() -> Self {
        Self {
            enable: true,
            sort_alphabetically: false,
            max_auto_vars: 32,
            ignore_names: Vec::new(),
        }
    }

    /// Return a disabled configuration.
    pub fn disabled() -> Self {
        Self {
            enable: false,
            sort_alphabetically: false,
            max_auto_vars: 0,
            ignore_names: Vec::new(),
        }
    }

    /// Whether `name` is in the ignore list.
    pub fn is_ignored(&self, name: &str) -> bool {
        self.ignore_names.iter().any(|n| n == name)
    }
}

impl Default for AutoBoundConfig {
    fn default() -> Self {
        Self::default_config()
    }
}

/// Errors that can occur during auto-bound implicit processing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AutoBoundError {
    /// Too many free variables were found.
    TooManyVars {
        /// Number of variables found.
        count: usize,
        /// Maximum allowed.
        max: usize,
    },
    /// The inferred type of a variable is ambiguous.
    AmbiguousType(String),
    /// The variable name conflicts with an already-declared name.
    ConflictsWithExisting(String),
}

impl std::fmt::Display for AutoBoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AutoBoundError::TooManyVars { count, max } => write!(
                f,
                "too many auto-bound implicit variables: found {count}, maximum is {max}"
            ),
            AutoBoundError::AmbiguousType(name) => {
                write!(f, "cannot infer type of auto-bound variable `{name}`")
            }
            AutoBoundError::ConflictsWithExisting(name) => {
                write!(
                    f,
                    "auto-bound variable `{name}` conflicts with an existing declaration"
                )
            }
        }
    }
}

impl std::error::Error for AutoBoundError {}

/// Classification of an auto-bound variable by its intended role.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum AutoBoundKind {
    /// Universe-level variable (`u`, `v`, `u1`, …).
    Universe,
    /// Type variable (single lowercase Greek letter, `α`, `β`, …).
    TypeVar,
    /// Term variable (`n`, `m`, `k`, propositional `h`, `p`, `q`, …).
    TermVar,
}

impl AutoBoundKind {
    /// Human-readable label for the kind.
    pub fn label(self) -> &'static str {
        match self {
            AutoBoundKind::Universe => "universe",
            AutoBoundKind::TypeVar => "type variable",
            AutoBoundKind::TermVar => "term variable",
        }
    }
}
