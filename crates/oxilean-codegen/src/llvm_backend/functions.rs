//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::{
    FcmpPred, IcmpPred, LLVMAnalysisCache, LLVMConstantFoldingHelper, LLVMDepGraph,
    LLVMDominatorTree, LLVMExtCache, LLVMExtConstFolder, LLVMExtDepGraph, LLVMExtDomTree,
    LLVMExtLiveness, LLVMExtPassConfig, LLVMExtPassPhase, LLVMExtPassRegistry, LLVMExtPassStats,
    LLVMExtWorklist, LLVMLivenessInfo, LLVMPassConfig, LLVMPassPhase, LLVMPassRegistry,
    LLVMPassStats, LLVMWorklist, LlvmAttr, LlvmBackend, LlvmFunc, LlvmInstr, LlvmLinkage,
    LlvmModule, LlvmType, LlvmValue,
};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub(super) fn test_llvm_type_display() {
        assert_eq!(LlvmType::I32.to_string(), "i32");
        assert_eq!(LlvmType::Ptr.to_string(), "ptr");
        assert_eq!(
            LlvmType::Array(4, Box::new(LlvmType::I32)).to_string(),
            "[4 x i32]"
        );
        assert_eq!(
            LlvmType::Vector(8, Box::new(LlvmType::F32)).to_string(),
            "<8 x float>"
        );
        assert_eq!(
            LlvmType::Struct(vec![LlvmType::I64, LlvmType::Ptr]).to_string(),
            "{ i64, ptr }"
        );
        assert_eq!(
            LlvmType::FuncType {
                ret: Box::new(LlvmType::I32),
                params: vec![LlvmType::I64, LlvmType::Ptr],
                variadic: false,
            }
            .to_string(),
            "i32 (i64, ptr)"
        );
        assert_eq!(
            LlvmType::FuncType {
                ret: Box::new(LlvmType::I32),
                params: vec![LlvmType::I8],
                variadic: true,
            }
            .to_string(),
            "i32 (i8, ...)"
        );
        assert_eq!(
            LlvmType::Named("MyStruct".to_string()).to_string(),
            "%MyStruct"
        );
    }
    #[test]
    pub(super) fn test_llvm_value_display() {
        assert_eq!(LlvmValue::Const(42).to_string(), "42");
        assert_eq!(LlvmValue::Undef.to_string(), "undef");
        assert_eq!(LlvmValue::Null.to_string(), "null");
        assert_eq!(LlvmValue::True_.to_string(), "true");
        assert_eq!(LlvmValue::False_.to_string(), "false");
        assert_eq!(
            LlvmValue::GlobalRef("printf".to_string()).to_string(),
            "@printf"
        );
        assert_eq!(LlvmValue::LocalRef("x0".to_string()).to_string(), "%x0");
        assert_eq!(LlvmValue::ZeroInitializer.to_string(), "zeroinitializer");
        assert_eq!(
            LlvmValue::ConstArray(
                LlvmType::I32,
                vec![LlvmValue::Const(1), LlvmValue::Const(2)]
            )
            .to_string(),
            "[i32 1, i32 2]"
        );
    }
    #[test]
    pub(super) fn test_icmp_pred_display() {
        assert_eq!(IcmpPred::Eq.to_string(), "eq");
        assert_eq!(IcmpPred::Ne.to_string(), "ne");
        assert_eq!(IcmpPred::Slt.to_string(), "slt");
        assert_eq!(IcmpPred::Sgt.to_string(), "sgt");
        assert_eq!(IcmpPred::Sle.to_string(), "sle");
        assert_eq!(IcmpPred::Sge.to_string(), "sge");
        assert_eq!(IcmpPred::Ult.to_string(), "ult");
        assert_eq!(IcmpPred::Ugt.to_string(), "ugt");
        assert_eq!(IcmpPred::Ule.to_string(), "ule");
        assert_eq!(IcmpPred::Uge.to_string(), "uge");
        assert_eq!(FcmpPred::Oeq.to_string(), "oeq");
        assert_eq!(FcmpPred::Uno.to_string(), "uno");
        assert_eq!(FcmpPred::True_.to_string(), "true");
        assert_eq!(FcmpPred::False_.to_string(), "false");
    }
    #[test]
    pub(super) fn test_llvm_instr_add_display() {
        let instr = LlvmInstr::Add {
            result: "r0".to_string(),
            lhs: LlvmValue::LocalRef("x0".to_string()),
            rhs: LlvmValue::LocalRef("x1".to_string()),
        };
        assert_eq!(instr.to_string(), "  %r0 = add i64 %x0, %x1");
        let instr2 = LlvmInstr::ICmp {
            result: "cmp".to_string(),
            pred: IcmpPred::Slt,
            lhs: LlvmValue::LocalRef("a".to_string()),
            rhs: LlvmValue::Const(0),
        };
        assert_eq!(instr2.to_string(), "  %cmp = icmp slt i64 %a, 0");
        let instr3 = LlvmInstr::Ret(Some((LlvmType::I64, LlvmValue::Const(0))));
        assert_eq!(instr3.to_string(), "  ret i64 0");
        let instr4 = LlvmInstr::Ret(None);
        assert_eq!(instr4.to_string(), "  ret void");
    }
    #[test]
    pub(super) fn test_llvm_func_display() {
        let func = LlvmFunc {
            name: "add_two".to_string(),
            ret_ty: LlvmType::I64,
            params: vec![
                (LlvmType::I64, "a".to_string()),
                (LlvmType::I64, "b".to_string()),
            ],
            body: vec![
                LlvmInstr::Add {
                    result: "result".to_string(),
                    lhs: LlvmValue::LocalRef("a".to_string()),
                    rhs: LlvmValue::LocalRef("b".to_string()),
                },
                LlvmInstr::Ret(Some((
                    LlvmType::I64,
                    LlvmValue::LocalRef("result".to_string()),
                ))),
            ],
            linkage: LlvmLinkage::External,
            attrs: vec![LlvmAttr::NoUnwind],
            is_declare: false,
        };
        let text = func.to_string();
        assert!(text.contains("define"));
        assert!(text.contains("@add_two"));
        assert!(text.contains("i64 %a"));
        assert!(text.contains("i64 %b"));
        assert!(text.contains("%result = add i64 %a, %b"));
        assert!(text.contains("ret i64 %result"));
        assert!(text.contains("nounwind"));
    }
    #[test]
    pub(super) fn test_llvm_module_emit() {
        let func = LlvmFunc {
            name: "main_func".to_string(),
            ret_ty: LlvmType::I64,
            params: vec![],
            body: vec![LlvmInstr::Ret(Some((LlvmType::I64, LlvmValue::Const(42))))],
            linkage: LlvmLinkage::External,
            attrs: vec![],
            is_declare: false,
        };
        let module = LlvmModule {
            source_filename: "test.ll".to_string(),
            target_triple: "x86_64-unknown-linux-gnu".to_string(),
            data_layout: String::new(),
            type_aliases: vec![],
            globals: vec![],
            functions: vec![func],
            metadata: vec![],
        };
        let text = module.emit();
        assert!(text.contains("source_filename = \"test.ll\""));
        assert!(text.contains("target triple = \"x86_64-unknown-linux-gnu\""));
        assert!(text.contains("define i64 @main_func()"));
        assert!(text.contains("ret i64 42"));
    }
    #[test]
    pub(super) fn test_mangle_name() {
        assert_eq!(LlvmBackend::mangle_name("Nat.add"), "Nat_add");
        assert_eq!(LlvmBackend::mangle_name("foo bar"), "foo_bar");
        assert_eq!(LlvmBackend::mangle_name("hello_world"), "hello_world");
        assert_eq!(LlvmBackend::mangle_name("a.b.c"), "a_b_c");
        assert_eq!(LlvmBackend::mangle_name("my-func"), "my_func");
    }
    #[test]
    pub(super) fn test_llvm_type_for_lcnf() {
        assert_eq!(LlvmBackend::llvm_type_for(&LcnfType::Nat), LlvmType::I64);
        assert_eq!(LlvmBackend::llvm_type_for(&LcnfType::Object), LlvmType::Ptr);
        assert_eq!(
            LlvmBackend::llvm_type_for(&LcnfType::LcnfString),
            LlvmType::Ptr
        );
        assert_eq!(LlvmBackend::llvm_type_for(&LcnfType::Erased), LlvmType::I64);
        assert_eq!(LlvmBackend::llvm_type_for(&LcnfType::Unit), LlvmType::I64);
        assert_eq!(
            LlvmBackend::llvm_type_for(&LcnfType::Fun(
                vec![LcnfType::Nat],
                Box::new(LcnfType::Nat)
            )),
            LlvmType::Ptr
        );
    }
    #[test]
    pub(super) fn test_compile_decl_simple() {
        let mut backend = LlvmBackend::new();
        let decl = LcnfFunDecl {
            name: "answer".to_string(),
            original_name: None,
            params: vec![],
            ret_type: LcnfType::Nat,
            body: LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(42))),
            is_recursive: false,
            is_lifted: false,
            inline_cost: 1,
        };
        let func = backend.compile_decl(&decl);
        assert_eq!(func.name, "answer");
        assert_eq!(func.ret_ty, LlvmType::I64);
        assert!(func.params.is_empty());
        assert!(!func.body.is_empty());
        let text = func.to_string();
        assert!(text.contains("ret i64 42"));
    }
    #[test]
    pub(super) fn test_llvm_backend_emit_module() {
        let mut backend = LlvmBackend::new();
        let decl = LcnfFunDecl {
            name: "Nat.succ".to_string(),
            original_name: None,
            params: vec![LcnfParam {
                id: LcnfVarId(0),
                name: "n".to_string(),
                ty: LcnfType::Nat,
                erased: false,
                borrowed: false,
            }],
            ret_type: LcnfType::Nat,
            body: LcnfExpr::Return(LcnfArg::Var(LcnfVarId(0))),
            is_recursive: false,
            is_lifted: false,
            inline_cost: 1,
        };
        let result = backend.emit_module(&[decl]);
        assert!(result.is_ok());
        let text = result.expect("text should be Some/Ok");
        assert!(text.contains("define"));
        assert!(text.contains("Nat_succ"));
        assert!(text.contains("i64"));
        assert!(text.contains("ret"));
    }
}
#[cfg(test)]
mod LLVM_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = LLVMPassConfig::new("test_pass", LLVMPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = LLVMPassStats::new();
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
        let mut reg = LLVMPassRegistry::new();
        reg.register(LLVMPassConfig::new("pass_a", LLVMPassPhase::Analysis));
        reg.register(LLVMPassConfig::new("pass_b", LLVMPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = LLVMAnalysisCache::new(10);
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
        let mut wl = LLVMWorklist::new();
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
        let mut dt = LLVMDominatorTree::new(5);
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
        let mut liveness = LLVMLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(LLVMConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(LLVMConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(LLVMConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            LLVMConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(LLVMConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = LLVMDepGraph::new();
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
mod llvmext_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_llvmext_phase_order() {
        assert_eq!(LLVMExtPassPhase::Early.order(), 0);
        assert_eq!(LLVMExtPassPhase::Middle.order(), 1);
        assert_eq!(LLVMExtPassPhase::Late.order(), 2);
        assert_eq!(LLVMExtPassPhase::Finalize.order(), 3);
        assert!(LLVMExtPassPhase::Early.is_early());
        assert!(!LLVMExtPassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_llvmext_config_builder() {
        let c = LLVMExtPassConfig::new("p")
            .with_phase(LLVMExtPassPhase::Late)
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
    pub(super) fn test_llvmext_stats() {
        let mut s = LLVMExtPassStats::new();
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
    pub(super) fn test_llvmext_registry() {
        let mut r = LLVMExtPassRegistry::new();
        r.register(LLVMExtPassConfig::new("a").with_phase(LLVMExtPassPhase::Early));
        r.register(LLVMExtPassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&LLVMExtPassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_llvmext_cache() {
        let mut c = LLVMExtCache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_llvmext_worklist() {
        let mut w = LLVMExtWorklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_llvmext_dom_tree() {
        let mut dt = LLVMExtDomTree::new(5);
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
    pub(super) fn test_llvmext_liveness() {
        let mut lv = LLVMExtLiveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_llvmext_const_folder() {
        let mut cf = LLVMExtConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_llvmext_dep_graph() {
        let mut g = LLVMExtDepGraph::new(4);
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
