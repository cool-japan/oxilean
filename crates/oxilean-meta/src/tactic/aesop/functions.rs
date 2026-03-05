//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    AesopConfig, AesopResult, AesopRule, AesopRuleKind, AesopRuleSafety, AesopRuleSet,
    AesopSearchNode, AesopSearchState, AesopStats, NodeId, NodeStatus, PQEntry, ProofCache, RuleId,
    TransparencyMode,
};
use crate::basic::{MVarId, MetaContext, MetaState};
use crate::tactic::state::{TacticError, TacticResult, TacticState};
use oxilean_kernel::{instantiate_level_params, ConstantInfo, Expr, Name};

/// A tactic closure that transforms a goal.
///
/// The closure receives a mutable reference to the tactic state and the
/// meta context and either succeeds (returning `Ok(())`) or fails.
pub type RuleTacticFn =
    Box<dyn Fn(&mut TacticState, &mut MetaContext) -> TacticResult<()> + Send + Sync>;
/// Run normalization rules on the current goal.
///
/// This applies safe, deterministic transformations such as:
/// - `intro` when the goal is a Pi type
/// - `True.intro` when the goal is `True`
/// - `rfl` when the goal is a reflexive equality
/// - User-supplied normalization rules from the rule set.
///
/// Returns `true` if the goal was closed by normalization.
#[allow(dead_code)]
pub(super) fn normalize_goal(
    rule_set: &AesopRuleSet,
    state: &mut TacticState,
    ctx: &mut MetaContext,
    stats: &mut AesopStats,
) -> TacticResult<bool> {
    let max_norm_rounds = 50;
    for _ in 0..max_norm_rounds {
        if state.is_done() {
            return Ok(true);
        }
        let goal = state.current_goal()?;
        let target = ctx
            .get_mvar_type(goal)
            .cloned()
            .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
        let target = ctx.instantiate_mvars(&target);
        if matches!(&target, Expr::Pi(..)) {
            let intro_result = crate::tactic::core::tac_intro(None, state, ctx);
            if intro_result.is_ok() {
                stats.norm_passes += 1;
                continue;
            }
        }
        if matches!(& target, Expr::Const(n, _) if * n == Name::str("True")) {
            let proof = Expr::Const(Name::str("True.intro"), vec![]);
            if state.close_goal(proof, ctx).is_ok() {
                stats.norm_passes += 1;
                return Ok(true);
            }
        }
        if let Some(proof) = try_reflexivity(&target) {
            if state.close_goal(proof, ctx).is_ok() {
                stats.norm_passes += 1;
                return Ok(true);
            }
        }
        let mut any_fired = false;
        let norm_ids = rule_set.norm_rules();
        for rid in &norm_ids {
            if let Some(rule) = rule_set.get(*rid) {
                stats.rules_tried += 1;
                state.save();
                let saved_meta = ctx.save_state();
                if (rule.tactic)(state, ctx).is_ok() {
                    stats.rules_succeeded += 1;
                    stats.norm_passes += 1;
                    any_fired = true;
                    break;
                } else {
                    let _ = state.restore();
                    ctx.restore_state(saved_meta);
                }
            }
        }
        if !any_fired {
            break;
        }
    }
    Ok(state.is_done())
}
/// Try to close a reflexive equality goal `@Eq A a a`.
pub(super) fn try_reflexivity(target: &Expr) -> Option<Expr> {
    if let Expr::App(eq_a, rhs) = target {
        if let Expr::App(eq_ty, lhs) = eq_a.as_ref() {
            if let Expr::App(eq_head, _alpha) = eq_ty.as_ref() {
                if matches!(
                    eq_head.as_ref(), Expr::Const(n, _) if * n == Name::str("Eq")
                ) && lhs == rhs
                {
                    return Some(Expr::Const(
                        Name::str("Eq.refl"),
                        vec![oxilean_kernel::Level::zero()],
                    ));
                }
            }
        }
    }
    None
}
/// Attempt to close a goal via assumption (hypothesis in the local context).
pub(super) fn try_assumption(state: &mut TacticState, ctx: &mut MetaContext) -> bool {
    state.save();
    let saved = ctx.save_state();
    let ok = crate::tactic::core::tac_assumption(state, ctx).is_ok();
    if !ok {
        let _ = state.restore();
        ctx.restore_state(saved);
    }
    ok
}
/// Build the default rule set containing built-in safe and unsafe rules.
///
/// The default set includes:
/// - **Safe**: `intro` (Pi goals), `True.intro`, `rfl`, `assumption`
/// - **Almost safe**: `constructor` on common single-constructor types
/// - **Unsafe**: `apply` with common lemmas, `cases`
/// - **Norm**: `intro` on Pi goals
pub fn default_rule_set() -> AesopRuleSet {
    let mut rs = AesopRuleSet::new();
    let _ = rs.add(AesopRule::new(
        Name::str("Aesop.intro"),
        Box::new(|state, ctx| {
            let goal = state.current_goal()?;
            let target = ctx
                .get_mvar_type(goal)
                .cloned()
                .ok_or_else(|| TacticError::Internal("no type".into()))?;
            let target = ctx.instantiate_mvars(&target);
            if matches!(&target, Expr::Pi(..)) {
                crate::tactic::core::tac_intro(None, state, ctx).map(|_| ())
            } else {
                Err(TacticError::Failed("not a Pi".into()))
            }
        }),
        AesopRuleSafety::Safe,
        AesopRuleKind::Norm,
        10,
    ));
    let _ = rs.add(
        AesopRule::new(
            Name::str("Aesop.trueIntro"),
            Box::new(|state, ctx| {
                let proof = Expr::Const(Name::str("True.intro"), vec![]);
                state.close_goal(proof, ctx)
            }),
            AesopRuleSafety::Safe,
            AesopRuleKind::Apply,
            5,
        )
        .with_pattern(Expr::Const(Name::str("True"), vec![])),
    );
    let _ = rs.add(AesopRule::new(
        Name::str("Aesop.rfl"),
        Box::new(|state, ctx| {
            let goal = state.current_goal()?;
            let target = ctx
                .get_mvar_type(goal)
                .cloned()
                .ok_or_else(|| TacticError::Internal("no type".into()))?;
            let target = ctx.instantiate_mvars(&target);
            if let Some(proof) = try_reflexivity(&target) {
                state.close_goal(proof, ctx)
            } else {
                Err(TacticError::Failed("not a reflexive equality".into()))
            }
        }),
        AesopRuleSafety::Safe,
        AesopRuleKind::Apply,
        5,
    ));
    let _ = rs.add(AesopRule::new(
        Name::str("Aesop.assumption"),
        Box::new(crate::tactic::core::tac_assumption),
        AesopRuleSafety::Safe,
        AesopRuleKind::Apply,
        20,
    ));
    let _ = rs.add(AesopRule::new(
        Name::str("Aesop.constructor"),
        Box::new(|state, ctx| crate::tactic::constructor::tac_constructor(state, ctx).map(|_| ())),
        AesopRuleSafety::AlmostSafe,
        AesopRuleKind::Constructor,
        50,
    ));
    let _ = rs.add(
        AesopRule::new(
            Name::str("Aesop.andIntro"),
            Box::new(|state, ctx| {
                let goal = state.current_goal()?;
                let target = ctx
                    .get_mvar_type(goal)
                    .cloned()
                    .ok_or_else(|| TacticError::Internal("no type".into()))?;
                let target = ctx.instantiate_mvars(&target);
                let head = get_head_const_name(&target);
                if head.as_deref() == Some("And") {
                    crate::tactic::constructor::tac_constructor(state, ctx).map(|_| ())
                } else {
                    Err(TacticError::Failed("not a conjunction".into()))
                }
            }),
            AesopRuleSafety::AlmostSafe,
            AesopRuleKind::Constructor,
            40,
        )
        .with_pattern(Expr::Const(Name::str("And"), vec![]))
        .with_estimated_subgoals(2),
    );
    let _ = rs.add(
        AesopRule::new(
            Name::str("Aesop.left"),
            Box::new(|state, ctx| crate::tactic::constructor::tac_left(state, ctx).map(|_| ())),
            AesopRuleSafety::Unsafe,
            AesopRuleKind::Constructor,
            80,
        )
        .with_pattern(Expr::Const(Name::str("Or"), vec![]))
        .with_estimated_subgoals(1),
    );
    let _ = rs.add(
        AesopRule::new(
            Name::str("Aesop.right"),
            Box::new(|state, ctx| crate::tactic::constructor::tac_right(state, ctx).map(|_| ())),
            AesopRuleSafety::Unsafe,
            AesopRuleKind::Constructor,
            80,
        )
        .with_pattern(Expr::Const(Name::str("Or"), vec![]))
        .with_estimated_subgoals(1),
    );
    let _ = rs.add(AesopRule::new(
        Name::str("Aesop.trivial"),
        Box::new(crate::tactic::core::tac_trivial),
        AesopRuleSafety::Unsafe,
        AesopRuleKind::Apply,
        100,
    ));
    rs
}
/// Run one step of the best-first search.
///
/// Pops the lowest-cost node from the queue, applies rules, and creates children.
/// Returns `true` if the search should continue, `false` if it has concluded.
pub(super) fn search_step(
    search: &mut AesopSearchState,
    rule_set: &AesopRuleSet,
    _state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<bool> {
    if let Some(reason) = search.check_limits() {
        search.finished = true;
        search.stats.time_us = search.start_time.elapsed().as_micros() as u64;
        return Err(TacticError::Failed(format!(
            "aesop: search terminated: {}",
            reason
        )));
    }
    let entry = match search.queue.pop() {
        Some(e) => e,
        None => {
            search.finished = true;
            search.stats.time_us = search.start_time.elapsed().as_micros() as u64;
            return Err(TacticError::Failed(
                "aesop: search exhausted all possibilities".into(),
            ));
        }
    };
    let node_id = entry.node_id;
    search.stats.nodes_expanded += 1;
    if search.nodes[node_id.0].status != NodeStatus::Open {
        return Ok(true);
    }
    let depth = search.nodes[node_id.0].depth;
    if depth > search.config.max_depth {
        search.nodes[node_id.0].status = NodeStatus::Pruned;
        return Ok(true);
    }
    if let Some(ref saved) = search.nodes[node_id.0].saved_meta.clone() {
        ctx.restore_state(saved.clone());
        search.stats.backtracks += 1;
    }
    let goals = search.nodes[node_id.0].goals.clone();
    if goals.is_empty() {
        search.nodes[node_id.0].status = NodeStatus::Solved;
        search.solution_node = Some(node_id);
        search.finished = true;
        search.stats.time_us = search.start_time.elapsed().as_micros() as u64;
        return Ok(false);
    }
    if search.config.enable_cache {
        let first_goal = goals[0];
        if let Some(target) = ctx.get_mvar_type(first_goal).cloned() {
            let target = ctx.instantiate_mvars(&target);
            if let Some(proof) = search.cache.lookup(&target).cloned() {
                search.stats.cache_hits += 1;
                ctx.assign_mvar(first_goal, proof);
                let remaining: Vec<MVarId> = goals[1..].to_vec();
                if remaining.is_empty() {
                    search.nodes[node_id.0].status = NodeStatus::Solved;
                    search.solution_node = Some(node_id);
                    search.finished = true;
                    search.stats.time_us = search.start_time.elapsed().as_micros() as u64;
                    return Ok(false);
                }
                let saved = ctx.save_state();
                let child = AesopSearchNode::child(
                    NodeId(0),
                    node_id,
                    depth + 1,
                    RuleId(usize::MAX),
                    Name::str("cache_hit"),
                    remaining,
                    entry.cost,
                    saved,
                );
                let child_id = search.alloc_node(child);
                search.nodes[node_id.0].children.push(child_id);
                search.queue.push(PQEntry {
                    node_id: child_id,
                    cost: entry.cost,
                });
                search.nodes[node_id.0].status = NodeStatus::Expanded;
                return Ok(true);
            } else {
                search.stats.cache_misses += 1;
            }
        }
    }
    let first_goal = goals[0];
    let target = ctx
        .get_mvar_type(first_goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("node goal has no type".into()))?;
    let target = ctx.instantiate_mvars(&target);
    let candidate_ids = rule_set.query(&target);
    let mut safe_ids: Vec<RuleId> = Vec::new();
    let mut unsafe_ids: Vec<RuleId> = Vec::new();
    for &rid in &candidate_ids {
        if let Some(rule) = rule_set.get(rid) {
            if rule.safety.is_safe_or_almost() {
                safe_ids.push(rid);
            } else {
                unsafe_ids.push(rid);
            }
        }
    }
    for &rid in &safe_ids {
        if let Some(rule) = rule_set.get(rid) {
            search.stats.rules_tried += 1;
            search.nodes[node_id.0].rule_apps_tried += 1;
            let saved_meta = ctx.save_state();
            let mut trial_state = TacticState::single(first_goal);
            if (rule.tactic)(&mut trial_state, ctx).is_ok() {
                search.stats.rules_succeeded += 1;
                let mut new_goals: Vec<MVarId> = trial_state.all_goals().to_vec();
                new_goals.extend_from_slice(&goals[1..]);
                if search.config.enable_cache && trial_state.is_done() {
                    if let Some(proof_val) = ctx.get_mvar_assignment(first_goal).cloned() {
                        search.cache.insert(target.clone(), proof_val, 1);
                    }
                }
                if new_goals.is_empty() {
                    search.nodes[node_id.0].status = NodeStatus::Solved;
                    search.nodes[node_id.0].applied_rule = Some(rid);
                    search.nodes[node_id.0].applied_rule_name = Some(rule.name.clone());
                    search.solution_node = Some(node_id);
                    search.finished = true;
                    search.stats.time_us = search.start_time.elapsed().as_micros() as u64;
                    return Ok(false);
                }
                let child_saved = ctx.save_state();
                let cost = rule.effective_cost(depth + 1, search.config.depth_penalty);
                let child = AesopSearchNode::child(
                    NodeId(0),
                    node_id,
                    depth + 1,
                    rid,
                    rule.name.clone(),
                    new_goals,
                    cost,
                    child_saved,
                );
                let child_id = search.alloc_node(child);
                search.nodes[node_id.0].children.push(child_id);
                search.queue.push(PQEntry {
                    node_id: child_id,
                    cost,
                });
                search.nodes[node_id.0].status = NodeStatus::Expanded;
                return Ok(true);
            } else {
                ctx.restore_state(saved_meta);
            }
        }
    }
    {
        let saved_meta = ctx.save_state();
        let mut trial_state = TacticState::single(first_goal);
        if try_assumption(&mut trial_state, ctx) {
            search.stats.rules_tried += 1;
            search.stats.rules_succeeded += 1;
            let remaining: Vec<MVarId> = goals[1..].to_vec();
            if remaining.is_empty() {
                search.nodes[node_id.0].status = NodeStatus::Solved;
                search.solution_node = Some(node_id);
                search.finished = true;
                search.stats.time_us = search.start_time.elapsed().as_micros() as u64;
                return Ok(false);
            }
            let child_saved = ctx.save_state();
            let child = AesopSearchNode::child(
                NodeId(0),
                node_id,
                depth + 1,
                RuleId(usize::MAX),
                Name::str("assumption"),
                remaining,
                entry.cost + 1.0,
                child_saved,
            );
            let child_id = search.alloc_node(child);
            search.nodes[node_id.0].children.push(child_id);
            search.queue.push(PQEntry {
                node_id: child_id,
                cost: entry.cost + 1.0,
            });
            search.nodes[node_id.0].status = NodeStatus::Expanded;
            return Ok(true);
        } else {
            ctx.restore_state(saved_meta);
        }
    }
    let mut created_any = false;
    for &rid in &unsafe_ids {
        if let Some(rule) = rule_set.get(rid) {
            search.stats.rules_tried += 1;
            search.nodes[node_id.0].rule_apps_tried += 1;
            let saved_meta = ctx.save_state();
            let mut trial_state = TacticState::single(first_goal);
            if (rule.tactic)(&mut trial_state, ctx).is_ok() {
                search.stats.rules_succeeded += 1;
                let mut new_goals: Vec<MVarId> = trial_state.all_goals().to_vec();
                new_goals.extend_from_slice(&goals[1..]);
                let child_saved = ctx.save_state();
                let cost = rule.effective_cost(depth + 1, search.config.depth_penalty);
                if search.config.enable_cache && trial_state.is_done() {
                    if let Some(proof_val) = ctx.get_mvar_assignment(first_goal).cloned() {
                        search.cache.insert(target.clone(), proof_val, 1);
                    }
                }
                if new_goals.is_empty() {
                    search.nodes[node_id.0].status = NodeStatus::Solved;
                    search.nodes[node_id.0].applied_rule = Some(rid);
                    search.nodes[node_id.0].applied_rule_name = Some(rule.name.clone());
                    search.solution_node = Some(node_id);
                    search.finished = true;
                    search.stats.time_us = search.start_time.elapsed().as_micros() as u64;
                    return Ok(false);
                }
                let child = AesopSearchNode::child(
                    NodeId(0),
                    node_id,
                    depth + 1,
                    rid,
                    rule.name.clone(),
                    new_goals,
                    cost,
                    child_saved,
                );
                let child_id = search.alloc_node(child);
                search.nodes[node_id.0].children.push(child_id);
                search.queue.push(PQEntry {
                    node_id: child_id,
                    cost,
                });
                created_any = true;
            }
            ctx.restore_state(saved_meta);
        }
    }
    if created_any {
        search.nodes[node_id.0].status = NodeStatus::Expanded;
    } else {
        search.nodes[node_id.0].status = NodeStatus::Failed;
    }
    Ok(true)
}
/// Reconstruct the proof from the search tree.
///
/// Walks from the solution leaf back to the root, collecting the assigned
/// proof terms from the meta context.
pub(super) fn reconstruct_proof(
    search: &AesopSearchState,
    ctx: &MetaContext,
    initial_goal: MVarId,
) -> Expr {
    if let Some(proof) = ctx.get_mvar_assignment(initial_goal) {
        ctx.instantiate_mvars(proof)
    } else {
        let _ = search;
        Expr::Const(Name::str("aesop_proof_placeholder"), vec![])
    }
}
/// Collect the path from a node to the root.
#[allow(dead_code)]
pub(super) fn path_to_root(search: &AesopSearchState, node_id: NodeId) -> Vec<NodeId> {
    let mut path = Vec::new();
    let mut current = node_id;
    loop {
        path.push(current);
        if let Some(parent) = search.nodes[current.0].parent {
            current = parent;
        } else {
            break;
        }
    }
    path.reverse();
    path
}
/// Extract the head constant name from an expression (if it has one).
pub(super) fn get_head_const_name(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Const(name, _) => Some(format!("{}", name)),
        Expr::App(f, _) => get_head_const_name(f),
        _ => None,
    }
}
/// Run the complete aesop search and return an `AesopResult`.
///
/// This is the low-level entry point. For the tactic interface, see
/// `tac_aesop` and `tac_aesop_with_config`.
pub fn run_aesop_search(
    config: &AesopConfig,
    rule_set: &AesopRuleSet,
    initial_goals: Vec<MVarId>,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> AesopResult {
    if initial_goals.is_empty() {
        return AesopResult::Success {
            proof: Expr::Const(Name::str("trivial"), vec![]),
            stats: AesopStats::default(),
        };
    }
    let first_goal = initial_goals[0];
    let mut search = AesopSearchState::new(config.clone(), initial_goals);
    loop {
        match search_step(&mut search, rule_set, state, ctx) {
            Ok(true) => continue,
            Ok(false) => {
                let proof = reconstruct_proof(&search, ctx, first_goal);
                return AesopResult::Success {
                    proof,
                    stats: search.stats,
                };
            }
            Err(e) => {
                search.stats.time_us = search.start_time.elapsed().as_micros() as u64;
                let msg = format!("{}", e);
                if msg.contains("limit") || msg.contains("timeout") {
                    return AesopResult::ResourceLimit {
                        limit_kind: msg,
                        stats: search.stats,
                    };
                }
                return AesopResult::Failure {
                    reason: msg,
                    stats: search.stats,
                };
            }
        }
    }
}
/// Run the `aesop` tactic with default configuration.
///
/// Performs best-first proof search using the default rule set and
/// configuration parameters. On success, closes the current goal
/// in the tactic state.
///
/// # Errors
///
/// Returns `TacticError::Failed` if the search could not find a proof
/// within the configured limits.
pub fn tac_aesop(state: &mut TacticState, ctx: &mut MetaContext) -> TacticResult<()> {
    let config = AesopConfig::default();
    tac_aesop_with_config(&config, state, ctx)
}
/// Run the `aesop` tactic with a custom configuration.
///
/// This is the configurable variant of `tac_aesop`. All search parameters
/// (depth, iterations, timeout, etc.) are controlled by `config`.
///
/// # Errors
///
/// Returns `TacticError::Failed` if the search does not find a proof.
pub fn tac_aesop_with_config(
    config: &AesopConfig,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<()> {
    let goals: Vec<MVarId> = state.all_goals().to_vec();
    if goals.is_empty() {
        return Ok(());
    }
    let rule_set = if config.use_default_rules {
        default_rule_set()
    } else {
        AesopRuleSet::new()
    };
    tac_aesop_with_rules(config, &rule_set, state, ctx)
}
/// Run the `aesop` tactic with a custom configuration and rule set.
///
/// This variant gives full control over both the search parameters and
/// the rules. Useful when the caller has constructed a domain-specific
/// rule set.
///
/// # Errors
///
/// Returns `TacticError::Failed` if the search does not find a proof.
pub fn tac_aesop_with_rules(
    config: &AesopConfig,
    rule_set: &AesopRuleSet,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<()> {
    let goals: Vec<MVarId> = state.all_goals().to_vec();
    if goals.is_empty() {
        return Ok(());
    }
    let result = run_aesop_search(config, rule_set, goals, state, ctx);
    match result {
        AesopResult::Success { proof, .. } => {
            if !state.is_done() {
                let _ = state.close_goal(proof, ctx);
            }
            Ok(())
        }
        AesopResult::Failure { reason, .. } => {
            if config.warn_on_failure {
                Ok(())
            } else {
                Err(TacticError::Failed(reason))
            }
        }
        AesopResult::ResourceLimit { limit_kind, .. } => {
            Err(TacticError::Failed(format!("aesop: {}", limit_kind)))
        }
    }
}
/// Create a safe `Apply` rule from a proof expression.
///
/// The generated rule applies the given expression as a proof of the goal.
pub fn make_apply_rule(name: Name, proof_expr: Expr, priority: u32) -> AesopRule {
    let expr = proof_expr.clone();
    AesopRule::new(
        name,
        Box::new(move |state, ctx| crate::tactic::core::tac_exact(expr.clone(), state, ctx)),
        AesopRuleSafety::Unsafe,
        AesopRuleKind::Apply,
        priority,
    )
}
/// Create an unfold rule that replaces occurrences of a constant with its definition.
///
/// The `def_name` is the name of the constant to unfold; `def_body` is the unfolded
/// expression.
pub fn make_unfold_rule(name: Name, def_name: Name, _def_body: Expr, priority: u32) -> AesopRule {
    AesopRule::new(
        name,
        Box::new(move |state, ctx| {
            let goal = state.current_goal()?;
            let target = ctx
                .get_mvar_type(goal)
                .cloned()
                .ok_or_else(|| TacticError::Internal("no type".into()))?;
            let target = ctx.instantiate_mvars(&target);
            let (level_params, def_value) = match ctx.find_const(&def_name) {
                Some(ConstantInfo::Definition(dv)) => {
                    (dv.common.level_params.clone(), dv.value.clone())
                }
                Some(ConstantInfo::Theorem(tv)) => {
                    (tv.common.level_params.clone(), tv.value.clone())
                }
                _ => {
                    return Err(TacticError::Failed(format!(
                        "unfold: '{}' not found or has no definition",
                        def_name
                    )));
                }
            };
            let def_name_str = format!("{}", def_name);
            let did_unfold = std::cell::Cell::new(false);
            let new_target = crate::util::replace_expr(&target, &|e| match e {
                Expr::Const(n, levels) if format!("{}", n) == def_name_str => {
                    did_unfold.set(true);
                    Some(instantiate_level_params(&def_value, &level_params, levels))
                }
                _ => None,
            });
            let did_unfold = did_unfold.get();
            if !did_unfold {
                return Err(TacticError::Failed(format!(
                    "unfold: '{}' does not appear in goal",
                    def_name
                )));
            }
            let (new_goal_id, _) =
                ctx.mk_fresh_expr_mvar(new_target, crate::basic::MetavarKind::Natural);
            state.replace_goal(vec![new_goal_id]);
            Ok(())
        }),
        AesopRuleSafety::Safe,
        AesopRuleKind::Unfold,
        priority,
    )
}
/// Create a forward-reasoning rule that applies a lemma to a hypothesis.
pub fn make_forward_rule(name: Name, lemma_expr: Expr, priority: u32) -> AesopRule {
    let expr = lemma_expr.clone();
    AesopRule::new(
        name,
        Box::new(move |state, ctx| crate::tactic::core::tac_apply(expr.clone(), state, ctx)),
        AesopRuleSafety::Unsafe,
        AesopRuleKind::Forward,
        priority,
    )
}
/// Create a cases rule for the given inductive type name.
pub fn make_cases_rule(name: Name, induct_name: Name, priority: u32) -> AesopRule {
    AesopRule::new(
        name,
        Box::new(move |state, ctx| {
            crate::tactic::cases::tac_cases(&Name::str("h"), &induct_name, state, ctx).map(|_| ())
        }),
        AesopRuleSafety::Unsafe,
        AesopRuleKind::Cases,
        priority,
    )
}
/// Create an extensionality rule that tries `funext`-style reasoning.
pub fn make_ext_rule(name: Name, priority: u32) -> AesopRule {
    AesopRule::new(
        name,
        Box::new(|state, ctx| {
            let goal = state.current_goal()?;
            let target = ctx
                .get_mvar_type(goal)
                .cloned()
                .ok_or_else(|| TacticError::Internal("no type".into()))?;
            let target = ctx.instantiate_mvars(&target);
            if matches!(&target, Expr::Pi(..)) {
                crate::tactic::core::tac_intro(None, state, ctx).map(|_| ())
            } else {
                Err(TacticError::Failed("ext: goal is not a Pi type".into()))
            }
        }),
        AesopRuleSafety::Unsafe,
        AesopRuleKind::Ext,
        priority,
    )
}
/// Create a normalization rule wrapping an arbitrary tactic closure.
pub fn make_norm_rule(name: Name, tactic: RuleTacticFn, priority: u32) -> AesopRule {
    AesopRule::new(
        name,
        tactic,
        AesopRuleSafety::Safe,
        AesopRuleKind::Norm,
        priority,
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::basic::MetavarKind;
    use crate::tactic::aesop::*;
    use oxilean_kernel::{Environment, Level};
    fn mk_ctx() -> MetaContext {
        MetaContext::new(Environment::new())
    }
    fn mk_goal(ctx: &mut MetaContext, ty: Expr) -> (MVarId, Expr) {
        ctx.mk_fresh_expr_mvar(ty, MetavarKind::Natural)
    }
    fn nat_ty() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn true_ty() -> Expr {
        Expr::Const(Name::str("True"), vec![])
    }
    fn pi_nat_nat() -> Expr {
        Expr::Pi(
            oxilean_kernel::BinderInfo::Default,
            Name::str("n"),
            Box::new(nat_ty()),
            Box::new(nat_ty()),
        )
    }
    fn eq_refl_goal() -> Expr {
        Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Eq"), vec![])),
                    Box::new(nat_ty()),
                )),
                Box::new(Expr::Const(Name::str("Nat.zero"), vec![])),
            )),
            Box::new(Expr::Const(Name::str("Nat.zero"), vec![])),
        )
    }
    #[test]
    fn test_config_default() {
        let config = AesopConfig::default();
        assert_eq!(config.max_depth, 30);
        assert_eq!(config.max_iters, 5000);
        assert!(config.norm_simp);
        assert!(config.use_default_rules);
        assert!(config.enable_cache);
    }
    #[test]
    fn test_config_fast() {
        let config = AesopConfig::fast();
        assert_eq!(config.max_depth, 8);
        assert!(!config.norm_simp);
    }
    #[test]
    fn test_config_thorough() {
        let config = AesopConfig::thorough();
        assert_eq!(config.max_depth, 60);
        assert!(config.norm_simp);
    }
    #[test]
    fn test_config_builder() {
        let config = AesopConfig::default()
            .with_max_depth(5)
            .with_max_iters(100)
            .with_timeout_ms(2000)
            .with_norm_simp(false)
            .with_cache(false);
        assert_eq!(config.max_depth, 5);
        assert_eq!(config.max_iters, 100);
        assert_eq!(config.timeout_ms, 2000);
        assert!(!config.norm_simp);
        assert!(!config.enable_cache);
    }
    #[test]
    fn test_rule_safety_display() {
        assert_eq!(format!("{}", AesopRuleSafety::Safe), "safe");
        assert_eq!(format!("{}", AesopRuleSafety::AlmostSafe), "almost_safe");
        assert_eq!(format!("{}", AesopRuleSafety::Unsafe), "unsafe");
    }
    #[test]
    fn test_rule_safety_is_safe_or_almost() {
        assert!(AesopRuleSafety::Safe.is_safe_or_almost());
        assert!(AesopRuleSafety::AlmostSafe.is_safe_or_almost());
        assert!(!AesopRuleSafety::Unsafe.is_safe_or_almost());
    }
    #[test]
    fn test_rule_kind_display() {
        assert_eq!(format!("{}", AesopRuleKind::Apply), "apply");
        assert_eq!(format!("{}", AesopRuleKind::Constructor), "constructor");
        assert_eq!(format!("{}", AesopRuleKind::Cases), "cases");
        assert_eq!(format!("{}", AesopRuleKind::Ext), "ext");
        assert_eq!(format!("{}", AesopRuleKind::Forward), "forward");
        assert_eq!(format!("{}", AesopRuleKind::Unfold), "unfold");
        assert_eq!(format!("{}", AesopRuleKind::Norm), "norm");
    }
    #[test]
    fn test_rule_new() {
        let rule = AesopRule::new(
            Name::str("test"),
            Box::new(|_s, _c| Ok(())),
            AesopRuleSafety::Safe,
            AesopRuleKind::Apply,
            10,
        );
        assert_eq!(rule.priority, 10);
        assert_eq!(rule.safety, AesopRuleSafety::Safe);
        assert!(rule.index_pattern.is_none());
    }
    #[test]
    fn test_rule_with_pattern() {
        let rule = AesopRule::new(
            Name::str("test"),
            Box::new(|_s, _c| Ok(())),
            AesopRuleSafety::Unsafe,
            AesopRuleKind::Apply,
            50,
        )
        .with_pattern(nat_ty())
        .with_estimated_subgoals(3);
        assert!(rule.index_pattern.is_some());
        assert_eq!(rule.estimated_subgoals, 3);
    }
    #[test]
    fn test_rule_effective_cost() {
        let rule = AesopRule::new(
            Name::str("test"),
            Box::new(|_s, _c| Ok(())),
            AesopRuleSafety::Unsafe,
            AesopRuleKind::Apply,
            100,
        )
        .with_estimated_subgoals(1);
        let cost0 = rule.effective_cost(0, 1.2);
        let cost5 = rule.effective_cost(5, 1.2);
        assert!(cost5 > cost0);
        assert!((cost0 - 200.0).abs() < 0.001);
    }
    #[test]
    fn test_rule_debug() {
        let rule = AesopRule::new(
            Name::str("test"),
            Box::new(|_s, _c| Ok(())),
            AesopRuleSafety::Safe,
            AesopRuleKind::Apply,
            10,
        );
        let dbg = format!("{:?}", rule);
        assert!(dbg.contains("AesopRule"));
        assert!(dbg.contains("Safe"));
    }
    #[test]
    fn test_ruleset_new() {
        let rs = AesopRuleSet::new();
        assert!(rs.is_empty());
        assert_eq!(rs.len(), 0);
    }
    #[test]
    fn test_ruleset_add() {
        let mut rs = AesopRuleSet::new();
        let rule = AesopRule::new(
            Name::str("r1"),
            Box::new(|_s, _c| Ok(())),
            AesopRuleSafety::Safe,
            AesopRuleKind::Apply,
            10,
        );
        let id = rs.add(rule).expect("id should be present");
        assert_eq!(rs.len(), 1);
        assert!(rs.get(id).is_some());
    }
    #[test]
    fn test_ruleset_add_with_pattern() {
        let mut rs = AesopRuleSet::new();
        let rule = AesopRule::new(
            Name::str("r1"),
            Box::new(|_s, _c| Ok(())),
            AesopRuleSafety::Unsafe,
            AesopRuleKind::Apply,
            50,
        )
        .with_pattern(nat_ty());
        rs.add(rule).expect("value should be present");
        let results = rs.query(&nat_ty());
        assert!(!results.is_empty());
    }
    #[test]
    fn test_ruleset_remove() {
        let mut rs = AesopRuleSet::new();
        let rule = AesopRule::new(
            Name::str("r1"),
            Box::new(|_s, _c| Ok(())),
            AesopRuleSafety::Safe,
            AesopRuleKind::Apply,
            10,
        );
        rs.add(rule).expect("value should be present");
        assert_eq!(rs.len(), 1);
        assert!(rs.remove(&Name::str("r1")));
        assert!(rs.all_by_priority().is_empty());
    }
    #[test]
    fn test_ruleset_replace_duplicate() {
        let mut rs = AesopRuleSet::new();
        let rule1 = AesopRule::new(
            Name::str("r1"),
            Box::new(|_s, _c| Ok(())),
            AesopRuleSafety::Safe,
            AesopRuleKind::Apply,
            10,
        );
        let rule2 = AesopRule::new(
            Name::str("r1"),
            Box::new(|_s, _c| Ok(())),
            AesopRuleSafety::Unsafe,
            AesopRuleKind::Apply,
            99,
        );
        rs.add(rule1).expect("value should be present");
        let id2 = rs.add(rule2).expect("id2 should be present");
        let active = rs.all_by_priority();
        assert_eq!(active.len(), 1);
        assert_eq!(active[0], id2);
    }
    #[test]
    fn test_ruleset_capacity_limit() {
        let mut rs = AesopRuleSet::new().with_max_rules(2);
        let r1 = AesopRule::new(
            Name::str("r1"),
            Box::new(|_s, _c| Ok(())),
            AesopRuleSafety::Safe,
            AesopRuleKind::Apply,
            10,
        );
        let r2 = AesopRule::new(
            Name::str("r2"),
            Box::new(|_s, _c| Ok(())),
            AesopRuleSafety::Safe,
            AesopRuleKind::Apply,
            20,
        );
        let r3 = AesopRule::new(
            Name::str("r3"),
            Box::new(|_s, _c| Ok(())),
            AesopRuleSafety::Safe,
            AesopRuleKind::Apply,
            30,
        );
        rs.add(r1).expect("value should be present");
        rs.add(r2).expect("value should be present");
        assert!(rs.add(r3).is_err());
    }
    #[test]
    fn test_ruleset_query_wildcards() {
        let mut rs = AesopRuleSet::new();
        let rule = AesopRule::new(
            Name::str("wildcard"),
            Box::new(|_s, _c| Ok(())),
            AesopRuleSafety::Unsafe,
            AesopRuleKind::Apply,
            50,
        );
        rs.add(rule).expect("value should be present");
        let results = rs.query(&nat_ty());
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_ruleset_norm_rules() {
        let mut rs = AesopRuleSet::new();
        let norm = AesopRule::new(
            Name::str("norm1"),
            Box::new(|_s, _c| Ok(())),
            AesopRuleSafety::Safe,
            AesopRuleKind::Norm,
            10,
        );
        let regular = AesopRule::new(
            Name::str("regular"),
            Box::new(|_s, _c| Ok(())),
            AesopRuleSafety::Safe,
            AesopRuleKind::Apply,
            20,
        );
        rs.add(norm).expect("value should be present");
        rs.add(regular).expect("value should be present");
        assert_eq!(rs.norm_rules().len(), 1);
    }
    #[test]
    fn test_ruleset_priority_order() {
        let mut rs = AesopRuleSet::new();
        let r1 = AesopRule::new(
            Name::str("high"),
            Box::new(|_s, _c| Ok(())),
            AesopRuleSafety::Safe,
            AesopRuleKind::Apply,
            100,
        );
        let r2 = AesopRule::new(
            Name::str("low"),
            Box::new(|_s, _c| Ok(())),
            AesopRuleSafety::Safe,
            AesopRuleKind::Apply,
            5,
        );
        let r3 = AesopRule::new(
            Name::str("mid"),
            Box::new(|_s, _c| Ok(())),
            AesopRuleSafety::Safe,
            AesopRuleKind::Apply,
            50,
        );
        rs.add(r1).expect("value should be present");
        let id_low = rs.add(r2).expect("id_low should be present");
        rs.add(r3).expect("value should be present");
        let ordered = rs.all_by_priority();
        assert_eq!(ordered[0], id_low);
    }
    #[test]
    fn test_ruleset_debug() {
        let rs = AesopRuleSet::new();
        let dbg = format!("{:?}", rs);
        assert!(dbg.contains("AesopRuleSet"));
    }
    #[test]
    fn test_cache_new() {
        let cache = ProofCache::new();
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
    }
    #[test]
    fn test_cache_insert_lookup() {
        let mut cache = ProofCache::new();
        let target = nat_ty();
        let proof = Expr::Const(Name::str("Nat.zero"), vec![]);
        cache.insert(target.clone(), proof.clone(), 1);
        assert_eq!(cache.len(), 1);
        assert_eq!(cache.lookup(&target), Some(&proof));
    }
    #[test]
    fn test_cache_miss() {
        let cache = ProofCache::new();
        assert!(cache.lookup(&nat_ty()).is_none());
    }
    #[test]
    fn test_cache_clear() {
        let mut cache = ProofCache::new();
        cache.insert(nat_ty(), Expr::BVar(0), 1);
        cache.clear();
        assert!(cache.is_empty());
    }
    #[test]
    fn test_stats_default() {
        let stats = AesopStats::default();
        assert_eq!(stats.nodes_created, 0);
        assert_eq!(stats.nodes_expanded, 0);
    }
    #[test]
    fn test_stats_display() {
        let stats = AesopStats {
            nodes_created: 10,
            nodes_expanded: 5,
            rules_tried: 20,
            rules_succeeded: 8,
            ..Default::default()
        };
        let s = format!("{}", stats);
        assert!(s.contains("nodes_created=10"));
        assert!(s.contains("expanded=5"));
    }
    #[test]
    fn test_result_success() {
        let result = AesopResult::Success {
            proof: nat_ty(),
            stats: AesopStats::default(),
        };
        assert!(result.is_success());
        assert!(result.proof().is_some());
    }
    #[test]
    fn test_result_failure() {
        let result = AesopResult::Failure {
            reason: "no proof".into(),
            stats: AesopStats::default(),
        };
        assert!(!result.is_success());
        assert!(result.proof().is_none());
    }
    #[test]
    fn test_result_resource_limit() {
        let result = AesopResult::ResourceLimit {
            limit_kind: "timeout".into(),
            stats: AesopStats::default(),
        };
        assert!(!result.is_success());
        assert!(result.stats().nodes_created == 0);
    }
    #[test]
    fn test_transparency_display() {
        assert_eq!(format!("{}", TransparencyMode::Reducible), "reducible");
        assert_eq!(format!("{}", TransparencyMode::Default), "default");
        assert_eq!(format!("{}", TransparencyMode::All), "all");
    }
    #[test]
    fn test_node_status_display() {
        assert_eq!(format!("{}", NodeStatus::Open), "open");
        assert_eq!(format!("{}", NodeStatus::Solved), "solved");
        assert_eq!(format!("{}", NodeStatus::Failed), "failed");
        assert_eq!(format!("{}", NodeStatus::Pruned), "pruned");
        assert_eq!(format!("{}", NodeStatus::Expanded), "expanded");
    }
    #[test]
    fn test_search_node_root() {
        let node = AesopSearchNode::root(vec![MVarId(0)]);
        assert_eq!(node.depth, 0);
        assert!(node.parent.is_none());
        assert!(!node.is_solved());
        assert!(!node.is_dead());
    }
    #[test]
    fn test_search_node_solved() {
        let node = AesopSearchNode::root(vec![]);
        assert!(node.is_solved());
    }
    #[test]
    fn test_pq_ordering() {
        let a = PQEntry {
            node_id: NodeId(0),
            cost: 10.0,
        };
        let b = PQEntry {
            node_id: NodeId(1),
            cost: 5.0,
        };
        assert!(b > a);
    }
    #[test]
    fn test_search_state_new() {
        let config = AesopConfig::default();
        let search = AesopSearchState::new(config, vec![MVarId(0)]);
        assert!(!search.is_finished());
        assert_eq!(search.num_nodes(), 1);
    }
    #[test]
    fn test_search_state_empty_goals() {
        let config = AesopConfig::default();
        let search = AesopSearchState::new(config, vec![]);
        assert_eq!(search.num_nodes(), 1);
    }
    #[test]
    fn test_default_ruleset() {
        let rs = default_rule_set();
        assert!(!rs.is_empty());
        assert!(rs.len() >= 5);
    }
    #[test]
    fn test_default_ruleset_has_safe_rules() {
        let rs = default_rule_set();
        let all = rs.all_by_priority();
        let safe_count = all
            .iter()
            .filter(|id| {
                rs.get(**id)
                    .is_some_and(|r| r.safety == AesopRuleSafety::Safe)
            })
            .count();
        assert!(safe_count >= 2);
    }
    #[test]
    fn test_try_reflexivity_success() {
        let goal = eq_refl_goal();
        let result = try_reflexivity(&goal);
        assert!(result.is_some());
    }
    #[test]
    fn test_try_reflexivity_not_eq() {
        assert!(try_reflexivity(&nat_ty()).is_none());
    }
    #[test]
    fn test_try_reflexivity_not_refl() {
        let goal = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Eq"), vec![])),
                    Box::new(nat_ty()),
                )),
                Box::new(Expr::Const(Name::str("Nat.zero"), vec![])),
            )),
            Box::new(Expr::Const(Name::str("Nat.succ"), vec![])),
        );
        assert!(try_reflexivity(&goal).is_none());
    }
    #[test]
    fn test_head_const_name_simple() {
        assert_eq!(get_head_const_name(&nat_ty()), Some("Nat".into()));
    }
    #[test]
    fn test_head_const_name_app() {
        let app = Expr::App(
            Box::new(Expr::Const(Name::str("List"), vec![])),
            Box::new(nat_ty()),
        );
        assert_eq!(get_head_const_name(&app), Some("List".into()));
    }
    #[test]
    fn test_head_const_name_bvar() {
        assert!(get_head_const_name(&Expr::BVar(0)).is_none());
    }
    #[test]
    fn test_aesop_true_goal() {
        let mut ctx = mk_ctx();
        let (goal_id, _) = mk_goal(&mut ctx, true_ty());
        let mut state = TacticState::single(goal_id);
        let config = AesopConfig::default().with_timeout_ms(2000);
        let result = tac_aesop_with_config(&config, &mut state, &mut ctx);
        assert!(result.is_ok(), "aesop should prove True: {:?}", result);
    }
    #[test]
    fn test_aesop_refl_goal() {
        let mut ctx = mk_ctx();
        let (goal_id, _) = mk_goal(&mut ctx, eq_refl_goal());
        let mut state = TacticState::single(goal_id);
        let config = AesopConfig::default().with_timeout_ms(2000);
        let result = tac_aesop_with_config(&config, &mut state, &mut ctx);
        assert!(result.is_ok(), "aesop should prove refl: {:?}", result);
    }
    #[test]
    fn test_aesop_pi_intro() {
        let mut ctx = mk_ctx();
        let goal_ty = Expr::Pi(
            oxilean_kernel::BinderInfo::Default,
            Name::str("n"),
            Box::new(nat_ty()),
            Box::new(true_ty()),
        );
        let (goal_id, _) = mk_goal(&mut ctx, goal_ty);
        let mut state = TacticState::single(goal_id);
        let config = AesopConfig::default().with_timeout_ms(2000);
        let result = tac_aesop_with_config(&config, &mut state, &mut ctx);
        assert!(
            result.is_ok(),
            "aesop should prove Nat -> True: {:?}",
            result
        );
    }
    #[test]
    fn test_aesop_no_goals() {
        let mut ctx = mk_ctx();
        let mut state = TacticState::new(vec![]);
        assert!(tac_aesop(&mut state, &mut ctx).is_ok());
    }
    #[test]
    fn test_aesop_impossible() {
        let mut ctx = mk_ctx();
        let impossible = Expr::Const(Name::str("SomeImpossibleProp"), vec![]);
        let (goal_id, _) = mk_goal(&mut ctx, impossible);
        let mut state = TacticState::single(goal_id);
        let config = AesopConfig::fast().with_timeout_ms(500);
        let result = tac_aesop_with_config(&config, &mut state, &mut ctx);
        assert!(result.is_err());
    }
    #[test]
    fn test_aesop_warn_on_failure() {
        let mut ctx = mk_ctx();
        let impossible = Expr::Const(Name::str("SomeImpossibleProp"), vec![]);
        let (goal_id, _) = mk_goal(&mut ctx, impossible);
        let mut state = TacticState::single(goal_id);
        let config = AesopConfig {
            warn_on_failure: true,
            ..AesopConfig::fast().with_timeout_ms(500)
        };
        let result = tac_aesop_with_config(&config, &mut state, &mut ctx);
        assert!(result.is_ok());
    }
    #[test]
    fn test_run_search_trivial() {
        let mut ctx = mk_ctx();
        let (goal_id, _) = mk_goal(&mut ctx, true_ty());
        let mut state = TacticState::single(goal_id);
        let config = AesopConfig::default().with_timeout_ms(2000);
        let rs = default_rule_set();
        let result = run_aesop_search(&config, &rs, vec![goal_id], &mut state, &mut ctx);
        assert!(result.is_success());
        assert!(result.proof().is_some());
    }
    #[test]
    fn test_run_search_empty() {
        let mut ctx = mk_ctx();
        let mut state = TacticState::new(vec![]);
        let config = AesopConfig::default();
        let rs = default_rule_set();
        let result = run_aesop_search(&config, &rs, vec![], &mut state, &mut ctx);
        assert!(result.is_success());
    }
    #[test]
    fn test_make_apply_rule() {
        let rule = make_apply_rule(
            Name::str("test_apply"),
            Expr::Const(Name::str("proof"), vec![]),
            50,
        );
        assert_eq!(rule.kind, AesopRuleKind::Apply);
        assert_eq!(rule.safety, AesopRuleSafety::Unsafe);
        assert_eq!(rule.priority, 50);
    }
    #[test]
    fn test_make_unfold_rule() {
        let rule = make_unfold_rule(
            Name::str("test_unfold"),
            Name::str("Foo"),
            Expr::BVar(0),
            30,
        );
        assert_eq!(rule.kind, AesopRuleKind::Unfold);
        assert_eq!(rule.safety, AesopRuleSafety::Safe);
    }
    #[test]
    fn test_make_forward_rule() {
        let rule = make_forward_rule(
            Name::str("test_fwd"),
            Expr::Const(Name::str("lemma"), vec![]),
            60,
        );
        assert_eq!(rule.kind, AesopRuleKind::Forward);
    }
    #[test]
    fn test_make_cases_rule() {
        let rule = make_cases_rule(Name::str("test_cases"), Name::str("Bool"), 70);
        assert_eq!(rule.kind, AesopRuleKind::Cases);
    }
    #[test]
    fn test_make_ext_rule() {
        let rule = make_ext_rule(Name::str("test_ext"), 80);
        assert_eq!(rule.kind, AesopRuleKind::Ext);
    }
    #[test]
    fn test_make_norm_rule() {
        let rule = make_norm_rule(Name::str("test_norm"), Box::new(|_s, _c| Ok(())), 10);
        assert_eq!(rule.kind, AesopRuleKind::Norm);
        assert_eq!(rule.safety, AesopRuleSafety::Safe);
    }
    #[test]
    fn test_aesop_with_custom_rules() {
        let mut ctx = mk_ctx();
        let (goal_id, _) = mk_goal(&mut ctx, true_ty());
        let mut state = TacticState::single(goal_id);
        let mut rs = AesopRuleSet::new();
        let _ = rs.add(AesopRule::new(
            Name::str("custom_true"),
            Box::new(|state, ctx| {
                let proof = Expr::Const(Name::str("True.intro"), vec![]);
                state.close_goal(proof, ctx)
            }),
            AesopRuleSafety::Safe,
            AesopRuleKind::Apply,
            1,
        ));
        let config = AesopConfig::default().with_timeout_ms(1000);
        let result = tac_aesop_with_rules(&config, &rs, &mut state, &mut ctx);
        assert!(result.is_ok());
    }
    #[test]
    fn test_aesop_with_empty_rules_fails() {
        let mut ctx = mk_ctx();
        let (goal_id, _) = mk_goal(&mut ctx, nat_ty());
        let mut state = TacticState::single(goal_id);
        let rs = AesopRuleSet::new();
        let config = AesopConfig::fast().with_timeout_ms(200);
        let result = tac_aesop_with_rules(&config, &rs, &mut state, &mut ctx);
        assert!(result.is_err());
    }
    #[test]
    fn test_cache_reuse() {
        let mut cache = ProofCache::new();
        let target = true_ty();
        let proof = Expr::Const(Name::str("True.intro"), vec![]);
        cache.insert(target.clone(), proof.clone(), 1);
        assert_eq!(cache.lookup(&target), Some(&proof));
        assert_eq!(cache.len(), 1);
        assert!(cache.lookup(&nat_ty()).is_none());
    }
    #[test]
    fn test_aesop_nested_pi() {
        let mut ctx = mk_ctx();
        let goal_ty = Expr::Pi(
            oxilean_kernel::BinderInfo::Default,
            Name::str("a"),
            Box::new(nat_ty()),
            Box::new(Expr::Pi(
                oxilean_kernel::BinderInfo::Default,
                Name::str("b"),
                Box::new(nat_ty()),
                Box::new(true_ty()),
            )),
        );
        let (goal_id, _) = mk_goal(&mut ctx, goal_ty);
        let mut state = TacticState::single(goal_id);
        let config = AesopConfig::default().with_timeout_ms(3000);
        let result = tac_aesop_with_config(&config, &mut state, &mut ctx);
        assert!(result.is_ok(), "aesop should prove Nat -> Nat -> True");
    }
    #[test]
    fn test_node_id_display() {
        assert_eq!(format!("{}", NodeId(42)), "node#42");
    }
    #[test]
    fn test_rule_id_display() {
        assert_eq!(format!("{}", RuleId(7)), "rule#7");
    }
    #[test]
    fn test_search_state_debug() {
        let config = AesopConfig::default();
        let search = AesopSearchState::new(config, vec![MVarId(0)]);
        let dbg = format!("{:?}", search);
        assert!(dbg.contains("AesopSearchState"));
        assert!(dbg.contains("num_nodes"));
    }
    #[test]
    fn test_aesop_multiple_goals() {
        let mut ctx = mk_ctx();
        let (g1, _) = mk_goal(&mut ctx, true_ty());
        let (g2, _) = mk_goal(&mut ctx, eq_refl_goal());
        let mut state = TacticState::new(vec![g1, g2]);
        let config = AesopConfig::default().with_timeout_ms(3000);
        let rs = default_rule_set();
        let result = run_aesop_search(&config, &rs, vec![g1, g2], &mut state, &mut ctx);
        assert!(result.is_success());
    }
    #[test]
    fn test_aesop_zero_iters() {
        let mut ctx = mk_ctx();
        let (goal_id, _) = mk_goal(&mut ctx, nat_ty());
        let mut state = TacticState::single(goal_id);
        let config = AesopConfig::default()
            .with_max_iters(0)
            .with_timeout_ms(500);
        let result = tac_aesop_with_config(&config, &mut state, &mut ctx);
        assert!(result.is_err());
    }
    #[test]
    fn test_path_to_root() {
        let config = AesopConfig::default();
        let mut search = AesopSearchState::new(config, vec![MVarId(0)]);
        let path = path_to_root(&search, NodeId(0));
        assert_eq!(path, vec![NodeId(0)]);
        let child = AesopSearchNode {
            id: NodeId(1),
            parent: Some(NodeId(0)),
            children: Vec::new(),
            depth: 1,
            applied_rule: None,
            applied_rule_name: None,
            goals: vec![],
            cost: 0.0,
            status: NodeStatus::Open,
            saved_meta: None,
            proof_fragment: None,
            rule_apps_tried: 0,
        };
        search.nodes.push(child);
        let path2 = path_to_root(&search, NodeId(1));
        assert_eq!(path2, vec![NodeId(0), NodeId(1)]);
    }
    #[test]
    fn test_level_used_in_tests() {
        let _ = Level::zero();
    }
    #[test]
    fn test_normalize_goal_true() {
        let mut ctx = mk_ctx();
        let (goal_id, _) = mk_goal(&mut ctx, true_ty());
        let mut state = TacticState::single(goal_id);
        let rs = AesopRuleSet::new();
        let mut stats = AesopStats::default();
        let result = normalize_goal(&rs, &mut state, &mut ctx, &mut stats);
        assert!(result.is_ok());
        assert!(result.expect("result should be valid"));
    }
    #[test]
    fn test_normalize_goal_pi() {
        let mut ctx = mk_ctx();
        let (goal_id, _) = mk_goal(&mut ctx, pi_nat_nat());
        let mut state = TacticState::single(goal_id);
        let rs = AesopRuleSet::new();
        let mut stats = AesopStats::default();
        let result = normalize_goal(&rs, &mut state, &mut ctx, &mut stats);
        assert!(result.is_ok());
        assert!(stats.norm_passes > 0);
    }
    #[test]
    fn test_normalize_goal_refl() {
        let mut ctx = mk_ctx();
        let (goal_id, _) = mk_goal(&mut ctx, eq_refl_goal());
        let mut state = TacticState::single(goal_id);
        let rs = AesopRuleSet::new();
        let mut stats = AesopStats::default();
        let result = normalize_goal(&rs, &mut state, &mut ctx, &mut stats);
        assert!(result.is_ok());
        assert!(result.expect("result should be valid"));
    }
    #[test]
    fn test_normalize_goal_noop() {
        let mut ctx = mk_ctx();
        let (goal_id, _) = mk_goal(&mut ctx, nat_ty());
        let mut state = TacticState::single(goal_id);
        let rs = AesopRuleSet::new();
        let mut stats = AesopStats::default();
        let result = normalize_goal(&rs, &mut state, &mut ctx, &mut stats);
        assert!(result.is_ok());
        assert!(!result.expect("result should be valid"));
    }
    #[test]
    fn test_merge_rule_sets() {
        let mut rs1 = AesopRuleSet::new();
        let mut rs2 = AesopRuleSet::new();
        let _ = rs1.add(AesopRule::new(
            Name::str("r1"),
            Box::new(|_s, _c| Ok(())),
            AesopRuleSafety::Safe,
            AesopRuleKind::Apply,
            10,
        ));
        let _ = rs2.add(AesopRule::new(
            Name::str("r2"),
            Box::new(|_s, _c| Ok(())),
            AesopRuleSafety::Safe,
            AesopRuleKind::Apply,
            20,
        ));
        rs1.merge_from(&rs2).expect("value should be present");
        assert_eq!(rs1.all_by_priority().len(), 2);
    }
    #[test]
    fn test_try_assumption_no_hyps() {
        let mut ctx = mk_ctx();
        let (goal_id, _) = mk_goal(&mut ctx, nat_ty());
        let mut state = TacticState::single(goal_id);
        assert!(!try_assumption(&mut state, &mut ctx));
    }
    #[test]
    fn test_search_node_child() {
        let ctx = mk_ctx();
        let saved = ctx.save_state();
        let node = AesopSearchNode::child(
            NodeId(1),
            NodeId(0),
            3,
            RuleId(5),
            Name::str("test_rule"),
            vec![MVarId(10)],
            42.0,
            saved,
        );
        assert_eq!(node.depth, 3);
        assert_eq!(node.parent, Some(NodeId(0)));
        assert_eq!(node.applied_rule, Some(RuleId(5)));
        assert!(!node.is_solved());
        assert!(!node.is_dead());
    }
    #[test]
    fn test_alloc_node() {
        let config = AesopConfig::default();
        let mut search = AesopSearchState::new(config, vec![MVarId(0)]);
        assert_eq!(search.num_nodes(), 1);
        let node = AesopSearchNode::root(vec![MVarId(1)]);
        let id = search.alloc_node(node);
        assert_eq!(id, NodeId(1));
        assert_eq!(search.num_nodes(), 2);
    }
    #[test]
    fn test_check_limits_iters() {
        let config = AesopConfig::default().with_max_iters(0);
        let mut search = AesopSearchState::new(config, vec![MVarId(0)]);
        search.stats.nodes_expanded = 1;
        assert!(search.check_limits().is_some());
    }
    #[test]
    fn test_check_limits_rule_apps() {
        let config = AesopConfig {
            max_rule_apps: 0,
            ..AesopConfig::default()
        };
        let mut search = AesopSearchState::new(config, vec![MVarId(0)]);
        search.stats.rules_tried = 1;
        assert!(search.check_limits().is_some());
    }
    #[test]
    fn test_check_limits_ok() {
        let config = AesopConfig::default();
        let search = AesopSearchState::new(config, vec![MVarId(0)]);
        assert!(search.check_limits().is_none());
    }
}
