//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;

use super::types::{
    AddressingModel, Decoration, MemoryModel, SPIRVAnalysisCache, SPIRVConstantFoldingHelper,
    SPIRVDepGraph, SPIRVDominatorTree, SPIRVLivenessInfo, SPIRVPassConfig, SPIRVPassPhase,
    SPIRVPassRegistry, SPIRVPassStats, SPIRVWorklist, SpirVBackend, SpirVBasicBlock,
    SpirVCapability, SpirVFunction, SpirVInstruction, SpirVModule, SpirVOp, SpirVType,
    StorageClass,
};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub(super) fn test_spirv_type_display() {
        assert_eq!(SpirVType::Void.to_string(), "void");
        assert_eq!(SpirVType::Bool.to_string(), "bool");
        assert_eq!(
            SpirVType::Int {
                width: 32,
                signed: true
            }
            .to_string(),
            "i32"
        );
        assert_eq!(
            SpirVType::Int {
                width: 64,
                signed: false
            }
            .to_string(),
            "u64"
        );
        assert_eq!(SpirVType::Float { width: 32 }.to_string(), "f32");
        assert_eq!(SpirVType::Sampler.to_string(), "sampler");
    }
    #[test]
    pub(super) fn test_spirv_type_display_compound() {
        let vec4 = SpirVType::Vector {
            element: Box::new(SpirVType::Float { width: 32 }),
            count: 4,
        };
        assert_eq!(vec4.to_string(), "vec4<f32>");
        let mat4 = SpirVType::Matrix {
            column_type: Box::new(SpirVType::Vector {
                element: Box::new(SpirVType::Float { width: 32 }),
                count: 4,
            }),
            column_count: 4,
        };
        assert!(mat4.to_string().contains("mat4x"));
        let arr = SpirVType::Array {
            element: Box::new(SpirVType::Int {
                width: 32,
                signed: true,
            }),
            length: 16,
        };
        assert_eq!(arr.to_string(), "[i32; 16]");
    }
    #[test]
    pub(super) fn test_spirv_instruction_emit_text() {
        let instr = SpirVInstruction::with_result(5, 3, SpirVOp::FAdd, vec![6, 7]);
        let text = instr.emit_text();
        assert!(text.contains("%5 ="));
        assert!(text.contains("OpFAdd"));
        assert!(text.contains("%6"));
        assert!(text.contains("%7"));
    }
    #[test]
    pub(super) fn test_spirv_instruction_no_result() {
        let instr = SpirVInstruction::no_result(SpirVOp::Return, vec![]);
        assert!(instr.result_id.is_none());
        let text = instr.emit_text();
        assert!(text.contains("OpReturn"));
    }
    #[test]
    pub(super) fn test_spirv_instruction_word_count() {
        let instr = SpirVInstruction::with_result(1, 2, SpirVOp::FAdd, vec![3, 4]);
        assert_eq!(instr.word_count(), 5);
        let void_ret = SpirVInstruction::no_result(SpirVOp::Return, vec![]);
        assert_eq!(void_ret.word_count(), 1);
    }
    #[test]
    pub(super) fn test_spirv_basic_block() {
        let mut block = SpirVBasicBlock::new(10);
        block.push(SpirVInstruction::with_result(
            11,
            2,
            SpirVOp::FAdd,
            vec![3, 4],
        ));
        block.push(SpirVInstruction::no_result(SpirVOp::Return, vec![]));
        assert_eq!(block.instr_count(), 2);
        let text = block.emit_text();
        assert!(text.contains("%10 = OpLabel"));
        assert!(text.contains("OpFAdd"));
        assert!(text.contains("OpReturn"));
    }
    #[test]
    pub(super) fn test_spirv_function_emit() {
        let mut func = SpirVFunction::new(1, Some("main".to_string()), 2, 3);
        func.add_param(4, 5);
        let mut block = SpirVBasicBlock::new(6);
        block.push(SpirVInstruction::no_result(SpirVOp::Return, vec![]));
        func.add_block(block);
        let text = func.emit_text();
        assert!(text.contains("OpFunction"));
        assert!(text.contains("OpFunctionParameter"));
        assert!(text.contains("OpFunctionEnd"));
        assert!(text.contains("; main"));
    }
    #[test]
    pub(super) fn test_spirv_module_new() {
        let module = SpirVModule::new();
        assert_eq!(module.bound, 1);
        assert!(module.capabilities.is_empty());
        assert!(module.functions.is_empty());
    }
    #[test]
    pub(super) fn test_spirv_module_emit_text() {
        let mut module = SpirVModule::new();
        module.add_capability(SpirVCapability::Shader);
        module.add_capability(SpirVCapability::Float64);
        module.memory_model = (AddressingModel::Logical, MemoryModel::GLSL450);
        let text = module.emit_text();
        assert!(text.contains("OpCapability Shader"));
        assert!(text.contains("OpCapability Float64"));
        assert!(text.contains("OpMemoryModel Logical GLSL450"));
    }
    #[test]
    pub(super) fn test_spirv_backend_configure_vulkan() {
        let mut backend = SpirVBackend::new();
        backend.configure_for_vulkan();
        assert!(backend
            .module
            .capabilities
            .contains(&SpirVCapability::Shader));
        assert_eq!(backend.module.memory_model.1, MemoryModel::GLSL450);
        assert!(backend.glsl_ext_id.is_some());
    }
    #[test]
    pub(super) fn test_spirv_backend_type_declarations() {
        let mut backend = SpirVBackend::new();
        let f32_id = backend.declare_float_type(32);
        let f32_id2 = backend.declare_float_type(32);
        assert_eq!(f32_id, f32_id2);
        let i32_id = backend.declare_int_type(32, true);
        let u32_id = backend.declare_int_type(32, false);
        assert_ne!(i32_id, u32_id);
        let vec4_id = backend.declare_vector_type(f32_id, 4);
        let vec4_id2 = backend.declare_vector_type(f32_id, 4);
        assert_eq!(vec4_id, vec4_id2);
    }
    #[test]
    pub(super) fn test_spirv_backend_begin_function() {
        let mut backend = SpirVBackend::new();
        let f32_id = backend.declare_float_type(32);
        let func = backend.begin_function("add_f32", f32_id, vec![f32_id, f32_id]);
        assert_eq!(func.params.len(), 2);
        assert_eq!(func.name.as_deref(), Some("add_f32"));
        backend.finish_function(func);
        assert_eq!(backend.function_count(), 1);
        assert!(backend.lookup_symbol("add_f32").is_some());
    }
    #[test]
    pub(super) fn test_spirv_backend_compute_kernel() {
        let mut backend = SpirVBackend::new();
        let func_id = backend.emit_compute_kernel("fill_buffer", 64, 1, 1);
        assert!(func_id > 0);
        assert_eq!(backend.function_count(), 1);
        let text = backend.emit_text();
        assert!(text.contains("fill_buffer"));
        assert!(text.contains("OpEntryPoint"));
        assert!(text.contains("LocalSize"));
    }
    #[test]
    pub(super) fn test_spirv_binary_header() {
        let backend = SpirVBackend::new();
        let header = backend.emit_binary_header();
        assert_eq!(header.len(), 5);
        assert_eq!(header[0], 0x0723_0203);
        assert_eq!(header[4], 0);
    }
    #[test]
    pub(super) fn test_spirv_module_word_count() {
        let mut backend = SpirVBackend::new();
        backend.emit_compute_kernel("test_kernel", 32, 1, 1);
        let wc = backend.module.estimate_word_count();
        assert!(wc >= 5);
    }
    #[test]
    pub(super) fn test_spirv_decoration_and_names() {
        let mut module = SpirVModule::new();
        module.set_name(1, "my_var");
        module.decorate(1, Decoration::Binding(0));
        module.decorate(1, Decoration::DescriptorSet(0));
        assert_eq!(
            module
                .debug_names
                .get(&1)
                .expect("value should be present in map"),
            "my_var"
        );
        assert_eq!(
            module
                .decorations
                .get(&1)
                .expect("value should be present in map")
                .len(),
            2
        );
        let text = module.emit_text();
        assert!(text.contains("OpName %1 \"my_var\""));
        assert!(text.contains("OpDecorate %1 Binding(0)"));
    }
    #[test]
    pub(super) fn test_spirv_op_display() {
        assert_eq!(SpirVOp::FAdd.to_string(), "OpFAdd");
        assert_eq!(SpirVOp::IMul.to_string(), "OpIMul");
        assert_eq!(
            SpirVOp::MatrixTimesVector.to_string(),
            "OpMatrixTimesVector"
        );
        assert_eq!(
            SpirVOp::CompositeConstruct.to_string(),
            "OpCompositeConstruct"
        );
        assert_eq!(SpirVOp::Return.to_string(), "OpReturn");
        assert_eq!(SpirVOp::Load.to_string(), "OpLoad");
    }
    #[test]
    pub(super) fn test_storage_class_display() {
        assert_eq!(StorageClass::Uniform.to_string(), "Uniform");
        assert_eq!(StorageClass::Function.to_string(), "Function");
        assert_eq!(StorageClass::Workgroup.to_string(), "Workgroup");
        assert_eq!(StorageClass::Input.to_string(), "Input");
        assert_eq!(StorageClass::Output.to_string(), "Output");
    }
}
#[cfg(test)]
mod SPIRV_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = SPIRVPassConfig::new("test_pass", SPIRVPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = SPIRVPassStats::new();
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
        let mut reg = SPIRVPassRegistry::new();
        reg.register(SPIRVPassConfig::new("pass_a", SPIRVPassPhase::Analysis));
        reg.register(SPIRVPassConfig::new("pass_b", SPIRVPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = SPIRVAnalysisCache::new(10);
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
        let mut wl = SPIRVWorklist::new();
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
        let mut dt = SPIRVDominatorTree::new(5);
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
        let mut liveness = SPIRVLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(SPIRVConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(SPIRVConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(SPIRVConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            SPIRVConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(SPIRVConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = SPIRVDepGraph::new();
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
