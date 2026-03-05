//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    DhallAnalysisCache, DhallBackend, DhallConstantFoldingHelper, DhallDecl, DhallDepGraph,
    DhallDominatorTree, DhallExpr, DhallExtCache, DhallExtConstFolder, DhallExtDepGraph,
    DhallExtDomTree, DhallExtLiveness, DhallExtPassConfig, DhallExtPassPhase, DhallExtPassRegistry,
    DhallExtPassStats, DhallExtWorklist, DhallFile, DhallFunction, DhallImport, DhallLivenessInfo,
    DhallPassConfig, DhallPassPhase, DhallPassRegistry, DhallPassStats, DhallRecord, DhallType,
    DhallWorklist, DhallX2Cache, DhallX2ConstFolder, DhallX2DepGraph, DhallX2DomTree,
    DhallX2Liveness, DhallX2PassConfig, DhallX2PassPhase, DhallX2PassRegistry, DhallX2PassStats,
    DhallX2Worklist,
};

/// Escape characters inside a Dhall double-quoted string.
pub(super) fn escape_dhall_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace("${", "\\${")
        .replace('\n', "\\n")
        .replace('\t', "\\t")
}
/// Shorthand: boolean literal.
pub fn dhall_bool(b: bool) -> DhallExpr {
    DhallExpr::BoolLit(b)
}
/// Shorthand: natural number literal.
pub fn dhall_nat(n: u64) -> DhallExpr {
    DhallExpr::NaturalLit(n)
}
/// Shorthand: integer literal.
pub fn dhall_int(n: i64) -> DhallExpr {
    DhallExpr::IntegerLit(n)
}
/// Shorthand: text literal.
pub fn dhall_text(s: &str) -> DhallExpr {
    DhallExpr::TextLit(s.to_string())
}
/// Shorthand: variable.
pub fn dhall_var(name: &str) -> DhallExpr {
    DhallExpr::Var(name.to_string())
}
/// Shorthand: function application.
pub fn dhall_app(f: DhallExpr, x: DhallExpr) -> DhallExpr {
    DhallExpr::Application(Box::new(f), Box::new(x))
}
/// Shorthand: lambda.
pub fn dhall_lam(label: &str, ty: DhallType, body: DhallExpr) -> DhallExpr {
    DhallExpr::Lambda(Box::new(DhallFunction::new(label, ty, body)))
}
/// Shorthand: if-then-else.
pub fn dhall_if(cond: DhallExpr, t: DhallExpr, f: DhallExpr) -> DhallExpr {
    DhallExpr::If(Box::new(cond), Box::new(t), Box::new(f))
}
/// Shorthand: let-in expression.
pub fn dhall_let(decls: Vec<DhallDecl>, body: DhallExpr) -> DhallExpr {
    DhallExpr::Let(decls, Box::new(body))
}
/// Shorthand: record literal.
pub fn dhall_record(fields: Vec<(&str, DhallExpr)>) -> DhallExpr {
    DhallExpr::RecordLit(Box::new(DhallRecord {
        fields: fields
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect(),
    }))
}
/// Shorthand: field selection.
pub fn dhall_select(record: DhallExpr, field: &str) -> DhallExpr {
    DhallExpr::Select(Box::new(record), field.to_string())
}
/// Shorthand: list literal.
pub fn dhall_list(items: Vec<DhallExpr>) -> DhallExpr {
    DhallExpr::ListLit(items)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub(super) fn test_emit_literals() {
        assert_eq!(dhall_bool(true).emit(0), "True");
        assert_eq!(dhall_bool(false).emit(0), "False");
        assert_eq!(dhall_nat(42).emit(0), "42");
        assert_eq!(dhall_int(5).emit(0), "+5");
        assert_eq!(dhall_int(-3).emit(0), "-3");
        assert_eq!(DhallExpr::DoubleLit(3.14).emit(0), "3.14");
        assert_eq!(dhall_text("hello").emit(0), "\"hello\"");
    }
    #[test]
    pub(super) fn test_emit_record() {
        let r = dhall_record(vec![("name", dhall_text("Alice")), ("age", dhall_nat(30))]);
        let s = r.emit(0);
        assert!(s.contains("name = \"Alice\""));
        assert!(s.contains("age = 30"));
        assert!(s.starts_with('{'));
        assert!(s.ends_with('}'));
    }
    #[test]
    pub(super) fn test_emit_lambda_and_apply() {
        let lam = dhall_lam("x", DhallType::Natural, dhall_var("x"));
        let s = lam.emit(0);
        assert_eq!(s, r"\(x : Natural) -> x");
        let app = dhall_app(lam, dhall_nat(5));
        let s2 = app.emit(0);
        assert!(s2.contains(r"\(x : Natural)"));
        assert!(s2.contains('5'));
    }
    #[test]
    pub(super) fn test_emit_if_then_else() {
        let expr = dhall_if(dhall_bool(true), dhall_nat(1), dhall_nat(0));
        let s = expr.emit(0);
        assert!(s.contains("if True"));
        assert!(s.contains("then"));
        assert!(s.contains('1'));
        assert!(s.contains("else"));
        assert!(s.contains('0'));
    }
    #[test]
    pub(super) fn test_emit_let_in() {
        let expr = dhall_let(
            vec![
                DhallDecl::typed("x", DhallType::Natural, dhall_nat(5)),
                DhallDecl::new("y", dhall_nat(10)),
            ],
            DhallExpr::NaturalOp(
                "+".into(),
                Box::new(dhall_var("x")),
                Box::new(dhall_var("y")),
            ),
        );
        let s = expr.emit(0);
        assert!(s.contains("let x : Natural = 5"));
        assert!(s.contains("let y = 10"));
        assert!(s.contains("in"));
        assert!(s.contains("(x + y)"));
    }
    #[test]
    pub(super) fn test_emit_union_type() {
        let t = DhallType::Union(vec![
            ("Red".into(), None),
            ("Green".into(), None),
            ("Blue".into(), None),
        ]);
        let s = t.to_string();
        assert!(s.contains("Red"));
        assert!(s.contains("Green"));
        assert!(s.contains("Blue"));
        assert!(s.contains('|'));
        assert!(s.starts_with('<'));
        assert!(s.ends_with('>'));
    }
    #[test]
    pub(super) fn test_emit_optional() {
        let some_e = DhallExpr::Some(Box::new(dhall_nat(42)));
        assert_eq!(some_e.emit(0), "Some 42");
        let none_e = DhallExpr::None(DhallType::Natural);
        assert_eq!(none_e.emit(0), "None Natural");
    }
    #[test]
    pub(super) fn test_emit_list() {
        let empty = DhallExpr::EmptyList(DhallType::Natural);
        assert_eq!(empty.emit(0), "[] : List Natural");
        let lst = dhall_list(vec![dhall_nat(1), dhall_nat(2), dhall_nat(3)]);
        let s = lst.emit(0);
        assert!(s.contains('1'));
        assert!(s.contains('2'));
        assert!(s.contains('3'));
        assert!(s.starts_with('['));
        assert!(s.ends_with(']'));
    }
    #[test]
    pub(super) fn test_emit_record_merge() {
        let r1 = dhall_record(vec![("a", dhall_nat(1))]);
        let r2 = dhall_record(vec![("b", dhall_nat(2))]);
        let merged = DhallExpr::RecordMerge(Box::new(r1), Box::new(r2));
        let s = merged.emit(0);
        assert!(s.contains("//"));
        assert!(s.contains("a = 1"));
        assert!(s.contains("b = 2"));
    }
    #[test]
    pub(super) fn test_emit_text_interpolation() {
        let expr = DhallExpr::TextInterp("Hello, ".into(), Box::new(dhall_var("name")), "!".into());
        let s = expr.emit(0);
        assert!(s.contains("Hello, "));
        assert!(s.contains("${name}"));
        assert!(s.contains('!'));
    }
    #[test]
    pub(super) fn test_dhall_file_emit() {
        let file = DhallFile::new(dhall_record(vec![
            ("host", dhall_text("localhost")),
            ("port", dhall_nat(8080)),
        ]))
        .import(
            "Prelude",
            DhallImport::Remote("https://prelude.dhall-lang.org/v21.1.0/package.dhall".into()),
        )
        .declare(DhallDecl::typed(
            "defaultPort",
            DhallType::Natural,
            dhall_nat(80),
        ));
        let s = file.emit();
        assert!(s.contains("Dhall configuration generated by OxiLean"));
        assert!(s.contains("let Prelude = https://prelude.dhall-lang.org"));
        assert!(s.contains("let defaultPort : Natural = 80"));
        assert!(s.contains("host = \"localhost\""));
        assert!(s.contains("port = 8080"));
    }
    #[test]
    pub(super) fn test_make_service_config() {
        let backend = DhallBackend::new();
        let cfg = backend.make_service_config(
            true,
            "my-service",
            9000,
            vec![("debug".into(), dhall_bool(false))],
        );
        let s = cfg.emit(0);
        assert!(s.contains("enable = True"));
        assert!(s.contains("name = \"my-service\""));
        assert!(s.contains("port = 9000"));
        assert!(s.contains("debug = False"));
    }
    #[test]
    pub(super) fn test_make_enum() {
        let backend = DhallBackend::new();
        let t = backend.make_enum(vec!["Success", "Failure", "Pending"]);
        let s = t.to_string();
        assert!(s.contains("Success"));
        assert!(s.contains("Failure"));
        assert!(s.contains("Pending"));
    }
    #[test]
    pub(super) fn test_dhall_type_display() {
        assert_eq!(DhallType::Bool.to_string(), "Bool");
        assert_eq!(DhallType::Natural.to_string(), "Natural");
        assert_eq!(DhallType::Type.to_string(), "Type");
        assert_eq!(DhallType::Kind.to_string(), "Kind");
        assert_eq!(
            DhallType::List(Box::new(DhallType::Text)).to_string(),
            "List Text"
        );
        assert_eq!(
            DhallType::Optional(Box::new(DhallType::Integer)).to_string(),
            "Optional Integer"
        );
        assert_eq!(
            DhallType::Function(Box::new(DhallType::Natural), Box::new(DhallType::Bool))
                .to_string(),
            "Natural -> Bool"
        );
    }
    #[test]
    pub(super) fn test_escape_string() {
        let s = dhall_text("say \"hi\" and ${var}").emit(0);
        assert_eq!(s, r#""say \"hi\" and \${var}""#);
    }
    #[test]
    pub(super) fn test_assert_equivalent() {
        let expr = DhallExpr::Assert(Box::new(dhall_nat(1)), Box::new(dhall_nat(1)));
        let s = expr.emit(0);
        assert_eq!(s, "assert : 1 === 1");
    }
    #[test]
    pub(super) fn test_merge_optional() {
        let backend = DhallBackend::new();
        let merged = backend.make_optional_merge(
            DhallExpr::Some(Box::new(dhall_nat(42))),
            dhall_lam("x", DhallType::Natural, dhall_var("x")),
            dhall_nat(0),
            DhallType::Natural,
        );
        let s = merged.emit(0);
        assert!(s.contains("merge"));
        assert!(s.contains("Some"));
        assert!(s.contains("None"));
        assert!(s.contains(": Natural"));
    }
}
#[cfg(test)]
mod Dhall_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = DhallPassConfig::new("test_pass", DhallPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = DhallPassStats::new();
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
        let mut reg = DhallPassRegistry::new();
        reg.register(DhallPassConfig::new("pass_a", DhallPassPhase::Analysis));
        reg.register(DhallPassConfig::new("pass_b", DhallPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = DhallAnalysisCache::new(10);
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
        let mut wl = DhallWorklist::new();
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
        let mut dt = DhallDominatorTree::new(5);
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
        let mut liveness = DhallLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(DhallConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(DhallConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(DhallConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            DhallConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(DhallConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = DhallDepGraph::new();
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
mod dhallext_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_dhallext_phase_order() {
        assert_eq!(DhallExtPassPhase::Early.order(), 0);
        assert_eq!(DhallExtPassPhase::Middle.order(), 1);
        assert_eq!(DhallExtPassPhase::Late.order(), 2);
        assert_eq!(DhallExtPassPhase::Finalize.order(), 3);
        assert!(DhallExtPassPhase::Early.is_early());
        assert!(!DhallExtPassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_dhallext_config_builder() {
        let c = DhallExtPassConfig::new("p")
            .with_phase(DhallExtPassPhase::Late)
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
    pub(super) fn test_dhallext_stats() {
        let mut s = DhallExtPassStats::new();
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
    pub(super) fn test_dhallext_registry() {
        let mut r = DhallExtPassRegistry::new();
        r.register(DhallExtPassConfig::new("a").with_phase(DhallExtPassPhase::Early));
        r.register(DhallExtPassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&DhallExtPassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_dhallext_cache() {
        let mut c = DhallExtCache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_dhallext_worklist() {
        let mut w = DhallExtWorklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_dhallext_dom_tree() {
        let mut dt = DhallExtDomTree::new(5);
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
    pub(super) fn test_dhallext_liveness() {
        let mut lv = DhallExtLiveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_dhallext_const_folder() {
        let mut cf = DhallExtConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_dhallext_dep_graph() {
        let mut g = DhallExtDepGraph::new(4);
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
mod dhallx2_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_dhallx2_phase_order() {
        assert_eq!(DhallX2PassPhase::Early.order(), 0);
        assert_eq!(DhallX2PassPhase::Middle.order(), 1);
        assert_eq!(DhallX2PassPhase::Late.order(), 2);
        assert_eq!(DhallX2PassPhase::Finalize.order(), 3);
        assert!(DhallX2PassPhase::Early.is_early());
        assert!(!DhallX2PassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_dhallx2_config_builder() {
        let c = DhallX2PassConfig::new("p")
            .with_phase(DhallX2PassPhase::Late)
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
    pub(super) fn test_dhallx2_stats() {
        let mut s = DhallX2PassStats::new();
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
    pub(super) fn test_dhallx2_registry() {
        let mut r = DhallX2PassRegistry::new();
        r.register(DhallX2PassConfig::new("a").with_phase(DhallX2PassPhase::Early));
        r.register(DhallX2PassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&DhallX2PassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_dhallx2_cache() {
        let mut c = DhallX2Cache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_dhallx2_worklist() {
        let mut w = DhallX2Worklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_dhallx2_dom_tree() {
        let mut dt = DhallX2DomTree::new(5);
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
    pub(super) fn test_dhallx2_liveness() {
        let mut lv = DhallX2Liveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_dhallx2_const_folder() {
        let mut cf = DhallX2ConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_dhallx2_dep_graph() {
        let mut g = DhallX2DepGraph::new(4);
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
