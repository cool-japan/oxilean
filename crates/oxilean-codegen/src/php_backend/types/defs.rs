use super::super::functions::*;
use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};

/// PHP function/method parameter.
#[derive(Debug, Clone, PartialEq)]
pub struct PHPParam {
    /// Parameter name (without `$`)
    pub name: std::string::String,
    /// Optional type hint
    pub ty: Option<PHPType>,
    /// Optional default value
    pub default: Option<PHPExpr>,
    /// Whether this is a reference parameter (`&$name`)
    pub by_ref: bool,
    /// Whether this is a variadic parameter (`...$name`)
    pub variadic: bool,
    /// Whether this is a promoted constructor property (PHP 8.0+)
    pub promoted: Option<PHPVisibility>,
}

/// A complete PHP script / file.
#[derive(Debug, Clone)]
pub struct PHPScript {
    /// Whether to include `declare(strict_types=1)`
    pub strict_types: bool,
    /// Namespace (optional)
    pub namespace: Option<std::string::String>,
    /// `use` import statements
    pub uses: Vec<(std::string::String, Option<std::string::String>)>,
    /// Top-level functions
    pub functions: Vec<PHPFunction>,
    /// Top-level classes
    pub classes: Vec<PHPClass>,
    /// Interfaces
    pub interfaces: Vec<PHPInterface>,
    /// Traits
    pub traits: Vec<PHPTrait>,
    /// Enums
    pub enums: Vec<PHPEnum>,
    /// Top-level statements (main body)
    pub main: Vec<std::string::String>,
}

/// Statistics for PHPExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct PHPExtPassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}

/// Worklist for PHPExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PHPExtWorklist {
    pub(crate) items: std::collections::VecDeque<usize>,
    pub(crate) present: Vec<bool>,
}

/// PHP trait declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct PHPTrait {
    /// Trait name
    pub name: std::string::String,
    /// Properties
    pub properties: Vec<PHPProperty>,
    /// Methods
    pub methods: Vec<PHPFunction>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PHPAnalysisCache {
    pub(crate) entries: std::collections::HashMap<String, PHPCacheEntry>,
    pub(crate) max_size: usize,
    pub(crate) hits: u64,
    pub(crate) misses: u64,
}

/// PHP class declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct PHPClass {
    /// Class name
    pub name: std::string::String,
    /// Optional parent class
    pub parent: Option<std::string::String>,
    /// Implemented interfaces
    pub interfaces: Vec<std::string::String>,
    /// Used traits
    pub traits: Vec<std::string::String>,
    /// Whether this is abstract
    pub is_abstract: bool,
    /// Whether this is final
    pub is_final: bool,
    /// Whether this is readonly (PHP 8.2+)
    pub is_readonly: bool,
    /// Properties
    pub properties: Vec<PHPProperty>,
    /// Methods
    pub methods: Vec<PHPFunction>,
    /// Class constants
    pub constants: Vec<(std::string::String, PHPType, std::string::String)>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PHPWorklist {
    pub(crate) items: std::collections::VecDeque<u32>,
    pub(crate) in_worklist: std::collections::HashSet<u32>,
}

#[allow(dead_code)]
pub struct PHPPassRegistry {
    pub(crate) configs: Vec<PHPPassConfig>,
    pub(crate) stats: std::collections::HashMap<String, PHPPassStats>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PHPLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}

/// Pass registry for PHPExt.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct PHPExtPassRegistry {
    pub(crate) configs: Vec<PHPExtPassConfig>,
    pub(crate) stats: Vec<PHPExtPassStats>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PHPDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}

/// Liveness analysis for PHPExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct PHPExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}

/// Constant folding helper for PHPExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct PHPExtConstFolder {
    pub(crate) folds: usize,
    pub(crate) failures: usize,
    pub(crate) enabled: bool,
}

/// A PHP top-level function or method.
#[derive(Debug, Clone, PartialEq)]
pub struct PHPFunction {
    /// Function name
    pub name: std::string::String,
    /// Parameters
    pub params: Vec<PHPParam>,
    /// Optional return type
    pub return_type: Option<PHPType>,
    /// Body lines (raw PHP code)
    pub body: Vec<std::string::String>,
    /// Whether this is a static method
    pub is_static: bool,
    /// Whether this is abstract
    pub is_abstract: bool,
    /// Visibility (for methods)
    pub visibility: Option<PHPVisibility>,
    /// Docblock comment
    pub doc_comment: Option<std::string::String>,
}

/// PHP member visibility modifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PHPVisibility {
    Public,
    Protected,
    Private,
}

/// PHP 8.1 backed enum variant.
#[derive(Debug, Clone, PartialEq)]
pub struct PHPEnumCase {
    /// Variant name
    pub name: std::string::String,
    /// Backing value for backed enums
    pub value: Option<std::string::String>,
}

/// PHP type for type hints and declarations.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PHPType {
    /// `int`
    Int,
    /// `float`
    Float,
    /// `string`
    String,
    /// `bool`
    Bool,
    /// `array`
    Array,
    /// `null`
    Null,
    /// `mixed`
    Mixed,
    /// `callable`
    Callable,
    /// `void`
    Void,
    /// `never`
    Never,
    /// `object`
    Object,
    /// `iterable`
    Iterable,
    /// `?T` (nullable type)
    Nullable(Box<PHPType>),
    /// `T1|T2|...` (union type)
    Union(Vec<PHPType>),
    /// `T1&T2&...` (intersection type, PHP 8.1+)
    Intersection(Vec<PHPType>),
    /// Named class/interface type
    Named(std::string::String),
    /// `self`
    Self_,
    /// `static`
    Static,
    /// `parent`
    Parent,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PHPCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}

/// Pass execution phase for PHPExt.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PHPExtPassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum PHPPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}

/// Configuration for PHPExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PHPExtPassConfig {
    pub name: String,
    pub phase: PHPExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}

/// Analysis cache for PHPExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct PHPExtCache {
    pub(crate) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(crate) cap: usize,
    pub(crate) total_hits: u64,
    pub(crate) total_misses: u64,
}

/// PHP 8.x code generation backend for OxiLean.
pub struct PHPBackend {
    /// Indent string (default: 4 spaces)
    pub(crate) indent: std::string::String,
    /// Name mangling table
    pub(crate) mangle_cache: HashMap<std::string::String, std::string::String>,
    /// Whether to emit docblocks
    pub(crate) emit_docs: bool,
}

/// Dependency graph for PHPExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PHPExtDepGraph {
    pub(crate) n: usize,
    pub(crate) adj: Vec<Vec<usize>>,
    pub(crate) rev: Vec<Vec<usize>>,
    pub(crate) edge_count: usize,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct PHPPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}

/// Dominator tree for PHPExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PHPExtDomTree {
    pub(crate) idom: Vec<Option<usize>>,
    pub(crate) children: Vec<Vec<usize>>,
    pub(crate) depth: Vec<usize>,
}

/// PHP 8.1 enum declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct PHPEnum {
    /// Enum name
    pub name: std::string::String,
    /// Backing type (int or string), `None` for pure enums
    pub backing_type: Option<PHPType>,
    /// Cases
    pub cases: Vec<PHPEnumCase>,
    /// Implemented interfaces
    pub implements: Vec<std::string::String>,
    /// Methods
    pub methods: Vec<PHPFunction>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PHPPassConfig {
    pub phase: PHPPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}

#[allow(dead_code)]
pub struct PHPConstantFoldingHelper;

/// PHP expression AST node.
#[derive(Debug, Clone, PartialEq)]
pub enum PHPExpr {
    /// Literal value (int, float, string, bool, null)
    Lit(std::string::String),
    /// Variable reference: `$name`
    Var(std::string::String),
    /// Binary operation: `lhs op rhs`
    BinOp(Box<PHPExpr>, std::string::String, Box<PHPExpr>),
    /// Unary operation: `op operand`
    UnaryOp(std::string::String, Box<PHPExpr>),
    /// Function/method call: `name(args...)`
    FuncCall(std::string::String, Vec<PHPExpr>),
    /// Array literal: `[expr, ...]`
    ArrayLit(Vec<PHPExpr>),
    /// Associative array literal: `[key => val, ...]`
    ArrayMap(Vec<(PHPExpr, PHPExpr)>),
    /// Object instantiation: `new ClassName(args...)`
    New(std::string::String, Vec<PHPExpr>),
    /// Property access: `$obj->prop`
    Arrow(Box<PHPExpr>, std::string::String),
    /// Null-safe property access: `$obj?->prop`
    NullSafe(Box<PHPExpr>, std::string::String),
    /// Static property/method access: `Class::member`
    StaticAccess(std::string::String, std::string::String),
    /// Array index: `$arr[idx]`
    Index(Box<PHPExpr>, Box<PHPExpr>),
    /// Ternary: `$cond ? $then : $else`
    Ternary(Box<PHPExpr>, Box<PHPExpr>, Box<PHPExpr>),
    /// Null coalescing: `$a ?? $b`
    NullCoalesce(Box<PHPExpr>, Box<PHPExpr>),
    /// Closure / anonymous function
    Closure {
        params: Vec<PHPParam>,
        use_vars: Vec<std::string::String>,
        return_type: Option<PHPType>,
        body: Vec<std::string::String>,
    },
    /// Arrow function (PHP 7.4+): `fn($x) => expr`
    ArrowFn {
        params: Vec<PHPParam>,
        return_type: Option<PHPType>,
        body: Box<PHPExpr>,
    },
    /// Match expression (PHP 8.0+)
    Match {
        subject: Box<PHPExpr>,
        arms: Vec<(PHPExpr, PHPExpr)>,
        default: Option<Box<PHPExpr>>,
    },
    /// Named argument: `name: value`
    NamedArg(std::string::String, Box<PHPExpr>),
    /// Spread operator: `...$arr`
    Spread(Box<PHPExpr>),
    /// Cast: `(type)$expr`
    Cast(std::string::String, Box<PHPExpr>),
    /// `isset($var)`
    Isset(Box<PHPExpr>),
    /// `empty($var)`
    Empty(Box<PHPExpr>),
}

/// A PHP class property.
#[derive(Debug, Clone, PartialEq)]
pub struct PHPProperty {
    /// Property name (without `$`)
    pub name: std::string::String,
    /// Optional type hint
    pub ty: Option<PHPType>,
    /// Visibility
    pub visibility: PHPVisibility,
    /// Whether this is static
    pub is_static: bool,
    /// Whether this is readonly (PHP 8.1+)
    pub readonly: bool,
    /// Optional default value expression (as string)
    pub default: Option<std::string::String>,
}

/// A PHP namespace block.
#[derive(Debug, Clone, PartialEq)]
pub struct PHPNamespace {
    /// Namespace path (e.g. `OxiLean\\Runtime`)
    pub path: std::string::String,
    /// `use` import statements
    pub uses: Vec<(std::string::String, Option<std::string::String>)>,
    /// Functions in this namespace
    pub functions: Vec<PHPFunction>,
    /// Classes in this namespace
    pub classes: Vec<PHPClass>,
    /// Interfaces in this namespace
    pub interfaces: Vec<PHPInterface>,
    /// Traits in this namespace
    pub traits: Vec<PHPTrait>,
    /// Enums in this namespace
    pub enums: Vec<PHPEnum>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PHPDepGraph {
    pub(crate) nodes: Vec<u32>,
    pub(crate) edges: Vec<(u32, u32)>,
}

/// PHP interface declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct PHPInterface {
    /// Interface name
    pub name: std::string::String,
    /// Interfaces this extends
    pub extends: Vec<std::string::String>,
    /// Method signatures (abstract methods)
    pub methods: Vec<PHPFunction>,
    /// Constants
    pub constants: Vec<(std::string::String, std::string::String)>,
}
