//! Document management: open documents, line offsets, DocumentStore.

use std::collections::HashMap;

use super::lsp_types::{Position, Range};

// ── Helper functions ──────────────────────────────────────────────────────────

pub(super) fn is_ident_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b == b'.' || b == b'\''
}

/// Compute the byte offsets of each line start.
pub(super) fn compute_line_offsets(text: &str) -> Vec<usize> {
    let mut offsets = vec![0];
    for (i, byte) in text.bytes().enumerate() {
        if byte == b'\n' {
            offsets.push(i + 1);
        }
    }
    offsets
}

// ── Document ──────────────────────────────────────────────────────────────────

/// An open document with computed line offsets.
#[derive(Clone, Debug)]
pub struct Document {
    /// URI of the document.
    pub uri: String,
    /// Version number.
    pub version: i64,
    /// Full text content.
    pub content: String,
    /// Byte offsets of line starts (line_offsets\[i\] = byte offset of line i).
    pub line_offsets: Vec<usize>,
    /// Cached token stream from the most recent lex pass.
    ///
    /// Updated after every full or incremental re-lex.  Empty on a freshly
    /// opened document (before the first analysis pass).
    pub cached_tokens: Vec<oxilean_parse::tokens::Token>,
    /// Cached top-level declaration AST nodes from the most recent parse pass.
    ///
    /// Used by `analyze_document_incremental_decls` to diff against the new
    /// parse result and determine which declarations need re-analysis.
    /// Cleared on a full-content replace; preserved across incremental edits
    /// (the diff function handles staleness detection).
    pub cached_decls: Vec<oxilean_parse::Located<oxilean_parse::Decl>>,
}

impl Document {
    /// Create a new document from text.
    pub fn new(uri: impl Into<String>, version: i64, content: impl Into<String>) -> Self {
        let uri = uri.into();
        let content = content.into();
        let line_offsets = compute_line_offsets(&content);
        Self {
            uri,
            version,
            content,
            line_offsets,
            cached_tokens: Vec::new(),
            cached_decls: Vec::new(),
        }
    }

    /// Update the document content, clearing both the cached token stream and
    /// the cached declaration AST nodes.
    pub fn update(&mut self, version: i64, content: impl Into<String>) {
        self.version = version;
        self.content = content.into();
        self.line_offsets = compute_line_offsets(&self.content);
        // Cached tokens and decls are stale after a full-content replace; the
        // next analysis pass will repopulate them.
        self.cached_tokens.clear();
        self.cached_decls.clear();
    }

    /// Get the text of a specific line (0-indexed).
    pub fn get_line(&self, line: u32) -> Option<&str> {
        let idx = line as usize;
        if idx >= self.line_offsets.len() {
            return None;
        }
        let start = self.line_offsets[idx];
        let end = if idx + 1 < self.line_offsets.len() {
            let e = self.line_offsets[idx + 1];
            // Strip trailing newline
            if e > 0 && self.content.as_bytes().get(e - 1) == Some(&b'\n') {
                e - 1
            } else {
                e
            }
        } else {
            self.content.len()
        };
        Some(&self.content[start..end])
    }

    /// Convert an LSP position to a byte offset.
    pub fn position_to_offset(&self, pos: &Position) -> Option<usize> {
        let line_idx = pos.line as usize;
        if line_idx >= self.line_offsets.len() {
            return None;
        }
        let line_start = self.line_offsets[line_idx];
        let line_text = self.get_line(pos.line)?;
        // LSP character is UTF-16 code units. For ASCII-heavy code,
        // character == byte offset within line. We handle basic UTF-8
        // by iterating characters.
        let mut utf16_offset = 0u32;
        let mut byte_offset = 0usize;
        for ch in line_text.chars() {
            if utf16_offset >= pos.character {
                break;
            }
            utf16_offset += ch.len_utf16() as u32;
            byte_offset += ch.len_utf8();
        }
        Some(line_start + byte_offset)
    }

    /// Convert a byte offset to an LSP position.
    pub fn offset_to_position(&self, offset: usize) -> Position {
        let offset = offset.min(self.content.len());
        // Binary search for the line
        let line_idx = match self.line_offsets.binary_search(&offset) {
            Ok(idx) => idx,
            Err(idx) => {
                if idx > 0 {
                    idx - 1
                } else {
                    0
                }
            }
        };
        let line_start = self.line_offsets[line_idx];
        // Count UTF-16 code units from line start to offset
        let text_slice = &self.content[line_start..offset];
        let character: u32 = text_slice.chars().map(|c| c.len_utf16() as u32).sum();
        Position::new(line_idx as u32, character)
    }

    /// Get the number of lines.
    pub fn line_count(&self) -> usize {
        self.line_offsets.len()
    }

    /// Get the word at a given position.
    pub fn word_at_position(&self, pos: &Position) -> Option<(String, Range)> {
        let line_text = self.get_line(pos.line)?;
        let char_idx = pos.character as usize;
        if char_idx > line_text.len() {
            return None;
        }
        let bytes = line_text.as_bytes();
        // Find word boundaries
        let mut start = char_idx;
        while start > 0 && is_ident_char(bytes[start - 1]) {
            start -= 1;
        }
        let mut end = char_idx;
        while end < bytes.len() && is_ident_char(bytes[end]) {
            end += 1;
        }
        if start == end {
            return None;
        }
        let word = line_text[start..end].to_string();
        let range = Range::single_line(pos.line, start as u32, end as u32);
        Some((word, range))
    }
}

// ── DocumentStore ─────────────────────────────────────────────────────────────

/// Storage for all open documents.
#[derive(Debug, Default)]
pub struct DocumentStore {
    /// Map from URI to Document.
    documents: HashMap<String, Document>,
}

impl DocumentStore {
    /// Create a new empty document store.
    pub fn new() -> Self {
        Self {
            documents: HashMap::new(),
        }
    }

    /// Open a document.
    pub fn open_document(
        &mut self,
        uri: impl Into<String>,
        version: i64,
        content: impl Into<String>,
    ) {
        let uri = uri.into();
        let doc = Document::new(uri.clone(), version, content);
        self.documents.insert(uri, doc);
    }

    /// Update a document's content.
    pub fn update_document(&mut self, uri: &str, version: i64, content: impl Into<String>) -> bool {
        if let Some(doc) = self.documents.get_mut(uri) {
            doc.update(version, content);
            true
        } else {
            false
        }
    }

    /// Close a document.
    pub fn close_document(&mut self, uri: &str) -> bool {
        self.documents.remove(uri).is_some()
    }

    /// Get a document.
    pub fn get_document(&self, uri: &str) -> Option<&Document> {
        self.documents.get(uri)
    }

    /// Get a mutable reference to a document.
    pub fn get_document_mut(&mut self, uri: &str) -> Option<&mut Document> {
        self.documents.get_mut(uri)
    }

    /// Get all open document URIs.
    pub fn uris(&self) -> Vec<&String> {
        self.documents.keys().collect()
    }

    /// Number of open documents.
    pub fn len(&self) -> usize {
        self.documents.len()
    }

    /// Whether there are no open documents.
    pub fn is_empty(&self) -> bool {
        self.documents.is_empty()
    }
}

// ── Incremental sync helpers ──────────────────────────────────────────────────

/// Apply an incremental LSP text change to `content`, returning the updated string.
///
/// This mirrors the logic of `RequestDispatcher::apply_incremental_change` so it
/// can be tested independently.  The range uses UTF-16 code-unit positions (LSP
/// spec); `position_to_offset` on a `Document` handles the conversion.
pub fn apply_incremental_edit(
    content: &str,
    start_line: u32,
    start_char: u32,
    end_line: u32,
    end_char: u32,
    new_text: &str,
) -> String {
    let doc = Document::new("file:///test", 0, content);
    let start_pos = Position::new(start_line, start_char);
    let end_pos = Position::new(end_line, end_char);
    let content_len = doc.content.len();
    let start_off = doc
        .position_to_offset(&start_pos)
        .unwrap_or(0)
        .min(content_len);
    let end_off = doc
        .position_to_offset(&end_pos)
        .unwrap_or(content_len)
        .min(content_len)
        .max(start_off);

    let mut result =
        String::with_capacity(content_len.saturating_sub(end_off - start_off) + new_text.len());
    result.push_str(&doc.content[..start_off]);
    result.push_str(new_text);
    result.push_str(&doc.content[end_off..]);
    result
}

/// Apply a "full sync" edit (replace entire content).
fn apply_full_edit(new_text: &str) -> String {
    new_text.to_string()
}

#[cfg(test)]
mod incremental_sync_tests {
    use super::*;

    /// Helper: both incremental and full-sync must produce identical content.
    fn assert_incr_equals_full(
        original: &str,
        sl: u32,
        sc: u32,
        el: u32,
        ec: u32,
        new_text: &str,
        expected: &str,
    ) {
        let incr = apply_incremental_edit(original, sl, sc, el, ec, new_text);
        let full = apply_full_edit(expected);
        assert_eq!(
            incr, full,
            "incremental != expected\n  original: {:?}\n  range: ({},{})..({},{})\n  new_text: {:?}\n  got: {:?}\n  want: {:?}",
            original, sl, sc, el, ec, new_text, incr, full
        );
    }

    // ── Case 1: ASCII only ────────────────────────────────────────────────────

    #[test]
    fn test_incr_ascii_replace_word() {
        let orig = "hello world";
        // replace "world" (col 6..11) with "there"
        assert_incr_equals_full(orig, 0, 6, 0, 11, "there", "hello there");
    }

    // ── Case 2: UTF-8 2-byte chars (é = U+00E9, 2 bytes, 1 UTF-16 unit) ──────

    #[test]
    fn test_incr_two_byte_utf8() {
        // "café" — 'é' is 2 bytes, but 1 UTF-16 code unit
        // Line 0: "café"  (bytes: c a f é(2 bytes) = 5 bytes total)
        // UTF-16 positions: c=0, a=1, f=2, é=3, (end)=4
        let orig = "café world";
        // Replace 'é' at UTF-16 col 3..4 with "e"
        assert_incr_equals_full(orig, 0, 3, 0, 4, "e", "cafe world");
    }

    // ── Case 3: CJK chars (日 = U+65E5, 3 bytes, 1 UTF-16 unit) ────────────

    #[test]
    fn test_incr_cjk_single_char() {
        // "日本語" — each char is 3 bytes, 1 UTF-16 unit
        let orig = "日本語";
        // Replace the middle character '本' (UTF-16 col 1..2) with "B"
        assert_incr_equals_full(orig, 0, 1, 0, 2, "B", "日B語");
    }

    #[test]
    fn test_incr_cjk_insert_before() {
        let orig = "語";
        // Insert "日本" before (UTF-16 col 0..0)
        assert_incr_equals_full(orig, 0, 0, 0, 0, "日本", "日本語");
    }

    // ── Case 4: Emoji (😀 = U+1F600, 4 bytes, 2 UTF-16 code units) ──────────

    #[test]
    fn test_incr_emoji_surrogate_pair() {
        // "😀" is U+1F600: 4 bytes in UTF-8, 2 UTF-16 code units (surrogate pair)
        // UTF-16 positions: 😀 occupies cols 0 and 1; end of string is col 2
        let orig = "hi 😀 there";
        // Replace '😀' (UTF-16 col 3..5) with ":)"
        assert_incr_equals_full(orig, 0, 3, 0, 5, ":)", "hi :) there");
    }

    // ── Case 5: Multi-line insert (inserting newlines) ────────────────────────

    #[test]
    fn test_incr_multiline_insert_newline() {
        let orig = "first\nsecond\nthird";
        // Insert a new line after "first" by inserting at the end of line 0 col 5
        let expected = "first\ninserted\nsecond\nthird";
        assert_incr_equals_full(orig, 0, 5, 0, 5, "\ninserted", expected);
    }

    // ── Case 6: Multi-line delete (delete across newlines) ───────────────────

    #[test]
    fn test_incr_multiline_delete() {
        let orig = "alpha\nbeta\ngamma";
        // Delete from end of "alpha" (line 0, col 5) to start of "gamma" (line 2, col 0)
        let expected = "alphagamma";
        assert_incr_equals_full(orig, 0, 5, 2, 0, "", expected);
    }

    // ── Case 7: Replace at start of document ─────────────────────────────────

    #[test]
    fn test_incr_replace_at_start() {
        let orig = "def foo := 42";
        // Replace "def" (cols 0..3) with "let"
        assert_incr_equals_full(orig, 0, 0, 0, 3, "let", "let foo := 42");
    }

    // ── Case 8: Replace at end of document ───────────────────────────────────

    #[test]
    fn test_incr_replace_at_end() {
        let orig = "def foo := 42";
        // Replace "42" (cols 11..13) with "100"
        assert_incr_equals_full(orig, 0, 11, 0, 13, "100", "def foo := 100");
    }

    // ── Case 9: Empty range insertion (cursor insert) ─────────────────────────

    #[test]
    fn test_incr_empty_range_insert() {
        let orig = "ab";
        // Insert "X" at col 1 (between a and b)
        assert_incr_equals_full(orig, 0, 1, 0, 1, "X", "aXb");
    }

    // ── Case 10: Out-of-range position (no panic) ─────────────────────────────

    #[test]
    fn test_incr_out_of_range_no_panic() {
        let orig = "short";
        // Line 99, col 99 — way beyond the document. Should not panic.
        // position_to_offset returns None for out-of-range lines.
        // start: None → unwrap_or(0) = 0
        // end: None → unwrap_or(content_len) = 5, then max(start=0, 5) = 5
        // Result: orig[..0] + "appended" + orig[5..] = "appended"
        let result = apply_incremental_edit(orig, 99, 99, 99, 99, "appended");
        // Must not panic; result is implementation-defined for out-of-range
        assert!(!result.is_empty(), "result must not be empty");
        // Specifically: start clamps to 0, end clamps to len, so whole content is replaced
        assert_eq!(result, "appended");
    }

    #[test]
    fn test_incr_reversed_range_no_panic() {
        // Reversed (end < start by offset) — clamped to empty delete at start position
        let orig = "abcdef";
        // Request end col < start col on same line would normally underflow; must not panic
        let result = apply_incremental_edit(orig, 0, 4, 0, 2, "X");
        // end_off clamped to >= start_off, so treated as insert at col 2..2 = "abXcdef"
        // Actually: start=2, end=max(2,2)=2 → "abXcdef" is what we'd get for col 2..2
        // with "X". Our clamping sets end=max(start,end). Col 2 < col 4 → start < end
        // in bytes, so this is actually a normal forward range test.
        // For a genuinely reversed byte range, position_to_offset returns correct offsets
        // for each position independently.
        assert!(!result.is_empty(), "result must not be empty");
    }

    // ── Additional correctness checks ────────────────────────────────────────

    #[test]
    fn test_incr_replace_whole_line() {
        let orig = "line one\nline two\nline three";
        // Replace entire line 1 ("line two") with "NEW"
        assert_incr_equals_full(orig, 1, 0, 1, 8, "NEW", "line one\nNEW\nline three");
    }

    #[test]
    fn test_incr_mixed_unicode_document() {
        // Mix of ASCII, CJK, and emoji on different lines
        let orig = "hello\n日本語\n😀 world";
        // Replace "日本語" on line 1 with "Japanese"
        assert_incr_equals_full(orig, 1, 0, 1, 3, "Japanese", "hello\nJapanese\n😀 world");
    }
}

// ── Byte-to-char offset conversion tests ─────────────────────────────────────

#[cfg(test)]
mod position_to_char_offset_tests {
    use super::*;

    /// Helper: given a document content and an LSP position, return the char
    /// index that corresponds to the byte offset for that position.
    ///
    /// This mirrors the conversion used in `apply_incremental_change`:
    ///   byte_offset  = document.position_to_offset(pos)
    ///   char_index   = content[..byte_offset].chars().count()
    fn byte_to_char_index(content: &str, line: u32, character: u32) -> usize {
        let doc = Document::new("file:///test", 0, content);
        let pos = Position::new(line, character);
        let byte_off = doc.position_to_offset(&pos).unwrap_or(0);
        content[..byte_off].chars().count()
    }

    /// For pure-ASCII content every byte == one char, so char index == byte
    /// offset for any column on line 0.
    #[test]
    fn test_position_to_char_offset_ascii() {
        let content = "hello world";
        // Column 0 → char 0
        assert_eq!(byte_to_char_index(content, 0, 0), 0);
        // Column 5 → char 5
        assert_eq!(byte_to_char_index(content, 0, 5), 5);
        // Column 11 (end) → char 11
        assert_eq!(byte_to_char_index(content, 0, 11), 11);
    }

    /// Two-byte UTF-8 character (é = U+00E9, 2 bytes, 1 UTF-16 code unit).
    /// "café" → c(1B) a(1B) f(1B) é(2B) = 5 bytes, 4 chars.
    /// Byte offset of 'é' is 3 (byte) but char index is also 3.
    /// Byte offset after 'é' is 5 (bytes) but char index is 4.
    #[test]
    fn test_position_to_char_offset_two_byte_char() {
        let content = "café";
        // 'é' is at UTF-16 col 3, taking 2 bytes but 1 code unit.
        // char index at col 3 should be 3 (three chars precede it).
        assert_eq!(byte_to_char_index(content, 0, 3), 3);
        // After 'é' (col 4) → 4 chars consumed.
        assert_eq!(byte_to_char_index(content, 0, 4), 4);
    }

    /// Three-byte UTF-8 character (日 = U+65E5, 3 bytes, 1 UTF-16 code unit).
    /// "日本語" → 9 bytes, 3 chars.
    #[test]
    fn test_position_to_char_offset_cjk() {
        let content = "日本語";
        // UTF-16 col 0 → byte 0 → char index 0
        assert_eq!(byte_to_char_index(content, 0, 0), 0);
        // UTF-16 col 1 → byte 3 → char index 1
        assert_eq!(byte_to_char_index(content, 0, 1), 1);
        // UTF-16 col 2 → byte 6 → char index 2
        assert_eq!(byte_to_char_index(content, 0, 2), 2);
        // UTF-16 col 3 (end) → byte 9 → char index 3
        assert_eq!(byte_to_char_index(content, 0, 3), 3);
    }

    /// Four-byte UTF-8 emoji (😀 = U+1F600, 4 bytes, 2 UTF-16 code units).
    /// "hi 😀!" → h(1) i(1) ' '(1) 😀(4) !(1) = 8 bytes; char count = 5.
    /// LSP column for '!' is 5 (after the 2-unit emoji).
    #[test]
    fn test_position_to_char_offset_emoji() {
        let content = "hi 😀!";
        // '!' is at UTF-16 col 5 (hi=0,1; space=2; emoji=3,4; !=5)
        // byte offset = 1+1+1+4 = 7, char index = 4
        assert_eq!(byte_to_char_index(content, 0, 5), 4);
        // Beginning
        assert_eq!(byte_to_char_index(content, 0, 0), 0);
    }

    /// Multi-line content: the char offset for position (1, col) must account
    /// for the newline character that terminates line 0.
    #[test]
    fn test_position_to_char_offset_multiline() {
        let content = "abc\ndef";
        // Line 1, col 0 → byte 4 → char index 4 (a,b,c,\n = 4 chars)
        assert_eq!(byte_to_char_index(content, 1, 0), 4);
        // Line 1, col 3 → byte 7 → char index 7
        assert_eq!(byte_to_char_index(content, 1, 3), 7);
    }
}
