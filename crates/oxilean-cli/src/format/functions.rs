//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{print_expr, BinderInfo, Expr, Level, Literal, Name};

use super::types::{
    Alignment, AlignmentStyle, ColumnSpec, Comment, Doc, FormatDiff, FormatOnSaveConfig,
    FormatOptions, FormatRule, FormatStats, Formatter, FunctionStyle, ImportGroup, LineBreaker,
    PatternStyle, StyleConfig,
};

/// Flatten a document to single-line form.
#[allow(dead_code)]
pub fn flatten(doc: &Doc) -> Doc {
    match doc {
        Doc::Nil => Doc::Nil,
        Doc::Text(s) => Doc::Text(s.clone()),
        Doc::Line => Doc::Text(" ".to_string()),
        Doc::HardLine => Doc::HardLine,
        Doc::Nest(n, inner) => Doc::Nest(*n, Box::new(flatten(inner))),
        Doc::Group(inner) => flatten(inner),
        Doc::Concat(a, b) => Doc::Concat(Box::new(flatten(a)), Box::new(flatten(b))),
        Doc::FlatAlt(flat, _breaking) => flatten(flat),
    }
}
/// Check if a (flattened) document fits in the given remaining width.
#[allow(dead_code)]
pub fn fits(doc: &Doc, remaining: usize) -> bool {
    let mut width_used: usize = 0;
    let mut stack = vec![doc.clone()];
    while let Some(d) = stack.pop() {
        match d {
            Doc::Nil => {}
            Doc::Text(s) => {
                width_used += s.len();
                if width_used > remaining {
                    return false;
                }
            }
            Doc::Line => {
                width_used += 1;
                if width_used > remaining {
                    return false;
                }
            }
            Doc::HardLine => return false,
            Doc::Nest(_, inner) => stack.push(*inner),
            Doc::Group(inner) => stack.push(*inner),
            Doc::Concat(a, b) => {
                stack.push(*b);
                stack.push(*a);
            }
            Doc::FlatAlt(flat, _) => stack.push(*flat),
        }
    }
    true
}
/// Precedence levels for expression formatting.
#[allow(dead_code)]
mod prec {
    pub const MIN: u32 = 0;
    pub const ARROW: u32 = 25;
    pub const APP: u32 = 10;
    pub const ATOM: u32 = 100;
}
/// Format a kernel Expr as a Doc.
#[allow(dead_code)]
pub fn format_expr(expr: &Expr, p: u32) -> Doc {
    match expr {
        Expr::Sort(level) => format_sort(level),
        Expr::BVar(idx) => Doc::text(format!("#{}", idx)),
        Expr::FVar(fvar) => Doc::text(format!("@{}", fvar.0)),
        Expr::Const(name, levels) => {
            if levels.is_empty() {
                Doc::text(name.to_string())
            } else {
                let level_strs: Vec<String> = levels.iter().map(format_level_str).collect();
                Doc::text(format!("{}.{{{}}}", name, level_strs.join(", ")))
            }
        }
        Expr::App(_, _) => format_app(expr, p),
        Expr::Lam(bi, name, ty, body) => format_lambda(*bi, name, ty, body, p),
        Expr::Pi(bi, name, ty, body) => format_pi(*bi, name, ty, body, p),
        Expr::Let(name, ty, val, body) => format_let(name, ty, val, body, p),
        Expr::Lit(lit) => format_literal(lit),
        Expr::Proj(name, idx, e) => {
            let inner = format_expr(e, prec::ATOM);
            Doc::concat(inner, Doc::text(format!(".{}.{}", name, idx)))
        }
    }
}
/// Format a sort expression.
#[allow(dead_code)]
fn format_sort(level: &Level) -> Doc {
    if level.is_zero() {
        Doc::text("Prop")
    } else if *level == Level::succ(Level::zero()) {
        Doc::text("Type")
    } else {
        Doc::text(format!("Sort {}", format_level_str(level)))
    }
}
/// Format a level to a string.
#[allow(dead_code)]
fn format_level_str(level: &Level) -> String {
    match level {
        Level::Zero => "0".to_string(),
        Level::Succ(inner) => {
            if let Some(n) = level_to_nat(level) {
                n.to_string()
            } else {
                format!("{}+1", format_level_str(inner))
            }
        }
        Level::Max(a, b) => {
            format!("max({}, {})", format_level_str(a), format_level_str(b))
        }
        Level::IMax(a, b) => {
            format!("imax({}, {})", format_level_str(a), format_level_str(b))
        }
        Level::Param(name) => name.to_string(),
        Level::MVar(id) => format!("?u_{}", id.0),
    }
}
/// Convert a level to a natural number if possible.
#[allow(dead_code)]
fn level_to_nat(level: &Level) -> Option<u32> {
    match level {
        Level::Zero => Some(0),
        Level::Succ(inner) => level_to_nat(inner).map(|n| n + 1),
        _ => None,
    }
}
/// Format Pi types with arrow notation for non-dependent cases.
#[allow(dead_code)]
fn format_pi(bi: BinderInfo, name: &Name, ty: &Expr, body: &Expr, p: u32) -> Doc {
    let is_dep = name_is_used(name);
    let inner = if is_dep {
        let binder = format_binder(bi, name, ty);
        let body_doc = format_expr(body, prec::MIN);
        Doc::concat(
            Doc::text("forall "),
            Doc::concat(binder, Doc::concat(Doc::text(", "), body_doc)),
        )
    } else {
        let ty_doc = format_expr(ty, prec::ARROW + 1);
        let body_doc = format_expr(body, prec::ARROW);
        Doc::concat(ty_doc, Doc::concat(Doc::text(" -> "), body_doc))
    };
    if p > prec::MIN {
        Doc::concat(Doc::text("("), Doc::concat(inner, Doc::text(")")))
    } else {
        inner
    }
}
/// Format lambda expressions with `fun` keyword.
#[allow(dead_code)]
fn format_lambda(bi: BinderInfo, name: &Name, ty: &Expr, body: &Expr, p: u32) -> Doc {
    let binder = format_binder(bi, name, ty);
    let mut binders = vec![binder];
    let mut current = body;
    while let Expr::Lam(bi2, name2, ty2, body2) = current {
        binders.push(format_binder(*bi2, name2, ty2));
        current = body2;
    }
    let binders_doc = binders
        .into_iter()
        .reduce(Doc::space_concat)
        .unwrap_or(Doc::Nil);
    let body_doc = format_expr(current, prec::MIN);
    let inner = Doc::concat(
        Doc::text("fun "),
        Doc::concat(binders_doc, Doc::concat(Doc::text(" => "), body_doc)),
    );
    if p > prec::MIN {
        Doc::concat(Doc::text("("), Doc::concat(inner, Doc::text(")")))
    } else {
        inner
    }
}
/// Format function application with precedence.
#[allow(dead_code)]
fn format_app(expr: &Expr, p: u32) -> Doc {
    let (head, args) = collect_app_args(expr);
    let head_doc = format_expr(head, prec::APP);
    let args_docs: Vec<Doc> = args.iter().map(|a| format_expr(a, prec::APP + 1)).collect();
    let mut result = head_doc;
    for arg_doc in args_docs {
        result = Doc::space_concat(result, arg_doc);
    }
    if p > prec::APP {
        Doc::concat(Doc::text("("), Doc::concat(result, Doc::text(")")))
    } else {
        result
    }
}
/// Format let bindings.
#[allow(dead_code)]
fn format_let(name: &Name, ty: &Expr, val: &Expr, body: &Expr, _p: u32) -> Doc {
    let ty_doc = format_expr(ty, prec::MIN);
    let val_doc = format_expr(val, prec::MIN);
    let body_doc = format_expr(body, prec::MIN);
    Doc::group(Doc::concat(
        Doc::text(format!("let {} : ", name)),
        Doc::concat(
            ty_doc,
            Doc::concat(
                Doc::text(" := "),
                Doc::line_concat(val_doc, Doc::concat(Doc::text("in "), body_doc)),
            ),
        ),
    ))
}
/// Format a literal value.
#[allow(dead_code)]
fn format_literal(lit: &Literal) -> Doc {
    match lit {
        Literal::Nat(n) => Doc::text(n.to_string()),
        Literal::Str(s) => Doc::text(format!("\"{}\"", s)),
    }
}
/// Format a binder with its info.
#[allow(dead_code)]
fn format_binder(bi: BinderInfo, name: &Name, ty: &Expr) -> Doc {
    let ty_doc = format_expr(ty, prec::MIN);
    let name_str = name.to_string();
    match bi {
        BinderInfo::Default => Doc::text(format!("({} : {})", name_str, doc_to_string(&ty_doc))),
        BinderInfo::Implicit => Doc::text(format!("{{{} : {}}}", name_str, doc_to_string(&ty_doc))),
        BinderInfo::StrictImplicit => {
            Doc::text(format!("{{{{{} : {}}}}}", name_str, doc_to_string(&ty_doc)))
        }
        BinderInfo::InstImplicit => {
            Doc::text(format!("[{} : {}]", name_str, doc_to_string(&ty_doc)))
        }
    }
}
/// Helper to convert a Doc to a string (at default width).
#[allow(dead_code)]
fn doc_to_string(doc: &Doc) -> String {
    doc.pretty_print(100)
}
/// Collect head and arguments from nested application.
#[allow(dead_code)]
fn collect_app_args(expr: &Expr) -> (&Expr, Vec<&Expr>) {
    let mut args = Vec::new();
    let mut e = expr;
    while let Expr::App(f, a) = e {
        args.push(a.as_ref());
        e = f;
    }
    args.reverse();
    (e, args)
}
/// Check if a name appears to be used (not anonymous or _).
#[allow(dead_code)]
fn name_is_used(name: &Name) -> bool {
    !name.is_anonymous() && name.to_string() != "_"
}
/// Format a match expression (placeholder).
#[allow(dead_code)]
pub fn format_match(scrutinee: &Expr, arms: &[(String, Expr)]) -> Doc {
    let mut result = Doc::concat(
        Doc::text("match "),
        Doc::concat(format_expr(scrutinee, prec::MIN), Doc::text(" with")),
    );
    for (pattern, body) in arms {
        let arm_doc = Doc::concat(
            Doc::text(format!("| {} => ", pattern)),
            format_expr(body, prec::MIN),
        );
        result = Doc::line_concat(result, arm_doc);
    }
    result
}
/// Format a full declaration.
#[allow(dead_code)]
pub fn format_declaration(decl: &oxilean_kernel::Declaration) -> String {
    match decl {
        oxilean_kernel::Declaration::Axiom { name, ty, .. } => format_axiom_str(name, ty),
        oxilean_kernel::Declaration::Definition { name, ty, val, .. } => {
            format_def_str(name, ty, val)
        }
        oxilean_kernel::Declaration::Theorem { name, ty, val, .. } => {
            format_theorem_str(name, ty, val)
        }
        oxilean_kernel::Declaration::Opaque { name, ty, .. } => {
            format!("opaque {} : {}", name, print_expr(ty))
        }
    }
}
/// Format a definition: `def name : Type := body`
#[allow(dead_code)]
fn format_def_str(name: &Name, ty: &Expr, val: &Expr) -> String {
    let ty_str = print_expr(ty);
    let val_str = print_expr(val);
    if val_str.len() > 60 {
        format!("def {} : {} :=\n  {}", name, ty_str, val_str)
    } else {
        format!("def {} : {} := {}", name, ty_str, val_str)
    }
}
/// Format a theorem: `theorem name : Type := proof`
#[allow(dead_code)]
fn format_theorem_str(name: &Name, ty: &Expr, val: &Expr) -> String {
    let ty_str = print_expr(ty);
    let val_str = print_expr(val);
    if val_str.len() > 60 {
        format!("theorem {} : {} :=\n  {}", name, ty_str, val_str)
    } else {
        format!("theorem {} : {} := {}", name, ty_str, val_str)
    }
}
/// Format an axiom: `axiom name : Type`
#[allow(dead_code)]
fn format_axiom_str(name: &Name, ty: &Expr) -> String {
    format!("axiom {} : {}", name, print_expr(ty))
}
/// Format an inductive type with constructors.
#[allow(dead_code)]
pub fn format_inductive_str(name: &Name, ty: &Expr, ctors: &[(Name, Expr)]) -> String {
    let mut result = format!("inductive {} : {} where", name, print_expr(ty));
    for (ctor_name, ctor_ty) in ctors {
        result.push_str(&format!("\n  | {} : {}", ctor_name, print_expr(ctor_ty)));
    }
    result
}
/// Format a structure.
#[allow(dead_code)]
pub fn format_structure_str(name: &Name, fields: &[(Name, Expr)]) -> String {
    let mut result = format!("structure {} where", name);
    for (field_name, field_ty) in fields {
        result.push_str(&format!("\n  {} : {}", field_name, print_expr(field_ty)));
    }
    result
}
/// Parse import priority for sorting.
#[allow(dead_code)]
fn classify_import(import_line: &str) -> (ImportGroup, String) {
    let trimmed = import_line.trim();
    if trimmed.starts_with("import") || trimmed.starts_with("open") {
        if let Some(path_start) = trimmed.split_whitespace().nth(1) {
            if path_start.starts_with('.') || path_start.starts_with("..") {
                (ImportGroup::Local, path_start.to_string())
            } else if path_start.starts_with("Std.") || path_start.starts_with("Core.") {
                (ImportGroup::Stdlib, path_start.to_string())
            } else {
                (ImportGroup::External, path_start.to_string())
            }
        } else {
            (ImportGroup::External, String::new())
        }
    } else {
        (ImportGroup::External, String::new())
    }
}
/// Extract and preserve comments from source text.
#[allow(dead_code)]
fn extract_comments(source: &str) -> Vec<Comment> {
    let mut comments = Vec::new();
    let lines: Vec<&str> = source.lines().collect();
    for (line_num, line) in lines.iter().enumerate() {
        if let Some(comment_pos) = line.find("--") {
            if is_not_in_string(line, comment_pos) {
                comments.push(Comment {
                    line: line_num + 1,
                    text: line[comment_pos..].to_string(),
                    is_line_comment: true,
                });
            }
        }
    }
    comments
}
/// Check if a position in a line is within a string literal.
#[allow(dead_code)]
fn is_not_in_string(line: &str, pos: usize) -> bool {
    let mut in_string = false;
    let mut escape_next = false;
    for (i, ch) in line.chars().enumerate() {
        if i >= pos {
            break;
        }
        if escape_next {
            escape_next = false;
            continue;
        }
        if ch == '\\' {
            escape_next = true;
            continue;
        }
        if ch == '"' {
            in_string = !in_string;
        }
    }
    !in_string
}
/// Generate a unified diff for display.
#[allow(dead_code)]
pub fn generate_unified_diff(original: &str, formatted: &str, _context_lines: usize) -> String {
    let orig_lines: Vec<&str> = original.lines().collect();
    let fmt_lines: Vec<&str> = formatted.lines().collect();
    let mut diff = String::new();
    diff.push_str("--- original\n");
    diff.push_str("+++ formatted\n");
    let mut i = 0;
    let max = orig_lines.len().max(fmt_lines.len());
    while i < max {
        let orig = orig_lines.get(i).unwrap_or(&"");
        let fmt = fmt_lines.get(i).unwrap_or(&"");
        if orig == fmt {
            diff.push(' ');
            diff.push_str(orig);
            diff.push('\n');
        } else {
            diff.push('-');
            diff.push_str(orig);
            diff.push('\n');
            diff.push('+');
            diff.push_str(fmt);
            diff.push('\n');
        }
        i += 1;
    }
    diff
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_doc_nil() {
        let doc = Doc::Nil;
        assert_eq!(doc.pretty_print(80), "");
    }
    #[test]
    fn test_doc_text() {
        let doc = Doc::text("hello");
        assert_eq!(doc.pretty_print(80), "hello");
    }
    #[test]
    fn test_doc_concat() {
        let doc = Doc::concat(Doc::text("hello"), Doc::text(" world"));
        assert_eq!(doc.pretty_print(80), "hello world");
    }
    #[test]
    fn test_doc_line_flat() {
        let doc = Doc::group(Doc::concat(
            Doc::text("hello"),
            Doc::concat(Doc::Line, Doc::text("world")),
        ));
        assert_eq!(doc.pretty_print(80), "hello world");
    }
    #[test]
    fn test_doc_line_break() {
        let doc = Doc::group(Doc::concat(
            Doc::text("hello"),
            Doc::concat(Doc::Line, Doc::text("world")),
        ));
        assert_eq!(doc.pretty_print(5), "hello\nworld");
    }
    #[test]
    fn test_doc_nest() {
        let inner = Doc::concat(Doc::text("x"), Doc::concat(Doc::Line, Doc::text("y")));
        let doc = Doc::Nest(2, Box::new(inner));
        let result = doc.pretty_print(1);
        assert!(result.contains("  y"));
    }
    #[test]
    fn test_doc_group() {
        let inner = Doc::concat(Doc::text("a"), Doc::concat(Doc::Line, Doc::text("b")));
        let doc = Doc::group(inner);
        assert_eq!(doc.pretty_print(80), "a b");
    }
    #[test]
    fn test_doc_flat_alt() {
        let doc = Doc::FlatAlt(Box::new(Doc::text("flat")), Box::new(Doc::text("break")));
        assert_eq!(doc.pretty_print(80), "break");
    }
    #[test]
    fn test_doc_hard_line() {
        let doc = Doc::concat(Doc::text("a"), Doc::concat(Doc::HardLine, Doc::text("b")));
        assert!(doc.pretty_print(80).contains('\n'));
    }
    #[test]
    fn test_format_sort_prop() {
        let expr = Expr::Sort(Level::zero());
        let doc = format_expr(&expr, 0);
        assert_eq!(doc.pretty_print(80), "Prop");
    }
    #[test]
    fn test_format_sort_type() {
        let expr = Expr::Sort(Level::succ(Level::zero()));
        let doc = format_expr(&expr, 0);
        assert_eq!(doc.pretty_print(80), "Type");
    }
    #[test]
    fn test_format_literal_nat() {
        let expr = Expr::Lit(Literal::Nat(42));
        let doc = format_expr(&expr, 0);
        assert_eq!(doc.pretty_print(80), "42");
    }
    #[test]
    fn test_format_literal_str() {
        let expr = Expr::Lit(Literal::Str("hello".to_string()));
        let doc = format_expr(&expr, 0);
        assert_eq!(doc.pretty_print(80), "\"hello\"");
    }
    #[test]
    fn test_format_const() {
        let expr = Expr::Const(Name::str("Nat"), vec![]);
        let doc = format_expr(&expr, 0);
        assert_eq!(doc.pretty_print(80), "Nat");
    }
    #[test]
    fn test_format_app() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Lit(Literal::Nat(1));
        let app = Expr::App(Box::new(f), Box::new(a));
        let doc = format_expr(&app, 0);
        let result = doc.pretty_print(80);
        assert!(result.contains("f"));
        assert!(result.contains("1"));
    }
    #[test]
    fn test_format_lambda() {
        let ty = Expr::Sort(Level::zero());
        let body = Expr::BVar(0);
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(ty),
            Box::new(body),
        );
        let doc = format_expr(&lam, 0);
        let result = doc.pretty_print(80);
        assert!(result.contains("fun"));
        assert!(result.contains("x"));
    }
    #[test]
    fn test_format_pi_arrow() {
        let ty = Expr::Sort(Level::succ(Level::zero()));
        let body = Expr::Sort(Level::zero());
        let pi = Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(ty),
            Box::new(body),
        );
        let doc = format_expr(&pi, 0);
        let result = doc.pretty_print(80);
        assert!(result.contains("->"));
    }
    #[test]
    fn test_format_pi_forall() {
        let ty = Expr::Sort(Level::succ(Level::zero()));
        let body = Expr::BVar(0);
        let pi = Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(ty),
            Box::new(body),
        );
        let doc = format_expr(&pi, 0);
        let result = doc.pretty_print(80);
        assert!(result.contains("forall"));
    }
    #[test]
    fn test_format_axiom_declaration() {
        let result = format_axiom_str(&Name::str("propext"), &Expr::Sort(Level::zero()));
        assert_eq!(result, "axiom propext : Prop");
    }
    #[test]
    fn test_format_def_declaration() {
        let result = format_def_str(
            &Name::str("answer"),
            &Expr::Const(Name::str("Nat"), vec![]),
            &Expr::Lit(Literal::Nat(42)),
        );
        assert!(result.contains("def answer"));
        assert!(result.contains("Nat"));
        assert!(result.contains("42"));
    }
    #[test]
    fn test_format_theorem_declaration() {
        let result = format_theorem_str(
            &Name::str("trivial"),
            &Expr::Sort(Level::zero()),
            &Expr::Const(Name::str("True.intro"), vec![]),
        );
        assert!(result.contains("theorem trivial"));
    }
    #[test]
    fn test_format_inductive_str() {
        let result = format_inductive_str(
            &Name::str("Bool"),
            &Expr::Sort(Level::succ(Level::zero())),
            &[
                (Name::str("true"), Expr::Const(Name::str("Bool"), vec![])),
                (Name::str("false"), Expr::Const(Name::str("Bool"), vec![])),
            ],
        );
        assert!(result.contains("inductive Bool"));
        assert!(result.contains("| true"));
        assert!(result.contains("| false"));
    }
    #[test]
    fn test_format_structure_str() {
        let result = format_structure_str(
            &Name::str("Point"),
            &[
                (Name::str("x"), Expr::Const(Name::str("Nat"), vec![])),
                (Name::str("y"), Expr::Const(Name::str("Nat"), vec![])),
            ],
        );
        assert!(result.contains("structure Point where"));
        assert!(result.contains("x : Nat"));
        assert!(result.contains("y : Nat"));
    }
    #[test]
    fn test_formatter_create() {
        let formatter = Formatter::new();
        assert_eq!(formatter.options().indent_width, 2);
    }
    #[test]
    fn test_with_options() {
        let options = FormatOptions {
            indent_width: 4,
            max_width: 80,
            use_spaces: false,
            rules: vec![],
        };
        let formatter = Formatter::with_options(options);
        assert_eq!(formatter.options().indent_width, 4);
        assert_eq!(formatter.options().max_width, 80);
        assert!(!formatter.options().use_spaces);
    }
    #[test]
    fn test_set_options() {
        let mut formatter = Formatter::new();
        let options = FormatOptions {
            indent_width: 4,
            max_width: 120,
            use_spaces: true,
            rules: vec![],
        };
        formatter.set_options(options);
        assert_eq!(formatter.options().indent_width, 4);
    }
    #[test]
    fn test_normalize_whitespace() {
        let formatter = Formatter::new();
        let input = "hello  \nworld  \n\n\n\nend";
        let result = formatter.normalize_whitespace(input);
        assert!(!result.contains("  \n"));
        assert!(!result.contains("\n\n\n"));
    }
    #[test]
    fn test_sort_imports() {
        let formatter = Formatter::new();
        let input = "import Mathlib.B\nimport Mathlib.A\ndef x := 1";
        let result = formatter.sort_imports(input);
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines[0], "import Mathlib.A");
        assert_eq!(lines[1], "import Mathlib.B");
    }
    #[test]
    fn test_indent_block() {
        let formatter = Formatter::new();
        let result = formatter.indent_block("a\nb\nc");
        assert!(result.starts_with("  a"));
        assert!(result.contains("\n  b"));
    }
    #[test]
    fn test_wrap_line_short() {
        let formatter = Formatter::new();
        let result = formatter.wrap_line("short line");
        assert_eq!(result, "short line");
    }
    #[test]
    fn test_wrap_line_long() {
        let options = FormatOptions {
            max_width: 20,
            ..Default::default()
        };
        let formatter = Formatter::with_options(options);
        let input = "this is a very long line that should be wrapped";
        let result = formatter.wrap_line(input);
        assert!(result.contains('\n'));
    }
    #[test]
    fn test_diff_no_changes() {
        let diff = FormatDiff::compute_diff("hello\nworld", "hello\nworld");
        assert!(!diff.has_changes());
        assert_eq!(diff.num_changes(), 0);
    }
    #[test]
    fn test_diff_with_changes() {
        let diff = FormatDiff::compute_diff("hello  world\n", "hello world\n");
        assert!(diff.has_changes());
        assert_eq!(diff.num_changes(), 1);
        assert_eq!(diff.changes[0].line, 1);
    }
    #[test]
    fn test_diff_apply() {
        let diff = FormatDiff::compute_diff("old", "new");
        assert_eq!(diff.apply_changes(), "new");
    }
    #[test]
    fn test_diff_show() {
        let diff = FormatDiff::compute_diff("old line\n", "new line\n");
        let output = diff.show_diff();
        assert!(output.contains("Line 1:"));
        assert!(output.contains("- old line"));
        assert!(output.contains("+ new line"));
    }
    #[test]
    fn test_diff_show_no_changes() {
        let diff = FormatDiff::compute_diff("same", "same");
        assert_eq!(diff.show_diff(), "No changes.");
    }
    #[test]
    fn test_format_file_basic() {
        let formatter = Formatter::new();
        let result = formatter.format_file("def x := 1  \ndef y := 2  \n");
        assert!(result.is_ok());
        let formatted = result.expect("formatting should succeed");
        assert!(!formatted.contains("  \n"));
    }
    #[test]
    fn test_format_source_full() {
        let formatter = Formatter::new();
        let input = "import B\nimport A\n\n\n\n\ndef x := 1\n";
        let result = formatter.format_source(input);
        assert!(result.starts_with("import A"));
    }
    #[test]
    fn test_classify_import_stdlib() {
        let (group, _) = classify_import("import Std.Data");
        assert_eq!(group, ImportGroup::Stdlib);
    }
    #[test]
    fn test_classify_import_local() {
        let (group, _) = classify_import("import ./local");
        assert_eq!(group, ImportGroup::Local);
    }
    #[test]
    fn test_classify_import_external() {
        let (group, _) = classify_import("import Mathlib.Data");
        assert_eq!(group, ImportGroup::External);
    }
    #[test]
    fn test_extract_single_comment() {
        let source = "def x := 1 -- this is a comment";
        let comments = extract_comments(source);
        assert_eq!(comments.len(), 1);
        assert_eq!(comments[0].line, 1);
        assert!(comments[0].text.contains("this is a comment"));
    }
    #[test]
    fn test_extract_no_comments() {
        let source = "def x := 1\ndef y := 2";
        let comments = extract_comments(source);
        assert_eq!(comments.len(), 0);
    }
    #[test]
    fn test_extract_multiple_comments() {
        let source = "def x := 1 -- comment 1\ndef y := 2 -- comment 2";
        let comments = extract_comments(source);
        assert_eq!(comments.len(), 2);
    }
    #[test]
    fn test_is_not_in_string_outside() {
        assert!(is_not_in_string("hello -- world", 7));
    }
    #[test]
    fn test_is_not_in_string_inside() {
        assert!(!is_not_in_string("\"hello -- world\"", 7));
    }
    #[test]
    fn test_style_config_default() {
        let config = StyleConfig::default();
        assert_eq!(config.function_style, FunctionStyle::Smart);
        assert_eq!(config.pattern_style, PatternStyle::Compact);
        assert_eq!(config.alignment, AlignmentStyle::Pipes);
    }
    #[test]
    fn test_line_breaker_short_line() {
        let breaker = LineBreaker::new(80, 2);
        let result = breaker.break_line("short line");
        assert_eq!(result, "short line");
    }
    #[test]
    fn test_line_breaker_long_line() {
        let breaker = LineBreaker::new(20, 2);
        let result = breaker.break_line("this is a very long line with commas, and more, stuff");
        assert!(result.contains('\n'));
    }
    #[test]
    fn test_format_on_save_default() {
        let config = FormatOnSaveConfig::default();
        assert!(config.enabled);
        assert!(config.patterns.contains(&"*.lean".to_string()));
        assert!(config.keep_backups);
    }
    #[test]
    fn test_unified_diff_no_changes() {
        let diff = generate_unified_diff("line1\nline2", "line1\nline2", 3);
        assert!(diff.contains("--- original"));
        assert!(diff.contains("+++ formatted"));
        assert!(diff.contains(" line1"));
    }
    #[test]
    fn test_unified_diff_with_changes() {
        let diff = generate_unified_diff("old", "new", 3);
        assert!(diff.contains("-old"));
        assert!(diff.contains("+new"));
    }
    #[test]
    fn test_unified_diff_multiline() {
        let original = "a\nb\nc";
        let formatted = "a\nx\nc";
        let diff = generate_unified_diff(original, formatted, 3);
        assert!(diff.contains(" a"));
        assert!(diff.contains("-b"));
        assert!(diff.contains("+x"));
        assert!(diff.contains(" c"));
    }
    #[test]
    fn test_format_file_with_extension() {
        let config = FormatOnSaveConfig::default();
        let filename = "test.lean";
        let matches = config
            .patterns
            .iter()
            .any(|p| filename.ends_with(p.trim_start_matches('*')));
        assert!(matches);
    }
    #[test]
    fn test_format_file_non_matching_extension() {
        let config = FormatOnSaveConfig::default();
        let filename = "test.rs";
        let matches = config
            .patterns
            .iter()
            .any(|p| filename.ends_with(p.trim_start_matches('*')));
        assert!(!matches);
    }
    #[test]
    fn test_format_bvar() {
        let expr = Expr::BVar(5);
        let doc = format_expr(&expr, 0);
        assert_eq!(doc.pretty_print(80), "#5");
    }
    #[test]
    fn test_format_multiple_apps() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a1 = Expr::Lit(Literal::Nat(1));
        let a2 = Expr::Lit(Literal::Nat(2));
        let app1 = Expr::App(Box::new(f), Box::new(a1));
        let app2 = Expr::App(Box::new(app1), Box::new(a2));
        let doc = format_expr(&app2, 0);
        let result = doc.pretty_print(80);
        assert!(result.contains("f"));
        assert!(result.contains("1"));
        assert!(result.contains("2"));
    }
    #[test]
    fn test_normalize_mixed_whitespace() {
        let formatter = Formatter::new();
        let input = "line1\n  \nline2";
        let result = formatter.normalize_whitespace(input);
        assert!(!result.contains("  \n"));
    }
    #[test]
    fn test_sort_imports_mixed() {
        let formatter = Formatter::new();
        let input = "open Nat\nimport Z\nimport A\nopen Bool";
        let result = formatter.sort_imports(input);
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines[0], "import A");
        assert_eq!(lines[1], "import Z");
    }
    #[test]
    fn test_line_breaker_args_short() {
        let breaker = LineBreaker::new(80, 2);
        let result = breaker.break_function_args("a, b, c");
        assert_eq!(result, "a, b, c");
    }
    #[test]
    fn test_line_breaker_args_long() {
        let breaker = LineBreaker::new(20, 2);
        let result = breaker.break_function_args("first_argument, second_argument, third_argument");
        assert!(result.contains('\n'));
    }
    #[test]
    fn test_line_breaker_nested_parens() {
        let breaker = LineBreaker::new(30, 2);
        let result = breaker.break_line("func((a, b), (c, d), (e, f))");
        assert!(result.contains("func"));
    }
    #[test]
    fn test_format_rule_indent_block() {
        let rule = FormatRule::IndentBlock(4);
        assert_eq!(rule, FormatRule::IndentBlock(4));
    }
    #[test]
    fn test_format_rule_normalize_operators() {
        let rule = FormatRule::NormalizeOperators;
        assert_eq!(rule, FormatRule::NormalizeOperators);
    }
    #[test]
    fn test_format_options_with_rules() {
        let options = FormatOptions {
            indent_width: 2,
            max_width: 100,
            use_spaces: true,
            rules: vec![FormatRule::IndentBlock(2), FormatRule::NormalizeOperators],
        };
        assert_eq!(options.rules.len(), 2);
    }
    #[test]
    fn test_comment_basic() {
        let comment = Comment {
            line: 1,
            text: "-- this is a comment".to_string(),
            is_line_comment: true,
        };
        assert_eq!(comment.line, 1);
        assert!(comment.text.contains("this is a comment"));
    }
    #[test]
    fn test_is_not_in_string_complex() {
        let line = r#"let s = "hello -- world""#;
        assert!(!is_not_in_string(line, line.find("--").unwrap_or(0)));
    }
    #[test]
    fn test_extract_comments_multiple_lines() {
        let source = "a -- c1\nb -- c2\nc -- c3";
        let comments = extract_comments(source);
        assert_eq!(comments.len(), 3);
        assert_eq!(comments[0].line, 1);
        assert_eq!(comments[1].line, 2);
        assert_eq!(comments[2].line, 3);
    }
    #[test]
    fn test_function_style_compact() {
        assert_eq!(FunctionStyle::Compact, FunctionStyle::Compact);
        assert_ne!(FunctionStyle::Compact, FunctionStyle::Expanded);
    }
    #[test]
    fn test_function_style_smart() {
        assert_eq!(FunctionStyle::Smart, FunctionStyle::Smart);
    }
    #[test]
    fn test_pattern_style_compact() {
        assert_eq!(PatternStyle::Compact, PatternStyle::Compact);
    }
    #[test]
    fn test_alignment_style_pipes() {
        assert_eq!(AlignmentStyle::Pipes, AlignmentStyle::Pipes);
    }
    #[test]
    fn test_style_config_with_custom_settings() {
        let config = StyleConfig {
            function_style: FunctionStyle::Expanded,
            pattern_style: PatternStyle::Expanded,
            alignment: AlignmentStyle::Equals,
        };
        assert_eq!(config.function_style, FunctionStyle::Expanded);
    }
    #[test]
    fn test_format_on_save_enabled_by_default() {
        let config = FormatOnSaveConfig::default();
        assert!(config.enabled);
    }
    #[test]
    fn test_format_on_save_disable() {
        let config = FormatOnSaveConfig {
            enabled: false,
            ..Default::default()
        };
        assert!(!config.enabled);
    }
    #[test]
    fn test_format_on_save_custom_patterns() {
        let config = FormatOnSaveConfig {
            enabled: true,
            patterns: vec!["*.lean".to_string(), "*.lean.old".to_string()],
            keep_backups: false,
        };
        assert_eq!(config.patterns.len(), 2);
        assert!(!config.keep_backups);
    }
    #[test]
    fn test_format_on_save_add_pattern() {
        let mut config = FormatOnSaveConfig::default();
        config.patterns.push("*.bak".to_string());
        assert_eq!(config.patterns.len(), 2);
    }
    #[test]
    fn test_unified_diff_headers() {
        let diff = generate_unified_diff("old", "new", 3);
        assert!(diff.contains("--- original"));
        assert!(diff.contains("+++ formatted"));
    }
    #[test]
    fn test_unified_diff_unchanged_lines() {
        let diff = generate_unified_diff("a\nb\nc", "a\nb\nc", 3);
        assert!(diff.contains(" a"));
        assert!(diff.contains(" b"));
        assert!(diff.contains(" c"));
    }
    #[test]
    fn test_unified_diff_added_lines() {
        let diff = generate_unified_diff("a", "a\nb", 3);
        assert!(diff.contains("+b"));
    }
    #[test]
    fn test_unified_diff_removed_lines() {
        let diff = generate_unified_diff("a\nb", "a", 3);
        assert!(diff.contains("-b"));
    }
    #[test]
    fn test_unified_diff_complex() {
        let original = "line1\nline2\nline3\nline4\nline5";
        let formatted = "line1\nmodified2\nline3\nline4\nline5";
        let diff = generate_unified_diff(original, formatted, 3);
        assert!(diff.contains("-line2"));
        assert!(diff.contains("+modified2"));
    }
    #[test]
    fn test_format_long_function_definition() {
        let formatter = Formatter::new();
        let long_def =
            "def very_long_function_name_that_is_quite_descriptive : Type -> Type -> Type := ...";
        let wrapped = formatter.wrap_line(long_def);
        assert!(!wrapped.is_empty());
    }
    #[test]
    fn test_format_nested_structure() {
        let formatter = Formatter::new();
        let nested =
            "structure Nested where\n  inner : structure InnerStruct where\n    field : Nat";
        let result = formatter.format_source(nested);
        assert!(!result.is_empty());
    }
    #[test]
    fn test_format_preserves_meaningful_content() {
        let formatter = Formatter::new();
        let source = "def example := 42 -- important comment";
        let result = formatter.format_source(source);
        assert!(result.contains("42"));
        assert!(result.contains("important"));
    }
    #[test]
    fn test_indent_block_empty_lines() {
        let formatter = Formatter::new();
        let text = "a\n\nb";
        let result = formatter.indent_block(text);
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines[0], "  a");
        assert_eq!(lines[1], "");
        assert_eq!(lines[2], "  b");
    }
    #[test]
    fn test_normalize_consecutive_spaces() {
        let formatter = Formatter::new();
        let input = "hello    world";
        let result = formatter.normalize_whitespace_in_line(input);
        assert_eq!(result, "hello world");
    }
    #[test]
    fn test_format_with_tabs_to_spaces() {
        let options = FormatOptions {
            indent_width: 4,
            max_width: 100,
            use_spaces: true,
            rules: vec![],
        };
        let formatter = Formatter::with_options(options);
        assert!(formatter.options().use_spaces);
    }
    #[test]
    fn test_format_with_tabs_preservation() {
        let options = FormatOptions {
            indent_width: 4,
            max_width: 100,
            use_spaces: false,
            rules: vec![],
        };
        let formatter = Formatter::with_options(options);
        assert!(!formatter.options().use_spaces);
    }
    #[test]
    fn test_format_empty_file() {
        let formatter = Formatter::new();
        let result = formatter
            .format_file("")
            .expect("formatting should succeed");
        assert_eq!(result, "");
    }
    #[test]
    fn test_format_only_whitespace() {
        let formatter = Formatter::new();
        let result = formatter
            .format_file("   \n   \n   ")
            .expect("formatting should succeed");
        assert!(result.trim().is_empty() || result.is_empty());
    }
    #[test]
    fn test_format_unicode_content() {
        let formatter = Formatter::new();
        let result = formatter
            .format_file("def α : Type := ∀ β, β")
            .expect("formatting should succeed");
        assert!(result.contains("α"));
        assert!(result.contains("∀"));
    }
    #[test]
    fn test_diff_line_count_increase() {
        let diff = FormatDiff::compute_diff("a\nb", "a\nx\ny\nb");
        assert!(!diff.changes.is_empty());
    }
    #[test]
    fn test_diff_line_count_decrease() {
        let diff = FormatDiff::compute_diff("a\nx\ny\nb", "a\nb");
        assert!(!diff.changes.is_empty());
    }
    #[test]
    fn test_flatten_document() {
        let doc = Doc::Line;
        let flattened = flatten(&doc);
        assert_eq!(flattened.pretty_print(80), " ");
    }
    #[test]
    fn test_fits_function_width_check() {
        let doc = Doc::text("hello world");
        assert!(fits(&doc, 20));
        assert!(!fits(&doc, 5));
    }
    #[test]
    fn test_format_with_multiple_rules() {
        let options = FormatOptions {
            indent_width: 2,
            max_width: 100,
            use_spaces: true,
            rules: vec![
                FormatRule::IndentBlock(2),
                FormatRule::AlignEquals,
                FormatRule::NormalizeOperators,
                FormatRule::BreakBeforeWhere,
            ],
        };
        let formatter = Formatter::with_options(options);
        assert_eq!(formatter.options().rules.len(), 4);
    }
}
#[allow(dead_code)]
pub fn word_wrap(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();
    for word in text.split_whitespace() {
        if current.is_empty() {
            current.push_str(word);
        } else if current.len() + 1 + word.len() <= width {
            current.push(' ');
            current.push_str(word);
        } else {
            lines.push(current.clone());
            current = word.to_string();
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }
    lines
}
#[allow(dead_code)]
pub fn truncate_with_ellipsis(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else if max_len <= 3 {
        "...".to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}
#[allow(dead_code)]
pub fn pad_left(s: &str, width: usize, pad: char) -> String {
    if s.len() >= width {
        s.to_string()
    } else {
        format!("{}{}", pad.to_string().repeat(width - s.len()), s)
    }
}
#[allow(dead_code)]
pub fn pad_right(s: &str, width: usize, pad: char) -> String {
    if s.len() >= width {
        s.to_string()
    } else {
        format!("{}{}", s, pad.to_string().repeat(width - s.len()))
    }
}
#[allow(dead_code)]
pub fn normalize_whitespace(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}
#[allow(dead_code)]
pub fn count_leading_spaces(s: &str) -> usize {
    s.chars().take_while(|c| *c == ' ').count()
}
#[allow(dead_code)]
pub fn strip_trailing_whitespace(s: &str) -> String {
    s.lines().map(|l| l.trim_end()).collect::<Vec<_>>().join(
        "
",
    )
}
#[allow(dead_code)]
pub fn highlight_diff_line(line: &str) -> String {
    if line.starts_with('+') {
        format!("[ADD] {}", line)
    } else if line.starts_with('-') {
        format!("[DEL] {}", line)
    } else if line.starts_with('@') {
        format!("[HDR] {}", line)
    } else {
        line.to_string()
    }
}
#[cfg(test)]
mod format_extra_tests {
    use super::*;
    #[test]
    fn test_column_left() {
        let col = ColumnSpec::new("Name", 8, Alignment::Left);
        assert_eq!(col.format_cell("foo"), "foo     ");
    }
    #[test]
    fn test_word_wrap() {
        let lines = word_wrap("hello world foo bar", 10);
        assert!(!lines.is_empty());
    }
    #[test]
    fn test_truncate() {
        assert_eq!(truncate_with_ellipsis("hello world", 8), "hello...");
    }
    #[test]
    fn test_format_stats() {
        let s = FormatStats::new();
        assert_eq!(s.change_ratio(), 0.0);
    }
    #[test]
    fn test_strip_trailing() {
        let out = strip_trailing_whitespace(
            "hello   
world  ",
        );
        assert_eq!(
            out,
            "hello
world"
        );
    }
}
#[allow(dead_code)]
pub fn repeat_char(c: char, n: usize) -> String {
    std::iter::repeat(c).take(n).collect()
}
#[allow(dead_code)]
pub fn box_text(text: &str, border: char) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let top = repeat_char(border, width + 4);
    let mut out = String::new();
    out.push_str(&top);
    out.push('\n');
    for line in &lines {
        out.push(border);
        out.push(' ');
        out.push_str(line);
        let pad = width.saturating_sub(line.len());
        out.push_str(&repeat_char(' ', pad));
        out.push(' ');
        out.push(border);
        out.push('\n');
    }
    out.push_str(&top);
    out.push('\n');
    out
}
#[allow(dead_code)]
pub fn numbered_lines(text: &str) -> String {
    text.lines()
        .enumerate()
        .map(|(i, l)| format!("{:>4}: {}", i + 1, l))
        .collect::<Vec<_>>()
        .join(
            "
",
        )
}
#[allow(dead_code)]
pub fn indent_text(text: &str, spaces: usize) -> String {
    let prefix = repeat_char(' ', spaces);
    text.lines()
        .map(|l| format!("{}{}", prefix, l))
        .collect::<Vec<_>>()
        .join(
            "
",
        )
}
#[allow(dead_code)]
pub fn center_text(text: &str, width: usize) -> String {
    let text_len = text.len();
    if text_len >= width {
        return text.to_string();
    }
    let total_pad = width - text_len;
    let left = total_pad / 2;
    let right = total_pad - left;
    format!(
        "{}{}{}",
        repeat_char(' ', left),
        text,
        repeat_char(' ', right)
    )
}
#[allow(dead_code)]
pub fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}
#[allow(dead_code)]
pub fn format_duration_ms(ms: u64) -> String {
    if ms >= 60_000 {
        format!("{:.1}m", ms as f64 / 60_000.0)
    } else if ms >= 1_000 {
        format!("{:.2}s", ms as f64 / 1_000.0)
    } else {
        format!("{}ms", ms)
    }
}
#[allow(dead_code)]
pub fn pluralize(count: usize, singular: &str, plural: &str) -> String {
    if count == 1 {
        format!("1 {}", singular)
    } else {
        format!("{} {}", count, plural)
    }
}
#[allow(dead_code)]
pub fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
#[cfg(test)]
mod format_primitives_tests {
    use super::*;
    #[test]
    fn test_repeat_char() {
        assert_eq!(repeat_char('-', 5), "-----");
    }
    #[test]
    fn test_numbered_lines() {
        let out = numbered_lines(
            "a
b
c",
        );
        assert!(out.contains("   1: a"));
        assert!(out.contains("   2: b"));
    }
    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(512), "512 B");
    }
    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration_ms(500), "500ms");
        assert_eq!(format_duration_ms(1500), "1.50s");
    }
    #[test]
    fn test_pluralize() {
        assert_eq!(pluralize(1, "item", "items"), "1 item");
        assert_eq!(pluralize(3, "item", "items"), "3 items");
    }
    #[test]
    fn test_escape_html() {
        assert_eq!(
            escape_html("<b>foo & \"bar \"</b>"),
            "&lt;b&gt;foo &amp; &quot;bar &quot;&lt;/b&gt;"
        );
    }
}
