//! Bridge between oxilean-elab's TacticState and oxilean-meta's MetaContext/TacticState.
//!
//! This allows the elaborator's tactic evaluator to call meta-layer tactic
//! implementations which use a richer, MetaContext-aware state representation.
//!
//! # Design
//!
//! The elab layer represents proof state as a flat list of `Goal` structs, each
//! carrying its own hypothesis list and target expression. The meta layer instead
//! stores goals as bare `MVarId`s into a shared `MetaContext` that owns all type
//! information and assignments.
//!
//! `MetaBridge` converts between these two representations:
//! - **`from_elab_state`**: Allocates fresh MVarIds in a new MetaContext to mirror
//!   each elab `Goal`, registering its hypotheses as local declarations.
//! - **`to_elab_state`**: Inspects which MVarIds are now assigned (solved) and
//!   rebuilds the elab `TacticState` with the correct goal/solved split.
//!
//! `try_meta_tactic` is the main entry point: build the bridge, run a meta-tactic
//! closure, tear it down.

use oxilean_kernel::{BinderInfo, Environment, Name};
use oxilean_meta::tactic::TacticError as MetaTacticError;
use oxilean_meta::tactic::TacticState as MetaTacticState;
use oxilean_meta::{MVarId, MetaContext, MetavarKind};

use crate::tactic::types::{Goal, TacticError, TacticState};

// ─── Core bridge struct ───────────────────────────────────────────────────────

/// A two-way bridge connecting the elab `TacticState` to a meta-layer
/// `MetaContext` + `MetaTacticState`.
///
/// Invariant: `goal_map[i].1` is the `MVarId` in `meta_ctx` that corresponds to
/// `elab_state.goals()[i]`. The MVarId is unassigned at construction; a meta tactic
/// marks it assigned when it solves the goal.
pub struct MetaBridge {
    /// The meta context that owns all metavariable declarations and assignments.
    pub meta_ctx: MetaContext,
    /// The meta-layer tactic state (a list of `MVarId`s).
    pub meta_state: MetaTacticState,
    /// Maps each elab `Goal::name` to the corresponding `MVarId` in `meta_ctx`,
    /// in the same order as `elab_state.goals()`.
    pub goal_map: Vec<(Name, MVarId)>,
}

// ─── Construction ─────────────────────────────────────────────────────────────

impl MetaBridge {
    /// Build a `MetaBridge` from an elab `TacticState`.
    ///
    /// For each `Goal` in `elab_state`:
    /// 1. Register every hypothesis as a local declaration in a fresh `MetaContext`.
    /// 2. Create a fresh metavariable whose type is the goal target.
    /// 3. Record the `(goal.name, mvar_id)` mapping.
    ///
    /// Returns `Err(TacticError::InternalError)` if the meta context cannot be
    /// populated (e.g., type construction fails).
    pub fn from_elab_state(
        elab_state: &TacticState,
        env: &Environment,
    ) -> Result<Self, TacticError> {
        let mut meta_ctx = MetaContext::new(env.clone());
        let mut mvar_ids: Vec<MVarId> = Vec::with_capacity(elab_state.goals().len());
        let mut goal_map: Vec<(Name, MVarId)> = Vec::with_capacity(elab_state.goals().len());

        for goal in elab_state.goals() {
            // Register each hypothesis as a local declaration so the meta context
            // has the correct local context when creating the goal metavariable.
            for (hyp_name, hyp_ty) in &goal.hypotheses {
                // Let-bound entries in local_ctx have a value; plain hypotheses don't.
                // We look for a matching let-bound entry first.
                let let_val =
                    goal.local_ctx.iter().find_map(
                        |(n, _ty, val)| {
                            if n == hyp_name {
                                val.as_ref()
                            } else {
                                None
                            }
                        },
                    );

                if let Some(val) = let_val {
                    meta_ctx.mk_let_decl(hyp_name.clone(), hyp_ty.clone(), val.clone());
                } else {
                    meta_ctx.mk_local_decl(hyp_name.clone(), hyp_ty.clone(), BinderInfo::Default);
                }
            }

            // Create a fresh metavariable for the goal target.
            let (mvar_id, _placeholder) =
                meta_ctx.mk_fresh_expr_mvar(goal.target.clone(), MetavarKind::Natural);

            mvar_ids.push(mvar_id);
            goal_map.push((goal.name.clone(), mvar_id));
        }

        let meta_state = MetaTacticState::new(mvar_ids);

        Ok(Self {
            meta_ctx,
            meta_state,
            goal_map,
        })
    }

    /// Reconstruct an elab `TacticState` from the bridge state after a meta
    /// tactic has run.
    ///
    /// Any MVarId that is now assigned in `meta_ctx` is treated as solved.
    /// Goals whose MVarId remains unassigned are kept in the returned state's
    /// goal list (using the original goal data from `original`).
    ///
    /// `original` must be the same elab state that was passed to
    /// [`Self::from_elab_state`]; in particular `original.goals()` must be in the
    /// same order as `self.goal_map`.
    pub fn to_elab_state(&self, original: &TacticState) -> TacticState {
        let mut result = TacticState::new();

        // Copy already-solved goals from the original state.
        result.solved = original.solved.clone();

        // Rebuild the goal list, routing each goal to either "remaining" or "solved".
        let original_goals = original.goals();
        for (i, (goal_name, mvar_id)) in self.goal_map.iter().enumerate() {
            if self.meta_ctx.is_mvar_assigned(*mvar_id) {
                // The meta tactic solved this goal.
                result.solved.push(goal_name.clone());
            } else {
                // Still open — keep the original goal.
                if let Some(goal) = original_goals.get(i) {
                    result.add_goal(goal.clone());
                }
            }
        }

        result
    }
}

// ─── Error conversion ─────────────────────────────────────────────────────────

/// Map a meta-layer `TacticError` to an elab-layer `TacticError`.
///
/// The two error hierarchies overlap in intent but differ in structure:
///
/// | Meta variant                          | Elab variant               |
/// |---------------------------------------|----------------------------|
/// | `NoGoals`                             | `NoGoals`                  |
/// | `Failed(msg)`                         | `InternalError(msg)`       |
/// | `TypeMismatch { expected, got }`      | `TypeMismatch(formatted)`  |
/// | `UnknownHyp(name)`                   | `GoalNotFound(name)`       |
/// | `GoalMismatch(msg)`                   | `InternalError(msg)`       |
/// | `Internal(msg)`                       | `InternalError(msg)`       |
fn map_meta_error(err: MetaTacticError) -> TacticError {
    match err {
        MetaTacticError::NoGoals => TacticError::NoGoals,
        MetaTacticError::Failed(msg) => TacticError::InternalError(msg),
        MetaTacticError::TypeMismatch { expected, got } => TacticError::TypeMismatch(format!(
            "type mismatch: expected `{expected:?}`, got `{got:?}`"
        )),
        MetaTacticError::UnknownHyp(name) => TacticError::GoalNotFound(name),
        MetaTacticError::GoalMismatch(msg) => TacticError::InternalError(msg),
        MetaTacticError::Internal(msg) => TacticError::InternalError(msg),
    }
}

// ─── Public helper ────────────────────────────────────────────────────────────

/// Run a meta-layer tactic closure against an elab `TacticState`.
///
/// # Type parameters
///
/// - `F` — closure that receives a mutable `MetaBridge` and returns
///   `Result<R, MetaTacticError>`.
/// - `R` — the success value produced by the closure (ignored; only the
///   state transformation matters).
///
/// # Steps
///
/// 1. Construct a `MetaBridge` from `state` and `env`.
/// 2. Invoke `f(&mut bridge)`.
/// 3. On success, call [`MetaBridge::to_elab_state`] to rebuild the elab state.
/// 4. On failure, map the meta error to an elab error and propagate.
///
/// # Example
///
/// ```rust,ignore
/// use oxilean_elab::meta_bridge::try_meta_tactic;
///
/// let new_state = try_meta_tactic(&state, &env, |bridge| {
///     oxilean_meta::tactic::tac_intro(&mut bridge.meta_ctx, &mut bridge.meta_state, my_name)
/// })?;
/// ```
pub fn try_meta_tactic<F, R>(
    state: &TacticState,
    env: &Environment,
    f: F,
) -> Result<TacticState, TacticError>
where
    F: FnOnce(&mut MetaBridge) -> Result<R, MetaTacticError>,
{
    let mut bridge = MetaBridge::from_elab_state(state, env)?;

    f(&mut bridge).map_err(map_meta_error)?;

    Ok(bridge.to_elab_state(state))
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use oxilean_kernel::{Environment, Expr, Level, Name};

    fn mk_env() -> Environment {
        Environment::new()
    }

    fn prop_expr() -> Expr {
        // A simple Prop-level sort: `Sort 0`
        Expr::Sort(Level::Zero)
    }

    fn make_goal(name: &str, target: Expr) -> Goal {
        Goal::new(Name::str(name), target)
    }

    #[test]
    fn test_from_elab_empty_state() {
        let env = mk_env();
        let state = TacticState::new();
        let bridge = MetaBridge::from_elab_state(&state, &env)
            .expect("bridge construction should succeed for empty state");
        assert!(bridge.goal_map.is_empty());
        assert!(bridge.meta_state.is_done());
    }

    #[test]
    fn test_from_elab_single_goal() {
        let env = mk_env();
        let mut state = TacticState::new();
        state.add_goal(make_goal("main", prop_expr()));

        let bridge =
            MetaBridge::from_elab_state(&state, &env).expect("bridge construction should succeed");

        assert_eq!(bridge.goal_map.len(), 1);
        assert_eq!(bridge.goal_map[0].0, Name::str("main"));
        assert_eq!(bridge.meta_state.num_goals(), 1);
    }

    #[test]
    fn test_to_elab_state_no_progress() {
        let env = mk_env();
        let mut state = TacticState::new();
        state.add_goal(make_goal("g1", prop_expr()));
        state.add_goal(make_goal("g2", prop_expr()));

        let bridge =
            MetaBridge::from_elab_state(&state, &env).expect("bridge construction should succeed");

        // Nothing was solved — the rebuilt state should still have 2 goals.
        let result = bridge.to_elab_state(&state);
        assert_eq!(result.goals().len(), 2);
        assert!(result.solved.is_empty());
    }

    #[test]
    fn test_to_elab_state_after_solve() {
        let env = mk_env();
        let mut state = TacticState::new();
        state.add_goal(make_goal("g1", prop_expr()));
        state.add_goal(make_goal("g2", prop_expr()));

        let mut bridge =
            MetaBridge::from_elab_state(&state, &env).expect("bridge construction should succeed");

        // Simulate solving the first goal by assigning its metavariable.
        let mvar_id = bridge.goal_map[0].1;
        bridge.meta_ctx.assign_mvar(mvar_id, prop_expr());

        let result = bridge.to_elab_state(&state);
        // g1 is solved, g2 remains.
        assert_eq!(result.goals().len(), 1);
        assert_eq!(result.goals()[0].name, Name::str("g2"));
        assert_eq!(result.solved, vec![Name::str("g1")]);
    }

    #[test]
    fn test_goal_with_hypothesis() {
        let env = mk_env();
        let mut state = TacticState::new();

        let mut goal = make_goal("hyp_goal", prop_expr());
        goal.add_hypothesis(Name::str("h"), prop_expr());
        state.add_goal(goal);

        let bridge = MetaBridge::from_elab_state(&state, &env)
            .expect("bridge construction with hypothesis should succeed");

        assert_eq!(bridge.goal_map.len(), 1);
        // The local context in meta_ctx should have the hypothesis registered.
        assert!(!bridge.meta_ctx.get_local_hyps().is_empty());
    }

    #[test]
    fn test_try_meta_tactic_identity() {
        let env = mk_env();
        let mut state = TacticState::new();
        state.add_goal(make_goal("g", prop_expr()));

        // A no-op closure — doesn't solve anything.
        let result = try_meta_tactic(&state, &env, |_bridge| Ok::<(), MetaTacticError>(()))
            .expect("try_meta_tactic should succeed");

        assert_eq!(result.goals().len(), 1);
    }

    #[test]
    fn test_try_meta_tactic_error_propagation() {
        let env = mk_env();
        let mut state = TacticState::new();
        state.add_goal(make_goal("g", prop_expr()));

        let result = try_meta_tactic(&state, &env, |_bridge| {
            Err::<(), MetaTacticError>(MetaTacticError::NoGoals)
        });

        assert!(result.is_err());
        assert!(matches!(result, Err(TacticError::NoGoals)));
    }
}
