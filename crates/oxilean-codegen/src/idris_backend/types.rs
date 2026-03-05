//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

/// A `data` declaration in Idris 2.
#[derive(Debug, Clone, PartialEq)]
pub struct IdrisData {
    /// Type name.
    pub name: String,
    /// Type parameters: `(a : Type)`, `(n : Nat)`.
    pub params: Vec<(String, IdrisType)>,
    /// The kind of this type (usually `Type`).
    pub kind: IdrisType,
    /// Constructors.
    pub constructors: Vec<IdrisConstructor>,
    /// Visibility.
    pub visibility: Visibility,
    /// Optional doc comment.
    pub doc: Option<String>,
}
impl IdrisData {
    /// Emit this data declaration.
    pub fn emit(&self, indent: usize) -> String {
        let pad = " ".repeat(indent);
        let mut out = String::new();
        if let Some(doc) = &self.doc {
            for line in doc.lines() {
                out.push_str(&format!("{}||| {}\n", pad, line));
            }
        }
        let params_str: String = self
            .params
            .iter()
            .map(|(n, t)| format!(" ({} : {})", n, t))
            .collect();
        out.push_str(&format!(
            "{}{}data {}{} : {} where\n",
            pad, self.visibility, self.name, params_str, self.kind
        ));
        for ctor in &self.constructors {
            if let Some(doc) = &ctor.doc {
                out.push_str(&format!("{}  ||| {}\n", pad, doc));
            }
            out.push_str(&format!("{}  {} : {}\n", pad, ctor.name, ctor.ty));
        }
        out
    }
}
/// A `record` declaration in Idris 2.
#[derive(Debug, Clone, PartialEq)]
pub struct IdrisRecord {
    /// Record name.
    pub name: String,
    /// Type parameters.
    pub params: Vec<(String, IdrisType)>,
    /// The kind (usually `Type`).
    pub kind: IdrisType,
    /// Constructor name (e.g. `MkPoint`).
    pub constructor: String,
    /// Fields: `(name, type)`.
    pub fields: Vec<(String, IdrisType)>,
    /// Visibility.
    pub visibility: Visibility,
    /// Optional doc comment.
    pub doc: Option<String>,
}
impl IdrisRecord {
    /// Emit this record declaration.
    pub fn emit(&self, indent: usize) -> String {
        let pad = " ".repeat(indent);
        let mut out = String::new();
        if let Some(doc) = &self.doc {
            for line in doc.lines() {
                out.push_str(&format!("{}||| {}\n", pad, line));
            }
        }
        let params_str: String = self
            .params
            .iter()
            .map(|(n, t)| format!(" ({} : {})", n, t))
            .collect();
        out.push_str(&format!(
            "{}{}record {}{} : {} where\n",
            pad, self.visibility, self.name, params_str, self.kind
        ));
        out.push_str(&format!("{}  constructor {}\n", pad, self.constructor));
        for (fname, ftype) in &self.fields {
            out.push_str(&format!("{}  {} : {}\n", pad, fname, ftype));
        }
        out
    }
}
/// An Idris 2 `namespace` block.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct IdrisNamespaceBlock {
    /// Namespace name.
    pub name: String,
    /// Declarations in this namespace.
    pub decls: Vec<IdrisDecl>,
}
impl IdrisNamespaceBlock {
    /// Create a new namespace block.
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        IdrisNamespaceBlock {
            name: name.into(),
            decls: Vec::new(),
        }
    }
    /// Add a declaration.
    #[allow(dead_code)]
    pub fn add(&mut self, decl: IdrisDecl) {
        self.decls.push(decl);
    }
    /// Emit the namespace block.
    #[allow(dead_code)]
    pub fn emit(&self, backend: &IdrisBackend) -> String {
        let mut out = format!("namespace {}\n", self.name);
        for decl in &self.decls {
            for line in backend.emit_decl(decl).lines() {
                out.push_str("    ");
                out.push_str(line);
                out.push('\n');
            }
        }
        out
    }
}
/// A tactic in an Idris 2 proof script.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum IdrisTactic {
    /// `intro x` — introduce a variable.
    Intro(String),
    /// `intros` — introduce all variables.
    Intros,
    /// `exact e` — close goal with exact term.
    Exact(IdrisExpr),
    /// `refl` — close reflexivity goal.
    Refl,
    /// `trivial` — close trivial goal.
    Trivial,
    /// `decide` — close decidable goal.
    Decide,
    /// `rewrite h` — rewrite using equality.
    Rewrite(String),
    /// `rewrite <- h` — rewrite backwards.
    RewriteBack(String),
    /// `apply f` — apply a function.
    Apply(String),
    /// `cases x` — case split on x.
    Cases(String),
    /// `induction x` — do induction on x.
    Induction(String),
    /// `search` — auto-search for a term.
    Search,
    /// `auto` — auto-search.
    Auto,
    /// `with e` — with view pattern.
    With(String),
    /// `let x = e` — introduce local definition.
    Let(String, IdrisExpr),
    /// `have x : T by tac` — prove an intermediate.
    Have(String, IdrisType),
    /// `focus n` — focus on sub-goal n.
    Focus(usize),
    /// `claim n t` — claim sub-goal.
    Claim(String, IdrisType),
    /// `unfold n` — unfold a definition.
    Unfold(String),
    /// `compute` — reduce to normal form.
    Compute,
    /// `normals` — normalize.
    Normals,
    /// `fail msg` — fail with message.
    Fail(String),
    /// Sequence of tactics.
    Seq(Vec<IdrisTactic>),
}
/// A top-level Idris 2 function definition.
#[derive(Debug, Clone, PartialEq)]
pub struct IdrisFunction {
    /// Function name.
    pub name: String,
    /// Type signature (the full type).
    pub type_sig: IdrisType,
    /// Function clauses: `(patterns, rhs)`.
    pub clauses: Vec<(Vec<IdrisPattern>, IdrisExpr)>,
    /// Totality annotation.
    pub totality: Totality,
    /// Visibility.
    pub visibility: Visibility,
    /// Optional `%inline` or other pragmas.
    pub pragmas: Vec<String>,
    /// Optional doc comment.
    pub doc: Option<String>,
}
impl IdrisFunction {
    /// Create a new function with a single clause.
    pub fn new(name: impl Into<String>, ty: IdrisType, body: IdrisExpr) -> Self {
        IdrisFunction {
            name: name.into(),
            type_sig: ty,
            clauses: vec![(vec![], body)],
            totality: Totality::Default,
            visibility: Visibility::Default,
            pragmas: vec![],
            doc: None,
        }
    }
    /// Create a function with multiple clauses.
    pub fn with_clauses(
        name: impl Into<String>,
        ty: IdrisType,
        clauses: Vec<(Vec<IdrisPattern>, IdrisExpr)>,
    ) -> Self {
        IdrisFunction {
            name: name.into(),
            type_sig: ty,
            clauses,
            totality: Totality::Default,
            visibility: Visibility::Default,
            pragmas: vec![],
            doc: None,
        }
    }
    /// Emit this function definition.
    pub fn emit(&self, indent: usize) -> String {
        let pad = " ".repeat(indent);
        let mut out = String::new();
        if let Some(doc) = &self.doc {
            for line in doc.lines() {
                out.push_str(&format!("{}||| {}\n", pad, line));
            }
        }
        for pragma in &self.pragmas {
            out.push_str(&format!("{}%{}\n", pad, pragma));
        }
        let tot = format!("{}", self.totality);
        if !tot.is_empty() {
            out.push_str(&format!("{}{}", pad, tot));
        }
        out.push_str(&format!(
            "{}{}{} : {}\n",
            pad, self.visibility, self.name, self.type_sig
        ));
        for (pats, rhs) in &self.clauses {
            if pats.is_empty() {
                out.push_str(&format!("{}{} = {}\n", pad, self.name, rhs));
            } else {
                let pat_str = pats
                    .iter()
                    .map(|p| format!("{}", p))
                    .collect::<Vec<_>>()
                    .join(" ");
                out.push_str(&format!("{}{} {} = {}\n", pad, self.name, pat_str, rhs));
            }
        }
        out
    }
}
/// An import directive in an Idris 2 module.
#[derive(Debug, Clone, PartialEq)]
pub struct IdrisImport {
    /// Module path, e.g. `["Data", "List"]`.
    pub path: Vec<String>,
    /// Optional `as` alias.
    pub alias: Option<String>,
    /// Whether this is a `public import`.
    pub public: bool,
}
impl IdrisImport {
    /// Create a simple import.
    pub fn new(path: Vec<String>) -> Self {
        IdrisImport {
            path,
            alias: None,
            public: false,
        }
    }
    /// Create a public import.
    pub fn public_import(path: Vec<String>) -> Self {
        IdrisImport {
            path,
            alias: None,
            public: true,
        }
    }
    /// Emit this import directive.
    pub fn emit(&self) -> String {
        let path_str = self.path.join(".");
        let prefix = if self.public {
            "public import "
        } else {
            "import "
        };
        if let Some(alias) = &self.alias {
            format!("{}{} as {}", prefix, path_str, alias)
        } else {
            format!("{}{}", prefix, path_str)
        }
    }
}
/// Fluent helper for building Idris 2 patterns.
#[allow(dead_code)]
pub struct IdrisPatternBuilder;
impl IdrisPatternBuilder {
    /// Build a constructor pattern `Con p1 p2 p3`.
    #[allow(dead_code)]
    pub fn con(name: impl Into<String>, args: Vec<IdrisPattern>) -> IdrisPattern {
        IdrisPattern::Con(name.into(), args)
    }
    /// Build a variable binding pattern.
    #[allow(dead_code)]
    pub fn var(name: impl Into<String>) -> IdrisPattern {
        IdrisPattern::Var(name.into())
    }
    /// Build a wildcard pattern `_`.
    #[allow(dead_code)]
    pub fn wildcard() -> IdrisPattern {
        IdrisPattern::Wildcard
    }
    /// Build a literal pattern.
    #[allow(dead_code)]
    pub fn lit(l: IdrisLiteral) -> IdrisPattern {
        IdrisPattern::Lit(l)
    }
    /// Build a tuple pattern `(p1, p2)`.
    #[allow(dead_code)]
    pub fn tuple(pats: Vec<IdrisPattern>) -> IdrisPattern {
        IdrisPattern::Tuple(pats)
    }
    /// Build an `as` pattern `pat@name`.
    #[allow(dead_code)]
    pub fn as_pat(name: impl Into<String>, pat: IdrisPattern) -> IdrisPattern {
        IdrisPattern::As(name.into(), Box::new(pat))
    }
}
/// A type in Idris 2's type theory.
#[derive(Debug, Clone, PartialEq)]
pub enum IdrisType {
    /// `Type` — the type of types (universe).
    Type,
    /// `Integer` — arbitrary-precision integer.
    Integer,
    /// `Nat` — natural number.
    Nat,
    /// `Bool` — boolean.
    Bool,
    /// `String` — text string.
    String,
    /// `Char` — unicode character.
    Char,
    /// `Double` — 64-bit floating-point.
    Double,
    /// `List a` — list type.
    List(Box<IdrisType>),
    /// `Vect n a` — length-indexed vector.
    Vect(Box<IdrisExpr>, Box<IdrisType>),
    /// `(a, b)` — pair / product type.
    Pair(Box<IdrisType>, Box<IdrisType>),
    /// `()` / `Unit` — unit type.
    Unit,
    /// `a -> b` — function type (unrestricted arrow).
    Function(Box<IdrisType>, Box<IdrisType>),
    /// `(1 x : a) -> b` — linear function type.
    Linear(Box<IdrisType>, Box<IdrisType>),
    /// `(0 x : a) -> b` — erased argument function.
    Erased(Box<IdrisType>, Box<IdrisType>),
    /// `(x : a) -> b x` — dependent function type (Pi).
    Pi(String, Box<IdrisType>, Box<IdrisType>),
    /// A named data type reference, e.g. `MyType`, `Maybe Int`.
    Data(String, Vec<IdrisType>),
    /// An interface constraint, e.g. `Eq a => ...`
    Interface(String, Vec<IdrisType>),
    /// A type variable (used in polymorphic types).
    Var(String),
    /// `IO a` — IO action type.
    IO(Box<IdrisType>),
    /// `Maybe a` — optional type.
    Maybe(Box<IdrisType>),
    /// `Either a b` — sum type.
    Either(Box<IdrisType>, Box<IdrisType>),
}
impl IdrisType {
    /// Whether this type needs parentheses in argument position.
    pub(super) fn needs_parens(&self) -> bool {
        matches!(
            self,
            IdrisType::Function(_, _)
                | IdrisType::Linear(_, _)
                | IdrisType::Erased(_, _)
                | IdrisType::Pi(_, _, _)
                | IdrisType::Interface(_, _)
        ) || matches!(self, IdrisType::Data(_, args) if ! args.is_empty())
    }
    /// Format with optional parenthesisation.
    pub(super) fn fmt_parens(&self) -> String {
        if self.needs_parens() {
            format!("({})", self)
        } else {
            format!("{}", self)
        }
    }
}
/// A constructor in a `data` or `record` type.
#[derive(Debug, Clone, PartialEq)]
pub struct IdrisConstructor {
    /// Constructor name.
    pub name: String,
    /// Constructor type signature (usually a chain of `->` ending at the data type).
    pub ty: IdrisType,
    /// Optional doc comment.
    pub doc: Option<String>,
}
/// An `interface` (type class) declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct IdrisInterface {
    /// Interface name.
    pub name: String,
    /// Superclass constraints: `Eq a =>`.
    pub constraints: Vec<(String, Vec<IdrisType>)>,
    /// Type parameters.
    pub params: Vec<(String, IdrisType)>,
    /// Methods: `(name, type_sig)`.
    pub methods: Vec<(String, IdrisType)>,
    /// Default method implementations.
    pub defaults: Vec<IdrisFunction>,
    /// Visibility.
    pub visibility: Visibility,
    /// Optional doc comment.
    pub doc: Option<String>,
}
impl IdrisInterface {
    /// Emit this interface declaration.
    pub fn emit(&self, indent: usize) -> String {
        let pad = " ".repeat(indent);
        let mut out = String::new();
        if let Some(doc) = &self.doc {
            for line in doc.lines() {
                out.push_str(&format!("{}||| {}\n", pad, line));
            }
        }
        let constraints_str: String = self
            .constraints
            .iter()
            .map(|(c, args)| {
                if args.is_empty() {
                    c.clone()
                } else {
                    format!(
                        "{} {}",
                        c,
                        args.iter()
                            .map(|a| format!("{}", a))
                            .collect::<Vec<_>>()
                            .join(" ")
                    )
                }
            })
            .collect::<Vec<_>>()
            .join(", ");
        let params_str: String = self
            .params
            .iter()
            .map(|(n, t)| format!(" ({} : {})", n, t))
            .collect();
        if constraints_str.is_empty() {
            out.push_str(&format!(
                "{}{}interface {}{} where\n",
                pad, self.visibility, self.name, params_str
            ));
        } else {
            out.push_str(&format!(
                "{}{}interface {} => {}{} where\n",
                pad, self.visibility, constraints_str, self.name, params_str
            ));
        }
        for (mname, mtype) in &self.methods {
            out.push_str(&format!("{}  {} : {}\n", pad, mname, mtype));
        }
        for default in &self.defaults {
            out.push_str(&default.emit(indent + 2));
        }
        out
    }
}
/// A `parameters` block, grouping shared implicit parameters.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct IdrisParametersBlock {
    /// Shared parameters declared for all contained definitions.
    pub params: Vec<(String, IdrisType)>,
    /// Declarations inside the block.
    pub decls: Vec<IdrisDecl>,
}
impl IdrisParametersBlock {
    /// Create a new parameters block.
    #[allow(dead_code)]
    pub fn new(params: Vec<(String, IdrisType)>) -> Self {
        IdrisParametersBlock {
            params,
            decls: Vec::new(),
        }
    }
    /// Add a declaration to the block.
    #[allow(dead_code)]
    pub fn add(&mut self, decl: IdrisDecl) {
        self.decls.push(decl);
    }
    /// Emit the parameters block.
    #[allow(dead_code)]
    pub fn emit(&self, backend: &IdrisBackend) -> String {
        let params_str: Vec<String> = self
            .params
            .iter()
            .map(|(n, t)| format!("({} : {})", n, t))
            .collect();
        let mut out = format!("parameters {}\n", params_str.join(" "));
        for decl in &self.decls {
            for line in backend.emit_decl(decl).lines() {
                out.push_str("    ");
                out.push_str(line);
                out.push('\n');
            }
        }
        out
    }
}
/// An Idris 2 `implementation` (instance) of an interface.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct IdrisImplementation {
    /// Optional implementation name.
    pub name: Option<String>,
    /// Interface name being implemented.
    pub interface: String,
    /// Type arguments.
    pub type_args: Vec<IdrisType>,
    /// Constraints on the implementation.
    pub constraints: Vec<IdrisType>,
    /// Method clauses: function definitions.
    pub clauses: Vec<IdrisFunction>,
    /// Visibility.
    pub visibility: Visibility,
}
impl IdrisImplementation {
    /// Create a new implementation.
    #[allow(dead_code)]
    pub fn new(interface: impl Into<String>, type_args: Vec<IdrisType>) -> Self {
        IdrisImplementation {
            name: None,
            interface: interface.into(),
            type_args,
            constraints: Vec::new(),
            clauses: Vec::new(),
            visibility: Visibility::Default,
        }
    }
    /// Add a method clause.
    #[allow(dead_code)]
    pub fn add_method(&mut self, func: IdrisFunction) {
        self.clauses.push(func);
    }
    /// Emit the implementation block.
    #[allow(dead_code)]
    pub fn emit(&self, backend: &IdrisBackend) -> String {
        let name_part = self
            .name
            .as_ref()
            .map(|n| format!("[{}] ", n))
            .unwrap_or_default();
        let constraints_str = if self.constraints.is_empty() {
            String::new()
        } else {
            let cs: Vec<String> = self.constraints.iter().map(|c| format!("{}", c)).collect();
            format!("{} => ", cs.join(", "))
        };
        let type_args_str: Vec<String> = self.type_args.iter().map(|t| format!("{}", t)).collect();
        let mut out = format!(
            "{}implementation {}{}{} {} where\n",
            self.visibility,
            name_part,
            constraints_str,
            self.interface,
            type_args_str.join(" "),
        );
        for clause in &self.clauses {
            let decl = IdrisDecl::Func(clause.clone());
            for line in backend.emit_decl(&decl).lines() {
                out.push_str("    ");
                out.push_str(line);
                out.push('\n');
            }
        }
        out
    }
}
/// Idris 2 compiler pragmas and directives.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum IdrisPragma {
    /// `%name x xs, xss` — suggest names for case-split.
    Name(String, Vec<String>),
    /// `%auto_implicit` — enable auto-implicit arguments.
    AutoImplicit,
    /// `%default total` — make all definitions total by default.
    DefaultTotal,
    /// `%default partial` — allow partial definitions by default.
    DefaultPartial,
    /// `%default covering` — require covering definitions by default.
    DefaultCovering,
    /// `%inline` — hint that function should be inlined.
    Inline,
    /// `%noinline` — hint that function should not be inlined.
    NoInline,
    /// `%hint` — register as an auto-search hint.
    Hint,
    /// `%extern` — mark function as externally defined.
    Extern,
    /// `%builtin NaturalToInteger` etc.
    Builtin(String),
    /// `%foreign <backend> <impl>`.
    Foreign { backend: String, impl_str: String },
    /// `%transform` — mark as a rewrite rule.
    Transform(String),
    /// `%deprecate` — mark as deprecated.
    Deprecate(Option<String>),
    /// `%hide Module.name` — hide from namespace.
    Hide(String),
    /// `%unbound_implicits off`.
    UnboundImplicitsOff,
    /// `%ambiguity_depth N`.
    AmbiguityDepth(u32),
    /// `%search_timeout N`.
    SearchTimeout(u32),
    /// `%logging topic N`.
    Logging { topic: String, level: u32 },
    /// `%language ElabReflection` or similar.
    Language(String),
    /// `%run_elab <expr>` — run elaborator reflection.
    RunElab(String),
}
/// A pattern in an Idris 2 function clause.
#[derive(Debug, Clone, PartialEq)]
pub enum IdrisPattern {
    /// Wildcard `_`.
    Wildcard,
    /// Variable `x`.
    Var(String),
    /// Constructor application `Ctor p1 p2`.
    Con(String, Vec<IdrisPattern>),
    /// Literal pattern.
    Lit(IdrisLiteral),
    /// Tuple pattern `(p1, p2)`.
    Tuple(Vec<IdrisPattern>),
    /// Nil `[]`.
    Nil,
    /// Cons `x :: xs`.
    Cons(Box<IdrisPattern>, Box<IdrisPattern>),
    /// As-pattern `n@p`.
    As(String, Box<IdrisPattern>),
    /// Implicit argument pattern `{p}`.
    Implicit(Box<IdrisPattern>),
    /// Inaccessible (dot) pattern `.t`.
    Dot(Box<IdrisExpr>),
}
/// A top-level declaration in an Idris 2 module.
#[derive(Debug, Clone, PartialEq)]
pub enum IdrisDecl {
    /// A function definition.
    Func(IdrisFunction),
    /// A `data` declaration.
    Data(IdrisData),
    /// A `record` declaration.
    Record(IdrisRecord),
    /// An `interface` declaration.
    Interface(IdrisInterface),
    /// An `implementation` block.
    Implementation(IdrisImpl),
    /// A `namespace N where` block.
    Namespace(String, Vec<IdrisDecl>),
    /// A `mutual` block for mutually recursive definitions.
    Mutual(Vec<IdrisDecl>),
    /// A `parameters` block: `parameters (x : T) where`.
    Parameters(Vec<(String, IdrisType)>, Vec<IdrisDecl>),
    /// A `%` pragma: `%name T x, y, z`.
    Pragma(String),
    /// A type-level `postulate` (axiom).
    Postulate(String, IdrisType, Visibility),
    /// A line comment.
    Comment(String),
    /// A blank separator.
    Blank,
}
impl IdrisDecl {
    /// Emit this declaration at the given indent level.
    pub fn emit(&self, indent: usize) -> String {
        let pad = " ".repeat(indent);
        match self {
            IdrisDecl::Func(f) => f.emit(indent),
            IdrisDecl::Data(d) => d.emit(indent),
            IdrisDecl::Record(r) => r.emit(indent),
            IdrisDecl::Interface(i) => i.emit(indent),
            IdrisDecl::Implementation(im) => im.emit(indent),
            IdrisDecl::Namespace(name, decls) => {
                let mut out = format!("{}namespace {} where\n", pad, name);
                for d in decls {
                    out.push_str(&d.emit(indent + 2));
                }
                out
            }
            IdrisDecl::Mutual(decls) => {
                let mut out = format!("{}mutual\n", pad);
                for d in decls {
                    out.push_str(&d.emit(indent + 2));
                }
                out
            }
            IdrisDecl::Parameters(params, decls) => {
                let params_str: String = params
                    .iter()
                    .map(|(n, t)| format!("({} : {})", n, t))
                    .collect::<Vec<_>>()
                    .join(" ");
                let mut out = format!("{}parameters {}\n", pad, params_str);
                for d in decls {
                    out.push_str(&d.emit(indent + 2));
                }
                out
            }
            IdrisDecl::Pragma(s) => format!("{}%{}\n", pad, s),
            IdrisDecl::Postulate(name, ty, vis) => {
                format!("{}{}postulate {} : {}\n", pad, vis, name, ty)
            }
            IdrisDecl::Comment(s) => format!("{}-- {}\n", pad, s),
            IdrisDecl::Blank => String::from("\n"),
        }
    }
}
/// Collection of common Idris 2 standard library patterns for code generation.
#[allow(dead_code)]
pub struct IdrisStdlibSnippets;
impl IdrisStdlibSnippets {
    /// Emit a `mapMaybe` helper over a list.
    #[allow(dead_code)]
    pub fn map_maybe_fn() -> IdrisFunction {
        IdrisFunction::with_clauses(
            "mapMaybe",
            IdrisTypeBuilder::arrow(vec![
                IdrisTypeBuilder::arrow(vec![
                    IdrisType::Var("a".to_string()),
                    IdrisTypeBuilder::maybe(IdrisType::Var("b".to_string())),
                ]),
                IdrisTypeBuilder::list(IdrisType::Var("a".to_string())),
                IdrisTypeBuilder::list(IdrisType::Var("b".to_string())),
            ]),
            vec![
                (
                    vec![
                        IdrisPatternBuilder::wildcard(),
                        IdrisPatternBuilder::con("Nil", vec![]),
                    ],
                    IdrisExpr::Var("Nil".to_string()),
                ),
                (
                    vec![
                        IdrisPatternBuilder::var("f"),
                        IdrisPatternBuilder::con(
                            "::",
                            vec![
                                IdrisPatternBuilder::var("x"),
                                IdrisPatternBuilder::var("xs"),
                            ],
                        ),
                    ],
                    IdrisExprBuilder::case_of(
                        IdrisExprBuilder::app_chain(
                            IdrisExpr::Var("f".to_string()),
                            vec![IdrisExpr::Var("x".to_string())],
                        ),
                        vec![
                            (
                                IdrisPatternBuilder::con("Nothing", vec![]),
                                IdrisExprBuilder::app_chain(
                                    IdrisExpr::Var("mapMaybe".to_string()),
                                    vec![
                                        IdrisExpr::Var("f".to_string()),
                                        IdrisExpr::Var("xs".to_string()),
                                    ],
                                ),
                            ),
                            (
                                IdrisPatternBuilder::con(
                                    "Just",
                                    vec![IdrisPatternBuilder::var("y")],
                                ),
                                IdrisExprBuilder::app_chain(
                                    IdrisExpr::Var("::".to_string()),
                                    vec![
                                        IdrisExpr::Var("y".to_string()),
                                        IdrisExprBuilder::app_chain(
                                            IdrisExpr::Var("mapMaybe".to_string()),
                                            vec![
                                                IdrisExpr::Var("f".to_string()),
                                                IdrisExpr::Var("xs".to_string()),
                                            ],
                                        ),
                                    ],
                                ),
                            ),
                        ],
                    ),
                ),
            ],
        )
    }
    /// Emit a `foldr` implementation.
    #[allow(dead_code)]
    pub fn foldr_fn() -> String {
        "foldr : (a -> b -> b) -> b -> List a -> b\nfoldr f z [] = z\nfoldr f z (x :: xs) = f x (foldr f z xs)\n"
            .to_string()
    }
    /// Emit a `zip` implementation.
    #[allow(dead_code)]
    pub fn zip_fn() -> String {
        "zip : List a -> List b -> List (a, b)\nzip [] _ = []\nzip _ [] = []\nzip (x :: xs) (y :: ys) = (x, y) :: zip xs ys\n"
            .to_string()
    }
    /// Emit a `nub` (remove duplicates) implementation using DecEq.
    #[allow(dead_code)]
    pub fn nub_fn() -> String {
        "nub : DecEq a => List a -> List a\nnub [] = []\nnub (x :: xs) = x :: nub (filter (/= x) xs)\n"
            .to_string()
    }
    /// Emit a simple `show` implementation for a nat-like type.
    #[allow(dead_code)]
    pub fn show_nat_instance(type_name: &str) -> String {
        format!("Show {} where\n    show x = show (toNat x)\n", type_name)
    }
}
/// A literal value in Idris 2.
#[derive(Debug, Clone, PartialEq)]
pub enum IdrisLiteral {
    /// Integer literal `42`, `-7`.
    Int(i64),
    /// Natural number literal `3`.
    Nat(u64),
    /// Float literal `3.14`.
    Float(f64),
    /// Character literal `'a'`.
    Char(char),
    /// String literal `"hello"`.
    Str(String),
    /// Boolean `True`.
    True,
    /// Boolean `False`.
    False,
    /// Unit `()`.
    Unit,
}
/// An expression in Idris 2.
#[derive(Debug, Clone, PartialEq)]
pub enum IdrisExpr {
    /// A literal value.
    Lit(IdrisLiteral),
    /// A variable reference `x`.
    Var(String),
    /// A fully qualified name `Module.name`.
    Qualified(Vec<String>),
    /// Function application `f x`.
    App(Box<IdrisExpr>, Box<IdrisExpr>),
    /// Lambda `\x => body`.
    Lam(Vec<String>, Box<IdrisExpr>),
    /// Let binding `let x = val in body`.
    Let(String, Box<IdrisExpr>, Box<IdrisExpr>),
    /// `case scrutinee of { alts }`.
    CaseOf(Box<IdrisExpr>, Vec<(IdrisPattern, IdrisExpr)>),
    /// `if cond then t else e`.
    IfThenElse(Box<IdrisExpr>, Box<IdrisExpr>, Box<IdrisExpr>),
    /// A `do` block: list of statements.
    Do(Vec<IdrisDoStmt>),
    /// Tuple `(e1, e2, ...)`.
    Tuple(Vec<IdrisExpr>),
    /// List literal `[e1, e2, ...]`.
    ListLit(Vec<IdrisExpr>),
    /// Type annotation `(e : T)`.
    Annot(Box<IdrisExpr>, Box<IdrisType>),
    /// An idiom bracket `[| f x y |]`.
    Idiom(Box<IdrisExpr>),
    /// A proof term / tactic block `believe_me x`.
    ProofTerm(Box<IdrisExpr>),
    /// `with` view pattern expression.
    WithPattern(Box<IdrisExpr>, Vec<(IdrisPattern, IdrisExpr)>),
    /// Infix operator expression `a `op` b`.
    Infix(String, Box<IdrisExpr>, Box<IdrisExpr>),
    /// A hole `?name`.
    Hole(String),
    /// `refl` — reflexivity proof.
    Refl,
    /// `?_` — anonymous hole.
    AnonHole,
    /// A record update `{ field = val }`.
    RecordUpdate(Box<IdrisExpr>, Vec<(String, IdrisExpr)>),
    /// A negative integer `-n`.
    Neg(Box<IdrisExpr>),
}
impl IdrisExpr {
    /// Whether this expression needs parentheses in application position.
    pub(super) fn needs_parens(&self) -> bool {
        matches!(
            self,
            IdrisExpr::App(_, _)
                | IdrisExpr::Lam(_, _)
                | IdrisExpr::Let(_, _, _)
                | IdrisExpr::CaseOf(_, _)
                | IdrisExpr::IfThenElse(_, _, _)
                | IdrisExpr::Do(_)
                | IdrisExpr::Infix(_, _, _)
                | IdrisExpr::Annot(_, _)
                | IdrisExpr::Neg(_)
        )
    }
    /// Format with parens if needed.
    pub(super) fn fmt_arg(&self) -> String {
        if self.needs_parens() {
            format!("({})", self)
        } else {
            format!("{}", self)
        }
    }
}
/// An `implementation` (type class instance) block.
#[derive(Debug, Clone, PartialEq)]
pub struct IdrisImpl {
    /// Optional implementation name (e.g. `[NatEq]`).
    pub impl_name: Option<String>,
    /// Constraints: `Eq a =>`.
    pub constraints: Vec<(String, Vec<IdrisType>)>,
    /// Interface being implemented.
    pub interface: String,
    /// Type arguments.
    pub args: Vec<IdrisType>,
    /// Method implementations.
    pub methods: Vec<IdrisFunction>,
    /// Visibility.
    pub visibility: Visibility,
}
impl IdrisImpl {
    /// Emit this implementation block.
    pub fn emit(&self, indent: usize) -> String {
        let pad = " ".repeat(indent);
        let mut out = String::new();
        let constraints_str: String = self
            .constraints
            .iter()
            .map(|(c, args)| {
                if args.is_empty() {
                    c.clone()
                } else {
                    format!(
                        "{} {}",
                        c,
                        args.iter()
                            .map(|a| format!("{}", a))
                            .collect::<Vec<_>>()
                            .join(" ")
                    )
                }
            })
            .collect::<Vec<_>>()
            .join(", ");
        let args_str: String = self.args.iter().map(|a| format!(" {}", a)).collect();
        let name_str = self
            .impl_name
            .as_ref()
            .map(|n| format!(" [{}]", n))
            .unwrap_or_default();
        if constraints_str.is_empty() {
            out.push_str(&format!(
                "{}{}implementation{} {}{} where\n",
                pad, self.visibility, name_str, self.interface, args_str
            ));
        } else {
            out.push_str(&format!(
                "{}{}implementation{} {} => {}{} where\n",
                pad, self.visibility, name_str, constraints_str, self.interface, args_str
            ));
        }
        for method in &self.methods {
            out.push_str(&method.emit(indent + 2));
        }
        out
    }
}
/// The Idris 2 code generation backend.
///
/// Converts `IdrisModule` / individual declarations into `.idr` source text.
#[derive(Debug, Default)]
pub struct IdrisBackend {
    /// Whether to add `-- AUTO-GENERATED` header.
    pub add_header: bool,
    /// Whether to emit `%default total` at the top of each module.
    pub default_total: bool,
    /// Whether to emit `%auto_implicit` pragmas.
    pub auto_implicit: bool,
    /// Configuration options.
    pub options: IdrisBackendOptions,
}
impl IdrisBackend {
    /// Create a backend with default settings.
    pub fn new() -> Self {
        IdrisBackend {
            add_header: false,
            default_total: false,
            auto_implicit: false,
            options: IdrisBackendOptions {
                blank_between_decls: true,
                emit_docs: true,
            },
        }
    }
    /// Create a backend optimised for proof-oriented Idris 2 output.
    pub fn proof_mode() -> Self {
        IdrisBackend {
            add_header: true,
            default_total: true,
            auto_implicit: true,
            options: IdrisBackendOptions {
                blank_between_decls: true,
                emit_docs: true,
            },
        }
    }
    /// Emit a complete `IdrisModule` as `.idr` source text.
    pub fn emit_module(&self, module: &IdrisModule) -> String {
        let mut out = String::new();
        if self.add_header {
            out.push_str("-- AUTO-GENERATED by OxiLean\n\n");
        }
        if let Some(doc) = &module.doc {
            for line in doc.lines() {
                out.push_str(&format!("||| {}\n", line));
            }
        }
        let module_str = module.module_name.join(".");
        out.push_str(&format!("module {}\n\n", module_str));
        if self.default_total {
            out.push_str("%default total\n\n");
        }
        if self.auto_implicit {
            out.push_str("%auto_implicit_depth 50\n\n");
        }
        for imp in &module.imports {
            out.push_str(&imp.emit());
            out.push('\n');
        }
        if !module.imports.is_empty() {
            out.push('\n');
        }
        for decl in &module.declarations {
            let s = decl.emit(0);
            out.push_str(&s);
            if self.options.blank_between_decls
                && !matches!(decl, IdrisDecl::Blank | IdrisDecl::Comment(_))
            {
                out.push('\n');
            }
        }
        out
    }
    /// Emit a single declaration.
    pub fn emit_decl(&self, decl: &IdrisDecl) -> String {
        decl.emit(0)
    }
    /// Emit a single function.
    pub fn emit_function(&self, func: &IdrisFunction) -> String {
        func.emit(0)
    }
    /// Emit a single data declaration.
    pub fn emit_data(&self, data: &IdrisData) -> String {
        data.emit(0)
    }
    /// Emit a single record.
    pub fn emit_record(&self, rec: &IdrisRecord) -> String {
        rec.emit(0)
    }
    /// Emit a single interface.
    pub fn emit_interface(&self, iface: &IdrisInterface) -> String {
        iface.emit(0)
    }
    /// Emit a single implementation.
    pub fn emit_impl(&self, im: &IdrisImpl) -> String {
        im.emit(0)
    }
}
/// Export visibility of a top-level declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Visibility {
    /// `public export` — type and implementation visible.
    PublicExport,
    /// `export` — type visible, implementation hidden.
    Export,
    /// `private` — hidden from other modules.
    Private,
    /// No annotation (module-local default).
    Default,
}
/// Totality annotation for a function.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Totality {
    /// `total` — must be total (no partial/diverging cases).
    Total,
    /// `partial` — allowed to be partial.
    Partial,
    /// `covering` — all cases must be handled but may be partial.
    Covering,
    /// No explicit annotation (use module default).
    Default,
}
/// A statement inside a `do` block.
#[derive(Debug, Clone, PartialEq)]
pub enum IdrisDoStmt {
    /// `x <- action`
    Bind(String, IdrisExpr),
    /// `let x = val`
    Let(String, IdrisExpr),
    /// A bare expression (last statement or side-effect).
    Expr(IdrisExpr),
    /// `let x : T = val`
    LetTyped(String, IdrisType, IdrisExpr),
    /// `ignore action`
    Ignore(IdrisExpr),
}
/// Configuration options for the Idris backend.
#[derive(Debug, Default, Clone)]
pub struct IdrisBackendOptions {
    /// Emit blank lines between top-level declarations.
    pub blank_between_decls: bool,
    /// Emit `||| doc` comments for items with doc strings.
    pub emit_docs: bool,
}
/// An Idris 2 `interface` definition.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct IdrisInterfaceExt {
    /// Interface name.
    pub name: String,
    /// Type parameters.
    pub params: Vec<(String, IdrisType)>,
    /// Superclass constraints.
    pub constraints: Vec<IdrisType>,
    /// Method declarations: (name, type, optional default impl).
    pub methods: Vec<(String, IdrisType, Option<IdrisExpr>)>,
    /// Visibility.
    pub visibility: Visibility,
    /// Optional doc comment.
    pub doc: Option<String>,
}
impl IdrisInterfaceExt {
    /// Create a minimal interface definition.
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, params: Vec<(String, IdrisType)>) -> Self {
        IdrisInterfaceExt {
            name: name.into(),
            params,
            constraints: Vec::new(),
            methods: Vec::new(),
            visibility: Visibility::PublicExport,
            doc: None,
        }
    }
    /// Add a method signature.
    #[allow(dead_code)]
    pub fn add_method(&mut self, name: impl Into<String>, ty: IdrisType) {
        self.methods.push((name.into(), ty, None));
    }
    /// Add a method with a default implementation.
    #[allow(dead_code)]
    pub fn add_method_with_default(
        &mut self,
        name: impl Into<String>,
        ty: IdrisType,
        default: IdrisExpr,
    ) {
        self.methods.push((name.into(), ty, Some(default)));
    }
    /// Emit the interface definition.
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        let mut out = String::new();
        if let Some(doc) = &self.doc {
            for line in doc.lines() {
                out.push_str(&format!("||| {}\n", line));
            }
        }
        let params_str: Vec<String> = self
            .params
            .iter()
            .map(|(n, t)| format!("({} : {})", n, t))
            .collect();
        let constraints_str = if self.constraints.is_empty() {
            String::new()
        } else {
            let cs: Vec<String> = self.constraints.iter().map(|c| format!("{}", c)).collect();
            format!("{} => ", cs.join(", "))
        };
        out.push_str(&format!(
            "{}interface {}{} {} where\n",
            self.visibility,
            constraints_str,
            self.name,
            params_str.join(" ")
        ));
        for (mname, mty, mdefault) in &self.methods {
            out.push_str(&format!("    {} : {}\n", mname, mty));
            if let Some(def) = mdefault {
                out.push_str(&format!("    {} _ = {}\n", mname, def));
            }
        }
        out
    }
}
/// Fluent helper for building complex Idris 2 types.
#[allow(dead_code)]
pub struct IdrisTypeBuilder;
impl IdrisTypeBuilder {
    /// Build a function type `a -> b -> ... -> z`.
    #[allow(dead_code)]
    pub fn arrow(types: Vec<IdrisType>) -> IdrisType {
        assert!(!types.is_empty(), "arrow type must have at least one type");
        let mut it = types.into_iter().rev();
        let mut result = it
            .next()
            .expect("types is non-empty; guaranteed by assert above");
        for ty in it {
            result = IdrisType::Function(Box::new(ty), Box::new(result));
        }
        result
    }
    /// Build a type application `T a b c`.
    #[allow(dead_code)]
    pub fn app(head: impl Into<String>, args: Vec<IdrisType>) -> IdrisType {
        IdrisType::Data(head.into(), args)
    }
    /// Build a `Vect n a` type.
    #[allow(dead_code)]
    pub fn vect(n_expr: impl Into<String>, elem_ty: IdrisType) -> IdrisType {
        IdrisType::Data(
            "Vect".to_string(),
            vec![IdrisType::Var(n_expr.into()), elem_ty],
        )
    }
    /// Build a `List a` type.
    #[allow(dead_code)]
    pub fn list(elem_ty: IdrisType) -> IdrisType {
        IdrisType::List(Box::new(elem_ty))
    }
    /// Build a `Maybe a` type.
    #[allow(dead_code)]
    pub fn maybe(ty: IdrisType) -> IdrisType {
        IdrisType::Data("Maybe".to_string(), vec![ty])
    }
    /// Build an `Either a b` type.
    #[allow(dead_code)]
    pub fn either(left: IdrisType, right: IdrisType) -> IdrisType {
        IdrisType::Data("Either".to_string(), vec![left, right])
    }
    /// Build a `Pair a b` (tuple) type.
    #[allow(dead_code)]
    pub fn pair(a: IdrisType, b: IdrisType) -> IdrisType {
        IdrisType::Data("Pair".to_string(), vec![a, b])
    }
    /// Build an `IO a` type.
    #[allow(dead_code)]
    pub fn io(ty: IdrisType) -> IdrisType {
        IdrisType::Data("IO".to_string(), vec![ty])
    }
    /// `Nat`
    #[allow(dead_code)]
    pub fn nat() -> IdrisType {
        IdrisType::Nat
    }
    /// `Bool`
    #[allow(dead_code)]
    pub fn bool() -> IdrisType {
        IdrisType::Bool
    }
    /// `String`
    #[allow(dead_code)]
    pub fn string() -> IdrisType {
        IdrisType::String
    }
    /// `Integer`
    #[allow(dead_code)]
    pub fn integer() -> IdrisType {
        IdrisType::Integer
    }
    /// Dependent function type `(x : a) -> b`.
    #[allow(dead_code)]
    pub fn pi(param: impl Into<String>, domain: IdrisType, codomain: IdrisType) -> IdrisType {
        IdrisType::Pi(param.into(), Box::new(domain), Box::new(codomain))
    }
}
/// Fluent builder for constructing complete IdrisModule objects.
#[derive(Debug)]
#[allow(dead_code)]
pub struct IdrisModuleBuilder {
    pub(super) module: IdrisModule,
}
impl IdrisModuleBuilder {
    /// Start building a new module with the given hierarchical name.
    #[allow(dead_code)]
    pub fn new(parts: Vec<String>) -> Self {
        IdrisModuleBuilder {
            module: IdrisModule::new(parts),
        }
    }
    /// Add an import.
    #[allow(dead_code)]
    pub fn import(mut self, imp: IdrisImport) -> Self {
        self.module.import(imp);
        self
    }
    /// Add a public import.
    #[allow(dead_code)]
    pub fn public_import(mut self, parts: Vec<String>) -> Self {
        self.module.import(IdrisImport::public_import(parts));
        self
    }
    /// Add a declaration.
    #[allow(dead_code)]
    pub fn decl(mut self, decl: IdrisDecl) -> Self {
        self.module.add(decl);
        self
    }
    /// Add a pragma.
    #[allow(dead_code)]
    pub fn pragma(mut self, pragma: IdrisPragma) -> Self {
        self.module.add(IdrisDecl::Pragma(format!("{}", pragma)));
        self
    }
    /// Add a comment.
    #[allow(dead_code)]
    pub fn comment(mut self, text: impl Into<String>) -> Self {
        self.module.add(IdrisDecl::Comment(text.into()));
        self
    }
    /// Consume the builder and produce the module.
    #[allow(dead_code)]
    pub fn build(self) -> IdrisModule {
        self.module
    }
}
/// Fluent helper for building complex Idris 2 expressions.
#[allow(dead_code)]
pub struct IdrisExprBuilder;
impl IdrisExprBuilder {
    /// Build a function application chain: `f x y z`.
    #[allow(dead_code)]
    pub fn app_chain(func: IdrisExpr, args: Vec<IdrisExpr>) -> IdrisExpr {
        args.into_iter().fold(func, |acc, arg| {
            IdrisExpr::App(Box::new(acc), Box::new(arg))
        })
    }
    /// Build a `case expr of` expression.
    #[allow(dead_code)]
    pub fn case_of(scrutinee: IdrisExpr, alts: Vec<(IdrisPattern, IdrisExpr)>) -> IdrisExpr {
        IdrisExpr::CaseOf(Box::new(scrutinee), alts)
    }
    /// Build a let-in chain: `let x1=e1; x2=e2; ... in body`.
    #[allow(dead_code)]
    pub fn let_chain(bindings: Vec<(String, IdrisExpr)>, body: IdrisExpr) -> IdrisExpr {
        bindings.into_iter().rev().fold(body, |acc, (name, val)| {
            IdrisExpr::Let(name, Box::new(val), Box::new(acc))
        })
    }
    /// Build a lambda over multiple parameters: `\x, y, z => body`.
    #[allow(dead_code)]
    pub fn lam(params: Vec<impl Into<String>>, body: IdrisExpr) -> IdrisExpr {
        IdrisExpr::Lam(
            params.into_iter().map(|p| p.into()).collect(),
            Box::new(body),
        )
    }
    /// Build a do-block.
    #[allow(dead_code)]
    pub fn do_block(stmts: Vec<IdrisDoStmt>) -> IdrisExpr {
        IdrisExpr::Do(stmts)
    }
    /// Build `if c then t else e`.
    #[allow(dead_code)]
    pub fn if_then_else(cond: IdrisExpr, then_e: IdrisExpr, else_e: IdrisExpr) -> IdrisExpr {
        IdrisExpr::IfThenElse(Box::new(cond), Box::new(then_e), Box::new(else_e))
    }
    /// Build a tuple expression.
    #[allow(dead_code)]
    pub fn tuple(elems: Vec<IdrisExpr>) -> IdrisExpr {
        IdrisExpr::Tuple(elems)
    }
    /// Build a list literal.
    #[allow(dead_code)]
    pub fn list_lit(elems: Vec<IdrisExpr>) -> IdrisExpr {
        IdrisExpr::ListLit(elems)
    }
    /// Build a type-annotated expression `(e : T)`.
    #[allow(dead_code)]
    pub fn annot(expr: IdrisExpr, ty: IdrisType) -> IdrisExpr {
        IdrisExpr::Annot(Box::new(expr), Box::new(ty))
    }
    /// Build an infix expression `l op r`.
    #[allow(dead_code)]
    pub fn infix(op: impl Into<String>, l: IdrisExpr, r: IdrisExpr) -> IdrisExpr {
        IdrisExpr::Infix(op.into(), Box::new(l), Box::new(r))
    }
}
/// Elaboration attributes that can be applied to types, functions, or constructors.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum IdrisAttribute {
    /// `@{auto}` — auto-implicit solve.
    Auto,
    /// `@{interface}` — interface auto-search.
    Interface,
    /// `@{search}` — proof search.
    Search,
    /// `[totality]` — totality requirement.
    Totality(Totality),
    /// `[inline]` — inline hint.
    Inline,
    /// `[static]` — static argument.
    Static,
}
/// Configuration controlling how Idris 2 code is generated.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct IdrisCodegenConfig {
    /// Whether to emit docstring comments.
    pub emit_docs: bool,
    /// Whether to emit `%logging` pragmas for debugging.
    pub emit_logging: bool,
    /// Default totality annotation.
    pub default_totality: Totality,
    /// Whether to emit `%inline` on small functions.
    pub auto_inline: bool,
    /// Maximum body size (in lines) for auto-inlining.
    pub auto_inline_limit: usize,
    /// Whether to add `%name` pragmas.
    pub emit_name_pragmas: bool,
    /// Whether to emit a module header comment.
    pub emit_header_comment: bool,
    /// Target backend: `"chez"`, `"node"`, `"refc"`, etc.
    pub target_backend: String,
}
/// Metrics about a generated Idris 2 module.
#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct IdrisModuleMetrics {
    /// Number of function definitions.
    pub num_functions: usize,
    /// Number of data type definitions.
    pub num_data_types: usize,
    /// Number of record definitions.
    pub num_records: usize,
    /// Number of import declarations.
    pub num_imports: usize,
    /// Total number of clauses across all functions.
    pub total_clauses: usize,
    /// Number of mutual blocks.
    pub num_mutual_blocks: usize,
    /// Number of pragma declarations.
    pub num_pragmas: usize,
}
impl IdrisModuleMetrics {
    /// Compute metrics from an IdrisModule.
    #[allow(dead_code)]
    pub fn compute(module: &IdrisModule) -> Self {
        let mut m = IdrisModuleMetrics {
            num_imports: module.imports.len(),
            ..Default::default()
        };
        for decl in &module.declarations {
            Self::count_decl(decl, &mut m);
        }
        m
    }
    pub(super) fn count_decl(decl: &IdrisDecl, m: &mut IdrisModuleMetrics) {
        match decl {
            IdrisDecl::Func(f) => {
                m.num_functions += 1;
                m.total_clauses += f.clauses.len();
            }
            IdrisDecl::Data(_) => {
                m.num_data_types += 1;
            }
            IdrisDecl::Record(_) => {
                m.num_records += 1;
            }
            IdrisDecl::Mutual(decls) => {
                m.num_mutual_blocks += 1;
                for d in decls {
                    Self::count_decl(d, m);
                }
            }
            IdrisDecl::Pragma(_) => {
                m.num_pragmas += 1;
            }
            _ => {}
        }
    }
    /// Return a human-readable summary.
    #[allow(dead_code)]
    pub fn summary(&self) -> String {
        format!(
            "functions={} data_types={} records={} imports={} total_clauses={} mutual_blocks={} pragmas={}",
            self.num_functions, self.num_data_types, self.num_records, self.num_imports,
            self.total_clauses, self.num_mutual_blocks, self.num_pragmas,
        )
    }
}
/// A complete Idris 2 proof in elaborator-reflection style.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct IdrisProofScript {
    /// The theorem name being proved.
    pub theorem_name: String,
    /// The type being proved.
    pub goal_type: IdrisType,
    /// Sequence of tactics.
    pub tactics: Vec<IdrisTactic>,
}
impl IdrisProofScript {
    /// Create a new proof script.
    #[allow(dead_code)]
    pub fn new(theorem_name: impl Into<String>, goal_type: IdrisType) -> Self {
        IdrisProofScript {
            theorem_name: theorem_name.into(),
            goal_type,
            tactics: Vec::new(),
        }
    }
    /// Append a tactic.
    #[allow(dead_code)]
    pub fn push(&mut self, tactic: IdrisTactic) {
        self.tactics.push(tactic);
    }
    /// Emit the proof as an Idris 2 function definition using proof-by-reflection.
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        let mut out = format!(
            "{} : {}\n{} = ?{}_proof\n",
            self.theorem_name, self.goal_type, self.theorem_name, self.theorem_name
        );
        out.push_str(&format!("-- Proof sketch for {}:\n", self.theorem_name));
        for tac in &self.tactics {
            out.push_str(&format!("--   {}\n", tac));
        }
        out
    }
}
/// The quantity/multiplicity annotation on a binder in Idris 2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Quantity {
    /// `0` — erased, not available at runtime.
    Zero,
    /// `1` — linear, used exactly once.
    One,
    /// Unrestricted (the default, no annotation).
    Unrestricted,
}
/// A complete Idris 2 source module (maps to a `.idr` file).
#[derive(Debug, Clone)]
pub struct IdrisModule {
    /// Module name, e.g. `["Data", "MyList"]`.
    pub module_name: Vec<String>,
    /// Import directives.
    pub imports: Vec<IdrisImport>,
    /// Top-level declarations.
    pub declarations: Vec<IdrisDecl>,
    /// Optional module-level doc comment.
    pub doc: Option<String>,
}
impl IdrisModule {
    /// Create a new module.
    pub fn new(name: Vec<String>) -> Self {
        IdrisModule {
            module_name: name,
            imports: vec![],
            declarations: vec![],
            doc: None,
        }
    }
    /// Set the module doc comment.
    pub fn with_doc(mut self, doc: impl Into<String>) -> Self {
        self.doc = Some(doc.into());
        self
    }
    /// Add an import.
    pub fn import(&mut self, imp: IdrisImport) {
        self.imports.push(imp);
    }
    /// Add a declaration.
    pub fn add(&mut self, decl: IdrisDecl) {
        self.declarations.push(decl);
    }
    /// Add a blank line.
    pub fn blank(&mut self) {
        self.declarations.push(IdrisDecl::Blank);
    }
}
