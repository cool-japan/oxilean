//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::UsageMap;
use super::functions::*;

/// Strategy for β-reduction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Strategy {
    /// Normal order (leftmost-outermost).
    NormalOrder,
    /// Applicative order (leftmost-innermost / call-by-value).
    ApplicativeOrder,
    /// Head reduction (reduce only the head redex).
    HeadReduction,
}
/// Check α-equivalence of two terms (de Bruijn terms are equal up to renaming
/// of bound variables by definition — so alpha-equivalence is just structural equality
/// after normalization of bound variable names, which in de Bruijn form is plain equality).
#[allow(dead_code)]
pub struct AlphaEquivalenceChecker;
#[allow(dead_code)]
impl AlphaEquivalenceChecker {
    /// Create a new checker.
    pub fn new() -> Self {
        AlphaEquivalenceChecker
    }
    /// Check whether two terms are alpha-equivalent.
    /// With de Bruijn indices, alpha-equivalence is structural equality.
    pub fn alpha_equiv(&self, t1: &Term, t2: &Term) -> bool {
        t1 == t2
    }
    /// Check alpha-equivalence modulo one layer of beta-reduction.
    /// Useful when terms may differ by a single administrative redex.
    pub fn alpha_equiv_after_head_step(&self, t1: &Term, t2: &Term) -> bool {
        if self.alpha_equiv(t1, t2) {
            return true;
        }
        let s1 = beta_step(t1, Strategy::HeadReduction).unwrap_or_else(|| t1.clone());
        let s2 = beta_step(t2, Strategy::HeadReduction).unwrap_or_else(|| t2.clone());
        self.alpha_equiv(&s1, &s2)
    }
    /// Check whether two terms are alpha-equivalent after full normalization
    /// (with a step limit).
    pub fn alpha_equiv_normalized(&self, t1: &Term, t2: &Term, limit: usize) -> bool {
        let (n1, _) = t1.normalize(limit);
        let (n2, _) = t2.normalize(limit);
        self.alpha_equiv(&n1, &n2)
    }
}
/// A β-reducer with configurable strategy and step budget.
#[allow(dead_code)]
pub struct BetaReducer {
    strategy: Strategy,
    max_steps: usize,
}
#[allow(dead_code)]
impl BetaReducer {
    /// Create a new reducer with the given strategy and step limit.
    pub fn new(strategy: Strategy, max_steps: usize) -> Self {
        BetaReducer {
            strategy,
            max_steps,
        }
    }
    /// Reduce the term to normal form under the configured strategy.
    /// Returns `(result, steps_taken, converged)`.
    pub fn reduce(&self, term: &Term) -> (Term, usize, bool) {
        let mut current = term.clone();
        let mut steps = 0;
        loop {
            if steps >= self.max_steps {
                return (current, steps, false);
            }
            match beta_step(&current, self.strategy) {
                None => return (current, steps, true),
                Some(next) => {
                    current = next;
                    steps += 1;
                }
            }
        }
    }
    /// Check if a term is already in normal form under this strategy.
    pub fn is_normal_form(&self, term: &Term) -> bool {
        beta_step(term, self.strategy).is_none()
    }
    /// Count reduction steps without storing intermediate terms.
    pub fn count_steps(&self, term: &Term) -> Option<usize> {
        let (_, steps, converged) = self.reduce(term);
        if converged {
            Some(steps)
        } else {
            None
        }
    }
}
/// A simple type: base or arrow.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimpleType {
    /// A base type with a name (e.g., "o", "ι").
    Base(String),
    /// Function type A → B.
    Arrow(Box<SimpleType>, Box<SimpleType>),
}
impl SimpleType {
    /// Arrow type constructor (convenience).
    pub fn arr(a: SimpleType, b: SimpleType) -> Self {
        SimpleType::Arrow(Box::new(a), Box::new(b))
    }
}
/// A typing context: list of types for de Bruijn variables.
/// Index 0 = innermost binder.
#[derive(Debug, Clone)]
pub struct Context(pub Vec<SimpleType>);
impl Context {
    /// Empty context.
    pub fn empty() -> Self {
        Context(vec![])
    }
    /// Extend the context with a new type (push to front = index 0).
    pub fn extend(&self, ty: SimpleType) -> Context {
        let mut v = vec![ty];
        v.extend(self.0.clone());
        Context(v)
    }
    /// Look up the type of de Bruijn variable `k`.
    pub fn get(&self, k: usize) -> Option<&SimpleType> {
        self.0.get(k)
    }
}
/// Checks compatibility (duality) between two session types.
#[allow(dead_code)]
pub struct SessionTypeCompatibility;
#[allow(dead_code)]
impl SessionTypeCompatibility {
    /// Create a new compatibility checker.
    pub fn new() -> Self {
        SessionTypeCompatibility
    }
    /// Compute the dual of a session type (swap sends↔receives, select↔offer).
    pub fn dual(&self, s: &BinarySession) -> BinarySession {
        match s {
            BinarySession::Send(label, cont) => {
                BinarySession::Recv(label.clone(), Box::new(self.dual(cont)))
            }
            BinarySession::Recv(label, cont) => {
                BinarySession::Send(label.clone(), Box::new(self.dual(cont)))
            }
            BinarySession::Select(branches) => {
                let dual_branches: Vec<(String, BinarySession)> = branches
                    .iter()
                    .map(|(l, s)| (l.clone(), self.dual(s)))
                    .collect();
                BinarySession::Offer(dual_branches)
            }
            BinarySession::Offer(branches) => {
                let dual_branches: Vec<(String, BinarySession)> = branches
                    .iter()
                    .map(|(l, s)| (l.clone(), self.dual(s)))
                    .collect();
                BinarySession::Select(dual_branches)
            }
            BinarySession::Rec(x, body) => BinarySession::Rec(x.clone(), Box::new(self.dual(body))),
            BinarySession::Var(x) => BinarySession::Var(x.clone()),
            BinarySession::End => BinarySession::End,
        }
    }
    /// Check whether `s1` and `s2` are dual (i.e., compatible).
    /// A process holding `s1` can communicate with one holding `s2`.
    pub fn are_dual(&self, s1: &BinarySession, s2: &BinarySession) -> bool {
        &self.dual(s1) == s2
    }
    /// Check that a pair of sessions can reduce to `End` together (session progress).
    /// This is a simple syntactic check: both are End, or both are dual in one step.
    pub fn compatible(&self, s1: &BinarySession, s2: &BinarySession) -> bool {
        match (s1, s2) {
            (BinarySession::End, BinarySession::End) => true,
            (BinarySession::Send(l1, c1), BinarySession::Recv(l2, c2)) => {
                l1 == l2 && self.compatible(c1, c2)
            }
            (BinarySession::Recv(l1, c1), BinarySession::Send(l2, c2)) => {
                l1 == l2 && self.compatible(c1, c2)
            }
            (BinarySession::Select(bs1), BinarySession::Offer(bs2)) => bs1.iter().all(|(l, s)| {
                bs2.iter()
                    .find(|(l2, _)| l == l2)
                    .map(|(_, s2)| self.compatible(s, s2))
                    .unwrap_or(false)
            }),
            (BinarySession::Offer(bs1), BinarySession::Select(bs2)) => bs2.iter().all(|(l, s)| {
                bs1.iter()
                    .find(|(l1, _)| l == l1)
                    .map(|(_, s1)| self.compatible(s1, s))
                    .unwrap_or(false)
            }),
            _ => false,
        }
    }
}
/// An untyped lambda calculus term using de Bruijn indices.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    /// de Bruijn variable (0 = innermost binder).
    Var(usize),
    /// Lambda abstraction: λ.body.
    Lam(Box<Term>),
    /// Application: (f arg).
    App(Box<Term>, Box<Term>),
}
impl Term {
    /// Shift all free variables by `d` when entering a context of depth `cutoff`.
    pub fn shift(&self, d: isize, cutoff: usize) -> Term {
        match self {
            Term::Var(k) => {
                if *k >= cutoff {
                    let new_k = (*k as isize + d) as usize;
                    Term::Var(new_k)
                } else {
                    Term::Var(*k)
                }
            }
            Term::Lam(body) => Term::Lam(Box::new(body.shift(d, cutoff + 1))),
            Term::App(f, a) => {
                Term::App(Box::new(f.shift(d, cutoff)), Box::new(a.shift(d, cutoff)))
            }
        }
    }
    /// Substitute `sub` for de Bruijn index `j` in `self`.
    pub fn subst(&self, j: usize, sub: &Term) -> Term {
        match self {
            Term::Var(k) => {
                if *k == j {
                    sub.shift(j as isize, 0)
                } else if *k > j {
                    Term::Var(k - 1)
                } else {
                    Term::Var(*k)
                }
            }
            Term::Lam(body) => Term::Lam(Box::new(body.subst(j + 1, sub))),
            Term::App(f, a) => Term::App(Box::new(f.subst(j, sub)), Box::new(a.subst(j, sub))),
        }
    }
    /// Perform one step of β-reduction (normal order: leftmost-outermost).
    /// Returns `Some(reduced)` if a redex was found, `None` if already normal.
    pub fn beta_step_normal(&self) -> Option<Term> {
        match self {
            Term::App(f, a) => {
                if let Term::Lam(body) = f.as_ref() {
                    return Some(body.subst(0, a));
                }
                if let Some(f2) = f.beta_step_normal() {
                    return Some(Term::App(Box::new(f2), a.clone()));
                }
                if let Some(a2) = a.beta_step_normal() {
                    return Some(Term::App(f.clone(), Box::new(a2)));
                }
                None
            }
            Term::Lam(body) => body.beta_step_normal().map(|b2| Term::Lam(Box::new(b2))),
            Term::Var(_) => None,
        }
    }
    /// Fully normalize by normal-order β-reduction (with a step limit to avoid divergence).
    pub fn normalize(&self, limit: usize) -> (Term, usize) {
        let mut t = self.clone();
        let mut steps = 0;
        while steps < limit {
            match t.beta_step_normal() {
                Some(t2) => {
                    t = t2;
                    steps += 1;
                }
                None => break,
            }
        }
        (t, steps)
    }
    /// Check if the term is in beta-normal form.
    pub fn is_normal(&self) -> bool {
        self.beta_step_normal().is_none()
    }
    /// Count the number of nodes in the term tree.
    pub fn size(&self) -> usize {
        match self {
            Term::Var(_) => 1,
            Term::Lam(b) => 1 + b.size(),
            Term::App(f, a) => 1 + f.size() + a.size(),
        }
    }
}
/// A bidirectional type inference system for STLC.
/// Implements the "Check" and "Synth" modes of bidirectional typing.
#[allow(dead_code)]
pub struct TypeInferenceSystem;
#[allow(dead_code)]
impl TypeInferenceSystem {
    /// Create a new type inference system.
    pub fn new() -> Self {
        TypeInferenceSystem
    }
    /// Synthesis mode: try to infer the type of `term` in context `ctx`.
    /// Returns `Some(ty)` if successful.
    pub fn synthesize(&self, ctx: &Context, term: &Term) -> Option<SimpleType> {
        match term {
            Term::Var(k) => ctx.get(*k).cloned(),
            Term::App(f, a) => {
                let ft = self.synthesize(ctx, f)?;
                match ft {
                    SimpleType::Arrow(dom, cod) => {
                        if self.check(ctx, a, &dom) {
                            Some(*cod)
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            }
            Term::Lam(_) => None,
        }
    }
    /// Checking mode: verify that `term` has type `ty` in context `ctx`.
    pub fn check(&self, ctx: &Context, term: &Term, ty: &SimpleType) -> bool {
        match (term, ty) {
            (Term::Lam(body), SimpleType::Arrow(dom, cod)) => {
                let extended = ctx.extend(*dom.clone());
                self.check(&extended, body, cod)
            }
            _ => match self.synthesize(ctx, term) {
                Some(inferred) => inferred == *ty,
                None => false,
            },
        }
    }
    /// Attempt to reconstruct the type of an annotated term.
    /// A term is "annotated" if it is an application with a synthesizable function.
    pub fn infer_with_annotation(
        &self,
        ctx: &Context,
        term: &Term,
        hint: Option<&SimpleType>,
    ) -> Option<SimpleType> {
        match hint {
            Some(ty) => {
                if self.check(ctx, term, ty) {
                    Some(ty.clone())
                } else {
                    None
                }
            }
            None => self.synthesize(ctx, term),
        }
    }
}
/// A simple linearity checker that tracks variable usage counts.
/// In linear typing, each variable must be used exactly once.
#[allow(dead_code)]
pub struct LinearTypeChecker;
#[allow(dead_code)]
impl LinearTypeChecker {
    /// Create a new linearity checker.
    pub fn new() -> Self {
        LinearTypeChecker
    }
    /// Count free variable occurrences in `term`, relative to a context of `depth` variables.
    /// Returns a vector of length `depth` with usage counts for each variable.
    pub fn count_uses(&self, term: &Term, depth: usize) -> UsageMap {
        let mut counts = vec![0usize; depth];
        self.count_uses_inner(term, depth, 0, &mut counts);
        counts
    }
    fn count_uses_inner(&self, term: &Term, depth: usize, offset: usize, counts: &mut UsageMap) {
        match term {
            Term::Var(k) => {
                let adjusted = if *k >= offset { k - offset } else { return };
                if adjusted < depth {
                    counts[adjusted] += 1;
                }
            }
            Term::Lam(body) => {
                self.count_uses_inner(body, depth, offset + 1, counts);
            }
            Term::App(f, a) => {
                self.count_uses_inner(f, depth, offset, counts);
                self.count_uses_inner(a, depth, offset, counts);
            }
        }
    }
    /// Check if `term` is linear in the context of `depth` variables.
    /// Returns `true` iff every variable in the context is used exactly once.
    pub fn is_linear(&self, term: &Term, depth: usize) -> bool {
        let uses = self.count_uses(term, depth);
        uses.iter().all(|&c| c == 1)
    }
    /// Check if `term` is affine in the context of `depth` variables.
    /// Returns `true` iff every variable in the context is used at most once.
    pub fn is_affine(&self, term: &Term, depth: usize) -> bool {
        let uses = self.count_uses(term, depth);
        uses.iter().all(|&c| c <= 1)
    }
    /// Check if `term` is relevant in the context of `depth` variables.
    /// Returns `true` iff every variable in the context is used at least once.
    pub fn is_relevant(&self, term: &Term, depth: usize) -> bool {
        let uses = self.count_uses(term, depth);
        uses.iter().all(|&c| c >= 1)
    }
}
/// A session type in the binary session type system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinarySession {
    /// Send a value of type `label` and continue as `next`.
    Send(String, Box<BinarySession>),
    /// Receive a value of type `label` and continue as `next`.
    Recv(String, Box<BinarySession>),
    /// Internal choice: select one of the offered branches.
    Select(Vec<(String, BinarySession)>),
    /// External choice: offer a set of branches.
    Offer(Vec<(String, BinarySession)>),
    /// Recursive session: μX.S where `name` is the recursion variable.
    Rec(String, Box<BinarySession>),
    /// Session variable reference.
    Var(String),
    /// Completed session.
    End,
}
