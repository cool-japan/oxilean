//! Types for the unification hint system.
//!
//! Unification hints are user-declared equations that guide the definitional
//! equality checker when structural comparison fails. They are analogous to
//! Lean 4's `@[unif_hint]` attribute.
//!
//! # Design
//!
//! A `UnifHint` encodes a conditional equation:
//! ```text
//! hypotheses ⊢ lhs ≡ rhs
//! ```
//! The checker tries to match both `lhs` and `rhs` against the pair being
//! checked, and if matching succeeds it emits proof obligations for every
//! hypothesis. If all hypotheses are definitionally equal (by recursion) the
//! hint fires and the pair is considered definitionally equal.

use crate::{Expr, Name};
use std::collections::HashMap;
use std::fmt;

// ── Pattern / matching ────────────────────────────────────────────────────────

/// A pattern used in unification hint matching.
///
/// Patterns are a subset of `Expr` where `Var(name)` represents a fresh
/// pattern variable that can match any sub-expression.  All other shapes
/// match structurally.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum HintPattern {
    /// A pattern variable: matches any expression and binds it to `name`.
    Var(Name),
    /// Match a `Sort` with a fixed level — we keep the full `Expr` here
    /// for simplicity, pattern-variables inside levels are not supported.
    Expr(Expr),
}

impl HintPattern {
    /// Construct a pattern variable.
    pub fn var(name: impl Into<Name>) -> Self {
        HintPattern::Var(name.into())
    }

    /// Construct a literal expression pattern (no pattern variables inside).
    pub fn expr(e: Expr) -> Self {
        HintPattern::Expr(e)
    }

    /// Returns `true` if this is a pattern variable.
    pub fn is_var(&self) -> bool {
        matches!(self, HintPattern::Var(_))
    }
}

impl fmt::Display for HintPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HintPattern::Var(n) => write!(f, "?{}", n),
            HintPattern::Expr(e) => write!(f, "{:?}", e),
        }
    }
}

// ── UnifHint ─────────────────────────────────────────────────────────────────

/// A single unification hint.
///
/// Encodes the conditional equation:
/// ```text
/// hypotheses ⊢ lhs ≡ rhs
/// ```
///
/// When the definitional equality checker is stuck comparing `t` and `s`,
/// it tries to match `(lhs, rhs)` (or `(rhs, lhs)`) against `(t, s)`.
/// If a pattern substitution `σ` is found, it checks every hypothesis
/// `σ(a) ≡ σ(b)` recursively.  If all succeed the hint fires.
#[derive(Clone, Debug)]
pub struct UnifHint {
    /// Optional human-readable name for this hint.
    pub name: Option<Name>,
    /// Left-hand side of the equation (may contain `HintPattern::Var` nodes
    /// but is stored as a plain `Expr` with free variables acting as
    /// pattern placeholders — see `UnifHintDB::find_hints`).
    pub lhs: Expr,
    /// Right-hand side of the equation.
    pub rhs: Expr,
    /// Conditional hypotheses: `(hyp_name, (lhs_hyp, rhs_hyp))`.
    /// Each pair must be definitionally equal under the matched substitution.
    pub hypotheses: Vec<(Name, (Expr, Expr))>,
    /// Priority: higher-priority hints are tried first.
    pub priority: i32,
}

impl UnifHint {
    /// Create a new unconditional hint `lhs ≡ rhs`.
    pub fn new(lhs: Expr, rhs: Expr) -> Self {
        Self {
            name: None,
            lhs,
            rhs,
            hypotheses: Vec::new(),
            priority: 0,
        }
    }

    /// Create a new conditional hint with hypotheses.
    pub fn with_hypotheses(lhs: Expr, rhs: Expr, hypotheses: Vec<(Name, (Expr, Expr))>) -> Self {
        Self {
            name: None,
            lhs,
            rhs,
            hypotheses,
            priority: 0,
        }
    }

    /// Attach a human-readable name.
    pub fn named(mut self, name: impl Into<Name>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set the priority (higher = tried first).
    pub fn with_priority(mut self, p: i32) -> Self {
        self.priority = p;
        self
    }

    /// Returns the number of hypotheses.
    pub fn hypothesis_count(&self) -> usize {
        self.hypotheses.len()
    }

    /// Returns `true` if this hint has no hypotheses (is unconditional).
    pub fn is_unconditional(&self) -> bool {
        self.hypotheses.is_empty()
    }
}

impl fmt::Display for UnifHint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref n) = self.name {
            write!(f, "[{}] ", n)?;
        }
        write!(f, "{:?} ≡ {:?}", self.lhs, self.rhs)?;
        if !self.hypotheses.is_empty() {
            write!(f, " where ")?;
            for (i, (hname, (hl, hr))) in self.hypotheses.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}: {:?} ≡ {:?}", hname, hl, hr)?;
            }
        }
        Ok(())
    }
}

// ── Match result ──────────────────────────────────────────────────────────────

/// The outcome of attempting to match a `UnifHint` against a pair of
/// expressions.
#[derive(Debug, Clone)]
pub enum HintMatchResult {
    /// Matching succeeded; the contained substitution maps pattern-variable
    /// names to concrete expressions.
    Matched(PatternSubst),
    /// Matching failed (structural mismatch or conflicting bindings).
    NoMatch,
}

impl HintMatchResult {
    /// Returns `true` if matching succeeded.
    pub fn is_match(&self) -> bool {
        matches!(self, HintMatchResult::Matched(_))
    }

    /// Unwraps the substitution, panicking if not a match.
    /// Only used in tests — production code uses `if let`.
    #[cfg(test)]
    pub fn unwrap_subst(self) -> PatternSubst {
        match self {
            HintMatchResult::Matched(s) => s,
            HintMatchResult::NoMatch => panic!("HintMatchResult::unwrap_subst called on NoMatch"),
        }
    }
}

// ── PatternSubst ──────────────────────────────────────────────────────────────

/// A substitution from pattern-variable names to concrete expressions,
/// produced during unification-hint matching.
#[derive(Clone, Debug, Default)]
pub struct PatternSubst {
    bindings: HashMap<String, Expr>,
}

impl PatternSubst {
    /// Create an empty substitution.
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    /// Try to bind `name → expr`.  Returns `false` if `name` is already
    /// bound to a *different* expression (conflict).
    pub fn bind(&mut self, name: &Name, expr: Expr) -> bool {
        let key = name.to_string();
        match self.bindings.get(&key) {
            Some(existing) if existing != &expr => false,
            _ => {
                self.bindings.insert(key, expr);
                true
            }
        }
    }

    /// Look up a binding.
    pub fn get(&self, name: &Name) -> Option<&Expr> {
        self.bindings.get(&name.to_string())
    }

    /// Apply this substitution to an expression: replace every `FVar(id)`
    /// whose string representation matches a bound pattern variable with the
    /// bound expression.  This is a best-effort structural walk.
    pub fn apply(&self, expr: &Expr) -> Expr {
        if self.bindings.is_empty() {
            return expr.clone();
        }
        apply_subst_expr(self, expr)
    }

    /// The number of bindings.
    pub fn len(&self) -> usize {
        self.bindings.len()
    }

    /// Returns `true` if the substitution is empty.
    pub fn is_empty(&self) -> bool {
        self.bindings.is_empty()
    }

    /// Iterate over all `(name_string, expr)` pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &Expr)> {
        self.bindings.iter().map(|(k, v)| (k.as_str(), v))
    }
}

/// Recursively apply a `PatternSubst` to an `Expr`.
fn apply_subst_expr(subst: &PatternSubst, expr: &Expr) -> Expr {
    match expr {
        // FVar: check if its ID string matches a binding
        Expr::FVar(id) => {
            let key = format!("{}", id.0);
            if let Some(replacement) = subst.bindings.get(&key) {
                return replacement.clone();
            }
            expr.clone()
        }
        // Const: check if the name matches a binding
        Expr::Const(n, levels) => {
            let key = n.to_string();
            if let Some(replacement) = subst.bindings.get(&key) {
                return replacement.clone();
            }
            Expr::Const(n.clone(), levels.clone())
        }
        Expr::App(f, a) => Expr::App(
            Box::new(apply_subst_expr(subst, f)),
            Box::new(apply_subst_expr(subst, a)),
        ),
        Expr::Lam(bi, n, ty, body) => Expr::Lam(
            *bi,
            n.clone(),
            Box::new(apply_subst_expr(subst, ty)),
            Box::new(apply_subst_expr(subst, body)),
        ),
        Expr::Pi(bi, n, ty, body) => Expr::Pi(
            *bi,
            n.clone(),
            Box::new(apply_subst_expr(subst, ty)),
            Box::new(apply_subst_expr(subst, body)),
        ),
        Expr::Let(n, ty, val, body) => Expr::Let(
            n.clone(),
            Box::new(apply_subst_expr(subst, ty)),
            Box::new(apply_subst_expr(subst, val)),
            Box::new(apply_subst_expr(subst, body)),
        ),
        Expr::Proj(n, idx, inner) => {
            Expr::Proj(n.clone(), *idx, Box::new(apply_subst_expr(subst, inner)))
        }
        // Leaf nodes: Sort, BVar, Lit — no substitution possible
        _ => expr.clone(),
    }
}

// ── UnifHintDB ────────────────────────────────────────────────────────────────

/// A registry of `UnifHint`s.
///
/// Hints are stored in priority order (highest first).  `find_hints` returns
/// every hint whose `(lhs, rhs)` or `(rhs, lhs)` can be structurally matched
/// against the query pair `(t, s)`.
#[derive(Clone, Debug, Default)]
pub struct UnifHintDB {
    hints: Vec<UnifHint>,
}

impl UnifHintDB {
    /// Create an empty database.
    pub fn new() -> Self {
        Self { hints: Vec::new() }
    }

    /// Add a hint, keeping the list sorted by priority (highest first).
    pub fn add_hint(&mut self, hint: UnifHint) {
        // Find insertion position (stable sort by descending priority)
        let pos = self
            .hints
            .iter()
            .position(|h| h.priority < hint.priority)
            .unwrap_or(self.hints.len());
        self.hints.insert(pos, hint);
    }

    /// Returns the total number of registered hints.
    pub fn len(&self) -> usize {
        self.hints.len()
    }

    /// Returns `true` if no hints are registered.
    pub fn is_empty(&self) -> bool {
        self.hints.is_empty()
    }

    /// Retrieve all hints (in priority order).
    pub fn all_hints(&self) -> &[UnifHint] {
        &self.hints
    }

    /// Find all hints that *structurally* match `(lhs_query, rhs_query)`.
    ///
    /// For each registered hint we try both orientations:
    /// 1. `hint.lhs` ↔ `lhs_query` AND `hint.rhs` ↔ `rhs_query`
    /// 2. `hint.rhs` ↔ `lhs_query` AND `hint.lhs` ↔ `rhs_query`
    ///
    /// "Structural match" means: every `Const` node in the hint pattern whose
    /// name begins with `?` is treated as a pattern variable (it may match any
    /// expression); every other node must match the corresponding node in the
    /// query structurally.
    ///
    /// Returns a list of `(hint_ref, substitution, swapped)` triples where
    /// `swapped` is `true` when orientation 2 was used.
    pub fn find_hints<'a>(
        &'a self,
        lhs_query: &Expr,
        rhs_query: &Expr,
    ) -> Vec<(&'a UnifHint, PatternSubst, bool)> {
        let mut results = Vec::new();
        for hint in &self.hints {
            // Try forward orientation
            let mut subst = PatternSubst::new();
            if match_expr_pattern(&hint.lhs, lhs_query, &mut subst)
                && match_expr_pattern(&hint.rhs, rhs_query, &mut subst)
            {
                results.push((hint, subst, false));
                continue;
            }
            // Try swapped orientation
            let mut subst2 = PatternSubst::new();
            if match_expr_pattern(&hint.rhs, lhs_query, &mut subst2)
                && match_expr_pattern(&hint.lhs, rhs_query, &mut subst2)
            {
                results.push((hint, subst2, true));
            }
        }
        results
    }

    /// Remove all hints with a given name.
    pub fn remove_named(&mut self, name: &Name) {
        self.hints.retain(|h| h.name.as_ref() != Some(name));
    }

    /// Remove all hints.
    pub fn clear(&mut self) {
        self.hints.clear();
    }
}

// ── Pattern matching ──────────────────────────────────────────────────────────

/// Match `pattern` against `target`, accumulating bindings in `subst`.
///
/// A `Const` node whose name string starts with `?` is treated as a pattern
/// variable.  All other nodes must match structurally.
///
/// Returns `false` on mismatch or conflicting bindings.
pub fn match_expr_pattern(pattern: &Expr, target: &Expr, subst: &mut PatternSubst) -> bool {
    match pattern {
        // Pattern variable: Const whose name starts with '?'
        Expr::Const(n, _) if n.to_string().starts_with('?') => {
            // Strip the leading '?'
            let var_key = n.to_string()[1..].to_string();
            let var_name = Name::str(&var_key);
            subst.bind(&var_name, target.clone())
        }
        // Structural cases
        Expr::Sort(lp) => matches!(target, Expr::Sort(lt) if lp == lt),
        Expr::BVar(ip) => matches!(target, Expr::BVar(it) if ip == it),
        Expr::FVar(fp) => matches!(target, Expr::FVar(ft) if fp == ft),
        Expr::Const(np, lsp) => {
            if let Expr::Const(nt, lst) = target {
                np == nt
                    && lsp.len() == lst.len()
                    && lsp.iter().zip(lst.iter()).all(|(lp, lt)| lp == lt)
            } else {
                false
            }
        }
        Expr::App(fp, ap) => {
            if let Expr::App(ft, at_) = target {
                match_expr_pattern(fp, ft, subst) && match_expr_pattern(ap, at_, subst)
            } else {
                false
            }
        }
        Expr::Lam(_, _, typ, bodyp) => {
            if let Expr::Lam(_, _, tyt, bodyt) = target {
                match_expr_pattern(typ, tyt, subst) && match_expr_pattern(bodyp, bodyt, subst)
            } else {
                false
            }
        }
        Expr::Pi(_, _, typ, bodyp) => {
            if let Expr::Pi(_, _, tyt, bodyt) = target {
                match_expr_pattern(typ, tyt, subst) && match_expr_pattern(bodyp, bodyt, subst)
            } else {
                false
            }
        }
        Expr::Let(_, typ, valp, bodyp) => {
            if let Expr::Let(_, tyt, valt, bodyt) = target {
                match_expr_pattern(typ, tyt, subst)
                    && match_expr_pattern(valp, valt, subst)
                    && match_expr_pattern(bodyp, bodyt, subst)
            } else {
                false
            }
        }
        Expr::Lit(lp) => matches!(target, Expr::Lit(lt) if lp == lt),
        Expr::Proj(np, ip, ep) => {
            if let Expr::Proj(nt, it, et) = target {
                np == nt && ip == it && match_expr_pattern(ep, et, subst)
            } else {
                false
            }
        }
    }
}
