//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::{LcnfArg, LcnfExpr, LcnfFunDecl, LcnfLetValue, LcnfParam, LcnfVarId};
use std::collections::HashMap;

use super::types::{
    ArityMap, BetaEtaConfig, BetaEtaPass, BetaEtaReport, CtorEnv, ExtendedPassConfig,
    ExtendedPassReport, FreshIdGen, KnownValue, LetBinding, LitEnv, ModuleOptStats, OptHint,
    ParamUsageSummary,
};

/// Run the beta/eta pass with default configuration on a function declaration.
pub fn run_beta_eta(decl: &mut LcnfFunDecl) -> BetaEtaReport {
    let mut pass = BetaEtaPass::default();
    pass.run(decl);
    pass.report
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::lcnf::{
        LcnfArg, LcnfExpr, LcnfFunDecl, LcnfLetValue, LcnfParam, LcnfType, LcnfVarId,
    };
    pub(super) fn var(n: u64) -> LcnfVarId {
        LcnfVarId(n)
    }
    pub(super) fn param(id: u64, name: &str) -> LcnfParam {
        LcnfParam {
            id: var(id),
            name: name.to_string(),
            ty: LcnfType::Object,
            erased: false,
            borrowed: false,
        }
    }
    pub(super) fn make_decl(name: &str, params: Vec<LcnfParam>, body: LcnfExpr) -> LcnfFunDecl {
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
    #[test]
    pub(super) fn test_trivial_return_unchanged() {
        let mut decl = make_decl(
            "id",
            vec![param(0, "x")],
            LcnfExpr::Return(LcnfArg::Var(var(0))),
        );
        let report = run_beta_eta(&mut decl);
        assert_eq!(report.beta_reductions, 0);
        assert_eq!(report.eta_reductions, 0);
        assert!(matches!(decl.body, LcnfExpr::Return(LcnfArg::Var(_))));
    }
    #[test]
    pub(super) fn test_beta_copy_propagation() {
        let body = LcnfExpr::Let {
            id: var(1),
            name: "copy".into(),
            ty: LcnfType::Object,
            value: LcnfLetValue::FVar(var(0)),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(var(1)))),
        };
        let mut decl = make_decl("f", vec![param(0, "x")], body);
        let report = run_beta_eta(&mut decl);
        assert_eq!(report.beta_reductions, 1);
    }
    #[test]
    pub(super) fn test_eta_reduction_wrapper() {
        let body = LcnfExpr::Let {
            id: var(10),
            name: "r".into(),
            ty: LcnfType::Object,
            value: LcnfLetValue::App(
                LcnfArg::Var(var(99)),
                vec![LcnfArg::Var(var(0)), LcnfArg::Var(var(1))],
            ),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(var(10)))),
        };
        let mut decl = make_decl("wrapper", vec![param(0, "a"), param(1, "b")], body);
        let report = run_beta_eta(&mut decl);
        assert_eq!(report.eta_reductions, 1);
        assert!(matches!(decl.body, LcnfExpr::TailCall(_, _)));
    }
    #[test]
    pub(super) fn test_eta_no_reduction_wrong_args() {
        let body = LcnfExpr::Let {
            id: var(10),
            name: "r".into(),
            ty: LcnfType::Object,
            value: LcnfLetValue::App(
                LcnfArg::Var(var(99)),
                vec![LcnfArg::Var(var(1)), LcnfArg::Var(var(0))],
            ),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(var(10)))),
        };
        let mut decl = make_decl("f", vec![param(0, "a"), param(1, "b")], body);
        let report = run_beta_eta(&mut decl);
        assert_eq!(report.eta_reductions, 0);
    }
    #[test]
    pub(super) fn test_curried_opportunity_counted() {
        let body = LcnfExpr::Let {
            id: var(5),
            name: "t".into(),
            ty: LcnfType::Object,
            value: LcnfLetValue::App(LcnfArg::Var(var(99)), vec![LcnfArg::Var(var(0))]),
            body: Box::new(LcnfExpr::Let {
                id: var(6),
                name: "r".into(),
                ty: LcnfType::Object,
                value: LcnfLetValue::App(LcnfArg::Var(var(5)), vec![LcnfArg::Var(var(1))]),
                body: Box::new(LcnfExpr::Return(LcnfArg::Var(var(6)))),
            }),
        };
        let mut decl = make_decl("curried", vec![param(0, "a"), param(1, "b")], body);
        let report = run_beta_eta(&mut decl);
        assert_eq!(report.curried_opportunities, 1);
    }
    #[test]
    pub(super) fn test_beta_no_reduction_on_lit() {
        let body = LcnfExpr::Let {
            id: var(1),
            name: "x".into(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Lit(crate::lcnf::LcnfLit::Nat(42)),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(var(1)))),
        };
        let mut decl = make_decl("const", vec![], body);
        let report = run_beta_eta(&mut decl);
        assert_eq!(report.beta_reductions, 0);
        assert_eq!(report.eta_reductions, 0);
    }
    #[test]
    pub(super) fn test_eta_single_param() {
        let body = LcnfExpr::Let {
            id: var(2),
            name: "r".into(),
            ty: LcnfType::Object,
            value: LcnfLetValue::App(LcnfArg::Var(var(50)), vec![LcnfArg::Var(var(0))]),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(var(2)))),
        };
        let mut decl = make_decl("wrap1", vec![param(0, "x")], body);
        let report = run_beta_eta(&mut decl);
        assert_eq!(report.eta_reductions, 1);
        assert!(matches!(decl.body, LcnfExpr::TailCall(_, _)));
    }
    #[test]
    pub(super) fn test_fvar_chain() {
        let body = LcnfExpr::Let {
            id: var(1),
            name: "a".into(),
            ty: LcnfType::Object,
            value: LcnfLetValue::FVar(var(0)),
            body: Box::new(LcnfExpr::Let {
                id: var(2),
                name: "b".into(),
                ty: LcnfType::Object,
                value: LcnfLetValue::FVar(var(1)),
                body: Box::new(LcnfExpr::Return(LcnfArg::Var(var(2)))),
            }),
        };
        let mut decl = make_decl("chain", vec![param(0, "x")], body);
        let report = run_beta_eta(&mut decl);
        assert_eq!(report.beta_reductions, 2);
    }
}
/// Count how many times each variable is used in an expression.
#[allow(dead_code)]
pub fn count_uses(expr: &LcnfExpr, uses: &mut HashMap<LcnfVarId, usize>) {
    match expr {
        LcnfExpr::Let { value, body, .. } => {
            count_uses_in_value(value, uses);
            count_uses(body, uses);
        }
        LcnfExpr::Case {
            scrutinee,
            alts,
            default,
            ..
        } => {
            *uses.entry(*scrutinee).or_insert(0) += 1;
            for alt in alts {
                count_uses(&alt.body, uses);
            }
            if let Some(def) = default {
                count_uses(def, uses);
            }
        }
        LcnfExpr::Return(arg) => count_uses_in_arg(arg, uses),
        LcnfExpr::TailCall(func, args) => {
            count_uses_in_arg(func, uses);
            for a in args {
                count_uses_in_arg(a, uses);
            }
        }
        LcnfExpr::Unreachable => {}
    }
}
#[allow(dead_code)]
pub(super) fn count_uses_in_value(value: &LcnfLetValue, uses: &mut HashMap<LcnfVarId, usize>) {
    match value {
        LcnfLetValue::App(func, args) => {
            count_uses_in_arg(func, uses);
            for a in args {
                count_uses_in_arg(a, uses);
            }
        }
        LcnfLetValue::Ctor(_, _, args) | LcnfLetValue::Reuse(_, _, _, args) => {
            for a in args {
                count_uses_in_arg(a, uses);
            }
        }
        LcnfLetValue::FVar(id) | LcnfLetValue::Reset(id) => {
            *uses.entry(*id).or_insert(0) += 1;
        }
        LcnfLetValue::Proj(_, _, id) => {
            *uses.entry(*id).or_insert(0) += 1;
        }
        LcnfLetValue::Lit(_) | LcnfLetValue::Erased => {}
    }
}
#[allow(dead_code)]
pub(super) fn count_uses_in_arg(arg: &LcnfArg, uses: &mut HashMap<LcnfVarId, usize>) {
    if let LcnfArg::Var(id) = arg {
        *uses.entry(*id).or_insert(0) += 1;
    }
}
/// Remove `let x = v` bindings where `x` is never used and `v` has no side effects.
#[allow(dead_code)]
pub fn dead_let_elim(expr: LcnfExpr, report: &mut ExtendedPassReport) -> LcnfExpr {
    let mut uses: HashMap<LcnfVarId, usize> = HashMap::new();
    count_uses(&expr, &mut uses);
    dead_let_elim_inner(expr, &uses, report)
}
pub(super) fn dead_let_elim_inner(
    expr: LcnfExpr,
    uses: &HashMap<LcnfVarId, usize>,
    report: &mut ExtendedPassReport,
) -> LcnfExpr {
    match expr {
        LcnfExpr::Let {
            id,
            name,
            ty,
            value,
            body,
        } => {
            let use_count = uses.get(&id).copied().unwrap_or(0);
            let _is_pure = is_pure_value(&value);
            if use_count == 0 && is_pure_value(&value) {
                report.dead_lets_eliminated += 1;
                dead_let_elim_inner(*body, uses, report)
            } else {
                LcnfExpr::Let {
                    id,
                    name,
                    ty,
                    value,
                    body: Box::new(dead_let_elim_inner(*body, uses, report)),
                }
            }
        }
        LcnfExpr::Case {
            scrutinee,
            scrutinee_ty,
            alts,
            default,
        } => {
            let new_alts = alts
                .into_iter()
                .map(|alt| {
                    let new_body = dead_let_elim_inner(alt.body, uses, report);
                    crate::lcnf::LcnfAlt {
                        ctor_name: alt.ctor_name,
                        ctor_tag: alt.ctor_tag,
                        params: alt.params,
                        body: new_body,
                    }
                })
                .collect();
            let new_default = default.map(|d| Box::new(dead_let_elim_inner(*d, uses, report)));
            LcnfExpr::Case {
                scrutinee,
                scrutinee_ty,
                alts: new_alts,
                default: new_default,
            }
        }
        other => other,
    }
}
#[allow(dead_code)]
pub(super) fn is_pure_value(value: &LcnfLetValue) -> bool {
    matches!(
        value,
        LcnfLetValue::Lit(_)
            | LcnfLetValue::Erased
            | LcnfLetValue::FVar(_)
            | LcnfLetValue::Ctor(_, _, _)
            | LcnfLetValue::Proj(_, _, _)
    )
}
/// Float let-bindings out of case alternative arms.
#[allow(dead_code)]
pub fn let_float(expr: LcnfExpr, report: &mut ExtendedPassReport) -> LcnfExpr {
    let_float_inner(expr, report, 0)
}
pub(super) fn let_float_inner(
    expr: LcnfExpr,
    report: &mut ExtendedPassReport,
    depth: usize,
) -> LcnfExpr {
    if depth > 64 {
        return expr;
    }
    match expr {
        LcnfExpr::Let {
            id,
            name,
            ty,
            value,
            body,
        } => {
            let new_body = let_float_inner(*body, report, depth + 1);
            LcnfExpr::Let {
                id,
                name,
                ty,
                value,
                body: Box::new(new_body),
            }
        }
        LcnfExpr::Case {
            scrutinee,
            scrutinee_ty,
            alts,
            default,
        } => {
            let new_alts: Vec<crate::lcnf::LcnfAlt> = alts
                .into_iter()
                .map(|alt| {
                    let new_body = let_float_inner(alt.body, report, depth + 1);
                    crate::lcnf::LcnfAlt {
                        ctor_name: alt.ctor_name,
                        ctor_tag: alt.ctor_tag,
                        params: alt.params,
                        body: new_body,
                    }
                })
                .collect();
            let new_default = default.map(|d| Box::new(let_float_inner(*d, report, depth + 1)));
            LcnfExpr::Case {
                scrutinee,
                scrutinee_ty,
                alts: new_alts,
                default: new_default,
            }
        }
        other => other,
    }
}
/// Eliminate `case x of { K ... -> body }` when `x` is known to be `K`.
#[allow(dead_code)]
pub fn case_of_known_ctor(
    expr: LcnfExpr,
    env: &CtorEnv,
    report: &mut ExtendedPassReport,
) -> LcnfExpr {
    match expr {
        LcnfExpr::Let {
            id,
            name,
            ty,
            value,
            body,
        } => {
            let mut new_env = env.clone();
            if let LcnfLetValue::Ctor(ref cname, tag, _) = value {
                new_env.record(id, cname.clone(), tag as u16);
            }
            let new_body = case_of_known_ctor(*body, &new_env, report);
            LcnfExpr::Let {
                id,
                name,
                ty,
                value,
                body: Box::new(new_body),
            }
        }
        LcnfExpr::Case {
            scrutinee,
            scrutinee_ty,
            alts,
            default,
        } => {
            if let Some((known_name, known_tag)) = env.get(&scrutinee) {
                let matching_alt = alts
                    .iter()
                    .find(|a| &a.ctor_name == known_name && a.ctor_tag == u32::from(*known_tag));
                if let Some(alt) = matching_alt {
                    report.case_of_known_ctor_elims += 1;
                    return case_of_known_ctor(alt.body.clone(), env, report);
                } else if let Some(def) = default {
                    report.case_of_known_ctor_elims += 1;
                    return case_of_known_ctor(*def, env, report);
                }
            }
            let new_alts = alts
                .into_iter()
                .map(|alt| {
                    let new_body = case_of_known_ctor(alt.body, env, report);
                    crate::lcnf::LcnfAlt {
                        ctor_name: alt.ctor_name,
                        ctor_tag: alt.ctor_tag,
                        params: alt.params,
                        body: new_body,
                    }
                })
                .collect();
            let new_default = default.map(|d| Box::new(case_of_known_ctor(*d, env, report)));
            LcnfExpr::Case {
                scrutinee,
                scrutinee_ty,
                alts: new_alts,
                default: new_default,
            }
        }
        other => other,
    }
}
/// Flatten a let-chain into a vector of bindings plus a terminal expression.
#[allow(dead_code)]
pub fn flatten_let_chain(expr: &LcnfExpr) -> (Vec<LetBinding>, &LcnfExpr) {
    let mut bindings = Vec::new();
    let mut cur = expr;
    loop {
        match cur {
            LcnfExpr::Let {
                id,
                name,
                ty,
                value,
                body,
            } => {
                bindings.push(LetBinding {
                    id: *id,
                    name: name.clone(),
                    ty: ty.clone(),
                    value: value.clone(),
                });
                cur = body;
            }
            other => return (bindings, other),
        }
    }
}
/// Reconstruct an expression from a flattened let-chain and a terminal.
#[allow(dead_code)]
pub fn rebuild_let_chain(bindings: Vec<LetBinding>, terminal: LcnfExpr) -> LcnfExpr {
    bindings
        .into_iter()
        .rev()
        .fold(terminal, |body, b| LcnfExpr::Let {
            id: b.id,
            name: b.name,
            ty: b.ty,
            value: b.value,
            body: Box::new(body),
        })
}
/// Estimate the inline cost of an expression.
#[allow(dead_code)]
pub fn inline_cost(expr: &LcnfExpr) -> usize {
    match expr {
        LcnfExpr::Let { body, .. } => 1 + inline_cost(body),
        LcnfExpr::Case { alts, default, .. } => {
            let alt_cost: usize = alts.iter().map(|a| 1 + inline_cost(&a.body)).sum();
            let def_cost = default.as_ref().map(|d| inline_cost(d)).unwrap_or(0);
            1 + alt_cost + def_cost
        }
        LcnfExpr::TailCall(_, args) => args.len(),
        LcnfExpr::Return(_) => 0,
        LcnfExpr::Unreachable => 0,
    }
}
/// Substitute all occurrences of `from_id` with `to_arg` in `expr`.
#[allow(dead_code)]
pub fn subst_var_in_expr(expr: LcnfExpr, from_id: LcnfVarId, to_arg: &LcnfArg) -> LcnfExpr {
    match expr {
        LcnfExpr::Let {
            id,
            name,
            ty,
            mut value,
            body,
        } => {
            subst_var_in_value_mut(&mut value, from_id, to_arg);
            let new_body = if id == from_id {
                *body
            } else {
                subst_var_in_expr(*body, from_id, to_arg)
            };
            LcnfExpr::Let {
                id,
                name,
                ty,
                value,
                body: Box::new(new_body),
            }
        }
        LcnfExpr::Case {
            scrutinee,
            scrutinee_ty,
            alts,
            default,
        } => {
            let new_scrutinee = if scrutinee == from_id {
                if let LcnfArg::Var(new_id) = to_arg {
                    *new_id
                } else {
                    scrutinee
                }
            } else {
                scrutinee
            };
            let new_alts = alts
                .into_iter()
                .map(|alt| {
                    let bound_in_alt = alt.params.iter().any(|p| p.id == from_id);
                    let new_body = if bound_in_alt {
                        alt.body
                    } else {
                        subst_var_in_expr(alt.body, from_id, to_arg)
                    };
                    crate::lcnf::LcnfAlt {
                        ctor_name: alt.ctor_name,
                        ctor_tag: alt.ctor_tag,
                        params: alt.params,
                        body: new_body,
                    }
                })
                .collect();
            let new_default = default.map(|d| Box::new(subst_var_in_expr(*d, from_id, to_arg)));
            LcnfExpr::Case {
                scrutinee: new_scrutinee,
                scrutinee_ty,
                alts: new_alts,
                default: new_default,
            }
        }
        LcnfExpr::Return(mut arg) => {
            subst_var_in_arg_mut(&mut arg, from_id, to_arg);
            LcnfExpr::Return(arg)
        }
        LcnfExpr::TailCall(mut func, mut args) => {
            subst_var_in_arg_mut(&mut func, from_id, to_arg);
            for a in args.iter_mut() {
                subst_var_in_arg_mut(a, from_id, to_arg);
            }
            LcnfExpr::TailCall(func, args)
        }
        LcnfExpr::Unreachable => LcnfExpr::Unreachable,
    }
}
pub(super) fn subst_var_in_arg_mut(arg: &mut LcnfArg, from_id: LcnfVarId, to_arg: &LcnfArg) {
    if let LcnfArg::Var(id) = arg {
        if *id == from_id {
            *arg = to_arg.clone();
        }
    }
}
pub(super) fn subst_var_in_value_mut(
    value: &mut LcnfLetValue,
    from_id: LcnfVarId,
    to_arg: &LcnfArg,
) {
    match value {
        LcnfLetValue::App(func, args) => {
            subst_var_in_arg_mut(func, from_id, to_arg);
            for a in args.iter_mut() {
                subst_var_in_arg_mut(a, from_id, to_arg);
            }
        }
        LcnfLetValue::Ctor(_, _, args) | LcnfLetValue::Reuse(_, _, _, args) => {
            for a in args.iter_mut() {
                subst_var_in_arg_mut(a, from_id, to_arg);
            }
        }
        LcnfLetValue::FVar(id) => {
            if *id == from_id {
                if let LcnfArg::Var(new_id) = to_arg {
                    *id = *new_id;
                }
            }
        }
        LcnfLetValue::Proj(_, _, id) | LcnfLetValue::Reset(id) => {
            if *id == from_id {
                if let LcnfArg::Var(new_id) = to_arg {
                    *id = *new_id;
                }
            }
        }
        LcnfLetValue::Lit(_) | LcnfLetValue::Erased => {}
    }
}
/// Alpha-rename all bound variables in `expr` using `gen` to produce fresh IDs.
#[allow(dead_code)]
pub fn alpha_rename(expr: LcnfExpr, gen: &mut FreshIdGen) -> LcnfExpr {
    let mut rename_map: HashMap<LcnfVarId, LcnfVarId> = HashMap::new();
    alpha_rename_inner(expr, gen, &mut rename_map)
}
pub(super) fn alpha_rename_inner(
    expr: LcnfExpr,
    gen: &mut FreshIdGen,
    rename: &mut HashMap<LcnfVarId, LcnfVarId>,
) -> LcnfExpr {
    match expr {
        LcnfExpr::Let {
            id,
            name,
            ty,
            value,
            body,
        } => {
            let new_value = rename_in_value(value, rename);
            let new_id = gen.fresh();
            rename.insert(id, new_id);
            let new_body = alpha_rename_inner(*body, gen, rename);
            LcnfExpr::Let {
                id: new_id,
                name,
                ty,
                value: new_value,
                body: Box::new(new_body),
            }
        }
        LcnfExpr::Case {
            scrutinee,
            scrutinee_ty,
            alts,
            default,
        } => {
            let new_scrutinee = rename.get(&scrutinee).copied().unwrap_or(scrutinee);
            let new_alts = alts
                .into_iter()
                .map(|alt| {
                    let mut child_rename = rename.clone();
                    let new_params: Vec<crate::lcnf::LcnfParam> = alt
                        .params
                        .into_iter()
                        .map(|p| {
                            let new_pid = gen.fresh();
                            child_rename.insert(p.id, new_pid);
                            crate::lcnf::LcnfParam {
                                id: new_pid,
                                name: p.name,
                                ty: p.ty,
                                erased: p.erased,
                                borrowed: p.borrowed,
                            }
                        })
                        .collect();
                    let new_body = alpha_rename_inner(alt.body, gen, &mut child_rename);
                    crate::lcnf::LcnfAlt {
                        ctor_name: alt.ctor_name,
                        ctor_tag: alt.ctor_tag,
                        params: new_params,
                        body: new_body,
                    }
                })
                .collect();
            let new_default = default.map(|d| {
                let mut child_rename = rename.clone();
                Box::new(alpha_rename_inner(*d, gen, &mut child_rename))
            });
            LcnfExpr::Case {
                scrutinee: new_scrutinee,
                scrutinee_ty,
                alts: new_alts,
                default: new_default,
            }
        }
        LcnfExpr::Return(arg) => LcnfExpr::Return(rename_in_arg(arg, rename)),
        LcnfExpr::TailCall(func, args) => LcnfExpr::TailCall(
            rename_in_arg(func, rename),
            args.into_iter().map(|a| rename_in_arg(a, rename)).collect(),
        ),
        LcnfExpr::Unreachable => LcnfExpr::Unreachable,
    }
}
pub(super) fn rename_in_arg(arg: LcnfArg, rename: &HashMap<LcnfVarId, LcnfVarId>) -> LcnfArg {
    match arg {
        LcnfArg::Var(id) => LcnfArg::Var(rename.get(&id).copied().unwrap_or(id)),
        other => other,
    }
}
pub(super) fn rename_in_value(
    value: LcnfLetValue,
    rename: &HashMap<LcnfVarId, LcnfVarId>,
) -> LcnfLetValue {
    match value {
        LcnfLetValue::App(func, args) => LcnfLetValue::App(
            rename_in_arg(func, rename),
            args.into_iter().map(|a| rename_in_arg(a, rename)).collect(),
        ),
        LcnfLetValue::Ctor(name, tag, args) => LcnfLetValue::Ctor(
            name,
            tag,
            args.into_iter().map(|a| rename_in_arg(a, rename)).collect(),
        ),
        LcnfLetValue::Reuse(slot, name, tag, args) => LcnfLetValue::Reuse(
            slot,
            name,
            tag,
            args.into_iter().map(|a| rename_in_arg(a, rename)).collect(),
        ),
        LcnfLetValue::FVar(id) => LcnfLetValue::FVar(rename.get(&id).copied().unwrap_or(id)),
        LcnfLetValue::Proj(field, ty, id) => {
            LcnfLetValue::Proj(field, ty, rename.get(&id).copied().unwrap_or(id))
        }
        LcnfLetValue::Reset(id) => LcnfLetValue::Reset(rename.get(&id).copied().unwrap_or(id)),
        other => other,
    }
}
/// Full copy-propagation pass.
#[allow(dead_code)]
pub fn full_copy_propagation(expr: &mut LcnfExpr) -> usize {
    let mut env: HashMap<LcnfVarId, LcnfVarId> = HashMap::new();
    full_copy_prop_inner(expr, &mut env)
}
pub(super) fn full_copy_prop_inner(
    expr: &mut LcnfExpr,
    env: &mut HashMap<LcnfVarId, LcnfVarId>,
) -> usize {
    let mut count = 0;
    match expr {
        LcnfExpr::Let {
            id, value, body, ..
        } => {
            full_copy_prop_value(value, env);
            if let LcnfLetValue::FVar(src) = value {
                let canonical = resolve_chain(*src, env);
                env.insert(*id, canonical);
                count += 1;
            }
            count += full_copy_prop_inner(body, env);
        }
        LcnfExpr::Case {
            scrutinee,
            alts,
            default,
            ..
        } => {
            if let Some(c) = env.get(scrutinee) {
                *scrutinee = *c;
                count += 1;
            }
            for alt in alts.iter_mut() {
                let mut child_env = env.clone();
                count += full_copy_prop_inner(&mut alt.body, &mut child_env);
            }
            if let Some(def) = default {
                let mut child_env = env.clone();
                count += full_copy_prop_inner(def, &mut child_env);
            }
        }
        LcnfExpr::Return(arg) => {
            if let LcnfArg::Var(id) = arg {
                if let Some(c) = env.get(id) {
                    *id = *c;
                    count += 1;
                }
            }
        }
        LcnfExpr::TailCall(func, args) => {
            if let LcnfArg::Var(id) = func {
                if let Some(c) = env.get(id) {
                    *id = *c;
                    count += 1;
                }
            }
            for a in args.iter_mut() {
                if let LcnfArg::Var(id) = a {
                    if let Some(c) = env.get(id) {
                        *id = *c;
                        count += 1;
                    }
                }
            }
        }
        LcnfExpr::Unreachable => {}
    }
    count
}
pub(super) fn resolve_chain(id: LcnfVarId, env: &HashMap<LcnfVarId, LcnfVarId>) -> LcnfVarId {
    let mut cur = id;
    for _ in 0..64 {
        if let Some(&next) = env.get(&cur) {
            if next == cur {
                break;
            }
            cur = next;
        } else {
            break;
        }
    }
    cur
}
pub(super) fn full_copy_prop_value(value: &mut LcnfLetValue, env: &HashMap<LcnfVarId, LcnfVarId>) {
    match value {
        LcnfLetValue::App(func, args) => {
            if let LcnfArg::Var(id) = func {
                if let Some(c) = env.get(id) {
                    *id = *c;
                }
            }
            for a in args.iter_mut() {
                if let LcnfArg::Var(id) = a {
                    if let Some(c) = env.get(id) {
                        *id = *c;
                    }
                }
            }
        }
        LcnfLetValue::Ctor(_, _, args) | LcnfLetValue::Reuse(_, _, _, args) => {
            for a in args.iter_mut() {
                if let LcnfArg::Var(id) = a {
                    if let Some(c) = env.get(id) {
                        *id = *c;
                    }
                }
            }
        }
        LcnfLetValue::FVar(id) => {
            if let Some(c) = env.get(id) {
                *id = *c;
            }
        }
        LcnfLetValue::Proj(_, _, id) | LcnfLetValue::Reset(id) => {
            if let Some(c) = env.get(id) {
                *id = *c;
            }
        }
        LcnfLetValue::Lit(_) | LcnfLetValue::Erased => {}
    }
}
/// Propagate literal values through the expression.
#[allow(dead_code)]
pub fn lit_propagate(expr: &LcnfExpr, env: &mut LitEnv) {
    match expr {
        LcnfExpr::Let {
            id, value, body, ..
        } => {
            match value {
                LcnfLetValue::Lit(crate::lcnf::LcnfLit::Nat(n)) => {
                    env.record_nat(*id, *n);
                }
                LcnfLetValue::Lit(crate::lcnf::LcnfLit::Str(s)) => {
                    env.record_str(*id, s.clone());
                }
                _ => {}
            }
            lit_propagate(body, env);
        }
        LcnfExpr::Case { alts, default, .. } => {
            for alt in alts {
                let mut child_env = env.clone();
                lit_propagate(&alt.body, &mut child_env);
            }
            if let Some(def) = default {
                let mut child_env = env.clone();
                lit_propagate(def, &mut child_env);
            }
        }
        LcnfExpr::Return(_) | LcnfExpr::TailCall(_, _) | LcnfExpr::Unreachable => {}
    }
}
/// Compute the maximum nesting depth of an expression.
#[allow(dead_code)]
pub fn max_depth(expr: &LcnfExpr) -> usize {
    match expr {
        LcnfExpr::Let { body, .. } => 1 + max_depth(body),
        LcnfExpr::Case { alts, default, .. } => {
            let alt_max = alts
                .iter()
                .map(|a| 1 + max_depth(&a.body))
                .max()
                .unwrap_or(0);
            let def_max = default.as_ref().map(|d| max_depth(d)).unwrap_or(0);
            alt_max.max(def_max)
        }
        _ => 0,
    }
}
/// Count the total number of let-bindings in an expression.
#[allow(dead_code)]
pub fn count_lets(expr: &LcnfExpr) -> usize {
    match expr {
        LcnfExpr::Let { body, .. } => 1 + count_lets(body),
        LcnfExpr::Case { alts, default, .. } => {
            alts.iter().map(|a| count_lets(&a.body)).sum::<usize>()
                + default.as_ref().map(|d| count_lets(d)).unwrap_or(0)
        }
        _ => 0,
    }
}
/// Count the total number of case expressions in an expression.
#[allow(dead_code)]
pub fn count_cases(expr: &LcnfExpr) -> usize {
    match expr {
        LcnfExpr::Let { body, .. } => count_cases(body),
        LcnfExpr::Case { alts, default, .. } => {
            1 + alts.iter().map(|a| count_cases(&a.body)).sum::<usize>()
                + default.as_ref().map(|d| count_cases(d)).unwrap_or(0)
        }
        _ => 0,
    }
}
/// Count the total number of tail calls in an expression.
#[allow(dead_code)]
pub fn count_tail_calls(expr: &LcnfExpr) -> usize {
    match expr {
        LcnfExpr::Let { body, .. } => count_tail_calls(body),
        LcnfExpr::Case { alts, default, .. } => {
            alts.iter()
                .map(|a| count_tail_calls(&a.body))
                .sum::<usize>()
                + default.as_ref().map(|d| count_tail_calls(d)).unwrap_or(0)
        }
        LcnfExpr::TailCall(_, _) => 1,
        _ => 0,
    }
}
/// Collect all variables that are reachable from an expression.
#[allow(dead_code)]
pub fn collect_reachable(expr: &LcnfExpr, reachable: &mut std::collections::HashSet<LcnfVarId>) {
    match expr {
        LcnfExpr::Let { value, body, .. } => {
            collect_reachable_in_value(value, reachable);
            collect_reachable(body, reachable);
        }
        LcnfExpr::Case {
            scrutinee,
            alts,
            default,
            ..
        } => {
            reachable.insert(*scrutinee);
            for alt in alts {
                collect_reachable(&alt.body, reachable);
            }
            if let Some(def) = default {
                collect_reachable(def, reachable);
            }
        }
        LcnfExpr::Return(arg) => {
            if let LcnfArg::Var(id) = arg {
                reachable.insert(*id);
            }
        }
        LcnfExpr::TailCall(func, args) => {
            if let LcnfArg::Var(id) = func {
                reachable.insert(*id);
            }
            for a in args {
                if let LcnfArg::Var(id) = a {
                    reachable.insert(*id);
                }
            }
        }
        LcnfExpr::Unreachable => {}
    }
}
pub(super) fn collect_reachable_in_value(
    value: &LcnfLetValue,
    reachable: &mut std::collections::HashSet<LcnfVarId>,
) {
    match value {
        LcnfLetValue::App(func, args) => {
            if let LcnfArg::Var(id) = func {
                reachable.insert(*id);
            }
            for a in args {
                if let LcnfArg::Var(id) = a {
                    reachable.insert(*id);
                }
            }
        }
        LcnfLetValue::Ctor(_, _, args) | LcnfLetValue::Reuse(_, _, _, args) => {
            for a in args {
                if let LcnfArg::Var(id) = a {
                    reachable.insert(*id);
                }
            }
        }
        LcnfLetValue::FVar(id) | LcnfLetValue::Reset(id) => {
            reachable.insert(*id);
        }
        LcnfLetValue::Proj(_, _, id) => {
            reachable.insert(*id);
        }
        LcnfLetValue::Lit(_) | LcnfLetValue::Erased => {}
    }
}
/// Run dead-let elimination on a function declaration.
#[allow(dead_code)]
pub fn run_dead_let_elim(decl: &mut LcnfFunDecl) -> ExtendedPassReport {
    let mut report = ExtendedPassReport::default();
    let body = std::mem::replace(&mut decl.body, LcnfExpr::Unreachable);
    decl.body = dead_let_elim(body, &mut report);
    report
}
/// Run case-of-known-constructor elimination on a function declaration.
#[allow(dead_code)]
pub fn run_case_of_known_ctor(decl: &mut LcnfFunDecl) -> ExtendedPassReport {
    let mut report = ExtendedPassReport::default();
    let env = CtorEnv::new();
    let body = std::mem::replace(&mut decl.body, LcnfExpr::Unreachable);
    decl.body = case_of_known_ctor(body, &env, &mut report);
    report
}
/// A peephole rule: matches a pattern and returns a replacement, or `None`.
pub type PeepholeRule = fn(&LcnfLetValue) -> Option<LcnfLetValue>;
/// Run a list of peephole rules over all let-values in an expression.
#[allow(dead_code)]
pub fn peephole_pass(expr: &mut LcnfExpr, rules: &[PeepholeRule]) -> usize {
    let mut count = 0;
    match expr {
        LcnfExpr::Let { value, body, .. } => {
            for rule in rules {
                if let Some(new_val) = rule(value) {
                    *value = new_val;
                    count += 1;
                    break;
                }
            }
            count += peephole_pass(body, rules);
        }
        LcnfExpr::Case { alts, default, .. } => {
            for alt in alts.iter_mut() {
                count += peephole_pass(&mut alt.body, rules);
            }
            if let Some(def) = default {
                count += peephole_pass(def, rules);
            }
        }
        _ => {}
    }
    count
}
/// Peephole rule: `App(f, [])` -> `Erased`.
#[allow(dead_code)]
pub fn rule_nullary_app_to_erased(value: &LcnfLetValue) -> Option<LcnfLetValue> {
    if let LcnfLetValue::App(_, args) = value {
        if args.is_empty() {
            return Some(LcnfLetValue::Erased);
        }
    }
    None
}
/// Run all beta/eta + extended passes in a fixed-point loop.
#[allow(dead_code)]
pub fn run_optimizer(
    decl: &mut LcnfFunDecl,
    beta_cfg: BetaEtaConfig,
    ext_cfg: ExtendedPassConfig,
    max_iterations: usize,
) -> (BetaEtaReport, ExtendedPassReport) {
    let mut total_beta = BetaEtaReport::default();
    let mut total_ext = ExtendedPassReport::default();
    for _iter in 0..max_iterations {
        let before_lets = count_lets(&decl.body);
        let before_cases = count_cases(&decl.body);
        let mut beta_pass = BetaEtaPass::new(beta_cfg.clone());
        beta_pass.run(decl);
        total_beta.beta_reductions += beta_pass.report.beta_reductions;
        total_beta.eta_reductions += beta_pass.report.eta_reductions;
        total_beta.curried_opportunities += beta_pass.report.curried_opportunities;
        if ext_cfg.do_dead_let {
            let r = run_dead_let_elim(decl);
            total_ext.dead_lets_eliminated += r.dead_lets_eliminated;
        }
        if ext_cfg.do_case_of_known_ctor {
            let r = run_case_of_known_ctor(decl);
            total_ext.case_of_known_ctor_elims += r.case_of_known_ctor_elims;
        }
        if ext_cfg.do_let_float {
            let body = std::mem::replace(&mut decl.body, LcnfExpr::Unreachable);
            let mut r = ExtendedPassReport::default();
            decl.body = let_float(body, &mut r);
            total_ext.lets_floated += r.lets_floated;
        }
        let after_lets = count_lets(&decl.body);
        let after_cases = count_cases(&decl.body);
        if after_lets == before_lets && after_cases == before_cases {
            break;
        }
    }
    (total_beta, total_ext)
}
/// Compute which parameters of a function are used in its body.
#[allow(dead_code)]
pub fn param_usage_summary(decl: &LcnfFunDecl) -> ParamUsageSummary {
    let mut uses: HashMap<LcnfVarId, usize> = HashMap::new();
    count_uses(&decl.body, &mut uses);
    let used = decl
        .params
        .iter()
        .map(|p| uses.get(&p.id).copied().unwrap_or(0) > 0)
        .collect();
    ParamUsageSummary {
        func_name: decl.name.clone(),
        used,
    }
}
/// Collect optimization hints from a function declaration.
#[allow(dead_code)]
pub fn collect_hints(decl: &LcnfFunDecl) -> Vec<OptHint> {
    let mut hints = Vec::new();
    collect_hints_expr(&decl.body, &mut hints, &HashMap::new());
    let cost = inline_cost(&decl.body);
    if cost <= 5 {
        hints.push(OptHint::InlineCandidate {
            func_name: decl.name.clone(),
            cost,
        });
    }
    hints
}
pub(super) fn collect_hints_expr(
    expr: &LcnfExpr,
    hints: &mut Vec<OptHint>,
    id_to_name: &HashMap<LcnfVarId, String>,
) {
    match expr {
        LcnfExpr::Let {
            id,
            name,
            value: LcnfLetValue::App(func, args),
            body,
            ..
        } if args.len() == 1 => {
            if let LcnfExpr::Let {
                value: LcnfLetValue::App(LcnfArg::Var(callee), _),
                ..
            } = body.as_ref()
            {
                if callee == id {
                    let outer = if let LcnfArg::Var(fid) = func {
                        id_to_name
                            .get(fid)
                            .cloned()
                            .unwrap_or_else(|| format!("_x{}", fid.0))
                    } else {
                        "unknown".into()
                    };
                    hints.push(OptHint::MergeCurriedApp {
                        intermediate: *id,
                        outer_func: outer,
                    });
                }
            }
            let mut child_map = id_to_name.clone();
            child_map.insert(*id, name.clone());
            collect_hints_expr(body, hints, &child_map);
        }
        LcnfExpr::Let { id, name, body, .. } => {
            let mut child_map = id_to_name.clone();
            child_map.insert(*id, name.clone());
            collect_hints_expr(body, hints, &child_map);
        }
        LcnfExpr::Case { alts, default, .. } => {
            for alt in alts {
                collect_hints_expr(&alt.body, hints, id_to_name);
            }
            if let Some(def) = default {
                collect_hints_expr(def, hints, id_to_name);
            }
        }
        _ => {}
    }
}
/// Produce a compact human-readable representation of an `LcnfExpr`.
#[allow(dead_code)]
pub fn pp_expr(expr: &LcnfExpr) -> String {
    match expr {
        LcnfExpr::Let {
            id,
            name,
            value,
            body,
            ..
        } => {
            format!(
                "let {}:{} = {};\n{}",
                id,
                name,
                pp_value(value),
                pp_expr(body)
            )
        }
        LcnfExpr::Case {
            scrutinee,
            alts,
            default,
            ..
        } => {
            let mut s = format!("case {} of {{\n", scrutinee);
            for alt in alts {
                s.push_str(&format!(
                    "  | {} -> {}\n",
                    alt.ctor_name,
                    pp_expr(&alt.body)
                ));
            }
            if let Some(def) = default {
                s.push_str(&format!("  | _ -> {}\n", pp_expr(def)));
            }
            s.push('}');
            s
        }
        LcnfExpr::Return(arg) => format!("return {}", pp_arg(arg)),
        LcnfExpr::TailCall(func, args) => {
            let arg_strs: Vec<String> = args.iter().map(pp_arg).collect();
            format!("tailcall {}({})", pp_arg(func), arg_strs.join(", "))
        }
        LcnfExpr::Unreachable => "unreachable".into(),
    }
}
pub(super) fn pp_value(value: &LcnfLetValue) -> String {
    match value {
        LcnfLetValue::App(func, args) => {
            let arg_strs: Vec<String> = args.iter().map(pp_arg).collect();
            format!("App({}, [{}])", pp_arg(func), arg_strs.join(", "))
        }
        LcnfLetValue::Ctor(name, tag, args) => {
            let arg_strs: Vec<String> = args.iter().map(pp_arg).collect();
            format!("Ctor({}, {}, [{}])", name, tag, arg_strs.join(", "))
        }
        LcnfLetValue::Reuse(slot, name, tag, args) => {
            let arg_strs: Vec<String> = args.iter().map(pp_arg).collect();
            format!(
                "Reuse({}, {}, {}, [{}])",
                slot,
                name,
                tag,
                arg_strs.join(", ")
            )
        }
        LcnfLetValue::FVar(id) => format!("FVar({})", id),
        LcnfLetValue::Proj(field, ty, id) => format!("Proj({}, {}, {})", field, ty, id),
        LcnfLetValue::Reset(id) => format!("Reset({})", id),
        LcnfLetValue::Lit(lit) => format!("Lit({:?})", lit),
        LcnfLetValue::Erased => "Erased".into(),
    }
}
pub(super) fn pp_arg(arg: &LcnfArg) -> String {
    match arg {
        LcnfArg::Var(id) => format!("{}", id),
        LcnfArg::Type(ty) => format!("Type({})", ty),
        LcnfArg::Lit(lit) => format!("Lit({:?})", lit),
        LcnfArg::Erased => "Erased".to_string(),
    }
}
/// Run the full optimizer over every function in a module.
#[allow(dead_code)]
pub fn run_module_optimizer(
    decls: &mut Vec<LcnfFunDecl>,
    beta_cfg: BetaEtaConfig,
    ext_cfg: ExtendedPassConfig,
    max_iterations: usize,
) -> ModuleOptStats {
    let mut stats = ModuleOptStats::default();
    for decl in decls.iter_mut() {
        let (beta, ext) = run_optimizer(decl, beta_cfg.clone(), ext_cfg.clone(), max_iterations);
        stats.total_beta += beta.beta_reductions;
        stats.total_eta += beta.eta_reductions;
        stats.total_dead_lets += ext.dead_lets_eliminated;
        stats.total_cokc += ext.case_of_known_ctor_elims;
        stats.functions_processed += 1;
    }
    stats
}
/// Count trailing erased args in applications.
#[allow(dead_code)]
pub fn count_trailing_erased_args(expr: &LcnfExpr) -> usize {
    match expr {
        LcnfExpr::Let {
            value: LcnfLetValue::App(_, args),
            body,
            ..
        } => {
            let trailing = args
                .iter()
                .rev()
                .take_while(|a| matches!(a, LcnfArg::Type(_)))
                .count();
            trailing + count_trailing_erased_args(body)
        }
        LcnfExpr::Let { body, .. } => count_trailing_erased_args(body),
        LcnfExpr::Case { alts, default, .. } => {
            alts.iter()
                .map(|a| count_trailing_erased_args(&a.body))
                .sum::<usize>()
                + default
                    .as_ref()
                    .map(|d| count_trailing_erased_args(d))
                    .unwrap_or(0)
        }
        _ => 0,
    }
}
#[cfg(test)]
mod extended_tests {
    use super::*;
    use crate::lcnf::{
        LcnfArg, LcnfExpr, LcnfFunDecl, LcnfLetValue, LcnfParam, LcnfType, LcnfVarId,
    };
    pub(super) fn var(n: u64) -> LcnfVarId {
        LcnfVarId(n)
    }
    pub(super) fn param(id: u64, name: &str) -> LcnfParam {
        LcnfParam {
            id: var(id),
            name: name.to_string(),
            ty: LcnfType::Object,
            erased: false,
            borrowed: false,
        }
    }
    pub(super) fn make_decl(name: &str, params: Vec<LcnfParam>, body: LcnfExpr) -> LcnfFunDecl {
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
    #[test]
    pub(super) fn test_count_uses_simple() {
        let expr = LcnfExpr::Return(LcnfArg::Var(var(0)));
        let mut uses = HashMap::new();
        count_uses(&expr, &mut uses);
        assert_eq!(uses.get(&var(0)), Some(&1));
    }
    #[test]
    pub(super) fn test_dead_let_elim_removes_unused_fvar() {
        let body = LcnfExpr::Let {
            id: var(1),
            name: "x".into(),
            ty: LcnfType::Object,
            value: LcnfLetValue::FVar(var(0)),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(var(0)))),
        };
        let mut report = ExtendedPassReport::default();
        let result = dead_let_elim(body, &mut report);
        assert_eq!(report.dead_lets_eliminated, 1);
        assert!(matches!(result, LcnfExpr::Return(LcnfArg::Var(_))));
    }
    #[test]
    pub(super) fn test_dead_let_elim_keeps_used_binding() {
        let body = LcnfExpr::Let {
            id: var(1),
            name: "x".into(),
            ty: LcnfType::Object,
            value: LcnfLetValue::FVar(var(0)),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(var(1)))),
        };
        let mut report = ExtendedPassReport::default();
        let _result = dead_let_elim(body, &mut report);
        assert_eq!(report.dead_lets_eliminated, 0);
    }
    #[test]
    pub(super) fn test_inline_cost_single_let() {
        let body = LcnfExpr::Let {
            id: var(0),
            name: "x".into(),
            ty: LcnfType::Object,
            value: LcnfLetValue::Erased,
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(var(0)))),
        };
        assert_eq!(inline_cost(&body), 1);
    }
    #[test]
    pub(super) fn test_flatten_let_chain() {
        let expr = LcnfExpr::Let {
            id: var(0),
            name: "a".into(),
            ty: LcnfType::Object,
            value: LcnfLetValue::Erased,
            body: Box::new(LcnfExpr::Let {
                id: var(1),
                name: "b".into(),
                ty: LcnfType::Object,
                value: LcnfLetValue::Erased,
                body: Box::new(LcnfExpr::Return(LcnfArg::Var(var(1)))),
            }),
        };
        let (bindings, terminal) = flatten_let_chain(&expr);
        assert_eq!(bindings.len(), 2);
        assert!(matches!(terminal, LcnfExpr::Return(_)));
    }
    #[test]
    pub(super) fn test_rebuild_let_chain_roundtrip() {
        let original = LcnfExpr::Let {
            id: var(0),
            name: "a".into(),
            ty: LcnfType::Object,
            value: LcnfLetValue::Erased,
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(var(0)))),
        };
        let (bindings, terminal) = flatten_let_chain(&original);
        let rebuilt = rebuild_let_chain(bindings, terminal.clone());
        assert!(matches!(rebuilt, LcnfExpr::Let { id, .. } if id == var(0)));
    }
    #[test]
    pub(super) fn test_fresh_id_gen_sequential() {
        let mut gen = FreshIdGen::new(100);
        assert_eq!(gen.fresh(), LcnfVarId(100));
        assert_eq!(gen.fresh(), LcnfVarId(101));
        assert_eq!(gen.fresh(), LcnfVarId(102));
    }
    #[test]
    pub(super) fn test_alpha_rename_changes_ids() {
        let expr = LcnfExpr::Let {
            id: var(0),
            name: "x".into(),
            ty: LcnfType::Object,
            value: LcnfLetValue::Erased,
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(var(0)))),
        };
        let mut gen = FreshIdGen::new(1000);
        let renamed = alpha_rename(expr, &mut gen);
        if let LcnfExpr::Let { id, body, .. } = &renamed {
            assert_eq!(*id, LcnfVarId(1000));
            if let LcnfExpr::Return(LcnfArg::Var(ret_id)) = body.as_ref() {
                assert_eq!(*ret_id, LcnfVarId(1000));
            } else {
                panic!("expected Return(Var(1000))");
            }
        } else {
            panic!("expected Let");
        }
    }
    #[test]
    pub(super) fn test_subst_var_in_expr_replaces_return() {
        let expr = LcnfExpr::Return(LcnfArg::Var(var(0)));
        let result = subst_var_in_expr(expr, var(0), &LcnfArg::Var(var(99)));
        assert!(matches!(result, LcnfExpr::Return(LcnfArg::Var(id)) if id == var(99)));
    }
    #[test]
    pub(super) fn test_subst_var_no_change_wrong_id() {
        let expr = LcnfExpr::Return(LcnfArg::Var(var(5)));
        let result = subst_var_in_expr(expr, var(0), &LcnfArg::Var(var(99)));
        assert!(matches!(result, LcnfExpr::Return(LcnfArg::Var(id)) if id == var(5)));
    }
    #[test]
    pub(super) fn test_max_depth_nested_lets() {
        let expr = LcnfExpr::Let {
            id: var(0),
            name: "a".into(),
            ty: LcnfType::Object,
            value: LcnfLetValue::Erased,
            body: Box::new(LcnfExpr::Let {
                id: var(1),
                name: "b".into(),
                ty: LcnfType::Object,
                value: LcnfLetValue::Erased,
                body: Box::new(LcnfExpr::Return(LcnfArg::Var(var(1)))),
            }),
        };
        assert_eq!(max_depth(&expr), 2);
    }
    #[test]
    pub(super) fn test_count_lets() {
        let expr = LcnfExpr::Let {
            id: var(0),
            name: "a".into(),
            ty: LcnfType::Object,
            value: LcnfLetValue::Erased,
            body: Box::new(LcnfExpr::Let {
                id: var(1),
                name: "b".into(),
                ty: LcnfType::Object,
                value: LcnfLetValue::Erased,
                body: Box::new(LcnfExpr::Return(LcnfArg::Var(var(1)))),
            }),
        };
        assert_eq!(count_lets(&expr), 2);
    }
    #[test]
    pub(super) fn test_count_cases_zero() {
        let expr = LcnfExpr::Return(LcnfArg::Var(var(0)));
        assert_eq!(count_cases(&expr), 0);
    }
    #[test]
    pub(super) fn test_collect_reachable() {
        let expr = LcnfExpr::Return(LcnfArg::Var(var(42)));
        let mut reachable = std::collections::HashSet::new();
        collect_reachable(&expr, &mut reachable);
        assert!(reachable.contains(&var(42)));
    }
    #[test]
    pub(super) fn test_ctor_env_record_and_get() {
        let mut env = CtorEnv::new();
        env.record(var(5), "Cons".into(), 1);
        assert_eq!(env.get(&var(5)), Some(&("Cons".into(), 1u16)));
        assert_eq!(env.get(&var(6)), None);
    }
    #[test]
    pub(super) fn test_case_of_known_ctor_eliminates() {
        let body = LcnfExpr::Let {
            id: var(1),
            name: "x".into(),
            ty: LcnfType::Object,
            value: LcnfLetValue::Ctor("True".into(), 0, vec![]),
            body: Box::new(LcnfExpr::Case {
                scrutinee: var(1),
                scrutinee_ty: LcnfType::Object,
                alts: vec![
                    crate::lcnf::LcnfAlt {
                        ctor_name: "True".into(),
                        ctor_tag: 0,
                        params: vec![],
                        body: LcnfExpr::Return(LcnfArg::Var(var(0))),
                    },
                    crate::lcnf::LcnfAlt {
                        ctor_name: "False".into(),
                        ctor_tag: 1,
                        params: vec![],
                        body: LcnfExpr::Unreachable,
                    },
                ],
                default: None,
            }),
        };
        let env = CtorEnv::new();
        let mut report = ExtendedPassReport::default();
        let _result = case_of_known_ctor(body, &env, &mut report);
        assert_eq!(report.case_of_known_ctor_elims, 1);
    }
    #[test]
    pub(super) fn test_full_copy_propagation() {
        let mut expr = LcnfExpr::Let {
            id: var(1),
            name: "x".into(),
            ty: LcnfType::Object,
            value: LcnfLetValue::FVar(var(0)),
            body: Box::new(LcnfExpr::Let {
                id: var(2),
                name: "y".into(),
                ty: LcnfType::Object,
                value: LcnfLetValue::FVar(var(1)),
                body: Box::new(LcnfExpr::Return(LcnfArg::Var(var(2)))),
            }),
        };
        let propagated = full_copy_propagation(&mut expr);
        assert!(propagated > 0);
    }
    #[test]
    pub(super) fn test_run_optimizer_no_panic() {
        let body = LcnfExpr::Return(LcnfArg::Var(var(0)));
        let mut decl = make_decl("simple", vec![param(0, "x")], body);
        let (beta, ext) = run_optimizer(
            &mut decl,
            BetaEtaConfig::default(),
            ExtendedPassConfig::default(),
            3,
        );
        let _ = (beta, ext);
    }
    #[test]
    pub(super) fn test_lit_env_record_and_get() {
        let mut env = LitEnv::new();
        env.record_nat(var(0), 42);
        env.record_str(var(1), "hello".into());
        assert_eq!(env.get(&var(0)), Some(&KnownValue::Nat(42)));
        assert_eq!(env.get(&var(1)), Some(&KnownValue::Str("hello".into())));
        assert_eq!(env.get(&var(2)), None);
    }
    #[test]
    pub(super) fn test_peephole_nullary_app_to_erased() {
        let mut expr = LcnfExpr::Let {
            id: var(0),
            name: "x".into(),
            ty: LcnfType::Object,
            value: LcnfLetValue::App(LcnfArg::Var(var(99)), vec![]),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(var(0)))),
        };
        let rules: Vec<PeepholeRule> = vec![rule_nullary_app_to_erased];
        let n = peephole_pass(&mut expr, &rules);
        assert_eq!(n, 1);
        if let LcnfExpr::Let { value, .. } = &expr {
            assert!(matches!(value, LcnfLetValue::Erased));
        }
    }
    #[test]
    pub(super) fn test_run_dead_let_elim_wrapper() {
        let body = LcnfExpr::Let {
            id: var(1),
            name: "unused".into(),
            ty: LcnfType::Object,
            value: LcnfLetValue::FVar(var(0)),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(var(0)))),
        };
        let mut decl = make_decl("f", vec![param(0, "x")], body);
        let report = run_dead_let_elim(&mut decl);
        assert_eq!(report.dead_lets_eliminated, 1);
    }
    #[test]
    pub(super) fn test_count_tail_calls_in_case() {
        let expr = LcnfExpr::Case {
            scrutinee: var(0),
            scrutinee_ty: LcnfType::Object,
            alts: vec![
                crate::lcnf::LcnfAlt {
                    ctor_name: "A".into(),
                    ctor_tag: 0,
                    params: vec![],
                    body: LcnfExpr::TailCall(LcnfArg::Var(var(1)), vec![]),
                },
                crate::lcnf::LcnfAlt {
                    ctor_name: "B".into(),
                    ctor_tag: 1,
                    params: vec![],
                    body: LcnfExpr::TailCall(LcnfArg::Var(var(2)), vec![]),
                },
            ],
            default: None,
        };
        assert_eq!(count_tail_calls(&expr), 2);
    }
    #[test]
    pub(super) fn test_is_pure_value_lit() {
        assert!(is_pure_value(&LcnfLetValue::Lit(
            crate::lcnf::LcnfLit::Nat(0)
        )));
    }
    #[test]
    pub(super) fn test_is_pure_value_app_not_pure() {
        let app = LcnfLetValue::App(LcnfArg::Var(var(0)), vec![]);
        assert!(!is_pure_value(&app));
    }
    #[test]
    pub(super) fn test_arity_map_from_decls() {
        let decls = vec![
            make_decl(
                "f",
                vec![param(0, "x"), param(1, "y")],
                LcnfExpr::Return(LcnfArg::Var(var(0))),
            ),
            make_decl(
                "g",
                vec![param(2, "z")],
                LcnfExpr::Return(LcnfArg::Var(var(2))),
            ),
        ];
        let am = ArityMap::from_decls(&decls);
        assert_eq!(am.get("f"), Some(2));
        assert_eq!(am.get("g"), Some(1));
        assert_eq!(am.get("unknown"), None);
    }
    #[test]
    pub(super) fn test_param_usage_summary_used() {
        let decl = make_decl(
            "f",
            vec![param(0, "x"), param(1, "y")],
            LcnfExpr::Return(LcnfArg::Var(var(0))),
        );
        let summary = param_usage_summary(&decl);
        assert_eq!(summary.used, vec![true, false]);
    }
    #[test]
    pub(super) fn test_collect_hints_inline_candidate() {
        let decl = make_decl(
            "tiny",
            vec![param(0, "x")],
            LcnfExpr::Return(LcnfArg::Var(var(0))),
        );
        let hints = collect_hints(&decl);
        assert!(hints
            .iter()
            .any(|h| matches!(h, OptHint::InlineCandidate { .. })));
    }
    #[test]
    pub(super) fn test_pp_expr_return() {
        let expr = LcnfExpr::Return(LcnfArg::Var(var(0)));
        let s = pp_expr(&expr);
        assert!(s.contains("return"));
    }
    #[test]
    pub(super) fn test_pp_expr_tailcall() {
        let expr = LcnfExpr::TailCall(LcnfArg::Var(var(1)), vec![LcnfArg::Var(var(0))]);
        let s = pp_expr(&expr);
        assert!(s.contains("tailcall"));
    }
    #[test]
    pub(super) fn test_pp_expr_unreachable() {
        let expr = LcnfExpr::Unreachable;
        assert_eq!(pp_expr(&expr), "unreachable");
    }
    #[test]
    pub(super) fn test_module_optimizer_no_panic() {
        let mut decls = vec![
            make_decl(
                "f",
                vec![param(0, "x")],
                LcnfExpr::Return(LcnfArg::Var(var(0))),
            ),
            make_decl(
                "g",
                vec![param(1, "y")],
                LcnfExpr::Return(LcnfArg::Var(var(1))),
            ),
        ];
        let stats = run_module_optimizer(
            &mut decls,
            BetaEtaConfig::default(),
            ExtendedPassConfig::default(),
            3,
        );
        assert_eq!(stats.functions_processed, 2);
    }
    #[test]
    pub(super) fn test_count_trailing_erased_args_zero() {
        let expr = LcnfExpr::Return(LcnfArg::Var(var(0)));
        assert_eq!(count_trailing_erased_args(&expr), 0);
    }
    #[test]
    pub(super) fn test_extended_config_defaults() {
        let cfg = ExtendedPassConfig::default();
        assert!(cfg.do_let_float);
        assert!(cfg.do_case_of_case);
        assert!(cfg.do_dead_let);
        assert_eq!(cfg.max_case_of_case, 8);
    }
    #[test]
    pub(super) fn test_beta_eta_config_defaults() {
        let cfg = BetaEtaConfig::default();
        assert!(cfg.do_eta);
        assert!(cfg.do_beta);
        assert_eq!(cfg.max_depth, 256);
    }
}
