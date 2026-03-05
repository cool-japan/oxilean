//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::match_basic::{MetaMatchArm, MetaPattern};
use oxilean_kernel::{Expr, Name};

use super::types::{ColumnStrategy, CompiledMatch, DecisionTree, ExhaustivenessReport, TreeStats};

#[cfg(test)]
mod dectree_extended_tests_2 {
    use super::*;
    use crate::match_dectree::*;
    fn mk_leaf_arm(arm_idx: usize) -> MetaMatchArm {
        MetaMatchArm {
            patterns: vec![MetaPattern::Wildcard],
            guard: None,
            rhs: Expr::Const(Name::str(format!("rhs_{}", arm_idx)), vec![]),
        }
    }
    fn mk_ctor_arm(ctor: &str) -> MetaMatchArm {
        MetaMatchArm {
            patterns: vec![MetaPattern::Constructor(Name::str(ctor), vec![])],
            guard: None,
            rhs: Expr::Const(Name::str("rhs"), vec![]),
        }
    }
    #[test]
    fn test_compiled_match_wildcard() {
        let arms = vec![mk_leaf_arm(0)];
        let cm = CompiledMatch::compile(&arms, 1);
        assert!(cm.is_exhaustive);
        assert_eq!(cm.num_arms, 1);
    }
    #[test]
    fn test_tree_depth_leaf() {
        let tree = DecisionTree::Leaf {
            arm_idx: 0,
            bindings: vec![],
        };
        assert_eq!(tree_depth(&tree), 0);
    }
    #[test]
    fn test_count_leaves() {
        let tree = DecisionTree::Leaf {
            arm_idx: 0,
            bindings: vec![],
        };
        assert_eq!(count_leaves(&tree), 1);
    }
    #[test]
    fn test_count_switches_none() {
        let tree = DecisionTree::Failure;
        assert_eq!(count_switches(&tree), 0);
    }
    #[test]
    fn test_reachable_arms_failure() {
        let tree = DecisionTree::Failure;
        assert!(reachable_arms(&tree).is_empty());
    }
    #[test]
    fn test_reachable_arms_leaf() {
        let tree = DecisionTree::Leaf {
            arm_idx: 3,
            bindings: vec![],
        };
        assert_eq!(reachable_arms(&tree), vec![3]);
    }
    #[test]
    fn test_find_unreachable_arms() {
        let tree = DecisionTree::Leaf {
            arm_idx: 0,
            bindings: vec![],
        };
        let unreachable = find_unreachable_arms(3, &tree);
        assert_eq!(unreachable, vec![1, 2]);
    }
    #[test]
    fn test_tree_stats_leaf() {
        let tree = DecisionTree::Leaf {
            arm_idx: 0,
            bindings: vec![],
        };
        let stats = TreeStats::from_tree(&tree, 1);
        assert_eq!(stats.leaf_nodes, 1);
        assert_eq!(stats.switch_nodes, 0);
        assert_eq!(stats.total_nodes, 1);
    }
    #[test]
    fn test_exhaustiveness_report() {
        let r = ExhaustivenessReport::exhaustive();
        assert!(r.is_exhaustive);
        assert_eq!(r.format(), "exhaustive");
    }
    #[test]
    fn test_exhaustiveness_incomplete() {
        let r = ExhaustivenessReport::incomplete(vec!["Nat.succ".to_string()]);
        assert!(!r.is_exhaustive);
        assert!(r.format().contains("Nat.succ"));
    }
    #[test]
    fn test_check_exhaustiveness_leaf() {
        let tree = DecisionTree::Leaf {
            arm_idx: 0,
            bindings: vec![],
        };
        let report = check_exhaustiveness(&tree);
        assert!(report.is_exhaustive);
    }
    #[test]
    fn test_check_exhaustiveness_failure() {
        let report = check_exhaustiveness(&DecisionTree::Failure);
        assert!(!report.is_exhaustive);
    }
    #[test]
    fn test_column_strategy_description() {
        assert_eq!(
            ColumnStrategy::Leftmost.description(),
            "leftmost non-wildcard"
        );
        assert_eq!(
            ColumnStrategy::MostDiscriminating.description(),
            "most discriminating"
        );
    }
    #[test]
    fn test_print_tree_leaf() {
        let tree = DecisionTree::Leaf {
            arm_idx: 0,
            bindings: vec![],
        };
        let s = print_tree(&tree);
        assert!(s.contains("Leaf"));
        assert!(s.contains("arm=0"));
    }
    #[test]
    fn test_print_tree_failure() {
        let s = print_tree(&DecisionTree::Failure);
        assert!(s.contains("Failure"));
    }
    #[test]
    fn test_extract_equations_empty() {
        let tree = DecisionTree::Failure;
        let arms: Vec<MetaMatchArm> = vec![];
        let eqs = extract_equations(&tree, &arms);
        assert!(eqs.is_empty());
    }
    #[test]
    fn test_coverage_analysis_basic() {
        let arms = vec![mk_ctor_arm("Nat.zero"), mk_ctor_arm("Nat.succ")];
        let cov = analyze_coverage(&arms);
        assert!(cov.covered.contains(&"Nat.zero".to_string()));
        assert!(cov.covered.contains(&"Nat.succ".to_string()));
        assert!(!cov.has_overlap());
    }
    #[test]
    fn test_coverage_analysis_overlap() {
        let arms = vec![mk_ctor_arm("Nat.zero"), mk_ctor_arm("Nat.zero")];
        let cov = analyze_coverage(&arms);
        assert!(cov.has_overlap());
    }
    #[test]
    fn test_build_pattern_matrix() {
        let arms = vec![mk_leaf_arm(0), mk_leaf_arm(1)];
        let matrix = build_pattern_matrix(&arms);
        assert_eq!(matrix.len(), 2);
        assert_eq!(matrix[0].len(), 1);
    }
    #[test]
    fn test_count_pattern_kinds() {
        let arms = vec![mk_leaf_arm(0), mk_leaf_arm(1)];
        let kinds = count_pattern_kinds(&arms, 0);
        assert_eq!(kinds, 1);
    }
    #[test]
    fn test_pattern_kind_str() {
        assert_eq!(pattern_kind_str(&MetaPattern::Wildcard), "_");
        assert_eq!(pattern_kind_str(&MetaPattern::Var(Name::str("x"))), "var");
    }
    #[test]
    fn test_has_any_guards_none() {
        let arms = vec![mk_leaf_arm(0)];
        assert!(!has_any_guards(&arms));
    }
    #[test]
    fn test_has_any_guards_some() {
        let mut arm = mk_leaf_arm(0);
        arm.guard = Some(Expr::Const(Name::str("True"), vec![]));
        assert!(has_any_guards(&[arm]));
    }
    #[test]
    fn test_count_guarded_arms() {
        let mut arm0 = mk_leaf_arm(0);
        arm0.guard = Some(Expr::Const(Name::str("True"), vec![]));
        let arm1 = mk_leaf_arm(1);
        assert_eq!(count_guarded_arms(&[arm0, arm1]), 1);
    }
    #[test]
    fn test_build_optimized_tree() {
        let arms = vec![mk_leaf_arm(0)];
        let tree = build_optimized_tree(&arms, 1);
        assert!(matches!(tree, DecisionTree::Leaf { arm_idx: 0, .. }));
    }
    #[test]
    fn test_merge_equivalent_branches_leaf() {
        let tree = DecisionTree::Leaf {
            arm_idx: 0,
            bindings: vec![],
        };
        let merged = merge_equivalent_branches(tree.clone());
        assert_eq!(merged, tree);
    }
    #[test]
    fn test_compiled_match_summary() {
        let arms = vec![mk_leaf_arm(0)];
        let cm = CompiledMatch::compile(&arms, 1);
        let summary = cm.summary();
        assert!(summary.contains("arms=1"));
    }
    #[test]
    fn test_extract_leaf_bindings_none() {
        let tree = DecisionTree::Leaf {
            arm_idx: 0,
            bindings: vec![],
        };
        assert!(extract_leaf_bindings(&tree, 1).is_none());
    }
    #[test]
    fn test_extract_leaf_bindings_found() {
        let tree = DecisionTree::Leaf {
            arm_idx: 2,
            bindings: vec![],
        };
        assert_eq!(extract_leaf_bindings(&tree, 2), Some(vec![]));
    }
}
