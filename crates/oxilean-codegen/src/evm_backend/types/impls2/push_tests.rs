//! The PUSH instructions `EvmBackend` emits: storage slots, the dispatcher's
//! jump destinations and the init-code length operands.

use super::*;

/// PUSHn opcode byte for an n-byte operand (PUSH1 = 0x60).
fn push_opcode_byte(len: usize) -> u8 {
    0x5f + len as u8
}

/// The shortest big-endian byte string for `slot` (a single zero byte for 0).
fn shortest_be(slot: u64) -> Vec<u8> {
    let bytes = slot.to_be_bytes();
    let first_nonzero = bytes.iter().position(|&b| b != 0).unwrap_or(7);
    bytes[first_nonzero..].to_vec()
}

fn slots_to_check() -> Vec<u64> {
    let mut slots = vec![
        0,
        1,
        2,
        0xff,
        0x100,
        0xffff,
        0x1_0000,
        u64::MAX,
        u64::MAX - 1,
    ];
    for shift in 0..64 {
        let p = 1u64 << shift;
        slots.push(p);
        slots.push(p - 1);
        slots.push(p.wrapping_add(1));
    }
    slots
}

#[test]
fn sload_and_sstore_push_the_shortest_operand_for_every_slot() {
    let backend = EvmBackend::new();
    for slot in slots_to_check() {
        let expected = shortest_be(slot);
        for (instrs, op) in [
            (backend.emit_sload(slot), EvmOpcode::Sload),
            (backend.emit_sstore(slot), EvmOpcode::Sstore),
        ] {
            assert_eq!(instrs.len(), 2, "slot {slot}");
            assert_eq!(
                instrs[0].data.as_deref(),
                Some(expected.as_slice()),
                "slot {slot}"
            );
            assert_eq!(
                instrs[0].opcode.byte(),
                push_opcode_byte(expected.len()),
                "slot {slot}"
            );
            assert_eq!(
                instrs[0].comment.as_deref(),
                Some(format!("storage slot {slot}").as_str())
            );
            assert_eq!(instrs[1].opcode.byte(), op.byte());
            assert_eq!(instrs[1].data, None);
        }
    }
}

fn contract_with_functions(names: &[&str]) -> EvmContract {
    let mut contract = EvmContract::new("C");
    for name in names {
        let signature = format!("{name}()");
        let selector = EvmBackend::compute_selector(&signature);
        contract.add_function(EvmFunction::new(*name, selector, signature));
    }
    contract
}

#[test]
fn dispatcher_pushes_a_two_byte_zero_destination_per_function() {
    let backend = EvmBackend::new();
    let contract = contract_with_functions(&["a", "b", "c"]);
    let instrs = backend.emit_dispatcher(&contract);
    // Five fixed instructions, five per function, three closing ones.
    assert_eq!(instrs.len(), 5 + 5 * 3 + 3);
    for (i, name) in ["a", "b", "c"].iter().enumerate() {
        let dest = &instrs[5 + 5 * i + 3];
        assert_eq!(dest.opcode.byte(), push_opcode_byte(2));
        assert_eq!(dest.data.as_deref(), Some([0u8, 0u8].as_slice()));
        assert_eq!(
            dest.comment.as_deref(),
            Some(format!("dest: {name}").as_str())
        );
        assert_eq!(instrs[5 + 5 * i + 4].opcode.byte(), EvmOpcode::Jumpi.byte());
    }
}

/// The deployment stub `emit_init_code` places after the constructor, with
/// `len_push` the encoded length operand (opcode byte followed by its bytes).
fn expected_init_code(constructor: &[u8], len_push: &[u8], runtime: &[u8]) -> Vec<u8> {
    let mut out = constructor.to_vec();
    out.extend_from_slice(len_push);
    out.extend_from_slice(&[push_opcode_byte(2), 0x00, 0x00]);
    out.extend_from_slice(&[push_opcode_byte(1), 0x00]);
    out.push(EvmOpcode::Codecopy.byte());
    out.extend_from_slice(len_push);
    out.extend_from_slice(&[push_opcode_byte(1), 0x00]);
    out.push(EvmOpcode::Return.byte());
    out.extend_from_slice(runtime);
    out
}

#[test]
fn init_code_for_a_small_runtime_pushes_a_two_byte_length() {
    let backend = EvmBackend::new();
    let mut contract = contract_with_functions(&["f"]);
    contract.constructor_code = vec![
        EvmInstruction::push1(0x2a),
        EvmInstruction::new(EvmOpcode::Pop),
    ];
    let runtime = backend.emit_runtime_bytes(&contract);
    assert!(runtime.len() <= 0xffff);
    let constructor = backend.emit_constructor_bytes(&contract);
    let len = (runtime.len() as u16).to_be_bytes();
    let len_push = [push_opcode_byte(2), len[0], len[1]];
    assert_eq!(
        backend.emit_init_code(&contract),
        expected_init_code(&constructor, &len_push, &runtime)
    );
}

#[test]
fn init_code_at_the_two_byte_limit_still_uses_two_bytes() {
    let backend = EvmBackend::new();
    let mut contract = EvmContract::new("Big");
    contract.add_function(EvmFunction::new("pad", [0; 4], "pad()"));
    let with_function = backend.emit_runtime_bytes(&contract).len();
    let mut func = EvmFunction::new("pad", [0; 4], "pad()");
    func.blocks.push(EvmBasicBlock {
        label: "entry".to_string(),
        instructions: vec![EvmInstruction::new(EvmOpcode::Stop); 0xffff - with_function],
        is_jump_target: false,
    });
    contract.functions.clear();
    contract.add_function(func);
    let runtime = backend.emit_runtime_bytes(&contract);
    assert_eq!(runtime.len(), 0xffff);
    let init = backend.emit_init_code(&contract);
    let len_push = [push_opcode_byte(2), 0xff, 0xff];
    assert_eq!(init, expected_init_code(&[], &len_push, &runtime));
}

#[test]
fn init_code_for_a_large_runtime_pushes_a_four_byte_length() {
    let backend = EvmBackend::new();
    let mut contract = EvmContract::new("Big");
    let mut func = EvmFunction::new("pad", [1, 2, 3, 4], "pad()");
    func.blocks.push(EvmBasicBlock {
        label: "entry".to_string(),
        instructions: vec![EvmInstruction::new(EvmOpcode::Stop); 70_000],
        is_jump_target: false,
    });
    contract.add_function(func);
    contract.constructor_code = vec![EvmInstruction::push1(1)];
    let runtime = backend.emit_runtime_bytes(&contract);
    assert!(runtime.len() > 0xffff);
    let constructor = backend.emit_constructor_bytes(&contract);
    let len = (runtime.len() as u32).to_be_bytes();
    let len_push = [push_opcode_byte(4), len[0], len[1], len[2], len[3]];
    assert_eq!(
        backend.emit_init_code(&contract),
        expected_init_code(&constructor, &len_push, &runtime)
    );
}
