//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    RISCVExtCache, RISCVExtConstFolder, RISCVExtDepGraph, RISCVExtDomTree, RISCVExtLiveness,
    RISCVExtPassConfig, RISCVExtPassPhase, RISCVExtPassRegistry, RISCVExtPassStats,
    RISCVExtWorklist, RISCVX2Cache, RISCVX2ConstFolder, RISCVX2DepGraph, RISCVX2DomTree,
    RISCVX2Liveness, RISCVX2PassConfig, RISCVX2PassPhase, RISCVX2PassRegistry, RISCVX2PassStats,
    RISCVX2Worklist, RiscVBackend, RiscVFunction, RiscVInstr, RiscVReg, RvAnalysisCache,
    RvConstantFoldingHelper, RvDepGraph, RvDominatorTree, RvLivenessInfo, RvPassConfig,
    RvPassPhase, RvPassRegistry, RvPassStats, RvWorklist,
};

#[cfg(test)]
mod tests {
    use super::*;
    pub(super) fn rv32() -> RiscVBackend {
        RiscVBackend::new(false)
    }
    pub(super) fn rv64() -> RiscVBackend {
        RiscVBackend::new(true)
    }
    #[test]
    pub(super) fn test_reg_zero_name() {
        assert_eq!(RiscVReg::Zero.name(), "zero");
    }
    #[test]
    pub(super) fn test_reg_ra_name() {
        assert_eq!(RiscVReg::Ra.name(), "ra");
    }
    #[test]
    pub(super) fn test_reg_sp_name() {
        assert_eq!(RiscVReg::Sp.name(), "sp");
    }
    #[test]
    pub(super) fn test_reg_a0_name() {
        assert_eq!(RiscVReg::A0.name(), "a0");
    }
    #[test]
    pub(super) fn test_reg_s0_name() {
        assert_eq!(RiscVReg::S0.name(), "s0");
    }
    #[test]
    pub(super) fn test_reg_t6_name() {
        assert_eq!(RiscVReg::T6.name(), "t6");
    }
    #[test]
    pub(super) fn test_reg_s11_name() {
        assert_eq!(RiscVReg::S11.name(), "s11");
    }
    #[test]
    pub(super) fn test_reg_index_zero() {
        assert_eq!(RiscVReg::Zero.index(), 0);
    }
    #[test]
    pub(super) fn test_reg_index_a0() {
        assert_eq!(RiscVReg::A0.index(), 10);
    }
    #[test]
    pub(super) fn test_reg_index_t6() {
        assert_eq!(RiscVReg::T6.index(), 31);
    }
    #[test]
    pub(super) fn test_emit_lui() {
        let b = rv32();
        let s = b.emit_instr(&RiscVInstr::LUI(RiscVReg::A0, 0xDEAD));
        assert!(s.contains("lui"));
        assert!(s.contains("a0"));
        assert!(s.contains("57005"));
    }
    #[test]
    pub(super) fn test_emit_addi() {
        let b = rv32();
        let s = b.emit_instr(&RiscVInstr::ADDI(RiscVReg::A0, RiscVReg::A0, 1));
        assert!(s.contains("addi"));
        assert!(s.contains("a0"));
        assert!(s.contains('1'));
    }
    #[test]
    pub(super) fn test_emit_add() {
        let b = rv64();
        let s = b.emit_instr(&RiscVInstr::ADD(RiscVReg::T0, RiscVReg::A0, RiscVReg::A1));
        assert!(s.contains("add"));
        assert!(s.contains("t0"));
        assert!(s.contains("a0"));
        assert!(s.contains("a1"));
    }
    #[test]
    pub(super) fn test_emit_sub() {
        let b = rv64();
        let s = b.emit_instr(&RiscVInstr::SUB(RiscVReg::T1, RiscVReg::A0, RiscVReg::A1));
        assert!(s.contains("sub"));
        assert!(s.contains("t1"));
    }
    #[test]
    pub(super) fn test_emit_and() {
        let b = rv32();
        let s = b.emit_instr(&RiscVInstr::AND(RiscVReg::A0, RiscVReg::A1, RiscVReg::A2));
        assert!(s.contains("and"));
        assert!(s.contains("a0"));
        assert!(s.contains("a1"));
        assert!(s.contains("a2"));
    }
    #[test]
    pub(super) fn test_emit_or() {
        let b = rv32();
        let s = b.emit_instr(&RiscVInstr::OR(RiscVReg::A0, RiscVReg::A1, RiscVReg::A2));
        assert!(s.contains("or"));
    }
    #[test]
    pub(super) fn test_emit_xor() {
        let b = rv32();
        let s = b.emit_instr(&RiscVInstr::XOR(RiscVReg::A0, RiscVReg::A1, RiscVReg::A2));
        assert!(s.contains("xor"));
    }
    #[test]
    pub(super) fn test_emit_lw() {
        let b = rv32();
        let s = b.emit_instr(&RiscVInstr::LW(RiscVReg::A0, RiscVReg::Sp, 8));
        assert!(s.contains("lw"));
        assert!(s.contains("8(sp)"));
    }
    #[test]
    pub(super) fn test_emit_sw() {
        let b = rv32();
        let s = b.emit_instr(&RiscVInstr::SW(RiscVReg::A0, RiscVReg::Sp, 0));
        assert!(s.contains("sw"));
        assert!(s.contains("sp"));
    }
    #[test]
    pub(super) fn test_emit_ld() {
        let b = rv64();
        let s = b.emit_instr(&RiscVInstr::LD(RiscVReg::Ra, RiscVReg::Sp, -8));
        assert!(s.contains("ld"));
        assert!(s.contains("-8(sp)"));
    }
    #[test]
    pub(super) fn test_emit_sd() {
        let b = rv64();
        let s = b.emit_instr(&RiscVInstr::SD(RiscVReg::Ra, RiscVReg::Sp, -8));
        assert!(s.contains("sd"));
        assert!(s.contains("-8(sp)"));
    }
    #[test]
    pub(super) fn test_emit_beq() {
        let b = rv32();
        let s = b.emit_instr(&RiscVInstr::BEQ(RiscVReg::A0, RiscVReg::Zero, 16));
        assert!(s.contains("beq"));
        assert!(s.contains("a0"));
        assert!(s.contains("zero"));
    }
    #[test]
    pub(super) fn test_emit_jal() {
        let b = rv32();
        let s = b.emit_instr(&RiscVInstr::JAL(RiscVReg::Ra, -4));
        assert!(s.contains("jal"));
        assert!(s.contains("ra"));
        assert!(s.contains("-4"));
    }
    #[test]
    pub(super) fn test_emit_jalr() {
        let b = rv32();
        let s = b.emit_instr(&RiscVInstr::JALR(RiscVReg::Zero, RiscVReg::Ra, 0));
        assert!(s.contains("jalr"));
        assert!(s.contains("zero"));
        assert!(s.contains("ra"));
    }
    #[test]
    pub(super) fn test_emit_ret_pseudo() {
        let b = rv64();
        let s = b.emit_instr(&RiscVInstr::RET);
        assert!(s.contains("ret"));
    }
    #[test]
    pub(super) fn test_emit_li_pseudo() {
        let b = rv64();
        let s = b.emit_instr(&RiscVInstr::LI(RiscVReg::A0, 42));
        assert!(s.contains("li"));
        assert!(s.contains("a0"));
        assert!(s.contains("42"));
    }
    #[test]
    pub(super) fn test_emit_mv_pseudo() {
        let b = rv32();
        let s = b.emit_instr(&RiscVInstr::MV(RiscVReg::A0, RiscVReg::A1));
        assert!(s.contains("mv"));
        assert!(s.contains("a0"));
        assert!(s.contains("a1"));
    }
    #[test]
    pub(super) fn test_emit_call_pseudo() {
        let b = rv64();
        let s = b.emit_instr(&RiscVInstr::CALL("my_func".into()));
        assert!(s.contains("call"));
        assert!(s.contains("my_func"));
    }
    #[test]
    pub(super) fn test_emit_ecall() {
        let b = rv32();
        let s = b.emit_instr(&RiscVInstr::ECALL);
        assert!(s.contains("ecall"));
    }
    #[test]
    pub(super) fn test_emit_label() {
        let b = rv32();
        let s = b.emit_instr(&RiscVInstr::Label("loop_top".into()));
        assert_eq!(s, "loop_top:");
    }
    #[test]
    pub(super) fn test_emit_directive_with_arg() {
        let b = rv32();
        let s = b.emit_instr(&RiscVInstr::Directive("section".into(), ".text".into()));
        assert!(s.contains(".section"));
        assert!(s.contains(".text"));
    }
    #[test]
    pub(super) fn test_emit_directive_no_arg() {
        let b = rv32();
        let s = b.emit_instr(&RiscVInstr::Directive("text".into(), String::new()));
        assert!(s.contains(".text"));
    }
    #[test]
    pub(super) fn test_emit_mul() {
        let b = rv32();
        let s = b.emit_instr(&RiscVInstr::MUL(RiscVReg::A0, RiscVReg::A1, RiscVReg::A2));
        assert!(s.contains("mul"));
    }
    #[test]
    pub(super) fn test_emit_div() {
        let b = rv32();
        let s = b.emit_instr(&RiscVInstr::DIV(RiscVReg::A0, RiscVReg::A1, RiscVReg::A2));
        assert!(s.contains("div"));
    }
    #[test]
    pub(super) fn test_emit_function_contains_globl() {
        let b = rv64();
        let mut f = RiscVFunction::new("add_two");
        f.push(RiscVInstr::ADD(RiscVReg::A0, RiscVReg::A0, RiscVReg::A1));
        f.push(RiscVInstr::RET);
        let s = b.emit_function(&f);
        assert!(s.contains(".globl"));
        assert!(s.contains("add_two"));
        assert!(s.contains("add"));
        assert!(s.contains("ret"));
    }
    #[test]
    pub(super) fn test_emit_function_size_directive() {
        let b = rv32();
        let f = RiscVFunction::new("empty_fn");
        let s = b.emit_function(&f);
        assert!(s.contains(".size"));
        assert!(s.contains("empty_fn"));
    }
    #[test]
    pub(super) fn test_prologue_rv32_has_sw() {
        let b = rv32();
        let prologue = b.prologue(16);
        let emitted: Vec<String> = prologue.iter().map(|i| b.emit_instr(i)).collect();
        let joined = emitted.join("\n");
        assert!(joined.contains("addi    sp"));
        assert!(joined.contains("sw"));
    }
    #[test]
    pub(super) fn test_prologue_rv64_has_sd() {
        let b = rv64();
        let prologue = b.prologue(16);
        let emitted: Vec<String> = prologue.iter().map(|i| b.emit_instr(i)).collect();
        let joined = emitted.join("\n");
        assert!(joined.contains("addi    sp"));
        assert!(joined.contains("sd"));
    }
    #[test]
    pub(super) fn test_epilogue_rv32_has_lw_ret() {
        let b = rv32();
        let epilogue = b.epilogue();
        let emitted: Vec<String> = epilogue.iter().map(|i| b.emit_instr(i)).collect();
        let joined = emitted.join("\n");
        assert!(joined.contains("lw"));
        assert!(joined.contains("ret"));
    }
    #[test]
    pub(super) fn test_epilogue_rv64_has_ld_ret() {
        let b = rv64();
        let epilogue = b.epilogue();
        let emitted: Vec<String> = epilogue.iter().map(|i| b.emit_instr(i)).collect();
        let joined = emitted.join("\n");
        assert!(joined.contains("ld"));
        assert!(joined.contains("ret"));
    }
    #[test]
    pub(super) fn test_prologue_16byte_alignment() {
        let b = rv64();
        let prologue = b.prologue(1);
        let first = b.emit_instr(&prologue[0]);
        assert!(first.contains("addi    sp, sp, -16"));
    }
    #[test]
    pub(super) fn test_prologue_33_aligned_to_48() {
        let b = rv64();
        let prologue = b.prologue(33);
        let first = b.emit_instr(&prologue[0]);
        assert!(first.contains("addi    sp, sp, -48"));
    }
    #[test]
    pub(super) fn test_calling_convention_args_count() {
        assert_eq!(RiscVBackend::calling_convention_args().len(), 8);
    }
    #[test]
    pub(super) fn test_calling_convention_args_first_is_a0() {
        let args = RiscVBackend::calling_convention_args();
        assert_eq!(args[0].name(), "a0");
    }
    #[test]
    pub(super) fn test_calling_convention_args_last_is_a7() {
        let args = RiscVBackend::calling_convention_args();
        assert_eq!(args[7].name(), "a7");
    }
    #[test]
    pub(super) fn test_caller_saved_count() {
        assert_eq!(RiscVBackend::caller_saved().len(), 7);
    }
    #[test]
    pub(super) fn test_callee_saved_count() {
        assert_eq!(RiscVBackend::callee_saved().len(), 12);
    }
    #[test]
    pub(super) fn test_emit_addiw() {
        let b = rv64();
        let s = b.emit_instr(&RiscVInstr::ADDIW(RiscVReg::A0, RiscVReg::A0, -1));
        assert!(s.contains("addiw"));
        assert!(s.contains("-1"));
    }
    #[test]
    pub(super) fn test_emit_slli() {
        let b = rv64();
        let s = b.emit_instr(&RiscVInstr::SLLI(RiscVReg::A0, RiscVReg::A0, 3));
        assert!(s.contains("slli"));
        assert!(s.contains('3'));
    }
    #[test]
    pub(super) fn test_emit_nop() {
        let b = rv32();
        let s = b.emit_instr(&RiscVInstr::NOP);
        assert!(s.contains("nop"));
    }
    #[test]
    pub(super) fn test_emit_ebreak() {
        let b = rv32();
        let s = b.emit_instr(&RiscVInstr::EBREAK);
        assert!(s.contains("ebreak"));
    }
}
#[cfg(test)]
mod Rv_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = RvPassConfig::new("test_pass", RvPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = RvPassStats::new();
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
        let mut reg = RvPassRegistry::new();
        reg.register(RvPassConfig::new("pass_a", RvPassPhase::Analysis));
        reg.register(RvPassConfig::new("pass_b", RvPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = RvAnalysisCache::new(10);
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
        let mut wl = RvWorklist::new();
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
        let mut dt = RvDominatorTree::new(5);
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
        let mut liveness = RvLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(RvConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(RvConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(RvConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            RvConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(RvConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = RvDepGraph::new();
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
mod riscvext_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_riscvext_phase_order() {
        assert_eq!(RISCVExtPassPhase::Early.order(), 0);
        assert_eq!(RISCVExtPassPhase::Middle.order(), 1);
        assert_eq!(RISCVExtPassPhase::Late.order(), 2);
        assert_eq!(RISCVExtPassPhase::Finalize.order(), 3);
        assert!(RISCVExtPassPhase::Early.is_early());
        assert!(!RISCVExtPassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_riscvext_config_builder() {
        let c = RISCVExtPassConfig::new("p")
            .with_phase(RISCVExtPassPhase::Late)
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
    pub(super) fn test_riscvext_stats() {
        let mut s = RISCVExtPassStats::new();
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
    pub(super) fn test_riscvext_registry() {
        let mut r = RISCVExtPassRegistry::new();
        r.register(RISCVExtPassConfig::new("a").with_phase(RISCVExtPassPhase::Early));
        r.register(RISCVExtPassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&RISCVExtPassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_riscvext_cache() {
        let mut c = RISCVExtCache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_riscvext_worklist() {
        let mut w = RISCVExtWorklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_riscvext_dom_tree() {
        let mut dt = RISCVExtDomTree::new(5);
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
    pub(super) fn test_riscvext_liveness() {
        let mut lv = RISCVExtLiveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_riscvext_const_folder() {
        let mut cf = RISCVExtConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_riscvext_dep_graph() {
        let mut g = RISCVExtDepGraph::new(4);
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
mod riscvx2_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_riscvx2_phase_order() {
        assert_eq!(RISCVX2PassPhase::Early.order(), 0);
        assert_eq!(RISCVX2PassPhase::Middle.order(), 1);
        assert_eq!(RISCVX2PassPhase::Late.order(), 2);
        assert_eq!(RISCVX2PassPhase::Finalize.order(), 3);
        assert!(RISCVX2PassPhase::Early.is_early());
        assert!(!RISCVX2PassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_riscvx2_config_builder() {
        let c = RISCVX2PassConfig::new("p")
            .with_phase(RISCVX2PassPhase::Late)
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
    pub(super) fn test_riscvx2_stats() {
        let mut s = RISCVX2PassStats::new();
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
    pub(super) fn test_riscvx2_registry() {
        let mut r = RISCVX2PassRegistry::new();
        r.register(RISCVX2PassConfig::new("a").with_phase(RISCVX2PassPhase::Early));
        r.register(RISCVX2PassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&RISCVX2PassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_riscvx2_cache() {
        let mut c = RISCVX2Cache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_riscvx2_worklist() {
        let mut w = RISCVX2Worklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_riscvx2_dom_tree() {
        let mut dt = RISCVX2DomTree::new(5);
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
    pub(super) fn test_riscvx2_liveness() {
        let mut lv = RISCVX2Liveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_riscvx2_const_folder() {
        let mut cf = RISCVX2ConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_riscvx2_dep_graph() {
        let mut g = RISCVX2DepGraph::new(4);
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
