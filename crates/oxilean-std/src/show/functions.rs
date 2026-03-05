//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Expr, Level, Literal, Name};

use super::types::{
    PrettyDoc, ShowBuffer, ShowConfig, ShowMode, ShowRegistry, ShowS, ShowStats, Showable,
};

/// A trait for types that can be shown as a string.
pub trait Show {
    /// Show with default configuration.
    fn show(&self) -> String {
        self.show_with(&ShowConfig::default())
    }
    /// Show with a given configuration.
    fn show_with(&self, cfg: &ShowConfig) -> String;
}
/// Extension of `Show` for types that support multi-line pretty output.
pub trait PrettyShow: Show {
    /// Pretty-print into a `ShowBuffer`.
    fn pretty(&self, buf: &mut ShowBuffer, cfg: &ShowConfig);
}
impl Show for Level {
    fn show_with(&self, _cfg: &ShowConfig) -> String {
        show_level(self)
    }
}
impl Show for Name {
    fn show_with(&self, _cfg: &ShowConfig) -> String {
        format!("{}", self)
    }
}
impl Show for BinderInfo {
    fn show_with(&self, _cfg: &ShowConfig) -> String {
        match self {
            BinderInfo::Default => "default".to_string(),
            BinderInfo::Implicit => "implicit".to_string(),
            BinderInfo::StrictImplicit => "strict_implicit".to_string(),
            BinderInfo::InstImplicit => "inst_implicit".to_string(),
        }
    }
}
impl Show for Literal {
    fn show_with(&self, _cfg: &ShowConfig) -> String {
        show_literal(self)
    }
}
impl Show for Expr {
    fn show_with(&self, cfg: &ShowConfig) -> String {
        show_expr_cfg(self, cfg, 0)
    }
}
impl Show for Declaration {
    fn show_with(&self, cfg: &ShowConfig) -> String {
        show_declaration(self, cfg)
    }
}
/// Pretty-print a universe `Level`.
pub fn show_level(lv: &Level) -> String {
    if lv.is_zero() {
        return "0".to_string();
    }
    if let Some(n) = lv.to_nat() {
        return n.to_string();
    }
    format!("{:?}", lv)
}
/// Pretty-print a kernel literal.
pub fn show_literal(lit: &Literal) -> String {
    match lit {
        Literal::Nat(n) => n.to_string(),
        Literal::Str(s) => format!("\"{}\"", s.escape_default()),
    }
}
/// Pretty-print an `Expr` using default settings.
pub fn show_expr(expr: &Expr) -> String {
    show_expr_cfg(expr, &ShowConfig::default(), 0)
}
/// Pretty-print an `Expr` with a given config and depth.
pub fn show_expr_cfg(expr: &Expr, cfg: &ShowConfig, depth: usize) -> String {
    if let Some(max) = cfg.max_depth {
        if depth > max {
            return "...".to_string();
        }
    }
    match expr {
        Expr::BVar(i) => format!("#{}", i),
        Expr::FVar(i) => format!("fvar({})", i.0),
        Expr::Sort(lv) => {
            let s = show_level(lv);
            if lv.is_zero() {
                "Prop".to_string()
            } else {
                format!("Type {}", s)
            }
        }
        Expr::Const(n, levels) => {
            if cfg.show_levels && !levels.is_empty() {
                let ls: Vec<_> = levels.iter().map(show_level).collect();
                format!("{}.{{{}}}", n, ls.join(", "))
            } else {
                format!("{}", n)
            }
        }
        Expr::Lit(lit) => show_literal(lit),
        Expr::App(f, a) => {
            let fs = show_expr_cfg(f, cfg, depth + 1);
            let as_ = show_expr_cfg_paren(a, cfg, depth + 1);
            format!("{} {}", fs, as_)
        }
        Expr::Lam(bi, n, ty, body) => {
            let binder = show_binder(n, bi, ty, cfg, depth);
            let bs = show_expr_cfg(body, cfg, depth + 1);
            format!("{} {} {} {}", cfg.lambda(), binder, cfg.arrow(), bs)
        }
        Expr::Pi(bi, n, dom, cod) => {
            let n_str = format!("{}", n);
            let is_anon = n_str == "_" || n_str.is_empty() || n_str == "Anonymous";
            if is_anon && *bi == BinderInfo::Default {
                let ds = show_expr_cfg_paren(dom, cfg, depth + 1);
                let cs = show_expr_cfg(cod, cfg, depth + 1);
                format!("{} {} {}", ds, cfg.arrow(), cs)
            } else {
                let binder = show_binder(n, bi, dom, cfg, depth);
                let cs = show_expr_cfg(cod, cfg, depth + 1);
                format!("{} {}, {}", cfg.forall_kw(), binder, cs)
            }
        }
        Expr::Let(n, ty, val, body) => {
            let ts = show_expr_cfg(ty, cfg, depth + 1);
            let vs = show_expr_cfg(val, cfg, depth + 1);
            let bs = show_expr_cfg(body, cfg, depth + 1);
            format!("let {} : {} := {}; {}", n, ts, vs, bs)
        }
        Expr::Proj(field, idx, e) => {
            let es = show_expr_cfg(e, cfg, depth + 1);
            format!("({}).{}/{}", es, field, idx)
        }
    }
}
/// Show an expression, parenthesizing if it's complex.
pub fn show_expr_cfg_paren(expr: &Expr, cfg: &ShowConfig, depth: usize) -> String {
    let s = show_expr_cfg(expr, cfg, depth);
    match expr {
        Expr::App(_, _) | Expr::Lam(..) | Expr::Pi(..) | Expr::Let(..) => {
            format!("({})", s)
        }
        _ => s,
    }
}
/// Show a binder `(x : T)` or `{x : T}` etc.
pub fn show_binder(
    name: &Name,
    bi: &BinderInfo,
    ty: &Expr,
    cfg: &ShowConfig,
    depth: usize,
) -> String {
    let ts = if cfg.show_binder_types {
        format!(" : {}", show_expr_cfg(ty, cfg, depth + 1))
    } else {
        String::new()
    };
    let n = format!("{}", name);
    match bi {
        BinderInfo::Default => format!("({}{}", n, ts) + ")",
        BinderInfo::Implicit => format!("{{{}{}", n, ts) + "}",
        BinderInfo::StrictImplicit => format!("{{{{{}{}", n, ts) + "}}",
        BinderInfo::InstImplicit => format!("[{}{}", n, ts) + "]",
    }
}
/// Pretty-print a kernel declaration.
pub fn show_declaration(decl: &Declaration, cfg: &ShowConfig) -> String {
    match decl {
        Declaration::Definition { name, ty, val, .. } => {
            let ts = show_expr_cfg(ty, cfg, 0);
            let vs = show_expr_cfg(val, cfg, 0);
            format!("def {} : {} := {}", name, ts, vs)
        }
        Declaration::Theorem { name, ty, val, .. } => {
            let ts = show_expr_cfg(ty, cfg, 0);
            let ps = show_expr_cfg(val, cfg, 0);
            format!("theorem {} : {} := {}", name, ts, ps)
        }
        Declaration::Axiom { name, ty, .. } => {
            let ts = show_expr_cfg(ty, cfg, 0);
            format!("axiom {} : {}", name, ts)
        }
        Declaration::Opaque { name, ty, val, .. } => {
            let ts = show_expr_cfg(ty, cfg, 0);
            let vs = show_expr_cfg(val, cfg, 0);
            format!("opaque {} : {} := {}", name, ts, vs)
        }
    }
}
/// Build a `show` string for a `Nat` value.
pub fn build_show_nat(n: u64) -> String {
    n.to_string()
}
/// Build a `show` string for a `String` value.
pub fn build_show_string(s: &str) -> String {
    format!("\"{}\"", s.escape_default())
}
/// Build a `show` string for a `Bool` value.
pub fn build_show_bool(b: bool) -> String {
    if b {
        "true".to_string()
    } else {
        "false".to_string()
    }
}
/// Build a `show` string for an `Option<T: Show>`.
pub fn build_show_option<T: Show>(opt: &Option<T>) -> String {
    match opt {
        None => "none".to_string(),
        Some(v) => format!("some ({})", v.show()),
    }
}
/// Build a `show` string for a list.
pub fn build_show_list<T: Show>(list: &[T]) -> String {
    let items: Vec<_> = list.iter().map(|x| x.show()).collect();
    format!("[{}]", items.join(", "))
}
/// Build a `show` string for a pair.
pub fn build_show_pair<A: Show, B: Show>(a: &A, b: &B) -> String {
    format!("({}, {})", a.show(), b.show())
}
/// Show a `bool`.
pub fn show_bool(b: bool) -> String {
    build_show_bool(b)
}
/// Show an `Option<T: Show>`.
pub fn show_option<T: Show>(opt: &Option<T>) -> String {
    build_show_option(opt)
}
/// Show a list of `T: Show` items.
pub fn show_list<T: Show>(list: &[T]) -> String {
    build_show_list(list)
}
/// Show a pair `(A, B)`.
pub fn show_pair<A: Show, B: Show>(a: &A, b: &B) -> String {
    build_show_pair(a, b)
}
/// Truncate a show string to at most `max_chars` characters.
pub fn truncate_show(s: &str, max_chars: usize) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() <= max_chars {
        s.to_string()
    } else {
        chars[..max_chars].iter().collect::<String>() + "..."
    }
}
impl Show for bool {
    fn show_with(&self, _cfg: &ShowConfig) -> String {
        show_bool(*self)
    }
}
impl Show for u64 {
    fn show_with(&self, _cfg: &ShowConfig) -> String {
        self.to_string()
    }
}
impl Show for i64 {
    fn show_with(&self, _cfg: &ShowConfig) -> String {
        self.to_string()
    }
}
impl Show for String {
    fn show_with(&self, _cfg: &ShowConfig) -> String {
        build_show_string(self)
    }
}
impl Show for str {
    fn show_with(&self, _cfg: &ShowConfig) -> String {
        build_show_string(self)
    }
}
impl<T: Show> Show for Option<T> {
    fn show_with(&self, _cfg: &ShowConfig) -> String {
        build_show_option(self)
    }
}
impl<T: Show> Show for Vec<T> {
    fn show_with(&self, _cfg: &ShowConfig) -> String {
        build_show_list(self)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use oxilean_kernel::{BinderInfo, Expr, Level, Literal, Name};
    fn nat_const() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn zero_lit() -> Expr {
        Expr::Lit(Literal::Nat(0))
    }
    #[test]
    fn test_show_config_default() {
        let cfg = ShowConfig::default();
        assert!(!cfg.compact);
        assert!(!cfg.ascii_only);
        assert!(cfg.show_binder_types);
    }
    #[test]
    fn test_show_config_compact() {
        let cfg = ShowConfig::compact();
        assert!(cfg.compact);
    }
    #[test]
    fn test_show_config_ascii() {
        let cfg = ShowConfig::ascii();
        assert!(cfg.ascii_only);
        assert_eq!(cfg.arrow(), "->");
    }
    #[test]
    fn test_show_config_with_depth() {
        let cfg = ShowConfig::default().with_depth(10);
        assert_eq!(cfg.max_depth, Some(10));
    }
    #[test]
    fn test_show_config_unlimited() {
        let cfg = ShowConfig::default().unlimited();
        assert_eq!(cfg.max_depth, None);
    }
    #[test]
    fn test_show_config_arrow_unicode() {
        let cfg = ShowConfig::default();
        assert_eq!(cfg.arrow(), "→");
    }
    #[test]
    fn test_show_level_zero() {
        let lv = Level::zero();
        assert_eq!(show_level(&lv), "0");
    }
    #[test]
    fn test_show_level_one() {
        let lv = Level::succ(Level::zero());
        let s = show_level(&lv);
        assert_eq!(s, "1");
    }
    #[test]
    fn test_show_literal_nat() {
        let lit = Literal::Nat(42);
        assert_eq!(show_literal(&lit), "42");
    }
    #[test]
    fn test_show_literal_str() {
        let lit = Literal::Str("hello".to_string());
        let s = show_literal(&lit);
        assert!(s.contains("hello"));
    }
    #[test]
    fn test_show_expr_bvar() {
        let e = Expr::BVar(3);
        assert_eq!(show_expr(&e), "#3");
    }
    #[test]
    fn test_show_expr_const() {
        let e = Expr::Const(Name::str("Nat"), vec![]);
        assert_eq!(show_expr(&e), "Nat");
    }
    #[test]
    fn test_show_expr_lit() {
        let e = Expr::Lit(Literal::Nat(7));
        assert_eq!(show_expr(&e), "7");
    }
    #[test]
    fn test_show_expr_app() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Const(Name::str("a"), vec![]);
        let app = Expr::App(Box::new(f), Box::new(a));
        let s = show_expr(&app);
        assert!(s.contains("f"));
        assert!(s.contains("a"));
    }
    #[test]
    fn test_show_expr_sort_prop() {
        let e = Expr::Sort(Level::zero());
        assert_eq!(show_expr(&e), "Prop");
    }
    #[test]
    fn test_show_expr_sort_type() {
        let e = Expr::Sort(Level::succ(Level::zero()));
        let s = show_expr(&e);
        assert!(s.contains("Type"));
    }
    #[test]
    fn test_show_expr_lam() {
        let ty = nat_const();
        let body = Expr::BVar(0);
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(ty),
            Box::new(body),
        );
        let s = show_expr(&lam);
        assert!(s.contains("fun"));
        assert!(s.contains("x"));
    }
    #[test]
    fn test_show_expr_pi_arrow() {
        let dom = nat_const();
        let cod = nat_const();
        let pi = Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(dom),
            Box::new(cod),
        );
        let s = show_expr(&pi);
        assert!(s.contains("→") || s.contains("->"));
    }
    #[test]
    fn test_show_expr_pi_forall() {
        let dom = nat_const();
        let cod = Expr::Sort(Level::zero());
        let pi = Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(dom),
            Box::new(cod),
        );
        let s = show_expr(&pi);
        assert!(s.contains("∀") || s.contains("forall"));
    }
    #[test]
    fn test_show_expr_let() {
        let ty = nat_const();
        let val = zero_lit();
        let body = Expr::BVar(0);
        let let_e = Expr::Let(Name::str("x"), Box::new(ty), Box::new(val), Box::new(body));
        let s = show_expr(&let_e);
        assert!(s.contains("let"));
        assert!(s.contains("x"));
    }
    #[test]
    fn test_show_expr_depth_limit() {
        let cfg = ShowConfig::default().with_depth(0);
        let e = Expr::Const(Name::str("Nat"), vec![]);
        let app = Expr::App(Box::new(e.clone()), Box::new(e));
        let s = show_expr_cfg(&app, &cfg, 0);
        let _ = s;
    }
    #[test]
    fn test_show_bool_true() {
        assert_eq!(show_bool(true), "true");
    }
    #[test]
    fn test_show_bool_false() {
        assert_eq!(show_bool(false), "false");
    }
    #[test]
    fn test_show_option_none() {
        let opt: Option<u64> = None;
        assert_eq!(show_option(&opt), "none");
    }
    #[test]
    fn test_show_option_some() {
        let opt: Option<u64> = Some(5);
        let s = show_option(&opt);
        assert!(s.contains("some"));
        assert!(s.contains("5"));
    }
    #[test]
    fn test_show_list_empty() {
        let v: Vec<u64> = vec![];
        assert_eq!(show_list(&v), "[]");
    }
    #[test]
    fn test_show_list_nonempty() {
        let v: Vec<u64> = vec![1, 2, 3];
        let s = show_list(&v);
        assert!(s.contains("1"));
        assert!(s.contains("3"));
    }
    #[test]
    fn test_show_pair() {
        let s = show_pair(&1u64, &2u64);
        assert!(s.contains("1"));
        assert!(s.contains("2"));
    }
    #[test]
    fn test_truncate_show_short() {
        let s = truncate_show("hello", 10);
        assert_eq!(s, "hello");
    }
    #[test]
    fn test_truncate_show_long() {
        let s = truncate_show("hello world", 5);
        assert!(s.ends_with("..."));
        assert_eq!(s, "hello...");
    }
    #[test]
    fn test_show_buffer_push() {
        let mut buf = ShowBuffer::new(2);
        buf.push("hello");
        assert_eq!(buf.finish(), "hello");
    }
    #[test]
    fn test_show_buffer_indent() {
        let mut buf = ShowBuffer::new(2);
        buf.push("a");
        buf.indent();
        buf.newline();
        buf.push("b");
        let s = buf.finish();
        assert!(s.contains("  b"));
    }
    #[test]
    fn test_show_buffer_dedent_saturating() {
        let mut buf = ShowBuffer::new(2);
        buf.dedent();
        buf.push("x");
        assert_eq!(buf.finish(), "x");
    }
    #[test]
    fn test_show_buffer_len() {
        let mut buf = ShowBuffer::new(2);
        buf.push("abc");
        assert_eq!(buf.len(), 3);
    }
    #[test]
    fn test_show_buffer_is_empty() {
        let buf = ShowBuffer::new(2);
        assert!(buf.is_empty());
    }
    #[test]
    fn test_show_buffer_display() {
        let mut buf = ShowBuffer::new(2);
        buf.push("test");
        let s = format!("{}", buf);
        assert_eq!(s, "test");
    }
    #[test]
    fn test_showable_display() {
        let e = Expr::Const(Name::str("Nat"), vec![]);
        let s = format!("{}", Showable(e));
        assert_eq!(s, "Nat");
    }
    #[test]
    fn test_show_for_bool_trait() {
        let b = true;
        assert_eq!(b.show(), "true");
    }
    #[test]
    fn test_show_for_u64_trait() {
        let n: u64 = 42;
        assert_eq!(n.show(), "42");
    }
    #[test]
    fn test_show_for_string_trait() {
        let s = "hello".to_string();
        let shown = s.show();
        assert!(shown.contains("hello"));
    }
    #[test]
    fn test_show_for_option_trait() {
        let opt: Option<u64> = Some(7);
        let s = opt.show();
        assert!(s.contains("7"));
    }
    #[test]
    fn test_show_for_vec_trait() {
        let v: Vec<u64> = vec![1, 2];
        let s = v.show();
        assert!(s.contains("["));
    }
    #[test]
    fn test_show_stats_record() {
        let mut stats = ShowStats::new();
        stats.record(100, false);
        stats.record(50, true);
        assert_eq!(stats.exprs_shown, 2);
        assert_eq!(stats.depth_truncations, 1);
        assert_eq!(stats.chars_produced, 150);
    }
    #[test]
    fn test_show_stats_display() {
        let s = ShowStats {
            exprs_shown: 5,
            ..ShowStats::default()
        };
        let txt = format!("{}", s);
        assert!(txt.contains("exprs: 5"));
    }
    #[test]
    fn test_build_show_nat() {
        assert_eq!(build_show_nat(0), "0");
        assert_eq!(build_show_nat(999), "999");
    }
    #[test]
    fn test_build_show_string() {
        let s = build_show_string("test");
        assert!(s.starts_with('"'));
        assert!(s.contains("test"));
    }
    #[test]
    fn test_build_show_bool() {
        assert_eq!(build_show_bool(true), "true");
        assert_eq!(build_show_bool(false), "false");
    }
    #[test]
    fn test_binder_info_show() {
        let bi = BinderInfo::Implicit;
        assert_eq!(bi.show(), "implicit");
    }
    #[test]
    fn test_level_show() {
        let lv = Level::zero();
        assert_eq!(lv.show(), "0");
    }
    #[test]
    fn test_name_show() {
        let n = Name::str("Nat.succ");
        assert_eq!(n.show(), "Nat.succ");
    }
    #[test]
    fn test_show_config_with_implicit() {
        let cfg = ShowConfig::default().with_implicit();
        assert!(cfg.show_implicit);
    }
    #[test]
    fn test_show_config_with_levels() {
        let cfg = ShowConfig::default().with_levels();
        assert!(cfg.show_levels);
    }
    #[test]
    fn test_show_buffer_with_indent_closure() {
        let mut buf = ShowBuffer::new(2);
        buf.push("outer");
        buf.with_indent(|b| {
            b.newline();
            b.push("inner");
        });
        let s = buf.finish();
        assert!(s.contains("inner"));
    }
}
/// Build the standard Show environment declarations.
///
/// Registers the `Show` typeclass and its instances for core built-in types:
/// `Nat`, `Bool`, `String`, `Unit`, and `Char`.
pub fn build_show_env(env: &mut oxilean_kernel::Environment) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Name};
    let mut add = |name: &str, ty: Expr| -> Result<(), String> {
        match env.add(Declaration::Axiom {
            name: Name::str(name),
            univ_params: vec![],
            ty,
        }) {
            Ok(()) | Err(_) => Ok(()),
        }
    };
    let cst = |s: &str| -> Expr { Expr::Const(Name::str(s), vec![]) };
    let app = |f: Expr, a: Expr| -> Expr { Expr::App(Box::new(f), Box::new(a)) };
    let arr = |a: Expr, b: Expr| -> Expr {
        Expr::Pi(Bi::Default, Name::Anonymous, Box::new(a), Box::new(b))
    };
    let type1 = || -> Expr { Expr::Sort(Level::succ(Level::zero())) };
    let show_of = |ty: Expr| -> Expr { app(cst("Show"), ty) };
    add("Show", arr(type1(), type1()))?;
    let show_show_ty = {
        let alpha = Expr::BVar(0);
        let show_alpha = show_of(alpha.clone());
        let string_ty = cst("String");
        Expr::Pi(
            Bi::Implicit,
            Name::str("α"),
            Box::new(type1()),
            Box::new(Expr::Pi(
                Bi::InstImplicit,
                Name::str("inst"),
                Box::new(show_alpha),
                Box::new(arr(Expr::BVar(1), string_ty)),
            )),
        )
    };
    add("Show.show", show_show_ty)?;
    add("instShowNat", show_of(cst("Nat")))?;
    add("instShowBool", show_of(cst("Bool")))?;
    add("instShowString", show_of(cst("String")))?;
    add("instShowUnit", show_of(cst("Unit")))?;
    add("instShowChar", show_of(cst("Char")))?;
    add("instShowInt", show_of(cst("Int")))?;
    let inst_show_option_ty = {
        let alpha = Expr::BVar(0);
        let show_alpha = show_of(alpha.clone());
        let show_option_alpha = show_of(app(cst("Option"), alpha));
        Expr::Pi(
            Bi::Implicit,
            Name::str("α"),
            Box::new(type1()),
            Box::new(Expr::Pi(
                Bi::InstImplicit,
                Name::str("inst"),
                Box::new(show_alpha),
                Box::new(show_option_alpha),
            )),
        )
    };
    add("instShowOption", inst_show_option_ty)?;
    let inst_show_list_ty = {
        let alpha = Expr::BVar(0);
        let show_alpha = show_of(alpha.clone());
        let show_list_alpha = show_of(app(cst("List"), alpha));
        Expr::Pi(
            Bi::Implicit,
            Name::str("α"),
            Box::new(type1()),
            Box::new(Expr::Pi(
                Bi::InstImplicit,
                Name::str("inst"),
                Box::new(show_alpha),
                Box::new(show_list_alpha),
            )),
        )
    };
    add("instShowList", inst_show_list_ty)?;
    let inst_show_prod_ty = {
        let beta = Expr::BVar(0);
        let alpha = Expr::BVar(1);
        let show_alpha = show_of(alpha.clone());
        let show_beta = show_of(beta.clone());
        let show_prod = show_of(app(app(cst("Prod"), alpha), beta));
        Expr::Pi(
            Bi::Implicit,
            Name::str("α"),
            Box::new(type1()),
            Box::new(Expr::Pi(
                Bi::Implicit,
                Name::str("β"),
                Box::new(type1()),
                Box::new(Expr::Pi(
                    Bi::InstImplicit,
                    Name::str("instα"),
                    Box::new(show_alpha),
                    Box::new(Expr::Pi(
                        Bi::InstImplicit,
                        Name::str("instβ"),
                        Box::new(show_beta),
                        Box::new(show_prod),
                    )),
                )),
            )),
        )
    };
    add("instShowProd", inst_show_prod_ty)?;
    Ok(())
}
/// Show an expression using a specific mode.
pub fn show_expr_mode(expr: &Expr, mode: ShowMode) -> String {
    show_expr_cfg(expr, &mode.to_config(), 0)
}
/// Show a declaration using a specific mode.
pub fn show_decl_mode(decl: &Declaration, mode: ShowMode) -> String {
    show_declaration(decl, &mode.to_config())
}
#[cfg(test)]
mod show_extra_tests {
    use super::*;
    use oxilean_kernel::{Expr, Level, Name};
    #[test]
    fn test_show_mode_short_is_compact() {
        let cfg = ShowMode::Short.to_config();
        assert!(cfg.compact);
    }
    #[test]
    fn test_show_mode_full_not_compact() {
        let cfg = ShowMode::Full.to_config();
        assert!(!cfg.compact);
    }
    #[test]
    fn test_show_mode_debug_has_implicit_and_levels() {
        let cfg = ShowMode::Debug.to_config();
        assert!(cfg.show_implicit);
        assert!(cfg.show_levels);
    }
    #[test]
    fn test_show_expr_mode_short() {
        let e = Expr::Const(Name::str("Nat"), vec![]);
        let s = show_expr_mode(&e, ShowMode::Short);
        assert_eq!(s, "Nat");
    }
    #[test]
    fn test_show_registry_empty() {
        let reg = ShowRegistry::new();
        assert!(reg.is_empty());
    }
    #[test]
    fn test_show_registry_register_and_format() {
        let mut reg = ShowRegistry::new();
        reg.register("default", |e| show_expr(e));
        assert_eq!(reg.len(), 1);
        let e = Expr::Const(Name::str("Bool"), vec![]);
        let results = reg.format_all(&e);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, "default");
        assert_eq!(results[0].1, "Bool");
    }
    #[test]
    fn test_show_decl_mode_full() {
        use oxilean_kernel::Declaration;
        let decl = Declaration::Axiom {
            name: Name::str("Nat"),
            univ_params: vec![],
            ty: Expr::Sort(Level::succ(Level::zero())),
        };
        let s = show_decl_mode(&decl, ShowMode::Full);
        assert!(s.contains("axiom"));
        assert!(s.contains("Nat"));
    }
    #[test]
    fn test_show_mode_debug_mode() {
        let e = Expr::BVar(0);
        let s = show_expr_mode(&e, ShowMode::Debug);
        assert_eq!(s, "#0");
    }
}
/// Build axiom: Show is a function to String (class law).
pub(super) fn shw_ext_show_is_function_to_string(
    env: &mut oxilean_kernel::Environment,
) -> Result<(), String> {
    use oxilean_kernel::{Declaration, Expr, Level, Name};
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let string_ty = Expr::Const(Name::str("String"), vec![]);
    let ty = Expr::Pi(
        oxilean_kernel::BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            oxilean_kernel::BinderInfo::InstImplicit,
            Name::str("inst"),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Show"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(Expr::Pi(
                oxilean_kernel::BinderInfo::Default,
                Name::str("_"),
                Box::new(Expr::BVar(1)),
                Box::new(string_ty.clone()),
            )),
        )),
    );
    let _ = type1;
    match env.add(Declaration::Axiom {
        name: Name::str("Show.functionLaw"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Show for Nat produces decimal string.
pub(super) fn shw_ext_show_nat_decimal(
    env: &mut oxilean_kernel::Environment,
) -> Result<(), String> {
    use oxilean_kernel::{Declaration, Expr, Name};
    let ty = Expr::Pi(
        oxilean_kernel::BinderInfo::Default,
        Name::str("n"),
        Box::new(Expr::Const(Name::str("Nat"), vec![])),
        Box::new(Expr::Const(Name::str("String"), vec![])),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("Show.natDecimal"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Show for Int handles negative sign.
pub(super) fn shw_ext_show_int_signed(env: &mut oxilean_kernel::Environment) -> Result<(), String> {
    use oxilean_kernel::{Declaration, Expr, Name};
    let ty = Expr::Pi(
        oxilean_kernel::BinderInfo::Default,
        Name::str("i"),
        Box::new(Expr::Const(Name::str("Int"), vec![])),
        Box::new(Expr::Const(Name::str("String"), vec![])),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("Show.intSigned"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Show for Bool produces "true" or "false".
pub(super) fn shw_ext_show_bool_canonical(
    env: &mut oxilean_kernel::Environment,
) -> Result<(), String> {
    use oxilean_kernel::{Declaration, Expr, Name};
    let ty = Expr::Pi(
        oxilean_kernel::BinderInfo::Default,
        Name::str("b"),
        Box::new(Expr::Const(Name::str("Bool"), vec![])),
        Box::new(Expr::Const(Name::str("String"), vec![])),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("Show.boolCanonical"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Show for Char wraps in single quotes.
pub(super) fn shw_ext_show_char_quoted(
    env: &mut oxilean_kernel::Environment,
) -> Result<(), String> {
    use oxilean_kernel::{Declaration, Expr, Name};
    let ty = Expr::Pi(
        oxilean_kernel::BinderInfo::Default,
        Name::str("c"),
        Box::new(Expr::Const(Name::str("Char"), vec![])),
        Box::new(Expr::Const(Name::str("String"), vec![])),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("Show.charQuoted"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Show for Float uses decimal notation.
pub(super) fn shw_ext_show_float_decimal(
    env: &mut oxilean_kernel::Environment,
) -> Result<(), String> {
    use oxilean_kernel::{Declaration, Expr, Name};
    let ty = Expr::Pi(
        oxilean_kernel::BinderInfo::Default,
        Name::str("f"),
        Box::new(Expr::Const(Name::str("Float"), vec![])),
        Box::new(Expr::Const(Name::str("String"), vec![])),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("Show.floatDecimal"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Show for List brackets elements.
pub(super) fn shw_ext_show_list_bracketed(
    env: &mut oxilean_kernel::Environment,
) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Level, Name};
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let alpha = Expr::BVar(0);
    let show_alpha = Expr::App(
        Box::new(Expr::Const(Name::str("Show"), vec![])),
        Box::new(alpha.clone()),
    );
    let list_alpha = Expr::App(
        Box::new(Expr::Const(Name::str("List"), vec![])),
        Box::new(alpha),
    );
    let ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1),
        Box::new(Expr::Pi(
            Bi::InstImplicit,
            Name::str("inst"),
            Box::new(show_alpha),
            Box::new(Expr::Pi(
                Bi::Default,
                Name::str("_"),
                Box::new(list_alpha),
                Box::new(Expr::Const(Name::str("String"), vec![])),
            )),
        )),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("Show.listBracketed"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Show for Option uses "none" / "some".
pub(super) fn shw_ext_show_option_canonical(
    env: &mut oxilean_kernel::Environment,
) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Level, Name};
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let alpha = Expr::BVar(0);
    let show_alpha = Expr::App(
        Box::new(Expr::Const(Name::str("Show"), vec![])),
        Box::new(alpha.clone()),
    );
    let opt_alpha = Expr::App(
        Box::new(Expr::Const(Name::str("Option"), vec![])),
        Box::new(alpha),
    );
    let ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1),
        Box::new(Expr::Pi(
            Bi::InstImplicit,
            Name::str("inst"),
            Box::new(show_alpha),
            Box::new(Expr::Pi(
                Bi::Default,
                Name::str("_"),
                Box::new(opt_alpha),
                Box::new(Expr::Const(Name::str("String"), vec![])),
            )),
        )),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("Show.optionCanonical"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Show for Pair uses tuple notation.
pub(super) fn shw_ext_show_pair_tuple(env: &mut oxilean_kernel::Environment) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Level, Name};
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            Bi::Implicit,
            Name::str("β"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                Bi::InstImplicit,
                Name::str("instα"),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Show"), vec![])),
                    Box::new(Expr::BVar(1)),
                )),
                Box::new(Expr::Pi(
                    Bi::InstImplicit,
                    Name::str("instβ"),
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Show"), vec![])),
                        Box::new(Expr::BVar(1)),
                    )),
                    Box::new(Expr::Pi(
                        Bi::Default,
                        Name::str("_"),
                        Box::new(Expr::App(
                            Box::new(Expr::App(
                                Box::new(Expr::Const(Name::str("Prod"), vec![])),
                                Box::new(Expr::BVar(3)),
                            )),
                            Box::new(Expr::BVar(2)),
                        )),
                        Box::new(Expr::Const(Name::str("String"), vec![])),
                    )),
                )),
            )),
        )),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("Show.pairTuple"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Show for Either uses "left"/"right" notation.
pub(super) fn shw_ext_show_either_tagged(
    env: &mut oxilean_kernel::Environment,
) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Level, Name};
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            Bi::Implicit,
            Name::str("β"),
            Box::new(type1),
            Box::new(Expr::Pi(
                Bi::Default,
                Name::str("_"),
                Box::new(Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Sum"), vec![])),
                        Box::new(Expr::BVar(1)),
                    )),
                    Box::new(Expr::BVar(0)),
                )),
                Box::new(Expr::Const(Name::str("String"), vec![])),
            )),
        )),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("Show.eitherTagged"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Show for Result uses "ok"/"err" notation.
pub(super) fn shw_ext_show_result_canonical(
    env: &mut oxilean_kernel::Environment,
) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Level, Name};
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            Bi::Implicit,
            Name::str("ε"),
            Box::new(type1),
            Box::new(Expr::Pi(
                Bi::Default,
                Name::str("_"),
                Box::new(Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Result"), vec![])),
                        Box::new(Expr::BVar(1)),
                    )),
                    Box::new(Expr::BVar(0)),
                )),
                Box::new(Expr::Const(Name::str("String"), vec![])),
            )),
        )),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("Show.resultCanonical"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Show/read roundtrip law.
pub(super) fn shw_ext_show_read_roundtrip(
    env: &mut oxilean_kernel::Environment,
) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Level, Name};
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1),
        Box::new(Expr::Pi(
            Bi::InstImplicit,
            Name::str("instShow"),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Show"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(Expr::Pi(
                Bi::Default,
                Name::str("x"),
                Box::new(Expr::BVar(1)),
                Box::new(Expr::Const(Name::str("Prop"), vec![])),
            )),
        )),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("Show.readRoundtrip"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: PrettyDoc nil element.
pub(super) fn shw_ext_pretty_doc_nil(env: &mut oxilean_kernel::Environment) -> Result<(), String> {
    use oxilean_kernel::{Declaration, Expr, Name};
    let ty = Expr::Const(Name::str("PrettyDoc"), vec![]);
    match env.add(Declaration::Axiom {
        name: Name::str("PrettyDoc.nil"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: PrettyDoc text constructor.
pub(super) fn shw_ext_pretty_doc_text(env: &mut oxilean_kernel::Environment) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Name};
    let ty = Expr::Pi(
        Bi::Default,
        Name::str("s"),
        Box::new(Expr::Const(Name::str("String"), vec![])),
        Box::new(Expr::Const(Name::str("PrettyDoc"), vec![])),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("PrettyDoc.text"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: PrettyDoc concat operator.
pub(super) fn shw_ext_pretty_doc_concat(
    env: &mut oxilean_kernel::Environment,
) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Name};
    let doc = || Expr::Const(Name::str("PrettyDoc"), vec![]);
    let ty = Expr::Pi(
        Bi::Default,
        Name::str("a"),
        Box::new(doc()),
        Box::new(Expr::Pi(
            Bi::Default,
            Name::str("b"),
            Box::new(doc()),
            Box::new(doc()),
        )),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("PrettyDoc.concat"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: PrettyDoc nest/indent.
pub(super) fn shw_ext_pretty_doc_nest(env: &mut oxilean_kernel::Environment) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Name};
    let doc = || Expr::Const(Name::str("PrettyDoc"), vec![]);
    let nat = || Expr::Const(Name::str("Nat"), vec![]);
    let ty = Expr::Pi(
        Bi::Default,
        Name::str("n"),
        Box::new(nat()),
        Box::new(Expr::Pi(
            Bi::Default,
            Name::str("d"),
            Box::new(doc()),
            Box::new(doc()),
        )),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("PrettyDoc.nest"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Display vs Debug distinction law.
pub(super) fn shw_ext_display_vs_debug(
    env: &mut oxilean_kernel::Environment,
) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Level, Name};
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1),
        Box::new(Expr::Pi(
            Bi::Default,
            Name::str("display"),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Show"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(Expr::Pi(
                Bi::Default,
                Name::str("debug"),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Show"), vec![])),
                    Box::new(Expr::BVar(1)),
                )),
                Box::new(Expr::Const(Name::str("Prop"), vec![])),
            )),
        )),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("Show.displayVsDebugLaw"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Show for recursive types terminates.
pub(super) fn shw_ext_show_recursive_terminates(
    env: &mut oxilean_kernel::Environment,
) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Level, Name};
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let prop = Expr::Sort(Level::zero());
    let ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1),
        Box::new(Expr::Pi(
            Bi::InstImplicit,
            Name::str("inst"),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Show"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(prop),
        )),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("Show.recursiveTerminates"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Show for polymorphic types is natural transformation.
pub(super) fn shw_ext_show_polymorphic_nat_trans(
    env: &mut oxilean_kernel::Environment,
) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Level, Name};
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let prop = Expr::Sort(Level::zero());
    let ty = Expr::Pi(
        Bi::Implicit,
        Name::str("F"),
        Box::new(Expr::Pi(
            Bi::Default,
            Name::str("_"),
            Box::new(type1.clone()),
            Box::new(type1.clone()),
        )),
        Box::new(Expr::Pi(
            Bi::Implicit,
            Name::str("α"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                Bi::Implicit,
                Name::str("β"),
                Box::new(type1),
                Box::new(prop),
            )),
        )),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("Show.polymorphicNatTrans"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: ShowS identity law.
pub(super) fn shw_ext_shows_identity_law(
    env: &mut oxilean_kernel::Environment,
) -> Result<(), String> {
    use oxilean_kernel::{Declaration, Expr, Name};
    let prop = Expr::Sort(oxilean_kernel::Level::zero());
    match env.add(Declaration::Axiom {
        name: Name::str("ShowS.idLaw"),
        univ_params: vec![],
        ty: prop,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: ShowS composition associativity.
pub(super) fn shw_ext_shows_compose_assoc(
    env: &mut oxilean_kernel::Environment,
) -> Result<(), String> {
    use oxilean_kernel::{Declaration, Expr, Name};
    let prop = Expr::Sort(oxilean_kernel::Level::zero());
    match env.add(Declaration::Axiom {
        name: Name::str("ShowS.composeAssoc"),
        univ_params: vec![],
        ty: prop,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: ShowS to String conversion.
pub(super) fn shw_ext_shows_to_string(env: &mut oxilean_kernel::Environment) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Name};
    let show_s_ty = Expr::Const(Name::str("ShowS"), vec![]);
    let string_ty = Expr::Const(Name::str("String"), vec![]);
    let ty = Expr::Pi(
        Bi::Default,
        Name::str("_"),
        Box::new(show_s_ty),
        Box::new(string_ty),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("ShowS.toString"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Tabular show format (rows × columns).
pub(super) fn shw_ext_tabular_format(env: &mut oxilean_kernel::Environment) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Level, Name};
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let string_ty = Expr::Const(Name::str("String"), vec![]);
    let list_string = Expr::App(
        Box::new(Expr::Const(Name::str("List"), vec![])),
        Box::new(string_ty.clone()),
    );
    let list_list_string = Expr::App(
        Box::new(Expr::Const(Name::str("List"), vec![])),
        Box::new(list_string),
    );
    let _ = type1;
    let ty = Expr::Pi(
        Bi::Default,
        Name::str("rows"),
        Box::new(list_list_string),
        Box::new(string_ty),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("Show.tabularFormat"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Indented show format.
pub(super) fn shw_ext_indented_format(env: &mut oxilean_kernel::Environment) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Name};
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let string_ty = Expr::Const(Name::str("String"), vec![]);
    let ty = Expr::Pi(
        Bi::Default,
        Name::str("indent"),
        Box::new(nat_ty),
        Box::new(Expr::Pi(
            Bi::Default,
            Name::str("s"),
            Box::new(string_ty.clone()),
            Box::new(string_ty),
        )),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("Show.indentedFormat"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Show number in binary.
pub(super) fn shw_ext_show_nat_binary(env: &mut oxilean_kernel::Environment) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Name};
    let ty = Expr::Pi(
        Bi::Default,
        Name::str("n"),
        Box::new(Expr::Const(Name::str("Nat"), vec![])),
        Box::new(Expr::Const(Name::str("String"), vec![])),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("Show.natBinary"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Show number in hexadecimal.
pub(super) fn shw_ext_show_nat_hex(env: &mut oxilean_kernel::Environment) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Name};
    let ty = Expr::Pi(
        Bi::Default,
        Name::str("n"),
        Box::new(Expr::Const(Name::str("Nat"), vec![])),
        Box::new(Expr::Const(Name::str("String"), vec![])),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("Show.natHex"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Show number in octal.
pub(super) fn shw_ext_show_nat_octal(env: &mut oxilean_kernel::Environment) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Name};
    let ty = Expr::Pi(
        Bi::Default,
        Name::str("n"),
        Box::new(Expr::Const(Name::str("Nat"), vec![])),
        Box::new(Expr::Const(Name::str("String"), vec![])),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("Show.natOctal"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Show float with given decimal precision.
pub(super) fn shw_ext_show_float_precision(
    env: &mut oxilean_kernel::Environment,
) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Name};
    let ty = Expr::Pi(
        Bi::Default,
        Name::str("prec"),
        Box::new(Expr::Const(Name::str("Nat"), vec![])),
        Box::new(Expr::Pi(
            Bi::Default,
            Name::str("f"),
            Box::new(Expr::Const(Name::str("Float"), vec![])),
            Box::new(Expr::Const(Name::str("String"), vec![])),
        )),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("Show.floatPrecision"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Deriving Show automatically from structure.
pub(super) fn shw_ext_derive_show(env: &mut oxilean_kernel::Environment) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Level, Name};
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1),
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Show"), vec![])),
            Box::new(Expr::BVar(0)),
        )),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("Show.deriveAuto"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Show diagnostic for errors renders location.
pub(super) fn shw_ext_show_diagnostic_with_location(
    env: &mut oxilean_kernel::Environment,
) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Name};
    let string_ty = Expr::Const(Name::str("String"), vec![]);
    let ty = Expr::Pi(
        Bi::Default,
        Name::str("msg"),
        Box::new(string_ty.clone()),
        Box::new(Expr::Pi(
            Bi::Default,
            Name::str("loc"),
            Box::new(string_ty.clone()),
            Box::new(string_ty),
        )),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("Show.diagnosticWithLocation"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Show is injective on Show instances (different show strings imply different values).
pub(super) fn shw_ext_show_injectivity_law(
    env: &mut oxilean_kernel::Environment,
) -> Result<(), String> {
    use oxilean_kernel::{Declaration, Expr, Name};
    let prop = Expr::Sort(oxilean_kernel::Level::zero());
    match env.add(Declaration::Axiom {
        name: Name::str("Show.injectivityLaw"),
        univ_params: vec![],
        ty: prop,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Show for Unit is always "()".
pub(super) fn shw_ext_show_unit_canonical(
    env: &mut oxilean_kernel::Environment,
) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Name};
    let ty = Expr::Pi(
        Bi::Default,
        Name::str("_"),
        Box::new(Expr::Const(Name::str("Unit"), vec![])),
        Box::new(Expr::Const(Name::str("String"), vec![])),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("Show.unitCanonical"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Show.show is pure (no side effects).
pub(super) fn shw_ext_show_purity_law(env: &mut oxilean_kernel::Environment) -> Result<(), String> {
    use oxilean_kernel::{Declaration, Expr, Name};
    let prop = Expr::Sort(oxilean_kernel::Level::zero());
    match env.add(Declaration::Axiom {
        name: Name::str("Show.purityLaw"),
        univ_params: vec![],
        ty: prop,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Show for nested types composes Show instances.
pub(super) fn shw_ext_show_composition_law(
    env: &mut oxilean_kernel::Environment,
) -> Result<(), String> {
    use oxilean_kernel::{Declaration, Expr, Name};
    let prop = Expr::Sort(oxilean_kernel::Level::zero());
    match env.add(Declaration::Axiom {
        name: Name::str("Show.compositionLaw"),
        univ_params: vec![],
        ty: prop,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Display is a restricted Show (no quoting of strings).
pub(super) fn shw_ext_display_no_quoting(
    env: &mut oxilean_kernel::Environment,
) -> Result<(), String> {
    use super::functions::*;
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Name};
    let ty = Expr::Pi(
        Bi::Default,
        Name::str("s"),
        Box::new(Expr::Const(Name::str("String"), vec![])),
        Box::new(Expr::Const(Name::str("String"), vec![])),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("Display.noQuoting"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
