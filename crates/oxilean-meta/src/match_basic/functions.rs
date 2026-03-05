//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    DecisionTree, MatchBasicAnalysisPass, MatchBasicCache, MatchBasicConfig, MatchBasicConfigValue,
    MatchBasicDiagnostics, MatchBasicDiff, MatchBasicExtConfig4100, MatchBasicExtConfigVal4100,
    MatchBasicExtDiag4100, MatchBasicExtDiff4100, MatchBasicExtPass4100, MatchBasicExtPipeline4100,
    MatchBasicExtResult4100, MatchBasicLogger, MatchBasicPipeline, MatchBasicPriorityQueue,
    MatchBasicRegistry, MatchBasicResult, MatchBasicStats, MatchBasicUtil0, MatchResult,
    MetaMatchArm, MetaMatchExpr, MetaPattern, PatternMatrix, PatternRow,
};
use oxilean_kernel::{Expr, Literal, Name};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::match_basic::*;
    #[test]
    fn test_wildcard_pattern() {
        let p = MetaPattern::Wildcard;
        assert!(p.is_irrefutable());
        assert!(!p.is_constructor());
        assert_eq!(p.num_bindings(), 0);
        assert_eq!(p.depth(), 0);
    }
    #[test]
    fn test_var_pattern() {
        let p = MetaPattern::Var(Name::str("x"));
        assert!(p.is_irrefutable());
        assert_eq!(p.num_bindings(), 1);
        assert_eq!(p.bound_vars(), vec![Name::str("x")]);
    }
    #[test]
    fn test_ctor_pattern() {
        let p = MetaPattern::Constructor(
            Name::str("Nat.succ"),
            vec![MetaPattern::Var(Name::str("n"))],
        );
        assert!(p.is_constructor());
        assert_eq!(p.ctor_name(), Some(&Name::str("Nat.succ")));
        assert_eq!(p.num_bindings(), 1);
        assert_eq!(p.depth(), 1);
    }
    #[test]
    fn test_nested_ctor() {
        let p = MetaPattern::Constructor(
            Name::str("Nat.succ"),
            vec![MetaPattern::Constructor(
                Name::str("Nat.succ"),
                vec![MetaPattern::Var(Name::str("n"))],
            )],
        );
        assert_eq!(p.depth(), 2);
        assert_eq!(p.num_bindings(), 1);
    }
    #[test]
    fn test_literal_pattern() {
        let p = MetaPattern::Literal(Literal::Nat(42));
        assert!(p.is_literal());
        assert!(!p.is_irrefutable());
        assert_eq!(p.num_bindings(), 0);
    }
    #[test]
    fn test_as_pattern() {
        let p = MetaPattern::As(
            Box::new(MetaPattern::Constructor(
                Name::str("Nat.succ"),
                vec![MetaPattern::Var(Name::str("n"))],
            )),
            Name::str("m"),
        );
        assert_eq!(p.num_bindings(), 2);
        let vars = p.bound_vars();
        assert!(vars.contains(&Name::str("m")));
        assert!(vars.contains(&Name::str("n")));
    }
    #[test]
    fn test_or_pattern() {
        let p = MetaPattern::Or(
            Box::new(MetaPattern::Literal(Literal::Nat(0))),
            Box::new(MetaPattern::Literal(Literal::Nat(1))),
        );
        assert_eq!(p.num_bindings(), 0);
    }
    #[test]
    fn test_match_expr_creation() {
        let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
        let discr = Expr::Const(Name::str("n"), vec![]);
        let mut match_expr = MetaMatchExpr::new(vec![discr], vec![nat_ty]);
        assert_eq!(match_expr.num_discriminants(), 1);
        assert_eq!(match_expr.num_arms(), 0);
        match_expr.add_arm(MetaMatchArm {
            patterns: vec![MetaPattern::Wildcard],
            guard: None,
            rhs: Expr::Lit(Literal::Nat(0)),
        });
        assert_eq!(match_expr.num_arms(), 1);
    }
    #[test]
    fn test_validate_patterns() {
        let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
        let discr = Expr::Const(Name::str("n"), vec![]);
        let mut match_expr = MetaMatchExpr::new(vec![discr], vec![nat_ty]);
        match_expr.add_arm(MetaMatchArm {
            patterns: vec![MetaPattern::Wildcard],
            guard: None,
            rhs: Expr::Lit(Literal::Nat(0)),
        });
        assert!(match_expr.validate_patterns().is_ok());
        match_expr.add_arm(MetaMatchArm {
            patterns: vec![MetaPattern::Wildcard, MetaPattern::Wildcard],
            guard: None,
            rhs: Expr::Lit(Literal::Nat(1)),
        });
        assert!(match_expr.validate_patterns().is_err());
    }
    #[test]
    fn test_subpatterns() {
        let p = MetaPattern::Constructor(
            Name::str("Pair"),
            vec![
                MetaPattern::Var(Name::str("x")),
                MetaPattern::Var(Name::str("y")),
            ],
        );
        assert_eq!(p.subpatterns().len(), 2);
        let w = MetaPattern::Wildcard;
        assert_eq!(w.subpatterns().len(), 0);
    }
}
/// Try to syntactically match a `MetaPattern` against an expression.
///
/// This is a simple structural matcher that does NOT perform WHNF reduction.
/// It is used as a first-pass check before invoking the full elaborator.
pub fn try_match(pattern: &MetaPattern, expr: &Expr) -> MatchResult {
    match pattern {
        MetaPattern::Wildcard => MatchResult::Success(vec![]),
        MetaPattern::Var(name) => MatchResult::Success(vec![(name.clone(), expr.clone())]),
        MetaPattern::Inaccessible(_) => MatchResult::Success(vec![]),
        MetaPattern::Literal(lit) => {
            if let Expr::Lit(e_lit) = expr {
                if e_lit == lit {
                    MatchResult::Success(vec![])
                } else {
                    MatchResult::Failure
                }
            } else {
                MatchResult::Undetermined
            }
        }
        MetaPattern::Constructor(ctor_name, sub_pats) => {
            let (head, args) = collect_app_ref(expr);
            match head {
                Expr::Const(name, _) if name == ctor_name => {
                    if args.len() != sub_pats.len() {
                        return MatchResult::Failure;
                    }
                    let mut all_bindings = Vec::new();
                    for (sub_pat, arg) in sub_pats.iter().zip(args.iter()) {
                        match try_match(sub_pat, arg) {
                            MatchResult::Success(b) => all_bindings.extend(b),
                            MatchResult::Failure => return MatchResult::Failure,
                            MatchResult::Undetermined => return MatchResult::Undetermined,
                        }
                    }
                    MatchResult::Success(all_bindings)
                }
                Expr::Const(_, _) => MatchResult::Failure,
                _ => MatchResult::Undetermined,
            }
        }
        MetaPattern::As(inner, name) => match try_match(inner, expr) {
            MatchResult::Success(mut b) => {
                b.push((name.clone(), expr.clone()));
                MatchResult::Success(b)
            }
            other => other,
        },
        MetaPattern::Or(left, right) => match try_match(left, expr) {
            MatchResult::Failure => try_match(right, expr),
            other => other,
        },
    }
}
/// Collect head and arguments of a nested application (returns references).
pub(super) fn collect_app_ref(expr: &Expr) -> (&Expr, Vec<&Expr>) {
    let mut args = Vec::new();
    let mut e = expr;
    while let Expr::App(f, a) = e {
        args.push(a.as_ref());
        e = f;
    }
    args.reverse();
    (e, args)
}
/// Build a trivial decision tree for a single wildcard arm.
pub fn trivial_decision_tree(arm_idx: usize) -> DecisionTree {
    DecisionTree::Leaf(arm_idx)
}
/// Build a simple switch decision tree for constructor patterns.
pub fn simple_switch_tree(
    discr_idx: usize,
    cases: Vec<(oxilean_kernel::Name, usize)>,
    default_arm: Option<usize>,
) -> DecisionTree {
    let case_nodes = cases
        .into_iter()
        .map(|(name, arm_idx)| (name, Box::new(DecisionTree::Leaf(arm_idx))))
        .collect();
    DecisionTree::Switch {
        discr_idx,
        cases: case_nodes,
        default: default_arm.map(|idx| Box::new(DecisionTree::Leaf(idx))),
    }
}
/// Check if a list of patterns covers all cases for a given type.
///
/// This is a simplified check: if any pattern is a wildcard or variable,
/// the match is trivially exhaustive.
pub fn is_exhaustive(patterns: &[MetaPattern]) -> bool {
    patterns.iter().any(|p| p.is_irrefutable())
}
/// Check if a list of patterns has any redundant (unreachable) arms.
///
/// A pattern is redundant if a previous pattern already covers its cases.
pub fn find_redundant_arms(patterns: &[MetaPattern]) -> Vec<usize> {
    let mut redundant = Vec::new();
    let mut covered_wildcards = false;
    for (i, p) in patterns.iter().enumerate() {
        if covered_wildcards {
            redundant.push(i);
        } else if p.is_irrefutable() {
            covered_wildcards = true;
        }
    }
    redundant
}
/// Format a `MetaPattern` for display in error messages.
pub fn format_pattern(p: &MetaPattern) -> String {
    match p {
        MetaPattern::Wildcard => "_".to_string(),
        MetaPattern::Var(n) => format!("{}", n),
        MetaPattern::Literal(Literal::Nat(n)) => format!("{}", n),
        MetaPattern::Literal(Literal::Str(s)) => format!("\"{}\"", s),
        MetaPattern::Constructor(name, sub_pats) => {
            if sub_pats.is_empty() {
                format!("{}", name)
            } else {
                let args: Vec<String> = sub_pats.iter().map(format_pattern).collect();
                format!("({} {})", name, args.join(" "))
            }
        }
        MetaPattern::As(inner, name) => format!("{} as {}", format_pattern(inner), name),
        MetaPattern::Or(l, r) => format!("{} | {}", format_pattern(l), format_pattern(r)),
        MetaPattern::Inaccessible(_) => "._".to_string(),
    }
}
#[cfg(test)]
mod tests_extra {
    use super::*;
    use crate::match_basic::*;
    use oxilean_kernel::{Literal, Name};
    #[test]
    fn test_try_match_wildcard() {
        let p = MetaPattern::Wildcard;
        let e = Expr::Lit(Literal::Nat(42));
        let r = try_match(&p, &e);
        assert!(r.is_success());
        assert_eq!(r.bindings().len(), 0);
    }
    #[test]
    fn test_try_match_var() {
        let p = MetaPattern::Var(Name::str("x"));
        let e = Expr::Lit(Literal::Nat(5));
        let r = try_match(&p, &e);
        assert!(r.is_success());
        assert_eq!(r.bindings().len(), 1);
        assert_eq!(r.bindings()[0].0, Name::str("x"));
    }
    #[test]
    fn test_try_match_literal_ok() {
        let p = MetaPattern::Literal(Literal::Nat(42));
        let e = Expr::Lit(Literal::Nat(42));
        assert!(try_match(&p, &e).is_success());
    }
    #[test]
    fn test_try_match_literal_fail() {
        let p = MetaPattern::Literal(Literal::Nat(0));
        let e = Expr::Lit(Literal::Nat(1));
        assert!(try_match(&p, &e).is_failure());
    }
    #[test]
    fn test_try_match_constructor_ok() {
        let p = MetaPattern::Constructor(
            Name::str("Nat.succ"),
            vec![MetaPattern::Var(Name::str("n"))],
        );
        let e = Expr::App(
            Box::new(Expr::Const(Name::str("Nat.succ"), vec![])),
            Box::new(Expr::Lit(Literal::Nat(0))),
        );
        let r = try_match(&p, &e);
        assert!(r.is_success());
        assert_eq!(r.bindings().len(), 1);
    }
    #[test]
    fn test_try_match_constructor_wrong_name() {
        let p = MetaPattern::Constructor(Name::str("Nat.succ"), vec![MetaPattern::Wildcard]);
        let e = Expr::App(
            Box::new(Expr::Const(Name::str("Nat.zero"), vec![])),
            Box::new(Expr::Lit(Literal::Nat(0))),
        );
        assert!(try_match(&p, &e).is_failure());
    }
    #[test]
    fn test_try_match_as_pattern() {
        let p = MetaPattern::As(Box::new(MetaPattern::Wildcard), Name::str("x"));
        let e = Expr::Lit(Literal::Nat(7));
        let r = try_match(&p, &e);
        assert!(r.is_success());
        assert_eq!(r.bindings().len(), 1);
    }
    #[test]
    fn test_try_match_or_first_succeeds() {
        let p = MetaPattern::Or(
            Box::new(MetaPattern::Literal(Literal::Nat(0))),
            Box::new(MetaPattern::Literal(Literal::Nat(1))),
        );
        let e = Expr::Lit(Literal::Nat(0));
        assert!(try_match(&p, &e).is_success());
    }
    #[test]
    fn test_try_match_or_second_succeeds() {
        let p = MetaPattern::Or(
            Box::new(MetaPattern::Literal(Literal::Nat(0))),
            Box::new(MetaPattern::Literal(Literal::Nat(1))),
        );
        let e = Expr::Lit(Literal::Nat(1));
        assert!(try_match(&p, &e).is_success());
    }
    #[test]
    fn test_decision_tree_leaf() {
        let t = DecisionTree::Leaf(0);
        assert!(t.is_leaf());
        assert_eq!(t.num_reachable_arms(), 1);
        assert_eq!(t.depth(), 0);
    }
    #[test]
    fn test_decision_tree_fail() {
        let t = DecisionTree::Fail;
        assert!(t.is_fail());
        assert_eq!(t.num_reachable_arms(), 0);
    }
    #[test]
    fn test_simple_switch_tree() {
        let t = simple_switch_tree(
            0,
            vec![(Name::str("Nat.zero"), 0), (Name::str("Nat.succ"), 1)],
            None,
        );
        assert_eq!(t.num_reachable_arms(), 2);
        assert_eq!(t.depth(), 1);
    }
    #[test]
    fn test_is_exhaustive() {
        let pats = vec![MetaPattern::Literal(Literal::Nat(0)), MetaPattern::Wildcard];
        assert!(is_exhaustive(&pats));
    }
    #[test]
    fn test_is_exhaustive_false() {
        let pats = vec![MetaPattern::Literal(Literal::Nat(0))];
        assert!(!is_exhaustive(&pats));
    }
    #[test]
    fn test_find_redundant_arms() {
        let pats = vec![
            MetaPattern::Wildcard,
            MetaPattern::Literal(Literal::Nat(0)),
            MetaPattern::Var(Name::str("x")),
        ];
        let redundant = find_redundant_arms(&pats);
        assert_eq!(redundant, vec![1, 2]);
    }
    #[test]
    fn test_format_pattern_wildcard() {
        assert_eq!(format_pattern(&MetaPattern::Wildcard), "_");
    }
    #[test]
    fn test_format_pattern_constructor() {
        let p = MetaPattern::Constructor(
            Name::str("Nat.succ"),
            vec![MetaPattern::Var(Name::str("n"))],
        );
        let s = format_pattern(&p);
        assert!(s.contains("Nat.succ"));
        assert!(s.contains("n"));
    }
}
/// Check whether two patterns have disjoint sets of bound variable names.
pub fn patterns_disjoint(p1: &MetaPattern, p2: &MetaPattern) -> bool {
    let v1 = p1.bound_vars();
    let v2 = p2.bound_vars();
    !v1.iter().any(|n| v2.contains(n))
}
/// Count literal patterns in a list.
pub fn count_literals(patterns: &[MetaPattern]) -> usize {
    patterns.iter().filter(|p| p.is_literal()).count()
}
/// Count constructor patterns in a list.
pub fn count_constructors(patterns: &[MetaPattern]) -> usize {
    patterns.iter().filter(|p| p.is_constructor()).count()
}
/// Count wildcard/variable (irrefutable) patterns in a list.
pub fn count_irrefutable(patterns: &[MetaPattern]) -> usize {
    patterns.iter().filter(|p| p.is_irrefutable()).count()
}
/// Get the maximum depth among all patterns in a list.
pub fn max_pattern_depth(patterns: &[MetaPattern]) -> usize {
    patterns.iter().map(|p| p.depth()).max().unwrap_or(0)
}
/// Flatten a nested Or pattern into a flat list of alternatives.
pub fn flatten_or_pattern(p: &MetaPattern) -> Vec<&MetaPattern> {
    match p {
        MetaPattern::Or(l, r) => {
            let mut alts = flatten_or_pattern(l);
            alts.extend(flatten_or_pattern(r));
            alts
        }
        _ => vec![p],
    }
}
/// Check that an Or-pattern's two branches bind exactly the same variable names.
pub fn or_pattern_vars_match(left: &MetaPattern, right: &MetaPattern) -> bool {
    let mut lv = left.bound_vars();
    let mut rv = right.bound_vars();
    lv.sort_by_key(|n| n.to_string());
    rv.sort_by_key(|n| n.to_string());
    lv == rv
}
/// Validate an Or-pattern: both branches must bind the same variables.
pub fn validate_or_pattern(p: &MetaPattern) -> Result<(), String> {
    match p {
        MetaPattern::Or(left, right) => {
            if !or_pattern_vars_match(left, right) {
                Err(format!(
                    "Or-pattern branches bind different variables: {:?} vs {:?}",
                    left.bound_vars(),
                    right.bound_vars()
                ))
            } else {
                validate_or_pattern(left)?;
                validate_or_pattern(right)
            }
        }
        MetaPattern::Constructor(_, sub) => {
            for p in sub {
                validate_or_pattern(p)?;
            }
            Ok(())
        }
        MetaPattern::As(inner, _) => validate_or_pattern(inner),
        _ => Ok(()),
    }
}
/// Simplify a pattern by flattening trivial As-patterns.
///
/// `As(Wildcard, x)` → `Var(x)` since wildcard + name = variable.
pub fn simplify_pattern(p: MetaPattern) -> MetaPattern {
    match p {
        MetaPattern::As(inner, name) => match *inner {
            MetaPattern::Wildcard => MetaPattern::Var(name),
            other => MetaPattern::As(Box::new(simplify_pattern(other)), name),
        },
        MetaPattern::Constructor(name, sub) => {
            MetaPattern::Constructor(name, sub.into_iter().map(simplify_pattern).collect())
        }
        MetaPattern::Or(l, r) => MetaPattern::Or(
            Box::new(simplify_pattern(*l)),
            Box::new(simplify_pattern(*r)),
        ),
        other => other,
    }
}
#[cfg(test)]
mod match_basic_new_tests {
    use super::*;
    use crate::match_basic::*;
    #[test]
    fn test_or_pattern_vars_match_both_empty() {
        let l = MetaPattern::Wildcard;
        let r = MetaPattern::Wildcard;
        assert!(or_pattern_vars_match(&l, &r));
    }
    #[test]
    fn test_or_pattern_vars_match_same_var() {
        let l = MetaPattern::Var(Name::str("x"));
        let r = MetaPattern::Var(Name::str("x"));
        assert!(or_pattern_vars_match(&l, &r));
    }
    #[test]
    fn test_or_pattern_vars_match_different() {
        let l = MetaPattern::Var(Name::str("x"));
        let r = MetaPattern::Var(Name::str("y"));
        assert!(!or_pattern_vars_match(&l, &r));
    }
    #[test]
    fn test_validate_or_pattern_ok() {
        let p = MetaPattern::Or(
            Box::new(MetaPattern::Wildcard),
            Box::new(MetaPattern::Wildcard),
        );
        assert!(validate_or_pattern(&p).is_ok());
    }
    #[test]
    fn test_validate_or_pattern_err() {
        let p = MetaPattern::Or(
            Box::new(MetaPattern::Var(Name::str("x"))),
            Box::new(MetaPattern::Var(Name::str("y"))),
        );
        assert!(validate_or_pattern(&p).is_err());
    }
    #[test]
    fn test_simplify_pattern_as_wildcard() {
        let p = MetaPattern::As(Box::new(MetaPattern::Wildcard), Name::str("x"));
        let simplified = simplify_pattern(p);
        assert_eq!(simplified, MetaPattern::Var(Name::str("x")));
    }
    #[test]
    fn test_simplify_pattern_no_change() {
        let p = MetaPattern::Literal(Literal::Nat(42));
        let simplified = simplify_pattern(p.clone());
        assert_eq!(simplified, p);
    }
    #[test]
    fn test_pattern_row_is_all_irrefutable() {
        let row = PatternRow::new(
            vec![MetaPattern::Wildcard, MetaPattern::Var(Name::str("x"))],
            0,
        );
        assert!(row.is_all_irrefutable());
    }
    #[test]
    fn test_pattern_row_not_all_irrefutable() {
        let row = PatternRow::new(
            vec![MetaPattern::Wildcard, MetaPattern::Literal(Literal::Nat(0))],
            0,
        );
        assert!(!row.is_all_irrefutable());
    }
    #[test]
    fn test_pattern_row_first_refutable() {
        let row = PatternRow::new(
            vec![MetaPattern::Wildcard, MetaPattern::Literal(Literal::Nat(0))],
            0,
        );
        let refutable = row.first_refutable();
        assert!(refutable.is_some());
        assert_eq!(refutable.expect("refutable should be valid").0, 1);
    }
    #[test]
    fn test_pattern_matrix_basic() {
        let mut matrix = PatternMatrix::new(2);
        assert!(matrix.is_empty());
        matrix.add_row(PatternRow::new(
            vec![MetaPattern::Wildcard, MetaPattern::Wildcard],
            0,
        ));
        assert_eq!(matrix.num_rows(), 1);
        assert!(matrix.first_row_is_catchall());
    }
    #[test]
    fn test_pattern_matrix_specialize_wildcard() {
        let mut matrix = PatternMatrix::new(1);
        matrix.add_row(PatternRow::new(vec![MetaPattern::Wildcard], 0));
        let specialized = matrix.specialize(0, &Name::str("Nat.zero"), 0);
        assert_eq!(specialized.num_rows(), 1);
    }
    #[test]
    fn test_pattern_matrix_default_matrix() {
        let mut matrix = PatternMatrix::new(2);
        matrix.add_row(PatternRow::new(
            vec![MetaPattern::Wildcard, MetaPattern::Wildcard],
            0,
        ));
        matrix.add_row(PatternRow::new(
            vec![MetaPattern::Literal(Literal::Nat(0)), MetaPattern::Wildcard],
            1,
        ));
        let def = matrix.default_matrix(0);
        assert_eq!(def.num_rows(), 1);
    }
}
/// Compute a simple hash of a MatchBasic name.
#[allow(dead_code)]
pub fn matchbasic_hash(name: &str) -> u64 {
    let mut h: u64 = 14695981039346656037;
    for b in name.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(1099511628211);
    }
    h
}
/// Check if a MatchBasic name is valid.
#[allow(dead_code)]
pub fn matchbasic_is_valid_name(name: &str) -> bool {
    !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_')
}
/// Count the occurrences of a character in a MatchBasic string.
#[allow(dead_code)]
pub fn matchbasic_count_char(s: &str, c: char) -> usize {
    s.chars().filter(|&ch| ch == c).count()
}
/// Truncate a MatchBasic string to a maximum length.
#[allow(dead_code)]
pub fn matchbasic_truncate(s: &str, max_len: usize) -> &str {
    if s.len() <= max_len {
        s
    } else {
        &s[..max_len]
    }
}
/// Join MatchBasic strings with a separator.
#[allow(dead_code)]
pub fn matchbasic_join(parts: &[&str], sep: &str) -> String {
    parts.join(sep)
}
#[cfg(test)]
mod matchbasic_ext_tests {
    use super::*;
    use crate::match_basic::*;
    #[test]
    fn test_matchbasic_util_new() {
        let u = MatchBasicUtil0::new(1, "test", 42);
        assert_eq!(u.id, 1);
        assert_eq!(u.name, "test");
        assert_eq!(u.value, 42);
        assert!(u.is_active());
    }
    #[test]
    fn test_matchbasic_util_tag() {
        let u = MatchBasicUtil0::new(2, "tagged", 10).with_tag("important");
        assert!(u.has_tag("important"));
        assert_eq!(u.tag_count(), 1);
    }
    #[test]
    fn test_matchbasic_util_disable() {
        let u = MatchBasicUtil0::new(3, "disabled", 100).disable();
        assert!(!u.is_active());
        assert_eq!(u.score(), 0);
    }
    #[test]
    fn test_matchbasic_registry_register() {
        let mut reg = MatchBasicRegistry::new(10);
        let u = MatchBasicUtil0::new(1, "a", 1);
        assert!(reg.register(u));
        assert_eq!(reg.count(), 1);
    }
    #[test]
    fn test_matchbasic_registry_lookup() {
        let mut reg = MatchBasicRegistry::new(10);
        reg.register(MatchBasicUtil0::new(5, "five", 5));
        assert!(reg.lookup(5).is_some());
        assert!(reg.lookup(99).is_none());
    }
    #[test]
    fn test_matchbasic_registry_capacity() {
        let mut reg = MatchBasicRegistry::new(2);
        reg.register(MatchBasicUtil0::new(1, "a", 1));
        reg.register(MatchBasicUtil0::new(2, "b", 2));
        assert!(reg.is_full());
        assert!(!reg.register(MatchBasicUtil0::new(3, "c", 3)));
    }
    #[test]
    fn test_matchbasic_registry_score() {
        let mut reg = MatchBasicRegistry::new(10);
        reg.register(MatchBasicUtil0::new(1, "a", 10));
        reg.register(MatchBasicUtil0::new(2, "b", 20));
        assert_eq!(reg.total_score(), 30);
    }
    #[test]
    fn test_matchbasic_cache_hit_miss() {
        let mut cache = MatchBasicCache::new();
        cache.insert("key1", 42);
        assert_eq!(cache.get("key1"), Some(42));
        assert_eq!(cache.get("key2"), None);
        assert_eq!(cache.hits, 1);
        assert_eq!(cache.misses, 1);
    }
    #[test]
    fn test_matchbasic_cache_hit_rate() {
        let mut cache = MatchBasicCache::new();
        cache.insert("k", 1);
        cache.get("k");
        cache.get("k");
        cache.get("nope");
        assert!((cache.hit_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_matchbasic_cache_clear() {
        let mut cache = MatchBasicCache::new();
        cache.insert("k", 1);
        cache.clear();
        assert_eq!(cache.size(), 0);
        assert_eq!(cache.hits, 0);
    }
    #[test]
    fn test_matchbasic_logger_basic() {
        let mut logger = MatchBasicLogger::new(100);
        logger.log("msg1");
        logger.log("msg2");
        assert_eq!(logger.count(), 2);
        assert_eq!(logger.last(), Some("msg2"));
    }
    #[test]
    fn test_matchbasic_logger_capacity() {
        let mut logger = MatchBasicLogger::new(2);
        logger.log("a");
        logger.log("b");
        logger.log("c");
        assert_eq!(logger.count(), 2);
    }
    #[test]
    fn test_matchbasic_stats_success() {
        let mut stats = MatchBasicStats::new();
        stats.record_success(100);
        stats.record_success(200);
        assert_eq!(stats.total_ops, 2);
        assert_eq!(stats.successful_ops, 2);
        assert!((stats.success_rate() - 1.0).abs() < 1e-9);
    }
    #[test]
    fn test_matchbasic_stats_failure() {
        let mut stats = MatchBasicStats::new();
        stats.record_success(100);
        stats.record_failure();
        assert!((stats.success_rate() - 0.5).abs() < 1e-9);
    }
    #[test]
    fn test_matchbasic_stats_merge() {
        let mut a = MatchBasicStats::new();
        let mut b = MatchBasicStats::new();
        a.record_success(100);
        b.record_failure();
        a.merge(&b);
        assert_eq!(a.total_ops, 2);
    }
    #[test]
    fn test_matchbasic_priority_queue() {
        let mut pq = MatchBasicPriorityQueue::new();
        pq.push(MatchBasicUtil0::new(1, "low", 1), 1);
        pq.push(MatchBasicUtil0::new(2, "high", 2), 100);
        let (_, p) = pq.pop().expect("collection should not be empty");
        assert_eq!(p, 100);
    }
    #[test]
    fn test_matchbasic_hash() {
        let h1 = matchbasic_hash("foo");
        let h2 = matchbasic_hash("foo");
        assert_eq!(h1, h2);
        let h3 = matchbasic_hash("bar");
        assert_ne!(h1, h3);
    }
    #[test]
    fn test_matchbasic_valid_name() {
        assert!(matchbasic_is_valid_name("foo_bar"));
        assert!(!matchbasic_is_valid_name("foo-bar"));
        assert!(!matchbasic_is_valid_name(""));
    }
    #[test]
    fn test_matchbasic_join() {
        let parts = ["a", "b", "c"];
        assert_eq!(matchbasic_join(&parts, ", "), "a, b, c");
    }
}
#[cfg(test)]
mod matchbasic_analysis_tests {
    use super::*;
    use crate::match_basic::*;
    #[test]
    fn test_matchbasic_result_ok() {
        let r = MatchBasicResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_matchbasic_result_err() {
        let r = MatchBasicResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_matchbasic_result_partial() {
        let r = MatchBasicResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_matchbasic_result_skipped() {
        let r = MatchBasicResult::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_matchbasic_analysis_pass_run() {
        let mut p = MatchBasicAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_matchbasic_analysis_pass_empty_input() {
        let mut p = MatchBasicAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_matchbasic_analysis_pass_success_rate() {
        let mut p = MatchBasicAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_matchbasic_analysis_pass_disable() {
        let mut p = MatchBasicAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_matchbasic_pipeline_basic() {
        let mut pipeline = MatchBasicPipeline::new("main_pipeline");
        pipeline.add_pass(MatchBasicAnalysisPass::new("pass1"));
        pipeline.add_pass(MatchBasicAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_matchbasic_pipeline_disabled_pass() {
        let mut pipeline = MatchBasicPipeline::new("partial");
        let mut p = MatchBasicAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(MatchBasicAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_matchbasic_diff_basic() {
        let mut d = MatchBasicDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_matchbasic_diff_summary() {
        let mut d = MatchBasicDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }
    #[test]
    fn test_matchbasic_config_set_get() {
        let mut cfg = MatchBasicConfig::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_matchbasic_config_read_only() {
        let mut cfg = MatchBasicConfig::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_matchbasic_config_remove() {
        let mut cfg = MatchBasicConfig::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_matchbasic_diagnostics_basic() {
        let mut diag = MatchBasicDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_matchbasic_diagnostics_max_errors() {
        let mut diag = MatchBasicDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_matchbasic_diagnostics_clear() {
        let mut diag = MatchBasicDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_matchbasic_config_value_types() {
        let b = MatchBasicConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = MatchBasicConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = MatchBasicConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = MatchBasicConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = MatchBasicConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
#[cfg(test)]
mod match_basic_ext_tests_4100 {
    use super::*;
    use crate::match_basic::*;
    #[test]
    fn test_match_basic_ext_result_ok_4100() {
        let r = MatchBasicExtResult4100::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_match_basic_ext_result_err_4100() {
        let r = MatchBasicExtResult4100::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_match_basic_ext_result_partial_4100() {
        let r = MatchBasicExtResult4100::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_match_basic_ext_result_skipped_4100() {
        let r = MatchBasicExtResult4100::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_match_basic_ext_pass_run_4100() {
        let mut p = MatchBasicExtPass4100::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_match_basic_ext_pass_empty_4100() {
        let mut p = MatchBasicExtPass4100::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_match_basic_ext_pass_rate_4100() {
        let mut p = MatchBasicExtPass4100::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_match_basic_ext_pass_disable_4100() {
        let mut p = MatchBasicExtPass4100::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_match_basic_ext_pipeline_basic_4100() {
        let mut pipeline = MatchBasicExtPipeline4100::new("main_pipeline");
        pipeline.add_pass(MatchBasicExtPass4100::new("pass1"));
        pipeline.add_pass(MatchBasicExtPass4100::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_match_basic_ext_pipeline_disabled_4100() {
        let mut pipeline = MatchBasicExtPipeline4100::new("partial");
        let mut p = MatchBasicExtPass4100::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(MatchBasicExtPass4100::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_match_basic_ext_diff_basic_4100() {
        let mut d = MatchBasicExtDiff4100::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_match_basic_ext_config_set_get_4100() {
        let mut cfg = MatchBasicExtConfig4100::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_match_basic_ext_config_read_only_4100() {
        let mut cfg = MatchBasicExtConfig4100::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_match_basic_ext_config_remove_4100() {
        let mut cfg = MatchBasicExtConfig4100::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_match_basic_ext_diagnostics_basic_4100() {
        let mut diag = MatchBasicExtDiag4100::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_match_basic_ext_diagnostics_max_errors_4100() {
        let mut diag = MatchBasicExtDiag4100::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_match_basic_ext_diagnostics_clear_4100() {
        let mut diag = MatchBasicExtDiag4100::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_match_basic_ext_config_value_types_4100() {
        let b = MatchBasicExtConfigVal4100::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = MatchBasicExtConfigVal4100::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = MatchBasicExtConfigVal4100::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = MatchBasicExtConfigVal4100::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = MatchBasicExtConfigVal4100::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
