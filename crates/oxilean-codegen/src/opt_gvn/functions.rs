//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::HashMap;

use super::types::{
    AlgebraicSimplifier, AnticipationSet, CCPState, CongruenceClosure, DomTree, ExprCanonicaliser,
    FixpointGVN, GVNAnalysisCache, GVNConfig, GVNConstantFoldingHelper, GVNDepGraph,
    GVNDominatorTree, GVNFact, GVNFunctionSummary, GVNLivenessInfo, GVNPass, GVNPassConfig,
    GVNPassPhase, GVNPassRegistry, GVNPassStats, GVNPipeline, GVNPrePass, GVNReport, GVNStatistics,
    GVNWorklist, HashConsingTable, InterproceduralGVN, LeaderFinder, LoadEliminatorGVN, NormArg,
    NormExpr, PhiNode, PhiNodeSet, PhiOperand, PredicateGVN, RedundancyCollector,
    ScopedValueContext, ValueTable,
};

/// A unique identifier for a value equivalence class.
pub type ValueNumber = u32;
pub(super) fn norm_expr_from_value_conservative(value: &LcnfLetValue) -> NormExpr {
    match value {
        LcnfLetValue::Lit(LcnfLit::Nat(n)) => NormExpr::Lit(*n),
        LcnfLetValue::Lit(LcnfLit::Str(s)) => NormExpr::LitStr(s.clone()),
        LcnfLetValue::Erased => NormExpr::Erased,
        LcnfLetValue::FVar(v) => NormExpr::FVar(v.0 as ValueNumber),
        _ => NormExpr::Unknown,
    }
}
pub(super) fn gvn_norm_value(value: &LcnfLetValue, fact: &GVNFact) -> NormExpr {
    match value {
        LcnfLetValue::Lit(LcnfLit::Nat(n)) => NormExpr::Lit(*n),
        LcnfLetValue::Lit(LcnfLit::Str(s)) => NormExpr::LitStr(s.clone()),
        LcnfLetValue::Erased => NormExpr::Erased,
        LcnfLetValue::FVar(v) => {
            let vn = fact.get(v).unwrap_or(v.0 as ValueNumber + 1_000_000);
            NormExpr::FVar(vn)
        }
        LcnfLetValue::Proj(name, idx, v) => {
            let vn = fact.get(v).unwrap_or(v.0 as ValueNumber + 1_000_000);
            NormExpr::Proj(name.clone(), *idx, vn)
        }
        _ => NormExpr::Unknown,
    }
}
pub(super) fn var(n: u64) -> LcnfVarId {
    LcnfVarId(n)
}
pub(super) fn lit_val(n: u64) -> LcnfLetValue {
    LcnfLetValue::Lit(LcnfLit::Nat(n))
}
pub(super) fn fvar_val(n: u64) -> LcnfLetValue {
    LcnfLetValue::FVar(LcnfVarId(n))
}
pub(super) fn arg_lit(n: u64) -> LcnfArg {
    LcnfArg::Lit(LcnfLit::Nat(n))
}
pub(super) fn make_decl(name: &str, body: LcnfExpr) -> LcnfFunDecl {
    LcnfFunDecl {
        name: name.to_string(),
        params: vec![],
        ret_type: LcnfType::Nat,
        original_name: None,
        is_recursive: false,
        is_lifted: false,
        inline_cost: 0,
        body,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    pub(super) fn let_expr(id: u64, val: LcnfLetValue, body: LcnfExpr) -> LcnfExpr {
        LcnfExpr::Let {
            id: var(id),
            name: format!("x{}", var(id).0),
            ty: LcnfType::Nat,
            value: val,
            body: Box::new(body),
        }
    }
    #[test]
    pub(super) fn test_gvn_default_config() {
        let cfg = GVNConfig::default();
        assert!(cfg.do_phi_translation);
        assert_eq!(cfg.max_depth, 100);
    }
    #[test]
    pub(super) fn test_value_table_empty() {
        let t = ValueTable::new();
        assert!(t.is_empty());
        assert_eq!(t.len(), 0);
    }
    #[test]
    pub(super) fn test_value_table_insert_lookup() {
        let mut t = ValueTable::new();
        let key = NormExpr::Lit(42);
        let vn = t.insert(key.clone(), lit_val(42), var(1));
        assert_eq!(t.lookup(&key), Some(vn));
        assert_eq!(t.len(), 1);
    }
    #[test]
    pub(super) fn test_value_table_canonical_var() {
        let mut t = ValueTable::new();
        let key = NormExpr::Lit(7);
        let vn = t.insert(key, lit_val(7), var(5));
        assert_eq!(t.canonical_var(vn), Some(var(5)));
    }
    #[test]
    pub(super) fn test_gvn_fact_insert_get() {
        let mut f = GVNFact::new();
        f.insert(var(1), 42);
        assert_eq!(f.get(&var(1)), Some(42));
        assert_eq!(f.get(&var(99)), None);
    }
    #[test]
    pub(super) fn test_hash_consing_intern() {
        let mut hct = HashConsingTable::new();
        let key = NormExpr::Lit(5);
        hct.intern(key.clone(), lit_val(5));
        assert_eq!(hct.len(), 1);
        hct.intern(key, lit_val(5));
        assert_eq!(hct.len(), 1);
    }
    #[test]
    pub(super) fn test_congruence_closure_union_find() {
        let mut cc = CongruenceClosure::new();
        cc.union(1, 2);
        assert!(cc.are_equal(1, 2));
        assert!(!cc.are_equal(1, 3));
    }
    #[test]
    pub(super) fn test_congruence_closure_transitivity() {
        let mut cc = CongruenceClosure::new();
        cc.union(1, 2);
        cc.union(2, 3);
        assert!(cc.are_equal(1, 3));
    }
    #[test]
    pub(super) fn test_gvn_report_default() {
        let r = GVNReport::default();
        assert_eq!(r.expressions_numbered, 0);
        assert_eq!(r.redundancies_eliminated, 0);
    }
    #[test]
    pub(super) fn test_gvn_report_merge() {
        let mut r1 = GVNReport {
            expressions_numbered: 3,
            redundancies_eliminated: 1,
            phi_translations: 2,
        };
        let r2 = GVNReport {
            expressions_numbered: 2,
            redundancies_eliminated: 4,
            phi_translations: 1,
        };
        r1.merge(&r2);
        assert_eq!(r1.expressions_numbered, 5);
        assert_eq!(r1.redundancies_eliminated, 5);
        assert_eq!(r1.phi_translations, 3);
    }
    #[test]
    pub(super) fn test_gvn_run_no_redundancy() {
        let body = let_expr(
            1,
            lit_val(1),
            let_expr(2, lit_val(2), LcnfExpr::Return(arg_lit(0))),
        );
        let mut decls = vec![make_decl("f", body)];
        let mut pass = GVNPass::default();
        pass.run(&mut decls);
        assert_eq!(pass.report().redundancies_eliminated, 0);
    }
    #[test]
    pub(super) fn test_gvn_run_redundant_literal() {
        let body = let_expr(
            1,
            lit_val(42),
            let_expr(2, lit_val(42), LcnfExpr::Return(arg_lit(0))),
        );
        let mut decls = vec![make_decl("f", body)];
        let mut pass = GVNPass::default();
        pass.run(&mut decls);
        assert!(pass.report().redundancies_eliminated >= 1);
    }
    #[test]
    pub(super) fn test_gvn_assigns_value_numbers() {
        let body = let_expr(1, lit_val(10), LcnfExpr::Return(arg_lit(10)));
        let mut decls = vec![make_decl("f", body)];
        let mut pass = GVNPass::default();
        pass.run(&mut decls);
        assert!(pass.report().expressions_numbered >= 1);
    }
    #[test]
    pub(super) fn test_gvn_copy_binding_rewritten() {
        let mut decls = vec![make_decl(
            "f",
            let_expr(
                10,
                lit_val(7),
                let_expr(
                    11,
                    lit_val(7),
                    LcnfExpr::Return(LcnfArg::Var(LcnfVarId(11))),
                ),
            ),
        )];
        let mut pass = GVNPass::default();
        pass.run(&mut decls);
        if let LcnfExpr::Let { body, .. } = &decls[0].body {
            if let LcnfExpr::Let { value, .. } = body.as_ref() {
                assert!(matches!(value, LcnfLetValue::FVar(v) if * v == LcnfVarId(10)));
            }
        }
    }
    #[test]
    pub(super) fn test_gvn_run_multiple_decls() {
        let mut decls = vec![
            make_decl("a", let_expr(1, lit_val(1), LcnfExpr::Return(arg_lit(0)))),
            make_decl("b", let_expr(2, lit_val(2), LcnfExpr::Return(arg_lit(0)))),
        ];
        let mut pass = GVNPass::default();
        pass.run(&mut decls);
        assert!(pass.report().expressions_numbered >= 2);
    }
    #[test]
    pub(super) fn test_gvn_depth_limit() {
        let mut cfg = GVNConfig::default();
        cfg.max_depth = 1;
        let body = let_expr(
            1,
            lit_val(5),
            let_expr(2, lit_val(5), LcnfExpr::Return(arg_lit(0))),
        );
        let mut decls = vec![make_decl("f", body)];
        let mut pass = GVNPass::new(cfg);
        pass.run(&mut decls);
    }
    #[test]
    pub(super) fn test_norm_expr_equality() {
        let a = NormExpr::Lit(42);
        let b = NormExpr::Lit(42);
        assert_eq!(a, b);
        let c = NormExpr::Lit(43);
        assert_ne!(a, c);
    }
    #[test]
    pub(super) fn test_norm_arg_equality() {
        assert_eq!(NormArg::LitNat(5), NormArg::LitNat(5));
        assert_ne!(NormArg::LitNat(5), NormArg::LitNat(6));
    }
    #[test]
    pub(super) fn test_dom_tree_build() {
        let body = let_expr(1, lit_val(5), LcnfExpr::Return(arg_lit(0)));
        let decl = make_decl("f", body);
        let dt = DomTree::build_from_decl(&decl);
        assert_eq!(dt.num_nodes(), 1);
        assert!(dt.roots.contains(&var(1)));
    }
    #[test]
    pub(super) fn test_dom_tree_dominates_self() {
        let body = let_expr(1, lit_val(5), LcnfExpr::Return(arg_lit(0)));
        let decl = make_decl("f", body);
        let dt = DomTree::build_from_decl(&decl);
        assert!(dt.dominates(var(1), var(1)));
    }
    #[test]
    pub(super) fn test_dom_tree_nested() {
        let body = let_expr(
            1,
            lit_val(1),
            let_expr(2, lit_val(2), LcnfExpr::Return(arg_lit(0))),
        );
        let decl = make_decl("f", body);
        let dt = DomTree::build_from_decl(&decl);
        assert!(dt.dominates(var(1), var(2)));
    }
    #[test]
    pub(super) fn test_anticipation_set_basic() {
        let mut a = AnticipationSet::new();
        a.add(NormExpr::Lit(5));
        assert!(a.contains(&NormExpr::Lit(5)));
        assert!(!a.contains(&NormExpr::Lit(6)));
    }
    #[test]
    pub(super) fn test_anticipation_set_meet() {
        let mut a = AnticipationSet::new();
        a.add(NormExpr::Lit(1));
        a.add(NormExpr::Lit(2));
        let mut b = AnticipationSet::new();
        b.add(NormExpr::Lit(2));
        b.add(NormExpr::Lit(3));
        let meet = a.meet(&b);
        assert!(meet.contains(&NormExpr::Lit(2)));
        assert!(!meet.contains(&NormExpr::Lit(1)));
        assert!(!meet.contains(&NormExpr::Lit(3)));
    }
    #[test]
    pub(super) fn test_gvn_pre_compute_anticipation() {
        let body = let_expr(1, lit_val(7), LcnfExpr::Return(arg_lit(0)));
        let decl = make_decl("f", body);
        let mut pre = GVNPrePass::new();
        pre.compute_anticipation(&decl);
        assert!(pre.anticipation.contains_key(&var(1)));
    }
    #[test]
    pub(super) fn test_phi_node_trivial() {
        let phi = PhiNode::new(
            var(1),
            vec![
                PhiOperand {
                    branch_idx: 0,
                    vn: 5,
                },
                PhiOperand {
                    branch_idx: 1,
                    vn: 5,
                },
            ],
            42,
        );
        assert!(phi.is_trivial());
        assert_eq!(phi.trivial_vn(), Some(5));
    }
    #[test]
    pub(super) fn test_phi_node_non_trivial() {
        let phi = PhiNode::new(
            var(1),
            vec![
                PhiOperand {
                    branch_idx: 0,
                    vn: 3,
                },
                PhiOperand {
                    branch_idx: 1,
                    vn: 7,
                },
            ],
            42,
        );
        assert!(!phi.is_trivial());
        assert_eq!(phi.trivial_vn(), None);
    }
    #[test]
    pub(super) fn test_phi_node_set_add_remove_trivial() {
        let mut pns = PhiNodeSet::new(100);
        pns.add_phi(
            var(1),
            vec![
                PhiOperand {
                    branch_idx: 0,
                    vn: 5,
                },
                PhiOperand {
                    branch_idx: 1,
                    vn: 5,
                },
            ],
        );
        pns.add_phi(
            var(2),
            vec![
                PhiOperand {
                    branch_idx: 0,
                    vn: 3,
                },
                PhiOperand {
                    branch_idx: 1,
                    vn: 7,
                },
            ],
        );
        assert_eq!(pns.num_phis(), 2);
        let removed = pns.remove_trivial();
        assert_eq!(removed, 1);
        assert_eq!(pns.num_phis(), 1);
    }
    #[test]
    pub(super) fn test_leader_finder_basic() {
        let mut lf = LeaderFinder::new();
        lf.record(42, var(1));
        lf.record(42, var(2));
        assert_eq!(lf.leader(42), Some(var(1)));
        assert_eq!(lf.members(42).len(), 2);
        assert_eq!(lf.num_redundancies(), 1);
    }
    #[test]
    pub(super) fn test_load_eliminator_basic() {
        let body = LcnfExpr::Let {
            id: var(1),
            name: "c".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Ctor("Pair".to_string(), 0, vec![arg_lit(10), arg_lit(20)]),
            body: Box::new(LcnfExpr::Let {
                id: var(2),
                name: "p1".to_string(),
                ty: LcnfType::Nat,
                value: LcnfLetValue::Proj("Pair".to_string(), 0, var(1)),
                body: Box::new(LcnfExpr::Let {
                    id: var(3),
                    name: "p2".to_string(),
                    ty: LcnfType::Nat,
                    value: LcnfLetValue::Proj("Pair".to_string(), 0, var(1)),
                    body: Box::new(LcnfExpr::Return(LcnfArg::Var(var(3)))),
                }),
            }),
        };
        let mut decl = make_decl("f", body);
        let mut le = LoadEliminatorGVN::new();
        le.run(&mut decl);
        assert_eq!(le.eliminated, 1);
    }
    #[test]
    pub(super) fn test_algebraic_simplifier_default_rules() {
        let s = AlgebraicSimplifier::new();
        assert!(!s.rules.is_empty());
    }
    #[test]
    pub(super) fn test_ccp_state_constant_folding() {
        let body = LcnfExpr::Let {
            id: var(1),
            name: "x".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Lit(LcnfLit::Nat(0)),
            body: Box::new(LcnfExpr::Case {
                scrutinee: var(1),
                scrutinee_ty: LcnfType::Nat,
                alts: vec![LcnfAlt {
                    ctor_name: "zero".to_string(),
                    ctor_tag: 0,
                    params: vec![],
                    body: LcnfExpr::Return(arg_lit(42)),
                }],
                default: None,
            }),
        };
        let mut decl = make_decl("f", body);
        let mut ccp = CCPState::new();
        ccp.run(&mut decl);
        assert!(ccp.folded >= 1);
    }
    #[test]
    pub(super) fn test_predicate_gvn_enter_exit_branch() {
        let mut pgvn = PredicateGVN::new();
        pgvn.enter_branch(var(1), 0);
        assert!(pgvn.knows_eq_lit(var(1), &LcnfLit::Nat(0)));
        pgvn.exit_branch();
        assert!(!pgvn.knows_eq_lit(var(1), &LcnfLit::Nat(0)));
    }
    #[test]
    pub(super) fn test_fixpoint_gvn_converges() {
        let body = let_expr(
            1,
            lit_val(5),
            let_expr(2, lit_val(5), LcnfExpr::Return(arg_lit(0))),
        );
        let mut decl = make_decl("f", body);
        let mut fp = FixpointGVN::new(10);
        fp.run(&mut decl);
        assert!(fp.iterations <= 10);
    }
    #[test]
    pub(super) fn test_gvn_statistics_total_redundancies() {
        let mut stats = GVNStatistics::new();
        stats.lit_redundancies = 3;
        stats.proj_redundancies = 2;
        assert_eq!(stats.total_redundancies(), 5);
    }
    #[test]
    pub(super) fn test_scoped_value_context_push_pop() {
        let mut ctx = ScopedValueContext::new();
        ctx.bind(var(1), 100);
        assert_eq!(ctx.lookup(&var(1)), Some(100));
        ctx.push_scope();
        ctx.bind(var(2), 200);
        assert_eq!(ctx.lookup(&var(2)), Some(200));
        ctx.pop_scope();
        assert_eq!(ctx.lookup(&var(2)), None);
        assert_eq!(ctx.lookup(&var(1)), Some(100));
    }
    #[test]
    pub(super) fn test_scoped_value_context_depth() {
        let mut ctx = ScopedValueContext::new();
        assert_eq!(ctx.scope_depth(), 1);
        ctx.push_scope();
        assert_eq!(ctx.scope_depth(), 2);
        ctx.pop_scope();
        assert_eq!(ctx.scope_depth(), 1);
    }
    #[test]
    pub(super) fn test_expr_canonicaliser_sorts_commutative() {
        let mut ec = ExprCanonicaliser::new();
        let expr = NormExpr::App(NormArg::Vn(0), vec![NormArg::Vn(5), NormArg::Vn(3)]);
        let canon = ec.canonicalise(expr);
        if let NormExpr::App(_, args) = &canon {
            assert_eq!(args[0], NormArg::Vn(3));
            assert_eq!(args[1], NormArg::Vn(5));
        }
        assert_eq!(ec.canonicalisations, 1);
    }
    #[test]
    pub(super) fn test_gvn_function_summary_pure() {
        let mut s = GVNFunctionSummary::new();
        s.mark_pure();
        assert!(s.is_pure_fn);
    }
    #[test]
    pub(super) fn test_interprocedural_gvn_pure_query() {
        let mut igvn = InterproceduralGVN::new();
        let mut s = GVNFunctionSummary::new();
        s.mark_pure();
        igvn.add_summary("pure_fn".to_string(), s);
        assert!(igvn.calls_are_equal("pure_fn"));
        assert!(!igvn.calls_are_equal("impure"));
    }
    #[test]
    pub(super) fn test_redundancy_collector_basic() {
        let body = let_expr(
            1,
            lit_val(42),
            let_expr(2, lit_val(42), LcnfExpr::Return(arg_lit(0))),
        );
        let decl = make_decl("f", body);
        let mut rc = RedundancyCollector::new();
        rc.collect(&decl);
        assert!(rc.num_redundancies() >= 1);
    }
    #[test]
    pub(super) fn test_gvn_pipeline_default() {
        let body = let_expr(
            1,
            lit_val(7),
            let_expr(2, lit_val(7), LcnfExpr::Return(arg_lit(0))),
        );
        let mut decls = vec![make_decl("f", body)];
        let mut pipeline = GVNPipeline::new();
        pipeline.run(&mut decls);
        assert!(pipeline.stats.total_vns >= 1);
    }
    #[test]
    pub(super) fn test_gvn_fact_meet() {
        let mut f1 = GVNFact::new();
        f1.insert(var(1), 10);
        f1.insert(var(2), 20);
        let mut f2 = GVNFact::new();
        f2.insert(var(1), 10);
        f2.insert(var(2), 99);
        let meet = f1.meet(&f2);
        assert_eq!(meet.get(&var(1)), Some(10));
        assert_eq!(meet.get(&var(2)), None);
    }
    #[test]
    pub(super) fn test_value_table_try_merge_compatible() {
        let mut t1 = ValueTable::new();
        let vn1 = t1.insert(NormExpr::Lit(1), lit_val(1), var(1));
        let mut t2 = ValueTable::new();
        t2.insert(NormExpr::Lit(2), lit_val(2), var(2));
        let ok = t1.try_merge(&t2);
        assert!(ok);
        let _ = vn1;
    }
    #[test]
    pub(super) fn test_gvn_pipeline_with_load_elim() {
        let body = LcnfExpr::Let {
            id: var(1),
            name: "c".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Ctor("Pair".to_string(), 0, vec![arg_lit(5)]),
            body: Box::new(LcnfExpr::Let {
                id: var(2),
                name: "p".to_string(),
                ty: LcnfType::Nat,
                value: LcnfLetValue::Proj("Pair".to_string(), 0, var(1)),
                body: Box::new(LcnfExpr::Let {
                    id: var(3),
                    name: "p2".to_string(),
                    ty: LcnfType::Nat,
                    value: LcnfLetValue::Proj("Pair".to_string(), 0, var(1)),
                    body: Box::new(LcnfExpr::Return(LcnfArg::Var(var(3)))),
                }),
            }),
        };
        let mut decls = vec![make_decl("f", body)];
        let mut pipeline = GVNPipeline {
            do_load_elim: true,
            do_base_gvn: false,
            ..GVNPipeline::default()
        };
        pipeline.run(&mut decls);
        assert!(pipeline.stats.proj_redundancies >= 1);
    }
    #[test]
    pub(super) fn test_congruence_closure_idempotent_union() {
        let mut cc = CongruenceClosure::new();
        cc.union(1, 1);
        assert!(cc.are_equal(1, 1));
    }
    #[test]
    pub(super) fn test_fixpoint_gvn_max_iter_respected() {
        let body = LcnfExpr::Return(arg_lit(0));
        let mut decl = make_decl("f", body);
        let mut fp = FixpointGVN::new(3);
        fp.run(&mut decl);
        assert!(fp.iterations <= 3);
    }
    #[test]
    pub(super) fn test_norm_expr_proj_hash() {
        use std::collections::HashSet;
        let mut s: HashSet<NormExpr> = HashSet::new();
        s.insert(NormExpr::Proj("Foo".to_string(), 0, 5));
        s.insert(NormExpr::Proj("Foo".to_string(), 0, 5));
        assert_eq!(s.len(), 1);
        s.insert(NormExpr::Proj("Foo".to_string(), 1, 5));
        assert_eq!(s.len(), 2);
    }
}
#[cfg(test)]
mod GVN_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = GVNPassConfig::new("test_pass", GVNPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = GVNPassStats::new();
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
        let mut reg = GVNPassRegistry::new();
        reg.register(GVNPassConfig::new("pass_a", GVNPassPhase::Analysis));
        reg.register(GVNPassConfig::new("pass_b", GVNPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = GVNAnalysisCache::new(10);
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
        let mut wl = GVNWorklist::new();
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
        let mut dt = GVNDominatorTree::new(5);
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
        let mut liveness = GVNLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(GVNConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(GVNConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(GVNConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            GVNConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(GVNConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = GVNDepGraph::new();
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
