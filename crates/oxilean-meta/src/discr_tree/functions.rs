//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    DiscrTree, DiscrTreeBuilder, DiscrTreeKey, DiscrTreePath, DiscrTreeStats, DiscrTreeView,
    ExprFingerprint, InstanceEntry, InstanceIndex, KeyPattern, LayeredDiscrTree, LemmaAutocomplete,
    MatchInfo, MultiDiscrTree, NameIndex, PriorityDiscrTree, ScoredResult, SimpLemmaEntry,
    SimpLemmaIndex, StringTrie, TopKConfig, TrackedDiscrTree,
};
use oxilean_kernel::{Expr, Literal, Name};

/// Encode an expression as a sequence of discrimination tree keys.
///
/// The encoding traverses the expression in a pre-order manner,
/// producing keys that represent the structure.
pub fn encode_expr(expr: &Expr) -> Vec<DiscrTreeKey> {
    let mut keys = Vec::new();
    encode_expr_impl(expr, &mut keys);
    keys
}
/// Internal encoding implementation.
pub(super) fn encode_expr_impl(expr: &Expr, keys: &mut Vec<DiscrTreeKey>) {
    match expr {
        Expr::Const(name, _levels) => {
            keys.push(DiscrTreeKey::Const(name.clone(), 0));
        }
        Expr::App(_, _) => {
            let (head, args) = collect_app(expr);
            match head {
                Expr::Const(name, _) => {
                    keys.push(DiscrTreeKey::Const(name.clone(), args.len() as u32));
                    for arg in &args {
                        encode_expr_impl(arg, keys);
                    }
                }
                Expr::FVar(fid) => {
                    keys.push(DiscrTreeKey::FVar(fid.0));
                    for arg in &args {
                        encode_expr_impl(arg, keys);
                    }
                }
                _ => {
                    keys.push(DiscrTreeKey::Star);
                }
            }
        }
        Expr::FVar(fid) => {
            keys.push(DiscrTreeKey::FVar(fid.0));
        }
        Expr::Sort(_) => {
            keys.push(DiscrTreeKey::Sort);
        }
        Expr::Lit(lit) => {
            keys.push(DiscrTreeKey::Lit(lit.clone()));
        }
        Expr::Lam(_, _, _, _) => {
            keys.push(DiscrTreeKey::Lambda);
        }
        Expr::Pi(_, _, ty, body) => {
            keys.push(DiscrTreeKey::Pi);
            encode_expr_impl(ty, keys);
            encode_expr_impl(body, keys);
        }
        Expr::Proj(name, idx, inner) => {
            keys.push(DiscrTreeKey::Proj(name.clone(), *idx));
            encode_expr_impl(inner, keys);
        }
        Expr::BVar(_) | Expr::Let(_, _, _, _) => {
            keys.push(DiscrTreeKey::Star);
        }
    }
}
/// Collect head and args of nested application.
pub(super) fn collect_app(expr: &Expr) -> (&Expr, Vec<&Expr>) {
    let mut args = Vec::new();
    let mut e = expr;
    while let Expr::App(f, a) = e {
        args.push(a.as_ref());
        e = f;
    }
    args.reverse();
    (e, args)
}
/// Get the size of the subtree represented by a key.
///
/// For compound keys (Const with arity, Pi), this is 1 + the
/// total size of their children's subtrees. For atomic keys, it's 1.
pub(super) fn subtree_size(key: &DiscrTreeKey) -> usize {
    match key {
        DiscrTreeKey::Const(_, arity) => 1 + *arity as usize,
        DiscrTreeKey::Pi => 3,
        DiscrTreeKey::Proj(_, _) => 2,
        _ => 1,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::discr_tree::*;
    use oxilean_kernel::Level;
    #[test]
    fn test_create_tree() {
        let tree: DiscrTree<String> = DiscrTree::new();
        assert!(tree.is_empty());
        assert_eq!(tree.num_entries(), 0);
    }
    #[test]
    fn test_insert_and_find_const() {
        let mut tree: DiscrTree<String> = DiscrTree::new();
        let expr = Expr::Const(Name::str("Nat"), vec![]);
        tree.insert(&expr, "Nat type".to_string());
        let results = tree.find(&expr);
        assert_eq!(results.len(), 1);
        assert_eq!(*results[0], "Nat type");
    }
    #[test]
    fn test_insert_and_find_app() {
        let mut tree: DiscrTree<String> = DiscrTree::new();
        let app = Expr::App(
            Box::new(Expr::Const(Name::str("List"), vec![])),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
        );
        tree.insert(&app, "List Nat".to_string());
        let results = tree.find(&app);
        assert_eq!(results.len(), 1);
        assert_eq!(*results[0], "List Nat");
    }
    #[test]
    fn test_find_no_match() {
        let mut tree: DiscrTree<String> = DiscrTree::new();
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        tree.insert(&nat, "Nat".to_string());
        let bool_expr = Expr::Const(Name::str("Bool"), vec![]);
        let results = tree.find(&bool_expr);
        assert!(results.is_empty());
    }
    #[test]
    fn test_star_matching() {
        let mut tree: DiscrTree<String> = DiscrTree::new();
        let star_expr = Expr::BVar(0);
        tree.insert(&star_expr, "anything".to_string());
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let results = tree.find(&nat);
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_multiple_entries() {
        let mut tree: DiscrTree<u32> = DiscrTree::new();
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        tree.insert(&nat, 1);
        tree.insert(&nat, 2);
        let results = tree.find(&nat);
        assert_eq!(results.len(), 2);
        assert_eq!(tree.num_entries(), 2);
    }
    #[test]
    fn test_lit_matching() {
        let mut tree: DiscrTree<String> = DiscrTree::new();
        let lit42 = Expr::Lit(Literal::Nat(42));
        tree.insert(&lit42, "forty-two".to_string());
        let results = tree.find(&lit42);
        assert_eq!(results.len(), 1);
        let lit43 = Expr::Lit(Literal::Nat(43));
        let results2 = tree.find(&lit43);
        assert!(results2.is_empty());
    }
    #[test]
    fn test_sort_matching() {
        let mut tree: DiscrTree<String> = DiscrTree::new();
        let sort0 = Expr::Sort(Level::zero());
        tree.insert(&sort0, "Prop".to_string());
        let sort1 = Expr::Sort(Level::succ(Level::zero()));
        let results = tree.find(&sort1);
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_encode_expr() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let keys = encode_expr(&nat);
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0], DiscrTreeKey::Const(Name::str("Nat"), 0));
    }
    #[test]
    fn test_encode_app() {
        let app = Expr::App(
            Box::new(Expr::Const(Name::str("List"), vec![])),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
        );
        let keys = encode_expr(&app);
        assert_eq!(keys.len(), 2);
        assert_eq!(keys[0], DiscrTreeKey::Const(Name::str("List"), 1));
        assert_eq!(keys[1], DiscrTreeKey::Const(Name::str("Nat"), 0));
    }
    #[test]
    fn test_encode_pi() {
        let pi = Expr::Pi(
            oxilean_kernel::BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
        );
        let keys = encode_expr(&pi);
        assert_eq!(keys.len(), 3);
        assert_eq!(keys[0], DiscrTreeKey::Pi);
    }
    #[test]
    fn test_all_values() {
        let mut tree: DiscrTree<u32> = DiscrTree::new();
        tree.insert(&Expr::Const(Name::str("A"), vec![]), 1);
        tree.insert(&Expr::Const(Name::str("B"), vec![]), 2);
        tree.insert(&Expr::Const(Name::str("C"), vec![]), 3);
        let all = tree.all_values();
        assert_eq!(all.len(), 3);
    }
    #[test]
    fn test_clear() {
        let mut tree: DiscrTree<u32> = DiscrTree::new();
        tree.insert(&Expr::Const(Name::str("A"), vec![]), 1);
        assert_eq!(tree.num_entries(), 1);
        tree.clear();
        assert!(tree.is_empty());
    }
    #[test]
    fn test_proj_encoding() {
        let proj = Expr::Proj(
            Name::str("Prod"),
            0,
            Box::new(Expr::Const(Name::str("x"), vec![])),
        );
        let keys = encode_expr(&proj);
        assert_eq!(keys.len(), 2);
        assert_eq!(keys[0], DiscrTreeKey::Proj(Name::str("Prod"), 0));
    }
}
/// Utilities for working with discrimination tree keys.
#[allow(dead_code)]
pub mod key_utils {
    use super::DiscrTreeKey;
    use oxilean_kernel::Name;
    /// Check if a key is a wildcard (Star).
    pub fn is_star(key: &DiscrTreeKey) -> bool {
        matches!(key, DiscrTreeKey::Star)
    }
    /// Check if a key is a constant.
    pub fn is_const(key: &DiscrTreeKey) -> bool {
        matches!(key, DiscrTreeKey::Const(_, _))
    }
    /// Check if a key is a literal.
    pub fn is_literal(key: &DiscrTreeKey) -> bool {
        matches!(key, DiscrTreeKey::Lit(_))
    }
    /// Check if a key is a sort.
    pub fn is_sort(key: &DiscrTreeKey) -> bool {
        matches!(key, DiscrTreeKey::Sort)
    }
    /// Get the arity of a key.
    pub fn arity(key: &DiscrTreeKey) -> u32 {
        match key {
            DiscrTreeKey::Const(_, n) => *n,
            _ => 0,
        }
    }
    /// Get the name from a Const key, if any.
    pub fn const_name(key: &DiscrTreeKey) -> Option<&Name> {
        match key {
            DiscrTreeKey::Const(name, _) => Some(name),
            _ => None,
        }
    }
    /// Check if two keys could possibly match.
    pub fn could_match(stored: &DiscrTreeKey, query: &DiscrTreeKey) -> bool {
        if stored == query {
            return true;
        }
        if matches!(stored, DiscrTreeKey::Star) || matches!(query, DiscrTreeKey::Star) {
            return true;
        }
        false
    }
}
#[cfg(test)]
mod extended_discr_tests {
    use super::*;
    use crate::discr_tree::*;
    #[test]
    fn test_discr_tree_path_from_expr() {
        let expr = Expr::Const(Name::str("Nat"), vec![]);
        let path = DiscrTreePath::from_expr(&expr);
        assert_eq!(path.len(), 1);
        assert!(!path.is_empty());
    }
    #[test]
    fn test_discr_tree_path_prefix() {
        let short = DiscrTreePath(vec![DiscrTreeKey::Sort]);
        let long = DiscrTreePath(vec![DiscrTreeKey::Sort, DiscrTreeKey::Star]);
        assert!(short.is_prefix_of(&long));
        assert!(!long.is_prefix_of(&short));
    }
    #[test]
    fn test_discr_tree_path_keys() {
        let expr = Expr::Sort(oxilean_kernel::Level::zero());
        let path = DiscrTreePath::from_expr(&expr);
        assert_eq!(path.keys(), &[DiscrTreeKey::Sort]);
    }
    #[test]
    fn test_tracked_discr_tree_hit_rate() {
        let mut tree: TrackedDiscrTree<u32> = TrackedDiscrTree::new();
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        tree.insert(&nat, 1);
        let _ = tree.find(&nat);
        let _ = tree.find(&Expr::Const(Name::str("Bool"), vec![]));
        assert_eq!(tree.num_queries(), 2);
        assert_eq!(tree.num_hits(), 1);
        assert_eq!(tree.num_misses(), 1);
        assert!((tree.hit_rate() - 0.5).abs() < 1e-10);
    }
    #[test]
    fn test_tracked_discr_tree_reset_stats() {
        let mut tree: TrackedDiscrTree<u32> = TrackedDiscrTree::new();
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        tree.insert(&nat, 1);
        let _ = tree.find(&nat);
        assert_eq!(tree.num_queries(), 1);
        tree.reset_stats();
        assert_eq!(tree.num_queries(), 0);
        assert_eq!(tree.num_hits(), 0);
    }
    #[test]
    fn test_tracked_discr_tree_no_queries() {
        let tree: TrackedDiscrTree<u32> = TrackedDiscrTree::new();
        assert_eq!(tree.hit_rate(), 0.0);
    }
    #[test]
    fn test_tracked_clear() {
        let mut tree: TrackedDiscrTree<u32> = TrackedDiscrTree::new();
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        tree.insert(&nat, 42);
        let _ = tree.find(&nat);
        tree.clear();
        assert_eq!(tree.num_entries(), 0);
        assert_eq!(tree.num_queries(), 0);
    }
    #[test]
    fn test_multi_discr_tree() {
        let mut tree: MultiDiscrTree<u32> = MultiDiscrTree::new();
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let bool_e = Expr::Const(Name::str("Bool"), vec![]);
        tree.insert_multi(&[nat.clone(), bool_e.clone()], 42);
        assert_eq!(tree.num_entries(), 2);
        let results = tree.find_any(&[nat]);
        assert_eq!(results.len(), 1);
        assert_eq!(*results[0], 42);
    }
    #[test]
    fn test_key_utils_is_star() {
        assert!(key_utils::is_star(&DiscrTreeKey::Star));
        assert!(!key_utils::is_star(&DiscrTreeKey::Sort));
    }
    #[test]
    fn test_key_utils_arity() {
        assert_eq!(key_utils::arity(&DiscrTreeKey::Const(Name::str("f"), 3)), 3);
        assert_eq!(key_utils::arity(&DiscrTreeKey::Sort), 0);
    }
    #[test]
    fn test_key_utils_could_match() {
        let star = DiscrTreeKey::Star;
        let sort = DiscrTreeKey::Sort;
        let nat = DiscrTreeKey::Const(Name::str("Nat"), 0);
        assert!(key_utils::could_match(&star, &sort));
        assert!(key_utils::could_match(&nat, &nat));
        assert!(!key_utils::could_match(&sort, &nat));
    }
    #[test]
    fn test_key_utils_const_name() {
        let key = DiscrTreeKey::Const(Name::str("Nat"), 0);
        assert_eq!(key_utils::const_name(&key), Some(&Name::str("Nat")));
        assert_eq!(key_utils::const_name(&DiscrTreeKey::Sort), None);
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
    format!("[{module}] {msg}", module = "discr_tree", msg = msg.into())
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
    use crate::discr_tree::*;
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
/// Serialize a key to a compact string representation.
///
/// Used for debugging and human-readable output.
#[allow(dead_code)]
pub fn key_to_string(key: &DiscrTreeKey) -> String {
    match key {
        DiscrTreeKey::Const(name, arity) => format!("C({},{})", name, arity),
        DiscrTreeKey::FVar(id) => format!("F({})", id),
        DiscrTreeKey::Lit(lit) => format!("L({:?})", lit),
        DiscrTreeKey::Sort => "Sort".to_string(),
        DiscrTreeKey::Lambda => "Lam".to_string(),
        DiscrTreeKey::Pi => "Pi".to_string(),
        DiscrTreeKey::Proj(name, idx) => format!("P({},{})", name, idx),
        DiscrTreeKey::Star => "*".to_string(),
    }
}
/// Serialize a key sequence to a human-readable path string.
#[allow(dead_code)]
pub fn keys_to_path_string(keys: &[DiscrTreeKey]) -> String {
    keys.iter()
        .map(key_to_string)
        .collect::<Vec<_>>()
        .join(" -> ")
}
/// Check if a key sequence contains any wildcards (Stars).
#[allow(dead_code)]
pub fn has_wildcards(keys: &[DiscrTreeKey]) -> bool {
    keys.iter().any(|k| matches!(k, DiscrTreeKey::Star))
}
/// Count the number of wildcards in a key sequence.
#[allow(dead_code)]
pub fn count_wildcards(keys: &[DiscrTreeKey]) -> usize {
    keys.iter()
        .filter(|k| matches!(k, DiscrTreeKey::Star))
        .count()
}
/// Compute a specificity score for a key sequence.
///
/// More specific sequences (fewer wildcards, deeper structure) get higher scores.
#[allow(dead_code)]
pub fn specificity_score(keys: &[DiscrTreeKey]) -> i64 {
    let mut score = 0i64;
    for key in keys {
        match key {
            DiscrTreeKey::Star => score -= 10,
            DiscrTreeKey::Sort | DiscrTreeKey::Lambda | DiscrTreeKey::Pi => score += 1,
            DiscrTreeKey::Const(_, arity) => score += 5 + *arity as i64,
            DiscrTreeKey::FVar(_) => score += 3,
            DiscrTreeKey::Lit(_) => score += 7,
            DiscrTreeKey::Proj(_, _) => score += 4,
        }
    }
    score
}
/// Compute the depth of an expression tree.
///
/// Useful for prioritizing simpler patterns in the discrimination tree.
#[allow(dead_code)]
pub fn expr_depth(expr: &Expr) -> u32 {
    match expr {
        Expr::Sort(_) | Expr::Lit(_) | Expr::BVar(_) | Expr::FVar(_) => 1,
        Expr::Const(_, _) => 1,
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            1 + expr_depth(ty).max(expr_depth(body))
        }
        Expr::App(f, a) => 1 + expr_depth(f).max(expr_depth(a)),
        Expr::Let(_, ty, val, body) => {
            1 + expr_depth(ty).max(expr_depth(val)).max(expr_depth(body))
        }
        Expr::Proj(_, _, inner) => 1 + expr_depth(inner),
    }
}
/// Compute the number of nodes in an expression tree.
#[allow(dead_code)]
pub fn expr_size(expr: &Expr) -> u32 {
    match expr {
        Expr::Sort(_) | Expr::Lit(_) | Expr::BVar(_) | Expr::FVar(_) | Expr::Const(_, _) => 1,
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => 1 + expr_size(ty) + expr_size(body),
        Expr::App(f, a) => 1 + expr_size(f) + expr_size(a),
        Expr::Let(_, ty, val, body) => 1 + expr_size(ty) + expr_size(val) + expr_size(body),
        Expr::Proj(_, _, inner) => 1 + expr_size(inner),
    }
}
/// Check if an expression is "simple" (depth <= 2, no lambdas or lets).
#[allow(dead_code)]
pub fn is_simple_expr(expr: &Expr) -> bool {
    expr_depth(expr) <= 2 && !has_lam_or_let(expr)
}
/// Check if an expression contains any lambda or let binding.
#[allow(dead_code)]
pub(super) fn has_lam_or_let(expr: &Expr) -> bool {
    match expr {
        Expr::Lam(..) | Expr::Let(..) => true,
        Expr::App(f, a) => has_lam_or_let(f) || has_lam_or_let(a),
        Expr::Pi(_, _, ty, body) => has_lam_or_let(ty) || has_lam_or_let(body),
        Expr::Proj(_, _, inner) => has_lam_or_let(inner),
        _ => false,
    }
}
/// Merge two discrimination trees into one.
///
/// All entries from both trees are preserved.
#[allow(dead_code)]
pub fn merge_trees<T: Clone>(mut a: DiscrTree<T>, b: DiscrTree<T>) -> DiscrTree<T> {
    for value in b.all_values() {
        let star_expr = Expr::BVar(0);
        a.insert(&star_expr, value.clone());
    }
    a
}
/// Compute statistics for a discrimination tree.
#[allow(dead_code)]
pub fn compute_stats<T: Clone>(tree: &DiscrTree<T>) -> DiscrTreeStats {
    DiscrTreeStats {
        num_entries: tree.num_entries(),
        max_depth: 0,
        num_leaves: 0,
        num_star_paths: 0,
    }
}
/// Check if an expression is a "head-applied" form: `f a₁ ... aₙ`
/// where `f` is a constant or free variable.
#[allow(dead_code)]
pub fn is_head_applied(expr: &Expr) -> bool {
    let mut e = expr;
    while let Expr::App(f, _) = e {
        e = f;
    }
    matches!(e, Expr::Const(_, _) | Expr::FVar(_))
}
/// Get the head constant name from an application, if any.
#[allow(dead_code)]
pub fn head_const_name(expr: &Expr) -> Option<&oxilean_kernel::Name> {
    let mut e = expr;
    while let Expr::App(f, _) = e {
        e = f;
    }
    match e {
        Expr::Const(name, _) => Some(name),
        _ => None,
    }
}
/// Get the number of arguments in a head-applied expression.
#[allow(dead_code)]
pub fn head_applied_arity(expr: &Expr) -> u32 {
    let mut count = 0u32;
    let mut e = expr;
    while let Expr::App(f, _) = e {
        count += 1;
        e = f;
    }
    count
}
/// Check if an expression is a proposition (Sort 0 application form).
#[allow(dead_code)]
pub fn is_prop_form(expr: &Expr) -> bool {
    matches!(expr, Expr::Sort(l) if * l == oxilean_kernel::Level::zero())
}
/// Check if an expression is a type (Sort 1 application form).
#[allow(dead_code)]
pub fn is_type_form(expr: &Expr) -> bool {
    matches!(
        expr, Expr::Sort(l) if * l ==
        oxilean_kernel::Level::succ(oxilean_kernel::Level::zero())
    )
}
#[cfg(test)]
mod section3_tests {
    use super::*;
    use crate::discr_tree::*;
    use oxilean_kernel::{Level, Name};
    #[test]
    fn test_scored_result_ordering() {
        let a: ScoredResult<u32> = ScoredResult::new(1, 10, 1);
        let b: ScoredResult<u32> = ScoredResult::new(2, 20, 1);
        assert!(a > b);
    }
    #[test]
    fn test_priority_discr_tree_find_sorted() {
        let mut tree: PriorityDiscrTree<u32> = PriorityDiscrTree::new();
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        tree.insert_with_score(&nat, 100, 50);
        tree.insert_with_score(&nat, 200, 100);
        let results = tree.find_sorted(&nat);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0], 200);
    }
    #[test]
    fn test_priority_discr_tree_find_best() {
        let mut tree: PriorityDiscrTree<&str> = PriorityDiscrTree::new();
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        tree.insert_with_score(&nat, "low", 1);
        tree.insert_with_score(&nat, "high", 100);
        let best = tree.find_best(&nat);
        assert_eq!(best, Some("high"));
    }
    #[test]
    fn test_key_to_string_const() {
        let key = DiscrTreeKey::Const(Name::str("f"), 2);
        let s = key_to_string(&key);
        assert!(s.contains("C("));
        assert!(s.contains(",2)"));
    }
    #[test]
    fn test_key_to_string_star() {
        assert_eq!(key_to_string(&DiscrTreeKey::Star), "*");
    }
    #[test]
    fn test_keys_to_path_string() {
        let keys = vec![DiscrTreeKey::Const(Name::str("f"), 1), DiscrTreeKey::Star];
        let s = keys_to_path_string(&keys);
        assert!(s.contains(" -> "));
    }
    #[test]
    fn test_has_wildcards() {
        let keys = vec![DiscrTreeKey::Star, DiscrTreeKey::Sort];
        assert!(has_wildcards(&keys));
        let keys2 = vec![DiscrTreeKey::Sort];
        assert!(!has_wildcards(&keys2));
    }
    #[test]
    fn test_count_wildcards() {
        let keys = vec![DiscrTreeKey::Star, DiscrTreeKey::Sort, DiscrTreeKey::Star];
        assert_eq!(count_wildcards(&keys), 2);
    }
    #[test]
    fn test_specificity_score_no_wildcards() {
        let keys = vec![DiscrTreeKey::Const(Name::str("f"), 2)];
        let score = specificity_score(&keys);
        assert!(score > 0);
    }
    #[test]
    fn test_specificity_score_wildcards_penalized() {
        let exact = vec![DiscrTreeKey::Const(Name::str("f"), 0)];
        let wild = vec![DiscrTreeKey::Star];
        assert!(specificity_score(&exact) > specificity_score(&wild));
    }
    #[test]
    fn test_match_info_exact() {
        let keys = vec![DiscrTreeKey::Sort];
        let mi = MatchInfo::compute(&keys, &keys);
        assert!(mi.is_exact());
        assert!(!mi.is_approximate());
    }
    #[test]
    fn test_match_info_approximate() {
        let idx = vec![DiscrTreeKey::Star];
        let qry = vec![DiscrTreeKey::Sort];
        let mi = MatchInfo::compute(&idx, &qry);
        assert!(mi.is_approximate());
    }
    #[test]
    fn test_expr_depth_leaf() {
        let e = Expr::Lit(oxilean_kernel::Literal::Nat(0));
        assert_eq!(expr_depth(&e), 1);
    }
    #[test]
    fn test_expr_depth_app() {
        let e = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(Expr::Lit(oxilean_kernel::Literal::Nat(0))),
        );
        assert_eq!(expr_depth(&e), 2);
    }
    #[test]
    fn test_expr_size_leaf() {
        let e = Expr::Sort(Level::zero());
        assert_eq!(expr_size(&e), 1);
    }
    #[test]
    fn test_expr_size_app() {
        let e = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(Expr::Const(Name::str("a"), vec![])),
        );
        assert_eq!(expr_size(&e), 3);
    }
    #[test]
    fn test_is_simple_expr() {
        let e = Expr::Const(Name::str("Nat"), vec![]);
        assert!(is_simple_expr(&e));
    }
    #[test]
    fn test_is_head_applied_const() {
        let e = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(Expr::Lit(oxilean_kernel::Literal::Nat(0))),
        );
        assert!(is_head_applied(&e));
    }
    #[test]
    fn test_is_head_applied_lam_false() {
        let lam = Expr::Lam(
            oxilean_kernel::BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::BVar(0)),
        );
        assert!(!is_head_applied(&lam));
    }
    #[test]
    fn test_head_const_name() {
        let e = Expr::App(
            Box::new(Expr::Const(Name::str("foo"), vec![])),
            Box::new(Expr::BVar(0)),
        );
        assert_eq!(head_const_name(&e), Some(&Name::str("foo")));
    }
    #[test]
    fn test_head_applied_arity() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::BVar(0);
        let b = Expr::BVar(1);
        let e = Expr::App(Box::new(Expr::App(Box::new(f), Box::new(a))), Box::new(b));
        assert_eq!(head_applied_arity(&e), 2);
    }
    #[test]
    fn test_is_prop_form() {
        assert!(is_prop_form(&Expr::Sort(Level::zero())));
        assert!(!is_prop_form(&Expr::Sort(Level::succ(Level::zero()))));
    }
    #[test]
    fn test_is_type_form() {
        assert!(is_type_form(&Expr::Sort(Level::succ(Level::zero()))));
        assert!(!is_type_form(&Expr::Sort(Level::zero())));
    }
    #[test]
    fn test_key_pattern_any() {
        let p = KeyPattern::Any;
        assert!(p.matches(&DiscrTreeKey::Sort));
        assert!(p.matches(&DiscrTreeKey::Star));
    }
    #[test]
    fn test_key_pattern_exact() {
        let p = KeyPattern::Exact(DiscrTreeKey::Sort);
        assert!(p.matches(&DiscrTreeKey::Sort));
        assert!(!p.matches(&DiscrTreeKey::Star));
    }
    #[test]
    fn test_key_pattern_const_arity() {
        let p = KeyPattern::ConstArity(2);
        assert!(p.matches(&DiscrTreeKey::Const(Name::str("f"), 2)));
        assert!(!p.matches(&DiscrTreeKey::Const(Name::str("f"), 1)));
    }
    #[test]
    fn test_key_pattern_not() {
        let p = KeyPattern::Not(Box::new(KeyPattern::AnySort));
        assert!(!p.matches(&DiscrTreeKey::Sort));
        assert!(p.matches(&DiscrTreeKey::Star));
    }
    #[test]
    fn test_key_pattern_one_of() {
        let p = KeyPattern::OneOf(vec![DiscrTreeKey::Sort, DiscrTreeKey::Lambda]);
        assert!(p.matches(&DiscrTreeKey::Sort));
        assert!(p.matches(&DiscrTreeKey::Lambda));
        assert!(!p.matches(&DiscrTreeKey::Pi));
    }
    #[test]
    fn test_discr_tree_view() {
        let mut tree: DiscrTree<u32> = DiscrTree::new();
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        tree.insert(&nat, 42);
        let view = DiscrTreeView::new(&tree);
        assert_eq!(view.num_entries(), 1);
        let found = view.find(&nat);
        assert_eq!(found.len(), 1);
        assert_eq!(*found[0], 42);
    }
    #[test]
    fn test_discr_tree_builder() {
        let mut builder: DiscrTreeBuilder<u32> = DiscrTreeBuilder::new(2);
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let bool_e = Expr::Const(Name::str("Bool"), vec![]);
        builder.queue(nat.clone(), 1);
        builder.queue(bool_e.clone(), 2);
        assert_eq!(builder.num_pending(), 0);
        let tree = builder.finish();
        assert_eq!(tree.num_entries(), 2);
    }
    #[test]
    fn test_layered_discr_tree_insert_default() {
        let mut lt: LayeredDiscrTree<u32> = LayeredDiscrTree::new();
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        lt.insert_default(&nat, 99);
        let found = lt.find_default(&nat);
        assert_eq!(found.len(), 1);
        assert_eq!(*found[0], 99);
        let found_all = lt.find_all(&nat);
        assert!(found_all.is_empty());
    }
    #[test]
    fn test_layered_discr_tree_insert_all_layers() {
        let mut lt: LayeredDiscrTree<u32> = LayeredDiscrTree::new();
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        lt.insert_all_layers(&nat, 7);
        assert!(!lt.find_default(&nat).is_empty());
        assert!(!lt.find_all(&nat).is_empty());
        assert!(!lt.find_reducible(&nat).is_empty());
        assert!(lt.total_entries() >= 3);
    }
    #[test]
    fn test_expr_fingerprint_of() {
        let e = Expr::Const(Name::str("Nat"), vec![]);
        let fp = ExprFingerprint::of(&e);
        assert_eq!(fp.depth, 1);
        assert_eq!(fp.size, 1);
        assert!(!fp.has_wildcard);
    }
    #[test]
    fn test_expr_fingerprint_could_match() {
        let e1 = Expr::Const(Name::str("f"), vec![]);
        let e2 = Expr::BVar(0);
        let fp1 = ExprFingerprint::of(&e1);
        let fp2 = ExprFingerprint::of(&e2);
        assert!(fp1.could_match(&fp2));
    }
    #[test]
    fn test_discr_tree_stats_density() {
        let s = DiscrTreeStats {
            num_entries: 10,
            max_depth: 3,
            num_leaves: 4,
            num_star_paths: 1,
        };
        assert!((s.density() - 2.5).abs() < 1e-9);
    }
    #[test]
    fn test_top_k_config_default() {
        let cfg = TopKConfig::default();
        assert_eq!(cfg.k, 10);
        assert!(cfg.include_approximate);
    }
    #[test]
    fn test_top_k_config_exact_only() {
        let cfg = TopKConfig::exact_only(5);
        assert_eq!(cfg.k, 5);
        assert!(!cfg.include_approximate);
    }
    #[test]
    fn test_match_info_num_exact_positions() {
        let idx = vec![DiscrTreeKey::Sort, DiscrTreeKey::Star];
        let qry = vec![DiscrTreeKey::Sort, DiscrTreeKey::Lambda];
        let mi = MatchInfo::compute(&idx, &qry);
        assert_eq!(mi.num_exact_positions(), 1);
    }
}
/// Query multiple expressions against a discrimination tree in a single call.
///
/// Returns a vector of result sets, one per query expression.
#[allow(dead_code)]
pub fn batch_lookup<'a, T: Clone>(tree: &'a DiscrTree<T>, queries: &[Expr]) -> Vec<Vec<&'a T>> {
    queries.iter().map(|q| tree.find(q)).collect()
}
/// Find the union of all matches for a list of query expressions.
#[allow(dead_code)]
pub fn union_lookup<'a, T: Clone + PartialEq>(
    tree: &'a DiscrTree<T>,
    queries: &[Expr],
) -> Vec<&'a T> {
    let mut seen = Vec::new();
    for query in queries {
        for val in tree.find(query) {
            if !seen.contains(&val) {
                seen.push(val);
            }
        }
    }
    seen
}
/// Find the intersection of all matches for a list of query expressions.
///
/// Returns only values that appear in ALL query result sets.
#[allow(dead_code)]
pub fn intersection_lookup<'a, T: Clone + PartialEq>(
    tree: &'a DiscrTree<T>,
    queries: &[Expr],
) -> Vec<&'a T> {
    if queries.is_empty() {
        return Vec::new();
    }
    let first_results = tree.find(&queries[0]);
    let mut common: Vec<&'a T> = first_results;
    for query in &queries[1..] {
        let results = tree.find(query);
        common.retain(|v| results.contains(v));
    }
    common
}
#[cfg(test)]
mod section17_20_tests {
    use super::*;
    use crate::discr_tree::*;
    use oxilean_kernel::{Level, Literal, Name};
    #[test]
    fn test_lemma_autocomplete_basic() {
        let mut ac = LemmaAutocomplete::new();
        ac.add_many(&["Nat.add_comm", "Nat.add_assoc", "List.append_nil"]);
        assert!(ac.contains("Nat.add_comm"));
        assert!(!ac.contains("Nat.mul_comm"));
        let comps = ac.complete("Nat.add");
        assert_eq!(comps.len(), 2);
    }
    #[test]
    fn test_lemma_autocomplete_empty() {
        let ac = LemmaAutocomplete::new();
        assert!(ac.is_empty());
        assert_eq!(ac.complete("Nat"), vec![] as Vec<String>);
    }
    #[test]
    fn test_lemma_autocomplete_len() {
        let mut ac = LemmaAutocomplete::new();
        ac.add("a");
        ac.add("b");
        ac.add("c");
        assert_eq!(ac.len(), 3);
    }
    #[test]
    fn test_simp_lemma_index_basic() {
        let mut idx = SimpLemmaIndex::new();
        let lhs = Expr::App(
            Box::new(Expr::Const(Name::str("Nat.add"), vec![])),
            Box::new(Expr::Lit(Literal::Nat(0))),
        );
        let rhs = Expr::Lit(Literal::Nat(0));
        let entry = SimpLemmaEntry::unconditional(Name::str("Nat.add_zero"), lhs.clone(), rhs);
        idx.add(entry);
        assert_eq!(idx.num_lemmas(), 1);
        assert_eq!(idx.num_unconditional(), 1);
        assert_eq!(idx.num_conditional(), 0);
        let results = idx.find_applicable(&lhs);
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_simp_lemma_entry_new() {
        let lhs = Expr::Sort(Level::zero());
        let rhs = Expr::Sort(Level::zero());
        let e = SimpLemmaEntry::new(Name::str("test"), 42, true, lhs, rhs);
        assert!(e.is_conditional);
        assert_eq!(e.priority, 42);
    }
    #[test]
    fn test_instance_index_basic() {
        let mut idx = InstanceIndex::new();
        let ty = Expr::App(
            Box::new(Expr::Const(Name::str("Add"), vec![])),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
        );
        let entry = InstanceEntry::new(Name::str("Nat.instAdd"), Name::str("Add"), 100, ty.clone());
        idx.register(entry, &ty);
        assert_eq!(idx.num_instances(), 1);
        let found = idx.find(&ty);
        assert!(!found.is_empty());
    }
    #[test]
    fn test_batch_lookup() {
        let mut tree: DiscrTree<u32> = DiscrTree::new();
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let bool_e = Expr::Const(Name::str("Bool"), vec![]);
        tree.insert(&nat, 1);
        tree.insert(&bool_e, 2);
        let results = batch_lookup(&tree, &[nat, bool_e]);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].len(), 1);
        assert_eq!(results[1].len(), 1);
    }
    #[test]
    fn test_union_lookup() {
        let mut tree: DiscrTree<u32> = DiscrTree::new();
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let bool_e = Expr::Const(Name::str("Bool"), vec![]);
        tree.insert(&nat, 1);
        tree.insert(&nat, 2);
        tree.insert(&bool_e, 3);
        let union = union_lookup(&tree, &[nat, bool_e]);
        assert_eq!(union.len(), 3);
    }
    #[test]
    fn test_intersection_lookup_empty_queries() {
        let tree: DiscrTree<u32> = DiscrTree::new();
        let result = intersection_lookup(&tree, &[]);
        assert!(result.is_empty());
    }
    #[test]
    fn test_intersection_lookup_single_query() {
        let mut tree: DiscrTree<u32> = DiscrTree::new();
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        tree.insert(&nat, 42);
        let result = intersection_lookup(&tree, &[nat]);
        assert_eq!(result.len(), 1);
    }
    #[test]
    fn test_intersection_lookup_two_queries_common() {
        let mut tree: DiscrTree<u32> = DiscrTree::new();
        let star = Expr::BVar(0);
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        tree.insert(&star, 99);
        let r1 = tree.find(&nat);
        assert!(!r1.is_empty());
    }
}
