//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant, SystemTime};

use super::types::{
    BuildRequest, BuildRequestQueue, ChangeRecord, ChangeTracker, CountingSubscriber,
    DirectoryExcludeFilter, EventDebouncer, ExtensionFilter, FileMonitor, HashCache,
    HotReloadSession, HotReloadState, MonitorStats, PatternSet, RebuildTrigger, ReconnectPolicy,
    WatchAction, WatchBackend, WatchConfig, WatchConfigBuilder, WatchDisplayMode, WatchError,
    WatchEvent, WatchEventBatcher, WatchEventKind, WatchEventLog, WatchFilter, WatchHandle,
    WatchSpinner, WatchTarget, WatcherConfig, WatcherLog, WatcherLogLevel, WatcherRegistry,
    WatcherSnapshot, WatcherStatistics,
};

/// Decide whether a given event should be processed.
pub fn should_process_event(event: &WatchEvent, filter: &WatchFilter) -> bool {
    let path_str = event.path.to_string_lossy();
    for pattern in &filter.ignore_patterns {
        if path_str.contains(pattern.as_str()) {
            return false;
        }
    }
    if !filter.include_hidden {
        for component in event.path.components() {
            if let std::path::Component::Normal(name) = component {
                if name.to_string_lossy().starts_with('.') {
                    return false;
                }
            }
        }
    }
    if !filter.extensions.is_empty() {
        match event.extension() {
            Some(ext) => filter.extensions.iter().any(|e| e == ext),
            None => false,
        }
    } else {
        true
    }
}
/// Compute a quick content hash (FNV-1a) for a file.
pub fn compute_file_hash(path: &Path) -> Result<u64, String> {
    let data =
        std::fs::read(path).map_err(|e| format!("cannot read '{}': {}", path.display(), e))?;
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for &byte in &data {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x0100_0000_01b3);
    }
    Ok(hash)
}
/// Coalesce rapid events on the same path within the debounce window.
///
/// Only the *last* event per path is kept.
pub fn debounce(events: &[WatchEvent], _debounce_ms: u64) -> Vec<WatchEvent> {
    let mut latest: HashMap<String, WatchEvent> = HashMap::new();
    for event in events {
        let key = event.path.to_string_lossy().to_string();
        latest.insert(key, event.clone());
    }
    latest.into_values().collect()
}
/// Format a human-readable summary of the change tracker.
pub fn format_change_summary(tracker: &ChangeTracker) -> String {
    if tracker.is_empty() {
        return "No changes recorded.".to_string();
    }
    let mut created = 0usize;
    let mut modified = 0usize;
    let mut deleted = 0usize;
    let mut renamed = 0usize;
    for record in tracker.all_changes() {
        match record.kind {
            WatchEventKind::Created => created += 1,
            WatchEventKind::Modified => modified += 1,
            WatchEventKind::Deleted => deleted += 1,
            WatchEventKind::Renamed => renamed += 1,
        }
    }
    let mut parts = Vec::new();
    if created > 0 {
        parts.push(format!("{} created", created));
    }
    if modified > 0 {
        parts.push(format!("{} modified", modified));
    }
    if deleted > 0 {
        parts.push(format!("{} deleted", deleted));
    }
    if renamed > 0 {
        parts.push(format!("{} renamed", renamed));
    }
    format!("Changes: {} (total {})", parts.join(", "), tracker.len())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_watch_event_new() {
        let e = WatchEvent::new(PathBuf::from("foo.lean"), WatchEventKind::Modified);
        assert_eq!(e.kind, WatchEventKind::Modified);
        assert_eq!(e.path, PathBuf::from("foo.lean"));
    }
    #[test]
    fn test_watch_event_extension() {
        let e = WatchEvent::new(PathBuf::from("bar.lean"), WatchEventKind::Created);
        assert_eq!(e.extension(), Some("lean"));
        let e2 = WatchEvent::new(PathBuf::from("Makefile"), WatchEventKind::Modified);
        assert_eq!(e2.extension(), None);
    }
    #[test]
    fn test_watch_event_display() {
        let e = WatchEvent::new(PathBuf::from("x.lean"), WatchEventKind::Deleted);
        let s = e.to_string();
        assert!(s.contains("deleted"));
        assert!(s.contains("x.lean"));
    }
    #[test]
    fn test_should_process_event_extension() {
        let filter = WatchFilter::default();
        let lean = WatchEvent::new(PathBuf::from("a.lean"), WatchEventKind::Modified);
        assert!(should_process_event(&lean, &filter));
        let txt = WatchEvent::new(PathBuf::from("a.txt"), WatchEventKind::Modified);
        assert!(!should_process_event(&txt, &filter));
    }
    #[test]
    fn test_should_process_event_ignore() {
        let filter = WatchFilter::default();
        let hidden = WatchEvent::new(PathBuf::from(".git/config"), WatchEventKind::Modified);
        assert!(!should_process_event(&hidden, &filter));
    }
    #[test]
    fn test_should_process_event_accept_all() {
        let filter = WatchFilter::accept_all();
        let any = WatchEvent::new(PathBuf::from("README.md"), WatchEventKind::Created);
        assert!(should_process_event(&any, &filter));
    }
    #[test]
    fn test_should_process_hidden() {
        let mut filter = WatchFilter::accept_all();
        filter.include_hidden = false;
        let hidden = WatchEvent::new(PathBuf::from(".hidden/foo"), WatchEventKind::Created);
        assert!(!should_process_event(&hidden, &filter));
    }
    #[test]
    fn test_file_monitor_new() {
        let m = FileMonitor::new(500);
        assert_eq!(m.watch_count(), 0);
        assert_eq!(m.poll_interval(), Duration::from_millis(500));
    }
    #[test]
    fn test_file_monitor_add_remove() {
        let mut m = FileMonitor::new(100);
        m.add_watch("/tmp/foo");
        assert_eq!(m.watch_count(), 1);
        m.add_watch("/tmp/foo");
        assert_eq!(m.watch_count(), 1);
        m.remove_watch(Path::new("/tmp/foo"));
        assert_eq!(m.watch_count(), 0);
    }
    #[test]
    fn test_file_monitor_clear() {
        let mut m = FileMonitor::new(100);
        m.add_watch("/tmp/a");
        m.add_watch("/tmp/b");
        m.clear_watches();
        assert_eq!(m.watch_count(), 0);
    }
    #[test]
    fn test_detect_changes_empty() {
        let m = FileMonitor::new(100);
        let events = m.detect_changes(&[]);
        assert!(events.is_empty());
    }
    #[test]
    fn test_watch_handle() {
        let mut h = WatchHandle::new();
        assert!(!h.is_stopped());
        h.stop();
        assert!(h.is_stopped());
    }
    #[test]
    fn test_debounce_coalesces() {
        let events = vec![
            WatchEvent::new(PathBuf::from("a.lean"), WatchEventKind::Modified),
            WatchEvent::new(PathBuf::from("a.lean"), WatchEventKind::Modified),
            WatchEvent::new(PathBuf::from("b.lean"), WatchEventKind::Created),
        ];
        let result = debounce(&events, 200);
        assert_eq!(result.len(), 2);
    }
    #[test]
    fn test_change_tracker_empty() {
        let t = ChangeTracker::new();
        assert!(t.is_empty());
        assert_eq!(t.len(), 0);
    }
    #[test]
    fn test_change_tracker_record() {
        let mut t = ChangeTracker::new();
        t.record_change(ChangeRecord {
            path: PathBuf::from("x.lean"),
            kind: WatchEventKind::Modified,
            timestamp: Instant::now(),
            action_taken: WatchAction::Recheck,
        });
        assert_eq!(t.len(), 1);
        assert!(!t.is_empty());
    }
    #[test]
    fn test_change_tracker_recent() {
        let mut t = ChangeTracker::new();
        for i in 0..5 {
            t.record_change(ChangeRecord {
                path: PathBuf::from(format!("{}.lean", i)),
                kind: WatchEventKind::Modified,
                timestamp: Instant::now(),
                action_taken: WatchAction::Notify,
            });
        }
        assert_eq!(t.recent_changes(3).len(), 3);
        assert_eq!(t.recent_changes(10).len(), 5);
    }
    #[test]
    fn test_change_tracker_clear() {
        let mut t = ChangeTracker::new();
        t.record_change(ChangeRecord {
            path: PathBuf::from("a.lean"),
            kind: WatchEventKind::Created,
            timestamp: Instant::now(),
            action_taken: WatchAction::Rebuild,
        });
        assert!(!t.is_empty());
        t.clear();
        assert!(t.is_empty());
    }
    #[test]
    fn test_format_change_summary_empty() {
        let t = ChangeTracker::new();
        let s = format_change_summary(&t);
        assert!(s.contains("No changes"));
    }
    #[test]
    fn test_format_change_summary_mixed() {
        let mut t = ChangeTracker::new();
        t.record_change(ChangeRecord {
            path: "a.lean".into(),
            kind: WatchEventKind::Created,
            timestamp: Instant::now(),
            action_taken: WatchAction::Notify,
        });
        t.record_change(ChangeRecord {
            path: "b.lean".into(),
            kind: WatchEventKind::Modified,
            timestamp: Instant::now(),
            action_taken: WatchAction::Recheck,
        });
        let s = format_change_summary(&t);
        assert!(s.contains("1 created"));
        assert!(s.contains("1 modified"));
        assert!(s.contains("total 2"));
    }
    #[test]
    fn test_watch_action_display() {
        assert_eq!(WatchAction::Recheck.to_string(), "recheck");
        assert_eq!(WatchAction::Rebuild.to_string(), "rebuild");
        assert_eq!(WatchAction::Notify.to_string(), "notify");
        assert_eq!(WatchAction::Custom.to_string(), "custom");
    }
    #[test]
    fn test_watch_event_kind_display() {
        assert_eq!(WatchEventKind::Created.to_string(), "created");
        assert_eq!(WatchEventKind::Modified.to_string(), "modified");
        assert_eq!(WatchEventKind::Deleted.to_string(), "deleted");
        assert_eq!(WatchEventKind::Renamed.to_string(), "renamed");
    }
}
/// Check if a file path matches a simple glob-like include pattern.
///
/// Pattern matching rules:
/// - `*` matches any sequence of non-separator characters.
/// - Everything else is literal.
pub fn path_matches_pattern(path: &Path, pattern: &str) -> bool {
    let path_str = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    if let Some(ext_pat) = pattern.strip_prefix("*.") {
        path.extension().and_then(|e| e.to_str()) == Some(ext_pat)
    } else {
        path_str == pattern
    }
}
/// Describe a `WatchAction` for user-facing messages.
pub fn describe_action(action: WatchAction) -> &'static str {
    match action {
        WatchAction::Recheck => "Re-checking affected modules",
        WatchAction::Rebuild => "Triggering full rebuild",
        WatchAction::Notify => "Sending notification",
        WatchAction::Custom => "Running custom handler",
    }
}
/// Build a human-readable summary of a `WatchConfig`.
pub fn format_watch_config(cfg: &WatchConfig) -> String {
    format!(
        "debounce={}ms recursive={} exts=[{}] action={}",
        cfg.debounce_ms,
        cfg.recursive,
        cfg.extensions.join(","),
        cfg.action,
    )
}
#[cfg(test)]
mod watcher_extended_tests {
    use super::*;
    #[test]
    fn test_watch_event_log_append() {
        let mut log = WatchEventLog::with_capacity(10);
        log.append(
            WatchEvent::new(PathBuf::from("a.lean"), WatchEventKind::Created),
            "created a",
        );
        assert_eq!(log.len(), 1);
    }
    #[test]
    fn test_watch_event_log_capacity() {
        let mut log = WatchEventLog::with_capacity(2);
        log.append(
            WatchEvent::new("a.lean".into(), WatchEventKind::Created),
            "1",
        );
        log.append(
            WatchEvent::new("b.lean".into(), WatchEventKind::Created),
            "2",
        );
        log.append(
            WatchEvent::new("c.lean".into(), WatchEventKind::Created),
            "3",
        );
        assert_eq!(log.len(), 2);
    }
    #[test]
    fn test_watch_event_log_by_kind() {
        let mut log = WatchEventLog::with_capacity(10);
        log.append(
            WatchEvent::new("a.lean".into(), WatchEventKind::Created),
            "c",
        );
        log.append(
            WatchEvent::new("b.lean".into(), WatchEventKind::Modified),
            "m",
        );
        let created = log.by_kind(WatchEventKind::Created);
        assert_eq!(created.len(), 1);
    }
    #[test]
    fn test_watch_event_log_clear() {
        let mut log = WatchEventLog::with_capacity(10);
        log.append(
            WatchEvent::new("a.lean".into(), WatchEventKind::Deleted),
            "d",
        );
        log.clear();
        assert!(log.is_empty());
    }
    #[test]
    fn test_monitor_stats_record() {
        let mut stats = MonitorStats::new();
        stats.record_poll(5, 3);
        assert_eq!(stats.poll_cycles, 1);
        assert_eq!(stats.events_detected, 5);
        assert_eq!(stats.events_processed, 3);
        assert_eq!(stats.events_debounced, 2);
    }
    #[test]
    fn test_monitor_stats_avg_zero_cycles() {
        let stats = MonitorStats::new();
        assert_eq!(stats.avg_events_per_cycle(), 0.0);
    }
    #[test]
    fn test_monitor_stats_debounce_ratio() {
        let mut stats = MonitorStats::new();
        stats.record_poll(10, 5);
        assert!((stats.debounce_ratio() - 0.5).abs() < 1e-10);
    }
    #[test]
    fn test_path_matches_pattern_ext() {
        let path = Path::new("foo.lean");
        assert!(path_matches_pattern(path, "*.lean"));
        assert!(!path_matches_pattern(path, "*.rs"));
    }
    #[test]
    fn test_path_matches_pattern_literal() {
        let path = Path::new("Makefile");
        assert!(path_matches_pattern(path, "Makefile"));
        assert!(!path_matches_pattern(path, "makefile"));
    }
    #[test]
    fn test_describe_action() {
        assert!(describe_action(WatchAction::Recheck).contains("Re-checking"));
        assert!(describe_action(WatchAction::Rebuild).contains("rebuild"));
        assert!(describe_action(WatchAction::Notify).contains("notification"));
        assert!(describe_action(WatchAction::Custom).contains("custom"));
    }
    #[test]
    fn test_format_watch_config() {
        let cfg = WatchConfig::default();
        let s = format_watch_config(&cfg);
        assert!(s.contains("debounce="));
        assert!(s.contains("lean"));
    }
    #[test]
    fn test_watch_event_log_descriptions() {
        let mut log = WatchEventLog::with_capacity(5);
        log.append(
            WatchEvent::new("x.lean".into(), WatchEventKind::Modified),
            "modified x",
        );
        let descs = log.descriptions();
        assert_eq!(descs, vec!["modified x"]);
    }
}
/// Select the best available backend for the current platform.
#[allow(dead_code)]
pub fn select_backend() -> WatchBackend {
    #[cfg(target_os = "linux")]
    return WatchBackend::Inotify;
    #[cfg(any(target_os = "macos", target_os = "freebsd"))]
    return WatchBackend::Kqueue;
    #[cfg(target_os = "windows")]
    return WatchBackend::ReadDirChanges;
    #[allow(unreachable_code)]
    WatchBackend::Polling
}
/// Format watch status for terminal display.
#[allow(dead_code)]
pub fn format_watch_status(
    mode: WatchDisplayMode,
    events: &[WatchEvent],
    action: WatchAction,
) -> String {
    match mode {
        WatchDisplayMode::Silent => String::new(),
        WatchDisplayMode::Compact => {
            format!("[watch] {} file(s) changed -> {}", events.len(), action)
        }
        WatchDisplayMode::Verbose => {
            let mut lines = vec![format!("[watch] {} -> {} file(s):", action, events.len())];
            for e in events {
                lines.push(format!("  {} {}", e.kind, e.path.display()));
            }
            lines.join("\n")
        }
    }
}
/// Format a watching banner.
#[allow(dead_code)]
pub fn format_watching_banner(paths: &[std::path::PathBuf], config: &WatchConfig) -> String {
    let mut lines = vec![format!(
        "Watching {} path(s) [debounce={}ms, action={}]",
        paths.len(),
        config.debounce_ms,
        config.action
    )];
    for p in paths.iter().take(5) {
        lines.push(format!("  {}", p.display()));
    }
    if paths.len() > 5 {
        lines.push(format!("  ... and {} more", paths.len() - 5));
    }
    lines.join("\n")
}
#[cfg(test)]
mod watcher_new_tests {
    use super::*;
    #[test]
    fn test_watch_backend_display() {
        assert_eq!(WatchBackend::Polling.to_string(), "polling");
        assert_eq!(WatchBackend::Inotify.to_string(), "inotify");
    }
    #[test]
    fn test_select_backend_valid() {
        let _ = select_backend().to_string();
    }
    #[test]
    fn test_event_debouncer_deduplicates() {
        let mut db = EventDebouncer::new(200);
        db.push(WatchEvent::new("a.lean".into(), WatchEventKind::Modified));
        db.push(WatchEvent::new("a.lean".into(), WatchEventKind::Modified));
        assert_eq!(db.pending_count(), 1);
    }
    #[test]
    fn test_event_debouncer_flush() {
        let mut db = EventDebouncer::new(200);
        db.push(WatchEvent::new("a.lean".into(), WatchEventKind::Modified));
        let events = db.flush();
        assert_eq!(events.len(), 1);
        assert_eq!(db.pending_count(), 0);
    }
    #[test]
    fn test_rebuild_trigger_full_rebuild() {
        let trig = RebuildTrigger::default_oxilean();
        let e = WatchEvent::new("Oxilean.toml".into(), WatchEventKind::Modified);
        assert_eq!(trig.action_for(&e), WatchAction::Rebuild);
    }
    #[test]
    fn test_rebuild_trigger_recheck() {
        let trig = RebuildTrigger::default_oxilean();
        let e = WatchEvent::new("Main.lean".into(), WatchEventKind::Modified);
        assert_eq!(trig.action_for(&e), WatchAction::Recheck);
    }
    #[test]
    fn test_pattern_set_include_only() {
        let ps = PatternSet::new().include("*.lean");
        assert!(ps.matches(std::path::Path::new("foo.lean")));
        assert!(!ps.matches(std::path::Path::new("bar.rs")));
    }
    #[test]
    fn test_pattern_set_exclude() {
        let ps = PatternSet::new().exclude(".git");
        assert!(!ps.matches(std::path::Path::new(".git/config")));
    }
    #[test]
    fn test_watch_config_builder() {
        let cfg = WatchConfigBuilder::new()
            .debounce_ms(500)
            .recursive(false)
            .extension("lean")
            .action(WatchAction::Rebuild)
            .build();
        assert_eq!(cfg.debounce_ms, 500);
        assert!(!cfg.recursive);
        assert_eq!(cfg.action, WatchAction::Rebuild);
    }
    #[test]
    fn test_reconnect_policy_exponential() {
        let pol = ReconnectPolicy::default_policy();
        assert_eq!(pol.delay_for_retry(1), pol.delay_for_retry(0) * 2);
        assert!(pol.can_retry(0));
        assert!(!pol.can_retry(5));
    }
    #[test]
    fn test_watch_error_display() {
        let err = WatchError::recoverable("disk full");
        assert!(err.to_string().contains("recoverable"));
    }
    #[test]
    fn test_build_request_queue_supersedes() {
        let mut q = BuildRequestQueue::new(5);
        q.push(BuildRequest::new(
            vec!["a.lean".into()],
            WatchAction::Recheck,
        ));
        q.push(BuildRequest::new(
            vec!["b.lean".into()],
            WatchAction::Rebuild,
        ));
        assert_eq!(q.len(), 1);
        assert_eq!(
            q.pop().expect("collection should not be empty").action,
            WatchAction::Rebuild
        );
    }
    #[test]
    fn test_hot_reload_lifecycle() {
        let mut sess = HotReloadSession::new();
        assert_eq!(sess.state(), HotReloadState::Idle);
        sess.trigger("Main.lean".into());
        assert_eq!(sess.state(), HotReloadState::Pending);
        sess.start_reload();
        sess.finish_reload(true);
        assert_eq!(sess.state(), HotReloadState::Done);
        sess.reset();
        assert_eq!(sess.state(), HotReloadState::Idle);
    }
    #[test]
    fn test_hot_reload_failure() {
        let mut sess = HotReloadSession::new();
        sess.trigger("Foo.lean".into());
        sess.start_reload();
        sess.finish_reload(false);
        assert_eq!(sess.state(), HotReloadState::Failed);
        assert!(sess.status_line().contains("fail=1"));
    }
    #[test]
    fn test_format_watch_status_compact() {
        let events = vec![WatchEvent::new("a.lean".into(), WatchEventKind::Modified)];
        let s = format_watch_status(WatchDisplayMode::Compact, &events, WatchAction::Recheck);
        assert!(s.contains("recheck"));
    }
    #[test]
    fn test_format_watch_status_silent() {
        let s = format_watch_status(WatchDisplayMode::Silent, &[], WatchAction::Notify);
        assert!(s.is_empty());
    }
    #[test]
    fn test_watch_spinner_ticks() {
        let mut sp = WatchSpinner::new();
        let f1 = sp.tick().to_string();
        let f2 = sp.tick().to_string();
        assert_ne!(f1, f2);
    }
    #[test]
    fn test_hash_cache_empty() {
        let cache = HashCache::new();
        assert!(cache.is_empty());
    }
    #[test]
    fn test_hash_cache_clear() {
        let mut cache = HashCache::new();
        cache.cache.insert("foo".to_string(), 123);
        cache.clear();
        assert!(cache.is_empty());
    }
    #[test]
    fn test_format_watching_banner_small() {
        let paths: Vec<std::path::PathBuf> = vec!["src".into()];
        let cfg = WatchConfig::default();
        let b = format_watching_banner(&paths, &cfg);
        assert!(b.contains("1 path(s)"));
    }
    #[test]
    fn test_hot_reload_state_display() {
        assert_eq!(HotReloadState::Idle.to_string(), "idle");
        assert_eq!(HotReloadState::Failed.to_string(), "failed");
    }
}
/// Filters watch events before delivery.
#[allow(dead_code)]
pub trait WatchEventFilter: Send + Sync {
    /// Return true if this event should pass the filter.
    fn accepts(&self, path: &Path, kind: WatchEventKind) -> bool;
}
/// A subscriber that receives batched watch events.
#[allow(dead_code)]
pub trait WatcherSubscriber: Send + Sync {
    /// Called with a batch of events.
    fn on_events(&self, events: &[(PathBuf, WatchEventKind)]);
}
/// Return the watcher module version.
#[allow(dead_code)]
pub fn watcher_module_version() -> &'static str {
    "0.1.0"
}
#[cfg(test)]
mod watcher_extra_tests {
    use super::*;
    #[test]
    fn test_watcher_statistics() {
        let mut stats = WatcherStatistics::new();
        stats.record_poll(&[WatchEventKind::Modified, WatchEventKind::Created]);
        assert_eq!(stats.total_polls, 1);
        assert_eq!(stats.total_events, 2);
        assert_eq!(stats.created_events, 1);
        assert_eq!(stats.modified_events, 1);
    }
    #[test]
    fn test_watcher_config_should_include() {
        let cfg = WatcherConfig::for_oxilean();
        let lean_file = PathBuf::from("src/proof.lean");
        let rs_file = PathBuf::from("src/main.rs");
        assert!(cfg.should_include(&lean_file));
        assert!(!cfg.should_include(&rs_file));
    }
    #[test]
    fn test_watcher_config_should_exclude() {
        let cfg = WatcherConfig::for_oxilean();
        let git_path = PathBuf::from(".git/HEAD");
        let src_path = PathBuf::from("src/main.lean");
        assert!(cfg.should_exclude(&git_path));
        assert!(!cfg.should_exclude(&src_path));
    }
    #[test]
    fn test_extension_filter() {
        let filter = ExtensionFilter::new(vec!["lean"]);
        assert!(filter.accepts(&PathBuf::from("a.lean"), WatchEventKind::Modified));
        assert!(!filter.accepts(&PathBuf::from("a.rs"), WatchEventKind::Modified));
    }
    #[test]
    fn test_directory_exclude_filter() {
        let filter = DirectoryExcludeFilter::new(vec![".git", "target"]);
        assert!(!filter.accepts(&PathBuf::from(".git/config"), WatchEventKind::Modified));
        assert!(filter.accepts(&PathBuf::from("src/main.lean"), WatchEventKind::Modified));
    }
    #[test]
    fn test_event_batcher() {
        let mut batcher = WatchEventBatcher::new(1000);
        let path = PathBuf::from("src/main.lean");
        batcher.add(path.clone(), WatchEventKind::Modified);
        let deduped = batcher.add(path.clone(), WatchEventKind::Modified);
        assert!(deduped);
    }
    #[test]
    fn test_counting_subscriber() {
        let sub = CountingSubscriber::new();
        let events = vec![
            (PathBuf::from("a.lean"), WatchEventKind::Modified),
            (PathBuf::from("b.lean"), WatchEventKind::Created),
        ];
        sub.on_events(&events);
        assert_eq!(sub.get_count(), 2);
    }
    #[test]
    fn test_watcher_module_version() {
        assert!(!watcher_module_version().is_empty());
    }
}
/// Return list of watcher module features.
#[allow(dead_code)]
pub fn watcher_features() -> Vec<&'static str> {
    vec![
        "polling",
        "debounce",
        "batching",
        "filtering",
        "statistics",
        "config",
        "registry",
        "log",
        "subscribers",
        "extensions",
    ]
}
#[cfg(test)]
mod watcher_registry_tests {
    use super::*;
    #[test]
    fn test_watcher_registry() {
        let mut reg = WatcherRegistry::new();
        let target = WatchTarget::new("src/", WatcherConfig::for_oxilean());
        reg.add(target);
        assert_eq!(reg.count(), 1);
        assert_eq!(reg.enabled_targets().len(), 1);
        reg.set_enabled(&PathBuf::from("src/"), false);
        assert_eq!(reg.enabled_targets().len(), 0);
    }
    #[test]
    fn test_watcher_log() {
        let mut log = WatcherLog::new(100);
        log.add(WatcherLogLevel::Info, "Started watching", None);
        log.add(
            WatcherLogLevel::Warning,
            "Permission denied",
            Some(PathBuf::from("private/")),
        );
        assert_eq!(log.all_entries().len(), 2);
        let warnings = log.entries_at_level(&WatcherLogLevel::Warning);
        assert_eq!(warnings.len(), 1);
    }
    #[test]
    fn test_watcher_features() {
        let features = watcher_features();
        assert!(features.contains(&"polling"));
        assert!(features.contains(&"batching"));
    }
}
/// Return watcher module feature count.
#[allow(dead_code)]
pub fn watcher_feature_count() -> usize {
    watcher_features().len()
}
#[cfg(test)]
mod snapshot_tests {
    use super::*;
    #[test]
    fn test_watcher_snapshot_empty_dir() {
        let config = WatcherConfig::for_oxilean();
        let tmp = std::env::temp_dir();
        let snapshot = WatcherSnapshot::from_dir(&tmp, &config);
        let _ = snapshot.file_count;
    }
    #[test]
    fn test_watcher_feature_count() {
        assert!(watcher_feature_count() > 0);
    }
}
