use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};
use std::fmt;

/// Constant folding helper for JuliaExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct JuliaExtConstFolder {
    pub(crate) folds: usize,
    pub(crate) failures: usize,
    pub(crate) enabled: bool,
}

/// Julia statement.
#[derive(Debug, Clone, PartialEq)]
pub enum JuliaStmt {
    /// Expression statement: `expr`
    Expr(JuliaExpr),
    /// Local assignment: `x = expr`
    Assign(JuliaExpr, JuliaExpr),
    /// Augmented assignment: `x += expr`
    AugAssign(JuliaExpr, String, JuliaExpr),
    /// Local variable declaration: `local x::T = expr`
    Local(String, Option<JuliaType>, Option<JuliaExpr>),
    /// Global variable declaration: `global x`
    Global(String),
    /// Const declaration: `const x = expr`
    Const(String, Option<JuliaType>, JuliaExpr),
    /// Return statement: `return expr`
    Return(Option<JuliaExpr>),
    /// Break statement: `break`
    Break,
    /// Continue statement: `continue`
    Continue,
    /// If/elseif/else statement
    If {
        cond: JuliaExpr,
        then_body: Vec<JuliaStmt>,
        elseif_branches: Vec<(JuliaExpr, Vec<JuliaStmt>)>,
        else_body: Option<Vec<JuliaStmt>>,
    },
    /// For loop: `for x in iter`
    For {
        vars: Vec<String>,
        iter: JuliaExpr,
        body: Vec<JuliaStmt>,
    },
    /// While loop: `while cond`
    While {
        cond: JuliaExpr,
        body: Vec<JuliaStmt>,
    },
    /// Try/catch/finally block
    TryCatch {
        try_body: Vec<JuliaStmt>,
        catch_var: Option<String>,
        catch_body: Vec<JuliaStmt>,
        finally_body: Option<Vec<JuliaStmt>>,
    },
    /// Function definition (see JuliaFunction)
    FunctionDef(JuliaFunction),
    /// Struct definition (see JuliaStruct)
    StructDef(JuliaStruct),
    /// Abstract type definition: `abstract type Foo <: Bar end`
    AbstractTypeDef {
        name: String,
        type_params: Vec<String>,
        supertype: Option<String>,
    },
    /// Primitive type definition: `primitive type Foo 64 end`
    PrimitiveTypeDef {
        name: String,
        bits: u32,
        supertype: Option<String>,
    },
    /// Module definition
    ModuleDef(JuliaModule),
    /// Using statement: `using Module`
    Using(Vec<String>),
    /// Import statement: `import Module: sym1, sym2`
    Import(String, Vec<String>),
    /// Export statement: `export sym1, sym2`
    Export(Vec<String>),
    /// Include statement: `include("file.jl")`
    Include(String),
    /// Macro definition: `macro name(args...) body end`
    MacroDef {
        name: String,
        params: Vec<String>,
        body: Vec<JuliaStmt>,
    },
    /// Line comment: `# comment`
    Comment(String),
    /// Empty line
    Blank,
}

/// A part of an interpolated string.
#[derive(Debug, Clone, PartialEq)]
pub enum JuliaStringPart {
    /// Literal text segment
    Text(String),
    /// Interpolated expression: `$(expr)`
    Expr(JuliaExpr),
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JulPassConfig {
    pub phase: JulPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}

/// Dominator tree for JuliaExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JuliaExtDomTree {
    pub(crate) idom: Vec<Option<usize>>,
    pub(crate) children: Vec<Vec<usize>>,
    pub(crate) depth: Vec<usize>,
}

/// A Julia struct (composite type) definition.
#[derive(Debug, Clone, PartialEq)]
pub struct JuliaStruct {
    /// Struct name
    pub name: String,
    /// Type parameters: `{T, S}`
    pub type_params: Vec<String>,
    /// Supertype: `<: AbstractFoo`
    pub supertype: Option<String>,
    /// Whether this struct is mutable
    pub is_mutable: bool,
    /// Fields: (name, type, optional default)
    pub fields: Vec<(String, Option<JuliaType>, Option<JuliaExpr>)>,
    /// Inner constructors
    pub inner_constructors: Vec<JuliaFunction>,
    /// Doc string
    pub doc: Option<String>,
}

/// Julia parameter in function signatures.
#[derive(Debug, Clone, PartialEq)]
pub struct JuliaParam {
    /// Parameter name
    pub name: String,
    /// Optional type annotation
    pub ty: Option<JuliaType>,
    /// Optional default value
    pub default: Option<JuliaExpr>,
    /// Whether this is a keyword parameter
    pub is_keyword: bool,
    /// Whether this is a splat parameter (`args...`)
    pub is_splat: bool,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct JulPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JulAnalysisCache {
    pub(crate) entries: std::collections::HashMap<String, JulCacheEntry>,
    pub(crate) max_size: usize,
    pub(crate) hits: u64,
    pub(crate) misses: u64,
}

/// Configuration for JuliaExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JuliaExtPassConfig {
    pub name: String,
    pub phase: JuliaExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}

/// Newtype wrapper for Display on JuliaExpr (avoids orphan impl).
pub struct JuliaExprDisplay<'a>(pub(crate) &'a JuliaExpr);

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JulDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}

/// Liveness analysis for JuliaExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct JuliaExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}

/// A dispatch table for multiple dispatch — groups method variants of a function.
#[derive(Debug, Clone)]
pub struct DispatchTable {
    /// Function name shared by all methods
    pub name: String,
    /// Method specializations, ordered by specificity (most specific first)
    pub methods: Vec<JuliaFunction>,
}

/// Analysis cache for JuliaExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct JuliaExtCache {
    pub(crate) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(crate) cap: usize,
    pub(crate) total_hits: u64,
    pub(crate) total_misses: u64,
}

/// Julia code generation backend.
pub struct JuliaBackend {
    /// Indentation level
    pub(crate) indent: usize,
    /// Output buffer
    pub(crate) output: String,
    /// Registered dispatch tables (function name → dispatch table)
    pub(crate) dispatch_tables: HashMap<String, DispatchTable>,
}

/// A Julia module definition.
#[derive(Debug, Clone, PartialEq)]
pub struct JuliaModule {
    /// Module name
    pub name: String,
    /// Whether this is a bare module (no automatic includes)
    pub is_bare: bool,
    /// Using statements
    pub usings: Vec<Vec<String>>,
    /// Import statements: (module, symbols)
    pub imports: Vec<(String, Vec<String>)>,
    /// Export list
    pub exports: Vec<String>,
    /// Module body (functions, structs, constants, etc.)
    pub body: Vec<JuliaStmt>,
}

/// Dependency graph for JuliaExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JuliaExtDepGraph {
    pub(crate) n: usize,
    pub(crate) adj: Vec<Vec<usize>>,
    pub(crate) rev: Vec<Vec<usize>>,
    pub(crate) edge_count: usize,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JulWorklist {
    pub(crate) items: std::collections::VecDeque<u32>,
    pub(crate) in_worklist: std::collections::HashSet<u32>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum JulPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JulLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}

#[allow(dead_code)]
pub struct JulConstantFoldingHelper;

pub struct JuliaStmtDisplay<'a>(pub(crate) &'a JuliaStmt);

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JulCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}

/// Pass registry for JuliaExt.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct JuliaExtPassRegistry {
    pub(crate) configs: Vec<JuliaExtPassConfig>,
    pub(crate) stats: Vec<JuliaExtPassStats>,
}

#[allow(dead_code)]
pub struct JulPassRegistry {
    pub(crate) configs: Vec<JulPassConfig>,
    pub(crate) stats: std::collections::HashMap<String, JulPassStats>,
}

/// Pass execution phase for JuliaExt.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JuliaExtPassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}

/// Julia type representation.
#[derive(Debug, Clone, PartialEq)]
pub enum JuliaType {
    /// `Int8`
    Int8,
    /// `Int16`
    Int16,
    /// `Int32`
    Int32,
    /// `Int64`
    Int64,
    /// `Int128`
    Int128,
    /// `UInt8`
    UInt8,
    /// `UInt16`
    UInt16,
    /// `UInt32`
    UInt32,
    /// `UInt64`
    UInt64,
    /// `UInt128`
    UInt128,
    /// `Float32`
    Float32,
    /// `Float64`
    Float64,
    /// `Bool`
    Bool,
    /// `String`
    String,
    /// `Char`
    Char,
    /// `Nothing`
    Nothing,
    /// `Any`
    Any,
    /// `Vector{T}` — 1-D array
    Vector(Box<JuliaType>),
    /// `Matrix{T}` — 2-D array
    Matrix(Box<JuliaType>),
    /// `Array{T, N}` — N-dimensional array
    Array(Box<JuliaType>, u32),
    /// `Tuple{T1, T2, ...}`
    Tuple(Vec<JuliaType>),
    /// `NamedTuple{names, types}`
    NamedTuple(Vec<(String, JuliaType)>),
    /// `Union{T1, T2, ...}`
    Union(Vec<JuliaType>),
    /// Abstract type: `AbstractType`
    Abstract(String),
    /// Parametric type: `Type{T1, T2}`
    Parametric(String, Vec<JuliaType>),
    /// Type variable: `T` (used in parametric definitions)
    TypeVar(String),
    /// Function type (callable): `Function`
    Function,
    /// `Dict{K, V}`
    Dict(Box<JuliaType>, Box<JuliaType>),
    /// `Set{T}`
    Set(Box<JuliaType>),
    /// `Ref{T}` — mutable reference
    Ref(Box<JuliaType>),
    /// Named (user-defined) type: `MyStruct`
    Named(String),
}

/// A Julia function definition with multiple dispatch support.
#[derive(Debug, Clone, PartialEq)]
pub struct JuliaFunction {
    /// Function name
    pub name: String,
    /// Type parameters for parametric methods: `{T, S}`
    pub type_params: Vec<String>,
    /// Type parameter bounds: `T <: Number`
    pub type_param_bounds: Vec<(String, String)>,
    /// Positional parameters
    pub params: Vec<JuliaParam>,
    /// Keyword-only parameters (after `;`)
    pub kwargs: Vec<JuliaParam>,
    /// Return type annotation
    pub return_type: Option<JuliaType>,
    /// Function body
    pub body: Vec<JuliaStmt>,
    /// Whether this is an inner (anonymous) function
    pub is_inner: bool,
    /// Doc string
    pub doc: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JulDepGraph {
    pub(crate) nodes: Vec<u32>,
    pub(crate) edges: Vec<(u32, u32)>,
}

/// Statistics for JuliaExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct JuliaExtPassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}

/// Worklist for JuliaExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JuliaExtWorklist {
    pub(crate) items: std::collections::VecDeque<usize>,
    pub(crate) present: Vec<bool>,
}

/// Julia expression AST node.
#[derive(Debug, Clone, PartialEq)]
pub enum JuliaExpr {
    /// Integer literal: `42`
    IntLit(i64),
    /// Unsigned integer literal: `0x2a`
    UIntLit(u64),
    /// Float literal: `3.14`
    FloatLit(f64),
    /// Boolean literal: `true` / `false`
    BoolLit(bool),
    /// String literal: `"hello"`
    StringLit(String),
    /// Char literal: `'a'`
    CharLit(char),
    /// Nothing literal: `nothing`
    Nothing,
    /// Variable reference: `x`
    Var(String),
    /// Field access: `obj.field`
    Field(Box<JuliaExpr>, String),
    /// Index access: `arr[i]`
    Index(Box<JuliaExpr>, Vec<JuliaExpr>),
    /// Slice: `arr[begin:end]`
    Slice(
        Box<JuliaExpr>,
        Option<Box<JuliaExpr>>,
        Option<Box<JuliaExpr>>,
    ),
    /// Function call: `f(args...)`
    Call(Box<JuliaExpr>, Vec<JuliaExpr>),
    /// Keyword arguments call: `f(a; key=val, ...)`
    CallKw(Box<JuliaExpr>, Vec<JuliaExpr>, Vec<(String, JuliaExpr)>),
    /// Broadcasting call: `f.(args...)`
    Broadcast(Box<JuliaExpr>, Vec<JuliaExpr>),
    /// Binary operation: `a + b`
    BinOp(String, Box<JuliaExpr>, Box<JuliaExpr>),
    /// Unary operation: `-x`
    UnOp(String, Box<JuliaExpr>),
    /// Comparison chain: `a < b <= c`
    CompareChain(Vec<JuliaExpr>, Vec<String>),
    /// Array literal: `[1, 2, 3]`
    ArrayLit(Vec<JuliaExpr>),
    /// Matrix literal (rows separated by semicolons): `[1 2; 3 4]`
    MatrixLit(Vec<Vec<JuliaExpr>>),
    /// Range: `1:10` or `1:2:10`
    Range(Box<JuliaExpr>, Option<Box<JuliaExpr>>, Box<JuliaExpr>),
    /// Tuple: `(a, b, c)`
    TupleLit(Vec<JuliaExpr>),
    /// Array comprehension: `[f(x) for x in xs]`
    ArrayComp(
        Box<JuliaExpr>,
        Vec<(String, JuliaExpr)>,
        Option<Box<JuliaExpr>>,
    ),
    /// Generator expression: `(f(x) for x in xs)`
    Generator(
        Box<JuliaExpr>,
        Vec<(String, JuliaExpr)>,
        Option<Box<JuliaExpr>>,
    ),
    /// Dict comprehension: `Dict(k => v for (k,v) in pairs)`
    DictComp(Box<JuliaExpr>, Box<JuliaExpr>, Vec<(String, JuliaExpr)>),
    /// Anonymous function: `x -> x + 1`
    Lambda(Vec<JuliaParam>, Box<JuliaExpr>),
    /// Short anonymous function with `do` block is represented as Lambda
    DoBlock(Box<JuliaExpr>, Vec<String>, Vec<JuliaStmt>),
    /// Ternary: `cond ? then : else`
    Ternary(Box<JuliaExpr>, Box<JuliaExpr>, Box<JuliaExpr>),
    /// Type assertion: `x::T`
    TypeAssert(Box<JuliaExpr>, JuliaType),
    /// Type conversion: `convert(T, x)`
    Convert(JuliaType, Box<JuliaExpr>),
    /// `isa` check: `x isa T`
    IsA(Box<JuliaExpr>, JuliaType),
    /// `typeof` call: `typeof(x)`
    TypeOf(Box<JuliaExpr>),
    /// Macro call: `@macro args...`
    Macro(String, Vec<JuliaExpr>),
    /// Interpolated string: `"text $(expr) more"`
    Interpolated(Vec<JuliaStringPart>),
    /// Splat: `args...`
    Splat(Box<JuliaExpr>),
    /// Named argument pair: `key = value`
    NamedArg(String, Box<JuliaExpr>),
    /// Pair (for Dict): `k => v`
    Pair(Box<JuliaExpr>, Box<JuliaExpr>),
    /// Block expression: `begin ... end`
    Block(Vec<JuliaStmt>),
}
