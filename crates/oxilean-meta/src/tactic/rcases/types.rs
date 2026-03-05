//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use crate::basic::{MVarId, MetaContext, MetavarKind};
use crate::tactic::state::{TacticError, TacticResult, TacticState};
use oxilean_kernel::{Expr, Level, Name};
use std::collections::{HashSet, VecDeque};

/// Configuration for the `rcases` tactic.
#[derive(Clone, Debug)]
pub struct RcasesConfig {
    /// Maximum recursion depth for nested pattern destructuring.
    pub max_depth: usize,
    /// Whether to use constructor names for generated variable names.
    pub use_constructor_names: bool,
    /// Whether to automatically clear unused hypotheses.
    pub clear_unused: bool,
}
/// Result of applying the `rcases` tactic.
#[derive(Clone, Debug)]
pub struct RcasesResult {
    /// New goals produced by the destructuring.
    pub goals: Vec<MVarId>,
    /// Variable bindings introduced (name -> expression).
    pub bindings: Vec<(Name, Expr)>,
    /// Patterns that were actually used during destructuring.
    pub patterns_used: Vec<RcasesPattern>,
}
impl RcasesResult {
    /// Create an empty result.
    pub(super) fn empty() -> Self {
        Self {
            goals: Vec::new(),
            bindings: Vec::new(),
            patterns_used: Vec::new(),
        }
    }
    /// Merge another result into this one.
    pub(super) fn merge(&mut self, other: RcasesResult) {
        self.goals.extend(other.goals);
        self.bindings.extend(other.bindings);
        self.patterns_used.extend(other.patterns_used);
    }
    /// Number of new goals produced.
    pub fn num_goals(&self) -> usize {
        self.goals.len()
    }
    /// Number of variable bindings introduced.
    pub fn num_bindings(&self) -> usize {
        self.bindings.len()
    }
}
/// Result of applying the `obtain` tactic.
#[derive(Clone, Debug)]
pub struct ObtainResult {
    /// The goal for proving the obtained type.
    pub proof_goal: MVarId,
    /// Goals remaining after destructuring the obtained value.
    pub remaining_goals: Vec<MVarId>,
    /// Variable bindings introduced by the destructuring.
    pub bindings: Vec<(Name, Expr)>,
    /// Patterns that were used.
    pub patterns_used: Vec<RcasesPattern>,
}
/// A recursive destructuring pattern for `rcases`.
///
/// Patterns describe how an inductive value should be decomposed.
/// They form a tree: each node is a product or sum destructuring,
/// and leaves are variable bindings or discards.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RcasesPattern {
    /// Bind to a single variable name.
    ///
    /// Example: `x`
    One(String),
    /// Discard the value (underscore pattern).
    ///
    /// Example: `_`
    Clear,
    /// Product/tuple destructuring.
    ///
    /// Example: `⟨a, b, c⟩`
    Tuple(Vec<RcasesPattern>),
    /// Sum/alternative destructuring.
    ///
    /// Example: `a | b | c`
    Alts(Vec<RcasesPattern>),
    /// Typed pattern with an explicit type annotation.
    ///
    /// Example: `(p : Nat)`
    Typed(Box<RcasesPattern>, Expr),
    /// Nested rcases (recursive destructuring).
    ///
    /// Example: `⟨⟨p⟩⟩` means "destruct, then destruct again"
    Nested(Box<RcasesPattern>),
}
impl RcasesPattern {
    /// Create a single-variable pattern.
    pub fn one(name: impl Into<String>) -> Self {
        RcasesPattern::One(name.into())
    }
    /// Create a discard pattern.
    pub fn clear() -> Self {
        RcasesPattern::Clear
    }
    /// Create a tuple pattern from a list of sub-patterns.
    pub fn tuple(pats: Vec<RcasesPattern>) -> Self {
        RcasesPattern::Tuple(pats)
    }
    /// Create an alternatives pattern from a list of sub-patterns.
    pub fn alts(pats: Vec<RcasesPattern>) -> Self {
        RcasesPattern::Alts(pats)
    }
    /// Create a typed pattern.
    pub fn typed(pat: RcasesPattern, ty: Expr) -> Self {
        RcasesPattern::Typed(Box::new(pat), ty)
    }
    /// Create a nested pattern.
    pub fn nested(pat: RcasesPattern) -> Self {
        RcasesPattern::Nested(Box::new(pat))
    }
    /// Check if this pattern is a simple variable binding (no destructuring).
    pub fn is_simple(&self) -> bool {
        matches!(self, RcasesPattern::One(_) | RcasesPattern::Clear)
    }
    /// Check if this pattern contains any sum/alternative patterns.
    pub fn has_alts(&self) -> bool {
        match self {
            RcasesPattern::Alts(_) => true,
            RcasesPattern::Tuple(pats) => pats.iter().any(|p| p.has_alts()),
            RcasesPattern::Typed(p, _) => p.has_alts(),
            RcasesPattern::Nested(p) => p.has_alts(),
            RcasesPattern::One(_) | RcasesPattern::Clear => false,
        }
    }
    /// Count the number of leaf variable bindings in this pattern.
    pub fn count_bindings(&self) -> usize {
        match self {
            RcasesPattern::One(_) => 1,
            RcasesPattern::Clear => 0,
            RcasesPattern::Tuple(pats) => pats.iter().map(|p| p.count_bindings()).sum(),
            RcasesPattern::Alts(pats) => pats.iter().map(|p| p.count_bindings()).max().unwrap_or(0),
            RcasesPattern::Typed(p, _) => p.count_bindings(),
            RcasesPattern::Nested(p) => p.count_bindings(),
        }
    }
    /// Collect all variable names bound by this pattern.
    pub fn collect_names(&self) -> Vec<String> {
        let mut names = Vec::new();
        self.collect_names_impl(&mut names);
        names
    }
    pub(super) fn collect_names_impl(&self, names: &mut Vec<String>) {
        match self {
            RcasesPattern::One(name) => names.push(name.clone()),
            RcasesPattern::Clear => {}
            RcasesPattern::Tuple(pats) => {
                for p in pats {
                    p.collect_names_impl(names);
                }
            }
            RcasesPattern::Alts(pats) => {
                for p in pats {
                    p.collect_names_impl(names);
                }
            }
            RcasesPattern::Typed(p, _) => p.collect_names_impl(names),
            RcasesPattern::Nested(p) => p.collect_names_impl(names),
        }
    }
    /// Get the expected number of constructor fields for a tuple pattern.
    /// Returns None if the pattern is not a tuple.
    pub fn expected_fields(&self) -> Option<usize> {
        match self {
            RcasesPattern::Tuple(pats) => Some(pats.len()),
            _ => None,
        }
    }
    /// Get the number of alternatives in an alt pattern.
    /// Returns None if the pattern is not an alt.
    pub fn num_alts(&self) -> Option<usize> {
        match self {
            RcasesPattern::Alts(pats) => Some(pats.len()),
            _ => None,
        }
    }
    /// Flatten nested tuples into a single tuple.
    ///
    /// `⟨a, ⟨b, c⟩⟩` => `⟨a, b, c⟩` (if the inner type is a product too)
    pub fn flatten_tuple(&self) -> RcasesPattern {
        match self {
            RcasesPattern::Tuple(pats) => {
                let mut flat = Vec::new();
                for p in pats {
                    let fp = p.flatten_tuple();
                    if let RcasesPattern::Tuple(inner) = &fp {
                        if inner.len() == 1 {
                            flat.push(inner[0].clone());
                        } else {
                            flat.push(fp);
                        }
                    } else {
                        flat.push(fp);
                    }
                }
                RcasesPattern::Tuple(flat)
            }
            RcasesPattern::Alts(pats) => {
                RcasesPattern::Alts(pats.iter().map(|p| p.flatten_tuple()).collect())
            }
            RcasesPattern::Typed(p, ty) => {
                RcasesPattern::Typed(Box::new(p.flatten_tuple()), ty.clone())
            }
            RcasesPattern::Nested(p) => RcasesPattern::Nested(Box::new(p.flatten_tuple())),
            other => other.clone(),
        }
    }
    /// Pad a tuple pattern to have exactly `n` elements, filling with wildcards.
    pub fn pad_to(&self, n: usize) -> RcasesPattern {
        match self {
            RcasesPattern::Tuple(pats) => {
                let mut padded = pats.clone();
                while padded.len() < n {
                    padded.push(RcasesPattern::Clear);
                }
                padded.truncate(n);
                RcasesPattern::Tuple(padded)
            }
            _ => {
                let mut pats = vec![self.clone()];
                while pats.len() < n {
                    pats.push(RcasesPattern::Clear);
                }
                pats.truncate(n);
                RcasesPattern::Tuple(pats)
            }
        }
    }
    /// Check whether the pattern requires sum-type destructuring.
    pub fn requires_sum_destruct(&self) -> bool {
        matches!(self, RcasesPattern::Alts(_))
    }
    /// Check whether the pattern requires product-type destructuring.
    pub fn requires_product_destruct(&self) -> bool {
        matches!(self, RcasesPattern::Tuple(pats) if pats.len() > 1)
    }
}
/// Parser state for rcases pattern parsing.
pub(super) struct PatternParser {
    /// Input characters as a peekable iterator.
    pub(super) chars: Vec<char>,
    /// Current position in the input.
    pub(super) pos: usize,
}
impl PatternParser {
    pub(super) fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            pos: 0,
        }
    }
    /// Peek at the current character without consuming it.
    pub(super) fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }
    /// Consume the current character and advance.
    pub(super) fn advance(&mut self) -> Option<char> {
        let ch = self.chars.get(self.pos).copied();
        if ch.is_some() {
            self.pos += 1;
        }
        ch
    }
    /// Skip whitespace characters.
    pub(super) fn skip_ws(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }
    /// Check if we have reached the end of input.
    pub(super) fn at_end(&self) -> bool {
        self.pos >= self.chars.len()
    }
    /// Expect a specific character, return error if not found.
    pub(super) fn expect(&mut self, expected: char) -> Result<(), String> {
        self.skip_ws();
        match self.advance() {
            Some(ch) if ch == expected => Ok(()),
            Some(ch) => Err(format!(
                "expected '{}' at position {}, found '{}'",
                expected, self.pos, ch
            )),
            None => Err(format!(
                "expected '{}' at position {}, found end of input",
                expected, self.pos
            )),
        }
    }
    /// Parse a pattern at the top level (handles alternatives with `|`).
    pub(super) fn parse_top(&mut self) -> Result<RcasesPattern, String> {
        self.skip_ws();
        let first = self.parse_atom()?;
        self.skip_ws();
        if self.peek() == Some('|') {
            let mut alts = vec![first];
            while self.peek() == Some('|') {
                self.advance();
                self.skip_ws();
                let alt = self.parse_atom()?;
                alts.push(alt);
                self.skip_ws();
            }
            Ok(RcasesPattern::Alts(alts))
        } else {
            Ok(first)
        }
    }
    /// Parse a single atom: variable, underscore, or angle-bracket tuple.
    pub(super) fn parse_atom(&mut self) -> Result<RcasesPattern, String> {
        self.skip_ws();
        match self.peek() {
            None => Err("unexpected end of input".to_string()),
            Some('_') => {
                self.advance();
                Ok(RcasesPattern::Clear)
            }
            Some('\u{27E8}') | Some('<') => self.parse_tuple(),
            Some('(') => self.parse_paren(),
            Some(ch) if ch.is_alphanumeric() || ch == '\'' => self.parse_ident(),
            Some(ch) => Err(format!(
                "unexpected character '{}' at position {}",
                ch, self.pos
            )),
        }
    }
    /// Parse a tuple pattern: `⟨p₁, p₂, ...⟩` or `<p₁, p₂, ...>`.
    pub(super) fn parse_tuple(&mut self) -> Result<RcasesPattern, String> {
        let open = self
            .advance()
            .ok_or_else(|| "unexpected end of input in tuple pattern".to_string())?;
        let close = match open {
            '\u{27E8}' => '\u{27E9}',
            '<' => '>',
            _ => return Err(format!("unexpected tuple open '{}'", open)),
        };
        self.skip_ws();
        if self.peek() == Some(close) {
            self.advance();
            return Ok(RcasesPattern::Tuple(Vec::new()));
        }
        let mut pats = Vec::new();
        let first = self.parse_top()?;
        pats.push(first);
        self.skip_ws();
        while self.peek() == Some(',') {
            self.advance();
            self.skip_ws();
            if self.peek() == Some(close) {
                break;
            }
            let pat = self.parse_top()?;
            pats.push(pat);
            self.skip_ws();
        }
        self.skip_ws();
        if self.peek() == Some(close) {
            self.advance();
        } else {
            return Err(format!(
                "expected '{}' at position {}, found {:?}",
                close,
                self.pos,
                self.peek()
            ));
        }
        Ok(RcasesPattern::Tuple(pats))
    }
    /// Parse a parenthesized pattern or typed pattern: `(p)` or `(p : ty)`.
    pub(super) fn parse_paren(&mut self) -> Result<RcasesPattern, String> {
        self.expect('(')?;
        self.skip_ws();
        let inner = self.parse_top()?;
        self.skip_ws();
        if self.peek() == Some(':') {
            self.advance();
            self.skip_ws();
            let ty_name = self.parse_type_expr()?;
            self.skip_ws();
            self.expect(')')?;
            let ty = Expr::Const(Name::str(ty_name), vec![]);
            return Ok(RcasesPattern::Typed(Box::new(inner), ty));
        }
        self.expect(')')?;
        Ok(inner)
    }
    /// Parse an identifier (variable name).
    pub(super) fn parse_ident(&mut self) -> Result<RcasesPattern, String> {
        let mut name = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' || ch == '\'' || ch == '.' {
                name.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        if name.is_empty() {
            return Err(format!("expected identifier at position {}", self.pos));
        }
        if name == "_" {
            return Ok(RcasesPattern::Clear);
        }
        Ok(RcasesPattern::One(name))
    }
    /// Parse a type expression up to the closing `)` of a typed pattern `(p : ty)`.
    ///
    /// Handles complex types by tracking bracket depth, so nested parentheses
    /// and angle brackets are parsed correctly.
    pub(super) fn parse_type_expr(&mut self) -> Result<String, String> {
        let mut ty = String::new();
        let mut depth: i32 = 1;
        while let Some(ch) = self.peek() {
            match ch {
                ')' => {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                    ty.push(ch);
                    self.advance();
                }
                '(' | '⟨' | '<' | '[' | '{' => {
                    depth += 1;
                    ty.push(ch);
                    self.advance();
                }
                '⟩' | '>' | ']' | '}' => {
                    depth -= 1;
                    ty.push(ch);
                    self.advance();
                }
                _ => {
                    ty.push(ch);
                    self.advance();
                }
            }
        }
        let ty = ty.trim().to_string();
        if ty.is_empty() {
            return Err(format!("expected type expression at position {}", self.pos));
        }
        Ok(ty)
    }
}
/// Information about an inductive type used for rcases destructuring.
#[derive(Clone, Debug)]
pub(super) struct InductiveInfo {
    /// Name of the inductive type.
    pub(super) name: Name,
    /// Constructors with their field information.
    pub(super) constructors: Vec<ConstructorFieldInfo>,
    /// Whether this is a structure (single constructor).
    pub(super) is_structure: bool,
    /// Number of type parameters.
    pub(super) num_params: u32,
}
impl InductiveInfo {
    /// Check if this is a product-like type (single constructor with multiple fields).
    pub(super) fn is_product_like(&self) -> bool {
        self.constructors.len() == 1 && self.constructors[0].num_fields > 0
    }
    /// Check if this is a sum-like type (multiple constructors).
    pub(super) fn is_sum_like(&self) -> bool {
        self.constructors.len() > 1
    }
    /// Get the total number of constructors.
    pub(super) fn num_constructors(&self) -> usize {
        self.constructors.len()
    }
}
/// Internal context for rcases processing, tracking recursion depth and state.
pub(super) struct RcasesEngine {
    /// Configuration.
    pub(super) config: RcasesConfig,
    /// Current recursion depth.
    pub(super) depth: usize,
    /// Fresh variable counter for generating unique names.
    pub(super) fresh_counter: u64,
    /// Set of names already introduced (to avoid collisions).
    pub(super) used_names: HashSet<String>,
    /// Queue of pending destructuring operations.
    pub(super) pending: VecDeque<PendingDestruct>,
}
impl RcasesEngine {
    pub(super) fn new(config: RcasesConfig) -> Self {
        Self {
            depth: 0,
            fresh_counter: 0,
            used_names: HashSet::new(),
            pending: VecDeque::new(),
            config,
        }
    }
    /// Generate a fresh variable name that doesn't collide with existing names.
    pub(super) fn fresh_name(&mut self, base: &str) -> String {
        let mut candidate = base.to_string();
        while self.used_names.contains(&candidate) {
            self.fresh_counter += 1;
            candidate = format!("{}_{}", base, self.fresh_counter);
        }
        self.used_names.insert(candidate.clone());
        candidate
    }
    /// Register a name as used.
    pub(super) fn register_name(&mut self, name: &str) {
        self.used_names.insert(name.to_string());
    }
    /// Core engine: process a single destructuring step.
    ///
    /// Given a pattern and a target expression on a specific goal,
    /// performs one level of destructuring and returns the result.
    /// Nested patterns are queued for further processing.
    pub(super) fn process_one(
        &mut self,
        pattern: &RcasesPattern,
        target: &Expr,
        goal_id: MVarId,
        state: &mut TacticState,
        ctx: &mut MetaContext,
    ) -> TacticResult<RcasesResult> {
        if self.depth > self.config.max_depth {
            return Err(TacticError::Failed(format!(
                "rcases: recursion depth limit ({}) exceeded",
                self.config.max_depth
            )));
        }
        self.depth += 1;
        let result = match pattern {
            RcasesPattern::One(name) => {
                let clean_name = self.fresh_name(name);
                let binding = (Name::str(clean_name), target.clone());
                Ok(RcasesResult {
                    goals: vec![goal_id],
                    bindings: vec![binding],
                    patterns_used: vec![pattern.clone()],
                })
            }
            RcasesPattern::Clear => Ok(RcasesResult {
                goals: vec![goal_id],
                bindings: Vec::new(),
                patterns_used: vec![pattern.clone()],
            }),
            RcasesPattern::Tuple(pats) => self.process_tuple(pats, target, goal_id, state, ctx),
            RcasesPattern::Alts(alts) => self.process_alts(alts, target, goal_id, state, ctx),
            RcasesPattern::Typed(inner, ty) => {
                self.process_typed(inner, ty, target, goal_id, state, ctx)
            }
            RcasesPattern::Nested(inner) => self.process_nested(inner, target, goal_id, state, ctx),
        };
        self.depth -= 1;
        result
    }
    /// Process a tuple pattern by destructuring a product-like type.
    pub(super) fn process_tuple(
        &mut self,
        pats: &[RcasesPattern],
        target: &Expr,
        goal_id: MVarId,
        state: &mut TacticState,
        ctx: &mut MetaContext,
    ) -> TacticResult<RcasesResult> {
        let info = analyze_inductive_type(target, ctx).ok_or_else(|| {
            TacticError::Failed(format!(
                "rcases: cannot determine inductive type of target {:?}",
                target
            ))
        })?;
        if info.constructors.is_empty() {
            return self.process_empty_type(&info, goal_id, state, ctx);
        }
        if info.is_product_like() {
            let ctor = &info.constructors[0];
            let padded_pats = if pats.len() < ctor.num_fields as usize {
                let mut p = pats.to_vec();
                while p.len() < ctor.num_fields as usize {
                    p.push(RcasesPattern::Clear);
                }
                p
            } else {
                pats.to_vec()
            };
            self.destruct_product(ctor, &padded_pats, target, goal_id, state, ctx)
        } else {
            let ctor = &info.constructors[0].clone();
            let padded_pats = if pats.len() < ctor.num_fields as usize {
                let mut p = pats.to_vec();
                while p.len() < ctor.num_fields as usize {
                    p.push(RcasesPattern::Clear);
                }
                p
            } else {
                pats.to_vec()
            };
            self.destruct_product(ctor, &padded_pats, target, goal_id, state, ctx)
        }
    }
    /// Process an alternatives pattern by destructuring a sum-like type.
    pub(super) fn process_alts(
        &mut self,
        alts: &[RcasesPattern],
        target: &Expr,
        goal_id: MVarId,
        state: &mut TacticState,
        ctx: &mut MetaContext,
    ) -> TacticResult<RcasesResult> {
        let info = analyze_inductive_type(target, ctx).ok_or_else(|| {
            TacticError::Failed(format!(
                "rcases: cannot determine inductive type of target {:?} for alternatives",
                target
            ))
        })?;
        if info.constructors.is_empty() {
            return self.process_empty_type(&info, goal_id, state, ctx);
        }
        self.destruct_sum(&info, alts, target, goal_id, state, ctx)
    }
    /// Process a typed pattern: verify the type matches, then process inner.
    pub(super) fn process_typed(
        &mut self,
        inner: &RcasesPattern,
        expected_ty: &Expr,
        target: &Expr,
        goal_id: MVarId,
        state: &mut TacticState,
        ctx: &mut MetaContext,
    ) -> TacticResult<RcasesResult> {
        if let Expr::FVar(fv) = target {
            if let Some(ty) = ctx.get_fvar_type(*fv).cloned() {
                let mut def_eq = crate::def_eq::MetaDefEq::new();
                if def_eq.is_def_eq(&ty, expected_ty, ctx)
                    == crate::def_eq::UnificationResult::NotEqual
                {
                    return Err(TacticError::TypeMismatch {
                        expected: expected_ty.clone(),
                        got: ty,
                    });
                }
            }
        }
        self.process_one(inner, target, goal_id, state, ctx)
    }
    /// Process a nested pattern: apply rcases recursively.
    pub(super) fn process_nested(
        &mut self,
        inner: &RcasesPattern,
        target: &Expr,
        goal_id: MVarId,
        state: &mut TacticState,
        ctx: &mut MetaContext,
    ) -> TacticResult<RcasesResult> {
        self.process_one(inner, target, goal_id, state, ctx)
    }
    /// Handle a type with no constructors (e.g., False).
    /// The goal is solved by contradiction — no new goals are produced.
    pub(super) fn process_empty_type(
        &mut self,
        info: &InductiveInfo,
        goal_id: MVarId,
        _state: &mut TacticState,
        ctx: &mut MetaContext,
    ) -> TacticResult<RcasesResult> {
        let goal_ty = ctx
            .get_mvar_type(goal_id)
            .cloned()
            .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
        let rec_name = Name::str(format!("{}.rec", info.name));
        let proof = Expr::App(
            Box::new(Expr::Const(rec_name, vec![Level::zero()])),
            Box::new(Expr::Sort(Level::zero())),
        );
        ctx.assign_mvar(goal_id, proof);
        let _ = goal_ty;
        Ok(RcasesResult {
            goals: Vec::new(),
            bindings: Vec::new(),
            patterns_used: Vec::new(),
        })
    }
    /// Destructure a product-like type (single constructor) and apply field patterns.
    pub(super) fn destruct_product(
        &mut self,
        ctor: &ConstructorFieldInfo,
        field_pats: &[RcasesPattern],
        target: &Expr,
        goal_id: MVarId,
        state: &mut TacticState,
        ctx: &mut MetaContext,
    ) -> TacticResult<RcasesResult> {
        let goal_ty = ctx
            .get_mvar_type(goal_id)
            .cloned()
            .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
        let goal_ty = ctx.instantiate_mvars(&goal_ty);
        let num_fields = ctor.num_fields as usize;
        if num_fields == 0 {
            return Ok(RcasesResult {
                goals: vec![goal_id],
                bindings: Vec::new(),
                patterns_used: Vec::new(),
            });
        }
        let mut result = RcasesResult::empty();
        let mut field_bindings = Vec::new();
        let mut field_exprs = Vec::new();
        let field_types_for_ctor = get_ctor_field_types(&ctor.ctor_name, ctx);
        for i in 0..num_fields {
            let field_name = if i < field_pats.len() {
                match &field_pats[i] {
                    RcasesPattern::One(name) => self.fresh_name(name),
                    RcasesPattern::Clear => self.fresh_name(&format!("_f{}", i)),
                    _ => {
                        let base = if i < ctor.field_names.len() {
                            format!("{}", ctor.field_names[i])
                        } else {
                            format!("x_{}", i)
                        };
                        self.fresh_name(&base)
                    }
                }
            } else {
                self.fresh_name(&format!("_f{}", i))
            };
            let field_ty = field_types_for_ctor
                .get(i)
                .cloned()
                .unwrap_or_else(|| Expr::Sort(Level::zero()));
            let (_field_mvar, field_expr) = ctx.mk_fresh_expr_mvar(field_ty, MetavarKind::Natural);
            field_bindings.push((Name::str(field_name.clone()), field_expr.clone()));
            field_exprs.push((_field_mvar, field_expr));
        }
        let mut ctor_app = Expr::Const(ctor.ctor_name.clone(), vec![Level::zero()]);
        for (_, fexpr) in &field_exprs {
            ctor_app = Expr::App(Box::new(ctor_app), Box::new(fexpr.clone()));
        }
        let (new_goal_id, new_goal_expr) =
            ctx.mk_fresh_expr_mvar(goal_ty.clone(), MetavarKind::Natural);
        let cases_on_name = Name::str(format!(
            "{}.casesOn",
            get_head_const(target)
                .map(|n| format!("{}", n))
                .unwrap_or_else(|| "unknown".to_string())
        ));
        let match_proof = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(cases_on_name, vec![Level::zero()])),
                Box::new(target.clone()),
            )),
            Box::new(new_goal_expr),
        );
        ctx.assign_mvar(goal_id, match_proof);
        result.goals.push(new_goal_id);
        result.bindings.extend(field_bindings);
        for i in 0..num_fields {
            if i < field_pats.len() && !field_pats[i].is_simple() {
                let (_field_mvar, field_expr) = &field_exprs[i];
                self.pending.push_back(PendingDestruct {
                    goal_id: new_goal_id,
                    pattern: field_pats[i].clone(),
                    target: field_expr.clone(),
                    depth_remaining: self.config.max_depth - self.depth,
                });
            }
        }
        result
            .patterns_used
            .push(RcasesPattern::Tuple(field_pats.to_vec()));
        while let Some(pending) = self.pending.pop_front() {
            if pending.depth_remaining == 0 {
                continue;
            }
            let nested_result = self.process_one(
                &pending.pattern,
                &pending.target,
                pending.goal_id,
                state,
                ctx,
            )?;
            result.merge(nested_result);
        }
        Ok(result)
    }
    /// Destructure a sum-like type (multiple constructors) and apply alternative patterns.
    pub(super) fn destruct_sum(
        &mut self,
        info: &InductiveInfo,
        alts: &[RcasesPattern],
        target: &Expr,
        goal_id: MVarId,
        state: &mut TacticState,
        ctx: &mut MetaContext,
    ) -> TacticResult<RcasesResult> {
        let goal_ty = ctx
            .get_mvar_type(goal_id)
            .cloned()
            .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
        let goal_ty = ctx.instantiate_mvars(&goal_ty);
        let _num_ctors = info.num_constructors();
        let aligned = align_pattern_with_constructors(&RcasesPattern::Alts(alts.to_vec()), info);
        let mut result = RcasesResult::empty();
        let mut case_goals = Vec::new();
        for (i, ctor) in info.constructors.iter().enumerate() {
            let (case_id, _case_expr) =
                ctx.mk_fresh_expr_mvar(goal_ty.clone(), MetavarKind::Natural);
            case_goals.push(case_id);
            let case_pat = if i < aligned.len() {
                &aligned[i]
            } else {
                &RcasesPattern::Clear
            };
            if ctor.num_fields > 0 {
                let field_names = generate_field_names(case_pat, ctor, &self.config);
                for fname in field_names.iter() {
                    let fname_str = format!("{}", fname);
                    self.register_name(&fname_str);
                    let field_expr = Expr::Const(fname.clone(), vec![]);
                    result.bindings.push((fname.clone(), field_expr));
                }
            }
            if !case_pat.is_simple() && ctor.num_fields > 0 {
                let sub_pats = match case_pat {
                    RcasesPattern::Tuple(pats) => pats.clone(),
                    _ => vec![case_pat.clone()],
                };
                for (j, sp) in sub_pats.iter().enumerate() {
                    if !sp.is_simple() && j < ctor.num_fields as usize {
                        let field_name = if j < ctor.field_names.len() {
                            &ctor.field_names[j]
                        } else {
                            &Name::str(format!("x_{}", j))
                        };
                        self.pending.push_back(PendingDestruct {
                            goal_id: case_id,
                            pattern: sp.clone(),
                            target: Expr::Const(field_name.clone(), vec![]),
                            depth_remaining: self.config.max_depth - self.depth,
                        });
                    }
                }
            }
        }
        let cases_on_name = Name::str(format!("{}.casesOn", info.name));
        let mut proof = Expr::Const(cases_on_name, vec![Level::zero()]);
        proof = Expr::App(Box::new(proof), Box::new(target.clone()));
        for case_id in case_goals.iter() {
            let case_ref = Expr::FVar(oxilean_kernel::FVarId::new(
                case_id.0 + crate::basic::MVAR_FVAR_OFFSET,
            ));
            proof = Expr::App(Box::new(proof), Box::new(case_ref));
        }
        ctx.assign_mvar(goal_id, proof);
        result.goals.extend(case_goals);
        result
            .patterns_used
            .push(RcasesPattern::Alts(alts.to_vec()));
        while let Some(pending) = self.pending.pop_front() {
            if pending.depth_remaining == 0 {
                continue;
            }
            let nested_result = self.process_one(
                &pending.pattern,
                &pending.target,
                pending.goal_id,
                state,
                ctx,
            )?;
            result.merge(nested_result);
        }
        Ok(result)
    }
}
/// Information about a constructor's fields, used for pattern matching.
#[derive(Clone, Debug)]
pub(super) struct ConstructorFieldInfo {
    /// Name of the constructor (e.g., `And.intro`).
    pub(super) ctor_name: Name,
    /// Number of fields in the constructor.
    pub(super) num_fields: u32,
    /// Names of the fields (if known).
    pub(super) field_names: Vec<Name>,
    /// Whether this is a recursive constructor (contains the inductive type).
    pub(super) is_recursive: bool,
}
/// Determine the "kind" of an inductive type for pattern matching purposes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum InductiveKind {
    /// Product-like: single constructor with multiple fields (And, Prod, Sigma, etc.)
    Product,
    /// Sum-like: multiple constructors (Or, Nat, List, etc.)
    Sum,
    /// Unit-like: single constructor with no fields (True, Unit)
    Unit,
    /// Empty: no constructors (False)
    Empty,
}
/// A pending destructuring operation in the rcases work queue.
#[derive(Clone, Debug)]
struct PendingDestruct {
    /// The goal where this destructuring should happen.
    pub(super) goal_id: MVarId,
    /// The pattern to apply.
    pub(super) pattern: RcasesPattern,
    /// The target expression to destructure.
    pub(super) target: Expr,
    /// Remaining depth budget.
    pub(super) depth_remaining: usize,
}
