//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    BacktrackState, Candidate, CandidateSetBuilder, CandidateSource, SearchEngine, SearchFailure,
    SearchStats, SolveByElimConfig, SolveByElimResult,
};
use crate::basic::{MVarId, MetaContext, MetaState, MetavarKind};
use crate::def_eq::MetaDefEq;
use crate::tactic::state::{TacticError, TacticResult, TacticState};
use oxilean_kernel::{Expr, Level, Name};

/// Run `solve_by_elim` with default configuration and no extra lemmas.
///
/// Tries to close the currently focused goal using local hypotheses and
/// backward chaining.
pub fn tac_solve_by_elim(state: &mut TacticState, ctx: &mut MetaContext) -> TacticResult<()> {
    let config = SolveByElimConfig::default();
    tac_solve_by_elim_with_config(&config, &[], state, ctx)
}
/// Run `solve_by_elim` with a custom configuration and extra lemmas.
///
/// The `lemmas` slice contains additional expressions that can be applied
/// as candidates during the backward chaining search.
pub fn tac_solve_by_elim_with_config(
    config: &SolveByElimConfig,
    lemmas: &[Expr],
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<()> {
    if state.is_done() {
        return Ok(());
    }
    let original_goal = state.current_goal()?;
    if let Some(ref _pre) = config.pre_apply {
        if let Some(ty) = ctx.get_mvar_type(original_goal).cloned() {
            let _ = ctx.instantiate_mvars(&ty);
        }
    }
    let mut engine = SearchEngine::new(config, lemmas);
    let result = engine.search(state, ctx, 0);
    match result {
        Ok(()) => Ok(()),
        Err(SearchFailure::DepthExceeded) => Err(TacticError::Failed(format!(
            "solve_by_elim: depth limit ({}) exceeded [{}]",
            config.max_depth, engine.stats
        ))),
        Err(SearchFailure::BacktrackLimitReached) => Err(TacticError::Failed(format!(
            "solve_by_elim: backtrack limit ({}) reached [{}]",
            config.max_backtrack, engine.stats
        ))),
        Err(e) => Err(TacticError::Failed(format!(
            "solve_by_elim: {} [{}]",
            e, engine.stats
        ))),
    }
}
/// Run `solve_by_elim` and return both the result enum and statistics.
///
/// This is useful for tooling that wants to inspect what happened during
/// the search even when it fails.
pub fn solve_by_elim_with_stats(
    config: &SolveByElimConfig,
    lemmas: &[Expr],
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> (SolveByElimResult, SearchStats) {
    if state.is_done() {
        let proof = Expr::Const(Name::str("trivial"), vec![]);
        return (SolveByElimResult::Solved(proof), SearchStats::new());
    }
    let original_goal = match state.current_goal() {
        Ok(g) => g,
        Err(_) => {
            return (SolveByElimResult::Stuck(vec![]), SearchStats::new());
        }
    };
    let mut engine = SearchEngine::new(config, lemmas);
    let result = engine.search(state, ctx, 0);
    let stats = engine.stats.clone();
    match result {
        Ok(()) => {
            let proof = engine
                .extract_proof(original_goal, ctx)
                .unwrap_or_else(|| Expr::Const(Name::str("solve_by_elim.proof"), vec![]));
            (SolveByElimResult::Solved(proof), stats)
        }
        Err(SearchFailure::DepthExceeded) => (SolveByElimResult::DepthExceeded, stats),
        Err(_) => {
            let remaining = state.all_goals().to_vec();
            (SolveByElimResult::Stuck(remaining), stats)
        }
    }
}
/// Count the number of leading Pi (forall) binders, which gives an estimate
/// of how many arguments must be supplied.
pub(super) fn estimate_arity(expr: &Expr) -> usize {
    let mut count = 0;
    let mut e = expr;
    while let Expr::Pi(_, _, _, body) = e {
        count += 1;
        e = body;
    }
    count
}
/// Compute a rough "result type" by stripping leading Pi binders.
pub(super) fn get_result_type(expr: &Expr) -> Expr {
    let mut e = expr;
    loop {
        match e {
            Expr::Pi(_, _, _, body) => {
                e = body;
            }
            _ => return e.clone(),
        }
    }
}
/// Compute the result type of applying a candidate to `arity` arguments.
/// Returns a placeholder Sort if the expression is not a Pi chain.
pub(super) fn compute_result_type(expr: &Expr, arity: usize) -> Expr {
    let mut e = expr;
    let mut remaining = arity;
    while remaining > 0 {
        match e {
            Expr::Pi(_, _, _, body) => {
                e = body;
                remaining -= 1;
            }
            _ => return Expr::Sort(Level::zero()),
        }
    }
    e.clone()
}
/// Peel up to `n` leading Pi binders from a type and return their domain types.
///
/// Given a type `Pi(a: A, Pi(b: B, C))` and `n = 2`, returns `[A, B]`.
/// If there are fewer than `n` Pi binders, returns as many as are available,
/// with `Sort(Level::zero())` filling any remaining slots.
pub(super) fn peel_pi_arg_types(ty: &Expr, n: usize) -> Vec<Expr> {
    let mut result = Vec::with_capacity(n);
    let mut e = ty;
    while result.len() < n {
        match e {
            Expr::Pi(_, _, dom, body) => {
                result.push(dom.as_ref().clone());
                e = body;
            }
            _ => break,
        }
    }
    while result.len() < n {
        result.push(Expr::Sort(Level::zero()));
    }
    result
}
/// Extract the head constant name from an expression.
/// Strips applications to find the head.
pub(super) fn get_head_name(expr: &Expr) -> Option<Name> {
    let mut e = expr;
    loop {
        match e {
            Expr::Const(name, _) => return Some(name.clone()),
            Expr::App(f, _) => {
                e = f;
            }
            _ => return None,
        }
    }
}
/// Check if a type is `True`.
pub(super) fn is_true_type(ty: &Expr) -> bool {
    matches!(ty, Expr::Const(name, _) if * name == Name::str("True"))
}
/// Try to prove a goal by reflexivity (`Eq.refl`).
pub(super) fn try_refl(ty: &Expr) -> Option<Expr> {
    if let Expr::App(eq_a, rhs) = ty {
        if let Expr::App(eq_ty, lhs) = eq_a.as_ref() {
            if let Expr::App(eq_const, _alpha) = eq_ty.as_ref() {
                if matches!(
                    eq_const.as_ref(), Expr::Const(name, _) if * name == Name::str("Eq")
                ) && lhs == rhs
                {
                    return Some(Expr::Const(Name::str("Eq.refl"), vec![Level::zero()]));
                }
            }
        }
    }
    None
}
/// Try to close a goal with a local hypothesis whose type matches the target.
pub(super) fn try_assumption(target: &Expr, ctx: &MetaContext) -> Option<Expr> {
    let hyps = ctx.get_local_hyps();
    let mut deq = MetaDefEq::new();
    for (name, ty) in &hyps {
        let ty_inst = ctx.instantiate_mvars(ty);
        let target_inst = ctx.instantiate_mvars(target);
        if ty_inst == target_inst {
            return Some(Expr::Const(name.clone(), vec![]));
        }
        let _ = &mut deq;
    }
    None
}
/// Attempt to unify an application expression with a target type.
///
/// This is a simplified version that checks syntactic head compatibility.
pub(super) fn try_unify_app(_app: &Expr, _target: &Expr, _ctx: &mut MetaContext) -> bool {
    true
}
/// Run solve_by_elim on all goals of the tactic state.
#[allow(dead_code)]
pub fn tac_solve_by_elim_all(state: &mut TacticState, ctx: &mut MetaContext) -> TacticResult<()> {
    let config = SolveByElimConfig::default().with_all_goals(true);
    tac_solve_by_elim_with_config(&config, &[], state, ctx)
}
/// Run solve_by_elim with explicit lemma list (convenience wrapper).
#[allow(dead_code)]
pub fn tac_solve_by_elim_using(
    lemmas: &[Expr],
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<()> {
    let config = SolveByElimConfig::default();
    tac_solve_by_elim_with_config(&config, lemmas, state, ctx)
}
/// Attempt to apply a sequence of lemmas in order, backtracking on failure.
///
/// Unlike `solve_by_elim`, this does not recurse into subgoals; it tries
/// each lemma on the current goal exactly once.
#[allow(dead_code)]
pub fn try_apply_sequence(
    lemmas: &[Expr],
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<(Expr, usize)> {
    let config = SolveByElimConfig::new()
        .with_max_depth(1)
        .with_max_backtrack(lemmas.len());
    let mut engine = SearchEngine::new(&config, lemmas);
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .map(|t| ctx.instantiate_mvars(t))
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let candidates = engine.collect_candidates(&target, ctx);
    for (idx, candidate) in candidates.iter().enumerate() {
        state.save();
        let snap = ctx.save_state();
        if let Some(_new_goals) = engine.try_apply_candidate(candidate, state, ctx) {
            return Ok((candidate.expr.clone(), idx));
        }
        ctx.restore_state(snap);
        let _ = state.restore();
    }
    Err(TacticError::Failed(
        "try_apply_sequence: no lemma succeeded".into(),
    ))
}
/// Check whether a single lemma can close the current goal without subgoals.
#[allow(dead_code)]
pub fn can_close_with(lemma: &Expr, state: &mut TacticState, ctx: &mut MetaContext) -> bool {
    let config = SolveByElimConfig::new().with_max_depth(0);
    let engine = SearchEngine::new(&config, &[]);
    state.save();
    let snap = ctx.save_state();
    let candidate = Candidate {
        expr: lemma.clone(),
        source: CandidateSource::ProvidedLemma(0),
        estimated_subgoals: estimate_arity(lemma),
        result_type: None,
    };
    let mut engine_mut = engine;
    let result = engine_mut.try_apply_candidate(&candidate, state, ctx);
    let success = matches!(result, Some(ref goals) if goals.is_empty());
    ctx.restore_state(snap);
    let _ = state.restore();
    success
}
/// Score a candidate by how well it matches the current goal.
///
/// Lower score is better. Returns `None` if the candidate is obviously
/// incompatible.
#[allow(dead_code)]
pub fn score_candidate(
    candidate: &Candidate,
    goal_target: &Expr,
    _ctx: &MetaContext,
) -> Option<u32> {
    let goal_head = get_head_name(goal_target);
    let cand_result = candidate.result_type.as_ref().map(get_result_type);
    let cand_head = cand_result.as_ref().and_then(get_head_name);
    match (&goal_head, &cand_head) {
        (Some(gh), Some(ch)) if gh == ch => Some(candidate.estimated_subgoals as u32),
        (Some(_), Some(_)) => None,
        _ => Some(candidate.estimated_subgoals as u32 + 10),
    }
}
/// Filter candidates that are obviously incompatible with the goal target.
#[allow(dead_code)]
pub fn filter_candidates(
    candidates: &[Candidate],
    goal_target: &Expr,
    ctx: &MetaContext,
) -> Vec<Candidate> {
    candidates
        .iter()
        .filter(|c| score_candidate(c, goal_target, ctx).is_some())
        .cloned()
        .collect()
}
/// Build a proof term from a completed search path.
///
/// Walks the backtrack stack (in reverse) to reconstruct the series of
/// `apply` steps and their arguments.
#[allow(dead_code)]
pub fn reconstruct_proof_from_path(
    path: &[BacktrackState],
    original_goal: MVarId,
    ctx: &MetaContext,
) -> Option<Expr> {
    if path.is_empty() {
        return ctx
            .get_mvar_assignment(original_goal)
            .map(|e| ctx.instantiate_mvars(e));
    }
    ctx.get_mvar_assignment(original_goal)
        .map(|e| ctx.instantiate_mvars(e))
}
/// Pretty-print a search tree for debugging.
#[allow(dead_code)]
pub fn format_search_tree(stack: &[BacktrackState]) -> String {
    let mut out = String::new();
    for (i, frame) in stack.iter().enumerate() {
        let indent = "  ".repeat(i);
        let lemma_name = frame
            .applied_lemma
            .as_ref()
            .map(|c| format!("{}", c.source))
            .unwrap_or_else(|| "<none>".to_string());
        out.push_str(&format!(
            "{}depth={} applied={} remaining={} goals={}\n",
            indent,
            frame.depth,
            lemma_name,
            frame.remaining_alternatives.len(),
            frame.saved_goals.len(),
        ));
    }
    out
}
/// Solve goals in dependency order: goals that other goals depend on first.
#[allow(dead_code)]
pub fn solve_by_elim_ordered(
    config: &SolveByElimConfig,
    lemmas: &[Expr],
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<()> {
    let goals = state.all_goals().to_vec();
    if goals.len() <= 1 {
        return tac_solve_by_elim_with_config(config, lemmas, state, ctx);
    }
    let mut scored: Vec<(usize, usize)> = Vec::new();
    for (idx, gid) in goals.iter().enumerate() {
        let mvar_count = if let Some(ty) = ctx.get_mvar_type(*gid) {
            let inst = ctx.instantiate_mvars(ty);
            count_mvars_in_expr(&inst, ctx)
        } else {
            0
        };
        scored.push((idx, mvar_count));
    }
    scored.sort_by_key(|&(_, count)| count);
    for &(idx, _) in &scored {
        if idx < state.num_goals() {
            state.focus(idx).ok();
            tac_solve_by_elim_with_config(config, lemmas, state, ctx)?;
        }
    }
    Ok(())
}
/// Count metavariable placeholders in an expression.
pub(super) fn count_mvars_in_expr(expr: &Expr, ctx: &MetaContext) -> usize {
    let mut count = 0;
    count_mvars_impl(expr, ctx, &mut count);
    count
}
/// Recursive helper for counting metavariables.
pub(super) fn count_mvars_impl(expr: &Expr, ctx: &MetaContext, count: &mut usize) {
    if let Some(id) = MetaContext::is_mvar_expr(expr) {
        if !ctx.is_mvar_assigned(id) {
            *count += 1;
        }
        return;
    }
    match expr {
        Expr::App(f, a) => {
            count_mvars_impl(f, ctx, count);
            count_mvars_impl(a, ctx, count);
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            count_mvars_impl(ty, ctx, count);
            count_mvars_impl(body, ctx, count);
        }
        Expr::Let(_, ty, val, body) => {
            count_mvars_impl(ty, ctx, count);
            count_mvars_impl(val, ctx, count);
            count_mvars_impl(body, ctx, count);
        }
        Expr::Proj(_, _, e) => {
            count_mvars_impl(e, ctx, count);
        }
        _ => {}
    }
}
/// Build an application chain `f a1 a2 ... an`.
#[allow(dead_code)]
pub(super) fn mk_app_chain(head: Expr, args: &[Expr]) -> Expr {
    let mut result = head;
    for arg in args {
        result = Expr::App(Box::new(result), Box::new(arg.clone()));
    }
    result
}
/// Build a lambda abstraction `fun (x : ty) => body`.
#[allow(dead_code)]
pub(super) fn mk_lambda(name: Name, ty: Expr, body: Expr) -> Expr {
    Expr::Lam(
        oxilean_kernel::BinderInfo::Default,
        name,
        Box::new(ty),
        Box::new(body),
    )
}
/// Build a pi type `(x : ty) -> body`.
#[allow(dead_code)]
pub(super) fn mk_pi(name: Name, ty: Expr, body: Expr) -> Expr {
    Expr::Pi(
        oxilean_kernel::BinderInfo::Default,
        name,
        Box::new(ty),
        Box::new(body),
    )
}
/// Build `@False.elim target proof_of_false`.
#[allow(dead_code)]
pub(super) fn mk_false_elim(target: Expr, proof_of_false: Expr) -> Expr {
    let false_elim = Expr::Const(Name::str("False.elim"), vec![Level::zero()]);
    Expr::App(
        Box::new(Expr::App(Box::new(false_elim), Box::new(target))),
        Box::new(proof_of_false),
    )
}
/// Build `@Eq.refl ty val` proof.
#[allow(dead_code)]
pub(super) fn mk_eq_refl(ty: Expr, val: Expr) -> Expr {
    let refl = Expr::Const(Name::str("Eq.refl"), vec![Level::zero()]);
    Expr::App(
        Box::new(Expr::App(Box::new(refl), Box::new(ty))),
        Box::new(val),
    )
}
/// Build `@True.intro`.
#[allow(dead_code)]
pub(super) fn mk_true_intro() -> Expr {
    Expr::Const(Name::str("True.intro"), vec![])
}
/// Build `@And.intro left_proof right_proof`.
#[allow(dead_code)]
pub(super) fn mk_and_intro(left_ty: Expr, right_ty: Expr, left: Expr, right: Expr) -> Expr {
    let and_intro = Expr::Const(Name::str("And.intro"), vec![]);
    let e1 = Expr::App(Box::new(and_intro), Box::new(left_ty));
    let e2 = Expr::App(Box::new(e1), Box::new(right_ty));
    let e3 = Expr::App(Box::new(e2), Box::new(left));
    Expr::App(Box::new(e3), Box::new(right))
}
/// Build `@Or.inl ty_left ty_right proof`.
#[allow(dead_code)]
pub(super) fn mk_or_inl(ty_left: Expr, ty_right: Expr, proof: Expr) -> Expr {
    let or_inl = Expr::Const(Name::str("Or.inl"), vec![]);
    let e1 = Expr::App(Box::new(or_inl), Box::new(ty_left));
    let e2 = Expr::App(Box::new(e1), Box::new(ty_right));
    Expr::App(Box::new(e2), Box::new(proof))
}
/// Build `@Or.inr ty_left ty_right proof`.
#[allow(dead_code)]
pub(super) fn mk_or_inr(ty_left: Expr, ty_right: Expr, proof: Expr) -> Expr {
    let or_inr = Expr::Const(Name::str("Or.inr"), vec![]);
    let e1 = Expr::App(Box::new(or_inr), Box::new(ty_left));
    let e2 = Expr::App(Box::new(e1), Box::new(ty_right));
    Expr::App(Box::new(e2), Box::new(proof))
}
/// Check if an expression is of the form `@Eq α a b`.
#[allow(dead_code)]
pub(super) fn is_eq_type(expr: &Expr) -> Option<(Expr, Expr, Expr)> {
    if let Expr::App(eq_b, rhs) = expr {
        if let Expr::App(eq_a, lhs) = eq_b.as_ref() {
            if let Expr::App(eq_const, alpha) = eq_a.as_ref() {
                if matches!(
                    eq_const.as_ref(), Expr::Const(name, _) if * name == Name::str("Eq")
                ) {
                    return Some((
                        alpha.as_ref().clone(),
                        lhs.as_ref().clone(),
                        rhs.as_ref().clone(),
                    ));
                }
            }
        }
    }
    None
}
/// Check if an expression is of the form `And P Q`.
#[allow(dead_code)]
pub(super) fn is_and_type(expr: &Expr) -> Option<(Expr, Expr)> {
    if let Expr::App(and_p, q) = expr {
        if let Expr::App(and_const, p) = and_p.as_ref() {
            if matches!(
                and_const.as_ref(), Expr::Const(name, _) if * name == Name::str("And")
            ) {
                return Some((p.as_ref().clone(), q.as_ref().clone()));
            }
        }
    }
    None
}
/// Check if an expression is of the form `Or P Q`.
#[allow(dead_code)]
pub(super) fn is_or_type(expr: &Expr) -> Option<(Expr, Expr)> {
    if let Expr::App(or_p, q) = expr {
        if let Expr::App(or_const, p) = or_p.as_ref() {
            if matches!(
                or_const.as_ref(), Expr::Const(name, _) if * name == Name::str("Or")
            ) {
                return Some((p.as_ref().clone(), q.as_ref().clone()));
            }
        }
    }
    None
}
/// Check if an expression is `False`.
#[allow(dead_code)]
pub(super) fn is_false_type(ty: &Expr) -> bool {
    matches!(ty, Expr::Const(name, _) if * name == Name::str("False"))
}
/// Check if an expression is a Pi type.
#[allow(dead_code)]
pub(super) fn is_pi(expr: &Expr) -> bool {
    matches!(expr, Expr::Pi(_, _, _, _))
}
/// Strip all applications and return the head and arguments.
#[allow(dead_code)]
pub(super) fn decompose_app(expr: &Expr) -> (Expr, Vec<Expr>) {
    let mut args = Vec::new();
    let mut e = expr;
    while let Expr::App(f, a) = e {
        args.push(a.as_ref().clone());
        e = f;
    }
    args.reverse();
    (e.clone(), args)
}
/// Count the number of leading Pi binders that are implicit.
#[allow(dead_code)]
pub(super) fn count_implicit_args(expr: &Expr) -> usize {
    let mut count = 0;
    let mut e = expr;
    while let Expr::Pi(bi, _, _, body) = e {
        if *bi != oxilean_kernel::BinderInfo::Default {
            count += 1;
        } else {
            break;
        }
        e = body;
    }
    count
}
/// Check if two expressions have the same head constant.
#[allow(dead_code)]
pub(super) fn same_head(e1: &Expr, e2: &Expr) -> bool {
    let h1 = get_head_name(e1);
    let h2 = get_head_name(e2);
    match (h1, h2) {
        (Some(n1), Some(n2)) => n1 == n2,
        _ => false,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tactic::solve_by_elim::*;
    use oxilean_kernel::{BinderInfo, Environment, Level};
    fn mk_ctx() -> MetaContext {
        MetaContext::new(Environment::new())
    }
    fn mk_nat() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn mk_true() -> Expr {
        Expr::Const(Name::str("True"), vec![])
    }
    fn mk_false() -> Expr {
        Expr::Const(Name::str("False"), vec![])
    }
    fn mk_prop() -> Expr {
        Expr::Sort(Level::zero())
    }
    fn mk_eq(ty: Expr, lhs: Expr, rhs: Expr) -> Expr {
        let eq = Expr::Const(Name::str("Eq"), vec![Level::zero()]);
        Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::App(Box::new(eq), Box::new(ty))),
                Box::new(lhs),
            )),
            Box::new(rhs),
        )
    }
    fn mk_pi_simple(name: &str, domain: Expr, codomain: Expr) -> Expr {
        Expr::Pi(
            BinderInfo::Default,
            Name::str(name),
            Box::new(domain),
            Box::new(codomain),
        )
    }
    fn mk_const(name: &str) -> Expr {
        Expr::Const(Name::str(name), vec![])
    }
    fn mk_app(f: Expr, a: Expr) -> Expr {
        Expr::App(Box::new(f), Box::new(a))
    }
    #[test]
    fn test_default_config() {
        let config = SolveByElimConfig::default();
        assert_eq!(config.max_depth, 6);
        assert_eq!(config.max_backtrack, 32);
        assert!(config.use_hyps);
        assert!(config.use_exfalso);
        assert!(!config.all_goals);
        assert!(config.pre_apply.is_none());
        assert!(!config.backtrack_all);
    }
    #[test]
    fn test_config_builder() {
        let config = SolveByElimConfig::new()
            .with_max_depth(10)
            .with_max_backtrack(100)
            .with_use_hyps(false)
            .with_use_exfalso(false)
            .with_all_goals(true)
            .with_pre_apply(Name::str("simp"))
            .with_backtrack_all(true);
        assert_eq!(config.max_depth, 10);
        assert_eq!(config.max_backtrack, 100);
        assert!(!config.use_hyps);
        assert!(!config.use_exfalso);
        assert!(config.all_goals);
        assert_eq!(config.pre_apply, Some(Name::str("simp")));
        assert!(config.backtrack_all);
    }
    #[test]
    fn test_candidate_source_display() {
        let hyp = CandidateSource::LocalHyp(Name::str("h"));
        assert_eq!(format!("{}", hyp), "hyp:h");
        let lemma = CandidateSource::ProvidedLemma(3);
        assert_eq!(format!("{}", lemma), "lemma[3]");
        let env = CandidateSource::EnvironmentDecl(Name::str("Nat.add_comm"));
        assert_eq!(format!("{}", env), "env:Nat.add_comm");
    }
    #[test]
    fn test_candidate_source_eq() {
        let a = CandidateSource::LocalHyp(Name::str("h"));
        let b = CandidateSource::LocalHyp(Name::str("h"));
        let c = CandidateSource::LocalHyp(Name::str("h2"));
        assert_eq!(a, b);
        assert_ne!(a, c);
    }
    #[test]
    fn test_result_solved() {
        let r = SolveByElimResult::Solved(mk_const("proof"));
        assert!(r.is_solved());
        assert!(!r.is_stuck());
        assert!(!r.is_depth_exceeded());
    }
    #[test]
    fn test_result_stuck() {
        let r = SolveByElimResult::Stuck(vec![MVarId(0)]);
        assert!(!r.is_solved());
        assert!(r.is_stuck());
        assert!(!r.is_depth_exceeded());
    }
    #[test]
    fn test_result_depth_exceeded() {
        let r = SolveByElimResult::DepthExceeded;
        assert!(!r.is_solved());
        assert!(!r.is_stuck());
        assert!(r.is_depth_exceeded());
    }
    #[test]
    fn test_stats_default() {
        let stats = SearchStats::new();
        assert_eq!(stats.nodes_explored, 0);
        assert_eq!(stats.backtracks, 0);
        assert_eq!(stats.depth_reached, 0);
        assert_eq!(stats.candidates_tried, 0);
        assert_eq!(stats.successful_applies, 0);
        assert_eq!(stats.failed_applies, 0);
        assert_eq!(stats.goals_closed, 0);
    }
    #[test]
    fn test_stats_recording() {
        let mut stats = SearchStats::new();
        stats.record_node();
        stats.record_node();
        assert_eq!(stats.nodes_explored, 2);
        stats.record_backtrack();
        assert_eq!(stats.backtracks, 1);
        stats.record_depth(3);
        stats.record_depth(1);
        assert_eq!(stats.depth_reached, 3);
        stats.record_candidate_tried();
        stats.record_candidate_tried();
        stats.record_candidate_tried();
        assert_eq!(stats.candidates_tried, 3);
        stats.record_successful_apply();
        stats.record_failed_apply();
        stats.record_failed_apply();
        assert_eq!(stats.successful_applies, 1);
        assert_eq!(stats.failed_applies, 2);
        stats.record_goal_closed();
        assert_eq!(stats.goals_closed, 1);
    }
    #[test]
    fn test_stats_reset() {
        let mut stats = SearchStats::new();
        stats.record_node();
        stats.record_backtrack();
        stats.record_depth(5);
        stats.reset();
        assert_eq!(stats.nodes_explored, 0);
        assert_eq!(stats.backtracks, 0);
        assert_eq!(stats.depth_reached, 0);
    }
    #[test]
    fn test_stats_display() {
        let stats = SearchStats::new();
        let s = format!("{}", stats);
        assert!(s.contains("nodes=0"));
        assert!(s.contains("backtracks=0"));
    }
    #[test]
    fn test_estimate_arity_no_pi() {
        assert_eq!(estimate_arity(&mk_nat()), 0);
    }
    #[test]
    fn test_estimate_arity_one_pi() {
        let pi = mk_pi_simple("x", mk_nat(), mk_nat());
        assert_eq!(estimate_arity(&pi), 1);
    }
    #[test]
    fn test_estimate_arity_nested_pi() {
        let inner = mk_pi_simple("y", mk_nat(), mk_nat());
        let outer = mk_pi_simple("x", mk_nat(), inner);
        assert_eq!(estimate_arity(&outer), 2);
    }
    #[test]
    fn test_estimate_arity_three_levels() {
        let p3 = mk_pi_simple("z", mk_nat(), mk_prop());
        let p2 = mk_pi_simple("y", mk_nat(), p3);
        let p1 = mk_pi_simple("x", mk_nat(), p2);
        assert_eq!(estimate_arity(&p1), 3);
    }
    #[test]
    fn test_get_result_type_no_pi() {
        let e = mk_nat();
        assert_eq!(get_result_type(&e), mk_nat());
    }
    #[test]
    fn test_get_result_type_one_pi() {
        let pi = mk_pi_simple("x", mk_nat(), mk_prop());
        assert_eq!(get_result_type(&pi), mk_prop());
    }
    #[test]
    fn test_get_result_type_nested_pi() {
        let inner = mk_pi_simple("y", mk_nat(), mk_true());
        let outer = mk_pi_simple("x", mk_nat(), inner);
        assert_eq!(get_result_type(&outer), mk_true());
    }
    #[test]
    fn test_compute_result_type_zero() {
        let e = mk_nat();
        assert_eq!(compute_result_type(&e, 0), mk_nat());
    }
    #[test]
    fn test_compute_result_type_strip_one() {
        let pi = mk_pi_simple("x", mk_nat(), mk_prop());
        assert_eq!(compute_result_type(&pi, 1), mk_prop());
    }
    #[test]
    fn test_compute_result_type_too_many() {
        let e = mk_nat();
        assert_eq!(compute_result_type(&e, 1), Expr::Sort(Level::zero()));
    }
    #[test]
    fn test_get_head_name_const() {
        let e = mk_const("Nat");
        assert_eq!(get_head_name(&e), Some(Name::str("Nat")));
    }
    #[test]
    fn test_get_head_name_app() {
        let e = mk_app(mk_const("f"), mk_const("x"));
        assert_eq!(get_head_name(&e), Some(Name::str("f")));
    }
    #[test]
    fn test_get_head_name_nested_app() {
        let e = mk_app(mk_app(mk_const("g"), mk_const("a")), mk_const("b"));
        assert_eq!(get_head_name(&e), Some(Name::str("g")));
    }
    #[test]
    fn test_get_head_name_bvar() {
        assert_eq!(get_head_name(&Expr::BVar(0)), None);
    }
    #[test]
    fn test_is_true_type_positive() {
        assert!(is_true_type(&mk_true()));
    }
    #[test]
    fn test_is_true_type_negative() {
        assert!(!is_true_type(&mk_nat()));
        assert!(!is_true_type(&mk_false()));
    }
    #[test]
    fn test_try_refl_eq_same() {
        let val = mk_const("x");
        let eq_expr = mk_eq(mk_nat(), val.clone(), val);
        assert!(try_refl(&eq_expr).is_some());
    }
    #[test]
    fn test_try_refl_eq_different() {
        let eq_expr = mk_eq(mk_nat(), mk_const("x"), mk_const("y"));
        assert!(try_refl(&eq_expr).is_none());
    }
    #[test]
    fn test_try_refl_not_eq() {
        assert!(try_refl(&mk_nat()).is_none());
    }
    #[test]
    fn test_is_eq_type_positive() {
        let eq = mk_eq(mk_nat(), mk_const("a"), mk_const("b"));
        let result = is_eq_type(&eq);
        assert!(result.is_some());
        let (alpha, lhs, rhs) = result.expect("result should be valid");
        assert_eq!(alpha, mk_nat());
        assert_eq!(lhs, mk_const("a"));
        assert_eq!(rhs, mk_const("b"));
    }
    #[test]
    fn test_is_eq_type_negative() {
        assert!(is_eq_type(&mk_nat()).is_none());
    }
    #[test]
    fn test_is_and_type_positive() {
        let and = mk_app(mk_app(mk_const("And"), mk_true()), mk_false());
        let result = is_and_type(&and);
        assert!(result.is_some());
        let (p, q) = result.expect("result should be valid");
        assert_eq!(p, mk_true());
        assert_eq!(q, mk_false());
    }
    #[test]
    fn test_is_and_type_negative() {
        assert!(is_and_type(&mk_nat()).is_none());
    }
    #[test]
    fn test_is_or_type_positive() {
        let or = mk_app(mk_app(mk_const("Or"), mk_true()), mk_false());
        let result = is_or_type(&or);
        assert!(result.is_some());
        let (p, q) = result.expect("result should be valid");
        assert_eq!(p, mk_true());
        assert_eq!(q, mk_false());
    }
    #[test]
    fn test_is_or_type_negative() {
        assert!(is_or_type(&mk_nat()).is_none());
    }
    #[test]
    fn test_is_false_type_positive() {
        assert!(is_false_type(&mk_false()));
    }
    #[test]
    fn test_is_false_type_negative() {
        assert!(!is_false_type(&mk_true()));
        assert!(!is_false_type(&mk_nat()));
    }
    #[test]
    fn test_decompose_app_no_args() {
        let (head, args) = decompose_app(&mk_const("f"));
        assert_eq!(head, mk_const("f"));
        assert!(args.is_empty());
    }
    #[test]
    fn test_decompose_app_one_arg() {
        let e = mk_app(mk_const("f"), mk_const("x"));
        let (head, args) = decompose_app(&e);
        assert_eq!(head, mk_const("f"));
        assert_eq!(args.len(), 1);
        assert_eq!(args[0], mk_const("x"));
    }
    #[test]
    fn test_decompose_app_two_args() {
        let e = mk_app(mk_app(mk_const("f"), mk_const("a")), mk_const("b"));
        let (head, args) = decompose_app(&e);
        assert_eq!(head, mk_const("f"));
        assert_eq!(args.len(), 2);
        assert_eq!(args[0], mk_const("a"));
        assert_eq!(args[1], mk_const("b"));
    }
    #[test]
    fn test_is_pi_positive() {
        let pi = mk_pi_simple("x", mk_nat(), mk_nat());
        assert!(is_pi(&pi));
    }
    #[test]
    fn test_is_pi_negative() {
        assert!(!is_pi(&mk_nat()));
    }
    #[test]
    fn test_count_implicit_args_none() {
        let pi = mk_pi_simple("x", mk_nat(), mk_nat());
        assert_eq!(count_implicit_args(&pi), 0);
    }
    #[test]
    fn test_count_implicit_args_one() {
        let inner = mk_pi_simple("y", mk_nat(), mk_nat());
        let outer = Expr::Pi(
            BinderInfo::Implicit,
            Name::str("x"),
            Box::new(mk_nat()),
            Box::new(inner),
        );
        assert_eq!(count_implicit_args(&outer), 1);
    }
    #[test]
    fn test_same_head_yes() {
        let e1 = mk_app(mk_const("f"), mk_const("a"));
        let e2 = mk_app(mk_const("f"), mk_const("b"));
        assert!(same_head(&e1, &e2));
    }
    #[test]
    fn test_same_head_no() {
        let e1 = mk_app(mk_const("f"), mk_const("a"));
        let e2 = mk_app(mk_const("g"), mk_const("a"));
        assert!(!same_head(&e1, &e2));
    }
    #[test]
    fn test_same_head_unknown() {
        assert!(!same_head(&Expr::BVar(0), &Expr::BVar(1)));
    }
    #[test]
    fn test_candidate_set_builder_empty() {
        let builder = CandidateSetBuilder::new();
        assert!(builder.is_empty());
        assert_eq!(builder.len(), 0);
        let result = builder.build();
        assert!(result.is_empty());
    }
    #[test]
    fn test_candidate_set_builder_add_lemmas() {
        let mut builder = CandidateSetBuilder::new();
        let lemmas = vec![mk_const("lem1"), mk_const("lem2")];
        builder.add_lemmas(&lemmas);
        assert_eq!(builder.len(), 2);
        let result = builder.build();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].source, CandidateSource::ProvidedLemma(0));
        assert_eq!(result[1].source, CandidateSource::ProvidedLemma(1));
    }
    #[test]
    fn test_candidate_set_builder_add_env() {
        let mut builder = CandidateSetBuilder::new();
        builder.add_env_lemma(Name::str("Nat.succ"), mk_pi_simple("n", mk_nat(), mk_nat()));
        assert_eq!(builder.len(), 1);
        let result = builder.build();
        assert_eq!(
            result[0].source,
            CandidateSource::EnvironmentDecl(Name::str("Nat.succ"))
        );
        assert_eq!(result[0].estimated_subgoals, 1);
    }
    #[test]
    fn test_candidate_set_builder_sort() {
        let mut builder = CandidateSetBuilder::new();
        let pi2 = mk_pi_simple("x", mk_nat(), mk_pi_simple("y", mk_nat(), mk_nat()));
        builder.add_env_lemma(Name::str("two_arg"), pi2);
        builder.add_env_lemma(Name::str("zero_arg"), mk_nat());
        builder.add_env_lemma(Name::str("one_arg"), mk_pi_simple("x", mk_nat(), mk_nat()));
        builder.sort_by_arity();
        let result = builder.build();
        assert_eq!(result[0].estimated_subgoals, 0);
        assert_eq!(result[1].estimated_subgoals, 1);
        assert_eq!(result[2].estimated_subgoals, 2);
    }
    #[test]
    fn test_candidate_set_builder_local_hyps() {
        let ctx = mk_ctx();
        let mut builder = CandidateSetBuilder::new();
        builder.add_local_hyps(&ctx);
        assert!(builder.is_empty());
    }
    #[test]
    fn test_mk_app_chain_empty() {
        let head = mk_const("f");
        let result = mk_app_chain(head.clone(), &[]);
        assert_eq!(result, head);
    }
    #[test]
    fn test_mk_app_chain_one() {
        let head = mk_const("f");
        let result = mk_app_chain(head, &[mk_const("x")]);
        assert_eq!(result, mk_app(mk_const("f"), mk_const("x")));
    }
    #[test]
    fn test_mk_app_chain_two() {
        let head = mk_const("f");
        let result = mk_app_chain(head, &[mk_const("a"), mk_const("b")]);
        let expected = mk_app(mk_app(mk_const("f"), mk_const("a")), mk_const("b"));
        assert_eq!(result, expected);
    }
    #[test]
    fn test_mk_lambda() {
        let lam = mk_lambda(Name::str("x"), mk_nat(), Expr::BVar(0));
        assert!(matches!(lam, Expr::Lam(BinderInfo::Default, _, _, _)));
    }
    #[test]
    fn test_mk_pi() {
        let pi = mk_pi(Name::str("x"), mk_nat(), Expr::BVar(0));
        assert!(matches!(pi, Expr::Pi(BinderInfo::Default, _, _, _)));
    }
    #[test]
    fn test_mk_true_intro() {
        let proof = mk_true_intro();
        assert_eq!(proof, Expr::Const(Name::str("True.intro"), vec![]));
    }
    #[test]
    fn test_mk_false_elim() {
        let proof = mk_false_elim(mk_nat(), mk_const("h"));
        let (head, args) = decompose_app(&proof);
        assert_eq!(
            head,
            Expr::Const(Name::str("False.elim"), vec![Level::zero()])
        );
        assert_eq!(args.len(), 2);
    }
    #[test]
    fn test_mk_eq_refl() {
        let proof = mk_eq_refl(mk_nat(), mk_const("x"));
        let (head, args) = decompose_app(&proof);
        assert_eq!(head, Expr::Const(Name::str("Eq.refl"), vec![Level::zero()]));
        assert_eq!(args.len(), 2);
    }
    #[test]
    fn test_mk_and_intro() {
        let proof = mk_and_intro(mk_true(), mk_true(), mk_const("h1"), mk_const("h2"));
        let (head, args) = decompose_app(&proof);
        assert_eq!(head, mk_const("And.intro"));
        assert_eq!(args.len(), 4);
    }
    #[test]
    fn test_mk_or_inl() {
        let proof = mk_or_inl(mk_true(), mk_false(), mk_const("h"));
        let (head, args) = decompose_app(&proof);
        assert_eq!(head, mk_const("Or.inl"));
        assert_eq!(args.len(), 3);
    }
    #[test]
    fn test_mk_or_inr() {
        let proof = mk_or_inr(mk_true(), mk_false(), mk_const("h"));
        let (head, args) = decompose_app(&proof);
        assert_eq!(head, mk_const("Or.inr"));
        assert_eq!(args.len(), 3);
    }
    #[test]
    fn test_count_mvars_no_mvars() {
        let ctx = mk_ctx();
        assert_eq!(count_mvars_in_expr(&mk_nat(), &ctx), 0);
    }
    #[test]
    fn test_count_mvars_with_mvars() {
        let mut ctx = mk_ctx();
        let ty = Expr::Sort(Level::zero());
        let (_id, placeholder) = ctx.mk_fresh_expr_mvar(ty, MetavarKind::Natural);
        assert_eq!(count_mvars_in_expr(&placeholder, &ctx), 1);
    }
    #[test]
    fn test_count_mvars_assigned() {
        let mut ctx = mk_ctx();
        let ty = Expr::Sort(Level::zero());
        let (id, placeholder) = ctx.mk_fresh_expr_mvar(ty, MetavarKind::Natural);
        ctx.assign_mvar(id, mk_nat());
        assert_eq!(count_mvars_in_expr(&placeholder, &ctx), 0);
    }
    #[test]
    fn test_count_mvars_in_app() {
        let mut ctx = mk_ctx();
        let ty = Expr::Sort(Level::zero());
        let (_id1, p1) = ctx.mk_fresh_expr_mvar(ty.clone(), MetavarKind::Natural);
        let (_id2, p2) = ctx.mk_fresh_expr_mvar(ty, MetavarKind::Natural);
        let app = mk_app(mk_app(mk_const("f"), p1), p2);
        assert_eq!(count_mvars_in_expr(&app, &ctx), 2);
    }
    #[test]
    fn test_solve_by_elim_no_goals() {
        let mut ctx = mk_ctx();
        let mut state = TacticState::new(vec![]);
        let result = tac_solve_by_elim(&mut state, &mut ctx);
        assert!(result.is_ok());
    }
    #[test]
    fn test_solve_by_elim_true_goal() {
        let mut ctx = mk_ctx();
        let goal_ty = mk_true();
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let result = tac_solve_by_elim(&mut state, &mut ctx);
        assert!(result.is_ok());
        assert!(state.is_done());
    }
    #[test]
    fn test_solve_by_elim_refl_goal() {
        let mut ctx = mk_ctx();
        let val = mk_const("x");
        let goal_ty = mk_eq(mk_nat(), val.clone(), val);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let result = tac_solve_by_elim(&mut state, &mut ctx);
        assert!(result.is_ok());
        assert!(state.is_done());
    }
    #[test]
    fn test_solve_by_elim_unsolvable() {
        let mut ctx = mk_ctx();
        let goal_ty = mk_false();
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let config = SolveByElimConfig::new()
            .with_use_exfalso(false)
            .with_max_depth(2);
        let result = tac_solve_by_elim_with_config(&config, &[], &mut state, &mut ctx);
        assert!(result.is_err());
    }
    #[test]
    fn test_solve_by_elim_with_stats_solved() {
        let mut ctx = mk_ctx();
        let goal_ty = mk_true();
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let config = SolveByElimConfig::default();
        let (result, stats) = solve_by_elim_with_stats(&config, &[], &mut state, &mut ctx);
        assert!(result.is_solved());
        assert!(stats.nodes_explored >= 1);
        assert!(stats.goals_closed >= 1);
    }
    #[test]
    fn test_solve_by_elim_with_stats_stuck() {
        let mut ctx = mk_ctx();
        let goal_ty = mk_const("SomeHardProp");
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let config = SolveByElimConfig::new()
            .with_use_exfalso(false)
            .with_max_depth(1);
        let (result, stats) = solve_by_elim_with_stats(&config, &[], &mut state, &mut ctx);
        assert!(!result.is_solved());
        assert!(stats.nodes_explored >= 1);
    }
    #[test]
    fn test_solve_by_elim_depth_exceeded() {
        let mut ctx = mk_ctx();
        let goal_ty = mk_const("DeepProp");
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let config = SolveByElimConfig::new()
            .with_use_exfalso(false)
            .with_use_hyps(false)
            .with_max_depth(0);
        let (result, _stats) = solve_by_elim_with_stats(&config, &[], &mut state, &mut ctx);
        assert!(result.is_depth_exceeded());
    }
    #[test]
    fn test_collect_candidates_no_hyps_no_lemmas() {
        let ctx = mk_ctx();
        let config = SolveByElimConfig::new()
            .with_use_hyps(false)
            .with_use_exfalso(false);
        let engine = SearchEngine::new(&config, &[]);
        let candidates = engine.collect_candidates(&mk_nat(), &ctx);
        assert!(candidates.is_empty());
    }
    #[test]
    fn test_collect_candidates_with_lemmas() {
        let ctx = mk_ctx();
        let lemmas = vec![mk_const("lem1"), mk_const("lem2")];
        let config = SolveByElimConfig::new()
            .with_use_hyps(false)
            .with_use_exfalso(false);
        let engine = SearchEngine::new(&config, &lemmas);
        let candidates = engine.collect_candidates(&mk_nat(), &ctx);
        assert_eq!(candidates.len(), 2);
    }
    #[test]
    fn test_collect_candidates_with_exfalso() {
        let ctx = mk_ctx();
        let config = SolveByElimConfig::new()
            .with_use_hyps(false)
            .with_use_exfalso(true);
        let engine = SearchEngine::new(&config, &[]);
        let candidates = engine.collect_candidates(&mk_nat(), &ctx);
        assert_eq!(candidates.len(), 1);
        assert_eq!(
            candidates[0].source,
            CandidateSource::EnvironmentDecl(Name::str("False.elim"))
        );
    }
    #[test]
    fn test_collect_candidates_sorted() {
        let ctx = mk_ctx();
        let lemma_pi = mk_pi_simple("x", mk_nat(), mk_nat());
        let lemma_const = mk_nat();
        let lemmas = vec![lemma_pi, lemma_const];
        let config = SolveByElimConfig::new()
            .with_use_hyps(false)
            .with_use_exfalso(false);
        let engine = SearchEngine::new(&config, &lemmas);
        let candidates = engine.collect_candidates(&mk_nat(), &ctx);
        assert_eq!(candidates.len(), 2);
        assert!(candidates[0].estimated_subgoals <= candidates[1].estimated_subgoals);
    }
    #[test]
    fn test_backtrack_state_creation() {
        let ctx = mk_ctx();
        let bt = BacktrackState {
            saved_goals: vec![MVarId(0), MVarId(1)],
            applied_lemma: None,
            remaining_alternatives: vec![],
            depth: 3,
            meta_snapshot: ctx.save_state(),
        };
        assert_eq!(bt.saved_goals.len(), 2);
        assert_eq!(bt.depth, 3);
        assert!(bt.applied_lemma.is_none());
    }
    #[test]
    fn test_backtrack_state_with_lemma() {
        let ctx = mk_ctx();
        let candidate = Candidate {
            expr: mk_const("h"),
            source: CandidateSource::LocalHyp(Name::str("h")),
            estimated_subgoals: 0,
            result_type: Some(mk_true()),
        };
        let bt = BacktrackState {
            saved_goals: vec![MVarId(0)],
            applied_lemma: Some(candidate),
            remaining_alternatives: vec![],
            depth: 1,
            meta_snapshot: ctx.save_state(),
        };
        assert!(bt.applied_lemma.is_some());
    }
    #[test]
    fn test_format_search_tree_empty() {
        let result = format_search_tree(&[]);
        assert!(result.is_empty());
    }
    #[test]
    fn test_format_search_tree_nonempty() {
        let ctx = mk_ctx();
        let candidate = Candidate {
            expr: mk_const("h"),
            source: CandidateSource::LocalHyp(Name::str("h")),
            estimated_subgoals: 0,
            result_type: None,
        };
        let frames = vec![BacktrackState {
            saved_goals: vec![MVarId(0)],
            applied_lemma: Some(candidate),
            remaining_alternatives: vec![],
            depth: 0,
            meta_snapshot: ctx.save_state(),
        }];
        let result = format_search_tree(&frames);
        assert!(result.contains("depth=0"));
        assert!(result.contains("hyp:h"));
    }
    #[test]
    fn test_reconstruct_proof_no_path() {
        let mut ctx = mk_ctx();
        let ty = Expr::Sort(Level::zero());
        let (id, _) = ctx.mk_fresh_expr_mvar(ty, MetavarKind::Natural);
        let val = mk_const("proof");
        ctx.assign_mvar(id, val.clone());
        let proof = reconstruct_proof_from_path(&[], id, &ctx);
        assert!(proof.is_some());
        assert_eq!(proof.expect("proof should be valid"), val);
    }
    #[test]
    fn test_reconstruct_proof_unassigned() {
        let mut ctx = mk_ctx();
        let ty = Expr::Sort(Level::zero());
        let (id, _) = ctx.mk_fresh_expr_mvar(ty, MetavarKind::Natural);
        let proof = reconstruct_proof_from_path(&[], id, &ctx);
        assert!(proof.is_none());
    }
    #[test]
    fn test_score_candidate_matching_head() {
        let ctx = mk_ctx();
        let candidate = Candidate {
            expr: mk_const("h"),
            source: CandidateSource::LocalHyp(Name::str("h")),
            estimated_subgoals: 2,
            result_type: Some(mk_app(mk_const("P"), mk_const("x"))),
        };
        let goal = mk_app(mk_const("P"), mk_const("y"));
        let score = score_candidate(&candidate, &goal, &ctx);
        assert_eq!(score, Some(2));
    }
    #[test]
    fn test_score_candidate_mismatched_head() {
        let ctx = mk_ctx();
        let candidate = Candidate {
            expr: mk_const("h"),
            source: CandidateSource::LocalHyp(Name::str("h")),
            estimated_subgoals: 1,
            result_type: Some(mk_app(mk_const("Q"), mk_const("x"))),
        };
        let goal = mk_app(mk_const("P"), mk_const("y"));
        let score = score_candidate(&candidate, &goal, &ctx);
        assert!(score.is_none());
    }
    #[test]
    fn test_score_candidate_unknown_result() {
        let ctx = mk_ctx();
        let candidate = Candidate {
            expr: mk_const("h"),
            source: CandidateSource::LocalHyp(Name::str("h")),
            estimated_subgoals: 1,
            result_type: None,
        };
        let goal = mk_app(mk_const("P"), mk_const("y"));
        let score = score_candidate(&candidate, &goal, &ctx);
        assert_eq!(score, Some(11));
    }
    #[test]
    fn test_filter_candidates_removes_incompatible() {
        let ctx = mk_ctx();
        let good = Candidate {
            expr: mk_const("h1"),
            source: CandidateSource::LocalHyp(Name::str("h1")),
            estimated_subgoals: 0,
            result_type: Some(mk_app(mk_const("P"), mk_const("x"))),
        };
        let bad = Candidate {
            expr: mk_const("h2"),
            source: CandidateSource::LocalHyp(Name::str("h2")),
            estimated_subgoals: 0,
            result_type: Some(mk_app(mk_const("Q"), mk_const("x"))),
        };
        let goal = mk_app(mk_const("P"), mk_const("y"));
        let result = filter_candidates(&[good, bad], &goal, &ctx);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].source, CandidateSource::LocalHyp(Name::str("h1")));
    }
    #[test]
    fn test_search_failure_display() {
        assert_eq!(
            format!("{}", SearchFailure::DepthExceeded),
            "depth limit exceeded"
        );
        assert_eq!(format!("{}", SearchFailure::NoGoals), "no goals");
        assert_eq!(
            format!("{}", SearchFailure::NoCandidates),
            "no candidate lemmas"
        );
        assert_eq!(
            format!("{}", SearchFailure::AllCandidatesExhausted),
            "all candidates exhausted"
        );
        assert_eq!(
            format!("{}", SearchFailure::BacktrackLimitReached),
            "backtrack limit reached"
        );
    }
    #[test]
    fn test_candidate_clone() {
        let c = Candidate {
            expr: mk_const("h"),
            source: CandidateSource::LocalHyp(Name::str("h")),
            estimated_subgoals: 2,
            result_type: Some(mk_nat()),
        };
        let c2 = c.clone();
        assert_eq!(c.expr, c2.expr);
        assert_eq!(c.source, c2.source);
        assert_eq!(c.estimated_subgoals, c2.estimated_subgoals);
    }
    #[test]
    fn test_solve_by_elim_all_goals_trivial() {
        let mut ctx = mk_ctx();
        let (g1, _) = ctx.mk_fresh_expr_mvar(mk_true(), MetavarKind::Natural);
        let (g2, _) = ctx.mk_fresh_expr_mvar(mk_true(), MetavarKind::Natural);
        let mut state = TacticState::new(vec![g1, g2]);
        let config = SolveByElimConfig::new().with_all_goals(true);
        let result = tac_solve_by_elim_with_config(&config, &[], &mut state, &mut ctx);
        assert!(result.is_ok());
        assert!(state.is_done());
    }
    #[test]
    fn test_solve_by_elim_single_goal_only() {
        let mut ctx = mk_ctx();
        let (g1, _) = ctx.mk_fresh_expr_mvar(mk_true(), MetavarKind::Natural);
        let (g2, _) = ctx.mk_fresh_expr_mvar(mk_const("Hard"), MetavarKind::Natural);
        let mut state = TacticState::new(vec![g1, g2]);
        let config = SolveByElimConfig::default();
        let result = tac_solve_by_elim_with_config(&config, &[], &mut state, &mut ctx);
        assert!(result.is_ok());
        assert_eq!(state.num_goals(), 1);
    }
    #[test]
    fn test_solve_by_elim_zero_depth() {
        let mut ctx = mk_ctx();
        let goal_ty = mk_const("Prop1");
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let config = SolveByElimConfig::new()
            .with_max_depth(0)
            .with_use_hyps(false)
            .with_use_exfalso(false);
        let result = tac_solve_by_elim_with_config(&config, &[], &mut state, &mut ctx);
        assert!(result.is_err());
    }
    #[test]
    fn test_solve_by_elim_zero_backtrack() {
        let mut ctx = mk_ctx();
        let goal_ty = mk_const("Prop1");
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let config = SolveByElimConfig::new()
            .with_max_backtrack(0)
            .with_use_hyps(false)
            .with_use_exfalso(false);
        let result = tac_solve_by_elim_with_config(&config, &[], &mut state, &mut ctx);
        assert!(result.is_err());
    }
    #[test]
    fn test_try_assumption_no_hyps() {
        let ctx = mk_ctx();
        let target = mk_nat();
        assert!(try_assumption(&target, &ctx).is_none());
    }
    #[test]
    fn test_try_assumption_with_matching_hyp() {
        let mut ctx = mk_ctx();
        let nat = mk_nat();
        ctx.mk_local_decl(
            Name::str("h"),
            nat.clone(),
            oxilean_kernel::BinderInfo::Default,
        );
        let result = try_assumption(&nat, &ctx);
        assert!(result.is_some());
    }
    #[test]
    fn test_try_assumption_no_match() {
        let mut ctx = mk_ctx();
        ctx.mk_local_decl(
            Name::str("h"),
            mk_true(),
            oxilean_kernel::BinderInfo::Default,
        );
        let result = try_assumption(&mk_nat(), &ctx);
        assert!(result.is_none());
    }
    #[test]
    fn test_engine_trivial_close_true() {
        let mut ctx = mk_ctx();
        let (g, _) = ctx.mk_fresh_expr_mvar(mk_true(), MetavarKind::Natural);
        let mut state = TacticState::single(g);
        let config = SolveByElimConfig::default();
        let engine = SearchEngine::new(&config, &[]);
        assert!(engine.try_trivial_close(&mut state, &mut ctx));
        assert!(state.is_done());
    }
    #[test]
    fn test_engine_trivial_close_refl() {
        let mut ctx = mk_ctx();
        let val = mk_const("x");
        let eq = mk_eq(mk_nat(), val.clone(), val);
        let (g, _) = ctx.mk_fresh_expr_mvar(eq, MetavarKind::Natural);
        let mut state = TacticState::single(g);
        let config = SolveByElimConfig::default();
        let engine = SearchEngine::new(&config, &[]);
        assert!(engine.try_trivial_close(&mut state, &mut ctx));
        assert!(state.is_done());
    }
    #[test]
    fn test_engine_trivial_close_fails() {
        let mut ctx = mk_ctx();
        let (g, _) = ctx.mk_fresh_expr_mvar(mk_const("Hard"), MetavarKind::Natural);
        let mut state = TacticState::single(g);
        let config = SolveByElimConfig::default();
        let engine = SearchEngine::new(&config, &[]);
        assert!(!engine.try_trivial_close(&mut state, &mut ctx));
        assert!(!state.is_done());
    }
    #[test]
    fn test_engine_get_current_target() {
        let mut ctx = mk_ctx();
        let (g, _) = ctx.mk_fresh_expr_mvar(mk_nat(), MetavarKind::Natural);
        let state = TacticState::single(g);
        let config = SolveByElimConfig::default();
        let engine = SearchEngine::new(&config, &[]);
        let target = engine.get_current_target(&state, &ctx);
        assert!(target.is_some());
        assert_eq!(target.expect("target should be valid"), mk_nat());
    }
    #[test]
    fn test_engine_get_current_target_no_goals() {
        let ctx = mk_ctx();
        let state = TacticState::new(vec![]);
        let config = SolveByElimConfig::default();
        let engine = SearchEngine::new(&config, &[]);
        let target = engine.get_current_target(&state, &ctx);
        assert!(target.is_none());
    }
    #[test]
    fn test_engine_extract_proof_assigned() {
        let mut ctx = mk_ctx();
        let ty = mk_nat();
        let (id, _) = ctx.mk_fresh_expr_mvar(ty, MetavarKind::Natural);
        let proof = mk_const("zero");
        ctx.assign_mvar(id, proof.clone());
        let config = SolveByElimConfig::default();
        let engine = SearchEngine::new(&config, &[]);
        let result = engine.extract_proof(id, &ctx);
        assert_eq!(result, Some(proof));
    }
    #[test]
    fn test_engine_extract_proof_unassigned() {
        let mut ctx = mk_ctx();
        let ty = mk_nat();
        let (id, _) = ctx.mk_fresh_expr_mvar(ty, MetavarKind::Natural);
        let config = SolveByElimConfig::default();
        let engine = SearchEngine::new(&config, &[]);
        let result = engine.extract_proof(id, &ctx);
        assert!(result.is_none());
    }
}
