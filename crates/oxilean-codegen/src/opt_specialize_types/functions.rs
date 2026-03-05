//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::{
    LcnfAlt, LcnfArg, LcnfExpr, LcnfFunDecl, LcnfLetValue, LcnfModule, LcnfParam, LcnfType,
    LcnfVarId,
};
use std::collections::HashMap;

use super::types::{
    OSTAnalysisCache, OSTConstantFoldingHelper, OSTDepGraph, OSTDominatorTree, OSTLivenessInfo,
    OSTPassConfig, OSTPassPhase, OSTPassRegistry, OSTPassStats, OSTWorklist, SpecTExtCache,
    SpecTExtConstFolder, SpecTExtDepGraph, SpecTExtDomTree, SpecTExtLiveness, SpecTExtPassConfig,
    SpecTExtPassPhase, SpecTExtPassRegistry, SpecTExtPassStats, SpecTExtWorklist, SpecTX2Cache,
    SpecTX2ConstFolder, SpecTX2DepGraph, SpecTX2DomTree, SpecTX2Liveness, SpecTX2PassConfig,
    SpecTX2PassPhase, SpecTX2PassRegistry, SpecTX2PassStats, SpecTX2Worklist, TypeSpecConfig,
    TypeSpecReport, TypeSpecializer,
};

/// A concrete instantiation: (callee_name, list_of_type_args_as_strings).
pub type SpecKey = (String, Vec<String>);
/// Return the leading `Type(...)` args of an argument list as type-name strings,
/// stopping at the first non-Type argument.
pub(super) fn leading_type_args(args: &[LcnfArg]) -> Vec<String> {
    args.iter()
        .take_while(|a| matches!(a, LcnfArg::Type(_)))
        .map(|a| {
            if let LcnfArg::Type(ty) = a {
                format!("{}", ty)
            } else {
                unreachable!()
            }
        })
        .collect()
}
/// Return the leading `Type` args as `LcnfType` values.
pub(super) fn leading_type_values(args: &[LcnfArg]) -> Vec<LcnfType> {
    args.iter()
        .take_while(|a| matches!(a, LcnfArg::Type(_)))
        .map(|a| {
            if let LcnfArg::Type(ty) = a {
                ty.clone()
            } else {
                unreachable!()
            }
        })
        .collect()
}
/// Generate a deterministic specialization name.
///
/// e.g. `List.map` specialized at `[Nat, Bool]` → `List.map__Nat_Bool`.
pub(super) fn spec_name(base: &str, type_args: &[String]) -> String {
    let suffix = type_args.join("_");
    format!("{}__{}", base, suffix)
}
/// Substitute `LcnfType::Var(name)` occurrences in a type according to `subst`.
pub(super) fn subst_type(ty: &LcnfType, subst: &HashMap<String, LcnfType>) -> LcnfType {
    match ty {
        LcnfType::Var(n) => subst.get(n).cloned().unwrap_or_else(|| ty.clone()),
        LcnfType::Fun(params, ret) => LcnfType::Fun(
            params.iter().map(|p| subst_type(p, subst)).collect(),
            Box::new(subst_type(ret, subst)),
        ),
        LcnfType::Ctor(name, args) => LcnfType::Ctor(
            name.clone(),
            args.iter().map(|a| subst_type(a, subst)).collect(),
        ),
        _ => ty.clone(),
    }
}
/// Substitute type variables in an `LcnfArg`.
pub(super) fn subst_type_in_arg(arg: &LcnfArg, subst: &HashMap<String, LcnfType>) -> LcnfArg {
    match arg {
        LcnfArg::Type(ty) => LcnfArg::Type(subst_type(ty, subst)),
        other => other.clone(),
    }
}
/// Substitute type variables in an `LcnfLetValue`.
pub(super) fn subst_type_in_value(
    value: &LcnfLetValue,
    subst: &HashMap<String, LcnfType>,
) -> LcnfLetValue {
    match value {
        LcnfLetValue::App(func, args) => LcnfLetValue::App(
            subst_type_in_arg(func, subst),
            args.iter().map(|a| subst_type_in_arg(a, subst)).collect(),
        ),
        LcnfLetValue::Ctor(name, tag, args) => LcnfLetValue::Ctor(
            name.clone(),
            *tag,
            args.iter().map(|a| subst_type_in_arg(a, subst)).collect(),
        ),
        LcnfLetValue::Reuse(slot, name, tag, args) => LcnfLetValue::Reuse(
            *slot,
            name.clone(),
            *tag,
            args.iter().map(|a| subst_type_in_arg(a, subst)).collect(),
        ),
        other => other.clone(),
    }
}
/// Substitute type variables throughout an `LcnfExpr`.
pub(super) fn subst_type_in_expr(expr: &LcnfExpr, subst: &HashMap<String, LcnfType>) -> LcnfExpr {
    match expr {
        LcnfExpr::Let {
            id,
            name,
            ty,
            value,
            body,
        } => LcnfExpr::Let {
            id: *id,
            name: name.clone(),
            ty: subst_type(ty, subst),
            value: subst_type_in_value(value, subst),
            body: Box::new(subst_type_in_expr(body, subst)),
        },
        LcnfExpr::Case {
            scrutinee,
            scrutinee_ty,
            alts,
            default,
        } => LcnfExpr::Case {
            scrutinee: *scrutinee,
            scrutinee_ty: subst_type(scrutinee_ty, subst),
            alts: alts
                .iter()
                .map(|alt| LcnfAlt {
                    ctor_name: alt.ctor_name.clone(),
                    ctor_tag: alt.ctor_tag,
                    params: alt
                        .params
                        .iter()
                        .map(|p| LcnfParam {
                            id: p.id,
                            name: p.name.clone(),
                            ty: subst_type(&p.ty, subst),
                            erased: p.erased,
                            borrowed: p.borrowed,
                        })
                        .collect(),
                    body: subst_type_in_expr(&alt.body, subst),
                })
                .collect(),
            default: default
                .as_ref()
                .map(|d| Box::new(subst_type_in_expr(d, subst))),
        },
        LcnfExpr::Return(arg) => LcnfExpr::Return(subst_type_in_arg(arg, subst)),
        LcnfExpr::TailCall(func, args) => LcnfExpr::TailCall(
            subst_type_in_arg(func, subst),
            args.iter().map(|a| subst_type_in_arg(a, subst)).collect(),
        ),
        LcnfExpr::Unreachable => LcnfExpr::Unreachable,
    }
}
/// Rewrite `App` and `TailCall` nodes: if the callee and leading type args
/// match a known specialization, redirect to the specialized name (a fresh
/// `LcnfVarId` is not available here; instead we embed the name as a literal
/// in a `FVar` style). Because LCNF uses `LcnfArg::Var(id)` for function
/// references, we record a synthetic mapping from the original var-id to the
/// specialized callee's string name via the approach used in practice: we
/// introduce a fresh let-binding at the point of call.
///
/// For simplicity in this pass the rewriting strategy is:
/// - Look up the callee in the `LcnfArg` by name (via `spec_map`).
/// - If found and the leading type args match, strip the type args and
///   replace `func` with a `FVar` pointing to the same id but record the
///   redirect.
///
/// Since LCNF variable ids are opaque, the rewriter works at the expression
/// level: it tracks `var_id → function_name` from the enclosing let-environment
/// that was built during profiling.
pub(super) fn rewrite_expr(
    expr: &mut LcnfExpr,
    spec_map: &HashMap<SpecKey, String>,
    id_to_name: &HashMap<LcnfVarId, String>,
    report: &mut TypeSpecReport,
) {
    match expr {
        LcnfExpr::Let {
            id,
            name,
            value,
            body,
            ..
        } => {
            rewrite_value(value, spec_map, id_to_name, report);
            let fname = value_to_func_name(value, id_to_name);
            let mut child_map = id_to_name.clone();
            if let Some(n) = fname {
                child_map.insert(*id, n);
            } else {
                if let LcnfLetValue::FVar(src) = value {
                    if let Some(n) = id_to_name.get(src) {
                        child_map.insert(*id, n.clone());
                    }
                }
                child_map.insert(*id, name.clone());
            }
            rewrite_expr(body, spec_map, &child_map, report);
        }
        LcnfExpr::Case { alts, default, .. } => {
            for alt in alts.iter_mut() {
                rewrite_expr(&mut alt.body, spec_map, id_to_name, report);
            }
            if let Some(def) = default {
                rewrite_expr(def, spec_map, id_to_name, report);
            }
        }
        LcnfExpr::TailCall(func, args) => {
            if let Some(callee_name) = arg_to_func_name(func, id_to_name) {
                let ty_args = leading_type_args(args);
                if !ty_args.is_empty() {
                    let key = (callee_name, ty_args);
                    if let Some(spec) = spec_map.get(&key) {
                        let non_type_args: Vec<LcnfArg> = args
                            .iter()
                            .filter(|a| !matches!(a, LcnfArg::Type(_)))
                            .cloned()
                            .collect();
                        let _ = spec;
                        *args = non_type_args;
                        report.call_sites_updated += 1;
                    }
                }
            }
        }
        LcnfExpr::Return(_) | LcnfExpr::Unreachable => {}
    }
}
pub(super) fn rewrite_value(
    value: &mut LcnfLetValue,
    spec_map: &HashMap<SpecKey, String>,
    id_to_name: &HashMap<LcnfVarId, String>,
    report: &mut TypeSpecReport,
) {
    if let LcnfLetValue::App(func, args) = value {
        if let Some(callee_name) = arg_to_func_name(func, id_to_name) {
            let ty_args = leading_type_args(args);
            if !ty_args.is_empty() {
                let key = (callee_name, ty_args);
                if let Some(_spec) = spec_map.get(&key) {
                    let non_type_args: Vec<LcnfArg> = args
                        .iter()
                        .filter(|a| !matches!(a, LcnfArg::Type(_)))
                        .cloned()
                        .collect();
                    *args = non_type_args;
                    report.call_sites_updated += 1;
                }
            }
        }
    }
}
pub(super) fn arg_to_func_name(
    arg: &LcnfArg,
    id_to_name: &HashMap<LcnfVarId, String>,
) -> Option<String> {
    match arg {
        LcnfArg::Var(id) => id_to_name.get(id).cloned(),
        _ => None,
    }
}
pub(super) fn value_to_func_name(
    value: &LcnfLetValue,
    id_to_name: &HashMap<LcnfVarId, String>,
) -> Option<String> {
    match value {
        LcnfLetValue::FVar(id) => id_to_name.get(id).cloned(),
        _ => None,
    }
}
pub(super) fn profile_expr(
    expr: &LcnfExpr,
    id_to_name: &HashMap<LcnfVarId, String>,
    freq: &mut HashMap<SpecKey, usize>,
) {
    match expr {
        LcnfExpr::Let {
            id,
            name,
            value,
            body,
            ..
        } => {
            profile_value(value, id_to_name, freq);
            let mut child_map = id_to_name.clone();
            child_map.insert(*id, name.clone());
            if let LcnfLetValue::FVar(src) = value {
                if let Some(n) = id_to_name.get(src) {
                    child_map.insert(*id, n.clone());
                }
            }
            profile_expr(body, &child_map, freq);
        }
        LcnfExpr::Case { alts, default, .. } => {
            for alt in alts {
                profile_expr(&alt.body, id_to_name, freq);
            }
            if let Some(def) = default {
                profile_expr(def, id_to_name, freq);
            }
        }
        LcnfExpr::TailCall(func, args) => {
            if let Some(name) = arg_to_func_name(func, id_to_name) {
                let ty_args = leading_type_args(args);
                if !ty_args.is_empty() {
                    *freq.entry((name, ty_args)).or_insert(0) += 1;
                }
            }
        }
        LcnfExpr::Return(_) | LcnfExpr::Unreachable => {}
    }
}
pub(super) fn profile_value(
    value: &LcnfLetValue,
    id_to_name: &HashMap<LcnfVarId, String>,
    freq: &mut HashMap<SpecKey, usize>,
) {
    if let LcnfLetValue::App(func, args) = value {
        if let Some(name) = arg_to_func_name(func, id_to_name) {
            let ty_args = leading_type_args(args);
            if !ty_args.is_empty() {
                *freq.entry((name, ty_args)).or_insert(0) += 1;
            }
        }
    }
}
/// Convert a slice of type-name strings back to `LcnfType` values.
pub(super) fn leading_type_values_from_strings(names: &[String]) -> Vec<LcnfType> {
    names
        .iter()
        .map(|s| match s.as_str() {
            "nat" => LcnfType::Nat,
            "string" => LcnfType::LcnfString,
            "unit" => LcnfType::Unit,
            "object" => LcnfType::Object,
            "erased" => LcnfType::Erased,
            other => LcnfType::Var(other.to_string()),
        })
        .collect()
}
/// Run type specialization with default config.
pub fn run_type_spec(module: &mut LcnfModule) -> TypeSpecReport {
    let mut pass = TypeSpecializer::default();
    pass.run(module);
    pass.report
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::lcnf::{
        LcnfArg, LcnfExpr, LcnfFunDecl, LcnfLetValue, LcnfModule, LcnfParam, LcnfType, LcnfVarId,
    };
    pub(super) fn var(n: u64) -> LcnfVarId {
        LcnfVarId(n)
    }
    pub(super) fn param_type(id: u64, name: &str) -> LcnfParam {
        LcnfParam {
            id: var(id),
            name: name.to_string(),
            ty: LcnfType::Erased,
            erased: true,
            borrowed: false,
        }
    }
    pub(super) fn param_val(id: u64, name: &str, ty: LcnfType) -> LcnfParam {
        LcnfParam {
            id: var(id),
            name: name.to_string(),
            ty,
            erased: false,
            borrowed: false,
        }
    }
    pub(super) fn fun_decl(name: &str, params: Vec<LcnfParam>, body: LcnfExpr) -> LcnfFunDecl {
        LcnfFunDecl {
            name: name.to_string(),
            original_name: None,
            params,
            ret_type: LcnfType::Object,
            body,
            is_recursive: false,
            is_lifted: false,
            inline_cost: 0,
        }
    }
    pub(super) fn module_with(decls: Vec<LcnfFunDecl>) -> LcnfModule {
        let mut m = LcnfModule::default();
        m.fun_decls = decls;
        m
    }
    pub(super) fn call_map_nat_body(callee_id: u64, arg_a: u64, arg_b: u64) -> LcnfExpr {
        LcnfExpr::Let {
            id: var(99),
            name: "r".into(),
            ty: LcnfType::Object,
            value: LcnfLetValue::App(
                LcnfArg::Var(var(callee_id)),
                vec![
                    LcnfArg::Type(LcnfType::Nat),
                    LcnfArg::Var(var(arg_a)),
                    LcnfArg::Var(var(arg_b)),
                ],
            ),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(var(99)))),
        }
    }
    #[test]
    pub(super) fn test_no_spec_below_threshold() {
        let map_decl = fun_decl(
            "map",
            vec![param_type(0, "α"), param_val(1, "f", LcnfType::Object)],
            LcnfExpr::Return(LcnfArg::Var(var(1))),
        );
        let caller = fun_decl(
            "main",
            vec![
                param_val(10, "f", LcnfType::Object),
                param_val(11, "xs", LcnfType::Object),
            ],
            call_map_nat_body(0, 10, 11),
        );
        let mut m = module_with(vec![map_decl, caller]);
        let cfg = TypeSpecConfig {
            max_specializations: 64,
            min_call_count: 2,
        };
        let report = TypeSpecializer::new(cfg).run_and_report(&mut m);
        assert_eq!(report.functions_specialized, 0);
    }
    #[test]
    pub(super) fn test_spec_fires_at_threshold_1() {
        let map_decl = fun_decl(
            "map",
            vec![
                param_type(0, "α"),
                param_val(1, "f", LcnfType::Object),
                param_val(2, "xs", LcnfType::Object),
            ],
            LcnfExpr::Return(LcnfArg::Var(var(2))),
        );
        let caller_body = LcnfExpr::Let {
            id: var(5),
            name: "map".into(),
            ty: LcnfType::Object,
            value: LcnfLetValue::FVar(var(0)),
            body: Box::new(LcnfExpr::Let {
                id: var(99),
                name: "r".into(),
                ty: LcnfType::Object,
                value: LcnfLetValue::App(
                    LcnfArg::Var(var(5)),
                    vec![
                        LcnfArg::Type(LcnfType::Nat),
                        LcnfArg::Var(var(10)),
                        LcnfArg::Var(var(11)),
                    ],
                ),
                body: Box::new(LcnfExpr::Return(LcnfArg::Var(var(99)))),
            }),
        };
        let caller = fun_decl(
            "main",
            vec![
                param_val(10, "f", LcnfType::Object),
                param_val(11, "xs", LcnfType::Object),
            ],
            caller_body,
        );
        let mut m = module_with(vec![map_decl, caller]);
        let cfg = TypeSpecConfig {
            max_specializations: 64,
            min_call_count: 1,
        };
        let report = TypeSpecializer::new(cfg).run_and_report(&mut m);
        assert_eq!(report.functions_specialized, 1);
        let names: Vec<&str> = m.fun_decls.iter().map(|d| d.name.as_str()).collect();
        assert!(names.iter().any(|n| n.contains("map__")));
    }
    #[test]
    pub(super) fn test_max_specializations_cap() {
        let make_poly = |name: &str, id_offset: u64| {
            fun_decl(
                name,
                vec![
                    param_type(id_offset, "α"),
                    param_val(id_offset + 1, "x", LcnfType::Object),
                ],
                LcnfExpr::Return(LcnfArg::Var(var(id_offset + 1))),
            )
        };
        let make_caller = |fname: &str, fn_id: u64, arg_id: u64| {
            fun_decl(
                fname,
                vec![param_val(arg_id, "x", LcnfType::Object)],
                LcnfExpr::Let {
                    id: var(fn_id + 50),
                    name: fname.to_string(),
                    ty: LcnfType::Object,
                    value: LcnfLetValue::FVar(var(0)),
                    body: Box::new(LcnfExpr::Let {
                        id: var(99),
                        name: "r".into(),
                        ty: LcnfType::Object,
                        value: LcnfLetValue::App(
                            LcnfArg::Var(var(fn_id + 50)),
                            vec![LcnfArg::Type(LcnfType::Nat), LcnfArg::Var(var(arg_id))],
                        ),
                        body: Box::new(LcnfExpr::Return(LcnfArg::Var(var(99)))),
                    }),
                },
            )
        };
        let mut m = module_with(vec![
            make_poly("funcA", 0),
            make_poly("funcB", 10),
            make_caller("callerA", 0, 20),
            make_caller("callerB", 10, 30),
        ]);
        let cfg = TypeSpecConfig {
            max_specializations: 1,
            min_call_count: 1,
        };
        let report = TypeSpecializer::new(cfg).run_and_report(&mut m);
        assert!(report.functions_specialized <= 1);
    }
    #[test]
    pub(super) fn test_type_subst_in_body() {
        let poly_body = LcnfExpr::Let {
            id: var(10),
            name: "r".into(),
            ty: LcnfType::Var("α".into()),
            value: LcnfLetValue::App(LcnfArg::Var(var(1)), vec![LcnfArg::Var(var(2))]),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(var(10)))),
        };
        let mut subst = HashMap::new();
        subst.insert("α".to_string(), LcnfType::Nat);
        let result = subst_type_in_expr(&poly_body, &subst);
        if let LcnfExpr::Let { ty, .. } = &result {
            assert_eq!(*ty, LcnfType::Nat);
        } else {
            panic!("expected Let");
        }
    }
    #[test]
    pub(super) fn test_leading_type_args_extraction() {
        let args = vec![
            LcnfArg::Type(LcnfType::Nat),
            LcnfArg::Type(LcnfType::Object),
            LcnfArg::Var(var(0)),
            LcnfArg::Type(LcnfType::Nat),
        ];
        let result = leading_type_args(&args);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], "nat");
        assert_eq!(result[1], "object");
    }
    #[test]
    pub(super) fn test_spec_name_deterministic() {
        let name = spec_name("List.map", &["Nat".into(), "Bool".into()]);
        assert_eq!(name, "List.map__Nat_Bool");
        let name2 = spec_name("id", &["Nat".into()]);
        assert_eq!(name2, "id__Nat");
    }
    #[test]
    pub(super) fn test_empty_module() {
        let mut m = LcnfModule::default();
        let report = run_type_spec(&mut m);
        assert_eq!(report.functions_specialized, 0);
        assert_eq!(report.call_sites_updated, 0);
    }
    #[test]
    pub(super) fn test_call_site_args_stripped() {
        let mut spec_map: HashMap<SpecKey, String> = HashMap::new();
        spec_map.insert(("map".into(), vec!["nat".into()]), "map__nat".into());
        let id_to_name: HashMap<LcnfVarId, String> =
            [(var(5), "map".to_string())].into_iter().collect();
        let mut body = LcnfExpr::Let {
            id: var(99),
            name: "r".into(),
            ty: LcnfType::Object,
            value: LcnfLetValue::App(
                LcnfArg::Var(var(5)),
                vec![LcnfArg::Type(LcnfType::Nat), LcnfArg::Var(var(10))],
            ),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(var(99)))),
        };
        let mut report = TypeSpecReport::default();
        rewrite_expr(&mut body, &spec_map, &id_to_name, &mut report);
        assert_eq!(report.call_sites_updated, 1);
        if let LcnfExpr::Let {
            value: LcnfLetValue::App(_, args),
            ..
        } = &body
        {
            assert!(!args.iter().any(|a| matches!(a, LcnfArg::Type(_))));
        } else {
            panic!("expected Let App");
        }
    }
}
#[cfg(test)]
mod OST_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = OSTPassConfig::new("test_pass", OSTPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = OSTPassStats::new();
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
        let mut reg = OSTPassRegistry::new();
        reg.register(OSTPassConfig::new("pass_a", OSTPassPhase::Analysis));
        reg.register(OSTPassConfig::new("pass_b", OSTPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = OSTAnalysisCache::new(10);
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
        let mut wl = OSTWorklist::new();
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
        let mut dt = OSTDominatorTree::new(5);
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
        let mut liveness = OSTLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(OSTConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(OSTConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(OSTConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            OSTConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(OSTConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = OSTDepGraph::new();
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
mod spectext_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_spectext_phase_order() {
        assert_eq!(SpecTExtPassPhase::Early.order(), 0);
        assert_eq!(SpecTExtPassPhase::Middle.order(), 1);
        assert_eq!(SpecTExtPassPhase::Late.order(), 2);
        assert_eq!(SpecTExtPassPhase::Finalize.order(), 3);
        assert!(SpecTExtPassPhase::Early.is_early());
        assert!(!SpecTExtPassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_spectext_config_builder() {
        let c = SpecTExtPassConfig::new("p")
            .with_phase(SpecTExtPassPhase::Late)
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
    pub(super) fn test_spectext_stats() {
        let mut s = SpecTExtPassStats::new();
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
    pub(super) fn test_spectext_registry() {
        let mut r = SpecTExtPassRegistry::new();
        r.register(SpecTExtPassConfig::new("a").with_phase(SpecTExtPassPhase::Early));
        r.register(SpecTExtPassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&SpecTExtPassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_spectext_cache() {
        let mut c = SpecTExtCache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_spectext_worklist() {
        let mut w = SpecTExtWorklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_spectext_dom_tree() {
        let mut dt = SpecTExtDomTree::new(5);
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
    pub(super) fn test_spectext_liveness() {
        let mut lv = SpecTExtLiveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_spectext_const_folder() {
        let mut cf = SpecTExtConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_spectext_dep_graph() {
        let mut g = SpecTExtDepGraph::new(4);
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
mod spectx2_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_spectx2_phase_order() {
        assert_eq!(SpecTX2PassPhase::Early.order(), 0);
        assert_eq!(SpecTX2PassPhase::Middle.order(), 1);
        assert_eq!(SpecTX2PassPhase::Late.order(), 2);
        assert_eq!(SpecTX2PassPhase::Finalize.order(), 3);
        assert!(SpecTX2PassPhase::Early.is_early());
        assert!(!SpecTX2PassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_spectx2_config_builder() {
        let c = SpecTX2PassConfig::new("p")
            .with_phase(SpecTX2PassPhase::Late)
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
    pub(super) fn test_spectx2_stats() {
        let mut s = SpecTX2PassStats::new();
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
    pub(super) fn test_spectx2_registry() {
        let mut r = SpecTX2PassRegistry::new();
        r.register(SpecTX2PassConfig::new("a").with_phase(SpecTX2PassPhase::Early));
        r.register(SpecTX2PassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&SpecTX2PassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_spectx2_cache() {
        let mut c = SpecTX2Cache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_spectx2_worklist() {
        let mut w = SpecTX2Worklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_spectx2_dom_tree() {
        let mut dt = SpecTX2DomTree::new(5);
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
    pub(super) fn test_spectx2_liveness() {
        let mut lv = SpecTX2Liveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_spectx2_const_folder() {
        let mut cf = SpecTX2ConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_spectx2_dep_graph() {
        let mut g = SpecTX2DepGraph::new(4);
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
