//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Expr, Name};

use super::types::{
    ColumnHeuristic, ColumnSelector, DecisionTree, DecisionTreeStats, Equation,
    EquationCompilationReport, EquationCompiler, EquationCompilerConfig, EquationNormalizer,
    EquationSet, ExhaustCheck, ExhaustivenessAnalyzer, ExhaustivenessResult, FullPatternMatrix,
    OverlapChecker, Pattern, PatternAnnotation, PatternAnnotationKind, PatternMatrix,
    PatternMatrix2, PatternRow,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::equation::*;
    use oxilean_kernel::Literal;
    #[test]
    fn test_pattern_is_irrefutable() {
        assert!(Pattern::Wild.is_irrefutable());
        assert!(Pattern::Var(Name::str("x")).is_irrefutable());
        assert!(!Pattern::Ctor(Name::str("Some"), vec![]).is_irrefutable());
        assert!(!Pattern::Lit(Literal::Nat(42)).is_irrefutable());
    }
    #[test]
    fn test_pattern_bound_vars() {
        let p = Pattern::Var(Name::str("x"));
        let vars = p.bound_vars();
        assert_eq!(vars.len(), 1);
        let p2 = Pattern::Wild;
        let vars2 = p2.bound_vars();
        assert_eq!(vars2.len(), 0);
    }
    #[test]
    fn test_compiler_create() {
        let compiler = EquationCompiler::new(vec![]);
        assert!(compiler.equations.is_empty());
    }
    #[test]
    fn test_compile_empty() {
        let compiler = EquationCompiler::new(vec![]);
        let tree = compiler.compile().expect("test operation should succeed");
        assert!(matches!(tree, DecisionTree::Fail));
    }
    #[test]
    fn test_check_exhaustive() {
        let compiler = EquationCompiler::new(vec![]);
        assert!(!compiler.check_exhaustive());
        let eq = Equation {
            patterns: vec![Pattern::Wild],
            rhs: Expr::BVar(0),
            guard: None,
            source_loc: None,
        };
        let compiler2 = EquationCompiler::new(vec![eq]);
        assert!(compiler2.check_exhaustive());
    }
    #[test]
    fn test_check_redundant() {
        let compiler = EquationCompiler::new(vec![]);
        let redundant = compiler.check_redundant();
        assert_eq!(redundant.len(), 0);
    }
}
/// Check exhaustiveness of a set of patterns for a given type.
///
/// Returns `Exhaustive` if all cases are covered, or `Missing`/`Redundant`
/// with diagnostic information.
pub fn check_exhaustiveness(patterns: &[Pattern], ty: &Name) -> ExhaustivenessResult {
    if patterns.is_empty() {
        return ExhaustivenessResult::Missing(vec![Pattern::Wild]);
    }
    for (i, pat) in patterns.iter().enumerate() {
        if pat.is_irrefutable() {
            let redundant: Vec<usize> = (i + 1..patterns.len()).collect();
            if !redundant.is_empty() {
                return ExhaustivenessResult::Redundant(redundant);
            }
            return ExhaustivenessResult::Exhaustive;
        }
    }
    let ty_str = ty.to_string();
    match ty_str.as_str() {
        "Bool" => {
            let has_true = patterns.iter().any(|p| {
                matches!(
                    p, Pattern::Ctor(n, _) if { let s = n.to_string(); s ==
                    "Bool.true" || s == "true" }
                )
            });
            let has_false = patterns.iter().any(|p| {
                matches!(
                    p, Pattern::Ctor(n, _) if { let s = n.to_string(); s ==
                    "Bool.false" || s == "false" }
                )
            });
            if has_true && has_false {
                ExhaustivenessResult::Exhaustive
            } else if !has_true {
                ExhaustivenessResult::Missing(vec![Pattern::Ctor(Name::str("Bool.true"), vec![])])
            } else {
                ExhaustivenessResult::Missing(vec![Pattern::Ctor(Name::str("Bool.false"), vec![])])
            }
        }
        "Nat" => {
            let has_zero = patterns.iter().any(|p| {
                matches!(
                    p, Pattern::Ctor(n, _) if { let s = n.to_string(); s ==
                    "Nat.zero" || s == "zero" }
                ) || matches!(p, Pattern::Lit(oxilean_kernel::Literal::Nat(0)))
            });
            let has_succ = patterns.iter().any(|p| {
                matches!(
                    p, Pattern::Ctor(n, _) if { let s = n.to_string(); s ==
                    "Nat.succ" || s == "succ" }
                )
            });
            if has_zero && has_succ {
                ExhaustivenessResult::Exhaustive
            } else if !has_zero {
                ExhaustivenessResult::Missing(vec![Pattern::Ctor(Name::str("Nat.zero"), vec![])])
            } else {
                ExhaustivenessResult::Missing(vec![Pattern::Ctor(
                    Name::str("Nat.succ"),
                    vec![Pattern::Wild],
                )])
            }
        }
        "Unit" | "PUnit" => {
            let has_unit = patterns.iter().any(|p| {
                matches!(
                    p, Pattern::Ctor(n, _) if { let s = n.to_string(); s ==
                    "Unit.unit" || s == "unit" || s == "PUnit.unit" }
                )
            });
            if has_unit {
                ExhaustivenessResult::Exhaustive
            } else {
                ExhaustivenessResult::Missing(vec![Pattern::Ctor(Name::str("Unit.unit"), vec![])])
            }
        }
        "Option" => {
            let has_none = patterns.iter().any(|p| {
                matches!(
                    p, Pattern::Ctor(n, _) if { let s = n.to_string(); s ==
                    "Option.none" || s == "none" || s == "None" }
                )
            });
            let has_some = patterns.iter().any(|p| {
                matches!(
                    p, Pattern::Ctor(n, _) if { let s = n.to_string(); s ==
                    "Option.some" || s == "some" || s == "Some" }
                )
            });
            if has_none && has_some {
                ExhaustivenessResult::Exhaustive
            } else if !has_none {
                ExhaustivenessResult::Missing(vec![Pattern::Ctor(Name::str("Option.none"), vec![])])
            } else {
                ExhaustivenessResult::Missing(vec![Pattern::Ctor(
                    Name::str("Option.some"),
                    vec![Pattern::Wild],
                )])
            }
        }
        "List" => {
            let has_nil = patterns.iter().any(|p| {
                matches!(
                    p, Pattern::Ctor(n, _) if { let s = n.to_string(); s ==
                    "List.nil" || s == "nil" || s == "[]" }
                )
            });
            let has_cons = patterns.iter().any(|p| {
                matches!(
                    p, Pattern::Ctor(n, _) if { let s = n.to_string(); s ==
                    "List.cons" || s == "cons" }
                )
            });
            if has_nil && has_cons {
                ExhaustivenessResult::Exhaustive
            } else if !has_nil {
                ExhaustivenessResult::Missing(vec![Pattern::Ctor(Name::str("List.nil"), vec![])])
            } else {
                ExhaustivenessResult::Missing(vec![Pattern::Ctor(
                    Name::str("List.cons"),
                    vec![Pattern::Wild, Pattern::Wild],
                )])
            }
        }
        "Sum" | "Either" => {
            let has_inl = patterns.iter().any(|p| {
                matches!(
                    p, Pattern::Ctor(n, _) if { let s = n.to_string(); s == "Sum.inl"
                    || s == "inl" || s == "Either.left" || s == "Left" }
                )
            });
            let has_inr = patterns.iter().any(|p| {
                matches!(
                    p, Pattern::Ctor(n, _) if { let s = n.to_string(); s == "Sum.inr"
                    || s == "inr" || s == "Either.right" || s == "Right" }
                )
            });
            if has_inl && has_inr {
                ExhaustivenessResult::Exhaustive
            } else if !has_inl {
                ExhaustivenessResult::Missing(vec![Pattern::Ctor(
                    Name::str("Sum.inl"),
                    vec![Pattern::Wild],
                )])
            } else {
                ExhaustivenessResult::Missing(vec![Pattern::Ctor(
                    Name::str("Sum.inr"),
                    vec![Pattern::Wild],
                )])
            }
        }
        _ => ExhaustivenessResult::Exhaustive,
    }
}
/// Flatten nested or-patterns into a list of alternatives.
///
/// `p1 | (p2 | p3)` becomes `[p1, p2, p3]`.
pub fn flatten_or(pat: &Pattern) -> Vec<Pattern> {
    match pat {
        Pattern::Or(p1, p2) => {
            let mut result = flatten_or(p1);
            result.extend(flatten_or(p2));
            result
        }
        _ => vec![pat.clone()],
    }
}
/// Check if two patterns overlap (could match the same term).
///
/// Returns `true` if there exists a term that both patterns match.
pub fn patterns_overlap(p1: &Pattern, p2: &Pattern) -> bool {
    match (p1, p2) {
        (Pattern::Wild, _) | (_, Pattern::Wild) => true,
        (Pattern::Var(_), _) | (_, Pattern::Var(_)) => true,
        (Pattern::Ctor(n1, _), Pattern::Ctor(n2, _)) => n1 == n2,
        (Pattern::Lit(l1), Pattern::Lit(l2)) => l1 == l2,
        (Pattern::Or(a, b), other) | (other, Pattern::Or(a, b)) => {
            patterns_overlap(a, other) || patterns_overlap(b, other)
        }
        (Pattern::As(_, inner), other) | (other, Pattern::As(_, inner)) => {
            patterns_overlap(inner, other)
        }
        _ => false,
    }
}
/// Create a constructor pattern with fresh variable sub-patterns.
///
/// Given a constructor name and arity, creates `Ctor(name, [Var(x0), ..., Var(xn)])`.
pub fn mk_ctor_pattern(name: Name, arity: usize) -> Pattern {
    let vars: Vec<Pattern> = (0..arity)
        .map(|i| Pattern::Var(Name::str(format!("x{}", i))))
        .collect();
    Pattern::Ctor(name, vars)
}
/// Normalize or-patterns by flattening and deduplicating.
pub fn normalize_or_patterns(pat: &Pattern) -> Pattern {
    let alts = flatten_or(pat);
    if alts.len() == 1 {
        return alts
            .into_iter()
            .next()
            .expect("alts has exactly one element");
    }
    let mut iter = alts.into_iter().rev();
    let mut result = iter.next().expect("alts is non-empty after flatten_or");
    for p in iter {
        result = Pattern::Or(Box::new(p), Box::new(result));
    }
    result
}
/// Compile a set of equations into a function body using a decision tree.
///
/// This is the main entry point for the equation compiler.
pub fn compile_function(equations: Vec<Equation>) -> Result<DecisionTree, String> {
    if equations.is_empty() {
        return Ok(DecisionTree::Fail);
    }
    let compiler = EquationCompiler::new(equations);
    compiler.compile()
}
#[cfg(test)]
mod extended_equation_tests {
    use super::*;
    use crate::equation::*;
    use oxilean_kernel::Literal;
    fn nat_const() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    #[test]
    fn test_equation_new() {
        let eq = Equation::new(vec![Pattern::Wild], nat_const());
        assert_eq!(eq.arity(), 1);
        assert!(!eq.has_guard());
    }
    #[test]
    fn test_equation_with_guard() {
        let eq = Equation::new(vec![Pattern::Wild], nat_const()).with_guard(nat_const());
        assert!(eq.has_guard());
    }
    #[test]
    fn test_equation_consume_first() {
        let eq = Equation::new(
            vec![Pattern::Wild, Pattern::Var(Name::str("x"))],
            nat_const(),
        );
        let (first, rest) = eq.consume_first().expect("test operation should succeed");
        assert!(matches!(first, Pattern::Wild));
        assert_eq!(rest.arity(), 1);
    }
    #[test]
    fn test_pattern_depth_zero() {
        assert_eq!(Pattern::Wild.depth(), 0);
        assert_eq!(Pattern::Var(Name::str("x")).depth(), 0);
        assert_eq!(Pattern::Lit(Literal::Nat(42)).depth(), 0);
    }
    #[test]
    fn test_pattern_depth_ctor() {
        let p = Pattern::Ctor(Name::str("Succ"), vec![Pattern::Wild]);
        assert_eq!(p.depth(), 1);
        let nested = Pattern::Ctor(Name::str("Succ"), vec![p]);
        assert_eq!(nested.depth(), 2);
    }
    #[test]
    fn test_pattern_count_ctors() {
        let p = Pattern::Ctor(
            Name::str("Pair"),
            vec![Pattern::Ctor(Name::str("Zero"), vec![]), Pattern::Wild],
        );
        assert_eq!(p.count_ctors(), 2);
    }
    #[test]
    fn test_pattern_matches_ctor() {
        let p = Pattern::Ctor(Name::str("Some"), vec![Pattern::Wild]);
        assert!(p.matches_ctor(&Name::str("Some")));
        assert!(!p.matches_ctor(&Name::str("None")));
    }
    #[test]
    fn test_flatten_or() {
        let p = Pattern::Or(
            Box::new(Pattern::Var(Name::str("x"))),
            Box::new(Pattern::Or(
                Box::new(Pattern::Wild),
                Box::new(Pattern::Var(Name::str("y"))),
            )),
        );
        let flat = flatten_or(&p);
        assert_eq!(flat.len(), 3);
    }
    #[test]
    fn test_patterns_overlap_wilds() {
        assert!(patterns_overlap(&Pattern::Wild, &Pattern::Wild));
        assert!(patterns_overlap(
            &Pattern::Wild,
            &Pattern::Var(Name::str("x"))
        ));
    }
    #[test]
    fn test_patterns_overlap_different_ctors() {
        let some = Pattern::Ctor(Name::str("Some"), vec![Pattern::Wild]);
        let none = Pattern::Ctor(Name::str("None"), vec![]);
        assert!(!patterns_overlap(&some, &none));
    }
    #[test]
    fn test_mk_ctor_pattern() {
        let p = mk_ctor_pattern(Name::str("Pair"), 2);
        assert!(matches!(p, Pattern::Ctor(_, pats) if pats.len() == 2));
    }
    #[test]
    fn test_decision_tree_leaves() {
        let leaf = DecisionTree::Leaf(nat_const());
        assert_eq!(leaf.num_leaves(), 1);
        assert_eq!(leaf.depth(), 0);
        assert!(leaf.is_exhaustive());
    }
    #[test]
    fn test_decision_tree_fail() {
        let fail = DecisionTree::Fail;
        assert_eq!(fail.num_leaves(), 1);
        assert!(!fail.is_exhaustive());
    }
    #[test]
    fn test_decision_tree_switch_depth() {
        let tree = DecisionTree::Switch {
            var: 0,
            cases: vec![
                (Name::str("Zero"), DecisionTree::Leaf(nat_const())),
                (Name::str("Succ"), DecisionTree::Leaf(nat_const())),
            ],
            default: None,
        };
        assert_eq!(tree.depth(), 1);
        assert_eq!(tree.num_leaves(), 2);
    }
    #[test]
    fn test_pattern_matrix_operations() {
        let mut matrix = PatternMatrix::new();
        let eq = Equation::new(vec![Pattern::Wild, Pattern::Wild], nat_const());
        matrix.add_row(eq);
        assert_eq!(matrix.num_rows(), 1);
        assert_eq!(matrix.num_cols(), 2);
    }
    #[test]
    fn test_pattern_matrix_first_ctor_column() {
        let mut matrix = PatternMatrix::new();
        let eq = Equation::new(
            vec![Pattern::Wild, Pattern::Ctor(Name::str("Zero"), vec![])],
            nat_const(),
        );
        matrix.add_row(eq);
        assert_eq!(matrix.first_ctor_column(), Some(1));
    }
    #[test]
    fn test_wildcards_to_vars() {
        let pat = Pattern::Ctor(Name::str("Pair"), vec![Pattern::Wild, Pattern::Wild]);
        let mut counter = 0u32;
        let renamed = pat.wildcards_to_vars(&mut counter);
        match &renamed {
            Pattern::Ctor(_, pats) => {
                assert!(pats.iter().all(|p| matches!(p, Pattern::Var(_))));
            }
            _ => panic!("expected ctor"),
        }
        assert_eq!(counter, 2);
    }
    #[test]
    fn test_check_exhaustiveness_empty() {
        let result = check_exhaustiveness(&[], &Name::str("Nat"));
        assert!(matches!(result, ExhaustivenessResult::Missing(_)));
    }
    #[test]
    fn test_check_exhaustiveness_wildcard() {
        let result = check_exhaustiveness(&[Pattern::Wild], &Name::str("Nat"));
        assert_eq!(result, ExhaustivenessResult::Exhaustive);
    }
    #[test]
    fn test_compile_function_single_leaf() {
        let eq = Equation::new(vec![], nat_const());
        let tree = compile_function(vec![eq]).expect("test operation should succeed");
        assert!(matches!(tree, DecisionTree::Leaf(_)));
    }
}
/// Coverage analysis: find uncovered constructor patterns.
///
/// Given a type name and the set of handled constructors, returns the names
/// of constructors that have no matching pattern.
pub fn find_uncovered_ctors(handled: &[Name], all_ctors: &[Name]) -> Vec<Name> {
    all_ctors
        .iter()
        .filter(|c| !handled.contains(c))
        .cloned()
        .collect()
}
/// Group equations by the constructor of their first pattern.
///
/// Returns a map from constructor name to the equations that start with that constructor.
/// Equations with irrefutable first patterns are placed in the `default` group.
pub fn group_by_ctor(equations: &[Equation]) -> (Vec<(Name, Vec<Equation>)>, Vec<Equation>) {
    let mut ctor_map: Vec<(Name, Vec<Equation>)> = Vec::new();
    let mut default_group: Vec<Equation> = Vec::new();
    for eq in equations {
        match eq.patterns.first() {
            Some(Pattern::Ctor(name, _)) => {
                if let Some((_, group)) = ctor_map.iter_mut().find(|(n, _)| n == name) {
                    group.push(eq.clone());
                } else {
                    ctor_map.push((name.clone(), vec![eq.clone()]));
                }
            }
            Some(Pattern::Wild) | Some(Pattern::Var(_)) | None => {
                default_group.push(eq.clone());
            }
            _ => default_group.push(eq.clone()),
        }
    }
    (ctor_map, default_group)
}
/// Specialize a set of equations for a given constructor.
///
/// If the first pattern is `Ctor(name, sub_pats)`, the sub-patterns are
/// prepended to the remaining patterns.
pub fn specialize_for_ctor(equations: &[Equation], ctor_name: &Name) -> Vec<Equation> {
    equations
        .iter()
        .filter_map(|eq| match eq.patterns.first() {
            Some(Pattern::Ctor(n, sub_pats)) if n == ctor_name => {
                let mut new_pats = sub_pats.clone();
                new_pats.extend_from_slice(&eq.patterns[1..]);
                Some(Equation {
                    patterns: new_pats,
                    rhs: eq.rhs.clone(),
                    guard: eq.guard.clone(),
                    source_loc: eq.source_loc,
                })
            }
            Some(Pattern::Wild) | Some(Pattern::Var(_)) => Some(Equation {
                patterns: eq.patterns[1..].to_vec(),
                rhs: eq.rhs.clone(),
                guard: eq.guard.clone(),
                source_loc: eq.source_loc,
            }),
            _ => None,
        })
        .collect()
}
/// Count the number of distinct constructor names appearing as
/// first patterns across all equations.
pub fn count_head_ctors(equations: &[Equation]) -> usize {
    let mut seen: Vec<Name> = Vec::new();
    for eq in equations {
        if let Some(Pattern::Ctor(n, _)) = eq.patterns.first() {
            if !seen.contains(n) {
                seen.push(n.clone());
            }
        }
    }
    seen.len()
}
#[cfg(test)]
mod extra_equation_tests {
    use super::*;
    use crate::equation::*;
    fn nat_const() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    #[test]
    fn test_find_uncovered_ctors_none_missing() {
        let handled = vec![Name::str("Zero"), Name::str("Succ")];
        let all = vec![Name::str("Zero"), Name::str("Succ")];
        let uncovered = find_uncovered_ctors(&handled, &all);
        assert!(uncovered.is_empty());
    }
    #[test]
    fn test_find_uncovered_ctors_some_missing() {
        let handled = vec![Name::str("Zero")];
        let all = vec![Name::str("Zero"), Name::str("Succ")];
        let uncovered = find_uncovered_ctors(&handled, &all);
        assert_eq!(uncovered.len(), 1);
        assert_eq!(uncovered[0], Name::str("Succ"));
    }
    #[test]
    fn test_group_by_ctor_basic() {
        let eq1 = Equation::new(vec![Pattern::Ctor(Name::str("Zero"), vec![])], nat_const());
        let eq2 = Equation::new(
            vec![Pattern::Ctor(Name::str("Succ"), vec![Pattern::Wild])],
            nat_const(),
        );
        let eq3 = Equation::new(vec![Pattern::Wild], nat_const());
        let (groups, defaults) = group_by_ctor(&[eq1, eq2, eq3]);
        assert_eq!(groups.len(), 2);
        assert_eq!(defaults.len(), 1);
    }
    #[test]
    fn test_specialize_for_ctor_removes_head() {
        let eq = Equation::new(
            vec![Pattern::Ctor(
                Name::str("Succ"),
                vec![Pattern::Var(Name::str("n"))],
            )],
            nat_const(),
        );
        let specialized = specialize_for_ctor(&[eq], &Name::str("Succ"));
        assert_eq!(specialized.len(), 1);
        assert_eq!(specialized[0].arity(), 1);
    }
    #[test]
    fn test_count_head_ctors() {
        let eq1 = Equation::new(vec![Pattern::Ctor(Name::str("Zero"), vec![])], nat_const());
        let eq2 = Equation::new(
            vec![Pattern::Ctor(Name::str("Succ"), vec![Pattern::Wild])],
            nat_const(),
        );
        let eq3 = Equation::new(vec![Pattern::Ctor(Name::str("Zero"), vec![])], nat_const());
        assert_eq!(count_head_ctors(&[eq1, eq2, eq3]), 2);
    }
}
/// Redundancy analysis: return the indices of equations that can never match
/// because all their cases are already covered by earlier equations.
///
/// This is a conservative approximation: we only detect rows that are
/// syntactically dominated by an earlier irrefutable row.
#[allow(dead_code)]
pub fn find_redundant_equations(equations: &[Equation]) -> Vec<usize> {
    let mut redundant = Vec::new();
    let mut has_catchall = false;
    for (i, eq) in equations.iter().enumerate() {
        if has_catchall {
            redundant.push(i);
        }
        if eq
            .patterns
            .iter()
            .all(|p| matches!(p, Pattern::Wild | Pattern::Var(_)))
        {
            has_catchall = true;
        }
    }
    redundant
}
/// Check whether two patterns are orthogonal (can never match the same value).
///
/// Two constructor patterns with different names are orthogonal.
/// Literals with different values are orthogonal.
/// Wild/Var patterns are never orthogonal to anything.
#[allow(dead_code)]
pub fn patterns_orthogonal(p1: &Pattern, p2: &Pattern) -> bool {
    match (p1, p2) {
        (Pattern::Ctor(n1, _), Pattern::Ctor(n2, _)) => n1 != n2,
        (Pattern::Lit(l1), Pattern::Lit(l2)) => l1 != l2,
        (Pattern::Wild, _) | (_, Pattern::Wild) => false,
        (Pattern::Var(_), _) | (_, Pattern::Var(_)) => false,
        _ => false,
    }
}
/// Estimate the maximum depth of nesting in a pattern.
#[allow(dead_code)]
pub fn pattern_depth(p: &Pattern) -> usize {
    match p {
        Pattern::Wild | Pattern::Var(_) | Pattern::Lit(_) => 0,
        Pattern::Ctor(_, sub) => 1 + sub.iter().map(pattern_depth).max().unwrap_or(0),
        Pattern::As(_, inner) => 1 + pattern_depth(inner),
        Pattern::Or(l, r) => 1 + pattern_depth(l).max(pattern_depth(r)),
    }
}
/// Count the total number of leaf patterns (Wild, Var, Lit) in a pattern tree.
#[allow(dead_code)]
pub fn count_leaf_patterns(p: &Pattern) -> usize {
    match p {
        Pattern::Wild | Pattern::Var(_) | Pattern::Lit(_) => 1,
        Pattern::Ctor(_, sub) => sub.iter().map(count_leaf_patterns).sum(),
        Pattern::As(_, inner) => count_leaf_patterns(inner),
        Pattern::Or(l, r) => count_leaf_patterns(l) + count_leaf_patterns(r),
    }
}
#[cfg(test)]
mod extra2_equation_tests {
    use super::*;
    use crate::equation::*;
    fn nat_expr() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    #[test]
    fn test_pattern_row_irrefutable() {
        let row = PatternRow::new(
            vec![Pattern::Wild, Pattern::Var(Name::str("x"))],
            nat_expr(),
        );
        assert!(row.is_irrefutable());
    }
    #[test]
    fn test_pattern_row_not_irrefutable() {
        let row = PatternRow::new(vec![Pattern::Ctor(Name::str("Zero"), vec![])], nat_expr());
        assert!(!row.is_irrefutable());
    }
    #[test]
    fn test_pattern_row_first_ctor_col() {
        let row = PatternRow::new(
            vec![Pattern::Wild, Pattern::Ctor(Name::str("Succ"), vec![])],
            nat_expr(),
        );
        assert_eq!(row.first_ctor_column(), Some(1));
    }
    #[test]
    fn test_pattern_matrix_from_equations() {
        let eqs = vec![
            Equation::new(vec![Pattern::Wild], nat_expr()),
            Equation::new(vec![Pattern::Ctor(Name::str("Zero"), vec![])], nat_expr()),
        ];
        let m = PatternMatrix2::from_equations(&eqs);
        assert_eq!(m.height(), 2);
        assert_eq!(m.num_columns, 1);
    }
    #[test]
    fn test_pattern_matrix_ctor_columns() {
        let eqs = vec![Equation::new(
            vec![Pattern::Wild, Pattern::Ctor(Name::str("Zero"), vec![])],
            nat_expr(),
        )];
        let m = PatternMatrix2::from_equations(&eqs);
        let ctor_cols = m.ctor_columns();
        assert!(ctor_cols.contains(&1));
        assert!(!ctor_cols.contains(&0));
    }
    #[test]
    fn test_find_redundant_equations() {
        let eqs = vec![
            Equation::new(vec![Pattern::Wild], nat_expr()),
            Equation::new(vec![Pattern::Ctor(Name::str("Zero"), vec![])], nat_expr()),
        ];
        let redundant = find_redundant_equations(&eqs);
        assert_eq!(redundant, vec![1]);
    }
    #[test]
    fn test_patterns_orthogonal_ctors() {
        let p1 = Pattern::Ctor(Name::str("Zero"), vec![]);
        let p2 = Pattern::Ctor(Name::str("Succ"), vec![Pattern::Wild]);
        assert!(patterns_orthogonal(&p1, &p2));
    }
    #[test]
    fn test_patterns_orthogonal_same_ctor() {
        let p1 = Pattern::Ctor(Name::str("Zero"), vec![]);
        let p2 = Pattern::Ctor(Name::str("Zero"), vec![]);
        assert!(!patterns_orthogonal(&p1, &p2));
    }
    #[test]
    fn test_patterns_orthogonal_wild() {
        let p1 = Pattern::Wild;
        let p2 = Pattern::Ctor(Name::str("Zero"), vec![]);
        assert!(!patterns_orthogonal(&p1, &p2));
    }
    #[test]
    fn test_pattern_depth_flat() {
        let p = Pattern::Ctor(Name::str("Succ"), vec![Pattern::Wild]);
        assert_eq!(pattern_depth(&p), 1);
    }
    #[test]
    fn test_pattern_depth_nested() {
        let inner = Pattern::Ctor(Name::str("Succ"), vec![Pattern::Wild]);
        let outer = Pattern::Ctor(Name::str("Succ"), vec![inner]);
        assert_eq!(pattern_depth(&outer), 2);
    }
    #[test]
    fn test_count_leaf_patterns() {
        let p = Pattern::Ctor(
            Name::str("Succ"),
            vec![Pattern::Wild, Pattern::Var(Name::str("n"))],
        );
        assert_eq!(count_leaf_patterns(&p), 2);
    }
    #[test]
    fn test_ctors_in_column() {
        let eqs = vec![
            Equation::new(vec![Pattern::Ctor(Name::str("Zero"), vec![])], nat_expr()),
            Equation::new(
                vec![Pattern::Ctor(Name::str("Succ"), vec![Pattern::Wild])],
                nat_expr(),
            ),
            Equation::new(vec![Pattern::Ctor(Name::str("Zero"), vec![])], nat_expr()),
        ];
        let m = PatternMatrix2::from_equations(&eqs);
        let ctors = m.ctors_in_column(0);
        assert_eq!(ctors.len(), 2);
    }
    #[test]
    fn test_pattern_matrix_is_empty() {
        let m = PatternMatrix2::new(1);
        assert!(m.is_empty());
    }
    #[test]
    fn test_pattern_row_with_guard() {
        let rhs = nat_expr();
        let guard = Expr::Const(Name::str("True"), vec![]);
        let row = PatternRow::new(vec![Pattern::Wild], rhs).with_guard(guard.clone());
        assert_eq!(row.guard, Some(guard));
    }
}
#[allow(dead_code)]
pub fn pattern_complexity(p: &Pattern) -> u32 {
    match p {
        Pattern::Wild | Pattern::Var(_) => 1,
        Pattern::Ctor(_, args) => 1 + args.iter().map(pattern_complexity).sum::<u32>(),
        Pattern::Lit(_) => 1,
        Pattern::Or(a, b) => pattern_complexity(a) + pattern_complexity(b),
        Pattern::As(_, inner) => 1 + pattern_complexity(inner),
    }
}
#[allow(dead_code)]
pub fn equation_complexity(eq: &Equation) -> u32 {
    eq.patterns.iter().map(pattern_complexity).sum()
}
#[allow(dead_code)]
pub fn sort_equations_by_complexity(equations: &mut [Equation]) {
    equations.sort_by_key(equation_complexity);
}
#[allow(dead_code)]
pub fn annotate_pattern(p: &Pattern) -> PatternAnnotation {
    match p {
        Pattern::Wild | Pattern::Var(_) => {
            PatternAnnotation::new(PatternAnnotationKind::Irrefutable)
        }
        Pattern::Ctor(_, args)
            if args
                .iter()
                .all(|a| matches!(a, Pattern::Wild | Pattern::Var(_))) =>
        {
            PatternAnnotation::new(PatternAnnotationKind::Refutable)
        }
        Pattern::Ctor(_, _) => PatternAnnotation::new(PatternAnnotationKind::Nested),
        Pattern::Lit(_) => PatternAnnotation::new(PatternAnnotationKind::Refutable),
        Pattern::Or(_, _) => PatternAnnotation::new(PatternAnnotationKind::Refutable),
        Pattern::As(_, inner) => annotate_pattern(inner),
    }
}
#[allow(dead_code)]
fn nat_expr_eq() -> Expr {
    Expr::Const(Name::str("Nat"), vec![])
}
#[cfg(test)]
mod equation_extended_tests {
    use super::*;
    use crate::equation::*;
    fn make_eq(patterns: Vec<Pattern>) -> Equation {
        Equation {
            patterns,
            rhs: nat_expr_eq(),
            guard: None,
            source_loc: None,
        }
    }
    #[test]
    fn test_decision_tree_stats_leaf() {
        let tree = DecisionTree::Leaf(nat_expr_eq());
        let stats = DecisionTreeStats::analyze(&tree);
        assert_eq!(stats.leaf_count, 1);
        assert_eq!(stats.total_nodes(), 1);
        assert!(stats.is_simple());
    }
    #[test]
    fn test_decision_tree_stats_switch() {
        let tree = DecisionTree::Switch {
            var: 0,
            cases: vec![
                (Name::str("zero"), DecisionTree::Leaf(nat_expr_eq())),
                (Name::str("succ"), DecisionTree::Leaf(nat_expr_eq())),
            ],
            default: Some(Box::new(DecisionTree::Fail)),
        };
        let stats = DecisionTreeStats::analyze(&tree);
        assert_eq!(stats.switch_count, 1);
        assert_eq!(stats.leaf_count, 2);
        assert_eq!(stats.fail_count, 1);
    }
    #[test]
    fn test_pattern_complexity() {
        let wild = Pattern::Wild;
        assert_eq!(pattern_complexity(&wild), 1);
        let ctor = Pattern::Ctor(Name::str("succ"), vec![Pattern::Wild]);
        assert_eq!(pattern_complexity(&ctor), 2);
    }
    #[test]
    fn test_equation_set_has_wildcards() {
        let eq = make_eq(vec![Pattern::Wild, Pattern::Var(Name::str("x"))]);
        let set = EquationSet::new(vec![eq]);
        assert!(set.has_wildcards_only());
        assert_eq!(set.arity(), Some(2));
        assert!(!set.has_guards());
    }
    #[test]
    fn test_equation_set_has_guards() {
        let mut eq = make_eq(vec![Pattern::Wild]);
        eq.guard = Some(nat_expr_eq());
        let set = EquationSet::new(vec![eq]);
        assert!(set.has_guards());
    }
    #[test]
    fn test_pattern_annotation() {
        let ann = annotate_pattern(&Pattern::Wild);
        assert!(ann.is_irrefutable());
        let ann2 = annotate_pattern(&Pattern::Lit(oxilean_kernel::Literal::Nat(0)));
        assert_eq!(ann2.kind, PatternAnnotationKind::Refutable);
    }
    #[test]
    fn test_equation_compiler_config() {
        let cfg = EquationCompilerConfig::new().permissive();
        assert!(cfg.allow_non_exhaustive);
        assert!(!cfg.check_redundancy);
        let strict = EquationCompilerConfig::new().strict();
        assert!(!strict.allow_non_exhaustive);
        assert!(strict.check_redundancy);
    }
    #[test]
    fn test_equation_compilation_report() {
        let mut report = EquationCompilationReport::new(3);
        report.redundant_indices.push(2);
        report.add_warning("pattern 2 is redundant");
        assert!(report.has_redundancy());
        assert!(report.is_exhaustive());
        assert_eq!(report.warning_count(), 1);
    }
}
#[cfg(test)]
mod matrix_tests {
    use super::*;
    use crate::equation::*;
    fn make_wild_eq(n: usize) -> Equation {
        Equation {
            patterns: vec![Pattern::Wild; n],
            guard: None,
            rhs: Expr::Sort(oxilean_kernel::Level::zero()),
            source_loc: None,
        }
    }
    fn make_ctor_eq(ctor: &str, n_args: usize) -> Equation {
        let args = vec![Pattern::Wild; n_args];
        Equation {
            patterns: vec![Pattern::Ctor(Name::str(ctor), args)],
            guard: None,
            rhs: Expr::Sort(oxilean_kernel::Level::zero()),
            source_loc: None,
        }
    }
    #[test]
    fn test_pattern_matrix_from_equations() {
        let eqs = vec![make_ctor_eq("zero", 0), make_wild_eq(1)];
        let matrix =
            FullPatternMatrix::from_equations(&eqs).expect("test operation should succeed");
        assert_eq!(matrix.arity(), 1);
        assert_eq!(matrix.num_rows(), 2);
    }
    #[test]
    fn test_specialize() {
        let eqs = vec![make_ctor_eq("succ", 1), make_wild_eq(1)];
        let matrix =
            FullPatternMatrix::from_equations(&eqs).expect("test operation should succeed");
        let spec = matrix.specialize(&Name::str("succ"), 1);
        assert_eq!(spec.num_rows(), 2);
    }
    #[test]
    fn test_default_matrix() {
        let eqs = vec![make_ctor_eq("zero", 0), make_wild_eq(1)];
        let matrix =
            FullPatternMatrix::from_equations(&eqs).expect("test operation should succeed");
        let def = matrix.default_matrix();
        assert_eq!(def.num_rows(), 1);
    }
    #[test]
    fn test_column_selector_left_to_right() {
        let eqs = vec![make_wild_eq(3)];
        let matrix =
            FullPatternMatrix::from_equations(&eqs).expect("test operation should succeed");
        let sel = ColumnSelector::new(ColumnHeuristic::LeftToRight);
        assert_eq!(sel.select(&matrix), 0);
    }
    #[test]
    fn test_overlap_checker_subsumes() {
        assert!(OverlapChecker::subsumes(
            &Pattern::Wild,
            &Pattern::Lit(oxilean_kernel::Literal::Nat(0))
        ));
        assert!(!OverlapChecker::subsumes(
            &Pattern::Lit(oxilean_kernel::Literal::Nat(0)),
            &Pattern::Wild
        ));
        assert!(OverlapChecker::subsumes(
            &Pattern::Lit(oxilean_kernel::Literal::Nat(5)),
            &Pattern::Lit(oxilean_kernel::Literal::Nat(5))
        ));
    }
    #[test]
    fn test_exhaustiveness_simple() {
        let analyzer = ExhaustivenessAnalyzer::new(10);
        let eqs = vec![make_wild_eq(1)];
        let result = analyzer.check_simple(&eqs);
        assert_eq!(result, ExhaustCheck::Exhaustive);
    }
    #[test]
    fn test_exhaustiveness_non_exhaustive() {
        let analyzer = ExhaustivenessAnalyzer::new(10);
        let eqs = vec![make_ctor_eq("zero", 0)];
        let result = analyzer.check_simple(&eqs);
        assert!(matches!(result, ExhaustCheck::NonExhaustive(_)));
    }
    #[test]
    fn test_normalizer_desugar_or() {
        let eq = Equation {
            patterns: vec![Pattern::Or(
                Box::new(Pattern::Lit(oxilean_kernel::Literal::Nat(0))),
                Box::new(Pattern::Lit(oxilean_kernel::Literal::Nat(1))),
            )],
            guard: None,
            rhs: Expr::Sort(oxilean_kernel::Level::zero()),
            source_loc: None,
        };
        let norm = EquationNormalizer::new();
        let result = norm.normalize(&[eq]);
        assert_eq!(result.len(), 2);
    }
    #[test]
    fn test_head_constructors() {
        let eqs = vec![
            make_ctor_eq("zero", 0),
            make_ctor_eq("succ", 1),
            make_ctor_eq("zero", 0),
        ];
        let matrix =
            FullPatternMatrix::from_equations(&eqs).expect("test operation should succeed");
        let ctors = matrix.head_constructors();
        assert_eq!(ctors.len(), 2);
    }
    #[test]
    fn test_overlap_checker_redundant_pairs() {
        let checker = OverlapChecker::new();
        let eqs = vec![make_wild_eq(1), make_wild_eq(1)];
        let pairs = checker.find_redundant_pairs(&eqs);
        assert!(!pairs.is_empty());
    }
}
#[allow(dead_code)]
pub fn equation_extension_version() -> &'static str {
    "oxilean-elab-equation-extension-v1"
}
