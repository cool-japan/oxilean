use super::super::functions::FORTRAN_KEYWORDS;
use super::super::functions::*;
use crate::lcnf::*;
use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum FortPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}

/// Dominator tree for FortranExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FortranExtDomTree {
    pub(crate) idom: Vec<Option<usize>>,
    pub(crate) children: Vec<Vec<usize>>,
    pub(crate) depth: Vec<usize>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct FortPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}

/// Liveness analysis for FortranExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct FortranExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}

#[allow(dead_code)]
pub struct FortPassRegistry {
    pub(crate) configs: Vec<FortPassConfig>,
    pub(crate) stats: std::collections::HashMap<String, FortPassStats>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FortAnalysisCache {
    pub(crate) entries: std::collections::HashMap<String, FortCacheEntry>,
    pub(crate) max_size: usize,
    pub(crate) hits: u64,
    pub(crate) misses: u64,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FortDepGraph {
    pub(crate) nodes: Vec<u32>,
    pub(crate) edges: Vec<(u32, u32)>,
}

/// Fortran binary operators.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FortranBinOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Concat,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
    Eqv,
    Neqv,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FortPassConfig {
    pub phase: FortPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}

/// Configuration for FortranExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FortranExtPassConfig {
    pub name: String,
    pub phase: FortranExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}

#[allow(dead_code)]
pub struct FortConstantFoldingHelper;

/// Worklist for FortranExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FortranExtWorklist {
    pub(crate) items: std::collections::VecDeque<usize>,
    pub(crate) present: Vec<bool>,
}

/// A Fortran derived-type declaration.
#[derive(Debug, Clone)]
pub struct FortranDerivedType {
    pub name: String,
    pub fields: Vec<FortranDecl>,
    pub doc: Option<String>,
}

/// A Fortran PROGRAM.
#[derive(Debug, Clone)]
pub struct FortranProgram {
    pub name: String,
    pub use_modules: Vec<String>,
    pub implicit_none: bool,
    pub decls: Vec<FortranDecl>,
    pub body: Vec<FortranStmt>,
    pub contains: Vec<FortranSubprogram>,
}

/// Fortran statement AST.
#[derive(Debug, Clone, PartialEq)]
pub enum FortranStmt {
    /// `var = expr`
    Assign(FortranExpr, FortranExpr),
    /// `CALL sub(args)`
    Call(String, Vec<FortranExpr>),
    /// `RETURN`
    Return,
    /// `IF (cond) THEN ... [ELSE IF ...] [ELSE ...] END IF`
    If(Vec<(FortranExpr, Vec<FortranStmt>)>, Vec<FortranStmt>),
    /// `SELECT CASE (expr) ... END SELECT`
    SelectCase(FortranExpr, Vec<FortranCase>, Vec<FortranStmt>),
    /// `DO [label] [var = lo, hi [, step]] ... END DO`
    Do(Option<String>, Vec<FortranStmt>),
    /// Counted DO loop: `DO var = lo, hi [, step]`
    DoCount(
        String,
        FortranExpr,
        FortranExpr,
        Option<FortranExpr>,
        Vec<FortranStmt>,
    ),
    /// `DO WHILE (cond) ... END DO`
    DoWhile(FortranExpr, Vec<FortranStmt>),
    /// `EXIT [label]`
    Exit(Option<String>),
    /// `CYCLE [label]`
    Cycle(Option<String>),
    /// `STOP [code]`
    Stop(Option<FortranExpr>),
    /// `ALLOCATE(var(dims), STAT=stat)`
    Allocate(FortranExpr, Option<String>),
    /// `DEALLOCATE(var, STAT=stat)`
    Deallocate(FortranExpr, Option<String>),
    /// `NULLIFY(ptr)`
    Nullify(FortranExpr),
    /// `PRINT *, expr1, expr2, ...`
    Print(Vec<FortranExpr>),
    /// `WRITE(unit, fmt) exprs`
    Write(String, String, Vec<FortranExpr>),
    /// `READ(unit, fmt) vars`
    Read(String, String, Vec<FortranExpr>),
    /// `OPEN(unit, FILE=..., STATUS=...)`
    Open(u32, String, String),
    /// `CLOSE(unit)`
    Close(u32),
    /// `CONTINUE`
    Continue,
    /// Raw Fortran statement
    Raw(String),
    /// Block: sequential statements
    Block(Vec<FortranStmt>),
}

/// Fortran 90+ code generation backend.
pub struct FortranBackend {
    pub(crate) var_counter: u64,
    /// Map original name → mangled Fortran identifier.
    pub(crate) name_cache: HashMap<String, String>,
    /// Fortran reserved words.
    pub(crate) reserved: HashSet<String>,
    /// Indentation step (spaces).
    pub(crate) indent_width: usize,
    /// Line-continuation character position (Fortran 90 free-form: `&`).
    pub(crate) max_line_len: usize,
}

/// Constant folding helper for FortranExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct FortranExtConstFolder {
    pub(crate) folds: usize,
    pub(crate) failures: usize,
    pub(crate) enabled: bool,
}

/// Fortran unary operators.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FortranUnaryOp {
    Neg,
    Not,
    Pos,
}

/// Fortran type declaration.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FortranType {
    /// `INTEGER` (default kind, typically 4 bytes)
    FtInteger,
    /// `INTEGER(KIND=k)` — explicit kind
    FtIntegerK(u8),
    /// `REAL` (default, typically 4 bytes)
    FtReal,
    /// `REAL(KIND=8)` / `DOUBLE PRECISION`
    FtDouble,
    /// `COMPLEX`
    FtComplex,
    /// `COMPLEX(KIND=8)` — double-precision complex
    FtComplexDouble,
    /// `LOGICAL`
    FtLogical,
    /// `CHARACTER(LEN=n)` — fixed-length string
    FtCharacter(Option<usize>),
    /// `CHARACTER(LEN=*)` — assumed-length string
    FtCharacterStar,
    /// 1-D array: `TYPE, DIMENSION(n) :: var`
    FtArray(Box<FortranType>, ArrayDimension),
    /// Derived type: `TYPE(name)`
    FtDerived(String),
    /// `CLASS(name)` (polymorphic, F2003)
    FtClass(String),
    /// `CLASS(*)` — unlimited polymorphic
    FtClassStar,
    /// `TYPE(*)` — assumed-type (F2018)
    FtAssumedType,
    /// Pointer to a type: `TYPE, POINTER :: var`
    FtPointer(Box<FortranType>),
    /// Allocatable: `TYPE, ALLOCATABLE :: var`
    FtAllocatable(Box<FortranType>),
    /// Void (used only for subroutines)
    FtVoid,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FortLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}

/// A Fortran FUNCTION or SUBROUTINE.
#[derive(Debug, Clone)]
pub struct FortranSubprogram {
    pub name: String,
    /// For a FUNCTION, the return type; for SUBROUTINE, use `FtVoid`.
    pub return_type: FortranType,
    /// Dummy argument names (order matters in Fortran).
    pub dummy_args: Vec<String>,
    /// All declarations (both dummy and local).
    pub decls: Vec<FortranDecl>,
    /// Executable body.
    pub body: Vec<FortranStmt>,
    /// Whether this is a pure function.
    pub is_pure: bool,
    /// Whether this is an elemental function.
    pub is_elemental: bool,
    /// Whether this is recursive.
    pub is_recursive: bool,
    pub doc: Option<String>,
}

/// Dependency graph for FortranExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FortranExtDepGraph {
    pub(crate) n: usize,
    pub(crate) adj: Vec<Vec<usize>>,
    pub(crate) rev: Vec<Vec<usize>>,
    pub(crate) edge_count: usize,
}

/// A variable declaration inside a subprogram.
#[derive(Debug, Clone)]
pub struct FortranDecl {
    pub ty: FortranType,
    pub name: String,
    pub intent: Option<FortranIntent>,
    pub is_parameter: bool,
    pub initial_value: Option<FortranExpr>,
    pub doc: Option<String>,
}

/// A CASE block: `CASE (values) stmts`
#[derive(Debug, Clone, PartialEq)]
pub struct FortranCase {
    /// `None` means `CASE DEFAULT`
    pub values: Option<Vec<FortranExpr>>,
    pub body: Vec<FortranStmt>,
}

/// Pass registry for FortranExt.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct FortranExtPassRegistry {
    pub(crate) configs: Vec<FortranExtPassConfig>,
    pub(crate) stats: Vec<FortranExtPassStats>,
}

/// A Fortran MODULE compilation unit.
#[derive(Debug, Clone)]
pub struct FortranModule {
    pub name: String,
    /// Modules this module USEs.
    pub use_modules: Vec<String>,
    /// `IMPLICIT NONE` — always recommended.
    pub implicit_none: bool,
    /// Module-level variable declarations.
    pub module_vars: Vec<FortranDecl>,
    /// Derived types defined in this module.
    pub derived_types: Vec<FortranDerivedType>,
    /// Subprograms in the CONTAINS section.
    pub contains: Vec<FortranSubprogram>,
    pub doc: Option<String>,
}

/// Fortran INTENT attribute.
#[derive(Debug, Clone, PartialEq)]
pub enum FortranIntent {
    In,
    Out,
    InOut,
}

/// Pass execution phase for FortranExt.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FortranExtPassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}

/// Analysis cache for FortranExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct FortranExtCache {
    pub(crate) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(crate) cap: usize,
    pub(crate) total_hits: u64,
    pub(crate) total_misses: u64,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FortWorklist {
    pub(crate) items: std::collections::VecDeque<u32>,
    pub(crate) in_worklist: std::collections::HashSet<u32>,
}

/// Statistics for FortranExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct FortranExtPassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}

/// Array dimension specification.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ArrayDimension {
    /// `DIMENSION(n)` — explicit size
    Explicit(usize),
    /// `DIMENSION(:)` — deferred (allocatable/pointer)
    Deferred,
    /// `DIMENSION(*)` — assumed-size
    Assumed,
    /// Multi-dimensional: `DIMENSION(n1, n2, ...)`
    Multi(Vec<usize>),
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FortCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FortDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}

/// Fortran expression AST.
#[derive(Debug, Clone, PartialEq)]
pub enum FortranExpr {
    /// Literal value
    Lit(FortranLit),
    /// Variable reference
    Var(String),
    /// Array element access: `arr(i)`
    ArrayIndex(Box<FortranExpr>, Vec<FortranExpr>),
    /// Structure component: `obj%field`
    Component(Box<FortranExpr>, String),
    /// Binary operation: `left .OP. right`
    BinOp(Box<FortranExpr>, FortranBinOp, Box<FortranExpr>),
    /// Unary operation: `.NOT. expr` or `-expr`
    UnaryOp(FortranUnaryOp, Box<FortranExpr>),
    /// Function/intrinsic call: `FUNC(args)`
    Call(String, Vec<FortranExpr>),
    /// Array constructor: `[e1, e2, ...]`  (F2003 syntax)
    ArrayCtor(Vec<FortranExpr>),
    /// Implied DO: `(expr, var=lo, hi)` inside an array constructor
    ImpliedDo(Box<FortranExpr>, String, Box<FortranExpr>, Box<FortranExpr>),
    /// Type constructor: `TypeName(field1=v1, field2=v2)`
    TypeCtor(String, Vec<(String, FortranExpr)>),
    /// Conditional expression via `MERGE`: `MERGE(a, b, mask)`
    Merge(Box<FortranExpr>, Box<FortranExpr>, Box<FortranExpr>),
    /// Raw Fortran expression snippet
    Raw(String),
}

/// Fortran literal values.
#[derive(Debug, Clone, PartialEq)]
pub enum FortranLit {
    /// Integer literal: `42_8`
    Int(i64),
    /// Real literal: `3.14_8`
    Real(f64),
    /// Logical literal: `.TRUE.` / `.FALSE.`
    Logical(bool),
    /// Character literal: `'hello'`
    Char(String),
}
