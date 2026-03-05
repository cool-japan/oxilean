//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    CaseFoldPool, Fnv1aHasher, FrequencyPool, GenerationalPool, InternedSlice, InternedString,
    NormForm, NormalizedPool, PoolDiff, PoolGrowthEstimator, PoolPartition, PoolSnapshot,
    PoolSortedView, PoolStatistics, PoolWriter, PrefixPool, Rope, RopeNode, StringCategory,
    StringIndex, StringPool, TriePool,
};
use std::fmt;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_interned_string_raw_roundtrip() {
        let is = InternedString::from_raw(42);
        assert_eq!(is.raw_index(), 42);
    }
    #[test]
    fn test_interned_string_display() {
        let is = InternedString::from_raw(7);
        assert_eq!(format!("{}", is), "#7");
    }
    #[test]
    fn test_interned_string_debug() {
        let is = InternedString::from_raw(3);
        assert_eq!(format!("{:?}", is), "InternedString(3)");
    }
    #[test]
    fn test_pool_intern_and_resolve() {
        let mut pool = StringPool::new();
        let id = pool.intern("hello");
        assert_eq!(pool.resolve(id), Some("hello"));
    }
    #[test]
    fn test_pool_deduplication() {
        let mut pool = StringPool::new();
        let id1 = pool.intern("hello");
        let id2 = pool.intern("hello");
        assert_eq!(id1, id2);
        assert_eq!(pool.len(), 1);
    }
    #[test]
    fn test_pool_multiple_strings() {
        let mut pool = StringPool::new();
        let a = pool.intern("alpha");
        let b = pool.intern("beta");
        let c = pool.intern("gamma");
        assert_ne!(a, b);
        assert_ne!(b, c);
        assert_eq!(pool.len(), 3);
        assert_eq!(pool.resolve(a), Some("alpha"));
        assert_eq!(pool.resolve(b), Some("beta"));
        assert_eq!(pool.resolve(c), Some("gamma"));
    }
    #[test]
    fn test_pool_lookup() {
        let mut pool = StringPool::new();
        pool.intern("test");
        assert!(pool.lookup("test").is_some());
        assert!(pool.lookup("nope").is_none());
    }
    #[test]
    fn test_pool_contains() {
        let mut pool = StringPool::new();
        pool.intern("present");
        assert!(pool.contains("present"));
        assert!(!pool.contains("absent"));
    }
    #[test]
    fn test_pool_intern_bulk() {
        let mut pool = StringPool::new();
        let ids = pool.intern_bulk(&["a", "b", "c", "a"]);
        assert_eq!(ids.len(), 4);
        assert_eq!(ids[0], ids[3]);
        assert_eq!(pool.len(), 3);
    }
    #[test]
    fn test_pool_intern_iter() {
        let mut pool = StringPool::new();
        let words = vec!["foo", "bar", "baz"];
        let ids = pool.intern_iter(words);
        assert_eq!(ids.len(), 3);
        assert_eq!(pool.resolve(ids[1]), Some("bar"));
    }
    #[test]
    fn test_pool_statistics_basic() {
        let mut pool = StringPool::new();
        pool.intern("hello");
        pool.intern("world");
        pool.intern("hello");
        let stats = pool.statistics();
        assert_eq!(stats.unique_count, 2);
        assert_eq!(stats.unique_bytes, 10);
        assert_eq!(stats.total_intern_requests, 3);
        assert_eq!(stats.total_requested_bytes, 15);
        assert_eq!(stats.bytes_saved(), 5);
    }
    #[test]
    fn test_pool_statistics_dedup_ratio() {
        let stats = PoolStatistics {
            unique_count: 2,
            unique_bytes: 10,
            total_intern_requests: 4,
            total_requested_bytes: 20,
        };
        assert!((stats.dedup_ratio() - 0.5).abs() < f64::EPSILON);
    }
    #[test]
    fn test_pool_statistics_avg_len() {
        let stats = PoolStatistics {
            unique_count: 5,
            unique_bytes: 50,
            total_intern_requests: 10,
            total_requested_bytes: 100,
        };
        assert!((stats.avg_string_len() - 10.0).abs() < f64::EPSILON);
    }
    #[test]
    fn test_pool_statistics_display() {
        let mut pool = StringPool::new();
        pool.intern("test");
        let s = format!("{}", pool.statistics());
        assert!(s.contains("unique: 1"));
    }
    #[test]
    fn test_snapshot_roundtrip() {
        let mut pool = StringPool::new();
        pool.intern("alpha");
        pool.intern("beta");
        pool.intern("gamma");
        let snap = pool.snapshot();
        assert_eq!(snap.len(), 3);
        assert_eq!(snap.get(InternedString::from_raw(1)), Some("beta"));
    }
    #[test]
    fn test_snapshot_restore() {
        let mut pool = StringPool::new();
        let id_a = pool.intern("one");
        let _id_b = pool.intern("two");
        let snap = pool.snapshot();
        let restored = snap.restore();
        assert_eq!(restored.len(), 2);
        assert_eq!(restored.resolve(id_a), Some("one"));
    }
    #[test]
    fn test_snapshot_encode_decode() {
        let mut pool = StringPool::new();
        pool.intern("hello");
        pool.intern("world");
        let snap = pool.snapshot();
        let encoded = snap.encode();
        let decoded = PoolSnapshot::decode(&encoded).expect("test operation should succeed");
        assert_eq!(snap, decoded);
    }
    #[test]
    fn test_snapshot_decode_empty() {
        let snap = PoolSnapshot { strings: vec![] };
        let encoded = snap.encode();
        let decoded = PoolSnapshot::decode(&encoded).expect("test operation should succeed");
        assert!(decoded.is_empty());
    }
    #[test]
    fn test_snapshot_decode_invalid() {
        assert!(PoolSnapshot::decode(&[0, 1]).is_none());
    }
    #[test]
    fn test_pool_clear() {
        let mut pool = StringPool::new();
        pool.intern("a");
        pool.intern("b");
        pool.clear();
        assert!(pool.is_empty());
        assert!(!pool.contains("a"));
    }
    #[test]
    fn test_pool_merge() {
        let mut pool1 = StringPool::new();
        pool1.intern("alpha");
        pool1.intern("beta");
        let mut pool2 = StringPool::new();
        pool2.intern("beta");
        pool2.intern("gamma");
        let mapping = pool1.merge(&pool2);
        assert_eq!(pool1.len(), 3);
        assert_eq!(pool1.resolve(mapping[0]), Some("beta"));
        assert_eq!(pool1.resolve(mapping[1]), Some("gamma"));
    }
    #[test]
    fn test_pool_iter() {
        let mut pool = StringPool::new();
        pool.intern("x");
        pool.intern("y");
        let pairs: Vec<_> = pool.iter().collect();
        assert_eq!(pairs.len(), 2);
        assert_eq!(pairs[0].1, "x");
        assert_eq!(pairs[1].1, "y");
    }
    #[test]
    fn test_pool_with_capacity() {
        let pool = StringPool::with_capacity(100);
        assert!(pool.is_empty());
    }
    #[test]
    fn test_pool_debug() {
        let mut pool = StringPool::new();
        pool.intern("test");
        let dbg = format!("{:?}", pool);
        assert!(dbg.contains("StringPool(1 strings)"));
    }
    #[test]
    fn test_pool_empty_string() {
        let mut pool = StringPool::new();
        let id = pool.intern("");
        assert_eq!(pool.resolve(id), Some(""));
        assert_eq!(pool.statistics().unique_bytes, 0);
    }
    #[test]
    fn test_snapshot_total_bytes() {
        let mut pool = StringPool::new();
        pool.intern("abc");
        pool.intern("de");
        let snap = pool.snapshot();
        assert_eq!(snap.total_bytes(), 5);
    }
    #[test]
    fn test_interned_string_ordering() {
        let a = InternedString::from_raw(1);
        let b = InternedString::from_raw(2);
        assert!(a < b);
    }
}
#[allow(dead_code)]
pub(super) fn rope_node_len(node: &RopeNode) -> usize {
    match node {
        RopeNode::Leaf(s) => s.len(),
        RopeNode::Concat(_, _, len) => *len,
    }
}
#[allow(dead_code)]
pub(super) fn rope_collect(node: &RopeNode, buf: &mut String) {
    match node {
        RopeNode::Leaf(s) => buf.push_str(s),
        RopeNode::Concat(l, r, _) => {
            rope_collect(l, buf);
            rope_collect(r, buf);
        }
    }
}
#[allow(dead_code)]
pub(super) fn rope_depth(node: &RopeNode) -> usize {
    match node {
        RopeNode::Leaf(_) => 1,
        RopeNode::Concat(l, r, _) => 1 + rope_depth(l).max(rope_depth(r)),
    }
}
#[allow(dead_code)]
pub(super) fn rope_fmt(node: &RopeNode, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match node {
        RopeNode::Leaf(s) => f.write_str(s),
        RopeNode::Concat(l, r, _) => {
            rope_fmt(l, f)?;
            rope_fmt(r, f)
        }
    }
}
/// Returns true for Unicode combining characters (approximate).
#[allow(dead_code)]
pub(super) fn is_combining(c: char) -> bool {
    let cp = c as u32;
    (0x0300..=0x036F).contains(&cp) || (0x20D0..=0x20FF).contains(&cp)
}
/// Fold compatibility characters to their base forms (partial).
#[allow(dead_code)]
pub(super) fn compat_fold(c: char) -> char {
    match c {
        '\u{FF01}'..='\u{FF5E}' => char::from_u32(c as u32 - 0xFF01 + 0x0021).unwrap_or(c),
        '\u{2070}' => '0',
        '\u{00B9}' => '1',
        '\u{00B2}' => '2',
        '\u{00B3}' => '3',
        '\u{2074}'..='\u{2079}' => char::from_u32(c as u32 - 0x2074 + 0x0034).unwrap_or(c),
        _ => c,
    }
}
/// Classify a string.
#[allow(dead_code)]
pub fn classify_string(s: &str) -> StringCategory {
    if s.is_empty() {
        return StringCategory::Empty;
    }
    if s.chars().all(|c| c.is_ascii_whitespace()) {
        return StringCategory::Whitespace;
    }
    if s.chars().all(|c| c.is_ascii_digit()) {
        return StringCategory::Numeric;
    }
    if s.chars().all(|c| c.is_ascii_alphabetic()) {
        return StringCategory::Alpha;
    }
    if s.chars().all(|c| c.is_ascii_alphanumeric()) {
        return StringCategory::AlphaNum;
    }
    let mut chars = s.chars();
    if let Some(first) = chars.next() {
        if (first.is_ascii_alphabetic() || first == '_')
            && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
        {
            return StringCategory::Identifier;
        }
    }
    StringCategory::Mixed
}
pub(super) const FNV_OFFSET_BASIS: u64 = 14695981039346656037;
pub(super) const FNV_PRIME: u64 = 1099511628211;
/// Split a string by a delimiter and intern each piece.
#[allow(dead_code)]
pub fn tokenize_and_intern(pool: &mut StringPool, s: &str, delimiter: char) -> Vec<InternedString> {
    s.split(delimiter)
        .filter(|piece| !piece.is_empty())
        .map(|piece| pool.intern(piece))
        .collect()
}
/// Join interned strings from a pool using a separator.
#[allow(dead_code)]
pub fn join_interned(pool: &StringPool, ids: &[InternedString], sep: &str) -> String {
    ids.iter()
        .filter_map(|id| pool.resolve(*id))
        .collect::<Vec<_>>()
        .join(sep)
}
#[cfg(test)]
mod tests_extended {
    use super::*;
    #[test]
    fn test_case_fold_pool_basic() {
        let mut pool = CaseFoldPool::new();
        let id1 = pool.intern("Hello");
        let id2 = pool.intern("HELLO");
        let id3 = pool.intern("hello");
        assert_eq!(id1, id2);
        assert_eq!(id2, id3);
        assert_eq!(pool.resolve(id1), Some("hello"));
    }
    #[test]
    fn test_case_fold_pool_contains() {
        let mut pool = CaseFoldPool::new();
        pool.intern("Rust");
        assert!(pool.contains("rust"));
        assert!(pool.contains("RUST"));
        assert!(!pool.contains("python"));
    }
    #[test]
    fn test_case_fold_pool_len() {
        let mut pool = CaseFoldPool::new();
        pool.intern("Alpha");
        pool.intern("alpha");
        pool.intern("ALPHA");
        assert_eq!(pool.len(), 1);
    }
    #[test]
    fn test_prefix_pool_longest_prefix() {
        let mut pool = PrefixPool::new();
        pool.intern("foo");
        pool.intern("foobar");
        pool.intern("foobarbaz");
        let lp = pool
            .longest_prefix("foobarbazqux")
            .expect("test operation should succeed");
        assert_eq!(pool.resolve(lp), Some("foobarbaz"));
    }
    #[test]
    fn test_prefix_pool_all_prefixes() {
        let mut pool = PrefixPool::new();
        pool.intern("a");
        pool.intern("ab");
        pool.intern("abc");
        pool.intern("xyz");
        let prefixes = pool.all_prefixes("abcdef");
        let strs: Vec<&str> = prefixes
            .iter()
            .map(|id| pool.resolve(*id).expect("lookup should succeed"))
            .collect();
        assert!(strs.contains(&"abc"));
        assert!(strs.contains(&"ab"));
        assert!(strs.contains(&"a"));
        assert!(!strs.contains(&"xyz"));
        assert_eq!(strs[0], "abc");
    }
    #[test]
    fn test_prefix_pool_no_prefix() {
        let mut pool = PrefixPool::new();
        pool.intern("xyz");
        assert!(pool.longest_prefix("abc").is_none());
    }
    #[test]
    fn test_rope_basic() {
        let r = Rope::from_str("hello");
        assert_eq!(r.len(), 5);
        assert_eq!(r.to_string(), "hello");
    }
    #[test]
    fn test_rope_concat() {
        let a = Rope::from_str("hello");
        let b = Rope::from_str(", world");
        let c = a.concat(b);
        assert_eq!(c.to_string(), "hello, world");
        assert_eq!(c.len(), 12);
    }
    #[test]
    fn test_rope_append_str() {
        let r = Rope::from_str("foo").append_str("bar").append_str("baz");
        assert_eq!(r.to_string(), "foobarbaz");
    }
    #[test]
    fn test_rope_empty() {
        let r = Rope::new();
        assert!(r.is_empty());
        assert_eq!(r.to_string(), "");
    }
    #[test]
    fn test_rope_concat_empty() {
        let a = Rope::new();
        let b = Rope::from_str("hello");
        let c = a.concat(b);
        assert_eq!(c.to_string(), "hello");
    }
    #[test]
    fn test_rope_depth() {
        let r = Rope::from_str("a")
            .concat(Rope::from_str("b"))
            .concat(Rope::from_str("c"));
        assert!(r.depth() >= 2);
    }
    #[test]
    fn test_rope_display() {
        let r = Rope::from_str("hello").concat(Rope::from_str(" world"));
        let s = format!("{}", r);
        assert_eq!(s, "hello world");
    }
    #[test]
    fn test_normalized_pool_nfc() {
        let mut pool = NormalizedPool::new(NormForm::Nfc);
        let id = pool.intern("hello");
        assert_eq!(pool.resolve(id), Some("hello"));
    }
    #[test]
    fn test_normalized_pool_nfkc_fullwidth() {
        let mut pool = NormalizedPool::new(NormForm::Nfkc);
        let s = "\u{FF01}";
        let id = pool.intern(s);
        let resolved = pool.resolve(id).expect("lookup should succeed");
        assert_eq!(resolved, "!");
    }
    #[test]
    fn test_normalized_pool_len() {
        let mut pool = NormalizedPool::new(NormForm::Nfc);
        pool.intern("test");
        pool.intern("test");
        assert_eq!(pool.len(), 1);
    }
    #[test]
    fn test_generational_pool_basic() {
        let mut pool = GenerationalPool::new();
        let h = pool.intern("hello");
        assert_eq!(pool.resolve(h), Some("hello"));
        assert!(pool.is_valid(h));
    }
    #[test]
    fn test_generational_pool_release() {
        let mut pool = GenerationalPool::new();
        let h = pool.intern("hello");
        pool.release(h);
        assert!(!pool.is_valid(h));
    }
    #[test]
    fn test_generational_pool_compact() {
        let mut pool = GenerationalPool::new();
        let h_live = pool.intern("live");
        let h_dead = pool.intern("dead");
        pool.release(h_dead);
        let removed = pool.compact();
        assert_eq!(removed, 1);
        assert!(!pool.is_valid(h_live));
        assert_eq!(pool.generation(), 1);
        assert_eq!(pool.live_count(), 1);
    }
    #[test]
    fn test_generational_pool_stale_resolve() {
        let mut pool = GenerationalPool::new();
        let h = pool.intern("test");
        pool.compact();
        assert!(pool.resolve(h).is_none());
    }
    #[test]
    fn test_string_index_prefix() {
        let mut pool = StringPool::new();
        pool.intern("apple");
        pool.intern("application");
        pool.intern("banana");
        pool.intern("apply");
        let index = StringIndex::build(&pool);
        let results = index.find_prefix("app");
        let strs: Vec<&str> = results
            .iter()
            .map(|id| pool.resolve(*id).expect("lookup should succeed"))
            .collect();
        assert!(strs.contains(&"apple"));
        assert!(strs.contains(&"application"));
        assert!(strs.contains(&"apply"));
        assert!(!strs.contains(&"banana"));
    }
    #[test]
    fn test_string_index_suffix() {
        let mut pool = StringPool::new();
        pool.intern("test");
        pool.intern("best");
        pool.intern("rest");
        pool.intern("apple");
        let index = StringIndex::build(&pool);
        let results = index.find_suffix("est");
        assert_eq!(results.len(), 3);
    }
    #[test]
    fn test_string_index_contains() {
        let mut pool = StringPool::new();
        pool.intern("hello");
        pool.intern("world");
        pool.intern("ell");
        let index = StringIndex::build(&pool);
        let results = index.find_contains("ell");
        let strs: Vec<&str> = results
            .iter()
            .map(|id| pool.resolve(*id).expect("lookup should succeed"))
            .collect();
        assert!(strs.contains(&"hello"));
        assert!(strs.contains(&"ell"));
        assert!(!strs.contains(&"world"));
    }
    #[test]
    fn test_pool_diff_compute() {
        let mut old_pool = StringPool::new();
        old_pool.intern("alpha");
        old_pool.intern("beta");
        let old_snap = old_pool.snapshot();
        let mut new_pool = StringPool::new();
        new_pool.intern("beta");
        new_pool.intern("gamma");
        let new_snap = new_pool.snapshot();
        let diff = PoolDiff::compute(&old_snap, &new_snap);
        assert!(diff.added.contains(&"gamma".to_string()));
        assert!(diff.removed.contains(&"alpha".to_string()));
        assert!(diff.common.contains(&"beta".to_string()));
    }
    #[test]
    fn test_pool_diff_empty() {
        let mut pool = StringPool::new();
        pool.intern("x");
        let snap = pool.snapshot();
        let diff = PoolDiff::compute(&snap, &snap);
        assert!(diff.is_empty());
    }
    #[test]
    fn test_classify_string() {
        assert_eq!(classify_string(""), StringCategory::Empty);
        assert_eq!(classify_string("   "), StringCategory::Whitespace);
        assert_eq!(classify_string("123"), StringCategory::Numeric);
        assert_eq!(classify_string("abc"), StringCategory::Alpha);
        assert_eq!(classify_string("abc123"), StringCategory::AlphaNum);
        assert_eq!(classify_string("my_var"), StringCategory::Identifier);
        assert_eq!(classify_string("hello world!"), StringCategory::Mixed);
    }
    #[test]
    fn test_frequency_pool_basic() {
        let mut pool = FrequencyPool::new();
        pool.intern("hello");
        pool.intern("hello");
        pool.intern("world");
        let id_hello = pool.pool().lookup("hello").expect("lookup should succeed");
        let id_world = pool.pool().lookup("world").expect("lookup should succeed");
        assert_eq!(pool.frequency(id_hello), 2);
        assert_eq!(pool.frequency(id_world), 1);
    }
    #[test]
    fn test_frequency_pool_top_k() {
        let mut pool = FrequencyPool::new();
        for _ in 0..5 {
            pool.intern("a");
        }
        for _ in 0..3 {
            pool.intern("b");
        }
        for _ in 0..1 {
            pool.intern("c");
        }
        let top = pool.top_k(2);
        let top_strs: Vec<&str> = top
            .iter()
            .map(|(id, _)| pool.resolve(*id).expect("lookup should succeed"))
            .collect();
        assert_eq!(top_strs[0], "a");
        assert_eq!(top_strs[1], "b");
    }
    #[test]
    fn test_frequency_pool_total_calls() {
        let mut pool = FrequencyPool::new();
        pool.intern("x");
        pool.intern("x");
        pool.intern("y");
        assert_eq!(pool.total_calls(), 3);
    }
    #[test]
    fn test_fnv1a_consistency() {
        let h1 = Fnv1aHasher::hash_str("hello");
        let h2 = Fnv1aHasher::hash_str("hello");
        assert_eq!(h1, h2);
    }
    #[test]
    fn test_fnv1a_different_strings() {
        let h1 = Fnv1aHasher::hash_str("hello");
        let h2 = Fnv1aHasher::hash_str("world");
        assert_ne!(h1, h2);
    }
    #[test]
    fn test_fnv1a_32() {
        let h = Fnv1aHasher::hash_str_32("test");
        assert_ne!(h, 0);
    }
    #[test]
    fn test_fnv1a_empty() {
        let h = Fnv1aHasher::hash_str("");
        assert_eq!(h, FNV_OFFSET_BASIS);
    }
    #[test]
    fn test_pool_writer() {
        let mut pool = StringPool::new();
        pool.intern("alpha");
        pool.intern("beta");
        let out = PoolWriter::write_to_string(&pool);
        assert!(out.contains("alpha"));
        assert!(out.contains("beta"));
    }
    #[test]
    fn test_tokenize_and_intern() {
        let mut pool = StringPool::new();
        let ids = tokenize_and_intern(&mut pool, "foo,bar,baz,foo", ',');
        assert_eq!(ids.len(), 4);
        assert_eq!(ids[0], ids[3]);
        assert_eq!(pool.len(), 3);
    }
    #[test]
    fn test_join_interned() {
        let mut pool = StringPool::new();
        let id_a = pool.intern("hello");
        let id_b = pool.intern("world");
        let joined = join_interned(&pool, &[id_a, id_b], ", ");
        assert_eq!(joined, "hello, world");
    }
    #[test]
    fn test_join_interned_empty() {
        let pool = StringPool::new();
        let joined = join_interned(&pool, &[], ", ");
        assert_eq!(joined, "");
    }
}
/// Compute the Levenshtein edit distance between two strings.
#[allow(dead_code)]
pub fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let m = a_chars.len();
    let n = b_chars.len();
    if m == 0 {
        return n;
    }
    if n == 0 {
        return m;
    }
    let mut dp: Vec<usize> = (0..=n).collect();
    for i in 1..=m {
        let mut prev = dp[0];
        dp[0] = i;
        for j in 1..=n {
            let temp = dp[j];
            dp[j] = if a_chars[i - 1] == b_chars[j - 1] {
                prev
            } else {
                1 + prev.min(dp[j]).min(dp[j - 1])
            };
            prev = temp;
        }
    }
    dp[n]
}
/// Normalized similarity in [0.0, 1.0] (1.0 = identical).
#[allow(dead_code)]
pub fn normalized_similarity(a: &str, b: &str) -> f64 {
    let max_len = a.chars().count().max(b.chars().count());
    if max_len == 0 {
        return 1.0;
    }
    let dist = levenshtein_distance(a, b);
    1.0 - dist as f64 / max_len as f64
}
/// Find all interned strings within `max_distance` of `query`.
#[allow(dead_code)]
pub fn fuzzy_search(
    pool: &StringPool,
    query: &str,
    max_distance: usize,
) -> Vec<(InternedString, usize)> {
    pool.iter()
        .filter_map(|(id, s)| {
            let dist = levenshtein_distance(query, s);
            if dist <= max_distance {
                Some((id, dist))
            } else {
                None
            }
        })
        .collect()
}
/// Compute a simple checksum (sum of all bytes mod 2^32) over a pool snapshot.
#[allow(dead_code)]
pub fn pool_checksum(snap: &PoolSnapshot) -> u32 {
    let encoded = snap.encode();
    encoded
        .iter()
        .fold(0u32, |acc, &b| acc.wrapping_add(b as u32))
}
/// Validate a snapshot's checksum against an expected value.
#[allow(dead_code)]
pub fn validate_checksum(snap: &PoolSnapshot, expected: u32) -> bool {
    pool_checksum(snap) == expected
}
#[cfg(test)]
mod tests_extended2 {
    use super::*;
    #[test]
    fn test_trie_pool_insert_and_find() {
        let mut pool = TriePool::new();
        pool.insert("apple");
        pool.insert("application");
        pool.insert("apply");
        pool.insert("banana");
        let results = pool.find_prefix("app");
        assert_eq!(results.len(), 3);
        let no_results = pool.find_prefix("xyz");
        assert!(no_results.is_empty());
    }
    #[test]
    fn test_trie_pool_exact() {
        let mut pool = TriePool::new();
        pool.insert("hello");
        assert!(pool.get("hello").is_some());
        assert!(pool.get("hell").is_none());
    }
    #[test]
    fn test_trie_pool_dedup() {
        let mut pool = TriePool::new();
        let id1 = pool.insert("test");
        let id2 = pool.insert("test");
        assert_eq!(id1, id2);
        assert_eq!(pool.len(), 1);
    }
    #[test]
    fn test_growth_estimator_basic() {
        let mut est = PoolGrowthEstimator::new(5);
        for i in 0..10 {
            est.record(i * 5);
        }
        let growth = est.avg_growth();
        assert!((growth - 5.0).abs() < f64::EPSILON);
    }
    #[test]
    fn test_growth_estimator_estimate() {
        let mut est = PoolGrowthEstimator::new(5);
        est.record(0);
        est.record(10);
        est.record(20);
        let future = est.estimate_after(3);
        assert!((future - 50.0).abs() < f64::EPSILON);
    }
    #[test]
    fn test_growth_estimator_no_history() {
        let est = PoolGrowthEstimator::new(5);
        assert_eq!(est.avg_growth(), 0.0);
        assert_eq!(est.estimate_after(5), 0.0);
    }
    #[test]
    fn test_levenshtein_identical() {
        assert_eq!(levenshtein_distance("hello", "hello"), 0);
    }
    #[test]
    fn test_levenshtein_empty() {
        assert_eq!(levenshtein_distance("", "abc"), 3);
        assert_eq!(levenshtein_distance("abc", ""), 3);
    }
    #[test]
    fn test_levenshtein_one_edit() {
        assert_eq!(levenshtein_distance("kitten", "sitten"), 1);
    }
    #[test]
    fn test_levenshtein_classic() {
        assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
    }
    #[test]
    fn test_normalized_similarity_identical() {
        assert!((normalized_similarity("abc", "abc") - 1.0).abs() < f64::EPSILON);
    }
    #[test]
    fn test_normalized_similarity_empty() {
        assert!((normalized_similarity("", "") - 1.0).abs() < f64::EPSILON);
    }
    #[test]
    fn test_fuzzy_search() {
        let mut pool = StringPool::new();
        pool.intern("hello");
        pool.intern("helo");
        pool.intern("world");
        let results = fuzzy_search(&pool, "hello", 1);
        let strs: Vec<&str> = results
            .iter()
            .map(|(id, _)| pool.resolve(*id).expect("lookup should succeed"))
            .collect();
        assert!(strs.contains(&"hello"));
        assert!(strs.contains(&"helo"));
        assert!(!strs.contains(&"world"));
    }
    #[test]
    fn test_pool_partition() {
        let mut pool = StringPool::new();
        pool.intern("foo");
        pool.intern("foobar");
        pool.intern("bar");
        let part = PoolPartition::by(&pool, |s| s.starts_with("foo"));
        assert_eq!(part.matching_count(), 2);
        assert_eq!(part.non_matching_count(), 1);
    }
    #[test]
    fn test_interned_slice_resolve() {
        let mut pool = StringPool::new();
        let id = pool.intern("hello world");
        let slice = InternedSlice::new(id, 6, 11);
        assert_eq!(slice.resolve(&pool), Some("world"));
    }
    #[test]
    fn test_interned_slice_empty() {
        let mut pool = StringPool::new();
        let id = pool.intern("test");
        let slice = InternedSlice::new(id, 2, 2);
        assert!(slice.is_empty());
    }
    #[test]
    fn test_interned_slice_out_of_bounds() {
        let mut pool = StringPool::new();
        let id = pool.intern("hi");
        let slice = InternedSlice::new(id, 0, 100);
        assert!(slice.resolve(&pool).is_none());
    }
    #[test]
    fn test_pool_checksum_deterministic() {
        let mut pool = StringPool::new();
        pool.intern("alpha");
        pool.intern("beta");
        let snap = pool.snapshot();
        let c1 = pool_checksum(&snap);
        let c2 = pool_checksum(&snap);
        assert_eq!(c1, c2);
    }
    #[test]
    fn test_pool_checksum_validate() {
        let mut pool = StringPool::new();
        pool.intern("test");
        let snap = pool.snapshot();
        let checksum = pool_checksum(&snap);
        assert!(validate_checksum(&snap, checksum));
        assert!(!validate_checksum(&snap, checksum.wrapping_add(1)));
    }
}
/// Check if `b` is a rotation of `a` (i.e., `b` appears as a substring
/// within the string formed by concatenating `a` with itself).
#[allow(dead_code)]
pub fn is_rotation(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }
    if a.is_empty() {
        return true;
    }
    let doubled = format!("{}{}", a, a);
    doubled.contains(b)
}
/// Check if `s` is a palindrome (bytes-level).
#[allow(dead_code)]
pub fn is_palindrome(s: &str) -> bool {
    let bytes = s.as_bytes();
    let n = bytes.len();
    if n == 0 {
        return true;
    }
    let mut i = 0;
    let mut j = n - 1;
    while i < j {
        if bytes[i] != bytes[j] {
            return false;
        }
        i += 1;
        j -= 1;
    }
    true
}
/// Reverse a string character by character.
#[allow(dead_code)]
pub fn reverse_str(s: &str) -> String {
    s.chars().rev().collect()
}
/// Capitalize the first character of `s`, leave the rest unchanged.
#[allow(dead_code)]
pub fn capitalize(s: &str) -> String {
    let mut iter = s.chars();
    match iter.next() {
        None => String::new(),
        Some(first) => {
            let upper: String = first.to_uppercase().collect();
            upper + iter.as_str()
        }
    }
}
/// Convert a string to title case (capitalize each word separated by whitespace).
#[allow(dead_code)]
pub fn to_title_case(s: &str) -> String {
    s.split_whitespace()
        .map(capitalize)
        .collect::<Vec<_>>()
        .join(" ")
}
/// Count the occurrences of `pattern` in `text` (non-overlapping).
#[allow(dead_code)]
pub fn count_occurrences(text: &str, pattern: &str) -> usize {
    if pattern.is_empty() {
        return 0;
    }
    let mut count = 0;
    let mut start = 0;
    while let Some(pos) = text[start..].find(pattern) {
        count += 1;
        start += pos + pattern.len();
    }
    count
}
/// Truncate a string to `max_chars` characters, appending `ellipsis` if truncated.
#[allow(dead_code)]
pub fn truncate_str(s: &str, max_chars: usize, ellipsis: &str) -> String {
    let char_count = s.chars().count();
    if char_count <= max_chars {
        s.to_string()
    } else {
        let elided = max_chars.saturating_sub(ellipsis.chars().count());
        let truncated: String = s.chars().take(elided).collect();
        truncated + ellipsis
    }
}
#[cfg(test)]
mod tests_string_utils {
    use super::*;
    #[test]
    fn test_is_rotation() {
        assert!(is_rotation("abcde", "cdeab"));
        assert!(!is_rotation("abcde", "abced"));
        assert!(is_rotation("", ""));
    }
    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome("racecar"));
        assert!(is_palindrome(""));
        assert!(!is_palindrome("hello"));
    }
    #[test]
    fn test_reverse_str() {
        assert_eq!(reverse_str("hello"), "olleh");
        assert_eq!(reverse_str(""), "");
    }
    #[test]
    fn test_capitalize() {
        assert_eq!(capitalize("hello"), "Hello");
        assert_eq!(capitalize(""), "");
        assert_eq!(capitalize("WORLD"), "WORLD");
    }
    #[test]
    fn test_to_title_case() {
        assert_eq!(to_title_case("hello world"), "Hello World");
        assert_eq!(to_title_case("foo bar baz"), "Foo Bar Baz");
    }
    #[test]
    fn test_count_occurrences() {
        assert_eq!(count_occurrences("hello world hello", "hello"), 2);
        assert_eq!(count_occurrences("aaa", "aa"), 1);
        assert_eq!(count_occurrences("test", ""), 0);
    }
    #[test]
    fn test_truncate_str() {
        assert_eq!(truncate_str("hello world", 5, "..."), "he...");
        assert_eq!(truncate_str("hi", 10, "..."), "hi");
        assert_eq!(truncate_str("abcde", 5, "..."), "abcde");
    }
}
#[cfg(test)]
mod tests_sorted_view {
    use super::*;
    #[test]
    fn test_sorted_view_order() {
        let mut pool = StringPool::new();
        pool.intern("zebra");
        pool.intern("apple");
        pool.intern("mango");
        let view = PoolSortedView::build(&pool);
        let strs: Vec<&str> = view.iter().map(|(_, s)| s).collect();
        assert_eq!(strs, vec!["apple", "mango", "zebra"]);
    }
    #[test]
    fn test_sorted_view_len() {
        let mut pool = StringPool::new();
        pool.intern("a");
        pool.intern("b");
        let view = PoolSortedView::build(&pool);
        assert_eq!(view.len(), 2);
    }
}
