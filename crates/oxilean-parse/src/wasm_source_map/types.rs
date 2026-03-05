//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;

/// Base64 alphabet used by VLQ encoding.
const BASE64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// A complete source map (Source Map spec v3).
pub struct SourceMap {
    pub mappings: Vec<SourceMapping>,
    pub sources: Vec<String>,
    pub names: Vec<String>,
    pub version: u32,
}
impl SourceMap {
    pub fn new() -> Self {
        Self {
            mappings: Vec::new(),
            sources: Vec::new(),
            names: Vec::new(),
            version: 3,
        }
    }
    /// Add a mapping entry.
    pub fn add_mapping(&mut self, m: SourceMapping) {
        self.mappings.push(m);
    }
    /// Add a source file, returning its index. Deduplicates.
    pub fn add_source(&mut self, file: &str) -> usize {
        if let Some(idx) = self.sources.iter().position(|s| s == file) {
            return idx;
        }
        let idx = self.sources.len();
        self.sources.push(file.to_string());
        idx
    }
    /// Generate a JSON source map string.
    pub fn to_json(&self) -> String {
        let sources_json = self
            .sources
            .iter()
            .map(|s| format!("\"{}\"", s.replace('"', "\\\"")))
            .collect::<Vec<_>>()
            .join(",");
        let mappings_str = self.encode_mappings();
        format!(
            "{{\"version\":{},\"sources\":[{}],\"mappings\":\"{}\"}}",
            self.version, sources_json, mappings_str
        )
    }
    /// Generate the VLQ-encoded mappings string.
    pub fn encode_mappings(&self) -> String {
        if self.mappings.is_empty() {
            return String::new();
        }
        let max_line = self
            .mappings
            .iter()
            .map(|m| m.generated_line)
            .max()
            .unwrap_or(0);
        let mut lines: Vec<Vec<&SourceMapping>> = vec![Vec::new(); (max_line + 1) as usize];
        for m in &self.mappings {
            if (m.generated_line as usize) < lines.len() {
                lines[m.generated_line as usize].push(m);
            }
        }
        for line in &mut lines {
            line.sort_by_key(|m| m.generated_col);
        }
        let mut result_lines: Vec<String> = Vec::new();
        let mut prev_src_line: i64 = 0;
        let mut prev_src_col: i64 = 0;
        let mut prev_src_file: i64 = 0;
        for line_mappings in &lines {
            let mut prev_gen_col: i64 = 0;
            let mut segments: Vec<String> = Vec::new();
            for m in line_mappings {
                let src_idx = self
                    .sources
                    .iter()
                    .position(|s| *s == m.source_file)
                    .unwrap_or(0) as i64;
                let gen_col_delta = m.generated_col as i64 - prev_gen_col;
                let src_file_delta = src_idx - prev_src_file;
                let src_line_delta = m.source_line as i64 - prev_src_line;
                let src_col_delta = m.source_col as i64 - prev_src_col;
                let seg = VlqEncoder::encode_segment(&[
                    gen_col_delta,
                    src_file_delta,
                    src_line_delta,
                    src_col_delta,
                ]);
                segments.push(seg);
                prev_gen_col = m.generated_col as i64;
                prev_src_file = src_idx;
                prev_src_line = m.source_line as i64;
                prev_src_col = m.source_col as i64;
            }
            result_lines.push(segments.join(","));
        }
        result_lines.join(";")
    }
    /// Look up the source mapping for a given generated position.
    pub fn lookup_source(&self, gen_line: u32, gen_col: u32) -> Option<&SourceMapping> {
        self.mappings
            .iter()
            .filter(|m| m.generated_line == gen_line && m.generated_col <= gen_col)
            .max_by_key(|m| m.generated_col)
    }
}
/// Add a source file name to the map.
impl SourceMap {
    /// Add a name to the names list, returning its index.
    #[allow(dead_code)]
    pub fn add_name(&mut self, name: &str) -> usize {
        if let Some(idx) = self.names.iter().position(|n| n == name) {
            return idx;
        }
        let idx = self.names.len();
        self.names.push(name.to_string());
        idx
    }
    /// Remove all mappings from the source map.
    #[allow(dead_code)]
    pub fn clear_mappings(&mut self) {
        self.mappings.clear();
    }
    /// Return the mapping count.
    #[allow(dead_code)]
    pub fn mapping_count(&self) -> usize {
        self.mappings.len()
    }
    /// Return the source count.
    #[allow(dead_code)]
    pub fn source_count(&self) -> usize {
        self.sources.len()
    }
    /// Return the name count.
    #[allow(dead_code)]
    pub fn name_count(&self) -> usize {
        self.names.len()
    }
    /// Validate the source map: check all source file indices are valid.
    #[allow(dead_code)]
    pub fn validate(&self) -> Result<(), String> {
        for m in &self.mappings {
            if !self.sources.iter().any(|s| s == &m.source_file) {
                return Err(format!(
                    "Source file '{}' not in sources list",
                    m.source_file
                ));
            }
        }
        Ok(())
    }
    /// Sort mappings by (generated_line, generated_col).
    #[allow(dead_code)]
    pub fn sort_mappings(&mut self) {
        self.mappings.sort_by(|a, b| {
            a.generated_line
                .cmp(&b.generated_line)
                .then(a.generated_col.cmp(&b.generated_col))
        });
    }
    /// Merge another source map into this one (re-numbering sources).
    #[allow(dead_code)]
    pub fn merge(&mut self, other: &SourceMap) {
        for m in &other.mappings {
            self.add_mapping(SourceMapping::new(
                m.generated_line,
                m.generated_col,
                m.source_line,
                m.source_col,
                &m.source_file,
            ));
            self.add_source(&m.source_file);
        }
    }
}
/// A WASM source annotation table.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct WasmAnnotationTable {
    /// All annotations sorted by WASM offset
    pub annotations: Vec<WasmAnnotation>,
}
impl WasmAnnotationTable {
    /// Create a new empty table.
    #[allow(dead_code)]
    pub fn new() -> Self {
        WasmAnnotationTable {
            annotations: Vec::new(),
        }
    }
    /// Add an annotation.
    #[allow(dead_code)]
    pub fn add(&mut self, ann: WasmAnnotation) {
        self.annotations.push(ann);
        self.annotations.sort_by_key(|a| a.wasm_offset);
    }
    /// Look up the annotation nearest to a WASM offset.
    #[allow(dead_code)]
    pub fn lookup(&self, wasm_offset: u32) -> Option<&WasmAnnotation> {
        let idx = self
            .annotations
            .partition_point(|a| a.wasm_offset <= wasm_offset);
        if idx == 0 {
            return None;
        }
        Some(&self.annotations[idx - 1])
    }
    /// Returns the number of annotations.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.annotations.len()
    }
    /// Returns true if empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.annotations.is_empty()
    }
}
/// A source map that maps to multiple generated files from a single source.
#[allow(dead_code)]
pub struct SourceToGeneratedMap {
    /// Source file name.
    pub source: String,
    /// Map from generated file name to its source map.
    pub generated_maps: std::collections::HashMap<String, SourceMap>,
}
impl SourceToGeneratedMap {
    /// Create a new source-to-generated map.
    #[allow(dead_code)]
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            generated_maps: std::collections::HashMap::new(),
        }
    }
    /// Register a generated file's source map.
    #[allow(dead_code)]
    pub fn add_generated(&mut self, gen_file: &str, sm: SourceMap) {
        self.generated_maps.insert(gen_file.to_string(), sm);
    }
    /// Look up all generated positions for a given source position.
    #[allow(dead_code)]
    pub fn find_generated(&self, src_line: u32, src_col: u32) -> Vec<(&str, GeneratedPosition)> {
        let mut results = Vec::new();
        for (gen_file, sm) in &self.generated_maps {
            for m in &sm.mappings {
                if m.source_line == src_line && m.source_col == src_col {
                    results.push((
                        gen_file.as_str(),
                        GeneratedPosition::new(m.generated_line, m.generated_col),
                    ));
                }
            }
        }
        results
    }
}
/// VLQ stream encoder: encode a stream of values lazily.
#[allow(dead_code)]
pub struct VlqStream {
    output: Vec<u8>,
}
impl VlqStream {
    /// Create a new stream encoder.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { output: Vec::new() }
    }
    /// Append a VLQ-encoded value.
    #[allow(dead_code)]
    pub fn push(&mut self, value: i64) {
        self.output.extend(VlqEncoder::encode_vlq(value));
    }
    /// Get the encoded output as a string.
    #[allow(dead_code)]
    pub fn finish(self) -> String {
        String::from_utf8(self.output).unwrap_or_default()
    }
    /// Length of the current output.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.output.len()
    }
    /// Whether output is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.output.is_empty()
    }
}
/// A source mapping entry.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct SourceMapEntry {
    /// Generated column in the output
    pub gen_col: u32,
    /// Source file index
    pub source_idx: u32,
    /// Original line number
    pub orig_line: u32,
    /// Original column number
    pub orig_col: u32,
    /// Optional symbol name index
    pub name_idx: Option<u32>,
}
impl SourceMapEntry {
    /// Create a new entry.
    #[allow(dead_code)]
    pub fn new(gen_col: u32, source_idx: u32, orig_line: u32, orig_col: u32) -> Self {
        SourceMapEntry {
            gen_col,
            source_idx,
            orig_line,
            orig_col,
            name_idx: None,
        }
    }
    /// Attach a symbol name index.
    #[allow(dead_code)]
    pub fn with_name(mut self, idx: u32) -> Self {
        self.name_idx = Some(idx);
        self
    }
}
/// A simple range of WASM offsets.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WasmOffsetRange {
    /// Inclusive start offset
    pub start: u32,
    /// Exclusive end offset
    pub end: u32,
}
impl WasmOffsetRange {
    /// Create a new range.
    #[allow(dead_code)]
    pub fn new(start: u32, end: u32) -> Self {
        WasmOffsetRange { start, end }
    }
    /// Returns the length of the range.
    #[allow(dead_code)]
    pub fn len(&self) -> u32 {
        self.end.saturating_sub(self.start)
    }
    /// Returns true if the range is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.start >= self.end
    }
    /// Returns true if the range contains the given offset.
    #[allow(dead_code)]
    pub fn contains(&self, offset: u32) -> bool {
        offset >= self.start && offset < self.end
    }
    /// Returns the overlap of two ranges, or None if disjoint.
    #[allow(dead_code)]
    pub fn overlap(&self, other: &WasmOffsetRange) -> Option<WasmOffsetRange> {
        let start = self.start.max(other.start);
        let end = self.end.min(other.end);
        if start < end {
            Some(WasmOffsetRange::new(start, end))
        } else {
            None
        }
    }
}
/// Utilities for base64 encoding/decoding without VLQ.
#[allow(dead_code)]
pub struct Base64Util;
impl Base64Util {
    /// Encode a byte slice to a base64 string.
    #[allow(dead_code)]
    pub fn encode(data: &[u8]) -> String {
        let mut out = String::new();
        for chunk in data.chunks(3) {
            let b0 = chunk[0] as u32;
            let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
            let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
            let n = (b0 << 16) | (b1 << 8) | b2;
            out.push(BASE64_CHARS[(n >> 18) as usize] as char);
            out.push(BASE64_CHARS[((n >> 12) & 0x3F) as usize] as char);
            if chunk.len() > 1 {
                out.push(BASE64_CHARS[((n >> 6) & 0x3F) as usize] as char);
            } else {
                out.push('=');
            }
            if chunk.len() > 2 {
                out.push(BASE64_CHARS[(n & 0x3F) as usize] as char);
            } else {
                out.push('=');
            }
        }
        out
    }
    /// Decode a base64 string (ignores padding).
    #[allow(dead_code)]
    pub fn decode(s: &str) -> Vec<u8> {
        let mut out = Vec::new();
        let chars: Vec<u8> = s
            .chars()
            .filter(|c| *c != '=')
            .map(|c| BASE64_CHARS.iter().position(|&b| b == c as u8).unwrap_or(0) as u8)
            .collect();
        for chunk in chars.chunks(4) {
            let b0 = chunk[0] as u32;
            let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
            let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
            let b3 = if chunk.len() > 3 { chunk[3] as u32 } else { 0 };
            let n = (b0 << 18) | (b1 << 12) | (b2 << 6) | b3;
            out.push((n >> 16) as u8);
            if chunk.len() > 2 {
                out.push(((n >> 8) & 0xFF) as u8);
            }
            if chunk.len() > 3 {
                out.push((n & 0xFF) as u8);
            }
        }
        out
    }
}
/// A source position (file, line, column).
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourcePosition {
    pub file: String,
    pub line: u32,
    pub col: u32,
}
impl SourcePosition {
    /// Create a new source position.
    #[allow(dead_code)]
    pub fn new(file: &str, line: u32, col: u32) -> Self {
        Self {
            file: file.to_string(),
            line,
            col,
        }
    }
}
/// VLQ encoder/decoder for source map segments.
pub struct VlqEncoder;
impl VlqEncoder {
    /// Encode a signed integer value as base64 VLQ bytes.
    pub fn encode_vlq(value: i64) -> Vec<u8> {
        let mut vlq: u64 = if value < 0 {
            ((-value as u64) << 1) | 1
        } else {
            (value as u64) << 1
        };
        let mut result = Vec::new();
        loop {
            let mut digit = (vlq & 0x1F) as u8;
            vlq >>= 5;
            if vlq > 0 {
                digit |= 0x20;
            }
            result.push(BASE64_CHARS[digit as usize]);
            if vlq == 0 {
                break;
            }
        }
        result
    }
    /// Decode one VLQ value from a byte slice. Returns (value, bytes_consumed).
    pub fn decode_vlq(bytes: &[u8]) -> (i64, usize) {
        let mut result: u64 = 0;
        let mut shift = 0u32;
        let mut idx = 0;
        loop {
            if idx >= bytes.len() {
                break;
            }
            let b = bytes[idx];
            let digit = BASE64_CHARS.iter().position(|&c| c == b).unwrap_or(0) as u64;
            idx += 1;
            let has_continuation = (digit & 0x20) != 0;
            let chunk = digit & 0x1F;
            result |= chunk << shift;
            shift += 5;
            if !has_continuation {
                break;
            }
        }
        let negative = (result & 1) != 0;
        let magnitude = (result >> 1) as i64;
        let value = if negative { -magnitude } else { magnitude };
        (value, idx)
    }
    /// Encode a segment (multiple values) as a VLQ string.
    pub fn encode_segment(values: &[i64]) -> String {
        let mut bytes = Vec::new();
        for &v in values {
            bytes.extend(Self::encode_vlq(v));
        }
        String::from_utf8(bytes).unwrap_or_default()
    }
}
/// Source range: a span of source positions.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceRange {
    pub start: SourcePosition,
    pub end: SourcePosition,
}
impl SourceRange {
    /// Create a new source range.
    #[allow(dead_code)]
    pub fn new(start: SourcePosition, end: SourcePosition) -> Self {
        Self { start, end }
    }
    /// Whether a position is contained in this range (same file, same line).
    #[allow(dead_code)]
    pub fn contains_position(&self, pos: &SourcePosition) -> bool {
        pos.file == self.start.file
            && pos.line >= self.start.line
            && pos.line <= self.end.line
            && (pos.line != self.start.line || pos.col >= self.start.col)
            && (pos.line != self.end.line || pos.col <= self.end.col)
    }
}
/// A source map merge utility.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct SourceMapMerger {
    /// The merged map
    pub result: FullSourceMap,
}
impl SourceMapMerger {
    /// Create a new merger.
    #[allow(dead_code)]
    pub fn new() -> Self {
        SourceMapMerger {
            result: FullSourceMap::new(),
        }
    }
    /// Add a source map (merging its sources and groups).
    #[allow(dead_code)]
    pub fn merge(&mut self, map: FullSourceMap) {
        let base_source_idx = self.result.sources.len() as u32;
        for src in map.sources {
            self.result.add_source(&src);
        }
        for mut group in map.groups {
            for entry in &mut group.entries {
                entry.source_idx += base_source_idx;
            }
            self.result.add_group(group);
        }
    }
    /// Finalise and return the merged map.
    #[allow(dead_code)]
    pub fn finish(self) -> FullSourceMap {
        self.result
    }
}
/// Reverse source map: generated position → original source position.
#[allow(dead_code)]
pub struct ReverseSourceMap {
    forward: SourceMap,
}
impl ReverseSourceMap {
    /// Build a reverse source map from a forward `SourceMap`.
    #[allow(dead_code)]
    pub fn build(sm: SourceMap) -> Self {
        Self { forward: sm }
    }
    /// Look up the source position for a generated position.
    #[allow(dead_code)]
    pub fn original(&self, gen_line: u32, gen_col: u32) -> Option<SourcePosition> {
        let m = self.forward.lookup_source(gen_line, gen_col)?;
        Some(SourcePosition::new(
            &m.source_file,
            m.source_line,
            m.source_col,
        ))
    }
    /// Number of mappings.
    #[allow(dead_code)]
    pub fn mapping_count(&self) -> usize {
        self.forward.mappings.len()
    }
}
/// A decoded segment from a source map mappings string.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct DecodedSegment {
    /// Generated column (absolute).
    pub gen_col: i64,
    /// Source file index (absolute, optional).
    pub src_file: Option<i64>,
    /// Source line (absolute, optional).
    pub src_line: Option<i64>,
    /// Source column (absolute, optional).
    pub src_col: Option<i64>,
    /// Names index (optional).
    pub names_idx: Option<i64>,
}
impl DecodedSegment {
    /// Create a segment with all fields.
    #[allow(dead_code)]
    pub fn full(gen_col: i64, src_file: i64, src_line: i64, src_col: i64) -> Self {
        Self {
            gen_col,
            src_file: Some(src_file),
            src_line: Some(src_line),
            src_col: Some(src_col),
            names_idx: None,
        }
    }
    /// Create a generated-only segment (no source info).
    #[allow(dead_code)]
    pub fn generated_only(gen_col: i64) -> Self {
        Self {
            gen_col,
            src_file: None,
            src_line: None,
            src_col: None,
            names_idx: None,
        }
    }
    /// Whether this segment has source information.
    #[allow(dead_code)]
    pub fn has_source(&self) -> bool {
        self.src_file.is_some()
    }
}
/// A WASM source coverage record: tracks which wasm offsets were executed.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct WasmCoverageRecord {
    /// Set of executed wasm offsets
    pub executed: std::collections::HashSet<u32>,
}
impl WasmCoverageRecord {
    /// Create an empty coverage record.
    #[allow(dead_code)]
    pub fn new() -> Self {
        WasmCoverageRecord {
            executed: std::collections::HashSet::new(),
        }
    }
    /// Mark a wasm offset as executed.
    #[allow(dead_code)]
    pub fn mark(&mut self, offset: u32) {
        self.executed.insert(offset);
    }
    /// Returns the number of executed offsets.
    #[allow(dead_code)]
    pub fn executed_count(&self) -> usize {
        self.executed.len()
    }
    /// Returns true if a given offset was executed.
    #[allow(dead_code)]
    pub fn was_executed(&self, offset: u32) -> bool {
        self.executed.contains(&offset)
    }
    /// Coverage fraction: executed / total_in_table.
    #[allow(dead_code)]
    pub fn coverage_fraction(&self, table: &WasmAnnotationTable) -> f64 {
        let total = total_annotations(table);
        if total == 0 {
            return 1.0;
        }
        let executed: usize = table
            .annotations
            .iter()
            .filter(|a| self.executed.contains(&a.wasm_offset))
            .count();
        executed as f64 / total as f64
    }
}
/// A range between two source positions.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy)]
pub struct SourceRangeExt {
    /// Start position
    pub start: SourcePos2,
    /// End position
    pub end: SourcePos2,
}
impl SourceRangeExt {
    /// Create a new source range.
    #[allow(dead_code)]
    pub fn new(start: SourcePos2, end: SourcePos2) -> Self {
        SourceRangeExt { start, end }
    }
    /// Check if a position is within this range.
    #[allow(dead_code)]
    pub fn contains(&self, pos: SourcePos2) -> bool {
        !pos.before(&self.start) && pos.before(&self.end)
    }
}
/// A source map group (one per generated line).
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Default)]
pub struct SourceMapGroup {
    /// Entries for this line
    pub entries: Vec<SourceMapEntry>,
}
impl SourceMapGroup {
    /// Create a new empty group.
    #[allow(dead_code)]
    pub fn new() -> Self {
        SourceMapGroup {
            entries: Vec::new(),
        }
    }
    /// Add an entry.
    #[allow(dead_code)]
    pub fn add(&mut self, entry: SourceMapEntry) {
        self.entries.push(entry);
    }
    /// Sort entries by generated column.
    #[allow(dead_code)]
    pub fn sort(&mut self) {
        self.entries.sort_by_key(|e| e.gen_col);
    }
}
/// A source map builder with a fluent API.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct SourceMapBuilder {
    /// The map being built
    pub map: FullSourceMap,
    /// Current line group
    current_group: Option<SourceMapGroup>,
}
impl SourceMapBuilder {
    /// Create a new builder.
    #[allow(dead_code)]
    pub fn new() -> Self {
        SourceMapBuilder {
            map: FullSourceMap::new(),
            current_group: Some(SourceMapGroup::new()),
        }
    }
    /// Add a source file.
    #[allow(dead_code)]
    pub fn source(mut self, name: &str) -> Self {
        self.map.add_source(name);
        self
    }
    /// Emit a mapping to the current line.
    #[allow(dead_code)]
    pub fn map_col(mut self, gen_col: u32, source_idx: u32, orig_line: u32, orig_col: u32) -> Self {
        let entry = SourceMapEntry::new(gen_col, source_idx, orig_line, orig_col);
        if let Some(ref mut group) = self.current_group {
            group.add(entry);
        }
        self
    }
    /// Advance to the next line.
    #[allow(dead_code)]
    pub fn next_line(mut self) -> Self {
        if let Some(group) = self.current_group.take() {
            self.map.add_group(group);
        }
        self.current_group = Some(SourceMapGroup::new());
        self
    }
    /// Finalise the builder and return the map.
    #[allow(dead_code)]
    pub fn build(mut self) -> FullSourceMap {
        if let Some(group) = self.current_group.take() {
            self.map.add_group(group);
        }
        self.map
    }
}
/// A source map statistics record.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Default)]
pub struct SourceMapStatsExt {
    /// Total segments across all groups
    pub total_segments: usize,
    /// Number of source files referenced
    pub source_count: usize,
    /// Number of symbol names
    pub name_count: usize,
    /// Number of groups (generated lines)
    pub group_count: usize,
}
impl SourceMapStatsExt {
    /// Compute stats from a FullSourceMap.
    #[allow(dead_code)]
    pub fn from_map(map: &FullSourceMap) -> Self {
        SourceMapStatsExt {
            total_segments: map.total_segments(),
            source_count: map.sources.len(),
            name_count: map.names.len(),
            group_count: map.groups.len(),
        }
    }
    /// Format the stats.
    #[allow(dead_code)]
    pub fn format(&self) -> String {
        format!(
            "segments={} sources={} names={} groups={}",
            self.total_segments, self.source_count, self.name_count, self.group_count
        )
    }
}
/// Options for source map generation.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct SourceMapOptions {
    /// Whether to include the `sourceRoot` field.
    pub source_root: Option<String>,
    /// Whether to embed source content inline.
    pub embed_sources: bool,
    /// Whether to include names mapping.
    pub include_names: bool,
}
impl SourceMapOptions {
    /// Create default options.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Enable source embedding.
    #[allow(dead_code)]
    pub fn with_embedded_sources(mut self) -> Self {
        self.embed_sources = true;
        self
    }
    /// Set source root.
    #[allow(dead_code)]
    pub fn with_source_root(mut self, root: &str) -> Self {
        self.source_root = Some(root.to_string());
        self
    }
}
/// A complete source map (V3 format).
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct FullSourceMap {
    /// Source file names
    pub sources: Vec<String>,
    /// Symbol names
    pub names: Vec<String>,
    /// Groups per generated line
    pub groups: Vec<SourceMapGroup>,
}
impl FullSourceMap {
    /// Create a new empty source map.
    #[allow(dead_code)]
    pub fn new() -> Self {
        FullSourceMap {
            sources: Vec::new(),
            names: Vec::new(),
            groups: Vec::new(),
        }
    }
    /// Add a source file.
    #[allow(dead_code)]
    pub fn add_source(&mut self, source: &str) -> u32 {
        let idx = self.sources.len() as u32;
        self.sources.push(source.to_string());
        idx
    }
    /// Add a symbol name.
    #[allow(dead_code)]
    pub fn add_name(&mut self, name: &str) -> u32 {
        let idx = self.names.len() as u32;
        self.names.push(name.to_string());
        idx
    }
    /// Add a line group.
    #[allow(dead_code)]
    pub fn add_group(&mut self, group: SourceMapGroup) {
        self.groups.push(group);
    }
    /// Returns the total number of mapped segments.
    #[allow(dead_code)]
    pub fn total_segments(&self) -> usize {
        self.groups.iter().map(|g| g.entries.len()).sum()
    }
    /// Look up the source location for a generated (line, col).
    #[allow(dead_code)]
    pub fn lookup(&self, gen_line: usize, gen_col: u32) -> Option<&SourceMapEntry> {
        let group = self.groups.get(gen_line)?;
        group.entries.iter().rev().find(|e| e.gen_col <= gen_col)
    }
}
/// A WASM binary source annotation.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct WasmAnnotation {
    /// WASM instruction offset
    pub wasm_offset: u32,
    /// Source file index
    pub source_idx: u32,
    /// Source line
    pub line: u32,
    /// Source column
    pub col: u32,
    /// Optional function name
    pub func_name: Option<String>,
}
impl WasmAnnotation {
    /// Create a new annotation.
    #[allow(dead_code)]
    pub fn new(wasm_offset: u32, source_idx: u32, line: u32, col: u32) -> Self {
        WasmAnnotation {
            wasm_offset,
            source_idx,
            line,
            col,
            func_name: None,
        }
    }
    /// Set the function name.
    #[allow(dead_code)]
    pub fn with_func(mut self, name: &str) -> Self {
        self.func_name = Some(name.to_string());
        self
    }
}
/// Source map statistics.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct SourceMapStats {
    /// Total mapping count.
    pub mapping_count: usize,
    /// Number of unique source files.
    pub source_count: usize,
    /// Number of generated lines covered.
    pub line_count: usize,
    /// Estimated encoded size in bytes.
    pub encoded_size: usize,
}
impl SourceMapStats {
    /// Compute stats from a source map.
    #[allow(dead_code)]
    pub fn from_map(sm: &SourceMap) -> Self {
        let encoded = sm.encode_mappings();
        let line_count = sm
            .mappings
            .iter()
            .map(|m| m.generated_line)
            .max()
            .map(|l| l as usize + 1)
            .unwrap_or(0);
        Self {
            mapping_count: sm.mappings.len(),
            source_count: sm.sources.len(),
            line_count,
            encoded_size: encoded.len(),
        }
    }
}
/// A multi-file source map that maps one generated file to multiple sources.
#[allow(dead_code)]
pub struct MultiFileSourceMap {
    maps: Vec<(String, SourceMap)>,
}
impl MultiFileSourceMap {
    /// Create an empty multi-file source map.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { maps: Vec::new() }
    }
    /// Add a named source map.
    #[allow(dead_code)]
    pub fn add(&mut self, name: &str, sm: SourceMap) {
        self.maps.push((name.to_string(), sm));
    }
    /// Retrieve a source map by name.
    #[allow(dead_code)]
    pub fn get(&self, name: &str) -> Option<&SourceMap> {
        self.maps.iter().find(|(n, _)| n == name).map(|(_, sm)| sm)
    }
    /// Number of maps.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.maps.len()
    }
    /// Whether the collection is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.maps.is_empty()
    }
    /// Generate a JSON index (list of names and URLs).
    #[allow(dead_code)]
    pub fn to_index_json(&self) -> String {
        let entries: Vec<String> = self
            .maps
            .iter()
            .map(|(name, _)| format!("\"{}\"", name))
            .collect();
        format!("[{}]", entries.join(","))
    }
}
/// A source position in (line, col) form.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourcePos2 {
    /// Line number (1-based)
    pub line: u32,
    /// Column number (1-based)
    pub col: u32,
}
impl SourcePos2 {
    /// Create a new position.
    #[allow(dead_code)]
    pub fn new(line: u32, col: u32) -> Self {
        SourcePos2 { line, col }
    }
    /// Returns true if this position comes before another.
    #[allow(dead_code)]
    pub fn before(&self, other: &SourcePos2) -> bool {
        self.line < other.line || (self.line == other.line && self.col < other.col)
    }
}
/// A single source mapping entry.
#[derive(Debug, Clone)]
pub struct SourceMapping {
    pub generated_line: u32,
    pub generated_col: u32,
    pub source_line: u32,
    pub source_col: u32,
    pub source_file: String,
}
impl SourceMapping {
    pub fn new(gen_line: u32, gen_col: u32, src_line: u32, src_col: u32, file: &str) -> Self {
        Self {
            generated_line: gen_line,
            generated_col: gen_col,
            source_line: src_line,
            source_col: src_col,
            source_file: file.to_string(),
        }
    }
}
/// A source map entry enriched with absolute (non-delta) positions.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AbsoluteMapping {
    pub gen_line: u32,
    pub gen_col: u32,
    pub src_file: usize,
    pub src_line: u32,
    pub src_col: u32,
}
impl AbsoluteMapping {
    /// Create from a `SourceMapping`.
    #[allow(dead_code)]
    pub fn from_mapping(m: &SourceMapping, src_file_idx: usize) -> Self {
        Self {
            gen_line: m.generated_line,
            gen_col: m.generated_col,
            src_file: src_file_idx,
            src_line: m.source_line,
            src_col: m.source_col,
        }
    }
}
/// A source map index entry for binary search.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SourceMapIndex {
    /// Sorted list of (gen_line, gen_col, mapping_idx).
    entries: Vec<(u32, u32, usize)>,
}
impl SourceMapIndex {
    /// Build an index from a `SourceMap`.
    #[allow(dead_code)]
    pub fn build(sm: &SourceMap) -> Self {
        let mut entries: Vec<(u32, u32, usize)> = sm
            .mappings
            .iter()
            .enumerate()
            .map(|(i, m)| (m.generated_line, m.generated_col, i))
            .collect();
        entries.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
        Self { entries }
    }
    /// Look up by generated position using binary search.
    #[allow(dead_code)]
    pub fn lookup(&self, line: u32, col: u32) -> Option<usize> {
        let target = (line, col, usize::MAX);
        let idx = self
            .entries
            .partition_point(|e| (e.0, e.1) <= (target.0, target.1));
        if idx == 0 {
            return None;
        }
        let (l, c, mapping_idx) = self.entries[idx - 1];
        if (l == line && c <= col) || l < line {
            Some(mapping_idx)
        } else {
            None
        }
    }
    /// Number of indexed entries.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Whether the index is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
/// A VLQ codec for source map encoding.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct VlqCodec;
impl VlqCodec {
    const BASE64_CHARS: &'static [u8] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    /// Encode an i32 value as a VLQ base64 string.
    #[allow(dead_code)]
    pub fn encode(value: i32) -> String {
        let mut vlq = if value < 0 {
            ((-value) << 1) | 1
        } else {
            value << 1
        };
        let mut result = String::new();
        loop {
            let mut digit = vlq & 0x1F;
            vlq >>= 5;
            if vlq > 0 {
                digit |= 0x20;
            }
            result.push(Self::BASE64_CHARS[digit as usize] as char);
            if vlq == 0 {
                break;
            }
        }
        result
    }
    /// Encode a sequence of i32 values.
    #[allow(dead_code)]
    pub fn encode_seq(values: &[i32]) -> String {
        values.iter().map(|&v| Self::encode(v)).collect()
    }
}
/// A source map validator.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct SourceMapValidator;
impl SourceMapValidator {
    /// Validate that all source indices are in bounds.
    #[allow(dead_code)]
    pub fn validate_source_indices(map: &FullSourceMap) -> Vec<String> {
        let mut errors = Vec::new();
        let source_count = map.sources.len() as u32;
        for (line, group) in map.groups.iter().enumerate() {
            for entry in &group.entries {
                if entry.source_idx >= source_count {
                    errors.push(format!(
                        "line {}: source index {} out of bounds (max {})",
                        line,
                        entry.source_idx,
                        source_count.saturating_sub(1)
                    ));
                }
            }
        }
        errors
    }
}
/// A generated position (line, column).
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedPosition {
    pub line: u32,
    pub col: u32,
}
impl GeneratedPosition {
    /// Create a new generated position.
    #[allow(dead_code)]
    pub fn new(line: u32, col: u32) -> Self {
        Self { line, col }
    }
}
/// Builder for constructing a WASM source map incrementally.
pub struct WasmSourceMapBuilder {
    pub source_map: SourceMap,
    pub current_file: String,
}
impl WasmSourceMapBuilder {
    pub fn new(file: &str) -> Self {
        let mut sm = SourceMap::new();
        sm.add_source(file);
        Self {
            source_map: sm,
            current_file: file.to_string(),
        }
    }
    /// Record a token mapping from generated position to source position.
    pub fn record_token(&mut self, gen_line: u32, gen_col: u32, src_line: u32, src_col: u32) {
        let m = SourceMapping::new(gen_line, gen_col, src_line, src_col, &self.current_file);
        self.source_map.add_mapping(m);
    }
    /// Finalize and return the completed source map.
    pub fn build(self) -> SourceMap {
        self.source_map
    }
}
/// Extended builder methods.
impl WasmSourceMapBuilder {
    /// Record a mapping with a name index.
    #[allow(dead_code)]
    pub fn record_named(
        &mut self,
        gen_line: u32,
        gen_col: u32,
        src_line: u32,
        src_col: u32,
        _name_idx: usize,
    ) {
        let m = SourceMapping::new(gen_line, gen_col, src_line, src_col, &self.current_file);
        self.source_map.add_mapping(m);
    }
    /// Switch to a different source file.
    #[allow(dead_code)]
    pub fn set_file(&mut self, file: &str) {
        self.source_map.add_source(file);
        self.current_file = file.to_string();
    }
    /// Number of mappings recorded so far.
    #[allow(dead_code)]
    pub fn mapping_count(&self) -> usize {
        self.source_map.mappings.len()
    }
    /// Generate JSON from the current state (non-consuming).
    #[allow(dead_code)]
    pub fn to_json(&self) -> String {
        self.source_map.to_json()
    }
}
/// A source map diff: the changes between two source map versions.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct SourceMapDiff {
    pub added: Vec<SourceMapping>,
    pub removed: Vec<SourceMapping>,
}
impl SourceMapDiff {
    /// Compute the diff between an old and new source map.
    #[allow(dead_code)]
    pub fn compute(old: &SourceMap, new: &SourceMap) -> Self {
        let old_keys: std::collections::HashSet<(u32, u32)> = old
            .mappings
            .iter()
            .map(|m| (m.generated_line, m.generated_col))
            .collect();
        let new_keys: std::collections::HashSet<(u32, u32)> = new
            .mappings
            .iter()
            .map(|m| (m.generated_line, m.generated_col))
            .collect();
        let added = new
            .mappings
            .iter()
            .filter(|m| !old_keys.contains(&(m.generated_line, m.generated_col)))
            .cloned()
            .collect();
        let removed = old
            .mappings
            .iter()
            .filter(|m| !new_keys.contains(&(m.generated_line, m.generated_col)))
            .cloned()
            .collect();
        Self { added, removed }
    }
    /// Whether there are any differences.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty()
    }
    /// Total change count.
    #[allow(dead_code)]
    pub fn change_count(&self) -> usize {
        self.added.len() + self.removed.len()
    }
}
