//! Type definitions

use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};
use std::fmt::Write as FmtWrite;

/// A simple constant-folding optimizer for MATLAB expressions.
#[allow(dead_code)]
pub struct MatlabOptimizer {
    /// Number of rewrites performed.
    pub rewrites: usize,
}
/// MATLAB classdef property.
#[derive(Debug, Clone, PartialEq)]
pub struct MatlabProperty {
    pub name: String,
    pub ty: Option<MatlabType>,
    pub default: Option<MatlabExpr>,
    pub access: PropAccess,
    pub is_constant: bool,
    pub is_dependent: bool,
}
/// MATLAB type representation.
#[derive(Debug, Clone, PartialEq)]
pub enum MatlabType {
    /// `double` — 64-bit float (default numeric type)
    Double,
    /// `single` — 32-bit float
    Single,
    /// `int8`
    Int8,
    /// `int16`
    Int16,
    /// `int32`
    Int32,
    /// `int64`
    Int64,
    /// `uint8`
    Uint8,
    /// `uint16`
    Uint16,
    /// `uint32`
    Uint32,
    /// `uint64`
    Uint64,
    /// `logical`
    Logical,
    /// `char` — character array / string (pre-R2016b)
    Char,
    /// `string` — string array (R2016b+)
    StringArray,
    /// `cell` — cell array
    Cell,
    /// Named struct type
    StructType(String),
    /// `function_handle` — `@func`
    FunctionHandle,
    /// `sparse` — sparse matrix
    Sparse,
    /// N-D array of a base type
    Array(Box<MatlabType>, Vec<Option<usize>>),
    /// Class instance
    Class(String),
    /// Any / unspecified
    Any,
}
/// Property access level.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropAccess {
    Public,
    Protected,
    Private,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MatlabCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
#[allow(dead_code)]
pub struct MatlabConstantFoldingHelper;
/// MATLAB expression.
#[derive(Debug, Clone, PartialEq)]
pub enum MatlabExpr {
    /// Literal value
    Lit(MatlabLiteral),
    /// Variable reference: `x`
    Var(String),
    /// Matrix literal: `[1 2; 3 4]`
    MatrixLit(Vec<Vec<MatlabExpr>>),
    /// Cell array literal: `{1, 'a', true}`
    CellLit(Vec<Vec<MatlabExpr>>),
    /// Colon range: `start:step:end` or `start:end`
    ColonRange {
        start: Box<MatlabExpr>,
        step: Option<Box<MatlabExpr>>,
        end: Box<MatlabExpr>,
    },
    /// Function call: `f(a, b)`
    Call(Box<MatlabExpr>, Vec<MatlabExpr>),
    /// Indexing: `A(i, j)` or `A{i}` (cell)
    Index {
        obj: Box<MatlabExpr>,
        indices: Vec<MatlabExpr>,
        cell_index: bool,
    },
    /// Struct field access: `s.field`
    FieldAccess(Box<MatlabExpr>, String),
    /// Binary operator: `a + b`, `a .* b`, `a & b`
    BinaryOp(String, Box<MatlabExpr>, Box<MatlabExpr>),
    /// Unary operator: `-x`, `~x`, `x'`, `x.'`
    UnaryOp(String, Box<MatlabExpr>, bool),
    /// Ternary-style if expression (MATLAB doesn't have this — emitted as inline)
    IfExpr(Box<MatlabExpr>, Box<MatlabExpr>, Box<MatlabExpr>),
    /// Anonymous function: `@(x, y) x + y`
    AnonFunc(Vec<String>, Box<MatlabExpr>),
    /// End keyword (for indexing)
    End,
    /// Colon alone (`:`) for all-elements indexing
    Colon,
    /// Nargin / nargout
    Nargin,
    Nargout,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MatlabDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
/// Configuration for the MATLAB code generator.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MatlabGenConfig {
    /// Emit statement suppression (`;`) by default.
    pub suppress_output: bool,
    /// Emit `%% section` markers.
    pub emit_section_markers: bool,
    /// Target Octave compatibility (avoid newer MATLAB features).
    pub octave_compat: bool,
    /// Indent string.
    pub indent: String,
    /// Whether to emit function-end `end` keywords (MATLAB 2016b+).
    pub emit_function_end: bool,
    /// Whether to use `@(x) ...` anonymous function syntax.
    pub prefer_anon_functions: bool,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MatlabDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MatlabWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
/// Literal values in MATLAB.
#[derive(Debug, Clone, PartialEq)]
pub enum MatlabLiteral {
    /// `42` or `42.0`
    Double(f64),
    /// `42` integer literal (will cast)
    Integer(i64),
    /// `true` / `false`
    Logical(bool),
    /// `'hello'` char array
    Char(String),
    /// `"hello"` string (R2016b+)
    Str(String),
    /// `[]` empty array / matrix
    Empty,
    /// `NaN`
    NaN,
    /// `Inf` / `-Inf`
    Inf(bool),
    /// `pi`
    Pi,
    /// `eps`
    Eps,
}
/// A MATLAB function documentation annotation.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct MatlabAnnotation {
    /// Short summary line.
    pub summary: String,
    /// Long description.
    pub description: Option<String>,
    /// Input parameter descriptions `(name, description)`.
    pub inputs: Vec<(String, String)>,
    /// Output descriptions `(name, description)`.
    pub outputs: Vec<(String, String)>,
    /// Example lines.
    pub examples: Vec<String>,
    /// See-also references.
    pub see_also: Vec<String>,
}
/// A basic type-consistency checker for MATLAB expressions.
#[allow(dead_code)]
pub struct MatlabTypeChecker {
    /// Variable type environment.
    pub env: HashMap<String, MatlabType>,
    /// Type errors collected.
    pub errors: Vec<String>,
}
#[allow(dead_code)]
pub struct MatlabPassRegistry {
    pub(super) configs: Vec<MatlabPassConfig>,
    pub(super) stats: std::collections::HashMap<String, MatlabPassStats>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MatlabAnalysisCache {
    pub(crate) entries: std::collections::HashMap<String, MatlabCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MatlabPassConfig {
    pub phase: MatlabPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum MatlabPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
/// A MATLAB struct literal.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct MatlabStructLiteral {
    /// Fields.
    pub fields: Vec<MatlabStructField>,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MatlabPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
/// A MATLAB matrix literal.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct MatlabMatrix {
    /// Rows of the matrix (each row is a list of expressions).
    pub rows: Vec<Vec<MatlabExpr>>,
}
/// Statistics about a generated MATLAB module.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MatlabStats {
    /// Number of functions.
    pub num_functions: usize,
    /// Number of classes.
    pub num_classes: usize,
    /// Total number of statements.
    pub total_stmts: usize,
    /// Number of matrix operations.
    pub matrix_ops: usize,
}
/// MATLAB function parameter with optional validation.
#[derive(Debug, Clone, PartialEq)]
pub struct MatlabParam {
    pub name: String,
    pub default_value: Option<MatlabExpr>,
    pub validator: Option<MatlabType>,
}
/// MATLAB classdef definition.
#[derive(Debug, Clone, PartialEq)]
pub struct MatlabClassdef {
    /// Class name
    pub name: String,
    /// Superclass names
    pub superclasses: Vec<String>,
    /// Properties blocks (grouped by access level)
    pub properties: Vec<MatlabProperty>,
    /// Methods
    pub methods: Vec<MatlabFunction>,
    /// Events
    pub events: Vec<String>,
    /// Enumeration members (for enumeration classes)
    pub enumerations: Vec<(String, Vec<MatlabExpr>)>,
}
/// Top-level MATLAB file structure.
#[derive(Debug, Clone)]
pub struct MatlabFile {
    /// Top-level functions (first is the main function)
    pub functions: Vec<MatlabFunction>,
    /// Script statements (for script files — no functions)
    pub scripts: Vec<MatlabStmt>,
    /// Class definition (for classdef files)
    pub classdef: Option<MatlabClassdef>,
    /// File-level comment block
    pub header_comment: Option<String>,
    /// Whether this is a script file (no function wrapper)
    pub is_script: bool,
}
/// A MATLAB script (executed top-to-bottom, no function signature).
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct MatlabScript {
    /// Script file name (without `.m`).
    pub name: String,
    /// Header comment lines.
    pub header_comments: Vec<String>,
    /// Body statements.
    pub statements: Vec<MatlabStmt>,
}
/// A MATLAB plot specification.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct MatlabPlot {
    /// Figure title.
    pub title: String,
    /// X-axis label.
    pub xlabel: String,
    /// Y-axis label.
    pub ylabel: String,
    /// Data series (each is a variable name + style string).
    pub series: Vec<(String, String)>,
    /// Whether to use a grid.
    pub grid: bool,
    /// Whether to use a legend.
    pub legend: bool,
    /// Figure size `[width, height]` in points.
    pub figure_size: Option<[f64; 2]>,
}
/// A MATLAB struct field value.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct MatlabStructField {
    /// Field name.
    pub name: String,
    /// Field value.
    pub value: MatlabExpr,
}
/// MATLAB statement.
#[derive(Debug, Clone, PartialEq)]
pub enum MatlabStmt {
    /// Assignment: `[a, b] = f(x)` or `a = expr`
    Assign {
        lhs: Vec<String>,
        rhs: MatlabExpr,
        suppress: bool,
    },
    /// Complex left-hand side: `A(i,j) = expr`
    AssignIndex {
        obj: MatlabExpr,
        indices: Vec<MatlabExpr>,
        cell_index: bool,
        rhs: MatlabExpr,
        suppress: bool,
    },
    /// Struct field assignment: `s.field = expr`
    AssignField {
        obj: String,
        field: String,
        rhs: MatlabExpr,
        suppress: bool,
    },
    /// `for var = range; body; end`
    ForLoop {
        var: String,
        range: MatlabExpr,
        body: Vec<MatlabStmt>,
    },
    /// `while cond; body; end`
    WhileLoop {
        cond: MatlabExpr,
        body: Vec<MatlabStmt>,
    },
    /// `if cond; ... elseif ...; else ...; end`
    IfElseIf {
        cond: MatlabExpr,
        then_body: Vec<MatlabStmt>,
        elseif_branches: Vec<(MatlabExpr, Vec<MatlabStmt>)>,
        else_body: Option<Vec<MatlabStmt>>,
    },
    /// `switch expr; case val; ...; otherwise; ...; end`
    SwitchCase {
        expr: MatlabExpr,
        cases: Vec<(MatlabExpr, Vec<MatlabStmt>)>,
        otherwise: Option<Vec<MatlabStmt>>,
    },
    /// `return`
    Return,
    /// `break`
    Break,
    /// `continue`
    Continue,
    /// `error(msg, args...)`
    Error(MatlabExpr, Vec<MatlabExpr>),
    /// `warning(msg, args...)`
    Warning(MatlabExpr, Vec<MatlabExpr>),
    /// `disp(expr)` or `fprintf(...)`
    Disp(MatlabExpr),
    /// Function definition block
    FunctionDef(MatlabFunction),
    /// `try; ...; catch e; ...; end`
    TryCatch {
        body: Vec<MatlabStmt>,
        catch_var: Option<String>,
        catch_body: Vec<MatlabStmt>,
    },
    /// Class property validation
    ValidateProp(String, MatlabExpr),
    /// Expression statement (with or without semicolon suppression)
    Expr(MatlabExpr, bool),
    /// Comment: `% text`
    Comment(String),
    /// `global x y z`
    Global(Vec<String>),
    /// `persistent x`
    Persistent(Vec<String>),
    /// `classdef` inner block statement
    ClassdefStmt(String),
}
/// Argument validation block entry.
#[derive(Debug, Clone, PartialEq)]
pub struct MatlabArgValidation {
    pub name: String,
    pub size: Option<Vec<Option<usize>>>,
    pub class: Option<MatlabType>,
    pub validators: Vec<String>,
    pub default: Option<MatlabExpr>,
}
/// Helpers for generating MATLAB input validation statements.
#[allow(dead_code)]
pub struct MatlabValidation;
/// A MATLAB cell array literal.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct MatlabCellArray {
    /// Elements of the cell array.
    pub elements: Vec<MatlabExpr>,
}
/// Backend state for emitting MATLAB source code.
pub struct MatlabBackend {
    /// Accumulated output buffer
    pub(super) output: String,
    /// Current indentation level
    pub(super) indent: usize,
    /// Indentation string (default: two spaces)
    pub(super) indent_str: String,
    /// Known class definitions
    pub(super) classes: HashMap<String, MatlabClassdef>,
    /// Whether to emit Octave-compatible output (no `end` keywords)
    pub(super) octave_compat: bool,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MatlabLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
/// Helpers for constructing common MATLAB numeric operations.
#[allow(dead_code)]
pub struct MatlabNumericOps;
/// A MATLAB function definition.
#[derive(Debug, Clone, PartialEq)]
pub struct MatlabFunction {
    /// Function name
    pub name: String,
    /// Input parameter names
    pub inputs: Vec<MatlabParam>,
    /// Output parameter names
    pub outputs: Vec<String>,
    /// Function body
    pub body: Vec<MatlabStmt>,
    /// Whether this is a nested function
    pub is_nested: bool,
    /// Whether this is a local function (appears after main function)
    pub is_local: bool,
    /// Help text (first comment block)
    pub help_text: Option<String>,
    /// Validation blocks (arguments ... end)
    pub argument_validation: Vec<MatlabArgValidation>,
}
/// A high-level builder for MATLAB modules (collections of functions).
#[allow(dead_code)]
pub struct MatlabModuleBuilder {
    /// Module name.
    pub name: String,
    /// Functions in declaration order.
    pub functions: Vec<MatlabFunction>,
    /// Classes in declaration order.
    pub classes: Vec<MatlabClassdef>,
    /// Scripts (stand-alone statements).
    pub scripts: Vec<MatlabScript>,
    /// Global variable declarations.
    pub globals: Vec<String>,
    /// Configuration.
    pub config: MatlabGenConfig,
}
