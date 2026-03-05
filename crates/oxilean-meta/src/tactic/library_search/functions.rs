//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    CacheLookup, CandidateError, LemmaCandidate, LemmaEntry, LemmaIndex, LibrarySearchConfig,
    ScoredEntry, ScoringCriteria, SearchCache, SearchResult, SearchState, TypeDirectedSearch,
    TypeSignature,
};
use crate::basic::{MVarId, MetaContext, MetavarKind};
use crate::def_eq::{MetaDefEq, UnificationResult};
use crate::discr_tree::DiscrTree;
use crate::tactic::state::{TacticError, TacticResult, TacticState};
use oxilean_kernel::{Expr, Level, Name};
use std::collections::{HashMap, HashSet, VecDeque};

pub(super) fn strip_leading_pis_local(ty: &Expr) -> Expr {
    let mut current = ty.clone();
    while let Expr::Pi(_, _, _, body) = &current {
        current = *body.clone();
    }
    current
}
pub(super) fn count_leading_pis_local(ty: &Expr) -> usize {
    let mut count = 0;
    let mut current = ty;
    while let Expr::Pi(_, _, _, body) = current {
        count += 1;
        current = body;
    }
    count
}
pub(super) fn compute_specificity_local(expr: &Expr) -> f64 {
    use crate::discr_tree::{encode_expr, DiscrTreeKey};
    let keys = encode_expr(expr);
    if keys.is_empty() {
        return 0.0;
    }
    let non_star = keys.iter().filter(|k| **k != DiscrTreeKey::Star).count();
    non_star as f64 / keys.len() as f64
}
/// `exact?` — find a single expression that closes the current goal.
///
/// This is the Lean 4 `exact?` tactic. It searches the environment for a
/// lemma whose fully-applied form has exactly the goal type.
pub fn tac_exact_question(state: &mut TacticState, ctx: &mut MetaContext) -> TacticResult<()> {
    let config = LibrarySearchConfig::exact_mode();
    tac_library_search_with_config(state, ctx, config)
}
/// `apply?` — find a lemma whose conclusion matches the goal, possibly
/// leaving subgoals for its arguments.
#[allow(dead_code)]
pub fn tac_apply_question(state: &mut TacticState, ctx: &mut MetaContext) -> TacticResult<()> {
    let config = LibrarySearchConfig::apply_mode();
    tac_library_search_with_config(state, ctx, config)
}
/// `library_search` — the main tactic entry point with default config.
pub fn tac_library_search(state: &mut TacticState, ctx: &mut MetaContext) -> TacticResult<()> {
    let config = LibrarySearchConfig::default();
    tac_library_search_with_config(state, ctx, config)
}
/// Core implementation: run library search with a given configuration.
pub(super) fn tac_library_search_with_config(
    state: &mut TacticState,
    ctx: &mut MetaContext,
    config: LibrarySearchConfig,
) -> TacticResult<()> {
    let goal_id = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal_id)
        .cloned()
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let target = ctx.instantiate_mvars(&target);
    let mut index = LemmaIndex::from_environment(ctx);
    if config.include_local {
        index.add_local_hyps(ctx);
    }
    let result = run_search(&target, &index, ctx, &config)?;
    match result {
        SearchResult::Found(proof, suggestion) => {
            if config.suggest_only {
                return Err(TacticError::Failed(format!("Try this: {}", suggestion)));
            }
            state.close_goal(proof, ctx)?;
            Ok(())
        }
        SearchResult::MultipleFound(candidates) => {
            if let Some(best) = candidates.first() {
                if config.suggest_only {
                    let suggestions: Vec<String> =
                        candidates.iter().map(|c| c.suggestion.clone()).collect();
                    return Err(TacticError::Failed(format!(
                        "Try one of:\n{}",
                        suggestions.join("\n")
                    )));
                }
                if let Some(proof) = &best.proof {
                    state.close_goal(proof.clone(), ctx)?;
                    return Ok(());
                }
            }
            Err(TacticError::Failed(
                "library_search: found candidates but none fully closed the goal".into(),
            ))
        }
        SearchResult::NotFound => Err(TacticError::Failed(
            "library_search: no matching lemma found".into(),
        )),
        SearchResult::TimedOut => Err(TacticError::Failed("library_search: timed out".into())),
    }
}
/// Run the search procedure against the given `target` type.
pub(super) fn run_search(
    target: &Expr,
    index: &LemmaIndex,
    ctx: &mut MetaContext,
    config: &LibrarySearchConfig,
) -> TacticResult<SearchResult> {
    let mut search_state = SearchState::new(config.clone());
    let entries: Vec<LemmaEntry> = if config.use_discr_tree {
        index.lookup(target).into_iter().cloned().collect()
    } else {
        index.all_entries().into_iter().cloned().collect()
    };
    let mut sorted_entries = entries;
    sorted_entries.sort_by(|a, b| {
        b.specificity
            .partial_cmp(&a.specificity)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let mut queue: VecDeque<(LemmaEntry, u32)> = VecDeque::new();
    for entry in &sorted_entries {
        queue.push_back((entry.clone(), 0));
    }
    while let Some((entry, depth)) = queue.pop_front() {
        if search_state.is_budget_exhausted() {
            break;
        }
        if depth > config.max_depth {
            continue;
        }
        if search_state.already_failed(&entry.name) {
            continue;
        }
        search_state.candidates_tried += 1;
        let saved = ctx.save_state();
        match try_candidate(target, &entry, ctx, config, depth) {
            Ok(candidate) => {
                if candidate.remaining_goals == 0 {
                    if !config.suggest_only && search_state.results.is_empty() {
                        ctx.restore_state(saved.clone());
                        let saved2 = ctx.save_state();
                        match try_candidate(target, &entry, ctx, config, depth) {
                            Ok(c) => {
                                if let Some(proof) = c.proof {
                                    return Ok(SearchResult::Found(proof, c.suggestion));
                                }
                                ctx.restore_state(saved2);
                            }
                            Err(_) => {
                                ctx.restore_state(saved2);
                            }
                        }
                    }
                    search_state.record_result(candidate);
                } else if config.allow_subgoals
                    && candidate.remaining_goals <= config.max_remaining_goals
                {
                    search_state.record_result(candidate);
                }
                ctx.restore_state(saved);
            }
            Err(_) => {
                ctx.restore_state(saved);
                search_state.mark_failed(&entry.name);
            }
        }
        if !config.allow_subgoals {
            if let Some(best) = search_state.results.first() {
                if best.remaining_goals == 0 {
                    break;
                }
            }
        }
    }
    if search_state.results.is_empty() {
        if search_state.is_timed_out() {
            return Ok(SearchResult::TimedOut);
        }
        return Ok(SearchResult::NotFound);
    }
    if search_state.results.len() == 1 {
        let c = search_state.results.remove(0);
        if let Some(proof) = c.proof {
            return Ok(SearchResult::Found(proof, c.suggestion));
        }
    }
    Ok(SearchResult::MultipleFound(search_state.results))
}
/// Attempt to use a single `LemmaEntry` to close (or partially close) the
/// goal whose type is `target`.
#[allow(clippy::too_many_arguments)]
pub(super) fn try_candidate(
    target: &Expr,
    entry: &LemmaEntry,
    ctx: &mut MetaContext,
    config: &LibrarySearchConfig,
    depth: u32,
) -> Result<LemmaCandidate, CandidateError> {
    let lemma_ty = freshen_universe_params(&entry.ty, entry.num_univ_params, ctx);
    let (applied_expr, arg_mvars, conclusion) =
        open_pis_as_mvars(&entry.name, &lemma_ty, ctx, config.max_synth_args);
    let mut deq = MetaDefEq::new();
    let unif = deq.is_def_eq(&conclusion, target, ctx);
    if unif != UnificationResult::Equal {
        return Err(CandidateError::UnificationFailed);
    }
    let mut applied_args = Vec::new();
    let mut remaining_goals: Vec<MVarId> = Vec::new();
    let mut num_synth = 0usize;
    for mvar_id in &arg_mvars {
        if ctx.is_mvar_assigned(*mvar_id) {
            let val = ctx.instantiate_mvars(
                &ctx.get_mvar_assignment(*mvar_id)
                    .cloned()
                    .unwrap_or(Expr::BVar(0)),
            );
            applied_args.push(val);
            num_synth += 1;
        } else {
            let arg_ty = ctx
                .get_mvar_type(*mvar_id)
                .cloned()
                .unwrap_or_else(|| Expr::Sort(Level::zero()));
            let arg_ty = ctx.instantiate_mvars(&arg_ty);
            if let Some(hyp_expr) = try_synth_from_hyps(&arg_ty, ctx, depth) {
                ctx.reassign_mvar(*mvar_id, hyp_expr.clone());
                applied_args.push(hyp_expr);
                num_synth += 1;
            } else if let Some(trivial) = try_synth_trivial(&arg_ty) {
                ctx.reassign_mvar(*mvar_id, trivial.clone());
                applied_args.push(trivial);
                num_synth += 1;
            } else {
                remaining_goals.push(*mvar_id);
                applied_args.push(Expr::BVar(0));
            }
        }
    }
    let proof = ctx.instantiate_mvars(&applied_expr);
    let has_unsolved = ctx.has_unassigned_mvars(&proof);
    let criteria = ScoringCriteria {
        specificity: entry.specificity,
        remaining_goals: remaining_goals.len(),
        edit_distance: compute_edit_distance(&entry.ty, target),
        is_local: entry.is_local,
        num_universe_params: entry.num_univ_params,
        num_synth_args: num_synth,
        total_args: arg_mvars.len(),
    };
    let score = criteria.score(config);
    if score < config.min_score {
        return Err(CandidateError::ScoreTooLow);
    }
    let suggestion = build_suggestion(&entry.name, &applied_args, remaining_goals.is_empty());
    let candidate = LemmaCandidate {
        name: entry.name.clone(),
        ty: entry.ty.clone(),
        score,
        applied_args,
        remaining_goals: remaining_goals.len(),
        proof: if remaining_goals.is_empty() && !has_unsolved {
            Some(proof)
        } else {
            None
        },
        suggestion,
        criteria,
    };
    Ok(candidate)
}
/// Open all leading Pi binders of `ty`, creating a fresh metavariable for
/// each parameter. Returns `(applied_expr, mvar_ids, conclusion)`.
///
/// `applied_expr` is `@name ?m1 ?m2 ...`; `conclusion` is the body with
/// all BVars replaced by the corresponding metavariable placeholders.
pub(super) fn open_pis_as_mvars(
    name: &Name,
    ty: &Expr,
    ctx: &mut MetaContext,
    max_args: usize,
) -> (Expr, Vec<MVarId>, Expr) {
    let mut current_ty = ty.clone();
    let mut mvar_ids = Vec::new();
    let mut mvar_exprs: Vec<Expr> = Vec::new();
    let mut count = 0usize;
    while let Expr::Pi(_bi, _binder_name, domain, body) = &current_ty {
        if count >= max_args {
            break;
        }
        let domain_inst = substitute_bvars_with_exprs(domain, &mvar_exprs);
        let (mvar_id, mvar_placeholder) = ctx.mk_fresh_expr_mvar(domain_inst, MetavarKind::Natural);
        mvar_ids.push(mvar_id);
        mvar_exprs.push(mvar_placeholder);
        current_ty = substitute_bvar0(
            body,
            mvar_exprs
                .last()
                .expect("mvar_exprs is non-empty; we just pushed to it"),
        );
        count += 1;
    }
    let mut applied = Expr::Const(name.clone(), vec![]);
    for me in &mvar_exprs {
        applied = Expr::App(Box::new(applied), Box::new(me.clone()));
    }
    let conclusion = current_ty;
    (applied, mvar_ids, conclusion)
}
/// Substitute BVar(0) in `body` with `replacement`, shifting other BVars.
pub(super) fn substitute_bvar0(body: &Expr, replacement: &Expr) -> Expr {
    substitute_bvar_inner(body, 0, replacement)
}
/// Recursive BVar substitution.
pub(super) fn substitute_bvar_inner(expr: &Expr, target_idx: u32, replacement: &Expr) -> Expr {
    match expr {
        Expr::BVar(n) => {
            if *n == target_idx {
                replacement.clone()
            } else if *n > target_idx {
                Expr::BVar(n - 1)
            } else {
                expr.clone()
            }
        }
        Expr::App(f, a) => {
            let f2 = substitute_bvar_inner(f, target_idx, replacement);
            let a2 = substitute_bvar_inner(a, target_idx, replacement);
            Expr::App(Box::new(f2), Box::new(a2))
        }
        Expr::Lam(bi, nm, ty, body) => {
            let ty2 = substitute_bvar_inner(ty, target_idx, replacement);
            let body2 = substitute_bvar_inner(body, target_idx + 1, replacement);
            Expr::Lam(*bi, nm.clone(), Box::new(ty2), Box::new(body2))
        }
        Expr::Pi(bi, nm, ty, body) => {
            let ty2 = substitute_bvar_inner(ty, target_idx, replacement);
            let body2 = substitute_bvar_inner(body, target_idx + 1, replacement);
            Expr::Pi(*bi, nm.clone(), Box::new(ty2), Box::new(body2))
        }
        Expr::Let(nm, ty, val, body) => {
            let ty2 = substitute_bvar_inner(ty, target_idx, replacement);
            let val2 = substitute_bvar_inner(val, target_idx, replacement);
            let body2 = substitute_bvar_inner(body, target_idx + 1, replacement);
            Expr::Let(nm.clone(), Box::new(ty2), Box::new(val2), Box::new(body2))
        }
        Expr::Proj(nm, idx, e) => {
            let e2 = substitute_bvar_inner(e, target_idx, replacement);
            Expr::Proj(nm.clone(), *idx, Box::new(e2))
        }
        _ => expr.clone(),
    }
}
/// Substitute BVar(i) with `replacements[replacements.len() - 1 - i]` for
/// all i in range.
pub(super) fn substitute_bvars_with_exprs(expr: &Expr, replacements: &[Expr]) -> Expr {
    if replacements.is_empty() {
        return expr.clone();
    }
    let mut result = expr.clone();
    for (i, repl) in replacements.iter().enumerate().rev() {
        result = substitute_bvar_inner(&result, i as u32, repl);
    }
    result
}
/// Try to find a local hypothesis whose type is definitionally equal to
/// `arg_ty`.
pub(super) fn try_synth_from_hyps(
    arg_ty: &Expr,
    ctx: &mut MetaContext,
    _depth: u32,
) -> Option<Expr> {
    let hyps = ctx.get_local_hyps();
    let mut deq = MetaDefEq::new();
    for (hyp_name, hyp_ty) in &hyps {
        let hyp_ty_inst = ctx.instantiate_mvars(hyp_ty);
        if deq.is_def_eq(&hyp_ty_inst, arg_ty, ctx) == UnificationResult::Equal {
            return Some(Expr::Const(hyp_name.clone(), vec![]));
        }
    }
    None
}
/// Try to synthesize a trivial proof for the argument type.
///
/// Handles `True`, `@Eq a x x`, `Sort _`.
pub(super) fn try_synth_trivial(arg_ty: &Expr) -> Option<Expr> {
    if matches!(arg_ty, Expr::Const(n, _) if * n == Name::str("True")) {
        return Some(Expr::Const(Name::str("True.intro"), vec![]));
    }
    if let Some(proof) = try_refl_proof(arg_ty) {
        return Some(proof);
    }
    if matches!(arg_ty, Expr::Sort(l) if * l == Level::zero()) {
        return Some(Expr::Const(Name::str("True"), vec![]));
    }
    None
}
/// If `ty` is `@Eq α a a`, produce `@Eq.refl α a`.
pub(super) fn try_refl_proof(ty: &Expr) -> Option<Expr> {
    if let Expr::App(eq_lhs_box, rhs) = ty {
        if let Expr::App(eq_alpha_box, lhs) = eq_lhs_box.as_ref() {
            if let Expr::App(eq_const_box, _alpha) = eq_alpha_box.as_ref() {
                if matches!(
                    eq_const_box.as_ref(), Expr::Const(n, _) if * n == Name::str("Eq")
                ) && lhs == rhs
                {
                    return Some(Expr::Const(Name::str("Eq.refl"), vec![Level::zero()]));
                }
            }
        }
    }
    None
}
/// Replace every universe-level parameter in `ty` with a fresh level
/// metavariable from `ctx`.
pub(super) fn freshen_universe_params(ty: &Expr, num_params: usize, ctx: &mut MetaContext) -> Expr {
    if num_params == 0 {
        return ty.clone();
    }
    let fresh_levels: Vec<Level> = (0..num_params).map(|_| ctx.mk_fresh_level_mvar()).collect();
    freshen_levels_in_expr(ty, &fresh_levels, 0)
}
/// Replace `Level::Param` references with the corresponding fresh level from
/// `levels`.
pub(super) fn freshen_levels_in_expr(expr: &Expr, levels: &[Level], _depth: u32) -> Expr {
    match expr {
        Expr::Sort(l) => Expr::Sort(freshen_level(l, levels)),
        Expr::Const(name, existing_levels) => {
            let new_levels: Vec<Level> = if existing_levels.is_empty() {
                levels.to_vec()
            } else {
                existing_levels
                    .iter()
                    .map(|l| freshen_level(l, levels))
                    .collect()
            };
            Expr::Const(name.clone(), new_levels)
        }
        Expr::App(f, a) => {
            let f2 = freshen_levels_in_expr(f, levels, _depth);
            let a2 = freshen_levels_in_expr(a, levels, _depth);
            Expr::App(Box::new(f2), Box::new(a2))
        }
        Expr::Lam(bi, nm, ty, body) => {
            let ty2 = freshen_levels_in_expr(ty, levels, _depth);
            let body2 = freshen_levels_in_expr(body, levels, _depth);
            Expr::Lam(*bi, nm.clone(), Box::new(ty2), Box::new(body2))
        }
        Expr::Pi(bi, nm, ty, body) => {
            let ty2 = freshen_levels_in_expr(ty, levels, _depth);
            let body2 = freshen_levels_in_expr(body, levels, _depth);
            Expr::Pi(*bi, nm.clone(), Box::new(ty2), Box::new(body2))
        }
        Expr::Let(nm, ty, val, body) => {
            let ty2 = freshen_levels_in_expr(ty, levels, _depth);
            let val2 = freshen_levels_in_expr(val, levels, _depth);
            let body2 = freshen_levels_in_expr(body, levels, _depth);
            Expr::Let(nm.clone(), Box::new(ty2), Box::new(val2), Box::new(body2))
        }
        Expr::Proj(nm, idx, e) => {
            let e2 = freshen_levels_in_expr(e, levels, _depth);
            Expr::Proj(nm.clone(), *idx, Box::new(e2))
        }
        _ => expr.clone(),
    }
}
/// Replace `Level::Param` with the corresponding fresh level.
pub(super) fn freshen_level(level: &Level, fresh: &[Level]) -> Level {
    match level {
        Level::Param(name) => {
            let idx = param_name_to_index(name, fresh.len());
            fresh[idx].clone()
        }
        Level::Succ(inner) => Level::succ(freshen_level(inner, fresh)),
        Level::Max(l, r) => Level::max(freshen_level(l, fresh), freshen_level(r, fresh)),
        Level::IMax(l, r) => Level::imax(freshen_level(l, fresh), freshen_level(r, fresh)),
        _ => level.clone(),
    }
}
/// Map a universe-parameter name to an index in `[0, count)`.
pub(super) fn param_name_to_index(name: &Name, count: usize) -> usize {
    if count == 0 {
        return 0;
    }
    let hash = simple_name_hash(name);
    hash % count
}
/// A simple hash for `Name` values.
pub(super) fn simple_name_hash(name: &Name) -> usize {
    let s = format!("{}", name);
    let mut h: usize = 5381;
    for byte in s.bytes() {
        h = h.wrapping_mul(33).wrapping_add(byte as usize);
    }
    h
}
/// Compute the specificity of an expression as the ratio of non-Star keys
/// in its DiscrTree encoding.
pub(super) fn compute_specificity(expr: &Expr) -> f64 {
    use crate::discr_tree::{encode_expr, DiscrTreeKey};
    let keys = encode_expr(expr);
    if keys.is_empty() {
        return 0.0;
    }
    let non_star = keys.iter().filter(|k| **k != DiscrTreeKey::Star).count();
    non_star as f64 / keys.len() as f64
}
/// Strip all leading Pi binders from `ty`, returning the final body.
pub(super) fn strip_leading_pis(ty: &Expr) -> Expr {
    let mut current = ty.clone();
    while let Expr::Pi(_, _, _, body) = &current {
        current = *body.clone();
    }
    current
}
/// Count the number of leading Pi binders.
#[allow(dead_code)]
pub(super) fn count_leading_pis(ty: &Expr) -> usize {
    let mut count = 0;
    let mut current = ty;
    while let Expr::Pi(_, _, _, body) = current {
        count += 1;
        current = body;
    }
    count
}
/// Compute a simple structural edit distance between two expressions.
///
/// This is not a full tree-edit-distance; it is a heuristic that counts
/// the number of positions where the two expression trees differ.
pub(super) fn compute_edit_distance(e1: &Expr, e2: &Expr) -> u32 {
    compute_edit_distance_impl(e1, e2, 0, 8)
}
pub(super) fn compute_edit_distance_impl(e1: &Expr, e2: &Expr, depth: u32, max_depth: u32) -> u32 {
    if depth >= max_depth {
        return if e1 == e2 { 0 } else { 1 };
    }
    if e1 == e2 {
        return 0;
    }
    match (e1, e2) {
        (Expr::App(f1, a1), Expr::App(f2, a2)) => {
            compute_edit_distance_impl(f1, f2, depth + 1, max_depth)
                + compute_edit_distance_impl(a1, a2, depth + 1, max_depth)
        }
        (Expr::Pi(_, _, d1, b1), Expr::Pi(_, _, d2, b2))
        | (Expr::Lam(_, _, d1, b1), Expr::Lam(_, _, d2, b2)) => {
            compute_edit_distance_impl(d1, d2, depth + 1, max_depth)
                + compute_edit_distance_impl(b1, b2, depth + 1, max_depth)
        }
        (Expr::Let(_, t1, v1, b1), Expr::Let(_, t2, v2, b2)) => {
            compute_edit_distance_impl(t1, t2, depth + 1, max_depth)
                + compute_edit_distance_impl(v1, v2, depth + 1, max_depth)
                + compute_edit_distance_impl(b1, b2, depth + 1, max_depth)
        }
        (Expr::Proj(_, _, e1i), Expr::Proj(_, _, e2i)) => {
            compute_edit_distance_impl(e1i, e2i, depth + 1, max_depth)
        }
        _ => 1,
    }
}
/// Build a human-readable tactic suggestion string.
pub(super) fn build_suggestion(name: &Name, args: &[Expr], is_exact: bool) -> String {
    let tactic = if is_exact { "exact" } else { "apply" };
    if args.is_empty() {
        format!("{} {}", tactic, name)
    } else {
        let arg_strs: Vec<String> = args.iter().map(format_expr_short).collect();
        format!("{} @{} {}", tactic, name, arg_strs.join(" "))
    }
}
/// Produce a short, one-line rendering of an expression (for suggestions).
pub(super) fn format_expr_short(expr: &Expr) -> String {
    match expr {
        Expr::Const(name, _) => format!("{}", name),
        Expr::BVar(n) => format!("?_{}", n),
        Expr::FVar(fid) => format!("fvar_{}", fid.0),
        Expr::Sort(Level::Zero) => "Prop".to_string(),
        Expr::Sort(_) => "Sort _".to_string(),
        Expr::Lit(lit) => format!("{}", lit),
        Expr::App(f, a) => {
            let fs = format_expr_short(f);
            let as_ = format_expr_short(a);
            format!("({} {})", fs, as_)
        }
        Expr::Lam(_, nm, _, _) => format!("(fun {} => ...)", nm),
        Expr::Pi(_, _, _, _) => "(_ -> _)".to_string(),
        Expr::Let(nm, _, _, _) => format!("(let {} := ...)", nm),
        Expr::Proj(nm, idx, _) => format!("{}.{}", nm, idx),
    }
}
/// Determine whether a declaration is a reasonable search candidate.
///
/// We skip constructors, recursors, and internal elaboration helpers.
pub(super) fn is_search_candidate(name: &Name, ci: &oxilean_kernel::ConstantInfo) -> bool {
    if ci.is_constructor() || ci.is_recursor() {
        return false;
    }
    let name_str = format!("{}", name);
    if name_str.starts_with('_') || name_str.contains("._") {
        return false;
    }
    if name_str.contains(".rec")
        || name_str.contains(".recOn")
        || name_str.contains(".casesOn")
        || name_str.contains(".brecOn")
        || name_str.contains(".below")
        || name_str.contains(".noConfusion")
        || name_str.contains(".noConfusionType")
        || name_str.contains(".sizeOf")
    {
        return false;
    }
    true
}
/// Add a single declaration to an existing `LemmaIndex`.
#[allow(dead_code)]
pub fn add_declaration_to_index(
    index: &mut LemmaIndex,
    name: Name,
    ty: Expr,
    num_univ_params: usize,
) {
    index.insert(name, ty, num_univ_params, false);
}
/// Remove all entries matching a predicate (rebuilds the tree).
#[allow(dead_code)]
pub fn filter_index(index: &LemmaIndex, pred: impl Fn(&Name) -> bool) -> LemmaIndex {
    let mut new_index = LemmaIndex::new();
    for entry in index.all_entries() {
        if pred(&entry.name) {
            new_index.insert(
                entry.name.clone(),
                entry.ty.clone(),
                entry.num_univ_params,
                entry.is_local,
            );
        }
    }
    new_index
}
/// Search for multiple goal types at once, sharing the same index.
#[allow(dead_code)]
pub fn batch_search(
    goals: &[Expr],
    index: &LemmaIndex,
    ctx: &mut MetaContext,
    config: &LibrarySearchConfig,
) -> Vec<SearchResult> {
    goals
        .iter()
        .map(|target| run_search(target, index, ctx, config).unwrap_or(SearchResult::NotFound))
        .collect()
}
/// Find all lemmas whose conclusion mentions a given constant name.
///
/// Returns candidates sorted by score descending, with specificity and
/// local-hypothesis bonuses computed from each entry.
#[allow(dead_code)]
pub fn find_lemmas_about(constant_name: &Name, index: &LemmaIndex) -> Vec<LemmaCandidate> {
    let target = Expr::Const(constant_name.clone(), vec![]);
    let config = LibrarySearchConfig::default();
    let entries = index.lookup(&target);
    let mut candidates: Vec<LemmaCandidate> = entries
        .into_iter()
        .map(|e| {
            LemmaCandidate::from_entry(
                e.name.clone(),
                e.ty.clone(),
                e.specificity,
                e.is_local,
                e.num_univ_params,
                &config,
            )
        })
        .collect();
    candidates.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    candidates
}
/// Hash an expression for cache lookup purposes.
pub fn hash_expr(expr: &Expr) -> u64 {
    let s = format!("{:?}", expr);
    let mut h: u64 = 14695981039346656037;
    for byte in s.bytes() {
        h ^= byte as u64;
        h = h.wrapping_mul(1099511628211);
    }
    h
}
/// Run a library search with caching support.
#[allow(dead_code)]
pub fn search_with_cache(
    target: &Expr,
    index: &LemmaIndex,
    ctx: &mut MetaContext,
    config: &LibrarySearchConfig,
    cache: &mut SearchCache,
) -> TacticResult<SearchResult> {
    let h = hash_expr(target);
    if let Some(cached) = cache.lookup(h) {
        return match cached {
            CacheLookup::Failed => Ok(SearchResult::NotFound),
            CacheLookup::Found(candidates) => {
                if candidates.is_empty() {
                    Ok(SearchResult::NotFound)
                } else {
                    Ok(SearchResult::MultipleFound(candidates))
                }
            }
        };
    }
    let result = run_search(target, index, ctx, config)?;
    match &result {
        SearchResult::NotFound | SearchResult::TimedOut => {
            cache.record_failure(h);
        }
        SearchResult::Found(_, _) => {}
        SearchResult::MultipleFound(cs) => {
            cache.record_success(h, cs.clone());
        }
    }
    Ok(result)
}
/// Run a priority-queue-based search (alternative to the BFS in `run_search`).
#[allow(dead_code)]
pub(super) fn run_priority_search(
    target: &Expr,
    index: &LemmaIndex,
    ctx: &mut MetaContext,
    config: &LibrarySearchConfig,
) -> TacticResult<SearchResult> {
    let mut search_state = SearchState::new(config.clone());
    let entries: Vec<LemmaEntry> = if config.use_discr_tree {
        index.lookup(target).into_iter().cloned().collect()
    } else {
        index.all_entries().into_iter().cloned().collect()
    };
    let mut pq: Vec<ScoredEntry> = entries
        .into_iter()
        .map(|e| {
            let priority = e.specificity;
            ScoredEntry {
                entry: e,
                priority,
                depth: 0,
            }
        })
        .collect();
    pq.sort();
    while let Some(scored) = pq.pop() {
        if search_state.is_budget_exhausted() {
            break;
        }
        if scored.depth > config.max_depth {
            continue;
        }
        if search_state.already_failed(&scored.entry.name) {
            continue;
        }
        search_state.candidates_tried += 1;
        let saved = ctx.save_state();
        match try_candidate(target, &scored.entry, ctx, config, scored.depth) {
            Ok(candidate) => {
                if candidate.remaining_goals == 0 {
                    if let Some(proof) = &candidate.proof {
                        ctx.restore_state(saved);
                        return Ok(SearchResult::Found(
                            proof.clone(),
                            candidate.suggestion.clone(),
                        ));
                    }
                }
                search_state.record_result(candidate);
                ctx.restore_state(saved);
            }
            Err(_) => {
                ctx.restore_state(saved);
                search_state.mark_failed(&scored.entry.name);
            }
        }
    }
    if search_state.results.is_empty() {
        if search_state.is_timed_out() {
            return Ok(SearchResult::TimedOut);
        }
        return Ok(SearchResult::NotFound);
    }
    Ok(SearchResult::MultipleFound(search_state.results))
}
/// Collect all constant names that appear in an expression.
#[allow(dead_code)]
pub(super) fn collect_constants(expr: &Expr) -> Vec<Name> {
    let mut names = Vec::new();
    collect_constants_impl(expr, &mut names);
    names
}
pub(super) fn collect_constants_impl(expr: &Expr, out: &mut Vec<Name>) {
    match expr {
        Expr::Const(name, _) => {
            if !out.contains(name) {
                out.push(name.clone());
            }
        }
        Expr::App(f, a) => {
            collect_constants_impl(f, out);
            collect_constants_impl(a, out);
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            collect_constants_impl(ty, out);
            collect_constants_impl(body, out);
        }
        Expr::Let(_, ty, val, body) => {
            collect_constants_impl(ty, out);
            collect_constants_impl(val, out);
            collect_constants_impl(body, out);
        }
        Expr::Proj(_, _, e) => {
            collect_constants_impl(e, out);
        }
        _ => {}
    }
}
/// Collect all free-variable IDs in an expression.
#[allow(dead_code)]
pub(super) fn collect_fvar_ids(expr: &Expr) -> Vec<u64> {
    let mut ids = Vec::new();
    collect_fvar_ids_impl(expr, &mut ids);
    ids
}
pub(super) fn collect_fvar_ids_impl(expr: &Expr, out: &mut Vec<u64>) {
    match expr {
        Expr::FVar(fid) => {
            if !out.contains(&fid.0) {
                out.push(fid.0);
            }
        }
        Expr::App(f, a) => {
            collect_fvar_ids_impl(f, out);
            collect_fvar_ids_impl(a, out);
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            collect_fvar_ids_impl(ty, out);
            collect_fvar_ids_impl(body, out);
        }
        Expr::Let(_, ty, val, body) => {
            collect_fvar_ids_impl(ty, out);
            collect_fvar_ids_impl(val, out);
            collect_fvar_ids_impl(body, out);
        }
        Expr::Proj(_, _, e) => {
            collect_fvar_ids_impl(e, out);
        }
        _ => {}
    }
}
/// Check whether `expr` is a proposition (its type is `Prop`).
#[allow(dead_code)]
pub(super) fn is_proposition_shaped(expr: &Expr) -> bool {
    let conclusion = strip_leading_pis(expr);
    match &conclusion {
        Expr::Sort(l) if *l == Level::zero() => true,
        Expr::Const(n, _) => {
            let s = format!("{}", n);
            matches!(
                s.as_str(),
                "True" | "False" | "Eq" | "And" | "Or" | "Not" | "Iff" | "Exists" | "HEq"
            )
        }
        Expr::App(head, _) => {
            let h = collect_head(head);
            if let Expr::Const(n, _) = h {
                let s = format!("{}", n);
                matches!(
                    s.as_str(),
                    "Eq" | "HEq"
                        | "And"
                        | "Or"
                        | "Not"
                        | "Iff"
                        | "Exists"
                        | "LE"
                        | "LT"
                        | "GE"
                        | "GT"
                        | "Dvd"
                        | "HasSubset"
                        | "Membership"
                )
            } else {
                false
            }
        }
        _ => false,
    }
}
/// Extract the head of a nested application chain.
pub(super) fn collect_head(expr: &Expr) -> &Expr {
    match expr {
        Expr::App(f, _) => collect_head(f),
        _ => expr,
    }
}
/// Decompose a goal type into head constant + arguments.
#[allow(dead_code)]
pub(super) fn decompose_goal(goal: &Expr) -> (Option<Name>, Vec<&Expr>) {
    let mut args = Vec::new();
    let mut current = goal;
    while let Expr::App(f, a) = current {
        args.push(a.as_ref());
        current = f;
    }
    args.reverse();
    let head = match current {
        Expr::Const(name, _) => Some(name.clone()),
        _ => None,
    };
    (head, args)
}
/// Attempt to match a candidate's conclusion against a goal, returning the
/// substitution map if successful.
#[allow(dead_code)]
pub(super) fn match_conclusion(
    conclusion: &Expr,
    goal: &Expr,
    ctx: &mut MetaContext,
) -> Option<HashMap<MVarId, Expr>> {
    let saved = ctx.save_state();
    let mut deq = MetaDefEq::new();
    let result = deq.is_def_eq(conclusion, goal, ctx);
    if result == UnificationResult::Equal {
        let unassigned_before: HashSet<MVarId> = HashSet::new();
        let mut assignments = HashMap::new();
        for mvar in ctx.unassigned_mvars() {
            if !unassigned_before.contains(&mvar) {
                if let Some(val) = ctx.get_mvar_assignment(mvar) {
                    assignments.insert(mvar, val.clone());
                }
            }
        }
        ctx.restore_state(saved);
        Some(assignments)
    } else {
        ctx.restore_state(saved);
        None
    }
}
/// Compute the size of an expression (number of AST nodes).
#[allow(dead_code)]
pub(super) fn expr_size(expr: &Expr) -> usize {
    match expr {
        Expr::Sort(_) | Expr::BVar(_) | Expr::FVar(_) | Expr::Const(_, _) | Expr::Lit(_) => 1,
        Expr::App(f, a) => 1 + expr_size(f) + expr_size(a),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => 1 + expr_size(ty) + expr_size(body),
        Expr::Let(_, ty, val, body) => 1 + expr_size(ty) + expr_size(val) + expr_size(body),
        Expr::Proj(_, _, e) => 1 + expr_size(e),
    }
}
/// Compute the depth of an expression tree.
#[allow(dead_code)]
pub(super) fn expr_depth(expr: &Expr) -> usize {
    match expr {
        Expr::Sort(_) | Expr::BVar(_) | Expr::FVar(_) | Expr::Const(_, _) | Expr::Lit(_) => 0,
        Expr::App(f, a) => 1 + expr_depth(f).max(expr_depth(a)),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            1 + expr_depth(ty).max(expr_depth(body))
        }
        Expr::Let(_, ty, val, body) => {
            1 + expr_depth(ty).max(expr_depth(val)).max(expr_depth(body))
        }
        Expr::Proj(_, _, e) => 1 + expr_depth(e),
    }
}
/// Extract the last component of a hierarchical name as a string.
#[allow(dead_code)]
pub(super) fn name_last_component(name: &Name) -> String {
    match name {
        Name::Anonymous => "_".to_string(),
        Name::Str(_, s) => s.clone(),
        Name::Num(_, n) => format!("{}", n),
    }
}
/// Check if a name is a sibling of another (same parent).
#[allow(dead_code)]
pub(super) fn names_are_siblings(a: &Name, b: &Name) -> bool {
    match (a, b) {
        (Name::Str(pa, _), Name::Str(pb, _)) => pa == pb,
        (Name::Num(pa, _), Name::Num(pb, _)) => pa == pb,
        _ => false,
    }
}
/// Get the parent name.
#[allow(dead_code)]
pub(super) fn name_parent(name: &Name) -> &Name {
    match name {
        Name::Str(parent, _) | Name::Num(parent, _) => parent,
        Name::Anonymous => &Name::Anonymous,
    }
}
/// Find the last top-level `->` in a type string (not inside parentheses).
pub(super) fn find_top_level_arrow(s: &str) -> Option<usize> {
    let bytes = s.as_bytes();
    let mut depth: i32 = 0;
    let mut last_arrow: Option<usize> = None;
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'(' => depth += 1,
            b')' => depth -= 1,
            b'-' if depth == 0 && i + 1 < bytes.len() && bytes[i + 1] == b'>' => {
                last_arrow = Some(i);
                i += 1;
            }
            _ => {}
        }
        i += 1;
    }
    last_arrow
}
/// Compute a match score between a query and a candidate signature.
///
/// Higher score = closer match.
pub(super) fn score_match(
    query: &TypeSignature,
    candidate: &TypeSignature,
    _exact_head: bool,
) -> f32 {
    if query.head == "_" || candidate.head == "_" {
        return 0.3;
    }
    if query.head != candidate.head {
        return 0.0;
    }
    let base = 0.5_f32;
    let arg_bonus: f32 = if query.args.is_empty() || candidate.args.is_empty() {
        0.0
    } else {
        let matched = query
            .args
            .iter()
            .zip(candidate.args.iter())
            .filter(|(a, b)| a.matches(b))
            .count() as f32;
        (matched / query.args.len().max(candidate.args.len()) as f32) * 0.5
    };
    base + arg_bonus
}
/// Return the conclusion (deepest RHS) of an implication chain.
pub(super) fn conclusion_of(sig: &TypeSignature) -> &TypeSignature {
    let mut cur = sig;
    while cur.head == "->" && cur.args.len() == 2 {
        cur = &cur.args[1];
    }
    cur
}
/// Heuristic library search using the type-directed search engine.
///
/// Given a goal type string, returns candidate lemma names from the prebuilt index.
/// This is the lightweight counterpart to the kernel-level `tac_library_search`.
pub fn run_library_search_advanced(goal_str: &str) -> Vec<String> {
    let engine = TypeDirectedSearch::with_prebuilt();
    let mut results = engine.search_for_goal(goal_str);
    let rw = engine.search_for_rewrite(goal_str);
    for r in rw {
        if !results.iter().any(|x| x.name == r.name) {
            results.push(r);
        }
    }
    let ranked = engine.rank_results(results);
    ranked.into_iter().map(|r| r.name).collect()
}
