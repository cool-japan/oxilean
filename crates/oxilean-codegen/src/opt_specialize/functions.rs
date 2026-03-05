//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::{HashMap, HashSet};

use super::types::{
    NumericSpecializer, SizeBudget, SpecAnalysisCache, SpecCallSite, SpecClosureArg, SpecConstArg,
    SpecConstantFoldingHelper, SpecDepGraph, SpecDominatorTree, SpecExtCache, SpecExtConstFolder,
    SpecExtDepGraph, SpecExtDomTree, SpecExtLiveness, SpecExtPassConfig, SpecExtPassPhase,
    SpecExtPassRegistry, SpecExtPassStats, SpecExtWorklist, SpecLivenessInfo, SpecPassConfig,
    SpecPassPhase, SpecPassRegistry, SpecPassStats, SpecTypeArg, SpecWorklist, SpecializationCache,
    SpecializationConfig, SpecializationKey, SpecializationPass, SpecializationStats,
};

/// Generate a short suffix for a type
pub(super) fn type_suffix(ty: &LcnfType) -> String {
    match ty {
        LcnfType::Nat => "nat".to_string(),
        LcnfType::Object => "obj".to_string(),
        LcnfType::Unit => "unit".to_string(),
        LcnfType::Erased => "e".to_string(),
        LcnfType::LcnfString => "str".to_string(),
        LcnfType::Var(name) => name.clone(),
        LcnfType::Ctor(name, _) => name.clone(),
        LcnfType::Fun(_, _) => "fn".to_string(),
        LcnfType::Irrelevant => "irr".to_string(),
    }
}
/// Analyze call sites in a function body to find specialization opportunities
pub(super) fn find_specialization_sites(
    expr: &LcnfExpr,
    known_constants: &HashMap<LcnfVarId, LcnfLit>,
    known_functions: &HashMap<LcnfVarId, String>,
    decl_names: &HashSet<String>,
) -> Vec<SpecCallSite> {
    let mut sites = Vec::new();
    let mut call_idx = 0;
    find_spec_sites_inner(
        expr,
        known_constants,
        known_functions,
        decl_names,
        &mut sites,
        &mut call_idx,
    );
    sites
}
pub(super) fn find_spec_sites_inner(
    expr: &LcnfExpr,
    known_constants: &HashMap<LcnfVarId, LcnfLit>,
    known_functions: &HashMap<LcnfVarId, String>,
    decl_names: &HashSet<String>,
    sites: &mut Vec<SpecCallSite>,
    call_idx: &mut usize,
) {
    match expr {
        LcnfExpr::Let {
            id, value, body, ..
        } => {
            let mut extended_consts = known_constants.clone();
            if let LcnfLetValue::Lit(lit) = value {
                extended_consts.insert(*id, lit.clone());
            }
            let mut extended_fns = known_functions.clone();
            if let LcnfLetValue::FVar(fvar) = value {
                if let Some(fname) = known_functions.get(fvar) {
                    extended_fns.insert(*id, fname.clone());
                }
            }
            if let LcnfLetValue::App(func, args) = value {
                let callee_name = match func {
                    LcnfArg::Var(v) => known_functions.get(v).cloned(),
                    _ => None,
                };
                if let Some(ref callee) = callee_name {
                    if decl_names.contains(callee.as_str()) {
                        let const_args: Vec<SpecConstArg> = args
                            .iter()
                            .map(|arg| match arg {
                                LcnfArg::Lit(LcnfLit::Nat(n)) => SpecConstArg::Nat(*n),
                                LcnfArg::Lit(LcnfLit::Str(s)) => SpecConstArg::Str(s.clone()),
                                LcnfArg::Var(v) => {
                                    if let Some(lit) = extended_consts.get(v) {
                                        match lit {
                                            LcnfLit::Nat(n) => SpecConstArg::Nat(*n),
                                            LcnfLit::Str(s) => SpecConstArg::Str(s.clone()),
                                        }
                                    } else {
                                        SpecConstArg::Unknown
                                    }
                                }
                                _ => SpecConstArg::Unknown,
                            })
                            .collect();
                        let closure_args: Vec<SpecClosureArg> = args
                            .iter()
                            .enumerate()
                            .map(|(i, arg)| {
                                let known_fn = match arg {
                                    LcnfArg::Var(v) => extended_fns.get(v).cloned(),
                                    _ => None,
                                };
                                SpecClosureArg {
                                    known_fn,
                                    param_idx: i,
                                }
                            })
                            .collect();
                        let callee_var = match func {
                            LcnfArg::Var(v) => Some(*v),
                            _ => None,
                        };
                        sites.push(SpecCallSite {
                            callee: callee.clone(),
                            call_idx: *call_idx,
                            type_args: vec![],
                            const_args,
                            closure_args,
                            callee_var,
                        });
                        *call_idx += 1;
                    }
                }
            }
            find_spec_sites_inner(
                body,
                &extended_consts,
                &extended_fns,
                decl_names,
                sites,
                call_idx,
            );
        }
        LcnfExpr::Case { alts, default, .. } => {
            for alt in alts {
                find_spec_sites_inner(
                    &alt.body,
                    known_constants,
                    known_functions,
                    decl_names,
                    sites,
                    call_idx,
                );
            }
            if let Some(def) = default {
                find_spec_sites_inner(
                    def,
                    known_constants,
                    known_functions,
                    decl_names,
                    sites,
                    call_idx,
                );
            }
        }
        LcnfExpr::TailCall(func, args) => {
            let callee_name = match func {
                LcnfArg::Var(v) => known_functions.get(v).cloned(),
                _ => None,
            };
            if let Some(callee) = callee_name {
                if decl_names.contains(callee.as_str()) {
                    let const_args: Vec<SpecConstArg> = args
                        .iter()
                        .map(|arg| match arg {
                            LcnfArg::Lit(LcnfLit::Nat(n)) => SpecConstArg::Nat(*n),
                            LcnfArg::Lit(LcnfLit::Str(s)) => SpecConstArg::Str(s.clone()),
                            LcnfArg::Var(v) => {
                                if let Some(lit) = known_constants.get(v) {
                                    match lit {
                                        LcnfLit::Nat(n) => SpecConstArg::Nat(*n),
                                        LcnfLit::Str(s) => SpecConstArg::Str(s.clone()),
                                    }
                                } else {
                                    SpecConstArg::Unknown
                                }
                            }
                            _ => SpecConstArg::Unknown,
                        })
                        .collect();
                    let closure_args: Vec<SpecClosureArg> = args
                        .iter()
                        .enumerate()
                        .map(|(i, arg)| {
                            let known_fn = match arg {
                                LcnfArg::Var(v) => known_functions.get(v).cloned(),
                                _ => None,
                            };
                            SpecClosureArg {
                                known_fn,
                                param_idx: i,
                            }
                        })
                        .collect();
                    let callee_var = match func {
                        LcnfArg::Var(v) => Some(*v),
                        _ => None,
                    };
                    sites.push(SpecCallSite {
                        callee,
                        call_idx: *call_idx,
                        type_args: vec![],
                        const_args,
                        closure_args,
                        callee_var,
                    });
                    *call_idx += 1;
                }
            }
        }
        LcnfExpr::Return(_) | LcnfExpr::Unreachable => {}
    }
}
/// Count the number of LCNF instructions in an expression
pub(super) fn count_instructions(expr: &LcnfExpr) -> usize {
    match expr {
        LcnfExpr::Let { body, .. } => 1 + count_instructions(body),
        LcnfExpr::Case { alts, default, .. } => {
            let alts_size: usize = alts.iter().map(|a| count_instructions(&a.body)).sum();
            let def_size = default.as_ref().map(|d| count_instructions(d)).unwrap_or(0);
            1 + alts_size + def_size
        }
        LcnfExpr::Return(_) | LcnfExpr::TailCall(_, _) | LcnfExpr::Unreachable => 1,
    }
}
/// Analyze whether a function parameter is always called with the same closure
pub(super) fn analyze_closure_uniformity(
    decl: &LcnfFunDecl,
    param_idx: usize,
    sites: &[SpecCallSite],
) -> Option<String> {
    let mut known_fn: Option<String> = None;
    for site in sites {
        if site.callee != decl.name {
            continue;
        }
        if param_idx >= site.closure_args.len() {
            return None;
        }
        match &site.closure_args[param_idx].known_fn {
            Some(fn_name) => {
                if let Some(ref existing) = known_fn {
                    if existing != fn_name {
                        return None;
                    }
                } else {
                    known_fn = Some(fn_name.clone());
                }
            }
            None => return None,
        }
    }
    known_fn
}
/// Check whether a parameter is used as a function (called) in the body
pub(super) fn is_called_as_function(expr: &LcnfExpr, param_id: LcnfVarId) -> bool {
    match expr {
        LcnfExpr::Let { value, body, .. } => {
            let called_here = matches!(
                value, LcnfLetValue::App(LcnfArg::Var(v), _) if * v == param_id
            );
            called_here || is_called_as_function(body, param_id)
        }
        LcnfExpr::Case { alts, default, .. } => {
            alts.iter()
                .any(|a| is_called_as_function(&a.body, param_id))
                || default
                    .as_ref()
                    .is_some_and(|d| is_called_as_function(d, param_id))
        }
        LcnfExpr::TailCall(LcnfArg::Var(v), _) => *v == param_id,
        _ => false,
    }
}
/// Main entry point: specialize functions in a module
pub fn specialize_module(module: &mut LcnfModule, config: &SpecializationConfig) {
    let mut pass = SpecializationPass::new(config.clone());
    pass.run(module);
}
/// Specialize a single function for numeric operations
pub fn specialize_numeric(decl: &LcnfFunDecl) -> Option<LcnfFunDecl> {
    let specializer = NumericSpecializer::new();
    if !specializer.is_numeric_op(&decl.name) {
        return None;
    }
    let mut spec = decl.clone();
    spec.name = format!("{}_u64", decl.name);
    for param in &mut spec.params {
        param.ty = specializer.specialize_nat_to_u64(&param.ty);
    }
    spec.ret_type = specializer.specialize_nat_to_u64(&spec.ret_type);
    Some(spec)
}
/// Check if a function is worth specializing based on heuristics
pub fn is_worth_specializing(decl: &LcnfFunDecl, config: &SpecializationConfig) -> bool {
    let size = count_instructions(&decl.body);
    if size > config.size_threshold {
        return false;
    }
    let has_poly = decl
        .params
        .iter()
        .any(|p| matches!(p.ty, LcnfType::Var(_) | LcnfType::Object | LcnfType::Erased));
    let has_fn_param = decl
        .params
        .iter()
        .any(|p| matches!(p.ty, LcnfType::Fun(_, _)));
    has_poly || (has_fn_param && config.specialize_closures)
}
#[cfg(test)]
mod tests {
    use super::*;
    pub(super) fn make_var(n: u64) -> LcnfVarId {
        LcnfVarId(n)
    }
    pub(super) fn make_param(n: u64, name: &str, ty: LcnfType) -> LcnfParam {
        LcnfParam {
            id: LcnfVarId(n),
            name: name.to_string(),
            ty,
            erased: false,
            borrowed: false,
        }
    }
    pub(super) fn make_simple_let(id: u64, value: LcnfLetValue, body: LcnfExpr) -> LcnfExpr {
        LcnfExpr::Let {
            id: LcnfVarId(id),
            name: format!("x{}", id),
            ty: LcnfType::Nat,
            value,
            body: Box::new(body),
        }
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
            inline_cost: 1,
        }
    }
    #[test]
    pub(super) fn test_config_default() {
        let config = SpecializationConfig::default();
        assert_eq!(config.max_specializations, 8);
        assert!(config.specialize_closures);
        assert!(config.specialize_numerics);
        assert_eq!(config.size_threshold, 200);
    }
    #[test]
    pub(super) fn test_spec_key_trivial() {
        let key = SpecializationKey {
            original: "foo".to_string(),
            type_args: vec![SpecTypeArg::Poly],
            const_args: vec![SpecConstArg::Unknown],
            closure_args: vec![SpecClosureArg {
                known_fn: None,
                param_idx: 0,
            }],
        };
        assert!(key.is_trivial());
    }
    #[test]
    pub(super) fn test_spec_key_non_trivial_type() {
        let key = SpecializationKey {
            original: "foo".to_string(),
            type_args: vec![SpecTypeArg::Concrete(LcnfType::Nat)],
            const_args: vec![],
            closure_args: vec![],
        };
        assert!(!key.is_trivial());
    }
    #[test]
    pub(super) fn test_spec_key_non_trivial_const() {
        let key = SpecializationKey {
            original: "foo".to_string(),
            type_args: vec![],
            const_args: vec![SpecConstArg::Nat(42)],
            closure_args: vec![],
        };
        assert!(!key.is_trivial());
    }
    #[test]
    pub(super) fn test_spec_key_mangled_name() {
        let key = SpecializationKey {
            original: "List.map".to_string(),
            type_args: vec![SpecTypeArg::Concrete(LcnfType::Nat)],
            const_args: vec![SpecConstArg::Unknown],
            closure_args: vec![],
        };
        let name = key.mangled_name();
        assert!(name.starts_with("List.map"));
        assert!(name.contains("_T0_nat"));
    }
    #[test]
    pub(super) fn test_spec_key_mangled_name_with_const() {
        let key = SpecializationKey {
            original: "repeat".to_string(),
            type_args: vec![],
            const_args: vec![SpecConstArg::Nat(3)],
            closure_args: vec![],
        };
        let name = key.mangled_name();
        assert!(name.contains("_C0_N3"));
    }
    #[test]
    pub(super) fn test_spec_key_mangled_name_with_closure() {
        let key = SpecializationKey {
            original: "List.map".to_string(),
            type_args: vec![],
            const_args: vec![],
            closure_args: vec![SpecClosureArg {
                known_fn: Some("double".to_string()),
                param_idx: 0,
            }],
        };
        let name = key.mangled_name();
        assert!(name.contains("_Fdouble"));
    }
    #[test]
    pub(super) fn test_type_suffix() {
        assert_eq!(type_suffix(&LcnfType::Nat), "nat");
        assert_eq!(type_suffix(&LcnfType::Object), "obj");
        assert_eq!(type_suffix(&LcnfType::Unit), "unit");
        assert_eq!(type_suffix(&LcnfType::LcnfString), "str");
    }
    #[test]
    pub(super) fn test_cache_operations() {
        let mut cache = SpecializationCache::new();
        let key = SpecializationKey {
            original: "foo".to_string(),
            type_args: vec![SpecTypeArg::Concrete(LcnfType::Nat)],
            const_args: vec![],
            closure_args: vec![],
        };
        assert!(cache.lookup(&key).is_none());
        cache.insert(key.clone(), "foo_nat".to_string(), 10);
        assert_eq!(cache.lookup(&key), Some("foo_nat"));
        assert_eq!(cache.specialization_count("foo"), 1);
        assert_eq!(cache.total_growth, 10);
    }
    #[test]
    pub(super) fn test_size_budget() {
        let mut budget = SizeBudget::new(100, 2.0);
        assert!(budget.can_afford(50));
        assert!(budget.can_afford(100));
        assert!(!budget.can_afford(101));
        budget.spend(50);
        assert!(budget.can_afford(50));
        assert!(!budget.can_afford(51));
        assert_eq!(budget.remaining(), 50);
    }
    #[test]
    pub(super) fn test_numeric_specializer() {
        let specializer = NumericSpecializer::new();
        assert!(specializer.is_numeric_op("Nat.add"));
        assert!(specializer.is_numeric_op("Nat.mul"));
        assert!(!specializer.is_numeric_op("List.map"));
    }
    #[test]
    pub(super) fn test_numeric_type_specialization() {
        let specializer = NumericSpecializer::new();
        let ty = LcnfType::Fun(vec![LcnfType::Nat, LcnfType::Nat], Box::new(LcnfType::Nat));
        let spec = specializer.specialize_nat_to_u64(&ty);
        assert_eq!(
            spec,
            LcnfType::Fun(vec![LcnfType::Nat, LcnfType::Nat], Box::new(LcnfType::Nat))
        );
    }
    #[test]
    pub(super) fn test_specialize_numeric() {
        let body = LcnfExpr::Return(LcnfArg::Var(make_var(0)));
        let decl = make_decl(
            "Nat.add",
            vec![
                make_param(0, "a", LcnfType::Nat),
                make_param(1, "b", LcnfType::Nat),
            ],
            body,
        );
        let result = specialize_numeric(&decl);
        assert!(result.is_some());
        let spec = result.expect("spec should be Some/Ok");
        assert_eq!(spec.name, "Nat.add_u64");
    }
    #[test]
    pub(super) fn test_specialize_numeric_non_numeric() {
        let body = LcnfExpr::Return(LcnfArg::Var(make_var(0)));
        let decl = make_decl("List.map", vec![make_param(0, "f", LcnfType::Object)], body);
        let result = specialize_numeric(&decl);
        assert!(result.is_none());
    }
    #[test]
    pub(super) fn test_is_worth_specializing_polymorphic() {
        let body = LcnfExpr::Return(LcnfArg::Var(make_var(0)));
        let decl = make_decl(
            "id",
            vec![make_param(0, "x", LcnfType::Var("a".to_string()))],
            body,
        );
        let config = SpecializationConfig::default();
        assert!(is_worth_specializing(&decl, &config));
    }
    #[test]
    pub(super) fn test_is_worth_specializing_concrete() {
        let body = LcnfExpr::Return(LcnfArg::Var(make_var(0)));
        let decl = make_decl(
            "add",
            vec![
                make_param(0, "a", LcnfType::Nat),
                make_param(1, "b", LcnfType::Nat),
            ],
            body,
        );
        let config = SpecializationConfig::default();
        assert!(!is_worth_specializing(&decl, &config));
    }
    #[test]
    pub(super) fn test_is_worth_specializing_higher_order() {
        let body = LcnfExpr::Return(LcnfArg::Var(make_var(0)));
        let fn_ty = LcnfType::Fun(vec![LcnfType::Nat], Box::new(LcnfType::Nat));
        let decl = make_decl("apply", vec![make_param(0, "f", fn_ty)], body);
        let config = SpecializationConfig::default();
        assert!(is_worth_specializing(&decl, &config));
    }
    #[test]
    pub(super) fn test_is_called_as_function() {
        let body = make_simple_let(
            5,
            LcnfLetValue::App(
                LcnfArg::Var(make_var(0)),
                vec![LcnfArg::Lit(LcnfLit::Nat(1))],
            ),
            LcnfExpr::Return(LcnfArg::Var(make_var(5))),
        );
        assert!(is_called_as_function(&body, make_var(0)));
        assert!(!is_called_as_function(&body, make_var(1)));
    }
    #[test]
    pub(super) fn test_count_instructions() {
        let expr = make_simple_let(
            1,
            LcnfLetValue::Lit(LcnfLit::Nat(42)),
            make_simple_let(
                2,
                LcnfLetValue::Lit(LcnfLit::Nat(10)),
                LcnfExpr::Return(LcnfArg::Var(make_var(2))),
            ),
        );
        assert_eq!(count_instructions(&expr), 3);
    }
    #[test]
    pub(super) fn test_substitute_constant() {
        let mut expr = make_simple_let(
            1,
            LcnfLetValue::FVar(make_var(0)),
            LcnfExpr::Return(LcnfArg::Var(make_var(1))),
        );
        let pass = SpecializationPass::new(SpecializationConfig::default());
        pass.substitute_constant(&mut expr, make_var(0), &LcnfLit::Nat(42));
        if let LcnfExpr::Let { value, .. } = &expr {
            assert_eq!(*value, LcnfLetValue::Lit(LcnfLit::Nat(42)));
        } else {
            panic!("Expected Let");
        }
    }
    #[test]
    pub(super) fn test_substitute_constant_in_return() {
        let mut expr = LcnfExpr::Return(LcnfArg::Var(make_var(0)));
        let pass = SpecializationPass::new(SpecializationConfig::default());
        pass.substitute_constant(&mut expr, make_var(0), &LcnfLit::Nat(99));
        assert_eq!(expr, LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(99))));
    }
    #[test]
    pub(super) fn test_specialize_module_empty() {
        let mut module = LcnfModule::default();
        let config = SpecializationConfig::default();
        specialize_module(&mut module, &config);
        assert!(module.fun_decls.is_empty());
    }
    #[test]
    pub(super) fn test_specialize_module_simple() {
        let body = LcnfExpr::Return(LcnfArg::Var(make_var(0)));
        let decl = make_decl(
            "id",
            vec![make_param(0, "x", LcnfType::Var("a".to_string()))],
            body,
        );
        let mut module = LcnfModule {
            fun_decls: vec![decl],
            extern_decls: vec![],
            name: "test".to_string(),
            metadata: LcnfModuleMetadata::default(),
        };
        let config = SpecializationConfig::default();
        specialize_module(&mut module, &config);
        assert!(!module.fun_decls.is_empty());
    }
    #[test]
    pub(super) fn test_closure_uniformity_analysis() {
        let body = LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0)));
        let decl = make_decl("apply", vec![make_param(0, "f", LcnfType::Object)], body);
        let sites = vec![
            SpecCallSite {
                callee: "apply".to_string(),
                call_idx: 0,
                type_args: vec![],
                const_args: vec![],
                closure_args: vec![SpecClosureArg {
                    known_fn: Some("double".to_string()),
                    param_idx: 0,
                }],
                callee_var: None,
            },
            SpecCallSite {
                callee: "apply".to_string(),
                call_idx: 1,
                type_args: vec![],
                const_args: vec![],
                closure_args: vec![SpecClosureArg {
                    known_fn: Some("double".to_string()),
                    param_idx: 0,
                }],
                callee_var: None,
            },
        ];
        let result = analyze_closure_uniformity(&decl, 0, &sites);
        assert_eq!(result, Some("double".to_string()));
    }
    #[test]
    pub(super) fn test_closure_uniformity_non_uniform() {
        let body = LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0)));
        let decl = make_decl("apply", vec![make_param(0, "f", LcnfType::Object)], body);
        let sites = vec![
            SpecCallSite {
                callee: "apply".to_string(),
                call_idx: 0,
                type_args: vec![],
                const_args: vec![],
                closure_args: vec![SpecClosureArg {
                    known_fn: Some("double".to_string()),
                    param_idx: 0,
                }],
                callee_var: None,
            },
            SpecCallSite {
                callee: "apply".to_string(),
                call_idx: 1,
                type_args: vec![],
                const_args: vec![],
                closure_args: vec![SpecClosureArg {
                    known_fn: Some("triple".to_string()),
                    param_idx: 0,
                }],
                callee_var: None,
            },
        ];
        let result = analyze_closure_uniformity(&decl, 0, &sites);
        assert!(result.is_none());
    }
    #[test]
    pub(super) fn test_find_specialization_sites() {
        let body = make_simple_let(
            1,
            LcnfLetValue::App(
                LcnfArg::Var(make_var(10)),
                vec![LcnfArg::Lit(LcnfLit::Nat(42))],
            ),
            LcnfExpr::Return(LcnfArg::Var(make_var(1))),
        );
        let known_consts: HashMap<LcnfVarId, LcnfLit> = HashMap::new();
        let mut known_fns: HashMap<LcnfVarId, String> = HashMap::new();
        known_fns.insert(make_var(10), "target_fn".to_string());
        let mut decl_names = HashSet::new();
        decl_names.insert("target_fn".to_string());
        let sites = find_specialization_sites(&body, &known_consts, &known_fns, &decl_names);
        assert_eq!(sites.len(), 1);
        assert_eq!(sites[0].callee, "target_fn");
        assert!(matches!(sites[0].const_args[0], SpecConstArg::Nat(42)));
    }
    #[test]
    pub(super) fn test_create_specialization() {
        let body = LcnfExpr::Return(LcnfArg::Var(make_var(0)));
        let decl = make_decl(
            "my_fn",
            vec![
                make_param(0, "x", LcnfType::Nat),
                make_param(1, "y", LcnfType::Nat),
            ],
            body,
        );
        let key = SpecializationKey {
            original: "my_fn".to_string(),
            type_args: vec![],
            const_args: vec![SpecConstArg::Nat(10), SpecConstArg::Unknown],
            closure_args: vec![],
        };
        let mut pass = SpecializationPass::new(SpecializationConfig::default());
        let result = pass.create_specialization(&decl, &key);
        assert!(result.is_some());
        let spec = result.expect("spec should be Some/Ok");
        assert!(spec.decl.name.contains("my_fn"));
        assert!(spec.decl.name.contains("_C0_N10"));
        assert_eq!(spec.decl.params.len(), 1);
    }
    #[test]
    pub(super) fn test_stats_default() {
        let stats = SpecializationStats::default();
        assert_eq!(stats.type_specializations, 0);
        assert_eq!(stats.const_specializations, 0);
        assert_eq!(stats.closure_specializations, 0);
    }
    #[test]
    pub(super) fn test_pass_fresh_id() {
        let mut pass = SpecializationPass::new(SpecializationConfig::default());
        let id1 = pass.fresh_id();
        let id2 = pass.fresh_id();
        assert_ne!(id1, id2);
    }
    #[test]
    pub(super) fn test_substitute_in_tailcall() {
        let mut expr = LcnfExpr::TailCall(
            LcnfArg::Var(make_var(10)),
            vec![LcnfArg::Var(make_var(0)), LcnfArg::Var(make_var(1))],
        );
        let pass = SpecializationPass::new(SpecializationConfig::default());
        pass.substitute_constant(&mut expr, make_var(0), &LcnfLit::Nat(7));
        if let LcnfExpr::TailCall(_, args) = &expr {
            assert_eq!(args[0], LcnfArg::Lit(LcnfLit::Nat(7)));
            assert_eq!(args[1], LcnfArg::Var(make_var(1)));
        } else {
            panic!("Expected TailCall");
        }
    }
    #[test]
    pub(super) fn test_is_called_in_case() {
        let body = LcnfExpr::Case {
            scrutinee: make_var(1),
            scrutinee_ty: LcnfType::Nat,
            alts: vec![LcnfAlt {
                ctor_name: "True".to_string(),
                ctor_tag: 0,
                params: vec![],
                body: make_simple_let(
                    5,
                    LcnfLetValue::App(
                        LcnfArg::Var(make_var(0)),
                        vec![LcnfArg::Lit(LcnfLit::Nat(1))],
                    ),
                    LcnfExpr::Return(LcnfArg::Var(make_var(5))),
                ),
            }],
            default: None,
        };
        assert!(is_called_as_function(&body, make_var(0)));
        assert!(!is_called_as_function(&body, make_var(2)));
    }
    #[test]
    pub(super) fn test_tailcall_specialization_site() {
        let expr = LcnfExpr::TailCall(
            LcnfArg::Var(make_var(10)),
            vec![LcnfArg::Lit(LcnfLit::Nat(5))],
        );
        let mut known_fns: HashMap<LcnfVarId, String> = HashMap::new();
        known_fns.insert(make_var(10), "recurse".to_string());
        let mut decl_names = HashSet::new();
        decl_names.insert("recurse".to_string());
        let known_consts: HashMap<LcnfVarId, LcnfLit> = HashMap::new();
        let sites = find_specialization_sites(&expr, &known_consts, &known_fns, &decl_names);
        assert_eq!(sites.len(), 1);
        assert_eq!(sites[0].callee, "recurse");
    }
    #[test]
    pub(super) fn test_recursive_specialization_disabled() {
        let body = LcnfExpr::Return(LcnfArg::Var(make_var(0)));
        let mut decl = make_decl("rec_fn", vec![make_param(0, "n", LcnfType::Nat)], body);
        decl.is_recursive = true;
        let key = SpecializationKey {
            original: "rec_fn".to_string(),
            type_args: vec![],
            const_args: vec![SpecConstArg::Nat(5)],
            closure_args: vec![],
        };
        let mut pass = SpecializationPass::new(SpecializationConfig {
            allow_recursive: false,
            ..SpecializationConfig::default()
        });
        let result = pass.create_specialization(&decl, &key);
        assert!(result.is_none());
    }
}
#[cfg(test)]
mod Spec_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = SpecPassConfig::new("test_pass", SpecPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = SpecPassStats::new();
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
        let mut reg = SpecPassRegistry::new();
        reg.register(SpecPassConfig::new("pass_a", SpecPassPhase::Analysis));
        reg.register(SpecPassConfig::new("pass_b", SpecPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = SpecAnalysisCache::new(10);
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
        let mut wl = SpecWorklist::new();
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
        let mut dt = SpecDominatorTree::new(5);
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
        let mut liveness = SpecLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(SpecConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(SpecConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(SpecConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            SpecConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(SpecConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = SpecDepGraph::new();
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
mod specext_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_specext_phase_order() {
        assert_eq!(SpecExtPassPhase::Early.order(), 0);
        assert_eq!(SpecExtPassPhase::Middle.order(), 1);
        assert_eq!(SpecExtPassPhase::Late.order(), 2);
        assert_eq!(SpecExtPassPhase::Finalize.order(), 3);
        assert!(SpecExtPassPhase::Early.is_early());
        assert!(!SpecExtPassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_specext_config_builder() {
        let c = SpecExtPassConfig::new("p")
            .with_phase(SpecExtPassPhase::Late)
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
    pub(super) fn test_specext_stats() {
        let mut s = SpecExtPassStats::new();
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
    pub(super) fn test_specext_registry() {
        let mut r = SpecExtPassRegistry::new();
        r.register(SpecExtPassConfig::new("a").with_phase(SpecExtPassPhase::Early));
        r.register(SpecExtPassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&SpecExtPassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_specext_cache() {
        let mut c = SpecExtCache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_specext_worklist() {
        let mut w = SpecExtWorklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_specext_dom_tree() {
        let mut dt = SpecExtDomTree::new(5);
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
    pub(super) fn test_specext_liveness() {
        let mut lv = SpecExtLiveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_specext_const_folder() {
        let mut cf = SpecExtConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_specext_dep_graph() {
        let mut g = SpecExtDepGraph::new(4);
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
