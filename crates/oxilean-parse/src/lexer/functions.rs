//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{LexerState, RawToken, TokenDiff};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::*;
    use crate::tokens::{StringPart, TokenKind};
    #[test]
    fn test_lex_keywords() {
        let mut lexer = Lexer::new("axiom definition theorem");
        let tokens = lexer.tokenize();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].kind, TokenKind::Axiom);
        assert_eq!(tokens[1].kind, TokenKind::Definition);
        assert_eq!(tokens[2].kind, TokenKind::Theorem);
    }
    #[test]
    fn test_lex_ident() {
        let mut lexer = Lexer::new("foo bar_123");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0].kind, TokenKind::Ident("foo".to_string()));
        assert_eq!(tokens[1].kind, TokenKind::Ident("bar_123".to_string()));
    }
    #[test]
    fn test_lex_numbers() {
        let mut lexer = Lexer::new("0 42 1000");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0].kind, TokenKind::Nat(0));
        assert_eq!(tokens[1].kind, TokenKind::Nat(42));
        assert_eq!(tokens[2].kind, TokenKind::Nat(1000));
    }
    #[test]
    fn test_lex_string() {
        let mut lexer = Lexer::new(r#""hello world""#);
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0].kind, TokenKind::String("hello world".to_string()));
    }
    #[test]
    fn test_lex_operators() {
        let mut lexer = Lexer::new("-> := = < >");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0].kind, TokenKind::Arrow);
        assert_eq!(tokens[1].kind, TokenKind::Assign);
        assert_eq!(tokens[2].kind, TokenKind::Eq);
        assert_eq!(tokens[3].kind, TokenKind::Lt);
        assert_eq!(tokens[4].kind, TokenKind::Gt);
    }
    #[test]
    fn test_skip_line_comment() {
        let mut lexer = Lexer::new("foo -- this is a comment\nbar");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0].kind, TokenKind::Ident("foo".to_string()));
        assert_eq!(tokens[1].kind, TokenKind::Ident("bar".to_string()));
    }
    #[test]
    fn test_skip_block_comment() {
        let mut lexer = Lexer::new("foo /- block comment -/ bar");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0].kind, TokenKind::Ident("foo".to_string()));
        assert_eq!(tokens[1].kind, TokenKind::Ident("bar".to_string()));
    }
    #[test]
    fn test_lex_new_keywords() {
        let mut lexer = Lexer::new("if then else match with do have suffices show where");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0].kind, TokenKind::If);
        assert_eq!(tokens[1].kind, TokenKind::Then);
        assert_eq!(tokens[2].kind, TokenKind::Else);
        assert_eq!(tokens[3].kind, TokenKind::Match);
        assert_eq!(tokens[4].kind, TokenKind::With);
        assert_eq!(tokens[5].kind, TokenKind::Do);
        assert_eq!(tokens[6].kind, TokenKind::Have);
        assert_eq!(tokens[7].kind, TokenKind::Suffices);
        assert_eq!(tokens[8].kind, TokenKind::Show);
        assert_eq!(tokens[9].kind, TokenKind::Where);
    }
    #[test]
    fn test_lex_boolean_ops() {
        let mut lexer = Lexer::new("&& || != !");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0].kind, TokenKind::AndAnd);
        assert_eq!(tokens[1].kind, TokenKind::OrOr);
        assert_eq!(tokens[2].kind, TokenKind::BangEq);
        assert_eq!(tokens[3].kind, TokenKind::Bang);
    }
    #[test]
    fn test_lex_left_arrow() {
        let mut lexer = Lexer::new("<-");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0].kind, TokenKind::LeftArrow);
    }
    #[test]
    fn test_lex_unicode_identifiers() {
        let mut lexer = Lexer::new("\u{03B1} \u{03B2} \u{03A0} \u{03A3}");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0].kind, TokenKind::Ident("\u{03B1}".to_string()));
        assert_eq!(tokens[1].kind, TokenKind::Ident("\u{03B2}".to_string()));
        assert_eq!(tokens[2].kind, TokenKind::Ident("\u{03A0}".to_string()));
        assert_eq!(tokens[3].kind, TokenKind::Ident("\u{03A3}".to_string()));
    }
    #[test]
    fn test_lex_unicode_mixed_ident() {
        let mut lexer = Lexer::new("\u{03B1}1 x\u{03B2}");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0].kind, TokenKind::Ident("\u{03B1}1".to_string()));
        assert_eq!(tokens[1].kind, TokenKind::Ident("x\u{03B2}".to_string()));
    }
    #[test]
    fn test_lex_hex_number() {
        let mut lexer = Lexer::new("0x1A 0xFF 0x0");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0].kind, TokenKind::Nat(0x1A));
        assert_eq!(tokens[1].kind, TokenKind::Nat(0xFF));
        assert_eq!(tokens[2].kind, TokenKind::Nat(0x0));
    }
    #[test]
    fn test_lex_binary_number() {
        let mut lexer = Lexer::new("0b1010 0b0 0b1111");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0].kind, TokenKind::Nat(0b1010));
        assert_eq!(tokens[1].kind, TokenKind::Nat(0b0));
        assert_eq!(tokens[2].kind, TokenKind::Nat(0b1111));
    }
    #[test]
    fn test_lex_octal_number() {
        let mut lexer = Lexer::new("0o17 0o0 0o777");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0].kind, TokenKind::Nat(0o17));
        assert_eq!(tokens[1].kind, TokenKind::Nat(0o0));
        assert_eq!(tokens[2].kind, TokenKind::Nat(0o777));
    }
    #[test]
    #[allow(clippy::approx_constant)]
    fn test_lex_float_basic() {
        let mut lexer = Lexer::new("3.14 1.0 0.5");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0].kind, TokenKind::Float(3.14));
        assert_eq!(tokens[1].kind, TokenKind::Float(1.0));
        assert_eq!(tokens[2].kind, TokenKind::Float(0.5));
    }
    #[test]
    fn test_lex_float_exponent() {
        let mut lexer = Lexer::new("1.0e10 1.5e-3 2.0E5");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0].kind, TokenKind::Float(1.0e10));
        assert_eq!(tokens[1].kind, TokenKind::Float(1.5e-3));
        assert_eq!(tokens[2].kind, TokenKind::Float(2.0e5));
    }
    #[test]
    fn test_lex_float_int_exponent() {
        let mut lexer = Lexer::new("1e10");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0].kind, TokenKind::Float(1e10));
    }
    #[test]
    fn test_lex_char_simple() {
        let mut lexer = Lexer::new("'a' 'z' '0'");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0].kind, TokenKind::Char('a'));
        assert_eq!(tokens[1].kind, TokenKind::Char('z'));
        assert_eq!(tokens[2].kind, TokenKind::Char('0'));
    }
    #[test]
    fn test_lex_char_escape() {
        let mut lexer = Lexer::new(r"'\n' '\t' '\\'");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0].kind, TokenKind::Char('\n'));
        assert_eq!(tokens[1].kind, TokenKind::Char('\t'));
        assert_eq!(tokens[2].kind, TokenKind::Char('\\'));
    }
    #[test]
    fn test_lex_char_hex_escape() {
        let mut lexer = Lexer::new(r"'\x41'");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0].kind, TokenKind::Char('A'));
    }
    #[test]
    fn test_lex_interpolated_string_simple() {
        let input = r#"s!"hello {name}""#;
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        match &tokens[0].kind {
            TokenKind::InterpolatedString(parts) => {
                assert_eq!(parts.len(), 2);
                match &parts[0] {
                    StringPart::Literal(s) => assert_eq!(s, "hello "),
                    _ => panic!("expected literal part"),
                }
                match &parts[1] {
                    StringPart::Interpolation(toks) => {
                        assert_eq!(toks.len(), 1);
                        assert_eq!(toks[0].kind, TokenKind::Ident("name".to_string()));
                    }
                    _ => panic!("expected interpolation part"),
                }
            }
            _ => panic!("expected InterpolatedString, got {:?}", tokens[0].kind),
        }
    }
    #[test]
    fn test_lex_interpolated_string_multiple_holes() {
        let input = r#"s!"{a} + {b}""#;
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        match &tokens[0].kind {
            TokenKind::InterpolatedString(parts) => {
                assert_eq!(parts.len(), 3);
            }
            _ => panic!("expected InterpolatedString"),
        }
    }
    #[test]
    fn test_lex_line_doc_comment() {
        let mut lexer = Lexer::new("--- This is a doc\nfoo");
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens[0].kind,
            TokenKind::DocComment("This is a doc".to_string())
        );
        assert_eq!(tokens[1].kind, TokenKind::Ident("foo".to_string()));
    }
    #[test]
    fn test_lex_block_doc_comment() {
        let mut lexer = Lexer::new("/-- A block doc comment -/ foo");
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens[0].kind,
            TokenKind::DocComment("A block doc comment".to_string())
        );
        assert_eq!(tokens[1].kind, TokenKind::Ident("foo".to_string()));
    }
    #[test]
    fn test_lex_return_keyword() {
        let mut lexer = Lexer::new("return x");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0].kind, TokenKind::Return);
        assert_eq!(tokens[1].kind, TokenKind::Ident("x".to_string()));
    }
    #[test]
    fn test_lex_ge_operator() {
        let mut lexer = Lexer::new("x >= y");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0].kind, TokenKind::Ident("x".to_string()));
        assert_eq!(tokens[1].kind, TokenKind::Ge);
        assert_eq!(tokens[2].kind, TokenKind::Ident("y".to_string()));
    }
    #[test]
    fn test_lex_dot_dot() {
        let mut lexer = Lexer::new("0..10");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0].kind, TokenKind::Nat(0));
        assert_eq!(tokens[1].kind, TokenKind::DotDot);
        assert_eq!(tokens[2].kind, TokenKind::Nat(10));
    }
    #[test]
    fn test_lex_fat_arrow() {
        let mut lexer = Lexer::new("| x => y");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0].kind, TokenKind::Bar);
        assert_eq!(tokens[1].kind, TokenKind::Ident("x".to_string()));
        assert_eq!(tokens[2].kind, TokenKind::FatArrow);
        assert_eq!(tokens[3].kind, TokenKind::Ident("y".to_string()));
    }
    #[test]
    fn test_lex_string_unicode_escape() {
        let mut lexer = Lexer::new(r#""\u{03B1}""#);
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0].kind, TokenKind::String("\u{03B1}".to_string()));
    }
    #[test]
    fn test_lex_string_hex_escape() {
        let mut lexer = Lexer::new(r#""\x41""#);
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0].kind, TokenKind::String("A".to_string()));
    }
    #[test]
    fn test_lex_string_null_escape() {
        let mut lexer = Lexer::new(r#""\0""#);
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0].kind, TokenKind::String("\0".to_string()));
    }
    #[test]
    fn test_lex_hex_error_no_digits() {
        let mut lexer = Lexer::new("0x ");
        let tokens = lexer.tokenize();
        assert!(matches!(&tokens[0].kind, TokenKind::Error(_)));
    }
    #[test]
    fn test_lex_bin_error_no_digits() {
        let mut lexer = Lexer::new("0b ");
        let tokens = lexer.tokenize();
        assert!(matches!(&tokens[0].kind, TokenKind::Error(_)));
    }
    #[test]
    fn test_lex_oct_error_no_digits() {
        let mut lexer = Lexer::new("0o ");
        let tokens = lexer.tokenize();
        assert!(matches!(&tokens[0].kind, TokenKind::Error(_)));
    }
    #[test]
    fn test_lex_number_with_underscore_separator() {
        let mut lexer = Lexer::new("1_000_000");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0].kind, TokenKind::Nat(1_000_000));
    }
    #[test]
    fn test_lex_hex_with_underscore() {
        let mut lexer = Lexer::new("0xFF_FF");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0].kind, TokenKind::Nat(0xFFFF));
    }
    #[test]
    fn test_lex_unicode_fat_arrow() {
        let mut lexer = Lexer::new("\u{21D2}");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0].kind, TokenKind::FatArrow);
    }
    #[test]
    fn test_lex_char_unicode_escape() {
        let input = r"'\u{03B1}'";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0].kind, TokenKind::Char('\u{03B1}'));
    }
    #[test]
    fn test_lex_combined_program() {
        let input = r#"
            /-- Documentation -/
            def add (x : Nat) (y : Nat) : Nat :=
                return x
        "#;
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        let kinds: Vec<&TokenKind> = tokens.iter().map(|t| &t.kind).collect();
        assert!(kinds.contains(&&TokenKind::DocComment("Documentation".to_string())));
        assert!(kinds.contains(&&TokenKind::Definition));
        assert!(kinds.contains(&&TokenKind::Return));
    }
    #[test]
    fn test_lex_multiple_doc_comments() {
        let input = "/-- First doc -/\n/-- Second doc -/\nfoo";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens[0].kind,
            TokenKind::DocComment("First doc".to_string())
        );
        assert_eq!(
            tokens[1].kind,
            TokenKind::DocComment("Second doc".to_string())
        );
        assert_eq!(tokens[2].kind, TokenKind::Ident("foo".to_string()));
    }
}
/// A character escape decoder.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn decode_escape(c: char) -> char {
    match c {
        'n' => '\n',
        't' => '\t',
        'r' => '\r',
        '0' => '\0',
        '\\' => '\\',
        '\'' => '\'',
        '"' => '"',
        _ => c,
    }
}
/// Decode a string literal body (with escape sequences).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn decode_string_literal(s: &str) -> Result<String, String> {
    let mut result = String::new();
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some(esc) => result.push(decode_escape(esc)),
                None => return Err("trailing backslash".to_string()),
            }
        } else {
            result.push(c);
        }
    }
    Ok(result)
}
/// Checks whether a string is a valid identifier.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn is_valid_lean_ident(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let mut chars = s.chars();
    let first = chars
        .next()
        .expect("string is non-empty per is_empty check above");
    if !first.is_alphabetic() && first != '_' {
        return false;
    }
    chars.all(|c| c.is_alphanumeric() || c == '_' || c == '\'')
}
/// Checks whether a string is a Lean4-style keyword.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn is_lean_keyword(s: &str) -> bool {
    matches!(
        s,
        "def"
            | "theorem"
            | "lemma"
            | "let"
            | "fun"
            | "have"
            | "show"
            | "match"
            | "with"
            | "do"
            | "if"
            | "then"
            | "else"
            | "return"
            | "forall"
            | "exists"
            | "structure"
            | "class"
            | "instance"
            | "namespace"
            | "section"
            | "end"
            | "import"
            | "open"
            | "variable"
            | "axiom"
            | "opaque"
            | "abbrev"
            | "by"
            | "exact"
            | "apply"
            | "intro"
    )
}
/// Checks that a sequence of raw tokens covers the entire source.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn tokens_cover_source(src: &str, tokens: &[RawToken]) -> bool {
    if tokens.is_empty() {
        return src.is_empty();
    }
    let first_start = tokens[0].start;
    let last_end = tokens
        .last()
        .expect("tokens non-empty per is_empty check above")
        .end;
    let mut pos = first_start;
    for tok in tokens {
        if tok.start != pos {
            return false;
        }
        pos = tok.end;
    }
    first_start == 0 && last_end == src.len()
}
#[cfg(test)]
mod lexer_ext_tests {
    use super::*;
    use crate::lexer::*;
    use crate::tokens::{StringPart, TokenKind};
    #[test]
    fn test_char_class_alpha() {
        let cls = CharClass::Alpha;
        assert!(cls.matches('a'));
        assert!(!cls.matches('1'));
    }
    #[test]
    fn test_char_class_digit() {
        let cls = CharClass::Digit;
        assert!(cls.matches('5'));
        assert!(!cls.matches('a'));
    }
    #[test]
    fn test_char_class_one_of() {
        let cls = CharClass::OneOf(vec!['+', '-', '*']);
        assert!(cls.matches('+'));
        assert!(!cls.matches('a'));
    }
    #[test]
    fn test_scanner_basic() {
        let mut sc = Scanner::new("hello world");
        assert_eq!(sc.peek(), Some('h'));
        let _ = sc.next();
        assert_eq!(sc.position(), 1);
    }
    #[test]
    fn test_scanner_skip_whitespace() {
        let mut sc = Scanner::new("  hello");
        sc.skip_whitespace();
        assert_eq!(sc.peek(), Some('h'));
    }
    #[test]
    fn test_scanner_consume_while() {
        let mut sc = Scanner::new("abc123");
        let s = sc.consume_while(|c| c.is_alphabetic());
        assert_eq!(s, "abc");
    }
    #[test]
    fn test_decode_escape() {
        assert_eq!(decode_escape('n'), '\n');
        assert_eq!(decode_escape('t'), '\t');
        assert_eq!(decode_escape('x'), 'x');
    }
    #[test]
    fn test_decode_string_literal() {
        assert_eq!(
            decode_string_literal("hello\\nworld").expect("test operation should succeed"),
            "hello\nworld"
        );
        assert!(decode_string_literal("bad\\").is_err());
    }
    #[test]
    fn test_is_valid_lean_ident() {
        assert!(is_valid_lean_ident("foo"));
        assert!(is_valid_lean_ident("_bar"));
        assert!(!is_valid_lean_ident("123"));
        assert!(!is_valid_lean_ident(""));
    }
    #[test]
    fn test_is_lean_keyword() {
        assert!(is_lean_keyword("def"));
        assert!(is_lean_keyword("theorem"));
        assert!(!is_lean_keyword("myFun"));
    }
    #[test]
    fn test_keyword_trie() {
        let mut trie = KeywordTrie::new();
        trie.insert("def");
        trie.insert("theorem");
        assert!(trie.contains("def"));
        assert!(trie.contains("theorem"));
        assert!(!trie.contains("de"));
        assert!(!trie.contains("defn"));
    }
    #[test]
    fn test_trivia_accumulator() {
        let mut acc = TriviaAccumulator::new();
        acc.push("  ");
        acc.push("\n");
        assert!(acc.has_newlines);
        acc.clear();
        assert!(acc.text.is_empty());
        assert!(!acc.has_newlines);
    }
    #[test]
    fn test_lex_summary_format() {
        let mut s = LexSummary::new();
        s.token_count = 10;
        s.ident_count = 5;
        let out = s.format();
        assert!(out.contains("tokens=10"));
        assert!(out.contains("idents=5"));
    }
}
/// Checks whether a character is an operator start character.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn is_op_start(c: char) -> bool {
    matches!(
        c,
        '+' | '-'
            | '*'
            | '/'
            | '%'
            | '='
            | '<'
            | '>'
            | '!'
            | '&'
            | '|'
            | '^'
            | '~'
            | '@'
            | '#'
            | '$'
            | '.'
            | ':'
            | '?'
            | '\\'
    )
}
/// Checks whether a character can continue an operator.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn is_op_cont(c: char) -> bool {
    is_op_start(c)
}
#[cfg(test)]
mod lexer_ext2_tests {
    use super::*;
    use crate::lexer::*;
    use crate::tokens::{StringPart, TokenKind};
    #[test]
    fn test_token_category_display() {
        assert_eq!(TokenCategory::Ident.to_string(), "ident");
        assert_eq!(TokenCategory::Eof.to_string(), "eof");
    }
    #[test]
    fn test_lexer_mode_stack() {
        let mut stack = LexerModeStack::new();
        assert_eq!(stack.current(), &LexerMode::Normal);
        stack.push(LexerMode::StringLit);
        assert_eq!(stack.current(), &LexerMode::StringLit);
        let _ = stack.pop();
        assert_eq!(stack.current(), &LexerMode::Normal);
        assert_eq!(stack.depth(), 1);
    }
    #[test]
    fn test_is_op_start() {
        assert!(is_op_start('+'));
        assert!(is_op_start('<'));
        assert!(!is_op_start('a'));
        assert!(!is_op_start('0'));
    }
    #[test]
    fn test_line_map() {
        let src = "hello\nworld\nfoo";
        let lm = LineMap::from_str(src);
        assert_eq!(lm.line_count(), 3);
        assert_eq!(lm.line_for_offset(0), 1);
        assert_eq!(lm.line_for_offset(6), 2);
        assert_eq!(lm.col_for_offset(6), 1);
    }
}
/// Compute a simple token-level diff between two source strings.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn compute_token_diff(a: &str, b: &str) -> TokenDiff {
    use std::collections::HashMap;
    let ta: Vec<String> = a.split_whitespace().map(|s| s.to_string()).collect();
    let tb: Vec<String> = b.split_whitespace().map(|s| s.to_string()).collect();
    let mut freq_a: HashMap<&str, usize> = HashMap::new();
    let mut freq_b: HashMap<&str, usize> = HashMap::new();
    for t in &ta {
        *freq_a.entry(t.as_str()).or_insert(0) += 1;
    }
    for t in &tb {
        *freq_b.entry(t.as_str()).or_insert(0) += 1;
    }
    let mut matching = 0usize;
    for (tok, count_a) in &freq_a {
        let count_b = freq_b.get(tok).copied().unwrap_or(0);
        matching += (*count_a).min(count_b);
    }
    let only_left = ta.len().saturating_sub(matching);
    let only_right = tb.len().saturating_sub(matching);
    TokenDiff {
        only_left,
        only_right,
        matching,
    }
}
#[cfg(test)]
mod lexer_ext3_tests {
    use super::*;
    use crate::lexer::*;
    use crate::tokens::{StringPart, TokenKind};
    #[test]
    fn test_filtered_token_stream_empty() {
        let stream = FilteredTokenStream::new(Vec::new());
        assert!(stream.is_eof());
        assert_eq!(stream.remaining(), 0);
    }
    #[test]
    fn test_compute_token_diff_identical() {
        let diff = compute_token_diff("a b c", "a b c");
        assert_eq!(diff.matching, 3);
        assert_eq!(diff.only_left, 0);
        assert_eq!(diff.only_right, 0);
    }
    #[test]
    fn test_compute_token_diff_different() {
        let diff = compute_token_diff("a b c", "a b d");
        assert_eq!(diff.matching, 2);
        assert_eq!(diff.only_left, 1);
        assert_eq!(diff.only_right, 1);
    }
}
/// Run a simple state machine over a character.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn lex_transition(state: &LexerState, c: char) -> LexerState {
    match state {
        LexerState::Start => {
            if c.is_alphabetic() || c == '_' {
                LexerState::Ident
            } else if c.is_ascii_digit() {
                LexerState::Number
            } else if c == '"' {
                LexerState::StringStart
            } else if c == '-' || c == '/' {
                LexerState::LineComment
            } else if is_op_start(c) {
                LexerState::Operator
            } else {
                LexerState::Done
            }
        }
        LexerState::Ident => {
            if c.is_alphanumeric() || c == '_' || c == '\'' {
                LexerState::Ident
            } else {
                LexerState::Done
            }
        }
        LexerState::Number => {
            if c.is_ascii_digit() || c == '_' {
                LexerState::Number
            } else {
                LexerState::Done
            }
        }
        LexerState::Operator => {
            if is_op_cont(c) {
                LexerState::Operator
            } else {
                LexerState::Done
            }
        }
        _ => LexerState::Done,
    }
}
/// A character frequency counter for source analysis.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn char_frequency(src: &str) -> std::collections::HashMap<char, usize> {
    let mut freq = std::collections::HashMap::new();
    for c in src.chars() {
        *freq.entry(c).or_insert(0) += 1;
    }
    freq
}
#[cfg(test)]
mod lexer_state_tests {
    use super::*;
    use crate::lexer::*;
    use crate::tokens::{StringPart, TokenKind};
    #[test]
    fn test_lex_transition() {
        let s = lex_transition(&LexerState::Start, 'f');
        assert_eq!(s, LexerState::Ident);
        let s2 = lex_transition(&LexerState::Start, '5');
        assert_eq!(s2, LexerState::Number);
        let s3 = lex_transition(&LexerState::Start, '+');
        assert_eq!(s3, LexerState::Operator);
    }
    #[test]
    fn test_char_frequency() {
        let freq = char_frequency("hello");
        assert_eq!(freq[&'l'], 2);
        assert_eq!(freq[&'h'], 1);
    }
}
/// A list of Lean 4 reserved symbols.
#[allow(dead_code)]
#[allow(missing_docs)]
pub const LEAN_RESERVED_SYMBOLS: &[&str] = &[
    "->", "=>", ":=", "::", ".", "..", "...", "<|", "|>", "@", "#", "!", "?", "←", "→", "↔", "∀",
    "∃", "∧", "∨", "¬", "⟨", "⟩", "⟪", "⟫",
];
/// Returns true if a string is a reserved symbol.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn is_reserved_symbol(s: &str) -> bool {
    LEAN_RESERVED_SYMBOLS.contains(&s)
}
/// A whitespace normaliser for source code.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn normalize_whitespace_for_lex(src: &str) -> String {
    src.lines()
        .map(|l| l.trim_end())
        .collect::<Vec<_>>()
        .join("\n")
}
#[cfg(test)]
mod lexer_reserved_tests {
    use super::*;
    use crate::lexer::*;
    use crate::tokens::{StringPart, TokenKind};
    #[test]
    fn test_is_reserved_symbol() {
        assert!(is_reserved_symbol("->"));
        assert!(is_reserved_symbol(":="));
        assert!(!is_reserved_symbol("+"));
        assert!(!is_reserved_symbol("foo"));
    }
    #[test]
    fn test_normalize_whitespace_for_lex() {
        let src = "foo   \nbar  \nbaz";
        let out = normalize_whitespace_for_lex(src);
        assert!(!out.contains("   "));
    }
}
/// Returns true if the character is valid inside a string literal.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn is_string_char(c: char) -> bool {
    c != '"' && c != '\\' && c != '\n'
}
/// Returns true if source starts with a block comment.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn starts_block_comment(src: &str) -> bool {
    src.starts_with("/-")
}
/// Returns true if source starts with a line comment.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn starts_line_comment(src: &str) -> bool {
    src.starts_with("--")
}
/// Count occurrences of a char in source.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn count_char(src: &str, c: char) -> usize {
    src.chars().filter(|&x| x == c).count()
}
#[cfg(test)]
mod lexer_pad {
    use super::*;
    use crate::lexer::*;
    use crate::tokens::{StringPart, TokenKind};
    #[test]
    fn test_is_string_char() {
        assert!(is_string_char('a'));
        assert!(!is_string_char('"'));
    }
    #[test]
    fn test_starts_comments() {
        assert!(starts_line_comment("-- hello"));
        assert!(starts_block_comment("/- hello"));
    }
    #[test]
    fn test_count_char() {
        assert_eq!(count_char("hello world", 'l'), 3);
    }
}
/// Reserved symbols that cannot be used as identifiers.
#[allow(dead_code)]
#[allow(missing_docs)]
pub const LEAN_RESERVED_SYMBOLS_EXT2: &[&str] = &[
    ":=", "->", "=>", "::", "...", "..", ".", ":", "|", "\\", "#", "@", "!", "?", ";", ",", "(",
    ")", "[", "]", "{", "}",
];
/// Returns true if the given string is a reserved symbol.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn is_reserved_symbol_ext2(s: &str) -> bool {
    LEAN_RESERVED_SYMBOLS_EXT2.contains(&s)
}
/// Normalize whitespace for lexing (collapse runs of spaces/tabs to a single space).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn normalize_whitespace_for_lex_ext2(src: &str) -> String {
    let mut result = String::with_capacity(src.len());
    let mut prev_space = false;
    for c in src.chars() {
        if c == ' ' || c == '\t' {
            if !prev_space {
                result.push(' ');
            }
            prev_space = true;
        } else {
            result.push(c);
            prev_space = false;
        }
    }
    result
}
#[cfg(test)]
mod lexer_pad2 {
    use super::*;
    use crate::lexer::*;
    use crate::tokens::{StringPart, TokenKind};
    #[test]
    fn test_is_reserved_symbol_ext2() {
        assert!(is_reserved_symbol_ext2(":="));
        assert!(!is_reserved_symbol_ext2("foo"));
    }
    #[test]
    fn test_normalize_whitespace_for_lex_ext2() {
        assert_eq!(normalize_whitespace_for_lex_ext2("a  b   c"), "a b c");
        assert_eq!(normalize_whitespace_for_lex_ext2("no extra"), "no extra");
    }
    #[test]
    fn test_char_freq_table() {
        let t = CharFreqTable::from_str("aabbc");
        assert_eq!(t.count('a'), 2);
        assert_eq!(t.count('b'), 2);
        assert_eq!(t.count('c'), 1);
        assert_eq!(t.count('z'), 0);
    }
}
/// Returns true if the given string is a valid Lean operator token.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn is_lean_op_token(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| "!#$%&*+-./:<=>?@\\^|~".contains(c))
}
