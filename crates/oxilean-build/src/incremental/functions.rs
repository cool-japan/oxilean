//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::manifest::Version;
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use super::types::{
    ArtifactCache, ArtifactKind, ArtifactRegistry, BuildArtifact, BuildPhaseTimer, BuildSession,
    CacheStats, ChangeKind, DependencyEdgeKind, DependencyExtractor, DetectedChange, FileEntry,
    FileEvent, FileEventKind, FileEventProcessor, Fingerprint, FingerprintCache,
    IncrementalBuildOrchestrator, IncrementalBuildProgress, IncrementalBuildSchedule,
    IncrementalCacheEvictionPolicy, IncrementalCompileHistory, IncrementalCompiler,
    IncrementalConfig, IncrementalDelta, IncrementalStats, InvalidationReason, InvalidationResult,
    ModuleCompileRecord, ModuleDep, ModuleDepKind, ModuleGraph,
};

/// Detect changes between two sets of file entries.
pub fn detect_changes(
    old: &HashMap<String, FileEntry>,
    new: &HashMap<String, FileEntry>,
) -> Vec<DetectedChange> {
    let mut changes = Vec::new();
    for (module, old_entry) in old {
        match new.get(module) {
            Some(new_entry) => {
                if old_entry.fingerprint != new_entry.fingerprint {
                    changes.push(DetectedChange {
                        module_path: module.clone(),
                        file_path: new_entry.path.clone(),
                        kind: ChangeKind::Modified,
                    });
                }
            }
            None => {
                changes.push(DetectedChange {
                    module_path: module.clone(),
                    file_path: old_entry.path.clone(),
                    kind: ChangeKind::Deleted,
                });
            }
        }
    }
    for (module, new_entry) in new {
        if !old.contains_key(module) {
            changes.push(DetectedChange {
                module_path: module.clone(),
                file_path: new_entry.path.clone(),
                kind: ChangeKind::Added,
            });
        }
    }
    changes
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fingerprint_basic() {
        let fp1 = Fingerprint::from_bytes(b"hello world");
        let fp2 = Fingerprint::from_bytes(b"hello world");
        let fp3 = Fingerprint::from_bytes(b"hello world!");
        assert_eq!(fp1, fp2);
        assert_ne!(fp1, fp3);
    }
    #[test]
    fn test_fingerprint_combine() {
        let fp1 = Fingerprint::new(1, 2);
        let fp2 = Fingerprint::new(3, 4);
        let combined = fp1.combine(&fp2);
        assert!(!combined.is_zero());
    }
    #[test]
    fn test_module_graph_dependents() {
        let mut graph = ModuleGraph::new();
        graph.add_module(FileEntry::new(Path::new("a.lean"), "A"));
        graph.add_module(FileEntry::new(Path::new("b.lean"), "B"));
        graph.add_module(FileEntry::new(Path::new("c.lean"), "C"));
        graph.add_dependency(ModuleDep {
            from: "A".to_string(),
            to: "B".to_string(),
            kind: ModuleDepKind::Import,
        });
        graph.add_dependency(ModuleDep {
            from: "B".to_string(),
            to: "C".to_string(),
            kind: ModuleDepKind::Import,
        });
        let dependents = graph.transitive_dependents("C");
        assert!(dependents.contains("B"));
        assert!(dependents.contains("A"));
    }
    #[test]
    fn test_invalidation_propagation() {
        let config = IncrementalConfig::default();
        let mut compiler = IncrementalCompiler::new(config, Version::new(0, 1, 0));
        compiler.register_module("A", Path::new("a.lean"));
        compiler.register_module("B", Path::new("b.lean"));
        compiler.register_module("C", Path::new("c.lean"));
        compiler.register_dependency("A", "B", ModuleDepKind::Import);
        compiler.register_dependency("B", "C", ModuleDepKind::Import);
        compiler
            .update_module_fingerprint("A", b"module A v1")
            .expect("test operation should succeed");
        compiler
            .update_module_fingerprint("B", b"module B v1")
            .expect("test operation should succeed");
        compiler
            .update_module_fingerprint("C", b"module C v1")
            .expect("test operation should succeed");
        let result = compiler.compute_invalidation();
        assert_eq!(result.invalidated_count(), 3);
        compiler.commit_fingerprints();
        compiler
            .update_module_fingerprint("C", b"module C v2")
            .expect("test operation should succeed");
        let result = compiler.compute_invalidation();
        assert!(result.invalidated.contains("C"));
        assert!(result.invalidated.contains("B"));
        assert!(result.invalidated.contains("A"));
    }
    #[test]
    fn test_artifact_cache() {
        let mut cache = ArtifactCache::new(Path::new("/tmp/cache"));
        let fp = Fingerprint::from_bytes(b"test");
        let artifact = BuildArtifact {
            kind: ArtifactKind::Object,
            path: PathBuf::from("/tmp/cache/test.o"),
            source_fingerprint: fp,
            artifact_fingerprint: Fingerprint::zero(),
            dep_fingerprints: BTreeMap::new(),
            compiler_version: "0.1.1".to_string(),
            build_time: None,
            build_flags: Vec::new(),
        };
        cache.store("test", artifact);
        let found = cache.lookup(
            "test",
            &ArtifactKind::Object,
            &fp,
            &BTreeMap::new(),
            "0.1.1",
        );
        assert!(found.is_some());
    }
    #[test]
    fn test_change_detection() {
        let mut old = HashMap::new();
        old.insert(
            "A".to_string(),
            FileEntry {
                path: PathBuf::from("a.lean"),
                fingerprint: Fingerprint::new(1, 1),
                size: 100,
                modified: None,
                module_path: "A".to_string(),
            },
        );
        old.insert(
            "B".to_string(),
            FileEntry {
                path: PathBuf::from("b.lean"),
                fingerprint: Fingerprint::new(2, 2),
                size: 200,
                modified: None,
                module_path: "B".to_string(),
            },
        );
        let mut new_map = HashMap::new();
        new_map.insert(
            "A".to_string(),
            FileEntry {
                path: PathBuf::from("a.lean"),
                fingerprint: Fingerprint::new(1, 1),
                size: 100,
                modified: None,
                module_path: "A".to_string(),
            },
        );
        new_map.insert(
            "C".to_string(),
            FileEntry {
                path: PathBuf::from("c.lean"),
                fingerprint: Fingerprint::new(3, 3),
                size: 300,
                modified: None,
                module_path: "C".to_string(),
            },
        );
        let changes = detect_changes(&old, &new_map);
        assert_eq!(changes.len(), 2);
    }
    #[test]
    fn test_build_session() {
        let order = vec!["C".to_string(), "B".to_string(), "A".to_string()];
        let mut session = BuildSession::new(order);
        assert_eq!(session.next_module(), Some("C"));
        assert!(!session.is_complete());
        session.start_module("C");
        session.complete_module("C");
        assert_eq!(session.next_module(), Some("B"));
        session.start_module("B");
        session.complete_module("B");
        session.start_module("A");
        session.complete_module("A");
        assert!(session.is_complete());
        assert!(!session.has_failures());
    }
    #[test]
    fn test_build_session_failure() {
        let order = vec!["A".to_string(), "B".to_string()];
        let mut session = BuildSession::new(order);
        session.start_module("A");
        session.fail_module("A", "type error");
        assert!(session.has_failures());
        let failures = session.failures();
        assert_eq!(failures.len(), 1);
        assert_eq!(failures[0].0, "A");
        assert_eq!(failures[0].1, "type error");
    }
    #[test]
    fn test_build_session_cache_hit() {
        let order = vec!["A".to_string(), "B".to_string()];
        let mut session = BuildSession::new(order);
        session.cache_hit("A");
        session.start_module("B");
        session.complete_module("B");
        let counts = session.count_by_state();
        assert_eq!(*counts.get("cached").unwrap_or(&0), 1);
        assert_eq!(*counts.get("done").unwrap_or(&0), 1);
    }
    #[test]
    fn test_file_event_processor() {
        let mut processor = FileEventProcessor::new();
        processor.watch_dir(Path::new("/project/src"));
        processor.queue_event(FileEvent {
            path: PathBuf::from("src/main.lean"),
            kind: FileEventKind::Modified,
            timestamp: None,
        });
        processor.queue_event(FileEvent {
            path: PathBuf::from("src/readme.txt"),
            kind: FileEventKind::Modified,
            timestamp: None,
        });
        assert_eq!(processor.pending_count(), 1);
        let events = processor.drain_events();
        assert_eq!(events.len(), 1);
        assert_eq!(processor.pending_count(), 0);
    }
    #[test]
    fn test_dependency_extractor() {
        let extractor = DependencyExtractor::new();
        let source = r#"
import Mathlib.Topology.Basic
import Mathlib.Algebra.Ring
open Nat
-- import ShouldBeIgnored
"#;
        let deps = extractor.extract(source);
        assert_eq!(deps.len(), 3);
        assert_eq!(deps[0].0, "Mathlib.Topology.Basic");
        assert_eq!(deps[0].1, ModuleDepKind::Import);
        assert_eq!(deps[2].0, "Nat");
        assert_eq!(deps[2].1, ModuleDepKind::Open);
    }
    #[test]
    fn test_fingerprint_cache_serialize() {
        let mut cache = FingerprintCache::new();
        cache.set(
            "A",
            Fingerprint::new(0x1234567890abcdef, 0xfedcba0987654321),
        );
        cache.set(
            "B",
            Fingerprint::new(0x1111111111111111, 0x2222222222222222),
        );
        let serialized = cache.serialize();
        assert!(serialized.contains("A"));
        assert!(serialized.contains("B"));
        let deserialized = FingerprintCache::deserialize(&serialized);
        assert_eq!(deserialized.count(), 2);
        assert_eq!(
            deserialized.get("A"),
            Some(Fingerprint::new(0x1234567890abcdef, 0xfedcba0987654321))
        );
    }
    #[test]
    fn test_fingerprint_display() {
        let fp = Fingerprint::new(0x00000000000000ff, 0x0000000000000001);
        let display = format!("{}", fp);
        assert_eq!(display.len(), 32);
    }
    #[test]
    fn test_invalidation_result_rebuild_fraction() {
        let mut result = InvalidationResult::new();
        result.invalidate(
            "A",
            InvalidationReason::SourceChanged {
                module: "A".to_string(),
                old_fp: Fingerprint::zero(),
                new_fp: Fingerprint::new(1, 1),
            },
        );
        result.mark_valid("B");
        result.mark_valid("C");
        result.mark_valid("D");
        let fraction = result.rebuild_fraction();
        assert!((fraction - 0.25).abs() < 0.01);
    }
    #[test]
    fn test_module_graph_counts() {
        let mut graph = ModuleGraph::new();
        graph.add_module(FileEntry::new(Path::new("a.lean"), "A"));
        graph.add_module(FileEntry::new(Path::new("b.lean"), "B"));
        graph.add_dependency(ModuleDep {
            from: "A".to_string(),
            to: "B".to_string(),
            kind: ModuleDepKind::Import,
        });
        assert_eq!(graph.module_count(), 2);
        assert_eq!(graph.dependency_count(), 1);
    }
    #[test]
    fn test_artifact_validity() {
        let fp = Fingerprint::from_bytes(b"hello");
        let artifact = BuildArtifact {
            kind: ArtifactKind::Object,
            path: PathBuf::from("/tmp/test.o"),
            source_fingerprint: fp,
            artifact_fingerprint: Fingerprint::zero(),
            dep_fingerprints: BTreeMap::new(),
            compiler_version: "0.1.1".to_string(),
            build_time: None,
            build_flags: Vec::new(),
        };
        assert!(artifact.is_valid(&fp, &BTreeMap::new(), "0.1.1"));
        let new_fp = Fingerprint::from_bytes(b"world");
        assert!(!artifact.is_valid(&new_fp, &BTreeMap::new(), "0.1.1"));
        assert!(!artifact.is_valid(&fp, &BTreeMap::new(), "0.2.0"));
    }
    #[test]
    fn test_cache_stats() {
        let stats = CacheStats {
            hits: 8,
            misses: 2,
            bytes_served: 0,
            total_artifacts: 10,
            total_bytes: 0,
        };
        let rate = stats.hit_rate();
        assert!((rate - 0.8).abs() < 0.01);
    }
    #[test]
    fn test_incremental_compiler_reset() {
        let config = IncrementalConfig::default();
        let mut compiler = IncrementalCompiler::new(config, Version::new(0, 1, 0));
        compiler.register_module("A", Path::new("a.lean"));
        compiler
            .update_module_fingerprint("A", b"contents")
            .expect("test operation should succeed");
        compiler.commit_fingerprints();
        compiler.reset();
        let result = compiler.compute_invalidation();
        assert!(result.invalidated.contains("A"));
    }
    #[test]
    fn test_incremental_stats() {
        let stats = IncrementalStats {
            total_modules: 100,
            rebuilt_modules: 10,
            cached_modules: 90,
            estimated_time_saved: Duration::from_secs(45),
            total_time: Duration::from_secs(5),
            cache_hit_rate: 0.9,
            invalidation_chains: 3,
            max_chain_length: 4,
        };
        let summary = stats.summary();
        assert!(summary.contains("10/100"));
        assert!(summary.contains("90%"));
    }
}
#[cfg(test)]
mod incr_extra_tests {
    use super::*;
    #[test]
    fn module_compile_record_success_fields() {
        let r = ModuleCompileRecord::success("Mod", true, 200, 3);
        assert!(r.success);
        assert!(r.from_cache);
        assert_eq!(r.elapsed_ms, 200);
        assert_eq!(r.warning_count, 3);
    }
    #[test]
    fn module_compile_record_failure_fields() {
        let r = ModuleCompileRecord::failure("BadMod", 50);
        assert!(!r.success);
        assert_eq!(r.module, "BadMod");
    }
    #[test]
    fn compile_history_hit_rate() {
        let mut h = IncrementalCompileHistory::with_capacity(10);
        h.push(ModuleCompileRecord::success("A", true, 0, 0));
        h.push(ModuleCompileRecord::success("B", false, 100, 0));
        h.push(ModuleCompileRecord::success("C", true, 0, 0));
        assert!((h.hit_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn compile_history_evicts_at_capacity() {
        let mut h = IncrementalCompileHistory::with_capacity(3);
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
    fn artifact_registry_register_and_get() {
        let mut reg = ArtifactRegistry::new();
        reg.register("Mod.A", PathBuf::from("build/A.o"));
        reg.register("Mod.A", PathBuf::from("build/A.d"));
        assert_eq!(reg.get("Mod.A").len(), 2);
        assert_eq!(reg.total_artifacts(), 2);
    }
    #[test]
    fn artifact_registry_remove() {
        let mut reg = ArtifactRegistry::new();
        reg.register("X", PathBuf::from("build/X.o"));
        reg.remove("X");
        assert!(reg.get("X").is_empty());
    }
    #[test]
    fn incremental_delta_total_affected() {
        let mut delta = IncrementalDelta::new();
        delta.source_changed.push("A".to_string());
        delta.transitive.push("B".to_string());
        delta.transitive.push("C".to_string());
        assert_eq!(delta.total_affected(), 3);
        assert!(delta.has_changes());
    }
    #[test]
    fn incremental_schedule_layer_count() {
        let mut sched = IncrementalBuildSchedule::new();
        sched.add_layer(vec!["A".to_string(), "B".to_string()]);
        sched.add_layer(vec!["C".to_string()]);
        sched.mark_skip("D");
        assert_eq!(sched.layer_count(), 2);
        assert_eq!(sched.build_count(), 3);
        assert_eq!(sched.skip_count(), 1);
    }
    #[test]
    fn dependency_edge_kind_display() {
        assert_eq!(format!("{}", DependencyEdgeKind::Full), "full");
        assert_eq!(format!("{}", DependencyEdgeKind::TypeOnly), "type-only");
        assert_eq!(format!("{}", DependencyEdgeKind::Weak), "weak");
    }
    #[test]
    fn orchestrator_record_success_and_artifacts() {
        let config = IncrementalConfig::default();
        let ver = Version::new(0, 1, 0);
        let mut orch = IncrementalBuildOrchestrator::new(config, ver);
        orch.register("Mod", Path::new("Mod.lean"));
        orch.record_success("Mod", PathBuf::from("build/Mod.o"), 150);
        assert_eq!(orch.artifacts.get("Mod").len(), 1);
        assert_eq!(orch.history.len(), 1);
    }
    #[test]
    fn orchestrator_record_cached() {
        let config = IncrementalConfig::default();
        let ver = Version::new(0, 1, 0);
        let mut orch = IncrementalBuildOrchestrator::new(config, ver);
        orch.record_cached("Mod");
        assert!((orch.history.hit_rate() - 1.0).abs() < 1e-9);
    }
}
#[cfg(test)]
mod progress_tests {
    use super::*;
    #[test]
    fn incremental_cache_eviction_display() {
        assert_eq!(format!("{}", IncrementalCacheEvictionPolicy::Lru), "lru");
        assert_eq!(format!("{}", IncrementalCacheEvictionPolicy::Fifo), "fifo");
    }
    #[test]
    fn build_progress_fraction() {
        let mut p = IncrementalBuildProgress::new(10);
        p.record_done();
        p.record_done();
        assert!((p.fraction() - 0.2).abs() < 1e-9);
    }
    #[test]
    fn build_progress_is_done() {
        let mut p = IncrementalBuildProgress::new(3);
        p.record_done();
        p.record_done();
        p.record_failure();
        assert!(p.is_done());
    }
    #[test]
    fn build_progress_display() {
        let p = IncrementalBuildProgress::new(5);
        let s = format!("{}", p);
        assert!(s.contains("/5"));
    }
}
/// Returns the incremental build subsystem version.
pub fn incremental_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
#[cfg(test)]
mod incr_version_test {
    use super::*;
    #[test]
    fn incr_version_nonempty() {
        assert!(!incremental_version().is_empty());
    }
}
#[cfg(test)]
mod phase_timer_tests {
    use super::*;
    #[test]
    fn phase_timer_total() {
        let t = BuildPhaseTimer {
            parse_ms: 100,
            typecheck_ms: 200,
            codegen_ms: 50,
            link_ms: 25,
        };
        assert_eq!(t.total_ms(), 375);
    }
    #[test]
    fn phase_timer_display() {
        let t = BuildPhaseTimer::new();
        assert!(!format!("{}", t).is_empty());
    }
    #[test]
    fn phase_timer_parse_fraction_zero() {
        let t = BuildPhaseTimer::new();
        assert!((t.parse_fraction() - 0.0).abs() < 1e-9);
    }
}
/// Returns true when incremental compilation is fully supported on this platform.
pub fn incremental_fully_supported() -> bool {
    true
}
/// Returns the maximum supported fingerprint cache size (number of entries).
pub fn max_fingerprint_cache_size() -> usize {
    65536
}
