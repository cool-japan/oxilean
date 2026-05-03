//! Implementation blocks (part 2)

use super::defs::*;
use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};
use tiny_keccak::{Hasher, Keccak};

/// Compute the first 4 bytes of keccak256 over the given UTF-8 string.
/// This yields the canonical Ethereum ABI function selector.
fn keccak256_selector(sig: &str) -> [u8; 4] {
    let mut keccak = Keccak::v256();
    let mut output = [0u8; 32];
    keccak.update(sig.as_bytes());
    keccak.finalize(&mut output);
    [output[0], output[1], output[2], output[3]]
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
#[allow(dead_code)]
impl EvmSelector {
    /// Build an `EvmSelector` from an ABI canonical signature string.
    /// The 4-byte selector is the first 4 bytes of keccak256(signature).
    pub fn from_signature(sig: &str) -> Self {
        let selector = keccak256_selector(sig);
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
impl EvmBackend {
    /// Create a new EVM backend instance.
    pub fn new() -> Self {
        Self::default()
    }
    /// Compute a 4-byte Ethereum ABI function selector from a canonical
    /// signature string (e.g. `"transfer(address,uint256)"`).
    ///
    /// Returns the first 4 bytes of keccak256(signature), as specified
    /// by the Ethereum ABI encoding standard.
    pub fn compute_selector(signature: &str) -> [u8; 4] {
        keccak256_selector(signature)
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
