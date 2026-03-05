//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    ConvenienceAnalysisPass, ConvenienceConfig, ConvenienceConfigValue, ConvenienceDiagnostics,
    ConvenienceDiff, ConvenienceExtConfig4000, ConvenienceExtConfigVal4000, ConvenienceExtDiag4000,
    ConvenienceExtDiff4000, ConvenienceExtPass4000, ConvenienceExtPipeline4000,
    ConvenienceExtResult4000, ConveniencePipeline, ConvenienceResult, MetaConvenienceBuilder,
    MetaConvenienceCounterMap, MetaConvenienceExtMap, MetaConvenienceExtUtil,
    MetaConvenienceStateMachine, MetaConvenienceWindow, MetaConvenienceWorkQueue,
};
use oxilean_kernel::{BinderInfo, Expr, Literal, Name};

/// Construct a `Const` (global name reference)
pub fn mk_const(name: &str) -> Expr {
    Expr::Const(Name::str(name), vec![])
}
/// Construct an `App(f, a)`
pub fn mk_app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
/// Apply a function to multiple arguments: `f a0 a1 a2 ...`
pub fn mk_app_n(f: Expr, args: Vec<Expr>) -> Expr {
    args.into_iter().fold(f, mk_app)
}
/// Construct a `Pi(binder_name, ty, body)` — explicit binder
pub fn mk_pi(name: &str, ty: Expr, body: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str(name),
        Box::new(ty),
        Box::new(body),
    )
}
/// Construct an implicit `Pi {name : ty} -> body`
pub fn mk_pi_implicit(name: &str, ty: Expr, body: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str(name),
        Box::new(ty),
        Box::new(body),
    )
}
/// Construct a `Lam(name, ty, body)` — explicit binder
pub fn mk_lam(name: &str, ty: Expr, body: Expr) -> Expr {
    Expr::Lam(
        BinderInfo::Default,
        Name::str(name),
        Box::new(ty),
        Box::new(body),
    )
}
/// Construct an implicit `Lam {name : ty} => body`
pub fn mk_lam_implicit(name: &str, ty: Expr, body: Expr) -> Expr {
    Expr::Lam(
        BinderInfo::Implicit,
        Name::str(name),
        Box::new(ty),
        Box::new(body),
    )
}
/// `A → B` (non-dependent Pi with anonymous binder)
pub fn mk_arrow(a: Expr, b: Expr) -> Expr {
    mk_pi("_", a, b)
}
/// Construct `Sort 0` (Prop)
pub fn mk_prop() -> Expr {
    Expr::Sort(oxilean_kernel::Level::Zero)
}
/// Construct `Sort 1` (Type 0)
pub fn mk_type0() -> Expr {
    Expr::Sort(oxilean_kernel::Level::succ(oxilean_kernel::Level::Zero))
}
/// Construct a natural number literal
pub fn mk_nat_lit(n: u64) -> Expr {
    Expr::Lit(Literal::Nat(n))
}
/// Construct a string literal
pub fn mk_str_lit(s: &str) -> Expr {
    Expr::Lit(Literal::Str(s.to_string()))
}
/// Construct `BVar(i)`
pub fn mk_bvar(i: u32) -> Expr {
    Expr::BVar(i)
}
/// `And A B` = `Expr::App(App(Const("And"), A), B)`
pub fn mk_and(a: Expr, b: Expr) -> Expr {
    mk_app(mk_app(mk_const("And"), a), b)
}
/// `Or A B`
pub fn mk_or(a: Expr, b: Expr) -> Expr {
    mk_app(mk_app(mk_const("Or"), a), b)
}
/// `Not A` = `App(Const("Not"), A)`
pub fn mk_not(a: Expr) -> Expr {
    mk_app(mk_const("Not"), a)
}
/// `Iff A B`
pub fn mk_iff(a: Expr, b: Expr) -> Expr {
    mk_app(mk_app(mk_const("Iff"), a), b)
}
/// `Eq T a b`
pub fn mk_eq(ty: Expr, a: Expr, b: Expr) -> Expr {
    mk_app(mk_app(mk_app(mk_const("Eq"), ty), a), b)
}
/// `Exists (fun x : ty => body)`
pub fn mk_exists(ty: Expr, body: Expr) -> Expr {
    mk_app(mk_const("Exists"), mk_lam("x", ty, body))
}
/// `True`
pub fn mk_true() -> Expr {
    mk_const("True")
}
/// `False`
pub fn mk_false() -> Expr {
    mk_const("False")
}
/// `Nat`
pub fn mk_nat() -> Expr {
    mk_const("Nat")
}
/// `Bool`
pub fn mk_bool() -> Expr {
    mk_const("Bool")
}
/// If `e` is `App(f, a)`, return `Some((f, a))`
pub fn as_app(e: &Expr) -> Option<(&Expr, &Expr)> {
    match e {
        Expr::App(f, a) => Some((f, a)),
        _ => None,
    }
}
/// Collect all args: `f a0 a1 a2` → `(f, [a0, a1, a2])`
pub fn collect_apps(e: &Expr) -> (&Expr, Vec<&Expr>) {
    let mut args = vec![];
    let mut cur = e;
    while let Some((f, a)) = as_app(cur) {
        args.push(a);
        cur = f;
    }
    args.reverse();
    (cur, args)
}
/// If `e` is `Pi(info, n, ty, body)`, return `Some((n, info, ty, body))`
pub fn as_pi(e: &Expr) -> Option<(&Name, BinderInfo, &Expr, &Expr)> {
    match e {
        Expr::Pi(bi, n, ty, body) => Some((n, *bi, ty, body)),
        _ => None,
    }
}
/// If `e` is `Lam(info, n, ty, body)`, return `Some((n, info, ty, body))`
pub fn as_lam(e: &Expr) -> Option<(&Name, BinderInfo, &Expr, &Expr)> {
    match e {
        Expr::Lam(bi, n, ty, body) => Some((n, *bi, ty, body)),
        _ => None,
    }
}
/// If `e` is `Const(n, _)`, return Some(n)
pub fn as_const(e: &Expr) -> Option<&Name> {
    match e {
        Expr::Const(n, _) => Some(n),
        _ => None,
    }
}
/// Count nested Pi binders
pub fn pi_arity(e: &Expr) -> usize {
    let mut count = 0;
    let mut cur = e;
    while let Some((_, _, _, body)) = as_pi(cur) {
        count += 1;
        cur = body;
    }
    count
}
/// Count nested Lam binders
pub fn lam_arity(e: &Expr) -> usize {
    let mut count = 0;
    let mut cur = e;
    while let Some((_, _, _, body)) = as_lam(cur) {
        count += 1;
        cur = body;
    }
    count
}
/// Is the expression a Sort (any universe level)?
pub fn is_sort(e: &Expr) -> bool {
    matches!(e, Expr::Sort(_))
}
/// Is the expression a Prop (Sort 0)?
pub fn is_prop(e: &Expr) -> bool {
    matches!(e, Expr::Sort(oxilean_kernel::Level::Zero))
}
/// Is the expression a bound variable?
pub fn is_bvar(e: &Expr) -> bool {
    matches!(e, Expr::BVar(_))
}
/// Does the expression contain any BVar at index i?
pub fn has_bvar(e: &Expr, i: u32) -> bool {
    match e {
        Expr::BVar(j) => *j == i,
        Expr::App(f, a) => has_bvar(f, i) || has_bvar(a, i),
        Expr::Lam(_, _, ty, b) => has_bvar(ty, i) || has_bvar(b, i + 1),
        Expr::Pi(_, _, ty, b) => has_bvar(ty, i) || has_bvar(b, i + 1),
        Expr::Let(_, ty, val, b) => has_bvar(ty, i) || has_bvar(val, i) || has_bvar(b, i + 1),
        _ => false,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::convenience::*;
    #[test]
    fn test_mk_const() {
        let e = mk_const("Nat");
        assert!(matches!(e, Expr::Const(_, _)));
        assert_eq!(as_const(&e).map(|n| n.to_string()), Some("Nat".to_string()));
    }
    #[test]
    fn test_mk_app() {
        let f = mk_const("f");
        let a = mk_const("a");
        let app = mk_app(f, a);
        let (head, args) = collect_apps(&app);
        assert_eq!(as_const(head).map(|n| n.to_string()), Some("f".to_string()));
        assert_eq!(args.len(), 1);
    }
    #[test]
    fn test_mk_app_n() {
        let f = mk_const("f");
        let args = vec![mk_const("a"), mk_const("b"), mk_const("c")];
        let app = mk_app_n(f, args);
        let (head, collected_args) = collect_apps(&app);
        assert_eq!(as_const(head).map(|n| n.to_string()), Some("f".to_string()));
        assert_eq!(collected_args.len(), 3);
    }
    #[test]
    fn test_mk_pi() {
        let ty = mk_nat();
        let body = mk_bvar(0);
        let pi = mk_pi("x", ty, body);
        let (name, bi, _, _) = as_pi(&pi).expect("value should be present");
        assert_eq!(name.to_string(), "x");
        assert_eq!(bi, BinderInfo::Default);
    }
    #[test]
    fn test_mk_lam() {
        let ty = mk_nat();
        let body = mk_bvar(0);
        let lam = mk_lam("x", ty, body);
        let (name, bi, _, _) = as_lam(&lam).expect("value should be present");
        assert_eq!(name.to_string(), "x");
        assert_eq!(bi, BinderInfo::Default);
    }
    #[test]
    fn test_mk_arrow() {
        let a = mk_nat();
        let b = mk_nat();
        let arr = mk_arrow(a, b);
        let (name, _, _, _) = as_pi(&arr).expect("value should be present");
        assert_eq!(name.to_string(), "_");
    }
    #[test]
    fn test_mk_prop() {
        let p = mk_prop();
        assert!(is_sort(&p));
        assert!(is_prop(&p));
    }
    #[test]
    fn test_collect_apps() {
        let e = mk_app_n(
            mk_const("f"),
            vec![mk_const("a"), mk_const("b"), mk_const("c")],
        );
        let (head, args) = collect_apps(&e);
        assert_eq!(as_const(head).map(|n| n.to_string()), Some("f".to_string()));
        assert_eq!(args.len(), 3);
        assert_eq!(
            as_const(args[0]).map(|n| n.to_string()),
            Some("a".to_string())
        );
        assert_eq!(
            as_const(args[2]).map(|n| n.to_string()),
            Some("c".to_string())
        );
    }
    #[test]
    fn test_as_app() {
        let app = mk_app(mk_const("f"), mk_const("a"));
        let result = as_app(&app);
        assert!(result.is_some());
        let c = mk_const("f");
        assert!(as_app(&c).is_none());
    }
    #[test]
    fn test_has_bvar() {
        let bv = mk_bvar(0);
        assert!(has_bvar(&bv, 0));
        assert!(!has_bvar(&bv, 1));
        let c = mk_const("Nat");
        assert!(!has_bvar(&c, 0));
        let lam = mk_lam("x", mk_nat(), mk_bvar(0));
        assert!(!has_bvar(&lam, 0));
        let app = mk_app(mk_bvar(0), mk_bvar(1));
        assert!(has_bvar(&app, 0));
        assert!(has_bvar(&app, 1));
        assert!(!has_bvar(&app, 2));
    }
}
/// Construct a `Let(name, ty, val, body)` binding
#[allow(dead_code)]
pub fn mk_let(name: &str, ty: Expr, val: Expr, body: Expr) -> Expr {
    Expr::Let(Name::str(name), Box::new(ty), Box::new(val), Box::new(body))
}
/// `Iff A B` — propositional iff
#[allow(dead_code)]
pub fn mk_iff_prop(a: Expr, b: Expr) -> Expr {
    mk_app(mk_app(mk_const("Iff"), a), b)
}
/// `Int` — the integer type
#[allow(dead_code)]
pub fn mk_int() -> Expr {
    mk_const("Int")
}
/// `String` — the string type
#[allow(dead_code)]
pub fn mk_string_type() -> Expr {
    mk_const("String")
}
/// `List T` — parametric list type
#[allow(dead_code)]
pub fn mk_list(ty: Expr) -> Expr {
    mk_app(mk_const("List"), ty)
}
/// `Option T` — parametric option type
#[allow(dead_code)]
pub fn mk_option(ty: Expr) -> Expr {
    mk_app(mk_const("Option"), ty)
}
/// `Prod A B` (Sigma type / product)
#[allow(dead_code)]
pub fn mk_prod(a: Expr, b: Expr) -> Expr {
    mk_app(mk_app(mk_const("Prod"), a), b)
}
/// `Sum A B` (coproduct / Either)
#[allow(dead_code)]
pub fn mk_sum(a: Expr, b: Expr) -> Expr {
    mk_app(mk_app(mk_const("Sum"), a), b)
}
/// Apply function `f` to a list of argument expressions (left-fold)
#[allow(dead_code)]
pub fn mk_app_list(f: Expr, args: &[Expr]) -> Expr {
    args.iter().cloned().fold(f, mk_app)
}
/// `Eq.refl T a` — the reflexivity proof
#[allow(dead_code)]
pub fn mk_refl(ty: Expr, a: Expr) -> Expr {
    mk_app(mk_app(mk_const("Eq.refl"), ty), a)
}
/// `trivial` — the trivial proof of True
#[allow(dead_code)]
pub fn mk_trivial() -> Expr {
    mk_const("trivial")
}
/// `sorry` — a placeholder proof
#[allow(dead_code)]
pub fn mk_sorry(ty: Expr) -> Expr {
    mk_app(mk_const("sorry"), ty)
}
/// `And.intro h1 h2`
#[allow(dead_code)]
pub fn mk_and_intro(h1: Expr, h2: Expr) -> Expr {
    mk_app(mk_app(mk_const("And.intro"), h1), h2)
}
/// `Or.inl h` — left injection into Or
#[allow(dead_code)]
pub fn mk_or_inl(h: Expr) -> Expr {
    mk_app(mk_const("Or.inl"), h)
}
/// `Or.inr h` — right injection into Or
#[allow(dead_code)]
pub fn mk_or_inr(h: Expr) -> Expr {
    mk_app(mk_const("Or.inr"), h)
}
/// `Nat.zero`
#[allow(dead_code)]
pub fn mk_nat_zero() -> Expr {
    mk_const("Nat.zero")
}
/// `Nat.succ n`
#[allow(dead_code)]
pub fn mk_nat_succ(n: Expr) -> Expr {
    mk_app(mk_const("Nat.succ"), n)
}
/// Build a numeral `n` as iterated `Nat.succ` applications
#[allow(dead_code)]
pub fn mk_nat_numeral(n: u64) -> Expr {
    let mut e = mk_nat_zero();
    for _ in 0..n {
        e = mk_nat_succ(e);
    }
    e
}
/// `List.nil T`
#[allow(dead_code)]
pub fn mk_list_nil(ty: Expr) -> Expr {
    mk_app(mk_const("List.nil"), ty)
}
/// `List.cons T head tail`
#[allow(dead_code)]
pub fn mk_list_cons(ty: Expr, head: Expr, tail: Expr) -> Expr {
    mk_app(mk_app(mk_app(mk_const("List.cons"), ty), head), tail)
}
/// `Prod.mk a b`
#[allow(dead_code)]
pub fn mk_prod_mk(a: Expr, b: Expr) -> Expr {
    mk_app(mk_app(mk_const("Prod.mk"), a), b)
}
/// `Prod.fst p`
#[allow(dead_code)]
pub fn mk_fst(p: Expr) -> Expr {
    mk_app(mk_const("Prod.fst"), p)
}
/// `Prod.snd p`
#[allow(dead_code)]
pub fn mk_snd(p: Expr) -> Expr {
    mk_app(mk_const("Prod.snd"), p)
}
/// `funext h` — function extensionality
#[allow(dead_code)]
pub fn mk_funext(h: Expr) -> Expr {
    mk_app(mk_const("funext"), h)
}
/// `congrArg f h` — congruence on argument
#[allow(dead_code)]
pub fn mk_congr_arg(f: Expr, h: Expr) -> Expr {
    mk_app(mk_app(mk_const("congrArg"), f), h)
}
/// `congrFun h a` — congruence on function value at point
#[allow(dead_code)]
pub fn mk_congr_fun(h: Expr, a: Expr) -> Expr {
    mk_app(mk_app(mk_const("congrFun"), h), a)
}
/// `Eq.symm h` — symmetry of equality
#[allow(dead_code)]
pub fn mk_eq_symm(h: Expr) -> Expr {
    mk_app(mk_const("Eq.symm"), h)
}
/// `Eq.trans h1 h2` — transitivity of equality
#[allow(dead_code)]
pub fn mk_eq_trans(h1: Expr, h2: Expr) -> Expr {
    mk_app(mk_app(mk_const("Eq.trans"), h1), h2)
}
/// `Eq.mp h pf` — modus ponens using an equality of propositions
#[allow(dead_code)]
pub fn mk_eq_mp(h: Expr, pf: Expr) -> Expr {
    mk_app(mk_app(mk_const("Eq.mp"), h), pf)
}
/// `absurd h1 h2` — derive False from `h1 : P` and `h2 : ¬P`
#[allow(dead_code)]
pub fn mk_absurd(h1: Expr, h2: Expr) -> Expr {
    mk_app(mk_app(mk_const("absurd"), h1), h2)
}
/// `False.elim target h` — ex falso
#[allow(dead_code)]
pub fn mk_false_elim(target: Expr, h: Expr) -> Expr {
    mk_app(mk_app(mk_const("False.elim"), target), h)
}
/// Build a chain of Pi binders from `[(name, ty)]` to `body`
#[allow(dead_code)]
pub fn mk_pi_telescope(binders: &[(&str, Expr)], body: Expr) -> Expr {
    binders
        .iter()
        .rev()
        .fold(body, |acc, (name, ty)| mk_pi(name, ty.clone(), acc))
}
/// Build a chain of Lam binders from `[(name, ty)]` to `body`
#[allow(dead_code)]
pub fn mk_lam_telescope(binders: &[(&str, Expr)], body: Expr) -> Expr {
    binders
        .iter()
        .rev()
        .fold(body, |acc, (name, ty)| mk_lam(name, ty.clone(), acc))
}
/// Build a chain of arrows `T1 → T2 → ... → Tn`
#[allow(dead_code)]
pub fn mk_arrow_chain(types: &[Expr]) -> Option<Expr> {
    if types.is_empty() {
        return None;
    }
    let mut rev = types.iter().cloned().rev();
    let last = rev.next().expect("types is non-empty; checked above");
    Some(rev.fold(last, |acc, ty| mk_arrow(ty, acc)))
}
/// Collect all Pi binders: returns `([(name, bi, ty)], final_body)`
#[allow(dead_code)]
pub fn collect_pi_telescope(e: &Expr) -> (Vec<(Name, BinderInfo, Expr)>, &Expr) {
    let mut binders = vec![];
    let mut cur = e;
    while let Some((name, bi, ty, body)) = as_pi(cur) {
        binders.push((name.clone(), bi, ty.clone()));
        cur = body;
    }
    (binders, cur)
}
/// Collect all Lam binders: returns `([(name, bi, ty)], final_body)`
#[allow(dead_code)]
pub fn collect_lam_telescope(e: &Expr) -> (Vec<(Name, BinderInfo, Expr)>, &Expr) {
    let mut binders = vec![];
    let mut cur = e;
    while let Some((name, bi, ty, body)) = as_lam(cur) {
        binders.push((name.clone(), bi, ty.clone()));
        cur = body;
    }
    (binders, cur)
}
/// Check whether `e` is a `Const` with the given name string
#[allow(dead_code)]
pub fn is_const(e: &Expr, name: &str) -> bool {
    match e {
        Expr::Const(n, _) => n.to_string() == name,
        _ => false,
    }
}
/// Check whether `e` is `App(App(Const("And"), _), _)`
#[allow(dead_code)]
pub fn is_and(e: &Expr) -> bool {
    let (head, args) = collect_apps(e);
    args.len() == 2 && is_const(head, "And")
}
/// Decompose `And A B` into `(A, B)` if possible
#[allow(dead_code)]
pub fn as_and(e: &Expr) -> Option<(&Expr, &Expr)> {
    let (head, args) = collect_apps(e);
    if args.len() == 2 && is_const(head, "And") {
        Some((args[0], args[1]))
    } else {
        None
    }
}
/// Decompose `Or A B` into `(A, B)` if possible
#[allow(dead_code)]
pub fn as_or(e: &Expr) -> Option<(&Expr, &Expr)> {
    let (head, args) = collect_apps(e);
    if args.len() == 2 && is_const(head, "Or") {
        Some((args[0], args[1]))
    } else {
        None
    }
}
/// Decompose `Not A` into `A` if possible
#[allow(dead_code)]
pub fn as_not(e: &Expr) -> Option<&Expr> {
    let (head, args) = collect_apps(e);
    if args.len() == 1 && is_const(head, "Not") {
        Some(args[0])
    } else {
        None
    }
}
/// Decompose `Eq T a b` into `(T, a, b)` if possible
#[allow(dead_code)]
pub fn as_eq(e: &Expr) -> Option<(&Expr, &Expr, &Expr)> {
    let (head, args) = collect_apps(e);
    if args.len() == 3 && is_const(head, "Eq") {
        Some((args[0], args[1], args[2]))
    } else {
        None
    }
}
/// Decompose `Iff A B` into `(A, B)` if possible
#[allow(dead_code)]
pub fn as_iff(e: &Expr) -> Option<(&Expr, &Expr)> {
    let (head, args) = collect_apps(e);
    if args.len() == 2 && is_const(head, "Iff") {
        Some((args[0], args[1]))
    } else {
        None
    }
}
/// Decompose `Exists (lam x : ty => body)` into `(ty, body_lam)` if possible
#[allow(dead_code)]
pub fn as_exists(e: &Expr) -> Option<(&Expr, &Expr)> {
    let (head, args) = collect_apps(e);
    if args.len() == 1 && is_const(head, "Exists") {
        if let Some((_, _, ty, body)) = as_lam(args[0]) {
            return Some((ty, body));
        }
    }
    None
}
/// If `e` is a `Lit(Nat(n))`, return `Some(n)`
#[allow(dead_code)]
pub fn as_nat_lit(e: &Expr) -> Option<u64> {
    match e {
        Expr::Lit(Literal::Nat(n)) => Some(*n),
        _ => None,
    }
}
/// If `e` is a `Lit(Str(s))`, return `Some(s)`
#[allow(dead_code)]
pub fn as_str_lit(e: &Expr) -> Option<&str> {
    match e {
        Expr::Lit(Literal::Str(s)) => Some(s.as_str()),
        _ => None,
    }
}
/// Does expression `e` contain any `FVar`?
#[allow(dead_code)]
pub fn has_fvar(e: &Expr) -> bool {
    match e {
        Expr::FVar(_) => true,
        Expr::App(f, a) => has_fvar(f) || has_fvar(a),
        Expr::Lam(_, _, ty, b) | Expr::Pi(_, _, ty, b) => has_fvar(ty) || has_fvar(b),
        Expr::Let(_, ty, val, b) => has_fvar(ty) || has_fvar(val) || has_fvar(b),
        Expr::Proj(_, _, inner) => has_fvar(inner),
        _ => false,
    }
}
/// Does expression `e` contain any `Const` with the given name?
#[allow(dead_code)]
pub fn has_const(e: &Expr, name: &str) -> bool {
    match e {
        Expr::Const(n, _) => n.to_string() == name,
        Expr::App(f, a) => has_const(f, name) || has_const(a, name),
        Expr::Lam(_, _, ty, b) | Expr::Pi(_, _, ty, b) => has_const(ty, name) || has_const(b, name),
        Expr::Let(_, ty, val, b) => {
            has_const(ty, name) || has_const(val, name) || has_const(b, name)
        }
        Expr::Proj(_, _, inner) => has_const(inner, name),
        _ => false,
    }
}
/// Count the number of `App` nodes
#[allow(dead_code)]
pub fn count_apps(e: &Expr) -> usize {
    match e {
        Expr::App(f, a) => 1 + count_apps(f) + count_apps(a),
        Expr::Lam(_, _, ty, b) | Expr::Pi(_, _, ty, b) => count_apps(ty) + count_apps(b),
        Expr::Let(_, ty, val, b) => count_apps(ty) + count_apps(val) + count_apps(b),
        Expr::Proj(_, _, inner) => count_apps(inner),
        _ => 0,
    }
}
/// Count the number of `Lam` binders
#[allow(dead_code)]
pub fn count_lams(e: &Expr) -> usize {
    match e {
        Expr::Lam(_, _, ty, b) => 1 + count_lams(ty) + count_lams(b),
        Expr::App(f, a) => count_lams(f) + count_lams(a),
        Expr::Pi(_, _, ty, b) => count_lams(ty) + count_lams(b),
        Expr::Let(_, ty, val, b) => count_lams(ty) + count_lams(val) + count_lams(b),
        Expr::Proj(_, _, inner) => count_lams(inner),
        _ => 0,
    }
}
/// Count the number of `Pi` binders
#[allow(dead_code)]
pub fn count_pis(e: &Expr) -> usize {
    match e {
        Expr::Pi(_, _, ty, b) => 1 + count_pis(ty) + count_pis(b),
        Expr::App(f, a) => count_pis(f) + count_pis(a),
        Expr::Lam(_, _, ty, b) => count_pis(ty) + count_pis(b),
        Expr::Let(_, ty, val, b) => count_pis(ty) + count_pis(val) + count_pis(b),
        Expr::Proj(_, _, inner) => count_pis(inner),
        _ => 0,
    }
}
/// Is the expression an atom (no subexpressions)?
#[allow(dead_code)]
pub fn is_atom(e: &Expr) -> bool {
    matches!(
        e,
        Expr::BVar(_) | Expr::FVar(_) | Expr::Sort(_) | Expr::Const(_, _) | Expr::Lit(_)
    )
}
/// Is the expression a lambda abstraction?
#[allow(dead_code)]
pub fn is_lam(e: &Expr) -> bool {
    matches!(e, Expr::Lam(_, _, _, _))
}
/// Is the expression a Pi type?
#[allow(dead_code)]
pub fn is_pi(e: &Expr) -> bool {
    matches!(e, Expr::Pi(_, _, _, _))
}
/// Is the expression an application?
#[allow(dead_code)]
pub fn is_app(e: &Expr) -> bool {
    matches!(e, Expr::App(_, _))
}
/// Is the expression a let binding?
#[allow(dead_code)]
pub fn is_let(e: &Expr) -> bool {
    matches!(e, Expr::Let(_, _, _, _))
}
/// Is the expression a literal?
#[allow(dead_code)]
pub fn is_lit(e: &Expr) -> bool {
    matches!(e, Expr::Lit(_))
}
/// Is the expression a projection?
#[allow(dead_code)]
pub fn is_proj(e: &Expr) -> bool {
    matches!(e, Expr::Proj(_, _, _))
}
/// Construct `Sort n` (universe level n)
#[allow(dead_code)]
pub fn mk_sort_n(n: u32) -> Expr {
    let mut level = oxilean_kernel::Level::Zero;
    for _ in 0..n {
        level = oxilean_kernel::Level::succ(level);
    }
    Expr::Sort(level)
}
/// Construct `Sort (max u v)`
#[allow(dead_code)]
pub fn mk_sort_max(u: oxilean_kernel::Level, v: oxilean_kernel::Level) -> Expr {
    Expr::Sort(oxilean_kernel::Level::max(u, v))
}
/// Construct `Sort (imax u v)`
#[allow(dead_code)]
pub fn mk_sort_imax(u: oxilean_kernel::Level, v: oxilean_kernel::Level) -> Expr {
    Expr::Sort(oxilean_kernel::Level::imax(u, v))
}
/// Build a `∀ x : T, P x` proposition
#[allow(dead_code)]
pub fn mk_forall(var: &str, ty: Expr, body: Expr) -> Expr {
    mk_pi(var, ty, body)
}
/// Build `∀ x y : T, P x y` (curried)
#[allow(dead_code)]
pub fn mk_forall2(x: &str, y: &str, ty: Expr, body: Expr) -> Expr {
    mk_pi(x, ty.clone(), mk_pi(y, ty, body))
}
/// Build the proposition `a = b : T`
#[allow(dead_code)]
pub fn mk_eq_prop(ty: Expr, a: Expr, b: Expr) -> Expr {
    mk_eq(ty, a, b)
}
/// Build `¬ (a = b)` — disequality
#[allow(dead_code)]
pub fn mk_ne(ty: Expr, a: Expr, b: Expr) -> Expr {
    mk_not(mk_eq(ty, a, b))
}
/// Build `a ≤ b` for natural numbers
#[allow(dead_code)]
pub fn mk_nat_le(a: Expr, b: Expr) -> Expr {
    mk_app(mk_app(mk_const("Nat.ble"), a), b)
}
/// Build `a < b` for natural numbers
#[allow(dead_code)]
pub fn mk_nat_lt(a: Expr, b: Expr) -> Expr {
    mk_app(mk_app(mk_const("Nat.blt"), a), b)
}
/// Build `a + b` for natural numbers
#[allow(dead_code)]
pub fn mk_nat_add(a: Expr, b: Expr) -> Expr {
    mk_app(mk_app(mk_const("Nat.add"), a), b)
}
/// Build `a * b` for natural numbers
#[allow(dead_code)]
pub fn mk_nat_mul(a: Expr, b: Expr) -> Expr {
    mk_app(mk_app(mk_const("Nat.mul"), a), b)
}
/// Build `Nat.sub a b`
#[allow(dead_code)]
pub fn mk_nat_sub(a: Expr, b: Expr) -> Expr {
    mk_app(mk_app(mk_const("Nat.sub"), a), b)
}
/// Replace every `Const(name, _)` occurrence with `replacement` in `e`.
#[allow(dead_code)]
pub fn replace_const(e: Expr, name: &str, replacement: &Expr) -> Expr {
    match e {
        Expr::Const(ref n, _) if n.to_string() == name => replacement.clone(),
        Expr::App(f, a) => Expr::App(
            Box::new(replace_const(*f, name, replacement)),
            Box::new(replace_const(*a, name, replacement)),
        ),
        Expr::Lam(bi, n, ty, body) => Expr::Lam(
            bi,
            n,
            Box::new(replace_const(*ty, name, replacement)),
            Box::new(replace_const(*body, name, replacement)),
        ),
        Expr::Pi(bi, n, ty, body) => Expr::Pi(
            bi,
            n,
            Box::new(replace_const(*ty, name, replacement)),
            Box::new(replace_const(*body, name, replacement)),
        ),
        Expr::Let(n, ty, val, body) => Expr::Let(
            n,
            Box::new(replace_const(*ty, name, replacement)),
            Box::new(replace_const(*val, name, replacement)),
            Box::new(replace_const(*body, name, replacement)),
        ),
        Expr::Proj(n, idx, inner) => {
            Expr::Proj(n, idx, Box::new(replace_const(*inner, name, replacement)))
        }
        other => other,
    }
}
/// Rename a binder: replace `BVar(0)` in `body` with `new_name` constant.
#[allow(dead_code)]
pub fn binder_name_to_const(body: &Expr, new_name: &str) -> Expr {
    subst_bvar_zero(body, &mk_const(new_name))
}
/// Substitute `BVar(0)` with `replacement` throughout `e`.
#[allow(dead_code)]
pub fn subst_bvar_zero(e: &Expr, replacement: &Expr) -> Expr {
    subst_bvar(e, 0, replacement)
}
/// Substitute `BVar(target)` with `replacement` throughout `e`, adjusting
/// bound-variable indices for binders.
#[allow(dead_code)]
pub fn subst_bvar(e: &Expr, target: u32, replacement: &Expr) -> Expr {
    match e {
        Expr::BVar(i) => {
            if *i == target {
                replacement.clone()
            } else {
                e.clone()
            }
        }
        Expr::App(f, a) => Expr::App(
            Box::new(subst_bvar(f, target, replacement)),
            Box::new(subst_bvar(a, target, replacement)),
        ),
        Expr::Lam(bi, n, ty, body) => Expr::Lam(
            *bi,
            n.clone(),
            Box::new(subst_bvar(ty, target, replacement)),
            Box::new(subst_bvar(body, target + 1, replacement)),
        ),
        Expr::Pi(bi, n, ty, body) => Expr::Pi(
            *bi,
            n.clone(),
            Box::new(subst_bvar(ty, target, replacement)),
            Box::new(subst_bvar(body, target + 1, replacement)),
        ),
        Expr::Let(n, ty, val, body) => Expr::Let(
            n.clone(),
            Box::new(subst_bvar(ty, target, replacement)),
            Box::new(subst_bvar(val, target, replacement)),
            Box::new(subst_bvar(body, target + 1, replacement)),
        ),
        Expr::Proj(n, idx, inner) => Expr::Proj(
            n.clone(),
            *idx,
            Box::new(subst_bvar(inner, target, replacement)),
        ),
        other => other.clone(),
    }
}
/// Alpha-equality: are two expressions equal up to binder renaming?
/// (Here we use structural equality since we use De Bruijn indices.)
#[allow(dead_code)]
pub fn alpha_eq(e1: &Expr, e2: &Expr) -> bool {
    e1 == e2
}
/// Are two expressions syntactically identical (same as `==`)?
#[allow(dead_code)]
pub fn syntactic_eq(e1: &Expr, e2: &Expr) -> bool {
    e1 == e2
}
/// Shift all free `BVar` indices by `delta` (upward) in expression `e`.
///
/// Free BVars are those with index >= `cutoff`.
#[allow(dead_code)]
pub fn shift_bvars(e: &Expr, cutoff: u32, delta: u32) -> Expr {
    match e {
        Expr::BVar(i) => {
            if *i >= cutoff {
                Expr::BVar(*i + delta)
            } else {
                e.clone()
            }
        }
        Expr::App(f, a) => Expr::App(
            Box::new(shift_bvars(f, cutoff, delta)),
            Box::new(shift_bvars(a, cutoff, delta)),
        ),
        Expr::Lam(bi, n, ty, body) => Expr::Lam(
            *bi,
            n.clone(),
            Box::new(shift_bvars(ty, cutoff, delta)),
            Box::new(shift_bvars(body, cutoff + 1, delta)),
        ),
        Expr::Pi(bi, n, ty, body) => Expr::Pi(
            *bi,
            n.clone(),
            Box::new(shift_bvars(ty, cutoff, delta)),
            Box::new(shift_bvars(body, cutoff + 1, delta)),
        ),
        Expr::Let(n, ty, val, body) => Expr::Let(
            n.clone(),
            Box::new(shift_bvars(ty, cutoff, delta)),
            Box::new(shift_bvars(val, cutoff, delta)),
            Box::new(shift_bvars(body, cutoff + 1, delta)),
        ),
        Expr::Proj(n, idx, inner) => {
            Expr::Proj(n.clone(), *idx, Box::new(shift_bvars(inner, cutoff, delta)))
        }
        other => other.clone(),
    }
}
/// Perform one-step top-level beta reduction: `(λ x. body) arg → body[arg/0]`
#[allow(dead_code)]
pub fn beta_reduce_once(e: &Expr) -> Option<Expr> {
    if let Expr::App(f, arg) = e {
        if let Expr::Lam(_, _, _, body) = f.as_ref() {
            let shifted_arg = shift_bvars(arg, 0, 0);
            return Some(subst_bvar(body, 0, &shifted_arg));
        }
    }
    None
}
/// Attempt full (head) beta normalization with a step limit.
#[allow(dead_code)]
pub fn beta_normalize(e: Expr, max_steps: usize) -> Expr {
    let mut cur = e;
    for _ in 0..max_steps {
        match beta_reduce_once(&cur) {
            Some(reduced) => cur = reduced,
            None => break,
        }
    }
    cur
}
/// Check if the head of an application chain is a specific constant
#[allow(dead_code)]
pub fn head_is_const(e: &Expr, name: &str) -> bool {
    let (head, _) = collect_apps(e);
    is_const(head, name)
}
/// Get the number of arguments applied to the head of an application chain
#[allow(dead_code)]
pub fn app_arg_count(e: &Expr) -> usize {
    let (_, args) = collect_apps(e);
    args.len()
}
/// Check if `e` is a fully applied binary operator `op a b`
#[allow(dead_code)]
pub fn is_binary_app(e: &Expr, op: &str) -> bool {
    let (head, args) = collect_apps(e);
    args.len() == 2 && is_const(head, op)
}
/// Check if `e` is a fully applied unary operator `op a`
#[allow(dead_code)]
pub fn is_unary_app(e: &Expr, op: &str) -> bool {
    let (head, args) = collect_apps(e);
    args.len() == 1 && is_const(head, op)
}
/// Construct a `Float` literal (if supported by kernel)
#[allow(dead_code)]
pub fn mk_float_lit(f: f64) -> Expr {
    Expr::Lit(Literal::Str(f.to_string()))
}
/// Check whether the expression is the boolean `true` constant
#[allow(dead_code)]
pub fn is_bool_true(e: &Expr) -> bool {
    is_const(e, "true") || is_const(e, "Bool.true")
}
/// Check whether the expression is the boolean `false` constant
#[allow(dead_code)]
pub fn is_bool_false(e: &Expr) -> bool {
    is_const(e, "false") || is_const(e, "Bool.false")
}
/// Measure the "weight" of an expression: atoms = 1, binders = 2, apps = 2.
#[allow(dead_code)]
pub fn expr_weight(e: &Expr) -> usize {
    match e {
        Expr::BVar(_) | Expr::FVar(_) | Expr::Sort(_) | Expr::Const(_, _) | Expr::Lit(_) => 1,
        Expr::App(f, a) => 2 + expr_weight(f) + expr_weight(a),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            2 + expr_weight(ty) + expr_weight(body)
        }
        Expr::Let(_, ty, val, body) => 3 + expr_weight(ty) + expr_weight(val) + expr_weight(body),
        Expr::Proj(_, _, inner) => 2 + expr_weight(inner),
    }
}
/// Measure the number of leaf nodes (atoms) in an expression
#[allow(dead_code)]
pub fn leaf_count(e: &Expr) -> usize {
    match e {
        Expr::BVar(_) | Expr::FVar(_) | Expr::Sort(_) | Expr::Const(_, _) | Expr::Lit(_) => 1,
        Expr::App(f, a) => leaf_count(f) + leaf_count(a),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => leaf_count(ty) + leaf_count(body),
        Expr::Let(_, ty, val, body) => leaf_count(ty) + leaf_count(val) + leaf_count(body),
        Expr::Proj(_, _, inner) => leaf_count(inner),
    }
}
#[cfg(test)]
mod extended_tests {
    use super::*;
    use crate::convenience::*;
    #[test]
    fn test_mk_let() {
        let e = mk_let("x", mk_nat(), mk_nat_lit(5), mk_bvar(0));
        assert!(is_let(&e));
    }
    #[test]
    fn test_mk_list() {
        let e = mk_list(mk_nat());
        let (head, args) = collect_apps(&e);
        assert_eq!(
            as_const(head).map(|n| n.to_string()),
            Some("List".to_string())
        );
        assert_eq!(args.len(), 1);
    }
    #[test]
    fn test_mk_pi_telescope() {
        let body = mk_bvar(1);
        let e = mk_pi_telescope(&[("x", mk_nat()), ("y", mk_nat())], body.clone());
        assert_eq!(pi_arity(&e), 2);
    }
    #[test]
    fn test_mk_lam_telescope() {
        let body = mk_bvar(0);
        let e = mk_lam_telescope(&[("x", mk_nat()), ("y", mk_nat())], body);
        assert_eq!(lam_arity(&e), 2);
    }
    #[test]
    fn test_mk_arrow_chain() {
        let types = vec![mk_nat(), mk_nat(), mk_nat()];
        let arr = mk_arrow_chain(&types).expect("arr should be present");
        assert_eq!(pi_arity(&arr), 2);
    }
    #[test]
    fn test_collect_pi_telescope() {
        let e = mk_pi_telescope(&[("a", mk_nat()), ("b", mk_bool())], mk_prop());
        let (binders, _body) = collect_pi_telescope(&e);
        assert_eq!(binders.len(), 2);
    }
    #[test]
    fn test_is_const_predicate() {
        let e = mk_const("Nat");
        assert!(is_const(&e, "Nat"));
        assert!(!is_const(&e, "Int"));
    }
    #[test]
    fn test_as_and() {
        let e = mk_and(mk_const("A"), mk_const("B"));
        let r = as_and(&e);
        assert!(r.is_some());
        let (a, b) = r.expect("r should be valid");
        assert!(is_const(a, "A"));
        assert!(is_const(b, "B"));
    }
    #[test]
    fn test_as_or() {
        let e = mk_or(mk_const("A"), mk_const("B"));
        let r = as_or(&e);
        assert!(r.is_some());
    }
    #[test]
    fn test_as_not() {
        let e = mk_not(mk_const("P"));
        assert!(as_not(&e).is_some());
    }
    #[test]
    fn test_as_eq() {
        let e = mk_eq(mk_nat(), mk_nat_lit(1), mk_nat_lit(2));
        let r = as_eq(&e);
        assert!(r.is_some());
        let (_ty, a, b) = r.expect("r should be valid");
        assert_eq!(as_nat_lit(a), Some(1));
        assert_eq!(as_nat_lit(b), Some(2));
    }
    #[test]
    fn test_has_fvar() {
        let e = mk_const("Nat");
        assert!(!has_fvar(&e));
        let fv = Expr::FVar(oxilean_kernel::FVarId(42));
        assert!(has_fvar(&fv));
        let app = mk_app(mk_const("f"), fv);
        assert!(has_fvar(&app));
    }
    #[test]
    fn test_has_const() {
        let e = mk_app(mk_const("f"), mk_const("Nat"));
        assert!(has_const(&e, "Nat"));
        assert!(has_const(&e, "f"));
        assert!(!has_const(&e, "Int"));
    }
    #[test]
    fn test_count_apps() {
        let e = mk_app_n(mk_const("f"), vec![mk_const("a"), mk_const("b")]);
        assert_eq!(count_apps(&e), 2);
    }
    #[test]
    fn test_replace_const() {
        let e = mk_app(mk_const("Nat"), mk_const("Nat"));
        let result = replace_const(e, "Nat", &mk_const("Int"));
        assert!(has_const(&result, "Int"));
        assert!(!has_const(&result, "Nat"));
    }
    #[test]
    fn test_subst_bvar_zero() {
        let body = mk_bvar(0);
        let result = subst_bvar_zero(&body, &mk_const("Nat"));
        assert!(is_const(&result, "Nat"));
    }
    #[test]
    fn test_shift_bvars() {
        let e = mk_bvar(0);
        let shifted = shift_bvars(&e, 0, 2);
        assert_eq!(shifted, mk_bvar(2));
        let e2 = mk_bvar(0);
        let unshifted = shift_bvars(&e2, 1, 2);
        assert_eq!(unshifted, mk_bvar(0));
    }
    #[test]
    fn test_beta_reduce_once() {
        let lam = mk_lam("x", mk_nat(), mk_bvar(0));
        let app = mk_app(lam, mk_const("Nat"));
        let reduced = beta_reduce_once(&app).expect("reduced should be present");
        assert!(is_const(&reduced, "Nat"));
    }
    #[test]
    fn test_beta_reduce_once_no_redex() {
        let e = mk_app(mk_const("f"), mk_const("Nat"));
        assert!(beta_reduce_once(&e).is_none());
    }
    #[test]
    fn test_head_is_const() {
        let e = mk_app_n(mk_const("List"), vec![mk_const("Nat")]);
        assert!(head_is_const(&e, "List"));
        assert!(!head_is_const(&e, "Array"));
    }
    #[test]
    fn test_is_binary_app() {
        let e = mk_and(mk_const("A"), mk_const("B"));
        assert!(is_binary_app(&e, "And"));
        assert!(!is_binary_app(&e, "Or"));
    }
    #[test]
    fn test_is_unary_app() {
        let e = mk_not(mk_const("P"));
        assert!(is_unary_app(&e, "Not"));
        assert!(!is_unary_app(&e, "And"));
    }
    #[test]
    fn test_expr_weight() {
        assert_eq!(expr_weight(&mk_const("Nat")), 1);
        assert_eq!(expr_weight(&mk_app(mk_const("f"), mk_const("a"))), 4);
    }
    #[test]
    fn test_leaf_count() {
        let e = mk_app_n(mk_const("f"), vec![mk_const("a"), mk_const("b")]);
        assert_eq!(leaf_count(&e), 3);
    }
    #[test]
    fn test_mk_nat_numeral() {
        let three = mk_nat_numeral(3);
        assert!(is_app(&three));
    }
    #[test]
    fn test_is_atom() {
        assert!(is_atom(&mk_const("Nat")));
        assert!(is_atom(&mk_bvar(0)));
        assert!(!is_atom(&mk_app(mk_const("f"), mk_const("a"))));
    }
    #[test]
    fn test_mk_refl() {
        let r = mk_refl(mk_nat(), mk_nat_lit(5));
        assert!(head_is_const(&r, "Eq.refl"));
    }
    #[test]
    fn test_mk_nat_add() {
        let e = mk_nat_add(mk_nat_lit(1), mk_nat_lit(2));
        assert!(head_is_const(&e, "Nat.add"));
    }
    #[test]
    fn test_mk_sort_n() {
        let s0 = mk_sort_n(0);
        assert!(is_sort(&s0));
        assert!(is_prop(&s0));
        let s1 = mk_sort_n(1);
        assert!(is_sort(&s1));
        assert!(!is_prop(&s1));
    }
    #[test]
    fn test_as_nat_lit() {
        let e = mk_nat_lit(42);
        assert_eq!(as_nat_lit(&e), Some(42));
        assert_eq!(as_nat_lit(&mk_const("x")), None);
    }
    #[test]
    fn test_as_str_lit() {
        let e = mk_str_lit("hello");
        assert_eq!(as_str_lit(&e), Some("hello"));
    }
    #[test]
    fn test_is_bool_true_false() {
        assert!(is_bool_true(&mk_const("true")));
        assert!(is_bool_false(&mk_const("false")));
        assert!(!is_bool_true(&mk_const("false")));
    }
    #[test]
    fn test_count_pis_lams() {
        let pis = mk_pi_telescope(&[("a", mk_nat()), ("b", mk_nat())], mk_prop());
        assert_eq!(count_pis(&pis), 2);
        let lams = mk_lam_telescope(&[("a", mk_nat()), ("b", mk_nat())], mk_bvar(0));
        assert_eq!(count_lams(&lams), 2);
    }
}
#[cfg(test)]
mod metaconvenience_ext2_tests {
    use super::*;
    use crate::convenience::*;
    #[test]
    fn test_metaconvenience_ext_util_basic() {
        let mut u = MetaConvenienceExtUtil::new("test");
        u.push(10);
        u.push(20);
        assert_eq!(u.sum(), 30);
        assert_eq!(u.len(), 2);
    }
    #[test]
    fn test_metaconvenience_ext_util_min_max() {
        let mut u = MetaConvenienceExtUtil::new("mm");
        u.push(5);
        u.push(1);
        u.push(9);
        assert_eq!(u.min_val(), Some(1));
        assert_eq!(u.max_val(), Some(9));
    }
    #[test]
    fn test_metaconvenience_ext_util_flags() {
        let mut u = MetaConvenienceExtUtil::new("flags");
        u.set_flag(3);
        assert!(u.has_flag(3));
        assert!(!u.has_flag(2));
    }
    #[test]
    fn test_metaconvenience_ext_util_pop() {
        let mut u = MetaConvenienceExtUtil::new("pop");
        u.push(42);
        assert_eq!(u.pop(), Some(42));
        assert!(u.is_empty());
    }
    #[test]
    fn test_metaconvenience_ext_map_basic() {
        let mut m: MetaConvenienceExtMap<i32> = MetaConvenienceExtMap::new();
        m.insert("key", 42);
        assert_eq!(m.get("key"), Some(&42));
        assert!(m.contains("key"));
        assert!(!m.contains("other"));
    }
    #[test]
    fn test_metaconvenience_ext_map_get_or_default() {
        let mut m: MetaConvenienceExtMap<i32> = MetaConvenienceExtMap::new();
        m.insert("k", 5);
        assert_eq!(m.get_or_default("k"), 5);
        assert_eq!(m.get_or_default("missing"), 0);
    }
    #[test]
    fn test_metaconvenience_ext_map_keys_sorted() {
        let mut m: MetaConvenienceExtMap<i32> = MetaConvenienceExtMap::new();
        m.insert("z", 1);
        m.insert("a", 2);
        m.insert("m", 3);
        let keys = m.keys_sorted();
        assert_eq!(keys[0].as_str(), "a");
        assert_eq!(keys[2].as_str(), "z");
    }
    #[test]
    fn test_metaconvenience_window_mean() {
        let mut w = MetaConvenienceWindow::new(3);
        w.push(1.0);
        w.push(2.0);
        w.push(3.0);
        assert!((w.mean() - 2.0).abs() < 1e-10);
    }
    #[test]
    fn test_metaconvenience_window_evict() {
        let mut w = MetaConvenienceWindow::new(2);
        w.push(10.0);
        w.push(20.0);
        w.push(30.0);
        assert_eq!(w.len(), 2);
        assert!((w.mean() - 25.0).abs() < 1e-10);
    }
    #[test]
    fn test_metaconvenience_window_std_dev() {
        let mut w = MetaConvenienceWindow::new(10);
        for i in 0..10 {
            w.push(i as f64);
        }
        assert!(w.std_dev() > 0.0);
    }
    #[test]
    fn test_metaconvenience_builder_basic() {
        let b = MetaConvenienceBuilder::new("test")
            .add_item("a")
            .add_item("b")
            .set_config("key", "val");
        assert_eq!(b.item_count(), 2);
        assert!(b.has_config("key"));
        assert_eq!(b.get_config("key"), Some("val"));
    }
    #[test]
    fn test_metaconvenience_builder_summary() {
        let b = MetaConvenienceBuilder::new("suite").add_item("x");
        let s = b.build_summary();
        assert!(s.contains("suite"));
    }
    #[test]
    fn test_metaconvenience_state_machine_start() {
        let mut sm = MetaConvenienceStateMachine::new();
        assert!(sm.start());
        assert!(sm.state.is_running());
    }
    #[test]
    fn test_metaconvenience_state_machine_complete() {
        let mut sm = MetaConvenienceStateMachine::new();
        sm.start();
        sm.complete();
        assert!(sm.state.is_terminal());
    }
    #[test]
    fn test_metaconvenience_state_machine_fail() {
        let mut sm = MetaConvenienceStateMachine::new();
        sm.fail("oops");
        assert!(sm.state.is_terminal());
        assert_eq!(sm.state.error_msg(), Some("oops"));
    }
    #[test]
    fn test_metaconvenience_state_machine_no_transition_after_terminal() {
        let mut sm = MetaConvenienceStateMachine::new();
        sm.complete();
        assert!(!sm.start());
    }
    #[test]
    fn test_metaconvenience_work_queue_basic() {
        let mut wq = MetaConvenienceWorkQueue::new(10);
        wq.enqueue("task1".to_string());
        wq.enqueue("task2".to_string());
        assert_eq!(wq.pending_count(), 2);
        let t = wq.dequeue();
        assert_eq!(t, Some("task1".to_string()));
        assert_eq!(wq.processed_count(), 1);
    }
    #[test]
    fn test_metaconvenience_work_queue_capacity() {
        let mut wq = MetaConvenienceWorkQueue::new(2);
        wq.enqueue("a".to_string());
        wq.enqueue("b".to_string());
        assert!(wq.is_full());
        assert!(!wq.enqueue("c".to_string()));
    }
    #[test]
    fn test_metaconvenience_counter_map_basic() {
        let mut cm = MetaConvenienceCounterMap::new();
        cm.increment("apple");
        cm.increment("apple");
        cm.increment("banana");
        assert_eq!(cm.count("apple"), 2);
        assert_eq!(cm.count("banana"), 1);
        assert_eq!(cm.num_unique(), 2);
    }
    #[test]
    fn test_metaconvenience_counter_map_frequency() {
        let mut cm = MetaConvenienceCounterMap::new();
        cm.increment("a");
        cm.increment("a");
        cm.increment("b");
        assert!((cm.frequency("a") - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_metaconvenience_counter_map_most_common() {
        let mut cm = MetaConvenienceCounterMap::new();
        cm.increment("x");
        cm.increment("y");
        cm.increment("x");
        let (k, v) = cm.most_common().expect("most_common should succeed");
        assert_eq!(k.as_str(), "x");
        assert_eq!(v, 2);
    }
}
#[cfg(test)]
mod convenience_analysis_tests {
    use super::*;
    use crate::convenience::*;
    #[test]
    fn test_convenience_result_ok() {
        let r = ConvenienceResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_convenience_result_err() {
        let r = ConvenienceResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_convenience_result_partial() {
        let r = ConvenienceResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_convenience_result_skipped() {
        let r = ConvenienceResult::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_convenience_analysis_pass_run() {
        let mut p = ConvenienceAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_convenience_analysis_pass_empty_input() {
        let mut p = ConvenienceAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_convenience_analysis_pass_success_rate() {
        let mut p = ConvenienceAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_convenience_analysis_pass_disable() {
        let mut p = ConvenienceAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_convenience_pipeline_basic() {
        let mut pipeline = ConveniencePipeline::new("main_pipeline");
        pipeline.add_pass(ConvenienceAnalysisPass::new("pass1"));
        pipeline.add_pass(ConvenienceAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_convenience_pipeline_disabled_pass() {
        let mut pipeline = ConveniencePipeline::new("partial");
        let mut p = ConvenienceAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(ConvenienceAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_convenience_diff_basic() {
        let mut d = ConvenienceDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_convenience_diff_summary() {
        let mut d = ConvenienceDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }
    #[test]
    fn test_convenience_config_set_get() {
        let mut cfg = ConvenienceConfig::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_convenience_config_read_only() {
        let mut cfg = ConvenienceConfig::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_convenience_config_remove() {
        let mut cfg = ConvenienceConfig::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_convenience_diagnostics_basic() {
        let mut diag = ConvenienceDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_convenience_diagnostics_max_errors() {
        let mut diag = ConvenienceDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_convenience_diagnostics_clear() {
        let mut diag = ConvenienceDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_convenience_config_value_types() {
        let b = ConvenienceConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = ConvenienceConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = ConvenienceConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = ConvenienceConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = ConvenienceConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
#[cfg(test)]
mod convenience_ext_tests_4000 {
    use super::*;
    use crate::convenience::*;
    #[test]
    fn test_convenience_ext_result_ok_4000() {
        let r = ConvenienceExtResult4000::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_convenience_ext_result_err_4000() {
        let r = ConvenienceExtResult4000::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_convenience_ext_result_partial_4000() {
        let r = ConvenienceExtResult4000::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_convenience_ext_result_skipped_4000() {
        let r = ConvenienceExtResult4000::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_convenience_ext_pass_run_4000() {
        let mut p = ConvenienceExtPass4000::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_convenience_ext_pass_empty_4000() {
        let mut p = ConvenienceExtPass4000::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_convenience_ext_pass_rate_4000() {
        let mut p = ConvenienceExtPass4000::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_convenience_ext_pass_disable_4000() {
        let mut p = ConvenienceExtPass4000::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_convenience_ext_pipeline_basic_4000() {
        let mut pipeline = ConvenienceExtPipeline4000::new("main_pipeline");
        pipeline.add_pass(ConvenienceExtPass4000::new("pass1"));
        pipeline.add_pass(ConvenienceExtPass4000::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_convenience_ext_pipeline_disabled_4000() {
        let mut pipeline = ConvenienceExtPipeline4000::new("partial");
        let mut p = ConvenienceExtPass4000::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(ConvenienceExtPass4000::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_convenience_ext_diff_basic_4000() {
        let mut d = ConvenienceExtDiff4000::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_convenience_ext_config_set_get_4000() {
        let mut cfg = ConvenienceExtConfig4000::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_convenience_ext_config_read_only_4000() {
        let mut cfg = ConvenienceExtConfig4000::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_convenience_ext_config_remove_4000() {
        let mut cfg = ConvenienceExtConfig4000::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_convenience_ext_diagnostics_basic_4000() {
        let mut diag = ConvenienceExtDiag4000::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_convenience_ext_diagnostics_max_errors_4000() {
        let mut diag = ConvenienceExtDiag4000::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_convenience_ext_diagnostics_clear_4000() {
        let mut diag = ConvenienceExtDiag4000::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_convenience_ext_config_value_types_4000() {
        let b = ConvenienceExtConfigVal4000::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = ConvenienceExtConfigVal4000::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = ConvenienceExtConfigVal4000::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = ConvenienceExtConfigVal4000::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = ConvenienceExtConfigVal4000::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
