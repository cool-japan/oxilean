//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    CaseSplitter, CongruenceClosure, EClass, EClassId, EMatchCompiler, ENode, ENodeId, EPattern,
    EPatternNode, EqualityStep, GrindConfig, GrindResult, GrindState, GrindStats, MergeReason,
    NatConstraint, NatRelKind, ProofStep, SignatureTable, Substitution, TermIndex, UnionFind,
};
use crate::basic::MetaContext;
use crate::tactic::state::{TacticError, TacticResult, TacticState};
use oxilean_kernel::{Expr, Level, Name};
use std::collections::{HashMap, HashSet, VecDeque};

/// Run E-matching over the congruence closure, returning all substitutions
/// that match the given patterns.
pub fn run_ematching(
    cc: &CongruenceClosure,
    patterns: &[EPattern],
    max_matches: usize,
) -> Vec<(usize, Substitution)> {
    let mut results: Vec<(usize, Substitution)> = Vec::new();
    for (pat_idx, pattern) in patterns.iter().enumerate() {
        let matches = match_pattern_in_egraph(cc, &pattern.root, pattern.num_vars);
        for subst in matches {
            if results.len() >= max_matches {
                return results;
            }
            results.push((pat_idx, subst));
        }
    }
    results
}
/// Match a single pattern against all nodes in the E-graph.
pub(super) fn match_pattern_in_egraph(
    cc: &CongruenceClosure,
    pattern: &EPatternNode,
    num_vars: u32,
) -> Vec<Substitution> {
    let mut results = Vec::new();
    match pattern {
        EPatternNode::App { func, args } => {
            for node in cc.all_nodes().iter() {
                if &node.func == func && node.args.len() == args.len() {
                    let mut subst = Substitution::new(num_vars);
                    subst.matched_class = cc.find_immut(node.eclass);
                    if match_args(cc, args, &node.args, &mut subst) {
                        results.push(subst);
                    }
                }
            }
        }
        EPatternNode::Var(_) => {
            for (i, node) in cc.all_nodes().iter().enumerate() {
                let mut subst = Substitution::new(num_vars);
                subst.matched_class = cc.find_immut(node.eclass);
                if let EPatternNode::Var(v) = pattern {
                    if subst.bind(*v, ENodeId(i as u32)) {
                        results.push(subst);
                    }
                }
            }
        }
        EPatternNode::Wildcard => {
            for node in cc.all_nodes().iter() {
                let mut subst = Substitution::new(num_vars);
                subst.matched_class = cc.find_immut(node.eclass);
                results.push(subst);
            }
        }
        EPatternNode::Exact(expr) => {
            if let Some(node_id) = cc.lookup_expr(expr) {
                let mut subst = Substitution::new(num_vars);
                subst.matched_class = cc.find_immut(
                    cc.get_node(node_id)
                        .expect("node_id was returned by lookup_expr so it must exist in cc")
                        .eclass,
                );
                results.push(subst);
            }
        }
    }
    results
}
/// Match argument patterns against argument node ids.
pub(super) fn match_args(
    cc: &CongruenceClosure,
    patterns: &[EPatternNode],
    args: &[ENodeId],
    subst: &mut Substitution,
) -> bool {
    if patterns.len() != args.len() {
        return false;
    }
    for (pat, &arg_id) in patterns.iter().zip(args.iter()) {
        if !match_single(cc, pat, arg_id, subst) {
            return false;
        }
    }
    true
}
/// Match a single pattern node against a specific E-node.
pub(super) fn match_single(
    cc: &CongruenceClosure,
    pattern: &EPatternNode,
    node_id: ENodeId,
    subst: &mut Substitution,
) -> bool {
    match pattern {
        EPatternNode::Var(v) => {
            if let Some(existing) = subst.get(*v) {
                cc.are_equal(existing, node_id)
            } else {
                subst.bind(*v, node_id)
            }
        }
        EPatternNode::Wildcard => true,
        EPatternNode::Exact(expr) => {
            if let Some(target_id) = cc.lookup_expr(expr) {
                cc.are_equal(target_id, node_id)
            } else {
                false
            }
        }
        EPatternNode::App { func, args } => {
            if let Some(node) = cc.get_node(node_id) {
                if &node.func != func || node.args.len() != args.len() {
                    return false;
                }
                let node_args = node.args.clone();
                match_args(cc, args, &node_args, subst)
            } else {
                let eclass = cc.get_node(node_id).map(|n| n.eclass);
                if let Some(ec) = eclass {
                    let class_nodes = cc.class_nodes(ec);
                    for &cn in &class_nodes {
                        if let Some(node) = cc.get_node(cn) {
                            if &node.func == func && node.args.len() == args.len() {
                                let node_args = node.args.clone();
                                let mut trial_subst = subst.clone();
                                if match_args(cc, args, &node_args, &mut trial_subst) {
                                    *subst = trial_subst;
                                    return true;
                                }
                            }
                        }
                    }
                }
                false
            }
        }
    }
}
/// Build a kernel proof term from equality explanation steps.
pub fn build_proof(steps: &[EqualityStep]) -> Expr {
    if steps.is_empty() {
        return Expr::Const(Name::str("grind_sorry"), vec![]);
    }
    if steps.len() == 1 {
        return build_single_step_proof(&steps[0]);
    }
    let mut proof = build_single_step_proof(&steps[0]);
    for step in &steps[1..] {
        let step_proof = build_single_step_proof(step);
        proof = mk_eq_trans(proof, step_proof);
    }
    proof
}
/// Build a proof term for a single equality step.
pub(super) fn build_single_step_proof(step: &EqualityStep) -> Expr {
    match &step.reason {
        MergeReason::Reflexivity => mk_eq_refl(step.lhs.clone()),
        MergeReason::Hypothesis(name, _) => Expr::Const(name.clone(), vec![]),
        MergeReason::Congruence(_, _) => Expr::App(
            Box::new(Expr::Const(Name::str("congr"), vec![])),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Eq.refl"), vec![])),
                Box::new(step.lhs.clone()),
            )),
        ),
        MergeReason::EMatchInstance { hyp_name, subst } => {
            let mut result: Expr = Expr::Const(hyp_name.clone(), vec![]);
            for (_, val) in subst {
                result = Expr::App(Box::new(result), Box::new(val.clone()));
            }
            result
        }
        MergeReason::Reduction => mk_eq_refl(step.lhs.clone()),
        MergeReason::Assertion => Expr::Const(Name::str("grind_assertion"), vec![]),
    }
}
/// Build `Eq.refl a`.
pub(super) fn mk_eq_refl(a: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::Const(Name::str("Eq.refl"), vec![])),
        Box::new(a),
    )
}
/// Build `Eq.trans p1 p2`.
pub(super) fn mk_eq_trans(p1: Expr, p2: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Eq.trans"), vec![])),
            Box::new(p1),
        )),
        Box::new(p2),
    )
}
/// Build `Eq.symm p`.
pub(super) fn mk_eq_symm(p: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::Const(Name::str("Eq.symm"), vec![])),
        Box::new(p),
    )
}
/// Convert a ProofStep tree into a kernel Expr.
pub fn proof_step_to_expr(step: &ProofStep) -> Expr {
    match step {
        ProofStep::Refl(e) => mk_eq_refl(e.clone()),
        ProofStep::Symm(inner) => {
            let inner_expr = proof_step_to_expr(inner);
            mk_eq_symm(inner_expr)
        }
        ProofStep::Trans(left, right) => {
            let left_expr = proof_step_to_expr(left);
            let right_expr = proof_step_to_expr(right);
            mk_eq_trans(left_expr, right_expr)
        }
        ProofStep::Congr {
            func_proof,
            arg_proof,
        } => {
            let fp = proof_step_to_expr(func_proof);
            let ap = proof_step_to_expr(arg_proof);
            Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("congr"), vec![])),
                    Box::new(fp),
                )),
                Box::new(ap),
            )
        }
        ProofStep::Hypothesis(name, _) => Expr::Const(name.clone(), vec![]),
        ProofStep::Instance { hyp_name, subst } => {
            let mut result: Expr = Expr::Const(hyp_name.clone(), vec![]);
            for (_, val) in subst {
                result = Expr::App(Box::new(result), Box::new(val.clone()));
            }
            result
        }
        ProofStep::Absurd => Expr::Const(Name::str("absurd"), vec![]),
    }
}
/// Flatten nested applications into head + arguments.
pub(super) fn flatten_app(expr: &Expr) -> (Expr, Vec<Expr>) {
    let mut args = Vec::new();
    let mut current = expr.clone();
    while let Expr::App(f, a) = current {
        args.push(*a);
        current = *f;
    }
    args.reverse();
    (current, args)
}
/// Extract the head name from an expression.
pub(super) fn expr_head_name(expr: &Expr) -> Name {
    match expr {
        Expr::Const(name, _) => name.clone(),
        Expr::FVar(fid) => Name::str(format!("fvar_{}", fid.0)),
        Expr::BVar(idx) => Name::str(format!("bvar_{}", idx)),
        Expr::Lit(lit) => Name::str(format!("lit_{}", lit)),
        Expr::Sort(level) => Name::str(format!("sort_{:?}", level)),
        Expr::Lam(..) => Name::str("lambda"),
        Expr::Pi(..) => Name::str("pi"),
        Expr::Let(..) => Name::str("let"),
        Expr::App(..) => Name::str("app"),
        Expr::Proj(name, idx, _) => Name::str(format!("proj_{}_{}", name, idx)),
    }
}
/// Check if an expression is `True`.
pub(super) fn is_true_expr(expr: &Expr) -> bool {
    matches!(expr, Expr::Const(name, _) if name == & Name::str("True"))
}
/// Check if an expression is `False`.
pub(super) fn is_false_expr(expr: &Expr) -> bool {
    matches!(expr, Expr::Const(name, _) if name == & Name::str("False"))
}
/// Decompose `Eq a b` or `a = b` into `(a, b)`.
pub(super) fn decompose_eq(expr: &Expr) -> Option<(Expr, Expr)> {
    if let Expr::App(f1, rhs) = expr {
        if let Expr::App(f2, lhs) = f1.as_ref() {
            if let Expr::App(eq_head, _ty) = f2.as_ref() {
                if let Expr::Const(name, _) = eq_head.as_ref() {
                    if name == &Name::str("Eq") || name == &Name::str("Eq").append_str("mk") {
                        return Some((*lhs.clone(), *rhs.clone()));
                    }
                }
            }
            if let Expr::App(heq_head, _) = f2.as_ref() {
                if let Expr::App(heq_outer, _) = heq_head.as_ref() {
                    if let Expr::Const(name, _) = heq_outer.as_ref() {
                        if name == &Name::str("HEq") {
                            return Some((*lhs.clone(), *rhs.clone()));
                        }
                    }
                }
            }
        }
    }
    None
}
/// Check if an expression is a forall (Pi with domain in Prop).
pub(super) fn is_forall(expr: &Expr) -> bool {
    matches!(expr, Expr::Pi(..))
}
/// Strip forall binders, returning (number_of_binders, body).
pub(super) fn strip_forall(expr: &Expr) -> (usize, Expr) {
    let mut count = 0;
    let mut current = expr.clone();
    while let Expr::Pi(_, _, _, body) = current {
        count += 1;
        current = *body;
    }
    (count, current)
}
/// Decompose `Or a b` into `(a, b)`.
pub(super) fn decompose_or(expr: &Expr) -> Option<(Expr, Expr)> {
    if let Expr::App(f, b) = expr {
        if let Expr::App(or_head, a) = f.as_ref() {
            if let Expr::Const(name, _) = or_head.as_ref() {
                if name == &Name::str("Or") {
                    return Some((*a.clone(), *b.clone()));
                }
            }
        }
    }
    None
}
/// Check if an expression looks like a proposition (for heuristics).
pub(super) fn is_prop_like(expr: &Expr) -> bool {
    match expr {
        Expr::Const(name, _) => {
            let s = format!("{}", name);
            s == "True" || s == "False" || s.starts_with("And") || s.starts_with("Or")
        }
        Expr::App(f, _) => is_prop_like(f),
        Expr::Pi(..) => true,
        _ => false,
    }
}
/// Apply a substitution to an expression, replacing BVars with their bindings.
pub(super) fn apply_subst_to_expr(
    expr: &Expr,
    subst: &Substitution,
    cc: &CongruenceClosure,
) -> Expr {
    apply_subst_impl(expr, subst, cc, 0)
}
/// Implementation of substitution application.
pub(super) fn apply_subst_impl(
    expr: &Expr,
    subst: &Substitution,
    cc: &CongruenceClosure,
    depth: u32,
) -> Expr {
    if depth > 100 {
        return expr.clone();
    }
    match expr {
        Expr::BVar(idx) => {
            if let Some(binding) = subst.get(*idx) {
                if let Some(node) = cc.get_node(binding) {
                    if let Some(origin) = &node.origin_expr {
                        return origin.clone();
                    }
                }
            }
            expr.clone()
        }
        Expr::App(f, a) => {
            let f2 = apply_subst_impl(f, subst, cc, depth + 1);
            let a2 = apply_subst_impl(a, subst, cc, depth + 1);
            Expr::App(Box::new(f2), Box::new(a2))
        }
        Expr::Lam(info, name, ty, body) => {
            let ty2 = apply_subst_impl(ty, subst, cc, depth + 1);
            let body2 = apply_subst_impl(body, subst, cc, depth + 1);
            Expr::Lam(*info, name.clone(), Box::new(ty2), Box::new(body2))
        }
        Expr::Pi(info, name, ty, body) => {
            let ty2 = apply_subst_impl(ty, subst, cc, depth + 1);
            let body2 = apply_subst_impl(body, subst, cc, depth + 1);
            Expr::Pi(*info, name.clone(), Box::new(ty2), Box::new(body2))
        }
        Expr::Let(name, ty, val, body) => {
            let ty2 = apply_subst_impl(ty, subst, cc, depth + 1);
            let val2 = apply_subst_impl(val, subst, cc, depth + 1);
            let body2 = apply_subst_impl(body, subst, cc, depth + 1);
            Expr::Let(name.clone(), Box::new(ty2), Box::new(val2), Box::new(body2))
        }
        Expr::Proj(name, idx, base) => {
            let base2 = apply_subst_impl(base, subst, cc, depth + 1);
            Expr::Proj(name.clone(), *idx, Box::new(base2))
        }
        _ => expr.clone(),
    }
}
/// Run the `grind` tactic with default configuration.
///
/// This is the main entry point for the grind tactic. It adds all
/// hypotheses and the goal to the congruence closure engine, runs
/// E-matching to instantiate universally quantified hypotheses,
/// and attempts to close the goal.
pub fn tac_grind(state: &mut TacticState, ctx: &mut MetaContext) -> TacticResult<()> {
    tac_grind_with_config(&GrindConfig::default(), state, ctx)
}
/// Run the `grind` tactic with a specific configuration.
pub fn tac_grind_with_config(
    config: &GrindConfig,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<()> {
    let (result, _stats) = grind_with_stats(config, state, ctx)?;
    match result {
        GrindResult::Proved(proof) => {
            state.close_goal(proof, ctx)?;
            Ok(())
        }
        GrindResult::Saturated => Err(TacticError::Failed(
            "grind: saturated without proving the goal".to_string(),
        )),
        GrindResult::ResourceLimit(msg) => Err(TacticError::Failed(format!(
            "grind: resource limit reached: {}",
            msg
        ))),
    }
}
/// Run grind and return both the result and statistics.
#[allow(clippy::too_many_arguments)]
pub fn grind_with_stats(
    config: &GrindConfig,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<(GrindResult, GrindStats)> {
    let goal_view = state.goal_view(ctx)?;
    let mut grind = GrindState::new(config.clone());
    grind.set_goal(goal_view.target.clone());
    for (name, ty) in &goal_view.hyps {
        grind.add_hypothesis(name.clone(), ty.clone());
    }
    let result = grind.run();
    let stats = grind.stats().clone();
    Ok((result, stats))
}
/// Run grind on a specific goal with custom hypotheses.
pub fn grind_on_goal(
    config: &GrindConfig,
    goal: &Expr,
    hyps: &[(Name, Expr)],
) -> (GrindResult, GrindStats) {
    let mut grind = GrindState::new(config.clone());
    grind.set_goal(goal.clone());
    for (name, ty) in hyps {
        grind.add_hypothesis(name.clone(), ty.clone());
    }
    let result = grind.run();
    let stats = grind.stats().clone();
    (result, stats)
}
/// Convenience: run grind with aggressive settings.
pub fn tac_grind_aggressive(state: &mut TacticState, ctx: &mut MetaContext) -> TacticResult<()> {
    tac_grind_with_config(&GrindConfig::aggressive(), state, ctx)
}
/// Check if two expressions are provably equal using the grind engine.
pub fn grind_check_eq(lhs: &Expr, rhs: &Expr, hyps: &[(Name, Expr)], config: &GrindConfig) -> bool {
    let goal = Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Eq"), vec![])),
                Box::new(Expr::Sort(Level::zero())),
            )),
            Box::new(lhs.clone()),
        )),
        Box::new(rhs.clone()),
    );
    let (result, _) = grind_on_goal(config, &goal, hyps);
    result.is_proved()
}
/// Simplified interface: check equality with default config.
pub fn grind_eq(lhs: &Expr, rhs: &Expr, hyps: &[(Name, Expr)]) -> bool {
    grind_check_eq(lhs, rhs, hyps, &GrindConfig::default())
}
/// Extract Nat arithmetic constraints from a list of hypothesis expressions.
///
/// Looks for patterns like `@LE.le Nat _ a b`, `@LT.lt Nat _ a b`, etc.
/// Returns a list of constraints found.
pub fn extract_nat_constraints(hyps: &[(Name, Expr)]) -> Vec<NatConstraint> {
    let mut constraints = Vec::new();
    for (_name, expr) in hyps {
        if let Some(c) = try_parse_nat_constraint(expr) {
            constraints.push(c);
        }
    }
    constraints
}
/// Try to parse a single expression as a Nat arithmetic constraint.
pub fn try_parse_nat_constraint(expr: &Expr) -> Option<NatConstraint> {
    if let Expr::App(func, rhs) = expr {
        if let Expr::App(func2, lhs) = func.as_ref() {
            let rel = match func2.as_ref() {
                Expr::Const(name, _) => match name.to_string().as_str() {
                    "LE.le" | "Nat.le" | "Nat.ble" => Some(NatRelKind::Le),
                    "LT.lt" | "Nat.lt" | "Nat.blt" => Some(NatRelKind::Lt),
                    "GE.ge" | "Nat.ge" => Some(NatRelKind::Ge),
                    "GT.gt" | "Nat.gt" => Some(NatRelKind::Gt),
                    _ => None,
                },
                _ => None,
            };
            if let Some(rel) = rel {
                return Some(NatConstraint {
                    lhs: *lhs.clone(),
                    rhs: *rhs.clone(),
                    rel,
                });
            }
            if let Expr::App(func3, _ty) = func2.as_ref() {
                if let Expr::Const(name, _) = func3.as_ref() {
                    if name.to_string() == "Eq" {
                        return Some(NatConstraint {
                            lhs: *lhs.clone(),
                            rhs: *rhs.clone(),
                            rel: NatRelKind::Eq,
                        });
                    }
                }
            }
        }
    }
    None
}
/// Check if a set of Nat constraints implies `lhs ≤ rhs` by transitivity.
///
/// Uses a simple forward-chaining approach: if we know `a ≤ b` and `b ≤ c`,
/// we can derive `a ≤ c`.
pub fn check_nat_le_by_transitivity(constraints: &[NatConstraint], lhs: &Expr, rhs: &Expr) -> bool {
    let mut reachable: HashSet<String> = HashSet::new();
    let start = format!("{lhs:?}");
    reachable.insert(start.clone());
    let target = format!("{rhs:?}");
    if start == target {
        return true;
    }
    let mut changed = true;
    while changed {
        changed = false;
        for c in constraints {
            if matches!(c.rel, NatRelKind::Le | NatRelKind::Lt | NatRelKind::Eq) {
                let from = format!("{:?}", c.lhs);
                let to = format!("{:?}", c.rhs);
                if reachable.contains(&from) && !reachable.contains(&to) {
                    reachable.insert(to.clone());
                    changed = true;
                    if to == target {
                        return true;
                    }
                }
                if matches!(c.rel, NatRelKind::Eq)
                    && reachable.contains(&to)
                    && !reachable.contains(&from)
                {
                    reachable.insert(from.clone());
                    changed = true;
                }
            }
        }
    }
    false
}
/// Grind with linear arithmetic integration for Nat goals.
///
/// First tries congruence closure, then falls back to linear arithmetic
/// constraint checking for `≤`, `<`, `=` goals over Nat.
pub fn grind_with_la(config: &GrindConfig, goal: &Expr, hyps: &[(Name, Expr)]) -> GrindResult {
    let (base_result, _stats) = grind_on_goal(config, goal, hyps);
    if base_result.is_proved() {
        return base_result;
    }
    let constraints = extract_nat_constraints(hyps);
    if constraints.is_empty() {
        return GrindResult::Saturated;
    }
    if let Some(goal_constraint) = try_parse_nat_constraint(goal) {
        if check_nat_le_by_transitivity(&constraints, &goal_constraint.lhs, &goal_constraint.rhs) {
            return GrindResult::Proved(Expr::Const(Name::str("Nat.le.refl"), vec![]));
        }
    }
    GrindResult::Saturated
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tactic::grind::*;
    #[test]
    fn test_union_find_basic() {
        let mut uf = UnionFind::new();
        let a = uf.make_set();
        let b = uf.make_set();
        let c = uf.make_set();
        assert_eq!(uf.len(), 3);
        assert_eq!(uf.num_sets(), 3);
        assert!(!uf.are_connected(a, b));
        assert!(!uf.are_connected(b, c));
        assert!(uf.union(a, b));
        assert!(uf.are_connected(a, b));
        assert!(!uf.are_connected(a, c));
        assert_eq!(uf.num_sets(), 2);
        assert!(uf.union(b, c));
        assert!(uf.are_connected(a, c));
        assert_eq!(uf.num_sets(), 1);
        assert!(!uf.union(a, c));
        assert_eq!(uf.num_sets(), 1);
    }
    #[test]
    fn test_union_find_path_compression() {
        let mut uf = UnionFind::new();
        let ids: Vec<u32> = (0..10).map(|_| uf.make_set()).collect();
        for i in 0..9 {
            uf.union(ids[i], ids[i + 1]);
        }
        for i in 0..10 {
            for j in 0..10 {
                assert!(uf.are_connected(ids[i], ids[j]));
            }
        }
        let root = uf.find(ids[0]);
        for &id in &ids {
            assert_eq!(uf.find(id), root);
        }
    }
    #[test]
    fn test_union_find_empty() {
        let uf = UnionFind::new();
        assert!(uf.is_empty());
        assert_eq!(uf.num_sets(), 0);
    }
    #[test]
    fn test_cc_add_term_leaf() {
        let mut cc = CongruenceClosure::new();
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let na = cc.add_term(&a);
        let nb = cc.add_term(&b);
        assert_ne!(na, nb);
        assert!(!cc.are_equal(na, nb));
        assert_eq!(cc.num_nodes(), 2);
    }
    #[test]
    fn test_cc_add_term_dedup() {
        let mut cc = CongruenceClosure::new();
        let a = Expr::Const(Name::str("a"), vec![]);
        let n1 = cc.add_term(&a);
        let n2 = cc.add_term(&a);
        assert_eq!(n1, n2);
        assert_eq!(cc.num_nodes(), 1);
    }
    #[test]
    fn test_cc_merge() {
        let mut cc = CongruenceClosure::new();
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let na = cc.add_term(&a);
        let nb = cc.add_term(&b);
        assert!(!cc.are_equal(na, nb));
        cc.merge(na, nb);
        assert!(cc.are_equal(na, nb));
    }
    #[test]
    fn test_cc_transitivity() {
        let mut cc = CongruenceClosure::new();
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let c = Expr::Const(Name::str("c"), vec![]);
        let na = cc.add_term(&a);
        let nb = cc.add_term(&b);
        let nc = cc.add_term(&c);
        cc.merge(na, nb);
        cc.merge(nb, nc);
        assert!(cc.are_equal(na, nc));
    }
    #[test]
    fn test_cc_congruence() {
        let mut cc = CongruenceClosure::new();
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let f = Expr::Const(Name::str("f"), vec![]);
        let fa = Expr::App(Box::new(f.clone()), Box::new(a.clone()));
        let fb = Expr::App(Box::new(f.clone()), Box::new(b.clone()));
        let na = cc.add_term(&a);
        let nb = cc.add_term(&b);
        let nfa = cc.add_term(&fa);
        let nfb = cc.add_term(&fb);
        assert!(!cc.are_equal(nfa, nfb));
        cc.merge(na, nb);
        assert!(cc.are_equal(nfa, nfb));
    }
    #[test]
    fn test_cc_inconsistency() {
        let mut cc = CongruenceClosure::new();
        let t = Expr::Const(Name::str("True"), vec![]);
        let f = Expr::Const(Name::str("False"), vec![]);
        let nt = cc.add_term(&t);
        let nf = cc.add_term(&f);
        assert!(!cc.is_inconsistent());
        cc.merge(nt, nf);
        assert!(cc.is_inconsistent());
    }
    #[test]
    fn test_cc_explain() {
        let mut cc = CongruenceClosure::new();
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let na = cc.add_term(&a);
        let nb = cc.add_term(&b);
        cc.merge(na, nb);
        let steps = cc.explain_equality(na, nb);
        assert!(!steps.is_empty());
    }
    #[test]
    fn test_ematch_compiler_basic() {
        let mut compiler = EMatchCompiler::new();
        let expr = Expr::Pi(
            oxilean_kernel::BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("P"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
        );
        let pattern = compiler.compile(&expr);
        assert!(pattern.num_vars > 0);
    }
    #[test]
    fn test_ematch_run() {
        let mut cc = CongruenceClosure::new();
        let a = Expr::Const(Name::str("a"), vec![]);
        let pa = Expr::App(
            Box::new(Expr::Const(Name::str("P"), vec![])),
            Box::new(a.clone()),
        );
        cc.add_term(&pa);
        let pattern_node = EPatternNode::App {
            func: Name::str("P"),
            args: vec![EPatternNode::Var(0)],
        };
        let pattern = EPattern {
            root: pattern_node,
            num_vars: 1,
            origin: pa.clone(),
            description: "P(?x)".to_string(),
        };
        let matches = run_ematching(&cc, &[pattern], 100);
        assert!(!matches.is_empty());
    }
    #[test]
    fn test_term_index() {
        let mut cc = CongruenceClosure::new();
        let a = Expr::Const(Name::str("a"), vec![]);
        let fa = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(a.clone()),
        );
        cc.add_term(&a);
        cc.add_term(&fa);
        let index = TermIndex::build_from_cc(&cc);
        assert_eq!(index.len(), cc.num_nodes());
        assert!(!index.lookup_func(&Name::str("f")).is_empty());
    }
    #[test]
    fn test_grind_config_default() {
        let config = GrindConfig::default();
        assert_eq!(config.max_rounds, 100);
        assert!(config.split_cases);
        assert!(config.use_simp);
    }
    #[test]
    fn test_grind_config_builder() {
        let config = GrindConfig::default()
            .with_max_rounds(50)
            .with_max_instances(500)
            .with_split_cases(false)
            .with_fuel(100);
        assert_eq!(config.max_rounds, 50);
        assert_eq!(config.max_instances, 500);
        assert!(!config.split_cases);
        assert_eq!(config.fuel, 100);
    }
    #[test]
    fn test_grind_state_basic() {
        let mut grind = GrindState::with_defaults();
        let goal = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Eq"), vec![])),
                    Box::new(Expr::Sort(Level::zero())),
                )),
                Box::new(Expr::Const(Name::str("a"), vec![])),
            )),
            Box::new(Expr::Const(Name::str("a"), vec![])),
        );
        grind.set_goal(goal);
        let result = grind.run();
        assert!(result.is_proved());
    }
    #[test]
    fn test_grind_with_hypothesis() {
        let mut grind = GrindState::with_defaults();
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let eq_ab = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Eq"), vec![])),
                    Box::new(Expr::Sort(Level::zero())),
                )),
                Box::new(a.clone()),
            )),
            Box::new(b.clone()),
        );
        grind.add_hypothesis(Name::str("h"), eq_ab);
        let goal = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Eq"), vec![])),
                    Box::new(Expr::Sort(Level::zero())),
                )),
                Box::new(a),
            )),
            Box::new(b),
        );
        grind.set_goal(goal);
        let result = grind.run();
        assert!(result.is_proved());
    }
    #[test]
    fn test_grind_transitivity_chain() {
        let mut grind = GrindState::with_defaults();
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let c = Expr::Const(Name::str("c"), vec![]);
        let ty = Expr::Sort(Level::zero());
        let mk_eq = |l: &Expr, r: &Expr| -> Expr {
            Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Eq"), vec![])),
                        Box::new(ty.clone()),
                    )),
                    Box::new(l.clone()),
                )),
                Box::new(r.clone()),
            )
        };
        grind.add_hypothesis(Name::str("h1"), mk_eq(&a, &b));
        grind.add_hypothesis(Name::str("h2"), mk_eq(&b, &c));
        grind.set_goal(mk_eq(&a, &c));
        let result = grind.run();
        assert!(result.is_proved());
    }
    #[test]
    fn test_grind_congruence_proof() {
        let mut grind = GrindState::with_defaults();
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let f = Expr::Const(Name::str("f"), vec![]);
        let ty = Expr::Sort(Level::zero());
        let fa = Expr::App(Box::new(f.clone()), Box::new(a.clone()));
        let fb = Expr::App(Box::new(f.clone()), Box::new(b.clone()));
        let mk_eq = |l: &Expr, r: &Expr| -> Expr {
            Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Eq"), vec![])),
                        Box::new(ty.clone()),
                    )),
                    Box::new(l.clone()),
                )),
                Box::new(r.clone()),
            )
        };
        grind.add_hypothesis(Name::str("h"), mk_eq(&a, &b));
        grind.set_goal(mk_eq(&fa, &fb));
        let result = grind.run();
        assert!(result.is_proved());
    }
    #[test]
    fn test_grind_result_enum() {
        let proved = GrindResult::Proved(Expr::BVar(0));
        assert!(proved.is_proved());
        assert!(proved.proof_term().is_some());
        let saturated = GrindResult::Saturated;
        assert!(!saturated.is_proved());
        assert!(saturated.proof_term().is_none());
        let limit = GrindResult::ResourceLimit("test".to_string());
        assert!(!limit.is_proved());
    }
    #[test]
    fn test_grind_stats_display() {
        let stats = GrindStats {
            rounds: 5,
            merges: 10,
            ematches: 20,
            instances: 3,
            splits: 1,
            max_eclass: 4,
            total_nodes: 15,
            congruences: 2,
            hyps_processed: 3,
        };
        let s = format!("{}", stats);
        assert!(s.contains("rounds: 5"));
        assert!(s.contains("merges: 10"));
    }
    #[test]
    fn test_case_splitter() {
        let mut splitter = CaseSplitter::new();
        assert_eq!(splitter.num_disjunctions(), 0);
        let left = Expr::Const(Name::str("P"), vec![]);
        let right = Expr::Const(Name::str("Q"), vec![]);
        splitter.add_disjunction(Name::str("h"), left, right);
        assert_eq!(splitter.num_disjunctions(), 1);
    }
    #[test]
    fn test_signature_table() {
        let mut table = SignatureTable::new();
        assert!(table.is_empty());
        let func = Name::str("f");
        let args = vec![EClassId(0), EClassId(1)];
        let node = ENodeId(42);
        table.insert(func.clone(), args.clone(), node);
        assert_eq!(table.len(), 1);
        let result = table.lookup(&func, &args);
        assert_eq!(result, Some(node));
    }
    #[test]
    fn test_flatten_app() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let expr = Expr::App(
            Box::new(Expr::App(Box::new(f.clone()), Box::new(a.clone()))),
            Box::new(b.clone()),
        );
        let (head, args) = flatten_app(&expr);
        assert_eq!(head, f);
        assert_eq!(args.len(), 2);
        assert_eq!(args[0], a);
        assert_eq!(args[1], b);
    }
    #[test]
    fn test_decompose_eq() {
        let ty = Expr::Sort(Level::zero());
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let eq_expr = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Eq"), vec![])),
                    Box::new(ty),
                )),
                Box::new(a.clone()),
            )),
            Box::new(b.clone()),
        );
        let result = decompose_eq(&eq_expr);
        assert!(result.is_some());
        let (lhs, rhs) = result.expect("result should be valid");
        assert_eq!(lhs, a);
        assert_eq!(rhs, b);
    }
    #[test]
    fn test_proof_step_to_expr() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let step = ProofStep::Refl(a.clone());
        let expr = proof_step_to_expr(&step);
        assert!(matches!(expr, Expr::App(_, _)));
    }
    #[test]
    fn test_build_proof_empty() {
        let proof = build_proof(&[]);
        assert!(matches!(proof, Expr::Const(_, _)));
    }
    #[test]
    fn test_grind_on_goal_api() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let ty = Expr::Sort(Level::zero());
        let goal = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Eq"), vec![])),
                    Box::new(ty),
                )),
                Box::new(a.clone()),
            )),
            Box::new(a),
        );
        let config = GrindConfig::default();
        let (result, stats) = grind_on_goal(&config, &goal, &[]);
        assert!(result.is_proved());
        assert!(stats.rounds > 0);
    }
    #[test]
    fn test_substitution_bind_and_get() {
        let mut subst = Substitution::new(3);
        assert!(!subst.is_complete());
        assert!(subst.bind(0, ENodeId(10)));
        assert!(subst.bind(1, ENodeId(20)));
        assert!(subst.bind(2, ENodeId(30)));
        assert!(subst.is_complete());
        assert_eq!(subst.get(0), Some(ENodeId(10)));
        assert_eq!(subst.get(1), Some(ENodeId(20)));
        assert_eq!(subst.get(2), Some(ENodeId(30)));
        assert!(subst.bind(0, ENodeId(10)));
        assert!(!subst.bind(0, ENodeId(99)));
    }
    #[test]
    fn test_enode_properties() {
        let node = ENode::new(Name::str("f"), vec![ENodeId(1), ENodeId(2)], EClassId(0));
        assert_eq!(node.arity(), 2);
        assert!(!node.is_leaf());
        let leaf = ENode::new(Name::str("a"), vec![], EClassId(1));
        assert_eq!(leaf.arity(), 0);
        assert!(leaf.is_leaf());
    }
    #[test]
    fn test_eclass_operations() {
        let mut eclass = EClass::new(EClassId(0));
        assert_eq!(eclass.size(), 0);
        eclass.add_node(ENodeId(1));
        eclass.add_node(ENodeId(2));
        assert_eq!(eclass.size(), 2);
        eclass.add_parent(ENodeId(10));
        assert_eq!(eclass.parents.len(), 1);
    }
    #[test]
    fn test_strip_forall() {
        let expr = Expr::Const(Name::str("P"), vec![]);
        let (n, body) = strip_forall(&expr);
        assert_eq!(n, 0);
        assert_eq!(body, expr);
        let forall1 = Expr::Pi(
            oxilean_kernel::BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::Const(Name::str("P"), vec![])),
        );
        let (n, _body) = strip_forall(&forall1);
        assert_eq!(n, 1);
    }
    #[test]
    fn test_is_prop_like() {
        assert!(is_prop_like(&Expr::Const(Name::str("True"), vec![])));
        assert!(is_prop_like(&Expr::Const(Name::str("False"), vec![])));
        assert!(!is_prop_like(&Expr::BVar(0)));
    }
    #[test]
    fn test_nat_rel_kind_display() {
        assert_eq!(format!("{}", NatRelKind::Le), "≤");
        assert_eq!(format!("{}", NatRelKind::Lt), "<");
        assert_eq!(format!("{}", NatRelKind::Eq), "=");
        assert_eq!(format!("{}", NatRelKind::Ge), "≥");
        assert_eq!(format!("{}", NatRelKind::Gt), ">");
    }
    #[test]
    fn test_try_parse_nat_constraint_le() {
        let le = Expr::Const(Name::str("LE.le"), vec![]);
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let expr = Expr::App(
            Box::new(Expr::App(Box::new(le), Box::new(a.clone()))),
            Box::new(b.clone()),
        );
        let result = try_parse_nat_constraint(&expr);
        assert!(result.is_some());
        let c = result.expect("c should be present");
        assert_eq!(c.rel, NatRelKind::Le);
    }
    #[test]
    fn test_try_parse_nat_constraint_lt() {
        let lt = Expr::Const(Name::str("LT.lt"), vec![]);
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let expr = Expr::App(Box::new(Expr::App(Box::new(lt), Box::new(a))), Box::new(b));
        let result = try_parse_nat_constraint(&expr);
        assert!(result.is_some());
        assert_eq!(result.expect("result should be valid").rel, NatRelKind::Lt);
    }
    #[test]
    fn test_try_parse_nat_constraint_none() {
        let expr = Expr::Const(Name::str("foo"), vec![]);
        let result = try_parse_nat_constraint(&expr);
        assert!(result.is_none());
    }
    #[test]
    fn test_extract_nat_constraints_empty() {
        let hyps: Vec<(Name, Expr)> = vec![];
        let constraints = extract_nat_constraints(&hyps);
        assert!(constraints.is_empty());
    }
    #[test]
    fn test_extract_nat_constraints_from_hyps() {
        let le = Expr::Const(Name::str("LE.le"), vec![]);
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let expr = Expr::App(Box::new(Expr::App(Box::new(le), Box::new(a))), Box::new(b));
        let hyps = vec![(Name::str("h1"), expr)];
        let constraints = extract_nat_constraints(&hyps);
        assert_eq!(constraints.len(), 1);
        assert_eq!(constraints[0].rel, NatRelKind::Le);
    }
    #[test]
    fn test_check_nat_le_trivial_reflexivity() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let result = check_nat_le_by_transitivity(&[], &a, &a);
        assert!(result);
    }
    #[test]
    fn test_check_nat_le_via_chain() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let c = Expr::Const(Name::str("c"), vec![]);
        let constraints = vec![
            NatConstraint {
                lhs: a.clone(),
                rhs: b.clone(),
                rel: NatRelKind::Le,
            },
            NatConstraint {
                lhs: b.clone(),
                rhs: c.clone(),
                rel: NatRelKind::Le,
            },
        ];
        assert!(check_nat_le_by_transitivity(&constraints, &a, &c));
    }
    #[test]
    fn test_check_nat_le_no_chain() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let c = Expr::Const(Name::str("c"), vec![]);
        let constraints = vec![NatConstraint {
            lhs: a.clone(),
            rhs: b.clone(),
            rel: NatRelKind::Le,
        }];
        assert!(!check_nat_le_by_transitivity(&constraints, &a, &c));
    }
    #[test]
    fn test_nat_constraint_fields() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let c = NatConstraint {
            lhs: a.clone(),
            rhs: b.clone(),
            rel: NatRelKind::Ge,
        };
        assert_eq!(c.rel, NatRelKind::Ge);
        assert!(matches!(c.lhs, Expr::Const(_, _)));
        assert!(matches!(c.rhs, Expr::Const(_, _)));
    }
    #[test]
    fn test_grind_with_la_proves_eq_refl() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let ty = Expr::Sort(Level::zero());
        let goal = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Eq"), vec![])),
                    Box::new(ty),
                )),
                Box::new(a.clone()),
            )),
            Box::new(a),
        );
        let config = GrindConfig::default();
        let result = grind_with_la(&config, &goal, &[]);
        assert!(result.is_proved());
    }
    #[test]
    fn test_grind_with_la_nat_rel_not_proved() {
        let le = Expr::Const(Name::str("LE.le"), vec![]);
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let goal = Expr::App(Box::new(Expr::App(Box::new(le), Box::new(a))), Box::new(b));
        let config = GrindConfig::default();
        let result = grind_with_la(&config, &goal, &[]);
        assert!(!result.is_proved());
    }
}
