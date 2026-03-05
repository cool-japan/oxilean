//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::functions::*;
use std::collections::{HashMap, VecDeque};

/// A line-oriented record writer.
///
/// Wraps a `BufferedWriter` and writes records as newline-separated strings.
#[allow(dead_code)]
pub struct RecordWriter {
    inner: BufferedWriter,
    record_count: usize,
}
impl RecordWriter {
    /// Create a record writer with the given flush threshold.
    #[allow(dead_code)]
    pub fn new(flush_threshold: usize) -> Self {
        Self {
            inner: BufferedWriter::new(flush_threshold),
            record_count: 0,
        }
    }
    /// Write a record (followed by newline).
    #[allow(dead_code)]
    pub fn write_record(&mut self, record: &str) {
        self.inner.writeln(record);
        self.record_count += 1;
    }
    /// Number of records written.
    #[allow(dead_code)]
    pub fn record_count(&self) -> usize {
        self.record_count
    }
    /// Total bytes written (including newlines).
    #[allow(dead_code)]
    pub fn total_written(&self) -> usize {
        self.inner.total_written()
    }
    /// Flush the internal buffer.
    #[allow(dead_code)]
    pub fn flush(&mut self) {
        self.inner.flush();
    }
}
/// A capability token representing a permission level for IO operations.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Capability {
    /// The permission level (higher = more privileged).
    pub level: u32,
    /// Human-readable description.
    pub description: String,
}
#[allow(dead_code)]
impl Capability {
    /// Create a new capability token.
    pub fn new(level: u32, description: impl Into<String>) -> Self {
        Self {
            level,
            description: description.into(),
        }
    }
    /// Null capability (no permissions).
    pub fn null() -> Self {
        Self::new(0, "null")
    }
    /// Read-only capability.
    pub fn read_only() -> Self {
        Self::new(1, "read-only")
    }
    /// Read-write capability.
    pub fn read_write() -> Self {
        Self::new(2, "read-write")
    }
    /// Full (root) capability.
    pub fn full() -> Self {
        Self::new(u32::MAX, "full")
    }
    /// Check if this capability is sufficient for the required level.
    pub fn is_sufficient_for(&self, required: u32) -> bool {
        self.level >= required
    }
    /// Attenuate (reduce) the capability to a lower level.
    pub fn attenuate(&self, max_level: u32) -> Capability {
        Capability::new(
            self.level.min(max_level),
            format!("attenuated({})", max_level),
        )
    }
}
/// A simple stream delimiter splitter.
///
/// Splits a byte stream on a given delimiter character and yields records.
#[allow(dead_code)]
pub struct DelimiterSplitter {
    buffer: Vec<u8>,
    delimiter: u8,
}
impl DelimiterSplitter {
    /// Create a new splitter with the given delimiter.
    #[allow(dead_code)]
    pub fn new(delimiter: u8) -> Self {
        Self {
            buffer: Vec::new(),
            delimiter,
        }
    }
    /// Feed bytes into the splitter.
    #[allow(dead_code)]
    pub fn feed(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
    }
    /// Extract all complete records from the buffer.
    ///
    /// A record ends at the delimiter. Incomplete records remain buffered.
    #[allow(dead_code)]
    pub fn drain(&mut self) -> Vec<Vec<u8>> {
        let mut records = Vec::new();
        while let Some(pos) = self.buffer.iter().position(|&b| b == self.delimiter) {
            let record = self.buffer.drain(..pos).collect();
            self.buffer.drain(..1);
            records.push(record);
        }
        records
    }
    /// How many bytes are currently buffered (waiting for delimiter).
    #[allow(dead_code)]
    pub fn buffered_len(&self) -> usize {
        self.buffer.len()
    }
}
/// Simulated file metadata.
#[derive(Debug, Clone)]
pub struct FileMetadata {
    /// File path.
    pub path: String,
    /// Size in bytes.
    pub size: u64,
    /// Whether this is a directory.
    pub is_dir: bool,
    /// Whether this is a regular file.
    pub is_file: bool,
    /// Whether the file is read-only.
    pub read_only: bool,
}
impl FileMetadata {
    /// Create metadata for a regular file.
    pub fn regular_file(path: impl Into<String>, size: u64) -> Self {
        Self {
            path: path.into(),
            size,
            is_dir: false,
            is_file: true,
            read_only: false,
        }
    }
    /// Create metadata for a directory.
    pub fn directory(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            size: 0,
            is_dir: true,
            is_file: false,
            read_only: false,
        }
    }
    /// Mark the file as read-only.
    pub fn with_read_only(mut self) -> Self {
        self.read_only = true;
        self
    }
}
/// A simple in-memory key-value store that simulates persistent IO.
///
/// This can be used in tests or elaboration to mock file system operations.
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct MockFs {
    files: std::collections::HashMap<String, Vec<u8>>,
}
impl MockFs {
    /// Create an empty mock file system.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Write bytes to a file.
    #[allow(dead_code)]
    pub fn write(&mut self, path: &str, data: Vec<u8>) {
        self.files.insert(path.to_string(), data);
    }
    /// Write a string to a file.
    #[allow(dead_code)]
    pub fn write_str(&mut self, path: &str, content: &str) {
        self.write(path, content.as_bytes().to_vec());
    }
    /// Read the contents of a file.
    #[allow(dead_code)]
    pub fn read(&self, path: &str) -> Result<Vec<u8>, IoError> {
        self.files
            .get(path)
            .cloned()
            .ok_or_else(|| IoError::not_found(path))
    }
    /// Read the contents of a file as a string.
    #[allow(dead_code)]
    pub fn read_str(&self, path: &str) -> Result<String, IoError> {
        let bytes = self.read(path)?;
        String::from_utf8(bytes).map_err(|_| IoError::invalid_data("invalid UTF-8"))
    }
    /// Check if a file exists.
    #[allow(dead_code)]
    pub fn exists(&self, path: &str) -> bool {
        self.files.contains_key(path)
    }
    /// Delete a file. Returns true if it existed.
    #[allow(dead_code)]
    pub fn remove(&mut self, path: &str) -> bool {
        self.files.remove(path).is_some()
    }
    /// List all file paths in the mock file system.
    #[allow(dead_code)]
    pub fn list_paths(&self) -> Vec<&str> {
        self.files.keys().map(String::as_str).collect()
    }
    /// File size in bytes, or an error if not found.
    #[allow(dead_code)]
    pub fn file_size(&self, path: &str) -> Result<u64, IoError> {
        self.files
            .get(path)
            .map(|v| v.len() as u64)
            .ok_or_else(|| IoError::not_found(path))
    }
}
/// A simple Hoare-logic verifier for IO programs.
///
/// Records pre/postcondition pairs for IO operations and checks consistency.
#[allow(dead_code)]
pub struct HoareVerifier {
    /// Each entry: (operation name, precondition description, postcondition description).
    triples: Vec<(String, String, String)>,
}
#[allow(dead_code)]
impl HoareVerifier {
    /// Create an empty verifier.
    pub fn new() -> Self {
        Self {
            triples: Vec::new(),
        }
    }
    /// Add a Hoare triple.
    pub fn add_triple(
        &mut self,
        op: impl Into<String>,
        pre: impl Into<String>,
        post: impl Into<String>,
    ) {
        self.triples.push((op.into(), pre.into(), post.into()));
    }
    /// Count registered triples.
    pub fn triple_count(&self) -> usize {
        self.triples.len()
    }
    /// Look up the postcondition for a given operation.
    pub fn postcondition_of(&self, op: &str) -> Option<&str> {
        self.triples
            .iter()
            .find(|(o, _, _)| o == op)
            .map(|(_, _, post)| post.as_str())
    }
    /// Check if an operation has been registered.
    pub fn has_triple(&self, op: &str) -> bool {
        self.triples.iter().any(|(o, _, _)| o == op)
    }
    /// Get all registered operation names.
    pub fn operations(&self) -> Vec<&str> {
        self.triples.iter().map(|(o, _, _)| o.as_str()).collect()
    }
}
/// Kinds of I/O errors that can occur in the IO monad.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IoErrorKind {
    /// File not found.
    NotFound,
    /// Permission denied.
    PermissionDenied,
    /// Connection refused.
    ConnectionRefused,
    /// Connection timed out.
    TimedOut,
    /// Unexpected end of file.
    UnexpectedEof,
    /// Write failed (disk full, etc.).
    WriteZero,
    /// Invalid data or format error.
    InvalidData,
    /// An unrecognised I/O error.
    Other,
}
/// A typed session channel enforcing a protocol.
#[allow(dead_code)]
pub struct SessionChannel {
    protocol: Vec<SessionAction>,
    cursor: usize,
    messages: Vec<String>,
}
#[allow(dead_code)]
impl SessionChannel {
    /// Create a channel with the given protocol.
    pub fn new(protocol: Vec<SessionAction>) -> Self {
        Self {
            protocol,
            cursor: 0,
            messages: Vec::new(),
        }
    }
    /// Attempt to perform a send action.
    ///
    /// Returns `Ok(())` if the protocol expects a send at this point.
    pub fn send(&mut self, msg: impl Into<String>) -> Result<(), String> {
        if self.cursor >= self.protocol.len() {
            return Err("protocol exhausted".to_string());
        }
        if self.protocol[self.cursor] != SessionAction::Send {
            return Err(format!(
                "protocol violation: expected {:?}, got Send",
                self.protocol[self.cursor]
            ));
        }
        self.messages.push(msg.into());
        self.cursor += 1;
        Ok(())
    }
    /// Attempt to perform a receive action.
    ///
    /// Returns the next message if the protocol expects a receive.
    pub fn recv(&mut self) -> Result<Option<String>, String> {
        if self.cursor >= self.protocol.len() {
            return Err("protocol exhausted".to_string());
        }
        if self.protocol[self.cursor] != SessionAction::Recv {
            return Err(format!(
                "protocol violation: expected {:?}, got Recv",
                self.protocol[self.cursor]
            ));
        }
        self.cursor += 1;
        Ok(self.messages.pop())
    }
    /// Check if the protocol has been fully followed.
    pub fn is_complete(&self) -> bool {
        self.cursor >= self.protocol.len()
    }
    /// Number of steps remaining in the protocol.
    pub fn remaining_steps(&self) -> usize {
        self.protocol.len().saturating_sub(self.cursor)
    }
}
/// An asynchronous task queue.
///
/// Simulates a queue of pending async IO tasks identified by integer handles.
#[allow(dead_code)]
pub struct AsyncTaskQueue {
    pending: std::collections::VecDeque<(usize, String)>,
    completed: std::collections::HashMap<usize, String>,
    next_id: usize,
}
#[allow(dead_code)]
impl AsyncTaskQueue {
    /// Create an empty task queue.
    pub fn new() -> Self {
        Self {
            pending: std::collections::VecDeque::new(),
            completed: std::collections::HashMap::new(),
            next_id: 0,
        }
    }
    /// Enqueue a new task, returning its handle.
    pub fn enqueue(&mut self, description: impl Into<String>) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.pending.push_back((id, description.into()));
        id
    }
    /// Complete the next pending task with the given result.
    pub fn complete_next(&mut self, result: impl Into<String>) -> Option<usize> {
        if let Some((id, _)) = self.pending.pop_front() {
            self.completed.insert(id, result.into());
            Some(id)
        } else {
            None
        }
    }
    /// Get the result of a completed task.
    pub fn result_of(&self, id: usize) -> Option<&str> {
        self.completed.get(&id).map(String::as_str)
    }
    /// Number of pending tasks.
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }
    /// Number of completed tasks.
    pub fn completed_count(&self) -> usize {
        self.completed.len()
    }
    /// Check if a task has completed.
    pub fn is_complete(&self, id: usize) -> bool {
        self.completed.contains_key(&id)
    }
}
/// An IO action with its expected type.
#[derive(Debug, Clone)]
pub struct IoAction {
    /// Kind of action.
    pub kind: IoActionKind,
    /// Result type of the action (the `α` in `IO α`).
    pub result_type: Expr,
}
impl IoAction {
    /// Create a new IO action.
    pub fn new(kind: IoActionKind, result_type: Expr) -> Self {
        Self { kind, result_type }
    }
    /// Create a `println` action.
    pub fn println() -> Self {
        Self::new(
            IoActionKind::Println,
            Expr::Const(Name::str("Unit"), vec![]),
        )
    }
    /// Create a `readLine` action.
    pub fn read_line() -> Self {
        Self::new(
            IoActionKind::ReadStdin,
            Expr::Const(Name::str("String"), vec![]),
        )
    }
    /// Create an `exit` action.
    pub fn exit(code: i32) -> Self {
        Self::new(
            IoActionKind::Exit(code),
            Expr::Const(Name::str("Empty"), vec![]),
        )
    }
}
/// An I/O error value.
#[derive(Debug, Clone)]
pub struct IoError {
    /// Error kind.
    pub kind: IoErrorKind,
    /// Human-readable description.
    pub message: String,
}
impl IoError {
    /// Create a new IoError.
    pub fn new(kind: IoErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }
    /// Create a "not found" error.
    pub fn not_found(path: &str) -> Self {
        Self::new(IoErrorKind::NotFound, format!("file not found: {}", path))
    }
    /// Create a "permission denied" error.
    pub fn permission_denied(path: &str) -> Self {
        Self::new(
            IoErrorKind::PermissionDenied,
            format!("permission denied: {}", path),
        )
    }
    /// Create an "unexpected EOF" error.
    pub fn unexpected_eof() -> Self {
        Self::new(IoErrorKind::UnexpectedEof, "unexpected end of file")
    }
    /// Create an "invalid data" error.
    pub fn invalid_data(msg: impl Into<String>) -> Self {
        Self::new(IoErrorKind::InvalidData, msg)
    }
}
/// A Software Transactional Memory (STM) log.
///
/// Records reads and writes during a transaction for conflict detection.
#[allow(dead_code)]
pub struct StmLog {
    /// Cells read during this transaction: (cell_id, value_at_read_time).
    reads: Vec<(usize, i64)>,
    /// Cells written during this transaction: (cell_id, new_value).
    writes: Vec<(usize, i64)>,
    /// Whether the transaction has been aborted.
    aborted: bool,
}
#[allow(dead_code)]
impl StmLog {
    /// Create an empty STM log.
    pub fn new() -> Self {
        Self {
            reads: Vec::new(),
            writes: Vec::new(),
            aborted: false,
        }
    }
    /// Record a read.
    pub fn record_read(&mut self, cell_id: usize, value: i64) {
        self.reads.push((cell_id, value));
    }
    /// Record a write.
    pub fn record_write(&mut self, cell_id: usize, value: i64) {
        self.writes.push((cell_id, value));
    }
    /// Abort the transaction.
    pub fn abort(&mut self) {
        self.aborted = true;
    }
    /// Check if aborted.
    pub fn is_aborted(&self) -> bool {
        self.aborted
    }
    /// Check for write-write conflicts with another log.
    pub fn conflicts_with(&self, other: &StmLog) -> bool {
        for (wid, _) in &self.writes {
            for (owid, _) in &other.writes {
                if wid == owid {
                    return true;
                }
            }
        }
        false
    }
    /// Number of reads recorded.
    pub fn read_count(&self) -> usize {
        self.reads.len()
    }
    /// Number of writes recorded.
    pub fn write_count(&self) -> usize {
        self.writes.len()
    }
}
/// A descriptor for an IO action being elaborated.
///
/// Used by the elaborator to track what IO operations are being performed.
#[derive(Debug, Clone, PartialEq)]
pub enum IoActionKind {
    /// Read from standard input.
    ReadStdin,
    /// Write to standard output.
    WriteStdout,
    /// Write to standard error.
    WriteStderr,
    /// Open a file for reading.
    OpenRead(String),
    /// Open a file for writing.
    OpenWrite(String),
    /// Close a file.
    Close,
    /// Print a line.
    Println,
    /// Flush a writer.
    Flush,
    /// Sleep for a duration (milliseconds).
    Sleep(u64),
    /// Exit with a code.
    Exit(i32),
}
/// A simple in-memory buffered reader over a byte slice.
///
/// Simulates buffered line reading for I/O processing within the elaborator.
pub struct BufferedReader {
    /// Internal data buffer.
    data: Vec<u8>,
    /// Current read position.
    pos: usize,
    /// Buffer capacity.
    capacity: usize,
}
impl BufferedReader {
    /// Create a new buffered reader with default capacity (8 KiB).
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            capacity: data.len(),
            data,
            pos: 0,
        }
    }
    /// Create a buffered reader from a string.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Self {
        Self::new(s.as_bytes().to_vec())
    }
    /// Read a single byte, or `None` if at end.
    pub fn read_byte(&mut self) -> Option<u8> {
        if self.pos < self.data.len() {
            let b = self.data[self.pos];
            self.pos += 1;
            Some(b)
        } else {
            None
        }
    }
    /// Read the next line (up to and including `\n`).
    ///
    /// Returns `None` when the buffer is exhausted.
    pub fn read_line(&mut self) -> Option<String> {
        if self.pos >= self.data.len() {
            return None;
        }
        let start = self.pos;
        while self.pos < self.data.len() && self.data[self.pos] != b'\n' {
            self.pos += 1;
        }
        if self.pos < self.data.len() {
            self.pos += 1;
        }
        String::from_utf8(self.data[start..self.pos].to_vec()).ok()
    }
    /// Read all remaining content as a string.
    pub fn read_to_string(&mut self) -> Result<String, IoError> {
        let remaining = &self.data[self.pos..];
        self.pos = self.data.len();
        String::from_utf8(remaining.to_vec())
            .map_err(|_| IoError::invalid_data("invalid UTF-8 sequence"))
    }
    /// Read exactly `n` bytes.
    ///
    /// Returns an error if fewer than `n` bytes remain.
    pub fn read_exact(&mut self, n: usize) -> Result<Vec<u8>, IoError> {
        if self.pos + n > self.data.len() {
            return Err(IoError::unexpected_eof());
        }
        let slice = self.data[self.pos..self.pos + n].to_vec();
        self.pos += n;
        Ok(slice)
    }
    /// Peek at the next byte without consuming it.
    pub fn peek(&self) -> Option<u8> {
        self.data.get(self.pos).copied()
    }
    /// Check if the reader is at end of data.
    pub fn is_eof(&self) -> bool {
        self.pos >= self.data.len()
    }
    /// Return the number of bytes remaining.
    pub fn remaining(&self) -> usize {
        self.data.len().saturating_sub(self.pos)
    }
    /// Reset the reader to the beginning.
    pub fn reset(&mut self) {
        self.pos = 0;
    }
    /// Skip `n` bytes.
    pub fn skip(&mut self, n: usize) {
        self.pos = (self.pos + n).min(self.data.len());
    }
    /// Return the buffer capacity.
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    /// Collect all lines into a vector.
    pub fn lines(&mut self) -> Vec<String> {
        let mut result = Vec::new();
        while let Some(line) = self.read_line() {
            result.push(line);
        }
        result
    }
}
/// IO environment variable registry (mock).
///
/// Simulates `std::env::var` for testing IO elaboration.
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct EnvRegistry {
    vars: std::collections::HashMap<String, String>,
}
impl EnvRegistry {
    /// Create an empty registry.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set an environment variable.
    #[allow(dead_code)]
    pub fn set(&mut self, key: &str, val: &str) {
        self.vars.insert(key.to_string(), val.to_string());
    }
    /// Get an environment variable.
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&str> {
        self.vars.get(key).map(String::as_str)
    }
    /// Remove an environment variable.
    #[allow(dead_code)]
    pub fn remove(&mut self, key: &str) {
        self.vars.remove(key);
    }
    /// List all variable names.
    #[allow(dead_code)]
    pub fn keys(&self) -> Vec<&str> {
        self.vars.keys().map(String::as_str).collect()
    }
}
/// A session channel that enforces a simple send/receive protocol.
///
/// The protocol is represented as a stack of expected actions.
/// Each action is either `Send` or `Recv`.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum SessionAction {
    /// Expect a send action.
    Send,
    /// Expect a receive action.
    Recv,
}
/// A simple in-memory buffered writer.
///
/// Accumulates output and allows flushing to a target.
pub struct BufferedWriter {
    /// Accumulated output bytes.
    buffer: Vec<u8>,
    /// Flush threshold in bytes (auto-flush when exceeded).
    flush_threshold: usize,
    /// Total bytes written (including past flushes).
    total_written: usize,
}
impl BufferedWriter {
    /// Create a new buffered writer with a given flush threshold.
    pub fn new(flush_threshold: usize) -> Self {
        Self {
            buffer: Vec::new(),
            flush_threshold,
            total_written: 0,
        }
    }
    /// Create a buffered writer with default threshold (4 KiB).
    pub fn default_threshold() -> Self {
        Self::new(4096)
    }
    /// Write bytes to the buffer.
    ///
    /// Auto-flushes (clears the buffer) if the threshold is exceeded.
    pub fn write_bytes(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
        self.total_written += data.len();
        if self.buffer.len() >= self.flush_threshold {
            self.flush();
        }
    }
    /// Write a string to the buffer.
    pub fn write_str(&mut self, s: &str) {
        self.write_bytes(s.as_bytes());
    }
    /// Write a string followed by a newline.
    pub fn writeln(&mut self, s: &str) {
        self.write_str(s);
        self.write_bytes(b"\n");
    }
    /// Flush the buffer (clear it, simulating a write to an underlying sink).
    pub fn flush(&mut self) {
        self.buffer.clear();
    }
    /// Get the current buffer contents as a string.
    pub fn as_str(&self) -> Result<&str, IoError> {
        std::str::from_utf8(&self.buffer)
            .map_err(|_| IoError::invalid_data("buffer contains invalid UTF-8"))
    }
    /// Get the number of buffered (unflushed) bytes.
    pub fn buffered_len(&self) -> usize {
        self.buffer.len()
    }
    /// Get the total number of bytes written (including flushed).
    pub fn total_written(&self) -> usize {
        self.total_written
    }
    /// Check if the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}
/// A chain of IO actions represented as a list of action descriptors.
///
/// Used during elaboration to validate that sequenced IO operations have
/// compatible types.
#[derive(Debug, Default)]
pub struct IoActionPipeline {
    /// Ordered list of IO actions in this pipeline.
    actions: Vec<IoAction>,
}
impl IoActionPipeline {
    /// Create an empty pipeline.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add an action to the pipeline.
    pub fn push(&mut self, action: IoAction) {
        self.actions.push(action);
    }
    /// Get the number of actions.
    pub fn len(&self) -> usize {
        self.actions.len()
    }
    /// Check if the pipeline is empty.
    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }
    /// Get all actions.
    pub fn actions(&self) -> &[IoAction] {
        &self.actions
    }
    /// Check if any action is an exit.
    pub fn has_exit(&self) -> bool {
        self.actions
            .iter()
            .any(|a| matches!(a.kind, IoActionKind::Exit(_)))
    }
    /// Get the last action's result type (the overall result type of the pipeline).
    pub fn result_type(&self) -> Option<&Expr> {
        self.actions.last().map(|a| &a.result_type)
    }
    /// Remove all actions.
    pub fn clear(&mut self) {
        self.actions.clear();
    }
}
