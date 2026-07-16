//! Functions for incremental type checking
#![allow(dead_code)]

use std::collections::{HashMap, HashSet, VecDeque};

use oxilean_parse::{
    incremental::{diff_modules, DeclFingerprint, EditKind},
    Decl, Located,
};

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

/// Parse `source` into a sequence of located declarations using the real parser.
///
/// On parse error the loop stops early (best-effort); partial results are
/// returned rather than propagating the error, which mirrors the tolerance
/// required for an incremental editor checker.
fn parse_source_decls(source: &str) -> Vec<Located<Decl>> {
    use oxilean_parse::{Lexer, Parser};

    let tokens = Lexer::new(source).tokenize();
    let mut parser = Parser::new(tokens);
    let mut decls = Vec::new();
    loop {
        if parser.is_eof() {
            break;
        }
        match parser.parse_decl() {
            Ok(d) => decls.push(d),
            Err(e) => {
                if e.is_eof() {
                    break;
                }
                // Non-EOF parse error: advance one token and continue so that
                // the rest of the file is still processed.
                parser.advance();
            }
        }
    }
    decls
}

/// Derive a `DeclHash` from a `DeclFingerprint` body hash so that the entry
/// hash stored in `IncrementalCache` stays consistent with diff-based checks.
fn fingerprint_to_decl_hash(fp: &DeclFingerprint) -> DeclHash {
    DeclHash(fp.body_hash)
}

/// Perform an incremental type-check of `source`.
///
/// If `old_cache` is `None` a fresh cache is created and all declarations are
/// checked.  Otherwise `diff_modules` is used to compare the previously parsed
/// AST with the freshly-parsed AST: only declarations that are `Inserted`,
/// `Modified`, or transitively invalidated through the dependency graph are
/// re-checked; `Unchanged` declarations are served from the cache.
///
/// The first changed declaration index and all subsequent declarations are
/// re-checked (tail-invalidation), matching the semantics of a sequential
/// proof assistant where later declarations may depend on earlier ones.
pub fn incremental_check(
    source: &str,
    old_cache: Option<IncrementalCache>,
) -> IncrementalCheckResult {
    let mut cache = old_cache.unwrap_or_default();
    cache.version = cache.version.saturating_add(1);

    // ── 1. Parse new source ──────────────────────────────────────────────────
    let new_decls = parse_source_decls(source);

    // ── 2. Diff against the previous parse ──────────────────────────────────
    let edits = diff_modules(&cache.prev_decls, &new_decls);

    // ── 3. Determine which declarations need re-checking ────────────────────
    // Tail-invalidation: once the first changed declaration is encountered,
    // all subsequent declarations must also be re-checked (they may depend on
    // earlier declarations that just changed).
    let mut tail_invalidate = false;
    let mut need_recheck: HashSet<String> = HashSet::new();
    let mut deleted_names: Vec<String> = Vec::new();

    for edit in &edits {
        match edit.kind {
            EditKind::Deleted => {
                deleted_names.push(edit.fingerprint.name.clone());
                tail_invalidate = true;
            }
            EditKind::Inserted | EditKind::Modified => {
                need_recheck.insert(edit.fingerprint.name.clone());
                tail_invalidate = true;
            }
            EditKind::Unchanged => {
                if tail_invalidate {
                    // All declarations after the first change must be re-checked.
                    need_recheck.insert(edit.fingerprint.name.clone());
                }
            }
        }
    }

    // ── 4. Remove deleted declarations from the cache ────────────────────────
    for name in &deleted_names {
        cache.entries.remove(name.as_str());
    }

    // ── 5. Transitively invalidate dependents of changed/removed decls ───────
    let changed_for_invalidation: Vec<String> = need_recheck
        .iter()
        .cloned()
        .chain(deleted_names.iter().cloned())
        .collect();

    if !changed_for_invalidation.is_empty() {
        invalidate_dependents(&mut cache, &changed_for_invalidation);
    }

    // Any entry still marked Pending also needs a recheck.
    for (name, entry) in &cache.entries {
        if matches!(entry.status, CheckStatus::Pending) {
            need_recheck.insert(name.clone());
        }
    }

    // ── 6. Build known-name set for dependency extraction ────────────────────
    let known_names: HashSet<String> = new_decls
        .iter()
        .filter_map(|d| d.value.name().map(str::to_string))
        .collect();

    // ── 7. Build a fingerprint lookup for the new decls ──────────────────────
    let new_fps: HashMap<String, DeclFingerprint> = new_decls
        .iter()
        .filter_map(|d| {
            d.value
                .name()
                .map(|n| (n.to_string(), DeclFingerprint::of(d)))
        })
        .collect();

    // ── 8. Loop over new declarations: cache hit or re-check ─────────────────
    let mut diagnostics: Vec<DiagnosticInfo> = Vec::new();
    let mut recheck_count = 0usize;
    let mut cache_hit_count = 0usize;

    for located_decl in &new_decls {
        let name = match located_decl.value.name() {
            Some(n) => n.to_string(),
            None => continue, // anonymous / unnamed decls are skipped
        };

        let fp = match new_fps.get(&name) {
            Some(f) => f,
            None => continue,
        };
        let new_hash = fingerprint_to_decl_hash(fp);

        // Obtain a body string for simulate_check (fall back to empty string)
        let body = extract_body_text(source, located_decl);

        if need_recheck.contains(name.as_str()) {
            recheck_count += 1;

            let deps = extract_deps_from_body(&body, &known_names)
                .into_iter()
                .filter(|d| *d != name)
                .collect::<Vec<_>>();

            let (status, maybe_diag) = simulate_check(&name, &body);
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
            // Cache hit — declaration is Unchanged and no deps were invalidated.
            cache_hit_count += 1;
            if let Some(entry) = cache.entries.get_mut(name.as_str()) {
                entry.hash = new_hash;
            }
        }
    }

    // ── 9. Persist the parsed declaration sequence for the next call ─────────
    cache.prev_decls = new_decls;

    IncrementalCheckResult {
        cache,
        diagnostics,
        recheck_count,
        cache_hit_count,
    }
}

/// Extract a best-effort body string for a located declaration.
///
/// When the span is available and valid within the source, we slice it
/// directly.  Otherwise we fall back to an empty string so that
/// `simulate_check` can still run.
fn extract_body_text(source: &str, located: &Located<Decl>) -> String {
    let span = located.span.clone();
    let start = span.start;
    let end = span.end;
    if start <= end && end <= source.len() {
        source[start..end].to_string()
    } else {
        String::new()
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
        // `foo` is modified (first change); tail-invalidation also forces `bar`
        // to be re-checked because it follows the first changed declaration.
        // Total: 2 re-checks, 0 cache hits.
        assert_eq!(second.recheck_count, 2);
        assert_eq!(second.cache_hit_count, 0);
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

    // --- diff_modules-based incremental_check tests -------------------------

    /// Running the same 3-decl source twice: the second pass should get
    /// 3 cache hits and 0 rechecks because `diff_modules` reports all as
    /// `Unchanged`.
    #[test]
    fn test_incremental_check_full_equals_incremental() {
        let src = "theorem a : True := True.intro\n\
                   theorem b : True := True.intro\n\
                   theorem c : True := True.intro";

        // First pass: cold cache → all 3 declarations checked
        let r1 = incremental_check(src, None);
        assert_eq!(r1.recheck_count, 3, "first pass should check all 3 decls");
        assert_eq!(r1.cache_hit_count, 0);

        // Second pass: same source → all 3 should be cache hits
        let r2 = incremental_check(src, Some(r1.cache));
        assert_eq!(
            r2.cache_hit_count, 3,
            "second pass with identical source must hit cache for all 3 decls"
        );
        assert_eq!(r2.recheck_count, 0);
    }

    /// Modifying the middle declaration (`b`) should cause `b` to be
    /// re-checked; tail-invalidation also forces `c` to be re-checked.
    /// Only `a` (unchanged, before the first change) is a cache hit.
    #[test]
    fn test_incremental_edit_middle_decl() {
        let src1 = "theorem a : True := True.intro\n\
                    theorem b : True := True.intro\n\
                    theorem c : True := True.intro";
        // Change `b` to use `by exact True.intro`
        let src2 = "theorem a : True := True.intro\n\
                    theorem b : True := by exact True.intro\n\
                    theorem c : True := True.intro";

        let r1 = incremental_check(src1, None);
        let r2 = incremental_check(src2, Some(r1.cache));

        // `a` is unchanged → cache hit
        assert_eq!(r2.cache_hit_count, 1, "only `a` should be a cache hit");
        // `b` is modified, `c` is tail-invalidated → both re-checked
        assert_eq!(r2.recheck_count, 2, "`b` and `c` should both be re-checked");
    }

    /// Appending a third declaration: the first two are cache hits, the
    /// third is a recheck (Inserted).
    #[test]
    fn test_incremental_append_decl() {
        let src1 = "theorem a : True := True.intro\n\
                    theorem b : True := True.intro";
        let src2 = "theorem a : True := True.intro\n\
                    theorem b : True := True.intro\n\
                    theorem c : True := True.intro";

        let r1 = incremental_check(src1, None);
        let r2 = incremental_check(src2, Some(r1.cache));

        assert_eq!(r2.cache_hit_count, 2, "`a` and `b` should be cache hits");
        assert_eq!(r2.recheck_count, 1, "only `c` should be re-checked");
        assert!(r2.cache.entries.contains_key("c"));
    }

    /// Deleting the middle declaration: the cache should not contain the
    /// removed declaration, and the remaining two are present (though `c`
    /// may be re-checked due to tail-invalidation caused by the deletion).
    #[test]
    fn test_incremental_delete_decl() {
        let src1 = "theorem a : True := True.intro\n\
                    theorem b : True := True.intro\n\
                    theorem c : True := True.intro";
        // Remove `b`
        let src2 = "theorem a : True := True.intro\n\
                    theorem c : True := True.intro";

        let r1 = incremental_check(src1, None);
        let r2 = incremental_check(src2, Some(r1.cache));

        // `b` must be gone from the cache
        assert!(
            !r2.cache.entries.contains_key("b"),
            "`b` should have been removed from the cache"
        );
        // `a` and `c` must still be present
        assert!(r2.cache.entries.contains_key("a"));
        assert!(r2.cache.entries.contains_key("c"));
        // No orphaned entries
        assert_eq!(r2.cache.entries.len(), 2);
    }
}
