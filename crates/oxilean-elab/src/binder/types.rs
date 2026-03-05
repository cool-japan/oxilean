//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use crate::context::ElabContext;
use oxilean_kernel::{BinderInfo, Expr, FVarId, Level, Name};
use oxilean_parse::{Binder, BinderKind, Located, SurfaceExpr};

/// Result of elaborating a single binder.
///
/// Contains the elaborated name, type, binder info, and the free variable
/// ID allocated for this binder in the local context.
#[derive(Clone, Debug)]
pub struct BinderElabResult {
    /// The binder name
    pub name: Name,
    /// The elaborated type
    pub ty: Expr,
    /// Binder info (explicit, implicit, instance, strict implicit)
    pub info: BinderInfo,
    /// The free variable ID allocated for this binder
    pub fvar: FVarId,
}
/// Count of binders by kind.
#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
pub struct BinderKindCount {
    /// Number of explicit (default) binders.
    pub explicit: usize,
    /// Number of implicit `{x}` binders.
    pub implicit: usize,
    /// Number of instance `[x]` binders.
    pub instance: usize,
    /// Number of strict `⦃x⦄` binders.
    pub strict: usize,
}
impl BinderKindCount {
    /// Total binder count.
    #[allow(dead_code)]
    pub fn total(&self) -> usize {
        self.explicit + self.implicit + self.instance + self.strict
    }
    /// Number of binders that are any form of implicit.
    #[allow(dead_code)]
    pub fn any_implicit(&self) -> usize {
        self.implicit + self.instance + self.strict
    }
}
/// Strategy for inferring a binder's type from surrounding context.
#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum BinderTypeInference {
    /// Infer from the expected type of the expression being built.
    FromExpected,
    /// Infer from the type of a sibling binder.
    FromSibling(usize),
    /// Infer from the environment (the constant has a known type).
    FromEnvironment,
    /// Cannot infer; a metavariable will be created.
    Fresh,
}
/// Information about a candidate for auto-bound implicit binding.
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct AutoBoundImplicitInfo {
    /// The variable name.
    pub name: String,
    /// Whether this looks like a universe-polymorphic type variable.
    pub looks_like_type_var: bool,
    /// Whether this looks like a proposition variable.
    pub looks_like_prop_var: bool,
    /// Whether this looks like a term-level variable.
    pub looks_like_term_var: bool,
}
impl AutoBoundImplicitInfo {
    /// Create an info record for a variable name.
    #[allow(dead_code)]
    pub fn for_name(name: &str) -> Self {
        let first = name.chars().next().unwrap_or('_');
        Self {
            name: name.to_string(),
            looks_like_type_var: is_greek_letter(first)
                || (first.is_uppercase() && name.len() == 1),
            looks_like_prop_var: matches!(first, 'P' | 'Q' | 'R') && name.len() == 1,
            looks_like_term_var: first.is_lowercase() && name.len() == 1,
        }
    }
}
/// Result of elaborating binder types only (without pushing to context).
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct BinderTypeResult {
    /// The binder name
    pub name: Name,
    /// The elaborated type
    pub ty: Expr,
    /// Binder info
    pub info: BinderInfo,
}
/// Distinguish strict implicit `⦃x⦄` from regular `{x}` and instance `[x]`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ImplicitStrictness {
    /// Regular implicit `{x}` — insert metavar eagerly.
    Regular,
    /// Strict implicit `⦃x⦄` — only insert when determinable.
    Strict,
    /// Instance implicit `[x]` — resolve by typeclass synthesis.
    Instance,
}
impl ImplicitStrictness {
    /// Derive from a `BinderKind`.
    #[allow(dead_code)]
    pub fn from_kind(kind: &BinderKind) -> Option<Self> {
        match kind {
            BinderKind::Implicit => Some(ImplicitStrictness::Regular),
            BinderKind::StrictImplicit => Some(ImplicitStrictness::Strict),
            BinderKind::Instance => Some(ImplicitStrictness::Instance),
            BinderKind::Default => None,
        }
    }
    /// Whether this implicit should be inserted eagerly.
    #[allow(dead_code)]
    pub fn is_eager(self) -> bool {
        self == ImplicitStrictness::Regular
    }
}
/// A telescope is a sequence of typed binders.
#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
pub struct Telescope {
    /// The individual binders in order.
    pub binders: Vec<BinderElabResult>,
}
impl Telescope {
    /// Create an empty telescope.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Number of binders.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.binders.len()
    }
    /// Whether the telescope is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.binders.is_empty()
    }
    /// Append a binder.
    #[allow(dead_code)]
    pub fn push(&mut self, b: BinderElabResult) {
        self.binders.push(b);
    }
    /// Build a Pi-type from this telescope and a body.
    #[allow(dead_code)]
    pub fn to_pi(&self, body: Expr) -> Expr {
        pi_binders(&self.binders, body)
    }
    /// Build a lambda abstraction from this telescope and a body.
    #[allow(dead_code)]
    pub fn to_lam(&self, body: Expr) -> Expr {
        abstract_binders(&self.binders, body)
    }
    /// Get the FVar IDs in telescope order.
    #[allow(dead_code)]
    pub fn fvars(&self) -> Vec<FVarId> {
        self.binders.iter().map(|b| b.fvar).collect()
    }
    /// Get the binder names in telescope order.
    #[allow(dead_code)]
    pub fn names(&self) -> Vec<Name> {
        self.binders.iter().map(|b| b.name.clone()).collect()
    }
    /// Get the binder types in telescope order.
    #[allow(dead_code)]
    pub fn types(&self) -> Vec<Expr> {
        self.binders.iter().map(|b| b.ty.clone()).collect()
    }
    /// Split into implicit prefix and explicit suffix.
    #[allow(dead_code)]
    pub fn split_implicit(&self) -> (Vec<&BinderElabResult>, Vec<&BinderElabResult>) {
        let mut implicit = Vec::new();
        let mut explicit = Vec::new();
        let mut saw_explicit = false;
        for b in &self.binders {
            if saw_explicit || b.info == BinderInfo::Default {
                saw_explicit = true;
                explicit.push(b);
            } else {
                implicit.push(b);
            }
        }
        (implicit, explicit)
    }
}
/// A directed edge in the binder dependency graph.
#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub struct BinderDep {
    /// The binder index that depends on another.
    pub from: usize,
    /// The binder index that is depended upon.
    pub to: usize,
}
/// A binder together with an optional default value.
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct BinderWithDefault {
    /// The underlying elaborated binder.
    pub result: BinderElabResult,
    /// Optional default value expression.
    pub default_val: Option<Expr>,
}
impl BinderWithDefault {
    /// Create a binder with no default.
    #[allow(dead_code)]
    pub fn no_default(result: BinderElabResult) -> Self {
        Self {
            result,
            default_val: None,
        }
    }
    /// Create a binder with a default value.
    #[allow(dead_code)]
    pub fn with_default(result: BinderElabResult, val: Expr) -> Self {
        Self {
            result,
            default_val: Some(val),
        }
    }
    /// Whether this binder has a default value.
    #[allow(dead_code)]
    pub fn has_default(&self) -> bool {
        self.default_val.is_some()
    }
    /// Get the default value or a fresh metavariable fallback.
    #[allow(dead_code)]
    pub fn get_or_meta(&self, ctx: &mut ElabContext) -> Expr {
        if let Some(val) = &self.default_val {
            val.clone()
        } else {
            let (_id, meta) = ctx.fresh_meta(self.result.ty.clone());
            meta
        }
    }
}
/// Universe classification for a binder's type.
#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum BinderUniverse {
    /// Lives in `Prop` (Sort 0).
    Prop,
    /// Lives in `Type u` (Sort (succ _)).
    Type,
    /// Lives in some `Sort u`.
    Sort,
    /// Cannot determine the universe.
    Unknown,
}
/// Snapshot of a binder scope — free variables currently live in context.
#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
pub struct BinderScope {
    /// Free variable IDs in scope.
    pub live: Vec<FVarId>,
    /// Names in scope in declaration order.
    pub names: Vec<Name>,
}
impl BinderScope {
    /// Create an empty scope.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Extend with a new binding.
    #[allow(dead_code)]
    pub fn push(&mut self, fvar: FVarId, name: Name) {
        self.live.push(fvar);
        self.names.push(name);
    }
    /// Remove the most recent binding.
    #[allow(dead_code)]
    pub fn pop(&mut self) {
        self.live.pop();
        self.names.pop();
    }
    /// Check whether a name is in scope.
    #[allow(dead_code)]
    pub fn contains_name(&self, name: &Name) -> bool {
        self.names.contains(name)
    }
    /// Check whether an FVar is in scope.
    #[allow(dead_code)]
    pub fn contains_fvar(&self, fvar: FVarId) -> bool {
        self.live.contains(&fvar)
    }
    /// Number of live bindings.
    #[allow(dead_code)]
    pub fn depth(&self) -> usize {
        self.live.len()
    }
    /// Produce a child scope with the same bindings.
    #[allow(dead_code)]
    pub fn child(&self) -> Self {
        self.clone()
    }
    /// Build a scope from elaborated binders.
    #[allow(dead_code)]
    pub fn from_binders(binders: &[BinderElabResult]) -> Self {
        let mut scope = Self::new();
        for b in binders {
            scope.push(b.fvar, b.name.clone());
        }
        scope
    }
}
/// Error type for binder validation.
#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum BinderValidationError {
    /// A binder name is empty.
    EmptyName,
    /// A binder name is a reserved keyword.
    ReservedName(String),
    /// A type annotation is not valid in this position.
    InvalidTypeAnnotation(String),
    /// Mixing anonymous and named binders is ambiguous.
    AmbiguousMixedBinders,
    /// An instance binder `[x]` has no type annotation.
    InstanceBinderWithoutType,
    /// Other validation error.
    Other(String),
}
