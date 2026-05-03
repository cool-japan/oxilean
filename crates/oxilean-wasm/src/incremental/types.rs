//! Types for incremental type checking support
#![allow(dead_code)]

use std::collections::HashMap;

#[cfg(feature = "wasm")]
use serde::Serialize;

/// Content hash of a single declaration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "wasm", derive(Serialize))]
pub struct DeclHash(pub u64);

/// Status of a checked declaration
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "wasm", derive(Serialize))]
pub enum CheckStatus {
    /// Declaration passed type checking
    Ok,
    /// Declaration failed type checking with an error message
    Error(String),
    /// Declaration is queued for (re-)checking
    Pending,
}

/// A single entry in the incremental cache
#[derive(Debug, Clone)]
#[cfg_attr(feature = "wasm", derive(Serialize))]
pub struct IncrementalEntry {
    /// Fully-qualified name of the declaration
    pub name: String,
    /// Content hash at the time this entry was cached
    pub hash: DeclHash,
    /// Cache version at which this entry was last checked
    pub checked_at: u64,
    /// Names of declarations this entry depends on
    pub deps: Vec<String>,
    /// Current check status
    pub status: CheckStatus,
}

/// The incremental cache, mapping declaration names to their entries
#[derive(Debug, Clone)]
#[cfg_attr(feature = "wasm", derive(Serialize))]
pub struct IncrementalCache {
    /// All cached entries keyed by declaration name
    pub entries: HashMap<String, IncrementalEntry>,
    /// Monotonically increasing version counter; bumped on every check pass
    pub version: u64,
}

impl IncrementalCache {
    /// Create a new, empty cache
    pub fn new() -> Self {
        IncrementalCache {
            entries: HashMap::new(),
            version: 0,
        }
    }
}

impl Default for IncrementalCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Declaration-level diff between the previous cache state and new source
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "wasm", derive(Serialize))]
pub struct EditDelta {
    /// Declaration names that are new (not in old cache)
    pub added: Vec<String>,
    /// Declaration names that were removed from the source
    pub removed: Vec<String>,
    /// Declaration names whose content hash changed
    pub modified: Vec<String>,
}

/// Diagnostic attached to a specific declaration
#[derive(Debug, Clone)]
#[cfg_attr(feature = "wasm", derive(Serialize))]
pub struct DiagnosticInfo {
    /// Declaration name associated with the diagnostic
    pub name: String,
    /// Human-readable diagnostic message
    pub message: String,
    /// Severity level: 0 = hint, 1 = info, 2 = warning, 3 = error
    pub severity: u8,
}

/// Result returned by an incremental check pass
#[derive(Debug, Clone)]
#[cfg_attr(feature = "wasm", derive(Serialize))]
pub struct IncrementalCheckResult {
    /// Updated cache after the check pass
    pub cache: IncrementalCache,
    /// All diagnostics produced during this pass
    pub diagnostics: Vec<DiagnosticInfo>,
    /// Number of declarations that were actually re-checked
    pub recheck_count: usize,
    /// Number of declarations served from cache without re-checking
    pub cache_hit_count: usize,
}
