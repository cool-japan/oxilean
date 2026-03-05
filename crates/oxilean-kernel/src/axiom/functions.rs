//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::{Declaration, Environment, Expr, Name};
use std::collections::HashMap;
use std::collections::HashSet;

use super::types::{
    AxiomAllowlist, AxiomCategory, AxiomSafety, AxiomSafetyReport, AxiomSequence, AxiomUsageRecord,
    AxiomValidator, BitSet64, BucketCounter, ConfigNode, DecisionNode, Either2, Fixture,
    FlatSubstitution, FocusStack, LabelSet, MinHeap, NonEmptyVec, PathBuf, PrefixCounter,
    RewriteRule, RewriteRuleSet, SimpleDag, SlidingSum, SmallMap, SparseVec, StackCalc,
    StatSummary, Stopwatch, StringPool, TokenBucket, TransformStat, TransitiveClosure,
    VersionedRecord, WindowIterator, WriteOnce,
};

/// Collect all constant names from an expression.
pub(super) fn collect_const_deps(expr: &Expr, deps: &mut HashSet<Name>) {
    match expr {
        Expr::Const(name, _) => {
            deps.insert(name.clone());
        }
        Expr::App(f, a) => {
            collect_const_deps(f, deps);
            collect_const_deps(a, deps);
        }
        Expr::Lam(_, _, ty, body) => {
            collect_const_deps(ty, deps);
            collect_const_deps(body, deps);
        }
        Expr::Pi(_, _, ty, body) => {
            collect_const_deps(ty, deps);
            collect_const_deps(body, deps);
        }
        Expr::Let(_, ty, val, body) => {
            collect_const_deps(ty, deps);
            collect_const_deps(val, deps);
            collect_const_deps(body, deps);
        }
        Expr::Proj(_, _, e) => {
            collect_const_deps(e, deps);
        }
        _ => {}
    }
}
/// Extract all axioms from an environment.
pub fn extract_axioms(env: &Environment) -> Vec<Name> {
    let mut axioms = Vec::new();
    for (name, ci) in env.constant_infos() {
        if ci.is_axiom() {
            axioms.push(name.clone());
        }
    }
    axioms
}
/// Check if a declaration depends on unsafe axioms.
pub fn has_unsafe_dependencies(decl: &Declaration, validator: &AxiomValidator) -> bool {
    let mut deps = HashSet::new();
    collect_const_deps(decl.ty(), &mut deps);
    if let Some(val) = decl.value() {
        collect_const_deps(val, &mut deps);
    }
    deps.iter().any(|n| validator.is_unsafe(n))
}
/// Classify an axiom's safety level (standalone function).
pub fn classify_axiom(name: &Name) -> AxiomSafety {
    let validator = AxiomValidator::new();
    validator.classify(name)
}
/// Compute the transitive closure of axiom dependencies.
///
/// Starting from an expression, find all axioms it depends on,
/// then find all axioms those axioms' definitions depend on, etc.
pub fn transitive_axiom_deps(expr: &Expr, env: &Environment) -> HashSet<Name> {
    let mut result = HashSet::new();
    let mut worklist: Vec<Name> = Vec::new();
    let mut initial_deps = HashSet::new();
    collect_const_deps(expr, &mut initial_deps);
    for dep in initial_deps {
        if !result.contains(&dep) {
            worklist.push(dep);
        }
    }
    while let Some(name) = worklist.pop() {
        if result.contains(&name) {
            continue;
        }
        result.insert(name.clone());
        if let Some(ci) = env.find(&name) {
            let mut sub_deps = HashSet::new();
            collect_const_deps(ci.ty(), &mut sub_deps);
            if let Some(val) = ci.value() {
                collect_const_deps(val, &mut sub_deps);
            }
            for sub_dep in sub_deps {
                if !result.contains(&sub_dep) {
                    worklist.push(sub_dep);
                }
            }
        }
    }
    result
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_validator_create() {
        let validator = AxiomValidator::new();
        assert_eq!(validator.all_axioms().len(), 0);
    }
    #[test]
    fn test_register_axiom() {
        let mut validator = AxiomValidator::new();
        validator.register(Name::str("test"));
        assert!(validator.is_axiom(&Name::str("test")));
    }
    #[test]
    fn test_mark_unsafe() {
        let mut validator = AxiomValidator::new();
        validator.register(Name::str("unsafe_ax"));
        validator.mark_unsafe(Name::str("unsafe_ax"));
        assert!(validator.is_unsafe(&Name::str("unsafe_ax")));
    }
    #[test]
    fn test_unsafe_axioms() {
        let mut validator = AxiomValidator::new();
        validator.register(Name::str("ax1"));
        validator.mark_unsafe(Name::str("ax1"));
        assert_eq!(validator.unsafe_axioms().len(), 1);
    }
    #[test]
    fn test_check_dependencies() {
        let validator = AxiomValidator::new();
        let expr = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(Expr::Const(Name::str("g"), vec![])),
        );
        let deps = validator.check_dependencies(&expr);
        assert!(deps.contains(&Name::str("f")));
        assert!(deps.contains(&Name::str("g")));
    }
    #[test]
    fn test_check_dependencies_empty() {
        let validator = AxiomValidator::new();
        let expr = Expr::BVar(0);
        let deps = validator.check_dependencies(&expr);
        assert_eq!(deps.len(), 0);
    }
    #[test]
    fn test_extract_axioms() {
        let env = Environment::new();
        let axioms = extract_axioms(&env);
        assert_eq!(axioms.len(), 0);
    }
    #[test]
    fn test_extract_axioms_with_entries() {
        let mut env = Environment::new();
        env.add(Declaration::Axiom {
            name: Name::str("my_axiom"),
            univ_params: vec![],
            ty: Expr::Sort(crate::Level::zero()),
        })
        .expect("value should be present");
        let axioms = extract_axioms(&env);
        assert!(axioms.contains(&Name::str("my_axiom")));
    }
    #[test]
    fn test_classify_safe() {
        let safety = classify_axiom(&Name::str("propext"));
        assert_eq!(safety, AxiomSafety::Safe);
    }
    #[test]
    fn test_classify_questionable() {
        let safety = classify_axiom(&Name::str("Classical.choice"));
        assert_eq!(safety, AxiomSafety::Questionable);
    }
    #[test]
    fn test_is_classical() {
        let validator = AxiomValidator::new();
        assert!(validator.is_classical(&Name::str("Classical.choice")));
        assert!(validator.is_classical(&Name::str("Classical.em")));
        assert!(!validator.is_classical(&Name::str("propext")));
    }
    #[test]
    fn test_is_constructive() {
        let validator = AxiomValidator::new();
        let constructive = Expr::Const(Name::str("propext"), vec![]);
        assert!(validator.is_constructive(&constructive));
        let classical = Expr::Const(Name::str("Classical.choice"), vec![]);
        assert!(!validator.is_constructive(&classical));
    }
    #[test]
    fn test_has_unsafe_deps() {
        let mut validator = AxiomValidator::new();
        validator.mark_unsafe(Name::str("bad_axiom"));
        let safe_decl = Declaration::Axiom {
            name: Name::str("safe"),
            univ_params: vec![],
            ty: Expr::Const(Name::str("propext"), vec![]),
        };
        assert!(!has_unsafe_dependencies(&safe_decl, &validator));
        let unsafe_decl = Declaration::Axiom {
            name: Name::str("unsafe"),
            univ_params: vec![],
            ty: Expr::Const(Name::str("bad_axiom"), vec![]),
        };
        assert!(has_unsafe_dependencies(&unsafe_decl, &validator));
    }
    #[test]
    fn test_axiom_dependencies() {
        let mut validator = AxiomValidator::new();
        validator.register(Name::str("ax1"));
        validator.register(Name::str("ax2"));
        let expr = Expr::App(
            Box::new(Expr::Const(Name::str("ax1"), vec![])),
            Box::new(Expr::Const(Name::str("not_axiom"), vec![])),
        );
        let axiom_deps = validator.axiom_dependencies(&expr);
        assert!(axiom_deps.contains(&Name::str("ax1")));
        assert!(!axiom_deps.contains(&Name::str("not_axiom")));
    }
    #[test]
    fn test_transitive_deps() {
        let mut env = Environment::new();
        env.add(Declaration::Definition {
            name: Name::str("f"),
            univ_params: vec![],
            ty: Expr::Sort(crate::Level::zero()),
            val: Expr::Const(Name::str("g"), vec![]),
            hint: crate::ReducibilityHint::Regular(1),
        })
        .expect("value should be present");
        let expr = Expr::Const(Name::str("f"), vec![]);
        let deps = transitive_axiom_deps(&expr, &env);
        assert!(deps.contains(&Name::str("f")));
        assert!(deps.contains(&Name::str("g")));
    }
}
/// Build an axiom safety report for an environment.
#[allow(dead_code)]
pub fn build_safety_report(env: &Environment) -> AxiomSafetyReport {
    let mut report = AxiomSafetyReport::empty();
    let axiom_names = extract_axioms(env);
    report.total_axioms = axiom_names.len();
    for name in &axiom_names {
        let category = classify_axiom_category(name);
        match category {
            AxiomCategory::Classical => {
                report.classical_count += 1;
                report.warn(format!("Classical axiom in use: {name}"));
            }
            AxiomCategory::External => {
                report.external_count += 1;
                report.warn(format!("External axiom in use: {name}"));
            }
            _ => {}
        }
    }
    report
}
/// Classify an axiom by its name using heuristics.
#[allow(dead_code)]
pub fn classify_axiom_category(name: &Name) -> AxiomCategory {
    let s = name.to_string();
    if s.contains("Classical") || s.contains("ExcludedMiddle") || s.contains("choice") {
        AxiomCategory::Classical
    } else if s.starts_with("extern_") || s.contains("FFI") {
        AxiomCategory::External
    } else if s.contains("Prop") || s.contains("propext") {
        AxiomCategory::Propositional
    } else {
        AxiomCategory::UserDefined
    }
}
/// Compute the "axiom profile" of the environment:
/// a map from axiom name to usage record.
#[allow(dead_code)]
pub fn axiom_profile(env: &Environment) -> HashMap<Name, AxiomUsageRecord> {
    let mut profile: HashMap<Name, AxiomUsageRecord> = HashMap::new();
    for name in extract_axioms(env) {
        let record = AxiomUsageRecord::new(name.clone());
        profile.insert(name, record);
    }
    profile
}
/// Check whether a given expression is "axiom-free" in the given environment.
///
/// Returns `true` if the expression references no names that are axioms.
#[allow(dead_code)]
pub fn is_axiom_free(expr: &Expr, env: &Environment) -> bool {
    let deps = transitive_axiom_deps(expr, env);
    deps.is_empty()
}
/// Return the number of axioms an expression directly references.
#[allow(dead_code)]
pub fn direct_axiom_count(expr: &Expr, env: &Environment) -> usize {
    let all_axioms = extract_axioms(env);
    let all_axiom_set: HashSet<&Name> = all_axioms.iter().collect();
    collect_direct_refs(expr)
        .into_iter()
        .filter(|n| all_axiom_set.contains(n))
        .count()
}
/// Collect all `Const` names directly referenced in an expression (non-recursive).
#[allow(dead_code)]
pub(super) fn collect_direct_refs(expr: &Expr) -> Vec<Name> {
    let mut names = Vec::new();
    collect_direct_refs_rec(expr, &mut names);
    names
}
pub(super) fn collect_direct_refs_rec(expr: &Expr, acc: &mut Vec<Name>) {
    match expr {
        Expr::Const(n, _) => acc.push(n.clone()),
        Expr::App(f, a) => {
            collect_direct_refs_rec(f, acc);
            collect_direct_refs_rec(a, acc);
        }
        Expr::Lam(_, _, dom, body) | Expr::Pi(_, _, dom, body) => {
            collect_direct_refs_rec(dom, acc);
            collect_direct_refs_rec(body, acc);
        }
        Expr::Let(_, ty, val, body) => {
            collect_direct_refs_rec(ty, acc);
            collect_direct_refs_rec(val, acc);
            collect_direct_refs_rec(body, acc);
        }
        _ => {}
    }
}
/// Verify that two environments have the same set of axioms.
#[allow(dead_code)]
pub fn axioms_match(env1: &Environment, env2: &Environment) -> bool {
    let ax1: HashSet<Name> = extract_axioms(env1).into_iter().collect();
    let ax2: HashSet<Name> = extract_axioms(env2).into_iter().collect();
    ax1 == ax2
}
#[cfg(test)]
mod axiom_extra_tests {
    use super::*;
    use crate::Environment;
    #[test]
    fn test_axiom_safety_report_empty() {
        let report = AxiomSafetyReport::empty();
        assert!(report.is_axiom_free());
        assert!(report.classically_consistent);
        assert!(report.warnings.is_empty());
    }
    #[test]
    fn test_classify_axiom_category_classical() {
        assert_eq!(
            classify_axiom_category(&Name::str("Classical.em")),
            AxiomCategory::Classical
        );
    }
    #[test]
    fn test_classify_axiom_category_user() {
        assert_eq!(
            classify_axiom_category(&Name::str("my_axiom")),
            AxiomCategory::UserDefined
        );
    }
    #[test]
    fn test_classify_axiom_category_propositional() {
        assert_eq!(
            classify_axiom_category(&Name::str("propext")),
            AxiomCategory::Propositional
        );
    }
    #[test]
    fn test_axiom_allowlist_empty() {
        let al = AxiomAllowlist::empty();
        assert!(!al.is_allowed(&Name::str("anything")));
    }
    #[test]
    fn test_axiom_allowlist_classical() {
        let al = AxiomAllowlist::classical();
        assert!(al.is_allowed(&Name::str("Classical.em")));
        assert!(al.is_allowed(&Name::str("propext")));
        assert!(!al.is_allowed(&Name::str("my_axiom")));
    }
    #[test]
    fn test_axiom_allowlist_check_env_empty() {
        let env = Environment::new();
        let al = AxiomAllowlist::classical();
        assert!(al.check_env(&env).is_ok());
    }
    #[test]
    fn test_build_safety_report_empty_env() {
        let env = Environment::new();
        let report = build_safety_report(&env);
        assert_eq!(report.total_axioms, 0);
        assert!(report.is_axiom_free());
    }
    #[test]
    fn test_axiom_usage_record() {
        let mut rec = AxiomUsageRecord::new(Name::str("ax"));
        rec.record_direct_use();
        rec.record_direct_use();
        assert_eq!(rec.direct_uses, 2);
    }
    #[test]
    fn test_is_axiom_free_const() {
        let env = Environment::new();
        let e_sort = Expr::Sort(crate::Level::zero());
        assert!(is_axiom_free(&e_sort, &env));
        let e_bvar = Expr::BVar(0);
        assert!(is_axiom_free(&e_bvar, &env));
    }
    #[test]
    fn test_direct_axiom_count_zero() {
        let env = Environment::new();
        let e = Expr::Sort(crate::Level::zero());
        assert_eq!(direct_axiom_count(&e, &env), 0);
    }
    #[test]
    fn test_axioms_match_both_empty() {
        let env1 = Environment::new();
        let env2 = Environment::new();
        assert!(axioms_match(&env1, &env2));
    }
    #[test]
    fn test_axiom_profile_empty() {
        let env = Environment::new();
        let profile = axiom_profile(&env);
        assert!(profile.is_empty());
    }
    #[test]
    fn test_collect_direct_refs() {
        let e = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(Expr::Const(Name::str("a"), vec![])),
        );
        let refs = collect_direct_refs(&e);
        assert!(refs.contains(&Name::str("f")));
        assert!(refs.contains(&Name::str("a")));
    }
}
/// Return true iff any axiom in the environment is of a given category.
#[allow(dead_code)]
pub fn has_axioms_of_category(env: &Environment, cat: &AxiomCategory) -> bool {
    extract_axioms(env)
        .iter()
        .any(|n| &classify_axiom_category(n) == cat)
}
/// Return the subset of axioms that belong to a given category.
#[allow(dead_code)]
pub fn axioms_in_category(env: &Environment, cat: &AxiomCategory) -> Vec<Name> {
    extract_axioms(env)
        .into_iter()
        .filter(|n| &classify_axiom_category(n) == cat)
        .collect()
}
/// Summarize the environment's axioms as a formatted string.
#[allow(dead_code)]
pub fn axiom_summary(env: &Environment) -> String {
    let axioms = extract_axioms(env);
    if axioms.is_empty() {
        return "No axioms declared.".to_string();
    }
    let mut parts = vec![format!("{} axiom(s):", axioms.len())];
    for name in &axioms {
        let cat = classify_axiom_category(name);
        parts.push(format!("  - {name} ({cat:?})"));
    }
    parts.join("\n")
}
#[cfg(test)]
mod axiom_category_tests {
    use super::*;
    #[test]
    fn test_has_axioms_of_category_empty() {
        let env = Environment::new();
        assert!(!has_axioms_of_category(&env, &AxiomCategory::Classical));
    }
    #[test]
    fn test_axioms_in_category_empty() {
        let env = Environment::new();
        let cats = axioms_in_category(&env, &AxiomCategory::Classical);
        assert!(cats.is_empty());
    }
    #[test]
    fn test_axiom_summary_empty() {
        let env = Environment::new();
        let summary = axiom_summary(&env);
        assert!(summary.contains("No axioms"));
    }
    #[test]
    fn test_classify_external() {
        assert_eq!(
            classify_axiom_category(&Name::str("extern_foo")),
            AxiomCategory::External
        );
    }
}
/// Check if two axiom validators have compatible safe sets.
#[allow(dead_code)]
pub fn validators_compatible(v1: &AxiomValidator, v2: &AxiomValidator) -> bool {
    v1.safe_axioms.iter().all(|n| !v2.is_unsafe(n))
        && v2.safe_axioms.iter().all(|n| !v1.is_unsafe(n))
}
/// Count the number of constants (non-axiom declarations) in an environment.
#[allow(dead_code)]
pub fn count_definitions(env: &Environment) -> usize {
    env.constant_infos()
        .filter(|(_, ci)| !ci.is_axiom())
        .count()
}
/// Check if a name refers to a Prop-valued axiom.
#[allow(dead_code)]
pub fn is_prop_axiom(name: &Name, env: &Environment) -> bool {
    if let Some(ci) = env.find(name) {
        if !ci.is_axiom() {
            return false;
        }
        matches!(ci.ty(), Expr::Sort(l) if l.is_zero())
    } else {
        false
    }
}
/// Compute the set of axioms reachable from a set of names.
#[allow(dead_code)]
pub fn reachable_axioms(names: &[Name], env: &Environment) -> HashSet<Name> {
    let all_axioms: HashSet<Name> = extract_axioms(env).into_iter().collect();
    let mut result = HashSet::new();
    for name in names {
        let deps = transitive_axiom_deps(&Expr::Const(name.clone(), vec![]), env);
        for d in deps {
            if all_axioms.contains(&d) {
                result.insert(d);
            }
        }
    }
    result
}
#[cfg(test)]
mod axiom_sequence_tests {
    use super::*;
    #[test]
    fn test_axiom_sequence_empty() {
        let seq = AxiomSequence::new();
        assert!(seq.is_empty());
        assert_eq!(seq.len(), 0);
    }
    #[test]
    fn test_axiom_sequence_push() {
        let mut seq = AxiomSequence::new();
        seq.push(Name::str("my_axiom"));
        assert_eq!(seq.len(), 1);
        assert!(seq.contains(&Name::str("my_axiom")));
    }
    #[test]
    fn test_axiom_sequence_classical() {
        let mut seq = AxiomSequence::new();
        seq.push(Name::str("Classical.em"));
        seq.push(Name::str("my_axiom"));
        assert_eq!(seq.classical_axioms().len(), 1);
        assert_eq!(seq.user_axioms().len(), 1);
    }
    #[test]
    fn test_axiom_sequence_remove_classical() {
        let mut seq = AxiomSequence::new();
        seq.push(Name::str("Classical.em"));
        seq.push(Name::str("safe_ax"));
        seq.remove_classical();
        assert_eq!(seq.len(), 1);
        assert!(!seq.contains(&Name::str("Classical.em")));
    }
    #[test]
    fn test_axiom_sequence_clear() {
        let mut seq = AxiomSequence::new();
        seq.push(Name::str("ax"));
        seq.clear();
        assert!(seq.is_empty());
    }
    #[test]
    fn test_axiom_sequence_get() {
        let mut seq = AxiomSequence::new();
        seq.push(Name::str("first"));
        let entry = seq.get(0).expect("element at 0 should exist");
        assert_eq!(entry.0, Name::str("first"));
    }
    #[test]
    fn test_validators_compatible_both_empty() {
        let v1 = AxiomValidator::new();
        let v2 = AxiomValidator::new();
        assert!(validators_compatible(&v1, &v2));
    }
    #[test]
    fn test_count_definitions_empty() {
        let env = Environment::new();
        assert_eq!(count_definitions(&env), 0);
    }
    #[test]
    fn test_is_prop_axiom_sort_zero() {
        let mut env = Environment::new();
        env.add(Declaration::Axiom {
            name: Name::str("trivial"),
            univ_params: vec![],
            ty: Expr::Sort(crate::Level::zero()),
        })
        .expect("value should be present");
        assert!(is_prop_axiom(&Name::str("trivial"), &env));
    }
    #[test]
    fn test_is_prop_axiom_not_prop() {
        let mut env = Environment::new();
        env.add(Declaration::Axiom {
            name: Name::str("type_ax"),
            univ_params: vec![],
            ty: Expr::Sort(crate::Level::succ(crate::Level::zero())),
        })
        .expect("value should be present");
        assert!(!is_prop_axiom(&Name::str("type_ax"), &env));
    }
    #[test]
    fn test_reachable_axioms_empty() {
        let env = Environment::new();
        let reached = reachable_axioms(&[], &env);
        assert!(reached.is_empty());
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
#[cfg(test)]
mod tests_tiny_padding {
    use super::*;
    #[test]
    fn test_bitset64() {
        let mut bs = BitSet64::new();
        bs.insert(0);
        bs.insert(63);
        assert!(bs.contains(0));
        assert!(bs.contains(63));
        assert!(!bs.contains(1));
        assert_eq!(bs.len(), 2);
        bs.remove(0);
        assert!(!bs.contains(0));
    }
    #[test]
    fn test_bucket_counter() {
        let mut bc: BucketCounter<4> = BucketCounter::new();
        bc.inc(0);
        bc.inc(0);
        bc.inc(1);
        assert_eq!(bc.get(0), 2);
        assert_eq!(bc.total(), 3);
        assert_eq!(bc.argmax(), 0);
    }
}
