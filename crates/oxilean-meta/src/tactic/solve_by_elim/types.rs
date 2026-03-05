//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use crate::basic::{MVarId, MetaContext, MetaState, MetavarKind};
use crate::def_eq::MetaDefEq;
use crate::tactic::state::TacticState;
use oxilean_kernel::{Expr, Level, Name};

/// Saved state for one level of the backtracking search.
#[derive(Clone, Debug)]
pub struct BacktrackState {
    /// Goal IDs that were active when this state was saved.
    pub saved_goals: Vec<MVarId>,
    /// The candidate that was applied at this node.
    pub applied_lemma: Option<Candidate>,
    /// Remaining candidates that have not yet been tried.
    pub remaining_alternatives: Vec<Candidate>,
    /// Depth at which this backtrack point was created.
    pub depth: usize,
    /// MetaContext snapshot for restoring assignments.
    pub meta_snapshot: MetaState,
}
/// The backward chaining search engine.
///
/// Maintains the search tree, backtrack stack, and statistics. Separated from
/// the top-level entry points so that it can be tested independently.
pub(super) struct SearchEngine<'a> {
    /// Configuration.
    pub(super) config: &'a SolveByElimConfig,
    /// User-provided lemmas.
    pub(super) lemmas: &'a [Expr],
    /// Backtrack stack.
    pub(super) backtrack_stack: Vec<BacktrackState>,
    /// Collected statistics.
    pub(super) stats: SearchStats,
    /// Global backtrack budget remaining.
    pub(super) backtracks_remaining: usize,
}
impl<'a> SearchEngine<'a> {
    /// Create a new search engine.
    pub(super) fn new(config: &'a SolveByElimConfig, lemmas: &'a [Expr]) -> Self {
        Self {
            config,
            lemmas,
            backtrack_stack: Vec::new(),
            stats: SearchStats::new(),
            backtracks_remaining: config.max_backtrack,
        }
    }
    /// Collect all candidate lemmas for the current goal.
    pub(super) fn collect_candidates(
        &self,
        goal_target: &Expr,
        ctx: &MetaContext,
    ) -> Vec<Candidate> {
        let mut candidates = Vec::new();
        if self.config.use_hyps {
            let hyps = ctx.get_local_hyps();
            for (name, ty) in hyps.iter().rev() {
                let estimated = estimate_arity(&ctx.instantiate_mvars(ty));
                candidates.push(Candidate {
                    expr: Expr::Const(name.clone(), vec![]),
                    source: CandidateSource::LocalHyp(name.clone()),
                    estimated_subgoals: estimated,
                    result_type: Some(ctx.instantiate_mvars(ty)),
                });
            }
        }
        for (i, lemma) in self.lemmas.iter().enumerate() {
            let estimated = estimate_arity(lemma);
            candidates.push(Candidate {
                expr: lemma.clone(),
                source: CandidateSource::ProvidedLemma(i),
                estimated_subgoals: estimated,
                result_type: None,
            });
        }
        if self.config.use_exfalso {
            candidates.push(Candidate {
                expr: Expr::Const(Name::str("False.elim"), vec![Level::zero()]),
                source: CandidateSource::EnvironmentDecl(Name::str("False.elim")),
                estimated_subgoals: 1,
                result_type: None,
            });
        }
        candidates.sort_by_key(|c| c.estimated_subgoals);
        let goal_head = get_head_name(goal_target);
        if let Some(ref gh) = goal_head {
            let filtered: Vec<Candidate> = candidates
                .into_iter()
                .filter(|c| {
                    if let Some(ref rt) = c.result_type {
                        let rt_result = get_result_type(rt);
                        let rh = get_head_name(&rt_result);
                        rh.is_none() || rh.as_ref() == Some(gh)
                    } else {
                        true
                    }
                })
                .collect();
            return filtered;
        }
        candidates
    }
    /// Attempt to apply a candidate to the current goal.
    ///
    /// On success, returns the list of new subgoal MVarIds.
    /// On failure, returns None.
    pub(super) fn try_apply_candidate(
        &mut self,
        candidate: &Candidate,
        state: &mut TacticState,
        ctx: &mut MetaContext,
    ) -> Option<Vec<MVarId>> {
        self.stats.record_candidate_tried();
        let goal = match state.current_goal() {
            Ok(g) => g,
            Err(_) => return None,
        };
        let target = match ctx.get_mvar_type(goal) {
            Some(t) => ctx.instantiate_mvars(t),
            None => return None,
        };
        let candidate_expr = &candidate.expr;
        let arity = candidate.estimated_subgoals;
        let arg_types = candidate
            .result_type
            .as_ref()
            .map(|ty| peel_pi_arg_types(ty, arity))
            .unwrap_or_else(|| vec![Expr::Sort(Level::zero()); arity]);
        let mut arg_mvars: Vec<(MVarId, Expr)> = Vec::new();
        for i in 0..arity {
            let arg_ty = arg_types
                .get(i)
                .cloned()
                .unwrap_or_else(|| Expr::Sort(Level::zero()));
            let (mid, mex) = ctx.mk_fresh_expr_mvar(arg_ty, MetavarKind::Natural);
            arg_mvars.push((mid, mex));
        }
        let mut app_expr = candidate_expr.clone();
        for (_mid, mex) in &arg_mvars {
            app_expr = Expr::App(Box::new(app_expr), Box::new(mex.clone()));
        }
        let mut deq = MetaDefEq::new();
        let result_ty = candidate
            .result_type
            .as_ref()
            .map(|ty| compute_result_type(ty, arity))
            .unwrap_or_else(|| Expr::Sort(Level::zero()));
        let result_ty_inst = ctx.instantiate_mvars(&result_ty);
        let target_inst = ctx.instantiate_mvars(&target);
        let unify_ok = if result_ty_inst != Expr::Sort(Level::zero()) {
            deq.is_def_eq(&result_ty_inst, &target_inst, ctx).is_equal()
        } else {
            try_unify_app(&app_expr, &target_inst, ctx)
        };
        if !unify_ok {
            self.stats.record_failed_apply();
            return None;
        }
        let app_inst = ctx.instantiate_mvars(&app_expr);
        ctx.reassign_mvar(goal, app_inst);
        let new_goals: Vec<MVarId> = arg_mvars
            .iter()
            .filter(|(mid, _)| !ctx.is_mvar_assigned(*mid))
            .map(|(mid, _)| *mid)
            .collect();
        state.replace_goal(new_goals.clone());
        self.stats.record_successful_apply();
        Some(new_goals)
    }
    /// Run the backward chaining search on the currently focused goal.
    ///
    /// Returns `Ok(())` if the goal (and possibly all goals) are solved.
    pub(super) fn search(
        &mut self,
        state: &mut TacticState,
        ctx: &mut MetaContext,
        depth: usize,
    ) -> Result<(), SearchFailure> {
        self.stats.record_node();
        self.stats.record_depth(depth);
        if state.is_done() {
            return Ok(());
        }
        if depth >= self.config.max_depth {
            return Err(SearchFailure::DepthExceeded);
        }
        if self.try_trivial_close(state, ctx) {
            self.stats.record_goal_closed();
            if state.is_done() {
                return Ok(());
            }
            if self.config.all_goals {
                return self.search(state, ctx, depth);
            }
            return Ok(());
        }
        let goal_target = match self.get_current_target(state, ctx) {
            Some(t) => t,
            None => return Err(SearchFailure::NoGoals),
        };
        let candidates = self.collect_candidates(&goal_target, ctx);
        if candidates.is_empty() {
            return Err(SearchFailure::NoCandidates);
        }
        self.try_candidates(candidates, state, ctx, depth)
    }
    /// Try each candidate in order, backtracking on failure.
    pub(super) fn try_candidates(
        &mut self,
        candidates: Vec<Candidate>,
        state: &mut TacticState,
        ctx: &mut MetaContext,
        depth: usize,
    ) -> Result<(), SearchFailure> {
        let saved_goals = state.all_goals().to_vec();
        let meta_snapshot = ctx.save_state();
        for (idx, candidate) in candidates.iter().enumerate() {
            state.save();
            let inner_meta_snap = ctx.save_state();
            let remaining: Vec<Candidate> = candidates[idx + 1..].to_vec();
            self.backtrack_stack.push(BacktrackState {
                saved_goals: saved_goals.clone(),
                applied_lemma: Some(candidate.clone()),
                remaining_alternatives: remaining,
                depth,
                meta_snapshot: inner_meta_snap.clone(),
            });
            let apply_result = self.try_apply_candidate(candidate, state, ctx);
            match apply_result {
                Some(new_goals) => {
                    let recurse_result = if new_goals.is_empty() {
                        self.stats.record_goal_closed();
                        if state.is_done() || !self.config.all_goals {
                            Ok(())
                        } else {
                            self.search(state, ctx, depth)
                        }
                    } else {
                        self.solve_subgoals(state, ctx, depth + 1)
                    };
                    match recurse_result {
                        Ok(()) => {
                            self.backtrack_stack.pop();
                            return Ok(());
                        }
                        Err(_) => {
                            self.perform_backtrack(state, ctx, &inner_meta_snap)?;
                        }
                    }
                }
                None => {
                    self.perform_backtrack(state, ctx, &inner_meta_snap)?;
                }
            }
        }
        ctx.restore_state(meta_snapshot);
        Err(SearchFailure::AllCandidatesExhausted)
    }
    /// Solve all current subgoals recursively.
    pub(super) fn solve_subgoals(
        &mut self,
        state: &mut TacticState,
        ctx: &mut MetaContext,
        depth: usize,
    ) -> Result<(), SearchFailure> {
        while !state.is_done() {
            self.search(state, ctx, depth)?;
            if !self.config.all_goals {
                break;
            }
        }
        Ok(())
    }
    /// Perform a backtrack step: restore tactic state and meta context.
    pub(super) fn perform_backtrack(
        &mut self,
        state: &mut TacticState,
        ctx: &mut MetaContext,
        snapshot: &MetaState,
    ) -> Result<(), SearchFailure> {
        if self.backtracks_remaining == 0 {
            return Err(SearchFailure::BacktrackLimitReached);
        }
        self.backtracks_remaining -= 1;
        self.stats.record_backtrack();
        ctx.restore_state(snapshot.clone());
        let _ = state.restore();
        self.backtrack_stack.pop();
        Ok(())
    }
    /// Try to close the current goal trivially (assumption, rfl, True.intro).
    pub(super) fn try_trivial_close(&self, state: &mut TacticState, ctx: &mut MetaContext) -> bool {
        let goal = match state.current_goal() {
            Ok(g) => g,
            Err(_) => return false,
        };
        let target = match ctx.get_mvar_type(goal) {
            Some(t) => ctx.instantiate_mvars(t),
            None => return false,
        };
        if is_true_type(&target) {
            let proof = Expr::Const(Name::str("True.intro"), vec![]);
            if state.close_goal(proof, ctx).is_ok() {
                return true;
            }
        }
        if let Some(proof) = try_refl(&target) {
            if state.close_goal(proof, ctx).is_ok() {
                return true;
            }
        }
        if let Some(proof) = try_assumption(&target, ctx) {
            if state.close_goal(proof, ctx).is_ok() {
                return true;
            }
        }
        false
    }
    /// Get the target type of the currently focused goal.
    pub(super) fn get_current_target(
        &self,
        state: &TacticState,
        ctx: &MetaContext,
    ) -> Option<Expr> {
        let goal = state.current_goal().ok()?;
        let ty = ctx.get_mvar_type(goal)?;
        Some(ctx.instantiate_mvars(ty))
    }
    /// Extract the proof term for the original goal after the search succeeds.
    pub(super) fn extract_proof(&self, original_goal: MVarId, ctx: &MetaContext) -> Option<Expr> {
        ctx.get_mvar_assignment(original_goal)
            .map(|e| ctx.instantiate_mvars(e))
    }
}
/// A single candidate that can be applied to a goal.
#[derive(Clone, Debug)]
pub struct Candidate {
    /// The expression to apply.
    pub expr: Expr,
    /// Where this candidate originated.
    pub source: CandidateSource,
    /// Estimated number of subgoals after application (lower is better).
    pub estimated_subgoals: usize,
    /// The result type of this candidate (after all arguments are supplied).
    pub result_type: Option<Expr>,
}
/// Statistics collected during the search.
#[derive(Clone, Debug, Default)]
pub struct SearchStats {
    /// Total number of search nodes explored.
    pub nodes_explored: usize,
    /// Total number of backtracks performed.
    pub backtracks: usize,
    /// Maximum depth reached during the search.
    pub depth_reached: usize,
    /// Total number of candidates tried across all nodes.
    pub candidates_tried: usize,
    /// Number of successful applications (before potential backtrack).
    pub successful_applies: usize,
    /// Number of failed applications.
    pub failed_applies: usize,
    /// Number of goals that were closed during the search.
    pub goals_closed: usize,
}
impl SearchStats {
    /// Create empty statistics.
    pub fn new() -> Self {
        Self::default()
    }
    /// Reset all counters to zero.
    pub fn reset(&mut self) {
        *self = Self::default();
    }
    /// Record that a node was explored.
    pub(super) fn record_node(&mut self) {
        self.nodes_explored += 1;
    }
    /// Record a backtrack.
    pub(super) fn record_backtrack(&mut self) {
        self.backtracks += 1;
    }
    /// Update the maximum depth reached.
    pub(super) fn record_depth(&mut self, depth: usize) {
        if depth > self.depth_reached {
            self.depth_reached = depth;
        }
    }
    /// Record that a candidate was tried.
    pub(super) fn record_candidate_tried(&mut self) {
        self.candidates_tried += 1;
    }
    /// Record a successful application.
    pub(super) fn record_successful_apply(&mut self) {
        self.successful_applies += 1;
    }
    /// Record a failed application.
    pub(super) fn record_failed_apply(&mut self) {
        self.failed_applies += 1;
    }
    /// Record that a goal was closed.
    pub(super) fn record_goal_closed(&mut self) {
        self.goals_closed += 1;
    }
}
/// Configuration for the `solve_by_elim` tactic.
#[derive(Clone, Debug)]
pub struct SolveByElimConfig {
    /// Maximum recursion depth before giving up on a branch.
    pub max_depth: usize,
    /// Maximum total number of backtracks before giving up entirely.
    pub max_backtrack: usize,
    /// Whether to include local hypotheses as candidates.
    pub use_hyps: bool,
    /// Whether to try `False.elim` when no other candidate works.
    pub use_exfalso: bool,
    /// Whether to solve all remaining goals or just the focused one.
    pub all_goals: bool,
    /// Optional pre-processing label applied before each apply step.
    /// When `Some(name)`, a normalisation pass identified by `name` is run.
    pub pre_apply: Option<Name>,
    /// Whether to backtrack across goal boundaries.
    /// When true, failure on a later goal can undo work on earlier goals.
    pub backtrack_all: bool,
}
impl SolveByElimConfig {
    /// Create a new configuration with default values.
    pub fn new() -> Self {
        Self::default()
    }
    /// Builder: set max depth.
    pub fn with_max_depth(mut self, d: usize) -> Self {
        self.max_depth = d;
        self
    }
    /// Builder: set max backtrack count.
    pub fn with_max_backtrack(mut self, b: usize) -> Self {
        self.max_backtrack = b;
        self
    }
    /// Builder: set whether to use local hypotheses.
    pub fn with_use_hyps(mut self, v: bool) -> Self {
        self.use_hyps = v;
        self
    }
    /// Builder: set whether to try exfalso.
    pub fn with_use_exfalso(mut self, v: bool) -> Self {
        self.use_exfalso = v;
        self
    }
    /// Builder: set whether to solve all goals.
    pub fn with_all_goals(mut self, v: bool) -> Self {
        self.all_goals = v;
        self
    }
    /// Builder: set pre-apply normalization.
    pub fn with_pre_apply(mut self, name: Name) -> Self {
        self.pre_apply = Some(name);
        self
    }
    /// Builder: set backtrack_all flag.
    pub fn with_backtrack_all(mut self, v: bool) -> Self {
        self.backtrack_all = v;
        self
    }
}
/// Result of the `solve_by_elim` search.
#[derive(Clone, Debug)]
pub enum SolveByElimResult {
    /// All targeted goals were solved; contains the proof term for the
    /// original goal.
    Solved(Expr),
    /// Search got stuck with these goals remaining.
    Stuck(Vec<MVarId>),
    /// The depth limit was exceeded during search.
    DepthExceeded,
}
impl SolveByElimResult {
    /// Check if the search succeeded.
    pub fn is_solved(&self) -> bool {
        matches!(self, SolveByElimResult::Solved(_))
    }
    /// Check if the search got stuck.
    pub fn is_stuck(&self) -> bool {
        matches!(self, SolveByElimResult::Stuck(_))
    }
    /// Check if depth was exceeded.
    pub fn is_depth_exceeded(&self) -> bool {
        matches!(self, SolveByElimResult::DepthExceeded)
    }
}
/// Where a candidate lemma came from.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CandidateSource {
    /// A hypothesis from the local context.
    LocalHyp(Name),
    /// A lemma explicitly provided by the user.
    ProvidedLemma(usize),
    /// A declaration from the global environment.
    EnvironmentDecl(Name),
}
/// Builder for constructing a candidate set from various sources.
#[allow(dead_code)]
pub struct CandidateSetBuilder {
    /// Accumulated candidates.
    pub(super) candidates: Vec<Candidate>,
}
#[allow(dead_code)]
impl CandidateSetBuilder {
    /// Create a new empty builder.
    pub fn new() -> Self {
        Self {
            candidates: Vec::new(),
        }
    }
    /// Add hypotheses from the local context.
    pub fn add_local_hyps(&mut self, ctx: &MetaContext) -> &mut Self {
        let hyps = ctx.get_local_hyps();
        for (name, ty) in hyps.iter().rev() {
            let ty_inst = ctx.instantiate_mvars(ty);
            let arity = estimate_arity(&ty_inst);
            self.candidates.push(Candidate {
                expr: Expr::Const(name.clone(), vec![]),
                source: CandidateSource::LocalHyp(name.clone()),
                estimated_subgoals: arity,
                result_type: Some(ty_inst),
            });
        }
        self
    }
    /// Add explicitly provided lemmas.
    pub fn add_lemmas(&mut self, lemmas: &[Expr]) -> &mut Self {
        for (i, lemma) in lemmas.iter().enumerate() {
            let arity = estimate_arity(lemma);
            self.candidates.push(Candidate {
                expr: lemma.clone(),
                source: CandidateSource::ProvidedLemma(i),
                estimated_subgoals: arity,
                result_type: None,
            });
        }
        self
    }
    /// Add a single named lemma from the environment.
    pub fn add_env_lemma(&mut self, name: Name, ty: Expr) -> &mut Self {
        let arity = estimate_arity(&ty);
        self.candidates.push(Candidate {
            expr: Expr::Const(name.clone(), vec![]),
            source: CandidateSource::EnvironmentDecl(name),
            estimated_subgoals: arity,
            result_type: Some(ty),
        });
        self
    }
    /// Sort candidates by estimated subgoals (ascending).
    pub fn sort_by_arity(&mut self) -> &mut Self {
        self.candidates.sort_by_key(|c| c.estimated_subgoals);
        self
    }
    /// Build the final candidate list.
    pub fn build(self) -> Vec<Candidate> {
        self.candidates
    }
    /// Get current number of candidates.
    pub fn len(&self) -> usize {
        self.candidates.len()
    }
    /// Check if the builder has no candidates.
    pub fn is_empty(&self) -> bool {
        self.candidates.is_empty()
    }
}
/// Internal failure modes during search.
#[derive(Clone, Debug)]
pub(super) enum SearchFailure {
    /// Depth limit exceeded.
    DepthExceeded,
    /// No goals to solve.
    NoGoals,
    /// No candidate lemmas available.
    NoCandidates,
    /// All candidates were tried and none worked.
    AllCandidatesExhausted,
    /// Backtrack budget exhausted.
    BacktrackLimitReached,
}
