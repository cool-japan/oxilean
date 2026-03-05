//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    ArmRedundancy, CompletenessReport, ConstructorSpec, CoverageMap, ExhaustivenessResult,
    InductiveSignature, MatchExhaustAnalysisPass, MatchExhaustBuilder, MatchExhaustConfig,
    MatchExhaustConfigValue, MatchExhaustCounterMap, MatchExhaustDiagnostics, MatchExhaustDiff,
    MatchExhaustExtConfig2000, MatchExhaustExtConfigVal2000, MatchExhaustExtDiag2000,
    MatchExhaustExtDiff2000, MatchExhaustExtMap, MatchExhaustExtPass2000,
    MatchExhaustExtPipeline2000, MatchExhaustExtResult2000, MatchExhaustExtUtil,
    MatchExhaustPipeline, MatchExhaustResult, MatchExhaustStateMachine, MatchExhaustWindow,
    MatchExhaustWorkQueue, MissingPattern, PatternMatrix, RedundancyReason, RedundantArm,
};
use crate::match_basic::MetaPattern;
use oxilean_kernel::{Literal, Name};

/// Check exhaustiveness of a pattern matrix.
///
/// Given a list of constructor specifications for the type being matched,
/// and the first-column patterns from each arm, determines if all
/// constructors are covered.
pub fn check_exhaustive(
    ctors: &[ConstructorSpec],
    patterns: &[Vec<MetaPattern>],
    col: usize,
) -> ExhaustivenessResult {
    if patterns.is_empty() {
        if ctors.is_empty() {
            return ExhaustivenessResult {
                is_exhaustive: true,
                missing: Vec::new(),
                unreachable_arms: Vec::new(),
            };
        }
        let missing: Vec<MissingPattern> = ctors
            .iter()
            .map(|c| {
                MissingPattern::simple(
                    c.name.clone(),
                    format!("Missing case for constructor {}", c.name),
                )
            })
            .collect();
        return ExhaustivenessResult {
            is_exhaustive: false,
            missing,
            unreachable_arms: Vec::new(),
        };
    }
    let has_catchall = patterns
        .iter()
        .any(|arm| arm.get(col).map(|p| p.is_irrefutable()).unwrap_or(false));
    if has_catchall {
        let unreachable = find_unreachable_arms(patterns, col);
        return ExhaustivenessResult {
            is_exhaustive: true,
            missing: Vec::new(),
            unreachable_arms: unreachable,
        };
    }
    let covered: Vec<Name> = patterns
        .iter()
        .filter_map(|arm| arm.get(col).and_then(|p| p.ctor_name().cloned()))
        .collect();
    let missing: Vec<MissingPattern> = ctors
        .iter()
        .filter(|c| !covered.contains(&c.name))
        .map(|c| {
            MissingPattern::simple(
                c.name.clone(),
                format!("Missing case for constructor {}", c.name),
            )
        })
        .collect();
    let unreachable = find_unreachable_arms(patterns, col);
    ExhaustivenessResult {
        is_exhaustive: missing.is_empty(),
        missing,
        unreachable_arms: unreachable,
    }
}
/// Find unreachable arms in a pattern matrix.
///
/// An arm is unreachable if all its patterns are subsumed
/// by earlier arms.
pub(super) fn find_unreachable_arms(patterns: &[Vec<MetaPattern>], col: usize) -> Vec<usize> {
    let mut unreachable = Vec::new();
    let mut seen_catchall = false;
    for (i, arm) in patterns.iter().enumerate() {
        if seen_catchall {
            unreachable.push(i);
            continue;
        }
        if let Some(pat) = arm.get(col) {
            if pat.is_irrefutable() {
                seen_catchall = true;
            }
        }
    }
    unreachable
}
/// Check if a set of literal patterns is exhaustive for a given type.
///
/// For types like Bool, this can be determined.
/// For Nat/String, literal patterns are never exhaustive without a wildcard.
pub fn check_literal_exhaustive(type_name: &Name, literals: &[Literal]) -> bool {
    let name_str = format!("{}", type_name);
    match name_str.as_str() {
        "Bool" => {
            let has_true = literals.iter().any(|l| matches!(l, Literal::Nat(1)));
            let has_false = literals.iter().any(|l| matches!(l, Literal::Nat(0)));
            has_true && has_false
        }
        _ => false,
    }
}
/// Check if a pattern subsumes another pattern.
///
/// Pattern `p1` subsumes `p2` if every value matching `p2` also matches `p1`.
pub fn pattern_subsumes(p1: &MetaPattern, p2: &MetaPattern) -> bool {
    match (p1, p2) {
        (MetaPattern::Wildcard, _) | (MetaPattern::Var(_), _) => true,
        (MetaPattern::Constructor(n1, pats1), MetaPattern::Constructor(n2, pats2)) => {
            n1 == n2
                && pats1.len() == pats2.len()
                && pats1
                    .iter()
                    .zip(pats2.iter())
                    .all(|(a, b)| pattern_subsumes(a, b))
        }
        (MetaPattern::Literal(l1), MetaPattern::Literal(l2)) => l1 == l2,
        (MetaPattern::As(inner, _), other) => pattern_subsumes(inner, other),
        (other, MetaPattern::As(inner, _)) => pattern_subsumes(other, inner),
        (p, MetaPattern::Or(q1, q2)) => pattern_subsumes(p, q1) && pattern_subsumes(p, q2),
        (MetaPattern::Or(p_left, p_right), q) => {
            pattern_subsumes(p_left, q) || pattern_subsumes(p_right, q)
        }
        _ => false,
    }
}
/// Analyze all arms for redundancy.
///
/// Returns redundancy information for each arm in order.
pub fn analyze_redundancy(patterns: &[Vec<MetaPattern>], col: usize) -> Vec<ArmRedundancy> {
    let mut result = Vec::with_capacity(patterns.len());
    let mut active: Vec<&Vec<MetaPattern>> = Vec::new();
    for (i, arm) in patterns.iter().enumerate() {
        let mut is_redundant = false;
        let mut subsumed_by = None;
        if let Some(pat) = arm.get(col) {
            for (j, earlier) in active.iter().enumerate() {
                if let Some(earlier_pat) = earlier.get(col) {
                    if pattern_subsumes(earlier_pat, pat) {
                        is_redundant = true;
                        subsumed_by = Some(j);
                        break;
                    }
                }
            }
        }
        result.push(ArmRedundancy {
            arm_index: i,
            is_redundant,
            subsumed_by,
        });
        if !is_redundant {
            active.push(arm);
        }
    }
    result
}
/// Compute coverage for a set of patterns.
///
/// Returns a `CoverageMap` indicating which constructors are covered and
/// which are missing.
pub fn compute_coverage(
    ctors: &[ConstructorSpec],
    patterns: &[Vec<MetaPattern>],
    col: usize,
) -> CoverageMap {
    let mut map = CoverageMap::from_constructors(ctors);
    for (i, arm) in patterns.iter().enumerate() {
        if let Some(pat) = arm.get(col) {
            match pat {
                MetaPattern::Wildcard | MetaPattern::Var(_) => {
                    for ctor in ctors {
                        map.mark_covered_by_catchall(&ctor.name);
                    }
                }
                MetaPattern::Constructor(name, _) => {
                    map.mark_covered(name, i);
                }
                MetaPattern::Or(p1, p2) => {
                    if let Some(n1) = p1.ctor_name() {
                        map.mark_covered(n1, i);
                    }
                    if let Some(n2) = p2.ctor_name() {
                        map.mark_covered(n2, i);
                    }
                }
                MetaPattern::As(inner, _) => {
                    if let Some(n) = inner.ctor_name() {
                        map.mark_covered(n, i);
                    }
                }
                _ => {}
            }
        }
    }
    map
}
/// Check that a pattern list is well-founded (no inaccessible patterns in
/// positions that have not been established by unification).
///
/// This is a basic check: inaccessible patterns (.e) are only valid
/// when the kernel has confirmed the value equals `e`.
pub fn check_no_spurious_inaccessible(patterns: &[Vec<MetaPattern>]) -> Vec<(usize, usize)> {
    let mut bad = Vec::new();
    for (i, arm) in patterns.iter().enumerate() {
        for (j, pat) in arm.iter().enumerate() {
            if matches!(pat, MetaPattern::Inaccessible(_)) {
                bad.push((i, j));
            }
        }
    }
    bad
}
/// Check for duplicate literal patterns.
///
/// Returns a list of (arm_index, literal) pairs for literals that appear
/// more than once.
pub fn find_duplicate_literals(patterns: &[Vec<MetaPattern>], col: usize) -> Vec<(usize, Literal)> {
    let mut seen: Vec<Literal> = Vec::new();
    let mut duplicates = Vec::new();
    for (i, arm) in patterns.iter().enumerate() {
        if let Some(MetaPattern::Literal(lit)) = arm.get(col) {
            if seen.contains(lit) {
                duplicates.push((i, lit.clone()));
            } else {
                seen.push(lit.clone());
            }
        }
    }
    duplicates
}
/// Maximum nesting depth of a pattern.
pub fn pattern_depth(pat: &MetaPattern) -> usize {
    match pat {
        MetaPattern::Wildcard | MetaPattern::Var(_) | MetaPattern::Literal(_) => 1,
        MetaPattern::Inaccessible(_) => 1,
        MetaPattern::Constructor(_, sub_pats) => {
            1 + sub_pats.iter().map(pattern_depth).max().unwrap_or(0)
        }
        MetaPattern::As(inner, _) => 1 + pattern_depth(inner),
        MetaPattern::Or(p1, p2) => pattern_depth(p1).max(pattern_depth(p2)),
    }
}
/// Maximum pattern depth across all arms in the given column.
pub fn max_pattern_depth_in_col(patterns: &[Vec<MetaPattern>], col: usize) -> usize {
    patterns
        .iter()
        .filter_map(|arm| arm.get(col))
        .map(pattern_depth)
        .max()
        .unwrap_or(0)
}
/// Perform a full completeness check on a match expression.
pub fn check_completeness(
    ctors: &[ConstructorSpec],
    patterns: &[Vec<MetaPattern>],
    col: usize,
) -> CompletenessReport {
    let exhaust = check_exhaustive(ctors, patterns, col);
    let redundancy = analyze_redundancy(patterns, col);
    let redundant_arms: Vec<RedundantArm> = redundancy
        .iter()
        .filter(|r| r.is_redundant)
        .map(|r| RedundantArm {
            arm_index: r.arm_index,
            reason: match r.subsumed_by {
                Some(prev) => RedundancyReason::SubsumedBy {
                    subsuming_arm: prev,
                },
                None => RedundancyReason::PreviousCatchall { catchall_arm: 0 },
            },
        })
        .collect();
    let useful_arms = patterns.len() - redundant_arms.len();
    CompletenessReport {
        exhaustive: exhaust.is_exhaustive,
        missing_patterns: exhaust.missing,
        redundant_arms,
        total_arms: patterns.len(),
        useful_arms,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::match_exhaust::*;
    fn mk_ctors() -> Vec<ConstructorSpec> {
        vec![
            ConstructorSpec::new(Name::str("Nat.zero"), 0, Name::str("Nat")),
            ConstructorSpec::new(Name::str("Nat.succ"), 1, Name::str("Nat")),
        ]
    }
    fn mk_bool_ctors() -> Vec<ConstructorSpec> {
        vec![
            ConstructorSpec::new(Name::str("Bool.true"), 0, Name::str("Bool")),
            ConstructorSpec::new(Name::str("Bool.false"), 0, Name::str("Bool")),
        ]
    }
    #[test]
    fn test_exhaustive_with_wildcard() {
        let ctors = mk_ctors();
        let patterns = vec![
            vec![MetaPattern::Constructor(Name::str("Nat.zero"), vec![])],
            vec![MetaPattern::Wildcard],
        ];
        let result = check_exhaustive(&ctors, &patterns, 0);
        assert!(result.is_exhaustive);
        assert!(result.missing.is_empty());
    }
    #[test]
    fn test_exhaustive_all_ctors() {
        let ctors = mk_ctors();
        let patterns = vec![
            vec![MetaPattern::Constructor(Name::str("Nat.zero"), vec![])],
            vec![MetaPattern::Constructor(
                Name::str("Nat.succ"),
                vec![MetaPattern::Var(Name::str("n"))],
            )],
        ];
        let result = check_exhaustive(&ctors, &patterns, 0);
        assert!(result.is_exhaustive);
    }
    #[test]
    fn test_non_exhaustive() {
        let ctors = mk_ctors();
        let patterns = vec![vec![MetaPattern::Constructor(
            Name::str("Nat.zero"),
            vec![],
        )]];
        let result = check_exhaustive(&ctors, &patterns, 0);
        assert!(!result.is_exhaustive);
        assert_eq!(result.missing.len(), 1);
        assert_eq!(result.missing[0].ctor_name, Name::str("Nat.succ"));
    }
    #[test]
    fn test_empty_patterns() {
        let ctors = mk_ctors();
        let patterns: Vec<Vec<MetaPattern>> = vec![];
        let result = check_exhaustive(&ctors, &patterns, 0);
        assert!(!result.is_exhaustive);
        assert_eq!(result.missing.len(), 2);
    }
    #[test]
    fn test_unreachable_arms() {
        let ctors = mk_ctors();
        let patterns = vec![
            vec![MetaPattern::Wildcard],
            vec![MetaPattern::Constructor(Name::str("Nat.zero"), vec![])],
        ];
        let result = check_exhaustive(&ctors, &patterns, 0);
        assert!(result.is_exhaustive);
        assert_eq!(result.unreachable_arms, vec![1]);
    }
    #[test]
    fn test_wildcard_subsumes_all() {
        let w = MetaPattern::Wildcard;
        let v = MetaPattern::Var(Name::str("x"));
        let c = MetaPattern::Constructor(Name::str("Nat.zero"), vec![]);
        let l = MetaPattern::Literal(Literal::Nat(42));
        assert!(pattern_subsumes(&w, &v));
        assert!(pattern_subsumes(&w, &c));
        assert!(pattern_subsumes(&w, &l));
    }
    #[test]
    fn test_ctor_subsumes_same() {
        let c1 = MetaPattern::Constructor(Name::str("Nat.succ"), vec![MetaPattern::Wildcard]);
        let c2 = MetaPattern::Constructor(
            Name::str("Nat.succ"),
            vec![MetaPattern::Var(Name::str("n"))],
        );
        assert!(pattern_subsumes(&c1, &c2));
    }
    #[test]
    fn test_ctor_no_subsume_different() {
        let c1 = MetaPattern::Constructor(Name::str("Nat.zero"), vec![]);
        let c2 = MetaPattern::Constructor(Name::str("Nat.succ"), vec![MetaPattern::Wildcard]);
        assert!(!pattern_subsumes(&c1, &c2));
    }
    #[test]
    fn test_literal_subsumes() {
        let l1 = MetaPattern::Literal(Literal::Nat(42));
        let l2 = MetaPattern::Literal(Literal::Nat(42));
        let l3 = MetaPattern::Literal(Literal::Nat(43));
        assert!(pattern_subsumes(&l1, &l2));
        assert!(!pattern_subsumes(&l1, &l3));
    }
    #[test]
    fn test_check_literal_exhaustive_bool() {
        let lits = vec![Literal::Nat(0), Literal::Nat(1)];
        assert!(check_literal_exhaustive(&Name::str("Bool"), &lits));
    }
    #[test]
    fn test_check_literal_not_exhaustive() {
        let lits = vec![Literal::Nat(0)];
        assert!(!check_literal_exhaustive(&Name::str("Nat"), &lits));
    }
    #[test]
    fn test_pattern_matrix_specialize() {
        let mut mat = PatternMatrix::new(1);
        mat.add_row(vec![MetaPattern::Constructor(
            Name::str("Nat.succ"),
            vec![MetaPattern::Wildcard],
        )]);
        mat.add_row(vec![MetaPattern::Wildcard]);
        let specialized = mat.specialize(0, &Name::str("Nat.succ"), 1);
        assert_eq!(specialized.num_rows(), 2);
        assert_eq!(specialized.num_cols(), 1);
    }
    #[test]
    fn test_pattern_matrix_default_matrix() {
        let mut mat = PatternMatrix::new(1);
        mat.add_row(vec![MetaPattern::Constructor(
            Name::str("Nat.zero"),
            vec![],
        )]);
        mat.add_row(vec![MetaPattern::Wildcard]);
        let def = mat.default_matrix(0);
        assert_eq!(def.num_rows(), 1);
        assert_eq!(def.num_cols(), 0);
    }
    #[test]
    fn test_pattern_matrix_constructors_in_col() {
        let mut mat = PatternMatrix::new(1);
        mat.add_row(vec![MetaPattern::Constructor(
            Name::str("Nat.zero"),
            vec![],
        )]);
        mat.add_row(vec![MetaPattern::Constructor(
            Name::str("Nat.succ"),
            vec![MetaPattern::Wildcard],
        )]);
        mat.add_row(vec![MetaPattern::Constructor(
            Name::str("Nat.zero"),
            vec![],
        )]);
        let ctors = mat.constructors_in_col(0);
        assert_eq!(ctors.len(), 2);
    }
    #[test]
    fn test_coverage_map() {
        let ctors = mk_ctors();
        let mut map = CoverageMap::from_constructors(&ctors);
        assert!(!map.is_complete());
        map.mark_covered(&Name::str("Nat.zero"), 0);
        assert!(!map.is_complete());
        map.mark_covered(&Name::str("Nat.succ"), 1);
        assert!(map.is_complete());
    }
    #[test]
    fn test_compute_coverage_full() {
        let ctors = mk_ctors();
        let patterns = vec![
            vec![MetaPattern::Constructor(Name::str("Nat.zero"), vec![])],
            vec![MetaPattern::Constructor(
                Name::str("Nat.succ"),
                vec![MetaPattern::Wildcard],
            )],
        ];
        let map = compute_coverage(&ctors, &patterns, 0);
        assert!(map.is_complete());
    }
    #[test]
    fn test_compute_coverage_catchall() {
        let ctors = mk_ctors();
        let patterns = vec![vec![MetaPattern::Wildcard]];
        let map = compute_coverage(&ctors, &patterns, 0);
        assert!(map.is_complete());
    }
    #[test]
    fn test_redundancy_analysis() {
        let patterns = vec![
            vec![MetaPattern::Wildcard],
            vec![MetaPattern::Constructor(Name::str("Nat.zero"), vec![])],
        ];
        let info = analyze_redundancy(&patterns, 0);
        assert!(!info[0].is_redundant);
        assert!(info[1].is_redundant);
    }
    #[test]
    fn test_find_duplicate_literals() {
        let patterns = vec![
            vec![MetaPattern::Literal(Literal::Nat(1))],
            vec![MetaPattern::Literal(Literal::Nat(2))],
            vec![MetaPattern::Literal(Literal::Nat(1))],
        ];
        let dups = find_duplicate_literals(&patterns, 0);
        assert_eq!(dups.len(), 1);
        assert_eq!(dups[0].0, 2);
    }
    #[test]
    fn test_pattern_depth() {
        let wild = MetaPattern::Wildcard;
        assert_eq!(pattern_depth(&wild), 1);
        let nested = MetaPattern::Constructor(
            Name::str("Nat.succ"),
            vec![MetaPattern::Constructor(
                Name::str("Nat.succ"),
                vec![MetaPattern::Wildcard],
            )],
        );
        assert_eq!(pattern_depth(&nested), 3);
    }
    #[test]
    fn test_check_no_spurious_inaccessible() {
        use oxilean_kernel::Expr;
        let patterns = vec![
            vec![MetaPattern::Inaccessible(Expr::Lit(Literal::Nat(0)))],
            vec![MetaPattern::Wildcard],
        ];
        let bad = check_no_spurious_inaccessible(&patterns);
        assert_eq!(bad.len(), 1);
        assert_eq!(bad[0], (0, 0));
    }
    #[test]
    fn test_check_completeness_perfect() {
        let ctors = mk_bool_ctors();
        let patterns = vec![
            vec![MetaPattern::Constructor(Name::str("Bool.true"), vec![])],
            vec![MetaPattern::Constructor(Name::str("Bool.false"), vec![])],
        ];
        let report = check_completeness(&ctors, &patterns, 0);
        assert!(report.is_complete());
        assert!(report.redundant_arms.is_empty());
        assert!(report.exhaustive);
    }
    #[test]
    fn test_check_completeness_redundant() {
        let ctors = mk_ctors();
        let patterns = vec![
            vec![MetaPattern::Wildcard],
            vec![MetaPattern::Constructor(Name::str("Nat.zero"), vec![])],
        ];
        let report = check_completeness(&ctors, &patterns, 0);
        assert!(report.exhaustive);
        assert!(!report.is_complete());
        assert_eq!(report.redundant_arms.len(), 1);
    }
    #[test]
    fn test_inductive_signature_nat() {
        let sig = InductiveSignature::mk_nat();
        assert_eq!(sig.num_constructors(), 2);
        assert!(!sig.is_singleton());
        assert!(!sig.is_prop);
    }
    #[test]
    fn test_inductive_signature_bool() {
        let sig = InductiveSignature::mk_bool();
        assert!(sig.is_finite);
        let ctor = sig.get_constructor(&Name::str("Bool.true"));
        assert!(ctor.is_some());
        assert_eq!(ctor.expect("ctor should be valid").num_fields, 0);
    }
    #[test]
    fn test_constructor_spec_is_nullary() {
        let c = ConstructorSpec::new(Name::str("Nat.zero"), 0, Name::str("Nat"));
        assert!(c.is_nullary());
        assert!(!c.is_unary());
    }
    #[test]
    fn test_missing_pattern_full_description() {
        let m = MissingPattern {
            ctor_name: Name::str("Nat.zero"),
            description: "Missing zero case".to_string(),
            sub_missing: vec![MissingPattern::simple(
                Name::str("inner"),
                "inner missing".to_string(),
            )],
        };
        let desc = m.full_description();
        assert!(desc.contains("Missing zero case"));
        assert!(desc.contains("inner missing"));
    }
    #[test]
    fn test_exhaustiveness_result_summary() {
        let r = ExhaustivenessResult {
            is_exhaustive: false,
            missing: vec![MissingPattern::simple(
                Name::str("x"),
                "Missing x".to_string(),
            )],
            unreachable_arms: vec![2],
        };
        let s = r.summary();
        assert!(s.contains("missing"));
        assert!(s.contains("unreachable"));
    }
    #[test]
    fn test_or_pattern_subsumes() {
        let or_pat = MetaPattern::Or(
            Box::new(MetaPattern::Constructor(Name::str("A"), vec![])),
            Box::new(MetaPattern::Constructor(Name::str("B"), vec![])),
        );
        let a = MetaPattern::Constructor(Name::str("A"), vec![]);
        assert!(pattern_subsumes(&or_pat, &a));
    }
}
#[cfg(test)]
mod matchexhaust_ext2_tests {
    use super::*;
    use crate::match_exhaust::*;
    #[test]
    fn test_matchexhaust_ext_util_basic() {
        let mut u = MatchExhaustExtUtil::new("test");
        u.push(10);
        u.push(20);
        assert_eq!(u.sum(), 30);
        assert_eq!(u.len(), 2);
    }
    #[test]
    fn test_matchexhaust_ext_util_min_max() {
        let mut u = MatchExhaustExtUtil::new("mm");
        u.push(5);
        u.push(1);
        u.push(9);
        assert_eq!(u.min_val(), Some(1));
        assert_eq!(u.max_val(), Some(9));
    }
    #[test]
    fn test_matchexhaust_ext_util_flags() {
        let mut u = MatchExhaustExtUtil::new("flags");
        u.set_flag(3);
        assert!(u.has_flag(3));
        assert!(!u.has_flag(2));
    }
    #[test]
    fn test_matchexhaust_ext_util_pop() {
        let mut u = MatchExhaustExtUtil::new("pop");
        u.push(42);
        assert_eq!(u.pop(), Some(42));
        assert!(u.is_empty());
    }
    #[test]
    fn test_matchexhaust_ext_map_basic() {
        let mut m: MatchExhaustExtMap<i32> = MatchExhaustExtMap::new();
        m.insert("key", 42);
        assert_eq!(m.get("key"), Some(&42));
        assert!(m.contains("key"));
        assert!(!m.contains("other"));
    }
    #[test]
    fn test_matchexhaust_ext_map_get_or_default() {
        let mut m: MatchExhaustExtMap<i32> = MatchExhaustExtMap::new();
        m.insert("k", 5);
        assert_eq!(m.get_or_default("k"), 5);
        assert_eq!(m.get_or_default("missing"), 0);
    }
    #[test]
    fn test_matchexhaust_ext_map_keys_sorted() {
        let mut m: MatchExhaustExtMap<i32> = MatchExhaustExtMap::new();
        m.insert("z", 1);
        m.insert("a", 2);
        m.insert("m", 3);
        let keys = m.keys_sorted();
        assert_eq!(keys[0].as_str(), "a");
        assert_eq!(keys[2].as_str(), "z");
    }
    #[test]
    fn test_matchexhaust_window_mean() {
        let mut w = MatchExhaustWindow::new(3);
        w.push(1.0);
        w.push(2.0);
        w.push(3.0);
        assert!((w.mean() - 2.0).abs() < 1e-10);
    }
    #[test]
    fn test_matchexhaust_window_evict() {
        let mut w = MatchExhaustWindow::new(2);
        w.push(10.0);
        w.push(20.0);
        w.push(30.0);
        assert_eq!(w.len(), 2);
        assert!((w.mean() - 25.0).abs() < 1e-10);
    }
    #[test]
    fn test_matchexhaust_window_std_dev() {
        let mut w = MatchExhaustWindow::new(10);
        for i in 0..10 {
            w.push(i as f64);
        }
        assert!(w.std_dev() > 0.0);
    }
    #[test]
    fn test_matchexhaust_builder_basic() {
        let b = MatchExhaustBuilder::new("test")
            .add_item("a")
            .add_item("b")
            .set_config("key", "val");
        assert_eq!(b.item_count(), 2);
        assert!(b.has_config("key"));
        assert_eq!(b.get_config("key"), Some("val"));
    }
    #[test]
    fn test_matchexhaust_builder_summary() {
        let b = MatchExhaustBuilder::new("suite").add_item("x");
        let s = b.build_summary();
        assert!(s.contains("suite"));
    }
    #[test]
    fn test_matchexhaust_state_machine_start() {
        let mut sm = MatchExhaustStateMachine::new();
        assert!(sm.start());
        assert!(sm.state.is_running());
    }
    #[test]
    fn test_matchexhaust_state_machine_complete() {
        let mut sm = MatchExhaustStateMachine::new();
        sm.start();
        sm.complete();
        assert!(sm.state.is_terminal());
    }
    #[test]
    fn test_matchexhaust_state_machine_fail() {
        let mut sm = MatchExhaustStateMachine::new();
        sm.fail("oops");
        assert!(sm.state.is_terminal());
        assert_eq!(sm.state.error_msg(), Some("oops"));
    }
    #[test]
    fn test_matchexhaust_state_machine_no_transition_after_terminal() {
        let mut sm = MatchExhaustStateMachine::new();
        sm.complete();
        assert!(!sm.start());
    }
    #[test]
    fn test_matchexhaust_work_queue_basic() {
        let mut wq = MatchExhaustWorkQueue::new(10);
        wq.enqueue("task1".to_string());
        wq.enqueue("task2".to_string());
        assert_eq!(wq.pending_count(), 2);
        let t = wq.dequeue();
        assert_eq!(t, Some("task1".to_string()));
        assert_eq!(wq.processed_count(), 1);
    }
    #[test]
    fn test_matchexhaust_work_queue_capacity() {
        let mut wq = MatchExhaustWorkQueue::new(2);
        wq.enqueue("a".to_string());
        wq.enqueue("b".to_string());
        assert!(wq.is_full());
        assert!(!wq.enqueue("c".to_string()));
    }
    #[test]
    fn test_matchexhaust_counter_map_basic() {
        let mut cm = MatchExhaustCounterMap::new();
        cm.increment("apple");
        cm.increment("apple");
        cm.increment("banana");
        assert_eq!(cm.count("apple"), 2);
        assert_eq!(cm.count("banana"), 1);
        assert_eq!(cm.num_unique(), 2);
    }
    #[test]
    fn test_matchexhaust_counter_map_frequency() {
        let mut cm = MatchExhaustCounterMap::new();
        cm.increment("a");
        cm.increment("a");
        cm.increment("b");
        assert!((cm.frequency("a") - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_matchexhaust_counter_map_most_common() {
        let mut cm = MatchExhaustCounterMap::new();
        cm.increment("x");
        cm.increment("y");
        cm.increment("x");
        let (k, v) = cm.most_common().expect("most_common should succeed");
        assert_eq!(k.as_str(), "x");
        assert_eq!(v, 2);
    }
}
#[cfg(test)]
mod matchexhaust_analysis_tests {
    use super::*;
    use crate::match_exhaust::*;
    #[test]
    fn test_matchexhaust_result_ok() {
        let r = MatchExhaustResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_matchexhaust_result_err() {
        let r = MatchExhaustResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_matchexhaust_result_partial() {
        let r = MatchExhaustResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_matchexhaust_result_skipped() {
        let r = MatchExhaustResult::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_matchexhaust_analysis_pass_run() {
        let mut p = MatchExhaustAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_matchexhaust_analysis_pass_empty_input() {
        let mut p = MatchExhaustAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_matchexhaust_analysis_pass_success_rate() {
        let mut p = MatchExhaustAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_matchexhaust_analysis_pass_disable() {
        let mut p = MatchExhaustAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_matchexhaust_pipeline_basic() {
        let mut pipeline = MatchExhaustPipeline::new("main_pipeline");
        pipeline.add_pass(MatchExhaustAnalysisPass::new("pass1"));
        pipeline.add_pass(MatchExhaustAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_matchexhaust_pipeline_disabled_pass() {
        let mut pipeline = MatchExhaustPipeline::new("partial");
        let mut p = MatchExhaustAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(MatchExhaustAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_matchexhaust_diff_basic() {
        let mut d = MatchExhaustDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_matchexhaust_diff_summary() {
        let mut d = MatchExhaustDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }
    #[test]
    fn test_matchexhaust_config_set_get() {
        let mut cfg = MatchExhaustConfig::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_matchexhaust_config_read_only() {
        let mut cfg = MatchExhaustConfig::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_matchexhaust_config_remove() {
        let mut cfg = MatchExhaustConfig::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_matchexhaust_diagnostics_basic() {
        let mut diag = MatchExhaustDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_matchexhaust_diagnostics_max_errors() {
        let mut diag = MatchExhaustDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_matchexhaust_diagnostics_clear() {
        let mut diag = MatchExhaustDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_matchexhaust_config_value_types() {
        let b = MatchExhaustConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = MatchExhaustConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = MatchExhaustConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = MatchExhaustConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = MatchExhaustConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
#[cfg(test)]
mod match_exhaust_ext_tests_2000 {
    use super::*;
    use crate::match_exhaust::*;
    #[test]
    fn test_match_exhaust_ext_result_ok_2000() {
        let r = MatchExhaustExtResult2000::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_match_exhaust_ext_result_err_2000() {
        let r = MatchExhaustExtResult2000::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_match_exhaust_ext_result_partial_2000() {
        let r = MatchExhaustExtResult2000::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_match_exhaust_ext_result_skipped_2000() {
        let r = MatchExhaustExtResult2000::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_match_exhaust_ext_pass_run_2000() {
        let mut p = MatchExhaustExtPass2000::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_match_exhaust_ext_pass_empty_2000() {
        let mut p = MatchExhaustExtPass2000::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_match_exhaust_ext_pass_rate_2000() {
        let mut p = MatchExhaustExtPass2000::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_match_exhaust_ext_pass_disable_2000() {
        let mut p = MatchExhaustExtPass2000::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_match_exhaust_ext_pipeline_basic_2000() {
        let mut pipeline = MatchExhaustExtPipeline2000::new("main_pipeline");
        pipeline.add_pass(MatchExhaustExtPass2000::new("pass1"));
        pipeline.add_pass(MatchExhaustExtPass2000::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_match_exhaust_ext_pipeline_disabled_2000() {
        let mut pipeline = MatchExhaustExtPipeline2000::new("partial");
        let mut p = MatchExhaustExtPass2000::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(MatchExhaustExtPass2000::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_match_exhaust_ext_diff_basic_2000() {
        let mut d = MatchExhaustExtDiff2000::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_match_exhaust_ext_config_set_get_2000() {
        let mut cfg = MatchExhaustExtConfig2000::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_match_exhaust_ext_config_read_only_2000() {
        let mut cfg = MatchExhaustExtConfig2000::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_match_exhaust_ext_config_remove_2000() {
        let mut cfg = MatchExhaustExtConfig2000::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_match_exhaust_ext_diagnostics_basic_2000() {
        let mut diag = MatchExhaustExtDiag2000::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_match_exhaust_ext_diagnostics_max_errors_2000() {
        let mut diag = MatchExhaustExtDiag2000::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_match_exhaust_ext_diagnostics_clear_2000() {
        let mut diag = MatchExhaustExtDiag2000::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_match_exhaust_ext_config_value_types_2000() {
        let b = MatchExhaustExtConfigVal2000::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = MatchExhaustExtConfigVal2000::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = MatchExhaustExtConfigVal2000::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = MatchExhaustExtConfigVal2000::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = MatchExhaustExtConfigVal2000::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
