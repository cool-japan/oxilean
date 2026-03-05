//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

/// A token with optional semantic annotations.
use super::functions::*;
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AnnotatedToken {
    /// The underlying token.
    pub token: Token,
    /// Zero or more annotations.
    pub annotations: Vec<TokenAnnotation>,
}
#[allow(dead_code)]
impl AnnotatedToken {
    /// Create an unannotated token.
    pub fn plain(token: Token) -> Self {
        Self {
            token,
            annotations: Vec::new(),
        }
    }
    /// Add an annotation.
    pub fn annotate(&mut self, ann: TokenAnnotation) {
        self.annotations.push(ann);
    }
    /// Check if the token has a given annotation.
    pub fn has_annotation(&self, ann: &TokenAnnotation) -> bool {
        self.annotations.contains(ann)
    }
    /// Check if the token is a binding site.
    pub fn is_binding_site(&self) -> bool {
        self.annotations
            .iter()
            .any(|a| matches!(a, TokenAnnotation::BindingSite(_)))
    }
    /// Check if the token is a use site.
    pub fn is_use_site(&self) -> bool {
        self.annotations
            .iter()
            .any(|a| matches!(a, TokenAnnotation::UseSite(_)))
    }
    /// Get the resolved name, if any.
    pub fn resolved_name(&self) -> Option<&str> {
        self.annotations.iter().find_map(|a| {
            if let TokenAnnotation::ResolvedName(s) = a {
                Some(s.as_str())
            } else {
                None
            }
        })
    }
}
/// The broad role of a token in OxiLean syntax.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenRole {
    /// A declaration-starting keyword.
    DeclStart,
    /// A proof/tactic keyword.
    TacticKeyword,
    /// A type-level keyword.
    TypeLevel,
    /// A term-level keyword.
    TermLevel,
    /// A punctuation/operator token.
    Punctuation,
    /// A name or identifier.
    Name,
    /// A literal value.
    Literal,
    /// An end-of-input marker.
    Eof,
    /// An error token.
    Error,
}
/// Statistics about a token sequence.
#[derive(Debug, Clone, Default)]
pub struct TokenStats {
    /// Total number of tokens.
    pub total: usize,
    /// Number of identifiers.
    pub identifiers: usize,
    /// Number of keyword tokens.
    pub keywords: usize,
    /// Number of numeric literals.
    pub nat_literals: usize,
    /// Number of string literals.
    pub string_literals: usize,
    /// Number of operators.
    pub operators: usize,
    /// Number of error tokens.
    pub errors: usize,
    /// Number of doc comments.
    pub doc_comments: usize,
}
impl TokenStats {
    /// Compute statistics for a slice of tokens.
    pub fn compute(tokens: &[Token]) -> Self {
        let mut stats = Self {
            total: tokens.len(),
            ..Default::default()
        };
        for tok in tokens {
            match &tok.kind {
                TokenKind::Ident(_) => stats.identifiers += 1,
                TokenKind::Nat(_) => stats.nat_literals += 1,
                TokenKind::String(_) => stats.string_literals += 1,
                TokenKind::Error(_) => stats.errors += 1,
                TokenKind::DocComment(_) => stats.doc_comments += 1,
                k if Token {
                    kind: k.clone(),
                    span: tok.span.clone(),
                }
                .is_keyword() =>
                {
                    stats.keywords += 1
                }
                k if is_arithmetic_op(k) || is_comparison_op(k) || is_logical_op(k) => {
                    stats.operators += 1;
                }
                _ => {}
            }
        }
        stats
    }
    /// Check if there are any error tokens.
    pub fn has_errors(&self) -> bool {
        self.errors > 0
    }
}
/// An iterator over a sequence of tokens.
///
/// Supports peeking, consuming, and backtracking (via snapshots).
pub struct TokenStream {
    /// The underlying token sequence.
    tokens: Vec<Token>,
    /// Current read position.
    pos: usize,
}
impl TokenStream {
    /// Create a new token stream from a vector of tokens.
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }
    /// Peek at the current token without consuming it.
    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }
    /// Peek at the token `n` positions ahead (0 = current).
    pub fn peek_ahead(&self, n: usize) -> Option<&Token> {
        self.tokens.get(self.pos + n)
    }
    /// Get the current token kind without consuming.
    pub fn current_kind(&self) -> Option<&TokenKind> {
        self.tokens.get(self.pos).map(|t| &t.kind)
    }
    /// Check if the current token matches a given kind (by tag, ignoring data).
    pub fn at_keyword(&self, kw: &TokenKind) -> bool {
        self.tokens
            .get(self.pos)
            .is_some_and(|t| std::mem::discriminant(&t.kind) == std::mem::discriminant(kw))
    }
    /// Check if the current token is an identifier with a specific name.
    pub fn at_ident(&self, name: &str) -> bool {
        matches!(
            self.tokens.get(self.pos).map(| t | & t.kind), Some(TokenKind::Ident(s)) if s
            == name
        )
    }
    /// Check if we are at the end of the stream.
    pub fn is_eof(&self) -> bool {
        self.pos >= self.tokens.len()
            || matches!(
                self.tokens.get(self.pos).map(|t| &t.kind),
                Some(TokenKind::Eof)
            )
    }
    /// Consume and return the current token.
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&Token> {
        if self.pos < self.tokens.len() {
            let tok = &self.tokens[self.pos];
            self.pos += 1;
            Some(tok)
        } else {
            None
        }
    }
    /// Consume the current token if it matches the given kind.
    ///
    /// Returns `true` if consumed, `false` otherwise.
    pub fn consume_if(&mut self, kind: &TokenKind) -> bool {
        if self.tokens.get(self.pos).is_some_and(|t| &t.kind == kind) {
            self.pos += 1;
            true
        } else {
            false
        }
    }
    /// Expect a specific token kind; return an error string if it doesn't match.
    pub fn expect(&mut self, kind: &TokenKind) -> Result<&Token, String> {
        match self.tokens.get(self.pos) {
            Some(t) if &t.kind == kind => {
                self.pos += 1;
                Ok(&self.tokens[self.pos - 1])
            }
            Some(t) => Err(format!(
                "Expected {} but got {} at line {}",
                kind, t.kind, t.span.line
            )),
            None => Err(format!("Expected {} but got end of file", kind)),
        }
    }
    /// Expect an identifier and return its name.
    pub fn expect_ident(&mut self) -> Result<String, String> {
        match self.tokens.get(self.pos) {
            Some(Token {
                kind: TokenKind::Ident(s),
                ..
            }) => {
                let name = s.clone();
                self.pos += 1;
                Ok(name)
            }
            Some(t) => Err(format!(
                "Expected identifier but got {} at line {}",
                t.kind, t.span.line
            )),
            None => Err("Expected identifier but got end of file".to_string()),
        }
    }
    /// Take a snapshot of the current position (for backtracking).
    pub fn snapshot(&self) -> usize {
        self.pos
    }
    /// Restore to a previous snapshot position.
    pub fn restore(&mut self, snap: usize) {
        self.pos = snap;
    }
    /// Get the remaining token count.
    pub fn remaining(&self) -> usize {
        self.tokens.len().saturating_sub(self.pos)
    }
    /// Get the current span (or a dummy span if at EOF).
    pub fn current_span(&self) -> Span {
        self.tokens
            .get(self.pos)
            .map(|t| t.span.clone())
            .unwrap_or_else(|| Span::new(0, 0, 1, 1))
    }
    /// Skip tokens until we hit one of the given token kinds (for error recovery).
    pub fn skip_until(&mut self, kinds: &[TokenKind]) {
        while let Some(t) = self.tokens.get(self.pos) {
            if kinds
                .iter()
                .any(|k| std::mem::discriminant(&t.kind) == std::mem::discriminant(k))
            {
                break;
            }
            self.pos += 1;
        }
    }
    /// Collect all tokens from the current position to the end.
    pub fn drain_remaining(&mut self) -> Vec<Token> {
        let rest = self.tokens[self.pos..].to_vec();
        self.pos = self.tokens.len();
        rest
    }
    /// Get the total number of tokens (including EOF).
    pub fn total_len(&self) -> usize {
        self.tokens.len()
    }
}
/// A token with location information.
#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    /// Token kind
    pub kind: TokenKind,
    /// Source span
    pub span: Span,
}
impl Token {
    /// Create a new token.
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
    /// Check if this token is an identifier.
    pub fn is_ident(&self) -> bool {
        matches!(self.kind, TokenKind::Ident(_))
    }
    /// Get the identifier name if this is an identifier.
    pub fn as_ident(&self) -> Option<&str> {
        match &self.kind {
            TokenKind::Ident(s) => Some(s),
            _ => None,
        }
    }
    /// Check if this token is a keyword.
    pub fn is_keyword(&self) -> bool {
        matches!(
            self.kind,
            TokenKind::Axiom
                | TokenKind::Definition
                | TokenKind::Theorem
                | TokenKind::Lemma
                | TokenKind::Fun
                | TokenKind::Forall
                | TokenKind::Let
                | TokenKind::In
                | TokenKind::If
                | TokenKind::Then
                | TokenKind::Else
                | TokenKind::Match
                | TokenKind::With
                | TokenKind::Do
                | TokenKind::Have
                | TokenKind::Suffices
                | TokenKind::Show
                | TokenKind::Where
                | TokenKind::From
                | TokenKind::By
                | TokenKind::Return
        )
    }
}
/// A cursor for reading tokens with position tracking and backtracking.
#[allow(dead_code)]
pub struct TokenCursor<'a> {
    tokens: &'a [Token],
    pos: usize,
    /// Saved positions for backtracking.
    checkpoints: Vec<usize>,
}
#[allow(dead_code)]
impl<'a> TokenCursor<'a> {
    /// Create a cursor over a slice of tokens.
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens,
            pos: 0,
            checkpoints: Vec::new(),
        }
    }
    /// Peek at the current token.
    pub fn peek(&self) -> Option<&'a Token> {
        self.tokens.get(self.pos)
    }
    /// Peek `n` positions ahead.
    pub fn peek_n(&self, n: usize) -> Option<&'a Token> {
        self.tokens.get(self.pos + n)
    }
    /// Consume the current token.
    pub fn advance(&mut self) -> Option<&'a Token> {
        if self.pos < self.tokens.len() {
            let tok = &self.tokens[self.pos];
            self.pos += 1;
            Some(tok)
        } else {
            None
        }
    }
    /// Current position.
    pub fn position(&self) -> usize {
        self.pos
    }
    /// Remaining tokens.
    pub fn remaining(&self) -> usize {
        self.tokens.len().saturating_sub(self.pos)
    }
    /// Whether we are at end of input.
    pub fn is_eof(&self) -> bool {
        self.pos >= self.tokens.len()
    }
    /// Save current position as a checkpoint.
    pub fn save(&mut self) {
        self.checkpoints.push(self.pos);
    }
    /// Restore the most recent checkpoint.
    pub fn restore(&mut self) {
        if let Some(saved) = self.checkpoints.pop() {
            self.pos = saved;
        }
    }
    /// Discard the most recent checkpoint (commit the advance).
    pub fn commit(&mut self) {
        self.checkpoints.pop();
    }
    /// Consume if the current token kind matches.
    pub fn try_consume(&mut self, kind: &TokenKind) -> bool {
        if self.tokens.get(self.pos).is_some_and(|t| &t.kind == kind) {
            self.pos += 1;
            true
        } else {
            false
        }
    }
    /// Expect the current kind matches, else return an error string.
    pub fn expect_kind(&mut self, kind: &TokenKind) -> Result<&'a Token, String> {
        match self.tokens.get(self.pos) {
            Some(t) if &t.kind == kind => {
                self.pos += 1;
                Ok(&self.tokens[self.pos - 1])
            }
            Some(t) => Err(format!("expected {:?} got {:?}", kind, t.kind)),
            None => Err(format!("expected {:?} got EOF", kind)),
        }
    }
    /// Get a slice of the remaining tokens.
    pub fn remaining_slice(&self) -> &'a [Token] {
        &self.tokens[self.pos..]
    }
    /// Collect tokens until `pred` returns `true`.
    pub fn collect_until<F: Fn(&Token) -> bool>(&mut self, pred: F) -> Vec<&'a Token> {
        let mut result = Vec::new();
        while let Some(tok) = self.tokens.get(self.pos) {
            if pred(tok) {
                break;
            }
            result.push(tok);
            self.pos += 1;
        }
        result
    }
    /// Skip tokens as long as `pred` returns `true`.
    pub fn skip_while<F: Fn(&Token) -> bool>(&mut self, pred: F) {
        while self.tokens.get(self.pos).is_some_and(&pred) {
            self.pos += 1;
        }
    }
}
/// A set of token kind discriminants for fast membership testing.
#[allow(dead_code)]
pub struct TokenKindSet {
    /// The included kinds (compared by discriminant).
    kinds: Vec<std::mem::Discriminant<TokenKind>>,
}
#[allow(dead_code)]
impl TokenKindSet {
    /// Create an empty set.
    pub fn new() -> Self {
        Self { kinds: Vec::new() }
    }
    /// Add a kind to the set.
    pub fn add(&mut self, kind: &TokenKind) {
        let d = std::mem::discriminant(kind);
        if !self.kinds.contains(&d) {
            self.kinds.push(d);
        }
    }
    /// Check if the set contains a kind.
    pub fn contains(&self, kind: &TokenKind) -> bool {
        self.kinds.contains(&std::mem::discriminant(kind))
    }
    /// Number of kinds in the set.
    pub fn len(&self) -> usize {
        self.kinds.len()
    }
    /// Whether the set is empty.
    pub fn is_empty(&self) -> bool {
        self.kinds.is_empty()
    }
}
/// A contiguous range of tokens with a combined span.
#[allow(dead_code)]
pub struct TokenRange {
    /// The tokens in this range.
    pub tokens: Vec<Token>,
    /// Combined span covering all tokens.
    pub span: Span,
}
#[allow(dead_code)]
impl TokenRange {
    /// Create a token range from a vector of tokens.
    pub fn from_tokens(tokens: Vec<Token>) -> Option<Self> {
        if tokens.is_empty() {
            return None;
        }
        let first = tokens
            .first()
            .expect("tokens non-empty per is_empty check above");
        let last = tokens
            .last()
            .expect("tokens non-empty per is_empty check above");
        let span = first.span.merge(&last.span);
        Some(Self { tokens, span })
    }
    /// Number of tokens.
    pub fn len(&self) -> usize {
        self.tokens.len()
    }
    /// Whether the range is empty.
    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }
    /// Get a token by index.
    pub fn get(&self, i: usize) -> Option<&Token> {
        self.tokens.get(i)
    }
    /// Iterate over the tokens.
    pub fn iter(&self) -> std::slice::Iter<'_, Token> {
        self.tokens.iter()
    }
}
/// Token kind.
#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    /// Axiom keyword
    Axiom,
    /// Definition keyword
    Definition,
    /// Theorem keyword
    Theorem,
    /// Lemma keyword
    Lemma,
    /// Opaque keyword
    Opaque,
    /// Inductive keyword
    Inductive,
    /// Structure keyword
    Structure,
    /// Class keyword
    Class,
    /// Instance keyword
    Instance,
    /// Namespace keyword
    Namespace,
    /// Section keyword
    Section,
    /// Variable keyword
    Variable,
    /// Variables keyword
    Variables,
    /// Parameter keyword
    Parameter,
    /// Parameters keyword
    Parameters,
    /// Constant keyword
    Constant,
    /// Constants keyword
    Constants,
    /// End keyword
    End,
    /// Import keyword
    Import,
    /// Export keyword
    Export,
    /// Open keyword
    Open,
    /// Attribute keyword
    Attribute,
    /// Return keyword
    Return,
    /// Type keyword
    Type,
    /// Prop keyword
    Prop,
    /// Sort keyword
    Sort,
    /// Fun or lambda (lambda)
    Fun,
    /// Forall
    Forall,
    /// Let keyword
    Let,
    /// In keyword
    In,
    /// If keyword
    If,
    /// Then keyword
    Then,
    /// Else keyword
    Else,
    /// Match keyword
    Match,
    /// With keyword
    With,
    /// Do keyword
    Do,
    /// Have keyword
    Have,
    /// Suffices keyword
    Suffices,
    /// Show keyword
    Show,
    /// Where keyword
    Where,
    /// From keyword
    From,
    /// By keyword
    By,
    /// Arrow ->
    Arrow,
    /// Fat arrow =>
    FatArrow,
    /// And
    And,
    /// Or
    Or,
    /// Not
    Not,
    /// Iff
    Iff,
    /// Exists
    Exists,
    /// Equality =
    Eq,
    /// Not equal
    Ne,
    /// Less than <
    Lt,
    /// Less than or equal
    Le,
    /// Greater than >
    Gt,
    /// Greater than or equal
    Ge,
    /// Left parenthesis (
    LParen,
    /// Right parenthesis )
    RParen,
    /// Left brace {
    LBrace,
    /// Right brace }
    RBrace,
    /// Left bracket [
    LBracket,
    /// Right bracket ]
    RBracket,
    /// Left angle bracket (for anonymous ctor)
    LAngle,
    /// Right angle bracket (for anonymous ctor)
    RAngle,
    /// Comma ,
    Comma,
    /// Colon :
    Colon,
    /// Semicolon ;
    Semicolon,
    /// Dot .
    Dot,
    /// Dot dot ..
    DotDot,
    /// Bar |
    Bar,
    /// At @
    At,
    /// Hash #
    Hash,
    /// Plus +
    Plus,
    /// Minus -
    Minus,
    /// Star *
    Star,
    /// Slash /
    Slash,
    /// Percent %
    Percent,
    /// Caret ^
    Caret,
    /// Assignment :=
    Assign,
    /// Double ampersand &&
    AndAnd,
    /// Double pipe ||
    OrOr,
    /// Not equal !=
    BangEq,
    /// Bang !
    Bang,
    /// Left arrow <-
    LeftArrow,
    /// Identifier
    Ident(String),
    /// Natural number literal
    Nat(u64),
    /// Float literal
    Float(f64),
    /// String literal
    String(String),
    /// Character literal
    Char(char),
    /// Doc comment (block `/-- ... -/` or line `--- ...`)
    DocComment(String),
    /// Interpolated string: `s!"hello {name}"`
    InterpolatedString(Vec<StringPart>),
    /// Underscore _
    Underscore,
    /// Question mark ?
    Question,
    /// End of file
    Eof,
    /// Error token
    Error(String),
}
/// Extra semantic information attached to a token.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenAnnotation {
    /// This token is an introduction site for the named variable.
    BindingSite(String),
    /// This token is a use-site for the named variable.
    UseSite(String),
    /// This token's identifier refers to a declaration with this qualified name.
    ResolvedName(String),
    /// This token starts a tactic block.
    TacticStart,
    /// This token ends a tactic block.
    TacticEnd,
    /// This token is a hole (implicit argument).
    ImplicitHole,
    /// This token is a type annotation separator.
    TypeSeparator,
}
/// Source location information.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Span {
    /// Start position (byte offset)
    pub start: usize,
    /// End position (byte offset)
    pub end: usize,
    /// Line number (1-indexed)
    pub line: usize,
    /// Column number (1-indexed)
    pub column: usize,
}
impl Span {
    /// Create a new span.
    pub fn new(start: usize, end: usize, line: usize, column: usize) -> Self {
        Self {
            start,
            end,
            line,
            column,
        }
    }
    /// Create a span that covers both spans.
    pub fn merge(&self, other: &Span) -> Span {
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
            line: self.line,
            column: self.column,
        }
    }
}
/// A matcher that finds the closing bracket/delimiter for an opening one.
#[allow(dead_code)]
pub struct TokenPairMatcher;
#[allow(dead_code)]
impl TokenPairMatcher {
    /// Return the closing counterpart of an opening token kind.
    pub fn closing_of(kind: &TokenKind) -> Option<TokenKind> {
        match kind {
            TokenKind::LParen => Some(TokenKind::RParen),
            TokenKind::LBrace => Some(TokenKind::RBrace),
            TokenKind::LBracket => Some(TokenKind::RBracket),
            TokenKind::LAngle => Some(TokenKind::RAngle),
            _ => None,
        }
    }
    /// Return the opening counterpart of a closing token kind.
    pub fn opening_of(kind: &TokenKind) -> Option<TokenKind> {
        match kind {
            TokenKind::RParen => Some(TokenKind::LParen),
            TokenKind::RBrace => Some(TokenKind::LBrace),
            TokenKind::RBracket => Some(TokenKind::LBracket),
            TokenKind::RAngle => Some(TokenKind::LAngle),
            _ => None,
        }
    }
    /// Find the index of the closing token matching the opener at `start`.
    ///
    /// Returns `None` if no balanced closer is found.
    pub fn find_closing(tokens: &[Token], start: usize) -> Option<usize> {
        let opener = &tokens.get(start)?.kind;
        let closer = Self::closing_of(opener)?;
        let mut depth = 0usize;
        for (i, tok) in tokens[start..].iter().enumerate() {
            if std::mem::discriminant(&tok.kind) == std::mem::discriminant(opener) {
                depth += 1;
            } else if std::mem::discriminant(&tok.kind) == std::mem::discriminant(&closer) {
                depth -= 1;
                if depth == 0 {
                    return Some(start + i);
                }
            }
        }
        None
    }
    /// Extract the balanced group starting at `start`, returning the inner tokens.
    pub fn extract_group(tokens: &[Token], start: usize) -> Option<&[Token]> {
        let end = Self::find_closing(tokens, start)?;
        Some(&tokens[start + 1..end])
    }
}
/// A token together with optional preceding and following tokens.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ContextualToken {
    /// The main token.
    pub token: Token,
    /// The immediately preceding token (if any).
    pub prev: Option<Token>,
    /// The immediately following token (if any).
    pub next: Option<Token>,
}
#[allow(dead_code)]
impl ContextualToken {
    /// Create a contextual token.
    pub fn new(token: Token, prev: Option<Token>, next: Option<Token>) -> Self {
        Self { token, prev, next }
    }
    /// Build contextual tokens from a token slice.
    pub fn from_slice(tokens: &[Token]) -> Vec<ContextualToken> {
        tokens
            .iter()
            .enumerate()
            .map(|(i, tok)| {
                let prev = if i > 0 {
                    Some(tokens[i - 1].clone())
                } else {
                    None
                };
                let next = tokens.get(i + 1).cloned();
                ContextualToken::new(tok.clone(), prev, next)
            })
            .collect()
    }
    /// Whether this token starts a line (column 1).
    pub fn is_line_start(&self) -> bool {
        self.token.span.column == 1
    }
    /// Whether the previous and current token are on the same line.
    pub fn same_line_as_prev(&self) -> bool {
        self.prev
            .as_ref()
            .is_some_and(|p| p.span.line == self.token.span.line)
    }
    /// Whether the current and next token are on the same line.
    pub fn same_line_as_next(&self) -> bool {
        self.next
            .as_ref()
            .is_some_and(|n| n.span.line == self.token.span.line)
    }
}
/// A part of an interpolated string.
#[derive(Clone, Debug, PartialEq)]
pub enum StringPart {
    /// A literal text segment.
    Literal(String),
    /// An interpolation hole containing tokens.
    Interpolation(Vec<Token>),
}
/// A growable buffer of tokens that supports efficient push and random access.
#[allow(dead_code)]
pub struct TokenBuffer {
    tokens: Vec<Token>,
}
#[allow(dead_code)]
impl TokenBuffer {
    /// Create an empty token buffer.
    pub fn new() -> Self {
        Self { tokens: Vec::new() }
    }
    /// Create a buffer with pre-allocated capacity.
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            tokens: Vec::with_capacity(cap),
        }
    }
    /// Push a token onto the buffer.
    pub fn push(&mut self, token: Token) {
        self.tokens.push(token);
    }
    /// Get a reference to the token at position `i`.
    pub fn get(&self, i: usize) -> Option<&Token> {
        self.tokens.get(i)
    }
    /// The number of tokens in the buffer.
    pub fn len(&self) -> usize {
        self.tokens.len()
    }
    /// Whether the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }
    /// Consume the buffer into a `Vec<Token>`.
    pub fn into_tokens(self) -> Vec<Token> {
        self.tokens
    }
    /// Drain the buffer into a `TokenStream`.
    pub fn into_stream(self) -> TokenStream {
        TokenStream::new(self.tokens)
    }
    /// Append all tokens from another buffer.
    pub fn extend_from(&mut self, other: TokenBuffer) {
        self.tokens.extend(other.tokens);
    }
    /// Truncate to the first `n` tokens.
    pub fn truncate(&mut self, n: usize) {
        self.tokens.truncate(n);
    }
    /// Get the last token in the buffer.
    pub fn last(&self) -> Option<&Token> {
        self.tokens.last()
    }
    /// Retain only tokens that satisfy `pred`.
    pub fn retain<F: FnMut(&Token) -> bool>(&mut self, pred: F) {
        self.tokens.retain(pred);
    }
    /// Split the buffer at position `mid`, returning the second half.
    pub fn split_off(&mut self, mid: usize) -> TokenBuffer {
        TokenBuffer {
            tokens: self.tokens.split_off(mid),
        }
    }
    /// Count tokens of a given kind discriminant.
    pub fn count_kind(&self, kind: &TokenKind) -> usize {
        count_tokens(&self.tokens, kind)
    }
    /// Find the first position of a token matching a predicate.
    pub fn find_first<F: Fn(&Token) -> bool>(&self, pred: F) -> Option<usize> {
        self.tokens.iter().position(pred)
    }
    /// Slice a range from the buffer.
    pub fn slice(&self, start: usize, end: usize) -> &[Token] {
        &self.tokens[start..end.min(self.tokens.len())]
    }
}
