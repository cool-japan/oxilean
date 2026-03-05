//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    ColumnHeuristic, ColumnScore, CoverageAnalysis, DecisionBranch, DecisionDag, DecisionTree,
    DecisionTreeStats, ExhaustivenessInfo, ExhaustivenessReport, ExtendedTreeStats, GuardedArm,
    JumpTable, JumpTableEntry, MatchEquation, PatRow, TreeStats,
};
use crate::match_basic::{MetaMatchArm, MetaPattern};
use oxilean_kernel::{Expr, Name};

/// Build a decision tree from match arms.
///
/// The `num_columns` parameter indicates how many discriminants there are.
/// Each arm must have exactly `num_columns` patterns.
pub fn build_decision_tree(arms: &[MetaMatchArm], num_columns: usize) -> DecisionTree {
    let rows: Vec<PatRow> = arms
        .iter()
        .enumerate()
        .map(|(i, arm)| PatRow {
            patterns: arm.patterns.clone(),
            arm_idx: i,
            rhs: arm.rhs.clone(),
        })
        .collect();
    compile_matrix(&rows, num_columns, &mut 0)
}
/// Core compilation algorithm.
///
/// Recursively selects a column to split on, partitions the
/// matrix by constructor, and recurses on each sub-matrix.
pub(super) fn compile_matrix(rows: &[PatRow], num_columns: usize, fresh: &mut u64) -> DecisionTree {
    if rows.is_empty() {
        return DecisionTree::Failure;
    }
    if num_columns == 0 || rows[0].patterns.iter().all(|p| p.is_irrefutable()) {
        let bindings = collect_bindings(&rows[0].patterns, fresh);
        return DecisionTree::Leaf {
            arm_idx: rows[0].arm_idx,
            bindings,
        };
    }
    let col = select_column(rows, num_columns);
    let ctors = collect_constructors(rows, col);
    if ctors.is_empty() {
        let reduced_rows: Vec<PatRow> = rows
            .iter()
            .map(|row| {
                let mut pats = row.patterns.clone();
                pats.remove(col);
                PatRow {
                    patterns: pats,
                    arm_idx: row.arm_idx,
                    rhs: row.rhs.clone(),
                }
            })
            .collect();
        return compile_matrix(&reduced_rows, num_columns - 1, fresh);
    }
    let mut branches = Vec::new();
    let mut default_rows: Vec<PatRow> = Vec::new();
    for (ctor_name, num_fields) in &ctors {
        let specialized = specialize_matrix(rows, col, ctor_name, *num_fields, fresh);
        let new_cols = num_columns - 1 + (*num_fields as usize);
        let subtree = compile_matrix(&specialized, new_cols, fresh);
        let field_names: Vec<Name> = (0..*num_fields)
            .map(|_i| {
                *fresh += 1;
                Name::str(format!("_field_{}", fresh))
            })
            .collect();
        branches.push(DecisionBranch {
            ctor_name: ctor_name.clone(),
            num_fields: *num_fields,
            field_names,
            subtree,
        });
    }
    for row in rows {
        let pat = row.patterns.get(col);
        if pat.map(|p| p.is_irrefutable()).unwrap_or(true) {
            let mut pats = row.patterns.clone();
            pats.remove(col);
            default_rows.push(PatRow {
                patterns: pats,
                arm_idx: row.arm_idx,
                rhs: row.rhs.clone(),
            });
        }
    }
    let default = if default_rows.is_empty() {
        None
    } else {
        Some(Box::new(compile_matrix(
            &default_rows,
            num_columns - 1,
            fresh,
        )))
    };
    DecisionTree::Switch {
        column: col,
        branches,
        default,
    }
}
/// Select the best column to split on using the "most discriminating" heuristic.
///
/// Picks the column with the highest number of distinct non-wildcard patterns.
/// This tends to reduce the depth of the resulting decision tree compared to
/// the naive "first non-wildcard column" approach.
pub(super) fn select_column(rows: &[PatRow], num_columns: usize) -> usize {
    let mut best_col = 0;
    let mut best_score = 0usize;
    for col in 0..num_columns {
        let mut seen_ctors: Vec<String> = Vec::new();
        let mut non_wildcard_rows = 0usize;
        for row in rows {
            match row.patterns.get(col) {
                Some(MetaPattern::Constructor(name, _)) => {
                    non_wildcard_rows += 1;
                    let key = name.to_string();
                    if !seen_ctors.contains(&key) {
                        seen_ctors.push(key);
                    }
                }
                Some(MetaPattern::Literal(lit)) => {
                    non_wildcard_rows += 1;
                    let key = format!("{:?}", lit);
                    if !seen_ctors.contains(&key) {
                        seen_ctors.push(key);
                    }
                }
                _ => {}
            }
        }
        let score = seen_ctors.len() * 1000 + non_wildcard_rows;
        if score > best_score {
            best_score = score;
            best_col = col;
        }
    }
    best_col
}
/// Collect distinct constructors in a column.
pub(super) fn collect_constructors(rows: &[PatRow], col: usize) -> Vec<(Name, u32)> {
    let mut seen = Vec::new();
    for row in rows {
        if let Some(MetaPattern::Constructor(name, subpats)) = row.patterns.get(col) {
            if !seen.iter().any(|(n, _): &(Name, u32)| n == name) {
                seen.push((name.clone(), subpats.len() as u32));
            }
        }
    }
    seen
}
/// Specialize a pattern matrix for a given constructor.
///
/// For each row:
/// - If column `col` is `Ctor(name, p₁...pₖ)`, replace it with `p₁...pₖ`
/// - If column `col` is wildcard/var, replace it with `k` wildcards
/// - Otherwise, remove the row
pub(super) fn specialize_matrix(
    rows: &[PatRow],
    col: usize,
    ctor_name: &Name,
    num_fields: u32,
    _fresh: &mut u64,
) -> Vec<PatRow> {
    let mut result = Vec::new();
    for row in rows {
        let pat = row
            .patterns
            .get(col)
            .cloned()
            .unwrap_or(MetaPattern::Wildcard);
        match &pat {
            MetaPattern::Constructor(name, subpats) => {
                if name == ctor_name {
                    let mut new_pats = Vec::new();
                    for i in 0..col {
                        if let Some(p) = row.patterns.get(i) {
                            new_pats.push(p.clone());
                        }
                    }
                    new_pats.extend(subpats.iter().cloned());
                    while new_pats.len() < col + num_fields as usize {
                        new_pats.push(MetaPattern::Wildcard);
                    }
                    for i in (col + 1)..row.patterns.len() {
                        if let Some(p) = row.patterns.get(i) {
                            new_pats.push(p.clone());
                        }
                    }
                    result.push(PatRow {
                        patterns: new_pats,
                        arm_idx: row.arm_idx,
                        rhs: row.rhs.clone(),
                    });
                }
            }
            MetaPattern::Wildcard | MetaPattern::Var(_) => {
                let mut new_pats = Vec::new();
                for i in 0..col {
                    if let Some(p) = row.patterns.get(i) {
                        new_pats.push(p.clone());
                    }
                }
                for _ in 0..num_fields {
                    new_pats.push(MetaPattern::Wildcard);
                }
                for i in (col + 1)..row.patterns.len() {
                    if let Some(p) = row.patterns.get(i) {
                        new_pats.push(p.clone());
                    }
                }
                result.push(PatRow {
                    patterns: new_pats,
                    arm_idx: row.arm_idx,
                    rhs: row.rhs.clone(),
                });
            }
            MetaPattern::Or(left, right) => {
                let mut left_pats = row.patterns.clone();
                left_pats[col] = *left.clone();
                let left_row = PatRow {
                    patterns: left_pats,
                    arm_idx: row.arm_idx,
                    rhs: row.rhs.clone(),
                };
                let mut right_pats = row.patterns.clone();
                right_pats[col] = *right.clone();
                let right_row = PatRow {
                    patterns: right_pats,
                    arm_idx: row.arm_idx,
                    rhs: row.rhs.clone(),
                };
                let left_specialized =
                    specialize_matrix(&[left_row], col, ctor_name, num_fields, _fresh);
                let right_specialized =
                    specialize_matrix(&[right_row], col, ctor_name, num_fields, _fresh);
                result.extend(left_specialized);
                result.extend(right_specialized);
            }
            MetaPattern::As(inner, _) => {
                let mut as_pats = row.patterns.clone();
                as_pats[col] = *inner.clone();
                let as_row = PatRow {
                    patterns: as_pats,
                    arm_idx: row.arm_idx,
                    rhs: row.rhs.clone(),
                };
                let specialized = specialize_matrix(&[as_row], col, ctor_name, num_fields, _fresh);
                result.extend(specialized);
            }
            _ => {}
        }
    }
    result
}
/// Collect variable bindings from a pattern row.
pub(super) fn collect_bindings(patterns: &[MetaPattern], fresh: &mut u64) -> Vec<(Name, Expr)> {
    let mut bindings = Vec::new();
    for (i, pat) in patterns.iter().enumerate() {
        collect_pattern_bindings(pat, i, &mut bindings, fresh);
    }
    bindings
}
/// Collect bindings from a single pattern.
pub(super) fn collect_pattern_bindings(
    pat: &MetaPattern,
    col: usize,
    bindings: &mut Vec<(Name, Expr)>,
    _fresh: &mut u64,
) {
    match pat {
        MetaPattern::Var(name) => {
            bindings.push((name.clone(), Expr::BVar(col as u32)));
        }
        MetaPattern::As(inner, name) => {
            bindings.push((name.clone(), Expr::BVar(col as u32)));
            collect_pattern_bindings(inner, col, bindings, _fresh);
        }
        MetaPattern::Constructor(_, subpats) => {
            for (i, sp) in subpats.iter().enumerate() {
                collect_pattern_bindings(sp, col + i, bindings, _fresh);
            }
        }
        _ => {}
    }
}
/// Generate equations from match arms for the equation compiler.
///
/// Each arm produces an equation: `match_fn p₁ ... pₙ = rhs`.
pub fn generate_equations(arms: &[MetaMatchArm]) -> Vec<MatchEquation> {
    arms.iter()
        .enumerate()
        .map(|(i, arm)| MatchEquation {
            lhs_patterns: arm.patterns.clone(),
            rhs: arm.rhs.clone(),
            arm_idx: i,
        })
        .collect()
}
/// Count the number of leaves in a decision tree.
pub fn count_leaves(tree: &DecisionTree) -> usize {
    match tree {
        DecisionTree::Leaf { .. } => 1,
        DecisionTree::Failure => 0,
        DecisionTree::Switch {
            branches, default, ..
        } => {
            let branch_count: usize = branches.iter().map(|b| count_leaves(&b.subtree)).sum();
            let default_count = default.as_ref().map(|d| count_leaves(d)).unwrap_or(0);
            branch_count + default_count
        }
    }
}
/// Count the depth of a decision tree.
pub fn tree_depth(tree: &DecisionTree) -> usize {
    match tree {
        DecisionTree::Leaf { .. } | DecisionTree::Failure => 0,
        DecisionTree::Switch {
            branches, default, ..
        } => {
            let branch_max = branches
                .iter()
                .map(|b| tree_depth(&b.subtree))
                .max()
                .unwrap_or(0);
            let default_max = default.as_ref().map(|d| tree_depth(d)).unwrap_or(0);
            1 + branch_max.max(default_max)
        }
    }
}
/// Collect all arm indices referenced by leaves in the tree.
pub fn collect_referenced_arms(tree: &DecisionTree) -> Vec<usize> {
    let mut arms = Vec::new();
    collect_arms_impl(tree, &mut arms);
    arms.sort();
    arms.dedup();
    arms
}
pub(super) fn collect_arms_impl(tree: &DecisionTree, arms: &mut Vec<usize>) {
    match tree {
        DecisionTree::Leaf { arm_idx, .. } => {
            arms.push(*arm_idx);
        }
        DecisionTree::Failure => {}
        DecisionTree::Switch {
            branches, default, ..
        } => {
            for branch in branches {
                collect_arms_impl(&branch.subtree, arms);
            }
            if let Some(def) = default {
                collect_arms_impl(def, arms);
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::match_dectree::*;
    use oxilean_kernel::Literal;
    fn mk_ctor(name: &str, subpats: Vec<MetaPattern>) -> MetaPattern {
        MetaPattern::Constructor(Name::str(name), subpats)
    }
    fn mk_var(name: &str) -> MetaPattern {
        MetaPattern::Var(Name::str(name))
    }
    fn mk_wild() -> MetaPattern {
        MetaPattern::Wildcard
    }
    fn dummy_rhs(n: u64) -> Expr {
        Expr::Lit(Literal::Nat(n))
    }
    #[test]
    fn test_build_simple_match() {
        let arms = vec![
            MetaMatchArm {
                patterns: vec![mk_ctor("Nat.zero", vec![])],
                guard: None,
                rhs: dummy_rhs(0),
            },
            MetaMatchArm {
                patterns: vec![mk_ctor("Nat.succ", vec![mk_var("m")])],
                guard: None,
                rhs: dummy_rhs(1),
            },
        ];
        let tree = build_decision_tree(&arms, 1);
        match &tree {
            DecisionTree::Switch {
                column, branches, ..
            } => {
                assert_eq!(*column, 0);
                assert_eq!(branches.len(), 2);
                assert_eq!(branches[0].ctor_name, Name::str("Nat.zero"));
                assert_eq!(branches[1].ctor_name, Name::str("Nat.succ"));
            }
            _ => panic!("Expected Switch, got {:?}", tree),
        }
    }
    #[test]
    fn test_build_wildcard_match() {
        let arms = vec![MetaMatchArm {
            patterns: vec![mk_wild()],
            guard: None,
            rhs: dummy_rhs(42),
        }];
        let tree = build_decision_tree(&arms, 1);
        match &tree {
            DecisionTree::Leaf { arm_idx, .. } => {
                assert_eq!(*arm_idx, 0);
            }
            _ => panic!("Expected Leaf, got {:?}", tree),
        }
    }
    #[test]
    fn test_build_var_match() {
        let arms = vec![MetaMatchArm {
            patterns: vec![mk_var("x")],
            guard: None,
            rhs: dummy_rhs(0),
        }];
        let tree = build_decision_tree(&arms, 1);
        match &tree {
            DecisionTree::Leaf { arm_idx, bindings } => {
                assert_eq!(*arm_idx, 0);
                assert_eq!(bindings.len(), 1);
                assert_eq!(bindings[0].0, Name::str("x"));
            }
            _ => panic!("Expected Leaf, got {:?}", tree),
        }
    }
    #[test]
    fn test_build_nested_match() {
        let arms = vec![
            MetaMatchArm {
                patterns: vec![mk_ctor("Nat.zero", vec![])],
                guard: None,
                rhs: dummy_rhs(0),
            },
            MetaMatchArm {
                patterns: vec![mk_ctor("Nat.succ", vec![mk_ctor("Nat.zero", vec![])])],
                guard: None,
                rhs: dummy_rhs(1),
            },
            MetaMatchArm {
                patterns: vec![mk_ctor(
                    "Nat.succ",
                    vec![mk_ctor("Nat.succ", vec![mk_var("m")])],
                )],
                guard: None,
                rhs: dummy_rhs(2),
            },
        ];
        let tree = build_decision_tree(&arms, 3);
        assert!(count_leaves(&tree) > 0);
    }
    #[test]
    fn test_build_with_default() {
        let arms = vec![
            MetaMatchArm {
                patterns: vec![mk_ctor("Nat.zero", vec![])],
                guard: None,
                rhs: dummy_rhs(0),
            },
            MetaMatchArm {
                patterns: vec![mk_wild()],
                guard: None,
                rhs: dummy_rhs(1),
            },
        ];
        let tree = build_decision_tree(&arms, 1);
        match &tree {
            DecisionTree::Switch {
                branches, default, ..
            } => {
                assert_eq!(branches.len(), 1);
                assert!(default.is_some());
            }
            _ => panic!("Expected Switch, got {:?}", tree),
        }
    }
    #[test]
    fn test_empty_arms() {
        let arms: Vec<MetaMatchArm> = vec![];
        let tree = build_decision_tree(&arms, 1);
        assert!(matches!(tree, DecisionTree::Failure));
    }
    #[test]
    fn test_count_leaves() {
        let leaf = DecisionTree::Leaf {
            arm_idx: 0,
            bindings: vec![],
        };
        assert_eq!(count_leaves(&leaf), 1);
        assert_eq!(count_leaves(&DecisionTree::Failure), 0);
        let switch = DecisionTree::Switch {
            column: 0,
            branches: vec![
                DecisionBranch {
                    ctor_name: Name::str("A"),
                    num_fields: 0,
                    field_names: vec![],
                    subtree: DecisionTree::Leaf {
                        arm_idx: 0,
                        bindings: vec![],
                    },
                },
                DecisionBranch {
                    ctor_name: Name::str("B"),
                    num_fields: 0,
                    field_names: vec![],
                    subtree: DecisionTree::Leaf {
                        arm_idx: 1,
                        bindings: vec![],
                    },
                },
            ],
            default: None,
        };
        assert_eq!(count_leaves(&switch), 2);
    }
    #[test]
    fn test_tree_depth() {
        let leaf = DecisionTree::Leaf {
            arm_idx: 0,
            bindings: vec![],
        };
        assert_eq!(tree_depth(&leaf), 0);
        let switch = DecisionTree::Switch {
            column: 0,
            branches: vec![DecisionBranch {
                ctor_name: Name::str("A"),
                num_fields: 0,
                field_names: vec![],
                subtree: DecisionTree::Leaf {
                    arm_idx: 0,
                    bindings: vec![],
                },
            }],
            default: None,
        };
        assert_eq!(tree_depth(&switch), 1);
    }
    #[test]
    fn test_collect_referenced_arms() {
        let tree = DecisionTree::Switch {
            column: 0,
            branches: vec![
                DecisionBranch {
                    ctor_name: Name::str("A"),
                    num_fields: 0,
                    field_names: vec![],
                    subtree: DecisionTree::Leaf {
                        arm_idx: 0,
                        bindings: vec![],
                    },
                },
                DecisionBranch {
                    ctor_name: Name::str("B"),
                    num_fields: 0,
                    field_names: vec![],
                    subtree: DecisionTree::Leaf {
                        arm_idx: 2,
                        bindings: vec![],
                    },
                },
            ],
            default: Some(Box::new(DecisionTree::Leaf {
                arm_idx: 1,
                bindings: vec![],
            })),
        };
        let arms = collect_referenced_arms(&tree);
        assert_eq!(arms, vec![0, 1, 2]);
    }
    #[test]
    fn test_generate_equations() {
        let arms = vec![
            MetaMatchArm {
                patterns: vec![mk_ctor("Nat.zero", vec![])],
                guard: None,
                rhs: dummy_rhs(0),
            },
            MetaMatchArm {
                patterns: vec![mk_ctor("Nat.succ", vec![mk_var("n")])],
                guard: None,
                rhs: dummy_rhs(1),
            },
        ];
        let eqs = generate_equations(&arms);
        assert_eq!(eqs.len(), 2);
        assert_eq!(eqs[0].arm_idx, 0);
        assert_eq!(eqs[1].arm_idx, 1);
    }
    #[test]
    fn test_collect_constructors() {
        let rows = vec![
            PatRow {
                patterns: vec![mk_ctor("Nat.zero", vec![])],
                arm_idx: 0,
                rhs: dummy_rhs(0),
            },
            PatRow {
                patterns: vec![mk_ctor("Nat.succ", vec![mk_wild()])],
                arm_idx: 1,
                rhs: dummy_rhs(1),
            },
            PatRow {
                patterns: vec![mk_wild()],
                arm_idx: 2,
                rhs: dummy_rhs(2),
            },
        ];
        let ctors = collect_constructors(&rows, 0);
        assert_eq!(ctors.len(), 2);
        assert_eq!(ctors[0].0, Name::str("Nat.zero"));
        assert_eq!(ctors[0].1, 0);
        assert_eq!(ctors[1].0, Name::str("Nat.succ"));
        assert_eq!(ctors[1].1, 1);
    }
}
/// Compute statistics for a decision tree.
pub fn compute_tree_stats(tree: &DecisionTree) -> DecisionTreeStats {
    let mut stats = DecisionTreeStats {
        num_nodes: 0,
        max_depth: 0,
        num_leaves: 0,
        num_failures: 0,
        num_arms_referenced: 0,
    };
    let mut arm_set = std::collections::HashSet::new();
    compute_stats_impl(tree, 0, &mut stats, &mut arm_set);
    stats.num_arms_referenced = arm_set.len();
    stats
}
pub(super) fn compute_stats_impl(
    tree: &DecisionTree,
    depth: usize,
    stats: &mut DecisionTreeStats,
    arms: &mut std::collections::HashSet<usize>,
) {
    stats.num_nodes += 1;
    if depth > stats.max_depth {
        stats.max_depth = depth;
    }
    match tree {
        DecisionTree::Leaf { arm_idx, .. } => {
            stats.num_leaves += 1;
            arms.insert(*arm_idx);
        }
        DecisionTree::Failure => {
            stats.num_failures += 1;
        }
        DecisionTree::Switch {
            branches, default, ..
        } => {
            for branch in branches {
                compute_stats_impl(&branch.subtree, depth + 1, stats, arms);
            }
            if let Some(d) = default {
                compute_stats_impl(d, depth + 1, stats, arms);
            }
        }
    }
}
/// Simplify a tree by collapsing switches where all branches lead to the same leaf.
pub fn simplify_tree(tree: DecisionTree) -> DecisionTree {
    match tree {
        DecisionTree::Switch {
            column,
            branches,
            default,
        } => {
            let simplified_branches: Vec<DecisionBranch> = branches
                .into_iter()
                .map(|b| DecisionBranch {
                    subtree: simplify_tree(b.subtree),
                    ..b
                })
                .collect();
            let simplified_default = default.map(|d| Box::new(simplify_tree(*d)));
            let all_same_leaf = {
                let leaf_idx = simplified_branches.first().and_then(|b| {
                    if let DecisionTree::Leaf { arm_idx, .. } = &b.subtree {
                        Some(*arm_idx)
                    } else {
                        None
                    }
                });
                leaf_idx.is_some_and(|idx| {
                    simplified_branches.iter().all(|b| {
                        matches!(
                            & b.subtree, DecisionTree::Leaf { arm_idx, .. } if * arm_idx
                            == idx
                        )
                    }) && simplified_default.as_ref().map_or(true, |d| {
                        matches!(
                            d.as_ref(), DecisionTree::Leaf { arm_idx, .. } if * arm_idx
                            == idx
                        )
                    })
                })
            };
            if all_same_leaf {
                if let Some(first) = simplified_branches.first() {
                    if let DecisionTree::Leaf { arm_idx, .. } = &first.subtree {
                        return DecisionTree::Leaf {
                            arm_idx: *arm_idx,
                            bindings: vec![],
                        };
                    }
                }
            }
            DecisionTree::Switch {
                column,
                branches: simplified_branches,
                default: simplified_default,
            }
        }
        other => other,
    }
}
/// Check if a tree has any `Failure` leaves (i.e., is non-exhaustive).
pub fn is_exhaustive_tree(tree: &DecisionTree) -> bool {
    !has_failure(tree)
}
pub(super) fn has_failure(tree: &DecisionTree) -> bool {
    match tree {
        DecisionTree::Failure => true,
        DecisionTree::Leaf { .. } => false,
        DecisionTree::Switch {
            branches, default, ..
        } => {
            branches.iter().any(|b| has_failure(&b.subtree))
                || default.as_ref().is_some_and(|d| has_failure(d))
        }
    }
}
/// Produce a human-readable representation of a decision tree for debugging.
pub fn tree_to_string(tree: &DecisionTree) -> String {
    let mut buf = String::new();
    tree_to_string_impl(tree, 0, &mut buf);
    buf
}
pub(super) fn tree_to_string_impl(tree: &DecisionTree, indent: usize, buf: &mut String) {
    let prefix = "  ".repeat(indent);
    match tree {
        DecisionTree::Leaf { arm_idx, .. } => {
            buf.push_str(&format!(
                "{}Leaf(arm={})
",
                prefix, arm_idx
            ));
        }
        DecisionTree::Failure => {
            buf.push_str(&format!(
                "{}Failure
",
                prefix
            ));
        }
        DecisionTree::Switch {
            column,
            branches,
            default,
        } => {
            buf.push_str(&format!(
                "{}Switch(col={})
",
                prefix, column
            ));
            for branch in branches {
                buf.push_str(&format!(
                    "{}  | {} ->
",
                    prefix, branch.ctor_name
                ));
                tree_to_string_impl(&branch.subtree, indent + 2, buf);
            }
            if let Some(d) = default {
                buf.push_str(&format!(
                    "{}  | _ ->
",
                    prefix
                ));
                tree_to_string_impl(d, indent + 2, buf);
            }
        }
    }
}
#[cfg(test)]
mod extended_tests {
    use super::*;
    use crate::match_dectree::*;
    use oxilean_kernel::Name;
    fn mk_leaf(arm_idx: usize) -> DecisionTree {
        DecisionTree::Leaf {
            arm_idx,
            bindings: vec![],
        }
    }
    fn mk_branch(ctor: &str, sub: DecisionTree) -> DecisionBranch {
        DecisionBranch {
            ctor_name: Name::str(ctor),
            num_fields: 0,
            field_names: vec![],
            subtree: sub,
        }
    }
    fn mk_switch(
        column: usize,
        branches: Vec<DecisionBranch>,
        default: Option<DecisionTree>,
    ) -> DecisionTree {
        DecisionTree::Switch {
            column,
            branches,
            default: default.map(Box::new),
        }
    }
    #[test]
    fn test_compute_tree_stats_leaf() {
        let tree = mk_leaf(0);
        let stats = compute_tree_stats(&tree);
        assert_eq!(stats.num_leaves, 1);
        assert_eq!(stats.num_failures, 0);
        assert_eq!(stats.num_arms_referenced, 1);
        assert_eq!(stats.max_depth, 0);
    }
    #[test]
    fn test_compute_tree_stats_failure() {
        let tree = DecisionTree::Failure;
        let stats = compute_tree_stats(&tree);
        assert_eq!(stats.num_failures, 1);
        assert_eq!(stats.num_leaves, 0);
    }
    #[test]
    fn test_compute_tree_stats_switch() {
        let tree = mk_switch(
            0,
            vec![
                mk_branch("Nat.zero", mk_leaf(0)),
                mk_branch("Nat.succ", mk_leaf(1)),
            ],
            None,
        );
        let stats = compute_tree_stats(&tree);
        assert_eq!(stats.num_leaves, 2);
        assert_eq!(stats.num_arms_referenced, 2);
        assert_eq!(stats.max_depth, 1);
    }
    #[test]
    fn test_is_exhaustive_tree_no_failure() {
        let tree = mk_switch(
            0,
            vec![mk_branch("A", mk_leaf(0)), mk_branch("B", mk_leaf(1))],
            None,
        );
        assert!(is_exhaustive_tree(&tree));
    }
    #[test]
    fn test_is_exhaustive_tree_with_failure() {
        let tree = mk_switch(
            0,
            vec![mk_branch("A", mk_leaf(0))],
            Some(DecisionTree::Failure),
        );
        assert!(!is_exhaustive_tree(&tree));
    }
    #[test]
    fn test_simplify_tree_leaf_passthrough() {
        let tree = mk_leaf(5);
        let simplified = simplify_tree(tree);
        assert!(matches!(simplified, DecisionTree::Leaf { arm_idx: 5, .. }));
    }
    #[test]
    fn test_simplify_tree_all_same_leaf() {
        let tree = mk_switch(
            0,
            vec![mk_branch("A", mk_leaf(3)), mk_branch("B", mk_leaf(3))],
            None,
        );
        let simplified = simplify_tree(tree);
        assert!(matches!(simplified, DecisionTree::Leaf { arm_idx: 3, .. }));
    }
    #[test]
    fn test_tree_to_string_leaf() {
        let tree = mk_leaf(0);
        let s = tree_to_string(&tree);
        assert!(s.contains("Leaf(arm=0)"));
    }
    #[test]
    fn test_tree_to_string_switch() {
        let tree = mk_switch(
            0,
            vec![mk_branch("Nat.zero", mk_leaf(0))],
            Some(DecisionTree::Failure),
        );
        let s = tree_to_string(&tree);
        assert!(s.contains("Switch"));
        assert!(s.contains("Failure"));
    }
}
/// Score all columns using the given heuristic.
#[allow(dead_code)]
pub fn score_columns(
    arms: &[MetaMatchArm],
    num_columns: usize,
    heuristic: ColumnHeuristic,
) -> Vec<ColumnScore> {
    let mut scores = Vec::with_capacity(num_columns);
    for col in 0..num_columns {
        let mut distinct: std::collections::HashSet<String> = std::collections::HashSet::new();
        let mut wildcard_count = 0usize;
        for arm in arms {
            match arm.patterns.get(col) {
                Some(crate::match_basic::MetaPattern::Constructor(name, _)) => {
                    distinct.insert(name.to_string());
                }
                Some(crate::match_basic::MetaPattern::Wildcard)
                | Some(crate::match_basic::MetaPattern::Var(_)) => {
                    wildcard_count += 1;
                }
                _ => {}
            }
        }
        let distinct_ctors = distinct.len();
        let score = match heuristic {
            ColumnHeuristic::MostDistinct => distinct_ctors as i64,
            ColumnHeuristic::Leftmost => -(col as i64),
            ColumnHeuristic::FewestWildcards => -(wildcard_count as i64),
            ColumnHeuristic::Combined => (distinct_ctors as i64) * 1000 - (wildcard_count as i64),
        };
        scores.push(ColumnScore {
            column: col,
            distinct_ctors,
            wildcard_rows: wildcard_count,
            score,
        });
    }
    scores
}
/// Select the best column index using the given heuristic.
#[allow(dead_code)]
pub fn select_column_heuristic(
    arms: &[MetaMatchArm],
    num_columns: usize,
    heuristic: ColumnHeuristic,
) -> usize {
    let scores = score_columns(arms, num_columns, heuristic);
    scores
        .into_iter()
        .max_by_key(|s| s.score)
        .map(|s| s.column)
        .unwrap_or(0)
}
/// Extract jump tables from a decision tree (one per Switch node whose direct children are leaves).
#[allow(dead_code)]
pub fn extract_jump_tables(tree: &DecisionTree) -> Vec<JumpTable> {
    let mut tables = Vec::new();
    extract_jump_tables_rec(tree, &mut tables);
    tables
}
pub(super) fn extract_jump_tables_rec(tree: &DecisionTree, tables: &mut Vec<JumpTable>) {
    match tree {
        DecisionTree::Leaf { .. } | DecisionTree::Failure => {}
        DecisionTree::Switch {
            column,
            branches,
            default,
        } => {
            let entries: Vec<JumpTableEntry> = branches
                .iter()
                .enumerate()
                .filter_map(|(i, b)| {
                    if let DecisionTree::Leaf { arm_idx, .. } = &b.subtree {
                        Some(JumpTableEntry {
                            ctor_name: b.ctor_name.clone(),
                            ctor_idx: i,
                            arm_idx: *arm_idx,
                        })
                    } else {
                        None
                    }
                })
                .collect();
            let default_arm = default.as_ref().and_then(|d| {
                if let DecisionTree::Leaf { arm_idx, .. } = d.as_ref() {
                    Some(*arm_idx)
                } else {
                    None
                }
            });
            if !entries.is_empty() || default_arm.is_some() {
                tables.push(JumpTable {
                    column: *column,
                    entries,
                    default_arm,
                });
            }
            for branch in branches {
                extract_jump_tables_rec(&branch.subtree, tables);
            }
            if let Some(d) = default {
                extract_jump_tables_rec(d, tables);
            }
        }
    }
}
/// Render a decision tree as an ASCII art string.
#[allow(dead_code)]
pub fn render_ascii(tree: &DecisionTree) -> String {
    let mut buf = String::new();
    render_ascii_rec(tree, "", "", &mut buf);
    buf
}
pub(super) fn render_ascii_rec(
    tree: &DecisionTree,
    prefix: &str,
    child_prefix: &str,
    buf: &mut String,
) {
    match tree {
        DecisionTree::Leaf { arm_idx, bindings } => {
            buf.push_str(&format!(
                "{}[MATCH arm={}  bindings={}]\n",
                prefix,
                arm_idx,
                bindings.len()
            ));
        }
        DecisionTree::Failure => {
            buf.push_str(&format!("{}[FAIL]\n", prefix));
        }
        DecisionTree::Switch {
            column,
            branches,
            default,
        } => {
            buf.push_str(&format!("{}Switch(col={})\n", prefix, column));
            let total = branches.len() + if default.is_some() { 1 } else { 0 };
            for (i, branch) in branches.iter().enumerate() {
                let is_last = i + 1 == total;
                let connector = if is_last { "+-" } else { "|-" };
                let new_prefix = format!("{}{}[{}] ", child_prefix, connector, branch.ctor_name);
                let new_child_prefix =
                    format!("{}{}", child_prefix, if is_last { "   " } else { "|  " });
                render_ascii_rec(&branch.subtree, &new_prefix, &new_child_prefix, buf);
            }
            if let Some(def) = default {
                let new_prefix = format!("{}+-[_] ", child_prefix);
                let new_child_prefix = format!("{}   ", child_prefix);
                render_ascii_rec(def, &new_prefix, &new_child_prefix, buf);
            }
        }
    }
}
/// Flatten a nested constructor pattern into a sequence of (depth, ctor, arity) triples.
#[allow(dead_code)]
pub fn flatten_nested_ctor(
    pat: &crate::match_basic::MetaPattern,
    depth: usize,
) -> Vec<(usize, Name, usize)> {
    match pat {
        crate::match_basic::MetaPattern::Constructor(name, subpats) => {
            let mut result = vec![(depth, name.clone(), subpats.len())];
            for sub in subpats {
                result.extend(flatten_nested_ctor(sub, depth + 1));
            }
            result
        }
        _ => vec![],
    }
}
/// Compute the maximum nesting depth of a pattern.
#[allow(dead_code)]
pub fn pattern_nesting_depth(pat: &crate::match_basic::MetaPattern) -> usize {
    match pat {
        crate::match_basic::MetaPattern::Constructor(_, subpats) => {
            1 + subpats.iter().map(pattern_nesting_depth).max().unwrap_or(0)
        }
        crate::match_basic::MetaPattern::As(inner, _) => 1 + pattern_nesting_depth(inner),
        crate::match_basic::MetaPattern::Or(inner, _) => 1 + pattern_nesting_depth(inner),
        _ => 0,
    }
}
/// Expand all or-patterns in a list of arms into separate arms.
#[allow(dead_code)]
pub fn expand_or_patterns(arms: &[MetaMatchArm]) -> Vec<MetaMatchArm> {
    let mut result = Vec::new();
    for arm in arms {
        expand_or_in_arm(arm, 0, &mut result);
    }
    result
}
pub(super) fn expand_or_in_arm(arm: &MetaMatchArm, col: usize, result: &mut Vec<MetaMatchArm>) {
    if col >= arm.patterns.len() {
        result.push(arm.clone());
        return;
    }
    if let crate::match_basic::MetaPattern::Or(left, right) = &arm.patterns[col] {
        let mut left_arm = arm.clone();
        left_arm.patterns[col] = *left.clone();
        expand_or_in_arm(&left_arm, col + 1, result);
        let mut right_arm = arm.clone();
        right_arm.patterns[col] = *right.clone();
        expand_or_in_arm(&right_arm, col + 1, result);
    } else {
        expand_or_in_arm(arm, col + 1, result);
    }
}
/// Partition arms into guarded and unguarded sets.
#[allow(dead_code)]
pub fn partition_by_guard(arms: &[MetaMatchArm]) -> (Vec<GuardedArm>, Vec<GuardedArm>) {
    let guarded = arms
        .iter()
        .enumerate()
        .filter_map(|(i, arm)| {
            arm.guard.as_ref().map(|g| GuardedArm {
                arm_idx: i,
                guard: Some(g.clone()),
                rhs: arm.rhs.clone(),
            })
        })
        .collect();
    let unguarded = arms
        .iter()
        .enumerate()
        .filter_map(|(i, arm)| {
            if arm.guard.is_none() {
                Some(GuardedArm {
                    arm_idx: i,
                    guard: None,
                    rhs: arm.rhs.clone(),
                })
            } else {
                None
            }
        })
        .collect();
    (guarded, unguarded)
}
/// Check if any arm has a guard.
#[allow(dead_code)]
pub fn has_guarded_arms(arms: &[MetaMatchArm]) -> bool {
    arms.iter().any(|a| a.guard.is_some())
}
/// Compute extended statistics for a decision tree.
#[allow(dead_code)]
pub fn compute_extended_stats(tree: &DecisionTree) -> ExtendedTreeStats {
    let mut stats = ExtendedTreeStats::default();
    compute_extended_stats_rec(tree, 0, &mut stats);
    stats.finalize();
    stats
}
pub(super) fn compute_extended_stats_rec(
    tree: &DecisionTree,
    depth: usize,
    stats: &mut ExtendedTreeStats,
) {
    if depth > stats.tree_depth {
        stats.tree_depth = depth;
    }
    match tree {
        DecisionTree::Leaf { .. } => stats.num_leaves += 1,
        DecisionTree::Failure => stats.num_failures += 1,
        DecisionTree::Switch {
            branches, default, ..
        } => {
            stats.num_switches += 1;
            let n = branches.len() + if default.is_some() { 1 } else { 0 };
            stats.total_branches += n;
            stats.sum_branching += n;
            if n > stats.max_branching_factor {
                stats.max_branching_factor = n;
            }
            for b in branches {
                compute_extended_stats_rec(&b.subtree, depth + 1, stats);
            }
            if let Some(d) = default {
                compute_extended_stats_rec(d, depth + 1, stats);
            }
        }
    }
}
#[cfg(test)]
mod dectree_extended_tests {
    use super::*;
    use crate::match_dectree::*;
    use oxilean_kernel::{Literal, Name};
    fn mk_ctor(name: &str, subs: Vec<MetaPattern>) -> MetaPattern {
        MetaPattern::Constructor(Name::str(name), subs)
    }
    fn mk_wild() -> MetaPattern {
        MetaPattern::Wildcard
    }
    fn dummy_rhs(n: u64) -> oxilean_kernel::Expr {
        oxilean_kernel::Expr::Lit(Literal::Nat(n))
    }
    fn mk_arm(pats: Vec<MetaPattern>) -> MetaMatchArm {
        MetaMatchArm {
            patterns: pats,
            guard: None,
            rhs: dummy_rhs(0),
        }
    }
    fn mk_leaf_dt(arm_idx: usize) -> DecisionTree {
        DecisionTree::Leaf {
            arm_idx,
            bindings: vec![],
        }
    }
    fn mk_branch_dt(ctor: &str, sub: DecisionTree) -> DecisionBranch {
        DecisionBranch {
            ctor_name: Name::str(ctor),
            num_fields: 0,
            field_names: vec![],
            subtree: sub,
        }
    }
    #[test]
    fn test_column_heuristic_labels() {
        assert_eq!(ColumnHeuristic::MostDistinct.label(), "most-distinct");
        assert_eq!(ColumnHeuristic::Combined.label(), "combined");
    }
    #[test]
    fn test_score_columns_single() {
        let arms = vec![
            mk_arm(vec![mk_ctor("Nat.zero", vec![])]),
            mk_arm(vec![mk_ctor("Nat.succ", vec![mk_wild()])]),
        ];
        let scores = score_columns(&arms, 1, ColumnHeuristic::MostDistinct);
        assert_eq!(scores.len(), 1);
        assert_eq!(scores[0].distinct_ctors, 2);
    }
    #[test]
    fn test_select_column_heuristic() {
        let arms = vec![
            mk_arm(vec![mk_ctor("Nat.zero", vec![]), mk_wild()]),
            mk_arm(vec![mk_ctor("Nat.succ", vec![mk_wild()]), mk_wild()]),
        ];
        let col = select_column_heuristic(&arms, 2, ColumnHeuristic::MostDistinct);
        assert_eq!(col, 0);
    }
    #[test]
    fn test_decision_dag_from_tree() {
        let tree = DecisionTree::Switch {
            column: 0,
            branches: vec![
                mk_branch_dt("Nat.zero", mk_leaf_dt(0)),
                mk_branch_dt("Nat.succ", mk_leaf_dt(1)),
            ],
            default: None,
        };
        let dag = DecisionDag::from_tree(&tree);
        assert!(dag.num_nodes() >= 3);
        assert_eq!(dag.num_leaves(), 2);
    }
    #[test]
    fn test_decision_dag_empty() {
        let dag = DecisionDag::new();
        assert_eq!(dag.num_nodes(), 0);
    }
    #[test]
    fn test_jump_table_lookup() {
        let table = JumpTable {
            column: 0,
            entries: vec![
                JumpTableEntry {
                    ctor_name: Name::str("Nat.zero"),
                    ctor_idx: 0,
                    arm_idx: 0,
                },
                JumpTableEntry {
                    ctor_name: Name::str("Nat.succ"),
                    ctor_idx: 1,
                    arm_idx: 1,
                },
            ],
            default_arm: None,
        };
        assert_eq!(table.lookup(&Name::str("Nat.zero")), Some(0));
        assert_eq!(table.lookup(&Name::str("Nat.succ")), Some(1));
        assert_eq!(table.lookup(&Name::str("Unknown")), None);
    }
    #[test]
    fn test_jump_table_default() {
        let table = JumpTable {
            column: 0,
            entries: vec![],
            default_arm: Some(42),
        };
        assert_eq!(table.lookup(&Name::str("anything")), Some(42));
    }
    #[test]
    fn test_jump_table_is_uniform() {
        let table = JumpTable {
            column: 0,
            entries: vec![
                JumpTableEntry {
                    ctor_name: Name::str("A"),
                    ctor_idx: 0,
                    arm_idx: 5,
                },
                JumpTableEntry {
                    ctor_name: Name::str("B"),
                    ctor_idx: 1,
                    arm_idx: 5,
                },
            ],
            default_arm: Some(5),
        };
        assert!(table.is_uniform());
    }
    #[test]
    fn test_extract_jump_tables_leaf_switch() {
        let tree = DecisionTree::Switch {
            column: 0,
            branches: vec![
                mk_branch_dt("Nat.zero", mk_leaf_dt(0)),
                mk_branch_dt("Nat.succ", mk_leaf_dt(1)),
            ],
            default: None,
        };
        let tables = extract_jump_tables(&tree);
        assert_eq!(tables.len(), 1);
        assert_eq!(tables[0].entries.len(), 2);
    }
    #[test]
    fn test_render_ascii_leaf() {
        let s = render_ascii(&mk_leaf_dt(0));
        assert!(s.contains("MATCH"));
        assert!(s.contains("arm=0"));
    }
    #[test]
    fn test_render_ascii_switch() {
        let tree = DecisionTree::Switch {
            column: 0,
            branches: vec![mk_branch_dt("Nat.zero", mk_leaf_dt(0))],
            default: Some(Box::new(DecisionTree::Failure)),
        };
        let s = render_ascii(&tree);
        assert!(s.contains("Switch"));
        assert!(s.contains("Nat.zero"));
        assert!(s.contains("FAIL"));
    }
    #[test]
    fn test_flatten_nested_ctor() {
        let p = mk_ctor("Nat.succ", vec![mk_ctor("Nat.zero", vec![])]);
        let flat = flatten_nested_ctor(&p, 0);
        assert_eq!(flat.len(), 2);
        assert_eq!(flat[0].0, 0);
        assert_eq!(flat[1].0, 1);
    }
    #[test]
    fn test_pattern_nesting_depth_flat() {
        let p = mk_ctor("Nat.zero", vec![]);
        assert_eq!(pattern_nesting_depth(&p), 1);
    }
    #[test]
    fn test_pattern_nesting_depth_nested() {
        let p = mk_ctor("Nat.succ", vec![mk_ctor("Nat.succ", vec![mk_wild()])]);
        assert_eq!(pattern_nesting_depth(&p), 2);
    }
    #[test]
    fn test_expand_or_patterns_no_or() {
        let arms = vec![mk_arm(vec![mk_ctor("Nat.zero", vec![])])];
        let expanded = expand_or_patterns(&arms);
        assert_eq!(expanded.len(), 1);
    }
    #[test]
    fn test_expand_or_patterns_with_or() {
        let or_pat = MetaPattern::Or(
            Box::new(mk_ctor("Nat.zero", vec![])),
            Box::new(mk_ctor("Nat.succ", vec![mk_wild()])),
        );
        let arms = vec![MetaMatchArm {
            patterns: vec![or_pat],
            guard: None,
            rhs: dummy_rhs(0),
        }];
        let expanded = expand_or_patterns(&arms);
        assert_eq!(expanded.len(), 2);
    }
    #[test]
    fn test_partition_by_guard() {
        let arms = vec![
            MetaMatchArm {
                patterns: vec![mk_wild()],
                guard: None,
                rhs: dummy_rhs(0),
            },
            MetaMatchArm {
                patterns: vec![mk_wild()],
                guard: Some(oxilean_kernel::Expr::Lit(Literal::Nat(1))),
                rhs: dummy_rhs(1),
            },
        ];
        let (guarded, unguarded) = partition_by_guard(&arms);
        assert_eq!(guarded.len(), 1);
        assert_eq!(unguarded.len(), 1);
    }
    #[test]
    fn test_has_guarded_arms() {
        let arms = vec![MetaMatchArm {
            patterns: vec![mk_wild()],
            guard: Some(oxilean_kernel::Expr::Lit(Literal::Nat(1))),
            rhs: dummy_rhs(0),
        }];
        assert!(has_guarded_arms(&arms));
    }
    #[test]
    fn test_exhaustiveness_info_bool() {
        let arms = vec![
            mk_arm(vec![mk_ctor("Bool.true", vec![])]),
            mk_arm(vec![mk_ctor("Bool.false", vec![])]),
        ];
        let info = ExhaustivenessInfo::from_arms(&arms, 0);
        assert!(info.is_exhaustive);
    }
    #[test]
    fn test_exhaustiveness_info_wildcard() {
        let arms = vec![mk_arm(vec![mk_wild()])];
        let info = ExhaustivenessInfo::from_arms(&arms, 0);
        assert!(info.has_wildcard);
        assert!(info.is_exhaustive);
    }
    #[test]
    fn test_exhaustiveness_propagate() {
        assert!(ExhaustivenessInfo::propagate(&[true, true], false));
        assert!(ExhaustivenessInfo::propagate(&[false, false], true));
        assert!(!ExhaustivenessInfo::propagate(&[true, false], false));
    }
    #[test]
    fn test_compute_extended_stats_leaf() {
        let tree = mk_leaf_dt(0);
        let stats = compute_extended_stats(&tree);
        assert_eq!(stats.num_leaves, 1);
        assert_eq!(stats.num_switches, 0);
    }
    #[test]
    fn test_compute_extended_stats_switch() {
        let tree = DecisionTree::Switch {
            column: 0,
            branches: vec![
                mk_branch_dt("A", mk_leaf_dt(0)),
                mk_branch_dt("B", mk_leaf_dt(1)),
            ],
            default: None,
        };
        let stats = compute_extended_stats(&tree);
        assert_eq!(stats.num_switches, 1);
        assert_eq!(stats.num_leaves, 2);
        assert_eq!(stats.max_branching_factor, 2);
        assert!((stats.avg_branching_factor - 2.0).abs() < 1e-9);
    }
    #[test]
    fn test_dag_num_failures() {
        let tree = DecisionTree::Switch {
            column: 0,
            branches: vec![mk_branch_dt("Nat.zero", mk_leaf_dt(0))],
            default: Some(Box::new(DecisionTree::Failure)),
        };
        let dag = DecisionDag::from_tree(&tree);
        assert_eq!(dag.num_failures(), 1);
    }
    #[test]
    fn test_jump_table_size() {
        let table = JumpTable {
            column: 0,
            entries: vec![JumpTableEntry {
                ctor_name: Name::str("A"),
                ctor_idx: 0,
                arm_idx: 0,
            }],
            default_arm: None,
        };
        assert_eq!(table.size(), 1);
    }
}
/// Compute the depth of a decision tree (alternate analysis variant).
#[allow(dead_code)]
pub fn tree_depth_v2(tree: &DecisionTree) -> usize {
    match tree {
        DecisionTree::Leaf { .. } => 0,
        DecisionTree::Failure => 0,
        DecisionTree::Switch {
            branches, default, ..
        } => {
            let branch_depths = branches.iter().map(|b| 1 + tree_depth_v2(&b.subtree));
            let default_depth = default.as_ref().map(|d| 1 + tree_depth_v2(d)).unwrap_or(0);
            branch_depths
                .chain(std::iter::once(default_depth))
                .max()
                .unwrap_or(0)
        }
    }
}
/// Count the number of leaf nodes in a decision tree (alternate analysis variant).
#[allow(dead_code)]
pub fn count_leaves_v2(tree: &DecisionTree) -> usize {
    match tree {
        DecisionTree::Leaf { .. } => 1,
        DecisionTree::Failure => 1,
        DecisionTree::Switch {
            branches, default, ..
        } => {
            let branch_count: usize = branches.iter().map(|b| count_leaves_v2(&b.subtree)).sum();
            let default_count = default.as_ref().map(|d| count_leaves_v2(d)).unwrap_or(0);
            branch_count + default_count
        }
    }
}
/// Count the total number of switch nodes in a decision tree.
#[allow(dead_code)]
pub fn count_switches(tree: &DecisionTree) -> usize {
    match tree {
        DecisionTree::Leaf { .. } | DecisionTree::Failure => 0,
        DecisionTree::Switch {
            branches, default, ..
        } => {
            let branch_count: usize = branches.iter().map(|b| count_switches(&b.subtree)).sum();
            let default_count = default.as_ref().map(|d| count_switches(d)).unwrap_or(0);
            1 + branch_count + default_count
        }
    }
}
/// Collect all arm indices that appear in a decision tree (reachable arms).
#[allow(dead_code)]
pub fn reachable_arms(tree: &DecisionTree) -> Vec<usize> {
    let mut arms = Vec::new();
    collect_reachable_arms(tree, &mut arms);
    arms.sort_unstable();
    arms.dedup();
    arms
}
pub(super) fn collect_reachable_arms(tree: &DecisionTree, acc: &mut Vec<usize>) {
    match tree {
        DecisionTree::Leaf { arm_idx, .. } => acc.push(*arm_idx),
        DecisionTree::Failure => {}
        DecisionTree::Switch {
            branches, default, ..
        } => {
            for branch in branches {
                collect_reachable_arms(&branch.subtree, acc);
            }
            if let Some(default) = default {
                collect_reachable_arms(default, acc);
            }
        }
    }
}
/// Find unreachable arms in a match expression.
///
/// Returns the indices of arms that are shadowed by earlier arms.
#[allow(dead_code)]
pub fn find_unreachable_arms(num_arms: usize, tree: &DecisionTree) -> Vec<usize> {
    let reachable = reachable_arms(tree);
    (0..num_arms).filter(|i| !reachable.contains(i)).collect()
}
/// Merge branches with identical sub-trees.
///
/// If two constructor branches lead to the same leaf, they can
/// be merged into a shared sub-tree (assuming they bind no variables).
#[allow(dead_code)]
pub fn merge_equivalent_branches(tree: DecisionTree) -> DecisionTree {
    match tree {
        DecisionTree::Switch {
            column,
            mut branches,
            default,
        } => {
            for branch in &mut branches {
                branch.subtree = merge_equivalent_branches(branch.subtree.clone());
            }
            let default = default.map(|d| Box::new(merge_equivalent_branches(*d)));
            let all_same = branches.windows(2).all(|pair| {
                tree_leaf_arm(&pair[0].subtree) == tree_leaf_arm(&pair[1].subtree)
                    && pair[0].subtree == pair[1].subtree
            });
            if all_same && !branches.is_empty() && default.is_none() {
                if let DecisionTree::Leaf { arm_idx, .. } = &branches[0].subtree {
                    return DecisionTree::Leaf {
                        arm_idx: *arm_idx,
                        bindings: Vec::new(),
                    };
                }
            }
            DecisionTree::Switch {
                column,
                branches,
                default,
            }
        }
        other => other,
    }
}
/// Get the arm index if the tree is a simple leaf.
#[allow(dead_code)]
pub(super) fn tree_leaf_arm(tree: &DecisionTree) -> Option<usize> {
    match tree {
        DecisionTree::Leaf { arm_idx, .. } => Some(*arm_idx),
        _ => None,
    }
}
/// Eliminate dead branches (branches under a Failure node remain as-is).
#[allow(dead_code)]
pub fn prune_failure_branches(tree: DecisionTree) -> DecisionTree {
    match tree {
        DecisionTree::Switch {
            column,
            branches,
            default,
        } => {
            let new_branches: Vec<DecisionBranch> = branches
                .into_iter()
                .map(|mut b| {
                    b.subtree = prune_failure_branches(b.subtree);
                    b
                })
                .collect();
            let new_default = default.map(|d| Box::new(prune_failure_branches(*d)));
            DecisionTree::Switch {
                column,
                branches: new_branches,
                default: new_default,
            }
        }
        other => other,
    }
}
/// Flatten single-branch switches into their branch.
#[allow(dead_code)]
pub fn flatten_singleton_switches(tree: DecisionTree) -> DecisionTree {
    match tree {
        DecisionTree::Switch {
            column,
            mut branches,
            default,
        } => {
            for b in &mut branches {
                b.subtree = flatten_singleton_switches(b.subtree.clone());
            }
            let default = default.map(|d| Box::new(flatten_singleton_switches(*d)));
            if branches.len() == 1 && default.is_none() && branches[0].num_fields == 0 {
                return branches.remove(0).subtree;
            }
            DecisionTree::Switch {
                column,
                branches,
                default,
            }
        }
        other => other,
    }
}
pub(super) fn collect_tree_stats(tree: &DecisionTree, stats: &mut TreeStats, depth: usize) {
    stats.total_nodes += 1;
    if depth > stats.max_depth {
        stats.max_depth = depth;
    }
    match tree {
        DecisionTree::Leaf { .. } => {
            stats.leaf_nodes += 1;
        }
        DecisionTree::Failure => {
            stats.failure_nodes += 1;
        }
        DecisionTree::Switch {
            branches, default, ..
        } => {
            stats.switch_nodes += 1;
            for b in branches {
                collect_tree_stats(&b.subtree, stats, depth + 1);
            }
            if let Some(def) = default {
                collect_tree_stats(def, stats, depth + 1);
            }
        }
    }
}
/// Check whether a tree is exhaustive and collect missing pattern descriptions.
#[allow(dead_code)]
pub fn check_exhaustiveness(tree: &DecisionTree) -> ExhaustivenessReport {
    let mut missing = Vec::new();
    check_exhaustiveness_rec(tree, &mut missing);
    if missing.is_empty() {
        ExhaustivenessReport::exhaustive()
    } else {
        ExhaustivenessReport::incomplete(missing)
    }
}
pub(super) fn check_exhaustiveness_rec(tree: &DecisionTree, missing: &mut Vec<String>) {
    match tree {
        DecisionTree::Leaf { .. } => {}
        DecisionTree::Failure => {
            missing.push("_".to_string());
        }
        DecisionTree::Switch {
            branches, default, ..
        } => {
            for b in branches {
                check_exhaustiveness_rec(&b.subtree, missing);
            }
            if let Some(def) = default {
                check_exhaustiveness_rec(def, missing)
            }
        }
    }
}
/// Print a decision tree as an indented text representation.
#[allow(dead_code)]
pub fn print_tree(tree: &DecisionTree) -> String {
    let mut out = String::new();
    print_tree_rec(tree, &mut out, 0);
    out
}
pub(super) fn print_tree_rec(tree: &DecisionTree, out: &mut String, indent: usize) {
    let pad = " ".repeat(indent * 2);
    match tree {
        DecisionTree::Leaf { arm_idx, bindings } => {
            out.push_str(&format!("{}Leaf(arm={})\n", pad, arm_idx));
            for (name, _expr) in bindings {
                out.push_str(&format!("{}  bind {}\n", pad, name));
            }
        }
        DecisionTree::Failure => {
            out.push_str(&format!("{}Failure\n", pad));
        }
        DecisionTree::Switch {
            column,
            branches,
            default,
        } => {
            out.push_str(&format!("{}Switch(col={})\n", pad, column));
            for b in branches {
                out.push_str(&format!(
                    "{}  [{}](fields={})\n",
                    pad, b.ctor_name, b.num_fields
                ));
                print_tree_rec(&b.subtree, out, indent + 2);
            }
            if let Some(def) = default {
                out.push_str(&format!("{}  [default]\n", pad));
                print_tree_rec(def, out, indent + 2);
            }
        }
    }
}
/// Extract the match equations from a decision tree.
///
/// Returns one equation per reachable arm.
#[allow(dead_code)]
pub fn extract_equations(tree: &DecisionTree, arms: &[MetaMatchArm]) -> Vec<MatchEquation> {
    let mut equations = Vec::new();
    extract_equations_rec(tree, arms, &[], &mut equations);
    equations
}
pub(super) fn extract_equations_rec(
    tree: &DecisionTree,
    arms: &[MetaMatchArm],
    _path_patterns: &[MetaPattern],
    acc: &mut Vec<MatchEquation>,
) {
    match tree {
        DecisionTree::Leaf {
            arm_idx,
            bindings: _,
        } => {
            if let Some(arm) = arms.get(*arm_idx) {
                acc.push(MatchEquation {
                    lhs_patterns: arm.patterns.clone(),
                    rhs: arm.rhs.clone(),
                    arm_idx: *arm_idx,
                });
            }
        }
        DecisionTree::Failure => {}
        DecisionTree::Switch {
            branches, default, ..
        } => {
            for b in branches {
                extract_equations_rec(&b.subtree, arms, _path_patterns, acc);
            }
            if let Some(def) = default {
                extract_equations_rec(def, arms, _path_patterns, acc);
            }
        }
    }
}
/// Analyze constructor coverage from match arms.
#[allow(dead_code)]
pub fn analyze_coverage(arms: &[MetaMatchArm]) -> CoverageAnalysis {
    let mut covered: Vec<String> = Vec::new();
    let mut overlapping: Vec<String> = Vec::new();
    for arm in arms {
        for pat in &arm.patterns {
            if let Some(name) = pat.ctor_name() {
                let name_s = name.to_string();
                if covered.contains(&name_s) && !overlapping.contains(&name_s) {
                    overlapping.push(name_s.clone());
                }
                if !covered.contains(&name_s) {
                    covered.push(name_s);
                }
            }
        }
    }
    CoverageAnalysis {
        covered,
        uncovered: Vec::new(),
        overlapping,
    }
}
/// Extract variable bindings from the leaf of a decision tree for a given match arm.
#[allow(dead_code)]
pub fn extract_leaf_bindings(tree: &DecisionTree, arm_idx: usize) -> Option<Vec<(Name, Expr)>> {
    match tree {
        DecisionTree::Leaf {
            arm_idx: idx,
            bindings,
        } if *idx == arm_idx => Some(bindings.clone()),
        DecisionTree::Switch {
            branches, default, ..
        } => {
            for b in branches {
                if let Some(result) = extract_leaf_bindings(&b.subtree, arm_idx) {
                    return Some(result);
                }
            }
            if let Some(def) = default {
                return extract_leaf_bindings(def, arm_idx);
            }
            None
        }
        _ => None,
    }
}
/// Build and optimize a decision tree.
///
/// Runs all optimization passes: merge, prune, flatten, simplify.
#[allow(dead_code)]
pub fn build_optimized_tree(arms: &[MetaMatchArm], num_columns: usize) -> DecisionTree {
    let tree = build_decision_tree(arms, num_columns);
    let tree = simplify_tree(tree);
    let tree = merge_equivalent_branches(tree);
    let tree = prune_failure_branches(tree);
    flatten_singleton_switches(tree)
}
/// Check if any arm in a match has a guard.
#[allow(dead_code)]
pub fn has_any_guards(arms: &[MetaMatchArm]) -> bool {
    arms.iter().any(|a| a.guard.is_some())
}
/// Count the number of arms with guards.
#[allow(dead_code)]
pub fn count_guarded_arms(arms: &[MetaMatchArm]) -> usize {
    arms.iter().filter(|a| a.guard.is_some()).count()
}
/// Extract guard expressions from all arms.
#[allow(dead_code)]
pub fn extract_guards(arms: &[MetaMatchArm]) -> Vec<(usize, Expr)> {
    arms.iter()
        .enumerate()
        .filter_map(|(i, a)| a.guard.as_ref().map(|g| (i, g.clone())))
        .collect()
}
/// Build a pattern matrix from match arms.
///
/// Each row is a flat list of patterns, one per discriminant column.
#[allow(dead_code)]
pub fn build_pattern_matrix(arms: &[MetaMatchArm]) -> Vec<Vec<MetaPattern>> {
    arms.iter().map(|a| a.patterns.clone()).collect()
}
/// Count the distinct pattern kinds in a column.
#[allow(dead_code)]
pub fn count_pattern_kinds(arms: &[MetaMatchArm], col: usize) -> usize {
    let mut kinds: Vec<String> = arms
        .iter()
        .filter_map(|a| a.patterns.get(col))
        .map(pattern_kind_str)
        .collect();
    kinds.sort();
    kinds.dedup();
    kinds.len()
}
/// Return a string description of the pattern kind.
#[allow(dead_code)]
pub fn pattern_kind_str(pat: &MetaPattern) -> String {
    match pat {
        MetaPattern::Wildcard => "_".to_string(),
        MetaPattern::Var(_) => "var".to_string(),
        MetaPattern::Constructor(n, _) => format!("ctor:{}", n),
        MetaPattern::Literal(l) => format!("lit:{:?}", l),
        MetaPattern::As(_, _) => "as".to_string(),
        MetaPattern::Or(_, _) => "or".to_string(),
        MetaPattern::Inaccessible(_) => ".".to_string(),
    }
}
