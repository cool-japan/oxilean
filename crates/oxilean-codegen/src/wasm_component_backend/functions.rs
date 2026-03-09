//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;

use super::types::{
    CanonicalOptions, ComponentExport, ComponentImport, ComponentInstance, CoreModule,
    WCAnalysisCache, WCConstantFoldingHelper, WCDepGraph, WCDominatorTree, WCLivenessInfo,
    WCPassConfig, WCPassPhase, WCPassRegistry, WCPassStats, WCWorklist, WasmCExtCache,
    WasmCExtConstFolder, WasmCExtDepGraph, WasmCExtDomTree, WasmCExtLiveness, WasmCExtPassConfig,
    WasmCExtPassPhase, WasmCExtPassRegistry, WasmCExtPassStats, WasmCExtWorklist,
    WasmCompExtConfig, WasmCompExtDiagCollector, WasmCompExtDiagMsg, WasmCompExtEmitStats,
    WasmCompExtEventLog, WasmCompExtFeatures, WasmCompExtIdGen, WasmCompExtIncrKey,
    WasmCompExtNameScope, WasmCompExtPassTiming, WasmCompExtProfiler, WasmCompExtSourceBuffer,
    WasmCompExtVersion, WasmComponentBackend, WasmComponentExpr, WasmComponentType, WitInterface,
    WitResource,
};

/// Build a simple WIT interface for math operations.
pub fn build_math_interface() -> WitInterface {
    let mut iface = WitInterface::new("oxilean:math/", "arithmetic");
    iface.add_type("num", WasmComponentType::F64);
    iface.add_function(
        "add",
        WasmComponentType::Func(
            vec![
                ("a".to_string(), WasmComponentType::F64),
                ("b".to_string(), WasmComponentType::F64),
            ],
            vec![("result".to_string(), WasmComponentType::F64)],
        ),
    );
    iface.add_function(
        "sqrt",
        WasmComponentType::Func(
            vec![("x".to_string(), WasmComponentType::F64)],
            vec![(
                "result".to_string(),
                WasmComponentType::Option(Box::new(WasmComponentType::F64)),
            )],
        ),
    );
    iface
}
/// Build a simple WASI-like streams interface for demonstration.
pub fn build_streams_interface() -> WitInterface {
    let mut iface = WitInterface::new("wasi:io/", "streams");
    iface.add_resource(
        WitResource::new("output-stream")
            .with_constructor(vec![])
            .with_method(
                "write",
                WasmComponentType::Func(
                    vec![(
                        "bytes".to_string(),
                        WasmComponentType::List(Box::new(WasmComponentType::U8)),
                    )],
                    vec![(
                        "result".to_string(),
                        WasmComponentType::Result(
                            Box::new(None),
                            Box::new(Some(WasmComponentType::TypeRef("stream-error".to_string()))),
                        ),
                    )],
                ),
            ),
    );
    iface.add_type(
        "stream-error",
        WasmComponentType::Variant(vec![
            ("last-operation-failed".to_string(), None),
            ("closed".to_string(), None),
        ]),
    );
    iface
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub(super) fn test_component_type_display_primitives() {
        assert_eq!(WasmComponentType::Bool.to_string(), "bool");
        assert_eq!(WasmComponentType::U32.to_string(), "u32");
        assert_eq!(WasmComponentType::S64.to_string(), "s64");
        assert_eq!(WasmComponentType::F32.to_string(), "f32");
        assert_eq!(WasmComponentType::String.to_string(), "string");
        assert_eq!(WasmComponentType::Char.to_string(), "char");
    }
    #[test]
    pub(super) fn test_component_type_display_compound() {
        let list_ty = WasmComponentType::List(Box::new(WasmComponentType::U8));
        assert_eq!(list_ty.to_string(), "list<u8>");
        let opt_ty = WasmComponentType::Option(Box::new(WasmComponentType::String));
        assert_eq!(opt_ty.to_string(), "option<string>");
        let res_ty = WasmComponentType::Result(
            Box::new(Some(WasmComponentType::U32)),
            Box::new(Some(WasmComponentType::String)),
        );
        assert_eq!(res_ty.to_string(), "result<u32, string>");
        let result_no_err =
            WasmComponentType::Result(Box::new(Some(WasmComponentType::Bool)), Box::new(None));
        assert_eq!(result_no_err.to_string(), "result<bool>");
    }
    #[test]
    pub(super) fn test_component_type_display_record() {
        let record = WasmComponentType::Record(vec![
            ("x".to_string(), WasmComponentType::F32),
            ("y".to_string(), WasmComponentType::F32),
        ]);
        let s = record.to_string();
        assert!(s.starts_with("record {"));
        assert!(s.contains("x: f32"));
        assert!(s.contains("y: f32"));
    }
    #[test]
    pub(super) fn test_component_type_display_variant() {
        let variant = WasmComponentType::Variant(vec![
            ("none".to_string(), None),
            ("some".to_string(), Some(WasmComponentType::U32)),
        ]);
        let s = variant.to_string();
        assert!(s.starts_with("variant {"));
        assert!(s.contains("none"));
        assert!(s.contains("some(u32)"));
    }
    #[test]
    pub(super) fn test_component_type_display_enum_flags() {
        let enum_ty = WasmComponentType::Enum(vec![
            "red".to_string(),
            "green".to_string(),
            "blue".to_string(),
        ]);
        let s = enum_ty.to_string();
        assert!(s.contains("red"));
        assert!(s.contains("green"));
        let flags_ty = WasmComponentType::Flags(vec![
            "read".to_string(),
            "write".to_string(),
            "exec".to_string(),
        ]);
        let fs = flags_ty.to_string();
        assert!(fs.contains("read"));
        assert!(fs.contains("write"));
    }
    #[test]
    pub(super) fn test_component_export_emit() {
        let export = ComponentExport::func("my-func", "my_func_impl");
        let s = export.emit();
        assert!(s.contains("my-func"));
        assert!(s.contains("func"));
        assert!(s.contains("my_func_impl"));
    }
    #[test]
    pub(super) fn test_component_instance_construction() {
        let mut inst = ComponentInstance::new("my-component");
        assert_eq!(inst.export_count(), 0);
        assert_eq!(inst.import_count(), 0);
        inst.add_export(ComponentExport::func("greet", "greet_impl"));
        inst.add_import(ComponentImport::func(
            "wasi:io/",
            "write",
            vec![(
                "data".to_string(),
                WasmComponentType::List(Box::new(WasmComponentType::U8)),
            )],
            vec![],
        ));
        assert_eq!(inst.export_count(), 1);
        assert_eq!(inst.import_count(), 1);
    }
    #[test]
    pub(super) fn test_wit_interface_emit() {
        let mut iface = WitInterface::new("oxilean:core/", "types");
        iface.add_type("idx", WasmComponentType::U32);
        iface.add_function(
            "identity",
            WasmComponentType::Func(
                vec![("x".to_string(), WasmComponentType::U32)],
                vec![("result".to_string(), WasmComponentType::U32)],
            ),
        );
        let wit = iface.emit_wit();
        assert!(wit.contains("interface types {"));
        assert!(wit.contains("type idx = u32"));
        assert!(wit.contains("identity:"));
        assert!(wit.contains("func(x: u32)"));
    }
    #[test]
    pub(super) fn test_backend_emit_component_wat() {
        let mut backend = WasmComponentBackend::new("my-app");
        backend.define_type("size", WasmComponentType::U64);
        backend.add_export(ComponentExport::func("run", "run_impl"));
        let wat = backend.emit_component_wat();
        assert!(wat.contains("(component $my-app"));
        assert!(wat.contains("$size"));
        assert!(wat.contains("run"));
    }
    #[test]
    pub(super) fn test_backend_wit_package() {
        let mut backend = WasmComponentBackend::new("math-lib");
        backend.add_interface(build_math_interface());
        let pkg = backend.emit_wit_package("oxilean:math", Some("0.1.1"));
        assert!(pkg.contains("package oxilean:math@0.1.1"));
        assert!(pkg.contains("interface arithmetic {"));
        assert!(pkg.contains("type num = f64"));
    }
    #[test]
    pub(super) fn test_core_module_inline() {
        let module =
            CoreModule::inline_text("core", "(module (func $add (param i32 i32) (result i32)))");
        let s = module.emit();
        assert!(s.contains("(core module $core"));
        assert!(s.contains("func $add"));
    }
    #[test]
    pub(super) fn test_canonical_options_emit() {
        let opts = CanonicalOptions::with_memory_and_realloc("$mem", "$realloc");
        let s = opts.emit();
        assert!(s.contains("(memory $mem)"));
        assert!(s.contains("(realloc $realloc)"));
    }
    #[test]
    pub(super) fn test_wit_resource_emit() {
        let resource = WitResource::new("connection")
            .with_constructor(vec![
                ("host".to_string(), WasmComponentType::String),
                ("port".to_string(), WasmComponentType::U16),
            ])
            .with_method(
                "send",
                WasmComponentType::Func(
                    vec![(
                        "data".to_string(),
                        WasmComponentType::List(Box::new(WasmComponentType::U8)),
                    )],
                    vec![(
                        "n".to_string(),
                        WasmComponentType::Result(
                            Box::new(Some(WasmComponentType::U32)),
                            Box::new(None),
                        ),
                    )],
                ),
            );
        let s = resource.emit_wit();
        assert!(s.contains("resource connection {"));
        assert!(s.contains("constructor(host: string, port: u16)"));
        assert!(s.contains("send:"));
    }
    #[test]
    pub(super) fn test_component_expr_display() {
        let expr = WasmComponentExpr::Call {
            instance: "wasi-io".to_string(),
            func: "write".to_string(),
            args: vec![
                WasmComponentExpr::Var("buf".to_string()),
                WasmComponentExpr::IntLit(1024),
            ],
        };
        let s = expr.to_string();
        assert!(s.contains("wasi-io.write"));
        assert!(s.contains("buf"));
        assert!(s.contains("1024"));
        let some_expr = WasmComponentExpr::OptionSome(Box::new(WasmComponentExpr::BoolLit(true)));
        assert_eq!(some_expr.to_string(), "(some true)");
        let none_expr = WasmComponentExpr::OptionNone;
        assert_eq!(none_expr.to_string(), "none");
    }
    #[test]
    pub(super) fn test_streams_interface() {
        let iface = build_streams_interface();
        let wit = iface.emit_wit();
        assert!(wit.contains("interface streams {"));
        assert!(wit.contains("resource output-stream {"));
        assert!(wit.contains("stream-error"));
    }
    #[test]
    pub(super) fn test_backend_add_lifted_func() {
        let mut backend = WasmComponentBackend::new("compute");
        backend.set_canonical_opts("$memory", "$realloc");
        backend.add_lifted_func(
            "compute-sum",
            "core_compute_sum",
            vec![
                ("a".to_string(), WasmComponentType::S64),
                ("b".to_string(), WasmComponentType::S64),
            ],
            vec![("result".to_string(), WasmComponentType::S64)],
        );
        assert_eq!(backend.func_count(), 1);
        let wat = backend.emit_component_wat();
        assert!(wat.contains("canon lift"));
        assert!(wat.contains("core_compute_sum"));
    }
}
#[cfg(test)]
mod tests_wasm_comp_ext_extra {
    use super::*;
    #[test]
    pub(super) fn test_wasm_comp_ext_config() {
        let mut cfg = WasmCompExtConfig::new();
        cfg.set("mode", "release");
        cfg.set("verbose", "true");
        assert_eq!(cfg.get("mode"), Some("release"));
        assert!(cfg.get_bool("verbose"));
        assert!(cfg.get_int("mode").is_none());
        assert_eq!(cfg.len(), 2);
    }
    #[test]
    pub(super) fn test_wasm_comp_ext_source_buffer() {
        let mut buf = WasmCompExtSourceBuffer::new();
        buf.push_line("fn main() {");
        buf.indent();
        buf.push_line("println!(\"hello\");");
        buf.dedent();
        buf.push_line("}");
        assert!(buf.as_str().contains("fn main()"));
        assert!(buf.as_str().contains("    println!"));
        assert_eq!(buf.line_count(), 3);
        buf.reset();
        assert!(buf.is_empty());
    }
    #[test]
    pub(super) fn test_wasm_comp_ext_name_scope() {
        let mut scope = WasmCompExtNameScope::new();
        assert!(scope.declare("x"));
        assert!(!scope.declare("x"));
        assert!(scope.is_declared("x"));
        let scope = scope.push_scope();
        assert_eq!(scope.depth(), 1);
        let mut scope = scope.pop_scope();
        assert_eq!(scope.depth(), 0);
        scope.declare("y");
        assert_eq!(scope.len(), 2);
    }
    #[test]
    pub(super) fn test_wasm_comp_ext_diag_collector() {
        let mut col = WasmCompExtDiagCollector::new();
        col.emit(WasmCompExtDiagMsg::warning("pass_a", "slow"));
        col.emit(WasmCompExtDiagMsg::error("pass_b", "fatal"));
        assert!(col.has_errors());
        assert_eq!(col.errors().len(), 1);
        assert_eq!(col.warnings().len(), 1);
        col.clear();
        assert!(col.is_empty());
    }
    #[test]
    pub(super) fn test_wasm_comp_ext_id_gen() {
        let mut gen = WasmCompExtIdGen::new();
        assert_eq!(gen.next_id(), 0);
        assert_eq!(gen.next_id(), 1);
        gen.skip(10);
        assert_eq!(gen.next_id(), 12);
        gen.reset();
        assert_eq!(gen.peek_next(), 0);
    }
    #[test]
    pub(super) fn test_wasm_comp_ext_incr_key() {
        let k1 = WasmCompExtIncrKey::new(100, 200);
        let k2 = WasmCompExtIncrKey::new(100, 200);
        let k3 = WasmCompExtIncrKey::new(999, 200);
        assert!(k1.matches(&k2));
        assert!(!k1.matches(&k3));
    }
    #[test]
    pub(super) fn test_wasm_comp_ext_profiler() {
        let mut p = WasmCompExtProfiler::new();
        p.record(WasmCompExtPassTiming::new("pass_a", 1000, 50, 200, 100));
        p.record(WasmCompExtPassTiming::new("pass_b", 500, 30, 100, 200));
        assert_eq!(p.total_elapsed_us(), 1500);
        assert_eq!(
            p.slowest_pass()
                .expect("slowest pass should exist")
                .pass_name,
            "pass_a"
        );
        assert_eq!(p.profitable_passes().len(), 1);
    }
    #[test]
    pub(super) fn test_wasm_comp_ext_event_log() {
        let mut log = WasmCompExtEventLog::new(3);
        log.push("event1");
        log.push("event2");
        log.push("event3");
        assert_eq!(log.len(), 3);
        log.push("event4");
        assert_eq!(log.len(), 3);
        assert_eq!(
            log.iter()
                .next()
                .expect("iterator should have next element"),
            "event2"
        );
    }
    #[test]
    pub(super) fn test_wasm_comp_ext_version() {
        let v = WasmCompExtVersion::new(1, 2, 3).with_pre("alpha");
        assert!(!v.is_stable());
        assert_eq!(format!("{}", v), "1.2.3-alpha");
        let stable = WasmCompExtVersion::new(2, 0, 0);
        assert!(stable.is_stable());
        assert!(stable.is_compatible_with(&WasmCompExtVersion::new(2, 0, 0)));
        assert!(!stable.is_compatible_with(&WasmCompExtVersion::new(3, 0, 0)));
    }
    #[test]
    pub(super) fn test_wasm_comp_ext_features() {
        let mut f = WasmCompExtFeatures::new();
        f.enable("sse2");
        f.enable("avx2");
        assert!(f.is_enabled("sse2"));
        assert!(!f.is_enabled("avx512"));
        f.disable("avx2");
        assert!(!f.is_enabled("avx2"));
        let mut g = WasmCompExtFeatures::new();
        g.enable("sse2");
        g.enable("neon");
        let union = f.union(&g);
        assert!(union.is_enabled("sse2") && union.is_enabled("neon"));
        let inter = f.intersection(&g);
        assert!(inter.is_enabled("sse2"));
    }
    #[test]
    pub(super) fn test_wasm_comp_ext_emit_stats() {
        let mut s = WasmCompExtEmitStats::new();
        s.bytes_emitted = 50_000;
        s.items_emitted = 500;
        s.elapsed_ms = 100;
        assert!(s.is_clean());
        assert!((s.throughput_bps() - 500_000.0).abs() < 1.0);
        let disp = format!("{}", s);
        assert!(disp.contains("bytes=50000"));
    }
}
#[cfg(test)]
mod WC_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = WCPassConfig::new("test_pass", WCPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = WCPassStats::new();
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
        let mut reg = WCPassRegistry::new();
        reg.register(WCPassConfig::new("pass_a", WCPassPhase::Analysis));
        reg.register(WCPassConfig::new("pass_b", WCPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = WCAnalysisCache::new(10);
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
        let mut wl = WCWorklist::new();
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
        let mut dt = WCDominatorTree::new(5);
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
        let mut liveness = WCLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(WCConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(WCConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(WCConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            WCConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(WCConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = WCDepGraph::new();
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
mod wasmcext_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_wasmcext_phase_order() {
        assert_eq!(WasmCExtPassPhase::Early.order(), 0);
        assert_eq!(WasmCExtPassPhase::Middle.order(), 1);
        assert_eq!(WasmCExtPassPhase::Late.order(), 2);
        assert_eq!(WasmCExtPassPhase::Finalize.order(), 3);
        assert!(WasmCExtPassPhase::Early.is_early());
        assert!(!WasmCExtPassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_wasmcext_config_builder() {
        let c = WasmCExtPassConfig::new("p")
            .with_phase(WasmCExtPassPhase::Late)
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
    pub(super) fn test_wasmcext_stats() {
        let mut s = WasmCExtPassStats::new();
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
    pub(super) fn test_wasmcext_registry() {
        let mut r = WasmCExtPassRegistry::new();
        r.register(WasmCExtPassConfig::new("a").with_phase(WasmCExtPassPhase::Early));
        r.register(WasmCExtPassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&WasmCExtPassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_wasmcext_cache() {
        let mut c = WasmCExtCache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_wasmcext_worklist() {
        let mut w = WasmCExtWorklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_wasmcext_dom_tree() {
        let mut dt = WasmCExtDomTree::new(5);
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
    pub(super) fn test_wasmcext_liveness() {
        let mut lv = WasmCExtLiveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_wasmcext_const_folder() {
        let mut cf = WasmCExtConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_wasmcext_dep_graph() {
        let mut g = WasmCExtDepGraph::new(4);
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
