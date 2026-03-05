//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    CUDAAnalysisCache, CUDAConstantFoldingHelper, CUDADepGraph, CUDADominatorTree, CUDAExtCache,
    CUDAExtConstFolder, CUDAExtDepGraph, CUDAExtDomTree, CUDAExtLiveness, CUDAExtPassConfig,
    CUDAExtPassPhase, CUDAExtPassRegistry, CUDAExtPassStats, CUDAExtWorklist, CUDALivenessInfo,
    CUDAPassConfig, CUDAPassPhase, CUDAPassRegistry, CUDAPassStats, CUDAWorklist, CudaBackend,
    CudaBinOp, CudaExpr, CudaKernel, CudaModule, CudaParam, CudaQualifier, CudaStmt, CudaType,
    DeviceFunction, LaunchBounds, LaunchConfig, MemcpyKind,
};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub(super) fn test_cuda_type_display() {
        assert_eq!(format!("{}", CudaType::Int), "int");
        assert_eq!(format!("{}", CudaType::Float), "float");
        assert_eq!(format!("{}", CudaType::Double), "double");
        assert_eq!(format!("{}", CudaType::Half), "__half");
        assert_eq!(format!("{}", CudaType::Dim3), "dim3");
        assert_eq!(format!("{}", CudaType::CudaErrorT), "cudaError_t");
        assert_eq!(
            format!("{}", CudaType::Pointer(Box::new(CudaType::Float))),
            "float*"
        );
        assert_eq!(
            format!("{}", CudaType::Shared(Box::new(CudaType::Float))),
            "__shared__ float"
        );
        assert_eq!(
            format!("{}", CudaType::Constant(Box::new(CudaType::Int))),
            "__constant__ int"
        );
        assert_eq!(
            format!("{}", CudaType::Device(Box::new(CudaType::Double))),
            "__device__ double"
        );
    }
    #[test]
    pub(super) fn test_cuda_qualifier_display() {
        assert_eq!(format!("{}", CudaQualifier::Global), "__global__");
        assert_eq!(format!("{}", CudaQualifier::Device), "__device__");
        assert_eq!(format!("{}", CudaQualifier::Host), "__host__");
        assert_eq!(format!("{}", CudaQualifier::Shared), "__shared__");
        assert_eq!(format!("{}", CudaQualifier::Constant), "__constant__");
        assert_eq!(format!("{}", CudaQualifier::Managed), "__managed__");
        assert_eq!(format!("{}", CudaQualifier::Restrict), "__restrict__");
        assert_eq!(format!("{}", CudaQualifier::Volatile), "volatile");
    }
    #[test]
    pub(super) fn test_cuda_expr_emit() {
        let backend = CudaBackend::new();
        assert_eq!(backend.emit_expr(&CudaExpr::ThreadIdx('x')), "threadIdx.x");
        assert_eq!(backend.emit_expr(&CudaExpr::BlockIdx('y')), "blockIdx.y");
        assert_eq!(backend.emit_expr(&CudaExpr::BlockDim('z')), "blockDim.z");
        assert_eq!(backend.emit_expr(&CudaExpr::GridDim('x')), "gridDim.x");
        assert_eq!(backend.emit_expr(&CudaExpr::SyncThreads), "__syncthreads()");
        assert_eq!(backend.emit_expr(&CudaExpr::WarpSize), "warpSize");
        let add = CudaExpr::BinOp(
            Box::new(CudaExpr::Var("a".into())),
            CudaBinOp::Add,
            Box::new(CudaExpr::Var("b".into())),
        );
        assert_eq!(backend.emit_expr(&add), "(a + b)");
        let atomic = CudaExpr::AtomicAdd(
            Box::new(CudaExpr::Var("ptr".into())),
            Box::new(CudaExpr::LitInt(1)),
        );
        assert_eq!(backend.emit_expr(&atomic), "atomicAdd(ptr, 1)");
        let cast = CudaExpr::Cast(CudaType::Float, Box::new(CudaExpr::Var("n".into())));
        assert_eq!(backend.emit_expr(&cast), "((float)n)");
    }
    #[test]
    pub(super) fn test_cuda_stmt_emit() {
        let backend = CudaBackend::new();
        let decl = CudaStmt::VarDecl {
            ty: CudaType::Int,
            name: "idx".into(),
            init: Some(CudaExpr::LitInt(0)),
        };
        assert_eq!(backend.emit_stmt(&decl, 0), "int idx = 0;");
        let malloc = CudaStmt::CudaMalloc {
            ptr: "d_data".into(),
            size: CudaExpr::LitInt(1024),
        };
        let s = backend.emit_stmt(&malloc, 0);
        assert!(s.contains("cudaMalloc"));
        assert!(s.contains("(void**)&d_data"));
        assert!(s.contains("1024"));
        let memcpy = CudaStmt::CudaMemcpy {
            dst: CudaExpr::Var("d_data".into()),
            src: CudaExpr::Var("h_data".into()),
            size: CudaExpr::LitInt(1024),
            kind: MemcpyKind::HostToDevice,
        };
        let s = backend.emit_stmt(&memcpy, 0);
        assert!(s.contains("cudaMemcpy"));
        assert!(s.contains("cudaMemcpyHostToDevice"));
        let ret = CudaStmt::Return(Some(CudaExpr::LitInt(0)));
        assert_eq!(backend.emit_stmt(&ret, 0), "return 0;");
        let sync = CudaStmt::DeviceSync;
        assert!(backend
            .emit_stmt(&sync, 0)
            .contains("cudaDeviceSynchronize"));
    }
    #[test]
    pub(super) fn test_kernel_emit() {
        let backend = CudaBackend::new();
        let idx_decl = CudaStmt::VarDecl {
            ty: CudaType::Int,
            name: "idx".into(),
            init: Some(CudaExpr::BinOp(
                Box::new(CudaExpr::BinOp(
                    Box::new(CudaExpr::BlockIdx('x')),
                    CudaBinOp::Mul,
                    Box::new(CudaExpr::BlockDim('x')),
                )),
                CudaBinOp::Add,
                Box::new(CudaExpr::ThreadIdx('x')),
            )),
        };
        let guard = CudaStmt::IfElse {
            cond: CudaExpr::BinOp(
                Box::new(CudaExpr::Var("idx".into())),
                CudaBinOp::Lt,
                Box::new(CudaExpr::Var("n".into())),
            ),
            then_body: vec![CudaStmt::Assign {
                lhs: CudaExpr::Index(
                    Box::new(CudaExpr::Var("c".into())),
                    Box::new(CudaExpr::Var("idx".into())),
                ),
                rhs: CudaExpr::BinOp(
                    Box::new(CudaExpr::Index(
                        Box::new(CudaExpr::Var("a".into())),
                        Box::new(CudaExpr::Var("idx".into())),
                    )),
                    CudaBinOp::Add,
                    Box::new(CudaExpr::Index(
                        Box::new(CudaExpr::Var("b".into())),
                        Box::new(CudaExpr::Var("idx".into())),
                    )),
                ),
            }],
            else_body: None,
        };
        let kernel = CudaKernel::new("vec_add")
            .add_param(CudaParam::new(
                CudaType::Pointer(Box::new(CudaType::Float)),
                "a",
            ))
            .add_param(CudaParam::new(
                CudaType::Pointer(Box::new(CudaType::Float)),
                "b",
            ))
            .add_param(CudaParam::new(
                CudaType::Pointer(Box::new(CudaType::Float)),
                "c",
            ))
            .add_param(CudaParam::new(CudaType::Int, "n"))
            .add_stmt(idx_decl)
            .add_stmt(guard)
            .with_launch_bounds(LaunchBounds::new(256));
        let cu = backend.emit_kernel(&kernel);
        assert!(cu.contains("__global__"));
        assert!(cu.contains("vec_add"));
        assert!(cu.contains("__launch_bounds__(256)"));
        assert!(cu.contains("threadIdx.x"));
        assert!(cu.contains("blockIdx.x"));
        assert!(cu.contains("blockDim.x"));
    }
    #[test]
    pub(super) fn test_kernel_launch_stmt() {
        let backend = CudaBackend::new();
        let config =
            LaunchConfig::simple_1d(CudaExpr::Var("grid".into()), CudaExpr::Var("block".into()));
        let launch = CudaStmt::KernelLaunch {
            name: "my_kernel".into(),
            config,
            args: vec![
                CudaExpr::Var("d_a".into()),
                CudaExpr::Var("d_b".into()),
                CudaExpr::LitInt(1024),
            ],
        };
        let s = backend.emit_stmt(&launch, 0);
        assert!(s.contains("my_kernel<<<"));
        assert!(s.contains("grid"));
        assert!(s.contains("block"));
        assert!(s.contains("d_a"));
        assert!(s.contains("d_b"));
        assert!(s.contains("1024"));
    }
    #[test]
    pub(super) fn test_module_emit() {
        let backend = CudaBackend::new();
        let module = CudaModule::new()
            .add_constant(CudaType::Int, "BLOCK_SIZE", Some(CudaExpr::LitInt(256)))
            .add_kernel(CudaKernel::new("dummy_kernel").add_stmt(CudaStmt::Return(None)));
        let cu = backend.emit_module(&module);
        assert!(cu.contains("#include <cuda_runtime.h>"));
        assert!(cu.contains("__constant__ int BLOCK_SIZE = 256;"));
        assert!(cu.contains("__global__"));
        assert!(cu.contains("dummy_kernel"));
        assert!(cu.contains("CUDA_CHECK"));
    }
    #[test]
    pub(super) fn test_device_function_and_warp_intrinsics() {
        let backend = CudaBackend::new();
        let shfl = CudaExpr::ShflDownSync(
            Box::new(CudaExpr::LitInt(0xffffffff)),
            Box::new(CudaExpr::Var("val".into())),
            Box::new(CudaExpr::LitInt(16)),
        );
        let f = DeviceFunction::host_device("warp_reduce_sum", CudaType::Float)
            .with_inline()
            .add_param(CudaParam::new(CudaType::Float, "val"))
            .add_stmt(CudaStmt::Expr(shfl))
            .add_stmt(CudaStmt::Return(Some(CudaExpr::Var("val".into()))));
        let src = backend.emit_device_function(&f);
        assert!(src.contains("__host__"));
        assert!(src.contains("__device__"));
        assert!(src.contains("inline"));
        assert!(src.contains("warp_reduce_sum"));
        assert!(src.contains("__shfl_down_sync"));
        let ballot = CudaExpr::BallotSync(
            Box::new(CudaExpr::LitInt(0xffffffff)),
            Box::new(CudaExpr::Var("pred".into())),
        );
        assert!(backend.emit_expr(&ballot).contains("__ballot_sync"));
        let popc = CudaExpr::Popc(Box::new(CudaExpr::Var("mask".into())));
        assert!(backend.emit_expr(&popc).contains("__popc"));
    }
}
#[cfg(test)]
mod CUDA_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = CUDAPassConfig::new("test_pass", CUDAPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = CUDAPassStats::new();
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
        let mut reg = CUDAPassRegistry::new();
        reg.register(CUDAPassConfig::new("pass_a", CUDAPassPhase::Analysis));
        reg.register(CUDAPassConfig::new("pass_b", CUDAPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = CUDAAnalysisCache::new(10);
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
        let mut wl = CUDAWorklist::new();
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
        let mut dt = CUDADominatorTree::new(5);
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
        let mut liveness = CUDALivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(CUDAConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(CUDAConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(CUDAConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            CUDAConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(CUDAConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = CUDADepGraph::new();
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
mod cudaext_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_cudaext_phase_order() {
        assert_eq!(CUDAExtPassPhase::Early.order(), 0);
        assert_eq!(CUDAExtPassPhase::Middle.order(), 1);
        assert_eq!(CUDAExtPassPhase::Late.order(), 2);
        assert_eq!(CUDAExtPassPhase::Finalize.order(), 3);
        assert!(CUDAExtPassPhase::Early.is_early());
        assert!(!CUDAExtPassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_cudaext_config_builder() {
        let c = CUDAExtPassConfig::new("p")
            .with_phase(CUDAExtPassPhase::Late)
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
    pub(super) fn test_cudaext_stats() {
        let mut s = CUDAExtPassStats::new();
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
    pub(super) fn test_cudaext_registry() {
        let mut r = CUDAExtPassRegistry::new();
        r.register(CUDAExtPassConfig::new("a").with_phase(CUDAExtPassPhase::Early));
        r.register(CUDAExtPassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&CUDAExtPassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_cudaext_cache() {
        let mut c = CUDAExtCache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_cudaext_worklist() {
        let mut w = CUDAExtWorklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_cudaext_dom_tree() {
        let mut dt = CUDAExtDomTree::new(5);
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
    pub(super) fn test_cudaext_liveness() {
        let mut lv = CUDAExtLiveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_cudaext_const_folder() {
        let mut cf = CUDAExtConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_cudaext_dep_graph() {
        let mut g = CUDAExtDepGraph::new(4);
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
