//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use crate::tokens::{Span, Token, TokenKind};

/// Rich metadata attached to a token.
#[derive(Clone, Debug, PartialEq)]
#[allow(missing_docs)]
pub struct TokenMeta {
    /// The underlying token.
    pub token: Token,
    /// High-level category.
    pub category: TokenCategory,
    /// Raw source text for this token.
    #[allow(missing_docs)]
    pub text: String,
    /// `true` if any whitespace preceded this token on the same line.
    pub preceded_by_space: bool,
    /// `true` if a newline preceded this token.
    pub preceded_by_newline: bool,
}
impl TokenMeta {
    /// Construct a new `TokenMeta`.
    #[allow(missing_docs)]
    pub fn new(
        token: Token,
        category: TokenCategory,
        text: impl Into<String>,
        preceded_by_space: bool,
        preceded_by_newline: bool,
    ) -> Self {
        Self {
            token,
            category,
            text: text.into(),
            preceded_by_space,
            preceded_by_newline,
        }
    }
    /// Convenience: construct from a `Token` and its source slice.
    #[allow(missing_docs)]
    pub fn from_token(token: Token, source: &str) -> Self {
        let span = &token.span;
        let text = source.get(span.start..span.end).unwrap_or("").to_string();
        let category = categorise(&token.kind);
        Self {
            token,
            category,
            text,
            preceded_by_space: false,
            preceded_by_newline: false,
        }
    }
    /// Return the span of this token.
    #[allow(missing_docs)]
    pub fn span(&self) -> &Span {
        &self.token.span
    }
    /// Return the `TokenKind`.
    #[allow(missing_docs)]
    pub fn kind(&self) -> &TokenKind {
        &self.token.kind
    }
    /// `true` if this token is an identifier.
    #[allow(missing_docs)]
    pub fn is_ident(&self) -> bool {
        self.token.is_ident()
    }
    /// `true` if this token is a keyword.
    #[allow(missing_docs)]
    pub fn is_keyword(&self) -> bool {
        self.category == TokenCategory::Keyword
    }
    /// `true` if this token is a literal.
    #[allow(missing_docs)]
    pub fn is_literal(&self) -> bool {
        self.category == TokenCategory::Literal
    }
}
impl TokenMeta {
    /// Annotate with preceding newline based on source position.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn set_preceded_by_newline(&mut self, v: bool) {
        self.preceded_by_newline = v;
    }
    /// Annotate with preceding space.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn set_preceded_by_space(&mut self, v: bool) {
        self.preceded_by_space = v;
    }
    /// Length of the token's source span.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn len(&self) -> usize {
        self.token.span.end.saturating_sub(self.token.span.start)
    }
    /// Whether this token's source span is empty.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Whether this token is a numeric literal.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_numeric(&self) -> bool {
        matches!(self.token.kind, TokenKind::Nat(_) | TokenKind::Float(_))
    }
    /// Whether this token is a string literal.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_string(&self) -> bool {
        matches!(self.token.kind, TokenKind::String(_))
    }
    /// Whether this token is an operator.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_operator(&self) -> bool {
        self.category == TokenCategory::Operator
    }
}
/// High-level category of a token used for error messages and syntax
/// highlighting.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum TokenCategory {
    /// A keyword (`theorem`, `def`, `fun`, …).
    Keyword,
    /// An identifier (`foo`, `α`, `myVar`).
    Identifier,
    /// A numeric or string literal.
    Literal,
    /// A punctuation symbol (`(`, `)`, `,`, …).
    Punctuation,
    /// An operator (`+`, `->`, `≤`, …).
    Operator,
    /// A comment.
    Comment,
    /// End-of-file sentinel.
    Eof,
    /// Anything else.
    Other,
}
impl TokenCategory {
    /// Human-readable name used in error messages.
    #[allow(missing_docs)]
    pub fn name(&self) -> &'static str {
        match self {
            TokenCategory::Keyword => "keyword",
            TokenCategory::Identifier => "identifier",
            TokenCategory::Literal => "literal",
            TokenCategory::Punctuation => "punctuation",
            TokenCategory::Operator => "operator",
            TokenCategory::Comment => "comment",
            TokenCategory::Eof => "end-of-file",
            TokenCategory::Other => "token",
        }
    }
    /// Return `true` if tokens of this category may appear as expression
    /// starters.
    #[allow(missing_docs)]
    pub fn can_start_expr(&self) -> bool {
        matches!(
            self,
            TokenCategory::Identifier | TokenCategory::Literal | TokenCategory::Punctuation
        )
    }
}
impl TokenCategory {
    /// Whether this category is meaningful for formatting (not Eof/Other).
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_meaningful(&self) -> bool {
        !matches!(self, TokenCategory::Eof | TokenCategory::Other)
    }
    /// Return an ANSI color code for this category.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn ansi_color(&self) -> &'static str {
        match self {
            TokenCategory::Keyword => ansi::BOLD_BLUE,
            TokenCategory::Identifier => ansi::RESET,
            TokenCategory::Literal => ansi::BOLD_GREEN,
            TokenCategory::Operator => ansi::CYAN,
            TokenCategory::Punctuation => ansi::YELLOW,
            TokenCategory::Comment => ansi::GREEN,
            TokenCategory::Eof => ansi::RESET,
            TokenCategory::Other => ansi::RESET,
        }
    }
    /// All categories in canonical order.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn all() -> &'static [TokenCategory] {
        &[
            TokenCategory::Keyword,
            TokenCategory::Identifier,
            TokenCategory::Literal,
            TokenCategory::Punctuation,
            TokenCategory::Operator,
            TokenCategory::Comment,
            TokenCategory::Eof,
            TokenCategory::Other,
        ]
    }
}
/// A priority level for operator tokens.
///
/// Higher values bind more tightly (e.g., `*` before `+`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(missing_docs)]
pub struct OperatorPriority(pub u32);
impl OperatorPriority {
    /// Create a priority.
    pub fn new(p: u32) -> Self {
        Self(p)
    }
    /// Lowest possible priority.
    #[allow(missing_docs)]
    pub const MIN: Self = Self(0);
    /// Highest possible priority.
    pub const MAX: Self = Self(u32::MAX);
}
/// A classified token enriched with operator metadata.
#[derive(Clone, Debug)]
#[allow(missing_docs)]
pub struct RichToken {
    /// Underlying token.
    pub token: Token,
    /// High-level category.
    pub category: TokenCategory,
    /// Operator arity (if applicable).
    #[allow(missing_docs)]
    pub arity: OperatorArity,
    /// Operator priority (if applicable).
    pub priority: OperatorPriority,
}
impl RichToken {
    /// Create a `RichToken` from a plain `Token`.
    #[allow(missing_docs)]
    pub fn from_token(token: Token) -> Self {
        let category = categorise(&token.kind);
        let arity = operator_arity(&token.kind);
        let priority = operator_priority(&token.kind);
        Self {
            token,
            category,
            arity,
            priority,
        }
    }
    /// Return `true` if this is an infix binary operator.
    #[allow(missing_docs)]
    pub fn is_infix(&self) -> bool {
        self.arity == OperatorArity::Binary
    }
    /// Return `true` if this is a prefix unary operator.
    #[allow(missing_docs)]
    pub fn is_prefix(&self) -> bool {
        self.arity == OperatorArity::Unary
    }
}
/// Options for token stream reformatting.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct ReformatOptions {
    /// Insert a space before operators.
    pub space_before_op: bool,
    /// Insert a space after operators.
    pub space_after_op: bool,
    /// Insert a space after commas.
    #[allow(missing_docs)]
    pub space_after_comma: bool,
    /// Remove spaces before closing brackets.
    pub no_space_before_close: bool,
}
/// A peekable stream of tokens with span tracking.
#[derive(Clone, Debug)]
#[allow(missing_docs)]
pub struct TokenStream {
    tokens: Vec<Token>,
    pos: usize,
}
impl TokenStream {
    /// Create a stream from a token list.
    #[allow(missing_docs)]
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }
    /// Peek at the current token without consuming it.
    #[allow(missing_docs)]
    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }
    /// Peek at the token `n` positions ahead without consuming.
    #[allow(missing_docs)]
    pub fn peek_ahead(&self, n: usize) -> Option<&Token> {
        self.tokens.get(self.pos + n)
    }
    /// Consume and return the next token.
    #[allow(clippy::should_implement_trait)]
    #[allow(missing_docs)]
    pub fn next(&mut self) -> Option<Token> {
        if self.pos < self.tokens.len() {
            let tok = self.tokens[self.pos].clone();
            self.pos += 1;
            Some(tok)
        } else {
            None
        }
    }
    /// Consume the next token only if its kind matches `expected`.
    #[allow(missing_docs)]
    pub fn eat(&mut self, expected: &TokenKind) -> Option<Token> {
        if self.peek().map(|t| &t.kind) == Some(expected) {
            self.next()
        } else {
            None
        }
    }
    /// Consume tokens while the predicate holds.
    #[allow(missing_docs)]
    pub fn eat_while<F>(&mut self, mut pred: F) -> Vec<Token>
    where
        F: FnMut(&Token) -> bool,
    {
        let mut consumed = Vec::new();
        while let Some(tok) = self.peek() {
            if pred(tok) {
                consumed.push(self.next().expect("peek confirmed token exists"));
            } else {
                break;
            }
        }
        consumed
    }
    /// Return the current position in the stream (number consumed).
    #[allow(missing_docs)]
    pub fn position(&self) -> usize {
        self.pos
    }
    /// `true` if all tokens have been consumed.
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.pos >= self.tokens.len()
    }
    /// Remaining token count.
    #[allow(missing_docs)]
    pub fn remaining(&self) -> usize {
        self.tokens.len().saturating_sub(self.pos)
    }
    /// Rewind to a previously saved position.
    #[allow(missing_docs)]
    pub fn rewind(&mut self, saved: usize) {
        self.pos = saved.min(self.tokens.len());
    }
    /// Save the current position so it can be restored later.
    #[allow(missing_docs)]
    pub fn save(&self) -> usize {
        self.pos
    }
    /// Consume the next token and return an error if the kind is wrong.
    #[allow(missing_docs)]
    pub fn expect(&mut self, expected: &TokenKind) -> Result<Token, String> {
        match self.peek() {
            Some(tok) if &tok.kind == expected => {
                Ok(self.next().expect("peek confirmed token exists"))
            }
            Some(tok) => Err(format!(
                "expected {:?}, got {:?} at {}:{}",
                expected, tok.kind, tok.span.line, tok.span.column
            )),
            None => Err(format!("expected {:?}, got end-of-file", expected)),
        }
    }
    /// Collect all remaining tokens as a `Vec`.
    #[allow(missing_docs)]
    pub fn collect_remaining(mut self) -> Vec<Token> {
        let mut result = Vec::new();
        while let Some(t) = self.next() {
            result.push(t);
        }
        result
    }
}
impl TokenStream {
    /// Peek at the `n`-th token from the current position (0 = next).
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn look_ahead(&self, n: usize) -> Option<&Token> {
        self.tokens.get(self.pos + n)
    }
    /// Consume and discard tokens while `pred` holds.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn skip_while<F: FnMut(&Token) -> bool>(&mut self, mut pred: F) {
        while let Some(tok) = self.peek() {
            if pred(tok) {
                self.pos += 1;
            } else {
                break;
            }
        }
    }
    /// Consume tokens up to and including the next occurrence of `kind`.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn skip_to_inclusive(&mut self, kind: &TokenKind) {
        while let Some(tok) = self.peek() {
            let found = &tok.kind == kind;
            self.pos += 1;
            if found {
                break;
            }
        }
    }
    /// Consume tokens up to (but not including) the next occurrence of `kind`.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn skip_to(&mut self, kind: &TokenKind) {
        while let Some(tok) = self.peek() {
            if &tok.kind == kind {
                break;
            }
            self.pos += 1;
        }
    }
    /// Return a slice of the next `n` tokens without consuming them.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn peek_slice(&self, n: usize) -> &[Token] {
        let end = (self.pos + n).min(self.tokens.len());
        &self.tokens[self.pos..end]
    }
    /// Check if the next `n` tokens match the given kind sequence.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn matches_sequence(&self, kinds: &[&TokenKind]) -> bool {
        for (i, k) in kinds.iter().enumerate() {
            match self.tokens.get(self.pos + i) {
                Some(tok) if &&tok.kind == k => {}
                _ => return false,
            }
        }
        true
    }
    /// Consume exactly `n` tokens and return them.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn consume_n(&mut self, n: usize) -> Vec<Token> {
        let mut result = Vec::with_capacity(n);
        for _ in 0..n {
            if let Some(tok) = self.next() {
                result.push(tok);
            }
        }
        result
    }
    /// Peek at all remaining tokens.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn peek_all(&self) -> &[Token] {
        &self.tokens[self.pos..]
    }
    /// Insert tokens at the current position (for synthetic token injection).
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn inject(&mut self, tokens: Vec<Token>) {
        let mut new_tokens = self.tokens[..self.pos].to_vec();
        new_tokens.extend(tokens);
        new_tokens.extend_from_slice(&self.tokens[self.pos..]);
        self.tokens = new_tokens;
    }
    /// Return the total token count (including consumed).
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn len(&self) -> usize {
        self.tokens.len()
    }
}
/// A window iterator that yields consecutive N-grams of tokens.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct TokenNgramIter<'a> {
    pub(crate) tokens: &'a [Token],
    pub(crate) window: usize,
    pub(crate) pos: usize,
}
impl<'a> TokenNgramIter<'a> {
    /// Create a new N-gram iterator with the given window size.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(tokens: &'a [Token], window: usize) -> Self {
        Self {
            tokens,
            window,
            pos: 0,
        }
    }
}
/// Matching bracket pairs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum BracketKind {
    /// `(` / `)`
    Paren,
    /// `{` / `}`
    Brace,
    /// `[` / `]`
    Bracket,
}
/// A simple pattern for matching token sequences.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub enum TokenPattern {
    /// Match a specific token kind.
    Exact(TokenKind),
    /// Match any token in the given category.
    Category(TokenCategory),
    /// Match any token (wildcard).
    Any,
    /// Match an optional token.
    Optional(Box<TokenPattern>),
    /// Match a sequence of patterns.
    Sequence(Vec<TokenPattern>),
    /// Match one of the alternatives.
    Alternatives(Vec<TokenPattern>),
}
impl TokenPattern {
    /// Check if a single token matches this pattern (non-recursive).
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn matches_single(&self, tok: &Token) -> bool {
        match self {
            TokenPattern::Exact(k) => &tok.kind == k,
            TokenPattern::Category(cat) => categorise(&tok.kind) == *cat,
            TokenPattern::Any => true,
            TokenPattern::Optional(inner) => inner.matches_single(tok),
            TokenPattern::Sequence(_) => false,
            TokenPattern::Alternatives(alts) => alts.iter().any(|a| a.matches_single(tok)),
        }
    }
    /// Try to match this pattern against the start of a token slice.
    /// Returns the number of tokens consumed, or `None` if no match.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn try_match(&self, tokens: &[Token]) -> Option<usize> {
        match self {
            TokenPattern::Exact(k) => {
                if tokens.first().map(|t| &t.kind) == Some(k) {
                    Some(1)
                } else {
                    None
                }
            }
            TokenPattern::Category(cat) => {
                if tokens.first().map(|t| categorise(&t.kind)) == Some(*cat) {
                    Some(1)
                } else {
                    None
                }
            }
            TokenPattern::Any => {
                if tokens.is_empty() {
                    None
                } else {
                    Some(1)
                }
            }
            TokenPattern::Optional(inner) => Some(inner.try_match(tokens).unwrap_or(0)),
            TokenPattern::Sequence(pats) => {
                let mut consumed = 0;
                for pat in pats {
                    match pat.try_match(&tokens[consumed..]) {
                        Some(n) => consumed += n,
                        None => return None,
                    }
                }
                Some(consumed)
            }
            TokenPattern::Alternatives(alts) => {
                for alt in alts {
                    if let Some(n) = alt.try_match(tokens) {
                        return Some(n);
                    }
                }
                None
            }
        }
    }
    /// Find all non-overlapping matches of this pattern in a slice.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn find_all<'a>(&self, tokens: &'a [Token]) -> Vec<&'a [Token]> {
        let mut results = Vec::new();
        let mut pos = 0;
        while pos < tokens.len() {
            if let Some(n) = self.try_match(&tokens[pos..]) {
                if n > 0 {
                    results.push(&tokens[pos..pos + n]);
                    pos += n;
                } else {
                    pos += 1;
                }
            } else {
                pos += 1;
            }
        }
        results
    }
}
/// A token annotated with its depth and category.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct AnnotatedToken {
    pub token: Token,
    pub category: TokenCategory,
    pub depth: i32,
    pub index: usize,
}
impl AnnotatedToken {
    /// Create an annotated token.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(token: Token, depth: i32, index: usize) -> Self {
        let category = categorise(&token.kind);
        Self {
            token,
            category,
            depth,
            index,
        }
    }
}
/// Classify a TokenKind as unary, binary, or non-operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum OperatorArity {
    /// A prefix operator (e.g., `-`, `¬`).
    Unary,
    /// An infix operator (e.g., `+`, `*`, `→`).
    Binary,
    /// Not an operator.
    None,
}
