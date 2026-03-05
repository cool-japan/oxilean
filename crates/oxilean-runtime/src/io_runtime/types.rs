//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::object::RtObject;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, Read, Write};

use super::functions::IoResult;

/// A single I/O event in the log.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct IoEvent {
    pub kind: IoEventKind,
    pub path: Option<String>,
    pub bytes: usize,
    pub timestamp_ms: u64,
    pub success: bool,
}
/// Executes IO actions.
pub struct IoExecutor<'a> {
    /// The I/O runtime.
    runtime: &'a mut IoRuntime,
}
impl<'a> IoExecutor<'a> {
    /// Create a new IO executor.
    pub fn new(runtime: &'a mut IoRuntime) -> Self {
        IoExecutor { runtime }
    }
    /// Execute a println action.
    pub fn println(&mut self, s: &str) -> IoValue {
        match self.runtime.exec_println(s) {
            Ok(()) => IoValue::unit(),
            Err(e) => IoValue::error(e),
        }
    }
    /// Execute a getLine action.
    pub fn get_line(&mut self) -> IoValue {
        match self.runtime.exec_get_line() {
            Ok(line) => IoValue::pure_val(RtObject::string(line)),
            Err(e) => IoValue::error(e),
        }
    }
    /// Execute a file read action.
    pub fn read_file(&mut self, path: &str) -> IoValue {
        match self.runtime.exec_read_file(path) {
            Ok(contents) => IoValue::pure_val(RtObject::string(contents)),
            Err(e) => IoValue::error(e),
        }
    }
    /// Execute a file write action.
    pub fn write_file(&mut self, path: &str, contents: &str) -> IoValue {
        match self.runtime.exec_write_file(path, contents) {
            Ok(()) => IoValue::unit(),
            Err(e) => IoValue::error(e),
        }
    }
    /// Execute a new ref action.
    pub fn new_ref(&mut self, value: RtObject) -> IoValue {
        let id = self.runtime.new_ref(value);
        IoValue::pure_val(RtObject::nat(id))
    }
    /// Execute a read ref action.
    pub fn read_ref(&mut self, id: u64) -> IoValue {
        match self.runtime.read_ref(id) {
            Ok(value) => IoValue::pure_val(value),
            Err(e) => IoValue::error(e),
        }
    }
    /// Execute a write ref action.
    pub fn write_ref(&mut self, id: u64, value: RtObject) -> IoValue {
        match self.runtime.write_ref(id, value) {
            Ok(()) => IoValue::unit(),
            Err(e) => IoValue::error(e),
        }
    }
    /// Get the current time.
    pub fn get_time(&mut self) -> IoValue {
        match self.runtime.get_time_nanos() {
            Ok(nanos) => IoValue::pure_val(RtObject::nat(nanos)),
            Err(e) => IoValue::error(e),
        }
    }
    /// Get an environment variable.
    pub fn get_env(&self, key: &str) -> IoValue {
        match self.runtime.get_env_var(key) {
            Some(value) => IoValue::pure_val(RtObject::string(value)),
            None => IoValue::pure_val(RtObject::string(String::new())),
        }
    }
}
/// An I/O interaction for replay.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum MockIoOp {
    Read { expected: Vec<u8>, result: Vec<u8> },
    Write { expected: Vec<u8>, ok: bool },
    ReadLine { result: String },
    ReadError { kind: IoErrorKind },
}
/// A simple polling file watcher (in-memory simulation).
#[allow(dead_code)]
pub struct IoFileWatcher {
    records: HashMap<String, FileRecord>,
    poll_interval_ms: u64,
}
#[allow(dead_code)]
impl IoFileWatcher {
    /// Create a file watcher with the given poll interval.
    pub fn new(poll_interval_ms: u64) -> Self {
        Self {
            records: HashMap::new(),
            poll_interval_ms,
        }
    }
    /// Register a file to watch.
    pub fn watch(&mut self, path: &str, current_size: u64, now_ms: u64) {
        self.records.insert(
            path.to_string(),
            FileRecord {
                path: path.to_string(),
                size: current_size,
                last_seen_ms: now_ms,
                change_count: 0,
            },
        );
    }
    /// Poll for changes. Returns paths that have changed.
    pub fn poll(&mut self, sizes: &HashMap<String, u64>, now_ms: u64) -> Vec<String> {
        let mut changed = Vec::new();
        for (path, record) in self.records.iter_mut() {
            if now_ms < record.last_seen_ms + self.poll_interval_ms {
                continue;
            }
            if let Some(&new_size) = sizes.get(path.as_str()) {
                if new_size != record.size {
                    record.size = new_size;
                    record.change_count += 1;
                    changed.push(path.clone());
                }
            }
            record.last_seen_ms = now_ms;
        }
        changed
    }
    /// Get the record for a path.
    pub fn record(&self, path: &str) -> Option<&FileRecord> {
        self.records.get(path)
    }
    /// Number of watched files.
    pub fn watch_count(&self) -> usize {
        self.records.len()
    }
    /// Unwatch a file.
    pub fn unwatch(&mut self, path: &str) {
        self.records.remove(path);
    }
}
/// A pair of channels forming a bidirectional pipe.
#[allow(dead_code)]
pub struct PipePair {
    pub a_to_b: IoChannel,
    pub b_to_a: IoChannel,
}
#[allow(dead_code)]
impl PipePair {
    /// Create a fresh pipe pair.
    pub fn new() -> Self {
        Self {
            a_to_b: IoChannel::new(),
            b_to_a: IoChannel::new(),
        }
    }
    /// Send data from A to B.
    pub fn send_a_to_b(&mut self, data: &[u8]) -> bool {
        self.a_to_b.write(data)
    }
    /// Send data from B to A.
    pub fn send_b_to_a(&mut self, data: &[u8]) -> bool {
        self.b_to_a.write(data)
    }
    /// Receive data on the B side.
    pub fn recv_b(&mut self, n: usize) -> Vec<u8> {
        self.a_to_b.read(n)
    }
    /// Receive data on the A side.
    pub fn recv_a(&mut self, n: usize) -> Vec<u8> {
        self.b_to_a.read(n)
    }
    /// Close both channels.
    pub fn close(&mut self) {
        self.a_to_b.close();
        self.b_to_a.close();
    }
}
/// What to do when an I/O error occurs.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IoErrorPolicy {
    /// Propagate the error immediately.
    Propagate,
    /// Ignore the error and continue.
    Ignore,
    /// Retry the operation up to N times.
    Retry { max: u32 },
    /// Log the error and continue.
    LogAndContinue,
}
/// String formatting operations.
pub struct StringFormatter;
impl StringFormatter {
    /// Format a runtime object as a string.
    pub fn format_object(obj: &RtObject) -> String {
        format!("{}", obj)
    }
    /// Convert a natural number to a string.
    pub fn nat_to_string(n: u64) -> String {
        format!("{}", n)
    }
    /// Convert an integer to a string.
    pub fn int_to_string(n: i64) -> String {
        format!("{}", n)
    }
    /// Convert a float to a string.
    pub fn float_to_string(f: f64) -> String {
        format!("{}", f)
    }
    /// Convert a boolean to a string.
    pub fn bool_to_string(b: bool) -> String {
        if b {
            "true".to_string()
        } else {
            "false".to_string()
        }
    }
    /// Convert a character to a string.
    pub fn char_to_string(c: char) -> String {
        c.to_string()
    }
    /// Format a list of objects as a string.
    pub fn format_list(elements: &[RtObject], separator: &str) -> String {
        elements
            .iter()
            .map(|e| format!("{}", e))
            .collect::<Vec<_>>()
            .join(separator)
    }
    /// Pad a string on the left to a minimum width.
    pub fn pad_left(s: &str, width: usize, pad_char: char) -> String {
        if s.len() >= width {
            return s.to_string();
        }
        let padding = std::iter::repeat(pad_char)
            .take(width - s.len())
            .collect::<String>();
        format!("{}{}", padding, s)
    }
    /// Pad a string on the right to a minimum width.
    pub fn pad_right(s: &str, width: usize, pad_char: char) -> String {
        if s.len() >= width {
            return s.to_string();
        }
        let padding = std::iter::repeat(pad_char)
            .take(width - s.len())
            .collect::<String>();
        format!("{}{}", s, padding)
    }
    /// Convert a string to uppercase.
    pub fn to_upper(s: &str) -> String {
        s.to_uppercase()
    }
    /// Convert a string to lowercase.
    pub fn to_lower(s: &str) -> String {
        s.to_lowercase()
    }
    /// Trim whitespace from both ends.
    pub fn trim(s: &str) -> String {
        s.trim().to_string()
    }
    /// Split a string by a separator.
    pub fn split(s: &str, sep: &str) -> Vec<String> {
        s.split(sep).map(|p| p.to_string()).collect()
    }
    /// Join strings with a separator.
    pub fn join(parts: &[String], sep: &str) -> String {
        parts.join(sep)
    }
    /// Check if a string starts with a prefix.
    pub fn starts_with(s: &str, prefix: &str) -> bool {
        s.starts_with(prefix)
    }
    /// Check if a string ends with a suffix.
    pub fn ends_with(s: &str, suffix: &str) -> bool {
        s.ends_with(suffix)
    }
    /// Replace all occurrences of a pattern.
    pub fn replace(s: &str, from: &str, to: &str) -> String {
        s.replace(from, to)
    }
    /// Check if a string contains a substring.
    pub fn contains(s: &str, substr: &str) -> bool {
        s.contains(substr)
    }
}
/// A category of I/O event.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IoEventKind {
    Read,
    Write,
    Open,
    Close,
    Error,
    Flush,
    Seek,
    Truncate,
}
/// The main I/O runtime that executes IO actions.
pub struct IoRuntime {
    /// Mutable references (for IO.Ref).
    pub(super) refs: HashMap<u64, RtObject>,
    /// Next reference ID.
    next_ref_id: u64,
    /// Whether I/O is enabled (can be disabled for sandboxing).
    pub(super) io_enabled: bool,
    /// Output buffer (for testing/capturing output).
    pub(super) output_buffer: Option<Vec<String>>,
    /// Input queue (for testing/mocking input).
    pub(super) input_queue: Vec<String>,
    /// Statistics.
    stats: IoStats,
    /// Environment variables.
    env_vars: HashMap<String, String>,
    /// Command-line arguments.
    args: Vec<String>,
}
impl IoRuntime {
    /// Create a new I/O runtime.
    pub fn new() -> Self {
        IoRuntime {
            refs: HashMap::new(),
            next_ref_id: 0,
            io_enabled: true,
            output_buffer: None,
            input_queue: Vec::new(),
            stats: IoStats::default(),
            env_vars: HashMap::new(),
            args: Vec::new(),
        }
    }
    /// Create a sandboxed I/O runtime (no real I/O).
    pub fn sandboxed() -> Self {
        IoRuntime {
            refs: HashMap::new(),
            next_ref_id: 0,
            io_enabled: false,
            output_buffer: Some(Vec::new()),
            input_queue: Vec::new(),
            stats: IoStats::default(),
            env_vars: HashMap::new(),
            args: Vec::new(),
        }
    }
    /// Enable output capturing.
    pub fn enable_capture(&mut self) {
        self.output_buffer = Some(Vec::new());
    }
    /// Get captured output.
    pub fn captured_output(&self) -> Option<&[String]> {
        self.output_buffer.as_deref()
    }
    /// Push input for mocking.
    pub fn push_input(&mut self, input: String) {
        self.input_queue.push(input);
    }
    /// Set command-line arguments.
    pub fn set_args(&mut self, args: Vec<String>) {
        self.args = args;
    }
    /// Set an environment variable.
    pub fn set_env(&mut self, key: String, value: String) {
        self.env_vars.insert(key, value);
    }
    /// Execute a println operation.
    pub fn exec_println(&mut self, s: &str) -> IoResult<()> {
        self.stats.console_outputs += 1;
        self.stats.bytes_written += s.len() as u64 + 1;
        if let Some(ref mut buf) = self.output_buffer {
            buf.push(s.to_string());
            return Ok(());
        }
        if !self.io_enabled {
            return Err(IoError::new(
                IoErrorKind::Unsupported,
                "I/O disabled in sandbox mode",
            ));
        }
        ConsoleOps::println(s)
    }
    /// Execute a print operation.
    pub fn exec_print(&mut self, s: &str) -> IoResult<()> {
        self.stats.console_outputs += 1;
        self.stats.bytes_written += s.len() as u64;
        if let Some(ref mut buf) = self.output_buffer {
            buf.push(s.to_string());
            return Ok(());
        }
        if !self.io_enabled {
            return Err(IoError::new(
                IoErrorKind::Unsupported,
                "I/O disabled in sandbox mode",
            ));
        }
        ConsoleOps::print(s)
    }
    /// Execute a getLine operation.
    pub fn exec_get_line(&mut self) -> IoResult<String> {
        self.stats.console_inputs += 1;
        if !self.input_queue.is_empty() {
            let input = self.input_queue.remove(0);
            self.stats.bytes_read += input.len() as u64;
            return Ok(input);
        }
        if !self.io_enabled {
            return Err(IoError::new(
                IoErrorKind::Unsupported,
                "I/O disabled in sandbox mode",
            ));
        }
        let line = ConsoleOps::get_line()?;
        self.stats.bytes_read += line.len() as u64;
        Ok(line)
    }
    /// Execute a file read operation.
    pub fn exec_read_file(&mut self, path: &str) -> IoResult<String> {
        self.stats.file_reads += 1;
        if !self.io_enabled {
            return Err(IoError::new(
                IoErrorKind::Unsupported,
                "I/O disabled in sandbox mode",
            ));
        }
        let contents = FileOps::read_file(path)?;
        self.stats.bytes_read += contents.len() as u64;
        Ok(contents)
    }
    /// Execute a file write operation.
    pub fn exec_write_file(&mut self, path: &str, contents: &str) -> IoResult<()> {
        self.stats.file_writes += 1;
        self.stats.bytes_written += contents.len() as u64;
        if !self.io_enabled {
            return Err(IoError::new(
                IoErrorKind::Unsupported,
                "I/O disabled in sandbox mode",
            ));
        }
        FileOps::write_file(path, contents)
    }
    /// Create a new mutable reference.
    pub fn new_ref(&mut self, value: RtObject) -> u64 {
        let id = self.next_ref_id;
        self.next_ref_id += 1;
        self.refs.insert(id, value);
        self.stats.refs_created += 1;
        id
    }
    /// Read a mutable reference.
    pub fn read_ref(&mut self, id: u64) -> IoResult<RtObject> {
        self.stats.ref_reads += 1;
        self.refs.get(&id).cloned().ok_or_else(|| {
            IoError::new(IoErrorKind::InvalidData, format!("invalid ref id: {}", id))
        })
    }
    /// Write to a mutable reference.
    pub fn write_ref(&mut self, id: u64, value: RtObject) -> IoResult<()> {
        self.stats.ref_writes += 1;
        if let std::collections::hash_map::Entry::Occupied(mut e) = self.refs.entry(id) {
            e.insert(value);
            Ok(())
        } else {
            Err(IoError::new(
                IoErrorKind::InvalidData,
                format!("invalid ref id: {}", id),
            ))
        }
    }
    /// Modify a mutable reference with a function.
    pub fn modify_ref(&mut self, id: u64, f: impl FnOnce(RtObject) -> RtObject) -> IoResult<()> {
        let value = self.read_ref(id)?;
        let new_value = f(value);
        self.write_ref(id, new_value)
    }
    /// Get an environment variable.
    pub fn get_env_var(&self, key: &str) -> Option<String> {
        self.env_vars
            .get(key)
            .cloned()
            .or_else(|| std::env::var(key).ok())
    }
    /// Get the command-line arguments.
    pub fn get_args(&self) -> &[String] {
        &self.args
    }
    /// Get the current time as nanoseconds since epoch.
    pub fn get_time_nanos(&self) -> IoResult<u64> {
        if !self.io_enabled {
            return Ok(0);
        }
        Ok(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| IoError::internal(format!("time error: {}", e)))?
            .as_nanos() as u64)
    }
    /// Get the statistics.
    pub fn stats(&self) -> &IoStats {
        &self.stats
    }
    /// Check if I/O is enabled.
    pub fn is_enabled(&self) -> bool {
        self.io_enabled
    }
    /// Enable or disable I/O.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.io_enabled = enabled;
    }
    /// Reset the runtime state.
    pub fn reset(&mut self) {
        self.refs.clear();
        self.next_ref_id = 0;
        self.stats = IoStats::default();
        self.output_buffer = if self.output_buffer.is_some() {
            Some(Vec::new())
        } else {
            None
        };
        self.input_queue.clear();
    }
}
/// An in-memory virtual filesystem for use in tests and sandboxed execution.
#[allow(dead_code)]
pub struct VirtualFilesystem {
    files: HashMap<String, Vec<u8>>,
    dirs: std::collections::HashSet<String>,
    read_only: bool,
}
#[allow(dead_code)]
impl VirtualFilesystem {
    /// Create an empty virtual filesystem.
    pub fn new() -> Self {
        let mut dirs = std::collections::HashSet::new();
        dirs.insert("/".to_string());
        Self {
            files: HashMap::new(),
            dirs,
            read_only: false,
        }
    }
    /// Mark the filesystem as read-only.
    pub fn set_read_only(&mut self, ro: bool) {
        self.read_only = ro;
    }
    /// Create a directory.
    pub fn mkdir(&mut self, path: &str) -> bool {
        if self.read_only {
            return false;
        }
        self.dirs.insert(path.to_string());
        true
    }
    /// Write a file.
    pub fn write_file(&mut self, path: &str, contents: &[u8]) -> Result<(), IoError> {
        if self.read_only {
            return Err(IoError {
                kind: IoErrorKind::PermissionDenied,
                message: format!("filesystem is read-only"),
                path: Some(path.to_string()),
            });
        }
        self.files.insert(path.to_string(), contents.to_vec());
        Ok(())
    }
    /// Read a file.
    pub fn read_file(&self, path: &str) -> Result<Vec<u8>, IoError> {
        self.files.get(path).cloned().ok_or_else(|| IoError {
            kind: IoErrorKind::FileNotFound,
            message: format!("file not found: {}", path),
            path: Some(path.to_string()),
        })
    }
    /// Delete a file.
    pub fn delete_file(&mut self, path: &str) -> bool {
        if self.read_only {
            return false;
        }
        self.files.remove(path).is_some()
    }
    /// Check whether a file exists.
    pub fn file_exists(&self, path: &str) -> bool {
        self.files.contains_key(path)
    }
    /// Check whether a directory exists.
    pub fn dir_exists(&self, path: &str) -> bool {
        self.dirs.contains(path)
    }
    /// List files in a directory (shallow).
    pub fn list_dir(&self, dir: &str) -> Vec<String> {
        let prefix = if dir.ends_with('/') {
            dir.to_string()
        } else {
            format!("{}/", dir)
        };
        self.files
            .keys()
            .filter(|p| p.starts_with(&prefix) && !p[prefix.len()..].contains('/'))
            .cloned()
            .collect()
    }
    /// File size.
    pub fn file_size(&self, path: &str) -> Option<usize> {
        self.files.get(path).map(|v| v.len())
    }
    /// Append to a file (creating it if it doesn't exist).
    pub fn append_file(&mut self, path: &str, contents: &[u8]) -> Result<(), IoError> {
        if self.read_only {
            return Err(IoError {
                kind: IoErrorKind::PermissionDenied,
                message: format!("filesystem is read-only"),
                path: Some(path.to_string()),
            });
        }
        self.files
            .entry(path.to_string())
            .or_default()
            .extend_from_slice(contents);
        Ok(())
    }
    /// Copy a file.
    pub fn copy_file(&mut self, src: &str, dst: &str) -> Result<(), IoError> {
        let contents = self.read_file(src)?;
        self.write_file(dst, &contents)
    }
    /// Rename a file.
    pub fn rename_file(&mut self, src: &str, dst: &str) -> Result<(), IoError> {
        let contents = self.read_file(src)?;
        self.write_file(dst, &contents)?;
        self.delete_file(src);
        Ok(())
    }
    /// Total number of files.
    pub fn file_count(&self) -> usize {
        self.files.len()
    }
    /// Total bytes stored.
    pub fn total_bytes(&self) -> usize {
        self.files.values().map(|v| v.len()).sum()
    }
}
/// A mock I/O object that replays a scripted sequence of operations.
#[allow(dead_code)]
pub struct IoMock {
    script: std::collections::VecDeque<MockIoOp>,
    actual_calls: Vec<String>,
}
#[allow(dead_code)]
impl IoMock {
    /// Create a mock from a scripted sequence.
    pub fn new(script: Vec<MockIoOp>) -> Self {
        Self {
            script: script.into(),
            actual_calls: Vec::new(),
        }
    }
    /// Simulate a read, consuming the next scripted response.
    pub fn read(&mut self, _buf: &mut Vec<u8>) -> Option<Vec<u8>> {
        if let Some(op) = self.script.pop_front() {
            match op {
                MockIoOp::Read { result, .. } => {
                    self.actual_calls.push(format!("read:{}", result.len()));
                    Some(result)
                }
                MockIoOp::ReadError { .. } => {
                    self.actual_calls.push("read:error".to_string());
                    None
                }
                _ => None,
            }
        } else {
            None
        }
    }
    /// Simulate a write.
    pub fn write(&mut self, data: &[u8]) -> bool {
        if let Some(op) = self.script.pop_front() {
            match op {
                MockIoOp::Write { ok, .. } => {
                    self.actual_calls.push(format!("write:{}", data.len()));
                    ok
                }
                _ => false,
            }
        } else {
            false
        }
    }
    /// Get the actual call log.
    pub fn calls(&self) -> &[String] {
        &self.actual_calls
    }
    /// Whether all scripted operations were consumed.
    pub fn is_exhausted(&self) -> bool {
        self.script.is_empty()
    }
    /// Remaining scripted operations.
    pub fn remaining(&self) -> usize {
        self.script.len()
    }
}
/// Real-time bandwidth and latency metrics for I/O.
#[allow(dead_code)]
pub struct IoMetrics {
    read_bytes: Vec<(u64, u64)>,
    write_bytes: Vec<(u64, u64)>,
    window_ms: u64,
    max_samples: usize,
}
#[allow(dead_code)]
impl IoMetrics {
    /// Create metrics tracker with the given window.
    pub fn new(window_ms: u64, max_samples: usize) -> Self {
        Self {
            read_bytes: Vec::new(),
            write_bytes: Vec::new(),
            window_ms,
            max_samples,
        }
    }
    /// Record a read event.
    pub fn record_read(&mut self, bytes: u64, now_ms: u64) {
        self.read_bytes.push((now_ms, bytes));
        if self.read_bytes.len() > self.max_samples {
            self.read_bytes.remove(0);
        }
    }
    /// Record a write event.
    pub fn record_write(&mut self, bytes: u64, now_ms: u64) {
        self.write_bytes.push((now_ms, bytes));
        if self.write_bytes.len() > self.max_samples {
            self.write_bytes.remove(0);
        }
    }
    /// Read bandwidth in bytes/ms over the window.
    pub fn read_bw(&self, now_ms: u64) -> f64 {
        let cutoff = now_ms.saturating_sub(self.window_ms);
        let total: u64 = self
            .read_bytes
            .iter()
            .filter(|(t, _)| *t >= cutoff)
            .map(|(_, b)| b)
            .sum();
        if self.window_ms == 0 {
            0.0
        } else {
            total as f64 / self.window_ms as f64
        }
    }
    /// Write bandwidth in bytes/ms over the window.
    pub fn write_bw(&self, now_ms: u64) -> f64 {
        let cutoff = now_ms.saturating_sub(self.window_ms);
        let total: u64 = self
            .write_bytes
            .iter()
            .filter(|(t, _)| *t >= cutoff)
            .map(|(_, b)| b)
            .sum();
        if self.window_ms == 0 {
            0.0
        } else {
            total as f64 / self.window_ms as f64
        }
    }
    /// Total read bytes recorded.
    pub fn total_read(&self) -> u64 {
        self.read_bytes.iter().map(|(_, b)| b).sum()
    }
    /// Total write bytes recorded.
    pub fn total_write(&self) -> u64 {
        self.write_bytes.iter().map(|(_, b)| b).sum()
    }
}
/// An in-memory byte channel connecting a writer and reader.
#[allow(dead_code)]
pub struct IoChannel {
    buf: std::collections::VecDeque<u8>,
    closed: bool,
    bytes_written: u64,
    bytes_read: u64,
}
#[allow(dead_code)]
impl IoChannel {
    /// Create an empty channel.
    pub fn new() -> Self {
        Self {
            buf: std::collections::VecDeque::new(),
            closed: false,
            bytes_written: 0,
            bytes_read: 0,
        }
    }
    /// Write bytes into the channel.
    pub fn write(&mut self, data: &[u8]) -> bool {
        if self.closed {
            return false;
        }
        self.buf.extend(data.iter().copied());
        self.bytes_written += data.len() as u64;
        true
    }
    /// Read up to `n` bytes from the channel.
    pub fn read(&mut self, n: usize) -> Vec<u8> {
        let take = n.min(self.buf.len());
        let mut out = Vec::with_capacity(take);
        for _ in 0..take {
            if let Some(b) = self.buf.pop_front() {
                out.push(b);
            }
        }
        self.bytes_read += out.len() as u64;
        out
    }
    /// Read all available bytes.
    pub fn read_all(&mut self) -> Vec<u8> {
        let out: Vec<u8> = self.buf.drain(..).collect();
        self.bytes_read += out.len() as u64;
        out
    }
    /// Read a line (up to and including `\n`).
    pub fn read_line(&mut self) -> Option<String> {
        let pos = self.buf.iter().position(|&b| b == b'\n')?;
        let line_bytes: Vec<u8> = self.buf.drain(..=pos).collect();
        self.bytes_read += line_bytes.len() as u64;
        String::from_utf8(line_bytes).ok()
    }
    /// Close the channel (no more writes allowed).
    pub fn close(&mut self) {
        self.closed = true;
    }
    /// Whether the channel is closed.
    pub fn is_closed(&self) -> bool {
        self.closed
    }
    /// Bytes available to read.
    pub fn available(&self) -> usize {
        self.buf.len()
    }
    /// Total bytes written.
    pub fn bytes_written(&self) -> u64 {
        self.bytes_written
    }
    /// Total bytes read.
    pub fn bytes_read(&self) -> u64 {
        self.bytes_read
    }
}
/// Console I/O operations.
pub struct ConsoleOps;
impl ConsoleOps {
    /// Print a string to stdout (no newline).
    pub fn print(s: &str) -> IoResult<()> {
        print!("{}", s);
        io::stdout().flush().map_err(|e| IoError::from_io_error(&e))
    }
    /// Print a string to stdout with a newline.
    pub fn println(s: &str) -> IoResult<()> {
        println!("{}", s);
        Ok(())
    }
    /// Print a string to stderr.
    pub fn eprint(s: &str) -> IoResult<()> {
        eprint!("{}", s);
        io::stderr().flush().map_err(|e| IoError::from_io_error(&e))
    }
    /// Print a string to stderr with a newline.
    pub fn eprintln(s: &str) -> IoResult<()> {
        eprintln!("{}", s);
        Ok(())
    }
    /// Read a line from stdin.
    pub fn get_line() -> IoResult<String> {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .map_err(|e| IoError::from_io_error(&e))?;
        if line.ends_with('\n') {
            line.pop();
            if line.ends_with('\r') {
                line.pop();
            }
        }
        Ok(line)
    }
    /// Read all of stdin as a string.
    pub fn read_stdin() -> IoResult<String> {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .map_err(|e| IoError::from_io_error(&e))?;
        Ok(buffer)
    }
}
/// A policy object that controls I/O error handling.
#[allow(dead_code)]
pub struct IoPolicy {
    pub read_error: IoErrorPolicy,
    pub write_error: IoErrorPolicy,
    pub open_error: IoErrorPolicy,
}
#[allow(dead_code)]
impl IoPolicy {
    /// Strict policy: propagate all errors.
    pub fn strict() -> Self {
        Self {
            read_error: IoErrorPolicy::Propagate,
            write_error: IoErrorPolicy::Propagate,
            open_error: IoErrorPolicy::Propagate,
        }
    }
    /// Lenient policy: log and continue.
    pub fn lenient() -> Self {
        Self {
            read_error: IoErrorPolicy::LogAndContinue,
            write_error: IoErrorPolicy::LogAndContinue,
            open_error: IoErrorPolicy::LogAndContinue,
        }
    }
    /// Retry policy with 3 attempts.
    pub fn retry() -> Self {
        Self {
            read_error: IoErrorPolicy::Retry { max: 3 },
            write_error: IoErrorPolicy::Retry { max: 3 },
            open_error: IoErrorPolicy::Retry { max: 3 },
        }
    }
}
/// Kind of I/O error.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IoErrorKind {
    /// File not found.
    FileNotFound,
    /// Permission denied.
    PermissionDenied,
    /// File already exists.
    AlreadyExists,
    /// I/O operation failed.
    IoFailed,
    /// Invalid input/data.
    InvalidData,
    /// Timeout.
    TimedOut,
    /// User-thrown exception.
    UserError,
    /// Internal runtime error.
    InternalError,
    /// End of file.
    EndOfFile,
    /// Interrupted operation.
    Interrupted,
    /// Unsupported operation.
    Unsupported,
}
impl IoErrorKind {
    /// Convert from standard I/O error kind.
    fn from_io_error_kind(kind: io::ErrorKind) -> Self {
        match kind {
            io::ErrorKind::NotFound => IoErrorKind::FileNotFound,
            io::ErrorKind::PermissionDenied => IoErrorKind::PermissionDenied,
            io::ErrorKind::AlreadyExists => IoErrorKind::AlreadyExists,
            io::ErrorKind::InvalidData => IoErrorKind::InvalidData,
            io::ErrorKind::TimedOut => IoErrorKind::TimedOut,
            io::ErrorKind::Interrupted => IoErrorKind::Interrupted,
            io::ErrorKind::UnexpectedEof => IoErrorKind::EndOfFile,
            _ => IoErrorKind::IoFailed,
        }
    }
}
/// A record of a file's last observed modification time.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct FileRecord {
    pub path: String,
    pub size: u64,
    pub last_seen_ms: u64,
    pub change_count: u64,
}
/// A structured log of I/O events for tracing and debugging.
#[allow(dead_code)]
pub struct IoLog {
    events: Vec<IoEvent>,
    max_events: usize,
    overflowed: bool,
}
#[allow(dead_code)]
impl IoLog {
    /// Create a new I/O log with the given capacity.
    pub fn new(max_events: usize) -> Self {
        Self {
            events: Vec::new(),
            max_events: max_events.max(1),
            overflowed: false,
        }
    }
    /// Record an event.
    pub fn record(&mut self, event: IoEvent) {
        if self.events.len() >= self.max_events {
            self.overflowed = true;
            self.events.remove(0);
        }
        self.events.push(event);
    }
    /// Get all events.
    pub fn events(&self) -> &[IoEvent] {
        &self.events
    }
    /// Filter events by kind.
    pub fn events_of_kind(&self, kind: &IoEventKind) -> Vec<&IoEvent> {
        self.events.iter().filter(|e| &e.kind == kind).collect()
    }
    /// Total bytes read across all Read events.
    pub fn total_bytes_read(&self) -> usize {
        self.events
            .iter()
            .filter(|e| e.kind == IoEventKind::Read)
            .map(|e| e.bytes)
            .sum()
    }
    /// Total bytes written across all Write events.
    pub fn total_bytes_written(&self) -> usize {
        self.events
            .iter()
            .filter(|e| e.kind == IoEventKind::Write)
            .map(|e| e.bytes)
            .sum()
    }
    /// Number of errors.
    pub fn error_count(&self) -> usize {
        self.events
            .iter()
            .filter(|e| e.kind == IoEventKind::Error)
            .count()
    }
    /// Whether the log has overflowed.
    pub fn has_overflowed(&self) -> bool {
        self.overflowed
    }
    /// Clear the log.
    pub fn clear(&mut self) {
        self.events.clear();
        self.overflowed = false;
    }
    /// Total event count.
    pub fn len(&self) -> usize {
        self.events.len()
    }
    /// Whether the log is empty.
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}
/// File I/O operations.
pub struct FileOps;
impl FileOps {
    /// Read an entire file as a string.
    pub fn read_file(path: &str) -> IoResult<String> {
        std::fs::read_to_string(path).map_err(|e| {
            IoError::with_path(
                IoErrorKind::from_io_error_kind(e.kind()),
                e.to_string(),
                path,
            )
        })
    }
    /// Read a file as bytes.
    pub fn read_file_bytes(path: &str) -> IoResult<Vec<u8>> {
        std::fs::read(path).map_err(|e| {
            IoError::with_path(
                IoErrorKind::from_io_error_kind(e.kind()),
                e.to_string(),
                path,
            )
        })
    }
    /// Write a string to a file (overwrite).
    pub fn write_file(path: &str, contents: &str) -> IoResult<()> {
        std::fs::write(path, contents).map_err(|e| {
            IoError::with_path(
                IoErrorKind::from_io_error_kind(e.kind()),
                e.to_string(),
                path,
            )
        })
    }
    /// Write bytes to a file (overwrite).
    pub fn write_file_bytes(path: &str, contents: &[u8]) -> IoResult<()> {
        std::fs::write(path, contents).map_err(|e| {
            IoError::with_path(
                IoErrorKind::from_io_error_kind(e.kind()),
                e.to_string(),
                path,
            )
        })
    }
    /// Append a string to a file.
    pub fn append_file(path: &str, contents: &str) -> IoResult<()> {
        use std::fs::OpenOptions;
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .map_err(|e| {
                IoError::with_path(
                    IoErrorKind::from_io_error_kind(e.kind()),
                    e.to_string(),
                    path,
                )
            })?;
        file.write_all(contents.as_bytes()).map_err(|e| {
            IoError::with_path(
                IoErrorKind::from_io_error_kind(e.kind()),
                e.to_string(),
                path,
            )
        })
    }
    /// Check if a file exists.
    pub fn file_exists(path: &str) -> bool {
        std::path::Path::new(path).exists()
    }
    /// Delete a file.
    pub fn delete_file(path: &str) -> IoResult<()> {
        std::fs::remove_file(path).map_err(|e| {
            IoError::with_path(
                IoErrorKind::from_io_error_kind(e.kind()),
                e.to_string(),
                path,
            )
        })
    }
    /// Get the size of a file in bytes.
    pub fn file_size(path: &str) -> IoResult<u64> {
        std::fs::metadata(path).map(|m| m.len()).map_err(|e| {
            IoError::with_path(
                IoErrorKind::from_io_error_kind(e.kind()),
                e.to_string(),
                path,
            )
        })
    }
    /// Create a directory (and parents).
    pub fn create_dir(path: &str) -> IoResult<()> {
        std::fs::create_dir_all(path).map_err(|e| {
            IoError::with_path(
                IoErrorKind::from_io_error_kind(e.kind()),
                e.to_string(),
                path,
            )
        })
    }
    /// List directory entries.
    pub fn list_dir(path: &str) -> IoResult<Vec<String>> {
        let entries = std::fs::read_dir(path).map_err(|e| {
            IoError::with_path(
                IoErrorKind::from_io_error_kind(e.kind()),
                e.to_string(),
                path,
            )
        })?;
        let mut result = Vec::new();
        for entry in entries {
            match entry {
                Ok(e) => {
                    if let Some(name) = e.file_name().to_str() {
                        result.push(name.to_string());
                    }
                }
                Err(e) => {
                    return Err(IoError::with_path(
                        IoErrorKind::from_io_error_kind(e.kind()),
                        e.to_string(),
                        path,
                    ));
                }
            }
        }
        Ok(result)
    }
}
/// A buffered I/O sink that accumulates writes and flushes on demand.
#[allow(dead_code)]
pub struct IoBuffer {
    buf: Vec<u8>,
    flush_threshold: usize,
    flush_count: u64,
    write_count: u64,
    total_bytes: u64,
}
#[allow(dead_code)]
impl IoBuffer {
    /// Create a buffer with the given flush threshold (bytes).
    pub fn new(flush_threshold: usize) -> Self {
        Self {
            buf: Vec::new(),
            flush_threshold,
            flush_count: 0,
            write_count: 0,
            total_bytes: 0,
        }
    }
    /// Write bytes to the buffer. Auto-flushes when threshold is exceeded.
    pub fn write(&mut self, data: &[u8]) -> Vec<u8> {
        self.buf.extend_from_slice(data);
        self.write_count += 1;
        self.total_bytes += data.len() as u64;
        if self.buf.len() >= self.flush_threshold {
            self.flush()
        } else {
            Vec::new()
        }
    }
    /// Write a UTF-8 string.
    pub fn write_str(&mut self, s: &str) -> Vec<u8> {
        self.write(s.as_bytes())
    }
    /// Flush the buffer, returning its contents.
    pub fn flush(&mut self) -> Vec<u8> {
        let out = std::mem::take(&mut self.buf);
        if !out.is_empty() {
            self.flush_count += 1;
        }
        out
    }
    /// Current buffer occupancy in bytes.
    pub fn buffered_bytes(&self) -> usize {
        self.buf.len()
    }
    /// Whether the buffer contains data.
    pub fn has_data(&self) -> bool {
        !self.buf.is_empty()
    }
    /// Number of write calls.
    pub fn write_count(&self) -> u64 {
        self.write_count
    }
    /// Number of flush calls.
    pub fn flush_count(&self) -> u64 {
        self.flush_count
    }
    /// Total bytes written (including not-yet-flushed data).
    pub fn total_bytes(&self) -> u64 {
        self.total_bytes
    }
}
/// Rate-limits I/O operations by byte budget per time window.
#[allow(dead_code)]
pub struct IoThrottle {
    bytes_per_window: u64,
    window_ms: u64,
    used_in_window: u64,
    window_start_ms: u64,
    total_throttled_bytes: u64,
    throttle_events: u64,
}
#[allow(dead_code)]
impl IoThrottle {
    /// Create a throttle allowing `bytes_per_window` bytes per `window_ms`.
    pub fn new(bytes_per_window: u64, window_ms: u64) -> Self {
        Self {
            bytes_per_window,
            window_ms,
            used_in_window: 0,
            window_start_ms: 0,
            total_throttled_bytes: 0,
            throttle_events: 0,
        }
    }
    /// Try to consume `bytes` from the current window.
    /// Returns `true` if allowed, `false` if throttled.
    pub fn try_consume(&mut self, bytes: u64, now_ms: u64) -> bool {
        if now_ms >= self.window_start_ms + self.window_ms {
            self.window_start_ms = now_ms;
            self.used_in_window = 0;
        }
        if self.used_in_window + bytes <= self.bytes_per_window {
            self.used_in_window += bytes;
            true
        } else {
            self.total_throttled_bytes += bytes;
            self.throttle_events += 1;
            false
        }
    }
    /// Bytes remaining in the current window.
    pub fn remaining(&self) -> u64 {
        self.bytes_per_window.saturating_sub(self.used_in_window)
    }
    /// Total throttle events so far.
    pub fn throttle_events(&self) -> u64 {
        self.throttle_events
    }
    /// Total bytes that were throttled.
    pub fn total_throttled_bytes(&self) -> u64 {
        self.total_throttled_bytes
    }
}
/// Exception handling for the I/O runtime.
pub struct ErrorHandling;
impl ErrorHandling {
    /// Create an exception object from an error message.
    pub fn make_exception(message: &str) -> RtObject {
        RtObject::string(message.to_string())
    }
    /// Create an exception object from an IoError.
    pub fn from_io_error(err: &IoError) -> RtObject {
        RtObject::string(err.to_string())
    }
    /// Try to extract an error message from an exception object.
    pub fn get_message(exception: &RtObject) -> Option<String> {
        crate::object::StringOps::as_str(exception)
    }
    /// Check if an object represents an error.
    pub fn is_error(obj: &RtObject) -> bool {
        if let Some(idx) = obj.as_small_ctor() {
            return idx == 1;
        }
        false
    }
    /// Create an "ok" result.
    pub fn ok(value: RtObject) -> RtObject {
        RtObject::constructor(0, vec![value])
    }
    /// Create an "error" result.
    pub fn error(message: String) -> RtObject {
        RtObject::constructor(1, vec![RtObject::string(message)])
    }
}
/// The result of executing an I/O action.
#[derive(Clone, Debug)]
pub enum IoValue {
    /// Pure value (no side effects performed).
    Pure(RtObject),
    /// Error occurred.
    Error(IoError),
    /// Operation returned nothing meaningful (e.g., println).
    Unit,
}
impl IoValue {
    /// Create a pure value.
    pub fn pure_val(obj: RtObject) -> Self {
        IoValue::Pure(obj)
    }
    /// Create an error.
    pub fn error(err: IoError) -> Self {
        IoValue::Error(err)
    }
    /// Create a unit value.
    pub fn unit() -> Self {
        IoValue::Unit
    }
    /// Check if this is an error.
    pub fn is_error(&self) -> bool {
        matches!(self, IoValue::Error(_))
    }
    /// Convert to a runtime object.
    pub fn to_rt_object(&self) -> RtObject {
        match self {
            IoValue::Pure(obj) => obj.clone(),
            IoValue::Error(err) => err.to_rt_object(),
            IoValue::Unit => RtObject::unit(),
        }
    }
    /// Convert to a result.
    pub fn to_result(self) -> IoResult<RtObject> {
        match self {
            IoValue::Pure(obj) => Ok(obj),
            IoValue::Error(err) => Err(err),
            IoValue::Unit => Ok(RtObject::unit()),
        }
    }
}
/// String operations that work directly on `RtObject` values.
pub struct StringRtOps;
impl StringRtOps {
    /// Concatenate two string objects.
    pub fn concat(a: &RtObject, b: &RtObject) -> Option<RtObject> {
        crate::object::StringOps::concat(a, b)
    }
    /// Get the length of a string object.
    pub fn length(obj: &RtObject) -> Option<RtObject> {
        crate::object::StringOps::byte_len(obj).map(|n| RtObject::nat(n as u64))
    }
    /// Convert a natural number to a string object.
    pub fn nat_repr(obj: &RtObject) -> Option<RtObject> {
        crate::object::StringOps::nat_to_string(obj)
    }
    /// Get a character at an index.
    pub fn get_char(obj: &RtObject, index: &RtObject) -> Option<RtObject> {
        let idx = index.as_small_nat()? as usize;
        crate::object::StringOps::char_at(obj, idx)
    }
    /// Take a substring.
    pub fn substr(obj: &RtObject, start: &RtObject, len: &RtObject) -> Option<RtObject> {
        let s = start.as_small_nat()? as usize;
        let l = len.as_small_nat()? as usize;
        crate::object::StringOps::substring(obj, s, l)
    }
}
/// Statistics for I/O operations.
#[derive(Clone, Debug, Default)]
pub struct IoStats {
    /// Number of file reads.
    pub file_reads: u64,
    /// Number of file writes.
    pub file_writes: u64,
    /// Number of console outputs.
    pub console_outputs: u64,
    /// Number of console inputs.
    pub console_inputs: u64,
    /// Number of exceptions thrown.
    pub exceptions_thrown: u64,
    /// Number of exceptions caught.
    pub exceptions_caught: u64,
    /// Number of refs created.
    pub refs_created: u64,
    /// Number of ref reads.
    pub ref_reads: u64,
    /// Number of ref writes.
    pub ref_writes: u64,
    /// Total bytes read.
    pub bytes_read: u64,
    /// Total bytes written.
    pub bytes_written: u64,
}
/// Aggregate statistics for an I/O session.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct IoSessionStats {
    pub reads: u64,
    pub writes: u64,
    pub flushes: u64,
    pub errors: u64,
    pub bytes_read: u64,
    pub bytes_written: u64,
    pub open_count: u64,
    pub close_count: u64,
}
#[allow(dead_code)]
impl IoSessionStats {
    /// Record a read of `n` bytes.
    pub fn record_read(&mut self, n: u64) {
        self.reads += 1;
        self.bytes_read += n;
    }
    /// Record a write of `n` bytes.
    pub fn record_write(&mut self, n: u64) {
        self.writes += 1;
        self.bytes_written += n;
    }
    /// Record a flush.
    pub fn record_flush(&mut self) {
        self.flushes += 1;
    }
    /// Record an error.
    pub fn record_error(&mut self) {
        self.errors += 1;
    }
    /// Record a file open.
    pub fn record_open(&mut self) {
        self.open_count += 1;
    }
    /// Record a file close.
    pub fn record_close(&mut self) {
        self.close_count += 1;
    }
    /// Total I/O operations.
    pub fn total_ops(&self) -> u64 {
        self.reads + self.writes + self.flushes
    }
    /// Read/write ratio (reads / (reads + writes)).
    pub fn read_ratio(&self) -> f64 {
        let total = self.reads + self.writes;
        if total == 0 {
            0.0
        } else {
            self.reads as f64 / total as f64
        }
    }
}
/// Runtime I/O error.
#[derive(Clone, Debug)]
pub struct IoError {
    /// Error kind.
    pub kind: IoErrorKind,
    /// Error message.
    pub message: String,
    /// Optional file path associated with the error.
    pub path: Option<String>,
}
impl IoError {
    /// Create a new I/O error.
    pub fn new(kind: IoErrorKind, message: impl Into<String>) -> Self {
        IoError {
            kind,
            message: message.into(),
            path: None,
        }
    }
    /// Create a new I/O error with a file path.
    pub fn with_path(
        kind: IoErrorKind,
        message: impl Into<String>,
        path: impl Into<String>,
    ) -> Self {
        IoError {
            kind,
            message: message.into(),
            path: Some(path.into()),
        }
    }
    /// Create a file-not-found error.
    pub fn file_not_found(path: impl Into<String>) -> Self {
        let p = path.into();
        IoError {
            kind: IoErrorKind::FileNotFound,
            message: format!("file not found: {}", p),
            path: Some(p),
        }
    }
    /// Create a user error.
    pub fn user_error(message: impl Into<String>) -> Self {
        IoError::new(IoErrorKind::UserError, message)
    }
    /// Create an internal error.
    pub fn internal(message: impl Into<String>) -> Self {
        IoError::new(IoErrorKind::InternalError, message)
    }
    /// Convert from a standard I/O error.
    pub fn from_io_error(err: &io::Error) -> Self {
        let kind = match err.kind() {
            io::ErrorKind::NotFound => IoErrorKind::FileNotFound,
            io::ErrorKind::PermissionDenied => IoErrorKind::PermissionDenied,
            io::ErrorKind::AlreadyExists => IoErrorKind::AlreadyExists,
            io::ErrorKind::InvalidData => IoErrorKind::InvalidData,
            io::ErrorKind::TimedOut => IoErrorKind::TimedOut,
            io::ErrorKind::Interrupted => IoErrorKind::Interrupted,
            io::ErrorKind::UnexpectedEof => IoErrorKind::EndOfFile,
            _ => IoErrorKind::IoFailed,
        };
        IoError::new(kind, err.to_string())
    }
    /// Convert to a runtime object (for the error monad).
    pub fn to_rt_object(&self) -> RtObject {
        RtObject::string(self.message.clone())
    }
}
