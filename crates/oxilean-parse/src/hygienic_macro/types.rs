//! Types for hygienic macro expansion.

use std::collections::HashMap;

/// A unique scope identifier used for hygienic renaming.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ScopeId(pub u64);

impl std::fmt::Display for ScopeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "scope#{}", self.0)
    }
}

/// A variable tagged with its hygiene scope.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MacroVar {
    /// The original variable name.
    pub name: String,
    /// The scope in which this variable lives.
    pub scope: ScopeId,
}

impl MacroVar {
    /// Create a new `MacroVar`.
    pub fn new(name: impl Into<String>, scope: ScopeId) -> Self {
        Self {
            name: name.into(),
            scope,
        }
    }
}

impl std::fmt::Display for MacroVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}@{}", self.name, self.scope)
    }
}

/// Hygiene context: tracks scopes and name bindings for macro expansion.
///
/// The `bindings` map stores, for each original name, a stack of
/// `(scope_id, fresh_name)` pairs.  The most recent (innermost) binding
/// is at the back of the vector.
#[derive(Debug, Clone)]
pub struct HygieneCtx {
    /// The scope that is currently active.
    pub current_scope: ScopeId,
    /// Mapping from original name → stack of `(scope, fresh_name)`.
    pub bindings: HashMap<String, Vec<(ScopeId, String)>>,
    /// Monotonically increasing counter for fresh-name generation.
    pub(super) counter: u64,
    /// Stack of active scope ids (innermost last).
    pub(super) scope_stack: Vec<ScopeId>,
}

/// Definition of a macro.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MacroDef {
    /// The macro name.
    pub name: String,
    /// Formal parameter names.
    pub params: Vec<String>,
    /// The body template (a source string using `$param` style references).
    pub body: String,
    /// The scope in which this macro was defined.
    pub def_scope: ScopeId,
}

/// A call-site invocation of a macro.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MacroCall {
    /// The name of the macro being called.
    pub name: String,
    /// Actual arguments (source strings).
    pub args: Vec<String>,
    /// The scope at the call site.
    pub call_scope: ScopeId,
}

/// Result of expanding a macro.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpandResult {
    /// The expanded source text.
    pub expanded: String,
    /// Names introduced (bound) by the expansion.
    pub introduced_names: Vec<MacroVar>,
    /// Names referenced (used) by the expansion.
    pub used_names: Vec<MacroVar>,
}

/// Describes what kind of hygiene violation occurred.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ViolationKind {
    /// A free variable in the macro body would be captured by a binding at the call site.
    CapturingFree,
    /// A bound variable in the macro body would capture a name from the call site.
    CapturingBound,
    /// A name introduced in the expansion shadows an outer binding.
    ShadowingOuter,
}

impl std::fmt::Display for ViolationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ViolationKind::CapturingFree => write!(f, "capturing-free"),
            ViolationKind::CapturingBound => write!(f, "capturing-bound"),
            ViolationKind::ShadowingOuter => write!(f, "shadowing-outer"),
        }
    }
}

/// A detected hygiene violation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HygieneViolation {
    /// The name involved in the violation.
    pub name: String,
    /// The scope where the name was defined.
    pub def_scope: ScopeId,
    /// The scope where the name is being used.
    pub use_scope: ScopeId,
    /// What kind of violation this is.
    pub kind: ViolationKind,
}

impl std::fmt::Display for HygieneViolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "hygiene violation ({}) for '{}': defined in {} but used in {}",
            self.kind, self.name, self.def_scope, self.use_scope
        )
    }
}
