//! Functions for incremental type checking
#![allow(dead_code)]

use std::collections::{HashMap, HashSet, VecDeque};

use super::types::{
    CheckStatus, DeclHash, DiagnosticInfo, EditDelta, IncrementalCache, IncrementalCheckResult,
    IncrementalEntry,
};

// ---------------------------------------------------------------------------
// Hashing
// ---------------------------------------------------------------------------

/// Compute a simple FNV-1a 64-bit hash of a declaration's source text.
///
/// FNV-1a is chosen because it is dependency-free and deterministic across
/// platforms — suitable for a cache key.
pub fn hash_declaration(source: &str) -> DeclHash {
    const FNV_OFFSET: u64 = 14_695_981_039_346_656_037;
    const FNV_PRIME: u64 = 1_099_511_628_211;

    let mut hash = FNV_OFFSET;
    for byte in source.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    DeclHash(hash)
}

// ---------------------------------------------------------------------------
// Declaration extraction
// ---------------------------------------------------------------------------

/// Keywords that introduce a new top-level declaration.
const DECL_KEYWORDS: &[&str] = &[
    "theorem",
    "def",
    "lemma",
    "axiom",
    "inductive",
    "structure",
    "class",
    "instance",
    "abbrev",
    "opaque",
    "noncomputable",
];

/// Extract top-level declarations from OxiLean source text.
///
/// Returns a `Vec<(name, body)>` where `body` is the full text of the
/// declaration (from its keyword to the line before the next keyword).
///
/// # Algorithm
///
/// The parser operates line-by-line.  A line is considered a declaration
/// starter when its first non-whitespace token is one of `DECL_KEYWORDS`
/// and it contains a name token immediately afterwards.  Everything up to
/// (but not including) the next starter is accumulated as the body.
pub fn extract_declarations(source: &str) -> Vec<(String, String)> {
    let lines: Vec<&str> = source.lines().collect();
    let mut result: Vec<(String, String)> = Vec::new();

    // Indices of lines that begin a new declaration
    let mut starter_indices: Vec<usize> = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        if let Some(name) = try_parse_decl_name(line) {
            starter_indices.push(idx);
            result.push((name, String::new()));
        }
    }

    // Populate bodies by slicing the source line ranges
    for (pos, &start_idx) in starter_indices.iter().enumerate() {
        let end_idx = starter_indices.get(pos + 1).copied().unwrap_or(lines.len());
        let body = lines[start_idx..end_idx].join("\n");
        if let Some(entry) = result.get_mut(pos) {
            entry.1 = body;
        }
    }

    result
}

/// Attempt to parse a declaration name from a single source line.
///
/// Returns `Some(name)` if the line starts a top-level declaration, or
/// `None` otherwise.
fn try_parse_decl_name(line: &str) -> Option<String> {
    let trimmed = line.trim_start();

    // Handle `noncomputable def / noncomputable theorem` etc.
    let effective = trimmed
        .strip_prefix("noncomputable")
        .map(str::trim_start)
        .unwrap_or(trimmed);

    // Check that the first token is a declaration keyword
    let rest = DECL_KEYWORDS
        .iter()
        .filter(|&&kw| kw != "noncomputable")
        .find_map(|&kw| {
            effective.strip_prefix(kw).and_then(|after| {
                // Must be followed by whitespace or end-of-line (not a longer word)
                if after.is_empty() || after.starts_with(char::is_whitespace) {
                    Some(after)
                } else {
                    None
                }
            })
        })?;

    // Extract the name token (first word after the keyword)
    let name = rest
        .split_whitespace()
        .next()
        .map(|tok| {
            // Strip trailing colon or opening paren that might be attached
            tok.trim_end_matches(':')
                .trim_end_matches('(')
                .trim_end_matches('{')
        })
        .filter(|tok| !tok.is_empty() && is_valid_ident_start(tok))
        .map(str::to_string)?;

    Some(name)
}

/// Heuristic: does `s` look like a valid Lean identifier start?
fn is_valid_ident_start(s: &str) -> bool {
    s.chars()
        .next()
        .map(|c| c.is_alphabetic() || c == '_')
        .unwrap_or(false)
}

// ---------------------------------------------------------------------------
// Edit delta computation
// ---------------------------------------------------------------------------

/// Compute the declaration-level diff between a previous cache state and the
/// declarations extracted from new source.
pub fn compute_edit_delta(
    old_cache: &IncrementalCache,
    new_decls: &[(String, String)],
) -> EditDelta {
    let new_map: HashMap<&str, DeclHash> = new_decls
        .iter()
        .map(|(name, body)| (name.as_str(), hash_declaration(body)))
        .collect();

    let old_names: HashSet<&str> = old_cache.entries.keys().map(String::as_str).collect();
    let new_names: HashSet<&str> = new_map.keys().copied().collect();

    let added: Vec<String> = new_names
        .difference(&old_names)
        .map(|s| (*s).to_string())
        .collect();

    let removed: Vec<String> = old_names
        .difference(&new_names)
        .map(|s| (*s).to_string())
        .collect();

    let modified: Vec<String> = new_names
        .intersection(&old_names)
        .filter(|&&name| {
            old_cache
                .entries
                .get(name)
                .map(|e| e.hash != new_map[name])
                .unwrap_or(false)
        })
        .map(|s| (*s).to_string())
        .collect();

    EditDelta {
        added,
        removed,
        modified,
    }
}

// ---------------------------------------------------------------------------
// Dependency invalidation
// ---------------------------------------------------------------------------

/// Transitively mark all dependents of `changed` declarations as `Pending`.
///
/// The algorithm performs a BFS over the reverse-dependency graph: for each
/// changed declaration, any declaration that lists it in its `deps` is itself
/// marked `Pending` and enqueued for further propagation.
///
/// The reverse-dependency map is built using owned `String`s so that the
/// immutable borrow of `cache.entries` is fully released before any mutable
/// borrows are taken.
pub fn invalidate_dependents(cache: &mut IncrementalCache, changed: &[String]) {
    // Build owned reverse-dependency map: dep_name -> Vec<dependent_name>
    let mut reverse_deps: HashMap<String, Vec<String>> = HashMap::new();
    for (name, entry) in &cache.entries {
        for dep in &entry.deps {
            reverse_deps
                .entry(dep.clone())
                .or_default()
                .push(name.clone());
        }
    }
    // The immutable borrow of `cache.entries` ends here.

    let mut queue: VecDeque<String> = changed.iter().cloned().collect();
    let mut visited: HashSet<String> = changed.iter().cloned().collect();

    while let Some(current) = queue.pop_front() {
        if let Some(dependents) = reverse_deps.get(&current) {
            for dependent in dependents {
                if !visited.contains(dependent.as_str()) {
                    visited.insert(dependent.clone());
                    if let Some(entry) = cache.entries.get_mut(dependent.as_str()) {
                        entry.status = CheckStatus::Pending;
                    }
                    queue.push_back(dependent.clone());
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Dependency extraction (heuristic)
// ---------------------------------------------------------------------------

/// Extract heuristic dependencies from a declaration body.
///
/// Scans the body for identifiers that match known declaration names (provided
/// as `known_names`).  This is a conservative over-approximation: it may
/// produce false positives but will never miss a true dependency.
fn extract_deps_from_body(body: &str, known_names: &HashSet<String>) -> Vec<String> {
    // Collect all word-like tokens from the body
    let tokens: HashSet<&str> = body
        .split(|c: char| !c.is_alphanumeric() && c != '_')
        .filter(|tok| !tok.is_empty())
        .collect();

    known_names
        .iter()
        .filter(|name| tokens.contains(name.as_str()))
        .cloned()
        .collect()
}

// ---------------------------------------------------------------------------
// Main incremental check entry point
// ---------------------------------------------------------------------------

/// Perform an incremental type-check of `source`.
///
/// If `old_cache` is `None` a fresh cache is created and all declarations are
/// checked.  Otherwise only declarations that are new, modified, or
/// transitively invalidated by their dependencies are re-checked; the rest are
/// served from the cache.
pub fn incremental_check(
    source: &str,
    old_cache: Option<IncrementalCache>,
) -> IncrementalCheckResult {
    let mut cache = old_cache.unwrap_or_default();
    cache.version = cache.version.saturating_add(1);

    let new_decls = extract_declarations(source);

    // Compute what changed at the declaration level
    let delta = compute_edit_delta(&cache, &new_decls);

    // Remove deleted declarations from the cache
    for removed_name in &delta.removed {
        cache.entries.remove(removed_name);
    }

    // Collect names that need re-checking: added + modified
    let mut need_recheck: HashSet<String> = HashSet::new();
    for name in delta.added.iter().chain(delta.modified.iter()) {
        need_recheck.insert(name.clone());
    }

    // Transitively invalidate dependents of modified/removed declarations
    let changed_for_invalidation: Vec<String> = delta
        .modified
        .iter()
        .chain(delta.removed.iter())
        .cloned()
        .collect();

    if !changed_for_invalidation.is_empty() {
        invalidate_dependents(&mut cache, &changed_for_invalidation);
    }

    // Any entry still marked Pending also needs a recheck
    for (name, entry) in &cache.entries {
        if matches!(entry.status, CheckStatus::Pending) {
            need_recheck.insert(name.clone());
        }
    }

    // Build the known-name set for dependency extraction
    let known_names: HashSet<String> = new_decls.iter().map(|(n, _)| n.clone()).collect();

    let mut diagnostics: Vec<DiagnosticInfo> = Vec::new();
    let mut recheck_count = 0usize;
    let mut cache_hit_count = 0usize;

    for (name, body) in &new_decls {
        let new_hash = hash_declaration(body);

        if need_recheck.contains(name.as_str()) {
            // Re-check this declaration
            recheck_count += 1;

            let deps = extract_deps_from_body(body, &known_names)
                .into_iter()
                .filter(|d| d != name)
                .collect::<Vec<_>>();

            let (status, maybe_diag) = simulate_check(name, body);

            if let Some(diag) = maybe_diag {
                diagnostics.push(diag);
            }

            cache.entries.insert(
                name.clone(),
                IncrementalEntry {
                    name: name.clone(),
                    hash: new_hash,
                    checked_at: cache.version,
                    deps,
                    status,
                },
            );
        } else {
            // Cache hit — declaration unchanged and all deps still valid
            cache_hit_count += 1;
            // Refresh the hash in case we are building from a cold start
            // (this is a no-op when the hash is already correct)
            if let Some(entry) = cache.entries.get_mut(name.as_str()) {
                entry.hash = new_hash;
            }
        }
    }

    IncrementalCheckResult {
        cache,
        diagnostics,
        recheck_count,
        cache_hit_count,
    }
}

// ---------------------------------------------------------------------------
// Type-check simulation
// ---------------------------------------------------------------------------

/// Simulate type-checking a single declaration.
///
/// In the real implementation this would call into the kernel.  Here we apply
/// a set of heuristic rules that are sufficient to exercise the incremental
/// machinery in tests.
fn simulate_check(name: &str, body: &str) -> (CheckStatus, Option<DiagnosticInfo>) {
    // Rule 1: declarations containing `sorry` are warnings
    if body.contains("sorry") {
        return (
            CheckStatus::Ok,
            Some(DiagnosticInfo {
                name: name.to_string(),
                message: format!("'{}' uses sorry — proof is incomplete", name),
                severity: 2,
            }),
        );
    }

    // Rule 2: declarations whose body looks like it is missing a proof body
    // (`theorem foo : T` with nothing after the colon type) are errors
    if is_missing_proof_body(body) {
        return (
            CheckStatus::Error(format!(
                "'{}' appears to be missing a proof or definition body",
                name
            )),
            Some(DiagnosticInfo {
                name: name.to_string(),
                message: format!("'{}' is missing a proof or definition body", name),
                severity: 3,
            }),
        );
    }

    (CheckStatus::Ok, None)
}

/// Heuristic: returns `true` when a theorem or lemma has no `:= …` or `by …`.
fn is_missing_proof_body(body: &str) -> bool {
    let is_theorem =
        body.trim_start().starts_with("theorem") || body.trim_start().starts_with("lemma");
    if !is_theorem {
        return false;
    }
    // A well-formed theorem has either `:=` or `by` somewhere after the type
    !body.contains(":=") && !body.contains(" by ") && !body.contains("\nby ")
}

// ---------------------------------------------------------------------------
// Cache statistics
// ---------------------------------------------------------------------------

/// Return a human-readable summary of cache statistics.
pub fn cache_stats(cache: &IncrementalCache) -> String {
    let total = cache.entries.len();
    let ok_count = cache
        .entries
        .values()
        .filter(|e| matches!(e.status, CheckStatus::Ok))
        .count();
    let error_count = cache
        .entries
        .values()
        .filter(|e| matches!(e.status, CheckStatus::Error(_)))
        .count();
    let pending_count = cache
        .entries
        .values()
        .filter(|e| matches!(e.status, CheckStatus::Pending))
        .count();

    format!(
        "IncrementalCache v{}: {} total ({} ok, {} error, {} pending)",
        cache.version, total, ok_count, error_count, pending_count
    )
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- hash_declaration ---------------------------------------------------

    #[test]
    fn test_hash_declaration_deterministic() {
        let h1 = hash_declaration("theorem foo : True := trivial");
        let h2 = hash_declaration("theorem foo : True := trivial");
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_hash_declaration_different_inputs() {
        let h1 = hash_declaration("theorem foo : True := trivial");
        let h2 = hash_declaration("theorem bar : True := trivial");
        assert_ne!(h1, h2);
    }

    #[test]
    fn test_hash_declaration_empty() {
        let h = hash_declaration("");
        // Should return the FNV offset basis — just verify it's non-zero
        assert_ne!(h.0, 0);
    }

    #[test]
    fn test_hash_declaration_whitespace_sensitive() {
        let h1 = hash_declaration("def f := 1");
        let h2 = hash_declaration("def f :=  1");
        assert_ne!(h1, h2);
    }

    // --- extract_declarations -----------------------------------------------

    #[test]
    fn test_extract_declarations_empty() {
        let decls = extract_declarations("");
        assert!(decls.is_empty());
    }

    #[test]
    fn test_extract_declarations_single_theorem() {
        let src = "theorem foo : True := trivial";
        let decls = extract_declarations(src);
        assert_eq!(decls.len(), 1);
        assert_eq!(decls[0].0, "foo");
    }

    #[test]
    fn test_extract_declarations_multiple() {
        let src = "theorem foo : True := trivial\ndef bar := 42\nlemma baz : 1 = 1 := rfl";
        let decls = extract_declarations(src);
        assert_eq!(decls.len(), 3);
        let names: Vec<&str> = decls.iter().map(|(n, _)| n.as_str()).collect();
        assert!(names.contains(&"foo"));
        assert!(names.contains(&"bar"));
        assert!(names.contains(&"baz"));
    }

    #[test]
    fn test_extract_declarations_body_contains_keyword_lines() {
        // Multi-line theorem body that includes `intro` (a keyword inside tactic)
        let src = "theorem t : True := by\n  trivial\ndef f := 0";
        let decls = extract_declarations(src);
        assert_eq!(decls.len(), 2);
        // The body of the theorem should include the `by` line
        assert!(decls[0].1.contains("trivial"));
    }

    #[test]
    fn test_extract_declarations_axiom() {
        let src = "axiom em : ∀ (p : Prop), p ∨ ¬p";
        let decls = extract_declarations(src);
        assert_eq!(decls.len(), 1);
        assert_eq!(decls[0].0, "em");
    }

    #[test]
    fn test_extract_declarations_comments_ignored() {
        // Lines starting with `--` should not produce declarations
        let src = "-- this is a comment\ntheorem foo : True := trivial";
        let decls = extract_declarations(src);
        assert_eq!(decls.len(), 1);
        assert_eq!(decls[0].0, "foo");
    }

    // --- compute_edit_delta -------------------------------------------------

    #[test]
    fn test_compute_edit_delta_all_added() {
        let cache = IncrementalCache::new();
        let decls = vec![
            (
                "foo".to_string(),
                "theorem foo : True := trivial".to_string(),
            ),
            ("bar".to_string(), "def bar := 42".to_string()),
        ];
        let delta = compute_edit_delta(&cache, &decls);
        assert_eq!(delta.added.len(), 2);
        assert!(delta.removed.is_empty());
        assert!(delta.modified.is_empty());
    }

    #[test]
    fn test_compute_edit_delta_removed() {
        let mut cache = IncrementalCache::new();
        cache.entries.insert(
            "foo".to_string(),
            IncrementalEntry {
                name: "foo".to_string(),
                hash: hash_declaration("theorem foo : True := trivial"),
                checked_at: 1,
                deps: vec![],
                status: CheckStatus::Ok,
            },
        );
        let decls: Vec<(String, String)> = vec![];
        let delta = compute_edit_delta(&cache, &decls);
        assert!(delta.added.is_empty());
        assert_eq!(delta.removed.len(), 1);
        assert!(delta.modified.is_empty());
    }

    #[test]
    fn test_compute_edit_delta_modified() {
        let body_old = "theorem foo : True := trivial";
        let body_new = "theorem foo : True := by trivial";
        let mut cache = IncrementalCache::new();
        cache.entries.insert(
            "foo".to_string(),
            IncrementalEntry {
                name: "foo".to_string(),
                hash: hash_declaration(body_old),
                checked_at: 1,
                deps: vec![],
                status: CheckStatus::Ok,
            },
        );
        let decls = vec![("foo".to_string(), body_new.to_string())];
        let delta = compute_edit_delta(&cache, &decls);
        assert!(delta.added.is_empty());
        assert!(delta.removed.is_empty());
        assert_eq!(delta.modified.len(), 1);
        assert_eq!(delta.modified[0], "foo");
    }

    #[test]
    fn test_compute_edit_delta_unchanged() {
        let body = "theorem foo : True := trivial";
        let mut cache = IncrementalCache::new();
        cache.entries.insert(
            "foo".to_string(),
            IncrementalEntry {
                name: "foo".to_string(),
                hash: hash_declaration(body),
                checked_at: 1,
                deps: vec![],
                status: CheckStatus::Ok,
            },
        );
        let decls = vec![("foo".to_string(), body.to_string())];
        let delta = compute_edit_delta(&cache, &decls);
        assert!(delta.added.is_empty());
        assert!(delta.removed.is_empty());
        assert!(delta.modified.is_empty());
    }

    // --- invalidate_dependents ----------------------------------------------

    #[test]
    fn test_invalidate_dependents_direct() {
        let mut cache = IncrementalCache::new();
        cache.entries.insert(
            "base".to_string(),
            IncrementalEntry {
                name: "base".to_string(),
                hash: DeclHash(0),
                checked_at: 1,
                deps: vec![],
                status: CheckStatus::Ok,
            },
        );
        cache.entries.insert(
            "derived".to_string(),
            IncrementalEntry {
                name: "derived".to_string(),
                hash: DeclHash(1),
                checked_at: 1,
                deps: vec!["base".to_string()],
                status: CheckStatus::Ok,
            },
        );
        invalidate_dependents(&mut cache, &["base".to_string()]);
        assert!(matches!(
            cache.entries["derived"].status,
            CheckStatus::Pending
        ));
        // The changed node itself is not touched by invalidate_dependents
        assert!(matches!(cache.entries["base"].status, CheckStatus::Ok));
    }

    #[test]
    fn test_invalidate_dependents_transitive() {
        let mut cache = IncrementalCache::new();
        for (name, deps) in [
            ("a", vec![]),
            ("b", vec!["a"]),
            ("c", vec!["b"]),
            ("d", vec!["c"]),
        ] {
            cache.entries.insert(
                name.to_string(),
                IncrementalEntry {
                    name: name.to_string(),
                    hash: DeclHash(0),
                    checked_at: 1,
                    deps: deps.iter().map(|s| s.to_string()).collect(),
                    status: CheckStatus::Ok,
                },
            );
        }
        invalidate_dependents(&mut cache, &["a".to_string()]);
        assert!(matches!(cache.entries["b"].status, CheckStatus::Pending));
        assert!(matches!(cache.entries["c"].status, CheckStatus::Pending));
        assert!(matches!(cache.entries["d"].status, CheckStatus::Pending));
    }

    #[test]
    fn test_invalidate_dependents_no_dependents() {
        let mut cache = IncrementalCache::new();
        cache.entries.insert(
            "standalone".to_string(),
            IncrementalEntry {
                name: "standalone".to_string(),
                hash: DeclHash(0),
                checked_at: 1,
                deps: vec![],
                status: CheckStatus::Ok,
            },
        );
        invalidate_dependents(&mut cache, &["standalone".to_string()]);
        // Nothing else to invalidate
        assert!(matches!(
            cache.entries["standalone"].status,
            CheckStatus::Ok
        ));
    }

    // --- incremental_check --------------------------------------------------

    #[test]
    fn test_incremental_check_empty_source() {
        let result = incremental_check("", None);
        assert_eq!(result.recheck_count, 0);
        assert_eq!(result.cache_hit_count, 0);
        assert!(result.diagnostics.is_empty());
        assert!(result.cache.entries.is_empty());
    }

    #[test]
    fn test_incremental_check_fresh_cache() {
        let src = "theorem foo : True := trivial\ndef bar := 42";
        let result = incremental_check(src, None);
        assert_eq!(result.recheck_count, 2);
        assert_eq!(result.cache_hit_count, 0);
        assert_eq!(result.cache.entries.len(), 2);
    }

    #[test]
    fn test_incremental_check_unchanged_source_is_cache_hit() {
        let src = "theorem foo : True := trivial\ndef bar := 42";
        let first = incremental_check(src, None);
        let second = incremental_check(src, Some(first.cache));
        // Nothing changed — all should be cache hits
        assert_eq!(second.recheck_count, 0);
        assert_eq!(second.cache_hit_count, 2);
    }

    #[test]
    fn test_incremental_check_only_rechecks_modified() {
        let src1 = "theorem foo : True := trivial\ndef bar := 42";
        let src2 = "theorem foo : True := by trivial\ndef bar := 42";
        let first = incremental_check(src1, None);
        let second = incremental_check(src2, Some(first.cache));
        // Only `foo` changed
        assert_eq!(second.recheck_count, 1);
        assert_eq!(second.cache_hit_count, 1);
    }

    #[test]
    fn test_incremental_check_sorry_produces_warning() {
        let src = "theorem foo : True := sorry";
        let result = incremental_check(src, None);
        assert!(!result.diagnostics.is_empty());
        assert_eq!(result.diagnostics[0].severity, 2);
        assert!(result.diagnostics[0].message.contains("sorry"));
    }

    #[test]
    fn test_incremental_check_version_increments() {
        let src = "def x := 1";
        let r1 = incremental_check(src, None);
        let v1 = r1.cache.version;
        let r2 = incremental_check(src, Some(r1.cache));
        assert_eq!(r2.cache.version, v1 + 1);
    }

    #[test]
    fn test_incremental_check_added_decl() {
        let src1 = "theorem foo : True := trivial";
        let src2 = "theorem foo : True := trivial\ndef bar := 42";
        let r1 = incremental_check(src1, None);
        let r2 = incremental_check(src2, Some(r1.cache));
        // `foo` unchanged (cache hit), `bar` added (recheck)
        assert_eq!(r2.recheck_count, 1);
        assert_eq!(r2.cache_hit_count, 1);
    }

    #[test]
    fn test_incremental_check_removed_decl() {
        let src1 = "theorem foo : True := trivial\ndef bar := 42";
        let src2 = "theorem foo : True := trivial";
        let r1 = incremental_check(src1, None);
        let r2 = incremental_check(src2, Some(r1.cache));
        assert_eq!(r2.cache.entries.len(), 1);
        assert!(r2.cache.entries.contains_key("foo"));
    }

    #[test]
    fn test_incremental_check_dependent_invalidated() {
        // `bar` depends on `foo`; changing `foo` should force re-check of `bar`
        let src1 = "def foo := 1\ndef bar := foo + 1";
        let r1 = incremental_check(src1, None);

        // Manually set bar's deps to include foo so dependency is tracked
        let mut cache = r1.cache;
        if let Some(entry) = cache.entries.get_mut("bar") {
            entry.deps = vec!["foo".to_string()];
        }

        let src2 = "def foo := 2\ndef bar := foo + 1";
        let r2 = incremental_check(src2, Some(cache));
        // Both foo (modified) and bar (dep-invalidated) should be re-checked
        assert_eq!(r2.recheck_count, 2);
    }

    // --- cache_stats --------------------------------------------------------

    #[test]
    fn test_cache_stats_empty() {
        let cache = IncrementalCache::new();
        let stats = cache_stats(&cache);
        assert!(stats.contains('0'));
    }

    #[test]
    fn test_cache_stats_counts() {
        let mut cache = IncrementalCache::new();
        cache.version = 3;
        cache.entries.insert(
            "a".to_string(),
            IncrementalEntry {
                name: "a".to_string(),
                hash: DeclHash(0),
                checked_at: 1,
                deps: vec![],
                status: CheckStatus::Ok,
            },
        );
        cache.entries.insert(
            "b".to_string(),
            IncrementalEntry {
                name: "b".to_string(),
                hash: DeclHash(1),
                checked_at: 2,
                deps: vec![],
                status: CheckStatus::Error("oops".to_string()),
            },
        );
        cache.entries.insert(
            "c".to_string(),
            IncrementalEntry {
                name: "c".to_string(),
                hash: DeclHash(2),
                checked_at: 3,
                deps: vec![],
                status: CheckStatus::Pending,
            },
        );
        let stats = cache_stats(&cache);
        assert!(stats.contains("1 ok"));
        assert!(stats.contains("1 error"));
        assert!(stats.contains("1 pending"));
        assert!(stats.contains('3')); // version
    }

    #[test]
    fn test_cache_stats_all_ok() {
        let src = "theorem p : True := trivial\ntheorem q : True := trivial";
        let result = incremental_check(src, None);
        let stats = cache_stats(&result.cache);
        assert!(stats.contains("ok"));
        assert!(stats.contains('2'));
    }
}
