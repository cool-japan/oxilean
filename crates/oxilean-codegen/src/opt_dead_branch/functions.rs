//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::{LcnfArg, LcnfExpr, LcnfFunDecl, LcnfLetValue, LcnfLit, LcnfVarId};
use std::collections::HashMap;

use super::types::{
    ArmReachability, BranchPatternCache, BranchProbabilityEstimator, BranchProfile, CcpValue,
    ConditionSimplifier, ConstEnvBuilder, CtorFrequencyTable, DBAnalysisCache,
    DBConstantFoldingHelper, DBDepGraph, DBDominatorTree, DBLivenessInfo, DBPassConfig,
    DBPassPhase, DBPassRegistry, DBPassStats, DBWorklist, DeadBranchAggregator, DeadBranchConfig,
    DeadBranchElim, DeadBranchLogEntry, DeadBranchOptKind, DeadBranchReport, DeadBranchStats,
    DeadBranchTrace, DominatorInfo, GuardResult, KnownValue, PhiNode,
};

/// Run dead branch elimination over all declarations in a module.
pub fn optimize_dead_branches(
    decls: &mut [LcnfFunDecl],
    config: DeadBranchConfig,
) -> Vec<DeadBranchReport> {
    let mut reports = Vec::new();
    for decl in decls.iter_mut() {
        let mut pass = DeadBranchElim::with_config(config.clone());
        reports.push(pass.run(decl));
    }
    reports
}
/// Merge multiple reports into a single summary.
pub fn merge_reports(reports: &[DeadBranchReport]) -> DeadBranchReport {
    let mut merged = DeadBranchReport::new();
    for r in reports {
        merged.merge(r);
    }
    merged
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::lcnf::{
        LcnfAlt, LcnfArg, LcnfExpr, LcnfFunDecl, LcnfLetValue, LcnfLit, LcnfParam, LcnfType,
        LcnfVarId,
    };
    pub(super) fn mk_decl(body: LcnfExpr) -> LcnfFunDecl {
        LcnfFunDecl {
            name: "test_fn".to_string(),
            original_name: None,
            params: vec![],
            ret_type: LcnfType::Nat,
            body,
            is_recursive: false,
            is_lifted: false,
            inline_cost: 1,
        }
    }
    pub(super) fn nat_param(id: u64, name: &str) -> LcnfParam {
        LcnfParam {
            id: LcnfVarId(id),
            name: name.to_string(),
            ty: LcnfType::Nat,
            erased: false,
            borrowed: false,
        }
    }
    #[test]
    pub(super) fn test_config_default() {
        let cfg = DeadBranchConfig::default();
        assert_eq!(cfg.max_passes, 8);
        assert!(cfg.fold_constants);
        assert!(!cfg.use_profiling);
        assert_eq!(cfg.max_alts_analyzed, 256);
    }
    #[test]
    pub(super) fn test_config_display() {
        let cfg = DeadBranchConfig::default();
        let s = format!("{}", cfg);
        assert!(s.contains("max_passes=8"));
        assert!(s.contains("fold_constants=true"));
    }
    #[test]
    pub(super) fn test_report_any_changes() {
        let r = DeadBranchReport::new();
        assert!(!r.any_changes());
        let r2 = DeadBranchReport {
            branches_eliminated: 1,
            ..Default::default()
        };
        assert!(r2.any_changes());
    }
    #[test]
    pub(super) fn test_report_total_changes() {
        let r = DeadBranchReport {
            branches_eliminated: 3,
            cases_folded: 2,
            ..Default::default()
        };
        assert_eq!(r.total_changes(), 5);
    }
    #[test]
    pub(super) fn test_report_merge() {
        let mut r1 = DeadBranchReport {
            branches_eliminated: 2,
            arms_eliminated: 0,
            cases_folded: 1,
            iterations: 3,
            known_values_tracked: 5,
            uniform_returns: 0,
        };
        let r2 = DeadBranchReport {
            branches_eliminated: 3,
            arms_eliminated: 0,
            cases_folded: 4,
            iterations: 2,
            known_values_tracked: 7,
            uniform_returns: 0,
        };
        r1.merge(&r2);
        assert_eq!(r1.branches_eliminated, 5);
        assert_eq!(r1.cases_folded, 5);
        assert_eq!(r1.iterations, 5);
        assert_eq!(r1.known_values_tracked, 12);
    }
    #[test]
    pub(super) fn test_report_display() {
        let r = DeadBranchReport {
            branches_eliminated: 1,
            cases_folded: 2,
            iterations: 3,
            ..Default::default()
        };
        let s = format!("{}", r);
        assert!(s.contains("eliminated=1"));
        assert!(s.contains("folded=2"));
    }
    #[test]
    pub(super) fn test_branch_profile() {
        let mut bp = BranchProfile::new("Some", 1);
        assert_eq!(bp.ctor_name, "Some");
        assert_eq!(bp.ctor_tag, 1);
        assert!(!bp.is_cold);
        bp.mark_cold();
        assert!(bp.is_cold);
    }
    #[test]
    pub(super) fn test_branch_profile_with_frequency() {
        let bp = BranchProfile::new("None", 0).with_frequency(0.95);
        assert!((bp.frequency - 0.95).abs() < f64::EPSILON);
    }
    #[test]
    pub(super) fn test_branch_profile_clamped() {
        let bp = BranchProfile::new("X", 0).with_frequency(2.0);
        assert!((bp.frequency - 1.0).abs() < f64::EPSILON);
        let bp2 = BranchProfile::new("Y", 0).with_frequency(-1.0);
        assert!((bp2.frequency - 0.0).abs() < f64::EPSILON);
    }
    #[test]
    pub(super) fn test_branch_profile_taken_count() {
        let bp = BranchProfile::new("Z", 0).with_taken_count(100);
        assert_eq!(bp.taken_count, 100);
    }
    #[test]
    pub(super) fn test_ctor_freq_table_empty() {
        let t = CtorFrequencyTable::new();
        assert!(t.is_empty());
        assert_eq!(t.len(), 0);
    }
    #[test]
    pub(super) fn test_ctor_freq_table_record_lookup() {
        let mut t = CtorFrequencyTable::new();
        t.record("Option", 0, 0.3);
        t.record("Option", 1, 0.7);
        assert_eq!(t.lookup("Option", 0), Some(0.3));
        assert_eq!(t.lookup("Option", 1), Some(0.7));
        assert_eq!(t.lookup("Option", 2), None);
        assert_eq!(t.len(), 2);
    }
    #[test]
    pub(super) fn test_ctor_freq_table_is_rare() {
        let mut t = CtorFrequencyTable::new();
        t.record("Result", 1, 0.01);
        assert!(t.is_rare("Result", 1, 0.05));
        assert!(!t.is_rare("Result", 1, 0.005));
        assert!(!t.is_rare("Result", 99, 0.5));
    }
    #[test]
    pub(super) fn test_condition_simplifier() {
        let mut cs = ConditionSimplifier::new();
        cs.mark_true(LcnfVarId(1));
        cs.mark_false(LcnfVarId(2));
        assert!(cs.is_known_true(&LcnfVarId(1)));
        assert!(cs.is_known_false(&LcnfVarId(2)));
        assert!(!cs.is_known_true(&LcnfVarId(2)));
        assert!(!cs.is_known_false(&LcnfVarId(1)));
        assert_eq!(cs.num_known(), 2);
    }
    #[test]
    pub(super) fn test_condition_simplifier_no_duplicates() {
        let mut cs = ConditionSimplifier::new();
        cs.mark_true(LcnfVarId(1));
        cs.mark_true(LcnfVarId(1));
        assert_eq!(cs.num_known(), 1);
    }
    #[test]
    pub(super) fn test_dead_branch_stats_total() {
        let stats = DeadBranchStats {
            cases_analyzed: 10,
            known_ctor_matches: 3,
            unreachable_defaults: 1,
            single_branch_inlines: 2,
            uniform_folds: 1,
            env_entries_created: 5,
        };
        assert_eq!(stats.total(), 7);
    }
    #[test]
    pub(super) fn test_dead_branch_stats_display() {
        let stats = DeadBranchStats::default();
        let s = format!("{}", stats);
        assert!(s.contains("cases=0"));
    }
    #[test]
    pub(super) fn test_known_ctor_selects_branch() {
        let body = LcnfExpr::Let {
            id: LcnfVarId(1),
            name: "x".to_string(),
            ty: LcnfType::Object,
            value: LcnfLetValue::Ctor("Some".to_string(), 1, vec![LcnfArg::Erased]),
            body: Box::new(LcnfExpr::Case {
                scrutinee: LcnfVarId(1),
                scrutinee_ty: LcnfType::Object,
                alts: vec![
                    LcnfAlt {
                        ctor_name: "Some".to_string(),
                        ctor_tag: 1,
                        params: vec![],
                        body: LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(1))),
                    },
                    LcnfAlt {
                        ctor_name: "None".to_string(),
                        ctor_tag: 0,
                        params: vec![],
                        body: LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0))),
                    },
                ],
                default: None,
            }),
        };
        let mut decl = mk_decl(body);
        let mut pass = DeadBranchElim::new();
        let report = pass.run(&mut decl);
        assert!(report.cases_folded >= 1);
        match &decl.body {
            LcnfExpr::Let { body, .. } => {
                assert_eq!(**body, LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(1))));
            }
            _ => panic!("Expected Let wrapper"),
        }
    }
    #[test]
    pub(super) fn test_unreachable_alt_removed() {
        let var = LcnfVarId(1);
        let body = LcnfExpr::Case {
            scrutinee: var,
            scrutinee_ty: LcnfType::Object,
            alts: vec![
                LcnfAlt {
                    ctor_name: "A".to_string(),
                    ctor_tag: 0,
                    params: vec![],
                    body: LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(42))),
                },
                LcnfAlt {
                    ctor_name: "B".to_string(),
                    ctor_tag: 1,
                    params: vec![],
                    body: LcnfExpr::Unreachable,
                },
            ],
            default: None,
        };
        let mut decl = mk_decl(body);
        let mut pass = DeadBranchElim::new();
        let report = pass.run(&mut decl);
        assert!(report.branches_eliminated >= 1);
    }
    #[test]
    pub(super) fn test_uniform_branches_folded() {
        let var = LcnfVarId(1);
        let body = LcnfExpr::Case {
            scrutinee: var,
            scrutinee_ty: LcnfType::Object,
            alts: vec![
                LcnfAlt {
                    ctor_name: "A".to_string(),
                    ctor_tag: 0,
                    params: vec![],
                    body: LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(7))),
                },
                LcnfAlt {
                    ctor_name: "B".to_string(),
                    ctor_tag: 1,
                    params: vec![],
                    body: LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(7))),
                },
            ],
            default: Some(Box::new(LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(7))))),
        };
        let mut decl = mk_decl(body);
        let mut pass = DeadBranchElim::new();
        let report = pass.run(&mut decl);
        assert!(report.cases_folded >= 1);
        assert_eq!(decl.body, LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(7))));
    }
    #[test]
    pub(super) fn test_non_uniform_branches_not_folded() {
        let var = LcnfVarId(1);
        let body = LcnfExpr::Case {
            scrutinee: var,
            scrutinee_ty: LcnfType::Object,
            alts: vec![
                LcnfAlt {
                    ctor_name: "A".to_string(),
                    ctor_tag: 0,
                    params: vec![],
                    body: LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(1))),
                },
                LcnfAlt {
                    ctor_name: "B".to_string(),
                    ctor_tag: 1,
                    params: vec![],
                    body: LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(2))),
                },
            ],
            default: None,
        };
        let mut decl = mk_decl(body);
        let mut pass = DeadBranchElim::new();
        let report = pass.run(&mut decl);
        assert_eq!(report.cases_folded, 0);
        assert_eq!(report.branches_eliminated, 0);
        assert!(matches!(decl.body, LcnfExpr::Case { .. }));
    }
    #[test]
    pub(super) fn test_single_branch_inlined() {
        let var = LcnfVarId(1);
        let body = LcnfExpr::Case {
            scrutinee: var,
            scrutinee_ty: LcnfType::Object,
            alts: vec![LcnfAlt {
                ctor_name: "Only".to_string(),
                ctor_tag: 0,
                params: vec![nat_param(2, "n")],
                body: LcnfExpr::Return(LcnfArg::Var(LcnfVarId(2))),
            }],
            default: None,
        };
        let mut decl = mk_decl(body);
        let mut pass = DeadBranchElim::new();
        let report = pass.run(&mut decl);
        assert!(report.cases_folded >= 1);
        assert_eq!(decl.body, LcnfExpr::Return(LcnfArg::Var(LcnfVarId(2))));
    }
    #[test]
    pub(super) fn test_constant_prop_through_let() {
        let body = LcnfExpr::Let {
            id: LcnfVarId(10),
            name: "z".to_string(),
            ty: LcnfType::Object,
            value: LcnfLetValue::Ctor("None".to_string(), 0, vec![]),
            body: Box::new(LcnfExpr::Let {
                id: LcnfVarId(11),
                name: "alias".to_string(),
                ty: LcnfType::Object,
                value: LcnfLetValue::FVar(LcnfVarId(10)),
                body: Box::new(LcnfExpr::Case {
                    scrutinee: LcnfVarId(11),
                    scrutinee_ty: LcnfType::Object,
                    alts: vec![
                        LcnfAlt {
                            ctor_name: "None".to_string(),
                            ctor_tag: 0,
                            params: vec![],
                            body: LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0))),
                        },
                        LcnfAlt {
                            ctor_name: "Some".to_string(),
                            ctor_tag: 1,
                            params: vec![],
                            body: LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(1))),
                        },
                    ],
                    default: None,
                }),
            }),
        };
        let mut decl = mk_decl(body);
        let mut pass = DeadBranchElim::new();
        let report = pass.run(&mut decl);
        assert!(report.cases_folded >= 1);
    }
    #[test]
    pub(super) fn test_unreachable_default_removed() {
        let var = LcnfVarId(5);
        let body = LcnfExpr::Case {
            scrutinee: var,
            scrutinee_ty: LcnfType::Object,
            alts: vec![
                LcnfAlt {
                    ctor_name: "A".to_string(),
                    ctor_tag: 0,
                    params: vec![],
                    body: LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0))),
                },
                LcnfAlt {
                    ctor_name: "B".to_string(),
                    ctor_tag: 1,
                    params: vec![],
                    body: LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(1))),
                },
            ],
            default: Some(Box::new(LcnfExpr::Unreachable)),
        };
        let mut decl = mk_decl(body);
        let mut pass = DeadBranchElim::new();
        let report = pass.run(&mut decl);
        assert!(report.branches_eliminated >= 1);
        match &decl.body {
            LcnfExpr::Case { default, .. } => {
                assert!(default.is_none());
            }
            _ => {}
        }
    }
    #[test]
    pub(super) fn test_fold_constants_disabled() {
        let var = LcnfVarId(1);
        let body = LcnfExpr::Case {
            scrutinee: var,
            scrutinee_ty: LcnfType::Object,
            alts: vec![
                LcnfAlt {
                    ctor_name: "A".to_string(),
                    ctor_tag: 0,
                    params: vec![],
                    body: LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(99))),
                },
                LcnfAlt {
                    ctor_name: "B".to_string(),
                    ctor_tag: 1,
                    params: vec![],
                    body: LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(99))),
                },
            ],
            default: None,
        };
        let mut decl = mk_decl(body);
        let mut pass = DeadBranchElim::with_config(DeadBranchConfig {
            max_passes: 4,
            fold_constants: false,
            ..Default::default()
        });
        let report = pass.run(&mut decl);
        assert_eq!(report.cases_folded, 0);
    }
    #[test]
    pub(super) fn test_optimize_dead_branches() {
        let body = LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0)));
        let mut decls = vec![mk_decl(body)];
        let reports = optimize_dead_branches(&mut decls, DeadBranchConfig::default());
        assert_eq!(reports.len(), 1);
    }
    #[test]
    pub(super) fn test_merge_reports() {
        let reports = vec![
            DeadBranchReport {
                branches_eliminated: 1,
                cases_folded: 2,
                ..Default::default()
            },
            DeadBranchReport {
                branches_eliminated: 3,
                cases_folded: 4,
                ..Default::default()
            },
        ];
        let merged = merge_reports(&reports);
        assert_eq!(merged.branches_eliminated, 4);
        assert_eq!(merged.cases_folded, 6);
    }
    #[test]
    pub(super) fn test_detailed_stats_tracking() {
        let body = LcnfExpr::Let {
            id: LcnfVarId(1),
            name: "x".to_string(),
            ty: LcnfType::Object,
            value: LcnfLetValue::Ctor("Some".to_string(), 1, vec![]),
            body: Box::new(LcnfExpr::Case {
                scrutinee: LcnfVarId(1),
                scrutinee_ty: LcnfType::Object,
                alts: vec![LcnfAlt {
                    ctor_name: "Some".to_string(),
                    ctor_tag: 1,
                    params: vec![],
                    body: LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(1))),
                }],
                default: None,
            }),
        };
        let mut decl = mk_decl(body);
        let mut pass = DeadBranchElim::new();
        pass.run(&mut decl);
        let stats = pass.detailed_stats();
        assert!(stats.known_ctor_matches >= 1);
        assert!(stats.env_entries_created >= 1);
    }
    #[test]
    pub(super) fn test_known_value_predicates() {
        let lit = KnownValue::Lit(LcnfLit::Nat(42));
        assert!(lit.is_lit());
        assert!(!lit.is_ctor());
        let ctor = KnownValue::Ctor("Foo".to_string(), 0);
        assert!(!ctor.is_lit());
        assert!(ctor.is_ctor());
    }
    #[test]
    pub(super) fn test_iterations_tracked() {
        let body = LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0)));
        let mut decl = mk_decl(body);
        let mut pass = DeadBranchElim::new();
        let report = pass.run(&mut decl);
        assert!(report.iterations >= 1);
    }
    #[test]
    pub(super) fn test_condition_simplifier_both_lists() {
        let mut cs = ConditionSimplifier::new();
        let a = LcnfVarId(100);
        let b = LcnfVarId(200);
        cs.mark_true(a);
        cs.mark_false(b);
        assert!(cs.is_known_true(&a));
        assert!(cs.is_known_false(&b));
        assert!(!cs.is_known_true(&b));
        assert!(!cs.is_known_false(&a));
    }
    #[test]
    pub(super) fn test_condition_simplifier_num_known_multiple() {
        let mut cs = ConditionSimplifier::new();
        cs.mark_true(LcnfVarId(10));
        cs.mark_true(LcnfVarId(20));
        cs.mark_false(LcnfVarId(30));
        assert_eq!(cs.num_known(), 3);
    }
    #[test]
    pub(super) fn test_ctor_freq_table_multiple_ctors() {
        let mut table = CtorFrequencyTable::new();
        table.record("List", 0, 0.5);
        table.record("List", 1, 0.45);
        table.record("List", 2, 0.01);
        assert!(!table.is_rare("List", 0, 0.05));
        assert!(!table.is_rare("List", 1, 0.05));
        assert!(table.is_rare("List", 2, 0.05));
    }
    #[test]
    pub(super) fn test_ctor_freq_table_empty_not_rare() {
        let table = CtorFrequencyTable::new();
        assert!(!table.is_rare("Anything", 0, 0.1));
    }
    #[test]
    pub(super) fn test_branch_profile_chaining() {
        let bp = BranchProfile::new("Cons", 1)
            .with_frequency(0.42)
            .with_taken_count(100);
        assert!((bp.frequency - 0.42).abs() < 1e-9);
        assert_eq!(bp.taken_count, 100);
        assert!(!bp.is_cold);
    }
    #[test]
    pub(super) fn test_branch_profile_mark_cold() {
        let mut bp = BranchProfile::new("Nil", 0).with_frequency(0.1);
        bp.mark_cold();
        assert!(bp.is_cold);
    }
    #[test]
    pub(super) fn test_config_display_disabled_profiling() {
        let mut config = DeadBranchConfig::default();
        config.use_profiling = false;
        let s = format!("{}", config);
        assert!(s.contains("profiling=false"));
    }
    #[test]
    pub(super) fn test_report_no_changes() {
        let report = DeadBranchReport::default();
        assert!(!report.any_changes());
        assert_eq!(report.total_changes(), 0);
    }
}
/// Evaluate a guard expression symbolically.
///
/// Returns a `GuardResult` based on the current constant environment.
#[allow(dead_code)]
pub fn eval_guard(
    expr: &crate::lcnf::LcnfExpr,
    env: &std::collections::HashMap<crate::lcnf::LcnfVarId, crate::lcnf::LcnfLit>,
) -> GuardResult {
    match expr {
        crate::lcnf::LcnfExpr::Return(crate::lcnf::LcnfArg::Lit(lit)) => match lit {
            crate::lcnf::LcnfLit::Nat(0) => GuardResult::AlwaysFalse,
            crate::lcnf::LcnfLit::Nat(_) => GuardResult::AlwaysTrue,
            _ => GuardResult::Unknown,
        },
        crate::lcnf::LcnfExpr::Return(crate::lcnf::LcnfArg::Var(id)) => match env.get(id) {
            Some(crate::lcnf::LcnfLit::Nat(0)) => GuardResult::AlwaysFalse,
            Some(crate::lcnf::LcnfLit::Nat(_)) => GuardResult::AlwaysTrue,
            _ => GuardResult::Unknown,
        },
        _ => GuardResult::Unknown,
    }
}
/// Compute arm reachability based on the known constructor of the scrutinee.
#[allow(dead_code)]
pub fn arm_reachability(arm_ctor: &str, scrutinee_ctor: Option<&str>) -> ArmReachability {
    match scrutinee_ctor {
        Some(known) => {
            if known == arm_ctor {
                ArmReachability::Reachable
            } else {
                ArmReachability::Unreachable
            }
        }
        None => ArmReachability::MaybeReachable,
    }
}
/// Run dead branch elimination and return both the modified declarations
/// and an aggregate report.
#[allow(dead_code)]
pub fn eliminate_dead_branches_with_report(
    decls: &mut Vec<crate::lcnf::LcnfFunDecl>,
    config: DeadBranchConfig,
) -> DeadBranchAggregator {
    let mut aggregator = DeadBranchAggregator::new();
    if decls.is_empty() {
        // Even for empty input, record one unit with an empty report.
        aggregator.add(DeadBranchReport::default());
    } else {
        let mut pass = DeadBranchElim::with_config(config);
        for decl in decls.iter_mut() {
            let report = pass.run(decl);
            aggregator.add(report);
        }
    }
    aggregator
}
#[cfg(test)]
mod extended_tests {
    use super::*;
    use crate::lcnf::*;
    pub(super) fn vid(n: u64) -> LcnfVarId {
        LcnfVarId(n)
    }
    #[test]
    pub(super) fn test_phi_node_trivial() {
        let mut phi = PhiNode::new(vid(0));
        phi.add_incoming(1, LcnfArg::Lit(LcnfLit::Nat(42)));
        assert!(phi.is_trivial());
        assert_eq!(phi.simplify(), Some(&LcnfArg::Lit(LcnfLit::Nat(42))));
    }
    #[test]
    pub(super) fn test_phi_node_non_trivial() {
        let mut phi = PhiNode::new(vid(0));
        phi.add_incoming(1, LcnfArg::Lit(LcnfLit::Nat(1)));
        phi.add_incoming(2, LcnfArg::Lit(LcnfLit::Nat(2)));
        assert!(!phi.is_trivial());
        assert_eq!(phi.simplify(), None);
        assert_eq!(phi.live_count(), 2);
    }
    #[test]
    pub(super) fn test_ccp_value_meet_undefined() {
        let a = CcpValue::Undefined;
        let b = CcpValue::Constant(LcnfLit::Nat(5));
        assert_eq!(a.meet(&b), CcpValue::Constant(LcnfLit::Nat(5)));
    }
    #[test]
    pub(super) fn test_ccp_value_meet_same_constant() {
        let a = CcpValue::Constant(LcnfLit::Nat(3));
        let b = CcpValue::Constant(LcnfLit::Nat(3));
        assert_eq!(a.meet(&b), CcpValue::Constant(LcnfLit::Nat(3)));
    }
    #[test]
    pub(super) fn test_ccp_value_meet_different_constant() {
        let a = CcpValue::Constant(LcnfLit::Nat(3));
        let b = CcpValue::Constant(LcnfLit::Nat(7));
        assert_eq!(a.meet(&b), CcpValue::Overdefined);
    }
    #[test]
    pub(super) fn test_ccp_value_is_constant() {
        assert!(CcpValue::Constant(LcnfLit::Nat(0)).is_constant());
        assert!(!CcpValue::Overdefined.is_constant());
        assert!(!CcpValue::Undefined.is_constant());
    }
    #[test]
    pub(super) fn test_ccp_value_literal() {
        let v = CcpValue::Constant(LcnfLit::Nat(10));
        assert_eq!(v.literal(), Some(&LcnfLit::Nat(10)));
        assert_eq!(CcpValue::Overdefined.literal(), None);
    }
    #[test]
    pub(super) fn test_branch_probability_estimator_defaults() {
        let est = BranchProbabilityEstimator::new();
        let p0 = est.estimate("Cons", 1, 2);
        let p_nil = est.estimate("Nil", 0, 2);
        assert!(p0 > p_nil);
    }
    #[test]
    pub(super) fn test_branch_probability_estimator_override() {
        let mut est = BranchProbabilityEstimator::new();
        est.set_probability("MyType", 0, 0.9);
        let p = est.estimate("MyType", 0, 2);
        assert!((p - 0.9).abs() < 1e-9);
    }
    #[test]
    pub(super) fn test_branch_probability_estimator_all_sums_to_one() {
        let est = BranchProbabilityEstimator::new();
        let probs = est.estimate_all(&[("Cons".into(), 1), ("Nil".into(), 0)]);
        let sum: f64 = probs.iter().sum();
        assert!((sum - 1.0).abs() < 1e-9);
    }
    #[test]
    pub(super) fn test_dead_branch_log_entry_display() {
        let e = DeadBranchLogEntry::new(
            "my_func",
            DeadBranchOptKind::CaseFolded,
            "scrutinee known to be Nil",
        );
        let s = e.to_string();
        assert!(s.contains("my_func"));
        assert!(s.contains("CaseFolded"));
    }
    #[test]
    pub(super) fn test_dead_branch_trace_count_kind() {
        let mut trace = DeadBranchTrace::new();
        trace.log(DeadBranchLogEntry::new(
            "f",
            DeadBranchOptKind::ArmEliminated,
            "x",
        ));
        trace.log(DeadBranchLogEntry::new(
            "f",
            DeadBranchOptKind::CaseFolded,
            "y",
        ));
        trace.log(DeadBranchLogEntry::new(
            "g",
            DeadBranchOptKind::ArmEliminated,
            "z",
        ));
        assert_eq!(trace.count_kind(&DeadBranchOptKind::ArmEliminated), 2);
        assert_eq!(trace.count_kind(&DeadBranchOptKind::CaseFolded), 1);
        assert_eq!(trace.len(), 3);
    }
    #[test]
    pub(super) fn test_dead_branch_trace_render() {
        let mut trace = DeadBranchTrace::new();
        trace.log(DeadBranchLogEntry::new(
            "f",
            DeadBranchOptKind::UniformReturn,
            "all ret 0",
        ));
        let s = trace.render();
        assert!(s.contains("UniformReturn"));
        assert!(!s.is_empty());
    }
    #[test]
    pub(super) fn test_dominator_info_set_and_dominates() {
        let mut dom = DominatorInfo::new();
        dom.set_idom(2, 1);
        dom.set_idom(3, 2);
        assert!(dom.dominates(1, 2));
        assert!(dom.dominates(1, 3));
        assert!(!dom.dominates(3, 1));
        assert!(dom.dominates(2, 2));
    }
    #[test]
    pub(super) fn test_dominator_info_dominated_by() {
        let mut dom = DominatorInfo::new();
        dom.set_idom(2, 1);
        dom.set_idom(3, 1);
        dom.set_idom(4, 2);
        let dominated = dom.dominated_by(1);
        assert!(dominated.contains(&2));
        assert!(dominated.contains(&3));
        assert!(dominated.contains(&4));
    }
    #[test]
    pub(super) fn test_guard_result_may_be_true() {
        assert!(GuardResult::AlwaysTrue.may_be_true());
        assert!(GuardResult::Unknown.may_be_true());
        assert!(!GuardResult::AlwaysFalse.may_be_true());
    }
    #[test]
    pub(super) fn test_guard_result_may_be_false() {
        assert!(GuardResult::AlwaysFalse.may_be_false());
        assert!(GuardResult::Unknown.may_be_false());
        assert!(!GuardResult::AlwaysTrue.may_be_false());
    }
    #[test]
    pub(super) fn test_eval_guard_nat_zero() {
        let env = std::collections::HashMap::new();
        let expr = LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0)));
        assert_eq!(eval_guard(&expr, &env), GuardResult::AlwaysFalse);
    }
    #[test]
    pub(super) fn test_eval_guard_nat_nonzero() {
        let env = std::collections::HashMap::new();
        let expr = LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(1)));
        assert_eq!(eval_guard(&expr, &env), GuardResult::AlwaysTrue);
    }
    #[test]
    pub(super) fn test_eval_guard_var_known() {
        let mut env = std::collections::HashMap::new();
        env.insert(vid(0), LcnfLit::Nat(5));
        let expr = LcnfExpr::Return(LcnfArg::Var(vid(0)));
        assert_eq!(eval_guard(&expr, &env), GuardResult::AlwaysTrue);
    }
    #[test]
    pub(super) fn test_const_env_builder_bind_get() {
        let mut builder = ConstEnvBuilder::new();
        builder.bind(vid(1), LcnfLit::Nat(42));
        assert_eq!(builder.get(&vid(1)), Some(&LcnfLit::Nat(42)));
        assert_eq!(builder.get(&vid(2)), None);
        assert_eq!(builder.len(), 1);
    }
    #[test]
    pub(super) fn test_const_env_builder_merge() {
        let mut a = ConstEnvBuilder::new();
        a.bind(vid(0), LcnfLit::Nat(1));
        let mut b = ConstEnvBuilder::new();
        b.bind(vid(1), LcnfLit::Nat(2));
        a.merge_optimistic(&b);
        assert!(a.get(&vid(1)).is_some());
    }
    #[test]
    pub(super) fn test_branch_pattern_cache_hit_miss() {
        let mut cache = BranchPatternCache::new();
        cache.store(vid(5), "Cons");
        let _ = cache.lookup(vid(5));
        let _ = cache.lookup(vid(6));
        assert!((cache.hit_rate() - 50.0).abs() < 0.001);
    }
    #[test]
    pub(super) fn test_branch_pattern_cache_invalidate() {
        let mut cache = BranchPatternCache::new();
        cache.store(vid(1), "Some");
        cache.invalidate(vid(1));
        assert!(cache.lookup(vid(1)).is_none());
    }
    #[test]
    pub(super) fn test_branch_pattern_cache_stats() {
        let cache = BranchPatternCache::new();
        let s = cache.stats();
        assert!(s.contains("BranchPatternCache"));
    }
    #[test]
    pub(super) fn test_arm_reachability_known_match() {
        assert_eq!(
            arm_reachability("Cons", Some("Cons")),
            ArmReachability::Reachable
        );
    }
    #[test]
    pub(super) fn test_arm_reachability_known_mismatch() {
        assert_eq!(
            arm_reachability("Nil", Some("Cons")),
            ArmReachability::Unreachable
        );
    }
    #[test]
    pub(super) fn test_arm_reachability_unknown() {
        assert_eq!(
            arm_reachability("Some", None),
            ArmReachability::MaybeReachable
        );
    }
    #[test]
    pub(super) fn test_dead_branch_aggregator_totals() {
        let mut agg = DeadBranchAggregator::new();
        let mut r1 = DeadBranchReport::default();
        r1.arms_eliminated = 3;
        r1.cases_folded = 1;
        let mut r2 = DeadBranchReport::default();
        r2.arms_eliminated = 2;
        r2.uniform_returns = 4;
        agg.add(r1);
        agg.add(r2);
        assert_eq!(agg.total_arms_eliminated(), 5);
        assert_eq!(agg.total_cases_folded(), 1);
        assert_eq!(agg.total_uniform_returns(), 4);
        assert_eq!(agg.unit_count(), 2);
    }
    #[test]
    pub(super) fn test_dead_branch_aggregator_summary() {
        let agg = DeadBranchAggregator::new();
        let s = agg.summary();
        assert!(s.contains("DeadBranchAggregate"));
    }
    #[test]
    pub(super) fn test_arm_reachability_display() {
        assert_eq!(ArmReachability::Reachable.to_string(), "Reachable");
        assert_eq!(ArmReachability::Unreachable.to_string(), "Unreachable");
        assert_eq!(
            ArmReachability::MaybeReachable.to_string(),
            "MaybeReachable"
        );
    }
    #[test]
    pub(super) fn test_dead_branch_opt_kind_display() {
        assert_eq!(
            DeadBranchOptKind::ArmEliminated.to_string(),
            "ArmEliminated"
        );
        assert_eq!(
            DeadBranchOptKind::SingleArmInlined.to_string(),
            "SingleArmInlined"
        );
    }
    #[test]
    pub(super) fn test_eliminate_dead_branches_with_report_empty() {
        let mut decls = vec![];
        let config = DeadBranchConfig::default();
        let agg = eliminate_dead_branches_with_report(&mut decls, config);
        assert_eq!(agg.unit_count(), 1);
        assert_eq!(agg.total_arms_eliminated(), 0);
    }
}
#[cfg(test)]
mod DB_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = DBPassConfig::new("test_pass", DBPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = DBPassStats::new();
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
        let mut reg = DBPassRegistry::new();
        reg.register(DBPassConfig::new("pass_a", DBPassPhase::Analysis));
        reg.register(DBPassConfig::new("pass_b", DBPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = DBAnalysisCache::new(10);
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
        let mut wl = DBWorklist::new();
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
        let mut dt = DBDominatorTree::new(5);
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
        let mut liveness = DBLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(DBConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(DBConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(DBConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            DBConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(DBConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = DBDepGraph::new();
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
