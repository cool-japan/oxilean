//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};

use super::metavarcontext_type::MetaVarContext;
use super::types::{
    ConstraintQueue, DelayedAssignment, DelayedAssignmentQueue, MetaAssignmentHistory,
    MetaConstraint, MetaEqClass, MetaSubstitution, MetaVar, MetaVarEvent, MetaVarGraph,
    MetaVarGroup, MetaVarKind, MetaVarLog, MetaVarPool, MetaVarPriority, MetaVarStats,
    PendingConstraint, RichMetaContext, UnificationResult,
};
use oxilean_kernel::{Expr, FVarId, Level};

/// Check whether an expression mentions any metavariable.
pub fn expr_has_meta(expr: &Expr, _id: u64) -> bool {
    walk_expr_for_meta(expr)
}
fn walk_expr_for_meta(expr: &Expr) -> bool {
    match expr {
        Expr::App(f, a) => walk_expr_for_meta(f) || walk_expr_for_meta(a),
        Expr::Lam(_, _, ty, body) => walk_expr_for_meta(ty) || walk_expr_for_meta(body),
        Expr::Pi(_, _, ty, body) => walk_expr_for_meta(ty) || walk_expr_for_meta(body),
        Expr::Let(_, ty, val, body) => {
            walk_expr_for_meta(ty) || walk_expr_for_meta(val) || walk_expr_for_meta(body)
        }
        _ => false,
    }
}
/// Collect metavariable IDs referenced in an expression.
pub fn collect_metas(expr: &Expr) -> HashSet<u64> {
    let mut result = HashSet::new();
    collect_metas_helper(expr, &mut result);
    result
}
#[allow(clippy::only_used_in_recursion)]
fn collect_metas_helper(expr: &Expr, acc: &mut HashSet<u64>) {
    match expr {
        Expr::App(f, a) => {
            collect_metas_helper(f, acc);
            collect_metas_helper(a, acc);
        }
        Expr::Lam(_, _, ty, body) => {
            collect_metas_helper(ty, acc);
            collect_metas_helper(body, acc);
        }
        Expr::Pi(_, _, ty, body) => {
            collect_metas_helper(ty, acc);
            collect_metas_helper(body, acc);
        }
        Expr::Let(_, ty, val, body) => {
            collect_metas_helper(ty, acc);
            collect_metas_helper(val, acc);
            collect_metas_helper(body, acc);
        }
        _ => {}
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::metavar::*;
    fn sort() -> Expr {
        Expr::Sort(Level::zero())
    }
    #[test]
    fn test_fresh_meta() {
        let mut ctx = MetaVarContext::new();
        let id1 = ctx.fresh(sort());
        let id2 = ctx.fresh(sort());
        assert_ne!(id1, id2);
        assert_eq!(ctx.count(), 2);
    }
    #[test]
    fn test_assign_meta() {
        let mut ctx = MetaVarContext::new();
        let id = ctx.fresh(sort());
        assert!(!ctx.is_solved(id));
        ctx.assign(id, sort());
        assert!(ctx.is_solved(id));
        assert_eq!(ctx.get_assignment(id), Some(&sort()));
    }
    #[test]
    fn test_unsolved() {
        let mut ctx = MetaVarContext::new();
        let id1 = ctx.fresh(sort());
        let id2 = ctx.fresh(sort());
        let id3 = ctx.fresh(sort());
        ctx.assign(id1, sort());
        assert_eq!(ctx.unsolved_count(), 2);
        let unsolved = ctx.unsolved();
        assert!(unsolved.contains(&id2));
        assert!(unsolved.contains(&id3));
    }
    #[test]
    fn test_frozen_meta_cannot_be_assigned() {
        let mut ctx = MetaVarContext::new();
        let id = ctx.fresh(sort());
        ctx.freeze(id);
        assert!(ctx.is_frozen(id));
        assert!(!ctx.assign(id, sort()));
        assert!(!ctx.is_solved(id));
    }
    #[test]
    fn test_force_assign_ignores_freeze() {
        let mut ctx = MetaVarContext::new();
        let id = ctx.fresh(sort());
        ctx.freeze(id);
        assert!(ctx.force_assign(id, sort()));
        assert!(ctx.is_solved(id));
    }
    #[test]
    fn test_snapshot_restore() {
        let mut ctx = MetaVarContext::new();
        let id1 = ctx.fresh(sort());
        let snap = ctx.snapshot();
        let _id2 = ctx.fresh(sort());
        ctx.assign(id1, sort());
        assert_eq!(ctx.count(), 2);
        ctx.restore(&snap);
        assert_eq!(ctx.count(), 1);
        assert!(!ctx.is_solved(id1));
    }
    #[test]
    fn test_all_solved() {
        let mut ctx = MetaVarContext::new();
        let id1 = ctx.fresh(sort());
        let id2 = ctx.fresh(sort());
        assert!(!ctx.all_solved());
        ctx.assign(id1, sort());
        ctx.assign(id2, sort());
        assert!(ctx.all_solved());
    }
    #[test]
    fn test_fresh_kinds() {
        let mut ctx = MetaVarContext::new();
        let id_nat = ctx.fresh(sort());
        let id_syn = ctx.fresh_synthetic(sort());
        let id_opa = ctx.fresh_opaque(sort());
        assert!(ctx.get(id_nat).expect("key should exist").is_natural());
        assert!(ctx.get(id_syn).expect("key should exist").is_synthetic());
        assert!(ctx
            .get(id_opa)
            .expect("key should exist")
            .is_synthetic_opaque());
    }
    #[test]
    fn test_constraints() {
        let mut ctx = MetaVarContext::new();
        ctx.add_constraint(MetaConstraint::new_eq(sort(), sort(), "test"));
        assert_eq!(ctx.constraint_count(), 1);
        ctx.clear_constraints();
        assert_eq!(ctx.constraint_count(), 0);
    }
    #[test]
    fn test_summary() {
        let mut ctx = MetaVarContext::new();
        let id = ctx.fresh(sort());
        ctx.assign(id, sort());
        let s = ctx.summary();
        assert!(s.contains("total: 1"));
        assert!(s.contains("solved: 1"));
    }
    #[test]
    fn test_depth() {
        let mut ctx = MetaVarContext::new();
        assert_eq!(ctx.depth(), 0);
        ctx.push_depth();
        ctx.push_depth();
        assert_eq!(ctx.depth(), 2);
        ctx.pop_depth();
        assert_eq!(ctx.depth(), 1);
    }
    #[test]
    fn test_named_meta_display() {
        let mut ctx = MetaVarContext::new();
        let id = ctx.fresh_named(sort(), "myHole");
        assert_eq!(
            ctx.get(id).expect("key should exist").display_name(),
            "myHole"
        );
    }
}
/// Substitute all assigned metavariables in an expression (top-level only).
///
/// Performs one pass of substitution — for full normalization, repeat until
/// no change.
pub fn subst_metas_once(expr: &Expr, ctx: &MetaVarContext) -> (Expr, bool) {
    match expr {
        Expr::FVar(fvar_id) => {
            let meta_id = fvar_id.0.saturating_sub(1_000_000);
            if fvar_id.0 >= 1_000_000 {
                if let Some(val) = ctx.get_assignment(meta_id) {
                    return (val.clone(), true);
                }
            }
            (expr.clone(), false)
        }
        Expr::App(f, a) => {
            let (f2, c1) = subst_metas_once(f, ctx);
            let (a2, c2) = subst_metas_once(a, ctx);
            (Expr::App(Box::new(f2), Box::new(a2)), c1 || c2)
        }
        Expr::Lam(i, n, ty, body) => {
            let (ty2, c1) = subst_metas_once(ty, ctx);
            let (body2, c2) = subst_metas_once(body, ctx);
            (
                Expr::Lam(*i, n.clone(), Box::new(ty2), Box::new(body2)),
                c1 || c2,
            )
        }
        Expr::Pi(i, n, ty, body) => {
            let (ty2, c1) = subst_metas_once(ty, ctx);
            let (body2, c2) = subst_metas_once(body, ctx);
            (
                Expr::Pi(*i, n.clone(), Box::new(ty2), Box::new(body2)),
                c1 || c2,
            )
        }
        Expr::Let(n, ty, val, body) => {
            let (ty2, c1) = subst_metas_once(ty, ctx);
            let (val2, c2) = subst_metas_once(val, ctx);
            let (body2, c3) = subst_metas_once(body, ctx);
            (
                Expr::Let(n.clone(), Box::new(ty2), Box::new(val2), Box::new(body2)),
                c1 || c2 || c3,
            )
        }
        _ => (expr.clone(), false),
    }
}
/// Repeatedly substitute until no change (fixed-point).
pub fn subst_metas_full(expr: &Expr, ctx: &MetaVarContext) -> Expr {
    let (mut current, mut changed) = subst_metas_once(expr, ctx);
    while changed {
        let (next, c) = subst_metas_once(&current, ctx);
        current = next;
        changed = c;
    }
    current
}
/// Count metavariable occurrences (encoded as FVars with ID >= 1_000_000).
pub fn count_meta_occurrences(expr: &Expr) -> usize {
    match expr {
        Expr::FVar(id) if id.0 >= 1_000_000 => 1,
        Expr::App(f, a) => count_meta_occurrences(f) + count_meta_occurrences(a),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            count_meta_occurrences(ty) + count_meta_occurrences(body)
        }
        Expr::Let(_, ty, val, body) => {
            count_meta_occurrences(ty) + count_meta_occurrences(val) + count_meta_occurrences(body)
        }
        _ => 0,
    }
}
/// Check whether an expression contains any unassigned metavariable.
pub fn has_unassigned_meta(expr: &Expr, ctx: &MetaVarContext) -> bool {
    match expr {
        Expr::FVar(id) if id.0 >= 1_000_000 => {
            let meta_id = id.0 - 1_000_000;
            !ctx.is_assigned(meta_id)
        }
        Expr::App(f, a) => has_unassigned_meta(f, ctx) || has_unassigned_meta(a, ctx),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            has_unassigned_meta(ty, ctx) || has_unassigned_meta(body, ctx)
        }
        Expr::Let(_, ty, val, body) => {
            has_unassigned_meta(ty, ctx)
                || has_unassigned_meta(val, ctx)
                || has_unassigned_meta(body, ctx)
        }
        _ => false,
    }
}
/// Build a human-readable table of all metavariables and their statuses.
pub fn meta_status_table(ctx: &MetaVarContext) -> Vec<String> {
    let mut rows: Vec<String> = ctx
        .metas
        .iter()
        .map(|(id, m)| {
            let status = if m.is_assigned() {
                "assigned"
            } else {
                "pending"
            };
            let frozen = if ctx.frozen.contains(id) {
                " [frozen]"
            } else {
                ""
            };
            format!("?{} : {:?} — {}{}", id, m.ty, status, frozen)
        })
        .collect();
    rows.sort();
    rows
}
#[cfg(test)]
mod rich_tests {
    use super::*;
    use crate::metavar::*;
    use oxilean_kernel::{FVarId, Level};
    fn sort() -> Expr {
        Expr::Sort(Level::zero())
    }
    #[test]
    fn test_rich_context_fresh_assign() {
        let mut ctx = RichMetaContext::new();
        ctx.enable_logging();
        let id = ctx.fresh(sort());
        assert!(ctx.assign(id, sort()));
        assert_eq!(ctx.get(id), Some(&sort()));
        assert_eq!(ctx.log().len(), 1);
        assert_eq!(ctx.log()[0].id, id);
    }
    #[test]
    fn test_rich_context_constraints() {
        let mut ctx = RichMetaContext::new();
        ctx.push_constraint(PendingConstraint::Unification {
            lhs: sort(),
            rhs: sort(),
        });
        ctx.push_constraint(PendingConstraint::Typing {
            expr: sort(),
            ty: sort(),
        });
        assert_eq!(ctx.pending_count(), 2);
        assert_eq!(ctx.unification_count(), 1);
        let cs = ctx.take_constraints();
        assert_eq!(cs.len(), 2);
        assert_eq!(ctx.pending_count(), 0);
    }
    #[test]
    fn test_rich_context_snapshot_restore() {
        let mut ctx = RichMetaContext::new();
        let id = ctx.fresh(sort());
        let snap = ctx.snapshot();
        assert!(ctx.assign(id, sort()));
        ctx.restore(snap);
        assert!(!ctx.inner.is_assigned(id));
    }
    #[test]
    fn test_pending_constraint_kinds() {
        let u = PendingConstraint::Unification {
            lhs: sort(),
            rhs: sort(),
        };
        let t = PendingConstraint::Typing {
            expr: sort(),
            ty: sort(),
        };
        let d = PendingConstraint::Delayed {
            id: 0,
            expr: sort(),
        };
        assert!(u.is_unification());
        assert!(t.is_typing());
        assert!(d.is_delayed());
    }
    #[test]
    fn test_count_meta_occurrences() {
        let meta_expr = Expr::FVar(FVarId(1_000_001));
        assert_eq!(count_meta_occurrences(&meta_expr), 1);
        let app = Expr::App(Box::new(meta_expr.clone()), Box::new(meta_expr.clone()));
        assert_eq!(count_meta_occurrences(&app), 2);
    }
    #[test]
    fn test_is_fully_assigned() {
        let mut ctx = RichMetaContext::new();
        let id = ctx.fresh(sort());
        assert!(!ctx.is_fully_assigned());
        ctx.assign(id, sort());
        assert!(ctx.is_fully_assigned());
    }
    #[test]
    fn test_unassigned_ids() {
        let mut ctx = RichMetaContext::new();
        let id1 = ctx.fresh(sort());
        let id2 = ctx.fresh(sort());
        ctx.assign(id1, sort());
        let unassigned = ctx.unassigned_ids();
        assert!(!unassigned.contains(&id1));
        assert!(unassigned.contains(&id2));
    }
    #[test]
    fn test_meta_status_table() {
        let mut ctx = MetaVarContext::new();
        ctx.fresh(sort());
        let table = meta_status_table(&ctx);
        assert_eq!(table.len(), 1);
        assert!(table[0].contains("pending"));
    }
    #[test]
    fn test_depth_tracking_rich() {
        let mut ctx = RichMetaContext::new();
        assert_eq!(ctx.depth(), 0);
        ctx.push_depth();
        ctx.push_depth();
        assert_eq!(ctx.depth(), 2);
        ctx.pop_depth();
        assert_eq!(ctx.depth(), 1);
    }
}
/// Determine whether meta `id` occurs (directly or transitively) in `expr`.
///
/// Uses the encoding where metas are FVars with ID offset 1_000_000.
pub fn occurs_check(id: u64, expr: &Expr, ctx: &MetaVarContext) -> bool {
    let fvar_id = 1_000_000 + id;
    occurs_in_expr(fvar_id, expr, ctx, &mut HashSet::new())
}
fn occurs_in_expr(
    target_fvar: u64,
    expr: &Expr,
    ctx: &MetaVarContext,
    visited: &mut HashSet<u64>,
) -> bool {
    match expr {
        Expr::FVar(fid) => {
            if fid.0 == target_fvar {
                return true;
            }
            if fid.0 >= 1_000_000 {
                let mid = fid.0 - 1_000_000;
                if !visited.contains(&mid) {
                    visited.insert(mid);
                    if let Some(val) = ctx.get_assignment(mid) {
                        return occurs_in_expr(target_fvar, val, ctx, visited);
                    }
                }
            }
            false
        }
        Expr::App(f, a) => {
            occurs_in_expr(target_fvar, f, ctx, visited)
                || occurs_in_expr(target_fvar, a, ctx, visited)
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            occurs_in_expr(target_fvar, ty, ctx, visited)
                || occurs_in_expr(target_fvar, body, ctx, visited)
        }
        Expr::Let(_, ty, val, body) => {
            occurs_in_expr(target_fvar, ty, ctx, visited)
                || occurs_in_expr(target_fvar, val, ctx, visited)
                || occurs_in_expr(target_fvar, body, ctx, visited)
        }
        _ => false,
    }
}
/// Check that all non-meta FVar IDs in `expr` appear in `scope`.
///
/// Meta-encoded FVars (`id >= 1_000_000`) are unconditionally allowed.
/// Returns `true` if every non-meta FVar in `expr` is listed in `scope`.
pub fn scope_check_expr(expr: &Expr, scope: &[u64]) -> bool {
    scope_check_impl(expr, scope)
}
fn scope_check_impl(expr: &Expr, scope: &[u64]) -> bool {
    match expr {
        Expr::FVar(FVarId(id)) => {
            if *id >= 1_000_000 {
                return true;
            }
            scope.contains(id)
        }
        Expr::App(f, a) => scope_check_impl(f, scope) && scope_check_impl(a, scope),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            scope_check_impl(ty, scope) && scope_check_impl(body, scope)
        }
        Expr::Let(_, ty, val, body) => {
            scope_check_impl(ty, scope)
                && scope_check_impl(val, scope)
                && scope_check_impl(body, scope)
        }
        Expr::Proj(_, _, inner) => scope_check_impl(inner, scope),
        _ => true,
    }
}
/// Attempt assignment with an occurs check (to avoid circular assignments).
///
/// Returns `false` if the occurs check fails, the metavar is frozen, or it
/// does not exist.  The occurs check is also run by `MetaVarContext::assign`
/// internally, so this function is a thin convenience wrapper.
pub fn safe_assign(id: u64, value: &Expr, ctx: &mut MetaVarContext) -> bool {
    ctx.assign(id, value.clone())
}
/// Merge two `MetaVarContext`s by taking all assignments from `other` and
/// applying them to `base`. Returns the number of new assignments applied.
pub fn merge_contexts(base: &mut MetaVarContext, other: &MetaVarContext) -> usize {
    let mut count = 0;
    for (id, meta) in &other.metas {
        if let Some(val) = &meta.assignment {
            if !base.is_solved(*id) && base.assign(*id, val.clone()) {
                count += 1;
            }
        }
    }
    count
}
#[cfg(test)]
mod meta_extra_tests {
    use super::*;
    use crate::metavar::*;
    fn sort() -> Expr {
        Expr::Sort(Level::zero())
    }
    #[test]
    fn test_checkpoint_rollback() {
        let mut ctx = MetaVarContext::new();
        let id1 = ctx.fresh(sort());
        let cp = ctx.checkpoint();
        let id2 = ctx.fresh(sort());
        assert!(ctx.metas.contains_key(&id2));
        ctx.rollback(cp.clone());
        assert!(!ctx.metas.contains_key(&id2));
        assert!(ctx.metas.contains_key(&id1));
    }
    #[test]
    fn test_assigned_at_depth() {
        let mut ctx = MetaVarContext::new();
        let id = ctx.fresh(sort());
        ctx.assign(id, sort());
        assert_eq!(ctx.assigned_at_depth(0), 1);
        assert_eq!(ctx.assigned_at_depth(0xFFFF), 1);
    }
    #[test]
    fn test_metas_of_kind() {
        let mut ctx = MetaVarContext::new();
        let _id1 = ctx.fresh(sort());
        let _id2 = ctx.fresh_synthetic(sort());
        let natural = ctx.metas_of_kind(&MetaVarKind::Natural);
        let synthetic = ctx.metas_of_kind(&MetaVarKind::Synthetic);
        assert_eq!(natural.len(), 1);
        assert_eq!(synthetic.len(), 1);
    }
    #[test]
    fn test_assignment_map() {
        let mut ctx = MetaVarContext::new();
        let id = ctx.fresh(sort());
        ctx.assign(id, sort());
        let map = ctx.assignment_map();
        assert!(map.contains_key(&id));
    }
    #[test]
    fn test_safe_assign_no_occurs() {
        let mut ctx = MetaVarContext::new();
        let id = ctx.fresh(sort());
        assert!(safe_assign(id, &sort(), &mut ctx));
    }
    #[test]
    fn test_occurs_check_simple() {
        let mut ctx = MetaVarContext::new();
        let id = ctx.fresh(sort());
        let meta_expr = Expr::FVar(FVarId(1_000_000 + id));
        assert!(occurs_check(id, &meta_expr, &ctx));
    }
    #[test]
    fn test_merge_contexts() {
        let mut base = MetaVarContext::new();
        let other = MetaVarContext::new();
        let _id = base.fresh(sort());
        let count = merge_contexts(&mut base, &other);
        assert_eq!(count, 0);
    }
    #[test]
    fn test_debug_summary() {
        let mut ctx = MetaVarContext::new();
        ctx.fresh(sort());
        let s = ctx.debug_summary();
        assert!(s.contains("MetaVarContext"));
    }
}
#[cfg(test)]
mod metavar_extended_tests {
    use super::*;
    use crate::metavar::*;
    use oxilean_kernel::{Expr, Level};
    fn sort() -> Expr {
        Expr::Sort(Level::zero())
    }
    #[test]
    fn test_metavar_graph_add_edge() {
        let mut g = MetaVarGraph::new();
        g.add_edge(1, 2);
        g.add_edge(1, 3);
        assert!(g.has_dependents(1));
        assert_eq!(g.dependents_of(1).len(), 2);
        assert_eq!(g.edge_count(), 2);
    }
    #[test]
    fn test_metavar_graph_remove_node() {
        let mut g = MetaVarGraph::new();
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        g.remove_node(2);
        assert!(!g.has_dependents(2));
        assert!(!g.dependents_of(1).contains(&2));
    }
    #[test]
    fn test_metavar_graph_transitive() {
        let mut g = MetaVarGraph::new();
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        g.add_edge(3, 4);
        let deps = g.transitive_dependents(1);
        assert!(deps.contains(&2));
        assert!(deps.contains(&3));
        assert!(deps.contains(&4));
    }
    #[test]
    fn test_assignment_history_record() {
        let mut h = MetaAssignmentHistory::new();
        h.record(0, &sort());
        h.record(1, &sort());
        assert_eq!(h.len(), 2);
        assert_eq!(h.current_step(), 2);
        assert_eq!(h.find(0).len(), 1);
    }
    #[test]
    fn test_assignment_history_clear() {
        let mut h = MetaAssignmentHistory::new();
        h.record(0, &sort());
        h.clear();
        assert!(h.is_empty());
        assert_eq!(h.current_step(), 0);
    }
    #[test]
    fn test_delayed_assignment_is_ready() {
        let mut ctx = MetaVarContext::new();
        let id = ctx.fresh(sort());
        ctx.assign(id, sort());
        let da = DelayedAssignment::new(99, sort(), vec![id].into_iter().collect());
        assert!(da.is_ready(&ctx));
    }
    #[test]
    fn test_delayed_assignment_not_ready() {
        let ctx = MetaVarContext::new();
        let waiting = vec![42u64].into_iter().collect();
        let da = DelayedAssignment::new(99, sort(), waiting);
        assert!(!da.is_ready(&ctx));
    }
    #[test]
    fn test_delayed_queue_drain_ready() {
        let mut ctx = MetaVarContext::new();
        let id = ctx.fresh(sort());
        ctx.assign(id, sort());
        let mut q = DelayedAssignmentQueue::new();
        let da_ready = DelayedAssignment::new(99, sort(), vec![id].into_iter().collect());
        let da_not_ready = DelayedAssignment::new(98, sort(), vec![12345u64].into_iter().collect());
        q.enqueue(da_ready);
        q.enqueue(da_not_ready);
        let ready = q.drain_ready(&ctx);
        assert_eq!(ready.len(), 1);
        assert_eq!(q.len(), 1);
    }
    #[test]
    fn test_meta_eq_class_same_class() {
        let mut uf = MetaEqClass::new();
        uf.add(1);
        uf.add(2);
        assert!(!uf.same_class(1, 2));
        uf.union(1, 2);
        assert!(uf.same_class(1, 2));
    }
    #[test]
    fn test_meta_eq_class_class_count() {
        let mut uf = MetaEqClass::new();
        uf.add(1);
        uf.add(2);
        uf.add(3);
        assert_eq!(uf.class_count(), 3);
        uf.union(1, 2);
        assert_eq!(uf.class_count(), 2);
        uf.union(2, 3);
        assert_eq!(uf.class_count(), 1);
    }
    #[test]
    fn test_meta_eq_class_transitivity() {
        let mut uf = MetaEqClass::new();
        uf.add(1);
        uf.add(2);
        uf.add(3);
        uf.union(1, 2);
        uf.union(2, 3);
        assert!(uf.same_class(1, 3));
    }
    #[test]
    fn test_metavar_graph_no_dependents() {
        let g = MetaVarGraph::new();
        assert!(!g.has_dependents(42));
        assert!(g.dependents_of(42).is_empty());
    }
}
#[cfg(test)]
mod scope_and_occurs_tests {
    use super::*;
    use crate::metavar::*;
    fn sort() -> Expr {
        Expr::Sort(Level::zero())
    }
    #[test]
    fn test_assign_rejects_circular() {
        let mut ctx = MetaVarContext::new();
        let id = ctx.fresh(sort());
        let meta_expr = Expr::FVar(FVarId(1_000_000 + id));
        assert!(!ctx.assign(id, meta_expr));
        assert!(!ctx.is_solved(id));
    }
    #[test]
    fn test_assign_accepts_non_circular() {
        let mut ctx = MetaVarContext::new();
        let id = ctx.fresh(sort());
        assert!(ctx.assign(id, sort()));
        assert!(ctx.is_solved(id));
    }
    #[test]
    fn test_assign_rejects_transitive_cycle() {
        let mut ctx = MetaVarContext::new();
        let a = ctx.fresh(sort());
        let b = ctx.fresh(sort());
        let b_fvar = Expr::FVar(FVarId(1_000_000 + b));
        assert!(ctx.assign(a, b_fvar));
        let a_fvar = Expr::FVar(FVarId(1_000_000 + a));
        assert!(!ctx.assign(b, a_fvar));
    }
    #[test]
    fn test_fresh_captures_empty_scope() {
        let mut ctx = MetaVarContext::new();
        let id = ctx.fresh(sort());
        assert!(ctx.get(id).expect("key should exist").scope_vars.is_empty());
    }
    #[test]
    fn test_fresh_captures_current_scope() {
        let mut ctx = MetaVarContext::new();
        ctx.push_scope_var(42);
        ctx.push_scope_var(43);
        let id = ctx.fresh(sort());
        let scope = &ctx.get(id).expect("key should exist").scope_vars;
        assert!(scope.contains(&42));
        assert!(scope.contains(&43));
        assert_eq!(scope.len(), 2);
    }
    #[test]
    fn test_push_pop_scope_var() {
        let mut ctx = MetaVarContext::new();
        ctx.push_scope_var(10);
        ctx.push_scope_var(20);
        assert_eq!(ctx.current_scope().len(), 2);
        ctx.pop_scope_var();
        assert_eq!(ctx.current_scope().len(), 1);
        assert!(ctx.is_in_scope(10));
        assert!(!ctx.is_in_scope(20));
    }
    #[test]
    fn test_meta_fvar_always_in_scope() {
        let ctx = MetaVarContext::new();
        assert!(ctx.is_in_scope(1_000_000));
        assert!(ctx.is_in_scope(1_000_001));
        assert!(ctx.is_in_scope(u64::MAX));
    }
    #[test]
    fn test_fvar_not_in_empty_scope() {
        let ctx = MetaVarContext::new();
        assert!(!ctx.is_in_scope(0));
        assert!(!ctx.is_in_scope(5));
    }
    #[test]
    fn test_metavar_fvar_in_scope_method() {
        let mut meta = MetaVar::new(0, sort());
        meta.scope_vars = vec![1, 2, 3];
        assert!(meta.fvar_in_scope(1));
        assert!(meta.fvar_in_scope(2));
        assert!(!meta.fvar_in_scope(4));
        assert!(meta.fvar_in_scope(1_000_000));
    }
    #[test]
    fn test_scope_check_expr_no_fvars() {
        assert!(scope_check_expr(&Expr::BVar(0), &[]));
        assert!(scope_check_expr(&sort(), &[]));
    }
    #[test]
    fn test_scope_check_expr_fvar_in_scope() {
        let e = Expr::FVar(FVarId(5));
        assert!(scope_check_expr(&e, &[3, 5, 7]));
    }
    #[test]
    fn test_scope_check_expr_fvar_out_of_scope() {
        let e = Expr::FVar(FVarId(5));
        assert!(!scope_check_expr(&e, &[3, 7]));
    }
    #[test]
    fn test_scope_check_expr_meta_fvar_always_ok() {
        let e = Expr::FVar(FVarId(1_000_042));
        assert!(scope_check_expr(&e, &[]));
    }
    #[test]
    fn test_scope_check_expr_app() {
        let f = Expr::FVar(FVarId(1));
        let a = Expr::FVar(FVarId(2));
        let app = Expr::App(Box::new(f), Box::new(a));
        assert!(scope_check_expr(&app, &[1, 2]));
        assert!(!scope_check_expr(&app, &[1]));
    }
    #[test]
    fn test_assign_checked_ok() {
        let mut ctx = MetaVarContext::new();
        let id = ctx.fresh(sort());
        assert!(ctx.assign_checked(id, sort()).is_ok());
    }
    #[test]
    fn test_assign_checked_frozen() {
        let mut ctx = MetaVarContext::new();
        let id = ctx.fresh(sort());
        ctx.freeze(id);
        assert!(ctx.assign_checked(id, sort()).is_err());
    }
    #[test]
    fn test_assign_checked_nonexistent() {
        let mut ctx = MetaVarContext::new();
        assert!(ctx.assign_checked(999, sort()).is_err());
    }
    #[test]
    fn test_assign_checked_occurs_fail() {
        let mut ctx = MetaVarContext::new();
        let id = ctx.fresh(sort());
        let self_ref = Expr::FVar(FVarId(1_000_000 + id));
        let err = ctx.assign_checked(id, self_ref);
        assert!(err.is_err());
        assert!(err.unwrap_err().contains("occurs check"));
    }
    #[test]
    fn test_assign_checked_scope_fail() {
        let mut ctx = MetaVarContext::new();
        let id = ctx.fresh(sort());
        let out_of_scope = Expr::FVar(FVarId(7));
        let err = ctx.assign_checked(id, out_of_scope);
        assert!(err.is_err());
        assert!(err.unwrap_err().contains("scope check"));
    }
    #[test]
    fn test_assign_checked_scope_ok_with_var() {
        let mut ctx = MetaVarContext::new();
        ctx.push_scope_var(7);
        let id = ctx.fresh(sort());
        let in_scope = Expr::FVar(FVarId(7));
        assert!(ctx.assign_checked(id, in_scope).is_ok());
    }
}
/// Collect all free metavariable IDs in an expression.
///
/// Since metavariables are encoded as `FVar(id)` where `id >= 1_000_000`,
/// this function collects those FVar IDs.
#[allow(dead_code)]
pub fn collect_meta_fvars(expr: &Expr) -> HashSet<u64> {
    let mut result = HashSet::new();
    collect_meta_fvars_rec(expr, &mut result);
    result
}
fn collect_meta_fvars_rec(expr: &Expr, out: &mut HashSet<u64>) {
    match expr {
        Expr::FVar(FVarId(id)) if *id >= 1_000_000 => {
            out.insert(*id);
        }
        Expr::App(f, a) => {
            collect_meta_fvars_rec(f, out);
            collect_meta_fvars_rec(a, out);
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            collect_meta_fvars_rec(ty, out);
            collect_meta_fvars_rec(body, out);
        }
        Expr::Let(_, ty, val, body) => {
            collect_meta_fvars_rec(ty, out);
            collect_meta_fvars_rec(val, out);
            collect_meta_fvars_rec(body, out);
        }
        Expr::Proj(_, _, inner) => collect_meta_fvars_rec(inner, out),
        _ => {}
    }
}
/// Check if an expression is metavariable-free.
#[allow(dead_code)]
pub fn is_meta_free(expr: &Expr) -> bool {
    collect_meta_fvars(expr).is_empty()
}
/// Substitute a single metavariable FVar with an expression.
#[allow(dead_code)]
pub fn subst_meta_fvar(expr: &Expr, meta_id: u64, replacement: &Expr) -> Expr {
    match expr {
        Expr::FVar(FVarId(id)) if *id == meta_id => replacement.clone(),
        Expr::App(f, a) => Expr::App(
            Box::new(subst_meta_fvar(f, meta_id, replacement)),
            Box::new(subst_meta_fvar(a, meta_id, replacement)),
        ),
        Expr::Lam(info, name, ty, body) => Expr::Lam(
            *info,
            name.clone(),
            Box::new(subst_meta_fvar(ty, meta_id, replacement)),
            Box::new(subst_meta_fvar(body, meta_id, replacement)),
        ),
        Expr::Pi(info, name, ty, body) => Expr::Pi(
            *info,
            name.clone(),
            Box::new(subst_meta_fvar(ty, meta_id, replacement)),
            Box::new(subst_meta_fvar(body, meta_id, replacement)),
        ),
        Expr::Let(name, ty, val, body) => Expr::Let(
            name.clone(),
            Box::new(subst_meta_fvar(ty, meta_id, replacement)),
            Box::new(subst_meta_fvar(val, meta_id, replacement)),
            Box::new(subst_meta_fvar(body, meta_id, replacement)),
        ),
        Expr::Proj(name, idx, inner) => Expr::Proj(
            name.clone(),
            *idx,
            Box::new(subst_meta_fvar(inner, meta_id, replacement)),
        ),
        _ => expr.clone(),
    }
}
/// Apply a `MetaSubstitution` to an expression.
#[allow(dead_code)]
pub fn apply_substitution(expr: &Expr, subst: &MetaSubstitution) -> Expr {
    match expr {
        Expr::FVar(FVarId(id)) if *id >= 1_000_000 => {
            if let Some(replacement) = subst.get(*id) {
                apply_substitution(replacement, subst)
            } else {
                expr.clone()
            }
        }
        Expr::App(f, a) => Expr::App(
            Box::new(apply_substitution(f, subst)),
            Box::new(apply_substitution(a, subst)),
        ),
        Expr::Lam(info, name, ty, body) => Expr::Lam(
            *info,
            name.clone(),
            Box::new(apply_substitution(ty, subst)),
            Box::new(apply_substitution(body, subst)),
        ),
        Expr::Pi(info, name, ty, body) => Expr::Pi(
            *info,
            name.clone(),
            Box::new(apply_substitution(ty, subst)),
            Box::new(apply_substitution(body, subst)),
        ),
        Expr::Let(name, ty, val, body) => Expr::Let(
            name.clone(),
            Box::new(apply_substitution(ty, subst)),
            Box::new(apply_substitution(val, subst)),
            Box::new(apply_substitution(body, subst)),
        ),
        Expr::Proj(name, idx, inner) => Expr::Proj(
            name.clone(),
            *idx,
            Box::new(apply_substitution(inner, subst)),
        ),
        _ => expr.clone(),
    }
}
/// Count the number of free BVar occurrences in an expression.
#[allow(dead_code)]
pub fn count_bvar_occurrences(expr: &Expr, target: u32) -> usize {
    match expr {
        Expr::BVar(i) => {
            if *i == target {
                1
            } else {
                0
            }
        }
        Expr::App(f, a) => count_bvar_occurrences(f, target) + count_bvar_occurrences(a, target),
        Expr::Lam(_, _, ty, body) => {
            count_bvar_occurrences(ty, target) + count_bvar_occurrences(body, target + 1)
        }
        Expr::Pi(_, _, ty, body) => {
            count_bvar_occurrences(ty, target) + count_bvar_occurrences(body, target + 1)
        }
        Expr::Let(_, ty, val, body) => {
            count_bvar_occurrences(ty, target)
                + count_bvar_occurrences(val, target)
                + count_bvar_occurrences(body, target + 1)
        }
        Expr::Proj(_, _, inner) => count_bvar_occurrences(inner, target),
        _ => 0,
    }
}
/// Check if a BVar is used (to detect unused binders).
#[allow(dead_code)]
pub fn bvar_is_used(expr: &Expr, depth: u32) -> bool {
    count_bvar_occurrences(expr, depth) > 0
}
#[cfg(test)]
mod tests_extended {
    use super::*;
    use crate::metavar::*;
    fn sort() -> Expr {
        Expr::Sort(oxilean_kernel::Level::Zero)
    }
    fn mk_fvar(id: u64) -> Expr {
        Expr::FVar(FVarId(id))
    }
    fn mk_meta_fvar(id: u64) -> Expr {
        Expr::FVar(FVarId(1_000_000 + id))
    }
    #[test]
    fn test_priority_ordering() {
        assert!(MetaVarPriority::Immediate > MetaVarPriority::High);
        assert!(MetaVarPriority::High > MetaVarPriority::Normal);
        assert!(MetaVarPriority::Normal > MetaVarPriority::Low);
    }
    #[test]
    fn test_priority_display() {
        assert_eq!(format!("{}", MetaVarPriority::Low), "low");
        assert_eq!(format!("{}", MetaVarPriority::Normal), "normal");
        assert_eq!(format!("{}", MetaVarPriority::High), "high");
        assert_eq!(format!("{}", MetaVarPriority::Immediate), "immediate");
    }
    #[test]
    fn test_stats_from_empty_ctx() {
        let ctx = MetaVarContext::new();
        let stats = MetaVarStats::from_ctx(&ctx);
        assert_eq!(stats.total, 0);
        assert_eq!(stats.solved, 0);
        assert!(stats.is_fully_solved());
        assert!((stats.solve_ratio() - 1.0).abs() < 1e-9);
    }
    #[test]
    fn test_stats_from_ctx_with_metas() {
        let mut ctx = MetaVarContext::new();
        let id1 = ctx.fresh(sort());
        let id2 = ctx.fresh_synthetic(sort());
        let _id3 = ctx.fresh_opaque(sort());
        ctx.assign(id1, sort());
        ctx.assign(id2, sort());
        let stats = MetaVarStats::from_ctx(&ctx);
        assert_eq!(stats.total, 3);
        assert_eq!(stats.solved, 2);
        assert_eq!(stats.unsolved, 1);
        assert_eq!(stats.natural, 1);
        assert_eq!(stats.synthetic, 1);
        assert_eq!(stats.synthetic_opaque, 1);
    }
    #[test]
    fn test_stats_summary_contains_percent() {
        let mut ctx = MetaVarContext::new();
        let id = ctx.fresh(sort());
        ctx.assign(id, sort());
        let stats = MetaVarStats::from_ctx(&ctx);
        let s = stats.summary();
        assert!(s.contains('%'));
    }
    #[test]
    fn test_log_disabled_no_events() {
        let mut log = MetaVarLog::new();
        log.record(MetaVarEvent::Created {
            id: 0,
            kind: MetaVarKind::Natural,
            depth: 0,
        });
        assert_eq!(log.events().len(), 0);
    }
    #[test]
    fn test_log_enabled_records() {
        let mut log = MetaVarLog::enabled();
        log.record(MetaVarEvent::Created {
            id: 0,
            kind: MetaVarKind::Natural,
            depth: 0,
        });
        log.record(MetaVarEvent::Assigned { id: 0 });
        assert_eq!(log.count_created(), 1);
        assert_eq!(log.count_assigned(), 1);
    }
    #[test]
    fn test_log_enable_disable() {
        let mut log = MetaVarLog::new();
        assert!(!log.is_enabled());
        log.enable();
        assert!(log.is_enabled());
        log.record(MetaVarEvent::Assigned { id: 0 });
        assert_eq!(log.count_assigned(), 1);
        log.disable();
        log.record(MetaVarEvent::Assigned { id: 1 });
        assert_eq!(log.count_assigned(), 1);
    }
    #[test]
    fn test_constraint_queue_push_pop() {
        let mut q = ConstraintQueue::new();
        let c = MetaConstraint::new_eq(mk_fvar(1), mk_fvar(2), "test");
        q.push(c);
        assert!(!q.is_empty());
        assert_eq!(q.len(), 1);
        let popped = q.pop();
        assert!(popped.is_some());
        assert!(q.is_empty());
    }
    #[test]
    fn test_constraint_queue_simple_first() {
        let mut q = ConstraintQueue::new();
        let complex_c = MetaConstraint::new_eq(
            Expr::App(Box::new(mk_fvar(1)), Box::new(mk_fvar(2))),
            mk_fvar(3),
            "complex",
        );
        let simple_c = MetaConstraint::new_eq(mk_fvar(10), sort(), "simple");
        q.push(complex_c);
        q.push(simple_c);
        let first = q.pop().expect("collection should not be empty");
        assert_eq!(first.origin, "simple");
    }
    #[test]
    fn test_meta_subst_insert_get() {
        let mut s = MetaSubstitution::new();
        s.insert(42, sort());
        assert!(s.contains(42));
        assert!(!s.contains(43));
        assert!(s.get(42).is_some());
    }
    #[test]
    fn test_meta_subst_merge() {
        let mut s1 = MetaSubstitution::new();
        s1.insert(1, sort());
        let mut s2 = MetaSubstitution::new();
        s2.insert(2, sort());
        s1.merge(s2);
        assert!(s1.contains(1));
        assert!(s1.contains(2));
        assert_eq!(s1.len(), 2);
    }
    #[test]
    fn test_meta_subst_apply_to_ctx() {
        let mut ctx = MetaVarContext::new();
        let id = ctx.fresh(sort());
        let mut s = MetaSubstitution::new();
        s.insert(id, sort());
        let applied = s.apply_to_ctx(&mut ctx);
        assert_eq!(applied, 1);
        assert!(ctx.is_solved(id));
    }
    #[test]
    fn test_unification_result_success() {
        let r = UnificationResult::trivial();
        assert!(r.is_success());
        assert!(!r.is_failure());
        assert!(!r.is_delayed());
        assert!(r.substitution().is_some());
    }
    #[test]
    fn test_unification_result_failure() {
        let r = UnificationResult::fail("mismatch");
        assert!(r.is_failure());
        assert_eq!(r.error_message(), Some("mismatch"));
    }
    #[test]
    fn test_meta_var_group_add() {
        let mut g = MetaVarGroup::new("goal_metas");
        g.add(1);
        g.add(2);
        assert_eq!(g.len(), 2);
        assert!(g.contains(1));
        assert!(!g.contains(3));
    }
    #[test]
    fn test_meta_var_group_close() {
        let mut g = MetaVarGroup::new("closed_group");
        g.add(1);
        g.close();
        g.add(2);
        assert_eq!(g.len(), 1);
    }
    #[test]
    fn test_meta_var_group_solved_count() {
        let mut ctx = MetaVarContext::new();
        let id1 = ctx.fresh(sort());
        let id2 = ctx.fresh(sort());
        ctx.assign(id1, sort());
        let mut g = MetaVarGroup::new("test");
        g.add(id1);
        g.add(id2);
        assert_eq!(g.solved_count(&ctx), 1);
        assert!(!g.all_solved(&ctx));
    }
    #[test]
    fn test_meta_var_pool_create_and_add() {
        let mut ctx = MetaVarContext::new();
        let id = ctx.fresh(sort());
        let mut pool = MetaVarPool::new();
        pool.create_group("g1");
        pool.add_to_group("g1", id);
        let g = pool.get_group("g1").expect("test operation should succeed");
        assert!(g.contains(id));
    }
    #[test]
    fn test_meta_var_pool_all_solved() {
        let mut ctx = MetaVarContext::new();
        let id = ctx.fresh(sort());
        ctx.assign(id, sort());
        let mut pool = MetaVarPool::new();
        pool.create_group("only");
        pool.add_to_group("only", id);
        assert!(pool.all_groups_solved(&ctx));
    }
    #[test]
    fn test_collect_meta_fvars_none() {
        let e = sort();
        assert!(collect_meta_fvars(&e).is_empty());
    }
    #[test]
    fn test_collect_meta_fvars_found() {
        let e = mk_meta_fvar(5);
        let found = collect_meta_fvars(&e);
        assert!(found.contains(&1_000_005));
    }
    #[test]
    fn test_is_meta_free() {
        assert!(is_meta_free(&sort()));
        assert!(!is_meta_free(&mk_meta_fvar(0)));
    }
    #[test]
    fn test_apply_substitution_replaces() {
        let meta_id = 1_000_007u64;
        let e = Expr::FVar(FVarId(meta_id));
        let mut subst = MetaSubstitution::new();
        subst.insert(meta_id, sort());
        let result = apply_substitution(&e, &subst);
        assert!(matches!(result, Expr::Sort(_)));
    }
    #[test]
    fn test_apply_substitution_no_match() {
        let e = sort();
        let subst = MetaSubstitution::new();
        let result = apply_substitution(&e, &subst);
        assert!(matches!(result, Expr::Sort(_)));
    }
    #[test]
    fn test_count_bvar_occurrences_match() {
        let e = Expr::BVar(0);
        assert_eq!(count_bvar_occurrences(&e, 0), 1);
        assert_eq!(count_bvar_occurrences(&e, 1), 0);
    }
    #[test]
    fn test_count_bvar_occurrences_in_app() {
        let e = Expr::App(Box::new(Expr::BVar(0)), Box::new(Expr::BVar(0)));
        assert_eq!(count_bvar_occurrences(&e, 0), 2);
    }
    #[test]
    fn test_bvar_is_used() {
        let body = Expr::BVar(0);
        assert!(bvar_is_used(&body, 0));
        assert!(!bvar_is_used(&body, 1));
    }
}
