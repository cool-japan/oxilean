//! Types for type-directed function search.

/// A parsed type expression.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TypeExpr {
    /// Type variable, e.g. `a`, `α`.
    Var(String),
    /// Type constructor, e.g. `Nat`, `Bool`, `String`.
    Con(String),
    /// Type application, e.g. `List Nat` = `App(Con("List"), Con("Nat"))`.
    App(Box<TypeExpr>, Box<TypeExpr>),
    /// Function arrow, e.g. `Nat → Bool` = `Arrow(Con("Nat"), Con("Bool"))`.
    Arrow(Box<TypeExpr>, Box<TypeExpr>),
    /// Tuple type, e.g. `(Nat, Bool)`.
    Tuple(Vec<TypeExpr>),
    /// List type sugar, e.g. `[Nat]` = `List(Con("Nat"))`.
    List(Box<TypeExpr>),
    /// Option type sugar, e.g. `Option Nat`.
    Option(Box<TypeExpr>),
}

/// A function type signature consisting of argument types and a return type.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeSignature {
    /// Argument types (may be empty for constants).
    pub args: Vec<TypeExpr>,
    /// Return type.
    pub ret: TypeExpr,
}

impl TypeSignature {
    /// Construct a signature from args and return type.
    pub fn new(args: Vec<TypeExpr>, ret: TypeExpr) -> Self {
        Self { args, ret }
    }

    /// Convert to a single `TypeExpr` using arrows.
    ///
    /// `(a, b) -> c` becomes `Arrow(a, Arrow(b, c))`.
    pub fn to_type_expr(&self) -> TypeExpr {
        let mut result = self.ret.clone();
        for arg in self.args.iter().rev() {
            result = TypeExpr::Arrow(Box::new(arg.clone()), Box::new(result));
        }
        result
    }
}

/// A function entry in the search database.
#[derive(Clone, Debug)]
pub struct FunctionEntry {
    /// Function name.
    pub name: String,
    /// Module path (e.g. `"Std.List"`).
    pub module: String,
    /// Type signature.
    pub signature: TypeSignature,
    /// Documentation string.
    pub doc: String,
}

/// The searchable database of function entries.
#[derive(Clone, Debug, Default)]
pub struct SearchDB {
    /// All registered function entries.
    pub entries: Vec<FunctionEntry>,
}

/// A search query specifying a type signature and result count limit.
#[derive(Clone, Debug)]
pub struct SearchQuery {
    /// The query type signature.
    pub signature: TypeSignature,
    /// Maximum number of results to return.
    pub max_results: usize,
}

/// How a candidate function's type matches the query type.
#[derive(Clone, Debug, PartialEq)]
pub enum MatchKind {
    /// Types are syntactically identical after normalization.
    Exact,
    /// Types are identical up to consistent renaming of type variables.
    UpToRenaming,
    /// Candidate is a specialization of the query (query is more general).
    SpecializationOf,
    /// Candidate is a generalization of the query (candidate is more general).
    GeneralizationOf,
    /// Types partially overlap (structural similarity but not full unification).
    Partial,
}

/// A single search result with relevance score.
#[derive(Clone, Debug)]
pub struct SearchResult {
    /// The matched function entry.
    pub entry: FunctionEntry,
    /// Relevance score in 0.0..=1.0 (higher = better match).
    pub score: f64,
    /// Kind of match found.
    pub match_kind: MatchKind,
}
