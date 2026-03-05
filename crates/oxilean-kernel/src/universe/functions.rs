//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::level;
use crate::{Level, Name};
use std::collections::{HashMap, HashSet};

use super::types::{
    ConfigNode, DecisionNode, Either2, FlatSubstitution, FocusStack, LabelSet,
    LevelComparisonTable, LevelNormalForm, NonEmptyVec, PathBuf, RewriteRule, RewriteRuleSet,
    SimpleDag, SlidingSum, SmallMap, SparseVec, StackCalc, StatSummary, Stopwatch, StringPool,
    TokenBucket, TransformStat, TransitiveClosure, UnivChecker, UnivConstraint, UnivConstraintSet,
    UnivPolySignature, UnivSatChecker, UniverseInstantiation, VersionedRecord, WindowIterator,
    WriteOnce,
};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_checker() {
        let checker = UnivChecker::new();
        assert_eq!(checker.all_constraints().len(), 0);
        assert_eq!(checker.all_univ_vars().len(), 0);
    }
    #[test]
    fn test_add_univ_var() {
        let mut checker = UnivChecker::new();
        checker.add_univ_var(Name::str("u"));
        assert_eq!(checker.all_univ_vars().len(), 1);
    }
    #[test]
    fn test_satisfiable_lt() {
        let mut checker = UnivChecker::new();
        checker.add_constraint(UnivConstraint::Lt(
            Level::zero(),
            Level::succ(Level::zero()),
        ));
        assert!(checker.check().is_ok());
    }
    #[test]
    fn test_unsatisfiable_lt() {
        let mut checker = UnivChecker::new();
        let u = Level::succ(Level::zero());
        checker.add_constraint(UnivConstraint::Lt(u.clone(), u));
        assert!(checker.check().is_err());
    }
    #[test]
    fn test_satisfiable_eq() {
        let mut checker = UnivChecker::new();
        let u = Level::zero();
        checker.add_constraint(UnivConstraint::Eq(u.clone(), u));
        assert!(checker.check().is_ok());
    }
    #[test]
    fn test_le_constraint() {
        let mut checker = UnivChecker::new();
        checker.add_constraint(UnivConstraint::Le(
            Level::zero(),
            Level::succ(Level::zero()),
        ));
        assert!(checker.check().is_ok());
    }
    #[test]
    fn test_le_equal() {
        let mut checker = UnivChecker::new();
        let l = Level::succ(Level::zero());
        checker.add_constraint(UnivConstraint::Le(l.clone(), l));
        assert!(checker.check().is_ok());
    }
    #[test]
    fn test_le_violation() {
        let mut checker = UnivChecker::new();
        checker.add_constraint(UnivConstraint::Le(
            Level::succ(Level::succ(Level::zero())),
            Level::succ(Level::zero()),
        ));
        assert!(checker.check().is_err());
    }
    #[test]
    fn test_level_def_eq_normalized() {
        let checker = UnivChecker::new();
        let l1 = Level::max(Level::param(Name::str("u")), Level::param(Name::str("v")));
        let l2 = Level::max(Level::param(Name::str("v")), Level::param(Name::str("u")));
        assert!(checker.is_level_def_eq(&l1, &l2));
    }
    #[test]
    fn test_level_mvar_assignment() {
        let mut checker = UnivChecker::new();
        let m = checker.fresh_level_mvar();
        let one = Level::succ(Level::zero());
        if let Level::MVar(id) = &m {
            checker.assign_mvar(*id, one.clone());
        }
        checker.add_constraint(UnivConstraint::Eq(m, one));
        assert!(checker.check().is_ok());
    }
    #[test]
    fn test_solve_simple() {
        let mut checker = UnivChecker::new();
        let m = checker.fresh_level_mvar();
        let two = Level::succ(Level::succ(Level::zero()));
        checker.add_constraint(UnivConstraint::Eq(m.clone(), two.clone()));
        assert!(checker.solve_simple());
        if let Level::MVar(id) = &m {
            assert_eq!(checker.get_mvar_assignment(id), Some(&two));
        }
    }
    #[test]
    fn test_is_geq() {
        let checker = UnivChecker::new();
        assert!(checker.is_geq(&Level::succ(Level::zero()), &Level::zero()));
        assert!(checker.is_geq(&Level::zero(), &Level::zero()));
        assert!(!checker.is_geq(&Level::zero(), &Level::succ(Level::zero())));
    }
    #[test]
    fn test_is_gt() {
        let checker = UnivChecker::new();
        assert!(checker.is_gt(&Level::succ(Level::zero()), &Level::zero()));
        assert!(!checker.is_gt(&Level::zero(), &Level::zero()));
    }
    #[test]
    fn test_clear() {
        let mut checker = UnivChecker::new();
        checker.add_constraint(UnivConstraint::Eq(Level::zero(), Level::zero()));
        let m = checker.fresh_level_mvar();
        if let Level::MVar(id) = &m {
            checker.assign_mvar(*id, Level::zero());
        }
        checker.clear();
        assert_eq!(checker.all_constraints().len(), 0);
    }
}
/// Compute the maximum of two universe levels.
pub fn level_max(l1: Level, l2: Level) -> Level {
    Level::max(l1, l2)
}
/// Compute succ(l).
pub fn level_succ(l: Level) -> Level {
    Level::succ(l)
}
/// Compute imax(l1, l2).
pub fn level_imax(l1: Level, l2: Level) -> Level {
    Level::imax(l1, l2)
}
/// The type of Sort(l) is Sort(succ(l)).
pub fn sort_type_level(l: &Level) -> Level {
    Level::succ(l.clone())
}
/// The Pi type level: imax(domain, codomain).
pub fn pi_type_level(domain: &Level, codomain: &Level) -> Level {
    Level::imax(domain.clone(), codomain.clone())
}
/// Convert a concrete level to a natural number.
pub fn level_to_nat(l: &Level) -> Option<u32> {
    match l {
        Level::Zero => Some(0),
        Level::Succ(inner) => level_to_nat(inner).map(|n| n + 1),
        Level::Max(a, b) => {
            let a_n = level_to_nat(a)?;
            let b_n = level_to_nat(b)?;
            Some(a_n.max(b_n))
        }
        Level::IMax(a, b) => {
            let b_n = level_to_nat(b)?;
            if b_n == 0 {
                Some(0)
            } else {
                let a_n = level_to_nat(a)?;
                Some(a_n.max(b_n))
            }
        }
        Level::Param(_) | Level::MVar(_) => None,
    }
}
/// Prop = Sort 0.
pub fn prop_level() -> Level {
    Level::zero()
}
/// Type 0 = Sort 1.
pub fn type0_level() -> Level {
    Level::succ(Level::zero())
}
/// Type 1 = Sort 2.
pub fn type1_level() -> Level {
    Level::succ(Level::succ(Level::zero()))
}
/// Check if a level is definitionally Prop.
pub fn is_prop_level(l: &Level) -> bool {
    level::is_equivalent(l, &Level::zero())
}
/// Count succ constructors in a concrete level.
pub fn count_succs(l: &Level) -> Option<u32> {
    match l {
        Level::Zero => Some(0),
        Level::Succ(inner) => count_succs(inner).map(|n| n + 1),
        _ => None,
    }
}
/// Peel off n succ constructors.
pub fn peel_succs(l: &Level, n: u32) -> Option<Level> {
    if n == 0 {
        return Some(l.clone());
    }
    match l {
        Level::Succ(inner) => peel_succs(inner, n - 1),
        _ => None,
    }
}
/// Add n succ constructors to a level.
pub fn add_succs(l: Level, n: u32) -> Level {
    let mut r = l;
    for _ in 0..n {
        r = Level::succ(r);
    }
    r
}
/// Collect universe parameters in a level expression.
pub fn collect_level_params(l: &Level) -> Vec<Name> {
    let mut params = std::collections::HashSet::new();
    collect_level_params_impl(l, &mut params);
    let mut result: Vec<Name> = params.into_iter().collect();
    result.sort_by(|a, b| format!("{}", a).cmp(&format!("{}", b)));
    result
}
fn collect_level_params_impl(l: &Level, params: &mut std::collections::HashSet<Name>) {
    match l {
        Level::Param(name) => {
            params.insert(name.clone());
        }
        Level::Succ(inner) => collect_level_params_impl(inner, params),
        Level::Max(a, b) | Level::IMax(a, b) => {
            collect_level_params_impl(a, params);
            collect_level_params_impl(b, params);
        }
        Level::Zero | Level::MVar(_) => {}
    }
}
/// Substitute a universe parameter.
pub fn substitute_level_param(l: &Level, param_name: &Name, replacement: &Level) -> Level {
    match l {
        Level::Param(name) if name == param_name => replacement.clone(),
        Level::Succ(inner) => Level::succ(substitute_level_param(inner, param_name, replacement)),
        Level::Max(a, b) => Level::max(
            substitute_level_param(a, param_name, replacement),
            substitute_level_param(b, param_name, replacement),
        ),
        Level::IMax(a, b) => Level::imax(
            substitute_level_param(a, param_name, replacement),
            substitute_level_param(b, param_name, replacement),
        ),
        Level::Zero | Level::Param(_) | Level::MVar(_) => l.clone(),
    }
}
/// Format a level as a string.
pub fn format_level(l: &Level) -> String {
    match l {
        Level::Zero => "0".to_string(),
        Level::Succ(_) => {
            if let Some(n) = count_succs(l) {
                if n <= 4 {
                    return n.to_string();
                }
            }
            if let Level::Succ(inner) = l {
                format!("succ({})", format_level(inner))
            } else {
                "?".to_string()
            }
        }
        Level::Max(a, b) => format!("max({}, {})", format_level(a), format_level(b)),
        Level::IMax(a, b) => format!("imax({}, {})", format_level(a), format_level(b)),
        Level::Param(name) => format!("{}", name),
        Level::MVar(id) => format!("?u{}", id.0),
    }
}
/// Parse a simple level from string.
pub fn parse_level_str(s: &str) -> Option<Level> {
    match s.trim() {
        "0" => Some(Level::zero()),
        "1" => Some(Level::succ(Level::zero())),
        "2" => Some(Level::succ(Level::succ(Level::zero()))),
        "Prop" => Some(Level::zero()),
        "Type" => Some(Level::succ(Level::zero())),
        s if s.starts_with("succ(") && s.ends_with(')') => {
            parse_level_str(&s[5..s.len() - 1]).map(Level::succ)
        }
        s if s
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '.') =>
        {
            Some(Level::param(Name::str(s)))
        }
        _ => None,
    }
}
pub(super) fn collect_nf_comps(l: &Level, offset: u32, comps: &mut Vec<(Option<Name>, u32)>) {
    match l {
        Level::Zero => comps.push((None, offset)),
        Level::Param(name) => comps.push((Some(name.clone()), offset)),
        Level::Succ(inner) => collect_nf_comps(inner, offset + 1, comps),
        Level::Max(a, b) => {
            collect_nf_comps(a, offset, comps);
            collect_nf_comps(b, offset, comps);
        }
        Level::IMax(a, b) => {
            collect_nf_comps(a, offset, comps);
            collect_nf_comps(b, offset, comps);
        }
        Level::MVar(_) => {}
    }
}
#[cfg(test)]
mod universe_arith_tests {
    use super::*;
    #[test]
    fn test_level_to_nat_zero() {
        assert_eq!(level_to_nat(&Level::zero()), Some(0));
    }
    #[test]
    fn test_level_to_nat_succ() {
        assert_eq!(level_to_nat(&Level::succ(Level::zero())), Some(1));
        assert_eq!(
            level_to_nat(&Level::succ(Level::succ(Level::zero()))),
            Some(2)
        );
    }
    #[test]
    fn test_level_to_nat_param() {
        assert_eq!(level_to_nat(&Level::param(Name::str("u"))), None);
    }
    #[test]
    fn test_is_prop_level() {
        assert!(is_prop_level(&Level::zero()));
        assert!(!is_prop_level(&Level::succ(Level::zero())));
    }
    #[test]
    fn test_pi_type_level() {
        let pi_l = pi_type_level(&Level::zero(), &Level::succ(Level::zero()));
        assert!(level::is_equivalent(&pi_l, &Level::succ(Level::zero())));
    }
    #[test]
    fn test_collect_level_params() {
        let l = Level::max(
            Level::param(Name::str("u")),
            Level::succ(Level::param(Name::str("v"))),
        );
        let params = collect_level_params(&l);
        assert!(params.contains(&Name::str("u")));
        assert!(params.contains(&Name::str("v")));
    }
    #[test]
    fn test_substitute_level_param() {
        let l = Level::succ(Level::param(Name::str("u")));
        let result = substitute_level_param(&l, &Name::str("u"), &Level::zero());
        assert_eq!(result, Level::succ(Level::zero()));
    }
    #[test]
    fn test_add_succs() {
        let result = add_succs(Level::zero(), 3);
        assert_eq!(level_to_nat(&result), Some(3));
    }
    #[test]
    fn test_peel_succs() {
        let l = Level::succ(Level::succ(Level::succ(Level::zero())));
        assert_eq!(peel_succs(&l, 3), Some(Level::zero()));
        assert_eq!(peel_succs(&l, 4), None);
    }
    #[test]
    fn test_parse_level_str() {
        assert_eq!(parse_level_str("0"), Some(Level::zero()));
        assert_eq!(parse_level_str("1"), Some(Level::succ(Level::zero())));
        assert_eq!(parse_level_str("u"), Some(Level::param(Name::str("u"))));
    }
    #[test]
    fn test_format_level() {
        assert_eq!(format_level(&Level::zero()), "0");
        assert_eq!(format_level(&Level::succ(Level::zero())), "1");
    }
    #[test]
    fn test_sat_checker() {
        let mut checker = UnivSatChecker::new();
        checker.add_lower_bound(Name::str("u"), 1);
        checker.add_upper_bound(Name::str("u"), 3);
        assert!(checker.is_satisfiable());
    }
    #[test]
    fn test_sat_checker_unsat() {
        let mut checker = UnivSatChecker::new();
        checker.add_lower_bound(Name::str("u"), 5);
        checker.add_upper_bound(Name::str("u"), 3);
        assert!(!checker.is_satisfiable());
    }
    #[test]
    fn test_sort_type_level() {
        assert_eq!(sort_type_level(&Level::zero()), Level::succ(Level::zero()));
    }
    #[test]
    fn test_prop_level() {
        assert_eq!(prop_level(), Level::zero());
    }
    #[test]
    fn test_count_succs() {
        assert_eq!(count_succs(&Level::zero()), Some(0));
        assert_eq!(count_succs(&Level::succ(Level::zero())), Some(1));
    }
    #[test]
    fn test_level_normal_form() {
        let l = Level::max(Level::succ(Level::zero()), Level::zero());
        let nf = LevelNormalForm::from_level(&l);
        assert!(!nf.components.is_empty());
    }
    #[test]
    fn test_type0_type1() {
        assert_eq!(level_to_nat(&type0_level()), Some(1));
        assert_eq!(level_to_nat(&type1_level()), Some(2));
    }
    #[test]
    fn test_level_max_fn() {
        let l = level_max(Level::zero(), Level::succ(Level::zero()));
        assert!(level::is_equivalent(&l, &Level::succ(Level::zero())));
    }
    #[test]
    fn test_level_succ_fn() {
        assert_eq!(level_succ(Level::zero()), Level::succ(Level::zero()));
    }
}
/// Compute the maximum of a list of levels.
///
/// Returns `Level::Zero` for an empty list.
#[allow(dead_code)]
pub fn level_max_many(levels: &[Level]) -> Level {
    levels
        .iter()
        .fold(Level::zero(), |acc, l| level_max(acc, l.clone()))
}
/// Check whether a level is *structurally* a max-expression.
#[allow(dead_code)]
pub fn is_max_level(l: &Level) -> bool {
    matches!(l, Level::Max(_, _))
}
/// Check whether a level is *structurally* an imax-expression.
#[allow(dead_code)]
pub fn is_imax_level(l: &Level) -> bool {
    matches!(l, Level::IMax(_, _))
}
/// Return the depth of nesting of `Succ` constructors.
///
/// `count_succs(succ(succ(zero))) == 2`
#[allow(dead_code)]
pub fn count_succ_depth(l: &Level) -> u32 {
    let mut depth = 0u32;
    let mut cur = l;
    while let Level::Succ(inner) = cur {
        depth += 1;
        cur = inner;
    }
    depth
}
/// Compute the `n`-th successor of a level.
///
/// `add_succ_n(zero, 3) == succ(succ(succ(zero)))`
#[allow(dead_code)]
pub fn add_succ_n(l: Level, n: u32) -> Level {
    (0..n).fold(l, |acc, _| Level::succ(acc))
}
/// Attempt to evaluate a closed level to a concrete `u32`.
///
/// Returns `None` if the level contains `Param` or `MVar` nodes.
#[allow(dead_code)]
pub fn eval_closed_level(l: &Level) -> Option<u32> {
    level_to_nat(l)
}
/// Check whether two levels have the same structural shape (ignoring parameters).
#[allow(dead_code)]
pub fn same_level_shape(l1: &Level, l2: &Level) -> bool {
    match (l1, l2) {
        (Level::Zero, Level::Zero) => true,
        (Level::Succ(a), Level::Succ(b)) => same_level_shape(a, b),
        (Level::Max(a1, b1), Level::Max(a2, b2)) => {
            same_level_shape(a1, a2) && same_level_shape(b1, b2)
        }
        (Level::IMax(a1, b1), Level::IMax(a2, b2)) => {
            same_level_shape(a1, a2) && same_level_shape(b1, b2)
        }
        (Level::Param(_), Level::Param(_)) => true,
        (Level::MVar(_), Level::MVar(_)) => true,
        _ => false,
    }
}
/// Check if an instantiation is valid for a given list of universe parameters.
pub fn is_valid_instantiation(params: &[Name], inst: &UniverseInstantiation) -> bool {
    params.iter().all(|p| inst.subst.contains_key(p))
}
/// Instantiate a list of levels using the given substitution map.
pub fn instantiate_levels(levels: &[Level], params: &[Name], args: &[Level]) -> Vec<Level> {
    let mut inst = UniverseInstantiation::new();
    for (p, l) in params.iter().zip(args.iter()) {
        inst.add(p.clone(), l.clone());
    }
    levels.iter().map(|l| inst.apply(l)).collect()
}
#[cfg(test)]
mod poly_tests {
    use super::*;
    #[test]
    fn test_universe_instantiation_apply() {
        let mut inst = UniverseInstantiation::new();
        inst.add(Name::str("u"), Level::succ(Level::zero()));
        let l = Level::param(Name::str("u"));
        let result = inst.apply(&l);
        assert_eq!(result, Level::succ(Level::zero()));
    }
    #[test]
    fn test_universe_instantiation_compose() {
        let mut inst1 = UniverseInstantiation::new();
        inst1.add(Name::str("v"), Level::zero());
        let mut inst2 = UniverseInstantiation::new();
        inst2.add(Name::str("u"), Level::param(Name::str("v")));
        let composed = inst1.compose(&inst2);
        let l = Level::param(Name::str("u"));
        let result = composed.apply(&l);
        assert_eq!(result, Level::zero());
    }
    #[test]
    fn test_is_valid_instantiation() {
        let mut inst = UniverseInstantiation::new();
        inst.add(Name::str("u"), Level::zero());
        let params = vec![Name::str("u")];
        assert!(is_valid_instantiation(&params, &inst));
        let params2 = vec![Name::str("u"), Name::str("v")];
        assert!(!is_valid_instantiation(&params2, &inst));
    }
    #[test]
    fn test_instantiate_levels() {
        let levels = vec![
            Level::param(Name::str("u")),
            Level::succ(Level::param(Name::str("u"))),
        ];
        let params = vec![Name::str("u")];
        let args = vec![Level::zero()];
        let result = instantiate_levels(&levels, &params, &args);
        assert_eq!(result[0], Level::zero());
        assert_eq!(result[1], Level::succ(Level::zero()));
    }
    #[test]
    fn test_univ_constraint_set_dedup() {
        let mut s = UnivConstraintSet::new();
        s.add(UnivConstraint::Eq(Level::zero(), Level::zero()));
        s.add(UnivConstraint::Eq(Level::zero(), Level::zero()));
        s.dedup();
        assert_eq!(s.len(), 1);
    }
    #[test]
    fn test_univ_constraint_set_merge() {
        let mut s1 = UnivConstraintSet::new();
        s1.add(UnivConstraint::Le(
            Level::zero(),
            Level::succ(Level::zero()),
        ));
        let mut s2 = UnivConstraintSet::new();
        s2.add(UnivConstraint::Eq(Level::zero(), Level::zero()));
        s1.merge(&s2);
        assert_eq!(s1.len(), 2);
    }
    #[test]
    fn test_univ_poly_signature_instantiate() {
        let sig = UnivPolySignature::new(vec![Name::str("u"), Name::str("v")]);
        let args = vec![Level::zero(), Level::succ(Level::zero())];
        let inst = sig.instantiate(&args).expect("inst should be present");
        assert_eq!(inst.apply(&Level::param(Name::str("u"))), Level::zero());
        assert_eq!(
            inst.apply(&Level::param(Name::str("v"))),
            Level::succ(Level::zero())
        );
    }
    #[test]
    fn test_univ_poly_signature_wrong_arity() {
        let sig = UnivPolySignature::new(vec![Name::str("u")]);
        assert!(sig.instantiate(&[]).is_none());
    }
    #[test]
    fn test_univ_poly_signature_check_ok() {
        let mut sig = UnivPolySignature::new(vec![Name::str("u")]);
        sig.add_constraint(UnivConstraint::Le(
            Level::param(Name::str("u")),
            Level::succ(Level::param(Name::str("u"))),
        ));
        let args = vec![Level::zero()];
        let inst = sig.instantiate(&args).expect("inst should be present");
        assert!(sig.check_instantiation(&inst).is_ok());
    }
    #[test]
    fn test_univ_constraint_display() {
        let c = UnivConstraint::Lt(Level::zero(), Level::succ(Level::zero()));
        let s = format!("{}", c);
        assert!(s.contains('<'));
    }
    #[test]
    fn test_level_comparison_table() {
        let levels = vec![
            Level::zero(),
            Level::succ(Level::zero()),
            Level::succ(Level::succ(Level::zero())),
        ];
        let table = LevelComparisonTable::new(levels);
        assert_eq!(table.len(), 3);
        assert_eq!(table.geq(1, 0), Some(true));
        assert_eq!(table.geq(0, 1), Some(false));
    }
    #[test]
    fn test_level_comparison_table_max() {
        let levels = vec![Level::zero(), Level::succ(Level::zero())];
        let table = LevelComparisonTable::new(levels);
        let max = table.max_idx();
        assert_eq!(max, Some(1));
    }
    #[test]
    fn test_same_level_shape_max() {
        let l1 = Level::max(Level::param(Name::str("u")), Level::param(Name::str("v")));
        let l2 = Level::max(Level::param(Name::str("a")), Level::param(Name::str("b")));
        assert!(same_level_shape(&l1, &l2));
    }
    #[test]
    fn test_count_succ_depth() {
        assert_eq!(count_succ_depth(&Level::zero()), 0);
        assert_eq!(
            count_succ_depth(&Level::succ(Level::succ(Level::zero()))),
            2
        );
    }
    #[test]
    fn test_add_succ_n() {
        let result = add_succ_n(Level::zero(), 4);
        assert_eq!(level_to_nat(&result), Some(4));
    }
    #[test]
    fn test_eval_closed_level_param() {
        let l = Level::param(Name::str("u"));
        assert!(eval_closed_level(&l).is_none());
    }
    #[test]
    fn test_is_max_level() {
        let l = Level::max(Level::zero(), Level::zero());
        assert!(is_max_level(&l));
        assert!(!is_imax_level(&l));
    }
    #[test]
    fn test_is_imax_level() {
        let l = Level::imax(Level::zero(), Level::zero());
        assert!(is_imax_level(&l));
        assert!(!is_max_level(&l));
    }
    #[test]
    fn test_level_max_many_empty() {
        let result = level_max_many(&[]);
        assert_eq!(result, Level::zero());
    }
    #[test]
    fn test_level_max_many_one() {
        let l = Level::succ(Level::zero());
        let result = level_max_many(std::slice::from_ref(&l));
        assert!(crate::level::is_equivalent(&result, &l));
    }
    #[test]
    fn test_univ_instantiation_len() {
        let mut inst = UniverseInstantiation::new();
        assert_eq!(inst.len(), 0);
        inst.add(Name::str("u"), Level::zero());
        assert_eq!(inst.len(), 1);
    }
}
#[cfg(test)]
mod tests_padding_infra {
    use super::*;
    #[test]
    fn test_stat_summary() {
        let mut ss = StatSummary::new();
        ss.record(10.0);
        ss.record(20.0);
        ss.record(30.0);
        assert_eq!(ss.count(), 3);
        assert!((ss.mean().expect("mean should succeed") - 20.0).abs() < 1e-9);
        assert_eq!(ss.min().expect("min should succeed") as i64, 10);
        assert_eq!(ss.max().expect("max should succeed") as i64, 30);
    }
    #[test]
    fn test_transform_stat() {
        let mut ts = TransformStat::new();
        ts.record_before(100.0);
        ts.record_after(80.0);
        let ratio = ts.mean_ratio().expect("ratio should be present");
        assert!((ratio - 0.8).abs() < 1e-9);
    }
    #[test]
    fn test_small_map() {
        let mut m: SmallMap<u32, &str> = SmallMap::new();
        m.insert(3, "three");
        m.insert(1, "one");
        m.insert(2, "two");
        assert_eq!(m.get(&2), Some(&"two"));
        assert_eq!(m.len(), 3);
        let keys = m.keys();
        assert_eq!(*keys[0], 1);
        assert_eq!(*keys[2], 3);
    }
    #[test]
    fn test_label_set() {
        let mut ls = LabelSet::new();
        ls.add("foo");
        ls.add("bar");
        ls.add("foo");
        assert_eq!(ls.count(), 2);
        assert!(ls.has("bar"));
        assert!(!ls.has("baz"));
    }
    #[test]
    fn test_config_node() {
        let mut root = ConfigNode::section("root");
        let child = ConfigNode::leaf("key", "value");
        root.add_child(child);
        assert_eq!(root.num_children(), 1);
    }
    #[test]
    fn test_versioned_record() {
        let mut vr = VersionedRecord::new(0u32);
        vr.update(1);
        vr.update(2);
        assert_eq!(*vr.current(), 2);
        assert_eq!(vr.version(), 2);
        assert!(vr.has_history());
        assert_eq!(*vr.at_version(0).expect("value should be present"), 0);
    }
    #[test]
    fn test_simple_dag() {
        let mut dag = SimpleDag::new(4);
        dag.add_edge(0, 1);
        dag.add_edge(1, 2);
        dag.add_edge(2, 3);
        assert!(dag.can_reach(0, 3));
        assert!(!dag.can_reach(3, 0));
        let order = dag.topological_sort().expect("order should be present");
        assert_eq!(order, vec![0, 1, 2, 3]);
    }
    #[test]
    fn test_focus_stack() {
        let mut fs: FocusStack<&str> = FocusStack::new();
        fs.focus("a");
        fs.focus("b");
        assert_eq!(fs.current(), Some(&"b"));
        assert_eq!(fs.depth(), 2);
        fs.blur();
        assert_eq!(fs.current(), Some(&"a"));
    }
}
#[cfg(test)]
mod tests_extra_iterators {
    use super::*;
    #[test]
    fn test_window_iterator() {
        let data = vec![1u32, 2, 3, 4, 5];
        let windows: Vec<_> = WindowIterator::new(&data, 3).collect();
        assert_eq!(windows.len(), 3);
        assert_eq!(windows[0], &[1, 2, 3]);
        assert_eq!(windows[2], &[3, 4, 5]);
    }
    #[test]
    fn test_non_empty_vec() {
        let mut nev = NonEmptyVec::singleton(10u32);
        nev.push(20);
        nev.push(30);
        assert_eq!(nev.len(), 3);
        assert_eq!(*nev.first(), 10);
        assert_eq!(*nev.last(), 30);
    }
}
#[cfg(test)]
mod tests_padding2 {
    use super::*;
    #[test]
    fn test_sliding_sum() {
        let mut ss = SlidingSum::new(3);
        ss.push(1.0);
        ss.push(2.0);
        ss.push(3.0);
        assert!((ss.sum() - 6.0).abs() < 1e-9);
        ss.push(4.0);
        assert!((ss.sum() - 9.0).abs() < 1e-9);
        assert_eq!(ss.count(), 3);
    }
    #[test]
    fn test_path_buf() {
        let mut pb = PathBuf::new();
        pb.push("src");
        pb.push("main");
        assert_eq!(pb.as_str(), "src/main");
        assert_eq!(pb.depth(), 2);
        pb.pop();
        assert_eq!(pb.as_str(), "src");
    }
    #[test]
    fn test_string_pool() {
        let mut pool = StringPool::new();
        let s = pool.take();
        assert!(s.is_empty());
        pool.give("hello".to_string());
        let s2 = pool.take();
        assert!(s2.is_empty());
        assert_eq!(pool.free_count(), 0);
    }
    #[test]
    fn test_transitive_closure() {
        let mut tc = TransitiveClosure::new(4);
        tc.add_edge(0, 1);
        tc.add_edge(1, 2);
        tc.add_edge(2, 3);
        assert!(tc.can_reach(0, 3));
        assert!(!tc.can_reach(3, 0));
        let r = tc.reachable_from(0);
        assert_eq!(r.len(), 4);
    }
    #[test]
    fn test_token_bucket() {
        let mut tb = TokenBucket::new(100, 10);
        assert_eq!(tb.available(), 100);
        assert!(tb.try_consume(50));
        assert_eq!(tb.available(), 50);
        assert!(!tb.try_consume(60));
        assert_eq!(tb.capacity(), 100);
    }
    #[test]
    fn test_rewrite_rule_set() {
        let mut rrs = RewriteRuleSet::new();
        rrs.add(RewriteRule::unconditional(
            "beta",
            "App(Lam(x, b), v)",
            "b[x:=v]",
        ));
        rrs.add(RewriteRule::conditional("comm", "a + b", "b + a"));
        assert_eq!(rrs.len(), 2);
        assert_eq!(rrs.unconditional_rules().len(), 1);
        assert_eq!(rrs.conditional_rules().len(), 1);
        assert!(rrs.get("beta").is_some());
        let disp = rrs
            .get("beta")
            .expect("element at \'beta\' should exist")
            .display();
        assert!(disp.contains("→"));
    }
}
#[cfg(test)]
mod tests_padding3 {
    use super::*;
    #[test]
    fn test_decision_node() {
        let tree = DecisionNode::Branch {
            key: "x".into(),
            val: "1".into(),
            yes_branch: Box::new(DecisionNode::Leaf("yes".into())),
            no_branch: Box::new(DecisionNode::Leaf("no".into())),
        };
        let mut ctx = std::collections::HashMap::new();
        ctx.insert("x".into(), "1".into());
        assert_eq!(tree.evaluate(&ctx), "yes");
        ctx.insert("x".into(), "2".into());
        assert_eq!(tree.evaluate(&ctx), "no");
        assert_eq!(tree.depth(), 1);
    }
    #[test]
    fn test_flat_substitution() {
        let mut sub = FlatSubstitution::new();
        sub.add("foo", "bar");
        sub.add("baz", "qux");
        assert_eq!(sub.apply("foo and baz"), "bar and qux");
        assert_eq!(sub.len(), 2);
    }
    #[test]
    fn test_stopwatch() {
        let mut sw = Stopwatch::start();
        sw.split();
        sw.split();
        assert_eq!(sw.num_splits(), 2);
        assert!(sw.elapsed_ms() >= 0.0);
        for &s in sw.splits() {
            assert!(s >= 0.0);
        }
    }
    #[test]
    fn test_either2() {
        let e: Either2<i32, &str> = Either2::First(42);
        assert!(e.is_first());
        let mapped = e.map_first(|x| x * 2);
        assert_eq!(mapped.first(), Some(84));
        let e2: Either2<i32, &str> = Either2::Second("hello");
        assert!(e2.is_second());
        assert_eq!(e2.second(), Some("hello"));
    }
    #[test]
    fn test_write_once() {
        let wo: WriteOnce<u32> = WriteOnce::new();
        assert!(!wo.is_written());
        assert!(wo.write(42));
        assert!(!wo.write(99));
        assert_eq!(wo.read(), Some(42));
    }
    #[test]
    fn test_sparse_vec() {
        let mut sv: SparseVec<i32> = SparseVec::new(100);
        sv.set(5, 10);
        sv.set(50, 20);
        assert_eq!(*sv.get(5), 10);
        assert_eq!(*sv.get(50), 20);
        assert_eq!(*sv.get(0), 0);
        assert_eq!(sv.nnz(), 2);
        sv.set(5, 0);
        assert_eq!(sv.nnz(), 1);
    }
    #[test]
    fn test_stack_calc() {
        let mut calc = StackCalc::new();
        calc.push(3);
        calc.push(4);
        calc.add();
        assert_eq!(calc.peek(), Some(7));
        calc.push(2);
        calc.mul();
        assert_eq!(calc.peek(), Some(14));
    }
}
