//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;

use super::types::{
    EVMAnalysisCache, EVMConstantFoldingHelper, EVMDepGraph, EVMDominatorTree, EVMLivenessInfo,
    EVMPassConfig, EVMPassPhase, EVMPassRegistry, EVMPassStats, EVMWorklist, EvmBackend,
    EvmBasicBlock, EvmContract, EvmInstruction, EvmOpcode, EvmOpcodeCategory, EvmOpcodeDesc,
    StorageLayout,
};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub(super) fn test_opcode_bytes() {
        assert_eq!(EvmOpcode::Stop.byte(), 0x00);
        assert_eq!(EvmOpcode::Add.byte(), 0x01);
        assert_eq!(EvmOpcode::Push1.byte(), 0x60);
        assert_eq!(EvmOpcode::Push32.byte(), 0x7f);
        assert_eq!(EvmOpcode::Dup1.byte(), 0x80);
        assert_eq!(EvmOpcode::Swap1.byte(), 0x90);
        assert_eq!(EvmOpcode::Return.byte(), 0xf3);
        assert_eq!(EvmOpcode::Revert.byte(), 0xfd);
        assert_eq!(EvmOpcode::Invalid.byte(), 0xfe);
    }
    #[test]
    pub(super) fn test_opcode_mnemonic() {
        assert_eq!(EvmOpcode::Add.mnemonic(), "ADD");
        assert_eq!(EvmOpcode::Mstore.mnemonic(), "MSTORE");
        assert_eq!(EvmOpcode::Calldataload.mnemonic(), "CALLDATALOAD");
        assert_eq!(EvmOpcode::Jumpdest.mnemonic(), "JUMPDEST");
    }
    #[test]
    pub(super) fn test_instruction_encode() {
        let stop = EvmInstruction::new(EvmOpcode::Stop);
        assert_eq!(stop.encode(), vec![0x00]);
        assert_eq!(stop.byte_len(), 1);
        let push1 = EvmInstruction::push1(0x42);
        assert_eq!(push1.encode(), vec![0x60, 0x42]);
        assert_eq!(push1.byte_len(), 2);
        let push4 = EvmInstruction::push4(0xdeadbeef);
        assert_eq!(push4.encode(), vec![0x63, 0xde, 0xad, 0xbe, 0xef]);
        assert_eq!(push4.byte_len(), 5);
    }
    #[test]
    pub(super) fn test_push_auto_size() {
        let p =
            EvmInstruction::push(vec![0x01, 0x02, 0x03]).expect("p instruction should be valid");
        assert_eq!(p.opcode, EvmOpcode::Push3);
        assert_eq!(p.encode(), vec![0x62, 0x01, 0x02, 0x03]);
        assert!(EvmInstruction::push(vec![]).is_none());
        assert!(EvmInstruction::push(vec![0u8; 33]).is_none());
    }
    #[test]
    pub(super) fn test_basic_block_encode() {
        let mut block = EvmBasicBlock::new_jump_target("entry");
        block.push_op(EvmOpcode::Caller);
        block.push_op(EvmOpcode::Stop);
        let bytes = block.encode();
        assert_eq!(bytes, vec![0x5b, 0x33, 0x00]);
        assert_eq!(block.byte_len(), 3);
    }
    #[test]
    pub(super) fn test_storage_layout() {
        let mut layout = StorageLayout::new();
        assert!(layout.is_empty());
        let slot_a = layout.allocate("balance");
        let slot_b = layout.allocate("totalSupply");
        assert_eq!(slot_a, 0);
        assert_eq!(slot_b, 1);
        assert_eq!(layout.slot_of("balance"), Some(0));
        assert_eq!(layout.slot_of("totalSupply"), Some(1));
        assert_eq!(layout.slot_of("unknown"), None);
        assert_eq!(layout.len(), 2);
        assert!(!layout.is_empty());
    }
    #[test]
    pub(super) fn test_compute_selector() {
        let sel = EvmBackend::compute_selector("transfer(address,uint256)");
        let sel2 = EvmBackend::compute_selector("transfer(address,uint256)");
        assert_eq!(sel, sel2);
        let sel3 = EvmBackend::compute_selector("balanceOf(address)");
        assert_ne!(sel, sel3);
    }
    #[test]
    pub(super) fn test_build_arithmetic_function() {
        let sel = EvmBackend::compute_selector("add(uint256,uint256)");
        let func = EvmBackend::build_arithmetic_function(
            "add",
            "add(uint256,uint256)",
            sel,
            EvmOpcode::Add,
        );
        assert_eq!(func.name, "add");
        assert_eq!(func.selector, sel);
        assert_eq!(func.blocks.len(), 1);
        assert!(func.blocks[0].is_jump_target);
        let bytes = func.encode();
        assert!(!bytes.is_empty());
        assert_eq!(
            *bytes.last().expect("collection should not be empty"),
            EvmOpcode::Return.byte()
        );
    }
    #[test]
    pub(super) fn test_emit_hex_and_assembly() {
        let backend = EvmBackend::new();
        let mut contract = EvmContract::new("SimpleToken");
        contract.set_metadata("version", "0.8.0");
        contract.allocate_storage("_balance");
        contract.allocate_storage("_totalSupply");
        let sel = EvmBackend::compute_selector("add(uint256,uint256)");
        let func = EvmBackend::build_arithmetic_function(
            "add",
            "add(uint256,uint256)",
            sel,
            EvmOpcode::Add,
        );
        contract.add_function(func);
        let hex = backend.emit_hex(&contract);
        assert!(!hex.is_empty());
        assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));
        let hex_prefixed = backend.emit_hex_prefixed(&contract);
        assert!(hex_prefixed.starts_with("0x"));
        let asm = backend.emit_assembly(&contract);
        assert!(asm.contains("SimpleToken"));
        assert!(asm.contains("CALLDATALOAD"));
        assert!(asm.contains("_balance"));
        assert!(asm.contains("_totalSupply"));
        assert!(asm.contains("add"));
    }
}
/// EVM standard opcode table
#[allow(dead_code)]
pub fn evm_opcode_table() -> Vec<EvmOpcodeDesc> {
    vec![
        EvmOpcodeDesc {
            name: "STOP".into(),
            opcode: 0x00,
            stack_in: 0,
            stack_out: 0,
            gas: 0,
            category: EvmOpcodeCategory::Stop,
            description: "Halts execution".into(),
        },
        EvmOpcodeDesc {
            name: "ADD".into(),
            opcode: 0x01,
            stack_in: 2,
            stack_out: 1,
            gas: 3,
            category: EvmOpcodeCategory::Arithmetic,
            description: "Addition".into(),
        },
        EvmOpcodeDesc {
            name: "MUL".into(),
            opcode: 0x02,
            stack_in: 2,
            stack_out: 1,
            gas: 5,
            category: EvmOpcodeCategory::Arithmetic,
            description: "Multiplication".into(),
        },
        EvmOpcodeDesc {
            name: "SUB".into(),
            opcode: 0x03,
            stack_in: 2,
            stack_out: 1,
            gas: 3,
            category: EvmOpcodeCategory::Arithmetic,
            description: "Subtraction".into(),
        },
        EvmOpcodeDesc {
            name: "DIV".into(),
            opcode: 0x04,
            stack_in: 2,
            stack_out: 1,
            gas: 5,
            category: EvmOpcodeCategory::Arithmetic,
            description: "Integer division".into(),
        },
        EvmOpcodeDesc {
            name: "MOD".into(),
            opcode: 0x06,
            stack_in: 2,
            stack_out: 1,
            gas: 5,
            category: EvmOpcodeCategory::Arithmetic,
            description: "Modulo".into(),
        },
        EvmOpcodeDesc {
            name: "EXP".into(),
            opcode: 0x0a,
            stack_in: 2,
            stack_out: 1,
            gas: 10,
            category: EvmOpcodeCategory::Arithmetic,
            description: "Exponentiation".into(),
        },
        EvmOpcodeDesc {
            name: "LT".into(),
            opcode: 0x10,
            stack_in: 2,
            stack_out: 1,
            gas: 3,
            category: EvmOpcodeCategory::Comparison,
            description: "Less than".into(),
        },
        EvmOpcodeDesc {
            name: "GT".into(),
            opcode: 0x11,
            stack_in: 2,
            stack_out: 1,
            gas: 3,
            category: EvmOpcodeCategory::Comparison,
            description: "Greater than".into(),
        },
        EvmOpcodeDesc {
            name: "EQ".into(),
            opcode: 0x14,
            stack_in: 2,
            stack_out: 1,
            gas: 3,
            category: EvmOpcodeCategory::Comparison,
            description: "Equality".into(),
        },
        EvmOpcodeDesc {
            name: "ISZERO".into(),
            opcode: 0x15,
            stack_in: 1,
            stack_out: 1,
            gas: 3,
            category: EvmOpcodeCategory::Comparison,
            description: "Is zero".into(),
        },
        EvmOpcodeDesc {
            name: "AND".into(),
            opcode: 0x16,
            stack_in: 2,
            stack_out: 1,
            gas: 3,
            category: EvmOpcodeCategory::Bitwise,
            description: "Bitwise AND".into(),
        },
        EvmOpcodeDesc {
            name: "OR".into(),
            opcode: 0x17,
            stack_in: 2,
            stack_out: 1,
            gas: 3,
            category: EvmOpcodeCategory::Bitwise,
            description: "Bitwise OR".into(),
        },
        EvmOpcodeDesc {
            name: "XOR".into(),
            opcode: 0x18,
            stack_in: 2,
            stack_out: 1,
            gas: 3,
            category: EvmOpcodeCategory::Bitwise,
            description: "Bitwise XOR".into(),
        },
        EvmOpcodeDesc {
            name: "NOT".into(),
            opcode: 0x19,
            stack_in: 1,
            stack_out: 1,
            gas: 3,
            category: EvmOpcodeCategory::Bitwise,
            description: "Bitwise NOT".into(),
        },
        EvmOpcodeDesc {
            name: "SHA3".into(),
            opcode: 0x20,
            stack_in: 2,
            stack_out: 1,
            gas: 30,
            category: EvmOpcodeCategory::Sha3,
            description: "Keccak-256".into(),
        },
        EvmOpcodeDesc {
            name: "SLOAD".into(),
            opcode: 0x54,
            stack_in: 1,
            stack_out: 1,
            gas: 2100,
            category: EvmOpcodeCategory::Storage,
            description: "Load storage".into(),
        },
        EvmOpcodeDesc {
            name: "SSTORE".into(),
            opcode: 0x55,
            stack_in: 2,
            stack_out: 0,
            gas: 20000,
            category: EvmOpcodeCategory::Storage,
            description: "Store to storage".into(),
        },
        EvmOpcodeDesc {
            name: "JUMP".into(),
            opcode: 0x56,
            stack_in: 1,
            stack_out: 0,
            gas: 8,
            category: EvmOpcodeCategory::Control,
            description: "Unconditional jump".into(),
        },
        EvmOpcodeDesc {
            name: "JUMPI".into(),
            opcode: 0x57,
            stack_in: 2,
            stack_out: 0,
            gas: 10,
            category: EvmOpcodeCategory::Control,
            description: "Conditional jump".into(),
        },
        EvmOpcodeDesc {
            name: "RETURN".into(),
            opcode: 0xf3,
            stack_in: 2,
            stack_out: 0,
            gas: 0,
            category: EvmOpcodeCategory::System,
            description: "Return from call".into(),
        },
        EvmOpcodeDesc {
            name: "REVERT".into(),
            opcode: 0xfd,
            stack_in: 2,
            stack_out: 0,
            gas: 0,
            category: EvmOpcodeCategory::System,
            description: "Revert".into(),
        },
        EvmOpcodeDesc {
            name: "CALL".into(),
            opcode: 0xf1,
            stack_in: 7,
            stack_out: 1,
            gas: 100,
            category: EvmOpcodeCategory::System,
            description: "Message call".into(),
        },
        EvmOpcodeDesc {
            name: "DELEGATECALL".into(),
            opcode: 0xf4,
            stack_in: 6,
            stack_out: 1,
            gas: 100,
            category: EvmOpcodeCategory::System,
            description: "Delegatecall".into(),
        },
        EvmOpcodeDesc {
            name: "STATICCALL".into(),
            opcode: 0xfa,
            stack_in: 6,
            stack_out: 1,
            gas: 100,
            category: EvmOpcodeCategory::System,
            description: "Staticcall".into(),
        },
        EvmOpcodeDesc {
            name: "CREATE".into(),
            opcode: 0xf0,
            stack_in: 3,
            stack_out: 1,
            gas: 32000,
            category: EvmOpcodeCategory::System,
            description: "Create contract".into(),
        },
        EvmOpcodeDesc {
            name: "CREATE2".into(),
            opcode: 0xf5,
            stack_in: 4,
            stack_out: 1,
            gas: 32000,
            category: EvmOpcodeCategory::System,
            description: "Create2 contract".into(),
        },
    ]
}
/// EVM version string
#[allow(dead_code)]
pub const EVM_PASS_VERSION: &str = "1.0.0";
/// EVM address size
#[allow(dead_code)]
pub const EVM_ADDRESS_SIZE_BYTES: usize = 20;
/// EVM word size
#[allow(dead_code)]
pub const EVM_WORD_SIZE_BYTES: usize = 32;
/// EVM stack max depth
#[allow(dead_code)]
pub const EVM_STACK_MAX_DEPTH: usize = 1024;
/// EVM max code size (EIP-170)
#[allow(dead_code)]
pub const EVM_MAX_CODE_SIZE: usize = 24576;
/// EVM max init code size (EIP-3860)
#[allow(dead_code)]
pub const EVM_MAX_INIT_CODE_SIZE: usize = 49152;
/// EVM gas estimation
#[allow(dead_code)]
pub fn evm_estimate_gas(bytecode_len: usize, storage_ops: usize, call_ops: usize) -> u64 {
    let base = 21_000u64;
    let code_gas = (bytecode_len / 32) as u64 * 200;
    let storage_gas = storage_ops as u64 * 20_000;
    let call_gas = call_ops as u64 * 100;
    base + code_gas + storage_gas + call_gas
}
/// EVM version pass string
#[allow(dead_code)]
pub const EVM_BACKEND_PASS_VERSION: &str = "1.0.0";
/// EVM deployment gas overhead
#[allow(dead_code)]
pub const EVM_DEPLOY_GAS_OVERHEAD: u64 = 32_000;
/// EVM selector ABI encoding prefix
#[allow(dead_code)]
pub const EVM_ABI_SELECTOR_SIZE: usize = 4;
#[cfg(test)]
mod EVM_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = EVMPassConfig::new("test_pass", EVMPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = EVMPassStats::new();
        stats.record_run(10, 100, 3);
        stats.record_run(20, 200, 5);
        assert_eq!(stats.total_runs, 2);
        assert!((stats.average_changes_per_run() - 15.0).abs() < 0.01);
        assert!((stats.success_rate() - 1.0).abs() < 0.01);
        let s = stats.format_summary();
        assert!(s.contains("Runs: 2/2"));
    }
    #[test]
    pub(super) fn test_pass_registry() {
        let mut reg = EVMPassRegistry::new();
        reg.register(EVMPassConfig::new("pass_a", EVMPassPhase::Analysis));
        reg.register(EVMPassConfig::new("pass_b", EVMPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = EVMAnalysisCache::new(10);
        cache.insert("key1".to_string(), vec![1, 2, 3]);
        assert!(cache.get("key1").is_some());
        assert!(cache.get("key2").is_none());
        assert!((cache.hit_rate() - 0.5).abs() < 0.01);
        cache.invalidate("key1");
        assert!(!cache.entries["key1"].valid);
        assert_eq!(cache.size(), 1);
    }
    #[test]
    pub(super) fn test_worklist() {
        let mut wl = EVMWorklist::new();
        assert!(wl.push(1));
        assert!(wl.push(2));
        assert!(!wl.push(1));
        assert_eq!(wl.len(), 2);
        assert_eq!(wl.pop(), Some(1));
        assert!(!wl.contains(1));
        assert!(wl.contains(2));
    }
    #[test]
    pub(super) fn test_dominator_tree() {
        let mut dt = EVMDominatorTree::new(5);
        dt.set_idom(1, 0);
        dt.set_idom(2, 0);
        dt.set_idom(3, 1);
        assert!(dt.dominates(0, 3));
        assert!(dt.dominates(1, 3));
        assert!(!dt.dominates(2, 3));
        assert!(dt.dominates(3, 3));
    }
    #[test]
    pub(super) fn test_liveness() {
        let mut liveness = EVMLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(EVMConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(EVMConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(EVMConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            EVMConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(EVMConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = EVMDepGraph::new();
        g.add_dep(1, 2);
        g.add_dep(2, 3);
        g.add_dep(1, 3);
        assert_eq!(g.dependencies_of(2), vec![1]);
        let topo = g.topological_sort();
        assert_eq!(topo.len(), 3);
        assert!(!g.has_cycle());
        let pos: std::collections::HashMap<u32, usize> =
            topo.iter().enumerate().map(|(i, &n)| (n, i)).collect();
        assert!(pos[&1] < pos[&2]);
        assert!(pos[&1] < pos[&3]);
        assert!(pos[&2] < pos[&3]);
    }
}
/// EVM identifier mangler
#[allow(dead_code)]
pub fn evm_mangle_identifier(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}
/// EVM keccak mock (for selector computation placeholder)
#[allow(dead_code)]
pub fn evm_keccak_placeholder(input: &[u8]) -> [u8; 32] {
    let mut out = [0u8; 32];
    for (i, &b) in input.iter().take(32).enumerate() {
        out[i] = b;
    }
    out
}
/// EVM is valid selector
#[allow(dead_code)]
pub fn evm_is_valid_selector(selector: &[u8]) -> bool {
    selector.len() == 4
}
/// EVM code pass version
#[allow(dead_code)]
pub const EVM_CODE_PASS_VERSION: &str = "1.0.0";
/// EVM solidity min version
#[allow(dead_code)]
pub const EVM_SOLIDITY_MIN_VERSION: &str = "0.8.0";
