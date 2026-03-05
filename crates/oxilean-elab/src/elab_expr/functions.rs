//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Expr, FVarId, Level, Literal, Name};

use super::types::{
    BuiltArg, CoercionInsert, CoercionKind, ElabConfig, ElabContext, ElabHole, ElabHoleReport,
    ElabMode, ElabResult, ElabSessionSummary, ElabStats, ElabSyntheticArg, ElabTrace, ElabTrace2,
    ElabTraceKind, ExpectedTypeFrame, ExpectedTypeStack, HoleCtx, HoleState, ImplicitArg,
    ImplicitArgQueue, MetaIdGen, PendingImplicit, SyntheticArgBuilder, UniverseVarAlloc,
};

/// Elaborate a numeric literal into a kernel `Nat` expression.
///
/// Returns `(Expr::Lit(Literal::Nat(n)), Nat_type)`.
pub fn elab_nat_lit(n: u64) -> ElabResult {
    let expr = Expr::Lit(Literal::Nat(n));
    let ty = Expr::Const(Name::str("Nat"), vec![]);
    ElabResult::new(expr, ty)
}
/// Elaborate a string literal into a kernel `String` expression.
pub fn elab_str_lit(s: &str) -> ElabResult {
    let expr = Expr::Lit(Literal::Str(s.to_string()));
    let ty = Expr::Const(Name::str("String"), vec![]);
    ElabResult::new(expr, ty)
}
/// Elaborate a boolean constant.
pub fn elab_bool_lit(b: bool) -> ElabResult {
    let name = if b { "Bool.true" } else { "Bool.false" };
    let expr = Expr::Const(Name::str(name), vec![]);
    let ty = Expr::Const(Name::str("Bool"), vec![]);
    ElabResult::new(expr, ty)
}
/// Elaborate a character literal.
pub fn elab_char_lit(c: char) -> ElabResult {
    let expr = Expr::Lit(Literal::Nat(c as u64));
    let ty = Expr::Const(Name::str("Char"), vec![]);
    ElabResult::new(expr, ty)
}
/// Parse a universe level string like "0", "1", "u+1" into a `Level`.
///
/// Returns `Level::zero()` on failure (lenient parsing for elaboration).
pub fn elab_level_str(s: &str) -> Level {
    let s = s.trim();
    if let Ok(n) = s.parse::<u32>() {
        let mut l = Level::zero();
        for _ in 0..n {
            l = Level::succ(l);
        }
        return l;
    }
    if let Some(plus_pos) = s.find('+') {
        let name_part = s[..plus_pos].trim();
        let num_part = s[plus_pos + 1..].trim();
        if !name_part.is_empty() {
            if let Ok(n) = num_part.parse::<u32>() {
                let mut l = Level::Param(Name::str(name_part));
                for _ in 0..n {
                    l = Level::succ(l);
                }
                return l;
            }
        }
    }
    if let Some(inner) = s.strip_prefix("max(").and_then(|r| r.strip_suffix(')')) {
        if let Some(comma) = inner.find(',') {
            let a = elab_level_str(inner[..comma].trim());
            let b = elab_level_str(inner[comma + 1..].trim());
            return Level::max(a, b);
        }
    }
    if let Some(inner) = s.strip_prefix("imax(").and_then(|r| r.strip_suffix(')')) {
        if let Some(comma) = inner.find(',') {
            let a = elab_level_str(inner[..comma].trim());
            let b = elab_level_str(inner[comma + 1..].trim());
            return Level::imax(a, b);
        }
    }
    if s.chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '\'')
        && !s.is_empty()
    {
        return Level::Param(Name::str(s));
    }
    Level::zero()
}
/// The sort `Prop` (universe 0).
pub fn prop_type() -> Expr {
    Expr::Sort(Level::zero())
}
/// The sort `Type 0` (universe 1 in Lean 4 terms, Sort 1).
pub fn type0() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
/// The sort `Type 1`.
pub fn type1() -> Expr {
    Expr::Sort(Level::succ(Level::succ(Level::zero())))
}
/// The sort at level `n` (0-based: n=0 → Prop, n=1 → Type 0).
pub fn type_at(n: u32) -> Expr {
    let mut lv = Level::zero();
    for _ in 0..n {
        lv = Level::succ(lv);
    }
    Expr::Sort(lv)
}
/// Build a simple non-dependent arrow `A → B`.
pub fn mk_arrow(a: Expr, b: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(a),
        Box::new(b),
    )
}
/// Build a dependent Pi type `(x : A) → B`.
pub fn mk_pi(binder: Name, info: BinderInfo, dom: Expr, cod: Expr) -> Expr {
    Expr::Pi(info, binder, Box::new(dom), Box::new(cod))
}
/// Build a lambda `fun (x : A) => body`.
pub fn mk_lam(binder: Name, info: BinderInfo, ty: Expr, body: Expr) -> Expr {
    Expr::Lam(info, binder, Box::new(ty), Box::new(body))
}
/// Build a telescope of Pi types from a list of `(name, binder_info, domain)` triples.
///
/// The innermost type is `ret`.
pub fn mk_pi_telescope(binders: &[(Name, BinderInfo, Expr)], ret: Expr) -> Expr {
    binders.iter().rev().fold(ret, |acc, (n, bi, dom)| {
        mk_pi(n.clone(), *bi, dom.clone(), acc)
    })
}
/// Build a telescope of lambdas from binders.
pub fn mk_lam_telescope(binders: &[(Name, BinderInfo, Expr)], body: Expr) -> Expr {
    binders.iter().rev().fold(body, |acc, (n, bi, ty)| {
        mk_lam(n.clone(), *bi, ty.clone(), acc)
    })
}
/// Apply `f` to a list of arguments left-associatively.
pub fn mk_app(f: Expr, args: Vec<Expr>) -> Expr {
    args.into_iter()
        .fold(f, |acc, arg| Expr::App(Box::new(acc), Box::new(arg)))
}
/// Build `Const(name) applied to args`.
pub fn mk_const_app(name: Name, levels: Vec<Level>, args: Vec<Expr>) -> Expr {
    let f = Expr::Const(name, levels);
    mk_app(f, args)
}
/// Build a two-argument application `f a b`.
pub fn mk_app2(f: Expr, a: Expr, b: Expr) -> Expr {
    let fa = Expr::App(Box::new(f), Box::new(a));
    Expr::App(Box::new(fa), Box::new(b))
}
/// Build `And A B` (propositional conjunction).
pub fn mk_and(a: Expr, b: Expr) -> Expr {
    mk_const_app(Name::str("And"), vec![], vec![a, b])
}
/// Build `Or A B` (propositional disjunction).
pub fn mk_or(a: Expr, b: Expr) -> Expr {
    mk_const_app(Name::str("Or"), vec![], vec![a, b])
}
/// Build `Not A` (negation).
pub fn mk_not(a: Expr) -> Expr {
    mk_const_app(Name::str("Not"), vec![], vec![a])
}
/// Build `Eq A a b` (propositional equality).
pub fn mk_eq(ty: Expr, lhs: Expr, rhs: Expr) -> Expr {
    mk_const_app(Name::str("Eq"), vec![Level::zero()], vec![ty, lhs, rhs])
}
/// Elaborate a hole `_` by generating a fresh metavariable expression.
///
/// Creates a fresh metavar ID using the generator and encodes it as
/// `Expr::FVar(FVarId(1_000_000 + id))`, following the convention used by
/// `ElabContext::fresh_meta`.
pub fn elab_hole(gen: &mut MetaIdGen) -> Expr {
    let id = gen.fresh();
    Expr::FVar(FVarId(1_000_000 + id))
}
/// Generate a fresh metavariable ID string.
pub fn fresh_meta_id(gen: &mut MetaIdGen) -> String {
    format!("?m{}", gen.fresh())
}
/// Collect the implicit arguments that should be auto-inserted when
/// applying a term of type `ty`. Returns a list of `ImplicitArg` in order.
pub fn collect_implicit_args(ty: &Expr) -> Vec<ImplicitArg> {
    let mut args = Vec::new();
    let mut cur = ty;
    loop {
        match cur {
            Expr::Pi(bi, n, dom, cod) if *bi != BinderInfo::Default => {
                args.push(ImplicitArg::new(n.clone(), *dom.clone(), *bi));
                cur = cod;
            }
            _ => break,
        }
    }
    args
}
/// Count the number of leading lambda binders in an expression.
pub fn count_lam_binders(expr: &Expr) -> usize {
    let mut n = 0;
    let mut cur = expr;
    while let Expr::Lam(_, _, _, body) = cur {
        n += 1;
        cur = body;
    }
    n
}
/// Count the number of leading Pi binders in an expression.
pub fn count_pi_binders(expr: &Expr) -> usize {
    let mut n = 0;
    let mut cur = expr;
    while let Expr::Pi(_, _, _, cod) = cur {
        n += 1;
        cur = cod;
    }
    n
}
/// Test whether an expression is of sort Prop (universe 0).
pub fn is_prop(expr: &Expr) -> bool {
    matches!(expr, Expr::Sort(lv) if lv.is_zero())
}
/// Test whether an expression is of sort Type at any level.
pub fn is_type(expr: &Expr) -> bool {
    matches!(expr, Expr::Sort(_))
}
/// Test whether a free variable `fvar_id` appears free in `expr`.
pub fn contains_fvar(expr: &Expr, fvar_id: u64) -> bool {
    match expr {
        Expr::FVar(id) => id.0 == fvar_id,
        Expr::App(f, a) => contains_fvar(f, fvar_id) || contains_fvar(a, fvar_id),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) | Expr::Let(_, ty, body, _) => {
            contains_fvar(ty, fvar_id) || contains_fvar(body, fvar_id)
        }
        _ => false,
    }
}
/// Collect the set of free variable IDs occurring in `expr`.
pub fn collect_fvars(expr: &Expr) -> Vec<u64> {
    let mut set = Vec::new();
    collect_fvars_rec(expr, &mut set);
    set.sort_unstable();
    set.dedup();
    set
}
fn collect_fvars_rec(expr: &Expr, out: &mut Vec<u64>) {
    match expr {
        Expr::FVar(id) => out.push(id.0),
        Expr::App(f, a) => {
            collect_fvars_rec(f, out);
            collect_fvars_rec(a, out);
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            collect_fvars_rec(ty, out);
            collect_fvars_rec(body, out);
        }
        Expr::Let(_, ty, val, body) => {
            collect_fvars_rec(ty, out);
            collect_fvars_rec(val, out);
            collect_fvars_rec(body, out);
        }
        _ => {}
    }
}
/// Compute the syntactic depth (nesting level) of an expression.
pub fn expr_depth(expr: &Expr) -> usize {
    match expr {
        Expr::App(f, a) => 1 + expr_depth(f).max(expr_depth(a)),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            1 + expr_depth(ty).max(expr_depth(body))
        }
        Expr::Let(_, ty, val, body) => {
            1 + expr_depth(ty).max(expr_depth(val)).max(expr_depth(body))
        }
        _ => 0,
    }
}
/// Test syntactic (structural) equality of two expressions.
/// Does NOT perform alpha-equivalence or beta-reduction.
pub fn syntactic_eq(a: &Expr, b: &Expr) -> bool {
    match (a, b) {
        (Expr::BVar(i), Expr::BVar(j)) => i == j,
        (Expr::FVar(i), Expr::FVar(j)) => i == j,
        (Expr::Sort(l1), Expr::Sort(l2)) => l1 == l2,
        (Expr::Const(n1, ls1), Expr::Const(n2, ls2)) => n1 == n2 && ls1 == ls2,
        (Expr::Lit(l1), Expr::Lit(l2)) => l1 == l2,
        (Expr::App(f1, a1), Expr::App(f2, a2)) => syntactic_eq(f1, f2) && syntactic_eq(a1, a2),
        (Expr::Lam(n1, bi1, ty1, b1), Expr::Lam(n2, bi2, ty2, b2)) => {
            n1 == n2 && bi1 == bi2 && syntactic_eq(ty1, ty2) && syntactic_eq(b1, b2)
        }
        (Expr::Pi(n1, bi1, d1, c1), Expr::Pi(n2, bi2, d2, c2)) => {
            n1 == n2 && bi1 == bi2 && syntactic_eq(d1, d2) && syntactic_eq(c1, c2)
        }
        _ => false,
    }
}
/// Convenience: elaborate a surface source string into a `(Expr, Expr)` pair
/// using the parse → elab pipeline with an empty environment.
///
/// Returns `None` when parsing or elaboration fails.
pub fn elab_with_env(src: &str) -> Option<ElabResult> {
    use oxilean_kernel::Environment;
    use oxilean_parse::{Lexer, Parser};
    let mut lexer = Lexer::new(src);
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let located_expr = parser.parse_expr().ok()?;
    let env = Environment::new();
    let mut ctx = crate::context::ElabContext::new(&env);
    let expr = crate::elaborate::elaborate_expr(&mut ctx, &located_expr).ok()?;
    let ty = match &expr {
        Expr::Lit(lit) => match lit {
            Literal::Nat(_) => Expr::Const(Name::str("Nat"), vec![]),
            Literal::Str(_) => Expr::Const(Name::str("String"), vec![]),
        },
        Expr::Const(_, _) => Expr::Sort(Level::zero()),
        _ => Expr::Sort(Level::zero()),
    };
    Some(ElabResult::new(expr, ty))
}
/// Test whether `expr` is a free variable.
pub fn is_fvar(expr: &Expr) -> bool {
    matches!(expr, Expr::FVar(_))
}
/// Test whether `expr` is a bound variable.
pub fn is_bvar(expr: &Expr) -> bool {
    matches!(expr, Expr::BVar(_))
}
/// Test whether `expr` is a constant (possibly applied).
pub fn head_is_const(expr: &Expr) -> bool {
    let mut cur = expr;
    loop {
        match cur {
            Expr::App(f, _) => cur = f,
            Expr::Const(_, _) => return true,
            _ => return false,
        }
    }
}
/// Extract the head constant name from a (possibly applied) expression.
pub fn head_const_name(expr: &Expr) -> Option<&Name> {
    let mut cur = expr;
    loop {
        match cur {
            Expr::App(f, _) => cur = f,
            Expr::Const(n, _) => return Some(n),
            _ => return None,
        }
    }
}
/// Count the number of arguments in a spine `f a1 a2 ... an`.
pub fn spine_arg_count(expr: &Expr) -> usize {
    let mut n = 0;
    let mut cur = expr;
    while let Expr::App(f, _) = cur {
        n += 1;
        cur = f;
    }
    n
}
/// Collect the spine of an application: `[f, a1, a2, ..., an]`.
pub fn spine(expr: &Expr) -> Vec<&Expr> {
    let mut parts = Vec::new();
    let mut cur = expr;
    while let Expr::App(f, a) = cur {
        parts.push(a.as_ref());
        cur = f;
    }
    parts.push(cur);
    parts.reverse();
    parts
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::elab_expr::*;
    #[test]
    fn test_elab_mode_display() {
        assert_eq!(format!("{}", ElabMode::Synth), "synth");
        assert_eq!(format!("{}", ElabMode::Check), "check");
    }
    #[test]
    fn test_elab_mode_predicates() {
        assert!(ElabMode::Synth.is_synth());
        assert!(!ElabMode::Synth.is_check());
        assert!(ElabMode::Check.is_check());
    }
    #[test]
    fn test_elab_nat_lit() {
        let r = elab_nat_lit(42);
        assert!(matches!(r.expr, Expr::Lit(Literal::Nat(42))));
        assert!(matches!(& r.ty, Expr::Const(n, _) if n.last_str() == Some("Nat")));
    }
    #[test]
    fn test_elab_str_lit() {
        let r = elab_str_lit("hello");
        assert!(matches!(& r.ty, Expr::Const(n, _) if n.last_str() == Some("String")));
    }
    #[test]
    fn test_elab_bool_lit_true() {
        let r = elab_bool_lit(true);
        assert!(matches!(& r.expr, Expr::Const(n, _) if n.last_str() == Some("Bool.true")));
    }
    #[test]
    fn test_elab_bool_lit_false() {
        let r = elab_bool_lit(false);
        assert!(matches!(& r.expr, Expr::Const(n, _) if n.last_str() == Some("Bool.false")));
    }
    #[test]
    fn test_prop_type() {
        let p = prop_type();
        assert!(is_prop(&p));
        assert!(is_type(&p));
    }
    #[test]
    fn test_type0_not_prop() {
        let t = type0();
        assert!(!is_prop(&t));
        assert!(is_type(&t));
    }
    #[test]
    fn test_type_at() {
        let t0 = type_at(0);
        assert!(is_prop(&t0));
        let t1 = type_at(1);
        assert!(!is_prop(&t1));
    }
    #[test]
    fn test_mk_arrow() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let arr = mk_arrow(nat.clone(), nat.clone());
        assert!(matches!(arr, Expr::Pi(BinderInfo::Default, _, _, _)));
    }
    #[test]
    fn test_mk_app() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let app = mk_app(f, vec![a, b]);
        assert!(matches!(app, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_app_empty() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let app = mk_app(f.clone(), vec![]);
        assert!(syntactic_eq(&app, &f));
    }
    #[test]
    fn test_count_lam_binders() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let body = Expr::BVar(0);
        let lam1 = mk_lam(
            Name::str("x"),
            BinderInfo::Default,
            nat.clone(),
            body.clone(),
        );
        let lam2 = mk_lam(Name::str("y"), BinderInfo::Default, nat.clone(), lam1);
        assert_eq!(count_lam_binders(&lam2), 2);
    }
    #[test]
    fn test_count_pi_binders() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let binders = vec![
            (Name::str("x"), BinderInfo::Default, nat.clone()),
            (Name::str("y"), BinderInfo::Default, nat.clone()),
        ];
        let pi = mk_pi_telescope(&binders, prop_type());
        assert_eq!(count_pi_binders(&pi), 2);
    }
    #[test]
    fn test_contains_fvar_true() {
        let e = Expr::FVar(FVarId(3));
        assert!(contains_fvar(&e, 3));
        assert!(!contains_fvar(&e, 7));
    }
    #[test]
    fn test_collect_fvars() {
        let a = Expr::FVar(FVarId(1));
        let b = Expr::FVar(FVarId(2));
        let app = Expr::App(Box::new(a), Box::new(b));
        let fvars = collect_fvars(&app);
        assert_eq!(fvars, vec![1, 2]);
    }
    #[test]
    fn test_expr_depth_const() {
        let e = Expr::Const(Name::str("Nat"), vec![]);
        assert_eq!(expr_depth(&e), 0);
    }
    #[test]
    fn test_expr_depth_app() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Const(Name::str("a"), vec![]);
        let app = Expr::App(Box::new(f), Box::new(a));
        assert_eq!(expr_depth(&app), 1);
    }
    #[test]
    fn test_syntactic_eq_same() {
        let e = Expr::Const(Name::str("Nat"), vec![]);
        assert!(syntactic_eq(&e, &e));
    }
    #[test]
    fn test_syntactic_eq_diff() {
        let a = Expr::Const(Name::str("Nat"), vec![]);
        let b = Expr::Const(Name::str("Int"), vec![]);
        assert!(!syntactic_eq(&a, &b));
    }
    #[test]
    fn test_collect_implicit_args_none() {
        let e = Expr::Const(Name::str("Nat"), vec![]);
        let args = collect_implicit_args(&e);
        assert!(args.is_empty());
    }
    #[test]
    fn test_collect_implicit_args_one() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let pi = mk_pi(Name::str("A"), BinderInfo::Implicit, nat, prop_type());
        let args = collect_implicit_args(&pi);
        assert_eq!(args.len(), 1);
        assert_eq!(args[0].info, BinderInfo::Implicit);
    }
    #[test]
    fn test_meta_id_gen() {
        let mut gen = MetaIdGen::new();
        assert_eq!(gen.fresh(), 0);
        assert_eq!(gen.fresh(), 1);
        assert_eq!(gen.peek(), 2);
    }
    #[test]
    fn test_fresh_meta_id_format() {
        let mut gen = MetaIdGen::new();
        let id = fresh_meta_id(&mut gen);
        assert!(id.starts_with("?m"));
    }
    #[test]
    fn test_is_fvar() {
        assert!(is_fvar(&Expr::FVar(FVarId(0))));
        assert!(!is_fvar(&Expr::BVar(0)));
    }
    #[test]
    fn test_is_bvar() {
        assert!(is_bvar(&Expr::BVar(0)));
        assert!(!is_bvar(&Expr::FVar(FVarId(0))));
    }
    #[test]
    fn test_head_const_name() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Const(Name::str("a"), vec![]);
        let app = Expr::App(Box::new(f), Box::new(a));
        let n = head_const_name(&app).expect("test operation should succeed");
        assert_eq!(n.last_str(), Some("f"));
    }
    #[test]
    fn test_head_is_const() {
        let f = Expr::Const(Name::str("f"), vec![]);
        assert!(head_is_const(&f));
        let bv = Expr::BVar(0);
        assert!(!head_is_const(&bv));
    }
    #[test]
    fn test_spine_arg_count() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Const(Name::str("a"), vec![]);
        let app = mk_app(f, vec![a.clone(), a.clone(), a]);
        assert_eq!(spine_arg_count(&app), 3);
    }
    #[test]
    fn test_spine() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Const(Name::str("a"), vec![]);
        let app = mk_app(f, vec![a.clone(), a.clone()]);
        let s = spine(&app);
        assert_eq!(s.len(), 3);
    }
    #[test]
    fn test_coercion_insert_numeric() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let int = Expr::Const(Name::str("Int"), vec![]);
        let expr = Expr::Lit(Literal::Nat(1));
        let ci = CoercionInsert::new(expr, nat, int, CoercionKind::NatToInt);
        assert!(ci.is_numeric());
    }
    #[test]
    fn test_elab_result_into_pair() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let lit = Expr::Lit(Literal::Nat(0));
        let r = ElabResult::new(lit, nat);
        let (e, t) = r.into_pair();
        assert!(matches!(e, Expr::Lit(_)));
        assert!(matches!(t, Expr::Const(_, _)));
    }
    #[test]
    fn test_elab_trace_enabled() {
        let mut trace = ElabTrace::enabled();
        assert!(trace.enabled);
        let e = Expr::BVar(0);
        trace.record("test", &e, ElabMode::Synth);
        assert_eq!(trace.len(), 1);
    }
    #[test]
    fn test_elab_trace_disabled() {
        let mut trace = ElabTrace::new();
        let e = Expr::BVar(0);
        trace.record("test", &e, ElabMode::Synth);
        assert_eq!(trace.len(), 0);
    }
    #[test]
    fn test_elab_trace_display() {
        let mut trace = ElabTrace::enabled();
        let e = Expr::BVar(0);
        trace.record("step", &e, ElabMode::Check);
        let s = format!("{}", trace);
        assert!(s.contains("step"));
    }
    #[test]
    fn test_mk_and() {
        let a = prop_type();
        let b = prop_type();
        let and = mk_and(a, b);
        assert!(head_is_const(&and));
        assert_eq!(
            head_const_name(&and)
                .expect("test operation should succeed")
                .last_str(),
            Some("And")
        );
    }
    #[test]
    fn test_mk_not() {
        let a = prop_type();
        let not_a = mk_not(a);
        assert!(head_is_const(&not_a));
    }
    #[test]
    fn test_level_str_zero() {
        let lv = elab_level_str("0");
        assert!(lv.is_zero());
    }
    #[test]
    fn test_level_str_one() {
        let lv = elab_level_str("1");
        assert!(!lv.is_zero());
    }
    #[test]
    fn test_mk_pi_telescope_empty() {
        let ret = prop_type();
        let pi = mk_pi_telescope(&[], ret.clone());
        assert!(syntactic_eq(&pi, &ret));
    }
    #[test]
    fn test_mk_lam_telescope_single() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let body = Expr::BVar(0);
        let binders = vec![(Name::str("x"), BinderInfo::Default, nat)];
        let lam = mk_lam_telescope(&binders, body);
        assert!(matches!(lam, Expr::Lam(_, _, _, _)));
    }
    #[test]
    fn test_elab_with_env_nat_literal() {
        let result = elab_with_env("42");
        assert!(result.is_some(), "expected Some but got None");
        let r = result.expect("test operation should succeed");
        assert!(matches!(r.expr, Expr::Lit(_)));
    }
    #[test]
    fn test_elab_with_env_invalid_returns_none() {
        assert!(elab_with_env("!!!").is_none());
    }
    #[test]
    fn test_implicit_arg_is_instance() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let arg = ImplicitArg::new(Name::str("inst"), nat, BinderInfo::InstImplicit);
        assert!(arg.is_instance());
        assert!(!arg.is_strict());
    }
    #[test]
    fn test_implicit_arg_is_strict() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let arg = ImplicitArg::new(Name::str("A"), nat, BinderInfo::StrictImplicit);
        assert!(arg.is_strict());
    }
    #[test]
    fn test_elabsynthetic_arg_with_span() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let arg = ElabSyntheticArg::new(Name::str("x"), nat, BinderInfo::Default).with_span(0, 10);
        assert_eq!(arg.span, Some((0, 10)));
    }
    #[test]
    fn test_mk_eq() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let z = Expr::Lit(Literal::Nat(0));
        let eq = mk_eq(nat, z.clone(), z);
        assert!(head_is_const(&eq));
        assert_eq!(
            head_const_name(&eq)
                .expect("test operation should succeed")
                .last_str(),
            Some("Eq")
        );
    }
    #[test]
    fn test_elab_char_lit() {
        let r = elab_char_lit('A');
        assert!(matches!(r.expr, Expr::Lit(Literal::Nat(65))));
        assert!(matches!(& r.ty, Expr::Const(n, _) if n.last_str() == Some("Char")));
    }
    #[test]
    fn test_mk_app2() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let app = mk_app2(f, a, b);
        assert_eq!(spine_arg_count(&app), 2);
    }
    #[test]
    fn test_mk_const_app_no_args() {
        let e = mk_const_app(Name::str("Nat"), vec![], vec![]);
        assert!(matches!(e, Expr::Const(_, _)));
    }
    #[test]
    fn test_mk_or() {
        let a = prop_type();
        let b = prop_type();
        let or = mk_or(a, b);
        assert_eq!(
            head_const_name(&or)
                .expect("test operation should succeed")
                .last_str(),
            Some("Or")
        );
    }
}
#[cfg(test)]
mod hole_and_trace_tests {
    use super::*;
    use crate::elab_expr::*;
    #[test]
    fn test_hole_state() {
        assert!(HoleState::Unassigned.is_unassigned());
        assert!(!HoleState::Assigned.is_unassigned());
        assert!(!HoleState::Assigned.is_unassigned());
        assert!(HoleState::Assigned.is_assigned());
        assert!(!HoleState::Unassigned.is_assigned());
    }
    #[test]
    fn test_hole_ctx_lifecycle() {
        let mut hole = HoleCtx::new(0, Name::str("h"), Expr::Const(Name::str("Nat"), vec![]), 2);
        assert!(hole.is_pending());
        hole.assign();
        assert!(hole.state.is_assigned());
        assert!(!hole.is_pending());
    }
    #[test]
    fn test_hole_ctx_reject() {
        let mut hole = HoleCtx::new(1, Name::str("h"), Expr::Const(Name::str("Bool"), vec![]), 0);
        hole.reject("type mismatch");
        assert!(matches!(hole.state, HoleState::Rejected(_)));
    }
    #[test]
    fn test_elab_hole_alloc_assign() {
        let mut env = ElabHole::new();
        assert!(env.is_empty());
        let id0 = env.alloc(Name::str("h0"), Expr::Const(Name::str("Nat"), vec![]), 0);
        let id1 = env.alloc(Name::str("h1"), Expr::Const(Name::str("Bool"), vec![]), 1);
        assert_eq!(env.len(), 2);
        assert_eq!(env.unassigned_count(), 2);
        env.assign(id0);
        assert_eq!(env.unassigned_count(), 1);
        assert!(env.get(id0).expect("key should exist").state.is_assigned());
        assert!(env.get(id1).expect("key should exist").is_pending());
    }
    #[test]
    fn test_elab_hole_all_assigned() {
        let mut env = ElabHole::new();
        let id = env.alloc(Name::str("h"), Expr::Const(Name::str("Nat"), vec![]), 0);
        assert!(!env.all_assigned());
        env.assign(id);
        assert!(env.all_assigned());
    }
    #[test]
    fn test_elab_hole_unassigned_list() {
        let mut env = ElabHole::new();
        let id0 = env.alloc(Name::str("h0"), Expr::Const(Name::str("Nat"), vec![]), 0);
        env.alloc(Name::str("h1"), Expr::Const(Name::str("Bool"), vec![]), 0);
        env.assign(id0);
        let pending = env.unassigned_holes();
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].name, Name::str("h1"));
    }
    #[test]
    fn test_universe_var_alloc() {
        let mut alloc = UniverseVarAlloc::new();
        assert!(alloc.is_empty());
        let v0 = alloc.alloc();
        assert_eq!(v0.id, 0);
        assert_eq!(v0.name, "u0");
        let v1 = alloc.alloc();
        assert_eq!(v1.id, 1);
        assert_eq!(v1.name, "u1");
        assert_eq!(alloc.len(), 2);
        assert!(alloc.get(0).is_some());
        assert!(alloc.get(99).is_none());
    }
    #[test]
    fn test_elab_trace_disabled() {
        let mut trace = ElabTrace2::new();
        trace.record(ElabTraceKind::HoleAlloc {
            hole_id: 0,
            name: "h".to_string(),
        });
        assert!(trace.is_empty());
    }
    #[test]
    fn test_elab_trace_enabled() {
        let mut trace = ElabTrace2::enabled();
        trace.record(ElabTraceKind::HoleAlloc {
            hole_id: 0,
            name: "h0".to_string(),
        });
        trace.record(ElabTraceKind::ImplicitInsert {
            arg_name: "inst".to_string(),
        });
        trace.record(ElabTraceKind::ImplicitInsert {
            arg_name: "alpha".to_string(),
        });
        assert_eq!(trace.len(), 3);
        assert_eq!(trace.count_hole_allocs(), 1);
        assert_eq!(trace.count_implicit_inserts(), 2);
    }
    #[test]
    fn test_elab_trace_depth() {
        let mut trace = ElabTrace2::enabled();
        trace.enter();
        trace.enter();
        trace.record(ElabTraceKind::HoleAlloc {
            hole_id: 0,
            name: "h".to_string(),
        });
        let entry = &trace.entries()[0];
        assert_eq!(entry.depth, 2);
        trace.exit();
        trace.record(ElabTraceKind::HoleAssign { hole_id: 0 });
        let entry2 = &trace.entries()[1];
        assert_eq!(entry2.depth, 1);
    }
    #[test]
    fn test_elab_trace_clear() {
        let mut trace = ElabTrace2::enabled();
        trace.enter();
        trace.record(ElabTraceKind::UnivAlloc { univ_id: 0 });
        assert_eq!(trace.len(), 1);
        trace.clear();
        assert!(trace.is_empty());
    }
}
#[cfg(test)]
mod implicit_queue_tests {
    use super::*;
    use crate::elab_expr::*;
    #[test]
    fn test_pending_implicit_basic() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let pi = PendingImplicit::new(Name::str("alpha"), nat, BinderInfo::Implicit);
        assert!(!pi.is_strict());
        assert!(!pi.is_instance());
        assert!(pi.fvar_id.is_none());
    }
    #[test]
    fn test_pending_implicit_strict() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let pi = PendingImplicit::new(Name::str("alpha"), nat, BinderInfo::StrictImplicit);
        assert!(pi.is_strict());
    }
    #[test]
    fn test_pending_implicit_instance() {
        let nat = Expr::Const(Name::str("Inhabited"), vec![]);
        let pi = PendingImplicit::new(Name::str("inst"), nat, BinderInfo::InstImplicit);
        assert!(pi.is_instance());
    }
    #[test]
    fn test_pending_implicit_with_fvar() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let fvar_id = FVarId::new(42);
        let pi = PendingImplicit::new(Name::str("x"), nat, BinderInfo::Implicit).with_fvar(fvar_id);
        assert_eq!(pi.fvar_id, Some(fvar_id));
    }
    #[test]
    fn test_implicit_arg_queue_basic() {
        let mut queue = ImplicitArgQueue::new();
        assert!(queue.is_empty());
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        queue.push(PendingImplicit::new(
            Name::str("a"),
            nat.clone(),
            BinderInfo::Implicit,
        ));
        queue.push(PendingImplicit::new(
            Name::str("inst"),
            nat.clone(),
            BinderInfo::InstImplicit,
        ));
        queue.push(PendingImplicit::new(
            Name::str("b"),
            nat,
            BinderInfo::StrictImplicit,
        ));
        assert_eq!(queue.len(), 3);
        assert_eq!(queue.count_instances(), 1);
        assert_eq!(queue.count_strict(), 1);
        let first = queue.pop().expect("collection should not be empty");
        assert_eq!(first.name, Name::str("a"));
        assert_eq!(queue.len(), 2);
    }
    #[test]
    fn test_implicit_arg_queue_drain() {
        let mut queue = ImplicitArgQueue::new();
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        queue.push(PendingImplicit::new(
            Name::str("a"),
            nat.clone(),
            BinderInfo::Implicit,
        ));
        queue.push(PendingImplicit::new(
            Name::str("b"),
            nat,
            BinderInfo::Implicit,
        ));
        let drained = queue.drain();
        assert_eq!(drained.len(), 2);
        assert!(queue.is_empty());
    }
    #[test]
    fn test_elab_context_mode_switching() {
        let mut ctx = ElabContext::new();
        assert_eq!(ctx.mode, ElabMode::Synth);
        assert!(!ctx.in_check_mode());
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        ctx.push_expected_ty(nat.clone());
        assert!(ctx.in_check_mode());
        assert_eq!(ctx.depth, 1);
        ctx.push_expected_ty(nat);
        assert_eq!(ctx.depth, 2);
        ctx.pop_expected_ty();
        assert_eq!(ctx.depth, 1);
        ctx.pop_expected_ty();
        assert!(!ctx.in_check_mode());
        assert_eq!(ctx.depth, 0);
    }
    #[test]
    fn test_elab_context_positions() {
        let mut ctx = ElabContext::new();
        ctx.push_position(1, 5);
        ctx.push_position(2, 10);
        assert_eq!(ctx.pop_position(), Some((2, 10)));
        assert_eq!(ctx.pop_position(), Some((1, 5)));
        assert!(ctx.pop_position().is_none());
    }
    #[test]
    fn test_elab_context_current_expected_ty() {
        let mut ctx = ElabContext::new();
        assert!(ctx.current_expected_ty().is_none());
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        ctx.push_expected_ty(nat.clone());
        assert!(ctx.current_expected_ty().is_some());
    }
}
#[cfg(test)]
mod stats_config_tests {
    use super::*;
    use crate::elab_expr::*;
    #[test]
    fn test_elab_stats_basic() {
        let mut stats = ElabStats::new();
        stats.holes_allocated = 10;
        stats.holes_assigned = 8;
        stats.exprs_elaborated = 25;
        stats.implicits_inserted = 3;
        stats.coercions_applied = 1;
        stats.max_depth = 12;
        assert!((stats.hole_fill_rate() - 0.8).abs() < 1e-9);
        assert!(!stats.all_holes_filled());
    }
    #[test]
    fn test_elab_stats_all_holes_filled() {
        let mut stats = ElabStats::new();
        stats.holes_allocated = 5;
        stats.holes_assigned = 5;
        assert!(stats.all_holes_filled());
        assert!((stats.hole_fill_rate() - 1.0).abs() < 1e-9);
    }
    #[test]
    fn test_elab_stats_empty() {
        let stats = ElabStats::new();
        assert_eq!(stats.hole_fill_rate(), 1.0);
        assert!(stats.all_holes_filled());
    }
    #[test]
    fn test_elab_stats_merge() {
        let mut a = ElabStats::new();
        a.holes_allocated = 5;
        a.max_depth = 3;
        let mut b = ElabStats::new();
        b.holes_allocated = 7;
        b.max_depth = 10;
        a.merge(&b);
        assert_eq!(a.holes_allocated, 12);
        assert_eq!(a.max_depth, 10);
    }
    #[test]
    fn test_elab_config_default() {
        let config = ElabConfig::default();
        assert!(config.insert_implicits);
        assert!(config.infer_universes);
        assert!(config.auto_coercions);
        assert!(config.allow_sorry);
        assert!(!config.trace_enabled);
        assert_eq!(config.max_depth, 256);
    }
    #[test]
    fn test_elab_config_strict() {
        let config = ElabConfig::strict();
        assert!(!config.allow_sorry);
        assert!(!config.auto_coercions);
        assert!(config.insert_implicits);
    }
    #[test]
    fn test_elab_config_debug() {
        let config = ElabConfig::debug();
        assert!(config.trace_enabled);
        assert!(config.allow_sorry);
    }
    #[test]
    fn test_hole_state_irrelevant() {
        let mut hole = HoleCtx::new(
            42,
            Name::str("h"),
            Expr::Const(Name::str("Prop"), vec![]),
            0,
        );
        hole.mark_irrelevant();
        assert!(matches!(hole.state, HoleState::Irrelevant));
        assert!(!hole.is_pending());
    }
    #[test]
    fn test_universe_var_alloc_names() {
        let mut alloc = UniverseVarAlloc::new();
        for i in 0..5 {
            let v = alloc.alloc();
            assert_eq!(v.name, format!("u{}", i));
        }
        assert_eq!(alloc.len(), 5);
        let all = alloc.all_vars();
        assert_eq!(all.len(), 5);
        assert_eq!(all[0].name, "u0");
        assert_eq!(all[4].name, "u4");
    }
    #[test]
    fn test_elab_mode_display() {
        assert_eq!(format!("{}", ElabMode::Synth), "synth");
        assert_eq!(format!("{}", ElabMode::Check), "check");
    }
}
#[cfg(test)]
mod expected_type_stack_tests {
    use super::*;
    use crate::elab_expr::*;
    #[test]
    fn test_expected_type_stack_push_pop() {
        let mut stack = ExpectedTypeStack::new();
        assert!(stack.is_empty());
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let bool_expr = Expr::Const(Name::str("Bool"), vec![]);
        stack.push(ExpectedTypeFrame::new(nat.clone()));
        stack.push(ExpectedTypeFrame::new(bool_expr.clone()));
        assert_eq!(stack.depth(), 2);
        let top = stack.top().expect("test operation should succeed");
        assert!(matches!(& top.ty, Expr::Const(n, _) if n.last_str() == Some("Bool")));
        let popped = stack.pop().expect("collection should not be empty");
        assert!(matches!(& popped.ty, Expr::Const(n, _) if n.last_str() == Some("Bool")));
        assert_eq!(stack.depth(), 1);
    }
    #[test]
    fn test_expected_type_frame_from_implicit() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let frame = ExpectedTypeFrame::from_implicit_inference(nat).with_location(3, 7);
        assert!(frame.from_implicit);
        assert_eq!(frame.location, Some((3, 7)));
    }
    #[test]
    fn test_expected_type_stack_empty_pop() {
        let mut stack = ExpectedTypeStack::new();
        assert!(stack.pop().is_none());
        assert!(stack.top().is_none());
    }
}
#[cfg(test)]
mod synthetic_arg_tests {
    use super::*;
    use crate::elab_expr::*;
    #[test]
    fn test_built_arg_explicit() {
        let e = Expr::Const(Name::str("x"), vec![]);
        let arg = BuiltArg::Explicit(e.clone());
        assert!(arg.is_explicit());
        assert!(!arg.is_implicit());
    }
    #[test]
    fn test_built_arg_implicit() {
        let ty = Expr::Const(Name::str("Nat"), vec![]);
        let arg = BuiltArg::Implicit { hole_id: 42, ty };
        assert!(arg.is_implicit());
        assert!(!arg.is_explicit());
    }
    #[test]
    fn test_synthetic_arg_builder() {
        let mut builder = SyntheticArgBuilder::new();
        assert!(builder.is_empty());
        let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
        builder.push_explicit(Expr::Const(Name::str("x"), vec![]));
        builder.push_implicit(0, nat_ty.clone());
        builder.push_implicit(1, nat_ty);
        builder.push_explicit(Expr::Const(Name::str("y"), vec![]));
        assert_eq!(builder.len(), 4);
        assert_eq!(builder.explicit_count(), 2);
        assert_eq!(builder.implicit_count(), 2);
        let args = builder.build();
        assert_eq!(args.len(), 4);
    }
    #[test]
    fn test_elab_result_into_pair() {
        let expr = Expr::Const(Name::str("x"), vec![]);
        let ty = Expr::Const(Name::str("Nat"), vec![]);
        let result = ElabResult::new(expr.clone(), ty.clone());
        let (e, t) = result.into_pair();
        assert!(matches!(e, Expr::Const(n, _) if n.last_str() == Some("x")));
        assert!(matches!(t, Expr::Const(n, _) if n.last_str() == Some("Nat")));
    }
}
#[cfg(test)]
mod session_summary_tests {
    use super::*;
    use crate::elab_expr::*;
    #[test]
    fn test_elab_session_summary() {
        let mut summary = ElabSessionSummary::new();
        assert!(summary.is_clean());
        summary.record_declaration();
        summary.record_declaration();
        summary.record_sorry();
        summary.record_type_error();
        assert_eq!(summary.declarations, 2);
        assert_eq!(summary.sorry_count, 1);
        assert_eq!(summary.type_errors, 1);
        assert!(!summary.is_clean());
    }
    #[test]
    fn test_elab_session_summary_empty() {
        let summary = ElabSessionSummary::new();
        assert!(summary.is_clean());
        assert_eq!(summary.declarations, 0);
    }
}
#[cfg(test)]
mod hole_report_tests {
    use super::*;
    use crate::elab_expr::*;
    #[test]
    fn test_elab_hole_report() {
        let mut report = ElabHoleReport::new();
        assert!(report.is_empty());
        report.add("h0", "Nat", 0);
        report.add("h1", "Bool", 2);
        assert_eq!(report.len(), 2);
        let diag = report.diagnostic();
        assert!(diag.contains("2 unresolved"));
        assert!(diag.contains("?h0"));
        assert!(diag.contains("Nat"));
    }
    #[test]
    fn test_elab_hole_report_empty_diagnostic() {
        let report = ElabHoleReport::new();
        assert!(report.diagnostic().contains("No unresolved"));
    }
}
