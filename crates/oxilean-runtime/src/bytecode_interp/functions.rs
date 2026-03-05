//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    BasicBlock, BytecodeChunk, BytecodeCompiler, CallResult, ChunkBuilder, ChunkStats,
    ConstantFolder, DeadCodeEliminator, EncodedInstruction, FramedInterpreter, InlineCache,
    Interpreter, LivenessInfo, Opcode, OpcodeInfo, OpcodeProfile, PeepholeOptimizer,
    ProfilingInterpreter, StackValue,
};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compile_nat_executes() {
        let chunk = BytecodeCompiler::compile_nat(42);
        let mut interp = Interpreter::new();
        let result = interp
            .execute_chunk(&chunk)
            .expect("execution should succeed");
        assert_eq!(result, StackValue::Nat(42));
    }
    #[test]
    fn test_compile_add_executes() {
        let chunk = BytecodeCompiler::compile_add(10, 32);
        let mut interp = Interpreter::new();
        let result = interp
            .execute_chunk(&chunk)
            .expect("execution should succeed");
        assert_eq!(result, StackValue::Nat(42));
    }
    #[test]
    fn test_compile_if_true() {
        let chunk = BytecodeCompiler::compile_if(true, 100, 200);
        let mut interp = Interpreter::new();
        let result = interp
            .execute_chunk(&chunk)
            .expect("execution should succeed");
        assert_eq!(result, StackValue::Nat(100));
    }
    #[test]
    fn test_compile_if_false() {
        let chunk = BytecodeCompiler::compile_if(false, 100, 200);
        let mut interp = Interpreter::new();
        let result = interp
            .execute_chunk(&chunk)
            .expect("execution should succeed");
        assert_eq!(result, StackValue::Nat(200));
    }
    #[test]
    fn test_push_bool() {
        let mut chunk = BytecodeChunk::new("test");
        chunk.push_op(Opcode::PushBool(true));
        chunk.push_op(Opcode::Halt);
        let mut interp = Interpreter::new();
        let result = interp
            .execute_chunk(&chunk)
            .expect("execution should succeed");
        assert_eq!(result, StackValue::Bool(true));
    }
    #[test]
    fn test_push_str() {
        let mut chunk = BytecodeChunk::new("test");
        chunk.push_op(Opcode::PushStr("hello".to_string()));
        chunk.push_op(Opcode::Halt);
        let mut interp = Interpreter::new();
        let result = interp
            .execute_chunk(&chunk)
            .expect("execution should succeed");
        assert_eq!(result, StackValue::Str("hello".to_string()));
    }
    #[test]
    fn test_eq_same() {
        let mut chunk = BytecodeChunk::new("test");
        chunk.push_op(Opcode::Push(5));
        chunk.push_op(Opcode::Push(5));
        chunk.push_op(Opcode::Eq);
        chunk.push_op(Opcode::Halt);
        let mut interp = Interpreter::new();
        let result = interp
            .execute_chunk(&chunk)
            .expect("execution should succeed");
        assert_eq!(result, StackValue::Bool(true));
    }
    #[test]
    fn test_lt_comparison() {
        let mut chunk = BytecodeChunk::new("test");
        chunk.push_op(Opcode::Push(3));
        chunk.push_op(Opcode::Push(5));
        chunk.push_op(Opcode::Lt);
        chunk.push_op(Opcode::Halt);
        let mut interp = Interpreter::new();
        let result = interp
            .execute_chunk(&chunk)
            .expect("execution should succeed");
        assert_eq!(result, StackValue::Bool(true));
    }
    #[test]
    fn test_div_by_zero_error() {
        let mut chunk = BytecodeChunk::new("test");
        chunk.push_op(Opcode::Push(10));
        chunk.push_op(Opcode::Push(0));
        chunk.push_op(Opcode::Div);
        chunk.push_op(Opcode::Halt);
        let mut interp = Interpreter::new();
        let result = interp.execute_chunk(&chunk);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("division by zero"));
    }
}
/// Serialize an entire [`BytecodeChunk`] to a flat byte vector.
#[allow(dead_code)]
pub fn serialize_chunk(chunk: &BytecodeChunk) -> Vec<u8> {
    let mut out = Vec::new();
    let name_bytes = chunk.name.as_bytes();
    out.extend_from_slice(&(name_bytes.len() as u32).to_le_bytes());
    out.extend_from_slice(name_bytes);
    out.extend_from_slice(&(chunk.opcodes.len() as u32).to_le_bytes());
    for op in &chunk.opcodes {
        let enc = EncodedInstruction::encode(op);
        out.extend_from_slice(&enc.bytes);
    }
    out
}
/// Deserialize a [`BytecodeChunk`] from bytes produced by [`serialize_chunk`].
#[allow(dead_code)]
pub fn deserialize_chunk(data: &[u8]) -> Option<BytecodeChunk> {
    if data.len() < 4 {
        return None;
    }
    let name_len = u32::from_le_bytes(data[0..4].try_into().ok()?) as usize;
    if data.len() < 4 + name_len + 4 {
        return None;
    }
    let name = std::str::from_utf8(&data[4..4 + name_len])
        .ok()?
        .to_string();
    let mut pos = 4 + name_len;
    let op_count = u32::from_le_bytes(data[pos..pos + 4].try_into().ok()?) as usize;
    pos += 4;
    let mut chunk = BytecodeChunk::new(&name);
    for _ in 0..op_count {
        let (op, consumed) = EncodedInstruction::decode(&data[pos..])?;
        chunk.push_op(op);
        pos += consumed;
    }
    Some(chunk)
}
/// Disassemble a [`BytecodeChunk`] into a human-readable string.
#[allow(dead_code)]
pub fn disassemble(chunk: &BytecodeChunk) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "=== chunk: {} ({} ops) ===\n",
        chunk.name,
        chunk.opcodes.len()
    ));
    for (i, op) in chunk.opcodes.iter().enumerate() {
        out.push_str(&format!("{:04}  {}\n", i, format_op(op)));
    }
    out
}
/// Format a single opcode as a string.
#[allow(dead_code)]
pub fn format_op(op: &Opcode) -> String {
    match op {
        Opcode::Push(n) => format!("PUSH        {}", n),
        Opcode::PushBool(b) => format!("PUSH_BOOL   {}", b),
        Opcode::PushStr(s) => format!("PUSH_STR    {:?}", s),
        Opcode::PushNil => "PUSH_NIL".to_string(),
        Opcode::Pop => "POP".to_string(),
        Opcode::Dup => "DUP".to_string(),
        Opcode::Swap => "SWAP".to_string(),
        Opcode::Add => "ADD".to_string(),
        Opcode::Sub => "SUB".to_string(),
        Opcode::Mul => "MUL".to_string(),
        Opcode::Div => "DIV".to_string(),
        Opcode::Mod => "MOD".to_string(),
        Opcode::Eq => "EQ".to_string(),
        Opcode::Lt => "LT".to_string(),
        Opcode::Le => "LE".to_string(),
        Opcode::Not => "NOT".to_string(),
        Opcode::And => "AND".to_string(),
        Opcode::Or => "OR".to_string(),
        Opcode::Jump(off) => format!("JUMP        {:+}", off),
        Opcode::JumpIf(off) => format!("JUMP_IF     {:+}", off),
        Opcode::JumpIfNot(off) => format!("JUMP_IFNOT  {:+}", off),
        Opcode::Call(pos) => format!("CALL        @{}", pos),
        Opcode::Return => "RETURN".to_string(),
        Opcode::Load(idx) => format!("LOAD        L{}", idx),
        Opcode::Store(idx) => format!("STORE       L{}", idx),
        Opcode::LoadGlobal(n) => format!("LOAD_GLOBAL {}", n),
        Opcode::MakeClosure(n) => format!("MAKE_CLOSURE {}", n),
        Opcode::Apply => "APPLY".to_string(),
        Opcode::Halt => "HALT".to_string(),
    }
}
/// Return the full dispatch table for all supported opcodes.
#[allow(dead_code)]
pub fn opcode_dispatch_table() -> Vec<OpcodeInfo> {
    vec![
        OpcodeInfo {
            tag: 0x01,
            mnemonic: "PUSH",
            byte_size: 9,
            is_branch: false,
            is_terminator: false,
        },
        OpcodeInfo {
            tag: 0x02,
            mnemonic: "PUSH_BOOL",
            byte_size: 2,
            is_branch: false,
            is_terminator: false,
        },
        OpcodeInfo {
            tag: 0x03,
            mnemonic: "PUSH_STR",
            byte_size: 0,
            is_branch: false,
            is_terminator: false,
        },
        OpcodeInfo {
            tag: 0x04,
            mnemonic: "PUSH_NIL",
            byte_size: 1,
            is_branch: false,
            is_terminator: false,
        },
        OpcodeInfo {
            tag: 0x10,
            mnemonic: "POP",
            byte_size: 1,
            is_branch: false,
            is_terminator: false,
        },
        OpcodeInfo {
            tag: 0x11,
            mnemonic: "DUP",
            byte_size: 1,
            is_branch: false,
            is_terminator: false,
        },
        OpcodeInfo {
            tag: 0x12,
            mnemonic: "SWAP",
            byte_size: 1,
            is_branch: false,
            is_terminator: false,
        },
        OpcodeInfo {
            tag: 0x20,
            mnemonic: "ADD",
            byte_size: 1,
            is_branch: false,
            is_terminator: false,
        },
        OpcodeInfo {
            tag: 0x21,
            mnemonic: "SUB",
            byte_size: 1,
            is_branch: false,
            is_terminator: false,
        },
        OpcodeInfo {
            tag: 0x22,
            mnemonic: "MUL",
            byte_size: 1,
            is_branch: false,
            is_terminator: false,
        },
        OpcodeInfo {
            tag: 0x23,
            mnemonic: "DIV",
            byte_size: 1,
            is_branch: false,
            is_terminator: false,
        },
        OpcodeInfo {
            tag: 0x24,
            mnemonic: "MOD",
            byte_size: 1,
            is_branch: false,
            is_terminator: false,
        },
        OpcodeInfo {
            tag: 0x30,
            mnemonic: "EQ",
            byte_size: 1,
            is_branch: false,
            is_terminator: false,
        },
        OpcodeInfo {
            tag: 0x31,
            mnemonic: "LT",
            byte_size: 1,
            is_branch: false,
            is_terminator: false,
        },
        OpcodeInfo {
            tag: 0x32,
            mnemonic: "LE",
            byte_size: 1,
            is_branch: false,
            is_terminator: false,
        },
        OpcodeInfo {
            tag: 0x40,
            mnemonic: "NOT",
            byte_size: 1,
            is_branch: false,
            is_terminator: false,
        },
        OpcodeInfo {
            tag: 0x41,
            mnemonic: "AND",
            byte_size: 1,
            is_branch: false,
            is_terminator: false,
        },
        OpcodeInfo {
            tag: 0x42,
            mnemonic: "OR",
            byte_size: 1,
            is_branch: false,
            is_terminator: false,
        },
        OpcodeInfo {
            tag: 0x50,
            mnemonic: "JUMP",
            byte_size: 5,
            is_branch: true,
            is_terminator: true,
        },
        OpcodeInfo {
            tag: 0x51,
            mnemonic: "JUMP_IF",
            byte_size: 5,
            is_branch: true,
            is_terminator: false,
        },
        OpcodeInfo {
            tag: 0x52,
            mnemonic: "JUMP_IFNOT",
            byte_size: 5,
            is_branch: true,
            is_terminator: false,
        },
        OpcodeInfo {
            tag: 0x60,
            mnemonic: "CALL",
            byte_size: 5,
            is_branch: true,
            is_terminator: false,
        },
        OpcodeInfo {
            tag: 0x61,
            mnemonic: "RETURN",
            byte_size: 1,
            is_branch: true,
            is_terminator: true,
        },
        OpcodeInfo {
            tag: 0x70,
            mnemonic: "LOAD",
            byte_size: 5,
            is_branch: false,
            is_terminator: false,
        },
        OpcodeInfo {
            tag: 0x71,
            mnemonic: "STORE",
            byte_size: 5,
            is_branch: false,
            is_terminator: false,
        },
        OpcodeInfo {
            tag: 0x72,
            mnemonic: "LOAD_GLOBAL",
            byte_size: 0,
            is_branch: false,
            is_terminator: false,
        },
        OpcodeInfo {
            tag: 0x80,
            mnemonic: "MAKE_CLOSURE",
            byte_size: 5,
            is_branch: false,
            is_terminator: false,
        },
        OpcodeInfo {
            tag: 0x81,
            mnemonic: "APPLY",
            byte_size: 1,
            is_branch: false,
            is_terminator: false,
        },
        OpcodeInfo {
            tag: 0xFF,
            mnemonic: "HALT",
            byte_size: 1,
            is_branch: false,
            is_terminator: true,
        },
    ]
}
/// Handle a function call according to arity conventions.
#[allow(dead_code)]
pub fn handle_call(fn_pos: u32, arity: u32, args: Vec<StackValue>) -> CallResult {
    let n = args.len() as u32;
    if n == arity {
        CallResult::Exact { fn_pos, args }
    } else if n < arity {
        CallResult::Partial {
            captured: args,
            remaining_arity: arity - n,
        }
    } else {
        let first_args = args[..arity as usize].to_vec();
        let rest_args = args[arity as usize..].to_vec();
        CallResult::Over {
            fn_pos,
            first_args,
            rest_args,
        }
    }
}
/// Return a short string identifying the kind of opcode (for profiling keys).
#[allow(dead_code)]
pub(super) fn opcode_kind(op: &Opcode) -> String {
    match op {
        Opcode::Push(_) => "Push",
        Opcode::PushBool(_) => "PushBool",
        Opcode::PushStr(_) => "PushStr",
        Opcode::PushNil => "PushNil",
        Opcode::Pop => "Pop",
        Opcode::Dup => "Dup",
        Opcode::Swap => "Swap",
        Opcode::Add => "Add",
        Opcode::Sub => "Sub",
        Opcode::Mul => "Mul",
        Opcode::Div => "Div",
        Opcode::Mod => "Mod",
        Opcode::Eq => "Eq",
        Opcode::Lt => "Lt",
        Opcode::Le => "Le",
        Opcode::Not => "Not",
        Opcode::And => "And",
        Opcode::Or => "Or",
        Opcode::Jump(_) => "Jump",
        Opcode::JumpIf(_) => "JumpIf",
        Opcode::JumpIfNot(_) => "JumpIfNot",
        Opcode::Call(_) => "Call",
        Opcode::Return => "Return",
        Opcode::Load(_) => "Load",
        Opcode::Store(_) => "Store",
        Opcode::LoadGlobal(_) => "LoadGlobal",
        Opcode::MakeClosure(_) => "MakeClosure",
        Opcode::Apply => "Apply",
        Opcode::Halt => "Halt",
    }
    .to_string()
}
/// Perform basic block decomposition of a chunk.
///
/// Returns the list of basic blocks in order of their start offset.
#[allow(dead_code)]
pub fn find_basic_blocks(chunk: &BytecodeChunk) -> Vec<BasicBlock> {
    let n = chunk.opcodes.len();
    if n == 0 {
        return Vec::new();
    }
    let mut leaders = std::collections::BTreeSet::new();
    leaders.insert(0);
    for (i, op) in chunk.opcodes.iter().enumerate() {
        match op {
            Opcode::Jump(off) => {
                let target = (i as i64 + 1 + *off as i64) as usize;
                if target < n {
                    leaders.insert(target);
                }
                if i + 1 < n {
                    leaders.insert(i + 1);
                }
            }
            Opcode::JumpIf(off) | Opcode::JumpIfNot(off) => {
                let target = (i as i64 + 1 + *off as i64) as usize;
                if target < n {
                    leaders.insert(target);
                }
                if i + 1 < n {
                    leaders.insert(i + 1);
                }
            }
            Opcode::Call(_) | Opcode::Return | Opcode::Halt => {
                if i + 1 < n {
                    leaders.insert(i + 1);
                }
            }
            _ => {}
        }
    }
    let leaders_vec: Vec<usize> = leaders.into_iter().collect();
    let mut blocks = Vec::new();
    for (k, &start) in leaders_vec.iter().enumerate() {
        let end = if k + 1 < leaders_vec.len() {
            leaders_vec[k + 1]
        } else {
            n
        };
        blocks.push(BasicBlock {
            start,
            end,
            successors: Vec::new(),
        });
    }
    blocks
}
/// Estimate the maximum stack depth required to execute a chunk.
///
/// This is a conservative estimate computed by simulating the abstract
/// stack height through the instruction sequence (ignoring branches).
#[allow(dead_code)]
pub(super) fn estimate_max_stack_depth(chunk: &BytecodeChunk) -> usize {
    let mut depth: i64 = 0;
    let mut max_depth: i64 = 0;
    for op in &chunk.opcodes {
        depth += stack_delta(op);
        if depth > max_depth {
            max_depth = depth;
        }
    }
    max_depth.max(0) as usize
}
/// Compute the net stack height change of a single opcode.
#[allow(dead_code)]
pub(super) fn stack_delta(op: &Opcode) -> i64 {
    match op {
        Opcode::Push(_) | Opcode::PushBool(_) | Opcode::PushStr(_) | Opcode::PushNil => 1,
        Opcode::Pop => -1,
        Opcode::Dup => 1,
        Opcode::Swap => 0,
        Opcode::Add | Opcode::Sub | Opcode::Mul | Opcode::Div | Opcode::Mod => -1,
        Opcode::Eq | Opcode::Lt | Opcode::Le => -1,
        Opcode::Not => 0,
        Opcode::And | Opcode::Or => -1,
        Opcode::Jump(_) => 0,
        Opcode::JumpIf(_) | Opcode::JumpIfNot(_) => -1,
        Opcode::Call(_) => 0,
        Opcode::Return => 0,
        Opcode::Load(_) => 1,
        Opcode::Store(_) => 0,
        Opcode::LoadGlobal(_) => 1,
        Opcode::MakeClosure(n) => -(*n as i64) + 1,
        Opcode::Apply => -1,
        Opcode::Halt => 0,
    }
}
#[cfg(test)]
mod tests_extended {
    use super::*;
    #[test]
    fn test_encode_decode_push() {
        let op = Opcode::Push(12345);
        let enc = EncodedInstruction::encode(&op);
        let (decoded, consumed) =
            EncodedInstruction::decode(&enc.bytes).expect("test operation should succeed");
        assert_eq!(decoded, op);
        assert_eq!(consumed, enc.bytes.len());
    }
    #[test]
    fn test_encode_decode_push_str() {
        let op = Opcode::PushStr("hello world".to_string());
        let enc = EncodedInstruction::encode(&op);
        let (decoded, _) =
            EncodedInstruction::decode(&enc.bytes).expect("test operation should succeed");
        assert_eq!(decoded, op);
    }
    #[test]
    fn test_encode_decode_jump() {
        let op = Opcode::Jump(-10);
        let enc = EncodedInstruction::encode(&op);
        let (decoded, consumed) =
            EncodedInstruction::decode(&enc.bytes).expect("test operation should succeed");
        assert_eq!(decoded, op);
        assert_eq!(consumed, 5);
    }
    #[test]
    fn test_encode_decode_halt() {
        let op = Opcode::Halt;
        let enc = EncodedInstruction::encode(&op);
        let (decoded, consumed) =
            EncodedInstruction::decode(&enc.bytes).expect("test operation should succeed");
        assert_eq!(decoded, op);
        assert_eq!(consumed, 1);
    }
    #[test]
    fn test_serialize_deserialize_chunk() {
        let chunk = BytecodeCompiler::compile_add(3, 4);
        let bytes = serialize_chunk(&chunk);
        let restored = deserialize_chunk(&bytes).expect("test operation should succeed");
        assert_eq!(restored.name, chunk.name);
        assert_eq!(restored.opcodes, chunk.opcodes);
    }
    #[test]
    fn test_disassemble_smoke() {
        let chunk = BytecodeCompiler::compile_nat(99);
        let asm = disassemble(&chunk);
        assert!(asm.contains("PUSH"));
        assert!(asm.contains("HALT"));
    }
    #[test]
    fn test_peephole_push_pop() {
        let mut chunk = BytecodeChunk::new("test");
        chunk.push_op(Opcode::Push(1));
        chunk.push_op(Opcode::Pop);
        chunk.push_op(Opcode::Push(42));
        chunk.push_op(Opcode::Halt);
        let opt = PeepholeOptimizer::new(1).optimize(&chunk);
        assert_eq!(opt.opcodes.len(), 2);
        assert_eq!(opt.opcodes[0], Opcode::Push(42));
    }
    #[test]
    fn test_peephole_const_fold_add() {
        let mut chunk = BytecodeChunk::new("test");
        chunk.push_op(Opcode::Push(10));
        chunk.push_op(Opcode::Push(20));
        chunk.push_op(Opcode::Add);
        chunk.push_op(Opcode::Halt);
        let opt = PeepholeOptimizer::new(1).optimize(&chunk);
        assert_eq!(opt.opcodes[0], Opcode::Push(30));
        assert_eq!(opt.opcodes[1], Opcode::Halt);
    }
    #[test]
    fn test_peephole_bool_and_fold() {
        let mut chunk = BytecodeChunk::new("test");
        chunk.push_op(Opcode::PushBool(true));
        chunk.push_op(Opcode::PushBool(false));
        chunk.push_op(Opcode::And);
        chunk.push_op(Opcode::Halt);
        let opt = PeepholeOptimizer::new(1).optimize(&chunk);
        assert_eq!(opt.opcodes[0], Opcode::PushBool(false));
    }
    #[test]
    fn test_peephole_not_fold() {
        let mut chunk = BytecodeChunk::new("test");
        chunk.push_op(Opcode::PushBool(true));
        chunk.push_op(Opcode::Not);
        chunk.push_op(Opcode::Halt);
        let opt = PeepholeOptimizer::new(1).optimize(&chunk);
        assert_eq!(opt.opcodes[0], Opcode::PushBool(false));
    }
    #[test]
    fn test_handle_call_exact() {
        let r = handle_call(5, 2, vec![StackValue::Nat(1), StackValue::Nat(2)]);
        assert!(matches!(r, CallResult::Exact { fn_pos: 5, .. }));
    }
    #[test]
    fn test_handle_call_partial() {
        let r = handle_call(5, 3, vec![StackValue::Nat(1)]);
        assert!(matches!(
            r,
            CallResult::Partial {
                remaining_arity: 2,
                ..
            }
        ));
    }
    #[test]
    fn test_handle_call_over() {
        let r = handle_call(
            5,
            2,
            vec![StackValue::Nat(1), StackValue::Nat(2), StackValue::Nat(3)],
        );
        if let CallResult::Over { rest_args, .. } = r {
            assert_eq!(rest_args.len(), 1);
        } else {
            panic!("expected Over");
        }
    }
    #[test]
    fn test_profiling_interpreter() {
        let chunk = BytecodeCompiler::compile_add(5, 10);
        let mut pi = ProfilingInterpreter::new();
        let result = pi.execute_chunk(&chunk).expect("execution should succeed");
        assert_eq!(result, StackValue::Nat(15));
        assert!(pi.profile.count("Add") > 0);
        assert!(pi.profile.total_executed > 0);
    }
    #[test]
    fn test_opcode_profile_top_opcodes() {
        let mut p = OpcodeProfile::new();
        p.record(&Opcode::Push(1));
        p.record(&Opcode::Push(2));
        p.record(&Opcode::Add);
        let top = p.top_opcodes();
        assert_eq!(top[0].0, "Push");
        assert_eq!(top[0].1, 2);
    }
    #[test]
    fn test_find_basic_blocks_simple() {
        let chunk = BytecodeCompiler::compile_nat(1);
        let blocks = find_basic_blocks(&chunk);
        assert!(!blocks.is_empty());
    }
    #[test]
    fn test_find_basic_blocks_if() {
        let chunk = BytecodeCompiler::compile_if(true, 1, 2);
        let blocks = find_basic_blocks(&chunk);
        assert!(blocks.len() >= 2);
    }
    #[test]
    fn test_estimate_max_stack_depth_add() {
        let chunk = BytecodeCompiler::compile_add(1, 2);
        let depth = estimate_max_stack_depth(&chunk);
        assert!(depth >= 2);
    }
    #[test]
    fn test_framed_interpreter_push_pop() {
        let mut fi = FramedInterpreter::new();
        fi.push_frame(0, "main")
            .expect("test operation should succeed");
        assert_eq!(fi.depth(), 1);
        let ret = fi.pop_frame().expect("test operation should succeed");
        assert_eq!(ret, 0);
        assert_eq!(fi.depth(), 0);
    }
    #[test]
    fn test_framed_interpreter_max_depth() {
        let mut fi = FramedInterpreter::new().with_max_depth(2);
        fi.push_frame(0, "a")
            .expect("test operation should succeed");
        fi.push_frame(1, "b")
            .expect("test operation should succeed");
        let err = fi.push_frame(2, "c");
        assert!(err.is_err());
    }
    #[test]
    fn test_framed_interpreter_stack_trace() {
        let mut fi = FramedInterpreter::new();
        fi.push_frame(0, "main")
            .expect("test operation should succeed");
        fi.push_frame(10, "helper")
            .expect("test operation should succeed");
        let trace = fi.stack_trace();
        assert!(trace.contains("helper"));
        assert!(trace.contains("main"));
    }
    #[test]
    fn test_opcode_dispatch_table_not_empty() {
        let table = opcode_dispatch_table();
        assert!(!table.is_empty());
        assert!(table.iter().any(|i| i.mnemonic == "HALT"));
    }
    #[test]
    fn test_decode_unknown_tag() {
        let result = EncodedInstruction::decode(&[0xAB]);
        assert!(result.is_none());
    }
    #[test]
    fn test_serialize_empty_chunk() {
        let chunk = BytecodeChunk::new("empty");
        let bytes = serialize_chunk(&chunk);
        let restored = deserialize_chunk(&bytes).expect("test operation should succeed");
        assert_eq!(restored.name, "empty");
        assert!(restored.is_empty());
    }
    #[test]
    fn test_format_op_mnemonic() {
        assert_eq!(format_op(&Opcode::Halt), "HALT");
        assert!(format_op(&Opcode::Push(0)).contains("PUSH"));
        assert!(format_op(&Opcode::LoadGlobal("foo".to_string())).contains("LOAD_GLOBAL"));
    }
}
/// Compute a simplified backward liveness analysis for local variables.
///
/// Only tracks `Load` and `Store` instructions as uses/defs of locals.
#[allow(dead_code)]
pub fn compute_liveness(chunk: &BytecodeChunk) -> LivenessInfo {
    let n = chunk.opcodes.len();
    let mut live_sets: Vec<std::collections::BTreeSet<u32>> = vec![Default::default(); n + 1];
    for i in (0..n).rev() {
        let mut live = live_sets[i + 1].clone();
        match &chunk.opcodes[i] {
            Opcode::Load(idx) => {
                live.insert(*idx);
            }
            Opcode::Store(idx) => {
                live.remove(idx);
            }
            _ => {}
        }
        live_sets[i] = live;
    }
    LivenessInfo {
        live_before: live_sets[..n].to_vec(),
    }
}
#[cfg(test)]
mod tests_phase2 {
    use super::*;
    #[test]
    fn test_constant_folder_add() {
        let mut chunk = BytecodeChunk::new("t");
        chunk.push_op(Opcode::Push(3));
        chunk.push_op(Opcode::Push(4));
        chunk.push_op(Opcode::Add);
        chunk.push_op(Opcode::Halt);
        let folded = ConstantFolder::fold(&chunk);
        let mut interp = Interpreter::new();
        let result = interp
            .execute_chunk(&folded)
            .expect("execution should succeed");
        assert_eq!(result, StackValue::Nat(7));
    }
    #[test]
    fn test_constant_folder_mul() {
        let mut chunk = BytecodeChunk::new("t");
        chunk.push_op(Opcode::Push(6));
        chunk.push_op(Opcode::Push(7));
        chunk.push_op(Opcode::Mul);
        chunk.push_op(Opcode::Halt);
        let folded = ConstantFolder::fold(&chunk);
        let mut interp = Interpreter::new();
        let result = interp
            .execute_chunk(&folded)
            .expect("execution should succeed");
        assert_eq!(result, StackValue::Nat(42));
    }
    #[test]
    fn test_dead_code_elimination_after_halt() {
        let mut chunk = BytecodeChunk::new("t");
        chunk.push_op(Opcode::Push(1));
        chunk.push_op(Opcode::Halt);
        chunk.push_op(Opcode::Push(2));
        chunk.push_op(Opcode::Halt);
        let elim = DeadCodeEliminator::eliminate(&chunk);
        assert_eq!(elim.opcodes.len(), 2);
    }
    #[test]
    fn test_dead_code_elimination_jump() {
        let mut chunk = BytecodeChunk::new("t");
        chunk.push_op(Opcode::Jump(1));
        chunk.push_op(Opcode::Push(99));
        chunk.push_op(Opcode::Push(1));
        chunk.push_op(Opcode::Halt);
        let elim = DeadCodeEliminator::eliminate(&chunk);
        assert!(!elim.opcodes.contains(&Opcode::Push(99)));
    }
    #[test]
    fn test_inline_cache_monomorphic() {
        let mut ic = InlineCache::new();
        ic.record(10, 5);
        ic.record(10, 5);
        ic.record(10, 5);
        let entry = ic.entry(10).expect("test operation should succeed");
        assert!(entry.is_monomorphic);
        assert_eq!(entry.hit_count, 3);
    }
    #[test]
    fn test_inline_cache_polymorphic() {
        let mut ic = InlineCache::new();
        ic.record(10, 5);
        ic.record(10, 6);
        let entry = ic.entry(10).expect("test operation should succeed");
        assert!(!entry.is_monomorphic);
    }
    #[test]
    fn test_inline_cache_monomorphic_sites() {
        let mut ic = InlineCache::new();
        ic.record(0, 1);
        ic.record(5, 2);
        ic.record(5, 3);
        let mono = ic.monomorphic_sites();
        assert!(mono.contains(&0));
        assert!(!mono.contains(&5));
    }
    #[test]
    fn test_liveness_load_store() {
        let mut chunk = BytecodeChunk::new("t");
        chunk.push_op(Opcode::Push(42));
        chunk.push_op(Opcode::Store(0));
        chunk.push_op(Opcode::Load(0));
        chunk.push_op(Opcode::Halt);
        let info = compute_liveness(&chunk);
        assert!(info.live_before[2].contains(&0));
    }
    #[test]
    fn test_chunk_stats() {
        let chunk = BytecodeCompiler::compile_if(true, 10, 20);
        let stats = ChunkStats::compute(&chunk);
        assert!(stats.total_instructions > 0);
        assert!(stats.branch_count > 0);
    }
    #[test]
    fn test_stack_value_display_nat() {
        assert_eq!(format!("{}", StackValue::Nat(42)), "42");
    }
    #[test]
    fn test_stack_value_display_bool() {
        assert_eq!(format!("{}", StackValue::Bool(false)), "false");
    }
    #[test]
    fn test_stack_value_display_nil() {
        assert_eq!(format!("{}", StackValue::Nil), "nil");
    }
    #[test]
    fn test_opcode_profile_reset() {
        let mut p = OpcodeProfile::new();
        p.record(&Opcode::Push(1));
        p.reset();
        assert_eq!(p.total_executed, 0);
        assert_eq!(p.count("Push"), 0);
    }
    #[test]
    fn test_framed_interpreter_reset() {
        let mut fi = FramedInterpreter::new();
        fi.push_frame(0, "f")
            .expect("test operation should succeed");
        fi.stack.push(StackValue::Nat(1));
        fi.reset();
        assert!(fi.stack.is_empty());
        assert_eq!(fi.depth(), 0);
        assert_eq!(fi.ip, 0);
    }
    #[test]
    fn test_frame_at_depth() {
        let mut fi = FramedInterpreter::new();
        fi.push_frame(0, "outer")
            .expect("test operation should succeed");
        fi.push_frame(10, "inner")
            .expect("test operation should succeed");
        assert_eq!(
            fi.frame(0).expect("test operation should succeed").fn_name,
            "inner"
        );
        assert_eq!(
            fi.frame(1).expect("test operation should succeed").fn_name,
            "outer"
        );
    }
    #[test]
    fn test_encode_load_store() {
        let load = Opcode::Load(7);
        let enc = EncodedInstruction::encode(&load);
        let (decoded, _) =
            EncodedInstruction::decode(&enc.bytes).expect("test operation should succeed");
        assert_eq!(decoded, load);
        let store = Opcode::Store(3);
        let enc2 = EncodedInstruction::encode(&store);
        let (decoded2, _) =
            EncodedInstruction::decode(&enc2.bytes).expect("test operation should succeed");
        assert_eq!(decoded2, store);
    }
    #[test]
    fn test_encode_make_closure() {
        let op = Opcode::MakeClosure(5);
        let enc = EncodedInstruction::encode(&op);
        let (decoded, _) =
            EncodedInstruction::decode(&enc.bytes).expect("test operation should succeed");
        assert_eq!(decoded, op);
    }
    #[test]
    fn test_encode_load_global() {
        let op = Opcode::LoadGlobal("Nat.add".to_string());
        let enc = EncodedInstruction::encode(&op);
        let (decoded, _) =
            EncodedInstruction::decode(&enc.bytes).expect("test operation should succeed");
        assert_eq!(decoded, op);
    }
    #[test]
    fn test_chunk_stats_arith() {
        let mut chunk = BytecodeChunk::new("t");
        chunk.push_op(Opcode::Push(1));
        chunk.push_op(Opcode::Push(2));
        chunk.push_op(Opcode::Add);
        chunk.push_op(Opcode::Push(3));
        chunk.push_op(Opcode::Mul);
        chunk.push_op(Opcode::Halt);
        let stats = ChunkStats::compute(&chunk);
        assert_eq!(stats.arith_count, 2);
    }
    #[test]
    fn test_peephole_mul_fold() {
        let mut chunk = BytecodeChunk::new("t");
        chunk.push_op(Opcode::Push(3));
        chunk.push_op(Opcode::Push(3));
        chunk.push_op(Opcode::Mul);
        chunk.push_op(Opcode::Halt);
        let opt = PeepholeOptimizer::new(1).optimize(&chunk);
        assert_eq!(opt.opcodes[0], Opcode::Push(9));
    }
    #[test]
    fn test_peephole_or_fold() {
        let mut chunk = BytecodeChunk::new("t");
        chunk.push_op(Opcode::PushBool(false));
        chunk.push_op(Opcode::PushBool(true));
        chunk.push_op(Opcode::Or);
        chunk.push_op(Opcode::Halt);
        let opt = PeepholeOptimizer::new(1).optimize(&chunk);
        assert_eq!(opt.opcodes[0], Opcode::PushBool(true));
    }
    #[test]
    fn test_deserialize_invalid_data() {
        assert!(deserialize_chunk(&[]).is_none());
        assert!(deserialize_chunk(&[0xFF, 0xFF]).is_none());
    }
}
#[cfg(test)]
mod tests_builder {
    use super::*;
    #[test]
    fn test_chunk_builder_basic() {
        let chunk = ChunkBuilder::new("test")
            .push_nat(10)
            .push_nat(20)
            .add()
            .halt()
            .build();
        let mut interp = Interpreter::new();
        let result = interp
            .execute_chunk(&chunk)
            .expect("execution should succeed");
        assert_eq!(result, StackValue::Nat(30));
    }
    #[test]
    fn test_chunk_builder_sub() {
        let chunk = ChunkBuilder::new("sub")
            .push_nat(100)
            .push_nat(58)
            .sub()
            .halt()
            .build();
        let mut interp = Interpreter::new();
        let result = interp
            .execute_chunk(&chunk)
            .expect("execution should succeed");
        assert_eq!(result, StackValue::Nat(42));
    }
    #[test]
    fn test_chunk_builder_mul() {
        let chunk = ChunkBuilder::new("mul")
            .push_nat(6)
            .push_nat(7)
            .mul()
            .halt()
            .build();
        let mut interp = Interpreter::new();
        let result = interp
            .execute_chunk(&chunk)
            .expect("execution should succeed");
        assert_eq!(result, StackValue::Nat(42));
    }
    #[test]
    fn test_chunk_builder_div() {
        let chunk = ChunkBuilder::new("div")
            .push_nat(84)
            .push_nat(2)
            .div()
            .halt()
            .build();
        let mut interp = Interpreter::new();
        let result = interp
            .execute_chunk(&chunk)
            .expect("execution should succeed");
        assert_eq!(result, StackValue::Nat(42));
    }
    #[test]
    fn test_chunk_builder_mod() {
        let chunk = ChunkBuilder::new("mod")
            .push_nat(43)
            .push_nat(10)
            .modulo()
            .halt()
            .build();
        let mut interp = Interpreter::new();
        let result = interp
            .execute_chunk(&chunk)
            .expect("execution should succeed");
        assert_eq!(result, StackValue::Nat(3));
    }
    #[test]
    fn test_chunk_builder_bool() {
        let chunk = ChunkBuilder::new("bool")
            .push_bool(true)
            .not()
            .halt()
            .build();
        let mut interp = Interpreter::new();
        let result = interp
            .execute_chunk(&chunk)
            .expect("execution should succeed");
        assert_eq!(result, StackValue::Bool(false));
    }
    #[test]
    fn test_chunk_builder_current_ip() {
        let b = ChunkBuilder::new("t").push_nat(1).push_nat(2);
        assert_eq!(b.current_ip(), 2);
    }
    #[test]
    fn test_chunk_builder_load_store() {
        let chunk = ChunkBuilder::new("ls")
            .push_nat(99)
            .store(0)
            .load(0)
            .halt()
            .build();
        let mut interp = Interpreter::new();
        let result = interp
            .execute_chunk(&chunk)
            .expect("execution should succeed");
        assert_eq!(result, StackValue::Nat(99));
    }
    #[test]
    fn test_chunk_builder_swap() {
        let chunk = ChunkBuilder::new("swap")
            .push_nat(1)
            .push_nat(2)
            .swap()
            .halt()
            .build();
        let mut interp = Interpreter::new();
        let result = interp
            .execute_chunk(&chunk)
            .expect("execution should succeed");
        assert_eq!(result, StackValue::Nat(1));
    }
    #[test]
    fn test_chunk_builder_dup() {
        let chunk = ChunkBuilder::new("dup")
            .push_nat(7)
            .dup()
            .add()
            .halt()
            .build();
        let mut interp = Interpreter::new();
        let result = interp
            .execute_chunk(&chunk)
            .expect("execution should succeed");
        assert_eq!(result, StackValue::Nat(14));
    }
    #[test]
    fn test_chunk_builder_eq() {
        let chunk = ChunkBuilder::new("eq")
            .push_nat(5)
            .push_nat(5)
            .eq()
            .halt()
            .build();
        let mut interp = Interpreter::new();
        let result = interp
            .execute_chunk(&chunk)
            .expect("execution should succeed");
        assert_eq!(result, StackValue::Bool(true));
    }
    #[test]
    fn test_chunk_builder_le() {
        let chunk = ChunkBuilder::new("le")
            .push_nat(3)
            .push_nat(3)
            .le()
            .halt()
            .build();
        let mut interp = Interpreter::new();
        let result = interp
            .execute_chunk(&chunk)
            .expect("execution should succeed");
        assert_eq!(result, StackValue::Bool(true));
    }
    #[test]
    fn test_chunk_builder_lt() {
        let chunk = ChunkBuilder::new("lt")
            .push_nat(2)
            .push_nat(5)
            .lt()
            .halt()
            .build();
        let mut interp = Interpreter::new();
        let result = interp
            .execute_chunk(&chunk)
            .expect("execution should succeed");
        assert_eq!(result, StackValue::Bool(true));
    }
    #[test]
    fn test_chunk_builder_and() {
        let chunk = ChunkBuilder::new("and")
            .push_bool(true)
            .push_bool(true)
            .and()
            .halt()
            .build();
        let mut interp = Interpreter::new();
        let result = interp
            .execute_chunk(&chunk)
            .expect("execution should succeed");
        assert_eq!(result, StackValue::Bool(true));
    }
    #[test]
    fn test_chunk_builder_or() {
        let chunk = ChunkBuilder::new("or")
            .push_bool(false)
            .push_bool(true)
            .or()
            .halt()
            .build();
        let mut interp = Interpreter::new();
        let result = interp
            .execute_chunk(&chunk)
            .expect("execution should succeed");
        assert_eq!(result, StackValue::Bool(true));
    }
    #[test]
    fn test_stack_delta_push() {
        assert_eq!(stack_delta(&Opcode::Push(0)), 1);
        assert_eq!(stack_delta(&Opcode::PushBool(false)), 1);
        assert_eq!(stack_delta(&Opcode::Pop), -1);
        assert_eq!(stack_delta(&Opcode::Dup), 1);
        assert_eq!(stack_delta(&Opcode::Swap), 0);
        assert_eq!(stack_delta(&Opcode::Halt), 0);
    }
    #[test]
    fn test_stack_delta_arithmetic() {
        assert_eq!(stack_delta(&Opcode::Add), -1);
        assert_eq!(stack_delta(&Opcode::Sub), -1);
        assert_eq!(stack_delta(&Opcode::Mul), -1);
        assert_eq!(stack_delta(&Opcode::Div), -1);
        assert_eq!(stack_delta(&Opcode::Mod), -1);
    }
    #[test]
    fn test_stack_delta_jumps() {
        assert_eq!(stack_delta(&Opcode::JumpIf(0)), -1);
        assert_eq!(stack_delta(&Opcode::JumpIfNot(0)), -1);
        assert_eq!(stack_delta(&Opcode::Jump(0)), 0);
    }
    #[test]
    fn test_disassemble_if_chunk() {
        let chunk = BytecodeCompiler::compile_if(true, 5, 10);
        let asm = disassemble(&chunk);
        assert!(asm.contains("JUMP_IFNOT"));
        assert!(asm.contains("JUMP"));
    }
    #[test]
    fn test_constant_folder_not() {
        let mut chunk = BytecodeChunk::new("t");
        chunk.push_op(Opcode::PushBool(false));
        chunk.push_op(Opcode::Not);
        chunk.push_op(Opcode::Halt);
        let folded = ConstantFolder::fold(&chunk);
        let mut interp = Interpreter::new();
        let result = interp
            .execute_chunk(&folded)
            .expect("execution should succeed");
        assert_eq!(result, StackValue::Bool(true));
    }
}
