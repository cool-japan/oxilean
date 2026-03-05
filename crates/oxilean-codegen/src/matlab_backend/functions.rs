//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;

use super::types::{
    MatlabAnalysisCache, MatlabBackend, MatlabClassdef, MatlabConstantFoldingHelper,
    MatlabDepGraph, MatlabDominatorTree, MatlabExpr, MatlabFunction, MatlabLiteral,
    MatlabLivenessInfo, MatlabParam, MatlabPassConfig, MatlabPassPhase, MatlabPassRegistry,
    MatlabPassStats, MatlabProperty, MatlabStmt, MatlabType, MatlabWorklist, PropAccess,
};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub(super) fn test_matlab_literal_emission() {
        let backend = MatlabBackend::new();
        assert_eq!(backend.emit_literal(&MatlabLiteral::Double(3.0)), "3");
        assert_eq!(backend.emit_literal(&MatlabLiteral::Double(3.14)), "3.14");
        assert_eq!(backend.emit_literal(&MatlabLiteral::Integer(42)), "42");
        assert_eq!(backend.emit_literal(&MatlabLiteral::Logical(true)), "true");
        assert_eq!(
            backend.emit_literal(&MatlabLiteral::Logical(false)),
            "false"
        );
        assert_eq!(
            backend.emit_literal(&MatlabLiteral::Char("hello".to_string())),
            "'hello'"
        );
        assert_eq!(
            backend.emit_literal(&MatlabLiteral::Str("world".to_string())),
            "\"world\""
        );
        assert_eq!(backend.emit_literal(&MatlabLiteral::Empty), "[]");
        assert_eq!(backend.emit_literal(&MatlabLiteral::NaN), "NaN");
        assert_eq!(backend.emit_literal(&MatlabLiteral::Inf(false)), "Inf");
        assert_eq!(backend.emit_literal(&MatlabLiteral::Inf(true)), "-Inf");
        assert_eq!(backend.emit_literal(&MatlabLiteral::Pi), "pi");
    }
    #[test]
    pub(super) fn test_matlab_matrix_literal() {
        let backend = MatlabBackend::new();
        let mat = MatlabExpr::MatrixLit(vec![
            vec![
                MatlabExpr::Lit(MatlabLiteral::Integer(1)),
                MatlabExpr::Lit(MatlabLiteral::Integer(2)),
            ],
            vec![
                MatlabExpr::Lit(MatlabLiteral::Integer(3)),
                MatlabExpr::Lit(MatlabLiteral::Integer(4)),
            ],
        ]);
        assert_eq!(backend.emit_expr_pure(&mat), "[1, 2; 3, 4]");
        let vec_row = MatlabExpr::MatrixLit(vec![vec![
            MatlabExpr::Lit(MatlabLiteral::Integer(1)),
            MatlabExpr::Lit(MatlabLiteral::Integer(2)),
            MatlabExpr::Lit(MatlabLiteral::Integer(3)),
        ]]);
        assert_eq!(backend.emit_expr_pure(&vec_row), "[1, 2, 3]");
    }
    #[test]
    pub(super) fn test_matlab_colon_range() {
        let backend = MatlabBackend::new();
        let simple = MatlabExpr::ColonRange {
            start: Box::new(MatlabExpr::Lit(MatlabLiteral::Integer(1))),
            step: None,
            end: Box::new(MatlabExpr::Lit(MatlabLiteral::Integer(10))),
        };
        assert_eq!(backend.emit_expr_pure(&simple), "1:10");
        let stepped = MatlabExpr::ColonRange {
            start: Box::new(MatlabExpr::Lit(MatlabLiteral::Integer(0))),
            step: Some(Box::new(MatlabExpr::Lit(MatlabLiteral::Double(0.1)))),
            end: Box::new(MatlabExpr::Lit(MatlabLiteral::Integer(1))),
        };
        assert_eq!(backend.emit_expr_pure(&stepped), "0:0.1:1");
    }
    #[test]
    pub(super) fn test_matlab_binary_unary_ops() {
        let backend = MatlabBackend::new();
        let add = MatlabExpr::BinaryOp(
            "+".to_string(),
            Box::new(MatlabExpr::Var("a".to_string())),
            Box::new(MatlabExpr::Var("b".to_string())),
        );
        assert_eq!(backend.emit_expr_pure(&add), "a + b");
        let elem_mul = MatlabExpr::BinaryOp(
            ".*".to_string(),
            Box::new(MatlabExpr::Var("A".to_string())),
            Box::new(MatlabExpr::Var("B".to_string())),
        );
        assert_eq!(backend.emit_expr_pure(&elem_mul), "A .* B");
        let transpose = MatlabExpr::UnaryOp(
            "'".to_string(),
            Box::new(MatlabExpr::Var("M".to_string())),
            true,
        );
        assert_eq!(backend.emit_expr_pure(&transpose), "M'");
        let neg = MatlabExpr::UnaryOp(
            "-".to_string(),
            Box::new(MatlabExpr::Var("x".to_string())),
            false,
        );
        assert_eq!(backend.emit_expr_pure(&neg), "-x");
    }
    #[test]
    pub(super) fn test_matlab_anon_func_and_index() {
        let backend = MatlabBackend::new();
        let anon = MatlabExpr::AnonFunc(
            vec!["x".to_string(), "y".to_string()],
            Box::new(MatlabExpr::BinaryOp(
                "+".to_string(),
                Box::new(MatlabExpr::Var("x".to_string())),
                Box::new(MatlabExpr::Var("y".to_string())),
            )),
        );
        assert_eq!(backend.emit_expr_pure(&anon), "@(x, y) x + y");
        let index = MatlabExpr::Index {
            obj: Box::new(MatlabExpr::Var("A".to_string())),
            indices: vec![
                MatlabExpr::Lit(MatlabLiteral::Integer(2)),
                MatlabExpr::Lit(MatlabLiteral::Integer(3)),
            ],
            cell_index: false,
        };
        assert_eq!(backend.emit_expr_pure(&index), "A(2, 3)");
        let cell_index = MatlabExpr::Index {
            obj: Box::new(MatlabExpr::Var("C".to_string())),
            indices: vec![MatlabExpr::Lit(MatlabLiteral::Integer(1))],
            cell_index: true,
        };
        assert_eq!(backend.emit_expr_pure(&cell_index), "C{1}");
    }
    #[test]
    pub(super) fn test_matlab_for_while_loops() {
        let mut backend = MatlabBackend::new();
        let for_loop = MatlabStmt::ForLoop {
            var: "i".to_string(),
            range: MatlabExpr::ColonRange {
                start: Box::new(MatlabExpr::Lit(MatlabLiteral::Integer(1))),
                step: None,
                end: Box::new(MatlabExpr::Lit(MatlabLiteral::Integer(10))),
            },
            body: vec![MatlabStmt::Comment("loop body".to_string())],
        };
        backend.emit_stmt(&for_loop);
        let out = backend.take_output();
        assert!(out.contains("for i = 1:10"), "missing for: {}", out);
        assert!(out.contains("end"), "missing end: {}", out);
        let while_loop = MatlabStmt::WhileLoop {
            cond: MatlabExpr::Var("running".to_string()),
            body: vec![MatlabStmt::Break],
        };
        backend.emit_stmt(&while_loop);
        let out2 = backend.take_output();
        assert!(out2.contains("while running"), "missing while: {}", out2);
        assert!(out2.contains("break"), "missing break: {}", out2);
    }
    #[test]
    pub(super) fn test_matlab_function_emit() {
        let mut backend = MatlabBackend::new();
        let fun = MatlabFunction::new(
            "add_vectors",
            vec![MatlabParam::required("a"), MatlabParam::required("b")],
            vec!["result".to_string()],
            vec![MatlabStmt::Assign {
                lhs: vec!["result".to_string()],
                rhs: MatlabExpr::BinaryOp(
                    "+".to_string(),
                    Box::new(MatlabExpr::Var("a".to_string())),
                    Box::new(MatlabExpr::Var("b".to_string())),
                ),
                suppress: true,
            }],
        );
        backend.emit_function(&fun);
        let out = backend.take_output();
        assert!(
            out.contains("function result = add_vectors(a, b)"),
            "missing header: {}",
            out
        );
        assert!(out.contains("result = a + b;"), "missing body: {}", out);
        assert!(out.contains("end"), "missing end: {}", out);
    }
    #[test]
    pub(super) fn test_matlab_if_switch() {
        let mut backend = MatlabBackend::new();
        let if_stmt = MatlabStmt::IfElseIf {
            cond: MatlabExpr::BinaryOp(
                ">".to_string(),
                Box::new(MatlabExpr::Var("x".to_string())),
                Box::new(MatlabExpr::Lit(MatlabLiteral::Integer(0))),
            ),
            then_body: vec![MatlabStmt::Return],
            elseif_branches: vec![],
            else_body: Some(vec![MatlabStmt::Break]),
        };
        backend.emit_stmt(&if_stmt);
        let out = backend.take_output();
        assert!(out.contains("if x > 0"), "missing if: {}", out);
        assert!(out.contains("return;"), "missing return: {}", out);
        assert!(out.contains("else"), "missing else: {}", out);
        assert!(out.contains("break;"), "missing break: {}", out);
        let switch_stmt = MatlabStmt::SwitchCase {
            expr: MatlabExpr::Var("mode".to_string()),
            cases: vec![
                (
                    MatlabExpr::Lit(MatlabLiteral::Char("fast".to_string())),
                    vec![MatlabStmt::Comment("fast path".to_string())],
                ),
                (
                    MatlabExpr::Lit(MatlabLiteral::Char("slow".to_string())),
                    vec![MatlabStmt::Comment("slow path".to_string())],
                ),
            ],
            otherwise: Some(vec![MatlabStmt::Error(
                MatlabExpr::Lit(MatlabLiteral::Char("Unknown mode".to_string())),
                vec![],
            )]),
        };
        backend.emit_stmt(&switch_stmt);
        let out2 = backend.take_output();
        assert!(out2.contains("switch mode"), "missing switch: {}", out2);
        assert!(out2.contains("case 'fast'"), "missing case: {}", out2);
        assert!(out2.contains("otherwise"), "missing otherwise: {}", out2);
    }
    #[test]
    pub(super) fn test_matlab_classdef_emit() {
        let mut backend = MatlabBackend::new();
        let mut cls = MatlabClassdef::new("Vehicle").inherits("handle");
        cls.properties.push(MatlabProperty {
            name: "speed".to_string(),
            ty: Some(MatlabType::Double),
            default: Some(MatlabExpr::Lit(MatlabLiteral::Double(0.0))),
            access: PropAccess::Public,
            is_constant: false,
            is_dependent: false,
        });
        cls.methods.push(MatlabFunction::new(
            "accelerate",
            vec![MatlabParam::required("obj"), MatlabParam::required("delta")],
            vec![],
            vec![MatlabStmt::AssignField {
                obj: "obj".to_string(),
                field: "speed".to_string(),
                rhs: MatlabExpr::BinaryOp(
                    "+".to_string(),
                    Box::new(MatlabExpr::FieldAccess(
                        Box::new(MatlabExpr::Var("obj".to_string())),
                        "speed".to_string(),
                    )),
                    Box::new(MatlabExpr::Var("delta".to_string())),
                ),
                suppress: true,
            }],
        ));
        backend.emit_classdef(&cls);
        let out = backend.take_output();
        assert!(
            out.contains("classdef Vehicle < handle"),
            "missing classdef: {}",
            out
        );
        assert!(out.contains("properties"), "missing properties: {}", out);
        assert!(out.contains("methods"), "missing methods: {}", out);
        assert!(
            out.contains("function accelerate(obj, delta)"),
            "missing method: {}",
            out
        );
    }
}
#[cfg(test)]
mod Matlab_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = MatlabPassConfig::new("test_pass", MatlabPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = MatlabPassStats::new();
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
        let mut reg = MatlabPassRegistry::new();
        reg.register(MatlabPassConfig::new("pass_a", MatlabPassPhase::Analysis));
        reg.register(MatlabPassConfig::new("pass_b", MatlabPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = MatlabAnalysisCache::new(10);
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
        let mut wl = MatlabWorklist::new();
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
        let mut dt = MatlabDominatorTree::new(5);
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
        let mut liveness = MatlabLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(MatlabConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(MatlabConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(MatlabConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            MatlabConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(MatlabConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = MatlabDepGraph::new();
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
