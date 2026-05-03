//! Types for tactic reflection — inspect and construct tactic expressions at meta-level.

use std::fmt;

/// Direction for rewrite tactics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RewriteDir {
    /// Rewrite left-to-right (forward).
    LeftToRight,
    /// Rewrite right-to-left (backward).
    RightToLeft,
}

impl fmt::Display for RewriteDir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RewriteDir::LeftToRight => write!(f, "←→"),
            RewriteDir::RightToLeft => write!(f, "←"),
        }
    }
}

/// Reflective representation of tactics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TacticRepr {
    /// Apply a term to the current goal.
    Apply { expr: String },
    /// Introduce one or more names.
    Intro { names: Vec<String> },
    /// Rewrite using a hypothesis.
    Rewrite { hyp: String, dir: RewriteDir },
    /// Introduce a helper lemma with proof.
    Have {
        name: String,
        type_: String,
        proof: Box<TacticRepr>,
    },
    /// Close the goal with an exact term.
    Exact { expr: String },
    /// Simplify using named lemmas.
    Simp { lemmas: Vec<String> },
    /// Execute tactics sequentially.
    Seq(Vec<TacticRepr>),
    /// Try tactics in order until one succeeds.
    Alt(Vec<TacticRepr>),
    /// Repeat a tactic until it fails.
    Repeat(Box<TacticRepr>),
    /// Try a tactic; succeed even if it fails.
    Try(Box<TacticRepr>),
    /// Focus on a specific goal by index.
    Focus {
        goal_idx: usize,
        tac: Box<TacticRepr>,
    },
    /// Raw tactic string (unparsed).
    Raw(String),
}

/// A hypothesis paired with its type, plus the proof goal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GoalRepr {
    /// Hypotheses as (name, type) pairs.
    pub hyps: Vec<(String, String)>,
    /// The goal type to be proved.
    pub target: String,
}

impl GoalRepr {
    /// Create a new goal with the given hypotheses and target.
    pub fn new(hyps: Vec<(String, String)>, target: String) -> Self {
        Self { hyps, target }
    }

    /// Create a simple goal with no hypotheses.
    pub fn simple(target: impl Into<String>) -> Self {
        Self {
            hyps: Vec::new(),
            target: target.into(),
        }
    }
}

impl fmt::Display for GoalRepr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (name, ty) in &self.hyps {
            writeln!(f, "{name} : {ty}")?;
        }
        write!(f, "⊢ {}", self.target)
    }
}

/// A sequence of tactics forming a proof script.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TacticScript {
    /// The ordered sequence of tactic steps.
    pub steps: Vec<TacticRepr>,
}

impl TacticScript {
    /// Create a new empty tactic script.
    pub fn new() -> Self {
        Self { steps: Vec::new() }
    }

    /// Create a script from a list of steps.
    pub fn from_steps(steps: Vec<TacticRepr>) -> Self {
        Self { steps }
    }

    /// Append a step to the script.
    pub fn push(&mut self, step: TacticRepr) {
        self.steps.push(step);
    }

    /// Return the number of steps.
    pub fn len(&self) -> usize {
        self.steps.len()
    }

    /// Return true if the script has no steps.
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }
}

/// The current proof state: a list of goals and a focus index.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReflectionCtx {
    /// All remaining proof goals.
    pub goals: Vec<GoalRepr>,
    /// Index of the focused goal.
    pub focus: usize,
}

impl ReflectionCtx {
    /// Create a new context with the given goals, focused on index 0.
    pub fn new(goals: Vec<GoalRepr>) -> Self {
        Self { goals, focus: 0 }
    }

    /// Create an empty context (no goals — proof complete).
    pub fn empty() -> Self {
        Self {
            goals: Vec::new(),
            focus: 0,
        }
    }

    /// Return the currently focused goal, if any.
    pub fn focused_goal(&self) -> Option<&GoalRepr> {
        self.goals.get(self.focus)
    }

    /// Return the number of remaining goals.
    pub fn goal_count(&self) -> usize {
        self.goals.len()
    }

    /// Return true if all goals are closed.
    pub fn is_complete(&self) -> bool {
        self.goals.is_empty()
    }
}
