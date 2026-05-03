//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::Name;

use super::types::{
    ConfigNode, ConstraintSet, DecisionNode, Either2, Fixture, FlatSubstitution, FocusStack,
    LabelSet, Level, LevelConstraint, LevelMVarId, MinHeap, NonEmptyVec, PathBuf, PrefixCounter,
    RewriteRule, RewriteRuleSet, SimpleDag, SlidingSum, SmallMap, SparseVec, StackCalc,
    StatSummary, Stopwatch, StringPool, TokenBucket, TransformStat, TransitiveClosure,
    VersionedRecord, WindowIterator, WriteOnce,
};

/// Decompose a level into `(base, offset)` where level = succ^offset(base).
///
/// Strips all Succ wrappers. E.g., `succ(succ(param u))` -> `(param u, 2)`.
pub fn to_offset(l: &Level) -> (&Level, u32) {
    match l {
        Level::Succ(inner) => {
            let (base, k) = to_offset(inner);
            (base, k + 1)
        }
        _ => (l, 0),
    }
}
/// Reconstruct a level from `(base, offset)` as `succ^offset(base)`.
pub(super) fn from_offset(base: Level, offset: u32) -> Level {
    let mut result = base;
    for _ in 0..offset {
        result = Level::succ(result);
    }
    result
}
/// Collect all arguments of a nested max expression into a flat list.
pub(super) fn push_max_args(l: &Level, args: &mut Vec<Level>) {
    match l {
        Level::Max(l1, l2) => {
            push_max_args(l1, args);
            push_max_args(l2, args);
        }
        _ => args.push(l.clone()),
    }
}
/// Total order on levels for normalization.
///
/// This ordering ensures that normalization produces a canonical form.
pub(super) fn is_norm_lt(l1: &Level, l2: &Level) -> bool {
    let (b1, k1) = to_offset(l1);
    let (b2, k2) = to_offset(l2);
    fn kind_ord(l: &Level) -> u8 {
        match l {
            Level::Zero => 0,
            Level::Param(_) => 1,
            Level::MVar(_) => 2,
            Level::Max(_, _) => 3,
            Level::IMax(_, _) => 4,
            Level::Succ(_) => 5,
        }
    }
    let k1_ord = kind_ord(b1);
    let k2_ord = kind_ord(b2);
    if k1_ord != k2_ord {
        return k1_ord < k2_ord;
    }
    match (b1, b2) {
        (Level::Param(n1), Level::Param(n2)) => {
            let s1 = n1.to_string();
            let s2 = n2.to_string();
            if s1 != s2 {
                return s1 < s2;
            }
        }
        (Level::MVar(m1), Level::MVar(m2)) if m1.0 != m2.0 => {
            return m1.0 < m2.0;
        }
        (Level::MVar(_), Level::MVar(_)) => {}
        (Level::Max(a1, a2), Level::Max(b1_inner, b2_inner))
        | (Level::IMax(a1, a2), Level::IMax(b1_inner, b2_inner)) => {
            if a1 != b1_inner {
                return is_norm_lt(a1, b1_inner);
            }
            if a2 != b2_inner {
                return is_norm_lt(a2, b2_inner);
            }
        }
        _ => {}
    }
    k1 < k2
}
/// Normalize a universe level to canonical form.
///
/// Follows LEAN 4's normalization algorithm:
/// 1. Flatten nested max expressions
/// 2. Normalize each argument recursively
/// 3. Sort arguments
/// 4. Merge duplicates (keep larger offsets)
/// 5. Remove subsumed explicit levels
pub fn normalize(l: &Level) -> Level {
    let (base, k) = to_offset(l);
    match base {
        Level::Zero | Level::Param(_) | Level::MVar(_) => l.clone(),
        Level::Succ(_) => l.clone(),
        Level::IMax(l1, l2) => {
            let l1_norm = normalize(l1);
            let l2_norm = normalize(l2);
            if l2_norm.is_zero() {
                // imax(_, 0) = 0; with offset: Succ^k(0)
                return from_offset(Level::Zero, k);
            }
            if l2_norm.is_not_zero() {
                // imax(u, v) = max(u, v) when v != 0.
                // Distribute the outer offset k into each Max argument so the
                // result is in the same canonical form as the Max branch
                // (offsets are carried inside each component, not outside).
                return normalize(&Level::max(
                    from_offset(l1_norm, k),
                    from_offset(l2_norm, k),
                ));
            }
            if l1_norm.is_zero() {
                // imax(0, v) = v; with offset: Succ^k(v).
                // If v is a Max, from_offset produces Succ^k(Max(a,b)) which
                // is not the same canonical form as Max(Succ^k(a), Succ^k(b)).
                // Re-normalize to distribute the offset inside any Max.
                let with_offset = from_offset(l2_norm, k);
                return if k == 0 {
                    with_offset
                } else {
                    normalize(&with_offset)
                };
            }
            from_offset(Level::imax(l1_norm, l2_norm), k)
        }
        Level::Max(_, _) => {
            let mut args = Vec::new();
            push_max_args(base, &mut args);
            // Normalize each argument.  A component such as Succ(Max(…)) may
            // normalize to a fresh Max node; those must be re-flattened so the
            // resulting list has no nested Max nodes (idempotency requirement).
            let mut normed: Vec<Level> = Vec::new();
            for a in args {
                let normalized_a = normalize(&from_offset(a, k));
                // Re-flatten in case normalization produced a new Max.
                push_max_args(&normalized_a, &mut normed);
            }
            normed.sort_by(|a, b| {
                if is_norm_lt(a, b) {
                    std::cmp::Ordering::Less
                } else if a == b {
                    std::cmp::Ordering::Equal
                } else {
                    std::cmp::Ordering::Greater
                }
            });
            let mut merged: Vec<Level> = Vec::new();
            for arg in normed {
                let dominated = if let Some(last) = merged.last() {
                    let (base_last, k_last) = to_offset(last);
                    let (base_arg, k_arg) = to_offset(&arg);
                    if base_last == base_arg {
                        if k_arg > k_last {
                            merged.pop();
                            false
                        } else {
                            true
                        }
                    } else {
                        false
                    }
                } else {
                    false
                };
                if !dominated {
                    merged.push(arg);
                }
            }
            if merged.len() > 1 {
                let has_nonzero = merged.iter().any(|a| a.is_not_zero());
                if has_nonzero {
                    merged.retain(|a| !a.is_zero());
                }
            }
            if merged.is_empty() {
                Level::Zero
            } else if merged.len() == 1 {
                merged
                    .into_iter()
                    .next()
                    .expect("merged set must be non-empty")
            } else {
                let mut result = merged.pop().expect("merged set must be non-empty");
                while let Some(arg) = merged.pop() {
                    result = Level::max(arg, result);
                }
                result
            }
        }
    }
}
/// Check if two levels are semantically equivalent.
///
/// First tries structural equality, then falls back to normalization.
pub fn is_equivalent(l1: &Level, l2: &Level) -> bool {
    l1 == l2 || normalize(l1) == normalize(l2)
}
/// Check if `l1 >= l2` (level ordering).
///
/// Returns true if level `l1` is guaranteed to be >= `l2`
/// for all possible assignments to parameters and metavariables.
pub fn is_geq(l1: &Level, l2: &Level) -> bool {
    is_geq_core(&normalize(l1), &normalize(l2))
}
pub(super) fn is_geq_core(l1: &Level, l2: &Level) -> bool {
    if l1 == l2 {
        return true;
    }
    if l2.is_zero() {
        return true;
    }
    if let Level::Max(a, b) = l1 {
        if is_geq_core(a, l2) || is_geq_core(b, l2) {
            return true;
        }
    }
    if let Level::Max(b, c) = l2 {
        if is_geq_core(l1, b) && is_geq_core(l1, c) {
            return true;
        }
    }
    if let Level::IMax(b, c) = l2 {
        if is_geq_core(l1, b) && is_geq_core(l1, c) {
            return true;
        }
    }
    if let Level::IMax(_, b) = l1 {
        if is_geq_core(b, l2) {
            return true;
        }
    }
    let (b1, k1) = to_offset(l1);
    let (b2, k2) = to_offset(l2);
    if b1 == b2 {
        return k1 >= k2;
    }
    false
}
/// Check if `l1 <= l2` (level ordering).
pub fn is_leq(l1: &Level, l2: &Level) -> bool {
    is_geq(l2, l1)
}
/// Instantiate level parameters using a substitution.
///
/// Replaces `Param(name)` with the corresponding level if `name` is found
/// in `param_names`.
pub fn instantiate_level(level: &Level, param_names: &[Name], levels: &[Level]) -> Level {
    if param_names.is_empty() {
        return level.clone();
    }
    match level {
        Level::Param(name) => {
            for (i, pn) in param_names.iter().enumerate() {
                if pn == name {
                    if let Some(l) = levels.get(i) {
                        return l.clone();
                    }
                }
            }
            level.clone()
        }
        Level::Succ(l) => Level::succ(instantiate_level(l, param_names, levels)),
        Level::Max(l1, l2) => Level::max(
            instantiate_level(l1, param_names, levels),
            instantiate_level(l2, param_names, levels),
        ),
        Level::IMax(l1, l2) => Level::imax(
            instantiate_level(l1, param_names, levels),
            instantiate_level(l2, param_names, levels),
        ),
        Level::Zero | Level::MVar(_) => level.clone(),
    }
}
/// Collect all parameter names used in a level.
pub fn collect_level_params(l: &Level, params: &mut Vec<Name>) {
    match l {
        Level::Param(name) => {
            if !params.contains(name) {
                params.push(name.clone());
            }
        }
        Level::Succ(l) => collect_level_params(l, params),
        Level::Max(l1, l2) | Level::IMax(l1, l2) => {
            collect_level_params(l1, params);
            collect_level_params(l2, params);
        }
        Level::Zero | Level::MVar(_) => {}
    }
}
/// Collect all metavariable IDs used in a level.
pub fn collect_level_mvars(l: &Level, mvars: &mut Vec<LevelMVarId>) {
    match l {
        Level::MVar(id) => {
            if !mvars.contains(id) {
                mvars.push(*id);
            }
        }
        Level::Succ(l) => collect_level_mvars(l, mvars),
        Level::Max(l1, l2) | Level::IMax(l1, l2) => {
            collect_level_mvars(l1, mvars);
            collect_level_mvars(l2, mvars);
        }
        Level::Zero | Level::Param(_) => {}
    }
}
/// Replace level metavariables using a substitution function.
pub fn instantiate_level_mvars(
    level: &Level,
    subst: &dyn Fn(LevelMVarId) -> Option<Level>,
) -> Level {
    match level {
        Level::MVar(id) => {
            if let Some(l) = subst(*id) {
                instantiate_level_mvars(&l, subst)
            } else {
                level.clone()
            }
        }
        Level::Succ(l) => Level::succ(instantiate_level_mvars(l, subst)),
        Level::Max(l1, l2) => Level::max(
            instantiate_level_mvars(l1, subst),
            instantiate_level_mvars(l2, subst),
        ),
        Level::IMax(l1, l2) => Level::imax(
            instantiate_level_mvars(l1, subst),
            instantiate_level_mvars(l2, subst),
        ),
        Level::Zero | Level::Param(_) => level.clone(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_level_construction() {
        let l0 = Level::zero();
        let l1 = Level::succ(Level::zero());
        let l_param = Level::param(Name::str("u"));
        let l_max = Level::max(l1.clone(), l_param.clone());
        assert!(l0.is_zero());
        assert!(l_param.is_param());
        assert!(!l_max.is_zero());
    }
    #[test]
    fn test_level_display() {
        let l0 = Level::zero();
        let l1 = Level::succ(Level::zero());
        let l_max = Level::max(l0.clone(), l1.clone());
        let l_imax = Level::imax(l0, l1);
        assert_eq!(l_max.to_string(), "max(0, 1)");
        assert_eq!(l_imax.to_string(), "imax(0, 1)");
    }
    #[test]
    fn test_level_mvar() {
        let mv = Level::mvar(LevelMVarId(42));
        assert!(mv.is_mvar());
        assert!(mv.has_mvar());
        assert!(!mv.has_param());
        assert_eq!(mv.to_string(), "?u_42");
    }
    #[test]
    fn test_is_not_zero() {
        assert!(!Level::zero().is_not_zero());
        assert!(Level::succ(Level::zero()).is_not_zero());
        assert!(Level::succ(Level::param(Name::str("u"))).is_not_zero());
        assert!(!Level::param(Name::str("u")).is_not_zero());
        let m = Level::max(Level::zero(), Level::succ(Level::param(Name::str("u"))));
        assert!(m.is_not_zero());
    }
    #[test]
    fn test_to_offset() {
        let l = Level::succ(Level::succ(Level::param(Name::str("u"))));
        let (base, k) = to_offset(&l);
        assert_eq!(*base, Level::param(Name::str("u")));
        assert_eq!(k, 2);
        let zero = Level::zero();
        let (base0, k0) = to_offset(&zero);
        assert_eq!(*base0, Level::zero());
        assert_eq!(k0, 0);
    }
    #[test]
    fn test_from_nat_to_nat() {
        for n in 0..5 {
            let l = Level::from_nat(n);
            assert_eq!(l.to_nat(), Some(n));
        }
        assert_eq!(Level::param(Name::str("u")).to_nat(), None);
    }
    #[test]
    fn test_normalize_zero() {
        assert_eq!(normalize(&Level::zero()), Level::zero());
    }
    #[test]
    fn test_normalize_succ() {
        let l = Level::succ(Level::succ(Level::zero()));
        assert_eq!(normalize(&l), l);
    }
    #[test]
    fn test_normalize_max_same() {
        let u = Level::param(Name::str("u"));
        let m = Level::max(u.clone(), u.clone());
        assert_eq!(normalize(&m), u);
    }
    #[test]
    fn test_normalize_max_zero() {
        let u = Level::param(Name::str("u"));
        let m = Level::max(u.clone(), Level::zero());
        let n = normalize(&m);
        assert!(is_equivalent(&n, &u) || is_equivalent(&n, &m));
    }
    #[test]
    fn test_normalize_max_ordering() {
        let u = Level::param(Name::str("u"));
        let v = Level::param(Name::str("v"));
        let m1 = Level::max(u.clone(), v.clone());
        let m2 = Level::max(v, u);
        assert_eq!(normalize(&m1), normalize(&m2));
    }
    #[test]
    fn test_normalize_imax_zero() {
        let u = Level::param(Name::str("u"));
        let im = Level::imax(u, Level::zero());
        assert_eq!(normalize(&im), Level::zero());
    }
    #[test]
    fn test_normalize_imax_succ() {
        let u = Level::param(Name::str("u"));
        let v = Level::param(Name::str("v"));
        let im = Level::imax(u.clone(), Level::succ(v.clone()));
        let expected = normalize(&Level::max(u, Level::succ(v)));
        assert_eq!(normalize(&im), expected);
    }
    #[test]
    fn test_is_equivalent() {
        let u = Level::param(Name::str("u"));
        let v = Level::param(Name::str("v"));
        assert!(is_equivalent(
            &Level::max(u.clone(), v.clone()),
            &Level::max(v, u),
        ));
        assert!(!is_equivalent(
            &Level::param(Name::str("u")),
            &Level::param(Name::str("v")),
        ));
        assert!(is_equivalent(
            &Level::succ(Level::zero()),
            &Level::succ(Level::zero()),
        ));
    }
    #[test]
    fn test_is_geq() {
        let u = Level::param(Name::str("u"));
        let l0 = Level::zero();
        let l1 = Level::succ(Level::zero());
        let l2 = Level::succ(Level::succ(Level::zero()));
        assert!(is_geq(&u, &l0));
        assert!(is_geq(&l2, &l1));
        assert!(!is_geq(&l1, &l2));
        assert!(is_geq(&Level::succ(u.clone()), &u));
        let v = Level::param(Name::str("v"));
        assert!(is_geq(&Level::max(u.clone(), v.clone()), &u));
        assert!(is_geq(&Level::max(u.clone(), v.clone()), &v));
    }
    #[test]
    fn test_instantiate_level() {
        let l = Level::max(
            Level::param(Name::str("u")),
            Level::succ(Level::param(Name::str("v"))),
        );
        let result = instantiate_level(
            &l,
            &[Name::str("u"), Name::str("v")],
            &[Level::zero(), Level::succ(Level::zero())],
        );
        let expected = Level::max(Level::zero(), Level::succ(Level::succ(Level::zero())));
        assert_eq!(result, expected);
    }
    #[test]
    fn test_collect_level_params() {
        let l = Level::max(
            Level::param(Name::str("u")),
            Level::succ(Level::param(Name::str("v"))),
        );
        let mut params = Vec::new();
        collect_level_params(&l, &mut params);
        assert_eq!(params.len(), 2);
        assert!(params.contains(&Name::str("u")));
        assert!(params.contains(&Name::str("v")));
    }
    #[test]
    fn test_instantiate_level_mvars() {
        let l = Level::max(
            Level::mvar(LevelMVarId(0)),
            Level::succ(Level::mvar(LevelMVarId(1))),
        );
        let result = instantiate_level_mvars(&l, &|id| {
            if id.0 == 0 {
                Some(Level::zero())
            } else if id.0 == 1 {
                Some(Level::param(Name::str("u")))
            } else {
                None
            }
        });
        let expected = Level::max(Level::zero(), Level::succ(Level::param(Name::str("u"))));
        assert_eq!(result, expected);
    }
    #[test]
    fn test_depth() {
        assert_eq!(Level::zero().depth(), 0);
        assert_eq!(Level::succ(Level::zero()).depth(), 1);
        assert_eq!(
            Level::max(Level::param(Name::str("u")), Level::succ(Level::zero())).depth(),
            2
        );
    }
}
/// Compute the minimum of two levels: min(l1, l2).
///
/// Defined as `imax(l1, succ(l2))` is NOT min; instead use:
/// min(l1, l2) = l1 if l1 <= l2, else l2. This is approximate
/// (returns a conservative lower bound for polymorphic levels).
#[allow(dead_code)]
pub fn level_min(l1: &Level, l2: &Level) -> Level {
    if l1.is_zero() || l2.is_zero() {
        return Level::Zero;
    }
    if is_leq(l1, l2) {
        return l1.clone();
    }
    if is_leq(l2, l1) {
        return l2.clone();
    }
    Level::Zero
}
/// Create `max(max(l1, l2), l3)` and normalize.
#[allow(dead_code)]
pub fn level_max3(l1: Level, l2: Level, l3: Level) -> Level {
    normalize(&Level::max(Level::max(l1, l2), l3))
}
/// Bump a level by `n` successor applications.
///
/// `level_add(l, n)` = `succ^n(l)`.
#[allow(dead_code)]
pub fn level_add(l: Level, n: u32) -> Level {
    let mut result = l;
    for _ in 0..n {
        result = Level::succ(result);
    }
    result
}
/// The universe level for `Type n` (= succ^(n+1)(0)).
#[allow(dead_code)]
pub fn type_level(n: u32) -> Level {
    Level::from_nat(n + 1)
}
/// Check if `l` is definitely a successor (non-zero) level.
/// Returns true for `succ(anything)`.
#[allow(dead_code)]
pub fn is_definitely_succ(l: &Level) -> bool {
    matches!(l, Level::Succ(_))
}
/// Collect all levels appearing in a `Vec<Level>`, deduplicating by structural equality.
#[allow(dead_code)]
pub fn dedup_levels(levels: Vec<Level>) -> Vec<Level> {
    let mut seen: Vec<Level> = Vec::new();
    for l in levels {
        if !seen.contains(&l) {
            seen.push(l);
        }
    }
    seen
}
#[cfg(test)]
mod extra_level_tests {
    use super::*;
    #[test]
    fn test_level_min_zeros() {
        assert_eq!(
            level_min(&Level::zero(), &Level::succ(Level::zero())),
            Level::zero()
        );
    }
    #[test]
    fn test_level_min_equal() {
        let l1 = Level::succ(Level::zero());
        let l2 = Level::succ(Level::zero());
        assert_eq!(level_min(&l1, &l2), l1);
    }
    #[test]
    fn test_level_max3() {
        let l0 = Level::zero();
        let l1 = Level::succ(Level::zero());
        let l2 = Level::succ(Level::succ(Level::zero()));
        let result = level_max3(l0, l1, l2.clone());
        assert!(is_equivalent(&result, &l2));
    }
    #[test]
    fn test_level_add_zero_times() {
        let l = Level::param(Name::str("u"));
        assert_eq!(level_add(l.clone(), 0), l);
    }
    #[test]
    fn test_level_add_two_times() {
        let l = Level::zero();
        let result = level_add(l, 2);
        assert_eq!(result.to_nat(), Some(2));
    }
    #[test]
    fn test_type_level_zero() {
        assert_eq!(type_level(0), Level::succ(Level::zero()));
    }
    #[test]
    fn test_type_level_two() {
        assert_eq!(type_level(2).to_nat(), Some(3));
    }
    #[test]
    fn test_is_definitely_succ_true() {
        assert!(is_definitely_succ(&Level::succ(Level::zero())));
    }
    #[test]
    fn test_is_definitely_succ_false() {
        assert!(!is_definitely_succ(&Level::zero()));
        assert!(!is_definitely_succ(&Level::param(Name::str("u"))));
    }
    #[test]
    fn test_dedup_levels() {
        let l0 = Level::zero();
        let l1 = Level::succ(Level::zero());
        let levels = vec![l0.clone(), l0.clone(), l1.clone(), l1.clone()];
        let deduped = dedup_levels(levels);
        assert_eq!(deduped.len(), 2);
    }
    #[test]
    fn test_collect_level_mvars_multiple() {
        let l = Level::max(Level::mvar(LevelMVarId(0)), Level::mvar(LevelMVarId(1)));
        let mut mvars = Vec::new();
        collect_level_mvars(&l, &mut mvars);
        assert_eq!(mvars.len(), 2);
    }
    #[test]
    fn test_level_is_leq() {
        let l0 = Level::zero();
        let l1 = Level::succ(Level::zero());
        assert!(is_leq(&l0, &l1));
        assert!(!is_leq(&l1, &l0));
        assert!(is_leq(&l0, &l0));
    }
}
/// Flatten a level to a list of `(base, offset)` pairs representing max arguments.
///
/// E.g., `max(max(u, v+1), w+2)` → `[(u,0), (v,1), (w,2)]`.
#[allow(dead_code)]
pub fn flatten_max(l: &Level) -> Vec<(Level, u32)> {
    let norm = normalize(l);
    let mut result = Vec::new();
    push_flat(&norm, &mut result);
    result
}
pub(super) fn push_flat(l: &Level, acc: &mut Vec<(Level, u32)>) {
    match l {
        Level::Max(l1, l2) => {
            push_flat(l1, acc);
            push_flat(l2, acc);
        }
        _ => {
            let (base, k) = to_offset(l);
            acc.push((base.clone(), k));
        }
    }
}
/// Check if the level is a concrete numeral (succ^n(zero)).
#[allow(dead_code)]
pub fn is_numeral(l: &Level) -> bool {
    l.to_nat().is_some()
}
/// Check if a level expression is ground (no parameters, no mvars).
#[allow(dead_code)]
pub fn is_ground(l: &Level) -> bool {
    !l.has_param() && !l.has_mvar()
}
/// Evaluate a ground (no params, no mvars) level to a concrete `u32`.
///
/// Returns `None` if the level has parameters or metavariables.
#[allow(dead_code)]
pub fn eval_ground_level(l: &Level) -> Option<u32> {
    if !is_ground(l) {
        return None;
    }
    let norm = normalize(l);
    norm.to_nat()
}
/// Create a level that is the `max` of a slice of levels.
///
/// Returns `Level::Zero` for an empty slice.
#[allow(dead_code)]
pub fn max_of_slice(levels: &[Level]) -> Level {
    let mut iter = levels.iter();
    match iter.next() {
        None => Level::Zero,
        Some(first) => iter.fold(first.clone(), |acc, l| Level::max(acc, l.clone())),
    }
}
/// Create a level that is the `imax` of a slice of levels, left-folded.
///
/// Useful for computing the sort level of Pi types.
#[allow(dead_code)]
pub fn imax_fold(levels: &[Level]) -> Level {
    let mut iter = levels.iter();
    match iter.next() {
        None => Level::Zero,
        Some(first) => iter.fold(first.clone(), |acc, l| Level::imax(acc, l.clone())),
    }
}
/// Level of `Prop` (= 0).
#[allow(dead_code)]
pub const PROP_LEVEL: Level = Level::Zero;
/// Level of `Type 0` (= 1).
#[allow(dead_code)]
pub fn type0_level() -> Level {
    Level::succ(Level::Zero)
}
/// Level of `Type 1` (= 2).
#[allow(dead_code)]
pub fn type1_level() -> Level {
    Level::succ(Level::succ(Level::Zero))
}
#[cfg(test)]
mod extra2_level_tests {
    use super::*;
    #[test]
    fn test_level_constraint_leq_satisfied() {
        let l0 = Level::zero();
        let l1 = Level::succ(Level::zero());
        let c = LevelConstraint::Leq(l0, l1);
        assert!(c.is_satisfied());
    }
    #[test]
    fn test_level_constraint_leq_violated() {
        let l1 = Level::succ(Level::zero());
        let l0 = Level::zero();
        let c = LevelConstraint::Leq(l1, l0);
        assert!(!c.is_satisfied());
    }
    #[test]
    fn test_level_constraint_eq_satisfied() {
        let l = Level::succ(Level::zero());
        let c = LevelConstraint::Eq(l.clone(), l);
        assert!(c.is_satisfied());
    }
    #[test]
    fn test_constraint_set_all_satisfied() {
        let mut cs = ConstraintSet::new();
        cs.add_leq(Level::zero(), Level::succ(Level::zero()));
        cs.add_eq(Level::zero(), Level::zero());
        assert!(cs.all_satisfied());
    }
    #[test]
    fn test_constraint_set_unsatisfied() {
        let mut cs = ConstraintSet::new();
        cs.add_leq(Level::succ(Level::zero()), Level::zero());
        assert!(!cs.all_satisfied());
        assert_eq!(cs.unsatisfied().len(), 1);
    }
    #[test]
    fn test_flatten_max_single() {
        let l = Level::param(Name::str("u"));
        let flat = flatten_max(&l);
        assert_eq!(flat.len(), 1);
    }
    #[test]
    fn test_flatten_max_binary() {
        let u = Level::param(Name::str("u"));
        let v = Level::param(Name::str("v"));
        let m = Level::max(u, v);
        let flat = flatten_max(&m);
        assert_eq!(flat.len(), 2);
    }
    #[test]
    fn test_is_numeral_true() {
        assert!(is_numeral(&Level::zero()));
        assert!(is_numeral(&Level::succ(Level::zero())));
        assert!(is_numeral(&Level::from_nat(5)));
    }
    #[test]
    fn test_is_numeral_false() {
        assert!(!is_numeral(&Level::param(Name::str("u"))));
    }
    #[test]
    fn test_is_ground_zero() {
        assert!(is_ground(&Level::zero()));
    }
    #[test]
    fn test_is_ground_param() {
        assert!(!is_ground(&Level::param(Name::str("u"))));
    }
    #[test]
    fn test_eval_ground_level_numeral() {
        let l = Level::from_nat(3);
        assert_eq!(eval_ground_level(&l), Some(3));
    }
    #[test]
    fn test_eval_ground_level_param() {
        let l = Level::param(Name::str("u"));
        assert_eq!(eval_ground_level(&l), None);
    }
    #[test]
    fn test_max_of_slice_empty() {
        let result = max_of_slice(&[]);
        assert_eq!(result, Level::zero());
    }
    #[test]
    fn test_max_of_slice_single() {
        let l = Level::succ(Level::zero());
        let result = max_of_slice(std::slice::from_ref(&l));
        assert_eq!(result, l);
    }
    #[test]
    fn test_max_of_slice_multiple() {
        let l0 = Level::zero();
        let l1 = Level::succ(Level::zero());
        let l2 = Level::succ(Level::succ(Level::zero()));
        let result = max_of_slice(&[l0, l1, l2.clone()]);
        assert!(is_equivalent(&normalize(&result), &l2));
    }
    #[test]
    fn test_imax_fold_empty() {
        assert_eq!(imax_fold(&[]), Level::zero());
    }
    #[test]
    fn test_type0_level() {
        assert_eq!(type0_level().to_nat(), Some(1));
    }
    #[test]
    fn test_type1_level() {
        assert_eq!(type1_level().to_nat(), Some(2));
    }
    #[test]
    fn test_constraint_set_len() {
        let mut cs = ConstraintSet::new();
        cs.add_leq(Level::zero(), Level::succ(Level::zero()));
        assert_eq!(cs.len(), 1);
        assert!(!cs.is_empty());
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
#[cfg(test)]
mod tests_final_padding {
    use super::*;
    #[test]
    fn test_min_heap() {
        let mut h = MinHeap::new();
        h.push(5u32);
        h.push(1u32);
        h.push(3u32);
        assert_eq!(h.peek(), Some(&1));
        assert_eq!(h.pop(), Some(1));
        assert_eq!(h.pop(), Some(3));
        assert_eq!(h.pop(), Some(5));
        assert!(h.is_empty());
    }
    #[test]
    fn test_prefix_counter() {
        let mut pc = PrefixCounter::new();
        pc.record("hello");
        pc.record("help");
        pc.record("world");
        assert_eq!(pc.count_with_prefix("hel"), 2);
        assert_eq!(pc.count_with_prefix("wor"), 1);
        assert_eq!(pc.count_with_prefix("xyz"), 0);
    }
    #[test]
    fn test_fixture() {
        let mut f = Fixture::new();
        f.set("key1", "val1");
        f.set("key2", "val2");
        assert_eq!(f.get("key1"), Some("val1"));
        assert_eq!(f.get("key3"), None);
        assert_eq!(f.len(), 2);
    }
}
