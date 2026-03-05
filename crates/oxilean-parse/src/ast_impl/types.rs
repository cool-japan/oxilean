//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

#[allow(unused_imports)]
use crate::ast::{SimpleNodeKindExt, TreeNodeExt};
use crate::tokens::{Span, StringPart};

use super::functions::*;

/// Binder kind.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BinderKind {
    /// Default (explicit argument)
    Default,
    /// Implicit argument: {x : alpha}
    Implicit,
    /// Instance argument: [x : alpha]
    Instance,
    /// Strict implicit: {{x : alpha}}
    StrictImplicit,
}
/// A path in a tree, represented as a sequence of child indices.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreePathExt(pub Vec<usize>);
impl TreePathExt {
    /// Create an empty (root) path.
    #[allow(dead_code)]
    pub fn root() -> Self {
        TreePathExt(Vec::new())
    }
    /// Extend this path with a child index.
    #[allow(dead_code)]
    pub fn child(&self, idx: usize) -> Self {
        let mut v = self.0.clone();
        v.push(idx);
        TreePathExt(v)
    }
    /// Returns the depth of this path.
    #[allow(dead_code)]
    pub fn depth(&self) -> usize {
        self.0.len()
    }
}
/// Computes tree statistics (depth, breadth, leaf count).
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Default)]
pub struct TreeStatsExt2 {
    /// Maximum depth of the tree
    pub max_depth: usize,
    /// Total number of nodes
    pub node_count: usize,
    /// Total number of leaves
    pub leaf_count: usize,
    /// Maximum branching factor seen
    pub max_branching: usize,
}
/// Binder for lambda and Pi types.
#[derive(Clone, Debug, PartialEq)]
pub struct Binder {
    /// Variable name (can be _ for unused)
    pub name: String,
    /// Type annotation (optional for lambda, required for Pi)
    pub ty: Option<Box<Located<SurfaceExpr>>>,
    /// Binder info (implicit, instance, etc.)
    pub info: BinderKind,
}
/// A where clause for local definitions attached to a definition or theorem.
///
/// Example:
/// ```text
/// def foo (n : Nat) : Nat := bar n + baz n where
///   bar (x : Nat) : Nat := x + 1
///   baz (x : Nat) : Nat := x * 2
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct WhereClause {
    /// Name of the local definition.
    pub name: String,
    /// Parameters of the local definition.
    pub params: Vec<Binder>,
    /// Optional type annotation.
    pub ty: Option<Located<SurfaceExpr>>,
    /// Value (body) of the local definition.
    pub val: Located<SurfaceExpr>,
}
impl WhereClause {
    /// Create a new where clause.
    #[allow(dead_code)]
    pub fn new(
        name: String,
        params: Vec<Binder>,
        ty: Option<Located<SurfaceExpr>>,
        val: Located<SurfaceExpr>,
    ) -> Self {
        Self {
            name,
            params,
            ty,
            val,
        }
    }
}
/// A single step in a `calc` proof expression.
///
/// Example:
/// ```text
/// calc a = b := proof1
///      _ < c := proof2
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct CalcStep {
    /// Left-hand side of the step.
    pub lhs: Located<SurfaceExpr>,
    /// Relation symbol (e.g., `=`, `<`, `<=`).
    pub rel: String,
    /// Right-hand side of the step.
    pub rhs: Located<SurfaceExpr>,
    /// Proof/justification for this step.
    pub proof: Located<SurfaceExpr>,
}
impl CalcStep {
    /// Create a new calc step.
    #[allow(dead_code)]
    pub fn new(
        lhs: Located<SurfaceExpr>,
        rel: String,
        rhs: Located<SurfaceExpr>,
        proof: Located<SurfaceExpr>,
    ) -> Self {
        Self {
            lhs,
            rel,
            rhs,
            proof,
        }
    }
}
/// Surface-level expression (before elaboration).
#[derive(Clone, Debug, PartialEq)]
pub enum SurfaceExpr {
    /// Sort: Type, Prop, Type u, Sort u
    Sort(SortKind),
    /// Variable reference
    Var(String),
    /// Application: f a
    App(Box<Located<SurfaceExpr>>, Box<Located<SurfaceExpr>>),
    /// Lambda: fun (x : alpha) => body  or  lambda (x : alpha), body
    Lam(Vec<Binder>, Box<Located<SurfaceExpr>>),
    /// Pi type: forall (x : alpha), beta  or  (x : alpha) -> beta
    Pi(Vec<Binder>, Box<Located<SurfaceExpr>>),
    /// Let binding: let x : alpha := v in body
    Let(
        String,
        Option<Box<Located<SurfaceExpr>>>,
        Box<Located<SurfaceExpr>>,
        Box<Located<SurfaceExpr>>,
    ),
    /// Literal value
    Lit(Literal),
    /// Explicit type annotation: (e : tau)
    Ann(Box<Located<SurfaceExpr>>, Box<Located<SurfaceExpr>>),
    /// Placeholder: _
    Hole,
    /// Projection: e.n
    Proj(Box<Located<SurfaceExpr>>, String),
    /// If-then-else: if c then t else e
    If(
        Box<Located<SurfaceExpr>>,
        Box<Located<SurfaceExpr>>,
        Box<Located<SurfaceExpr>>,
    ),
    /// Match expression: match e with | pat => rhs | ...
    Match(Box<Located<SurfaceExpr>>, Vec<MatchArm>),
    /// Do notation: do { ... }
    Do(Vec<DoAction>),
    /// Have expression: have h : T := proof; body
    Have(
        String,
        Box<Located<SurfaceExpr>>,
        Box<Located<SurfaceExpr>>,
        Box<Located<SurfaceExpr>>,
    ),
    /// Suffices expression: suffices h : T by tactic; body
    Suffices(String, Box<Located<SurfaceExpr>>, Box<Located<SurfaceExpr>>),
    /// Show expression: show T from expr
    Show(Box<Located<SurfaceExpr>>, Box<Located<SurfaceExpr>>),
    /// Named argument application: f (x := e)
    NamedArg(Box<Located<SurfaceExpr>>, String, Box<Located<SurfaceExpr>>),
    /// Anonymous constructor: (langle) a, b, c (rangle)
    AnonymousCtor(Vec<Located<SurfaceExpr>>),
    /// List literal: [1, 2, 3]
    ListLit(Vec<Located<SurfaceExpr>>),
    /// Tuple: (a, b) or (a, b, c)
    Tuple(Vec<Located<SurfaceExpr>>),
    /// Return expression (used in `do` notation): `return e`
    Return(Box<Located<SurfaceExpr>>),
    /// String interpolation: `s!"hello {name}"`
    StringInterp(Vec<StringPart>),
    /// Range expression: `a..b`, `..b`, `a..`
    Range(
        Option<Box<Located<SurfaceExpr>>>,
        Option<Box<Located<SurfaceExpr>>>,
    ),
    /// By-tactic expression: `by tactic1; tactic2; ...`
    ByTactic(Vec<Located<TacticRef>>),
    /// Calc expression: calculational proof
    Calc(Vec<CalcStep>),
}
impl SurfaceExpr {
    /// Create a variable expression.
    #[allow(dead_code)]
    pub fn var(name: &str) -> Self {
        SurfaceExpr::Var(name.to_string())
    }
    /// Create a natural number literal expression.
    #[allow(dead_code)]
    pub fn nat(n: u64) -> Self {
        SurfaceExpr::Lit(Literal::Nat(n))
    }
    /// Create a string literal expression.
    #[allow(dead_code)]
    pub fn string(s: &str) -> Self {
        SurfaceExpr::Lit(Literal::String(s.to_string()))
    }
    /// Create a float literal expression.
    #[allow(dead_code)]
    pub fn float(v: f64) -> Self {
        SurfaceExpr::Lit(Literal::Float(v))
    }
    /// Check if this expression is a hole/placeholder.
    #[allow(dead_code)]
    pub fn is_hole(&self) -> bool {
        matches!(self, SurfaceExpr::Hole)
    }
    /// Check if this expression is a variable.
    #[allow(dead_code)]
    pub fn is_var(&self) -> bool {
        matches!(self, SurfaceExpr::Var(_))
    }
    /// Get the variable name if this is a variable.
    #[allow(dead_code)]
    pub fn as_var(&self) -> Option<&str> {
        match self {
            SurfaceExpr::Var(s) => Some(s),
            _ => None,
        }
    }
}
/// Match arm: pattern => expression
#[derive(Clone, Debug, PartialEq)]
pub struct MatchArm {
    /// Pattern
    pub pattern: Located<Pattern>,
    /// Guard (optional): | when condition
    pub guard: Option<Located<SurfaceExpr>>,
    /// Right-hand side
    pub rhs: Located<SurfaceExpr>,
}
/// A simple tree zipper for navigating trees.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct TreeZipper {
    /// Current focus node
    pub focus: TreeNodeExt,
    /// Context: (parent_label, left_siblings, right_siblings)
    pub context: Vec<(String, Vec<TreeNodeExt>, Vec<TreeNodeExt>)>,
}
impl TreeZipper {
    /// Create a zipper from a tree.
    #[allow(dead_code)]
    pub fn new(tree: TreeNodeExt) -> Self {
        TreeZipper {
            focus: tree,
            context: Vec::new(),
        }
    }
    /// Go down into the i-th child.
    #[allow(dead_code)]
    pub fn down(mut self, i: usize) -> Option<Self> {
        if i >= self.focus.children.len() {
            return None;
        }
        let mut children = self.focus.children.clone();
        let child = children.remove(i);
        let left = children[..i].to_vec();
        let right = children[i..].to_vec();
        self.context.push((self.focus.label.clone(), left, right));
        Some(TreeZipper {
            focus: child,
            context: self.context,
        })
    }
    /// Returns the depth (number of levels descended).
    #[allow(dead_code)]
    pub fn depth(&self) -> usize {
        self.context.len()
    }
    /// Returns the label of the current focus node.
    #[allow(dead_code)]
    pub fn label(&self) -> &str {
        &self.focus.label
    }
}
/// Pattern for match expressions.
#[derive(Clone, Debug, PartialEq)]
pub enum Pattern {
    /// Wildcard _
    Wild,
    /// Variable binding
    Var(String),
    /// Constructor application: C p1 p2 ...
    Ctor(String, Vec<Located<Pattern>>),
    /// Literal pattern
    Lit(Literal),
    /// Or pattern: p1 | p2
    Or(Box<Located<Pattern>>, Box<Located<Pattern>>),
}
/// Top-level declaration.
#[derive(Clone, Debug, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub enum Decl {
    /// Axiom declaration
    Axiom {
        /// Name
        name: String,
        /// Universe parameters
        univ_params: Vec<String>,
        /// Type
        ty: Located<SurfaceExpr>,
        /// Attributes
        attrs: Vec<AttributeKind>,
    },
    /// Definition
    Definition {
        /// Name
        name: String,
        /// Universe parameters
        univ_params: Vec<String>,
        /// Type (optional, can be inferred)
        ty: Option<Located<SurfaceExpr>>,
        /// Value
        val: Located<SurfaceExpr>,
        /// Where clauses (local definitions)
        where_clauses: Vec<WhereClause>,
        /// Attributes
        attrs: Vec<AttributeKind>,
    },
    /// Theorem
    Theorem {
        /// Name
        name: String,
        /// Universe parameters
        univ_params: Vec<String>,
        /// Type (statement)
        ty: Located<SurfaceExpr>,
        /// Proof
        proof: Located<SurfaceExpr>,
        /// Where clauses (local definitions)
        where_clauses: Vec<WhereClause>,
        /// Attributes
        attrs: Vec<AttributeKind>,
    },
    /// Inductive type
    Inductive {
        /// Name
        name: String,
        /// Universe parameters
        univ_params: Vec<String>,
        /// Parameters (non-varying)
        params: Vec<Binder>,
        /// Indices (varying)
        indices: Vec<Binder>,
        /// Type
        ty: Located<SurfaceExpr>,
        /// Constructors
        ctors: Vec<Constructor>,
    },
    /// Import declaration
    Import {
        /// Module path
        path: Vec<String>,
    },
    /// Namespace declaration
    Namespace {
        /// Namespace name
        name: String,
        /// Declarations inside
        decls: Vec<Located<Decl>>,
    },
    /// Structure declaration
    Structure {
        /// Structure name
        name: String,
        /// Universe parameters
        univ_params: Vec<String>,
        /// Parent structures (extends)
        extends: Vec<String>,
        /// Fields
        fields: Vec<FieldDecl>,
    },
    /// Class declaration
    ClassDecl {
        /// Class name
        name: String,
        /// Universe parameters
        univ_params: Vec<String>,
        /// Parent classes (extends)
        extends: Vec<String>,
        /// Fields/methods
        fields: Vec<FieldDecl>,
    },
    /// Instance declaration
    InstanceDecl {
        /// Optional instance name
        name: Option<String>,
        /// Class being instantiated
        class_name: String,
        /// The instance type
        ty: Located<SurfaceExpr>,
        /// Method definitions
        defs: Vec<(String, Located<SurfaceExpr>)>,
    },
    /// Section declaration
    SectionDecl {
        /// Section name
        name: String,
        /// Declarations inside
        decls: Vec<Located<Decl>>,
    },
    /// Variable declaration
    Variable {
        /// Binders
        binders: Vec<Binder>,
    },
    /// Open declaration
    Open {
        /// Module path
        path: Vec<String>,
        /// Specific names to open (empty = open all)
        names: Vec<String>,
    },
    /// Attribute declaration
    Attribute {
        /// Attribute names
        attrs: Vec<String>,
        /// Inner declaration
        decl: Box<Located<Decl>>,
    },
    /// Hash command (#check, #eval, #print)
    HashCmd {
        /// Command name (e.g. "check", "eval", "print")
        cmd: String,
        /// Argument expression
        arg: Located<SurfaceExpr>,
    },
    /// Mutual recursive definitions
    Mutual {
        /// The mutually recursive declarations
        decls: Vec<Located<Decl>>,
    },
    /// Derive declaration: `deriving instance1, instance2 for TypeName`
    Derive {
        /// Instance strategies to derive
        instances: Vec<String>,
        /// Type name to derive for
        type_name: String,
    },
    /// Notation declaration
    NotationDecl {
        /// Kind of notation
        kind: AstNotationKind,
        /// Precedence (optional)
        prec: Option<u32>,
        /// Name or symbol
        name: String,
        /// Notation body
        notation: String,
    },
    /// Universe declaration: `universe u v w`
    Universe {
        /// Universe variable names
        names: Vec<String>,
    },
}
impl Decl {
    /// Get the name of this declaration, if it has one.
    #[allow(dead_code)]
    pub fn name(&self) -> Option<&str> {
        match self {
            Decl::Axiom { name, .. }
            | Decl::Definition { name, .. }
            | Decl::Theorem { name, .. }
            | Decl::Inductive { name, .. }
            | Decl::Namespace { name, .. }
            | Decl::Structure { name, .. }
            | Decl::ClassDecl { name, .. }
            | Decl::SectionDecl { name, .. } => Some(name),
            Decl::InstanceDecl { name, .. } => name.as_deref(),
            Decl::Derive { type_name, .. } => Some(type_name),
            Decl::NotationDecl { name, .. } => Some(name),
            _ => None,
        }
    }
    /// Get the attributes if this declaration has typed attributes.
    #[allow(dead_code)]
    pub fn typed_attrs(&self) -> &[AttributeKind] {
        match self {
            Decl::Axiom { attrs, .. }
            | Decl::Definition { attrs, .. }
            | Decl::Theorem { attrs, .. } => attrs,
            _ => &[],
        }
    }
    /// Check if this declaration is a mutual block.
    #[allow(dead_code)]
    pub fn is_mutual(&self) -> bool {
        matches!(self, Decl::Mutual { .. })
    }
}
/// Kind of notation declaration in the AST.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AstNotationKind {
    /// Prefix operator (e.g., `-x`)
    Prefix,
    /// Postfix operator (e.g., `x!`)
    Postfix,
    /// Left-associative infix operator
    Infixl,
    /// Right-associative infix operator
    Infixr,
    /// General notation
    Notation,
}
/// A located expression (expression with source span).
#[derive(Clone, Debug, PartialEq)]
pub struct Located<T> {
    /// The value
    pub value: T,
    /// Source span
    pub span: Span,
}
impl<T> Located<T> {
    /// Create a new located value.
    pub fn new(value: T, span: Span) -> Self {
        Self { value, span }
    }
    /// Map the inner value while preserving the span.
    #[allow(dead_code)]
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Located<U> {
        Located {
            value: f(self.value),
            span: self.span,
        }
    }
}
/// Typed attribute kinds for declarations.
///
/// In Lean 4, attributes like `@[simp]`, `@[ext]`, `@[reducible]` control
/// how the elaborator and tactics treat definitions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AttributeKind {
    /// `@[simp]` - add to the simp set
    Simp,
    /// `@[ext]` - extensionality lemma
    Ext,
    /// `@[instance]` - typeclass instance
    Instance,
    /// `@[reducible]` - mark as reducible for definitional equality
    Reducible,
    /// `@[irreducible]` - mark as irreducible
    Irreducible,
    /// `@[inline]` - inline during compilation
    Inline,
    /// `@[noinline]` - do not inline
    NoInline,
    /// `@[specialize]` - specialization hint
    SpecializeAttr,
    /// Custom user attribute
    Custom(String),
}
impl AttributeKind {
    /// Check if this is a custom attribute.
    #[allow(dead_code)]
    pub fn is_custom(&self) -> bool {
        matches!(self, AttributeKind::Custom(_))
    }
    /// Get the name of the attribute as a string.
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            AttributeKind::Simp => "simp",
            AttributeKind::Ext => "ext",
            AttributeKind::Instance => "instance",
            AttributeKind::Reducible => "reducible",
            AttributeKind::Irreducible => "irreducible",
            AttributeKind::Inline => "inline",
            AttributeKind::NoInline => "noinline",
            AttributeKind::SpecializeAttr => "specialize",
            AttributeKind::Custom(s) => s,
        }
    }
}
/// Literal values.
#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    /// Natural number
    Nat(u64),
    /// Float
    Float(f64),
    /// String
    String(String),
    /// Character
    Char(char),
}
/// Do notation action.
#[derive(Clone, Debug, PartialEq)]
pub enum DoAction {
    /// let x := e
    Let(String, Located<SurfaceExpr>),
    /// let x : tau := e
    LetTyped(String, Located<SurfaceExpr>, Located<SurfaceExpr>),
    /// x <- e
    Bind(String, Located<SurfaceExpr>),
    /// pure expression
    Expr(Located<SurfaceExpr>),
    /// return e
    Return(Located<SurfaceExpr>),
}
/// Field declaration for structures and classes.
#[derive(Clone, Debug, PartialEq)]
pub struct FieldDecl {
    /// Field name
    pub name: String,
    /// Field type
    pub ty: Located<SurfaceExpr>,
    /// Default value (optional)
    pub default: Option<Located<SurfaceExpr>>,
}
/// An identity transform that returns the input unchanged.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct IdentityTransformExt;
/// Constructor for inductive types.
#[derive(Clone, Debug, PartialEq)]
pub struct Constructor {
    /// Constructor name
    pub name: String,
    /// Constructor type
    pub ty: Located<SurfaceExpr>,
}
/// A memo table for tree transformations (cache-aside pattern).
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct TransformMemo {
    /// Cached results keyed by fingerprint
    pub cache: std::collections::HashMap<u64, TreeNodeExt>,
}
impl TransformMemo {
    /// Create a new empty memo table.
    #[allow(dead_code)]
    pub fn new() -> Self {
        TransformMemo {
            cache: std::collections::HashMap::new(),
        }
    }
    /// Look up a cached result.
    #[allow(dead_code)]
    pub fn get(&self, key: u64) -> Option<&TreeNodeExt> {
        self.cache.get(&key)
    }
    /// Store a result.
    #[allow(dead_code)]
    pub fn store(&mut self, key: u64, result: TreeNodeExt) {
        self.cache.insert(key, result);
    }
    /// Returns the number of cached entries.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.cache.len()
    }
    /// Returns true if empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }
}
/// A tree statistics collector.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Default)]
pub struct TreeStats {
    /// Total node count
    pub nodes: usize,
    /// Leaf node count
    pub leaves: usize,
    /// Maximum depth
    pub max_depth: usize,
    /// Total tree size
    pub total_size: usize,
}
impl TreeStats {
    /// Create an empty stats record.
    #[allow(dead_code)]
    pub fn new() -> Self {
        TreeStats::default()
    }
    /// Compute stats from a tree.
    #[allow(dead_code)]
    pub fn from_tree(root: &TreeNodeExt) -> Self {
        let mut stats = TreeStats::new();
        collect_stats(root, 0, &mut stats);
        stats
    }
}
/// A zipper-based tree cursor for incremental editing.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct TreeCursor {
    /// Current node
    pub current: TreeNodeExt,
    /// Path back to root
    pub ancestors: Vec<TreeNodeExt>,
}
impl TreeCursor {
    /// Create a cursor at the root.
    #[allow(dead_code)]
    pub fn new(root: TreeNodeExt) -> Self {
        TreeCursor {
            current: root,
            ancestors: Vec::new(),
        }
    }
    /// Returns the kind of the current node.
    #[allow(dead_code)]
    pub fn kind(&self) -> &SimpleNodeKindExt {
        &self.current.kind
    }
    /// Returns the label of the current node.
    #[allow(dead_code)]
    pub fn label(&self) -> &str {
        &self.current.label
    }
    /// Returns the child count.
    #[allow(dead_code)]
    pub fn child_count(&self) -> usize {
        self.current.children.len()
    }
    /// Returns the depth.
    #[allow(dead_code)]
    pub fn depth(&self) -> usize {
        self.ancestors.len()
    }
}
/// A tree zipper for navigation.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct TreeZipperExt {
    /// The focused node
    pub focus: TreeNodeExt,
    /// The path to the focus
    pub path: TreePathExt,
    /// Ancestors (parent, sibling index, left siblings, right siblings)
    pub ancestors: Vec<(
        SimpleNodeKindExt,
        String,
        Vec<TreeNodeExt>,
        Vec<TreeNodeExt>,
    )>,
}
impl TreeZipperExt {
    /// Create a zipper from a root.
    #[allow(dead_code)]
    pub fn new(root: TreeNodeExt) -> Self {
        TreeZipperExt {
            focus: root,
            path: TreePathExt::root(),
            ancestors: Vec::new(),
        }
    }
    /// Move into the i-th child.
    #[allow(dead_code)]
    pub fn down(mut self, i: usize) -> Option<Self> {
        if i >= self.focus.children.len() {
            return None;
        }
        let mut children = self.focus.children;
        let child = children.remove(i);
        let left: Vec<TreeNodeExt> = children.drain(..i.min(children.len())).collect();
        let right: Vec<TreeNodeExt> = children;
        self.ancestors
            .push((self.focus.kind, self.focus.label, left, right));
        self.path = self.path.child(i);
        Some(TreeZipperExt {
            focus: child,
            path: self.path,
            ancestors: self.ancestors,
        })
    }
    /// Move back up to the parent.
    #[allow(dead_code)]
    pub fn up(mut self) -> Option<(Self, TreeNodeExt)> {
        let (kind, label, left, right) = self.ancestors.pop()?;
        let child = self.focus;
        let mut children = left;
        children.push(child.clone());
        children.extend(right);
        let mut new_path = self.path;
        new_path.0.pop();
        Some((
            TreeZipperExt {
                focus: TreeNodeExt {
                    kind,
                    label,
                    children,
                },
                path: new_path,
                ancestors: self.ancestors,
            },
            child,
        ))
    }
}
/// A transform that renames all leaf nodes matching a pattern.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct RenameTransformExt {
    /// Name to rename from
    pub from: String,
    /// Name to rename to
    pub to: String,
    /// Counter of renames performed
    pub count: usize,
}
impl RenameTransformExt {
    /// Create a new rename transform.
    #[allow(dead_code)]
    pub fn new(from: &str, to: &str) -> Self {
        RenameTransformExt {
            from: from.to_string(),
            to: to.to_string(),
            count: 0,
        }
    }
}
/// A simple tree cache that memoises node computations.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct NodeCache {
    /// Cached values by label
    pub values: std::collections::HashMap<String, u64>,
}
impl NodeCache {
    /// Create a new cache.
    #[allow(dead_code)]
    pub fn new() -> Self {
        NodeCache {
            values: std::collections::HashMap::new(),
        }
    }
    /// Store a value.
    #[allow(dead_code)]
    pub fn store(&mut self, key: &str, val: u64) {
        self.values.insert(key.to_string(), val);
    }
    /// Retrieve a value.
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<u64> {
        self.values.get(key).copied()
    }
}
/// A memoized computation table for tree transformations.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct TransformMemoExt2 {
    /// Cache from label to transformed label
    pub cache: std::collections::HashMap<String, String>,
}
impl TransformMemoExt2 {
    /// Create a new empty memo table.
    #[allow(dead_code)]
    pub fn new() -> Self {
        TransformMemoExt2 {
            cache: std::collections::HashMap::new(),
        }
    }
    /// Check if a label is already memoized.
    #[allow(dead_code)]
    pub fn has(&self, label: &str) -> bool {
        self.cache.contains_key(label)
    }
    /// Get the memoized result for a label.
    #[allow(dead_code)]
    pub fn get(&self, label: &str) -> Option<&str> {
        self.cache.get(label).map(|s| s.as_str())
    }
    /// Insert a memoized result.
    #[allow(dead_code)]
    pub fn insert(&mut self, label: &str, result: &str) {
        self.cache.insert(label.to_string(), result.to_string());
    }
    /// Returns the number of memoized entries.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.cache.len()
    }
    /// Returns true if empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }
}
/// Sort kinds.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SortKind {
    /// Type (Type 0)
    Type,
    /// Prop (Sort 0)
    Prop,
    /// Type u (explicit universe)
    TypeU(String),
    /// Sort u (explicit universe)
    SortU(String),
}
