//! # CommandParser - parsing Methods
//!
//! This module contains method implementations for `CommandParser`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::{
    dummy_span, parser_impl::Parser as DeclParser, Binder, BinderKind, Decl, Located, ParseError,
    ParseErrorKind, Span, Token, TokenKind,
};

use super::types::{AttributeDeclKind, Command, NotationKind, OpenItem, StructureField};

use super::commandparser_type::CommandParser;

impl CommandParser {
    /// Create a new command parser.
    pub fn new() -> Self {
        Self {
            pos: 0,
            tokens: Vec::new(),
        }
    }
    /// Expect a specific token kind; error if not matched.
    pub(super) fn expect(&mut self, kind: &TokenKind) -> Result<Span, ParseError> {
        if let Some(token) = self.current() {
            if &token.kind == kind {
                let span = token.span.clone();
                self.advance();
                return Ok(span);
            }
            return Err(ParseError::new(
                ParseErrorKind::InvalidSyntax(format!("expected {:?}, got {:?}", kind, token.kind)),
                token.span.clone(),
            ));
        }
        Err(ParseError::new(
            ParseErrorKind::UnexpectedEof {
                expected: vec![format!("{:?}", kind)],
            },
            self.eof_span(),
        ))
    }
    /// Parse an identifier token and return the string.
    pub(super) fn parse_ident(&mut self) -> Result<String, ParseError> {
        if let Some(token) = self.current() {
            if let TokenKind::Ident(s) = &token.kind {
                let result = s.clone();
                self.advance();
                return Ok(result);
            }
            return Err(ParseError::new(
                ParseErrorKind::InvalidSyntax(format!("expected identifier, got {:?}", token.kind)),
                token.span.clone(),
            ));
        }
        Err(ParseError::new(
            ParseErrorKind::UnexpectedEof {
                expected: vec!["identifier".to_string()],
            },
            self.eof_span(),
        ))
    }
    /// Span for unexpected eof.
    pub(super) fn eof_span(&self) -> Span {
        if let Some(last) = self.tokens.last() {
            Span::new(
                last.span.end,
                last.span.end,
                last.span.line,
                last.span.column + 1,
            )
        } else {
            dummy_span()
        }
    }
    /// Collect text from current position until eof.
    /// Returns the collected tokens joined by spaces.
    pub(super) fn collect_rest_as_string(&mut self) -> String {
        let mut parts = Vec::new();
        while let Some(token) = self.current() {
            match &token.kind {
                TokenKind::Eof => break,
                TokenKind::Ident(s) => parts.push(s.clone()),
                TokenKind::Nat(n) => parts.push(n.to_string()),
                TokenKind::String(s) => parts.push(format!("\"{}\"", s)),
                TokenKind::LParen => parts.push("(".to_string()),
                TokenKind::RParen => parts.push(")".to_string()),
                TokenKind::LBrace => parts.push("{".to_string()),
                TokenKind::RBrace => parts.push("}".to_string()),
                TokenKind::LBracket => parts.push("[".to_string()),
                TokenKind::RBracket => parts.push("]".to_string()),
                TokenKind::Arrow => parts.push("->".to_string()),
                TokenKind::Colon => parts.push(":".to_string()),
                TokenKind::Comma => parts.push(",".to_string()),
                TokenKind::Dot => parts.push(".".to_string()),
                TokenKind::Eq => parts.push("=".to_string()),
                TokenKind::Plus => parts.push("+".to_string()),
                TokenKind::Minus => parts.push("-".to_string()),
                TokenKind::Star => parts.push("*".to_string()),
                TokenKind::Slash => parts.push("/".to_string()),
                TokenKind::Assign => parts.push(":=".to_string()),
                TokenKind::Underscore => parts.push("_".to_string()),
                TokenKind::Bar => parts.push("|".to_string()),
                TokenKind::Semicolon => parts.push(";".to_string()),
                _ => parts.push(format!("{}", token.kind)),
            }
            self.advance();
        }
        parts.join(" ")
    }
    /// Parse binders: `(x : T)`, `{x : T}`, `[x : T]`, or bare `x`.
    pub(super) fn parse_binders(&mut self) -> Result<Vec<Binder>, ParseError> {
        let mut binders = Vec::new();
        loop {
            if self.check(&TokenKind::LParen) {
                self.advance();
                let names = self.parse_binder_names()?;
                let ty = if self.consume(&TokenKind::Colon) {
                    Some(self.collect_type_expr()?)
                } else {
                    None
                };
                self.expect(&TokenKind::RParen)?;
                for name in names {
                    binders.push(Binder {
                        name,
                        ty: ty.clone(),
                        info: BinderKind::Default,
                    });
                }
            } else if self.check(&TokenKind::LBrace) {
                self.advance();
                if self.check(&TokenKind::LBrace) {
                    self.advance();
                    let names = self.parse_binder_names()?;
                    let ty = if self.consume(&TokenKind::Colon) {
                        Some(self.collect_type_expr()?)
                    } else {
                        None
                    };
                    self.expect(&TokenKind::RBrace)?;
                    self.expect(&TokenKind::RBrace)?;
                    for name in names {
                        binders.push(Binder {
                            name,
                            ty: ty.clone(),
                            info: BinderKind::StrictImplicit,
                        });
                    }
                } else {
                    let names = self.parse_binder_names()?;
                    let ty = if self.consume(&TokenKind::Colon) {
                        Some(self.collect_type_expr()?)
                    } else {
                        None
                    };
                    self.expect(&TokenKind::RBrace)?;
                    for name in names {
                        binders.push(Binder {
                            name,
                            ty: ty.clone(),
                            info: BinderKind::Implicit,
                        });
                    }
                }
            } else if self.check(&TokenKind::LBracket) {
                self.advance();
                let names = self.parse_binder_names()?;
                let ty = if self.consume(&TokenKind::Colon) {
                    Some(self.collect_type_expr()?)
                } else {
                    None
                };
                self.expect(&TokenKind::RBracket)?;
                for name in names {
                    binders.push(Binder {
                        name,
                        ty: ty.clone(),
                        info: BinderKind::Instance,
                    });
                }
            } else if let Some(token) = self.current() {
                if let TokenKind::Ident(s) = &token.kind {
                    if Self::is_command_keyword(&token.kind)
                        || self.check(&TokenKind::Colon)
                        || matches!(s.as_str(), "extends" | "where" | "deriving" | "priority")
                    {
                        break;
                    }
                    let name = self.parse_ident()?;
                    binders.push(Binder {
                        name,
                        ty: None,
                        info: BinderKind::Default,
                    });
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        Ok(binders)
    }
    /// Parse one or more binder names (identifiers or _).
    pub(super) fn parse_binder_names(&mut self) -> Result<Vec<String>, ParseError> {
        let mut names = Vec::new();
        loop {
            if self.check(&TokenKind::Underscore) {
                self.advance();
                names.push("_".to_string());
            } else if let Some(token) = self.current() {
                if let TokenKind::Ident(_) = &token.kind {
                    names.push(self.parse_ident()?);
                } else {
                    break;
                }
            } else {
                break;
            }
            if self.check(&TokenKind::Colon)
                || self.check(&TokenKind::RParen)
                || self.check(&TokenKind::RBrace)
                || self.check(&TokenKind::RBracket)
            {
                break;
            }
        }
        if names.is_empty() {
            return Err(ParseError::new(
                ParseErrorKind::InvalidBinder("expected binder name".to_string()),
                self.current_span(),
            ));
        }
        Ok(names)
    }
    /// Collect a type expression until we see `)`, `}`, `]`, `,`, or eof.
    /// Returns it as a Located<SurfaceExpr>::Var (simplified).
    pub(super) fn collect_type_expr(
        &mut self,
    ) -> Result<Box<crate::Located<crate::SurfaceExpr>>, ParseError> {
        let span_start = self.current_span();
        let mut parts = Vec::new();
        let mut depth = 0i32;
        while let Some(token) = self.current() {
            match &token.kind {
                TokenKind::RParen | TokenKind::RBrace | TokenKind::RBracket if depth <= 0 => break,
                TokenKind::Comma if depth <= 0 => break,
                TokenKind::Assign if depth <= 0 => break,
                TokenKind::Eof => break,
                TokenKind::LParen | TokenKind::LBrace | TokenKind::LBracket => {
                    depth += 1;
                    parts.push(format!("{}", token.kind));
                    self.advance();
                }
                TokenKind::RParen | TokenKind::RBrace | TokenKind::RBracket => {
                    depth -= 1;
                    parts.push(format!("{}", token.kind));
                    self.advance();
                }
                _ => {
                    parts.push(format!("{}", token.kind));
                    self.advance();
                }
            }
        }
        let text = parts.join(" ");
        let span_end = self.current_span();
        let merged = span_start.merge(&span_end);
        Ok(Box::new(crate::Located::new(
            crate::SurfaceExpr::Var(text),
            merged,
        )))
    }
    /// Parse a command from a token stream.
    pub fn parse_command(&mut self, tokens: &[Token]) -> Result<Command, ParseError> {
        self.tokens = tokens.to_vec();
        self.pos = 0;
        if tokens.is_empty() {
            return Err(ParseError::new(
                ParseErrorKind::UnexpectedEof {
                    expected: vec!["command".to_string()],
                },
                dummy_span(),
            ));
        }
        let token = &self.tokens[self.pos];
        match &token.kind {
            TokenKind::Import => self.parse_import(),
            TokenKind::Export => self.parse_export(),
            TokenKind::Namespace => self.parse_namespace(),
            TokenKind::End => self.parse_end(),
            TokenKind::Open => self.parse_open(),
            TokenKind::Section => self.parse_section(),
            TokenKind::Variable | TokenKind::Variables => self.parse_variable(),
            TokenKind::Attribute => self.parse_attribute(),
            TokenKind::Structure => self.parse_structure(),
            TokenKind::Class => self.parse_class(),
            TokenKind::Instance => self.parse_instance(),
            TokenKind::Axiom
            | TokenKind::Definition
            | TokenKind::Theorem
            | TokenKind::Lemma
            | TokenKind::Inductive
            | TokenKind::Opaque
            | TokenKind::Constant
            | TokenKind::Constants => {
                let remaining: Vec<Token> = self.tokens[self.pos..].to_vec();
                let mut decl_parser = DeclParser::new(remaining);
                let located_decl: Located<Decl> = decl_parser.parse_decl()?;
                Ok(Command::Decl(located_decl.value))
            }
            TokenKind::Hash => self.parse_hash_command(),
            TokenKind::Ident(s) => {
                let s = s.clone();
                match s.as_str() {
                    "set_option" => self.parse_set_option(),
                    "universe" | "universes" => self.parse_universe(),
                    "notation" => self.parse_notation(NotationKind::Notation),
                    "prefix" => self.parse_notation(NotationKind::Prefix),
                    "infix" | "infixl" | "infixr" => self.parse_notation(NotationKind::Infix),
                    "postfix" => self.parse_notation(NotationKind::Postfix),
                    "derive" | "deriving" => self.parse_derive(),
                    "syntax" => self.parse_syntax(),
                    _ => Err(ParseError::new(
                        ParseErrorKind::InvalidSyntax(format!("unknown command: {}", s)),
                        token.span.clone(),
                    )),
                }
            }
            _ => Err(ParseError::new(
                ParseErrorKind::InvalidSyntax(format!("expected command, got {:?}", token.kind)),
                token.span.clone(),
            )),
        }
    }
    /// Parse `export <name1> <name2> ...`.
    pub(super) fn parse_export(&mut self) -> Result<Command, ParseError> {
        let start_span = self.current_span();
        self.advance();
        let mut names = Vec::new();
        while let Some(token) = self.current() {
            if let TokenKind::Ident(_) = &token.kind {
                names.push(self.parse_ident()?);
            } else {
                break;
            }
        }
        if names.is_empty() {
            return Err(ParseError::new(
                ParseErrorKind::InvalidSyntax("expected at least one name to export".to_string()),
                self.current_span(),
            ));
        }
        let span = start_span.merge(&self.current_span());
        Ok(Command::Export { names, span })
    }
    /// Parse `open <path> [( only | hiding | renaming )]`.
    pub(super) fn parse_open(&mut self) -> Result<Command, ParseError> {
        let start_span = self.current_span();
        self.advance();
        let path = self.parse_dotted_path()?;
        let mut items = Vec::new();
        if self.check_ident("only") {
            self.advance();
            self.expect(&TokenKind::LBracket)?;
            let names = self.parse_comma_separated_idents()?;
            self.expect(&TokenKind::RBracket)?;
            items.push(OpenItem::Only(names));
        } else if self.check_ident("hiding") {
            self.advance();
            self.expect(&TokenKind::LBracket)?;
            let names = self.parse_comma_separated_idents()?;
            self.expect(&TokenKind::RBracket)?;
            items.push(OpenItem::Hiding(names));
        } else if self.check_ident("renaming") {
            self.advance();
            let old_name = self.parse_ident()?;
            self.expect(&TokenKind::Arrow)?;
            let new_name = self.parse_ident()?;
            items.push(OpenItem::Renaming(old_name, new_name));
        } else {
            items.push(OpenItem::All);
        }
        let span = start_span.merge(&self.current_span());
        Ok(Command::Open { path, items, span })
    }
    /// Parse `variable (x : T) ...` or `variables (x : T) ...`.
    pub(super) fn parse_variable(&mut self) -> Result<Command, ParseError> {
        let start_span = self.current_span();
        self.advance();
        let binders = self.parse_binders()?;
        if binders.is_empty() {
            return Err(ParseError::new(
                ParseErrorKind::InvalidBinder("expected at least one binder".to_string()),
                self.current_span(),
            ));
        }
        let span = start_span.merge(&self.current_span());
        Ok(Command::Variable { binders, span })
    }
    /// Parse attribute commands:
    /// - `attribute [attr1, attr2] <name>` -> Command::Attribute
    /// - `attribute [attr_name] <target>` -> Command::ApplyAttribute
    /// - `attribute [attr_name]` -> Command::AttributeDecl
    pub(super) fn parse_attribute(&mut self) -> Result<Command, ParseError> {
        let start_span = self.current_span();
        self.advance();
        self.expect(&TokenKind::LBracket)?;
        let first_name = self.parse_ident()?;
        if self.check(&TokenKind::Comma) {
            let mut attrs = vec![first_name];
            while self.consume(&TokenKind::Comma) {
                attrs.push(self.parse_ident()?);
            }
            self.expect(&TokenKind::RBracket)?;
            let name = self.parse_ident()?;
            let span = start_span.merge(&self.current_span());
            return Ok(Command::Attribute { attrs, name, span });
        }
        let mut params = Vec::new();
        while !self.check(&TokenKind::RBracket) && !self.at_end() {
            if let Some(token) = self.current() {
                if let TokenKind::Ident(_) = &token.kind {
                    params.push(self.parse_ident()?);
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        self.expect(&TokenKind::RBracket)?;
        let kind = if first_name.contains("macro") {
            AttributeDeclKind::Macro
        } else if first_name.contains("builtin") {
            AttributeDeclKind::Builtin
        } else {
            AttributeDeclKind::Simple
        };
        let span = start_span.merge(&self.current_span());
        if let Some(token) = self.current() {
            if let TokenKind::Ident(_) = &token.kind {
                let target = self.parse_ident()?;
                let span = start_span.merge(&self.current_span());
                return Ok(Command::ApplyAttribute {
                    attr_name: first_name,
                    target,
                    params,
                    span,
                });
            }
        }
        Ok(Command::AttributeDecl {
            name: first_name,
            kind,
            span,
        })
    }
    /// Parse `#check <expr>` or `#eval <expr>` or `#print <name>`.
    pub(super) fn parse_hash_command(&mut self) -> Result<Command, ParseError> {
        let start_span = self.current_span();
        self.advance();
        let cmd_name = self.parse_ident()?;
        match cmd_name.as_str() {
            "check" => {
                let expr_str = self.collect_rest_as_string();
                let span = start_span.merge(&self.current_span());
                Ok(Command::Check { expr_str, span })
            }
            "eval" => {
                let expr_str = self.collect_rest_as_string();
                let span = start_span.merge(&self.current_span());
                Ok(Command::Eval { expr_str, span })
            }
            "print" => {
                let name = if let Ok(n) = self.parse_ident() {
                    n
                } else {
                    self.collect_rest_as_string()
                };
                let span = start_span.merge(&self.current_span());
                Ok(Command::Print { name, span })
            }
            "reduce" => {
                let expr_str = self.collect_rest_as_string();
                let span = start_span.merge(&self.current_span());
                Ok(Command::Reduce { expr_str, span })
            }
            _ => Err(ParseError::new(
                ParseErrorKind::InvalidSyntax(format!("unknown # command: #{}", cmd_name)),
                start_span,
            )),
        }
    }
    /// Parse `universe <name1> <name2> ...` or `universes <name1> ...`.
    pub(super) fn parse_universe(&mut self) -> Result<Command, ParseError> {
        let start_span = self.current_span();
        self.advance();
        let mut names = Vec::new();
        while let Some(token) = self.current() {
            if let TokenKind::Ident(_) = &token.kind {
                names.push(self.parse_ident()?);
            } else {
                break;
            }
        }
        if names.is_empty() {
            return Err(ParseError::new(
                ParseErrorKind::InvalidSyntax("expected at least one universe name".to_string()),
                self.current_span(),
            ));
        }
        let span = start_span.merge(&self.current_span());
        Ok(Command::Universe { names, span })
    }
    /// Parse `notation <prec>? <name> := <body>` or prefix/infix/postfix variant.
    pub(super) fn parse_notation(&mut self, kind: NotationKind) -> Result<Command, ParseError> {
        let start_span = self.current_span();
        self.advance();
        let prec = if let Some(token) = self.current() {
            if let TokenKind::Nat(n) = &token.kind {
                let p = *n as u32;
                self.advance();
                Some(p)
            } else {
                None
            }
        } else {
            None
        };
        let name = if let Some(token) = self.current() {
            match &token.kind {
                TokenKind::String(s) => {
                    let n = s.clone();
                    self.advance();
                    n
                }
                TokenKind::Ident(_) => self.parse_ident()?,
                _ => {
                    return Err(ParseError::new(
                        ParseErrorKind::InvalidSyntax(
                            "expected notation name or symbol".to_string(),
                        ),
                        token.span.clone(),
                    ));
                }
            }
        } else {
            return Err(ParseError::new(
                ParseErrorKind::UnexpectedEof {
                    expected: vec!["notation name".to_string()],
                },
                self.eof_span(),
            ));
        };
        let _ = self.consume(&TokenKind::Assign);
        let body = self.collect_rest_as_string();
        let span = start_span.merge(&self.current_span());
        Ok(Command::Notation {
            kind,
            name,
            prec,
            body,
            span,
        })
    }
    /// Parse `derive <strategy1>, <strategy2> for <type_name>` or
    /// `deriving <strategy> for <type_name>`.
    pub(super) fn parse_derive(&mut self) -> Result<Command, ParseError> {
        let start_span = self.current_span();
        self.advance();
        let mut strategies = Vec::new();
        loop {
            let strat = self.parse_ident()?;
            if strat == "for" {
                if strategies.is_empty() {
                    return Err(ParseError::new(
                        ParseErrorKind::InvalidSyntax(
                            "expected strategy name before 'for'".to_string(),
                        ),
                        self.current_span(),
                    ));
                }
                break;
            }
            strategies.push(strat);
            if self.consume(&TokenKind::Comma) {
                continue;
            }
            if self.check_ident("for") {
                self.advance();
                break;
            }
            break;
        }
        let type_name = self.parse_ident()?;
        let span = start_span.merge(&self.current_span());
        Ok(Command::Derive {
            strategies,
            type_name,
            span,
        })
    }
    /// Parse comma-separated identifiers (used inside `[...]`).
    pub(super) fn parse_comma_separated_idents(&mut self) -> Result<Vec<String>, ParseError> {
        let mut names = Vec::new();
        if let Some(token) = self.current() {
            if matches!(token.kind, TokenKind::RBracket) {
                return Ok(names);
            }
        }
        names.push(self.parse_ident()?);
        while self.consume(&TokenKind::Comma) {
            names.push(self.parse_ident()?);
        }
        Ok(names)
    }
    /// Parse `structure <name> [extends <base1>, <base2>] where [field decls] [deriving ...]`.
    pub(super) fn parse_structure(&mut self) -> Result<Command, ParseError> {
        let start_span = self.current_span();
        self.advance();
        let name = self.parse_ident()?;
        let mut extends = Vec::new();
        if self.check_ident("extends") {
            self.advance();
            extends.push(self.parse_ident()?);
            while self.consume(&TokenKind::Comma) {
                extends.push(self.parse_ident()?);
            }
        }
        let mut fields = Vec::new();
        let mut derives = Vec::new();
        if self.check_keyword("where") {
            self.advance();
            fields = self.parse_structure_fields()?;
        }
        if self.check_ident("deriving") {
            self.advance();
            derives.push(self.parse_ident()?);
            while self.consume(&TokenKind::Comma) {
                derives.push(self.parse_ident()?);
            }
        }
        let span = start_span.merge(&self.current_span());
        Ok(Command::Structure {
            name,
            extends,
            fields,
            derives,
            span,
        })
    }
    /// Parse structure field declarations
    pub(super) fn parse_structure_fields(&mut self) -> Result<Vec<StructureField>, ParseError> {
        let mut fields = Vec::new();
        while let Some(token) = self.current() {
            if let TokenKind::Ident(_) = &token.kind {
                if self.check_ident("deriving") {
                    break;
                }
                let field_name = self.parse_ident()?;
                self.expect(&TokenKind::Colon)?;
                let ty = self.collect_type_expr()?;
                let ty_str = match *ty {
                    crate::Located {
                        value: crate::SurfaceExpr::Var(ref s),
                        ..
                    } => s.clone(),
                    _ => "unknown".to_string(),
                };
                let default = if self.consume(&TokenKind::Assign) {
                    Some(self.collect_rest_as_string())
                } else {
                    None
                };
                fields.push(StructureField {
                    name: field_name,
                    ty: ty_str,
                    is_explicit: true,
                    default,
                });
                if !self.consume(&TokenKind::Comma) {
                    break;
                }
            } else {
                break;
            }
        }
        Ok(fields)
    }
    /// Parse `class <name> [(<params>)] [extends <base>] where [methods]`.
    pub(super) fn parse_class(&mut self) -> Result<Command, ParseError> {
        let start_span = self.current_span();
        self.advance();
        let name = self.parse_ident()?;
        let params = self.parse_binders()?;
        let mut extends = Vec::new();
        if self.check_ident("extends") {
            self.advance();
            extends.push(self.parse_ident()?);
            while self.consume(&TokenKind::Comma) {
                extends.push(self.parse_ident()?);
            }
        }
        let mut fields = Vec::new();
        if self.check_keyword("where") {
            self.advance();
            fields = self.parse_structure_fields()?;
        }
        let span = start_span.merge(&self.current_span());
        Ok(Command::Class {
            name,
            params,
            extends,
            fields,
            span,
        })
    }
    /// Parse `instance [<name> :] <type> [priority <n>] := <body>` or `by <tactic>`.
    pub(super) fn parse_instance(&mut self) -> Result<Command, ParseError> {
        let start_span = self.current_span();
        self.advance();
        let name = if self.check_ident("_") {
            self.advance();
            "_".to_string()
        } else {
            self.parse_ident()?
        };
        self.expect(&TokenKind::Colon)?;
        let mut ty_parts = Vec::new();
        while let Some(token) = self.current() {
            match &token.kind {
                TokenKind::Assign | TokenKind::Eof => break,
                TokenKind::Ident(s) if s == "priority" => break,
                _ => {
                    ty_parts.push(format!("{}", token.kind));
                    self.advance();
                }
            }
        }
        let ty_str = ty_parts.join(" ");
        let mut priority = None;
        if self.check_ident("priority") {
            self.advance();
            if let Some(token) = self.current() {
                if let TokenKind::Nat(n) = &token.kind {
                    priority = Some(*n as u32);
                    self.advance();
                }
            }
        }
        self.expect(&TokenKind::Assign)?;
        let body = self.collect_rest_as_string();
        let span = start_span.merge(&self.current_span());
        Ok(Command::Instance {
            name,
            ty: ty_str,
            priority,
            body,
            span,
        })
    }
    /// Parse `attribute [<attr_name> <params>]` or `attribute [<attr>] <target>`.
    #[allow(dead_code)]
    pub(super) fn parse_attribute_decl(&mut self) -> Result<Command, ParseError> {
        let start_span = self.current_span();
        self.advance();
        if !self.check(&TokenKind::LBracket) {
            return Err(ParseError::new(
                ParseErrorKind::InvalidSyntax("expected '[' after 'attribute'".to_string()),
                self.current_span(),
            ));
        }
        self.advance();
        let attr_name = self.parse_ident()?;
        let mut params = Vec::new();
        while !self.check(&TokenKind::RBracket) && !self.at_end() {
            if let Some(token) = self.current() {
                if let TokenKind::Ident(_) = &token.kind {
                    params.push(self.parse_ident()?);
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        self.expect(&TokenKind::RBracket)?;
        let kind = if attr_name.contains("macro") {
            AttributeDeclKind::Macro
        } else if attr_name.contains("builtin") {
            AttributeDeclKind::Builtin
        } else {
            AttributeDeclKind::Simple
        };
        let span = start_span.merge(&self.current_span());
        if let Some(token) = self.current() {
            if let TokenKind::Ident(_) = &token.kind {
                let target = self.parse_ident()?;
                return Ok(Command::ApplyAttribute {
                    attr_name,
                    target,
                    params,
                    span,
                });
            }
        }
        Ok(Command::AttributeDecl {
            name: attr_name,
            kind,
            span,
        })
    }
}
