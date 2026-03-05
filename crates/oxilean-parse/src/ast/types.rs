//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;

/// A combined source span holding both line-column positions and byte offsets.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span {
    /// Start position (line/col).
    pub start: Pos,
    /// End position (line/col).
    pub end: Pos,
    /// Byte range.
    pub bytes: ByteRange,
}
impl Span {
    /// Construct a span from positions and byte offsets.
    pub fn new(start: Pos, end: Pos, bytes: ByteRange) -> Self {
        Self { start, end, bytes }
    }
    /// A dummy span at start-of-file.
    pub fn dummy() -> Self {
        Self {
            start: Pos::start(),
            end: Pos::start(),
            bytes: ByteRange::empty(),
        }
    }
    /// Merge two spans into the smallest enclosing span.
    pub fn union(self, other: Span) -> Span {
        Span {
            start: if self.start.is_before(&other.start) {
                self.start
            } else {
                other.start
            },
            end: if other.end.is_before(&self.end) {
                self.end
            } else {
                other.end
            },
            bytes: self.bytes.union(other.bytes),
        }
    }
    /// Whether the byte range is non-empty.
    pub fn is_non_empty(&self) -> bool {
        !self.bytes.is_empty()
    }
}
/// A binder in a telescope (a sequence of bound variables).
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct BinderExtExt2 {
    /// Variable name
    pub name: String,
    /// Type annotation (as string)
    pub ty: Option<String>,
    /// Whether the binder is implicit
    pub implicit: bool,
}
impl BinderExtExt2 {
    /// Create an explicit binder.
    #[allow(dead_code)]
    pub fn explicit(name: &str, ty: Option<&str>) -> Self {
        BinderExtExt2 {
            name: name.to_string(),
            ty: ty.map(|s| s.to_string()),
            implicit: false,
        }
    }
    /// Create an implicit binder.
    #[allow(dead_code)]
    pub fn implicit(name: &str, ty: Option<&str>) -> Self {
        BinderExtExt2 {
            name: name.to_string(),
            ty: ty.map(|s| s.to_string()),
            implicit: true,
        }
    }
    /// Format as "(name : ty)" or "(name)".
    #[allow(dead_code)]
    pub fn format(&self) -> String {
        let brackets = if self.implicit {
            ("{", "}")
        } else {
            ("(", ")")
        };
        match &self.ty {
            Some(t) => format!("{}{} : {}{}", brackets.0, self.name, t, brackets.1),
            None => format!("{}{}{}", brackets.0, self.name, brackets.1),
        }
    }
}
/// A record of a macro invocation and its expansion.
#[derive(Clone, Debug)]
pub struct MacroExpansion {
    /// Name of the macro invoked.
    pub macro_name: String,
    /// Source span of the invocation.
    pub span: ByteRange,
    /// Number of expansion steps taken.
    pub expansion_steps: u32,
}
impl MacroExpansion {
    /// Create a macro expansion record.
    pub fn new(macro_name: impl Into<String>, span: ByteRange) -> Self {
        Self {
            macro_name: macro_name.into(),
            span,
            expansion_steps: 0,
        }
    }
    /// Increment expansion step count.
    pub fn increment(&mut self) {
        self.expansion_steps += 1;
    }
    /// Whether the expansion is considered deep (> 10 steps).
    pub fn is_deep(&self) -> bool {
        self.expansion_steps > 10
    }
}
/// Fixity of an operator notation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fixity {
    /// Left-associative infix operator (e.g., `+`).
    InfixLeft,
    /// Right-associative infix operator (e.g., `^`).
    InfixRight,
    /// Non-associative infix operator.
    Infix,
    /// Prefix operator (e.g., `-`).
    Prefix,
    /// Postfix operator.
    Postfix,
}
/// A line-column source position (1-based).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pos {
    /// 1-based line number.
    pub line: u32,
    /// 1-based column number (in bytes from line start).
    pub col: u32,
}
impl Pos {
    /// Create a position.
    pub fn new(line: u32, col: u32) -> Self {
        Self { line, col }
    }
    /// The start-of-file position (line 1, col 1).
    pub fn start() -> Self {
        Self { line: 1, col: 1 }
    }
    /// Advance to the next column.
    pub fn next_col(self) -> Self {
        Self {
            col: self.col + 1,
            ..self
        }
    }
    /// Advance to the next line (resets column to 1).
    pub fn next_line(self) -> Self {
        Self {
            line: self.line + 1,
            col: 1,
        }
    }
    /// Whether this position comes before `other`.
    pub fn is_before(&self, other: &Pos) -> bool {
        (self.line, self.col) < (other.line, other.col)
    }
}
/// A half-open byte range `[start, end)` in the source text.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ByteRange {
    /// Inclusive start byte offset.
    pub start: usize,
    /// Exclusive end byte offset.
    pub end: usize,
}
impl ByteRange {
    /// Create a byte range.
    pub fn new(start: usize, end: usize) -> Self {
        assert!(start <= end, "ByteRange: start > end");
        Self { start, end }
    }
    /// The empty range at offset 0.
    pub fn empty() -> Self {
        Self { start: 0, end: 0 }
    }
    /// Length of the range in bytes.
    pub fn len(&self) -> usize {
        self.end - self.start
    }
    /// Whether the range is empty.
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
    /// Merge two ranges into the smallest containing range.
    pub fn union(self, other: ByteRange) -> ByteRange {
        ByteRange {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }
    /// Whether this range contains a byte offset.
    pub fn contains(&self, offset: usize) -> bool {
        offset >= self.start && offset < self.end
    }
    /// Extract the covered slice from a source string.
    pub fn slice<'a>(&self, src: &'a str) -> &'a str {
        &src[self.start..self.end]
    }
}
/// A simple expression complexity metric.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ComplexityMetric {
    /// Weight for each application
    pub app_weight: u32,
    /// Weight for each lambda
    pub lam_weight: u32,
    /// Weight for each leaf
    pub leaf_weight: u32,
}
impl ComplexityMetric {
    /// Create a default metric.
    #[allow(dead_code)]
    pub fn default_metric() -> Self {
        ComplexityMetric {
            app_weight: 2,
            lam_weight: 3,
            leaf_weight: 1,
        }
    }
    /// Compute the complexity of a tree node.
    #[allow(dead_code)]
    pub fn compute(&self, node: &TreeNodeExt) -> u32 {
        match node.kind {
            SimpleNodeKindExt::Leaf => self.leaf_weight,
            SimpleNodeKindExt::App => {
                self.app_weight + node.children.iter().map(|c| self.compute(c)).sum::<u32>()
            }
            SimpleNodeKindExt::Lam => {
                self.lam_weight + node.children.iter().map(|c| self.compute(c)).sum::<u32>()
            }
            _ => 1 + node.children.iter().map(|c| self.compute(c)).sum::<u32>(),
        }
    }
}
/// Metadata attached to an AST node during parsing or elaboration.
#[derive(Clone, Debug, Default)]
pub struct AstMetadata {
    /// Source span of the node.
    pub span: Option<ByteRange>,
    /// Elaboration-time type annotation (stringified).
    pub type_hint: Option<String>,
    /// Whether this node was synthesized (not from real source).
    pub synthetic: bool,
    /// Arbitrary string tags for tooling.
    pub tags: Vec<String>,
}
impl AstMetadata {
    /// Create empty metadata.
    pub fn empty() -> Self {
        Self::default()
    }
    /// Create metadata with a span.
    pub fn with_span(span: ByteRange) -> Self {
        Self {
            span: Some(span),
            ..Default::default()
        }
    }
    /// Mark as synthetic.
    pub fn synthetic() -> Self {
        Self {
            synthetic: true,
            ..Default::default()
        }
    }
    /// Add a tag.
    pub fn add_tag(&mut self, tag: impl Into<String>) {
        self.tags.push(tag.into());
    }
    /// Whether a tag is present.
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}
/// An import declaration (`import Foo.Bar`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImportDecl {
    /// Dotted module path (e.g., `["Foo", "Bar"]`).
    pub path: Vec<String>,
    /// Source span.
    pub span: ByteRange,
}
impl ImportDecl {
    /// Create an import declaration.
    pub fn new(path: Vec<String>, span: ByteRange) -> Self {
        Self { path, span }
    }
    /// The module path as a dotted string (e.g., `"Foo.Bar"`).
    pub fn dotted_path(&self) -> String {
        self.path.join(".")
    }
    /// Whether the import is for a root module (single component).
    pub fn is_root(&self) -> bool {
        self.path.len() == 1
    }
}
/// A registry of notation entries indexed by symbol.
#[derive(Clone, Debug, Default)]
pub struct OperatorTable {
    /// All registered entries.
    entries: Vec<NotationEntry>,
}
impl OperatorTable {
    /// Create an empty table.
    pub fn new() -> Self {
        Self::default()
    }
    /// Register a notation entry.
    pub fn register(&mut self, entry: NotationEntry) {
        self.entries.push(entry);
    }
    /// Look up entries for a given symbol.
    pub fn lookup(&self, symbol: &str) -> Vec<&NotationEntry> {
        self.entries.iter().filter(|e| e.symbol == symbol).collect()
    }
    /// Number of registered entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Whether the table is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Infix entries.
    pub fn infix_entries(&self) -> Vec<&NotationEntry> {
        self.entries.iter().filter(|e| e.is_infix()).collect()
    }
}
/// A binder for a pi type or lambda.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct BinderExt {
    /// Variable name
    pub name: String,
    /// Type annotation (optional)
    pub ty: Option<String>,
    /// Whether this is implicit
    pub implicit: bool,
}
impl BinderExt {
    /// Create an explicit binder.
    #[allow(dead_code)]
    pub fn explicit(name: &str, ty: Option<&str>) -> Self {
        BinderExt {
            name: name.to_string(),
            ty: ty.map(|s| s.to_string()),
            implicit: false,
        }
    }
    /// Create an implicit binder.
    #[allow(dead_code)]
    pub fn implicit(name: &str, ty: Option<&str>) -> Self {
        BinderExt {
            name: name.to_string(),
            ty: ty.map(|s| s.to_string()),
            implicit: true,
        }
    }
    /// Format the binder.
    #[allow(dead_code)]
    pub fn format(&self) -> String {
        let inner = if let Some(ref ty) = self.ty {
            format!("{} : {}", self.name, ty)
        } else {
            self.name.clone()
        };
        if self.implicit {
            format!("{{{}}}", inner)
        } else {
            format!("({})", inner)
        }
    }
}
/// A let binding in an expression.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct LetBindingExtExt2 {
    /// Name of the bound variable
    pub name: String,
    /// Optional type annotation
    pub ty: Option<String>,
    /// The value being bound
    pub value: String,
    /// The body of the let expression
    pub body: String,
}
impl LetBindingExtExt2 {
    /// Create a new let binding.
    #[allow(dead_code)]
    pub fn new(name: &str, ty: Option<&str>, value: &str, body: &str) -> Self {
        LetBindingExtExt2 {
            name: name.to_string(),
            ty: ty.map(|s| s.to_string()),
            value: value.to_string(),
            body: body.to_string(),
        }
    }
    /// Format as "let name := value; body".
    #[allow(dead_code)]
    pub fn format(&self) -> String {
        match &self.ty {
            Some(t) => {
                format!("let {} : {} := {}; {}", self.name, t, self.value, self.body)
            }
            None => format!("let {} := {}; {}", self.name, self.value, self.body),
        }
    }
}
/// A type annotation node for the AST.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct TypeAnnotation {
    /// The expression being annotated
    pub expr: String,
    /// The type annotation
    pub ty: String,
}
impl TypeAnnotation {
    /// Create a new type annotation.
    #[allow(dead_code)]
    pub fn new(expr: &str, ty: &str) -> Self {
        TypeAnnotation {
            expr: expr.to_string(),
            ty: ty.to_string(),
        }
    }
    /// Format as "(expr : ty)".
    #[allow(dead_code)]
    pub fn format(&self) -> String {
        format!("({} : {})", self.expr, self.ty)
    }
}
/// A universe level expression (Lean 4 style).
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UniverseLevel {
    /// Universe 0 (Prop)
    Zero,
    /// Successor of a universe
    Succ(Box<UniverseLevel>),
    /// Maximum of two universes
    Max(Box<UniverseLevel>, Box<UniverseLevel>),
    /// A universe variable
    Var(String),
}
impl UniverseLevel {
    /// Returns the concrete level as a number, if possible.
    #[allow(dead_code)]
    pub fn concrete(&self) -> Option<usize> {
        match self {
            UniverseLevel::Zero => Some(0),
            UniverseLevel::Succ(inner) => inner.concrete().map(|n| n + 1),
            UniverseLevel::Max(a, b) => {
                let ca = a.concrete()?;
                let cb = b.concrete()?;
                Some(ca.max(cb))
            }
            UniverseLevel::Var(_) => None,
        }
    }
}
/// An argument to an attribute annotation.
#[derive(Clone, Debug, PartialEq)]
pub enum AttrArg {
    /// Identifier argument.
    Ident(String),
    /// Numeric argument.
    Num(i64),
    /// String argument.
    Str(String),
    /// List of arguments.
    List(Vec<AttrArg>),
}
impl AttrArg {
    /// Create an identifier attribute argument.
    pub fn ident(s: &str) -> Self {
        AttrArg::Ident(s.to_string())
    }
    /// Create a numeric attribute argument.
    pub fn num(n: i64) -> Self {
        AttrArg::Num(n)
    }
    /// Create a string attribute argument.
    pub fn str_arg(s: &str) -> Self {
        AttrArg::Str(s.to_string())
    }
    /// As an ident string, if this is an Ident.
    pub fn as_ident(&self) -> Option<&str> {
        match self {
            AttrArg::Ident(s) => Some(s),
            _ => None,
        }
    }
    /// As a number, if this is a Num.
    pub fn as_num(&self) -> Option<i64> {
        match self {
            AttrArg::Num(n) => Some(*n),
            _ => None,
        }
    }
}
/// A registered notation (operator or mixfix).
#[derive(Clone, Debug)]
pub struct NotationEntry {
    /// The notation symbol string (e.g., `+`).
    pub symbol: String,
    /// Associated declaration name.
    pub decl_name: String,
    /// Fixity and associativity.
    pub fixity: Fixity,
    /// Binding precedence.
    pub prec: Prec,
    /// Whether the notation is deprecated.
    pub deprecated: bool,
}
impl NotationEntry {
    /// Create a new notation entry.
    pub fn new(symbol: &str, decl_name: &str, fixity: Fixity, prec: Prec) -> Self {
        Self {
            symbol: symbol.to_string(),
            decl_name: decl_name.to_string(),
            fixity,
            prec,
            deprecated: false,
        }
    }
    /// Mark this notation as deprecated.
    pub fn deprecated(mut self) -> Self {
        self.deprecated = true;
        self
    }
    /// Whether this is an infix notation.
    pub fn is_infix(&self) -> bool {
        matches!(
            self.fixity,
            Fixity::InfixLeft | Fixity::InfixRight | Fixity::Infix
        )
    }
}
/// A simple AST node kind for generic traversal.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SimpleNodeKindExt {
    /// A leaf node (variable, literal)
    Leaf,
    /// An application node
    App,
    /// A lambda abstraction
    Lam,
    /// A pi-type / forall
    Pi,
    /// A let binding
    Let,
    /// A sort (Type/Prop)
    Sort,
    /// A match expression
    Match,
    /// A definition head
    Def,
    /// A theorem head
    Theorem,
    /// An annotation
    Ann,
    /// A projection
    Proj,
    /// A universe level
    Level,
}
/// A let-binding surface expression.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct LetBindingExt {
    /// Variable name
    pub name: String,
    /// Optional type annotation
    pub ty: Option<String>,
    /// The value expression (as string)
    pub value: String,
    /// The body expression (as string)
    pub body: String,
}
impl LetBindingExt {
    /// Create a new let binding.
    #[allow(dead_code)]
    pub fn new(name: &str, value: &str, body: &str) -> Self {
        LetBindingExt {
            name: name.to_string(),
            ty: None,
            value: value.to_string(),
            body: body.to_string(),
        }
    }
    /// Add a type annotation.
    #[allow(dead_code)]
    pub fn with_ty(mut self, ty: &str) -> Self {
        self.ty = Some(ty.to_string());
        self
    }
    /// Format as "let name : ty := value; body" or "let name := value; body".
    #[allow(dead_code)]
    pub fn format(&self) -> String {
        if let Some(ref ty) = self.ty {
            format!(
                "let {} : {} := {} in {}",
                self.name, ty, self.value, self.body
            )
        } else {
            format!("let {} := {} in {}", self.name, self.value, self.body)
        }
    }
}
/// A raw identifier as it appears in source, together with its span.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SurfaceIdent {
    /// The identifier string.
    pub name: String,
    /// Source span (byte offsets).
    pub span: ByteRange,
}
impl SurfaceIdent {
    /// Create a surface identifier.
    pub fn new(name: &str, span: ByteRange) -> Self {
        Self {
            name: name.to_string(),
            span,
        }
    }
    /// Create a synthetic identifier with no real source span.
    pub fn synthetic(name: &str) -> Self {
        Self {
            name: name.to_string(),
            span: ByteRange::empty(),
        }
    }
    /// Whether the identifier was synthesized (not from real source).
    pub fn is_synthetic(&self) -> bool {
        self.span.is_empty()
    }
    /// Whether the identifier starts with an underscore.
    pub fn is_anonymous(&self) -> bool {
        self.name.starts_with('_')
    }
    /// Whether the identifier contains a dot (qualified name).
    pub fn is_qualified(&self) -> bool {
        self.name.contains('.')
    }
    /// Split a qualified name at the last dot, if any.
    pub fn split_last(&self) -> Option<(&str, &str)> {
        let pos = self.name.rfind('.')?;
        Some((&self.name[..pos], &self.name[pos + 1..]))
    }
}
/// A telescope of binders.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct Telescope {
    /// The binders in order
    pub binders: Vec<BinderExt>,
}
impl Telescope {
    /// Create an empty telescope.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Telescope {
            binders: Vec::new(),
        }
    }
    /// Add a binder.
    #[allow(dead_code)]
    #[allow(clippy::should_implement_trait)]
    pub fn add(mut self, b: BinderExt) -> Self {
        self.binders.push(b);
        self
    }
    /// Format the telescope.
    #[allow(dead_code)]
    pub fn format(&self) -> String {
        self.binders
            .iter()
            .map(|b| b.format())
            .collect::<Vec<_>>()
            .join(" ")
    }
    /// Returns the number of binders.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.binders.len()
    }
    /// Returns true if empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.binders.is_empty()
    }
}
/// A declaration header.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct DeclHeaderExt {
    /// The declaration name
    pub name: String,
    /// The telescope of binders
    pub params: Vec<BinderExt>,
    /// The return type
    pub return_type: Option<String>,
}
impl DeclHeaderExt {
    /// Create a new declaration header.
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        DeclHeaderExt {
            name: name.to_string(),
            params: Vec::new(),
            return_type: None,
        }
    }
    /// Add a parameter.
    #[allow(dead_code)]
    pub fn add_param(mut self, b: BinderExt) -> Self {
        self.params.push(b);
        self
    }
    /// Set the return type.
    #[allow(dead_code)]
    pub fn with_return_type(mut self, ty: &str) -> Self {
        self.return_type = Some(ty.to_string());
        self
    }
}
/// A match expression surface form.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct MatchExprExt {
    /// The scrutinee
    pub scrutinee: String,
    /// The arms
    pub arms: Vec<(String, String)>,
}
impl MatchExprExt {
    /// Create a new match expression.
    #[allow(dead_code)]
    pub fn new(scrutinee: &str) -> Self {
        MatchExprExt {
            scrutinee: scrutinee.to_string(),
            arms: Vec::new(),
        }
    }
    /// Add an arm.
    #[allow(dead_code)]
    pub fn add_arm(mut self, pat: &str, body: &str) -> Self {
        self.arms.push((pat.to_string(), body.to_string()));
        self
    }
    /// Format the match expression.
    #[allow(dead_code)]
    pub fn format(&self) -> String {
        let arms: Vec<String> = self
            .arms
            .iter()
            .map(|(p, b)| format!("  | {} -> {}", p, b))
            .collect();
        format!("match {} with\n{}", self.scrutinee, arms.join("\n"))
    }
}
/// A position-annotated wrapper for any value.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct WithPosExt<T> {
    /// The wrapped value
    pub inner: T,
    /// Byte start offset
    pub start: usize,
    /// Byte end offset
    pub end: usize,
}
impl<T> WithPosExt<T> {
    /// Create a new WithPosExt.
    #[allow(dead_code)]
    pub fn new(inner: T, start: usize, end: usize) -> Self {
        WithPosExt { inner, start, end }
    }
    /// Map the inner value.
    #[allow(dead_code)]
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> WithPosExt<U> {
        WithPosExt {
            inner: f(self.inner),
            start: self.start,
            end: self.end,
        }
    }
}
/// A parse error with location information.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseError {
    /// Human-readable error message.
    pub message: String,
    /// Location in the source where the error occurred.
    pub pos: Pos,
    /// Optional hint for how to fix the error.
    pub hint: Option<String>,
}
impl ParseError {
    /// Create a parse error.
    pub fn new(message: impl Into<String>, pos: Pos) -> Self {
        Self {
            message: message.into(),
            pos,
            hint: None,
        }
    }
    /// Attach a hint to the error.
    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }
    /// Format the error for display.
    pub fn display(&self) -> String {
        let mut s = format!("error at {}: {}", self.pos, self.message);
        if let Some(h) = &self.hint {
            s.push_str(&format!("\n  hint: {}", h));
        }
        s
    }
}
/// A flat representation of an AST (pre-order traversal).
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct FlatAstExt {
    /// Nodes in pre-order
    pub nodes: Vec<(SimpleNodeKindExt, String, usize)>,
}
impl FlatAstExt {
    /// Flatten a tree node into pre-order sequence.
    #[allow(dead_code)]
    pub fn from_tree(root: &TreeNodeExt) -> Self {
        let mut nodes = Vec::new();
        flatten_tree_ext(root, 0, &mut nodes);
        FlatAstExt { nodes }
    }
    /// Returns the number of nodes.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.nodes.len()
    }
    /// Returns true if empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}
/// A telescope: a sequence of binders.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Default)]
pub struct TelescopeExt2 {
    /// The binders in order
    pub binders: Vec<BinderExtExt2>,
}
impl TelescopeExt2 {
    /// Create an empty telescope.
    #[allow(dead_code)]
    pub fn new() -> Self {
        TelescopeExt2 {
            binders: Vec::new(),
        }
    }
    /// Add a binder.
    #[allow(dead_code)]
    pub fn push(&mut self, b: BinderExtExt2) {
        self.binders.push(b);
    }
    /// Returns the number of binders.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.binders.len()
    }
    /// Returns true if empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.binders.is_empty()
    }
    /// Format as a space-separated list of binder strings.
    #[allow(dead_code)]
    pub fn format(&self) -> String {
        self.binders
            .iter()
            .map(|b| b.format())
            .collect::<Vec<_>>()
            .join(" ")
    }
}
/// A match expression in the AST.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct MatchExprExtExt2 {
    /// The discriminant expression
    pub scrutinee: String,
    /// The match arms as (pattern, body) pairs
    pub arms: Vec<(String, String)>,
}
impl MatchExprExtExt2 {
    /// Create a new match expression.
    #[allow(dead_code)]
    pub fn new(scrutinee: &str, arms: Vec<(&str, &str)>) -> Self {
        MatchExprExtExt2 {
            scrutinee: scrutinee.to_string(),
            arms: arms
                .into_iter()
                .map(|(p, b)| (p.to_string(), b.to_string()))
                .collect(),
        }
    }
    /// Returns the number of arms.
    #[allow(dead_code)]
    pub fn arm_count(&self) -> usize {
        self.arms.len()
    }
    /// Returns the patterns as a vector.
    #[allow(dead_code)]
    pub fn patterns(&self) -> Vec<&str> {
        self.arms.iter().map(|(p, _)| p.as_str()).collect()
    }
}
/// A generic tree node for testing AST operations.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct TreeNodeExt {
    /// Kind of this node
    pub kind: SimpleNodeKindExt,
    /// Label or identifier
    pub label: String,
    /// Children of this node
    pub children: Vec<TreeNodeExt>,
}
impl TreeNodeExt {
    /// Create a leaf node.
    #[allow(dead_code)]
    pub fn leaf(label: &str) -> Self {
        TreeNodeExt {
            kind: SimpleNodeKindExt::Leaf,
            label: label.to_string(),
            children: Vec::new(),
        }
    }
    /// Create an app node with two children.
    #[allow(dead_code)]
    pub fn app(func: TreeNodeExt, arg: TreeNodeExt) -> Self {
        TreeNodeExt {
            kind: SimpleNodeKindExt::App,
            label: "@".to_string(),
            children: vec![func, arg],
        }
    }
    /// Create a lam node.
    #[allow(dead_code)]
    pub fn lam(binder: &str, body: TreeNodeExt) -> Self {
        TreeNodeExt {
            kind: SimpleNodeKindExt::Lam,
            label: binder.to_string(),
            children: vec![body],
        }
    }
    /// Returns the depth of this tree.
    #[allow(dead_code)]
    pub fn depth(&self) -> usize {
        if self.children.is_empty() {
            return 0;
        }
        1 + self.children.iter().map(|c| c.depth()).max().unwrap_or(0)
    }
    /// Returns the total number of nodes.
    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        1 + self.children.iter().map(|c| c.size()).sum::<usize>()
    }
    /// Visit all nodes with a visitor.
    #[allow(dead_code)]
    pub fn visit<V: AstNodeVisitorExt>(&self, visitor: &mut V, depth: usize) {
        let kind_str = format!("{:?}", self.kind);
        visitor.visit_node(&kind_str, depth);
        for child in &self.children {
            child.visit(visitor, depth + 1);
        }
    }
}
/// A type synonym definition.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct TypeSynonym {
    /// Name of the synonym
    pub name: String,
    /// Type parameters
    pub params: Vec<String>,
    /// Definition
    pub def: String,
}
impl TypeSynonym {
    /// Create a new type synonym.
    #[allow(dead_code)]
    pub fn new(name: &str, params: Vec<&str>, def: &str) -> Self {
        TypeSynonym {
            name: name.to_string(),
            params: params.into_iter().map(|s| s.to_string()).collect(),
            def: def.to_string(),
        }
    }
    /// Format as "abbrev Name params := def".
    #[allow(dead_code)]
    pub fn format(&self) -> String {
        let params = self.params.join(" ");
        if params.is_empty() {
            format!("abbrev {} := {}", self.name, self.def)
        } else {
            format!("abbrev {} {} := {}", self.name, params, self.def)
        }
    }
}
/// A tag identifying the kind of an AST node.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AstNodeKind {
    /// A variable or constant name.
    Var,
    /// A lambda expression.
    Lam,
    /// A Pi / forall type.
    Pi,
    /// An application.
    App,
    /// A let-expression.
    Let,
    /// A natural-number literal.
    NatLit,
    /// A string literal.
    StrLit,
    /// A sort (Prop/Type).
    Sort,
    /// A hole `_`.
    Hole,
    /// A `def` declaration.
    DefDecl,
    /// A `theorem` declaration.
    TheoremDecl,
    /// An `axiom` declaration.
    AxiomDecl,
}
/// A simple tag identifying broad categories of tokens.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TokenKindTag {
    /// An identifier or keyword.
    Ident,
    /// A numeric literal.
    Num,
    /// A string literal.
    Str,
    /// An operator symbol.
    Op,
    /// A delimiter (paren, bracket, brace).
    Delim,
    /// End of file.
    Eof,
}
/// Numeric precedence for an operator.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Prec(pub u32);
impl Prec {
    /// Default term precedence (Lean 4 uses 1024 for atoms).
    pub const ATOM: Prec = Prec(1024);
    /// Precedence for function application.
    pub const APP: Prec = Prec(1000);
    /// Precedence for `*` / `/`.
    pub const MUL: Prec = Prec(70);
    /// Precedence for `+` / `-`.
    pub const ADD: Prec = Prec(65);
    /// Precedence for comparison operators.
    pub const CMP: Prec = Prec(50);
    /// Minimum precedence (weakest binding).
    pub const MIN: Prec = Prec(0);
    /// Create a precedence value.
    pub fn new(p: u32) -> Self {
        Prec(p)
    }
    /// Raw numeric value.
    pub fn value(self) -> u32 {
        self.0
    }
    /// One level tighter binding.
    pub fn tighter(self) -> Self {
        Prec(self.0 + 1)
    }
}
/// An expression annotation kind.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnnotationKind {
    /// Type annotation
    Type,
    /// Docstring
    Doc,
    /// Derivation hint
    Derive,
    /// Attribute
    Attr,
}
/// A counting visitor that counts each kind.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct CountingVisitorExt {
    /// Map from node kind to count
    pub counts: std::collections::HashMap<String, usize>,
    /// Current depth
    pub depth: usize,
}
impl CountingVisitorExt {
    /// Create a new counting visitor.
    #[allow(dead_code)]
    pub fn new() -> Self {
        CountingVisitorExt {
            counts: std::collections::HashMap::new(),
            depth: 0,
        }
    }
}
/// A stack of open namespaces, used during parsing.
#[derive(Clone, Debug, Default)]
pub struct NamespaceStack {
    stack: Vec<String>,
}
impl NamespaceStack {
    /// Create an empty stack.
    pub fn new() -> Self {
        Self::default()
    }
    /// Push a namespace.
    pub fn push(&mut self, ns: impl Into<String>) {
        self.stack.push(ns.into());
    }
    /// Pop the top namespace. Returns the popped name, or `None` if empty.
    pub fn pop(&mut self) -> Option<String> {
        self.stack.pop()
    }
    /// The current fully-qualified namespace path (dot-separated).
    pub fn current_path(&self) -> String {
        self.stack.join(".")
    }
    /// Qualify a name with the current namespace.
    pub fn qualify(&self, name: &str) -> String {
        if self.stack.is_empty() {
            name.to_string()
        } else {
            format!("{}.{}", self.current_path(), name)
        }
    }
    /// Depth of the stack.
    pub fn depth(&self) -> usize {
        self.stack.len()
    }
    /// Whether the stack is empty (top-level).
    pub fn is_top_level(&self) -> bool {
        self.stack.is_empty()
    }
}
/// A structure field.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct StructField {
    /// Field name
    pub name: String,
    /// Field type
    pub ty: String,
    /// Optional default value
    pub default: Option<String>,
}
impl StructField {
    /// Create a new field.
    #[allow(dead_code)]
    pub fn new(name: &str, ty: &str) -> Self {
        StructField {
            name: name.to_string(),
            ty: ty.to_string(),
            default: None,
        }
    }
    /// Set the default value.
    #[allow(dead_code)]
    pub fn with_default(mut self, v: &str) -> Self {
        self.default = Some(v.to_string());
        self
    }
}
/// A documentation comment attached to a declaration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DocComment {
    /// The raw comment text (without the `/--` delimiters).
    pub text: String,
    /// Whether this is a module-level doc comment.
    pub is_module_doc: bool,
    /// Byte range of the comment in the source.
    pub span: ByteRange,
}
impl DocComment {
    /// Create a doc comment.
    pub fn new(text: &str, span: ByteRange) -> Self {
        Self {
            text: text.to_string(),
            is_module_doc: false,
            span,
        }
    }
    /// Create a module-level doc comment.
    pub fn module_doc(text: &str) -> Self {
        Self {
            text: text.to_string(),
            is_module_doc: true,
            span: ByteRange::empty(),
        }
    }
    /// The first line of the doc comment (for summaries).
    pub fn first_line(&self) -> &str {
        self.text.lines().next().unwrap_or("").trim()
    }
    /// Whether the doc comment is empty.
    pub fn is_empty(&self) -> bool {
        self.text.trim().is_empty()
    }
}
/// A scope-management declaration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ScopeDecl {
    /// `section Name`
    Section(String),
    /// `namespace Name`
    Namespace(String),
    /// `end Name`
    End(String),
    /// `open Name` (brings names into scope)
    Open(Vec<String>),
}
impl ScopeDecl {
    /// The name argument (for Section/Namespace/End).
    pub fn name(&self) -> Option<&str> {
        match self {
            ScopeDecl::Section(n) | ScopeDecl::Namespace(n) | ScopeDecl::End(n) => Some(n),
            ScopeDecl::Open(_) => None,
        }
    }
    /// Whether this opens new scope.
    pub fn opens_scope(&self) -> bool {
        matches!(self, ScopeDecl::Section(_) | ScopeDecl::Namespace(_))
    }
    /// Whether this closes scope.
    pub fn closes_scope(&self) -> bool {
        matches!(self, ScopeDecl::End(_))
    }
}
/// A substitution table for tree nodes.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct SubstTableExt {
    /// Mappings from variable name to replacement node
    pub mappings: Vec<(String, TreeNodeExt)>,
}
impl SubstTableExt {
    /// Create a new empty substitution table.
    #[allow(dead_code)]
    pub fn new() -> Self {
        SubstTableExt {
            mappings: Vec::new(),
        }
    }
    /// Add a mapping.
    #[allow(dead_code)]
    pub fn add(&mut self, var: &str, replacement: TreeNodeExt) {
        self.mappings.push((var.to_string(), replacement));
    }
    /// Apply the substitution to a tree node.
    #[allow(dead_code)]
    pub fn apply(&self, node: &TreeNodeExt) -> TreeNodeExt {
        match node.kind {
            SimpleNodeKindExt::Leaf => {
                if let Some((_, replacement)) = self.mappings.iter().find(|(v, _)| v == &node.label)
                {
                    replacement.clone()
                } else {
                    node.clone()
                }
            }
            _ => {
                let new_children: Vec<TreeNodeExt> =
                    node.children.iter().map(|c| self.apply(c)).collect();
                TreeNodeExt {
                    kind: node.kind.clone(),
                    label: node.label.clone(),
                    children: new_children,
                }
            }
        }
    }
}
/// Visibility of a declaration.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Visibility {
    /// Visible everywhere (the default).
    #[default]
    Public,
    /// Visible only within the current file/module.
    Private,
    /// Visible within the current namespace and sub-namespaces.
    Protected,
}
impl Visibility {
    /// Whether this visibility allows access from outside the module.
    pub fn is_public(&self) -> bool {
        matches!(self, Visibility::Public)
    }
    /// Whether this visibility restricts access.
    pub fn is_restricted(&self) -> bool {
        !self.is_public()
    }
}
