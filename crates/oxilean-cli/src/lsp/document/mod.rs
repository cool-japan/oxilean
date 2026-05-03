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
        }
    }

    /// Update the document content.
    pub fn update(&mut self, version: i64, content: impl Into<String>) {
        self.version = version;
        self.content = content.into();
        self.line_offsets = compute_line_offsets(&self.content);
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
