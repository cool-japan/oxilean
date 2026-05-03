//! Types for the `norm_cast` tactic.
//!
//! The `norm_cast` tactic normalises coercions (type casts) in goals and
//! hypotheses, pushing them into a canonical position (usually inward).

/// A single cast rule describing how to rewrite a cast expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CastRule {
    /// The source type of the coercion.
    pub from_type: String,
    /// The destination type of the coercion.
    pub to_type: String,
    /// The name of the cast function / coercion function.
    pub cast_fn: String,
    /// Priority used to order rule application (higher = applied first).
    pub priority: i32,
}

/// A registry of cast rules, kept sorted by descending priority.
#[derive(Debug, Clone, Default)]
pub struct CastRuleDB {
    rules: Vec<CastRule>,
}

impl CastRuleDB {
    /// Create an empty rule database.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a rule to the database, maintaining priority order.
    pub fn add_rule(&mut self, rule: CastRule) {
        // Insert so that higher-priority rules come first.
        let pos = self.rules.partition_point(|r| r.priority >= rule.priority);
        self.rules.insert(pos, rule);
    }

    /// Find all rules that match the given source/destination type pair,
    /// returned in descending priority order (highest priority first).
    pub fn find_rules(&self, from: &str, to: &str) -> Vec<CastRule> {
        self.rules
            .iter()
            .filter(|r| r.from_type == from && r.to_type == to)
            .cloned()
            .collect()
    }

    /// Return all rules in the database (highest priority first).
    pub fn all_rules(&self) -> &[CastRule] {
        &self.rules
    }
}

/// Configuration that controls how `norm_cast` operates.
#[derive(Debug, Clone)]
pub struct NormCastConfig {
    /// When `true`, coercions are pushed as far inward as possible.
    pub push_casts_inward: bool,
    /// When `true`, same-type cast chains `T → T → U` are collapsed.
    pub squash_casts: bool,
    /// When `true`, numeric literals in different types are normalised.
    pub normalize_numerals: bool,
}

impl Default for NormCastConfig {
    fn default() -> Self {
        Self {
            push_casts_inward: true,
            squash_casts: true,
            normalize_numerals: true,
        }
    }
}

/// The result returned after running cast normalisation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CastNormResult {
    /// Whether the expression was modified at all.
    pub changed: bool,
    /// Number of rewrite steps performed.
    pub num_steps: u32,
    /// The normalised expression string.
    pub result_expr: String,
}

/// A single rewrite step recorded during cast normalisation.
#[derive(Debug, Clone)]
pub struct CastRewriteStep {
    /// The rule that was applied.
    pub rule: CastRule,
    /// Position in the expression (character offset) where it was applied.
    pub position: usize,
    /// The sub-expression before rewriting.
    pub before: String,
    /// The sub-expression after rewriting.
    pub after: String,
}

/// Working state accumulated while running `norm_cast` on a single goal.
#[derive(Debug, Clone)]
pub struct NormCastTacticState {
    /// The current (possibly partially normalised) goal expression.
    pub goal_expr: String,
    /// The ordered sequence of cast rewrites applied so far.
    pub rewrite_steps: Vec<CastRewriteStep>,
}

impl NormCastTacticState {
    /// Create a new state for the given goal expression.
    pub fn new(goal_expr: impl Into<String>) -> Self {
        Self {
            goal_expr: goal_expr.into(),
            rewrite_steps: Vec::new(),
        }
    }

    /// Record a rewrite step.
    pub fn record_step(&mut self, step: CastRewriteStep) {
        self.goal_expr = step.after.clone();
        self.rewrite_steps.push(step);
    }

    /// Total number of steps recorded so far.
    pub fn num_steps(&self) -> u32 {
        self.rewrite_steps.len() as u32
    }
}

/// The result of applying `norm_cast` to a full goal with hypotheses.
#[derive(Debug, Clone)]
pub struct NormCastResult {
    /// The normalised goal expression.
    pub goal: String,
    /// The normalised hypotheses: (name, normalised-type).
    pub hypotheses: Vec<(String, String)>,
    /// Whether any expression was changed (goal or any hypothesis).
    pub changed: bool,
    /// Total rewrite steps across goal and all hypotheses.
    pub total_steps: u32,
}
