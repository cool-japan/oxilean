use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};
use std::fmt::Write as FmtWrite;

/// An R function definition.
#[derive(Debug, Clone, PartialEq)]
pub struct RFunction {
    /// Function name (used when emitted as assignment)
    pub name: String,
    /// Formal parameters
    pub formals: Vec<RFormal>,
    /// Function body statements
    pub body: Vec<RStmt>,
    /// Whether this is a generic function (UseMethod-based)
    pub is_generic: bool,
    /// S3 method dispatch class, if any (e.g. `"numeric"`)
    pub s3_methods: Vec<(String, RFunction)>,
    /// Documentation string (Roxygen2 style)
    pub doc: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RLangDepGraph {
    pub(crate) nodes: Vec<u32>,
    pub(crate) edges: Vec<(u32, u32)>,
}

/// Which apply-family function to use.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApplyVariant {
    Sapply,
    Vapply,
    Lapply,
    Apply,
    Tapply,
    Mapply,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RLangDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}

/// Formal parameter in a function definition.
#[derive(Debug, Clone, PartialEq)]
pub struct RFormal {
    pub name: String,
    pub default: Option<RExpr>,
}

/// Top-level R file structure.
#[derive(Debug, Clone)]
pub struct RFile {
    /// Package imports (`library` calls)
    pub imports: Vec<String>,
    /// Top-level function definitions
    pub functions: Vec<RFunction>,
    /// Top-level script statements (non-function)
    pub scripts: Vec<RStmt>,
    /// Data object definitions
    pub data_objects: Vec<RDataObject>,
    /// File-level comment header
    pub header_comment: Option<String>,
    /// Shebang line (e.g., `#!/usr/bin/env Rscript`)
    pub shebang: Option<String>,
}

/// Literal values in R.
#[derive(Debug, Clone, PartialEq)]
pub enum RLiteral {
    /// `42L` or `42` integer
    Integer(i64),
    /// `3.14` numeric
    Numeric(f64),
    /// `TRUE` / `FALSE`
    Logical(bool),
    /// `"hello"` character
    Character(String),
    /// `1+2i` complex
    Complex(f64, f64),
    /// `NULL`
    Null,
    /// `NA`
    Na,
    /// `NA_integer_`
    NaInteger,
    /// `NA_real_`
    NaReal,
    /// `NA_character_`
    NaCharacter,
    /// `NA_complex_`
    NaComplex,
    /// `Inf`
    Inf,
    /// `NaN`
    NaN,
}

/// Assignment operator variant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RAssignOp {
    /// `<-` (standard)
    LeftArrow,
    /// `<<-` (global / super-assignment)
    SuperArrow,
    /// `=` (function args context, or R2 style)
    Equals,
    /// `->` (right-assign, uncommon)
    RightArrow,
}

/// R type representation.
#[derive(Debug, Clone, PartialEq)]
pub enum RType {
    /// `numeric` — double-precision float (default numeric mode)
    Numeric,
    /// `integer` — 32-bit integer (suffix `L` in literals)
    Integer,
    /// `logical` — boolean (TRUE/FALSE)
    Logical,
    /// `character` — string
    Character,
    /// `complex` — complex number
    Complex,
    /// `raw` — raw bytes
    Raw,
    /// `list` — heterogeneous list
    List,
    /// `data.frame` — tabular data
    DataFrame,
    /// `matrix` — 2-D homogeneous array
    Matrix(Box<RType>),
    /// `array` — N-dimensional homogeneous array
    Array(Box<RType>, Vec<usize>),
    /// `function` — R function
    Function,
    /// `environment` — R environment
    Environment,
    /// S3 class (informal, name-based)
    S3(String),
    /// S4 class (formal, slot-based)
    S4(String),
    /// R5 / Reference Class
    R5(String),
    /// R6 class (package R6)
    R6(String),
    /// `NULL`
    Null,
    /// `NA` (any mode)
    Na,
    /// Vector of a base type
    Vector(Box<RType>),
    /// Named list / environment
    Named(String),
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct RLangPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}

/// Dominator tree for RLangExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RLangExtDomTree {
    pub(crate) idom: Vec<Option<usize>>,
    pub(crate) children: Vec<Vec<usize>>,
    pub(crate) depth: Vec<usize>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RLangPassConfig {
    pub phase: RLangPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}

/// Named or unnamed argument in a function call.
#[derive(Debug, Clone, PartialEq)]
pub struct RArg {
    pub name: Option<String>,
    pub value: RExpr,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RLangWorklist {
    pub(crate) items: std::collections::VecDeque<u32>,
    pub(crate) in_worklist: std::collections::HashSet<u32>,
}

/// Vectorized operation descriptor — captures element-wise or reduction ops.
#[derive(Debug, Clone, PartialEq)]
pub struct VectorizedOp {
    /// The base R operator/function to apply element-wise
    pub op: String,
    /// Whether `Vectorize()` wrapper is needed
    pub needs_vectorize: bool,
    /// Whether `sapply`/`vapply` should be used
    pub use_apply_family: Option<ApplyVariant>,
    /// Whether broadcasting rules apply (recycling)
    pub uses_recycling: bool,
}

/// Configuration for RLangExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RLangExtPassConfig {
    pub name: String,
    pub phase: RLangExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}

#[allow(dead_code)]
pub struct RLangPassRegistry {
    pub(crate) configs: Vec<RLangPassConfig>,
    pub(crate) stats: std::collections::HashMap<String, RLangPassStats>,
}

/// Backend state for emitting R source code.
pub struct RBackend {
    /// Accumulated output buffer
    pub(crate) output: String,
    /// Current indentation level
    pub(crate) indent: usize,
    /// Indentation string (default: two spaces)
    pub(crate) indent_str: String,
    /// Known S4 class definitions
    pub(crate) s4_classes: HashMap<String, Vec<(String, RType)>>,
    /// Known S3 generics
    pub(crate) s3_generics: Vec<String>,
    /// Vectorized operation registry
    pub(crate) vectorized_ops: HashMap<String, VectorizedOp>,
}

/// A data object to be emitted (e.g., saved with `saveRDS` or inlined).
#[derive(Debug, Clone, PartialEq)]
pub struct RDataObject {
    /// Variable name
    pub name: String,
    /// The expression producing the data
    pub value: RExpr,
    /// Optional comment
    pub comment: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RLangLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}

#[allow(dead_code)]
pub struct RLangConstantFoldingHelper;

/// Pass execution phase for RLangExt.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RLangExtPassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}

/// Liveness analysis for RLangExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct RLangExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}

/// Pass registry for RLangExt.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct RLangExtPassRegistry {
    pub(crate) configs: Vec<RLangExtPassConfig>,
    pub(crate) stats: Vec<RLangExtPassStats>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RLangAnalysisCache {
    pub(crate) entries: std::collections::HashMap<String, RLangCacheEntry>,
    pub(crate) max_size: usize,
    pub(crate) hits: u64,
    pub(crate) misses: u64,
}

/// R statement.
#[derive(Debug, Clone, PartialEq)]
pub enum RStmt {
    /// Assignment: `x <- expr` or `x <<- expr` or `x = expr`
    Assign(RAssignOp, String, RExpr),
    /// Complex left-hand side assignment: `x$field <- expr`
    AssignLhs(RAssignOp, RExpr, RExpr),
    /// `for (var in seq) { body }`
    ForLoop {
        var: String,
        seq: RExpr,
        body: Vec<RStmt>,
    },
    /// `while (cond) { body }`
    WhileLoop { cond: RExpr, body: Vec<RStmt> },
    /// `repeat { body }`
    Repeat(Vec<RStmt>),
    /// `if (cond) { then } else if ... else { else }`
    IfElse {
        cond: RExpr,
        then_body: Vec<RStmt>,
        else_if_branches: Vec<(RExpr, Vec<RStmt>)>,
        else_body: Option<Vec<RStmt>>,
    },
    /// `return(expr)`
    Return(Option<RExpr>),
    /// `next` (continue)
    Next,
    /// `break`
    Break,
    /// Function definition: `name <- function(formals) { body }`
    FunctionDef(RFunction),
    /// `library(pkg)` or `require(pkg)`
    Library { pkg: String, use_require: bool },
    /// `source("file.R")`
    Source(String),
    /// Expression statement
    Expr(RExpr),
    /// Comment: `# text`
    Comment(String),
    /// `stopifnot(...)` assertion
    Stopifnot(Vec<RExpr>),
    /// `tryCatch({ body }, error = function(e) { handler })`
    TryCatch {
        body: Vec<RStmt>,
        handlers: Vec<(String, RFormal, Vec<RStmt>)>,
        finally: Option<Vec<RStmt>>,
    },
    /// S4 method definition: `setMethod(generic, signature, function)`
    SetMethod {
        generic: String,
        signature: Vec<String>,
        fun: RFunction,
    },
    /// S4 class definition
    SetClass {
        class: String,
        contains: Option<String>,
        slots: Vec<(String, RType)>,
    },
}

/// Analysis cache for RLangExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct RLangExtCache {
    pub(crate) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(crate) cap: usize,
    pub(crate) total_hits: u64,
    pub(crate) total_misses: u64,
}

/// Statistics for RLangExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct RLangExtPassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}

/// Worklist for RLangExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RLangExtWorklist {
    pub(crate) items: std::collections::VecDeque<usize>,
    pub(crate) present: Vec<bool>,
}

/// Constant folding helper for RLangExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct RLangExtConstFolder {
    pub(crate) folds: usize,
    pub(crate) failures: usize,
    pub(crate) enabled: bool,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum RLangPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}

/// Dependency graph for RLangExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RLangExtDepGraph {
    pub(crate) n: usize,
    pub(crate) adj: Vec<Vec<usize>>,
    pub(crate) rev: Vec<Vec<usize>>,
    pub(crate) edge_count: usize,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RLangCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}

/// R expression.
#[derive(Debug, Clone, PartialEq)]
pub enum RExpr {
    /// Literal value
    Lit(RLiteral),
    /// Variable reference: `x`
    Var(String),
    /// Function call: `f(a, b, named = c)`
    Call(Box<RExpr>, Vec<RArg>),
    /// Infix operator: `a + b`, `a & b`, `a %in% b`
    InfixOp(String, Box<RExpr>, Box<RExpr>),
    /// Unary operator: `!x`, `-x`, `+x`
    UnaryOp(String, Box<RExpr>),
    /// Single-bracket index: `x[i]`
    IndexSingle(Box<RExpr>, Vec<RExpr>),
    /// Double-bracket index: `x[[i]]`
    IndexDouble(Box<RExpr>, Box<RExpr>),
    /// Dollar-sign access: `x$field`
    DollarAccess(Box<RExpr>, String),
    /// At-sign access: `x@slot` (S4)
    AtAccess(Box<RExpr>, String),
    /// Formula: `y ~ x + z`
    Formula(Option<Box<RExpr>>, Box<RExpr>),
    /// If-else expression: `if (cond) a else b`
    IfElse(Box<RExpr>, Box<RExpr>, Option<Box<RExpr>>),
    /// Anonymous function (lambda): `function(x, y) x + y`
    Lambda(Vec<RFormal>, Box<RExpr>),
    /// Native pipe: `x |> f()`
    Pipe(Box<RExpr>, Box<RExpr>),
    /// Magrittr pipe: `x %>% f()`
    MagrittrPipe(Box<RExpr>, Box<RExpr>),
    /// Sequence: `1:10`
    Seq(Box<RExpr>, Box<RExpr>),
    /// c() vector constructor
    CVec(Vec<RExpr>),
    /// list() constructor
    ListExpr(Vec<RArg>),
    /// Block expression: `{ stmt; ...; expr }`
    Block(Vec<RStmt>),
    /// Namespace access: `pkg::func`
    Namespace(String, String),
    /// Double-colon access: `pkg:::func` (internal)
    NamespaceInternal(String, String),
}
