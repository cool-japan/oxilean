//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet, VecDeque};

use super::types::{
    ArtifactTracker, BuildCycle, BuildScheduleHint, ChangeBatch, ChangeDetector, CompileHistory,
    ConcurrentInvalidationSet, CycleDetector, DependencyGraphBuilder, DirtySet, EdgeKind,
    FileFingerprint, FingerprintStore, FullIncrementalCache, GraphEdgeKindFilter, IncrArtifactKind,
    IncrementalBuildMetrics, IncrementalBuildReport, IncrementalBuildState, IncrementalCacheEntry,
    IncrementalCompilationConfig, IncrementalEngine, IncrementalFingerprinter, IncrementalGraph,
    IncrementalScheduler, IncrementalStats, InterfaceHash, InvalidationCause, InvalidationLog,
    ModuleCache, ModuleCompileRecord, RebuildQueue, SourceRootTracker, WatchEvent,
    WatchEventProcessor,
};

#[cfg(test)]
mod tests {
    use super::*;
    fn fp(path: &str, hash: u64) -> FileFingerprint {
        FileFingerprint::new(path, 1024, 1_000_000, hash)
    }
    #[test]
    fn fingerprint_matches_identical() {
        let a = fp("Mod.lean", 42);
        let b = fp("Mod.lean", 42);
        assert!(a.matches(&b));
    }
    #[test]
    fn fingerprint_mismatch_on_hash_change() {
        let a = fp("Mod.lean", 42);
        let b = fp("Mod.lean", 99);
        assert!(!a.matches(&b));
    }
    #[test]
    fn graph_invalidate_propagates() {
        let mut g = IncrementalGraph::new();
        g.add_module("A", fp("A.lean", 1));
        g.add_module("B", fp("B.lean", 2));
        g.add_module("C", fp("C.lean", 3));
        g.add_edge("B", "A", EdgeKind::Direct);
        g.add_edge("C", "B", EdgeKind::Direct);
        g.invalidate("A");
        assert!(g.invalidated.contains("A"));
        assert!(g.invalidated.contains("B"));
        assert!(g.invalidated.contains("C"));
    }
    #[test]
    fn graph_clean_count() {
        let mut g = IncrementalGraph::new();
        g.add_module("A", fp("A.lean", 1));
        g.add_module("B", fp("B.lean", 2));
        g.add_module("C", fp("C.lean", 3));
        g.add_edge("B", "A", EdgeKind::Direct);
        g.invalidate("A");
        assert_eq!(g.clean_count(), 1);
    }
    #[test]
    fn graph_fingerprint_changed_detects_update() {
        let mut g = IncrementalGraph::new();
        g.add_module("Mod", fp("Mod.lean", 10));
        assert!(!g.fingerprint_changed("Mod", &fp("Mod.lean", 10)));
        assert!(g.fingerprint_changed("Mod", &fp("Mod.lean", 99)));
    }
    #[test]
    fn graph_update_fingerprint() {
        let mut g = IncrementalGraph::new();
        g.add_module("Mod", fp("Mod.lean", 10));
        g.update_fingerprint("Mod", fp("Mod.lean", 20));
        assert!(!g.fingerprint_changed("Mod", &fp("Mod.lean", 20)));
    }
    #[test]
    fn change_batch_totals() {
        let mut batch = ChangeBatch::new();
        assert!(batch.is_empty());
        batch.record_change("a.lean");
        batch.record_add("b.lean");
        batch.record_remove("c.lean");
        assert!(!batch.is_empty());
        assert_eq!(batch.total_changes(), 3);
    }
    #[test]
    fn change_batch_records_correctly() {
        let mut batch = ChangeBatch::new();
        batch.record_change("x.lean");
        batch.record_change("y.lean");
        assert_eq!(batch.changed_files.len(), 2);
        assert_eq!(batch.added_files.len(), 0);
        assert_eq!(batch.removed_files.len(), 0);
    }
}
#[cfg(test)]
mod extended_tests {
    use super::*;
    #[test]
    fn artifact_tracker_record_and_get() {
        let mut t = ArtifactTracker::new();
        t.record("Mod.A", "build/Mod.A.o");
        t.record("Mod.A", "build/Mod.A.d");
        assert_eq!(t.get("Mod.A").len(), 2);
        assert!(t.has_artifacts("Mod.A"));
        assert_eq!(t.total_artifacts(), 2);
    }
    #[test]
    fn artifact_tracker_remove() {
        let mut t = ArtifactTracker::new();
        t.record("X", "build/X.o");
        t.remove("X");
        assert!(!t.has_artifacts("X"));
    }
    #[test]
    fn fingerprinter_is_up_to_date() {
        let mut fp = IncrementalFingerprinter::new();
        fp.record("Mod", b"source content");
        assert!(fp.is_up_to_date("Mod", b"source content"));
        assert!(!fp.is_up_to_date("Mod", b"changed content"));
    }
    #[test]
    fn fingerprinter_invalidate() {
        let mut fp = IncrementalFingerprinter::new();
        fp.record("Mod", b"data");
        fp.invalidate("Mod");
        assert!(!fp.has("Mod"));
    }
    #[test]
    fn dirty_set_mark_and_unmark() {
        let mut ds = DirtySet::new();
        ds.mark("A");
        ds.mark("B");
        assert!(ds.is_dirty("A"));
        ds.unmark("A");
        assert!(!ds.is_dirty("A"));
        assert_eq!(ds.len(), 1);
    }
    #[test]
    fn dirty_set_sorted() {
        let mut ds = DirtySet::new();
        ds.mark_all(&["C", "A", "B"]);
        let sorted = ds.sorted();
        assert_eq!(sorted, vec!["A", "B", "C"]);
    }
    #[test]
    fn incremental_build_state_feed_change() {
        let mut state = IncrementalBuildState::new();
        state.feed_change("Mod", b"new content");
        assert!(state.dirty.is_dirty("Mod"));
    }
    #[test]
    fn incremental_build_state_mark_built() {
        let mut state = IncrementalBuildState::new();
        state.feed_change("Mod", b"data");
        state.mark_built("Mod", "build/Mod.o");
        assert!(!state.dirty.is_dirty("Mod"));
        assert!(state.tracker.has_artifacts("Mod"));
    }
    #[test]
    fn dep_graph_builder_propagation() {
        let mut graph = DependencyGraphBuilder::new()
            .add_module("A", 100, 1000, 0xaaa)
            .add_module("B", 200, 2000, 0xbbb)
            .depends("B", "A")
            .build();
        graph.invalidate("A");
        assert!(graph.invalidated.contains("A"));
        assert!(graph.invalidated.contains("B"));
    }
    #[test]
    fn change_detector_changed_modules() {
        let mut old = HashMap::new();
        old.insert("A".to_string(), 1u64);
        old.insert("B".to_string(), 2u64);
        let mut new = HashMap::new();
        new.insert("A".to_string(), 1u64);
        new.insert("B".to_string(), 99u64);
        new.insert("C".to_string(), 3u64);
        let det = ChangeDetector::new(old, new);
        let changed = det.changed_modules();
        assert!(changed.contains(&"B"));
        assert!(changed.contains(&"C"));
        assert!(!changed.contains(&"A"));
    }
    #[test]
    fn change_detector_deleted_modules() {
        let mut old = HashMap::new();
        old.insert("Gone".to_string(), 5u64);
        let new: HashMap<String, u64> = HashMap::new();
        let det = ChangeDetector::new(old, new);
        assert!(det.deleted_modules().contains(&"Gone"));
    }
    #[test]
    fn incremental_stats_hit_rate() {
        let mut s = IncrementalStats::new();
        s.record_hit(100);
        s.record_hit(50);
        s.record_miss();
        assert!((s.hit_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn interface_hash_from_bytes_deterministic() {
        let h1 = InterfaceHash::from_bytes(b"public API");
        let h2 = InterfaceHash::from_bytes(b"public API");
        assert_eq!(h1, h2);
    }
    #[test]
    fn interface_hash_display() {
        let h = InterfaceHash::new(0xdeadbeef);
        let s = format!("{}", h);
        assert!(s.contains("InterfaceHash("));
    }
    #[test]
    fn module_cache_store_and_get() {
        let mut c = ModuleCache::new();
        let h = InterfaceHash::from_bytes(b"iface");
        c.store("Mod", h);
        assert_eq!(c.get("Mod"), Some(h));
        assert!(c.is_current("Mod", h));
    }
    #[test]
    fn module_cache_invalidate() {
        let mut c = ModuleCache::new();
        c.store("Mod", InterfaceHash::new(1));
        c.invalidate("Mod");
        assert!(c.get("Mod").is_none());
    }
    #[test]
    fn scheduler_hint_skip_on_no_change() {
        let mut sched = IncrementalScheduler::new();
        sched.record_built("Mod", b"src", b"iface");
        assert_eq!(sched.hint("Mod", b"src", b"iface"), BuildScheduleHint::Skip);
    }
    #[test]
    fn scheduler_hint_rebuild_on_iface_change() {
        let mut sched = IncrementalScheduler::new();
        sched.record_built("Mod", b"src v1", b"iface v1");
        assert_eq!(
            sched.hint("Mod", b"src v2", b"iface v2"),
            BuildScheduleHint::Rebuild
        );
    }
    #[test]
    fn rebuild_queue_push_pop() {
        let mut q = RebuildQueue::new();
        q.push_all(&["A", "B", "C"]);
        assert_eq!(q.pop(), Some("A".to_string()));
        assert_eq!(q.len(), 2);
    }
    #[test]
    fn rebuild_queue_drain_all() {
        let mut q = RebuildQueue::new();
        q.push_all(&["X", "Y"]);
        let all = q.drain_all();
        assert_eq!(all.len(), 2);
        assert!(q.is_empty());
    }
    #[test]
    fn watch_event_path() {
        let ev = WatchEvent::Modified("src/Mod.lean".to_string());
        assert_eq!(ev.path(), "src/Mod.lean");
        assert!(ev.file_exists_after());
        let del = WatchEvent::Deleted("src/Old.lean".to_string());
        assert!(!del.file_exists_after());
    }
    #[test]
    fn watch_event_renamed_path_is_new() {
        let ev = WatchEvent::Renamed("old.lean".to_string(), "new.lean".to_string());
        assert_eq!(ev.path(), "new.lean");
    }
    #[test]
    fn watch_event_processor_process() {
        let mut map = HashMap::new();
        map.insert("src/Mod.lean".to_string(), "Mathlib.Data.Nat".to_string());
        let proc = WatchEventProcessor::new(map);
        let ev = WatchEvent::Modified("src/Mod.lean".to_string());
        assert_eq!(proc.process(&ev), Some("Mathlib.Data.Nat"));
    }
    #[test]
    fn watch_event_processor_register_unregister() {
        let mut proc = WatchEventProcessor::new(HashMap::new());
        proc.register("a.lean", "ModA");
        assert_eq!(proc.registered_count(), 1);
        proc.unregister("a.lean");
        assert_eq!(proc.registered_count(), 0);
    }
    #[test]
    fn build_report_is_success() {
        let mut r = IncrementalBuildReport::new();
        r.add_rebuilt("A");
        r.add_skipped("B");
        assert!(r.is_success());
        r.add_failed("C");
        assert!(!r.is_success());
    }
    #[test]
    fn build_report_total() {
        let mut r = IncrementalBuildReport::new();
        r.add_rebuilt("A");
        r.add_rebuilt("B");
        r.add_skipped("C");
        assert_eq!(r.total(), 3);
    }
    #[test]
    fn fingerprint_store_put_and_get() {
        let mut store = FingerprintStore::new();
        let fp = FileFingerprint::new("Mod.lean", 512, 999, 0xabc);
        store.put("Mod", fp.clone());
        assert!(store.has("Mod"));
        assert_eq!(
            store.get("Mod").expect("key should exist").content_hash,
            0xabc
        );
    }
    #[test]
    fn fingerprint_store_merge() {
        let mut a = FingerprintStore::new();
        a.put("X", FileFingerprint::new("x", 1, 1, 1));
        let mut b = FingerprintStore::new();
        b.put("Y", FileFingerprint::new("y", 2, 2, 2));
        a.merge(b);
        assert_eq!(a.len(), 2);
    }
    #[test]
    fn incremental_config_defaults() {
        let cfg = IncrementalCompilationConfig::default();
        assert!(cfg.use_interface_hashes);
        assert!(cfg.transitive_propagation);
        assert_eq!(cfg.max_parallel_rebuilds, 4);
    }
    #[test]
    fn incremental_config_ci() {
        let cfg = IncrementalCompilationConfig::ci();
        assert!(cfg.verbose_logging);
        assert!(!cfg.persist_fingerprints);
    }
    #[test]
    fn incremental_config_with_parallelism() {
        let cfg = IncrementalCompilationConfig::default().with_parallelism(8);
        assert_eq!(cfg.max_parallel_rebuilds, 8);
    }
    #[test]
    fn incr_artifact_kind_display() {
        assert_eq!(format!("{}", IncrArtifactKind::Object), "object");
        assert_eq!(format!("{}", IncrArtifactKind::Interface), "interface");
        assert_eq!(format!("{}", IncrArtifactKind::Docs), "docs");
        assert_eq!(format!("{}", IncrArtifactKind::ProofExport), "proof-export");
    }
    #[test]
    fn build_schedule_hint_display() {
        assert_eq!(format!("{}", BuildScheduleHint::Skip), "skip");
        assert_eq!(format!("{}", BuildScheduleHint::Rebuild), "rebuild");
        assert_eq!(format!("{}", BuildScheduleHint::RelinkOnly), "relink-only");
    }
}
#[cfg(test)]
mod engine_tests {
    use super::*;
    #[test]
    fn incr_cache_entry_artifact_count() {
        let fp = FileFingerprint::new("Mod.lean", 512, 1000, 0xabc);
        let iface = InterfaceHash::new(0xdef);
        let entry =
            IncrementalCacheEntry::new(fp, iface, vec!["a.o".to_string(), "b.d".to_string()], 150);
        assert_eq!(entry.artifact_count(), 2);
        assert_eq!(entry.compile_ms, 150);
    }
    #[test]
    fn full_cache_put_and_get() {
        let mut cache = FullIncrementalCache::new();
        let fp = FileFingerprint::new("X.lean", 100, 500, 1);
        let entry = IncrementalCacheEntry::new(fp, InterfaceHash::new(2), vec![], 80);
        cache.put("X", entry);
        assert!(cache.get("X").is_some());
        assert_eq!(cache.len(), 1);
    }
    #[test]
    fn full_cache_total_compile_ms() {
        let mut cache = FullIncrementalCache::new();
        for i in 0u64..5 {
            let fp = FileFingerprint::new("m", 0, 0, i);
            let e = IncrementalCacheEntry::new(fp, InterfaceHash::new(i), vec![], 100 * (i + 1));
            cache.put(&format!("M{}", i), e);
        }
        assert_eq!(cache.total_compile_ms(), 100 + 200 + 300 + 400 + 500);
    }
    #[test]
    fn edge_filter_direct_dependents() {
        let graph = DependencyGraphBuilder::new()
            .add_module("A", 1, 1, 1)
            .add_module("B", 1, 1, 2)
            .add_module("C", 1, 1, 3)
            .depends("B", "A")
            .type_depends("C", "A")
            .build();
        let _ = graph.nodes.len();
        let direct = GraphEdgeKindFilter::direct_dependents(&graph, "A");
        assert!(direct.contains(&"B".to_string()));
        assert!(!direct.contains(&"C".to_string()));
    }
    #[test]
    fn invalidation_log_record_and_query() {
        let mut log = InvalidationLog::new();
        log.record("Mod.A", InvalidationCause::SourceChanged, 100);
        log.record("Mod.A", InvalidationCause::Explicit, 200);
        assert_eq!(log.for_module("Mod.A").len(), 2);
        assert_eq!(
            log.latest_for("Mod.A")
                .expect("test operation should succeed")
                .timestamp,
            200
        );
    }
    #[test]
    fn concurrent_set_mark_and_drain() {
        let mut set = ConcurrentInvalidationSet::new();
        set.mark("A");
        set.mark("B");
        let drained = set.drain();
        assert_eq!(drained.len(), 2);
        assert_eq!(set.count(), 0);
    }
    #[test]
    fn cycle_detector_no_cycle() {
        let graph = DependencyGraphBuilder::new()
            .add_module("A", 1, 1, 1)
            .add_module("B", 1, 1, 2)
            .depends("B", "A")
            .build();
        assert!(CycleDetector::is_acyclic(&graph));
    }
    #[test]
    fn cycle_detector_detects_cycle() {
        let mut graph = IncrementalGraph::new();
        let fp = FileFingerprint::new("x", 1, 1, 1);
        graph.add_module("A", fp.clone());
        graph.add_module("B", fp);
        graph.add_edge("A", "B", EdgeKind::Direct);
        graph.add_edge("B", "A", EdgeKind::Direct);
        assert!(!CycleDetector::is_acyclic(&graph));
    }
    #[test]
    fn build_cycle_display() {
        let cycle = BuildCycle::new(vec!["A".to_string(), "B".to_string(), "A".to_string()]);
        let s = format!("{}", cycle);
        assert!(s.contains("Cycle["));
        assert!(s.contains("A -> B"));
    }
    #[test]
    fn engine_on_file_change_miss_queues() {
        let mut engine = IncrementalEngine::new();
        let hint = engine.on_file_change("Mod", b"source", b"interface");
        assert_eq!(hint, BuildScheduleHint::Rebuild);
        assert!(!engine.queue.is_empty());
    }
    #[test]
    fn engine_on_file_change_skip_when_up_to_date() {
        let mut engine = IncrementalEngine::new();
        engine.on_file_change("Mod", b"src", b"iface");
        engine.on_build_complete("Mod", "build/Mod.o", b"src", b"iface");
        let hint = engine.on_file_change("Mod", b"src", b"iface");
        assert_eq!(hint, BuildScheduleHint::Skip);
    }
    #[test]
    fn engine_is_complete_after_all_built() {
        let mut engine = IncrementalEngine::new();
        engine.on_file_change("Mod", b"src", b"iface");
        engine.on_build_complete("Mod", "build/Mod.o", b"src", b"iface");
        let _ = engine.queue.drain_all();
        assert!(engine.is_complete());
    }
    #[test]
    fn engine_summary_not_empty() {
        let engine = IncrementalEngine::new();
        assert!(!engine.summary().is_empty());
    }
    #[test]
    fn invalidation_cause_display() {
        assert_eq!(
            format!("{}", InvalidationCause::SourceChanged),
            "source-changed"
        );
        assert_eq!(
            format!(
                "{}",
                InvalidationCause::DependencyInvalidated("Dep".to_string())
            ),
            "dependency-invalidated(Dep)"
        );
        assert_eq!(
            format!(
                "{}",
                InvalidationCause::BuildFlagChanged("--opt".to_string())
            ),
            "flag-changed(--opt)"
        );
    }
}
#[cfg(test)]
mod history_tests {
    use super::*;
    #[test]
    fn compile_history_cache_hit_rate() {
        let mut h = CompileHistory::with_capacity(10);
        h.push(ModuleCompileRecord::success("A", true, 100, 0));
        h.push(ModuleCompileRecord::success("B", false, 200, 1));
        h.push(ModuleCompileRecord::success("C", true, 150, 0));
        assert!((h.cache_hit_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn compile_history_evicts_at_capacity() {
        let mut h = CompileHistory::with_capacity(3);
        for i in 0..5 {
            h.push(ModuleCompileRecord::success(
                &format!("M{}", i),
                false,
                10,
                0,
            ));
        }
        assert_eq!(h.len(), 3);
    }
    #[test]
    fn compile_history_avg_elapsed() {
        let mut h = CompileHistory::with_capacity(10);
        h.push(ModuleCompileRecord::success("A", false, 100, 0));
        h.push(ModuleCompileRecord::success("B", false, 200, 0));
        assert!((h.avg_elapsed_ms() - 150.0).abs() < 1e-9);
    }
    #[test]
    fn compile_history_success_rate() {
        let mut h = CompileHistory::with_capacity(10);
        h.push(ModuleCompileRecord::success("A", false, 100, 0));
        h.push(ModuleCompileRecord::failure("B", 50));
        assert!((h.success_rate() - 0.5).abs() < 1e-9);
    }
    #[test]
    fn module_compile_record_fields() {
        let r = ModuleCompileRecord::failure("BadMod", 42);
        assert!(!r.success);
        assert_eq!(r.elapsed_ms, 42);
        assert_eq!(r.module, "BadMod");
    }
}
#[cfg(test)]
mod final_tests {
    use super::*;
    #[test]
    fn incremental_build_metrics_cache_hit_rate() {
        let m = IncrementalBuildMetrics {
            total_modules: 10,
            skipped_modules: 7,
            rebuilt_modules: 3,
            failed_modules: 0,
            wall_ms: 300,
            ..Default::default()
        };
        assert!((m.cache_hit_rate() - 0.7).abs() < 1e-9);
        assert!(m.is_success());
    }
    #[test]
    fn incremental_build_metrics_summary() {
        let m = IncrementalBuildMetrics::new();
        assert!(!m.summary().is_empty());
    }
    #[test]
    fn incremental_build_metrics_estimated_saved() {
        let m = IncrementalBuildMetrics {
            total_modules: 10,
            rebuilt_modules: 5,
            skipped_modules: 5,
            wall_ms: 500,
            ..Default::default()
        };
        let saved = m.estimated_time_saved_ms();
        assert!(saved > 0);
    }
    #[test]
    fn source_root_tracker_module_for_path() {
        let mut tracker = SourceRootTracker::new();
        tracker.register("src", "Mathlib");
        let mod_name = tracker.module_for_path("src/Data/Nat.lean");
        assert!(mod_name.is_some());
        assert!(mod_name
            .expect("test operation should succeed")
            .starts_with("Mathlib"));
    }
    #[test]
    fn source_root_tracker_unknown_path() {
        let tracker = SourceRootTracker::new();
        assert!(tracker.module_for_path("unknown/path.lean").is_none());
    }
}
/// Returns the version string of the incremental build subsystem.
pub fn incr_subsystem_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
#[cfg(test)]
mod version_test {
    use super::*;
    #[test]
    fn incr_subsystem_version_nonempty() {
        assert!(!incr_subsystem_version().is_empty());
    }
}
/// Returns `true` if incremental compilation is supported on this platform.
pub fn incr_supported() -> bool {
    true
}
/// Returns the default parallelism for the incremental engine.
pub fn default_incr_parallelism() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
}
#[cfg(test)]
mod platform_tests {
    use super::*;
    #[test]
    fn incr_supported_is_true() {
        assert!(incr_supported());
    }
    #[test]
    fn default_parallelism_positive() {
        assert!(default_incr_parallelism() >= 1);
    }
}
