//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::context::ElabContext;
use oxilean_kernel::{BinderInfo, Expr, Level, Name};
use oxilean_parse::{Lexer, Located, Parser, SortKind, StringPart, SurfaceExpr};

use super::types::ElabError;

/// Elaborate a surface expression into a kernel expression.
///
/// This is the main entry point for elaboration. It dispatches to
/// specialized elaboration functions based on the expression form.
pub fn elaborate_expr(
    ctx: &mut ElabContext,
    expr: &Located<SurfaceExpr>,
) -> Result<Expr, ElabError> {
    match &expr.value {
        SurfaceExpr::Sort(sort) => elaborate_sort(sort),
        SurfaceExpr::Var(name) => elaborate_var(ctx, name),
        SurfaceExpr::App(fun, arg) => elaborate_app(ctx, fun, arg, false),
        SurfaceExpr::Lam(binders, body) => elaborate_lambda(ctx, binders, body, None),
        SurfaceExpr::Pi(binders, body) => elaborate_pi(ctx, binders, body),
        SurfaceExpr::Let(name, ty_opt, val, body) => {
            elaborate_let(ctx, name, ty_opt.as_deref(), val, body, None)
        }
        SurfaceExpr::Lit(lit) => Ok(Expr::Lit(convert_literal(lit.clone()))),
        SurfaceExpr::Ann(inner, ty) => elaborate_annotation(ctx, inner, ty),
        SurfaceExpr::Hole => elaborate_hole(ctx),
        SurfaceExpr::Proj(inner, field) => elaborate_proj(ctx, inner, field),
        SurfaceExpr::If(cond, then_branch, else_branch) => {
            elaborate_if(ctx, cond, then_branch, else_branch)
        }
        SurfaceExpr::Match(scrutinee, arms) => elaborate_match(ctx, scrutinee, arms),
        SurfaceExpr::Do(actions) => elaborate_do(ctx, actions),
        SurfaceExpr::Have(name, ty, proof, body) => elaborate_have(ctx, name, ty, proof, body),
        SurfaceExpr::Suffices(name, ty, body) => elaborate_suffices(ctx, name, ty, body),
        SurfaceExpr::Show(ty, inner) => elaborate_show(ctx, ty, inner),
        SurfaceExpr::NamedArg(fun, arg_name, val) => elaborate_named_arg(ctx, fun, arg_name, val),
        SurfaceExpr::AnonymousCtor(fields) => elaborate_anonymous_ctor(ctx, fields),
        SurfaceExpr::ListLit(elems) => elaborate_list_lit(ctx, elems),
        SurfaceExpr::Tuple(elems) => elaborate_tuple(ctx, elems),
        SurfaceExpr::Return(inner) => elaborate_return(ctx, inner),
        SurfaceExpr::StringInterp(parts) => elaborate_string_interp(ctx, parts),
        SurfaceExpr::Range(lo, hi) => elaborate_range(ctx, lo.as_deref(), hi.as_deref()),
        SurfaceExpr::ByTactic(tactics) => elaborate_by_tactic(ctx, tactics),
        SurfaceExpr::Calc(steps) => elaborate_calc(ctx, steps),
    }
}
/// Elaborate an expression with an expected type.
///
/// When the expected type is known, we can:
/// - Use Pi domain as lambda binder type
/// - Propagate expected types through let bodies
/// - Insert coercions if the types don't match directly
#[allow(dead_code)]
pub fn elaborate_with_expected_type(
    ctx: &mut ElabContext,
    expr: &Located<SurfaceExpr>,
    expected_ty: &Expr,
) -> Result<Expr, ElabError> {
    match &expr.value {
        SurfaceExpr::Lam(binders, body) => elaborate_lambda(ctx, binders, body, Some(expected_ty)),
        SurfaceExpr::Let(name, ty_opt, val, body) => {
            elaborate_let(ctx, name, ty_opt.as_deref(), val, body, Some(expected_ty))
        }
        SurfaceExpr::Hole => {
            let (_id, meta) = ctx.fresh_meta(expected_ty.clone());
            Ok(meta)
        }
        SurfaceExpr::If(cond, then_b, else_b) => {
            elaborate_if_with_expected(ctx, cond, then_b, else_b, expected_ty)
        }
        SurfaceExpr::AnonymousCtor(fields) => {
            elaborate_anonymous_ctor_with_expected(ctx, fields, expected_ty)
        }
        SurfaceExpr::ListLit(elems) => elaborate_list_lit_with_expected(ctx, elems, expected_ty),
        _ => elaborate_expr(ctx, expr),
    }
}
fn elaborate_sort(sort: &SortKind) -> Result<Expr, ElabError> {
    match sort {
        SortKind::Type => Ok(Expr::Sort(Level::succ(Level::zero()))),
        SortKind::Prop => Ok(Expr::Sort(Level::zero())),
        SortKind::TypeU(u) => Ok(Expr::Sort(Level::Param(Name::str(u)))),
        SortKind::SortU(u) => Ok(Expr::Sort(Level::Param(Name::str(u)))),
    }
}
fn elaborate_var(ctx: &mut ElabContext, name: &str) -> Result<Expr, ElabError> {
    if let Some(entry) = ctx.lookup_local(&Name::str(name)) {
        return Ok(Expr::FVar(entry.fvar));
    }
    if let Some(_decl) = ctx.env().get(&Name::str(name)) {
        return Ok(Expr::Const(Name::str(name), vec![]));
    }
    let candidates = find_overloads(ctx, name);
    if candidates.len() == 1 {
        return Ok(Expr::Const(candidates[0].clone(), vec![]));
    }
    if candidates.len() > 1 {
        return Err(ElabError::OverloadAmbiguity(format!(
            "ambiguous name '{}', candidates: {:?}",
            name, candidates
        )));
    }
    Err(ElabError::NameNotFound(name.to_string()))
}
/// Find overloaded constant names matching a short name.
///
/// For example, looking up "add" might find "Nat.add", "Int.add", etc.
fn find_overloads(ctx: &ElabContext, short_name: &str) -> Vec<Name> {
    let mut candidates = Vec::new();
    for full_name in ctx.env().constant_names() {
        let name_str = format!("{}", full_name);
        if name_str.ends_with(short_name)
            && (name_str.len() == short_name.len()
                || name_str.as_bytes()[name_str.len() - short_name.len() - 1] == b'.')
        {
            candidates.push(full_name.clone());
        }
    }
    candidates
}
/// Elaborate a function application.
///
/// This handles implicit argument insertion: if the function type
/// has implicit parameters, fresh metavariables are created for them.
///
/// When `explicit` is true (for `@` prefixed applications), no implicit
/// insertion is performed.
fn elaborate_app(
    ctx: &mut ElabContext,
    fun: &Located<SurfaceExpr>,
    arg: &Located<SurfaceExpr>,
    explicit: bool,
) -> Result<Expr, ElabError> {
    let fun_expr = elaborate_expr(ctx, fun)?;
    let arg_expr = elaborate_expr(ctx, arg)?;
    if explicit {
        return Ok(Expr::App(Box::new(fun_expr), Box::new(arg_expr)));
    }
    let fun_with_implicits = insert_implicit_args(ctx, fun_expr)?;
    Ok(Expr::App(Box::new(fun_with_implicits), Box::new(arg_expr)))
}
/// Insert implicit arguments for a function expression.
///
/// Examines the inferred type of the function and, for each leading
/// implicit/instance-implicit parameter, creates a fresh metavariable.
fn insert_implicit_args(ctx: &mut ElabContext, fun: Expr) -> Result<Expr, ElabError> {
    let fun_ty = try_infer_type(ctx, &fun);
    match fun_ty {
        Some(ty) => insert_implicits_from_type(ctx, fun, &ty),
        None => Ok(fun),
    }
}
/// Given a function and its type, insert metavariables for leading implicit args.
fn insert_implicits_from_type(
    ctx: &mut ElabContext,
    mut fun: Expr,
    fun_ty: &Expr,
) -> Result<Expr, ElabError> {
    let mut ty = fun_ty.clone();
    loop {
        match &ty {
            Expr::Pi(BinderInfo::Implicit, _name, dom, cod)
            | Expr::Pi(BinderInfo::StrictImplicit, _name, dom, cod) => {
                let (_id, meta) = ctx.fresh_meta((**dom).clone());
                fun = Expr::App(Box::new(fun), Box::new(meta.clone()));
                ty = oxilean_kernel::instantiate(cod, &meta);
            }
            Expr::Pi(BinderInfo::InstImplicit, _name, dom, cod) => {
                let inst = try_synthesize_or_meta(ctx, dom);
                fun = Expr::App(Box::new(fun), Box::new(inst.clone()));
                ty = oxilean_kernel::instantiate(cod, &inst);
            }
            _ => break,
        }
    }
    Ok(fun)
}
/// Try to synthesize a type class instance, or create a metavar.
fn try_synthesize_or_meta(ctx: &mut ElabContext, class_ty: &Expr) -> Expr {
    if let Some(class_name) = extract_head_const(class_ty) {
        if let Some(decl) = ctx.env().get(&class_name) {
            return Expr::Const(decl.name().clone(), vec![]);
        }
    }
    let (_id, meta) = ctx.fresh_meta(class_ty.clone());
    meta
}
/// Extract the head constant from an expression (unwrapping applications).
fn extract_head_const(expr: &Expr) -> Option<Name> {
    match expr {
        Expr::Const(name, _) => Some(name.clone()),
        Expr::App(f, _) => extract_head_const(f),
        _ => None,
    }
}
/// Try to infer the type of an expression (simplified).
///
/// This is a best-effort type inference used during implicit insertion.
/// It returns None if the type cannot be easily determined.
fn try_infer_type(ctx: &ElabContext, expr: &Expr) -> Option<Expr> {
    match expr {
        Expr::FVar(fvar) => ctx.lookup_fvar(*fvar).map(|e| e.ty.clone()),
        Expr::Const(name, levels) => ctx.env().instantiate_const_type(name, levels),
        _ => None,
    }
}
/// Elaborate a named argument application: `f (x := val)`.
///
/// Named arguments allow specifying which parameter receives the value,
/// reordering arguments and skipping implicit ones.
///
/// Algorithm:
/// 1. Elaborate `f` to get `fun_expr` and try to infer its Pi-type.
/// 2. Walk the Pi-type's leading binders (inserting fresh metavars for
///    implicit/instance ones) until we find a binder whose name matches
///    `arg_name`.
/// 3. Apply the elaborated value at the matching position.
/// 4. If the name is not found, fall back to positional application.
fn elaborate_named_arg(
    ctx: &mut ElabContext,
    fun: &Located<SurfaceExpr>,
    arg_name: &str,
    val: &Located<SurfaceExpr>,
) -> Result<Expr, ElabError> {
    let fun_expr = elaborate_expr(ctx, fun)?;
    let val_expr = elaborate_expr(ctx, val)?;
    let fun_ty = try_infer_type(ctx, &fun_expr);
    if let Some(ty) = fun_ty {
        let mut cur_fun = fun_expr;
        let mut cur_ty = ty;
        loop {
            match cur_ty {
                Expr::Pi(binder_info, ref param_name, ref dom, ref cod) => {
                    let pname_str = format!("{}", param_name);
                    if pname_str == arg_name {
                        return Ok(Expr::App(Box::new(cur_fun), Box::new(val_expr)));
                    }
                    match binder_info {
                        BinderInfo::Default => {
                            let (_id, meta) = ctx.fresh_meta((**dom).clone());
                            cur_fun = Expr::App(Box::new(cur_fun), Box::new(meta.clone()));
                            cur_ty = oxilean_kernel::instantiate(cod, &meta);
                        }
                        _ => {
                            let (_id, meta) = ctx.fresh_meta((**dom).clone());
                            cur_fun = Expr::App(Box::new(cur_fun), Box::new(meta.clone()));
                            cur_ty = oxilean_kernel::instantiate(cod, &meta);
                        }
                    }
                }
                _ => {
                    return Ok(Expr::App(Box::new(cur_fun), Box::new(val_expr)));
                }
            }
        }
    } else {
        let fun_with_implicits = insert_implicit_args(ctx, fun_expr)?;
        Ok(Expr::App(Box::new(fun_with_implicits), Box::new(val_expr)))
    }
}
fn elaborate_lambda(
    ctx: &mut ElabContext,
    binders: &[oxilean_parse::Binder],
    body: &Located<SurfaceExpr>,
    expected_ty: Option<&Expr>,
) -> Result<Expr, ElabError> {
    if binders.is_empty() {
        return match expected_ty {
            Some(ety) => elaborate_with_expected_type(ctx, body, ety),
            None => elaborate_expr(ctx, body),
        };
    }
    let binder = &binders[0];
    let ty = if let Some(ty_surf) = &binder.ty {
        elaborate_expr(ctx, ty_surf)?
    } else if let Some(Expr::Pi(_, _, dom, _)) = expected_ty {
        (**dom).clone()
    } else {
        let (_id, meta) = ctx.fresh_meta(Expr::Sort(Level::succ(Level::zero())));
        meta
    };
    let _fvar = ctx.push_local(Name::str(&binder.name), ty.clone(), None);
    let inner_expected = match expected_ty {
        Some(Expr::Pi(_, _, _, cod)) => Some((**cod).clone()),
        _ => None,
    };
    let body_expr = if binders.len() > 1 {
        elaborate_lambda(ctx, &binders[1..], body, inner_expected.as_ref())?
    } else {
        match &inner_expected {
            Some(ety) => elaborate_with_expected_type(ctx, body, ety)?,
            None => elaborate_expr(ctx, body)?,
        }
    };
    ctx.pop_local();
    Ok(Expr::Lam(
        convert_binder_kind(&binder.info),
        Name::str(&binder.name),
        Box::new(ty),
        Box::new(body_expr),
    ))
}
fn elaborate_pi(
    ctx: &mut ElabContext,
    binders: &[oxilean_parse::Binder],
    body: &Located<SurfaceExpr>,
) -> Result<Expr, ElabError> {
    if binders.is_empty() {
        return elaborate_expr(ctx, body);
    }
    let binder = &binders[0];
    let ty = if let Some(ty_surf) = &binder.ty {
        elaborate_expr(ctx, ty_surf)?
    } else {
        return Err(ElabError::Other(
            "Pi binder requires type annotation".to_string(),
        ));
    };
    let _fvar = ctx.push_local(Name::str(&binder.name), ty.clone(), None);
    let body_expr = if binders.len() > 1 {
        elaborate_pi(ctx, &binders[1..], body)?
    } else {
        elaborate_expr(ctx, body)?
    };
    ctx.pop_local();
    Ok(Expr::Pi(
        convert_binder_kind(&binder.info),
        Name::str(&binder.name),
        Box::new(ty),
        Box::new(body_expr),
    ))
}
fn elaborate_let(
    ctx: &mut ElabContext,
    name: &str,
    ty_opt: Option<&Located<SurfaceExpr>>,
    val: &Located<SurfaceExpr>,
    body: &Located<SurfaceExpr>,
    expected_ty: Option<&Expr>,
) -> Result<Expr, ElabError> {
    let val_expr = elaborate_expr(ctx, val)?;
    let ty_expr = if let Some(ty_surf) = ty_opt {
        elaborate_expr(ctx, ty_surf)?
    } else {
        let (_id, meta) = ctx.fresh_meta(Expr::Sort(Level::succ(Level::zero())));
        meta
    };
    let _fvar = ctx.push_local(Name::str(name), ty_expr.clone(), Some(val_expr.clone()));
    let body_expr = match expected_ty {
        Some(ety) => elaborate_with_expected_type(ctx, body, ety)?,
        None => elaborate_expr(ctx, body)?,
    };
    ctx.pop_local();
    Ok(Expr::Let(
        Name::str(name),
        Box::new(ty_expr),
        Box::new(val_expr),
        Box::new(body_expr),
    ))
}
fn elaborate_annotation(
    ctx: &mut ElabContext,
    inner: &Located<SurfaceExpr>,
    ty: &Located<SurfaceExpr>,
) -> Result<Expr, ElabError> {
    let ty_expr = elaborate_expr(ctx, ty)?;
    elaborate_with_expected_type(ctx, inner, &ty_expr)
}
fn elaborate_hole(ctx: &mut ElabContext) -> Result<Expr, ElabError> {
    let ty = ctx
        .expected_type()
        .cloned()
        .unwrap_or_else(|| Expr::Sort(Level::zero()));
    let (_id, meta) = ctx.fresh_meta(ty);
    Ok(meta)
}
fn elaborate_proj(
    ctx: &mut ElabContext,
    inner: &Located<SurfaceExpr>,
    field: &str,
) -> Result<Expr, ElabError> {
    let expr_elab = elaborate_expr(ctx, inner)?;
    let idx = infer_projection_index(ctx, &expr_elab, field);
    Ok(Expr::Proj(Name::str(field), idx, Box::new(expr_elab)))
}
/// Try to determine the field index from the expression's type.
///
/// Looks up the inductive (structure) type of `expr` and searches for the
/// constructor field named `field`.  Returns the 0-based field index if
/// found, or 0 as a safe default.
///
/// Numeric field names (e.g. `.1`, `.2`) are handled as direct indices.
fn infer_projection_index(ctx: &ElabContext, expr: &Expr, field: &str) -> u32 {
    if let Ok(n) = field.parse::<u32>() {
        return n.saturating_sub(1);
    }
    let Some(ty) = try_infer_type(ctx, expr) else {
        return 0;
    };
    let Some(struct_name) = extract_head_const(&ty) else {
        return 0;
    };
    let Some(iv) = ctx.env().get_inductive_val(&struct_name) else {
        return 0;
    };
    let Some(ctor_name) = iv.ctors.first() else {
        return 0;
    };
    let Some(ctor_decl) = ctx.env().get(ctor_name) else {
        return 0;
    };
    let mut idx: u32 = 0;
    let mut cur = ctor_decl.ty();
    while let Expr::Pi(info, param_name, _, cod) = cur {
        if matches!(info, BinderInfo::Default) {
            let pname = format!("{}", param_name);
            if pname == field {
                return idx;
            }
            idx += 1;
        }
        cur = cod.as_ref();
    }
    0
}
/// Elaborate `if cond then t else e`.
///
/// Desugars to `@ite _ cond inst t e` where `inst` is a `Decidable cond` instance.
fn elaborate_if(
    ctx: &mut ElabContext,
    cond: &Located<SurfaceExpr>,
    then_branch: &Located<SurfaceExpr>,
    else_branch: &Located<SurfaceExpr>,
) -> Result<Expr, ElabError> {
    let cond_expr = elaborate_expr(ctx, cond)?;
    let then_expr = elaborate_expr(ctx, then_branch)?;
    let else_expr = elaborate_expr(ctx, else_branch)?;
    let ite_const = Expr::Const(Name::str("ite"), vec![]);
    let (_id, alpha_meta) = ctx.fresh_meta(Expr::Sort(Level::succ(Level::zero())));
    let decidable_ty = Expr::App(
        Box::new(Expr::Const(Name::str("Decidable"), vec![])),
        Box::new(cond_expr.clone()),
    );
    let (_id, inst_meta) = ctx.fresh_meta(decidable_ty);
    let result = mk_app5(
        ite_const, alpha_meta, cond_expr, inst_meta, then_expr, else_expr,
    );
    Ok(result)
}
/// Elaborate if-then-else with an expected type.
fn elaborate_if_with_expected(
    ctx: &mut ElabContext,
    cond: &Located<SurfaceExpr>,
    then_branch: &Located<SurfaceExpr>,
    else_branch: &Located<SurfaceExpr>,
    expected_ty: &Expr,
) -> Result<Expr, ElabError> {
    let cond_expr = elaborate_expr(ctx, cond)?;
    let then_expr = elaborate_with_expected_type(ctx, then_branch, expected_ty)?;
    let else_expr = elaborate_with_expected_type(ctx, else_branch, expected_ty)?;
    let ite_const = Expr::Const(Name::str("ite"), vec![]);
    let alpha = expected_ty.clone();
    let decidable_ty = Expr::App(
        Box::new(Expr::Const(Name::str("Decidable"), vec![])),
        Box::new(cond_expr.clone()),
    );
    let (_id, inst_meta) = ctx.fresh_meta(decidable_ty);
    let result = mk_app5(ite_const, alpha, cond_expr, inst_meta, then_expr, else_expr);
    Ok(result)
}
/// Elaborate `have h : T := proof; body`.
///
/// Desugars to `(fun (h : T) => body) proof`.
fn elaborate_have(
    ctx: &mut ElabContext,
    name: &str,
    ty: &Located<SurfaceExpr>,
    proof: &Located<SurfaceExpr>,
    body: &Located<SurfaceExpr>,
) -> Result<Expr, ElabError> {
    let ty_expr = elaborate_expr(ctx, ty)?;
    let proof_expr = elaborate_expr(ctx, proof)?;
    let _fvar = ctx.push_local(Name::str(name), ty_expr.clone(), None);
    let body_expr = elaborate_expr(ctx, body)?;
    ctx.pop_local();
    let lambda = Expr::Lam(
        BinderInfo::Default,
        Name::str(name),
        Box::new(ty_expr),
        Box::new(body_expr),
    );
    Ok(Expr::App(Box::new(lambda), Box::new(proof_expr)))
}
/// Elaborate `suffices h : T by tactic; body`.
///
/// Desugars similarly to have, but the proof obligation comes from the body.
/// `suffices h : T from body` becomes `(fun (h : T) => current_goal) body`
fn elaborate_suffices(
    ctx: &mut ElabContext,
    name: &str,
    ty: &Located<SurfaceExpr>,
    body: &Located<SurfaceExpr>,
) -> Result<Expr, ElabError> {
    let ty_expr = elaborate_expr(ctx, ty)?;
    let body_expr = elaborate_expr(ctx, body)?;
    let _fvar = ctx.push_local(Name::str(name), ty_expr.clone(), None);
    let (_id, goal_meta) = ctx.fresh_meta(Expr::Sort(Level::zero()));
    ctx.pop_local();
    let lambda = Expr::Lam(
        BinderInfo::Default,
        Name::str(name),
        Box::new(ty_expr),
        Box::new(goal_meta),
    );
    Ok(Expr::App(Box::new(lambda), Box::new(body_expr)))
}
/// Elaborate `show T from e`.
///
/// This is essentially a type annotation: elaborates `e` with expected type `T`.
fn elaborate_show(
    ctx: &mut ElabContext,
    ty: &Located<SurfaceExpr>,
    inner: &Located<SurfaceExpr>,
) -> Result<Expr, ElabError> {
    let ty_expr = elaborate_expr(ctx, ty)?;
    elaborate_with_expected_type(ctx, inner, &ty_expr)
}
/// Elaborate a match expression.
///
/// This creates a recursor/casesOn application for the scrutinee's type.
fn elaborate_match(
    ctx: &mut ElabContext,
    scrutinee: &Located<SurfaceExpr>,
    arms: &[oxilean_parse::MatchArm],
) -> Result<Expr, ElabError> {
    let scrutinee_expr = elaborate_expr(ctx, scrutinee)?;
    let mut arm_exprs = Vec::with_capacity(arms.len());
    for arm in arms {
        let rhs_expr = elaborate_expr(ctx, &arm.rhs)?;
        let arm_expr = if let Some(guard) = &arm.guard {
            let guard_expr = elaborate_expr(ctx, guard)?;
            let ite_const = Expr::Const(Name::str("ite"), vec![]);
            let (_id, alpha_meta) = ctx.fresh_meta(Expr::Sort(Level::succ(Level::zero())));
            let decidable_ty = Expr::App(
                Box::new(Expr::Const(Name::str("Decidable"), vec![])),
                Box::new(guard_expr.clone()),
            );
            let (_id2, inst_meta) = ctx.fresh_meta(decidable_ty);
            let sorry_else = Expr::Const(Name::str("sorry"), vec![]);
            mk_app5(
                ite_const, alpha_meta, guard_expr, inst_meta, rhs_expr, sorry_else,
            )
        } else {
            rhs_expr
        };
        arm_exprs.push(arm_expr);
    }
    if arms.is_empty() {
        return Err(ElabError::Other("match with no arms".to_string()));
    }
    let cases_on_name = infer_cases_on_name_from_arms(arms).unwrap_or_else(|| Name::str("casesOn"));
    let (_id, motive_meta) = ctx.fresh_meta(Expr::Sort(Level::succ(Level::zero())));
    let mut result = Expr::App(
        Box::new(Expr::Const(cases_on_name, vec![])),
        Box::new(motive_meta),
    );
    result = Expr::App(Box::new(result), Box::new(scrutinee_expr));
    for arm_expr in arm_exprs {
        result = Expr::App(Box::new(result), Box::new(arm_expr));
    }
    Ok(result)
}
/// Attempt to infer the `T.casesOn` constant name from the first constructor
/// pattern encountered in the match arms.
fn infer_cases_on_name_from_arms(arms: &[oxilean_parse::MatchArm]) -> Option<Name> {
    for arm in arms {
        if let Some(ctor) = ctor_name_from_pattern(&arm.pattern.value) {
            if let Some(type_name) = inductive_type_from_ctor(ctor) {
                return Some(Name::from_str(&format!("{}.casesOn", type_name)));
            }
        }
    }
    None
}
/// Extract the top-level constructor name from a pattern, if any.
fn ctor_name_from_pattern(pat: &oxilean_parse::Pattern) -> Option<&str> {
    match pat {
        oxilean_parse::Pattern::Ctor(name, _) => Some(name.as_str()),
        _ => None,
    }
}
/// Map a constructor name to the name of the inductive type that owns it.
/// Returns `None` for constructors of user-defined types (unknown statically).
fn inductive_type_from_ctor(ctor: &str) -> Option<&'static str> {
    let base = if let Some(pos) = ctor.rfind('.') {
        &ctor[pos + 1..]
    } else {
        ctor
    };
    match base {
        "true" | "false" => Some("Bool"),
        "zero" | "succ" => Some("Nat"),
        "nil" | "cons" => Some("List"),
        "some" | "none" => Some("Option"),
        "inl" | "inr" => Some("Sum"),
        "mk" => Some("Prod"),
        "unit" => Some("Unit"),
        "ok" | "error" => Some("Except"),
        _ => None,
    }
}
/// Elaborate do notation by desugaring into bind/pure/let chains.
///
/// - `do { let x := e; body }` → `let x := e in body`
/// - `do { x <- e; body }` → `bind e (fun x => body)`
/// - `do { return e }` → `pure e`
/// - `do { e }` → `e`
fn elaborate_do(
    ctx: &mut ElabContext,
    actions: &[oxilean_parse::DoAction],
) -> Result<Expr, ElabError> {
    if actions.is_empty() {
        return Err(ElabError::Other("empty do block".to_string()));
    }
    elaborate_do_actions(ctx, actions, 0)
}
fn elaborate_do_actions(
    ctx: &mut ElabContext,
    actions: &[oxilean_parse::DoAction],
    idx: usize,
) -> Result<Expr, ElabError> {
    if idx >= actions.len() {
        return Err(ElabError::Other("unexpected end of do block".to_string()));
    }
    let is_last = idx == actions.len() - 1;
    match &actions[idx] {
        oxilean_parse::DoAction::Let(name, val) => {
            let val_expr = elaborate_expr(ctx, val)?;
            if is_last {
                Ok(val_expr)
            } else {
                let ty_meta = {
                    let (_id, meta) = ctx.fresh_meta(Expr::Sort(Level::succ(Level::zero())));
                    meta
                };
                let _fvar =
                    ctx.push_local(Name::str(name), ty_meta.clone(), Some(val_expr.clone()));
                let rest = elaborate_do_actions(ctx, actions, idx + 1)?;
                ctx.pop_local();
                Ok(Expr::Let(
                    Name::str(name),
                    Box::new(ty_meta),
                    Box::new(val_expr),
                    Box::new(rest),
                ))
            }
        }
        oxilean_parse::DoAction::LetTyped(name, ty, val) => {
            let ty_expr = elaborate_expr(ctx, ty)?;
            let val_expr = elaborate_expr(ctx, val)?;
            if is_last {
                Ok(val_expr)
            } else {
                let _fvar =
                    ctx.push_local(Name::str(name), ty_expr.clone(), Some(val_expr.clone()));
                let rest = elaborate_do_actions(ctx, actions, idx + 1)?;
                ctx.pop_local();
                Ok(Expr::Let(
                    Name::str(name),
                    Box::new(ty_expr),
                    Box::new(val_expr),
                    Box::new(rest),
                ))
            }
        }
        oxilean_parse::DoAction::Bind(name, monadic_expr) => {
            let m_expr = elaborate_expr(ctx, monadic_expr)?;
            if is_last {
                Ok(m_expr)
            } else {
                let bind_const = Expr::Const(Name::str("Bind.bind"), vec![]);
                let ty_meta = {
                    let (_id, meta) = ctx.fresh_meta(Expr::Sort(Level::succ(Level::zero())));
                    meta
                };
                let _fvar = ctx.push_local(Name::str(name), ty_meta.clone(), None);
                let rest = elaborate_do_actions(ctx, actions, idx + 1)?;
                ctx.pop_local();
                let callback = Expr::Lam(
                    BinderInfo::Default,
                    Name::str(name),
                    Box::new(ty_meta),
                    Box::new(rest),
                );
                Ok(Expr::App(
                    Box::new(Expr::App(Box::new(bind_const), Box::new(m_expr))),
                    Box::new(callback),
                ))
            }
        }
        oxilean_parse::DoAction::Expr(expr) => {
            let e = elaborate_expr(ctx, expr)?;
            if is_last {
                Ok(e)
            } else {
                let bind_const = Expr::Const(Name::str("Bind.bind"), vec![]);
                let rest = elaborate_do_actions(ctx, actions, idx + 1)?;
                let unit_ty = Expr::Const(Name::str("Unit"), vec![]);
                let callback = Expr::Lam(
                    BinderInfo::Default,
                    Name::str("_"),
                    Box::new(unit_ty),
                    Box::new(rest),
                );
                Ok(Expr::App(
                    Box::new(Expr::App(Box::new(bind_const), Box::new(e))),
                    Box::new(callback),
                ))
            }
        }
        oxilean_parse::DoAction::Return(expr) => {
            let e = elaborate_expr(ctx, expr)?;
            let pure_const = Expr::Const(Name::str("Pure.pure"), vec![]);
            Ok(Expr::App(Box::new(pure_const), Box::new(e)))
        }
    }
}
/// Elaborate `return e` (outside of do notation).
///
/// Desugars to `Pure.pure e`.
fn elaborate_return(
    ctx: &mut ElabContext,
    inner: &Located<SurfaceExpr>,
) -> Result<Expr, ElabError> {
    let inner_expr = elaborate_expr(ctx, inner)?;
    let pure_const = Expr::Const(Name::str("Pure.pure"), vec![]);
    Ok(Expr::App(Box::new(pure_const), Box::new(inner_expr)))
}
/// Elaborate `[a, b, c]` into `List.cons a (List.cons b (List.cons c List.nil))`.
fn elaborate_list_lit(
    ctx: &mut ElabContext,
    elems: &[Located<SurfaceExpr>],
) -> Result<Expr, ElabError> {
    let mut result = Expr::Const(Name::str("List.nil"), vec![]);
    for elem in elems.iter().rev() {
        let elem_expr = elaborate_expr(ctx, elem)?;
        let cons = Expr::Const(Name::str("List.cons"), vec![]);
        result = Expr::App(
            Box::new(Expr::App(Box::new(cons), Box::new(elem_expr))),
            Box::new(result),
        );
    }
    Ok(result)
}
/// Elaborate list literal with expected type to extract element type.
fn elaborate_list_lit_with_expected(
    ctx: &mut ElabContext,
    elems: &[Located<SurfaceExpr>],
    expected_ty: &Expr,
) -> Result<Expr, ElabError> {
    let elem_ty = if let Expr::App(f, a) = expected_ty {
        if let Expr::Const(name, _) = f.as_ref() {
            if format!("{}", name) == "List" {
                Some((**a).clone())
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };
    let nil = if let Some(ref ety) = elem_ty {
        Expr::App(
            Box::new(Expr::Const(Name::str("List.nil"), vec![])),
            Box::new(ety.clone()),
        )
    } else {
        Expr::Const(Name::str("List.nil"), vec![])
    };
    let mut result = nil;
    for elem in elems.iter().rev() {
        let elem_expr = match &elem_ty {
            Some(ety) => elaborate_with_expected_type(ctx, elem, ety)?,
            None => elaborate_expr(ctx, elem)?,
        };
        let cons = Expr::Const(Name::str("List.cons"), vec![]);
        result = Expr::App(
            Box::new(Expr::App(Box::new(cons), Box::new(elem_expr))),
            Box::new(result),
        );
    }
    Ok(result)
}
/// Elaborate `(a, b)` into `Prod.mk a b`.
/// Elaborate `(a, b, c)` into `Prod.mk a (Prod.mk b c)`.
fn elaborate_tuple(
    ctx: &mut ElabContext,
    elems: &[Located<SurfaceExpr>],
) -> Result<Expr, ElabError> {
    if elems.is_empty() {
        return Ok(Expr::Const(Name::str("Unit.unit"), vec![]));
    }
    if elems.len() == 1 {
        return elaborate_expr(ctx, &elems[0]);
    }
    let mut result = elaborate_expr(ctx, &elems[elems.len() - 1])?;
    for elem in elems[..elems.len() - 1].iter().rev() {
        let elem_expr = elaborate_expr(ctx, elem)?;
        let prod_mk = Expr::Const(Name::str("Prod.mk"), vec![]);
        result = Expr::App(
            Box::new(Expr::App(Box::new(prod_mk), Box::new(elem_expr))),
            Box::new(result),
        );
    }
    Ok(result)
}
/// Elaborate `⟨a, b, c⟩` into a constructor application based on expected type.
///
/// Without expected type, falls back to creating a generic application.
fn elaborate_anonymous_ctor(
    ctx: &mut ElabContext,
    fields: &[Located<SurfaceExpr>],
) -> Result<Expr, ElabError> {
    let mut field_exprs = Vec::with_capacity(fields.len());
    for field in fields {
        field_exprs.push(elaborate_expr(ctx, field)?);
    }
    let (_id, ctor_meta) = ctx.fresh_meta(Expr::Sort(Level::succ(Level::zero())));
    let mut result = ctor_meta;
    for field_expr in field_exprs {
        result = Expr::App(Box::new(result), Box::new(field_expr));
    }
    Ok(result)
}
/// Elaborate anonymous constructor with expected type.
///
/// Uses the expected type to find the appropriate constructor.
fn elaborate_anonymous_ctor_with_expected(
    ctx: &mut ElabContext,
    fields: &[Located<SurfaceExpr>],
    expected_ty: &Expr,
) -> Result<Expr, ElabError> {
    let mut field_exprs = Vec::with_capacity(fields.len());
    for field in fields {
        field_exprs.push(elaborate_expr(ctx, field)?);
    }
    let ctor = if let Some(type_name) = extract_head_const(expected_ty) {
        if let Some(iv) = ctx.env().get_inductive_val(&type_name) {
            if let Some(ctor_name) = iv.ctors.first() {
                Expr::Const(ctor_name.clone(), Vec::new())
            } else {
                let (_id, meta) = ctx.fresh_meta(expected_ty.clone());
                meta
            }
        } else {
            let (_id, meta) = ctx.fresh_meta(expected_ty.clone());
            meta
        }
    } else {
        let (_id, meta) = ctx.fresh_meta(expected_ty.clone());
        meta
    };
    let mut result = ctor;
    for field_expr in field_exprs {
        result = Expr::App(Box::new(result), Box::new(field_expr));
    }
    Ok(result)
}
/// Elaborate `s!"hello {name}"` into string concatenation.
///
/// Each interpolation is converted to `toString` and concatenated with `String.append`.
fn elaborate_string_interp(ctx: &mut ElabContext, parts: &[StringPart]) -> Result<Expr, ElabError> {
    if parts.is_empty() {
        return Ok(Expr::Lit(oxilean_kernel::Literal::Str(String::new())));
    }
    let mut result: Option<Expr> = None;
    for part in parts {
        let part_expr = match part {
            StringPart::Literal(s) => Expr::Lit(oxilean_kernel::Literal::Str(s.clone())),
            StringPart::Interpolation(tokens) => {
                let elaborated = if tokens.is_empty() {
                    elaborate_hole(ctx)?
                } else {
                    let src: String = tokens.iter().map(|t| format!("{} ", t.kind)).collect();
                    let fresh_tokens = Lexer::new(&src).tokenize();
                    let parsed = Parser::new(fresh_tokens).parse_expr().map_err(|e| {
                        ElabError::Other(format!("string interpolation parse error: {e}"))
                    })?;
                    elaborate_expr(ctx, &parsed)?
                };
                let to_string = Expr::Const(Name::str("toString"), vec![]);
                Expr::App(Box::new(to_string), Box::new(elaborated))
            }
        };
        result = Some(match result {
            None => part_expr,
            Some(acc) => {
                let append = Expr::Const(Name::str("String.append"), vec![]);
                Expr::App(
                    Box::new(Expr::App(Box::new(append), Box::new(acc))),
                    Box::new(part_expr),
                )
            }
        });
    }
    Ok(result.unwrap_or_else(|| Expr::Lit(oxilean_kernel::Literal::Str(String::new()))))
}
/// Elaborate range expressions.
///
/// - `a..b` → `Range.mk a b`
/// - `..b` → `Range.mk 0 b`
/// - `a..` → open range starting at a
fn elaborate_range(
    ctx: &mut ElabContext,
    lo: Option<&Located<SurfaceExpr>>,
    hi: Option<&Located<SurfaceExpr>>,
) -> Result<Expr, ElabError> {
    let range_mk = Expr::Const(Name::str("Range.mk"), vec![]);
    let lo_expr = match lo {
        Some(e) => elaborate_expr(ctx, e)?,
        None => Expr::Lit(oxilean_kernel::Literal::Nat(0)),
    };
    let hi_expr = match hi {
        Some(e) => elaborate_expr(ctx, e)?,
        None => {
            let (_id, meta) = ctx.fresh_meta(Expr::Const(Name::str("Nat"), vec![]));
            meta
        }
    };
    Ok(Expr::App(
        Box::new(Expr::App(Box::new(range_mk), Box::new(lo_expr))),
        Box::new(hi_expr),
    ))
}
/// Elaborate `by tactic1; tactic2; ...`.
///
/// Builds a tactic proof state from the current local context, runs each
/// tactic sequentially, and assigns the resulting proof metavariable.
/// If the tactic block completes all goals, the metavar is assigned to a
/// `sorry`-axiom proof term (the tactic engine does not yet construct real
/// proof terms). If tactics fail or leave goals unsolved the metavar remains
/// unassigned so the constraint solver may attempt to unify it later.
fn elaborate_by_tactic(
    ctx: &mut ElabContext,
    tactics: &[Located<String>],
) -> Result<Expr, ElabError> {
    let goal_ty = match ctx.expected_type() {
        Some(ty) => ty.clone(),
        None => {
            let (_ty_id, ty_meta) = ctx.fresh_meta(Expr::Sort(Level::succ(Level::zero())));
            ty_meta
        }
    };
    let (proof_id, proof_meta) = ctx.fresh_meta(goal_ty.clone());
    let hyps: Vec<(Name, Expr)> = ctx
        .hypotheses()
        .into_iter()
        .map(|(n, ty)| (n.clone(), ty.clone()))
        .collect();
    let mut goal = crate::tactic::Goal::new(Name::str("main"), goal_ty);
    for (name, ty) in hyps {
        goal.add_hypothesis(name, ty);
    }
    let mut state = crate::tactic::TacticState::new();
    state.add_goal(goal);
    let tactic_strs: Vec<String> = tactics.iter().map(|t| t.value.clone()).collect();
    match crate::tactic::eval_tactic_block(&state, &tactic_strs) {
        Ok(final_state) if final_state.is_complete() => {
            let sorry_term = Expr::Const(Name::str("sorry"), vec![]);
            ctx.assign_meta(proof_id, sorry_term);
        }
        _ => {}
    }
    Ok(proof_meta)
}
/// Elaborate a `calc` block.
///
/// Each calc step `a rel b := proof` is chained with transitivity.
/// The overall result is a chain of transitivity proofs.
fn elaborate_calc(
    ctx: &mut ElabContext,
    steps: &[oxilean_parse::AstCalcStep],
) -> Result<Expr, ElabError> {
    if steps.is_empty() {
        return Err(ElabError::Other("empty calc block".to_string()));
    }
    let first_step = &steps[0];
    let mut proof = elaborate_expr(ctx, &first_step.proof)?;
    for step in &steps[1..] {
        let step_proof = elaborate_expr(ctx, &step.proof)?;
        let trans_name = match step.rel.as_str() {
            "=" => "Eq.trans",
            "<" => "lt_trans",
            "<=" | "≤" => "le_trans",
            ">" => "gt_trans",
            ">=" | "≥" => "ge_trans",
            _ => "Trans.trans",
        };
        let trans_const = Expr::Const(Name::str(trans_name), vec![]);
        proof = Expr::App(
            Box::new(Expr::App(Box::new(trans_const), Box::new(proof))),
            Box::new(step_proof),
        );
    }
    Ok(proof)
}
/// Try each overloaded candidate and pick the one that type-checks.
///
/// When a name resolves to multiple constants, this function elaborates
/// the expression with each candidate and returns the first one that succeeds.
#[allow(dead_code)]
pub fn resolve_overload(
    ctx: &mut ElabContext,
    candidates: &[Name],
    args: &[Located<SurfaceExpr>],
) -> Result<Expr, ElabError> {
    let mut errors = Vec::new();
    for candidate in candidates {
        let mut fun = Expr::Const(candidate.clone(), vec![]);
        let mut success = true;
        let mut elaborated_args = Vec::new();
        for arg in args {
            match elaborate_expr(ctx, arg) {
                Ok(arg_expr) => elaborated_args.push(arg_expr),
                Err(e) => {
                    errors.push((candidate.clone(), e));
                    success = false;
                    break;
                }
            }
        }
        if success {
            for arg_expr in elaborated_args {
                fun = Expr::App(Box::new(fun), Box::new(arg_expr));
            }
            return Ok(fun);
        }
    }
    if candidates.is_empty() {
        Err(ElabError::OverloadAmbiguity(
            "no candidates found".to_string(),
        ))
    } else {
        Err(ElabError::OverloadAmbiguity(format!(
            "none of {} candidates type-checked: {:?}",
            candidates.len(),
            errors
                .iter()
                .map(|(n, _)| format!("{}", n))
                .collect::<Vec<_>>()
        )))
    }
}
/// Convert a parse `BinderKind` to a kernel `BinderInfo`.
fn convert_binder_kind(kind: &oxilean_parse::BinderKind) -> oxilean_kernel::BinderInfo {
    match kind {
        oxilean_parse::BinderKind::Default => oxilean_kernel::BinderInfo::Default,
        oxilean_parse::BinderKind::Implicit => oxilean_kernel::BinderInfo::Implicit,
        oxilean_parse::BinderKind::Instance => oxilean_kernel::BinderInfo::InstImplicit,
        oxilean_parse::BinderKind::StrictImplicit => oxilean_kernel::BinderInfo::StrictImplicit,
    }
}
/// Convert a parse `Literal` to a kernel `Literal`.
fn convert_literal(lit: oxilean_parse::Literal) -> oxilean_kernel::Literal {
    match lit {
        oxilean_parse::Literal::Nat(n) => oxilean_kernel::Literal::Nat(n),
        oxilean_parse::Literal::String(s) => oxilean_kernel::Literal::Str(s),
        oxilean_parse::Literal::Char(c) => oxilean_kernel::Literal::Str(c.to_string()),
        oxilean_parse::Literal::Float(_) => oxilean_kernel::Literal::Nat(0),
    }
}
/// Build a 5-argument application.
fn mk_app5(f: Expr, a1: Expr, a2: Expr, a3: Expr, a4: Expr, a5: Expr) -> Expr {
    let app1 = Expr::App(Box::new(f), Box::new(a1));
    let app2 = Expr::App(Box::new(app1), Box::new(a2));
    let app3 = Expr::App(Box::new(app2), Box::new(a3));
    let app4 = Expr::App(Box::new(app3), Box::new(a4));
    Expr::App(Box::new(app4), Box::new(a5))
}
/// Build a 2-argument application.
#[allow(dead_code)]
fn mk_app2(f: Expr, a1: Expr, a2: Expr) -> Expr {
    Expr::App(Box::new(Expr::App(Box::new(f), Box::new(a1))), Box::new(a2))
}
/// Build a 3-argument application.
#[allow(dead_code)]
fn mk_app3(f: Expr, a1: Expr, a2: Expr, a3: Expr) -> Expr {
    let app1 = Expr::App(Box::new(f), Box::new(a1));
    let app2 = Expr::App(Box::new(app1), Box::new(a2));
    Expr::App(Box::new(app2), Box::new(a3))
}
/// Elaborate an explicit application (`@f args...`), which suppresses
/// implicit argument insertion.
#[allow(dead_code)]
pub fn elaborate_explicit_app(
    ctx: &mut ElabContext,
    fun: &Located<SurfaceExpr>,
    arg: &Located<SurfaceExpr>,
) -> Result<Expr, ElabError> {
    elaborate_app(ctx, fun, arg, true)
}
