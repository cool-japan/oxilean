//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    ZigAllocatorKind, ZigAllocatorUsage, ZigAnalysisCache, ZigAsyncFn, ZigBackend,
    ZigBuildConfiguration, ZigComptime, ZigConstantFoldingHelper, ZigDepGraph, ZigDominatorTree,
    ZigErrorSet, ZigExpr, ZigExtCache, ZigExtConstFolder, ZigExtDepGraph, ZigExtDomTree,
    ZigExtLiveness, ZigExtPassConfig, ZigExtPassPhase, ZigExtPassRegistry, ZigExtPassStats,
    ZigExtWorklist, ZigFn, ZigGenericFn, ZigImport, ZigLivenessInfo, ZigModule, ZigOptimizeMode,
    ZigPackedStruct, ZigPassConfig, ZigPassPhase, ZigPassRegistry, ZigPassStats, ZigSliceOps,
    ZigStmt, ZigStruct, ZigTaggedUnion, ZigTestBlock, ZigType, ZigWorklist,
};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub(super) fn test_zig_type_codegen_primitives() {
        assert_eq!(ZigType::Void.codegen(), "void");
        assert_eq!(ZigType::Bool.codegen(), "bool");
        assert_eq!(ZigType::U8.codegen(), "u8");
        assert_eq!(ZigType::U64.codegen(), "u64");
        assert_eq!(ZigType::I64.codegen(), "i64");
        assert_eq!(ZigType::F64.codegen(), "f64");
        assert_eq!(ZigType::Anyopaque.codegen(), "anyopaque");
    }
    #[test]
    pub(super) fn test_zig_type_codegen_composite() {
        let ptr = ZigType::Ptr(Box::new(ZigType::U8));
        assert_eq!(ptr.codegen(), "*u8");
        let slice = ZigType::Slice(Box::new(ZigType::I64));
        assert_eq!(slice.codegen(), "[]i64");
        let opt = ZigType::Optional(Box::new(ZigType::Bool));
        assert_eq!(opt.codegen(), "?bool");
        let eu = ZigType::ErrorUnion(Box::new(ZigType::Void));
        assert_eq!(eu.codegen(), "!void");
    }
    #[test]
    pub(super) fn test_zig_type_fn() {
        let fn_ty = ZigType::Fn(vec![ZigType::U64, ZigType::Bool], Box::new(ZigType::I64));
        let out = fn_ty.codegen();
        assert!(out.contains("fn("));
        assert!(out.contains("u64"));
        assert!(out.contains("bool"));
        assert!(out.contains("i64"));
    }
    #[test]
    pub(super) fn test_zig_expr_literals() {
        assert_eq!(ZigExpr::IntLit(42).codegen(), "42");
        assert_eq!(ZigExpr::BoolLit(true).codegen(), "true");
        assert_eq!(ZigExpr::BoolLit(false).codegen(), "false");
        assert_eq!(ZigExpr::NullLit.codegen(), "null");
        assert_eq!(
            ZigExpr::StringLit("hello".to_string()).codegen(),
            "\"hello\""
        );
        assert_eq!(ZigExpr::Ident("foo".to_string()).codegen(), "foo");
    }
    #[test]
    pub(super) fn test_zig_expr_binop() {
        let expr = ZigExpr::BinOp {
            op: "+".to_string(),
            lhs: Box::new(ZigExpr::IntLit(1)),
            rhs: Box::new(ZigExpr::IntLit(2)),
        };
        assert_eq!(expr.codegen(), "(1 + 2)");
    }
    #[test]
    pub(super) fn test_zig_expr_call() {
        let expr = ZigExpr::Call {
            callee: Box::new(ZigExpr::Ident("foo".to_string())),
            args: vec![ZigExpr::IntLit(1), ZigExpr::IntLit(2)],
        };
        assert_eq!(expr.codegen(), "foo(1, 2)");
    }
    #[test]
    pub(super) fn test_zig_fn_codegen() {
        let mut f = ZigFn::new("add", ZigType::U64);
        f.add_param("a", ZigType::U64);
        f.add_param("b", ZigType::U64);
        f.add_stmt(ZigStmt::Return(Some(ZigExpr::BinOp {
            op: "+".to_string(),
            lhs: Box::new(ZigExpr::Ident("a".to_string())),
            rhs: Box::new(ZigExpr::Ident("b".to_string())),
        })));
        let out = f.codegen();
        assert!(out.contains("fn add("));
        assert!(out.contains("a: u64"));
        assert!(out.contains("b: u64"));
        assert!(out.contains("u64"));
        assert!(out.contains("return"));
    }
    #[test]
    pub(super) fn test_zig_struct_codegen() {
        let mut s = ZigStruct::new("Point");
        s.add_field("x", ZigType::F64);
        s.add_field("y", ZigType::F64);
        let out = s.codegen();
        assert!(out.contains("const Point = struct"));
        assert!(out.contains("x: f64"));
        assert!(out.contains("y: f64"));
    }
    #[test]
    pub(super) fn test_zig_module_codegen() {
        let mut m = ZigModule::new("mymod");
        m.add_import("std");
        let mut s = ZigStruct::new("Foo");
        s.add_field("val", ZigType::U64);
        m.add_struct(s);
        let mut f = ZigFn::new("bar", ZigType::Void);
        f.add_stmt(ZigStmt::Return(None));
        m.add_fn(f);
        let out = m.codegen();
        assert!(out.contains("@import(\"std\")"));
        assert!(out.contains("const Foo = struct"));
        assert!(out.contains("fn bar()"));
    }
    #[test]
    pub(super) fn test_compile_name_sanitize() {
        assert_eq!(ZigBackend::compile_name("Foo.bar"), "Foo_bar");
        assert_eq!(ZigBackend::compile_name("123abc"), "ox_123abc");
        assert_eq!(ZigBackend::compile_name(""), "ox_empty");
        assert_eq!(ZigBackend::compile_name("fn"), "ox_fn");
        assert_eq!(ZigBackend::compile_name("const"), "ox_const");
        assert_eq!(ZigBackend::compile_name("my_var"), "my_var");
    }
}
#[allow(dead_code)]
pub fn zig_builtin_type_of(expr: &str) -> String {
    format!("@TypeOf({})", expr)
}
#[allow(dead_code)]
pub fn zig_builtin_int_cast(ty: &str, val: &str) -> String {
    format!("@intCast({}, {})", ty, val)
}
#[allow(dead_code)]
pub fn zig_builtin_float_cast(ty: &str, val: &str) -> String {
    format!("@floatCast({}, {})", ty, val)
}
#[allow(dead_code)]
pub fn zig_builtin_ptrcast(ty: &str, val: &str) -> String {
    format!("@ptrCast({}, {})", ty, val)
}
#[allow(dead_code)]
pub fn zig_builtin_size_of(ty: &str) -> String {
    format!("@sizeOf({})", ty)
}
#[allow(dead_code)]
pub fn zig_builtin_align_of(ty: &str) -> String {
    format!("@alignOf({})", ty)
}
#[allow(dead_code)]
pub fn zig_builtin_has_decl(ty: &str, name: &str) -> String {
    format!("@hasDecl({}, \"{}\")", ty, name)
}
#[allow(dead_code)]
pub fn zig_builtin_field(val: &str, field: &str) -> String {
    format!("@field({}, \"{}\")", val, field)
}
#[cfg(test)]
mod zig_extended_tests {
    use super::*;
    #[test]
    pub(super) fn test_error_set() {
        let mut es = ZigErrorSet::new("FileError");
        es.add_error("NotFound");
        es.add_error("AccessDenied");
        let s = es.emit();
        assert!(s.contains("const FileError = error{"));
        assert!(s.contains("NotFound"));
        assert!(s.contains("AccessDenied"));
    }
    #[test]
    pub(super) fn test_tagged_union() {
        let mut u = ZigTaggedUnion::new("Shape");
        u.add_field("circle", None);
        u.add_field("rectangle", None);
        let s = u.emit();
        assert!(s.contains("const Shape = union"));
        assert!(s.contains("circle"));
    }
    #[test]
    pub(super) fn test_packed_struct() {
        let mut ps = ZigPackedStruct::new("Flags");
        ps.add_field("enabled", ZigType::Bool, Some(1));
        ps.add_field("value", ZigType::Int, Some(7));
        assert_eq!(ps.total_bits(), 8);
        let s = ps.emit();
        assert!(s.contains("const Flags = packed struct"));
        assert!(s.contains("u1"));
        assert!(s.contains("u7"));
    }
    #[test]
    pub(super) fn test_generic_fn() {
        let f = ZigGenericFn::new("max")
            .type_param("T")
            .param("a", "T")
            .param("b", "T")
            .returns("T");
        let s = f.emit();
        assert!(s.contains("fn max("));
        assert!(s.contains("comptime T: type"));
    }
    #[test]
    pub(super) fn test_builtins() {
        assert_eq!(zig_builtin_size_of("u32"), "@sizeOf(u32)");
        assert_eq!(zig_builtin_type_of("x"), "@TypeOf(x)");
        assert_eq!(zig_builtin_int_cast("u8", "val"), "@intCast(u8, val)");
    }
    #[test]
    pub(super) fn test_test_block() {
        let t = ZigTestBlock::new("basic add").add_expect("1 + 1", "2");
        let s = t.emit();
        assert!(s.contains("test \"basic add\""));
        assert!(s.contains("expectEqual"));
    }
    #[test]
    pub(super) fn test_allocator() {
        let alloc = ZigAllocatorUsage::new(ZigAllocatorKind::GeneralPurpose, "gpa");
        let init = alloc.emit_init();
        assert!(init.contains("GeneralPurposeAllocator"));
        assert!(init.contains("gpa"));
        let iface = alloc.emit_interface_call();
        assert!(iface.contains("allocator()"));
    }
    #[test]
    pub(super) fn test_slice_ops() {
        assert_eq!(ZigSliceOps::len_expr("items"), "items.len");
        assert_eq!(ZigSliceOps::index_expr("arr", "i"), "arr[i]");
        assert_eq!(ZigSliceOps::slice_expr("buf", "0", "n"), "buf[0..n]");
        assert_eq!(ZigSliceOps::eql("a", "b"), "std.mem.eql(u8, a, b)");
    }
    #[test]
    pub(super) fn test_import() {
        let imp = ZigImport::std();
        assert_eq!(imp.emit(), "const std = @import(\"std\");");
    }
    #[test]
    pub(super) fn test_build_configuration() {
        let cfg = ZigBuildConfiguration::new("src/main.zig").optimize(ZigOptimizeMode::ReleaseFast);
        let build_zig = cfg.emit_build_zig();
        assert!(build_zig.contains("pub fn build"));
        assert!(build_zig.contains("src/main.zig"));
    }
    #[test]
    pub(super) fn test_async_fn() {
        let f = ZigAsyncFn::new("fetchData").param("url", "[]const u8");
        let s = f.emit();
        assert!(s.contains("async fn fetchData"));
        assert!(s.contains("url: []const u8"));
        let await_call = f.emit_await_call(&["\"https://example.com\""]);
        assert!(await_call.contains("await async fetchData"));
    }
    #[test]
    pub(super) fn test_comptime_block() {
        let ct = ZigComptime::new();
        let s = ct.emit();
        assert!(s.starts_with("comptime {"));
    }
}
#[cfg(test)]
mod Zig_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = ZigPassConfig::new("test_pass", ZigPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = ZigPassStats::new();
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
        let mut reg = ZigPassRegistry::new();
        reg.register(ZigPassConfig::new("pass_a", ZigPassPhase::Analysis));
        reg.register(ZigPassConfig::new("pass_b", ZigPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = ZigAnalysisCache::new(10);
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
        let mut wl = ZigWorklist::new();
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
        let mut dt = ZigDominatorTree::new(5);
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
        let mut liveness = ZigLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(ZigConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(ZigConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(ZigConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            ZigConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(ZigConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = ZigDepGraph::new();
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
mod zigext_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_zigext_phase_order() {
        assert_eq!(ZigExtPassPhase::Early.order(), 0);
        assert_eq!(ZigExtPassPhase::Middle.order(), 1);
        assert_eq!(ZigExtPassPhase::Late.order(), 2);
        assert_eq!(ZigExtPassPhase::Finalize.order(), 3);
        assert!(ZigExtPassPhase::Early.is_early());
        assert!(!ZigExtPassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_zigext_config_builder() {
        let c = ZigExtPassConfig::new("p")
            .with_phase(ZigExtPassPhase::Late)
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
    pub(super) fn test_zigext_stats() {
        let mut s = ZigExtPassStats::new();
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
    pub(super) fn test_zigext_registry() {
        let mut r = ZigExtPassRegistry::new();
        r.register(ZigExtPassConfig::new("a").with_phase(ZigExtPassPhase::Early));
        r.register(ZigExtPassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&ZigExtPassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_zigext_cache() {
        let mut c = ZigExtCache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_zigext_worklist() {
        let mut w = ZigExtWorklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_zigext_dom_tree() {
        let mut dt = ZigExtDomTree::new(5);
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
    pub(super) fn test_zigext_liveness() {
        let mut lv = ZigExtLiveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_zigext_const_folder() {
        let mut cf = ZigExtConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_zigext_dep_graph() {
        let mut g = ZigExtDepGraph::new(4);
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
