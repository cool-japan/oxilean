//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    MemOp, X86AnalysisCache, X86Backend, X86ConstantFoldingHelper, X86DepGraph, X86DominatorTree,
    X86ExtCache, X86ExtConstFolder, X86ExtDepGraph, X86ExtDomTree, X86ExtLiveness,
    X86ExtPassConfig, X86ExtPassPhase, X86ExtPassRegistry, X86ExtPassStats, X86ExtWorklist,
    X86Function, X86Instr, X86LivenessInfo, X86PassConfig, X86PassPhase, X86PassRegistry,
    X86PassStats, X86Reg, X86Worklist, X86X2Cache, X86X2ConstFolder, X86X2DepGraph, X86X2DomTree,
    X86X2Liveness, X86X2PassConfig, X86X2PassPhase, X86X2PassRegistry, X86X2PassStats,
    X86X2Worklist,
};

#[cfg(test)]
mod tests {
    use super::*;
    pub(super) fn be() -> X86Backend {
        X86Backend::new()
    }
    #[test]
    pub(super) fn test_reg_rax_att() {
        assert_eq!(X86Reg::RAX.name_att(), "%rax");
    }
    #[test]
    pub(super) fn test_reg_rsp_att() {
        assert_eq!(X86Reg::RSP.name_att(), "%rsp");
    }
    #[test]
    pub(super) fn test_reg_rbp_att() {
        assert_eq!(X86Reg::RBP.name_att(), "%rbp");
    }
    #[test]
    pub(super) fn test_reg_r15_att() {
        assert_eq!(X86Reg::R15.name_att(), "%r15");
    }
    #[test]
    pub(super) fn test_reg_xmm0_att() {
        assert_eq!(X86Reg::XMM0.name_att(), "%xmm0");
    }
    #[test]
    pub(super) fn test_reg_rax_intel() {
        assert_eq!(X86Reg::RAX.name_intel(), "rax");
    }
    #[test]
    pub(super) fn test_reg_r8d_intel() {
        assert_eq!(X86Reg::R8D.name_intel(), "r8d");
    }
    #[test]
    pub(super) fn test_emit_mov_reg_reg() {
        let s = be().emit_instr(&X86Instr::Mov(X86Reg::RAX, X86Reg::RBX));
        assert!(s.contains("movq"));
        assert!(s.contains("%rax"));
        assert!(s.contains("%rbx"));
    }
    #[test]
    pub(super) fn test_emit_mov_imm() {
        let s = be().emit_instr(&X86Instr::MovImm(X86Reg::RAX, 42));
        assert!(s.contains("movq"));
        assert!(s.contains("$42"));
        assert!(s.contains("%rax"));
    }
    #[test]
    pub(super) fn test_emit_push() {
        let s = be().emit_instr(&X86Instr::Push(X86Reg::RBP));
        assert!(s.contains("pushq"));
        assert!(s.contains("%rbp"));
    }
    #[test]
    pub(super) fn test_emit_pop() {
        let s = be().emit_instr(&X86Instr::Pop(X86Reg::RBP));
        assert!(s.contains("popq"));
        assert!(s.contains("%rbp"));
    }
    #[test]
    pub(super) fn test_emit_add() {
        let s = be().emit_instr(&X86Instr::Add(X86Reg::RAX, X86Reg::RBX));
        assert!(s.contains("addq"));
        assert!(s.contains("%rax"));
        assert!(s.contains("%rbx"));
    }
    #[test]
    pub(super) fn test_emit_add_imm() {
        let s = be().emit_instr(&X86Instr::AddImm(X86Reg::RSP, -16));
        assert!(s.contains("addq"));
        assert!(s.contains("$-16"));
        assert!(s.contains("%rsp"));
    }
    #[test]
    pub(super) fn test_emit_sub() {
        let s = be().emit_instr(&X86Instr::Sub(X86Reg::RSP, X86Reg::RAX));
        assert!(s.contains("subq"));
    }
    #[test]
    pub(super) fn test_emit_sub_imm() {
        let s = be().emit_instr(&X86Instr::SubImm(X86Reg::RSP, 32));
        assert!(s.contains("subq"));
        assert!(s.contains("$32"));
    }
    #[test]
    pub(super) fn test_emit_imul() {
        let s = be().emit_instr(&X86Instr::IMul(X86Reg::RAX, X86Reg::RCX));
        assert!(s.contains("imulq"));
        assert!(s.contains("%rax"));
        assert!(s.contains("%rcx"));
    }
    #[test]
    pub(super) fn test_emit_and() {
        let s = be().emit_instr(&X86Instr::And(X86Reg::RAX, X86Reg::RBX));
        assert!(s.contains("andq"));
    }
    #[test]
    pub(super) fn test_emit_or() {
        let s = be().emit_instr(&X86Instr::Or(X86Reg::RAX, X86Reg::RBX));
        assert!(s.contains("orq"));
    }
    #[test]
    pub(super) fn test_emit_xor() {
        let s = be().emit_instr(&X86Instr::Xor(X86Reg::RAX, X86Reg::RAX));
        assert!(s.contains("xorq"));
        assert_eq!(s.matches("%rax").count(), 2);
    }
    #[test]
    pub(super) fn test_emit_xor_imm() {
        let s = be().emit_instr(&X86Instr::XorImm(X86Reg::RAX, -1));
        assert!(s.contains("xorq"));
        assert!(s.contains("$-1"));
    }
    #[test]
    pub(super) fn test_emit_cmp() {
        let s = be().emit_instr(&X86Instr::Cmp(X86Reg::RAX, X86Reg::RBX));
        assert!(s.contains("cmpq"));
        assert!(s.contains("%rax"));
        assert!(s.contains("%rbx"));
    }
    #[test]
    pub(super) fn test_emit_cmp_imm() {
        let s = be().emit_instr(&X86Instr::CmpImm(X86Reg::RAX, 0));
        assert!(s.contains("cmpq"));
        assert!(s.contains("$0"));
    }
    #[test]
    pub(super) fn test_emit_je() {
        let s = be().emit_instr(&X86Instr::Je(".Lthen".into()));
        assert!(s.contains("je"));
        assert!(s.contains(".Lthen"));
    }
    #[test]
    pub(super) fn test_emit_jne() {
        let s = be().emit_instr(&X86Instr::Jne(".Lelse".into()));
        assert!(s.contains("jne"));
        assert!(s.contains(".Lelse"));
    }
    #[test]
    pub(super) fn test_emit_jmp() {
        let s = be().emit_instr(&X86Instr::Jmp(".Lend".into()));
        assert!(s.contains("jmp"));
        assert!(s.contains(".Lend"));
    }
    #[test]
    pub(super) fn test_emit_call() {
        let s = be().emit_instr(&X86Instr::Call("printf".into()));
        assert!(s.contains("call"));
        assert!(s.contains("printf"));
    }
    #[test]
    pub(super) fn test_emit_call_reg() {
        let s = be().emit_instr(&X86Instr::CallReg(X86Reg::RAX));
        assert!(s.contains("call"));
        assert!(s.contains("*%rax"));
    }
    #[test]
    pub(super) fn test_emit_ret() {
        let s = be().emit_instr(&X86Instr::Ret);
        assert!(s.contains("ret"));
    }
    #[test]
    pub(super) fn test_emit_label() {
        let s = be().emit_instr(&X86Instr::Label(".Lloop".into()));
        assert_eq!(s, ".Lloop:");
    }
    #[test]
    pub(super) fn test_emit_directive_with_arg() {
        let s = be().emit_instr(&X86Instr::Directive("section".into(), ".text".into()));
        assert!(s.contains(".section"));
        assert!(s.contains(".text"));
    }
    #[test]
    pub(super) fn test_emit_directive_no_arg() {
        let s = be().emit_instr(&X86Instr::Directive("text".into(), String::new()));
        assert!(s.contains(".text"));
    }
    #[test]
    pub(super) fn test_emit_shl() {
        let s = be().emit_instr(&X86Instr::Shl(X86Reg::RAX, 3));
        assert!(s.contains("shlq"));
        assert!(s.contains("$3"));
        assert!(s.contains("%rax"));
    }
    #[test]
    pub(super) fn test_emit_shr() {
        let s = be().emit_instr(&X86Instr::Shr(X86Reg::RBX, 1));
        assert!(s.contains("shrq"));
        assert!(s.contains("$1"));
    }
    #[test]
    pub(super) fn test_emit_sar() {
        let s = be().emit_instr(&X86Instr::Sar(X86Reg::RCX, 63));
        assert!(s.contains("sarq"));
        assert!(s.contains("$63"));
    }
    #[test]
    pub(super) fn test_emit_neg() {
        let s = be().emit_instr(&X86Instr::Neg(X86Reg::RAX));
        assert!(s.contains("negq"));
        assert!(s.contains("%rax"));
    }
    #[test]
    pub(super) fn test_emit_not() {
        let s = be().emit_instr(&X86Instr::Not(X86Reg::RBX));
        assert!(s.contains("notq"));
    }
    #[test]
    pub(super) fn test_emit_cqo() {
        let s = be().emit_instr(&X86Instr::Cqo);
        assert!(s.contains("cqo"));
    }
    #[test]
    pub(super) fn test_emit_raw() {
        let s = be().emit_instr(&X86Instr::Raw("/* custom asm */".into()));
        assert_eq!(s, "/* custom asm */");
    }
    #[test]
    pub(super) fn test_emit_movsd_load() {
        let mem = MemOp::new(X86Reg::RSP, -8);
        let s = be().emit_instr(&X86Instr::MovsdLoad(X86Reg::XMM0, mem));
        assert!(s.contains("movsd"));
        assert!(s.contains("%xmm0"));
        assert!(s.contains("-8(%rsp)"));
    }
    #[test]
    pub(super) fn test_emit_addsd() {
        let s = be().emit_instr(&X86Instr::AddsdReg(X86Reg::XMM0, X86Reg::XMM1));
        assert!(s.contains("addsd"));
        assert!(s.contains("%xmm0"));
        assert!(s.contains("%xmm1"));
    }
    #[test]
    pub(super) fn test_emit_function_contains_globl() {
        let mut f = X86Function::new("square");
        f.push(X86Instr::IMul(X86Reg::RDI, X86Reg::RDI));
        f.push(X86Instr::Mov(X86Reg::RAX, X86Reg::RDI));
        f.push(X86Instr::Ret);
        let s = be().emit_function(&f);
        assert!(s.contains(".globl"));
        assert!(s.contains("square"));
        assert!(s.contains("imulq"));
        assert!(s.contains("ret"));
    }
    #[test]
    pub(super) fn test_emit_function_size_directive() {
        let f = X86Function::new("empty_fn");
        let s = be().emit_function(&f);
        assert!(s.contains(".size"));
        assert!(s.contains("empty_fn"));
    }
    #[test]
    pub(super) fn test_prologue_pushes_rbp() {
        let prologue = be().prologue(0);
        let s = be().emit_instr(&prologue[0]);
        assert!(s.contains("pushq"));
        assert!(s.contains("%rbp"));
    }
    #[test]
    pub(super) fn test_prologue_sets_rbp() {
        let prologue = be().prologue(0);
        let s = be().emit_instr(&prologue[1]);
        assert!(s.contains("movq"));
        assert!(s.contains("%rbp"));
    }
    #[test]
    pub(super) fn test_prologue_16byte_alignment() {
        let prologue = be().prologue(1);
        let s = be().emit_instr(&prologue[2]);
        assert!(s.contains("subq"));
        assert!(s.contains("$16"));
    }
    #[test]
    pub(super) fn test_prologue_33_aligned_to_48() {
        let prologue = be().prologue(33);
        let s = be().emit_instr(&prologue[2]);
        assert!(s.contains("subq"));
        assert!(s.contains("$48"));
    }
    #[test]
    pub(super) fn test_epilogue_ends_with_ret() {
        let epilogue = be().epilogue();
        let last = be().emit_instr(epilogue.last().expect("collection should not be empty"));
        assert!(last.contains("ret"));
    }
    #[test]
    pub(super) fn test_epilogue_pops_rbp() {
        let epilogue = be().epilogue();
        let emitted: Vec<String> = epilogue.iter().map(|i| be().emit_instr(i)).collect();
        let joined = emitted.join("\n");
        assert!(joined.contains("popq"));
        assert!(joined.contains("%rbp"));
    }
    #[test]
    pub(super) fn test_calling_convention_first_arg_rdi() {
        let args = X86Backend::calling_convention_args();
        assert_eq!(args[0].name_att(), "%rdi");
    }
    #[test]
    pub(super) fn test_calling_convention_six_args() {
        assert_eq!(X86Backend::calling_convention_args().len(), 6);
    }
    #[test]
    pub(super) fn test_caller_saved_count() {
        assert_eq!(X86Backend::caller_saved().len(), 9);
    }
    #[test]
    pub(super) fn test_callee_saved_count() {
        assert_eq!(X86Backend::callee_saved().len(), 6);
    }
    #[test]
    pub(super) fn test_memop_zero_disp() {
        let mem = MemOp::new(X86Reg::RBP, 0);
        assert_eq!(mem.fmt_att(), "(%rbp)");
    }
    #[test]
    pub(super) fn test_memop_negative_disp() {
        let mem = MemOp::new(X86Reg::RBP, -8);
        assert_eq!(mem.fmt_att(), "-8(%rbp)");
    }
    #[test]
    pub(super) fn test_memop_positive_disp() {
        let mem = MemOp::new(X86Reg::RSP, 16);
        assert_eq!(mem.fmt_att(), "16(%rsp)");
    }
    #[test]
    pub(super) fn test_emit_mov_load() {
        let mem = MemOp::new(X86Reg::RBP, -8);
        let s = be().emit_instr(&X86Instr::MovLoad(X86Reg::RAX, mem));
        assert!(s.contains("movq"));
        assert!(s.contains("-8(%rbp)"));
        assert!(s.contains("%rax"));
    }
    #[test]
    pub(super) fn test_emit_mov_store() {
        let mem = MemOp::new(X86Reg::RBP, -16);
        let s = be().emit_instr(&X86Instr::MovStore(mem, X86Reg::RDI));
        assert!(s.contains("movq"));
        assert!(s.contains("%rdi"));
        assert!(s.contains("-16(%rbp)"));
    }
    #[test]
    pub(super) fn test_emit_lea() {
        let mem = MemOp::new(X86Reg::RBP, -24);
        let s = be().emit_instr(&X86Instr::Lea(X86Reg::RDI, mem));
        assert!(s.contains("leaq"));
        assert!(s.contains("-24(%rbp)"));
        assert!(s.contains("%rdi"));
    }
}
#[cfg(test)]
mod X86_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = X86PassConfig::new("test_pass", X86PassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = X86PassStats::new();
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
        let mut reg = X86PassRegistry::new();
        reg.register(X86PassConfig::new("pass_a", X86PassPhase::Analysis));
        reg.register(X86PassConfig::new("pass_b", X86PassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = X86AnalysisCache::new(10);
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
        let mut wl = X86Worklist::new();
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
        let mut dt = X86DominatorTree::new(5);
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
        let mut liveness = X86LivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(X86ConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(X86ConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(X86ConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            X86ConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(X86ConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = X86DepGraph::new();
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
mod x86ext_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_x86ext_phase_order() {
        assert_eq!(X86ExtPassPhase::Early.order(), 0);
        assert_eq!(X86ExtPassPhase::Middle.order(), 1);
        assert_eq!(X86ExtPassPhase::Late.order(), 2);
        assert_eq!(X86ExtPassPhase::Finalize.order(), 3);
        assert!(X86ExtPassPhase::Early.is_early());
        assert!(!X86ExtPassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_x86ext_config_builder() {
        let c = X86ExtPassConfig::new("p")
            .with_phase(X86ExtPassPhase::Late)
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
    pub(super) fn test_x86ext_stats() {
        let mut s = X86ExtPassStats::new();
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
    pub(super) fn test_x86ext_registry() {
        let mut r = X86ExtPassRegistry::new();
        r.register(X86ExtPassConfig::new("a").with_phase(X86ExtPassPhase::Early));
        r.register(X86ExtPassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&X86ExtPassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_x86ext_cache() {
        let mut c = X86ExtCache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_x86ext_worklist() {
        let mut w = X86ExtWorklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_x86ext_dom_tree() {
        let mut dt = X86ExtDomTree::new(5);
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
    pub(super) fn test_x86ext_liveness() {
        let mut lv = X86ExtLiveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_x86ext_const_folder() {
        let mut cf = X86ExtConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_x86ext_dep_graph() {
        let mut g = X86ExtDepGraph::new(4);
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
#[cfg(test)]
mod x86x2_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_x86x2_phase_order() {
        assert_eq!(X86X2PassPhase::Early.order(), 0);
        assert_eq!(X86X2PassPhase::Middle.order(), 1);
        assert_eq!(X86X2PassPhase::Late.order(), 2);
        assert_eq!(X86X2PassPhase::Finalize.order(), 3);
        assert!(X86X2PassPhase::Early.is_early());
        assert!(!X86X2PassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_x86x2_config_builder() {
        let c = X86X2PassConfig::new("p")
            .with_phase(X86X2PassPhase::Late)
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
    pub(super) fn test_x86x2_stats() {
        let mut s = X86X2PassStats::new();
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
    pub(super) fn test_x86x2_registry() {
        let mut r = X86X2PassRegistry::new();
        r.register(X86X2PassConfig::new("a").with_phase(X86X2PassPhase::Early));
        r.register(X86X2PassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&X86X2PassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_x86x2_cache() {
        let mut c = X86X2Cache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_x86x2_worklist() {
        let mut w = X86X2Worklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_x86x2_dom_tree() {
        let mut dt = X86X2DomTree::new(5);
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
    pub(super) fn test_x86x2_liveness() {
        let mut lv = X86X2Liveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_x86x2_const_folder() {
        let mut cf = X86X2ConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_x86x2_dep_graph() {
        let mut g = X86X2DepGraph::new(4);
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
