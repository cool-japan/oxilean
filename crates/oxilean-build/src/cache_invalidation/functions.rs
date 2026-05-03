//! Functions for smart build cache invalidation.

use std::collections::{HashMap, VecDeque};
use std::io::Read;

use super::types::{
    BuildCache, CacheConfig, CacheEntry, ContentHash, HashAlgorithm, InvalidationReason,
    InvalidationResult,
};
use crate::dep_analysis::{DepGraph, DepKind};

// ── Hash algorithms ──────────────────────────────────────────────────────────

/// Hash `content` using the selected algorithm and return a `ContentHash`.
pub fn hash_content(content: &[u8], algo: HashAlgorithm) -> ContentHash {
    let value = match algo {
        HashAlgorithm::Fnv1a => fnv1a_64(content),
        HashAlgorithm::Djb2 => djb2_64(content),
        HashAlgorithm::Murmur3 => murmur3_64(content),
    };
    ContentHash(value)
}

/// FNV-1a 64-bit hash (Fowler–Noll–Vo).
fn fnv1a_64(data: &[u8]) -> u64 {
    const OFFSET_BASIS: u64 = 14695981039346656037;
    const PRIME: u64 = 1099511628211;
    let mut hash = OFFSET_BASIS;
    for &byte in data {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(PRIME);
    }
    hash
}

/// DJB2 hash adapted to 64-bit arithmetic.
fn djb2_64(data: &[u8]) -> u64 {
    let mut hash: u64 = 5381;
    for &byte in data {
        hash = hash.wrapping_mul(33).wrapping_add(byte as u64);
    }
    hash
}

/// MurmurHash3 64-bit finaliser applied to a 64-bit seed derived from the data.
fn murmur3_64(data: &[u8]) -> u64 {
    // Build a 64-bit seed by treating the input as little-endian u64 chunks.
    let mut h: u64 = data.len() as u64;
    let mut chunks = data.chunks_exact(8);
    for chunk in chunks.by_ref() {
        let word = u64::from_le_bytes(chunk.try_into().unwrap_or([0u8; 8]));
        h ^= word.wrapping_mul(0xff51afd7ed558ccd);
    }
    // Remainder bytes.
    let rem = chunks.remainder();
    if !rem.is_empty() {
        let mut tail = [0u8; 8];
        tail[..rem.len()].copy_from_slice(rem);
        h ^= u64::from_le_bytes(tail);
    }
    // MurmurHash3 finaliser (fmix64).
    h ^= h >> 33;
    h = h.wrapping_mul(0xff51afd7ed558ccd);
    h ^= h >> 33;
    h = h.wrapping_mul(0xc4ceb9fe1a85ec53);
    h ^= h >> 33;
    h
}

/// Read the file at `path`, hash its content with FNV-1a, and return the hash.
/// Returns `None` when the file cannot be read (does not exist, permission denied, etc.).
pub fn hash_file(path: &str) -> Option<ContentHash> {
    hash_file_with_algo(path, HashAlgorithm::Fnv1a)
}

/// Read the file at `path` and hash its content with the given algorithm.
pub fn hash_file_with_algo(path: &str, algo: HashAlgorithm) -> Option<ContentHash> {
    let mut file = std::fs::File::open(path).ok()?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).ok()?;
    Some(hash_content(&buf, algo))
}

// ── BuildCache ───────────────────────────────────────────────────────────────

impl BuildCache {
    /// Construct an empty `BuildCache` with the supplied configuration.
    pub fn new(config: CacheConfig) -> Self {
        Self {
            entries: HashMap::new(),
            version: 1,
            config,
        }
    }

    /// Look up a cached entry by source-file path.
    pub fn lookup(&self, path: &str) -> Option<&CacheEntry> {
        self.entries.get(path)
    }

    /// Insert or replace a cache entry.  When the cache is at capacity the
    /// oldest-inserted entry (arbitrary in a HashMap; we pick the first key
    /// found) is evicted to make room.
    pub fn update(&mut self, entry: CacheEntry) {
        let max = self.config.max_entries;
        if !self.entries.contains_key(&entry.path) && self.entries.len() >= max {
            // Evict one arbitrary entry to stay within the configured limit.
            if let Some(victim) = self.entries.keys().next().cloned() {
                self.entries.remove(&victim);
            }
        }
        self.entries.insert(entry.path.clone(), entry);
    }

    /// Remove the entry for `path`.  Returns `true` when an entry was present.
    pub fn invalidate(&mut self, path: &str) -> bool {
        self.entries.remove(path).is_some()
    }

    /// Return the active hash algorithm from the cache configuration.
    pub fn hash_algorithm(&self) -> HashAlgorithm {
        self.config.hash_algorithm
    }

    /// Number of entries currently stored.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// `true` when the cache contains no entries.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

// ── Invalidation logic ───────────────────────────────────────────────────────

/// Determine which files need rebuilding given a set of `changed_files` and the
/// full dependency graph.
///
/// The algorithm:
/// 1. Any path in `changed_files` is stale (ContentChanged if in cache, Missing otherwise).
/// 2. Any path that *transitively* depends on a stale path is also stale
///    (DependencyChanged).
/// 3. All remaining cached paths are unchanged.
///
/// The `dep_graph` is used **only** to discover reverse-dependency edges (who
/// imports whom).  Files not present in the graph are treated as having no
/// dependants.
pub fn compute_invalidation(
    cache: &BuildCache,
    changed_files: &[String],
    dep_graph: &DepGraph,
) -> InvalidationResult {
    let mut to_rebuild: Vec<String> = Vec::new();
    let mut reasons: HashMap<String, InvalidationReason> = HashMap::new();

    // Build a path → dependants index from the dep_graph edges.
    // dep_graph edge (from → to) means "from imports to", so reversing: "to is needed by from".
    let mut dependants: HashMap<String, Vec<String>> = HashMap::new();
    for edge in &dep_graph.edges {
        if let (Some(from_node), Some(to_node)) =
            (dep_graph.nodes.get(edge.from), dep_graph.nodes.get(edge.to))
        {
            dependants
                .entry(to_node.path.clone())
                .or_default()
                .push(from_node.path.clone());
        }
    }

    // BFS from every changed file through the dependant graph.
    let mut stale: HashMap<String, InvalidationReason> = HashMap::new();
    let mut queue: VecDeque<String> = VecDeque::new();

    for path in changed_files {
        let reason = if cache.lookup(path).is_some() {
            InvalidationReason::ContentChanged
        } else {
            InvalidationReason::Missing
        };
        if stale.insert(path.clone(), reason).is_none() {
            queue.push_back(path.clone());
        }
    }

    while let Some(stale_path) = queue.pop_front() {
        if let Some(deps_of) = dependants.get(&stale_path) {
            for dep in deps_of {
                if !stale.contains_key(dep) {
                    stale.insert(
                        dep.clone(),
                        InvalidationReason::DependencyChanged {
                            dep: stale_path.clone(),
                        },
                    );
                    queue.push_back(dep.clone());
                }
            }
        }
    }

    for (path, reason) in stale {
        to_rebuild.push(path.clone());
        reasons.insert(path, reason);
    }

    // Unchanged = cached entries not in the rebuild set.
    let rebuild_set: std::collections::HashSet<&str> =
        to_rebuild.iter().map(String::as_str).collect();
    let unchanged: Vec<String> = cache
        .entries
        .keys()
        .filter(|p| !rebuild_set.contains(p.as_str()))
        .cloned()
        .collect();

    InvalidationResult {
        to_rebuild,
        reasons,
        unchanged,
    }
}

/// Return the rebuild order (topologically sorted) for the paths in `result`
/// using the dependency graph.
///
/// Files that are not represented in `dep_graph` appear at the beginning in
/// the order they were encountered.
pub fn rebuild_order(result: &InvalidationResult, graph: &DepGraph) -> Vec<String> {
    // Map path → node id (only for paths that appear in the graph).
    let path_to_id: HashMap<&str, usize> = graph
        .nodes
        .iter()
        .map(|n| (n.path.as_str(), n.id))
        .collect();

    let rebuild_set: std::collections::HashSet<&str> =
        result.to_rebuild.iter().map(String::as_str).collect();

    // Collect node ids for rebuild candidates that appear in the graph.
    let mut node_ids: Vec<usize> = result
        .to_rebuild
        .iter()
        .filter_map(|p| path_to_id.get(p.as_str()).copied())
        .collect();

    // Subgraph topological sort using Kahn's algorithm restricted to rebuild nodes.
    let rebuild_id_set: std::collections::HashSet<usize> = node_ids.iter().copied().collect();

    // Build-order semantics: edge (from→to) means "from imports to", i.e., to must be
    // compiled BEFORE from.  For topological ordering of the build we therefore treat the
    // edge direction as (to → from) — "to enables from".
    //
    // In Kahn's algorithm in_degree means "how many prerequisites does this node still have".
    // A node with in_degree == 0 has all prerequisites satisfied and can be built next.
    // So: for each (from, to) edge, `from` cannot start until `to` is done → from's in_degree++.
    let mut in_degree: HashMap<usize, usize> = node_ids.iter().map(|&id| (id, 0)).collect();
    for edge in &graph.edges {
        if rebuild_id_set.contains(&edge.from) && rebuild_id_set.contains(&edge.to) {
            // `from` depends on `to`, so `from` needs one more prerequisite.
            *in_degree.entry(edge.from).or_insert(0) += 1;
        }
    }

    let mut queue: VecDeque<usize> = in_degree
        .iter()
        .filter(|(_, &d)| d == 0)
        .map(|(&id, _)| id)
        .collect();

    // Reverse adjacency for the build-order propagation: when `to` is done, `from` can proceed.
    let mut adj: HashMap<usize, Vec<usize>> = HashMap::new();
    for edge in &graph.edges {
        if rebuild_id_set.contains(&edge.from) && rebuild_id_set.contains(&edge.to) {
            // Completing `to` reduces in_degree of `from`.
            adj.entry(edge.to).or_default().push(edge.from);
        }
    }

    let mut ordered_ids: Vec<usize> = Vec::new();
    while let Some(v) = queue.pop_front() {
        ordered_ids.push(v);
        if let Some(nexts) = adj.get(&v) {
            for &w in nexts {
                let d = in_degree.entry(w).or_insert(0);
                *d = d.saturating_sub(1);
                if *d == 0 {
                    queue.push_back(w);
                }
            }
        }
    }

    // Build the final list: sorted first, then paths not in the graph.
    let id_to_path: HashMap<usize, &str> = graph
        .nodes
        .iter()
        .map(|n| (n.id, n.path.as_str()))
        .collect();

    let mut ordered: Vec<String> = ordered_ids
        .iter()
        .filter_map(|id| id_to_path.get(id).map(|&p| p.to_string()))
        .collect();

    // Append rebuild paths that had no graph node.
    for path in &result.to_rebuild {
        if !rebuild_set.contains(path.as_str()) || !path_to_id.contains_key(path.as_str()) {
            if !ordered.contains(path) {
                ordered.push(path.clone());
            }
        }
    }

    // Append any that were in rebuild_set but missed by the topo sort (e.g. cycle nodes).
    node_ids.retain(|id| !ordered_ids.contains(id));
    for id in &node_ids {
        if let Some(&p) = id_to_path.get(id) {
            let ps = p.to_string();
            if !ordered.contains(&ps) {
                ordered.push(ps);
            }
        }
    }

    ordered
}

/// Remove cache entries whose `build_time_ms` is more than `max_age_ms` before
/// `current_time_ms`.  Returns the number of entries pruned.
///
/// Note: `build_time_ms` in `CacheEntry` tracks compilation duration, not a
/// timestamp.  For time-based pruning we treat the entry's `build_time_ms` as
/// the wall-clock time when it was last built.  Callers should store the epoch
/// millisecond timestamp in `build_time_ms` when creating entries for use with
/// this function.
pub fn prune_stale(cache: &mut BuildCache, max_age_ms: u64, current_time_ms: u64) -> usize {
    let cutoff = current_time_ms.saturating_sub(max_age_ms);
    let before = cache.entries.len();
    cache
        .entries
        .retain(|_, entry| entry.build_time_ms >= cutoff);
    before - cache.entries.len()
}

// ── Serialisation ────────────────────────────────────────────────────────────

/// Serialise the cache to a human-readable text format.
///
/// Format (one block per entry):
/// ```text
/// version=<n>
/// entry
///   path=<path>
///   content_hash=<hex>
///   build_time_ms=<u64>
///   output_hash=<hex>
///   dep=<dep_path>
///   dep=<dep_path>
/// end
/// ```
pub fn serialize_cache(cache: &BuildCache) -> String {
    let mut out = String::new();
    out.push_str(&format!("version={}\n", cache.version));
    for entry in cache.entries.values() {
        out.push_str("entry\n");
        out.push_str(&format!("  path={}\n", entry.path));
        out.push_str(&format!("  content_hash={:016x}\n", entry.content_hash.0));
        out.push_str(&format!("  build_time_ms={}\n", entry.build_time_ms));
        out.push_str(&format!("  output_hash={:016x}\n", entry.output_hash.0));
        for dep in &entry.deps {
            out.push_str(&format!("  dep={}\n", dep));
        }
        out.push_str("end\n");
    }
    out
}

/// Deserialise a cache previously produced by [`serialize_cache`].
/// Returns `Err` with a description when the input is malformed.
pub fn deserialize_cache(s: &str) -> Result<BuildCache, String> {
    let mut version: Option<u32> = None;
    let mut entries: Vec<CacheEntry> = Vec::new();

    let mut in_entry = false;
    let mut path: Option<String> = None;
    let mut content_hash: Option<ContentHash> = None;
    let mut build_time_ms: Option<u64> = None;
    let mut output_hash: Option<ContentHash> = None;
    let mut deps: Vec<String> = Vec::new();

    for (lineno, raw) in s.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }

        if line == "entry" {
            if in_entry {
                return Err(format!("line {}: nested 'entry' block", lineno + 1));
            }
            in_entry = true;
            path = None;
            content_hash = None;
            build_time_ms = None;
            output_hash = None;
            deps.clear();
            continue;
        }

        if line == "end" {
            if !in_entry {
                return Err(format!("line {}: 'end' without 'entry'", lineno + 1));
            }
            let entry = CacheEntry {
                path: path
                    .take()
                    .ok_or_else(|| format!("line {}: entry missing 'path'", lineno + 1))?,
                content_hash: content_hash
                    .take()
                    .ok_or_else(|| format!("line {}: entry missing 'content_hash'", lineno + 1))?,
                build_time_ms: build_time_ms
                    .take()
                    .ok_or_else(|| format!("line {}: entry missing 'build_time_ms'", lineno + 1))?,
                deps: std::mem::take(&mut deps),
                output_hash: output_hash
                    .take()
                    .ok_or_else(|| format!("line {}: entry missing 'output_hash'", lineno + 1))?,
            };
            entries.push(entry);
            in_entry = false;
            continue;
        }

        // Key=value lines.
        let (key, value) = line
            .split_once('=')
            .ok_or_else(|| format!("line {}: expected 'key=value', got '{}'", lineno + 1, line))?;

        if in_entry {
            match key {
                "path" => path = Some(value.to_string()),
                "content_hash" => {
                    let raw_hash = u64::from_str_radix(value, 16).map_err(|e| {
                        format!("line {}: bad content_hash '{}': {}", lineno + 1, value, e)
                    })?;
                    content_hash = Some(ContentHash(raw_hash));
                }
                "build_time_ms" => {
                    let ms = value.parse::<u64>().map_err(|e| {
                        format!("line {}: bad build_time_ms '{}': {}", lineno + 1, value, e)
                    })?;
                    build_time_ms = Some(ms);
                }
                "output_hash" => {
                    let raw_hash = u64::from_str_radix(value, 16).map_err(|e| {
                        format!("line {}: bad output_hash '{}': {}", lineno + 1, value, e)
                    })?;
                    output_hash = Some(ContentHash(raw_hash));
                }
                "dep" => deps.push(value.to_string()),
                other => {
                    return Err(format!(
                        "line {}: unknown field '{}' inside entry",
                        lineno + 1,
                        other
                    ));
                }
            }
        } else {
            match key {
                "version" => {
                    version = Some(value.parse::<u32>().map_err(|e| {
                        format!("line {}: bad version '{}': {}", lineno + 1, value, e)
                    })?);
                }
                other => {
                    return Err(format!(
                        "line {}: unexpected top-level key '{}'",
                        lineno + 1,
                        other
                    ));
                }
            }
        }
    }

    if in_entry {
        return Err("Unexpected end of input: unclosed 'entry' block".to_string());
    }

    let v = version.ok_or("Missing 'version' line")?;
    let config = CacheConfig {
        max_entries: usize::MAX,
        persist: true,
        hash_algorithm: HashAlgorithm::Fnv1a,
    };
    let mut cache = BuildCache::new(config);
    cache.version = v;
    for entry in entries {
        cache.entries.insert(entry.path.clone(), entry);
    }
    Ok(cache)
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    // ── Helpers ──────────────────────────────────────────────────────────────

    fn default_cache() -> BuildCache {
        BuildCache::new(CacheConfig::default())
    }

    fn make_entry(path: &str, hash_val: u64, deps: Vec<&str>) -> CacheEntry {
        CacheEntry::new(
            path,
            ContentHash(hash_val),
            1000,
            deps.into_iter().map(str::to_string).collect(),
            ContentHash(hash_val ^ 0xffff),
        )
    }

    fn write_temp_file(name: &str, content: &[u8]) -> std::path::PathBuf {
        let mut p = std::env::temp_dir();
        p.push(name);
        let mut f = std::fs::File::create(&p).expect("create temp file");
        f.write_all(content).expect("write temp file");
        p
    }

    // ── hash_content ─────────────────────────────────────────────────────

    #[test]
    fn test_hash_content_fnv1a_deterministic() {
        let h1 = hash_content(b"hello world", HashAlgorithm::Fnv1a);
        let h2 = hash_content(b"hello world", HashAlgorithm::Fnv1a);
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_hash_content_djb2_deterministic() {
        let h1 = hash_content(b"lean4rules", HashAlgorithm::Djb2);
        let h2 = hash_content(b"lean4rules", HashAlgorithm::Djb2);
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_hash_content_murmur3_deterministic() {
        let h1 = hash_content(b"oxilean", HashAlgorithm::Murmur3);
        let h2 = hash_content(b"oxilean", HashAlgorithm::Murmur3);
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_hash_content_different_algos_differ() {
        let content = b"same content for all";
        let h_fnv = hash_content(content, HashAlgorithm::Fnv1a);
        let h_djb = hash_content(content, HashAlgorithm::Djb2);
        let h_mur = hash_content(content, HashAlgorithm::Murmur3);
        // Extremely unlikely to collide across different algorithms.
        assert_ne!(h_fnv, h_djb);
        assert_ne!(h_fnv, h_mur);
    }

    #[test]
    fn test_hash_content_empty_is_stable() {
        let h = hash_content(b"", HashAlgorithm::Fnv1a);
        let h2 = hash_content(b"", HashAlgorithm::Fnv1a);
        assert_eq!(h, h2);
    }

    #[test]
    fn test_hash_content_different_content_differs() {
        let h1 = hash_content(b"aaa", HashAlgorithm::Fnv1a);
        let h2 = hash_content(b"bbb", HashAlgorithm::Fnv1a);
        assert_ne!(h1, h2);
    }

    // ── hash_file ────────────────────────────────────────────────────────

    #[test]
    fn test_hash_file_existing() {
        let p = write_temp_file("oxilean_cache_test_hash.lean", b"-- hello lean\n");
        let hash = hash_file(p.to_str().expect("valid utf8 path"));
        assert!(hash.is_some());
    }

    #[test]
    fn test_hash_file_missing_returns_none() {
        let result = hash_file("/tmp/oxilean_nonexistent_file_xyz_12345.lean");
        assert!(result.is_none());
    }

    #[test]
    fn test_hash_file_content_matches_direct_hash() {
        let content = b"theorem foo : True := trivial";
        let p = write_temp_file("oxilean_cache_test_direct.lean", content);
        let from_file = hash_file(p.to_str().expect("utf8")).expect("hash_file");
        let direct = hash_content(content, HashAlgorithm::Fnv1a);
        assert_eq!(from_file, direct);
    }

    // ── BuildCache CRUD ──────────────────────────────────────────────────

    #[test]
    fn test_cache_new_is_empty() {
        let c = default_cache();
        assert!(c.is_empty());
        assert_eq!(c.len(), 0);
    }

    #[test]
    fn test_cache_update_and_lookup() {
        let mut c = default_cache();
        c.update(make_entry("foo.lean", 42, vec![]));
        let entry = c.lookup("foo.lean");
        assert!(entry.is_some());
        assert_eq!(entry.expect("entry").content_hash, ContentHash(42));
    }

    #[test]
    fn test_cache_lookup_missing_returns_none() {
        let c = default_cache();
        assert!(c.lookup("missing.lean").is_none());
    }

    #[test]
    fn test_cache_invalidate_removes_entry() {
        let mut c = default_cache();
        c.update(make_entry("bar.lean", 7, vec![]));
        assert!(c.invalidate("bar.lean"));
        assert!(c.lookup("bar.lean").is_none());
    }

    #[test]
    fn test_cache_invalidate_missing_returns_false() {
        let mut c = default_cache();
        assert!(!c.invalidate("ghost.lean"));
    }

    #[test]
    fn test_cache_update_replaces_existing() {
        let mut c = default_cache();
        c.update(make_entry("x.lean", 1, vec![]));
        c.update(make_entry("x.lean", 99, vec![]));
        let entry = c.lookup("x.lean").expect("entry");
        assert_eq!(entry.content_hash, ContentHash(99));
        assert_eq!(c.len(), 1); // Still one entry.
    }

    #[test]
    fn test_cache_max_entries_eviction() {
        let config = CacheConfig {
            max_entries: 3,
            persist: false,
            hash_algorithm: HashAlgorithm::Fnv1a,
        };
        let mut c = BuildCache::new(config);
        for i in 0u64..5 {
            c.update(make_entry(&format!("{}.lean", i), i, vec![]));
        }
        // Cache must not exceed max_entries.
        assert!(c.len() <= 3);
    }

    // ── compute_invalidation ─────────────────────────────────────────────

    #[test]
    fn test_invalidation_missing_file() {
        let c = default_cache();
        let graph = DepGraph::new();
        let changed = vec!["/src/Foo.lean".to_string()];
        let result = compute_invalidation(&c, &changed, &graph);
        assert!(result.to_rebuild.contains(&"/src/Foo.lean".to_string()));
        assert!(matches!(
            result.reasons["/src/Foo.lean"],
            InvalidationReason::Missing
        ));
    }

    #[test]
    fn test_invalidation_content_changed() {
        let mut c = default_cache();
        c.update(make_entry("/src/A.lean", 1, vec![]));
        let graph = DepGraph::new();
        let result = compute_invalidation(&c, &["/src/A.lean".to_string()], &graph);
        assert!(matches!(
            result.reasons["/src/A.lean"],
            InvalidationReason::ContentChanged
        ));
    }

    #[test]
    fn test_invalidation_unchanged_files_excluded() {
        let mut c = default_cache();
        c.update(make_entry("/src/A.lean", 1, vec![]));
        c.update(make_entry("/src/B.lean", 2, vec![]));
        let graph = DepGraph::new();
        let result = compute_invalidation(&c, &["/src/A.lean".to_string()], &graph);
        assert!(result.unchanged.contains(&"/src/B.lean".to_string()));
        assert!(!result.to_rebuild.contains(&"/src/B.lean".to_string()));
    }

    #[test]
    fn test_invalidation_propagates_through_dep_graph() {
        // B imports A → changing A should mark both A and B for rebuild.
        let mut c = default_cache();
        c.update(make_entry("/src/A.lean", 1, vec![]));
        c.update(make_entry("/src/B.lean", 2, vec!["/src/A.lean"]));

        let mut graph = DepGraph::new();
        let b_id = graph.add_node("B", "/src/B.lean", 0, 0);
        let a_id = graph.add_node("A", "/src/A.lean", 0, 0);
        graph.add_edge(b_id, a_id, DepKind::Import); // B imports A

        let result = compute_invalidation(&c, &["/src/A.lean".to_string()], &graph);
        let rebuild_set: std::collections::HashSet<&str> =
            result.to_rebuild.iter().map(String::as_str).collect();
        assert!(rebuild_set.contains("/src/A.lean"));
        assert!(rebuild_set.contains("/src/B.lean"));
        assert!(matches!(
            &result.reasons["/src/B.lean"],
            InvalidationReason::DependencyChanged { dep } if dep == "/src/A.lean"
        ));
    }

    #[test]
    fn test_invalidation_empty_changed_list() {
        let mut c = default_cache();
        c.update(make_entry("/src/A.lean", 1, vec![]));
        let graph = DepGraph::new();
        let result = compute_invalidation(&c, &[], &graph);
        assert!(result.to_rebuild.is_empty());
        assert_eq!(result.unchanged.len(), 1);
    }

    // ── rebuild_order ────────────────────────────────────────────────────

    #[test]
    fn test_rebuild_order_respects_dependency_direction() {
        // B imports A → A must be rebuilt before B.
        let mut graph = DepGraph::new();
        let b = graph.add_node("B", "/src/B.lean", 0, 0);
        let a = graph.add_node("A", "/src/A.lean", 0, 0);
        graph.add_edge(b, a, DepKind::Import);

        let result = InvalidationResult {
            to_rebuild: vec!["/src/B.lean".to_string(), "/src/A.lean".to_string()],
            reasons: HashMap::new(),
            unchanged: vec![],
        };
        let order = rebuild_order(&result, &graph);
        let pos_a = order.iter().position(|p| p == "/src/A.lean");
        let pos_b = order.iter().position(|p| p == "/src/B.lean");
        assert!(pos_a.is_some() && pos_b.is_some());
        assert!(pos_a < pos_b, "A must come before B (B depends on A)");
    }

    #[test]
    fn test_rebuild_order_contains_all_rebuild_entries() {
        let graph = DepGraph::new();
        let result = InvalidationResult {
            to_rebuild: vec!["/a.lean".to_string(), "/b.lean".to_string()],
            reasons: HashMap::new(),
            unchanged: vec![],
        };
        let order = rebuild_order(&result, &graph);
        assert!(order.contains(&"/a.lean".to_string()));
        assert!(order.contains(&"/b.lean".to_string()));
    }

    // ── prune_stale ──────────────────────────────────────────────────────

    #[test]
    fn test_prune_stale_removes_old_entries() {
        let mut c = default_cache();
        // Entry with build_time_ms = 100 (treat as timestamp).
        let mut e = make_entry("/old.lean", 1, vec![]);
        e.build_time_ms = 100;
        c.update(e);
        // Entry with build_time_ms = 9000.
        let mut e2 = make_entry("/new.lean", 2, vec![]);
        e2.build_time_ms = 9000;
        c.update(e2);

        // current_time = 10000, max_age = 5000 → cutoff = 5000 → 100 < 5000 → old is stale.
        let pruned = prune_stale(&mut c, 5000, 10000);
        assert_eq!(pruned, 1);
        assert!(c.lookup("/old.lean").is_none());
        assert!(c.lookup("/new.lean").is_some());
    }

    #[test]
    fn test_prune_stale_empty_cache() {
        let mut c = default_cache();
        let pruned = prune_stale(&mut c, 1000, 5000);
        assert_eq!(pruned, 0);
    }

    #[test]
    fn test_prune_stale_all_fresh() {
        let mut c = default_cache();
        let mut e = make_entry("/recent.lean", 1, vec![]);
        e.build_time_ms = 9000;
        c.update(e);
        let pruned = prune_stale(&mut c, 5000, 10000);
        assert_eq!(pruned, 0);
        assert!(c.lookup("/recent.lean").is_some());
    }

    // ── serialize / deserialize ──────────────────────────────────────────

    #[test]
    fn test_serialize_deserialize_roundtrip() {
        let mut c = default_cache();
        c.update(make_entry("/src/A.lean", 0xdeadbeef, vec!["/src/B.lean"]));
        c.update(make_entry("/src/B.lean", 0xcafebabe, vec![]));

        let text = serialize_cache(&c);
        let c2 = deserialize_cache(&text).expect("deserialize");
        assert_eq!(c2.version, c.version);
        assert_eq!(c2.len(), 2);
        let a = c2.lookup("/src/A.lean").expect("A entry");
        assert_eq!(a.content_hash, ContentHash(0xdeadbeef));
        assert_eq!(a.deps, vec!["/src/B.lean".to_string()]);
    }

    #[test]
    fn test_serialize_empty_cache() {
        let c = default_cache();
        let text = serialize_cache(&c);
        assert!(text.contains("version=1"));
        assert!(!text.contains("entry"));
    }

    #[test]
    fn test_deserialize_bad_version_returns_err() {
        let bad = "version=notanumber\n";
        assert!(deserialize_cache(bad).is_err());
    }

    #[test]
    fn test_deserialize_unclosed_entry_returns_err() {
        let bad = "version=1\nentry\n  path=/foo.lean\n";
        assert!(deserialize_cache(bad).is_err());
    }

    #[test]
    fn test_deserialize_missing_path_returns_err() {
        let bad = "version=1\nentry\n  content_hash=0000000000000000\n  build_time_ms=100\n  output_hash=0000000000000001\nend\n";
        assert!(deserialize_cache(bad).is_err());
    }

    #[test]
    fn test_serialize_to_temp_file_and_read_back() {
        let mut c = default_cache();
        c.update(make_entry("/src/Kernel.lean", 0xabcd1234, vec![]));
        let text = serialize_cache(&c);

        let mut p = std::env::temp_dir();
        p.push("oxilean_cache_test_serial.txt");
        {
            let mut f = std::fs::File::create(&p).expect("create");
            f.write_all(text.as_bytes()).expect("write");
        }

        let mut read_back = String::new();
        {
            let mut f = std::fs::File::open(&p).expect("open");
            f.read_to_string(&mut read_back).expect("read");
        }

        let c2 = deserialize_cache(&read_back).expect("deserialize from file");
        assert_eq!(
            c2.lookup("/src/Kernel.lean").expect("entry").content_hash,
            ContentHash(0xabcd1234)
        );
    }

    // ── ContentHash display ───────────────────────────────────────────────

    #[test]
    fn test_content_hash_display_hex() {
        let h = ContentHash(0xdeadbeefcafebabe);
        assert_eq!(h.to_string(), "deadbeefcafebabe");
    }

    #[test]
    fn test_content_hash_value() {
        let h = ContentHash(12345);
        assert_eq!(h.value(), 12345);
    }

    // ── InvalidationReason display ────────────────────────────────────────

    #[test]
    fn test_invalidation_reason_display() {
        assert_eq!(
            InvalidationReason::ContentChanged.to_string(),
            "content_changed"
        );
        assert_eq!(InvalidationReason::Missing.to_string(), "missing");
        assert_eq!(
            InvalidationReason::ForceRebuild.to_string(),
            "force_rebuild"
        );
        let dep_reason = InvalidationReason::DependencyChanged {
            dep: "foo.lean".to_string(),
        };
        assert!(dep_reason.to_string().contains("foo.lean"));
    }
}
