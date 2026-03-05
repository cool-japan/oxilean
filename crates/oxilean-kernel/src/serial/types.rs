//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::io::Write;

use super::functions::kind_tags;
use super::functions::{
    count_by_kind, has_valid_magic, peek_decl_count, peek_version, HEADER_SIZE, MAGIC, VERSION,
};

/// A checksummed binary blob validator.
#[allow(dead_code)]
pub struct BlobValidator {
    expected_checksum: u32,
}
#[allow(dead_code)]
impl BlobValidator {
    /// Create a validator for a given expected checksum.
    pub fn new(expected_checksum: u32) -> Self {
        BlobValidator { expected_checksum }
    }
    /// Compute the FNV-1a 32-bit checksum of data.
    pub fn compute_checksum(data: &[u8]) -> u32 {
        let mut hash: u32 = 2_166_136_261;
        for &b in data {
            hash ^= b as u32;
            hash = hash.wrapping_mul(16_777_619);
        }
        hash
    }
    /// Validate the given data against the expected checksum.
    pub fn validate(&self, data: &[u8]) -> bool {
        Self::compute_checksum(data) == self.expected_checksum
    }
    /// Return the expected checksum.
    pub fn expected(&self) -> u32 {
        self.expected_checksum
    }
}
/// Statistics about a serialized file.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct FileStats {
    pub total_decls: u32,
    pub axioms: u32,
    pub definitions: u32,
    pub theorems: u32,
    pub opaques: u32,
    pub inductives: u32,
    pub other: u32,
    pub total_bytes: usize,
}
#[allow(dead_code)]
impl FileStats {
    /// Build stats from a list of declarations and total byte size.
    pub fn from_decls(decls: &[SerialDecl], total_bytes: usize) -> Self {
        let counts = count_by_kind(decls);
        FileStats {
            total_decls: decls.len() as u32,
            axioms: counts[kind_tags::AXIOM as usize],
            definitions: counts[kind_tags::DEFINITION as usize],
            theorems: counts[kind_tags::THEOREM as usize],
            opaques: counts[kind_tags::OPAQUE as usize],
            inductives: counts[kind_tags::INDUCTIVE as usize],
            other: counts[kind_tags::OTHER as usize],
            total_bytes,
        }
    }
    /// Return the average bytes per declaration.
    pub fn bytes_per_decl(&self) -> f64 {
        if self.total_decls == 0 {
            0.0
        } else {
            self.total_bytes as f64 / self.total_decls as f64
        }
    }
    /// Format a one-line summary.
    pub fn summary(&self) -> String {
        format!(
            "total={} axioms={} defs={} theorems={} inductives={} bytes={}",
            self.total_decls,
            self.axioms,
            self.definitions,
            self.theorems,
            self.inductives,
            self.total_bytes
        )
    }
}
/// Streaming binary reader for the OleanC format.
pub struct OleanReader<'a> {
    data: &'a [u8],
    pos: usize,
}
impl<'a> OleanReader<'a> {
    /// Create a new reader over the given byte slice.
    pub fn new(data: &'a [u8]) -> Self {
        OleanReader { data, pos: 0 }
    }
    /// Number of bytes remaining to be read.
    pub fn remaining(&self) -> usize {
        self.data.len().saturating_sub(self.pos)
    }
    fn ensure(&self, n: usize) -> Result<(), OleanError> {
        if self.remaining() < n {
            Err(OleanError::UnexpectedEof)
        } else {
            Ok(())
        }
    }
    /// Read and validate the OleanC file header.
    pub fn read_header(&mut self) -> Result<OleanHeader, OleanError> {
        self.ensure(HEADER_SIZE)?;
        let magic = &self.data[self.pos..self.pos + 4];
        if magic != MAGIC {
            return Err(OleanError::InvalidMagic);
        }
        self.pos += 4;
        let version = self.read_u32()?;
        if version != VERSION {
            return Err(OleanError::UnsupportedVersion(version));
        }
        let decl_count = self.read_u32()?;
        let metadata_offset = self.read_u64()?;
        Ok(OleanHeader {
            version,
            decl_count,
            metadata_offset,
        })
    }
    /// Read a length-prefixed UTF-8 string.
    pub fn read_string(&mut self) -> Result<String, OleanError> {
        let len = self.read_u32()? as usize;
        self.ensure(len)?;
        let bytes = self.data[self.pos..self.pos + len].to_vec();
        self.pos += len;
        Ok(String::from_utf8(bytes)?)
    }
    /// Read a single byte.
    pub fn read_u8(&mut self) -> Result<u8, OleanError> {
        self.ensure(1)?;
        let v = self.data[self.pos];
        self.pos += 1;
        Ok(v)
    }
    /// Read a u32 in little-endian order.
    pub fn read_u32(&mut self) -> Result<u32, OleanError> {
        self.ensure(4)?;
        let bytes: [u8; 4] = self.data[self.pos..self.pos + 4]
            .try_into()
            .expect("slice length must match array size");
        self.pos += 4;
        Ok(u32::from_le_bytes(bytes))
    }
    /// Read a u64 in little-endian order.
    pub fn read_u64(&mut self) -> Result<u64, OleanError> {
        self.ensure(8)?;
        let bytes: [u8; 8] = self.data[self.pos..self.pos + 8]
            .try_into()
            .expect("slice length must match array size");
        self.pos += 8;
        Ok(u64::from_le_bytes(bytes))
    }
    /// Read an i64 in little-endian order.
    pub fn read_i64(&mut self) -> Result<i64, OleanError> {
        self.ensure(8)?;
        let bytes: [u8; 8] = self.data[self.pos..self.pos + 8]
            .try_into()
            .expect("slice length must match array size");
        self.pos += 8;
        Ok(i64::from_le_bytes(bytes))
    }
    /// Read a bool from a single byte.
    pub fn read_bool(&mut self) -> Result<bool, OleanError> {
        Ok(self.read_u8()? != 0)
    }
}
/// A section header for variable-length sections in a binary format.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SectionHeader {
    pub tag: u8,
    pub length: u32,
    pub offset: u64,
}
#[allow(dead_code)]
impl SectionHeader {
    /// Create a new section header.
    pub fn new(tag: u8, length: u32, offset: u64) -> Self {
        SectionHeader {
            tag,
            length,
            offset,
        }
    }
    /// The size of a serialized section header (1 + 4 + 8 = 13 bytes).
    pub const SIZE: usize = 13;
    /// Write this header into a writer.
    pub fn write(&self, w: &mut OleanWriter) {
        w.write_u8(self.tag);
        w.write_u32(self.length);
        w.write_u64(self.offset);
    }
    /// Read a section header from a reader.
    pub fn read(r: &mut OleanReader<'_>) -> Result<Self, OleanError> {
        let tag = r.read_u8()?;
        let length = r.read_u32()?;
        let offset = r.read_u64()?;
        Ok(SectionHeader {
            tag,
            length,
            offset,
        })
    }
}
/// A streaming writer that computes a checksum as it writes.
#[allow(dead_code)]
pub struct ChecksummedWriter {
    inner: OleanWriter,
    running_hash: u32,
}
#[allow(dead_code)]
impl ChecksummedWriter {
    /// Create a new checksummed writer.
    pub fn new() -> Self {
        ChecksummedWriter {
            inner: OleanWriter::new(),
            running_hash: 2_166_136_261,
        }
    }
    /// Write a byte, updating the checksum.
    pub fn write_byte(&mut self, b: u8) {
        self.running_hash ^= b as u32;
        self.running_hash = self.running_hash.wrapping_mul(16_777_619);
        self.inner.write_u8(b);
    }
    /// Write multiple bytes.
    pub fn write_bytes(&mut self, data: &[u8]) {
        for &b in data {
            self.write_byte(b);
        }
    }
    /// Write a u32.
    pub fn write_u32(&mut self, v: u32) {
        self.write_bytes(&v.to_le_bytes());
    }
    /// Write a u64.
    pub fn write_u64(&mut self, v: u64) {
        self.write_bytes(&v.to_le_bytes());
    }
    /// Write a string (length-prefixed).
    pub fn write_string(&mut self, s: &str) {
        self.write_u32(s.len() as u32);
        self.write_bytes(s.as_bytes());
    }
    /// Return the current running checksum.
    pub fn current_checksum(&self) -> u32 {
        self.running_hash
    }
    /// Finish and return the data with appended checksum.
    pub fn finish_with_checksum(mut self) -> Vec<u8> {
        let checksum = self.running_hash;
        self.inner.write_u32(checksum);
        self.inner.finish()
    }
    /// Return the number of bytes written (excluding checksum trailer).
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    /// Returns whether the collection is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
/// Versioned name → id mapping for a name table section.
#[allow(dead_code)]
pub struct NameTable {
    entries: Vec<(String, u32)>,
}
#[allow(dead_code)]
impl NameTable {
    /// Create an empty name table.
    pub fn new() -> Self {
        NameTable {
            entries: Vec::new(),
        }
    }
    /// Intern a name; returns the assigned id.
    pub fn intern(&mut self, name: &str) -> u32 {
        if let Some(&(_, id)) = self.entries.iter().find(|(n, _)| n == name) {
            return id;
        }
        let id = self.entries.len() as u32;
        self.entries.push((name.to_string(), id));
        id
    }
    /// Look up a name by id.
    pub fn lookup_id(&self, id: u32) -> Option<&str> {
        self.entries
            .iter()
            .find(|(_, i)| *i == id)
            .map(|(n, _)| n.as_str())
    }
    /// Look up an id by name.
    pub fn lookup_name(&self, name: &str) -> Option<u32> {
        self.entries
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, id)| *id)
    }
    /// Serialize this table into an OleanWriter.
    pub fn write(&self, w: &mut OleanWriter) {
        w.write_u32(self.entries.len() as u32);
        for (name, id) in &self.entries {
            w.write_string(name);
            w.write_u32(*id);
        }
    }
    /// Deserialize from a reader.
    pub fn read(r: &mut OleanReader<'_>) -> Result<Self, OleanError> {
        let count = r.read_u32()? as usize;
        let mut t = NameTable::new();
        for _ in 0..count {
            let name = r.read_string()?;
            let id = r.read_u32()?;
            t.entries.push((name, id));
        }
        Ok(t)
    }
    /// Return the number of interned names.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Return whether the table is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Return all names in intern order.
    pub fn names(&self) -> Vec<&str> {
        self.entries.iter().map(|(n, _)| n.as_str()).collect()
    }
}
/// A compact encoding for small sets of declaration kinds.
#[allow(dead_code)]
pub struct DeclKindSet {
    mask: u8,
}
#[allow(dead_code)]
impl DeclKindSet {
    /// Create an empty set.
    pub fn new() -> Self {
        DeclKindSet { mask: 0 }
    }
    /// Add a kind tag to the set.
    pub fn add(&mut self, tag: u8) {
        if tag < 8 {
            self.mask |= 1 << tag;
        }
    }
    /// Return whether a tag is in the set.
    pub fn contains(&self, tag: u8) -> bool {
        tag < 8 && (self.mask >> tag) & 1 != 0
    }
    /// Return the raw bitmask.
    pub fn mask(&self) -> u8 {
        self.mask
    }
    /// Return the number of distinct kinds in the set.
    pub fn count(&self) -> u32 {
        self.mask.count_ones()
    }
    /// Return whether the set is empty.
    pub fn is_empty(&self) -> bool {
        self.mask == 0
    }
    /// Write to a writer.
    pub fn write(&self, w: &mut OleanWriter) {
        w.write_u8(self.mask);
    }
    /// Read from a reader.
    pub fn read(r: &mut OleanReader<'_>) -> Result<Self, OleanError> {
        Ok(DeclKindSet { mask: r.read_u8()? })
    }
}
/// Error type for OleanC serialization/deserialization.
#[derive(Debug)]
pub enum OleanError {
    InvalidMagic,
    UnsupportedVersion(u32),
    UnexpectedEof,
    InvalidUtf8(std::string::FromUtf8Error),
    InvalidDeclKind(u8),
    IoError(std::io::Error),
}
/// A string interning table for efficient repeated string serialization.
#[allow(dead_code)]
pub struct StringPool {
    strings: Vec<String>,
}
#[allow(dead_code)]
impl StringPool {
    /// Create an empty pool.
    pub fn new() -> Self {
        StringPool {
            strings: Vec::new(),
        }
    }
    /// Intern a string, returning its index.
    pub fn intern(&mut self, s: &str) -> u32 {
        if let Some(idx) = self.strings.iter().position(|x| x == s) {
            return idx as u32;
        }
        let idx = self.strings.len() as u32;
        self.strings.push(s.to_string());
        idx
    }
    /// Retrieve a string by index.
    pub fn get(&self, idx: u32) -> Option<&str> {
        self.strings.get(idx as usize).map(|s| s.as_str())
    }
    /// Return the number of interned strings.
    pub fn len(&self) -> usize {
        self.strings.len()
    }
    /// Return whether the pool is empty.
    pub fn is_empty(&self) -> bool {
        self.strings.is_empty()
    }
    /// Serialize the pool.
    pub fn write(&self, w: &mut OleanWriter) {
        w.write_u32(self.strings.len() as u32);
        for s in &self.strings {
            w.write_string(s);
        }
    }
    /// Deserialize a pool.
    pub fn read(r: &mut OleanReader<'_>) -> Result<Self, OleanError> {
        let count = r.read_u32()? as usize;
        let mut pool = StringPool::new();
        for _ in 0..count {
            let s = r.read_string()?;
            pool.strings.push(s);
        }
        Ok(pool)
    }
    /// Return all strings.
    pub fn all_strings(&self) -> &[String] {
        &self.strings
    }
}
/// An index of declaration names for fast lookup.
#[allow(dead_code)]
pub struct DeclIndex {
    names: Vec<(String, u32)>,
}
#[allow(dead_code)]
impl DeclIndex {
    /// Create an empty index.
    pub fn new() -> Self {
        DeclIndex { names: Vec::new() }
    }
    /// Add a name with its byte offset in the binary file.
    pub fn add(&mut self, name: &str, offset: u32) {
        self.names.push((name.to_string(), offset));
    }
    /// Look up the offset for a name.
    pub fn find_offset(&self, name: &str) -> Option<u32> {
        self.names.iter().find(|(n, _)| n == name).map(|(_, o)| *o)
    }
    /// Return whether the index contains a name.
    pub fn contains(&self, name: &str) -> bool {
        self.names.iter().any(|(n, _)| n == name)
    }
    /// Return the number of indexed names.
    pub fn len(&self) -> usize {
        self.names.len()
    }
    /// Return whether the index is empty.
    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }
    /// Return names in sorted order for binary search.
    pub fn sorted_names(&self) -> Vec<&str> {
        let mut v: Vec<&str> = self.names.iter().map(|(n, _)| n.as_str()).collect();
        v.sort_unstable();
        v
    }
    /// Binary search for a name (requires sorted order).
    pub fn binary_search(&self, name: &str) -> Option<u32> {
        let mut lo = 0usize;
        let mut hi = self.names.len();
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            match self.names[mid].0.as_str().cmp(name) {
                std::cmp::Ordering::Equal => return Some(self.names[mid].1),
                std::cmp::Ordering::Less => lo = mid + 1,
                std::cmp::Ordering::Greater => hi = mid,
            }
        }
        None
    }
    /// Serialize the index.
    pub fn write(&self, w: &mut OleanWriter) {
        w.write_u32(self.names.len() as u32);
        for (name, offset) in &self.names {
            w.write_string(name);
            w.write_u32(*offset);
        }
    }
    /// Deserialize the index.
    pub fn read(r: &mut OleanReader<'_>) -> Result<Self, OleanError> {
        let count = r.read_u32()? as usize;
        let mut idx = DeclIndex::new();
        for _ in 0..count {
            let name = r.read_string()?;
            let offset = r.read_u32()?;
            idx.names.push((name, offset));
        }
        Ok(idx)
    }
}
/// Writes a tagged union of declaration metadata.
#[allow(dead_code)]
pub struct MetadataWriter {
    buf: OleanWriter,
    entry_count: u32,
}
#[allow(dead_code)]
impl MetadataWriter {
    /// Create a new metadata writer.
    pub fn new() -> Self {
        MetadataWriter {
            buf: OleanWriter::new(),
            entry_count: 0,
        }
    }
    /// Write a key-value pair where value is a string.
    pub fn write_str_entry(&mut self, key: &str, value: &str) {
        self.buf.write_u8(0);
        self.buf.write_string(key);
        self.buf.write_string(value);
        self.entry_count += 1;
    }
    /// Write a key-value pair where value is a u64.
    pub fn write_u64_entry(&mut self, key: &str, value: u64) {
        self.buf.write_u8(1);
        self.buf.write_string(key);
        self.buf.write_u64(value);
        self.entry_count += 1;
    }
    /// Write a key-value pair where value is a bool.
    pub fn write_bool_entry(&mut self, key: &str, value: bool) {
        self.buf.write_u8(2);
        self.buf.write_string(key);
        self.buf.write_bool(value);
        self.entry_count += 1;
    }
    /// Return the number of entries written.
    pub fn entry_count(&self) -> u32 {
        self.entry_count
    }
    /// Finish and produce a section with a count prefix.
    pub fn finish(self) -> Vec<u8> {
        let mut w = OleanWriter::new();
        w.write_u32(self.entry_count);
        let inner_bytes = self.buf.finish();
        w.buf.extend_from_slice(&inner_bytes);
        w.finish()
    }
}
/// A table of section headers for a multi-section binary file.
#[allow(dead_code)]
pub struct SectionTable {
    headers: Vec<SectionHeader>,
}
#[allow(dead_code)]
impl SectionTable {
    /// Create an empty section table.
    pub fn new() -> Self {
        SectionTable {
            headers: Vec::new(),
        }
    }
    /// Add a section header.
    pub fn add(&mut self, header: SectionHeader) {
        self.headers.push(header);
    }
    /// Look up a section header by tag.
    pub fn find(&self, tag: u8) -> Option<&SectionHeader> {
        self.headers.iter().find(|h| h.tag == tag)
    }
    /// Return the number of sections.
    pub fn len(&self) -> usize {
        self.headers.len()
    }
    /// Return whether the table is empty.
    pub fn is_empty(&self) -> bool {
        self.headers.is_empty()
    }
    /// Serialize the entire table.
    pub fn write(&self, w: &mut OleanWriter) {
        w.write_u32(self.headers.len() as u32);
        for h in &self.headers {
            h.write(w);
        }
    }
    /// Deserialize a section table from a reader.
    pub fn read(r: &mut OleanReader<'_>) -> Result<Self, OleanError> {
        let count = r.read_u32()? as usize;
        let mut table = SectionTable::new();
        for _ in 0..count {
            table.add(SectionHeader::read(r)?);
        }
        Ok(table)
    }
}
/// An OleanReader with checkpoint/rollback support.
#[allow(dead_code)]
pub struct CheckpointedReader<'a> {
    data: &'a [u8],
    pos: usize,
    checkpoint: Option<usize>,
}
#[allow(dead_code)]
impl<'a> CheckpointedReader<'a> {
    /// Create a new reader.
    pub fn new(data: &'a [u8]) -> Self {
        CheckpointedReader {
            data,
            pos: 0,
            checkpoint: None,
        }
    }
    /// Save the current position as a checkpoint.
    pub fn save(&mut self) {
        self.checkpoint = Some(self.pos);
    }
    /// Roll back to the last checkpoint.
    pub fn rollback(&mut self) -> bool {
        if let Some(cp) = self.checkpoint {
            self.pos = cp;
            true
        } else {
            false
        }
    }
    /// Return remaining bytes.
    pub fn remaining(&self) -> usize {
        self.data.len().saturating_sub(self.pos)
    }
    /// Read a u8.
    pub fn read_u8(&mut self) -> Result<u8, OleanError> {
        if self.remaining() < 1 {
            return Err(OleanError::UnexpectedEof);
        }
        let v = self.data[self.pos];
        self.pos += 1;
        Ok(v)
    }
    /// Read a u32 little-endian.
    pub fn read_u32(&mut self) -> Result<u32, OleanError> {
        if self.remaining() < 4 {
            return Err(OleanError::UnexpectedEof);
        }
        let bytes: [u8; 4] = self.data[self.pos..self.pos + 4]
            .try_into()
            .expect("slice length must match array size");
        self.pos += 4;
        Ok(u32::from_le_bytes(bytes))
    }
    /// Read a length-prefixed string.
    pub fn read_string(&mut self) -> Result<String, OleanError> {
        let len = self.read_u32()? as usize;
        if self.remaining() < len {
            return Err(OleanError::UnexpectedEof);
        }
        let s = String::from_utf8(self.data[self.pos..self.pos + len].to_vec())?;
        self.pos += len;
        Ok(s)
    }
    /// Current position in bytes.
    pub fn pos(&self) -> usize {
        self.pos
    }
}
/// A buffered writer that flushes to a Vec when full.
#[allow(dead_code)]
pub struct BufferedOleanWriter {
    buf: Vec<u8>,
    flush_threshold: usize,
    total_written: usize,
}
#[allow(dead_code)]
impl BufferedOleanWriter {
    /// Create a buffered writer with a given flush threshold.
    pub fn new(flush_threshold: usize) -> Self {
        BufferedOleanWriter {
            buf: Vec::with_capacity(flush_threshold),
            flush_threshold,
            total_written: 0,
        }
    }
    /// Write a byte.
    pub fn write_u8(&mut self, b: u8) {
        self.buf.push(b);
        self.total_written += 1;
    }
    /// Write a u32.
    pub fn write_u32(&mut self, v: u32) {
        for b in v.to_le_bytes() {
            self.write_u8(b);
        }
    }
    /// Write a u64.
    pub fn write_u64(&mut self, v: u64) {
        for b in v.to_le_bytes() {
            self.write_u8(b);
        }
    }
    /// Write a length-prefixed string.
    pub fn write_string(&mut self, s: &str) {
        self.write_u32(s.len() as u32);
        for b in s.as_bytes() {
            self.write_u8(*b);
        }
    }
    /// Return total bytes written.
    pub fn total_written(&self) -> usize {
        self.total_written
    }
    /// Return current buffered bytes (not yet flushed to output).
    pub fn buffered(&self) -> usize {
        self.buf.len()
    }
    /// Flush the buffer and return all accumulated bytes.
    pub fn flush(self) -> Vec<u8> {
        self.buf
    }
    /// Return whether the buffer is over the flush threshold.
    pub fn should_flush(&self) -> bool {
        self.buf.len() >= self.flush_threshold
    }
}
/// A merge strategy for combining two serialized declaration lists.
#[allow(dead_code)]
pub enum MergeStrategy {
    /// Keep all declarations from both lists.
    Union,
    /// Keep only declarations present in both (by name).
    Intersection,
    /// Prefer the first list; only add from second if not in first.
    PreferFirst,
    /// Prefer the second list; only add from first if not in second.
    PreferSecond,
}
/// Diagnostic information about a parsed OleanC file.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FormatDiagnostics {
    pub version: u32,
    pub decl_count: u32,
    pub total_bytes: usize,
    pub magic_ok: bool,
    pub checksum_ok: Option<bool>,
    pub sections: Vec<String>,
}
#[allow(dead_code)]
impl FormatDiagnostics {
    /// Parse diagnostics from raw bytes.
    pub fn from_bytes(data: &[u8]) -> Self {
        let magic_ok = has_valid_magic(data);
        let version = peek_version(data).unwrap_or(0);
        let decl_count = peek_decl_count(data).unwrap_or(0);
        FormatDiagnostics {
            version,
            decl_count,
            total_bytes: data.len(),
            magic_ok,
            checksum_ok: None,
            sections: Vec::new(),
        }
    }
    /// Format a diagnostic report.
    pub fn report(&self) -> String {
        format!(
            "magic={} version={} decls={} bytes={}",
            self.magic_ok, self.version, self.decl_count, self.total_bytes
        )
    }
    /// Return whether the file appears well-formed.
    pub fn is_well_formed(&self) -> bool {
        self.magic_ok && self.version > 0
    }
}
/// Parsed OleanC file header.
#[derive(Debug, Clone)]
pub struct OleanHeader {
    pub version: u32,
    pub decl_count: u32,
    pub metadata_offset: u64,
}
/// A serializable record for a kernel declaration.
#[derive(Debug, Clone, PartialEq)]
pub enum SerialDecl {
    Axiom {
        name: String,
        kind_tag: u8,
    },
    Definition {
        name: String,
        kind_tag: u8,
    },
    Theorem {
        name: String,
        kind_tag: u8,
    },
    Opaque {
        name: String,
        kind_tag: u8,
    },
    Inductive {
        name: String,
        ctor_count: u32,
        kind_tag: u8,
    },
    Other {
        name: String,
        kind_tag: u8,
    },
}
impl SerialDecl {
    /// Return the declaration's name.
    pub fn name(&self) -> &str {
        match self {
            SerialDecl::Axiom { name, .. } => name,
            SerialDecl::Definition { name, .. } => name,
            SerialDecl::Theorem { name, .. } => name,
            SerialDecl::Opaque { name, .. } => name,
            SerialDecl::Inductive { name, .. } => name,
            SerialDecl::Other { name, .. } => name,
        }
    }
    /// Return the declaration's kind tag byte.
    pub fn kind_tag(&self) -> u8 {
        match self {
            SerialDecl::Axiom { kind_tag, .. } => *kind_tag,
            SerialDecl::Definition { kind_tag, .. } => *kind_tag,
            SerialDecl::Theorem { kind_tag, .. } => *kind_tag,
            SerialDecl::Opaque { kind_tag, .. } => *kind_tag,
            SerialDecl::Inductive { kind_tag, .. } => *kind_tag,
            SerialDecl::Other { kind_tag, .. } => *kind_tag,
        }
    }
}
/// Streaming binary writer for the OleanC format.
pub struct OleanWriter {
    buf: Vec<u8>,
}
impl OleanWriter {
    /// Create a new, empty writer.
    pub fn new() -> Self {
        OleanWriter { buf: Vec::new() }
    }
    /// Write the OleanC file header.
    ///
    /// The metadata section offset is set to `HEADER_SIZE` (immediately after the header)
    /// when no additional body is present; callers may update it afterwards.
    pub fn write_header(&mut self, decl_count: u32) -> &mut Self {
        self.buf.extend_from_slice(MAGIC);
        self.write_u32(VERSION);
        self.write_u32(decl_count);
        self.write_u64(HEADER_SIZE as u64);
        self
    }
    /// Write a length-prefixed UTF-8 string (u32 length + bytes).
    pub fn write_string(&mut self, s: &str) -> &mut Self {
        let bytes = s.as_bytes();
        self.write_u32(bytes.len() as u32);
        self.buf.extend_from_slice(bytes);
        self
    }
    /// Write a single byte.
    pub fn write_u8(&mut self, v: u8) -> &mut Self {
        self.buf.push(v);
        self
    }
    /// Write a u32 in little-endian order.
    pub fn write_u32(&mut self, v: u32) -> &mut Self {
        self.buf.extend_from_slice(&v.to_le_bytes());
        self
    }
    /// Write a u64 in little-endian order.
    pub fn write_u64(&mut self, v: u64) -> &mut Self {
        self.buf.extend_from_slice(&v.to_le_bytes());
        self
    }
    /// Write an i64 in little-endian order.
    pub fn write_i64(&mut self, v: i64) -> &mut Self {
        self.buf.extend_from_slice(&v.to_le_bytes());
        self
    }
    /// Write a bool as a single byte (0 or 1).
    pub fn write_bool(&mut self, v: bool) -> &mut Self {
        self.write_u8(if v { 1 } else { 0 })
    }
    /// Return the current number of bytes written.
    pub fn len(&self) -> usize {
        self.buf.len()
    }
    /// Return `true` if no bytes have been written yet.
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }
    /// Consume the writer and return the accumulated bytes.
    pub fn finish(self) -> Vec<u8> {
        self.buf
    }
}
/// A delta-compressed list of u32 values (successive differences).
#[allow(dead_code)]
pub struct DeltaList {
    deltas: Vec<i32>,
    base: u32,
}
#[allow(dead_code)]
impl DeltaList {
    /// Encode a sorted list of u32 values as delta-compressed form.
    pub fn encode(values: &[u32]) -> Self {
        let mut deltas = Vec::with_capacity(values.len());
        let mut prev = 0u32;
        for &v in values {
            let delta = v as i64 - prev as i64;
            deltas.push(delta as i32);
            prev = v;
        }
        DeltaList {
            deltas,
            base: values.first().copied().unwrap_or(0),
        }
    }
    /// Decode back to a list of u32 values.
    pub fn decode(&self) -> Vec<u32> {
        let mut result = Vec::with_capacity(self.deltas.len());
        let mut cur: i64 = 0;
        for &d in &self.deltas {
            cur += d as i64;
            result.push(cur as u32);
        }
        result
    }
    /// Return the number of encoded values.
    pub fn len(&self) -> usize {
        self.deltas.len()
    }
    /// Return whether the list is empty.
    pub fn is_empty(&self) -> bool {
        self.deltas.is_empty()
    }
    /// Write to an OleanWriter.
    pub fn write(&self, w: &mut OleanWriter) {
        w.write_u32(self.base);
        w.write_u32(self.deltas.len() as u32);
        for &d in &self.deltas {
            w.write_i64(d as i64);
        }
    }
    /// Read from an OleanReader.
    pub fn read(r: &mut OleanReader<'_>) -> Result<Self, OleanError> {
        let base = r.read_u32()?;
        let count = r.read_u32()? as usize;
        let mut deltas = Vec::with_capacity(count);
        for _ in 0..count {
            deltas.push(r.read_i64()? as i32);
        }
        Ok(DeltaList { deltas, base })
    }
}
/// Reads metadata entries produced by `MetadataWriter`.
#[allow(dead_code)]
pub struct MetadataReader<'a> {
    inner: OleanReader<'a>,
    count: u32,
    read: u32,
}
#[allow(dead_code)]
impl<'a> MetadataReader<'a> {
    /// Create a reader from raw metadata bytes.
    pub fn new(data: &'a [u8]) -> Result<Self, OleanError> {
        let mut r = OleanReader::new(data);
        let count = r.read_u32()?;
        Ok(MetadataReader {
            inner: r,
            count,
            read: 0,
        })
    }
    /// Return whether more entries are available.
    pub fn has_next(&self) -> bool {
        self.read < self.count
    }
    /// Read the next (key, value) pair.
    pub fn next_entry(&mut self) -> Result<(String, MetadataValue), OleanError> {
        let tag = self.inner.read_u8()?;
        let key = self.inner.read_string()?;
        let value = match tag {
            0 => MetadataValue::Str(self.inner.read_string()?),
            1 => MetadataValue::U64(self.inner.read_u64()?),
            2 => MetadataValue::Bool(self.inner.read_bool()?),
            _ => return Err(OleanError::InvalidDeclKind(tag)),
        };
        self.read += 1;
        Ok((key, value))
    }
    /// Read all entries into a vec.
    pub fn read_all(&mut self) -> Result<Vec<(String, MetadataValue)>, OleanError> {
        let mut entries = Vec::new();
        while self.has_next() {
            entries.push(self.next_entry()?);
        }
        Ok(entries)
    }
}
/// A named section within an extended binary file.
#[allow(dead_code)]
pub struct BinarySection {
    pub header: SectionHeader,
    pub data: Vec<u8>,
}
#[allow(dead_code)]
impl BinarySection {
    /// Create a new section with raw data.
    pub fn new(tag: u8, data: Vec<u8>, offset: u64) -> Self {
        let length = data.len() as u32;
        BinarySection {
            header: SectionHeader::new(tag, length, offset),
            data,
        }
    }
    /// Serialize the section to bytes (header + data).
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut w = OleanWriter::new();
        self.header.write(&mut w);
        w.finish()
            .into_iter()
            .chain(self.data.iter().copied())
            .collect()
    }
    /// Return the section tag.
    pub fn tag(&self) -> u8 {
        self.header.tag
    }
    /// Return the section data length.
    pub fn data_len(&self) -> usize {
        self.data.len()
    }
}
/// A metadata entry value.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum MetadataValue {
    Str(String),
    U64(u64),
    Bool(bool),
}
/// A structured error with context for serialization failures.
#[allow(dead_code)]
#[derive(Debug)]
pub struct SerialError {
    pub inner: OleanError,
    pub context: String,
    pub byte_offset: usize,
}
#[allow(dead_code)]
impl SerialError {
    /// Create a new serial error.
    pub fn new(inner: OleanError, context: impl Into<String>, byte_offset: usize) -> Self {
        SerialError {
            inner,
            context: context.into(),
            byte_offset,
        }
    }
    /// Format a human-readable description.
    pub fn describe(&self) -> String {
        format!(
            "{} at byte {}: {}",
            self.context, self.byte_offset, self.inner
        )
    }
}
/// Checks binary compatibility between two OleanC files.
#[allow(dead_code)]
pub struct CompatibilityChecker {
    known_versions: Vec<u32>,
}
#[allow(dead_code)]
impl CompatibilityChecker {
    /// Create a checker that knows about the given versions.
    pub fn new(known_versions: Vec<u32>) -> Self {
        CompatibilityChecker { known_versions }
    }
    /// Return whether the version is known-compatible.
    pub fn is_compatible(&self, version: u32) -> bool {
        self.known_versions.contains(&version)
    }
    /// Return the latest known version.
    pub fn latest(&self) -> Option<u32> {
        self.known_versions.iter().max().copied()
    }
    /// Return whether an upgrade is needed from `old` to `new`.
    pub fn needs_upgrade(&self, old: u32, new: u32) -> bool {
        old < new && self.is_compatible(new)
    }
}
/// Computes the diff between two name lists.
#[allow(dead_code)]
pub struct DeclDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
#[allow(dead_code)]
impl DeclDiff {
    /// Compute the diff between `old_names` and `new_names`.
    pub fn compute(old_names: &[String], new_names: &[String]) -> Self {
        let added = new_names
            .iter()
            .filter(|n| !old_names.contains(n))
            .cloned()
            .collect();
        let removed = old_names
            .iter()
            .filter(|n| !new_names.contains(n))
            .cloned()
            .collect();
        let unchanged = old_names
            .iter()
            .filter(|n| new_names.contains(n))
            .cloned()
            .collect();
        DeclDiff {
            added,
            removed,
            unchanged,
        }
    }
    /// Return whether there are any changes.
    pub fn has_changes(&self) -> bool {
        !self.added.is_empty() || !self.removed.is_empty()
    }
    /// Format a summary.
    pub fn summary(&self) -> String {
        format!(
            "+{} -{} ={} declarations",
            self.added.len(),
            self.removed.len(),
            self.unchanged.len()
        )
    }
}
/// A multi-file archive of OleanC declarations.
#[allow(dead_code)]
pub struct OleanArchive {
    files: Vec<(String, Vec<SerialDecl>)>,
}
#[allow(dead_code)]
impl OleanArchive {
    /// Create an empty archive.
    pub fn new() -> Self {
        OleanArchive { files: Vec::new() }
    }
    /// Add a file to the archive.
    pub fn add_file(&mut self, name: impl Into<String>, decls: Vec<SerialDecl>) {
        self.files.push((name.into(), decls));
    }
    /// Return the total number of declarations across all files.
    pub fn total_decls(&self) -> usize {
        self.files.iter().map(|(_, d)| d.len()).sum()
    }
    /// Return the number of files.
    pub fn file_count(&self) -> usize {
        self.files.len()
    }
    /// Return all declaration names across all files.
    pub fn all_names(&self) -> Vec<&str> {
        self.files
            .iter()
            .flat_map(|(_, d)| d.iter().map(|decl| decl.name()))
            .collect()
    }
    /// Find a declaration by name across all files.
    pub fn find_decl(&self, name: &str) -> Option<(&str, &SerialDecl)> {
        for (fname, decls) in &self.files {
            if let Some(d) = decls.iter().find(|d| d.name() == name) {
                return Some((fname.as_str(), d));
            }
        }
        None
    }
    /// Return whether the archive is empty.
    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }
}
