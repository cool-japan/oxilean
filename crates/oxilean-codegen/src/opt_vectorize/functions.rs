//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::{LcnfExpr, LcnfFunDecl, LcnfLetValue};
use std::collections::HashMap;

use super::types::{
    CmpOp, DependenceGraph, DependenceKind, LatencyClass, LoopTransformConfig, LoopTransformer,
    ReductionInfo, ReductionKind, SIMDCostModel, SIMDOp, SIMDTarget, SIMDTargetInfo,
    StrideAnalysisResult, StridePattern, VecAnalysisCache, VecConstantFoldingHelper, VecDepGraph,
    VecDominatorTree, VecLivenessInfo, VecPassConfig, VecPassPhase, VecPassRegistry, VecPassStats,
    VecWorklist, VectorInstr, VectorInstrBuilder, VectorPrologueEpilogue, VectorRegisterFile,
    VectorScheduler, VectorWidth, VectorizationAnalysis, VectorizationCandidate,
    VectorizationConfig, VectorizationHint, VectorizationPass, VectorizationPipeline,
    VectorizationReport,
};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub(super) fn vector_width_lanes() {
        assert_eq!(VectorWidth::W128.lanes_f32(), 4);
        assert_eq!(VectorWidth::W256.lanes_f32(), 8);
        assert_eq!(VectorWidth::W512.lanes_f32(), 16);
        assert_eq!(VectorWidth::W256.lanes_f64(), 4);
    }
    #[test]
    pub(super) fn simd_target_max_width() {
        assert_eq!(SIMDTarget::X86AVX.max_width(), VectorWidth::W256);
        assert_eq!(SIMDTarget::X86AVX512.max_width(), VectorWidth::W512);
        assert_eq!(SIMDTarget::ArmNeon.max_width(), VectorWidth::W128);
    }
    #[test]
    pub(super) fn candidate_no_dep() {
        let c = VectorizationCandidate {
            func_name: "loop_add".to_string(),
            loop_var: "i".to_string(),
            loop_bound: Some(1024),
            array_reads: vec!["a".to_string()],
            array_writes: vec!["b".to_string()],
            is_inner_loop: true,
            has_loop_carried_dep: false,
        };
        let analysis = VectorizationAnalysis::new();
        assert!(analysis.can_vectorize(&c));
        let speedup = analysis.estimate_speedup(&c, VectorWidth::W256);
        assert!(speedup > 1.0, "speedup={}", speedup);
    }
    #[test]
    pub(super) fn candidate_with_dep_rejected() {
        let c = VectorizationCandidate {
            func_name: "loop_reduce".to_string(),
            loop_var: "i".to_string(),
            loop_bound: Some(256),
            array_reads: vec!["acc".to_string()],
            array_writes: vec!["acc".to_string()],
            is_inner_loop: true,
            has_loop_carried_dep: true,
        };
        let analysis = VectorizationAnalysis::new();
        assert!(!analysis.can_vectorize(&c));
        assert_eq!(analysis.estimate_speedup(&c, VectorWidth::W256), 1.0);
    }
    #[test]
    pub(super) fn emit_vector_loop_fma() {
        let config = VectorizationConfig {
            enable_fma: true,
            target: SIMDTarget::X86AVX,
            ..VectorizationConfig::default()
        };
        let pass = VectorizationPass::new(config);
        let candidate = VectorizationCandidate {
            func_name: "dot_product".to_string(),
            loop_var: "i".to_string(),
            loop_bound: Some(512),
            array_reads: vec!["a".to_string(), "b".to_string()],
            array_writes: vec!["result".to_string()],
            is_inner_loop: true,
            has_loop_carried_dep: false,
        };
        let instrs = pass.emit_vector_loop(&candidate, VectorWidth::W256);
        assert!(!instrs.is_empty());
        let has_fma = instrs.iter().any(|i| i.op == SIMDOp::Fma);
        assert!(has_fma, "expected FMA instruction");
    }
    #[test]
    pub(super) fn vector_instr_display() {
        let instr = VectorInstr::new(
            SIMDOp::Add,
            VectorWidth::W128,
            "v0",
            vec!["v1".to_string(), "v2".to_string()],
        );
        let s = format!("{}", instr);
        assert!(s.contains("vadd"));
        assert!(s.contains("128"));
    }
    #[test]
    pub(super) fn report_merge() {
        let mut r1 = VectorizationReport {
            loops_analyzed: 3,
            loops_vectorized: 2,
            rejected_dep: 1,
            ..VectorizationReport::default()
        };
        let r2 = VectorizationReport {
            loops_analyzed: 2,
            loops_vectorized: 1,
            rejected_trip_count: 1,
            ..VectorizationReport::default()
        };
        r1.merge(&r2);
        assert_eq!(r1.loops_analyzed, 5);
        assert_eq!(r1.loops_vectorized, 3);
        assert_eq!(r1.rejected_dep, 1);
        assert_eq!(r1.rejected_trip_count, 1);
    }
    #[test]
    pub(super) fn effective_width_caps_at_target() {
        let config = VectorizationConfig {
            preferred_width: VectorWidth::W512,
            target: SIMDTarget::X86SSE,
            ..VectorizationConfig::default()
        };
        let pass = VectorizationPass::new(config);
        assert_eq!(pass.effective_width(), VectorWidth::W128);
    }
}
/// Returns the abstract latency for a `SIMDOp`.
#[allow(dead_code)]
pub fn simd_op_latency(op: &SIMDOp) -> LatencyClass {
    match op {
        SIMDOp::Broadcast => LatencyClass::SingleCycle,
        SIMDOp::Add | SIMDOp::Sub => LatencyClass::Short,
        SIMDOp::Mul => LatencyClass::Short,
        SIMDOp::Div => LatencyClass::Medium,
        SIMDOp::Sqrt => LatencyClass::Long,
        SIMDOp::Fma => LatencyClass::Short,
        SIMDOp::Load | SIMDOp::Store => LatencyClass::Memory,
        SIMDOp::Shuffle | SIMDOp::Blend => LatencyClass::Short,
        SIMDOp::Compare(_) => LatencyClass::Short,
        SIMDOp::Min | SIMDOp::Max => LatencyClass::Short,
        SIMDOp::HorizontalAdd => LatencyClass::Medium,
    }
}
/// A map from function name to a list of vectorization hints.
#[allow(dead_code)]
pub type HintMap = HashMap<String, Vec<VectorizationHint>>;
#[cfg(test)]
mod extended_tests {
    use super::*;
    #[test]
    pub(super) fn test_vector_register_file_alloc() {
        let mut rf = VectorRegisterFile::new(4);
        let r0 = rf.alloc("v");
        let r1 = rf.alloc("v");
        let r2 = rf.alloc("v");
        let r3 = rf.alloc("v");
        assert_eq!(rf.allocation.len(), 4);
        assert!(rf.is_full());
        let _r4 = rf.alloc("v");
        assert!(rf.spill_count() > 0);
        rf.free(&r0);
        assert!(!rf.is_full());
        let _ = (r1, r2, r3);
    }
    #[test]
    pub(super) fn test_vector_scheduler_ordering() {
        let mut builder = VectorInstrBuilder::new(VectorWidth::W256);
        let load_a = builder.load("a_ptr");
        let load_b = builder.load("b_ptr");
        let mul = builder.mul(&load_a, &load_b);
        let _hadd = builder.hadd(&mul);
        let instrs = builder.build();
        let scheduled = VectorScheduler::schedule(&instrs);
        assert_eq!(scheduled.len(), instrs.len());
        let makespan = VectorScheduler::makespan(&scheduled);
        assert!(makespan > 0);
    }
    #[test]
    pub(super) fn test_simd_cost_model() {
        let model = SIMDCostModel::default();
        let candidate = VectorizationCandidate {
            func_name: "test".into(),
            loop_var: "i".into(),
            loop_bound: Some(1024),
            array_reads: vec!["a".into(), "b".into()],
            array_writes: vec!["c".into()],
            is_inner_loop: true,
            has_loop_carried_dep: false,
        };
        let gain = model.throughput_gain(&candidate, VectorWidth::W256);
        assert!(gain > 0.0);
    }
    #[test]
    pub(super) fn test_dependence_graph() {
        let mut dg = DependenceGraph::default();
        dg.add_edge("a", "b", DependenceKind::True, 0);
        dg.add_edge("b", "c", DependenceKind::Anti, 1);
        dg.add_edge("c", "c", DependenceKind::Output, 2);
        assert!(dg.has_carried_dependence());
        assert_eq!(dg.max_distance(), 2);
        assert_eq!(dg.edges_of_kind(DependenceKind::True).len(), 1);
        assert_eq!(dg.edges_of_kind(DependenceKind::Anti).len(), 1);
    }
    #[test]
    pub(super) fn test_loop_transformer() {
        let candidate = VectorizationCandidate::new("my_loop", "i");
        let transformer = LoopTransformer::new();
        let result = transformer.transform(&candidate, VectorWidth::W256);
        assert!(result.transformed_name.contains("my_loop"));
        assert!(result.strip_mined);
        assert!(result.vector_instr_count > 0);
    }
    #[test]
    pub(super) fn test_vectorization_hints_display() {
        assert_eq!(VectorizationHint::Force.to_string(), "#[vectorize(force)]");
        assert_eq!(
            VectorizationHint::Disable.to_string(),
            "#[vectorize(disable)]"
        );
        assert_eq!(
            VectorizationHint::Unroll(4).to_string(),
            "#[vectorize(unroll=4)]"
        );
        assert_eq!(
            VectorizationHint::Width(VectorWidth::W256).to_string(),
            "#[vectorize(width=256)]"
        );
    }
    #[test]
    pub(super) fn test_reduction_info() {
        let sum = ReductionInfo::sum("acc");
        assert_eq!(sum.kind, ReductionKind::Sum);
        assert_eq!(sum.initial_value, 0);
        assert_eq!(sum.reduction_op(), SIMDOp::Add);
        let prod = ReductionInfo::product("p");
        assert_eq!(prod.initial_value, 1);
        assert_eq!(prod.reduction_op(), SIMDOp::Mul);
    }
    #[test]
    pub(super) fn test_vector_instr_builder() {
        let mut builder = VectorInstrBuilder::new(VectorWidth::W128);
        let a = builder.load("a_ptr");
        let b = builder.load("b_ptr");
        let c = builder.broadcast("scalar");
        let fma_r = builder.fma(&a, &b, &c);
        let _hadd = builder.hadd(&fma_r);
        let instrs = builder.build();
        assert!(!instrs.is_empty());
        let has_fma = instrs.iter().any(|i| i.op == SIMDOp::Fma);
        assert!(has_fma);
    }
    #[test]
    pub(super) fn test_simd_target_info() {
        let avx512 = SIMDTargetInfo::new(SIMDTarget::X86AVX512);
        assert_eq!(avx512.num_vector_registers(), 16);
        assert!(avx512.supports_masking());
        assert!(avx512.supports_scatter());
        assert_eq!(avx512.preferred_alignment(), 32);
        let neon = SIMDTargetInfo::new(SIMDTarget::ArmNeon);
        assert_eq!(neon.num_vector_registers(), 32);
        assert!(neon.supports_masking());
        assert!(!neon.supports_gather());
    }
    #[test]
    pub(super) fn test_prologue_epilogue() {
        let pe = VectorPrologueEpilogue::new(VectorWidth::W256);
        assert_eq!(pe.prologue_iterations(0, 4), 0);
        assert_eq!(pe.prologue_iterations(12, 4), 5);
        assert_eq!(pe.epilogue_iterations(100, 4), 0);
        assert_eq!(pe.epilogue_iterations(101, 4), 1);
    }
    #[test]
    pub(super) fn test_stride_pattern_display() {
        assert_eq!(StridePattern::Unit.to_string(), "unit");
        assert_eq!(StridePattern::Constant(2).to_string(), "const(2)");
        assert_eq!(StridePattern::Irregular.to_string(), "irregular");
    }
    #[test]
    pub(super) fn test_stride_analysis_result() {
        let unit = StrideAnalysisResult::unit("arr");
        assert!(unit.is_vectorizable);
        let stride2 = StrideAnalysisResult::constant("arr", 2);
        assert!(!stride2.is_vectorizable);
        let neg1 = StrideAnalysisResult::constant("arr", -1);
        assert!(neg1.is_vectorizable);
        let irregular = StrideAnalysisResult::irregular("arr");
        assert!(!irregular.is_vectorizable);
    }
    #[test]
    pub(super) fn test_vectorization_pipeline() {
        use crate::lcnf::*;
        let decl = LcnfFunDecl {
            name: "loop_sum".to_string(),
            original_name: None,
            params: vec![
                LcnfParam {
                    id: LcnfVarId(0),
                    ty: LcnfType::Nat,
                    name: "i".to_string(),
                    erased: false,
                    borrowed: false,
                },
                LcnfParam {
                    id: LcnfVarId(1),
                    ty: LcnfType::Nat,
                    name: "acc".to_string(),
                    erased: false,
                    borrowed: false,
                },
            ],
            ret_type: LcnfType::Nat,
            body: LcnfExpr::Let {
                id: LcnfVarId(2),
                name: "bound".to_string(),
                ty: LcnfType::Nat,
                value: LcnfLetValue::Lit(LcnfLit::Nat(1024)),
                body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(1)))),
            },
            is_recursive: true,
            is_lifted: false,
            inline_cost: 10,
        };
        let pipeline = VectorizationPipeline::new();
        let mut decls = vec![decl];
        let result = pipeline.run(&mut decls);
        assert!(result.report.loops_analyzed >= 0);
    }
    #[test]
    pub(super) fn test_latency_ordering() {
        assert!(simd_op_latency(&SIMDOp::Add) < simd_op_latency(&SIMDOp::Sqrt));
        assert!(simd_op_latency(&SIMDOp::Mul) <= simd_op_latency(&SIMDOp::Div));
        assert_eq!(
            simd_op_latency(&SIMDOp::Broadcast),
            LatencyClass::SingleCycle
        );
        assert_eq!(simd_op_latency(&SIMDOp::Load), LatencyClass::Memory);
    }
    #[test]
    pub(super) fn test_reduction_kind_display() {
        assert_eq!(ReductionKind::Sum.to_string(), "sum");
        assert_eq!(ReductionKind::DotProduct.to_string(), "dot_product");
        assert_eq!(ReductionKind::Min.to_string(), "min");
    }
    #[test]
    pub(super) fn test_dependence_kind_display() {
        assert_eq!(DependenceKind::True.to_string(), "RAW");
        assert_eq!(DependenceKind::Anti.to_string(), "WAR");
        assert_eq!(DependenceKind::Output.to_string(), "WAW");
        assert_eq!(DependenceKind::Input.to_string(), "RAR");
    }
    #[test]
    pub(super) fn test_loop_transform_config_default() {
        let cfg = LoopTransformConfig::default();
        assert_eq!(cfg.unroll_factor, 4);
        assert_eq!(cfg.tile_size, 64);
        assert!(cfg.strip_mine);
    }
    #[test]
    pub(super) fn test_vector_instr_builder_blend_cmp() {
        let mut builder = VectorInstrBuilder::new(VectorWidth::W256);
        let a = builder.load("a_ptr");
        let b = builder.load("b_ptr");
        let mask = builder.cmp(CmpOp::Lt, &a, &b);
        let _blended = builder.blend(&a, &b, &mask);
        let instrs = builder.build();
        let has_cmp = instrs.iter().any(|i| matches!(i.op, SIMDOp::Compare(_)));
        let has_blend = instrs.iter().any(|i| i.op == SIMDOp::Blend);
        assert!(has_cmp);
        assert!(has_blend);
    }
}
#[cfg(test)]
mod Vec_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = VecPassConfig::new("test_pass", VecPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = VecPassStats::new();
        stats.record_run(10, 100, 3);
        stats.record_run(20, 200, 5);
        assert_eq!(stats.total_runs, 2);
        assert!((stats.average_changes_per_run() - 15.0).abs() < 0.01);
        assert!((stats.success_rate() - 1.0).abs() < 0.01);
        let s = stats.format_summary();
        assert!(s.contains("Runs: 2/2"));
    }
    #[test]
    pub(super) fn test_pass_registry() {
        let mut reg = VecPassRegistry::new();
        reg.register(VecPassConfig::new("pass_a", VecPassPhase::Analysis));
        reg.register(VecPassConfig::new("pass_b", VecPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = VecAnalysisCache::new(10);
        cache.insert("key1".to_string(), vec![1, 2, 3]);
        assert!(cache.get("key1").is_some());
        assert!(cache.get("key2").is_none());
        assert!((cache.hit_rate() - 0.5).abs() < 0.01);
        cache.invalidate("key1");
        assert!(!cache.entries["key1"].valid);
        assert_eq!(cache.size(), 1);
    }
    #[test]
    pub(super) fn test_worklist() {
        let mut wl = VecWorklist::new();
        assert!(wl.push(1));
        assert!(wl.push(2));
        assert!(!wl.push(1));
        assert_eq!(wl.len(), 2);
        assert_eq!(wl.pop(), Some(1));
        assert!(!wl.contains(1));
        assert!(wl.contains(2));
    }
    #[test]
    pub(super) fn test_dominator_tree() {
        let mut dt = VecDominatorTree::new(5);
        dt.set_idom(1, 0);
        dt.set_idom(2, 0);
        dt.set_idom(3, 1);
        assert!(dt.dominates(0, 3));
        assert!(dt.dominates(1, 3));
        assert!(!dt.dominates(2, 3));
        assert!(dt.dominates(3, 3));
    }
    #[test]
    pub(super) fn test_liveness() {
        let mut liveness = VecLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(VecConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(VecConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(VecConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            VecConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(VecConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = VecDepGraph::new();
        g.add_dep(1, 2);
        g.add_dep(2, 3);
        g.add_dep(1, 3);
        assert_eq!(g.dependencies_of(2), vec![1]);
        let topo = g.topological_sort();
        assert_eq!(topo.len(), 3);
        assert!(!g.has_cycle());
        let pos: std::collections::HashMap<u32, usize> =
            topo.iter().enumerate().map(|(i, &n)| (n, i)).collect();
        assert!(pos[&1] < pos[&2]);
        assert!(pos[&1] < pos[&3]);
        assert!(pos[&2] < pos[&3]);
    }
}
