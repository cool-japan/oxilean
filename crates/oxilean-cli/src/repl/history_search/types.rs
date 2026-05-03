//! Types for enhanced REPL history with search.

use std::path::PathBuf;

/// A single REPL history entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoryEntry {
    /// The command that was entered.
    pub command: String,
    /// Unix timestamp (seconds since epoch) when the command was entered.
    pub timestamp: u64,
    /// Session identifier.
    pub session_id: u32,
}

impl HistoryEntry {
    /// Create a new history entry.
    pub fn new(command: impl Into<String>, timestamp: u64, session_id: u32) -> Self {
        Self {
            command: command.into(),
            timestamp,
            session_id,
        }
    }
}

/// Query parameters for searching history.
#[derive(Debug, Clone)]
pub struct HistorySearchQuery {
    /// The pattern to search for.
    pub pattern: String,
    /// Whether the search is case-sensitive.
    pub case_sensitive: bool,
    /// Whether to treat `pattern` as a regex (currently substring match when false).
    pub regex: bool,
    /// Maximum number of results to return.
    pub limit: usize,
}

impl HistorySearchQuery {
    /// Create a new search query.
    pub fn new(pattern: impl Into<String>) -> Self {
        Self {
            pattern: pattern.into(),
            case_sensitive: false,
            regex: false,
            limit: 50,
        }
    }

    /// Set case-sensitivity.
    pub fn case_sensitive(mut self, v: bool) -> Self {
        self.case_sensitive = v;
        self
    }

    /// Enable regex matching.
    pub fn regex(mut self, v: bool) -> Self {
        self.regex = v;
        self
    }

    /// Set result limit.
    pub fn limit(mut self, n: usize) -> Self {
        self.limit = n;
        self
    }
}

/// The result of a history search.
#[derive(Debug, Clone)]
pub struct HistorySearchResult {
    /// Matching entries (capped at `query.limit`).
    pub entries: Vec<HistoryEntry>,
    /// Total number of matches found (before the limit).
    pub total_matches: usize,
}

impl HistorySearchResult {
    /// Create an empty result.
    pub fn empty() -> Self {
        Self {
            entries: Vec::new(),
            total_matches: 0,
        }
    }
}

/// In-memory store of history entries, backed optionally by a file.
pub struct HistoryStore {
    /// All entries, oldest first.
    pub(super) entries: Vec<HistoryEntry>,
    /// Maximum number of entries to retain.
    pub(super) max_size: usize,
    /// Current session identifier.
    pub(super) session_id: u32,
    /// Configuration used to create this store.
    pub(super) config: HistoryConfig,
}

/// Configuration for a `HistoryStore`.
#[derive(Debug, Clone)]
pub struct HistoryConfig {
    /// Maximum number of entries stored in memory.
    pub max_entries: usize,
    /// Path to persist history on disk (CSV format).
    pub persist_file: Option<PathBuf>,
    /// Whether consecutive duplicate commands are discarded.
    pub deduplicate: bool,
}

impl HistoryConfig {
    /// Create a default configuration (1 000 entries, no file, dedup enabled).
    pub fn default_config() -> Self {
        Self {
            max_entries: 1_000,
            persist_file: None,
            deduplicate: true,
        }
    }

    /// Set the maximum number of entries.
    pub fn max_entries(mut self, n: usize) -> Self {
        self.max_entries = n;
        self
    }

    /// Set the persistence file path.
    pub fn persist_file(mut self, path: impl Into<PathBuf>) -> Self {
        self.persist_file = Some(path.into());
        self
    }

    /// Toggle consecutive-duplicate filtering.
    pub fn deduplicate(mut self, v: bool) -> Self {
        self.deduplicate = v;
        self
    }
}
