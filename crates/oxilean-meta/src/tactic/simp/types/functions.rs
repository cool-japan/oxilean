//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    NameIndex, SimpConfig, SimpDirection, SimpLemma, SimpLemmaSet, SimpPriority, SimpRegistry,
    SimpResult, SimpTheorems, SimpTrace, SimpTraceEntry, SimpTypesBuilder, SimpTypesCounterMap,
    SimpTypesExtMap, SimpTypesExtUtil, SimpTypesStateMachine, SimpTypesWindow, SimpTypesWorkQueue,
    StringTrie, TacticSimpTypesAnalysisPass, TacticSimpTypesConfig, TacticSimpTypesConfigValue,
    TacticSimpTypesDiagnostics, TacticSimpTypesDiff, TacticSimpTypesPipeline,
    TacticSimpTypesResult, TypesExtConfig3200, TypesExtConfigVal3200, TypesExtDiag3200,
    TypesExtDiff3200, TypesExtPass3200, TypesExtPipeline3200, TypesExtResult3200,
};
use oxilean_kernel::{Expr, Name};

/// Build a default simp lemma set with basic Nat/Bool/Logic rewrites.
///
/// This includes common identity laws for Nat arithmetic and Bool operations
/// that are always safe to apply left-to-right.
pub fn default_simp_lemmas() -> SimpTheorems {
    let mut db = SimpTheorems::new();
    let zero = Expr::Lit(oxilean_kernel::Literal::Nat(0));
    let one = Expr::Lit(oxilean_kernel::Literal::Nat(1));
    let true_c = Expr::Const(Name::str("true"), vec![]);
    let false_c = Expr::Const(Name::str("false"), vec![]);
    let add = Expr::Const(Name::str("Nat.add"), vec![]);
    let mul = Expr::Const(Name::str("Nat.mul"), vec![]);
    let band = Expr::Const(Name::str("Bool.and"), vec![]);
    let bor = Expr::Const(Name::str("Bool.or"), vec![]);
    let n = Expr::BVar(0);
    let mk_bin = |f: Expr, a: Expr, b: Expr| -> Expr {
        Expr::App(Box::new(Expr::App(Box::new(f), Box::new(a))), Box::new(b))
    };
    db.add_lemma(SimpLemma {
        name: Name::str("Nat.add_zero"),
        lhs: mk_bin(add.clone(), n.clone(), zero.clone()),
        rhs: n.clone(),
        proof: Expr::Const(Name::str("Nat.add_zero"), vec![]),
        priority: 1000,
        is_conditional: false,
        is_forward: true,
    });
    db.add_lemma(SimpLemma {
        name: Name::str("Nat.zero_add"),
        lhs: mk_bin(add.clone(), zero.clone(), n.clone()),
        rhs: n.clone(),
        proof: Expr::Const(Name::str("Nat.zero_add"), vec![]),
        priority: 1000,
        is_conditional: false,
        is_forward: true,
    });
    db.add_lemma(SimpLemma {
        name: Name::str("Nat.mul_one"),
        lhs: mk_bin(mul.clone(), n.clone(), one.clone()),
        rhs: n.clone(),
        proof: Expr::Const(Name::str("Nat.mul_one"), vec![]),
        priority: 1000,
        is_conditional: false,
        is_forward: true,
    });
    db.add_lemma(SimpLemma {
        name: Name::str("Nat.one_mul"),
        lhs: mk_bin(mul.clone(), one.clone(), n.clone()),
        rhs: n.clone(),
        proof: Expr::Const(Name::str("Nat.one_mul"), vec![]),
        priority: 1000,
        is_conditional: false,
        is_forward: true,
    });
    db.add_lemma(SimpLemma {
        name: Name::str("Nat.mul_zero"),
        lhs: mk_bin(mul.clone(), n.clone(), zero.clone()),
        rhs: zero.clone(),
        proof: Expr::Const(Name::str("Nat.mul_zero"), vec![]),
        priority: 1000,
        is_conditional: false,
        is_forward: true,
    });
    db.add_lemma(SimpLemma {
        name: Name::str("Nat.zero_mul"),
        lhs: mk_bin(mul, zero.clone(), n.clone()),
        rhs: zero,
        proof: Expr::Const(Name::str("Nat.zero_mul"), vec![]),
        priority: 1000,
        is_conditional: false,
        is_forward: true,
    });
    db.add_lemma(SimpLemma {
        name: Name::str("Bool.and_true"),
        lhs: mk_bin(band.clone(), n.clone(), true_c.clone()),
        rhs: n.clone(),
        proof: Expr::Const(Name::str("Bool.and_true"), vec![]),
        priority: 1000,
        is_conditional: false,
        is_forward: true,
    });
    db.add_lemma(SimpLemma {
        name: Name::str("Bool.true_and"),
        lhs: mk_bin(band.clone(), true_c.clone(), n.clone()),
        rhs: n.clone(),
        proof: Expr::Const(Name::str("Bool.true_and"), vec![]),
        priority: 1000,
        is_conditional: false,
        is_forward: true,
    });
    db.add_lemma(SimpLemma {
        name: Name::str("Bool.and_false"),
        lhs: mk_bin(band.clone(), n.clone(), false_c.clone()),
        rhs: false_c.clone(),
        proof: Expr::Const(Name::str("Bool.and_false"), vec![]),
        priority: 1000,
        is_conditional: false,
        is_forward: true,
    });
    db.add_lemma(SimpLemma {
        name: Name::str("Bool.false_and"),
        lhs: mk_bin(band, false_c.clone(), n.clone()),
        rhs: false_c.clone(),
        proof: Expr::Const(Name::str("Bool.false_and"), vec![]),
        priority: 1000,
        is_conditional: false,
        is_forward: true,
    });
    db.add_lemma(SimpLemma {
        name: Name::str("Bool.or_false"),
        lhs: mk_bin(bor.clone(), n.clone(), false_c.clone()),
        rhs: n.clone(),
        proof: Expr::Const(Name::str("Bool.or_false"), vec![]),
        priority: 1000,
        is_conditional: false,
        is_forward: true,
    });
    db.add_lemma(SimpLemma {
        name: Name::str("Bool.false_or"),
        lhs: mk_bin(bor.clone(), false_c.clone(), n.clone()),
        rhs: n.clone(),
        proof: Expr::Const(Name::str("Bool.false_or"), vec![]),
        priority: 1000,
        is_conditional: false,
        is_forward: true,
    });
    db.add_lemma(SimpLemma {
        name: Name::str("Bool.or_true"),
        lhs: mk_bin(bor.clone(), n.clone(), true_c.clone()),
        rhs: true_c.clone(),
        proof: Expr::Const(Name::str("Bool.or_true"), vec![]),
        priority: 1000,
        is_conditional: false,
        is_forward: true,
    });
    db.add_lemma(SimpLemma {
        name: Name::str("Bool.true_or"),
        lhs: mk_bin(bor, true_c.clone(), n.clone()),
        rhs: true_c,
        proof: Expr::Const(Name::str("Bool.true_or"), vec![]),
        priority: 1000,
        is_conditional: false,
        is_forward: true,
    });
    let zero = Expr::Lit(oxilean_kernel::Literal::Nat(0));
    let n = Expr::BVar(0);
    let list_nil = Expr::Const(Name::str("List.nil"), vec![]);
    let list_append = Expr::Const(Name::str("List.append"), vec![]);
    let l = Expr::BVar(0);
    db.add_lemma(SimpLemma {
        name: Name::str("List.nil_append"),
        lhs: mk_bin(list_append.clone(), list_nil.clone(), l.clone()),
        rhs: l.clone(),
        proof: Expr::Const(Name::str("List.nil_append"), vec![]),
        priority: 1000,
        is_conditional: false,
        is_forward: true,
    });
    db.add_lemma(SimpLemma {
        name: Name::str("List.append_nil"),
        lhs: mk_bin(list_append.clone(), l.clone(), list_nil.clone()),
        rhs: l.clone(),
        proof: Expr::Const(Name::str("List.append_nil"), vec![]),
        priority: 1000,
        is_conditional: false,
        is_forward: true,
    });
    let opt_none = Expr::Const(Name::str("Option.none"), vec![]);
    let opt_map = Expr::Const(Name::str("Option.map"), vec![]);
    let f_var = Expr::BVar(1);
    db.add_lemma(SimpLemma {
        name: Name::str("Option.map_none"),
        lhs: mk_bin(opt_map.clone(), f_var.clone(), opt_none.clone()),
        rhs: opt_none.clone(),
        proof: Expr::Const(Name::str("Option.map_none"), vec![]),
        priority: 1000,
        is_conditional: false,
        is_forward: true,
    });
    let and_c = Expr::Const(Name::str("And"), vec![]);
    let or_c = Expr::Const(Name::str("Or"), vec![]);
    let true_c = Expr::Const(Name::str("True"), vec![]);
    let false_c = Expr::Const(Name::str("False"), vec![]);
    let p = Expr::BVar(0);
    db.add_lemma(SimpLemma {
        name: Name::str("true_and"),
        lhs: mk_bin(and_c.clone(), true_c.clone(), p.clone()),
        rhs: p.clone(),
        proof: Expr::Const(Name::str("true_and"), vec![]),
        priority: 1000,
        is_conditional: false,
        is_forward: true,
    });
    db.add_lemma(SimpLemma {
        name: Name::str("and_true"),
        lhs: mk_bin(and_c.clone(), p.clone(), true_c.clone()),
        rhs: p.clone(),
        proof: Expr::Const(Name::str("and_true"), vec![]),
        priority: 1000,
        is_conditional: false,
        is_forward: true,
    });
    db.add_lemma(SimpLemma {
        name: Name::str("false_and"),
        lhs: mk_bin(and_c.clone(), false_c.clone(), p.clone()),
        rhs: false_c.clone(),
        proof: Expr::Const(Name::str("false_and"), vec![]),
        priority: 1000,
        is_conditional: false,
        is_forward: true,
    });
    db.add_lemma(SimpLemma {
        name: Name::str("and_false"),
        lhs: mk_bin(and_c.clone(), p.clone(), false_c.clone()),
        rhs: false_c.clone(),
        proof: Expr::Const(Name::str("and_false"), vec![]),
        priority: 1000,
        is_conditional: false,
        is_forward: true,
    });
    db.add_lemma(SimpLemma {
        name: Name::str("true_or"),
        lhs: mk_bin(or_c.clone(), true_c.clone(), p.clone()),
        rhs: true_c.clone(),
        proof: Expr::Const(Name::str("true_or"), vec![]),
        priority: 1000,
        is_conditional: false,
        is_forward: true,
    });
    db.add_lemma(SimpLemma {
        name: Name::str("or_true"),
        lhs: mk_bin(or_c.clone(), p.clone(), true_c.clone()),
        rhs: true_c.clone(),
        proof: Expr::Const(Name::str("or_true"), vec![]),
        priority: 1000,
        is_conditional: false,
        is_forward: true,
    });
    db.add_lemma(SimpLemma {
        name: Name::str("false_or"),
        lhs: mk_bin(or_c.clone(), false_c.clone(), p.clone()),
        rhs: p.clone(),
        proof: Expr::Const(Name::str("false_or"), vec![]),
        priority: 1000,
        is_conditional: false,
        is_forward: true,
    });
    db.add_lemma(SimpLemma {
        name: Name::str("or_false"),
        lhs: mk_bin(or_c.clone(), p.clone(), false_c.clone()),
        rhs: p.clone(),
        proof: Expr::Const(Name::str("or_false"), vec![]),
        priority: 1000,
        is_conditional: false,
        is_forward: true,
    });
    let not_c = Expr::Const(Name::str("Not"), vec![]);
    db.add_lemma(SimpLemma {
        name: Name::str("not_false"),
        lhs: Expr::App(Box::new(not_c.clone()), Box::new(false_c.clone())),
        rhs: true_c.clone(),
        proof: Expr::Const(Name::str("not_false"), vec![]),
        priority: 1000,
        is_conditional: false,
        is_forward: true,
    });
    db.add_lemma(SimpLemma {
        name: Name::str("not_true"),
        lhs: Expr::App(Box::new(not_c.clone()), Box::new(true_c.clone())),
        rhs: false_c.clone(),
        proof: Expr::Const(Name::str("not_true"), vec![]),
        priority: 1000,
        is_conditional: false,
        is_forward: true,
    });
    let nat_sub = Expr::Const(Name::str("Nat.sub"), vec![]);
    db.add_lemma(SimpLemma {
        name: Name::str("Nat.sub_self"),
        lhs: mk_bin(nat_sub.clone(), n.clone(), n.clone()),
        rhs: zero.clone(),
        proof: Expr::Const(Name::str("Nat.sub_self"), vec![]),
        priority: 1000,
        is_conditional: false,
        is_forward: true,
    });
    db.add_lemma(SimpLemma {
        name: Name::str("Nat.sub_zero"),
        lhs: mk_bin(nat_sub.clone(), n.clone(), zero.clone()),
        rhs: n.clone(),
        proof: Expr::Const(Name::str("Nat.sub_zero"), vec![]),
        priority: 1000,
        is_conditional: false,
        is_forward: true,
    });
    db.add_lemma(SimpLemma {
        name: Name::str("Nat.zero_sub"),
        lhs: mk_bin(nat_sub, zero.clone(), n.clone()),
        rhs: zero.clone(),
        proof: Expr::Const(Name::str("Nat.zero_sub"), vec![]),
        priority: 1000,
        is_conditional: false,
        is_forward: true,
    });
    db
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tactic::simp::types::*;
    fn mk_lemma(name: &str, lhs: Expr, rhs: Expr) -> SimpLemma {
        SimpLemma {
            name: Name::str(name),
            lhs,
            rhs,
            proof: Expr::Const(Name::str(name), vec![]),
            priority: 1000,
            is_conditional: false,
            is_forward: true,
        }
    }
    #[test]
    fn test_simp_theorems_new() {
        let st = SimpTheorems::new();
        assert_eq!(st.num_lemmas(), 0);
    }
    #[test]
    fn test_add_lemma() {
        let mut st = SimpTheorems::new();
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        st.add_lemma(mk_lemma("test", a, b));
        assert_eq!(st.num_lemmas(), 1);
    }
    #[test]
    fn test_remove_lemma() {
        let mut st = SimpTheorems::new();
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        st.add_lemma(mk_lemma("test", a, b));
        st.remove_lemma(&Name::str("test"));
        assert_eq!(st.num_lemmas(), 0);
    }
    #[test]
    fn test_config_default() {
        let config = SimpConfig::default();
        assert!(config.beta);
        assert!(config.use_default_lemmas);
    }
    #[test]
    fn test_config_only() {
        let config = SimpConfig::only();
        assert!(!config.use_default_lemmas);
        assert!(config.beta);
    }
    #[test]
    fn test_simp_result_unchanged() {
        let r = SimpResult::Unchanged;
        assert!(!r.is_simplified());
        assert!(!r.is_proved());
        assert!(r.new_expr().is_none());
    }
    #[test]
    fn test_simp_result_simplified() {
        let r = SimpResult::Simplified {
            new_expr: Expr::Const(Name::str("b"), vec![]),
            proof: None,
        };
        assert!(r.is_simplified());
        assert!(!r.is_proved());
        assert!(r.new_expr().is_some());
    }
    #[test]
    fn test_simp_result_proved() {
        let r = SimpResult::Proved(Expr::Const(Name::str("proof"), vec![]));
        assert!(r.is_simplified());
        assert!(r.is_proved());
    }
    #[test]
    fn test_merge() {
        let mut st1 = SimpTheorems::new();
        let mut st2 = SimpTheorems::new();
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        st2.add_lemma(mk_lemma("test", a, b));
        st1.merge(&st2);
        assert_eq!(st1.num_lemmas(), 1);
    }
}
#[cfg(test)]
mod extended_simp_tests {
    use super::*;
    use crate::tactic::simp::types::*;
    #[test]
    fn test_simp_priority_ordering() {
        assert!(SimpPriority::HIGH < SimpPriority::DEFAULT);
        assert!(SimpPriority::DEFAULT < SimpPriority::LOW);
    }
    #[test]
    fn test_simp_priority_display() {
        assert_eq!(format!("{}", SimpPriority::DEFAULT), "1000");
    }
    #[test]
    fn test_simp_direction_flip() {
        assert_eq!(SimpDirection::Forward.flip(), SimpDirection::Backward);
        assert_eq!(SimpDirection::Backward.flip(), SimpDirection::Forward);
    }
    #[test]
    fn test_simp_direction_is_forward() {
        assert!(SimpDirection::Forward.is_forward());
        assert!(!SimpDirection::Backward.is_forward());
    }
    #[test]
    fn test_simp_lemma_set_empty() {
        let set = SimpLemmaSet::new("test");
        assert_eq!(set.label(), "test");
        assert!(set.is_empty());
    }
    #[test]
    fn test_simp_lemma_set_add() {
        let mut set = SimpLemmaSet::new("nat");
        let lemma = SimpLemma {
            name: Name::str("test"),
            lhs: Expr::BVar(0),
            rhs: Expr::BVar(0),
            proof: Expr::Const(Name::str("rfl"), vec![]),
            priority: 1000,
            is_conditional: false,
            is_forward: true,
        };
        set.add(lemma);
        assert_eq!(set.len(), 1);
        assert!(!set.is_empty());
    }
    #[test]
    fn test_simp_registry_new() {
        let reg = SimpRegistry::new();
        assert_eq!(reg.num_sets(), 0);
        assert_eq!(reg.total_lemmas(), 0);
    }
    #[test]
    fn test_simp_registry_register_set() {
        let mut reg = SimpRegistry::new();
        let set = SimpLemmaSet::new("logic");
        reg.register_set(set);
        assert_eq!(reg.num_sets(), 1);
        assert!(reg.find_set("logic").is_some());
        assert!(reg.find_set("missing").is_none());
    }
    #[test]
    fn test_simp_trace_entry_changed() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let entry = SimpTraceEntry::new(a.clone(), b.clone(), Some(Name::str("lem")), 1);
        assert!(entry.did_change());
        assert_eq!(entry.steps, 1);
    }
    #[test]
    fn test_simp_trace_no_change() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let entry = SimpTraceEntry::new(a.clone(), a, None, 0);
        assert!(!entry.did_change());
    }
    #[test]
    fn test_simp_trace_collect() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let mut trace = SimpTrace::new();
        trace.push(SimpTraceEntry::new(
            a.clone(),
            b.clone(),
            Some(Name::str("lem1")),
            2,
        ));
        trace.push(SimpTraceEntry::new(b.clone(), a, None, 1));
        assert_eq!(trace.total_steps(), 3);
        assert_eq!(trace.num_changes(), 2);
        assert_eq!(trace.lemmas_used().len(), 1);
    }
    #[test]
    fn test_simp_priority_custom() {
        let p = SimpPriority::custom(500);
        assert!(p < SimpPriority::DEFAULT);
        assert!(p > SimpPriority::HIGH);
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
    format!("[{module}] {msg}", module = "simp_types", msg = msg.into())
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
    use crate::tactic::simp::types::*;
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
mod simptypes_ext2_tests {
    use super::*;
    use crate::tactic::simp::types::*;
    #[test]
    fn test_simptypes_ext_util_basic() {
        let mut u = SimpTypesExtUtil::new("test");
        u.push(10);
        u.push(20);
        assert_eq!(u.sum(), 30);
        assert_eq!(u.len(), 2);
    }
    #[test]
    fn test_simptypes_ext_util_min_max() {
        let mut u = SimpTypesExtUtil::new("mm");
        u.push(5);
        u.push(1);
        u.push(9);
        assert_eq!(u.min_val(), Some(1));
        assert_eq!(u.max_val(), Some(9));
    }
    #[test]
    fn test_simptypes_ext_util_flags() {
        let mut u = SimpTypesExtUtil::new("flags");
        u.set_flag(3);
        assert!(u.has_flag(3));
        assert!(!u.has_flag(2));
    }
    #[test]
    fn test_simptypes_ext_util_pop() {
        let mut u = SimpTypesExtUtil::new("pop");
        u.push(42);
        assert_eq!(u.pop(), Some(42));
        assert!(u.is_empty());
    }
    #[test]
    fn test_simptypes_ext_map_basic() {
        let mut m: SimpTypesExtMap<i32> = SimpTypesExtMap::new();
        m.insert("key", 42);
        assert_eq!(m.get("key"), Some(&42));
        assert!(m.contains("key"));
        assert!(!m.contains("other"));
    }
    #[test]
    fn test_simptypes_ext_map_get_or_default() {
        let mut m: SimpTypesExtMap<i32> = SimpTypesExtMap::new();
        m.insert("k", 5);
        assert_eq!(m.get_or_default("k"), 5);
        assert_eq!(m.get_or_default("missing"), 0);
    }
    #[test]
    fn test_simptypes_ext_map_keys_sorted() {
        let mut m: SimpTypesExtMap<i32> = SimpTypesExtMap::new();
        m.insert("z", 1);
        m.insert("a", 2);
        m.insert("m", 3);
        let keys = m.keys_sorted();
        assert_eq!(keys[0].as_str(), "a");
        assert_eq!(keys[2].as_str(), "z");
    }
    #[test]
    fn test_simptypes_window_mean() {
        let mut w = SimpTypesWindow::new(3);
        w.push(1.0);
        w.push(2.0);
        w.push(3.0);
        assert!((w.mean() - 2.0).abs() < 1e-10);
    }
    #[test]
    fn test_simptypes_window_evict() {
        let mut w = SimpTypesWindow::new(2);
        w.push(10.0);
        w.push(20.0);
        w.push(30.0);
        assert_eq!(w.len(), 2);
        assert!((w.mean() - 25.0).abs() < 1e-10);
    }
    #[test]
    fn test_simptypes_window_std_dev() {
        let mut w = SimpTypesWindow::new(10);
        for i in 0..10 {
            w.push(i as f64);
        }
        assert!(w.std_dev() > 0.0);
    }
    #[test]
    fn test_simptypes_builder_basic() {
        let b = SimpTypesBuilder::new("test")
            .add_item("a")
            .add_item("b")
            .set_config("key", "val");
        assert_eq!(b.item_count(), 2);
        assert!(b.has_config("key"));
        assert_eq!(b.get_config("key"), Some("val"));
    }
    #[test]
    fn test_simptypes_builder_summary() {
        let b = SimpTypesBuilder::new("suite").add_item("x");
        let s = b.build_summary();
        assert!(s.contains("suite"));
    }
    #[test]
    fn test_simptypes_state_machine_start() {
        let mut sm = SimpTypesStateMachine::new();
        assert!(sm.start());
        assert!(sm.state.is_running());
    }
    #[test]
    fn test_simptypes_state_machine_complete() {
        let mut sm = SimpTypesStateMachine::new();
        sm.start();
        sm.complete();
        assert!(sm.state.is_terminal());
    }
    #[test]
    fn test_simptypes_state_machine_fail() {
        let mut sm = SimpTypesStateMachine::new();
        sm.fail("oops");
        assert!(sm.state.is_terminal());
        assert_eq!(sm.state.error_msg(), Some("oops"));
    }
    #[test]
    fn test_simptypes_state_machine_no_transition_after_terminal() {
        let mut sm = SimpTypesStateMachine::new();
        sm.complete();
        assert!(!sm.start());
    }
    #[test]
    fn test_simptypes_work_queue_basic() {
        let mut wq = SimpTypesWorkQueue::new(10);
        wq.enqueue("task1".to_string());
        wq.enqueue("task2".to_string());
        assert_eq!(wq.pending_count(), 2);
        let t = wq.dequeue();
        assert_eq!(t, Some("task1".to_string()));
        assert_eq!(wq.processed_count(), 1);
    }
    #[test]
    fn test_simptypes_work_queue_capacity() {
        let mut wq = SimpTypesWorkQueue::new(2);
        wq.enqueue("a".to_string());
        wq.enqueue("b".to_string());
        assert!(wq.is_full());
        assert!(!wq.enqueue("c".to_string()));
    }
    #[test]
    fn test_simptypes_counter_map_basic() {
        let mut cm = SimpTypesCounterMap::new();
        cm.increment("apple");
        cm.increment("apple");
        cm.increment("banana");
        assert_eq!(cm.count("apple"), 2);
        assert_eq!(cm.count("banana"), 1);
        assert_eq!(cm.num_unique(), 2);
    }
    #[test]
    fn test_simptypes_counter_map_frequency() {
        let mut cm = SimpTypesCounterMap::new();
        cm.increment("a");
        cm.increment("a");
        cm.increment("b");
        assert!((cm.frequency("a") - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_simptypes_counter_map_most_common() {
        let mut cm = SimpTypesCounterMap::new();
        cm.increment("x");
        cm.increment("y");
        cm.increment("x");
        let (k, v) = cm.most_common().expect("most_common should succeed");
        assert_eq!(k.as_str(), "x");
        assert_eq!(v, 2);
    }
}
#[cfg(test)]
mod tacticsimptypes_analysis_tests {
    use super::*;
    use crate::tactic::simp::types::*;
    #[test]
    fn test_tacticsimptypes_result_ok() {
        let r = TacticSimpTypesResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticsimptypes_result_err() {
        let r = TacticSimpTypesResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticsimptypes_result_partial() {
        let r = TacticSimpTypesResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_tacticsimptypes_result_skipped() {
        let r = TacticSimpTypesResult::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_tacticsimptypes_analysis_pass_run() {
        let mut p = TacticSimpTypesAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_tacticsimptypes_analysis_pass_empty_input() {
        let mut p = TacticSimpTypesAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_tacticsimptypes_analysis_pass_success_rate() {
        let mut p = TacticSimpTypesAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_tacticsimptypes_analysis_pass_disable() {
        let mut p = TacticSimpTypesAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_tacticsimptypes_pipeline_basic() {
        let mut pipeline = TacticSimpTypesPipeline::new("main_pipeline");
        pipeline.add_pass(TacticSimpTypesAnalysisPass::new("pass1"));
        pipeline.add_pass(TacticSimpTypesAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_tacticsimptypes_pipeline_disabled_pass() {
        let mut pipeline = TacticSimpTypesPipeline::new("partial");
        let mut p = TacticSimpTypesAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(TacticSimpTypesAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_tacticsimptypes_diff_basic() {
        let mut d = TacticSimpTypesDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_tacticsimptypes_diff_summary() {
        let mut d = TacticSimpTypesDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }
    #[test]
    fn test_tacticsimptypes_config_set_get() {
        let mut cfg = TacticSimpTypesConfig::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_tacticsimptypes_config_read_only() {
        let mut cfg = TacticSimpTypesConfig::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_tacticsimptypes_config_remove() {
        let mut cfg = TacticSimpTypesConfig::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_tacticsimptypes_diagnostics_basic() {
        let mut diag = TacticSimpTypesDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_tacticsimptypes_diagnostics_max_errors() {
        let mut diag = TacticSimpTypesDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_tacticsimptypes_diagnostics_clear() {
        let mut diag = TacticSimpTypesDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_tacticsimptypes_config_value_types() {
        let b = TacticSimpTypesConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = TacticSimpTypesConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = TacticSimpTypesConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = TacticSimpTypesConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = TacticSimpTypesConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
#[cfg(test)]
mod types_ext_tests_3200 {
    use super::*;
    use crate::tactic::simp::types::*;
    #[test]
    fn test_types_ext_result_ok_3200() {
        let r = TypesExtResult3200::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_types_ext_result_err_3200() {
        let r = TypesExtResult3200::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_types_ext_result_partial_3200() {
        let r = TypesExtResult3200::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_types_ext_result_skipped_3200() {
        let r = TypesExtResult3200::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_types_ext_pass_run_3200() {
        let mut p = TypesExtPass3200::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_types_ext_pass_empty_3200() {
        let mut p = TypesExtPass3200::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_types_ext_pass_rate_3200() {
        let mut p = TypesExtPass3200::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_types_ext_pass_disable_3200() {
        let mut p = TypesExtPass3200::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_types_ext_pipeline_basic_3200() {
        let mut pipeline = TypesExtPipeline3200::new("main_pipeline");
        pipeline.add_pass(TypesExtPass3200::new("pass1"));
        pipeline.add_pass(TypesExtPass3200::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_types_ext_pipeline_disabled_3200() {
        let mut pipeline = TypesExtPipeline3200::new("partial");
        let mut p = TypesExtPass3200::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(TypesExtPass3200::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_types_ext_diff_basic_3200() {
        let mut d = TypesExtDiff3200::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_types_ext_config_set_get_3200() {
        let mut cfg = TypesExtConfig3200::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_types_ext_config_read_only_3200() {
        let mut cfg = TypesExtConfig3200::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_types_ext_config_remove_3200() {
        let mut cfg = TypesExtConfig3200::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_types_ext_diagnostics_basic_3200() {
        let mut diag = TypesExtDiag3200::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_types_ext_diagnostics_max_errors_3200() {
        let mut diag = TypesExtDiag3200::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_types_ext_diagnostics_clear_3200() {
        let mut diag = TypesExtDiag3200::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_types_ext_config_value_types_3200() {
        let b = TypesExtConfigVal3200::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = TypesExtConfigVal3200::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = TypesExtConfigVal3200::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = TypesExtConfigVal3200::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = TypesExtConfigVal3200::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
