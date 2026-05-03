//! Types for smart build cache invalidation.

use std::collections::HashMap;

/// A content fingerprint produced by one of the supported hash algorithms.
///
/// Wraps a raw 64-bit hash value; equality comparison is the primary use-case.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ContentHash(pub u64);

impl ContentHash {
    /// The raw 64-bit hash value.
    pub fn value(self) -> u64 {
        self.0
    }
}

impl std::fmt::Display for ContentHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:016x}", self.0)
    }
}

/// One cached build unit (a single source file).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CacheEntry {
    /// Source-file path (used as the primary cache key).
    pub path: String,
    /// Hash of the source file's content at the time of the cached build.
    pub content_hash: ContentHash,
    /// Wall-clock time in milliseconds that this build unit took to compile.
    pub build_time_ms: u64,
    /// Paths of files that this unit directly imports / depends upon.
    pub deps: Vec<String>,
    /// Hash of the build output (object file, .olean, etc.).
    pub output_hash: ContentHash,
}

impl CacheEntry {
    /// Construct a new cache entry.
    pub fn new(
        path: impl Into<String>,
        content_hash: ContentHash,
        build_time_ms: u64,
        deps: Vec<String>,
        output_hash: ContentHash,
    ) -> Self {
        Self {
            path: path.into(),
            content_hash,
            build_time_ms,
            deps,
            output_hash,
        }
    }
}

/// The full build cache: a map from source-file path to its cache entry, plus
/// a schema version for on-disk persistence compatibility.
#[derive(Debug, Clone)]
pub struct BuildCache {
    /// All cached entries, keyed by path.
    pub entries: HashMap<String, CacheEntry>,
    /// Schema version (currently 1).
    pub version: u32,
    /// Configuration applied when this cache was created.
    pub(super) config: CacheConfig,
}

/// Why a particular source file must be rebuilt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidationReason {
    /// The file's content hash has changed since the last successful build.
    ContentChanged,
    /// One of the file's declared dependencies has changed.
    DependencyChanged {
        /// Path of the changed dependency that triggered invalidation.
        dep: String,
    },
    /// The cache entry is absent (first build or cache was cleared).
    Missing,
    /// A forced rebuild was requested regardless of content or dependencies.
    ForceRebuild,
}

impl std::fmt::Display for InvalidationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidationReason::ContentChanged => write!(f, "content_changed"),
            InvalidationReason::DependencyChanged { dep } => {
                write!(f, "dependency_changed({})", dep)
            }
            InvalidationReason::Missing => write!(f, "missing"),
            InvalidationReason::ForceRebuild => write!(f, "force_rebuild"),
        }
    }
}

/// Summarises which files need rebuilding after an invalidation check.
#[derive(Debug, Clone)]
pub struct InvalidationResult {
    /// Paths that must be rebuilt, in no particular order.
    pub to_rebuild: Vec<String>,
    /// Per-path reason for the rebuild decision.
    pub reasons: HashMap<String, InvalidationReason>,
    /// Paths whose cached outputs are still valid.
    pub unchanged: Vec<String>,
}

/// Cache configuration governing capacity and hashing behaviour.
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Maximum number of entries stored in memory.
    pub max_entries: usize,
    /// Whether the cache should be serialised to / loaded from disk.
    pub persist: bool,
    /// Which hash algorithm to use when fingerprinting file content.
    pub hash_algorithm: HashAlgorithm,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_entries: 4096,
            persist: false,
            hash_algorithm: HashAlgorithm::Fnv1a,
        }
    }
}

/// Supported hash algorithms for content fingerprinting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashAlgorithm {
    /// FNV-1a 64-bit — fast and well-distributed for short strings.
    Fnv1a,
    /// DJB2 — classic string hash by Daniel J. Bernstein.
    Djb2,
    /// MurmurHash3 finaliser — excellent avalanche, zero-allocation.
    Murmur3,
}

impl std::fmt::Display for HashAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HashAlgorithm::Fnv1a => write!(f, "fnv1a"),
            HashAlgorithm::Djb2 => write!(f, "djb2"),
            HashAlgorithm::Murmur3 => write!(f, "murmur3"),
        }
    }
}
