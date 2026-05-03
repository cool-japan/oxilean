//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::expr::{Expr, Literal};
use std::collections::{HashMap, HashSet, VecDeque};

use super::types::{
    CodegenConfig, CodegenError, CodegenPipeline, CodegenTarget, ExprToIr, IrExpr, IrLit,
    IrMatchArm, IrPattern, IrToC, IrToRust, IrType, LibAnalysisCache, LibConstantFoldingHelper,
    LibDepGraph, LibDominatorTree, LibLivenessInfo, LibPassConfig, LibPassPhase, LibPassRegistry,
    LibPassStats, LibWorklist, Optimizer, RustEmitter, SymbolManager,
};

pub type CodegenResult<T> = Result<T, CodegenError>;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ir_type_display() {
        assert_eq!(IrType::Unit.to_string(), "()");
        assert_eq!(IrType::Bool.to_string(), "bool");
        assert_eq!(IrType::Int.to_string(), "i64");
        assert_eq!(IrType::Nat.to_string(), "nat");
    }
    #[test]
    fn test_ir_literal_display() {
        assert_eq!(IrLit::Unit.to_string(), "()");
        assert_eq!(IrLit::Bool(true).to_string(), "true");
        assert_eq!(IrLit::Bool(false).to_string(), "false");
        assert_eq!(IrLit::Nat(42).to_string(), "42");
        assert_eq!(IrLit::Int(-42).to_string(), "-42");
    }
    #[test]
    fn test_ir_var_expression() {
        let expr = IrExpr::Var("x".to_string());
        match expr {
            IrExpr::Var(name) => assert_eq!(name, "x"),
            _ => panic!("Expected Var"),
        }
    }
    #[test]
    fn test_ir_lit_expression() {
        let expr = IrExpr::Lit(IrLit::Int(42));
        match expr {
            IrExpr::Lit(IrLit::Int(n)) => assert_eq!(n, 42),
            _ => panic!("Expected Lit"),
        }
    }
    #[test]
    fn test_ir_app_expression() {
        let func = Box::new(IrExpr::Var("f".to_string()));
        let arg = IrExpr::Lit(IrLit::Int(1));
        let expr = IrExpr::App {
            func,
            args: vec![arg],
        };
        match expr {
            IrExpr::App { args, .. } => assert_eq!(args.len(), 1),
            _ => panic!("Expected App"),
        }
    }
    #[test]
    fn test_ir_let_expression() {
        let expr = IrExpr::Let {
            name: "x".to_string(),
            ty: IrType::Int,
            value: Box::new(IrExpr::Lit(IrLit::Int(5))),
            body: Box::new(IrExpr::Var("x".to_string())),
        };
        match expr {
            IrExpr::Let { name, .. } => assert_eq!(name, "x"),
            _ => panic!("Expected Let"),
        }
    }
    #[test]
    fn test_ir_lambda_expression() {
        let expr = IrExpr::Lambda {
            params: vec![("x".to_string(), IrType::Int)],
            body: Box::new(IrExpr::Var("x".to_string())),
            captured: vec![],
        };
        match expr {
            IrExpr::Lambda { params, .. } => assert_eq!(params.len(), 1),
            _ => panic!("Expected Lambda"),
        }
    }
    #[test]
    fn test_ir_if_expression() {
        let expr = IrExpr::If {
            cond: Box::new(IrExpr::Lit(IrLit::Bool(true))),
            then_branch: Box::new(IrExpr::Lit(IrLit::Int(1))),
            else_branch: Box::new(IrExpr::Lit(IrLit::Int(0))),
        };
        match expr {
            IrExpr::If { .. } => {}
            _ => panic!("Expected If"),
        }
    }
    #[test]
    fn test_ir_match_expression() {
        let arms = vec![IrMatchArm {
            pattern: IrPattern::Literal("true".to_string()),
            body: Box::new(IrExpr::Lit(IrLit::Int(1))),
        }];
        let expr = IrExpr::Match {
            scrutinee: Box::new(IrExpr::Lit(IrLit::Bool(true))),
            arms,
        };
        match expr {
            IrExpr::Match { arms, .. } => assert_eq!(arms.len(), 1),
            _ => panic!("Expected Match"),
        }
    }
    #[test]
    fn test_ir_struct_expression() {
        let expr = IrExpr::Struct {
            name: "Point".to_string(),
            fields: vec![("x".to_string(), IrExpr::Lit(IrLit::Int(1)))],
        };
        match expr {
            IrExpr::Struct { name, .. } => assert_eq!(name, "Point"),
            _ => panic!("Expected Struct"),
        }
    }
    #[test]
    fn test_ir_field_expression() {
        let expr = IrExpr::Field {
            object: Box::new(IrExpr::Var("point".to_string())),
            field: "x".to_string(),
        };
        match expr {
            IrExpr::Field { field, .. } => assert_eq!(field, "x"),
            _ => panic!("Expected Field"),
        }
    }
    #[test]
    fn test_ir_alloc_expression() {
        let expr = IrExpr::Alloc(Box::new(IrExpr::Lit(IrLit::Int(42))));
        match expr {
            IrExpr::Alloc(_) => {}
            _ => panic!("Expected Alloc"),
        }
    }
    #[test]
    fn test_ir_deref_expression() {
        let expr = IrExpr::Deref(Box::new(IrExpr::Var("ptr".to_string())));
        match expr {
            IrExpr::Deref(_) => {}
            _ => panic!("Expected Deref"),
        }
    }
    #[test]
    fn test_ir_seq_expression() {
        let expr = IrExpr::Seq(vec![IrExpr::Lit(IrLit::Int(1)), IrExpr::Lit(IrLit::Int(2))]);
        match expr {
            IrExpr::Seq(exprs) => assert_eq!(exprs.len(), 2),
            _ => panic!("Expected Seq"),
        }
    }
    #[test]
    fn test_codegen_config_default() {
        let config = CodegenConfig::default();
        assert_eq!(config.target, CodegenTarget::Rust);
        assert!(config.optimize);
        assert!(!config.debug_info);
        assert!(config.emit_comments);
        assert_eq!(config.inline_threshold, 50);
    }
    #[test]
    fn test_codegen_target_display() {
        assert_eq!(CodegenTarget::Rust.to_string(), "Rust");
        assert_eq!(CodegenTarget::C.to_string(), "C");
        assert_eq!(CodegenTarget::LlvmIr.to_string(), "LLVM IR");
        assert_eq!(CodegenTarget::Interpreter.to_string(), "Interpreter");
    }
    #[test]
    fn test_codegen_error_display() {
        let err = CodegenError::UnboundVariable("x".to_string());
        assert_eq!(err.to_string(), "Unbound variable: x");
        let err = CodegenError::UnsupportedExpression("foo".to_string());
        assert_eq!(err.to_string(), "Unsupported expression: foo");
        let err = CodegenError::TypeMismatch {
            expected: "Int".to_string(),
            found: "Bool".to_string(),
        };
        assert_eq!(err.to_string(), "Type mismatch: expected Int, found Bool");
    }
    #[test]
    fn test_symbol_manager_fresh_names() {
        let mut mgr = SymbolManager::new();
        let name1 = mgr.fresh_name("var");
        let name2 = mgr.fresh_name("var");
        assert_ne!(name1, name2);
        assert!(name1.starts_with("var_"));
        assert!(name2.starts_with("var_"));
    }
    #[test]
    fn test_symbol_manager_scopes() {
        let mut mgr = SymbolManager::new();
        mgr.push_scope();
        mgr.bind("x".to_string());
        assert!(mgr.is_bound("x"));
        mgr.pop_scope();
        assert!(!mgr.is_bound("x"));
    }
    #[test]
    fn test_expr_to_ir_new() {
        let compiler = ExprToIr::new();
        assert_eq!(compiler.closure_vars.len(), 0);
    }
    #[test]
    fn test_rust_emitter_indent() {
        let mut emitter = RustEmitter::new();
        assert_eq!(emitter.indent_level, 0);
        emitter.indent();
        assert_eq!(emitter.indent_level, 1);
        emitter.dedent();
        assert_eq!(emitter.indent_level, 0);
    }
    #[test]
    fn test_rust_emitter_emit_simple() {
        let mut emitter = RustEmitter::new();
        emitter.emit("let x = 5;");
        let output = emitter.result();
        assert!(output.contains("let x = 5;"));
    }
    #[test]
    fn test_ir_to_rust_emit_var() -> CodegenResult<()> {
        let config = CodegenConfig::default();
        let gen = IrToRust::new(config);
        let expr = IrExpr::Var("x".to_string());
        let output = gen.emit(&expr)?;
        assert!(output.contains("x"));
        Ok(())
    }
    #[test]
    fn test_ir_to_rust_emit_literal() -> CodegenResult<()> {
        let config = CodegenConfig::default();
        let gen = IrToRust::new(config);
        let expr = IrExpr::Lit(IrLit::Int(42));
        let output = gen.emit(&expr)?;
        assert!(output.contains("42"));
        Ok(())
    }
    #[test]
    fn test_ir_to_rust_emit_if() -> CodegenResult<()> {
        let config = CodegenConfig::default();
        let gen = IrToRust::new(config);
        let expr = IrExpr::If {
            cond: Box::new(IrExpr::Lit(IrLit::Bool(true))),
            then_branch: Box::new(IrExpr::Lit(IrLit::Int(1))),
            else_branch: Box::new(IrExpr::Lit(IrLit::Int(0))),
        };
        let output = gen.emit(&expr)?;
        assert!(output.contains("if"));
        assert!(output.contains("else"));
        Ok(())
    }
    #[test]
    fn test_ir_to_rust_emit_struct() -> CodegenResult<()> {
        let config = CodegenConfig::default();
        let gen = IrToRust::new(config);
        let fields = vec![("x".to_string(), IrType::Int)];
        let output = gen.emit_struct("Point", &fields)?;
        assert!(output.contains("struct Point"));
        assert!(output.contains("x: i64"));
        Ok(())
    }
    #[test]
    fn test_ir_to_rust_emit_function() -> CodegenResult<()> {
        let config = CodegenConfig::default();
        let gen = IrToRust::new(config);
        let params = vec![("x".to_string(), IrType::Int)];
        let body = IrExpr::Var("x".to_string());
        let output = gen.emit_function("identity", &params, &IrType::Int, &body)?;
        assert!(output.contains("fn identity"));
        assert!(output.contains("x: i64"));
        Ok(())
    }
    #[test]
    fn test_optimizer_constant_fold_if_true() -> CodegenResult<()> {
        let config = CodegenConfig::default();
        let opt = Optimizer::new(config);
        let expr = IrExpr::If {
            cond: Box::new(IrExpr::Lit(IrLit::Bool(true))),
            then_branch: Box::new(IrExpr::Lit(IrLit::Int(1))),
            else_branch: Box::new(IrExpr::Lit(IrLit::Int(0))),
        };
        let result = opt.constant_fold(&expr)?;
        match result {
            IrExpr::Lit(IrLit::Int(n)) => assert_eq!(n, 1),
            _ => panic!("Expected constant 1"),
        }
        Ok(())
    }
    #[test]
    fn test_optimizer_constant_fold_if_false() -> CodegenResult<()> {
        let config = CodegenConfig::default();
        let opt = Optimizer::new(config);
        let expr = IrExpr::If {
            cond: Box::new(IrExpr::Lit(IrLit::Bool(false))),
            then_branch: Box::new(IrExpr::Lit(IrLit::Int(1))),
            else_branch: Box::new(IrExpr::Lit(IrLit::Int(0))),
        };
        let result = opt.constant_fold(&expr)?;
        match result {
            IrExpr::Lit(IrLit::Int(n)) => assert_eq!(n, 0),
            _ => panic!("Expected constant 0"),
        }
        Ok(())
    }
    #[test]
    fn test_optimizer_dead_code_elimination() -> CodegenResult<()> {
        let config = CodegenConfig::default();
        let opt = Optimizer::new(config);
        let expr = IrExpr::Let {
            name: "unused".to_string(),
            ty: IrType::Int,
            value: Box::new(IrExpr::Lit(IrLit::Int(42))),
            body: Box::new(IrExpr::Lit(IrLit::Int(0))),
        };
        let result = opt.dead_code_eliminate(&expr)?;
        match result {
            IrExpr::Lit(IrLit::Int(n)) => assert_eq!(n, 0),
            _ => panic!("Expected dead code removed"),
        }
        Ok(())
    }
    #[test]
    fn test_ir_type_function() {
        let ty = IrType::Function {
            params: vec![IrType::Int, IrType::Bool],
            ret: Box::new(IrType::String),
        };
        let output = ty.to_string();
        assert!(output.contains("fn("));
        assert!(output.contains("i64"));
        assert!(output.contains("bool"));
    }
    #[test]
    fn test_ir_type_struct() {
        let ty = IrType::Struct {
            name: "Point".to_string(),
            fields: vec![
                ("x".to_string(), IrType::Int),
                ("y".to_string(), IrType::Int),
            ],
        };
        let output = ty.to_string();
        assert!(output.contains("struct Point"));
        assert!(output.contains("x: i64"));
        assert!(output.contains("y: i64"));
    }
    #[test]
    fn test_ir_type_array() {
        let ty = IrType::Array {
            elem: Box::new(IrType::Int),
            len: 10,
        };
        let output = ty.to_string();
        assert!(output.contains("[i64; 10]"));
    }
    #[test]
    fn test_ir_pattern_wildcard() {
        let pattern = IrPattern::Wildcard;
        match pattern {
            IrPattern::Wildcard => {}
            _ => panic!("Expected Wildcard"),
        }
    }
    #[test]
    fn test_ir_pattern_literal() {
        let pattern = IrPattern::Literal("true".to_string());
        match pattern {
            IrPattern::Literal(lit) => assert_eq!(lit, "true"),
            _ => panic!("Expected Literal"),
        }
    }
    #[test]
    fn test_ir_pattern_constructor() {
        let pattern = IrPattern::Constructor {
            name: "Some".to_string(),
            args: vec!["x".to_string()],
        };
        match pattern {
            IrPattern::Constructor { name, args } => {
                assert_eq!(name, "Some");
                assert_eq!(args.len(), 1);
            }
            _ => panic!("Expected Constructor"),
        }
    }
    #[test]
    fn test_ir_pattern_tuple() {
        let pattern = IrPattern::Tuple(vec!["x".to_string(), "y".to_string()]);
        match pattern {
            IrPattern::Tuple(vars) => assert_eq!(vars.len(), 2),
            _ => panic!("Expected Tuple"),
        }
    }
    #[test]
    fn test_codegen_pipeline_new() {
        let config = CodegenConfig::default();
        let _pipeline = CodegenPipeline::new(config);
    }
    #[test]
    fn test_ir_to_c_emit_var() -> CodegenResult<()> {
        let config = CodegenConfig::default();
        let gen = IrToC::new(config);
        let expr = IrExpr::Var("x".to_string());
        let output = gen.emit(&expr)?;
        assert!(output.contains("x"));
        Ok(())
    }
    #[test]
    fn test_ir_to_c_emit_literal() -> CodegenResult<()> {
        let config = CodegenConfig::default();
        let gen = IrToC::new(config);
        let expr = IrExpr::Lit(IrLit::Int(42));
        let output = gen.emit(&expr)?;
        assert!(output.contains("42"));
        Ok(())
    }
    #[test]
    fn test_ir_to_c_emit_app() -> CodegenResult<()> {
        let config = CodegenConfig::default();
        let gen = IrToC::new(config);
        let expr = IrExpr::App {
            func: Box::new(IrExpr::Var("f".to_string())),
            args: vec![IrExpr::Lit(IrLit::Int(1))],
        };
        let output = gen.emit(&expr)?;
        assert!(output.contains("f("));
        assert!(output.contains("1"));
        Ok(())
    }
    #[test]
    fn test_ir_to_c_emit_field() -> CodegenResult<()> {
        let config = CodegenConfig::default();
        let gen = IrToC::new(config);
        let expr = IrExpr::Field {
            object: Box::new(IrExpr::Var("point".to_string())),
            field: "x".to_string(),
        };
        let output = gen.emit(&expr)?;
        assert!(output.contains("point"));
        assert!(output.contains("->x"));
        Ok(())
    }
    #[test]
    fn test_ir_to_c_unsupported_let() -> CodegenResult<()> {
        let config = CodegenConfig::default();
        let gen = IrToC::new(config);
        let expr = IrExpr::Let {
            name: "x".to_string(),
            ty: IrType::Int,
            value: Box::new(IrExpr::Lit(IrLit::Int(5))),
            body: Box::new(IrExpr::Var("x".to_string())),
        };
        let result = gen.emit(&expr);
        assert!(result.is_err());
        Ok(())
    }
    #[test]
    fn test_ir_match_arm() {
        let arm = IrMatchArm {
            pattern: IrPattern::Literal("true".to_string()),
            body: Box::new(IrExpr::Lit(IrLit::Int(1))),
        };
        assert_eq!(arm.pattern, IrPattern::Literal("true".to_string()));
    }
    #[test]
    fn test_ir_to_rust_type_conversion() -> CodegenResult<()> {
        let config = CodegenConfig::default();
        let gen = IrToRust::new(config);
        assert_eq!(gen.emit_type(&IrType::Bool)?, "bool");
        assert_eq!(gen.emit_type(&IrType::Int)?, "i64");
        assert_eq!(gen.emit_type(&IrType::Nat)?, "u64");
        Ok(())
    }
    #[test]
    fn test_optimizer_is_var_used() {
        let config = CodegenConfig::default();
        let opt = Optimizer::new(config);
        let expr = IrExpr::Var("x".to_string());
        assert!(opt.is_var_used("x", &expr));
        assert!(!opt.is_var_used("y", &expr));
    }
    #[test]
    fn test_optimizer_inline() -> CodegenResult<()> {
        let config = CodegenConfig::default();
        let opt = Optimizer::new(config);
        let expr = IrExpr::Lit(IrLit::Int(42));
        let result = opt.inline(&expr)?;
        assert_eq!(result, expr);
        Ok(())
    }
    #[test]
    fn test_optimizer_cse() -> CodegenResult<()> {
        let config = CodegenConfig::default();
        let opt = Optimizer::new(config);
        let expr = IrExpr::Lit(IrLit::Int(42));
        let result = opt.common_subexpr_eliminate(&expr)?;
        assert_eq!(result, expr);
        Ok(())
    }
    #[test]
    fn test_closure_vars_tracking() {
        let compiler = ExprToIr::new();
        assert!(compiler.closure_vars.is_empty());
    }
    #[test]
    fn test_ir_string_literal() {
        let lit = IrLit::String("hello".to_string());
        assert_eq!(lit.to_string(), "\"hello\"");
    }
    #[test]
    fn test_ir_to_rust_emit_lambda() -> CodegenResult<()> {
        let config = CodegenConfig::default();
        let gen = IrToRust::new(config);
        let expr = IrExpr::Lambda {
            params: vec![("x".to_string(), IrType::Int)],
            body: Box::new(IrExpr::Var("x".to_string())),
            captured: vec![],
        };
        let output = gen.emit(&expr)?;
        assert!(output.contains("|"));
        Ok(())
    }
    #[test]
    fn test_ir_to_rust_emit_match() -> CodegenResult<()> {
        let config = CodegenConfig::default();
        let gen = IrToRust::new(config);
        let arms = vec![IrMatchArm {
            pattern: IrPattern::Literal("true".to_string()),
            body: Box::new(IrExpr::Lit(IrLit::Int(1))),
        }];
        let output = gen.emit_match(&IrExpr::Lit(IrLit::Bool(true)), &arms)?;
        assert!(output.contains("match"));
        Ok(())
    }
    #[test]
    fn test_ir_to_rust_emit_struct_expr() -> CodegenResult<()> {
        let config = CodegenConfig::default();
        let gen = IrToRust::new(config);
        let expr = IrExpr::Struct {
            name: "Point".to_string(),
            fields: vec![("x".to_string(), IrExpr::Lit(IrLit::Int(1)))],
        };
        let output = gen.emit(&expr)?;
        assert!(output.contains("Point"));
        assert!(output.contains("x"));
        Ok(())
    }
}
#[cfg(test)]
mod Lib_infra_tests {
    use super::*;
    #[test]
    fn test_pass_config() {
        let config = LibPassConfig::new("test_pass", LibPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    fn test_pass_stats() {
        let mut stats = LibPassStats::new();
        stats.record_run(10, 100, 3);
        stats.record_run(20, 200, 5);
        assert_eq!(stats.total_runs, 2);
        assert!((stats.average_changes_per_run() - 15.0).abs() < 0.01);
        assert!((stats.success_rate() - 1.0).abs() < 0.01);
        let s = stats.format_summary();
        assert!(s.contains("Runs: 2/2"));
    }
    #[test]
    fn test_pass_registry() {
        let mut reg = LibPassRegistry::new();
        reg.register(LibPassConfig::new("pass_a", LibPassPhase::Analysis));
        reg.register(LibPassConfig::new("pass_b", LibPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    fn test_analysis_cache() {
        let mut cache = LibAnalysisCache::new(10);
        cache.insert("key1".to_string(), vec![1, 2, 3]);
        assert!(cache.get("key1").is_some());
        assert!(cache.get("key2").is_none());
        assert!((cache.hit_rate() - 0.5).abs() < 0.01);
        cache.invalidate("key1");
        assert!(!cache.entries["key1"].valid);
        assert_eq!(cache.size(), 1);
    }
    #[test]
    fn test_worklist() {
        let mut wl = LibWorklist::new();
        assert!(wl.push(1));
        assert!(wl.push(2));
        assert!(!wl.push(1));
        assert_eq!(wl.len(), 2);
        assert_eq!(wl.pop(), Some(1));
        assert!(!wl.contains(1));
        assert!(wl.contains(2));
    }
    #[test]
    fn test_dominator_tree() {
        let mut dt = LibDominatorTree::new(5);
        dt.set_idom(1, 0);
        dt.set_idom(2, 0);
        dt.set_idom(3, 1);
        assert!(dt.dominates(0, 3));
        assert!(dt.dominates(1, 3));
        assert!(!dt.dominates(2, 3));
        assert!(dt.dominates(3, 3));
    }
    #[test]
    fn test_liveness() {
        let mut liveness = LibLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    fn test_constant_folding() {
        assert_eq!(LibConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(LibConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(LibConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            LibConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(LibConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    fn test_dep_graph() {
        let mut g = LibDepGraph::new();
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
