//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

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
#[allow(dead_code)]
impl EvmOptPipeline {
    pub fn new(runs: u32) -> Self {
        Self {
            passes: vec![
                EvmOptPass::ConstantFolding,
                EvmOptPass::DeadCodeElim,
                EvmOptPass::CommonSubexprElim,
                EvmOptPass::Peephole,
            ],
            enabled: true,
            runs,
        }
    }
    pub fn add(&mut self, pass: EvmOptPass) {
        self.passes.push(pass);
    }
    pub fn disable(&mut self) {
        self.enabled = false;
    }
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
impl YulObject {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }
    pub fn add_function(&mut self, f: YulFunction) {
        self.functions.push(f);
    }
    pub fn add_sub_object(&mut self, o: YulObject) {
        self.sub_objects.push(o);
    }
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
impl EvmBasicBlock {
    /// Create a new basic block with the given label.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            instructions: Vec::new(),
            is_jump_target: false,
        }
    }
    /// Create a new basic block that is a jump target (has JUMPDEST).
    pub fn new_jump_target(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            instructions: Vec::new(),
            is_jump_target: true,
        }
    }
    /// Append an instruction to this block.
    pub fn push_instr(&mut self, instr: EvmInstruction) {
        self.instructions.push(instr);
    }
    /// Append a simple opcode (no immediate data) to this block.
    pub fn push_op(&mut self, opcode: EvmOpcode) {
        self.instructions.push(EvmInstruction::new(opcode));
    }
    /// Total byte count of all instructions in this block (excluding JUMPDEST).
    pub fn byte_len(&self) -> usize {
        let jumpdest_len = if self.is_jump_target { 1 } else { 0 };
        jumpdest_len
            + self
                .instructions
                .iter()
                .map(|i| i.byte_len())
                .sum::<usize>()
    }
    /// Encode this block to raw bytes.
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::new();
        if self.is_jump_target {
            out.push(EvmOpcode::Jumpdest.byte());
        }
        for instr in &self.instructions {
            out.extend(instr.encode());
        }
        out
    }
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
impl StorageLayout {
    /// Create an empty storage layout.
    pub fn new() -> Self {
        Self::default()
    }
    /// Allocate a new slot for a named variable and return the slot index.
    pub fn allocate(&mut self, name: impl Into<String>) -> u64 {
        let slot = self.next_slot;
        self.slots.insert(name.into(), slot);
        self.next_slot += 1;
        slot
    }
    /// Look up the slot for a variable by name.
    pub fn slot_of(&self, name: &str) -> Option<u64> {
        self.slots.get(name).copied()
    }
    /// Number of allocated slots.
    pub fn len(&self) -> usize {
        self.slots.len()
    }
    /// Returns true if no slots have been allocated.
    pub fn is_empty(&self) -> bool {
        self.slots.is_empty()
    }
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
impl EvmOpcode {
    /// Returns the byte value of this opcode.
    pub fn byte(&self) -> u8 {
        match self {
            EvmOpcode::Stop => 0x00,
            EvmOpcode::Add => 0x01,
            EvmOpcode::Mul => 0x02,
            EvmOpcode::Sub => 0x03,
            EvmOpcode::Div => 0x04,
            EvmOpcode::Sdiv => 0x05,
            EvmOpcode::Mod => 0x06,
            EvmOpcode::Smod => 0x07,
            EvmOpcode::Addmod => 0x08,
            EvmOpcode::Mulmod => 0x09,
            EvmOpcode::Exp => 0x0a,
            EvmOpcode::Signextend => 0x0b,
            EvmOpcode::Lt => 0x10,
            EvmOpcode::Gt => 0x11,
            EvmOpcode::Slt => 0x12,
            EvmOpcode::Sgt => 0x13,
            EvmOpcode::Eq => 0x14,
            EvmOpcode::Iszero => 0x15,
            EvmOpcode::And => 0x16,
            EvmOpcode::Or => 0x17,
            EvmOpcode::Xor => 0x18,
            EvmOpcode::Not => 0x19,
            EvmOpcode::Byte => 0x1a,
            EvmOpcode::Shl => 0x1b,
            EvmOpcode::Shr => 0x1c,
            EvmOpcode::Sar => 0x1d,
            EvmOpcode::Sha3 => 0x20,
            EvmOpcode::Address => 0x30,
            EvmOpcode::Balance => 0x31,
            EvmOpcode::Origin => 0x32,
            EvmOpcode::Caller => 0x33,
            EvmOpcode::Callvalue => 0x34,
            EvmOpcode::Calldataload => 0x35,
            EvmOpcode::Calldatasize => 0x36,
            EvmOpcode::Calldatacopy => 0x37,
            EvmOpcode::Codesize => 0x38,
            EvmOpcode::Codecopy => 0x39,
            EvmOpcode::Gasprice => 0x3a,
            EvmOpcode::Extcodesize => 0x3b,
            EvmOpcode::Extcodecopy => 0x3c,
            EvmOpcode::Returndatasize => 0x3d,
            EvmOpcode::Returndatacopy => 0x3e,
            EvmOpcode::Extcodehash => 0x3f,
            EvmOpcode::Blockhash => 0x40,
            EvmOpcode::Coinbase => 0x41,
            EvmOpcode::Timestamp => 0x42,
            EvmOpcode::Number => 0x43,
            EvmOpcode::Prevrandao => 0x44,
            EvmOpcode::Gaslimit => 0x45,
            EvmOpcode::Chainid => 0x46,
            EvmOpcode::Selfbalance => 0x47,
            EvmOpcode::Basefee => 0x48,
            EvmOpcode::Blobbasefee => 0x4a,
            EvmOpcode::Pop => 0x50,
            EvmOpcode::Mload => 0x51,
            EvmOpcode::Mstore => 0x52,
            EvmOpcode::Mstore8 => 0x53,
            EvmOpcode::Sload => 0x54,
            EvmOpcode::Sstore => 0x55,
            EvmOpcode::Jump => 0x56,
            EvmOpcode::Jumpi => 0x57,
            EvmOpcode::Pc => 0x58,
            EvmOpcode::Msize => 0x59,
            EvmOpcode::Gas => 0x5a,
            EvmOpcode::Jumpdest => 0x5b,
            EvmOpcode::Tload => 0x5c,
            EvmOpcode::Tstore => 0x5d,
            EvmOpcode::Mcopy => 0x5e,
            EvmOpcode::Push1 => 0x60,
            EvmOpcode::Push2 => 0x61,
            EvmOpcode::Push3 => 0x62,
            EvmOpcode::Push4 => 0x63,
            EvmOpcode::Push5 => 0x64,
            EvmOpcode::Push6 => 0x65,
            EvmOpcode::Push7 => 0x66,
            EvmOpcode::Push8 => 0x67,
            EvmOpcode::Push9 => 0x68,
            EvmOpcode::Push10 => 0x69,
            EvmOpcode::Push11 => 0x6a,
            EvmOpcode::Push12 => 0x6b,
            EvmOpcode::Push13 => 0x6c,
            EvmOpcode::Push14 => 0x6d,
            EvmOpcode::Push15 => 0x6e,
            EvmOpcode::Push16 => 0x6f,
            EvmOpcode::Push17 => 0x70,
            EvmOpcode::Push18 => 0x71,
            EvmOpcode::Push19 => 0x72,
            EvmOpcode::Push20 => 0x73,
            EvmOpcode::Push21 => 0x74,
            EvmOpcode::Push22 => 0x75,
            EvmOpcode::Push23 => 0x76,
            EvmOpcode::Push24 => 0x77,
            EvmOpcode::Push25 => 0x78,
            EvmOpcode::Push26 => 0x79,
            EvmOpcode::Push27 => 0x7a,
            EvmOpcode::Push28 => 0x7b,
            EvmOpcode::Push29 => 0x7c,
            EvmOpcode::Push30 => 0x7d,
            EvmOpcode::Push31 => 0x7e,
            EvmOpcode::Push32 => 0x7f,
            EvmOpcode::Dup1 => 0x80,
            EvmOpcode::Dup2 => 0x81,
            EvmOpcode::Dup3 => 0x82,
            EvmOpcode::Dup4 => 0x83,
            EvmOpcode::Dup5 => 0x84,
            EvmOpcode::Dup6 => 0x85,
            EvmOpcode::Dup7 => 0x86,
            EvmOpcode::Dup8 => 0x87,
            EvmOpcode::Dup9 => 0x88,
            EvmOpcode::Dup10 => 0x89,
            EvmOpcode::Dup11 => 0x8a,
            EvmOpcode::Dup12 => 0x8b,
            EvmOpcode::Dup13 => 0x8c,
            EvmOpcode::Dup14 => 0x8d,
            EvmOpcode::Dup15 => 0x8e,
            EvmOpcode::Dup16 => 0x8f,
            EvmOpcode::Swap1 => 0x90,
            EvmOpcode::Swap2 => 0x91,
            EvmOpcode::Swap3 => 0x92,
            EvmOpcode::Swap4 => 0x93,
            EvmOpcode::Swap5 => 0x94,
            EvmOpcode::Swap6 => 0x95,
            EvmOpcode::Swap7 => 0x96,
            EvmOpcode::Swap8 => 0x97,
            EvmOpcode::Swap9 => 0x98,
            EvmOpcode::Swap10 => 0x99,
            EvmOpcode::Swap11 => 0x9a,
            EvmOpcode::Swap12 => 0x9b,
            EvmOpcode::Swap13 => 0x9c,
            EvmOpcode::Swap14 => 0x9d,
            EvmOpcode::Swap15 => 0x9e,
            EvmOpcode::Swap16 => 0x9f,
            EvmOpcode::Log0 => 0xa0,
            EvmOpcode::Log1 => 0xa1,
            EvmOpcode::Log2 => 0xa2,
            EvmOpcode::Log3 => 0xa3,
            EvmOpcode::Log4 => 0xa4,
            EvmOpcode::Create => 0xf0,
            EvmOpcode::Call => 0xf1,
            EvmOpcode::Callcode => 0xf2,
            EvmOpcode::Return => 0xf3,
            EvmOpcode::Delegatecall => 0xf4,
            EvmOpcode::Create2 => 0xf5,
            EvmOpcode::Staticcall => 0xfa,
            EvmOpcode::Revert => 0xfd,
            EvmOpcode::Invalid => 0xfe,
            EvmOpcode::Selfdestruct => 0xff,
        }
    }
    /// Returns the mnemonic string for assembly output.
    pub fn mnemonic(&self) -> &'static str {
        match self {
            EvmOpcode::Stop => "STOP",
            EvmOpcode::Add => "ADD",
            EvmOpcode::Mul => "MUL",
            EvmOpcode::Sub => "SUB",
            EvmOpcode::Div => "DIV",
            EvmOpcode::Sdiv => "SDIV",
            EvmOpcode::Mod => "MOD",
            EvmOpcode::Smod => "SMOD",
            EvmOpcode::Addmod => "ADDMOD",
            EvmOpcode::Mulmod => "MULMOD",
            EvmOpcode::Exp => "EXP",
            EvmOpcode::Signextend => "SIGNEXTEND",
            EvmOpcode::Lt => "LT",
            EvmOpcode::Gt => "GT",
            EvmOpcode::Slt => "SLT",
            EvmOpcode::Sgt => "SGT",
            EvmOpcode::Eq => "EQ",
            EvmOpcode::Iszero => "ISZERO",
            EvmOpcode::And => "AND",
            EvmOpcode::Or => "OR",
            EvmOpcode::Xor => "XOR",
            EvmOpcode::Not => "NOT",
            EvmOpcode::Byte => "BYTE",
            EvmOpcode::Shl => "SHL",
            EvmOpcode::Shr => "SHR",
            EvmOpcode::Sar => "SAR",
            EvmOpcode::Sha3 => "SHA3",
            EvmOpcode::Address => "ADDRESS",
            EvmOpcode::Balance => "BALANCE",
            EvmOpcode::Origin => "ORIGIN",
            EvmOpcode::Caller => "CALLER",
            EvmOpcode::Callvalue => "CALLVALUE",
            EvmOpcode::Calldataload => "CALLDATALOAD",
            EvmOpcode::Calldatasize => "CALLDATASIZE",
            EvmOpcode::Calldatacopy => "CALLDATACOPY",
            EvmOpcode::Codesize => "CODESIZE",
            EvmOpcode::Codecopy => "CODECOPY",
            EvmOpcode::Gasprice => "GASPRICE",
            EvmOpcode::Extcodesize => "EXTCODESIZE",
            EvmOpcode::Extcodecopy => "EXTCODECOPY",
            EvmOpcode::Returndatasize => "RETURNDATASIZE",
            EvmOpcode::Returndatacopy => "RETURNDATACOPY",
            EvmOpcode::Extcodehash => "EXTCODEHASH",
            EvmOpcode::Blockhash => "BLOCKHASH",
            EvmOpcode::Coinbase => "COINBASE",
            EvmOpcode::Timestamp => "TIMESTAMP",
            EvmOpcode::Number => "NUMBER",
            EvmOpcode::Prevrandao => "PREVRANDAO",
            EvmOpcode::Gaslimit => "GASLIMIT",
            EvmOpcode::Chainid => "CHAINID",
            EvmOpcode::Selfbalance => "SELFBALANCE",
            EvmOpcode::Basefee => "BASEFEE",
            EvmOpcode::Blobbasefee => "BLOBBASEFEE",
            EvmOpcode::Pop => "POP",
            EvmOpcode::Mload => "MLOAD",
            EvmOpcode::Mstore => "MSTORE",
            EvmOpcode::Mstore8 => "MSTORE8",
            EvmOpcode::Sload => "SLOAD",
            EvmOpcode::Sstore => "SSTORE",
            EvmOpcode::Jump => "JUMP",
            EvmOpcode::Jumpi => "JUMPI",
            EvmOpcode::Pc => "PC",
            EvmOpcode::Msize => "MSIZE",
            EvmOpcode::Gas => "GAS",
            EvmOpcode::Jumpdest => "JUMPDEST",
            EvmOpcode::Tload => "TLOAD",
            EvmOpcode::Tstore => "TSTORE",
            EvmOpcode::Mcopy => "MCOPY",
            EvmOpcode::Push1 => "PUSH1",
            EvmOpcode::Push2 => "PUSH2",
            EvmOpcode::Push3 => "PUSH3",
            EvmOpcode::Push4 => "PUSH4",
            EvmOpcode::Push5 => "PUSH5",
            EvmOpcode::Push6 => "PUSH6",
            EvmOpcode::Push7 => "PUSH7",
            EvmOpcode::Push8 => "PUSH8",
            EvmOpcode::Push9 => "PUSH9",
            EvmOpcode::Push10 => "PUSH10",
            EvmOpcode::Push11 => "PUSH11",
            EvmOpcode::Push12 => "PUSH12",
            EvmOpcode::Push13 => "PUSH13",
            EvmOpcode::Push14 => "PUSH14",
            EvmOpcode::Push15 => "PUSH15",
            EvmOpcode::Push16 => "PUSH16",
            EvmOpcode::Push17 => "PUSH17",
            EvmOpcode::Push18 => "PUSH18",
            EvmOpcode::Push19 => "PUSH19",
            EvmOpcode::Push20 => "PUSH20",
            EvmOpcode::Push21 => "PUSH21",
            EvmOpcode::Push22 => "PUSH22",
            EvmOpcode::Push23 => "PUSH23",
            EvmOpcode::Push24 => "PUSH24",
            EvmOpcode::Push25 => "PUSH25",
            EvmOpcode::Push26 => "PUSH26",
            EvmOpcode::Push27 => "PUSH27",
            EvmOpcode::Push28 => "PUSH28",
            EvmOpcode::Push29 => "PUSH29",
            EvmOpcode::Push30 => "PUSH30",
            EvmOpcode::Push31 => "PUSH31",
            EvmOpcode::Push32 => "PUSH32",
            EvmOpcode::Dup1 => "DUP1",
            EvmOpcode::Dup2 => "DUP2",
            EvmOpcode::Dup3 => "DUP3",
            EvmOpcode::Dup4 => "DUP4",
            EvmOpcode::Dup5 => "DUP5",
            EvmOpcode::Dup6 => "DUP6",
            EvmOpcode::Dup7 => "DUP7",
            EvmOpcode::Dup8 => "DUP8",
            EvmOpcode::Dup9 => "DUP9",
            EvmOpcode::Dup10 => "DUP10",
            EvmOpcode::Dup11 => "DUP11",
            EvmOpcode::Dup12 => "DUP12",
            EvmOpcode::Dup13 => "DUP13",
            EvmOpcode::Dup14 => "DUP14",
            EvmOpcode::Dup15 => "DUP15",
            EvmOpcode::Dup16 => "DUP16",
            EvmOpcode::Swap1 => "SWAP1",
            EvmOpcode::Swap2 => "SWAP2",
            EvmOpcode::Swap3 => "SWAP3",
            EvmOpcode::Swap4 => "SWAP4",
            EvmOpcode::Swap5 => "SWAP5",
            EvmOpcode::Swap6 => "SWAP6",
            EvmOpcode::Swap7 => "SWAP7",
            EvmOpcode::Swap8 => "SWAP8",
            EvmOpcode::Swap9 => "SWAP9",
            EvmOpcode::Swap10 => "SWAP10",
            EvmOpcode::Swap11 => "SWAP11",
            EvmOpcode::Swap12 => "SWAP12",
            EvmOpcode::Swap13 => "SWAP13",
            EvmOpcode::Swap14 => "SWAP14",
            EvmOpcode::Swap15 => "SWAP15",
            EvmOpcode::Swap16 => "SWAP16",
            EvmOpcode::Log0 => "LOG0",
            EvmOpcode::Log1 => "LOG1",
            EvmOpcode::Log2 => "LOG2",
            EvmOpcode::Log3 => "LOG3",
            EvmOpcode::Log4 => "LOG4",
            EvmOpcode::Create => "CREATE",
            EvmOpcode::Call => "CALL",
            EvmOpcode::Callcode => "CALLCODE",
            EvmOpcode::Return => "RETURN",
            EvmOpcode::Delegatecall => "DELEGATECALL",
            EvmOpcode::Create2 => "CREATE2",
            EvmOpcode::Staticcall => "STATICCALL",
            EvmOpcode::Revert => "REVERT",
            EvmOpcode::Invalid => "INVALID",
            EvmOpcode::Selfdestruct => "SELFDESTRUCT",
        }
    }
    /// Returns the number of immediate data bytes following this opcode (for PUSH).
    pub fn immediate_size(&self) -> usize {
        match self {
            EvmOpcode::Push1 => 1,
            EvmOpcode::Push2 => 2,
            EvmOpcode::Push3 => 3,
            EvmOpcode::Push4 => 4,
            EvmOpcode::Push5 => 5,
            EvmOpcode::Push6 => 6,
            EvmOpcode::Push7 => 7,
            EvmOpcode::Push8 => 8,
            EvmOpcode::Push9 => 9,
            EvmOpcode::Push10 => 10,
            EvmOpcode::Push11 => 11,
            EvmOpcode::Push12 => 12,
            EvmOpcode::Push13 => 13,
            EvmOpcode::Push14 => 14,
            EvmOpcode::Push15 => 15,
            EvmOpcode::Push16 => 16,
            EvmOpcode::Push17 => 17,
            EvmOpcode::Push18 => 18,
            EvmOpcode::Push19 => 19,
            EvmOpcode::Push20 => 20,
            EvmOpcode::Push21 => 21,
            EvmOpcode::Push22 => 22,
            EvmOpcode::Push23 => 23,
            EvmOpcode::Push24 => 24,
            EvmOpcode::Push25 => 25,
            EvmOpcode::Push26 => 26,
            EvmOpcode::Push27 => 27,
            EvmOpcode::Push28 => 28,
            EvmOpcode::Push29 => 29,
            EvmOpcode::Push30 => 30,
            EvmOpcode::Push31 => 31,
            EvmOpcode::Push32 => 32,
            _ => 0,
        }
    }
    /// Returns the appropriate PUSH opcode for a given number of bytes.
    pub fn push_for_size(n: usize) -> Option<EvmOpcode> {
        match n {
            1 => Some(EvmOpcode::Push1),
            2 => Some(EvmOpcode::Push2),
            3 => Some(EvmOpcode::Push3),
            4 => Some(EvmOpcode::Push4),
            8 => Some(EvmOpcode::Push8),
            20 => Some(EvmOpcode::Push20),
            32 => Some(EvmOpcode::Push32),
            _ => None,
        }
    }
}
#[allow(dead_code)]
pub struct EVMPassRegistry {
    pub(super) configs: Vec<EVMPassConfig>,
    pub(super) stats: std::collections::HashMap<String, EVMPassStats>,
}
impl EVMPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        EVMPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: EVMPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), EVMPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&EVMPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&EVMPassStats> {
        self.stats.get(name)
    }
    #[allow(dead_code)]
    pub fn total_passes(&self) -> usize {
        self.configs.len()
    }
    #[allow(dead_code)]
    pub fn enabled_count(&self) -> usize {
        self.enabled_passes().len()
    }
    #[allow(dead_code)]
    pub fn update_stats(&mut self, name: &str, changes: u64, time_ms: u64, iter: u32) {
        if let Some(stats) = self.stats.get_mut(name) {
            stats.record_run(changes, time_ms, iter);
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum EVMPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl EVMPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            EVMPassPhase::Analysis => "analysis",
            EVMPassPhase::Transformation => "transformation",
            EVMPassPhase::Verification => "verification",
            EVMPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, EVMPassPhase::Transformation | EVMPassPhase::Cleanup)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EVMWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl EVMWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        EVMWorklist {
            items: std::collections::VecDeque::new(),
            in_worklist: std::collections::HashSet::new(),
        }
    }
    #[allow(dead_code)]
    pub fn push(&mut self, item: u32) -> bool {
        if self.in_worklist.insert(item) {
            self.items.push_back(item);
            true
        } else {
            false
        }
    }
    #[allow(dead_code)]
    pub fn pop(&mut self) -> Option<u32> {
        let item = self.items.pop_front()?;
        self.in_worklist.remove(&item);
        Some(item)
    }
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.items.len()
    }
    #[allow(dead_code)]
    pub fn contains(&self, item: u32) -> bool {
        self.in_worklist.contains(&item)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EVMLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl EVMLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        EVMLivenessInfo {
            live_in: vec![std::collections::HashSet::new(); block_count],
            live_out: vec![std::collections::HashSet::new(); block_count],
            defs: vec![std::collections::HashSet::new(); block_count],
            uses: vec![std::collections::HashSet::new(); block_count],
        }
    }
    #[allow(dead_code)]
    pub fn add_def(&mut self, block: usize, var: u32) {
        if block < self.defs.len() {
            self.defs[block].insert(var);
        }
    }
    #[allow(dead_code)]
    pub fn add_use(&mut self, block: usize, var: u32) {
        if block < self.uses.len() {
            self.uses[block].insert(var);
        }
    }
    #[allow(dead_code)]
    pub fn is_live_in(&self, block: usize, var: u32) -> bool {
        self.live_in
            .get(block)
            .map(|s| s.contains(&var))
            .unwrap_or(false)
    }
    #[allow(dead_code)]
    pub fn is_live_out(&self, block: usize, var: u32) -> bool {
        self.live_out
            .get(block)
            .map(|s| s.contains(&var))
            .unwrap_or(false)
    }
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
#[allow(dead_code)]
impl EvmExtIdGen {
    pub fn new(prefix: &str) -> Self {
        Self {
            counter: 0,
            prefix: prefix.to_string(),
        }
    }
    pub fn next(&mut self) -> String {
        let id = self.counter;
        self.counter += 1;
        format!("{}_{}", self.prefix, id)
    }
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
    pub(super) entries: std::collections::HashMap<String, EVMCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl EVMAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        EVMAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&EVMCacheEntry> {
        if self.entries.contains_key(key) {
            self.hits += 1;
            self.entries.get(key)
        } else {
            self.misses += 1;
            None
        }
    }
    #[allow(dead_code)]
    pub fn insert(&mut self, key: String, data: Vec<u8>) {
        if self.entries.len() >= self.max_size {
            if let Some(oldest) = self.entries.keys().next().cloned() {
                self.entries.remove(&oldest);
            }
        }
        self.entries.insert(
            key.clone(),
            EVMCacheEntry {
                key,
                data,
                timestamp: 0,
                valid: true,
            },
        );
    }
    #[allow(dead_code)]
    pub fn invalidate(&mut self, key: &str) {
        if let Some(entry) = self.entries.get_mut(key) {
            entry.valid = false;
        }
    }
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    #[allow(dead_code)]
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            return 0.0;
        }
        self.hits as f64 / total as f64
    }
    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.entries.len()
    }
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
impl EvmExtSourceBuffer {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn write(&mut self, s: &str) {
        let pad = "    ".repeat(self.indent);
        self.current.push_str(&pad);
        self.current.push_str(s);
    }
    pub fn writeln(&mut self, s: &str) {
        let pad = "    ".repeat(self.indent);
        self.current.push_str(&pad);
        self.current.push_str(s);
        self.current.push('\n');
    }
    pub fn indent(&mut self) {
        self.indent += 1;
    }
    pub fn dedent(&mut self) {
        if self.indent > 0 {
            self.indent -= 1;
        }
    }
    pub fn begin_section(&mut self, name: &str) {
        let done = std::mem::take(&mut self.current);
        if !done.is_empty() {
            self.sections.push(("anon".to_string(), done));
        }
        self.current = format!("// === {} ===\n", name);
    }
    pub fn finish(mut self) -> String {
        let done = std::mem::take(&mut self.current);
        if !done.is_empty() {
            self.sections.push(("anon".to_string(), done));
        }
        self.sections
            .iter()
            .map(|(_, s)| s.as_str())
            .collect::<Vec<_>>()
            .join("")
    }
}
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct EvmDiagSink {
    pub diags: Vec<EvmDiag>,
}
#[allow(dead_code)]
impl EvmDiagSink {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push(&mut self, level: EvmDiagLevel, msg: &str) {
        self.diags.push(EvmDiag {
            level,
            message: msg.to_string(),
            location: None,
        });
    }
    pub fn has_errors(&self) -> bool {
        self.diags.iter().any(|d| d.level == EvmDiagLevel::Error)
    }
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
impl EvmContract {
    /// Create a new contract with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            functions: Vec::new(),
            storage_layout: StorageLayout::new(),
            constructor_code: Vec::new(),
            metadata: HashMap::new(),
        }
    }
    /// Add a function to this contract.
    pub fn add_function(&mut self, func: EvmFunction) {
        self.functions.push(func);
    }
    /// Allocate a storage slot for a state variable.
    pub fn allocate_storage(&mut self, name: impl Into<String>) -> u64 {
        self.storage_layout.allocate(name)
    }
    /// Add a metadata key-value pair.
    pub fn set_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.metadata.insert(key.into(), value.into());
    }
}
/// EVM pass profiler
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct EvmExtProfiler {
    pub timings: Vec<(String, u64)>,
}
#[allow(dead_code)]
impl EvmExtProfiler {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn record(&mut self, pass: &str, us: u64) {
        self.timings.push((pass.to_string(), us));
    }
    pub fn total_us(&self) -> u64 {
        self.timings.iter().map(|(_, t)| *t).sum()
    }
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
impl EVMConstantFoldingHelper {
    #[allow(dead_code)]
    pub fn fold_add_i64(a: i64, b: i64) -> Option<i64> {
        a.checked_add(b)
    }
    #[allow(dead_code)]
    pub fn fold_sub_i64(a: i64, b: i64) -> Option<i64> {
        a.checked_sub(b)
    }
    #[allow(dead_code)]
    pub fn fold_mul_i64(a: i64, b: i64) -> Option<i64> {
        a.checked_mul(b)
    }
    #[allow(dead_code)]
    pub fn fold_div_i64(a: i64, b: i64) -> Option<i64> {
        if b == 0 {
            None
        } else {
            a.checked_div(b)
        }
    }
    #[allow(dead_code)]
    pub fn fold_add_f64(a: f64, b: f64) -> f64 {
        a + b
    }
    #[allow(dead_code)]
    pub fn fold_mul_f64(a: f64, b: f64) -> f64 {
        a * b
    }
    #[allow(dead_code)]
    pub fn fold_neg_i64(a: i64) -> Option<i64> {
        a.checked_neg()
    }
    #[allow(dead_code)]
    pub fn fold_not_bool(a: bool) -> bool {
        !a
    }
    #[allow(dead_code)]
    pub fn fold_and_bool(a: bool, b: bool) -> bool {
        a && b
    }
    #[allow(dead_code)]
    pub fn fold_or_bool(a: bool, b: bool) -> bool {
        a || b
    }
    #[allow(dead_code)]
    pub fn fold_shl_i64(a: i64, b: u32) -> Option<i64> {
        a.checked_shl(b)
    }
    #[allow(dead_code)]
    pub fn fold_shr_i64(a: i64, b: u32) -> Option<i64> {
        a.checked_shr(b)
    }
    #[allow(dead_code)]
    pub fn fold_rem_i64(a: i64, b: i64) -> Option<i64> {
        if b == 0 {
            None
        } else {
            Some(a % b)
        }
    }
    #[allow(dead_code)]
    pub fn fold_bitand_i64(a: i64, b: i64) -> i64 {
        a & b
    }
    #[allow(dead_code)]
    pub fn fold_bitor_i64(a: i64, b: i64) -> i64 {
        a | b
    }
    #[allow(dead_code)]
    pub fn fold_bitxor_i64(a: i64, b: i64) -> i64 {
        a ^ b
    }
    #[allow(dead_code)]
    pub fn fold_bitnot_i64(a: i64) -> i64 {
        !a
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EVMDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl EVMDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        EVMDepGraph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn add_node(&mut self, id: u32) {
        if !self.nodes.contains(&id) {
            self.nodes.push(id);
        }
    }
    #[allow(dead_code)]
    pub fn add_dep(&mut self, dep: u32, dependent: u32) {
        self.add_node(dep);
        self.add_node(dependent);
        self.edges.push((dep, dependent));
    }
    #[allow(dead_code)]
    pub fn dependents_of(&self, node: u32) -> Vec<u32> {
        self.edges
            .iter()
            .filter(|(d, _)| *d == node)
            .map(|(_, dep)| *dep)
            .collect()
    }
    #[allow(dead_code)]
    pub fn dependencies_of(&self, node: u32) -> Vec<u32> {
        self.edges
            .iter()
            .filter(|(_, dep)| *dep == node)
            .map(|(d, _)| *d)
            .collect()
    }
    #[allow(dead_code)]
    pub fn topological_sort(&self) -> Vec<u32> {
        let mut in_degree: std::collections::HashMap<u32, u32> = std::collections::HashMap::new();
        for &n in &self.nodes {
            in_degree.insert(n, 0);
        }
        for (_, dep) in &self.edges {
            *in_degree.entry(*dep).or_insert(0) += 1;
        }
        let mut queue: std::collections::VecDeque<u32> = self
            .nodes
            .iter()
            .filter(|&&n| in_degree[&n] == 0)
            .copied()
            .collect();
        let mut result = Vec::new();
        while let Some(node) = queue.pop_front() {
            result.push(node);
            for dep in self.dependents_of(node) {
                let cnt = in_degree.entry(dep).or_insert(0);
                *cnt = cnt.saturating_sub(1);
                if *cnt == 0 {
                    queue.push_back(dep);
                }
            }
        }
        result
    }
    #[allow(dead_code)]
    pub fn has_cycle(&self) -> bool {
        self.topological_sort().len() < self.nodes.len()
    }
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
#[allow(dead_code)]
impl EvmSelector {
    pub fn from_signature(sig: &str) -> Self {
        let bytes = sig.as_bytes();
        let selector = [
            bytes.get(0).copied().unwrap_or(0),
            bytes.get(1).copied().unwrap_or(0),
            bytes.get(2).copied().unwrap_or(0),
            bytes.get(3).copied().unwrap_or(0),
        ];
        Self {
            signature: sig.to_string(),
            selector,
        }
    }
    pub fn hex(&self) -> String {
        format!(
            "{:02x}{:02x}{:02x}{:02x}",
            self.selector[0], self.selector[1], self.selector[2], self.selector[3]
        )
    }
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
impl EvmBackend {
    /// Create a new EVM backend instance.
    pub fn new() -> Self {
        Self::default()
    }
    /// Compute a 4-byte function selector from an ABI signature string.
    ///
    /// This is a placeholder using a fast mixing hash (not real keccak256).
    /// In production this should use a proper keccak256 crate.
    pub fn compute_selector(signature: &str) -> [u8; 4] {
        let mut hash: u32 = 0x811c9dc5;
        for &b in signature.as_bytes() {
            hash ^= b as u32;
            hash = hash.wrapping_mul(0x01000193);
        }
        hash.to_be_bytes()
    }
    /// Emit the standard ABI dispatcher preamble.
    ///
    /// This code:
    /// 1. Loads the first 4 bytes of calldata (the function selector).
    /// 2. Compares against each known selector.
    /// 3. Jumps to the appropriate handler block.
    /// 4. Falls through to REVERT if no selector matches.
    pub fn emit_dispatcher(&self, contract: &EvmContract) -> Vec<EvmInstruction> {
        let mut instrs = vec![
            EvmInstruction::new(EvmOpcode::Push1).with_comment("calldata offset 0"),
            EvmInstruction::push1(0).with_comment("offset = 0"),
            EvmInstruction::new(EvmOpcode::Calldataload)
                .with_comment("load 32 bytes from calldata[0]"),
            EvmInstruction::push1(0xe0).with_comment("shift 224 bits"),
            EvmInstruction::new(EvmOpcode::Shr).with_comment("selector = calldata >> 224"),
        ];
        for func in &contract.functions {
            let sel_val = u32::from_be_bytes(func.selector);
            instrs.push(
                EvmInstruction::new(EvmOpcode::Dup1).with_comment(format!("check {}", func.name)),
            );
            instrs.push(
                EvmInstruction::push4(sel_val)
                    .with_comment(format!("selector for {}", func.signature)),
            );
            instrs.push(EvmInstruction::new(EvmOpcode::Eq));
            instrs.push(
                EvmInstruction::push(vec![0x00, 0x00])
                    .expect("push of 2-byte slice is always valid (1..=32 bytes)")
                    .with_comment(format!("dest: {}", func.name)),
            );
            instrs.push(EvmInstruction::new(EvmOpcode::Jumpi));
        }
        instrs.push(EvmInstruction::push1(0).with_comment("revert size 0"));
        instrs.push(EvmInstruction::push1(0).with_comment("revert offset 0"));
        instrs.push(EvmInstruction::new(EvmOpcode::Revert).with_comment("no matching selector"));
        instrs
    }
    /// Emit instructions to load a storage variable onto the stack.
    pub fn emit_sload(&self, slot: u64) -> Vec<EvmInstruction> {
        let mut instrs = Vec::new();
        let bytes = slot.to_be_bytes();
        let trimmed: Vec<u8> = {
            let first_nonzero = bytes.iter().position(|&b| b != 0).unwrap_or(7);
            bytes[first_nonzero..].to_vec()
        };
        let push_instr = EvmInstruction::push(if trimmed.is_empty() {
            vec![0x00]
        } else {
            trimmed
        })
        .expect("push of 1..=8 byte slot always valid (within 1..=32 byte range)")
        .with_comment(format!("storage slot {}", slot));
        instrs.push(push_instr);
        instrs.push(
            EvmInstruction::new(EvmOpcode::Sload).with_comment(format!("SLOAD slot {}", slot)),
        );
        instrs
    }
    /// Emit instructions to store the top-of-stack value into a storage slot.
    ///
    /// Assumes the value to store is already on the stack.
    pub fn emit_sstore(&self, slot: u64) -> Vec<EvmInstruction> {
        let mut instrs = Vec::new();
        let bytes = slot.to_be_bytes();
        let trimmed: Vec<u8> = {
            let first_nonzero = bytes.iter().position(|&b| b != 0).unwrap_or(7);
            bytes[first_nonzero..].to_vec()
        };
        let push_instr = EvmInstruction::push(if trimmed.is_empty() {
            vec![0x00]
        } else {
            trimmed
        })
        .expect("push of 1..=8 byte slot always valid (within 1..=32 byte range)")
        .with_comment(format!("storage slot {}", slot));
        instrs.push(push_instr);
        instrs.push(
            EvmInstruction::new(EvmOpcode::Sstore).with_comment(format!("SSTORE slot {}", slot)),
        );
        instrs
    }
    /// Encode the constructor code portion of a contract to raw bytes.
    pub fn emit_constructor_bytes(&self, contract: &EvmContract) -> Vec<u8> {
        contract
            .constructor_code
            .iter()
            .flat_map(|i| i.encode())
            .collect()
    }
    /// Encode the full runtime bytecode of a contract to raw bytes.
    ///
    /// Layout: dispatcher preamble + function bodies (each starting with JUMPDEST).
    pub fn emit_runtime_bytes(&self, contract: &EvmContract) -> Vec<u8> {
        let mut out = Vec::new();
        for instr in self.emit_dispatcher(contract) {
            out.extend(instr.encode());
        }
        for func in &contract.functions {
            out.extend(func.encode());
        }
        out
    }
    /// Encode the complete init code (constructor + runtime deployment stub).
    ///
    /// The init code:
    /// 1. Runs constructor logic.
    /// 2. Returns a copy of the runtime bytecode so the EVM stores it.
    pub fn emit_init_code(&self, contract: &EvmContract) -> Vec<u8> {
        let runtime = self.emit_runtime_bytes(contract);
        let runtime_len = runtime.len();
        let constructor = self.emit_constructor_bytes(contract);
        let mut init = Vec::new();
        init.extend_from_slice(&constructor);
        if runtime_len <= 0xffff {
            let len_bytes = (runtime_len as u16).to_be_bytes();
            init.extend(
                EvmInstruction::push(len_bytes.to_vec())
                    .expect("2-byte push is always valid")
                    .encode(),
            );
        } else {
            let len_bytes = (runtime_len as u32).to_be_bytes();
            init.extend(
                EvmInstruction::push(len_bytes.to_vec())
                    .expect("4-byte push is always valid")
                    .encode(),
            );
        }
        init.extend(
            EvmInstruction::push(vec![0x00, 0x00])
                .expect("2-byte push is always valid")
                .encode(),
        );
        init.extend(EvmInstruction::push1(0x00).encode());
        init.push(EvmOpcode::Codecopy.byte());
        if runtime_len <= 0xffff {
            let len_bytes = (runtime_len as u16).to_be_bytes();
            init.extend(
                EvmInstruction::push(len_bytes.to_vec())
                    .expect("2-byte push is always valid")
                    .encode(),
            );
        } else {
            let len_bytes = (runtime_len as u32).to_be_bytes();
            init.extend(
                EvmInstruction::push(len_bytes.to_vec())
                    .expect("4-byte push is always valid")
                    .encode(),
            );
        }
        init.extend(EvmInstruction::push1(0x00).encode());
        init.push(EvmOpcode::Return.byte());
        init.extend_from_slice(&runtime);
        init
    }
    /// Encode the runtime bytecode as a hex string (no 0x prefix).
    pub fn emit_hex(&self, contract: &EvmContract) -> String {
        let bytes = self.emit_runtime_bytes(contract);
        bytes.iter().map(|b| format!("{:02x}", b)).collect()
    }
    /// Encode the runtime bytecode as a hex string with `0x` prefix.
    pub fn emit_hex_prefixed(&self, contract: &EvmContract) -> String {
        format!("0x{}", self.emit_hex(contract))
    }
    /// Encode the full init code as a hex string with `0x` prefix.
    pub fn emit_init_hex(&self, contract: &EvmContract) -> String {
        let bytes = self.emit_init_code(contract);
        format!(
            "0x{}",
            bytes
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>()
        )
    }
    /// Format a single instruction as an assembly line with optional byte offset.
    pub(super) fn format_instr_line(offset: usize, instr: &EvmInstruction) -> String {
        let mut line = format!("{:04x}  {}", offset, instr.opcode.mnemonic());
        if let Some(ref data) = instr.data {
            line.push(' ');
            line.push_str("0x");
            for b in data {
                line.push_str(&format!("{:02x}", b));
            }
        }
        if let Some(ref c) = instr.comment {
            while line.len() < 30 {
                line.push(' ');
            }
            line.push_str("; ");
            line.push_str(c);
        }
        line
    }
    /// Emit human-readable assembly text for an `EvmContract`.
    pub fn emit_assembly(&self, contract: &EvmContract) -> String {
        let mut out = String::new();
        out.push_str(&format!("; Contract: {}\n", contract.name));
        for (k, v) in &contract.metadata {
            out.push_str(&format!("; {}: {}\n", k, v));
        }
        out.push('\n');
        if !contract.storage_layout.is_empty() {
            out.push_str("; Storage Layout:\n");
            let mut slots: Vec<_> = contract.storage_layout.slots.iter().collect();
            slots.sort_by_key(|(_, &s)| s);
            for (name, slot) in &slots {
                out.push_str(&format!(";   slot {:3}: {}\n", slot, name));
            }
            out.push('\n');
        }
        if !contract.constructor_code.is_empty() {
            out.push_str("constructor:\n");
            let mut offset = 0usize;
            for instr in &contract.constructor_code {
                out.push_str(&format!("  {}\n", Self::format_instr_line(offset, instr)));
                offset += instr.byte_len();
            }
            out.push('\n');
        }
        out.push_str("runtime_dispatcher:\n");
        let dispatcher = self.emit_dispatcher(contract);
        let mut offset = 0usize;
        for instr in &dispatcher {
            out.push_str(&format!("  {}\n", Self::format_instr_line(offset, instr)));
            offset += instr.byte_len();
        }
        out.push('\n');
        for func in &contract.functions {
            let sel_hex: String = func.selector.iter().map(|b| format!("{:02x}", b)).collect();
            out.push_str(&format!(
                "function {} (selector: 0x{}) ; {}\n",
                func.name, sel_hex, func.signature
            ));
            if func.is_payable {
                out.push_str(";   payable\n");
            }
            if func.is_view {
                out.push_str(";   view\n");
            }
            for block in &func.blocks {
                out.push_str(&format!("  .{}:\n", block.label));
                if block.is_jump_target {
                    out.push_str(&format!("    {:04x}  JUMPDEST\n", offset));
                    offset += 1;
                }
                for instr in &block.instructions {
                    out.push_str(&format!("    {}\n", Self::format_instr_line(offset, instr)));
                    offset += instr.byte_len();
                }
            }
            out.push('\n');
        }
        out
    }
    /// Build a simple two-argument arithmetic function (e.g. `add(uint256,uint256)`).
    ///
    /// Loads arg0 from calldata\[4\], arg1 from calldata\[36\], applies `op`, stores
    /// result in memory[0..32], and returns 32 bytes.
    #[allow(clippy::too_many_arguments)]
    pub fn build_arithmetic_function(
        name: &str,
        signature: &str,
        selector: [u8; 4],
        op: EvmOpcode,
    ) -> EvmFunction {
        let mut func = EvmFunction::new(name, selector, signature);
        let mut block = EvmBasicBlock::new_jump_target("entry");
        block.push_instr(EvmInstruction::push1(4).with_comment("calldata offset for arg0"));
        block.push_op(EvmOpcode::Calldataload);
        block.push_instr(EvmInstruction::push1(36).with_comment("calldata offset for arg1"));
        block.push_op(EvmOpcode::Calldataload);
        block.push_instr(EvmInstruction::new(op).with_comment("arithmetic op"));
        block.push_instr(EvmInstruction::push1(0).with_comment("mem offset"));
        block.push_op(EvmOpcode::Mstore);
        block.push_instr(EvmInstruction::push1(32).with_comment("return size"));
        block.push_instr(EvmInstruction::push1(0).with_comment("return offset"));
        block.push_op(EvmOpcode::Return);
        func.add_block(block);
        func
    }
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
impl EVMPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: EVMPassPhase) -> Self {
        EVMPassConfig {
            phase,
            enabled: true,
            max_iterations: 10,
            debug_output: false,
            pass_name: name.into(),
        }
    }
    #[allow(dead_code)]
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
    #[allow(dead_code)]
    pub fn with_debug(mut self) -> Self {
        self.debug_output = true;
        self
    }
    #[allow(dead_code)]
    pub fn max_iter(mut self, n: u32) -> Self {
        self.max_iterations = n;
        self
    }
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
impl EvmInstruction {
    /// Create a plain instruction with no immediate data.
    pub fn new(opcode: EvmOpcode) -> Self {
        Self {
            opcode,
            data: None,
            comment: None,
        }
    }
    /// Create a PUSH instruction with the given byte vector.
    ///
    /// Automatically selects the correct PUSH1..PUSH32 opcode.
    pub fn push(bytes: Vec<u8>) -> Option<Self> {
        let len = bytes.len();
        if len == 0 || len > 32 {
            return None;
        }
        let opcode = match len {
            1 => EvmOpcode::Push1,
            2 => EvmOpcode::Push2,
            3 => EvmOpcode::Push3,
            4 => EvmOpcode::Push4,
            5 => EvmOpcode::Push5,
            6 => EvmOpcode::Push6,
            7 => EvmOpcode::Push7,
            8 => EvmOpcode::Push8,
            9 => EvmOpcode::Push9,
            10 => EvmOpcode::Push10,
            11 => EvmOpcode::Push11,
            12 => EvmOpcode::Push12,
            13 => EvmOpcode::Push13,
            14 => EvmOpcode::Push14,
            15 => EvmOpcode::Push15,
            16 => EvmOpcode::Push16,
            17 => EvmOpcode::Push17,
            18 => EvmOpcode::Push18,
            19 => EvmOpcode::Push19,
            20 => EvmOpcode::Push20,
            21 => EvmOpcode::Push21,
            22 => EvmOpcode::Push22,
            23 => EvmOpcode::Push23,
            24 => EvmOpcode::Push24,
            25 => EvmOpcode::Push25,
            26 => EvmOpcode::Push26,
            27 => EvmOpcode::Push27,
            28 => EvmOpcode::Push28,
            29 => EvmOpcode::Push29,
            30 => EvmOpcode::Push30,
            31 => EvmOpcode::Push31,
            32 => EvmOpcode::Push32,
            _ => return None,
        };
        Some(Self {
            opcode,
            data: Some(bytes),
            comment: None,
        })
    }
    /// Create a PUSH1 instruction for a single byte value.
    pub fn push1(byte: u8) -> Self {
        Self {
            opcode: EvmOpcode::Push1,
            data: Some(vec![byte]),
            comment: None,
        }
    }
    /// Create a PUSH4 instruction for a 4-byte value (used for function selectors).
    pub fn push4(val: u32) -> Self {
        let bytes = val.to_be_bytes().to_vec();
        Self {
            opcode: EvmOpcode::Push4,
            data: Some(bytes),
            comment: None,
        }
    }
    /// Create a PUSH32 instruction for a 32-byte value.
    pub fn push32(val: [u8; 32]) -> Self {
        Self {
            opcode: EvmOpcode::Push32,
            data: Some(val.to_vec()),
            comment: None,
        }
    }
    /// Attach a comment to this instruction (for assembly output).
    pub fn with_comment(mut self, comment: impl Into<String>) -> Self {
        self.comment = Some(comment.into());
        self
    }
    /// Encode this instruction to its raw byte representation.
    pub fn encode(&self) -> Vec<u8> {
        let mut out = vec![self.opcode.byte()];
        if let Some(ref data) = self.data {
            out.extend_from_slice(data);
        }
        out
    }
    /// Byte length of this instruction (opcode + immediate data).
    pub fn byte_len(&self) -> usize {
        1 + self.data.as_ref().map(|d| d.len()).unwrap_or(0)
    }
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
#[allow(dead_code)]
impl EvmStackDepth {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push(&mut self, n: i32) {
        self.current += n;
        self.max = self.max.max(self.current);
    }
    pub fn pop(&mut self, n: i32) {
        self.current -= n;
        self.min = self.min.min(self.current);
    }
    pub fn apply_opcode(&mut self, desc: &EvmOpcodeDesc) {
        self.pop(desc.stack_in as i32);
        self.push(desc.stack_out as i32);
    }
    pub fn is_valid(&self) -> bool {
        self.current >= 0 && self.current <= 1024
    }
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
#[allow(dead_code)]
impl EvmNameMangler {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn mangle(&mut self, name: &str) -> String {
        if let Some(m) = self.map.get(name) {
            return m.clone();
        }
        let mangled: String = name
            .chars()
            .map(|c| {
                if c.is_alphanumeric() || c == '_' {
                    c
                } else {
                    '_'
                }
            })
            .collect();
        let reserved = ["receive", "fallback", "constructor"];
        let mut candidate = if reserved.contains(&mangled.as_str()) {
            format!("ox_{}", mangled)
        } else {
            mangled.clone()
        };
        let base = candidate.clone();
        let mut cnt = 0;
        while self.used.contains(&candidate) {
            cnt += 1;
            candidate = format!("{}_{}", base, cnt);
        }
        self.used.insert(candidate.clone());
        self.map.insert(name.to_string(), candidate.clone());
        candidate
    }
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
#[allow(dead_code)]
impl EvmContractTemplate {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            spdx: "MIT".to_string(),
            pragma: "^0.8.0".to_string(),
        }
    }
    pub fn emit_header(&self) -> String {
        format!(
            "// SPDX-License-Identifier: {}\npragma solidity {};\n\ncontract {} {{\n",
            self.spdx, self.pragma, self.name
        )
    }
    pub fn emit_footer(&self) -> String {
        "}\n".to_string()
    }
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
impl EvmFunction {
    /// Create a new function with the given name, selector, and signature.
    pub fn new(name: impl Into<String>, selector: [u8; 4], signature: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            selector,
            signature: signature.into(),
            blocks: Vec::new(),
            is_payable: false,
            is_view: false,
        }
    }
    /// Add a basic block to this function.
    pub fn add_block(&mut self, block: EvmBasicBlock) {
        self.blocks.push(block);
    }
    /// Total byte count of all blocks in this function.
    pub fn byte_len(&self) -> usize {
        self.blocks.iter().map(|b| b.byte_len()).sum()
    }
    /// Encode this function's blocks to raw bytes.
    pub fn encode(&self) -> Vec<u8> {
        self.blocks.iter().flat_map(|b| b.encode()).collect()
    }
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
impl EVMPassStats {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn record_run(&mut self, changes: u64, time_ms: u64, iterations: u32) {
        self.total_runs += 1;
        self.successful_runs += 1;
        self.total_changes += changes;
        self.time_ms += time_ms;
        self.iterations_used = iterations;
    }
    #[allow(dead_code)]
    pub fn average_changes_per_run(&self) -> f64 {
        if self.total_runs == 0 {
            return 0.0;
        }
        self.total_changes as f64 / self.total_runs as f64
    }
    #[allow(dead_code)]
    pub fn success_rate(&self) -> f64 {
        if self.total_runs == 0 {
            return 0.0;
        }
        self.successful_runs as f64 / self.total_runs as f64
    }
    #[allow(dead_code)]
    pub fn format_summary(&self) -> String {
        format!(
            "Runs: {}/{}, Changes: {}, Time: {}ms",
            self.successful_runs, self.total_runs, self.total_changes, self.time_ms
        )
    }
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
impl EVMDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        EVMDominatorTree {
            idom: vec![None; size],
            dom_children: vec![Vec::new(); size],
            dom_depth: vec![0; size],
        }
    }
    #[allow(dead_code)]
    pub fn set_idom(&mut self, node: usize, idom: u32) {
        self.idom[node] = Some(idom);
    }
    #[allow(dead_code)]
    pub fn dominates(&self, a: usize, b: usize) -> bool {
        if a == b {
            return true;
        }
        let mut cur = b;
        loop {
            match self.idom[cur] {
                Some(parent) if parent as usize == a => return true,
                Some(parent) if parent as usize == cur => return false,
                Some(parent) => cur = parent as usize,
                None => return false,
            }
        }
    }
    #[allow(dead_code)]
    pub fn depth(&self, node: usize) -> u32 {
        self.dom_depth.get(node).copied().unwrap_or(0)
    }
}
