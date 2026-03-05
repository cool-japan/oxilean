//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::{Expr, Name};
use std::collections::HashMap;

use super::types::{
    CompileResult, ConfigNode, ConstructorInfo, DecisionNode, DecisionTree, Either2,
    FlatSubstitution, FocusStack, LabelSet, MatchArm, MatchCompiler, MatchStats, NonEmptyVec,
    PathBuf, Pattern, PatternStats, RewriteRule, RewriteRuleSet, SimpleDag, SlidingSum, SmallMap,
    SparseVec, StackCalc, StatSummary, Stopwatch, StringPool, TokenBucket, TransformStat,
    TransitiveClosure, VersionedRecord, WindowIterator, WriteOnce,
};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fresh_var() {
        let mut compiler = MatchCompiler::new();
        let v1 = compiler.fresh_var();
        let v2 = compiler.fresh_var();
        assert_ne!(v1, v2);
    }
    #[test]
    fn test_check_exhaustive_wildcard() {
        let mut compiler = MatchCompiler::new();
        compiler.register_constructors(
            Name::str("Bool"),
            vec![
                ConstructorInfo {
                    name: Name::str("true"),
                    num_fields: 0,
                    inductive: Name::str("Bool"),
                },
                ConstructorInfo {
                    name: Name::str("false"),
                    num_fields: 0,
                    inductive: Name::str("Bool"),
                },
            ],
        );
        assert!(compiler
            .check_exhaustive(&[Pattern::Wildcard], &Name::str("Bool"))
            .is_ok());
    }
    #[test]
    fn test_check_exhaustive_all_ctors() {
        let mut compiler = MatchCompiler::new();
        compiler.register_constructors(
            Name::str("Bool"),
            vec![
                ConstructorInfo {
                    name: Name::str("true"),
                    num_fields: 0,
                    inductive: Name::str("Bool"),
                },
                ConstructorInfo {
                    name: Name::str("false"),
                    num_fields: 0,
                    inductive: Name::str("Bool"),
                },
            ],
        );
        let patterns = vec![
            Pattern::Constructor(Name::str("true"), vec![]),
            Pattern::Constructor(Name::str("false"), vec![]),
        ];
        assert!(compiler
            .check_exhaustive(&patterns, &Name::str("Bool"))
            .is_ok());
    }
    #[test]
    fn test_check_exhaustive_missing() {
        let mut compiler = MatchCompiler::new();
        compiler.register_constructors(
            Name::str("Bool"),
            vec![
                ConstructorInfo {
                    name: Name::str("true"),
                    num_fields: 0,
                    inductive: Name::str("Bool"),
                },
                ConstructorInfo {
                    name: Name::str("false"),
                    num_fields: 0,
                    inductive: Name::str("Bool"),
                },
            ],
        );
        let patterns = vec![Pattern::Constructor(Name::str("true"), vec![])];
        assert!(compiler
            .check_exhaustive(&patterns, &Name::str("Bool"))
            .is_err());
    }
    #[test]
    fn test_check_redundant() {
        let compiler = MatchCompiler::new();
        let patterns = vec![
            Pattern::Constructor(Name::str("true"), vec![]),
            Pattern::Constructor(Name::str("true"), vec![]),
            Pattern::Constructor(Name::str("false"), vec![]),
        ];
        let redundant = compiler.check_redundant(&patterns);
        assert_eq!(redundant, vec![1]);
    }
    #[test]
    fn test_check_redundant_after_wildcard() {
        let compiler = MatchCompiler::new();
        let patterns = vec![
            Pattern::Wildcard,
            Pattern::Constructor(Name::str("true"), vec![]),
        ];
        let redundant = compiler.check_redundant(&patterns);
        assert_eq!(redundant, vec![1]);
    }
    #[test]
    fn test_compile_simple_match() {
        let mut compiler = MatchCompiler::new();
        compiler.register_constructors(
            Name::str("Bool"),
            vec![
                ConstructorInfo {
                    name: Name::str("true"),
                    num_fields: 0,
                    inductive: Name::str("Bool"),
                },
                ConstructorInfo {
                    name: Name::str("false"),
                    num_fields: 0,
                    inductive: Name::str("Bool"),
                },
            ],
        );
        let scrutinee = Expr::BVar(0);
        let arms = vec![
            MatchArm {
                patterns: vec![Pattern::Constructor(Name::str("true"), vec![])],
                rhs: Expr::Lit(crate::Literal::Nat(1)),
                guard: None,
            },
            MatchArm {
                patterns: vec![Pattern::Constructor(Name::str("false"), vec![])],
                rhs: Expr::Lit(crate::Literal::Nat(0)),
                guard: None,
            },
        ];
        let result = compiler
            .compile_match(&[scrutinee], &arms)
            .expect("result should be present");
        assert!(result.unreachable_arms.is_empty());
        assert!(result.missing_patterns.is_empty());
    }
    #[test]
    fn test_compile_wildcard_match() {
        let mut compiler = MatchCompiler::new();
        let scrutinee = Expr::BVar(0);
        let arms = vec![MatchArm {
            patterns: vec![Pattern::Wildcard],
            rhs: Expr::Lit(crate::Literal::Nat(42)),
            guard: None,
        }];
        let result = compiler
            .compile_match(&[scrutinee], &arms)
            .expect("result should be present");
        assert!(result.unreachable_arms.is_empty());
    }
    #[test]
    fn test_pattern_eq() {
        assert_eq!(Pattern::Wildcard, Pattern::Wildcard);
        assert_ne!(Pattern::Var(Name::str("x")), Pattern::Var(Name::str("y")));
        assert_eq!(
            Pattern::Constructor(Name::str("C"), vec![]),
            Pattern::Constructor(Name::str("C"), vec![])
        );
    }
}
/// Compute statistics for a compile result.
pub fn compute_match_stats(result: &CompileResult, num_arms: usize) -> MatchStats {
    let referenced: std::collections::HashSet<usize> = referenced_arm_indices(&result.tree);
    let reachable_arms: Vec<usize> = (0..num_arms).filter(|i| referenced.contains(i)).collect();
    let unreachable_arms: Vec<usize> = (0..num_arms).filter(|i| !referenced.contains(i)).collect();
    MatchStats {
        num_arms,
        reachable_arms,
        unreachable_arms: unreachable_arms.clone(),
        is_exhaustive: result.unreachable_arms.is_empty(),
        tree_depth: decision_tree_depth(&result.tree),
    }
}
/// Compute the depth of a decision tree.
pub fn decision_tree_depth(tree: &DecisionTree) -> usize {
    match tree {
        DecisionTree::Leaf { .. } => 0,
        DecisionTree::Failure => 0,
        DecisionTree::Switch {
            branches, default, ..
        } => {
            let max_case = branches
                .iter()
                .map(|(_, _, t)| decision_tree_depth(t))
                .max()
                .unwrap_or(0);
            let default_depth = default
                .as_ref()
                .map(|t| decision_tree_depth(t))
                .unwrap_or(0);
            1 + max_case.max(default_depth)
        }
    }
}
/// Count the number of leaf nodes in a decision tree.
pub fn count_decision_tree_leaves(tree: &DecisionTree) -> usize {
    match tree {
        DecisionTree::Leaf { .. } | DecisionTree::Failure => 1,
        DecisionTree::Switch {
            branches, default, ..
        } => {
            let case_leaves: usize = branches
                .iter()
                .map(|(_, _, t)| count_decision_tree_leaves(t))
                .sum();
            let default_leaves = default
                .as_ref()
                .map(|t| count_decision_tree_leaves(t))
                .unwrap_or(0);
            case_leaves + default_leaves
        }
    }
}
/// Collect all arm indices referenced in a decision tree.
pub fn referenced_arm_indices(tree: &DecisionTree) -> std::collections::HashSet<usize> {
    let mut out = std::collections::HashSet::new();
    collect_refs(tree, &mut out);
    out
}
pub(super) fn collect_refs(tree: &DecisionTree, out: &mut std::collections::HashSet<usize>) {
    match tree {
        DecisionTree::Leaf { arm_idx, .. } => {
            out.insert(*arm_idx);
        }
        DecisionTree::Failure => {}
        DecisionTree::Switch {
            branches, default, ..
        } => {
            for (_, _, t) in branches {
                collect_refs(t, out);
            }
            if let Some(d) = default {
                collect_refs(d, out);
            }
        }
    }
}
/// Check if a pattern is irrefutable (always matches).
pub fn is_irrefutable_pattern(p: &Pattern) -> bool {
    match p {
        Pattern::Wildcard | Pattern::Var(_) => true,
        Pattern::Constructor(_, sub) => sub.iter().all(is_irrefutable_pattern),
        Pattern::Literal(_) => false,
        Pattern::Or(alts) => alts.iter().any(is_irrefutable_pattern),
        Pattern::As(_, inner) => is_irrefutable_pattern(inner),
        Pattern::Inaccessible(_) => true,
    }
}
/// Get the constructor name from a pattern (if it is a constructor pattern).
pub fn pattern_constructor(p: &Pattern) -> Option<&Name> {
    if let Pattern::Constructor(name, _) = p {
        Some(name)
    } else {
        None
    }
}
/// Get the sub-patterns of a constructor pattern.
pub fn pattern_subpatterns(p: &Pattern) -> Option<&[Pattern]> {
    if let Pattern::Constructor(_, sub) = p {
        Some(sub)
    } else {
        None
    }
}
/// Count the number of variable bindings in a pattern.
pub fn count_pattern_bindings(p: &Pattern) -> usize {
    match p {
        Pattern::Var(_) => 1,
        Pattern::Wildcard | Pattern::Literal(_) | Pattern::Inaccessible(_) => 0,
        Pattern::Constructor(_, sub) => sub.iter().map(count_pattern_bindings).sum(),
        Pattern::Or(alts) => alts.iter().map(count_pattern_bindings).min().unwrap_or(0),
        Pattern::As(_, inner) => 1 + count_pattern_bindings(inner),
    }
}
/// Flatten Or patterns into a flat list of alternatives.
pub fn flatten_or_patterns(p: &Pattern) -> Vec<&Pattern> {
    if let Pattern::Or(alts) = p {
        alts.iter().flat_map(|a| flatten_or_patterns(a)).collect()
    } else {
        vec![p]
    }
}
#[cfg(test)]
mod extended_tests {
    use super::*;
    #[test]
    fn test_is_irrefutable_wildcard() {
        assert!(is_irrefutable_pattern(&Pattern::Wildcard));
    }
    #[test]
    fn test_is_irrefutable_var() {
        assert!(is_irrefutable_pattern(&Pattern::Var(Name::str("x"))));
    }
    #[test]
    fn test_is_irrefutable_ctor_false() {
        let p = Pattern::Constructor(Name::str("Nat.succ"), vec![Pattern::Wildcard]);
        assert!(is_irrefutable_pattern(&p));
        let p2 = Pattern::Constructor(
            Name::str("Nat.succ"),
            vec![Pattern::Literal(crate::Literal::Nat(0))],
        );
        assert!(!is_irrefutable_pattern(&p2));
    }
    #[test]
    fn test_pattern_constructor() {
        let p = Pattern::Constructor(Name::str("Foo"), vec![]);
        assert_eq!(pattern_constructor(&p), Some(&Name::str("Foo")));
        assert_eq!(pattern_constructor(&Pattern::Wildcard), None);
    }
    #[test]
    fn test_pattern_subpatterns() {
        let sub = vec![Pattern::Wildcard, Pattern::Wildcard];
        let p = Pattern::Constructor(Name::str("Pair"), sub.clone());
        assert_eq!(pattern_subpatterns(&p).map(|s| s.len()), Some(2));
        assert!(pattern_subpatterns(&Pattern::Wildcard).is_none());
    }
    #[test]
    fn test_count_pattern_bindings_var() {
        assert_eq!(count_pattern_bindings(&Pattern::Var(Name::str("x"))), 1);
    }
    #[test]
    fn test_count_pattern_bindings_ctor() {
        let p = Pattern::Constructor(
            Name::str("Pair"),
            vec![Pattern::Var(Name::str("a")), Pattern::Var(Name::str("b"))],
        );
        assert_eq!(count_pattern_bindings(&p), 2);
    }
    #[test]
    fn test_flatten_or_patterns() {
        let p = Pattern::Or(vec![Pattern::Var(Name::str("x")), Pattern::Wildcard]);
        let flat = flatten_or_patterns(&p);
        assert_eq!(flat.len(), 2);
    }
    #[test]
    fn test_decision_tree_depth_leaf() {
        let tree = DecisionTree::Leaf {
            arm_idx: 0,
            bindings: vec![],
        };
        assert_eq!(decision_tree_depth(&tree), 0);
    }
    #[test]
    fn test_decision_tree_depth_switch() {
        let tree = DecisionTree::Switch {
            scrutinee: crate::Expr::BVar(0),
            branches: vec![
                (
                    Name::str("Nat.zero"),
                    vec![],
                    DecisionTree::Leaf {
                        arm_idx: 0,
                        bindings: vec![],
                    },
                ),
                (
                    Name::str("Nat.succ"),
                    vec![],
                    DecisionTree::Leaf {
                        arm_idx: 1,
                        bindings: vec![],
                    },
                ),
            ],
            default: None,
        };
        assert_eq!(decision_tree_depth(&tree), 1);
    }
    #[test]
    fn test_count_decision_tree_leaves() {
        let tree = DecisionTree::Switch {
            scrutinee: crate::Expr::BVar(0),
            branches: vec![
                (
                    Name::str("A"),
                    vec![],
                    DecisionTree::Leaf {
                        arm_idx: 0,
                        bindings: vec![],
                    },
                ),
                (
                    Name::str("B"),
                    vec![],
                    DecisionTree::Leaf {
                        arm_idx: 1,
                        bindings: vec![],
                    },
                ),
            ],
            default: Some(Box::new(DecisionTree::Failure)),
        };
        assert_eq!(count_decision_tree_leaves(&tree), 3);
    }
    #[test]
    fn test_referenced_arm_indices() {
        let tree = DecisionTree::Switch {
            scrutinee: crate::Expr::BVar(0),
            branches: vec![
                (
                    Name::str("A"),
                    vec![],
                    DecisionTree::Leaf {
                        arm_idx: 0,
                        bindings: vec![],
                    },
                ),
                (
                    Name::str("B"),
                    vec![],
                    DecisionTree::Leaf {
                        arm_idx: 2,
                        bindings: vec![],
                    },
                ),
            ],
            default: None,
        };
        let refs = referenced_arm_indices(&tree);
        assert!(refs.contains(&0));
        assert!(refs.contains(&2));
        assert!(!refs.contains(&1));
    }
}
/// Check if a pattern is irrefutable (always matches).
pub fn is_irrefutable(p: &Pattern) -> bool {
    match p {
        Pattern::Wildcard | Pattern::Var(_) => true,
        Pattern::As(_, inner) => is_irrefutable(inner),
        _ => false,
    }
}
/// Collect all variable names bound in a pattern.
pub fn pattern_binders(p: &Pattern) -> Vec<Name> {
    let mut vars = Vec::new();
    collect_binders(p, &mut vars);
    vars
}
pub(super) fn collect_binders(p: &Pattern, vars: &mut Vec<Name>) {
    match p {
        Pattern::Var(n) => vars.push(n.clone()),
        Pattern::As(n, inner) => {
            vars.push(n.clone());
            collect_binders(inner, vars);
        }
        Pattern::Constructor(_, pats) | Pattern::Or(pats) => {
            for sub in pats {
                collect_binders(sub, vars);
            }
        }
        Pattern::Wildcard | Pattern::Literal(_) | Pattern::Inaccessible(_) => {}
    }
}
/// Count the maximum nesting depth of a pattern.
pub fn pattern_depth(p: &Pattern) -> usize {
    match p {
        Pattern::Wildcard | Pattern::Var(_) | Pattern::Literal(_) | Pattern::Inaccessible(_) => 0,
        Pattern::As(_, inner) => 1 + pattern_depth(inner),
        Pattern::Constructor(_, pats) | Pattern::Or(pats) => {
            1 + pats.iter().map(pattern_depth).max().unwrap_or(0)
        }
    }
}
#[cfg(test)]
mod match_compile_extra_tests {
    use super::*;
    fn mk_ctor(name: &str, fields: Vec<Pattern>) -> Pattern {
        Pattern::Constructor(Name::str(name), fields)
    }
    #[test]
    fn test_pattern_stats_wildcards() {
        let pats = vec![Pattern::Wildcard, Pattern::Var(Name::str("x"))];
        let stats = PatternStats::from_patterns(&pats);
        assert_eq!(stats.wildcards, 2);
        assert_eq!(stats.constructors, 0);
    }
    #[test]
    fn test_pattern_stats_constructors() {
        let pats = vec![
            mk_ctor("Nat.zero", vec![]),
            mk_ctor("Nat.succ", vec![Pattern::Wildcard]),
        ];
        let stats = PatternStats::from_patterns(&pats);
        assert_eq!(stats.constructors, 2);
        assert_eq!(stats.wildcards, 1);
    }
    #[test]
    fn test_pattern_stats_display() {
        let stats = PatternStats {
            total_patterns: 3,
            wildcards: 1,
            constructors: 2,
            ..PatternStats::default()
        };
        let txt = format!("{}", stats);
        assert!(txt.contains("total: 3"));
    }
    #[test]
    fn test_is_irrefutable_wildcard() {
        assert!(is_irrefutable(&Pattern::Wildcard));
    }
    #[test]
    fn test_is_irrefutable_var() {
        assert!(is_irrefutable(&Pattern::Var(Name::str("x"))));
    }
    #[test]
    fn test_is_irrefutable_constructor() {
        assert!(!is_irrefutable(&mk_ctor("C", vec![])));
    }
    #[test]
    fn test_is_irrefutable_as_wildcard() {
        let p = Pattern::As(Name::str("x"), Box::new(Pattern::Wildcard));
        assert!(is_irrefutable(&p));
    }
    #[test]
    fn test_pattern_binders_var() {
        let p = Pattern::Var(Name::str("x"));
        let binders = pattern_binders(&p);
        assert_eq!(binders.len(), 1);
        assert_eq!(binders[0], Name::str("x"));
    }
    #[test]
    fn test_pattern_binders_constructor() {
        let p = mk_ctor(
            "Pair",
            vec![Pattern::Var(Name::str("a")), Pattern::Var(Name::str("b"))],
        );
        let binders = pattern_binders(&p);
        assert_eq!(binders.len(), 2);
    }
    #[test]
    fn test_pattern_depth_wildcard() {
        assert_eq!(pattern_depth(&Pattern::Wildcard), 0);
    }
    #[test]
    fn test_pattern_depth_constructor() {
        let p = mk_ctor("C", vec![mk_ctor("D", vec![Pattern::Wildcard])]);
        assert_eq!(pattern_depth(&p), 2);
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
