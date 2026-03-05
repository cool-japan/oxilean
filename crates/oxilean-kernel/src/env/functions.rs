//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::declaration::{ConstantInfo, ConstantVal, DefinitionSafety, DefinitionVal};
use crate::reduce::ReducibilityHint;
use crate::{Expr, Level, Name};
use std::collections::HashMap;

use super::types::{
    ConfigNode, DecisionNode, Declaration, Either2, EnvError, EnvKindCounts, EnvStats, Environment,
    EnvironmentBuilder, EnvironmentSnapshot, EnvironmentView, Fixture, FlatSubstitution,
    FocusStack, LabelSet, MinHeap, NonEmptyVec, PathBuf, PrefixCounter, RewriteRule,
    RewriteRuleSet, SimpleDag, SlidingSum, SmallMap, SparseVec, StackCalc, StatSummary, Stopwatch,
    StringPool, TokenBucket, TransformStat, TransitiveClosure, VersionedRecord, WindowIterator,
    WriteOnce,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Level, Literal};
    #[test]
    fn test_add_and_get() {
        let mut env = Environment::new();
        let nat_ty = Expr::Sort(Level::zero());
        let zero_decl = Declaration::Axiom {
            name: Name::str("Nat.zero"),
            univ_params: vec![],
            ty: nat_ty,
        };
        env.add(zero_decl).expect("value should be present");
        assert!(env.contains(&Name::str("Nat.zero")));
        assert_eq!(env.len(), 1);
        let retrieved = env
            .get(&Name::str("Nat.zero"))
            .expect("retrieved should be present");
        assert_eq!(retrieved.name(), &Name::str("Nat.zero"));
    }
    #[test]
    fn test_duplicate_declaration() {
        let mut env = Environment::new();
        let ty = Expr::Sort(Level::zero());
        let decl1 = Declaration::Axiom {
            name: Name::str("foo"),
            univ_params: vec![],
            ty: ty.clone(),
        };
        let decl2 = Declaration::Axiom {
            name: Name::str("foo"),
            univ_params: vec![],
            ty,
        };
        env.add(decl1).expect("value should be present");
        let result = env.add(decl2);
        assert!(matches!(result, Err(EnvError::DuplicateDeclaration(_))));
    }
    #[test]
    fn test_definition_with_value() {
        let mut env = Environment::new();
        let nat_ty = Expr::Sort(Level::zero());
        let val = Expr::Lit(Literal::Nat(42));
        let decl = Declaration::Definition {
            name: Name::str("answer"),
            univ_params: vec![],
            ty: nat_ty,
            val: val.clone(),
            hint: ReducibilityHint::Regular(1),
        };
        env.add(decl).expect("value should be present");
        let (defn_val, hint) = env
            .get_defn(&Name::str("answer"))
            .expect("value should be present");
        assert_eq!(defn_val, val);
        assert_eq!(hint, ReducibilityHint::Regular(1));
    }
    #[test]
    fn test_constant_info_lookup() {
        let mut env = Environment::new();
        let ci = ConstantInfo::Axiom(crate::declaration::AxiomVal {
            common: ConstantVal {
                name: Name::str("propext"),
                level_params: vec![],
                ty: Expr::Sort(Level::zero()),
            },
            is_unsafe: false,
        });
        env.add_constant(ci).expect("value should be present");
        assert!(env.contains(&Name::str("propext")));
        let found = env
            .find(&Name::str("propext"))
            .expect("found should be present");
        assert!(found.is_axiom());
    }
    #[test]
    fn test_inductive_queries() {
        let mut env = Environment::new();
        let ind = ConstantInfo::Inductive(crate::declaration::InductiveVal {
            common: ConstantVal {
                name: Name::str("Nat"),
                level_params: vec![],
                ty: Expr::Sort(Level::succ(Level::zero())),
            },
            num_params: 0,
            num_indices: 0,
            all: vec![Name::str("Nat")],
            ctors: vec![Name::str("Nat.zero"), Name::str("Nat.succ")],
            num_nested: 0,
            is_rec: true,
            is_unsafe: false,
            is_reflexive: false,
            is_prop: false,
        });
        env.add_constant(ind).expect("value should be present");
        assert!(env.is_inductive(&Name::str("Nat")));
        assert!(!env.is_constructor(&Name::str("Nat")));
        let iv = env
            .get_inductive_val(&Name::str("Nat"))
            .expect("iv should be present");
        assert_eq!(iv.ctors.len(), 2);
    }
    #[test]
    fn test_instantiate_const_type() {
        let mut env = Environment::new();
        let ci = ConstantInfo::Axiom(crate::declaration::AxiomVal {
            common: ConstantVal {
                name: Name::str("List"),
                level_params: vec![Name::str("u")],
                ty: Expr::Sort(Level::param(Name::str("u"))),
            },
            is_unsafe: false,
        });
        env.add_constant(ci).expect("value should be present");
        let result = env
            .instantiate_const_type(&Name::str("List"), &[Level::succ(Level::zero())])
            .expect("value should be present");
        assert_eq!(result, Expr::Sort(Level::succ(Level::zero())));
    }
}
/// Merge two environments into one.
#[allow(dead_code)]
pub fn merge_environments(
    base: Environment,
    extension: Environment,
) -> Result<Environment, EnvError> {
    let mut result = base;
    for (_, ci) in extension.constant_infos() {
        result.add_constant(ci.clone())?;
    }
    Ok(result)
}
/// Filter an environment to only include declarations satisfying a predicate.
#[allow(dead_code)]
pub fn filter_environment<F>(env: &Environment, predicate: F) -> Environment
where
    F: Fn(&Name) -> bool,
{
    let mut result = Environment::new();
    for (name, ci) in env.constant_infos() {
        if predicate(name) {
            let _ = result.add_constant(ci.clone());
        }
    }
    result
}
#[cfg(test)]
mod extended_env_tests {
    use super::*;
    use crate::{
        declaration::{AxiomVal, ConstantVal},
        Level, Literal, ReducibilityHint,
    };
    fn mk_axiom(name: &str) -> crate::declaration::ConstantInfo {
        crate::declaration::ConstantInfo::Axiom(AxiomVal {
            common: ConstantVal {
                name: Name::str(name),
                level_params: vec![],
                ty: Expr::Sort(Level::zero()),
            },
            is_unsafe: false,
        })
    }
    #[test]
    fn test_environment_view_axiom_names() {
        let mut env = Environment::new();
        env.add_constant(mk_axiom("ax1"))
            .expect("value should be present");
        env.add_constant(mk_axiom("ax2"))
            .expect("value should be present");
        let view = EnvironmentView::new(&env);
        let names = view.axiom_names();
        assert_eq!(names.len(), 2);
    }
    #[test]
    fn test_environment_view_counts() {
        let mut env = Environment::new();
        env.add_constant(mk_axiom("a"))
            .expect("value should be present");
        let view = EnvironmentView::new(&env);
        let counts = view.count_by_kind();
        assert_eq!(counts.axioms, 1);
        assert_eq!(counts.total(), 1);
    }
    #[test]
    fn test_env_builder_empty() {
        let builder = EnvironmentBuilder::new();
        assert!(builder.is_empty());
        let env = builder.build().expect("env should be present");
        assert!(env.is_empty());
    }
    #[test]
    fn test_env_builder_with_decls() {
        let decl = Declaration::Axiom {
            name: Name::str("foo"),
            univ_params: vec![],
            ty: Expr::Sort(Level::zero()),
        };
        let env = EnvironmentBuilder::new()
            .add_decl(decl)
            .build()
            .expect("env should be present");
        assert!(env.contains(&Name::str("foo")));
    }
    #[test]
    fn test_env_builder_duplicate_error() {
        let decl1 = Declaration::Axiom {
            name: Name::str("dup"),
            univ_params: vec![],
            ty: Expr::Sort(Level::zero()),
        };
        let decl2 = Declaration::Axiom {
            name: Name::str("dup"),
            univ_params: vec![],
            ty: Expr::Sort(Level::zero()),
        };
        let result = EnvironmentBuilder::new()
            .add_decl(decl1)
            .add_decl(decl2)
            .build();
        assert!(result.is_err());
    }
    #[test]
    fn test_env_builder_with_constant() {
        let env = EnvironmentBuilder::new()
            .add_constant(mk_axiom("myax"))
            .build()
            .expect("value should be present");
        assert!(env.contains(&Name::str("myax")));
    }
    #[test]
    fn test_filter_environment() {
        let mut env = Environment::new();
        env.add_constant(mk_axiom("Nat.zero"))
            .expect("value should be present");
        env.add_constant(mk_axiom("Bool.true"))
            .expect("value should be present");
        let nat_only = filter_environment(&env, |name| name.to_string().starts_with("Nat"));
        assert!(nat_only.contains(&Name::str("Nat.zero")));
        assert!(!nat_only.contains(&Name::str("Bool.true")));
    }
    #[test]
    fn test_env_error_display() {
        let e = EnvError::DuplicateDeclaration(Name::str("foo"));
        assert!(format!("{}", e).contains("foo"));
        let e2 = EnvError::NotFound(Name::str("bar"));
        assert!(format!("{}", e2).contains("bar"));
    }
    #[test]
    fn test_env_kind_counts_total() {
        let counts = EnvKindCounts {
            axioms: 2,
            inductives: 1,
            constructors: 3,
            recursors: 1,
            definitions: 5,
            theorems: 4,
            other: 0,
        };
        assert_eq!(counts.total(), 16);
    }
    #[test]
    fn test_merge_environments_disjoint() {
        let mut env1 = Environment::new();
        env1.add_constant(mk_axiom("a"))
            .expect("value should be present");
        let mut env2 = Environment::new();
        env2.add_constant(mk_axiom("b"))
            .expect("value should be present");
        let merged = merge_environments(env1, env2).expect("merged should be present");
        assert!(merged.contains(&Name::str("a")));
        assert!(merged.contains(&Name::str("b")));
    }
    #[test]
    fn test_merge_environments_conflict() {
        let mut env1 = Environment::new();
        env1.add_constant(mk_axiom("shared"))
            .expect("value should be present");
        let mut env2 = Environment::new();
        env2.add_constant(mk_axiom("shared"))
            .expect("value should be present");
        let result = merge_environments(env1, env2);
        assert!(result.is_err());
    }
    #[test]
    fn test_env_builder_len() {
        let b = EnvironmentBuilder::new()
            .add_constant(mk_axiom("a"))
            .add_constant(mk_axiom("b"));
        assert_eq!(b.len(), 2);
        assert!(!b.is_empty());
    }
    #[test]
    fn test_definition_value_retrieval() {
        let mut env = Environment::new();
        let val = Expr::Lit(Literal::Nat(99));
        env.add(Declaration::Definition {
            name: Name::str("myval"),
            univ_params: vec![],
            ty: Expr::Sort(Level::zero()),
            val: val.clone(),
            hint: ReducibilityHint::Regular(1),
        })
        .expect("value should be present");
        let (v, _) = env
            .get_defn(&Name::str("myval"))
            .expect("value should be present");
        assert_eq!(v, val);
    }
}
/// Compute detailed statistics about an environment.
pub fn env_stats(env: &Environment) -> EnvStats {
    let mut stats = EnvStats::default();
    for (_, ci) in env.constant_infos() {
        stats.total += 1;
        if ci.is_axiom() {
            stats.axioms += 1;
        }
        if ci.is_definition() {
            stats.definitions += 1;
        }
        if ci.is_theorem() {
            stats.theorems += 1;
        }
        if ci.is_inductive() {
            stats.inductives += 1;
        }
        if ci.is_constructor() {
            stats.constructors += 1;
        }
        if ci.is_recursor() {
            stats.recursors += 1;
        }
    }
    stats
}
/// Return all constants whose names start with `prefix`.
pub fn constants_with_prefix<'a>(env: &'a Environment, prefix: &str) -> Vec<&'a Name> {
    env.constant_names()
        .filter(|n| n.to_string().starts_with(prefix))
        .collect()
}
/// Check if the environment contains any constant from a given set of names.
pub fn contains_any(env: &Environment, names: &[Name]) -> bool {
    names.iter().any(|n| env.contains(n))
}
/// Return the subset of `names` that are present in `env`.
pub fn present_names<'a>(env: &'a Environment, names: &[Name]) -> Vec<&'a Name> {
    env.constant_names().filter(|n| names.contains(n)).collect()
}
/// Return the subset of `names` that are NOT present in `env`.
pub fn missing_names(env: &Environment, names: &[Name]) -> Vec<Name> {
    names.iter().filter(|n| !env.contains(n)).cloned().collect()
}
#[cfg(test)]
mod env_new_tests {
    use super::*;
    use crate::{
        declaration::{AxiomVal, ConstantVal},
        Level,
    };
    fn mk_axiom_ci(name: &str) -> crate::declaration::ConstantInfo {
        crate::declaration::ConstantInfo::Axiom(AxiomVal {
            common: ConstantVal {
                name: Name::str(name),
                level_params: vec![],
                ty: Expr::Sort(Level::zero()),
            },
            is_unsafe: false,
        })
    }
    #[test]
    fn test_env_snapshot_basic() {
        let mut env = Environment::new();
        env.add_constant(mk_axiom_ci("a"))
            .expect("value should be present");
        env.add_constant(mk_axiom_ci("b"))
            .expect("value should be present");
        let snap = EnvironmentSnapshot::from_env(&env);
        assert_eq!(snap.len(), 2);
        assert!(!snap.is_empty());
    }
    #[test]
    fn test_env_snapshot_diff() {
        let mut env = Environment::new();
        env.add_constant(mk_axiom_ci("a"))
            .expect("value should be present");
        let snap1 = EnvironmentSnapshot::from_env(&env);
        env.add_constant(mk_axiom_ci("b"))
            .expect("value should be present");
        let snap2 = EnvironmentSnapshot::from_env(&env);
        let added = snap1.diff(&snap2);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0], &Name::str("b"));
    }
    #[test]
    fn test_env_stats_empty() {
        let env = Environment::new();
        let stats = env_stats(&env);
        assert_eq!(stats.total, 0);
    }
    #[test]
    fn test_env_stats_with_axioms() {
        let mut env = Environment::new();
        env.add_constant(mk_axiom_ci("x"))
            .expect("value should be present");
        env.add_constant(mk_axiom_ci("y"))
            .expect("value should be present");
        let stats = env_stats(&env);
        assert_eq!(stats.total, 2);
        assert_eq!(stats.axioms, 2);
    }
    #[test]
    fn test_env_stats_display() {
        let env = Environment::new();
        let stats = env_stats(&env);
        let s = format!("{}", stats);
        assert!(s.contains("total=0"));
    }
    #[test]
    fn test_constants_with_prefix() {
        let mut env = Environment::new();
        env.add_constant(mk_axiom_ci("Nat.zero"))
            .expect("value should be present");
        env.add_constant(mk_axiom_ci("Nat.succ"))
            .expect("value should be present");
        env.add_constant(mk_axiom_ci("Bool.true"))
            .expect("value should be present");
        let nat_consts = constants_with_prefix(&env, "Nat");
        assert_eq!(nat_consts.len(), 2);
    }
    #[test]
    fn test_contains_any() {
        let mut env = Environment::new();
        env.add_constant(mk_axiom_ci("foo"))
            .expect("value should be present");
        assert!(contains_any(&env, &[Name::str("foo"), Name::str("bar")]));
        assert!(!contains_any(&env, &[Name::str("baz")]));
    }
    #[test]
    fn test_missing_names() {
        let mut env = Environment::new();
        env.add_constant(mk_axiom_ci("a"))
            .expect("value should be present");
        let missing = missing_names(&env, &[Name::str("a"), Name::str("b"), Name::str("c")]);
        assert_eq!(missing.len(), 2);
        assert!(missing.contains(&Name::str("b")));
        assert!(missing.contains(&Name::str("c")));
    }
    #[test]
    fn test_present_names() {
        let mut env = Environment::new();
        env.add_constant(mk_axiom_ci("x"))
            .expect("value should be present");
        env.add_constant(mk_axiom_ci("y"))
            .expect("value should be present");
        let present = present_names(&env, &[Name::str("x"), Name::str("z")]);
        assert_eq!(present.len(), 1);
    }
    #[test]
    fn test_env_snapshot_empty() {
        let env = Environment::new();
        let snap = EnvironmentSnapshot::from_env(&env);
        assert!(snap.is_empty());
        assert_eq!(snap.len(), 0);
    }
    #[test]
    fn test_env_snapshot_diff_no_change() {
        let env = Environment::new();
        let snap1 = EnvironmentSnapshot::from_env(&env);
        let snap2 = EnvironmentSnapshot::from_env(&env);
        let added = snap1.diff(&snap2);
        assert!(added.is_empty());
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
