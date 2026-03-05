//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use oxilean_kernel::{BinderInfo, Expr, Level, Literal, Name};
use std::collections::{HashMap, HashSet, VecDeque};

use super::types::{
    ClosureConverter, ConversionError, ConversionStats, LambdaLifter, ProofEraser, ToLcnfConfig,
    ToLcnfState,
};

/// Convert a kernel `Name` to its string representation for LCNF.
pub(super) fn name_to_string(name: &Name) -> String {
    match name {
        Name::Anonymous => "_".to_string(),
        _ => name.to_string(),
    }
}
/// Mangle a kernel name into a valid LCNF identifier.
pub(super) fn mangle_name(name: &Name) -> String {
    let s = name_to_string(name);
    s.replace(['.', ' '], "_")
        .replace('<', "_lt_")
        .replace('>', "_gt_")
        .replace('{', "_lb_")
        .replace('}', "_rb_")
}
/// Convert a binder info to a human-readable string (for debugging).
pub(super) fn binder_info_str(bi: &BinderInfo) -> &'static str {
    match bi {
        BinderInfo::Default => "explicit",
        BinderInfo::Implicit => "implicit",
        BinderInfo::StrictImplicit => "strict_implicit",
        BinderInfo::InstImplicit => "inst_implicit",
    }
}
/// Convert a kernel expression (used as a type) to an LCNF type.
///
/// This performs a best-effort translation. Types that cannot be represented
/// in LCNF are mapped to `LcnfType::Object`.
pub(super) fn convert_type(expr: &Expr, state: &ToLcnfState) -> LcnfType {
    convert_type_inner(expr, state, 0)
}
/// Inner recursive type conversion with depth tracking.
pub(super) fn convert_type_inner(expr: &Expr, state: &ToLcnfState, depth: usize) -> LcnfType {
    if depth > 64 {
        return LcnfType::Object;
    }
    match expr {
        Expr::Sort(level) if level.is_zero() => LcnfType::Irrelevant,
        Expr::Sort(_) => LcnfType::Erased,
        Expr::BVar(idx) => {
            if let Some(name) = state.lookup_bvar_name(*idx) {
                LcnfType::Var(name.to_string())
            } else {
                LcnfType::Var(format!("bv_{}", idx))
            }
        }
        Expr::FVar(fid) => LcnfType::Var(format!("fv_{}", fid.0)),
        Expr::Const(name, _levels) => {
            let name_str = name_to_string(name);
            match name_str.as_str() {
                "Nat" => LcnfType::Nat,
                "String" => LcnfType::LcnfString,
                "Unit" | "PUnit" | "True" => LcnfType::Unit,
                "Prop" => LcnfType::Irrelevant,
                _ => {
                    if state.is_proof_name(&name_str) {
                        LcnfType::Irrelevant
                    } else {
                        LcnfType::Ctor(name_str, Vec::new())
                    }
                }
            }
        }
        Expr::App(func, arg) => {
            let func_ty = convert_type_inner(func, state, depth + 1);
            let arg_ty = convert_type_inner(arg, state, depth + 1);
            match func_ty {
                LcnfType::Ctor(name, mut args) => {
                    args.push(arg_ty);
                    LcnfType::Ctor(name, args)
                }
                _ => LcnfType::Object,
            }
        }
        Expr::Pi(_bi, _name, domain, codomain) => {
            let dom_ty = convert_type_inner(domain, state, depth + 1);
            let cod_ty = convert_type_inner(codomain, state, depth + 1);
            if !has_bvar_ref(codomain, 0) {
                LcnfType::Fun(vec![dom_ty], Box::new(cod_ty))
            } else {
                LcnfType::Object
            }
        }
        Expr::Lam(_, _, _, _) => LcnfType::Object,
        Expr::Let(_name, _ty, _val, body) => convert_type_inner(body, state, depth + 1),
        Expr::Lit(_) => LcnfType::Object,
        Expr::Proj(name, idx, _base) => {
            let name_str = name_to_string(name);
            LcnfType::Var(format!("{}.{}", name_str, idx))
        }
    }
}
/// Flatten a nested Pi type into a list of parameter types and a return type.
pub(super) fn flatten_pi_type(
    expr: &Expr,
    state: &ToLcnfState,
) -> (Vec<(String, LcnfType, bool)>, LcnfType) {
    let mut params = Vec::new();
    let mut current = expr;
    loop {
        match current {
            Expr::Pi(bi, name, domain, codomain) => {
                let name_str = name_to_string(name);
                let dom_ty = convert_type(domain, state);
                let is_erased = matches!(bi, BinderInfo::Implicit | BinderInfo::StrictImplicit)
                    || dom_ty == LcnfType::Irrelevant;
                params.push((name_str, dom_ty, is_erased));
                current = codomain;
            }
            _ => {
                let ret_ty = convert_type(current, state);
                return (params, ret_ty);
            }
        }
    }
}
/// Merge function types: given params [(ty1), (ty2), ...] and return type,
/// build a single LcnfType::Fun.
pub(super) fn build_fun_type(params: &[LcnfType], ret: &LcnfType) -> LcnfType {
    if params.is_empty() {
        ret.clone()
    } else {
        LcnfType::Fun(params.to_vec(), Box::new(ret.clone()))
    }
}
/// Check if an expression references BVar(target).
pub(super) fn has_bvar_ref(expr: &Expr, target: u32) -> bool {
    match expr {
        Expr::BVar(n) => *n == target,
        Expr::FVar(_) | Expr::Sort(_) | Expr::Const(_, _) | Expr::Lit(_) => false,
        Expr::App(f, a) => has_bvar_ref(f, target) || has_bvar_ref(a, target),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            has_bvar_ref(ty, target) || has_bvar_ref(body, target + 1)
        }
        Expr::Let(_, ty, val, body) => {
            has_bvar_ref(ty, target) || has_bvar_ref(val, target) || has_bvar_ref(body, target + 1)
        }
        Expr::Proj(_, _, e) => has_bvar_ref(e, target),
    }
}
/// Convert a kernel expression to an LCNF expression.
///
/// This is the main recursive translator. It walks the kernel AST and
/// produces an ANF-style LCNF expression tree.
#[allow(clippy::too_many_arguments)]
pub(super) fn convert_expr(
    expr: &Expr,
    state: &mut ToLcnfState,
    is_tail: bool,
) -> Result<LcnfExpr, ConversionError> {
    state.enter_depth()?;
    state.stats.exprs_visited += 1;
    let result = convert_expr_inner(expr, state, is_tail);
    state.leave_depth();
    result
}
/// Inner conversion dispatcher.
pub(super) fn convert_expr_inner(
    expr: &Expr,
    state: &mut ToLcnfState,
    is_tail: bool,
) -> Result<LcnfExpr, ConversionError> {
    match expr {
        Expr::BVar(idx) => convert_bvar(*idx, state),
        Expr::FVar(fid) => convert_fvar(fid.0, state),
        Expr::Sort(level) => convert_sort(level, state),
        Expr::Const(name, levels) => convert_const(name, levels, state),
        Expr::App(func, arg) => convert_app(func, arg, state, is_tail),
        Expr::Lam(bi, name, ty, body) => convert_lam(bi, name, ty, body, state),
        Expr::Pi(bi, name, ty, body) => convert_pi(bi, name, ty, body, state),
        Expr::Let(name, ty, val, body) => convert_let(name, ty, val, body, state),
        Expr::Lit(lit) => convert_lit(lit, state),
        Expr::Proj(name, idx, base) => convert_proj(name, *idx, base, state),
    }
}
/// Convert a bound variable reference.
pub(super) fn convert_bvar(idx: u32, state: &mut ToLcnfState) -> Result<LcnfExpr, ConversionError> {
    match state.lookup_bvar(idx) {
        Some(var_id) => Ok(LcnfExpr::Return(LcnfArg::Var(var_id))),
        None => Err(ConversionError::UnboundVariable(format!(
            "BVar({}) not in scope (stack depth: {})",
            idx,
            state.bvar_stack.len()
        ))),
    }
}
/// Convert a free variable reference.
pub(super) fn convert_fvar(id: u64, state: &mut ToLcnfState) -> Result<LcnfExpr, ConversionError> {
    let fvar_name = format!("fv_{}", id);
    if let Some(var_id) = state.lookup_name(&fvar_name) {
        Ok(LcnfExpr::Return(LcnfArg::Var(var_id)))
    } else {
        let var_id = state.fresh_named_var(&fvar_name);
        state.name_map.insert(fvar_name, var_id);
        let let_id = state.emit_let("fvar", LcnfType::Object, LcnfLetValue::FVar(var_id));
        Ok(state.wrap_pending_lets(LcnfExpr::Return(LcnfArg::Var(let_id))))
    }
}
/// Convert a Sort expression.
///
/// Sorts are type-level constructs and typically erased. If type erasure is
/// enabled, they become Erased; otherwise, they are represented as a unit value.
pub(super) fn convert_sort(
    level: &Level,
    state: &mut ToLcnfState,
) -> Result<LcnfExpr, ConversionError> {
    if state.config.erase_types {
        state.stats.types_erased += 1;
        state.metadata.types_erased += 1;
        Ok(LcnfExpr::Return(LcnfArg::Erased))
    } else {
        let level_val = level_to_u64(level);
        let lit_val = LcnfLetValue::Lit(LcnfLit::Nat(level_val));
        let id = state.emit_let("sort", LcnfType::Erased, lit_val);
        Ok(state.wrap_pending_lets(LcnfExpr::Return(LcnfArg::Var(id))))
    }
}
/// Convert a universe level to a u64 approximation.
pub(super) fn level_to_u64(level: &Level) -> u64 {
    match level {
        Level::Zero => 0,
        Level::Succ(inner) => level_to_u64(inner).saturating_add(1),
        Level::Max(l1, l2) => level_to_u64(l1).max(level_to_u64(l2)),
        Level::IMax(_, l2) => {
            let v2 = level_to_u64(l2);
            if v2 == 0 {
                0
            } else {
                v2
            }
        }
        Level::Param(_) => 1,
        Level::MVar(_) => 1,
    }
}
/// Convert a named constant reference.
pub(super) fn convert_const(
    name: &Name,
    _levels: &[Level],
    state: &mut ToLcnfState,
) -> Result<LcnfExpr, ConversionError> {
    let name_str = name_to_string(name);
    let mangled = mangle_name(name);
    if state.config.erase_proofs && state.is_proof_name(&name_str) {
        state.stats.proofs_erased += 1;
        state.metadata.proofs_erased += 1;
        return Ok(LcnfExpr::Return(LcnfArg::Erased));
    }
    if state.config.erase_types && state.is_type_name(&name_str) {
        state.stats.types_erased += 1;
        state.metadata.types_erased += 1;
        return Ok(LcnfExpr::Return(LcnfArg::Erased));
    }
    if let Some(var_id) = state.lookup_name(&mangled) {
        Ok(LcnfExpr::Return(LcnfArg::Var(var_id)))
    } else {
        let var_id = state.fresh_named_var(&mangled);
        state.name_map.insert(mangled, var_id);
        Ok(LcnfExpr::Return(LcnfArg::Var(var_id)))
    }
}
/// Convert a function application.
///
/// This is one of the most complex cases. We need to:
/// 1. Flatten nested applications: `((f a) b)` -> `f(a, b)`
/// 2. Ensure all arguments are atomic (ANF requirement)
/// 3. Detect tail calls
pub(super) fn convert_app(
    func: &Expr,
    arg: &Expr,
    state: &mut ToLcnfState,
    is_tail: bool,
) -> Result<LcnfExpr, ConversionError> {
    let (head, args) = flatten_app(func, arg);
    let head_arg = convert_to_atomic(head, state, "func")?;
    let mut lcnf_args = Vec::new();
    for (i, a) in args.iter().enumerate() {
        if should_erase_arg(a, state) {
            lcnf_args.push(LcnfArg::Erased);
            continue;
        }
        let hint = format!("arg{}", i);
        let arg_val = convert_to_atomic(a, state, &hint)?;
        lcnf_args.push(arg_val);
    }
    if is_tail {
        state.stats.tail_calls_detected += 1;
        let result = LcnfExpr::TailCall(head_arg, lcnf_args);
        Ok(state.wrap_pending_lets(result))
    } else {
        let app_val = LcnfLetValue::App(head_arg, lcnf_args);
        let result_id = state.emit_let("app", LcnfType::Object, app_val);
        let result = LcnfExpr::Return(LcnfArg::Var(result_id));
        Ok(state.wrap_pending_lets(result))
    }
}
/// Flatten a nested application spine.
///
/// `App(App(App(f, a), b), c)` -> `(f, [a, b, c])`
pub(super) fn flatten_app<'a>(func: &'a Expr, arg: &'a Expr) -> (&'a Expr, Vec<&'a Expr>) {
    let mut args = vec![arg];
    let mut head = func;
    while let Expr::App(inner_func, inner_arg) = head {
        args.push(inner_arg);
        head = inner_func;
    }
    args.reverse();
    (head, args)
}
/// Check whether a kernel argument expression should be erased.
pub(super) fn should_erase_arg(expr: &Expr, state: &ToLcnfState) -> bool {
    if !state.config.erase_proofs && !state.config.erase_types {
        return false;
    }
    match expr {
        Expr::Sort(_) => state.config.erase_types,
        Expr::Const(name, _) => {
            let name_str = name_to_string(name);
            (state.config.erase_proofs && state.is_proof_name(&name_str))
                || (state.config.erase_types && state.is_type_name(&name_str))
        }
        Expr::Pi(_, _, _, _) => state.config.erase_types,
        _ => false,
    }
}
/// Convert an expression to an atomic LCNF argument.
///
/// If the expression is already atomic (a variable or literal), return it directly.
/// Otherwise, generate a let binding for it and return the variable reference.
pub(super) fn convert_to_atomic(
    expr: &Expr,
    state: &mut ToLcnfState,
    hint: &str,
) -> Result<LcnfArg, ConversionError> {
    match expr {
        Expr::BVar(idx) => match state.lookup_bvar(*idx) {
            Some(var_id) => Ok(LcnfArg::Var(var_id)),
            None => Err(ConversionError::UnboundVariable(format!(
                "BVar({}) in atomic conversion",
                idx
            ))),
        },
        Expr::Lit(Literal::Nat(n)) => Ok(LcnfArg::Lit(LcnfLit::Nat(*n))),
        Expr::Lit(Literal::Str(s)) => Ok(LcnfArg::Lit(LcnfLit::Str(s.clone()))),
        Expr::Const(name, _levels) => {
            let mangled = mangle_name(name);
            if let Some(var_id) = state.lookup_name(&mangled) {
                Ok(LcnfArg::Var(var_id))
            } else {
                let var_id = state.fresh_named_var(&mangled);
                state.name_map.insert(mangled, var_id);
                Ok(LcnfArg::Var(var_id))
            }
        }
        Expr::FVar(fid) => {
            let fvar_name = format!("fv_{}", fid.0);
            if let Some(var_id) = state.lookup_name(&fvar_name) {
                Ok(LcnfArg::Var(var_id))
            } else {
                let var_id = state.fresh_named_var(&fvar_name);
                state.name_map.insert(fvar_name, var_id);
                Ok(LcnfArg::Var(var_id))
            }
        }
        Expr::Sort(_) => {
            if state.config.erase_types {
                Ok(LcnfArg::Erased)
            } else {
                let ty = convert_type(expr, state);
                Ok(LcnfArg::Type(ty))
            }
        }
        _ => {
            let lcnf = convert_expr(expr, state, false)?;
            let id = bind_expr_to_var(lcnf, state, hint)?;
            Ok(LcnfArg::Var(id))
        }
    }
}
/// Bind a complex LCNF expression to a fresh variable, returning the variable ID.
///
/// If the expression is already a simple Return of a variable, extract it directly.
pub(super) fn bind_expr_to_var(
    expr: LcnfExpr,
    state: &mut ToLcnfState,
    hint: &str,
) -> Result<LcnfVarId, ConversionError> {
    match expr {
        LcnfExpr::Return(LcnfArg::Var(id)) => Ok(id),
        LcnfExpr::Return(arg) => {
            let val = match arg {
                LcnfArg::Lit(lit) => LcnfLetValue::Lit(lit),
                LcnfArg::Erased => LcnfLetValue::Erased,
                LcnfArg::Type(_) => LcnfLetValue::Erased,
                LcnfArg::Var(id) => LcnfLetValue::FVar(id),
            };
            let id = state.emit_let(hint, LcnfType::Object, val);
            Ok(id)
        }
        _ => {
            let id = state.fresh_named_var(hint);
            Ok(id)
        }
    }
}
/// Convert a lambda abstraction.
///
/// Lambdas are converted to either:
/// - An inline closure (if small enough)
/// - A lifted top-level function (if lambda lifting is enabled)
#[allow(clippy::too_many_arguments)]
pub(super) fn convert_lam(
    bi: &BinderInfo,
    name: &Name,
    ty: &Expr,
    body: &Expr,
    state: &mut ToLcnfState,
) -> Result<LcnfExpr, ConversionError> {
    let name_str = name_to_string(name);
    let param_ty = convert_type(ty, state);
    let is_erased = match bi {
        BinderInfo::Implicit | BinderInfo::StrictImplicit => {
            state.config.erase_types && param_ty == LcnfType::Irrelevant
        }
        _ => false,
    };
    let param_id = state.fresh_named_var(&name_str);
    let param = LcnfParam {
        id: param_id,
        name: name_str.clone(),
        ty: param_ty.clone(),
        erased: is_erased,
        borrowed: false,
    };
    state.push_bvar(param_id, &name_str);
    let body_lcnf = convert_expr(body, state, true)?;
    state.pop_bvar();
    let expr_size = estimate_expr_size(body);
    if state.config.lambda_lift && expr_size > state.config.max_inline_size {
        let free_vars = collect_free_vars_expr(body, &state.bvar_stack);
        let lift_name = state.fresh_lift_name(&name_str);
        let mut all_params: Vec<LcnfParam> = free_vars
            .iter()
            .map(|&fv_id| LcnfParam {
                id: fv_id,
                name: format!("fv_{}", fv_id.0),
                ty: LcnfType::Object,
                erased: false,
                borrowed: false,
            })
            .collect();
        all_params.push(param);
        let lifted_decl = LcnfFunDecl {
            name: lift_name.clone(),
            original_name: Some(name.clone()),
            params: all_params,
            ret_type: LcnfType::Object,
            body: body_lcnf,
            is_recursive: false,
            is_lifted: true,
            inline_cost: expr_size,
        };
        state.lifted_funs.push(lifted_decl);
        state.stats.lambdas_lifted += 1;
        state.metadata.lambdas_lifted += 1;
        let lift_var_id = state.fresh_named_var(&lift_name);
        state.name_map.insert(lift_name, lift_var_id);
        if free_vars.is_empty() {
            Ok(LcnfExpr::Return(LcnfArg::Var(lift_var_id)))
        } else {
            let captured_args: Vec<LcnfArg> =
                free_vars.iter().map(|&id| LcnfArg::Var(id)).collect();
            let papp_val = LcnfLetValue::App(LcnfArg::Var(lift_var_id), captured_args);
            let papp_id = state.emit_let("papp", LcnfType::Object, papp_val);
            Ok(state.wrap_pending_lets(LcnfExpr::Return(LcnfArg::Var(papp_id))))
        }
    } else {
        let closure_name = state.fresh_lift_name("closure");
        let closure_decl = LcnfFunDecl {
            name: closure_name.clone(),
            original_name: Some(name.clone()),
            params: vec![param],
            ret_type: LcnfType::Object,
            body: body_lcnf,
            is_recursive: false,
            is_lifted: false,
            inline_cost: expr_size,
        };
        state.lifted_funs.push(closure_decl);
        let closure_var = state.fresh_named_var(&closure_name);
        state.name_map.insert(closure_name, closure_var);
        Ok(LcnfExpr::Return(LcnfArg::Var(closure_var)))
    }
}
/// Convert a Pi (dependent function) type expression.
///
/// Pi types are type-level constructs. When erase_types is enabled, they are erased.
/// Otherwise, they produce a type representation.
pub(super) fn convert_pi(
    _bi: &BinderInfo,
    _name: &Name,
    _ty: &Expr,
    _body: &Expr,
    state: &mut ToLcnfState,
) -> Result<LcnfExpr, ConversionError> {
    if state.config.erase_types {
        state.stats.types_erased += 1;
        state.metadata.types_erased += 1;
        Ok(LcnfExpr::Return(LcnfArg::Erased))
    } else {
        let pi_val = LcnfLetValue::Erased;
        let id = state.emit_let("pi_type", LcnfType::Erased, pi_val);
        Ok(state.wrap_pending_lets(LcnfExpr::Return(LcnfArg::Var(id))))
    }
}
/// Convert a let binding.
///
/// Let bindings map naturally to LCNF let bindings.
pub(super) fn convert_let(
    name: &Name,
    ty: &Expr,
    val: &Expr,
    body: &Expr,
    state: &mut ToLcnfState,
) -> Result<LcnfExpr, ConversionError> {
    let name_str = name_to_string(name);
    let lcnf_ty = convert_type(ty, state);
    let val_lcnf = convert_expr(val, state, false)?;
    let var_id = bind_expr_to_var(val_lcnf, state, &name_str)?;
    state.push_bvar(var_id, &name_str);
    let body_lcnf = convert_expr(body, state, true)?;
    state.pop_bvar();
    let let_name = if state.config.debug_names {
        name_str
    } else {
        format!("_x{}", var_id.0)
    };
    let let_val = LcnfLetValue::FVar(var_id);
    let result = LcnfExpr::Let {
        id: var_id,
        name: let_name,
        ty: lcnf_ty,
        value: let_val,
        body: Box::new(body_lcnf),
    };
    Ok(state.wrap_pending_lets(result))
}
/// Convert a literal expression.
pub(super) fn convert_lit(
    lit: &Literal,
    state: &mut ToLcnfState,
) -> Result<LcnfExpr, ConversionError> {
    let lcnf_lit = match lit {
        Literal::Nat(n) => LcnfLit::Nat(*n),
        Literal::Str(s) => LcnfLit::Str(s.clone()),
    };
    let ty = match lit {
        Literal::Nat(_) => LcnfType::Nat,
        Literal::Str(_) => LcnfType::LcnfString,
    };
    let id = state.emit_let("lit", ty, LcnfLetValue::Lit(lcnf_lit));
    Ok(state.wrap_pending_lets(LcnfExpr::Return(LcnfArg::Var(id))))
}
/// Convert a structure projection.
pub(super) fn convert_proj(
    name: &Name,
    idx: u32,
    base: &Expr,
    state: &mut ToLcnfState,
) -> Result<LcnfExpr, ConversionError> {
    let name_str = name_to_string(name);
    let base_arg = convert_to_atomic(base, state, "proj_base")?;
    let base_var = match base_arg {
        LcnfArg::Var(id) => id,
        _ => state.emit_let(
            "proj_base",
            LcnfType::Object,
            match base_arg {
                LcnfArg::Lit(l) => LcnfLetValue::Lit(l),
                LcnfArg::Erased => LcnfLetValue::Erased,
                _ => LcnfLetValue::Erased,
            },
        ),
    };
    let proj_val = LcnfLetValue::Proj(name_str, idx, base_var);
    let proj_id = state.emit_let("proj", LcnfType::Object, proj_val);
    Ok(state.wrap_pending_lets(LcnfExpr::Return(LcnfArg::Var(proj_id))))
}
/// Estimate the size (number of AST nodes) of a kernel expression.
pub(super) fn estimate_expr_size(expr: &Expr) -> usize {
    match expr {
        Expr::BVar(_) | Expr::FVar(_) | Expr::Sort(_) | Expr::Lit(_) => 1,
        Expr::Const(_, levels) => 1 + levels.len(),
        Expr::App(f, a) => 1 + estimate_expr_size(f) + estimate_expr_size(a),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            1 + estimate_expr_size(ty) + estimate_expr_size(body)
        }
        Expr::Let(_, ty, val, body) => {
            1 + estimate_expr_size(ty) + estimate_expr_size(val) + estimate_expr_size(body)
        }
        Expr::Proj(_, _, base) => 1 + estimate_expr_size(base),
    }
}
/// Collect free variable IDs referenced in a kernel expression,
/// given the current bvar stack for resolving bound variables.
pub(super) fn collect_free_vars_expr(expr: &Expr, bvar_stack: &[LcnfVarId]) -> Vec<LcnfVarId> {
    let mut free = Vec::new();
    let mut seen = HashSet::new();
    collect_free_vars_inner(expr, bvar_stack, 0, &mut free, &mut seen);
    free
}
/// Inner recursive free variable collector.
#[allow(clippy::too_many_arguments)]
pub(super) fn collect_free_vars_inner(
    expr: &Expr,
    bvar_stack: &[LcnfVarId],
    depth: u32,
    free: &mut Vec<LcnfVarId>,
    seen: &mut HashSet<LcnfVarId>,
) {
    match expr {
        Expr::BVar(idx) => {
            let adjusted = idx.saturating_sub(depth);
            let stack_len = bvar_stack.len();
            if (*idx >= depth) && (adjusted as usize) < stack_len {
                let var_id = bvar_stack[stack_len - 1 - adjusted as usize];
                if seen.insert(var_id) {
                    free.push(var_id);
                }
            }
        }
        Expr::FVar(_) => {}
        Expr::Sort(_) | Expr::Lit(_) | Expr::Const(_, _) => {}
        Expr::App(f, a) => {
            collect_free_vars_inner(f, bvar_stack, depth, free, seen);
            collect_free_vars_inner(a, bvar_stack, depth, free, seen);
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            collect_free_vars_inner(ty, bvar_stack, depth, free, seen);
            collect_free_vars_inner(body, bvar_stack, depth + 1, free, seen);
        }
        Expr::Let(_, ty, val, body) => {
            collect_free_vars_inner(ty, bvar_stack, depth, free, seen);
            collect_free_vars_inner(val, bvar_stack, depth, free, seen);
            collect_free_vars_inner(body, bvar_stack, depth + 1, free, seen);
        }
        Expr::Proj(_, _, base) => {
            collect_free_vars_inner(base, bvar_stack, depth, free, seen);
        }
    }
}
/// Convert a single kernel expression to LCNF.
///
/// This is the main entry point for converting a standalone expression.
pub fn expr_to_lcnf(expr: &Expr, config: &ToLcnfConfig) -> Result<LcnfExpr, ConversionError> {
    let mut state = ToLcnfState::new(config);
    let lcnf = convert_expr(expr, &mut state, true)?;
    Ok(lcnf)
}
/// Convert a kernel declaration (name, parameters, body) to an LCNF function declaration.
///
/// This processes a single top-level definition, applying all configured passes.
pub fn decl_to_lcnf(
    name: &Name,
    params: &[(Name, Expr)],
    body: &Expr,
    config: &ToLcnfConfig,
) -> Result<LcnfFunDecl, ConversionError> {
    let mut state = ToLcnfState::new(config);
    let name_str = mangle_name(name);
    let mut lcnf_params = Vec::new();
    for (pname, pty) in params {
        let pname_str = name_to_string(pname);
        let param_ty = convert_type(pty, &state);
        let param_id = state.fresh_named_var(&pname_str);
        let is_erased = param_ty == LcnfType::Irrelevant
            || (config.erase_types && param_ty == LcnfType::Erased);
        let param = LcnfParam {
            id: param_id,
            name: pname_str.clone(),
            ty: param_ty,
            erased: is_erased,
            borrowed: false,
        };
        state.push_bvar(param_id, &pname_str);
        lcnf_params.push(param);
    }
    let body_lcnf = convert_expr(body, &mut state, true)?;
    for _ in params {
        state.pop_bvar();
    }
    let ret_type = infer_return_type(&body_lcnf);
    let is_recursive = check_recursive(&body_lcnf, &name_str, &state);
    let inline_cost = compute_inline_cost(&body_lcnf);
    let mut decl = LcnfFunDecl {
        name: name_str,
        original_name: Some(name.clone()),
        params: lcnf_params,
        ret_type,
        body: body_lcnf,
        is_recursive,
        is_lifted: false,
        inline_cost,
    };
    if config.erase_proofs {
        let mut eraser = ProofEraser::new();
        eraser.erase_decl(&mut decl);
        state.stats.proofs_erased += eraser.erased_count;
        state.metadata.proofs_erased += eraser.erased_count;
    }
    let mut lifted = state.take_lifted_funs();
    if !lifted.is_empty() && config.lambda_lift {
        let mut lifter = LambdaLifter::new(config.max_inline_size);
        lifter.lift_module(&mut lifted);
    }
    Ok(decl)
}
/// Convert a collection of kernel declarations to an LCNF module.
///
/// This is the main entry point for batch conversion of a module.
#[allow(clippy::type_complexity)]
pub fn module_to_lcnf(
    decls: &[(Name, Vec<(Name, Expr)>, Expr)],
    config: &ToLcnfConfig,
) -> Result<LcnfModule, ConversionError> {
    let mut module = LcnfModule {
        fun_decls: Vec::new(),
        extern_decls: Vec::new(),
        name: String::new(),
        metadata: LcnfModuleMetadata::default(),
    };
    let mut all_lifted = Vec::new();
    for (name, params, body) in decls {
        let decl = decl_to_lcnf(name, params, body, config)?;
        module.fun_decls.push(decl);
        module.metadata.decl_count += 1;
    }
    if config.lambda_lift {
        let mut lifter = LambdaLifter::new(config.max_inline_size);
        lifter.lift_module(&mut module.fun_decls);
        module.metadata.lambdas_lifted += lifter.lifted.len();
        all_lifted.append(&mut lifter.lifted);
    }
    module.fun_decls.append(&mut all_lifted);
    let mut closure_conv = ClosureConverter::new();
    closure_conv.convert_module(&mut module);
    if config.erase_proofs {
        let mut eraser = ProofEraser::new();
        for decl in &mut module.fun_decls {
            eraser.erase_decl(decl);
        }
        module.metadata.proofs_erased += eraser.erased_count;
    }
    Ok(module)
}
/// Infer the return type of an LCNF expression (heuristic).
pub(super) fn infer_return_type(expr: &LcnfExpr) -> LcnfType {
    match expr {
        LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(_))) => LcnfType::Nat,
        LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Str(_))) => LcnfType::LcnfString,
        LcnfExpr::Return(LcnfArg::Erased) => LcnfType::Erased,
        LcnfExpr::Let { body, .. } => infer_return_type(body),
        LcnfExpr::Case { alts, default, .. } => {
            if let Some(alt) = alts.first() {
                infer_return_type(&alt.body)
            } else if let Some(def) = default {
                infer_return_type(def)
            } else {
                LcnfType::Object
            }
        }
        LcnfExpr::Unreachable => LcnfType::Object,
        _ => LcnfType::Object,
    }
}
/// Check if an expression is likely recursive (references a given name).
pub(super) fn check_recursive(expr: &LcnfExpr, _name: &str, _state: &ToLcnfState) -> bool {
    match expr {
        LcnfExpr::TailCall(_, _) => true,
        LcnfExpr::Let { body, .. } => check_recursive(body, _name, _state),
        LcnfExpr::Case { alts, default, .. } => {
            alts.iter()
                .any(|alt| check_recursive(&alt.body, _name, _state))
                || default
                    .as_ref()
                    .map(|d| check_recursive(d, _name, _state))
                    .unwrap_or(false)
        }
        _ => false,
    }
}
/// Compute the inline cost of an LCNF expression.
pub(super) fn compute_inline_cost(expr: &LcnfExpr) -> usize {
    match expr {
        LcnfExpr::Return(_) => 1,
        LcnfExpr::Unreachable => 0,
        LcnfExpr::TailCall(_, args) => 2 + args.len(),
        LcnfExpr::Let { body, .. } => 1 + compute_inline_cost(body),
        LcnfExpr::Case { alts, default, .. } => {
            let alt_cost: usize = alts.iter().map(|a| compute_inline_cost(&a.body)).sum();
            let def_cost = default
                .as_ref()
                .map(|d| compute_inline_cost(d))
                .unwrap_or(0);
            2 + alt_cost + def_cost
        }
    }
}
/// Count the total number of let bindings in an LCNF expression.
pub(super) fn count_let_bindings(expr: &LcnfExpr) -> usize {
    match expr {
        LcnfExpr::Let { body, .. } => 1 + count_let_bindings(body),
        LcnfExpr::Case { alts, default, .. } => {
            let alt_count: usize = alts.iter().map(|a| count_let_bindings(&a.body)).sum();
            let def_count = default.as_ref().map(|d| count_let_bindings(d)).unwrap_or(0);
            alt_count + def_count
        }
        _ => 0,
    }
}
/// Collect all variable IDs used in an LCNF expression.
pub(super) fn collect_used_vars(expr: &LcnfExpr) -> HashSet<LcnfVarId> {
    let mut vars = HashSet::new();
    collect_used_vars_inner(expr, &mut vars);
    vars
}
/// Inner recursive variable collector.
pub(super) fn collect_used_vars_inner(expr: &LcnfExpr, vars: &mut HashSet<LcnfVarId>) {
    match expr {
        LcnfExpr::Let {
            id, value, body, ..
        } => {
            vars.insert(*id);
            collect_used_vars_let_value(value, vars);
            collect_used_vars_inner(body, vars);
        }
        LcnfExpr::Case {
            scrutinee,
            alts,
            default,
            ..
        } => {
            vars.insert(*scrutinee);
            for alt in alts {
                for p in &alt.params {
                    vars.insert(p.id);
                }
                collect_used_vars_inner(&alt.body, vars);
            }
            if let Some(def) = default {
                collect_used_vars_inner(def, vars);
            }
        }
        LcnfExpr::Return(arg) => {
            collect_used_vars_arg(arg, vars);
        }
        LcnfExpr::TailCall(func, args) => {
            collect_used_vars_arg(func, vars);
            for a in args {
                collect_used_vars_arg(a, vars);
            }
        }
        LcnfExpr::Unreachable => {}
    }
}
/// Collect used variables in a let-value.
pub(super) fn collect_used_vars_let_value(value: &LcnfLetValue, vars: &mut HashSet<LcnfVarId>) {
    match value {
        LcnfLetValue::App(func, args) => {
            collect_used_vars_arg(func, vars);
            for a in args {
                collect_used_vars_arg(a, vars);
            }
        }
        LcnfLetValue::Proj(_, _, var) => {
            vars.insert(*var);
        }
        LcnfLetValue::Ctor(_, _, args) => {
            for a in args {
                collect_used_vars_arg(a, vars);
            }
        }
        LcnfLetValue::FVar(var) => {
            vars.insert(*var);
        }
        LcnfLetValue::Lit(_)
        | LcnfLetValue::Erased
        | LcnfLetValue::Reset(_)
        | LcnfLetValue::Reuse(_, _, _, _) => {}
    }
}
/// Collect used variables in an argument.
pub(super) fn collect_used_vars_arg(arg: &LcnfArg, vars: &mut HashSet<LcnfVarId>) {
    if let LcnfArg::Var(id) = arg {
        vars.insert(*id);
    }
}
/// Substitute a variable with an argument throughout an expression.
pub(super) fn substitute_var(expr: &mut LcnfExpr, from: LcnfVarId, to: &LcnfArg) {
    match expr {
        LcnfExpr::Let { value, body, .. } => {
            substitute_var_in_value(value, from, to);
            substitute_var(body, from, to);
        }
        LcnfExpr::Case { alts, default, .. } => {
            for alt in alts.iter_mut() {
                substitute_var(&mut alt.body, from, to);
            }
            if let Some(def) = default.as_mut() {
                substitute_var(def, from, to);
            }
        }
        LcnfExpr::Return(arg) => {
            substitute_var_in_arg(arg, from, to);
        }
        LcnfExpr::TailCall(func, args) => {
            substitute_var_in_arg(func, from, to);
            for a in args.iter_mut() {
                substitute_var_in_arg(a, from, to);
            }
        }
        LcnfExpr::Unreachable => {}
    }
}
/// Substitute a variable in a let-value.
pub(super) fn substitute_var_in_value(value: &mut LcnfLetValue, from: LcnfVarId, to: &LcnfArg) {
    match value {
        LcnfLetValue::App(func, args) => {
            substitute_var_in_arg(func, from, to);
            for a in args.iter_mut() {
                substitute_var_in_arg(a, from, to);
            }
        }
        LcnfLetValue::Ctor(_, _, args) => {
            for a in args.iter_mut() {
                substitute_var_in_arg(a, from, to);
            }
        }
        LcnfLetValue::Proj(_, _, var) => {
            if *var == from {
                if let LcnfArg::Var(new_id) = to {
                    *var = *new_id;
                }
            }
        }
        LcnfLetValue::FVar(var) => {
            if *var == from {
                if let LcnfArg::Var(new_id) = to {
                    *var = *new_id;
                }
            }
        }
        LcnfLetValue::Lit(_)
        | LcnfLetValue::Erased
        | LcnfLetValue::Reset(_)
        | LcnfLetValue::Reuse(_, _, _, _) => {}
    }
}
/// Substitute a variable in an argument.
pub(super) fn substitute_var_in_arg(arg: &mut LcnfArg, from: LcnfVarId, to: &LcnfArg) {
    if let LcnfArg::Var(id) = arg {
        if *id == from {
            *arg = to.clone();
        }
    }
}
/// Pretty-print an LCNF function declaration (for debugging).
pub(super) fn pretty_print_decl(decl: &LcnfFunDecl) -> String {
    let mut out = String::new();
    out.push_str(&format!("def {} (", decl.name));
    for (i, p) in decl.params.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        if p.erased {
            out.push_str(&format!("[{}:{}]", p.name, p.ty));
        } else {
            out.push_str(&format!("{}:{}", p.name, p.ty));
        }
    }
    out.push_str(&format!(") : {} :=\n", decl.ret_type));
    out.push_str(&format!("  {}", decl.body));
    if decl.is_recursive {
        out.push_str(" [recursive]");
    }
    if decl.is_lifted {
        out.push_str(" [lifted]");
    }
    out
}
/// Pretty-print an entire LCNF module (for debugging).
pub(super) fn pretty_print_module(module: &LcnfModule) -> String {
    let mut out = String::new();
    out.push_str(&format!("-- Module: {}\n", module.name));
    out.push_str(&format!(
        "-- {} declarations, {} lifted\n",
        module.metadata.decl_count, module.metadata.lambdas_lifted
    ));
    out.push('\n');
    for decl in &module.fun_decls {
        out.push_str(&pretty_print_decl(decl));
        out.push_str("\n\n");
    }
    for ext in &module.extern_decls {
        out.push_str(&format!("extern {} (", ext.name));
        for (i, p) in ext.params.iter().enumerate() {
            if i > 0 {
                out.push_str(", ");
            }
            out.push_str(&format!("{}:{}", p.name, p.ty));
        }
        out.push_str(&format!(") : {}\n", ext.ret_type));
    }
    out
}
#[cfg(test)]
mod tests {
    use super::*;
    use oxilean_kernel::{Expr, Level, Literal, Name};
    pub(super) fn default_config() -> ToLcnfConfig {
        ToLcnfConfig::default()
    }
    pub(super) fn minimal_config() -> ToLcnfConfig {
        ToLcnfConfig::minimal()
    }
    #[test]
    pub(super) fn test_convert_literal_nat() {
        let config = default_config();
        let expr = Expr::Lit(Literal::Nat(42));
        let result = expr_to_lcnf(&expr, &config).expect("result LCNF conversion should succeed");
        match &result {
            LcnfExpr::Let { value, .. } => match value {
                LcnfLetValue::Lit(LcnfLit::Nat(n)) => assert_eq!(*n, 42),
                _ => panic!("Expected Lit(Nat(42)), got {:?}", value),
            },
            _ => {}
        }
    }
    #[test]
    pub(super) fn test_convert_literal_str() {
        let config = default_config();
        let expr = Expr::Lit(Literal::Str("hello".to_string()));
        let result = expr_to_lcnf(&expr, &config).expect("result LCNF conversion should succeed");
        match &result {
            LcnfExpr::Let { value, .. } => match value {
                LcnfLetValue::Lit(LcnfLit::Str(s)) => assert_eq!(s, "hello"),
                _ => panic!("Expected Lit(Str(\"hello\")), got {:?}", value),
            },
            _ => {}
        }
    }
    #[test]
    pub(super) fn test_convert_sort_erased() {
        let config = default_config();
        let expr = Expr::Sort(Level::zero());
        let result = expr_to_lcnf(&expr, &config).expect("result LCNF conversion should succeed");
        match &result {
            LcnfExpr::Return(LcnfArg::Erased) => {}
            _ => panic!("Expected Erased for Sort when erase_types=true"),
        }
    }
    #[test]
    pub(super) fn test_convert_sort_not_erased() {
        let config = minimal_config();
        let expr = Expr::Sort(Level::zero());
        let result = expr_to_lcnf(&expr, &config).expect("result LCNF conversion should succeed");
        match &result {
            LcnfExpr::Return(LcnfArg::Erased) => {
                panic!("Should not be erased with erase_types=false")
            }
            _ => {}
        }
    }
    #[test]
    pub(super) fn test_convert_const() {
        let config = default_config();
        let expr = Expr::Const(Name::str("Nat"), vec![]);
        let result = expr_to_lcnf(&expr, &config).expect("result LCNF conversion should succeed");
        match &result {
            LcnfExpr::Return(LcnfArg::Var(_)) => {}
            _ => panic!("Expected Return(Var) for Const"),
        }
    }
    #[test]
    pub(super) fn test_convert_simple_app() {
        let config = default_config();
        let func = Expr::Const(Name::str("Nat.succ"), vec![]);
        let arg = Expr::Lit(Literal::Nat(0));
        let expr = Expr::App(Box::new(func), Box::new(arg));
        let result = expr_to_lcnf(&expr, &config).expect("result LCNF conversion should succeed");
        assert!(matches!(
            &result,
            LcnfExpr::TailCall(_, _) | LcnfExpr::Let { .. }
        ));
    }
    #[test]
    pub(super) fn test_convert_let_binding() {
        let config = minimal_config();
        let expr = Expr::Let(
            Name::str("x"),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
            Box::new(Expr::Lit(Literal::Nat(5))),
            Box::new(Expr::BVar(0)),
        );
        let result = expr_to_lcnf(&expr, &config);
        assert!(result.is_ok());
    }
    #[test]
    pub(super) fn test_convert_projection() {
        let config = default_config();
        let base = Expr::Const(Name::str("p"), vec![]);
        let expr = Expr::Proj(Name::str("Prod"), 0, Box::new(base));
        let result = expr_to_lcnf(&expr, &config);
        assert!(result.is_ok());
    }
    #[test]
    pub(super) fn test_convert_pi_erased() {
        let config = default_config();
        let expr = Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
        );
        let result = expr_to_lcnf(&expr, &config).expect("result LCNF conversion should succeed");
        match &result {
            LcnfExpr::Return(LcnfArg::Erased) => {}
            _ => panic!("Expected Erased for Pi type with erase_types=true"),
        }
    }
    #[test]
    pub(super) fn test_type_conversion_nat() {
        let state = ToLcnfState::new(&default_config());
        let expr = Expr::Const(Name::str("Nat"), vec![]);
        let ty = convert_type(&expr, &state);
        assert_eq!(ty, LcnfType::Nat);
    }
    #[test]
    pub(super) fn test_type_conversion_string() {
        let state = ToLcnfState::new(&default_config());
        let expr = Expr::Const(Name::str("String"), vec![]);
        let ty = convert_type(&expr, &state);
        assert_eq!(ty, LcnfType::LcnfString);
    }
    #[test]
    pub(super) fn test_type_conversion_prop() {
        let state = ToLcnfState::new(&default_config());
        let expr = Expr::Sort(Level::zero());
        let ty = convert_type(&expr, &state);
        assert_eq!(ty, LcnfType::Irrelevant);
    }
    #[test]
    pub(super) fn test_type_conversion_arrow() {
        let state = ToLcnfState::new(&default_config());
        let expr = Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
        );
        let ty = convert_type(&expr, &state);
        match ty {
            LcnfType::Fun(params, ret) => {
                assert_eq!(params.len(), 1);
                assert_eq!(params[0], LcnfType::Nat);
                assert_eq!(*ret, LcnfType::Nat);
            }
            _ => panic!("Expected Fun type for Nat -> Nat"),
        }
    }
    #[test]
    pub(super) fn test_name_mangling() {
        let name = Name::str("Nat").append_str("add");
        assert_eq!(mangle_name(&name), "Nat_add");
    }
    #[test]
    pub(super) fn test_fresh_var_generation() {
        let mut state = ToLcnfState::new(&default_config());
        let v1 = state.fresh_var();
        let v2 = state.fresh_var();
        assert_ne!(v1, v2);
        assert_eq!(v1.0 + 1, v2.0);
    }
    #[test]
    pub(super) fn test_bvar_stack() {
        let mut state = ToLcnfState::new(&default_config());
        let id1 = state.fresh_var();
        let id2 = state.fresh_var();
        state.push_bvar(id1, "x");
        state.push_bvar(id2, "y");
        assert_eq!(state.lookup_bvar(0), Some(id2));
        assert_eq!(state.lookup_bvar(1), Some(id1));
        assert_eq!(state.lookup_bvar(2), None);
        state.pop_bvar();
        assert_eq!(state.lookup_bvar(0), Some(id1));
    }
    #[test]
    pub(super) fn test_estimate_expr_size() {
        let lit = Expr::Lit(Literal::Nat(1));
        assert_eq!(estimate_expr_size(&lit), 1);
        let app = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(Expr::Lit(Literal::Nat(1))),
        );
        assert_eq!(estimate_expr_size(&app), 3);
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
            Box::new(Expr::BVar(0)),
        );
        assert_eq!(estimate_expr_size(&lam), 3);
    }
    #[test]
    pub(super) fn test_flatten_app() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Lit(Literal::Nat(1));
        let b = Expr::Lit(Literal::Nat(2));
        let app1 = Expr::App(Box::new(f), Box::new(a));
        let app2 = Expr::App(Box::new(app1), Box::new(b));
        let inner_func = match &app2 {
            Expr::App(func, _) => func,
            _ => panic!("Expected App"),
        };
        let inner_arg = match &app2 {
            Expr::App(_, arg) => arg,
            _ => panic!("Expected App"),
        };
        let (head, args) = flatten_app(inner_func, inner_arg);
        assert!(matches!(head, Expr::Const(_, _)));
        assert_eq!(args.len(), 2);
    }
    #[test]
    pub(super) fn test_proof_eraser() {
        let mut eraser = ProofEraser::new();
        let proof_id = LcnfVarId(100);
        eraser.proof_vars.insert(proof_id);
        let mut expr = LcnfExpr::Return(LcnfArg::Var(proof_id));
        eraser.erase_expr(&mut expr);
        assert_eq!(eraser.erased_count, 1);
        match &expr {
            LcnfExpr::Return(LcnfArg::Erased) => {}
            _ => panic!("Expected erased return"),
        }
    }
    #[test]
    pub(super) fn test_decl_to_lcnf_simple() {
        let config = default_config();
        let name = Name::str("id");
        let params = vec![(Name::str("x"), Expr::Const(Name::str("Nat"), vec![]))];
        let body = Expr::BVar(0);
        let result = decl_to_lcnf(&name, &params, &body, &config);
        assert!(result.is_ok());
        let decl = result.expect("decl should be Some/Ok");
        assert_eq!(decl.name, "id");
        assert_eq!(decl.params.len(), 1);
    }
    #[test]
    pub(super) fn test_module_to_lcnf_empty() {
        let config = default_config();
        let decls: Vec<(Name, Vec<(Name, Expr)>, Expr)> = vec![];
        let result = module_to_lcnf(&decls, &config);
        assert!(result.is_ok());
        let module = result.expect("module should be Some/Ok");
        assert!(module.fun_decls.is_empty());
    }
    #[test]
    pub(super) fn test_module_to_lcnf_single() {
        let config = default_config();
        let decls = vec![(Name::str("const42"), vec![], Expr::Lit(Literal::Nat(42)))];
        let result = module_to_lcnf(&decls, &config);
        assert!(result.is_ok());
        let module = result.expect("module should be Some/Ok");
        assert!(!module.fun_decls.is_empty());
        assert_eq!(module.metadata.decl_count, 1);
    }
    #[test]
    pub(super) fn test_conversion_error_display() {
        let err = ConversionError::UnboundVariable("x".to_string());
        assert_eq!(err.to_string(), "Unbound variable: x");
        let err = ConversionError::DepthLimitExceeded(1000);
        assert_eq!(err.to_string(), "Depth limit exceeded: 1000");
        let err = ConversionError::UnsupportedExpr("Rec".to_string());
        assert_eq!(err.to_string(), "Unsupported expression: Rec");
    }
    #[test]
    pub(super) fn test_conversion_stats_default() {
        let stats = ConversionStats::default();
        assert_eq!(stats.exprs_visited, 0);
        assert_eq!(stats.let_bindings_generated, 0);
        assert_eq!(stats.lambdas_lifted, 0);
    }
    #[test]
    pub(super) fn test_config_variants() {
        let full = ToLcnfConfig::full();
        assert!(full.erase_proofs);
        assert!(full.erase_types);
        assert!(full.lambda_lift);
        let minimal = ToLcnfConfig::minimal();
        assert!(!minimal.erase_proofs);
        assert!(!minimal.erase_types);
        assert!(!minimal.lambda_lift);
        let debug = ToLcnfConfig::debug();
        assert!(debug.debug_names);
    }
    #[test]
    pub(super) fn test_level_to_u64() {
        assert_eq!(level_to_u64(&Level::zero()), 0);
        assert_eq!(level_to_u64(&Level::succ(Level::zero())), 1);
        assert_eq!(level_to_u64(&Level::succ(Level::succ(Level::zero()))), 2);
        assert_eq!(
            level_to_u64(&Level::max(
                Level::succ(Level::zero()),
                Level::succ(Level::succ(Level::zero()))
            )),
            2
        );
    }
    #[test]
    pub(super) fn test_has_bvar_ref() {
        let expr = Expr::BVar(0);
        assert!(has_bvar_ref(&expr, 0));
        assert!(!has_bvar_ref(&expr, 1));
        let expr2 = Expr::Const(Name::str("Nat"), vec![]);
        assert!(!has_bvar_ref(&expr2, 0));
        let app = Expr::App(
            Box::new(Expr::BVar(0)),
            Box::new(Expr::Lit(Literal::Nat(1))),
        );
        assert!(has_bvar_ref(&app, 0));
    }
    #[test]
    pub(super) fn test_collect_used_vars() {
        let id = LcnfVarId(5);
        let expr = LcnfExpr::Return(LcnfArg::Var(id));
        let vars = collect_used_vars(&expr);
        assert!(vars.contains(&id));
    }
    #[test]
    pub(super) fn test_compute_inline_cost() {
        let simple = LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(1)));
        assert_eq!(compute_inline_cost(&simple), 1);
        let let_expr = LcnfExpr::Let {
            id: LcnfVarId(0),
            name: "x".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Lit(LcnfLit::Nat(1)),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(0)))),
        };
        assert_eq!(compute_inline_cost(&let_expr), 2);
    }
    #[test]
    pub(super) fn test_count_let_bindings() {
        let simple = LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(1)));
        assert_eq!(count_let_bindings(&simple), 0);
        let nested = LcnfExpr::Let {
            id: LcnfVarId(0),
            name: "x".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Lit(LcnfLit::Nat(1)),
            body: Box::new(LcnfExpr::Let {
                id: LcnfVarId(1),
                name: "y".to_string(),
                ty: LcnfType::Nat,
                value: LcnfLetValue::Lit(LcnfLit::Nat(2)),
                body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(1)))),
            }),
        };
        assert_eq!(count_let_bindings(&nested), 2);
    }
    #[test]
    pub(super) fn test_substitute_var() {
        let from = LcnfVarId(0);
        let to = LcnfArg::Var(LcnfVarId(10));
        let mut expr = LcnfExpr::Return(LcnfArg::Var(from));
        substitute_var(&mut expr, from, &to);
        match &expr {
            LcnfExpr::Return(LcnfArg::Var(id)) => assert_eq!(*id, LcnfVarId(10)),
            _ => panic!("Expected substituted var"),
        }
    }
    #[test]
    pub(super) fn test_lambda_lifter_free_vars() {
        let lifter = LambdaLifter::new(8);
        let id0 = LcnfVarId(0);
        let id1 = LcnfVarId(1);
        let expr = LcnfExpr::Let {
            id: id0,
            name: "x".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Lit(LcnfLit::Nat(1)),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(id1))),
        };
        let free = lifter.free_vars_of_expr(&expr);
        assert!(free.contains(&id1));
        assert!(!free.contains(&id0));
    }
    #[test]
    pub(super) fn test_closure_converter_fresh_name() {
        let mut conv = ClosureConverter::new();
        let n1 = conv.fresh_closure_name();
        let n2 = conv.fresh_closure_name();
        assert_ne!(n1, n2);
        assert!(n1.starts_with("Closure_"));
    }
    #[test]
    pub(super) fn test_closure_converter_build_env() {
        let mut conv = ClosureConverter::new();
        let captured = vec![
            (LcnfVarId(0), LcnfType::Nat),
            (LcnfVarId(1), LcnfType::LcnfString),
        ];
        let (name, val) = conv.build_closure_env(&captured);
        assert!(name.starts_with("Closure_"));
        match val {
            LcnfLetValue::Ctor(_, tag, args) => {
                assert_eq!(tag, 0);
                assert_eq!(args.len(), 2);
            }
            _ => panic!("Expected Ctor"),
        }
        assert!(conv.closure_structs.contains_key(&name));
    }
    #[test]
    pub(super) fn test_build_fun_type() {
        let ty = build_fun_type(&[LcnfType::Nat, LcnfType::Nat], &LcnfType::Nat);
        match ty {
            LcnfType::Fun(params, ret) => {
                assert_eq!(params.len(), 2);
                assert_eq!(*ret, LcnfType::Nat);
            }
            _ => panic!("Expected Fun type"),
        }
        let ty2 = build_fun_type(&[], &LcnfType::Nat);
        assert_eq!(ty2, LcnfType::Nat);
    }
    #[test]
    pub(super) fn test_pretty_print_decl_format() {
        let decl = LcnfFunDecl {
            name: "test_fn".to_string(),
            original_name: Some(Name::str("test_fn")),
            params: vec![LcnfParam {
                id: LcnfVarId(0),
                name: "x".to_string(),
                ty: LcnfType::Nat,
                erased: false,
                borrowed: false,
            }],
            ret_type: LcnfType::Nat,
            body: LcnfExpr::Return(LcnfArg::Var(LcnfVarId(0))),
            is_recursive: false,
            is_lifted: false,
            inline_cost: 1,
        };
        let output = pretty_print_decl(&decl);
        assert!(output.contains("def test_fn"));
        assert!(output.contains("x:nat"));
    }
    #[test]
    pub(super) fn test_infer_return_type() {
        let nat_ret = LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(1)));
        assert_eq!(infer_return_type(&nat_ret), LcnfType::Nat);
        let str_ret = LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Str("hi".into())));
        assert_eq!(infer_return_type(&str_ret), LcnfType::LcnfString);
        let erased_ret = LcnfExpr::Return(LcnfArg::Erased);
        assert_eq!(infer_return_type(&erased_ret), LcnfType::Erased);
    }
    #[test]
    pub(super) fn test_conversion_stats_display() {
        let stats = ConversionStats {
            exprs_visited: 10,
            let_bindings_generated: 5,
            lambdas_lifted: 2,
            proofs_erased: 1,
            types_erased: 3,
            closures_converted: 0,
            max_depth: 4,
            tail_calls_detected: 1,
            fresh_vars_allocated: 8,
            free_var_computations: 2,
        };
        let output = format!("{}", stats);
        assert!(output.contains("Expressions visited:   10"));
        assert!(output.contains("Lambdas lifted:        2"));
    }
    #[test]
    pub(super) fn test_flatten_pi_type() {
        let state = ToLcnfState::new(&default_config());
        let pi = Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("y"),
                Box::new(Expr::Const(Name::str("Nat"), vec![])),
                Box::new(Expr::Const(Name::str("Nat"), vec![])),
            )),
        );
        let (params, ret) = flatten_pi_type(&pi, &state);
        assert_eq!(params.len(), 2);
        assert_eq!(params[0].1, LcnfType::Nat);
        assert_eq!(params[1].1, LcnfType::Nat);
        assert_eq!(ret, LcnfType::Nat);
    }
    #[test]
    pub(super) fn test_depth_limit() {
        let mut state = ToLcnfState::new(&default_config());
        state.max_depth = 5;
        for _ in 0..5 {
            assert!(state.enter_depth().is_ok());
        }
        assert!(state.enter_depth().is_err());
    }
    #[test]
    pub(super) fn test_binder_info_str() {
        assert_eq!(binder_info_str(&BinderInfo::Default), "explicit");
        assert_eq!(binder_info_str(&BinderInfo::Implicit), "implicit");
        assert_eq!(
            binder_info_str(&BinderInfo::StrictImplicit),
            "strict_implicit"
        );
        assert_eq!(binder_info_str(&BinderInfo::InstImplicit), "inst_implicit");
    }
    #[test]
    pub(super) fn test_convert_lambda_simple() {
        let config = minimal_config();
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
            Box::new(Expr::BVar(0)),
        );
        let result = expr_to_lcnf(&lam, &config);
        assert!(result.is_ok(), "Lambda conversion should succeed");
    }
    #[test]
    pub(super) fn test_module_multiple_decls() {
        let config = default_config();
        let decls = vec![
            (
                Name::str("f"),
                vec![(Name::str("x"), Expr::Const(Name::str("Nat"), vec![]))],
                Expr::BVar(0),
            ),
            (Name::str("g"), vec![], Expr::Lit(Literal::Nat(0))),
        ];
        let result = module_to_lcnf(&decls, &config);
        assert!(result.is_ok());
        let module = result.expect("module should be Some/Ok");
        assert_eq!(module.metadata.decl_count, 2);
    }
    #[test]
    pub(super) fn test_nested_app_conversion() {
        let config = default_config();
        let expr = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Nat.add"), vec![])),
                Box::new(Expr::Lit(Literal::Nat(1))),
            )),
            Box::new(Expr::Lit(Literal::Nat(2))),
        );
        let result = expr_to_lcnf(&expr, &config);
        assert!(result.is_ok());
    }
    #[test]
    pub(super) fn test_proof_erasure_let() {
        let mut eraser = ProofEraser::new();
        let proof_id = LcnfVarId(50);
        let mut expr = LcnfExpr::Let {
            id: proof_id,
            name: "proof".to_string(),
            ty: LcnfType::Irrelevant,
            value: LcnfLetValue::Lit(LcnfLit::Nat(0)),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(proof_id))),
        };
        eraser.erase_expr(&mut expr);
        match &expr {
            LcnfExpr::Let { value, body, .. } => {
                assert!(matches!(value, LcnfLetValue::Erased));
                match body.as_ref() {
                    LcnfExpr::Return(LcnfArg::Erased) => {}
                    _ => panic!("Expected erased return in body"),
                }
            }
            _ => panic!("Expected Let"),
        }
    }
    #[test]
    pub(super) fn test_wrap_pending_lets() {
        let mut state = ToLcnfState::new(&default_config());
        let _id1 = state.emit_let("a", LcnfType::Nat, LcnfLetValue::Lit(LcnfLit::Nat(1)));
        let id2 = state.emit_let("b", LcnfType::Nat, LcnfLetValue::Lit(LcnfLit::Nat(2)));
        let terminal = LcnfExpr::Return(LcnfArg::Var(id2));
        let wrapped = state.wrap_pending_lets(terminal);
        match &wrapped {
            LcnfExpr::Let { body, .. } => match body.as_ref() {
                LcnfExpr::Let { body: inner, .. } => {
                    assert!(matches!(inner.as_ref(), LcnfExpr::Return(_)));
                }
                _ => panic!("Expected nested let"),
            },
            _ => panic!("Expected outer let"),
        }
    }
}
