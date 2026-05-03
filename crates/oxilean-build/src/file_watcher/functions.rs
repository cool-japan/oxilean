//! File watcher implementation functions

use super::types::{FileEvent, FileEventKind, FileSnapshot, FileWatcher, WatcherConfig};
use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};

// ============================================================
// FileWatcher constructor + public API
// ============================================================

impl FileWatcher {
    /// Create a new `FileWatcher` with the provided configuration.
    /// No paths are watched until [`watch`](FileWatcher::watch) is called.
    pub fn new(config: WatcherConfig) -> Self {
        Self {
            config,
            watched_roots: Vec::new(),
            snapshot: HashMap::new(),
        }
    }

    /// Begin tracking `path`.  If `path` is a directory and
    /// `config.recursive` is `true`, all files inside are tracked.
    ///
    /// Returns an error only if the metadata of `path` itself cannot be
    /// read.  Individual files that become unreadable later are silently
    /// treated as deleted.
    pub fn watch(&mut self, path: &Path) -> io::Result<()> {
        let canonical = canonicalize_or_absolute(path)?;
        // Avoid duplicate roots.
        if !self.watched_roots.contains(&canonical) {
            self.watched_roots.push(canonical.clone());
        }
        // Seed the snapshot with current state so we don't fire spurious
        // Created events on the very first poll.
        self.seed_snapshot(&canonical)?;
        Ok(())
    }

    /// Stop tracking `path` (and all files beneath it if it is a directory).
    /// No error is returned if `path` was not being watched.
    pub fn unwatch(&mut self, path: &Path) {
        let canonical = match canonicalize_or_absolute(path) {
            Ok(p) => p,
            Err(_) => path.to_path_buf(),
        };
        self.watched_roots.retain(|r| r != &canonical);
        // Remove all snapshot entries whose path starts with this root.
        self.snapshot.retain(|k, _| !k.starts_with(&canonical));
    }

    /// Scan all watched paths and return the set of [`FileEvent`]s observed
    /// since the last call (or since construction if never called before).
    ///
    /// After returning, the internal snapshot is updated to reflect the
    /// current on-disk state.
    pub fn poll(&mut self) -> Vec<FileEvent> {
        let roots: Vec<PathBuf> = self.watched_roots.clone();
        let mut current: HashMap<PathBuf, FileSnapshot> = HashMap::new();

        for root in &roots {
            collect_files(root, &self.config, &mut current);
        }

        let mut events: Vec<FileEvent> = Vec::new();

        // Detect Created and Modified.
        for (path, new_snap) in &current {
            match self.snapshot.get(path) {
                None => {
                    events.push(FileEvent::new(path.clone(), FileEventKind::Created));
                }
                Some(old_snap) => {
                    if old_snap != new_snap {
                        events.push(FileEvent::new(path.clone(), FileEventKind::Modified));
                    }
                }
            }
        }

        // Detect Deleted.
        for path in self.snapshot.keys() {
            if !current.contains_key(path) {
                events.push(FileEvent::new(path.clone(), FileEventKind::Deleted));
            }
        }

        self.snapshot = current;
        events
    }

    /// Return the poll interval (in milliseconds) from the configuration.
    pub fn poll_interval_ms(&self) -> u64 {
        self.config.poll_interval_ms
    }

    /// Return the number of files currently in the snapshot.
    pub fn snapshot_size(&self) -> usize {
        self.snapshot.len()
    }

    /// Return a slice of the currently watched root paths.
    pub fn watched_roots(&self) -> &[PathBuf] {
        &self.watched_roots
    }
}

// ============================================================
// Internal helpers
// ============================================================

/// Seed the snapshot with the current state of `root`.  This prevents the
/// first call to `poll` from generating spurious Created events.
pub(super) fn seed_snapshot_for(
    config: &WatcherConfig,
    snapshot: &mut HashMap<PathBuf, FileSnapshot>,
    root: &Path,
) {
    collect_files(root, config, snapshot);
}

impl FileWatcher {
    fn seed_snapshot(&mut self, root: &Path) -> io::Result<()> {
        seed_snapshot_for(&self.config, &mut self.snapshot, root);
        Ok(())
    }
}

/// Recursively (if configured) walk `root` and populate `out` with snapshots
/// for every file that matches the extension filter.
pub(super) fn collect_files(
    root: &Path,
    config: &WatcherConfig,
    out: &mut HashMap<PathBuf, FileSnapshot>,
) {
    let meta = match std::fs::metadata(root) {
        Ok(m) => m,
        Err(_) => return,
    };

    if meta.is_file() {
        if config.matches_extension(root) {
            if let Some(snap) = snapshot_of(root, &meta) {
                out.insert(root.to_path_buf(), snap);
            }
        }
        return;
    }

    if meta.is_dir() {
        let entries = match std::fs::read_dir(root) {
            Ok(e) => e,
            Err(_) => return,
        };
        for entry in entries.flatten() {
            let child = entry.path();
            let child_meta = match std::fs::metadata(&child) {
                Ok(m) => m,
                Err(_) => continue,
            };
            if child_meta.is_dir() && config.recursive {
                collect_files(&child, config, out);
            } else if child_meta.is_file() {
                if config.matches_extension(&child) {
                    if let Some(snap) = snapshot_of(&child, &child_meta) {
                        out.insert(child, snap);
                    }
                }
            }
        }
    }
}

/// Build a [`FileSnapshot`] from already-fetched metadata, returning `None`
/// if the mtime is unavailable (e.g. WASI targets).
pub(super) fn snapshot_of(_path: &Path, meta: &std::fs::Metadata) -> Option<FileSnapshot> {
    let mtime = meta.modified().ok()?;
    let size = meta.len();
    Some(FileSnapshot::new(mtime, size))
}

/// Return the canonical form of `path`, falling back to the absolute path
/// constructed relative to the current directory when canonicalization fails.
pub(super) fn canonicalize_or_absolute(path: &Path) -> io::Result<PathBuf> {
    if path.is_absolute() {
        return Ok(path.to_path_buf());
    }
    match std::fs::canonicalize(path) {
        Ok(p) => Ok(p),
        Err(_) => {
            // Path may not exist yet; construct an absolute path manually.
            let cwd = std::env::current_dir()?;
            Ok(cwd.join(path))
        }
    }
}
