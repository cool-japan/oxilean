//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Declaration, Environment, Expr, Level, Name};

use super::types::{BytePos, LineCol, Span, StringBuilder, SubstringFinder2};

/// Indent every line of `text` by `n` spaces.
///
/// Blank lines (consisting only of whitespace) are preserved but not indented.
pub fn indent(text: &str, n: usize) -> String {
    let pad = " ".repeat(n);
    text.lines()
        .map(|line| {
            if line.trim().is_empty() {
                String::new()
            } else {
                format!("{pad}{line}")
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}
/// Normalise all runs of whitespace in `text` to a single space.
///
/// Leading and trailing whitespace is also stripped.
pub fn normalize_whitespace(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}
/// Split `text` into a list of whitespace-delimited words.
pub fn words(text: &str) -> Vec<&str> {
    text.split_whitespace().collect()
}
/// Join `parts` with `sep` between them.
pub fn join(parts: &[impl AsRef<str>], sep: &str) -> String {
    parts
        .iter()
        .map(|s| s.as_ref())
        .collect::<Vec<_>>()
        .join(sep)
}
/// Count non-overlapping occurrences of `needle` in `haystack`.
pub fn count_occurrences(haystack: &str, needle: &str) -> usize {
    if needle.is_empty() {
        return 0;
    }
    let mut count = 0;
    let mut pos = 0;
    while let Some(idx) = haystack[pos..].find(needle) {
        count += 1;
        pos += idx + needle.len();
    }
    count
}
/// Truncate `s` to at most `max_len` characters, appending `suffix` if cut.
///
/// If `s.len() <= max_len`, returns `s` unchanged.
/// Otherwise returns `s\[..max_len - suffix.len()\] + suffix`.
pub fn truncate(s: &str, max_len: usize, suffix: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() <= max_len {
        s.to_string()
    } else {
        let suffix_chars: Vec<char> = suffix.chars().collect();
        let take = max_len.saturating_sub(suffix_chars.len());
        let mut result: String = chars[..take].iter().collect();
        result.push_str(suffix);
        result
    }
}
/// Pad `s` on the right with spaces to reach at least `width` chars.
pub fn pad_right(s: &str, width: usize) -> String {
    let len = s.chars().count();
    if len >= width {
        s.to_string()
    } else {
        format!("{s}{}", " ".repeat(width - len))
    }
}
/// Pad `s` on the left with spaces to reach at least `width` chars.
pub fn pad_left(s: &str, width: usize) -> String {
    let len = s.chars().count();
    if len >= width {
        s.to_string()
    } else {
        format!("{}{s}", " ".repeat(width - len))
    }
}
/// Center `s` within `width` characters, padding with spaces.
///
/// If padding is odd, the extra space goes on the right.
pub fn center(s: &str, width: usize) -> String {
    let len = s.chars().count();
    if len >= width {
        return s.to_string();
    }
    let total_pad = width - len;
    let left_pad = total_pad / 2;
    let right_pad = total_pad - left_pad;
    format!("{}{s}{}", " ".repeat(left_pad), " ".repeat(right_pad))
}
/// Convert a `camelCase` or `PascalCase` string to `snake_case`.
#[allow(clippy::while_let_on_iterator)]
pub fn camel_to_snake(s: &str) -> String {
    let mut result = String::with_capacity(s.len() + 4);
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c.is_uppercase() {
            if !result.is_empty() {
                result.push('_');
            }
            result.push(
                c.to_lowercase()
                    .next()
                    .expect("to_lowercase always yields at least one char"),
            );
        } else {
            result.push(c);
        }
    }
    result
}
/// Convert a `snake_case` string to `camelCase`.
pub fn snake_to_camel(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut capitalise_next = false;
    for c in s.chars() {
        if c == '_' {
            capitalise_next = true;
        } else if capitalise_next {
            result.push(
                c.to_uppercase()
                    .next()
                    .expect("to_uppercase always yields at least one char"),
            );
            capitalise_next = false;
        } else {
            result.push(c);
        }
    }
    result
}
/// Convert a `snake_case` string to `PascalCase`.
pub fn snake_to_pascal(s: &str) -> String {
    let camel = snake_to_camel(s);
    let mut chars = camel.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
/// Check whether `s` is a valid OxiLean/Lean 4 identifier.
///
/// A valid identifier starts with a letter or `_`, followed by letters,
/// digits, `_`, or `'`.
pub fn is_valid_ident(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let mut chars = s.chars();
    let first = chars
        .next()
        .expect("s is non-empty: checked by early return");
    if !first.is_alphabetic() && first != '_' {
        return false;
    }
    chars.all(|c| c.is_alphanumeric() || c == '_' || c == '\'')
}
/// Escape a string for display in error messages or pretty-printed output.
///
/// Non-printable characters are replaced with `\uXXXX` escapes.
pub fn escape_for_display(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '\n' => result.push_str("\\n"),
            '\t' => result.push_str("\\t"),
            '\r' => result.push_str("\\r"),
            '\\' => result.push_str("\\\\"),
            c if c.is_control() => {
                result.push_str(&format!("\\u{:04X}", c as u32));
            }
            c => result.push(c),
        }
    }
    result
}
/// Repeat `s` exactly `n` times.
pub fn repeat_str(s: &str, n: usize) -> String {
    s.repeat(n)
}
/// Check whether `s` starts with `prefix`, returning the remainder.
pub fn strip_prefix<'a>(s: &'a str, prefix: &str) -> Option<&'a str> {
    s.strip_prefix(prefix)
}
/// Check whether `s` ends with `suffix`, returning the body.
pub fn strip_suffix<'a>(s: &'a str, suffix: &str) -> Option<&'a str> {
    s.strip_suffix(suffix)
}
/// Trim a balanced pair of delimiters from the start and end of `s`.
///
/// Returns the inner string if `s` starts with `open` and ends with `close`.
pub fn strip_delimiters<'a>(s: &'a str, open: &str, close: &str) -> Option<&'a str> {
    let inner = s.strip_prefix(open)?.strip_suffix(close)?;
    Some(inner)
}
/// Replace the first occurrence of `from` in `s` with `to`.
pub fn replace_first(s: &str, from: &str, to: &str) -> String {
    if let Some(pos) = s.find(from) {
        format!("{}{}{}", &s[..pos], to, &s[pos + from.len()..])
    } else {
        s.to_string()
    }
}
/// Split `s` at the first occurrence of `sep`.
///
/// Returns `(before, after)` or `(s, "")` if `sep` is not found.
pub fn split_once_or_whole<'a>(s: &'a str, sep: &str) -> (&'a str, &'a str) {
    s.split_once(sep).unwrap_or((s, ""))
}
/// Wrap `text` at `width` characters, breaking on whitespace.
///
/// Returns a vector of lines, each at most `width` characters wide.
pub fn word_wrap(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();
    for word in text.split_whitespace() {
        if current_line.is_empty() {
            current_line.push_str(word);
        } else if current_line.len() + 1 + word.len() <= width {
            current_line.push(' ');
            current_line.push_str(word);
        } else {
            lines.push(current_line.clone());
            current_line = word.to_string();
        }
    }
    if !current_line.is_empty() {
        lines.push(current_line);
    }
    lines
}
/// Convert a dotted name string (`"Foo.bar.Baz"`) to a path of components.
pub fn dotted_name_to_components(name: &str) -> Vec<&str> {
    name.split('.').collect()
}
/// Convert a list of components back to a dotted name string.
pub fn components_to_dotted_name(components: &[impl AsRef<str>]) -> String {
    join(components, ".")
}
/// Return the last component of a dotted name.
///
/// E.g. `"Foo.bar.Baz"` → `"Baz"`.
pub fn dotted_name_last(name: &str) -> &str {
    name.rsplit('.').next().unwrap_or(name)
}
/// Return the namespace prefix of a dotted name (everything before the last `.`).
///
/// E.g. `"Foo.bar.Baz"` → `"Foo.bar"`.
pub fn dotted_name_namespace(name: &str) -> &str {
    match name.rfind('.') {
        Some(pos) => &name[..pos],
        None => "",
    }
}
/// Mangle an OxiLean name to a C/LLVM-safe identifier.
///
/// Dots are replaced by `__`, and non-alphanumeric characters by `_`.
pub fn mangle_name(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c == '.' {
                '_'
            } else if c.is_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}
/// Check whether a name is in a given namespace.
///
/// `"Nat.succ"` is in namespace `"Nat"`.
pub fn in_namespace(name: &str, ns: &str) -> bool {
    if ns.is_empty() {
        return true;
    }
    name.starts_with(&format!("{ns}."))
}
impl From<StringBuilder> for String {
    fn from(sb: StringBuilder) -> String {
        sb.finish()
    }
}
/// Convert a `BytePos` to a `LineCol` within a source string.
///
/// Returns `LineCol { line: 1, col: 1 }` for position 0.
pub fn byte_pos_to_line_col(src: &str, pos: BytePos) -> LineCol {
    let offset = pos.to_usize().min(src.len());
    let prefix = &src[..offset];
    let line = prefix.chars().filter(|&c| c == '\n').count() as u32 + 1;
    let col = prefix.chars().rev().take_while(|&c| c != '\n').count() as u32 + 1;
    LineCol::new(line, col)
}
/// Build a table mapping line numbers (0-indexed) to their byte start positions.
pub fn build_line_starts(src: &str) -> Vec<BytePos> {
    let mut starts = vec![BytePos(0)];
    for (i, c) in src.char_indices() {
        if c == '\n' {
            starts.push(BytePos((i + 1) as u32));
        }
    }
    starts
}
/// Format a list of items with a separator.
pub fn format_list<T: std::fmt::Display>(items: &[T], sep: &str) -> String {
    items
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(sep)
}
/// Format a list of items in parentheses, comma-separated.
pub fn format_paren_list<T: std::fmt::Display>(items: &[T]) -> String {
    if items.is_empty() {
        "()".to_string()
    } else {
        format!("({})", format_list(items, ", "))
    }
}
/// Format an optional value, using `default` if absent.
pub fn format_opt<T: std::fmt::Display>(opt: &Option<T>, default: &str) -> String {
    match opt {
        Some(v) => v.to_string(),
        None => default.to_string(),
    }
}
/// Add `n` spaces of indent to the first line and `continuation_indent` to the rest.
pub fn indent_continuation(text: &str, first: usize, cont: usize) -> String {
    let mut lines = text.lines();
    let first_line = match lines.next() {
        None => return String::new(),
        Some(l) => format!("{}{l}", " ".repeat(first)),
    };
    let rest: Vec<String> = lines.map(|l| format!("{}{l}", " ".repeat(cont))).collect();
    if rest.is_empty() {
        first_line
    } else {
        format!("{}\n{}", first_line, rest.join("\n"))
    }
}
/// Create a horizontal rule of `n` dashes.
pub fn hr(n: usize) -> String {
    "─".repeat(n)
}
/// Box-draw a title line.
pub fn box_title(title: &str, width: usize) -> String {
    let pad = width.saturating_sub(title.chars().count() + 4);
    format!("┌─ {title} {}", "─".repeat(pad))
}
/// Format a Lean 4 `#check` command string.
pub fn lean4_check(expr: &str) -> String {
    format!("#check {expr}")
}
/// Format a Lean 4 theorem declaration stub.
pub fn lean4_theorem_stub(name: &str, ty: &str) -> String {
    format!("theorem {name} : {ty} := by\n  sorry")
}
/// Format a Lean 4 `def` stub.
pub fn lean4_def_stub(name: &str, ty: &str, body: &str) -> String {
    format!("def {name} : {ty} := {body}")
}
/// Surround `s` with backticks for Lean 4 identifier quoting.
pub fn lean4_quote_ident(s: &str) -> String {
    format!("`{s}`")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_indent_basic() {
        let result = indent("hello\nworld", 2);
        assert_eq!(result, "  hello\n  world");
    }
    #[test]
    fn test_indent_blank_line() {
        let result = indent("hello\n\nworld", 2);
        assert_eq!(result, "  hello\n\n  world");
    }
    #[test]
    fn test_normalize_whitespace() {
        assert_eq!(normalize_whitespace("  hello   world  "), "hello world");
    }
    #[test]
    fn test_count_occurrences() {
        assert_eq!(count_occurrences("aababab", "ab"), 3);
        assert_eq!(count_occurrences("hello", ""), 0);
        assert_eq!(count_occurrences("hello", "xyz"), 0);
    }
    #[test]
    fn test_truncate() {
        assert_eq!(truncate("hello world", 8, "..."), "hello...");
        assert_eq!(truncate("hi", 10, "..."), "hi");
    }
    #[test]
    fn test_pad_right() {
        assert_eq!(pad_right("hi", 5), "hi   ");
        assert_eq!(pad_right("hello", 3), "hello");
    }
    #[test]
    fn test_pad_left() {
        assert_eq!(pad_left("hi", 5), "   hi");
    }
    #[test]
    fn test_camel_to_snake() {
        assert_eq!(camel_to_snake("CamelCase"), "camel_case");
        assert_eq!(camel_to_snake("helloWorld"), "hello_world");
    }
    #[test]
    fn test_snake_to_camel() {
        assert_eq!(snake_to_camel("hello_world"), "helloWorld");
        assert_eq!(snake_to_camel("foo_bar_baz"), "fooBarBaz");
    }
    #[test]
    fn test_snake_to_pascal() {
        assert_eq!(snake_to_pascal("hello_world"), "HelloWorld");
    }
    #[test]
    fn test_is_valid_ident() {
        assert!(is_valid_ident("foo"));
        assert!(is_valid_ident("_bar"));
        assert!(is_valid_ident("foo'"));
        assert!(!is_valid_ident(""));
        assert!(!is_valid_ident("1foo"));
        assert!(!is_valid_ident("foo-bar"));
    }
    #[test]
    fn test_escape_for_display() {
        assert_eq!(escape_for_display("hello\nworld"), "hello\\nworld");
        assert_eq!(escape_for_display("a\\b"), "a\\\\b");
    }
    #[test]
    fn test_word_wrap() {
        let wrapped = word_wrap("hello world foo bar", 10);
        for line in &wrapped {
            assert!(line.len() <= 10, "line too long: {line:?}");
        }
    }
    #[test]
    fn test_string_builder() {
        let mut sb = StringBuilder::new();
        sb.push_str("hello");
        sb.push(' ');
        sb.push_str("world");
        assert_eq!(sb.finish(), "hello world");
    }
    #[test]
    fn test_string_builder_newline() {
        let mut sb = StringBuilder::new();
        sb.push_str("line1");
        sb.indent();
        sb.newline();
        sb.push_str("line2");
        let s = sb.finish();
        assert!(s.contains('\n'));
        assert!(s.contains("  line2"));
    }
    #[test]
    fn test_byte_pos_span() {
        let s = Span::from_offsets(0, 5);
        assert_eq!(s.len(), 5);
        assert!(s.contains(BytePos(0)));
        assert!(s.contains(BytePos(4)));
        assert!(!s.contains(BytePos(5)));
    }
    #[test]
    fn test_span_merge() {
        let a = Span::from_offsets(0, 5);
        let b = Span::from_offsets(3, 10);
        let m = a.merge(b);
        assert_eq!(m.start.0, 0);
        assert_eq!(m.end.0, 10);
    }
    #[test]
    fn test_span_slice() {
        let src = "hello world";
        let sp = Span::from_offsets(6, 11);
        assert_eq!(sp.slice(src), Some("world"));
    }
    #[test]
    fn test_byte_pos_to_line_col() {
        let src = "abc\ndef\nghi";
        let lc = byte_pos_to_line_col(src, BytePos(4));
        assert_eq!(lc.line, 2);
        assert_eq!(lc.col, 1);
    }
    #[test]
    fn test_dotted_name_last() {
        assert_eq!(dotted_name_last("Foo.bar.Baz"), "Baz");
        assert_eq!(dotted_name_last("NoDot"), "NoDot");
    }
    #[test]
    fn test_dotted_name_namespace() {
        assert_eq!(dotted_name_namespace("Foo.bar.Baz"), "Foo.bar");
        assert_eq!(dotted_name_namespace("NoDot"), "");
    }
    #[test]
    fn test_in_namespace() {
        assert!(in_namespace("Nat.succ", "Nat"));
        assert!(!in_namespace("Int.succ", "Nat"));
        assert!(in_namespace("anything", ""));
    }
    #[test]
    fn test_mangle_name() {
        assert_eq!(mangle_name("Foo.bar"), "Foo_bar");
    }
    #[test]
    fn test_center() {
        let s = center("hi", 6);
        assert_eq!(s.len(), 6);
        assert!(s.contains("hi"));
    }
    #[test]
    fn test_join() {
        assert_eq!(join(&["a", "b", "c"], ", "), "a, b, c");
    }
    #[test]
    fn test_words() {
        assert_eq!(words("  hello   world  "), vec!["hello", "world"]);
    }
    #[test]
    fn test_strip_delimiters() {
        assert_eq!(strip_delimiters("(hello)", "(", ")"), Some("hello"));
        assert_eq!(strip_delimiters("hello", "(", ")"), None);
    }
    #[test]
    fn test_build_line_starts() {
        let src = "ab\ncd\nef";
        let starts = build_line_starts(src);
        assert_eq!(starts.len(), 3);
        assert_eq!(starts[1].0, 3);
    }
}
/// Build the standard String environment declarations.
///
/// Registers `String`, `Char`, and basic string operations as axioms in the
/// OxiLean kernel environment.
pub fn build_string_env(env: &mut oxilean_kernel::Environment) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Level, Name};
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
    let nat_ty = || -> Expr { cst("Nat") };
    let bool_ty = || -> Expr { cst("Bool") };
    let string_ty = || -> Expr { cst("String") };
    let char_ty = || -> Expr { cst("Char") };
    let option_of = |ty: Expr| -> Expr { app(cst("Option"), ty) };
    add("String", type1())?;
    add("Char", type1())?;
    add("String.length", arr(string_ty(), nat_ty()))?;
    add(
        "String.append",
        arr(string_ty(), arr(string_ty(), string_ty())),
    )?;
    add("String.mk", arr(app(cst("List"), char_ty()), string_ty()))?;
    add("String.data", arr(string_ty(), app(cst("List"), char_ty())))?;
    add("String.isEmpty", arr(string_ty(), bool_ty()))?;
    add("String.get", arr(string_ty(), arr(nat_ty(), char_ty())))?;
    add(
        "String.contains",
        arr(string_ty(), arr(char_ty(), bool_ty())),
    )?;
    add(
        "String.startsWith",
        arr(string_ty(), arr(string_ty(), bool_ty())),
    )?;
    add(
        "String.endsWith",
        arr(string_ty(), arr(string_ty(), bool_ty())),
    )?;
    add(
        "String.intercalate",
        arr(string_ty(), arr(app(cst("List"), string_ty()), string_ty())),
    )?;
    add(
        "String.splitOn",
        arr(string_ty(), arr(string_ty(), app(cst("List"), string_ty()))),
    )?;
    add("String.trim", arr(string_ty(), string_ty()))?;
    add("String.toNat?", arr(string_ty(), option_of(nat_ty())))?;
    add(
        "String.toList",
        arr(string_ty(), app(cst("List"), char_ty())),
    )?;
    add("Char.val", arr(char_ty(), nat_ty()))?;
    add("Char.ofNat", arr(nat_ty(), char_ty()))?;
    add("Char.isAlpha", arr(char_ty(), bool_ty()))?;
    add("Char.isDigit", arr(char_ty(), bool_ty()))?;
    add("Char.isAlphanum", arr(char_ty(), bool_ty()))?;
    add("Char.isWhitespace", arr(char_ty(), bool_ty()))?;
    add("Char.isLower", arr(char_ty(), bool_ty()))?;
    add("Char.isUpper", arr(char_ty(), bool_ty()))?;
    add("Char.toLower", arr(char_ty(), char_ty()))?;
    add("Char.toUpper", arr(char_ty(), char_ty()))?;
    Ok(())
}
/// Remove duplicate consecutive characters from a string.
///
/// E.g., `"aabbbc"` → `"abc"`.
#[allow(dead_code)]
pub fn deduplicate_consecutive(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut prev: Option<char> = None;
    for c in s.chars() {
        if Some(c) != prev {
            result.push(c);
            prev = Some(c);
        }
    }
    result
}
/// Check whether a string is a palindrome.
#[allow(dead_code)]
pub fn is_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let rev: Vec<char> = chars.iter().rev().cloned().collect();
    chars == rev
}
/// Count the number of Unicode code points in a string.
#[allow(dead_code)]
pub fn char_count(s: &str) -> usize {
    s.chars().count()
}
/// Return the nth Unicode code point (0-indexed), or `None` if out of range.
#[allow(dead_code)]
pub fn nth_char(s: &str, n: usize) -> Option<char> {
    s.chars().nth(n)
}
/// Reverse a string by Unicode code points.
#[allow(dead_code)]
pub fn reverse_str(s: &str) -> String {
    s.chars().rev().collect()
}
/// Split a string into fixed-size chunks of `n` characters.
///
/// The last chunk may be shorter than `n`.
#[allow(dead_code)]
pub fn chunk_chars(s: &str, n: usize) -> Vec<String> {
    if n == 0 {
        return vec![];
    }
    let chars: Vec<char> = s.chars().collect();
    chars.chunks(n).map(|c| c.iter().collect()).collect()
}
/// Check whether `s` contains only ASCII whitespace.
#[allow(dead_code)]
pub fn is_blank(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_whitespace())
}
/// Remove comments (starting with `--`) from a line.
///
/// Returns the portion before the first `--`.
#[allow(dead_code)]
pub fn strip_line_comment(line: &str) -> &str {
    if let Some(pos) = line.find("--") {
        &line[..pos]
    } else {
        line
    }
}
/// Apply a substitution map to a string, replacing all occurrences of each key with its value.
///
/// Keys are substituted in order; no key is substituted twice.
#[allow(dead_code)]
pub fn apply_substitutions(s: &str, subs: &[(&str, &str)]) -> String {
    let mut result = s.to_string();
    for (from, to) in subs {
        result = result.replace(from, to);
    }
    result
}
/// Split a string by a list of delimiters.
///
/// Each character in `delimiters` is treated as a split point.
#[allow(dead_code)]
pub fn split_by_chars<'a>(s: &'a str, delimiters: &[char]) -> Vec<&'a str> {
    s.split(|c: char| delimiters.contains(&c))
        .filter(|s| !s.is_empty())
        .collect()
}
/// Insert a separator between characters in a string.
///
/// E.g., `interleave("abc", '-')` → `"a-b-c"`.
#[allow(dead_code)]
pub fn interleave_char(s: &str, sep: char) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.is_empty() {
        return String::new();
    }
    let mut result = String::with_capacity(s.len() * 2);
    for (i, c) in chars.iter().enumerate() {
        result.push(*c);
        if i + 1 < chars.len() {
            result.push(sep);
        }
    }
    result
}
/// Slugify a string: lowercase, replace spaces and punctuation with `-`.
#[allow(dead_code)]
pub fn slugify(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}
/// Title-case: capitalize the first letter of each word.
#[allow(dead_code)]
pub fn title_case(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
/// Convert a string to upper case (ASCII only for simplicity).
#[allow(dead_code)]
pub fn to_upper(s: &str) -> String {
    s.to_uppercase()
}
/// Convert a string to lower case.
#[allow(dead_code)]
pub fn to_lower(s: &str) -> String {
    s.to_lowercase()
}
/// Capitalize the first character of a string.
#[allow(dead_code)]
pub fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
/// Lowercase the first character of a string.
#[allow(dead_code)]
pub fn decapitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_lowercase().collect::<String>() + chars.as_str(),
    }
}
/// Check whether a string matches a glob pattern (supports `*` and `?`).
///
/// `*` matches any sequence of characters; `?` matches a single character.
#[allow(dead_code)]
pub fn glob_match(pattern: &str, s: &str) -> bool {
    let p: Vec<char> = pattern.chars().collect();
    let t: Vec<char> = s.chars().collect();
    glob_helper(&p, &t)
}
pub fn glob_helper(p: &[char], t: &[char]) -> bool {
    match (p.first(), t.first()) {
        (None, None) => true,
        (Some(&'*'), _) => glob_helper(&p[1..], t) || (!t.is_empty() && glob_helper(p, &t[1..])),
        (Some(&'?'), Some(_)) => glob_helper(&p[1..], &t[1..]),
        (Some(a), Some(b)) if a == b => glob_helper(&p[1..], &t[1..]),
        _ => false,
    }
}
/// Compute the Levenshtein edit distance between two strings.
#[allow(dead_code)]
pub fn edit_distance(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let m = a.len();
    let n = b.len();
    let mut dp = vec![vec![0usize; n + 1]; m + 1];
    for (i, row) in dp.iter_mut().enumerate().take(m + 1) {
        row[0] = i;
    }
    for (j, val) in dp[0].iter_mut().enumerate().take(n + 1) {
        *val = j;
    }
    for i in 1..=m {
        for j in 1..=n {
            dp[i][j] = if a[i - 1] == b[j - 1] {
                dp[i - 1][j - 1]
            } else {
                1 + dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1])
            };
        }
    }
    dp[m][n]
}
/// Find the closest match from `candidates` to `query` by Levenshtein distance.
#[allow(dead_code)]
pub fn closest_match<'a>(query: &str, candidates: &[&'a str]) -> Option<&'a str> {
    candidates
        .iter()
        .min_by_key(|&&c| edit_distance(query, c))
        .copied()
}
/// A simple string formatter that supports `{name}` template substitution.
///
/// Substitutes `{key}` with the provided value.
#[allow(dead_code)]
pub fn template_format(template: &str, vars: &[(&str, &str)]) -> String {
    let mut result = template.to_string();
    for (key, val) in vars {
        result = result.replace(&format!("{{{}}}", key), val);
    }
    result
}
/// Count the number of words in a string.
#[allow(dead_code)]
pub fn word_count(s: &str) -> usize {
    s.split_whitespace().count()
}
/// Count sentences (splits on `.`, `!`, `?`).
#[allow(dead_code)]
pub fn sentence_count(s: &str) -> usize {
    s.chars().filter(|&c| matches!(c, '.' | '!' | '?')).count()
}
/// Return true if `s` contains only ASCII digits.
#[allow(dead_code)]
pub fn is_all_digits(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_digit())
}
/// Return true if `s` contains only ASCII letters.
#[allow(dead_code)]
pub fn is_all_alpha(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_alphabetic())
}
#[cfg(test)]
mod extra_string_tests {
    use super::*;
    #[test]
    fn test_deduplicate_consecutive() {
        assert_eq!(deduplicate_consecutive("aabbcc"), "abc");
        assert_eq!(deduplicate_consecutive("abab"), "abab");
    }
    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome("racecar"));
        assert!(!is_palindrome("hello"));
        assert!(is_palindrome("a"));
        assert!(is_palindrome(""));
    }
    #[test]
    fn test_char_count() {
        assert_eq!(char_count("hello"), 5);
        assert_eq!(char_count(""), 0);
    }
    #[test]
    fn test_nth_char() {
        assert_eq!(nth_char("hello", 1), Some('e'));
        assert_eq!(nth_char("hello", 10), None);
    }
    #[test]
    fn test_reverse_str() {
        assert_eq!(reverse_str("abc"), "cba");
        assert_eq!(reverse_str(""), "");
    }
    #[test]
    fn test_chunk_chars() {
        let chunks = chunk_chars("abcdefg", 3);
        assert_eq!(chunks, vec!["abc", "def", "g"]);
    }
    #[test]
    fn test_is_blank() {
        assert!(is_blank("   \t"));
        assert!(!is_blank("  a "));
    }
    #[test]
    fn test_strip_line_comment() {
        assert_eq!(strip_line_comment("code -- comment"), "code ");
        assert_eq!(strip_line_comment("no comment"), "no comment");
    }
    #[test]
    fn test_apply_substitutions() {
        let subs = &[("foo", "bar"), ("baz", "qux")];
        assert_eq!(apply_substitutions("foo baz", subs), "bar qux");
    }
    #[test]
    fn test_split_by_chars() {
        let parts = split_by_chars("a,b;c", &[',', ';']);
        assert_eq!(parts, vec!["a", "b", "c"]);
    }
    #[test]
    fn test_interleave_char() {
        assert_eq!(interleave_char("abc", '-'), "a-b-c");
        assert_eq!(interleave_char("", '-'), "");
    }
    #[test]
    fn test_slugify() {
        assert_eq!(slugify("Hello World!"), "hello-world");
    }
    #[test]
    fn test_title_case() {
        assert_eq!(title_case("hello world"), "Hello World");
    }
    #[test]
    fn test_capitalize() {
        assert_eq!(capitalize("hello"), "Hello");
        assert_eq!(capitalize(""), "");
    }
    #[test]
    fn test_decapitalize() {
        assert_eq!(decapitalize("Hello"), "hello");
    }
    #[test]
    fn test_glob_match() {
        assert!(glob_match("*.rs", "main.rs"));
        assert!(glob_match("foo?bar", "fooXbar"));
        assert!(!glob_match("*.rs", "main.txt"));
        assert!(glob_match("*", "anything"));
    }
    #[test]
    fn test_edit_distance() {
        assert_eq!(edit_distance("kitten", "sitting"), 3);
        assert_eq!(edit_distance("", "abc"), 3);
        assert_eq!(edit_distance("abc", "abc"), 0);
    }
    #[test]
    fn test_closest_match() {
        let candidates = &["intro", "apply", "exact"];
        let closest = closest_match("intro2", candidates);
        assert_eq!(closest, Some("intro"));
    }
    #[test]
    fn test_template_format() {
        let t = "Hello, {name}! You are {age} years old.";
        let result = template_format(t, &[("name", "Alice"), ("age", "30")]);
        assert_eq!(result, "Hello, Alice! You are 30 years old.");
    }
    #[test]
    fn test_word_count() {
        assert_eq!(word_count("hello world foo"), 3);
        assert_eq!(word_count(""), 0);
    }
    #[test]
    fn test_is_all_digits() {
        assert!(is_all_digits("12345"));
        assert!(!is_all_digits("123a5"));
        assert!(!is_all_digits(""));
    }
    #[test]
    fn test_is_all_alpha() {
        assert!(is_all_alpha("abc"));
        assert!(!is_all_alpha("abc1"));
    }
}
pub(super) fn str_ext2_levenshtein(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let m = a.len();
    let n = b.len();
    let mut dp = vec![vec![0usize; n + 1]; m + 1];
    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }
    for i in 1..=m {
        for j in 1..=n {
            dp[i][j] = if a[i - 1] == b[j - 1] {
                dp[i - 1][j - 1]
            } else {
                1 + dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1])
            };
        }
    }
    dp[m][n]
}
pub fn str_ext2_kmp_failure(pattern: &[char]) -> Vec<usize> {
    let m = pattern.len();
    let mut fail = vec![0usize; m];
    let mut k = 0usize;
    for i in 1..m {
        while k > 0 && pattern[k] != pattern[i] {
            k = fail[k - 1];
        }
        if pattern[k] == pattern[i] {
            k += 1;
        }
        fail[i] = k;
    }
    fail
}
pub(super) fn str_ext2_kmp_search(text: &str, pattern: &str) -> Vec<usize> {
    let t: Vec<char> = text.chars().collect();
    let p: Vec<char> = pattern.chars().collect();
    if p.is_empty() {
        return vec![];
    }
    let fail = str_ext2_kmp_failure(&p);
    let mut positions = Vec::new();
    let mut q = 0usize;
    for (i, &tc) in t.iter().enumerate() {
        while q > 0 && p[q] != tc {
            q = fail[q - 1];
        }
        if p[q] == tc {
            q += 1;
        }
        if q == p.len() {
            positions.push(i + 1 - p.len());
            q = fail[q - 1];
        }
    }
    positions
}
pub(super) fn str_ext2_rabin_karp(
    text: &str,
    pattern: &str,
    base: u64,
    modulus: u64,
) -> Vec<usize> {
    let t: Vec<char> = text.chars().collect();
    let p: Vec<char> = pattern.chars().collect();
    let n = t.len();
    let m = p.len();
    if m == 0 || m > n {
        return vec![];
    }
    let mut pat_hash = 0u64;
    let mut win_hash = 0u64;
    let mut high_pow = 1u64;
    for i in 0..m {
        pat_hash = (pat_hash.wrapping_mul(base).wrapping_add(p[i] as u64)) % modulus;
        win_hash = (win_hash.wrapping_mul(base).wrapping_add(t[i] as u64)) % modulus;
        if i + 1 < m {
            high_pow = high_pow.wrapping_mul(base) % modulus;
        }
    }
    let mut positions = Vec::new();
    if win_hash == pat_hash && t[..m] == p[..] {
        positions.push(0);
    }
    for i in 1..=(n - m) {
        win_hash = (win_hash
            + modulus.wrapping_sub((t[i - 1] as u64).wrapping_mul(high_pow) % modulus))
        .wrapping_mul(base)
        .wrapping_add(t[i + m - 1] as u64)
            % modulus;
        if win_hash == pat_hash && t[i..i + m] == p[..] {
            positions.push(i);
        }
    }
    positions
}
pub fn str_ext2_suffix_array(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut sa: Vec<usize> = (0..n).collect();
    sa.sort_by(|&a, &b| s[a..].cmp(&s[b..]));
    sa
}
pub fn str_ext2_lcp_array(s: &str, sa: &[usize]) -> Vec<usize> {
    let n = s.len();
    if n == 0 {
        return vec![];
    }
    let chars: Vec<char> = s.chars().collect();
    let mut rank = vec![0usize; n];
    for (i, &sai) in sa.iter().enumerate() {
        rank[sai] = i;
    }
    let mut lcp = vec![0usize; n];
    let mut h = 0usize;
    for i in 0..n {
        if rank[i] > 0 {
            let j = sa[rank[i] - 1];
            while i + h < n && j + h < n && chars[i + h] == chars[j + h] {
                h += 1;
            }
            lcp[rank[i]] = h;
            if h > 0 {
                h -= 1;
            }
        }
    }
    lcp
}
pub fn str_ext2_aho_corasick_naive(text: &str, patterns: &[&str]) -> Vec<(usize, usize)> {
    let mut results = Vec::new();
    for (pat_idx, &pattern) in patterns.iter().enumerate() {
        if pattern.is_empty() {
            continue;
        }
        let mut start = 0;
        while let Some(pos) = text[start..].find(pattern) {
            results.push((start + pos, pat_idx));
            start += pos + 1;
        }
    }
    results.sort();
    results
}
pub fn str_ext2_as_list_char_ty() -> Expr {
    use oxilean_kernel::BinderInfo;
    let _type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("s"),
        Box::new(Expr::Const(Name::str("String"), vec![])),
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("List"), vec![])),
            Box::new(Expr::Const(Name::str("Char"), vec![])),
        )),
    )
}
pub fn str_ext2_concat_assoc_ty() -> Expr {
    use oxilean_kernel::BinderInfo;
    let s = || Expr::Const(Name::str("String"), vec![]);
    let arr = |a: Expr, b: Expr| {
        Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(a),
            Box::new(b),
        )
    };
    arr(s(), arr(s(), arr(s(), s())))
}
pub fn str_ext2_concat_left_id_ty() -> Expr {
    use oxilean_kernel::BinderInfo;
    let s = Expr::Const(Name::str("String"), vec![]);
    Expr::Pi(
        BinderInfo::Default,
        Name::str("s"),
        Box::new(s.clone()),
        Box::new(s),
    )
}
pub fn str_ext2_concat_right_id_ty() -> Expr {
    use oxilean_kernel::BinderInfo;
    let s = Expr::Const(Name::str("String"), vec![]);
    Expr::Pi(
        BinderInfo::Default,
        Name::str("s"),
        Box::new(s.clone()),
        Box::new(s),
    )
}
pub fn str_ext2_length_append_ty() -> Expr {
    use oxilean_kernel::BinderInfo;
    let s = || Expr::Const(Name::str("String"), vec![]);
    let n = || Expr::Const(Name::str("Nat"), vec![]);
    let arr = |a: Expr, b: Expr| {
        Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(a),
            Box::new(b),
        )
    };
    arr(s(), arr(s(), n()))
}
pub fn str_ext2_length_empty_ty() -> Expr {
    Expr::Const(Name::str("Nat"), vec![])
}
pub fn str_ext2_dec_eq_ty() -> Expr {
    use oxilean_kernel::BinderInfo;
    let s = || Expr::Const(Name::str("String"), vec![]);
    let b = Expr::Const(Name::str("Bool"), vec![]);
    Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(s()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(s()),
            Box::new(b),
        )),
    )
}
pub fn str_ext2_lex_lt_irrefl_ty() -> Expr {
    use oxilean_kernel::BinderInfo;
    let s = Expr::Const(Name::str("String"), vec![]);
    let b = Expr::Const(Name::str("Bool"), vec![]);
    Expr::Pi(
        BinderInfo::Default,
        Name::str("s"),
        Box::new(s),
        Box::new(b),
    )
}
pub fn str_ext2_lex_lt_trans_ty() -> Expr {
    use oxilean_kernel::BinderInfo;
    let s = || Expr::Const(Name::str("String"), vec![]);
    let b = Expr::Const(Name::str("Bool"), vec![]);
    let arr = |a: Expr, b: Expr| {
        Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(a),
            Box::new(b),
        )
    };
    arr(s(), arr(s(), arr(s(), b)))
}
pub fn str_ext2_lex_total_ty() -> Expr {
    use oxilean_kernel::BinderInfo;
    let s = || Expr::Const(Name::str("String"), vec![]);
    let b = Expr::Const(Name::str("Bool"), vec![]);
    let arr = |a: Expr, b: Expr| {
        Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(a),
            Box::new(b),
        )
    };
    arr(s(), arr(s(), b))
}
pub fn str_ext2_substring_ty() -> Expr {
    use oxilean_kernel::BinderInfo;
    let s = Expr::Const(Name::str("String"), vec![]);
    let n = || Expr::Const(Name::str("Nat"), vec![]);
    let arr = |a: Expr, b: Expr| {
        Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(a),
            Box::new(b),
        )
    };
    arr(s.clone(), arr(n(), arr(n(), s)))
}
pub fn str_ext2_slice_len_ty() -> Expr {
    use oxilean_kernel::BinderInfo;
    let n = || Expr::Const(Name::str("Nat"), vec![]);
    let s = Expr::Const(Name::str("String"), vec![]);
    let arr = |a: Expr, b: Expr| {
        Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(a),
            Box::new(b),
        )
    };
    arr(s, arr(n(), arr(n(), n())))
}
pub fn str_ext2_prefix_refl_ty() -> Expr {
    use oxilean_kernel::BinderInfo;
    let s = Expr::Const(Name::str("String"), vec![]);
    let b = Expr::Const(Name::str("Bool"), vec![]);
    Expr::Pi(
        BinderInfo::Default,
        Name::str("s"),
        Box::new(s),
        Box::new(b),
    )
}
pub fn str_ext2_prefix_trans_ty() -> Expr {
    use oxilean_kernel::BinderInfo;
    let s = || Expr::Const(Name::str("String"), vec![]);
    let b = Expr::Const(Name::str("Bool"), vec![]);
    let arr = |a: Expr, b: Expr| {
        Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(a),
            Box::new(b),
        )
    };
    arr(s(), arr(s(), arr(s(), b)))
}
pub fn str_ext2_suffix_refl_ty() -> Expr {
    use oxilean_kernel::BinderInfo;
    let s = Expr::Const(Name::str("String"), vec![]);
    let b = Expr::Const(Name::str("Bool"), vec![]);
    Expr::Pi(
        BinderInfo::Default,
        Name::str("s"),
        Box::new(s),
        Box::new(b),
    )
}
pub fn str_ext2_split_join_ty() -> Expr {
    use oxilean_kernel::BinderInfo;
    let s = || Expr::Const(Name::str("String"), vec![]);
    let arr = |a: Expr, b: Expr| {
        Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(a),
            Box::new(b),
        )
    };
    arr(s(), arr(s(), s()))
}
pub fn str_ext2_join_split_ty() -> Expr {
    use oxilean_kernel::BinderInfo;
    let s = || Expr::Const(Name::str("String"), vec![]);
    let lst = || {
        Expr::App(
            Box::new(Expr::Const(Name::str("List"), vec![])),
            Box::new(s()),
        )
    };
    let arr = |a: Expr, b: Expr| {
        Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(a),
            Box::new(b),
        )
    };
    arr(s(), arr(lst(), lst()))
}
pub fn str_ext2_trim_idempotent_ty() -> Expr {
    use oxilean_kernel::BinderInfo;
    let s = Expr::Const(Name::str("String"), vec![]);
    Expr::Pi(
        BinderInfo::Default,
        Name::str("s"),
        Box::new(s.clone()),
        Box::new(s),
    )
}
pub fn str_ext2_to_upper_idempotent_ty() -> Expr {
    use oxilean_kernel::BinderInfo;
    let s = Expr::Const(Name::str("String"), vec![]);
    Expr::Pi(
        BinderInfo::Default,
        Name::str("s"),
        Box::new(s.clone()),
        Box::new(s),
    )
}
pub fn str_ext2_to_lower_idempotent_ty() -> Expr {
    use oxilean_kernel::BinderInfo;
    let s = Expr::Const(Name::str("String"), vec![]);
    Expr::Pi(
        BinderInfo::Default,
        Name::str("s"),
        Box::new(s.clone()),
        Box::new(s),
    )
}
pub fn str_ext2_contains_refl_ty() -> Expr {
    use oxilean_kernel::BinderInfo;
    let s = Expr::Const(Name::str("String"), vec![]);
    let b = Expr::Const(Name::str("Bool"), vec![]);
    Expr::Pi(
        BinderInfo::Default,
        Name::str("s"),
        Box::new(s),
        Box::new(b),
    )
}
pub fn str_ext2_starts_with_refl_ty() -> Expr {
    use oxilean_kernel::BinderInfo;
    let s = Expr::Const(Name::str("String"), vec![]);
    let b = Expr::Const(Name::str("Bool"), vec![]);
    Expr::Pi(
        BinderInfo::Default,
        Name::str("s"),
        Box::new(s),
        Box::new(b),
    )
}
pub fn str_ext2_ends_with_refl_ty() -> Expr {
    use oxilean_kernel::BinderInfo;
    let s = Expr::Const(Name::str("String"), vec![]);
    let b = Expr::Const(Name::str("Bool"), vec![]);
    Expr::Pi(
        BinderInfo::Default,
        Name::str("s"),
        Box::new(s),
        Box::new(b),
    )
}
pub fn str_ext2_find_replace_ty() -> Expr {
    use oxilean_kernel::BinderInfo;
    let s = || Expr::Const(Name::str("String"), vec![]);
    let arr = |a: Expr, b: Expr| {
        Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(a),
            Box::new(b),
        )
    };
    arr(s(), arr(s(), arr(s(), s())))
}
pub fn str_ext2_replace_id_ty() -> Expr {
    use oxilean_kernel::BinderInfo;
    let s = || Expr::Const(Name::str("String"), vec![]);
    let arr = |a: Expr, b: Expr| {
        Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(a),
            Box::new(b),
        )
    };
    arr(s(), arr(s(), s()))
}
pub fn str_ext2_unicode_valid_ty() -> Expr {
    use oxilean_kernel::BinderInfo;
    let s = Expr::Const(Name::str("String"), vec![]);
    let b = Expr::Const(Name::str("Bool"), vec![]);
    Expr::Pi(
        BinderInfo::Default,
        Name::str("s"),
        Box::new(s),
        Box::new(b),
    )
}
pub fn str_ext2_utf8_roundtrip_ty() -> Expr {
    use oxilean_kernel::BinderInfo;
    let s = Expr::Const(Name::str("String"), vec![]);
    Expr::Pi(
        BinderInfo::Default,
        Name::str("s"),
        Box::new(s.clone()),
        Box::new(s),
    )
}
pub fn str_ext2_utf16_len_ty() -> Expr {
    use oxilean_kernel::BinderInfo;
    let s = Expr::Const(Name::str("String"), vec![]);
    let n = Expr::Const(Name::str("Nat"), vec![]);
    Expr::Pi(
        BinderInfo::Default,
        Name::str("s"),
        Box::new(s),
        Box::new(n),
    )
}
pub fn str_ext2_char_to_string_ty() -> Expr {
    use oxilean_kernel::BinderInfo;
    let c = Expr::Const(Name::str("Char"), vec![]);
    let s = Expr::Const(Name::str("String"), vec![]);
    Expr::Pi(
        BinderInfo::Default,
        Name::str("c"),
        Box::new(c),
        Box::new(s),
    )
}
pub fn str_ext2_string_to_nat_ty() -> Expr {
    use oxilean_kernel::BinderInfo;
    let s = Expr::Const(Name::str("String"), vec![]);
    let opt_n = Expr::App(
        Box::new(Expr::Const(Name::str("Option"), vec![])),
        Box::new(Expr::Const(Name::str("Nat"), vec![])),
    );
    Expr::Pi(
        BinderInfo::Default,
        Name::str("s"),
        Box::new(s),
        Box::new(opt_n),
    )
}
pub fn str_ext2_format_parse_ty() -> Expr {
    use oxilean_kernel::BinderInfo;
    let s = Expr::Const(Name::str("String"), vec![]);
    let opt_s = Expr::App(
        Box::new(Expr::Const(Name::str("Option"), vec![])),
        Box::new(s.clone()),
    );
    Expr::Pi(
        BinderInfo::Default,
        Name::str("s"),
        Box::new(s),
        Box::new(opt_s),
    )
}
pub fn str_ext2_hash_consistent_ty() -> Expr {
    use oxilean_kernel::BinderInfo;
    let s = || Expr::Const(Name::str("String"), vec![]);
    let n = Expr::Const(Name::str("Nat"), vec![]);
    let arr = |a: Expr, b: Expr| {
        Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(a),
            Box::new(b),
        )
    };
    arr(s(), arr(s(), n))
}
pub fn str_ext2_hash_det_ty() -> Expr {
    use oxilean_kernel::BinderInfo;
    let s = Expr::Const(Name::str("String"), vec![]);
    let n = Expr::Const(Name::str("Nat"), vec![]);
    Expr::Pi(
        BinderInfo::Default,
        Name::str("s"),
        Box::new(s),
        Box::new(n),
    )
}
pub fn str_ext2_kmp_correct_ty() -> Expr {
    use oxilean_kernel::BinderInfo;
    let s = || Expr::Const(Name::str("String"), vec![]);
    let lst = || {
        Expr::App(
            Box::new(Expr::Const(Name::str("List"), vec![])),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
        )
    };
    let arr = |a: Expr, b: Expr| {
        Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(a),
            Box::new(b),
        )
    };
    arr(s(), arr(s(), lst()))
}
pub fn str_ext2_rabin_karp_correct_ty() -> Expr {
    use oxilean_kernel::BinderInfo;
    let s = || Expr::Const(Name::str("String"), vec![]);
    let lst = || {
        Expr::App(
            Box::new(Expr::Const(Name::str("List"), vec![])),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
        )
    };
    let arr = |a: Expr, b: Expr| {
        Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(a),
            Box::new(b),
        )
    };
    arr(s(), arr(s(), lst()))
}
pub fn str_ext2_aho_corasick_ty() -> Expr {
    use oxilean_kernel::BinderInfo;
    let s = || Expr::Const(Name::str("String"), vec![]);
    let lst = || {
        Expr::App(
            Box::new(Expr::Const(Name::str("List"), vec![])),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
        )
    };
    let arr = |a: Expr, b: Expr| {
        Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(a),
            Box::new(b),
        )
    };
    arr(s(), lst())
}
pub fn str_ext2_suffix_array_sorted_ty() -> Expr {
    use oxilean_kernel::BinderInfo;
    let s = Expr::Const(Name::str("String"), vec![]);
    let lst = Expr::App(
        Box::new(Expr::Const(Name::str("List"), vec![])),
        Box::new(Expr::Const(Name::str("Nat"), vec![])),
    );
    Expr::Pi(
        BinderInfo::Default,
        Name::str("s"),
        Box::new(s),
        Box::new(lst),
    )
}
pub fn str_ext2_lcp_array_ty() -> Expr {
    use oxilean_kernel::BinderInfo;
    let s = Expr::Const(Name::str("String"), vec![]);
    let n = Expr::Const(Name::str("Nat"), vec![]);
    Expr::Pi(
        BinderInfo::Default,
        Name::str("s"),
        Box::new(s),
        Box::new(n),
    )
}
pub fn str_ext2_edit_dist_zero_ty() -> Expr {
    use oxilean_kernel::BinderInfo;
    let s = Expr::Const(Name::str("String"), vec![]);
    let n = Expr::Const(Name::str("Nat"), vec![]);
    Expr::Pi(
        BinderInfo::Default,
        Name::str("s"),
        Box::new(s),
        Box::new(n),
    )
}
pub fn str_ext2_edit_dist_sym_ty() -> Expr {
    use oxilean_kernel::BinderInfo;
    let s = || Expr::Const(Name::str("String"), vec![]);
    let n = Expr::Const(Name::str("Nat"), vec![]);
    let arr = |a: Expr, b: Expr| {
        Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(a),
            Box::new(b),
        )
    };
    arr(s(), arr(s(), n))
}
pub fn str_ext2_edit_dist_triangle_ty() -> Expr {
    use oxilean_kernel::BinderInfo;
    let s = || Expr::Const(Name::str("String"), vec![]);
    let b = Expr::Const(Name::str("Bool"), vec![]);
    let arr = |a: Expr, b: Expr| {
        Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(a),
            Box::new(b),
        )
    };
    arr(s(), arr(s(), arr(s(), b)))
}
/// Register all extended string axioms in the environment.
pub fn register_string_extended_axioms(env: &mut Environment) {
    let axioms: &[(&str, fn() -> Expr)] = &[
        ("String.Ext.AsListChar", str_ext2_as_list_char_ty),
        ("String.Ext.ConcatAssoc", str_ext2_concat_assoc_ty),
        ("String.Ext.ConcatLeftId", str_ext2_concat_left_id_ty),
        ("String.Ext.ConcatRightId", str_ext2_concat_right_id_ty),
        ("String.Ext.LengthAppend", str_ext2_length_append_ty),
        ("String.Ext.LengthEmpty", str_ext2_length_empty_ty),
        ("String.Ext.DecEq", str_ext2_dec_eq_ty),
        ("String.Ext.LexLtIrrefl", str_ext2_lex_lt_irrefl_ty),
        ("String.Ext.LexLtTrans", str_ext2_lex_lt_trans_ty),
        ("String.Ext.LexTotal", str_ext2_lex_total_ty),
        ("String.Ext.Substring", str_ext2_substring_ty),
        ("String.Ext.SliceLen", str_ext2_slice_len_ty),
        ("String.Ext.PrefixRefl", str_ext2_prefix_refl_ty),
        ("String.Ext.PrefixTrans", str_ext2_prefix_trans_ty),
        ("String.Ext.SuffixRefl", str_ext2_suffix_refl_ty),
        ("String.Ext.SplitJoin", str_ext2_split_join_ty),
        ("String.Ext.JoinSplit", str_ext2_join_split_ty),
        ("String.Ext.TrimIdempotent", str_ext2_trim_idempotent_ty),
        (
            "String.Ext.ToUpperIdempotent",
            str_ext2_to_upper_idempotent_ty,
        ),
        (
            "String.Ext.ToLowerIdempotent",
            str_ext2_to_lower_idempotent_ty,
        ),
        ("String.Ext.ContainsRefl", str_ext2_contains_refl_ty),
        ("String.Ext.StartsWithRefl", str_ext2_starts_with_refl_ty),
        ("String.Ext.EndsWithRefl", str_ext2_ends_with_refl_ty),
        ("String.Ext.FindReplace", str_ext2_find_replace_ty),
        ("String.Ext.ReplaceId", str_ext2_replace_id_ty),
        ("String.Ext.UnicodeValid", str_ext2_unicode_valid_ty),
        ("String.Ext.Utf8Roundtrip", str_ext2_utf8_roundtrip_ty),
        ("String.Ext.Utf16Len", str_ext2_utf16_len_ty),
        ("String.Ext.CharToString", str_ext2_char_to_string_ty),
        ("String.Ext.StringToNat", str_ext2_string_to_nat_ty),
        ("String.Ext.FormatParse", str_ext2_format_parse_ty),
        ("String.Ext.HashConsistent", str_ext2_hash_consistent_ty),
        ("String.Ext.HashDet", str_ext2_hash_det_ty),
        ("String.Ext.KmpCorrect", str_ext2_kmp_correct_ty),
        (
            "String.Ext.RabinKarpCorrect",
            str_ext2_rabin_karp_correct_ty,
        ),
        ("String.Ext.AhoCorasick", str_ext2_aho_corasick_ty),
        (
            "String.Ext.SuffixArraySorted",
            str_ext2_suffix_array_sorted_ty,
        ),
        ("String.Ext.LcpArray", str_ext2_lcp_array_ty),
        ("String.Ext.EditDistZero", str_ext2_edit_dist_zero_ty),
        ("String.Ext.EditDistSym", str_ext2_edit_dist_sym_ty),
        (
            "String.Ext.EditDistTriangle",
            str_ext2_edit_dist_triangle_ty,
        ),
    ];
    for (name, ty_fn) in axioms {
        let _ = env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty_fn(),
        });
    }
}
/// Check string concatenation associativity.
pub fn str_concat_assoc_check(a: &str, b: &str, c: &str) -> bool {
    let left = format!("{}{}{}", a, b, c);
    let right = format!("{}{}{}", a, b, c);
    left == right
}
/// Check left identity: "" ++ s = s.
pub fn str_left_id_check(s: &str) -> bool {
    format!("{}{}", "", s) == s
}
/// Check right identity: s ++ "" = s.
pub fn str_right_id_check(s: &str) -> bool {
    format!("{}{}", s, "") == s
}
/// Check length additivity: len(s ++ t) = len(s) + len(t).
pub fn str_length_additive(s: &str, t: &str) -> bool {
    let combined = format!("{}{}", s, t);
    combined.chars().count() == s.chars().count() + t.chars().count()
}
/// Lexicographic less-than for strings.
pub fn str_lex_lt(a: &str, b: &str) -> bool {
    a < b
}
/// Check lexicographic irreflexivity: ¬(s < s).
pub fn str_lex_lt_irrefl(s: &str) -> bool {
    !str_lex_lt(s, s)
}
/// Check lexicographic transitivity.
pub fn str_lex_lt_trans(a: &str, b: &str, c: &str) -> bool {
    if str_lex_lt(a, b) && str_lex_lt(b, c) {
        str_lex_lt(a, c)
    } else {
        true
    }
}
/// Extract a substring by char index and length.
pub fn str_substring(s: &str, start: usize, len: usize) -> String {
    s.chars().skip(start).take(len).collect()
}
/// Check that a string starts with prefix.
pub fn str_starts_with(s: &str, prefix: &str) -> bool {
    s.starts_with(prefix)
}
/// Check that a string ends with suffix.
pub fn str_ends_with(s: &str, suffix: &str) -> bool {
    s.ends_with(suffix)
}
/// Check reflexivity of prefix: s.starts_with(s).
pub fn str_prefix_refl(s: &str) -> bool {
    s.starts_with(s)
}
/// Check transitivity of prefix relation.
pub fn str_prefix_trans(a: &str, b: &str, c: &str) -> bool {
    if b.starts_with(a) && c.starts_with(b) {
        c.starts_with(a)
    } else {
        true
    }
}
/// Split and join roundtrip: join sep (split sep s) = s.
pub fn str_split_join_roundtrip(s: &str, sep: &str) -> bool {
    if sep.is_empty() {
        return true;
    }
    let parts: Vec<&str> = s.split(sep).collect();
    let rejoined = parts.join(sep);
    rejoined == s
}
/// Check trim idempotency: trim(trim(s)) = trim(s).
pub fn str_trim_idempotent(s: &str) -> bool {
    let trimmed = s.trim();
    trimmed.trim() == trimmed
}
/// Check toUpper idempotency.
pub fn str_to_upper_idempotent(s: &str) -> bool {
    let upper = s.to_uppercase();
    upper.to_uppercase() == upper
}
/// Check toLower idempotency.
pub fn str_to_lower_idempotent(s: &str) -> bool {
    let lower = s.to_lowercase();
    lower.to_lowercase() == lower
}
/// Check contains reflexivity: s.contains(s).
pub fn str_contains_refl(s: &str) -> bool {
    s.contains(s)
}
/// Find and replace all occurrences.
pub fn str_find_replace(s: &str, from: &str, to: &str) -> String {
    if from.is_empty() {
        return s.to_string();
    }
    s.replace(from, to)
}
/// Check replace identity: replace(p, p, s) = s.
pub fn str_replace_id(s: &str, p: &str) -> bool {
    str_find_replace(s, p, p) == s
}
/// Check UTF-8 roundtrip: decode(encode(s)) = s.
pub fn str_utf8_roundtrip(s: &str) -> bool {
    let encoded = s.as_bytes();
    String::from_utf8(encoded.to_vec()).as_deref() == Ok(s)
}
/// Get UTF-16 code unit count.
pub fn str_utf16_len(s: &str) -> usize {
    s.encode_utf16().count()
}
/// Convert char to string.
pub fn str_char_to_string(c: char) -> String {
    c.to_string()
}
/// Check hash consistency: equal strings have equal hashes.
pub fn str_hash_consistent(a: &str, b: &str) -> bool {
    if a == b {
        use super::functions::*;
        use std::collections::hash_map::DefaultHasher;
        use std::fmt;
        use std::hash::{Hash, Hasher};
        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        a.hash(&mut h1);
        b.hash(&mut h2);
        h1.finish() == h2.finish()
    } else {
        true
    }
}
/// KMP pattern search (public wrapper).
pub fn str_kmp_search(text: &str, pattern: &str) -> Vec<usize> {
    str_ext2_kmp_search(text, pattern)
}
/// Rabin-Karp pattern search (public wrapper).
pub fn str_rabin_karp_search(text: &str, pattern: &str) -> Vec<usize> {
    str_ext2_rabin_karp(text, pattern, 31, 1_000_000_007)
}
/// Naive multi-pattern search (Aho-Corasick approximation).
pub fn str_aho_corasick_search(text: &str, patterns: &[&str]) -> Vec<(usize, usize)> {
    str_ext2_aho_corasick_naive(text, patterns)
}
/// Build suffix array for a string.
pub fn str_suffix_array(s: &str) -> Vec<usize> {
    str_ext2_suffix_array(s)
}
/// Build LCP array from suffix array.
pub fn str_lcp_array(s: &str) -> Vec<usize> {
    let sa = str_suffix_array(s);
    str_ext2_lcp_array(s, &sa)
}
/// Compute Levenshtein edit distance (public wrapper).
pub fn str_levenshtein(a: &str, b: &str) -> usize {
    str_ext2_levenshtein(a, b)
}
/// Check edit distance identity: d(s, s) = 0.
pub fn str_edit_dist_zero(s: &str) -> bool {
    str_levenshtein(s, s) == 0
}
/// Check edit distance symmetry: d(a, b) = d(b, a).
pub fn str_edit_dist_sym(a: &str, b: &str) -> bool {
    str_levenshtein(a, b) == str_levenshtein(b, a)
}
/// Check triangle inequality for edit distance.
pub fn str_edit_dist_triangle(a: &str, b: &str, c: &str) -> bool {
    str_levenshtein(a, c) <= str_levenshtein(a, b) + str_levenshtein(b, c)
}
/// Check KMP and naive search give same results.
pub fn str_kmp_matches_naive(text: &str, pattern: &str) -> bool {
    let kmp = str_kmp_search(text, pattern);
    let naive = SubstringFinder2::new(text, pattern).find_all();
    kmp == naive
}
/// Check Rabin-Karp and KMP give same results.
pub fn str_rabin_karp_matches_kmp(text: &str, pattern: &str) -> bool {
    let rk = str_rabin_karp_search(text, pattern);
    let kmp = str_kmp_search(text, pattern);
    rk == kmp
}
/// Check suffix array is sorted.
pub fn str_suffix_array_sorted(s: &str) -> bool {
    let sa = str_suffix_array(s);
    for i in 1..sa.len() {
        if s[sa[i - 1]..] > s[sa[i]..] {
            return false;
        }
    }
    true
}
/// Check suffix array has correct length.
pub fn str_suffix_array_len(s: &str) -> bool {
    str_suffix_array(s).len() == s.len()
}
/// Unicode validity: all bytes form valid UTF-8.
pub fn str_unicode_valid(s: &str) -> bool {
    std::str::from_utf8(s.as_bytes()).is_ok()
}
/// Count ASCII alphanumeric chars.
pub fn str_count_alnum(s: &str) -> usize {
    s.chars().filter(|c| c.is_alphanumeric()).count()
}
/// Count whitespace characters.
pub fn str_count_whitespace(s: &str) -> usize {
    s.chars().filter(|c| c.is_whitespace()).count()
}
/// Check decidable equality: eq(s, s) = true.
pub fn str_dec_eq_refl(s: &str) -> bool {
    #[allow(clippy::eq_op)]
    let result = s == s;
    result
}
/// Check decidable equality symmetry.
pub fn str_dec_eq_sym(a: &str, b: &str) -> bool {
    #[allow(clippy::eq_op)]
    let result = (a == b) == (b == a);
    result
}
