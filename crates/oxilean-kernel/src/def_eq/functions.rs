//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::reduce::{Reducer, ReducibilityHint, TransparencyMode};
use crate::{Environment, Expr};
use std::collections::HashMap;

use super::types::{
    BatchDefEqChecker, ConfigNode, DecisionNode, DefEqChecker, DefEqConfig, DefEqStats, Either2,
    FlatSubstitution, FocusStack, LabelSet, NameIndex, NonEmptyVec, PathBuf, RewriteRule,
    RewriteRuleSet, SimpleDag, SlidingSum, SmallMap, SparseVec, StackCalc, StatSummary, Stopwatch,
    StringPool, StringTrie, TokenBucket, TransformStat, TransitiveClosure, VersionedRecord,
    WindowIterator, WriteOnce,
};

/// Standalone function for definitional equality (without environment).
pub fn is_def_eq_simple(t: &Expr, s: &Expr) -> bool {
    let env = Environment::new();
    let mut checker = DefEqChecker::new(&env);
    checker.is_def_eq(t, s)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BinderInfo, Declaration, Level, Literal, Name, ReducibilityHint};
    #[test]
    fn test_reflexivity() {
        let expr = Expr::Lit(Literal::Nat(42));
        assert!(is_def_eq_simple(&expr, &expr));
    }
    #[test]
    fn test_beta_reduction() {
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::BVar(0)),
        );
        let arg = Expr::Lit(Literal::Nat(42));
        let app = Expr::App(Box::new(lam), Box::new(arg.clone()));
        assert!(is_def_eq_simple(&app, &arg));
    }
    #[test]
    fn test_delta_reduction() {
        let mut env = Environment::new();
        let forty_two = Expr::Lit(Literal::Nat(42));
        env.add(Declaration::Definition {
            name: Name::str("answer"),
            univ_params: vec![],
            ty: Expr::Const(Name::str("Nat"), vec![]),
            val: forty_two.clone(),
            hint: ReducibilityHint::Regular(1),
        })
        .expect("value should be present");
        let mut checker = DefEqChecker::new(&env);
        let answer_const = Expr::Const(Name::str("answer"), vec![]);
        assert!(checker.is_def_eq(&answer_const, &forty_two));
    }
    #[test]
    fn test_level_equivalence() {
        let s1 = Expr::Sort(Level::max(
            Level::param(Name::str("u")),
            Level::param(Name::str("v")),
        ));
        let s2 = Expr::Sort(Level::max(
            Level::param(Name::str("v")),
            Level::param(Name::str("u")),
        ));
        assert!(is_def_eq_simple(&s1, &s2));
    }
    #[test]
    fn test_not_equal() {
        let n1 = Expr::Lit(Literal::Nat(1));
        let n2 = Expr::Lit(Literal::Nat(2));
        assert!(!is_def_eq_simple(&n1, &n2));
    }
    #[test]
    fn test_lazy_delta() {
        let mut env = Environment::new();
        let val = Expr::Lit(Literal::Nat(42));
        env.add(Declaration::Definition {
            name: Name::str("a"),
            univ_params: vec![],
            ty: Expr::Const(Name::str("Nat"), vec![]),
            val: val.clone(),
            hint: ReducibilityHint::Regular(1),
        })
        .expect("value should be present");
        env.add(Declaration::Definition {
            name: Name::str("b"),
            univ_params: vec![],
            ty: Expr::Const(Name::str("Nat"), vec![]),
            val,
            hint: ReducibilityHint::Regular(2),
        })
        .expect("value should be present");
        let mut checker = DefEqChecker::new(&env);
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        assert!(checker.is_def_eq(&a, &b));
    }
    #[test]
    fn test_app_equality() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Lit(Literal::Nat(1));
        let app1 = Expr::App(Box::new(f.clone()), Box::new(a.clone()));
        let app2 = Expr::App(Box::new(f), Box::new(a));
        assert!(is_def_eq_simple(&app1, &app2));
    }
    #[test]
    fn test_pi_equality() {
        let pi1 = Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::Sort(Level::zero())),
        );
        let pi2 = Expr::Pi(
            BinderInfo::Default,
            Name::str("y"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::Sort(Level::zero())),
        );
        assert!(is_def_eq_simple(&pi1, &pi2));
    }
}
/// Syntactic equality check (no reduction).
#[allow(dead_code)]
pub fn syntactic_eq(t: &Expr, s: &Expr) -> bool {
    match (t, s) {
        (Expr::BVar(i), Expr::BVar(j)) => i == j,
        (Expr::FVar(a), Expr::FVar(b)) => a == b,
        (Expr::Sort(l1), Expr::Sort(l2)) => crate::level::is_equivalent(l1, l2),
        (Expr::Const(n1, ls1), Expr::Const(n2, ls2)) => {
            n1 == n2
                && ls1.len() == ls2.len()
                && ls1
                    .iter()
                    .zip(ls2.iter())
                    .all(|(l1, l2)| crate::level::is_equivalent(l1, l2))
        }
        (Expr::App(f1, a1), Expr::App(f2, a2)) => syntactic_eq(f1, f2) && syntactic_eq(a1, a2),
        (Expr::Lam(_, _, ty1, b1), Expr::Lam(_, _, ty2, b2)) => {
            syntactic_eq(ty1, ty2) && syntactic_eq(b1, b2)
        }
        (Expr::Pi(_, _, ty1, b1), Expr::Pi(_, _, ty2, b2)) => {
            syntactic_eq(ty1, ty2) && syntactic_eq(b1, b2)
        }
        (Expr::Let(_, ty1, v1, b1), Expr::Let(_, ty2, v2, b2)) => {
            syntactic_eq(ty1, ty2) && syntactic_eq(v1, v2) && syntactic_eq(b1, b2)
        }
        (Expr::Lit(l1), Expr::Lit(l2)) => l1 == l2,
        (Expr::Proj(n1, i1, e1), Expr::Proj(n2, i2, e2)) => {
            n1 == n2 && i1 == i2 && syntactic_eq(e1, e2)
        }
        _ => false,
    }
}
/// Check definitional equality with a specific transparency mode.
#[allow(dead_code)]
pub fn is_def_eq_with_mode(t: &Expr, s: &Expr, env: &Environment, mode: TransparencyMode) -> bool {
    let mut checker = DefEqChecker::new(env);
    checker.set_transparency(mode);
    checker.is_def_eq(t, s)
}
/// Check definitional equality for a sequence of argument lists.
#[allow(dead_code)]
pub fn is_def_eq_args(args1: &[Expr], args2: &[Expr], checker: &mut DefEqChecker<'_>) -> bool {
    if args1.len() != args2.len() {
        return false;
    }
    args1
        .iter()
        .zip(args2.iter())
        .all(|(a, b)| checker.is_def_eq(a, b))
}
#[cfg(test)]
mod extended_def_eq_tests {
    use super::*;
    use crate::{BinderInfo, Level, Literal, Name};
    #[test]
    fn test_syntactic_eq_lit() {
        let a = Expr::Lit(Literal::Nat(5));
        let b = Expr::Lit(Literal::Nat(5));
        let c = Expr::Lit(Literal::Nat(6));
        assert!(syntactic_eq(&a, &b));
        assert!(!syntactic_eq(&a, &c));
    }
    #[test]
    fn test_syntactic_eq_bvar() {
        assert!(syntactic_eq(&Expr::BVar(0), &Expr::BVar(0)));
        assert!(!syntactic_eq(&Expr::BVar(0), &Expr::BVar(1)));
    }
    #[test]
    fn test_syntactic_eq_const() {
        let a = Expr::Const(Name::str("Nat"), vec![]);
        let b = Expr::Const(Name::str("Nat"), vec![]);
        let c = Expr::Const(Name::str("Bool"), vec![]);
        assert!(syntactic_eq(&a, &b));
        assert!(!syntactic_eq(&a, &c));
    }
    #[test]
    fn test_batch_checker_basic() {
        let env = Environment::new();
        let mut batch = BatchDefEqChecker::new(&env);
        let a = Expr::Lit(Literal::Nat(1));
        let b = Expr::Lit(Literal::Nat(1));
        let c = Expr::Lit(Literal::Nat(2));
        assert!(batch.check(&a, &b));
        assert!(!batch.check(&a, &c));
    }
    #[test]
    fn test_batch_check_all() {
        let env = Environment::new();
        let mut batch = BatchDefEqChecker::new(&env);
        let pairs = vec![
            (Expr::Lit(Literal::Nat(1)), Expr::Lit(Literal::Nat(1))),
            (Expr::Lit(Literal::Nat(2)), Expr::Lit(Literal::Nat(2))),
        ];
        assert!(batch.check_all(&pairs));
    }
    #[test]
    fn test_batch_check_all_fail() {
        let env = Environment::new();
        let mut batch = BatchDefEqChecker::new(&env);
        let pairs = vec![
            (Expr::Lit(Literal::Nat(1)), Expr::Lit(Literal::Nat(1))),
            (Expr::Lit(Literal::Nat(2)), Expr::Lit(Literal::Nat(3))),
        ];
        assert!(!batch.check_all(&pairs));
    }
    #[test]
    fn test_batch_count_equal() {
        let env = Environment::new();
        let mut batch = BatchDefEqChecker::new(&env);
        let pairs = vec![
            (Expr::Lit(Literal::Nat(1)), Expr::Lit(Literal::Nat(1))),
            (Expr::Lit(Literal::Nat(2)), Expr::Lit(Literal::Nat(3))),
            (Expr::Lit(Literal::Nat(4)), Expr::Lit(Literal::Nat(4))),
        ];
        assert_eq!(batch.count_equal(&pairs), 2);
    }
    #[test]
    fn test_def_eq_config_default() {
        let cfg = DefEqConfig::default();
        assert!(cfg.proof_irrelevance);
        assert!(cfg.eta);
        assert!(cfg.lazy_delta);
    }
    #[test]
    fn test_def_eq_config_opaque() {
        let cfg = DefEqConfig::opaque();
        assert!(!cfg.lazy_delta);
    }
    #[test]
    fn test_def_eq_stats_hit_rate() {
        let stats = DefEqStats {
            cache_hits: 8,
            cache_misses: 2,
            ..Default::default()
        };
        assert!((stats.cache_hit_rate() - 0.8).abs() < 1e-10);
        assert_eq!(stats.total_cache_accesses(), 10);
    }
    #[test]
    fn test_def_eq_stats_no_accesses() {
        let stats = DefEqStats::default();
        assert_eq!(stats.cache_hit_rate(), 1.0);
        assert_eq!(stats.total_reductions(), 0);
    }
    #[test]
    fn test_is_def_eq_args_equal() {
        let env = Environment::new();
        let mut checker = DefEqChecker::new(&env);
        let args1 = vec![Expr::Lit(Literal::Nat(1)), Expr::Lit(Literal::Nat(2))];
        let args2 = vec![Expr::Lit(Literal::Nat(1)), Expr::Lit(Literal::Nat(2))];
        assert!(is_def_eq_args(&args1, &args2, &mut checker));
    }
    #[test]
    fn test_is_def_eq_args_different_lengths() {
        let env = Environment::new();
        let mut checker = DefEqChecker::new(&env);
        let args1 = vec![Expr::Lit(Literal::Nat(1))];
        let args2 = vec![Expr::Lit(Literal::Nat(1)), Expr::Lit(Literal::Nat(2))];
        assert!(!is_def_eq_args(&args1, &args2, &mut checker));
    }
    #[test]
    fn test_syntactic_eq_app() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let x = Expr::Lit(Literal::Nat(1));
        let app1 = Expr::App(Box::new(f.clone()), Box::new(x.clone()));
        let app2 = Expr::App(Box::new(f), Box::new(x));
        assert!(syntactic_eq(&app1, &app2));
    }
    #[test]
    fn test_syntactic_eq_pi() {
        let ty = Expr::Sort(Level::zero());
        let pi1 = Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(ty.clone()),
            Box::new(ty.clone()),
        );
        let pi2 = Expr::Pi(
            BinderInfo::Default,
            Name::str("y"),
            Box::new(ty.clone()),
            Box::new(ty),
        );
        assert!(syntactic_eq(&pi1, &pi2));
    }
    #[test]
    fn test_batch_check_any() {
        let env = Environment::new();
        let mut batch = BatchDefEqChecker::new(&env);
        let pairs = vec![
            (Expr::Lit(Literal::Nat(1)), Expr::Lit(Literal::Nat(2))),
            (Expr::Lit(Literal::Nat(3)), Expr::Lit(Literal::Nat(3))),
        ];
        assert!(batch.check_any(&pairs));
    }
    /// Build a minimal environment with:
    ///   - `TrueProp : Prop` (an axiom with type `Sort 0`)
    ///   - `proof1 : TrueProp` (an axiom)
    ///   - `proof2 : TrueProp` (a different axiom for the same proposition)
    fn make_proof_irrel_env() -> Environment {
        use crate::{BinderInfo, Declaration, ReducibilityHint};
        let mut env = Environment::new();
        let prop = Expr::Sort(Level::zero());
        env.add(Declaration::Axiom {
            name: Name::str("TrueProp"),
            univ_params: vec![],
            ty: prop.clone(),
        })
        .expect("value should be present");
        env.add(Declaration::Axiom {
            name: Name::str("proof1"),
            univ_params: vec![],
            ty: Expr::Const(Name::str("TrueProp"), vec![]),
        })
        .expect("value should be present");
        env.add(Declaration::Axiom {
            name: Name::str("proof2"),
            univ_params: vec![],
            ty: Expr::Const(Name::str("TrueProp"), vec![]),
        })
        .expect("value should be present");
        env.add(Declaration::Axiom {
            name: Name::str("FalseProp"),
            univ_params: vec![],
            ty: prop.clone(),
        })
        .expect("value should be present");
        env.add(Declaration::Axiom {
            name: Name::str("bad_proof"),
            univ_params: vec![],
            ty: Expr::Const(Name::str("FalseProp"), vec![]),
        })
        .expect("value should be present");
        env.add(Declaration::Axiom {
            name: Name::str("SetProp"),
            univ_params: vec![],
            ty: Expr::Sort(Level::succ(Level::zero())),
        })
        .expect("value should be present");
        env.add(Declaration::Axiom {
            name: Name::str("set_elem1"),
            univ_params: vec![],
            ty: Expr::Const(Name::str("SetProp"), vec![]),
        })
        .expect("value should be present");
        env.add(Declaration::Axiom {
            name: Name::str("set_elem2"),
            univ_params: vec![],
            ty: Expr::Const(Name::str("SetProp"), vec![]),
        })
        .expect("value should be present");
        let _ = BinderInfo::Default;
        let _ = ReducibilityHint::Regular(1);
        env
    }
    #[test]
    fn test_proof_irrelevance_same_prop() {
        let env = make_proof_irrel_env();
        let mut checker = DefEqChecker::new(&env);
        let p1 = Expr::Const(Name::str("proof1"), vec![]);
        let p2 = Expr::Const(Name::str("proof2"), vec![]);
        assert!(
            checker.is_def_eq(&p1, &p2),
            "Two proofs of TrueProp must be definitionally equal by proof irrelevance"
        );
    }
    #[test]
    fn test_proof_irrelevance_different_props() {
        let env = make_proof_irrel_env();
        let mut checker = DefEqChecker::new(&env);
        let p1 = Expr::Const(Name::str("proof1"), vec![]);
        let bad = Expr::Const(Name::str("bad_proof"), vec![]);
        assert!(
            !checker.is_def_eq(&p1, &bad),
            "Proofs of different Props must NOT be identified by proof irrelevance"
        );
    }
    #[test]
    fn test_proof_irrelevance_disabled() {
        let env = make_proof_irrel_env();
        let mut checker = DefEqChecker::new(&env);
        checker.set_proof_irrelevance(false);
        let p1 = Expr::Const(Name::str("proof1"), vec![]);
        let p2 = Expr::Const(Name::str("proof2"), vec![]);
        assert!(
            !checker.is_def_eq(&p1, &p2),
            "With proof irrelevance disabled, distinct axioms must not be def-eq"
        );
    }
    #[test]
    fn test_proof_irrelevance_does_not_apply_to_types() {
        let env = make_proof_irrel_env();
        let mut checker = DefEqChecker::new(&env);
        let e1 = Expr::Const(Name::str("set_elem1"), vec![]);
        let e2 = Expr::Const(Name::str("set_elem2"), vec![]);
        assert!(
            !checker.is_def_eq(&e1, &e2),
            "Terms of Sort 1 (Type) must NOT be identified by proof irrelevance"
        );
    }
    #[test]
    fn test_proof_irrelevance_self() {
        let env = make_proof_irrel_env();
        let mut checker = DefEqChecker::new(&env);
        let p1 = Expr::Const(Name::str("proof1"), vec![]);
        assert!(checker.is_def_eq(&p1, &p1));
    }
}
/// Version tag for this module.
#[allow(dead_code)]
pub const MODULE_VERSION: &str = "1.0.0";
/// Marker trait for types that can be used in module-specific contexts.
///
/// This is a doc-only trait providing context about the module's design philosophy.
#[allow(dead_code)]
pub trait ModuleMarker: Sized + Clone + std::fmt::Debug {}
/// Generic result type for operations in this module.
#[allow(dead_code)]
pub type ModuleResult<T> = Result<T, String>;
/// Create a module-level error string.
#[allow(dead_code)]
pub fn module_err(msg: impl Into<String>) -> String {
    format!("[{module}] {msg}", module = "def_eq", msg = msg.into())
}
/// Compute the Levenshtein distance between two string slices.
///
/// This is used for providing "did you mean?" suggestions in error messages.
#[allow(dead_code)]
pub fn levenshtein_distance(a: &str, b: &str) -> usize {
    let la = a.len();
    let lb = b.len();
    if la == 0 {
        return lb;
    }
    if lb == 0 {
        return la;
    }
    let mut row: Vec<usize> = (0..=lb).collect();
    for (i, ca) in a.chars().enumerate() {
        let mut prev = i;
        row[0] = i + 1;
        for (j, cb) in b.chars().enumerate() {
            let old = row[j + 1];
            row[j + 1] = if ca == cb {
                prev
            } else {
                1 + old.min(row[j]).min(prev)
            };
            prev = old;
        }
    }
    row[lb]
}
/// Find the closest match in a list of candidates using Levenshtein distance.
///
/// Returns None if candidates is empty.
#[allow(dead_code)]
pub fn closest_match<'a>(query: &str, candidates: &[&'a str]) -> Option<&'a str> {
    candidates
        .iter()
        .min_by_key(|&&c| levenshtein_distance(query, c))
        .copied()
}
/// Format a list of names for display in an error message.
#[allow(dead_code)]
pub fn format_name_list(names: &[&str]) -> String {
    match names.len() {
        0 => "(none)".to_string(),
        1 => names[0].to_string(),
        2 => format!("{} and {}", names[0], names[1]),
        _ => {
            let mut s = names[..names.len() - 1].join(", ");
            s.push_str(", and ");
            s.push_str(names[names.len() - 1]);
            s
        }
    }
}
/// Collect all strings from a trie node.
pub(super) fn collect_strings(node: &StringTrie, results: &mut Vec<String>) {
    if let Some(v) = &node.value {
        results.push(v.clone());
    }
    for child in node.children.values() {
        collect_strings(child, results);
    }
}
#[cfg(test)]
mod utility_tests {
    use super::*;
    #[test]
    fn test_levenshtein_same_string() {
        assert_eq!(levenshtein_distance("hello", "hello"), 0);
    }
    #[test]
    fn test_levenshtein_empty() {
        assert_eq!(levenshtein_distance("", "abc"), 3);
        assert_eq!(levenshtein_distance("abc", ""), 3);
    }
    #[test]
    fn test_levenshtein_one_edit() {
        assert_eq!(levenshtein_distance("cat", "bat"), 1);
        assert_eq!(levenshtein_distance("cat", "cats"), 1);
        assert_eq!(levenshtein_distance("cats", "cat"), 1);
    }
    #[test]
    fn test_closest_match_found() {
        let candidates = &["intro", "intros", "exact", "apply"];
        let result = closest_match("intoo", candidates);
        assert!(result.is_some());
        assert_eq!(result.expect("result should be valid"), "intro");
    }
    #[test]
    fn test_closest_match_empty() {
        let result = closest_match("x", &[]);
        assert!(result.is_none());
    }
    #[test]
    fn test_format_name_list_empty() {
        assert_eq!(format_name_list(&[]), "(none)");
    }
    #[test]
    fn test_format_name_list_one() {
        assert_eq!(format_name_list(&["foo"]), "foo");
    }
    #[test]
    fn test_format_name_list_two() {
        assert_eq!(format_name_list(&["a", "b"]), "a and b");
    }
    #[test]
    fn test_format_name_list_many() {
        let result = format_name_list(&["a", "b", "c"]);
        assert!(result.contains("a"));
        assert!(result.contains("b"));
        assert!(result.contains("c"));
        assert!(result.contains("and"));
    }
    #[test]
    fn test_string_trie_insert_contains() {
        let mut trie = StringTrie::new();
        trie.insert("hello");
        trie.insert("help");
        trie.insert("world");
        assert!(trie.contains("hello"));
        assert!(trie.contains("help"));
        assert!(trie.contains("world"));
        assert!(!trie.contains("hell"));
        assert_eq!(trie.len(), 3);
    }
    #[test]
    fn test_string_trie_starts_with() {
        let mut trie = StringTrie::new();
        trie.insert("hello");
        trie.insert("help");
        trie.insert("world");
        let results = trie.starts_with("hel");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_string_trie_empty_prefix() {
        let mut trie = StringTrie::new();
        trie.insert("a");
        trie.insert("b");
        let results = trie.starts_with("");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_name_index_basic() {
        let mut idx = NameIndex::new();
        let id1 = idx.insert("Nat");
        let id2 = idx.insert("Bool");
        let id3 = idx.insert("Nat");
        assert_eq!(id1, id3);
        assert_ne!(id1, id2);
        assert_eq!(idx.len(), 2);
    }
    #[test]
    fn test_name_index_get() {
        let mut idx = NameIndex::new();
        let id = idx.insert("test");
        assert_eq!(idx.get_id("test"), Some(id));
        assert_eq!(idx.get_name(id), Some("test"));
        assert_eq!(idx.get_id("missing"), None);
    }
    #[test]
    fn test_name_index_empty() {
        let idx = NameIndex::new();
        assert!(idx.is_empty());
        assert_eq!(idx.len(), 0);
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
