//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::{
    LcnfArg, LcnfExpr, LcnfFunDecl, LcnfLetValue, LcnfLit, LcnfParam, LcnfType, LcnfVarId,
};
use std::collections::HashMap;

use super::types::{
    BindingEnv, BindingTime, PEAnalysisCache, PEConstantFoldingHelper, PEDepGraph, PEDominatorTree,
    PEExtCache, PEExtConstFolder, PEExtDepGraph, PEExtDomTree, PEExtLiveness, PEExtPassConfig,
    PEExtPassPhase, PEExtPassRegistry, PEExtPassStats, PEExtWorklist, PELivenessInfo, PEPassConfig,
    PEPassPhase, PEPassRegistry, PEPassStats, PEWorklist, PartialEvalConfig, PartialEvalReport,
    PartialEvaluator, PartialValue, SpecializationKey,
};

/// Classify each parameter of a function as Static or Dynamic based on a
/// sample call-site environment.
pub fn analyze_binding_times(
    decl: &LcnfFunDecl,
    call_site_env: &[PartialValue],
) -> Vec<BindingTime> {
    decl.params
        .iter()
        .enumerate()
        .map(|(i, _param)| {
            let pv = call_site_env.get(i).unwrap_or(&PartialValue::Unknown);
            BindingTime::from_partial(pv)
        })
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::lcnf::{
        LcnfAlt, LcnfArg, LcnfExpr, LcnfFunDecl, LcnfLetValue, LcnfLit, LcnfParam, LcnfType,
        LcnfVarId,
    };
    pub(super) fn make_nat_lit(id: u64, n: u64, body: LcnfExpr) -> LcnfExpr {
        LcnfExpr::Let {
            id: LcnfVarId(id),
            name: format!("v{}", id),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Lit(LcnfLit::Nat(n)),
            body: Box::new(body),
        }
    }
    pub(super) fn make_return_nat(n: u64) -> LcnfExpr {
        LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(n)))
    }
    pub(super) fn make_decl(name: &str, params: Vec<LcnfParam>, body: LcnfExpr) -> LcnfFunDecl {
        LcnfFunDecl {
            name: name.to_string(),
            original_name: None,
            params,
            ret_type: LcnfType::Nat,
            body,
            is_recursive: false,
            is_lifted: false,
            inline_cost: 0,
        }
    }
    pub(super) fn make_param(id: u64, name: &str) -> LcnfParam {
        LcnfParam {
            id: LcnfVarId(id),
            name: name.to_string(),
            ty: LcnfType::Nat,
            erased: false,
            borrowed: false,
        }
    }
    #[test]
    pub(super) fn test_partial_value_is_known() {
        let v = PartialValue::Known(LcnfLit::Nat(42));
        assert!(v.is_known());
        assert!(!v.is_unknown());
    }
    #[test]
    pub(super) fn test_partial_value_is_unknown() {
        assert!(PartialValue::Unknown.is_unknown());
        assert!(!PartialValue::Unknown.is_known());
    }
    #[test]
    pub(super) fn test_partial_value_as_lit() {
        let v = PartialValue::Known(LcnfLit::Nat(7));
        assert_eq!(v.as_lit(), Some(&LcnfLit::Nat(7)));
        assert_eq!(PartialValue::Unknown.as_lit(), None);
    }
    #[test]
    pub(super) fn test_partial_value_meet_same_known() {
        let a = PartialValue::Known(LcnfLit::Nat(3));
        let b = PartialValue::Known(LcnfLit::Nat(3));
        assert_eq!(
            PartialValue::meet(&a, &b),
            PartialValue::Known(LcnfLit::Nat(3))
        );
    }
    #[test]
    pub(super) fn test_partial_value_meet_different_known_gives_unknown() {
        let a = PartialValue::Known(LcnfLit::Nat(1));
        let b = PartialValue::Known(LcnfLit::Nat(2));
        assert_eq!(PartialValue::meet(&a, &b), PartialValue::Unknown);
    }
    #[test]
    pub(super) fn test_partial_value_meet_contradiction_propagates() {
        let a = PartialValue::Contradiction;
        let b = PartialValue::Known(LcnfLit::Nat(1));
        assert_eq!(PartialValue::meet(&a, &b), PartialValue::Contradiction);
    }
    #[test]
    pub(super) fn test_partial_value_display() {
        assert!(PartialValue::Unknown.display().contains("Unknown"));
        assert!(PartialValue::Contradiction
            .display()
            .contains("Contradiction"));
    }
    #[test]
    pub(super) fn test_binding_env_lookup_unknown_default() {
        let env = BindingEnv::new();
        assert_eq!(env.lookup(LcnfVarId(99)), &PartialValue::Unknown);
    }
    #[test]
    pub(super) fn test_binding_env_bind_and_lookup() {
        let mut env = BindingEnv::new();
        env.bind(LcnfVarId(1), PartialValue::Known(LcnfLit::Nat(5)));
        assert_eq!(
            env.lookup(LcnfVarId(1)),
            &PartialValue::Known(LcnfLit::Nat(5))
        );
    }
    #[test]
    pub(super) fn test_binding_env_len() {
        let mut env = BindingEnv::new();
        assert_eq!(env.len(), 0);
        env.bind(LcnfVarId(0), PartialValue::Unknown);
        assert_eq!(env.len(), 1);
    }
    #[test]
    pub(super) fn test_binding_env_is_empty() {
        let env = BindingEnv::new();
        assert!(env.is_empty());
    }
    #[test]
    pub(super) fn test_binding_env_merge_from() {
        let mut e1 = BindingEnv::new();
        e1.bind(LcnfVarId(0), PartialValue::Known(LcnfLit::Nat(1)));
        let mut e2 = BindingEnv::new();
        e2.bind(LcnfVarId(1), PartialValue::Known(LcnfLit::Nat(2)));
        e1.merge_from(&e2);
        assert!(e1.lookup(LcnfVarId(1)).is_known());
    }
    #[test]
    pub(super) fn test_spec_key_mangled_name_with_nat() {
        let key = SpecializationKey::new("foo", vec![PartialValue::Known(LcnfLit::Nat(42))]);
        assert!(key.mangled_name().contains("42"));
        assert!(key.mangled_name().starts_with("foo"));
    }
    #[test]
    pub(super) fn test_spec_key_mangled_name_unknown_not_added() {
        let key = SpecializationKey::new("bar", vec![PartialValue::Unknown]);
        assert_eq!(key.mangled_name(), "bar");
    }
    #[test]
    pub(super) fn test_spec_key_has_static_args_true() {
        let key = SpecializationKey::new(
            "f",
            vec![PartialValue::Known(LcnfLit::Nat(0)), PartialValue::Unknown],
        );
        assert!(key.has_static_args());
    }
    #[test]
    pub(super) fn test_spec_key_has_static_args_false() {
        let key = SpecializationKey::new("f", vec![PartialValue::Unknown]);
        assert!(!key.has_static_args());
    }
    #[test]
    pub(super) fn test_default_config() {
        let cfg = PartialEvalConfig::default();
        assert_eq!(cfg.max_specializations, 100);
        assert_eq!(cfg.max_depth, 50);
        assert!(cfg.enable_memoization);
    }
    #[test]
    pub(super) fn test_aggressive_config_larger_limits() {
        let agg = PartialEvalConfig::aggressive();
        let def = PartialEvalConfig::default();
        assert!(agg.max_specializations >= def.max_specializations);
    }
    #[test]
    pub(super) fn test_conservative_config_smaller_limits() {
        let con = PartialEvalConfig::conservative();
        assert!(!con.specialize_hot_paths);
    }
    #[test]
    pub(super) fn test_eval_return_lit() {
        let mut pe = PartialEvaluator::default_eval();
        let mut env = BindingEnv::new();
        let expr = make_return_nat(99);
        let (new_expr, pv) = pe.eval_expr(&expr, &mut env, 0);
        assert!(pv.is_known());
        assert_eq!(new_expr, make_return_nat(99));
    }
    #[test]
    pub(super) fn test_eval_let_lit_binds_known() {
        let mut pe = PartialEvaluator::default_eval();
        let mut env = BindingEnv::new();
        let expr = make_nat_lit(0, 7, make_return_nat(0));
        let (_new_expr, _pv) = pe.eval_expr(&expr, &mut env, 0);
        assert_eq!(
            env.lookup(LcnfVarId(0)),
            &PartialValue::Known(LcnfLit::Nat(7))
        );
    }
    #[test]
    pub(super) fn test_eval_unreachable_gives_contradiction() {
        let mut pe = PartialEvaluator::default_eval();
        let mut env = BindingEnv::new();
        let (_, pv) = pe.eval_expr(&LcnfExpr::Unreachable, &mut env, 0);
        assert_eq!(pv, PartialValue::Contradiction);
    }
    #[test]
    pub(super) fn test_eval_case_with_known_scrutinee_eliminates_branch() {
        let mut pe = PartialEvaluator::default_eval();
        let mut env = BindingEnv::new();
        env.bind(LcnfVarId(0), PartialValue::Known(LcnfLit::Nat(0)));
        let alts = vec![
            LcnfAlt {
                ctor_name: "zero".to_string(),
                ctor_tag: 0,
                params: vec![],
                body: make_return_nat(10),
            },
            LcnfAlt {
                ctor_name: "succ".to_string(),
                ctor_tag: 1,
                params: vec![make_param(2, "n")],
                body: make_return_nat(20),
            },
        ];
        let result = pe.try_eval_case(&LcnfVarId(0), &alts, &None, &mut env, 0);
        assert!(result.is_some());
        let (expr, _) = result.expect("expected Some/Ok value");
        assert_eq!(expr, make_return_nat(10));
        assert_eq!(pe.report().branches_eliminated, 1);
    }
    #[test]
    pub(super) fn test_eval_case_unknown_scrutinee_keeps_all_branches() {
        let mut pe = PartialEvaluator::default_eval();
        let mut env = BindingEnv::new();
        env.bind(LcnfVarId(0), PartialValue::Unknown);
        let case_expr = LcnfExpr::Case {
            scrutinee: LcnfVarId(0),
            scrutinee_ty: LcnfType::Nat,
            alts: vec![
                LcnfAlt {
                    ctor_name: "a".to_string(),
                    ctor_tag: 0,
                    params: vec![],
                    body: make_return_nat(1),
                },
                LcnfAlt {
                    ctor_name: "b".to_string(),
                    ctor_tag: 1,
                    params: vec![],
                    body: make_return_nat(2),
                },
            ],
            default: None,
        };
        let (new_expr, _) = pe.eval_expr(&case_expr, &mut env, 0);
        if let LcnfExpr::Case { alts, .. } = new_expr {
            assert_eq!(alts.len(), 2);
        } else {
            panic!("Expected Case expression");
        }
    }
    #[test]
    pub(super) fn test_specialize_creates_new_decl() {
        let mut pe = PartialEvaluator::default_eval();
        let decl = make_decl(
            "double",
            vec![make_param(0, "n")],
            LcnfExpr::Return(LcnfArg::Var(LcnfVarId(0))),
        );
        pe.known_fns.insert("double".to_string(), decl);
        let result = pe.specialize_function("double", vec![PartialValue::Known(LcnfLit::Nat(5))]);
        assert!(result.is_some());
        let spec = result.expect("spec should be Some/Ok");
        assert!(spec.name.contains("double"));
        assert_eq!(pe.report().functions_specialized, 1);
    }
    #[test]
    pub(super) fn test_specialize_unknown_function_returns_none() {
        let mut pe = PartialEvaluator::default_eval();
        let result = pe.specialize_function("nonexistent", vec![PartialValue::Unknown]);
        assert!(result.is_none());
    }
    #[test]
    pub(super) fn test_specialize_drops_static_params() {
        let mut pe = PartialEvaluator::default_eval();
        let decl = make_decl(
            "f",
            vec![make_param(0, "x"), make_param(1, "y")],
            make_return_nat(0),
        );
        pe.known_fns.insert("f".to_string(), decl);
        let spec = pe
            .specialize_function(
                "f",
                vec![PartialValue::Known(LcnfLit::Nat(10)), PartialValue::Unknown],
            )
            .expect("operation should succeed");
        assert_eq!(spec.params.len(), 1);
    }
    #[test]
    pub(super) fn test_run_empty_decls() {
        let mut pe = PartialEvaluator::default_eval();
        let mut decls: Vec<LcnfFunDecl> = vec![];
        pe.run(&mut decls);
        assert_eq!(pe.report().total_optimizations(), 0);
    }
    #[test]
    pub(super) fn test_run_counts_constants() {
        let mut pe = PartialEvaluator::default_eval();
        let body = make_nat_lit(0, 42, make_return_nat(0));
        let decl = make_decl("g", vec![], body);
        let mut decls = vec![decl];
        pe.run(&mut decls);
        assert!(pe.report().constants_computed > 0);
    }
    #[test]
    pub(super) fn test_report_merge() {
        let mut r1 = PartialEvalReport {
            branches_eliminated: 3,
            functions_specialized: 1,
            constants_computed: 5,
            memo_hits: 2,
            lets_removed: 0,
        };
        let r2 = PartialEvalReport {
            branches_eliminated: 7,
            functions_specialized: 2,
            constants_computed: 3,
            memo_hits: 1,
            lets_removed: 4,
        };
        r1.merge(&r2);
        assert_eq!(r1.branches_eliminated, 10);
        assert_eq!(r1.functions_specialized, 3);
        assert_eq!(r1.lets_removed, 4);
    }
    #[test]
    pub(super) fn test_report_total_optimizations() {
        let r = PartialEvalReport {
            branches_eliminated: 2,
            functions_specialized: 1,
            constants_computed: 3,
            memo_hits: 10,
            lets_removed: 1,
        };
        assert_eq!(r.total_optimizations(), 7);
    }
    #[test]
    pub(super) fn test_report_summary_contains_fields() {
        let r = PartialEvalReport {
            branches_eliminated: 4,
            functions_specialized: 2,
            constants_computed: 6,
            memo_hits: 3,
            lets_removed: 1,
        };
        let s = r.summary();
        assert!(s.contains("branches_elim=4"));
        assert!(s.contains("specialized=2"));
    }
    #[test]
    pub(super) fn test_binding_time_static_for_known() {
        assert_eq!(
            BindingTime::from_partial(&PartialValue::Known(LcnfLit::Nat(0))),
            BindingTime::Static
        );
    }
    #[test]
    pub(super) fn test_binding_time_dynamic_for_unknown() {
        assert_eq!(
            BindingTime::from_partial(&PartialValue::Unknown),
            BindingTime::Dynamic
        );
    }
    #[test]
    pub(super) fn test_binding_time_mixed_for_partial() {
        let pv = PartialValue::Partial(vec![
            PartialValue::Known(LcnfLit::Nat(1)),
            PartialValue::Unknown,
        ]);
        assert_eq!(BindingTime::from_partial(&pv), BindingTime::Mixed);
    }
    #[test]
    pub(super) fn test_analyze_binding_times_length() {
        let decl = make_decl(
            "h",
            vec![make_param(0, "a"), make_param(1, "b")],
            make_return_nat(0),
        );
        let times = analyze_binding_times(
            &decl,
            &[PartialValue::Known(LcnfLit::Nat(1)), PartialValue::Unknown],
        );
        assert_eq!(times.len(), 2);
        assert_eq!(times[0], BindingTime::Static);
        assert_eq!(times[1], BindingTime::Dynamic);
    }
    #[test]
    pub(super) fn test_aggressive_prop_removes_let() {
        let mut cfg = PartialEvalConfig::default();
        cfg.aggressive_const_prop = true;
        let mut pe = PartialEvaluator::new(cfg);
        let mut env = BindingEnv::new();
        let expr = LcnfExpr::Let {
            id: LcnfVarId(0),
            name: "x".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Lit(LcnfLit::Nat(5)),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(0)))),
        };
        let (new_expr, _) = pe.eval_expr(&expr, &mut env, 0);
        assert_eq!(pe.report().lets_removed, 1);
        assert_eq!(new_expr, LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(5))));
    }
}
#[cfg(test)]
mod PE_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = PEPassConfig::new("test_pass", PEPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = PEPassStats::new();
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
        let mut reg = PEPassRegistry::new();
        reg.register(PEPassConfig::new("pass_a", PEPassPhase::Analysis));
        reg.register(PEPassConfig::new("pass_b", PEPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = PEAnalysisCache::new(10);
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
        let mut wl = PEWorklist::new();
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
        let mut dt = PEDominatorTree::new(5);
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
        let mut liveness = PELivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(PEConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(PEConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(PEConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            PEConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(PEConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = PEDepGraph::new();
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
mod peext_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_peext_phase_order() {
        assert_eq!(PEExtPassPhase::Early.order(), 0);
        assert_eq!(PEExtPassPhase::Middle.order(), 1);
        assert_eq!(PEExtPassPhase::Late.order(), 2);
        assert_eq!(PEExtPassPhase::Finalize.order(), 3);
        assert!(PEExtPassPhase::Early.is_early());
        assert!(!PEExtPassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_peext_config_builder() {
        let c = PEExtPassConfig::new("p")
            .with_phase(PEExtPassPhase::Late)
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
    pub(super) fn test_peext_stats() {
        let mut s = PEExtPassStats::new();
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
    pub(super) fn test_peext_registry() {
        let mut r = PEExtPassRegistry::new();
        r.register(PEExtPassConfig::new("a").with_phase(PEExtPassPhase::Early));
        r.register(PEExtPassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&PEExtPassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_peext_cache() {
        let mut c = PEExtCache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_peext_worklist() {
        let mut w = PEExtWorklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_peext_dom_tree() {
        let mut dt = PEExtDomTree::new(5);
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
    pub(super) fn test_peext_liveness() {
        let mut lv = PEExtLiveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_peext_const_folder() {
        let mut cf = PEExtConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_peext_dep_graph() {
        let mut g = PEExtDepGraph::new(4);
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
