//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::HashMap;

use super::types::{
    ConstantPool, JVMAnalysisCache, JVMConstantFoldingHelper, JVMDepGraph, JVMDominatorTree,
    JVMExtCache, JVMExtConstFolder, JVMExtDepGraph, JVMExtDomTree, JVMExtLiveness,
    JVMExtPassConfig, JVMExtPassPhase, JVMExtPassRegistry, JVMExtPassStats, JVMExtWorklist,
    JVMLivenessInfo, JVMPassConfig, JVMPassPhase, JVMPassRegistry, JVMPassStats, JVMWorklist,
    JvmBackend, JvmClass, JvmCodegenError, JvmField, JvmOpcode, JvmType, MethodDescriptor,
};

/// JVM access flag bit-masks (JVMS Table 4.5-A / 4.6-A).
pub mod access_flags {
    pub const PUBLIC: u16 = 0x0001;
    pub const PRIVATE: u16 = 0x0002;
    pub const PROTECTED: u16 = 0x0004;
    pub const STATIC: u16 = 0x0008;
    pub const FINAL: u16 = 0x0010;
    pub const SUPER: u16 = 0x0020;
    pub const SYNCHRONIZED: u16 = 0x0020;
    pub const VOLATILE: u16 = 0x0040;
    pub const TRANSIENT: u16 = 0x0080;
    pub const NATIVE: u16 = 0x0100;
    pub const INTERFACE: u16 = 0x0200;
    pub const ABSTRACT: u16 = 0x0400;
    pub const STRICT: u16 = 0x0800;
    pub const SYNTHETIC: u16 = 0x1000;
    pub const ANNOTATION: u16 = 0x2000;
    pub const ENUM: u16 = 0x4000;
}
pub type JvmResult<T> = Result<T, JvmCodegenError>;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub(super) fn test_jvm_type_descriptor() {
        assert_eq!(JvmType::Int.descriptor(), "I");
        assert_eq!(JvmType::Long.descriptor(), "J");
        assert_eq!(JvmType::Double.descriptor(), "D");
        assert_eq!(JvmType::Boolean.descriptor(), "Z");
        assert_eq!(JvmType::Void.descriptor(), "V");
        assert_eq!(
            JvmType::Object("java/lang/String".to_string()).descriptor(),
            "Ljava/lang/String;"
        );
        assert_eq!(JvmType::Array(Box::new(JvmType::Int)).descriptor(), "[I");
        assert_eq!(
            JvmType::Generic("T".to_string()).descriptor(),
            "Ljava/lang/Object;"
        );
    }
    #[test]
    pub(super) fn test_jvm_type_slot_size_and_predicates() {
        assert_eq!(JvmType::Int.slot_size(), 1);
        assert_eq!(JvmType::Long.slot_size(), 2);
        assert_eq!(JvmType::Double.slot_size(), 2);
        assert_eq!(JvmType::Object("Foo".to_string()).slot_size(), 1);
        assert!(JvmType::Long.is_wide());
        assert!(!JvmType::Int.is_wide());
        assert!(JvmType::Object("Foo".to_string()).is_reference());
        assert!(JvmType::Int.is_int_category());
        assert!(!JvmType::Long.is_int_category());
    }
    #[test]
    pub(super) fn test_method_descriptor_rendering() {
        let md = MethodDescriptor::new(
            vec![
                JvmType::Int,
                JvmType::Object("java/lang/String".to_string()),
            ],
            JvmType::Void,
        );
        assert_eq!(md.to_string(), "(ILjava/lang/String;)V");
        let md2 = MethodDescriptor::new(vec![], JvmType::Long);
        assert_eq!(md2.to_string(), "()J");
    }
    #[test]
    pub(super) fn test_constant_pool_deduplication() {
        let mut cp = ConstantPool::new();
        let idx1 = cp.utf8("hello");
        let idx2 = cp.utf8("hello");
        let idx3 = cp.utf8("world");
        assert_eq!(idx1, idx2);
        assert_ne!(idx1, idx3);
        assert_eq!(cp.entries().len(), 2);
    }
    #[test]
    pub(super) fn test_jvm_class_construction_and_summary() {
        let mut cls = JvmClass::new("com/example/Foo");
        cls.set_superclass("com/example/Base");
        cls.add_interface("com/example/IFoo");
        cls.add_field(JvmField::new("count", &JvmType::Int, access_flags::PRIVATE));
        let backend = JvmBackend::default_backend();
        let init = backend.emit_default_init("com/example/Base");
        cls.add_method(init);
        assert_eq!(cls.name, "com/example/Foo");
        assert_eq!(cls.superclass, "com/example/Base");
        assert_eq!(cls.interfaces.len(), 1);
        assert_eq!(cls.fields.len(), 1);
        assert_eq!(cls.methods.len(), 1);
        let summary = cls.summary();
        assert!(summary.contains("com/example/Foo"));
        assert!(summary.contains("com/example/Base"));
        assert!(summary.contains("IFoo"));
        assert!(summary.contains("count"));
    }
    #[test]
    pub(super) fn test_emit_binop() {
        let backend = JvmBackend::default_backend();
        let add = backend.emit_binop("+").expect("add emit should succeed");
        assert!(matches!(add.opcode, JvmOpcode::Iadd));
        let mul = backend.emit_binop("mul").expect("mul emit should succeed");
        assert!(matches!(mul.opcode, JvmOpcode::Imul));
        let ladd = backend
            .emit_binop("ladd")
            .expect("ladd emit should succeed");
        assert!(matches!(ladd.opcode, JvmOpcode::Ladd));
        let err = backend.emit_binop("unknown");
        assert!(err.is_err());
    }
    #[test]
    pub(super) fn test_emit_load_store_return() {
        let backend = JvmBackend::default_backend();
        let load_int = backend.emit_load(2, &JvmType::Int);
        assert!(matches!(load_int.opcode, JvmOpcode::Iload(2)));
        let load_ref = backend.emit_load(0, &JvmType::Object("Foo".to_string()));
        assert!(matches!(load_ref.opcode, JvmOpcode::Aload(0)));
        let store_long = backend.emit_store(1, &JvmType::Long);
        assert!(matches!(store_long.opcode, JvmOpcode::Lstore(1)));
        let ret_void = backend.emit_return(&JvmType::Void);
        assert!(matches!(ret_void.opcode, JvmOpcode::Return_));
        let ret_double = backend.emit_return(&JvmType::Double);
        assert!(matches!(ret_double.opcode, JvmOpcode::Dreturn));
        let ret_ref = backend.emit_return(&JvmType::Object("X".to_string()));
        assert!(matches!(ret_ref.opcode, JvmOpcode::Areturn));
    }
    #[test]
    pub(super) fn test_emit_new_default_sequence() {
        let backend = JvmBackend::default_backend();
        let instrs = backend.emit_new_default("com/example/Foo");
        assert_eq!(instrs.len(), 3);
        assert!(matches!(instrs[0].opcode, JvmOpcode::New(_)));
        assert!(matches!(instrs[1].opcode, JvmOpcode::Dup));
        assert!(matches!(instrs[2].opcode, JvmOpcode::Invokespecial { .. }));
    }
    #[test]
    pub(super) fn test_emit_clinit() {
        let backend = JvmBackend::default_backend();
        let clinit = backend.emit_clinit("com/example/Singleton");
        assert_eq!(clinit.name, "<clinit>");
        assert_eq!(clinit.descriptor, "()V");
        assert!(clinit.access_flags & access_flags::STATIC != 0);
        assert!(!clinit.code.is_empty());
        assert!(matches!(
            clinit
                .code
                .last()
                .expect("opcode should be accessible")
                .opcode,
            JvmOpcode::Return_
        ));
    }
    #[test]
    pub(super) fn test_emit_fun_decl() {
        let mut backend = JvmBackend::default_backend();
        let decl = LcnfFunDecl {
            name: "Main_hello".to_string(),
            original_name: None,
            params: vec![],
            ret_type: LcnfType::Nat,
            body: LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(42))),
            is_recursive: false,
            is_lifted: false,
            inline_cost: 1,
        };
        let cls = backend
            .emit_fun_decl(&decl)
            .expect("cls emit should succeed");
        assert!(cls.name.contains("hello"));
        assert!(!cls.methods.is_empty());
        let apply = cls.methods.iter().find(|m| m.name == "apply");
        assert!(apply.is_some());
        let m = apply.expect("m should be Some/Ok");
        assert!(m.access_flags & access_flags::STATIC != 0);
    }
}
#[cfg(test)]
mod JVM_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = JVMPassConfig::new("test_pass", JVMPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = JVMPassStats::new();
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
        let mut reg = JVMPassRegistry::new();
        reg.register(JVMPassConfig::new("pass_a", JVMPassPhase::Analysis));
        reg.register(JVMPassConfig::new("pass_b", JVMPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = JVMAnalysisCache::new(10);
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
        let mut wl = JVMWorklist::new();
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
        let mut dt = JVMDominatorTree::new(5);
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
        let mut liveness = JVMLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(JVMConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(JVMConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(JVMConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            JVMConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(JVMConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = JVMDepGraph::new();
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
mod jvmext_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_jvmext_phase_order() {
        assert_eq!(JVMExtPassPhase::Early.order(), 0);
        assert_eq!(JVMExtPassPhase::Middle.order(), 1);
        assert_eq!(JVMExtPassPhase::Late.order(), 2);
        assert_eq!(JVMExtPassPhase::Finalize.order(), 3);
        assert!(JVMExtPassPhase::Early.is_early());
        assert!(!JVMExtPassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_jvmext_config_builder() {
        let c = JVMExtPassConfig::new("p")
            .with_phase(JVMExtPassPhase::Late)
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
    pub(super) fn test_jvmext_stats() {
        let mut s = JVMExtPassStats::new();
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
    pub(super) fn test_jvmext_registry() {
        let mut r = JVMExtPassRegistry::new();
        r.register(JVMExtPassConfig::new("a").with_phase(JVMExtPassPhase::Early));
        r.register(JVMExtPassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&JVMExtPassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_jvmext_cache() {
        let mut c = JVMExtCache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_jvmext_worklist() {
        let mut w = JVMExtWorklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_jvmext_dom_tree() {
        let mut dt = JVMExtDomTree::new(5);
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
    pub(super) fn test_jvmext_liveness() {
        let mut lv = JVMExtLiveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_jvmext_const_folder() {
        let mut cf = JVMExtConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_jvmext_dep_graph() {
        let mut g = JVMExtDepGraph::new(4);
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
