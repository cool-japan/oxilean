//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;

use super::functions::{
    compat_fold, is_combining, rope_collect, rope_depth, rope_node_len, FNV_OFFSET_BASIS, FNV_PRIME,
};

/// A node in the trie.
#[allow(dead_code)]
struct TrieNode {
    children: HashMap<char, Box<TrieNode>>,
    terminal: Option<InternedString>,
}
#[allow(dead_code)]
impl TrieNode {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            terminal: None,
        }
    }
}
/// An FNV-1a hash state for fast string hashing.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Fnv1aHasher {
    state: u64,
}
#[allow(dead_code)]
impl Fnv1aHasher {
    /// Create a new hasher.
    pub fn new() -> Self {
        Self {
            state: FNV_OFFSET_BASIS,
        }
    }
    /// Feed a byte into the hash.
    pub fn write_byte(&mut self, byte: u8) {
        self.state ^= byte as u64;
        self.state = self.state.wrapping_mul(FNV_PRIME);
    }
    /// Feed a string into the hash.
    pub fn write_str(&mut self, s: &str) {
        for byte in s.as_bytes() {
            self.write_byte(*byte);
        }
    }
    /// Get the final hash value.
    pub fn finish(&self) -> u64 {
        self.state
    }
    /// Hash a string in one call.
    pub fn hash_str(s: &str) -> u64 {
        let mut h = Self::new();
        h.write_str(s);
        h.finish()
    }
    /// Hash a string, returning a u32 by xor-folding.
    pub fn hash_str_32(s: &str) -> u32 {
        let h = Self::hash_str(s);
        ((h >> 32) ^ h) as u32
    }
}
/// Statistics for a [`StringPool`].
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PoolStatistics {
    /// Number of unique strings stored.
    pub unique_count: usize,
    /// Total bytes of unique string data stored.
    pub unique_bytes: usize,
    /// Total number of intern requests (including duplicates).
    pub total_intern_requests: usize,
    /// Total bytes that would have been stored without deduplication.
    pub total_requested_bytes: usize,
}
impl PoolStatistics {
    /// Bytes saved by deduplication.
    pub fn bytes_saved(&self) -> usize {
        self.total_requested_bytes.saturating_sub(self.unique_bytes)
    }
    /// Deduplication ratio (0.0 to 1.0). Returns 0.0 if no bytes were requested.
    pub fn dedup_ratio(&self) -> f64 {
        if self.total_requested_bytes == 0 {
            0.0
        } else {
            self.bytes_saved() as f64 / self.total_requested_bytes as f64
        }
    }
    /// Average string length.
    pub fn avg_string_len(&self) -> f64 {
        if self.unique_count == 0 {
            0.0
        } else {
            self.unique_bytes as f64 / self.unique_count as f64
        }
    }
}
/// A string interning pool.
///
/// Stores each unique string exactly once and returns lightweight handles
/// ([`InternedString`]) for fast comparison.
pub struct StringPool {
    /// Map from string content to its interned index.
    map: HashMap<String, u32>,
    /// Index-ordered storage of all unique strings.
    pub(super) strings: Vec<String>,
    /// Running statistics.
    stats: PoolStatistics,
}
impl StringPool {
    /// Create an empty string pool.
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            strings: Vec::new(),
            stats: PoolStatistics::default(),
        }
    }
    /// Create a pool pre-sized for the given capacity.
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            map: HashMap::with_capacity(cap),
            strings: Vec::with_capacity(cap),
            stats: PoolStatistics::default(),
        }
    }
    /// Intern a string. If the string is already in the pool, returns the
    /// existing handle. Otherwise inserts it and returns a new handle.
    pub fn intern(&mut self, s: &str) -> InternedString {
        self.stats.total_intern_requests += 1;
        self.stats.total_requested_bytes += s.len();
        if let Some(&idx) = self.map.get(s) {
            return InternedString { index: idx };
        }
        let idx = self.strings.len() as u32;
        self.strings.push(s.to_string());
        self.map.insert(s.to_string(), idx);
        self.stats.unique_count += 1;
        self.stats.unique_bytes += s.len();
        InternedString { index: idx }
    }
    /// Intern multiple strings at once, returning handles in the same order.
    pub fn intern_bulk(&mut self, strs: &[&str]) -> Vec<InternedString> {
        strs.iter().map(|s| self.intern(s)).collect()
    }
    /// Intern strings from an iterator.
    pub fn intern_iter<'a, I>(&mut self, iter: I) -> Vec<InternedString>
    where
        I: IntoIterator<Item = &'a str>,
    {
        iter.into_iter().map(|s| self.intern(s)).collect()
    }
    /// Resolve an interned string back to its content.
    pub fn resolve(&self, id: InternedString) -> Option<&str> {
        self.strings.get(id.index as usize).map(|s| s.as_str())
    }
    /// Look up an interned string by content. Returns `None` if the string
    /// has not been interned.
    pub fn lookup(&self, s: &str) -> Option<InternedString> {
        self.map.get(s).map(|&idx| InternedString { index: idx })
    }
    /// Check whether a string has been interned.
    pub fn contains(&self, s: &str) -> bool {
        self.map.contains_key(s)
    }
    /// Number of unique strings in the pool.
    pub fn len(&self) -> usize {
        self.strings.len()
    }
    /// Whether the pool is empty.
    pub fn is_empty(&self) -> bool {
        self.strings.is_empty()
    }
    /// Get pool statistics.
    pub fn statistics(&self) -> &PoolStatistics {
        &self.stats
    }
    /// Create a snapshot of the pool for serialization.
    pub fn snapshot(&self) -> PoolSnapshot {
        PoolSnapshot {
            strings: self.strings.clone(),
        }
    }
    /// Iterate over all interned strings with their handles.
    pub fn iter(&self) -> impl Iterator<Item = (InternedString, &str)> {
        self.strings
            .iter()
            .enumerate()
            .map(|(i, s)| (InternedString { index: i as u32 }, s.as_str()))
    }
    /// Clear the pool, removing all interned strings.
    pub fn clear(&mut self) {
        self.map.clear();
        self.strings.clear();
        self.stats.unique_count = 0;
        self.stats.unique_bytes = 0;
    }
    /// Merge another pool into this one. Returns a mapping from old indices to
    /// new indices.
    pub fn merge(&mut self, other: &StringPool) -> Vec<InternedString> {
        other.strings.iter().map(|s| self.intern(s)).collect()
    }
}
/// A handle that includes a generation marker for garbage-collection-friendly
/// use cases. The generation allows detecting stale handles after pool compaction.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GenerationalString {
    index: u32,
    generation: u16,
}
#[allow(dead_code)]
impl GenerationalString {
    /// The raw index component.
    pub fn index(self) -> u32 {
        self.index
    }
    /// The generation component.
    pub fn generation(self) -> u16 {
        self.generation
    }
}
/// A simple categorization of a string's character content.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StringCategory {
    /// All ASCII alphanumeric.
    AlphaNum,
    /// All ASCII letters.
    Alpha,
    /// All ASCII digits.
    Numeric,
    /// Contains only whitespace.
    Whitespace,
    /// Empty string.
    Empty,
    /// Identifier-like (starts with letter/underscore, rest alphanumeric/underscore).
    Identifier,
    /// Other.
    Mixed,
}
/// A string pool backed by a trie for fast prefix enumeration.
#[allow(dead_code)]
pub struct TriePool {
    root: TrieNode,
    pool: StringPool,
}
#[allow(dead_code)]
impl TriePool {
    /// Create an empty trie pool.
    pub fn new() -> Self {
        Self {
            root: TrieNode::new(),
            pool: StringPool::new(),
        }
    }
    /// Insert a string into the trie pool.
    pub fn insert(&mut self, s: &str) -> InternedString {
        let id = self.pool.intern(s);
        let mut node = &mut self.root;
        for ch in s.chars() {
            node = node
                .children
                .entry(ch)
                .or_insert_with(|| Box::new(TrieNode::new()));
        }
        node.terminal = Some(id);
        id
    }
    /// Collect all strings in the subtrie rooted at `node`.
    fn collect_all(node: &TrieNode, result: &mut Vec<InternedString>) {
        if let Some(id) = node.terminal {
            result.push(id);
        }
        for child in node.children.values() {
            Self::collect_all(child, result);
        }
    }
    /// Find all strings with the given prefix.
    pub fn find_prefix(&self, prefix: &str) -> Vec<InternedString> {
        let mut node = &self.root;
        for ch in prefix.chars() {
            match node.children.get(&ch) {
                Some(child) => node = child,
                None => return Vec::new(),
            }
        }
        let mut result = Vec::new();
        Self::collect_all(node, &mut result);
        result
    }
    /// Exact lookup.
    pub fn get(&self, s: &str) -> Option<InternedString> {
        self.pool.lookup(s)
    }
    /// Resolve an interned string.
    pub fn resolve(&self, id: InternedString) -> Option<&str> {
        self.pool.resolve(id)
    }
    /// Number of strings.
    pub fn len(&self) -> usize {
        self.pool.len()
    }
    /// Whether the pool is empty.
    pub fn is_empty(&self) -> bool {
        self.pool.is_empty()
    }
}
/// Estimates future pool growth based on historical intern rates.
#[allow(dead_code)]
pub struct PoolGrowthEstimator {
    history: Vec<usize>,
    window: usize,
}
#[allow(dead_code)]
impl PoolGrowthEstimator {
    /// Create a new estimator with the given smoothing window.
    pub fn new(window: usize) -> Self {
        Self {
            history: Vec::new(),
            window: window.max(1),
        }
    }
    /// Record the current pool size.
    pub fn record(&mut self, size: usize) {
        self.history.push(size);
        if self.history.len() > self.window * 2 {
            let drain_to = self.history.len() - self.window;
            self.history.drain(..drain_to);
        }
    }
    /// Estimate the average growth rate (strings per observation).
    pub fn avg_growth(&self) -> f64 {
        if self.history.len() < 2 {
            return 0.0;
        }
        let _n = self.history.len();
        let deltas: Vec<f64> = self
            .history
            .windows(2)
            .map(|w| w[1] as f64 - w[0] as f64)
            .collect();
        deltas.iter().sum::<f64>() / deltas.len() as f64
    }
    /// Estimate the pool size after `n` more observations.
    pub fn estimate_after(&self, n: usize) -> f64 {
        let last = self.history.last().copied().unwrap_or(0) as f64;
        last + self.avg_growth() * n as f64
    }
    /// Number of recorded observations.
    pub fn observation_count(&self) -> usize {
        self.history.len()
    }
}
/// A rope data structure for efficient large string construction.
///
/// A rope is a binary tree of string slices that supports O(log n)
/// concatenation. Flattening to a `String` is O(n).
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Rope {
    pub(super) root: Option<RopeNode>,
}
#[allow(dead_code)]
impl Rope {
    /// Create an empty rope.
    pub fn new() -> Self {
        Self { root: None }
    }
    /// Create a rope from a string.
    pub fn from_str(s: &str) -> Self {
        if s.is_empty() {
            Self { root: None }
        } else {
            Self {
                root: Some(RopeNode::Leaf(s.to_string())),
            }
        }
    }
    /// Total byte length.
    pub fn len(&self) -> usize {
        match &self.root {
            None => 0,
            Some(node) => rope_node_len(node),
        }
    }
    /// Whether the rope is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Concatenate two ropes.
    pub fn concat(self, other: Rope) -> Rope {
        match (self.root, other.root) {
            (None, r) => Rope { root: r },
            (l, None) => Rope { root: l },
            (Some(l), Some(r)) => {
                let total = rope_node_len(&l) + rope_node_len(&r);
                Rope {
                    root: Some(RopeNode::Concat(Box::new(l), Box::new(r), total)),
                }
            }
        }
    }
    /// Append a string slice to the rope.
    pub fn append_str(self, s: &str) -> Rope {
        self.concat(Rope::from_str(s))
    }
    /// Flatten the rope into a `String`.
    ///
    /// Note: This uses the `Display` implementation which performs
    /// the same in-order traversal of the rope tree.
    pub fn collect_string(&self) -> String {
        // Use Display trait (which does the same rope traversal)
        use std::fmt::Write;
        let mut buf = String::with_capacity(self.len());
        if let Some(node) = &self.root {
            rope_collect(node, &mut buf);
        }
        buf
    }
    /// Depth of the rope tree (for balance diagnostics).
    pub fn depth(&self) -> usize {
        match &self.root {
            None => 0,
            Some(node) => rope_depth(node),
        }
    }
}
/// The result of comparing two pool snapshots.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PoolDiff {
    /// Strings present in `new` but not in `old`.
    pub added: Vec<String>,
    /// Strings present in `old` but not in `new`.
    pub removed: Vec<String>,
    /// Strings present in both.
    pub common: Vec<String>,
}
#[allow(dead_code)]
impl PoolDiff {
    /// Compute the diff between two snapshots.
    pub fn compute(old: &PoolSnapshot, new: &PoolSnapshot) -> Self {
        use std::collections::HashSet;
        let old_set: HashSet<&str> = old.strings.iter().map(|s| s.as_str()).collect();
        let new_set: HashSet<&str> = new.strings.iter().map(|s| s.as_str()).collect();
        let added = new_set
            .difference(&old_set)
            .map(|s| s.to_string())
            .collect();
        let removed = old_set
            .difference(&new_set)
            .map(|s| s.to_string())
            .collect();
        let common = old_set
            .intersection(&new_set)
            .map(|s| s.to_string())
            .collect();
        Self {
            added,
            removed,
            common,
        }
    }
    /// Number of added strings.
    pub fn added_count(&self) -> usize {
        self.added.len()
    }
    /// Number of removed strings.
    pub fn removed_count(&self) -> usize {
        self.removed.len()
    }
    /// Whether the two snapshots are identical.
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty()
    }
}
/// A string pool that normalizes strings to lowercase before interning,
/// enabling case-insensitive comparisons via cheap integer equality.
#[allow(dead_code)]
pub struct CaseFoldPool {
    inner: StringPool,
}
#[allow(dead_code)]
impl CaseFoldPool {
    /// Create an empty case-folding pool.
    pub fn new() -> Self {
        Self {
            inner: StringPool::new(),
        }
    }
    /// Intern a string after folding to lowercase.
    pub fn intern(&mut self, s: &str) -> InternedString {
        let folded = s.to_lowercase();
        self.inner.intern(&folded)
    }
    /// Resolve back to the canonicalized (lowercase) form.
    pub fn resolve(&self, id: InternedString) -> Option<&str> {
        self.inner.resolve(id)
    }
    /// Check whether the lowercase version of `s` has been interned.
    pub fn contains(&self, s: &str) -> bool {
        self.inner.contains(&s.to_lowercase())
    }
    /// Number of unique strings.
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    /// Whether the pool is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    /// Access the inner pool.
    pub fn inner(&self) -> &StringPool {
        &self.inner
    }
}
/// Normalization form for Unicode strings.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NormForm {
    /// Decompose then recompose (canonical composition).
    Nfc,
    /// Canonical decomposition only.
    Nfd,
    /// Compatibility decomposition then recompose.
    Nfkc,
    /// Compatibility decomposition only.
    Nfkd,
}
/// A handle to an interned string. This is a lightweight index into a [`StringPool`].
///
/// Two `InternedString` values that came from the same pool compare equal
/// if and only if they refer to the same underlying string content.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct InternedString {
    /// Index into the pool's string storage.
    pub(super) index: u32,
}
impl InternedString {
    /// Create an interned string from a raw index.
    /// Intended for deserialization; use [`StringPool::intern`] for normal usage.
    pub fn from_raw(index: u32) -> Self {
        Self { index }
    }
    /// The raw index of this interned string.
    pub fn raw_index(self) -> u32 {
        self.index
    }
}
/// A serializable snapshot of a [`StringPool`].
///
/// Can be used to persist the pool to disk or transfer it across processes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PoolSnapshot {
    /// The interned strings, in index order.
    pub strings: Vec<String>,
}
impl PoolSnapshot {
    /// Number of strings in the snapshot.
    pub fn len(&self) -> usize {
        self.strings.len()
    }
    /// Whether the snapshot is empty.
    pub fn is_empty(&self) -> bool {
        self.strings.is_empty()
    }
    /// Get a string by its interned index.
    pub fn get(&self, id: InternedString) -> Option<&str> {
        self.strings.get(id.index as usize).map(|s| s.as_str())
    }
    /// Restore a [`StringPool`] from this snapshot.
    pub fn restore(&self) -> StringPool {
        let mut pool = StringPool::new();
        for s in &self.strings {
            pool.intern(s);
        }
        pool
    }
    /// Total bytes of string data in the snapshot.
    pub fn total_bytes(&self) -> usize {
        self.strings.iter().map(|s| s.len()).sum()
    }
    /// Encode the snapshot as a single byte vector (length-prefixed strings).
    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        let count = self.strings.len() as u32;
        buf.extend_from_slice(&count.to_le_bytes());
        for s in &self.strings {
            let len = s.len() as u32;
            buf.extend_from_slice(&len.to_le_bytes());
            buf.extend_from_slice(s.as_bytes());
        }
        buf
    }
    /// Decode a snapshot from bytes produced by [`Self::encode`].
    pub fn decode(data: &[u8]) -> Option<Self> {
        if data.len() < 4 {
            return None;
        }
        let count = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;
        let mut pos = 4;
        let mut strings = Vec::with_capacity(count);
        for _ in 0..count {
            if pos + 4 > data.len() {
                return None;
            }
            let len = u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]])
                as usize;
            pos += 4;
            if pos + len > data.len() {
                return None;
            }
            let s = std::str::from_utf8(&data[pos..pos + len]).ok()?;
            strings.push(s.to_string());
            pos += len;
        }
        Some(Self { strings })
    }
}
/// A pool that interns strings after Unicode normalization.
/// Uses a simple ASCII-safe approximation for common cases.
#[allow(dead_code)]
pub struct NormalizedPool {
    form: NormForm,
    inner: StringPool,
}
#[allow(dead_code)]
impl NormalizedPool {
    /// Create a normalized pool with the given normalization form.
    pub fn new(form: NormForm) -> Self {
        Self {
            form,
            inner: StringPool::new(),
        }
    }
    /// Intern a string after normalization.
    pub fn intern(&mut self, s: &str) -> InternedString {
        let normalized = self.normalize(s);
        self.inner.intern(&normalized)
    }
    /// Normalize a string according to the pool's form.
    pub fn normalize(&self, s: &str) -> String {
        match self.form {
            NormForm::Nfc | NormForm::Nfd => s.chars().filter(|c| !is_combining(*c)).collect(),
            NormForm::Nfkc | NormForm::Nfkd => s
                .chars()
                .filter(|c| !is_combining(*c))
                .map(compat_fold)
                .collect(),
        }
    }
    /// Resolve an interned string.
    pub fn resolve(&self, id: InternedString) -> Option<&str> {
        self.inner.resolve(id)
    }
    /// Pool length.
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    /// Whether the pool is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}
/// A node in the rope tree.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub(super) enum RopeNode {
    Leaf(String),
    Concat(Box<RopeNode>, Box<RopeNode>, usize),
}
/// A string pool that also supports finding the longest interned prefix
/// of a given string.
#[allow(dead_code)]
pub struct PrefixPool {
    pool: StringPool,
}
#[allow(dead_code)]
impl PrefixPool {
    /// Create a new prefix pool.
    pub fn new() -> Self {
        Self {
            pool: StringPool::new(),
        }
    }
    /// Intern a string.
    pub fn intern(&mut self, s: &str) -> InternedString {
        self.pool.intern(s)
    }
    /// Find the longest interned string that is a prefix of `s`.
    pub fn longest_prefix(&self, s: &str) -> Option<InternedString> {
        let mut best: Option<InternedString> = None;
        let mut best_len = 0usize;
        for (id, interned) in self.pool.iter() {
            if s.starts_with(interned) && interned.len() >= best_len {
                best_len = interned.len();
                best = Some(id);
            }
        }
        best
    }
    /// All interned strings that are prefixes of `s`, ordered by length descending.
    pub fn all_prefixes(&self, s: &str) -> Vec<InternedString> {
        let mut result: Vec<(usize, InternedString)> = self
            .pool
            .iter()
            .filter(|(_, interned)| s.starts_with(*interned))
            .map(|(id, interned)| (interned.len(), id))
            .collect();
        result.sort_by_key(|b| std::cmp::Reverse(b.0));
        result.into_iter().map(|(_, id)| id).collect()
    }
    /// Resolve an interned string.
    pub fn resolve(&self, id: InternedString) -> Option<&str> {
        self.pool.resolve(id)
    }
    /// Number of interned strings.
    pub fn len(&self) -> usize {
        self.pool.len()
    }
    /// Whether the pool is empty.
    pub fn is_empty(&self) -> bool {
        self.pool.is_empty()
    }
}
/// A sorted index over an existing `StringPool` for fast prefix searches.
#[allow(dead_code)]
pub struct StringIndex {
    /// Sorted (string_content, InternedString) pairs.
    sorted: Vec<(String, InternedString)>,
}
#[allow(dead_code)]
impl StringIndex {
    /// Build the index from a pool.
    pub fn build(pool: &StringPool) -> Self {
        let mut sorted: Vec<(String, InternedString)> =
            pool.iter().map(|(id, s)| (s.to_string(), id)).collect();
        sorted.sort_by(|a, b| a.0.cmp(&b.0));
        Self { sorted }
    }
    /// Find all interned strings with the given prefix.
    pub fn find_prefix(&self, prefix: &str) -> Vec<InternedString> {
        let start = self.sorted.partition_point(|(s, _)| s.as_str() < prefix);
        self.sorted[start..]
            .iter()
            .take_while(|(s, _)| s.starts_with(prefix))
            .map(|(_, id)| *id)
            .collect()
    }
    /// Find all interned strings with the given suffix.
    pub fn find_suffix(&self, suffix: &str) -> Vec<InternedString> {
        self.sorted
            .iter()
            .filter(|(s, _)| s.ends_with(suffix))
            .map(|(_, id)| *id)
            .collect()
    }
    /// Find all interned strings containing the given substring.
    pub fn find_contains(&self, sub: &str) -> Vec<InternedString> {
        self.sorted
            .iter()
            .filter(|(s, _)| s.contains(sub))
            .map(|(_, id)| *id)
            .collect()
    }
    /// Number of entries in the index.
    pub fn len(&self) -> usize {
        self.sorted.len()
    }
    /// Whether the index is empty.
    pub fn is_empty(&self) -> bool {
        self.sorted.is_empty()
    }
}
/// Writes the contents of a pool to a `Write` sink in a human-readable format.
#[allow(dead_code)]
pub struct PoolWriter;
#[allow(dead_code)]
impl PoolWriter {
    /// Write pool contents to a string.
    pub fn write_to_string(pool: &StringPool) -> String {
        let mut out = String::new();
        for (id, s) in pool.iter() {
            out.push_str(&format!("{}: {:?}\n", id.raw_index(), s));
        }
        out
    }
    /// Write pool statistics to a string.
    pub fn write_stats_to_string(pool: &StringPool) -> String {
        format!("{}", pool.statistics())
    }
}
/// The result of partitioning a pool's strings.
#[allow(dead_code)]
pub struct PoolPartition {
    pub matching: Vec<InternedString>,
    pub non_matching: Vec<InternedString>,
}
#[allow(dead_code)]
impl PoolPartition {
    /// Partition all strings in a pool by predicate.
    pub fn by<F>(pool: &StringPool, mut pred: F) -> Self
    where
        F: FnMut(&str) -> bool,
    {
        let mut matching = Vec::new();
        let mut non_matching = Vec::new();
        for (id, s) in pool.iter() {
            if pred(s) {
                matching.push(id);
            } else {
                non_matching.push(id);
            }
        }
        Self {
            matching,
            non_matching,
        }
    }
    /// Number of matching strings.
    pub fn matching_count(&self) -> usize {
        self.matching.len()
    }
    /// Number of non-matching strings.
    pub fn non_matching_count(&self) -> usize {
        self.non_matching.len()
    }
}
/// An interned sub-slice: stores a base interned string plus a byte range.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InternedSlice {
    pub base: InternedString,
    pub start: u32,
    pub end: u32,
}
#[allow(dead_code)]
impl InternedSlice {
    /// Create a new interned slice.
    pub fn new(base: InternedString, start: u32, end: u32) -> Self {
        Self { base, start, end }
    }
    /// Get the byte length of the slice.
    pub fn len(&self) -> usize {
        (self.end - self.start) as usize
    }
    /// Whether the slice is empty.
    pub fn is_empty(&self) -> bool {
        self.start >= self.end
    }
    /// Resolve the slice against a pool.
    pub fn resolve<'a>(&self, pool: &'a StringPool) -> Option<&'a str> {
        let base = pool.resolve(self.base)?;
        let start = self.start as usize;
        let end = self.end as usize;
        if end > base.len() {
            return None;
        }
        base.get(start..end)
    }
}
/// A string pool that supports generational compaction.
///
/// When `compact` is called, unreferenced strings are removed and the
/// generation counter is incremented, invalidating old handles.
#[allow(dead_code)]
pub struct GenerationalPool {
    pool: StringPool,
    generation: u16,
    ref_counts: Vec<u32>,
}
#[allow(dead_code)]
impl GenerationalPool {
    /// Create an empty generational pool.
    pub fn new() -> Self {
        Self {
            pool: StringPool::new(),
            generation: 0,
            ref_counts: Vec::new(),
        }
    }
    /// Intern a string and return a generational handle.
    pub fn intern(&mut self, s: &str) -> GenerationalString {
        let id = self.pool.intern(s);
        let idx = id.raw_index() as usize;
        while self.ref_counts.len() <= idx {
            self.ref_counts.push(0);
        }
        self.ref_counts[idx] = self.ref_counts[idx].saturating_add(1);
        GenerationalString {
            index: id.raw_index(),
            generation: self.generation,
        }
    }
    /// Release a handle, decrementing its reference count.
    pub fn release(&mut self, handle: GenerationalString) {
        if handle.generation != self.generation {
            return;
        }
        let idx = handle.index as usize;
        if idx < self.ref_counts.len() {
            self.ref_counts[idx] = self.ref_counts[idx].saturating_sub(1);
        }
    }
    /// Resolve a handle to its string content.
    pub fn resolve(&self, handle: GenerationalString) -> Option<&str> {
        if handle.generation != self.generation {
            return None;
        }
        self.pool.resolve(InternedString::from_raw(handle.index))
    }
    /// Check if a handle is still valid (correct generation and non-zero refcount).
    pub fn is_valid(&self, handle: GenerationalString) -> bool {
        if handle.generation != self.generation {
            return false;
        }
        let idx = handle.index as usize;
        idx < self.ref_counts.len() && self.ref_counts[idx] > 0
    }
    /// Compact the pool: remove strings with zero reference count.
    /// All handles from the previous generation are invalidated.
    pub fn compact(&mut self) -> usize {
        let mut new_pool = StringPool::new();
        let mut new_refs: Vec<u32> = Vec::new();
        let mut removed = 0usize;
        for (id, s) in self.pool.iter() {
            let idx = id.raw_index() as usize;
            let rc = if idx < self.ref_counts.len() {
                self.ref_counts[idx]
            } else {
                0
            };
            if rc > 0 {
                let new_id = new_pool.intern(s);
                let new_idx = new_id.raw_index() as usize;
                while new_refs.len() <= new_idx {
                    new_refs.push(0);
                }
                new_refs[new_idx] = rc;
            } else {
                removed += 1;
            }
        }
        self.pool = new_pool;
        self.ref_counts = new_refs;
        self.generation = self.generation.wrapping_add(1);
        removed
    }
    /// Current generation counter.
    pub fn generation(&self) -> u16 {
        self.generation
    }
    /// Number of live strings (with ref_count > 0).
    pub fn live_count(&self) -> usize {
        self.ref_counts.iter().filter(|&&rc| rc > 0).count()
    }
    /// Total strings in the pool (including those with rc=0).
    pub fn total_count(&self) -> usize {
        self.pool.len()
    }
}
/// A string pool that tracks how often each string is interned.
#[allow(dead_code)]
pub struct FrequencyPool {
    pool: StringPool,
    counts: Vec<u64>,
}
#[allow(dead_code)]
impl FrequencyPool {
    /// Create an empty frequency pool.
    pub fn new() -> Self {
        Self {
            pool: StringPool::new(),
            counts: Vec::new(),
        }
    }
    /// Intern a string and increment its frequency.
    pub fn intern(&mut self, s: &str) -> InternedString {
        let id = self.pool.intern(s);
        let idx = id.raw_index() as usize;
        while self.counts.len() <= idx {
            self.counts.push(0);
        }
        self.counts[idx] += 1;
        id
    }
    /// Get the frequency of an interned string.
    pub fn frequency(&self, id: InternedString) -> u64 {
        self.counts
            .get(id.raw_index() as usize)
            .copied()
            .unwrap_or(0)
    }
    /// Get the top-k most frequent strings.
    pub fn top_k(&self, k: usize) -> Vec<(InternedString, u64)> {
        let mut pairs: Vec<(InternedString, u64)> = self
            .pool
            .iter()
            .map(|(id, _)| (id, self.frequency(id)))
            .collect();
        pairs.sort_by_key(|b| std::cmp::Reverse(b.1));
        pairs.truncate(k);
        pairs
    }
    /// Resolve an interned string.
    pub fn resolve(&self, id: InternedString) -> Option<&str> {
        self.pool.resolve(id)
    }
    /// Inner pool reference.
    pub fn pool(&self) -> &StringPool {
        &self.pool
    }
    /// Total number of intern calls (sum of all frequencies).
    pub fn total_calls(&self) -> u64 {
        self.counts.iter().sum()
    }
    /// Number of unique strings.
    pub fn len(&self) -> usize {
        self.pool.len()
    }
    /// Whether the pool is empty.
    pub fn is_empty(&self) -> bool {
        self.pool.is_empty()
    }
}
/// A sorted snapshot of pool strings for deterministic output.
#[allow(dead_code)]
pub struct PoolSortedView {
    entries: Vec<(InternedString, String)>,
}
#[allow(dead_code)]
impl PoolSortedView {
    /// Build a sorted view from a pool (alphabetical order).
    pub fn build(pool: &StringPool) -> Self {
        let mut entries: Vec<(InternedString, String)> =
            pool.iter().map(|(id, s)| (id, s.to_string())).collect();
        entries.sort_by(|a, b| a.1.cmp(&b.1));
        Self { entries }
    }
    /// Iterate in sorted order.
    pub fn iter(&self) -> impl Iterator<Item = (InternedString, &str)> {
        self.entries.iter().map(|(id, s)| (*id, s.as_str()))
    }
    /// Number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Whether the view is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
