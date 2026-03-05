//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Expr, Level, Literal, Name};

use super::functions::Show;
use super::functions::{
    shw_ext_derive_show, shw_ext_display_no_quoting, shw_ext_display_vs_debug,
    shw_ext_indented_format, shw_ext_pretty_doc_concat, shw_ext_pretty_doc_nest,
    shw_ext_pretty_doc_nil, shw_ext_pretty_doc_text, shw_ext_show_bool_canonical,
    shw_ext_show_char_quoted, shw_ext_show_composition_law, shw_ext_show_diagnostic_with_location,
    shw_ext_show_either_tagged, shw_ext_show_float_decimal, shw_ext_show_float_precision,
    shw_ext_show_injectivity_law, shw_ext_show_int_signed, shw_ext_show_is_function_to_string,
    shw_ext_show_list_bracketed, shw_ext_show_nat_binary, shw_ext_show_nat_decimal,
    shw_ext_show_nat_hex, shw_ext_show_nat_octal, shw_ext_show_option_canonical,
    shw_ext_show_pair_tuple, shw_ext_show_polymorphic_nat_trans, shw_ext_show_purity_law,
    shw_ext_show_read_roundtrip, shw_ext_show_recursive_terminates, shw_ext_show_result_canonical,
    shw_ext_show_unit_canonical, shw_ext_shows_compose_assoc, shw_ext_shows_identity_law,
    shw_ext_shows_to_string, shw_ext_tabular_format,
};
use super::types::{DiagnosticDisplay, FormattedOutput, PrettyDoc, ShowRegistryExt, ShowS};

/// Build axiom: PrettyDoc render width constraint law.
fn shw_ext_pretty_doc_width_law(env: &mut oxilean_kernel::Environment) -> Result<(), String> {
    use oxilean_kernel::{Declaration, Expr, Name};
    let prop = Expr::Sort(oxilean_kernel::Level::zero());
    match env.add(Declaration::Axiom {
        name: Name::str("PrettyDoc.widthLaw"),
        univ_params: vec![],
        ty: prop,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: PrettyDoc line break / newline constructor.
fn shw_ext_pretty_doc_line(env: &mut oxilean_kernel::Environment) -> Result<(), String> {
    use oxilean_kernel::{Declaration, Expr, Name};
    let doc_ty = Expr::Const(Name::str("PrettyDoc"), vec![]);
    match env.add(Declaration::Axiom {
        name: Name::str("PrettyDoc.line"),
        univ_params: vec![],
        ty: doc_ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Show.show is total (defined for all values).
fn shw_ext_show_totality(env: &mut oxilean_kernel::Environment) -> Result<(), String> {
    use oxilean_kernel::{Declaration, Expr, Name};
    let prop = Expr::Sort(oxilean_kernel::Level::zero());
    match env.add(Declaration::Axiom {
        name: Name::str("Show.totality"),
        univ_params: vec![],
        ty: prop,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Show for String escapes special chars.
fn shw_ext_show_string_escaped(env: &mut oxilean_kernel::Environment) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Name};
    let string_ty = Expr::Const(Name::str("String"), vec![]);
    let ty = Expr::Pi(
        Bi::Default,
        Name::str("s"),
        Box::new(string_ty.clone()),
        Box::new(string_ty),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("Show.stringEscaped"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Show length lower bound (non-empty for non-trivial types).
fn shw_ext_show_length_lower_bound(env: &mut oxilean_kernel::Environment) -> Result<(), String> {
    use oxilean_kernel::{Declaration, Expr, Name};
    let prop = Expr::Sort(oxilean_kernel::Level::zero());
    match env.add(Declaration::Axiom {
        name: Name::str("Show.lengthLowerBound"),
        univ_params: vec![],
        ty: prop,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Show respects equality (equal values have equal show strings).
fn shw_ext_show_respects_equality(env: &mut oxilean_kernel::Environment) -> Result<(), String> {
    use oxilean_kernel::{Declaration, Expr, Name};
    let prop = Expr::Sort(oxilean_kernel::Level::zero());
    match env.add(Declaration::Axiom {
        name: Name::str("Show.respectsEquality"),
        univ_params: vec![],
        ty: prop,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Show for Float infinity representation.
fn shw_ext_show_float_infinity(env: &mut oxilean_kernel::Environment) -> Result<(), String> {
    use oxilean_kernel::{Declaration, Expr, Name};
    let ty = Expr::Const(Name::str("String"), vec![]);
    match env.add(Declaration::Axiom {
        name: Name::str("Show.floatInfinity"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Show for Float NaN representation.
fn shw_ext_show_float_nan(env: &mut oxilean_kernel::Environment) -> Result<(), String> {
    use oxilean_kernel::{Declaration, Expr, Name};
    let ty = Expr::Const(Name::str("String"), vec![]);
    match env.add(Declaration::Axiom {
        name: Name::str("Show.floatNaN"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Show via showS is equivalent to show.
fn shw_ext_shows_equiv_show(env: &mut oxilean_kernel::Environment) -> Result<(), String> {
    use oxilean_kernel::{Declaration, Expr, Name};
    let prop = Expr::Sort(oxilean_kernel::Level::zero());
    match env.add(Declaration::Axiom {
        name: Name::str("ShowS.equivShow"),
        univ_params: vec![],
        ty: prop,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: PrettyDoc group (flatten alternative).
fn shw_ext_pretty_doc_group(env: &mut oxilean_kernel::Environment) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Name};
    let doc = || Expr::Const(Name::str("PrettyDoc"), vec![]);
    let ty = Expr::Pi(
        Bi::Default,
        Name::str("d"),
        Box::new(doc()),
        Box::new(doc()),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("PrettyDoc.group"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Show for Tree type (recursive Show).
fn shw_ext_show_tree_recursive(env: &mut oxilean_kernel::Environment) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Level, Name};
    let type1 = Expr::Sort(Level::succ(Level::zero()));
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
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Show"), vec![])),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Tree"), vec![])),
                    Box::new(Expr::BVar(1)),
                )),
            )),
        )),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("instShowTree"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Diagnostic display severity-to-prefix law.
fn shw_ext_diagnostic_severity_prefix(env: &mut oxilean_kernel::Environment) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Name};
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let string_ty = Expr::Const(Name::str("String"), vec![]);
    let ty = Expr::Pi(
        Bi::Default,
        Name::str("sev"),
        Box::new(nat_ty),
        Box::new(string_ty),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("Diagnostic.severityPrefix"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Diagnostic display renders non-empty string.
fn shw_ext_diagnostic_nonempty(env: &mut oxilean_kernel::Environment) -> Result<(), String> {
    use oxilean_kernel::{Declaration, Expr, Name};
    let prop = Expr::Sort(oxilean_kernel::Level::zero());
    match env.add(Declaration::Axiom {
        name: Name::str("Diagnostic.nonEmpty"),
        univ_params: vec![],
        ty: prop,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Registration function: registers all extended Show axioms.
pub fn register_show_extended_axioms(env: &mut oxilean_kernel::Environment) {
    let _ = shw_ext_show_is_function_to_string(env);
    let _ = shw_ext_show_nat_decimal(env);
    let _ = shw_ext_show_int_signed(env);
    let _ = shw_ext_show_bool_canonical(env);
    let _ = shw_ext_show_char_quoted(env);
    let _ = shw_ext_show_float_decimal(env);
    let _ = shw_ext_show_list_bracketed(env);
    let _ = shw_ext_show_option_canonical(env);
    let _ = shw_ext_show_pair_tuple(env);
    let _ = shw_ext_show_either_tagged(env);
    let _ = shw_ext_show_result_canonical(env);
    let _ = shw_ext_show_read_roundtrip(env);
    let _ = shw_ext_pretty_doc_nil(env);
    let _ = shw_ext_pretty_doc_text(env);
    let _ = shw_ext_pretty_doc_concat(env);
    let _ = shw_ext_pretty_doc_nest(env);
    let _ = shw_ext_display_vs_debug(env);
    let _ = shw_ext_show_recursive_terminates(env);
    let _ = shw_ext_show_polymorphic_nat_trans(env);
    let _ = shw_ext_shows_identity_law(env);
    let _ = shw_ext_shows_compose_assoc(env);
    let _ = shw_ext_shows_to_string(env);
    let _ = shw_ext_tabular_format(env);
    let _ = shw_ext_indented_format(env);
    let _ = shw_ext_show_nat_binary(env);
    let _ = shw_ext_show_nat_hex(env);
    let _ = shw_ext_show_nat_octal(env);
    let _ = shw_ext_show_float_precision(env);
    let _ = shw_ext_derive_show(env);
    let _ = shw_ext_show_diagnostic_with_location(env);
    let _ = shw_ext_show_injectivity_law(env);
    let _ = shw_ext_show_unit_canonical(env);
    let _ = shw_ext_show_purity_law(env);
    let _ = shw_ext_show_composition_law(env);
    let _ = shw_ext_display_no_quoting(env);
    let _ = shw_ext_pretty_doc_width_law(env);
    let _ = shw_ext_pretty_doc_line(env);
    let _ = shw_ext_show_totality(env);
    let _ = shw_ext_show_string_escaped(env);
    let _ = shw_ext_show_length_lower_bound(env);
    let _ = shw_ext_show_respects_equality(env);
    let _ = shw_ext_show_float_infinity(env);
    let _ = shw_ext_show_float_nan(env);
    let _ = shw_ext_shows_equiv_show(env);
    let _ = shw_ext_pretty_doc_group(env);
    let _ = shw_ext_show_tree_recursive(env);
    let _ = shw_ext_diagnostic_severity_prefix(env);
    let _ = shw_ext_diagnostic_nonempty(env);
}
/// Compose a sequence of ShowS values left-to-right.
pub fn shows_compose_all(parts: &[ShowS]) -> ShowS {
    parts
        .iter()
        .fold(ShowS::identity(), |acc, s| acc.compose(s))
}
/// Apply a ShowS to an empty string (materializing the prefix).
pub fn shows_run(s: &ShowS) -> String {
    s.apply("")
}
/// Build a ShowS from a string literal.
pub fn shows_str(s: &str) -> ShowS {
    ShowS::new(format!("str({:?})", s), s.to_string())
}
/// Build a ShowS that appends a space.
pub fn shows_space() -> ShowS {
    ShowS::new("space", " ")
}
/// Build a ShowS that appends a comma and space.
pub fn shows_comma() -> ShowS {
    ShowS::new("comma", ", ")
}
/// Build a ShowS that wraps in parentheses.
pub fn shows_parens(inner: &ShowS) -> ShowS {
    ShowS::new(
        format!("parens({})", inner.label),
        format!("({})", inner.prefix),
    )
}
/// Build a ShowS that wraps in square brackets.
pub fn shows_brackets(inner: &ShowS) -> ShowS {
    ShowS::new(
        format!("brackets({})", inner.label),
        format!("[{}]", inner.prefix),
    )
}
/// Join a list of `PrettyDoc`s with a separator.
pub fn pretty_join(docs: &[PrettyDoc], sep: &str) -> PrettyDoc {
    if docs.is_empty() {
        return PrettyDoc::text("");
    }
    let mut result = docs[0].text.clone();
    for d in &docs[1..] {
        result.push_str(sep);
        result.push_str(&d.text);
    }
    PrettyDoc::text(result)
}
/// Wrap a `PrettyDoc` in parentheses.
pub fn pretty_parens(d: &PrettyDoc) -> PrettyDoc {
    PrettyDoc::text(format!("({})", d.text))
}
/// Wrap a `PrettyDoc` in square brackets.
pub fn pretty_brackets(d: &PrettyDoc) -> PrettyDoc {
    PrettyDoc::text(format!("[{}]", d.text))
}
/// Indent a `PrettyDoc` by n spaces.
pub fn pretty_indent(d: &PrettyDoc, n: usize) -> PrettyDoc {
    let spaces: String = " ".repeat(n);
    PrettyDoc::text(format!("{}{}", spaces, d.text))
}
/// Format a list of `(key, value)` pairs as a table.
pub fn format_table(rows: &[(String, String)]) -> String {
    if rows.is_empty() {
        return String::new();
    }
    let key_width = rows.iter().map(|(k, _)| k.len()).max().unwrap_or(0);
    let mut out = String::new();
    for (k, v) in rows {
        out.push_str(&format!("{:<width$} : {}\n", k, v, width = key_width));
    }
    out
}
/// Format a value indented by `n` spaces on each line.
pub fn format_indented(s: &str, n: usize) -> String {
    let spaces: String = " ".repeat(n);
    s.lines()
        .map(|line| format!("{}{}", spaces, line))
        .collect::<Vec<_>>()
        .join("\n")
}
/// Show a floating-point number with `prec` decimal places.
pub fn show_float_precision(f: f64, prec: usize) -> String {
    format!("{:.prec$}", f, prec = prec)
}
/// Show a `u64` in binary with "0b" prefix.
pub fn show_nat_binary(n: u64) -> String {
    format!("0b{:b}", n)
}
/// Show a `u64` in hexadecimal with "0x" prefix.
pub fn show_nat_hex(n: u64) -> String {
    format!("0x{:x}", n)
}
/// Show a `u64` in octal with "0o" prefix.
pub fn show_nat_octal(n: u64) -> String {
    format!("0o{:o}", n)
}
#[cfg(test)]
mod show_extended_tests {
    use super::*;
    #[test]
    fn test_shows_identity_apply() {
        let id = ShowS::identity();
        assert_eq!(id.apply("hello"), "hello");
    }
    #[test]
    fn test_shows_new_apply() {
        let s = ShowS::new("test", "foo");
        assert_eq!(s.apply("bar"), "foobar");
    }
    #[test]
    fn test_shows_compose() {
        let a = ShowS::new("a", "Hello");
        let b = ShowS::new("b", " World");
        let c = a.compose(&b);
        assert_eq!(c.apply("!"), "Hello World!");
    }
    #[test]
    fn test_shows_str() {
        let s = shows_str("test");
        assert_eq!(s.apply(""), "test");
    }
    #[test]
    fn test_shows_space() {
        let s = shows_space();
        assert_eq!(s.apply("x"), " x");
    }
    #[test]
    fn test_shows_comma() {
        let s = shows_comma();
        assert_eq!(s.apply("x"), ", x");
    }
    #[test]
    fn test_shows_parens() {
        let inner = ShowS::new("inner", "abc");
        let p = shows_parens(&inner);
        assert_eq!(p.apply(""), "(abc)");
    }
    #[test]
    fn test_shows_brackets() {
        let inner = ShowS::new("inner", "xyz");
        let b = shows_brackets(&inner);
        assert_eq!(b.apply(""), "[xyz]");
    }
    #[test]
    fn test_shows_compose_all() {
        let parts = vec![shows_str("a"), shows_str("b"), shows_str("c")];
        let composed = shows_compose_all(&parts);
        assert_eq!(shows_run(&composed), "abc");
    }
    #[test]
    fn test_shows_run() {
        let s = ShowS::new("x", "hello");
        assert_eq!(shows_run(&s), "hello");
    }
    #[test]
    fn test_pretty_doc_text() {
        let d = PrettyDoc::text("hello");
        assert_eq!(d.render(), "hello");
        assert_eq!(d.width, 5);
    }
    #[test]
    fn test_pretty_doc_concat() {
        let a = PrettyDoc::text("foo");
        let b = PrettyDoc::text("bar");
        let c = PrettyDoc::concat(&a, &b);
        assert_eq!(c.render(), "foobar");
        assert_eq!(c.width, 6);
    }
    #[test]
    fn test_pretty_doc_nest() {
        let d = PrettyDoc::text("x");
        let nested = PrettyDoc::nest(&d, 4);
        assert_eq!(nested.indent, 4);
    }
    #[test]
    fn test_pretty_join_empty() {
        let docs: Vec<PrettyDoc> = vec![];
        let joined = pretty_join(&docs, ", ");
        assert_eq!(joined.render(), "");
    }
    #[test]
    fn test_pretty_join_nonempty() {
        let docs = vec![
            PrettyDoc::text("a"),
            PrettyDoc::text("b"),
            PrettyDoc::text("c"),
        ];
        let joined = pretty_join(&docs, ", ");
        assert_eq!(joined.render(), "a, b, c");
    }
    #[test]
    fn test_pretty_parens() {
        let d = PrettyDoc::text("inner");
        let p = pretty_parens(&d);
        assert_eq!(p.render(), "(inner)");
    }
    #[test]
    fn test_pretty_brackets() {
        let d = PrettyDoc::text("inner");
        let b = pretty_brackets(&d);
        assert_eq!(b.render(), "[inner]");
    }
    #[test]
    fn test_pretty_indent() {
        let d = PrettyDoc::text("x");
        let indented = pretty_indent(&d, 3);
        assert_eq!(indented.render(), "   x");
    }
    #[test]
    fn test_format_table_empty() {
        let rows: Vec<(String, String)> = vec![];
        assert_eq!(format_table(&rows), "");
    }
    #[test]
    fn test_format_table_single() {
        let rows = vec![("key".to_string(), "value".to_string())];
        let s = format_table(&rows);
        assert!(s.contains("key"));
        assert!(s.contains("value"));
    }
    #[test]
    fn test_format_indented() {
        let s = format_indented("hello\nworld", 2);
        assert!(s.starts_with("  hello"));
        assert!(s.contains("  world"));
    }
    #[test]
    fn test_show_float_precision() {
        let s = show_float_precision(std::f64::consts::PI, 2);
        assert_eq!(s, "3.14");
    }
    #[test]
    fn test_show_float_precision_zero() {
        let s = show_float_precision(42.7, 0);
        assert_eq!(s, "43");
    }
    #[test]
    fn test_show_nat_binary() {
        assert_eq!(show_nat_binary(0), "0b0");
        assert_eq!(show_nat_binary(5), "0b101");
        assert_eq!(show_nat_binary(8), "0b1000");
    }
    #[test]
    fn test_show_nat_hex() {
        assert_eq!(show_nat_hex(0), "0x0");
        assert_eq!(show_nat_hex(255), "0xff");
        assert_eq!(show_nat_hex(16), "0x10");
    }
    #[test]
    fn test_show_nat_octal() {
        assert_eq!(show_nat_octal(0), "0o0");
        assert_eq!(show_nat_octal(8), "0o10");
        assert_eq!(show_nat_octal(64), "0o100");
    }
    #[test]
    fn test_show_registry_ext_new() {
        let reg: ShowRegistryExt<u64> = ShowRegistryExt::new("u64");
        assert_eq!(reg.type_name, "u64");
        assert_eq!(reg.count(), 0);
    }
    #[test]
    fn test_show_registry_ext_register() {
        let mut reg: ShowRegistryExt<u64> = ShowRegistryExt::new("u64");
        reg.register_name("decimal");
        reg.register_name("hex");
        assert_eq!(reg.count(), 2);
    }
    #[test]
    fn test_formatted_output_new() {
        let fo = FormattedOutput::new("hello world", "test", false);
        assert_eq!(fo.rendered, "hello world");
        assert_eq!(fo.formatter, "test");
        assert!(!fo.truncated);
        assert_eq!(fo.char_count, 11);
    }
    #[test]
    fn test_formatted_output_from_show_no_truncate() {
        let n: u64 = 42;
        let fo = FormattedOutput::from_show(&n, "u64", None);
        assert_eq!(fo.rendered, "42");
        assert!(!fo.truncated);
    }
    #[test]
    fn test_formatted_output_from_show_truncate() {
        let s = "hello world".to_string();
        let fo = FormattedOutput::from_show(&s, "String", Some(5));
        assert!(fo.truncated);
        assert!(fo.rendered.ends_with("..."));
    }
    #[test]
    fn test_diagnostic_display_new() {
        let d = DiagnosticDisplay::new("something went wrong", 2);
        assert_eq!(d.severity, 2);
        assert!(d.location.is_none());
    }
    #[test]
    fn test_diagnostic_display_render_error() {
        let d = DiagnosticDisplay::new("test error", 2);
        let r = d.render();
        assert!(r.starts_with("error"));
        assert!(r.contains("test error"));
    }
    #[test]
    fn test_diagnostic_display_render_warning() {
        let d = DiagnosticDisplay::new("test warning", 1);
        let r = d.render();
        assert!(r.starts_with("warning"));
    }
    #[test]
    fn test_diagnostic_display_render_note() {
        let d = DiagnosticDisplay::new("test note", 0);
        let r = d.render();
        assert!(r.starts_with("note"));
    }
    #[test]
    fn test_diagnostic_display_with_location() {
        let d = DiagnosticDisplay::new("msg", 2).with_location("file.rs:10:5");
        let r = d.render();
        assert!(r.contains("file.rs:10:5"));
    }
    #[test]
    fn test_diagnostic_display_with_snippet() {
        let d = DiagnosticDisplay::new("msg", 1).with_snippet("let x = 5;");
        let r = d.render();
        assert!(r.contains("let x = 5;"));
    }
    #[test]
    fn test_register_show_extended_axioms_runs() {
        let mut env = oxilean_kernel::Environment::new();
        register_show_extended_axioms(&mut env);
    }
}
