//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;

use super::types::{
    AffineMap, CmpfPred, CmpiPred, MLIRAnalysisCache, MLIRConstantFoldingHelper, MLIRDepGraph,
    MLIRDominatorTree, MLIRExtCache, MLIRExtConstFolder, MLIRExtDepGraph, MLIRExtDomTree,
    MLIRExtLiveness, MLIRExtPassConfig, MLIRExtPassPhase, MLIRExtPassRegistry, MLIRExtPassStats,
    MLIRExtWorklist, MLIRLivenessInfo, MLIRPassConfig, MLIRPassPhase, MLIRPassRegistry,
    MLIRPassStats, MLIRWorklist, MlirAttr, MlirBackend, MlirBlock, MlirBuilder, MlirDialect,
    MlirFunc, MlirGlobal, MlirModule, MlirRegion, MlirType, MlirValue, SsaCounter,
};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub(super) fn test_mlir_type_integer_display() {
        assert_eq!(MlirType::Integer(64, false).to_string(), "i64");
        assert_eq!(MlirType::Integer(32, false).to_string(), "i32");
        assert_eq!(MlirType::Integer(1, false).to_string(), "i1");
    }
    #[test]
    pub(super) fn test_mlir_type_signed_integer_display() {
        assert_eq!(MlirType::Integer(32, true).to_string(), "si32");
    }
    #[test]
    pub(super) fn test_mlir_type_float_display() {
        assert_eq!(MlirType::Float(32).to_string(), "f32");
        assert_eq!(MlirType::Float(64).to_string(), "f64");
        assert_eq!(MlirType::Float(16).to_string(), "f16");
    }
    #[test]
    pub(super) fn test_mlir_type_index() {
        assert_eq!(MlirType::Index.to_string(), "index");
    }
    #[test]
    pub(super) fn test_mlir_type_none() {
        assert_eq!(MlirType::NoneType.to_string(), "none");
    }
    #[test]
    pub(super) fn test_mlir_type_tensor() {
        let ty = MlirType::Tensor(vec![2, 3], Box::new(MlirType::Float(32)));
        assert_eq!(ty.to_string(), "tensor<2x3xf32>");
    }
    #[test]
    pub(super) fn test_mlir_type_tensor_dynamic() {
        let ty = MlirType::Tensor(vec![-1, 4], Box::new(MlirType::Integer(64, false)));
        assert_eq!(ty.to_string(), "tensor<?x4xi64>");
    }
    #[test]
    pub(super) fn test_mlir_type_vector() {
        let ty = MlirType::Vector(vec![4], Box::new(MlirType::Float(32)));
        assert_eq!(ty.to_string(), "vector<4xf32>");
    }
    #[test]
    pub(super) fn test_mlir_type_memref() {
        let ty = MlirType::MemRef(
            Box::new(MlirType::Float(64)),
            vec![10, 20],
            AffineMap::Constant,
        );
        assert!(ty.to_string().starts_with("memref<10x20xf64"));
    }
    #[test]
    pub(super) fn test_mlir_type_tuple() {
        let ty = MlirType::Tuple(vec![MlirType::Integer(32, false), MlirType::Float(64)]);
        assert_eq!(ty.to_string(), "tuple<i32, f64>");
    }
    #[test]
    pub(super) fn test_mlir_type_func_type() {
        let ty = MlirType::FuncType(
            vec![MlirType::Integer(64, false), MlirType::Integer(64, false)],
            vec![MlirType::Integer(64, false)],
        );
        assert_eq!(ty.to_string(), "(i64, i64) -> i64");
    }
    #[test]
    pub(super) fn test_mlir_value_numbered() {
        let v = MlirValue::numbered(0, MlirType::Integer(64, false));
        assert_eq!(v.to_string(), "%0");
    }
    #[test]
    pub(super) fn test_mlir_value_named() {
        let v = MlirValue::named("arg0", MlirType::Float(32));
        assert_eq!(v.to_string(), "%arg0");
    }
    #[test]
    pub(super) fn test_attr_integer() {
        let attr = MlirAttr::Integer(42, MlirType::Integer(64, false));
        assert_eq!(attr.to_string(), "42 : i64");
    }
    #[test]
    pub(super) fn test_attr_string() {
        let attr = MlirAttr::Str("hello".to_string());
        assert_eq!(attr.to_string(), "\"hello\"");
    }
    #[test]
    pub(super) fn test_attr_string_escaping() {
        let attr = MlirAttr::Str("say \"hi\"".to_string());
        assert!(attr.to_string().contains("\\\""));
    }
    #[test]
    pub(super) fn test_attr_symbol() {
        let attr = MlirAttr::Symbol("add".to_string());
        assert_eq!(attr.to_string(), "@add");
    }
    #[test]
    pub(super) fn test_attr_array() {
        let attr = MlirAttr::Array(vec![
            MlirAttr::Integer(1, MlirType::Integer(32, false)),
            MlirAttr::Integer(2, MlirType::Integer(32, false)),
        ]);
        assert!(attr.to_string().starts_with('['));
        assert!(attr.to_string().ends_with(']'));
    }
    #[test]
    pub(super) fn test_attr_dict() {
        let attr = MlirAttr::Dict(vec![("key".to_string(), MlirAttr::Bool(true))]);
        let s = attr.to_string();
        assert!(s.contains("key"));
        assert!(s.contains("true"));
    }
    #[test]
    pub(super) fn test_builder_const_int() {
        let mut b = MlirBuilder::new();
        let v = b.const_int(42, 64);
        assert_eq!(v.name, "0");
        assert!(matches!(v.ty, MlirType::Integer(64, false)));
        let ops = b.take_ops();
        assert_eq!(ops.len(), 1);
        assert_eq!(ops[0].op_name, "arith.constant");
    }
    #[test]
    pub(super) fn test_builder_const_float() {
        let mut b = MlirBuilder::new();
        let v = b.const_float(3.14, 64);
        assert!(matches!(v.ty, MlirType::Float(64)));
        let ops = b.take_ops();
        assert_eq!(ops[0].op_name, "arith.constant");
    }
    #[test]
    pub(super) fn test_builder_addi() {
        let mut b = MlirBuilder::new();
        let lhs = MlirValue::named("x", MlirType::Integer(64, false));
        let rhs = MlirValue::named("y", MlirType::Integer(64, false));
        let result = b.addi(lhs, rhs);
        assert!(matches!(result.ty, MlirType::Integer(64, false)));
        let ops = b.take_ops();
        assert_eq!(ops[0].op_name, "arith.addi");
    }
    #[test]
    pub(super) fn test_builder_muli() {
        let mut b = MlirBuilder::new();
        let lhs = MlirValue::named("a", MlirType::Integer(32, false));
        let rhs = MlirValue::named("b", MlirType::Integer(32, false));
        let _ = b.muli(lhs, rhs);
        let ops = b.take_ops();
        assert_eq!(ops[0].op_name, "arith.muli");
    }
    #[test]
    pub(super) fn test_builder_cmpi() {
        let mut b = MlirBuilder::new();
        let lhs = MlirValue::named("a", MlirType::Integer(64, false));
        let rhs = MlirValue::named("b", MlirType::Integer(64, false));
        let result = b.cmpi(CmpiPred::Slt, lhs, rhs);
        assert!(matches!(result.ty, MlirType::Integer(1, false)));
        let ops = b.take_ops();
        assert_eq!(ops[0].op_name, "arith.cmpi");
    }
    #[test]
    pub(super) fn test_builder_math_sin() {
        let mut b = MlirBuilder::new();
        let v = MlirValue::named("x", MlirType::Float(64));
        let _ = b.sin(v);
        let ops = b.take_ops();
        assert_eq!(ops[0].op_name, "math.sin");
    }
    #[test]
    pub(super) fn test_builder_return_op() {
        let mut b = MlirBuilder::new();
        let v = MlirValue::named("result", MlirType::Integer(64, false));
        b.return_op(vec![v]);
        let ops = b.take_ops();
        assert_eq!(ops[0].op_name, "func.return");
    }
    #[test]
    pub(super) fn test_builder_call() {
        let mut b = MlirBuilder::new();
        let arg = MlirValue::named("x", MlirType::Integer(64, false));
        let results = b.call("foo", vec![arg], vec![MlirType::Integer(64, false)]);
        assert_eq!(results.len(), 1);
        let ops = b.take_ops();
        assert_eq!(ops[0].op_name, "func.call");
    }
    #[test]
    pub(super) fn test_module_emit_empty() {
        let module = MlirModule::new();
        let output = module.emit();
        assert!(output.starts_with("module {"));
        assert!(output.ends_with("}\n"));
    }
    #[test]
    pub(super) fn test_module_emit_named() {
        let module = MlirModule::named("my_module");
        let output = module.emit();
        assert!(output.starts_with("module @my_module {"));
    }
    #[test]
    pub(super) fn test_module_emit_func() {
        let mut module = MlirModule::new();
        let int_ty = MlirType::Integer(64, false);
        let mut builder = MlirBuilder::new();
        let arg0 = MlirValue::named("arg0", int_ty.clone());
        let arg1 = MlirValue::named("arg1", int_ty.clone());
        let sum = builder.addi(arg0.clone(), arg1.clone());
        builder.return_op(vec![sum]);
        let block = MlirBlock::entry(vec![arg0, arg1], builder.take_ops());
        let func = MlirFunc::new(
            "add",
            vec![
                ("arg0".to_string(), int_ty.clone()),
                ("arg1".to_string(), int_ty.clone()),
            ],
            vec![int_ty],
            MlirRegion::single_block(block),
        );
        module.add_function(func);
        let output = module.emit();
        assert!(output.contains("func.func @add"));
        assert!(output.contains("arith.addi"));
        assert!(output.contains("func.return"));
    }
    #[test]
    pub(super) fn test_backend_compile_add_func() {
        let mut backend = MlirBackend::new();
        backend.compile_add_func("add64", 64);
        let output = backend.emit_module();
        assert!(output.contains("module {"));
        assert!(output.contains("func.func"));
    }
    #[test]
    pub(super) fn test_backend_run_passes_empty() {
        let backend = MlirBackend::new();
        assert_eq!(backend.run_passes(), "");
    }
    #[test]
    pub(super) fn test_backend_run_passes_nonempty() {
        let mut backend = MlirBackend::new();
        backend.add_pass("convert-arith-to-llvm");
        backend.add_pass("convert-func-to-llvm");
        let passes = backend.run_passes();
        assert!(passes.contains("mlir-opt"));
        assert!(passes.contains("convert-arith-to-llvm"));
    }
    #[test]
    pub(super) fn test_mlir_global_emit() {
        let global = MlirGlobal::constant("pi", MlirType::Float(64), MlirAttr::Float(3.14159));
        let output = global.emit();
        assert!(output.contains("@pi"));
        assert!(output.contains("constant"));
    }
    #[test]
    pub(super) fn test_ssa_counter_sequential() {
        let mut counter = SsaCounter::new();
        let v0 = counter.next(MlirType::Integer(64, false));
        let v1 = counter.next(MlirType::Integer(64, false));
        assert_eq!(v0.name, "0");
        assert_eq!(v1.name, "1");
    }
    #[test]
    pub(super) fn test_ssa_counter_reset() {
        let mut counter = SsaCounter::new();
        let _ = counter.next(MlirType::Integer(32, false));
        counter.reset();
        let v = counter.next(MlirType::Integer(32, false));
        assert_eq!(v.name, "0");
    }
    #[test]
    pub(super) fn test_dialect_display() {
        assert_eq!(MlirDialect::Arith.to_string(), "arith");
        assert_eq!(MlirDialect::Func.to_string(), "func");
        assert_eq!(MlirDialect::SCF.to_string(), "scf");
        assert_eq!(MlirDialect::GPU.to_string(), "gpu");
        assert_eq!(MlirDialect::Linalg.to_string(), "linalg");
    }
    #[test]
    pub(super) fn test_cmpi_pred_display() {
        assert_eq!(CmpiPred::Eq.to_string(), "eq");
        assert_eq!(CmpiPred::Slt.to_string(), "slt");
        assert_eq!(CmpiPred::Uge.to_string(), "uge");
    }
    #[test]
    pub(super) fn test_cmpf_pred_display() {
        assert_eq!(CmpfPred::Oeq.to_string(), "oeq");
        assert_eq!(CmpfPred::Une.to_string(), "une");
    }
    #[test]
    pub(super) fn test_builder_extsi() {
        let mut b = MlirBuilder::new();
        let v = MlirValue::named("small", MlirType::Integer(32, false));
        let result = b.extsi(v, 64);
        assert!(matches!(result.ty, MlirType::Integer(64, false)));
        let ops = b.take_ops();
        assert_eq!(ops[0].op_name, "arith.extsi");
    }
    #[test]
    pub(super) fn test_builder_trunci() {
        let mut b = MlirBuilder::new();
        let v = MlirValue::named("big", MlirType::Integer(64, false));
        let result = b.trunci(v, 32);
        assert!(matches!(result.ty, MlirType::Integer(32, false)));
        let ops = b.take_ops();
        assert_eq!(ops[0].op_name, "arith.trunci");
    }
    #[test]
    pub(super) fn test_backend_compile_decl() {
        let mut backend = MlirBackend::new();
        backend.compile_decl(
            "my_func",
            vec![MlirType::Integer(64, false)],
            MlirType::Integer(64, false),
        );
        let output = backend.emit_module();
        assert!(output.contains("my_func"));
    }
    #[test]
    pub(super) fn test_affine_map_identity() {
        let am = AffineMap::Identity(2);
        let s = am.to_string();
        assert!(s.contains("affine_map"));
        assert!(s.contains("d0"));
        assert!(s.contains("d1"));
    }
    #[test]
    pub(super) fn test_mlir_block_labeled() {
        let block = MlirBlock::labeled("bb1", vec![], vec![]);
        let s = format!("{}", block);
        assert!(s.contains("^bb1"));
    }
    #[test]
    pub(super) fn test_mlir_func_declaration() {
        let decl = MlirFunc::declaration(
            "extern_func",
            vec![MlirType::Integer(32, false)],
            vec![MlirType::Integer(32, false)],
        );
        let output = decl.emit();
        assert!(output.contains("extern_func"));
        assert!(output.contains("private"));
    }
    #[test]
    pub(super) fn test_builder_math_ops() {
        let mut b = MlirBuilder::new();
        let v = MlirValue::named("x", MlirType::Float(32));
        let _ = b.cos(v.clone());
        let _ = b.exp(v.clone());
        let _ = b.log(v.clone());
        let _ = b.sqrt(v);
        let ops = b.take_ops();
        assert_eq!(ops[0].op_name, "math.cos");
        assert_eq!(ops[1].op_name, "math.exp");
        assert_eq!(ops[2].op_name, "math.log");
        assert_eq!(ops[3].op_name, "math.sqrt");
    }
    #[test]
    pub(super) fn test_unranked_memref() {
        let ty = MlirType::UnrankedMemRef(Box::new(MlirType::Float(32)));
        assert_eq!(ty.to_string(), "memref<*xf32>");
    }
    #[test]
    pub(super) fn test_complex_type() {
        let ty = MlirType::Complex(Box::new(MlirType::Float(32)));
        assert_eq!(ty.to_string(), "complex<f32>");
    }
}
#[cfg(test)]
mod MLIR_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = MLIRPassConfig::new("test_pass", MLIRPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = MLIRPassStats::new();
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
        let mut reg = MLIRPassRegistry::new();
        reg.register(MLIRPassConfig::new("pass_a", MLIRPassPhase::Analysis));
        reg.register(MLIRPassConfig::new("pass_b", MLIRPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = MLIRAnalysisCache::new(10);
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
        let mut wl = MLIRWorklist::new();
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
        let mut dt = MLIRDominatorTree::new(5);
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
        let mut liveness = MLIRLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(MLIRConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(MLIRConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(MLIRConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            MLIRConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(MLIRConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = MLIRDepGraph::new();
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
mod mlirext_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_mlirext_phase_order() {
        assert_eq!(MLIRExtPassPhase::Early.order(), 0);
        assert_eq!(MLIRExtPassPhase::Middle.order(), 1);
        assert_eq!(MLIRExtPassPhase::Late.order(), 2);
        assert_eq!(MLIRExtPassPhase::Finalize.order(), 3);
        assert!(MLIRExtPassPhase::Early.is_early());
        assert!(!MLIRExtPassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_mlirext_config_builder() {
        let c = MLIRExtPassConfig::new("p")
            .with_phase(MLIRExtPassPhase::Late)
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
    pub(super) fn test_mlirext_stats() {
        let mut s = MLIRExtPassStats::new();
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
    pub(super) fn test_mlirext_registry() {
        let mut r = MLIRExtPassRegistry::new();
        r.register(MLIRExtPassConfig::new("a").with_phase(MLIRExtPassPhase::Early));
        r.register(MLIRExtPassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&MLIRExtPassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_mlirext_cache() {
        let mut c = MLIRExtCache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_mlirext_worklist() {
        let mut w = MLIRExtWorklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_mlirext_dom_tree() {
        let mut dt = MLIRExtDomTree::new(5);
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
    pub(super) fn test_mlirext_liveness() {
        let mut lv = MLIRExtLiveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_mlirext_const_folder() {
        let mut cf = MLIRExtConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_mlirext_dep_graph() {
        let mut g = MLIRExtDepGraph::new(4);
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
