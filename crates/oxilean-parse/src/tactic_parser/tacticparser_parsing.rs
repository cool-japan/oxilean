//! # TacticParser - parsing Methods
//!
//! This module contains method implementations for `TacticParser`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::{ParseError, ParseErrorKind, Span, Token, TokenKind};

use super::types::{CalcStep, CaseArm, ConvSide, RewriteRule, TacticExpr};

use super::tacticparser_type::TacticParser;

impl<'a> TacticParser<'a> {
    /// Create a new tactic parser.
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, pos: 0 }
    }
    /// Parse `by <tactic>` or `by { t1; t2; ... }`.
    pub fn parse_by_block(&mut self) -> Result<TacticExpr, ParseError> {
        if !self.check_keyword("by") {
            return Err(ParseError::new(
                ParseErrorKind::InvalidSyntax("expected 'by'".to_string()),
                self.current_span(),
            ));
        }
        self.advance();
        if self.check_op("{") {
            self.parse_brace_block()
        } else {
            self.parse_seq()
        }
    }
    pub(super) fn parse_seq(&mut self) -> Result<TacticExpr, ParseError> {
        let mut left = self.parse_all_goals()?;
        while self.check_op(";") {
            self.advance();
            let right = self.parse_all_goals()?;
            left = TacticExpr::Seq(Box::new(left), Box::new(right));
        }
        Ok(left)
    }
    #[allow(dead_code)]
    pub(super) fn parse_all_goals(&mut self) -> Result<TacticExpr, ParseError> {
        let mut left = self.parse_alt()?;
        while self.check_op("<;>") {
            self.advance();
            let right = self.parse_alt()?;
            left = TacticExpr::AllGoals(Box::new(left), Box::new(right));
        }
        Ok(left)
    }
    pub(super) fn parse_alt(&mut self) -> Result<TacticExpr, ParseError> {
        let mut left = self.parse_primary()?;
        while self.check_op("<|>") {
            self.advance();
            let right = self.parse_primary()?;
            left = TacticExpr::Alt(Box::new(left), Box::new(right));
        }
        Ok(left)
    }
    pub(super) fn parse_primary(&mut self) -> Result<TacticExpr, ParseError> {
        if self.check_op("{") {
            return self.parse_brace_block();
        }
        if self.check_keyword("repeat") {
            self.advance();
            let inner = self.parse_primary()?;
            return Ok(TacticExpr::Repeat(Box::new(inner)));
        }
        if self.check_keyword("try") {
            self.advance();
            let inner = self.parse_primary()?;
            return Ok(TacticExpr::Try(Box::new(inner)));
        }
        if self.check_keyword("focus") {
            self.advance();
            let n = self.expect_nat()?;
            let inner = self.parse_primary()?;
            return Ok(TacticExpr::Focus(n, Box::new(inner)));
        }
        if self.check_keyword("all") {
            self.advance();
            let inner = self.parse_primary()?;
            return Ok(TacticExpr::All(Box::new(inner)));
        }
        if self.check_keyword("any") {
            self.advance();
            let inner = self.parse_primary()?;
            return Ok(TacticExpr::Any(Box::new(inner)));
        }
        if self.check_keyword("intro") {
            return self.parse_intro();
        }
        if self.check_keyword("intros") {
            return self.parse_intros();
        }
        if self.check_keyword("apply") {
            return self.parse_apply();
        }
        if self.check_keyword("exact") {
            return self.parse_exact();
        }
        if self.check_keyword("generalize") {
            return self.parse_generalize();
        }
        if self.check_keyword("obtain") {
            return self.parse_obtain();
        }
        if self.check_keyword("rcases") {
            return self.parse_rcases();
        }
        if self.check_keyword("first") {
            return self.parse_first();
        }
        if self.check_keyword("tauto") {
            return self.parse_tauto();
        }
        if self.check_keyword("ac_rfl") {
            return self.parse_ac_rfl();
        }
        if self.check_keyword("rewrite") || self.check_keyword("rw") {
            return self.parse_rewrite();
        }
        if self.check_keyword("simp") {
            return self.parse_simp();
        }
        if self.check_keyword("cases") {
            return self.parse_cases();
        }
        if self.check_keyword("induction") {
            return self.parse_induction();
        }
        if self.check_keyword("have") {
            return self.parse_have();
        }
        if self.check_keyword("let") {
            return self.parse_let();
        }
        if self.check_keyword("show") {
            return self.parse_show();
        }
        if self.check_keyword("suffices") {
            return self.parse_suffices();
        }
        if self.check_keyword("calc") {
            return self.parse_calc();
        }
        if self.check_keyword("conv_lhs") {
            self.advance();
            self.expect_op("=>")?;
            let inner = self.parse_primary()?;
            return Ok(TacticExpr::Conv(ConvSide::Lhs, Box::new(inner)));
        }
        if self.check_keyword("conv_rhs") {
            self.advance();
            self.expect_op("=>")?;
            let inner = self.parse_primary()?;
            return Ok(TacticExpr::Conv(ConvSide::Rhs, Box::new(inner)));
        }
        if self.check_keyword("conv") {
            return self.parse_conv();
        }
        if self.check_keyword("omega") {
            self.advance();
            return Ok(TacticExpr::Omega);
        }
        if self.check_keyword("ring") {
            self.advance();
            return Ok(TacticExpr::Ring);
        }
        if self.check_keyword("decide") {
            self.advance();
            return Ok(TacticExpr::Decide);
        }
        if self.check_keyword("norm_num") {
            self.advance();
            return Ok(TacticExpr::NormNum);
        }
        if self.check_keyword("constructor") {
            self.advance();
            return Ok(TacticExpr::Constructor);
        }
        if self.check_keyword("left") {
            self.advance();
            return Ok(TacticExpr::Left);
        }
        if self.check_keyword("right") {
            self.advance();
            return Ok(TacticExpr::Right);
        }
        if self.check_keyword("existsi") || self.check_keyword("use") {
            return self.parse_existsi();
        }
        if self.check_keyword("clear") {
            return self.parse_clear();
        }
        if self.check_keyword("revert") {
            return self.parse_revert();
        }
        if self.check_keyword("subst") {
            return self.parse_subst();
        }
        if self.check_keyword("contradiction") {
            self.advance();
            return Ok(TacticExpr::Contradiction);
        }
        if self.check_keyword("exfalso") {
            self.advance();
            return Ok(TacticExpr::Exfalso);
        }
        if self.check_keyword("by_contra") {
            return self.parse_by_contra();
        }
        if self.check_keyword("assumption") {
            self.advance();
            return Ok(TacticExpr::Assumption);
        }
        if self.check_keyword("trivial") {
            self.advance();
            return Ok(TacticExpr::Trivial);
        }
        if self.check_keyword("rfl") {
            self.advance();
            return Ok(TacticExpr::Rfl);
        }
        let name = self.expect_ident()?;
        if self.check_op("(") {
            self.advance();
            let mut args = Vec::new();
            while !self.check_op(")") {
                args.push(self.expect_ident()?);
                if !self.check_op(",") {
                    break;
                }
                self.advance();
            }
            self.expect_op(")")?;
            Ok(TacticExpr::WithArgs(name, args))
        } else {
            Ok(TacticExpr::Basic(name))
        }
    }
    /// Parse `intro x y z ...`
    pub(super) fn parse_intro(&mut self) -> Result<TacticExpr, ParseError> {
        self.advance();
        let mut names = Vec::new();
        while let Some(token) = self.current() {
            match &token.kind {
                TokenKind::Ident(s) if !self.is_tactic_terminator(s) => {
                    names.push(s.clone());
                    self.advance();
                }
                TokenKind::Underscore => {
                    names.push("_".to_string());
                    self.advance();
                }
                _ => break,
            }
        }
        Ok(TacticExpr::Intro(names))
    }
    /// Parse the rewrite argument list: `[lem1, <-lem2, ...]`
    pub(super) fn parse_rewrite_args(&mut self) -> Result<Vec<RewriteRule>, ParseError> {
        self.expect_op("[")?;
        let mut rules = Vec::new();
        if self.check_op("]") {
            self.advance();
            return Ok(rules);
        }
        loop {
            let reverse = if self.check_op("<-") {
                self.advance();
                true
            } else {
                false
            };
            let lemma = self.expect_ident()?;
            rules.push(RewriteRule { lemma, reverse });
            if !self.check_op(",") {
                break;
            }
            self.advance();
        }
        self.expect_op("]")?;
        Ok(rules)
    }
    /// Parse optional `[lem1, lem2, *]` for simp.
    pub(super) fn parse_simp_lemma_list(&mut self) -> Result<Vec<String>, ParseError> {
        if !self.check_op("[") {
            return Ok(Vec::new());
        }
        self.advance();
        let mut lemmas = Vec::new();
        if self.check_op("]") {
            self.advance();
            return Ok(lemmas);
        }
        loop {
            if self.check_op("*") {
                lemmas.push("*".to_string());
                self.advance();
            } else if self.check_op("-") {
                self.advance();
                let name = self.expect_ident()?;
                lemmas.push(format!("-{}", name));
            } else {
                let name = self.expect_ident()?;
                lemmas.push(name);
            }
            if !self.check_op(",") {
                break;
            }
            self.advance();
        }
        self.expect_op("]")?;
        Ok(lemmas)
    }
    /// Parse optional `{ key := val, ... }` for simp config.
    pub(super) fn parse_simp_config(&mut self) -> Result<Vec<(String, String)>, ParseError> {
        if !self.check_op("{") {
            return Ok(Vec::new());
        }
        self.advance();
        let mut config = Vec::new();
        if self.check_op("}") {
            self.advance();
            return Ok(config);
        }
        loop {
            let key = self.expect_ident()?;
            self.expect_op(":=")?;
            let val = self.expect_ident()?;
            config.push((key, val));
            if !self.check_op(",") {
                break;
            }
            self.advance();
        }
        self.expect_op("}")?;
        Ok(config)
    }
    /// Parse `cases x with | c1 a b => t1 | c2 => t2`
    pub(super) fn parse_cases(&mut self) -> Result<TacticExpr, ParseError> {
        self.advance();
        let target = self.expect_ident()?;
        let arms = if self.check_keyword("with") {
            self.advance();
            self.parse_case_arms()?
        } else {
            Vec::new()
        };
        Ok(TacticExpr::Cases(target, arms))
    }
    /// Parse `induction x with | c a b => t1 | c2 => t2`
    pub(super) fn parse_induction(&mut self) -> Result<TacticExpr, ParseError> {
        self.advance();
        let target = self.expect_ident()?;
        let arms = if self.check_keyword("with") {
            self.advance();
            self.parse_case_arms()?
        } else {
            Vec::new()
        };
        Ok(TacticExpr::Induction(target, arms))
    }
    /// Parse `| name pat1 pat2 => tactic` arms.
    pub(super) fn parse_case_arms(&mut self) -> Result<Vec<CaseArm>, ParseError> {
        let mut arms = Vec::new();
        while self.check_op("|") {
            self.advance();
            let name = self.expect_ident()?;
            let mut bindings = Vec::new();
            while !self.check_op("=>") && !self.at_end() {
                if let Some(token) = self.current() {
                    if let TokenKind::Ident(s) = &token.kind {
                        bindings.push(s.clone());
                        self.advance();
                    } else if token.kind == TokenKind::Underscore {
                        bindings.push("_".to_string());
                        self.advance();
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            self.expect_op("=>")?;
            let tactic = self.parse_seq()?;
            arms.push(CaseArm {
                name,
                bindings,
                tactic,
            });
        }
        Ok(arms)
    }
    /// Parse `have h : T := by tactic` or `have h := by tactic`.
    pub(super) fn parse_have(&mut self) -> Result<TacticExpr, ParseError> {
        self.advance();
        let name = self.expect_ident()?;
        let ty = if self.check_op(":") {
            self.advance();
            let t = self.expect_ident()?;
            Some(t)
        } else {
            None
        };
        if self.check_op(":=") {
            self.advance();
        }
        let body = if self.check_keyword("by") {
            self.parse_by_block()?
        } else {
            self.parse_primary()?
        };
        Ok(TacticExpr::Have(name, ty, Box::new(body)))
    }
    /// Parse `suffices h : T by tactic`.
    pub(super) fn parse_suffices(&mut self) -> Result<TacticExpr, ParseError> {
        self.advance();
        let name = self.expect_ident()?;
        if self.check_op(":") {
            self.advance();
            let _ty = self.expect_ident()?;
        }
        let body = if self.check_keyword("by") {
            self.parse_by_block()?
        } else {
            self.parse_primary()?
        };
        Ok(TacticExpr::Suffices(name, Box::new(body)))
    }
    /// Parse calc steps: `_ rel rhs := by justification`.
    pub(super) fn parse_calc_steps(&mut self) -> Result<Vec<CalcStep>, ParseError> {
        let mut steps = Vec::new();
        while self.check_op("_") || self.check_keyword("_") {
            self.advance();
            let relation = self.expect_ident()?;
            let rhs = self.expect_ident()?;
            self.expect_op(":=")?;
            let justification = if self.check_keyword("by") {
                self.parse_by_block()?
            } else {
                self.parse_primary()?
            };
            steps.push(CalcStep {
                relation,
                rhs,
                justification,
            });
        }
        Ok(steps)
    }
    /// Parse `conv => tactic` (with optional side specification).
    pub(super) fn parse_conv(&mut self) -> Result<TacticExpr, ParseError> {
        self.advance();
        let side = if self.check_keyword("lhs") {
            self.advance();
            ConvSide::Lhs
        } else if self.check_keyword("rhs") {
            self.advance();
            ConvSide::Rhs
        } else {
            ConvSide::Lhs
        };
        self.expect_op("=>")?;
        let inner = self.parse_primary()?;
        Ok(TacticExpr::Conv(side, Box::new(inner)))
    }
    /// Parse `first | t1 | t2 | t3`.
    #[allow(dead_code)]
    pub(super) fn parse_first(&mut self) -> Result<TacticExpr, ParseError> {
        self.advance();
        let mut tactics = Vec::new();
        while self.check_op("|") {
            self.advance();
            tactics.push(self.parse_primary()?);
        }
        if tactics.is_empty() {
            tactics.push(self.parse_primary()?);
        }
        Ok(TacticExpr::First(tactics))
    }
    /// Parse `intros` (multiple intro)
    #[allow(dead_code)]
    pub(super) fn parse_intros(&mut self) -> Result<TacticExpr, ParseError> {
        self.advance();
        let mut names = Vec::new();
        while let Some(token) = self.current() {
            match &token.kind {
                TokenKind::Ident(s) if !self.is_tactic_terminator(s) => {
                    names.push(s.clone());
                    self.advance();
                }
                TokenKind::Underscore => {
                    names.push("_".to_string());
                    self.advance();
                }
                _ => break,
            }
        }
        Ok(TacticExpr::Intros(names))
    }
    /// Parse `obtain h, ... => tactic`.
    #[allow(dead_code)]
    pub(super) fn parse_obtain(&mut self) -> Result<TacticExpr, ParseError> {
        self.advance();
        let name = self.expect_ident()?;
        let body = if self.check_op(",") || self.check_op("=>") {
            self.advance();
            self.parse_primary()?
        } else {
            self.parse_primary()?
        };
        Ok(TacticExpr::Obtain(name, Box::new(body)))
    }
    /// Parse `rcases h with | c1 => ... | c2 => ...`.
    #[allow(dead_code)]
    pub(super) fn parse_rcases(&mut self) -> Result<TacticExpr, ParseError> {
        self.advance();
        let target = self.expect_ident()?;
        let mut patterns = Vec::new();
        if self.check_keyword("with") {
            self.advance();
            while self.check_op("|") {
                self.advance();
                let pat = self.expect_ident()?;
                patterns.push(pat);
                if self.check_op("=>") {
                    self.advance();
                    let _ = self.parse_primary();
                }
            }
        }
        Ok(TacticExpr::Rcases(target, patterns))
    }
    /// Parse a `{ t1; t2; ... }` block.
    pub(super) fn parse_brace_block(&mut self) -> Result<TacticExpr, ParseError> {
        self.expect_op("{")?;
        let mut tactics = Vec::new();
        while !self.check_op("}") && !self.at_end() {
            let t = self.parse_seq()?;
            tactics.push(t);
            let _ = self.consume_op(";");
        }
        self.expect_op("}")?;
        Ok(TacticExpr::Block(tactics))
    }
    /// Parse a list of identifiers (until we hit a non-ident).
    pub(super) fn parse_ident_list(&mut self) -> Result<Vec<String>, ParseError> {
        let mut names = Vec::new();
        while let Some(token) = self.current() {
            if let TokenKind::Ident(s) = &token.kind {
                if self.is_tactic_terminator(s) {
                    break;
                }
                names.push(s.clone());
                self.advance();
            } else {
                break;
            }
        }
        Ok(names)
    }
    pub(super) fn expect_ident(&mut self) -> Result<String, ParseError> {
        if let Some(token) = self.current() {
            if let TokenKind::Ident(s) = &token.kind {
                let result = s.clone();
                self.advance();
                return Ok(result);
            }
        }
        Err(ParseError::new(
            ParseErrorKind::InvalidSyntax("expected identifier".to_string()),
            self.current_span(),
        ))
    }
    pub(super) fn expect_nat(&mut self) -> Result<usize, ParseError> {
        if let Some(token) = self.current() {
            if let TokenKind::Nat(n) = &token.kind {
                let result = *n as usize;
                self.advance();
                return Ok(result);
            }
        }
        Err(ParseError::new(
            ParseErrorKind::InvalidSyntax("expected number".to_string()),
            self.current_span(),
        ))
    }
    pub(super) fn expect_op(&mut self, op: &str) -> Result<(), ParseError> {
        if self.check_op(op) {
            self.advance();
            Ok(())
        } else {
            Err(ParseError::new(
                ParseErrorKind::InvalidSyntax(format!("expected '{}'", op)),
                self.current_span(),
            ))
        }
    }
    pub(super) fn current_span(&self) -> Span {
        if let Some(token) = self.current() {
            token.span.clone()
        } else {
            Span::new(0, 0, 0, 0)
        }
    }
}
