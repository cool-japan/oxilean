//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    AllocationProfile, AllocationSiteProfile, AutoFdoConfig, BoltInstrumentationConfig,
    BranchPredictability, CallGraph, ContextSensitiveProfile, CoverageReport, DevirtualizationPass,
    FunctionProfile, GlobalInstrumentationRegistry, HotColdSplit, InlineHeuristic, InlineHint,
    InstrumentationCounters, InstrumentationPass, LoopIterationProfile, MemoryAccessPattern,
    MergeWeightMode, OptAction, PgoAnnotatedFunction, PgoConfig, PgoDataFormat, PgoDecision,
    PgoOptimizationLog, PgoPass, PgoStatisticsReport, PgoWorkflow, ProfileData, ProfileMerger,
    ProfileSummary, PropellerEdge, PropellerFunctionInfo, PropellerProfile, RawProfileData,
    SampleBasedProfileGenerator, SampleRecord, ThinLtoPgoData, VirtualCallRecord,
    WholeProgramDevirt,
};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub(super) fn test_record_call_increments_count() {
        let mut data = ProfileData::new();
        data.record_call("foo");
        data.record_call("foo");
        data.record_call("bar");
        assert_eq!(data.call_counts["foo"], 2);
        assert_eq!(data.call_counts["bar"], 1);
    }
    #[test]
    pub(super) fn test_record_edge_increments_weight() {
        let mut data = ProfileData::new();
        data.record_edge("main", "foo");
        data.record_edge("main", "foo");
        data.record_edge("foo", "bar");
        assert_eq!(data.edge_counts[&("main".to_owned(), "foo".to_owned())], 2);
        assert_eq!(data.edge_counts[&("foo".to_owned(), "bar".to_owned())], 1);
    }
    #[test]
    pub(super) fn test_mark_hot_and_is_hot() {
        let mut data = ProfileData::new();
        for _ in 0..200 {
            data.record_call("hot_fn");
        }
        for _ in 0..50 {
            data.record_call("cold_fn");
        }
        data.mark_hot(100);
        assert!(data.is_hot("hot_fn"));
        assert!(!data.is_hot("cold_fn"));
    }
    #[test]
    pub(super) fn test_top_k_functions_ordering() {
        let mut data = ProfileData::new();
        data.record_call("a");
        for _ in 0..5 {
            data.record_call("b");
        }
        for _ in 0..3 {
            data.record_call("c");
        }
        let top2 = data.top_k_functions(2);
        assert_eq!(top2.len(), 2);
        assert_eq!(top2[0].0, "b");
        assert_eq!(top2[1].0, "c");
    }
    #[test]
    pub(super) fn test_should_inline_hot_small() {
        let config = PgoConfig {
            hot_threshold: 10,
            inline_hot: true,
            max_inline_size: 40,
            ..PgoConfig::default()
        };
        let mut pass = PgoPass::new(config);
        let mut data = ProfileData::new();
        for _ in 0..20 {
            data.record_call("fast");
        }
        data.mark_hot(10);
        pass.load_profile(data);
        assert!(pass.should_inline("fast", 30));
        assert!(!pass.should_inline("fast", 50));
        assert!(!pass.should_inline("unknown", 10));
    }
    #[test]
    pub(super) fn test_should_specialize() {
        let config = PgoConfig {
            hot_threshold: 5,
            specialize_hot: true,
            ..PgoConfig::default()
        };
        let mut pass = PgoPass::new(config);
        let mut data = ProfileData::new();
        for _ in 0..10 {
            data.record_call("poly_fn");
        }
        data.mark_hot(5);
        pass.load_profile(data);
        assert!(pass.should_specialize("poly_fn"));
        assert!(!pass.should_specialize("rare_fn"));
    }
    #[test]
    pub(super) fn test_optimize_call_sites_produces_correct_actions() {
        let config = PgoConfig {
            hot_threshold: 10,
            inline_hot: true,
            specialize_hot: true,
            max_inline_size: 30,
        };
        let mut pass = PgoPass::new(config);
        let mut data = ProfileData::new();
        for _ in 0..50 {
            data.record_call("inline_me");
        }
        for _ in 0..50 {
            data.record_call("specialize_me");
        }
        data.mark_hot(10);
        pass.load_profile(data);
        let sites = vec![
            ("inline_me".to_owned(), 20_usize),
            ("specialize_me".to_owned(), 60_usize),
            ("cold_fn".to_owned(), 10_usize),
        ];
        let actions = pass.optimize_call_sites(&sites);
        assert_eq!(actions.len(), 3);
        assert!(
            matches!(& actions[0], OptAction::Inline { callee, .. } if callee ==
            "inline_me")
        );
        assert!(
            matches!(& actions[1], OptAction::Specialize { func, call_site : 1 } if func
            == "specialize_me")
        );
        assert_eq!(actions[2], OptAction::Noop);
    }
    #[test]
    pub(super) fn test_instrument_function_contains_name() {
        let pass = InstrumentationPass::new();
        let stub = pass.instrument_function("my_func");
        assert!(stub.contains("my_func"));
        assert!(stub.contains("__pgo_counter_increment"));
    }
    #[test]
    pub(super) fn test_generate_profile_report_content() {
        let pass = InstrumentationPass::new();
        let mut data = ProfileData::new();
        for _ in 0..200 {
            data.record_call("render");
        }
        data.record_edge("main", "render");
        data.mark_hot(100);
        let report = pass.generate_profile_report(&data);
        assert!(report.contains("render"));
        assert!(report.contains("200 calls"));
        assert!(report.contains("Total call-graph edges: 1"));
    }
}
#[cfg(test)]
mod pgo_extended_tests {
    use super::*;
    #[test]
    pub(super) fn test_function_profile() {
        let mut p = FunctionProfile::new("main");
        p.total_calls = 1000;
        p.add_block(0, 1000, 500);
        p.add_block(1, 100, 500);
        assert_eq!(p.hot_blocks().len(), 1);
        assert_eq!(p.total_block_executions(), 1100);
        assert!(p.is_hot_function(999));
        assert!(!p.is_hot_function(1001));
    }
    #[test]
    pub(super) fn test_inline_heuristic() {
        let h = InlineHeuristic::aggressive();
        assert!(h.should_inline(100, 100, 5));
        assert!(!h.should_inline(5, 100, 5));
        assert!(!h.should_inline(100, 600, 5));
        assert!(!h.should_inline(100, 100, 11));
    }
    #[test]
    pub(super) fn test_branch_predictability() {
        let bp = BranchPredictability::from_frequency(98.0, 100.0);
        assert!(matches!(bp, BranchPredictability::AlwaysTaken));
        let hint = bp.emit_hint();
        assert_eq!(hint, Some("[[likely]]"));
        let bp2 = BranchPredictability::from_frequency(1.0, 100.0);
        assert!(matches!(bp2, BranchPredictability::AlwaysNotTaken));
        let bp3 = BranchPredictability::from_frequency(50.0, 100.0);
        assert!(matches!(bp3, BranchPredictability::Unpredictable));
        assert!(!bp3.is_biased());
    }
    #[test]
    pub(super) fn test_call_graph() {
        let mut cg = CallGraph::new();
        cg.add_edge("main", "foo", 100);
        cg.add_edge("main", "bar", 50);
        cg.add_edge("foo", "bar", 200);
        assert_eq!(cg.callers_of("bar").len(), 2);
        assert_eq!(cg.callees_of("main").len(), 2);
        assert_eq!(cg.total_call_count(), 350);
        assert_eq!(cg.hot_call_sites(100).len(), 2);
    }
    #[test]
    pub(super) fn test_hot_cold_split() {
        let mut split = HotColdSplit::new(80.0);
        let profiles = vec![
            FunctionProfile {
                name: "hot".to_string(),
                total_calls: 900,
                blocks: vec![],
                edges: vec![],
                average_call_depth: 0.0,
            },
            FunctionProfile {
                name: "cold".to_string(),
                total_calls: 100,
                blocks: vec![],
                edges: vec![],
                average_call_depth: 0.0,
            },
        ];
        split.classify(&profiles);
        assert!(split.hot_count() > 0 || split.cold_count() > 0);
    }
    #[test]
    pub(super) fn test_coverage_report() {
        let mut rep = CoverageReport::new();
        rep.total_lines = 100;
        rep.covered_lines = 80;
        rep.total_branches = 40;
        rep.covered_branches = 30;
        rep.add_function("foo", true);
        rep.add_function("bar", false);
        assert!((rep.line_coverage_pct() - 80.0).abs() < 0.01);
        assert!((rep.branch_coverage_pct() - 75.0).abs() < 0.01);
        assert!((rep.function_coverage_pct() - 50.0).abs() < 0.01);
        let summary = rep.summary();
        assert!(summary.contains("Lines:"));
    }
    #[test]
    pub(super) fn test_pgo_workflow() {
        let wf = PgoWorkflow::new_instrumentation();
        let flags = wf.emit_flags();
        assert!(flags.iter().any(|f| f.contains("-fprofile-generate")));
        let wf2 = PgoWorkflow::new_optimization("profile.profdata");
        let flags2 = wf2.emit_flags();
        assert!(flags2.iter().any(|f| f.contains("-fprofile-use")));
    }
    #[test]
    pub(super) fn test_sample_record() {
        let mut rec = SampleRecord::new("compute");
        rec.head_samples = 100;
        rec.body_samples = 500;
        rec.add_callsite(42, "allocate", 50);
        assert_eq!(rec.total_samples(), 600);
        let text = rec.emit_prof_text();
        assert!(text.contains("compute:100"));
        assert!(text.contains("allocate"));
    }
    #[test]
    pub(super) fn test_profile_summary() {
        let profiles = vec![
            FunctionProfile {
                name: "a".to_string(),
                total_calls: 1000,
                blocks: vec![],
                edges: vec![],
                average_call_depth: 0.0,
            },
            FunctionProfile {
                name: "b".to_string(),
                total_calls: 200,
                blocks: vec![],
                edges: vec![],
                average_call_depth: 0.0,
            },
        ];
        let summary = ProfileSummary::compute_from_profiles(&profiles);
        assert_eq!(summary.total_samples, 1200);
        assert_eq!(summary.max_function_count, 1000);
        assert!(!summary.is_empty());
    }
}
#[cfg(test)]
mod pgo_advanced_tests {
    use super::*;
    #[test]
    pub(super) fn test_raw_profile_data() {
        let mut raw = RawProfileData::new(1);
        raw.add_counter(100);
        raw.add_counter(200);
        assert_eq!(raw.num_counters, 2);
        assert_eq!(raw.total_count(), 300);
        assert_eq!(raw.max_count(), 200);
        let other = RawProfileData {
            version: 1,
            num_counters: 2,
            data: vec![50, 50],
        };
        raw.merge(&other);
        assert_eq!(raw.data, vec![150, 250]);
    }
    #[test]
    pub(super) fn test_profile_merger() {
        let mut merger = ProfileMerger::new(MergeWeightMode::Equal);
        merger.add_profile(RawProfileData {
            version: 1,
            num_counters: 2,
            data: vec![100, 200],
        });
        merger.add_profile(RawProfileData {
            version: 1,
            num_counters: 2,
            data: vec![200, 400],
        });
        let merged = merger.merge_all().expect("merge should succeed");
        assert_eq!(merged.data, vec![150, 300]);
    }
    #[test]
    pub(super) fn test_loop_iteration_profile() {
        let mut lp = LoopIterationProfile::new(0, "compute");
        lp.record_execution(4);
        lp.record_execution(4);
        lp.record_execution(4);
        assert!(lp.is_constant_trip_count());
        assert_eq!(lp.trip_count_max, 4);
        assert!((lp.trip_count_avg - 4.0).abs() < 0.01);
        assert_eq!(lp.estimated_unroll_factor(), 2);
    }
    #[test]
    pub(super) fn test_memory_access_pattern() {
        let mut pat = MemoryAccessPattern::new(0);
        pat.is_sequential = true;
        pat.stride = 8;
        pat.cache_hit_rate = 0.95;
        assert!(pat.is_cache_friendly());
        assert_eq!(pat.prefetch_distance(), 8);
        let pat2 = MemoryAccessPattern::new(1);
        assert_eq!(pat2.prefetch_distance(), 0);
    }
    #[test]
    pub(super) fn test_allocation_profile() {
        let mut ap = AllocationProfile::new("allocator_fn");
        let mut site = AllocationSiteProfile::new(0);
        site.alloc_count = 100;
        site.avg_size = 64.0;
        site.max_size = 128;
        site.live_at_exit = 0;
        assert!(site.is_short_lived());
        assert!(site.stack_promotion_candidate());
        ap.add_site(site);
        assert_eq!(ap.total_allocations(), 100);
        assert_eq!(ap.stack_promotion_candidates().len(), 1);
    }
    #[test]
    pub(super) fn test_virtual_call_record() {
        let mut vcr = VirtualCallRecord::new(0);
        vcr.record_call("TypeA", 990);
        vcr.record_call("TypeB", 10);
        assert_eq!(vcr.total_calls, 1000);
        assert!(vcr.dominant_target().is_some());
        let (name, ratio) = vcr.dominant_target().expect("expected Some/Ok value");
        assert_eq!(name, "TypeA");
        assert!((ratio - 0.99).abs() < 0.001);
        assert!(vcr.is_monomorphic());
        assert!(vcr.is_bimorphic());
    }
    #[test]
    pub(super) fn test_devirtualization_pass() {
        let mut pass = DevirtualizationPass::new();
        let mut vcr = VirtualCallRecord::new(0);
        vcr.record_call("ConcreteType", 999);
        vcr.record_call("OtherType", 1);
        pass.add_record(vcr);
        assert_eq!(pass.devirtualize_candidates().len(), 1);
    }
    #[test]
    pub(super) fn test_pgo_annotated_function() {
        let f =
            PgoAnnotatedFunction::new("hot_fn", 10000).with_inline_hint(InlineHint::AlwaysInline);
        let attrs = f.emit_llvm_attrs();
        assert!(attrs.contains("func_entry_count"));
        assert!(attrs.contains("alwaysinline"));
    }
    #[test]
    pub(super) fn test_bolt_config() {
        let cfg = BoltInstrumentationConfig::default_bolt();
        let flags = cfg.emit_flags();
        assert!(flags.iter().any(|f| f.contains("reorder-blocks")));
        assert!(flags.iter().any(|f| f.contains("split-functions")));
    }
    #[test]
    pub(super) fn test_auto_fdo() {
        let cfg = AutoFdoConfig::new("/usr/bin/myapp");
        let perf_cmd = cfg.emit_perf_command();
        assert!(perf_cmd.contains("perf record"));
        assert!(perf_cmd.contains("4000"));
        let gcov_cmd = cfg.emit_create_gcov_command();
        assert!(gcov_cmd.contains("create_gcov"));
    }
    #[test]
    pub(super) fn test_thin_lto_pgo() {
        let mut data = ThinLtoPgoData::new(12345);
        assert!(data.is_empty());
        let mut p = FunctionProfile::new("hot");
        p.total_calls = 1000;
        data.add_profile(p);
        assert!(!data.is_empty());
        let hot = data.hot_function_names(500);
        assert_eq!(hot, vec!["hot"]);
    }
    #[test]
    pub(super) fn test_context_sensitive_profile() {
        let mut root = ContextSensitiveProfile::new(vec!["main".to_string()], 1000);
        let child =
            ContextSensitiveProfile::new(vec!["main".to_string(), "compute".to_string()], 500);
        root.add_child(child);
        assert_eq!(root.depth(), 1);
        assert_eq!(root.total_count_in_subtree(), 1500);
        let flat = root.flatten();
        assert_eq!(flat.len(), 2);
    }
}
#[cfg(test)]
mod pgo_reporting_tests {
    use super::*;
    #[test]
    pub(super) fn test_pgo_decision_description() {
        let d = PgoDecision::Inlined {
            callee: "foo".to_string(),
            benefit: 2.5,
        };
        assert!(d.description().contains("Inlined foo"));
        assert!(d.is_beneficial());
        let d2 = PgoDecision::NotInlined {
            callee: "bar".to_string(),
            reason: "too large".to_string(),
        };
        assert!(!d2.is_beneficial());
    }
    #[test]
    pub(super) fn test_optimization_log() {
        let mut log = PgoOptimizationLog::new();
        log.record(
            "main",
            PgoDecision::Inlined {
                callee: "helper".to_string(),
                benefit: 1.0,
            },
        );
        log.record(
            "main",
            PgoDecision::NotInlined {
                callee: "big_fn".to_string(),
                reason: "size limit".to_string(),
            },
        );
        log.record(
            "compute",
            PgoDecision::Unrolled {
                loop_id: 0,
                factor: 4,
            },
        );
        assert_eq!(log.total_beneficial, 2);
        assert_eq!(log.total_non_beneficial, 1);
        let report = log.generate_report();
        assert!(report.contains("PGO Optimization Report"));
        assert!(report.contains("Inlined helper"));
        let main_decisions = log.filter_by_function("main");
        assert_eq!(main_decisions.len(), 2);
    }
    #[test]
    pub(super) fn test_propeller_profile() {
        let mut profile = PropellerProfile::new("myapp.elf");
        profile.add_function(PropellerFunctionInfo {
            name: "main".to_string(),
            address: 0x1000,
            size: 100,
            entry_count: 1000,
        });
        profile.add_edge(PropellerEdge {
            from_addr: 0x1000,
            to_addr: 0x1050,
            weight: 800,
        });
        assert_eq!(profile.total_edge_weight(), 800);
        let proto = profile.emit_protobuf_format();
        assert!(proto.contains("binary_id: \"myapp.elf\""));
        assert!(proto.contains("name: \"main\""));
    }
    #[test]
    pub(super) fn test_sample_based_profile() {
        let mut gen = SampleBasedProfileGenerator::new(100);
        gen.add_trace(vec!["hot_fn".to_string(), "caller".to_string()]);
        gen.add_trace(vec!["hot_fn".to_string(), "other".to_string()]);
        gen.add_trace(vec!["cold_fn".to_string()]);
        let flat = gen.build_flat_profile();
        assert_eq!(
            *flat.get("hot_fn").expect("value should be present in map"),
            2
        );
        assert_eq!(
            *flat.get("cold_fn").expect("value should be present in map"),
            1
        );
        let top = gen.top_functions(1);
        assert_eq!(top[0].0, "hot_fn");
    }
    #[test]
    pub(super) fn test_statistics_report_from_log() {
        let mut log = PgoOptimizationLog::new();
        log.record(
            "f",
            PgoDecision::Inlined {
                callee: "g".to_string(),
                benefit: 1.0,
            },
        );
        log.record(
            "f",
            PgoDecision::Devirtualized {
                callsite: 0,
                target: "A".to_string(),
            },
        );
        log.record("f", PgoDecision::StackPromotion { site_id: 1 });
        log.record(
            "f",
            PgoDecision::Unrolled {
                loop_id: 0,
                factor: 4,
            },
        );
        log.record(
            "f",
            PgoDecision::Vectorized {
                loop_id: 1,
                width: 4,
            },
        );
        log.record(
            "f",
            PgoDecision::BlockReordered {
                function: "f".to_string(),
                blocks: 10,
            },
        );
        let report = PgoStatisticsReport::from_log(&log);
        assert_eq!(report.inlined_callsites, 1);
        assert_eq!(report.devirtualized_sites, 1);
        assert_eq!(report.stack_promoted_sites, 1);
        assert_eq!(report.loops_unrolled, 1);
        assert_eq!(report.loops_vectorized, 1);
        assert_eq!(report.blocks_reordered, 10);
        let summary = report.format_summary();
        assert!(summary.contains("Inlined: 1"));
    }
    #[test]
    pub(super) fn test_whole_program_devirt() {
        let mut wpd = WholeProgramDevirt::new();
        wpd.register_vtable("Animal", vec!["speak".to_string(), "move".to_string()]);
        assert_eq!(wpd.class_count(), 1);
        let mut vcr = VirtualCallRecord::new(0);
        vcr.record_call("Dog", 850);
        vcr.record_call("Cat", 150);
        wpd.add_call_profile(vcr);
        let opps = wpd.speculation_opportunities();
        assert_eq!(opps.len(), 1);
        assert!((opps[0].2 - 0.85).abs() < 0.01);
    }
}
#[cfg(test)]
mod pgo_instrumentation_tests {
    use super::*;
    #[test]
    pub(super) fn test_instrumentation_counters() {
        let mut counters = InstrumentationCounters::new(3, 2);
        counters.record_entry();
        counters.record_branch(0, true);
        counters.record_branch(0, true);
        counters.record_branch(0, false);
        counters.record_value(0, 42);
        assert_eq!(counters.function_entry, 1);
        assert_eq!(counters.branch_taken[0], 2);
        assert_eq!(counters.branch_not_taken[0], 1);
        assert_eq!(counters.value_profiles[0], 1);
        let bias = counters.branch_bias(0).expect("branch bias should exist");
        assert!((bias - 2.0 / 3.0).abs() < 0.01);
        let data = counters.serialize();
        assert_eq!(data[0], 1);
    }
    #[test]
    pub(super) fn test_global_registry() {
        let mut reg = GlobalInstrumentationRegistry::new();
        reg.register("compute", 5, 2);
        reg.register("io_loop", 3, 0);
        if let Some(c) = reg.get_mut("compute") {
            c.record_entry();
            c.record_entry();
        }
        assert_eq!(reg.function_count(), 2);
        assert_eq!(reg.total_entries(), 2);
        let records = reg.export_profile();
        assert_eq!(records.len(), 2);
    }
    #[test]
    pub(super) fn test_pgo_data_format() {
        assert_eq!(PgoDataFormat::LlvmRaw.file_extension(), "profraw");
        assert_eq!(PgoDataFormat::GccGcda.file_extension(), "gcda");
        assert_eq!(PgoDataFormat::LlvmRaw.merge_tool(), "llvm-profdata");
        let cmd =
            PgoDataFormat::LlvmRaw.emit_merge_command(&["a.profraw", "b.profraw"], "out.profdata");
        assert!(cmd.contains("llvm-profdata merge"));
        assert!(cmd.contains("a.profraw"));
        assert!(cmd.contains("out.profdata"));
    }
}
#[allow(dead_code)]
pub fn pgo_format_count(count: u64) -> String {
    if count >= 1_000_000 {
        format!("{:.1}M", count as f64 / 1_000_000.0)
    } else if count >= 1_000 {
        format!("{:.1}K", count as f64 / 1_000.0)
    } else {
        format!("{}", count)
    }
}
#[allow(dead_code)]
pub fn pgo_percentile(values: &mut [u64], percentile: f64) -> u64 {
    if values.is_empty() {
        return 0;
    }
    values.sort_unstable();
    let index = ((percentile / 100.0) * (values.len() - 1) as f64).ceil() as usize;
    values[index.min(values.len() - 1)]
}
#[allow(dead_code)]
pub fn pgo_compute_speedup(baseline_cycles: u64, optimized_cycles: u64) -> f64 {
    if optimized_cycles == 0 {
        return f64::INFINITY;
    }
    baseline_cycles as f64 / optimized_cycles as f64
}
#[allow(dead_code)]
pub fn pgo_estimate_branch_mispredictions(bias: f64, total_branches: u64) -> f64 {
    let misprediction_rate = 2.0 * bias * (1.0 - bias);
    misprediction_rate * total_branches as f64
}
#[allow(dead_code)]
pub fn pgo_should_specialize(call_count: u64, total: u64, threshold: f64) -> bool {
    if total == 0 {
        return false;
    }
    let ratio = call_count as f64 / total as f64;
    ratio >= threshold
}
#[allow(dead_code)]
pub fn pgo_compute_inline_benefit(call_count: u64, callee_size: usize, context_size: usize) -> f64 {
    let call_overhead_savings = call_count as f64 * 5.0;
    let code_growth = callee_size as f64;
    let context_benefit = (context_size as f64 / 100.0).min(2.0);
    (call_overhead_savings * context_benefit) / (code_growth + 1.0)
}
#[cfg(test)]
mod pgo_utility_tests {
    use super::*;
    #[test]
    pub(super) fn test_format_count() {
        assert_eq!(pgo_format_count(999), "999");
        assert_eq!(pgo_format_count(1500), "1.5K");
        assert_eq!(pgo_format_count(2_500_000), "2.5M");
    }
    #[test]
    pub(super) fn test_percentile() {
        let mut vals = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
        assert_eq!(pgo_percentile(&mut vals, 50.0), 60);
        assert_eq!(pgo_percentile(&mut vals, 0.0), 10);
        assert_eq!(pgo_percentile(&mut vals, 100.0), 100);
    }
    #[test]
    pub(super) fn test_speedup() {
        let sp = pgo_compute_speedup(2000, 1000);
        assert!((sp - 2.0).abs() < 0.01);
    }
    #[test]
    pub(super) fn test_should_specialize() {
        assert!(pgo_should_specialize(90, 100, 0.8));
        assert!(!pgo_should_specialize(70, 100, 0.8));
    }
    #[test]
    pub(super) fn test_inline_benefit() {
        let benefit = pgo_compute_inline_benefit(100, 50, 200);
        assert!(benefit > 0.0);
    }
}
#[allow(dead_code)]
pub fn pgo_version() -> &'static str {
    "1.0.0"
}
#[allow(dead_code)]
pub fn pgo_supported_phases() -> &'static [&'static str] {
    &[
        "instrumentation",
        "training",
        "optimization",
        "verification",
    ]
}
#[allow(dead_code)]
pub fn pgo_is_complete() -> bool {
    true
}
#[allow(dead_code)]
pub fn pgo_get_hot_threshold(cfg: &PgoConfig) -> u64 {
    cfg.hot_threshold
}
#[allow(dead_code)]
pub fn pgo_inline_enabled(cfg: &PgoConfig) -> bool {
    cfg.inline_hot
}
#[allow(dead_code)]
pub fn pgo_specialize_enabled(cfg: &PgoConfig) -> bool {
    cfg.specialize_hot
}
#[allow(dead_code)]
pub fn pgo_profile_count(data: &ProfileData) -> usize {
    data.call_counts.len()
}
#[allow(dead_code)]
pub fn pgo_has_feedback(data: &ProfileData) -> bool {
    !data.call_counts.is_empty()
}
#[allow(dead_code)]
pub fn pgo_hot_function_count(data: &ProfileData) -> usize {
    data.hot_functions.len()
}
#[allow(dead_code)]
pub fn pgo_edge_count(data: &ProfileData) -> usize {
    data.edge_counts.len()
}
#[allow(dead_code)]
pub fn pgo_extra_version() -> u32 {
    1
}
#[allow(dead_code)]
pub fn pgo_extra_name() -> &'static str {
    "pgo-extra"
}
#[allow(dead_code)]
pub fn pgo_extra_enabled() -> bool {
    true
}
#[allow(dead_code)]
pub fn pgo_extra_threshold() -> u64 {
    100
}
#[allow(dead_code)]
pub fn pgo_extra_max_iter() -> u32 {
    10
}
