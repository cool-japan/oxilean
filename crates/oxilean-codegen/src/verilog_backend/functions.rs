//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    VerilogBackend, VerilogExpr, VerilogExtCache, VerilogExtConstFolder, VerilogExtDepGraph,
    VerilogExtDomTree, VerilogExtLiveness, VerilogExtPassConfig, VerilogExtPassPhase,
    VerilogExtPassRegistry, VerilogExtPassStats, VerilogExtWorklist, VerilogModule, VerilogPort,
    VerilogType, VerilogX2Cache, VerilogX2ConstFolder, VerilogX2DepGraph, VerilogX2DomTree,
    VerilogX2Liveness, VerilogX2PassConfig, VerilogX2PassPhase, VerilogX2PassRegistry,
    VerilogX2PassStats, VerilogX2Worklist, VlogAnalysisCache, VlogConstantFoldingHelper,
    VlogDepGraph, VlogDominatorTree, VlogLivenessInfo, VlogPassConfig, VlogPassPhase,
    VlogPassRegistry, VlogPassStats, VlogWorklist,
};

pub(super) fn range_suffix(width: u32) -> String {
    if width <= 1 {
        String::new()
    } else {
        format!(" [{width_m1}:0]", width_m1 = width - 1)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub(super) fn test_type_wire_single_bit() {
        let t = VerilogType::Wire(1);
        assert_eq!(t.to_string(), "wire");
    }
    #[test]
    pub(super) fn test_type_wire_multi_bit() {
        let t = VerilogType::Wire(8);
        assert_eq!(t.to_string(), "wire [7:0]");
    }
    #[test]
    pub(super) fn test_type_reg_32bit() {
        let t = VerilogType::Reg(32);
        assert_eq!(t.to_string(), "reg [31:0]");
    }
    #[test]
    pub(super) fn test_type_logic_sv() {
        let t = VerilogType::Logic(16);
        assert_eq!(t.to_string(), "logic [15:0]");
    }
    #[test]
    pub(super) fn test_type_logic_fallback_to_reg() {
        let backend = VerilogBackend::new(false);
        let t = VerilogType::Logic(4);
        assert_eq!(backend.emit_type(&t), "reg [3:0]");
    }
    #[test]
    pub(super) fn test_type_logic_kept_in_sv() {
        let backend = VerilogBackend::new(true);
        let t = VerilogType::Logic(4);
        assert_eq!(backend.emit_type(&t), "logic [3:0]");
    }
    #[test]
    pub(super) fn test_type_integer_real() {
        assert_eq!(VerilogType::Integer.to_string(), "integer");
        assert_eq!(VerilogType::Real.to_string(), "real");
    }
    #[test]
    pub(super) fn test_expr_literal_1bit() {
        let e = VerilogExpr::Lit(1, 1);
        assert_eq!(e.to_string(), "1'b1");
    }
    #[test]
    pub(super) fn test_expr_literal_multibit() {
        let e = VerilogExpr::Lit(0xFF, 8);
        assert_eq!(e.to_string(), "8'hFF");
    }
    #[test]
    pub(super) fn test_expr_var() {
        let e = VerilogExpr::Var("clk".to_string());
        assert_eq!(e.to_string(), "clk");
    }
    #[test]
    pub(super) fn test_expr_binop() {
        let lhs = VerilogExpr::Var("a".to_string());
        let rhs = VerilogExpr::Var("b".to_string());
        let e = VerilogExpr::BinOp(Box::new(lhs), "&".to_string(), Box::new(rhs));
        assert_eq!(e.to_string(), "(a & b)");
    }
    #[test]
    pub(super) fn test_expr_unop() {
        let inner = VerilogExpr::Var("x".to_string());
        let e = VerilogExpr::UnOp("~".to_string(), Box::new(inner));
        assert_eq!(e.to_string(), "(~x)");
    }
    #[test]
    pub(super) fn test_expr_concat() {
        let e = VerilogExpr::Concat(vec![
            VerilogExpr::Var("msb".to_string()),
            VerilogExpr::Var("lsb".to_string()),
        ]);
        assert_eq!(e.to_string(), "{msb, lsb}");
    }
    #[test]
    pub(super) fn test_expr_replicate() {
        let inner = VerilogExpr::Lit(0, 1);
        let e = VerilogExpr::Replicate(4, Box::new(inner));
        assert_eq!(e.to_string(), "{4{1'b0}}");
    }
    #[test]
    pub(super) fn test_expr_index() {
        let e = VerilogExpr::Index(Box::new(VerilogExpr::Var("data".to_string())), 7);
        assert_eq!(e.to_string(), "data[7]");
    }
    #[test]
    pub(super) fn test_expr_slice() {
        let e = VerilogExpr::Slice(Box::new(VerilogExpr::Var("data".to_string())), 15, 8);
        assert_eq!(e.to_string(), "data[15:8]");
    }
    #[test]
    pub(super) fn test_expr_ternary() {
        let e = VerilogExpr::Ternary(
            Box::new(VerilogExpr::Var("sel".to_string())),
            Box::new(VerilogExpr::Var("a".to_string())),
            Box::new(VerilogExpr::Var("b".to_string())),
        );
        assert_eq!(e.to_string(), "(sel ? a : b)");
    }
    #[test]
    pub(super) fn test_expr_call() {
        let e = VerilogExpr::Call(
            "$clog2".to_string(),
            vec![VerilogExpr::Var("WIDTH".to_string())],
        );
        assert_eq!(e.to_string(), "$clog2(WIDTH)");
    }
    #[test]
    pub(super) fn test_assign_stmt() {
        let b = VerilogBackend::new(false);
        assert_eq!(b.assign_stmt("out", "a & b"), "assign out = a & b;");
    }
    #[test]
    pub(super) fn test_nonblocking_assign() {
        let b = VerilogBackend::new(false);
        assert_eq!(b.nonblocking_assign("q", "d"), "q <= d;");
    }
    #[test]
    pub(super) fn test_blocking_assign() {
        let b = VerilogBackend::new(false);
        assert_eq!(b.blocking_assign("tmp", "a + b"), "tmp = a + b;");
    }
    #[test]
    pub(super) fn test_always_block_clocked() {
        let b = VerilogBackend::new(false);
        let result = b.always_block("posedge clk", "q <= d;");
        assert!(result.contains("always @(posedge clk)"));
        assert!(result.contains("q <= d;"));
    }
    #[test]
    pub(super) fn test_always_comb_sv() {
        let b = VerilogBackend::new(true);
        let result = b.always_block("comb", "y = a | b;");
        assert!(result.starts_with("always_comb"));
    }
    #[test]
    pub(super) fn test_always_ff_sv() {
        let b = VerilogBackend::new(true);
        let result = b.always_block("ff posedge clk", "q <= d;");
        assert!(result.starts_with("always_ff @(posedge clk)"));
    }
    #[test]
    pub(super) fn test_always_block_star_verilog() {
        let b = VerilogBackend::new(false);
        let result = b.always_block("*", "y = a & b;");
        assert!(result.contains("always @(*)"));
    }
    #[test]
    pub(super) fn test_if_else_with_else() {
        let b = VerilogBackend::new(false);
        let result = b.if_else("rst", "q <= 0;", Some("q <= d;"));
        assert!(result.contains("if (rst)"));
        assert!(result.contains("else"));
    }
    #[test]
    pub(super) fn test_if_no_else() {
        let b = VerilogBackend::new(false);
        let result = b.if_else("en", "q <= d;", None);
        assert!(result.contains("if (en)"));
        assert!(!result.contains("else"));
    }
    #[test]
    pub(super) fn test_initial_block() {
        let b = VerilogBackend::new(false);
        let result = b.initial_block("q = 0;");
        assert!(result.starts_with("initial begin"));
        assert!(result.contains("q = 0;"));
    }
    #[test]
    pub(super) fn test_declare_signal_wire() {
        let b = VerilogBackend::new(false);
        let result = b.declare_signal(&VerilogType::Wire(8), "data");
        assert_eq!(result, "wire [7:0] data;");
    }
    #[test]
    pub(super) fn test_declare_signal_reg() {
        let b = VerilogBackend::new(false);
        let result = b.declare_signal(&VerilogType::Reg(1), "flag");
        assert_eq!(result, "reg flag;");
    }
    #[test]
    pub(super) fn test_instantiate() {
        let b = VerilogBackend::new(false);
        let result = b.instantiate(
            "adder",
            "u_adder",
            &[("a", "sig_a"), ("b", "sig_b"), ("sum", "result")],
        );
        assert!(result.contains("adder u_adder ("));
        assert!(result.contains(".a(sig_a)"));
        assert!(result.contains(".sum(result)"));
    }
    #[test]
    pub(super) fn test_sys_task_display() {
        let b = VerilogBackend::new(false);
        let result = b.sys_task("$display", "value=%d", &["x"]);
        assert_eq!(result, "$display(\"value=%d\", x);");
    }
    #[test]
    pub(super) fn test_sys_task_no_args() {
        let b = VerilogBackend::new(false);
        let result = b.sys_task("$finish", "", &[]);
        assert_eq!(result, "$finish(\"\");");
    }
    #[test]
    pub(super) fn test_emit_simple_and_gate() {
        let b = VerilogBackend::new(false);
        let mut m = VerilogModule::new("and_gate");
        m.add_port(VerilogPort::Input("a".to_string(), 1));
        m.add_port(VerilogPort::Input("b".to_string(), 1));
        m.add_port(VerilogPort::Output("y".to_string(), 1));
        m.add_stmt(b.assign_stmt("y", "a & b"));
        let code = b.emit_module(&m);
        assert!(code.contains("module and_gate"));
        assert!(code.contains("input"));
        assert!(code.contains("output"));
        assert!(code.contains("assign y = a & b;"));
        assert!(code.contains("endmodule"));
    }
    #[test]
    pub(super) fn test_emit_module_with_params() {
        let b = VerilogBackend::new(false);
        let mut m = VerilogModule::new("fifo");
        m.add_param("DEPTH", 16);
        m.add_param("WIDTH", 8);
        m.add_port(VerilogPort::Input("clk".to_string(), 1));
        m.add_port(VerilogPort::Output("full".to_string(), 1));
        let code = b.emit_module(&m);
        assert!(code.contains("parameter DEPTH = 16"));
        assert!(code.contains("parameter WIDTH = 8"));
    }
    #[test]
    pub(super) fn test_emit_sv_module_uses_logic() {
        let b = VerilogBackend::new(true);
        let mut m = VerilogModule::new("sv_mod");
        m.add_port(VerilogPort::Input("clk".to_string(), 1));
        m.add_port(VerilogPort::Output("q".to_string(), 8));
        let code = b.emit_module(&m);
        assert!(code.contains("logic"));
    }
    #[test]
    pub(super) fn test_emit_module_no_ports_no_params() {
        let b = VerilogBackend::new(false);
        let m = VerilogModule::new("empty_mod");
        let code = b.emit_module(&m);
        assert!(code.contains("module empty_mod"));
        assert!(code.contains("endmodule"));
    }
    #[test]
    pub(super) fn test_emit_d_flip_flop() {
        let b = VerilogBackend::new(false);
        let mut m = VerilogModule::new("dff");
        m.add_port(VerilogPort::Input("clk".to_string(), 1));
        m.add_port(VerilogPort::Input("d".to_string(), 1));
        m.add_port(VerilogPort::Output("q".to_string(), 1));
        m.add_stmt(b.always_block("posedge clk", "q <= d;"));
        let code = b.emit_module(&m);
        assert!(code.contains("always @(posedge clk)"));
        assert!(code.contains("q <= d;"));
    }
    #[test]
    pub(super) fn test_verilog_backend_default() {
        let b = VerilogBackend::default();
        assert!(!b.system_verilog);
    }
    #[test]
    pub(super) fn test_port_accessors() {
        let p = VerilogPort::Input("my_sig".to_string(), 32);
        assert_eq!(p.name(), "my_sig");
        assert_eq!(p.width(), 32);
        assert_eq!(p.direction(), "input");
        let p2 = VerilogPort::Output("out_sig".to_string(), 16);
        assert_eq!(p2.direction(), "output");
        let p3 = VerilogPort::InOut("bidir".to_string(), 8);
        assert_eq!(p3.direction(), "inout");
    }
    #[test]
    pub(super) fn test_module_add_helpers() {
        let mut m = VerilogModule::new("test");
        m.add_port(VerilogPort::Input("clk".to_string(), 1));
        m.add_param("N", 4);
        m.add_stmt("assign x = 0;");
        assert_eq!(m.ports.len(), 1);
        assert_eq!(m.params.len(), 1);
        assert_eq!(m.body.len(), 1);
    }
}
#[cfg(test)]
mod Vlog_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = VlogPassConfig::new("test_pass", VlogPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = VlogPassStats::new();
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
        let mut reg = VlogPassRegistry::new();
        reg.register(VlogPassConfig::new("pass_a", VlogPassPhase::Analysis));
        reg.register(VlogPassConfig::new("pass_b", VlogPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = VlogAnalysisCache::new(10);
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
        let mut wl = VlogWorklist::new();
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
        let mut dt = VlogDominatorTree::new(5);
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
        let mut liveness = VlogLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(VlogConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(VlogConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(VlogConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            VlogConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(VlogConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = VlogDepGraph::new();
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
mod verilogext_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_verilogext_phase_order() {
        assert_eq!(VerilogExtPassPhase::Early.order(), 0);
        assert_eq!(VerilogExtPassPhase::Middle.order(), 1);
        assert_eq!(VerilogExtPassPhase::Late.order(), 2);
        assert_eq!(VerilogExtPassPhase::Finalize.order(), 3);
        assert!(VerilogExtPassPhase::Early.is_early());
        assert!(!VerilogExtPassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_verilogext_config_builder() {
        let c = VerilogExtPassConfig::new("p")
            .with_phase(VerilogExtPassPhase::Late)
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
    pub(super) fn test_verilogext_stats() {
        let mut s = VerilogExtPassStats::new();
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
    pub(super) fn test_verilogext_registry() {
        let mut r = VerilogExtPassRegistry::new();
        r.register(VerilogExtPassConfig::new("a").with_phase(VerilogExtPassPhase::Early));
        r.register(VerilogExtPassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&VerilogExtPassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_verilogext_cache() {
        let mut c = VerilogExtCache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_verilogext_worklist() {
        let mut w = VerilogExtWorklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_verilogext_dom_tree() {
        let mut dt = VerilogExtDomTree::new(5);
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
    pub(super) fn test_verilogext_liveness() {
        let mut lv = VerilogExtLiveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_verilogext_const_folder() {
        let mut cf = VerilogExtConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_verilogext_dep_graph() {
        let mut g = VerilogExtDepGraph::new(4);
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
mod verilogx2_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_verilogx2_phase_order() {
        assert_eq!(VerilogX2PassPhase::Early.order(), 0);
        assert_eq!(VerilogX2PassPhase::Middle.order(), 1);
        assert_eq!(VerilogX2PassPhase::Late.order(), 2);
        assert_eq!(VerilogX2PassPhase::Finalize.order(), 3);
        assert!(VerilogX2PassPhase::Early.is_early());
        assert!(!VerilogX2PassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_verilogx2_config_builder() {
        let c = VerilogX2PassConfig::new("p")
            .with_phase(VerilogX2PassPhase::Late)
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
    pub(super) fn test_verilogx2_stats() {
        let mut s = VerilogX2PassStats::new();
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
    pub(super) fn test_verilogx2_registry() {
        let mut r = VerilogX2PassRegistry::new();
        r.register(VerilogX2PassConfig::new("a").with_phase(VerilogX2PassPhase::Early));
        r.register(VerilogX2PassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&VerilogX2PassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_verilogx2_cache() {
        let mut c = VerilogX2Cache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_verilogx2_worklist() {
        let mut w = VerilogX2Worklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_verilogx2_dom_tree() {
        let mut dt = VerilogX2DomTree::new(5);
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
    pub(super) fn test_verilogx2_liveness() {
        let mut lv = VerilogX2Liveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_verilogx2_const_folder() {
        let mut cf = VerilogX2ConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_verilogx2_dep_graph() {
        let mut g = VerilogX2DepGraph::new(4);
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
