//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{Expr, Literal, Name};
use std::collections::HashMap;

/// An instance entry for typeclass synthesis.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct InstanceEntry {
    /// Instance name.
    pub name: oxilean_kernel::Name,
    /// The typeclass being instantiated.
    pub class: oxilean_kernel::Name,
    /// Priority for instance selection.
    pub priority: u32,
    /// Instance type (the full Pi-type of the instance).
    pub ty: Expr,
}
impl InstanceEntry {
    /// Create a new instance entry.
    #[allow(dead_code)]
    pub fn new(
        name: oxilean_kernel::Name,
        class: oxilean_kernel::Name,
        priority: u32,
        ty: Expr,
    ) -> Self {
        Self {
            name,
            class,
            priority,
            ty,
        }
    }
}
/// Simple trie for efficient string prefix lookup.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct StringTrie {
    pub(super) children: std::collections::HashMap<char, StringTrie>,
    pub(super) is_end: bool,
    pub(super) value: Option<String>,
}
impl StringTrie {
    /// Create a new empty trie.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Insert a string into the trie.
    #[allow(dead_code)]
    pub fn insert(&mut self, s: &str) {
        let mut node = self;
        for c in s.chars() {
            node = node.children.entry(c).or_default();
        }
        node.is_end = true;
        node.value = Some(s.to_string());
    }
    /// Check if a string is in the trie.
    #[allow(dead_code)]
    pub fn contains(&self, s: &str) -> bool {
        let mut node = self;
        for c in s.chars() {
            match node.children.get(&c) {
                Some(next) => node = next,
                None => return false,
            }
        }
        node.is_end
    }
    /// Find all strings with a given prefix.
    #[allow(dead_code)]
    pub fn starts_with(&self, prefix: &str) -> Vec<String> {
        let mut node = self;
        for c in prefix.chars() {
            match node.children.get(&c) {
                Some(next) => node = next,
                None => return vec![],
            }
        }
        let mut results = Vec::new();
        collect_strings(node, &mut results);
        results
    }
    /// Get the number of strings in the trie.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        let mut count = if self.is_end { 1 } else { 0 };
        for child in self.children.values() {
            count += child.len();
        }
        count
    }
    /// Check if the trie is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
/// A read-only view into a discrimination tree.
///
/// Allows sharing the tree data without cloning.
#[allow(dead_code)]
pub struct DiscrTreeView<'a, T: Clone> {
    pub(super) tree: &'a DiscrTree<T>,
}
impl<'a, T: Clone> DiscrTreeView<'a, T> {
    /// Create a view from a reference.
    #[allow(dead_code)]
    pub fn new(tree: &'a DiscrTree<T>) -> Self {
        Self { tree }
    }
    /// Find matches.
    #[allow(dead_code)]
    pub fn find(&self, expr: &Expr) -> Vec<&'a T> {
        self.tree.find(expr)
    }
    /// Number of entries.
    #[allow(dead_code)]
    pub fn num_entries(&self) -> usize {
        self.tree.num_entries()
    }
    /// All values.
    #[allow(dead_code)]
    pub fn all_values(&self) -> Vec<&'a T> {
        self.tree.all_values()
    }
}
/// A compact fingerprint of an expression for quick compatibility checks.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExprFingerprint {
    /// Hash of the top-level key.
    pub top_key_hash: u64,
    /// Depth estimate.
    pub depth: u8,
    /// Size estimate (capped at 255).
    pub size: u8,
    /// Whether the expression contains a wildcard.
    pub has_wildcard: bool,
}
impl ExprFingerprint {
    /// Compute a fingerprint from an expression.
    #[allow(dead_code)]
    pub fn of(expr: &Expr) -> Self {
        let keys = encode_expr(expr);
        let top_key_hash = if keys.is_empty() {
            0
        } else {
            use std::hash::{Hash, Hasher};
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            keys[0].hash(&mut hasher);
            hasher.finish()
        };
        let depth = expr_depth(expr).min(255) as u8;
        let size = expr_size(expr).min(255) as u8;
        let has_wildcard = has_wildcards(&keys);
        ExprFingerprint {
            top_key_hash,
            depth,
            size,
            has_wildcard,
        }
    }
    /// Check if two fingerprints could possibly match.
    #[allow(dead_code)]
    pub fn could_match(&self, other: &ExprFingerprint) -> bool {
        self.has_wildcard || other.has_wildcard || self.top_key_hash == other.top_key_hash
    }
}
/// Information about how a query matched an indexed pattern.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct MatchInfo {
    /// The keys from the index that matched.
    pub index_keys: Vec<DiscrTreeKey>,
    /// The keys from the query.
    pub query_keys: Vec<DiscrTreeKey>,
    /// Positions where wildcards were used.
    pub wildcard_positions: Vec<usize>,
    /// Overall match quality score.
    pub quality: i64,
}
impl MatchInfo {
    /// Compute a match info between index and query keys.
    #[allow(dead_code)]
    pub fn compute(index_keys: &[DiscrTreeKey], query_keys: &[DiscrTreeKey]) -> Self {
        let mut wildcard_positions = Vec::new();
        let min_len = index_keys.len().min(query_keys.len());
        for i in 0..min_len {
            if matches!(index_keys[i], DiscrTreeKey::Star)
                || matches!(query_keys[i], DiscrTreeKey::Star)
            {
                wildcard_positions.push(i);
            }
        }
        let quality = specificity_score(index_keys).min(specificity_score(query_keys));
        MatchInfo {
            index_keys: index_keys.to_vec(),
            query_keys: query_keys.to_vec(),
            wildcard_positions,
            quality,
        }
    }
    /// Whether the match is exact (no wildcards used).
    #[allow(dead_code)]
    pub fn is_exact(&self) -> bool {
        self.wildcard_positions.is_empty()
    }
    /// Whether any wildcards were used in the match.
    #[allow(dead_code)]
    pub fn is_approximate(&self) -> bool {
        !self.wildcard_positions.is_empty()
    }
    /// Number of positions matched exactly.
    #[allow(dead_code)]
    pub fn num_exact_positions(&self) -> usize {
        let total = self.index_keys.len().min(self.query_keys.len());
        total - self.wildcard_positions.len()
    }
}
/// A priority-scored result from a discrimination tree query.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ScoredResult<T: Clone> {
    /// The retrieved value.
    pub value: T,
    /// Score (higher = better match).
    pub score: i64,
    /// Depth at which the value was found.
    pub depth: usize,
}
impl<T: Clone> ScoredResult<T> {
    /// Create a new scored result.
    #[allow(dead_code)]
    pub fn new(value: T, score: i64, depth: usize) -> Self {
        Self {
            value,
            score,
            depth,
        }
    }
}
/// A multi-tree that stores values under multiple expression indices.
#[allow(dead_code)]
pub struct MultiDiscrTree<T: Clone + PartialEq> {
    pub(super) inner: DiscrTree<T>,
}
impl<T: Clone + PartialEq> MultiDiscrTree<T> {
    /// Create a new empty multi-tree.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            inner: DiscrTree::new(),
        }
    }
    /// Insert a value under multiple expression keys.
    #[allow(dead_code)]
    pub fn insert_multi(&mut self, exprs: &[Expr], value: T) {
        for expr in exprs {
            self.inner.insert(expr, value.clone());
        }
    }
    /// Find values matching any of the given expressions.
    #[allow(dead_code)]
    pub fn find_any(&self, exprs: &[Expr]) -> Vec<&T> {
        let mut results = Vec::new();
        for expr in exprs {
            for val in self.inner.find(expr) {
                if !results.contains(&val) {
                    results.push(val);
                }
            }
        }
        results
    }
    /// Get number of entries.
    #[allow(dead_code)]
    pub fn num_entries(&self) -> usize {
        self.inner.num_entries()
    }
}
/// A discrimination tree layer corresponding to a transparency level.
#[allow(dead_code)]
pub struct LayeredDiscrTree<T: Clone> {
    /// Full transparency (all definitions unfolded).
    pub(super) all: DiscrTree<T>,
    /// Default transparency (standard definitions).
    pub(super) default: DiscrTree<T>,
    /// Reducible only.
    pub(super) reducible: DiscrTree<T>,
}
impl<T: Clone> LayeredDiscrTree<T> {
    /// Create a new layered tree.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            all: DiscrTree::new(),
            default: DiscrTree::new(),
            reducible: DiscrTree::new(),
        }
    }
    /// Insert into all layers.
    #[allow(dead_code)]
    pub fn insert_all_layers(&mut self, expr: &Expr, value: T) {
        self.all.insert(expr, value.clone());
        self.default.insert(expr, value.clone());
        self.reducible.insert(expr, value);
    }
    /// Insert into only the default layer.
    #[allow(dead_code)]
    pub fn insert_default(&mut self, expr: &Expr, value: T) {
        self.default.insert(expr, value);
    }
    /// Find in the default layer.
    #[allow(dead_code)]
    pub fn find_default(&self, expr: &Expr) -> Vec<&T> {
        self.default.find(expr)
    }
    /// Find in the all-transparency layer.
    #[allow(dead_code)]
    pub fn find_all(&self, expr: &Expr) -> Vec<&T> {
        self.all.find(expr)
    }
    /// Find in the reducible layer.
    #[allow(dead_code)]
    pub fn find_reducible(&self, expr: &Expr) -> Vec<&T> {
        self.reducible.find(expr)
    }
    /// Total number of entries across all layers.
    #[allow(dead_code)]
    pub fn total_entries(&self) -> usize {
        self.all.num_entries() + self.default.num_entries() + self.reducible.num_entries()
    }
}
/// Builder for incrementally constructing a discrimination tree.
#[allow(dead_code)]
pub struct DiscrTreeBuilder<T: Clone> {
    pub(super) pending: Vec<(Expr, T)>,
    pub(super) tree: DiscrTree<T>,
    pub(super) batch_size: usize,
}
impl<T: Clone> DiscrTreeBuilder<T> {
    /// Create a new builder with a given batch size.
    #[allow(dead_code)]
    pub fn new(batch_size: usize) -> Self {
        Self {
            pending: Vec::new(),
            tree: DiscrTree::new(),
            batch_size,
        }
    }
    /// Queue an entry for insertion.
    #[allow(dead_code)]
    pub fn queue(&mut self, expr: Expr, value: T) {
        self.pending.push((expr, value));
        if self.pending.len() >= self.batch_size {
            self.flush();
        }
    }
    /// Flush all pending entries into the tree.
    #[allow(dead_code)]
    pub fn flush(&mut self) {
        for (expr, value) in self.pending.drain(..) {
            self.tree.insert(&expr, value);
        }
    }
    /// Finish building and return the tree.
    #[allow(dead_code)]
    pub fn finish(mut self) -> DiscrTree<T> {
        self.flush();
        self.tree
    }
    /// Number of pending entries.
    #[allow(dead_code)]
    pub fn num_pending(&self) -> usize {
        self.pending.len()
    }
}
/// Key used for indexing expressions in the discrimination tree.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DiscrTreeKey {
    /// Constant name with arity.
    Const(Name, u32),
    /// Free variable (opaque, matches only itself).
    FVar(u64),
    /// Literal value.
    Lit(Literal),
    /// Sort (matches any universe).
    Sort,
    /// Lambda (matches any lambda).
    Lambda,
    /// Pi (matches any Pi type).
    Pi,
    /// Projection with struct name and field index.
    Proj(Name, u32),
    /// Star (wildcard, matches anything; for metavars and bound vars).
    Star,
}
/// A discrimination tree with statistics tracking.
#[allow(dead_code)]
pub struct TrackedDiscrTree<T: Clone> {
    pub(super) inner: DiscrTree<T>,
    pub(super) num_queries: u64,
    pub(super) num_hits: u64,
    pub(super) num_misses: u64,
}
impl<T: Clone> TrackedDiscrTree<T> {
    /// Create a new tracked tree.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            inner: DiscrTree::new(),
            num_queries: 0,
            num_hits: 0,
            num_misses: 0,
        }
    }
    /// Insert a value.
    #[allow(dead_code)]
    pub fn insert(&mut self, expr: &Expr, value: T) {
        self.inner.insert(expr, value);
    }
    /// Find matches and track stats.
    #[allow(dead_code)]
    pub fn find(&mut self, expr: &Expr) -> Vec<&T> {
        self.num_queries += 1;
        let results = self.inner.find(expr);
        if results.is_empty() {
            self.num_misses += 1;
        } else {
            self.num_hits += 1;
        }
        results
    }
    /// Get the number of entries.
    #[allow(dead_code)]
    pub fn num_entries(&self) -> usize {
        self.inner.num_entries()
    }
    /// Get hit rate as fraction in [0, 1].
    #[allow(dead_code)]
    pub fn hit_rate(&self) -> f64 {
        if self.num_queries == 0 {
            0.0
        } else {
            self.num_hits as f64 / self.num_queries as f64
        }
    }
    /// Get total queries.
    #[allow(dead_code)]
    pub fn num_queries(&self) -> u64 {
        self.num_queries
    }
    /// Get total hits.
    #[allow(dead_code)]
    pub fn num_hits(&self) -> u64 {
        self.num_hits
    }
    /// Get total misses.
    #[allow(dead_code)]
    pub fn num_misses(&self) -> u64 {
        self.num_misses
    }
    /// Reset statistics.
    #[allow(dead_code)]
    pub fn reset_stats(&mut self) {
        self.num_queries = 0;
        self.num_hits = 0;
        self.num_misses = 0;
    }
    /// Clear the tree and stats.
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.inner.clear();
        self.reset_stats();
    }
}
/// A simple autocomplete index mapping name prefixes to matching names.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct LemmaAutocomplete {
    pub(super) trie: StringTrie,
}
impl LemmaAutocomplete {
    /// Create a new empty autocomplete index.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a lemma name to the index.
    #[allow(dead_code)]
    pub fn add(&mut self, name: &str) {
        self.trie.insert(name);
    }
    /// Add multiple names at once.
    #[allow(dead_code)]
    pub fn add_many(&mut self, names: &[&str]) {
        for name in names {
            self.trie.insert(name);
        }
    }
    /// Get completions for a prefix.
    #[allow(dead_code)]
    pub fn complete(&self, prefix: &str) -> Vec<String> {
        self.trie.starts_with(prefix)
    }
    /// Check if an exact name exists.
    #[allow(dead_code)]
    pub fn contains(&self, name: &str) -> bool {
        self.trie.contains(name)
    }
    /// Total number of names.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.trie.len()
    }
    /// Whether the index is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.trie.is_empty()
    }
}
/// Index of simp lemmas for fast lookup.
#[allow(dead_code)]
pub struct SimpLemmaIndex {
    pub(super) tree: DiscrTree<SimpLemmaEntry>,
    pub(super) num_conditional: usize,
    pub(super) num_unconditional: usize,
}
impl SimpLemmaIndex {
    /// Create a new empty index.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            tree: DiscrTree::new(),
            num_conditional: 0,
            num_unconditional: 0,
        }
    }
    /// Add a simp lemma to the index.
    #[allow(dead_code)]
    pub fn add(&mut self, entry: SimpLemmaEntry) {
        let lhs = entry.lhs.clone();
        if entry.is_conditional {
            self.num_conditional += 1;
        } else {
            self.num_unconditional += 1;
        }
        self.tree.insert(&lhs, entry);
    }
    /// Find applicable simp lemmas for an expression.
    #[allow(dead_code)]
    pub fn find_applicable(&self, expr: &Expr) -> Vec<&SimpLemmaEntry> {
        let mut results = self.tree.find(expr);
        results.sort_by(|a, b| b.priority.cmp(&a.priority));
        results
    }
    /// Number of total lemmas.
    #[allow(dead_code)]
    pub fn num_lemmas(&self) -> usize {
        self.tree.num_entries()
    }
    /// Number of conditional lemmas.
    #[allow(dead_code)]
    pub fn num_conditional(&self) -> usize {
        self.num_conditional
    }
    /// Number of unconditional lemmas.
    #[allow(dead_code)]
    pub fn num_unconditional(&self) -> usize {
        self.num_unconditional
    }
}
/// Index of typeclass instances.
#[allow(dead_code)]
pub struct InstanceIndex {
    /// Indexed by the "key type" (the conclusion of the instance).
    pub(super) tree: DiscrTree<InstanceEntry>,
    /// Total number of instances.
    pub(super) num_instances: usize,
}
impl InstanceIndex {
    /// Create a new empty index.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            tree: DiscrTree::new(),
            num_instances: 0,
        }
    }
    /// Register an instance.
    #[allow(dead_code)]
    pub fn register(&mut self, instance: InstanceEntry, key_expr: &Expr) {
        self.tree.insert(key_expr, instance);
        self.num_instances += 1;
    }
    /// Find instances applicable to a type expression.
    #[allow(dead_code)]
    pub fn find(&self, ty: &Expr) -> Vec<&InstanceEntry> {
        let mut results = self.tree.find(ty);
        results.sort_by(|a, b| b.priority.cmp(&a.priority));
        results
    }
    /// Total instances registered.
    #[allow(dead_code)]
    pub fn num_instances(&self) -> usize {
        self.num_instances
    }
}
/// A discrimination tree that returns results in priority order.
#[allow(dead_code)]
pub struct PriorityDiscrTree<T: Clone + PartialEq> {
    pub(super) inner: DiscrTree<(T, i64)>,
}
impl<T: Clone + PartialEq> PriorityDiscrTree<T> {
    /// Create a new priority discrimination tree.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            inner: DiscrTree::new(),
        }
    }
    /// Insert a value with a priority score.
    #[allow(dead_code)]
    pub fn insert_with_score(&mut self, expr: &Expr, value: T, score: i64) {
        self.inner.insert(expr, (value, score));
    }
    /// Find matching values, sorted by descending score.
    #[allow(dead_code)]
    pub fn find_sorted(&self, expr: &Expr) -> Vec<T> {
        let mut results: Vec<(T, i64)> = self
            .inner
            .find(expr)
            .into_iter()
            .map(|(v, s)| (v.clone(), *s))
            .collect();
        results.sort_by(|a, b| b.1.cmp(&a.1));
        results.into_iter().map(|(v, _)| v).collect()
    }
    /// Get the best (highest-score) match.
    #[allow(dead_code)]
    pub fn find_best(&self, expr: &Expr) -> Option<T> {
        self.find_sorted(expr).into_iter().next()
    }
    /// Number of entries.
    #[allow(dead_code)]
    pub fn num_entries(&self) -> usize {
        self.inner.num_entries()
    }
}
/// Discrimination tree for fast expression lookup.
///
/// Stores values of type `T` indexed by expression patterns.
/// Supports unification-aware retrieval where `Star` keys match
/// any expression.
pub struct DiscrTree<T: Clone> {
    /// Root node of the tree.
    pub(super) root: DiscrTreeNode<T>,
    /// Number of entries in the tree.
    pub(super) num_entries: usize,
}
impl<T: Clone> DiscrTree<T> {
    /// Create a new empty discrimination tree.
    pub fn new() -> Self {
        Self {
            root: DiscrTreeNode::new(),
            num_entries: 0,
        }
    }
    /// Insert a value indexed by an expression.
    pub fn insert(&mut self, expr: &Expr, value: T) {
        let keys = encode_expr(expr);
        let mut node = &mut self.root;
        for key in &keys {
            node = node.children.entry(key.clone()).or_default();
        }
        node.values.push(value);
        self.num_entries += 1;
    }
    /// Find all values whose indexed expression is compatible with the query.
    ///
    /// "Compatible" means the stored keys match the query keys,
    /// with `Star` keys matching any query key.
    pub fn find(&self, expr: &Expr) -> Vec<&T> {
        let keys = encode_expr(expr);
        let mut results = Vec::new();
        self.find_impl(&self.root, &keys, 0, &mut results);
        results
    }
    /// Implementation of find with backtracking.
    pub(super) fn find_impl<'a>(
        &'a self,
        node: &'a DiscrTreeNode<T>,
        keys: &[DiscrTreeKey],
        idx: usize,
        results: &mut Vec<&'a T>,
    ) {
        if idx >= keys.len() {
            results.extend(node.values.iter());
            return;
        }
        let query_key = &keys[idx];
        if let Some(child) = node.children.get(query_key) {
            self.find_impl(child, keys, idx + 1, results);
        }
        if let Some(star_child) = node.children.get(&DiscrTreeKey::Star) {
            let skip = subtree_size(query_key);
            self.find_impl(star_child, keys, idx + skip, results);
        }
        if *query_key == DiscrTreeKey::Star {
            for (stored_key, child) in &node.children {
                if *stored_key != DiscrTreeKey::Star {
                    let skip = subtree_size(stored_key);
                    self.find_impl(child, keys, idx + skip, results);
                }
            }
        }
    }
    /// Get the number of entries in the tree.
    pub fn num_entries(&self) -> usize {
        self.num_entries
    }
    /// Check if the tree is empty.
    pub fn is_empty(&self) -> bool {
        self.num_entries == 0
    }
    /// Get all values stored in the tree.
    pub fn all_values(&self) -> Vec<&T> {
        let mut results = Vec::new();
        self.collect_all_values(&self.root, &mut results);
        results
    }
    /// Recursively collect all values.
    pub(super) fn collect_all_values<'a>(
        &'a self,
        node: &'a DiscrTreeNode<T>,
        results: &mut Vec<&'a T>,
    ) {
        results.extend(node.values.iter());
        for child in node.children.values() {
            self.collect_all_values(child, results);
        }
    }
    /// Clear the tree.
    pub fn clear(&mut self) {
        self.root = DiscrTreeNode::new();
        self.num_entries = 0;
    }
}
/// Configuration for top-k retrieval.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct TopKConfig {
    /// Maximum number of results to return.
    pub k: usize,
    /// Minimum score threshold.
    pub min_score: i64,
    /// Whether to include approximate matches.
    pub include_approximate: bool,
}
impl TopKConfig {
    /// Create a config that only returns exact matches.
    #[allow(dead_code)]
    pub fn exact_only(k: usize) -> Self {
        Self {
            k,
            min_score: 0,
            include_approximate: false,
        }
    }
    /// Create a config with a minimum score.
    #[allow(dead_code)]
    pub fn with_min_score(k: usize, min_score: i64) -> Self {
        Self {
            k,
            min_score,
            include_approximate: true,
        }
    }
}
/// A node in the discrimination tree.
#[derive(Clone, Debug)]
pub(super) struct DiscrTreeNode<T: Clone> {
    /// Children indexed by next key.
    pub(super) children: HashMap<DiscrTreeKey, DiscrTreeNode<T>>,
    /// Values stored at this node (if it's a leaf).
    pub(super) values: Vec<T>,
}
impl<T: Clone> DiscrTreeNode<T> {
    pub(super) fn new() -> Self {
        Self {
            children: HashMap::new(),
            values: Vec::new(),
        }
    }
}
/// Statistics about a discrimination tree.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct DiscrTreeStats {
    /// Total number of entries.
    pub num_entries: usize,
    /// Depth of the tree.
    pub max_depth: usize,
    /// Number of leaf nodes.
    pub num_leaves: usize,
    /// Number of star (wildcard) paths.
    pub num_star_paths: usize,
}
impl DiscrTreeStats {
    /// Create empty stats.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Compute the density (entries per leaf).
    #[allow(dead_code)]
    pub fn density(&self) -> f64 {
        if self.num_leaves == 0 {
            0.0
        } else {
            self.num_entries as f64 / self.num_leaves as f64
        }
    }
}
/// A key path through the discrimination tree.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiscrTreePath(pub(crate) Vec<DiscrTreeKey>);
impl DiscrTreePath {
    /// Create a path from an expression.
    #[allow(dead_code)]
    pub fn from_expr(expr: &Expr) -> Self {
        DiscrTreePath(encode_expr(expr))
    }
    /// Get the length of the path.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.0.len()
    }
    /// Check if the path is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    /// Check if this path is a prefix of another.
    #[allow(dead_code)]
    pub fn is_prefix_of(&self, other: &DiscrTreePath) -> bool {
        other.0.starts_with(&self.0)
    }
    /// Get the keys in this path.
    #[allow(dead_code)]
    pub fn keys(&self) -> &[DiscrTreeKey] {
        &self.0
    }
}
/// A simple ordered index mapping names to integer IDs.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct NameIndex {
    pub(super) names: Vec<String>,
    pub(super) index: std::collections::HashMap<String, usize>,
}
impl NameIndex {
    /// Create a new empty name index.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Insert a name, returning its ID.
    /// If the name is already present, returns the existing ID.
    #[allow(dead_code)]
    pub fn insert(&mut self, name: impl Into<String>) -> usize {
        let name = name.into();
        if let Some(&id) = self.index.get(&name) {
            return id;
        }
        let id = self.names.len();
        self.index.insert(name.clone(), id);
        self.names.push(name);
        id
    }
    /// Get the ID for a name, if it exists.
    #[allow(dead_code)]
    pub fn get_id(&self, name: &str) -> Option<usize> {
        self.index.get(name).copied()
    }
    /// Get the name for an ID.
    #[allow(dead_code)]
    pub fn get_name(&self, id: usize) -> Option<&str> {
        self.names.get(id).map(|s| s.as_str())
    }
    /// Get the number of names in the index.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.names.len()
    }
    /// Check if the index is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }
    /// Get all names in insertion order.
    #[allow(dead_code)]
    pub fn all_names(&self) -> &[String] {
        &self.names
    }
}
/// A pattern language over discrimination tree key sequences.
///
/// Useful for specifying which entries to retrieve or filter.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum KeyPattern {
    /// Match any single key.
    Any,
    /// Match a specific key.
    Exact(DiscrTreeKey),
    /// Match any constant key.
    AnyConst,
    /// Match a constant with specific arity.
    ConstArity(u32),
    /// Match any literal.
    AnyLit,
    /// Match any Pi key.
    AnyPi,
    /// Match any Lambda key.
    AnyLam,
    /// Match any Sort key.
    AnySort,
    /// Match any of a set of keys.
    OneOf(Vec<DiscrTreeKey>),
    /// Negate a pattern.
    Not(Box<KeyPattern>),
}
impl KeyPattern {
    /// Test if a key matches this pattern.
    #[allow(dead_code)]
    pub fn matches(&self, key: &DiscrTreeKey) -> bool {
        match self {
            KeyPattern::Any => true,
            KeyPattern::Exact(k) => k == key,
            KeyPattern::AnyConst => matches!(key, DiscrTreeKey::Const(_, _)),
            KeyPattern::ConstArity(n) => {
                matches!(key, DiscrTreeKey::Const(_, m) if m == n)
            }
            KeyPattern::AnyLit => matches!(key, DiscrTreeKey::Lit(_)),
            KeyPattern::AnyPi => matches!(key, DiscrTreeKey::Pi),
            KeyPattern::AnyLam => matches!(key, DiscrTreeKey::Lambda),
            KeyPattern::AnySort => matches!(key, DiscrTreeKey::Sort),
            KeyPattern::OneOf(ks) => ks.contains(key),
            KeyPattern::Not(inner) => !inner.matches(key),
        }
    }
    /// Check if a sequence of keys starts with a matching key.
    #[allow(dead_code)]
    pub fn matches_first(&self, keys: &[DiscrTreeKey]) -> bool {
        keys.first().map(|k| self.matches(k)).unwrap_or(false)
    }
}
/// A simp lemma entry stored in the discrimination tree.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SimpLemmaEntry {
    /// Lemma name.
    pub name: oxilean_kernel::Name,
    /// Priority (higher = tried first).
    pub priority: i32,
    /// Whether this is a conditional lemma (has side conditions).
    pub is_conditional: bool,
    /// LHS expression that this lemma rewrites.
    pub lhs: Expr,
    /// RHS expression (result of rewriting).
    pub rhs: Expr,
}
impl SimpLemmaEntry {
    /// Create a new simp lemma entry.
    #[allow(dead_code)]
    pub fn new(
        name: oxilean_kernel::Name,
        priority: i32,
        is_conditional: bool,
        lhs: Expr,
        rhs: Expr,
    ) -> Self {
        Self {
            name,
            priority,
            is_conditional,
            lhs,
            rhs,
        }
    }
    /// Create an unconditional simp lemma.
    #[allow(dead_code)]
    pub fn unconditional(name: oxilean_kernel::Name, lhs: Expr, rhs: Expr) -> Self {
        Self::new(name, 1000, false, lhs, rhs)
    }
}
