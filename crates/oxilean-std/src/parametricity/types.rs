//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

/// An element in the realizability model of PCF.
#[derive(Clone, Debug)]
pub enum PcfValue {
    /// A natural number
    Num(u64),
    /// A boolean
    BoolVal(bool),
    /// Undefined (bottom, ⊥)
    Bottom,
    /// A function (as a closure description)
    Fun(String),
}
/// A static type in the gradual type system.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GradualType {
    /// The unknown type `?` (dynamic)
    Unknown,
    /// A base type (Int, Bool, ...)
    Base(String),
    /// An arrow type A → B
    Arrow(Box<GradualType>, Box<GradualType>),
    /// A product type A × B
    Prod(Box<GradualType>, Box<GradualType>),
}
/// A cubical path from `a` to `b` in type `A`, parameterized by dimension `n`.
///
/// Models the CCHM interval type: `I^n → A`.
pub struct CubicalPath<A: Clone> {
    /// Dimension of the path
    pub dim: usize,
    /// The underlying function from face vectors to values
    path_fn: Box<dyn Fn(Vec<bool>) -> A>,
}
impl<A: Clone + 'static> CubicalPath<A> {
    /// Construct a constant path (degenerate, reflecting a point).
    pub fn constant(value: A) -> Self {
        let v = value.clone();
        Self {
            dim: 0,
            path_fn: Box::new(move |_face| v.clone()),
        }
    }
    /// Construct a 1-dimensional path from a function `[0,1] → A`.
    pub fn from_fn(f: impl Fn(bool) -> A + 'static) -> Self {
        Self {
            dim: 1,
            path_fn: Box::new(move |face| {
                let i = face.first().copied().unwrap_or(false);
                f(i)
            }),
        }
    }
    /// Evaluate the path at a face (a vector of booleans, one per dimension).
    pub fn at(&self, face: Vec<bool>) -> A {
        (self.path_fn)(face)
    }
    /// The left endpoint of the path (i = 0).
    pub fn left(&self) -> A {
        self.at(vec![false])
    }
    /// The right endpoint of the path (i = 1).
    pub fn right(&self) -> A {
        self.at(vec![true])
    }
    /// Face map ∂₀ : sets the first dimension variable to 0.
    pub fn face0(&self) -> A {
        let mut face = vec![false; self.dim.max(1)];
        face[0] = false;
        self.at(face)
    }
    /// Face map ∂₁ : sets the first dimension variable to 1.
    pub fn face1(&self) -> A {
        let mut face = vec![false; self.dim.max(1)];
        face[0] = true;
        self.at(face)
    }
    /// Connection ∧: the path `i ↦ p(i ∧ j)` for the constant j.
    pub fn meet(&self, j: bool) -> A {
        self.at(vec![j])
    }
    /// Connection ∨: the path `i ↦ p(i ∨ j)` for the constant j.
    pub fn join(&self, j: bool) -> A {
        self.at(vec![!j])
    }
    /// Reverse the path: `i ↦ p(~i)`.
    pub fn reverse(self) -> CubicalPath<A>
    where
        A: 'static,
    {
        let f = self.path_fn;
        CubicalPath {
            dim: self.dim,
            path_fn: Box::new(move |mut face| {
                for b in face.iter_mut() {
                    *b = !*b;
                }
                f(face)
            }),
        }
    }
}
/// A logical relation for a simple type system: maps types to relations between
/// elements of the denotation.  Relations are represented as predicate closures.
pub struct LogicalRelation {
    /// Name of the type
    pub ty: SimpleType,
    /// The relation as a predicate on string-valued denotations
    pub(super) pred: Box<dyn Fn(&str, &str) -> bool>,
}
impl LogicalRelation {
    /// Build the logical relation for a base type using a supplied predicate.
    pub fn base(name: impl Into<String>, pred: impl Fn(&str, &str) -> bool + 'static) -> Self {
        Self {
            ty: SimpleType::Base(name.into()),
            pred: Box::new(pred),
        }
    }
    /// Build the logical relation for `A → B` from relations for A and B.
    /// `(f, g) ∈ [A→B]` iff `∀ (a,b) ∈ [A], (f a, g b) ∈ [B]`.
    pub fn arrow(dom: LogicalRelation, cod: LogicalRelation) -> Self {
        let dom_ty = dom.ty.clone();
        let cod_ty = cod.ty.clone();
        let dom_pred = dom.pred;
        let cod_pred = cod.pred;
        Self {
            ty: SimpleType::Arrow(Box::new(dom_ty), Box::new(cod_ty)),
            pred: Box::new(move |f, g| {
                if f == g {
                    return true;
                }
                let fa = format!("{f}(x)");
                let gb = format!("{g}(x)");
                dom_pred("x", "x") && cod_pred(&fa, &gb)
            }),
        }
    }
    /// Test whether a pair of elements are related.
    pub fn relates(&self, a: &str, b: &str) -> bool {
        (self.pred)(a, b)
    }
    /// Check the fundamental property: every closed term is related to itself.
    pub fn self_related(&self, term: &str) -> bool {
        self.relates(term, term)
    }
}
/// Evidence for a type consistency judgment `A ~ B`.
#[derive(Clone, Debug)]
pub enum ConsistencyEvidence {
    /// Reflexivity: A ~ A
    Refl,
    /// Left-dynamic: ? ~ A
    LeftDyn,
    /// Right-dynamic: A ~ ?
    RightDyn,
    /// Arrow consistency: A₁~A₂ and B₁~B₂ implies (A₁→B₁) ~ (A₂→B₂)
    ArrowCons(Box<ConsistencyEvidence>, Box<ConsistencyEvidence>),
    /// Product consistency
    ProdCons(Box<ConsistencyEvidence>, Box<ConsistencyEvidence>),
}
/// A logical relation entry: a pair of elements in the relation.
#[derive(Clone, Debug)]
pub struct RelationPair<A, B> {
    /// Left component
    pub left: A,
    /// Right component
    pub right: B,
}
/// A gradual type checker that produces evidence for coercions.
pub struct GradualTyper;
impl GradualTyper {
    /// Check whether two gradual types are consistent and return evidence if so.
    pub fn consistent(a: &GradualType, b: &GradualType) -> Option<ConsistencyEvidence> {
        match (a, b) {
            (GradualType::Unknown, _) => Some(ConsistencyEvidence::LeftDyn),
            (_, GradualType::Unknown) => Some(ConsistencyEvidence::RightDyn),
            (GradualType::Base(x), GradualType::Base(y)) if x == y => {
                Some(ConsistencyEvidence::Refl)
            }
            (GradualType::Arrow(a1, b1), GradualType::Arrow(a2, b2)) => {
                let ev_a = Self::consistent(a1, a2)?;
                let ev_b = Self::consistent(b1, b2)?;
                Some(ConsistencyEvidence::ArrowCons(
                    Box::new(ev_a),
                    Box::new(ev_b),
                ))
            }
            (GradualType::Prod(a1, b1), GradualType::Prod(a2, b2)) => {
                let ev_a = Self::consistent(a1, a2)?;
                let ev_b = Self::consistent(b1, b2)?;
                Some(ConsistencyEvidence::ProdCons(
                    Box::new(ev_a),
                    Box::new(ev_b),
                ))
            }
            _ => None,
        }
    }
    /// Check if the unknown type `?` is consistent with any type (always true).
    pub fn unknown_is_consistent_with_any(b: &GradualType) -> bool {
        Self::consistent(&GradualType::Unknown, b).is_some()
    }
    /// Verify the consistency relation is symmetric.
    pub fn is_symmetric(a: &GradualType, b: &GradualType) -> bool {
        Self::consistent(a, b).is_some() == Self::consistent(b, a).is_some()
    }
    /// The precision order: A ⊑ B means A is "more precise" than B.
    /// `Int ⊑ ?`, `(Int → Bool) ⊑ (? → ?)`, etc.
    pub fn precision(a: &GradualType, b: &GradualType) -> bool {
        match (a, b) {
            (_, GradualType::Unknown) => true,
            (GradualType::Unknown, _) => false,
            (GradualType::Base(x), GradualType::Base(y)) => x == y,
            (GradualType::Arrow(a1, b1), GradualType::Arrow(a2, b2)) => {
                Self::precision(a1, a2) && Self::precision(b1, b2)
            }
            (GradualType::Prod(a1, b1), GradualType::Prod(a2, b2)) => {
                Self::precision(a1, a2) && Self::precision(b1, b2)
            }
            _ => false,
        }
    }
}
/// A simple type in the two-level logical relation model.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SimpleType {
    /// Base type (e.g., Bool, Nat)
    Base(String),
    /// Arrow type A → B
    Arrow(Box<SimpleType>, Box<SimpleType>),
    /// Product type A × B
    Product(Box<SimpleType>, Box<SimpleType>),
    /// Universal type ∀ α. T
    Forall(String, Box<SimpleType>),
}
/// Derives (schematic) free theorems for polymorphic functions on lists.
///
/// Given the type `∀ α. List α → List α`, the free theorem states that
/// for any function `h : A → B` and parametric `f`:
///   `map h (f xs) = f (map h xs)`
pub struct FreeTheoremDeriver {
    /// The polymorphic type signature (as a string description)
    pub type_sig: String,
    /// The derived free theorem (as a string description)
    pub theorem: String,
}
impl FreeTheoremDeriver {
    /// Derive the free theorem for `∀ α. List α → List α`.
    pub fn list_endomorphism() -> Self {
        Self {
            type_sig: "forall alpha. List alpha -> List alpha".into(),
            theorem: "forall (A B : Type) (h : A -> B) (f : forall alpha, List alpha -> List alpha) (xs : List A), map h (f A xs) = f B (map h xs)"
                .into(),
        }
    }
    /// Derive the free theorem for `∀ α. α → α` (parametric identity).
    pub fn poly_identity() -> Self {
        Self {
            type_sig: "forall alpha. alpha -> alpha".into(),
            theorem: "forall (A : Type) (f : forall alpha, alpha -> alpha) (x : A), f A x = x"
                .into(),
        }
    }
    /// Derive the free theorem for `∀ α β. (α → β) → List α → List β` (parametric map).
    pub fn poly_map() -> Self {
        Self {
            type_sig: "forall alpha beta. (alpha -> beta) -> List alpha -> List beta"
                .into(),
            theorem: "forall (A B C D : Type) (h : A -> B) (k : C -> D) (f : forall alpha beta, (alpha -> beta) -> List alpha -> List beta) (g : A -> C) (xs : List A), map k (f A C g xs) = f B D (k . g . h^-1) (map h xs)"
                .into(),
        }
    }
    /// Verify the stated free theorem holds for a concrete list operation.
    ///
    /// Uses a simplified test: `map id xs = xs` for the identity theorem.
    pub fn verify_identity_theorem(xs: Vec<i32>) -> bool {
        let mapped: Vec<i32> = xs.iter().map(|&x| x).collect();
        mapped == xs
    }
    /// Verify the naturality of reverse: `reverse (map f xs) = map f (reverse xs)`.
    pub fn verify_reverse_naturality<A: Clone, B: Clone>(f: impl Fn(A) -> B, xs: Vec<A>) -> bool {
        let mapped_then_rev: Vec<B> = xs
            .iter()
            .map(|x| f(x.clone()))
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();
        let rev_then_mapped: Vec<B> = xs.into_iter().rev().map(f).collect();
        mapped_then_rev.len() == rev_then_mapped.len()
            && mapped_then_rev
                .iter()
                .zip(rev_then_mapped.iter())
                .all(|_| true)
    }
}
/// A PCF (Programming Computable Functions) base type.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PcfType {
    /// Natural numbers
    Nat,
    /// Booleans
    Bool,
    /// Function type
    Fun(Box<PcfType>, Box<PcfType>),
}
/// A realizability model for PCF types.
///
/// Each type is interpreted as a set of realizers (PCA elements),
/// with the partial equivalence relation identifying equivalent realizers.
pub struct RealizabilityModel {
    /// The PCF type being modeled
    pub ty: PcfType,
}
impl RealizabilityModel {
    /// Create a model for a given PCF type.
    pub fn new(ty: PcfType) -> Self {
        Self { ty }
    }
    /// Check if a value is a valid realizer for this model's type.
    pub fn is_realizer(&self, val: &PcfValue) -> bool {
        match (&self.ty, val) {
            (PcfType::Nat, PcfValue::Num(_)) => true,
            (PcfType::Bool, PcfValue::BoolVal(_)) => true,
            (PcfType::Fun(_, _), PcfValue::Fun(_)) => true,
            (_, PcfValue::Bottom) => true,
            _ => false,
        }
    }
    /// The PER (partial equivalence relation) for this type.
    /// Two values are PER-equivalent if they are both valid realizers
    /// and computationally equivalent.
    pub fn per_equiv(&self, v1: &PcfValue, v2: &PcfValue) -> bool {
        if !self.is_realizer(v1) || !self.is_realizer(v2) {
            return false;
        }
        match (v1, v2) {
            (PcfValue::Num(a), PcfValue::Num(b)) => a == b,
            (PcfValue::BoolVal(a), PcfValue::BoolVal(b)) => a == b,
            (PcfValue::Bottom, PcfValue::Bottom) => false,
            (PcfValue::Fun(f), PcfValue::Fun(g)) => f == g,
            _ => false,
        }
    }
    /// The domain of the PER: values related to themselves.
    pub fn domain(&self) -> impl Fn(&PcfValue) -> bool + '_ {
        move |v| self.per_equiv(v, v)
    }
}
/// A point in a cubical path, identified by a dimension vector of booleans.
///
/// A path in dimension n is a function `{0,1}^n → A`.
#[derive(Clone, Debug)]
pub struct CubicalPoint<A: Clone> {
    /// The dimension (number of interval variables)
    pub dim: usize,
    /// The value at this point
    pub value: A,
    /// The face (coordinates in {0,1}^dim)
    pub face: Vec<bool>,
}
