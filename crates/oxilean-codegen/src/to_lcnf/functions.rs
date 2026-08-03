//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use oxilean_kernel::Node;
use oxilean_kernel::{BinderInfo, Expr, Level, LevelView, Literal, Name, NameView};
use std::collections::{HashMap, HashSet, VecDeque};

use super::types::{
    ClosureConverter, ConversionError, ConversionStats, LambdaLifter, ProofEraser, ToLcnfConfig,
    ToLcnfState,
};

/// Convert a kernel `Name` to its string representation for LCNF.
pub(super) fn name_to_string(name: &Name) -> String {
    match name.view() {
        NameView::Anonymous => "_".to_string(),
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
    match level.view() {
        LevelView::Zero => 0,
        LevelView::Succ(inner) => level_to_u64(inner).saturating_add(1),
        LevelView::Max(l1, l2) => level_to_u64(l1).max(level_to_u64(l2)),
        LevelView::IMax(_, l2) => {
            let v2 = level_to_u64(l2);
            if v2 == 0 { 0 } else { v2 }
        }
        LevelView::Param(_) => 1,
        LevelView::MVar(_) => 1,
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
        note_const_type(state, &mangled, var_id);
        state.name_map.insert(mangled, var_id);
        Ok(LcnfExpr::Return(LcnfArg::Var(var_id)))
    }
}
/// Copy a constant's declared type onto the variable that stands for
/// it, so a later `arg_lcnf_type` on that variable answers.
///
/// Only meaningful when the caller supplied signatures; a no-op
/// otherwise.
pub(super) fn note_const_type(state: &mut ToLcnfState, mangled: &str, id: LcnfVarId) {
    if let Some(ty) = state.const_type(mangled).cloned() {
        state.record_var_type(id, &ty);
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
        let result_ty = app_result_type(head, &lcnf_args, state);
        let app_val = LcnfLetValue::App(head_arg, lcnf_args);
        let result_id = state.emit_let("app", result_ty, app_val);
        let result = LcnfExpr::Return(LcnfArg::Var(result_id));
        Ok(state.wrap_pending_lets(result))
    }
}
/// The mangled kernel name an application head denotes, if it denotes
/// one at all.
///
/// The `Proj`-over-`Const` case mirrors [`convert_proj`]'s fast path:
/// `oxilean-elab` lowers a namespaced reference like `UInt64.add` as
/// `Proj("add", _, Const("UInt64"))`, and both have to resolve to the
/// same composite name or the signature lookup misses.
pub(super) fn app_head_name(head: &Expr) -> Option<String> {
    match head {
        Expr::Const(name, _) => Some(mangle_name(name)),
        Expr::Proj(field, _, base) => match base.as_ref() {
            Expr::Const(base_name, _) => Some(mangle_name(
                &base_name.clone().append_str(name_to_string(field)),
            )),
            _ => None,
        },
        _ => None,
    }
}
/// The LCNF type of an already-converted argument, or `None` when it
/// carries no type information.
pub(super) fn arg_lcnf_type(arg: &LcnfArg, state: &ToLcnfState) -> Option<LcnfType> {
    match arg {
        LcnfArg::Var(id) => state.var_type(*id).cloned(),
        LcnfArg::Lit(LcnfLit::Nat(_)) => Some(LcnfType::Nat),
        LcnfArg::Lit(LcnfLit::Str(_)) => Some(LcnfType::LcnfString),
        // A signed literal has no `LcnfType` of its own — `Int` is not
        // in the enum, and picking a width here would be the same
        // mistake `app_result_type` refuses to make for `Nat`.
        LcnfArg::Lit(LcnfLit::Int(_)) | LcnfArg::Erased | LcnfArg::Type(_) => None,
    }
}
/// How a Lean stdlib typeclass projection relates its result type to
/// its operands' — for the projections whose env entry carries no
/// usable signature.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum TcResultShape {
    /// Result has the same type as the operands: `HAdd.hAdd`,
    /// `HShiftLeft.hShiftLeft`, `Neg.neg`, …
    Homogeneous,
    /// Result is `Bool` regardless of operand type: `LT.lt`,
    /// `BEq.beq`, `Decidable.decide`, …
    Boolean,
}
/// Classify a mangled typeclass-projection name.
///
/// Deliberately the *smallest* table that covers what actually reaches
/// `to_lcnf`. An unclassified head falls through to
/// `LcnfType::Object`, which is what every application got before, so
/// under-listing is safe; over-listing is not, because a wrong entry
/// claims a type the value does not have.
///
/// Three spellings that look like they belong here do not: `>` / `>=`
/// never survive elaboration (Lean desugars them to `LT.lt` / `LE.le`
/// with the operands swapped), and `&&` / `||` lower to bare
/// `and` / `or`, which no elaboration environment on this path
/// declares — they fail earlier with `NameNotFound`.
pub(super) fn tc_projection_result_shape(mangled: &str) -> Option<TcResultShape> {
    match mangled {
        // Arithmetic and bitwise — homogeneous in Lean's stdlib
        // instances for every type the rust-transpile path admits
        // (sized ints, floats). The shift operators are homogeneous
        // in Lean too: `HShiftLeft UInt64 UInt64 UInt64`.
        "HAdd_hAdd"
        | "HSub_hSub"
        | "HMul_hMul"
        | "HDiv_hDiv"
        | "HMod_hMod"
        | "HPow_hPow"
        | "HAnd_hAnd"
        | "HOr_hOr"
        | "HXor_hXor"
        | "HShiftLeft_hShiftLeft"
        | "HShiftRight_hShiftRight"
        | "Neg_neg" => Some(TcResultShape::Homogeneous),
        // Comparison. `Not_not` is the unary member: `a ≠ b` lowers to
        // `Not.not (Eq.eq a b)`, so leaving `Eq_eq` out would also put
        // a non-`bool` under Rust's `!`.
        "LT_lt" | "LE_le" | "BEq_beq" | "Eq_eq" | "Not_not" => Some(TcResultShape::Boolean),
        _ => None,
    }
}
/// Do a projection's operands have to agree with each other?
///
/// Distinct from [`TcResultShape`], which relates operands to the
/// *result*. `HPow.hPow` is `Homogeneous` in its result — `x ^ n` has
/// `x`'s type — while its exponent is routinely a different type, and
/// the shift operators are the same shape. Those are excluded here;
/// the unary projections have no sibling to agree with, so listing
/// them would be vacuous.
pub(super) fn operands_agree(mangled: &str) -> bool {
    matches!(
        mangled,
        "HAdd_hAdd"
            | "HSub_hSub"
            | "HMul_hMul"
            | "HDiv_hDiv"
            | "HMod_hMod"
            | "HAnd_hAnd"
            | "HOr_hOr"
            | "HXor_hXor"
            | "LT_lt"
            | "LE_le"
            | "BEq_beq"
            | "Eq_eq"
    )
}

/// Second pass: push types *backwards*, from an application into its
/// own operands.
///
/// [`app_result_type`] runs during conversion and can only look at what
/// it has already seen — operands, then result. That is one-directional
/// and one-pass, so it cannot type
///
/// ```lean
/// def g (n : UInt8) : Bool := if 1 + 1 < n then true else false
/// ```
///
/// The sum `1 + 1` has two literal operands and no type of its own; the
/// type it needs belongs to the *enclosing* comparison, which does not
/// exist yet when the sum's `Let` is emitted. The result was
/// `Box<dyn std::any::Any>` and Rust rejected it.
///
/// Running after conversion, the whole body is visible, so the
/// comparison's other operand answers: `LT.lt(_x5, _x0)` with
/// `_x0 : u8` forces `_x5 : u8`, and `_x5 = 1 + 1` takes it.
///
/// Only `Object` bindings are filled in — a type the conversion already
/// inferred is evidence, and this pass has no better information about
/// it. That also makes the pass monotone (types only ever go from
/// unknown to known), so the fixpoint terminates; `MAX_ROUNDS` is
/// insurance, not the mechanism.
///
/// `const_names` maps the head variable of an application back to its
/// kernel name, which is how a projection is recognised — the same map
/// [`decl_to_lcnf_full`] already returns for identifier emission.
pub fn propagate_operand_types(
    decl: &mut LcnfFunDecl,
    const_names: &std::collections::HashMap<LcnfVarId, String>,
) {
    const MAX_ROUNDS: usize = 8;

    let mut known: HashMap<LcnfVarId, LcnfType> = HashMap::new();
    for p in &decl.params {
        if is_informative_type(&p.ty) {
            known.insert(p.id, p.ty.clone());
        }
    }
    collect_known_types(&decl.body, &mut known);

    for _ in 0..MAX_ROUNDS {
        let mut changed = false;
        solve_operand_types(&decl.body, const_names, &mut known, &mut changed);
        if !changed {
            break;
        }
    }
    apply_known_types(&mut decl.body, &known);
}

/// Seed [`propagate_operand_types`] with every binding that already has
/// a type worth trusting.
fn collect_known_types(expr: &LcnfExpr, known: &mut HashMap<LcnfVarId, LcnfType>) {
    match expr {
        LcnfExpr::Let { id, ty, body, .. } => {
            if is_informative_type(ty) {
                known.insert(*id, ty.clone());
            }
            collect_known_types(body, known);
        }
        LcnfExpr::Case {
            scrutinee,
            scrutinee_ty,
            alts,
            default,
        } => {
            if is_informative_type(scrutinee_ty) {
                known.insert(*scrutinee, scrutinee_ty.clone());
            }
            for alt in alts {
                for p in &alt.params {
                    if is_informative_type(&p.ty) {
                        known.insert(p.id, p.ty.clone());
                    }
                }
                collect_known_types(&alt.body, known);
            }
            if let Some(d) = default {
                collect_known_types(d, known);
            }
        }
        LcnfExpr::Return(_) | LcnfExpr::Unreachable | LcnfExpr::TailCall(_, _) => {}
    }
}

/// One round of [`propagate_operand_types`]'s fixpoint.
fn solve_operand_types(
    expr: &LcnfExpr,
    const_names: &std::collections::HashMap<LcnfVarId, String>,
    known: &mut HashMap<LcnfVarId, LcnfType>,
    changed: &mut bool,
) {
    match expr {
        LcnfExpr::Let {
            id, value, body, ..
        } => {
            if let LcnfLetValue::App(LcnfArg::Var(head), args) = value {
                if let Some(name) = const_names.get(head) {
                    let operand_vars: Vec<LcnfVarId> = args
                        .iter()
                        .filter_map(|a| match a {
                            LcnfArg::Var(v) => Some(*v),
                            _ => None,
                        })
                        .collect();

                    if operands_agree(name) {
                        // Any operand that knows its type settles the
                        // rest, and — for a homogeneous projection —
                        // the result too.
                        let settled = operand_vars
                            .iter()
                            .find_map(|v| known.get(v).cloned())
                            .or_else(|| {
                                matches!(
                                    tc_projection_result_shape(name),
                                    Some(TcResultShape::Homogeneous)
                                )
                                .then(|| known.get(id).cloned())
                                .flatten()
                            });
                        if let Some(ty) = settled {
                            for v in &operand_vars {
                                if !known.contains_key(v) {
                                    known.insert(*v, ty.clone());
                                    *changed = true;
                                }
                            }
                            if matches!(
                                tc_projection_result_shape(name),
                                Some(TcResultShape::Homogeneous)
                            ) && !known.contains_key(id)
                            {
                                known.insert(*id, ty);
                                *changed = true;
                            }
                        }
                    }
                }
            }
            solve_operand_types(body, const_names, known, changed);
        }
        LcnfExpr::Case { alts, default, .. } => {
            for alt in alts {
                solve_operand_types(&alt.body, const_names, known, changed);
            }
            if let Some(d) = default {
                solve_operand_types(d, const_names, known, changed);
            }
        }
        LcnfExpr::Return(_) | LcnfExpr::Unreachable | LcnfExpr::TailCall(_, _) => {}
    }
}

/// Write [`propagate_operand_types`]'s conclusions back into the tree.
fn apply_known_types(expr: &mut LcnfExpr, known: &HashMap<LcnfVarId, LcnfType>) {
    match expr {
        LcnfExpr::Let { id, ty, body, .. } => {
            if !is_informative_type(ty) {
                if let Some(t) = known.get(id) {
                    *ty = t.clone();
                }
            }
            apply_known_types(body, known);
        }
        LcnfExpr::Case {
            scrutinee,
            scrutinee_ty,
            alts,
            default,
        } => {
            if !is_informative_type(scrutinee_ty) {
                if let Some(t) = known.get(scrutinee) {
                    *scrutinee_ty = t.clone();
                }
            }
            for alt in alts {
                for p in &mut alt.params {
                    if !is_informative_type(&p.ty) {
                        if let Some(t) = known.get(&p.id) {
                            p.ty = t.clone();
                        }
                    }
                }
                apply_known_types(&mut alt.body, known);
            }
            if let Some(d) = default {
                apply_known_types(d, known);
            }
        }
        LcnfExpr::Return(_) | LcnfExpr::Unreachable | LcnfExpr::TailCall(_, _) => {}
    }
}
/// Infer the LCNF type of an application's result.
///
/// Two sources, tried in order:
///
/// 1. **The callee's declared signature**, when the caller supplied
///    one via [`env_const_types`]. `double : UInt64 → UInt64` converts
///    to `Fun([UInt64], UInt64)`; peeling one `Fun` layer per supplied
///    argument yields `UInt64`. Peeling stops early if the arguments
///    outrun the arrows (an over-application through an opaque result
///    type), and a partial application legitimately leaves a `Fun`
///    behind.
///
/// 2. **The operands**, when the head is a typeclass projection.
///    Lean's `HAdd.hAdd` reaches `to_lcnf` as an axiom of type `Type`
///    — the arrows that would carry `α → α → α` are not there to peel,
///    and no instance has been resolved. For the homogeneous
///    projections the result type *is* an operand's type, so the first
///    operand with a known one answers it.
///
///    Only *variables* answer. A literal's own type is not evidence:
///    Lean coerces integer literals to whatever the surrounding
///    instance demands, so reading `1` as `Nat` types `1 + 1` as `u64`
///    even inside a `UInt8` function, and types `-1` as `u64` — the
///    one type Rust's unary `-` rejects. When every operand is a
///    literal we do not know, and say so.
///
/// Falls back to `LcnfType::Object` — the pre-2026-08-02 behaviour for
/// every application — when neither source knows.
pub(super) fn app_result_type(head: &Expr, args: &[LcnfArg], state: &ToLcnfState) -> LcnfType {
    let Some(name) = app_head_name(head) else {
        return LcnfType::Object;
    };
    // 1. Declared signature.
    if let Some(sig) = state.const_type(&name) {
        let mut cur = sig;
        let mut peeled = 0usize;
        while peeled < args.len() {
            match cur {
                LcnfType::Fun(params, ret) => {
                    // A `Fun` node may carry several params at once
                    // (`flatten_pi_type` builds those); consume them
                    // together.
                    if peeled + params.len() > args.len() {
                        break;
                    }
                    peeled += params.len();
                    cur = ret;
                }
                _ => break,
            }
        }
        if peeled == args.len() && !matches!(cur, LcnfType::Object) {
            return cur.clone();
        }
    }
    // 2. Operand-driven, for typeclass projections.
    match tc_projection_result_shape(&name) {
        Some(TcResultShape::Boolean) => LcnfType::Ctor("Bool".to_string(), Vec::new()),
        Some(TcResultShape::Homogeneous) => {
            for arg in args {
                let LcnfArg::Var(id) = arg else { continue };
                match state.var_type(*id) {
                    // An erasure marker is a type or instance argument,
                    // not an operand. Lean's fully-elaborated
                    // `HAdd.hAdd α β γ inst a b` puts three of them
                    // ahead of the real operands; taking one would
                    // claim the sum is `()`.
                    Some(LcnfType::Erased | LcnfType::Irrelevant | LcnfType::Unit) | None => {}
                    Some(ty) => return ty.clone(),
                }
            }
            LcnfType::Object
        }
        None => LcnfType::Object,
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
        Expr::Lit(Literal::Nat(n)) => {
            Ok(LcnfArg::Lit(LcnfLit::Nat(n.to_u64().unwrap_or(u64::MAX))))
        }
        Expr::Lit(Literal::Str(s)) => Ok(LcnfArg::Lit(LcnfLit::Str(s.clone()))),
        Expr::Const(name, _levels) => {
            let mangled = mangle_name(name);
            if let Some(var_id) = state.lookup_name(&mangled) {
                Ok(LcnfArg::Var(var_id))
            } else {
                let var_id = state.fresh_named_var(&mangled);
                note_const_type(state, &mangled, var_id);
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
        // OX7 (1b-β, 2026-05-27): mirror the
        // `convert_proj` fast path here. Without this,
        // a `Proj("add", _, Const("UInt64"))` head in
        // an App falls through to the general arm,
        // emits a let-binding via `convert_expr`, and
        // the App's head ends up as a fresh `_xN`
        // placeholder instead of the composite kernel
        // name.
        Expr::Proj(name, _idx, base) => {
            if let Expr::Const(base_name, _) = base.as_ref() {
                let mangled = mangle_name(&base_name.clone().append_str(name_to_string(name)));
                if let Some(var_id) = state.lookup_name(&mangled) {
                    return Ok(LcnfArg::Var(var_id));
                }
                let var_id = state.fresh_named_var(&mangled);
                note_const_type(state, &mangled, var_id);
                state.name_map.insert(mangled, var_id);
                return Ok(LcnfArg::Var(var_id));
            }
            let lcnf = convert_expr(expr, state, false)?;
            let id = bind_expr_to_var(lcnf, state, hint)?;
            Ok(LcnfArg::Var(id))
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
///
/// # The `Let`-spine case
///
/// A sub-conversion that ended in `wrap_pending_lets` hands us a
/// finished `Let` chain rather than a bare `Return`. The chain's
/// bindings are *not* on `state.pending_lets` any more — wrapping
/// drained them — so they have to be pushed back before the caller
/// splices our returned variable into its own expression, or that
/// variable is never bound.
///
/// This arm used to be a catch-all that allocated a fresh variable and
/// dropped `expr` on the floor. Every non-atomic subexpression went
/// through it — a nested application, a compound `if` condition, a
/// `let`-in, a `match` scrutinee — so the emitted Rust referenced
/// `_xN` identifiers that nothing declared and failed to compile with
/// `E0425: cannot find value`. Nothing caught it because no test in
/// either repository compiled the transpiler's output.
///
/// `Case` / `TailCall` / `Unreachable` are genuinely different: they
/// are control flow, not a value, and binding one to a variable needs a
/// join point the LCNF here has no representation for. They keep the
/// old fresh-variable behaviour, but now say so explicitly rather than
/// hiding inside a catch-all — see the `debug_assert` below.
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
        // Walk the `Let` spine, restoring each binding to the pending
        // stack in source order, then bind whatever the spine ends in.
        LcnfExpr::Let {
            id,
            name,
            ty,
            value,
            body,
        } => {
            state.pending_lets.push_back((id, name, ty, value));
            bind_expr_to_var(*body, state, hint)
        }
        other => {
            debug_assert!(
                matches!(
                    other,
                    LcnfExpr::Case { .. } | LcnfExpr::TailCall(..) | LcnfExpr::Unreachable
                ),
                "bind_expr_to_var: unhandled LcnfExpr shape reached the \
                 control-flow arm; a value-shaped expression must be \
                 flattened into pending_lets instead of dropped"
            );
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

    // Lowering the value already produced a binding for `var_id`. This
    // used to emit a *second* `Let` with the same id and
    // `LcnfLetValue::FVar(var_id)` as its value — i.e. `let _x2 = _x2;`,
    // which the Rust backend emitted verbatim and which does not
    // compile. The binding the value produced is the `let`; there is no
    // second one to make.
    //
    // What the declared type is still good for: the value's binding may
    // have been typed `Object` because nothing about the value said
    // otherwise (`let m : UInt8 := 1 + 1` — integer literals carry no
    // type evidence). The annotation is evidence, so hand it over.
    if is_informative_type(&lcnf_ty) {
        state.retype_pending(var_id, &lcnf_ty);
    }

    state.push_bvar(var_id, &name_str);
    let body_lcnf = convert_expr(body, state, true)?;
    state.pop_bvar();
    Ok(state.wrap_pending_lets(body_lcnf))
}
/// Does this converted type actually say something?
///
/// `Object` is LCNF's "unknown". `Var("fv_<N>")` is what
/// [`convert_type`] makes of an `FVar` the elaborator left in a type
/// position — an unresolved placeholder, not a name any backend can
/// emit; `oxilean-elab` puts those at `FVarId(1_000_000+)`, well clear
/// of the binder ids.
pub(super) fn is_informative_type(ty: &LcnfType) -> bool {
    match ty {
        LcnfType::Object => false,
        LcnfType::Var(name) => !name.starts_with("fv_"),
        _ => true,
    }
}
/// Convert a literal expression.
pub(super) fn convert_lit(
    lit: &Literal,
    state: &mut ToLcnfState,
) -> Result<LcnfExpr, ConversionError> {
    let lcnf_lit = match lit {
        Literal::Nat(n) => LcnfLit::Nat(n.to_u64().unwrap_or(u64::MAX)),
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
    // OX7 (1b-β, 2026-05-27): `oxilean-elab` lowers
    // namespace lookups like `UInt64.add` as a
    // `Proj("add", _, Const("UInt64"))` rather than a
    // direct `Const("UInt64.add")`. When the projection
    // base is a `Const`, treat the projection as a
    // composite kernel-name reference (the dot-joined
    // form) so the Rust backend can emit
    // `UInt64_add(_x0, _x1)` instead of stranding the
    // head in a fresh `_xN` placeholder.
    //
    // Heuristic limitation: a *real* field projection
    // whose base happens to be a 0-ary `Const` (rare —
    // would require the type itself to inhabit a
    // structure, not just be a structure type) would
    // also take this branch. For the rust-transpile
    // path's actual fixtures (primitive method calls,
    // user-namespaced fns) the heuristic is correct.
    if let Expr::Const(base_name, _) = base {
        let composite = base_name.clone().append_str(name_to_string(name));
        let _ = idx;
        return convert_const(&composite, &[], state);
    }
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
    let (decl, _state) = decl_to_lcnf_inner(
        name,
        params,
        body,
        config,
        &std::collections::HashMap::new(),
    )?;
    Ok(decl)
}

/// OX7 (1a, 2026-05-26) — same as [`decl_to_lcnf`] but
/// also returns the `LcnfVarId → kernel-name` mapping
/// built during conversion. Target backends that want
/// to emit `Nat.add` instead of `_x2` for `Const`
/// references consume this entry point and feed the
/// map to the backend (e.g.
/// `RustTargetBackend::set_const_names`).
///
/// The returned map excludes:
/// - Parameter IDs — their names are already on
///   `LcnfParam.name`; emitting them as their source
///   names would replace `_x0(_x1)` with `a(b)` for
///   variable references, which is wrong.
/// - Synthetic `fv_<N>` keys produced by
///   [`convert_fvar`] — those aren't real `Const`
///   references.
///
/// # Errors
/// Same as [`decl_to_lcnf`].
pub fn decl_to_lcnf_with_const_names(
    name: &Name,
    params: &[(Name, Expr)],
    body: &Expr,
    config: &ToLcnfConfig,
) -> Result<(LcnfFunDecl, std::collections::HashMap<LcnfVarId, String>), ConversionError> {
    decl_to_lcnf_full(name, params, None, body, config)
}

/// OX7 (#1+#2, 2026-05-26) — most flexible entry:
/// caller may supply the declared return-type
/// `Expr` (typically the rightmost codomain of the
/// declaration's `Pi`-typed signature). When `Some`,
/// `LcnfFunDecl::ret_type` reflects the declared type
/// directly instead of being heuristically inferred
/// from the body (which falls back to
/// `LcnfType::Object` for `TailCall` results).
/// Returns the same `LcnfVarId → kernel-name` map as
/// [`decl_to_lcnf_with_const_names`].
///
/// # Errors
/// Same as [`decl_to_lcnf`].
pub fn decl_to_lcnf_full(
    name: &Name,
    params: &[(Name, Expr)],
    ret_type_expr: Option<&Expr>,
    body: &Expr,
    config: &ToLcnfConfig,
) -> Result<(LcnfFunDecl, std::collections::HashMap<LcnfVarId, String>), ConversionError> {
    decl_to_lcnf_full_with_sigs(
        name,
        params,
        ret_type_expr,
        body,
        config,
        &std::collections::HashMap::new(),
    )
}

/// Build the constant-signature map [`decl_to_lcnf_full_with_sigs`]
/// consumes, from an elaboration environment.
///
/// Keys are mangled kernel names — the same spelling `to_lcnf` uses
/// internally — and values are each constant's type run through
/// `convert_type`. A `def double (n : UInt64) : UInt64` contributes
/// `"double" → Fun([Ctor("UInt64")], Ctor("UInt64"))`, which is what
/// lets an application of it be typed `UInt64` instead of `Object`.
///
/// Build this once per module and pass it to each declaration's
/// conversion; it does not depend on which declaration is being
/// converted.
#[must_use]
pub fn env_const_types(
    env: &oxilean_kernel::Environment,
    config: &ToLcnfConfig,
) -> std::collections::HashMap<String, LcnfType> {
    // `convert_type` reads only config-independent parts of the state
    // (the proof/type name sets and the bvar stack, all empty here),
    // but it takes a `&ToLcnfState`, so build a throwaway one.
    let state = ToLcnfState::new(config);
    env.constant_infos()
        .map(|(name, ci)| (mangle_name(name), convert_type(ci.ty(), &state)))
        .collect()
}

/// As [`decl_to_lcnf_full`], but with the signatures of the constants
/// the body may refer to.
///
/// Without them every application result is typed `LcnfType::Object`,
/// which the Rust backend renders `Box<dyn std::any::Any>` — so
/// `def quadruple (n : UInt64) : UInt64 := double (double n)` emitted
/// `let _x2: Box<dyn std::any::Any> = double(_x0); double(_x2)` and
/// did not compile. Build the map with [`env_const_types`].
///
/// # Errors
/// Same as [`decl_to_lcnf`].
pub fn decl_to_lcnf_full_with_sigs(
    name: &Name,
    params: &[(Name, Expr)],
    ret_type_expr: Option<&Expr>,
    body: &Expr,
    config: &ToLcnfConfig,
    const_types: &std::collections::HashMap<String, LcnfType>,
) -> Result<(LcnfFunDecl, std::collections::HashMap<LcnfVarId, String>), ConversionError> {
    let (mut decl, state) = decl_to_lcnf_inner(name, params, body, config, const_types)?;
    if let Some(rt_expr) = ret_type_expr {
        decl.ret_type = convert_type(rt_expr, &state);
    }
    let param_ids: std::collections::HashSet<LcnfVarId> =
        decl.params.iter().map(|p| p.id).collect();
    // `fresh_named_var` registers BOTH the `hint` name
    // AND the placeholder `_x<N>` (`name_<N>` in
    // debug_names mode) — so for `Const("Nat.add")` the
    // map contains both `Nat_add → 2` and `_x2 → 2`.
    // Filter out the synthetic placeholders so a
    // reverse map keyed by `LcnfVarId` always carries
    // the kernel-name side.
    let const_names: std::collections::HashMap<LcnfVarId, String> = state
        .name_map
        .iter()
        .filter(|(mangled, id)| {
            if param_ids.contains(id) {
                return false;
            }
            if mangled.starts_with("fv_") {
                return false;
            }
            // `_x<N>` placeholder, possibly with the
            // var_id `id.0` baked in.
            if let Some(rest) = mangled.strip_prefix("_x") {
                if rest.parse::<u64>().is_ok() {
                    return false;
                }
            }
            true
        })
        .map(|(mangled, id)| (*id, mangled.clone()))
        .collect();
    // Second pass: fill in bindings the one-pass conversion had to
    // leave as `Object` because the type they need lives in an
    // enclosing application it had not reached yet.
    propagate_operand_types(&mut decl, &const_names);
    Ok((decl, const_names))
}

/// Internal helper shared by [`decl_to_lcnf`] and
/// [`decl_to_lcnf_with_const_names`]. Returns the
/// converted decl alongside the final `ToLcnfState` so
/// the caller can extract auxiliary maps if needed.
fn decl_to_lcnf_inner(
    name: &Name,
    params: &[(Name, Expr)],
    body: &Expr,
    config: &ToLcnfConfig,
    const_types: &std::collections::HashMap<String, LcnfType>,
) -> Result<(LcnfFunDecl, ToLcnfState), ConversionError> {
    let mut state = ToLcnfState::new(config);
    if !const_types.is_empty() {
        state.type_map = const_types.clone();
    }
    let name_str = mangle_name(name);
    let mut lcnf_params = Vec::new();
    for (pname, pty) in params {
        let pname_str = name_to_string(pname);
        let param_ty = convert_type(pty, &state);
        let param_id = state.fresh_named_var(&pname_str);
        // The parameter's declared type is the root of the whole
        // `var_types` chain: without it, an operand-driven inference
        // like `n + 1` has nothing to start from.
        state.record_var_type(param_id, &param_ty);
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
    Ok((decl, state))
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
        let expr = Expr::Lit(Literal::nat(42));
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
        let arg = Expr::Lit(Literal::nat(0));
        let expr = Expr::App(Node::new(func), Node::new(arg));
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
            Node::new(Expr::Const(Name::str("Nat"), vec![])),
            Node::new(Expr::Lit(Literal::nat(5))),
            Node::new(Expr::BVar(0)),
        );
        let result = expr_to_lcnf(&expr, &config);
        assert!(result.is_ok());
    }
    #[test]
    pub(super) fn test_convert_projection() {
        let config = default_config();
        let base = Expr::Const(Name::str("p"), vec![]);
        let expr = Expr::Proj(Name::str("Prod"), 0, Node::new(base));
        let result = expr_to_lcnf(&expr, &config);
        assert!(result.is_ok());
    }
    #[test]
    pub(super) fn test_convert_pi_erased() {
        let config = default_config();
        let expr = Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Node::new(Expr::Const(Name::str("Nat"), vec![])),
            Node::new(Expr::Const(Name::str("Nat"), vec![])),
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
            Node::new(Expr::Const(Name::str("Nat"), vec![])),
            Node::new(Expr::Const(Name::str("Nat"), vec![])),
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
        let lit = Expr::Lit(Literal::nat(1));
        assert_eq!(estimate_expr_size(&lit), 1);
        let app = Expr::App(
            Node::new(Expr::Const(Name::str("f"), vec![])),
            Node::new(Expr::Lit(Literal::nat(1))),
        );
        assert_eq!(estimate_expr_size(&app), 3);
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Node::new(Expr::Const(Name::str("Nat"), vec![])),
            Node::new(Expr::BVar(0)),
        );
        assert_eq!(estimate_expr_size(&lam), 3);
    }
    #[test]
    pub(super) fn test_flatten_app() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Lit(Literal::nat(1));
        let b = Expr::Lit(Literal::nat(2));
        let app1 = Expr::App(Node::new(f), Node::new(a));
        let app2 = Expr::App(Node::new(app1), Node::new(b));
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
        let decls = vec![(Name::str("const42"), vec![], Expr::Lit(Literal::nat(42)))];
        let result = module_to_lcnf(&decls, &config);
        assert!(result.is_ok());
        let module = result.expect("module should be Some/Ok");
        assert!(!module.fun_decls.is_empty());
        assert_eq!(module.metadata.decl_count, 1);
    }
    /// OX7 spike (2026-05-26) — reproduce the
    /// `_x4(_x5, _x6)` body-corruption symptom and dump
    /// every LcnfVarId allocation along the way. Asserts
    /// the *correct* invariants so once the underlying
    /// bug is fixed this test stays green.
    ///
    /// OX7 typeclass step (2026-05-27) — ensure that a
    /// `Const("HAdd.hAdd")`-headed app lowers to a
    /// native Rust `BinOp { op: "+", ... }` instead of
    /// an opaque `Call(HAdd_hAdd, ...)`. Pairs with
    /// `RustTargetBackend::try_builtin_app` +
    /// `tc_projection_to_rust_binop`.
    #[test]
    pub(super) fn spike_ox7_hadd_lowers_to_native_binop() {
        use crate::rust_target_backend::RustTargetBackend;
        let config = default_config();
        let name = Name::str("add");
        let uint64 = Expr::Const(Name::str("UInt64"), vec![]);
        let params = vec![
            (Name::str("a"), uint64.clone()),
            (Name::str("b"), uint64.clone()),
        ];
        // body = HAdd.hAdd a b
        let body = Expr::App(
            Node::new(Expr::App(
                Node::new(Expr::Const(Name::from_str("HAdd.hAdd"), vec![])),
                Node::new(Expr::BVar(1)),
            )),
            Node::new(Expr::BVar(0)),
        );

        let (decl, const_names) = decl_to_lcnf_full(&name, &params, Some(&uint64), &body, &config)
            .expect("conversion must succeed");

        let mut backend = RustTargetBackend::new();
        backend.set_const_names(const_names);
        let rust_fn = backend.compile_decl(&decl).expect("compile must succeed");
        let emitted = rust_fn.emit();
        eprintln!("OX7 (HAdd → native BinOp) emitted:\n{}", emitted);
        // The head `HAdd_hAdd` should NOT appear — it's
        // replaced by a native `+` BinOp on the args.
        assert!(
            !emitted.contains("HAdd_hAdd"),
            "head must be folded away into native BinOp: {}",
            emitted
        );
        assert!(
            emitted.contains("_x0 + _x1"),
            "body must contain native `_x0 + _x1`: {}",
            emitted
        );
    }

    /// OX7 ite step (2026-05-25) — ensure that a
    /// `Const("ite")`-headed 5-arg app lowers to a
    /// native Rust `if … { … } else { … }` expression
    /// instead of an opaque `Call(ite, [α, c, inst, t, e])`.
    /// Pairs with `RustTargetBackend::try_builtin_app` +
    /// the `mangled == "ite"` arm.
    ///
    /// Fixture mirrors what `oxilean_elab::elaborate::
    /// elaborate_if` produces for
    /// `def chooseU64 (b : Bool) (a c : UInt64) : UInt64
    ///    := if b then a else c`:
    ///   `App(App(App(App(App(Const("ite"), α), b), inst), a), c)`.
    /// The α / inst slots are erased FVars; using
    /// `Expr::FVar` placeholders keeps the test
    /// realistic without needing a full Decidable
    /// instance.
    #[test]
    pub(super) fn spike_ox7_ite_lowers_to_native_if() {
        use crate::rust_target_backend::RustTargetBackend;
        use oxilean_kernel::FVarId;
        let config = default_config();
        let name = Name::str("chooseU64");
        let bool_ty = Expr::Const(Name::str("Bool"), vec![]);
        let uint64 = Expr::Const(Name::str("UInt64"), vec![]);
        let params = vec![
            (Name::str("b"), bool_ty.clone()),
            (Name::str("a"), uint64.clone()),
            (Name::str("c"), uint64.clone()),
        ];
        // body = ite α b inst a c
        // — α and inst arrive as fresh FVar placeholders
        //   from elab's metavar resolution.
        let alpha = Expr::FVar(FVarId(9_000_001));
        let inst = Expr::FVar(FVarId(9_000_002));
        let body = Expr::App(
            Node::new(Expr::App(
                Node::new(Expr::App(
                    Node::new(Expr::App(
                        Node::new(Expr::App(
                            Node::new(Expr::Const(Name::str("ite"), vec![])),
                            Node::new(alpha),
                        )),
                        Node::new(Expr::BVar(2)),
                    )),
                    Node::new(inst),
                )),
                Node::new(Expr::BVar(1)),
            )),
            Node::new(Expr::BVar(0)),
        );

        let (decl, const_names) = decl_to_lcnf_full(&name, &params, Some(&uint64), &body, &config)
            .expect("conversion must succeed");

        let mut backend = RustTargetBackend::new();
        backend.set_const_names(const_names);
        let rust_fn = backend.compile_decl(&decl).expect("compile must succeed");
        let emitted = rust_fn.emit();
        eprintln!("OX7 (ite → native if) emitted:\n{}", emitted);
        // The opaque `ite(...)` head must NOT remain —
        // it's replaced by a native Rust `if`-expression
        // on the cond / then / else slots.
        assert!(
            !emitted.contains("ite("),
            "head must be folded away into native If expr: {}",
            emitted
        );
        assert!(
            emitted.contains("if _x0"),
            "body must start the if-expr on cond `_x0` (b): {}",
            emitted
        );
        // Then/else branches reference the BVar(1)/BVar(0)
        // parameter slots — `_x1` / `_x2`.
        assert!(
            emitted.contains("_x1"),
            "then-branch must reference `_x1` (a): {}",
            emitted
        );
        assert!(
            emitted.contains("_x2"),
            "else-branch must reference `_x2` (c): {}",
            emitted
        );
        assert!(
            emitted.contains("} else {"),
            "must have a Rust-native else-block: {}",
            emitted
        );
    }

    /// OX7 Bool literal fold (2026-05-25) — ensure that
    /// Lean's `Bool.true` / `Bool.false` const
    /// references emit as native Rust `true` / `false`
    /// literals rather than bare identifiers (which
    /// have no Rust definition and break linking).
    ///
    /// The fixture is the original problem fixture:
    /// `def constU64 : UInt64 := if true then 1 else 0`.
    /// After elab + OX7 ite fold the kernel body is
    /// `ite α Bool.true inst 1 0`. The Bool literal
    /// fold must transform the slot-1 (cond) reference
    /// to `Bool.true` from an opaque
    /// `RustExpr::Var("Bool_true")` (or `"true_"`, if
    /// the namespace were stripped) into
    /// `RustExpr::Lit(RustLit::Bool(true))`, so the
    /// emitted Rust is `if true { 1 } else { 0 }`.
    ///
    /// The Bool argument has no FFI-relevant type info
    /// — Lean infers `α = UInt64` from the branches,
    /// and the `Decidable Bool.true` instance is
    /// `Bool.decEq` (an axiomised `instImplicit`
    /// metavar). We model both as fresh `FVar`
    /// placeholders (same trick as
    /// `spike_ox7_ite_lowers_to_native_if`).
    #[test]
    pub(super) fn spike_ox7_bool_lit_folds_to_native() {
        use crate::rust_target_backend::RustTargetBackend;
        use oxilean_kernel::FVarId;
        let config = default_config();
        let name = Name::str("constU64");
        let uint64 = Expr::Const(Name::str("UInt64"), vec![]);
        let params: Vec<(Name, Expr)> = vec![];
        // body = ite α Bool.true inst 1 0
        let alpha = Expr::FVar(FVarId(9_100_001));
        let inst = Expr::FVar(FVarId(9_100_002));
        let bool_true = Expr::Const(Name::str("Bool.true"), vec![]);
        let body = Expr::App(
            Node::new(Expr::App(
                Node::new(Expr::App(
                    Node::new(Expr::App(
                        Node::new(Expr::App(
                            Node::new(Expr::Const(Name::str("ite"), vec![])),
                            Node::new(alpha),
                        )),
                        Node::new(bool_true),
                    )),
                    Node::new(inst),
                )),
                Node::new(Expr::Lit(oxilean_kernel::Literal::nat(1))),
            )),
            Node::new(Expr::Lit(oxilean_kernel::Literal::nat(0))),
        );

        let (decl, const_names) = decl_to_lcnf_full(&name, &params, Some(&uint64), &body, &config)
            .expect("conversion must succeed");

        eprintln!("OX7 (bool lit fold) const_names = {:?}", const_names);
        // const_names MUST register `Bool_true` — that's
        // what the fold matches on.
        assert!(
            const_names.values().any(|n| n == "Bool_true"),
            "const_names must contain `Bool_true`, got: {:?}",
            const_names
        );

        let mut backend = RustTargetBackend::new();
        backend.set_const_names(const_names);
        let rust_fn = backend.compile_decl(&decl).expect("compile must succeed");
        let emitted = rust_fn.emit();
        eprintln!("OX7 (bool lit fold) emitted:\n{}", emitted);

        // After the fold, the emitted body must be a
        // native `if true { … } else { … }` — neither
        // `Bool_true` nor the keyword-escaped `true_`
        // may appear as identifiers in the output.
        assert!(
            emitted.contains("if true"),
            "cond slot must emit as native `true` literal: {}",
            emitted
        );
        assert!(
            !emitted.contains("Bool_true"),
            "no residual `Bool_true` identifier may remain: {}",
            emitted
        );
        assert!(
            !emitted.contains("true_"),
            "no residual `true_` identifier (keyword-escaped form) \
             may remain: {}",
            emitted
        );
        assert!(
            !emitted.contains("ite("),
            "ite head must be folded away by the ite step: {}",
            emitted
        );
    }

    /// OX7 HPow method-call step (2026-05-25) — ensure
    /// that a `Const("HPow.hPow")`-headed 2-arg app
    /// lowers to a native Rust
    /// `RustExpr::MethodCall { method: "pow", … }`
    /// (emits as `lhs.pow(rhs)`) instead of an opaque
    /// `Call(HPow_hPow, …)`. Pairs with
    /// `RustTargetBackend::try_builtin_app`'s
    /// `mangled == "HPow_hPow"` arm.
    ///
    /// Fixture mirrors what oxilean-elab produces for
    /// `def pow8 (n : UInt64) : UInt64 := n ^ 8`:
    /// the `^` operator desugars (through the `HPow`
    /// typeclass) to `App(App(Const("HPow.hPow"), n), 8)`.
    /// We construct the App tree directly with
    /// `Expr::BVar(0)` for `n` and a `Nat`-typed literal
    /// `8` (modelled as `Expr::Lit(Literal::nat(8))`).
    #[test]
    pub(super) fn spike_ox7_hpow_lowers_to_method_call() {
        use crate::rust_target_backend::RustTargetBackend;
        let config = default_config();
        let name = Name::str("pow8");
        let uint64 = Expr::Const(Name::str("UInt64"), vec![]);
        let params = vec![(Name::str("n"), uint64.clone())];
        // body = HPow.hPow n 8
        let body = Expr::App(
            Node::new(Expr::App(
                Node::new(Expr::Const(Name::from_str("HPow.hPow"), vec![])),
                Node::new(Expr::BVar(0)),
            )),
            Node::new(Expr::Lit(Literal::nat(8))),
        );

        let (decl, const_names) = decl_to_lcnf_full(&name, &params, Some(&uint64), &body, &config)
            .expect("conversion must succeed");

        let mut backend = RustTargetBackend::new();
        backend.set_const_names(const_names);
        let rust_fn = backend.compile_decl(&decl).expect("compile must succeed");
        let emitted = rust_fn.emit();
        eprintln!("OX7 (HPow → .pow method call) emitted:\n{}", emitted);
        // The head `HPow_hPow` should NOT appear — it's
        // replaced by a native Rust `.pow(…)` method
        // call on the lhs.
        assert!(
            !emitted.contains("HPow_hPow"),
            "head must be folded away into native .pow method call: {}",
            emitted
        );
        // The emitted body should call `.pow(...)` on the
        // lhs (`_x0`, the BVar(0) param slot for `n`).
        assert!(
            emitted.contains("_x0.pow("),
            "body must contain native `_x0.pow(…)` method call: {}",
            emitted
        );
        // And the rhs `8` should appear inside the call.
        assert!(
            emitted.contains(".pow(8)"),
            "method call must pass the rhs `8` as the exponent: {}",
            emitted
        );
    }

    /// OX7 spike (1b-β, 2026-05-27) — ensure that a
    /// `Proj("add", _, Const("UInt64"))` head (the way
    /// oxilean-elab lowers `UInt64.add`) emits as the
    /// composite kernel name `UInt64_add` rather than
    /// the bound-let placeholder `_xN`. Pairs with
    /// `convert_to_atomic`'s fast-path branch.
    #[test]
    pub(super) fn spike_ox7_proj_const_base_emits_composite_name() {
        use crate::rust_target_backend::RustTargetBackend;
        let config = default_config();
        let name = Name::str("add");
        let uint64 = Expr::Const(Name::str("UInt64"), vec![]);
        let params = vec![
            (Name::str("a"), uint64.clone()),
            (Name::str("b"), uint64.clone()),
        ];
        // body = Proj("add", 0, Const("UInt64")) a b
        // — the shape oxilean-elab produces for
        // `UInt64.add a b`.
        let proj = Expr::Proj(
            Name::str("add"),
            0,
            Node::new(Expr::Const(Name::str("UInt64"), vec![])),
        );
        let body = Expr::App(
            Node::new(Expr::App(Node::new(proj), Node::new(Expr::BVar(1)))),
            Node::new(Expr::BVar(0)),
        );

        let (decl, const_names) = decl_to_lcnf_full(&name, &params, Some(&uint64), &body, &config)
            .expect("conversion must succeed");

        eprintln!("OX7 (1b-β) const_names = {:?}", const_names);
        assert!(
            const_names.values().any(|n| n == "UInt64_add"),
            "const_names must contain `UInt64_add`, got: {:?}",
            const_names
        );

        let mut backend = RustTargetBackend::new();
        backend.set_const_names(const_names);
        let rust_fn = backend.compile_decl(&decl).expect("compile must succeed");
        let emitted = rust_fn.emit();
        eprintln!("OX7 (1b-β) emitted Rust:\n{}", emitted);
        assert!(
            emitted.contains("UInt64_add(_x0, _x1)"),
            "body must call `UInt64_add(_x0, _x1)`: {}",
            emitted
        );
    }

    /// OX7 spike (#1 + #2, 2026-05-26) — end-to-end
    /// check that `decl_to_lcnf_full` produces emitted
    /// Rust where (a) the declared return type is
    /// honoured (no `Box<dyn Any>` fallback) and (b)
    /// sized integer Lean primitives map to native
    /// Rust scalars. Fixture:
    /// `def add (a b : UInt64) : UInt64 := Nat.add a b`.
    #[test]
    pub(super) fn spike_ox7_uint64_native_mapping_and_ret_type() {
        use crate::rust_target_backend::RustTargetBackend;
        let config = default_config();
        let name = Name::str("add");
        let uint64 = Expr::Const(Name::str("UInt64"), vec![]);
        let params = vec![
            (Name::str("a"), uint64.clone()),
            (Name::str("b"), uint64.clone()),
        ];
        let body = Expr::App(
            Node::new(Expr::App(
                Node::new(Expr::Const(Name::str("Nat.add"), vec![])),
                Node::new(Expr::BVar(1)),
            )),
            Node::new(Expr::BVar(0)),
        );

        let (decl, const_names) = decl_to_lcnf_full(&name, &params, Some(&uint64), &body, &config)
            .expect("conversion must succeed");

        eprintln!("OX7 (#1+#2) ret_type = {:?}", decl.ret_type);
        // #1: ret_type is the declared `UInt64`, not the
        // body-inferred fallback `LcnfType::Object`.
        match &decl.ret_type {
            LcnfType::Ctor(n, args) if n == "UInt64" && args.is_empty() => {}
            other => panic!("expected Ctor(UInt64, []), got: {:?}", other),
        }

        let mut backend = RustTargetBackend::new();
        backend.set_const_names(const_names);
        let rust_fn = backend.compile_decl(&decl).expect("compile must succeed");
        let emitted = rust_fn.emit();
        eprintln!("OX7 (#1+#2) emitted Rust:\n{}", emitted);
        // #2: UInt64 maps to native `u64`.
        assert!(
            emitted.contains("_x0: u64"),
            "param `a` must be `u64`: {}",
            emitted
        );
        assert!(
            emitted.contains("_x1: u64"),
            "param `b` must be `u64`: {}",
            emitted
        );
        // #1: return type is now `u64`, not the
        // `Box<dyn std::any::Any>` fallback.
        assert!(
            emitted.contains("-> u64"),
            "return type must be `u64`: {}",
            emitted
        );
        assert!(
            !emitted.contains("Box<dyn std::any::Any>"),
            "no `Box<dyn Any>` fallback should leak: {}",
            emitted
        );
        // (1a) sanity — head still emits as `Nat_add`.
        assert!(
            emitted.contains("Nat_add"),
            "head must still be `Nat_add`: {}",
            emitted
        );
    }

    /// OX7 spike (1a, 2026-05-26) — end-to-end check
    /// of the const-name preservation path. Same fixture
    /// as `spike_ox7_nat_add_var_id_tracking` but goes
    /// all the way through `decl_to_lcnf_with_const_names`
    /// + `RustTargetBackend::compile_decl` and asserts
    /// the body emits `Nat_add(_x0, _x1)` instead of
    /// `_x2(_x0, _x1)`.
    #[test]
    pub(super) fn spike_ox7_nat_add_emit_uses_kernel_name() {
        use crate::rust_target_backend::RustTargetBackend;
        let config = default_config();
        let name = Name::str("add");
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let params = vec![(Name::str("a"), nat.clone()), (Name::str("b"), nat.clone())];
        let body = Expr::App(
            Node::new(Expr::App(
                Node::new(Expr::Const(Name::str("Nat.add"), vec![])),
                Node::new(Expr::BVar(1)),
            )),
            Node::new(Expr::BVar(0)),
        );

        let (decl, const_names) = decl_to_lcnf_with_const_names(&name, &params, &body, &config)
            .expect("conversion must succeed");

        // The Const reference `Nat.add` must be in the
        // map keyed by its var_id; `a`/`b` (params) must
        // NOT be present. Names in the map are already
        // run through `mangle_name` by `convert_const`
        // (kernel `.`/`:`/`'` get replaced with `_`), so
        // we check for `Nat_add` rather than `Nat.add`.
        eprintln!("OX7 emit spike: const_names = {:?}", const_names);
        eprintln!("OX7 emit spike: body = {:?}", decl.body);
        assert!(
            const_names.values().any(|n| n == "Nat_add"),
            "const_names should contain `Nat_add` (mangled), got: {:?}",
            const_names
        );
        for p in &decl.params {
            assert!(
                !const_names.contains_key(&p.id),
                "param id {:?} ({}) must NOT be in const_names",
                p.id,
                p.name
            );
        }

        // Emit and verify the body uses `Nat_add` rather
        // than `_x2` (or whichever fresh id the head got).
        let mut backend = RustTargetBackend::new();
        backend.set_const_names(const_names);
        let rust_fn = backend.compile_decl(&decl).expect("compile must succeed");
        let emitted = rust_fn.emit();
        eprintln!("OX7 emit spike: emitted Rust:\n{}", emitted);
        assert!(
            emitted.contains("Nat_add"),
            "emitted Rust must reference `Nat_add`, got:\n{}",
            emitted
        );
        assert!(
            !emitted.contains("_x2("),
            "emitted Rust must NOT call `_x2(`, got:\n{}",
            emitted
        );
    }

    /// Fixture: `def add (a b : Nat) : Nat := Nat.add a b`
    /// in raw kernel form (no elab).
    #[test]
    pub(super) fn spike_ox7_nat_add_var_id_tracking() {
        let config = default_config();
        let name = Name::str("add");
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let params = vec![(Name::str("a"), nat.clone()), (Name::str("b"), nat.clone())];
        // body = Nat.add a b
        // = App(App(Const("Nat.add"), BVar(1)), BVar(0))
        let nat_add = Expr::Const(Name::str("Nat.add"), vec![]);
        let body = Expr::App(
            Node::new(Expr::App(Node::new(nat_add), Node::new(Expr::BVar(1)))),
            Node::new(Expr::BVar(0)),
        );

        let decl = decl_to_lcnf(&name, &params, &body, &config).expect("decl_to_lcnf must succeed");

        // ── Invariant 1: params get var_ids 0 and 1.
        assert_eq!(decl.params.len(), 2, "expect 2 params");
        assert_eq!(decl.params[0].id.0, 0, "first param `a` must be _x0");
        assert_eq!(decl.params[1].id.0, 1, "second param `b` must be _x1");

        // ── Invariant 2: body's tail call references the
        // *param* ids, not freshly-allocated ones. Failure
        // mode (pre-fix): `_x5(_x6, _x7)` — head + args
        // all get fresh var_ids unrelated to the params.
        // Post-fix: body args should be Var(LcnfVarId(0))
        // and Var(LcnfVarId(1)) for `a` and `b`.
        match &decl.body {
            LcnfExpr::TailCall(head, args) => {
                eprintln!("OX7 spike: head = {:?}, args = {:?}", head, args);
                assert_eq!(args.len(), 2, "expect 2 args (a, b)");
                // `Nat.add a b` — args[0] is `a` (BVar 1
                // in de Bruijn after the App reversal in
                // flatten_app), args[1] is `b` (BVar 0).
                // Wait — flatten_app un-reverses; check
                // BVar order in body above:
                // App(App(Const, BVar(1)), BVar(0))
                // flatten yields head=Const, args=[BVar(1), BVar(0)].
                // BVar(1) = `a` (outer binder, var_id 0).
                // BVar(0) = `b` (inner binder, var_id 1).
                if let LcnfArg::Var(a_id) = args[0] {
                    assert_eq!(
                        a_id.0, 0,
                        "arg[0] should be param `a`'s var_id 0; got _x{}",
                        a_id.0
                    );
                } else {
                    panic!("arg[0] is not Var: {:?}", args[0]);
                }
                if let LcnfArg::Var(b_id) = args[1] {
                    assert_eq!(
                        b_id.0, 1,
                        "arg[1] should be param `b`'s var_id 1; got _x{}",
                        b_id.0
                    );
                } else {
                    panic!("arg[1] is not Var: {:?}", args[1]);
                }
            }
            other => panic!("expected TailCall body, got: {:?}", other),
        }
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
            Node::new(Expr::BVar(0)),
            Node::new(Expr::Lit(Literal::nat(1))),
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
            Node::new(Expr::Const(Name::str("Nat"), vec![])),
            Node::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("y"),
                Node::new(Expr::Const(Name::str("Nat"), vec![])),
                Node::new(Expr::Const(Name::str("Nat"), vec![])),
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
            Node::new(Expr::Const(Name::str("Nat"), vec![])),
            Node::new(Expr::BVar(0)),
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
            (Name::str("g"), vec![], Expr::Lit(Literal::nat(0))),
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
            Node::new(Expr::App(
                Node::new(Expr::Const(Name::str("Nat.add"), vec![])),
                Node::new(Expr::Lit(Literal::nat(1))),
            )),
            Node::new(Expr::Lit(Literal::nat(2))),
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

    // ── Application result typing (cause-2 fix, 2026-08-02) ──────────
    //
    // Before these, every `LcnfLetValue::App` binding was typed
    // `LcnfType::Object`, which the Rust backend renders
    // `Box<dyn std::any::Any>`. `def quadruple (n : UInt64) : UInt64
    // := double (double n)` therefore emitted
    // `let _x2: Box<dyn std::any::Any> = double(_x0); double(_x2)` —
    // syntactically plausible Rust that does not compile.

    /// Collect the `(name, ty)` of every `Let` in a converted body.
    fn let_types(expr: &LcnfExpr) -> Vec<(String, LcnfType)> {
        let mut out = Vec::new();
        let mut cur = expr;
        while let LcnfExpr::Let { name, ty, body, .. } = cur {
            out.push((name.clone(), ty.clone()));
            cur = body;
        }
        out
    }

    /// `UInt64 -> UInt64`, the shape `convert_type` produces for a
    /// non-dependent one-parameter signature.
    fn uint64_to_uint64() -> LcnfType {
        LcnfType::Fun(
            vec![LcnfType::Ctor("UInt64".to_string(), Vec::new())],
            Box::new(LcnfType::Ctor("UInt64".to_string(), Vec::new())),
        )
    }

    #[test]
    pub(super) fn test_app_result_type_from_declared_signature() {
        let config = default_config();
        let mut sigs = HashMap::new();
        sigs.insert("double".to_string(), uint64_to_uint64());

        // def quadruple (n : UInt64) : UInt64 := double (double n)
        let uint64 = Expr::Const(Name::str("UInt64"), vec![]);
        let dbl = Expr::Const(Name::str("double"), vec![]);
        let body = Expr::App(
            Node::new(dbl.clone()),
            Node::new(Expr::App(Node::new(dbl), Node::new(Expr::BVar(0)))),
        );
        let (decl, _) = decl_to_lcnf_full_with_sigs(
            &Name::str("quadruple"),
            &[(Name::str("n"), uint64.clone())],
            Some(&uint64),
            &body,
            &config,
            &sigs,
        )
        .expect("conversion should succeed");

        let lets = let_types(&decl.body);
        assert_eq!(lets.len(), 1, "expected one intermediate binding: {lets:?}");
        assert_eq!(
            lets[0].1,
            LcnfType::Ctor("UInt64".to_string(), Vec::new()),
            "inner `double n` must take the callee's declared return \
             type, not Object"
        );
    }

    #[test]
    pub(super) fn test_app_result_type_without_signature_stays_object() {
        // Same body, no signatures supplied: the pre-fix behaviour is
        // the documented fallback, not an accident.
        let config = default_config();
        let uint64 = Expr::Const(Name::str("UInt64"), vec![]);
        let dbl = Expr::Const(Name::str("double"), vec![]);
        let body = Expr::App(
            Node::new(dbl.clone()),
            Node::new(Expr::App(Node::new(dbl), Node::new(Expr::BVar(0)))),
        );
        let (decl, _) = decl_to_lcnf_full(
            &Name::str("quadruple"),
            &[(Name::str("n"), uint64.clone())],
            Some(&uint64),
            &body,
            &config,
        )
        .expect("conversion should succeed");

        let lets = let_types(&decl.body);
        assert_eq!(lets.len(), 1);
        assert_eq!(lets[0].1, LcnfType::Object);
    }

    #[test]
    pub(super) fn test_app_result_type_from_operands_for_tc_projection() {
        // def isBig (n : UInt64) : Bool := (n + 1) > 10
        //
        // `HAdd.hAdd` / `LT.lt` reach `to_lcnf` as bare axioms — Lean's
        // instance for them is never resolved on this path — so the
        // arrows that would carry `α → α → α` are not there to peel and
        // the result type has to come from the operands.
        let config = default_config();
        let uint64 = Expr::Const(Name::str("UInt64"), vec![]);
        let bool_ty = Expr::Const(Name::str("Bool"), vec![]);
        let hadd = Expr::Const(Name::from_str("HAdd.hAdd"), vec![]);
        let lt = Expr::Const(Name::from_str("LT.lt"), vec![]);

        // LT.lt 10 (HAdd.hAdd n 1) — `n > 10` unfolds to `10 < n`.
        let sum = Expr::App(
            Node::new(Expr::App(Node::new(hadd), Node::new(Expr::BVar(0)))),
            Node::new(Expr::Lit(Literal::nat(1))),
        );
        let cmp = Expr::App(
            Node::new(Expr::App(
                Node::new(lt),
                Node::new(Expr::Lit(Literal::nat(10))),
            )),
            Node::new(sum),
        );
        // Wrap in `ite` so the comparison is a scrutinee rather than
        // the tail call — a tail call takes the declaration's declared
        // return type and would not exercise the inference at all.
        let body = Expr::App(
            Node::new(Expr::App(
                Node::new(Expr::Const(Name::str("ite"), vec![])),
                Node::new(cmp),
            )),
            Node::new(Expr::Const(Name::from_str("Bool.true"), vec![])),
        );
        let (decl, _) = decl_to_lcnf_full(
            &Name::str("isBig"),
            &[(Name::str("n"), uint64)],
            Some(&bool_ty),
            &body,
            &config,
        )
        .expect("conversion should succeed");

        let tys: Vec<LcnfType> = let_types(&decl.body).into_iter().map(|(_, t)| t).collect();
        assert!(
            tys.contains(&LcnfType::Ctor("UInt64".to_string(), Vec::new())),
            "`n + 1` must take `n`'s type, not the literal's Nat and \
             not Object: {tys:?}"
        );
        assert!(
            tys.contains(&LcnfType::Ctor("Bool".to_string(), Vec::new())),
            "a comparison projection is Bool-valued regardless of \
             operand type: {tys:?}"
        );
    }

    #[test]
    pub(super) fn test_app_result_type_skips_erased_operands() {
        // Lean's fully-elaborated form of `a + b` is
        // `HAdd.hAdd α β γ inst a b` — three type arguments and an
        // instance ahead of the operands. Those reach `to_lcnf` as
        // references to `Sort`-typed constants, i.e. `LcnfType::Erased`.
        // Reading the result type off one of them would emit `()`.
        let config = default_config();
        let mut sigs = HashMap::new();
        sigs.insert("UInt64".to_string(), LcnfType::Erased);
        let uint64 = Expr::Const(Name::str("UInt64"), vec![]);
        let hadd = Expr::Const(Name::from_str("HAdd.hAdd"), vec![]);

        // ite (HAdd.hAdd UInt64 n n) — the `ite` wrapper keeps the sum
        // out of tail position so it gets a `Let` to inspect.
        let sum = Expr::App(
            Node::new(Expr::App(
                Node::new(Expr::App(Node::new(hadd), Node::new(uint64.clone()))),
                Node::new(Expr::BVar(0)),
            )),
            Node::new(Expr::BVar(0)),
        );
        let body = Expr::App(
            Node::new(Expr::Const(Name::str("ite"), vec![])),
            Node::new(sum),
        );
        let (decl, _) = decl_to_lcnf_full_with_sigs(
            &Name::str("f"),
            &[(Name::str("n"), uint64.clone())],
            None,
            &body,
            &config,
            &sigs,
        )
        .expect("conversion should succeed");

        let tys: Vec<LcnfType> = let_types(&decl.body).into_iter().map(|(_, t)| t).collect();
        assert!(
            tys.contains(&LcnfType::Ctor("UInt64".to_string(), Vec::new())),
            "the sum must take the operand's type, not the erased type \
             argument's: {tys:?}"
        );
        assert!(
            !tys.contains(&LcnfType::Erased),
            "no binding may be typed from an erasure marker: {tys:?}"
        );
    }

    #[test]
    pub(super) fn test_app_result_type_literal_only_operands_stay_object() {
        // `1 + 1` inside a `UInt8` function used to be typed from the
        // literal's own `Nat`, i.e. `u64`, which then failed to unify
        // with the surrounding `u8`. A literal carries no type
        // evidence in Lean — it is coerced to whatever the instance
        // demands — so the honest answer is "unknown".
        let config = default_config();
        let hadd = Expr::Const(Name::from_str("HAdd.hAdd"), vec![]);
        let sum = Expr::App(
            Node::new(Expr::App(
                Node::new(hadd),
                Node::new(Expr::Lit(Literal::nat(1))),
            )),
            Node::new(Expr::Lit(Literal::nat(1))),
        );
        // Wrapped so the sum is not the tail call.
        let body = Expr::App(
            Node::new(Expr::Const(Name::str("ite"), vec![])),
            Node::new(sum),
        );
        let (decl, _) = decl_to_lcnf_full(
            &Name::str("two"),
            &[(Name::str("n"), Expr::Const(Name::str("UInt8"), vec![]))],
            None,
            &body,
            &config,
        )
        .expect("conversion should succeed");

        for (name, ty) in let_types(&decl.body) {
            if name.starts_with("app") || name.starts_with("_x") {
                assert_ne!(
                    ty,
                    LcnfType::Nat,
                    "an all-literal sum must not claim `Nat`/`u64`"
                );
            }
        }
    }

    #[test]
    pub(super) fn test_app_result_type_eq_eq_is_boolean() {
        // leo4's translate layer lowers surface `=` to `Eq.eq`. Without
        // an entry it fell through to `Object` -> `Box<dyn Any>`, and
        // `a != b` (`Not.not (Eq.eq a b)`) then put that under Rust's
        // unary `!`.
        let config = default_config();
        let uint64 = Expr::Const(Name::str("UInt64"), vec![]);
        let eq = Expr::Const(Name::from_str("Eq.eq"), vec![]);
        let cmp = Expr::App(
            Node::new(Expr::App(Node::new(eq), Node::new(Expr::BVar(1)))),
            Node::new(Expr::BVar(0)),
        );
        let body = Expr::App(
            Node::new(Expr::Const(Name::str("ite"), vec![])),
            Node::new(cmp),
        );
        let (decl, _) = decl_to_lcnf_full(
            &Name::str("eqp"),
            &[(Name::str("a"), uint64.clone()), (Name::str("b"), uint64)],
            None,
            &body,
            &config,
        )
        .expect("conversion should succeed");

        let tys: Vec<LcnfType> = let_types(&decl.body).into_iter().map(|(_, t)| t).collect();
        assert!(
            tys.contains(&LcnfType::Ctor("Bool".to_string(), Vec::new())),
            "`a = b` must be Bool-valued: {tys:?}"
        );
    }

    #[test]
    pub(super) fn test_app_result_type_partial_application_is_not_the_return_type() {
        // Applying one argument to a two-parameter function yields a
        // function, never the final return type — peeling must not run
        // past the arguments actually supplied.
        let state_sig = LcnfType::Fun(
            vec![LcnfType::Nat],
            Box::new(LcnfType::Fun(
                vec![LcnfType::Nat],
                Box::new(LcnfType::LcnfString),
            )),
        );
        let mut sigs = HashMap::new();
        sigs.insert("f".to_string(), state_sig);
        let config = default_config();

        let nat = Expr::Const(Name::str("Nat"), vec![]);
        // `h (f x)`, not `f x` — the outer application keeps the
        // partial one out of tail position so it gets a `Let` whose
        // type we can inspect.
        let body = Expr::App(
            Node::new(Expr::Const(Name::str("h"), vec![])),
            Node::new(Expr::App(
                Node::new(Expr::Const(Name::str("f"), vec![])),
                Node::new(Expr::BVar(0)),
            )),
        );
        let (decl, _) = decl_to_lcnf_full_with_sigs(
            &Name::str("g"),
            &[(Name::str("x"), nat)],
            None,
            &body,
            &config,
            &sigs,
        )
        .expect("conversion should succeed");

        let lets = let_types(&decl.body);
        assert_eq!(lets.len(), 1, "expected one binding: {lets:?}");
        assert_eq!(
            lets[0].1,
            LcnfType::Fun(vec![LcnfType::Nat], Box::new(LcnfType::LcnfString)),
            "one argument to a two-parameter fn yields a function, not \
             its final return type"
        );
    }

    #[test]
    pub(super) fn test_convert_let_does_not_rebind_the_value() {
        // `let m := n + 1; m + m`.
        //
        // `convert_let` used to emit a second `Let` with the same id as
        // the one `bind_expr_to_var` had already produced, and
        // `LcnfLetValue::FVar(id)` as its value — `let _x2 = _x2;`,
        // which the Rust backend emitted verbatim.
        let config = default_config();
        let uint64 = Expr::Const(Name::str("UInt64"), vec![]);
        let hadd = Expr::Const(Name::from_str("HAdd.hAdd"), vec![]);
        let value = Expr::App(
            Node::new(Expr::App(Node::new(hadd.clone()), Node::new(Expr::BVar(0)))),
            Node::new(Expr::Lit(Literal::nat(1))),
        );
        let body = Expr::App(
            Node::new(Expr::App(Node::new(hadd), Node::new(Expr::BVar(0)))),
            Node::new(Expr::BVar(0)),
        );
        let decl_body = Expr::Let(
            Name::str("m"),
            Node::new(uint64.clone()),
            Node::new(value),
            Node::new(body),
        );
        let (decl, _) = decl_to_lcnf_full(
            &Name::str("viaLet"),
            &[(Name::str("n"), uint64.clone())],
            Some(&uint64),
            &decl_body,
            &config,
        )
        .expect("conversion should succeed");

        let mut seen: Vec<LcnfVarId> = Vec::new();
        let mut cur = &decl.body;
        while let LcnfExpr::Let {
            id, value, body, ..
        } = cur
        {
            assert!(
                !matches!(value, LcnfLetValue::FVar(v) if v == id),
                "a `let` must not bind a variable to itself: {id:?}"
            );
            assert!(!seen.contains(id), "`{id:?}` bound twice");
            seen.push(*id);
            cur = body;
        }
    }

    #[test]
    pub(super) fn test_convert_let_takes_the_declared_binder_type() {
        // `let m : UInt8 := 1 + 1` — the value is all literals and so
        // cannot type itself, but the annotation can.
        let config = default_config();
        let uint8 = Expr::Const(Name::str("UInt8"), vec![]);
        let hadd = Expr::Const(Name::from_str("HAdd.hAdd"), vec![]);
        let value = Expr::App(
            Node::new(Expr::App(
                Node::new(hadd.clone()),
                Node::new(Expr::Lit(Literal::nat(1))),
            )),
            Node::new(Expr::Lit(Literal::nat(1))),
        );
        let body = Expr::App(
            Node::new(Expr::App(Node::new(hadd), Node::new(Expr::BVar(0)))),
            Node::new(Expr::BVar(0)),
        );
        let decl_body = Expr::Let(
            Name::str("m"),
            Node::new(uint8.clone()),
            Node::new(value),
            Node::new(body),
        );
        let (decl, _) = decl_to_lcnf_full(&Name::str("h"), &[], Some(&uint8), &decl_body, &config)
            .expect("conversion should succeed");

        let tys: Vec<LcnfType> = let_types(&decl.body).into_iter().map(|(_, t)| t).collect();
        assert!(
            tys.contains(&LcnfType::Ctor("UInt8".to_string(), Vec::new())),
            "the annotation must reach the value's binding: {tys:?}"
        );
    }

    #[test]
    pub(super) fn test_is_informative_type_rejects_elab_placeholders() {
        // `convert_type` turns an `FVar` left in a type position into
        // `Var("fv_<N>")`. That is not a type any backend can emit, and
        // must not overwrite a real one.
        assert!(!is_informative_type(&LcnfType::Object));
        assert!(!is_informative_type(&LcnfType::Var("fv_1000001".into())));
        assert!(is_informative_type(&LcnfType::Var("Alpha".into())));
        assert!(is_informative_type(&LcnfType::Nat));
    }

    #[test]
    pub(super) fn test_propagate_operand_types_settles_a_literal_only_sum() {
        // `if 1 + 1 < n then … ` with `n : UInt8`.
        //
        // The sum's operands are both literals, so conversion has
        // nothing to type it from and leaves `Object`. The comparison
        // it feeds does know — `n` is `UInt8` — but does not exist yet
        // when the sum's `Let` is emitted. The second pass closes that.
        let config = default_config();
        let uint8 = Expr::Const(Name::str("UInt8"), vec![]);
        let hadd = Expr::Const(Name::from_str("HAdd.hAdd"), vec![]);
        let lt = Expr::Const(Name::from_str("LT.lt"), vec![]);
        let sum = Expr::App(
            Node::new(Expr::App(
                Node::new(hadd),
                Node::new(Expr::Lit(Literal::nat(1))),
            )),
            Node::new(Expr::Lit(Literal::nat(1))),
        );
        let cmp = Expr::App(
            Node::new(Expr::App(Node::new(lt), Node::new(sum))),
            Node::new(Expr::BVar(0)),
        );
        let body = Expr::App(
            Node::new(Expr::Const(Name::str("ite"), vec![])),
            Node::new(cmp),
        );
        let (decl, _) = decl_to_lcnf_full(
            &Name::str("g"),
            &[(Name::str("n"), uint8.clone())],
            None,
            &body,
            &config,
        )
        .expect("conversion should succeed");

        let tys: Vec<LcnfType> = let_types(&decl.body).into_iter().map(|(_, t)| t).collect();
        assert!(
            !tys.contains(&LcnfType::Object),
            "no binding should be left `Object`: {tys:?}"
        );
        assert!(
            tys.iter()
                .filter(|t| **t == LcnfType::Ctor("UInt8".to_string(), Vec::new()))
                .count()
                >= 1,
            "the sum must take `n`'s type through the comparison: {tys:?}"
        );
    }

    #[test]
    pub(super) fn test_operands_agree_excludes_the_heterogeneous_ones() {
        // `HPow` takes its exponent in a different type from its base,
        // and the shifts take a shift amount, so neither may push a
        // type sideways even though both are `Homogeneous` in *result*.
        assert!(operands_agree("HAdd_hAdd"));
        assert!(operands_agree("LT_lt"));
        assert!(!operands_agree("HPow_hPow"));
        assert!(!operands_agree("HShiftLeft_hShiftLeft"));
        assert!(!operands_agree("Neg_neg"));
        assert_eq!(
            tc_projection_result_shape("HPow_hPow"),
            Some(TcResultShape::Homogeneous),
            "still homogeneous in its result — the two notions differ"
        );
    }

    #[test]
    pub(super) fn test_env_const_types_keys_are_mangled() {
        use oxilean_kernel::env::{Declaration, Environment};

        let mut env = Environment::new();
        env.add(Declaration::Axiom {
            name: Name::from_str("Foo.bar"),
            univ_params: Vec::new(),
            ty: Expr::Pi(
                BinderInfo::Default,
                Name::str("_"),
                Node::new(Expr::Const(Name::str("Nat"), vec![])),
                Node::new(Expr::Const(Name::str("Nat"), vec![])),
            ),
        })
        .expect("add");

        let sigs = env_const_types(&env, &default_config());
        // `to_lcnf` looks constants up by their mangled spelling; a map
        // keyed by the dotted form would silently never hit.
        assert!(
            sigs.contains_key("Foo_bar"),
            "keys must be mangled: {:?}",
            sigs.keys().collect::<Vec<_>>()
        );
        assert_eq!(
            sigs["Foo_bar"],
            LcnfType::Fun(vec![LcnfType::Nat], Box::new(LcnfType::Nat))
        );
    }
}
