//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
/// A half-open byte range `[start, end)` in a source file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Span {
    /// Inclusive start byte position.
    pub start: BytePos,
    /// Exclusive end byte position.
    pub end: BytePos,
}
impl Span {
    /// Create a new span.
    pub fn new(start: BytePos, end: BytePos) -> Self {
        Self { start, end }
    }
    /// Create a span from raw byte offsets.
    pub fn from_offsets(start: u32, end: u32) -> Self {
        Self {
            start: BytePos(start),
            end: BytePos(end),
        }
    }
    /// Create a zero-length span at position `pos`.
    pub fn point(pos: BytePos) -> Self {
        Self {
            start: pos,
            end: pos,
        }
    }
    /// Length in bytes.
    pub fn len(self) -> u32 {
        self.end.0.saturating_sub(self.start.0)
    }
    /// Returns `true` if this span is empty.
    pub fn is_empty(self) -> bool {
        self.start >= self.end
    }
    /// Merge two spans into the smallest span covering both.
    pub fn merge(self, other: Span) -> Span {
        Span {
            start: BytePos(self.start.0.min(other.start.0)),
            end: BytePos(self.end.0.max(other.end.0)),
        }
    }
    /// Check whether this span contains `pos`.
    pub fn contains(self, pos: BytePos) -> bool {
        self.start <= pos && pos < self.end
    }
    /// Slice the source text covered by this span.
    ///
    /// Returns `None` if the span is out of bounds.
    pub fn slice<'a>(&self, src: &'a str) -> Option<&'a str> {
        let s = self.start.to_usize();
        let e = self.end.to_usize();
        src.get(s..e)
    }
    /// Shift both endpoints by `n` bytes.
    pub fn shift(self, n: u32) -> Self {
        Span {
            start: self.start.shift(n),
            end: self.end.shift(n),
        }
    }
}
/// Extended string encoder/decoder.
#[allow(dead_code)]
pub struct StringEncoderExt {
    /// Encoding type label.
    pub encoding: String,
}
impl StringEncoderExt {
    /// Create a UTF-8 encoder.
    pub fn utf8() -> Self {
        Self {
            encoding: "UTF-8".to_string(),
        }
    }
    /// Encode a string to bytes (UTF-8).
    pub fn encode(&self, s: &str) -> Vec<u8> {
        s.as_bytes().to_vec()
    }
    /// Decode bytes from UTF-8.
    pub fn decode(&self, bytes: &[u8]) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(bytes.to_vec())
    }
    /// Check round-trip property.
    pub fn roundtrip(&self, s: &str) -> bool {
        self.decode(&self.encode(s)).as_deref() == Ok(s)
    }
}
/// Extended string monoid utilities.
#[allow(dead_code)]
pub struct StringMonoidExt {
    /// Accumulated string.
    pub buffer: String,
}
impl StringMonoidExt {
    /// Create a new empty monoid.
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }
    /// Append another string (monoid operation).
    pub fn mappend(&mut self, other: &str) {
        self.buffer.push_str(other);
    }
    /// Check identity: buffer == ""
    pub fn is_identity(&self) -> bool {
        self.buffer.is_empty()
    }
}
/// A (1-based) line and column position in a source file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LineCol {
    /// 1-based line number.
    pub line: u32,
    /// 1-based column number (in characters, not bytes).
    pub col: u32,
}
impl LineCol {
    /// Create a new `LineCol`.
    pub fn new(line: u32, col: u32) -> Self {
        Self { line, col }
    }
}
/// Extended substring finder using KMP or naive search.
#[allow(dead_code)]
pub struct SubstringFinder2 {
    /// The haystack to search in.
    pub text: String,
    /// The pattern to search for.
    pub pattern: String,
}
impl SubstringFinder2 {
    /// Create a new finder.
    pub fn new(text: impl Into<String>, pattern: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            pattern: pattern.into(),
        }
    }
    /// Find all non-overlapping occurrences using naive search.
    pub fn find_all(&self) -> Vec<usize> {
        let mut positions = Vec::new();
        let t = &self.text;
        let p = &self.pattern;
        if p.is_empty() {
            return positions;
        }
        let mut start = 0;
        while let Some(pos) = t[start..].find(p.as_str()) {
            positions.push(start + pos);
            start += pos + p.len();
        }
        positions
    }
    /// Find using KMP algorithm.
    pub fn find_kmp(&self) -> Vec<usize> {
        str_ext2_kmp_search(&self.text, &self.pattern)
    }
}
/// Levenshtein metric (version 2) with additional operations.
#[allow(dead_code)]
pub struct LevenshteinMetric2 {
    /// Maximum allowed distance (None = unlimited).
    pub max_dist: Option<usize>,
}
impl LevenshteinMetric2 {
    /// Create an unbounded metric.
    pub fn new() -> Self {
        Self { max_dist: None }
    }
    /// Create with a maximum distance threshold.
    pub fn with_max(d: usize) -> Self {
        Self { max_dist: Some(d) }
    }
    /// Compute the distance between two strings.
    pub fn distance(&self, a: &str, b: &str) -> usize {
        str_ext2_levenshtein(a, b)
    }
    /// Check if distance is within threshold.
    pub fn within_threshold(&self, a: &str, b: &str) -> bool {
        match self.max_dist {
            Some(d) => self.distance(a, b) <= d,
            None => true,
        }
    }
    /// Check metric identity: d(a, a) = 0.
    pub fn identity_law(&self, a: &str) -> bool {
        self.distance(a, a) == 0
    }
    /// Check metric symmetry: d(a, b) = d(b, a).
    pub fn symmetry_law(&self, a: &str, b: &str) -> bool {
        self.distance(a, b) == self.distance(b, a)
    }
}
/// A mutable string builder with convenient push methods.
///
/// Accumulates string fragments and can be finalised into a `String`.
/// Supports indentation tracking for pretty-printing.
#[derive(Debug, Default, Clone)]
pub struct StringBuilder {
    pub(super) buf: String,
    pub(super) indent_level: usize,
    indent_width: usize,
}
impl StringBuilder {
    /// Create a new empty `StringBuilder`.
    pub fn new() -> Self {
        Self {
            buf: String::new(),
            indent_level: 0,
            indent_width: 2,
        }
    }
    /// Create a `StringBuilder` with pre-allocated capacity.
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            buf: String::with_capacity(cap),
            indent_level: 0,
            indent_width: 2,
        }
    }
    /// Set the number of spaces per indent level (default: 2).
    pub fn set_indent_width(&mut self, w: usize) -> &mut Self {
        self.indent_width = w;
        self
    }
    /// Append a string slice.
    pub fn push_str(&mut self, s: &str) -> &mut Self {
        self.buf.push_str(s);
        self
    }
    /// Append a single character.
    pub fn push(&mut self, c: char) -> &mut Self {
        self.buf.push(c);
        self
    }
    /// Append a newline followed by the current indentation.
    pub fn newline(&mut self) -> &mut Self {
        self.buf.push('\n');
        self.buf
            .push_str(&" ".repeat(self.indent_level * self.indent_width));
        self
    }
    /// Append `s` followed by a newline (and indent for next line).
    pub fn line(&mut self, s: &str) -> &mut Self {
        self.buf.push_str(s);
        self.newline();
        self
    }
    /// Increase the indentation level by 1.
    pub fn indent(&mut self) -> &mut Self {
        self.indent_level += 1;
        self
    }
    /// Decrease the indentation level by 1 (minimum 0).
    pub fn dedent(&mut self) -> &mut Self {
        self.indent_level = self.indent_level.saturating_sub(1);
        self
    }
    /// Append `s` formatted with `format!`.
    pub fn push_fmt(&mut self, args: std::fmt::Arguments<'_>) -> &mut Self {
        use std::fmt::Write;
        let _ = self.buf.write_fmt(args);
        self
    }
    /// Current byte length of the buffer.
    pub fn len(&self) -> usize {
        self.buf.len()
    }
    /// Returns `true` if the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }
    /// Finalise and return the accumulated string.
    pub fn finish(self) -> String {
        self.buf
    }
    /// Get a reference to the accumulated string so far.
    pub fn as_str(&self) -> &str {
        &self.buf
    }
    /// Clear the buffer, keeping capacity.
    pub fn clear(&mut self) {
        self.buf.clear();
    }
    /// Append a separator only if the buffer is non-empty.
    pub fn sep(&mut self, s: &str) -> &mut Self {
        if !self.buf.is_empty() {
            self.buf.push_str(s);
        }
        self
    }
}
/// A byte position in a source file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BytePos(pub u32);
impl BytePos {
    /// Create a `BytePos` from a `usize` offset.
    pub fn from_usize(n: usize) -> Self {
        Self(n as u32)
    }
    /// Convert to `usize`.
    pub fn to_usize(self) -> usize {
        self.0 as usize
    }
    /// Shift the position forward by `n` bytes.
    pub fn shift(self, n: u32) -> Self {
        Self(self.0 + n)
    }
}
/// Rolling hash for Rabin-Karp pattern matching.
#[allow(dead_code)]
pub struct RollingHashExt {
    /// Base for the polynomial hash.
    pub base: u64,
    /// Modulus for the hash.
    pub modulus: u64,
}
impl RollingHashExt {
    /// Create with standard parameters.
    pub fn new() -> Self {
        Self {
            base: 31,
            modulus: 1_000_000_007,
        }
    }
    /// Compute hash of a string.
    pub fn hash_str(&self, s: &str) -> u64 {
        s.bytes().fold(0u64, |acc, b| {
            (acc.wrapping_mul(self.base).wrapping_add(b as u64)) % self.modulus
        })
    }
    /// Find pattern in text using Rabin-Karp.
    pub fn find(&self, text: &str, pattern: &str) -> Vec<usize> {
        str_ext2_rabin_karp(text, pattern, self.base, self.modulus)
    }
}
