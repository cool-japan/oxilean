//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use crate::context::ElabContext;
use crate::elaborate::{elaborate_expr, elaborate_with_expected_type, ElabError};
use oxilean_kernel::{BinderInfo, Environment, Expr, Level, Name};
use oxilean_parse::{AttributeKind, Binder, Decl, Located, SurfaceExpr, WhereClause};

use std::collections::HashMap;

/// Processed attribute flags extracted from attribute list.
#[derive(Clone, Debug, Default)]
pub struct ProcessedAttrs {
    /// Is this a simp lemma?
    pub is_simp: bool,
    /// Is this an ext lemma?
    pub is_ext: bool,
    /// Is this a type class instance?
    pub is_instance: bool,
    /// Is this reducible?
    pub is_reducible: bool,
    /// Is this irreducible?
    pub is_irreducible: bool,
    /// Is this inline?
    pub is_inline: bool,
    /// Is this macro_inline?
    pub is_macro_inline: bool,
    /// Is this a specialize target?
    pub is_specialize: bool,
    /// Custom attribute names
    pub custom: Vec<String>,
}
/// A declaration that has been elaborated but not yet added to the environment.
#[derive(Clone, Debug)]
pub enum PendingDecl {
    /// Elaborated definition
    Definition {
        /// Declaration name
        name: Name,
        /// Elaborated type
        ty: Expr,
        /// Elaborated value
        val: Expr,
        /// Attributes
        attrs: Vec<AttributeKind>,
    },
    /// Elaborated theorem
    Theorem {
        /// Declaration name
        name: Name,
        /// Elaborated type (statement)
        ty: Expr,
        /// Elaborated proof
        proof: Expr,
        /// Attributes
        attrs: Vec<AttributeKind>,
    },
    /// Elaborated axiom
    Axiom {
        /// Declaration name
        name: Name,
        /// Elaborated type
        ty: Expr,
        /// Attributes
        attrs: Vec<AttributeKind>,
    },
    /// Elaborated inductive type
    Inductive {
        /// Type name
        name: Name,
        /// Elaborated type
        ty: Expr,
        /// Elaborated constructors (name, type)
        ctors: Vec<(Name, Expr)>,
        /// Attributes
        attrs: Vec<AttributeKind>,
    },
    /// Elaborated opaque constant
    Opaque {
        /// Declaration name
        name: Name,
        /// Elaborated type
        ty: Expr,
        /// Elaborated value
        val: Expr,
    },
}
impl PendingDecl {
    /// Get the name of the pending declaration.
    #[allow(dead_code)]
    pub fn name(&self) -> &Name {
        match self {
            PendingDecl::Definition { name, .. }
            | PendingDecl::Theorem { name, .. }
            | PendingDecl::Axiom { name, .. }
            | PendingDecl::Inductive { name, .. }
            | PendingDecl::Opaque { name, .. } => name,
        }
    }
    /// Get the type of the pending declaration.
    #[allow(dead_code)]
    pub fn ty(&self) -> &Expr {
        match self {
            PendingDecl::Definition { ty, .. }
            | PendingDecl::Theorem { ty, .. }
            | PendingDecl::Axiom { ty, .. }
            | PendingDecl::Inductive { ty, .. }
            | PendingDecl::Opaque { ty, .. } => ty,
        }
    }
    /// Check if this is a theorem.
    #[allow(dead_code)]
    pub fn is_theorem(&self) -> bool {
        matches!(self, PendingDecl::Theorem { .. })
    }
    /// Check if this is a definition.
    #[allow(dead_code)]
    pub fn is_definition(&self) -> bool {
        matches!(self, PendingDecl::Definition { .. })
    }
    /// Check if this is an axiom.
    #[allow(dead_code)]
    pub fn is_axiom(&self) -> bool {
        matches!(self, PendingDecl::Axiom { .. })
    }
    /// Check if this is an inductive type.
    #[allow(dead_code)]
    pub fn is_inductive(&self) -> bool {
        matches!(self, PendingDecl::Inductive { .. })
    }
}
/// A validator for pending declarations.
///
/// Checks that names are well-formed, types are non-trivially malformed,
/// and attributes are consistent before the declaration is admitted.
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct DeclValidator {
    /// Validation errors found.
    pub errors: Vec<ValidationError>,
    /// Validation warnings found.
    pub warnings: Vec<ValidationWarning>,
}
impl DeclValidator {
    /// Create a new validator.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Validate a pending declaration.
    #[allow(dead_code)]
    pub fn validate(&mut self, decl: &PendingDecl) {
        if decl.name().to_string().is_empty() {
            self.errors.push(ValidationError::EmptyName);
        }
        match decl {
            PendingDecl::Theorem { name, proof, .. } => {
                if expr_contains_sorry(proof) {
                    self.warnings
                        .push(ValidationWarning::SorryProof(name.clone()));
                }
            }
            PendingDecl::Inductive { name, ctors, .. } => {
                if ctors.is_empty() {
                    self.errors
                        .push(ValidationError::NoConstructors(name.clone()));
                }
                if ctors.len() > 64 {
                    self.warnings.push(ValidationWarning::ManyConstructors(
                        name.clone(),
                        ctors.len(),
                    ));
                }
            }
            _ => {}
        }
    }
    /// Whether any errors were found.
    #[allow(dead_code)]
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    /// Whether any warnings were found.
    #[allow(dead_code)]
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }
    /// Clear all errors and warnings.
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.errors.clear();
        self.warnings.clear();
    }
}
/// A warning found during declaration validation.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ValidationWarning {
    /// The declaration has a `sorry` in its proof.
    SorryProof(Name),
    /// The declaration has no type annotation.
    MissingAnnotation(Name),
    /// An inductive type has a very large number of constructors.
    ManyConstructors(Name, usize),
}
/// An error found during declaration validation.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ValidationError {
    /// The declaration name is empty.
    EmptyName,
    /// A theorem has a trivial (empty) proof.
    TrivialProof(Name),
    /// An inductive type has no constructors.
    NoConstructors(Name),
    /// Two attributes conflict with each other.
    ConflictingAttributes(Name, String),
}
/// A repository of pending declarations, indexed by name.
///
/// Provides insertion, lookup, and iteration over all pending declarations
/// in the order they were added.
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct DeclRepository {
    /// Declarations in insertion order.
    decls: Vec<PendingDecl>,
    /// Index from name to position in `decls`.
    index: std::collections::HashMap<Name, usize>,
}
impl DeclRepository {
    /// Create an empty repository.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Insert a declaration. Returns `false` if the name already exists.
    #[allow(dead_code)]
    pub fn insert(&mut self, decl: PendingDecl) -> bool {
        let name = decl.name().clone();
        if self.index.contains_key(&name) {
            return false;
        }
        let idx = self.decls.len();
        self.index.insert(name, idx);
        self.decls.push(decl);
        true
    }
    /// Look up a declaration by name.
    #[allow(dead_code)]
    pub fn get(&self, name: &Name) -> Option<&PendingDecl> {
        self.index.get(name).and_then(|&i| self.decls.get(i))
    }
    /// Number of declarations.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.decls.len()
    }
    /// Whether the repository is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.decls.is_empty()
    }
    /// Iterate over all declarations in insertion order.
    #[allow(dead_code)]
    pub fn iter(&self) -> impl Iterator<Item = &PendingDecl> {
        self.decls.iter()
    }
    /// Drain all declarations.
    #[allow(dead_code)]
    pub fn drain(&mut self) -> Vec<PendingDecl> {
        self.index.clear();
        std::mem::take(&mut self.decls)
    }
    /// Check if a name is already in the repository.
    #[allow(dead_code)]
    pub fn contains(&self, name: &Name) -> bool {
        self.index.contains_key(name)
    }
    /// All names in insertion order.
    #[allow(dead_code)]
    pub fn names(&self) -> Vec<&Name> {
        self.decls.iter().map(|d| d.name()).collect()
    }
}
/// A filter for selecting declarations by their kind or attributes.
#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct DeclFilter {
    /// If true, include definitions.
    pub include_definitions: bool,
    /// If true, include theorems.
    pub include_theorems: bool,
    /// If true, include axioms.
    pub include_axioms: bool,
    /// If true, include inductive types.
    pub include_inductives: bool,
    /// If set, only include declarations with this attribute.
    pub require_attr: Option<oxilean_parse::AttributeKind>,
}
impl DeclFilter {
    /// Create a filter that accepts everything.
    #[allow(dead_code)]
    pub fn all() -> Self {
        Self {
            include_definitions: true,
            include_theorems: true,
            include_axioms: true,
            include_inductives: true,
            require_attr: None,
        }
    }
    /// Create a filter that accepts only theorems.
    #[allow(dead_code)]
    pub fn theorems_only() -> Self {
        Self {
            include_theorems: true,
            ..Self::default()
        }
    }
    /// Create a filter that accepts only definitions.
    #[allow(dead_code)]
    pub fn definitions_only() -> Self {
        Self {
            include_definitions: true,
            ..Self::default()
        }
    }
    /// Create a filter that accepts only simp lemmas (theorems with Simp attr).
    #[allow(dead_code)]
    pub fn simp_lemmas() -> Self {
        Self {
            include_theorems: true,
            require_attr: Some(oxilean_parse::AttributeKind::Simp),
            ..Self::default()
        }
    }
    /// Check if a declaration passes this filter.
    #[allow(dead_code)]
    pub fn accepts(&self, decl: &PendingDecl) -> bool {
        let kind_ok = match decl {
            PendingDecl::Definition { .. } => self.include_definitions,
            PendingDecl::Theorem { .. } => self.include_theorems,
            PendingDecl::Axiom { .. } => self.include_axioms,
            PendingDecl::Inductive { .. } => self.include_inductives,
            PendingDecl::Opaque { .. } => self.include_definitions,
        };
        if !kind_ok {
            return false;
        }
        if let Some(ref required) = self.require_attr {
            let attrs = match decl {
                PendingDecl::Definition { attrs, .. } => attrs.as_slice(),
                PendingDecl::Theorem { attrs, .. } => attrs.as_slice(),
                PendingDecl::Axiom { attrs, .. } => attrs.as_slice(),
                PendingDecl::Inductive { attrs, .. } => attrs.as_slice(),
                PendingDecl::Opaque { .. } => &[],
            };
            return attrs.contains(required);
        }
        true
    }
}
/// Errors that can occur during declaration elaboration.
#[derive(Clone, Debug)]
pub enum DeclElabError {
    /// General elaboration error
    ElabError(String),
    /// Type mismatch between expected and actual
    TypeMismatch {
        /// Expected type description
        expected: String,
        /// Actual type description
        got: String,
    },
    /// Duplicate name in scope
    DuplicateName(String),
    /// Invalid recursion pattern
    InvalidRecursion(String),
    /// Missing type annotation where required
    MissingType(String),
    /// Unsupported declaration kind
    UnsupportedDecl(String),
}
/// Manages namespace prefixes for declarations.
///
/// Tracks the current namespace stack and provides utilities for
/// qualifying names with namespace prefixes.
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct NamespaceManager {
    /// Stack of namespace prefixes.
    stack: Vec<String>,
}
impl NamespaceManager {
    /// Create an empty namespace manager (root namespace).
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Push a namespace onto the stack.
    #[allow(dead_code)]
    pub fn push(&mut self, ns: impl Into<String>) {
        self.stack.push(ns.into());
    }
    /// Pop the current namespace.
    #[allow(dead_code)]
    pub fn pop(&mut self) -> Option<String> {
        self.stack.pop()
    }
    /// Get the current full namespace prefix.
    #[allow(dead_code)]
    pub fn current(&self) -> String {
        self.stack.join(".")
    }
    /// Qualify a name with the current namespace.
    #[allow(dead_code)]
    pub fn qualify(&self, name: &str) -> Name {
        if self.stack.is_empty() {
            Name::str(name)
        } else {
            Name::str(format!("{}.{}", self.current(), name))
        }
    }
    /// Depth of the namespace stack.
    #[allow(dead_code)]
    pub fn depth(&self) -> usize {
        self.stack.len()
    }
    /// Whether we're at the root namespace.
    #[allow(dead_code)]
    pub fn is_root(&self) -> bool {
        self.stack.is_empty()
    }
}
/// A pipeline that applies a sequence of transformations to pending declarations.
///
/// Each stage is a function from `PendingDecl` to `Option<PendingDecl>`.
/// Returning `None` drops the declaration.
#[allow(dead_code)]
pub struct DeclPipeline {
    stages: Vec<Box<dyn Fn(PendingDecl) -> Option<PendingDecl>>>,
}
impl DeclPipeline {
    /// Create an empty pipeline.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { stages: Vec::new() }
    }
    /// Add a stage to the pipeline.
    #[allow(dead_code)]
    pub fn add_stage<F: Fn(PendingDecl) -> Option<PendingDecl> + 'static>(&mut self, f: F) {
        self.stages.push(Box::new(f));
    }
    /// Run a declaration through the pipeline.
    ///
    /// Returns `None` if any stage drops the declaration.
    #[allow(dead_code)]
    pub fn run(&self, mut decl: PendingDecl) -> Option<PendingDecl> {
        for stage in &self.stages {
            decl = stage(decl)?;
        }
        Some(decl)
    }
    /// Run multiple declarations through the pipeline.
    #[allow(dead_code)]
    pub fn run_all(&self, decls: Vec<PendingDecl>) -> Vec<PendingDecl> {
        decls.into_iter().filter_map(|d| self.run(d)).collect()
    }
    /// Number of pipeline stages.
    #[allow(dead_code)]
    pub fn stage_count(&self) -> usize {
        self.stages.len()
    }
}
/// Top-level declaration elaborator.
///
/// This struct manages the state needed during elaboration of declarations,
/// including the environment, elaboration context, and any pending declarations.
#[allow(dead_code)]
pub struct DeclElaborator<'env> {
    /// Reference to the global environment
    env: &'env Environment,
    /// Elaboration context for the current declaration
    ctx: ElabContext<'env>,
    /// Pending declarations that have been elaborated but not committed
    pending_decls: Vec<PendingDecl>,
    /// Universe parameters for the current declaration
    univ_params: Vec<Name>,
}
impl<'env> DeclElaborator<'env> {
    /// Create a new declaration elaborator.
    pub fn new(env: &'env Environment) -> Self {
        Self {
            env,
            ctx: ElabContext::new(env),
            pending_decls: Vec::new(),
            univ_params: Vec::new(),
        }
    }
    /// Get the pending declarations.
    #[allow(dead_code)]
    pub fn pending_decls(&self) -> &[PendingDecl] {
        &self.pending_decls
    }
    /// Take all pending declarations out.
    #[allow(dead_code)]
    pub fn take_pending(&mut self) -> Vec<PendingDecl> {
        std::mem::take(&mut self.pending_decls)
    }
    /// Get the environment.
    #[allow(dead_code)]
    pub fn env(&self) -> &Environment {
        self.env
    }
    /// Register universe parameters for the current declaration.
    #[allow(dead_code)]
    #[allow(dead_code)]
    fn register_univ_params(&mut self, params: &[String]) {
        self.univ_params.clear();
        for p in params {
            let name = Name::str(p);
            self.univ_params.push(name.clone());
            self.ctx.push_univ_param(name);
        }
    }
    /// Reset context for a new declaration.
    #[allow(dead_code)]
    #[allow(dead_code)]
    fn reset_context(&mut self) {
        self.ctx = ElabContext::new(self.env);
        self.univ_params.clear();
    }
    /// Elaborate a declaration and add it to pending.
    #[allow(dead_code)]
    pub fn elaborate(&mut self, decl: &Decl) -> Result<PendingDecl, DeclElabError> {
        let pending = elaborate_decl(self.env, decl)?;
        self.pending_decls.push(pending.clone());
        Ok(pending)
    }
}
/// Statistics about a set of pending declarations.
#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct DeclStats {
    /// Number of definitions.
    pub definition_count: usize,
    /// Number of theorems.
    pub theorem_count: usize,
    /// Number of axioms.
    pub axiom_count: usize,
    /// Number of inductive types.
    pub inductive_count: usize,
    /// Number of theorems with sorry.
    pub sorry_count: usize,
    /// Number of simp lemmas.
    pub simp_count: usize,
}
impl DeclStats {
    /// Compute statistics from a slice of pending declarations.
    #[allow(dead_code)]
    pub fn from_decls(decls: &[PendingDecl]) -> Self {
        let mut stats = DeclStats::default();
        for decl in decls {
            match decl {
                PendingDecl::Definition { .. } => stats.definition_count += 1,
                PendingDecl::Theorem { proof, attrs, .. } => {
                    stats.theorem_count += 1;
                    if expr_contains_sorry(proof) {
                        stats.sorry_count += 1;
                    }
                    if attrs.contains(&oxilean_parse::AttributeKind::Simp) {
                        stats.simp_count += 1;
                    }
                }
                PendingDecl::Axiom { .. } => stats.axiom_count += 1,
                PendingDecl::Inductive { .. } => stats.inductive_count += 1,
                PendingDecl::Opaque { .. } => stats.definition_count += 1,
            }
        }
        stats
    }
    /// Total number of declarations.
    #[allow(dead_code)]
    pub fn total(&self) -> usize {
        self.definition_count + self.theorem_count + self.axiom_count + self.inductive_count
    }
}
