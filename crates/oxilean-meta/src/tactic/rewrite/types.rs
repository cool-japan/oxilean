//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use crate::basic::MetaContext;
use crate::tactic::state::{TacticResult, TacticState};
use oxilean_kernel::{Expr, Name};

/// A database of rewrite hints.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct RewriteHintDb {
    /// The hints, in insertion order.
    pub hints: Vec<RewriteHint>,
}
#[allow(dead_code)]
impl RewriteHintDb {
    /// Create an empty hint database.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a hint.
    pub fn add(&mut self, lemma: Name, priority: i32, direction: RewriteDirection) {
        self.hints.push(RewriteHint {
            lemma,
            priority,
            direction,
        });
    }
    /// Sort by priority (highest first).
    pub fn sort_by_priority(&mut self) {
        self.hints.sort_by(|a, b| b.priority.cmp(&a.priority));
    }
    /// Get all hint names.
    pub fn names(&self) -> Vec<&Name> {
        self.hints.iter().map(|h| &h.lemma).collect()
    }
    /// Number of hints.
    pub fn len(&self) -> usize {
        self.hints.len()
    }
    /// Whether the database is empty.
    pub fn is_empty(&self) -> bool {
        self.hints.is_empty()
    }
}
/// Result of matching a rewrite pattern against an expression.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct MatchResult {
    /// Variable assignments from the match.
    pub assignments: std::collections::HashMap<Name, Expr>,
}
#[allow(dead_code)]
impl MatchResult {
    /// Create an empty match result.
    pub fn empty() -> Self {
        MatchResult {
            assignments: std::collections::HashMap::new(),
        }
    }
    /// Assign a pattern variable.
    pub fn assign(&mut self, var: Name, val: Expr) -> bool {
        if let Some(existing) = self.assignments.get(&var) {
            existing == &val
        } else {
            self.assignments.insert(var, val);
            true
        }
    }
    /// Look up a pattern variable.
    pub fn get(&self, var: &Name) -> Option<&Expr> {
        self.assignments.get(var)
    }
    /// Apply substitution to an expression.
    pub fn apply_to(&self, expr: &Expr) -> Expr {
        match expr {
            Expr::Const(name, levels) => {
                if let Some(replacement) = self.assignments.get(name) {
                    replacement.clone()
                } else {
                    Expr::Const(name.clone(), levels.clone())
                }
            }
            Expr::App(f, a) => {
                let f2 = self.apply_to(f);
                let a2 = self.apply_to(a);
                Expr::App(Box::new(f2), Box::new(a2))
            }
            Expr::Lam(bi, n, ty, body) => {
                let ty2 = self.apply_to(ty);
                let body2 = self.apply_to(body);
                Expr::Lam(*bi, n.clone(), Box::new(ty2), Box::new(body2))
            }
            Expr::Pi(bi, n, ty, body) => {
                let ty2 = self.apply_to(ty);
                let body2 = self.apply_to(body);
                Expr::Pi(*bi, n.clone(), Box::new(ty2), Box::new(body2))
            }
            other => other.clone(),
        }
    }
}
/// A single step in a rewrite chain.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct RewriteStep {
    /// The name of the rule applied.
    pub rule_name: Name,
    /// The direction.
    pub direction: RewriteDirection,
    /// The occurrence index (-1 = all occurrences).
    pub occurrence: i32,
    /// The expression before this step.
    pub before: Expr,
    /// The expression after this step.
    pub after: Expr,
}
/// A setoid-aware rewrite: rewrites modulo a given equivalence relation.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SetoidRewrite {
    /// The equivalence relation name.
    pub rel_name: String,
    /// The hypothesis name.
    pub hyp_name: Name,
    /// Direction of the rewrite.
    pub direction: RewriteDirection,
}
#[allow(dead_code)]
impl SetoidRewrite {
    /// Create a new setoid rewrite.
    pub fn new(rel_name: &str, hyp_name: Name, direction: RewriteDirection) -> Self {
        Self {
            rel_name: rel_name.to_string(),
            hyp_name,
            direction,
        }
    }
    /// Check if this is a forward setoid rewrite.
    pub fn is_forward(&self) -> bool {
        self.direction == RewriteDirection::Forward
    }
    /// Check if this is a backward setoid rewrite.
    pub fn is_backward(&self) -> bool {
        self.direction == RewriteDirection::Backward
    }
    /// Render a human-readable description of this rewrite.
    pub fn describe(&self) -> String {
        let dir = match self.direction {
            RewriteDirection::Forward => "→",
            RewriteDirection::Backward => "←",
        };
        format!(
            "setoid_rw [{}{}] (rel: {})",
            dir, self.hyp_name, self.rel_name
        )
    }
}
/// Statistics for a rewrite tactic run.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct RewriteTacticStats {
    /// Number of hypotheses tried.
    pub hyps_tried: usize,
    /// Number of successful rewrites.
    pub rewrites_applied: usize,
    /// Number of goals closed.
    pub goals_closed: usize,
    /// Number of goals opened (sub-goals from conditions).
    pub goals_opened: usize,
    /// Whether the run was aborted early.
    pub aborted: bool,
}
#[allow(dead_code)]
impl RewriteTacticStats {
    /// Create zeroed stats.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a successful rewrite.
    pub fn record_success(&mut self) {
        self.rewrites_applied += 1;
    }
    /// Record a failed rewrite attempt.
    pub fn record_failure(&mut self) {
        self.hyps_tried += 1;
    }
    /// Whether any progress was made.
    pub fn any_progress(&self) -> bool {
        self.rewrites_applied > 0
    }
    /// Summary string.
    pub fn summary(&self) -> String {
        format!(
            "rw_stats(applied={}, tried={}, closed={})",
            self.rewrites_applied, self.hyps_tried, self.goals_closed
        )
    }
}
/// A named hint for the rewrite tactic.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct RewriteHint {
    /// The lemma name.
    pub lemma: Name,
    /// The priority (higher = applied first).
    pub priority: i32,
    /// The direction.
    pub direction: RewriteDirection,
}
/// Annotation on a rewrite hypothesis, controlling how it is used.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct RewriteHypAnnotation {
    /// The hypothesis name.
    pub name: Name,
    /// Whether to apply it at most once.
    pub once: bool,
    /// Whether to apply it to hypotheses as well as the goal.
    pub all: bool,
    /// Whether to flip direction.
    pub reverse: bool,
}
#[allow(dead_code)]
impl RewriteHypAnnotation {
    /// Create a default annotation for a named hypothesis.
    pub fn default_for(name: Name) -> Self {
        Self {
            name,
            once: false,
            all: false,
            reverse: false,
        }
    }
    /// Set the once flag.
    pub fn once(mut self) -> Self {
        self.once = true;
        self
    }
    /// Set the all flag.
    pub fn all(mut self) -> Self {
        self.all = true;
        self
    }
    /// Set the reverse flag.
    pub fn reverse(mut self) -> Self {
        self.reverse = true;
        self
    }
    /// Get the rewrite direction based on the reverse flag.
    pub fn direction(&self) -> RewriteDirection {
        if self.reverse {
            RewriteDirection::Backward
        } else {
            RewriteDirection::Forward
        }
    }
}
/// A pending rewrite sequence: a list of (hyp_name, direction) pairs.
#[allow(dead_code)]
pub struct RewriteSeq {
    /// Steps: each entry is (hypothesis name, is_reversed).
    pub steps: Vec<(Name, bool)>,
}
#[allow(dead_code)]
impl RewriteSeq {
    /// Create an empty rewrite sequence.
    pub fn empty() -> Self {
        RewriteSeq { steps: Vec::new() }
    }
    /// Add a forward rewrite step.
    pub fn then(mut self, name: Name) -> Self {
        self.steps.push((name, false));
        self
    }
    /// Add a backward rewrite step.
    pub fn then_rev(mut self, name: Name) -> Self {
        self.steps.push((name, true));
        self
    }
    /// Return the number of steps.
    pub fn len(&self) -> usize {
        self.steps.len()
    }
    /// Return true iff the sequence is empty.
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }
    /// Execute all steps in the sequence on a tactic state.
    ///
    /// For each step `(name, reversed)`, performs a named rewrite:
    /// forward if `reversed` is false, backward if true.
    pub fn execute(
        &self,
        state: &mut TacticState,
        ctx: &mut MetaContext,
    ) -> crate::tactic::state::TacticResult<()> {
        for (name, reversed) in &self.steps {
            let dir = if *reversed {
                RewriteDirection::Backward
            } else {
                RewriteDirection::Forward
            };
            tac_rewrite_named(name, dir, state, ctx)?;
        }
        Ok(())
    }
}
/// Information about an equality hypothesis suitable for rewriting.
#[allow(dead_code)]
pub struct RewriteHypInfo {
    /// The hypothesis name.
    pub name: String,
    /// The left-hand side of the equality.
    pub lhs: Expr,
    /// The right-hand side of the equality.
    pub rhs: Expr,
}
/// A collection of rewrite rules forming a rewrite system.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct RewriteSystem {
    /// The rules in priority order (earlier = higher priority).
    pub rules: Vec<RewriteRule>,
}
#[allow(dead_code)]
impl RewriteSystem {
    /// Create an empty rewrite system.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a rule.
    pub fn add_rule(&mut self, rule: RewriteRule) {
        self.rules.push(rule);
    }
    /// Count the rules.
    pub fn len(&self) -> usize {
        self.rules.len()
    }
    /// Check if the system is empty.
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
    /// Try to apply any rule to the outermost expression.
    pub fn apply_once(&self, expr: &Expr) -> Option<Expr> {
        for rule in &self.rules {
            if let Some(result) = rule.apply(expr) {
                return Some(result);
            }
        }
        None
    }
    /// Apply rules exhaustively to normal form (up to `limit` steps).
    pub fn normalize(&self, expr: &Expr, limit: usize) -> (Expr, usize) {
        let mut current = expr.clone();
        let mut steps = 0;
        loop {
            if steps >= limit {
                break;
            }
            let rewritten = self.apply_once(&current);
            match rewritten {
                Some(new_expr) => {
                    current = new_expr;
                    steps += 1;
                }
                None => break,
            }
        }
        (current, steps)
    }
    /// Apply rules anywhere in the expression (one pass, left-to-right).
    pub fn apply_anywhere_once(&self, expr: &Expr) -> Option<Expr> {
        for rule in &self.rules {
            if let Some(result) = rule.apply_anywhere(expr) {
                return Some(result);
            }
        }
        None
    }
}
/// Configuration for the rewrite normalization loop.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct RewriteLoopConfig {
    /// Maximum number of rewrite steps.
    pub max_steps: usize,
    /// Whether to apply rules in random order.
    pub randomize: bool,
    /// Whether to stop after the first successful rewrite.
    pub stop_after_first: bool,
}
#[allow(dead_code)]
impl RewriteLoopConfig {
    /// Default configuration.
    pub fn default_config() -> Self {
        Self {
            max_steps: 1000,
            randomize: false,
            stop_after_first: false,
        }
    }
    /// Config that stops after first success.
    pub fn single_step() -> Self {
        Self {
            max_steps: 1,
            randomize: false,
            stop_after_first: true,
        }
    }
}
/// A position in an expression tree, described as a path of child indices.
///
/// - `0` = function in `App(f, a)`
/// - `1` = argument in `App(f, a)`
/// - `2` = body of `Lam`/`Pi`
/// - `3` = type annotation of `Lam`/`Pi`
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RewritePosition(pub Vec<usize>);
#[allow(dead_code)]
impl RewritePosition {
    /// The root position.
    pub fn root() -> Self {
        RewritePosition(Vec::new())
    }
    /// Extend the position by one step.
    pub fn extend(&self, step: usize) -> Self {
        let mut path = self.0.clone();
        path.push(step);
        RewritePosition(path)
    }
    /// The depth of this position.
    pub fn depth(&self) -> usize {
        self.0.len()
    }
    /// Check if this is the root.
    pub fn is_root(&self) -> bool {
        self.0.is_empty()
    }
}
/// Information about an equality type `@Eq α a b`.
#[derive(Clone, Debug)]
pub struct EqualityInfo {
    /// The type `α`.
    pub ty: Expr,
    /// The left-hand side `a`.
    pub lhs: Expr,
    /// The right-hand side `b`.
    pub rhs: Expr,
}
impl EqualityInfo {
    /// Try to extract equality info from an expression.
    ///
    /// Expects `@Eq α a b` form.
    pub fn from_expr(expr: &Expr) -> Option<Self> {
        if let Expr::App(eq_a_lhs, rhs) = expr {
            if let Expr::App(eq_a, lhs) = eq_a_lhs.as_ref() {
                if let Expr::App(eq_const, alpha) = eq_a.as_ref() {
                    if matches!(
                        eq_const.as_ref(), Expr::Const(name, _) if * name ==
                        Name::str("Eq")
                    ) {
                        return Some(EqualityInfo {
                            ty: *alpha.clone(),
                            lhs: *lhs.clone(),
                            rhs: *rhs.clone(),
                        });
                    }
                }
            }
        }
        None
    }
}
/// Direction of rewriting.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RewriteDirection {
    /// Rewrite left-to-right (default): replace LHS with RHS.
    Forward,
    /// Rewrite right-to-left: replace RHS with LHS.
    Backward,
}
/// A symbolic rewrite sequence.
#[derive(Clone, Debug, Default)]
pub struct RewriteSequence {
    /// The rewrites to perform.
    pub rewrites: Vec<(Name, RewriteDirection)>,
}
impl RewriteSequence {
    /// Create empty.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a forward rewrite.
    pub fn then_forward(mut self, name: Name) -> Self {
        self.rewrites.push((name, RewriteDirection::Forward));
        self
    }
    /// Add a backward rewrite.
    pub fn then_backward(mut self, name: Name) -> Self {
        self.rewrites.push((name, RewriteDirection::Backward));
        self
    }
    /// Execute all rewrites.
    pub fn execute(&self, state: &mut TacticState, ctx: &mut MetaContext) -> TacticResult<usize> {
        let mut count = 0;
        for (name, dir) in &self.rewrites {
            tac_rewrite_named(name, *dir, state, ctx)?;
            count += 1;
        }
        Ok(count)
    }
}
/// A rewrite rule: LHS pattern, RHS replacement, optional side conditions.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct RewriteRule {
    /// The left-hand side pattern.
    pub lhs: Expr,
    /// The right-hand side.
    pub rhs: Expr,
    /// The name of the rule (e.g., a lemma name).
    pub name: Name,
    /// Conditions that must hold for the rule to fire.
    pub conditions: Vec<Expr>,
    /// Direction: forward = lhs → rhs, backward = rhs → lhs.
    pub direction: RewriteDirection,
}
#[allow(dead_code)]
impl RewriteRule {
    /// Create a new unconditional rewrite rule.
    pub fn new(name: Name, lhs: Expr, rhs: Expr) -> Self {
        Self {
            lhs,
            rhs,
            name,
            conditions: Vec::new(),
            direction: RewriteDirection::Forward,
        }
    }
    /// Create a rule with a given direction.
    pub fn with_direction(mut self, dir: RewriteDirection) -> Self {
        self.direction = dir;
        self
    }
    /// Add a side condition.
    pub fn with_condition(mut self, cond: Expr) -> Self {
        self.conditions.push(cond);
        self
    }
    /// Check whether this is a conditional rule.
    pub fn is_conditional(&self) -> bool {
        !self.conditions.is_empty()
    }
    /// Apply the rule to an expression. Returns the rewritten expression if the pattern matches.
    pub fn apply(&self, expr: &Expr) -> Option<Expr> {
        if self.is_conditional() {
            return None;
        }
        let (pattern, replacement) = match self.direction {
            RewriteDirection::Forward => (&self.lhs, &self.rhs),
            RewriteDirection::Backward => (&self.rhs, &self.lhs),
        };
        if expr == pattern {
            Some(replacement.clone())
        } else {
            None
        }
    }
    /// Apply the rule anywhere in `expr` (subterm rewriting).
    pub fn apply_anywhere(&self, expr: &Expr) -> Option<Expr> {
        if self.is_conditional() {
            return None;
        }
        let (pattern, replacement) = match self.direction {
            RewriteDirection::Forward => (&self.lhs, &self.rhs),
            RewriteDirection::Backward => (&self.rhs, &self.lhs),
        };
        let new_expr = replace_subexpr(expr, pattern, replacement);
        if new_expr == *expr {
            None
        } else {
            Some(new_expr)
        }
    }
}
/// A pattern variable for matching rewrite rules.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PatternVar(pub Name);
/// A trace of multiple rewrite steps.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct RewriteTrace {
    /// The steps taken.
    pub steps: Vec<RewriteStep>,
    /// The final expression.
    pub final_expr: Option<Expr>,
}
#[allow(dead_code)]
impl RewriteTrace {
    /// Create an empty trace.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a step.
    pub fn push(&mut self, step: RewriteStep) {
        self.steps.push(step);
    }
    /// How many steps were taken.
    pub fn len(&self) -> usize {
        self.steps.len()
    }
    /// Whether no steps were taken.
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }
    /// Return the expression after all rewrites (or the original if empty).
    pub fn result(&self) -> Option<&Expr> {
        self.final_expr.as_ref()
    }
    /// Return a summary string.
    pub fn summary(&self) -> String {
        if self.steps.is_empty() {
            return "no rewrites".to_string();
        }
        let names: Vec<String> = self.steps.iter().map(|s| s.rule_name.to_string()).collect();
        format!("{} step(s): [{}]", self.steps.len(), names.join(", "))
    }
}
