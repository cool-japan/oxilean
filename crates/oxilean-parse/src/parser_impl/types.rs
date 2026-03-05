//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::ast_impl::*;
use crate::error_impl::ParseError;
use crate::tokens::{StringPart, Token, TokenKind};

/// Parser state.
pub struct Parser {
    /// Token stream
    tokens: Vec<Token>,
    /// Current position
    pos: usize,
}
impl Parser {
    /// Create a new parser from tokens.
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }
    /// Get the current token.
    fn current(&self) -> &Token {
        self.tokens
            .get(self.pos)
            .unwrap_or(&self.tokens[self.tokens.len() - 1])
    }
    /// Peek at the next token (one ahead of current).
    fn peek(&self) -> &Token {
        self.tokens
            .get(self.pos + 1)
            .unwrap_or(&self.tokens[self.tokens.len() - 1])
    }
    /// Peek at the token two ahead of current.
    #[allow(dead_code)]
    fn peek2(&self) -> &Token {
        self.tokens
            .get(self.pos + 2)
            .unwrap_or(&self.tokens[self.tokens.len() - 1])
    }
    /// Check if we're at the end of input.
    /// Return `true` when the token stream is exhausted.
    pub fn is_eof(&self) -> bool {
        matches!(self.current().kind, TokenKind::Eof)
    }
    /// Advance past the current token and return it.
    ///
    /// Public so that callers that drive the parser declaration-by-declaration
    /// (e.g. the build executor) can skip past error positions.
    pub fn advance(&mut self) -> Token {
        let tok = self.current().clone();
        if !self.is_eof() {
            self.pos += 1;
        }
        tok
    }
    /// Expect a specific token kind; error if not found.
    fn expect(&mut self, kind: TokenKind) -> Result<Token, ParseError> {
        if self.current().kind == kind {
            Ok(self.advance())
        } else {
            Err(ParseError::unexpected(
                vec![format!("{}", kind)],
                self.current().kind.clone(),
                self.current().span.clone(),
            ))
        }
    }
    /// Check if current token matches a kind (no consume).
    fn check(&self, kind: &TokenKind) -> bool {
        &self.current().kind == kind
    }
    /// Consume a token if it matches, returning true on success.
    fn consume(&mut self, kind: TokenKind) -> bool {
        if self.check(&kind) {
            self.advance();
            true
        } else {
            false
        }
    }
    /// Check if the current token is an identifier matching the given string.
    fn check_ident(&self, name: &str) -> bool {
        matches!(& self.current().kind, TokenKind::Ident(s) if s == name)
    }
    /// Consume an identifier matching the given string.
    #[allow(dead_code)]
    fn consume_ident(&mut self, name: &str) -> bool {
        if self.check_ident(name) {
            self.advance();
            true
        } else {
            false
        }
    }
}
impl Parser {
    /// Parse a top-level declaration.
    pub fn parse_decl(&mut self) -> Result<Located<Decl>, ParseError> {
        let start = self.current().span.clone();
        if self.check(&TokenKind::At) && self.peek().kind == TokenKind::LBracket {
            return self.parse_attribute_decl();
        }
        if self.check(&TokenKind::Attribute) {
            return self.parse_attribute_keyword();
        }
        match &self.current().kind {
            TokenKind::Axiom => self.parse_axiom(),
            TokenKind::Definition => self.parse_definition(),
            TokenKind::Theorem | TokenKind::Lemma => self.parse_theorem(),
            TokenKind::Inductive => self.parse_inductive(),
            TokenKind::Import => self.parse_import(),
            TokenKind::Namespace => self.parse_namespace(),
            TokenKind::Structure => self.parse_structure(),
            TokenKind::Class => self.parse_class(),
            TokenKind::Instance => self.parse_instance(),
            TokenKind::Section => self.parse_section(),
            TokenKind::Variable
            | TokenKind::Variables
            | TokenKind::Parameter
            | TokenKind::Parameters => self.parse_variable(),
            TokenKind::Open => self.parse_open(),
            TokenKind::Hash => self.parse_hash_cmd(),
            _ => Err(ParseError::unexpected(
                vec!["declaration".to_string()],
                self.current().kind.clone(),
                start,
            )),
        }
    }
    /// Parse an axiom declaration: `axiom name {u, v} : type`
    fn parse_axiom(&mut self) -> Result<Located<Decl>, ParseError> {
        let start = self.current().span.clone();
        self.expect(TokenKind::Axiom)?;
        let name = self.parse_ident()?;
        let univ_params = self.parse_univ_params()?;
        self.expect(TokenKind::Colon)?;
        let ty = self.parse_expr()?;
        let end = ty.span.clone();
        Ok(Located::new(
            Decl::Axiom {
                name,
                univ_params,
                ty,
                attrs: vec![],
            },
            start.merge(&end),
        ))
    }
    /// Parse a definition: `def name {u} : type := value`
    fn parse_definition(&mut self) -> Result<Located<Decl>, ParseError> {
        let start = self.current().span.clone();
        self.expect(TokenKind::Definition)?;
        let name = self.parse_ident()?;
        let univ_params = self.parse_univ_params()?;
        let ty = if self.consume(TokenKind::Colon) {
            Some(self.parse_expr()?)
        } else {
            None
        };
        self.expect(TokenKind::Assign)?;
        let val = self.parse_expr()?;
        let end = val.span.clone();
        Ok(Located::new(
            Decl::Definition {
                name,
                univ_params,
                ty,
                val,
                where_clauses: vec![],
                attrs: vec![],
            },
            start.merge(&end),
        ))
    }
    /// Parse a theorem or lemma: `theorem name : type := proof`
    fn parse_theorem(&mut self) -> Result<Located<Decl>, ParseError> {
        let start = self.current().span.clone();
        if self.check(&TokenKind::Theorem) {
            self.expect(TokenKind::Theorem)?;
        } else {
            self.expect(TokenKind::Lemma)?;
        }
        let name = self.parse_ident()?;
        let univ_params = self.parse_univ_params()?;
        self.expect(TokenKind::Colon)?;
        let ty = self.parse_expr()?;
        self.expect(TokenKind::Assign)?;
        let proof = self.parse_expr()?;
        let end = proof.span.clone();
        Ok(Located::new(
            Decl::Theorem {
                name,
                univ_params,
                ty,
                proof,
                where_clauses: vec![],
                attrs: vec![],
            },
            start.merge(&end),
        ))
    }
    /// Parse an inductive type: `inductive Name : Type | ctor : ...`
    fn parse_inductive(&mut self) -> Result<Located<Decl>, ParseError> {
        let start = self.current().span.clone();
        self.expect(TokenKind::Inductive)?;
        let name = self.parse_ident()?;
        let univ_params = self.parse_univ_params()?;
        let params = self.parse_binders()?;
        let indices = Vec::new();
        self.expect(TokenKind::Colon)?;
        let ty = self.parse_expr()?;
        let mut ctors = Vec::new();
        if self.consume(TokenKind::Bar) {
            loop {
                let ctor_name = self.parse_ident()?;
                self.expect(TokenKind::Colon)?;
                let ctor_ty = self.parse_expr()?;
                ctors.push(Constructor {
                    name: ctor_name,
                    ty: ctor_ty,
                });
                if !self.consume(TokenKind::Bar) {
                    break;
                }
            }
        }
        let end = self.current().span.clone();
        Ok(Located::new(
            Decl::Inductive {
                name,
                univ_params,
                params,
                indices,
                ty,
                ctors,
            },
            start.merge(&end),
        ))
    }
    /// Parse an import: `import Foo.Bar`
    fn parse_import(&mut self) -> Result<Located<Decl>, ParseError> {
        let start = self.current().span.clone();
        self.expect(TokenKind::Import)?;
        let mut path = vec![self.parse_ident()?];
        while self.consume(TokenKind::Dot) {
            path.push(self.parse_ident()?);
        }
        let end = self.current().span.clone();
        Ok(Located::new(Decl::Import { path }, start.merge(&end)))
    }
    /// Parse a namespace: `namespace Name ... end Name`
    fn parse_namespace(&mut self) -> Result<Located<Decl>, ParseError> {
        let start = self.current().span.clone();
        self.expect(TokenKind::Namespace)?;
        let name = self.parse_ident()?;
        let mut decls = Vec::new();
        while !self.check(&TokenKind::End) && !self.is_eof() {
            decls.push(self.parse_decl()?);
        }
        self.expect(TokenKind::End)?;
        if !self.is_eof() && self.check_ident(&name) {
            self.advance();
        }
        let end = self.current().span.clone();
        Ok(Located::new(
            Decl::Namespace { name, decls },
            start.merge(&end),
        ))
    }
    /// Parse a structure: `structure Name where field : Type ...`
    fn parse_structure(&mut self) -> Result<Located<Decl>, ParseError> {
        let start = self.current().span.clone();
        self.expect(TokenKind::Structure)?;
        let name = self.parse_ident()?;
        let univ_params = self.parse_univ_params()?;
        let mut extends = Vec::new();
        if self.check_ident("extends") {
            self.advance();
            extends.push(self.parse_ident()?);
            while self.consume(TokenKind::Comma) {
                extends.push(self.parse_ident()?);
            }
        }
        self.expect(TokenKind::Where)?;
        let fields = self.parse_field_decls()?;
        let end = self.current().span.clone();
        Ok(Located::new(
            Decl::Structure {
                name,
                univ_params,
                extends,
                fields,
            },
            start.merge(&end),
        ))
    }
    /// Parse a class: `class Name where method : Type ...`
    fn parse_class(&mut self) -> Result<Located<Decl>, ParseError> {
        let start = self.current().span.clone();
        self.expect(TokenKind::Class)?;
        let name = self.parse_ident()?;
        let univ_params = self.parse_univ_params()?;
        let mut extends = Vec::new();
        if self.check_ident("extends") {
            self.advance();
            extends.push(self.parse_ident()?);
            while self.consume(TokenKind::Comma) {
                extends.push(self.parse_ident()?);
            }
        }
        self.expect(TokenKind::Where)?;
        let fields = self.parse_field_decls()?;
        let end = self.current().span.clone();
        Ok(Located::new(
            Decl::ClassDecl {
                name,
                univ_params,
                extends,
                fields,
            },
            start.merge(&end),
        ))
    }
    /// Parse field declarations for structures/classes.
    fn parse_field_decls(&mut self) -> Result<Vec<FieldDecl>, ParseError> {
        let mut fields = Vec::new();
        while !self.is_eof() && !self.check(&TokenKind::End) && !self.is_decl_start() {
            if let TokenKind::Ident(_) = &self.current().kind {
                if self.peek().kind == TokenKind::Colon {
                    let field_name = self.parse_ident()?;
                    self.expect(TokenKind::Colon)?;
                    let ty = self.parse_field_type()?;
                    let default = if self.consume(TokenKind::Assign) {
                        Some(self.parse_field_type()?)
                    } else {
                        None
                    };
                    fields.push(FieldDecl {
                        name: field_name,
                        ty,
                        default,
                    });
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        Ok(fields)
    }
    /// Parse an expression for a field type, stopping before `ident :` patterns
    /// that would indicate the start of the next field.
    fn parse_field_type(&mut self) -> Result<Located<SurfaceExpr>, ParseError> {
        let start = self.current().span.clone();
        let mut expr = self.parse_primary()?;
        while !self.is_eof() && !self.is_stop_token() {
            if let TokenKind::Ident(_) = &self.current().kind {
                if self.peek().kind == TokenKind::Colon {
                    break;
                }
            }
            if self.check(&TokenKind::Arrow) {
                self.advance();
                let rhs = self.parse_field_type()?;
                let span = start.merge(&rhs.span);
                let binder = Binder {
                    name: "_".to_string(),
                    ty: Some(Box::new(expr)),
                    info: BinderKind::Default,
                };
                expr = Located::new(SurfaceExpr::Pi(vec![binder], Box::new(rhs)), span);
            } else if self.can_start_expr() {
                let arg = self.parse_primary()?;
                let span = start.merge(&arg.span);
                expr = Located::new(SurfaceExpr::App(Box::new(expr), Box::new(arg)), span);
            } else {
                break;
            }
        }
        Ok(expr)
    }
    /// Parse an instance: `instance [name] : ClassName Type where method := ...`
    fn parse_instance(&mut self) -> Result<Located<Decl>, ParseError> {
        let start = self.current().span.clone();
        self.expect(TokenKind::Instance)?;
        let name = if let TokenKind::Ident(_) = &self.current().kind {
            if self.peek().kind == TokenKind::Colon {
                let n = self.parse_ident()?;
                Some(n)
            } else {
                None
            }
        } else {
            None
        };
        self.expect(TokenKind::Colon)?;
        let class_name = self.parse_ident()?;
        let ty = self.parse_expr()?;
        let mut defs = Vec::new();
        if self.consume(TokenKind::Where) {
            while !self.is_eof() && !self.check(&TokenKind::End) && !self.is_decl_start() {
                if let TokenKind::Ident(_) = &self.current().kind {
                    if self.peek().kind == TokenKind::Assign {
                        let method_name = self.parse_ident()?;
                        self.expect(TokenKind::Assign)?;
                        let method_body = self.parse_expr()?;
                        defs.push((method_name, method_body));
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
        let end = self.current().span.clone();
        Ok(Located::new(
            Decl::InstanceDecl {
                name,
                class_name,
                ty,
                defs,
            },
            start.merge(&end),
        ))
    }
    /// Parse a section: `section Name ... end Name`
    fn parse_section(&mut self) -> Result<Located<Decl>, ParseError> {
        let start = self.current().span.clone();
        self.expect(TokenKind::Section)?;
        let name = self.parse_ident()?;
        let mut decls = Vec::new();
        while !self.check(&TokenKind::End) && !self.is_eof() {
            decls.push(self.parse_decl()?);
        }
        self.expect(TokenKind::End)?;
        if !self.is_eof() && self.check_ident(&name) {
            self.advance();
        }
        let end = self.current().span.clone();
        Ok(Located::new(
            Decl::SectionDecl { name, decls },
            start.merge(&end),
        ))
    }
    /// Parse variable/parameter declarations: `variable (x : T)`
    fn parse_variable(&mut self) -> Result<Located<Decl>, ParseError> {
        let start = self.current().span.clone();
        self.advance();
        let binders = self.parse_binders()?;
        let end = self.current().span.clone();
        Ok(Located::new(Decl::Variable { binders }, start.merge(&end)))
    }
    /// Parse open: `open Name [in expr]` or `open Name (name1 name2)`
    fn parse_open(&mut self) -> Result<Located<Decl>, ParseError> {
        let start = self.current().span.clone();
        self.expect(TokenKind::Open)?;
        let mut path = vec![self.parse_ident()?];
        while self.consume(TokenKind::Dot) {
            path.push(self.parse_ident()?);
        }
        let mut names = Vec::new();
        if self.consume(TokenKind::LParen) {
            while !self.check(&TokenKind::RParen) && !self.is_eof() {
                names.push(self.parse_ident()?);
            }
            self.expect(TokenKind::RParen)?;
        }
        let end = self.current().span.clone();
        Ok(Located::new(Decl::Open { path, names }, start.merge(&end)))
    }
    /// Parse attribute prefix: `@[simp, ext] theorem ...`
    fn parse_attribute_decl(&mut self) -> Result<Located<Decl>, ParseError> {
        let start = self.current().span.clone();
        self.expect(TokenKind::At)?;
        self.expect(TokenKind::LBracket)?;
        let mut attrs = Vec::new();
        attrs.push(self.parse_ident()?);
        while self.consume(TokenKind::Comma) {
            attrs.push(self.parse_ident()?);
        }
        self.expect(TokenKind::RBracket)?;
        let decl = self.parse_decl()?;
        let end = decl.span.clone();
        Ok(Located::new(
            Decl::Attribute {
                attrs,
                decl: Box::new(decl),
            },
            start.merge(&end),
        ))
    }
    /// Parse attribute keyword form: `attribute [simp] name`
    fn parse_attribute_keyword(&mut self) -> Result<Located<Decl>, ParseError> {
        let start = self.current().span.clone();
        self.expect(TokenKind::Attribute)?;
        self.expect(TokenKind::LBracket)?;
        let mut attrs = Vec::new();
        attrs.push(self.parse_ident()?);
        while self.consume(TokenKind::Comma) {
            attrs.push(self.parse_ident()?);
        }
        self.expect(TokenKind::RBracket)?;
        let name = self.parse_ident()?;
        let end_span = self.current().span.clone();
        let inner = Located::new(
            Decl::Axiom {
                name,
                univ_params: vec![],
                ty: Located::new(SurfaceExpr::Hole, end_span.clone()),
                attrs: vec![],
            },
            end_span.clone(),
        );
        Ok(Located::new(
            Decl::Attribute {
                attrs,
                decl: Box::new(inner),
            },
            start.merge(&end_span),
        ))
    }
    /// Parse hash commands: `#check expr`, `#eval expr`, `#print name`
    fn parse_hash_cmd(&mut self) -> Result<Located<Decl>, ParseError> {
        let start = self.current().span.clone();
        self.expect(TokenKind::Hash)?;
        let cmd = self.parse_ident()?;
        let arg = self.parse_expr()?;
        let end = arg.span.clone();
        Ok(Located::new(Decl::HashCmd { cmd, arg }, start.merge(&end)))
    }
    /// Check if current token starts a declaration.
    fn is_decl_start(&self) -> bool {
        matches!(
            self.current().kind,
            TokenKind::Axiom
                | TokenKind::Definition
                | TokenKind::Theorem
                | TokenKind::Lemma
                | TokenKind::Opaque
                | TokenKind::Inductive
                | TokenKind::Structure
                | TokenKind::Class
                | TokenKind::Instance
                | TokenKind::Namespace
                | TokenKind::Section
                | TokenKind::Variable
                | TokenKind::Variables
                | TokenKind::Parameter
                | TokenKind::Parameters
                | TokenKind::Open
                | TokenKind::Attribute
                | TokenKind::Import
                | TokenKind::Export
                | TokenKind::Hash
        ) || (self.check(&TokenKind::At) && self.peek().kind == TokenKind::LBracket)
    }
    /// Parse universe parameters: `{u, v}` - only when LBrace follows
    fn parse_univ_params(&mut self) -> Result<Vec<String>, ParseError> {
        if self.consume(TokenKind::LBrace) {
            let mut params = Vec::new();
            if let TokenKind::Ident(name) = &self.current().kind {
                params.push(name.clone());
                self.advance();
            }
            while self.consume(TokenKind::Comma) {
                if let TokenKind::Ident(name) = &self.current().kind {
                    params.push(name.clone());
                    self.advance();
                }
            }
            self.expect(TokenKind::RBrace)?;
            Ok(params)
        } else {
            Ok(Vec::new())
        }
    }
}
impl Parser {
    /// Parse an expression (entry point, lowest precedence).
    pub fn parse_expr(&mut self) -> Result<Located<SurfaceExpr>, ParseError> {
        self.parse_expr_prec(0)
    }
    /// Parse expression with precedence climbing.
    ///
    /// Precedence table (low to high):
    ///   1  : Arrow (right-assoc)
    ///   5  : Iff
    ///   8  : OrOr / Or
    ///  12  : AndAnd / And
    ///  20  : Eq Ne Lt Le Gt Ge (comparison, non-assoc)
    ///  30  : Plus Minus (left-assoc)
    ///  40  : Star Slash Percent (left-assoc)
    ///  50  : Caret (right-assoc, exponentiation)
    /// 100  : Application (juxtaposition)
    fn parse_expr_prec(&mut self, min_prec: u32) -> Result<Located<SurfaceExpr>, ParseError> {
        let start = self.current().span.clone();
        let mut expr = self.parse_prefix()?;
        loop {
            if self.check(&TokenKind::Dot) {
                if let TokenKind::Ident(_) = &self.peek().kind {
                    self.advance();
                    let field = self.parse_ident()?;
                    let span = start.merge(&self.current().span);
                    expr = Located::new(SurfaceExpr::Proj(Box::new(expr), field), span);
                    continue;
                }
            }
            break;
        }
        while !self.is_eof() && !self.is_stop_token() {
            let (prec, assoc) = self.get_infix_prec_assoc();
            if prec < min_prec {
                break;
            }
            if self.check(&TokenKind::Arrow) {
                self.advance();
                let next_prec = if assoc == Assoc::Right {
                    prec
                } else {
                    prec + 1
                };
                let rhs = self.parse_expr_prec(next_prec)?;
                let span = start.merge(&rhs.span);
                let binder = Binder {
                    name: "_".to_string(),
                    ty: Some(Box::new(expr)),
                    info: BinderKind::Default,
                };
                expr = Located::new(SurfaceExpr::Pi(vec![binder], Box::new(rhs)), span);
            } else if let Some(op_name) = self.get_binop_name() {
                let op_tok = self.advance();
                let op_span = op_tok.span.clone();
                let next_prec = if assoc == Assoc::Right {
                    prec
                } else {
                    prec + 1
                };
                let rhs = self.parse_expr_prec(next_prec)?;
                let span = start.merge(&rhs.span);
                let op_var = Located::new(SurfaceExpr::Var(op_name), op_span);
                let app1 = Located::new(
                    SurfaceExpr::App(Box::new(op_var), Box::new(expr)),
                    span.clone(),
                );
                expr = Located::new(SurfaceExpr::App(Box::new(app1), Box::new(rhs)), span);
            } else if prec == 100 {
                if self.can_start_expr() {
                    let arg = self.parse_app_arg()?;
                    let span = start.merge(&arg.span);
                    expr = Located::new(SurfaceExpr::App(Box::new(expr), Box::new(arg)), span);
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        Ok(expr)
    }
    /// Parse an application argument.
    /// Handles named arguments: `(x := e)` as well as normal primaries.
    fn parse_app_arg(&mut self) -> Result<Located<SurfaceExpr>, ParseError> {
        if self.check(&TokenKind::LParen) {
            if let TokenKind::Ident(_) = &self.peek().kind {
                if self.peek2().kind == TokenKind::Assign {
                    return self.parse_named_arg();
                }
            }
        }
        self.parse_primary()
    }
    /// Parse a named argument: `(x := expr)`
    fn parse_named_arg(&mut self) -> Result<Located<SurfaceExpr>, ParseError> {
        let start = self.current().span.clone();
        self.expect(TokenKind::LParen)?;
        let name = self.parse_ident()?;
        self.expect(TokenKind::Assign)?;
        let val = self.parse_expr()?;
        self.expect(TokenKind::RParen)?;
        let end = self.current().span.clone();
        Ok(Located::new(
            SurfaceExpr::NamedArg(
                Box::new(Located::new(SurfaceExpr::Hole, start.clone())),
                name,
                Box::new(val),
            ),
            start.merge(&end),
        ))
    }
    /// Return the binary operator name for the current token, if it is a binop.
    fn get_binop_name(&self) -> Option<String> {
        match &self.current().kind {
            TokenKind::Plus => Some("+".to_string()),
            TokenKind::Minus => Some("-".to_string()),
            TokenKind::Star => Some("*".to_string()),
            TokenKind::Slash => Some("/".to_string()),
            TokenKind::Percent => Some("%".to_string()),
            TokenKind::Caret => Some("^".to_string()),
            TokenKind::AndAnd => Some("&&".to_string()),
            TokenKind::OrOr => Some("||".to_string()),
            TokenKind::And => Some("And".to_string()),
            TokenKind::Or => Some("Or".to_string()),
            TokenKind::Iff => Some("Iff".to_string()),
            TokenKind::Eq => Some("Eq".to_string()),
            TokenKind::Ne => Some("Ne".to_string()),
            TokenKind::BangEq => Some("Ne".to_string()),
            TokenKind::Lt => Some("Lt".to_string()),
            TokenKind::Le => Some("Le".to_string()),
            TokenKind::Gt => Some("Gt".to_string()),
            TokenKind::Ge => Some("Ge".to_string()),
            _ => None,
        }
    }
    /// Check if current token is a stop token (ends expression parsing).
    fn is_stop_token(&self) -> bool {
        matches!(
            self.current().kind,
            TokenKind::RParen
                | TokenKind::RBrace
                | TokenKind::RBracket
                | TokenKind::RAngle
                | TokenKind::Comma
                | TokenKind::In
                | TokenKind::Bar
                | TokenKind::Semicolon
                | TokenKind::Then
                | TokenKind::Else
                | TokenKind::With
                | TokenKind::Where
                | TokenKind::End
                | TokenKind::From
                | TokenKind::By
                | TokenKind::Assign
        )
    }
    /// Check if current token can start an expression.
    fn can_start_expr(&self) -> bool {
        matches!(
            self.current().kind,
            TokenKind::Ident(_)
                | TokenKind::Nat(_)
                | TokenKind::String(_)
                | TokenKind::Type
                | TokenKind::Prop
                | TokenKind::Sort
                | TokenKind::Fun
                | TokenKind::Forall
                | TokenKind::Exists
                | TokenKind::Let
                | TokenKind::If
                | TokenKind::Match
                | TokenKind::Do
                | TokenKind::Have
                | TokenKind::Suffices
                | TokenKind::Show
                | TokenKind::Underscore
                | TokenKind::LParen
                | TokenKind::LBracket
                | TokenKind::LAngle
                | TokenKind::Not
                | TokenKind::Bang
                | TokenKind::Question
        )
    }
    /// Get infix operator precedence and associativity.
    fn get_infix_prec_assoc(&self) -> (u32, Assoc) {
        match &self.current().kind {
            TokenKind::Arrow => (1, Assoc::Right),
            TokenKind::Iff => (5, Assoc::Left),
            TokenKind::OrOr | TokenKind::Or => (8, Assoc::Left),
            TokenKind::AndAnd | TokenKind::And => (12, Assoc::Left),
            TokenKind::Eq
            | TokenKind::Ne
            | TokenKind::BangEq
            | TokenKind::Lt
            | TokenKind::Le
            | TokenKind::Gt
            | TokenKind::Ge => (20, Assoc::Left),
            TokenKind::Plus | TokenKind::Minus => (30, Assoc::Left),
            TokenKind::Star | TokenKind::Slash | TokenKind::Percent => (40, Assoc::Left),
            TokenKind::Caret => (50, Assoc::Right),
            _ => (100, Assoc::Left),
        }
    }
}
impl Parser {
    /// Parse a prefix expression (unary operators or primary).
    fn parse_prefix(&mut self) -> Result<Located<SurfaceExpr>, ParseError> {
        let start = self.current().span.clone();
        match &self.current().kind {
            TokenKind::Not | TokenKind::Bang => {
                self.advance();
                let operand = self.parse_prefix()?;
                let span = start.merge(&operand.span);
                let not_var = Located::new(SurfaceExpr::Var("Not".to_string()), start);
                Ok(Located::new(
                    SurfaceExpr::App(Box::new(not_var), Box::new(operand)),
                    span,
                ))
            }
            TokenKind::Minus => {
                self.advance();
                let operand = self.parse_prefix()?;
                let span = start.merge(&operand.span);
                let neg_var = Located::new(SurfaceExpr::Var("Neg".to_string()), start);
                Ok(Located::new(
                    SurfaceExpr::App(Box::new(neg_var), Box::new(operand)),
                    span,
                ))
            }
            _ => self.parse_primary(),
        }
    }
    /// Parse primary expression (atoms and compound forms).
    fn parse_primary(&mut self) -> Result<Located<SurfaceExpr>, ParseError> {
        let start = self.current().span.clone();
        match &self.current().kind.clone() {
            TokenKind::Ident(name) => {
                let name = name.clone();
                self.advance();
                Ok(Located::new(SurfaceExpr::Var(name), start))
            }
            TokenKind::Nat(n) => {
                let n = *n;
                self.advance();
                Ok(Located::new(SurfaceExpr::Lit(Literal::Nat(n)), start))
            }
            TokenKind::String(s) => {
                let s = s.clone();
                self.advance();
                Ok(Located::new(SurfaceExpr::Lit(Literal::String(s)), start))
            }
            TokenKind::Type => {
                self.advance();
                if let TokenKind::Ident(u) = &self.current().kind {
                    let u = u.clone();
                    let end = self.current().span.clone();
                    self.advance();
                    Ok(Located::new(
                        SurfaceExpr::Sort(SortKind::TypeU(u)),
                        start.merge(&end),
                    ))
                } else if let TokenKind::Nat(n) = &self.current().kind {
                    if *n > 0 {
                        let u = format!("{}", n);
                        let end = self.current().span.clone();
                        self.advance();
                        Ok(Located::new(
                            SurfaceExpr::Sort(SortKind::TypeU(u)),
                            start.merge(&end),
                        ))
                    } else {
                        Ok(Located::new(SurfaceExpr::Sort(SortKind::Type), start))
                    }
                } else {
                    Ok(Located::new(SurfaceExpr::Sort(SortKind::Type), start))
                }
            }
            TokenKind::Prop => {
                self.advance();
                Ok(Located::new(SurfaceExpr::Sort(SortKind::Prop), start))
            }
            TokenKind::Sort => {
                self.advance();
                if let TokenKind::Ident(u) = &self.current().kind {
                    let u = u.clone();
                    let end = self.current().span.clone();
                    self.advance();
                    Ok(Located::new(
                        SurfaceExpr::Sort(SortKind::SortU(u)),
                        start.merge(&end),
                    ))
                } else {
                    Ok(Located::new(SurfaceExpr::Sort(SortKind::Prop), start))
                }
            }
            TokenKind::Fun => self.parse_lambda(),
            TokenKind::Forall => self.parse_pi(),
            TokenKind::Exists => self.parse_exists(),
            TokenKind::Let => self.parse_let(),
            TokenKind::If => self.parse_if(),
            TokenKind::Match => self.parse_match(),
            TokenKind::Do => self.parse_do(),
            TokenKind::Have => self.parse_have(),
            TokenKind::Suffices => self.parse_suffices(),
            TokenKind::Show => self.parse_show(),
            TokenKind::Underscore => {
                self.advance();
                Ok(Located::new(SurfaceExpr::Hole, start))
            }
            TokenKind::Question => {
                self.advance();
                Ok(Located::new(SurfaceExpr::Hole, start))
            }
            TokenKind::LParen => self.parse_paren_or_tuple(),
            TokenKind::LBracket => self.parse_list_literal(),
            TokenKind::LAngle => self.parse_anonymous_ctor(),
            _ => Err(ParseError::unexpected(
                vec!["expression".to_string()],
                self.current().kind.clone(),
                start,
            )),
        }
    }
    /// Parse lambda expression: `fun (x : T) => body`
    fn parse_lambda(&mut self) -> Result<Located<SurfaceExpr>, ParseError> {
        let start = self.current().span.clone();
        self.expect(TokenKind::Fun)?;
        let binders = self.parse_binders()?;
        self.expect(TokenKind::Arrow)?;
        let body = self.parse_expr()?;
        let end = body.span.clone();
        Ok(Located::new(
            SurfaceExpr::Lam(binders, Box::new(body)),
            start.merge(&end),
        ))
    }
    /// Parse Pi type: `forall (x : T), body`
    fn parse_pi(&mut self) -> Result<Located<SurfaceExpr>, ParseError> {
        let start = self.current().span.clone();
        self.expect(TokenKind::Forall)?;
        let binders = self.parse_binders()?;
        self.expect(TokenKind::Comma)?;
        let body = self.parse_expr()?;
        let end = body.span.clone();
        Ok(Located::new(
            SurfaceExpr::Pi(binders, Box::new(body)),
            start.merge(&end),
        ))
    }
    /// Parse existential quantifier: `∃ binders, body` → `Exists (fun binders => body)`
    fn parse_exists(&mut self) -> Result<Located<SurfaceExpr>, ParseError> {
        let start = self.current().span.clone();
        self.expect(TokenKind::Exists)?;
        let binders = self.parse_binders()?;
        self.expect(TokenKind::Comma)?;
        let body = self.parse_expr()?;
        let end = body.span.clone();
        let span = start.merge(&end);
        let exists_var = Located::new(SurfaceExpr::Var("Exists".to_string()), span.clone());
        let lam = Located::new(SurfaceExpr::Lam(binders, Box::new(body)), span.clone());
        Ok(Located::new(
            SurfaceExpr::App(Box::new(exists_var), Box::new(lam)),
            span,
        ))
    }
    /// Parse let expression: `let x : T := val in body`
    fn parse_let(&mut self) -> Result<Located<SurfaceExpr>, ParseError> {
        let start = self.current().span.clone();
        self.expect(TokenKind::Let)?;
        let name = self.parse_ident()?;
        let ty = if self.consume(TokenKind::Colon) {
            Some(Box::new(self.parse_expr()?))
        } else {
            None
        };
        self.expect(TokenKind::Assign)?;
        let val = self.parse_expr()?;
        self.expect(TokenKind::In)?;
        let body = self.parse_expr()?;
        let end = body.span.clone();
        Ok(Located::new(
            SurfaceExpr::Let(name, ty, Box::new(val), Box::new(body)),
            start.merge(&end),
        ))
    }
    /// Parse if-then-else: `if cond then t else e`
    fn parse_if(&mut self) -> Result<Located<SurfaceExpr>, ParseError> {
        let start = self.current().span.clone();
        self.expect(TokenKind::If)?;
        let cond = self.parse_expr()?;
        self.expect(TokenKind::Then)?;
        let then_branch = self.parse_expr()?;
        self.expect(TokenKind::Else)?;
        let else_branch = self.parse_expr()?;
        let end = else_branch.span.clone();
        Ok(Located::new(
            SurfaceExpr::If(Box::new(cond), Box::new(then_branch), Box::new(else_branch)),
            start.merge(&end),
        ))
    }
    /// Parse match expression: `match e with | pat => rhs | ...`
    fn parse_match(&mut self) -> Result<Located<SurfaceExpr>, ParseError> {
        let start = self.current().span.clone();
        self.expect(TokenKind::Match)?;
        let scrutinee = self.parse_expr()?;
        self.expect(TokenKind::With)?;
        let mut arms = Vec::new();
        self.consume(TokenKind::Bar);
        loop {
            let pat = self.parse_pattern()?;
            let guard = if self.check(&TokenKind::If) {
                self.advance();
                Some(self.parse_expr_prec(2)?)
            } else {
                None
            };
            self.expect(TokenKind::Arrow)?;
            let rhs = self.parse_expr()?;
            arms.push(MatchArm {
                pattern: pat,
                guard,
                rhs,
            });
            if !self.consume(TokenKind::Bar) {
                break;
            }
        }
        let end = self.current().span.clone();
        Ok(Located::new(
            SurfaceExpr::Match(Box::new(scrutinee), arms),
            start.merge(&end),
        ))
    }
    /// Parse a pattern for match arms.
    fn parse_pattern(&mut self) -> Result<Located<Pattern>, ParseError> {
        let start = self.current().span.clone();
        match &self.current().kind.clone() {
            TokenKind::Underscore => {
                self.advance();
                Ok(Located::new(Pattern::Wild, start))
            }
            TokenKind::Nat(n) => {
                let n = *n;
                self.advance();
                Ok(Located::new(Pattern::Lit(Literal::Nat(n)), start))
            }
            TokenKind::String(s) => {
                let s = s.clone();
                self.advance();
                Ok(Located::new(Pattern::Lit(Literal::String(s)), start))
            }
            TokenKind::Ident(name) => {
                let name = name.clone();
                self.advance();
                let mut sub_pats = Vec::new();
                while self.can_start_pattern() {
                    sub_pats.push(self.parse_atomic_pattern()?);
                }
                if sub_pats.is_empty() {
                    Ok(Located::new(Pattern::Var(name), start))
                } else {
                    let end = sub_pats
                        .last()
                        .expect("sub_pats non-empty per else branch")
                        .span
                        .clone();
                    Ok(Located::new(
                        Pattern::Ctor(name, sub_pats),
                        start.merge(&end),
                    ))
                }
            }
            TokenKind::LParen => {
                self.advance();
                let inner = self.parse_pattern()?;
                self.expect(TokenKind::RParen)?;
                Ok(inner)
            }
            _ => Err(ParseError::unexpected(
                vec!["pattern".to_string()],
                self.current().kind.clone(),
                start,
            )),
        }
    }
    /// Parse an atomic pattern (used as sub-patterns for constructors).
    fn parse_atomic_pattern(&mut self) -> Result<Located<Pattern>, ParseError> {
        let start = self.current().span.clone();
        match &self.current().kind.clone() {
            TokenKind::Underscore => {
                self.advance();
                Ok(Located::new(Pattern::Wild, start))
            }
            TokenKind::Nat(n) => {
                let n = *n;
                self.advance();
                Ok(Located::new(Pattern::Lit(Literal::Nat(n)), start))
            }
            TokenKind::String(s) => {
                let s = s.clone();
                self.advance();
                Ok(Located::new(Pattern::Lit(Literal::String(s)), start))
            }
            TokenKind::Ident(name) => {
                let name = name.clone();
                self.advance();
                Ok(Located::new(Pattern::Var(name), start))
            }
            TokenKind::LParen => {
                self.advance();
                let inner = self.parse_pattern()?;
                self.expect(TokenKind::RParen)?;
                Ok(inner)
            }
            _ => Err(ParseError::unexpected(
                vec!["pattern".to_string()],
                self.current().kind.clone(),
                start,
            )),
        }
    }
    /// Check if current token can start a pattern.
    fn can_start_pattern(&self) -> bool {
        matches!(
            self.current().kind,
            TokenKind::Ident(_)
                | TokenKind::Nat(_)
                | TokenKind::String(_)
                | TokenKind::Underscore
                | TokenKind::LParen
        )
    }
    /// Parse do notation: `do { action1; action2; ... }` or `do action1; action2`
    fn parse_do(&mut self) -> Result<Located<SurfaceExpr>, ParseError> {
        let start = self.current().span.clone();
        self.expect(TokenKind::Do)?;
        let has_brace = self.consume(TokenKind::LBrace);
        let mut actions = Vec::new();
        loop {
            if has_brace && self.check(&TokenKind::RBrace) {
                break;
            }
            if self.is_eof() {
                break;
            }
            let action = self.parse_do_action()?;
            actions.push(action);
            if !self.consume(TokenKind::Semicolon) {
                break;
            }
        }
        if has_brace {
            self.expect(TokenKind::RBrace)?;
        }
        let end = self.current().span.clone();
        Ok(Located::new(SurfaceExpr::Do(actions), start.merge(&end)))
    }
    /// Parse a single do-notation action.
    fn parse_do_action(&mut self) -> Result<DoAction, ParseError> {
        if self.check(&TokenKind::Let) {
            self.advance();
            let name = self.parse_ident()?;
            if self.consume(TokenKind::Colon) {
                let ty = self.parse_expr()?;
                self.expect(TokenKind::Assign)?;
                let val = self.parse_expr()?;
                return Ok(DoAction::LetTyped(name, ty, val));
            }
            self.expect(TokenKind::Assign)?;
            let val = self.parse_expr()?;
            return Ok(DoAction::Let(name, val));
        }
        if let TokenKind::Ident(_) = &self.current().kind {
            if self.peek().kind == TokenKind::LeftArrow {
                let name = self.parse_ident()?;
                self.expect(TokenKind::LeftArrow)?;
                let val = self.parse_expr()?;
                return Ok(DoAction::Bind(name, val));
            }
        }
        let expr = self.parse_expr()?;
        Ok(DoAction::Expr(expr))
    }
    /// Parse have expression: `have h : T := proof; body`
    fn parse_have(&mut self) -> Result<Located<SurfaceExpr>, ParseError> {
        let start = self.current().span.clone();
        self.expect(TokenKind::Have)?;
        let name = self.parse_ident()?;
        self.expect(TokenKind::Colon)?;
        let ty = self.parse_expr()?;
        self.expect(TokenKind::Assign)?;
        let proof = self.parse_expr()?;
        self.expect(TokenKind::Semicolon)?;
        let body = self.parse_expr()?;
        let end = body.span.clone();
        Ok(Located::new(
            SurfaceExpr::Have(name, Box::new(ty), Box::new(proof), Box::new(body)),
            start.merge(&end),
        ))
    }
    /// Parse suffices expression: `suffices h : T by tactic; body`
    fn parse_suffices(&mut self) -> Result<Located<SurfaceExpr>, ParseError> {
        let start = self.current().span.clone();
        self.expect(TokenKind::Suffices)?;
        let name = self.parse_ident()?;
        self.expect(TokenKind::Colon)?;
        let ty = self.parse_expr()?;
        self.expect(TokenKind::By)?;
        let tactic = self.parse_expr()?;
        let end = tactic.span.clone();
        Ok(Located::new(
            SurfaceExpr::Suffices(name, Box::new(ty), Box::new(tactic)),
            start.merge(&end),
        ))
    }
    /// Parse show expression: `show T from expr`
    fn parse_show(&mut self) -> Result<Located<SurfaceExpr>, ParseError> {
        let start = self.current().span.clone();
        self.expect(TokenKind::Show)?;
        let ty = self.parse_expr()?;
        self.expect(TokenKind::From)?;
        let proof = self.parse_expr()?;
        let end = proof.span.clone();
        Ok(Located::new(
            SurfaceExpr::Show(Box::new(ty), Box::new(proof)),
            start.merge(&end),
        ))
    }
    /// Parse parenthesized expression, tuple, or type annotation.
    ///
    /// `(e)` -- grouping
    /// `(e : T)` -- type annotation
    /// `(e, f, ...)` -- tuple
    fn parse_paren_or_tuple(&mut self) -> Result<Located<SurfaceExpr>, ParseError> {
        let start = self.current().span.clone();
        self.expect(TokenKind::LParen)?;
        if self.consume(TokenKind::RParen) {
            return Ok(Located::new(SurfaceExpr::Tuple(vec![]), start));
        }
        let first = self.parse_expr()?;
        if self.consume(TokenKind::Colon) {
            let ty = self.parse_expr()?;
            let span = start.merge(&ty.span);
            self.expect(TokenKind::RParen)?;
            return Ok(Located::new(
                SurfaceExpr::Ann(Box::new(first), Box::new(ty)),
                span,
            ));
        }
        if self.consume(TokenKind::Comma) {
            let mut elems = vec![first];
            elems.push(self.parse_expr()?);
            while self.consume(TokenKind::Comma) {
                elems.push(self.parse_expr()?);
            }
            self.expect(TokenKind::RParen)?;
            let end = self.current().span.clone();
            return Ok(Located::new(SurfaceExpr::Tuple(elems), start.merge(&end)));
        }
        self.expect(TokenKind::RParen)?;
        Ok(first)
    }
    /// Parse list literal: `[e1, e2, ...]` or `[]`
    fn parse_list_literal(&mut self) -> Result<Located<SurfaceExpr>, ParseError> {
        let start = self.current().span.clone();
        self.expect(TokenKind::LBracket)?;
        let mut elems = Vec::new();
        if !self.check(&TokenKind::RBracket) {
            elems.push(self.parse_expr()?);
            while self.consume(TokenKind::Comma) {
                elems.push(self.parse_expr()?);
            }
        }
        self.expect(TokenKind::RBracket)?;
        let end = self.current().span.clone();
        Ok(Located::new(SurfaceExpr::ListLit(elems), start.merge(&end)))
    }
    /// Parse anonymous constructor: `(langle) a, b, c (rangle)`
    fn parse_anonymous_ctor(&mut self) -> Result<Located<SurfaceExpr>, ParseError> {
        let start = self.current().span.clone();
        self.expect(TokenKind::LAngle)?;
        let mut elems = Vec::new();
        if !self.check(&TokenKind::RAngle) {
            elems.push(self.parse_expr()?);
            while self.consume(TokenKind::Comma) {
                elems.push(self.parse_expr()?);
            }
        }
        self.expect(TokenKind::RAngle)?;
        let end = self.current().span.clone();
        Ok(Located::new(
            SurfaceExpr::AnonymousCtor(elems),
            start.merge(&end),
        ))
    }
}
impl Parser {
    /// Parse binders for lambda, forall, variable declarations.
    ///
    /// Supports:
    /// - `(x : T)` -- explicit
    /// - `{x : T}` -- implicit
    /// - `[x : T]` -- instance
    /// - `{{x : T}}` -- strict implicit
    /// - `x` -- simple binder without type
    /// - Multiple binder groups
    pub fn parse_binders(&mut self) -> Result<Vec<Binder>, ParseError> {
        let mut binders = Vec::new();
        loop {
            match &self.current().kind {
                TokenKind::LParen => {
                    self.advance();
                    self.parse_binder_group(&mut binders, BinderKind::Default)?;
                    self.expect(TokenKind::RParen)?;
                }
                TokenKind::LBrace => {
                    if self.peek().kind == TokenKind::LBrace {
                        self.advance();
                        self.advance();
                        self.parse_binder_group(&mut binders, BinderKind::StrictImplicit)?;
                        self.expect(TokenKind::RBrace)?;
                        self.expect(TokenKind::RBrace)?;
                    } else {
                        self.advance();
                        self.parse_binder_group(&mut binders, BinderKind::Implicit)?;
                        self.expect(TokenKind::RBrace)?;
                    }
                }
                TokenKind::LBracket => {
                    self.advance();
                    self.parse_binder_group(&mut binders, BinderKind::Instance)?;
                    self.expect(TokenKind::RBracket)?;
                }
                TokenKind::Ident(_) | TokenKind::Underscore => {
                    let name = if self.check(&TokenKind::Underscore) {
                        self.advance();
                        "_".to_string()
                    } else {
                        self.parse_ident()?
                    };
                    binders.push(Binder {
                        name,
                        ty: None,
                        info: BinderKind::Default,
                    });
                }
                _ => break,
            }
            if !matches!(
                self.current().kind,
                TokenKind::LParen
                    | TokenKind::LBrace
                    | TokenKind::LBracket
                    | TokenKind::Ident(_)
                    | TokenKind::Underscore
            ) {
                break;
            }
        }
        Ok(binders)
    }
    /// Parse a binder group inside delimiters: `x y : T` or `x : T, y : T`
    fn parse_binder_group(
        &mut self,
        binders: &mut Vec<Binder>,
        kind: BinderKind,
    ) -> Result<(), ParseError> {
        let mut names = Vec::new();
        loop {
            let name = if self.check(&TokenKind::Underscore) {
                self.advance();
                "_".to_string()
            } else if let TokenKind::Ident(_) = &self.current().kind {
                self.parse_ident()?
            } else {
                break;
            };
            names.push(name);
            if self.check(&TokenKind::Colon) {
                break;
            }
            if self.check(&TokenKind::Comma) {
                break;
            }
        }
        if names.is_empty() {
            return Err(ParseError::unexpected(
                vec!["identifier".to_string()],
                self.current().kind.clone(),
                self.current().span.clone(),
            ));
        }
        let ty = if self.consume(TokenKind::Colon) {
            Some(self.parse_expr()?)
        } else {
            None
        };
        for name in names {
            binders.push(Binder {
                name,
                ty: ty.as_ref().map(|t| Box::new(t.clone())),
                info: kind.clone(),
            });
        }
        while self.consume(TokenKind::Comma) {
            let name = if self.check(&TokenKind::Underscore) {
                self.advance();
                "_".to_string()
            } else {
                self.parse_ident()?
            };
            let more_ty = if self.consume(TokenKind::Colon) {
                Some(Box::new(self.parse_expr()?))
            } else {
                None
            };
            binders.push(Binder {
                name,
                ty: more_ty,
                info: kind.clone(),
            });
        }
        Ok(())
    }
}
impl Parser {
    /// Parse where clauses for definitions and theorems.
    #[allow(dead_code)]
    fn parse_where_clauses(&mut self) -> Result<Vec<WhereClause>, ParseError> {
        let mut clauses = Vec::new();
        if !self.check_ident("where") {
            return Ok(clauses);
        }
        self.advance();
        loop {
            if self.is_eof() || self.is_decl_start() {
                break;
            }
            let name = self.parse_ident()?;
            let params = self.parse_binders()?;
            let ty = if self.consume(TokenKind::Colon) {
                Some(self.parse_expr()?)
            } else {
                None
            };
            self.expect(TokenKind::Assign)?;
            let val = self.parse_expr()?;
            clauses.push(WhereClause {
                name,
                params,
                ty,
                val,
            });
            if !self.consume(TokenKind::Comma) {
                break;
            }
        }
        Ok(clauses)
    }
    /// Parse calc expression: `calc x = y := proof1 _ = z := proof2`
    #[allow(dead_code)]
    fn parse_calc(&mut self) -> Result<Located<SurfaceExpr>, ParseError> {
        let start = self.current().span.clone();
        if !self.check_ident("calc") {
            return Err(ParseError::unexpected(
                vec!["calc".to_string()],
                self.current().kind.clone(),
                start,
            ));
        }
        self.advance();
        let mut steps = Vec::new();
        let lhs = self.parse_expr()?;
        let rel = self.parse_ident()?;
        let rhs = self.parse_expr()?;
        self.expect(TokenKind::Assign)?;
        let proof = self.parse_expr()?;
        steps.push(CalcStep {
            lhs,
            rel,
            rhs,
            proof,
        });
        while self.consume(TokenKind::Underscore) {
            let rel = self.parse_ident()?;
            let rhs = self.parse_expr()?;
            self.expect(TokenKind::Assign)?;
            let proof = self.parse_expr()?;
            let prev_rhs = steps
                .last()
                .expect("steps non-empty: first step pushed before loop")
                .rhs
                .clone();
            steps.push(CalcStep {
                lhs: prev_rhs,
                rel,
                rhs,
                proof,
            });
        }
        let end = self.current().span.clone();
        Ok(Located::new(SurfaceExpr::Calc(steps), start.merge(&end)))
    }
    /// Parse by-tactic expression: `by simp; ring`
    #[allow(dead_code)]
    fn parse_by_tactic(&mut self) -> Result<Located<SurfaceExpr>, ParseError> {
        let start = self.current().span.clone();
        self.expect(TokenKind::By)?;
        let mut tactics = Vec::new();
        loop {
            if self.is_eof() || self.is_stop_token() {
                break;
            }
            let tactic_name = self.parse_ident()?;
            let span = self.current().span.clone();
            tactics.push(Located::new(tactic_name, span));
            if !self.consume(TokenKind::Semicolon) {
                break;
            }
        }
        let end = self.current().span.clone();
        Ok(Located::new(
            SurfaceExpr::ByTactic(tactics),
            start.merge(&end),
        ))
    }
    /// Parse return expression (for do notation): `return e`
    #[allow(dead_code)]
    fn parse_return(&mut self) -> Result<Located<SurfaceExpr>, ParseError> {
        let start = self.current().span.clone();
        if !self.check_ident("return") {
            return Err(ParseError::unexpected(
                vec!["return".to_string()],
                self.current().kind.clone(),
                start,
            ));
        }
        self.advance();
        let val = self.parse_expr()?;
        let end = val.span.clone();
        Ok(Located::new(
            SurfaceExpr::Return(Box::new(val)),
            start.merge(&end),
        ))
    }
    /// Parse range expression: `a..b`, `..b`, `a..`
    #[allow(dead_code)]
    fn parse_range(&mut self) -> Result<Located<SurfaceExpr>, ParseError> {
        let start = self.current().span.clone();
        let start_expr = if self.check(&TokenKind::Dot) && self.peek().kind == TokenKind::Dot {
            None
        } else {
            Some(Box::new(self.parse_expr()?))
        };
        if !self.check(&TokenKind::Dot) || self.peek().kind != TokenKind::Dot {
            return Err(ParseError::unexpected(
                vec!["..".to_string()],
                self.current().kind.clone(),
                self.current().span.clone(),
            ));
        }
        self.advance();
        self.advance();
        let end_expr = if self.is_stop_token() {
            None
        } else {
            Some(Box::new(self.parse_expr()?))
        };
        let end = self.current().span.clone();
        Ok(Located::new(
            SurfaceExpr::Range(start_expr, end_expr),
            start.merge(&end),
        ))
    }
    /// Parse string interpolation: `s!"hello {name}"`
    #[allow(dead_code)]
    fn parse_string_interp(&mut self) -> Result<Located<SurfaceExpr>, ParseError> {
        let start = self.current().span.clone();
        if let TokenKind::String(s) = &self.current().kind {
            let s = s.clone();
            self.advance();
            let part = StringPart::Literal(s);
            let end = self.current().span.clone();
            Ok(Located::new(
                SurfaceExpr::StringInterp(vec![part]),
                start.merge(&end),
            ))
        } else {
            Err(ParseError::unexpected(
                vec!["string".to_string()],
                self.current().kind.clone(),
                start,
            ))
        }
    }
    /// Parse implicit argument application
    #[allow(dead_code)]
    fn parse_implicit_app(&mut self) -> Result<Located<SurfaceExpr>, ParseError> {
        let start = self.current().span.clone();
        let mut expr = self.parse_primary()?;
        loop {
            if self.check(&TokenKind::LBrace) {
                self.advance();
                let arg = self.parse_expr()?;
                self.expect(TokenKind::RBrace)?;
                let span = start.merge(&arg.span);
                expr = Located::new(SurfaceExpr::App(Box::new(expr), Box::new(arg)), span);
            } else {
                break;
            }
        }
        Ok(expr)
    }
    /// Parse constructor with named fields: `{ field := value, ... }`
    #[allow(dead_code)]
    fn parse_named_ctor(&mut self) -> Result<Located<SurfaceExpr>, ParseError> {
        let start = self.current().span.clone();
        self.expect(TokenKind::LBrace)?;
        let mut fields = Vec::new();
        if !self.check(&TokenKind::RBrace) {
            loop {
                let field_name = self.parse_ident()?;
                self.expect(TokenKind::Assign)?;
                let field_val = self.parse_expr()?;
                fields.push((field_name, field_val));
                if !self.consume(TokenKind::Comma) {
                    break;
                }
            }
        }
        self.expect(TokenKind::RBrace)?;
        let end = self.current().span.clone();
        let mut result = Located::new(SurfaceExpr::Hole, start.clone());
        for (field_name, field_val) in fields {
            result = Located::new(
                SurfaceExpr::NamedArg(Box::new(result), field_name, Box::new(field_val)),
                start.merge(&end),
            );
        }
        Ok(result)
    }
    /// Attempt error recovery by synchronizing to the next safe token
    #[allow(dead_code)]
    fn synchronize(&mut self) {
        while !self.is_eof() {
            match &self.current().kind {
                TokenKind::Semicolon | TokenKind::Comma | TokenKind::End => {
                    self.advance();
                    break;
                }
                TokenKind::Axiom
                | TokenKind::Definition
                | TokenKind::Theorem
                | TokenKind::Lemma => break,
                _ => {
                    self.advance();
                }
            }
        }
    }
    /// Parse optional type annotation after an expression
    #[allow(dead_code)]
    fn parse_optional_type_ann(&mut self) -> Result<Option<Located<SurfaceExpr>>, ParseError> {
        if self.consume(TokenKind::Colon) {
            Ok(Some(self.parse_expr()?))
        } else {
            Ok(None)
        }
    }
    /// Parse an identifier.
    fn parse_ident(&mut self) -> Result<String, ParseError> {
        if let TokenKind::Ident(name) = &self.current().kind {
            let name = name.clone();
            self.advance();
            Ok(name)
        } else {
            Err(ParseError::unexpected(
                vec!["identifier".to_string()],
                self.current().kind.clone(),
                self.current().span.clone(),
            ))
        }
    }
}
/// Associativity of an operator.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Assoc {
    /// Left-associative
    Left,
    /// Right-associative
    Right,
}
