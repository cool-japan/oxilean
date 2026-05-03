//! Tests for the file_watcher module (polling-based, pure Rust).

#[cfg(test)]
mod test_impl {
    use crate::file_watcher::{FileEventKind, FileWatcher, WatcherConfig};
    use std::fs;
    use std::io::Write;
    use std::path::PathBuf;

    // --------------------------------------------------------
    // Helpers
    // --------------------------------------------------------

    fn tmp_dir(label: &str) -> PathBuf {
        let mut d = std::env::temp_dir();
        d.push(format!("oxilean_fw_test_{}_{}", label, std::process::id()));
        fs::create_dir_all(&d).expect("create test dir");
        d
    }

    fn write_file(dir: &PathBuf, name: &str, content: &str) -> PathBuf {
        let path = dir.join(name);
        let mut f = fs::File::create(&path).expect("create file");
        f.write_all(content.as_bytes()).expect("write file");
        path
    }

    fn touch_file(path: &PathBuf) {
        // Append a byte to change mtime + size reliably.
        let mut f = fs::OpenOptions::new()
            .append(true)
            .open(path)
            .expect("open for touch");
        f.write_all(b" ").expect("touch write");
    }

    // --------------------------------------------------------
    // 1. Basic construction
    // --------------------------------------------------------
    #[test]
    fn test_new_watcher_empty() {
        let w = FileWatcher::new(WatcherConfig::default());
        assert_eq!(w.snapshot_size(), 0);
        assert_eq!(w.watched_roots().len(), 0);
    }

    // --------------------------------------------------------
    // 2. Watch a directory seeds snapshot (no spurious events)
    // --------------------------------------------------------
    #[test]
    fn test_watch_seeds_snapshot_no_spurious_created() {
        let dir = tmp_dir("seed");
        write_file(&dir, "a.rs", "fn main() {}");
        write_file(&dir, "b.rs", "fn foo() {}");

        let mut w = FileWatcher::new(WatcherConfig::default());
        w.watch(&dir).expect("watch");

        // First poll should return zero events (snapshot was seeded).
        let events = w.poll();
        assert!(
            events.is_empty(),
            "Expected no events after seeding, got: {:?}",
            events
        );
    }

    // --------------------------------------------------------
    // 3. Created event for a new file
    // --------------------------------------------------------
    #[test]
    fn test_created_event() {
        let dir = tmp_dir("created");
        let mut w = FileWatcher::new(WatcherConfig::default());
        w.watch(&dir).expect("watch");
        // No events yet.
        let _ = w.poll();

        // Now create a file.
        write_file(&dir, "new.rs", "let x = 1;");

        let events = w.poll();
        assert_eq!(
            events.len(),
            1,
            "Expected 1 Created event, got {:?}",
            events
        );
        assert_eq!(events[0].kind, FileEventKind::Created);
        assert!(events[0].path.ends_with("new.rs"));
    }

    // --------------------------------------------------------
    // 4. Modified event when mtime/size changes
    // --------------------------------------------------------
    #[test]
    fn test_modified_event() {
        let dir = tmp_dir("modified");
        let file = write_file(&dir, "mod.rs", "initial");

        let mut w = FileWatcher::new(WatcherConfig::default());
        w.watch(&dir).expect("watch");
        let _ = w.poll();

        // Modify the file.
        touch_file(&file);

        let events = w.poll();
        assert_eq!(
            events.len(),
            1,
            "Expected 1 Modified event, got {:?}",
            events
        );
        assert_eq!(events[0].kind, FileEventKind::Modified);
        assert!(events[0].path.ends_with("mod.rs"));
    }

    // --------------------------------------------------------
    // 5. Deleted event when file is removed
    // --------------------------------------------------------
    #[test]
    fn test_deleted_event() {
        let dir = tmp_dir("deleted");
        let file = write_file(&dir, "del.rs", "code");

        let mut w = FileWatcher::new(WatcherConfig::default());
        w.watch(&dir).expect("watch");
        let _ = w.poll();

        fs::remove_file(&file).expect("remove file");

        let events = w.poll();
        assert_eq!(
            events.len(),
            1,
            "Expected 1 Deleted event, got {:?}",
            events
        );
        assert_eq!(events[0].kind, FileEventKind::Deleted);
    }

    // --------------------------------------------------------
    // 6. Multiple events in one poll
    // --------------------------------------------------------
    #[test]
    fn test_multiple_events_in_one_poll() {
        let dir = tmp_dir("multi");
        let file_a = write_file(&dir, "a.rs", "a");
        let file_b = write_file(&dir, "b.rs", "b");

        let mut w = FileWatcher::new(WatcherConfig::default());
        w.watch(&dir).expect("watch");
        let _ = w.poll();

        // Modify a, delete b, create c.
        touch_file(&file_a);
        fs::remove_file(&file_b).expect("remove b");
        write_file(&dir, "c.rs", "c");

        let events = w.poll();
        assert_eq!(events.len(), 3, "Expected 3 events, got {:?}", events);

        let kinds: Vec<_> = events.iter().map(|e| &e.kind).collect();
        assert!(kinds.contains(&&FileEventKind::Modified));
        assert!(kinds.contains(&&FileEventKind::Deleted));
        assert!(kinds.contains(&&FileEventKind::Created));
    }

    // --------------------------------------------------------
    // 7. Snapshot is updated: no duplicate events
    // --------------------------------------------------------
    #[test]
    fn test_no_duplicate_events_after_poll() {
        let dir = tmp_dir("nodup");
        let file = write_file(&dir, "x.rs", "code");

        let mut w = FileWatcher::new(WatcherConfig::default());
        w.watch(&dir).expect("watch");
        let _ = w.poll();

        touch_file(&file);
        let ev1 = w.poll();
        assert_eq!(ev1.len(), 1);

        // Second poll without any further changes should yield nothing.
        let ev2 = w.poll();
        assert!(
            ev2.is_empty(),
            "Expected no events on second poll, got {:?}",
            ev2
        );
    }

    // --------------------------------------------------------
    // 8. Extension filter – only matching files are tracked
    // --------------------------------------------------------
    #[test]
    fn test_extension_filter_excludes_non_matching() {
        let dir = tmp_dir("extfilt");
        let config = WatcherConfig::default().with_extension("lean");

        let mut w = FileWatcher::new(config);
        w.watch(&dir).expect("watch");
        let _ = w.poll();

        write_file(&dir, "file.rs", "rs content");
        write_file(&dir, "file.lean", "lean content");

        let events = w.poll();
        assert_eq!(
            events.len(),
            1,
            "Expected only 1 .lean event, got {:?}",
            events
        );
        assert!(events[0].path.ends_with("file.lean"));
        assert_eq!(events[0].kind, FileEventKind::Created);
    }

    // --------------------------------------------------------
    // 9. Extension filter – multiple extensions
    // --------------------------------------------------------
    #[test]
    fn test_extension_filter_multiple() {
        let dir = tmp_dir("extmulti");
        let config = WatcherConfig::default()
            .with_extension("rs")
            .with_extension("toml");

        let mut w = FileWatcher::new(config);
        w.watch(&dir).expect("watch");
        let _ = w.poll();

        write_file(&dir, "src.rs", "rust");
        write_file(&dir, "Cargo.toml", "[package]");
        write_file(&dir, "readme.md", "# Readme");

        let events = w.poll();
        assert_eq!(events.len(), 2, "Expected 2 events, got {:?}", events);
        for ev in &events {
            let ext = ev.path.extension().and_then(|e| e.to_str()).unwrap_or("");
            assert!(ext == "rs" || ext == "toml", "Unexpected ext: {}", ext);
        }
    }

    // --------------------------------------------------------
    // 10. Non-recursive mode does not descend subdirs
    // --------------------------------------------------------
    #[test]
    fn test_non_recursive_does_not_descend() {
        let dir = tmp_dir("nonrec");
        let sub = dir.join("sub");
        fs::create_dir_all(&sub).expect("create sub");

        let config = WatcherConfig::new(500, false);
        let mut w = FileWatcher::new(config);
        w.watch(&dir).expect("watch");
        let _ = w.poll();

        // File in subdirectory should NOT generate events.
        write_file(&sub, "deep.rs", "deep");

        let events = w.poll();
        assert!(
            events.is_empty(),
            "Non-recursive watcher should ignore subdirectory, got {:?}",
            events
        );
    }

    // --------------------------------------------------------
    // 11. Recursive mode descends subdirs
    // --------------------------------------------------------
    #[test]
    fn test_recursive_descends_subdirs() {
        let dir = tmp_dir("rec");
        let sub = dir.join("sub");
        fs::create_dir_all(&sub).expect("create sub");

        let config = WatcherConfig::new(500, true);
        let mut w = FileWatcher::new(config);
        w.watch(&dir).expect("watch");
        let _ = w.poll();

        write_file(&sub, "deep.rs", "deep");

        let events = w.poll();
        assert_eq!(
            events.len(),
            1,
            "Expected 1 Created in sub, got {:?}",
            events
        );
        assert_eq!(events[0].kind, FileEventKind::Created);
    }

    // --------------------------------------------------------
    // 12. Unwatch stops tracking
    // --------------------------------------------------------
    #[test]
    fn test_unwatch_stops_tracking() {
        let dir = tmp_dir("unwatch");
        write_file(&dir, "a.rs", "a");

        let mut w = FileWatcher::new(WatcherConfig::default());
        w.watch(&dir).expect("watch");
        let _ = w.poll();

        w.unwatch(&dir);
        assert_eq!(w.watched_roots().len(), 0);
        assert_eq!(w.snapshot_size(), 0);

        write_file(&dir, "b.rs", "b");
        let events = w.poll();
        assert!(
            events.is_empty(),
            "Expected no events after unwatch, got {:?}",
            events
        );
    }

    // --------------------------------------------------------
    // 13. Watch a single file (not a directory)
    // --------------------------------------------------------
    #[test]
    fn test_watch_single_file() {
        let dir = tmp_dir("singlefile");
        let file = write_file(&dir, "only.rs", "content");

        let mut w = FileWatcher::new(WatcherConfig::default());
        w.watch(&file).expect("watch file");
        let _ = w.poll();

        touch_file(&file);

        let events = w.poll();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].kind, FileEventKind::Modified);
    }

    // --------------------------------------------------------
    // 14. poll_interval_ms accessor
    // --------------------------------------------------------
    #[test]
    fn test_poll_interval_ms() {
        let config = WatcherConfig {
            poll_interval_ms: 1234,
            ..Default::default()
        };
        let w = FileWatcher::new(config);
        assert_eq!(w.poll_interval_ms(), 1234);
    }

    // --------------------------------------------------------
    // 15. WatcherConfig::matches_extension empty list matches all
    // --------------------------------------------------------
    #[test]
    fn test_matches_extension_empty_list_matches_all() {
        let config = WatcherConfig::default();
        assert!(config.matches_extension(std::path::Path::new("foo.rs")));
        assert!(config.matches_extension(std::path::Path::new("bar.lean")));
        assert!(config.matches_extension(std::path::Path::new("noext")));
    }

    // --------------------------------------------------------
    // 16. WatcherConfig::matches_extension no ext on path
    // --------------------------------------------------------
    #[test]
    fn test_matches_extension_no_ext() {
        let config = WatcherConfig::default().with_extension("rs");
        // A file with no extension should NOT match.
        assert!(!config.matches_extension(std::path::Path::new("Makefile")));
    }

    // --------------------------------------------------------
    // 17. Duplicate watch calls are idempotent
    // --------------------------------------------------------
    #[test]
    fn test_duplicate_watch_is_idempotent() {
        let dir = tmp_dir("dupwatch");
        write_file(&dir, "a.rs", "a");

        let mut w = FileWatcher::new(WatcherConfig::default());
        w.watch(&dir).expect("watch 1");
        w.watch(&dir).expect("watch 2");

        assert_eq!(w.watched_roots().len(), 1, "Root should appear only once");

        let _ = w.poll();
        write_file(&dir, "b.rs", "b");
        let events = w.poll();
        // Should see exactly 1 Created (b.rs), not 2.
        assert_eq!(
            events.len(),
            1,
            "Expected 1 Created event, got {:?}",
            events
        );
    }

    // --------------------------------------------------------
    // 18. Watch then delete watched directory itself
    // --------------------------------------------------------
    #[test]
    fn test_deleted_directory_yields_deleted_events() {
        let dir = tmp_dir("deldir");
        let file = write_file(&dir, "x.rs", "x");

        let mut w = FileWatcher::new(WatcherConfig::default());
        w.watch(&dir).expect("watch");
        let _ = w.poll();

        fs::remove_file(&file).expect("remove file");
        fs::remove_dir(&dir).expect("remove dir");

        let events = w.poll();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].kind, FileEventKind::Deleted);
    }

    // --------------------------------------------------------
    // 19. Snapshot reflects latest state after repeated polls
    // --------------------------------------------------------
    #[test]
    fn test_snapshot_tracks_latest_state() {
        let dir = tmp_dir("snaptrack");
        let file = write_file(&dir, "t.rs", "v1");

        let mut w = FileWatcher::new(WatcherConfig::default());
        w.watch(&dir).expect("watch");
        let _ = w.poll();

        touch_file(&file); // v2
        let ev1 = w.poll();
        assert_eq!(ev1.len(), 1);
        assert_eq!(ev1[0].kind, FileEventKind::Modified);

        touch_file(&file); // v3
        let ev2 = w.poll();
        assert_eq!(ev2.len(), 1);
        assert_eq!(ev2[0].kind, FileEventKind::Modified);
    }

    // --------------------------------------------------------
    // 20. Empty directory – no events
    // --------------------------------------------------------
    #[test]
    fn test_empty_directory_no_events() {
        let dir = tmp_dir("emptydir");
        let mut w = FileWatcher::new(WatcherConfig::default());
        w.watch(&dir).expect("watch");
        let events = w.poll();
        assert!(
            events.is_empty(),
            "Empty dir should yield no events, got {:?}",
            events
        );
    }
}
