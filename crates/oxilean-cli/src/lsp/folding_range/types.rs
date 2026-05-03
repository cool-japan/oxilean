//! Types for the LSP folding range module.
//!
//! Provides data structures for `textDocument/foldingRange` LSP requests on
//! Lean4-like source files.  The specification reference is LSP §3.16.4.

use crate::lsp::JsonValue;

// ============================================================================
// FoldingRangeKind
// ============================================================================

/// The kind of a folding range, as defined by the LSP specification.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FoldingRangeKind {
    /// A comment folding range — consecutive `--` lines or `/-…-/` blocks.
    Comment,
    /// An imports folding range — consecutive `import` lines.
    Imports,
    /// A region folding range — `def`/`theorem`/`namespace`/`section` bodies,
    /// `do` blocks, and similar structural regions.
    Region,
}

impl FoldingRangeKind {
    /// Return the LSP string representation of this kind.
    pub fn as_str(self) -> &'static str {
        match self {
            FoldingRangeKind::Comment => "comment",
            FoldingRangeKind::Imports => "imports",
            FoldingRangeKind::Region => "region",
        }
    }

    /// Serialize to a JSON string value.
    pub fn to_json(self) -> JsonValue {
        JsonValue::String(self.as_str().to_string())
    }
}

// ============================================================================
// FoldingRange
// ============================================================================

/// A single folding range, corresponding to `FoldingRange` in LSP §3.16.4.
///
/// `start_line` and `end_line` are 0-indexed line numbers.  The optional
/// `start_character` / `end_character` fields indicate the character columns
/// at which folding begins/ends — these are omitted for whole-line ranges.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FoldingRange {
    /// The first line of the folding range (0-indexed, inclusive).
    pub start_line: u32,
    /// The last line of the folding range (0-indexed, inclusive).
    pub end_line: u32,
    /// Optional start column (0-indexed).
    pub start_character: Option<u32>,
    /// Optional end column (0-indexed).
    pub end_character: Option<u32>,
    /// The kind of folding range.
    pub kind: FoldingRangeKind,
}

impl FoldingRange {
    /// Create a whole-line folding range (no character offsets).
    pub fn whole_lines(start_line: u32, end_line: u32, kind: FoldingRangeKind) -> Self {
        Self {
            start_line,
            end_line,
            start_character: None,
            end_character: None,
            kind,
        }
    }

    /// Create a folding range with explicit character columns.
    pub fn with_chars(
        start_line: u32,
        end_line: u32,
        start_character: u32,
        end_character: u32,
        kind: FoldingRangeKind,
    ) -> Self {
        Self {
            start_line,
            end_line,
            start_character: Some(start_character),
            end_character: Some(end_character),
            kind,
        }
    }

    /// Return the number of lines spanned (1 means a single-line range).
    pub fn line_count(&self) -> u32 {
        self.end_line.saturating_sub(self.start_line) + 1
    }

    /// Return `true` if this range covers more than one line.
    pub fn is_multiline(&self) -> bool {
        self.end_line > self.start_line
    }

    /// Serialize to JSON (LSP `FoldingRange` format).
    pub fn to_json(&self) -> JsonValue {
        range_to_json(self)
    }
}

/// Serialize a `FoldingRange` to a JSON value.
///
/// This is a free function so it can be `pub use`-ed independently.
pub fn range_to_json(range: &FoldingRange) -> JsonValue {
    let mut entries = vec![
        (
            "startLine".to_string(),
            JsonValue::Number(range.start_line as f64),
        ),
        (
            "endLine".to_string(),
            JsonValue::Number(range.end_line as f64),
        ),
        ("kind".to_string(), range.kind.to_json()),
    ];
    if let Some(sc) = range.start_character {
        entries.push(("startCharacter".to_string(), JsonValue::Number(sc as f64)));
    }
    if let Some(ec) = range.end_character {
        entries.push(("endCharacter".to_string(), JsonValue::Number(ec as f64)));
    }
    JsonValue::Object(entries)
}

// ============================================================================
// FoldingRangeHandler
// ============================================================================

/// Stateless handler facade for `textDocument/foldingRange` LSP requests.
///
/// Holds no mutable state; all inputs are passed via method arguments so the
/// handler can safely be shared across concurrent requests.
#[derive(Clone, Debug, Default)]
pub struct FoldingRangeHandler;

impl FoldingRangeHandler {
    /// Create a new handler instance.
    pub fn new() -> Self {
        Self
    }
}
