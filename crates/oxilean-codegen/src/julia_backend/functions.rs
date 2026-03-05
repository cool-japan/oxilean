//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;

use super::types::{
    JulAnalysisCache, JulConstantFoldingHelper, JulDepGraph, JulDominatorTree, JulLivenessInfo,
    JulPassConfig, JulPassPhase, JulPassRegistry, JulPassStats, JulWorklist, JuliaBackend,
    JuliaExpr, JuliaExprDisplay, JuliaExtCache, JuliaExtConstFolder, JuliaExtDepGraph,
    JuliaExtDomTree, JuliaExtLiveness, JuliaExtPassConfig, JuliaExtPassPhase, JuliaExtPassRegistry,
    JuliaExtPassStats, JuliaExtWorklist, JuliaFunction, JuliaModule, JuliaParam, JuliaStmt,
    JuliaStmtDisplay, JuliaStringPart, JuliaStruct, JuliaType,
};
use std::fmt;

pub(super) fn emit_expr(f: &mut fmt::Formatter<'_>, expr: &JuliaExpr) -> fmt::Result {
    match expr {
        JuliaExpr::IntLit(n) => write!(f, "{}", n),
        JuliaExpr::UIntLit(n) => write!(f, "0x{:x}", n),
        JuliaExpr::FloatLit(v) => {
            if v.fract() == 0.0 {
                write!(f, "{:.1}", v)
            } else {
                write!(f, "{}", v)
            }
        }
        JuliaExpr::BoolLit(b) => write!(f, "{}", b),
        JuliaExpr::StringLit(s) => write!(f, "\"{}\"", s.replace('"', "\\\"")),
        JuliaExpr::CharLit(c) => write!(f, "'{}'", c),
        JuliaExpr::Nothing => write!(f, "nothing"),
        JuliaExpr::Var(name) => write!(f, "{}", name),
        JuliaExpr::Field(obj, field) => write!(f, "{}.{}", JuliaExprDisplay(obj), field),
        JuliaExpr::Index(arr, idxs) => {
            emit_expr(f, arr)?;
            write!(f, "[")?;
            for (i, idx) in idxs.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                emit_expr(f, idx)?;
            }
            write!(f, "]")
        }
        JuliaExpr::Slice(arr, lo, hi) => {
            emit_expr(f, arr)?;
            write!(f, "[")?;
            if let Some(ref lo) = lo {
                emit_expr(f, lo)?;
            } else {
                write!(f, "begin")?;
            }
            write!(f, ":")?;
            if let Some(ref hi) = hi {
                emit_expr(f, hi)?;
            } else {
                write!(f, "end")?;
            }
            write!(f, "]")
        }
        JuliaExpr::Call(func, args) => {
            emit_expr(f, func)?;
            write!(f, "(")?;
            for (i, a) in args.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                emit_expr(f, a)?;
            }
            write!(f, ")")
        }
        JuliaExpr::CallKw(func, args, kwargs) => {
            emit_expr(f, func)?;
            write!(f, "(")?;
            for (i, a) in args.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                emit_expr(f, a)?;
            }
            if !kwargs.is_empty() {
                if !args.is_empty() {
                    write!(f, "; ")?;
                } else {
                    write!(f, ";")?;
                }
                for (i, (k, v)) in kwargs.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}=", k)?;
                    emit_expr(f, v)?;
                }
            }
            write!(f, ")")
        }
        JuliaExpr::Broadcast(func, args) => {
            emit_expr(f, func)?;
            write!(f, ".(")?;
            for (i, a) in args.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                emit_expr(f, a)?;
            }
            write!(f, ")")
        }
        JuliaExpr::BinOp(op, lhs, rhs) => {
            write!(f, "(")?;
            emit_expr(f, lhs)?;
            write!(f, " {} ", op)?;
            emit_expr(f, rhs)?;
            write!(f, ")")
        }
        JuliaExpr::UnOp(op, operand) => {
            write!(f, "{}", op)?;
            emit_expr(f, operand)
        }
        JuliaExpr::CompareChain(exprs, ops) => {
            write!(f, "(")?;
            for (i, expr) in exprs.iter().enumerate() {
                if i > 0 {
                    write!(f, " {} ", ops[i - 1])?;
                }
                emit_expr(f, expr)?;
            }
            write!(f, ")")
        }
        JuliaExpr::ArrayLit(elems) => {
            write!(f, "[")?;
            for (i, e) in elems.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                emit_expr(f, e)?;
            }
            write!(f, "]")
        }
        JuliaExpr::MatrixLit(rows) => {
            write!(f, "[")?;
            for (i, row) in rows.iter().enumerate() {
                if i > 0 {
                    write!(f, "; ")?;
                }
                for (j, e) in row.iter().enumerate() {
                    if j > 0 {
                        write!(f, " ")?;
                    }
                    emit_expr(f, e)?;
                }
            }
            write!(f, "]")
        }
        JuliaExpr::Range(lo, step, hi) => {
            emit_expr(f, lo)?;
            if let Some(ref s) = step {
                write!(f, ":")?;
                emit_expr(f, s)?;
            }
            write!(f, ":")?;
            emit_expr(f, hi)
        }
        JuliaExpr::TupleLit(elems) => {
            write!(f, "(")?;
            for (i, e) in elems.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                emit_expr(f, e)?;
            }
            if elems.len() == 1 {
                write!(f, ",")?;
            }
            write!(f, ")")
        }
        JuliaExpr::ArrayComp(body, clauses, guard) => {
            write!(f, "[")?;
            emit_expr(f, body)?;
            for (var, iter) in clauses {
                write!(f, " for {} in ", var)?;
                emit_expr(f, iter)?;
            }
            if let Some(ref g) = guard {
                write!(f, " if ")?;
                emit_expr(f, g)?;
            }
            write!(f, "]")
        }
        JuliaExpr::Generator(body, clauses, guard) => {
            write!(f, "(")?;
            emit_expr(f, body)?;
            for (var, iter) in clauses {
                write!(f, " for {} in ", var)?;
                emit_expr(f, iter)?;
            }
            if let Some(ref g) = guard {
                write!(f, " if ")?;
                emit_expr(f, g)?;
            }
            write!(f, ")")
        }
        JuliaExpr::DictComp(k, v, clauses) => {
            write!(f, "Dict(")?;
            emit_expr(f, k)?;
            write!(f, " => ")?;
            emit_expr(f, v)?;
            for (var, iter) in clauses {
                write!(f, " for {} in ", var)?;
                emit_expr(f, iter)?;
            }
            write!(f, ")")
        }
        JuliaExpr::Lambda(params, body) => {
            if params.len() == 1 {
                write!(f, "{}", params[0])?;
            } else {
                write!(f, "(")?;
                for (i, p) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", p)?;
                }
                write!(f, ")")?;
            }
            write!(f, " -> ")?;
            emit_expr(f, body)
        }
        JuliaExpr::DoBlock(func, params, body) => {
            emit_expr(f, func)?;
            write!(f, " do")?;
            if !params.is_empty() {
                write!(f, " {}", params.join(", "))?;
            }
            writeln!(f)?;
            for stmt in body {
                writeln!(f, "    {}", JuliaStmtDisplay(stmt))?;
            }
            write!(f, "end")
        }
        JuliaExpr::Ternary(cond, then_, else_) => {
            emit_expr(f, cond)?;
            write!(f, " ? ")?;
            emit_expr(f, then_)?;
            write!(f, " : ")?;
            emit_expr(f, else_)
        }
        JuliaExpr::TypeAssert(expr, ty) => {
            write!(f, "(")?;
            emit_expr(f, expr)?;
            write!(f, ")::{}", ty)
        }
        JuliaExpr::Convert(ty, expr) => {
            write!(f, "convert({}, ", ty)?;
            emit_expr(f, expr)?;
            write!(f, ")")
        }
        JuliaExpr::IsA(expr, ty) => {
            emit_expr(f, expr)?;
            write!(f, " isa {}", ty)
        }
        JuliaExpr::TypeOf(expr) => {
            write!(f, "typeof(")?;
            emit_expr(f, expr)?;
            write!(f, ")")
        }
        JuliaExpr::Macro(name, args) => {
            write!(f, "@{}", name)?;
            for a in args {
                write!(f, " ")?;
                emit_expr(f, a)?;
            }
            Ok(())
        }
        JuliaExpr::Interpolated(parts) => {
            write!(f, "\"")?;
            for part in parts {
                match part {
                    JuliaStringPart::Text(s) => write!(f, "{}", s)?,
                    JuliaStringPart::Expr(e) => {
                        write!(f, "$(")?;
                        emit_expr(f, e)?;
                        write!(f, ")")?;
                    }
                }
            }
            write!(f, "\"")
        }
        JuliaExpr::Splat(expr) => {
            emit_expr(f, expr)?;
            write!(f, "...")
        }
        JuliaExpr::NamedArg(name, val) => {
            write!(f, "{}=", name)?;
            emit_expr(f, val)
        }
        JuliaExpr::Pair(k, v) => {
            emit_expr(f, k)?;
            write!(f, " => ")?;
            emit_expr(f, v)
        }
        JuliaExpr::Block(stmts) => {
            writeln!(f, "begin")?;
            for s in stmts {
                writeln!(f, "    {}", JuliaStmtDisplay(s))?;
            }
            write!(f, "end")
        }
    }
}
pub(super) fn emit_stmt_inline(f: &mut fmt::Formatter<'_>, stmt: &JuliaStmt) -> fmt::Result {
    match stmt {
        JuliaStmt::Expr(e) => emit_expr(f, e),
        JuliaStmt::Assign(lhs, rhs) => {
            emit_expr(f, lhs)?;
            write!(f, " = ")?;
            emit_expr(f, rhs)
        }
        JuliaStmt::Return(Some(e)) => {
            write!(f, "return ")?;
            emit_expr(f, e)
        }
        JuliaStmt::Return(None) => write!(f, "return"),
        JuliaStmt::Break => write!(f, "break"),
        JuliaStmt::Continue => write!(f, "continue"),
        JuliaStmt::Comment(s) => write!(f, "# {}", s),
        JuliaStmt::Blank => write!(f, ""),
        _ => write!(f, "# (complex stmt)"),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub(super) fn test_julia_type_display() {
        assert_eq!(JuliaType::Int64.to_string(), "Int64");
        assert_eq!(JuliaType::Float64.to_string(), "Float64");
        assert_eq!(JuliaType::Bool.to_string(), "Bool");
        assert_eq!(JuliaType::String.to_string(), "String");
        assert_eq!(JuliaType::Nothing.to_string(), "Nothing");
        assert_eq!(JuliaType::Any.to_string(), "Any");
        assert_eq!(
            JuliaType::Vector(Box::new(JuliaType::Float64)).to_string(),
            "Vector{Float64}"
        );
        assert_eq!(
            JuliaType::Matrix(Box::new(JuliaType::Int32)).to_string(),
            "Matrix{Int32}"
        );
    }
    #[test]
    pub(super) fn test_julia_parametric_type_display() {
        let ty = JuliaType::Parametric(
            "Dict".to_string(),
            vec![JuliaType::String, JuliaType::Int64],
        );
        assert_eq!(ty.to_string(), "Dict{String, Int64}");
        let union_ty = JuliaType::Union(vec![JuliaType::Int64, JuliaType::Nothing]);
        assert_eq!(union_ty.to_string(), "Union{Int64, Nothing}");
        let tuple_ty = JuliaType::Tuple(vec![JuliaType::Int64, JuliaType::Float64]);
        assert_eq!(tuple_ty.to_string(), "Tuple{Int64, Float64}");
    }
    #[test]
    pub(super) fn test_julia_expr_literals() {
        let mut backend = JuliaBackend::new();
        assert_eq!(backend.emit_expr(&JuliaExpr::IntLit(42)), "42");
        assert_eq!(backend.emit_expr(&JuliaExpr::FloatLit(3.14)), "3.14");
        assert_eq!(backend.emit_expr(&JuliaExpr::BoolLit(true)), "true");
        assert_eq!(
            backend.emit_expr(&JuliaExpr::StringLit("hello".to_string())),
            "\"hello\""
        );
        assert_eq!(backend.emit_expr(&JuliaExpr::Nothing), "nothing");
    }
    #[test]
    pub(super) fn test_julia_binop_and_unop() {
        let mut backend = JuliaBackend::new();
        let binop = JuliaExpr::BinOp(
            "+".to_string(),
            Box::new(JuliaExpr::Var("x".to_string())),
            Box::new(JuliaExpr::IntLit(1)),
        );
        assert_eq!(backend.emit_expr(&binop), "(x + 1)");
        let unop = JuliaExpr::UnOp("-".to_string(), Box::new(JuliaExpr::Var("y".to_string())));
        assert_eq!(backend.emit_expr(&unop), "-y");
    }
    #[test]
    pub(super) fn test_julia_function_emit() {
        let mut backend = JuliaBackend::new();
        let func = JuliaFunction::new("add")
            .with_type_param_bound("T", "Number")
            .with_param(JuliaParam::typed("a", JuliaType::TypeVar("T".to_string())))
            .with_param(JuliaParam::typed("b", JuliaType::TypeVar("T".to_string())))
            .with_return_type(JuliaType::TypeVar("T".to_string()))
            .with_body(vec![JuliaStmt::Return(Some(JuliaExpr::BinOp(
                "+".to_string(),
                Box::new(JuliaExpr::Var("a".to_string())),
                Box::new(JuliaExpr::Var("b".to_string())),
            )))]);
        backend.emit_function(&func);
        let out = backend.take_output();
        assert!(
            out.contains("function add{T <: Number}"),
            "missing signature: {}",
            out
        );
        assert!(
            out.contains("::T"),
            "missing return type annotation: {}",
            out
        );
        assert!(out.contains("return"), "missing return: {}", out);
        assert!(out.contains("end"), "missing end: {}", out);
    }
    #[test]
    pub(super) fn test_julia_struct_emit() {
        let mut backend = JuliaBackend::new();
        let s = JuliaStruct::new("Point")
            .with_type_param("T")
            .with_supertype("AbstractPoint")
            .with_field("x", JuliaType::TypeVar("T".to_string()))
            .with_field("y", JuliaType::TypeVar("T".to_string()));
        backend.emit_struct(&s);
        let out = backend.take_output();
        assert!(out.contains("struct Point{T}"), "missing header: {}", out);
        assert!(
            out.contains("<: AbstractPoint"),
            "missing supertype: {}",
            out
        );
        assert!(out.contains("x::T"), "missing field x: {}", out);
        assert!(out.contains("y::T"), "missing field y: {}", out);
        assert!(out.contains("end"), "missing end: {}", out);
    }
    #[test]
    pub(super) fn test_julia_multiple_dispatch() {
        let mut backend = JuliaBackend::new();
        let m1 = JuliaFunction::new("add")
            .with_param(JuliaParam::typed("a", JuliaType::Int64))
            .with_param(JuliaParam::typed("b", JuliaType::Int64))
            .with_return_type(JuliaType::Int64)
            .with_body(vec![JuliaStmt::Return(Some(JuliaExpr::BinOp(
                "+".to_string(),
                Box::new(JuliaExpr::Var("a".to_string())),
                Box::new(JuliaExpr::Var("b".to_string())),
            )))]);
        let m2 = JuliaFunction::new("add")
            .with_param(JuliaParam::typed("a", JuliaType::Float64))
            .with_param(JuliaParam::typed("b", JuliaType::Float64))
            .with_return_type(JuliaType::Float64)
            .with_body(vec![JuliaStmt::Return(Some(JuliaExpr::BinOp(
                "+".to_string(),
                Box::new(JuliaExpr::Var("a".to_string())),
                Box::new(JuliaExpr::Var("b".to_string())),
            )))]);
        backend.register_method(m1);
        backend.register_method(m2);
        let table = backend
            .dispatch_tables
            .get("add")
            .expect("table should be present in map");
        assert_eq!(table.num_methods(), 2);
        let found = table.find_method(&[JuliaType::Int64, JuliaType::Int64]);
        assert!(found.is_some());
        assert_eq!(
            found.expect("value should be Some/Ok").return_type,
            Some(JuliaType::Int64)
        );
        let found2 = table.find_method(&[JuliaType::Float64, JuliaType::Float64]);
        assert!(found2.is_some());
        assert_eq!(
            found2.expect("value should be Some/Ok").return_type,
            Some(JuliaType::Float64)
        );
    }
    #[test]
    pub(super) fn test_julia_array_comprehension() {
        let mut backend = JuliaBackend::new();
        let comp = JuliaExpr::ArrayComp(
            Box::new(JuliaExpr::BinOp(
                "*".to_string(),
                Box::new(JuliaExpr::Var("x".to_string())),
                Box::new(JuliaExpr::Var("x".to_string())),
            )),
            vec![(
                "x".to_string(),
                JuliaExpr::Range(
                    Box::new(JuliaExpr::IntLit(1)),
                    None,
                    Box::new(JuliaExpr::IntLit(10)),
                ),
            )],
            Some(Box::new(JuliaExpr::BinOp(
                ">".to_string(),
                Box::new(JuliaExpr::Var("x".to_string())),
                Box::new(JuliaExpr::IntLit(3)),
            ))),
        );
        let s = backend.emit_expr(&comp);
        assert!(s.contains("for x in"), "missing for clause: {}", s);
        assert!(s.contains("if"), "missing guard: {}", s);
    }
    #[test]
    pub(super) fn test_julia_module_emit() {
        let mut backend = JuliaBackend::new();
        let m = JuliaModule::new("MyMath")
            .using(vec!["LinearAlgebra".to_string()])
            .export("dot_product")
            .push(JuliaStmt::Comment("Module body".to_string()));
        backend.emit_module(&m);
        let out = backend.take_output();
        assert!(
            out.contains("module MyMath"),
            "missing module header: {}",
            out
        );
        assert!(
            out.contains("using LinearAlgebra"),
            "missing using: {}",
            out
        );
        assert!(
            out.contains("export dot_product"),
            "missing export: {}",
            out
        );
        assert!(out.contains("end"), "missing end: {}", out);
    }
    #[test]
    pub(super) fn test_julia_if_for_while_stmts() {
        let mut backend = JuliaBackend::new();
        let if_stmt = JuliaStmt::If {
            cond: JuliaExpr::BoolLit(true),
            then_body: vec![JuliaStmt::Return(Some(JuliaExpr::IntLit(1)))],
            elseif_branches: vec![],
            else_body: Some(vec![JuliaStmt::Return(Some(JuliaExpr::IntLit(0)))]),
        };
        backend.emit_stmt(&if_stmt);
        let out1 = backend.take_output();
        assert!(out1.contains("if true"), "missing if: {}", out1);
        assert!(out1.contains("else"), "missing else: {}", out1);
        let for_stmt = JuliaStmt::For {
            vars: vec!["i".to_string()],
            iter: JuliaExpr::Range(
                Box::new(JuliaExpr::IntLit(1)),
                None,
                Box::new(JuliaExpr::IntLit(10)),
            ),
            body: vec![JuliaStmt::Continue],
        };
        backend.emit_stmt(&for_stmt);
        let out2 = backend.take_output();
        assert!(out2.contains("for i in"), "missing for: {}", out2);
        let while_stmt = JuliaStmt::While {
            cond: JuliaExpr::BoolLit(true),
            body: vec![JuliaStmt::Break],
        };
        backend.emit_stmt(&while_stmt);
        let out3 = backend.take_output();
        assert!(out3.contains("while true"), "missing while: {}", out3);
    }
}
#[cfg(test)]
mod Jul_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = JulPassConfig::new("test_pass", JulPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = JulPassStats::new();
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
        let mut reg = JulPassRegistry::new();
        reg.register(JulPassConfig::new("pass_a", JulPassPhase::Analysis));
        reg.register(JulPassConfig::new("pass_b", JulPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = JulAnalysisCache::new(10);
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
        let mut wl = JulWorklist::new();
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
        let mut dt = JulDominatorTree::new(5);
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
        let mut liveness = JulLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(JulConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(JulConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(JulConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            JulConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(JulConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = JulDepGraph::new();
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
mod juliaext_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_juliaext_phase_order() {
        assert_eq!(JuliaExtPassPhase::Early.order(), 0);
        assert_eq!(JuliaExtPassPhase::Middle.order(), 1);
        assert_eq!(JuliaExtPassPhase::Late.order(), 2);
        assert_eq!(JuliaExtPassPhase::Finalize.order(), 3);
        assert!(JuliaExtPassPhase::Early.is_early());
        assert!(!JuliaExtPassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_juliaext_config_builder() {
        let c = JuliaExtPassConfig::new("p")
            .with_phase(JuliaExtPassPhase::Late)
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
    pub(super) fn test_juliaext_stats() {
        let mut s = JuliaExtPassStats::new();
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
    pub(super) fn test_juliaext_registry() {
        let mut r = JuliaExtPassRegistry::new();
        r.register(JuliaExtPassConfig::new("a").with_phase(JuliaExtPassPhase::Early));
        r.register(JuliaExtPassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&JuliaExtPassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_juliaext_cache() {
        let mut c = JuliaExtCache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_juliaext_worklist() {
        let mut w = JuliaExtWorklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_juliaext_dom_tree() {
        let mut dt = JuliaExtDomTree::new(5);
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
    pub(super) fn test_juliaext_liveness() {
        let mut lv = JuliaExtLiveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_juliaext_const_folder() {
        let mut cf = JuliaExtConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_juliaext_dep_graph() {
        let mut g = JuliaExtDepGraph::new(4);
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
