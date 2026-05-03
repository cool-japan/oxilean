//! File watcher types: FileEvent, FileEventKind, WatcherConfig, FileWatcher

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

// ============================================================
// FileEventKind
// ============================================================

/// The kind of file system event observed during polling.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileEventKind {
    /// An existing file's contents or metadata changed.
    Modified,
    /// A new file appeared that was not present in the previous snapshot.
    Created,
    /// A file that was present in the previous snapshot is gone.
    Deleted,
}

// ============================================================
// FileEvent
// ============================================================

/// A file system event returned by [`FileWatcher::poll`].
#[derive(Debug, Clone)]
pub struct FileEvent {
    /// Absolute path of the file that changed.
    pub path: PathBuf,
    /// The kind of change observed.
    pub kind: FileEventKind,
}

impl FileEvent {
    /// Construct a new `FileEvent`.
    pub fn new(path: PathBuf, kind: FileEventKind) -> Self {
        Self { path, kind }
    }
}

// ============================================================
// WatcherConfig
// ============================================================

/// Configuration for the polling-based [`FileWatcher`].
#[derive(Debug, Clone)]
pub struct WatcherConfig {
    /// How frequently (in milliseconds) to scan watched paths.
    /// Lower values yield lower latency but higher CPU usage.
    pub poll_interval_ms: u64,
    /// Whether to descend into sub-directories recursively when a directory
    /// path is added via [`FileWatcher::watch`].
    pub recursive: bool,
    /// If non-empty, only files whose extension (without the leading `.`) is
    /// in this list are tracked.  An empty list means "track every file".
    pub extensions: Vec<String>,
}

impl Default for WatcherConfig {
    fn default() -> Self {
        Self {
            poll_interval_ms: 500,
            recursive: true,
            extensions: Vec::new(),
        }
    }
}

impl WatcherConfig {
    /// Create a config with a specific poll interval and recursive enabled.
    pub fn new(poll_interval_ms: u64, recursive: bool) -> Self {
        Self {
            poll_interval_ms,
            recursive,
            extensions: Vec::new(),
        }
    }

    /// Add an extension filter (e.g. `"rs"`, `"lean"`).
    pub fn with_extension(mut self, ext: impl Into<String>) -> Self {
        self.extensions.push(ext.into());
        self
    }

    /// Returns `true` if the given path's extension matches the filter list,
    /// or the filter list is empty (match-all).
    pub fn matches_extension(&self, path: &Path) -> bool {
        if self.extensions.is_empty() {
            return true;
        }
        match path.extension().and_then(|e| e.to_str()) {
            Some(ext) => self.extensions.iter().any(|e| e == ext),
            None => false,
        }
    }
}

// ============================================================
// FileWatcher internal snapshot
// ============================================================

/// Snapshot entry: modification time and file size for a single file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct FileSnapshot {
    /// Last-modified time recorded from `std::fs::metadata`.
    pub mtime: SystemTime,
    /// File size in bytes.
    pub size: u64,
}

impl FileSnapshot {
    pub(super) fn new(mtime: SystemTime, size: u64) -> Self {
        Self { mtime, size }
    }
}

// ============================================================
// FileWatcher
// ============================================================

/// A pure-Rust, polling-based file watcher for incremental builds.
///
/// It maintains an internal snapshot of the mtime + size of every tracked
/// file.  Calling [`poll`](FileWatcher::poll) rescans all watched paths and
/// compares the current state to the snapshot, emitting [`FileEvent`]s for
/// any differences.
///
/// # Example
/// ```rust,no_run
/// use std::path::Path;
/// use oxilean_build::file_watcher::{FileWatcher, WatcherConfig};
///
/// let config = WatcherConfig::default();
/// let mut watcher = FileWatcher::new(config);
/// watcher.watch(Path::new("src")).expect("watch failed");
/// let events = watcher.poll();
/// ```
pub struct FileWatcher {
    /// Watcher configuration.
    pub(super) config: WatcherConfig,
    /// Root paths that are being watched (as supplied by the caller).
    pub(super) watched_roots: Vec<PathBuf>,
    /// The snapshot taken on the previous call to [`poll`] (or at
    /// construction time).  Maps absolute path → snapshot.
    pub(super) snapshot: HashMap<PathBuf, FileSnapshot>,
}
