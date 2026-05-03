//! Type definitions

use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};

/// EVM optimizer pipeline
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct EvmOptPipeline {
    pub passes: Vec<EvmOptPass>,
    pub enabled: bool,
    pub runs: u32,
}
/// EVM diagnostic
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvmDiagLevel {
    Info,
    Warning,
    Error,
}
/// EVM Yul object
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct YulObject {
    pub name: String,
    pub code: Vec<YulStmt>,
    pub functions: Vec<YulFunction>,
    pub sub_objects: Vec<YulObject>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EvmDiag {
    pub level: EvmDiagLevel,
    pub message: String,
    pub location: Option<String>,
}
/// A basic block of EVM instructions with a named label.
///
/// Each block starts with a JUMPDEST if it is a jump target.
#[derive(Debug, Clone)]
pub struct EvmBasicBlock {
    /// Symbolic label for this block (used in assembly output).
    pub label: String,
    /// The instructions in this block.
    pub instructions: Vec<EvmInstruction>,
    /// Whether this block needs a JUMPDEST at the start.
    pub is_jump_target: bool,
}
/// Describes the persistent storage layout of a contract.
///
/// Maps variable names to their storage slot (256-bit word index).
#[derive(Debug, Clone, Default)]
pub struct StorageLayout {
    /// Maps variable name → storage slot number.
    pub slots: HashMap<String, u64>,
    /// Next available slot index.
    pub(super) next_slot: u64,
}
/// All EVM opcodes as defined in the Yellow Paper and subsequent EIPs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EvmOpcode {
    /// Halts execution.
    Stop,
    /// Integer addition.
    Add,
    /// Integer multiplication.
    Mul,
    /// Integer subtraction.
    Sub,
    /// Integer division (unsigned).
    Div,
    /// Signed integer division.
    Sdiv,
    /// Modulo remainder (unsigned).
    Mod,
    /// Modulo remainder (signed).
    Smod,
    /// Addition modulo N.
    Addmod,
    /// Multiplication modulo N.
    Mulmod,
    /// Exponentiation.
    Exp,
    /// Sign extend.
    Signextend,
    /// Less-than comparison (unsigned).
    Lt,
    /// Greater-than comparison (unsigned).
    Gt,
    /// Less-than comparison (signed).
    Slt,
    /// Greater-than comparison (signed).
    Sgt,
    /// Equality comparison.
    Eq,
    /// Is-zero check.
    Iszero,
    /// Bitwise AND.
    And,
    /// Bitwise OR.
    Or,
    /// Bitwise XOR.
    Xor,
    /// Bitwise NOT.
    Not,
    /// Retrieve single byte from word.
    Byte,
    /// Left shift.
    Shl,
    /// Logical right shift.
    Shr,
    /// Arithmetic right shift.
    Sar,
    /// Compute Keccak-256 hash.
    Sha3,
    /// Address of currently executing account.
    Address,
    /// Balance of given account.
    Balance,
    /// Address of execution originator.
    Origin,
    /// Address of caller.
    Caller,
    /// Value deposited by caller.
    Callvalue,
    /// Input data of current environment.
    Calldataload,
    /// Size of input data.
    Calldatasize,
    /// Copy input data to memory.
    Calldatacopy,
    /// Size of code at given address.
    Codesize,
    /// Copy code to memory.
    Codecopy,
    /// Current gas price.
    Gasprice,
    /// Size of external code at given address.
    Extcodesize,
    /// Copy external code to memory.
    Extcodecopy,
    /// Size of return data from last call.
    Returndatasize,
    /// Copy return data to memory.
    Returndatacopy,
    /// Hash of external code at given address.
    Extcodehash,
    /// Hash of a recent block.
    Blockhash,
    /// Current block's beneficiary (coinbase).
    Coinbase,
    /// Current block's timestamp.
    Timestamp,
    /// Current block number.
    Number,
    /// Difficulty / prevrandao of current block.
    Prevrandao,
    /// Gas limit of current block.
    Gaslimit,
    /// Chain ID.
    Chainid,
    /// Balance of currently executing account.
    Selfbalance,
    /// Base fee of current block.
    Basefee,
    /// Blob base fee (EIP-7516).
    Blobbasefee,
    /// Remove item from stack.
    Pop,
    /// Load word from memory.
    Mload,
    /// Save word to memory.
    Mstore,
    /// Save byte to memory.
    Mstore8,
    /// Load word from storage.
    Sload,
    /// Save word to storage.
    Sstore,
    /// Alter program counter.
    Jump,
    /// Conditionally alter program counter.
    Jumpi,
    /// Value of program counter before current instruction.
    Pc,
    /// Size of active memory in bytes.
    Msize,
    /// Amount of available gas.
    Gas,
    /// Mark a valid jump destination.
    Jumpdest,
    /// Load word from transient storage.
    Tload,
    /// Save word to transient storage.
    Tstore,
    /// Copy memory areas.
    Mcopy,
    /// Push 1 byte onto stack.
    Push1,
    /// Push 2 bytes onto stack.
    Push2,
    /// Push 3 bytes onto stack.
    Push3,
    /// Push 4 bytes onto stack.
    Push4,
    /// Push 5 bytes onto stack.
    Push5,
    /// Push 6 bytes onto stack.
    Push6,
    /// Push 7 bytes onto stack.
    Push7,
    /// Push 8 bytes onto stack.
    Push8,
    /// Push 9 bytes onto stack.
    Push9,
    /// Push 10 bytes onto stack.
    Push10,
    /// Push 11 bytes onto stack.
    Push11,
    /// Push 12 bytes onto stack.
    Push12,
    /// Push 13 bytes onto stack.
    Push13,
    /// Push 14 bytes onto stack.
    Push14,
    /// Push 15 bytes onto stack.
    Push15,
    /// Push 16 bytes onto stack.
    Push16,
    /// Push 17 bytes onto stack.
    Push17,
    /// Push 18 bytes onto stack.
    Push18,
    /// Push 19 bytes onto stack.
    Push19,
    /// Push 20 bytes onto stack.
    Push20,
    /// Push 21 bytes onto stack.
    Push21,
    /// Push 22 bytes onto stack.
    Push22,
    /// Push 23 bytes onto stack.
    Push23,
    /// Push 24 bytes onto stack.
    Push24,
    /// Push 25 bytes onto stack.
    Push25,
    /// Push 26 bytes onto stack.
    Push26,
    /// Push 27 bytes onto stack.
    Push27,
    /// Push 28 bytes onto stack.
    Push28,
    /// Push 29 bytes onto stack.
    Push29,
    /// Push 30 bytes onto stack.
    Push30,
    /// Push 31 bytes onto stack.
    Push31,
    /// Push 32 bytes onto stack.
    Push32,
    /// Duplicate 1st stack item.
    Dup1,
    /// Duplicate 2nd stack item.
    Dup2,
    /// Duplicate 3rd stack item.
    Dup3,
    /// Duplicate 4th stack item.
    Dup4,
    /// Duplicate 5th stack item.
    Dup5,
    /// Duplicate 6th stack item.
    Dup6,
    /// Duplicate 7th stack item.
    Dup7,
    /// Duplicate 8th stack item.
    Dup8,
    /// Duplicate 9th stack item.
    Dup9,
    /// Duplicate 10th stack item.
    Dup10,
    /// Duplicate 11th stack item.
    Dup11,
    /// Duplicate 12th stack item.
    Dup12,
    /// Duplicate 13th stack item.
    Dup13,
    /// Duplicate 14th stack item.
    Dup14,
    /// Duplicate 15th stack item.
    Dup15,
    /// Duplicate 16th stack item.
    Dup16,
    /// Exchange 1st and 2nd stack items.
    Swap1,
    /// Exchange 1st and 3rd stack items.
    Swap2,
    /// Exchange 1st and 4th stack items.
    Swap3,
    /// Exchange 1st and 5th stack items.
    Swap4,
    /// Exchange 1st and 6th stack items.
    Swap5,
    /// Exchange 1st and 7th stack items.
    Swap6,
    /// Exchange 1st and 8th stack items.
    Swap7,
    /// Exchange 1st and 9th stack items.
    Swap8,
    /// Exchange 1st and 10th stack items.
    Swap9,
    /// Exchange 1st and 11th stack items.
    Swap10,
    /// Exchange 1st and 12th stack items.
    Swap11,
    /// Exchange 1st and 13th stack items.
    Swap12,
    /// Exchange 1st and 14th stack items.
    Swap13,
    /// Exchange 1st and 15th stack items.
    Swap14,
    /// Exchange 1st and 16th stack items.
    Swap15,
    /// Exchange 1st and 17th stack items.
    Swap16,
    /// Append log record with no topics.
    Log0,
    /// Append log record with 1 topic.
    Log1,
    /// Append log record with 2 topics.
    Log2,
    /// Append log record with 3 topics.
    Log3,
    /// Append log record with 4 topics.
    Log4,
    /// Create a new account with associated code.
    Create,
    /// Message-call into an account.
    Call,
    /// Message-call with another account's code.
    Callcode,
    /// Halt, returning output data.
    Return,
    /// Message-call into this account with caller's code.
    Delegatecall,
    /// Create a new account with deterministic address.
    Create2,
    /// Static message-call into an account.
    Staticcall,
    /// Halt execution reverting state changes.
    Revert,
    /// Designated invalid instruction.
    Invalid,
    /// Halt execution and register account for deletion.
    Selfdestruct,
}
#[allow(dead_code)]
pub struct EVMPassRegistry {
    pub(super) configs: Vec<EVMPassConfig>,
    pub(super) stats: std::collections::HashMap<String, EVMPassStats>,
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum EVMPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EVMWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EVMLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
/// EVM code size analyzer
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct EvmCodeSizeStats {
    pub bytecode_size: usize,
    pub deploy_bytecode_size: usize,
    pub constructor_size: usize,
    pub function_sizes: std::collections::HashMap<String, usize>,
}
/// EVM id generator
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct EvmExtIdGen {
    pub(super) counter: u64,
    pub(super) prefix: String,
}
/// EVM storage layout
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EvmStorageSlot {
    pub slot: u64,
    pub offset: u8,
    pub var_name: String,
    pub var_type: EvmAbiType,
}
/// EVM code statistics
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct EvmCodeStats {
    pub functions: usize,
    pub events: usize,
    pub modifiers: usize,
    pub storage_vars: usize,
    pub bytecode_size: usize,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EVMAnalysisCache {
    pub(crate) entries: std::collections::HashMap<String, EVMCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
/// EVM source buffer
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct EvmExtSourceBuffer {
    pub sections: Vec<(String, String)>,
    pub current: String,
    pub indent: usize,
}
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct EvmDiagSink {
    pub diags: Vec<EvmDiag>,
}
/// A full EVM smart contract.
///
/// Contains the constructor code, runtime functions, and storage layout.
#[derive(Debug, Clone)]
pub struct EvmContract {
    /// Contract name.
    pub name: String,
    /// Runtime functions (public entry points).
    pub functions: Vec<EvmFunction>,
    /// Storage variable layout.
    pub storage_layout: StorageLayout,
    /// Constructor bytecode (deployed as init code).
    pub constructor_code: Vec<EvmInstruction>,
    /// Compiler metadata comments.
    pub metadata: HashMap<String, String>,
}
/// EVM pass profiler
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct EvmExtProfiler {
    pub timings: Vec<(String, u64)>,
}
/// EVM Yul IR expression
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum YulExpr {
    Literal(u64),
    Variable(String),
    FunctionCall(String, Vec<YulExpr>),
}
/// EVM Yul function definition
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct YulFunction {
    pub name: String,
    pub params: Vec<String>,
    pub returns: Vec<String>,
    pub body: Vec<YulStmt>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EVMCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
/// EVM opcode descriptor
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EvmOpcodeDesc {
    pub name: String,
    pub opcode: u8,
    pub stack_in: u8,
    pub stack_out: u8,
    pub gas: u64,
    pub category: EvmOpcodeCategory,
    pub description: String,
}
#[allow(dead_code)]
pub struct EVMConstantFoldingHelper;
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EVMDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
/// EVM feature flags
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct EvmFeatureFlags {
    pub shanghai: bool,
    pub cancun: bool,
    pub prague: bool,
    pub support_transient_storage: bool,
    pub support_push0: bool,
}
/// EVM selector (function selector for ABI)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EvmSelector {
    pub signature: String,
    pub selector: [u8; 4],
}
/// EVM ABI function descriptor
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EvmAbiFunction {
    pub name: String,
    pub inputs: Vec<(String, EvmAbiType)>,
    pub outputs: Vec<(String, EvmAbiType)>,
    pub state_mutability: String,
    pub is_payable: bool,
    pub is_view: bool,
    pub is_pure: bool,
}
/// EVM bytecode code generation backend.
///
/// Converts an `EvmContract` into:
/// - Raw binary bytecode (`Vec<u8>`)
/// - Hex string representation
/// - Human-readable assembly text
#[derive(Debug, Default)]
pub struct EvmBackend {
    /// Label-to-offset mapping built during encoding.
    pub(super) label_offsets: HashMap<String, usize>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EVMPassConfig {
    pub phase: EVMPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
/// A single EVM instruction, consisting of an opcode and optional immediate data.
///
/// For PUSH1..PUSH32 opcodes the `data` field holds the bytes to push.
/// For all other opcodes `data` is `None`.
#[derive(Debug, Clone, PartialEq)]
pub struct EvmInstruction {
    /// The opcode for this instruction.
    pub opcode: EvmOpcode,
    /// Optional immediate data bytes (used by PUSH instructions).
    pub data: Option<Vec<u8>>,
    /// Optional human-readable comment for assembly output.
    pub comment: Option<String>,
}
/// EVM gas cost table
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EvmGasTable {
    pub stop: u64,
    pub add: u64,
    pub mul: u64,
    pub sub: u64,
    pub div: u64,
    pub sdiv: u64,
    pub mload: u64,
    pub mstore: u64,
    pub sload: u64,
    pub sstore_set: u64,
    pub sstore_clear: u64,
    pub call: u64,
    pub create: u64,
    pub sha3: u64,
    pub sha3_word: u64,
    pub log: u64,
    pub log_topic: u64,
    pub log_byte: u64,
}
/// EVM stack depth tracker
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct EvmStackDepth {
    pub current: i32,
    pub max: i32,
    pub min: i32,
}
/// EVM emit stats
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct EvmExtEmitStats {
    pub bytes_written: usize,
    pub items_emitted: usize,
    pub errors: usize,
    pub warnings: usize,
    pub functions_emitted: usize,
    pub events_emitted: usize,
}
/// EVM name mangler
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct EvmNameMangler {
    pub used: std::collections::HashSet<String>,
    pub map: std::collections::HashMap<String, String>,
}
/// EVM ABI error
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EvmAbiError {
    pub name: String,
    pub inputs: Vec<(String, EvmAbiType)>,
}
/// EVM backend config (extended)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EvmExtConfig {
    pub evm_version: String,
    pub optimize: bool,
    pub optimize_runs: u32,
    pub emit_ir: bool,
    pub emit_asm: bool,
    pub via_ir: bool,
    pub revert_strings: bool,
    pub use_yul: bool,
}
/// EVM ABI event
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EvmAbiEvent {
    pub name: String,
    pub inputs: Vec<(String, EvmAbiType, bool)>,
    pub is_anonymous: bool,
}
/// EVM Yul statement
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum YulStmt {
    Let(Vec<String>, Option<YulExpr>),
    Assign(Vec<String>, YulExpr),
    If(YulExpr, Vec<YulStmt>),
    Switch(YulExpr, Vec<(u64, Vec<YulStmt>)>, Option<Vec<YulStmt>>),
    For(Vec<YulStmt>, YulExpr, Vec<YulStmt>, Vec<YulStmt>),
    Break,
    Continue,
    Leave,
    Return(YulExpr, YulExpr),
    Revert(YulExpr, YulExpr),
    Pop(YulExpr),
    Expr(YulExpr),
}
/// EVM contract template (standard ERC-20 like)
#[allow(dead_code)]
pub struct EvmContractTemplate {
    pub name: String,
    pub spdx: String,
    pub pragma: String,
}
/// EVM opcode category
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvmOpcodeCategory {
    Stop,
    Arithmetic,
    Comparison,
    Bitwise,
    Sha3,
    EnvInfo,
    BlockInfo,
    MemStack,
    Storage,
    Control,
    Log,
    System,
    Push,
    Dup,
    Swap,
}
/// EVM pass summary
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct EvmPassSummary {
    pub pass_name: String,
    pub functions_compiled: usize,
    pub bytecodes_generated: usize,
    pub optimizations_applied: usize,
    pub duration_us: u64,
}
/// EVM code optimizer pass
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvmOptPass {
    DeadCodeElim,
    ConstantFolding,
    CommonSubexprElim,
    InlineFunctions,
    JumpElim,
    PushPop,
    Peephole,
}
/// An EVM function / entry point within a contract.
///
/// Each function has a 4-byte ABI selector and a sequence of basic blocks.
#[derive(Debug, Clone)]
pub struct EvmFunction {
    /// Human-readable name of the function.
    pub name: String,
    /// 4-byte function selector (first 4 bytes of keccak256(signature)).
    pub selector: [u8; 4],
    /// ABI signature string, e.g. `"transfer(address,uint256)"`.
    pub signature: String,
    /// Basic blocks forming the function body.
    pub blocks: Vec<EvmBasicBlock>,
    /// Whether this function is payable.
    pub is_payable: bool,
    /// Whether this function is a view (does not modify state).
    pub is_view: bool,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct EVMPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
/// EVM memory model
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EvmMemoryLayout {
    pub scratch_space: (u64, u64),
    pub free_mem_ptr: u64,
    pub zero_slot: u64,
    pub initial_free: u64,
}
/// EVM ABI type
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum EvmAbiType {
    Uint(u16),
    Int(u16),
    Address,
    Bool,
    Bytes(u8),
    BytesDyn,
    StringDyn,
    Tuple(Vec<EvmAbiType>),
    Array(Box<EvmAbiType>, Option<u64>),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EVMDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
