//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::HashMap;

use super::types::{
    ConstantFoldReport, CopyProp, CopyPropConfig, DeadBindingElim, DeadBindingReport, InlineConfig,
    InlineReport, InliningPass, InterferenceGraph, OptPipeline, PassKind, RegisterCoalescingHint,
    UsedVars,
};

/// Collects register coalescing hints from a copy propagation pass.
///
/// For each copy `let x = y`, if `x` and `y` do not interfere in the
/// interference graph, emit a hint to coalesce them.
#[allow(dead_code)]
pub fn collect_coalescing_hints(
    copies: &[(LcnfVarId, LcnfVarId)],
    ig: &InterferenceGraph,
) -> Vec<RegisterCoalescingHint> {
    let mut hints = Vec::new();
    for &(src, dst) in copies {
        let is_safe = !ig.interfere(src, dst);
        let benefit = if is_safe { 10 } else { 1 };
        hints.push(RegisterCoalescingHint::new(src, dst, is_safe, benefit));
    }
    hints.sort_by_key(|b| std::cmp::Reverse(b.benefit));
    hints
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::lcnf::{
        LcnfAlt, LcnfExpr, LcnfFunDecl, LcnfLetValue, LcnfLit, LcnfParam, LcnfType, LcnfVarId,
    };
    pub(super) fn make_decl(body: LcnfExpr) -> LcnfFunDecl {
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
    /// `let x = fvar(y); return x`  →  `return y`
    #[test]
    pub(super) fn test_simple_fvar_copy() {
        let body = LcnfExpr::Let {
            id: LcnfVarId(1),
            name: "x".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::FVar(LcnfVarId(0)),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(1)))),
        };
        let mut decl = make_decl(body);
        let mut pass = CopyProp::new(CopyPropConfig::default());
        pass.run(&mut decl);
        assert_eq!(decl.body, LcnfExpr::Return(LcnfArg::Var(LcnfVarId(0))));
        assert_eq!(pass.report().copies_eliminated, 1);
    }
    /// `let x = 42; return x`  →  `return 42`  (fold_literals=true)
    #[test]
    pub(super) fn test_literal_fold_enabled() {
        let body = LcnfExpr::Let {
            id: LcnfVarId(0),
            name: "x".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Lit(LcnfLit::Nat(42)),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(0)))),
        };
        let mut decl = make_decl(body);
        let mut pass = CopyProp::new(CopyPropConfig {
            fold_literals: true,
            ..Default::default()
        });
        pass.run(&mut decl);
        assert_eq!(decl.body, LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(42))));
        assert_eq!(pass.report().copies_eliminated, 1);
    }
    /// `let x = 42; return x`  stays when `fold_literals=false`.
    #[test]
    pub(super) fn test_literal_fold_disabled() {
        let body = LcnfExpr::Let {
            id: LcnfVarId(0),
            name: "x".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Lit(LcnfLit::Nat(7)),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(0)))),
        };
        let mut decl = make_decl(body);
        let mut pass = CopyProp::new(CopyPropConfig {
            fold_literals: false,
            ..Default::default()
        });
        pass.run(&mut decl);
        assert!(matches!(decl.body, LcnfExpr::Let { .. }));
        assert_eq!(pass.report().copies_eliminated, 0);
    }
    /// Transitive chain: `a=b, b=c, return a`  →  `return c`  (2 hops)
    #[test]
    pub(super) fn test_transitive_chain() {
        let body = LcnfExpr::Let {
            id: LcnfVarId(1),
            name: "b".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::FVar(LcnfVarId(0)),
            body: Box::new(LcnfExpr::Let {
                id: LcnfVarId(2),
                name: "a".to_string(),
                ty: LcnfType::Nat,
                value: LcnfLetValue::FVar(LcnfVarId(1)),
                body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(2)))),
            }),
        };
        let mut decl = make_decl(body);
        let mut pass = CopyProp::new(CopyPropConfig::default());
        pass.run(&mut decl);
        assert_eq!(decl.body, LcnfExpr::Return(LcnfArg::Var(LcnfVarId(0))));
        assert_eq!(pass.report().copies_eliminated, 2);
        assert_eq!(pass.report().chains_followed, 1);
    }
    /// Chain depth limit: with max_chain_depth=1 the second hop is not followed.
    #[test]
    pub(super) fn test_chain_depth_limit() {
        let body = LcnfExpr::Let {
            id: LcnfVarId(1),
            name: "b".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::FVar(LcnfVarId(0)),
            body: Box::new(LcnfExpr::Let {
                id: LcnfVarId(2),
                name: "a".to_string(),
                ty: LcnfType::Nat,
                value: LcnfLetValue::FVar(LcnfVarId(1)),
                body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(2)))),
            }),
        };
        let mut decl = make_decl(body);
        let mut pass = CopyProp::new(CopyPropConfig {
            max_chain_depth: 1,
            fold_literals: true,
        });
        pass.run(&mut decl);
        assert_eq!(decl.body, LcnfExpr::Return(LcnfArg::Var(LcnfVarId(0))));
    }
    /// App bindings are NOT propagated (conservative aliasing).
    #[test]
    pub(super) fn test_app_not_propagated() {
        let body = LcnfExpr::Let {
            id: LcnfVarId(1),
            name: "r".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::App(LcnfArg::Var(LcnfVarId(0)), vec![]),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(1)))),
        };
        let mut decl = make_decl(body);
        let mut pass = CopyProp::default_pass();
        pass.run(&mut decl);
        assert!(matches!(decl.body, LcnfExpr::Let { .. }));
        assert_eq!(pass.report().copies_eliminated, 0);
    }
    /// Copy propagation inside case branches is independent per branch.
    #[test]
    pub(super) fn test_copy_in_case_branches() {
        let case_expr = LcnfExpr::Case {
            scrutinee: LcnfVarId(0),
            scrutinee_ty: LcnfType::Object,
            alts: vec![LcnfAlt {
                ctor_name: "A".to_string(),
                ctor_tag: 0,
                params: vec![LcnfParam {
                    id: LcnfVarId(1),
                    name: "p".to_string(),
                    ty: LcnfType::Nat,
                    erased: false,
                    borrowed: false,
                }],
                body: LcnfExpr::Let {
                    id: LcnfVarId(2),
                    name: "q".to_string(),
                    ty: LcnfType::Nat,
                    value: LcnfLetValue::FVar(LcnfVarId(1)),
                    body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(2)))),
                },
            }],
            default: Some(Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(0))))),
        };
        let mut decl = make_decl(case_expr);
        let mut pass = CopyProp::default_pass();
        pass.run(&mut decl);
        match &decl.body {
            LcnfExpr::Case { alts, .. } => {
                assert_eq!(alts.len(), 1);
                assert_eq!(alts[0].body, LcnfExpr::Return(LcnfArg::Var(LcnfVarId(1))));
            }
            _ => panic!("Expected Case"),
        }
        assert!(pass.report().copies_eliminated >= 1);
    }
    /// Erased bindings are treated as copies and propagated.
    #[test]
    pub(super) fn test_erased_copy_propagated() {
        let body = LcnfExpr::Let {
            id: LcnfVarId(0),
            name: "e".to_string(),
            ty: LcnfType::Erased,
            value: LcnfLetValue::Erased,
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(0)))),
        };
        let mut decl = make_decl(body);
        let mut pass = CopyProp::default_pass();
        pass.run(&mut decl);
        assert_eq!(decl.body, LcnfExpr::Return(LcnfArg::Erased));
        assert_eq!(pass.report().copies_eliminated, 1);
    }
}
#[allow(dead_code)]
pub(super) fn collect_used(expr: &LcnfExpr, used: &mut UsedVars) {
    match expr {
        LcnfExpr::Return(arg) => collect_used_arg(arg, used),
        LcnfExpr::Let { value, body, .. } => {
            collect_used_value(value, used);
            collect_used(body, used);
        }
        LcnfExpr::Case {
            scrutinee,
            scrutinee_ty: _,
            alts,
            default,
            ..
        } => {
            used.vars.insert(*scrutinee);
            for alt in alts {
                collect_used(&alt.body, used);
            }
            if let Some(d) = default {
                collect_used(d, used);
            }
        }
        LcnfExpr::TailCall(callee, args) => {
            collect_used_arg(callee, used);
            for a in args {
                collect_used_arg(a, used);
            }
        }
        LcnfExpr::Unreachable => {}
    }
}
#[allow(dead_code)]
pub(super) fn collect_used_arg(arg: &LcnfArg, used: &mut UsedVars) {
    if let LcnfArg::Var(id) = arg {
        used.vars.insert(*id);
    }
}
#[allow(dead_code)]
pub(super) fn collect_used_value(val: &LcnfLetValue, used: &mut UsedVars) {
    match val {
        LcnfLetValue::FVar(id) => {
            used.vars.insert(*id);
        }
        LcnfLetValue::App(callee, args) => {
            collect_used_arg(callee, used);
            for a in args {
                collect_used_arg(a, used);
            }
        }
        LcnfLetValue::Ctor(_, _, args) => {
            for a in args {
                collect_used_arg(a, used);
            }
        }
        LcnfLetValue::Proj(_, _, id) => {
            used.vars.insert(*id);
        }
        _ => {}
    }
}
/// Inlining cost threshold (max `inline_cost` to be inlined).
#[allow(dead_code)]
pub const DEFAULT_INLINE_THRESHOLD: u32 = 5;
/// Count the number of `let`-bindings in an expression tree.
#[allow(dead_code)]
pub fn count_let_bindings(expr: &LcnfExpr) -> usize {
    match expr {
        LcnfExpr::Let { body, .. } => 1 + count_let_bindings(body),
        LcnfExpr::Case { alts, default, .. } => {
            let alt_sum: usize = alts.iter().map(|a| count_let_bindings(&a.body)).sum();
            let def_sum = default.as_ref().map_or(0, |d| count_let_bindings(d));
            alt_sum + def_sum
        }
        _ => 0,
    }
}
/// Return the maximum nesting depth of a LCNF expression.
#[allow(dead_code)]
pub fn expr_depth(expr: &LcnfExpr) -> usize {
    match expr {
        LcnfExpr::Let { body, .. } => 1 + expr_depth(body),
        LcnfExpr::Case { alts, default, .. } => {
            let alt_max = alts.iter().map(|a| expr_depth(&a.body)).max().unwrap_or(0);
            let def_max = default.as_ref().map_or(0, |d| expr_depth(d));
            1 + alt_max.max(def_max)
        }
        LcnfExpr::TailCall(_, args) => args.len(),
        _ => 0,
    }
}
/// Check whether an expression contains any tail calls to the given id.
#[allow(dead_code)]
pub fn has_tail_call_to(expr: &LcnfExpr, target: LcnfVarId) -> bool {
    match expr {
        LcnfExpr::TailCall(LcnfArg::Var(id), _) => *id == target,
        LcnfExpr::Let { body, .. } => has_tail_call_to(body, target),
        LcnfExpr::Case { alts, default, .. } => {
            alts.iter().any(|a| has_tail_call_to(&a.body, target))
                || default
                    .as_ref()
                    .is_some_and(|d| has_tail_call_to(d, target))
        }
        _ => false,
    }
}
/// Collect all `LcnfVarId`s bound by `let` in an expression.
#[allow(dead_code)]
pub fn collect_bound_vars(expr: &LcnfExpr, out: &mut Vec<LcnfVarId>) {
    match expr {
        LcnfExpr::Let { id, body, .. } => {
            out.push(*id);
            collect_bound_vars(body, out);
        }
        LcnfExpr::Case { alts, default, .. } => {
            for alt in alts {
                collect_bound_vars(&alt.body, out);
            }
            if let Some(d) = default {
                collect_bound_vars(d, out);
            }
        }
        _ => {}
    }
}

/// Find the maximum `LcnfVarId` numeric value in an expression (including bound and free vars).
pub(super) fn max_var_id_in_expr(expr: &LcnfExpr) -> u64 {
    match expr {
        LcnfExpr::Let {
            id, value, body, ..
        } => {
            let mut m = id.0;
            m = m.max(max_var_id_in_let_value(value));
            m = m.max(max_var_id_in_expr(body));
            m
        }
        LcnfExpr::Case {
            scrutinee,
            alts,
            default,
            ..
        } => {
            let mut m = scrutinee.0;
            for alt in alts {
                for p in &alt.params {
                    m = m.max(p.id.0);
                }
                m = m.max(max_var_id_in_expr(&alt.body));
            }
            if let Some(d) = default {
                m = m.max(max_var_id_in_expr(d));
            }
            m
        }
        LcnfExpr::Return(arg) => max_var_id_in_arg(arg),
        LcnfExpr::TailCall(func, args) => {
            let mut m = max_var_id_in_arg(func);
            for a in args {
                m = m.max(max_var_id_in_arg(a));
            }
            m
        }
        LcnfExpr::Unreachable => 0,
    }
}

pub(super) fn max_var_id_in_arg(arg: &LcnfArg) -> u64 {
    if let LcnfArg::Var(id) = arg {
        id.0
    } else {
        0
    }
}

pub(super) fn max_var_id_in_let_value(val: &LcnfLetValue) -> u64 {
    match val {
        LcnfLetValue::App(func, args) => {
            let mut m = max_var_id_in_arg(func);
            for a in args {
                m = m.max(max_var_id_in_arg(a));
            }
            m
        }
        LcnfLetValue::Ctor(_, _, args) | LcnfLetValue::Reuse(_, _, _, args) => {
            args.iter().map(max_var_id_in_arg).max().unwrap_or(0)
        }
        LcnfLetValue::Proj(_, _, id) | LcnfLetValue::Reset(id) | LcnfLetValue::FVar(id) => id.0,
        LcnfLetValue::Lit(_) | LcnfLetValue::Erased => 0,
    }
}

/// Offset all `LcnfVarId`s in `expr` by `offset`, avoiding ID collisions when inlining.
pub(super) fn offset_var_ids(expr: LcnfExpr, offset: u64) -> LcnfExpr {
    match expr {
        LcnfExpr::Let {
            id,
            name,
            ty,
            value,
            body,
        } => LcnfExpr::Let {
            id: LcnfVarId(id.0 + offset),
            name,
            ty,
            value: offset_var_ids_in_let_value(value, offset),
            body: Box::new(offset_var_ids(*body, offset)),
        },
        LcnfExpr::Case {
            scrutinee,
            scrutinee_ty,
            alts,
            default,
        } => LcnfExpr::Case {
            scrutinee: LcnfVarId(scrutinee.0 + offset),
            scrutinee_ty,
            alts: alts
                .into_iter()
                .map(|alt| LcnfAlt {
                    ctor_name: alt.ctor_name,
                    ctor_tag: alt.ctor_tag,
                    params: alt
                        .params
                        .into_iter()
                        .map(|p| LcnfParam {
                            id: LcnfVarId(p.id.0 + offset),
                            ..p
                        })
                        .collect(),
                    body: offset_var_ids(alt.body, offset),
                })
                .collect(),
            default: default.map(|d| Box::new(offset_var_ids(*d, offset))),
        },
        LcnfExpr::Return(arg) => LcnfExpr::Return(offset_var_ids_in_arg(arg, offset)),
        LcnfExpr::TailCall(func, args) => LcnfExpr::TailCall(
            offset_var_ids_in_arg(func, offset),
            args.into_iter()
                .map(|a| offset_var_ids_in_arg(a, offset))
                .collect(),
        ),
        LcnfExpr::Unreachable => LcnfExpr::Unreachable,
    }
}

pub(super) fn offset_var_ids_in_arg(arg: LcnfArg, offset: u64) -> LcnfArg {
    match arg {
        LcnfArg::Var(id) => LcnfArg::Var(LcnfVarId(id.0 + offset)),
        other => other,
    }
}

pub(super) fn offset_var_ids_in_let_value(val: LcnfLetValue, offset: u64) -> LcnfLetValue {
    match val {
        LcnfLetValue::App(func, args) => LcnfLetValue::App(
            offset_var_ids_in_arg(func, offset),
            args.into_iter()
                .map(|a| offset_var_ids_in_arg(a, offset))
                .collect(),
        ),
        LcnfLetValue::Ctor(name, tag, args) => LcnfLetValue::Ctor(
            name,
            tag,
            args.into_iter()
                .map(|a| offset_var_ids_in_arg(a, offset))
                .collect(),
        ),
        LcnfLetValue::Reuse(slot, name, tag, args) => LcnfLetValue::Reuse(
            LcnfVarId(slot.0 + offset),
            name,
            tag,
            args.into_iter()
                .map(|a| offset_var_ids_in_arg(a, offset))
                .collect(),
        ),
        LcnfLetValue::Proj(name, idx, var) => {
            LcnfLetValue::Proj(name, idx, LcnfVarId(var.0 + offset))
        }
        LcnfLetValue::Reset(var) => LcnfLetValue::Reset(LcnfVarId(var.0 + offset)),
        LcnfLetValue::FVar(id) => LcnfLetValue::FVar(LcnfVarId(id.0 + offset)),
        other @ (LcnfLetValue::Lit(_) | LcnfLetValue::Erased) => other,
    }
}

/// Substitute `params` for `args` in `body`, using `LcnfVarId` keys.
///
/// Builds a `HashMap<LcnfVarId, LcnfArg>` from `params[i].id → args[i]` and
/// walks the body replacing every `LcnfArg::Var(id)` found in the map.
pub(super) fn inline_substitute_params(
    body: &LcnfExpr,
    params: &[LcnfParam],
    args: &[LcnfArg],
) -> LcnfExpr {
    let subst: HashMap<LcnfVarId, LcnfArg> = params
        .iter()
        .zip(args.iter())
        .map(|(p, a)| (p.id, a.clone()))
        .collect();
    inline_subst_expr(body, &subst)
}

pub(super) fn inline_subst_expr(expr: &LcnfExpr, subst: &HashMap<LcnfVarId, LcnfArg>) -> LcnfExpr {
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
            ty: ty.clone(),
            value: inline_subst_let_value(value, subst),
            body: Box::new(inline_subst_expr(body, subst)),
        },
        LcnfExpr::Case {
            scrutinee,
            scrutinee_ty,
            alts,
            default,
        } => {
            let new_scrutinee = match subst.get(scrutinee) {
                Some(LcnfArg::Var(v)) => *v,
                _ => *scrutinee,
            };
            LcnfExpr::Case {
                scrutinee: new_scrutinee,
                scrutinee_ty: scrutinee_ty.clone(),
                alts: alts
                    .iter()
                    .map(|alt| LcnfAlt {
                        ctor_name: alt.ctor_name.clone(),
                        ctor_tag: alt.ctor_tag,
                        params: alt.params.clone(),
                        body: inline_subst_expr(&alt.body, subst),
                    })
                    .collect(),
                default: default
                    .as_ref()
                    .map(|d| Box::new(inline_subst_expr(d, subst))),
            }
        }
        LcnfExpr::Return(arg) => LcnfExpr::Return(inline_subst_arg(arg, subst)),
        LcnfExpr::TailCall(func, args) => LcnfExpr::TailCall(
            inline_subst_arg(func, subst),
            args.iter().map(|a| inline_subst_arg(a, subst)).collect(),
        ),
        LcnfExpr::Unreachable => LcnfExpr::Unreachable,
    }
}

pub(super) fn inline_subst_let_value(
    val: &LcnfLetValue,
    subst: &HashMap<LcnfVarId, LcnfArg>,
) -> LcnfLetValue {
    match val {
        LcnfLetValue::App(func, args) => LcnfLetValue::App(
            inline_subst_arg(func, subst),
            args.iter().map(|a| inline_subst_arg(a, subst)).collect(),
        ),
        LcnfLetValue::Ctor(name, tag, args) => LcnfLetValue::Ctor(
            name.clone(),
            *tag,
            args.iter().map(|a| inline_subst_arg(a, subst)).collect(),
        ),
        LcnfLetValue::Reuse(slot, name, tag, args) => LcnfLetValue::Reuse(
            *slot,
            name.clone(),
            *tag,
            args.iter().map(|a| inline_subst_arg(a, subst)).collect(),
        ),
        LcnfLetValue::Proj(name, idx, var) => LcnfLetValue::Proj(name.clone(), *idx, *var),
        LcnfLetValue::Reset(var) => LcnfLetValue::Reset(*var),
        LcnfLetValue::Lit(lit) => LcnfLetValue::Lit(lit.clone()),
        LcnfLetValue::Erased => LcnfLetValue::Erased,
        LcnfLetValue::FVar(id) => LcnfLetValue::FVar(*id),
    }
}

pub(super) fn inline_subst_arg(arg: &LcnfArg, subst: &HashMap<LcnfVarId, LcnfArg>) -> LcnfArg {
    match arg {
        LcnfArg::Var(id) => subst.get(id).cloned().unwrap_or(LcnfArg::Var(*id)),
        LcnfArg::Lit(lit) => LcnfArg::Lit(lit.clone()),
        LcnfArg::Erased => LcnfArg::Erased,
        LcnfArg::Type(ty) => LcnfArg::Type(ty.clone()),
    }
}

/// Splice an inlined body into a `Let` binding context.
///
/// When the outer expression was `let outer_id = callee(args); continuation`,
/// this function rewrites the callee's body so that its terminal `Return(v)`
/// becomes `let outer_id = v; continuation`, preserving ANF.
/// A terminal `TailCall` is left as-is (the continuation becomes unreachable).
pub(super) fn splice_inline_result(
    inlined: LcnfExpr,
    outer_id: LcnfVarId,
    outer_name: &str,
    outer_ty: &LcnfType,
    continuation: LcnfExpr,
) -> LcnfExpr {
    splice_inline_result_inner(inlined, outer_id, outer_name, outer_ty, &continuation)
}

fn splice_inline_result_inner(
    inlined: LcnfExpr,
    outer_id: LcnfVarId,
    outer_name: &str,
    outer_ty: &LcnfType,
    continuation: &LcnfExpr,
) -> LcnfExpr {
    match inlined {
        LcnfExpr::Return(val) => {
            // Convert Return(val) → let outer_id = val; continuation
            let let_val = match val {
                LcnfArg::Var(id) => LcnfLetValue::FVar(id),
                LcnfArg::Lit(lit) => LcnfLetValue::Lit(lit),
                LcnfArg::Erased => LcnfLetValue::Erased,
                LcnfArg::Type(_) => LcnfLetValue::Erased,
            };
            LcnfExpr::Let {
                id: outer_id,
                name: outer_name.to_string(),
                ty: outer_ty.clone(),
                value: let_val,
                body: Box::new(continuation.clone()),
            }
        }
        LcnfExpr::TailCall(_, _) => inlined,
        LcnfExpr::Unreachable => LcnfExpr::Unreachable,
        LcnfExpr::Let {
            id,
            name,
            ty,
            value,
            body,
        } => LcnfExpr::Let {
            id,
            name,
            ty,
            value,
            body: Box::new(splice_inline_result_inner(
                *body,
                outer_id,
                outer_name,
                outer_ty,
                continuation,
            )),
        },
        LcnfExpr::Case {
            scrutinee,
            scrutinee_ty,
            alts,
            default,
        } => LcnfExpr::Case {
            scrutinee,
            scrutinee_ty,
            alts: alts
                .into_iter()
                .map(|alt| LcnfAlt {
                    ctor_name: alt.ctor_name,
                    ctor_tag: alt.ctor_tag,
                    params: alt.params,
                    body: splice_inline_result_inner(
                        alt.body,
                        outer_id,
                        outer_name,
                        outer_ty,
                        continuation,
                    ),
                })
                .collect(),
            default: default.map(|d| {
                Box::new(splice_inline_result_inner(
                    *d,
                    outer_id,
                    outer_name,
                    outer_ty,
                    continuation,
                ))
            }),
        },
    }
}

/// Walk `expr`, inlining calls to functions present in `fn_map` when they
/// satisfy `config`'s criteria.  Returns the (possibly rewritten) expression
/// and increments `inlines_performed` for each inlining performed.
///
/// `id_counter` is a monotonically increasing u64 used to generate fresh
/// offsets so that the inlined bodies never collide with existing var IDs.
pub(super) fn inline_expr_walk(
    expr: LcnfExpr,
    fn_map: &HashMap<String, LcnfFunDecl>,
    config: &InlineConfig,
    caller_max_id: u64,
    id_counter: &mut u64,
    inlines_performed: &mut usize,
) -> LcnfExpr {
    match expr {
        LcnfExpr::Let {
            id,
            name,
            ty,
            value,
            body,
        } => {
            // Try to inline a direct function call in the let-value position.
            if let LcnfLetValue::App(LcnfArg::Lit(LcnfLit::Str(ref callee_name)), ref args) = value
            {
                if let Some(callee_decl) = fn_map.get(callee_name) {
                    let should_inline = {
                        if callee_decl.is_recursive && !config.inline_recursive {
                            false
                        } else {
                            callee_decl.inline_cost <= config.threshold as usize
                        }
                    };
                    if should_inline && callee_decl.params.len() == args.len() {
                        // Compute a fresh offset for this inline to avoid ID collisions.
                        let callee_max = max_var_id_in_expr(&callee_decl.body);
                        let offset = caller_max_id + *id_counter * (callee_max + 1) + 1;
                        *id_counter += 1;

                        // Freshen callee body var IDs to avoid collisions.
                        let fresh_body = offset_var_ids(callee_decl.body.clone(), offset);
                        let fresh_params: Vec<LcnfParam> = callee_decl
                            .params
                            .iter()
                            .map(|p| LcnfParam {
                                id: LcnfVarId(p.id.0 + offset),
                                ..p.clone()
                            })
                            .collect();

                        // Substitute the call arguments for parameters.
                        let substituted =
                            inline_substitute_params(&fresh_body, &fresh_params, args);

                        // Walk the body of the continuation recursively.
                        let new_body = inline_expr_walk(
                            *body,
                            fn_map,
                            config,
                            caller_max_id,
                            id_counter,
                            inlines_performed,
                        );

                        // Splice the inlined body so Return(v) → let id = v; new_body.
                        let spliced = splice_inline_result(substituted, id, &name, &ty, new_body);

                        *inlines_performed += 1;
                        return spliced;
                    }
                }
            }

            // No inline: recurse on both the value (may contain nested exprs in some IRs,
            // though LCNF values are atoms — kept for completeness) and the body.
            let new_body = inline_expr_walk(
                *body,
                fn_map,
                config,
                caller_max_id,
                id_counter,
                inlines_performed,
            );
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
            let new_alts = alts
                .into_iter()
                .map(|alt| LcnfAlt {
                    ctor_name: alt.ctor_name,
                    ctor_tag: alt.ctor_tag,
                    params: alt.params,
                    body: inline_expr_walk(
                        alt.body,
                        fn_map,
                        config,
                        caller_max_id,
                        id_counter,
                        inlines_performed,
                    ),
                })
                .collect();
            let new_default = default.map(|d| {
                Box::new(inline_expr_walk(
                    *d,
                    fn_map,
                    config,
                    caller_max_id,
                    id_counter,
                    inlines_performed,
                ))
            });
            LcnfExpr::Case {
                scrutinee,
                scrutinee_ty,
                alts: new_alts,
                default: new_default,
            }
        }
        // Terminals: no inlining possible.
        terminal @ (LcnfExpr::Return(_) | LcnfExpr::TailCall(_, _) | LcnfExpr::Unreachable) => {
            terminal
        }
    }
}

#[cfg(test)]
mod tests_extended {
    use super::*;
    pub(super) fn make_var(n: u32) -> LcnfVarId {
        LcnfVarId(u64::from(n))
    }
    pub(super) fn make_simple_decl(body: LcnfExpr) -> LcnfFunDecl {
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
    #[test]
    pub(super) fn test_dead_binding_removal() {
        let body = LcnfExpr::Let {
            id: LcnfVarId(99),
            name: "x".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Lit(LcnfLit::Nat(42)),
            body: Box::new(LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0)))),
        };
        let mut decl = make_simple_decl(body);
        let mut pass = DeadBindingElim::default_pass();
        pass.run(&mut decl);
        assert_eq!(decl.body, LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0))));
        assert!(pass.report().bindings_removed >= 0);
    }
    #[test]
    pub(super) fn test_count_let_bindings() {
        let body = LcnfExpr::Let {
            id: LcnfVarId(0),
            name: "a".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Lit(LcnfLit::Nat(1)),
            body: Box::new(LcnfExpr::Let {
                id: LcnfVarId(1),
                name: "b".to_string(),
                ty: LcnfType::Nat,
                value: LcnfLetValue::Lit(LcnfLit::Nat(2)),
                body: Box::new(LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0)))),
            }),
        };
        assert_eq!(count_let_bindings(&body), 2);
    }
    #[test]
    pub(super) fn test_expr_depth() {
        let body = LcnfExpr::Let {
            id: LcnfVarId(0),
            name: "a".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Lit(LcnfLit::Nat(0)),
            body: Box::new(LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0)))),
        };
        assert_eq!(expr_depth(&body), 1);
    }
    #[test]
    pub(super) fn test_has_tail_call_to() {
        let target = make_var(7);
        let body = LcnfExpr::TailCall(LcnfArg::Var(target), vec![]);
        assert!(has_tail_call_to(&body, target));
        assert!(!has_tail_call_to(&body, make_var(8)));
    }
    #[test]
    pub(super) fn test_collect_bound_vars() {
        let body = LcnfExpr::Let {
            id: LcnfVarId(5),
            name: "x".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Lit(LcnfLit::Nat(0)),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(5)))),
        };
        let mut bound = vec![];
        collect_bound_vars(&body, &mut bound);
        assert_eq!(bound, vec![LcnfVarId(5)]);
    }
    #[test]
    pub(super) fn test_opt_pipeline_default() {
        let body = LcnfExpr::Let {
            id: LcnfVarId(0),
            name: "x".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::FVar(LcnfVarId(1)),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(0)))),
        };
        let mut decl = make_simple_decl(body);
        decl.params.push(LcnfParam {
            id: LcnfVarId(1),
            name: "n".to_string(),
            ty: LcnfType::Nat,
            erased: false,
            borrowed: false,
        });
        let mut pipeline = OptPipeline::new();
        let result = pipeline.run(&mut decl);
        assert!(result.copy_prop.copies_eliminated >= 1);
    }
    #[test]
    pub(super) fn test_pass_kind_display() {
        assert_eq!(PassKind::CopyProp.to_string(), "CopyProp");
        assert_eq!(PassKind::DeadBinding.to_string(), "DeadBinding");
        assert_eq!(PassKind::ConstantFold.to_string(), "ConstantFold");
        assert_eq!(PassKind::Inlining.to_string(), "Inlining");
    }
    #[test]
    pub(super) fn test_inline_candidate() {
        let pass = InliningPass::default_pass();
        let cheap = LcnfFunDecl {
            name: "cheap".to_string(),
            original_name: None,
            params: vec![],
            ret_type: LcnfType::Nat,
            body: LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0))),
            is_recursive: false,
            is_lifted: false,
            inline_cost: 1,
        };
        let expensive = LcnfFunDecl {
            inline_cost: 100,
            name: "expensive".to_string(),
            ..cheap.clone()
        };
        assert!(pass.is_inline_candidate(&cheap));
        assert!(!pass.is_inline_candidate(&expensive));
    }
    #[test]
    pub(super) fn test_dead_binding_config_display() {
        let cfg = CopyPropConfig::default();
        let s = format!("{}", cfg);
        assert!(s.contains("CopyPropConfig"));
    }
    #[test]
    pub(super) fn test_dead_binding_report_display() {
        let r = DeadBindingReport {
            bindings_removed: 3,
            passes_run: 2,
        };
        let s = format!("{}", r);
        assert!(s.contains("removed=3"));
        assert!(s.contains("passes=2"));
    }
    #[test]
    pub(super) fn test_constant_fold_report_display() {
        let r = ConstantFoldReport { folds_performed: 7 };
        let s = format!("{}", r);
        assert!(s.contains("folds=7"));
    }
    #[test]
    pub(super) fn test_inline_report_display() {
        let r = InlineReport {
            inlines_performed: 2,
            functions_considered: 10,
        };
        let s = format!("{}", r);
        assert!(s.contains("inlined=2"));
        assert!(s.contains("considered=10"));
    }
}

#[cfg(test)]
mod tests_inlining_pass {
    use super::super::types::{FnMap, InlineConfig, InlineReport, InliningPass};
    use super::*;
    use crate::lcnf::{
        LcnfExpr, LcnfFunDecl, LcnfLetValue, LcnfLit, LcnfParam, LcnfType, LcnfVarId,
    };

    /// Build a minimal `LcnfFunDecl` suitable for testing.
    fn make_decl(
        name: &str,
        params: Vec<LcnfParam>,
        body: LcnfExpr,
        inline_cost: usize,
        is_recursive: bool,
    ) -> LcnfFunDecl {
        LcnfFunDecl {
            name: name.to_string(),
            original_name: None,
            params,
            ret_type: LcnfType::Nat,
            body,
            is_recursive,
            is_lifted: false,
            inline_cost,
        }
    }

    fn make_param(id: u64, name: &str) -> LcnfParam {
        LcnfParam {
            id: LcnfVarId(id),
            name: name.to_string(),
            ty: LcnfType::Nat,
            erased: false,
            borrowed: false,
        }
    }

    /// `callee` has inline_cost=1, body = `return 42`.
    /// `caller` body = `let x = callee(); return x`.
    /// After inlining: `let x = 42; return x`.
    /// Verify: inlines_performed == 1.
    #[test]
    fn inline_pass_simple_call() {
        // callee: fn trivial() = return Nat(42)   (no params)
        let callee = make_decl(
            "trivial",
            vec![],
            LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(42))),
            1,
            false,
        );

        // caller body: let x:100 = trivial(); return x:100
        let caller_body = LcnfExpr::Let {
            id: LcnfVarId(100),
            name: "x".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::App(LcnfArg::Lit(LcnfLit::Str("trivial".to_string())), vec![]),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(100)))),
        };
        let mut caller = make_decl("caller", vec![], caller_body, 5, false);

        let mut fn_map: FnMap = FnMap::new();
        fn_map.insert("trivial".to_string(), callee);

        let config = InlineConfig {
            threshold: 5,
            inline_recursive: false,
        };
        let mut pass = InliningPass::new(config);
        pass.run_with_context(&mut caller, &fn_map);

        assert_eq!(pass.report().inlines_performed, 1);
        assert_eq!(pass.report().functions_considered, 1);

        // After inlining: callee body `Return(Lit(42))` is spliced as
        // `let x:100 = 42; return x:100`
        let expected = LcnfExpr::Let {
            id: LcnfVarId(100),
            name: "x".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Lit(LcnfLit::Nat(42)),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(100)))),
        };
        assert_eq!(caller.body, expected);
    }

    /// Callee has `inline_cost = 100` which exceeds the threshold of 5.
    /// No inlining should occur.
    #[test]
    fn inline_pass_above_threshold_not_inlined() {
        let callee = make_decl(
            "big_fn",
            vec![],
            LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0))),
            100, // cost far above threshold
            false,
        );

        let caller_body = LcnfExpr::Let {
            id: LcnfVarId(0),
            name: "r".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::App(LcnfArg::Lit(LcnfLit::Str("big_fn".to_string())), vec![]),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(0)))),
        };
        let mut caller = make_decl("caller", vec![], caller_body, 5, false);

        let mut fn_map: FnMap = FnMap::new();
        fn_map.insert("big_fn".to_string(), callee);

        let config = InlineConfig {
            threshold: 5,
            inline_recursive: false,
        };
        let mut pass = InliningPass::new(config);
        pass.run_with_context(&mut caller, &fn_map);

        assert_eq!(pass.report().inlines_performed, 0);
        // Body is unchanged (still a Let).
        assert!(matches!(caller.body, LcnfExpr::Let { .. }));
    }

    /// Recursive callee with `inline_recursive = false` must not be inlined.
    #[test]
    fn inline_pass_recursive_not_inlined_when_disabled() {
        let rec_callee = make_decl(
            "rec_fn",
            vec![],
            LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(1))),
            1,
            true, // is_recursive
        );

        let caller_body = LcnfExpr::Let {
            id: LcnfVarId(0),
            name: "r".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::App(LcnfArg::Lit(LcnfLit::Str("rec_fn".to_string())), vec![]),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(0)))),
        };
        let mut caller = make_decl("caller", vec![], caller_body, 5, false);

        let mut fn_map: FnMap = FnMap::new();
        fn_map.insert("rec_fn".to_string(), rec_callee);

        let config = InlineConfig {
            threshold: 5,
            inline_recursive: false, // disable recursive inlining
        };
        let mut pass = InliningPass::new(config);
        pass.run_with_context(&mut caller, &fn_map);

        assert_eq!(pass.report().inlines_performed, 0);
    }

    /// `run_all` should inline `f` into `g`, then reach a fixed-point on the
    /// second pass (no new inlines).
    ///
    /// `f` body = `return 7`
    /// `g` body = `let r = f(); return r`
    /// After one pass: `g` body = `let r = 7; return r`
    /// Second pass: no `f` call left → fixed-point.
    #[test]
    fn inline_pass_run_all_fixpoint() {
        let f = make_decl(
            "f",
            vec![],
            LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(7))),
            1,
            false,
        );
        let g_body = LcnfExpr::Let {
            id: LcnfVarId(50),
            name: "r".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::App(LcnfArg::Lit(LcnfLit::Str("f".to_string())), vec![]),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(50)))),
        };
        let g = make_decl("g", vec![], g_body, 5, false);

        let mut decls = vec![f, g];
        let config = InlineConfig {
            threshold: 5,
            inline_recursive: false,
        };
        let mut pass = InliningPass::new(config);
        pass.run_all(&mut decls);

        // Exactly 1 inline should have been performed (f → g).
        assert_eq!(pass.report().inlines_performed, 1);

        // g's body should now be `let r = 7; return r`.
        let g_decl = decls.iter().find(|d| d.name == "g").expect("g not found");
        let expected_g_body = LcnfExpr::Let {
            id: LcnfVarId(50),
            name: "r".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Lit(LcnfLit::Nat(7)),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(50)))),
        };
        assert_eq!(g_decl.body, expected_g_body);
    }

    /// Tests that var-ID freshening prevents collisions when the callee body
    /// contains a Let binding with the same `LcnfVarId` as the caller.
    ///
    /// callee `wrap(p1:VarId(1))`:
    ///   `let t:VarId(0) = Ctor("Pair", 0, [p1]); return t:VarId(0)`
    ///
    /// caller body:
    ///   `let x:VarId(0) = wrap(Nat(5)); return x:VarId(0)`
    ///
    /// After inlining, the resulting expression must have distinct bound `VarId`s:
    /// the callee's `t` must have been offset so it no longer conflicts with
    /// the caller's `x`.
    #[test]
    fn inline_pass_freshen_var_ids_no_collision() {
        // callee: `wrap(p1) = let t:0 = Ctor("Pair", 0, [p1]); return t:0`
        // param p1 has id=1, return var t has id=0 — same as the caller's let-id
        let param_p1 = make_param(1, "p1");
        let callee_body = LcnfExpr::Let {
            id: LcnfVarId(0), // t — same numeric id as the caller's x
            name: "t".to_string(),
            ty: LcnfType::Object,
            value: LcnfLetValue::Ctor(
                "Pair".to_string(),
                0,
                vec![LcnfArg::Var(LcnfVarId(1))], // p1
            ),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(0)))),
        };
        let callee = make_decl("wrap", vec![param_p1.clone()], callee_body, 2, false);

        // caller: `let x:0 = wrap(Nat(5)); return x:0`
        let caller_body = LcnfExpr::Let {
            id: LcnfVarId(0), // x — SAME id as callee's t
            name: "x".to_string(),
            ty: LcnfType::Object,
            value: LcnfLetValue::App(
                LcnfArg::Lit(LcnfLit::Str("wrap".to_string())),
                vec![LcnfArg::Lit(LcnfLit::Nat(5))],
            ),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(0)))),
        };
        let mut caller = make_decl("caller", vec![], caller_body, 5, false);

        let mut fn_map: FnMap = FnMap::new();
        fn_map.insert("wrap".to_string(), callee);

        let config = InlineConfig {
            threshold: 5,
            inline_recursive: false,
        };
        let mut pass = InliningPass::new(config);
        pass.run_with_context(&mut caller, &fn_map);

        assert_eq!(pass.report().inlines_performed, 1, "expected one inline");

        // Collect all bound VarIds in the resulting expression.
        // They must all be distinct — no two Let bindings with the same id.
        let mut bound = Vec::new();
        collect_bound_vars(&caller.body, &mut bound);

        // The inlined body has one internal Let (callee's `t`, now freshened)
        // and the outer splice introduces `x:0` as the result binding.
        // So we expect 2 bound vars total.
        assert_eq!(
            bound.len(),
            2,
            "expected 2 bound vars after inlining, got {:?}",
            bound
        );

        // All bound var ids must be distinct.
        let ids: Vec<u64> = bound.iter().map(|v| v.0).collect();
        let unique_count = {
            let mut seen = std::collections::HashSet::new();
            ids.iter().filter(|&&id| seen.insert(id)).count()
        };
        assert_eq!(
            unique_count,
            bound.len(),
            "collision: bound var ids are not unique: {:?}",
            ids
        );
    }

    /// Inlining with a parameter: callee takes one param, caller passes a literal.
    ///
    /// `identity(p0) = return p0`
    /// `caller` body = `let x:200 = identity(Nat(99)); return x:200`
    /// After inlining: callee body `return p0` → p0 substituted with Nat(99)
    /// → `return Nat(99)` → spliced as `let x:200 = 99; return x:200`.
    #[test]
    fn inline_pass_with_param_substitution() {
        let param = make_param(10, "p0");
        let callee = make_decl(
            "identity",
            vec![param.clone()],
            LcnfExpr::Return(LcnfArg::Var(param.id)),
            1,
            false,
        );

        let caller_body = LcnfExpr::Let {
            id: LcnfVarId(200),
            name: "x".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::App(
                LcnfArg::Lit(LcnfLit::Str("identity".to_string())),
                vec![LcnfArg::Lit(LcnfLit::Nat(99))],
            ),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(200)))),
        };
        let mut caller = make_decl("caller", vec![], caller_body, 5, false);

        let mut fn_map: FnMap = FnMap::new();
        fn_map.insert("identity".to_string(), callee);

        let config = InlineConfig {
            threshold: 5,
            inline_recursive: false,
        };
        let mut pass = InliningPass::new(config);
        pass.run_with_context(&mut caller, &fn_map);

        assert_eq!(pass.report().inlines_performed, 1);

        // After inlining: spliced body is `let x:200 = Nat(99); return x:200`
        let expected = LcnfExpr::Let {
            id: LcnfVarId(200),
            name: "x".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Lit(LcnfLit::Nat(99)),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(200)))),
        };
        assert_eq!(caller.body, expected);
    }
}
