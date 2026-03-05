//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use std::collections::HashMap;

/// Homotopy level (h-level) of a type in HoTT.
///
/// Following the convention:
/// - h-level -2: contractible (unique up to paths)
/// - h-level -1: mere proposition (all paths between points are equal)
/// - h-level  0: set (all paths are trivial / UIP holds)
/// - h-level  1: groupoid
/// - h-level  2: 2-groupoid
/// - h-level  n: n-groupoid
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HomotopyLevel {
    /// h-level -2: there is exactly one element up to paths.
    Contractible,
    /// h-level -1: all inhabitants are path-equal.
    Proposition,
    /// h-level 0: all paths between any two points are trivially equal.
    Set,
    /// h-level 1: paths form a groupoid.
    Groupoid,
    /// h-level 2.
    TwoGroupoid,
    /// h-level n for n ≥ 3.
    N(u32),
}
impl HomotopyLevel {
    /// Returns the integer homotopy level (h-level - 2 convention).
    pub fn truncation_level(&self) -> i32 {
        match self {
            HomotopyLevel::Contractible => -2,
            HomotopyLevel::Proposition => -1,
            HomotopyLevel::Set => 0,
            HomotopyLevel::Groupoid => 1,
            HomotopyLevel::TwoGroupoid => 2,
            HomotopyLevel::N(n) => *n as i32,
        }
    }
    /// Returns true if the type is a mere proposition (or contractible).
    pub fn is_prop(&self) -> bool {
        *self == HomotopyLevel::Proposition || *self == HomotopyLevel::Contractible
    }
    /// Returns true if the type is a set (h-level ≤ 0).
    pub fn is_set(&self) -> bool {
        self <= &HomotopyLevel::Set
    }
}
/// Universe constraint: an expression that the checker evaluates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UniverseConstraint {
    /// `u ≤ v`
    Le(UniverseExpr, UniverseExpr),
    /// `u = v`
    Eq(UniverseExpr, UniverseExpr),
    /// `u < v`
    Lt(UniverseExpr, UniverseExpr),
}
/// A HoTT path: a proof of propositional equality between two terms.
pub struct HottPath {
    /// The left endpoint.
    pub start: MlttTerm,
    /// The right endpoint.
    pub end: MlttTerm,
    /// The proof term (of identity type).
    pub proof: MlttTerm,
}
impl HottPath {
    /// The reflexivity path `refl a : a = a`.
    pub fn refl(a: MlttTerm) -> Self {
        HottPath {
            start: a.clone(),
            end: a.clone(),
            proof: MlttTerm::Refl(Box::new(a)),
        }
    }
    /// Reverse the path: if `p : a = b`, produce `p⁻¹ : b = a`.
    pub fn sym(&self) -> Self {
        HottPath {
            start: self.end.clone(),
            end: self.start.clone(),
            proof: MlttTerm::J {
                motive: Box::new(MlttTerm::Var("_sym_motive".to_string())),
                base: Box::new(MlttTerm::Refl(Box::new(self.start.clone()))),
                path: Box::new(self.proof.clone()),
            },
        }
    }
    /// Returns true if the proof is a `refl` term.
    pub fn is_refl(&self) -> bool {
        matches!(self.proof, MlttTerm::Refl(_))
    }
}
/// A universe level in a cumulative hierarchy Type_0 : Type_1 : ...
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct UniverseLevel(pub u32);
impl UniverseLevel {
    /// The base universe (Type_0, also called `Set` in some formulations).
    pub fn zero() -> Self {
        UniverseLevel(0)
    }
    /// The successor universe: `succ(n) = n + 1`.
    pub fn succ(self) -> Self {
        UniverseLevel(self.0 + 1)
    }
    /// The maximum of two universe levels.
    pub fn max(a: Self, b: Self) -> Self {
        UniverseLevel(a.0.max(b.0))
    }
    /// Impredicative max: `imax(a, 0) = 0`, `imax(a, b) = max(a, b)` for b > 0.
    ///
    /// Used for the universe of Pi-types.
    pub fn imax(a: Self, b: Self) -> Self {
        if b.0 == 0 {
            UniverseLevel(0)
        } else {
            Self::max(a, b)
        }
    }
}
/// Universe polymorphism checker.
///
/// Collects universe constraints from a definition and checks whether there
/// exists a valid assignment of concrete levels to universe variables.
pub struct UniverseChecker {
    /// Registered universe variable names.
    vars: Vec<String>,
    /// Accumulated constraints.
    constraints: Vec<UniverseConstraint>,
}
impl UniverseChecker {
    /// Create a new checker with no variables or constraints.
    pub fn new() -> Self {
        UniverseChecker {
            vars: vec![],
            constraints: vec![],
        }
    }
    /// Declare a universe variable.
    pub fn declare(&mut self, name: String) {
        if !self.vars.contains(&name) {
            self.vars.push(name);
        }
    }
    /// Add a universe constraint.
    pub fn add_constraint(&mut self, c: UniverseConstraint) {
        self.constraints.push(c);
    }
    /// Check consistency of the accumulated constraints by brute-force search
    /// up to the given maximum level for each universe variable.
    pub fn is_consistent(&self, max_level: u32) -> bool {
        self.search(&mut std::collections::HashMap::new(), 0, max_level)
    }
    fn search(
        &self,
        assign: &mut std::collections::HashMap<String, u32>,
        idx: usize,
        max_level: u32,
    ) -> bool {
        if idx == self.vars.len() {
            return self
                .constraints
                .iter()
                .all(|c| self.check_constraint(c, assign));
        }
        for v in 0..=max_level {
            assign.insert(self.vars[idx].clone(), v);
            if self.search(assign, idx + 1, max_level) {
                return true;
            }
        }
        assign.remove(&self.vars[idx]);
        false
    }
    fn check_constraint(
        &self,
        c: &UniverseConstraint,
        assign: &std::collections::HashMap<String, u32>,
    ) -> bool {
        match c {
            UniverseConstraint::Le(a, b) => {
                matches!(
                    (a.eval(assign), b.eval(assign)), (Some(av), Some(bv)) if av <= bv
                )
            }
            UniverseConstraint::Eq(a, b) => {
                matches!(
                    (a.eval(assign), b.eval(assign)), (Some(av), Some(bv)) if av == bv
                )
            }
            UniverseConstraint::Lt(a, b) => {
                matches!(
                    (a.eval(assign), b.eval(assign)), (Some(av), Some(bv)) if av < bv
                )
            }
        }
    }
    /// Return the number of registered variables.
    pub fn num_vars(&self) -> usize {
        self.vars.len()
    }
    /// Return the number of constraints.
    pub fn num_constraints(&self) -> usize {
        self.constraints.len()
    }
}
/// A neutral value: a variable applied to zero or more semantic values.
#[derive(Debug, Clone)]
pub struct NbeNeutral {
    /// The head variable.
    pub head: String,
    /// The spine (list of argument values), outermost first.
    pub spine: Vec<NbeValue>,
}
/// A term in a simplified Martin-Löf Type Theory.
///
/// This is a Rust-level representation for meta-level manipulation
/// (not the kernel `Expr` type).
#[derive(Debug, Clone)]
pub enum MlttTerm {
    /// A free variable by name.
    Var(String),
    /// A universe: `Type_n`.
    Type(UniverseLevel),
    /// Dependent product: `Π (binder : domain), codomain`.
    Pi {
        binder: String,
        domain: Box<MlttTerm>,
        codomain: Box<MlttTerm>,
    },
    /// Lambda abstraction: `λ binder, body`.
    Lam { binder: String, body: Box<MlttTerm> },
    /// Application: `f a`.
    App(Box<MlttTerm>, Box<MlttTerm>),
    /// Dependent sum: `Σ (binder : fst), snd`.
    Sigma {
        binder: String,
        fst: Box<MlttTerm>,
        snd: Box<MlttTerm>,
    },
    /// A pair `(fst, snd)`.
    Pair(Box<MlttTerm>, Box<MlttTerm>),
    /// First projection.
    Fst(Box<MlttTerm>),
    /// Second projection.
    Snd(Box<MlttTerm>),
    /// Identity type: `lhs = rhs : ty`.
    Id {
        ty: Box<MlttTerm>,
        lhs: Box<MlttTerm>,
        rhs: Box<MlttTerm>,
    },
    /// Reflexivity proof: `refl a`.
    Refl(Box<MlttTerm>),
    /// J-eliminator application.
    J {
        motive: Box<MlttTerm>,
        base: Box<MlttTerm>,
        path: Box<MlttTerm>,
    },
    /// The natural number type.
    Nat,
    /// Zero.
    Zero,
    /// Successor.
    Succ(Box<MlttTerm>),
    /// Natural number recursor.
    NatRec {
        motive: Box<MlttTerm>,
        base: Box<MlttTerm>,
        step: Box<MlttTerm>,
        n: Box<MlttTerm>,
    },
    /// The unit type `⊤`.
    Unit,
    /// The unique inhabitant of `Unit`.
    Star,
    /// The empty type `⊥`.
    Empty,
    /// Ex falso: eliminate the empty type.
    Abort(Box<MlttTerm>),
}
impl MlttTerm {
    /// Build an application node.
    pub fn app(f: MlttTerm, a: MlttTerm) -> Self {
        MlttTerm::App(Box::new(f), Box::new(a))
    }
    /// Build a lambda abstraction.
    pub fn lam(binder: &str, body: MlttTerm) -> Self {
        MlttTerm::Lam {
            binder: binder.to_string(),
            body: Box::new(body),
        }
    }
    /// Returns true if the term is a universe.
    pub fn is_type(&self) -> bool {
        matches!(self, MlttTerm::Type(_))
    }
    /// Returns true if the term is in neutral (head-normal) form.
    ///
    /// Neutral terms: variables, applications, and projections.
    pub fn is_neutral(&self) -> bool {
        matches!(
            self,
            MlttTerm::Var(_) | MlttTerm::App(..) | MlttTerm::Fst(_) | MlttTerm::Snd(_)
        )
    }
    /// Perform one step of beta-reduction, if possible.
    ///
    /// Only reduces outermost redexes:
    /// - `(λ x. body) arg` → substitute `arg` for `x` in `body`
    /// - `fst (a, b)` → `a`
    /// - `snd (a, b)` → `b`
    pub fn beta_reduce(&self) -> Option<MlttTerm> {
        match self {
            MlttTerm::App(f, arg) => match f.as_ref() {
                MlttTerm::Lam { binder, body } => Some(subst_var(body, binder, arg)),
                _ => None,
            },
            MlttTerm::Fst(pair) => match pair.as_ref() {
                MlttTerm::Pair(a, _) => Some(*a.clone()),
                _ => None,
            },
            MlttTerm::Snd(pair) => match pair.as_ref() {
                MlttTerm::Pair(_, b) => Some(*b.clone()),
                _ => None,
            },
            _ => None,
        }
    }
}
/// Bidirectional typechecker entry point.
///
/// Splits type-checking into two modes:
/// - **Checking** (`check`): verify that a term has a given type.
/// - **Synthesis** (`synth`): infer the type of a term.
pub struct BidirectionalTypechecker {
    ctx: std::collections::HashMap<String, BidirType>,
}
impl BidirectionalTypechecker {
    /// Create a new empty bidirectional typechecker.
    pub fn new() -> Self {
        BidirectionalTypechecker {
            ctx: std::collections::HashMap::new(),
        }
    }
    /// Bind a variable to a type in the context.
    pub fn bind(&mut self, var: String, ty: BidirType) {
        self.ctx.insert(var, ty);
    }
    /// **Synthesis mode**: infer the type of `ty_expr`.
    ///
    /// Succeeds for annotated expressions and variables.
    pub fn synth(&self, ty_expr: &BidirType) -> Result<BidirType, String> {
        match ty_expr {
            BidirType::Ann(_, ann_ty) => Ok(*ann_ty.clone()),
            BidirType::Base(name) => self
                .ctx
                .get(name)
                .cloned()
                .ok_or_else(|| format!("Unknown type/variable: {name}")),
            BidirType::Type(n) => Ok(BidirType::Type(n + 1)),
            BidirType::Fun(a, b) => {
                let _ak = self.synth(a)?;
                let _bk = self.synth(b)?;
                Ok(BidirType::Type(0))
            }
            BidirType::Pi(x, dom, cod) => {
                let dk = self.synth(dom)?;
                let mut inner = BidirectionalTypechecker {
                    ctx: self.ctx.clone(),
                };
                inner.bind(x.clone(), *dom.clone());
                let ck = inner.synth(cod)?;
                let n = match (dk, ck) {
                    (BidirType::Type(a), BidirType::Type(b)) => a.max(b),
                    _ => 0,
                };
                Ok(BidirType::Type(n))
            }
        }
    }
    /// **Checking mode**: verify that `expr_ty` is a subtype of `expected`.
    pub fn check(&self, expr_ty: &BidirType, expected: &BidirType) -> Result<(), String> {
        if expr_ty == expected {
            return Ok(());
        }
        if let (BidirType::Type(n), BidirType::Type(m)) = (expr_ty, expected) {
            if n <= m {
                return Ok(());
            }
        }
        Err(format!(
            "Type mismatch in check mode: expected {expected}, got {expr_ty}"
        ))
    }
}
/// Normalization by evaluation engine for STLC.
pub struct NormalizationByEvaluation;
impl NormalizationByEvaluation {
    /// Evaluate an STLC term in an environment to a semantic value.
    pub fn eval(term: &STLCTerm, env: &[(String, NbeValue)]) -> NbeValue {
        match term {
            STLCTerm::Var(x) => env
                .iter()
                .rev()
                .find(|(n, _)| n == x)
                .map(|(_, v)| v.clone())
                .unwrap_or_else(|| {
                    NbeValue::Neutral(NbeNeutral {
                        head: x.clone(),
                        spine: vec![],
                    })
                }),
            STLCTerm::Lam(x, _ty, body) => NbeValue::Clos {
                param: x.clone(),
                body: *body.clone(),
                env: env.to_vec(),
            },
            STLCTerm::App(f, a) => {
                let fv = Self::eval(f, env);
                let av = Self::eval(a, env);
                Self::apply(fv, av)
            }
            STLCTerm::Pair(a, b) => {
                NbeValue::Pair(Box::new(Self::eval(a, env)), Box::new(Self::eval(b, env)))
            }
            STLCTerm::Fst(p) => match Self::eval(p, env) {
                NbeValue::Pair(a, _) => *a,
                other => NbeValue::Neutral(NbeNeutral {
                    head: "fst".to_string(),
                    spine: vec![other],
                }),
            },
            STLCTerm::Snd(p) => match Self::eval(p, env) {
                NbeValue::Pair(_, b) => *b,
                other => NbeValue::Neutral(NbeNeutral {
                    head: "snd".to_string(),
                    spine: vec![other],
                }),
            },
            STLCTerm::Star => NbeValue::Unit,
        }
    }
    /// Apply a closure (semantic function) to an argument value.
    pub fn apply(f: NbeValue, arg: NbeValue) -> NbeValue {
        match f {
            NbeValue::Clos {
                param,
                body,
                mut env,
            } => {
                env.push((param, arg));
                Self::eval(&body, &env)
            }
            NbeValue::Neutral(mut n) => {
                n.spine.push(arg);
                NbeValue::Neutral(n)
            }
            _ => NbeValue::Neutral(NbeNeutral {
                head: "<stuck>".to_string(),
                spine: vec![f, arg],
            }),
        }
    }
    /// Reify a semantic value back to a (β-normal) STLC term.
    pub fn reify(val: &NbeValue, fresh: &mut u32) -> STLCTerm {
        match val {
            NbeValue::Unit => STLCTerm::Star,
            NbeValue::Pair(a, b) => STLCTerm::Pair(
                Box::new(Self::reify(a, fresh)),
                Box::new(Self::reify(b, fresh)),
            ),
            NbeValue::Neutral(n) => {
                let mut result = STLCTerm::Var(n.head.clone());
                for arg in &n.spine {
                    result = STLCTerm::App(Box::new(result), Box::new(Self::reify(arg, fresh)));
                }
                result
            }
            NbeValue::Clos {
                param: _,
                body: _,
                env: _,
            } => {
                let x = format!("x{}", *fresh);
                *fresh += 1;
                let arg = NbeValue::Neutral(NbeNeutral {
                    head: x.clone(),
                    spine: vec![],
                });
                let result = Self::apply(val.clone(), arg);
                STLCTerm::Lam(
                    x,
                    STLCType::Base("_".to_string()),
                    Box::new(Self::reify(&result, fresh)),
                )
            }
        }
    }
    /// Normalise a closed STLC term to its beta-normal form.
    pub fn normalise(term: &STLCTerm) -> STLCTerm {
        let val = Self::eval(term, &[]);
        let mut counter = 0u32;
        Self::reify(&val, &mut counter)
    }
}
/// A type in a bidirectional type theory.
///
/// Supports checking and synthesis modes via explicit annotations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BidirType {
    /// Universe `Type_n`.
    Type(u32),
    /// Dependent product `Π (x : A). B`.
    Pi(String, Box<BidirType>, Box<BidirType>),
    /// Simple function type `A → B`.
    Fun(Box<BidirType>, Box<BidirType>),
    /// A named base type.
    Base(String),
    /// Type annotation: a term annotated with its type.
    Ann(Box<BidirType>, Box<BidirType>),
}
impl BidirType {
    /// Lift a `BidirType` to the next universe.
    pub fn in_type(&self) -> BidirType {
        BidirType::Type(match self {
            BidirType::Type(n) => n + 1,
            _ => 0,
        })
    }
}
/// Typechecker for the simply typed lambda calculus.
///
/// Implements bidirectional-style checking in a pure Hindley-style context
/// (since all binders are annotated, no inference is needed).
pub struct STLCTypechecker {
    /// Typing context: variable name → type.
    ctx: std::collections::HashMap<String, STLCType>,
}
impl STLCTypechecker {
    /// Create a new empty typechecker.
    pub fn new() -> Self {
        STLCTypechecker {
            ctx: std::collections::HashMap::new(),
        }
    }
    /// Extend the context with a new binding.
    pub fn extend(&mut self, var: String, ty: STLCType) {
        self.ctx.insert(var, ty);
    }
    /// Remove a binding from the context.
    pub fn remove(&mut self, var: &str) {
        self.ctx.remove(var);
    }
    /// Synthesise the type of a term, returning `Err` if ill-typed.
    pub fn synth(&self, term: &STLCTerm) -> Result<STLCType, String> {
        match term {
            STLCTerm::Var(x) => self
                .ctx
                .get(x)
                .cloned()
                .ok_or_else(|| format!("Unbound variable: {x}")),
            STLCTerm::Lam(x, dom, body) => {
                let mut inner = STLCTypechecker {
                    ctx: self.ctx.clone(),
                };
                inner.extend(x.clone(), dom.clone());
                let cod = inner.synth(body)?;
                Ok(STLCType::fun(dom.clone(), cod))
            }
            STLCTerm::App(f, a) => {
                let fty = self.synth(f)?;
                match fty {
                    STLCType::Fun(dom, cod) => {
                        self.check(a, &dom)?;
                        Ok(*cod)
                    }
                    other => Err(format!("Expected function type, got {other}")),
                }
            }
            STLCTerm::Pair(a, b) => {
                let ta = self.synth(a)?;
                let tb = self.synth(b)?;
                Ok(STLCType::prod(ta, tb))
            }
            STLCTerm::Fst(p) => {
                let pty = self.synth(p)?;
                match pty {
                    STLCType::Prod(a, _) => Ok(*a),
                    other => Err(format!("Expected product type for fst, got {other}")),
                }
            }
            STLCTerm::Snd(p) => {
                let pty = self.synth(p)?;
                match pty {
                    STLCType::Prod(_, b) => Ok(*b),
                    other => Err(format!("Expected product type for snd, got {other}")),
                }
            }
            STLCTerm::Star => Ok(STLCType::Unit),
        }
    }
    /// Check that a term has a given type, returning `Err` if it does not.
    pub fn check(&self, term: &STLCTerm, expected: &STLCType) -> Result<(), String> {
        let actual = self.synth(term)?;
        if &actual == expected {
            Ok(())
        } else {
            Err(format!("Type mismatch: expected {expected}, got {actual}"))
        }
    }
}
/// A simplified System F type inference engine (rank-1 polymorphism).
///
/// Handles type abstraction, type application, and generalisation over
/// free type variables.
pub struct SystemFTypeInference {
    /// Typing context: variable → System F type.
    ctx: std::collections::HashMap<String, SystemFType>,
    /// Counter for fresh type variable generation.
    fresh_counter: u32,
}
impl SystemFTypeInference {
    /// Create a new empty inference engine.
    pub fn new() -> Self {
        SystemFTypeInference {
            ctx: std::collections::HashMap::new(),
            fresh_counter: 0,
        }
    }
    /// Generate a fresh type variable name.
    pub fn fresh_tyvar(&mut self) -> String {
        let n = self.fresh_counter;
        self.fresh_counter += 1;
        format!("α{n}")
    }
    /// Extend context with a typed variable.
    pub fn bind(&mut self, var: String, ty: SystemFType) {
        self.ctx.insert(var, ty);
    }
    /// Look up a variable in the context.
    pub fn lookup(&self, var: &str) -> Option<&SystemFType> {
        self.ctx.get(var)
    }
    /// Instantiate a `∀` type with the given type argument.
    pub fn inst(&self, ty: &SystemFType, arg: &SystemFType) -> Result<SystemFType, String> {
        match ty {
            SystemFType::Forall(v, body) => Ok(body.instantiate(v, arg)),
            other => Err(format!("Cannot instantiate non-forall type {other}")),
        }
    }
    /// Generalise a type over free type variables (convert `T` → `∀αs. T`).
    pub fn generalise(&self, ty: SystemFType, free_vars: &[String]) -> SystemFType {
        free_vars
            .iter()
            .rev()
            .fold(ty, |acc, v| SystemFType::Forall(v.clone(), Box::new(acc)))
    }
}
/// A universe level expression (with variables).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UniverseExpr {
    /// A concrete level `n`.
    Lit(u32),
    /// A universe variable `u`.
    Var(String),
    /// `succ(u)`.
    Succ(Box<UniverseExpr>),
    /// `max(u, v)`.
    Max(Box<UniverseExpr>, Box<UniverseExpr>),
    /// `imax(u, v)`: impredicative max.
    IMax(Box<UniverseExpr>, Box<UniverseExpr>),
}
impl UniverseExpr {
    /// Evaluate to a concrete level under the given assignment.
    pub fn eval(&self, assign: &std::collections::HashMap<String, u32>) -> Option<u32> {
        match self {
            UniverseExpr::Lit(n) => Some(*n),
            UniverseExpr::Var(v) => assign.get(v).copied(),
            UniverseExpr::Succ(u) => u.eval(assign).map(|n| n + 1),
            UniverseExpr::Max(a, b) => {
                let av = a.eval(assign)?;
                let bv = b.eval(assign)?;
                Some(av.max(bv))
            }
            UniverseExpr::IMax(a, b) => {
                let av = a.eval(assign)?;
                let bv = b.eval(assign)?;
                Some(if bv == 0 { 0 } else { av.max(bv) })
            }
        }
    }
}
/// A semantic value in the NbE model for STLC.
///
/// NbE evaluates terms into a semantic domain and then "reifies" values back
/// into normal forms.  This gives a decision procedure for βη-equality.
#[derive(Debug, Clone)]
pub enum NbeValue {
    /// A lambda closure: `Clos(param_name, body_term, environment)`.
    Clos {
        param: String,
        body: STLCTerm,
        env: Vec<(String, NbeValue)>,
    },
    /// A pair.
    Pair(Box<NbeValue>, Box<NbeValue>),
    /// Unit.
    Unit,
    /// A neutral value that cannot be reduced further (variable-headed).
    Neutral(NbeNeutral),
}
/// A System F type (with universal quantification at the top level).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SystemFType {
    /// A type variable.
    TyVar(String),
    /// Universally quantified type `∀ α. T`.
    Forall(String, Box<SystemFType>),
    /// Function type `S → T`.
    Fun(Box<SystemFType>, Box<SystemFType>),
    /// Base type.
    Base(String),
}
impl SystemFType {
    /// Instantiate a universally quantified type at a concrete type.
    pub fn instantiate(&self, ty_var: &str, with: &SystemFType) -> SystemFType {
        match self {
            SystemFType::TyVar(v) => {
                if v == ty_var {
                    with.clone()
                } else {
                    self.clone()
                }
            }
            SystemFType::Forall(v, body) => {
                if v == ty_var {
                    self.clone()
                } else {
                    SystemFType::Forall(v.clone(), Box::new(body.instantiate(ty_var, with)))
                }
            }
            SystemFType::Fun(a, b) => SystemFType::Fun(
                Box::new(a.instantiate(ty_var, with)),
                Box::new(b.instantiate(ty_var, with)),
            ),
            SystemFType::Base(_) => self.clone(),
        }
    }
    /// Returns true if `α` appears free in this type.
    pub fn free_in(&self, ty_var: &str) -> bool {
        match self {
            SystemFType::TyVar(v) => v == ty_var,
            SystemFType::Forall(v, body) => v != ty_var && body.free_in(ty_var),
            SystemFType::Fun(a, b) => a.free_in(ty_var) || b.free_in(ty_var),
            SystemFType::Base(_) => false,
        }
    }
}
/// A term in STLC with named variables and type annotations on binders.
#[derive(Debug, Clone)]
pub enum STLCTerm {
    /// Variable.
    Var(String),
    /// Lambda abstraction `λ x : T. body`.
    Lam(String, STLCType, Box<STLCTerm>),
    /// Application `f a`.
    App(Box<STLCTerm>, Box<STLCTerm>),
    /// Pair `(a, b)`.
    Pair(Box<STLCTerm>, Box<STLCTerm>),
    /// First projection.
    Fst(Box<STLCTerm>),
    /// Second projection.
    Snd(Box<STLCTerm>),
    /// Unit value `⋆`.
    Star,
}
/// A type in the simply typed lambda calculus (STLC).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum STLCType {
    /// A base type (named).
    Base(String),
    /// Function type: `S → T`.
    Fun(Box<STLCType>, Box<STLCType>),
    /// Product type: `S × T`.
    Prod(Box<STLCType>, Box<STLCType>),
    /// Unit type `⊤`.
    Unit,
}
impl STLCType {
    /// Build a function type `s → t`.
    pub fn fun(s: STLCType, t: STLCType) -> Self {
        STLCType::Fun(Box::new(s), Box::new(t))
    }
    /// Build a product type `s × t`.
    pub fn prod(s: STLCType, t: STLCType) -> Self {
        STLCType::Prod(Box::new(s), Box::new(t))
    }
    /// Returns true if this is a base type.
    pub fn is_base(&self) -> bool {
        matches!(self, STLCType::Base(_))
    }
}
