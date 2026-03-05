//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::context::ElabContext;
use oxilean_kernel::{
    instantiate, instantiate_level_params, ConstantInfo, Expr, Level, Name, Reducer,
};

use super::types::{
    AnnotationPool, BiDirectionalInferState, BidirMode, BidirResult, CheckDirection, Constraint,
    ConstraintPriority, ConstraintSimplifier, ConstraintSolver, ExpectedTypeStack, InferCache,
    InferCacheExt, InferDecision, InferError, InferErrorCollector, InferErrorKind, InferFuel,
    InferHint, InferLogger, InferMode, InferRuleStats, InferSessionConfig, InferStats,
    InferStatsExt, MetaVarSubst, PrioritizedConstraint, SimpleConstraintSolver, SolveResult,
    TypeAnnotation, TypeAnnotationMap, TypeEnv, TypeInferenceRule, TypeInferencer,
    UnificationContext,
};

/// Metavariable ID (placeholder).
pub type MetaVarId = u64;
#[cfg(test)]
mod tests {
    use super::*;
    use crate::infer::*;
    use oxilean_kernel::Environment;
    #[test]
    fn test_infer_sort() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let mut inferencer = TypeInferencer::new(&mut ctx);
        let expr = Expr::Sort(Level::zero());
        let ty = inferencer.infer(&expr).expect("inference should succeed");
        assert!(matches!(ty, Expr::Sort(_)));
    }
    #[test]
    fn test_constraint_equal() {
        let c = Constraint::Equal(Expr::Sort(Level::zero()), Expr::Sort(Level::zero()));
        assert!(matches!(c, Constraint::Equal(_, _)));
    }
    #[test]
    fn test_inferencer_create() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let inferencer = TypeInferencer::new(&mut ctx);
        assert_eq!(inferencer.constraints().len(), 0);
    }
}
/// Simplify a list of constraints by removing trivially true ones.
///
/// `Equal(e, e)` constraints where both sides are syntactically identical
/// are removed as they are trivially satisfied.
pub fn simplify_constraints(constraints: &[Constraint]) -> Vec<Constraint> {
    constraints
        .iter()
        .filter(|c| match c {
            Constraint::Equal(e1, e2) => e1 != e2,
            _ => true,
        })
        .cloned()
        .collect()
}
/// Partition constraints into assignment constraints and equality constraints.
///
/// Returns `(assignments, equalities, has_types)`.
pub fn partition_constraints(
    constraints: &[Constraint],
) -> (Vec<&Constraint>, Vec<&Constraint>, Vec<&Constraint>) {
    let mut assignments = Vec::new();
    let mut equalities = Vec::new();
    let mut has_types = Vec::new();
    for c in constraints {
        match c {
            Constraint::Assign(_, _) => assignments.push(c),
            Constraint::Equal(_, _) => equalities.push(c),
            Constraint::HasType(_, _) => has_types.push(c),
        }
    }
    (assignments, equalities, has_types)
}
/// Sort constraints by priority for solving order.
///
/// Assignment constraints come first, then equalities, then has-type.
pub fn sort_constraints_by_priority(constraints: &mut [Constraint]) {
    constraints.sort_by_key(|c| match c {
        Constraint::Assign(_, _) => 0u8,
        Constraint::Equal(_, _) => 1,
        Constraint::HasType(_, _) => 2,
    });
}
/// Collect all metavariable IDs referenced in an expression.
pub fn collect_metavars(expr: &Expr) -> Vec<MetaVarId> {
    let mut result = Vec::new();
    collect_metavars_impl(expr, &mut result);
    result
}
/// The base offset used to encode metavariables as FVar IDs.
pub const METAVAR_BASE: u64 = 1_000_000;
fn collect_metavars_impl(expr: &Expr, out: &mut Vec<MetaVarId>) {
    match expr {
        Expr::FVar(fvar) if fvar.0 >= METAVAR_BASE => {
            let meta_id = fvar.0 - METAVAR_BASE;
            if !out.contains(&meta_id) {
                out.push(meta_id);
            }
        }
        Expr::App(f, a) => {
            collect_metavars_impl(f, out);
            collect_metavars_impl(a, out);
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            collect_metavars_impl(ty, out);
            collect_metavars_impl(body, out);
        }
        Expr::Let(_, ty, val, body) => {
            collect_metavars_impl(ty, out);
            collect_metavars_impl(val, out);
            collect_metavars_impl(body, out);
        }
        Expr::Proj(_, _, s) => collect_metavars_impl(s, out),
        _ => {}
    }
}
/// Create a fresh metavariable expression with the given ID.
pub fn mk_metavar(id: MetaVarId) -> Expr {
    Expr::FVar(oxilean_kernel::FVarId::new(id))
}
/// Check if an expression is a metavariable.
pub fn is_metavar(expr: &Expr) -> bool {
    matches!(expr, Expr::FVar(id) if id.0 >= METAVAR_BASE)
}
/// Check whether `expr` contains any metavariable (fast approximate check).
///
/// Returns `true` if any FVar with id >= [`METAVAR_BASE`] occurs anywhere in
/// `expr`.  This is conservative: it may return `true` even for ordinary
/// free variables that happen to share the ID space, but in practice the
/// `METAVAR_BASE` offset ensures correct separation.
pub fn contains_mvar_approx(expr: &Expr) -> bool {
    match expr {
        Expr::FVar(fvar) => fvar.0 >= METAVAR_BASE,
        Expr::App(f, a) => contains_mvar_approx(f) || contains_mvar_approx(a),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            contains_mvar_approx(ty) || contains_mvar_approx(body)
        }
        Expr::Let(_, ty, val, body) => {
            contains_mvar_approx(ty) || contains_mvar_approx(val) || contains_mvar_approx(body)
        }
        Expr::Proj(_, _, s) => contains_mvar_approx(s),
        _ => false,
    }
}
/// Infer the type of a literal.
pub fn infer_literal_type(lit: &oxilean_kernel::Literal) -> Expr {
    use oxilean_kernel::Literal;
    match lit {
        Literal::Nat(_) => Expr::Const(Name::str("Nat"), vec![]),
        Literal::Str(_) => Expr::Const(Name::str("String"), vec![]),
    }
}
/// Check if an expression is a type (Prop or Type i).
pub fn is_type_expr(expr: &Expr) -> bool {
    matches!(expr, Expr::Sort(_))
}
/// Compute the expected universe level for a Prop.
pub fn prop_level() -> Level {
    Level::zero()
}
/// Compute the expected universe level for Type 0.
pub fn type0_level() -> Level {
    Level::succ(Level::zero())
}
/// Compute the expected universe level for Type 1.
pub fn type1_level() -> Level {
    Level::succ(Level::succ(Level::zero()))
}
/// Check if two constraints are identical.
pub fn constraints_eq(c1: &Constraint, c2: &Constraint) -> bool {
    c1 == c2
}
/// Merge two constraint lists, removing duplicates.
pub fn merge_constraints(cs1: &[Constraint], cs2: &[Constraint]) -> Vec<Constraint> {
    let mut result: Vec<Constraint> = cs1.to_vec();
    for c in cs2 {
        if !result.contains(c) {
            result.push(c.clone());
        }
    }
    result
}
/// Substitute a metavariable assignment into a list of constraints.
///
/// Given `m := val`, removes the `Assign(m, ...)` constraint and replaces
/// all occurrences of metavar `m` in the remaining constraints with `val`.
pub fn apply_assignment(id: MetaVarId, val: &Expr, constraints: &[Constraint]) -> Vec<Constraint> {
    constraints
        .iter()
        .filter(|c| !matches!(c, Constraint::Assign(mid, _) if * mid == id))
        .map(|c| subst_meta_in_constraint(c, id, val))
        .collect()
}
/// Substitute metavar `id` → `val` inside a single constraint.
fn subst_meta_in_constraint(c: &Constraint, id: MetaVarId, val: &Expr) -> Constraint {
    match c {
        Constraint::Equal(e1, e2) => {
            Constraint::Equal(subst_meta_expr(e1, id, val), subst_meta_expr(e2, id, val))
        }
        Constraint::HasType(e, ty) => {
            Constraint::HasType(subst_meta_expr(e, id, val), subst_meta_expr(ty, id, val))
        }
        Constraint::Assign(mid, e) => Constraint::Assign(*mid, subst_meta_expr(e, id, val)),
    }
}
/// Recursively substitute metavar `id` → `val` inside `expr`.
fn subst_meta_expr(expr: &Expr, id: MetaVarId, val: &Expr) -> Expr {
    match expr {
        Expr::FVar(fv) if fv.0 >= METAVAR_BASE => {
            if fv.0 - METAVAR_BASE == id {
                val.clone()
            } else {
                expr.clone()
            }
        }
        Expr::App(f, a) => Expr::App(
            Box::new(subst_meta_expr(f, id, val)),
            Box::new(subst_meta_expr(a, id, val)),
        ),
        Expr::Lam(bi, name, ty, body) => Expr::Lam(
            *bi,
            name.clone(),
            Box::new(subst_meta_expr(ty, id, val)),
            Box::new(subst_meta_expr(body, id, val)),
        ),
        Expr::Pi(bi, name, ty, body) => Expr::Pi(
            *bi,
            name.clone(),
            Box::new(subst_meta_expr(ty, id, val)),
            Box::new(subst_meta_expr(body, id, val)),
        ),
        Expr::Let(name, ty, v, body) => Expr::Let(
            name.clone(),
            Box::new(subst_meta_expr(ty, id, val)),
            Box::new(subst_meta_expr(v, id, val)),
            Box::new(subst_meta_expr(body, id, val)),
        ),
        Expr::Proj(name, idx, inner) => Expr::Proj(
            name.clone(),
            *idx,
            Box::new(subst_meta_expr(inner, id, val)),
        ),
        _ => expr.clone(),
    }
}
#[cfg(test)]
mod extended_infer_tests {
    use super::*;
    use crate::infer::*;
    use oxilean_kernel::{Environment, Level, Literal};
    #[test]
    fn test_simplify_constraints_removes_trivial() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let cs = vec![
            Constraint::Equal(nat.clone(), nat.clone()),
            Constraint::HasType(Expr::BVar(0), nat.clone()),
        ];
        let simplified = simplify_constraints(&cs);
        assert_eq!(simplified.len(), 1);
        assert!(matches!(simplified[0], Constraint::HasType(_, _)));
    }
    #[test]
    fn test_partition_constraints() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let cs = vec![
            Constraint::Assign(1, nat.clone()),
            Constraint::Equal(nat.clone(), nat.clone()),
            Constraint::HasType(Expr::BVar(0), nat.clone()),
        ];
        let (assigns, eqs, has_types) = partition_constraints(&cs);
        assert_eq!(assigns.len(), 1);
        assert_eq!(eqs.len(), 1);
        assert_eq!(has_types.len(), 1);
    }
    #[test]
    fn test_sort_constraints_by_priority() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let mut cs = vec![
            Constraint::HasType(Expr::BVar(0), nat.clone()),
            Constraint::Assign(1, nat.clone()),
            Constraint::Equal(nat.clone(), nat.clone()),
        ];
        sort_constraints_by_priority(&mut cs);
        assert!(matches!(cs[0], Constraint::Assign(_, _)));
        assert!(matches!(cs[1], Constraint::Equal(_, _)));
        assert!(matches!(cs[2], Constraint::HasType(_, _)));
    }
    #[test]
    fn test_infer_literal_type_nat() {
        let ty = infer_literal_type(&Literal::Nat(42));
        assert!(matches!(ty, Expr::Const(n, _) if n == Name::str("Nat")));
    }
    #[test]
    fn test_infer_literal_type_str() {
        let ty = infer_literal_type(&Literal::Str("hello".to_string()));
        assert!(matches!(ty, Expr::Const(n, _) if n == Name::str("String")));
    }
    #[test]
    fn test_is_type_expr() {
        assert!(is_type_expr(&Expr::Sort(Level::zero())));
        assert!(!is_type_expr(&Expr::Const(Name::str("Nat"), vec![])));
    }
    #[test]
    fn test_merge_constraints_no_duplicates() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let c = Constraint::Equal(nat.clone(), nat.clone());
        let cs1 = vec![c.clone()];
        let cs2 = vec![c.clone()];
        let merged = merge_constraints(&cs1, &cs2);
        assert_eq!(merged.len(), 1);
    }
    #[test]
    fn test_constraint_solver_trivial() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let mut solver = ConstraintSolver::new();
        solver.add(Constraint::Equal(nat.clone(), nat.clone()));
        solver.add(Constraint::Assign(1, nat.clone()));
        let done = solver.solve_all();
        assert!(done);
        assert_eq!(solver.solved_count(), 1);
    }
    #[test]
    fn test_constraint_solver_assignment() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let mut solver = ConstraintSolver::new();
        solver.add(Constraint::Assign(42, nat.clone()));
        let _ = solver.solve_all();
        assert_eq!(solver.get_assignment(42), Some(&nat));
    }
    #[test]
    fn test_infer_stats_hit_rate() {
        let mut stats = InferStats::new();
        stats.record_cache_hit();
        stats.record_cache_hit();
        stats.record_cache_miss();
        let rate = stats.hit_rate();
        assert!((rate - 2.0 / 3.0).abs() < 1e-10);
    }
    #[test]
    fn test_infer_cache_operations() {
        let mut cache = InferCache::new();
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        assert!(cache.is_empty());
        cache.insert(1, nat.clone());
        assert_eq!(cache.len(), 1);
        assert_eq!(cache.get(1), Some(&nat));
        cache.clear();
        assert!(cache.is_empty());
    }
    #[test]
    fn test_mk_metavar() {
        let mv = mk_metavar(1_000_000);
        assert!(is_metavar(&mv));
    }
    #[test]
    fn test_prioritized_constraint() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let pc = PrioritizedConstraint::new(
            Constraint::Equal(nat.clone(), nat.clone()),
            ConstraintPriority::High,
        )
        .with_tag("test");
        assert!(pc.tag.is_some());
        assert_eq!(pc.priority, ConstraintPriority::High);
    }
    #[test]
    fn test_infer_error_display() {
        let e = InferError::FuelExhausted;
        assert!(!e.to_string().is_empty());
        let e2 = InferError::UnknownConst(Name::str("foo"));
        assert!(e2.to_string().contains("foo"));
    }
    #[test]
    fn test_infer_sort_prop() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let mut inferencer = TypeInferencer::new(&mut ctx);
        let prop = Expr::Sort(Level::zero());
        let ty = inferencer.infer(&prop).expect("inference should succeed");
        assert!(matches!(ty, Expr::Sort(_)));
    }
}
#[cfg(test)]
mod bidir_tests {
    use super::*;
    use crate::infer::*;
    use oxilean_kernel::Name;
    #[test]
    fn test_bidir_result_mode() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let r = BidirResult::from_infer(nat.clone(), vec![]);
        assert_eq!(r.mode, BidirMode::Infer);
        let r2 = BidirResult::from_check(nat, vec![]);
        assert_eq!(r2.mode, BidirMode::Check);
    }
    #[test]
    fn test_type_annotation_to_constraint() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let ann = TypeAnnotation::new(Expr::BVar(0), nat.clone());
        let c = ann.to_constraint(nat.clone());
        assert!(matches!(c, Constraint::Equal(_, _)));
    }
    #[test]
    fn test_annotation_pool() {
        let mut pool = AnnotationPool::new();
        assert!(pool.is_empty());
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        pool.push(TypeAnnotation::new(Expr::BVar(0), nat.clone()));
        assert_eq!(pool.len(), 1);
        let peeked = pool.peek().expect("test operation should succeed");
        assert_eq!(peeked.ty, nat);
        pool.pop();
        assert!(pool.is_empty());
    }
    #[test]
    fn test_type_env_lookup() {
        let mut env = TypeEnv::new();
        env.push("x", Expr::Const(Name::str("Nat"), vec![]));
        env.push("y", Expr::Const(Name::str("Bool"), vec![]));
        assert!(env.lookup("x").is_some());
        assert!(env.lookup("z").is_none());
        env.pop();
        assert!(env.lookup("y").is_none());
    }
    #[test]
    fn test_type_env_shadow() {
        let mut env = TypeEnv::new();
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let bool_e = Expr::Const(Name::str("Bool"), vec![]);
        env.push("x", nat.clone());
        env.push("x", bool_e.clone());
        assert_eq!(env.lookup("x"), Some(&bool_e));
    }
    #[test]
    fn test_constraint_simplifier() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let assignments = std::collections::HashMap::new();
        let simp = ConstraintSimplifier::new(assignments);
        let c = Constraint::Equal(nat.clone(), nat.clone());
        let simplified = simp.simplify(&c);
        assert!(matches!(simplified, Constraint::Equal(_, _)));
    }
    #[test]
    fn test_infer_fuel() {
        let mut fuel = InferFuel::new(3);
        assert!(fuel.is_ok());
        assert!(fuel.consume());
        assert!(fuel.consume());
        assert!(fuel.consume());
        assert!(!fuel.is_ok());
        assert!(!fuel.consume());
    }
    #[test]
    fn test_constraint_simplifier_all() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let cs = vec![
            Constraint::Equal(nat.clone(), nat.clone()),
            Constraint::Assign(1, nat.clone()),
        ];
        let simp = ConstraintSimplifier::new(Default::default());
        let simplified = simp.simplify_all(&cs);
        assert_eq!(simplified.len(), 2);
    }
    #[test]
    fn test_check_direction() {
        let d1 = CheckDirection::Check;
        let d2 = CheckDirection::Infer;
        assert_ne!(d1, d2);
        assert_eq!(d1, CheckDirection::Check);
    }
    #[test]
    fn test_prop_type_levels() {
        let p = prop_level();
        let t0 = type0_level();
        let t1 = type1_level();
        assert_ne!(p, t0);
        assert_ne!(t0, t1);
    }
    #[test]
    fn test_type_env_len() {
        let mut env = TypeEnv::new();
        assert_eq!(env.len(), 0);
        env.push("a", Expr::BVar(0));
        assert_eq!(env.len(), 1);
    }
}
#[cfg(test)]
mod infer_extended_tests {
    use super::*;
    use crate::infer::*;
    #[test]
    fn test_infer_cache_hit_rate() {
        let mut cache = InferCacheExt::new();
        cache.get(1);
        cache.insert(1, Expr::BVar(0));
        cache.get(1);
        cache.get(2);
        assert!((cache.hit_rate() - 1.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_meta_var_subst() {
        let mut subst = MetaVarSubst::new();
        subst.assign(0, Expr::BVar(0));
        subst.assign(1, Expr::BVar(1));
        assert!(subst.is_assigned(0));
        assert!(!subst.is_assigned(2));
        assert_eq!(subst.len(), 2);
        assert_eq!(subst.unassigned_count(&[0, 1, 2, 3]), 2);
    }
    #[test]
    fn test_infer_stats_unsolved() {
        let mut stats = InferStatsExt::new();
        stats.record_metavar();
        stats.record_metavar();
        stats.record_solved_metavar();
        assert_eq!(stats.unsolved_metavars(), 1);
        stats.record_constraint();
        stats.record_constraint();
        stats.record_solved_constraint();
        assert_eq!(stats.unsolved_constraints(), 1);
    }
    #[test]
    fn test_type_annotation_map() {
        let mut map = TypeAnnotationMap::new();
        map.annotate(Name::str("foo"), Expr::BVar(0));
        assert!(map.has_annotation(&Name::str("foo")));
        assert!(!map.has_annotation(&Name::str("bar")));
        assert_eq!(map.len(), 1);
    }
    #[test]
    fn test_expected_type_stack() {
        let mut stack = ExpectedTypeStack::new();
        assert!(stack.current().is_none());
        stack.push(Some(Expr::BVar(0)));
        assert!(stack.current().is_some());
        stack.push(None);
        assert!(stack.current().is_none());
        stack.pop();
        assert!(stack.current().is_some());
    }
    #[test]
    fn test_infer_hint_builder() {
        let hint = InferHint::new()
            .prefer_prop()
            .no_metavars()
            .with_unfold_depth(10);
        assert!(hint.prefer_prop);
        assert!(!hint.allow_metavars);
        assert_eq!(hint.unfold_depth, 10);
    }
    #[test]
    fn test_infer_decision_success() {
        let d1 = InferDecision::CacheHit;
        let d2 = InferDecision::RuleApplied("Pi-Intro");
        let d3 = InferDecision::Failed("cannot infer".to_string());
        assert!(d1.is_success());
        assert!(d2.is_success());
        assert!(!d3.is_success());
        assert_eq!(d2.rule_name(), Some("Pi-Intro"));
    }
    #[test]
    fn test_infer_logger_failures() {
        let mut logger = InferLogger::new(100);
        logger.log(Expr::BVar(0), InferDecision::CacheHit);
        logger.log(Expr::BVar(1), InferDecision::Failed("oops".to_string()));
        logger.log(Expr::BVar(2), InferDecision::RuleApplied("Var"));
        assert_eq!(logger.len(), 3);
        assert_eq!(logger.failures().len(), 1);
        assert_eq!(logger.count_rule("Var"), 1);
    }
    #[test]
    fn test_constraint_solver_trivial() {
        let mut solver = SimpleConstraintSolver::new();
        solver.add_constraint(Constraint::Equal(Expr::BVar(0), Expr::BVar(0)));
        let result = solver.solve_all();
        assert_eq!(result, SolveResult::Solved);
        assert_eq!(solver.solved_count(), 1);
    }
    #[test]
    fn test_constraint_solver_stuck() {
        let mut solver = SimpleConstraintSolver::new();
        solver.add_constraint(Constraint::Equal(Expr::BVar(0), Expr::BVar(1)));
        let result = solver.solve_all();
        assert_eq!(result, SolveResult::Stuck);
        assert_eq!(solver.pending_count(), 1);
    }
}
#[cfg(test)]
mod infer_extended_tests2 {
    use super::*;
    use crate::infer::*;
    #[test]
    fn test_unification_context_depth() {
        let mut ctx = UnificationContext::new(3);
        assert!(ctx.push_depth());
        assert!(ctx.push_depth());
        assert!(ctx.push_depth());
        assert!(!ctx.push_depth());
        ctx.pop_depth();
        assert!(ctx.push_depth());
    }
    #[test]
    fn test_unification_context_assign() {
        let mut ctx = UnificationContext::new(10);
        ctx.assign_meta(0, Expr::BVar(0));
        assert!(ctx.lookup_meta(0).is_some());
        assert!(ctx.lookup_meta(1).is_none());
        assert!(ctx.is_fully_assigned(&[0]));
        assert!(!ctx.is_fully_assigned(&[0, 1]));
    }
    #[test]
    fn test_type_inference_rules() {
        let all = TypeInferenceRule::all_rules();
        assert_eq!(all.len(), 10);
        let bvar_rule = TypeInferenceRule::BVar;
        assert!(bvar_rule.applicable_to(&Expr::BVar(0)));
        assert!(
            !bvar_rule.applicable_to(&Expr::App(Box::new(Expr::BVar(0)), Box::new(Expr::BVar(1))))
        );
    }
    #[test]
    fn test_infer_rule_stats() {
        let mut stats = InferRuleStats::new();
        let rule = TypeInferenceRule::App;
        stats.record_success(&rule);
        stats.record_success(&rule);
        stats.record_failure(&rule);
        assert_eq!(stats.success_count(&rule), 2);
        assert_eq!(stats.failure_count(&rule), 1);
        assert_eq!(stats.total_invocations(), 2);
        assert_eq!(stats.total_failures(), 1);
        let most_used = stats.most_used_rule();
        assert!(most_used.is_some());
    }
    #[test]
    fn test_infer_error_display() {
        let err = InferErrorKind::UnboundVariable(5);
        assert!(err.to_string().contains("de Bruijn index 5"));
        let err2 = InferErrorKind::RecursionLimit;
        assert!(err2.to_string().contains("recursion limit"));
    }
    #[test]
    fn test_infer_error_collector() {
        let mut coll = InferErrorCollector::new(3);
        coll.add(InferErrorKind::RecursionLimit);
        coll.add(InferErrorKind::UnboundVariable(0));
        coll.add(InferErrorKind::Custom("oops".to_string()));
        coll.add(InferErrorKind::RecursionLimit);
        assert_eq!(coll.count(), 3);
        assert!(coll.is_saturated());
        assert!(coll.has_errors());
    }
}
#[cfg(test)]
mod infer_bidir_tests {
    use super::*;
    use crate::infer::*;
    #[test]
    fn test_bidir_infer_mode() {
        let mut state = BiDirectionalInferState::infer_mode();
        assert_eq!(state.mode(), InferMode::Infer);
        assert!(state.expected_type().is_none());
        state.set_inferred(Expr::BVar(0));
        assert!(state.is_done());
    }
    #[test]
    fn test_bidir_check_mode() {
        let state = BiDirectionalInferState::check_mode(Expr::Sort(Level::Zero));
        assert_eq!(state.mode(), InferMode::Check);
        assert!(state.expected_type().is_some());
    }
    #[test]
    fn test_infer_session_config() {
        let cfg = InferSessionConfig::new()
            .without_cache()
            .with_logging()
            .with_max_depth(200);
        assert!(!cfg.enable_cache);
        assert!(cfg.enable_logging);
        assert_eq!(cfg.max_infer_depth, 200);
    }
}
