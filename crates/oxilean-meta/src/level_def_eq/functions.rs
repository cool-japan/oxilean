//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    LevelConstraint, LevelConstraintSolver, LevelConstraintSystem, LevelDefEq,
    LevelDefEqAnalysisPass, LevelDefEqBuilder, LevelDefEqConfig, LevelDefEqConfigValue,
    LevelDefEqCounterMap, LevelDefEqDiagnostics, LevelDefEqDiff, LevelDefEqExtConfig1200,
    LevelDefEqExtConfigVal1200, LevelDefEqExtDiag1200, LevelDefEqExtDiff1200, LevelDefEqExtMap,
    LevelDefEqExtPass1200, LevelDefEqExtPipeline1200, LevelDefEqExtResult1200, LevelDefEqExtUtil,
    LevelDefEqPipeline, LevelDefEqResult, LevelDefEqStateMachine, LevelDefEqWindow,
    LevelDefEqWorkQueue, LevelUnifResult, LevelUnifStats,
};
use crate::basic::MetaContext;
use oxilean_kernel::{Level, LevelMVarId};
use std::collections::HashSet;

/// Check if a level metavariable occurs in a level expression.
pub(super) fn level_occurs_check(mvar_id: u64, level: &Level) -> bool {
    match level {
        Level::MVar(LevelMVarId(id)) => *id == mvar_id,
        Level::Succ(inner) => level_occurs_check(mvar_id, inner),
        Level::Max(a, b) | Level::IMax(a, b) => {
            level_occurs_check(mvar_id, a) || level_occurs_check(mvar_id, b)
        }
        Level::Zero | Level::Param(_) => false,
    }
}
/// Collect unassigned level metavariables.
pub(super) fn collect_level_mvars_impl(
    level: &Level,
    ctx: &MetaContext,
    result: &mut HashSet<u64>,
) {
    match level {
        Level::MVar(LevelMVarId(id)) => {
            if let Some(assigned) = ctx.get_level_assignment(*id) {
                collect_level_mvars_impl(assigned, ctx, result);
            } else {
                result.insert(*id);
            }
        }
        Level::Succ(inner) => collect_level_mvars_impl(inner, ctx, result),
        Level::Max(a, b) | Level::IMax(a, b) => {
            collect_level_mvars_impl(a, ctx, result);
            collect_level_mvars_impl(b, ctx, result);
        }
        Level::Zero | Level::Param(_) => {}
    }
}
#[allow(clippy::if_same_then_else)]
/// Normalize a level expression.
///
/// Simplifications:
/// - `max(0, l) = l`
/// - `max(l, 0) = l`
/// - `max(l, l) = l`
/// - `imax(l, 0) = 0`
/// - `imax(0, l) = l`
pub(super) fn normalize_level(level: &Level) -> Level {
    match level {
        Level::Zero | Level::Param(_) | Level::MVar(_) => level.clone(),
        Level::Succ(inner) => {
            let inner_norm = normalize_level(inner);
            Level::succ(inner_norm)
        }
        Level::Max(a, b) => {
            let a_norm = normalize_level(a);
            let b_norm = normalize_level(b);
            if a_norm.is_zero() {
                b_norm
            } else if b_norm.is_zero() {
                a_norm
            } else if a_norm == b_norm {
                a_norm
            } else {
                Level::max(a_norm, b_norm)
            }
        }
        Level::IMax(a, b) => {
            let a_norm = normalize_level(a);
            let b_norm = normalize_level(b);
            if b_norm.is_zero() {
                Level::zero()
            } else if a_norm.is_zero() {
                b_norm
            } else if let Level::Succ(_) = &b_norm {
                Level::max(a_norm, b_norm)
            } else {
                Level::imax(a_norm, b_norm)
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::level_def_eq::*;
    use oxilean_kernel::{Environment, Name};
    fn mk_ctx() -> MetaContext {
        MetaContext::new(Environment::new())
    }
    #[test]
    fn test_level_eq_zero() {
        let mut ldeq = LevelDefEq::new();
        let mut ctx = mk_ctx();
        assert!(ldeq.is_level_def_eq(&Level::zero(), &Level::zero(), &mut ctx));
    }
    #[test]
    fn test_level_eq_succ() {
        let mut ldeq = LevelDefEq::new();
        let mut ctx = mk_ctx();
        let l1 = Level::succ(Level::zero());
        let l2 = Level::succ(Level::zero());
        assert!(ldeq.is_level_def_eq(&l1, &l2, &mut ctx));
    }
    #[test]
    fn test_level_eq_param() {
        let mut ldeq = LevelDefEq::new();
        let mut ctx = mk_ctx();
        let l1 = Level::param(Name::str("u"));
        let l2 = Level::param(Name::str("u"));
        assert!(ldeq.is_level_def_eq(&l1, &l2, &mut ctx));
        let l3 = Level::param(Name::str("v"));
        assert!(!ldeq.is_level_def_eq(&l1, &l3, &mut ctx));
    }
    #[test]
    fn test_level_mvar_assignment() {
        let mut ldeq = LevelDefEq::new();
        let mut ctx = mk_ctx();
        let mvar = ctx.mk_fresh_level_mvar();
        let target = Level::succ(Level::zero());
        assert!(ldeq.is_level_def_eq(&mvar, &target, &mut ctx));
        let inst = ctx.instantiate_level_mvars(&mvar);
        assert_eq!(inst, target);
    }
    #[test]
    fn test_level_mvar_both_sides() {
        let mut ldeq = LevelDefEq::new();
        let mut ctx = mk_ctx();
        let m1 = ctx.mk_fresh_level_mvar();
        let m2 = ctx.mk_fresh_level_mvar();
        assert!(ldeq.is_level_def_eq(&m1, &m2, &mut ctx));
    }
    #[test]
    fn test_level_max_commutativity() {
        let mut ldeq = LevelDefEq::new();
        let mut ctx = mk_ctx();
        let l1 = Level::max(Level::param(Name::str("u")), Level::param(Name::str("v")));
        let l2 = Level::max(Level::param(Name::str("v")), Level::param(Name::str("u")));
        assert!(ldeq.is_level_def_eq(&l1, &l2, &mut ctx));
    }
    #[test]
    fn test_level_max_zero() {
        let mut ldeq = LevelDefEq::new();
        let mut ctx = mk_ctx();
        let l1 = Level::max(Level::zero(), Level::param(Name::str("u")));
        let l2 = Level::param(Name::str("u"));
        assert!(ldeq.is_level_def_eq(&l1, &l2, &mut ctx));
    }
    #[test]
    fn test_level_leq() {
        let mut ldeq = LevelDefEq::new();
        let mut ctx = mk_ctx();
        assert!(ldeq.is_level_leq(&Level::zero(), &Level::succ(Level::zero()), &mut ctx));
        assert!(ldeq.is_level_leq(&Level::zero(), &Level::zero(), &mut ctx));
    }
    #[test]
    fn test_normalize_max_zero() {
        let l = Level::max(Level::zero(), Level::succ(Level::zero()));
        let norm = normalize_level(&l);
        assert_eq!(norm, Level::succ(Level::zero()));
    }
    #[test]
    fn test_normalize_imax_zero() {
        let l = Level::imax(Level::param(Name::str("u")), Level::zero());
        let norm = normalize_level(&l);
        assert_eq!(norm, Level::zero());
    }
    #[test]
    fn test_normalize_max_same() {
        let l = Level::max(Level::param(Name::str("u")), Level::param(Name::str("u")));
        let norm = normalize_level(&l);
        assert_eq!(norm, Level::param(Name::str("u")));
    }
    #[test]
    fn test_level_occurs_check() {
        assert!(level_occurs_check(0, &Level::MVar(LevelMVarId(0))));
        assert!(!level_occurs_check(0, &Level::MVar(LevelMVarId(1))));
        assert!(level_occurs_check(
            0,
            &Level::succ(Level::MVar(LevelMVarId(0)))
        ));
        assert!(!level_occurs_check(0, &Level::zero()));
    }
    #[test]
    fn test_collect_level_mvars() {
        let ldeq = LevelDefEq::new();
        let ctx = mk_ctx();
        let level = Level::max(
            Level::MVar(LevelMVarId(0)),
            Level::succ(Level::MVar(LevelMVarId(1))),
        );
        let mvars = ldeq.collect_level_mvars(&level, &ctx);
        assert!(mvars.contains(&0));
        assert!(mvars.contains(&1));
        assert_eq!(mvars.len(), 2);
    }
    #[test]
    fn test_ensure_not_zero() {
        let ldeq = LevelDefEq::new();
        let ctx = mk_ctx();
        assert!(!ldeq.ensure_not_zero(&Level::zero(), &ctx));
        assert!(ldeq.ensure_not_zero(&Level::succ(Level::zero()), &ctx));
        assert!(ldeq.ensure_not_zero(&Level::param(Name::str("u")), &ctx));
    }
    #[test]
    fn test_normalize_imax_succ() {
        let l = Level::imax(
            Level::param(Name::str("u")),
            Level::succ(Level::param(Name::str("v"))),
        );
        let norm = normalize_level(&l);
        assert!(matches!(norm, Level::Max(_, _)));
    }
}
/// Perform level unification.
pub fn level_unify(l1: &Level, l2: &Level, ctx: &mut MetaContext) -> LevelUnifResult {
    let mut ldeq = LevelDefEq::new();
    let l1_inst = ctx.instantiate_level_mvars(l1);
    let l2_inst = ctx.instantiate_level_mvars(l2);
    if ldeq.is_level_def_eq(&l1_inst, &l2_inst, ctx) {
        return LevelUnifResult::Success;
    }
    let has_mvars1 = has_level_mvars(&l1_inst);
    let has_mvars2 = has_level_mvars(&l2_inst);
    if has_mvars1 || has_mvars2 {
        LevelUnifResult::Postponed
    } else {
        LevelUnifResult::Failure
    }
}
/// Check if a level has metavariables.
pub fn has_level_mvars(l: &Level) -> bool {
    match l {
        Level::MVar(_) => true,
        Level::Succ(inner) => has_level_mvars(inner),
        Level::Max(a, b) | Level::IMax(a, b) => has_level_mvars(a) || has_level_mvars(b),
        Level::Zero | Level::Param(_) => false,
    }
}
/// Compute the minimum concrete value (lower bound).
pub fn level_lower_bound(l: &Level) -> u32 {
    match l {
        Level::Zero => 0,
        Level::Succ(inner) => level_lower_bound(inner) + 1,
        Level::Max(a, b) => level_lower_bound(a).max(level_lower_bound(b)),
        Level::IMax(_, _) => 0,
        Level::Param(_) | Level::MVar(_) => 0,
    }
}
/// Compute an upper bound (returns None if unbounded).
pub fn level_upper_bound(l: &Level) -> Option<u32> {
    match l {
        Level::Zero => Some(0),
        Level::Succ(inner) => level_upper_bound(inner).map(|n| n + 1),
        Level::Max(a, b) => match (level_upper_bound(a), level_upper_bound(b)) {
            (Some(x), Some(y)) => Some(x.max(y)),
            _ => None,
        },
        Level::IMax(_, b) => {
            if let Some(b_ub) = level_upper_bound(b) {
                if b_ub == 0 {
                    return Some(0);
                }
            }
            None
        }
        Level::Param(_) | Level::MVar(_) => None,
    }
}
/// Level simplification.
pub fn simplify_level(l: &Level) -> Level {
    match l {
        Level::Max(a, b) => {
            let a_s = simplify_level(a);
            let b_s = simplify_level(b);
            if a_s == b_s {
                return a_s;
            }
            if matches!(a_s, Level::Zero) {
                return b_s;
            }
            if matches!(b_s, Level::Zero) {
                return a_s;
            }
            if let (Level::Succ(ia), Level::Succ(ib)) = (&a_s, &b_s) {
                return Level::succ(simplify_level(&Level::max(*ia.clone(), *ib.clone())));
            }
            Level::max(a_s, b_s)
        }
        Level::IMax(a, b) => {
            let a_s = simplify_level(a);
            let b_s = simplify_level(b);
            if matches!(b_s, Level::Zero) {
                return Level::zero();
            }
            if matches!(a_s, Level::Zero) {
                return b_s;
            }
            if matches!(b_s, Level::Succ(_)) {
                return simplify_level(&Level::max(a_s, b_s));
            }
            Level::imax(a_s, b_s)
        }
        Level::Succ(inner) => Level::succ(simplify_level(inner)),
        _ => l.clone(),
    }
}
#[cfg(test)]
mod extended_level_def_eq_tests {
    use super::*;
    use crate::level_def_eq::*;
    use oxilean_kernel::{Environment, Name};
    fn mk_ctx() -> MetaContext {
        MetaContext::new(Environment::new())
    }
    #[test]
    fn test_constraint_system_add() {
        let mut sys = LevelConstraintSystem::new();
        sys.add_leq(Level::zero(), Level::succ(Level::zero()));
        sys.add_eq(Level::zero(), Level::zero());
        assert_eq!(sys.len(), 2);
        assert!(sys.is_satisfiable());
    }
    #[test]
    fn test_constraint_system_contradiction() {
        let mut sys = LevelConstraintSystem::new();
        let l = Level::param(Name::str("u"));
        sys.add_lt(l.clone(), l);
        assert!(!sys.is_satisfiable());
    }
    #[test]
    fn test_constraint_system_clear() {
        let mut sys = LevelConstraintSystem::new();
        sys.add_eq(Level::zero(), Level::zero());
        sys.clear();
        assert!(sys.is_empty());
    }
    #[test]
    fn test_level_unify_success() {
        let mut ctx = mk_ctx();
        assert_eq!(
            level_unify(&Level::zero(), &Level::zero(), &mut ctx),
            LevelUnifResult::Success
        );
    }
    #[test]
    fn test_level_unify_with_mvar() {
        let mut ctx = mk_ctx();
        let m = ctx.mk_fresh_level_mvar();
        let target = Level::succ(Level::zero());
        assert_eq!(level_unify(&m, &target, &mut ctx), LevelUnifResult::Success);
    }
    #[test]
    fn test_level_unify_failure() {
        let mut ctx = mk_ctx();
        assert_eq!(
            level_unify(&Level::zero(), &Level::succ(Level::zero()), &mut ctx),
            LevelUnifResult::Failure
        );
    }
    #[test]
    fn test_has_level_mvars() {
        assert!(!has_level_mvars(&Level::zero()));
        assert!(has_level_mvars(&Level::MVar(oxilean_kernel::LevelMVarId(
            0
        ))));
    }
    #[test]
    fn test_level_lower_bound() {
        assert_eq!(level_lower_bound(&Level::zero()), 0);
        assert_eq!(level_lower_bound(&Level::succ(Level::zero())), 1);
    }
    #[test]
    fn test_level_upper_bound() {
        assert_eq!(level_upper_bound(&Level::zero()), Some(0));
        assert_eq!(level_upper_bound(&Level::param(Name::str("u"))), None);
    }
    #[test]
    fn test_simplify_max_same() {
        let l = Level::max(Level::param(Name::str("u")), Level::param(Name::str("u")));
        assert_eq!(simplify_level(&l), Level::param(Name::str("u")));
    }
    #[test]
    fn test_simplify_max_zero() {
        let l = Level::max(Level::zero(), Level::param(Name::str("u")));
        assert_eq!(simplify_level(&l), Level::param(Name::str("u")));
    }
    #[test]
    fn test_simplify_imax_zero_rhs() {
        let l = Level::imax(Level::param(Name::str("u")), Level::zero());
        assert_eq!(simplify_level(&l), Level::zero());
    }
    #[test]
    fn test_level_unif_stats() {
        let mut stats = LevelUnifStats::new();
        stats.record_attempt(&LevelUnifResult::Success);
        stats.record_attempt(&LevelUnifResult::Failure);
        assert_eq!(stats.attempts, 2);
        assert!((stats.success_rate() - 0.5).abs() < 1e-9);
    }
    #[test]
    fn test_level_constraint_levels() {
        let l1 = Level::zero();
        let l2 = Level::succ(Level::zero());
        let c = LevelConstraint::Leq(l1.clone(), l2.clone());
        let (a, b) = c.levels();
        assert_eq!(*a, l1);
        assert_eq!(*b, l2);
    }
}
/// Check whether a level contains any `Param` nodes.
#[allow(dead_code)]
pub fn has_level_params(l: &Level) -> bool {
    match l {
        Level::Param(_) => true,
        Level::Succ(inner) => has_level_params(inner),
        Level::Max(a, b) | Level::IMax(a, b) => has_level_params(a) || has_level_params(b),
        Level::MVar(_) | Level::Zero => false,
    }
}
/// Collect all `Param` names in a level expression.
#[allow(dead_code)]
pub fn collect_level_params(l: &Level) -> HashSet<oxilean_kernel::Name> {
    let mut params = HashSet::new();
    collect_level_params_rec(l, &mut params);
    params
}
pub(super) fn collect_level_params_rec(l: &Level, acc: &mut HashSet<oxilean_kernel::Name>) {
    match l {
        Level::Param(n) => {
            acc.insert(n.clone());
        }
        Level::Succ(inner) => collect_level_params_rec(inner, acc),
        Level::Max(a, b) | Level::IMax(a, b) => {
            collect_level_params_rec(a, acc);
            collect_level_params_rec(b, acc);
        }
        Level::Zero | Level::MVar(_) => {}
    }
}
/// Substitute a `Param` name with a concrete level in a level expression.
#[allow(dead_code)]
pub fn subst_level_param(l: &Level, name: &oxilean_kernel::Name, replacement: &Level) -> Level {
    match l {
        Level::Param(n) if n == name => replacement.clone(),
        Level::Zero | Level::Param(_) | Level::MVar(_) => l.clone(),
        Level::Succ(inner) => Level::succ(subst_level_param(inner, name, replacement)),
        Level::Max(a, b) => Level::max(
            subst_level_param(a, name, replacement),
            subst_level_param(b, name, replacement),
        ),
        Level::IMax(a, b) => Level::imax(
            subst_level_param(a, name, replacement),
            subst_level_param(b, name, replacement),
        ),
    }
}
/// Substitute multiple `Param` names simultaneously.
#[allow(dead_code)]
pub fn subst_level_params(l: &Level, subst: &[(oxilean_kernel::Name, Level)]) -> Level {
    subst.iter().fold(l.clone(), |acc, (name, repl)| {
        subst_level_param(&acc, name, repl)
    })
}
/// Count the number of `Succ` wrappers at the top level.
#[allow(dead_code)]
pub fn level_succ_depth(l: &Level) -> u32 {
    let mut depth = 0u32;
    let mut cur = l;
    while let Level::Succ(inner) = cur {
        depth += 1;
        cur = inner;
    }
    depth
}
/// Peel off exactly `n` `Succ` layers; return `None` if there are fewer than `n`.
#[allow(dead_code)]
pub fn peel_level_succs(l: &Level, n: u32) -> Option<&Level> {
    let mut cur = l;
    for _ in 0..n {
        match cur {
            Level::Succ(inner) => cur = inner,
            _ => return None,
        }
    }
    Some(cur)
}
/// Return a lower bound for a level (the minimum it can be, treating `Param` as 0).
#[allow(dead_code)]
pub fn level_min_value(l: &Level) -> u32 {
    match l {
        Level::Zero => 0,
        Level::Succ(inner) => level_min_value(inner) + 1,
        Level::Max(a, b) => level_min_value(a).max(level_min_value(b)),
        Level::IMax(_, b) => level_min_value(b),
        Level::Param(_) | Level::MVar(_) => 0,
    }
}
/// Check whether `l1` is guaranteed to be definitionally equal to `l2`
/// without knowing the values of `Param` variables.
///
/// This is a conservative check: returns `true` only when equality is
/// structurally obvious.
#[allow(dead_code)]
pub fn levels_obviously_equal(l1: &Level, l2: &Level) -> bool {
    oxilean_kernel::level::is_equivalent(l1, l2)
}
/// Check whether `l1` is guaranteed not equal to `l2`.
///
/// Returns `true` only when we can determine inequality from concrete values.
#[allow(dead_code)]
pub fn levels_definitely_distinct(l1: &Level, l2: &Level) -> bool {
    let n1 = level_to_nat_opt(l1);
    let n2 = level_to_nat_opt(l2);
    match (n1, n2) {
        (Some(a), Some(b)) => a != b,
        _ => false,
    }
}
/// Convert a closed level (no Param/MVar) to a natural number.
#[allow(dead_code)]
pub fn level_to_nat_opt(l: &Level) -> Option<u32> {
    match l {
        Level::Zero => Some(0),
        Level::Succ(inner) => level_to_nat_opt(inner).map(|n| n + 1),
        Level::Max(a, b) => {
            let na = level_to_nat_opt(a)?;
            let nb = level_to_nat_opt(b)?;
            Some(na.max(nb))
        }
        Level::IMax(a, b) => {
            let na = level_to_nat_opt(a)?;
            let nb = level_to_nat_opt(b)?;
            if nb == 0 {
                Some(0)
            } else {
                Some(na.max(nb))
            }
        }
        Level::Param(_) | Level::MVar(_) => None,
    }
}
/// Format a level as a human-readable string for debugging.
#[allow(dead_code)]
pub fn format_level(l: &Level) -> String {
    match l {
        Level::Zero => "0".to_string(),
        Level::Succ(inner) => format!("(succ {})", format_level(inner)),
        Level::Max(a, b) => format!("(max {} {})", format_level(a), format_level(b)),
        Level::IMax(a, b) => format!("(imax {} {})", format_level(a), format_level(b)),
        Level::Param(n) => n.to_string(),
        Level::MVar(id) => format!("?{}", id.0),
    }
}
#[cfg(test)]
mod level_def_eq_extra_tests {
    use super::*;
    use crate::level_def_eq::*;
    use oxilean_kernel::{Level, Name};
    #[test]
    fn test_has_level_params_zero() {
        assert!(!has_level_params(&Level::zero()));
    }
    #[test]
    fn test_has_level_params_param() {
        assert!(has_level_params(&Level::param(Name::str("u"))));
    }
    #[test]
    fn test_collect_level_params() {
        let l = Level::max(Level::param(Name::str("u")), Level::param(Name::str("v")));
        let params = collect_level_params(&l);
        assert!(params.contains(&Name::str("u")));
        assert!(params.contains(&Name::str("v")));
        assert_eq!(params.len(), 2);
    }
    #[test]
    fn test_subst_level_param() {
        let l = Level::param(Name::str("u"));
        let replacement = Level::succ(Level::zero());
        let result = subst_level_param(&l, &Name::str("u"), &replacement);
        assert!(oxilean_kernel::level::is_equivalent(&result, &replacement));
    }
    #[test]
    fn test_subst_level_param_other() {
        let l = Level::param(Name::str("v"));
        let replacement = Level::succ(Level::zero());
        let result = subst_level_param(&l, &Name::str("u"), &replacement);
        assert!(oxilean_kernel::level::is_equivalent(&result, &l));
    }
    #[test]
    fn test_level_succ_depth() {
        let l = Level::succ(Level::succ(Level::zero()));
        assert_eq!(level_succ_depth(&l), 2);
    }
    #[test]
    fn test_peel_level_succs_ok() {
        let l = Level::succ(Level::succ(Level::zero()));
        let peeled = peel_level_succs(&l, 1).expect("peeled should be present");
        assert_eq!(level_succ_depth(peeled), 1);
    }
    #[test]
    fn test_peel_level_succs_too_many() {
        let l = Level::succ(Level::zero());
        assert!(peel_level_succs(&l, 3).is_none());
    }
    #[test]
    fn test_level_to_nat_opt_zero() {
        assert_eq!(level_to_nat_opt(&Level::zero()), Some(0));
    }
    #[test]
    fn test_level_to_nat_opt_succ() {
        assert_eq!(level_to_nat_opt(&Level::succ(Level::zero())), Some(1));
    }
    #[test]
    fn test_level_to_nat_opt_param() {
        assert_eq!(level_to_nat_opt(&Level::param(Name::str("u"))), None);
    }
    #[test]
    fn test_levels_definitely_distinct() {
        assert!(levels_definitely_distinct(
            &Level::zero(),
            &Level::succ(Level::zero())
        ));
        assert!(!levels_definitely_distinct(
            &Level::param(Name::str("u")),
            &Level::zero()
        ));
    }
    #[test]
    fn test_format_level_zero() {
        assert_eq!(format_level(&Level::zero()), "0");
    }
    #[test]
    fn test_format_level_succ() {
        assert_eq!(format_level(&Level::succ(Level::zero())), "(succ 0)");
    }
    #[test]
    fn test_constraint_solver_propagate() {
        let mut solver = LevelConstraintSolver::new();
        let mv = Level::MVar(LevelMVarId(0));
        let one = Level::succ(Level::zero());
        solver.add_eq(mv, one.clone());
        solver.propagate();
        assert_eq!(solver.assignments.get(&LevelMVarId(0)), Some(&one));
    }
    #[test]
    fn test_constraint_solver_check_equalities() {
        let mut solver = LevelConstraintSolver::new();
        solver.add_eq(Level::zero(), Level::zero());
        solver.propagate();
        assert!(solver.check_equalities());
    }
    #[test]
    fn test_constraint_solver_check_leqs() {
        let mut solver = LevelConstraintSolver::new();
        solver.add_leq(Level::zero(), Level::succ(Level::zero()));
        solver.propagate();
        assert!(solver.check_leqs());
    }
}
#[cfg(test)]
mod leveldefeq_ext2_tests {
    use super::*;
    use crate::level_def_eq::*;
    #[test]
    fn test_leveldefeq_ext_util_basic() {
        let mut u = LevelDefEqExtUtil::new("test");
        u.push(10);
        u.push(20);
        assert_eq!(u.sum(), 30);
        assert_eq!(u.len(), 2);
    }
    #[test]
    fn test_leveldefeq_ext_util_min_max() {
        let mut u = LevelDefEqExtUtil::new("mm");
        u.push(5);
        u.push(1);
        u.push(9);
        assert_eq!(u.min_val(), Some(1));
        assert_eq!(u.max_val(), Some(9));
    }
    #[test]
    fn test_leveldefeq_ext_util_flags() {
        let mut u = LevelDefEqExtUtil::new("flags");
        u.set_flag(3);
        assert!(u.has_flag(3));
        assert!(!u.has_flag(2));
    }
    #[test]
    fn test_leveldefeq_ext_util_pop() {
        let mut u = LevelDefEqExtUtil::new("pop");
        u.push(42);
        assert_eq!(u.pop(), Some(42));
        assert!(u.is_empty());
    }
    #[test]
    fn test_leveldefeq_ext_map_basic() {
        let mut m: LevelDefEqExtMap<i32> = LevelDefEqExtMap::new();
        m.insert("key", 42);
        assert_eq!(m.get("key"), Some(&42));
        assert!(m.contains("key"));
        assert!(!m.contains("other"));
    }
    #[test]
    fn test_leveldefeq_ext_map_get_or_default() {
        let mut m: LevelDefEqExtMap<i32> = LevelDefEqExtMap::new();
        m.insert("k", 5);
        assert_eq!(m.get_or_default("k"), 5);
        assert_eq!(m.get_or_default("missing"), 0);
    }
    #[test]
    fn test_leveldefeq_ext_map_keys_sorted() {
        let mut m: LevelDefEqExtMap<i32> = LevelDefEqExtMap::new();
        m.insert("z", 1);
        m.insert("a", 2);
        m.insert("m", 3);
        let keys = m.keys_sorted();
        assert_eq!(keys[0].as_str(), "a");
        assert_eq!(keys[2].as_str(), "z");
    }
    #[test]
    fn test_leveldefeq_window_mean() {
        let mut w = LevelDefEqWindow::new(3);
        w.push(1.0);
        w.push(2.0);
        w.push(3.0);
        assert!((w.mean() - 2.0).abs() < 1e-10);
    }
    #[test]
    fn test_leveldefeq_window_evict() {
        let mut w = LevelDefEqWindow::new(2);
        w.push(10.0);
        w.push(20.0);
        w.push(30.0);
        assert_eq!(w.len(), 2);
        assert!((w.mean() - 25.0).abs() < 1e-10);
    }
    #[test]
    fn test_leveldefeq_window_std_dev() {
        let mut w = LevelDefEqWindow::new(10);
        for i in 0..10 {
            w.push(i as f64);
        }
        assert!(w.std_dev() > 0.0);
    }
    #[test]
    fn test_leveldefeq_builder_basic() {
        let b = LevelDefEqBuilder::new("test")
            .add_item("a")
            .add_item("b")
            .set_config("key", "val");
        assert_eq!(b.item_count(), 2);
        assert!(b.has_config("key"));
        assert_eq!(b.get_config("key"), Some("val"));
    }
    #[test]
    fn test_leveldefeq_builder_summary() {
        let b = LevelDefEqBuilder::new("suite").add_item("x");
        let s = b.build_summary();
        assert!(s.contains("suite"));
    }
    #[test]
    fn test_leveldefeq_state_machine_start() {
        let mut sm = LevelDefEqStateMachine::new();
        assert!(sm.start());
        assert!(sm.state.is_running());
    }
    #[test]
    fn test_leveldefeq_state_machine_complete() {
        let mut sm = LevelDefEqStateMachine::new();
        sm.start();
        sm.complete();
        assert!(sm.state.is_terminal());
    }
    #[test]
    fn test_leveldefeq_state_machine_fail() {
        let mut sm = LevelDefEqStateMachine::new();
        sm.fail("oops");
        assert!(sm.state.is_terminal());
        assert_eq!(sm.state.error_msg(), Some("oops"));
    }
    #[test]
    fn test_leveldefeq_state_machine_no_transition_after_terminal() {
        let mut sm = LevelDefEqStateMachine::new();
        sm.complete();
        assert!(!sm.start());
    }
    #[test]
    fn test_leveldefeq_work_queue_basic() {
        let mut wq = LevelDefEqWorkQueue::new(10);
        wq.enqueue("task1".to_string());
        wq.enqueue("task2".to_string());
        assert_eq!(wq.pending_count(), 2);
        let t = wq.dequeue();
        assert_eq!(t, Some("task1".to_string()));
        assert_eq!(wq.processed_count(), 1);
    }
    #[test]
    fn test_leveldefeq_work_queue_capacity() {
        let mut wq = LevelDefEqWorkQueue::new(2);
        wq.enqueue("a".to_string());
        wq.enqueue("b".to_string());
        assert!(wq.is_full());
        assert!(!wq.enqueue("c".to_string()));
    }
    #[test]
    fn test_leveldefeq_counter_map_basic() {
        let mut cm = LevelDefEqCounterMap::new();
        cm.increment("apple");
        cm.increment("apple");
        cm.increment("banana");
        assert_eq!(cm.count("apple"), 2);
        assert_eq!(cm.count("banana"), 1);
        assert_eq!(cm.num_unique(), 2);
    }
    #[test]
    fn test_leveldefeq_counter_map_frequency() {
        let mut cm = LevelDefEqCounterMap::new();
        cm.increment("a");
        cm.increment("a");
        cm.increment("b");
        assert!((cm.frequency("a") - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_leveldefeq_counter_map_most_common() {
        let mut cm = LevelDefEqCounterMap::new();
        cm.increment("x");
        cm.increment("y");
        cm.increment("x");
        let (k, v) = cm.most_common().expect("most_common should succeed");
        assert_eq!(k.as_str(), "x");
        assert_eq!(v, 2);
    }
}
#[cfg(test)]
mod leveldefeq_analysis_tests {
    use super::*;
    use crate::level_def_eq::*;
    #[test]
    fn test_leveldefeq_result_ok() {
        let r = LevelDefEqResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_leveldefeq_result_err() {
        let r = LevelDefEqResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_leveldefeq_result_partial() {
        let r = LevelDefEqResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_leveldefeq_result_skipped() {
        let r = LevelDefEqResult::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_leveldefeq_analysis_pass_run() {
        let mut p = LevelDefEqAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_leveldefeq_analysis_pass_empty_input() {
        let mut p = LevelDefEqAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_leveldefeq_analysis_pass_success_rate() {
        let mut p = LevelDefEqAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_leveldefeq_analysis_pass_disable() {
        let mut p = LevelDefEqAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_leveldefeq_pipeline_basic() {
        let mut pipeline = LevelDefEqPipeline::new("main_pipeline");
        pipeline.add_pass(LevelDefEqAnalysisPass::new("pass1"));
        pipeline.add_pass(LevelDefEqAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_leveldefeq_pipeline_disabled_pass() {
        let mut pipeline = LevelDefEqPipeline::new("partial");
        let mut p = LevelDefEqAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(LevelDefEqAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_leveldefeq_diff_basic() {
        let mut d = LevelDefEqDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_leveldefeq_diff_summary() {
        let mut d = LevelDefEqDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }
    #[test]
    fn test_leveldefeq_config_set_get() {
        let mut cfg = LevelDefEqConfig::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_leveldefeq_config_read_only() {
        let mut cfg = LevelDefEqConfig::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_leveldefeq_config_remove() {
        let mut cfg = LevelDefEqConfig::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_leveldefeq_diagnostics_basic() {
        let mut diag = LevelDefEqDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_leveldefeq_diagnostics_max_errors() {
        let mut diag = LevelDefEqDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_leveldefeq_diagnostics_clear() {
        let mut diag = LevelDefEqDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_leveldefeq_config_value_types() {
        let b = LevelDefEqConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = LevelDefEqConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = LevelDefEqConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = LevelDefEqConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = LevelDefEqConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
#[cfg(test)]
mod level_def_eq_ext_tests_1200 {
    use super::*;
    use crate::level_def_eq::*;
    #[test]
    fn test_level_def_eq_ext_result_ok_1200() {
        let r = LevelDefEqExtResult1200::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_level_def_eq_ext_result_err_1200() {
        let r = LevelDefEqExtResult1200::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_level_def_eq_ext_result_partial_1200() {
        let r = LevelDefEqExtResult1200::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_level_def_eq_ext_result_skipped_1200() {
        let r = LevelDefEqExtResult1200::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_level_def_eq_ext_pass_run_1200() {
        let mut p = LevelDefEqExtPass1200::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_level_def_eq_ext_pass_empty_1200() {
        let mut p = LevelDefEqExtPass1200::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_level_def_eq_ext_pass_rate_1200() {
        let mut p = LevelDefEqExtPass1200::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_level_def_eq_ext_pass_disable_1200() {
        let mut p = LevelDefEqExtPass1200::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_level_def_eq_ext_pipeline_basic_1200() {
        let mut pipeline = LevelDefEqExtPipeline1200::new("main_pipeline");
        pipeline.add_pass(LevelDefEqExtPass1200::new("pass1"));
        pipeline.add_pass(LevelDefEqExtPass1200::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_level_def_eq_ext_pipeline_disabled_1200() {
        let mut pipeline = LevelDefEqExtPipeline1200::new("partial");
        let mut p = LevelDefEqExtPass1200::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(LevelDefEqExtPass1200::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_level_def_eq_ext_diff_basic_1200() {
        let mut d = LevelDefEqExtDiff1200::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_level_def_eq_ext_config_set_get_1200() {
        let mut cfg = LevelDefEqExtConfig1200::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_level_def_eq_ext_config_read_only_1200() {
        let mut cfg = LevelDefEqExtConfig1200::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_level_def_eq_ext_config_remove_1200() {
        let mut cfg = LevelDefEqExtConfig1200::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_level_def_eq_ext_diagnostics_basic_1200() {
        let mut diag = LevelDefEqExtDiag1200::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_level_def_eq_ext_diagnostics_max_errors_1200() {
        let mut diag = LevelDefEqExtDiag1200::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_level_def_eq_ext_diagnostics_clear_1200() {
        let mut diag = LevelDefEqExtDiag1200::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_level_def_eq_ext_config_value_types_1200() {
        let b = LevelDefEqExtConfigVal1200::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = LevelDefEqExtConfigVal1200::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = LevelDefEqExtConfigVal1200::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = LevelDefEqExtConfigVal1200::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = LevelDefEqExtConfigVal1200::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
