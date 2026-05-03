//! Impl blocks for expr_cache

use super::super::functions::*;
use std::collections::{HashMap, VecDeque};

use super::defs::*;

impl<V> AdaptiveCacheEntry<V> {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(value: V, priority: CachePriority, now: u64) -> Self {
        Self {
            value,
            priority,
            access_count: 0,
            last_access: now,
            insert_time: now,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn touch(&mut self, now: u64) {
        self.access_count += 1;
        self.last_access = now;
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn eviction_score(&self, now: u64) -> f64 {
        let age = (now - self.last_access) as f64;
        let freq = (self.access_count + 1) as f64;
        let boost = match self.priority {
            CachePriority::Pinned => f64::INFINITY,
            CachePriority::High => 8.0,
            CachePriority::Normal => 4.0,
            CachePriority::Low => 1.0,
        };
        (freq * boost) / (age + 1.0)
    }
}

impl SymbolInterner {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            symbols: std::collections::HashMap::new(),
            by_id: Vec::new(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn intern(&mut self, name: &str) -> u32 {
        if let Some(&id) = self.symbols.get(name) {
            return id;
        }
        let id = self.by_id.len() as u32;
        self.by_id.push(name.to_string());
        self.symbols.insert(name.to_string(), id);
        id
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn lookup(&self, id: u32) -> Option<&str> {
        self.by_id.get(id as usize).map(|s| s.as_str())
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn contains(&self, name: &str) -> bool {
        self.symbols.contains_key(name)
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn size(&self) -> usize {
        self.by_id.len()
    }
}

impl<V: Clone> MultiLevelCache<V> {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(l1_cap: usize, l2_cap: usize) -> Self {
        Self {
            l1: WindowCache::new(l1_cap),
            l2: std::collections::HashMap::new(),
            l2_capacity: l2_cap,
            l1_hits: 0,
            l2_hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn insert(&mut self, key: u64, value: V) {
        self.l1.insert(key, value.clone());
        if self.l2.len() < self.l2_capacity {
            self.l2.insert(key, value);
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn get(&mut self, key: &u64) -> Option<V> {
        if let Some(v) = self.l1.get(key) {
            self.l1_hits += 1;
            return Some(v.clone());
        }
        if let Some(v) = self.l2.get(key) {
            self.l2_hits += 1;
            return Some(v.clone());
        }
        self.misses += 1;
        None
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn l1_hit_rate(&self) -> f64 {
        let total = self.l1_hits + self.l2_hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.l1_hits as f64 / total as f64
        }
    }
}

impl<K: std::hash::Hash + Eq, V> VersionedCache<K, V> {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            entries: std::collections::HashMap::new(),
            version: 0,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn insert(&mut self, key: K, value: V) {
        self.entries.insert(key, (value, self.version));
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn get(&self, key: &K) -> Option<&V> {
        self.entries
            .get(key)
            .and_then(|(v, ver)| if *ver == self.version { Some(v) } else { None })
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn bump_version(&mut self) {
        self.version += 1;
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn purge_stale(&mut self) {
        let v = self.version;
        self.entries.retain(|_, (_, ver)| *ver == v);
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn version(&self) -> u64 {
        self.version
    }
}

impl TokenFrequencyTable {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn record(&mut self, token: &str) {
        *self.counts.entry(token.to_string()).or_insert(0) += 1;
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn count(&self, token: &str) -> u64 {
        self.counts.get(token).copied().unwrap_or(0)
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn top_n(&self, n: usize) -> Vec<(&str, u64)> {
        let mut pairs: Vec<_> = self.counts.iter().map(|(k, &v)| (k.as_str(), v)).collect();
        pairs.sort_by_key(|b| std::cmp::Reverse(b.1));
        pairs.truncate(n);
        pairs
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn unique_tokens(&self) -> usize {
        self.counts.len()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn total_tokens(&self) -> u64 {
        self.counts.values().sum()
    }
}

impl<V> AdaptiveLruCache<V> {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(initial: usize, min: usize, max: usize) -> Self {
        Self {
            inner: LruCache::new(initial),
            min_capacity: min,
            max_capacity: max,
            hits: 0,
            misses: 0,
            tune_interval: 1000,
            ops: 0,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn insert(&mut self, key: u64, value: V) {
        self.inner.insert(key, value);
        self.ops += 1;
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn get(&mut self, key: u64) -> Option<&V> {
        self.ops += 1;
        match self.inner.get(key) {
            Some(v) => {
                self.hits += 1;
                Some(v)
            }
            None => {
                self.misses += 1;
                None
            }
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl<K: std::hash::Hash + Eq, V> PolicyCache<K, V> {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(capacity: usize) -> Self {
        Self {
            entries: std::collections::HashMap::new(),
            clock: 0,
            capacity,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn insert(&mut self, key: K, value: V) {
        self.clock += 1;
        if self.entries.len() >= self.capacity {
            let first_key: Option<K> = self.entries.keys().next().map(|k| {
                let raw = k as *const K;
                unsafe { std::ptr::read(raw) }
            });
            if let Some(fk) = first_key {
                self.entries.remove(&fk);
            }
        }
        self.entries.insert(key, (value, 0, self.clock));
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn get(&mut self, key: &K) -> Option<&V> {
        self.clock += 1;
        let now = self.clock;
        if let Some((v, ac, la)) = self.entries.get_mut(key) {
            *ac += 1;
            *la = now;
            Some(v)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl StringInterner {
    /// Create an empty interner
    #[allow(missing_docs)]
    pub fn new() -> Self {
        StringInterner {
            strings: Vec::new(),
            map: HashMap::new(),
        }
    }
    /// Intern a string, returning a deduplicated ID
    #[allow(missing_docs)]
    pub fn intern(&mut self, s: &str) -> InternedStr {
        if let Some(&id) = self.map.get(s) {
            return InternedStr(id);
        }
        let id = self.strings.len() as u32;
        self.strings.push(s.to_string());
        self.map.insert(s.to_string(), id);
        InternedStr(id)
    }
    /// Look up the string for a given ID
    #[allow(missing_docs)]
    pub fn get(&self, id: InternedStr) -> Option<&str> {
        self.strings.get(id.0 as usize).map(String::as_str)
    }
    /// Number of unique strings interned
    #[allow(missing_docs)]
    pub fn len(&self) -> usize {
        self.strings.len()
    }
    /// True if no strings have been interned
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.strings.is_empty()
    }
    /// True if the given string has already been interned
    #[allow(missing_docs)]
    pub fn contains(&self, s: &str) -> bool {
        self.map.contains_key(s)
    }
    /// Estimated memory usage in bytes
    #[allow(missing_docs)]
    pub fn memory_bytes(&self) -> usize {
        let string_bytes: usize = self.strings.iter().map(|s| s.len() + 24).sum();
        let map_bytes = self.map.len() * 64;
        string_bytes + map_bytes
    }
}

impl LfuEviction {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(min_freq: u64, age_factor: f64) -> Self {
        Self {
            min_freq,
            age_factor,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn should_evict(&self, access_count: u64, last_access: u64, now: u64) -> bool {
        let age = now.saturating_sub(last_access) as f64;
        let effective = access_count as f64 / (1.0 + age * self.age_factor);
        effective < self.min_freq as f64
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn policy_name(&self) -> &'static str {
        "LFU-Age"
    }
}

impl BloomFilter {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(size_bits: usize, num_hashes: usize) -> Self {
        let bytes = (size_bits + 7) / 8;
        Self {
            bits: vec![0u8; bytes],
            size_bits,
            num_hashes,
        }
    }
    fn bit_indices(&self, key: u64) -> Vec<usize> {
        (0..self.num_hashes)
            .map(|i| {
                let h = fnv1a_hash(&key.to_le_bytes()) ^ (i as u64 * 2654435761);
                (h as usize) % self.size_bits
            })
            .collect()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn insert(&mut self, key: u64) {
        for idx in self.bit_indices(key) {
            self.bits[idx / 8] |= 1 << (idx % 8);
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn may_contain(&self, key: u64) -> bool {
        self.bit_indices(key)
            .iter()
            .all(|&idx| self.bits[idx / 8] & (1 << (idx % 8)) != 0)
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn clear(&mut self) {
        for b in &mut self.bits {
            *b = 0;
        }
    }
}

impl ExprDiffCache {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(max_size: usize) -> Self {
        Self {
            diffs: std::collections::HashMap::new(),
            max_size,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn store(&mut self, a: u64, b: u64, diff: impl Into<String>) {
        if self.diffs.len() >= self.max_size {
            if let Some(&k) = self.diffs.keys().next() {
                self.diffs.remove(&k);
            }
        }
        let key = if a <= b { (a, b) } else { (b, a) };
        self.diffs.insert(key, diff.into());
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn lookup(&self, a: u64, b: u64) -> Option<&str> {
        let key = if a <= b { (a, b) } else { (b, a) };
        self.diffs.get(&key).map(|s| s.as_str())
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn size(&self) -> usize {
        self.diffs.len()
    }
}

impl PersistentCache {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn insert(&mut self, key: u64, value: impl Into<String>) {
        self.entries.push((key, value.into()));
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn lookup(&self, key: u64) -> Option<&str> {
        self.entries
            .iter()
            .rev()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| v.as_str())
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn serialize(&self) -> String {
        self.entries
            .iter()
            .map(|(k, v)| format!("{}:{}", k, v))
            .collect::<Vec<_>>()
            .join("|")
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn deserialize(s: &str) -> Self {
        let mut cache = Self::new();
        for part in s.split('|') {
            if let Some((k, v)) = part.split_once(':') {
                if let Ok(key) = k.parse::<u64>() {
                    cache.insert(key, v);
                }
            }
        }
        cache
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }
}

impl ParseResultCache {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: std::collections::HashMap::new(),
            max_entries,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn lookup(&mut self, source: &str) -> Option<&ParseCacheEntry> {
        let key = fnv1a_hash(source.as_bytes());
        if let Some(e) = self.entries.get_mut(&key) {
            e.use_count += 1;
            self.hits += 1;
            Some(e)
        } else {
            self.misses += 1;
            None
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn store(&mut self, source: &str, result_repr: String, parse_time_us: u64) {
        if self.entries.len() >= self.max_entries {
            if let Some((&k, _)) = self.entries.iter().min_by_key(|(_, v)| v.use_count) {
                self.entries.remove(&k);
            }
        }
        let key = fnv1a_hash(source.as_bytes());
        self.entries.insert(
            key,
            ParseCacheEntry {
                source_hash: key,
                result_repr,
                parse_time_us,
                use_count: 1,
            },
        );
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn stats(&self) -> (u64, u64, f64) {
        (self.hits, self.misses, self.hit_rate())
    }
}

impl ParseCache {
    /// Create a new cache with the given maximum number of entries
    #[allow(missing_docs)]
    pub fn new(max_entries: usize) -> Self {
        ParseCache {
            entries: HashMap::new(),
            max_entries,
            hits: 0,
            misses: 0,
        }
    }
    /// Look up a cached entry by source text
    #[allow(missing_docs)]
    pub fn lookup(&mut self, text: &str) -> Option<&CacheEntry> {
        let hash = DeclHash::compute(text);
        if let Some(entry) = self.entries.get_mut(&hash) {
            entry.hit_count += 1;
            self.hits += 1;
            return self.entries.get(&hash);
        }
        self.misses += 1;
        None
    }
    /// Insert a new entry into the cache
    #[allow(missing_docs)]
    pub fn insert(&mut self, text: &str, name: Option<String>) {
        let hash = DeclHash::compute(text);
        if self.entries.len() >= self.max_entries {
            self.evict_lru();
        }
        let entry = CacheEntry {
            hash: hash.clone(),
            source: text.to_string(),
            decl_name: name,
            hit_count: 0,
        };
        self.entries.insert(hash, entry);
    }
    /// Fraction of lookups that were cache hits (0.0 if no lookups yet)
    #[allow(missing_docs)]
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }
    /// Number of entries in the cache
    #[allow(missing_docs)]
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// True if the cache is empty
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Evict the entry with the lowest hit_count if over capacity
    #[allow(missing_docs)]
    pub fn evict_lru(&mut self) {
        if self.entries.is_empty() {
            return;
        }
        let min_key = self
            .entries
            .iter()
            .min_by_key(|(_, e)| e.hit_count)
            .map(|(k, _)| k.clone());
        if let Some(key) = min_key {
            self.entries.remove(&key);
        }
    }
    /// Clear all entries and reset statistics
    #[allow(missing_docs)]
    pub fn clear(&mut self) {
        self.entries.clear();
        self.hits = 0;
        self.misses = 0;
    }
}

impl ExprSegment {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn from_slice(src: &str, start: usize, end: usize, kind: SegmentKind) -> Self {
        let hash = fnv1a_hash(&src.as_bytes()[start..end]);
        Self {
            start,
            end,
            hash,
            kind,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn len(&self) -> usize {
        self.end - self.start
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

impl CacheWarmup {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(sources: Vec<String>) -> Self {
        Self {
            sources,
            priority: CachePriority::Normal,
            max_warmup_ms: 100,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn with_priority(mut self, p: CachePriority) -> Self {
        self.priority = p;
        self
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn source_count(&self) -> usize {
        self.sources.len()
    }
}

impl SubexprFrequencyMap {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn record(&mut self, hash: u64) {
        *self.counts.entry(hash).or_insert(0) += 1;
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn frequency(&self, hash: u64) -> u32 {
        self.counts.get(&hash).copied().unwrap_or(0)
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn top_k(&self, k: usize) -> Vec<(u64, u32)> {
        let mut pairs: Vec<_> = self.counts.iter().map(|(&h, &c)| (h, c)).collect();
        pairs.sort_by_key(|b| std::cmp::Reverse(b.1));
        pairs.truncate(k);
        pairs
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn total_unique(&self) -> usize {
        self.counts.len()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn total_occurrences(&self) -> u64 {
        self.counts.values().map(|&c| c as u64).sum()
    }
}

impl CacheReport {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(size: usize, hits: u64, misses: u64, evictions: u64, mem: usize) -> Self {
        Self {
            cache_size: size,
            hit_count: hits,
            miss_count: misses,
            eviction_count: evictions,
            memory_bytes: mem,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn hit_rate(&self) -> f64 {
        let total = self.hit_count + self.miss_count;
        if total == 0 {
            0.0
        } else {
            self.hit_count as f64 / total as f64
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn summary(&self) -> String {
        format!(
            "size={} hits={} misses={} evictions={} hit_rate={:.1}% mem={}B",
            self.cache_size,
            self.hit_count,
            self.miss_count,
            self.eviction_count,
            self.hit_rate() * 100.0,
            self.memory_bytes
        )
    }
}

impl<V> LruCache<V> {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            map: std::collections::HashMap::new(),
            order: std::collections::VecDeque::new(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn insert(&mut self, key: u64, value: V) {
        if self.map.len() >= self.capacity {
            if let Some(old) = self.order.pop_front() {
                self.map.remove(&old);
            }
        }
        self.order.push_back(key);
        self.map.insert(key, value);
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn get(&self, key: u64) -> Option<&V> {
        self.map.get(&key)
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn len(&self) -> usize {
        self.map.len()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn contains(&self, key: u64) -> bool {
        self.map.contains_key(&key)
    }
}

impl CachePrewarmer {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(sources: Vec<String>) -> Self {
        Self {
            sources,
            warmup_count: 0,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn prewarm_all(&mut self, cache: &mut ParseResultCache) -> usize {
        let mut warmed = 0;
        for src in &self.sources {
            if cache.lookup(src).is_none() {
                let cs = compute_checksum(src);
                cache.store(src, format!("warmed:{}", cs), 0);
                warmed += 1;
            }
        }
        self.warmup_count += warmed;
        warmed
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn total_warmed(&self) -> usize {
        self.warmup_count
    }
}

impl WindowedCacheMetrics {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(window_size: usize) -> Self {
        Self {
            window_size,
            ..Default::default()
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn record_hit(&mut self) {
        self.window_hits += 1;
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn record_miss(&mut self) {
        self.window_misses += 1;
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn record_eviction(&mut self) {
        self.window_evictions += 1;
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn record_insert(&mut self) {
        self.window_inserts += 1;
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn hit_rate(&self) -> f64 {
        let total = self.window_hits + self.window_misses;
        if total == 0 {
            0.0
        } else {
            self.window_hits as f64 / total as f64
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn reset(&mut self) {
        self.window_hits = 0;
        self.window_misses = 0;
        self.window_evictions = 0;
        self.window_inserts = 0;
    }
}

impl TtlEviction {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(ttl_ticks: u64) -> Self {
        Self { ttl_ticks }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn should_evict(&self, _ac: u64, last_access: u64, now: u64) -> bool {
        now.saturating_sub(last_access) > self.ttl_ticks
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn policy_name(&self) -> &'static str {
        "TTL"
    }
}

impl MacroExpansionCache {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(max_size: usize) -> Self {
        Self {
            entries: std::collections::HashMap::new(),
            max_size,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn lookup(&mut self, macro_hash: u64, arg_hash: u64) -> Option<&MacroExpansionEntry> {
        let key = mix_hashes(macro_hash, arg_hash);
        if let Some(e) = self.entries.get_mut(&key) {
            e.use_count += 1;
            Some(e)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn store(&mut self, entry: MacroExpansionEntry) {
        if self.entries.len() >= self.max_size {
            if let Some((&k, _)) = self.entries.iter().min_by_key(|(_, v)| v.use_count) {
                self.entries.remove(&k);
            }
        }
        let key = mix_hashes(entry.macro_hash, entry.arg_hash);
        self.entries.insert(key, entry);
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn total_stored(&self) -> usize {
        self.entries.len()
    }
}

impl InternedStr {
    /// Return the raw index of this interned string
    #[allow(missing_docs)]
    pub fn idx(self) -> u32 {
        self.0
    }
}

impl CachePressureMonitor {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn record_insert(&mut self, current_size: usize) {
        self.inserts += 1;
        if current_size > self.peak_size {
            self.peak_size = current_size;
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn record_eviction(&mut self) {
        self.evictions += 1;
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn record_lookup(&mut self, hit: bool) {
        self.lookups += 1;
        if hit {
            self.hits += 1;
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn hit_rate(&self) -> f64 {
        if self.lookups == 0 {
            0.0
        } else {
            self.hits as f64 / self.lookups as f64
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn report(&self) -> String {
        format!(
            "hits={} misses={} hit_rate={:.1}% peak={}",
            self.hits,
            self.lookups.saturating_sub(self.hits),
            self.hit_rate() * 100.0,
            self.peak_size
        )
    }
}

impl<K: std::hash::Hash + Eq + Clone, V> WindowCache<K, V> {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(window: usize) -> Self {
        Self {
            map: std::collections::HashMap::new(),
            order: VecDeque::new(),
            window,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn insert(&mut self, key: K, value: V) {
        if self.map.len() >= self.window {
            if let Some(old) = self.order.pop_front() {
                self.map.remove(&old);
            }
        }
        self.order.push_back(key.clone());
        self.map.insert(key, value);
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn len(&self) -> usize {
        self.map.len()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

impl<K: std::hash::Hash + Eq, V> NamespacedCache<K, V> {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            namespaces: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn insert(&mut self, ns: &str, key: K, value: V) {
        self.namespaces
            .entry(ns.to_string())
            .or_default()
            .insert(key, value);
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn get(&self, ns: &str, key: &K) -> Option<&V> {
        self.namespaces.get(ns)?.get(key)
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn invalidate_namespace(&mut self, ns: &str) {
        self.namespaces.remove(ns);
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn namespace_count(&self) -> usize {
        self.namespaces.len()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn total_entries(&self) -> usize {
        self.namespaces.values().map(|m| m.len()).sum()
    }
}

impl ExprPool {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            exprs: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn intern(&mut self, repr: String) -> u64 {
        let hash = fnv1a_hash(repr.as_bytes());
        let entry = self.exprs.entry(hash).or_insert_with(|| (repr, 0));
        entry.1 += 1;
        hash
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn release(&mut self, hash: u64) {
        if let Some(entry) = self.exprs.get_mut(&hash) {
            if entry.1 > 0 {
                entry.1 -= 1;
            }
            if entry.1 == 0 {
                self.exprs.remove(&hash);
            }
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn get(&self, hash: u64) -> Option<&str> {
        self.exprs.get(&hash).map(|(s, _)| s.as_str())
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn total_exprs(&self) -> usize {
        self.exprs.len()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn total_refs(&self) -> usize {
        self.exprs.values().map(|(_, rc)| rc).sum()
    }
}

impl SegmentTable {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
            hashes_by_range: std::collections::BTreeMap::new(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn add(&mut self, seg: ExprSegment) {
        self.hashes_by_range.insert((seg.start, seg.end), seg.hash);
        self.segments.push(seg);
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn invalidate_range(&mut self, start: usize, end: usize) {
        self.segments.retain(|s| s.end <= start || s.start >= end);
        let keys: Vec<_> = self
            .hashes_by_range
            .range((start, 0)..=(end, usize::MAX))
            .map(|(k, _)| *k)
            .collect();
        for k in keys {
            self.hashes_by_range.remove(&k);
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn lookup_hash(&self, start: usize, end: usize) -> Option<u64> {
        self.hashes_by_range.get(&(start, end)).copied()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn count(&self) -> usize {
        self.segments.len()
    }
}

impl MemoTable {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            entries: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn lookup(&self, pos: usize, rule: &str) -> Option<&MemoEntry> {
        self.entries.get(&(pos, rule.to_string()))
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn store(&mut self, pos: usize, rule: impl Into<String>, entry: MemoEntry) {
        self.entries.insert((pos, rule.into()), entry);
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn size(&self) -> usize {
        self.entries.len()
    }
}

impl GlobalExprTable {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            by_repr: std::collections::HashMap::new(),
            by_hash: std::collections::HashMap::new(),
            next_id: 0,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn intern(&mut self, repr: impl Into<String>) -> u64 {
        let r = repr.into();
        if let Some(&id) = self.by_repr.get(&r) {
            return id;
        }
        let id = self.next_id;
        self.next_id += 1;
        self.by_repr.insert(r.clone(), id);
        self.by_hash.insert(id, r);
        id
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn lookup_repr(&self, id: u64) -> Option<&str> {
        self.by_hash.get(&id).map(|s| s.as_str())
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn table_size(&self) -> usize {
        self.by_hash.len()
    }
}

impl StringPool {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            pool: std::collections::HashSet::new(),
            total_saved_bytes: 0,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn intern(&mut self, s: &str) -> String {
        if !self.pool.contains(s) {
            self.pool.insert(s.to_string());
        } else {
            self.total_saved_bytes += s.len();
        }
        s.to_string()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn count(&self) -> usize {
        self.pool.len()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn saved_bytes(&self) -> usize {
        self.total_saved_bytes
    }
}

impl AlphaEqCache {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            known_equal: std::collections::HashSet::new(),
            known_inequal: std::collections::HashSet::new(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn mark_equal(&mut self, a: u64, b: u64) {
        let key = if a <= b { (a, b) } else { (b, a) };
        self.known_equal.insert(key);
        self.known_inequal.remove(&key);
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn mark_inequal(&mut self, a: u64, b: u64) {
        let key = if a <= b { (a, b) } else { (b, a) };
        self.known_inequal.insert(key);
        self.known_equal.remove(&key);
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn query(&self, a: u64, b: u64) -> Option<bool> {
        let key = if a <= b { (a, b) } else { (b, a) };
        if self.known_equal.contains(&key) {
            Some(true)
        } else if self.known_inequal.contains(&key) {
            Some(false)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn stats(&self) -> (usize, usize) {
        (self.known_equal.len(), self.known_inequal.len())
    }
}

impl DeclHash {
    /// Compute a DJB2-style hash of the text bytes
    #[allow(missing_docs)]
    pub fn compute(text: &str) -> Self {
        let mut hash: u64 = 5381;
        for byte in text.bytes() {
            hash = hash.wrapping_mul(33).wrapping_add(byte as u64);
        }
        DeclHash(hash)
    }
    /// Raw hash value
    #[allow(missing_docs)]
    pub fn value(&self) -> u64 {
        self.0
    }
}

impl ExprLocationIndex {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            index: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn record(&mut self, hash: u64, start: usize, end: usize) {
        self.index.entry(hash).or_default().push((start, end));
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn locations(&self, hash: u64) -> &[(usize, usize)] {
        self.index.get(&hash).map(|v| v.as_slice()).unwrap_or(&[])
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn count_occurrences(&self, hash: u64) -> usize {
        self.index.get(&hash).map(|v| v.len()).unwrap_or(0)
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn total_tracked(&self) -> usize {
        self.index.values().map(|v| v.len()).sum()
    }
}

impl CacheHealthReport {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_healthy(&self) -> bool {
        self.estimated_waste_pct < 50.0
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn summary(&self) -> String {
        format!(
            "total={} hot={} warm={} cold={} dead={} waste={:.1}%",
            self.total_entries,
            self.hot_entries,
            self.warm_entries,
            self.cold_entries,
            self.dead_entries,
            self.estimated_waste_pct
        )
    }
}

impl TokenWindow {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(capacity: usize) -> Self {
        Self {
            tokens: std::collections::VecDeque::new(),
            capacity,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn push(&mut self, tok: impl Into<String>) {
        self.tokens.push_back(tok.into());
        if self.tokens.len() > self.capacity {
            self.tokens.pop_front();
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn as_slice(&self) -> Vec<&str> {
        self.tokens.iter().map(|s| s.as_str()).collect()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn contains(&self, tok: &str) -> bool {
        self.tokens.iter().any(|t| t == tok)
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn len(&self) -> usize {
        self.tokens.len()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_full(&self) -> bool {
        self.tokens.len() == self.capacity
    }
}

impl<K: std::hash::Hash + Eq + Clone, V> TwoQueueCache<K, V> {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            clock: 0,
            main: std::collections::HashMap::new(),
            probation: std::collections::VecDeque::new(),
            protected: std::collections::VecDeque::new(),
            probation_cap: capacity,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn insert(&mut self, key: K, value: V) {
        self.clock += 1;
        let entry = AdaptiveCacheEntry::new(value, CachePriority::Normal, self.clock);
        if self.main.len() >= self.capacity {
            if let Some(k) = self.probation.pop_front() {
                self.main.remove(&k);
            } else if let Some(k) = self.protected.pop_front() {
                self.main.remove(&k);
            }
        }
        self.probation.push_back(key.clone());
        if self.probation.len() > self.probation_cap {
            if let Some(old) = self.probation.pop_front() {
                self.main.remove(&old);
            }
        }
        self.main.insert(key, entry);
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn get(&mut self, key: &K) -> Option<&V> {
        self.clock += 1;
        let now = self.clock;
        if let Some(entry) = self.main.get_mut(key) {
            entry.touch(now);
            Some(&entry.value)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn len(&self) -> usize {
        self.main.len()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.main.is_empty()
    }
}

impl InterningStats {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn record_hit(&mut self, str_len: usize) {
        self.total_intern_calls += 1;
        self.bytes_saved += str_len as u64;
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn record_new(&mut self) {
        self.total_intern_calls += 1;
        self.unique_strings += 1;
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn dedup_ratio(&self) -> f64 {
        if self.unique_strings == 0 {
            0.0
        } else {
            self.total_intern_calls as f64 / self.unique_strings as f64
        }
    }
}

impl CacheKeyBuilder {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            hash: 0xcbf29ce484222325,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn with_str(self, s: &str) -> Self {
        Self {
            hash: mix_hashes(self.hash, fnv1a_hash(s.as_bytes())),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn with_u64(self, n: u64) -> Self {
        Self {
            hash: mix_hashes(self.hash, n),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn with_usize(self, n: usize) -> Self {
        self.with_u64(n as u64)
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn build(self) -> u64 {
        self.hash
    }
}

impl NestingDepthTracker {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(max_depth: usize) -> Self {
        Self {
            current_depth: 0,
            max_depth,
            peak_depth: 0,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn enter(&mut self) -> Result<(), &'static str> {
        if self.current_depth >= self.max_depth {
            return Err("max nesting exceeded");
        }
        self.current_depth += 1;
        if self.current_depth > self.peak_depth {
            self.peak_depth = self.current_depth;
        }
        Ok(())
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn exit(&mut self) {
        if self.current_depth > 0 {
            self.current_depth -= 1;
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn depth(&self) -> usize {
        self.current_depth
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn peak(&self) -> usize {
        self.peak_depth
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_safe(&self) -> bool {
        self.current_depth < self.max_depth
    }
}

impl RollingHash {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(window_size: usize) -> Self {
        let base: u64 = 257;
        let modulus: u64 = 1_000_000_007;
        let mut base_pow = 1u64;
        for _ in 0..window_size.saturating_sub(1) {
            base_pow = base_pow.wrapping_mul(base) % modulus;
        }
        Self {
            base,
            modulus,
            current: 0,
            window_size,
            window: VecDeque::new(),
            base_pow,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn push(&mut self, byte: u8) -> u64 {
        self.current = (self.current.wrapping_mul(self.base) + byte as u64) % self.modulus;
        self.window.push_back(byte);
        if self.window.len() > self.window_size {
            let old = self
                .window
                .pop_front()
                .expect("window len > window_size >= 1");
            let rem = self.base_pow.wrapping_mul(old as u64) % self.modulus;
            self.current = (self.current + self.modulus - rem) % self.modulus;
        }
        self.current
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn current_hash(&self) -> u64 {
        self.current
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn window_full(&self) -> bool {
        self.window.len() == self.window_size
    }
}

impl CacheCoverageReport {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn record_cached(&mut self, bytes: usize) {
        self.cached_bytes += bytes;
        self.total_source_bytes += bytes;
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn record_uncached(&mut self, bytes: usize) {
        self.uncached_bytes += bytes;
        self.total_source_bytes += bytes;
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn coverage_pct(&self) -> f64 {
        if self.total_source_bytes == 0 {
            0.0
        } else {
            self.cached_bytes as f64 / self.total_source_bytes as f64 * 100.0
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn summary(&self) -> String {
        format!(
            "coverage={:.1}% cached={}B total={}B",
            self.coverage_pct(),
            self.cached_bytes,
            self.total_source_bytes
        )
    }
}

impl HashSet64 {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            inner: std::collections::HashSet::new(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn insert(&mut self, h: u64) -> bool {
        self.inner.insert(h)
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn contains(&self, h: u64) -> bool {
        self.inner.contains(&h)
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn clear(&mut self) {
        self.inner.clear();
    }
}

impl BumpAllocator {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: vec![0u8; capacity],
            offset: 0,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn alloc_str(&mut self, s: &str) -> Option<usize> {
        let bytes = s.as_bytes();
        if self.offset + bytes.len() > self.buffer.len() {
            return None;
        }
        let pos = self.offset;
        self.buffer[pos..pos + bytes.len()].copy_from_slice(bytes);
        self.offset += bytes.len();
        Some(pos)
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn get_str(&self, pos: usize, len: usize) -> Option<&str> {
        let end = pos + len;
        if end > self.buffer.len() {
            return None;
        }
        std::str::from_utf8(&self.buffer[pos..end]).ok()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn used(&self) -> usize {
        self.offset
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn remaining(&self) -> usize {
        self.buffer.len() - self.offset
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn reset(&mut self) {
        self.offset = 0;
    }
}

impl TypeCheckCache {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(capacity: usize) -> Self {
        Self {
            cache: std::collections::HashMap::new(),
            capacity,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn lookup(&self, hash: u64) -> Option<&TypeCheckResult> {
        self.cache.get(&hash)
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn store(&mut self, result: TypeCheckResult) {
        if self.cache.len() >= self.capacity {
            if let Some(&k) = self.cache.keys().next() {
                self.cache.remove(&k);
            }
        }
        self.cache.insert(result.expr_hash, result);
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn invalidate(&mut self, hash: u64) {
        self.cache.remove(&hash);
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn valid_count(&self) -> usize {
        self.cache.values().filter(|r| r.is_valid).count()
    }
}
