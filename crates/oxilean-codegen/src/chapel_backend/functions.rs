//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;

use super::types::{
    ChapelBackend, ChapelClass, ChapelExpr, ChapelExtCache, ChapelExtConstFolder,
    ChapelExtDepGraph, ChapelExtDomTree, ChapelExtLiveness, ChapelExtPassConfig,
    ChapelExtPassPhase, ChapelExtPassRegistry, ChapelExtPassStats, ChapelExtWorklist, ChapelField,
    ChapelIntent, ChapelModule, ChapelParam, ChapelProc, ChapelRecord, ChapelStmt, ChapelType,
    ChplAnalysisCache, ChplConstantFoldingHelper, ChplDepGraph, ChplDominatorTree,
    ChplLivenessInfo, ChplPassConfig, ChplPassPhase, ChplPassRegistry, ChplPassStats, ChplWorklist,
};

#[cfg(test)]
mod tests {
    use super::*;
    pub(super) fn var(s: &str) -> ChapelExpr {
        ChapelExpr::Var(s.to_string())
    }
    pub(super) fn int(n: i64) -> ChapelExpr {
        ChapelExpr::IntLit(n)
    }
    #[test]
    pub(super) fn test_type_display() {
        assert_eq!(ChapelType::Int(None).to_string(), "int");
        assert_eq!(ChapelType::Int(Some(32)).to_string(), "int(32)");
        assert_eq!(ChapelType::Real(Some(64)).to_string(), "real(64)");
        assert_eq!(ChapelType::Bool.to_string(), "bool");
        assert_eq!(ChapelType::String.to_string(), "string");
        assert_eq!(ChapelType::Void.to_string(), "void");
        let dom = ChapelType::Domain(2, Box::new(ChapelType::Int(None)));
        assert_eq!(dom.to_string(), "domain(2, int)");
        let arr = ChapelType::Array(
            Box::new(ChapelType::Named("D".to_string())),
            Box::new(ChapelType::Real(None)),
        );
        assert_eq!(arr.to_string(), "[D] real");
        let owned = ChapelType::Owned(Box::new(ChapelType::Named("MyClass".to_string())));
        assert_eq!(owned.to_string(), "owned MyClass");
        let atomic = ChapelType::Atomic(Box::new(ChapelType::Int(None)));
        assert_eq!(atomic.to_string(), "atomic int");
    }
    #[test]
    pub(super) fn test_emit_expressions() {
        let mut be = ChapelBackend::new();
        be.emit_expr(&ChapelExpr::BinOp(
            "+".to_string(),
            Box::new(int(1)),
            Box::new(int(2)),
        ));
        assert_eq!(be.finish(), "1 + 2");
        let mut be = ChapelBackend::new();
        be.emit_expr(&ChapelExpr::ReduceExpr(
            "+".to_string(),
            Box::new(var("arr")),
        ));
        assert_eq!(be.finish(), "+ reduce arr");
        let mut be = ChapelBackend::new();
        be.emit_expr(&ChapelExpr::RangeLit(
            Box::new(int(1)),
            Box::new(int(10)),
            false,
        ));
        assert_eq!(be.finish(), "1..10");
        let mut be = ChapelBackend::new();
        be.emit_expr(&ChapelExpr::ForallExpr(
            "i".to_string(),
            Box::new(ChapelExpr::DomainLit(vec![ChapelExpr::RangeLit(
                Box::new(int(0)),
                Box::new(int(9)),
                false,
            )])),
            Box::new(ChapelExpr::Apply(Box::new(var("f")), vec![var("i")])),
        ));
        let out = be.finish();
        assert!(out.contains("[i in"), "forall expr: {out}");
    }
    #[test]
    pub(super) fn test_var_const_decl() {
        let mut be = ChapelBackend::new();
        be.emit_stmt(&ChapelStmt::VarDecl(
            "x".to_string(),
            Some(ChapelType::Int(None)),
            Some(int(42)),
        ));
        let out = be.finish();
        assert!(out.contains("var x"), "var: {out}");
        assert!(out.contains("42"), "value: {out}");
        let mut be = ChapelBackend::new();
        be.emit_stmt(&ChapelStmt::ConstDecl(
            "PI".to_string(),
            Some(ChapelType::Real(None)),
            ChapelExpr::RealLit(3.14159),
        ));
        let out = be.finish();
        assert!(out.contains("const PI"), "const: {out}");
        assert!(out.contains("3.14159"), "value: {out}");
    }
    #[test]
    pub(super) fn test_forall_coforall() {
        let mut be = ChapelBackend::new();
        be.emit_stmt(&ChapelStmt::ForallLoop(
            "i".to_string(),
            ChapelExpr::DomainLit(vec![ChapelExpr::RangeLit(
                Box::new(int(0)),
                Box::new(var("n")),
                false,
            )]),
            vec![ChapelStmt::CompoundAssign(
                "+".to_string(),
                ChapelExpr::Index(Box::new(var("a")), Box::new(var("i"))),
                ChapelExpr::IntLit(1),
            )],
        ));
        let out = be.finish();
        assert!(out.contains("forall i in"), "forall: {out}");
        assert!(out.contains("+="), "compound assign: {out}");
        let mut be = ChapelBackend::new();
        be.emit_stmt(&ChapelStmt::CoforallLoop(
            "loc".to_string(),
            ChapelExpr::Var("Locales".to_string()),
            vec![ChapelStmt::ExprStmt(ChapelExpr::Apply(
                Box::new(var("doWork")),
                vec![var("loc")],
            ))],
        ));
        let out = be.finish();
        assert!(out.contains("coforall loc in Locales"), "coforall: {out}");
        assert!(out.contains("doWork"), "body: {out}");
    }
    #[test]
    pub(super) fn test_record_def() {
        let mut rec = ChapelRecord::new("Point");
        rec.add_field(ChapelField {
            name: "x".to_string(),
            ty: ChapelType::Real(None),
            is_const: false,
            default: Some(ChapelExpr::RealLit(0.0)),
        });
        rec.add_field(ChapelField {
            name: "y".to_string(),
            ty: ChapelType::Real(None),
            is_const: false,
            default: Some(ChapelExpr::RealLit(0.0)),
        });
        rec.add_method(ChapelProc::new(
            "norm",
            vec![],
            Some(ChapelType::Real(None)),
            vec![ChapelStmt::ReturnStmt(Some(ChapelExpr::Apply(
                Box::new(var("sqrt")),
                vec![ChapelExpr::BinOp(
                    "+".to_string(),
                    Box::new(ChapelExpr::BinOp(
                        "*".to_string(),
                        Box::new(ChapelExpr::FieldAccess(
                            Box::new(ChapelExpr::This),
                            "x".to_string(),
                        )),
                        Box::new(ChapelExpr::FieldAccess(
                            Box::new(ChapelExpr::This),
                            "x".to_string(),
                        )),
                    )),
                    Box::new(ChapelExpr::BinOp(
                        "*".to_string(),
                        Box::new(ChapelExpr::FieldAccess(
                            Box::new(ChapelExpr::This),
                            "y".to_string(),
                        )),
                        Box::new(ChapelExpr::FieldAccess(
                            Box::new(ChapelExpr::This),
                            "y".to_string(),
                        )),
                    )),
                )],
            )))],
        ));
        let mut be = ChapelBackend::new();
        be.emit_record(&rec);
        let out = be.finish();
        assert!(out.contains("record Point"), "record: {out}");
        assert!(out.contains("var x: real"), "field x: {out}");
        assert!(out.contains("var y: real"), "field y: {out}");
        assert!(out.contains("proc norm"), "method: {out}");
        assert!(out.contains("sqrt"), "body: {out}");
    }
    #[test]
    pub(super) fn test_proc_def() {
        let proc = ChapelProc::new(
            "sumArray",
            vec![ChapelParam::with_intent(
                "arr",
                ChapelType::Array(
                    Box::new(ChapelType::Named("D".to_string())),
                    Box::new(ChapelType::Real(None)),
                ),
                ChapelIntent::Ref,
            )],
            Some(ChapelType::Real(None)),
            vec![
                ChapelStmt::VarDecl(
                    "total".to_string(),
                    Some(ChapelType::Real(None)),
                    Some(ChapelExpr::RealLit(0.0)),
                ),
                ChapelStmt::ForallReduce(
                    "i".to_string(),
                    ChapelExpr::FieldAccess(Box::new(var("arr")), "domain".to_string()),
                    "+".to_string(),
                    "total".to_string(),
                    vec![ChapelStmt::CompoundAssign(
                        "+".to_string(),
                        var("total"),
                        ChapelExpr::Index(Box::new(var("arr")), Box::new(var("i"))),
                    )],
                ),
                ChapelStmt::ReturnStmt(Some(var("total"))),
            ],
        );
        let mut be = ChapelBackend::new();
        be.emit_proc(&proc);
        let out = be.finish();
        assert!(out.contains("proc sumArray"), "proc name: {out}");
        assert!(out.contains("ref arr"), "intent: {out}");
        assert!(out.contains("+ reduce total"), "reduce clause: {out}");
        assert!(out.contains("return total"), "return: {out}");
    }
    #[test]
    pub(super) fn test_full_module() {
        let mut module = ChapelModule::named("ParallelSort");
        module.set_doc("Parallel sorting utilities");
        module.add_use("Sort");
        module.add_config(
            "numTasks",
            ChapelType::Int(None),
            Some(ChapelExpr::IntLit(4)),
        );
        module.add_global(
            "VERSION",
            ChapelType::String,
            ChapelExpr::StrLit("1.0".to_string()),
        );
        module.add_proc(ChapelProc::new(
            "main",
            vec![],
            None,
            vec![
                ChapelStmt::VarDecl(
                    "D".to_string(),
                    Some(ChapelType::Domain(1, Box::new(ChapelType::Int(None)))),
                    Some(ChapelExpr::DomainLit(vec![ChapelExpr::RangeLit(
                        Box::new(int(1)),
                        Box::new(int(100)),
                        false,
                    )])),
                ),
                ChapelStmt::Writeln(vec![ChapelExpr::StrLit("done".to_string())]),
            ],
        ));
        let src = ChapelBackend::generate(&module);
        assert!(src.contains("module ParallelSort"), "module name: {src}");
        assert!(src.contains("use Sort"), "use: {src}");
        assert!(src.contains("config var numTasks"), "config: {src}");
        assert!(src.contains("const VERSION"), "global: {src}");
        assert!(src.contains("proc main"), "main: {src}");
        assert!(src.contains("writeln"), "writeln: {src}");
        assert!(src.contains("}"), "closing brace: {src}");
    }
    #[test]
    pub(super) fn test_intent_display_and_class() {
        assert_eq!(ChapelIntent::In.to_string(), "in");
        assert_eq!(ChapelIntent::Out.to_string(), "out");
        assert_eq!(ChapelIntent::InOut.to_string(), "inout");
        assert_eq!(ChapelIntent::Ref.to_string(), "ref");
        assert_eq!(ChapelIntent::ConstRef.to_string(), "const ref");
        assert_eq!(ChapelIntent::Param.to_string(), "param");
        assert_eq!(ChapelIntent::Type.to_string(), "type");
        let mut cls = ChapelClass::new("Animal").with_parent("Base");
        cls.add_field(ChapelField {
            name: "name".to_string(),
            ty: ChapelType::String,
            is_const: true,
            default: None,
        });
        cls.add_method(ChapelProc::new(
            "speak",
            vec![],
            None,
            vec![ChapelStmt::Halt("abstract".to_string())],
        ));
        let mut be = ChapelBackend::new();
        be.emit_class(&cls);
        let out = be.finish();
        assert!(out.contains("class Animal"), "class: {out}");
        assert!(out.contains(": Base"), "parent: {out}");
        assert!(out.contains("const name: string"), "field: {out}");
        assert!(out.contains("halt"), "halt: {out}");
    }
}
#[cfg(test)]
mod Chpl_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = ChplPassConfig::new("test_pass", ChplPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = ChplPassStats::new();
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
        let mut reg = ChplPassRegistry::new();
        reg.register(ChplPassConfig::new("pass_a", ChplPassPhase::Analysis));
        reg.register(ChplPassConfig::new("pass_b", ChplPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = ChplAnalysisCache::new(10);
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
        let mut wl = ChplWorklist::new();
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
        let mut dt = ChplDominatorTree::new(5);
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
        let mut liveness = ChplLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(ChplConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(ChplConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(ChplConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            ChplConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(ChplConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = ChplDepGraph::new();
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
mod chapelext_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_chapelext_phase_order() {
        assert_eq!(ChapelExtPassPhase::Early.order(), 0);
        assert_eq!(ChapelExtPassPhase::Middle.order(), 1);
        assert_eq!(ChapelExtPassPhase::Late.order(), 2);
        assert_eq!(ChapelExtPassPhase::Finalize.order(), 3);
        assert!(ChapelExtPassPhase::Early.is_early());
        assert!(!ChapelExtPassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_chapelext_config_builder() {
        let c = ChapelExtPassConfig::new("p")
            .with_phase(ChapelExtPassPhase::Late)
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
    pub(super) fn test_chapelext_stats() {
        let mut s = ChapelExtPassStats::new();
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
    pub(super) fn test_chapelext_registry() {
        let mut r = ChapelExtPassRegistry::new();
        r.register(ChapelExtPassConfig::new("a").with_phase(ChapelExtPassPhase::Early));
        r.register(ChapelExtPassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&ChapelExtPassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_chapelext_cache() {
        let mut c = ChapelExtCache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_chapelext_worklist() {
        let mut w = ChapelExtWorklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_chapelext_dom_tree() {
        let mut dt = ChapelExtDomTree::new(5);
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
    pub(super) fn test_chapelext_liveness() {
        let mut lv = ChapelExtLiveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_chapelext_const_folder() {
        let mut cf = ChapelExtConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_chapelext_dep_graph() {
        let mut g = ChapelExtDepGraph::new(4);
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
