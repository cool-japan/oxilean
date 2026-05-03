//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::infer::{Constraint, MetaVarId};
use oxilean_kernel::{Expr, Level, Literal, Name};
use std::collections::HashMap;

use super::types::{
    CheckpointManager, ConflictInfo, ConstraintGraph, ConstraintPriority, ConstraintQueue,
    ConstraintSimplifier, ConstraintSolver, IncrementalSolver, MetaVarContext, OccursCheck,
    PrioritySolver, ScheduledConstraint, SimplificationPhase, SolverConfig, SolverDiagCollector,
    SolverDiagnostic, SolverEventKind, SolverEventLog, SolverPipeline, SolverReport, SolverState,
    SolverStats, UnifyResult,
};

/// Check if two expressions are unifiable.
pub fn is_unifiable(e1: &Expr, e2: &Expr) -> bool {
    let mut solver = ConstraintSolver::new();
    solver.unify(e1, e2).is_ok()
}
/// Try basic structural unification between two expressions.
pub fn unify_basic(e1: &Expr, e2: &Expr, assignments: &HashMap<MetaVarId, Expr>) -> UnifyResult {
    if e1 == e2 {
        return UnifyResult::Solved(HashMap::new());
    }
    let e1_is_meta = matches!(e1, Expr::FVar(fv) if fv.0 >= MVAR_OFFSET);
    let e2_is_meta = matches!(e2, Expr::FVar(fv) if fv.0 >= MVAR_OFFSET);
    if e1_is_meta || e2_is_meta {
        let mut asgn = assignments.clone();
        match crate::unify::unify_meta_aware(e1, e2, &mut asgn) {
            Ok(()) => {
                let new_assigns: HashMap<MetaVarId, Expr> = asgn
                    .into_iter()
                    .filter(|(k, v)| assignments.get(k) != Some(v))
                    .collect();
                return UnifyResult::Solved(new_assigns);
            }
            Err(crate::unify::UnifyError::OccursCheck) => {
                return UnifyResult::Fail("occurs check failed".to_string());
            }
            Err(_) if e1_is_meta && e2_is_meta => {
                return UnifyResult::Defer;
            }
            Err(e) => return UnifyResult::Fail(e.to_string()),
        }
    }
    match (e1, e2) {
        (Expr::App(f1, a1), Expr::App(f2, a2)) => {
            let mut r1 = unify_basic(f1, f2, assignments);
            let r2 = unify_basic(a1, a2, assignments);
            match (&mut r1, r2) {
                (UnifyResult::Solved(m1), UnifyResult::Solved(m2)) => {
                    m1.extend(m2);
                    r1
                }
                (UnifyResult::Fail(_), _) => r1,
                (_, UnifyResult::Fail(msg)) => UnifyResult::Fail(msg),
                _ => UnifyResult::Defer,
            }
        }
        (Expr::Lam(_, _, ty1, b1), Expr::Lam(_, _, ty2, b2))
        | (Expr::Pi(_, _, ty1, b1), Expr::Pi(_, _, ty2, b2)) => {
            let mut rt = unify_basic(ty1, ty2, assignments);
            let rb = unify_basic(b1, b2, assignments);
            match (&mut rt, rb) {
                (UnifyResult::Solved(m1), UnifyResult::Solved(m2)) => {
                    m1.extend(m2);
                    rt
                }
                (UnifyResult::Fail(_), _) => rt,
                (_, UnifyResult::Fail(msg)) => UnifyResult::Fail(msg),
                _ => UnifyResult::Defer,
            }
        }
        _ => UnifyResult::Fail(format!("Cannot unify {:?} with {:?}", e1, e2)),
    }
}
/// The FVar ID offset that encodes metavariables (must match `context.rs`).
pub const MVAR_OFFSET: u64 = 1_000_000;
/// Apply known metavar assignments to an expression.
///
/// Metavariables are encoded as `Expr::FVar(FVarId(id))` where
/// `id >= MVAR_OFFSET`.  The key in `assignments` is `id - MVAR_OFFSET`.
pub fn apply_assignments_impl(expr: &Expr, assignments: &HashMap<MetaVarId, Expr>) -> Expr {
    match expr {
        Expr::FVar(fv) if fv.0 >= MVAR_OFFSET => {
            let meta_id = fv.0 - MVAR_OFFSET;
            if let Some(val) = assignments.get(&meta_id) {
                apply_assignments_impl(val, assignments)
            } else {
                expr.clone()
            }
        }
        Expr::App(f, a) => Expr::App(
            Box::new(apply_assignments_impl(f, assignments)),
            Box::new(apply_assignments_impl(a, assignments)),
        ),
        Expr::Lam(bi, name, ty, body) => Expr::Lam(
            *bi,
            name.clone(),
            Box::new(apply_assignments_impl(ty, assignments)),
            Box::new(apply_assignments_impl(body, assignments)),
        ),
        Expr::Pi(bi, name, ty, body) => Expr::Pi(
            *bi,
            name.clone(),
            Box::new(apply_assignments_impl(ty, assignments)),
            Box::new(apply_assignments_impl(body, assignments)),
        ),
        Expr::Let(name, ty, val, body) => Expr::Let(
            name.clone(),
            Box::new(apply_assignments_impl(ty, assignments)),
            Box::new(apply_assignments_impl(val, assignments)),
            Box::new(apply_assignments_impl(body, assignments)),
        ),
        Expr::Proj(name, idx, inner) => Expr::Proj(
            name.clone(),
            *idx,
            Box::new(apply_assignments_impl(inner, assignments)),
        ),
        _ => expr.clone(),
    }
}
/// Detect obvious conflicts in a list of constraints before solving.
///
/// Returns conflicts where two constants with different names are equated.
pub fn detect_obvious_conflicts(constraints: &[Constraint]) -> Vec<ConflictInfo> {
    let mut conflicts = Vec::new();
    for c in constraints {
        if let Constraint::Equal(e1, e2) = c {
            if let (Expr::Const(n1, _), Expr::Const(n2, _)) = (e1, e2) {
                if n1 != n2 {
                    conflicts.push(ConflictInfo::new(
                        e1.clone(),
                        e2.clone(),
                        format!("Cannot unify constant '{}' with '{}'", n1, n2),
                    ));
                }
            }
        }
    }
    conflicts
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::solver::*;
    #[test]
    fn test_solver_create() {
        let solver = ConstraintSolver::new();
        assert_eq!(solver.pending_count(), 0);
    }
    #[test]
    fn test_add_constraint() {
        let mut solver = ConstraintSolver::new();
        let c = Constraint::Equal(Expr::BVar(0), Expr::BVar(0));
        solver.add_constraint(c);
        assert_eq!(solver.pending_count(), 1);
    }
    #[test]
    fn test_solve_equal() {
        let mut solver = ConstraintSolver::new();
        let e = Expr::Lit(Literal::Nat(42));
        solver.add_constraint(Constraint::Equal(e.clone(), e));
        assert!(solver.solve().is_ok());
    }
    #[test]
    fn test_solve_assign() {
        let mut solver = ConstraintSolver::new();
        let expr = Expr::Lit(Literal::Nat(42));
        solver.add_constraint(Constraint::Assign(1, expr.clone()));
        solver.solve().expect("test operation should succeed");
        assert_eq!(solver.get_assignment(1), Some(&expr));
    }
    #[test]
    fn test_unify_same() {
        let e1 = Expr::Sort(Level::zero());
        let e2 = Expr::Sort(Level::zero());
        assert!(is_unifiable(&e1, &e2));
    }
    #[test]
    fn test_unify_app() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Lit(Literal::Nat(42));
        let e1 = Expr::App(Box::new(f.clone()), Box::new(a.clone()));
        let e2 = Expr::App(Box::new(f), Box::new(a));
        assert!(is_unifiable(&e1, &e2));
    }
    #[test]
    fn test_clear() {
        let mut solver = ConstraintSolver::new();
        solver.add_constraint(Constraint::Equal(Expr::BVar(0), Expr::BVar(0)));
        solver.clear();
        assert_eq!(solver.pending_count(), 0);
    }
    #[test]
    fn test_priority_solver_add_normal() {
        let mut solver = PrioritySolver::new();
        let e = Expr::Lit(Literal::Nat(1));
        solver.add_normal(Constraint::Equal(e.clone(), e));
        assert_eq!(solver.pending_count(), 1);
    }
    #[test]
    fn test_priority_solver_solve_assign() {
        let mut solver = PrioritySolver::new();
        let expr = Expr::Lit(Literal::Nat(99));
        solver.add_urgent_assign(42, expr.clone());
        solver.solve().expect("test operation should succeed");
        assert_eq!(solver.get_assignment(42), Some(&expr));
    }
    #[test]
    fn test_priority_solver_clear() {
        let mut solver = PrioritySolver::new();
        solver.add_normal(Constraint::Equal(Expr::BVar(0), Expr::BVar(0)));
        solver.clear();
        assert!(solver.is_empty());
    }
    #[test]
    fn test_scheduled_constraint_retry() {
        let mut sc = ScheduledConstraint::new(Constraint::Equal(Expr::BVar(0), Expr::BVar(0)));
        sc.max_retries = 2;
        assert!(!sc.is_exhausted());
        sc.increment_retry();
        sc.increment_retry();
        assert!(sc.is_exhausted());
    }
    #[test]
    fn test_priority_ordering() {
        assert!(ConstraintPriority::Urgent > ConstraintPriority::Normal);
        assert!(ConstraintPriority::Normal > ConstraintPriority::Deferred);
        assert!(ConstraintPriority::Deferred > ConstraintPriority::Postponed);
    }
    #[test]
    fn test_detect_obvious_conflicts_none() {
        let n = Name::str("Nat");
        let constraints = vec![Constraint::Equal(
            Expr::Const(n.clone(), vec![]),
            Expr::Const(n, vec![]),
        )];
        let conflicts = detect_obvious_conflicts(&constraints);
        assert!(conflicts.is_empty());
    }
    #[test]
    fn test_detect_obvious_conflicts_some() {
        let constraints = vec![Constraint::Equal(
            Expr::Const(Name::str("Nat"), vec![]),
            Expr::Const(Name::str("Int"), vec![]),
        )];
        let conflicts = detect_obvious_conflicts(&constraints);
        assert_eq!(conflicts.len(), 1);
        assert!(conflicts[0].message.contains("Nat"));
    }
    #[test]
    fn test_solver_stats_success_rate() {
        let mut stats = SolverStats::new();
        stats.total_processed = 10;
        stats.solved_immediate = 8;
        stats.solved_with_retry = 2;
        assert!((stats.success_rate() - 1.0).abs() < 1e-10);
        assert!(stats.all_solved());
    }
    #[test]
    fn test_solver_stats_partial() {
        let mut stats = SolverStats::new();
        stats.total_processed = 10;
        stats.solved_immediate = 5;
        stats.failed_total = 5;
        assert!(!stats.all_solved());
        assert!((stats.success_rate() - 0.5).abs() < 1e-10);
    }
    #[test]
    fn test_apply_assignments_app() {
        let assignments = HashMap::new();
        let e = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(Expr::Lit(Literal::Nat(1))),
        );
        let result = apply_assignments_impl(&e, &assignments);
        assert_eq!(result, e);
    }
    #[test]
    fn test_conflict_info_creation() {
        let lhs = Expr::Const(Name::str("A"), vec![]);
        let rhs = Expr::Const(Name::str("B"), vec![]);
        let info = ConflictInfo::new(lhs.clone(), rhs.clone(), "types differ");
        assert_eq!(info.message, "types differ");
        assert_eq!(info.lhs, lhs);
        assert_eq!(info.rhs, rhs);
    }
    #[test]
    fn test_priority_solver_equal_literals() {
        let mut solver = PrioritySolver::new();
        let e = Expr::Lit(Literal::Nat(7));
        solver.add_normal(Constraint::Equal(e.clone(), e));
        assert!(solver.solve().is_ok());
        assert_eq!(solver.num_solved, 1);
    }
}
/// Check if two constraint lists are equivalent up to ordering.
pub fn constraints_equivalent(a: &[Constraint], b: &[Constraint]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut used = vec![false; b.len()];
    'outer: for ca in a {
        for (j, cb) in b.iter().enumerate() {
            if !used[j] && ca == cb {
                used[j] = true;
                continue 'outer;
            }
        }
        return false;
    }
    true
}
/// Count constraints by kind.
pub fn count_constraint_kinds(constraints: &[Constraint]) -> (usize, usize, usize) {
    let mut eq = 0;
    let mut has_type = 0;
    let mut assign = 0;
    for c in constraints {
        match c {
            Constraint::Equal(_, _) => eq += 1,
            Constraint::HasType(_, _) => has_type += 1,
            Constraint::Assign(_, _) => assign += 1,
        }
    }
    (eq, has_type, assign)
}
/// Apply known assignments to a constraint.
pub fn apply_constraint_assignments(
    c: &Constraint,
    assignments: &HashMap<MetaVarId, Expr>,
) -> Constraint {
    match c {
        Constraint::Equal(e1, e2) => Constraint::Equal(
            apply_assignments_impl(e1, assignments),
            apply_assignments_impl(e2, assignments),
        ),
        Constraint::HasType(e, ty) => Constraint::HasType(
            apply_assignments_impl(e, assignments),
            apply_assignments_impl(ty, assignments),
        ),
        Constraint::Assign(m, e) => Constraint::Assign(*m, apply_assignments_impl(e, assignments)),
    }
}
/// Try to solve a single constraint, updating assignments if successful.
/// Returns `true` if the constraint was solved.
pub fn try_solve_constraint(c: &Constraint, assignments: &mut HashMap<MetaVarId, Expr>) -> bool {
    match c {
        Constraint::Equal(e1, e2) => {
            if e1 == e2 {
                return true;
            }
            matches!(
                unify_basic(e1, e2, assignments), UnifyResult::Solved(ref new_assigns) if
                { assignments.extend(new_assigns.clone()); true }
            )
        }
        Constraint::HasType(expr, ty) => check_has_type_basic(expr, ty).unwrap_or(true),
        Constraint::Assign(meta, expr) => {
            if let Some(existing) = assignments.get(meta) {
                existing == expr
            } else {
                assignments.insert(*meta, expr.clone());
                true
            }
        }
    }
}
/// Check obvious `HasType(expr, ty)` cases without a full type inference engine.
///
/// Returns `Some(true)` if we can confirm, `Some(false)` if we can refute,
/// and `None` if we cannot determine (requires full type inference).
pub fn check_has_type_basic(expr: &Expr, ty: &Expr) -> Option<bool> {
    let ty_is = |name: &str| matches!(ty, Expr::Const(n, _) if n == & Name::str(name));
    match expr {
        Expr::Lit(Literal::Nat(_)) => {
            if ty_is("Nat") || ty_is("Nat.nonemptyType") {
                return Some(true);
            }
            if ty_is("Bool") || ty_is("String") {
                return Some(false);
            }
        }
        Expr::Lit(Literal::Str(_)) => {
            if ty_is("String") {
                return Some(true);
            }
            if ty_is("Nat") || ty_is("Bool") {
                return Some(false);
            }
        }
        Expr::Sort(level) => {
            if let Expr::Sort(ty_level) = ty {
                let succ_l = Level::succ(level.clone());
                if ty_level == &succ_l {
                    return Some(true);
                }
            }
        }
        Expr::Pi(_, _, _, _) => {
            if matches!(ty, Expr::Sort(_)) {
                return Some(true);
            }
        }
        Expr::Lam(_, _, dom, _) => {
            if let Expr::Pi(_, _, pi_dom, _) = ty {
                if dom.as_ref() == pi_dom.as_ref() {
                    return None;
                }
            }
        }
        Expr::Const(name, _)
            if (name == &Name::str("Bool.true") || name == &Name::str("true")) && ty_is("Bool") =>
        {
            return Some(true);
        }
        Expr::Const(name, _)
            if (name == &Name::str("Bool.false") || name == &Name::str("false"))
                && ty_is("Bool") =>
        {
            return Some(true);
        }
        Expr::Const(name, _) if name == &Name::str("Nat.zero") && ty_is("Nat") => {
            return Some(true);
        }
        _ => {}
    }
    None
}
/// Normalize a constraint by applying all known metavariable assignments.
pub fn normalize_constraint(c: &Constraint, assignments: &HashMap<MetaVarId, Expr>) -> Constraint {
    apply_constraint_assignments(c, assignments)
}
/// Normalize a list of constraints.
pub fn normalize_constraints(
    constraints: &[Constraint],
    assignments: &HashMap<MetaVarId, Expr>,
) -> Vec<Constraint> {
    constraints
        .iter()
        .map(|c| normalize_constraint(c, assignments))
        .collect()
}
/// Remove trivially true constraints (`Equal(e, e)` where `e` is closed).
pub fn simplify_constraints(constraints: Vec<Constraint>) -> Vec<Constraint> {
    constraints
        .into_iter()
        .filter(|c| match c {
            Constraint::Equal(e1, e2) => e1 != e2,
            _ => true,
        })
        .collect()
}
/// Group constraints by kind for reporting.
pub fn group_constraints(
    constraints: &[Constraint],
) -> (Vec<&Constraint>, Vec<&Constraint>, Vec<&Constraint>) {
    let mut eq = Vec::new();
    let mut has_type = Vec::new();
    let mut assign = Vec::new();
    for c in constraints {
        match c {
            Constraint::Equal(_, _) => eq.push(c),
            Constraint::HasType(_, _) => has_type.push(c),
            Constraint::Assign(_, _) => assign.push(c),
        }
    }
    (eq, has_type, assign)
}
/// Format a constraint as a human-readable string.
pub fn format_constraint(c: &Constraint) -> String {
    match c {
        Constraint::Equal(e1, e2) => format!("{:?} =?= {:?}", e1, e2),
        Constraint::HasType(e, ty) => format!("{:?} : {:?}", e, ty),
        Constraint::Assign(m, e) => format!("?{} := {:?}", m, e),
    }
}
/// Format a list of constraints.
pub fn format_constraints(constraints: &[Constraint]) -> Vec<String> {
    constraints.iter().map(format_constraint).collect()
}
#[cfg(test)]
mod incremental_tests {
    use super::*;
    use crate::solver::*;
    #[test]
    fn test_incremental_solver_solve_assign() {
        let mut s = IncrementalSolver::new();
        let expr = Expr::Lit(Literal::Nat(42));
        s.add(Constraint::Assign(1, expr.clone()));
        s.step();
        assert_eq!(s.get_assignment(1), Some(&expr));
        assert!(s.is_complete());
    }
    #[test]
    fn test_incremental_solver_solve_equal() {
        let mut s = IncrementalSolver::new();
        let e = Expr::Const(Name::str("Nat"), vec![]);
        s.add(Constraint::Equal(e.clone(), e));
        s.step();
        assert!(s.is_complete());
    }
    #[test]
    fn test_incremental_solver_pending_count() {
        let mut s = IncrementalSolver::new();
        s.add(Constraint::Equal(Expr::BVar(0), Expr::BVar(1)));
        assert_eq!(s.pending_count(), 1);
    }
    #[test]
    fn test_incremental_solver_solve_all() {
        let mut s = IncrementalSolver::new();
        s.add(Constraint::Assign(1, Expr::Lit(Literal::Nat(1))));
        s.add(Constraint::Assign(2, Expr::Lit(Literal::Nat(2))));
        s.solve_all();
        assert!(s.is_complete());
        assert_eq!(s.num_solved, 2);
    }
    #[test]
    fn test_incremental_solver_clear() {
        let mut s = IncrementalSolver::new();
        s.add(Constraint::Assign(1, Expr::BVar(0)));
        s.clear();
        assert!(s.is_complete());
        assert_eq!(s.num_solved, 0);
    }
    #[test]
    fn test_normalize_constraint_eq() {
        let mut assignments = HashMap::new();
        assignments.insert(1u64, Expr::Const(Name::str("Nat"), vec![]));
        let c = Constraint::Equal(Expr::BVar(0), Expr::BVar(0));
        let nc = normalize_constraint(&c, &assignments);
        assert!(matches!(nc, Constraint::Equal(_, _)));
    }
    #[test]
    fn test_simplify_constraints() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let cs = vec![
            Constraint::Equal(nat.clone(), nat.clone()),
            Constraint::Assign(1, Expr::BVar(0)),
        ];
        let simplified = simplify_constraints(cs);
        assert_eq!(simplified.len(), 1);
    }
    #[test]
    fn test_group_constraints() {
        let cs = vec![
            Constraint::Equal(Expr::BVar(0), Expr::BVar(0)),
            Constraint::Assign(1, Expr::BVar(0)),
            Constraint::HasType(Expr::BVar(0), Expr::Sort(Level::zero())),
        ];
        let (eq, ht, as_) = group_constraints(&cs);
        assert_eq!(eq.len(), 1);
        assert_eq!(ht.len(), 1);
        assert_eq!(as_.len(), 1);
    }
    #[test]
    fn test_format_constraint() {
        let c = Constraint::Assign(42, Expr::BVar(0));
        let s = format_constraint(&c);
        assert!(s.contains("42"));
    }
    #[test]
    fn test_solver_state_assign_and_normalize() {
        let mut state = SolverState::new();
        let e = Expr::Const(Name::str("Nat"), vec![]);
        state.assign(0, e.clone());
        state.add_constraint(Constraint::Equal(e.clone(), e));
        state.normalize();
        assert!(state.is_solved());
    }
    #[test]
    fn test_solver_state_pending() {
        let mut state = SolverState::new();
        state.add_constraint(Constraint::Assign(1, Expr::BVar(0)));
        assert_eq!(state.pending(), 1);
        assert!(!state.is_solved());
    }
    #[test]
    fn test_solver_state_num_assignments() {
        let mut state = SolverState::new();
        state.assign(1, Expr::BVar(0));
        state.assign(2, Expr::BVar(1));
        assert_eq!(state.num_assignments(), 2);
    }
    #[test]
    fn test_constraints_equivalent_order() {
        let a = Expr::Const(Name::str("A"), vec![]);
        let b = Expr::Const(Name::str("B"), vec![]);
        let c1 = vec![
            Constraint::Equal(a.clone(), b.clone()),
            Constraint::Equal(b.clone(), a.clone()),
        ];
        let c2 = vec![
            Constraint::Equal(b.clone(), a.clone()),
            Constraint::Equal(a.clone(), b.clone()),
        ];
        assert!(constraints_equivalent(&c1, &c2));
    }
    #[test]
    fn test_count_constraint_kinds_all() {
        let cs = vec![
            Constraint::Equal(Expr::BVar(0), Expr::BVar(0)),
            Constraint::Equal(Expr::BVar(1), Expr::BVar(1)),
            Constraint::HasType(Expr::BVar(0), Expr::Sort(Level::zero())),
            Constraint::Assign(1, Expr::BVar(0)),
        ];
        let (eq, ht, as_) = count_constraint_kinds(&cs);
        assert_eq!(eq, 2);
        assert_eq!(ht, 1);
        assert_eq!(as_, 1);
    }
}
#[cfg(test)]
mod constraint_queue_tests {
    use super::*;
    use crate::solver::*;
    use oxilean_kernel::{FVarId, Level, Literal, Name};
    fn sort() -> Expr {
        Expr::Sort(Level::zero())
    }
    fn nat_lit(n: u64) -> Expr {
        Expr::Lit(Literal::Nat(n))
    }
    fn meta(id: u64) -> Expr {
        Expr::FVar(FVarId(MVAR_OFFSET + id))
    }
    #[test]
    fn test_constraint_queue_new() {
        let q = ConstraintQueue::new();
        assert!(q.is_empty());
        assert_eq!(q.len(), 0);
    }
    #[test]
    fn test_postpone_constraint_adds_to_queue() {
        let mut q = ConstraintQueue::new();
        q.postpone_constraint(sort(), sort());
        assert_eq!(q.len(), 1);
        assert_eq!(q.total_postponed, 1);
    }
    #[test]
    fn test_retry_postponed_trivial_equal() {
        let mut q = ConstraintQueue::new();
        q.postpone_constraint(sort(), sort());
        let mut assignments = HashMap::new();
        let solved = q.retry_postponed(&mut assignments);
        assert_eq!(solved, 1);
        assert!(q.is_empty());
        assert_eq!(q.total_retried, 1);
    }
    #[test]
    fn test_retry_postponed_constant_mismatch_stays() {
        let mut q = ConstraintQueue::new();
        let a = Expr::Const(Name::str("A"), vec![]);
        let b = Expr::Const(Name::str("B"), vec![]);
        q.postpone_constraint(a, b);
        let mut assignments = HashMap::new();
        let solved = q.retry_postponed(&mut assignments);
        assert_eq!(solved, 0);
        assert_eq!(q.len(), 1);
    }
    #[test]
    fn test_retry_postponed_meta_solved_by_assignment() {
        let mut q = ConstraintQueue::new();
        let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
        q.postpone_constraint(meta(0), nat_ty.clone());
        let mut assignments = HashMap::new();
        let solved_before = q.retry_postponed(&mut assignments);
        let _ = solved_before;
        q.clear();
        q.postpone_constraint(meta(0), nat_ty.clone());
        assignments.insert(0, nat_ty.clone());
        let solved = q.retry_postponed(&mut assignments);
        assert_eq!(solved, 1);
        assert!(q.is_empty());
    }
    #[test]
    fn test_retry_until_stable_no_progress() {
        let mut q = ConstraintQueue::new();
        let a = Expr::Const(Name::str("A"), vec![]);
        let b = Expr::Const(Name::str("B"), vec![]);
        q.postpone_constraint(a, b);
        let mut assignments = HashMap::new();
        let total = q.retry_until_stable(&mut assignments);
        assert_eq!(total, 0);
    }
    #[test]
    fn test_retry_until_stable_with_progress() {
        let mut q = ConstraintQueue::new();
        let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
        q.postpone_constraint(meta(0), nat_ty.clone());
        q.postpone_constraint(meta(1), nat_ty.clone());
        let mut assignments = HashMap::new();
        assignments.insert(0, nat_ty.clone());
        assignments.insert(1, nat_ty.clone());
        let total = q.retry_until_stable(&mut assignments);
        assert_eq!(total, 2);
        assert!(q.is_empty());
    }
    #[test]
    fn test_constraint_queue_clear() {
        let mut q = ConstraintQueue::new();
        q.postpone_constraint(sort(), sort());
        q.postpone_constraint(sort(), sort());
        q.clear();
        assert!(q.is_empty());
        assert_eq!(q.total_postponed, 2);
    }
    #[test]
    fn test_constraint_queue_drain() {
        let mut q = ConstraintQueue::new();
        q.postpone_constraint(nat_lit(1), nat_lit(1));
        q.postpone_constraint(nat_lit(2), nat_lit(2));
        let drained = q.drain();
        assert_eq!(drained.len(), 2);
        assert!(q.is_empty());
    }
    #[test]
    fn test_constraint_queue_iter() {
        let mut q = ConstraintQueue::new();
        q.postpone_constraint(sort(), sort());
        let pairs: Vec<_> = q.iter().collect();
        assert_eq!(pairs.len(), 1);
    }
    #[test]
    fn test_postpone_and_retry_new_assignments_propagate() {
        let mut q = ConstraintQueue::new();
        q.postpone_constraint(meta(0), meta(1));
        let mut assignments = HashMap::new();
        assignments.insert(0u64, Expr::Const(Name::str("Nat"), vec![]));
        let solved = q.retry_postponed(&mut assignments);
        assert_eq!(solved, 1);
        assert!(q.is_empty());
    }
    #[test]
    fn test_retry_updates_assignments() {
        let mut q = ConstraintQueue::new();
        let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
        q.postpone_constraint(meta(0), nat_ty.clone());
        let mut assignments = HashMap::new();
        let solved = q.retry_postponed(&mut assignments);
        if solved == 1 {
            assert!(assignments.contains_key(&0));
        }
    }
}
#[cfg(test)]
mod solver_ext_tests {
    use super::*;
    use crate::solver::*;
    use oxilean_kernel::FVarId;
    fn nat_const() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn bool_const() -> Expr {
        Expr::Const(Name::str("Bool"), vec![])
    }
    fn sort0() -> Expr {
        Expr::Sort(Level::zero())
    }
    fn meta(id: MetaVarId) -> Expr {
        Expr::FVar(FVarId(MVAR_OFFSET + id))
    }
    #[test]
    fn test_solver_diag_levels() {
        let d = SolverDiagnostic::error("type mismatch");
        assert!(d.is_error());
        let d2 = SolverDiagnostic::info("solved").with_constraint_id(42);
        assert!(!d2.is_error());
        assert_eq!(d2.constraint_id, Some(42));
    }
    #[test]
    fn test_diag_collector() {
        let mut col = SolverDiagCollector::new();
        col.add_info("started");
        col.add_warning("slow");
        col.add_error("failed");
        assert!(col.has_errors());
        assert_eq!(col.error_count(), 1);
        assert_eq!(col.warning_count(), 1);
        let errs = col.drain_errors();
        assert_eq!(errs.len(), 1);
        assert!(!col.has_errors());
    }
    #[test]
    fn test_constraint_graph() {
        let mut g = ConstraintGraph::new();
        let id1 = g.add_node(vec![], vec![0u64, 1u64]);
        let id2 = g.add_node(vec![0], vec![]);
        assert_eq!(g.num_nodes(), 2);
        let ready = g.ready_nodes();
        assert!(ready.contains(&id2));
        assert!(!ready.contains(&id1));
        let unblocked = g.unblock(0);
        assert!(unblocked.contains(&id1));
    }
    #[test]
    fn test_meta_var_context() {
        let mut ctx = MetaVarContext::new();
        let m0 = ctx.fresh(0);
        let m1 = ctx.fresh(1);
        assert!(!ctx.is_assigned(m0));
        assert_eq!(ctx.unassigned_count(), 2);
        assert!(ctx.assign(m0, nat_const()));
        assert!(ctx.is_assigned(m0));
        assert!(!ctx.is_assigned(m1));
        assert_eq!(ctx.unassigned_count(), 1);
        assert!(!ctx.assign(m0, bool_const()));
    }
    #[test]
    fn test_checkpoint_manager() {
        let mut mgr = CheckpointManager::new();
        let mut assignments = HashMap::new();
        assignments.insert(0u64, nat_const());
        let cid = mgr.save(&assignments, 3);
        assert_eq!(mgr.depth(), 1);
        let restored = mgr.restore(cid).expect("test operation should succeed");
        assert_eq!(restored.pending_count, 3);
        assert!(restored.assignments_snapshot.contains_key(&0));
        assert_eq!(mgr.depth(), 0);
    }
    #[test]
    fn test_solver_event_log() {
        let mut log = SolverEventLog::new(true);
        log.emit(SolverEventKind::SolveStarted);
        log.emit(SolverEventKind::MetaAssigned { meta: 0 });
        log.emit(SolverEventKind::MetaAssigned { meta: 1 });
        log.emit(SolverEventKind::SolveFinished { success: true });
        assert_eq!(log.count(), 4);
        assert_eq!(log.assignments_emitted(), 2);
        log.clear();
        assert_eq!(log.count(), 0);
    }
    #[test]
    fn test_event_log_disabled() {
        let mut log = SolverEventLog::new(false);
        log.emit(SolverEventKind::SolveStarted);
        assert_eq!(log.count(), 0);
    }
    #[test]
    fn test_occurs_check_direct() {
        let assignments = HashMap::new();
        assert!(OccursCheck::occurs(0, &meta(0), &assignments));
        assert!(!OccursCheck::occurs(0, &meta(1), &assignments));
        assert!(!OccursCheck::occurs(0, &nat_const(), &assignments));
    }
    #[test]
    fn test_occurs_check_in_app() {
        let assignments = HashMap::new();
        let app = Expr::App(Box::new(nat_const()), Box::new(meta(0)));
        assert!(OccursCheck::occurs(0, &app, &assignments));
        assert!(!OccursCheck::occurs(1, &app, &assignments));
    }
    #[test]
    fn test_constraint_simplifier_trivial() {
        let trivial = Constraint::Equal(nat_const(), nat_const());
        let non_trivial = Constraint::Equal(nat_const(), bool_const());
        let result = ConstraintSimplifier::simplify(vec![trivial, non_trivial]);
        assert_eq!(result.len(), 1);
    }
    #[test]
    fn test_solver_config_default() {
        let cfg = SolverConfig::default();
        assert!(cfg.occurs_check);
        assert!(cfg.enable_postponing);
        assert!(!cfg.strict_mode);
    }
    #[test]
    fn test_solver_config_strict() {
        let cfg = SolverConfig::strict();
        assert!(cfg.strict_mode);
        assert!(cfg.occurs_check);
    }
    #[test]
    fn test_solver_report_success() {
        let r = SolverReport::success(5, 10, 3);
        assert!(r.success);
        assert_eq!(r.assignments_made, 5);
        assert!(!r.has_warnings());
    }
    #[test]
    fn test_solver_report_failure() {
        let r = SolverReport::failure(vec!["type mismatch".to_string()]);
        assert!(!r.success);
        assert_eq!(r.errors.len(), 1);
    }
    #[test]
    fn test_sort0_trivial_constraint() {
        assert!(ConstraintSimplifier::is_trivial(&Constraint::Equal(
            sort0(),
            sort0()
        )));
        assert!(!ConstraintSimplifier::is_trivial(&Constraint::Equal(
            sort0(),
            nat_const()
        )));
    }
    #[test]
    fn test_meta_var_all_assigned() {
        let mut ctx = MetaVarContext::new();
        let m0 = ctx.fresh(0);
        assert!(!ctx.all_assigned());
        ctx.assign(m0, nat_const());
        assert!(ctx.all_assigned());
    }
}
#[allow(dead_code)]
pub trait SolverPhase: Send + Sync {
    fn name(&self) -> &'static str;
    fn run(
        &self,
        constraints: Vec<Constraint>,
        assignments: &mut HashMap<MetaVarId, Expr>,
    ) -> (Vec<Constraint>, Vec<String>);
}
#[allow(dead_code)]
pub fn solver_extension_version() -> &'static str {
    "oxilean-elab-solver-extension-v1"
}
#[allow(dead_code)]
pub fn solver_supports_occurs_check() -> bool {
    true
}
#[allow(dead_code)]
pub fn solver_supports_postponing() -> bool {
    true
}
#[cfg(test)]
mod pipeline_tests {
    use super::*;
    use crate::solver::*;
    #[test]
    fn test_simplification_phase() {
        let phase = SimplificationPhase;
        assert_eq!(phase.name(), "simplification");
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let bool_ty = Expr::Const(Name::str("Bool"), vec![]);
        let constraints = vec![
            Constraint::Equal(nat.clone(), nat.clone()),
            Constraint::Equal(nat.clone(), bool_ty.clone()),
        ];
        let mut assignments = HashMap::new();
        let (remaining, errors) = phase.run(constraints, &mut assignments);
        assert_eq!(remaining.len(), 1);
        assert!(errors.is_empty());
    }
    #[test]
    fn test_solver_pipeline_empty() {
        let pipeline = SolverPipeline::new();
        assert_eq!(pipeline.num_phases(), 0);
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let constraints = vec![Constraint::Equal(nat.clone(), nat.clone())];
        let mut assignments = HashMap::new();
        let (remaining, errors) = pipeline.run(constraints, &mut assignments);
        assert_eq!(remaining.len(), 1);
        assert!(errors.is_empty());
    }
    #[test]
    fn test_solver_pipeline_with_simplification() {
        let mut pipeline = SolverPipeline::new();
        pipeline.add_phase(Box::new(SimplificationPhase));
        assert_eq!(pipeline.num_phases(), 1);
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let constraints = vec![Constraint::Equal(nat.clone(), nat.clone())];
        let mut assignments = HashMap::new();
        let (remaining, _) = pipeline.run(constraints, &mut assignments);
        assert_eq!(remaining.len(), 0);
    }
    #[test]
    fn test_solver_extension_version() {
        assert!(!solver_extension_version().is_empty());
        assert!(solver_supports_occurs_check());
        assert!(solver_supports_postponing());
    }
}
