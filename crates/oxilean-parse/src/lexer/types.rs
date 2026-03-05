//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::tokens::{Span, StringPart, Token, TokenKind};

/// A lexer mode stack.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct LexerModeStack {
    /// The stack of modes
    pub stack: Vec<LexerMode>,
}
impl LexerModeStack {
    /// Create a new mode stack starting in Normal mode.
    #[allow(dead_code)]
    pub fn new() -> Self {
        LexerModeStack {
            stack: vec![LexerMode::Normal],
        }
    }
    /// Push a new mode.
    #[allow(dead_code)]
    pub fn push(&mut self, mode: LexerMode) {
        self.stack.push(mode);
    }
    /// Pop the current mode, returning to the previous.
    #[allow(dead_code)]
    pub fn pop(&mut self) -> Option<LexerMode> {
        if self.stack.len() > 1 {
            self.stack.pop()
        } else {
            None
        }
    }
    /// Returns the current mode.
    #[allow(dead_code)]
    pub fn current(&self) -> &LexerMode {
        self.stack.last().unwrap_or(&LexerMode::Normal)
    }
    /// Returns the depth of the stack.
    #[allow(dead_code)]
    pub fn depth(&self) -> usize {
        self.stack.len()
    }
}
/// Lexer for tokenizing OxiLean source code.
///
/// Supports:
/// - Keywords and identifiers (including Unicode letters: Greek, math symbols)
/// - Number literals: decimal, hex (`0x`), binary (`0b`), octal (`0o`)
/// - Float literals: `3.14`, `1.0e10`, `1.5e-3`
/// - String literals with enhanced escape sequences (`\u{1234}`, `\x41`, `\0`)
/// - Char literals: `'a'`, `'\n'`, `'\x41'`
/// - Interpolated strings: `s!"hello {name}"`
/// - Doc comments: `/-- ... -/` (block) and `--- ...` (line)
/// - Operators: `>=`, `..`, `=>`, `<-`, `->`, etc.
pub struct Lexer {
    /// Input source code
    input: Vec<char>,
    /// Current position
    pos: usize,
    /// Current line (1-indexed)
    line: usize,
    /// Current column (1-indexed)
    column: usize,
}
impl Lexer {
    /// Create a new lexer from source code.
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
            line: 1,
            column: 1,
        }
    }
    /// Get the current character without consuming it.
    fn peek(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }
    /// Get the character at offset from current position.
    fn peek_ahead(&self, offset: usize) -> Option<char> {
        self.input.get(self.pos + offset).copied()
    }
    /// Consume and return the current character.
    fn advance(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.pos += 1;
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(ch)
    }
    /// Skip whitespace and comments (but not doc comments).
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else if ch == '-' && self.peek_ahead(1) == Some('-') {
                if self.peek_ahead(2) == Some('-') {
                    break;
                }
                self.advance();
                self.advance();
                while let Some(ch) = self.peek() {
                    if ch == '\n' {
                        break;
                    }
                    self.advance();
                }
            } else if ch == '/' && self.peek_ahead(1) == Some('-') {
                if self.peek_ahead(2) == Some('-') {
                    break;
                }
                self.advance();
                self.advance();
                let mut depth = 1;
                while depth > 0 {
                    match self.peek() {
                        Some('/') if self.peek_ahead(1) == Some('-') => {
                            self.advance();
                            self.advance();
                            depth += 1;
                        }
                        Some('-') if self.peek_ahead(1) == Some('/') => {
                            self.advance();
                            self.advance();
                            depth -= 1;
                        }
                        Some(_) => {
                            self.advance();
                        }
                        None => break,
                    }
                }
            } else {
                break;
            }
        }
    }
    /// Lex a line doc comment: `--- ...` (everything until newline).
    fn lex_line_doc_comment(&mut self, start: usize, start_line: usize, start_col: usize) -> Token {
        self.advance();
        self.advance();
        self.advance();
        if self.peek() == Some(' ') {
            self.advance();
        }
        let mut s = String::new();
        while let Some(ch) = self.peek() {
            if ch == '\n' {
                break;
            }
            s.push(ch);
            self.advance();
        }
        Token::new(
            TokenKind::DocComment(s),
            Span::new(start, self.pos, start_line, start_col),
        )
    }
    /// Lex a block doc comment: `/-- ... -/`.
    fn lex_block_doc_comment(
        &mut self,
        start: usize,
        start_line: usize,
        start_col: usize,
    ) -> Token {
        self.advance();
        self.advance();
        self.advance();
        if self.peek() == Some(' ') {
            self.advance();
        }
        let mut s = String::new();
        let mut depth = 1;
        while depth > 0 {
            match self.peek() {
                Some('/') if self.peek_ahead(1) == Some('-') => {
                    self.advance();
                    self.advance();
                    depth += 1;
                    if depth > 1 {
                        s.push_str("/-");
                    }
                }
                Some('-') if self.peek_ahead(1) == Some('/') => {
                    depth -= 1;
                    if depth > 0 {
                        s.push_str("-/");
                    }
                    self.advance();
                    self.advance();
                }
                Some(ch) => {
                    s.push(ch);
                    self.advance();
                }
                None => break,
            }
        }
        let s = s.trim_end().to_string();
        Token::new(
            TokenKind::DocComment(s),
            Span::new(start, self.pos, start_line, start_col),
        )
    }
    /// Lex an identifier or keyword.
    /// Identifiers can contain Unicode letters (Greek, mathematical symbols, etc.),
    /// digits, underscores, and primes.
    fn lex_ident(&mut self, start: usize, start_line: usize, start_col: usize) -> Token {
        let mut s = String::new();
        if self.peek() == Some('_')
            && !self
                .peek_ahead(1)
                .is_some_and(|c| c.is_alphanumeric() || c == '_')
        {
            self.advance();
            return Token::new(
                TokenKind::Underscore,
                Span::new(start, self.pos, start_line, start_col),
            );
        }
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' || ch == '\'' {
                s.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        let kind = match s.as_str() {
            "axiom" => TokenKind::Axiom,
            "definition" | "def" => TokenKind::Definition,
            "theorem" => TokenKind::Theorem,
            "lemma" => TokenKind::Lemma,
            "opaque" => TokenKind::Opaque,
            "inductive" => TokenKind::Inductive,
            "structure" => TokenKind::Structure,
            "class" => TokenKind::Class,
            "instance" => TokenKind::Instance,
            "namespace" => TokenKind::Namespace,
            "section" => TokenKind::Section,
            "variable" => TokenKind::Variable,
            "variables" => TokenKind::Variables,
            "parameter" => TokenKind::Parameter,
            "parameters" => TokenKind::Parameters,
            "constant" => TokenKind::Constant,
            "constants" => TokenKind::Constants,
            "end" => TokenKind::End,
            "import" => TokenKind::Import,
            "export" => TokenKind::Export,
            "open" => TokenKind::Open,
            "attribute" => TokenKind::Attribute,
            "return" => TokenKind::Return,
            "Type" => TokenKind::Type,
            "Prop" => TokenKind::Prop,
            "Sort" => TokenKind::Sort,
            "fun" => TokenKind::Fun,
            "forall" => TokenKind::Forall,
            "let" => TokenKind::Let,
            "in" => TokenKind::In,
            "if" => TokenKind::If,
            "then" => TokenKind::Then,
            "else" => TokenKind::Else,
            "match" => TokenKind::Match,
            "with" => TokenKind::With,
            "do" => TokenKind::Do,
            "have" => TokenKind::Have,
            "suffices" => TokenKind::Suffices,
            "show" => TokenKind::Show,
            "where" => TokenKind::Where,
            "from" => TokenKind::From,
            "by" => TokenKind::By,
            "exists" => TokenKind::Exists,
            _ => TokenKind::Ident(s),
        };
        Token::new(kind, Span::new(start, self.pos, start_line, start_col))
    }
    /// Lex a hex number literal: `0x1A3F`.
    fn lex_hex_number(&mut self, start: usize, start_line: usize, start_col: usize) -> Token {
        let mut num = 0u64;
        let mut has_digits = false;
        while let Some(ch) = self.peek() {
            if let Some(d) = ch.to_digit(16) {
                num = num.wrapping_mul(16).wrapping_add(d as u64);
                has_digits = true;
                self.advance();
            } else if ch == '_' {
                self.advance();
            } else {
                break;
            }
        }
        if !has_digits {
            return Token::new(
                TokenKind::Error("expected hex digits after 0x".to_string()),
                Span::new(start, self.pos, start_line, start_col),
            );
        }
        Token::new(
            TokenKind::Nat(num),
            Span::new(start, self.pos, start_line, start_col),
        )
    }
    /// Lex a binary number literal: `0b1010`.
    fn lex_bin_number(&mut self, start: usize, start_line: usize, start_col: usize) -> Token {
        let mut num = 0u64;
        let mut has_digits = false;
        while let Some(ch) = self.peek() {
            match ch {
                '0' => {
                    num = num.wrapping_mul(2);
                    has_digits = true;
                    self.advance();
                }
                '1' => {
                    num = num.wrapping_mul(2).wrapping_add(1);
                    has_digits = true;
                    self.advance();
                }
                '_' => {
                    self.advance();
                }
                _ => break,
            }
        }
        if !has_digits {
            return Token::new(
                TokenKind::Error("expected binary digits after 0b".to_string()),
                Span::new(start, self.pos, start_line, start_col),
            );
        }
        Token::new(
            TokenKind::Nat(num),
            Span::new(start, self.pos, start_line, start_col),
        )
    }
    /// Lex an octal number literal: `0o17`.
    fn lex_oct_number(&mut self, start: usize, start_line: usize, start_col: usize) -> Token {
        let mut num = 0u64;
        let mut has_digits = false;
        while let Some(ch) = self.peek() {
            if let Some(d) = ch.to_digit(8) {
                num = num.wrapping_mul(8).wrapping_add(d as u64);
                has_digits = true;
                self.advance();
            } else if ch == '_' {
                self.advance();
            } else {
                break;
            }
        }
        if !has_digits {
            return Token::new(
                TokenKind::Error("expected octal digits after 0o".to_string()),
                Span::new(start, self.pos, start_line, start_col),
            );
        }
        Token::new(
            TokenKind::Nat(num),
            Span::new(start, self.pos, start_line, start_col),
        )
    }
    /// Lex a number (decimal integer or float, or hex/bin/oct).
    fn lex_number(&mut self, start: usize, start_line: usize, start_col: usize) -> Token {
        if self.peek() == Some('0') {
            match self.peek_ahead(1) {
                Some('x') | Some('X') => {
                    self.advance();
                    self.advance();
                    return self.lex_hex_number(start, start_line, start_col);
                }
                Some('b') | Some('B') => {
                    self.advance();
                    self.advance();
                    return self.lex_bin_number(start, start_line, start_col);
                }
                Some('o') | Some('O') => {
                    self.advance();
                    self.advance();
                    return self.lex_oct_number(start, start_line, start_col);
                }
                _ => {}
            }
        }
        let mut int_str = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                int_str.push(ch);
                self.advance();
            } else if ch == '_' && self.peek_ahead(1).is_some_and(|c| c.is_ascii_digit()) {
                self.advance();
            } else {
                break;
            }
        }
        let is_float =
            self.peek() == Some('.') && self.peek_ahead(1).is_some_and(|c| c.is_ascii_digit());
        if is_float {
            self.advance();
            let mut frac_str = String::new();
            frac_str.push_str(&int_str);
            frac_str.push('.');
            while let Some(ch) = self.peek() {
                if ch.is_ascii_digit() {
                    frac_str.push(ch);
                    self.advance();
                } else {
                    break;
                }
            }
            if self.peek() == Some('e') || self.peek() == Some('E') {
                frac_str.push('e');
                self.advance();
                if self.peek() == Some('+') || self.peek() == Some('-') {
                    frac_str.push(self.advance().expect("peek confirmed '+' or '-' exists"));
                }
                while let Some(ch) = self.peek() {
                    if ch.is_ascii_digit() {
                        frac_str.push(ch);
                        self.advance();
                    } else {
                        break;
                    }
                }
            }
            let val: f64 = frac_str.parse().unwrap_or(0.0);
            return Token::new(
                TokenKind::Float(val),
                Span::new(start, self.pos, start_line, start_col),
            );
        }
        if self.peek() == Some('e') || self.peek() == Some('E') {
            let mut exp_str = String::new();
            exp_str.push_str(&int_str);
            exp_str.push('e');
            self.advance();
            if self.peek() == Some('+') || self.peek() == Some('-') {
                exp_str.push(self.advance().expect("peek confirmed '+' or '-' exists"));
            }
            while let Some(ch) = self.peek() {
                if ch.is_ascii_digit() {
                    exp_str.push(ch);
                    self.advance();
                } else {
                    break;
                }
            }
            let val: f64 = exp_str.parse().unwrap_or(0.0);
            return Token::new(
                TokenKind::Float(val),
                Span::new(start, self.pos, start_line, start_col),
            );
        }
        let num: u64 = int_str.parse().unwrap_or(0);
        Token::new(
            TokenKind::Nat(num),
            Span::new(start, self.pos, start_line, start_col),
        )
    }
    /// Parse an escape sequence inside a string or char literal.
    /// Returns the resolved character or None on error.
    fn parse_escape_char(&mut self) -> Option<char> {
        match self.peek() {
            Some('n') => {
                self.advance();
                Some('\n')
            }
            Some('t') => {
                self.advance();
                Some('\t')
            }
            Some('r') => {
                self.advance();
                Some('\r')
            }
            Some('\\') => {
                self.advance();
                Some('\\')
            }
            Some('"') => {
                self.advance();
                Some('"')
            }
            Some('\'') => {
                self.advance();
                Some('\'')
            }
            Some('0') => {
                self.advance();
                Some('\0')
            }
            Some('x') => {
                self.advance();
                let hi = self.advance().and_then(|c| c.to_digit(16))?;
                let lo = self.advance().and_then(|c| c.to_digit(16))?;
                let val = (hi * 16 + lo) as u8;
                Some(val as char)
            }
            Some('u') => {
                self.advance();
                if self.peek() != Some('{') {
                    return None;
                }
                self.advance();
                let mut code = 0u32;
                let mut has_digit = false;
                while let Some(ch) = self.peek() {
                    if ch == '}' {
                        self.advance();
                        break;
                    }
                    if let Some(d) = ch.to_digit(16) {
                        code = code * 16 + d;
                        has_digit = true;
                        self.advance();
                    } else {
                        return None;
                    }
                }
                if !has_digit {
                    return None;
                }
                char::from_u32(code)
            }
            Some(ch) => {
                self.advance();
                Some(ch)
            }
            None => None,
        }
    }
    /// Lex a string literal (with enhanced escape sequences).
    fn lex_string(&mut self, start: usize, start_line: usize, start_col: usize) -> Token {
        self.advance();
        let mut s = String::new();
        while let Some(ch) = self.peek() {
            if ch == '"' {
                self.advance();
                return Token::new(
                    TokenKind::String(s),
                    Span::new(start, self.pos, start_line, start_col),
                );
            } else if ch == '\\' {
                self.advance();
                if let Some(escaped) = self.parse_escape_char() {
                    s.push(escaped);
                } else {
                    s.push('?');
                }
            } else {
                s.push(ch);
                self.advance();
            }
        }
        Token::new(
            TokenKind::Error("unterminated string".to_string()),
            Span::new(start, self.pos, start_line, start_col),
        )
    }
    /// Lex a char literal: `'a'`, `'\n'`, `'\x41'`, `'\u{03B1}'`.
    fn lex_char(&mut self, start: usize, start_line: usize, start_col: usize) -> Token {
        self.advance();
        let ch = if self.peek() == Some('\\') {
            self.advance();
            match self.parse_escape_char() {
                Some(c) => c,
                None => {
                    return Token::new(
                        TokenKind::Error("invalid escape in char literal".to_string()),
                        Span::new(start, self.pos, start_line, start_col),
                    );
                }
            }
        } else {
            match self.advance() {
                Some(c) => c,
                None => {
                    return Token::new(
                        TokenKind::Error("unterminated char literal".to_string()),
                        Span::new(start, self.pos, start_line, start_col),
                    );
                }
            }
        };
        if self.peek() == Some('\'') {
            self.advance();
            Token::new(
                TokenKind::Char(ch),
                Span::new(start, self.pos, start_line, start_col),
            )
        } else {
            Token::new(
                TokenKind::Error("unterminated char literal".to_string()),
                Span::new(start, self.pos, start_line, start_col),
            )
        }
    }
    /// Lex an interpolated string: `s!"hello {name}"`.
    /// Assumes `s` and `!` have NOT been consumed yet; we are at 's'.
    fn lex_interpolated_string(
        &mut self,
        start: usize,
        start_line: usize,
        start_col: usize,
    ) -> Token {
        self.advance();
        self.advance();
        self.advance();
        let mut parts: Vec<StringPart> = Vec::new();
        let mut current_lit = String::new();
        while let Some(ch) = self.peek() {
            if ch == '"' {
                self.advance();
                if !current_lit.is_empty() {
                    parts.push(StringPart::Literal(current_lit));
                }
                return Token::new(
                    TokenKind::InterpolatedString(parts),
                    Span::new(start, self.pos, start_line, start_col),
                );
            } else if ch == '{' {
                self.advance();
                if !current_lit.is_empty() {
                    parts.push(StringPart::Literal(current_lit.clone()));
                    current_lit.clear();
                }
                let mut interp_tokens = Vec::new();
                let mut depth = 1;
                loop {
                    let tok = self.next_token_raw();
                    match &tok.kind {
                        TokenKind::LBrace => depth += 1,
                        TokenKind::RBrace => {
                            depth -= 1;
                            if depth == 0 {
                                break;
                            }
                        }
                        TokenKind::Eof => break,
                        _ => {}
                    }
                    interp_tokens.push(tok);
                }
                parts.push(StringPart::Interpolation(interp_tokens));
            } else if ch == '\\' {
                self.advance();
                if let Some(escaped) = self.parse_escape_char() {
                    current_lit.push(escaped);
                } else {
                    current_lit.push('?');
                }
            } else {
                current_lit.push(ch);
                self.advance();
            }
        }
        Token::new(
            TokenKind::Error("unterminated interpolated string".to_string()),
            Span::new(start, self.pos, start_line, start_col),
        )
    }
    /// Lex a single token without skipping whitespace first.
    /// Used internally for interpolated string sub-lexing.
    fn next_token_raw(&mut self) -> Token {
        self.skip_whitespace();
        let start = self.pos;
        let start_line = self.line;
        let start_col = self.column;
        let Some(ch) = self.peek() else {
            return Token::new(
                TokenKind::Eof,
                Span::new(start, start, start_line, start_col),
            );
        };
        if ch.is_alphabetic() || ch == '_' {
            return self.lex_ident(start, start_line, start_col);
        }
        if ch.is_ascii_digit() {
            return self.lex_number(start, start_line, start_col);
        }
        if ch == '"' {
            return self.lex_string(start, start_line, start_col);
        }
        self.advance();
        let kind = match ch {
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '{' => TokenKind::LBrace,
            '}' => TokenKind::RBrace,
            '[' => TokenKind::LBracket,
            ']' => TokenKind::RBracket,
            ',' => TokenKind::Comma,
            ';' => TokenKind::Semicolon,
            '.' if self.peek() == Some('.') => {
                self.advance();
                TokenKind::DotDot
            }
            '.' => TokenKind::Dot,
            '|' if self.peek() == Some('|') => {
                self.advance();
                TokenKind::OrOr
            }
            '|' => TokenKind::Bar,
            '@' => TokenKind::At,
            '#' => TokenKind::Hash,
            '+' => TokenKind::Plus,
            '-' if self.peek() == Some('>') => {
                self.advance();
                TokenKind::Arrow
            }
            '-' => TokenKind::Minus,
            '>' if self.peek() == Some('=') => {
                self.advance();
                TokenKind::Ge
            }
            '>' => TokenKind::Gt,
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,
            '%' => TokenKind::Percent,
            '^' => TokenKind::Caret,
            '_' => TokenKind::Underscore,
            '?' => TokenKind::Question,
            '!' if self.peek() == Some('=') => {
                self.advance();
                TokenKind::BangEq
            }
            '!' => TokenKind::Bang,
            '&' if self.peek() == Some('&') => {
                self.advance();
                TokenKind::AndAnd
            }
            ':' if self.peek() == Some('=') => {
                self.advance();
                TokenKind::Assign
            }
            ':' => TokenKind::Colon,
            '=' if self.peek() == Some('>') => {
                self.advance();
                TokenKind::FatArrow
            }
            '=' => TokenKind::Eq,
            '<' if self.peek() == Some('-') => {
                self.advance();
                TokenKind::LeftArrow
            }
            '<' if self.peek() == Some('=') => {
                self.advance();
                TokenKind::Le
            }
            '<' => TokenKind::Lt,
            '\u{2190}' => TokenKind::LeftArrow,
            '\u{2264}' | '\u{2A7D}' => TokenKind::Le,
            '\u{2265}' | '\u{2A7E}' => TokenKind::Ge,
            '\u{2260}' => TokenKind::Ne,
            '\u{2192}' => TokenKind::Arrow,
            '\u{21D2}' => TokenKind::FatArrow,
            '\u{2227}' => TokenKind::And,
            '\u{2228}' => TokenKind::Or,
            '\u{00AC}' => TokenKind::Not,
            '\u{2194}' => TokenKind::Iff,
            '\u{2200}' => TokenKind::Forall,
            '\u{2203}' => TokenKind::Exists,
            '\u{03BB}' => TokenKind::Fun,
            '\u{27E8}' => TokenKind::LAngle,
            '\u{27E9}' => TokenKind::RAngle,
            _ => TokenKind::Error(format!("unexpected character: {}", ch)),
        };
        Token::new(kind, Span::new(start, self.pos, start_line, start_col))
    }
    /// Get the next token.
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let start = self.pos;
        let start_line = self.line;
        let start_col = self.column;
        let Some(ch) = self.peek() else {
            return Token::new(
                TokenKind::Eof,
                Span::new(start, start, start_line, start_col),
            );
        };
        if ch == '/' && self.peek_ahead(1) == Some('-') && self.peek_ahead(2) == Some('-') {
            return self.lex_block_doc_comment(start, start_line, start_col);
        }
        if ch == '-' && self.peek_ahead(1) == Some('-') && self.peek_ahead(2) == Some('-') {
            return self.lex_line_doc_comment(start, start_line, start_col);
        }
        if ch == 's' && self.peek_ahead(1) == Some('!') && self.peek_ahead(2) == Some('"') {
            return self.lex_interpolated_string(start, start_line, start_col);
        }
        if ch.is_alphabetic() || ch == '_' {
            return self.lex_ident(start, start_line, start_col);
        }
        if ch.is_ascii_digit() {
            return self.lex_number(start, start_line, start_col);
        }
        if ch == '"' {
            return self.lex_string(start, start_line, start_col);
        }
        if ch == '\'' {
            let is_char_lit = if self.peek_ahead(1) == Some('\\') {
                true
            } else {
                self.peek_ahead(2) == Some('\'')
            };
            if is_char_lit {
                return self.lex_char(start, start_line, start_col);
            }
        }
        self.advance();
        let kind = match ch {
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '{' => TokenKind::LBrace,
            '}' => TokenKind::RBrace,
            '[' => TokenKind::LBracket,
            ']' => TokenKind::RBracket,
            ',' => TokenKind::Comma,
            ';' => TokenKind::Semicolon,
            '.' if self.peek() == Some('.') => {
                self.advance();
                TokenKind::DotDot
            }
            '.' => TokenKind::Dot,
            '|' if self.peek() == Some('|') => {
                self.advance();
                TokenKind::OrOr
            }
            '|' => TokenKind::Bar,
            '@' => TokenKind::At,
            '#' => TokenKind::Hash,
            '+' => TokenKind::Plus,
            '-' if self.peek() == Some('>') => {
                self.advance();
                TokenKind::Arrow
            }
            '-' => TokenKind::Minus,
            '>' if self.peek() == Some('=') => {
                self.advance();
                TokenKind::Ge
            }
            '>' => TokenKind::Gt,
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,
            '%' => TokenKind::Percent,
            '^' => TokenKind::Caret,
            '_' => TokenKind::Underscore,
            '?' => TokenKind::Question,
            '\'' => TokenKind::Error("unexpected tick '".to_string()),
            '!' if self.peek() == Some('=') => {
                self.advance();
                TokenKind::BangEq
            }
            '!' => TokenKind::Bang,
            '&' if self.peek() == Some('&') => {
                self.advance();
                TokenKind::AndAnd
            }
            ':' if self.peek() == Some('=') => {
                self.advance();
                TokenKind::Assign
            }
            ':' => TokenKind::Colon,
            '=' if self.peek() == Some('>') => {
                self.advance();
                TokenKind::FatArrow
            }
            '=' => TokenKind::Eq,
            '<' if self.peek() == Some('-') => {
                self.advance();
                TokenKind::LeftArrow
            }
            '<' if self.peek() == Some('=') => {
                self.advance();
                TokenKind::Le
            }
            '<' => TokenKind::Lt,
            '\u{2190}' => TokenKind::LeftArrow,
            '\u{2264}' | '\u{2A7D}' => TokenKind::Le,
            '\u{2265}' | '\u{2A7E}' => TokenKind::Ge,
            '\u{2260}' => TokenKind::Ne,
            '\u{2192}' => TokenKind::Arrow,
            '\u{21D2}' => TokenKind::FatArrow,
            '\u{2227}' => TokenKind::And,
            '\u{2228}' => TokenKind::Or,
            '\u{00AC}' => TokenKind::Not,
            '\u{2194}' => TokenKind::Iff,
            '\u{2200}' => TokenKind::Forall,
            '\u{2203}' => TokenKind::Exists,
            '\u{03BB}' => TokenKind::Fun,
            '\u{27E8}' => TokenKind::LAngle,
            '\u{27E9}' => TokenKind::RAngle,
            _ => TokenKind::Error(format!("unexpected character: {}", ch)),
        };
        Token::new(kind, Span::new(start, self.pos, start_line, start_col))
    }
    /// Tokenize the entire input.
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token();
            let is_eof = matches!(tok.kind, TokenKind::Eof);
            tokens.push(tok);
            if is_eof {
                break;
            }
        }
        tokens
    }
}
/// A simple string scanner with position tracking.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct Scanner<'a> {
    /// The source text
    src: &'a str,
    /// Current byte position
    pos: usize,
    /// Current line number (1-based)
    line: usize,
    /// Current column number (1-based)
    col: usize,
}
impl<'a> Scanner<'a> {
    /// Create a new scanner at the start of the source.
    #[allow(dead_code)]
    pub fn new(src: &'a str) -> Self {
        Scanner {
            src,
            pos: 0,
            line: 1,
            col: 1,
        }
    }
    /// Peek at the current character without consuming it.
    #[allow(dead_code)]
    pub fn peek(&self) -> Option<char> {
        self.src[self.pos..].chars().next()
    }
    /// Consume the next character.
    #[allow(dead_code)]
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<char> {
        let c = self.src[self.pos..].chars().next()?;
        self.pos += c.len_utf8();
        if c == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        Some(c)
    }
    /// Returns true if the scanner is at the end.
    #[allow(dead_code)]
    pub fn is_eof(&self) -> bool {
        self.pos >= self.src.len()
    }
    /// Returns the current position.
    #[allow(dead_code)]
    pub fn position(&self) -> usize {
        self.pos
    }
    /// Returns the current line.
    #[allow(dead_code)]
    pub fn line(&self) -> usize {
        self.line
    }
    /// Returns the current column.
    #[allow(dead_code)]
    pub fn col(&self) -> usize {
        self.col
    }
    /// Skip whitespace.
    #[allow(dead_code)]
    pub fn skip_whitespace(&mut self) {
        while self.peek().map(|c| c.is_whitespace()).unwrap_or(false) {
            self.next();
        }
    }
    /// Consume characters while the predicate holds, returning the matched slice.
    #[allow(dead_code)]
    pub fn consume_while<F: Fn(char) -> bool>(&mut self, pred: F) -> &'a str {
        let start = self.pos;
        while self.peek().is_some_and(&pred) {
            self.next();
        }
        &self.src[start..self.pos]
    }
    /// Returns the remaining source.
    #[allow(dead_code)]
    pub fn remaining(&self) -> &'a str {
        &self.src[self.pos..]
    }
}
/// A lexer position record for backtracking.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy)]
pub struct LexerPos {
    /// Byte offset
    pub offset: usize,
    /// Line number
    pub line: usize,
    /// Column number
    pub col: usize,
}
/// A lexer rule table.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct LexerRuleTable {
    /// All rules in this table
    pub rules: Vec<LexerRule>,
}
impl LexerRuleTable {
    /// Create a new empty table.
    #[allow(dead_code)]
    pub fn new() -> Self {
        LexerRuleTable { rules: Vec::new() }
    }
    /// Add a rule.
    #[allow(dead_code)]
    pub fn add(&mut self, rule: LexerRule) {
        self.rules.push(rule);
    }
    /// Find the first rule whose start class matches a character.
    #[allow(dead_code)]
    pub fn find_rule(&self, c: char) -> Option<&LexerRule> {
        self.rules.iter().find(|r| r.start.matches(c))
    }
    /// Returns the number of rules.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.rules.len()
    }
    /// Returns true if the table is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
}
/// A token category for grouping.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenCategory {
    /// Identifier tokens
    Ident,
    /// Keyword tokens
    Keyword,
    /// Literal tokens
    Literal,
    /// Operator tokens
    Operator,
    /// Delimiter tokens (parens, brackets, etc.)
    Delimiter,
    /// Whitespace/trivia tokens
    Trivia,
    /// EOF token
    Eof,
}
/// A token diff record showing insertions/deletions between two token sequences.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct TokenDiff {
    /// Number of tokens only in left
    pub only_left: usize,
    /// Number of tokens only in right
    pub only_right: usize,
    /// Number of matching tokens
    pub matching: usize,
}
/// A summary of a lexed file.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Default)]
pub struct LexSummary {
    /// Total token count
    pub token_count: usize,
    /// Number of identifier tokens
    pub ident_count: usize,
    /// Number of keyword tokens
    pub keyword_count: usize,
    /// Number of operator tokens
    pub operator_count: usize,
    /// Number of literal tokens
    pub literal_count: usize,
    /// Number of comment tokens
    pub comment_count: usize,
    /// Number of whitespace tokens
    pub whitespace_count: usize,
}
impl LexSummary {
    /// Create a new empty summary.
    #[allow(dead_code)]
    pub fn new() -> Self {
        LexSummary::default()
    }
    /// Format the summary as a string.
    #[allow(dead_code)]
    pub fn format(&self) -> String {
        format!(
            "tokens={} idents={} keywords={} ops={} literals={} comments={} ws={}",
            self.token_count,
            self.ident_count,
            self.keyword_count,
            self.operator_count,
            self.literal_count,
            self.comment_count,
            self.whitespace_count
        )
    }
}
/// A keyword trie node for fast keyword detection.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct KeywordTrie {
    /// Children by character
    children: std::collections::HashMap<char, KeywordTrie>,
    /// Whether this node is a terminal keyword
    pub is_keyword: bool,
}
impl KeywordTrie {
    /// Create an empty trie.
    #[allow(dead_code)]
    pub fn new() -> Self {
        KeywordTrie {
            children: std::collections::HashMap::new(),
            is_keyword: false,
        }
    }
    /// Insert a keyword into the trie.
    #[allow(dead_code)]
    pub fn insert(&mut self, keyword: &str) {
        let mut node = self;
        for c in keyword.chars() {
            node = node.children.entry(c).or_default();
        }
        node.is_keyword = true;
    }
    /// Check if a string is in the trie.
    #[allow(dead_code)]
    pub fn contains(&self, s: &str) -> bool {
        let mut node = self;
        for c in s.chars() {
            match node.children.get(&c) {
                Some(child) => node = child,
                None => return false,
            }
        }
        node.is_keyword
    }
}
/// A lexer rule: a name, a start class, and a continuation class.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct LexerRule {
    /// Name of the token kind produced
    pub kind: String,
    /// Character class for the first character
    pub start: CharClass,
    /// Character class for continuation characters
    pub cont: CharClass,
}
impl LexerRule {
    /// Create a new lexer rule.
    #[allow(dead_code)]
    pub fn new(kind: &str, start: CharClass, cont: CharClass) -> Self {
        LexerRule {
            kind: kind.to_string(),
            start,
            cont,
        }
    }
}
/// A trivia (whitespace/comment) accumulator.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Default)]
pub struct TriviaAccumulator {
    /// Accumulated trivia text
    pub text: String,
    /// Whether any newlines were found
    pub has_newlines: bool,
}
impl TriviaAccumulator {
    /// Create a new accumulator.
    #[allow(dead_code)]
    pub fn new() -> Self {
        TriviaAccumulator {
            text: String::new(),
            has_newlines: false,
        }
    }
    /// Add trivia text.
    #[allow(dead_code)]
    pub fn push(&mut self, s: &str) {
        if s.contains('\n') {
            self.has_newlines = true;
        }
        self.text.push_str(s);
    }
    /// Clear the accumulator.
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.text.clear();
        self.has_newlines = false;
    }
}
/// A simple character frequency table over a source string.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct CharFreqTable {
    /// Frequency counts indexed by char code (0..128)
    pub counts: [u32; 128],
}
impl CharFreqTable {
    /// Build a frequency table from source.
    #[allow(dead_code)]
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(src: &str) -> Self {
        let mut counts = [0u32; 128];
        for c in src.chars() {
            if (c as usize) < 128 {
                counts[c as usize] += 1;
            }
        }
        CharFreqTable { counts }
    }
    /// Get the count for a character.
    #[allow(dead_code)]
    pub fn count(&self, c: char) -> u32 {
        if (c as usize) < 128 {
            self.counts[c as usize]
        } else {
            0
        }
    }
}
/// A simple tokenisation result with raw text.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct RawToken {
    /// Kind string
    pub kind: String,
    /// Raw text of the token
    pub text: String,
    /// Start byte offset
    pub start: usize,
    /// End byte offset
    pub end: usize,
    /// Line number
    pub line: usize,
    /// Column number
    pub col: usize,
}
/// A filtered view of a token sequence.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct FilteredTokenStream {
    /// The underlying tokens
    pub tokens: Vec<RawToken>,
    /// Current index
    pos: usize,
}
impl FilteredTokenStream {
    /// Create from a token list, filtering out whitespace.
    #[allow(dead_code)]
    pub fn new(tokens: Vec<RawToken>) -> Self {
        let tokens = tokens.into_iter().filter(|t| t.kind != "WS").collect();
        FilteredTokenStream { tokens, pos: 0 }
    }
    /// Peek at the current token.
    #[allow(dead_code)]
    pub fn peek(&self) -> Option<&RawToken> {
        self.tokens.get(self.pos)
    }
    /// Consume the current token.
    #[allow(dead_code)]
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&RawToken> {
        let tok = self.tokens.get(self.pos)?;
        self.pos += 1;
        Some(tok)
    }
    /// Returns true if at end.
    #[allow(dead_code)]
    pub fn is_eof(&self) -> bool {
        self.pos >= self.tokens.len()
    }
    /// Returns remaining token count.
    #[allow(dead_code)]
    pub fn remaining(&self) -> usize {
        self.tokens.len().saturating_sub(self.pos)
    }
    /// Returns position.
    #[allow(dead_code)]
    pub fn position(&self) -> usize {
        self.pos
    }
}
/// A simple character class for lexer rules.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub enum CharClass {
    /// Matches any ASCII letter
    Alpha,
    /// Matches any ASCII digit
    Digit,
    /// Matches any ASCII alphanumeric
    AlphaNum,
    /// Matches whitespace
    Whitespace,
    /// Matches a specific character
    Exact(char),
    /// Matches any character in a set
    OneOf(Vec<char>),
    /// Matches any character not in a set
    NoneOf(Vec<char>),
}
impl CharClass {
    /// Returns true if the character matches this class.
    #[allow(dead_code)]
    pub fn matches(&self, c: char) -> bool {
        match self {
            CharClass::Alpha => c.is_ascii_alphabetic(),
            CharClass::Digit => c.is_ascii_digit(),
            CharClass::AlphaNum => c.is_ascii_alphanumeric(),
            CharClass::Whitespace => c.is_whitespace(),
            CharClass::Exact(x) => c == *x,
            CharClass::OneOf(set) => set.contains(&c),
            CharClass::NoneOf(set) => !set.contains(&c),
        }
    }
}
/// A lexer mode for context-sensitive lexing.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexerMode {
    /// Normal mode
    Normal,
    /// Inside a string literal
    StringLit,
    /// Inside a comment
    Comment,
    /// Inside a tactic block
    Tactic,
}
/// A lexer state machine with explicit transitions.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexerState {
    /// Initial state
    Start,
    /// Scanning identifier
    Ident,
    /// Scanning number
    Number,
    /// Scanning operator
    Operator,
    /// Scanning string
    StringStart,
    /// Scanning line comment
    LineComment,
    /// Done
    Done,
}
/// A utility to find the line boundaries in a source string.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct LineMap {
    /// Byte offsets of each line start
    pub line_starts: Vec<usize>,
}
impl LineMap {
    /// Build a LineMap from source text.
    #[allow(dead_code)]
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(src: &str) -> Self {
        let mut line_starts = vec![0];
        for (i, c) in src.char_indices() {
            if c == '\n' {
                line_starts.push(i + 1);
            }
        }
        LineMap { line_starts }
    }
    /// Returns the line number (1-based) for a byte offset.
    #[allow(dead_code)]
    pub fn line_for_offset(&self, offset: usize) -> usize {
        match self.line_starts.binary_search(&offset) {
            Ok(idx) => idx + 1,
            Err(idx) => idx,
        }
    }
    /// Returns the column (1-based) for a byte offset.
    #[allow(dead_code)]
    pub fn col_for_offset(&self, offset: usize) -> usize {
        let line = self.line_for_offset(offset);
        let line_start = self.line_starts.get(line - 1).copied().unwrap_or(0);
        offset - line_start + 1
    }
    /// Returns the total number of lines.
    #[allow(dead_code)]
    pub fn line_count(&self) -> usize {
        self.line_starts.len()
    }
}
