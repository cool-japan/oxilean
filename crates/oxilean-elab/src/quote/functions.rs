//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Expr, Level, Literal, Name};

use super::types::{
    ExprBuilder, QuoteBinding, QuoteBuilder, QuoteContext, QuoteEnv, QuoteMatchResult,
    QuotePattern, QuoteScope, QuoteScopeStack, QuoteSession, QuoteStats, QuotedPattern,
};

/// Quote an expression into its reflected meta-level form.
///
/// Delegates to `quote_expr` which builds the full `mkXxx`-constructor
/// representation of the expression.
pub fn quote(expr: &Expr) -> Expr {
    quote_expr(expr)
}
/// Unquote a reflected expression back to its runtime `Expr` form.
///
/// Delegates to `unquote_expr` which recognises and inverts the `mkXxx`
/// constructors produced by `quote_expr`.
pub fn unquote(expr: &Expr) -> Result<Expr, String> {
    unquote_expr(expr)
}
/// Quote an expression into its reflected meta-level representation.
///
/// Converts a kernel `Expr` into a meta-level `Expr` that *represents*
/// the structure of the original expression.  For example:
/// - `Expr::Const(n, ls)` becomes `App(App(mkConst, reflect_name_q(n)), reflect_level_qs(ls))`
/// - `Expr::App(f, a)`   becomes `App(App(mkApp, quote_expr(f)), quote_expr(a))`
/// - `Expr::Sort(l)`     becomes `App(mkSort, reflect_level_q(l))`
///
/// This is analogous to Lean 4's `quote` for `Expr`.  Complex cases
/// (MVar, FVar) are reflected conservatively.
pub fn quote_expr(expr: &Expr) -> Expr {
    match expr {
        Expr::Sort(l) => mk_app2(mk_const("Expr.mkSort"), reflect_level_q(l)),
        Expr::BVar(i) => mk_app2(mk_const("Expr.mkBVar"), Expr::Lit(Literal::Nat(*i as u64))),
        Expr::FVar(id) => mk_app2(mk_const("Expr.mkFVar"), Expr::Lit(Literal::Nat(id.0))),
        Expr::Const(name, levels) => {
            let name_arg = reflect_name_q(name);
            let levels_arg = reflect_level_q_list_q(levels);
            mk_app3(mk_const("Expr.mkConst"), name_arg, levels_arg)
        }
        Expr::App(f, a) => {
            let qf = quote_expr(f);
            let qa = quote_expr(a);
            mk_app3(mk_const("Expr.mkApp"), qf, qa)
        }
        Expr::Lam(bi, name, ty, body) => {
            let bi_arg = reflect_binder_info_q(*bi);
            let name_arg = reflect_name_q(name);
            let ty_arg = quote_expr(ty);
            let body_arg = quote_expr(body);
            mk_app5(mk_const("Expr.mkLam"), bi_arg, name_arg, ty_arg, body_arg)
        }
        Expr::Pi(bi, name, ty, body) => {
            let bi_arg = reflect_binder_info_q(*bi);
            let name_arg = reflect_name_q(name);
            let ty_arg = quote_expr(ty);
            let body_arg = quote_expr(body);
            mk_app5(mk_const("Expr.mkPi"), bi_arg, name_arg, ty_arg, body_arg)
        }
        Expr::Let(name, ty, val, body) => {
            let name_arg = reflect_name_q(name);
            let ty_arg = quote_expr(ty);
            let val_arg = quote_expr(val);
            let body_arg = quote_expr(body);
            mk_app5(mk_const("Expr.mkLet"), name_arg, ty_arg, val_arg, body_arg)
        }
        Expr::Lit(lit) => {
            let lit_arg = reflect_literal_q(lit);
            mk_app2(mk_const("Expr.mkLit"), lit_arg)
        }
        Expr::Proj(struct_name, idx, inner) => {
            let sn_arg = reflect_name_q(struct_name);
            let idx_arg = Expr::Lit(Literal::Nat(*idx as u64));
            let inner_arg = quote_expr(inner);
            mk_app4(mk_const("Expr.mkProj"), sn_arg, idx_arg, inner_arg)
        }
    }
}
/// Unquote a meta-level reflected expression back to a kernel expression.
///
/// This is the inverse of `quote_expr`.  It recognises the `mkXxx`
/// constructors produced by `quote_expr` and deconstructs them back
/// into the original `Expr` node.  Returns an error if the reflected
/// form is not well-formed.
pub fn unquote_expr(expr: &Expr) -> Result<Expr, String> {
    let (head, args) = split_app(expr);
    match head {
        Expr::Const(name, _) => match name.to_string().as_str() {
            "Expr.mkSort" => {
                if args.len() == 1 {
                    let level = unquote_level_q(args[0])?;
                    Ok(Expr::Sort(level))
                } else {
                    Err(format!("Expr.mkSort: expected 1 arg, got {}", args.len()))
                }
            }
            "Expr.mkBVar" => {
                if args.len() == 1 {
                    if let Expr::Lit(Literal::Nat(i)) = args[0] {
                        Ok(Expr::BVar(*i as u32))
                    } else {
                        Err("Expr.mkBVar: expected Nat literal".to_string())
                    }
                } else {
                    Err(format!("Expr.mkBVar: expected 1 arg, got {}", args.len()))
                }
            }
            "Expr.mkFVar" => {
                if args.len() == 1 {
                    if let Expr::Lit(Literal::Nat(id)) = args[0] {
                        Ok(Expr::FVar(oxilean_kernel::FVarId(*id)))
                    } else {
                        Err("Expr.mkFVar: expected Nat literal".to_string())
                    }
                } else {
                    Err(format!("Expr.mkFVar: expected 1 arg, got {}", args.len()))
                }
            }
            "Expr.mkConst" => {
                if args.len() == 2 {
                    let name = unquote_name_q(args[0])?;
                    let levels = unquote_level_q_list_q(args[1])?;
                    Ok(Expr::Const(name, levels))
                } else {
                    Err(format!("Expr.mkConst: expected 2 args, got {}", args.len()))
                }
            }
            "Expr.mkApp" => {
                if args.len() == 2 {
                    let f = unquote_expr(args[0])?;
                    let a = unquote_expr(args[1])?;
                    Ok(Expr::App(Box::new(f), Box::new(a)))
                } else {
                    Err(format!("Expr.mkApp: expected 2 args, got {}", args.len()))
                }
            }
            "Expr.mkLam" => {
                if args.len() == 4 {
                    let bi = unquote_binder_info_q(args[0])?;
                    let name = unquote_name_q(args[1])?;
                    let ty = unquote_expr(args[2])?;
                    let body = unquote_expr(args[3])?;
                    Ok(Expr::Lam(bi, name, Box::new(ty), Box::new(body)))
                } else {
                    Err(format!("Expr.mkLam: expected 4 args, got {}", args.len()))
                }
            }
            "Expr.mkPi" => {
                if args.len() == 4 {
                    let bi = unquote_binder_info_q(args[0])?;
                    let name = unquote_name_q(args[1])?;
                    let ty = unquote_expr(args[2])?;
                    let body = unquote_expr(args[3])?;
                    Ok(Expr::Pi(bi, name, Box::new(ty), Box::new(body)))
                } else {
                    Err(format!("Expr.mkPi: expected 4 args, got {}", args.len()))
                }
            }
            "Expr.mkLet" => {
                if args.len() == 4 {
                    let name = unquote_name_q(args[0])?;
                    let ty = unquote_expr(args[1])?;
                    let val = unquote_expr(args[2])?;
                    let body = unquote_expr(args[3])?;
                    Ok(Expr::Let(name, Box::new(ty), Box::new(val), Box::new(body)))
                } else {
                    Err(format!("Expr.mkLet: expected 4 args, got {}", args.len()))
                }
            }
            "Expr.mkLit" => {
                if args.len() == 1 {
                    let lit = unquote_literal_q(args[0])?;
                    Ok(Expr::Lit(lit))
                } else {
                    Err(format!("Expr.mkLit: expected 1 arg, got {}", args.len()))
                }
            }
            "Expr.mkProj" => {
                if args.len() == 3 {
                    let struct_name = unquote_name_q(args[0])?;
                    let idx = if let Expr::Lit(Literal::Nat(i)) = args[1] {
                        *i as u32
                    } else {
                        return Err("Expr.mkProj: expected Nat index".to_string());
                    };
                    let inner = unquote_expr(args[2])?;
                    Ok(Expr::Proj(struct_name, idx, Box::new(inner)))
                } else {
                    Err(format!("Expr.mkProj: expected 3 args, got {}", args.len()))
                }
            }
            _ => Ok(expr.clone()),
        },
        _ => Ok(expr.clone()),
    }
}
/// Build `App(f, a)`.
fn mk_app2(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
/// Build `App(App(f, a), b)`.
fn mk_app3(f: Expr, a: Expr, b: Expr) -> Expr {
    mk_app2(mk_app2(f, a), b)
}
/// Build `App(App(App(f, a), b), c)`.
fn mk_app4(f: Expr, a: Expr, b: Expr, c: Expr) -> Expr {
    mk_app2(mk_app3(f, a, b), c)
}
/// Build `App(App(App(App(f, a), b), c), d)`.
fn mk_app5(f: Expr, a: Expr, b: Expr, c: Expr, d: Expr) -> Expr {
    mk_app2(mk_app4(f, a, b, c), d)
}
/// Build a `Const(name, [])` atom.
fn mk_const(name: &str) -> Expr {
    Expr::Const(Name::str(name), vec![])
}
/// Reflect a `Name` as `Expr::Const("Name.mk", [])` applied to string literals.
///
/// For simplicity, represents a name as `App(Name.mk, Str(display))`.
fn reflect_name_q(name: &Name) -> Expr {
    mk_app2(
        mk_const("Name.mk"),
        Expr::Lit(Literal::Str(name.to_string())),
    )
}
/// Unquote a reflected name back to a `Name`.
fn unquote_name_q(expr: &Expr) -> Result<Name, String> {
    match expr {
        Expr::App(f, arg) => {
            if let Expr::Const(n, _) = f.as_ref() {
                if n == &Name::str("Name.mk") {
                    if let Expr::Lit(Literal::Str(s)) = arg.as_ref() {
                        return Ok(Name::str(s.as_str()));
                    }
                }
            }
            Err(format!("unquote_name_q: unexpected form {:?}", expr))
        }
        _ => Err(format!("unquote_name_q: expected App, got {:?}", expr)),
    }
}
/// Reflect a `Level` as an expression.
fn reflect_level_q(level: &Level) -> Expr {
    match level {
        Level::Zero => mk_const("Level.zero"),
        Level::Succ(l) => mk_app2(mk_const("Level.succ"), reflect_level_q(l)),
        Level::Max(l1, l2) => mk_app3(
            mk_const("Level.max"),
            reflect_level_q(l1),
            reflect_level_q(l2),
        ),
        Level::IMax(l1, l2) => mk_app3(
            mk_const("Level.imax"),
            reflect_level_q(l1),
            reflect_level_q(l2),
        ),
        Level::Param(name) => mk_app2(mk_const("Level.param"), reflect_name_q(name)),
        Level::MVar(_) => mk_const("Level.zero"),
    }
}
/// Unquote a reflected level back to a `Level`.
fn unquote_level_q(expr: &Expr) -> Result<Level, String> {
    let (head, args) = split_app(expr);
    match head {
        Expr::Const(name, _) => match name.to_string().as_str() {
            "Level.zero" => Ok(Level::zero()),
            "Level.succ" if args.len() == 1 => Ok(Level::succ(unquote_level_q(args[0])?)),
            "Level.max" if args.len() == 2 => Ok(Level::max(
                unquote_level_q(args[0])?,
                unquote_level_q(args[1])?,
            )),
            "Level.imax" if args.len() == 2 => Ok(Level::imax(
                unquote_level_q(args[0])?,
                unquote_level_q(args[1])?,
            )),
            "Level.param" if args.len() == 1 => {
                let n = unquote_name_q(args[0])?;
                Ok(Level::param(n))
            }
            _ => Err(format!("unquote_level_q: unknown form {:?}", expr)),
        },
        _ => Err(format!(
            "unquote_level_q: expected Const head, got {:?}",
            expr
        )),
    }
}
/// Reflect a list of levels as a linked list of `List.cons` / `List.nil`.
fn reflect_level_q_list_q(levels: &[Level]) -> Expr {
    levels.iter().rev().fold(mk_const("List.nil"), |acc, l| {
        mk_app3(mk_const("List.cons"), reflect_level_q(l), acc)
    })
}
/// Unquote a reflected level list.
fn unquote_level_q_list_q(expr: &Expr) -> Result<Vec<Level>, String> {
    let mut result = Vec::new();
    let mut cur = expr;
    loop {
        let (head, args) = split_app(cur);
        match head {
            Expr::Const(name, _) if name == &Name::str("List.nil") => break,
            Expr::Const(name, _) if name == &Name::str("List.cons") && args.len() == 2 => {
                result.push(unquote_level_q(args[0])?);
                cur = args[1];
            }
            _ => {
                return Err(format!(
                    "unquote_level_q_list_q: unexpected form {:?}",
                    expr
                ));
            }
        }
    }
    Ok(result)
}
/// Reflect a `BinderInfo` as a constant expression.
fn reflect_binder_info_q(bi: BinderInfo) -> Expr {
    match bi {
        BinderInfo::Default => mk_const("BinderInfo.default"),
        BinderInfo::Implicit => mk_const("BinderInfo.implicit"),
        BinderInfo::StrictImplicit => mk_const("BinderInfo.strictImplicit"),
        BinderInfo::InstImplicit => mk_const("BinderInfo.instImplicit"),
    }
}
/// Unquote a reflected `BinderInfo`.
fn unquote_binder_info_q(expr: &Expr) -> Result<BinderInfo, String> {
    if let Expr::Const(name, _) = expr {
        match name.to_string().as_str() {
            "BinderInfo.default" => Ok(BinderInfo::Default),
            "BinderInfo.implicit" => Ok(BinderInfo::Implicit),
            "BinderInfo.strictImplicit" => Ok(BinderInfo::StrictImplicit),
            "BinderInfo.instImplicit" => Ok(BinderInfo::InstImplicit),
            _ => Err(format!("unquote_binder_info_q: unknown {:?}", name)),
        }
    } else {
        Err(format!(
            "unquote_binder_info_q: expected Const, got {:?}",
            expr
        ))
    }
}
/// Reflect a `Literal` as an expression.
fn reflect_literal_q(lit: &Literal) -> Expr {
    match lit {
        Literal::Nat(n) => mk_app2(mk_const("Literal.natVal"), Expr::Lit(Literal::Nat(*n))),
        Literal::Str(s) => mk_app2(
            mk_const("Literal.strVal"),
            Expr::Lit(Literal::Str(s.clone())),
        ),
    }
}
/// Unquote a reflected `Literal`.
fn unquote_literal_q(expr: &Expr) -> Result<Literal, String> {
    let (head, args) = split_app(expr);
    match head {
        Expr::Const(name, _) => match name.to_string().as_str() {
            "Literal.natVal" if args.len() == 1 => {
                if let Expr::Lit(Literal::Nat(n)) = args[0] {
                    Ok(Literal::Nat(*n))
                } else {
                    Err("Literal.natVal: expected Nat literal".to_string())
                }
            }
            "Literal.strVal" if args.len() == 1 => {
                if let Expr::Lit(Literal::Str(s)) = args[0] {
                    Ok(Literal::Str(s.clone()))
                } else {
                    Err("Literal.strVal: expected Str literal".to_string())
                }
            }
            _ => Err(format!("unquote_literal_q: unknown form {:?}", expr)),
        },
        _ => Err(format!(
            "unquote_literal_q: expected Const head, got {:?}",
            expr
        )),
    }
}
/// Quasi-quotation: quote with unquote escape hatches.
///
/// Inside a quasi-quotation, `$(name)` is replaced by the splice
/// previously registered under `name` in `ctx`.
pub fn quasi_quote(expr: &Expr, ctx: &mut QuoteContext) -> Result<Expr, String> {
    ctx.enter_quote();
    let result = quasi_quote_inner(expr, ctx)?;
    ctx.exit_quote();
    Ok(result)
}
fn quasi_quote_inner(expr: &Expr, ctx: &mut QuoteContext) -> Result<Expr, String> {
    if let Expr::App(f, arg) = expr {
        if let Expr::Const(f_name, _) = f.as_ref() {
            if f_name == &Name::str("$") {
                if let Expr::Const(splice_name, _) = arg.as_ref() {
                    if let Some(replacement) = ctx.take_splice(splice_name) {
                        return Ok(replacement);
                    } else if ctx.is_strict() {
                        return Err(format!("unbound splice: {:?}", splice_name));
                    } else {
                        return Ok(*arg.clone());
                    }
                }
            }
        }
    }
    Ok(expr.clone())
}
/// Apply a splice map to an expression.
///
/// Replaces all `Const(name, _)` nodes whose name is in `splices` with
/// the corresponding expression.
pub fn splice_expr(expr: &Expr, splices: &[(Name, Expr)]) -> Expr {
    match expr {
        Expr::Const(name, _) => {
            if let Some((_, replacement)) = splices.iter().find(|(n, _)| n == name) {
                return replacement.clone();
            }
            expr.clone()
        }
        Expr::App(f, a) => Expr::App(
            Box::new(splice_expr(f, splices)),
            Box::new(splice_expr(a, splices)),
        ),
        Expr::Lam(bi, n, ty, body) => Expr::Lam(
            *bi,
            n.clone(),
            Box::new(splice_expr(ty, splices)),
            Box::new(splice_expr(body, splices)),
        ),
        Expr::Pi(bi, n, ty, body) => Expr::Pi(
            *bi,
            n.clone(),
            Box::new(splice_expr(ty, splices)),
            Box::new(splice_expr(body, splices)),
        ),
        Expr::Let(n, ty, val, body) => Expr::Let(
            n.clone(),
            Box::new(splice_expr(ty, splices)),
            Box::new(splice_expr(val, splices)),
            Box::new(splice_expr(body, splices)),
        ),
        _ => expr.clone(),
    }
}
/// Check if an expression is a quotation.
pub fn is_quotation(expr: &Expr) -> bool {
    matches!(expr, Expr::Const(name, _) if name == & Name::str("quote"))
}
/// Check if an expression is an unquotation.
pub fn is_unquotation(expr: &Expr) -> bool {
    matches!(expr, Expr::Const(name, _) if name == & Name::str("unquote"))
}
/// Check if an expression is a lambda abstraction.
pub fn is_lambda(expr: &Expr) -> bool {
    matches!(expr, Expr::Lam(_, _, _, _))
}
/// Check if an expression is a pi type.
pub fn is_pi(expr: &Expr) -> bool {
    matches!(expr, Expr::Pi(_, _, _, _))
}
/// Check if an expression is a function application.
pub fn is_app(expr: &Expr) -> bool {
    matches!(expr, Expr::App(_, _))
}
/// Check if an expression is a constant.
pub fn is_const(expr: &Expr) -> bool {
    matches!(expr, Expr::Const(_, _))
}
/// Check if an expression is a literal.
pub fn is_literal(expr: &Expr) -> bool {
    matches!(expr, Expr::Lit(_))
}
/// Check if an expression is a sort.
pub fn is_sort(expr: &Expr) -> bool {
    matches!(expr, Expr::Sort(_))
}
/// Extract the head function and all arguments from a spine.
///
/// For `App(App(App(f, a1), a2), a3)` returns `(f, [a1, a2, a3])`.
pub fn unfold_app(expr: &Expr) -> (&Expr, Vec<&Expr>) {
    let mut args = Vec::new();
    let mut cur = expr;
    while let Expr::App(f, a) = cur {
        args.push(a.as_ref());
        cur = f.as_ref();
    }
    args.reverse();
    (cur, args)
}
/// Reflect a `Name` as a constant expression.
pub fn reflect_name(name: &Name) -> Expr {
    Expr::Const(name.clone(), vec![])
}
/// Reflect a natural number literal as an expression.
pub fn reflect_nat(n: u64) -> Expr {
    Expr::Lit(Literal::Nat(n))
}
/// Reflect a bool as a constant `Bool.true` or `Bool.false`.
pub fn reflect_bool(b: bool) -> Expr {
    if b {
        Expr::Const(Name::str("Bool.true"), vec![])
    } else {
        Expr::Const(Name::str("Bool.false"), vec![])
    }
}
/// Check whether an expression is `Bool.true`.
pub fn is_true_const(expr: &Expr) -> bool {
    matches!(expr, Expr::Const(n, _) if n == & Name::str("Bool.true"))
}
/// Check whether an expression is `Bool.false`.
pub fn is_false_const(expr: &Expr) -> bool {
    matches!(expr, Expr::Const(n, _) if n == & Name::str("Bool.false"))
}
/// Compute the depth of an expression tree.
pub fn expr_depth(expr: &Expr) -> usize {
    match expr {
        Expr::App(f, a) => 1 + expr_depth(f).max(expr_depth(a)),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            1 + expr_depth(ty).max(expr_depth(body))
        }
        Expr::Let(_, ty, val, body) => {
            1 + expr_depth(ty).max(expr_depth(val)).max(expr_depth(body))
        }
        Expr::Proj(_, _, inner) => 1 + expr_depth(inner),
        _ => 1,
    }
}
/// Count the total number of nodes in an expression tree.
pub fn expr_size(expr: &Expr) -> usize {
    match expr {
        Expr::App(f, a) => 1 + expr_size(f) + expr_size(a),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => 1 + expr_size(ty) + expr_size(body),
        Expr::Let(_, ty, val, body) => 1 + expr_size(ty) + expr_size(val) + expr_size(body),
        Expr::Proj(_, _, inner) => 1 + expr_size(inner),
        _ => 1,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::quote::*;
    fn nat_ty() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn sort0() -> Expr {
        Expr::Sort(Level::zero())
    }
    #[test]
    fn test_quote() {
        let expr = Expr::Lit(Literal::Nat(42));
        let quoted = quote(&expr);
        assert!(matches!(quoted, Expr::App(_, _)));
        let back = unquote(&quoted).expect("test operation should succeed");
        assert_eq!(back, expr);
    }
    #[test]
    fn test_unquote() {
        let expr = Expr::Lit(Literal::Nat(42));
        let unquoted = unquote(&expr).expect("test operation should succeed");
        assert_eq!(unquoted, expr);
    }
    #[test]
    fn test_quote_expr_sort() {
        let e = Expr::Sort(Level::zero());
        let q = quote_expr(&e);
        assert!(matches!(q, Expr::App(_, _)));
        let back = unquote_expr(&q).expect("test operation should succeed");
        assert_eq!(back, e);
    }
    #[test]
    fn test_quote_expr_bvar() {
        let e = Expr::BVar(3);
        let q = quote_expr(&e);
        let back = unquote_expr(&q).expect("test operation should succeed");
        assert_eq!(back, e);
    }
    #[test]
    fn test_quote_expr_const() {
        let e = Expr::Const(Name::str("Nat.succ"), vec![]);
        let q = quote_expr(&e);
        let back = unquote_expr(&q).expect("test operation should succeed");
        assert_eq!(back, e);
    }
    #[test]
    fn test_quote_expr_app() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::BVar(0);
        let e = Expr::App(Box::new(f), Box::new(a));
        let q = quote_expr(&e);
        let back = unquote_expr(&q).expect("test operation should succeed");
        assert_eq!(back, e);
    }
    #[test]
    fn test_quote_expr_lit_nat() {
        let e = Expr::Lit(Literal::Nat(42));
        let q = quote_expr(&e);
        let back = unquote_expr(&q).expect("test operation should succeed");
        assert_eq!(back, e);
    }
    #[test]
    fn test_quote_expr_lit_str() {
        let e = Expr::Lit(Literal::Str("hello".to_string()));
        let q = quote_expr(&e);
        let back = unquote_expr(&q).expect("test operation should succeed");
        assert_eq!(back, e);
    }
    #[test]
    fn test_quote_expr_lam() {
        let e = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
            Box::new(Expr::BVar(0)),
        );
        let q = quote_expr(&e);
        let back = unquote_expr(&q).expect("test operation should succeed");
        assert_eq!(back, e);
    }
    #[test]
    fn test_quote_expr_pi() {
        let e = Expr::Pi(
            BinderInfo::Implicit,
            Name::str("α"),
            Box::new(Expr::Sort(Level::succ(Level::zero()))),
            Box::new(Expr::BVar(0)),
        );
        let q = quote_expr(&e);
        let back = unquote_expr(&q).expect("test operation should succeed");
        assert_eq!(back, e);
    }
    #[test]
    fn test_quote_expr_let() {
        let e = Expr::Let(
            Name::str("x"),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
            Box::new(Expr::Lit(Literal::Nat(0))),
            Box::new(Expr::BVar(0)),
        );
        let q = quote_expr(&e);
        let back = unquote_expr(&q).expect("test operation should succeed");
        assert_eq!(back, e);
    }
    #[test]
    fn test_quote_expr_proj() {
        let e = Expr::Proj(
            Name::str("Prod"),
            1,
            Box::new(Expr::Const(Name::str("p"), vec![])),
        );
        let q = quote_expr(&e);
        let back = unquote_expr(&q).expect("test operation should succeed");
        assert_eq!(back, e);
    }
    #[test]
    fn test_unquote_expr_unknown_returns_identity() {
        let e = Expr::Const(Name::str("unknown.thing"), vec![]);
        let result = unquote_expr(&e).expect("test operation should succeed");
        assert_eq!(result, e);
    }
    #[test]
    fn test_quote_context_create() {
        let ctx = QuoteContext::new();
        assert_eq!(ctx.depth(), 0);
        assert!(!ctx.is_quoted());
    }
    #[test]
    fn test_quote_context_enter_exit() {
        let mut ctx = QuoteContext::new();
        ctx.enter_quote();
        assert_eq!(ctx.depth(), 1);
        assert!(ctx.is_quoted());
        ctx.exit_quote();
        assert_eq!(ctx.depth(), 0);
        assert!(!ctx.is_quoted());
    }
    #[test]
    fn test_quote_context_nested() {
        let mut ctx = QuoteContext::new();
        ctx.enter_quote();
        ctx.enter_quote();
        assert_eq!(ctx.depth(), 2);
        ctx.exit_quote();
        assert_eq!(ctx.depth(), 1);
        assert!(ctx.is_quoted());
    }
    #[test]
    fn test_add_splice() {
        let mut ctx = QuoteContext::new();
        let expr = Expr::Lit(Literal::Nat(42));
        ctx.add_splice(Name::str("x"), expr.clone());
        assert_eq!(ctx.splices().len(), 1);
        assert_eq!(ctx.splices()[0].0, Name::str("x"));
    }
    #[test]
    fn test_clear_splices() {
        let mut ctx = QuoteContext::new();
        ctx.add_splice(Name::str("x"), Expr::Lit(Literal::Nat(42)));
        ctx.clear_splices();
        assert_eq!(ctx.splices().len(), 0);
    }
    #[test]
    fn test_is_quotation() {
        let expr = Expr::Const(Name::str("quote"), vec![]);
        assert!(is_quotation(&expr));
        let non_quote = Expr::Sort(Level::zero());
        assert!(!is_quotation(&non_quote));
    }
    #[test]
    fn test_is_unquotation() {
        let expr = Expr::Const(Name::str("unquote"), vec![]);
        assert!(is_unquotation(&expr));
        let non_unquote = Expr::Sort(Level::zero());
        assert!(!is_unquotation(&non_unquote));
    }
    #[test]
    fn test_take_splice() {
        let mut ctx = QuoteContext::new();
        let e = Expr::Lit(Literal::Nat(1));
        ctx.add_splice(Name::str("a"), e.clone());
        let taken = ctx.take_splice(&Name::str("a"));
        assert_eq!(taken, Some(e));
        assert!(ctx.splices().is_empty());
    }
    #[test]
    fn test_splice_expr_const() {
        let splices = vec![(Name::str("X"), nat_ty())];
        let expr = Expr::Const(Name::str("X"), vec![]);
        let result = splice_expr(&expr, &splices);
        assert_eq!(result, nat_ty());
    }
    #[test]
    fn test_splice_expr_no_match() {
        let splices = vec![(Name::str("Y"), nat_ty())];
        let expr = Expr::Const(Name::str("Z"), vec![]);
        let result = splice_expr(&expr, &splices);
        assert_eq!(result, expr);
    }
    #[test]
    fn test_splice_expr_app() {
        let splices = vec![(Name::str("A"), nat_ty())];
        let a = Expr::Const(Name::str("A"), vec![]);
        let b = Expr::Const(Name::str("B"), vec![]);
        let app = Expr::App(Box::new(a), Box::new(b.clone()));
        let result = splice_expr(&app, &splices);
        assert!(matches!(result, Expr::App(f, _) if * f == nat_ty()));
    }
    #[test]
    fn test_expr_builder_nat_lit() {
        let e = ExprBuilder::nat_lit(42).build();
        assert_eq!(e, Expr::Lit(Literal::Nat(42)));
    }
    #[test]
    fn test_expr_builder_cnst() {
        let e = ExprBuilder::cnst("Nat").build();
        assert_eq!(e, nat_ty());
    }
    #[test]
    fn test_expr_builder_app() {
        let e = ExprBuilder::cnst("f").app(ExprBuilder::nat_lit(1)).build();
        assert!(is_app(&e));
    }
    #[test]
    fn test_expr_builder_app_many() {
        let e = ExprBuilder::cnst("f")
            .app_many([ExprBuilder::nat_lit(1), ExprBuilder::nat_lit(2)])
            .build();
        assert!(is_app(&e));
        assert_eq!(expr_depth(&e), 3);
    }
    #[test]
    fn test_expr_builder_lam() {
        let e = ExprBuilder::lam("x", ExprBuilder::cnst("Nat"), ExprBuilder::bvar(0)).build();
        assert!(is_lambda(&e));
    }
    #[test]
    fn test_expr_builder_pi() {
        let e = ExprBuilder::pi("x", ExprBuilder::cnst("Nat"), ExprBuilder::cnst("Nat")).build();
        assert!(is_pi(&e));
    }
    #[test]
    fn test_expr_builder_arrow() {
        let e = ExprBuilder::arrow(ExprBuilder::cnst("Nat"), ExprBuilder::cnst("Bool")).build();
        assert!(is_pi(&e));
    }
    #[test]
    fn test_expr_builder_let_bind() {
        let e = ExprBuilder::let_bind(
            "x",
            ExprBuilder::cnst("Nat"),
            ExprBuilder::nat_lit(0),
            ExprBuilder::bvar(0),
        )
        .build();
        assert!(matches!(e, Expr::Let(_, _, _, _)));
    }
    #[test]
    fn test_expr_builder_sort() {
        let prop = ExprBuilder::prop().build();
        assert!(is_sort(&prop));
    }
    #[test]
    fn test_unfold_app() {
        let e = ExprBuilder::cnst("f")
            .app(ExprBuilder::nat_lit(1))
            .app(ExprBuilder::nat_lit(2))
            .build();
        let (head, args) = unfold_app(&e);
        assert_eq!(*head, Expr::Const(Name::str("f"), vec![]));
        assert_eq!(args.len(), 2);
    }
    #[test]
    fn test_expr_depth_leaf() {
        assert_eq!(expr_depth(&nat_ty()), 1);
        assert_eq!(expr_depth(&sort0()), 1);
    }
    #[test]
    fn test_expr_depth_app() {
        let e = ExprBuilder::cnst("f").app(ExprBuilder::cnst("x")).build();
        assert_eq!(expr_depth(&e), 2);
    }
    #[test]
    fn test_expr_size_app() {
        let e = ExprBuilder::cnst("f")
            .app(ExprBuilder::nat_lit(1))
            .app(ExprBuilder::nat_lit(2))
            .build();
        assert_eq!(expr_size(&e), 5);
    }
    #[test]
    fn test_reflect_bool() {
        let t = reflect_bool(true);
        assert!(is_true_const(&t));
        let f = reflect_bool(false);
        assert!(is_false_const(&f));
    }
    #[test]
    fn test_reflect_nat() {
        let n = reflect_nat(99);
        assert_eq!(n, Expr::Lit(Literal::Nat(99)));
        assert!(is_literal(&n));
    }
    #[test]
    fn test_is_const() {
        assert!(is_const(&nat_ty()));
        assert!(!is_const(&sort0()));
    }
    #[test]
    fn test_quasi_quote_passthrough() {
        let mut ctx = QuoteContext::new();
        let expr = nat_ty();
        let result = quasi_quote(&expr, &mut ctx).expect("test operation should succeed");
        assert_eq!(result, expr);
        assert!(!ctx.is_quoted());
    }
    #[test]
    fn test_expr_builder_proj() {
        let e = ExprBuilder::proj("Prod", 0, ExprBuilder::cnst("p")).build();
        assert!(matches!(e, Expr::Proj(_, 0, _)));
    }
    #[test]
    fn test_expr_builder_str_lit() {
        let e = ExprBuilder::str_lit("hello").build();
        assert_eq!(e, Expr::Lit(Literal::Str("hello".into())));
        assert!(is_literal(&e));
    }
}
/// Recursively apply a splice map to all sub-expressions.
///
/// Unlike `splice_expr`, this also descends into binder types, bodies,
/// and let-bindings, ensuring every occurrence is replaced.
pub fn deep_splice(expr: &Expr, splices: &[(Name, Expr)]) -> Expr {
    if let Expr::Const(name, _) = expr {
        if let Some((_, repl)) = splices.iter().find(|(n, _)| n == name) {
            return repl.clone();
        }
    }
    match expr {
        Expr::App(f, a) => Expr::App(
            Box::new(deep_splice(f, splices)),
            Box::new(deep_splice(a, splices)),
        ),
        Expr::Lam(bi, n, ty, body) => Expr::Lam(
            *bi,
            n.clone(),
            Box::new(deep_splice(ty, splices)),
            Box::new(deep_splice(body, splices)),
        ),
        Expr::Pi(bi, n, ty, body) => Expr::Pi(
            *bi,
            n.clone(),
            Box::new(deep_splice(ty, splices)),
            Box::new(deep_splice(body, splices)),
        ),
        Expr::Let(n, ty, val, body) => Expr::Let(
            n.clone(),
            Box::new(deep_splice(ty, splices)),
            Box::new(deep_splice(val, splices)),
            Box::new(deep_splice(body, splices)),
        ),
        Expr::Proj(n, i, inner) => Expr::Proj(n.clone(), *i, Box::new(deep_splice(inner, splices))),
        _ => expr.clone(),
    }
}
/// Collect all free variable IDs referenced in an expression.
pub fn free_vars(expr: &Expr) -> Vec<oxilean_kernel::FVarId> {
    let mut vars = Vec::new();
    collect_fvars(expr, &mut vars);
    vars
}
fn collect_fvars(expr: &Expr, acc: &mut Vec<oxilean_kernel::FVarId>) {
    match expr {
        Expr::FVar(id) => {
            if !acc.contains(id) {
                acc.push(*id);
            }
        }
        Expr::App(f, a) => {
            collect_fvars(f, acc);
            collect_fvars(a, acc);
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            collect_fvars(ty, acc);
            collect_fvars(body, acc);
        }
        Expr::Let(_, ty, val, body) => {
            collect_fvars(ty, acc);
            collect_fvars(val, acc);
            collect_fvars(body, acc);
        }
        Expr::Proj(_, _, inner) => collect_fvars(inner, acc),
        _ => {}
    }
}
#[cfg(test)]
mod tests_extra {
    use super::*;
    use crate::quote::*;
    use oxilean_kernel::FVarId;
    fn nat_ty() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn bool_ty() -> Expr {
        Expr::Const(Name::str("Bool"), vec![])
    }
    #[test]
    fn test_deep_splice_nested() {
        let splices = vec![(Name::str("T"), nat_ty())];
        let inner = Expr::Const(Name::str("T"), vec![]);
        let lam = ExprBuilder::lam("x", ExprBuilder::cnst("T"), ExprBuilder::bvar(0)).build();
        let result = deep_splice(&lam, &splices);
        if let Expr::Lam(_, _, ty, _) = &result {
            assert_eq!(**ty, nat_ty());
        } else {
            panic!("expected Lam");
        }
        let _ = inner;
    }
    #[test]
    fn test_deep_splice_no_match() {
        let splices = vec![(Name::str("X"), nat_ty())];
        let expr = bool_ty();
        let result = deep_splice(&expr, &splices);
        assert_eq!(result, bool_ty());
    }
    #[test]
    fn test_free_vars_none() {
        let expr = nat_ty();
        let fvars = free_vars(&expr);
        assert!(fvars.is_empty());
    }
    #[test]
    fn test_free_vars_single() {
        let id = FVarId(42);
        let expr = Expr::FVar(id);
        let fvars = free_vars(&expr);
        assert_eq!(fvars.len(), 1);
        assert_eq!(fvars[0], id);
    }
    #[test]
    fn test_free_vars_in_app() {
        let id1 = FVarId(1);
        let id2 = FVarId(2);
        let expr = Expr::App(Box::new(Expr::FVar(id1)), Box::new(Expr::FVar(id2)));
        let mut fvars = free_vars(&expr);
        fvars.sort_by_key(|v| v.0);
        assert_eq!(fvars, vec![id1, id2]);
    }
    #[test]
    fn test_free_vars_no_duplicates() {
        let id = FVarId(5);
        let expr = Expr::App(Box::new(Expr::FVar(id)), Box::new(Expr::FVar(id)));
        let fvars = free_vars(&expr);
        assert_eq!(fvars.len(), 1);
    }
    #[test]
    fn test_expr_depth_nested_lam() {
        let lam = ExprBuilder::lam(
            "x",
            ExprBuilder::cnst("Nat"),
            ExprBuilder::lam("y", ExprBuilder::cnst("Nat"), ExprBuilder::bvar(0)),
        )
        .build();
        assert!(expr_depth(&lam) >= 3);
    }
    #[test]
    fn test_expr_size_single_node() {
        assert_eq!(expr_size(&nat_ty()), 1);
    }
    #[test]
    fn test_reflect_name() {
        let n = Name::str("Foo");
        let e = reflect_name(&n);
        assert_eq!(e, Expr::Const(n, vec![]));
    }
    #[test]
    fn test_is_lambda_and_pi() {
        let lam = ExprBuilder::lam("x", ExprBuilder::cnst("T"), ExprBuilder::bvar(0)).build();
        assert!(is_lambda(&lam));
        assert!(!is_pi(&lam));
        let pi = ExprBuilder::pi("x", ExprBuilder::cnst("T"), ExprBuilder::cnst("U")).build();
        assert!(is_pi(&pi));
        assert!(!is_lambda(&pi));
    }
    #[test]
    fn test_unfold_app_single() {
        let e = nat_ty();
        let (head, args) = unfold_app(&e);
        assert_eq!(*head, nat_ty());
        assert!(args.is_empty());
    }
    #[test]
    fn test_expr_builder_implicit_lam() {
        let e = ExprBuilder::lam_implicit("α", ExprBuilder::prop(), ExprBuilder::bvar(0)).build();
        if let Expr::Lam(bi, _, _, _) = e {
            assert_eq!(bi, BinderInfo::Implicit);
        } else {
            panic!("expected Lam");
        }
    }
}
/// Split an expression's application spine into head + arguments.
pub fn split_app(expr: &Expr) -> (&Expr, Vec<&Expr>) {
    let mut args = Vec::new();
    let mut cur = expr;
    while let Expr::App(f, a) = cur {
        args.push(a.as_ref());
        cur = f;
    }
    args.reverse();
    (cur, args)
}
/// Count how many arguments are in the application spine.
pub fn app_spine_len(expr: &Expr) -> usize {
    let (_, args) = split_app(expr);
    args.len()
}
/// Fold a head expression and argument list into a left-associated application.
pub fn fold_app(head: Expr, args: Vec<Expr>) -> Expr {
    args.into_iter()
        .fold(head, |acc, arg| Expr::App(Box::new(acc), Box::new(arg)))
}
#[cfg(test)]
mod quote_extra_tests {
    use super::*;
    use crate::quote::*;
    fn nat() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn bool_c() -> Expr {
        Expr::Const(Name::str("Bool"), vec![])
    }
    #[test]
    fn test_quoted_pattern_any() {
        let p = QuotedPattern::Any;
        assert!(p.matches(&nat()));
        assert!(p.matches(&Expr::BVar(0)));
    }
    #[test]
    fn test_quoted_pattern_const_match() {
        let p = QuotedPattern::Const(Name::str("Nat"));
        assert!(p.matches(&nat()));
        assert!(!p.matches(&bool_c()));
    }
    #[test]
    fn test_quoted_pattern_bvar() {
        let p = QuotedPattern::BVar(0);
        assert!(p.matches(&Expr::BVar(0)));
        assert!(!p.matches(&Expr::BVar(1)));
    }
    #[test]
    fn test_quoted_pattern_sort() {
        let p = QuotedPattern::Sort;
        assert!(p.matches(&Expr::Sort(Level::zero())));
        assert!(!p.matches(&nat()));
    }
    #[test]
    fn test_quoted_pattern_app() {
        let p = QuotedPattern::App(
            Box::new(QuotedPattern::Const(Name::str("f"))),
            Box::new(QuotedPattern::Any),
        );
        let e = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(Expr::BVar(0)),
        );
        assert!(p.matches(&e));
    }
    #[test]
    fn test_quoted_pattern_head_name() {
        let p = QuotedPattern::Const(Name::str("Nat"));
        assert_eq!(p.head_name(), Some(&Name::str("Nat")));
        let p2 = QuotedPattern::Any;
        assert_eq!(p2.head_name(), None);
    }
    #[test]
    fn test_split_app_atom() {
        let e = nat();
        let (head, args) = split_app(&e);
        assert_eq!(head, &nat());
        assert!(args.is_empty());
    }
    #[test]
    fn test_split_app_two_args() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::BVar(0);
        let b = Expr::BVar(1);
        let app = Expr::App(
            Box::new(Expr::App(Box::new(f), Box::new(a.clone()))),
            Box::new(b.clone()),
        );
        let (head, args) = split_app(&app);
        assert!(matches!(head, Expr::Const(n, _) if * n == Name::str("f")));
        assert_eq!(args.len(), 2);
        assert_eq!(*args[0], a);
        assert_eq!(*args[1], b);
    }
    #[test]
    fn test_app_spine_len() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a1 = Expr::BVar(0);
        let a2 = Expr::BVar(1);
        let e = Expr::App(Box::new(Expr::App(Box::new(f), Box::new(a1))), Box::new(a2));
        assert_eq!(app_spine_len(&e), 2);
    }
    #[test]
    fn test_fold_app() {
        let head = Expr::Const(Name::str("f"), vec![]);
        let args = vec![Expr::BVar(0), Expr::BVar(1)];
        let e = fold_app(head.clone(), args);
        assert_eq!(app_spine_len(&e), 2);
    }
}
/// Match `pattern` against `expr`, accumulating captures in `result`.
pub fn quote_match(pattern: &QuotePattern, expr: &Expr, result: &mut QuoteMatchResult) -> bool {
    match (pattern, expr) {
        (QuotePattern::Any, _) => true,
        (QuotePattern::AnySort, Expr::Sort(_)) => true,
        (QuotePattern::BVar(i), Expr::BVar(j)) => i == j,
        (QuotePattern::Const(n), Expr::Const(m, _)) => n == m,
        (QuotePattern::App(hp, ap), Expr::App(h, a)) => {
            quote_match(hp, h, result) && quote_match(ap, a, result)
        }
        (QuotePattern::Lam(maybe_name, body_pat), Expr::Lam(_bi, bname, _ty, body)) => {
            if let Some(expected) = maybe_name {
                if bname != expected {
                    return false;
                }
            }
            quote_match(body_pat, body, result)
        }
        (QuotePattern::Pi(maybe_name, dom_pat, cod_pat), Expr::Pi(_bi, bname, dom, cod)) => {
            if let Some(expected) = maybe_name {
                if bname != expected {
                    return false;
                }
            }
            quote_match(dom_pat, dom, result) && quote_match(cod_pat, cod, result)
        }
        (QuotePattern::Capture(name, inner), e) => {
            let mut sub = QuoteMatchResult::new();
            if quote_match(inner, e, &mut sub) {
                result.bind(name.clone(), e.clone());
                result.merge(sub);
                true
            } else {
                false
            }
        }
        _ => false,
    }
}
/// Beta-reduces `(fun x => body) arg` at the meta level.
#[allow(dead_code)]
pub fn quote_beta_reduce(expr: &Expr) -> Expr {
    match expr {
        Expr::App(f, arg) => {
            let f_red = quote_beta_reduce(f);
            let a_red = quote_beta_reduce(arg);
            if let Expr::Lam(_bi, _n, _ty, body) = &f_red {
                qsbv(body, 0, &a_red)
            } else {
                Expr::App(Box::new(f_red), Box::new(a_red))
            }
        }
        Expr::Lam(bi, n, ty, body) => {
            let body2 = quote_beta_reduce(body);
            Expr::Lam(*bi, n.clone(), ty.clone(), Box::new(body2))
        }
        Expr::Pi(bi, n, ty, body) => {
            let ty2 = quote_beta_reduce(ty);
            let body2 = quote_beta_reduce(body);
            Expr::Pi(*bi, n.clone(), Box::new(ty2), Box::new(body2))
        }
        other => other.clone(),
    }
}
/// Substitute de Bruijn variable `depth` with `replacement` in `expr`.
fn qsbv(expr: &Expr, depth: u32, replacement: &Expr) -> Expr {
    match expr {
        Expr::BVar(i) => {
            if *i == depth {
                replacement.clone()
            } else if *i > depth {
                Expr::BVar(i - 1)
            } else {
                expr.clone()
            }
        }
        Expr::App(f, a) => Expr::App(
            Box::new(qsbv(f, depth, replacement)),
            Box::new(qsbv(a, depth, replacement)),
        ),
        Expr::Lam(bi, n, ty, body) => {
            let ty2 = qsbv(ty, depth, replacement);
            let body2 = qsbv(body, depth + 1, replacement);
            Expr::Lam(*bi, n.clone(), Box::new(ty2), Box::new(body2))
        }
        Expr::Pi(bi, n, ty, body) => {
            let ty2 = qsbv(ty, depth, replacement);
            let body2 = qsbv(body, depth + 1, replacement);
            Expr::Pi(*bi, n.clone(), Box::new(ty2), Box::new(body2))
        }
        other => other.clone(),
    }
}
/// Build a quoted application chain: `f a₁ a₂ … aₙ`.
#[allow(dead_code)]
pub fn quote_app_chain(f: Expr, args: impl IntoIterator<Item = Expr>) -> Expr {
    args.into_iter()
        .fold(f, |acc, a| Expr::App(Box::new(acc), Box::new(a)))
}
/// Check whether two expressions are alpha-equivalent under quotation.
#[allow(dead_code)]
pub fn quote_alpha_eq(a: &Expr, b: &Expr) -> bool {
    match (a, b) {
        (Expr::BVar(i), Expr::BVar(j)) => i == j,
        (Expr::FVar(i), Expr::FVar(j)) => i == j,
        (Expr::Const(n, ls), Expr::Const(m, ms)) => n == m && ls.len() == ms.len(),
        (Expr::Sort(l1), Expr::Sort(l2)) => l1 == l2,
        (Expr::Lit(l1), Expr::Lit(l2)) => l1 == l2,
        (Expr::App(f1, a1), Expr::App(f2, a2)) => quote_alpha_eq(f1, f2) && quote_alpha_eq(a1, a2),
        (Expr::Lam(_, _, ty1, b1), Expr::Lam(_, _, ty2, b2)) => {
            quote_alpha_eq(ty1, ty2) && quote_alpha_eq(b1, b2)
        }
        (Expr::Pi(_, _, ty1, b1), Expr::Pi(_, _, ty2, b2)) => {
            quote_alpha_eq(ty1, ty2) && quote_alpha_eq(b1, b2)
        }
        _ => false,
    }
}
/// Collect all free de Bruijn variables (relative indices) in `expr`.
#[allow(dead_code)]
pub fn quote_free_bvars(expr: &Expr, depth: u32) -> std::collections::HashSet<u32> {
    let mut out = std::collections::HashSet::new();
    qfbv_rec(expr, depth, &mut out);
    out
}
fn qfbv_rec(expr: &Expr, depth: u32, out: &mut std::collections::HashSet<u32>) {
    match expr {
        Expr::BVar(i) => {
            if *i >= depth {
                out.insert(*i - depth);
            }
        }
        Expr::App(f, a) => {
            qfbv_rec(f, depth, out);
            qfbv_rec(a, depth, out);
        }
        Expr::Lam(_, _, ty, body) => {
            qfbv_rec(ty, depth, out);
            qfbv_rec(body, depth + 1, out);
        }
        Expr::Pi(_, _, ty, body) => {
            qfbv_rec(ty, depth, out);
            qfbv_rec(body, depth + 1, out);
        }
        _ => {}
    }
}
#[cfg(test)]
mod quote_ext_tests {
    use super::*;
    use crate::quote::*;
    fn nat_ty() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    #[test]
    fn test_quote_env_push_pop() {
        let mut env = QuoteEnv::new();
        assert!(env.is_empty());
        env.push(Name::str("x"), nat_ty());
        assert_eq!(env.depth(), 1);
        env.pop();
        assert!(env.is_empty());
    }
    #[test]
    fn test_quote_env_lookup() {
        let mut env = QuoteEnv::new();
        env.push(Name::str("a"), nat_ty());
        env.push(Name::str("b"), nat_ty());
        assert_eq!(env.lookup(&Name::str("b")), Some(0));
        assert_eq!(env.lookup(&Name::str("a")), Some(1));
        assert_eq!(env.lookup(&Name::str("c")), None);
    }
    #[test]
    fn test_quote_env_snapshot_restore() {
        let mut env = QuoteEnv::new();
        env.push(Name::str("x"), nat_ty());
        let snap = env.snapshot();
        env.push(Name::str("y"), nat_ty());
        assert_eq!(env.depth(), 2);
        env.restore(snap);
        assert_eq!(env.depth(), 1);
    }
    #[test]
    fn test_quote_scope_bindings_in_scope() {
        let scope = QuoteScope::opaque("test", 3);
        assert_eq!(scope.bindings_in_scope(5), 2);
        assert_eq!(scope.bindings_in_scope(3), 0);
    }
    #[test]
    fn test_quote_scope_stack_push_pop() {
        let mut stack = QuoteScopeStack::new();
        assert!(stack.is_empty());
        stack.push(QuoteScope::opaque("A", 0));
        stack.push(QuoteScope::transparent("B", 1));
        assert_eq!(stack.depth(), 2);
        assert!(stack.has_transparent_ancestor());
        stack.pop();
        assert_eq!(stack.depth(), 1);
        assert!(!stack.has_transparent_ancestor());
    }
    #[test]
    fn test_quote_session_bind_resolve() {
        let mut sess = QuoteSession::new();
        sess.bind(Name::str("x"), nat_ty());
        sess.bind(Name::str("y"), nat_ty());
        assert_eq!(sess.resolve(&Name::str("y")), Some(0));
        assert_eq!(sess.resolve(&Name::str("x")), Some(1));
    }
    #[test]
    fn test_quote_session_quote_depth() {
        let mut sess = QuoteSession::new();
        assert_eq!(sess.quote_depth, 0);
        sess.enter_quote();
        sess.enter_quote();
        assert_eq!(sess.quote_depth, 2);
        sess.exit_quote();
        assert_eq!(sess.quote_depth, 1);
    }
    #[test]
    fn test_quote_match_any() {
        let mut res = QuoteMatchResult::new();
        let e = Expr::Const(Name::str("f"), vec![]);
        assert!(quote_match(&QuotePattern::Any, &e, &mut res));
    }
    #[test]
    fn test_quote_match_const() {
        let mut res = QuoteMatchResult::new();
        let e = Expr::Const(Name::str("Nat"), vec![]);
        assert!(quote_match(
            &QuotePattern::Const(Name::str("Nat")),
            &e,
            &mut res
        ));
        assert!(!quote_match(
            &QuotePattern::Const(Name::str("Int")),
            &e,
            &mut res
        ));
    }
    #[test]
    fn test_quote_match_capture() {
        let mut res = QuoteMatchResult::new();
        let e = Expr::Const(Name::str("Nat"), vec![]);
        let pat = QuotePattern::Capture("n".to_string(), Box::new(QuotePattern::Any));
        assert!(quote_match(&pat, &e, &mut res));
        assert!(res.get("n").is_some());
    }
    #[test]
    fn test_quote_match_bvar() {
        let mut res = QuoteMatchResult::new();
        let e = Expr::BVar(3);
        assert!(quote_match(&QuotePattern::BVar(3), &e, &mut res));
        assert!(!quote_match(&QuotePattern::BVar(2), &e, &mut res));
    }
    #[test]
    fn test_quote_beta_reduce_simple() {
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(nat_ty()),
            Box::new(Expr::BVar(0)),
        );
        let app = Expr::App(Box::new(lam), Box::new(nat_ty()));
        let reduced = quote_beta_reduce(&app);
        assert_eq!(reduced, nat_ty());
    }
    #[test]
    fn test_quote_alpha_eq_consts() {
        let a = Expr::Const(Name::str("Nat"), vec![]);
        let b = Expr::Const(Name::str("Nat"), vec![]);
        let c = Expr::Const(Name::str("Int"), vec![]);
        assert!(quote_alpha_eq(&a, &b));
        assert!(!quote_alpha_eq(&a, &c));
    }
    #[test]
    fn test_quote_alpha_eq_bvars() {
        assert!(quote_alpha_eq(&Expr::BVar(0), &Expr::BVar(0)));
        assert!(!quote_alpha_eq(&Expr::BVar(0), &Expr::BVar(1)));
    }
    #[test]
    fn test_quote_free_bvars() {
        let e = Expr::App(Box::new(Expr::BVar(2)), Box::new(Expr::BVar(0)));
        let free = quote_free_bvars(&e, 0);
        assert!(free.contains(&0));
        assert!(free.contains(&2));
        assert!(!free.contains(&1));
    }
    #[test]
    fn test_quote_free_bvars_under_lambda() {
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(nat_ty()),
            Box::new(Expr::BVar(0)),
        );
        let free = quote_free_bvars(&lam, 0);
        assert!(free.is_empty());
    }
    #[test]
    fn test_quote_app_chain() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let args = vec![Expr::BVar(0), Expr::BVar(1), Expr::BVar(2)];
        let chain = quote_app_chain(f, args);
        let mut cur = &chain;
        let mut count = 0usize;
        while let Expr::App(g, _) = cur {
            count += 1;
            cur = g;
        }
        assert_eq!(count, 3);
    }
    #[test]
    fn test_quote_builder_konst() {
        let builder = QuoteBuilder::new();
        let e = builder.konst(Name::str("Nat.add"));
        assert!(matches!(e, Expr::Const(n, _) if n == Name::str("Nat.add")));
    }
    #[test]
    fn test_quote_builder_resolve() {
        let mut builder = QuoteBuilder::new();
        builder.session.bind(Name::str("x"), nat_ty());
        let resolved = builder.resolve(&Name::str("x"));
        assert_eq!(resolved, Some(Expr::BVar(0)));
        assert_eq!(builder.resolve(&Name::str("y")), None);
    }
    #[test]
    fn test_quote_stats_summary() {
        let mut stats = QuoteStats::new();
        stats.record_quote();
        stats.record_quote();
        stats.record_unquote(true);
        stats.record_unquote(false);
        stats.record_match(true);
        stats.record_match(false);
        let s = stats.summary();
        assert!(s.contains("quotes=2"));
        assert!(s.contains("unquotes=1/2"));
    }
    #[test]
    fn test_quote_stats_match_hit_rate() {
        let mut stats = QuoteStats::new();
        assert!((stats.match_hit_rate() - 1.0).abs() < 1e-10);
        stats.record_match(true);
        stats.record_match(false);
        assert!((stats.match_hit_rate() - 0.5).abs() < 1e-10);
    }
    #[test]
    fn test_quote_match_result_merge() {
        let mut r1 = QuoteMatchResult::new();
        r1.bind("a", Expr::BVar(0));
        let mut r2 = QuoteMatchResult::new();
        r2.bind("b", Expr::BVar(1));
        r1.merge(r2);
        assert!(r1.get("a").is_some());
        assert!(r1.get("b").is_some());
    }
    #[test]
    fn test_quote_binding_bvar_index() {
        let b = QuoteBinding::new(Name::str("x"), Expr::BVar(0), 2);
        assert_eq!(b.bvar_index(5), 2);
    }
    #[test]
    fn test_quote_env_lookup_type() {
        let mut env = QuoteEnv::new();
        let ty = Expr::Const(Name::str("Nat"), vec![]);
        env.push(Name::str("n"), ty.clone());
        assert_eq!(env.lookup_type(&Name::str("n")), Some(&ty));
        assert_eq!(env.lookup_type(&Name::str("m")), None);
    }
    #[test]
    fn test_quote_session_open_close_scope() {
        let mut sess = QuoteSession::new();
        sess.open_scope("outer");
        assert_eq!(sess.scopes.depth(), 1);
        sess.close_scope();
        assert!(sess.scopes.is_empty());
    }
}
/// Return `true` if `expr` contains any occurrence of `BVar(idx)` at binding
/// depth `depth`.
#[allow(dead_code)]
pub fn expr_contains_bvar(expr: &Expr, idx: u32, depth: u32) -> bool {
    match expr {
        Expr::BVar(i) => *i == idx + depth,
        Expr::App(f, a) => expr_contains_bvar(f, idx, depth) || expr_contains_bvar(a, idx, depth),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            expr_contains_bvar(ty, idx, depth) || expr_contains_bvar(body, idx, depth + 1)
        }
        _ => false,
    }
}
#[cfg(test)]
mod quote_util_tests {
    use super::*;
    use crate::quote::*;
    #[test]
    fn test_expr_contains_bvar_true() {
        let e = Expr::BVar(0);
        assert!(expr_contains_bvar(&e, 0, 0));
    }
    #[test]
    fn test_expr_contains_bvar_false() {
        let e = Expr::BVar(1);
        assert!(!expr_contains_bvar(&e, 0, 0));
    }
    #[test]
    fn test_expr_contains_bvar_app() {
        let e = Expr::App(Box::new(Expr::BVar(0)), Box::new(Expr::BVar(2)));
        assert!(expr_contains_bvar(&e, 0, 0));
        assert!(expr_contains_bvar(&e, 2, 0));
        assert!(!expr_contains_bvar(&e, 1, 0));
    }
}
