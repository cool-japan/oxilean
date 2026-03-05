//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::{Pattern, SurfaceExpr};

/// A pattern matrix row (for exhaustiveness checking).
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct PatternMatrixRow {
    /// Patterns in this row (one per column)
    pub patterns: Vec<String>,
    /// Body of this arm
    pub body: String,
}
impl PatternMatrixRow {
    /// Create a new row.
    #[allow(dead_code)]
    pub fn new(patterns: Vec<&str>, body: &str) -> Self {
        PatternMatrixRow {
            patterns: patterns.into_iter().map(|s| s.to_string()).collect(),
            body: body.to_string(),
        }
    }
    /// Returns true if this row is a wildcard row (all patterns are wildcards).
    #[allow(dead_code)]
    pub fn is_wildcard_row(&self) -> bool {
        self.patterns.iter().all(|p| p == "_")
    }
}
/// A row in the pattern matrix.
#[derive(Debug, Clone)]
pub struct PatternRow {
    /// The patterns for each column
    pub patterns: Vec<Pattern>,
    /// The body expression for this row
    pub body: SurfaceExpr,
    /// Optional guard expression
    pub guard: Option<SurfaceExpr>,
}
/// Known constructors for exhaustiveness checking.
#[derive(Debug, Clone)]
pub struct TypeConstructors {
    /// Name of the type
    pub type_name: String,
    /// Constructor information
    pub constructors: Vec<ConstructorInfo>,
}
/// A pattern type tag for classification.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatternTagExt {
    /// Wildcard
    Wild,
    /// Variable binding
    Var,
    /// Constructor
    Ctor,
    /// Literal
    Lit,
    /// Or-pattern
    Or,
    /// As-pattern
    As,
    /// Guard pattern
    Guard,
}
/// A pattern match arm.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct MatchArmExt {
    /// The pattern string
    pub pattern: String,
    /// The body string
    pub body: String,
    /// Optional guard
    pub guard: Option<String>,
}
impl MatchArmExt {
    /// Create a new match arm.
    #[allow(dead_code)]
    pub fn new(pattern: &str, body: &str) -> Self {
        MatchArmExt {
            pattern: pattern.to_string(),
            body: body.to_string(),
            guard: None,
        }
    }
    /// Add a guard.
    #[allow(dead_code)]
    pub fn with_guard(mut self, guard: &str) -> Self {
        self.guard = Some(guard.to_string());
        self
    }
}
/// A branch in a case tree switch.
#[derive(Debug, Clone, PartialEq)]
pub struct CaseBranch {
    /// Constructor name
    pub ctor: String,
    /// Number of fields for this constructor
    pub num_fields: usize,
    /// Subtree for this branch
    pub subtree: CaseTree,
}
/// Inaccessible pattern: `.(_)` - pattern that can't be matched
#[derive(Debug, Clone, PartialEq)]
pub struct InaccessiblePattern {
    /// The inaccessible pattern content
    pub inner: Box<Pattern>,
}
/// As-pattern: `pat @ name`
#[derive(Debug, Clone, PartialEq)]
pub struct AsPattern {
    /// The underlying pattern
    pub pattern: Box<Pattern>,
    /// The binding name
    pub name: String,
}
/// Pattern specialization result
#[derive(Debug, Clone, PartialEq)]
pub struct SpecializedPattern {
    /// The resulting patterns after specialization
    pub patterns: Vec<Pattern>,
    /// Whether this specialization applies
    pub applies: bool,
}
/// A match clause with a pattern and a body.
#[derive(Debug, Clone)]
pub struct MatchClause {
    /// The pattern to match
    pub pattern: Pattern,
    /// The body expression
    pub body: SurfaceExpr,
}
/// A variable binding in a pattern match.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct PatternBinding {
    /// Variable name
    pub name: String,
    /// Position in the constructor (0-indexed)
    pub position: usize,
    /// Type annotation if present
    pub ty: Option<String>,
}
impl PatternBinding {
    /// Create a new binding.
    #[allow(dead_code)]
    pub fn new(name: &str, position: usize) -> Self {
        PatternBinding {
            name: name.to_string(),
            position,
            ty: None,
        }
    }
    /// Set the type annotation.
    #[allow(dead_code)]
    pub fn with_type(mut self, ty: &str) -> Self {
        self.ty = Some(ty.to_string());
        self
    }
}
/// Result of pattern compilation.
#[derive(Debug, Clone, PartialEq)]
pub enum CaseTree {
    /// A leaf node: execute the body at the given index
    Leaf {
        /// Index into the original clause list
        body_idx: usize,
    },
    /// A switch on a scrutinee column
    Switch {
        /// Column index of the scrutinee
        scrutinee: usize,
        /// Constructor branches
        branches: Vec<CaseBranch>,
        /// Default branch if no constructor matches
        default: Option<Box<CaseTree>>,
    },
    /// Pattern match failure (non-exhaustive)
    Failure,
}
/// Array/list pattern: `[x, y, z]` or `[x, ..., z]`
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayPattern {
    /// Elements before the rest pattern (if any)
    pub prefix: Vec<crate::Located<Pattern>>,
    /// Rest pattern like `..` (true if present)
    pub rest: bool,
    /// Elements after the rest pattern (if any)
    pub suffix: Vec<crate::Located<Pattern>>,
}
/// A pattern coverage checker (simplified).
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct PatternCoverageExt {
    /// Number of arms
    pub arm_count: usize,
    /// Whether a wildcard arm is present
    pub has_wildcard: bool,
}
impl PatternCoverageExt {
    /// Create a new coverage checker.
    #[allow(dead_code)]
    pub fn new() -> Self {
        PatternCoverageExt {
            arm_count: 0,
            has_wildcard: false,
        }
    }
    /// Add an arm.
    #[allow(dead_code)]
    pub fn add_arm(&mut self, tag: PatternTagExt) {
        self.arm_count += 1;
        if tag == PatternTagExt::Wild || tag == PatternTagExt::Var {
            self.has_wildcard = true;
        }
    }
    /// Returns true if coverage is trivially complete (has a wildcard).
    #[allow(dead_code)]
    pub fn is_complete(&self) -> bool {
        self.has_wildcard
    }
}
/// Information about a single constructor.
#[derive(Debug, Clone)]
pub struct ConstructorInfo {
    /// Constructor name
    pub name: String,
    /// Number of arguments (arity)
    pub arity: usize,
}
/// Pattern coverage information
#[derive(Debug, Clone)]
pub struct PatternCoverage {
    /// Patterns that are covered
    pub covered: Vec<Pattern>,
    /// Patterns that are not covered
    pub uncovered: Vec<Pattern>,
}
/// A pattern transformer that applies renaming to variable patterns.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct PatternRenamer {
    /// Map from old name to new name
    pub renaming: std::collections::HashMap<String, String>,
}
impl PatternRenamer {
    /// Create a new renamer.
    #[allow(dead_code)]
    pub fn new() -> Self {
        PatternRenamer {
            renaming: std::collections::HashMap::new(),
        }
    }
    /// Add a renaming.
    #[allow(dead_code)]
    pub fn add(&mut self, from: &str, to: &str) {
        self.renaming.insert(from.to_string(), to.to_string());
    }
    /// Rename a pattern string.
    #[allow(dead_code)]
    pub fn rename(&self, pattern: &str) -> String {
        self.renaming
            .get(pattern)
            .cloned()
            .unwrap_or_else(|| pattern.to_string())
    }
}
