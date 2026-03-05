//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    CastConfig, CastDirection, CastLemma, CastLemmaSet, CastResult, CastStats, CastStep,
    NormCastExtConfig1400, NormCastExtConfigVal1400, NormCastExtDiag1400, NormCastExtDiff1400,
    NormCastExtPass1400, NormCastExtPipeline1400, NormCastExtResult1400,
    TacticNormCastAnalysisPass, TacticNormCastConfig, TacticNormCastConfigValue,
    TacticNormCastDiagnostics, TacticNormCastDiff, TacticNormCastPipeline, TacticNormCastResult,
};
use crate::basic::MetaContext;
use crate::tactic::state::{TacticError, TacticResult, TacticState};
use oxilean_kernel::{Expr, Level, Name};
use std::collections::{HashMap, HashSet};

/// Maximum number of cast lemmas in a lemma set.
pub(super) const MAX_CAST_LEMMAS: usize = 512;
/// Maximum depth for cast chain search.
pub(super) const MAX_CAST_CHAIN_DEPTH: usize = 8;
/// Maximum number of rewrite steps for cast normalization.
pub(super) const MAX_CAST_STEPS: usize = 128;
/// Find a chain of casts from one type to another.
///
/// Uses BFS on the coercion graph to find the shortest path.
pub fn find_cast_chain(from: &Name, to: &Name, db: &CastLemmaSet) -> TacticResult<Vec<CastStep>> {
    if from == to {
        return Ok(Vec::new());
    }
    if !db.query(from, to, None).is_empty() {
        let lemma = &db.query(from, to, None)[0];
        return Ok(vec![CastStep {
            from: from.clone(),
            to: to.clone(),
            coercion: lemma.lemma_expr.clone(),
            proof: lemma.lemma_expr.clone(),
        }]);
    }
    if let Some(intermediates) = db.coercion_graph().get(&(from.clone(), to.clone())) {
        let mut chain = Vec::new();
        let mut current = from.clone();
        for intermediate in intermediates {
            let lemmas = db.query(&current, intermediate, None);
            if let Some(lemma) = lemmas.first() {
                chain.push(CastStep {
                    from: current.clone(),
                    to: intermediate.clone(),
                    coercion: lemma.lemma_expr.clone(),
                    proof: lemma.lemma_expr.clone(),
                });
            }
            current = intermediate.clone();
        }
        let lemmas = db.query(&current, to, None);
        if let Some(lemma) = lemmas.first() {
            chain.push(CastStep {
                from: current,
                to: to.clone(),
                coercion: lemma.lemma_expr.clone(),
                proof: lemma.lemma_expr.clone(),
            });
        }
        if !chain.is_empty() {
            return Ok(chain);
        }
    }
    let mut visited: HashSet<Name> = HashSet::new();
    let mut queue: Vec<(Name, Vec<CastStep>)> = Vec::new();
    visited.insert(from.clone());
    queue.push((from.clone(), Vec::new()));
    while let Some((current, path)) = queue.pop() {
        if path.len() >= MAX_CAST_CHAIN_DEPTH {
            continue;
        }
        for (f, t) in db.coercion_graph().keys() {
            if f == &current && !visited.contains(t) {
                let lemmas = db.query(f, t, None);
                if let Some(lemma) = lemmas.first() {
                    let mut new_path = path.clone();
                    new_path.push(CastStep {
                        from: f.clone(),
                        to: t.clone(),
                        coercion: lemma.lemma_expr.clone(),
                        proof: lemma.lemma_expr.clone(),
                    });
                    if t == to {
                        return Ok(new_path);
                    }
                    visited.insert(t.clone());
                    queue.push((t.clone(), new_path));
                }
            }
        }
    }
    Err(TacticError::Failed(format!(
        "norm_cast: no cast chain found from {} to {}",
        from, to
    )))
}
/// Collect application arguments.
pub(super) fn collect_app_args(expr: &Expr) -> (Expr, Vec<Expr>) {
    let mut args = Vec::new();
    let mut head = expr.clone();
    while let Expr::App(f, a) = head {
        args.push(*a);
        head = *f;
    }
    args.reverse();
    (head, args)
}
/// Rebuild an application from head and arguments.
pub(super) fn mk_app(head: Expr, args: Vec<Expr>) -> Expr {
    let mut result = head;
    for arg in args {
        result = Expr::App(Box::new(result), Box::new(arg));
    }
    result
}
/// Get the head constant name.
pub(super) fn get_head_const(expr: &Expr) -> Option<Name> {
    match expr {
        Expr::Const(name, _) => Some(name.clone()),
        Expr::App(f, _) => get_head_const(f),
        _ => None,
    }
}
/// Detect if an expression is a cast: `@Coe.coe α β inst x`.
/// Returns `(from_type, to_type, inner_expr)` if so.
pub(super) fn detect_cast(expr: &Expr) -> Option<(Name, Name, Expr)> {
    let (head, args) = collect_app_args(expr);
    if let Expr::Const(name, _) = &head {
        let name_str = name.to_string();
        if name_str.contains("Nat.cast") && args.len() >= 2 {
            let to_type = get_type_name(&args[0]).unwrap_or(Name::str("unknown"));
            let inner = args.last().cloned().unwrap_or(Expr::Sort(Level::Zero));
            return Some((Name::str("Nat"), to_type, inner));
        }
        if name_str.contains("Int.cast") && args.len() >= 2 {
            let to_type = get_type_name(&args[0]).unwrap_or(Name::str("unknown"));
            let inner = args.last().cloned().unwrap_or(Expr::Sort(Level::Zero));
            return Some((Name::str("Int"), to_type, inner));
        }
        if name_str.contains("Coe.coe") && args.len() >= 4 {
            let from_type = get_type_name(&args[0]).unwrap_or(Name::str("unknown"));
            let to_type = get_type_name(&args[1]).unwrap_or(Name::str("unknown"));
            let inner = args.last().cloned().unwrap_or(Expr::Sort(Level::Zero));
            return Some((from_type, to_type, inner));
        }
    }
    None
}
/// Try to extract a type name from a type expression.
pub(super) fn get_type_name(expr: &Expr) -> Option<Name> {
    match expr {
        Expr::Const(name, _) => Some(name.clone()),
        Expr::App(f, _) => get_type_name(f),
        _ => None,
    }
}
/// Check if an expression contains any casts.
pub(super) fn contains_cast(expr: &Expr) -> bool {
    if detect_cast(expr).is_some() {
        return true;
    }
    match expr {
        Expr::App(f, a) => contains_cast(f) || contains_cast(a),
        Expr::Lam(_, _, ty, body) => contains_cast(ty) || contains_cast(body),
        Expr::Pi(_, _, ty, body) => contains_cast(ty) || contains_cast(body),
        Expr::Let(_, ty, val, body) => {
            contains_cast(ty) || contains_cast(val) || contains_cast(body)
        }
        _ => false,
    }
}
/// Syntactic equality check.
pub(super) fn exprs_equal(a: &Expr, b: &Expr) -> bool {
    match (a, b) {
        (Expr::Sort(l1), Expr::Sort(l2)) => l1 == l2,
        (Expr::BVar(i1), Expr::BVar(i2)) => i1 == i2,
        (Expr::FVar(id1), Expr::FVar(id2)) => id1 == id2,
        (Expr::Const(n1, ls1), Expr::Const(n2, ls2)) => n1 == n2 && ls1 == ls2,
        (Expr::App(f1, a1), Expr::App(f2, a2)) => exprs_equal(f1, f2) && exprs_equal(a1, a2),
        (Expr::Lam(bi1, n1, t1, b1), Expr::Lam(bi2, n2, t2, b2)) => {
            bi1 == bi2 && n1 == n2 && exprs_equal(t1, t2) && exprs_equal(b1, b2)
        }
        (Expr::Pi(bi1, n1, t1, b1), Expr::Pi(bi2, n2, t2, b2)) => {
            bi1 == bi2 && n1 == n2 && exprs_equal(t1, t2) && exprs_equal(b1, b2)
        }
        (Expr::Lit(l1), Expr::Lit(l2)) => l1 == l2,
        _ => false,
    }
}
/// Register default cast lemmas for common numeric types.
pub(super) fn register_default_cast_lemmas(set: &mut CastLemmaSet) {
    set.add_lemma(
        CastLemma::new(
            Name::str("Nat.cast_add"),
            Name::str("Nat"),
            Name::str("Int"),
            Some(Name::str("HAdd.hAdd")),
            CastDirection::Push,
            Expr::Const(Name::str("Nat.cast_add"), vec![]),
            1000,
        )
        .builtin(),
    );
    set.add_lemma(
        CastLemma::new(
            Name::str("Nat.cast_mul"),
            Name::str("Nat"),
            Name::str("Int"),
            Some(Name::str("HMul.hMul")),
            CastDirection::Push,
            Expr::Const(Name::str("Nat.cast_mul"), vec![]),
            1000,
        )
        .builtin(),
    );
    set.add_lemma(
        CastLemma::new(
            Name::str("Nat.cast_zero"),
            Name::str("Nat"),
            Name::str("Int"),
            None,
            CastDirection::Push,
            Expr::Const(Name::str("Nat.cast_zero"), vec![]),
            500,
        )
        .builtin(),
    );
    set.add_lemma(
        CastLemma::new(
            Name::str("Nat.cast_one"),
            Name::str("Nat"),
            Name::str("Int"),
            None,
            CastDirection::Push,
            Expr::Const(Name::str("Nat.cast_one"), vec![]),
            500,
        )
        .builtin(),
    );
    set.add_lemma(
        CastLemma::new(
            Name::str("Nat.cast_pow"),
            Name::str("Nat"),
            Name::str("Int"),
            Some(Name::str("HPow.hPow")),
            CastDirection::Push,
            Expr::Const(Name::str("Nat.cast_pow"), vec![]),
            1000,
        )
        .builtin(),
    );
    set.add_lemma(
        CastLemma::new(
            Name::str("Int.cast_add"),
            Name::str("Int"),
            Name::str("Rat"),
            Some(Name::str("HAdd.hAdd")),
            CastDirection::Push,
            Expr::Const(Name::str("Int.cast_add"), vec![]),
            1000,
        )
        .builtin(),
    );
    set.add_lemma(
        CastLemma::new(
            Name::str("Int.cast_mul"),
            Name::str("Int"),
            Name::str("Rat"),
            Some(Name::str("HMul.hMul")),
            CastDirection::Push,
            Expr::Const(Name::str("Int.cast_mul"), vec![]),
            1000,
        )
        .builtin(),
    );
    set.add_lemma(
        CastLemma::new(
            Name::str("Int.cast_neg"),
            Name::str("Int"),
            Name::str("Rat"),
            Some(Name::str("Neg.neg")),
            CastDirection::Push,
            Expr::Const(Name::str("Int.cast_neg"), vec![]),
            1000,
        )
        .builtin(),
    );
    set.add_lemma(
        CastLemma::new(
            Name::str("Int.cast_sub"),
            Name::str("Int"),
            Name::str("Rat"),
            Some(Name::str("HSub.hSub")),
            CastDirection::Push,
            Expr::Const(Name::str("Int.cast_sub"), vec![]),
            1000,
        )
        .builtin(),
    );
    set.add_lemma(
        CastLemma::new(
            Name::str("Nat.cast_add_rat"),
            Name::str("Nat"),
            Name::str("Rat"),
            Some(Name::str("HAdd.hAdd")),
            CastDirection::Push,
            Expr::Const(Name::str("Nat.cast_add"), vec![]),
            1100,
        )
        .builtin(),
    );
    set.add_coercion_path(Name::str("Nat"), Name::str("Rat"), vec![Name::str("Int")]);
}
/// Normalize casts in a push direction: push casts toward leaves.
///
/// Transforms `↑(a + b)` into `↑a + ↑b`.
pub(super) fn push_cast_step(
    expr: &Expr,
    db: &CastLemmaSet,
    _ctx: &MetaContext,
) -> Option<(Expr, Expr)> {
    if let Some((from, to, inner)) = detect_cast(expr) {
        if let Some(op_name) = get_head_const(&inner) {
            let lemmas = db.query(&from, &to, Some(&op_name));
            if let Some(lemma) = lemmas.iter().find(|l| l.direction == CastDirection::Push) {
                let (_head, args) = collect_app_args(&inner);
                let cast_args: Vec<Expr> = args
                    .into_iter()
                    .map(|a| build_cast(&from, &to, a))
                    .collect();
                let new_expr = mk_app(Expr::Const(op_name, vec![]), cast_args);
                return Some((new_expr, lemma.lemma_expr.clone()));
            }
        }
    }
    None
}
/// Normalize casts in a pull direction: pull casts toward root.
///
/// Transforms `↑a + ↑b` into `↑(a + b)`.
pub(super) fn pull_cast_step(
    expr: &Expr,
    db: &CastLemmaSet,
    _ctx: &MetaContext,
) -> Option<(Expr, Expr)> {
    let (head, args) = collect_app_args(expr);
    if let Expr::Const(op_name, _) = &head {
        let mut from_type: Option<Name> = None;
        let mut to_type: Option<Name> = None;
        let mut inner_args: Vec<Expr> = Vec::new();
        let mut all_cast = true;
        for arg in &args {
            if let Some((from, to, inner)) = detect_cast(arg) {
                if let Some(ref ft) = from_type {
                    if *ft != from {
                        all_cast = false;
                        break;
                    }
                } else {
                    from_type = Some(from);
                    to_type = Some(to);
                }
                inner_args.push(inner);
            } else {
                all_cast = false;
                break;
            }
        }
        if all_cast {
            if let (Some(from), Some(to)) = (from_type, to_type) {
                let lemmas = db.query(&from, &to, Some(op_name));
                if let Some(lemma) = lemmas.iter().find(|l| l.direction == CastDirection::Push) {
                    let inner_expr = mk_app(Expr::Const(op_name.clone(), vec![]), inner_args);
                    let new_expr = build_cast(&from, &to, inner_expr);
                    return Some((new_expr, lemma.lemma_expr.clone()));
                }
            }
        }
    }
    None
}
/// Squash adjacent casts: `↑(↑a)` -> `↑a`.
pub(super) fn squash_cast_step(
    expr: &Expr,
    db: &CastLemmaSet,
    _ctx: &MetaContext,
) -> Option<(Expr, Expr)> {
    if let Some((_from_outer, to_outer, inner)) = detect_cast(expr) {
        if let Some((from_inner, _to_inner, innermost)) = detect_cast(&inner) {
            let chain = find_cast_chain(&from_inner, &to_outer, db).ok()?;
            if !chain.is_empty() {
                let direct_cast = build_cast(&from_inner, &to_outer, innermost);
                let proof = Expr::Const(Name::str("_cast_squash"), vec![]);
                return Some((direct_cast, proof));
            }
        }
    }
    None
}
/// Build a cast expression: `↑(x : from) : to`.
pub(super) fn build_cast(from: &Name, to: &Name, inner: Expr) -> Expr {
    let cast_name = if from == &Name::str("Nat") {
        Name::str("Nat.cast")
    } else if from == &Name::str("Int") {
        Name::str("Int.cast")
    } else {
        Name::str("Coe.coe")
    };
    let cast_const = Expr::Const(cast_name, vec![]);
    let to_type = Expr::Const(to.clone(), vec![]);
    Expr::App(
        Box::new(Expr::App(Box::new(cast_const), Box::new(to_type))),
        Box::new(inner),
    )
}
/// Push casts toward leaves.
pub fn tac_push_cast(state: &mut TacticState, ctx: &mut MetaContext) -> TacticResult<CastResult> {
    let config = CastConfig::default();
    tac_push_cast_with_config(&config, state, ctx)
}
/// Push casts with configuration.
pub fn tac_push_cast_with_config(
    config: &CastConfig,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<CastResult> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let target = ctx.instantiate_mvars(&target);
    let db = build_lemma_set(config);
    let mut current = target.clone();
    let mut steps = 0;
    let mut stats = CastStats::default();
    let mut proofs: Vec<Expr> = Vec::new();
    loop {
        if steps >= config.max_steps {
            break;
        }
        if let Some((new_expr, proof)) = push_cast_step(&current, &db, ctx) {
            proofs.push(proof);
            current = new_expr;
            steps += 1;
            stats.push_steps += 1;
            stats.lemmas_applied += 1;
        } else {
            break;
        }
    }
    if steps == 0 {
        return Err(TacticError::Failed(
            "push_cast: no casts to push".to_string(),
        ));
    }
    let proof = combine_cast_proofs(&proofs);
    state.close_goal(proof.clone(), ctx)?;
    Ok(CastResult {
        success: true,
        normalized: current,
        proof,
        num_steps: steps,
        stats,
    })
}
/// Pull casts toward root.
pub fn tac_pull_cast(state: &mut TacticState, ctx: &mut MetaContext) -> TacticResult<CastResult> {
    let config = CastConfig::default();
    tac_pull_cast_with_config(&config, state, ctx)
}
/// Pull casts with configuration.
pub fn tac_pull_cast_with_config(
    config: &CastConfig,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<CastResult> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let target = ctx.instantiate_mvars(&target);
    let db = build_lemma_set(config);
    let mut current = target.clone();
    let mut steps = 0;
    let mut stats = CastStats::default();
    let mut proofs: Vec<Expr> = Vec::new();
    loop {
        if steps >= config.max_steps {
            break;
        }
        if let Some((new_expr, proof)) = pull_cast_step(&current, &db, ctx) {
            proofs.push(proof);
            current = new_expr;
            steps += 1;
            stats.pull_steps += 1;
            stats.lemmas_applied += 1;
        } else {
            break;
        }
    }
    if steps == 0 {
        return Err(TacticError::Failed(
            "pull_cast: no casts to pull".to_string(),
        ));
    }
    let proof = combine_cast_proofs(&proofs);
    state.close_goal(proof.clone(), ctx)?;
    Ok(CastResult {
        success: true,
        normalized: current,
        proof,
        num_steps: steps,
        stats,
    })
}
/// Normalize casts (apply push, pull, and squash as appropriate).
pub fn tac_norm_cast(state: &mut TacticState, ctx: &mut MetaContext) -> TacticResult<CastResult> {
    let config = CastConfig::default();
    tac_norm_cast_with_config(&config, state, ctx)
}
/// Normalize casts with configuration.
pub fn tac_norm_cast_with_config(
    config: &CastConfig,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<CastResult> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let target = ctx.instantiate_mvars(&target);
    let db = build_lemma_set(config);
    let mut current = target.clone();
    let mut steps = 0;
    let mut stats = CastStats::default();
    let mut proofs: Vec<Expr> = Vec::new();
    let mut changed = true;
    while changed && steps < config.max_steps {
        changed = false;
        if let Some((new_expr, proof)) = squash_cast_step(&current, &db, ctx) {
            proofs.push(proof);
            current = new_expr;
            steps += 1;
            stats.squash_steps += 1;
            stats.lemmas_applied += 1;
            changed = true;
            continue;
        }
        if let Some((new_expr, proof)) = push_cast_step(&current, &db, ctx) {
            proofs.push(proof);
            current = new_expr;
            steps += 1;
            stats.push_steps += 1;
            stats.lemmas_applied += 1;
            changed = true;
            continue;
        }
    }
    if steps == 0 {
        return Err(TacticError::Failed(
            "norm_cast: no casts to normalize".to_string(),
        ));
    }
    let proof = combine_cast_proofs(&proofs);
    state.close_goal(proof.clone(), ctx)?;
    Ok(CastResult {
        success: true,
        normalized: current,
        proof,
        num_steps: steps,
        stats,
    })
}
/// Close the current goal if it holds modulo cast normalization.
///
/// Takes a proof term, normalizes casts in the goal target, and builds a
/// composed proof that applies cast normalization steps before `proof_term`.
pub fn tac_exact_mod_cast(
    proof_term: &Expr,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<()> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let target = ctx.instantiate_mvars(&target);
    let config = CastConfig::default();
    let db = build_lemma_set(&config);
    let mut current = target.clone();
    let mut cast_proofs: Vec<Expr> = Vec::new();
    let mut steps = 0;
    let mut changed = true;
    while changed && steps < config.max_steps {
        changed = false;
        if let Some((new_expr, proof)) = squash_cast_step(&current, &db, ctx) {
            cast_proofs.push(proof);
            current = new_expr;
            steps += 1;
            changed = true;
        } else if let Some((new_expr, proof)) = push_cast_step(&current, &db, ctx) {
            cast_proofs.push(proof);
            current = new_expr;
            steps += 1;
            changed = true;
        }
    }
    let final_proof = if cast_proofs.is_empty() {
        proof_term.clone()
    } else {
        let cast_norm = combine_cast_proofs(&cast_proofs);
        Expr::App(Box::new(cast_norm), Box::new(proof_term.clone()))
    };
    state.close_goal(final_proof, ctx)?;
    Ok(())
}
/// Build a lemma set from configuration.
pub(super) fn build_lemma_set(config: &CastConfig) -> CastLemmaSet {
    let mut db = if config.use_defaults {
        CastLemmaSet::with_defaults()
    } else {
        CastLemmaSet::new()
    };
    for lemma in &config.extra_lemmas {
        db.add_lemma(lemma.clone());
    }
    db
}
/// Combine multiple proof steps into a single proof.
pub(super) fn combine_cast_proofs(proofs: &[Expr]) -> Expr {
    if proofs.is_empty() {
        return Expr::Const(Name::str("rfl"), vec![]);
    }
    if proofs.len() == 1 {
        return proofs[0].clone();
    }
    let mut combined = proofs[0].clone();
    for proof in &proofs[1..] {
        let trans = Expr::Const(Name::str("Eq.trans"), vec![]);
        combined = Expr::App(
            Box::new(Expr::App(Box::new(trans), Box::new(combined))),
            Box::new(proof.clone()),
        );
    }
    combined
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tactic::norm_cast::*;
    use oxilean_kernel::Environment;
    fn mk_test_ctx() -> MetaContext {
        MetaContext::new(Environment::new())
    }
    fn mk_const(name: &str) -> Expr {
        Expr::Const(Name::str(name), vec![])
    }
    fn mk_app2(f: Expr, a: Expr, b: Expr) -> Expr {
        Expr::App(Box::new(Expr::App(Box::new(f), Box::new(a))), Box::new(b))
    }
    #[test]
    fn test_cast_direction_display() {
        assert_eq!(format!("{}", CastDirection::Push), "push");
        assert_eq!(format!("{}", CastDirection::Pull), "pull");
        assert_eq!(format!("{}", CastDirection::Squash), "squash");
    }
    #[test]
    fn test_cast_direction_eq() {
        assert_eq!(CastDirection::Push, CastDirection::Push);
        assert_ne!(CastDirection::Push, CastDirection::Pull);
    }
    #[test]
    fn test_cast_lemma_creation() {
        let lemma = CastLemma::new(
            Name::str("test"),
            Name::str("Nat"),
            Name::str("Int"),
            Some(Name::str("HAdd.hAdd")),
            CastDirection::Push,
            mk_const("proof"),
            1000,
        );
        assert_eq!(lemma.from_type, Name::str("Nat"));
        assert_eq!(lemma.to_type, Name::str("Int"));
        assert_eq!(lemma.priority, 1000);
        assert!(!lemma.is_builtin);
    }
    #[test]
    fn test_cast_lemma_builtin() {
        let lemma = CastLemma::new(
            Name::str("test"),
            Name::str("Nat"),
            Name::str("Int"),
            None,
            CastDirection::Push,
            mk_const("proof"),
            1000,
        )
        .builtin();
        assert!(lemma.is_builtin);
    }
    #[test]
    fn test_cast_lemma_applies_to() {
        let lemma = CastLemma::new(
            Name::str("Nat.cast_add"),
            Name::str("Nat"),
            Name::str("Int"),
            Some(Name::str("HAdd.hAdd")),
            CastDirection::Push,
            mk_const("proof"),
            1000,
        );
        assert!(lemma.applies_to(
            &Name::str("Nat"),
            &Name::str("Int"),
            Some(&Name::str("HAdd.hAdd"))
        ));
        assert!(!lemma.applies_to(
            &Name::str("Int"),
            &Name::str("Rat"),
            Some(&Name::str("HAdd.hAdd"))
        ));
        assert!(!lemma.applies_to(
            &Name::str("Nat"),
            &Name::str("Int"),
            Some(&Name::str("HMul.hMul"))
        ));
    }
    #[test]
    fn test_cast_lemma_no_op_applies_to_any() {
        let lemma = CastLemma::new(
            Name::str("cast_zero"),
            Name::str("Nat"),
            Name::str("Int"),
            None,
            CastDirection::Push,
            mk_const("proof"),
            1000,
        );
        assert!(lemma.applies_to(
            &Name::str("Nat"),
            &Name::str("Int"),
            Some(&Name::str("anything"))
        ));
        assert!(lemma.applies_to(&Name::str("Nat"), &Name::str("Int"), None));
    }
    #[test]
    fn test_cast_lemma_type_pair() {
        let lemma = CastLemma::new(
            Name::str("test"),
            Name::str("Nat"),
            Name::str("Int"),
            None,
            CastDirection::Push,
            mk_const("proof"),
            1000,
        );
        let (from, to) = lemma.type_pair();
        assert_eq!(from, Name::str("Nat"));
        assert_eq!(to, Name::str("Int"));
    }
    #[test]
    fn test_cast_lemma_set_empty() {
        let set = CastLemmaSet::new();
        assert!(set.is_empty());
        assert_eq!(set.len(), 0);
    }
    #[test]
    fn test_cast_lemma_set_add_and_query() {
        let mut set = CastLemmaSet::new();
        set.add_lemma(CastLemma::new(
            Name::str("Nat.cast_add"),
            Name::str("Nat"),
            Name::str("Int"),
            Some(Name::str("HAdd.hAdd")),
            CastDirection::Push,
            mk_const("proof"),
            1000,
        ));
        let results = set.query(
            &Name::str("Nat"),
            &Name::str("Int"),
            Some(&Name::str("HAdd.hAdd")),
        );
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_cast_lemma_set_query_no_match() {
        let mut set = CastLemmaSet::new();
        set.add_lemma(CastLemma::new(
            Name::str("Nat.cast_add"),
            Name::str("Nat"),
            Name::str("Int"),
            Some(Name::str("HAdd.hAdd")),
            CastDirection::Push,
            mk_const("proof"),
            1000,
        ));
        let results = set.query(
            &Name::str("Int"),
            &Name::str("Rat"),
            Some(&Name::str("HAdd.hAdd")),
        );
        assert!(results.is_empty());
    }
    #[test]
    fn test_cast_lemma_set_with_defaults() {
        let set = CastLemmaSet::with_defaults();
        assert!(!set.is_empty());
        assert!(set.len() >= 5);
    }
    #[test]
    fn test_cast_lemma_set_query_by_direction() {
        let set = CastLemmaSet::with_defaults();
        let push_lemmas = set.query_by_direction(&CastDirection::Push);
        assert!(!push_lemmas.is_empty());
    }
    #[test]
    fn test_cast_lemma_set_query_by_operation() {
        let set = CastLemmaSet::with_defaults();
        let add_lemmas = set.query_by_operation(&Name::str("HAdd.hAdd"));
        assert!(!add_lemmas.is_empty());
    }
    #[test]
    fn test_cast_lemma_set_coercion_graph() {
        let set = CastLemmaSet::with_defaults();
        let graph = set.coercion_graph();
        let key = (Name::str("Nat"), Name::str("Rat"));
        assert!(graph.contains_key(&key));
    }
    #[test]
    fn test_cast_config_default() {
        let config = CastConfig::default();
        assert!(config.use_defaults);
        assert!(config.simp_after);
        assert_eq!(config.max_steps, MAX_CAST_STEPS);
    }
    #[test]
    fn test_cast_config_minimal() {
        let config = CastConfig::minimal();
        assert!(!config.use_defaults);
        assert!(!config.simp_after);
    }
    #[test]
    fn test_cast_stats_default() {
        let stats = CastStats::default();
        assert_eq!(stats.push_steps, 0);
        assert_eq!(stats.pull_steps, 0);
        assert_eq!(stats.squash_steps, 0);
        assert_eq!(stats.lemmas_applied, 0);
    }
    #[test]
    fn test_detect_cast_nat() {
        let cast = mk_app2(mk_const("Nat.cast"), mk_const("Int"), mk_const("x"));
        let result = detect_cast(&cast);
        assert!(result.is_some());
        let (from, to, _inner) = result.expect("result should be valid");
        assert_eq!(from, Name::str("Nat"));
        assert_eq!(to, Name::str("Int"));
    }
    #[test]
    fn test_detect_cast_int() {
        let cast = mk_app2(mk_const("Int.cast"), mk_const("Rat"), mk_const("x"));
        let result = detect_cast(&cast);
        assert!(result.is_some());
        let (from, _, _) = result.expect("result should be valid");
        assert_eq!(from, Name::str("Int"));
    }
    #[test]
    fn test_detect_cast_non_cast() {
        let expr = mk_const("not_a_cast");
        assert!(detect_cast(&expr).is_none());
    }
    #[test]
    fn test_contains_cast_simple() {
        let cast = mk_app2(mk_const("Nat.cast"), mk_const("Int"), mk_const("x"));
        assert!(contains_cast(&cast));
    }
    #[test]
    fn test_contains_cast_nested() {
        let cast = mk_app2(mk_const("Nat.cast"), mk_const("Int"), mk_const("x"));
        let add = mk_app2(mk_const("HAdd.hAdd"), cast, mk_const("y"));
        assert!(contains_cast(&add));
    }
    #[test]
    fn test_contains_cast_no_cast() {
        let expr = mk_app2(mk_const("HAdd.hAdd"), mk_const("a"), mk_const("b"));
        assert!(!contains_cast(&expr));
    }
    #[test]
    fn test_find_cast_chain_same_type() {
        let db = CastLemmaSet::with_defaults();
        let chain = find_cast_chain(&Name::str("Nat"), &Name::str("Nat"), &db)
            .expect("chain should be present");
        assert!(chain.is_empty());
    }
    #[test]
    fn test_find_cast_chain_direct() {
        let db = CastLemmaSet::with_defaults();
        let chain = find_cast_chain(&Name::str("Nat"), &Name::str("Int"), &db)
            .expect("chain should be present");
        assert_eq!(chain.len(), 1);
        assert_eq!(chain[0].from, Name::str("Nat"));
        assert_eq!(chain[0].to, Name::str("Int"));
    }
    #[test]
    fn test_find_cast_chain_via_intermediate() {
        let db = CastLemmaSet::with_defaults();
        let chain = find_cast_chain(&Name::str("Nat"), &Name::str("Rat"), &db)
            .expect("chain should be present");
        assert!(!chain.is_empty());
    }
    #[test]
    fn test_find_cast_chain_no_path() {
        let db = CastLemmaSet::new();
        let result = find_cast_chain(&Name::str("Nat"), &Name::str("Float"), &db);
        assert!(result.is_err());
    }
    #[test]
    fn test_build_cast_nat() {
        let inner = mk_const("x");
        let cast = build_cast(&Name::str("Nat"), &Name::str("Int"), inner);
        assert!(matches!(cast, Expr::App(_, _)));
    }
    #[test]
    fn test_build_cast_int() {
        let inner = mk_const("x");
        let cast = build_cast(&Name::str("Int"), &Name::str("Rat"), inner);
        assert!(matches!(cast, Expr::App(_, _)));
    }
    #[test]
    fn test_combine_cast_proofs_empty() {
        let proof = combine_cast_proofs(&[]);
        assert!(matches!(proof, Expr::Const(_, _)));
    }
    #[test]
    fn test_combine_cast_proofs_single() {
        let p = mk_const("h1");
        let proof = combine_cast_proofs(std::slice::from_ref(&p));
        assert!(exprs_equal(&proof, &p));
    }
    #[test]
    fn test_combine_cast_proofs_multiple() {
        let p1 = mk_const("h1");
        let p2 = mk_const("h2");
        let proof = combine_cast_proofs(&[p1, p2]);
        assert!(matches!(proof, Expr::App(_, _)));
    }
    #[test]
    fn test_get_type_name_const() {
        let nat = mk_const("Nat");
        assert_eq!(get_type_name(&nat), Some(Name::str("Nat")));
    }
    #[test]
    fn test_get_type_name_app() {
        let list_nat = Expr::App(Box::new(mk_const("List")), Box::new(mk_const("Nat")));
        assert_eq!(get_type_name(&list_nat), Some(Name::str("List")));
    }
    #[test]
    fn test_get_type_name_bvar() {
        assert!(get_type_name(&Expr::BVar(0)).is_none());
    }
    #[test]
    fn test_exprs_equal_same() {
        let a = mk_const("x");
        assert!(exprs_equal(&a, &a));
    }
    #[test]
    fn test_exprs_equal_different() {
        let a = mk_const("x");
        let b = mk_const("y");
        assert!(!exprs_equal(&a, &b));
    }
    #[test]
    fn test_cast_step_fields() {
        let step = CastStep {
            from: Name::str("Nat"),
            to: Name::str("Int"),
            coercion: mk_const("Nat.cast"),
            proof: mk_const("proof"),
        };
        assert_eq!(step.from, Name::str("Nat"));
        assert_eq!(step.to, Name::str("Int"));
    }
    #[test]
    fn test_cast_result_fields() {
        let result = CastResult {
            success: true,
            normalized: mk_const("result"),
            proof: mk_const("proof"),
            num_steps: 3,
            stats: CastStats::default(),
        };
        assert!(result.success);
        assert_eq!(result.num_steps, 3);
    }
    #[test]
    fn test_lemma_set_priority_ordering() {
        let mut set = CastLemmaSet::new();
        set.add_lemma(CastLemma::new(
            Name::str("low"),
            Name::str("Nat"),
            Name::str("Int"),
            Some(Name::str("HAdd.hAdd")),
            CastDirection::Push,
            mk_const("proof_low"),
            5000,
        ));
        set.add_lemma(CastLemma::new(
            Name::str("high"),
            Name::str("Nat"),
            Name::str("Int"),
            Some(Name::str("HAdd.hAdd")),
            CastDirection::Push,
            mk_const("proof_high"),
            100,
        ));
        let results = set.query(
            &Name::str("Nat"),
            &Name::str("Int"),
            Some(&Name::str("HAdd.hAdd")),
        );
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].name, Name::str("high"));
        assert_eq!(results[1].name, Name::str("low"));
    }
    #[test]
    fn test_build_lemma_set_with_config() {
        let config = CastConfig::default();
        let db = build_lemma_set(&config);
        assert!(!db.is_empty());
    }
    #[test]
    fn test_build_lemma_set_minimal() {
        let config = CastConfig::minimal();
        let db = build_lemma_set(&config);
        assert!(db.is_empty());
    }
    #[test]
    fn test_build_lemma_set_with_extras() {
        let extra = CastLemma::new(
            Name::str("custom"),
            Name::str("MyType"),
            Name::str("OtherType"),
            None,
            CastDirection::Push,
            mk_const("custom_proof"),
            1000,
        );
        let config = CastConfig::minimal().with_extra_lemmas(vec![extra]);
        let db = build_lemma_set(&config);
        assert_eq!(db.len(), 1);
    }
    #[test]
    fn test_collect_app_args_roundtrip() {
        let f = mk_const("f");
        let a = mk_const("a");
        let b = mk_const("b");
        let expr = mk_app2(f.clone(), a.clone(), b.clone());
        let (head, args) = collect_app_args(&expr);
        let rebuilt = mk_app(head, args);
        assert!(exprs_equal(&expr, &rebuilt));
    }
}
#[cfg(test)]
mod tacticnormcast_analysis_tests {
    use super::*;
    use crate::tactic::norm_cast::*;
    #[test]
    fn test_tacticnormcast_result_ok() {
        let r = TacticNormCastResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticnormcast_result_err() {
        let r = TacticNormCastResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticnormcast_result_partial() {
        let r = TacticNormCastResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_tacticnormcast_result_skipped() {
        let r = TacticNormCastResult::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_tacticnormcast_analysis_pass_run() {
        let mut p = TacticNormCastAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_tacticnormcast_analysis_pass_empty_input() {
        let mut p = TacticNormCastAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_tacticnormcast_analysis_pass_success_rate() {
        let mut p = TacticNormCastAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_tacticnormcast_analysis_pass_disable() {
        let mut p = TacticNormCastAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_tacticnormcast_pipeline_basic() {
        let mut pipeline = TacticNormCastPipeline::new("main_pipeline");
        pipeline.add_pass(TacticNormCastAnalysisPass::new("pass1"));
        pipeline.add_pass(TacticNormCastAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_tacticnormcast_pipeline_disabled_pass() {
        let mut pipeline = TacticNormCastPipeline::new("partial");
        let mut p = TacticNormCastAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(TacticNormCastAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_tacticnormcast_diff_basic() {
        let mut d = TacticNormCastDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_tacticnormcast_diff_summary() {
        let mut d = TacticNormCastDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }
    #[test]
    fn test_tacticnormcast_config_set_get() {
        let mut cfg = TacticNormCastConfig::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_tacticnormcast_config_read_only() {
        let mut cfg = TacticNormCastConfig::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_tacticnormcast_config_remove() {
        let mut cfg = TacticNormCastConfig::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_tacticnormcast_diagnostics_basic() {
        let mut diag = TacticNormCastDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_tacticnormcast_diagnostics_max_errors() {
        let mut diag = TacticNormCastDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_tacticnormcast_diagnostics_clear() {
        let mut diag = TacticNormCastDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_tacticnormcast_config_value_types() {
        let b = TacticNormCastConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = TacticNormCastConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = TacticNormCastConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = TacticNormCastConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = TacticNormCastConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
#[cfg(test)]
mod norm_cast_ext_tests_1400 {
    use super::*;
    use crate::tactic::norm_cast::*;
    #[test]
    fn test_norm_cast_ext_result_ok_1400() {
        let r = NormCastExtResult1400::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_norm_cast_ext_result_err_1400() {
        let r = NormCastExtResult1400::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_norm_cast_ext_result_partial_1400() {
        let r = NormCastExtResult1400::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_norm_cast_ext_result_skipped_1400() {
        let r = NormCastExtResult1400::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_norm_cast_ext_pass_run_1400() {
        let mut p = NormCastExtPass1400::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_norm_cast_ext_pass_empty_1400() {
        let mut p = NormCastExtPass1400::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_norm_cast_ext_pass_rate_1400() {
        let mut p = NormCastExtPass1400::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_norm_cast_ext_pass_disable_1400() {
        let mut p = NormCastExtPass1400::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_norm_cast_ext_pipeline_basic_1400() {
        let mut pipeline = NormCastExtPipeline1400::new("main_pipeline");
        pipeline.add_pass(NormCastExtPass1400::new("pass1"));
        pipeline.add_pass(NormCastExtPass1400::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_norm_cast_ext_pipeline_disabled_1400() {
        let mut pipeline = NormCastExtPipeline1400::new("partial");
        let mut p = NormCastExtPass1400::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(NormCastExtPass1400::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_norm_cast_ext_diff_basic_1400() {
        let mut d = NormCastExtDiff1400::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_norm_cast_ext_config_set_get_1400() {
        let mut cfg = NormCastExtConfig1400::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_norm_cast_ext_config_read_only_1400() {
        let mut cfg = NormCastExtConfig1400::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_norm_cast_ext_config_remove_1400() {
        let mut cfg = NormCastExtConfig1400::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_norm_cast_ext_diagnostics_basic_1400() {
        let mut diag = NormCastExtDiag1400::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_norm_cast_ext_diagnostics_max_errors_1400() {
        let mut diag = NormCastExtDiag1400::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_norm_cast_ext_diagnostics_clear_1400() {
        let mut diag = NormCastExtDiag1400::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_norm_cast_ext_config_value_types_1400() {
        let b = NormCastExtConfigVal1400::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = NormCastExtConfigVal1400::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = NormCastExtConfigVal1400::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = NormCastExtConfigVal1400::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = NormCastExtConfigVal1400::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
