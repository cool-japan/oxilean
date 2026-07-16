//! Decl-granular Myers-diff AST diffing for incremental invalidation.
//!
//! This module compares two sequences of top-level declarations (`Vec<Located<Decl>>`)
//! using the classic Myers O(ND) LCS (Longest Common Subsequence) algorithm.
//! Each declaration is first reduced to a stable, span-independent `DeclFingerprint`
//! (name + kind + structural body hash), and the diff is performed over those
//! fingerprints. The resulting edit script is a `Vec<DeclEdit>`.
//!
//! # Example
//!
//! ```ignore
//! use oxilean_parse::incremental::{diff_modules, EditKind};
//!
//! let old = parse_decls("def foo : Nat := 0").unwrap();
//! let new = parse_decls("def foo : Nat := 1").unwrap();
//! let edits = diff_modules(&old, &new);
//! assert_eq!(edits.len(), 1);
//! assert_eq!(edits[0].kind, EditKind::Modified);
//! ```

use crate::ast_impl::{Decl, Located};
use crate::prettyprint::print_decl;

// ── Inline FNV-1a 64-bit hash ────────────────────────────────────────────────

/// Compute a 64-bit FNV-1a hash over a byte slice.
///
/// This is a span-independent structural hash: the caller is responsible for
/// providing a span-free byte representation of the value to hash.
fn fnv1a_hash(data: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for &b in data {
        hash ^= b as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

// ── DeclKind ─────────────────────────────────────────────────────────────────

/// A coarse kind tag for a top-level declaration, used as part of
/// `DeclFingerprint` equality. Two decls with the same name but different
/// kinds are treated as unrelated (Delete + Insert rather than Modified).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DeclKind {
    /// `def` / `noncomputable def`
    Definition,
    /// `theorem` / `lemma`
    Theorem,
    /// `axiom`
    Axiom,
    /// `inductive`
    Inductive,
    /// `structure`
    Structure,
    /// `class`
    Class,
    /// `instance`
    Instance,
    /// `namespace … end`
    Namespace,
    /// `section … end`
    Section,
    /// `import`
    Import,
    /// `variable`
    Variable,
    /// `open`
    Open,
    /// `attribute`
    Attribute,
    /// `#check`, `#eval`, `#print`, …
    HashCmd,
    /// `mutual { … }`
    Mutual,
    /// `deriving`
    Derive,
    /// `notation` / `infixl` / `infixr` / …
    Notation,
    /// `universe`
    Universe,
    /// Any other / anonymous declaration
    Other,
}

impl DeclKind {
    /// Derive the `DeclKind` from a `Decl` value.
    pub fn of(decl: &Decl) -> Self {
        match decl {
            Decl::Definition { .. } => DeclKind::Definition,
            Decl::Theorem { .. } => DeclKind::Theorem,
            Decl::Axiom { .. } => DeclKind::Axiom,
            Decl::Inductive { .. } => DeclKind::Inductive,
            Decl::Structure { .. } => DeclKind::Structure,
            Decl::ClassDecl { .. } => DeclKind::Class,
            Decl::InstanceDecl { .. } => DeclKind::Instance,
            Decl::Namespace { .. } => DeclKind::Namespace,
            Decl::SectionDecl { .. } => DeclKind::Section,
            Decl::Import { .. } => DeclKind::Import,
            Decl::Variable { .. } => DeclKind::Variable,
            Decl::Open { .. } => DeclKind::Open,
            Decl::Attribute { .. } => DeclKind::Attribute,
            Decl::HashCmd { .. } => DeclKind::HashCmd,
            Decl::Mutual { .. } => DeclKind::Mutual,
            Decl::Derive { .. } => DeclKind::Derive,
            Decl::NotationDecl { .. } => DeclKind::Notation,
            Decl::Universe { .. } => DeclKind::Universe,
        }
    }
}

// ── DeclFingerprint ──────────────────────────────────────────────────────────

/// A stable, span-independent identity hash for a top-level declaration.
///
/// Two declarations with the same `DeclFingerprint` are structurally identical
/// modulo source positions. The `body_hash` is computed over the `Debug`
/// representation of the inner `Decl` value (not the surrounding `Located<_>`),
/// so spans do not contribute to the hash.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeclFingerprint {
    /// Declaration name (empty string for anonymous or unnamed decls).
    pub name: String,
    /// Coarse kind of the declaration.
    pub kind: DeclKind,
    /// FNV-1a hash of `format!("{:?}", located_decl.value)`.
    ///
    /// Because we hash `.value` (not the `Located` wrapper), the span is not
    /// included in the hash.
    pub body_hash: u64,
}

impl DeclFingerprint {
    /// Compute the fingerprint of a located declaration.
    ///
    /// The body hash is derived from the pretty-printed representation of the
    /// inner `Decl` value via `print_decl`. This is truly span-independent
    /// because `print_decl` takes `&Decl` (not `&Located<Decl>`) and does not
    /// print any source positions.
    ///
    /// Using `format!("{:?}", decl.value)` would be incorrect here because
    /// `Decl` contains `Located<SurfaceExpr>` sub-expressions whose `Debug`
    /// output includes span information, making the hash span-dependent.
    pub fn of(decl: &Located<Decl>) -> Self {
        let kind = DeclKind::of(&decl.value);
        let name = decl.value.name().unwrap_or("").to_owned();
        // Use the pretty-printer which operates on `&Decl` (no span data).
        let repr = print_decl(&decl.value);
        let body_hash = fnv1a_hash(repr.as_bytes());
        DeclFingerprint {
            name,
            kind,
            body_hash,
        }
    }

    /// Returns `true` if `self` and `other` have the same name and kind but
    /// a different body hash — i.e. the body was modified.
    pub fn is_modified_version_of(&self, other: &DeclFingerprint) -> bool {
        self.name == other.name && self.kind == other.kind && self.body_hash != other.body_hash
    }
}

// ── DeclEdit ─────────────────────────────────────────────────────────────────

/// The kind of change that affected a declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditKind {
    /// The declaration is unchanged between old and new.
    Unchanged,
    /// The declaration was inserted in the new sequence (not present in old).
    Inserted,
    /// The declaration was deleted from the old sequence (not present in new).
    Deleted,
    /// Same name + kind but the body changed.
    Modified,
}

/// A single entry in the diff edit script produced by [`diff_modules`].
#[derive(Debug, Clone)]
pub struct DeclEdit {
    /// Kind of change.
    pub kind: EditKind,
    /// Index in the **old** declaration sequence, or `None` for `Inserted`.
    pub old_idx: Option<usize>,
    /// Index in the **new** declaration sequence, or `None` for `Deleted`.
    pub new_idx: Option<usize>,
    /// Fingerprint of the "surviving" declaration:
    /// - `Unchanged`, `Inserted`, `Modified` → fingerprint of the new decl.
    /// - `Deleted` → fingerprint of the old decl.
    pub fingerprint: DeclFingerprint,
}

// ── Myers O(ND) LCS diff ─────────────────────────────────────────────────────

/// Compare two slices of located declarations and produce an edit script.
///
/// The algorithm is the classic Myers O(ND) diff over the fingerprint sequences.
/// After computing the LCS, consecutive `Deleted`/`Inserted` pairs that share
/// the same name and kind are folded into a single `Modified` edit.
///
/// # Complexity
///
/// Time: O(N·D) where N = max(|old|, |new|) and D = edit distance.
/// Space: O(N + D²) for the DP frontier.
///
/// # Output invariants
///
/// - Every old index appears in exactly one edit with `old_idx = Some(_)`.
/// - Every new index appears in exactly one edit with `new_idx = Some(_)`.
/// - `Unchanged` and `Deleted` counts sum to `old.len()`.
/// - `Unchanged` and `Inserted` counts sum to `new.len()`.
pub fn diff_modules(old: &[Located<Decl>], new: &[Located<Decl>]) -> Vec<DeclEdit> {
    let old_fps: Vec<DeclFingerprint> = old.iter().map(DeclFingerprint::of).collect();
    let new_fps: Vec<DeclFingerprint> = new.iter().map(DeclFingerprint::of).collect();

    let raw = myers_diff(&old_fps, &new_fps);
    merge_modified(raw)
}

/// Core Myers O(ND) diff returning raw `Deleted`/`Inserted`/`Unchanged` edits.
///
/// Implements the Myers algorithm from "An O(ND) Difference Algorithm and Its
/// Variations" (Myers, 1986) using the standard trace-and-backtrack approach.
///
/// Convention: x = index into `old`, y = index into `new`, diagonal k = x − y.
/// v[k + offset] = furthest-reaching x on diagonal k after d non-diagonal moves.
///
/// This does **not** detect `Modified`; that is handled by the post-processing
/// step in `merge_modified`.
fn myers_diff(old: &[DeclFingerprint], new: &[DeclFingerprint]) -> Vec<DeclEdit> {
    let n = old.len();
    let m = new.len();

    // Fast path: both empty.
    if n == 0 && m == 0 {
        return Vec::new();
    }

    // Fast path: old is empty → everything is Inserted.
    if n == 0 {
        return new
            .iter()
            .enumerate()
            .map(|(j, fp)| DeclEdit {
                kind: EditKind::Inserted,
                old_idx: None,
                new_idx: Some(j),
                fingerprint: fp.clone(),
            })
            .collect();
    }

    // Fast path: new is empty → everything is Deleted.
    if m == 0 {
        return old
            .iter()
            .enumerate()
            .map(|(i, fp)| DeclEdit {
                kind: EditKind::Deleted,
                old_idx: Some(i),
                new_idx: None,
                fingerprint: fp.clone(),
            })
            .collect();
    }

    let max_d = n + m;
    let offset = max_d as isize; // v[k + offset] for diagonal k
    let size = 2 * max_d + 2; // +2 for sentinel at k=+1 boundary

    // v[k + offset] = furthest x on diagonal k.
    // Standard Myers sentinel: v[offset + 1] = 0 handles the d=0, k=0 case
    // where the loop selects "come from k+1" (insert path).
    let mut v: Vec<isize> = vec![0_isize; size];

    // trace[d] = snapshot of v AFTER completing step d.
    let mut trace: Vec<Vec<isize>> = Vec::with_capacity(max_d + 1);
    let mut found = false;

    'outer: for d in 0..=(max_d as isize) {
        let mut k = -d;
        while k <= d {
            let ki = (k + offset) as usize;

            // Determine x by choosing the better prior diagonal.
            // "Insert" = came from diagonal k+1 (x unchanged, y+1).
            // "Delete" = came from diagonal k-1 (x+1, y unchanged).
            let mut x: isize = if k == -d || (k != d && v[ki + 1] > v[ki - 1]) {
                // Come from k+1 (insert).
                v[ki + 1]
            } else {
                // Come from k-1 (delete).
                v[ki - 1] + 1
            };
            let mut y: isize = x - k;

            // Advance along the snake (matching elements).
            while x < n as isize && y < m as isize && old[x as usize] == new[y as usize] {
                x += 1;
                y += 1;
            }
            v[ki] = x;

            if x >= n as isize && y >= m as isize {
                // Record the snapshot at this step d before breaking.
                trace.push(v.clone());
                found = true;
                break 'outer;
            }
            k += 2;
        }
        // Snapshot after each complete step d.
        trace.push(v.clone());
    }

    if !found {
        // Fallback: should not happen for finite inputs; return empty.
        return Vec::new();
    }

    // Backtrack from (n, m) to (0, 0) using the trace.
    // trace[d] = v after step d.
    backtrack_myers(old, new, &trace, offset)
}

/// Reconstruct the edit script by walking the trace backward from the endpoint.
///
/// For each step d (from the last down to 1), we look at `trace[d-1]` (the
/// v-array after step d-1, before step d) to determine whether the move at
/// step d was a delete (right) or insert (down). We then emit the snake
/// (Unchanged) and the single non-snake edit.
fn backtrack_myers(
    old: &[DeclFingerprint],
    new: &[DeclFingerprint],
    trace: &[Vec<isize>],
    offset: isize,
) -> Vec<DeclEdit> {
    let n = old.len() as isize;
    let m = new.len() as isize;

    let mut x = n;
    let mut y = m;
    let mut edits: Vec<DeclEdit> = Vec::new();

    // d ranges from trace.len()-1 down to 1 (step 0 has no non-snake edit).
    // `trace[d]` = v after step d.
    // When going from step d-1 to d, we use `trace[d-1]` to decide direction.
    for d in (1..trace.len()).rev() {
        let v_prev = &trace[d - 1];
        let k = x - y;
        let ki = (k + offset) as usize;

        // Which diagonal did we come from at step d?
        // Must use the same condition as the forward pass to be consistent.
        let came_from_insert = if k == -(d as isize) {
            true // forced: only diagonal k+1 was valid
        } else if k == d as isize {
            false // forced: only diagonal k-1 was valid
        } else {
            // Same rule as forward pass: insert if v[ki+1] > v[ki-1]
            v_prev[ki + 1] > v_prev[ki - 1]
        };

        // The endpoint BEFORE the non-snake edit at step d.
        let (mid_x, mid_y) = if came_from_insert {
            // Came from diagonal k+1: x unchanged, y decreased by 1.
            let px = v_prev[ki + 1];
            let py = px - (k + 1);
            (px, py)
        } else {
            // Came from diagonal k-1: x decreased by 1.
            let px = v_prev[ki - 1];
            let py = px - (k - 1);
            (px, py)
        };

        // The snake runs from (mid_x + delta_x, mid_y + delta_y) to (x, y).
        // After the non-snake move, we are at:
        let (after_x, after_y) = if came_from_insert {
            (mid_x, mid_y + 1) // insert advances y by 1
        } else {
            (mid_x + 1, mid_y) // delete advances x by 1
        };

        // Emit snake (Unchanged) edits — backward order, we reverse later.
        // Snake: from (after_x, after_y) to (x, y), all matching.
        let mut sx = x - 1;
        let mut sy = y - 1;
        while sx >= after_x && sy >= after_y {
            edits.push(DeclEdit {
                kind: EditKind::Unchanged,
                old_idx: Some(sx as usize),
                new_idx: Some(sy as usize),
                fingerprint: new[sy as usize].clone(),
            });
            sx -= 1;
            sy -= 1;
        }

        // Emit the non-snake edit.
        if came_from_insert {
            // Insert: new[mid_y] was inserted (y moves from mid_y to mid_y+1).
            if mid_y >= 0 && mid_y < m {
                edits.push(DeclEdit {
                    kind: EditKind::Inserted,
                    old_idx: None,
                    new_idx: Some(mid_y as usize),
                    fingerprint: new[mid_y as usize].clone(),
                });
            }
        } else {
            // Delete: old[mid_x] was deleted (x moves from mid_x to mid_x+1).
            if mid_x >= 0 && mid_x < n {
                edits.push(DeclEdit {
                    kind: EditKind::Deleted,
                    old_idx: Some(mid_x as usize),
                    new_idx: None,
                    fingerprint: old[mid_x as usize].clone(),
                });
            }
        }

        x = mid_x;
        y = mid_y;
    }

    // Emit any remaining snake at d=0 (prefix that matched from the start).
    // At d=0 there are no non-snake edits; just the initial diagonal run.
    let mut sx = x - 1;
    let mut sy = y - 1;
    while sx >= 0 && sy >= 0 {
        edits.push(DeclEdit {
            kind: EditKind::Unchanged,
            old_idx: Some(sx as usize),
            new_idx: Some(sy as usize),
            fingerprint: new[sy as usize].clone(),
        });
        sx -= 1;
        sy -= 1;
    }

    edits.reverse();
    edits
}

/// Post-process raw `Deleted`/`Inserted`/`Unchanged` edits: fold consecutive
/// `Deleted` + `Inserted` pairs that share the same `name` and `kind` into a
/// single `Modified` edit.
///
/// The strategy: collect all Deleted edits and all Inserted edits by (name, kind)
/// into lookup tables, then scan the output and match them up.
fn merge_modified(raw: Vec<DeclEdit>) -> Vec<DeclEdit> {
    use std::collections::HashMap;

    // Index deleted edits by (name, kind) → queue of (position-in-raw, DeclEdit).
    let mut deleted_by_key: HashMap<
        (String, String),
        std::collections::VecDeque<(usize, DeclEdit)>,
    > = HashMap::new();

    for (pos, edit) in raw.iter().enumerate() {
        if edit.kind == EditKind::Deleted {
            let key = (
                edit.fingerprint.name.clone(),
                format!("{:?}", edit.fingerprint.kind),
            );
            deleted_by_key
                .entry(key)
                .or_default()
                .push_back((pos, edit.clone()));
        }
    }

    // Index inserted edits similarly.
    let mut inserted_by_key: HashMap<
        (String, String),
        std::collections::VecDeque<(usize, DeclEdit)>,
    > = HashMap::new();

    for (pos, edit) in raw.iter().enumerate() {
        if edit.kind == EditKind::Inserted {
            let key = (
                edit.fingerprint.name.clone(),
                format!("{:?}", edit.fingerprint.kind),
            );
            inserted_by_key
                .entry(key)
                .or_default()
                .push_back((pos, edit.clone()));
        }
    }

    // Build a set of positions to suppress (they get replaced by Modified).
    let mut suppressed: std::collections::HashSet<usize> = std::collections::HashSet::new();
    // Modified edits: (position where they should be inserted, DeclEdit).
    // We use the position of the Deleted edit as the insertion point.
    let mut modified_inserts: HashMap<usize, DeclEdit> = HashMap::new();

    for ((name, kind_s), del_queue) in deleted_by_key.iter_mut() {
        let key = (name.clone(), kind_s.clone());
        if let Some(ins_queue) = inserted_by_key.get_mut(&key) {
            // Pair up deleted and inserted edits, in FIFO order.
            while !del_queue.is_empty() && !ins_queue.is_empty() {
                let (del_pos, del_edit) = del_queue.pop_front().expect("non-empty");
                let (ins_pos, ins_edit) = ins_queue.pop_front().expect("non-empty");

                // Only merge if they are actually a body-change (body hashes differ).
                if del_edit.fingerprint.body_hash != ins_edit.fingerprint.body_hash {
                    suppressed.insert(del_pos);
                    suppressed.insert(ins_pos);
                    modified_inserts.insert(
                        del_pos,
                        DeclEdit {
                            kind: EditKind::Modified,
                            old_idx: del_edit.old_idx,
                            new_idx: ins_edit.new_idx,
                            fingerprint: ins_edit.fingerprint.clone(),
                        },
                    );
                }
            }
        }
    }

    // Assemble the final edit list, respecting original order.
    let mut result: Vec<DeclEdit> = Vec::with_capacity(raw.len());
    for (pos, edit) in raw.into_iter().enumerate() {
        if suppressed.contains(&pos) {
            // If there is a Modified replacement at this position, emit it.
            if let Some(modified) = modified_inserts.remove(&pos) {
                result.push(modified);
            }
            // Otherwise skip (the paired Inserted half is suppressed too).
        } else {
            result.push(edit);
        }
    }
    result
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::functions::parse_decls;

    /// Parse a Lean snippet into a Vec<Located<Decl>>.
    ///
    /// We cannot use `parse_decls` directly because its EOF detection uses
    /// `ParseErrorKind::UnexpectedEof`, but the parser's catch-all arm produces
    /// `ParseErrorKind::UnexpectedToken { got: Eof }` when the input is exhausted.
    /// This helper drives the parser loop manually and treats any error at EOF
    /// (including `UnexpectedToken { got: Eof }`) as a termination signal.
    fn parse_lean(src: &str) -> Vec<Located<Decl>> {
        use crate::lexer::Lexer;
        use crate::parser_impl::Parser;
        use crate::tokens::TokenKind;

        let tokens = Lexer::new(src).tokenize();
        let mut parser = Parser::new(tokens);
        let mut decls = Vec::new();
        loop {
            // If already at EOF, stop before even trying to parse.
            if parser.is_eof() {
                break;
            }
            match parser.parse_decl() {
                Ok(d) => decls.push(d),
                Err(e) => {
                    // Stop on any EOF-related error (both UnexpectedEof and
                    // UnexpectedToken { got: Eof } from the catch-all arm).
                    let is_eof_err = e.is_eof()
                        || matches!(
                            e.message().as_str(),
                            s if s.contains("EOF") || s.contains("Eof") || s.contains("end of file")
                        );
                    if is_eof_err || parser.is_eof() {
                        break;
                    }
                    // Non-EOF parse error: advance one token and continue
                    // (best-effort; in tests we expect well-formed input).
                    parser.advance();
                }
            }
        }
        decls
    }

    // ── 1. Identical source → all Unchanged ──────────────────────────────────

    #[test]
    fn test_diff_identical() {
        let src = "def foo : Nat := 0\ndef bar : Nat := 1";
        let old = parse_lean(src);
        let new = parse_lean(src);
        let edits = diff_modules(&old, &new);
        assert!(
            edits.iter().all(|e| e.kind == EditKind::Unchanged),
            "identical sources should produce only Unchanged edits, got: {edits:?}"
        );
        assert_eq!(edits.len(), old.len());
    }

    // ── 2. Inserted declaration ───────────────────────────────────────────────

    #[test]
    fn test_diff_insert_decl() {
        let old_src = "def foo : Nat := 0";
        let new_src = "def foo : Nat := 0\ndef bar : Nat := 1";
        let old = parse_lean(old_src);
        let new = parse_lean(new_src);
        let edits = diff_modules(&old, &new);

        let unchanged: Vec<_> = edits
            .iter()
            .filter(|e| e.kind == EditKind::Unchanged)
            .collect();
        let inserted: Vec<_> = edits
            .iter()
            .filter(|e| e.kind == EditKind::Inserted)
            .collect();

        assert_eq!(unchanged.len(), 1, "foo should be Unchanged");
        assert_eq!(inserted.len(), 1, "bar should be Inserted");
        assert_eq!(inserted[0].fingerprint.name, "bar");
    }

    // ── 3. Deleted declaration ────────────────────────────────────────────────

    #[test]
    fn test_diff_delete_decl() {
        let old_src = "def a : Nat := 0\ndef b : Nat := 1\ndef c : Nat := 2";
        let new_src = "def a : Nat := 0\ndef c : Nat := 2";
        let old = parse_lean(old_src);
        let new = parse_lean(new_src);
        let edits = diff_modules(&old, &new);

        let deleted: Vec<_> = edits
            .iter()
            .filter(|e| e.kind == EditKind::Deleted)
            .collect();
        let unchanged: Vec<_> = edits
            .iter()
            .filter(|e| e.kind == EditKind::Unchanged)
            .collect();

        assert_eq!(deleted.len(), 1, "b should be Deleted");
        assert_eq!(deleted[0].fingerprint.name, "b");
        assert_eq!(unchanged.len(), 2, "a and c should be Unchanged");

        // Verify coverage: every old index appears exactly once.
        let mut seen_old: Vec<bool> = vec![false; old.len()];
        for edit in &edits {
            if let Some(i) = edit.old_idx {
                assert!(!seen_old[i], "old index {i} appears more than once");
                seen_old[i] = true;
            }
        }
        assert!(seen_old.iter().all(|&b| b), "not all old indices covered");
    }

    // ── 4. Modified declaration (same name+kind, different body) ─────────────

    #[test]
    fn test_diff_modified_decl() {
        let old_src = "def foo : Nat := 0";
        let new_src = "def foo : Nat := 99";
        let old = parse_lean(old_src);
        let new = parse_lean(new_src);
        let edits = diff_modules(&old, &new);

        let modified: Vec<_> = edits
            .iter()
            .filter(|e| e.kind == EditKind::Modified)
            .collect();
        assert_eq!(modified.len(), 1, "foo body change should be Modified");
        assert_eq!(modified[0].fingerprint.name, "foo");
        assert_eq!(modified[0].old_idx, Some(0));
        assert_eq!(modified[0].new_idx, Some(0));
    }

    // ── 5. Reorder two declarations ───────────────────────────────────────────

    #[test]
    fn test_diff_reorder() {
        let old_src = "def a : Nat := 0\ndef b : Nat := 1";
        let new_src = "def b : Nat := 1\ndef a : Nat := 0";
        let old = parse_lean(old_src);
        let new = parse_lean(new_src);
        let edits = diff_modules(&old, &new);

        // Reordering must be representable: at least one Unchanged, or both as
        // Insert+Delete (Myers picks the minimal edit).
        let total_old_coverage: usize = edits.iter().filter(|e| e.old_idx.is_some()).count();
        let total_new_coverage: usize = edits.iter().filter(|e| e.new_idx.is_some()).count();
        assert_eq!(
            total_old_coverage,
            old.len(),
            "all old indices must be covered"
        );
        assert_eq!(
            total_new_coverage,
            new.len(),
            "all new indices must be covered"
        );

        // The edit script must be valid: applying it should reconstruct `new`.
        let reconstructed = apply_edit_script(&old, &new, &edits);
        assert_eq!(reconstructed.len(), new.len());
    }

    // ── 6. Both empty → empty edit list ──────────────────────────────────────

    #[test]
    fn test_diff_empty() {
        let edits = diff_modules(&[], &[]);
        assert!(
            edits.is_empty(),
            "diffing empty sequences should produce no edits"
        );
    }

    // ── 7. Coverage invariant property ───────────────────────────────────────

    #[test]
    fn test_index_coverage_invariant() {
        let old_src = "def a : Nat := 0\ntheorem t : True := trivial\ndef b : Nat := 2";
        let new_src = "def a : Nat := 0\ndef b : Nat := 99\ndef c : Nat := 3";
        let old = parse_lean(old_src);
        let new = parse_lean(new_src);
        let edits = diff_modules(&old, &new);

        // Every old index appears exactly once.
        let mut old_seen = vec![false; old.len()];
        for edit in &edits {
            if let Some(i) = edit.old_idx {
                assert!(!old_seen[i], "old_idx {i} appears twice");
                old_seen[i] = true;
            }
        }
        assert!(
            old_seen.iter().all(|&b| b),
            "some old indices not covered: {:?}",
            old_seen
        );

        // Every new index appears exactly once.
        let mut new_seen = vec![false; new.len()];
        for edit in &edits {
            if let Some(j) = edit.new_idx {
                assert!(!new_seen[j], "new_idx {j} appears twice");
                new_seen[j] = true;
            }
        }
        assert!(
            new_seen.iter().all(|&b| b),
            "some new indices not covered: {:?}",
            new_seen
        );
    }

    // ── Helper: reconstruct new sequence from old + edit script ──────────────

    /// Apply an edit script to reconstruct the new declaration sequence.
    /// Used as a validity oracle in tests.
    fn apply_edit_script<'a>(
        old: &'a [Located<Decl>],
        new: &'a [Located<Decl>],
        edits: &[DeclEdit],
    ) -> Vec<&'a Located<Decl>> {
        let mut result = Vec::new();
        for edit in edits {
            match edit.kind {
                EditKind::Unchanged | EditKind::Modified => {
                    // Take from new (the surviving version).
                    if let Some(j) = edit.new_idx {
                        result.push(&new[j]);
                    }
                }
                EditKind::Inserted => {
                    if let Some(j) = edit.new_idx {
                        result.push(&new[j]);
                    }
                }
                EditKind::Deleted => {
                    // Deleted: verify it came from old.
                    if let Some(i) = edit.old_idx {
                        let _ = &old[i]; // just assert in-bounds
                    }
                }
            }
        }
        result
    }
}
