//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::{HashMap, HashSet};

use super::types::{
    BasicBlock, BlockId, CondCode, NatAnalysisCache, NatConstantFoldingHelper, NatDepGraph,
    NatDominatorTree, NatLivenessInfo, NatPassConfig, NatPassPhase, NatPassRegistry, NatPassStats,
    NatWorklist, NativeBackend, NativeEmitConfig, NativeEmitStats, NativeInst, NativeModule,
    NativeType, NativeValue, Register, RegisterAllocator,
};

/// Map an LCNF type to a native type.
pub(super) fn lcnf_type_to_native(ty: &LcnfType) -> NativeType {
    match ty {
        LcnfType::Nat => NativeType::I64,
        LcnfType::LcnfString => NativeType::Ptr,
        LcnfType::Object => NativeType::Ptr,
        LcnfType::Var(_) => NativeType::Ptr,
        LcnfType::Fun(_, _) => NativeType::Ptr,
        LcnfType::Ctor(_, _) => NativeType::Ptr,
        LcnfType::Erased | LcnfType::Irrelevant | LcnfType::Unit => NativeType::Void,
    }
}
/// Compile an LCNF module to native IR with default settings.
pub fn compile_to_native(module: &LcnfModule) -> NativeModule {
    let mut backend = NativeBackend::default_backend();
    backend.compile_module(module)
}
/// Compile and perform register allocation.
pub fn compile_and_regalloc(
    module: &LcnfModule,
    num_regs: usize,
) -> (NativeModule, Vec<HashMap<Register, Register>>) {
    let mut backend = NativeBackend::default_backend();
    let native_module = backend.compile_module(module);
    let mut allocations = Vec::new();
    for func in &native_module.functions {
        let mut allocator = RegisterAllocator::new(num_regs);
        let alloc = allocator.allocate(func);
        allocations.push(alloc);
    }
    (native_module, allocations)
}
#[cfg(test)]
mod tests {
    use super::*;
    pub(super) fn vid(n: u64) -> LcnfVarId {
        LcnfVarId(n)
    }
    pub(super) fn mk_param(n: u64, name: &str) -> LcnfParam {
        LcnfParam {
            id: vid(n),
            name: name.to_string(),
            ty: LcnfType::Nat,
            erased: false,
            borrowed: false,
        }
    }
    pub(super) fn mk_fun_decl(name: &str, body: LcnfExpr) -> LcnfFunDecl {
        LcnfFunDecl {
            name: name.to_string(),
            original_name: None,
            params: vec![mk_param(0, "n")],
            ret_type: LcnfType::Nat,
            body,
            is_recursive: false,
            is_lifted: false,
            inline_cost: 1,
        }
    }
    pub(super) fn mk_module(decls: Vec<LcnfFunDecl>) -> LcnfModule {
        LcnfModule {
            fun_decls: decls,
            extern_decls: vec![],
            name: "test_mod".to_string(),
            metadata: LcnfModuleMetadata::default(),
        }
    }
    #[test]
    pub(super) fn test_native_type_size() {
        assert_eq!(NativeType::I8.size_bytes(), 1);
        assert_eq!(NativeType::I16.size_bytes(), 2);
        assert_eq!(NativeType::I32.size_bytes(), 4);
        assert_eq!(NativeType::I64.size_bytes(), 8);
        assert_eq!(NativeType::Ptr.size_bytes(), 8);
        assert_eq!(NativeType::Void.size_bytes(), 0);
    }
    #[test]
    pub(super) fn test_native_type_display() {
        assert_eq!(NativeType::I64.to_string(), "i64");
        assert_eq!(NativeType::Ptr.to_string(), "ptr");
        assert_eq!(NativeType::Void.to_string(), "void");
    }
    #[test]
    pub(super) fn test_native_type_properties() {
        assert!(NativeType::I32.is_integer());
        assert!(!NativeType::I32.is_float());
        assert!(NativeType::F64.is_float());
        assert!(NativeType::Ptr.is_pointer());
    }
    #[test]
    pub(super) fn test_register_virtual_physical() {
        let vr = Register::virt(5);
        assert!(vr.is_virtual());
        assert!(!vr.is_physical());
        let pr = Register::phys(3);
        assert!(!pr.is_virtual());
        assert!(pr.is_physical());
    }
    #[test]
    pub(super) fn test_register_display() {
        assert_eq!(Register::virt(5).to_string(), "v5");
        assert_eq!(Register::phys(3).to_string(), "r3");
    }
    #[test]
    pub(super) fn test_native_value_display() {
        assert_eq!(NativeValue::Reg(Register::virt(0)).to_string(), "v0");
        assert_eq!(NativeValue::Imm(42).to_string(), "#42");
        assert_eq!(NativeValue::FRef("foo".to_string()).to_string(), "@foo");
        assert_eq!(NativeValue::StackSlot(3).to_string(), "ss3");
    }
    #[test]
    pub(super) fn test_block_id_display() {
        assert_eq!(BlockId(0).to_string(), "bb0");
        assert_eq!(BlockId(5).to_string(), "bb5");
    }
    #[test]
    pub(super) fn test_basic_block_successors() {
        let mut block = BasicBlock::new(BlockId(0));
        block.push_inst(NativeInst::Br { target: BlockId(1) });
        assert_eq!(block.successors(), vec![BlockId(1)]);
        let mut block2 = BasicBlock::new(BlockId(1));
        block2.push_inst(NativeInst::CondBr {
            cond: NativeValue::Reg(Register::virt(0)),
            then_target: BlockId(2),
            else_target: BlockId(3),
        });
        assert_eq!(block2.successors(), vec![BlockId(2), BlockId(3)]);
    }
    #[test]
    pub(super) fn test_inst_is_terminator() {
        assert!(NativeInst::Br { target: BlockId(0) }.is_terminator());
        assert!(NativeInst::Ret { value: None }.is_terminator());
        assert!(!NativeInst::Nop.is_terminator());
        assert!(!NativeInst::LoadImm {
            dst: Register::virt(0),
            ty: NativeType::I64,
            value: 0
        }
        .is_terminator());
    }
    #[test]
    pub(super) fn test_inst_dst_reg() {
        let inst = NativeInst::Add {
            dst: Register::virt(5),
            ty: NativeType::I64,
            lhs: NativeValue::Reg(Register::virt(0)),
            rhs: NativeValue::Imm(1),
        };
        assert_eq!(inst.dst_reg(), Some(Register::virt(5)));
        let inst2 = NativeInst::Ret { value: None };
        assert_eq!(inst2.dst_reg(), None);
    }
    #[test]
    pub(super) fn test_inst_src_regs() {
        let inst = NativeInst::Add {
            dst: Register::virt(5),
            ty: NativeType::I64,
            lhs: NativeValue::Reg(Register::virt(1)),
            rhs: NativeValue::Reg(Register::virt(2)),
        };
        let srcs = inst.src_regs();
        assert_eq!(srcs.len(), 2);
        assert!(srcs.contains(&Register::virt(1)));
        assert!(srcs.contains(&Register::virt(2)));
    }
    #[test]
    pub(super) fn test_compile_simple_function() {
        let body = LcnfExpr::Return(LcnfArg::Var(vid(0)));
        let decl = mk_fun_decl("identity", body);
        let module = mk_module(vec![decl]);
        let mut backend = NativeBackend::default_backend();
        let native_module = backend.compile_module(&module);
        assert_eq!(native_module.functions.len(), 1);
        let func = &native_module.functions[0];
        assert_eq!(func.name, "identity");
        assert!(!func.blocks.is_empty());
    }
    #[test]
    pub(super) fn test_compile_case_expression() {
        let body = LcnfExpr::Case {
            scrutinee: vid(0),
            scrutinee_ty: LcnfType::Ctor("Bool".into(), vec![]),
            alts: vec![
                LcnfAlt {
                    ctor_name: "False".into(),
                    ctor_tag: 0,
                    params: vec![],
                    body: LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0))),
                },
                LcnfAlt {
                    ctor_name: "True".into(),
                    ctor_tag: 1,
                    params: vec![],
                    body: LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(1))),
                },
            ],
            default: Some(Box::new(LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(99))))),
        };
        let decl = mk_fun_decl("to_nat", body);
        let module = mk_module(vec![decl]);
        let mut backend = NativeBackend::default_backend();
        let native_module = backend.compile_module(&module);
        let func = &native_module.functions[0];
        assert!(func.blocks.len() > 1);
    }
    #[test]
    pub(super) fn test_register_allocation() {
        let body = LcnfExpr::Let {
            id: vid(1),
            name: "a".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Lit(LcnfLit::Nat(42)),
            body: Box::new(LcnfExpr::Let {
                id: vid(2),
                name: "b".to_string(),
                ty: LcnfType::Nat,
                value: LcnfLetValue::Lit(LcnfLit::Nat(10)),
                body: Box::new(LcnfExpr::Return(LcnfArg::Var(vid(1)))),
            }),
        };
        let decl = mk_fun_decl("test_alloc", body);
        let module = mk_module(vec![decl]);
        let (native_module, allocations) = compile_and_regalloc(&module, 8);
        assert_eq!(native_module.functions.len(), 1);
        assert_eq!(allocations.len(), 1);
        let alloc = &allocations[0];
        for (vreg, phys) in alloc {
            assert!(vreg.is_virtual());
            assert!(phys.is_physical());
        }
    }
    #[test]
    pub(super) fn test_native_module_display() {
        let module = NativeModule::new("test");
        let s = module.to_string();
        assert!(s.contains("module: test"));
    }
    #[test]
    pub(super) fn test_native_emit_config_default() {
        let cfg = NativeEmitConfig::default();
        assert_eq!(cfg.opt_level, 1);
        assert!(!cfg.debug_info);
        assert_eq!(cfg.target_arch, "x86_64");
    }
    #[test]
    pub(super) fn test_native_emit_stats_display() {
        let stats = NativeEmitStats {
            functions_compiled: 3,
            blocks_generated: 10,
            ..Default::default()
        };
        let s = stats.to_string();
        assert!(s.contains("fns=3"));
        assert!(s.contains("blocks=10"));
    }
    #[test]
    pub(super) fn test_lcnf_type_to_native() {
        assert_eq!(lcnf_type_to_native(&LcnfType::Nat), NativeType::I64);
        assert_eq!(lcnf_type_to_native(&LcnfType::Object), NativeType::Ptr);
        assert_eq!(lcnf_type_to_native(&LcnfType::Unit), NativeType::Void);
    }
    #[test]
    pub(super) fn test_compile_let_chain() {
        let body = LcnfExpr::Let {
            id: vid(1),
            name: "a".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Lit(LcnfLit::Nat(42)),
            body: Box::new(LcnfExpr::Let {
                id: vid(2),
                name: "b".to_string(),
                ty: LcnfType::Nat,
                value: LcnfLetValue::App(LcnfArg::Var(vid(99)), vec![LcnfArg::Var(vid(1))]),
                body: Box::new(LcnfExpr::Return(LcnfArg::Var(vid(2)))),
            }),
        };
        let decl = mk_fun_decl("chain", body);
        let mut backend = NativeBackend::default_backend();
        let func = backend.compile_fun_decl(&decl);
        assert!(!func.blocks.is_empty());
        assert!(func.instruction_count() > 0);
    }
    #[test]
    pub(super) fn test_virtual_registers() {
        let body = LcnfExpr::Let {
            id: vid(1),
            name: "a".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Lit(LcnfLit::Nat(1)),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(vid(1)))),
        };
        let decl = mk_fun_decl("test", body);
        let mut backend = NativeBackend::default_backend();
        let func = backend.compile_fun_decl(&decl);
        let vregs = func.virtual_registers();
        assert!(!vregs.is_empty());
    }
    #[test]
    pub(super) fn test_cond_code_display() {
        assert_eq!(CondCode::Eq.to_string(), "eq");
        assert_eq!(CondCode::Lt.to_string(), "lt");
        assert_eq!(CondCode::Uge.to_string(), "uge");
    }
    #[test]
    pub(super) fn test_native_func_display() {
        let body = LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0)));
        let decl = mk_fun_decl("display_test", body);
        let mut backend = NativeBackend::default_backend();
        let func = backend.compile_fun_decl(&decl);
        let s = func.to_string();
        assert!(s.contains("func @display_test"));
    }
}
#[cfg(test)]
mod Nat_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = NatPassConfig::new("test_pass", NatPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = NatPassStats::new();
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
        let mut reg = NatPassRegistry::new();
        reg.register(NatPassConfig::new("pass_a", NatPassPhase::Analysis));
        reg.register(NatPassConfig::new("pass_b", NatPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = NatAnalysisCache::new(10);
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
        let mut wl = NatWorklist::new();
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
        let mut dt = NatDominatorTree::new(5);
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
        let mut liveness = NatLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(NatConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(NatConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(NatConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            NatConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(NatConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = NatDepGraph::new();
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
