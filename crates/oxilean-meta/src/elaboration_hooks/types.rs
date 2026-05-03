//! Types for elaboration hooks — callbacks that run during elaboration.

use std::fmt;

/// The kind of elaboration event that triggers a hook.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ElabHookKind {
    /// Fires before elaboration of an expression begins.
    PreElaborate,
    /// Fires after successful elaboration of an expression.
    PostElaborate,
    /// Fires when an elaboration error occurs.
    OnError,
    /// Fires when a declaration (def, theorem, etc.) is processed.
    OnDeclaration,
    /// Fires at the start of a tactic block.
    OnTacticBegin,
    /// Fires at the end of a tactic block.
    OnTacticEnd,
}

impl fmt::Display for ElabHookKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ElabHookKind::PreElaborate => write!(f, "PreElaborate"),
            ElabHookKind::PostElaborate => write!(f, "PostElaborate"),
            ElabHookKind::OnError => write!(f, "OnError"),
            ElabHookKind::OnDeclaration => write!(f, "OnDeclaration"),
            ElabHookKind::OnTacticBegin => write!(f, "OnTacticBegin"),
            ElabHookKind::OnTacticEnd => write!(f, "OnTacticEnd"),
        }
    }
}

/// Descriptor for a registered elaboration hook.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElabHook {
    /// Unique identifier assigned by the registry.
    pub id: u64,
    /// The kind of event this hook responds to.
    pub kind: ElabHookKind,
    /// Priority: lower values fire first.
    pub priority: i32,
    /// Human-readable name for diagnostics.
    pub name: String,
}

impl ElabHook {
    /// Create a new hook descriptor.
    pub fn new(id: u64, kind: ElabHookKind, priority: i32, name: impl Into<String>) -> Self {
        Self {
            id,
            kind,
            priority,
            name: name.into(),
        }
    }
}

impl fmt::Display for ElabHook {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Hook({}, {}, prio={})",
            self.name, self.kind, self.priority
        )
    }
}

/// An event passed to hooks when elaboration reaches a relevant point.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HookEvent {
    /// The kind of elaboration event.
    pub kind: ElabHookKind,
    /// The name of the declaration being processed, if applicable.
    pub decl_name: Option<String>,
    /// The expression being elaborated, rendered as a string, if applicable.
    pub expr: Option<String>,
    /// The error message, if this is an error event.
    pub error: Option<String>,
}

impl HookEvent {
    /// Create a new hook event.
    pub fn new(kind: ElabHookKind) -> Self {
        Self {
            kind,
            decl_name: None,
            expr: None,
            error: None,
        }
    }

    /// Builder: set the declaration name.
    pub fn with_decl(mut self, name: impl Into<String>) -> Self {
        self.decl_name = Some(name.into());
        self
    }

    /// Builder: set the expression string.
    pub fn with_expr(mut self, expr: impl Into<String>) -> Self {
        self.expr = Some(expr.into());
        self
    }

    /// Builder: set the error message.
    pub fn with_error(mut self, err: impl Into<String>) -> Self {
        self.error = Some(err.into());
        self
    }
}

/// The result returned by a hook after processing an event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HookResult {
    /// Continue elaboration normally.
    Continue,
    /// Abort elaboration with an error message.
    Abort(String),
    /// Replace the elaborated expression with the given string.
    Modify(String),
}

impl fmt::Display for HookResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HookResult::Continue => write!(f, "Continue"),
            HookResult::Abort(msg) => write!(f, "Abort({msg})"),
            HookResult::Modify(expr) => write!(f, "Modify({expr})"),
        }
    }
}

/// Registry of all registered elaboration hooks.
#[derive(Debug, Clone, Default)]
pub struct HookRegistry {
    /// All registered hooks.
    pub hooks: Vec<ElabHook>,
    /// Counter for assigning unique IDs.
    pub next_id: u64,
}

impl HookRegistry {
    /// Create a new, empty registry.
    pub fn new() -> Self {
        Self {
            hooks: Vec::new(),
            next_id: 1,
        }
    }
}

/// A record of hook executions during an elaboration event.
#[derive(Debug, Clone, Default)]
pub struct HookTrace {
    /// Ordered list of (hook, result) pairs.
    pub events: Vec<(ElabHook, HookResult)>,
}

impl HookTrace {
    /// Create a new empty trace.
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    /// Append a hook execution to the trace.
    pub fn record(&mut self, hook: ElabHook, result: HookResult) {
        self.events.push((hook, result));
    }

    /// Return the number of recorded events.
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Return true if no hooks were recorded.
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}
