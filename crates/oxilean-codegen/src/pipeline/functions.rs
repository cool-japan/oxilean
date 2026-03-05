//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::c_backend;
use crate::lcnf::*;
use crate::native_backend;
use crate::opt_join::{self, JoinPointConfig};
use crate::opt_reuse::{self, ReuseConfig};
use crate::opt_specialize::{self, SpecializationConfig};
use crate::CodegenTarget;
use oxilean_kernel::expr::Expr;
use oxilean_kernel::Name;

use super::types::{
    CompilerPipeline, OptLevel, PassId, PassStats, PipeAnalysisCache, PipeConstantFoldingHelper,
    PipeDepGraph, PipeDominatorTree, PipeExtCache, PipeExtConstFolder, PipeExtDepGraph,
    PipeExtDomTree, PipeExtLiveness, PipeExtPassConfig, PipeExtPassPhase, PipeExtPassRegistry,
    PipeExtPassStats, PipeExtWorklist, PipeLivenessInfo, PipePassConfig, PipePassPhase,
    PipePassRegistry, PipePassStats, PipeWorklist, PipeX2Cache, PipeX2ConstFolder, PipeX2DepGraph,
    PipeX2DomTree, PipeX2Liveness, PipeX2PassConfig, PipeX2PassPhase, PipeX2PassRegistry,
    PipeX2PassStats, PipeX2Worklist, PipelineBuilder, PipelineChangeSummary, PipelineConfig,
    PipelineResult, PipelineStats,
};

/// Type alias for the raw kernel input to `exprs_to_lcnf`: (name, params, body).
pub type LcnfDeclInput = (Name, Vec<(Name, Expr)>, Expr);
/// Peel leading `Lam` binders from a kernel expression, collecting the
/// parameter names and types.
///
/// Returns `(params, body)` where `params` is a list of `(Name, Expr)` pairs
/// (one per peeled lambda) and `body` is the residual expression after all
/// leading lambdas have been removed.
pub(super) fn peel_lam_params(expr: &Expr) -> (Vec<(Name, Expr)>, Expr) {
    let mut params = Vec::new();
    let mut cur = expr.clone();
    loop {
        match cur {
            Expr::Lam(_, name, ty, body) => {
                params.push((name, *ty));
                cur = *body;
            }
            other => {
                return (params, other);
            }
        }
    }
}
/// Run the join point optimization pass.
///
/// Identifies expressions shared across case branches and hoists them into
/// join-point let-bindings, reducing code duplication and enabling further
/// optimisations such as tail-call elimination.
pub(super) fn run_join_point_pass(module: &LcnfModule) -> LcnfModule {
    let config = JoinPointConfig::default();
    let mut result = module.clone();
    opt_join::optimize_join_points(&mut result, &config);
    result
}
/// Run the specialization pass.
///
/// Creates monomorphised copies of polymorphic functions for the concrete
/// argument types observed at call sites (e.g. `Nat → u64`), allowing the
/// back-end to emit more efficient code without runtime dispatch overhead.
pub(super) fn run_specialize_pass(module: &LcnfModule) -> LcnfModule {
    let config = SpecializationConfig::default();
    let mut result = module.clone();
    opt_specialize::specialize_module(&mut result, &config);
    result
}
/// Run the reuse optimization pass.
///
/// Performs reset–reuse analysis (inspired by "Counting Immutable Beans"):
/// when a heap-allocated value is the last use of a uniquely-owned cell the
/// cell's memory can be recycled for the result, avoiding a fresh allocation.
/// Also infers borrow annotations to reduce reference-counting traffic.
pub(super) fn run_reuse_pass(module: &LcnfModule) -> LcnfModule {
    let config = ReuseConfig::default();
    let mut result = module.clone();
    opt_reuse::optimize_reuse(&mut result, &config);
    result
}
/// Count the total number of let-bindings in a module.
pub(super) fn count_module_lets(module: &LcnfModule) -> usize {
    module
        .fun_decls
        .iter()
        .map(|d| count_expr_lets(&d.body))
        .sum()
}
/// Count let-bindings in an expression.
pub(super) fn count_expr_lets(expr: &LcnfExpr) -> usize {
    match expr {
        LcnfExpr::Let { body, .. } => 1 + count_expr_lets(body),
        LcnfExpr::Case { alts, default, .. } => {
            let alt_count: usize = alts.iter().map(|a| count_expr_lets(&a.body)).sum();
            let def_count = default.as_ref().map(|d| count_expr_lets(d)).unwrap_or(0);
            alt_count + def_count
        }
        _ => 0,
    }
}
/// Compile an LCNF module with the given optimization level and target.
pub fn compile_module(
    module: &LcnfModule,
    opt_level: OptLevel,
    target: CodegenTarget,
) -> PipelineResult {
    let config = PipelineConfig {
        opt_level,
        target,
        ..Default::default()
    };
    let pipeline = CompilerPipeline::new(config.clone());
    let mut stats = PipelineStats {
        input_decls: module.fun_decls.len(),
        ..Default::default()
    };
    let passes = config.effective_passes();
    let max_iter = config.effective_max_iterations();
    let mut optimized = module.clone();
    if !passes.is_empty() {
        optimized = pipeline.iterate_to_fixpoint(optimized, &passes, max_iter, &mut stats);
    }
    stats.output_decls = optimized.fun_decls.len();
    let mut result = PipelineResult {
        c_output: None,
        native_output: None,
        lcnf_module: optimized.clone(),
        stats,
    };
    match target {
        CodegenTarget::C => {
            let c_output = c_backend::compile_to_c_default(&optimized);
            result.c_output = Some(c_output);
        }
        CodegenTarget::LlvmIr | CodegenTarget::Rust => {
            let native_module = native_backend::compile_to_native(&optimized);
            result.native_output = Some(native_module);
        }
        CodegenTarget::Interpreter => {}
    }
    result
}
/// Quick compilation: O0, C target, no optimization.
pub fn compile_module_o0(module: &LcnfModule) -> PipelineResult {
    compile_module(module, OptLevel::O0, CodegenTarget::C)
}
/// Standard compilation: O2, C target.
pub fn compile_module_o2(module: &LcnfModule) -> PipelineResult {
    compile_module(module, OptLevel::O2, CodegenTarget::C)
}
#[cfg(test)]
mod tests {
    use super::*;
    pub(super) fn vid(n: u64) -> LcnfVarId {
        LcnfVarId(n)
    }
    pub(super) fn mk_param(n: u64, name: &str) -> LcnfParam {
        LcnfParam {
            id: vid(n),
            name: name.to_string(),
            ty: LcnfType::Nat,
            erased: false,
            borrowed: false,
        }
    }
    pub(super) fn mk_fun_decl(name: &str, body: LcnfExpr) -> LcnfFunDecl {
        LcnfFunDecl {
            name: name.to_string(),
            original_name: None,
            params: vec![mk_param(0, "x")],
            ret_type: LcnfType::Nat,
            body,
            is_recursive: false,
            is_lifted: false,
            inline_cost: 1,
        }
    }
    pub(super) fn mk_let(id: u64, value: LcnfLetValue, body: LcnfExpr) -> LcnfExpr {
        LcnfExpr::Let {
            id: vid(id),
            name: format!("x{}", id),
            ty: LcnfType::Nat,
            value,
            body: Box::new(body),
        }
    }
    pub(super) fn mk_module(decls: Vec<LcnfFunDecl>) -> LcnfModule {
        LcnfModule {
            fun_decls: decls,
            extern_decls: vec![],
            name: "test_mod".to_string(),
            metadata: LcnfModuleMetadata::default(),
        }
    }
    #[test]
    pub(super) fn test_opt_level_display() {
        assert_eq!(OptLevel::O0.to_string(), "O0");
        assert_eq!(OptLevel::O1.to_string(), "O1");
        assert_eq!(OptLevel::O2.to_string(), "O2");
        assert_eq!(OptLevel::O3.to_string(), "O3");
    }
    #[test]
    pub(super) fn test_opt_level_to_u8() {
        assert_eq!(OptLevel::O0.to_u8(), 0);
        assert_eq!(OptLevel::O1.to_u8(), 1);
        assert_eq!(OptLevel::O2.to_u8(), 2);
        assert_eq!(OptLevel::O3.to_u8(), 3);
    }
    #[test]
    pub(super) fn test_opt_level_default_passes() {
        assert!(OptLevel::O0.default_passes().is_empty());
        assert!(!OptLevel::O1.default_passes().is_empty());
        assert!(OptLevel::O2.default_passes().len() > OptLevel::O1.default_passes().len());
        assert!(OptLevel::O3.default_passes().len() >= OptLevel::O2.default_passes().len());
    }
    #[test]
    pub(super) fn test_pass_id_display() {
        assert_eq!(PassId::Dce.to_string(), "dce");
        assert_eq!(PassId::JoinPoints.to_string(), "join-points");
        assert_eq!(PassId::Specialize.to_string(), "specialize");
        assert_eq!(PassId::Reuse.to_string(), "reuse");
        assert_eq!(PassId::ClosureConvert.to_string(), "closure-convert");
        assert_eq!(
            PassId::Custom("my-pass".to_string()).to_string(),
            "custom:my-pass"
        );
    }
    #[test]
    pub(super) fn test_pipeline_config_default() {
        let cfg = PipelineConfig::default();
        assert_eq!(cfg.opt_level, OptLevel::O1);
        assert_eq!(cfg.target, CodegenTarget::C);
        assert!(!cfg.debug);
        assert!(!cfg.emit_ir);
    }
    #[test]
    pub(super) fn test_pipeline_config_effective_passes() {
        let mut cfg = PipelineConfig::default();
        assert_eq!(cfg.effective_passes(), OptLevel::O1.default_passes());
        cfg.passes = vec![PassId::Dce, PassId::JoinPoints];
        assert_eq!(cfg.effective_passes().len(), 2);
    }
    #[test]
    pub(super) fn test_full_pipeline_o0() {
        let module = mk_module(vec![mk_fun_decl(
            "test",
            LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(42))),
        )]);
        let result = compile_module_o0(&module);
        assert!(result.c_output.is_some());
        assert!(result.native_output.is_none());
    }
    #[test]
    pub(super) fn test_pipeline_with_single_pass() {
        let body = mk_let(
            1,
            LcnfLetValue::Lit(LcnfLit::Nat(42)),
            mk_let(
                2,
                LcnfLetValue::Lit(LcnfLit::Nat(99)),
                LcnfExpr::Return(LcnfArg::Var(vid(1))),
            ),
        );
        let _module = mk_module(vec![mk_fun_decl("test", body)]);
        let config = PipelineConfig {
            opt_level: OptLevel::O0,
            target: CodegenTarget::C,
            passes: vec![PassId::Dce],
            max_iterations: 3,
            ..Default::default()
        };
        let pipeline = CompilerPipeline::new(config.clone());
        let result = pipeline.run_pipeline(vec![], &config);
        let _ = result.stats.iterations;
    }
    #[test]
    pub(super) fn test_iterate_to_fixpoint() {
        let body = mk_let(
            1,
            LcnfLetValue::Lit(LcnfLit::Nat(42)),
            mk_let(
                2,
                LcnfLetValue::Lit(LcnfLit::Nat(99)),
                LcnfExpr::Return(LcnfArg::Var(vid(1))),
            ),
        );
        let module = mk_module(vec![mk_fun_decl("test", body)]);
        let pipeline = CompilerPipeline::default_pipeline();
        let mut stats = PipelineStats::default();
        let result = pipeline.iterate_to_fixpoint(module, &[PassId::Dce], 5, &mut stats);
        assert_eq!(result.fun_decls.len(), 1);
        let let_count = count_expr_lets(&result.fun_decls[0].body);
        assert!(let_count <= 1, "expected at most 1 let, got {}", let_count);
    }
    #[test]
    pub(super) fn test_run_pass_dce() {
        let body = mk_let(
            1,
            LcnfLetValue::Lit(LcnfLit::Nat(42)),
            LcnfExpr::Return(LcnfArg::Var(vid(0))),
        );
        let module = mk_module(vec![mk_fun_decl("test", body)]);
        let pipeline = CompilerPipeline::default_pipeline();
        let result = pipeline.run_pass(&module, &PassId::Dce);
        assert!(result.changed);
    }
    #[test]
    pub(super) fn test_run_pass_custom() {
        let module = mk_module(vec![mk_fun_decl(
            "test",
            LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0))),
        )]);
        let pipeline = CompilerPipeline::default_pipeline();
        let result = pipeline.run_pass(&module, &PassId::Custom("noop".to_string()));
        assert!(!result.changed);
    }
    #[test]
    pub(super) fn test_compile_module_c() {
        let module = mk_module(vec![mk_fun_decl(
            "main",
            LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0))),
        )]);
        let result = compile_module(&module, OptLevel::O1, CodegenTarget::C);
        assert!(result.c_output.is_some());
        assert!(result.native_output.is_none());
    }
    #[test]
    pub(super) fn test_compile_module_native() {
        let module = mk_module(vec![mk_fun_decl(
            "main",
            LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0))),
        )]);
        let result = compile_module(&module, OptLevel::O0, CodegenTarget::LlvmIr);
        assert!(result.c_output.is_none());
        assert!(result.native_output.is_some());
    }
    #[test]
    pub(super) fn test_pipeline_stats_display() {
        let stats = PipelineStats {
            total_time_us: 1234,
            iterations: 3,
            input_decls: 5,
            output_decls: 4,
            per_pass: vec![(
                PassId::Dce,
                PassStats {
                    decls_processed: 5,
                    transformations: 2,
                    time_us: 500,
                },
            )],
        };
        let s = stats.to_string();
        assert!(s.contains("total_time=1234us"));
        assert!(s.contains("iterations=3"));
        assert!(s.contains("dce"));
    }
    #[test]
    pub(super) fn test_pass_stats_display() {
        let stats = PassStats {
            decls_processed: 10,
            transformations: 3,
            time_us: 100,
        };
        let s = stats.to_string();
        assert!(s.contains("decls=10"));
        assert!(s.contains("transforms=3"));
    }
    #[test]
    pub(super) fn test_count_module_lets() {
        let body = mk_let(
            1,
            LcnfLetValue::Lit(LcnfLit::Nat(1)),
            mk_let(
                2,
                LcnfLetValue::Lit(LcnfLit::Nat(2)),
                LcnfExpr::Return(LcnfArg::Var(vid(2))),
            ),
        );
        let module = mk_module(vec![mk_fun_decl("test", body)]);
        assert_eq!(count_module_lets(&module), 2);
    }
    #[test]
    pub(super) fn test_empty_module_pipeline() {
        let module = mk_module(vec![]);
        let result = compile_module_o0(&module);
        assert_eq!(result.lcnf_module.fun_decls.len(), 0);
    }
    #[test]
    pub(super) fn test_compile_module_o2() {
        let body = mk_let(
            1,
            LcnfLetValue::Lit(LcnfLit::Nat(42)),
            mk_let(
                2,
                LcnfLetValue::FVar(vid(1)),
                mk_let(
                    3,
                    LcnfLetValue::Lit(LcnfLit::Nat(99)),
                    LcnfExpr::Return(LcnfArg::Var(vid(2))),
                ),
            ),
        );
        let module = mk_module(vec![mk_fun_decl("opt_test", body)]);
        let result = compile_module_o2(&module);
        assert!(result.c_output.is_some());
    }
}
#[cfg(test)]
mod extra_pipeline_tests {
    use super::*;
    pub(super) fn mk_simple_module() -> LcnfModule {
        LcnfModule {
            fun_decls: vec![LcnfFunDecl {
                name: "test".to_string(),
                original_name: None,
                params: vec![],
                ret_type: LcnfType::Nat,
                body: LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0))),
                is_recursive: false,
                is_lifted: false,
                inline_cost: 1,
            }],
            extern_decls: vec![],
            name: "mod".to_string(),
            metadata: LcnfModuleMetadata::default(),
        }
    }
    #[test]
    pub(super) fn test_pipeline_builder_defaults() {
        let cfg = PipelineBuilder::new().build();
        assert_eq!(cfg.opt_level, OptLevel::O1);
        assert!(!cfg.debug);
    }
    #[test]
    pub(super) fn test_pipeline_builder_opt_level() {
        let cfg = PipelineBuilder::new().opt_level(OptLevel::O3).build();
        assert_eq!(cfg.opt_level, OptLevel::O3);
    }
    #[test]
    pub(super) fn test_pipeline_builder_target() {
        let cfg = PipelineBuilder::new().target(CodegenTarget::LlvmIr).build();
        assert_eq!(cfg.target, CodegenTarget::LlvmIr);
    }
    #[test]
    pub(super) fn test_pipeline_builder_debug() {
        let cfg = PipelineBuilder::new().debug().build();
        assert!(cfg.debug);
    }
    #[test]
    pub(super) fn test_pipeline_builder_emit_ir() {
        let cfg = PipelineBuilder::new().emit_ir().build();
        assert!(cfg.emit_ir);
    }
    #[test]
    pub(super) fn test_pipeline_builder_with_passes() {
        let cfg = PipelineBuilder::new()
            .with_passes(vec![PassId::Dce, PassId::JoinPoints])
            .build();
        assert_eq!(cfg.passes.len(), 2);
    }
    #[test]
    pub(super) fn test_pipeline_builder_max_iterations() {
        let cfg = PipelineBuilder::new().max_iterations(7).build();
        assert_eq!(cfg.max_iterations, 7);
    }
    #[test]
    pub(super) fn test_change_summary_mark_active() {
        let mut s = PipelineChangeSummary::new();
        s.mark_active("dce");
        assert!(s.any_changed());
    }
    #[test]
    pub(super) fn test_change_summary_mark_converged() {
        let mut s = PipelineChangeSummary::new();
        s.mark_converged("join-points");
        assert!(!s.any_changed());
        assert_eq!(s.converged_passes.len(), 1);
    }
    #[test]
    pub(super) fn test_change_summary_display() {
        let mut s = PipelineChangeSummary::new();
        s.mark_active("dce");
        let text = format!("{}", s);
        assert!(text.contains("dce"));
    }
    #[test]
    pub(super) fn test_run_pipeline_with_builder() {
        let cfg = PipelineBuilder::new()
            .opt_level(OptLevel::O1)
            .target(CodegenTarget::C)
            .build();
        let pipeline = CompilerPipeline::new(cfg.clone());
        let result = pipeline.run_pipeline(vec![], &cfg);
        assert!(result.c_output.is_some());
    }
    #[test]
    pub(super) fn test_opt_level_max_iterations_ordering() {
        assert!(OptLevel::O0.max_iterations() <= OptLevel::O1.max_iterations());
        assert!(OptLevel::O1.max_iterations() <= OptLevel::O2.max_iterations());
        assert!(OptLevel::O2.max_iterations() <= OptLevel::O3.max_iterations());
    }
}
#[cfg(test)]
mod Pipe_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = PipePassConfig::new("test_pass", PipePassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = PipePassStats::new();
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
        let mut reg = PipePassRegistry::new();
        reg.register(PipePassConfig::new("pass_a", PipePassPhase::Analysis));
        reg.register(PipePassConfig::new("pass_b", PipePassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = PipeAnalysisCache::new(10);
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
        let mut wl = PipeWorklist::new();
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
        let mut dt = PipeDominatorTree::new(5);
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
        let mut liveness = PipeLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(PipeConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(PipeConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(PipeConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            PipeConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(PipeConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = PipeDepGraph::new();
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
mod pipeext_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_pipeext_phase_order() {
        assert_eq!(PipeExtPassPhase::Early.order(), 0);
        assert_eq!(PipeExtPassPhase::Middle.order(), 1);
        assert_eq!(PipeExtPassPhase::Late.order(), 2);
        assert_eq!(PipeExtPassPhase::Finalize.order(), 3);
        assert!(PipeExtPassPhase::Early.is_early());
        assert!(!PipeExtPassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_pipeext_config_builder() {
        let c = PipeExtPassConfig::new("p")
            .with_phase(PipeExtPassPhase::Late)
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
    pub(super) fn test_pipeext_stats() {
        let mut s = PipeExtPassStats::new();
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
    pub(super) fn test_pipeext_registry() {
        let mut r = PipeExtPassRegistry::new();
        r.register(PipeExtPassConfig::new("a").with_phase(PipeExtPassPhase::Early));
        r.register(PipeExtPassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&PipeExtPassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_pipeext_cache() {
        let mut c = PipeExtCache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_pipeext_worklist() {
        let mut w = PipeExtWorklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_pipeext_dom_tree() {
        let mut dt = PipeExtDomTree::new(5);
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
    pub(super) fn test_pipeext_liveness() {
        let mut lv = PipeExtLiveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_pipeext_const_folder() {
        let mut cf = PipeExtConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_pipeext_dep_graph() {
        let mut g = PipeExtDepGraph::new(4);
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
mod pipex2_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_pipex2_phase_order() {
        assert_eq!(PipeX2PassPhase::Early.order(), 0);
        assert_eq!(PipeX2PassPhase::Middle.order(), 1);
        assert_eq!(PipeX2PassPhase::Late.order(), 2);
        assert_eq!(PipeX2PassPhase::Finalize.order(), 3);
        assert!(PipeX2PassPhase::Early.is_early());
        assert!(!PipeX2PassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_pipex2_config_builder() {
        let c = PipeX2PassConfig::new("p")
            .with_phase(PipeX2PassPhase::Late)
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
    pub(super) fn test_pipex2_stats() {
        let mut s = PipeX2PassStats::new();
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
    pub(super) fn test_pipex2_registry() {
        let mut r = PipeX2PassRegistry::new();
        r.register(PipeX2PassConfig::new("a").with_phase(PipeX2PassPhase::Early));
        r.register(PipeX2PassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&PipeX2PassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_pipex2_cache() {
        let mut c = PipeX2Cache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_pipex2_worklist() {
        let mut w = PipeX2Worklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_pipex2_dom_tree() {
        let mut dt = PipeX2DomTree::new(5);
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
    pub(super) fn test_pipex2_liveness() {
        let mut lv = PipeX2Liveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_pipex2_const_folder() {
        let mut cf = PipeX2ConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_pipex2_dep_graph() {
        let mut g = PipeX2DepGraph::new(4);
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
