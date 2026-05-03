//! Types for dependent type theory formalizations.
//!
//! Covers Pure Type Systems (PTS), the λ-cube, Church and Scott encodings,
//! and various related proof-theoretic constructs.

/// Sort in a Pure Type System — the universe hierarchy.
///
/// - `Prop` is the sort of propositions (impredicative in CoC).
/// - `Type` is the sort of small types (predicative).
/// - `Kind` is the sort of `Type` itself (meta-level sort).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DttSort {
    /// The sort of propositions (Prop, or *).
    Prop,
    /// The sort of small types (Type, or □ in some literature).
    Type,
    /// The sort of type constructors / universes of types (Kind, or △).
    Kind,
}

impl DttSort {
    /// Returns true if this sort is `Prop`.
    pub fn is_prop(&self) -> bool {
        matches!(self, DttSort::Prop)
    }

    /// Returns true if this sort is `Type`.
    pub fn is_type(&self) -> bool {
        matches!(self, DttSort::Type)
    }

    /// Returns true if this sort is `Kind`.
    pub fn is_kind(&self) -> bool {
        matches!(self, DttSort::Kind)
    }
}

impl std::fmt::Display for DttSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DttSort::Prop => write!(f, "Prop"),
            DttSort::Type => write!(f, "Type"),
            DttSort::Kind => write!(f, "Kind"),
        }
    }
}

/// A term in a dependent type theory with de Bruijn indices.
///
/// This representation supports Pi (Π) types, Lambda (λ) abstractions,
/// applications, Sigma (Σ) types, pairs, projections, and natural number
/// eliminators, forming the core of the Calculus of Inductive Constructions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DttTerm {
    /// A de Bruijn variable with index `n` (0 = innermost binder).
    Var(usize),
    /// A sort (Prop, Type, or Kind).
    Sort(DttSort),
    /// Π-type: `Π (x : A) . B`. `body` may reference `x` as `Var(0)`.
    Pi {
        /// Type of the bound variable.
        domain: Box<DttTerm>,
        /// Body type (may depend on the variable as `Var(0)`).
        body: Box<DttTerm>,
    },
    /// Lambda abstraction: `λ (x : ty) . body`.
    Lambda {
        /// Type annotation of the bound variable.
        ty: Box<DttTerm>,
        /// Body (may reference the bound var as `Var(0)`).
        body: Box<DttTerm>,
    },
    /// Function application: `f a`.
    App {
        /// The function term.
        func: Box<DttTerm>,
        /// The argument term.
        arg: Box<DttTerm>,
    },
    /// Sigma type: `Σ (x : A) . B`.
    Sigma {
        /// First component type.
        first: Box<DttTerm>,
        /// Second component type (may depend on first component as `Var(0)`).
        second: Box<DttTerm>,
    },
    /// Dependent pair: `(fst, snd)` with a type annotation.
    Pair {
        /// First component.
        fst: Box<DttTerm>,
        /// Second component.
        snd: Box<DttTerm>,
        /// The Sigma type of this pair.
        ty: Box<DttTerm>,
    },
    /// First projection: `π₁ t`.
    Fst(Box<DttTerm>),
    /// Second projection: `π₂ t`.
    Snd(Box<DttTerm>),
    /// The natural number type `ℕ`.
    Nat,
    /// Zero: `0 : ℕ`.
    Zero,
    /// Successor: `S n`.
    Succ(Box<DttTerm>),
    /// Natural number eliminator (induction/recursion principle):
    /// `Nat.elim motive base step n`
    /// - `motive : ℕ → Sort s`
    /// - `base : motive 0`
    /// - `step : Π (n : ℕ) . motive n → motive (S n)`
    /// - `target : ℕ`
    Elim {
        /// The motive (type family over ℕ).
        motive: Box<DttTerm>,
        /// The base case.
        base: Box<DttTerm>,
        /// The step case.
        step: Box<DttTerm>,
        /// The natural number being eliminated.
        target: Box<DttTerm>,
    },
    /// A global constant (free variable) by name.
    Const(String),
}

impl DttTerm {
    /// Convenience: create a `Var`.
    pub fn var(n: usize) -> Self {
        DttTerm::Var(n)
    }

    /// Convenience: create a `Sort`.
    pub fn sort(s: DttSort) -> Self {
        DttTerm::Sort(s)
    }

    /// Convenience: create a `Pi` type.
    pub fn pi(domain: DttTerm, body: DttTerm) -> Self {
        DttTerm::Pi {
            domain: Box::new(domain),
            body: Box::new(body),
        }
    }

    /// Convenience: create a `Lambda`.
    pub fn lam(ty: DttTerm, body: DttTerm) -> Self {
        DttTerm::Lambda {
            ty: Box::new(ty),
            body: Box::new(body),
        }
    }

    /// Convenience: create an `App`.
    pub fn app(func: DttTerm, arg: DttTerm) -> Self {
        DttTerm::App {
            func: Box::new(func),
            arg: Box::new(arg),
        }
    }

    /// Convenience: create a `Sigma` type.
    pub fn sigma(first: DttTerm, second: DttTerm) -> Self {
        DttTerm::Sigma {
            first: Box::new(first),
            second: Box::new(second),
        }
    }

    /// Convenience: create a `Succ`.
    pub fn succ(n: DttTerm) -> Self {
        DttTerm::Succ(Box::new(n))
    }

    /// Convenience: create `Nat.elim`.
    pub fn elim(motive: DttTerm, base: DttTerm, step: DttTerm, target: DttTerm) -> Self {
        DttTerm::Elim {
            motive: Box::new(motive),
            base: Box::new(base),
            step: Box::new(step),
            target: Box::new(target),
        }
    }

    /// Returns true if this term is a beta-normal form (no reducible applications
    /// with a lambda head at the top level, and no eliminators on constructors).
    pub fn is_normal(&self) -> bool {
        match self {
            DttTerm::App { func, .. } => !matches!(func.as_ref(), DttTerm::Lambda { .. }),
            DttTerm::Elim { target, .. } => {
                !matches!(target.as_ref(), DttTerm::Zero | DttTerm::Succ(_))
            }
            DttTerm::Fst(t) => !matches!(t.as_ref(), DttTerm::Pair { .. }),
            DttTerm::Snd(t) => !matches!(t.as_ref(), DttTerm::Pair { .. }),
            _ => true,
        }
    }

    /// Count the number of subterms (including self).
    pub fn size(&self) -> usize {
        match self {
            DttTerm::Var(_)
            | DttTerm::Sort(_)
            | DttTerm::Nat
            | DttTerm::Zero
            | DttTerm::Const(_) => 1,
            DttTerm::Succ(n) | DttTerm::Fst(n) | DttTerm::Snd(n) => 1 + n.size(),
            DttTerm::Pi { domain, body }
            | DttTerm::Lambda { ty: domain, body }
            | DttTerm::Sigma {
                first: domain,
                second: body,
            } => 1 + domain.size() + body.size(),
            DttTerm::App { func, arg } => 1 + func.size() + arg.size(),
            DttTerm::Pair { fst, snd, ty } => 1 + fst.size() + snd.size() + ty.size(),
            DttTerm::Elim {
                motive,
                base,
                step,
                target,
            } => 1 + motive.size() + base.size() + step.size() + target.size(),
        }
    }
}

impl std::fmt::Display for DttTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DttTerm::Var(n) => write!(f, "#{n}"),
            DttTerm::Sort(s) => write!(f, "{s}"),
            DttTerm::Nat => write!(f, "ℕ"),
            DttTerm::Zero => write!(f, "0"),
            DttTerm::Succ(n) => write!(f, "S({n})"),
            DttTerm::Const(name) => write!(f, "{name}"),
            DttTerm::Pi { domain, body } => write!(f, "Π({domain}).{body}"),
            DttTerm::Lambda { ty, body } => write!(f, "λ({ty}).{body}"),
            DttTerm::App { func, arg } => write!(f, "({func} {arg})"),
            DttTerm::Sigma { first, second } => write!(f, "Σ({first}).{second}"),
            DttTerm::Pair { fst, snd, .. } => write!(f, "⟨{fst},{snd}⟩"),
            DttTerm::Fst(t) => write!(f, "π₁({t})"),
            DttTerm::Snd(t) => write!(f, "π₂({t})"),
            DttTerm::Elim {
                motive,
                base,
                step,
                target,
            } => {
                write!(f, "elim({motive},{base},{step},{target})")
            }
        }
    }
}

/// A typing context: an ordered list of (variable name hint, type) bindings.
///
/// The context is a telescope: entry `i` may mention `Var(j)` for `j < i`.
/// Index 0 in `Var` refers to the innermost (most recently added) binding.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DttContext {
    /// Entries in order from outermost to innermost.
    /// `entries\[k\].1` is the type of the variable that, after all entries are
    /// introduced, will have de Bruijn index `len - 1 - k`.
    pub entries: Vec<(String, DttTerm)>,
}

impl DttContext {
    /// Create an empty context (the empty telescope).
    pub fn empty() -> Self {
        DttContext {
            entries: Vec::new(),
        }
    }

    /// Extend the context with a new binding `(name : ty)`.
    ///
    /// The new variable will be `Var(0)` inside the extended context.
    pub fn extend(&self, name: impl Into<String>, ty: DttTerm) -> Self {
        let mut entries = self.entries.clone();
        entries.push((name.into(), ty));
        DttContext { entries }
    }

    /// Return the type of de Bruijn variable `n` in the current context,
    /// with appropriate shifting to account for the de Bruijn level.
    ///
    /// `Var(0)` refers to the last (innermost) entry.
    pub fn lookup(&self, n: usize) -> Option<&DttTerm> {
        let len = self.entries.len();
        if n < len {
            Some(&self.entries[len - 1 - n].1)
        } else {
            None
        }
    }

    /// Return the number of bindings in this context.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Return true if the context has no bindings.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// A judgment in dependent type theory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DttJudgment {
    /// Typing judgment: `Γ ⊢ t : T`.
    Typing {
        /// Context.
        ctx: DttContext,
        /// Term being typed.
        term: DttTerm,
        /// Its type.
        ty: DttTerm,
    },
    /// Definitional equality: `Γ ⊢ t₁ ≡ t₂ : T`.
    DefEq {
        /// Context.
        ctx: DttContext,
        /// First term.
        lhs: DttTerm,
        /// Second term.
        rhs: DttTerm,
        /// Common type.
        ty: DttTerm,
    },
    /// Type conversion: `Γ ⊢ t : A` and `A ≡ B`, so `Γ ⊢ t : B`.
    Conversion {
        /// Context.
        ctx: DttContext,
        /// Term.
        term: DttTerm,
        /// Original type.
        from_ty: DttTerm,
        /// Target type (definitionally equal to `from_ty`).
        to_ty: DttTerm,
    },
}

/// A Pure Type System (PTS) specification.
///
/// A PTS is determined by three sets:
/// - `sorts`: the universes (e.g. {Prop, Type, Kind}).
/// - `axioms`: maps sort `s` to `s'` meaning `s : s'`.
/// - `rules`: maps `(s₁, s₂)` to `s₃` meaning a Π-type with domain in `s₁`
///   and codomain in `s₂` lives in `s₃`.
///
/// This generalizes CoC, System F, System Fω, λP, etc.
#[derive(Debug, Clone)]
pub struct PureTypeSystem {
    /// Axioms: `axioms\[i\] = (s, s')` means sort `s` has type sort `s'`.
    pub axioms: Vec<(DttSort, DttSort)>,
    /// Rules: `rules\[i\] = (s1, s2, s3)` means
    /// if `A : s1` and `B\[x\] : s2` then `Π(x:A).B : s3`.
    pub rules: Vec<(DttSort, DttSort, DttSort)>,
}

impl PureTypeSystem {
    /// Look up the sort of a given sort (axiom lookup).
    pub fn axiom_sort(&self, s: &DttSort) -> Option<&DttSort> {
        self.axioms
            .iter()
            .find(|(from, _)| from == s)
            .map(|(_, to)| to)
    }

    /// Look up the rule for (s1, s2) and return s3.
    pub fn rule_sort(&self, s1: &DttSort, s2: &DttSort) -> Option<&DttSort> {
        self.rules
            .iter()
            .find(|(a, b, _)| a == s1 && b == s2)
            .map(|(_, _, c)| c)
    }
}
