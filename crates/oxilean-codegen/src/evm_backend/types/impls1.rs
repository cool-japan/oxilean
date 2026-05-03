//! Implementation blocks (part 1)

use super::defs::*;
use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};

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
