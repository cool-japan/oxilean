//! Functions for `HistoryStore` and history search utilities.

use super::types::{
    HistoryConfig, HistoryEntry, HistorySearchQuery, HistorySearchResult, HistoryStore,
};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

// ── helpers ─────────────────────────────────────────────────────────────────

/// Return the current Unix timestamp in seconds, falling back to 0 on error.
fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// Escape a CSV field by wrapping in quotes if it contains comma, quote, or newline.
fn csv_escape(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        let escaped = s.replace('"', "\"\"");
        format!("\"{}\"", escaped)
    } else {
        s.to_string()
    }
}

/// Parse a single CSV field that may be quoted.
fn csv_unescape(s: &str) -> String {
    let s = s.trim();
    if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
        s[1..s.len() - 1].replace("\"\"", "\"")
    } else {
        s.to_string()
    }
}

/// Split a CSV line into fields, respecting quoted values.
fn csv_split(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '"' if in_quotes => {
                if chars.peek() == Some(&'"') {
                    chars.next();
                    current.push('"');
                } else {
                    in_quotes = false;
                }
            }
            '"' => {
                in_quotes = true;
            }
            ',' if !in_quotes => {
                fields.push(current.clone());
                current.clear();
            }
            other => {
                current.push(other);
            }
        }
    }
    fields.push(current);
    fields
}

// ── HistoryStore impl ────────────────────────────────────────────────────────

impl HistoryStore {
    /// Create a new `HistoryStore` from the given configuration.
    pub fn new(config: HistoryConfig) -> Self {
        // Derive a pseudo-random session id from the current timestamp.
        let session_id = (now_secs() & 0xFFFF_FFFF) as u32;
        Self {
            entries: Vec::new(),
            max_size: config.max_entries,
            session_id,
            config,
        }
    }

    /// Create a store with a specific session_id (useful in tests).
    pub fn with_session_id(config: HistoryConfig, session_id: u32) -> Self {
        Self {
            entries: Vec::new(),
            max_size: config.max_entries,
            session_id,
            config,
        }
    }

    /// Return the current session id.
    pub fn session_id(&self) -> u32 {
        self.session_id
    }

    /// Return the total number of stored entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Return `true` when the store is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Add a command to the history.
    ///
    /// If `config.deduplicate` is true, the command is silently dropped when it
    /// is identical to the most-recent entry.  Entries beyond `max_size` are
    /// evicted from the front (oldest first).
    pub fn push(&mut self, cmd: &str) {
        if cmd.trim().is_empty() {
            return;
        }

        // Consecutive-duplicate suppression.
        if self.config.deduplicate {
            if let Some(last) = self.entries.last() {
                if last.command == cmd {
                    return;
                }
            }
        }

        let entry = HistoryEntry::new(cmd, now_secs(), self.session_id);
        self.entries.push(entry);

        // Evict oldest entries when the store overflows.
        while self.entries.len() > self.max_size {
            self.entries.remove(0);
        }
    }

    /// Search history entries according to the given query.
    pub fn search(&self, query: &HistorySearchQuery) -> HistorySearchResult {
        if query.pattern.is_empty() {
            // Empty pattern: return most-recent entries up to `limit`.
            let entries = self
                .entries
                .iter()
                .rev()
                .take(query.limit)
                .cloned()
                .collect::<Vec<_>>();
            let total = self.entries.len();
            return HistorySearchResult {
                entries,
                total_matches: total,
            };
        }

        let matching_indices =
            search_with_pattern(&self.entries, &query.pattern, query.case_sensitive);

        let total_matches = matching_indices.len();
        let entries = matching_indices
            .iter()
            .rev()
            .take(query.limit)
            .map(|&idx| self.entries[idx].clone())
            .collect();

        HistorySearchResult {
            entries,
            total_matches,
        }
    }

    /// Return the last `n` entries, newest first.
    pub fn last_n(&self, n: usize) -> Vec<&HistoryEntry> {
        self.entries.iter().rev().take(n).collect()
    }

    /// Return the top `n` commands by occurrence count, descending.
    ///
    /// Each element is `(command_string, count)`.
    pub fn most_frequent(&self, n: usize) -> Vec<(String, usize)> {
        use std::collections::HashMap;

        let mut counts: HashMap<&str, usize> = HashMap::new();
        for entry in &self.entries {
            *counts.entry(entry.command.as_str()).or_insert(0) += 1;
        }

        let mut pairs: Vec<(String, usize)> = counts
            .into_iter()
            .map(|(cmd, cnt)| (cmd.to_string(), cnt))
            .collect();

        // Sort descending by count, then alphabetically for stability.
        pairs.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        pairs.truncate(n);
        pairs
    }

    /// Persist the store to a CSV file.
    ///
    /// Format: `timestamp,session_id,command`
    pub fn save(&self, path: &Path) -> Result<(), String> {
        let mut lines = Vec::with_capacity(self.entries.len() + 1);
        lines.push("timestamp,session_id,command".to_string());
        for entry in &self.entries {
            lines.push(format!(
                "{},{},{}",
                entry.timestamp,
                entry.session_id,
                csv_escape(&entry.command)
            ));
        }
        let content = lines.join("\n") + "\n";
        std::fs::write(path, content)
            .map_err(|e| format!("Failed to write history file '{}': {}", path.display(), e))
    }

    /// Load a store from a CSV file created by `save`.
    pub fn load(path: &Path) -> Result<Self, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read history file '{}': {}", path.display(), e))?;

        let mut entries = Vec::new();
        let mut max_session_id = 0u32;

        for (line_no, line) in content.lines().enumerate() {
            // Skip header.
            if line_no == 0 && line.starts_with("timestamp") {
                continue;
            }
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let fields = csv_split(line);
            if fields.len() < 3 {
                continue; // malformed — skip silently
            }
            let timestamp: u64 = fields[0]
                .trim()
                .parse()
                .map_err(|_| format!("Bad timestamp on line {}", line_no + 1))?;
            let session_id: u32 = fields[1]
                .trim()
                .parse()
                .map_err(|_| format!("Bad session_id on line {}", line_no + 1))?;
            // Command may contain commas so join remaining fields.
            let command = if fields.len() == 3 {
                csv_unescape(&fields[2])
            } else {
                // Re-join excess fields (command contained an unquoted comma — should
                // not happen with proper quoting, but be defensive).
                fields[2..].join(",")
            };
            if command.is_empty() {
                continue;
            }
            max_session_id = max_session_id.max(session_id);
            entries.push(HistoryEntry {
                command,
                timestamp,
                session_id,
            });
        }

        let config = HistoryConfig::default_config();
        let max_size = config.max_entries;
        Ok(Self {
            entries,
            max_size,
            session_id: max_session_id,
            config,
        })
    }
}

/// Return the indices (in `entries`) of all entries whose `command` matches
/// `pattern`.  The search is a substring match; set `case_sensitive` to
/// control case folding.  Returns indices in ascending (oldest-first) order.
pub fn search_with_pattern(
    entries: &[HistoryEntry],
    pattern: &str,
    case_sensitive: bool,
) -> Vec<usize> {
    if pattern.is_empty() {
        return (0..entries.len()).collect();
    }

    let needle_lower;
    let needle: &str = if case_sensitive {
        pattern
    } else {
        needle_lower = pattern.to_lowercase();
        &needle_lower
    };

    entries
        .iter()
        .enumerate()
        .filter_map(|(idx, entry)| {
            let haystack_lower;
            let haystack: &str = if case_sensitive {
                &entry.command
            } else {
                haystack_lower = entry.command.to_lowercase();
                &haystack_lower
            };
            if haystack.contains(needle) {
                Some(idx)
            } else {
                None
            }
        })
        .collect()
}

// ── tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::temp_dir;

    fn make_store() -> HistoryStore {
        let cfg = HistoryConfig::default_config()
            .max_entries(100)
            .deduplicate(false);
        HistoryStore::with_session_id(cfg, 1)
    }

    fn make_store_dedup() -> HistoryStore {
        let cfg = HistoryConfig::default_config()
            .max_entries(100)
            .deduplicate(true);
        HistoryStore::with_session_id(cfg, 2)
    }

    // ── push / basic ────────────────────────────────────────────────────────

    #[test]
    fn test_push_adds_entry() {
        let mut store = make_store();
        store.push("hello");
        assert_eq!(store.len(), 1);
        assert_eq!(store.entries[0].command, "hello");
    }

    #[test]
    fn test_push_ignores_whitespace_only() {
        let mut store = make_store();
        store.push("   ");
        store.push("\t");
        assert_eq!(store.len(), 0);
    }

    #[test]
    fn test_push_multiple() {
        let mut store = make_store();
        for cmd in ["a", "b", "c"] {
            store.push(cmd);
        }
        assert_eq!(store.len(), 3);
    }

    #[test]
    fn test_push_dedup_consecutive() {
        let mut store = make_store_dedup();
        store.push("repeat");
        store.push("repeat");
        store.push("repeat");
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn test_push_dedup_non_consecutive_allowed() {
        let mut store = make_store_dedup();
        store.push("a");
        store.push("b");
        store.push("a");
        assert_eq!(store.len(), 3);
    }

    #[test]
    fn test_max_size_eviction() {
        let cfg = HistoryConfig::default_config()
            .max_entries(3)
            .deduplicate(false);
        let mut store = HistoryStore::with_session_id(cfg, 1);
        for i in 0..10u32 {
            store.push(&format!("cmd{}", i));
        }
        assert_eq!(store.len(), 3);
        // Oldest should be evicted; newest retained.
        assert_eq!(store.entries[2].command, "cmd9");
    }

    // ── last_n ──────────────────────────────────────────────────────────────

    #[test]
    fn test_last_n_returns_newest_first() {
        let mut store = make_store();
        store.push("first");
        store.push("second");
        store.push("third");
        let last = store.last_n(2);
        assert_eq!(last[0].command, "third");
        assert_eq!(last[1].command, "second");
    }

    #[test]
    fn test_last_n_more_than_len() {
        let mut store = make_store();
        store.push("only");
        let last = store.last_n(100);
        assert_eq!(last.len(), 1);
    }

    #[test]
    fn test_last_n_empty_store() {
        let store = make_store();
        assert!(store.last_n(5).is_empty());
    }

    // ── most_frequent ────────────────────────────────────────────────────────

    #[test]
    fn test_most_frequent_basic() {
        let mut store = make_store();
        for _ in 0..5 {
            store.push("popular");
        }
        for _ in 0..2 {
            store.push("less");
        }
        store.push("rare");
        let freq = store.most_frequent(2);
        assert_eq!(freq[0], ("popular".to_string(), 5));
        assert_eq!(freq[1], ("less".to_string(), 2));
    }

    #[test]
    fn test_most_frequent_limit() {
        let mut store = make_store();
        for cmd in ["a", "b", "c", "d", "e"] {
            store.push(cmd);
        }
        let freq = store.most_frequent(3);
        assert_eq!(freq.len(), 3);
    }

    #[test]
    fn test_most_frequent_empty() {
        let store = make_store();
        assert!(store.most_frequent(10).is_empty());
    }

    // ── search_with_pattern ──────────────────────────────────────────────────

    #[test]
    fn test_search_with_pattern_case_insensitive() {
        let entries = vec![
            HistoryEntry::new("Hello World", 0, 1),
            HistoryEntry::new("hello", 1, 1),
            HistoryEntry::new("Goodbye", 2, 1),
        ];
        let idx = search_with_pattern(&entries, "hello", false);
        assert_eq!(idx, vec![0, 1]);
    }

    #[test]
    fn test_search_with_pattern_case_sensitive() {
        let entries = vec![
            HistoryEntry::new("Hello", 0, 1),
            HistoryEntry::new("hello", 1, 1),
        ];
        let idx = search_with_pattern(&entries, "Hello", true);
        assert_eq!(idx, vec![0]);
    }

    #[test]
    fn test_search_with_pattern_empty_pattern() {
        let entries = vec![HistoryEntry::new("a", 0, 1), HistoryEntry::new("b", 1, 1)];
        let idx = search_with_pattern(&entries, "", false);
        assert_eq!(idx, vec![0, 1]);
    }

    #[test]
    fn test_search_with_pattern_no_match() {
        let entries = vec![HistoryEntry::new("xyz", 0, 1)];
        let idx = search_with_pattern(&entries, "abc", false);
        assert!(idx.is_empty());
    }

    #[test]
    fn test_search_with_pattern_empty_entries() {
        let idx = search_with_pattern(&[], "abc", false);
        assert!(idx.is_empty());
    }

    // ── HistoryStore::search ─────────────────────────────────────────────────

    #[test]
    fn test_store_search_limit() {
        let mut store = make_store();
        for i in 0..20u32 {
            store.push(&format!("cmd {}", i));
        }
        let query = HistorySearchQuery::new("cmd").limit(5);
        let result = store.search(&query);
        assert_eq!(result.entries.len(), 5);
        assert_eq!(result.total_matches, 20);
    }

    #[test]
    fn test_store_search_empty_pattern_returns_all() {
        let mut store = make_store();
        store.push("alpha");
        store.push("beta");
        let query = HistorySearchQuery::new("").limit(100);
        let result = store.search(&query);
        assert_eq!(result.total_matches, 2);
    }

    #[test]
    fn test_store_search_no_match() {
        let mut store = make_store();
        store.push("hello");
        let query = HistorySearchQuery::new("zzz");
        let result = store.search(&query);
        assert_eq!(result.total_matches, 0);
        assert!(result.entries.is_empty());
    }

    // ── save / load ──────────────────────────────────────────────────────────

    #[test]
    fn test_save_load_roundtrip() {
        let mut store = make_store();
        store.push("first command");
        store.push("second");
        store.push("cmd with, comma");

        let path = temp_dir().join("oxilean_hist_test_roundtrip.csv");
        store.save(&path).expect("save failed");

        let loaded = HistoryStore::load(&path).expect("load failed");
        assert_eq!(loaded.len(), 3);
        assert_eq!(loaded.entries[0].command, "first command");
        assert_eq!(loaded.entries[1].command, "second");
        assert_eq!(loaded.entries[2].command, "cmd with, comma");

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_save_preserves_session_id() {
        let mut store = make_store();
        store.push("cmd");

        let path = temp_dir().join("oxilean_hist_test_session.csv");
        store.save(&path).expect("save");

        let loaded = HistoryStore::load(&path).expect("load");
        assert_eq!(loaded.entries[0].session_id, 1);

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_load_missing_file_returns_error() {
        let path = temp_dir().join("oxilean_hist_nonexistent_xyz.csv");
        let result = HistoryStore::load(&path);
        assert!(result.is_err());
    }

    #[test]
    fn test_save_empty_store() {
        let store = make_store();
        let path = temp_dir().join("oxilean_hist_test_empty.csv");
        store.save(&path).expect("save");

        let loaded = HistoryStore::load(&path).expect("load");
        assert_eq!(loaded.len(), 0);

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_save_load_command_with_quotes() {
        let mut store = make_store();
        store.push(r#"say "hello""#);

        let path = temp_dir().join("oxilean_hist_test_quotes.csv");
        store.save(&path).expect("save");

        let loaded = HistoryStore::load(&path).expect("load");
        assert_eq!(loaded.entries[0].command, r#"say "hello""#);

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_save_load_unicode() {
        let mut store = make_store();
        store.push("αβγ ∀x. P x");

        let path = temp_dir().join("oxilean_hist_test_unicode.csv");
        store.save(&path).expect("save");

        let loaded = HistoryStore::load(&path).expect("load");
        assert_eq!(loaded.entries[0].command, "αβγ ∀x. P x");

        let _ = std::fs::remove_file(&path);
    }

    // ── is_empty / len ───────────────────────────────────────────────────────

    #[test]
    fn test_is_empty() {
        let store = make_store();
        assert!(store.is_empty());
    }

    #[test]
    fn test_not_empty_after_push() {
        let mut store = make_store();
        store.push("x");
        assert!(!store.is_empty());
    }

    // ── session id ───────────────────────────────────────────────────────────

    #[test]
    fn test_session_id_preserved() {
        let cfg = HistoryConfig::default_config();
        let store = HistoryStore::with_session_id(cfg, 42);
        assert_eq!(store.session_id(), 42);
    }

    #[test]
    fn test_entry_has_correct_session_id() {
        let cfg = HistoryConfig::default_config().deduplicate(false);
        let mut store = HistoryStore::with_session_id(cfg, 99);
        store.push("test");
        assert_eq!(store.entries[0].session_id, 99);
    }
}
