//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::{BinderInfo, Expr, Level, Name};

use super::types::{
    AnnotationTable, BiMap, ColorScheme, DiagMeta, Doc, DocBuilder, EscapeHelper, EventCounter,
    ExprPrinter, FmtWidth, FrequencyTable, IdDispenser, IndentStyle, IntervalSet, LoopClock,
    MemoSlot, PrettyConfig, PrettyDoc, PrettyPrinterState, PrettyTable, PrettyToken, PrintConfig,
    SExprPrinter, ScopeStack, SimpleLruCache, Slot, SparseBitSet, StringInterner, Timestamp,
    TokenPrinter, TypedId, WorkQueue, WorkStack,
};

/// Convert a level to a natural number if possible.
pub(super) fn level_to_nat(level: &Level) -> Option<u32> {
    match level {
        Level::Zero => Some(0),
        Level::Succ(l) => level_to_nat(l).map(|n| n + 1),
        _ => None,
    }
}
/// Decompose a level into (base, offset) where level = succ^offset(base).
pub(super) fn level_to_offset(level: &Level) -> (&Level, u32) {
    match level {
        Level::Succ(l) => {
            let (base, offset) = level_to_offset(l);
            (base, offset + 1)
        }
        _ => (level, 0),
    }
}
/// Collect function head and arguments from nested application.
pub(super) fn collect_app_args(expr: &Expr) -> (&Expr, Vec<&Expr>) {
    let mut args = Vec::new();
    let mut e = expr;
    while let Expr::App(f, a) = e {
        args.push(a.as_ref());
        e = f;
    }
    args.reverse();
    (e, args)
}
/// Pretty print an expression with unicode symbols.
pub fn print_expr(expr: &Expr) -> String {
    let mut printer = ExprPrinter::new();
    printer
        .print(expr)
        .expect("pretty-printer must succeed on valid expression");
    printer.output()
}
/// Pretty print an expression without unicode symbols.
pub fn print_expr_ascii(expr: &Expr) -> String {
    let mut printer = ExprPrinter::new().with_unicode(false);
    printer
        .print(expr)
        .expect("pretty-printer must succeed on valid expression");
    printer.output()
}
/// Pretty print with a specific configuration.
pub fn print_expr_with_config(expr: &Expr, config: PrintConfig) -> String {
    let mut printer = ExprPrinter::with_config(config);
    printer
        .print(expr)
        .expect("pretty-printer must succeed on valid expression");
    printer.output()
}
/// Pretty print a level expression.
pub fn print_level(level: &Level) -> String {
    let mut printer = ExprPrinter::new();
    printer
        .print_level(level)
        .expect("pretty-printer must succeed on valid level");
    printer.output()
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::Literal;
    #[test]
    fn test_print_sort_prop() {
        let expr = Expr::Sort(Level::zero());
        let output = print_expr(&expr);
        assert_eq!(output, "Prop");
    }
    #[test]
    fn test_print_sort_type() {
        let expr = Expr::Sort(Level::succ(Level::zero()));
        let output = print_expr(&expr);
        assert_eq!(output, "Type");
    }
    #[test]
    fn test_print_sort_type_n() {
        let expr = Expr::Sort(Level::succ(Level::succ(Level::zero())));
        let output = print_expr(&expr);
        assert_eq!(output, "Type 1");
    }
    #[test]
    fn test_print_sort_param() {
        let expr = Expr::Sort(Level::param(Name::str("u")));
        let output = print_expr(&expr);
        assert_eq!(output, "Sort u");
    }
    #[test]
    fn test_print_bvar() {
        let expr = Expr::BVar(0);
        assert_eq!(print_expr(&expr), "#0");
    }
    #[test]
    fn test_print_const() {
        let expr = Expr::Const(Name::str("Nat"), vec![]);
        assert_eq!(print_expr(&expr), "Nat");
    }
    #[test]
    fn test_print_const_with_levels() {
        let expr = Expr::Const(Name::str("List"), vec![Level::succ(Level::zero())]);
        let config = PrintConfig::verbose();
        let output = print_expr_with_config(&expr, config);
        assert!(output.contains("List"));
        assert!(output.contains("1"));
    }
    #[test]
    fn test_print_lit() {
        let expr = Expr::Lit(Literal::Nat(42));
        assert_eq!(print_expr(&expr), "42");
    }
    #[test]
    fn test_print_app() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Lit(Literal::Nat(1));
        let app = Expr::App(Box::new(f), Box::new(a));
        let output = print_expr(&app);
        assert!(output.contains("f"));
        assert!(output.contains("1"));
    }
    #[test]
    fn test_print_lambda_unicode() {
        let ty = Expr::Sort(Level::zero());
        let body = Expr::BVar(0);
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(ty),
            Box::new(body),
        );
        let output = print_expr(&lam);
        assert!(output.contains("λ"));
        assert!(output.contains("x"));
    }
    #[test]
    fn test_print_lambda_ascii() {
        let ty = Expr::Sort(Level::zero());
        let body = Expr::BVar(0);
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(ty),
            Box::new(body),
        );
        let output = print_expr_ascii(&lam);
        assert!(output.contains("fun"));
    }
    #[test]
    fn test_print_pi_arrow() {
        let ty = Expr::Sort(Level::succ(Level::zero()));
        let body = Expr::Sort(Level::succ(Level::zero()));
        let pi = Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(ty),
            Box::new(body),
        );
        let output = print_expr(&pi);
        assert!(output.contains("→"));
    }
    #[test]
    fn test_print_pi_forall() {
        let ty = Expr::Sort(Level::succ(Level::zero()));
        let body = Expr::BVar(0);
        let pi = Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(ty),
            Box::new(body),
        );
        let output = print_expr(&pi);
        assert!(output.contains("∀"));
        assert!(output.contains("x"));
    }
    #[test]
    fn test_print_implicit_binder() {
        let ty = Expr::Sort(Level::succ(Level::zero()));
        let body = Expr::BVar(0);
        let pi = Expr::Pi(
            BinderInfo::Implicit,
            Name::str("α"),
            Box::new(ty),
            Box::new(body),
        );
        let output = print_expr(&pi);
        assert!(output.contains("{"));
        assert!(output.contains("}"));
    }
    #[test]
    fn test_print_inst_implicit_binder() {
        let ty = Expr::Const(Name::str("Monad"), vec![]);
        let body = Expr::BVar(0);
        let pi = Expr::Pi(
            BinderInfo::InstImplicit,
            Name::str("m"),
            Box::new(ty),
            Box::new(body),
        );
        let output = print_expr(&pi);
        assert!(output.contains("["));
        assert!(output.contains("]"));
    }
    #[test]
    fn test_print_level_nat() {
        let level = Level::succ(Level::succ(Level::succ(Level::zero())));
        let output = print_level(&level);
        assert_eq!(output, "3");
    }
    #[test]
    fn test_print_level_param_plus() {
        let level = Level::succ(Level::param(Name::str("u")));
        let output = print_level(&level);
        assert_eq!(output, "u+1");
    }
    #[test]
    fn test_print_level_mvar() {
        let level = Level::MVar(crate::LevelMVarId(42));
        let output = print_level(&level);
        assert_eq!(output, "?u_42");
    }
    #[test]
    fn test_print_config_verbose() {
        let config = PrintConfig::verbose();
        assert!(config.show_implicit);
        assert!(config.show_universes);
    }
    #[test]
    fn test_print_config_ascii() {
        let config = PrintConfig::ascii();
        assert!(!config.unicode);
    }
}
/// Convert an `Expr` to a `Doc` for structured pretty printing.
#[allow(dead_code)]
pub fn expr_to_doc(expr: &Expr) -> Doc {
    expr_to_doc_prec(expr, 0)
}
pub(super) fn expr_to_doc_prec(expr: &Expr, prec: u32) -> Doc {
    match expr {
        Expr::Sort(level) => Doc::text(format!("Sort({})", level)),
        Expr::BVar(i) => Doc::text(format!("#{}", i)),
        Expr::FVar(id) => Doc::text(format!("@{}", id.0)),
        Expr::Const(name, _) => Doc::text(name.to_string()),
        Expr::Lit(lit) => Doc::text(format!("{}", lit)),
        Expr::App(f, a) => {
            let fd = expr_to_doc_prec(f, 10);
            let ad = expr_to_doc_prec(a, 11);
            let inner = fd.concat(Doc::text(" ")).concat(ad);
            if prec > 10 {
                Doc::text("(").concat(inner).concat(Doc::text(")"))
            } else {
                inner
            }
        }
        Expr::Lam(_, name, ty, body) => {
            let header = Doc::text(format!("fun ({} : ", name))
                .concat(expr_to_doc_prec(ty, 0))
                .concat(Doc::text(") -> "));
            let bd = expr_to_doc_prec(body, 0);
            let inner = header.concat(bd);
            if prec > 0 {
                Doc::text("(").concat(inner).concat(Doc::text(")"))
            } else {
                inner
            }
        }
        Expr::Pi(_, name, ty, body) => {
            if name.is_anonymous() || *name == Name::str("_") {
                let td = expr_to_doc_prec(ty, 25);
                let bd = expr_to_doc_prec(body, 24);
                let inner = td.concat(Doc::text(" -> ")).concat(bd);
                if prec > 0 {
                    Doc::text("(").concat(inner).concat(Doc::text(")"))
                } else {
                    inner
                }
            } else {
                let header = Doc::text(format!("forall ({} : ", name))
                    .concat(expr_to_doc_prec(ty, 0))
                    .concat(Doc::text("), "));
                let bd = expr_to_doc_prec(body, 0);
                let inner = header.concat(bd);
                if prec > 0 {
                    Doc::text("(").concat(inner).concat(Doc::text(")"))
                } else {
                    inner
                }
            }
        }
        Expr::Let(name, ty, val, body) => {
            let line1 = Doc::text(format!("let {} : ", name))
                .concat(expr_to_doc_prec(ty, 0))
                .concat(Doc::text(" := "))
                .concat(expr_to_doc_prec(val, 0));
            let line2 = Doc::text("in ").concat(expr_to_doc_prec(body, 0));
            Doc::Nest(2, Box::new(line1.concat(Doc::line()).concat(line2)))
        }
        Expr::Proj(name, idx, e) => {
            expr_to_doc_prec(e, 11).concat(Doc::text(format!(".{}.{}", name, idx)))
        }
    }
}
/// Pretty print a `Name` as a dot-separated string.
#[allow(dead_code)]
pub fn print_name_str(name: &Name) -> String {
    name.to_string()
}
/// Check if a name is a simple (single-component) name.
#[allow(dead_code)]
pub fn is_simple_name(name: &Name) -> bool {
    matches!(name, Name::Str(parent, _) if matches!(parent.as_ref(), Name::Anonymous))
}
/// Produce a one-line summary of an expression for debug output.
#[allow(dead_code)]
pub fn expr_summary(expr: &Expr) -> String {
    match expr {
        Expr::Sort(l) => format!("Sort({})", l),
        Expr::BVar(i) => format!("BVar({})", i),
        Expr::FVar(id) => format!("FVar({})", id.0),
        Expr::Const(n, _) => format!("Const({})", n),
        Expr::App(_, _) => {
            let (head, args) = collect_app_args(expr);
            format!("App({}, {} args)", expr_summary(head), args.len())
        }
        Expr::Lam(_, n, _, _) => format!("Lam({})", n),
        Expr::Pi(_, n, _, _) => {
            if n.is_anonymous() || *n == Name::str("_") {
                "Pi(->)".to_string()
            } else {
                format!("Pi(forall {})", n)
            }
        }
        Expr::Let(n, _, _, _) => format!("Let({})", n),
        Expr::Lit(l) => format!("Lit({})", l),
        Expr::Proj(n, i, _) => format!("Proj({}.{})", n, i),
    }
}
/// ANSI color codes for terminal output.
#[allow(dead_code)]
pub mod ansi {
    /// ANSI reset code.
    pub const RESET: &str = "\x1b[0m";
    /// ANSI bold code.
    pub const BOLD: &str = "\x1b[1m";
    /// ANSI dim code.
    pub const DIM: &str = "\x1b[2m";
    /// ANSI red color.
    pub const RED: &str = "\x1b[31m";
    /// ANSI green color.
    pub const GREEN: &str = "\x1b[32m";
    /// ANSI yellow color.
    pub const YELLOW: &str = "\x1b[33m";
    /// ANSI blue color.
    pub const BLUE: &str = "\x1b[34m";
    /// ANSI magenta color.
    pub const MAGENTA: &str = "\x1b[35m";
    /// ANSI cyan color.
    pub const CYAN: &str = "\x1b[36m";
}
/// Colorize an expression string for terminal display.
///
/// Keywords are highlighted in cyan, literals in yellow, types in blue.
#[allow(dead_code)]
pub fn colorize(expr: &Expr) -> String {
    match expr {
        Expr::Sort(_) => format!("{}{}{}", ansi::BLUE, print_expr(expr), ansi::RESET),
        Expr::Lit(_) => format!("{}{}{}", ansi::YELLOW, print_expr(expr), ansi::RESET),
        Expr::Const(_, _) => {
            format!("{}{}{}", ansi::GREEN, print_expr(expr), ansi::RESET)
        }
        _ => print_expr(expr),
    }
}
#[cfg(test)]
mod extra_prettyprint_tests {
    use super::*;
    use crate::Literal;
    #[test]
    fn test_doc_text_render() {
        let d = Doc::text("hello");
        assert_eq!(d.render(80), "hello");
    }
    #[test]
    fn test_doc_concat() {
        let d = Doc::text("foo").concat(Doc::text("bar"));
        assert_eq!(d.render(80), "foobar");
    }
    #[test]
    fn test_doc_line() {
        let d = Doc::text("a").concat(Doc::line()).concat(Doc::text("b"));
        let rendered = d.render(80);
        assert!(rendered.contains('\n'));
        assert!(rendered.contains('a'));
        assert!(rendered.contains('b'));
    }
    #[test]
    fn test_doc_nest() {
        let d = Doc::nest(4, Doc::text("x").concat(Doc::line()).concat(Doc::text("y")));
        let rendered = d.render(80);
        if let Some(pos) = rendered.find('\n') {
            let rest = &rendered[pos + 1..];
            assert!(rest.starts_with("    "));
        }
    }
    #[test]
    fn test_expr_to_doc_sort() {
        let e = Expr::Sort(Level::zero());
        let d = expr_to_doc(&e);
        let s = d.render(80);
        assert_eq!(s, "Sort(0)");
    }
    #[test]
    fn test_expr_to_doc_bvar() {
        let d = expr_to_doc(&Expr::BVar(3));
        assert_eq!(d.render(80), "#3");
    }
    #[test]
    fn test_expr_to_doc_const() {
        let d = expr_to_doc(&Expr::Const(Name::str("Nat"), vec![]));
        assert_eq!(d.render(80), "Nat");
    }
    #[test]
    fn test_expr_to_doc_app() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Lit(Literal::Nat(1));
        let app = Expr::App(Box::new(f), Box::new(a));
        let d = expr_to_doc(&app);
        let s = d.render(80);
        assert!(s.contains("f"));
        assert!(s.contains("1"));
    }
    #[test]
    fn test_expr_summary_sort() {
        let s = expr_summary(&Expr::Sort(Level::zero()));
        assert_eq!(s, "Sort(0)");
    }
    #[test]
    fn test_expr_summary_app() {
        let f = Expr::Const(Name::str("g"), vec![]);
        let a = Expr::Lit(Literal::Nat(5));
        let b = Expr::Lit(Literal::Nat(6));
        let app = Expr::App(Box::new(Expr::App(Box::new(f), Box::new(a))), Box::new(b));
        let s = expr_summary(&app);
        assert!(s.contains("2 args"));
        assert!(s.contains("g"));
    }
    #[test]
    fn test_expr_summary_pi_arrow() {
        let pi = Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::Sort(Level::zero())),
        );
        let s = expr_summary(&pi);
        assert!(s.contains("->"));
    }
    #[test]
    fn test_expr_summary_let() {
        let e = Expr::Let(
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::BVar(0)),
            Box::new(Expr::BVar(0)),
        );
        let s = expr_summary(&e);
        assert!(s.contains("x"));
    }
    #[test]
    fn test_is_simple_name_true() {
        let n = Name::Str(Box::new(Name::Anonymous), "foo".to_string());
        assert!(is_simple_name(&n));
    }
    #[test]
    fn test_is_simple_name_false() {
        let n = Name::Str(
            Box::new(Name::Str(Box::new(Name::Anonymous), "A".to_string())),
            "b".to_string(),
        );
        assert!(!is_simple_name(&n));
    }
    #[test]
    fn test_print_name_str() {
        let n = Name::str("MyName");
        assert_eq!(print_name_str(&n), "MyName");
    }
    #[test]
    fn test_colorize_sort_contains_text() {
        let e = Expr::Sort(Level::zero());
        let c = colorize(&e);
        assert!(c.contains("Prop") || c.contains("Sort"));
    }
    #[test]
    fn test_colorize_lit_contains_number() {
        let e = Expr::Lit(crate::Literal::Nat(42));
        let c = colorize(&e);
        assert!(c.contains("42"));
    }
    #[test]
    fn test_print_expr_let() {
        let e = Expr::Let(
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::Lit(crate::Literal::Nat(1))),
            Box::new(Expr::BVar(0)),
        );
        let s = print_expr(&e);
        assert!(s.contains("let"));
        assert!(s.contains("x"));
    }
    #[test]
    fn test_print_expr_proj() {
        let e = Expr::Proj(Name::str("Prod"), 0, Box::new(Expr::BVar(0)));
        let s = print_expr(&e);
        assert!(s.contains("Prod"));
    }
}
#[cfg(test)]
mod tests_prettyprint_extra {
    use super::*;
    #[test]
    fn test_color_scheme() {
        let cs = ColorScheme::DEFAULT;
        assert!(cs.is_colored());
        assert!(!ColorScheme::MONO.is_colored());
    }
    #[test]
    fn test_indent_style() {
        let s2 = IndentStyle::Spaces(2);
        assert_eq!(s2.one_level(), "  ");
        assert_eq!(s2.for_depth(3), "      ");
        let tabs = IndentStyle::Tabs;
        assert_eq!(tabs.one_level(), "\t");
    }
    #[test]
    fn test_pretty_doc_render() {
        let doc = PrettyDoc::concat(
            PrettyDoc::text("fun "),
            PrettyDoc::concat(
                PrettyDoc::text("x"),
                PrettyDoc::concat(PrettyDoc::text(" ->"), PrettyDoc::Newline),
            ),
        );
        let rendered = doc.render(80, IndentStyle::Spaces(2));
        assert!(rendered.contains("fun x ->"));
    }
    #[test]
    fn test_pretty_token_raw() {
        let tok = PrettyToken::Keyword("theorem".to_string());
        assert_eq!(tok.raw_text(), "theorem");
        let space = PrettyToken::Space;
        assert_eq!(space.raw_text(), " ");
    }
    #[test]
    fn test_token_printer() {
        let config = PrettyConfig::default_config();
        let printer = TokenPrinter::new(config);
        let tokens = vec![
            PrettyToken::Keyword("theorem".into()),
            PrettyToken::Space,
            PrettyToken::Ident("foo".into()),
        ];
        let result = printer.render(&tokens);
        assert_eq!(result, "theorem foo");
    }
    #[test]
    fn test_sexpr_printer() {
        let p = SExprPrinter::new();
        let s = p.app("Pi", &["x", "Nat", "body"]);
        assert_eq!(s, "(Pi x Nat body)");
        let empty = p.app("Unit", &[]);
        assert_eq!(empty, "Unit");
    }
    #[test]
    fn test_fmt_width() {
        assert_eq!(FmtWidth::decimal_width(0), 1);
        assert_eq!(FmtWidth::decimal_width(999), 3);
        assert_eq!(FmtWidth::pad_right("hi", 5), "hi   ");
        assert_eq!(FmtWidth::pad_left("hi", 5), "   hi");
        assert_eq!(FmtWidth::center("hi", 6), "  hi  ");
    }
    #[test]
    fn test_pretty_table() {
        let mut tbl = PrettyTable::new(vec!["Name".into(), "Value".into()]);
        tbl.add_row(vec!["alpha".into(), "1".into()]);
        tbl.add_row(vec!["beta".into(), "2".into()]);
        let rendered = tbl.render();
        assert!(rendered.contains("Name"));
        assert!(rendered.contains("alpha"));
    }
}
#[cfg(test)]
mod tests_prettyprint_extra2 {
    use super::*;
    #[test]
    fn test_printer_state() {
        let mut ps = PrettyPrinterState::new(80, IndentStyle::Spaces(2));
        ps.write("hello");
        assert_eq!(ps.col, 5);
        ps.push_indent();
        ps.newline();
        assert_eq!(ps.col, 2);
        let out = ps.finish();
        assert!(out.contains("hello"));
    }
    #[test]
    fn test_doc_builder() {
        let doc = DocBuilder::text("fun")
            .then_text(" x")
            .then_text(" ->")
            .then_newline()
            .then_text("  body")
            .build();
        let rendered = doc.render(80, IndentStyle::Spaces(2));
        assert!(rendered.contains("fun x ->"));
    }
}
#[cfg(test)]
mod tests_escape_helper {
    use super::*;
    #[test]
    fn test_escape_unescape() {
        let original = "hello\nworld\t\"end\"";
        let escaped = EscapeHelper::escape_str(original);
        assert!(escaped.starts_with('"'));
        let unescaped = EscapeHelper::unescape_str(&escaped);
        assert_eq!(unescaped, original);
    }
}
#[cfg(test)]
mod tests_common_infra {
    use super::*;
    #[test]
    fn test_event_counter() {
        let mut ec = EventCounter::new();
        ec.inc("hit");
        ec.inc("hit");
        ec.inc("miss");
        assert_eq!(ec.get("hit"), 2);
        assert_eq!(ec.get("miss"), 1);
        assert_eq!(ec.total(), 3);
        ec.reset();
        assert_eq!(ec.total(), 0);
    }
    #[test]
    fn test_diag_meta() {
        let mut m = DiagMeta::new();
        m.add("os", "linux");
        m.add("arch", "x86_64");
        assert_eq!(m.get("os"), Some("linux"));
        assert_eq!(m.len(), 2);
        let s = m.to_string();
        assert!(s.contains("os=linux"));
    }
    #[test]
    fn test_scope_stack() {
        let mut ss = ScopeStack::new();
        ss.push("Nat");
        ss.push("succ");
        assert_eq!(ss.current(), Some("succ"));
        assert_eq!(ss.depth(), 2);
        assert_eq!(ss.path(), "Nat.succ");
        ss.pop();
        assert_eq!(ss.current(), Some("Nat"));
    }
    #[test]
    fn test_annotation_table() {
        let mut tbl = AnnotationTable::new();
        tbl.annotate("doc", "first line");
        tbl.annotate("doc", "second line");
        assert_eq!(tbl.get_all("doc").len(), 2);
        assert!(tbl.has("doc"));
        assert!(!tbl.has("other"));
    }
    #[test]
    fn test_work_stack() {
        let mut ws = WorkStack::new();
        ws.push(1u32);
        ws.push(2u32);
        assert_eq!(ws.pop(), Some(2));
        assert_eq!(ws.len(), 1);
    }
    #[test]
    fn test_work_queue() {
        let mut wq = WorkQueue::new();
        wq.enqueue(1u32);
        wq.enqueue(2u32);
        assert_eq!(wq.dequeue(), Some(1));
        assert_eq!(wq.len(), 1);
    }
    #[test]
    fn test_sparse_bit_set() {
        let mut bs = SparseBitSet::new(128);
        bs.set(5);
        bs.set(63);
        bs.set(64);
        assert!(bs.get(5));
        assert!(bs.get(63));
        assert!(bs.get(64));
        assert!(!bs.get(0));
        assert_eq!(bs.count_ones(), 3);
        bs.clear(5);
        assert!(!bs.get(5));
    }
    #[test]
    fn test_loop_clock() {
        let mut clk = LoopClock::start();
        for _ in 0..10 {
            clk.tick();
        }
        assert_eq!(clk.iters(), 10);
        assert!(clk.elapsed_us() >= 0.0);
    }
}
#[cfg(test)]
mod tests_extra_data_structures {
    use super::*;
    #[test]
    fn test_simple_lru_cache() {
        let mut cache: SimpleLruCache<&str, u32> = SimpleLruCache::new(3);
        cache.put("a", 1);
        cache.put("b", 2);
        cache.put("c", 3);
        assert_eq!(cache.get(&"a"), Some(&1));
        cache.put("d", 4);
        assert!(cache.len() <= 3);
    }
    #[test]
    fn test_string_interner() {
        let mut si = StringInterner::new();
        let id1 = si.intern("hello");
        let id2 = si.intern("hello");
        assert_eq!(id1, id2);
        let id3 = si.intern("world");
        assert_ne!(id1, id3);
        assert_eq!(si.get(id1), Some("hello"));
        assert_eq!(si.len(), 2);
    }
    #[test]
    fn test_frequency_table() {
        let mut ft = FrequencyTable::new();
        ft.record("a");
        ft.record("b");
        ft.record("a");
        ft.record("a");
        assert_eq!(ft.freq(&"a"), 3);
        assert_eq!(ft.freq(&"b"), 1);
        assert_eq!(ft.most_frequent(), Some((&"a", 3)));
        assert_eq!(ft.total(), 4);
        assert_eq!(ft.distinct(), 2);
    }
    #[test]
    fn test_bimap() {
        let mut bm: BiMap<u32, &str> = BiMap::new();
        bm.insert(1, "one");
        bm.insert(2, "two");
        assert_eq!(bm.get_b(&1), Some(&"one"));
        assert_eq!(bm.get_a(&"two"), Some(&2));
        assert_eq!(bm.len(), 2);
    }
}
#[cfg(test)]
mod tests_interval_set {
    use super::*;
    #[test]
    fn test_interval_set() {
        let mut s = IntervalSet::new();
        s.add(1, 5);
        s.add(3, 8);
        assert_eq!(s.num_intervals(), 1);
        assert_eq!(s.cardinality(), 8);
        assert!(s.contains(4));
        assert!(!s.contains(9));
        s.add(10, 15);
        assert_eq!(s.num_intervals(), 2);
    }
}
/// Returns the current timestamp.
#[allow(dead_code)]
pub fn now_us() -> Timestamp {
    let us = std::time::SystemTime::UNIX_EPOCH
        .elapsed()
        .map(|d| d.as_micros() as u64)
        .unwrap_or(0);
    Timestamp::from_us(us)
}
#[cfg(test)]
mod tests_typed_utilities {
    use super::*;
    #[test]
    fn test_timestamp() {
        let t1 = Timestamp::from_us(1000);
        let t2 = Timestamp::from_us(1500);
        assert_eq!(t2.elapsed_since(t1), 500);
        assert!(t1 < t2);
    }
    #[test]
    fn test_typed_id() {
        struct Foo;
        let id: TypedId<Foo> = TypedId::new(42);
        assert_eq!(id.raw(), 42);
        assert_eq!(format!("{id}"), "#42");
    }
    #[test]
    fn test_id_dispenser() {
        struct Bar;
        let mut disp: IdDispenser<Bar> = IdDispenser::new();
        let a = disp.next();
        let b = disp.next();
        assert_eq!(a.raw(), 0);
        assert_eq!(b.raw(), 1);
        assert_eq!(disp.count(), 2);
    }
    #[test]
    fn test_slot() {
        let mut slot: Slot<u32> = Slot::empty();
        assert!(!slot.is_filled());
        slot.fill(99);
        assert!(slot.is_filled());
        assert_eq!(slot.get(), Some(&99));
        let v = slot.take();
        assert_eq!(v, Some(99));
        assert!(!slot.is_filled());
    }
    #[test]
    #[should_panic]
    fn test_slot_double_fill() {
        let mut slot: Slot<u32> = Slot::empty();
        slot.fill(1);
        slot.fill(2);
    }
    #[test]
    fn test_memo_slot() {
        let mut ms: MemoSlot<u32> = MemoSlot::new();
        assert!(!ms.is_cached());
        let val = ms.get_or_compute(|| 42);
        assert_eq!(*val, 42);
        assert!(ms.is_cached());
        ms.invalidate();
        assert!(!ms.is_cached());
    }
}
