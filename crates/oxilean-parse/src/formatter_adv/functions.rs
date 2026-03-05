//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::ast_impl::*;
use std::collections::HashMap;

use super::types::{Annotation, AstFormatter, Doc, FormatConfig, LayoutConfig, LayoutEngine};

/// Concatenate multiple documents with a separator.
#[allow(missing_docs)]
pub fn intersperse(docs: &[Doc], sep: Doc) -> Doc {
    if docs.is_empty() {
        return Doc::Nil;
    }
    let mut result = docs[0].clone();
    for d in &docs[1..] {
        result = result.cat(sep.clone()).cat(d.clone());
    }
    result
}
/// Join documents with spaces.
#[allow(missing_docs)]
pub fn hsep(docs: &[Doc]) -> Doc {
    intersperse(docs, Doc::text(" "))
}
/// Join documents with soft lines.
#[allow(missing_docs)]
pub fn vsep(docs: &[Doc]) -> Doc {
    intersperse(docs, Doc::SoftLine)
}
/// Join documents with hard lines.
#[allow(missing_docs)]
pub fn vcat(docs: &[Doc]) -> Doc {
    intersperse(docs, Doc::HardLine)
}
/// Wrap a document in parentheses.
#[allow(missing_docs)]
pub fn parens(doc: Doc) -> Doc {
    Doc::text("(").cat(doc).cat(Doc::text(")"))
}
/// Wrap a document in brackets.
#[allow(missing_docs)]
pub fn brackets(doc: Doc) -> Doc {
    Doc::text("[").cat(doc).cat(Doc::text("]"))
}
/// Wrap a document in braces.
#[allow(missing_docs)]
pub fn braces(doc: Doc) -> Doc {
    Doc::text("{").cat(doc).cat(Doc::text("}"))
}
/// Wrap a document in double braces.
#[allow(missing_docs)]
pub fn double_braces(doc: Doc) -> Doc {
    Doc::text("{{").cat(doc).cat(Doc::text("}}"))
}
/// Create a keyword document.
#[allow(missing_docs)]
pub fn keyword(s: &str) -> Doc {
    Doc::text(s).annotate(Annotation::Keyword)
}
/// Create an operator document.
#[allow(missing_docs)]
pub fn operator(s: &str) -> Doc {
    Doc::text(s).annotate(Annotation::Operator)
}
/// Create an identifier document.
#[allow(missing_docs)]
pub fn ident(s: &str) -> Doc {
    Doc::text(s).annotate(Annotation::Identifier)
}
/// Create a type name document.
#[allow(missing_docs)]
pub fn type_name(s: &str) -> Doc {
    Doc::text(s).annotate(Annotation::TypeName)
}
/// Format a surface expression to a string.
#[allow(missing_docs)]
pub fn format_expr(expr: &SurfaceExpr) -> String {
    let formatter = AstFormatter::new();
    let doc = formatter.format_expr(expr);
    let engine = LayoutEngine::default_engine();
    engine.layout(&doc)
}
/// Format a surface expression with custom configuration.
#[allow(missing_docs)]
pub fn format_expr_with_config(expr: &SurfaceExpr, config: FormatConfig) -> String {
    let formatter = AstFormatter::with_config(config.clone());
    let doc = formatter.format_expr(expr);
    let engine = LayoutEngine::new(LayoutConfig {
        max_width: config.max_width,
        indent_size: config.indent_size,
        ..LayoutConfig::default()
    });
    engine.layout(&doc)
}
/// Format a declaration to a string.
#[allow(missing_docs)]
pub fn format_decl(decl: &Decl) -> String {
    let formatter = AstFormatter::new();
    let doc = formatter.format_decl(decl);
    let engine = LayoutEngine::default_engine();
    engine.layout(&doc)
}
/// Format an entire module to a string.
#[allow(missing_docs)]
pub fn format_module(decls: &[Located<Decl>]) -> String {
    let formatter = AstFormatter::new();
    let doc = formatter.format_module(decls);
    let engine = LayoutEngine::default_engine();
    engine.layout(&doc)
}
/// Extract comments from source text with their byte offsets.
#[allow(missing_docs)]
pub fn extract_comments(source: &str) -> HashMap<usize, String> {
    let mut comments = HashMap::new();
    let mut offset = 0;
    for line in source.lines() {
        let trimmed = line.trim();
        if let Some(comment) = trimmed.strip_prefix("--") {
            comments.insert(offset, comment.to_string());
        }
        offset += line.len() + 1;
    }
    let bytes = source.as_bytes();
    let mut i = 0;
    while i + 1 < bytes.len() {
        if bytes[i] == b'/' && bytes[i + 1] == b'-' {
            let start = i;
            i += 2;
            let mut depth = 1;
            while i + 1 < bytes.len() && depth > 0 {
                if bytes[i] == b'/' && bytes[i + 1] == b'-' {
                    depth += 1;
                    i += 1;
                } else if bytes[i] == b'-' && bytes[i + 1] == b'/' {
                    depth -= 1;
                    i += 1;
                }
                i += 1;
            }
            let end = i.min(source.len());
            comments.insert(start, source[start..end].to_string());
        } else {
            i += 1;
        }
    }
    comments
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::formatter_adv::*;
    #[test]
    fn test_doc_text() {
        let doc = Doc::text("hello");
        let engine = LayoutEngine::default_engine();
        assert_eq!(engine.layout(&doc), "hello");
    }
    #[test]
    fn test_doc_cat() {
        let doc = Doc::text("hello").space(Doc::text("world"));
        let engine = LayoutEngine::default_engine();
        assert_eq!(engine.layout(&doc), "hello world");
    }
    #[test]
    fn test_doc_hardline() {
        let doc = Doc::text("line1")
            .cat(Doc::HardLine)
            .cat(Doc::text("line2"));
        let engine = LayoutEngine::default_engine();
        assert_eq!(engine.layout(&doc), "line1\nline2");
    }
    #[test]
    fn test_doc_nest() {
        let doc = Doc::text("outer")
            .cat(Doc::HardLine)
            .cat(Doc::text("inner").nest(4));
        let engine = LayoutEngine::default_engine();
        let result = engine.layout(&doc);
        assert!(result.contains("outer\n"));
    }
    #[test]
    fn test_doc_group_fits() {
        let doc = Doc::text("a").line(Doc::text("b")).group();
        let engine = LayoutEngine::new(LayoutConfig {
            max_width: 80,
            ..LayoutConfig::default()
        });
        let result = engine.layout(&doc);
        assert_eq!(result, "a b");
    }
    #[test]
    fn test_format_var() {
        let expr = SurfaceExpr::Var("x".to_string());
        assert_eq!(format_expr(&expr), "x");
    }
    #[test]
    fn test_format_literal() {
        let expr = SurfaceExpr::Lit(Literal::Nat(42));
        assert_eq!(format_expr(&expr), "42");
    }
    #[test]
    fn test_format_hole() {
        let expr = SurfaceExpr::Hole;
        assert_eq!(format_expr(&expr), "_");
    }
    #[test]
    fn test_parens_builder() {
        let doc = parens(Doc::text("x"));
        let engine = LayoutEngine::default_engine();
        assert_eq!(engine.layout(&doc), "(x)");
    }
    #[test]
    fn test_brackets_builder() {
        let doc = brackets(Doc::text("x"));
        let engine = LayoutEngine::default_engine();
        assert_eq!(engine.layout(&doc), "[x]");
    }
    #[test]
    fn test_extract_comments() {
        let source = "-- comment\ndef x := 1\n-- another";
        let comments = extract_comments(source);
        assert!(!comments.is_empty());
    }
}
/// Measures how similar two formatted outputs are (0.0 = identical, 1.0 = completely different).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn format_dissimilarity(a: &str, b: &str) -> f64 {
    if a == b {
        return 0.0;
    }
    let lines_a: std::collections::HashSet<_> = a.lines().collect();
    let lines_b: std::collections::HashSet<_> = b.lines().collect();
    let union = lines_a.union(&lines_b).count();
    let intersection = lines_a.intersection(&lines_b).count();
    if union == 0 {
        0.0
    } else {
        1.0 - intersection as f64 / union as f64
    }
}
/// Formats an expression with explicit precedence parentheses.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn format_with_precedence(expr: &str, prec: u8) -> String {
    if prec > 5 {
        format!("({})", expr)
    } else {
        expr.to_string()
    }
}
/// Escapes special characters in a string for display in formatted output.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn escape_for_display(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\t', "\\t")
}
/// Unescapes display-escaped string.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn unescape_display(s: &str) -> String {
    let mut out = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('\\') => out.push('\\'),
                Some('"') => out.push('"'),
                Some('n') => out.push('\n'),
                Some('t') => out.push('\t'),
                Some(x) => {
                    out.push('\\');
                    out.push(x);
                }
                None => out.push('\\'),
            }
        } else {
            out.push(c);
        }
    }
    out
}
/// Generate an ASCII box around a string.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn box_string(s: &str) -> String {
    let lines: Vec<_> = s.lines().collect();
    let max_w = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let border = format!("+{}+", "-".repeat(max_w + 2));
    let mut out = border.clone();
    for line in &lines {
        out.push('\n');
        out.push_str(&format!("| {}{} |", line, " ".repeat(max_w - line.len())));
    }
    out.push('\n');
    out.push_str(&border);
    out
}
/// Format a key-value table as a grid.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn format_kv_table(pairs: &[(&str, &str)]) -> String {
    let max_k = pairs.iter().map(|(k, _)| k.len()).max().unwrap_or(0);
    pairs
        .iter()
        .map(|(k, v)| format!("{}{} : {}", k, " ".repeat(max_k - k.len()), v))
        .collect::<Vec<_>>()
        .join("\n")
}
/// Splits a formatted output into sections based on blank-line boundaries.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn split_into_sections(text: &str) -> Vec<String> {
    let mut sections = Vec::new();
    let mut current = String::new();
    for line in text.lines() {
        if line.trim().is_empty() && !current.is_empty() {
            sections.push(current.trim().to_string());
            current = String::new();
        } else {
            if !current.is_empty() {
                current.push('\n');
            }
            current.push_str(line);
        }
    }
    if !current.trim().is_empty() {
        sections.push(current.trim().to_string());
    }
    sections
}
/// Heuristic: detect if an expression is "simple" (should be formatted flat).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn is_simple_expr(s: &str) -> bool {
    s.len() < 30 && !s.contains('\n') && s.chars().filter(|&c| c == '(').count() <= 2
}
/// Generate a separator line for use in formatted output.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn separator_line(width: usize, char_: char) -> String {
    std::iter::repeat(char_).take(width).collect()
}
#[cfg(test)]
mod extended_formatter_adv_tests_2 {
    use super::*;
    use crate::formatter_adv::*;
    #[test]
    fn test_formatter_indent_stack() {
        let mut stack = FormatterIndentStack::new();
        stack.push(2);
        assert_eq!(stack.current(), 2);
        stack.push(4);
        assert_eq!(stack.current(), 6);
        stack.pop();
        assert_eq!(stack.current(), 2);
    }
    #[test]
    fn test_ribbon_formatter() {
        let rf = RibbonFormatter::new(100, 0.6);
        assert_eq!(rf.ribbon_width(), 60);
        assert!(rf.fits(10, 40));
        assert!(!rf.fits(10, 80));
    }
    #[test]
    fn test_annotated_output() {
        let mut ao = AnnotatedOutput::new("hello world");
        ao.annotate(0, 5, 1);
        ao.annotate(6, 11, 2);
        assert_eq!(ao.node_at(0), Some(1));
        assert_eq!(ao.node_at(7), Some(2));
        assert_eq!(ao.node_at(5), None);
        assert_eq!(ao.annotation_count(), 2);
    }
    #[test]
    fn test_line_length_distribution() {
        let text = "hello\nworld!\nhi";
        let dist = LineLengthDistribution::compute(text);
        assert_eq!(dist.total_lines, 3);
        assert_eq!(dist.max_length(), 6);
        assert!(dist.mean_length() > 3.0);
    }
    #[test]
    fn test_line_breaker() {
        let lb = LineBreaker::new(10, 0, " ");
        let tokens = ["hello", "world", "foo", "bar"];
        let broken = lb.break_tokens(&tokens);
        for line in broken.lines() {
            assert!(line.len() <= 12);
        }
    }
    #[test]
    fn test_formatter_config() {
        let cfg = FormatterConfig::default_config();
        assert_eq!(cfg.line_width, 100);
        assert_eq!(cfg.indent_str(), "  ");
        let compact = FormatterConfig::compact();
        assert_eq!(compact.indent_size, 4);
    }
    #[test]
    fn test_format_dissimilarity() {
        assert_eq!(format_dissimilarity("a\nb", "a\nb"), 0.0);
        let d = format_dissimilarity("a\nb\nc", "a\nX\nc");
        assert!(d > 0.0 && d < 1.0);
    }
    #[test]
    fn test_format_context() {
        let ctx = FormatContext::new(80);
        let ctx2 = ctx.indented(4);
        assert_eq!(ctx2.indent, 4);
        assert_eq!(ctx2.remaining_width(), 76);
        let ctx3 = ctx2.in_type_mode();
        assert!(ctx3.in_type);
    }
    #[test]
    fn test_format_decision_log() {
        let mut log = FormatDecisionLog::new();
        log.record("expr1", true, 20, 80);
        log.record("expr2", false, 90, 80);
        assert_eq!(log.count(), 2);
        assert!((log.flat_fraction() - 0.5).abs() < 1e-9);
    }
    #[test]
    fn test_escape_unescape() {
        let original = "hello\nworld\t\"quote\"";
        let escaped = escape_for_display(original);
        let restored = unescape_display(&escaped);
        assert_eq!(restored, original);
    }
    #[test]
    fn test_box_string() {
        let boxed = box_string("hello\nworld");
        assert!(boxed.contains("+"));
        assert!(boxed.contains("| hello |") || boxed.contains("hello"));
    }
    #[test]
    fn test_format_kv_table() {
        let pairs = [("name", "Alice"), ("age", "30")];
        let table = format_kv_table(&pairs);
        assert!(table.contains("name"));
        assert!(table.contains("Alice"));
    }
    #[test]
    fn test_split_into_sections() {
        let text = "a\nb\n\nc\nd";
        let sections = split_into_sections(text);
        assert_eq!(sections.len(), 2);
        assert_eq!(sections[0], "a\nb");
        assert_eq!(sections[1], "c\nd");
    }
    #[test]
    fn test_is_simple_expr() {
        assert!(is_simple_expr("x + y"));
        assert!(!is_simple_expr(&"x".repeat(50)));
    }
    #[test]
    fn test_separator_line() {
        let sep = separator_line(10, '-');
        assert_eq!(sep, "----------");
        assert_eq!(sep.len(), 10);
    }
    #[test]
    fn test_format_with_precedence() {
        assert_eq!(format_with_precedence("x + y", 6), "(x + y)");
        assert_eq!(format_with_precedence("x", 3), "x");
    }
}
/// Word wrap utility.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn word_wrap2(text: &str, width: usize) -> String {
    let mut out = String::new();
    let mut line_len = 0usize;
    for word in text.split_whitespace() {
        if line_len > 0 && line_len + word.len() + 1 > width {
            out.push('\n');
            line_len = 0;
        } else if line_len > 0 {
            out.push(' ');
            line_len += 1;
        }
        out.push_str(word);
        line_len += word.len();
    }
    out
}
/// Truncate with ellipsis.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn truncate_ellipsis(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}
/// Add line numbers.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn numbered_lines(s: &str) -> String {
    s.lines()
        .enumerate()
        .map(|(i, l)| format!("{:4} | {}", i + 1, l))
        .collect::<Vec<_>>()
        .join("\n")
}
/// Format type annotation.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn fmt_type_ann(name: &str, ty: &str) -> String {
    format!("({} : {})", name, ty)
}
/// Format lambda.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn fmt_lambda(params: &[&str], body: &str) -> String {
    format!("fun {} -> {}", params.join(" "), body)
}
/// Format forall.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn fmt_forall(binders: &[(&str, &str)], body: &str) -> String {
    let bs: Vec<_> = binders
        .iter()
        .map(|(n, t)| format!("({} : {})", n, t))
        .collect();
    format!("forall {}, {}", bs.join(" "), body)
}
/// Format match.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn fmt_match(scrutinee: &str, arms: &[(&str, &str)]) -> String {
    let mut out = format!("match {} with\n", scrutinee);
    for (pat, body) in arms {
        out.push_str(&format!("  | {} -> {}\n", pat, body));
    }
    out
}
/// Check canonical format.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn check_canonical(s: &str) -> bool {
    if s.lines().any(|l| l.ends_with(' ')) {
        return false;
    }
    let mut prev_blank = false;
    for line in s.lines() {
        let blank = line.trim().is_empty();
        if blank && prev_blank {
            return false;
        }
        prev_blank = blank;
    }
    true
}
/// Canonicalise.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn canonicalise(s: &str) -> String {
    let mut out = String::new();
    let mut prev_blank = false;
    for line in s.lines() {
        let trimmed = line.trim_end();
        let blank = trimmed.is_empty();
        if blank && prev_blank {
            continue;
        }
        if !out.is_empty() {
            out.push('\n');
        }
        out.push_str(trimmed);
        prev_blank = blank;
    }
    out
}
/// Format a list.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn fmt_list(items: &[&str]) -> String {
    format!("[{}]", items.join(", "))
}
/// Format a tuple.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn fmt_tuple(items: &[&str]) -> String {
    format!("({})", items.join(", "))
}
/// Format a record.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn fmt_record(fields: &[(&str, &str)]) -> String {
    let body: Vec<_> = fields
        .iter()
        .map(|(k, v)| format!("{} := {}", k, v))
        .collect();
    format!("{{ {} }}", body.join(", "))
}
/// Format an if-then-else.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn fmt_ite(cond: &str, then_: &str, else_: &str) -> String {
    format!("if {} then {} else {}", cond, then_, else_)
}
/// Format fn signature.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn fmt_fn_sig(name: &str, args: &[(&str, &str)], ret: &str) -> String {
    let args_str: Vec<_> = args.iter().map(|(n, t)| fmt_type_ann(n, t)).collect();
    format!("def {} {} : {}", name, args_str.join(" "), ret)
}
/// Escape for display.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn escape_display(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\t', "\\t")
}
/// Diff two strings line by line.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn line_diff(a: &str, b: &str) -> Vec<String> {
    let la: Vec<_> = a.lines().collect();
    let lb: Vec<_> = b.lines().collect();
    let mut diff = Vec::new();
    let max = la.len().max(lb.len());
    for i in 0..max {
        match (la.get(i), lb.get(i)) {
            (Some(x), Some(y)) if x == y => diff.push(format!("  {}", x)),
            (Some(x), Some(y)) => {
                diff.push(format!("- {}", x));
                diff.push(format!("+ {}", y));
            }
            (Some(x), None) => diff.push(format!("- {}", x)),
            (None, Some(y)) => diff.push(format!("+ {}", y)),
            (None, None) => {}
        }
    }
    diff
}
/// Count added parens.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn count_parens_added(original: &str, formatted: &str) -> usize {
    let orig = original.chars().filter(|&c| c == '(').count();
    let fmt = formatted.chars().filter(|&c| c == '(').count();
    fmt.saturating_sub(orig)
}
/// Expansion ratio.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn expansion_ratio(original: &str, formatted: &str) -> f64 {
    if original.is_empty() {
        1.0
    } else {
        formatted.len() as f64 / original.len() as f64
    }
}
/// Generate box around string.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn ascii_box(s: &str) -> String {
    let lines: Vec<_> = s.lines().collect();
    let max_w = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let border = format!("+{}+", "-".repeat(max_w + 2));
    let mut out = border.clone();
    for line in &lines {
        out.push_str(&format!("\n| {}{} |", line, " ".repeat(max_w - line.len())));
    }
    out.push('\n');
    out.push_str(&border);
    out
}
/// KV table.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn kv_table(pairs: &[(&str, &str)]) -> String {
    let max_k = pairs.iter().map(|(k, _)| k.len()).max().unwrap_or(0);
    pairs
        .iter()
        .map(|(k, v)| format!("{}{} : {}", k, " ".repeat(max_k - k.len()), v))
        .collect::<Vec<_>>()
        .join("\n")
}
/// Heuristic simple expr check.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn simple_expr(s: &str) -> bool {
    s.len() < 30 && !s.contains('\n') && s.chars().filter(|&c| c == '(').count() <= 2
}
/// Separator line.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn sep_line(width: usize, ch: char) -> String {
    std::iter::repeat(ch).take(width).collect()
}
/// Stabilise format.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn stabilise<F: Fn(&str) -> String>(input: &str, f: F, max: usize) -> (String, usize) {
    let mut cur = input.to_string();
    for i in 0..max {
        let next = f(&cur);
        if next == cur {
            return (cur, i);
        }
        cur = next;
    }
    (cur, max)
}
#[cfg(test)]
mod extended_formatter_adv_tests_3 {
    use super::*;
    use crate::formatter_adv::*;
    #[test]
    fn test_format_token2() {
        let t = FormatToken::simple("hello").with_priority(FormatPriority::High);
        assert_eq!(t.priority, FormatPriority::High);
        assert_eq!(t.len(), 5);
    }
    #[test]
    fn test_token_stream2_render() {
        let mut ts = TokenStream2::new(80);
        ts.push(FormatToken::simple("a"));
        ts.push(FormatToken::simple("b"));
        let r = ts.render();
        assert!(r.contains("a"));
        assert!(r.contains("b"));
        assert_eq!(ts.token_count(), 2);
    }
    #[test]
    fn test_infix_expr_builder() {
        let b = InfixExprBuilder::new("x").add("+", "y").add("+", "z");
        assert_eq!(b.build_flat(), "x + y + z");
        let broken = b.build_broken(2);
        assert!(broken.contains('\n'));
    }
    #[test]
    fn test_let_binding_aligner2() {
        let mut a = LetBindingAligner2::new();
        a.add("x", "1");
        a.add("foo", "2");
        let r = a.render_aligned();
        assert!(r.contains("let x"));
        assert!(r.contains("let foo"));
    }
    #[test]
    fn test_flat_or_broken2() {
        let fb = FlatOrBroken2::new("short", "long\nversion");
        assert_eq!(fb.choose(100, 0), "short");
        assert_eq!(fb.choose(3, 0), "long\nversion");
    }
    #[test]
    fn test_word_wrap2() {
        let w = word_wrap2("the quick brown fox", 8);
        for line in w.lines() {
            assert!(line.len() <= 9);
        }
    }
    #[test]
    fn test_truncate_ellipsis() {
        assert_eq!(truncate_ellipsis("hello world", 8), "hello...");
        assert_eq!(truncate_ellipsis("hi", 10), "hi");
    }
    #[test]
    fn test_numbered_lines() {
        let n = numbered_lines("a\nb");
        assert!(n.contains("1 | a") || n.contains("   1 | a"));
    }
    #[test]
    fn test_fmt_helpers() {
        assert_eq!(fmt_type_ann("x", "Nat"), "(x : Nat)");
        assert_eq!(fmt_lambda(&["x", "y"], "x"), "fun x y -> x");
        assert_eq!(fmt_list(&["1", "2"]), "[1, 2]");
        assert_eq!(fmt_tuple(&["a", "b"]), "(a, b)");
        assert_eq!(fmt_ite("b", "t", "f"), "if b then t else f");
        assert_eq!(fmt_record(&[("k", "v")]), "{ k := v }");
    }
    #[test]
    fn test_check_canonical() {
        assert!(check_canonical("hello\nworld"));
        assert!(!check_canonical("hello \n"));
        assert!(!check_canonical("a\n\n\nb"));
    }
    #[test]
    fn test_canonicalise() {
        let r = canonicalise("a  \n\n\nb  ");
        assert!(check_canonical(&r));
    }
    #[test]
    fn test_fmt_match() {
        let m = fmt_match("n", &[("0", "Z"), ("k", "S k")]);
        assert!(m.contains("match n with"));
        assert!(m.contains("| 0 -> Z"));
    }
    #[test]
    fn test_fmt_forall() {
        let s = fmt_forall(&[("x", "Nat")], "x > 0");
        assert!(s.starts_with("forall"));
        assert!(s.contains("(x : Nat)"));
    }
    #[test]
    fn test_escape_display() {
        let s = escape_display("a\nb");
        assert!(s.contains("\\n"));
    }
    #[test]
    fn test_line_diff() {
        let d = line_diff("a\nb", "a\nX");
        assert!(d.iter().any(|l| l.starts_with("- b")));
        assert!(d.iter().any(|l| l.starts_with("+ X")));
    }
    #[test]
    fn test_count_parens_added() {
        assert_eq!(count_parens_added("x + y", "(x + y)"), 1);
    }
    #[test]
    fn test_expansion_ratio() {
        assert!(expansion_ratio("x", "x + 0") > 1.0);
        assert_eq!(expansion_ratio("", "abc"), 1.0);
    }
    #[test]
    fn test_ascii_box() {
        let b = ascii_box("hi");
        assert!(b.contains('+'));
        assert!(b.contains("hi"));
    }
    #[test]
    fn test_kv_table() {
        let t = kv_table(&[("name", "Alice"), ("age", "30")]);
        assert!(t.contains("name"));
        assert!(t.contains("Alice"));
    }
    #[test]
    fn test_simple_expr() {
        assert!(simple_expr("x + y"));
        assert!(!simple_expr(&"x".repeat(50)));
    }
    #[test]
    fn test_sep_line() {
        assert_eq!(sep_line(5, '-'), "-----");
    }
    #[test]
    fn test_stabilise() {
        let (r, _) = stabilise("  hello  ", |s| s.trim().to_string(), 10);
        assert_eq!(r, "hello");
    }
    #[test]
    fn test_fmt_fn_sig() {
        let s = fmt_fn_sig("foo", &[("x", "Nat")], "Bool");
        assert!(s.contains("def foo"));
        assert!(s.contains(": Bool"));
    }
}
/// Formats a list of items with optional trailing comma.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn format_comma_list(items: &[String], trailing_comma: bool) -> String {
    if items.is_empty() {
        return String::new();
    }
    let mut out = items.join(", ");
    if trailing_comma {
        out.push(',');
    }
    out
}
/// Formats a multi-line comment block with optional header.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn format_doc_comment(header: Option<&str>, lines: &[&str]) -> String {
    let mut out = String::from("/--\n");
    if let Some(h) = header {
        out.push_str(&format!("  {}\n\n", h));
    }
    for line in lines {
        out.push_str(&format!("  {}\n", line));
    }
    out.push_str("--/");
    out
}
/// Normalise whitespace: collapse multiple spaces, trim lines.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn normalise_whitespace(s: &str) -> String {
    s.lines()
        .map(|l| l.split_whitespace().collect::<Vec<_>>().join(" "))
        .collect::<Vec<_>>()
        .join("\n")
}
/// Remove all comments from source (lines starting with `--`).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn strip_comments(s: &str) -> String {
    s.lines()
        .filter(|l| !l.trim_start().starts_with("--"))
        .collect::<Vec<_>>()
        .join("\n")
}
/// Count identifiers in a formatted string (word characters not starting with digit).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn count_identifiers(s: &str) -> usize {
    let mut count = 0;
    let mut in_word = false;
    let mut word_start = true;
    for ch in s.chars() {
        if ch.is_alphanumeric() || ch == '_' {
            if !in_word {
                in_word = true;
                word_start = !ch.is_ascii_digit();
            }
        } else {
            if in_word && word_start {
                count += 1;
            }
            in_word = false;
            word_start = false;
        }
    }
    if in_word && word_start {
        count += 1;
    }
    count
}
/// Detect if output has consistent indentation (all indents are multiples of `unit`).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn has_consistent_indentation(s: &str, unit: usize) -> bool {
    if unit == 0 {
        return true;
    }
    s.lines().all(|l| {
        let indent = l.len() - l.trim_start().len();
        indent % unit == 0
    })
}
/// Format a sequence of items one-per-line with a prefix.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn format_bulleted_list(items: &[&str], bullet: &str) -> String {
    items
        .iter()
        .map(|item| format!("{} {}", bullet, item))
        .collect::<Vec<_>>()
        .join("\n")
}
/// Check if a string ends with exactly one newline.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn ends_with_single_newline(s: &str) -> bool {
    s.ends_with('\n') && !s.ends_with("\n\n")
}
/// Ensure string ends with exactly one newline.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn ensure_trailing_newline(s: &str) -> String {
    let trimmed = s.trim_end_matches('\n');
    format!("{}\n", trimmed)
}
/// Format a source file header with metadata.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn format_file_header(module_name: &str, imports: &[&str]) -> String {
    let mut out = format!("-- Module: {}\n\n", module_name);
    for import in imports {
        out.push_str(&format!("import {}\n", import));
    }
    if !imports.is_empty() {
        out.push('\n');
    }
    out
}
#[cfg(test)]
mod extended_formatter_adv_tests_4 {
    use super::*;
    use crate::formatter_adv::*;
    #[test]
    fn test_labeled_section_formatter() {
        let mut f = LabeledSectionFormatter::new();
        f.add_section("Header", "content here");
        assert_eq!(f.section_count(), 1);
        let r = f.render(20);
        assert!(r.contains("Header"));
        assert!(r.contains("content here"));
    }
    #[test]
    fn test_format_comma_list() {
        assert_eq!(
            format_comma_list(&["a".into(), "b".into(), "c".into()], false),
            "a, b, c"
        );
        assert_eq!(format_comma_list(&["x".into()], true), "x,");
        assert_eq!(format_comma_list(&[], false), "");
    }
    #[test]
    fn test_format_doc_comment() {
        let doc = format_doc_comment(Some("A header"), &["line 1", "line 2"]);
        assert!(doc.contains("/--"));
        assert!(doc.contains("A header"));
        assert!(doc.contains("line 1"));
        assert!(doc.contains("--/"));
    }
    #[test]
    fn test_doc_width() {
        let a = DocWidth::atom(5);
        let b = DocWidth::atom(3);
        let c = DocWidth::concat(a, b);
        assert_eq!(c.flat, 8);
        assert!(a.fits_in(10));
        assert!(!a.fits_in(4));
    }
    #[test]
    fn test_syntax_highlight_formatter() {
        let mut f = SyntaxHighlightFormatter::new();
        f.add_span(0, 3, HighlightKind::Keyword);
        f.add_span(4, 7, HighlightKind::Identifier);
        assert_eq!(f.spans_of_kind(HighlightKind::Keyword).len(), 1);
        assert_eq!(f.total_highlighted_chars(), 6);
    }
    #[test]
    fn test_decl_format() {
        let df = DeclFormat::OneLiner("def x := 1".into());
        assert!(df.is_one_liner());
        assert_eq!(df.line_count(), 1);
        let multi = DeclFormat::MultiLine(vec!["def x :=".into(), "  1".into()]);
        assert!(!multi.is_one_liner());
        assert_eq!(multi.line_count(), 2);
        assert!(multi.render().contains('\n'));
    }
    #[test]
    fn test_format_queue() {
        let mut q = FormatQueue::new();
        q.enqueue("task1");
        q.enqueue("task2");
        assert_eq!(q.len(), 2);
        assert_eq!(q.dequeue(), Some("task1".into()));
        assert_eq!(q.len(), 1);
    }
    #[test]
    fn test_normalise_whitespace() {
        let r = normalise_whitespace("  hello   world  \n  foo  ");
        assert_eq!(r, "hello world\nfoo");
    }
    #[test]
    fn test_strip_comments() {
        let src = "def x := 1\n-- this is a comment\ndef y := 2";
        let stripped = strip_comments(src);
        assert!(!stripped.contains("this is a comment"));
        assert!(stripped.contains("def x := 1"));
    }
    #[test]
    fn test_count_identifiers() {
        let count = count_identifiers("fun x y -> x + y");
        assert!(count >= 3);
    }
    #[test]
    fn test_has_consistent_indentation() {
        assert!(has_consistent_indentation("a\n  b\n    c", 2));
        assert!(!has_consistent_indentation("a\n   b", 2));
    }
    #[test]
    fn test_format_bulleted_list() {
        let r = format_bulleted_list(&["apple", "banana"], "-");
        assert_eq!(r, "- apple\n- banana");
    }
    #[test]
    fn test_ensure_trailing_newline() {
        assert_eq!(ensure_trailing_newline("hello"), "hello\n");
        assert_eq!(ensure_trailing_newline("hello\n"), "hello\n");
        assert_eq!(ensure_trailing_newline("hello\n\n"), "hello\n");
    }
    #[test]
    fn test_format_file_header() {
        let h = format_file_header("Foo.Bar", &["Foo.Baz", "Foo.Quux"]);
        assert!(h.contains("Module: Foo.Bar"));
        assert!(h.contains("import Foo.Baz"));
    }
    #[test]
    fn test_tree_renderer() {
        let mut tr = TreeRenderer::new(2);
        tr.add(0, "root");
        tr.add(1, "child");
        tr.add(2, "grandchild");
        assert_eq!(tr.node_count(), 3);
        let r = tr.render();
        assert!(r.contains("|- root"));
        assert!(r.contains("  |- child"));
        assert!(r.contains("    |- grandchild"));
    }
    #[test]
    fn test_ends_with_single_newline() {
        assert!(ends_with_single_newline("hello\n"));
        assert!(!ends_with_single_newline("hello\n\n"));
        assert!(!ends_with_single_newline("hello"));
    }
}
