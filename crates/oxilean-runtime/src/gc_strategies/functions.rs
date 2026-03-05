//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    BarrierAction, CardTable, CompactionStrategy, FinalizerQueue, FreeList, GcBenchmark,
    GcBenchmarkResult, GcConfig, GcCycleRecord, GcHandle, GcHistory, GcObjectHeader, GcPauseLog,
    GcPhase, GcRegion, GcRootSet, GcSafePoint, GcStats, GcStrategy, GcTuner, GenerationalGc,
    HeapFragmentation, IncrementalGc, MarkSweepGc, ObjectAgeTable, RegionBasedGc, SemispaceGc,
    StickyMarkBitsGc, TriColor, WriteBarrier,
};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gc_strategy_name() {
        assert_eq!(GcStrategy::MarkSweep.name(), "MarkSweep");
        assert_eq!(GcStrategy::Semispace.name(), "Semispace");
    }
    #[test]
    fn test_gc_strategy_concurrent() {
        assert!(GcStrategy::Incremental.is_concurrent());
        assert!(!GcStrategy::MarkSweep.is_concurrent());
        assert!(!GcStrategy::Generational.is_concurrent());
    }
    #[test]
    fn test_gc_stats_record() {
        let mut stats = GcStats::new();
        stats.record_collection(1024, 5000);
        assert_eq!(stats.collections, 1);
        assert_eq!(stats.bytes_collected, 1024);
        assert_eq!(stats.pause_time_ns, 5000);
    }
    #[test]
    fn test_gc_stats_throughput() {
        let mut stats = GcStats::new();
        stats.record_collection(0, 1000);
        let tp = stats.throughput_pct(10000);
        assert!((tp - 90.0).abs() < 1e-6, "expected 90%, got {tp}");
    }
    #[test]
    fn test_mark_sweep_allocate_and_sweep() {
        let mut gc = MarkSweepGc::new(1024);
        let off = gc.allocate(128).expect("allocation should succeed");
        assert_eq!(off, 0);
        let freed = gc.sweep();
        assert_eq!(freed, 128);
    }
    #[test]
    fn test_mark_sweep_mark_survives() {
        let mut gc = MarkSweepGc::new(256);
        gc.allocate(10).expect("allocation should succeed");
        gc.mark(0);
        let freed = gc.sweep();
        assert_eq!(freed, 9);
    }
    #[test]
    fn test_semispace_allocate_and_flip() {
        let mut gc = SemispaceGc::new(128);
        let off = gc.allocate(64).expect("allocation should succeed");
        assert_eq!(off, 0);
        let surviving = gc.flip();
        assert_eq!(surviving, 64);
    }
    #[test]
    fn test_generational_gc_report() {
        let mut gc = GenerationalGc::new();
        gc.young.allocate(32).expect("allocation should succeed");
        gc.minor_gc();
        let report = gc.stats_report();
        assert!(report.contains("Young:"));
        assert!(report.contains("Old:"));
    }
}
#[cfg(test)]
mod extended_tests {
    use super::*;
    #[test]
    fn test_incremental_gc_basic() {
        let mut gc = IncrementalGc::new();
        let a = gc.allocate(10);
        let b = gc.allocate(20);
        gc.add_root(a);
        gc.cells[a].add_child(b);
        gc.begin_collection();
        gc.mark_all();
        let freed = gc.sweep();
        assert_eq!(freed, 0);
        assert_eq!(gc.live_count(), 2);
    }
    #[test]
    fn test_incremental_gc_unreachable() {
        let mut gc = IncrementalGc::new();
        let a = gc.allocate(10);
        let _b = gc.allocate(20);
        gc.add_root(a);
        let freed = gc.collect();
        assert_eq!(freed, 20);
        assert_eq!(gc.live_count(), 1);
    }
    #[test]
    fn test_incremental_gc_write_barrier() {
        let mut gc = IncrementalGc::new();
        let a = gc.allocate(10);
        let b = gc.allocate(20);
        gc.add_root(a);
        gc.begin_collection();
        gc.cells[a].color = TriColor::Black;
        gc.write_barrier(a, b);
        assert_eq!(gc.cells[a].color, TriColor::Gray);
    }
    #[test]
    fn test_gc_config_defaults() {
        let cfg = GcConfig::default();
        assert_eq!(cfg.strategy, GcStrategy::MarkSweep);
        let issues = cfg.validate();
        assert!(issues.is_empty());
    }
    #[test]
    fn test_gc_config_builder() {
        let cfg = GcConfig::new(GcStrategy::Incremental)
            .with_heap_limit(1024 * 1024)
            .with_threshold(0.8)
            .with_incremental_steps(50)
            .with_write_barriers(true);
        assert_eq!(cfg.strategy, GcStrategy::Incremental);
        assert_eq!(cfg.heap_limit, 1024 * 1024);
        assert!((cfg.collection_threshold - 0.8).abs() < 1e-9);
        assert_eq!(cfg.incremental_steps, 50);
    }
    #[test]
    fn test_gc_handle_from_config() {
        let cfg = GcConfig::new(GcStrategy::MarkSweep).with_heap_limit(4096);
        let handle = GcHandle::from_config(cfg);
        assert_eq!(handle.strategy_name(), "MarkSweep");
        assert!(!handle.should_collect());
    }
    #[test]
    fn test_write_barrier_log() {
        let mut barrier = WriteBarrier::new();
        barrier.activate();
        barrier.record(0, 1, BarrierAction::GraySource);
        barrier.record(2, 3, BarrierAction::GrayDest);
        assert_eq!(barrier.pending_count(), 2);
        let actions = barrier.drain();
        assert_eq!(actions.len(), 2);
        assert_eq!(barrier.pending_count(), 0);
    }
    #[test]
    fn test_write_barrier_inactive() {
        let mut barrier = WriteBarrier::new();
        barrier.record(0, 1, BarrierAction::GraySource);
        assert_eq!(barrier.pending_count(), 0);
    }
    #[test]
    fn test_gc_root_set() {
        let mut roots = GcRootSet::new();
        roots.add(10);
        roots.add(20);
        roots.add_named("global_env", 30);
        assert_eq!(roots.len(), 3);
        assert_eq!(roots.lookup_named("global_env"), Some(30));
        roots.remove(10);
        assert_eq!(roots.len(), 2);
        assert!(!roots.all_roots().contains(&10));
    }
    #[test]
    fn test_gc_pause_log() {
        let mut log = GcPauseLog::new();
        log.record(0, 1000);
        log.record(5000, 2000);
        log.record(10000, 500);
        assert_eq!(log.count(), 3);
        assert_eq!(log.total_pause_ns(), 3500);
        assert_eq!(log.max_pause_ns(), 2000);
        assert_eq!(log.p99_pause_ns(), 2000);
    }
    #[test]
    fn test_compaction_strategy() {
        assert!(CompactionStrategy::Always.is_enabled());
        assert!(!CompactionStrategy::Never.is_enabled());
        assert_eq!(CompactionStrategy::MajorOnly.name(), "major-only");
    }
    #[test]
    fn test_heap_fragmentation() {
        let frag = HeapFragmentation {
            total_capacity: 1000,
            live_bytes: 400,
            largest_free_block: 200,
            free_block_count: 3,
        };
        assert!(frag.ratio() > 0.5);
        assert!(frag.is_high());
        assert!((frag.utilization() - 0.4).abs() < 1e-9);
    }
    #[test]
    fn test_gc_tuner_adjusts_threshold() {
        let mut tuner = GcTuner::new();
        tuner.target_pause_ns = 1_000_000;
        for _ in 0..10 {
            tuner.record_pause(3_000_000);
        }
        assert!(tuner.collection_threshold < 0.75);
    }
    #[test]
    fn test_gc_tuner_short_pauses() {
        let mut tuner = GcTuner::new();
        tuner.target_pause_ns = 1_000_000;
        for _ in 0..10 {
            tuner.record_pause(100_000);
        }
        assert!(tuner.collection_threshold > 0.75);
    }
    #[test]
    fn test_gc_object_header() {
        let mut hdr = GcObjectHeader::new(42, 128);
        assert_eq!(hdr.ref_count, 1);
        hdr.inc_ref();
        assert_eq!(hdr.ref_count, 2);
        let dead = hdr.dec_ref();
        assert!(!dead);
        let dead = hdr.dec_ref();
        assert!(dead);
        hdr.mark();
        assert!(hdr.marked);
        hdr.clear_mark();
        assert!(!hdr.marked);
        hdr.promote();
        assert_eq!(hdr.generation, 1);
        hdr.forward_to(0xFF00);
        assert!(hdr.is_forwarded());
        assert_eq!(hdr.forwarding, Some(0xFF00));
    }
    #[test]
    fn test_free_list_basic() {
        let mut fl = FreeList::new(1024);
        let a = fl.allocate(128).expect("alloc a");
        let b = fl.allocate(256).expect("alloc b");
        assert_ne!(a, b);
        assert_eq!(fl.allocated, 384);
        fl.free(a, 128);
        assert_eq!(fl.allocated, 256);
        let c = fl.allocate(64).expect("re-alloc");
        assert_eq!(c, 0);
    }
    #[test]
    fn test_free_list_coalesce() {
        let mut fl = FreeList::new(300);
        let _a = fl.allocate(100).expect("allocation should succeed");
        let b = fl.allocate(100).expect("allocation should succeed");
        let _c = fl.allocate(100).expect("allocation should succeed");
        fl.free(b, 100);
        fl.free(0, 100);
        fl.coalesce();
        let lf = fl.largest_free();
        assert!(lf >= 200);
    }
    #[test]
    fn test_card_table_basic() {
        let mut ct = CardTable::new(4096, 64);
        ct.mark_dirty(100);
        ct.mark_dirty(200);
        assert!(ct.is_dirty(100));
        assert!(ct.is_dirty(200));
        assert!(!ct.is_dirty(400));
        assert_eq!(ct.dirty_count(), 2);
        ct.clear();
        assert_eq!(ct.dirty_count(), 0);
    }
    #[test]
    fn test_card_table_range() {
        let ct = CardTable::new(1024, 256);
        let (start, end) = ct.card_range(1);
        assert_eq!(start, 256);
        assert_eq!(end, 512);
    }
    #[test]
    fn test_finalizer_queue() {
        let mut fq = FinalizerQueue::new();
        fq.enqueue(10);
        fq.enqueue(20);
        fq.enqueue(30);
        assert_eq!(fq.pending_count(), 3);
        assert!(fq.has_pending());
        let id = fq.drain_one().expect("test operation should succeed");
        assert_eq!(id, 10);
        assert_eq!(fq.finalized_count, 1);
        let drained = fq.drain_all();
        assert_eq!(drained, 2);
        assert!(!fq.has_pending());
        assert_eq!(fq.finalized_count, 3);
    }
    #[test]
    fn test_gc_phase_stop_the_world() {
        assert!(GcPhase::InitialMark.is_stop_the_world());
        assert!(GcPhase::Remark.is_stop_the_world());
        assert!(!GcPhase::ConcurrentMark.is_stop_the_world());
        assert!(!GcPhase::Sweep.is_stop_the_world());
        assert_eq!(GcPhase::Sweep.name(), "sweep");
    }
    #[test]
    fn test_gc_cycle_record() {
        let mut rec = GcCycleRecord::new(1, "MarkSweep", 1000);
        rec.finalize(300, 5_000_000);
        assert_eq!(rec.bytes_freed, 700);
        assert!((rec.survival_rate() - 0.3).abs() < 1e-9);
        assert!((rec.reclaim_rate() - 0.7).abs() < 1e-9);
    }
    #[test]
    fn test_gc_history_basic() {
        let mut history = GcHistory::with_capacity(3);
        for i in 0..5 {
            let mut rec = GcCycleRecord::new(i, "Test", 100);
            rec.finalize(50, 1000);
            history.record(rec);
        }
        assert_eq!(history.cycles.len(), 3);
        assert_eq!(history.total_freed(), 150);
    }
    #[test]
    fn test_gc_history_stats() {
        let mut history = GcHistory::new();
        for i in 0..4 {
            let mut rec = GcCycleRecord::new(i, "Test", 1000);
            rec.is_major = i % 2 == 0;
            rec.finalize(500, 2000);
            history.record(rec);
        }
        assert_eq!(history.major_count(), 2);
        assert_eq!(history.minor_count(), 2);
        assert!((history.avg_survival_rate() - 0.5).abs() < 1e-9);
    }
    #[test]
    fn test_gc_benchmark_mark_sweep() {
        let mut bench = GcBenchmark::new();
        bench.run_mark_sweep(50, 16);
        assert!(bench.results.contains_key("MarkSweep"));
        let result = &bench.results["MarkSweep"];
        assert_eq!(result.total_allocated, 50 * 16);
    }
    #[test]
    fn test_gc_benchmark_comparison() {
        let mut bench = GcBenchmark::new();
        bench.run_mark_sweep(10, 8);
        bench.run_semispace(10, 8);
        let report = bench.print_comparison();
        assert!(report.contains("GC Benchmark"));
    }
    #[test]
    fn test_tri_color_enum() {
        let c = TriColor::Gray;
        assert_eq!(c, TriColor::Gray);
        assert_ne!(c, TriColor::White);
    }
    #[test]
    fn test_gc_strategy_description() {
        assert!(GcStrategy::Generational
            .description()
            .contains("Generational"));
        assert!(GcStrategy::Semispace.description().contains("Copying"));
    }
}
#[cfg(test)]
mod extra_tests {
    use super::*;
    #[test]
    fn test_sticky_mark_bits_gc() {
        let mut gc = StickyMarkBitsGc::new(1024);
        gc.allocate(100).expect("allocation should succeed");
        gc.allocate(200).expect("allocation should succeed");
        gc.mark_sticky(0);
        let freed = gc.collect();
        assert!(freed <= 300);
    }
    #[test]
    fn test_sticky_mark_bits_reset() {
        let mut gc = StickyMarkBitsGc::new(256);
        gc.allocate(50).expect("allocation should succeed");
        gc.mark_sticky(0);
        gc.mark_sticky(10);
        assert!(gc.sticky_bytes() >= 2);
        gc.reset_bits();
        assert_eq!(gc.sticky_bytes(), 0);
    }
    #[test]
    fn test_region_based_gc_basic() {
        let mut gc = RegionBasedGc::new(4, 1024);
        let (r, _off) = gc.allocate(512).expect("alloc in region");
        assert_eq!(r, 0);
        assert_eq!(gc.total_used(), 512);
        assert_eq!(gc.total_free(), 4 * 1024 - 512);
    }
    #[test]
    fn test_region_based_gc_cset() {
        let mut gc = RegionBasedGc::new(4, 256);
        gc.allocate(128).expect("allocation should succeed");
        gc.regions[0].set_liveness(10);
        gc.select_cset(1);
        assert!(gc.regions[0].in_cset);
        let freed = gc.collect_cset();
        assert!(freed > 0);
    }
    #[test]
    fn test_region_based_gc_empty_count() {
        let gc = RegionBasedGc::new(8, 512);
        assert_eq!(gc.empty_region_count(), 8);
        assert_eq!(gc.full_region_count(), 0);
    }
    #[test]
    fn test_object_age_table() {
        let mut table = ObjectAgeTable::new(3);
        table.register(1);
        table.register(2);
        assert!(!table.age_object(1));
        assert!(!table.age_object(1));
        let promoted = table.age_object(1);
        assert!(promoted);
        assert_eq!(table.promoted_count, 1);
    }
    #[test]
    fn test_object_age_table_avg() {
        let mut table = ObjectAgeTable::new(10);
        table.register(10);
        table.register(20);
        table.age_object(10);
        table.age_object(10);
        assert!((table.avg_age() - 1.0).abs() < 1e-9);
    }
    #[test]
    fn test_object_age_old_objects() {
        let mut table = ObjectAgeTable::new(2);
        table.register(99);
        table.age_object(99);
        table.age_object(99);
        let old = table.old_objects();
        assert!(old.contains(&99));
    }
    #[test]
    fn test_gc_safepoint_stop_release() {
        let mut sp = GcSafePoint::new(3);
        sp.request_stop();
        sp.thread_at_safepoint();
        sp.thread_at_safepoint();
        assert!(!sp.all_stopped());
        sp.thread_at_safepoint();
        assert!(sp.all_stopped());
        sp.release();
        assert!(!sp.stop_requested);
        assert_eq!(sp.threads_at_safepoint, 0);
    }
    #[test]
    fn test_gc_safepoint_fraction() {
        let mut sp = GcSafePoint::new(4);
        sp.thread_at_safepoint();
        sp.thread_at_safepoint();
        assert!((sp.stopped_fraction() - 0.5).abs() < 1e-9);
    }
    #[test]
    fn test_gc_history_last_n() {
        let mut history = GcHistory::new();
        for i in 0..10u64 {
            let mut rec = GcCycleRecord::new(i, "Test", 100);
            rec.finalize(50, 100);
            history.record(rec);
        }
        let last3 = history.last_n(3);
        assert_eq!(last3.len(), 3);
        assert_eq!(last3[0].cycle_id, 7);
    }
    #[test]
    fn test_free_list_utilization() {
        let mut fl = FreeList::new(1000);
        fl.allocate(400).expect("allocation should succeed");
        fl.allocate(200).expect("allocation should succeed");
        assert!((fl.utilization() - 0.6).abs() < 1e-9);
    }
    #[test]
    fn test_gc_benchmark_result_overhead() {
        let result = GcBenchmarkResult {
            strategy: "Test".to_string(),
            total_allocated: 1000,
            total_freed: 800,
            collections: 5,
            alloc_time_ns: 8000,
            gc_time_ns: 2000,
        };
        assert!((result.gc_overhead() - 0.2).abs() < 1e-9);
    }
    #[test]
    fn test_gc_strategy_all_variants() {
        let strategies = [
            GcStrategy::RefCounting,
            GcStrategy::MarkSweep,
            GcStrategy::Semispace,
            GcStrategy::Generational,
            GcStrategy::Incremental,
        ];
        for s in &strategies {
            assert!(!s.name().is_empty());
            assert!(!s.description().is_empty());
        }
    }
    #[test]
    fn test_gc_stats_throughput_zero_total() {
        let stats = GcStats::new();
        assert!((stats.throughput_pct(0) - 100.0).abs() < 1e-9);
    }
    #[test]
    fn test_incremental_gc_step_mark() {
        let mut gc = IncrementalGc::new();
        let a = gc.allocate(10);
        let b = gc.allocate(10);
        gc.cells[a].add_child(b);
        gc.add_root(a);
        gc.begin_collection();
        let mut steps = 0;
        loop {
            steps += 1;
            if gc.step_mark() {
                break;
            }
            assert!(steps < 100, "mark did not terminate");
        }
        assert_eq!(gc.cells[a].color, TriColor::Black);
        assert_eq!(gc.cells[b].color, TriColor::Black);
    }
    #[test]
    fn test_region_humongous_flag() {
        let mut region = GcRegion::new(0, 4096);
        region.humongous = true;
        assert!(region.humongous);
        assert_eq!(region.free(), 4096);
    }
    #[test]
    fn test_gc_pause_log_empty() {
        let log = GcPauseLog::new();
        assert_eq!(log.count(), 0);
        assert_eq!(log.total_pause_ns(), 0);
        assert_eq!(log.max_pause_ns(), 0);
        assert_eq!(log.p99_pause_ns(), 0);
        assert!((log.avg_pause_ns() - 0.0).abs() < 1e-9);
    }
    #[test]
    fn test_barrier_action_variants() {
        let actions = [
            BarrierAction::None,
            BarrierAction::GraySource,
            BarrierAction::GrayDest,
            BarrierAction::SnapshotOld,
        ];
        for a in &actions {
            let b = *a;
            assert_eq!(a, &b);
        }
    }
}
