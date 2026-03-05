//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::basic::{MVarId, MetaContext};
use crate::infer_type::MetaInferType;
use crate::tactic::ring::functions::{expr_to_polynomial, simplify_add_mul_pow};
use crate::tactic::simp::main::simp as simp_expr;
use crate::tactic::simp::types::{SimpConfig, SimpResult, SimpTheorems};
use crate::tactic::state::{TacticError, TacticResult, TacticState};
use oxilean_kernel::{BinderInfo, Expr, Level, Literal, Name};

use super::types::{
    ConvConfig, ConvDirection, ConvEntrySide, ConvLocalProof, ConvOperation, ConvPath,
    ConvPathStep, ConvResult, ConvSimpConfig, ConvState, ConvStats, ConvTarget,
};

/// Maximum depth for nested conv navigation.
const MAX_CONV_DEPTH: usize = 64;
/// Maximum number of rewrite steps inside a conv session.
pub(super) const MAX_CONV_REWRITES: usize = 256;
/// Default conv configuration max depth.
pub(super) const DEFAULT_CONV_MAX_DEPTH: usize = 32;
/// Parse an expression as `@Eq α lhs rhs`, returning `(α, lhs, rhs)`.
pub(super) fn parse_eq_expr(expr: &Expr) -> Option<(Expr, Expr, Expr)> {
    if let Expr::App(f1, rhs) = expr {
        if let Expr::App(f2, lhs) = f1.as_ref() {
            if let Expr::App(eq_const, ty) = f2.as_ref() {
                if let Expr::Const(name, _) = &**eq_const {
                    if name.to_string().contains("Eq") || name.to_string().contains("eq") {
                        return Some((*ty.clone(), *lhs.clone(), *rhs.clone()));
                    }
                }
            }
        }
    }
    None
}
/// Parse a relational expression `lhs R rhs` where R is a binary relation.
/// Returns `(lhs, rhs, relation_name)`.
pub(super) fn parse_relation(expr: &Expr) -> Option<(Expr, Expr, Name)> {
    if let Expr::App(f1, rhs) = expr {
        if let Expr::App(rel, lhs) = f1.as_ref() {
            match rel.as_ref() {
                Expr::Const(name, _) => {
                    return Some((*lhs.clone(), *rhs.clone(), name.clone()));
                }
                Expr::App(inner_f, _) => {
                    if let Some(name) = get_head_const(inner_f) {
                        return Some((*lhs.clone(), *rhs.clone(), name));
                    }
                }
                _ => {}
            }
        }
    }
    None
}
/// Get the head constant of an expression (unwrapping applications).
pub(super) fn get_head_const(expr: &Expr) -> Option<Name> {
    match expr {
        Expr::Const(name, _) => Some(name.clone()),
        Expr::App(f, _) => get_head_const(f),
        _ => None,
    }
}
/// Collect the arguments of a curried application.
/// `f a1 a2 ... an` -> `(f, [a1, a2, ..., an])`
pub(super) fn collect_app_args(expr: &Expr) -> (Expr, Vec<Expr>) {
    let mut args = Vec::new();
    let mut head = expr.clone();
    while let Expr::App(f, a) = head {
        args.push(*a);
        head = *f;
    }
    args.reverse();
    (head, args)
}
/// Rebuild an application from head and arguments.
pub(super) fn mk_app(head: Expr, args: Vec<Expr>) -> Expr {
    let mut result = head;
    for arg in args {
        result = Expr::App(Box::new(result), Box::new(arg));
    }
    result
}
/// Build an equality expression: `@Eq α lhs rhs`.
pub(super) fn mk_eq(ty: &Expr, lhs: &Expr, rhs: &Expr) -> Expr {
    let eq_const = Expr::Const(Name::str("Eq"), vec![Level::Zero]);
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::App(Box::new(eq_const), Box::new(ty.clone()))),
            Box::new(lhs.clone()),
        )),
        Box::new(rhs.clone()),
    )
}
/// Build `@Eq.refl α a` : `a = a`.
pub(super) fn mk_eq_refl(ty: &Expr, a: &Expr) -> Expr {
    let refl_const = Expr::Const(Name::str("Eq").append_str("refl"), vec![Level::Zero]);
    Expr::App(
        Box::new(Expr::App(Box::new(refl_const), Box::new(ty.clone()))),
        Box::new(a.clone()),
    )
}
/// Build `@Eq.trans α a b c hab hbc` : if `a = b` and `b = c` then `a = c`.
#[allow(clippy::too_many_arguments)]
pub(super) fn mk_eq_trans(ty: &Expr, a: &Expr, b: &Expr, c: &Expr, hab: Expr, hbc: Expr) -> Expr {
    let trans_const = Expr::Const(Name::str("Eq").append_str("trans"), vec![Level::Zero]);
    mk_app(
        trans_const,
        vec![ty.clone(), a.clone(), b.clone(), c.clone(), hab, hbc],
    )
}
/// Build `@congr_arg α β f a b hab` : if `a = b` then `f a = f b`.
#[allow(clippy::too_many_arguments)]
pub(super) fn mk_congr_arg(
    alpha: &Expr,
    beta: &Expr,
    f: &Expr,
    a: &Expr,
    b: &Expr,
    hab: Expr,
) -> Expr {
    let congr_const = Expr::Const(Name::str("congr_arg"), vec![Level::Zero, Level::Zero]);
    mk_app(
        congr_const,
        vec![
            alpha.clone(),
            beta.clone(),
            f.clone(),
            a.clone(),
            b.clone(),
            hab,
        ],
    )
}
/// Build `@congr_fun α β f g hfg a` : if `f = g` then `f a = g a`.
#[allow(clippy::too_many_arguments)]
pub(super) fn mk_congr_fun(
    alpha: &Expr,
    beta: &Expr,
    f: &Expr,
    g: &Expr,
    hfg: Expr,
    a: &Expr,
) -> Expr {
    let congr_const = Expr::Const(Name::str("congr_fun"), vec![Level::Zero, Level::Zero]);
    mk_app(
        congr_const,
        vec![
            alpha.clone(),
            beta.clone(),
            f.clone(),
            g.clone(),
            hfg,
            a.clone(),
        ],
    )
}
/// Build `@congr α β f g a b hfg hab` : if `f = g` and `a = b` then `f a = g b`.
#[allow(clippy::too_many_arguments)]
pub(super) fn mk_congr(
    alpha: &Expr,
    beta: &Expr,
    f: &Expr,
    g: &Expr,
    a: &Expr,
    b: &Expr,
    hfg: Expr,
    hab: Expr,
) -> Expr {
    let congr_const = Expr::Const(Name::str("congr"), vec![Level::Zero, Level::Zero]);
    mk_app(
        congr_const,
        vec![
            alpha.clone(),
            beta.clone(),
            f.clone(),
            g.clone(),
            a.clone(),
            b.clone(),
            hfg,
            hab,
        ],
    )
}
/// Build `@funext α β f g h` : if `∀ x, f x = g x` then `f = g`.
pub(super) fn mk_funext(alpha: &Expr, beta: &Expr, f: &Expr, g: &Expr, h: Expr) -> Expr {
    let funext_const = Expr::Const(Name::str("funext"), vec![Level::Zero, Level::Zero]);
    mk_app(
        funext_const,
        vec![alpha.clone(), beta.clone(), f.clone(), g.clone(), h],
    )
}
/// Attempt to get the argument type and return type of a Pi/arrow type.
pub(super) fn decompose_pi(ty: &Expr) -> Option<(BinderInfo, Name, Expr, Expr)> {
    if let Expr::Pi(bi, name, dom, cod) = ty {
        Some((*bi, name.clone(), *dom.clone(), *cod.clone()))
    } else {
        None
    }
}
/// Attempt to get the binder info and body of a lambda.
pub(super) fn decompose_lambda(expr: &Expr) -> Option<(BinderInfo, Name, Expr, Expr)> {
    if let Expr::Lam(bi, name, ty, body) = expr {
        Some((*bi, name.clone(), *ty.clone(), *body.clone()))
    } else {
        None
    }
}
/// Enter conv mode, focusing on a specific target within the current goal.
///
/// The goal must be of the form `lhs R rhs` (typically `lhs = rhs`).
/// Returns a `ConvState` tracking the focused sub-expression and navigation path.
pub fn enter_conv(
    target: ConvTarget,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<ConvState> {
    let goal = state.current_goal()?;
    let goal_type = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let goal_type = ctx.instantiate_mvars(&goal_type);
    match target {
        ConvTarget::Lhs => enter_conv_side(&goal_type, goal, ConvEntrySide::Lhs, ctx),
        ConvTarget::Rhs => enter_conv_side(&goal_type, goal, ConvEntrySide::Rhs, ctx),
        ConvTarget::Fun => enter_conv_fun(&goal_type, goal, ctx),
        ConvTarget::Arg(n) => enter_conv_arg_top(n, &goal_type, goal, ctx),
        ConvTarget::Pattern(pat) => enter_conv_pattern(&pat, &goal_type, goal, ctx),
        ConvTarget::Enter(dirs) => enter_conv_directions(&dirs, &goal_type, goal, ctx),
    }
}
/// Enter conv mode on a specific side (lhs or rhs) of an equality.
pub(super) fn enter_conv_side(
    goal_type: &Expr,
    goal_mvar: MVarId,
    side: ConvEntrySide,
    _ctx: &mut MetaContext,
) -> TacticResult<ConvState> {
    if let Some((ty, lhs, rhs)) = parse_eq_expr(goal_type) {
        let focused = match side {
            ConvEntrySide::Lhs => lhs,
            ConvEntrySide::Rhs => rhs,
            ConvEntrySide::Whole => goal_type.clone(),
        };
        let mut conv_state = ConvState::new(focused, goal_type.clone(), goal_mvar, side);
        conv_state.eq_type = Some(ty);
        return Ok(conv_state);
    }
    if let Some((lhs, rhs, _rel)) = parse_relation(goal_type) {
        let focused = match side {
            ConvEntrySide::Lhs => lhs,
            ConvEntrySide::Rhs => rhs,
            ConvEntrySide::Whole => goal_type.clone(),
        };
        let conv_state = ConvState::new(focused, goal_type.clone(), goal_mvar, side);
        return Ok(conv_state);
    }
    Err(TacticError::GoalMismatch(
        "conv requires a goal of the form `lhs R rhs`".to_string(),
    ))
}
/// Enter conv mode focusing on the function part of the goal.
pub(super) fn enter_conv_fun(
    goal_type: &Expr,
    goal_mvar: MVarId,
    _ctx: &mut MetaContext,
) -> TacticResult<ConvState> {
    let (head, _args) = collect_app_args(goal_type);
    if _args.is_empty() {
        return Err(TacticError::GoalMismatch(
            "conv fun: goal is not an application".to_string(),
        ));
    }
    let mut conv_state = ConvState::new(
        head.clone(),
        goal_type.clone(),
        goal_mvar,
        ConvEntrySide::Whole,
    );
    conv_state
        .path
        .push(ConvPathStep::new(ConvDirection::Fun, goal_type.clone(), 0));
    Ok(conv_state)
}
/// Enter conv mode focusing on the nth argument of the top-level application.
pub(super) fn enter_conv_arg_top(
    n: usize,
    goal_type: &Expr,
    goal_mvar: MVarId,
    _ctx: &mut MetaContext,
) -> TacticResult<ConvState> {
    let (_head, args) = collect_app_args(goal_type);
    if n >= args.len() {
        return Err(TacticError::Failed(format!(
            "conv arg: argument index {} out of range (have {} args)",
            n,
            args.len()
        )));
    }
    let focused = args[n].clone();
    let mut conv_state =
        ConvState::new(focused, goal_type.clone(), goal_mvar, ConvEntrySide::Whole);
    conv_state.path.push(ConvPathStep::new(
        ConvDirection::Arg(n),
        goal_type.clone(),
        n,
    ));
    Ok(conv_state)
}
/// Enter conv mode by matching a pattern in the goal.
pub(super) fn enter_conv_pattern(
    pattern: &Expr,
    goal_type: &Expr,
    goal_mvar: MVarId,
    _ctx: &mut MetaContext,
) -> TacticResult<ConvState> {
    if let Some(path_steps) = find_pattern_in_expr(pattern, goal_type) {
        let focused = pattern.clone();
        let mut conv_state =
            ConvState::new(focused, goal_type.clone(), goal_mvar, ConvEntrySide::Whole);
        for step in path_steps {
            conv_state.path.push(step);
        }
        Ok(conv_state)
    } else {
        Err(TacticError::Failed(
            "conv pattern: pattern not found in goal".to_string(),
        ))
    }
}
/// Enter conv mode following a sequence of navigation directions.
pub(super) fn enter_conv_directions(
    directions: &[ConvDirection],
    goal_type: &Expr,
    goal_mvar: MVarId,
    _ctx: &mut MetaContext,
) -> TacticResult<ConvState> {
    let mut current = goal_type.clone();
    let mut conv_state = ConvState::new(
        current.clone(),
        goal_type.clone(),
        goal_mvar,
        ConvEntrySide::Whole,
    );
    for dir in directions {
        let step_result = navigate_one_step(dir, &current)?;
        conv_state.path.push(ConvPathStep::new(
            dir.clone(),
            current.clone(),
            step_result.1,
        ));
        current = step_result.0;
    }
    conv_state.focused = current;
    Ok(conv_state)
}
/// Navigate one step in the given direction, returning the new focused expression
/// and the position index.
pub(super) fn navigate_one_step(dir: &ConvDirection, expr: &Expr) -> TacticResult<(Expr, usize)> {
    match dir {
        ConvDirection::Left => {
            if let Expr::App(f, _a) = expr {
                Ok((*f.clone(), 0))
            } else {
                Err(TacticError::Failed(
                    "conv left: expression is not an application".to_string(),
                ))
            }
        }
        ConvDirection::Right => {
            if let Expr::App(_f, a) = expr {
                Ok((*a.clone(), 1))
            } else {
                Err(TacticError::Failed(
                    "conv right: expression is not an application".to_string(),
                ))
            }
        }
        ConvDirection::Arg(n) => {
            let (_head, args) = collect_app_args(expr);
            if *n >= args.len() {
                return Err(TacticError::Failed(format!(
                    "conv arg: argument index {} out of range (have {} args)",
                    n,
                    args.len()
                )));
            }
            Ok((args[*n].clone(), *n))
        }
        ConvDirection::Fun => {
            let (head, args) = collect_app_args(expr);
            if args.is_empty() {
                return Err(TacticError::Failed(
                    "conv fun: expression is not an application".to_string(),
                ));
            }
            Ok((head, 0))
        }
        ConvDirection::Ext => {
            if let Expr::Lam(_bi, _name, _ty, body) = expr {
                Ok((*body.clone(), 0))
            } else {
                Err(TacticError::Failed(
                    "conv ext: expression is not a lambda".to_string(),
                ))
            }
        }
    }
}
/// Search for a pattern within an expression, returning the navigation path if found.
pub(super) fn find_pattern_in_expr(pattern: &Expr, expr: &Expr) -> Option<Vec<ConvPathStep>> {
    if exprs_syntactically_equal(pattern, expr) {
        return Some(Vec::new());
    }
    match expr {
        Expr::App(f, a) => {
            if let Some(mut path) = find_pattern_in_expr(pattern, f) {
                path.insert(0, ConvPathStep::new(ConvDirection::Left, expr.clone(), 0));
                return Some(path);
            }
            if let Some(mut path) = find_pattern_in_expr(pattern, a) {
                path.insert(0, ConvPathStep::new(ConvDirection::Right, expr.clone(), 1));
                return Some(path);
            }
            None
        }
        Expr::Lam(_bi, _name, _ty, body) => {
            if let Some(mut path) = find_pattern_in_expr(pattern, body) {
                path.insert(0, ConvPathStep::new(ConvDirection::Ext, expr.clone(), 0));
                return Some(path);
            }
            None
        }
        _ => None,
    }
}
/// Shallow syntactic equality check.
pub(super) fn exprs_syntactically_equal(a: &Expr, b: &Expr) -> bool {
    match (a, b) {
        (Expr::Sort(l1), Expr::Sort(l2)) => l1 == l2,
        (Expr::BVar(i1), Expr::BVar(i2)) => i1 == i2,
        (Expr::FVar(id1), Expr::FVar(id2)) => id1 == id2,
        (Expr::Const(n1, ls1), Expr::Const(n2, ls2)) => n1 == n2 && ls1 == ls2,
        (Expr::App(f1, a1), Expr::App(f2, a2)) => {
            exprs_syntactically_equal(f1, f2) && exprs_syntactically_equal(a1, a2)
        }
        (Expr::Lam(bi1, n1, t1, b1), Expr::Lam(bi2, n2, t2, b2)) => {
            bi1 == bi2
                && n1 == n2
                && exprs_syntactically_equal(t1, t2)
                && exprs_syntactically_equal(b1, b2)
        }
        (Expr::Pi(bi1, n1, t1, b1), Expr::Pi(bi2, n2, t2, b2)) => {
            bi1 == bi2
                && n1 == n2
                && exprs_syntactically_equal(t1, t2)
                && exprs_syntactically_equal(b1, b2)
        }
        (Expr::Let(n1, t1, v1, b1), Expr::Let(n2, t2, v2, b2)) => {
            n1 == n2
                && exprs_syntactically_equal(t1, t2)
                && exprs_syntactically_equal(v1, v2)
                && exprs_syntactically_equal(b1, b2)
        }
        (Expr::Lit(l1), Expr::Lit(l2)) => l1 == l2,
        (Expr::Proj(n1, i1, e1), Expr::Proj(n2, i2, e2)) => {
            n1 == n2 && i1 == i2 && exprs_syntactically_equal(e1, e2)
        }
        _ => false,
    }
}
/// Focus on the left-hand side of an equality at the current position.
pub fn conv_lhs(conv: &mut ConvState, _ctx: &mut MetaContext) -> TacticResult<()> {
    if let Some((_ty, lhs, _rhs)) = parse_eq_expr(&conv.focused) {
        conv.path.push(ConvPathStep::new(
            ConvDirection::Left,
            conv.focused.clone(),
            0,
        ));
        conv.focused = lhs;
        Ok(())
    } else {
        Err(TacticError::GoalMismatch(
            "conv lhs: focused expression is not an equality".to_string(),
        ))
    }
}
/// Focus on the right-hand side of an equality at the current position.
pub fn conv_rhs(conv: &mut ConvState, _ctx: &mut MetaContext) -> TacticResult<()> {
    if let Some((_ty, _lhs, rhs)) = parse_eq_expr(&conv.focused) {
        conv.path.push(ConvPathStep::new(
            ConvDirection::Right,
            conv.focused.clone(),
            1,
        ));
        conv.focused = rhs;
        Ok(())
    } else {
        Err(TacticError::GoalMismatch(
            "conv rhs: focused expression is not an equality".to_string(),
        ))
    }
}
/// Focus on the nth argument of an application at the current position.
pub fn conv_arg(n: usize, conv: &mut ConvState, _ctx: &mut MetaContext) -> TacticResult<()> {
    if conv.path.depth() >= MAX_CONV_DEPTH {
        return Err(TacticError::Failed(
            "conv: maximum navigation depth exceeded".to_string(),
        ));
    }
    let (_head, args) = collect_app_args(&conv.focused);
    if n >= args.len() {
        return Err(TacticError::Failed(format!(
            "conv arg: argument index {} out of range (have {} args)",
            n,
            args.len()
        )));
    }
    let focused = args[n].clone();
    conv.path.push(ConvPathStep::new(
        ConvDirection::Arg(n),
        conv.focused.clone(),
        n,
    ));
    conv.focused = focused;
    Ok(())
}
/// Apply extensionality at the current position.
///
/// If the focused expression is a lambda `fun x => body`, this focuses on `body`
/// with `x` available as a local variable.
pub fn conv_ext(conv: &mut ConvState, ctx: &mut MetaContext) -> TacticResult<()> {
    if conv.path.depth() >= MAX_CONV_DEPTH {
        return Err(TacticError::Failed(
            "conv: maximum navigation depth exceeded".to_string(),
        ));
    }
    if let Some((bi, name, ty, body)) = decompose_lambda(&conv.focused) {
        let _fvar_id = ctx.mk_local_decl(name.clone(), ty, bi);
        conv.ext_names.push(name);
        conv.in_ext = true;
        conv.path.push(ConvPathStep::new(
            ConvDirection::Ext,
            conv.focused.clone(),
            0,
        ));
        conv.focused = body;
        Ok(())
    } else {
        Err(TacticError::Failed(
            "conv ext: focused expression is not a lambda".to_string(),
        ))
    }
}
/// Navigate up one level (undo one navigation step).
pub fn conv_up(conv: &mut ConvState) -> TacticResult<()> {
    if let Some(step) = conv.path.pop() {
        conv.focused = step.context_expr;
        if step.direction == ConvDirection::Ext && conv.in_ext {
            conv.ext_names.pop();
            if conv.ext_names.is_empty() {
                conv.in_ext = false;
            }
        }
        Ok(())
    } else {
        Err(TacticError::Failed(
            "conv up: already at the root".to_string(),
        ))
    }
}
/// Rewrite the focused expression using a lemma.
///
/// The lemma should be an equality `a = b`. If the focused expression matches `a`,
/// it is replaced by `b`.
pub fn conv_rw(lemma: &Expr, conv: &mut ConvState, ctx: &mut MetaContext) -> TacticResult<()> {
    if conv.rewrite_count >= MAX_CONV_REWRITES {
        return Err(TacticError::Failed(
            "conv rw: maximum number of rewrites exceeded".to_string(),
        ));
    }
    let lemma_inst = ctx.instantiate_mvars(lemma);
    let (_lhs, rhs, proof, eq_ty) = extract_rewrite_info(&lemma_inst, &conv.focused, ctx)?;
    let before = conv.focused.clone();
    conv.focused = rhs.clone();
    conv.record_rewrite(before, rhs, proof, eq_ty);
    Ok(())
}
/// Extract the lhs, rhs, and proof from a rewrite lemma applied to the focused expression.
///
/// This looks up the type of the lemma (via environment or type inference), parses it
/// as an equality `@Eq α lhs rhs`, and replaces occurrences of `lhs` in `focused`
/// with `rhs`. Returns `(lhs, new_focused, proof)`.
pub(super) fn extract_rewrite_info(
    lemma: &Expr,
    focused: &Expr,
    ctx: &mut MetaContext,
) -> TacticResult<(Expr, Expr, Expr, Option<Expr>)> {
    let lemma_ty = infer_lemma_type(lemma, ctx)?;
    let alpha = parse_eq_alpha(&lemma_ty);
    let (lhs, rhs) = parse_equality_type(&lemma_ty).ok_or_else(|| {
        TacticError::Failed(format!(
            "conv rw: lemma type is not an equality: {:?}",
            lemma_ty
        ))
    })?;
    let new_focused = replace_in_expr(focused, &lhs, &rhs);
    if exprs_syntactically_equal(focused, &new_focused) {
        return Err(TacticError::Failed(
            "conv rw: lhs of lemma does not appear in focused expression".to_string(),
        ));
    }
    Ok((lhs, new_focused, lemma.clone(), alpha))
}
/// Infer the type of a lemma expression.
///
/// For constants, looks up the type in the environment. For other expressions,
/// uses `MetaInferType` to infer the type.
pub(super) fn infer_lemma_type(lemma: &Expr, ctx: &mut MetaContext) -> TacticResult<Expr> {
    if let Expr::Const(name, _levels) = lemma {
        if let Some(info) = ctx.find_const(name) {
            return Ok(info.ty().clone());
        }
    }
    if let Expr::FVar(fvar_id) = lemma {
        if let Some(ty) = ctx.get_fvar_type(*fvar_id) {
            return Ok(ty.clone());
        }
    }
    let mut infer = MetaInferType::new();
    infer
        .infer_type(lemma, ctx)
        .map_err(|e| TacticError::Failed(format!("conv rw: failed to infer lemma type: {}", e)))
}
/// Parse an equality type `@Eq α lhs rhs` or `lhs = rhs` into `(lhs, rhs)`.
pub(super) fn parse_equality_type(ty: &Expr) -> Option<(Expr, Expr)> {
    if let Expr::App(eq_a_lhs, rhs) = ty {
        if let Expr::App(eq_a, lhs) = eq_a_lhs.as_ref() {
            if let Expr::App(eq_const, _alpha) = eq_a.as_ref() {
                if is_eq_const(eq_const) {
                    return Some((*lhs.clone(), *rhs.clone()));
                }
            }
            if is_eq_const(eq_a) {
                return Some((*lhs.clone(), *rhs.clone()));
            }
        }
    }
    None
}
/// Extract the α type from `@Eq α lhs rhs`, if present.
pub(super) fn parse_eq_alpha(ty: &Expr) -> Option<Expr> {
    if let Expr::App(eq_a_lhs, _rhs) = ty {
        if let Expr::App(eq_a, _lhs) = eq_a_lhs.as_ref() {
            if let Expr::App(eq_const, alpha) = eq_a.as_ref() {
                if is_eq_const(eq_const) {
                    return Some(*alpha.clone());
                }
            }
        }
    }
    None
}
/// Check if an expression is the `Eq` constant.
pub(super) fn is_eq_const(expr: &Expr) -> bool {
    matches!(
        expr, Expr::Const(name, _) if { let s = name.to_string(); s == "Eq" || s == "eq"
        }
    )
}
/// Replace all occurrences of `from` with `to` in `expr`.
pub(super) fn replace_in_expr(expr: &Expr, from: &Expr, to: &Expr) -> Expr {
    if exprs_syntactically_equal(expr, from) {
        return to.clone();
    }
    match expr {
        Expr::App(f, a) => Expr::App(
            Box::new(replace_in_expr(f, from, to)),
            Box::new(replace_in_expr(a, from, to)),
        ),
        Expr::Lam(bi, name, ty, body) => Expr::Lam(
            *bi,
            name.clone(),
            Box::new(replace_in_expr(ty, from, to)),
            Box::new(replace_in_expr(body, from, to)),
        ),
        Expr::Pi(bi, name, ty, body) => Expr::Pi(
            *bi,
            name.clone(),
            Box::new(replace_in_expr(ty, from, to)),
            Box::new(replace_in_expr(body, from, to)),
        ),
        Expr::Let(name, ty, val, body) => Expr::Let(
            name.clone(),
            Box::new(replace_in_expr(ty, from, to)),
            Box::new(replace_in_expr(val, from, to)),
            Box::new(replace_in_expr(body, from, to)),
        ),
        Expr::Proj(name, i, e) => {
            Expr::Proj(name.clone(), *i, Box::new(replace_in_expr(e, from, to)))
        }
        _ => expr.clone(),
    }
}
/// Simplify the focused expression using simp-like simplification.
pub fn conv_simp(
    config: &ConvSimpConfig,
    conv: &mut ConvState,
    ctx: &mut MetaContext,
) -> TacticResult<()> {
    if conv.rewrite_count >= MAX_CONV_REWRITES {
        return Err(TacticError::Failed(
            "conv simp: maximum number of rewrites exceeded".to_string(),
        ));
    }
    let simplified = simplify_expr(&conv.focused, config, ctx)?;
    let before = conv.focused.clone();
    if exprs_syntactically_equal(&before, &simplified) {
        return Err(TacticError::Failed(
            "conv simp: expression did not simplify".to_string(),
        ));
    }
    let proof = Expr::Const(Name::str("_conv_simp_proof"), vec![]);
    conv.focused = simplified.clone();
    conv.record_rewrite(before, simplified, proof, None);
    Ok(())
}
/// Simplify an expression using the simp engine.
///
/// Calls the simp rewriting engine on `expr` with the given configuration.
/// Returns the simplified expression, or an error if no simplification occurs.
pub(super) fn simplify_expr(
    expr: &Expr,
    config: &ConvSimpConfig,
    ctx: &mut MetaContext,
) -> TacticResult<Expr> {
    let instantiated = ctx.instantiate_mvars(expr);
    let theorems = SimpTheorems::new();
    let simp_config = SimpConfig {
        max_steps: if config.max_steps > 0 {
            config.max_steps as u32
        } else {
            1000
        },
        use_default_lemmas: config.use_defaults,
        ..SimpConfig::default()
    };
    match simp_expr(&instantiated, &theorems, &simp_config, ctx) {
        SimpResult::Simplified { new_expr, .. } => Ok(new_expr),
        SimpResult::Proved(_) => Ok(Expr::Const(Name::str("True"), vec![])),
        SimpResult::Unchanged => Ok(instantiated),
    }
}
/// Apply ring normalization at the current position.
pub fn conv_ring(conv: &mut ConvState, ctx: &mut MetaContext) -> TacticResult<()> {
    if conv.rewrite_count >= MAX_CONV_REWRITES {
        return Err(TacticError::Failed(
            "conv ring: maximum number of rewrites exceeded".to_string(),
        ));
    }
    let before = conv.focused.clone();
    let normalized = normalize_ring_expr(&conv.focused, ctx)?;
    if exprs_syntactically_equal(&before, &normalized) {
        return Err(TacticError::Failed(
            "conv ring: expression did not change".to_string(),
        ));
    }
    let proof = Expr::Const(Name::str("_conv_ring_proof"), vec![]);
    conv.focused = normalized.clone();
    conv.record_rewrite(before, normalized, proof, None);
    Ok(())
}
/// Normalize an expression using ring/polynomial laws.
///
/// Converts to a polynomial normal form, removes zero terms, and converts
/// back to an expression. This puts arithmetic expressions in a canonical form.
pub(super) fn normalize_ring_expr(expr: &Expr, ctx: &MetaContext) -> TacticResult<Expr> {
    let instantiated = ctx.instantiate_mvars(expr);
    match expr_to_polynomial(&instantiated, ctx) {
        Ok(poly) => {
            let normalized = simplify_add_mul_pow(&poly);
            Ok(polynomial_to_expr(&normalized))
        }
        Err(_) => Ok(instantiated),
    }
}
/// Convert a normalized `Polynomial` back to a kernel `Expr`.
///
/// Builds a sum of monomials: each monomial is a product of variables raised
/// to their exponents, scaled by the rational coefficient.
pub(super) fn polynomial_to_expr(poly: &crate::tactic::ring::Polynomial) -> Expr {
    if poly.is_zero() {
        return Expr::Const(Name::str("zero"), vec![]);
    }
    let term_exprs: Vec<Expr> = poly
        .terms
        .iter()
        .map(|(mono, (num, den))| monomial_term_to_expr(mono, *num, *den))
        .collect();
    term_exprs
        .into_iter()
        .reduce(|acc, t| {
            Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Nat.add"), vec![])),
                    Box::new(acc),
                )),
                Box::new(t),
            )
        })
        .unwrap_or_else(|| Expr::Const(Name::str("zero"), vec![]))
}
/// Convert a single monomial term `coeff * x1^e1 * x2^e2 * ...` to an `Expr`.
pub(super) fn monomial_term_to_expr(
    mono: &crate::tactic::ring::Monomial,
    num: i64,
    den: u32,
) -> Expr {
    let coeff_expr = if den == 1 {
        if num >= 0 {
            Expr::Lit(Literal::Nat(num as u64))
        } else {
            Expr::App(
                Box::new(Expr::Const(Name::str("Neg.neg"), vec![])),
                Box::new(Expr::Lit(Literal::Nat(num.unsigned_abs()))),
            )
        }
    } else {
        Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("HDiv.hDiv"), vec![])),
                Box::new(Expr::Lit(Literal::Nat(num.unsigned_abs()))),
            )),
            Box::new(Expr::Lit(Literal::Nat(den as u64))),
        )
    };
    if mono.exponents.is_empty() {
        return coeff_expr;
    }
    let var_expr = mono
        .exponents
        .iter()
        .map(|(var_name, exp)| {
            let var = Expr::Const(var_name.clone(), vec![]);
            if *exp == 1 {
                var
            } else {
                Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("HPow.hPow"), vec![])),
                        Box::new(var),
                    )),
                    Box::new(Expr::Lit(Literal::Nat(*exp as u64))),
                )
            }
        })
        .reduce(|acc, v| {
            Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Nat.mul"), vec![])),
                    Box::new(acc),
                )),
                Box::new(v),
            )
        })
        .expect("exponents is non-empty; checked above before building var_expr");
    if num == 1 && den == 1 {
        var_expr
    } else {
        Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Nat.mul"), vec![])),
                Box::new(coeff_expr),
            )),
            Box::new(var_expr),
        )
    }
}
/// Apply norm_num at the current position.
pub fn conv_norm_num(conv: &mut ConvState, ctx: &mut MetaContext) -> TacticResult<()> {
    if conv.rewrite_count >= MAX_CONV_REWRITES {
        return Err(TacticError::Failed(
            "conv norm_num: maximum number of rewrites exceeded".to_string(),
        ));
    }
    let before = conv.focused.clone();
    let normalized = normalize_numeric_expr(&conv.focused, ctx)?;
    if exprs_syntactically_equal(&before, &normalized) {
        return Err(TacticError::Failed(
            "conv norm_num: expression did not change".to_string(),
        ));
    }
    let proof = Expr::Const(Name::str("_conv_norm_num_proof"), vec![]);
    conv.focused = normalized.clone();
    conv.record_rewrite(before, normalized, proof, None);
    Ok(())
}
/// Normalize numeric expressions (simplified).
pub(super) fn normalize_numeric_expr(expr: &Expr, ctx: &MetaContext) -> TacticResult<Expr> {
    Ok(ctx.instantiate_mvars(expr))
}
/// Exit conv mode and reconstruct the proof.
///
/// Builds a proof term that justifies the transformation from the original
/// expression to the modified one, using congruence lemmas for each navigation
/// step and the local proofs for each rewrite.
pub fn exit_conv(
    conv: &ConvState,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<ConvResult> {
    if conv.local_proofs.is_empty() {
        return Ok(ConvResult {
            new_expr: conv.original_goal.clone(),
            proof: mk_eq_refl(
                &conv.eq_type.clone().unwrap_or(Expr::Sort(Level::Zero)),
                &conv.focused,
            ),
            num_rewrites: 0,
            changed: false,
            stats: ConvStats::default(),
        });
    }
    let local_proof = build_combined_local_proof(&conv.local_proofs)?;
    let full_proof =
        reconstruct_proof_from_path(&conv.path, &conv.focused, &local_proof, &conv.eq_type, ctx)?;
    let new_goal = rebuild_expr_from_path(&conv.path, &conv.focused)?;
    let _goal = state.current_goal()?;
    let proof_term = full_proof.clone();
    state.close_goal(proof_term, ctx)?;
    let stats = ConvStats {
        nav_steps: conv.path.depth(),
        max_depth_reached: conv.path.depth(),
        simp_calls: 0,
        ring_calls: 0,
        norm_num_calls: 0,
        ext_applications: conv.ext_names.len(),
        failed_rewrites: 0,
    };
    Ok(ConvResult {
        new_expr: new_goal,
        proof: full_proof,
        num_rewrites: conv.rewrite_count,
        changed: true,
        stats,
    })
}
/// Build a combined proof from multiple local proof steps.
pub(super) fn build_combined_local_proof(proofs: &[ConvLocalProof]) -> TacticResult<Expr> {
    if proofs.is_empty() {
        return Err(TacticError::Internal(
            "no local proofs to combine".to_string(),
        ));
    }
    if proofs.len() == 1 {
        return Ok(proofs[0].proof.clone());
    }
    let mut combined = proofs[0].proof.clone();
    let ty = proofs[0].ty.clone().unwrap_or(Expr::Sort(Level::Zero));
    for i in 1..proofs.len() {
        combined = mk_eq_trans(
            &ty,
            &proofs[0].before,
            &proofs[i - 1].after,
            &proofs[i].after,
            combined,
            proofs[i].proof.clone(),
        );
    }
    Ok(combined)
}
/// Reconstruct the full proof from the path and the local proof.
///
/// For each navigation step, wraps the proof in the appropriate congruence lemma.
pub(super) fn reconstruct_proof_from_path(
    path: &ConvPath,
    _focused_after: &Expr,
    local_proof: &Expr,
    eq_type: &Option<Expr>,
    _ctx: &MetaContext,
) -> TacticResult<Expr> {
    let ty = eq_type.clone().unwrap_or(Expr::Sort(Level::Zero));
    let mut proof = local_proof.clone();
    for step in path.steps().iter().rev() {
        proof = wrap_proof_with_congr(&step.direction, &step.context_expr, proof, &ty)?;
    }
    match path.entry_side() {
        ConvEntrySide::Lhs => {}
        ConvEntrySide::Rhs => {}
        ConvEntrySide::Whole => {}
    }
    Ok(proof)
}
/// Wrap a proof in a congruence lemma for one navigation step.
pub(super) fn wrap_proof_with_congr(
    direction: &ConvDirection,
    context_expr: &Expr,
    inner_proof: Expr,
    ty: &Expr,
) -> TacticResult<Expr> {
    match direction {
        ConvDirection::Left => {
            if let Expr::App(_f, a) = context_expr {
                let alpha = ty.clone();
                let beta = ty.clone();
                let f_old = Expr::Const(Name::str("_f_old"), vec![]);
                let f_new = Expr::Const(Name::str("_f_new"), vec![]);
                Ok(mk_congr_fun(&alpha, &beta, &f_old, &f_new, inner_proof, a))
            } else {
                Err(TacticError::Internal(
                    "conv reconstruct: expected application for Left step".to_string(),
                ))
            }
        }
        ConvDirection::Right => {
            if let Expr::App(f, _a) = context_expr {
                let alpha = ty.clone();
                let beta = ty.clone();
                let a_old = Expr::Const(Name::str("_a_old"), vec![]);
                let a_new = Expr::Const(Name::str("_a_new"), vec![]);
                Ok(mk_congr_arg(&alpha, &beta, f, &a_old, &a_new, inner_proof))
            } else {
                Err(TacticError::Internal(
                    "conv reconstruct: expected application for Right step".to_string(),
                ))
            }
        }
        ConvDirection::Arg(n) => {
            let (_head, args) = collect_app_args(context_expr);
            if *n >= args.len() {
                return Err(TacticError::Internal(
                    "conv reconstruct: arg index out of range".to_string(),
                ));
            }
            let alpha = ty.clone();
            let beta = ty.clone();
            let partial_app = {
                let mut result = _head.clone();
                for (i, arg) in args.iter().enumerate() {
                    if i == *n {
                        break;
                    }
                    result = Expr::App(Box::new(result), Box::new(arg.clone()));
                }
                result
            };
            let a_old = args[*n].clone();
            let a_new = Expr::Const(Name::str("_arg_new"), vec![]);
            Ok(mk_congr_arg(
                &alpha,
                &beta,
                &partial_app,
                &a_old,
                &a_new,
                inner_proof,
            ))
        }
        ConvDirection::Fun => {
            let (_head, args) = collect_app_args(context_expr);
            if args.is_empty() {
                return Ok(inner_proof);
            }
            let alpha = ty.clone();
            let beta = ty.clone();
            let f_old = Expr::Const(Name::str("_fun_old"), vec![]);
            let f_new = Expr::Const(Name::str("_fun_new"), vec![]);
            let first_arg = args[0].clone();
            Ok(mk_congr_fun(
                &alpha,
                &beta,
                &f_old,
                &f_new,
                inner_proof,
                &first_arg,
            ))
        }
        ConvDirection::Ext => {
            if let Expr::Lam(_bi, _name, lam_ty, _body) = context_expr {
                let alpha = *lam_ty.clone();
                let beta = ty.clone();
                let f_old = context_expr.clone();
                let f_new = Expr::Const(Name::str("_ext_new"), vec![]);
                Ok(mk_funext(&alpha, &beta, &f_old, &f_new, inner_proof))
            } else {
                Err(TacticError::Internal(
                    "conv reconstruct: expected lambda for Ext step".to_string(),
                ))
            }
        }
    }
}
/// Rebuild the expression by substituting the modified focused expression
/// back through the path.
pub(super) fn rebuild_expr_from_path(path: &ConvPath, new_focused: &Expr) -> TacticResult<Expr> {
    let steps = path.steps();
    if steps.is_empty() {
        return Ok(new_focused.clone());
    }
    let mut current = new_focused.clone();
    for step in steps.iter().rev() {
        current = rebuild_one_step(&step.direction, &step.context_expr, current)?;
    }
    Ok(current)
}
/// Rebuild one step of the expression tree.
pub(super) fn rebuild_one_step(
    direction: &ConvDirection,
    context_expr: &Expr,
    inner: Expr,
) -> TacticResult<Expr> {
    match direction {
        ConvDirection::Left => {
            if let Expr::App(_f, a) = context_expr {
                Ok(Expr::App(Box::new(inner), a.clone()))
            } else {
                Err(TacticError::Internal(
                    "rebuild: expected App for Left".into(),
                ))
            }
        }
        ConvDirection::Right => {
            if let Expr::App(f, _a) = context_expr {
                Ok(Expr::App(f.clone(), Box::new(inner)))
            } else {
                Err(TacticError::Internal(
                    "rebuild: expected App for Right".into(),
                ))
            }
        }
        ConvDirection::Arg(n) => {
            let (head, mut args) = collect_app_args(context_expr);
            if *n >= args.len() {
                return Err(TacticError::Internal("rebuild: arg out of range".into()));
            }
            args[*n] = inner;
            Ok(mk_app(head, args))
        }
        ConvDirection::Fun => {
            let (_head, args) = collect_app_args(context_expr);
            Ok(mk_app(inner, args))
        }
        ConvDirection::Ext => {
            if let Expr::Lam(bi, name, ty, _body) = context_expr {
                Ok(Expr::Lam(*bi, name.clone(), ty.clone(), Box::new(inner)))
            } else {
                Err(TacticError::Internal(
                    "rebuild: expected Lam for Ext".into(),
                ))
            }
        }
    }
}
/// Run a complete conv session: enter, perform rewrites, exit.
///
/// This is a higher-level API that combines enter_conv, a sequence of operations,
/// and exit_conv.
#[allow(clippy::too_many_arguments)]
pub fn run_conv_session(
    target: ConvTarget,
    operations: &[ConvOperation],
    config: &ConvConfig,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<ConvResult> {
    let mut conv = enter_conv(target, state, ctx)?;
    for (i, op) in operations.iter().enumerate() {
        if i >= config.max_rewrites {
            return Err(TacticError::Failed(
                "conv: maximum operations exceeded".to_string(),
            ));
        }
        match op {
            ConvOperation::Lhs => conv_lhs(&mut conv, ctx)?,
            ConvOperation::Rhs => conv_rhs(&mut conv, ctx)?,
            ConvOperation::Arg(n) => conv_arg(*n, &mut conv, ctx)?,
            ConvOperation::Ext => conv_ext(&mut conv, ctx)?,
            ConvOperation::Up => conv_up(&mut conv)?,
            ConvOperation::Rw(lemma) => conv_rw(lemma, &mut conv, ctx)?,
            ConvOperation::Simp(simp_config) => conv_simp(simp_config, &mut conv, ctx)?,
            ConvOperation::Ring => {
                if !config.allow_ring {
                    return Err(TacticError::Failed(
                        "conv: ring is not allowed in this configuration".to_string(),
                    ));
                }
                conv_ring(&mut conv, ctx)?;
            }
            ConvOperation::NormNum => {
                if !config.allow_norm_num {
                    return Err(TacticError::Failed(
                        "conv: norm_num is not allowed in this configuration".to_string(),
                    ));
                }
                conv_norm_num(&mut conv, ctx)?;
            }
        }
    }
    exit_conv(&conv, state, ctx)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::basic::MetavarKind;
    use crate::tactic::conv_mode::*;
    use oxilean_kernel::Environment;
    fn mk_test_ctx() -> MetaContext {
        MetaContext::new(Environment::new())
    }
    fn mk_test_state(ctx: &mut MetaContext) -> (TacticState, MVarId) {
        let ty = Expr::Sort(Level::Zero);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(ty, MetavarKind::Natural);
        let state = TacticState::single(mvar_id);
        (state, mvar_id)
    }
    fn mk_nat_const() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn mk_eq_goal(lhs: Expr, rhs: Expr) -> Expr {
        let eq_const = Expr::Const(Name::str("Eq"), vec![Level::Zero]);
        let nat = mk_nat_const();
        Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::App(Box::new(eq_const), Box::new(nat))),
                Box::new(lhs),
            )),
            Box::new(rhs),
        )
    }
    fn mk_app_expr(f: Expr, a: Expr) -> Expr {
        Expr::App(Box::new(f), Box::new(a))
    }
    fn mk_lam(name: &str, ty: Expr, body: Expr) -> Expr {
        Expr::Lam(
            BinderInfo::Default,
            Name::str(name),
            Box::new(ty),
            Box::new(body),
        )
    }
    #[test]
    fn test_conv_target_display() {
        assert_eq!(format!("{}", ConvTarget::Lhs), "lhs");
        assert_eq!(format!("{}", ConvTarget::Rhs), "rhs");
        assert_eq!(format!("{}", ConvTarget::Arg(2)), "arg 2");
        assert_eq!(format!("{}", ConvTarget::Fun), "fun");
    }
    #[test]
    fn test_conv_direction_display() {
        assert_eq!(format!("{}", ConvDirection::Left), "left");
        assert_eq!(format!("{}", ConvDirection::Right), "right");
        assert_eq!(format!("{}", ConvDirection::Arg(3)), "arg 3");
        assert_eq!(format!("{}", ConvDirection::Fun), "fun");
        assert_eq!(format!("{}", ConvDirection::Ext), "ext");
    }
    #[test]
    fn test_conv_target_enter_display() {
        let target = ConvTarget::Enter(vec![ConvDirection::Left, ConvDirection::Right]);
        assert_eq!(format!("{}", target), "enter [left, right]");
    }
    #[test]
    fn test_conv_path_empty() {
        let path = ConvPath::new(Expr::Sort(Level::Zero), ConvEntrySide::Lhs);
        assert!(path.is_empty());
        assert_eq!(path.depth(), 0);
    }
    #[test]
    fn test_conv_path_push_pop() {
        let mut path = ConvPath::new(Expr::Sort(Level::Zero), ConvEntrySide::Lhs);
        let step = ConvPathStep::new(ConvDirection::Left, Expr::Sort(Level::Zero), 0);
        path.push(step);
        assert_eq!(path.depth(), 1);
        assert!(!path.is_empty());
        let popped = path.pop();
        assert!(popped.is_some());
        assert!(path.is_empty());
    }
    #[test]
    fn test_conv_path_multiple_steps() {
        let mut path = ConvPath::new(Expr::Sort(Level::Zero), ConvEntrySide::Rhs);
        path.push(ConvPathStep::new(
            ConvDirection::Left,
            Expr::Sort(Level::Zero),
            0,
        ));
        path.push(ConvPathStep::new(
            ConvDirection::Right,
            Expr::Sort(Level::Zero),
            1,
        ));
        path.push(ConvPathStep::new(
            ConvDirection::Ext,
            Expr::Sort(Level::Zero),
            0,
        ));
        assert_eq!(path.depth(), 3);
        assert_eq!(path.steps().len(), 3);
    }
    #[test]
    fn test_conv_state_creation() {
        let focused = Expr::Const(Name::str("x"), vec![]);
        let goal = Expr::Sort(Level::Zero);
        let mvar = MVarId(0);
        let state = ConvState::new(focused.clone(), goal, mvar, ConvEntrySide::Lhs);
        assert!(state.is_at_root());
        assert_eq!(state.depth(), 0);
        assert_eq!(state.rewrite_count, 0);
    }
    #[test]
    fn test_conv_state_record_rewrite() {
        let focused = Expr::Const(Name::str("x"), vec![]);
        let goal = Expr::Sort(Level::Zero);
        let mvar = MVarId(0);
        let mut state = ConvState::new(focused.clone(), goal, mvar, ConvEntrySide::Lhs);
        let before = Expr::Const(Name::str("a"), vec![]);
        let after = Expr::Const(Name::str("b"), vec![]);
        let proof = Expr::Const(Name::str("hab"), vec![]);
        state.record_rewrite(before, after, proof, None);
        assert_eq!(state.rewrite_count, 1);
        assert_eq!(state.local_proofs.len(), 1);
    }
    #[test]
    fn test_conv_config_default() {
        let config = ConvConfig::default();
        assert!(config.allow_simp);
        assert!(config.allow_ring);
        assert!(config.allow_norm_num);
        assert!(config.auto_close);
        assert_eq!(config.max_depth, DEFAULT_CONV_MAX_DEPTH);
    }
    #[test]
    fn test_conv_config_rewrite_only() {
        let config = ConvConfig::rewrite_only();
        assert!(!config.allow_simp);
        assert!(!config.allow_ring);
        assert!(!config.allow_norm_num);
    }
    #[test]
    fn test_conv_config_depth_check() {
        let config = ConvConfig::default().with_max_depth(10);
        assert!(config.is_depth_ok(5));
        assert!(config.is_depth_ok(10));
        assert!(!config.is_depth_ok(11));
    }
    #[test]
    fn test_collect_app_args_no_args() {
        let expr = Expr::Const(Name::str("f"), vec![]);
        let (head, args) = collect_app_args(&expr);
        assert!(matches!(head, Expr::Const(_, _)));
        assert!(args.is_empty());
    }
    #[test]
    fn test_collect_app_args_single() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Const(Name::str("a"), vec![]);
        let expr = mk_app_expr(f.clone(), a.clone());
        let (head, args) = collect_app_args(&expr);
        assert!(matches!(head, Expr::Const(_, _)));
        assert_eq!(args.len(), 1);
    }
    #[test]
    fn test_collect_app_args_multiple() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let expr = mk_app_expr(mk_app_expr(f.clone(), a.clone()), b.clone());
        let (_head, args) = collect_app_args(&expr);
        assert_eq!(args.len(), 2);
    }
    #[test]
    fn test_mk_app_roundtrip() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let expr = mk_app(f.clone(), vec![a.clone(), b.clone()]);
        let (head, args) = collect_app_args(&expr);
        assert_eq!(args.len(), 2);
        assert!(exprs_syntactically_equal(&head, &f));
    }
    #[test]
    fn test_parse_eq_expr() {
        let lhs = Expr::Const(Name::str("a"), vec![]);
        let rhs = Expr::Const(Name::str("b"), vec![]);
        let eq_goal = mk_eq_goal(lhs.clone(), rhs.clone());
        let result = parse_eq_expr(&eq_goal);
        assert!(result.is_some());
        let (_ty, parsed_lhs, parsed_rhs) = result.expect("result should be valid");
        assert!(exprs_syntactically_equal(&parsed_lhs, &lhs));
        assert!(exprs_syntactically_equal(&parsed_rhs, &rhs));
    }
    #[test]
    fn test_parse_eq_expr_non_eq() {
        let expr = Expr::Const(Name::str("P"), vec![]);
        assert!(parse_eq_expr(&expr).is_none());
    }
    #[test]
    fn test_exprs_syntactically_equal_same() {
        let a = Expr::Const(Name::str("x"), vec![]);
        assert!(exprs_syntactically_equal(&a, &a));
    }
    #[test]
    fn test_exprs_syntactically_equal_different() {
        let a = Expr::Const(Name::str("x"), vec![]);
        let b = Expr::Const(Name::str("y"), vec![]);
        assert!(!exprs_syntactically_equal(&a, &b));
    }
    #[test]
    fn test_exprs_syntactically_equal_app() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Const(Name::str("a"), vec![]);
        let e1 = mk_app_expr(f.clone(), a.clone());
        let e2 = mk_app_expr(f.clone(), a.clone());
        assert!(exprs_syntactically_equal(&e1, &e2));
    }
    #[test]
    fn test_exprs_syntactically_equal_lam() {
        let ty = Expr::Const(Name::str("Nat"), vec![]);
        let body = Expr::BVar(0);
        let e1 = mk_lam("x", ty.clone(), body.clone());
        let e2 = mk_lam("x", ty.clone(), body.clone());
        assert!(exprs_syntactically_equal(&e1, &e2));
    }
    #[test]
    fn test_exprs_syntactically_equal_bvar() {
        let a = Expr::BVar(0);
        let b = Expr::BVar(0);
        assert!(exprs_syntactically_equal(&a, &b));
        let c = Expr::BVar(1);
        assert!(!exprs_syntactically_equal(&a, &c));
    }
    #[test]
    fn test_navigate_left() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Const(Name::str("a"), vec![]);
        let expr = mk_app_expr(f.clone(), a.clone());
        let (result, pos) =
            navigate_one_step(&ConvDirection::Left, &expr).expect("value should be present");
        assert!(exprs_syntactically_equal(&result, &f));
        assert_eq!(pos, 0);
    }
    #[test]
    fn test_navigate_right() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Const(Name::str("a"), vec![]);
        let expr = mk_app_expr(f.clone(), a.clone());
        let (result, pos) =
            navigate_one_step(&ConvDirection::Right, &expr).expect("value should be present");
        assert!(exprs_syntactically_equal(&result, &a));
        assert_eq!(pos, 1);
    }
    #[test]
    fn test_navigate_ext() {
        let ty = mk_nat_const();
        let body = Expr::BVar(0);
        let lam = mk_lam("x", ty, body.clone());
        let (result, _) =
            navigate_one_step(&ConvDirection::Ext, &lam).expect("value should be present");
        assert!(exprs_syntactically_equal(&result, &body));
    }
    #[test]
    fn test_navigate_left_non_app_fails() {
        let expr = Expr::Const(Name::str("x"), vec![]);
        let result = navigate_one_step(&ConvDirection::Left, &expr);
        assert!(result.is_err());
    }
    #[test]
    fn test_navigate_ext_non_lam_fails() {
        let expr = Expr::Const(Name::str("x"), vec![]);
        let result = navigate_one_step(&ConvDirection::Ext, &expr);
        assert!(result.is_err());
    }
    #[test]
    fn test_navigate_arg_out_of_range() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Const(Name::str("a"), vec![]);
        let expr = mk_app_expr(f, a);
        let result = navigate_one_step(&ConvDirection::Arg(5), &expr);
        assert!(result.is_err());
    }
    #[test]
    fn test_find_pattern_at_root() {
        let expr = Expr::Const(Name::str("x"), vec![]);
        let result = find_pattern_in_expr(&expr, &expr);
        assert!(result.is_some());
        assert!(result.expect("result should be valid").is_empty());
    }
    #[test]
    fn test_find_pattern_in_app() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Const(Name::str("target"), vec![]);
        let expr = mk_app_expr(f, a.clone());
        let result = find_pattern_in_expr(&a, &expr);
        assert!(result.is_some());
        assert_eq!(result.expect("result should be valid").len(), 1);
    }
    #[test]
    fn test_find_pattern_nested() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let g = Expr::Const(Name::str("g"), vec![]);
        let target = Expr::Const(Name::str("target"), vec![]);
        let inner = mk_app_expr(g, target.clone());
        let expr = mk_app_expr(f, inner);
        let result = find_pattern_in_expr(&target, &expr);
        assert!(result.is_some());
        assert_eq!(result.expect("result should be valid").len(), 2);
    }
    #[test]
    fn test_find_pattern_not_found() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Const(Name::str("a"), vec![]);
        let target = Expr::Const(Name::str("not_there"), vec![]);
        let expr = mk_app_expr(f, a);
        let result = find_pattern_in_expr(&target, &expr);
        assert!(result.is_none());
    }
    #[test]
    fn test_rebuild_empty_path() {
        let new_focused = Expr::Const(Name::str("result"), vec![]);
        let path = ConvPath::new(Expr::Sort(Level::Zero), ConvEntrySide::Lhs);
        let result = rebuild_expr_from_path(&path, &new_focused).expect("result should be present");
        assert!(exprs_syntactically_equal(&result, &new_focused));
    }
    #[test]
    fn test_rebuild_right_step() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Const(Name::str("a"), vec![]);
        let original = mk_app_expr(f.clone(), a.clone());
        let mut path = ConvPath::new(original.clone(), ConvEntrySide::Whole);
        path.push(ConvPathStep::new(ConvDirection::Right, original.clone(), 1));
        let new_focused = Expr::Const(Name::str("b"), vec![]);
        let rebuilt =
            rebuild_expr_from_path(&path, &new_focused).expect("rebuilt should be present");
        let expected = mk_app_expr(f, new_focused);
        assert!(exprs_syntactically_equal(&rebuilt, &expected));
    }
    #[test]
    fn test_rebuild_left_step() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Const(Name::str("a"), vec![]);
        let original = mk_app_expr(f.clone(), a.clone());
        let mut path = ConvPath::new(original.clone(), ConvEntrySide::Whole);
        path.push(ConvPathStep::new(ConvDirection::Left, original.clone(), 0));
        let new_focused = Expr::Const(Name::str("g"), vec![]);
        let rebuilt =
            rebuild_expr_from_path(&path, &new_focused).expect("rebuilt should be present");
        let expected = mk_app_expr(new_focused, a);
        assert!(exprs_syntactically_equal(&rebuilt, &expected));
    }
    #[test]
    fn test_rebuild_ext_step() {
        let ty = mk_nat_const();
        let body = Expr::BVar(0);
        let original = mk_lam("x", ty.clone(), body.clone());
        let mut path = ConvPath::new(original.clone(), ConvEntrySide::Whole);
        path.push(ConvPathStep::new(ConvDirection::Ext, original.clone(), 0));
        let new_body = Expr::Const(Name::str("result"), vec![]);
        let rebuilt = rebuild_expr_from_path(&path, &new_body).expect("rebuilt should be present");
        let expected = mk_lam("x", ty, new_body);
        assert!(exprs_syntactically_equal(&rebuilt, &expected));
    }
    #[test]
    fn test_conv_stats_default() {
        let stats = ConvStats::default();
        assert_eq!(stats.nav_steps, 0);
        assert_eq!(stats.max_depth_reached, 0);
        assert_eq!(stats.simp_calls, 0);
        assert_eq!(stats.ring_calls, 0);
        assert_eq!(stats.norm_num_calls, 0);
    }
    #[test]
    fn test_conv_result_no_change() {
        let result = ConvResult {
            new_expr: Expr::Sort(Level::Zero),
            proof: Expr::Const(Name::str("rfl"), vec![]),
            num_rewrites: 0,
            changed: false,
            stats: ConvStats::default(),
        };
        assert!(!result.changed);
        assert_eq!(result.num_rewrites, 0);
    }
    #[test]
    fn test_conv_entry_side_eq() {
        assert_eq!(ConvEntrySide::Lhs, ConvEntrySide::Lhs);
        assert_ne!(ConvEntrySide::Lhs, ConvEntrySide::Rhs);
        assert_ne!(ConvEntrySide::Rhs, ConvEntrySide::Whole);
    }
    #[test]
    fn test_conv_simp_config_default() {
        let config = ConvSimpConfig::default();
        assert!(config.lemmas.is_empty());
        assert!(!config.use_defaults);
        assert_eq!(config.max_steps, 0);
    }
    #[test]
    fn test_conv_operation_variants() {
        let ops = [
            ConvOperation::Lhs,
            ConvOperation::Rhs,
            ConvOperation::Arg(0),
            ConvOperation::Ext,
            ConvOperation::Up,
            ConvOperation::Ring,
            ConvOperation::NormNum,
        ];
        assert_eq!(ops.len(), 7);
    }
    #[test]
    fn test_decompose_pi() {
        let dom = mk_nat_const();
        let cod = Expr::Sort(Level::Zero);
        let pi = Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(dom.clone()),
            Box::new(cod.clone()),
        );
        let result = decompose_pi(&pi);
        assert!(result.is_some());
        let (bi, name, d, _c) = result.expect("result should be valid");
        assert_eq!(bi, BinderInfo::Default);
        assert_eq!(name, Name::str("x"));
        assert!(exprs_syntactically_equal(&d, &dom));
    }
    #[test]
    fn test_decompose_pi_non_pi() {
        let expr = Expr::Const(Name::str("Nat"), vec![]);
        assert!(decompose_pi(&expr).is_none());
    }
    #[test]
    fn test_decompose_lambda() {
        let ty = mk_nat_const();
        let body = Expr::BVar(0);
        let lam = mk_lam("x", ty.clone(), body.clone());
        let result = decompose_lambda(&lam);
        assert!(result.is_some());
        let (bi, name, _t, _b) = result.expect("result should be valid");
        assert_eq!(bi, BinderInfo::Default);
        assert_eq!(name, Name::str("x"));
    }
    #[test]
    fn test_decompose_lambda_non_lam() {
        let expr = Expr::Const(Name::str("f"), vec![]);
        assert!(decompose_lambda(&expr).is_none());
    }
    #[test]
    fn test_conv_state_full_lifecycle() {
        let focused = Expr::Const(Name::str("x"), vec![]);
        let goal = Expr::Sort(Level::Zero);
        let mvar = MVarId(0);
        let mut state = ConvState::new(focused.clone(), goal, mvar, ConvEntrySide::Lhs);
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let c = Expr::Const(Name::str("c"), vec![]);
        let proof_ab = Expr::Const(Name::str("hab"), vec![]);
        let proof_bc = Expr::Const(Name::str("hbc"), vec![]);
        state.record_rewrite(a.clone(), b.clone(), proof_ab, None);
        state.record_rewrite(b.clone(), c.clone(), proof_bc, None);
        assert_eq!(state.rewrite_count, 2);
        assert_eq!(state.local_proofs.len(), 2);
    }
    #[test]
    fn test_build_combined_local_proof_single() {
        let proof = ConvLocalProof {
            before: Expr::Const(Name::str("a"), vec![]),
            after: Expr::Const(Name::str("b"), vec![]),
            proof: Expr::Const(Name::str("hab"), vec![]),
            path_depth: 0,
            ty: None,
        };
        let result = build_combined_local_proof(&[proof]).expect("result should be present");
        assert!(matches!(result, Expr::Const(_, _)));
    }
    #[test]
    fn test_build_combined_local_proof_multiple() {
        let proof1 = ConvLocalProof {
            before: Expr::Const(Name::str("a"), vec![]),
            after: Expr::Const(Name::str("b"), vec![]),
            proof: Expr::Const(Name::str("hab"), vec![]),
            path_depth: 0,
            ty: None,
        };
        let proof2 = ConvLocalProof {
            before: Expr::Const(Name::str("b"), vec![]),
            after: Expr::Const(Name::str("c"), vec![]),
            proof: Expr::Const(Name::str("hbc"), vec![]),
            path_depth: 0,
            ty: None,
        };
        let result =
            build_combined_local_proof(&[proof1, proof2]).expect("result should be present");
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_build_combined_local_proof_empty() {
        let result = build_combined_local_proof(&[]);
        assert!(result.is_err());
    }
    #[test]
    fn test_get_head_const() {
        let c = Expr::Const(Name::str("Nat.add"), vec![]);
        assert_eq!(get_head_const(&c), Some(Name::str("Nat.add")));
        let app = mk_app_expr(c.clone(), Expr::Const(Name::str("x"), vec![]));
        assert_eq!(get_head_const(&app), Some(Name::str("Nat.add")));
        let bvar = Expr::BVar(0);
        assert_eq!(get_head_const(&bvar), None);
    }
    #[test]
    fn test_conv_up_at_root_fails() {
        let focused = Expr::Const(Name::str("x"), vec![]);
        let goal = Expr::Sort(Level::Zero);
        let mvar = MVarId(0);
        let mut state = ConvState::new(focused, goal, mvar, ConvEntrySide::Lhs);
        assert!(conv_up(&mut state).is_err());
    }
    #[test]
    fn test_conv_arg_depth_limit() {
        let focused = Expr::Const(Name::str("x"), vec![]);
        let goal = Expr::Sort(Level::Zero);
        let mvar = MVarId(0);
        let mut state = ConvState::new(focused, goal, mvar, ConvEntrySide::Lhs);
        for _ in 0..MAX_CONV_DEPTH {
            state.path.push(ConvPathStep::new(
                ConvDirection::Left,
                Expr::Sort(Level::Zero),
                0,
            ));
        }
        let mut ctx = mk_test_ctx();
        let result = conv_arg(0, &mut state, &mut ctx);
        assert!(result.is_err());
    }
}
