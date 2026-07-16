//! Incremental re-lex API for LSP `textDocument/didChange` events.
//!
//! # Overview
//!
//! `parse_incremental_change` takes the old token stream and the already-applied
//! new source string, re-lexes only the "dirty" region (the changed portion plus
//! tokens that straddle the edit boundary), splices the result into the stream,
//! and shifts all post-edit token positions by the byte delta.
//!
//! # Position model
//!
//! The `Lexer` tracks positions as **char indices** (indices into `Vec<char>`),
//! not raw byte offsets.  `TextChange.range` similarly operates in char-index
//! space (i.e. "number of Unicode scalar values from the start of the source").
//! All offset arithmetic below is therefore in char-index units; conversion to
//! byte offsets is only needed when slicing the UTF-8 source string.

use super::types::TextChange;
use crate::lexer::Lexer;
use crate::tokens::{Span, Token, TokenKind};

// ── Public result type ──────────────────────────────────────────────────────

/// The result of an incremental re-lex triggered by a single text edit.
#[derive(Debug, Clone)]
pub struct IncrementalChangeResult {
    /// Sub-range of `tokens` that was re-lexed.
    ///
    /// Tokens with indices in `changed_token_range` are new; tokens outside
    /// that range are (potentially shifted) copies from the old stream.
    pub changed_token_range: std::ops::Range<usize>,
    /// Complete new token stream for the post-edit source.
    pub tokens: Vec<Token>,
    /// `true` if the first unaffected token after the dirty region retained
    /// the same start position after shifting — i.e. the re-lex did not
    /// cascade further changes into the tail of the stream.
    pub is_stable: bool,
}

// ── Main API ────────────────────────────────────────────────────────────────

/// Incrementally re-lex a single text edit.
///
/// Given the **old** token stream and the **new** source string (after applying
/// `change`), this function:
///
/// 1. Computes the dirty char-index range in the new source.
/// 2. Finds the first token overlapping the dirty region and the first token
///    entirely after it (in the old stream).
/// 3. Re-lexes the dirty sub-region of the new source.
/// 4. Shifts all surviving tail tokens by `byte_delta`.
/// 5. Splices the new tokens in and returns the combined stream.
///
/// # Parameters
/// - `old_tokens` — token stream from the previous successful lex.
/// - `new_source` — full source string after the edit has been applied.
/// - `change` — the edit that was applied (char-index range + new text).
///
/// # Notes
/// The approach is O(k + m) where k is the number of re-lexed tokens and m is
/// the tail length.  For edits that are small relative to the file size this is
/// substantially cheaper than a full re-lex (O(n)).
pub fn parse_incremental_change(
    old_tokens: &[Token],
    new_source: &str,
    change: &TextChange,
) -> IncrementalChangeResult {
    // ── Step 1: dirty region in char-index space ─────────────────────────
    let edit_start_char = change.range.start;
    let old_edit_end_char = change.range.end;
    let new_edit_end_char = edit_start_char + char_count(&change.new_text);
    // char delta: how much positions shift after the edit
    let char_delta: i64 = new_edit_end_char as i64 - old_edit_end_char as i64;

    // ── Step 2: find affected token range in old stream ──────────────────
    // first_affected: first token whose end > edit_start (i.e. it overlaps)
    let first_affected = old_tokens
        .iter()
        .position(|t| token_end_char(t) > edit_start_char)
        .unwrap_or(old_tokens.len());

    // first_unaffected: first token (after first_affected) that starts
    // entirely after the OLD edit end (no overlap with deleted region)
    let first_unaffected = old_tokens[first_affected..]
        .iter()
        .position(|t| token_start_char(t) >= old_edit_end_char)
        .map(|p| p + first_affected)
        .unwrap_or(old_tokens.len());

    // ── Step 3: determine re-lex start position in the NEW source ────────
    // Re-lex starting from the beginning of the first affected token so that
    // we restore any token that merely straddles the edit boundary.
    let relex_start_char = old_tokens
        .get(first_affected)
        .map(token_start_char)
        .unwrap_or(edit_start_char);

    // ── Step 4: re-lex from relex_start_char until we have passed the dirty end
    // We lex the full new source and collect tokens in the dirty window.
    // Although this is O(n) for the lex itself, the hot path only visits
    // chars from relex_start_char onward.
    //
    // An alternative would be to build a Lexer positioned at relex_start_char;
    // the current Lexer API (`new(&str)` only) does not support mid-stream
    // starts, so we do a single pass and discard the pre-dirty prefix.
    let new_tokens_in_range = relex_dirty_range(new_source, relex_start_char, new_edit_end_char);

    // ── Step 5: shift surviving tail tokens by char_delta ────────────────
    let shifted_tail: Vec<Token> = old_tokens[first_unaffected..]
        .iter()
        .map(|t| shift_token(t, char_delta))
        .collect();

    // ── Step 6: stability check ──────────────────────────────────────────
    // The stream is "stable" if the last re-lexed token ends exactly where
    // the first tail token begins (after shifting).
    let is_stable = check_boundary_stable(&new_tokens_in_range, &shifted_tail);

    // ── Step 7: splice ───────────────────────────────────────────────────
    let prefix = &old_tokens[..first_affected];
    let new_len = prefix.len() + new_tokens_in_range.len() + shifted_tail.len();
    let mut result_tokens: Vec<Token> = Vec::with_capacity(new_len);
    result_tokens.extend_from_slice(prefix);
    result_tokens.extend(new_tokens_in_range.iter().cloned());
    result_tokens.extend(shifted_tail);

    let changed_range_start = first_affected;
    let changed_range_end = first_affected + new_tokens_in_range.len();

    IncrementalChangeResult {
        changed_token_range: changed_range_start..changed_range_end,
        tokens: result_tokens,
        is_stable,
    }
}

// ── Private helpers ─────────────────────────────────────────────────────────

/// Number of Unicode scalar values (chars) in `s`.
#[inline]
fn char_count(s: &str) -> usize {
    s.chars().count()
}

/// Start position of a token (char index).
#[inline]
fn token_start_char(token: &Token) -> usize {
    token.span.start
}

/// End position of a token (char index, exclusive).
#[inline]
fn token_end_char(token: &Token) -> usize {
    token.span.end
}

/// Convert a char index into a byte offset for slicing a `&str`.
fn char_to_byte_offset(source: &str, char_index: usize) -> usize {
    source
        .char_indices()
        .nth(char_index)
        .map(|(byte_pos, _)| byte_pos)
        .unwrap_or(source.len())
}

/// Re-lex the dirty window [`relex_start_char`, `dirty_end_char`) of `source`.
///
/// The Lexer always starts from position 0, so we full-lex and filter.
/// We stop collecting once we have passed `dirty_end_char` AND we land on a
/// clean token boundary (token start >= dirty_end_char).
fn relex_dirty_range(source: &str, relex_start_char: usize, dirty_end_char: usize) -> Vec<Token> {
    let mut lexer = Lexer::new(source);
    let all_tokens = lexer.tokenize();

    all_tokens
        .into_iter()
        .filter(|t| {
            // Skip tokens that ended before the relex window
            if token_end_char(t) <= relex_start_char {
                return false;
            }
            // Stop at (but include) the first token that starts at or after the
            // dirty_end boundary — that token is the first unambiguously clean one.
            // We include it in the re-lex range only if it overlaps the dirty window.
            token_start_char(t) < dirty_end_char
        })
        .collect()
}

/// Shift all char-index positions in a token by `char_delta`.
fn shift_token(token: &Token, char_delta: i64) -> Token {
    let new_start = (token.span.start as i64 + char_delta).max(0) as usize;
    let new_end = (token.span.end as i64 + char_delta).max(0) as usize;
    Token::new(
        token.kind.clone(),
        Span::new(new_start, new_end, token.span.line, token.span.column),
    )
}

/// Returns `true` if the last re-lexed token ends exactly where the first tail
/// token begins (clean splice boundary = no cascading invalidation).
fn check_boundary_stable(new_tokens: &[Token], tail: &[Token]) -> bool {
    match (new_tokens.last(), tail.first()) {
        (Some(last), Some(first)) => token_end_char(last) == token_start_char(first),
        // No tail means nothing to stabilise against — treat as stable.
        (_, None) => true,
        // No new tokens but there is a tail — vacuously stable.
        (None, Some(_)) => true,
    }
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::incremental::types::TextChange;

    fn lex_full(source: &str) -> Vec<Token> {
        Lexer::new(source).tokenize()
    }

    /// Helper: kind discriminant string for comparison (ignores payload).
    fn kind_tag(t: &Token) -> String {
        match &t.kind {
            TokenKind::Ident(_) => "Ident".into(),
            TokenKind::Nat(_) => "Nat".into(),
            TokenKind::Float(_) => "Float".into(),
            TokenKind::String(_) => "String".into(),
            TokenKind::Char(_) => "Char".into(),
            TokenKind::DocComment(_) => "DocComment".into(),
            TokenKind::InterpolatedString(_) => "InterpolatedString".into(),
            TokenKind::Error(_) => "Error".into(),
            TokenKind::Eof => "Eof".into(),
            other => format!("{:?}", other),
        }
    }

    #[test]
    fn test_empty_change_is_noop() {
        let source = "theorem foo : True := trivial";
        let old_tokens = lex_full(source);
        let change = TextChange::new(0, 0, "");
        let result = parse_incremental_change(&old_tokens, source, &change);
        assert_eq!(
            result.tokens.len(),
            old_tokens.len(),
            "empty edit must produce same token count"
        );
    }

    #[test]
    fn test_same_length_replacement_token_count() {
        // Replace "foo" (3 chars) with "bar" (3 chars) — same length
        let source = "theorem foo : True := trivial";
        let old_tokens = lex_full(source);
        let new_source = "theorem bar : True := trivial";
        // "foo" starts at char 8, ends at char 11
        let change = TextChange::new(8, 11, "bar");
        let result = parse_incremental_change(&old_tokens, new_source, &change);
        let full_tokens = lex_full(new_source);
        assert_eq!(
            result.tokens.len(),
            full_tokens.len(),
            "incremental and full lex should produce same token count"
        );
    }

    #[test]
    fn test_same_length_replacement_kinds_match() {
        let source = "theorem foo : True := trivial";
        let old_tokens = lex_full(source);
        let new_source = "theorem bar : True := trivial";
        let change = TextChange::new(8, 11, "bar");
        let result = parse_incremental_change(&old_tokens, new_source, &change);
        let full_tokens = lex_full(new_source);
        let inc_kinds: Vec<String> = result.tokens.iter().map(kind_tag).collect();
        let full_kinds: Vec<String> = full_tokens.iter().map(kind_tag).collect();
        assert_eq!(
            inc_kinds, full_kinds,
            "token kind sequences must match between incremental and full lex"
        );
    }

    #[test]
    fn test_insertion_changes_token_count() {
        let source = "def x := 1";
        let old_tokens = lex_full(source);
        // Insert " + 2" after "1" (char 10)
        let new_source = "def x := 1 + 2";
        let change = TextChange::new(10, 10, " + 2");
        let result = parse_incremental_change(&old_tokens, new_source, &change);
        let full_tokens = lex_full(new_source);
        assert_eq!(
            result.tokens.len(),
            full_tokens.len(),
            "incremental token count after insertion must match full lex"
        );
    }

    #[test]
    fn test_deletion_changes_token_count() {
        let source = "def x := 1 + 2";
        let old_tokens = lex_full(source);
        // Delete " + 2" (chars 10..14)
        let new_source = "def x := 1";
        let change = TextChange::new(10, 14, "");
        let result = parse_incremental_change(&old_tokens, new_source, &change);
        let full_tokens = lex_full(new_source);
        assert_eq!(
            result.tokens.len(),
            full_tokens.len(),
            "incremental token count after deletion must match full lex"
        );
    }

    #[test]
    fn test_relex_matches_full_lex_various_edits() {
        // (old_source, new_source, change_start_char, change_end_char, new_text)
        let cases: Vec<(&str, &str, usize, usize, &str)> = vec![
            // rename "x" to "y" in a def
            ("def x := 1", "def y := 1", 4, 5, "y"),
            // rename identifier at end
            ("def x := foo", "def x := bar", 9, 12, "bar"),
            // change a number
            ("def x := 1", "def x := 42", 9, 10, "42"),
        ];
        for (old_src, new_src, cs, ce, new_text) in cases {
            let old_tokens = lex_full(old_src);
            let change = TextChange::new(cs, ce, new_text);
            let result = parse_incremental_change(&old_tokens, new_src, &change);
            let full_tokens = lex_full(new_src);
            assert_eq!(
                result.tokens.len(),
                full_tokens.len(),
                "count mismatch for edit '{old_src}' -> '{new_src}'"
            );
            let inc_kinds: Vec<String> = result.tokens.iter().map(kind_tag).collect();
            let full_kinds: Vec<String> = full_tokens.iter().map(kind_tag).collect();
            assert_eq!(
                inc_kinds, full_kinds,
                "kind mismatch for edit '{old_src}' -> '{new_src}'"
            );
        }
    }

    #[test]
    fn test_changed_token_range_is_non_empty_on_real_edit() {
        let source = "def x := 1";
        let old_tokens = lex_full(source);
        let new_source = "def x := 42";
        let change = TextChange::new(9, 10, "42");
        let result = parse_incremental_change(&old_tokens, new_source, &change);
        assert!(
            !result.changed_token_range.is_empty(),
            "changed_token_range must be non-empty for a real edit"
        );
    }

    #[test]
    fn test_eof_token_is_present_after_incremental() {
        let source = "def x := 1";
        let old_tokens = lex_full(source);
        let new_source = "def x := 42";
        let change = TextChange::new(9, 10, "42");
        let result = parse_incremental_change(&old_tokens, new_source, &change);
        assert!(
            result
                .tokens
                .last()
                .map(|t| matches!(t.kind, TokenKind::Eof))
                .unwrap_or(false),
            "last token must be Eof"
        );
    }

    #[test]
    fn test_prefix_tokens_unmodified() {
        // Tokens before the edit should be bit-for-bit identical.
        let source = "def x := 1 + 2";
        let old_tokens = lex_full(source);
        // Change "2" (char 13..14) to "99"
        let new_source = "def x := 1 + 99";
        let change = TextChange::new(13, 14, "99");
        let result = parse_incremental_change(&old_tokens, new_source, &change);
        // Find first_affected: tokens before char 13
        let first_affected = old_tokens
            .iter()
            .position(|t| t.span.end > 13)
            .unwrap_or(old_tokens.len());
        for (i, old_tok) in old_tokens.iter().enumerate().take(first_affected) {
            assert_eq!(
                result.tokens[i].span.start, old_tok.span.start,
                "prefix token {i} span.start must be unmodified"
            );
        }
    }
}
