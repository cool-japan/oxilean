//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;

use super::types::{
    RArg, RBackend, RDataObject, RExpr, RFile, RFormal, RFunction, RLangAnalysisCache,
    RLangConstantFoldingHelper, RLangDepGraph, RLangDominatorTree, RLangExtCache,
    RLangExtConstFolder, RLangExtDepGraph, RLangExtDomTree, RLangExtLiveness, RLangExtPassConfig,
    RLangExtPassPhase, RLangExtPassRegistry, RLangExtPassStats, RLangExtWorklist,
    RLangLivenessInfo, RLangPassConfig, RLangPassPhase, RLangPassRegistry, RLangPassStats,
    RLangWorklist, RLiteral, RStmt, VectorizedOp,
};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub(super) fn test_r_literal_emission() {
        let backend = RBackend::new();
        assert_eq!(backend.emit_literal(&RLiteral::Integer(42)), "42L");
        assert_eq!(backend.emit_literal(&RLiteral::Numeric(3.14)), "3.14");
        assert_eq!(backend.emit_literal(&RLiteral::Logical(true)), "TRUE");
        assert_eq!(backend.emit_literal(&RLiteral::Logical(false)), "FALSE");
        assert_eq!(
            backend.emit_literal(&RLiteral::Character("hello".to_string())),
            "\"hello\""
        );
        assert_eq!(backend.emit_literal(&RLiteral::Null), "NULL");
        assert_eq!(backend.emit_literal(&RLiteral::Na), "NA");
        assert_eq!(backend.emit_literal(&RLiteral::Inf), "Inf");
        assert_eq!(backend.emit_literal(&RLiteral::NaN), "NaN");
        assert_eq!(backend.emit_literal(&RLiteral::Complex(1.0, 2.0)), "1+2i");
        assert_eq!(backend.emit_literal(&RLiteral::Complex(1.0, -2.0)), "1-2i");
    }
    #[test]
    pub(super) fn test_r_infix_and_unary() {
        let backend = RBackend::new();
        let add = RExpr::InfixOp(
            "+".to_string(),
            Box::new(RExpr::Lit(RLiteral::Integer(3))),
            Box::new(RExpr::Lit(RLiteral::Integer(4))),
        );
        assert_eq!(backend.emit_expr_pure(&add), "3L + 4L");
        let not_expr = RExpr::UnaryOp("!".to_string(), Box::new(RExpr::Var("x".to_string())));
        assert_eq!(backend.emit_expr_pure(&not_expr), "!x");
    }
    #[test]
    pub(super) fn test_r_index_and_access() {
        let backend = RBackend::new();
        let single = RExpr::IndexSingle(
            Box::new(RExpr::Var("v".to_string())),
            vec![RExpr::Lit(RLiteral::Integer(1))],
        );
        assert_eq!(backend.emit_expr_pure(&single), "v[1L]");
        let double = RExpr::IndexDouble(
            Box::new(RExpr::Var("lst".to_string())),
            Box::new(RExpr::Lit(RLiteral::Character("key".to_string()))),
        );
        assert_eq!(backend.emit_expr_pure(&double), "lst[[\"key\"]]");
        let dollar = RExpr::DollarAccess(Box::new(RExpr::Var("df".to_string())), "col".to_string());
        assert_eq!(backend.emit_expr_pure(&dollar), "df$col");
        let at = RExpr::AtAccess(Box::new(RExpr::Var("obj".to_string())), "slot1".to_string());
        assert_eq!(backend.emit_expr_pure(&at), "obj@slot1");
    }
    #[test]
    pub(super) fn test_r_formula_and_pipe() {
        let backend = RBackend::new();
        let formula = RExpr::Formula(
            Some(Box::new(RExpr::Var("y".to_string()))),
            Box::new(RExpr::InfixOp(
                "+".to_string(),
                Box::new(RExpr::Var("x1".to_string())),
                Box::new(RExpr::Var("x2".to_string())),
            )),
        );
        assert_eq!(backend.emit_expr_pure(&formula), "y ~ x1 + x2");
        let one_sided = RExpr::Formula(None, Box::new(RExpr::Var("x".to_string())));
        assert_eq!(backend.emit_expr_pure(&one_sided), "~ x");
        let pipe = RExpr::Pipe(
            Box::new(RExpr::Var("data".to_string())),
            Box::new(RExpr::Call(
                Box::new(RExpr::Var("mean".to_string())),
                vec![],
            )),
        );
        assert_eq!(backend.emit_expr_pure(&pipe), "data |> mean()");
    }
    #[test]
    pub(super) fn test_r_function_def_emit() {
        let mut backend = RBackend::new();
        let fun = RFunction::new(
            "add",
            vec![
                RFormal::required("x"),
                RFormal::with_default("y", RExpr::Lit(RLiteral::Integer(0))),
            ],
            vec![RStmt::Return(Some(RExpr::InfixOp(
                "+".to_string(),
                Box::new(RExpr::Var("x".to_string())),
                Box::new(RExpr::Var("y".to_string())),
            )))],
        );
        backend.emit_function(&fun);
        let out = backend.take_output();
        assert!(
            out.contains("add <- function(x, y = 0L)"),
            "missing header: {}",
            out
        );
        assert!(out.contains("return(x + y)"), "missing return: {}", out);
        assert!(out.contains('}'), "missing closing brace: {}", out);
    }
    #[test]
    pub(super) fn test_r_for_while_loops() {
        let mut backend = RBackend::new();
        let for_loop = RStmt::ForLoop {
            var: "i".to_string(),
            seq: RExpr::Seq(
                Box::new(RExpr::Lit(RLiteral::Integer(1))),
                Box::new(RExpr::Lit(RLiteral::Integer(10))),
            ),
            body: vec![RStmt::Expr(RExpr::Var("i".to_string()))],
        };
        backend.emit_stmt(&for_loop);
        let out = backend.take_output();
        assert!(out.contains("for (i in 1L:10L)"), "missing for: {}", out);
        let while_loop = RStmt::WhileLoop {
            cond: RExpr::Var("running".to_string()),
            body: vec![RStmt::Break],
        };
        backend.emit_stmt(&while_loop);
        let out2 = backend.take_output();
        assert!(out2.contains("while (running)"), "missing while: {}", out2);
        assert!(out2.contains("break"), "missing break: {}", out2);
    }
    #[test]
    pub(super) fn test_r_if_else_stmt() {
        let mut backend = RBackend::new();
        let stmt = RStmt::IfElse {
            cond: RExpr::InfixOp(
                ">".to_string(),
                Box::new(RExpr::Var("x".to_string())),
                Box::new(RExpr::Lit(RLiteral::Integer(0))),
            ),
            then_body: vec![RStmt::Return(Some(RExpr::Lit(RLiteral::Logical(true))))],
            else_if_branches: vec![],
            else_body: Some(vec![RStmt::Return(Some(RExpr::Lit(RLiteral::Logical(
                false,
            ))))]),
        };
        backend.emit_stmt(&stmt);
        let out = backend.take_output();
        assert!(out.contains("if (x > 0L)"), "missing if: {}", out);
        assert!(out.contains("return(TRUE)"), "missing then: {}", out);
        assert!(out.contains("} else {"), "missing else: {}", out);
        assert!(out.contains("return(FALSE)"), "missing else body: {}", out);
    }
    #[test]
    pub(super) fn test_r_vectorized_ops() {
        let backend = RBackend::new();
        let sapply_op = VectorizedOp::with_sapply("sqrt");
        let vec_expr = RExpr::Var("nums".to_string());
        let result = backend.emit_vectorized(&sapply_op, &vec_expr, &[]);
        assert_eq!(result, "sapply(nums, sqrt)");
        let vec_op = VectorizedOp::with_vectorize("my_func");
        let result2 = backend.emit_vectorized(&vec_op, &vec_expr, &[]);
        assert_eq!(result2, "Vectorize(my_func)(nums)");
        let elem_op = VectorizedOp::element_wise("sum");
        let result3 = backend.emit_vectorized(&elem_op, &vec_expr, &[]);
        assert_eq!(result3, "sum(nums)");
    }
    #[test]
    pub(super) fn test_r_s3_generic_with_methods() {
        let mut backend = RBackend::new();
        let mut generic =
            RFunction::new("describe", vec![RFormal::required("x")], vec![]).generic();
        let numeric_method = RFunction::new(
            "describe.numeric",
            vec![RFormal::required("x")],
            vec![RStmt::Expr(RExpr::Call(
                Box::new(RExpr::Var("cat".to_string())),
                vec![RArg::positional(RExpr::Lit(RLiteral::Character(
                    "numeric".to_string(),
                )))],
            ))],
        );
        generic.add_s3_method("numeric", numeric_method);
        backend.emit_function(&generic);
        let out = backend.take_output();
        assert!(
            out.contains("UseMethod(\"describe\")"),
            "missing UseMethod: {}",
            out
        );
        assert!(
            out.contains("describe.numeric <- function(x)"),
            "missing S3 method: {}",
            out
        );
        assert!(
            out.contains("cat(\"numeric\")"),
            "missing method body: {}",
            out
        );
    }
    #[test]
    pub(super) fn test_r_file_emit() {
        let mut backend = RBackend::new();
        let mut file = RFile::new().with_header("Generated by OxiLean");
        file.add_import("dplyr");
        file.add_import("ggplot2");
        file.add_function(RFunction::new(
            "compute",
            vec![RFormal::required("data")],
            vec![RStmt::Return(Some(RExpr::Var("data".to_string())))],
        ));
        file.add_data(RDataObject {
            name: "PI".to_string(),
            value: RExpr::Lit(RLiteral::Numeric(std::f64::consts::PI)),
            comment: Some("Pi constant".to_string()),
        });
        backend.emit_file(&file);
        let out = backend.take_output();
        assert!(out.contains("library(dplyr)"), "missing dplyr: {}", out);
        assert!(out.contains("library(ggplot2)"), "missing ggplot2: {}", out);
        assert!(
            out.contains("compute <- function(data)"),
            "missing function: {}",
            out
        );
        assert!(out.contains("PI <-"), "missing data: {}", out);
        assert!(
            out.contains("# Generated by OxiLean"),
            "missing header: {}",
            out
        );
    }
}
#[cfg(test)]
mod RLang_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = RLangPassConfig::new("test_pass", RLangPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = RLangPassStats::new();
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
        let mut reg = RLangPassRegistry::new();
        reg.register(RLangPassConfig::new("pass_a", RLangPassPhase::Analysis));
        reg.register(RLangPassConfig::new("pass_b", RLangPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = RLangAnalysisCache::new(10);
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
        let mut wl = RLangWorklist::new();
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
        let mut dt = RLangDominatorTree::new(5);
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
        let mut liveness = RLangLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(RLangConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(RLangConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(RLangConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            RLangConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(RLangConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = RLangDepGraph::new();
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
mod rlangext_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_rlangext_phase_order() {
        assert_eq!(RLangExtPassPhase::Early.order(), 0);
        assert_eq!(RLangExtPassPhase::Middle.order(), 1);
        assert_eq!(RLangExtPassPhase::Late.order(), 2);
        assert_eq!(RLangExtPassPhase::Finalize.order(), 3);
        assert!(RLangExtPassPhase::Early.is_early());
        assert!(!RLangExtPassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_rlangext_config_builder() {
        let c = RLangExtPassConfig::new("p")
            .with_phase(RLangExtPassPhase::Late)
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
    pub(super) fn test_rlangext_stats() {
        let mut s = RLangExtPassStats::new();
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
    pub(super) fn test_rlangext_registry() {
        let mut r = RLangExtPassRegistry::new();
        r.register(RLangExtPassConfig::new("a").with_phase(RLangExtPassPhase::Early));
        r.register(RLangExtPassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&RLangExtPassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_rlangext_cache() {
        let mut c = RLangExtCache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_rlangext_worklist() {
        let mut w = RLangExtWorklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_rlangext_dom_tree() {
        let mut dt = RLangExtDomTree::new(5);
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
    pub(super) fn test_rlangext_liveness() {
        let mut lv = RLangExtLiveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_rlangext_const_folder() {
        let mut cf = RLangExtConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_rlangext_dep_graph() {
        let mut g = RLangExtDepGraph::new(4);
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
