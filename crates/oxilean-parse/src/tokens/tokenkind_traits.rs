//! # TokenKind - Trait Implementations
//!
//! This module contains trait implementations for `TokenKind`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{StringPart, TokenKind};
use std::fmt;

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Axiom => write!(f, "axiom"),
            TokenKind::Definition => write!(f, "definition"),
            TokenKind::Theorem => write!(f, "theorem"),
            TokenKind::Lemma => write!(f, "lemma"),
            TokenKind::Fun => write!(f, "fun"),
            TokenKind::Forall => write!(f, "forall"),
            TokenKind::Arrow => write!(f, "->"),
            TokenKind::FatArrow => write!(f, "=>"),
            TokenKind::If => write!(f, "if"),
            TokenKind::Then => write!(f, "then"),
            TokenKind::Else => write!(f, "else"),
            TokenKind::Match => write!(f, "match"),
            TokenKind::With => write!(f, "with"),
            TokenKind::Do => write!(f, "do"),
            TokenKind::Have => write!(f, "have"),
            TokenKind::Suffices => write!(f, "suffices"),
            TokenKind::Show => write!(f, "show"),
            TokenKind::Where => write!(f, "where"),
            TokenKind::From => write!(f, "from"),
            TokenKind::By => write!(f, "by"),
            TokenKind::Return => write!(f, "return"),
            TokenKind::Ident(s) => write!(f, "{}", s),
            TokenKind::Nat(n) => write!(f, "{}", n),
            TokenKind::Float(v) => write!(f, "{}", v),
            TokenKind::String(s) => write!(f, "\"{}\"", s),
            TokenKind::Char(c) => write!(f, "'{}'", c),
            TokenKind::DocComment(s) => write!(f, "/-- {} -/", s),
            TokenKind::InterpolatedString(parts) => {
                write!(f, "s!\"")?;
                for part in parts {
                    match part {
                        StringPart::Literal(s) => write!(f, "{}", s)?,
                        StringPart::Interpolation(_) => write!(f, "{{...}}")?,
                    }
                }
                write!(f, "\"")
            }
            TokenKind::Eof => write!(f, "<eof>"),
            TokenKind::Error(msg) => write!(f, "<error: {}>", msg),
            TokenKind::LParen => write!(f, "("),
            TokenKind::RParen => write!(f, ")"),
            TokenKind::LBrace => write!(f, "{{"),
            TokenKind::RBrace => write!(f, "}}"),
            TokenKind::LBracket => write!(f, "["),
            TokenKind::RBracket => write!(f, "]"),
            TokenKind::LAngle => write!(f, "<"),
            TokenKind::RAngle => write!(f, ">"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::Semicolon => write!(f, ";"),
            TokenKind::Dot => write!(f, "."),
            TokenKind::DotDot => write!(f, ".."),
            TokenKind::Bar => write!(f, "|"),
            TokenKind::At => write!(f, "@"),
            TokenKind::Hash => write!(f, "#"),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Star => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::Percent => write!(f, "%"),
            TokenKind::Caret => write!(f, "^"),
            TokenKind::Assign => write!(f, ":="),
            TokenKind::AndAnd => write!(f, "&&"),
            TokenKind::OrOr => write!(f, "||"),
            TokenKind::BangEq => write!(f, "!="),
            TokenKind::Bang => write!(f, "!"),
            TokenKind::LeftArrow => write!(f, "<-"),
            TokenKind::Opaque => write!(f, "opaque"),
            TokenKind::Inductive => write!(f, "inductive"),
            TokenKind::Structure => write!(f, "structure"),
            TokenKind::Class => write!(f, "class"),
            TokenKind::Instance => write!(f, "instance"),
            TokenKind::Namespace => write!(f, "namespace"),
            TokenKind::Section => write!(f, "section"),
            TokenKind::Variable => write!(f, "variable"),
            TokenKind::Variables => write!(f, "variables"),
            TokenKind::Parameter => write!(f, "parameter"),
            TokenKind::Parameters => write!(f, "parameters"),
            TokenKind::Constant => write!(f, "constant"),
            TokenKind::Constants => write!(f, "constants"),
            TokenKind::End => write!(f, "end"),
            TokenKind::Import => write!(f, "import"),
            TokenKind::Export => write!(f, "export"),
            TokenKind::Open => write!(f, "open"),
            TokenKind::Attribute => write!(f, "attribute"),
            TokenKind::Type => write!(f, "Type"),
            TokenKind::Prop => write!(f, "Prop"),
            TokenKind::Sort => write!(f, "Sort"),
            TokenKind::Let => write!(f, "let"),
            TokenKind::In => write!(f, "in"),
            TokenKind::And => write!(f, "And"),
            TokenKind::Or => write!(f, "Or"),
            TokenKind::Not => write!(f, "Not"),
            TokenKind::Iff => write!(f, "Iff"),
            TokenKind::Exists => write!(f, "exists"),
            TokenKind::Eq => write!(f, "="),
            TokenKind::Ne => write!(f, "!="),
            TokenKind::Lt => write!(f, "<"),
            TokenKind::Le => write!(f, "<="),
            TokenKind::Gt => write!(f, ">"),
            TokenKind::Ge => write!(f, ">="),
            TokenKind::Underscore => write!(f, "_"),
            TokenKind::Question => write!(f, "?"),
        }
    }
}
