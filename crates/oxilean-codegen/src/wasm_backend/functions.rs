//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::{
    WasmAnalysisCache, WasmBackend, WasmConstantFoldingHelper, WasmDepGraph, WasmDominatorTree,
    WasmExtCache, WasmExtConstFolder, WasmExtDepGraph, WasmExtDomTree, WasmExtLiveness,
    WasmExtPassConfig, WasmExtPassPhase, WasmExtPassRegistry, WasmExtPassStats, WasmExtWorklist,
    WasmFunc, WasmImport, WasmImportKind, WasmInstr, WasmLivenessInfo, WasmModule, WasmPassConfig,
    WasmPassPhase, WasmPassRegistry, WasmPassStats, WasmType, WasmWorklist, WasmX2Cache,
    WasmX2ConstFolder, WasmX2DepGraph, WasmX2DomTree, WasmX2Liveness, WasmX2PassConfig,
    WasmX2PassPhase, WasmX2PassRegistry, WasmX2PassStats, WasmX2Worklist,
};

/// Mangle a Lean-style dotted name for use in WebAssembly identifiers.
///
/// WebAssembly identifiers cannot contain `.` or special characters,
/// so we replace them with underscores.
pub fn mangle_wasm_name(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => c,
            '.' => '_',
            _ => '_',
        })
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub(super) fn test_wat_module_empty() {
        let module = WasmModule::new();
        let wat = module.to_wat();
        assert!(wat.starts_with("(module\n"));
        assert!(wat.ends_with(')'));
        assert!(!wat.contains("func"));
        assert!(!wat.contains("import"));
    }
    #[test]
    pub(super) fn test_wasm_func_display() {
        let func = WasmFunc {
            name: "add".to_string(),
            params: vec![
                ("a".to_string(), WasmType::I32),
                ("b".to_string(), WasmType::I32),
            ],
            results: vec![WasmType::I32],
            locals: vec![],
            body: vec![
                WasmInstr::LocalGet("a".to_string()),
                WasmInstr::LocalGet("b".to_string()),
                WasmInstr::I32Add,
            ],
        };
        let s = format!("{}", func);
        assert!(s.contains("func $add"));
        assert!(s.contains("param $a i32"));
        assert!(s.contains("param $b i32"));
        assert!(s.contains("result i32"));
        assert!(s.contains("local.get $a"));
        assert!(s.contains("local.get $b"));
        assert!(s.contains("i32.add"));
    }
    #[test]
    pub(super) fn test_wasm_instr_display() {
        assert_eq!(format!("{}", WasmInstr::I32Const(42)), "i32.const 42");
        assert_eq!(format!("{}", WasmInstr::I64Const(-1)), "i64.const -1");
        assert_eq!(format!("{}", WasmInstr::F64Const(3.14)), "f64.const 3.14");
        assert_eq!(
            format!("{}", WasmInstr::LocalGet("x".to_string())),
            "local.get $x"
        );
        assert_eq!(
            format!("{}", WasmInstr::LocalSet("y".to_string())),
            "local.set $y"
        );
        assert_eq!(
            format!("{}", WasmInstr::Call("foo".to_string())),
            "call $foo"
        );
        assert_eq!(format!("{}", WasmInstr::I32Add), "i32.add");
        assert_eq!(format!("{}", WasmInstr::I32Sub), "i32.sub");
        assert_eq!(format!("{}", WasmInstr::I32Mul), "i32.mul");
        assert_eq!(format!("{}", WasmInstr::I32DivS), "i32.div_s");
        assert_eq!(format!("{}", WasmInstr::I64Add), "i64.add");
        assert_eq!(format!("{}", WasmInstr::I64Mul), "i64.mul");
        assert_eq!(format!("{}", WasmInstr::F64Add), "f64.add");
        assert_eq!(format!("{}", WasmInstr::F64Mul), "f64.mul");
        assert_eq!(format!("{}", WasmInstr::F64Div), "f64.div");
        assert_eq!(format!("{}", WasmInstr::F64Sqrt), "f64.sqrt");
        assert_eq!(format!("{}", WasmInstr::Return), "return");
        assert_eq!(format!("{}", WasmInstr::Drop), "drop");
        assert_eq!(format!("{}", WasmInstr::Nop), "nop");
        assert_eq!(format!("{}", WasmInstr::Unreachable), "unreachable");
        assert_eq!(format!("{}", WasmInstr::Select), "select");
        assert_eq!(format!("{}", WasmInstr::I32Eqz), "i32.eqz");
        assert_eq!(format!("{}", WasmInstr::I32Eq), "i32.eq");
        assert_eq!(format!("{}", WasmInstr::I32Ne), "i32.ne");
        assert_eq!(format!("{}", WasmInstr::I32LtS), "i32.lt_s");
        assert_eq!(format!("{}", WasmInstr::I32GtS), "i32.gt_s");
        assert_eq!(format!("{}", WasmInstr::I32LeS), "i32.le_s");
        assert_eq!(format!("{}", WasmInstr::I32GeS), "i32.ge_s");
        assert_eq!(format!("{}", WasmInstr::RefNull), "ref.null funcref");
        assert_eq!(format!("{}", WasmInstr::RefIsNull), "ref.is_null");
        assert_eq!(format!("{}", WasmInstr::BrIf(0)), "br_if 0");
        assert_eq!(format!("{}", WasmInstr::Block), "block");
        assert_eq!(format!("{}", WasmInstr::Loop), "loop");
        assert_eq!(format!("{}", WasmInstr::End), "end");
        assert_eq!(format!("{}", WasmInstr::MemLoad(4)), "i32.load align=4");
        assert_eq!(format!("{}", WasmInstr::MemStore(4)), "i32.store align=4");
        assert_eq!(format!("{}", WasmInstr::TableGet(0)), "table.get 0");
        assert_eq!(format!("{}", WasmInstr::TableSet(0)), "table.set 0");
        assert_eq!(format!("{}", WasmInstr::CallIndirect), "call_indirect");
    }
    #[test]
    pub(super) fn test_compile_simple_decl() {
        let decl = LcnfFunDecl {
            name: "Nat.succ".to_string(),
            original_name: None,
            params: vec![LcnfParam {
                id: LcnfVarId(0),
                ty: LcnfType::Nat,
                name: "n".to_string(),
                erased: false,
                borrowed: false,
            }],
            ret_type: LcnfType::Nat,
            body: LcnfExpr::Return(LcnfArg::Var(LcnfVarId(0))),
            is_recursive: false,
            is_lifted: false,
            inline_cost: 1,
        };
        let mut backend = WasmBackend::new();
        let func = backend.compile_decl(&decl);
        assert_eq!(func.name, "Nat_succ");
        assert_eq!(func.params.len(), 1);
        assert_eq!(func.params[0].0, "x0");
        assert_eq!(func.params[0].1, WasmType::I64);
        assert_eq!(func.results, vec![WasmType::I64]);
    }
    #[test]
    pub(super) fn test_emit_wasm_module() {
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
        let wat = WasmBackend::emit_module(&[decl]).expect("emit_module failed");
        assert!(wat.contains("(module"));
        assert!(wat.contains("func $answer"));
        assert!(wat.contains("export \"answer\""));
        assert!(wat.contains("i64.const 42"));
    }
    #[test]
    pub(super) fn test_wasm_types() {
        assert_eq!(format!("{}", WasmType::I32), "i32");
        assert_eq!(format!("{}", WasmType::I64), "i64");
        assert_eq!(format!("{}", WasmType::F32), "f32");
        assert_eq!(format!("{}", WasmType::F64), "f64");
        assert_eq!(format!("{}", WasmType::FuncRef), "funcref");
        assert_eq!(format!("{}", WasmType::ExternRef), "externref");
        assert_eq!(format!("{}", WasmType::V128), "v128");
        assert_eq!(
            WasmBackend::lcnf_type_to_wasm(&LcnfType::Nat),
            WasmType::I64
        );
        assert_eq!(
            WasmBackend::lcnf_type_to_wasm(&LcnfType::Object),
            WasmType::I32
        );
        assert_eq!(
            WasmBackend::lcnf_type_to_wasm(&LcnfType::LcnfString),
            WasmType::I32
        );
        assert_eq!(
            WasmBackend::lcnf_type_to_wasm(&LcnfType::Unit),
            WasmType::I32
        );
    }
    #[test]
    pub(super) fn test_memory_instructions() {
        let load4 = WasmInstr::MemLoad(4);
        let store8 = WasmInstr::MemStore(8);
        assert_eq!(format!("{}", load4), "i32.load align=4");
        assert_eq!(format!("{}", store8), "i32.store align=8");
        let mut module = WasmModule::new();
        module.memory = Some(2);
        let wat = module.to_wat();
        assert!(wat.contains("(memory 2)"));
        let module2 = WasmModule::new();
        let wat2 = module2.to_wat();
        assert!(!wat2.contains("(memory"));
    }
    #[test]
    pub(super) fn test_wasm_imports() {
        let mut module = WasmModule::new();
        module.imports.push(WasmImport {
            module: "env".to_string(),
            name: "log".to_string(),
            kind: WasmImportKind::Func {
                params: vec![WasmType::I32],
                results: vec![],
            },
        });
        module.imports.push(WasmImport {
            module: "js".to_string(),
            name: "mem".to_string(),
            kind: WasmImportKind::Memory { min_pages: 1 },
        });
        module.imports.push(WasmImport {
            module: "js".to_string(),
            name: "counter".to_string(),
            kind: WasmImportKind::Global {
                ty: WasmType::I32,
                mutable: true,
            },
        });
        let wat = module.to_wat();
        assert!(wat.contains(r#"import "env" "log""#));
        assert!(wat.contains(r#"import "js" "mem""#));
        assert!(wat.contains(r#"import "js" "counter""#));
        assert!(wat.contains("(func"));
        assert!(wat.contains("(memory 1)"));
        assert!(wat.contains("(global (mut i32))"));
        let import_imm = WasmImport {
            module: "env".to_string(),
            name: "val".to_string(),
            kind: WasmImportKind::Global {
                ty: WasmType::I64,
                mutable: false,
            },
        };
        let s = format!("{}", import_imm);
        assert!(s.contains("(global i64)"));
        assert!(!s.contains("mut"));
    }
}
#[cfg(test)]
mod Wasm_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = WasmPassConfig::new("test_pass", WasmPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = WasmPassStats::new();
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
        let mut reg = WasmPassRegistry::new();
        reg.register(WasmPassConfig::new("pass_a", WasmPassPhase::Analysis));
        reg.register(WasmPassConfig::new("pass_b", WasmPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = WasmAnalysisCache::new(10);
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
        let mut wl = WasmWorklist::new();
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
        let mut dt = WasmDominatorTree::new(5);
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
        let mut liveness = WasmLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(WasmConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(WasmConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(WasmConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            WasmConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(WasmConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = WasmDepGraph::new();
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
mod wasmext_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_wasmext_phase_order() {
        assert_eq!(WasmExtPassPhase::Early.order(), 0);
        assert_eq!(WasmExtPassPhase::Middle.order(), 1);
        assert_eq!(WasmExtPassPhase::Late.order(), 2);
        assert_eq!(WasmExtPassPhase::Finalize.order(), 3);
        assert!(WasmExtPassPhase::Early.is_early());
        assert!(!WasmExtPassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_wasmext_config_builder() {
        let c = WasmExtPassConfig::new("p")
            .with_phase(WasmExtPassPhase::Late)
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
    pub(super) fn test_wasmext_stats() {
        let mut s = WasmExtPassStats::new();
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
    pub(super) fn test_wasmext_registry() {
        let mut r = WasmExtPassRegistry::new();
        r.register(WasmExtPassConfig::new("a").with_phase(WasmExtPassPhase::Early));
        r.register(WasmExtPassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&WasmExtPassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_wasmext_cache() {
        let mut c = WasmExtCache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_wasmext_worklist() {
        let mut w = WasmExtWorklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_wasmext_dom_tree() {
        let mut dt = WasmExtDomTree::new(5);
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
    pub(super) fn test_wasmext_liveness() {
        let mut lv = WasmExtLiveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_wasmext_const_folder() {
        let mut cf = WasmExtConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_wasmext_dep_graph() {
        let mut g = WasmExtDepGraph::new(4);
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
mod wasmx2_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_wasmx2_phase_order() {
        assert_eq!(WasmX2PassPhase::Early.order(), 0);
        assert_eq!(WasmX2PassPhase::Middle.order(), 1);
        assert_eq!(WasmX2PassPhase::Late.order(), 2);
        assert_eq!(WasmX2PassPhase::Finalize.order(), 3);
        assert!(WasmX2PassPhase::Early.is_early());
        assert!(!WasmX2PassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_wasmx2_config_builder() {
        let c = WasmX2PassConfig::new("p")
            .with_phase(WasmX2PassPhase::Late)
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
    pub(super) fn test_wasmx2_stats() {
        let mut s = WasmX2PassStats::new();
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
    pub(super) fn test_wasmx2_registry() {
        let mut r = WasmX2PassRegistry::new();
        r.register(WasmX2PassConfig::new("a").with_phase(WasmX2PassPhase::Early));
        r.register(WasmX2PassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&WasmX2PassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_wasmx2_cache() {
        let mut c = WasmX2Cache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_wasmx2_worklist() {
        let mut w = WasmX2Worklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_wasmx2_dom_tree() {
        let mut dt = WasmX2DomTree::new(5);
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
    pub(super) fn test_wasmx2_liveness() {
        let mut lv = WasmX2Liveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_wasmx2_const_folder() {
        let mut cf = WasmX2ConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_wasmx2_dep_graph() {
        let mut g = WasmX2DepGraph::new(4);
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
