//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    MemFlags, MetalAddressSpace, MetalAnalysisCache, MetalBackend, MetalBinOp, MetalBuiltin,
    MetalConstantFoldingHelper, MetalDepGraph, MetalDominatorTree, MetalExpr, MetalExtCache,
    MetalExtConfig, MetalExtConstFolder, MetalExtDepGraph, MetalExtDiagCollector, MetalExtDiagMsg,
    MetalExtDomTree, MetalExtEmitStats, MetalExtEventLog, MetalExtFeatures, MetalExtIdGen,
    MetalExtIncrKey, MetalExtLiveness, MetalExtNameScope, MetalExtPassConfig, MetalExtPassPhase,
    MetalExtPassRegistry, MetalExtPassStats, MetalExtPassTiming, MetalExtProfiler,
    MetalExtSourceBuffer, MetalExtVersion, MetalExtWorklist, MetalField, MetalFunction,
    MetalLivenessInfo, MetalParam, MetalParamAttr, MetalPassConfig, MetalPassPhase,
    MetalPassRegistry, MetalPassStats, MetalShader, MetalStage, MetalStmt, MetalStruct, MetalType,
    MetalWorklist,
};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub(super) fn test_metal_type_display() {
        assert_eq!(format!("{}", MetalType::Float), "float");
        assert_eq!(format!("{}", MetalType::Float4), "float4");
        assert_eq!(format!("{}", MetalType::Float4x4), "float4x4");
        assert_eq!(format!("{}", MetalType::Half), "half");
        assert_eq!(format!("{}", MetalType::Uint3), "uint3");
        assert_eq!(format!("{}", MetalType::Bool), "bool");
        assert_eq!(format!("{}", MetalType::Sampler), "sampler");
        assert_eq!(format!("{}", MetalType::Void), "void");
        assert_eq!(
            format!("{}", MetalType::Texture(Box::new(MetalType::Float))),
            "texture2d<float>"
        );
        assert_eq!(
            format!(
                "{}",
                MetalType::Pointer(Box::new(MetalType::Float), MetalAddressSpace::Device)
            ),
            "device float*"
        );
        assert_eq!(
            format!("{}", MetalType::Array(Box::new(MetalType::Int), 8)),
            "int[8]"
        );
    }
    #[test]
    pub(super) fn test_metal_address_space_and_stage() {
        assert_eq!(format!("{}", MetalAddressSpace::Device), "device");
        assert_eq!(format!("{}", MetalAddressSpace::Constant), "constant");
        assert_eq!(format!("{}", MetalAddressSpace::Threadgroup), "threadgroup");
        assert_eq!(
            format!("{}", MetalAddressSpace::ThreadgroupImageblock),
            "threadgroup_imageblock"
        );
        assert_eq!(format!("{}", MetalAddressSpace::RayData), "ray_data");
        assert_eq!(format!("{}", MetalAddressSpace::Thread), "thread");
        assert_eq!(format!("{}", MetalStage::Vertex), "[[vertex]]");
        assert_eq!(format!("{}", MetalStage::Fragment), "[[fragment]]");
        assert_eq!(format!("{}", MetalStage::Kernel), "[[kernel]]");
        assert_eq!(format!("{}", MetalStage::Mesh), "[[mesh]]");
        assert_eq!(format!("{}", MetalStage::Device), "");
    }
    #[test]
    pub(super) fn test_metal_expr_emit() {
        let backend = MetalBackend::new();
        let add = MetalExpr::BinOp(
            Box::new(MetalExpr::Var("a".into())),
            MetalBinOp::Add,
            Box::new(MetalExpr::Var("b".into())),
        );
        assert_eq!(backend.emit_expr(&add), "(a + b)");
        let simd_sum = MetalExpr::SimdSum(Box::new(MetalExpr::Var("val".into())));
        assert_eq!(backend.emit_expr(&simd_sum), "simd_sum(val)");
        let dot = MetalExpr::Dot(
            Box::new(MetalExpr::Var("u".into())),
            Box::new(MetalExpr::Var("v".into())),
        );
        assert_eq!(backend.emit_expr(&dot), "dot(u, v)");
        let norm = MetalExpr::Normalize(Box::new(MetalExpr::Var("n".into())));
        assert_eq!(backend.emit_expr(&norm), "normalize(n)");
        let clamp = MetalExpr::Clamp(
            Box::new(MetalExpr::Var("x".into())),
            Box::new(MetalExpr::LitFloat(0.0)),
            Box::new(MetalExpr::LitFloat(1.0)),
        );
        assert!(backend.emit_expr(&clamp).contains("clamp"));
        let atom = MetalExpr::AtomicFetchAdd(
            Box::new(MetalExpr::Var("counter".into())),
            Box::new(MetalExpr::LitInt(1)),
        );
        assert!(backend
            .emit_expr(&atom)
            .contains("atomic_fetch_add_explicit"));
        assert!(backend.emit_expr(&atom).contains("memory_order_relaxed"));
        let bitcast = MetalExpr::AsType(MetalType::Uint, Box::new(MetalExpr::Var("f".into())));
        assert!(backend.emit_expr(&bitcast).contains("as_type<uint>"));
    }
    #[test]
    pub(super) fn test_metal_stmt_emit() {
        let backend = MetalBackend::new();
        let decl = MetalStmt::VarDecl {
            ty: MetalType::Float,
            name: "x".into(),
            init: Some(MetalExpr::LitFloat(1.0)),
            is_const: true,
        };
        let s = backend.emit_stmt(&decl, 0);
        assert!(s.contains("const float x"));
        let barrier = MetalStmt::Barrier(MemFlags::Threadgroup);
        let s = backend.emit_stmt(&barrier, 0);
        assert!(s.contains("threadgroup_barrier"));
        assert!(s.contains("mem_threadgroup"));
        let for_stmt = MetalStmt::ForLoop {
            init: Box::new(MetalStmt::VarDecl {
                ty: MetalType::Uint,
                name: "i".into(),
                init: Some(MetalExpr::LitInt(0)),
                is_const: false,
            }),
            cond: MetalExpr::BinOp(
                Box::new(MetalExpr::Var("i".into())),
                MetalBinOp::Lt,
                Box::new(MetalExpr::LitInt(32)),
            ),
            step: MetalExpr::BinOp(
                Box::new(MetalExpr::Var("i".into())),
                MetalBinOp::Add,
                Box::new(MetalExpr::LitInt(1)),
            ),
            body: vec![MetalStmt::Expr(MetalExpr::AtomicFetchAdd(
                Box::new(MetalExpr::Var("sum".into())),
                Box::new(MetalExpr::LitInt(1)),
            ))],
        };
        let s = backend.emit_stmt(&for_stmt, 0);
        assert!(s.contains("for"));
        assert!(s.contains("uint i = 0"));
    }
    #[test]
    pub(super) fn test_compute_kernel_emit() {
        let backend = MetalBackend::new();
        let tid_param = MetalParam::builtin(MetalBuiltin::ThreadPositionInGrid);
        let tpg_param = MetalParam::builtin(MetalBuiltin::ThreadsPerGrid);
        let kernel = MetalFunction::kernel("vec_add")
            .add_param(MetalParam::buffer(
                MetalType::Pointer(Box::new(MetalType::Float), MetalAddressSpace::Device),
                "a",
                0,
            ))
            .add_param(MetalParam::buffer(
                MetalType::Pointer(Box::new(MetalType::Float), MetalAddressSpace::Device),
                "b",
                1,
            ))
            .add_param(MetalParam::buffer(
                MetalType::Pointer(Box::new(MetalType::Float), MetalAddressSpace::Device),
                "c",
                2,
            ))
            .add_param(tid_param)
            .add_param(tpg_param)
            .add_stmt(MetalStmt::Assign {
                lhs: MetalExpr::Index(
                    Box::new(MetalExpr::Var("c".into())),
                    Box::new(MetalExpr::Var("tid".into())),
                ),
                rhs: MetalExpr::BinOp(
                    Box::new(MetalExpr::Index(
                        Box::new(MetalExpr::Var("a".into())),
                        Box::new(MetalExpr::Var("tid".into())),
                    )),
                    MetalBinOp::Add,
                    Box::new(MetalExpr::Index(
                        Box::new(MetalExpr::Var("b".into())),
                        Box::new(MetalExpr::Var("tid".into())),
                    )),
                ),
            });
        let src = backend.emit_function(&kernel);
        assert!(src.contains("[[kernel]]"));
        assert!(src.contains("vec_add"));
        assert!(src.contains("[[buffer(0)]]"));
        assert!(src.contains("[[buffer(1)]]"));
        assert!(src.contains("[[buffer(2)]]"));
        assert!(src.contains("[[thread_position_in_grid]]"));
    }
    #[test]
    pub(super) fn test_vertex_fragment_emit() {
        let backend = MetalBackend::new();
        let vout = MetalStruct::new("VertexOut")
            .add_field(MetalField::with_builtin(
                MetalType::Float4,
                "position",
                MetalBuiltin::Position,
            ))
            .add_field(MetalField::new(MetalType::Float2, "uv"));
        let struct_src = backend.emit_struct(&vout);
        assert!(struct_src.contains("struct VertexOut"));
        assert!(struct_src.contains("[[position]]"));
        assert!(struct_src.contains("float4 position"));
        assert!(struct_src.contains("float2 uv"));
        let frag = MetalFunction::fragment("simple_frag", MetalType::Float4)
            .add_param(MetalParam {
                ty: MetalType::Struct("VertexOut".into()),
                name: "in".into(),
                attr: MetalParamAttr::StageIn,
            })
            .add_stmt(MetalStmt::Return(Some(MetalExpr::Var(
                "float4(1.0)".into(),
            ))));
        let src = backend.emit_function(&frag);
        assert!(src.contains("[[fragment]]"));
        assert!(src.contains("float4 simple_frag"));
        assert!(src.contains("[[stage_in]]"));
    }
    #[test]
    pub(super) fn test_shader_module_emit() {
        let backend = MetalBackend::new();
        let shader = MetalShader::new()
            .add_constant(MetalType::Uint, "TILE_SIZE", MetalExpr::LitInt(16))
            .add_struct(
                MetalStruct::new("Uniforms")
                    .add_field(MetalField::new(MetalType::Float4x4, "modelMatrix")),
            )
            .add_function(MetalFunction::kernel("empty_kernel"));
        let src = backend.emit_shader(&shader);
        assert!(src.contains("#include <metal_stdlib>"));
        assert!(src.contains("using namespace metal;"));
        assert!(src.contains("constant uint TILE_SIZE = 16;"));
        assert!(src.contains("struct Uniforms"));
        assert!(src.contains("float4x4 modelMatrix"));
        assert!(src.contains("[[kernel]]"));
        assert!(src.contains("empty_kernel"));
    }
    #[test]
    pub(super) fn test_builtin_attributes_and_simd() {
        let backend = MetalBackend::new();
        assert!(MetalBuiltin::ThreadPositionInGrid
            .attribute()
            .contains("thread_position_in_grid"));
        assert!(MetalBuiltin::VertexId.attribute().contains("vertex_id"));
        assert!(MetalBuiltin::Position.attribute().contains("position"));
        assert!(MetalBuiltin::FrontFacing
            .attribute()
            .contains("front_facing"));
        assert!(MetalBuiltin::Depth.attribute().contains("depth"));
        assert_eq!(MetalBuiltin::Position.metal_type(), MetalType::Float4);
        assert_eq!(MetalBuiltin::FrontFacing.metal_type(), MetalType::Bool);
        assert_eq!(
            MetalBuiltin::ThreadPositionInGrid.metal_type(),
            MetalType::Uint3
        );
        let shfl = MetalExpr::SimdShuffleDown(
            Box::new(MetalExpr::Var("val".into())),
            Box::new(MetalExpr::LitInt(16)),
        );
        assert!(backend.emit_expr(&shfl).contains("simd_shuffle_down"));
        let bcast = MetalExpr::SimdBroadcast(
            Box::new(MetalExpr::Var("val".into())),
            Box::new(MetalExpr::LitInt(0)),
        );
        assert!(backend.emit_expr(&bcast).contains("simd_broadcast"));
        let sel = MetalExpr::Select(
            Box::new(MetalExpr::LitFloat(0.0)),
            Box::new(MetalExpr::LitFloat(1.0)),
            Box::new(MetalExpr::Var("mask".into())),
        );
        assert!(backend.emit_expr(&sel).contains("select"));
    }
}
#[cfg(test)]
mod tests_metal_ext_extra {
    use super::*;
    #[test]
    pub(super) fn test_metal_ext_config() {
        let mut cfg = MetalExtConfig::new();
        cfg.set("mode", "release");
        cfg.set("verbose", "true");
        assert_eq!(cfg.get("mode"), Some("release"));
        assert!(cfg.get_bool("verbose"));
        assert!(cfg.get_int("mode").is_none());
        assert_eq!(cfg.len(), 2);
    }
    #[test]
    pub(super) fn test_metal_ext_source_buffer() {
        let mut buf = MetalExtSourceBuffer::new();
        buf.push_line("fn main() {");
        buf.indent();
        buf.push_line("println!(\"hello\");");
        buf.dedent();
        buf.push_line("}");
        assert!(buf.as_str().contains("fn main()"));
        assert!(buf.as_str().contains("    println!"));
        assert_eq!(buf.line_count(), 3);
        buf.reset();
        assert!(buf.is_empty());
    }
    #[test]
    pub(super) fn test_metal_ext_name_scope() {
        let mut scope = MetalExtNameScope::new();
        assert!(scope.declare("x"));
        assert!(!scope.declare("x"));
        assert!(scope.is_declared("x"));
        let scope = scope.push_scope();
        assert_eq!(scope.depth(), 1);
        let mut scope = scope.pop_scope();
        assert_eq!(scope.depth(), 0);
        scope.declare("y");
        assert_eq!(scope.len(), 2);
    }
    #[test]
    pub(super) fn test_metal_ext_diag_collector() {
        let mut col = MetalExtDiagCollector::new();
        col.emit(MetalExtDiagMsg::warning("pass_a", "slow"));
        col.emit(MetalExtDiagMsg::error("pass_b", "fatal"));
        assert!(col.has_errors());
        assert_eq!(col.errors().len(), 1);
        assert_eq!(col.warnings().len(), 1);
        col.clear();
        assert!(col.is_empty());
    }
    #[test]
    pub(super) fn test_metal_ext_id_gen() {
        let mut gen = MetalExtIdGen::new();
        assert_eq!(gen.next_id(), 0);
        assert_eq!(gen.next_id(), 1);
        gen.skip(10);
        assert_eq!(gen.next_id(), 12);
        gen.reset();
        assert_eq!(gen.peek_next(), 0);
    }
    #[test]
    pub(super) fn test_metal_ext_incr_key() {
        let k1 = MetalExtIncrKey::new(100, 200);
        let k2 = MetalExtIncrKey::new(100, 200);
        let k3 = MetalExtIncrKey::new(999, 200);
        assert!(k1.matches(&k2));
        assert!(!k1.matches(&k3));
    }
    #[test]
    pub(super) fn test_metal_ext_profiler() {
        let mut p = MetalExtProfiler::new();
        p.record(MetalExtPassTiming::new("pass_a", 1000, 50, 200, 100));
        p.record(MetalExtPassTiming::new("pass_b", 500, 30, 100, 200));
        assert_eq!(p.total_elapsed_us(), 1500);
        assert_eq!(
            p.slowest_pass()
                .expect("slowest pass should exist")
                .pass_name,
            "pass_a"
        );
        assert_eq!(p.profitable_passes().len(), 1);
    }
    #[test]
    pub(super) fn test_metal_ext_event_log() {
        let mut log = MetalExtEventLog::new(3);
        log.push("event1");
        log.push("event2");
        log.push("event3");
        assert_eq!(log.len(), 3);
        log.push("event4");
        assert_eq!(log.len(), 3);
        assert_eq!(
            log.iter()
                .next()
                .expect("iterator should have next element"),
            "event2"
        );
    }
    #[test]
    pub(super) fn test_metal_ext_version() {
        let v = MetalExtVersion::new(1, 2, 3).with_pre("alpha");
        assert!(!v.is_stable());
        assert_eq!(format!("{}", v), "1.2.3-alpha");
        let stable = MetalExtVersion::new(2, 0, 0);
        assert!(stable.is_stable());
        assert!(stable.is_compatible_with(&MetalExtVersion::new(2, 0, 0)));
        assert!(!stable.is_compatible_with(&MetalExtVersion::new(3, 0, 0)));
    }
    #[test]
    pub(super) fn test_metal_ext_features() {
        let mut f = MetalExtFeatures::new();
        f.enable("sse2");
        f.enable("avx2");
        assert!(f.is_enabled("sse2"));
        assert!(!f.is_enabled("avx512"));
        f.disable("avx2");
        assert!(!f.is_enabled("avx2"));
        let mut g = MetalExtFeatures::new();
        g.enable("sse2");
        g.enable("neon");
        let union = f.union(&g);
        assert!(union.is_enabled("sse2") && union.is_enabled("neon"));
        let inter = f.intersection(&g);
        assert!(inter.is_enabled("sse2"));
    }
    #[test]
    pub(super) fn test_metal_ext_emit_stats() {
        let mut s = MetalExtEmitStats::new();
        s.bytes_emitted = 50_000;
        s.items_emitted = 500;
        s.elapsed_ms = 100;
        assert!(s.is_clean());
        assert!((s.throughput_bps() - 500_000.0).abs() < 1.0);
        let disp = format!("{}", s);
        assert!(disp.contains("bytes=50000"));
    }
}
#[cfg(test)]
mod Metal_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = MetalPassConfig::new("test_pass", MetalPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = MetalPassStats::new();
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
        let mut reg = MetalPassRegistry::new();
        reg.register(MetalPassConfig::new("pass_a", MetalPassPhase::Analysis));
        reg.register(MetalPassConfig::new("pass_b", MetalPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = MetalAnalysisCache::new(10);
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
        let mut wl = MetalWorklist::new();
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
        let mut dt = MetalDominatorTree::new(5);
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
        let mut liveness = MetalLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(MetalConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(MetalConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(MetalConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            MetalConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(MetalConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = MetalDepGraph::new();
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
#[cfg(test)]
mod metalext_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_metalext_phase_order() {
        assert_eq!(MetalExtPassPhase::Early.order(), 0);
        assert_eq!(MetalExtPassPhase::Middle.order(), 1);
        assert_eq!(MetalExtPassPhase::Late.order(), 2);
        assert_eq!(MetalExtPassPhase::Finalize.order(), 3);
        assert!(MetalExtPassPhase::Early.is_early());
        assert!(!MetalExtPassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_metalext_config_builder() {
        let c = MetalExtPassConfig::new("p")
            .with_phase(MetalExtPassPhase::Late)
            .with_max_iter(50)
            .with_debug(1);
        assert_eq!(c.name, "p");
        assert_eq!(c.max_iterations, 50);
        assert!(c.is_debug_enabled());
        assert!(c.enabled);
        let c2 = c.disabled();
        assert!(!c2.enabled);
    }
    #[test]
    pub(super) fn test_metalext_stats() {
        let mut s = MetalExtPassStats::new();
        s.visit();
        s.visit();
        s.modify();
        s.iterate();
        assert_eq!(s.nodes_visited, 2);
        assert_eq!(s.nodes_modified, 1);
        assert!(s.changed);
        assert_eq!(s.iterations, 1);
        let e = s.efficiency();
        assert!((e - 0.5).abs() < 1e-9);
    }
    #[test]
    pub(super) fn test_metalext_registry() {
        let mut r = MetalExtPassRegistry::new();
        r.register(MetalExtPassConfig::new("a").with_phase(MetalExtPassPhase::Early));
        r.register(MetalExtPassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&MetalExtPassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_metalext_cache() {
        let mut c = MetalExtCache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_metalext_worklist() {
        let mut w = MetalExtWorklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_metalext_dom_tree() {
        let mut dt = MetalExtDomTree::new(5);
        dt.set_idom(1, 0);
        dt.set_idom(2, 0);
        dt.set_idom(3, 1);
        dt.set_idom(4, 1);
        assert!(dt.dominates(0, 3));
        assert!(dt.dominates(1, 4));
        assert!(!dt.dominates(2, 3));
        assert_eq!(dt.depth_of(3), 2);
    }
    #[test]
    pub(super) fn test_metalext_liveness() {
        let mut lv = MetalExtLiveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_metalext_const_folder() {
        let mut cf = MetalExtConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_metalext_dep_graph() {
        let mut g = MetalExtDepGraph::new(4);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        assert!(!g.has_cycle());
        assert_eq!(g.topo_sort(), Some(vec![0, 1, 2, 3]));
        assert_eq!(g.reachable(0).len(), 4);
        let sccs = g.scc();
        assert_eq!(sccs.len(), 4);
    }
}
