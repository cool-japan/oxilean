//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant, SystemTime};

/// Watch config builder.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct WatchConfigBuilder {
    debounce_ms: Option<u64>,
    recursive: Option<bool>,
    extensions: Vec<String>,
    ignore_dirs: Vec<String>,
    action: Option<WatchAction>,
}
#[allow(dead_code)]
impl WatchConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn debounce_ms(mut self, ms: u64) -> Self {
        self.debounce_ms = Some(ms);
        self
    }
    pub fn recursive(mut self, r: bool) -> Self {
        self.recursive = Some(r);
        self
    }
    pub fn extension(mut self, ext: impl Into<String>) -> Self {
        self.extensions.push(ext.into());
        self
    }
    pub fn ignore_dir(mut self, dir: impl Into<String>) -> Self {
        self.ignore_dirs.push(dir.into());
        self
    }
    pub fn action(mut self, a: WatchAction) -> Self {
        self.action = Some(a);
        self
    }
    pub fn build(self) -> WatchConfig {
        let mut base = WatchConfig::default();
        if let Some(d) = self.debounce_ms {
            base.debounce_ms = d;
        }
        if let Some(r) = self.recursive {
            base.recursive = r;
        }
        if !self.extensions.is_empty() {
            base.extensions = self.extensions;
        }
        if !self.ignore_dirs.is_empty() {
            base.ignore_dirs = self.ignore_dirs;
        }
        if let Some(a) = self.action {
            base.action = a;
        }
        base
    }
}
/// Statistics about file watcher operations.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct WatcherStatistics {
    pub total_polls: u64,
    pub total_events: u64,
    pub created_events: u64,
    pub modified_events: u64,
    pub deleted_events: u64,
    pub files_watched: usize,
    pub directories_watched: usize,
    pub errors: u64,
}
impl WatcherStatistics {
    /// Create new zeroed stats.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a poll.
    #[allow(dead_code)]
    pub fn record_poll(&mut self, events: &[WatchEventKind]) {
        self.total_polls += 1;
        for e in events {
            self.total_events += 1;
            match e {
                WatchEventKind::Created => self.created_events += 1,
                WatchEventKind::Modified => self.modified_events += 1,
                WatchEventKind::Deleted => self.deleted_events += 1,
                WatchEventKind::Renamed => {
                    self.deleted_events += 1;
                    self.created_events += 1;
                    self.total_events += 1;
                }
            }
        }
    }
    /// Return the event rate per poll.
    #[allow(dead_code)]
    pub fn events_per_poll(&self) -> f64 {
        if self.total_polls == 0 {
            0.0
        } else {
            self.total_events as f64 / self.total_polls as f64
        }
    }
}
/// The kind of change observed for a file.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WatchEventKind {
    /// A new file appeared.
    Created,
    /// An existing file was modified.
    Modified,
    /// A file was deleted.
    Deleted,
    /// A file was renamed (approximated as delete + create).
    Renamed,
}
/// Hot reload state.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HotReloadState {
    Idle,
    Pending,
    Reloading,
    Done,
    Failed,
}
/// Action to take when a change is detected.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WatchAction {
    /// Re-type-check affected modules.
    Recheck,
    /// Full rebuild.
    Rebuild,
    /// Print a notification only.
    Notify,
    /// Custom action (handled by caller).
    Custom,
}
/// Tracks a history of file-system changes.
#[derive(Clone, Debug)]
pub struct ChangeTracker {
    /// Full history of changes.
    history: Vec<ChangeRecord>,
}
impl ChangeTracker {
    /// Create a new empty tracker.
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
        }
    }
    /// Record a change.
    pub fn record_change(&mut self, record: ChangeRecord) {
        self.history.push(record);
    }
    /// Return all recorded changes.
    pub fn all_changes(&self) -> &[ChangeRecord] {
        &self.history
    }
    /// Return the N most recent changes.
    pub fn recent_changes(&self, n: usize) -> &[ChangeRecord] {
        let start = self.history.len().saturating_sub(n);
        &self.history[start..]
    }
    /// Return changes since the given `Instant`.
    pub fn changes_since(&self, since: Instant) -> Vec<&ChangeRecord> {
        self.history
            .iter()
            .filter(|r| r.timestamp >= since)
            .collect()
    }
    /// Return the total number of recorded changes.
    pub fn len(&self) -> usize {
        self.history.len()
    }
    /// Return true if no changes have been recorded.
    pub fn is_empty(&self) -> bool {
        self.history.is_empty()
    }
    /// Clear all history.
    pub fn clear(&mut self) {
        self.history.clear();
    }
}
/// Polling-based file monitor.
#[derive(Clone, Debug)]
pub struct FileMonitor {
    /// Directories / files being watched.
    watched_paths: Vec<PathBuf>,
    /// How often to poll (milliseconds).
    poll_interval_ms: u64,
    /// Last known state of each file, keyed by canonical path string.
    last_seen: HashMap<String, FileState>,
}
impl FileMonitor {
    /// Create a new file monitor with the given poll interval.
    pub fn new(poll_interval_ms: u64) -> Self {
        Self {
            watched_paths: Vec::new(),
            poll_interval_ms,
            last_seen: HashMap::new(),
        }
    }
    /// Add a path (file or directory) to watch.
    pub fn add_watch(&mut self, path: impl Into<PathBuf>) {
        let p = path.into();
        if !self.watched_paths.contains(&p) {
            self.watched_paths.push(p);
        }
    }
    /// Remove a watched path.
    pub fn remove_watch(&mut self, path: &Path) {
        self.watched_paths.retain(|p| p != path);
    }
    /// Remove all watches.
    pub fn clear_watches(&mut self) {
        self.watched_paths.clear();
        self.last_seen.clear();
    }
    /// Return the number of watched paths.
    pub fn watch_count(&self) -> usize {
        self.watched_paths.len()
    }
    /// Return the poll interval.
    pub fn poll_interval(&self) -> Duration {
        Duration::from_millis(self.poll_interval_ms)
    }
    /// Poll all watched paths and return events for any changes since the
    /// last call.
    pub fn poll_changes(&mut self) -> Vec<WatchEvent> {
        let current = self.scan_all();
        let events = self.detect_changes(&current);
        self.last_seen = current
            .into_iter()
            .map(|fs| (fs.path.to_string_lossy().to_string(), fs))
            .collect();
        events
    }
    /// Recursively scan a single directory, returning file states.
    pub fn scan_directory(dir: &Path) -> Vec<FileState> {
        let mut results = Vec::new();
        Self::scan_directory_inner(dir, &mut results);
        results
    }
    fn scan_directory_inner(dir: &Path, results: &mut Vec<FileState>) {
        let entries = match std::fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => return,
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                Self::scan_directory_inner(&path, results);
            } else if let Ok(meta) = entry.metadata() {
                results.push(FileState::from_metadata(path, &meta));
            }
        }
    }
    /// Scan all watched paths and return combined file states.
    fn scan_all(&self) -> Vec<FileState> {
        let mut all = Vec::new();
        for p in &self.watched_paths {
            if p.is_dir() {
                all.extend(Self::scan_directory(p));
            } else if let Ok(meta) = std::fs::metadata(p) {
                all.push(FileState::from_metadata(p.clone(), &meta));
            }
        }
        all
    }
    /// Compare current scan results against `last_seen` and produce events.
    pub fn detect_changes(&self, current: &[FileState]) -> Vec<WatchEvent> {
        let mut events = Vec::new();
        let current_map: HashMap<String, &FileState> = current
            .iter()
            .map(|fs| (fs.path.to_string_lossy().to_string(), fs))
            .collect();
        for (key, state) in &current_map {
            match self.last_seen.get(key) {
                None => {
                    events.push(WatchEvent::new(state.path.clone(), WatchEventKind::Created));
                }
                Some(prev) => {
                    if state.modified != prev.modified || state.size != prev.size {
                        events.push(WatchEvent::new(
                            state.path.clone(),
                            WatchEventKind::Modified,
                        ));
                    }
                }
            }
        }
        for key in self.last_seen.keys() {
            if !current_map.contains_key(key) {
                events.push(WatchEvent::new(PathBuf::from(key), WatchEventKind::Deleted));
            }
        }
        events
    }
}
/// Watch display mode.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatchDisplayMode {
    Compact,
    Verbose,
    Silent,
}
/// Content hash cache.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct HashCache {
    pub(crate) cache: HashMap<String, u64>,
}
#[allow(dead_code)]
impl HashCache {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn update(&mut self, path: &std::path::Path) -> bool {
        let key = path.to_string_lossy().to_string();
        let new_hash = compute_file_hash(path).unwrap_or(0);
        let changed = self.cache.get(&key).copied().unwrap_or(u64::MAX) != new_hash;
        self.cache.insert(key, new_hash);
        changed
    }
    pub fn remove(&mut self, path: &std::path::Path) {
        self.cache.remove(&path.to_string_lossy().to_string());
    }
    pub fn get(&self, path: &std::path::Path) -> Option<u64> {
        self.cache.get(&path.to_string_lossy().to_string()).copied()
    }
    pub fn len(&self) -> usize {
        self.cache.len()
    }
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }
    pub fn clear(&mut self) {
        self.cache.clear();
    }
}
/// A logger for watcher events.
#[allow(dead_code)]
pub struct WatcherLog {
    entries: Vec<WatcherLogEntry>,
    max_entries: usize,
}
impl WatcherLog {
    /// Create a new log.
    #[allow(dead_code)]
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: vec![],
            max_entries,
        }
    }
    /// Add a log entry.
    #[allow(dead_code)]
    pub fn add(
        &mut self,
        level: WatcherLogLevel,
        message: impl Into<String>,
        path: Option<PathBuf>,
    ) {
        if self.entries.len() >= self.max_entries {
            self.entries.remove(0);
        }
        self.entries.push(WatcherLogEntry {
            timestamp_secs: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            level,
            message: message.into(),
            path,
        });
    }
    /// Return entries matching a level.
    #[allow(dead_code)]
    pub fn entries_at_level(&self, level: &WatcherLogLevel) -> Vec<&WatcherLogEntry> {
        self.entries.iter().filter(|e| &e.level == level).collect()
    }
    /// Return all entries.
    #[allow(dead_code)]
    pub fn all_entries(&self) -> &[WatcherLogEntry] {
        &self.entries
    }
}
/// Handle returned by `start_watch` to control the watch session.
#[derive(Debug)]
pub struct WatchHandle {
    /// Signal used to stop the session.
    stopped: bool,
}
impl WatchHandle {
    /// Create a new handle in the running state.
    pub fn new() -> Self {
        Self { stopped: false }
    }
    /// Signal the session to stop.
    pub fn stop(&mut self) {
        self.stopped = true;
    }
    /// Check whether the session has been stopped.
    pub fn is_stopped(&self) -> bool {
        self.stopped
    }
}
/// Filter that excludes specific directory names.
#[allow(dead_code)]
pub struct DirectoryExcludeFilter {
    pub excluded: Vec<String>,
}
impl DirectoryExcludeFilter {
    /// Create a filter for the given directory names.
    #[allow(dead_code)]
    pub fn new(excluded: Vec<&str>) -> Self {
        Self {
            excluded: excluded.into_iter().map(String::from).collect(),
        }
    }
}
/// Build request queue.
#[allow(dead_code)]
pub struct BuildRequestQueue {
    pending: Vec<BuildRequest>,
    max_pending: usize,
}
#[allow(dead_code)]
impl BuildRequestQueue {
    pub fn new(max_pending: usize) -> Self {
        Self {
            pending: Vec::new(),
            max_pending: max_pending.max(1),
        }
    }
    pub fn push(&mut self, req: BuildRequest) {
        if req.supersedes_pending {
            self.pending.clear();
        }
        if self.pending.len() < self.max_pending {
            self.pending.push(req);
        }
    }
    pub fn pop(&mut self) -> Option<BuildRequest> {
        if self.pending.is_empty() {
            None
        } else {
            Some(self.pending.remove(0))
        }
    }
    pub fn len(&self) -> usize {
        self.pending.len()
    }
    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }
}
/// Decides what action to take based on changed files.
#[allow(dead_code)]
pub struct RebuildTrigger {
    pub full_rebuild_patterns: Vec<String>,
    pub recheck_patterns: Vec<String>,
    pub default_action: WatchAction,
}
#[allow(dead_code)]
impl RebuildTrigger {
    pub fn default_oxilean() -> Self {
        Self {
            full_rebuild_patterns: vec!["Oxilean.toml".to_string()],
            recheck_patterns: vec!["*.lean".to_string(), "*.oxilean".to_string()],
            default_action: WatchAction::Notify,
        }
    }
    pub fn action_for(&self, event: &WatchEvent) -> WatchAction {
        for pat in &self.full_rebuild_patterns {
            if path_matches_pattern(&event.path, pat)
                || event
                    .path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("")
                    == pat.as_str()
            {
                return WatchAction::Rebuild;
            }
        }
        for pat in &self.recheck_patterns {
            if path_matches_pattern(&event.path, pat) {
                return WatchAction::Recheck;
            }
        }
        self.default_action
    }
    pub fn group_by_action<'a>(
        &self,
        events: &'a [WatchEvent],
    ) -> HashMap<WatchAction, Vec<&'a WatchEvent>> {
        let mut map: HashMap<WatchAction, Vec<&'a WatchEvent>> = HashMap::new();
        for event in events {
            map.entry(self.action_for(event)).or_default().push(event);
        }
        map
    }
}
/// A log entry from the watcher.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct WatcherLogEntry {
    pub timestamp_secs: u64,
    pub level: WatcherLogLevel,
    pub message: String,
    pub path: Option<PathBuf>,
}
/// Filter that only allows specific file extensions.
#[allow(dead_code)]
pub struct ExtensionFilter {
    pub extensions: Vec<String>,
}
impl ExtensionFilter {
    /// Create a filter for the given extensions.
    #[allow(dead_code)]
    pub fn new(extensions: Vec<&str>) -> Self {
        Self {
            extensions: extensions.into_iter().map(String::from).collect(),
        }
    }
}
/// Configuration for a file watcher.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct WatcherConfig {
    pub poll_interval_ms: u64,
    pub debounce_ms: u64,
    pub max_depth: u32,
    pub include_extensions: Vec<String>,
    pub exclude_patterns: Vec<String>,
    pub follow_symlinks: bool,
    pub batch_events: bool,
    pub batch_window_ms: u64,
}
impl WatcherConfig {
    /// Create a configuration for watching OxiLean files.
    #[allow(dead_code)]
    pub fn for_oxilean() -> Self {
        Self::default()
    }
    /// Check whether a file path matches the include extensions.
    #[allow(dead_code)]
    pub fn should_include(&self, path: &Path) -> bool {
        if self.include_extensions.is_empty() {
            return true;
        }
        if let Some(ext) = path.extension() {
            let ext_str = ext.to_string_lossy();
            return self
                .include_extensions
                .iter()
                .any(|e| e == ext_str.as_ref());
        }
        false
    }
    /// Check whether a path component should be excluded.
    #[allow(dead_code)]
    pub fn should_exclude(&self, path: &Path) -> bool {
        for component in path.components() {
            let name = component.as_os_str().to_string_lossy();
            if self.exclude_patterns.iter().any(|p| p == name.as_ref()) {
                return true;
            }
        }
        false
    }
}
/// A subscriber that counts events (for testing).
#[allow(dead_code)]
pub struct CountingSubscriber {
    pub count: std::sync::atomic::AtomicU64,
}
impl CountingSubscriber {
    /// Create a new counting subscriber.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            count: std::sync::atomic::AtomicU64::new(0),
        }
    }
    /// Return the current count.
    #[allow(dead_code)]
    pub fn get_count(&self) -> u64 {
        self.count.load(std::sync::atomic::Ordering::Relaxed)
    }
}
/// Manages multiple watch targets.
#[allow(dead_code)]
pub struct WatcherRegistry {
    targets: Vec<WatchTarget>,
}
impl WatcherRegistry {
    /// Create a new registry.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { targets: vec![] }
    }
    /// Add a watch target.
    #[allow(dead_code)]
    pub fn add(&mut self, target: WatchTarget) {
        self.targets.push(target);
    }
    /// Remove a target by path.
    #[allow(dead_code)]
    pub fn remove(&mut self, path: &Path) {
        self.targets.retain(|t| t.path != path);
    }
    /// Enable or disable a target.
    #[allow(dead_code)]
    pub fn set_enabled(&mut self, path: &Path, enabled: bool) {
        for target in self.targets.iter_mut() {
            if target.path == path {
                target.enabled = enabled;
            }
        }
    }
    /// Return all enabled targets.
    #[allow(dead_code)]
    pub fn enabled_targets(&self) -> Vec<&WatchTarget> {
        self.targets.iter().filter(|t| t.enabled).collect()
    }
    /// Return the target count.
    #[allow(dead_code)]
    pub fn count(&self) -> usize {
        self.targets.len()
    }
}
/// Configuration for a watch session.
#[derive(Clone, Debug)]
pub struct WatchConfig {
    /// Debounce window in milliseconds.
    pub debounce_ms: u64,
    /// Recurse into subdirectories.
    pub recursive: bool,
    /// File extensions to watch.
    pub extensions: Vec<String>,
    /// Directories to ignore.
    pub ignore_dirs: Vec<String>,
    /// Default action on change.
    pub action: WatchAction,
}
/// Statistics about file monitoring activity.
#[derive(Clone, Debug, Default)]
pub struct MonitorStats {
    /// Total poll cycles performed.
    pub poll_cycles: u64,
    /// Total events detected.
    pub events_detected: u64,
    /// Total events processed (after filtering).
    pub events_processed: u64,
    /// Total events debounced (suppressed).
    pub events_debounced: u64,
}
impl MonitorStats {
    /// Create fresh statistics.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record one poll cycle.
    pub fn record_poll(&mut self, detected: u64, processed: u64) {
        self.poll_cycles += 1;
        self.events_detected += detected;
        self.events_processed += processed;
        self.events_debounced += detected.saturating_sub(processed);
    }
    /// Average events per cycle.
    pub fn avg_events_per_cycle(&self) -> f64 {
        if self.poll_cycles == 0 {
            0.0
        } else {
            self.events_detected as f64 / self.poll_cycles as f64
        }
    }
    /// Debounce ratio (fraction of events debounced).
    pub fn debounce_ratio(&self) -> f64 {
        if self.events_detected == 0 {
            0.0
        } else {
            self.events_debounced as f64 / self.events_detected as f64
        }
    }
}
/// Watch error.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WatchError {
    pub message: String,
    pub recoverable: bool,
    pub retry_count: u32,
}
#[allow(dead_code)]
impl WatchError {
    pub fn recoverable(msg: impl Into<String>) -> Self {
        Self {
            message: msg.into(),
            recoverable: true,
            retry_count: 0,
        }
    }
    pub fn fatal(msg: impl Into<String>) -> Self {
        Self {
            message: msg.into(),
            recoverable: false,
            retry_count: 0,
        }
    }
    pub fn increment_retry(&mut self) {
        self.retry_count += 1;
    }
}
/// Batches rapid successive events for the same file.
#[allow(dead_code)]
pub struct WatchEventBatcher {
    pending: HashMap<PathBuf, (WatchEventKind, Instant)>,
    window_ms: u64,
}
impl WatchEventBatcher {
    /// Create a new batcher with the given window in milliseconds.
    #[allow(dead_code)]
    pub fn new(window_ms: u64) -> Self {
        Self {
            pending: HashMap::new(),
            window_ms,
        }
    }
    /// Add an event. Returns whether the event was de-duplicated.
    #[allow(dead_code)]
    pub fn add(&mut self, path: PathBuf, kind: WatchEventKind) -> bool {
        let now = Instant::now();
        if let Some((existing_kind, ts)) = self.pending.get_mut(&path) {
            if (ts.elapsed().as_millis() as u64) < self.window_ms {
                *existing_kind = kind;
                return true;
            }
        }
        self.pending.insert(path, (kind, now));
        false
    }
    /// Drain events that have passed the window.
    #[allow(dead_code)]
    pub fn drain_ready(&mut self) -> Vec<(PathBuf, WatchEventKind)> {
        let window_ms = self.window_ms;
        let mut ready = vec![];
        self.pending.retain(|path, (kind, ts)| {
            if ts.elapsed().as_millis() as u64 >= window_ms {
                ready.push((path.clone(), *kind));
                false
            } else {
                true
            }
        });
        ready
    }
}
/// Log level for watcher messages.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WatcherLogLevel {
    Error,
    Warning,
    Info,
    Debug,
}
/// A spinning indicator for watch output.
#[allow(dead_code)]
pub struct WatchSpinner {
    frames: Vec<&'static str>,
    idx: usize,
}
#[allow(dead_code)]
impl WatchSpinner {
    pub fn new() -> Self {
        Self {
            frames: vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
            idx: 0,
        }
    }
    pub fn tick(&mut self) -> &str {
        let frame = self.frames[self.idx % self.frames.len()];
        self.idx = self.idx.wrapping_add(1);
        frame
    }
    pub fn current(&self) -> &str {
        self.frames[self.idx % self.frames.len()]
    }
}
/// A simple event log that records watch events with descriptions.
#[derive(Clone, Debug, Default)]
pub struct WatchEventLog {
    entries: Vec<(WatchEvent, String)>,
    capacity: usize,
}
impl WatchEventLog {
    /// Create a new event log with a maximum capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entries: Vec::new(),
            capacity,
        }
    }
    /// Append an event with a description.
    pub fn append(&mut self, event: WatchEvent, description: impl Into<String>) {
        if self.entries.len() >= self.capacity && self.capacity > 0 {
            self.entries.remove(0);
        }
        self.entries.push((event, description.into()));
    }
    /// Number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Clear all entries.
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    /// Get entries for a specific event kind.
    pub fn by_kind(&self, kind: WatchEventKind) -> Vec<&(WatchEvent, String)> {
        self.entries
            .iter()
            .filter(|(e, _)| e.kind == kind)
            .collect()
    }
    /// Get descriptions of all entries.
    pub fn descriptions(&self) -> Vec<&str> {
        self.entries.iter().map(|(_, d)| d.as_str()).collect()
    }
}
/// Reconnect policy for watch sessions.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ReconnectPolicy {
    pub max_retries: u32,
    pub retry_delay_ms: u64,
    pub exponential_backoff: bool,
}
#[allow(dead_code)]
impl ReconnectPolicy {
    pub fn default_policy() -> Self {
        Self {
            max_retries: 5,
            retry_delay_ms: 500,
            exponential_backoff: true,
        }
    }
    pub fn delay_for_retry(&self, n: u32) -> u64 {
        if self.exponential_backoff {
            self.retry_delay_ms * (1u64 << n.min(10))
        } else {
            self.retry_delay_ms
        }
    }
    pub fn can_retry(&self, attempts: u32) -> bool {
        attempts < self.max_retries
    }
}
/// Record of a single observed change.
#[derive(Clone, Debug)]
pub struct ChangeRecord {
    /// File that changed.
    pub path: PathBuf,
    /// Kind of change.
    pub kind: WatchEventKind,
    /// When the change was detected.
    pub timestamp: Instant,
    /// What action was taken.
    pub action_taken: WatchAction,
}
/// Hot reload session.
#[allow(dead_code)]
pub struct HotReloadSession {
    state: HotReloadState,
    success_count: u32,
    fail_count: u32,
    last_changed: Option<std::path::PathBuf>,
}
#[allow(dead_code)]
impl HotReloadSession {
    pub fn new() -> Self {
        Self {
            state: HotReloadState::Idle,
            success_count: 0,
            fail_count: 0,
            last_changed: None,
        }
    }
    pub fn trigger(&mut self, path: std::path::PathBuf) {
        self.state = HotReloadState::Pending;
        self.last_changed = Some(path);
    }
    pub fn start_reload(&mut self) {
        self.state = HotReloadState::Reloading;
    }
    pub fn finish_reload(&mut self, success: bool) {
        if success {
            self.success_count += 1;
            self.state = HotReloadState::Done;
        } else {
            self.fail_count += 1;
            self.state = HotReloadState::Failed;
        }
    }
    pub fn reset(&mut self) {
        self.state = HotReloadState::Idle;
    }
    pub fn state(&self) -> HotReloadState {
        self.state
    }
    pub fn status_line(&self) -> String {
        format!(
            "[hot-reload] state={} ok={} fail={}",
            self.state, self.success_count, self.fail_count
        )
    }
}
/// Build request.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BuildRequest {
    pub changed_files: Vec<std::path::PathBuf>,
    pub action: WatchAction,
    pub created_at: std::time::Instant,
    pub supersedes_pending: bool,
}
#[allow(dead_code)]
impl BuildRequest {
    pub fn new(changed_files: Vec<std::path::PathBuf>, action: WatchAction) -> Self {
        Self {
            changed_files,
            action,
            created_at: std::time::Instant::now(),
            supersedes_pending: true,
        }
    }
    pub fn file_count(&self) -> usize {
        self.changed_files.len()
    }
    pub fn describe(&self) -> String {
        format!("{} {} file(s)", self.action, self.changed_files.len())
    }
}
/// Filtering rules for watch events.
#[derive(Clone, Debug)]
pub struct WatchFilter {
    /// Only process files with these extensions (empty = all).
    pub extensions: Vec<String>,
    /// Glob-like ignore patterns (simple substring matching).
    pub ignore_patterns: Vec<String>,
    /// Whether to include hidden files/directories (starting with `.`).
    pub include_hidden: bool,
}
impl WatchFilter {
    /// Create a filter that accepts everything.
    pub fn accept_all() -> Self {
        Self {
            extensions: Vec::new(),
            ignore_patterns: Vec::new(),
            include_hidden: true,
        }
    }
}
/// Backend used for file-system notifications.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatchBackend {
    Polling,
    Inotify,
    Kqueue,
    ReadDirChanges,
}
/// Include/exclude pattern set.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct PatternSet {
    pub include: Vec<String>,
    pub exclude: Vec<String>,
}
#[allow(dead_code)]
impl PatternSet {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn include(mut self, pat: impl Into<String>) -> Self {
        self.include.push(pat.into());
        self
    }
    pub fn exclude(mut self, pat: impl Into<String>) -> Self {
        self.exclude.push(pat.into());
        self
    }
    pub fn matches(&self, path: &std::path::Path) -> bool {
        for pat in &self.exclude {
            if path_matches_pattern(path, pat) || path.to_string_lossy().contains(pat.as_str()) {
                return false;
            }
        }
        if self.include.is_empty() {
            return true;
        }
        self.include.iter().any(|p| path_matches_pattern(path, p))
    }
    pub fn oxilean_sources() -> Self {
        Self::new()
            .include("*.lean")
            .include("*.oxilean")
            .exclude(".git")
            .exclude("build")
    }
}
/// A watch target with its configuration.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct WatchTarget {
    pub path: PathBuf,
    pub config: WatcherConfig,
    pub enabled: bool,
}
impl WatchTarget {
    /// Create a new enabled target.
    #[allow(dead_code)]
    pub fn new(path: impl Into<PathBuf>, config: WatcherConfig) -> Self {
        Self {
            path: path.into(),
            config,
            enabled: true,
        }
    }
}
/// A snapshot of observed file system state for a directory.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct WatcherSnapshot {
    pub file_count: usize,
    pub total_size_bytes: u64,
    pub newest_modification_secs: u64,
}
impl WatcherSnapshot {
    /// Create a snapshot from scanning a directory.
    #[allow(dead_code)]
    pub fn from_dir(dir: &Path, config: &WatcherConfig) -> Self {
        let mut snapshot = Self::default();
        let Ok(entries) = std::fs::read_dir(dir) else {
            return snapshot;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if !config.should_include(&path) || config.should_exclude(&path) {
                continue;
            }
            if let Ok(meta) = entry.metadata() {
                if meta.is_file() {
                    snapshot.file_count += 1;
                    snapshot.total_size_bytes += meta.len();
                    if let Ok(mtime) = meta.modified() {
                        let secs = mtime
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs();
                        if secs > snapshot.newest_modification_secs {
                            snapshot.newest_modification_secs = secs;
                        }
                    }
                }
            }
        }
        snapshot
    }
}
/// Snapshot of a single file's state.
#[derive(Clone, Debug)]
pub struct FileState {
    /// Path to the file.
    pub path: PathBuf,
    /// Last modification time (as duration since UNIX epoch).
    pub modified: Duration,
    /// File size in bytes.
    pub size: u64,
    /// Content hash (FNV-1a).
    pub hash: u64,
}
impl FileState {
    /// Create a new file state from metadata (without reading content).
    pub fn from_metadata(path: PathBuf, meta: &std::fs::Metadata) -> Self {
        let modified = meta
            .modified()
            .ok()
            .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
            .unwrap_or(Duration::ZERO);
        Self {
            path,
            modified,
            size: meta.len(),
            hash: 0,
        }
    }
}
/// A single observed file-system event.
#[derive(Clone, Debug)]
pub struct WatchEvent {
    /// Path of the affected file.
    pub path: PathBuf,
    /// Kind of change.
    pub kind: WatchEventKind,
    /// When the event was detected.
    pub timestamp: Instant,
}
impl WatchEvent {
    /// Create a new watch event.
    pub fn new(path: PathBuf, kind: WatchEventKind) -> Self {
        Self {
            path,
            kind,
            timestamp: Instant::now(),
        }
    }
    /// Return the file extension, if any.
    pub fn extension(&self) -> Option<&str> {
        self.path.extension().and_then(|e| e.to_str())
    }
}
/// A watch session ties a `FileMonitor` to a configuration and processes
/// events via a callback system.
#[derive(Debug)]
pub struct WatchSession {
    /// Underlying file monitor.
    monitor: FileMonitor,
    /// Session configuration.
    config: WatchConfig,
    /// Filter derived from config.
    filter: WatchFilter,
    /// Change tracker.
    tracker: ChangeTracker,
}
impl WatchSession {
    /// Create a new watch session.
    pub fn new(config: WatchConfig) -> Self {
        let filter = WatchFilter {
            extensions: config.extensions.clone(),
            ignore_patterns: config.ignore_dirs.clone(),
            include_hidden: false,
        };
        let monitor = FileMonitor::new(config.debounce_ms);
        Self {
            monitor,
            config,
            filter,
            tracker: ChangeTracker::new(),
        }
    }
    /// Add a directory to watch.
    pub fn watch_directory(&mut self, dir: impl Into<PathBuf>) {
        self.monitor.add_watch(dir);
    }
    /// Start watching and return a handle.
    ///
    /// In a real implementation this would spawn a thread.  Here we return
    /// a handle that can be used to poll manually.
    pub fn start_watch(&mut self) -> WatchHandle {
        let _ = self.monitor.poll_changes();
        WatchHandle::new()
    }
    /// Poll for events, debounce, and process them.  Returns the events
    /// that were acted upon.
    pub fn process_events(&mut self) -> Vec<WatchEvent> {
        let raw = self.monitor.poll_changes();
        let filtered: Vec<WatchEvent> = raw
            .into_iter()
            .filter(|e| should_process_event(e, &self.filter))
            .collect();
        let debounced = debounce(&filtered, self.config.debounce_ms);
        for event in &debounced {
            self.tracker.record_change(ChangeRecord {
                path: event.path.clone(),
                kind: event.kind,
                timestamp: Instant::now(),
                action_taken: self.config.action,
            });
        }
        debounced
    }
    /// Return a reference to the change tracker.
    pub fn tracker(&self) -> &ChangeTracker {
        &self.tracker
    }
    /// Return the current watch configuration.
    pub fn config(&self) -> &WatchConfig {
        &self.config
    }
}
/// Accumulates events within a debounce window.
#[allow(dead_code)]
pub struct EventDebouncer {
    window_ms: u64,
    pending: HashMap<String, WatchEvent>,
    batch_start: Option<std::time::Instant>,
}
#[allow(dead_code)]
impl EventDebouncer {
    pub fn new(window_ms: u64) -> Self {
        Self {
            window_ms,
            pending: HashMap::new(),
            batch_start: None,
        }
    }
    pub fn push(&mut self, event: WatchEvent) {
        if self.batch_start.is_none() {
            self.batch_start = Some(std::time::Instant::now());
        }
        self.pending
            .insert(event.path.to_string_lossy().to_string(), event);
    }
    pub fn is_ready(&self) -> bool {
        self.batch_start
            .map(|t| t.elapsed().as_millis() >= self.window_ms as u128)
            .unwrap_or(false)
    }
    pub fn flush(&mut self) -> Vec<WatchEvent> {
        self.batch_start = None;
        self.pending.drain().map(|(_, v)| v).collect()
    }
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }
    pub fn discard(&mut self) {
        self.pending.clear();
        self.batch_start = None;
    }
}
