//! oxilake.lock-aware cache invalidation.
//!
//! When `oxilake.lock` changes (dependency versions re-resolved), the entire
//! build cache must be invalidated because compiled outputs may depend on
//! different library versions.

use super::types::{ContentHash, InvalidationReason, InvalidationResult};
use std::path::Path;

/// Computes a stable hash of the lockfile's dependency record.
///
/// Uses FNV-1a over the raw lockfile bytes. The hash is stable: same bytes → same hash.
pub fn hash_lockfile_content(content: &str) -> ContentHash {
    let mut hash: u64 = 0xcbf29ce484222325; // FNV offset basis
    for byte in content.bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x100000001b3); // FNV prime
    }
    ContentHash(hash)
}

/// Parse an oxilake.lock file and extract dependency name → version pairs.
///
/// The lockfile format (written by oxilake) is simple key=value per entry:
/// ```toml
/// # OxiLake lock file
///
/// [[package]]
/// name = "mylib"
/// version = "0.1.0"
///
/// [[package]]
/// name = "another"
/// version = "0.2.3"
/// ```
///
/// Returns a sorted `Vec<(name, version)>` for stable comparison.
pub fn parse_lockfile_deps(content: &str) -> Vec<(String, String)> {
    let mut result: Vec<(String, String)> = Vec::new();
    let mut current_name: Option<String> = None;
    let mut current_version: Option<String> = None;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed == "[[package]]" {
            // Flush previous entry
            if let (Some(name), Some(version)) = (current_name.take(), current_version.take()) {
                result.push((name, version));
            }
            current_name = None;
            current_version = None;
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("name = ") {
            current_name = Some(rest.trim_matches('"').to_string());
        } else if let Some(rest) = trimmed.strip_prefix("version = ") {
            current_version = Some(rest.trim_matches('"').to_string());
        }
    }
    // Flush last entry
    if let (Some(name), Some(version)) = (current_name, current_version) {
        result.push((name, version));
    }
    result.sort();
    result
}

/// State record for the previously-seen lockfile hash.
///
/// Stored alongside the build cache so the invalidator can detect changes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LockfileState {
    /// FNV-1a hash of the lockfile content when last checked.
    pub content_hash: ContentHash,
    /// Resolved dependency count at last check.
    pub dep_count: usize,
}

impl LockfileState {
    /// Create from a lockfile content string.
    pub fn from_content(content: &str) -> Self {
        Self {
            content_hash: hash_lockfile_content(content),
            dep_count: parse_lockfile_deps(content).len(),
        }
    }
}

/// Checks the current lockfile against a previously-seen state.
///
/// If the lockfile changed (hash mismatch), all provided `known_paths` are
/// marked for rebuild with `InvalidationReason::LockfileChanged`.
///
/// If `lockfile_path` doesn't exist, this function does nothing (no oxilake project).
pub fn check_lockfile_invalidation(
    lockfile_path: &Path,
    prior_state: Option<&LockfileState>,
    known_paths: &[String],
) -> Option<InvalidationResult> {
    let content = match std::fs::read_to_string(lockfile_path) {
        Ok(c) => c,
        Err(_) => return None, // No lockfile — nothing to invalidate
    };

    let new_hash = hash_lockfile_content(&content);

    let prev_hash = match prior_state {
        None => {
            // First time seeing lockfile; record state but don't invalidate
            return None;
        }
        Some(s) => s.content_hash,
    };

    if new_hash == prev_hash {
        return None; // Unchanged
    }

    let reason = InvalidationReason::LockfileChanged {
        prev_hash: format!("{:016x}", prev_hash.value()),
        new_hash: format!("{:016x}", new_hash.value()),
    };

    let mut reasons = std::collections::HashMap::new();
    for p in known_paths {
        reasons.insert(p.clone(), reason.clone());
    }

    Some(InvalidationResult {
        to_rebuild: known_paths.to_vec(),
        reasons,
        unchanged: Vec::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn write_temp_lockfile(content: &str, name: &str) -> std::path::PathBuf {
        let mut p = std::env::temp_dir();
        p.push(name);
        let mut f = std::fs::File::create(&p).expect("create lockfile");
        f.write_all(content.as_bytes()).expect("write");
        p
    }

    const SAMPLE_LOCK: &str = r#"# OxiLake lock file

[[package]]
name = "oxilean-kernel"
version = "0.1.3"

[[package]]
name = "oxilean-std"
version = "0.1.3"
"#;

    #[test]
    fn test_hash_lockfile_deterministic() {
        let h1 = hash_lockfile_content(SAMPLE_LOCK);
        let h2 = hash_lockfile_content(SAMPLE_LOCK);
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_hash_lockfile_changes_on_diff_content() {
        let h1 = hash_lockfile_content(SAMPLE_LOCK);
        let h2 = hash_lockfile_content("# Different content\n");
        assert_ne!(h1, h2);
    }

    #[test]
    fn test_parse_lockfile_deps_two_packages() {
        let deps = parse_lockfile_deps(SAMPLE_LOCK);
        assert_eq!(deps.len(), 2);
        assert!(deps.contains(&("oxilean-kernel".into(), "0.1.3".into())));
        assert!(deps.contains(&("oxilean-std".into(), "0.1.3".into())));
    }

    #[test]
    fn test_parse_lockfile_deps_empty() {
        let deps = parse_lockfile_deps("# Empty lockfile\n");
        assert!(deps.is_empty());
    }

    #[test]
    fn test_lockfile_state_from_content() {
        let state = LockfileState::from_content(SAMPLE_LOCK);
        assert_eq!(state.dep_count, 2);
        assert_eq!(state.content_hash, hash_lockfile_content(SAMPLE_LOCK));
    }

    #[test]
    fn test_check_lockfile_no_prior_state_returns_none() {
        let path = write_temp_lockfile(SAMPLE_LOCK, "oxilake_no_prior.lock");
        let result = check_lockfile_invalidation(&path, None, &["src/Foo.lean".into()]);
        assert!(result.is_none());
    }

    #[test]
    fn test_check_lockfile_unchanged_returns_none() {
        let path = write_temp_lockfile(SAMPLE_LOCK, "oxilake_unchanged.lock");
        let prior = LockfileState::from_content(SAMPLE_LOCK);
        let result = check_lockfile_invalidation(&path, Some(&prior), &["src/Foo.lean".into()]);
        assert!(result.is_none());
    }

    #[test]
    fn test_check_lockfile_changed_invalidates_all() {
        let changed = SAMPLE_LOCK.replace("0.1.3", "0.2.0");
        let path = write_temp_lockfile(&changed, "oxilake_changed.lock");
        let prior = LockfileState::from_content(SAMPLE_LOCK);
        let paths = vec!["src/A.lean".into(), "src/B.lean".into()];
        let result =
            check_lockfile_invalidation(&path, Some(&prior), &paths).expect("should invalidate");
        assert_eq!(result.to_rebuild.len(), 2);
        assert!(result.unchanged.is_empty());
        assert!(matches!(
            &result.reasons["src/A.lean"],
            InvalidationReason::LockfileChanged { .. }
        ));
    }

    #[test]
    fn test_check_lockfile_missing_file_returns_none() {
        let p: std::path::PathBuf = "/nonexistent/path/oxilake.lock".into();
        let prior = LockfileState::from_content(SAMPLE_LOCK);
        let result = check_lockfile_invalidation(&p, Some(&prior), &["A.lean".into()]);
        assert!(result.is_none());
    }

    #[test]
    fn test_lockfile_changed_reason_display() {
        let reason = InvalidationReason::LockfileChanged {
            prev_hash: "aaaa".into(),
            new_hash: "bbbb".into(),
        };
        let s = reason.to_string();
        assert!(s.contains("lockfile_changed"));
        assert!(s.contains("aaaa"));
        assert!(s.contains("bbbb"));
    }
}
