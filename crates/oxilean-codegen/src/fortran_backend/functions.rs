//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::{HashMap, HashSet};

use super::types::{
    ArrayDimension, FortAnalysisCache, FortConstantFoldingHelper, FortDepGraph, FortDominatorTree,
    FortLivenessInfo, FortPassConfig, FortPassPhase, FortPassRegistry, FortPassStats, FortWorklist,
    FortranBackend, FortranBinOp, FortranDecl, FortranDerivedType, FortranExpr, FortranExtCache,
    FortranExtConstFolder, FortranExtDepGraph, FortranExtDomTree, FortranExtLiveness,
    FortranExtPassConfig, FortranExtPassPhase, FortranExtPassRegistry, FortranExtPassStats,
    FortranExtWorklist, FortranLit, FortranModule, FortranStmt, FortranSubprogram, FortranType,
};

/// Map an LCNF type to a Fortran type.
pub(super) fn lcnf_type_to_fortran(ty: &LcnfType) -> FortranType {
    match ty {
        LcnfType::Nat => FortranType::FtIntegerK(8),
        LcnfType::LcnfString => FortranType::FtCharacterStar,
        LcnfType::Unit | LcnfType::Erased | LcnfType::Irrelevant => FortranType::FtLogical,
        LcnfType::Object => FortranType::FtClassStar,
        LcnfType::Var(name) => FortranType::FtDerived(name.clone()),
        LcnfType::Fun(_, _) => FortranType::FtClassStar,
        LcnfType::Ctor(name, _) => FortranType::FtDerived(name.clone()),
    }
}
pub(super) fn mangle_fortran_ident(name: &str, reserved: &HashSet<String>) -> String {
    let base: String = name
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect();
    let base = if base.starts_with(|c: char| c.is_ascii_digit()) {
        format!("ox_{}", base)
    } else {
        base
    };
    let base = if base.is_empty() {
        "ox_empty".to_string()
    } else {
        base
    };
    let base: String = base.chars().take(63).collect();
    if reserved.contains(&base.to_lowercase()) {
        format!("ox_{}", base)
    } else {
        base
    }
}
pub(super) fn collect_ctor_names_module(module: &LcnfModule) -> HashSet<String> {
    let mut out = HashSet::new();
    for func in &module.fun_decls {
        collect_ctor_names_expr(&func.body, &mut out);
    }
    out
}
pub(super) fn collect_ctor_names_expr(expr: &LcnfExpr, out: &mut HashSet<String>) {
    match expr {
        LcnfExpr::Let { value, body, .. } => {
            collect_ctor_names_value(value, out);
            collect_ctor_names_expr(body, out);
        }
        LcnfExpr::Case { alts, default, .. } => {
            for alt in alts {
                out.insert(alt.ctor_name.clone());
                collect_ctor_names_expr(&alt.body, out);
            }
            if let Some(d) = default {
                collect_ctor_names_expr(d, out);
            }
        }
        LcnfExpr::Return(_) | LcnfExpr::Unreachable | LcnfExpr::TailCall(_, _) => {}
    }
}
pub(super) fn collect_ctor_names_value(value: &LcnfLetValue, out: &mut HashSet<String>) {
    match value {
        LcnfLetValue::Ctor(name, _, _) => {
            out.insert(name.clone());
        }
        LcnfLetValue::Reuse(_, name, _, _) => {
            out.insert(name.clone());
        }
        _ => {}
    }
}
/// Build a Fortran derived type for an OxiLean constructor.
pub(super) fn make_ctor_derived_type(name: &str) -> FortranDerivedType {
    let mut dt = FortranDerivedType::new(name);
    dt.fields
        .push(FortranDecl::local(FortranType::FtIntegerK(4), "tag"));
    for i in 0..8 {
        dt.fields.push(FortranDecl::local(
            FortranType::FtIntegerK(8),
            format!("field{}", i),
        ));
    }
    dt
}
pub static FORTRAN_KEYWORDS: &[&str] = &[
    "allocatable",
    "allocate",
    "assign",
    "assignment",
    "associate",
    "asynchronous",
    "backspace",
    "block",
    "call",
    "case",
    "class",
    "close",
    "codimension",
    "common",
    "complex",
    "contains",
    "contiguous",
    "continue",
    "critical",
    "cycle",
    "data",
    "deallocate",
    "default",
    "deferred",
    "dimension",
    "do",
    "double",
    "else",
    "elseif",
    "elsewhere",
    "end",
    "endblock",
    "enddo",
    "endfile",
    "endif",
    "entry",
    "enum",
    "enumerator",
    "equivalence",
    "error",
    "event",
    "exit",
    "extends",
    "external",
    "final",
    "flush",
    "forall",
    "format",
    "function",
    "generic",
    "go",
    "goto",
    "if",
    "images",
    "implicit",
    "import",
    "impure",
    "in",
    "include",
    "inout",
    "integer",
    "intent",
    "interface",
    "intrinsic",
    "kind",
    "len",
    "local",
    "lock",
    "logical",
    "memory",
    "module",
    "moduleprocedure",
    "namelist",
    "non_overridable",
    "none",
    "nopass",
    "nullify",
    "only",
    "open",
    "operator",
    "optional",
    "out",
    "parameter",
    "pass",
    "pause",
    "pointer",
    "print",
    "private",
    "procedure",
    "program",
    "protected",
    "public",
    "pure",
    "read",
    "real",
    "recursive",
    "result",
    "return",
    "rewind",
    "save",
    "select",
    "sequence",
    "stop",
    "submodule",
    "subroutine",
    "sync",
    "target",
    "then",
    "type",
    "unlock",
    "use",
    "value",
    "volatile",
    "wait",
    "where",
    "while",
    "write",
];
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub(super) fn test_fortran_type_display_primitives() {
        assert_eq!(format!("{}", FortranType::FtInteger), "INTEGER");
        assert_eq!(format!("{}", FortranType::FtIntegerK(8)), "INTEGER(KIND=8)");
        assert_eq!(format!("{}", FortranType::FtReal), "REAL");
        assert_eq!(format!("{}", FortranType::FtDouble), "REAL(KIND=8)");
        assert_eq!(format!("{}", FortranType::FtLogical), "LOGICAL");
        assert_eq!(
            format!("{}", FortranType::FtCharacterStar),
            "CHARACTER(LEN=*)"
        );
        assert_eq!(
            format!("{}", FortranType::FtCharacter(Some(80))),
            "CHARACTER(LEN=80)"
        );
    }
    #[test]
    pub(super) fn test_fortran_type_display_array() {
        let arr = FortranType::FtArray(
            Box::new(FortranType::FtDouble),
            ArrayDimension::Explicit(10),
        );
        assert_eq!(format!("{}", arr), "REAL(KIND=8), DIMENSION(10)");
        let deferred =
            FortranType::FtArray(Box::new(FortranType::FtInteger), ArrayDimension::Deferred);
        assert_eq!(format!("{}", deferred), "INTEGER, DIMENSION(:)");
    }
    #[test]
    pub(super) fn test_fortran_type_display_derived() {
        let dt = FortranType::FtDerived("MyType".to_string());
        assert_eq!(format!("{}", dt), "TYPE(MyType)");
    }
    #[test]
    pub(super) fn test_fortran_lit_display() {
        assert_eq!(format!("{}", FortranLit::Int(42)), "42_8");
        assert_eq!(format!("{}", FortranLit::Real(3.14)), "3.14_8");
        assert_eq!(format!("{}", FortranLit::Logical(true)), ".TRUE.");
        assert_eq!(format!("{}", FortranLit::Logical(false)), ".FALSE.");
        assert_eq!(
            format!("{}", FortranLit::Char("hello".to_string())),
            "'hello'"
        );
        assert_eq!(
            format!("{}", FortranLit::Char("it's".to_string())),
            "'it''s'"
        );
    }
    #[test]
    pub(super) fn test_fortran_expr_display() {
        let var = FortranExpr::Var("X".to_string());
        assert_eq!(format!("{}", var), "X");
        let component = FortranExpr::Component(
            Box::new(FortranExpr::Var("OBJ".to_string())),
            "field0".to_string(),
        );
        assert_eq!(format!("{}", component), "OBJ%field0");
        let bin = FortranExpr::BinOp(
            Box::new(FortranExpr::Lit(FortranLit::Int(1))),
            FortranBinOp::Add,
            Box::new(FortranExpr::Lit(FortranLit::Int(2))),
        );
        assert_eq!(format!("{}", bin), "(1_8 + 2_8)");
        let call = FortranExpr::Call("ABS".to_string(), vec![FortranExpr::Var("X".to_string())]);
        assert_eq!(format!("{}", call), "ABS(X)");
    }
    #[test]
    pub(super) fn test_fortran_stmt_emit_if() {
        let backend = FortranBackend::new();
        let stmt = FortranStmt::If(
            vec![(
                FortranExpr::BinOp(
                    Box::new(FortranExpr::Var("N".to_string())),
                    FortranBinOp::Gt,
                    Box::new(FortranExpr::Lit(FortranLit::Int(0))),
                ),
                vec![FortranStmt::Assign(
                    FortranExpr::Var("RES".to_string()),
                    FortranExpr::Lit(FortranLit::Int(1)),
                )],
            )],
            vec![FortranStmt::Assign(
                FortranExpr::Var("RES".to_string()),
                FortranExpr::Lit(FortranLit::Int(0)),
            )],
        );
        let code = backend.emit_stmt(&stmt, 0);
        assert!(code.contains("IF ((N > 0_8)) THEN"));
        assert!(code.contains("RES = 1_8"));
        assert!(code.contains("ELSE"));
        assert!(code.contains("RES = 0_8"));
        assert!(code.contains("END IF"));
    }
    #[test]
    pub(super) fn test_fortran_stmt_emit_do_count() {
        let backend = FortranBackend::new();
        let stmt = FortranStmt::DoCount(
            "I".to_string(),
            FortranExpr::Lit(FortranLit::Int(1)),
            FortranExpr::Lit(FortranLit::Int(10)),
            None,
            vec![FortranStmt::Print(vec![FortranExpr::Var("I".to_string())])],
        );
        let code = backend.emit_stmt(&stmt, 0);
        assert!(code.contains("DO I = 1_8, 10_8"));
        assert!(code.contains("PRINT *, I"));
        assert!(code.contains("END DO"));
    }
    #[test]
    pub(super) fn test_fortran_emit_subprogram_function() {
        let mut backend = FortranBackend::new();
        let mut func = FortranSubprogram::function("nat_add", FortranType::FtIntegerK(8));
        func.dummy_args = vec!["A".to_string(), "B".to_string()];
        func.decls = vec![
            FortranDecl::param_in(FortranType::FtIntegerK(8), "A"),
            FortranDecl::param_in(FortranType::FtIntegerK(8), "B"),
        ];
        func.body = vec![
            FortranStmt::Assign(
                FortranExpr::Var("NAT_ADD".to_string()),
                FortranExpr::BinOp(
                    Box::new(FortranExpr::Var("A".to_string())),
                    FortranBinOp::Add,
                    Box::new(FortranExpr::Var("B".to_string())),
                ),
            ),
            FortranStmt::Return,
        ];
        let code = backend.emit_subprogram(&func, 0);
        assert!(code.contains("INTEGER(KIND=8) FUNCTION NAT_ADD(A, B)"));
        assert!(code.contains("NAT_ADD = (A + B)"));
        assert!(code.contains("RETURN"));
        assert!(code.contains("END FUNCTION NAT_ADD"));
    }
    #[test]
    pub(super) fn test_fortran_emit_module() {
        let mut backend = FortranBackend::new();
        let mut module = FortranModule::new("test_mod");
        module.use_modules.push("iso_fortran_env".to_string());
        let mut func = FortranSubprogram::subroutine("hello_world");
        func.body = vec![FortranStmt::Print(vec![FortranExpr::Lit(
            FortranLit::Char("Hello, World!".to_string()),
        )])];
        module.contains.push(func);
        let code = backend.emit_module(&module);
        assert!(code.contains("MODULE TEST_MOD"));
        assert!(code.contains("USE ISO_FORTRAN_ENV"));
        assert!(code.contains("IMPLICIT NONE"));
        assert!(code.contains("SUBROUTINE HELLO_WORLD()"));
        assert!(code.contains("PRINT *, 'Hello, World!'"));
        assert!(code.contains("END MODULE TEST_MOD"));
    }
    #[test]
    pub(super) fn test_mangle_fortran_ident() {
        let mut backend = FortranBackend::new();
        assert_eq!(backend.mangle_name("natAdd"), "natAdd");
        assert_eq!(backend.mangle_name("2fast"), "ox_2fast");
        let m = backend.mangle_name("do");
        assert!(m.starts_with("ox_") || m != "do");
        assert_eq!(backend.mangle_name("foo.bar"), "foo_bar");
    }
}
#[cfg(test)]
mod Fort_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = FortPassConfig::new("test_pass", FortPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = FortPassStats::new();
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
        let mut reg = FortPassRegistry::new();
        reg.register(FortPassConfig::new("pass_a", FortPassPhase::Analysis));
        reg.register(FortPassConfig::new("pass_b", FortPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = FortAnalysisCache::new(10);
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
        let mut wl = FortWorklist::new();
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
        let mut dt = FortDominatorTree::new(5);
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
        let mut liveness = FortLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(FortConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(FortConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(FortConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            FortConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(FortConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = FortDepGraph::new();
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
mod fortranext_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_fortranext_phase_order() {
        assert_eq!(FortranExtPassPhase::Early.order(), 0);
        assert_eq!(FortranExtPassPhase::Middle.order(), 1);
        assert_eq!(FortranExtPassPhase::Late.order(), 2);
        assert_eq!(FortranExtPassPhase::Finalize.order(), 3);
        assert!(FortranExtPassPhase::Early.is_early());
        assert!(!FortranExtPassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_fortranext_config_builder() {
        let c = FortranExtPassConfig::new("p")
            .with_phase(FortranExtPassPhase::Late)
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
    pub(super) fn test_fortranext_stats() {
        let mut s = FortranExtPassStats::new();
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
    pub(super) fn test_fortranext_registry() {
        let mut r = FortranExtPassRegistry::new();
        r.register(FortranExtPassConfig::new("a").with_phase(FortranExtPassPhase::Early));
        r.register(FortranExtPassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&FortranExtPassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_fortranext_cache() {
        let mut c = FortranExtCache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_fortranext_worklist() {
        let mut w = FortranExtWorklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_fortranext_dom_tree() {
        let mut dt = FortranExtDomTree::new(5);
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
    pub(super) fn test_fortranext_liveness() {
        let mut lv = FortranExtLiveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_fortranext_const_folder() {
        let mut cf = FortranExtConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_fortranext_dep_graph() {
        let mut g = FortranExtDepGraph::new(4);
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
