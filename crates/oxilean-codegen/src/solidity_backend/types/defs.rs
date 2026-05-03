use super::super::functions::SOLIDITY_RUNTIME;
use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};

/// State mutability of a function.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateMutability {
    /// May read and modify state.
    NonPayable,
    /// May receive Ether.
    Payable,
    /// Reads state but does not modify it.
    View,
    /// Does not read or modify state.
    Pure,
}

/// Dominator tree for SolExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SolExtDomTree {
    pub(crate) idom: Vec<Option<usize>>,
    pub(crate) children: Vec<Vec<usize>>,
    pub(crate) depth: Vec<usize>,
}

/// Solidity expression AST node.
#[derive(Debug, Clone, PartialEq)]
pub enum SolidityExpr {
    /// Integer literal: `42`
    IntLit(i128),
    /// Boolean literal: `true` / `false`
    BoolLit(bool),
    /// String literal: `"hello"`
    StrLit(String),
    /// Address literal: `0x1234...`
    AddressLit(String),
    /// Hex literal: `0xdeadbeef`
    HexLit(String),
    /// Variable reference: `myVar`
    Var(String),
    /// `this`
    This,
    /// `msg.sender`
    MsgSender,
    /// `msg.value`
    MsgValue,
    /// `msg.data`
    MsgData,
    /// `block.timestamp`
    BlockTimestamp,
    /// `block.number`
    BlockNumber,
    /// `block.basefee`
    BlockBasefee,
    /// `tx.origin`
    TxOrigin,
    /// `gasleft()`
    GasLeft,
    /// Field access: `expr.field`
    FieldAccess(Box<SolidityExpr>, String),
    /// Index access: `expr[index]`
    Index(Box<SolidityExpr>, Box<SolidityExpr>),
    /// Function call: `f(args...)`
    Call(Box<SolidityExpr>, Vec<SolidityExpr>),
    /// Named argument call: `f({name: val, ...})`
    NamedCall(Box<SolidityExpr>, Vec<(String, SolidityExpr)>),
    /// Type cast: `uint256(expr)`
    Cast(SolidityType, Box<SolidityExpr>),
    /// `abi.encode(args...)`
    AbiEncode(Vec<SolidityExpr>),
    /// `abi.encodePacked(args...)`
    AbiEncodePacked(Vec<SolidityExpr>),
    /// `abi.encodeWithSelector(selector, args...)`
    AbiEncodeWithSelector(Box<SolidityExpr>, Vec<SolidityExpr>),
    /// `keccak256(data)`
    Keccak256(Box<SolidityExpr>),
    /// `sha256(data)`
    Sha256(Box<SolidityExpr>),
    /// `ecrecover(hash, v, r, s)`
    Ecrecover(
        Box<SolidityExpr>,
        Box<SolidityExpr>,
        Box<SolidityExpr>,
        Box<SolidityExpr>,
    ),
    /// Binary operation: `a + b`
    BinOp(String, Box<SolidityExpr>, Box<SolidityExpr>),
    /// Unary operation: `!a`, `-a`, `~a`
    UnaryOp(String, Box<SolidityExpr>),
    /// Ternary: `cond ? then_ : else_`
    Ternary(Box<SolidityExpr>, Box<SolidityExpr>, Box<SolidityExpr>),
    /// `new T(args...)`
    New(SolidityType, Vec<SolidityExpr>),
    /// `delete expr`
    Delete(Box<SolidityExpr>),
    /// Array literal: `[a, b, c]`
    ArrayLit(Vec<SolidityExpr>),
    /// Tuple literal: `(a, b, c)`
    TupleLit(Vec<SolidityExpr>),
    /// `type(T).max` / `type(T).min`
    TypeMax(SolidityType),
    TypeMin(SolidityType),
    /// `payable(addr)`
    Payable(Box<SolidityExpr>),
}

/// A struct definition.
#[derive(Debug, Clone)]
pub struct SolidityStruct {
    pub name: String,
    pub fields: Vec<(SolidityType, String)>,
    pub doc: Option<String>,
}

/// A function parameter or return value.
#[derive(Debug, Clone)]
pub struct SolidityParam {
    /// Parameter type.
    pub ty: SolidityType,
    /// Optional data location (`memory`, `calldata`, `storage`).
    pub location: Option<String>,
    /// Parameter name (may be empty for returns).
    pub name: String,
}

/// Compilation context for a single Solidity source unit.
#[derive(Debug, Clone)]
pub struct CompilationCtx {
    /// Pragma directives.
    pub pragmas: Vec<String>,
    /// Import statements.
    pub imports: Vec<String>,
    /// Whether to include the runtime library.
    pub include_runtime: bool,
}

#[allow(dead_code)]
pub struct SolPassRegistry {
    pub(crate) configs: Vec<SolPassConfig>,
    pub(crate) stats: std::collections::HashMap<String, SolPassStats>,
}

/// Pass execution phase for SolExt.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SolExtPassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}

/// Worklist for SolExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SolExtWorklist {
    pub(crate) items: std::collections::VecDeque<usize>,
    pub(crate) present: Vec<bool>,
}

/// An enum definition.
#[derive(Debug, Clone)]
pub struct SolidityEnum {
    pub name: String,
    pub variants: Vec<String>,
    pub doc: Option<String>,
}

/// A state variable in a contract.
#[derive(Debug, Clone)]
pub struct SolidityStateVar {
    pub ty: SolidityType,
    pub name: String,
    pub visibility: Visibility,
    pub is_immutable: bool,
    pub is_constant: bool,
    pub init: Option<SolidityExpr>,
    pub doc: Option<String>,
}

/// The main Solidity code generation backend.
#[derive(Debug, Default)]
pub struct SolidityBackend {
    /// Emitted contracts (in order).
    pub contracts: Vec<SolidityContract>,
    /// Compilation context.
    pub ctx: CompilationCtx,
    /// Type alias table: `alias → canonical SolidityType`.
    pub type_aliases: HashMap<String, SolidityType>,
    /// Source buffer accumulated during emission.
    pub source: String,
}

/// A custom Solidity error definition (Solidity 0.8+).
#[derive(Debug, Clone)]
pub struct SolidityError {
    pub name: String,
    pub params: Vec<SolidityParam>,
    pub doc: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SolLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SolCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}

/// Analysis cache for SolExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct SolExtCache {
    pub(crate) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(crate) cap: usize,
    pub(crate) total_hits: u64,
    pub(crate) total_misses: u64,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SolDepGraph {
    pub(crate) nodes: Vec<u32>,
    pub(crate) edges: Vec<(u32, u32)>,
}

/// Solidity ABI type representation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SolidityType {
    /// `uint256`
    Uint256,
    /// `uint128`
    Uint128,
    /// `uint64`
    Uint64,
    /// `uint32`
    Uint32,
    /// `uint8`
    Uint8,
    /// `int256`
    Int256,
    /// `int128`
    Int128,
    /// `int64`
    Int64,
    /// `int32`
    Int32,
    /// `int8`
    Int8,
    /// `address`
    Address,
    /// `address payable`
    AddressPayable,
    /// `bool`
    Bool,
    /// `bytes`
    Bytes,
    /// `bytes32`
    Bytes32,
    /// `bytes16`
    Bytes16,
    /// `bytes4`
    Bytes4,
    /// `string`
    StringTy,
    /// `mapping(K => V)`
    Mapping(Box<SolidityType>, Box<SolidityType>),
    /// `T[]` — dynamic array
    DynArray(Box<SolidityType>),
    /// `T[N]` — fixed-size array
    FixedArray(Box<SolidityType>, usize),
    /// A named struct or enum type
    Named(String),
    /// `tuple(T0, T1, ...)` — used for ABI encoding
    Tuple(Vec<SolidityType>),
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct SolPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SolDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SolPassConfig {
    pub phase: SolPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}

/// Visibility of a state variable or function.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    Public,
    Private,
    Internal,
    External,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum SolPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SolAnalysisCache {
    pub(crate) entries: std::collections::HashMap<String, SolCacheEntry>,
    pub(crate) max_size: usize,
    pub(crate) hits: u64,
    pub(crate) misses: u64,
}

/// Configuration for SolExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SolExtPassConfig {
    pub name: String,
    pub phase: SolExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}

/// Statistics for SolExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct SolExtPassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}

/// A Solidity modifier definition.
#[derive(Debug, Clone)]
pub struct SolidityModifier {
    pub name: String,
    pub params: Vec<SolidityParam>,
    pub body: Vec<SolidityStmt>,
    pub doc: Option<String>,
}

/// Pass registry for SolExt.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct SolExtPassRegistry {
    pub(crate) configs: Vec<SolExtPassConfig>,
    pub(crate) stats: Vec<SolExtPassStats>,
}

/// Contract kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContractKind {
    Contract,
    Abstract,
    Interface,
    Library,
}

/// A Solidity function (or constructor / fallback / receive).
#[derive(Debug, Clone)]
pub struct SolidityFunction {
    /// Function name (empty for constructor/fallback/receive).
    pub name: String,
    /// Input parameters.
    pub params: Vec<SolidityParam>,
    /// Return parameters.
    pub returns: Vec<SolidityParam>,
    /// Visibility.
    pub visibility: Visibility,
    /// State mutability.
    pub mutability: StateMutability,
    /// Whether this is `virtual`.
    pub is_virtual: bool,
    /// Whether this overrides a base function.
    pub is_override: bool,
    /// List of modifier invocations: `(name, args)`.
    pub modifiers: Vec<(String, Vec<SolidityExpr>)>,
    /// Function body statements (empty = abstract/interface).
    pub body: Vec<SolidityStmt>,
    /// NatSpec dev comment.
    pub doc: Option<String>,
}

/// Solidity statement AST node.
#[derive(Debug, Clone)]
pub enum SolidityStmt {
    /// Variable declaration: `T loc name = expr;`
    VarDecl {
        ty: SolidityType,
        location: Option<String>,
        name: String,
        init: Option<SolidityExpr>,
    },
    /// Assignment: `lhs = rhs;`
    Assign(SolidityExpr, SolidityExpr),
    /// Compound assignment: `lhs += rhs;`
    CompoundAssign(String, SolidityExpr, SolidityExpr),
    /// Expression statement: `f();`
    ExprStmt(SolidityExpr),
    /// `return expr;`
    Return(Option<SolidityExpr>),
    /// `if (cond) { then } else { else_ }`
    If(SolidityExpr, Vec<SolidityStmt>, Vec<SolidityStmt>),
    /// `while (cond) { body }`
    While(SolidityExpr, Vec<SolidityStmt>),
    /// `for (init; cond; update) { body }`
    For(
        Option<Box<SolidityStmt>>,
        Option<SolidityExpr>,
        Option<Box<SolidityStmt>>,
        Vec<SolidityStmt>,
    ),
    /// `do { body } while (cond);`
    DoWhile(Vec<SolidityStmt>, SolidityExpr),
    /// `emit EventName(args...);`
    Emit(String, Vec<SolidityExpr>),
    /// `revert ErrorName(args...);`
    Revert(String, Vec<SolidityExpr>),
    /// `require(cond, msg);`
    Require(SolidityExpr, Option<String>),
    /// `assert(cond);`
    Assert(SolidityExpr),
    /// `break;`
    Break,
    /// `continue;`
    Continue,
    /// `unchecked { stmts }`
    Unchecked(Vec<SolidityStmt>),
    /// `assembly { body }`
    Assembly(String),
    /// Multi-return: `(a, b) = f();`
    MultiAssign(Vec<SolidityExpr>, SolidityExpr),
    /// Block of statements `{ ... }`
    Block(Vec<SolidityStmt>),
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SolWorklist {
    pub(crate) items: std::collections::VecDeque<u32>,
    pub(crate) in_worklist: std::collections::HashSet<u32>,
}

/// A complete Solidity contract, interface, abstract contract, or library.
#[derive(Debug, Clone)]
pub struct SolidityContract {
    pub name: String,
    pub kind: ContractKind,
    /// Inheritance list.
    pub bases: Vec<String>,
    pub structs: Vec<SolidityStruct>,
    pub enums: Vec<SolidityEnum>,
    pub events: Vec<SolidityEvent>,
    pub errors: Vec<SolidityError>,
    pub state_vars: Vec<SolidityStateVar>,
    pub modifiers: Vec<SolidityModifier>,
    pub functions: Vec<SolidityFunction>,
    /// Constructor (if present).
    pub constructor: Option<SolidityFunction>,
    /// Receive function (if present).
    pub receive: Option<SolidityFunction>,
    /// Fallback function (if present).
    pub fallback: Option<SolidityFunction>,
    /// NatSpec title/dev comment.
    pub doc: Option<String>,
}

/// A Solidity event definition.
#[derive(Debug, Clone)]
pub struct SolidityEvent {
    pub name: String,
    /// `(ty, indexed, name)`
    pub fields: Vec<(SolidityType, bool, String)>,
    pub anonymous: bool,
    pub doc: Option<String>,
}

/// Liveness analysis for SolExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct SolExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}

/// Constant folding helper for SolExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct SolExtConstFolder {
    pub(crate) folds: usize,
    pub(crate) failures: usize,
    pub(crate) enabled: bool,
}

/// Dependency graph for SolExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SolExtDepGraph {
    pub(crate) n: usize,
    pub(crate) adj: Vec<Vec<usize>>,
    pub(crate) rev: Vec<Vec<usize>>,
    pub(crate) edge_count: usize,
}

#[allow(dead_code)]
pub struct SolConstantFoldingHelper;
