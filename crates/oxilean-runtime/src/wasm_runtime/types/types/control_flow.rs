//! Control-flow, call dispatch, and WasmModule::call_function implementation
//! for the WASM stack machine.
//!
//! This module is a continuation of `impls.rs` split off to stay under 2000
//! lines per file.

use super::defs::{
    CallFrame, ExecError, LabelFrame, LabelKind, StackMachine, WasmFunction, WasmInstruction,
    WasmModule, WasmValue,
};

impl StackMachine {
    /// Scan forward from `start` counting depth to find the matching `End`.
    /// `start` is the index of a Block/Loop/If instruction.
    /// Returns the index of the matching End.
    pub(crate) fn find_matching_end(instrs: &[WasmInstruction], start: usize) -> Option<usize> {
        let mut depth: usize = 0;
        for (i, instr) in instrs.iter().enumerate().skip(start) {
            match instr {
                WasmInstruction::Block { .. }
                | WasmInstruction::Loop { .. }
                | WasmInstruction::If { .. } => depth += 1,
                WasmInstruction::End => {
                    if depth == 1 {
                        return Some(i);
                    }
                    depth -= 1;
                }
                _ => {}
            }
        }
        None
    }

    /// Scan forward from `start` (the If instruction's index) to find the
    /// matching Else at depth 1, if any. Returns the index of the Else.
    pub(crate) fn find_matching_else(instrs: &[WasmInstruction], start: usize) -> Option<usize> {
        let mut depth: usize = 0;
        for (i, instr) in instrs.iter().enumerate().skip(start) {
            match instr {
                WasmInstruction::Block { .. }
                | WasmInstruction::Loop { .. }
                | WasmInstruction::If { .. } => depth += 1,
                WasmInstruction::Else => {
                    if depth == 1 {
                        return Some(i);
                    }
                }
                WasmInstruction::End => {
                    if depth == 1 {
                        return None;
                    }
                    depth -= 1;
                }
                _ => {}
            }
        }
        None
    }

    /// Perform a branch to label at `depth` levels up from the top.
    /// Returns the new pc to resume at (caller sets pc = returned value).
    /// Also trims the value stack to the label's arity.
    pub(crate) fn branch(&mut self, depth: u32) -> Result<usize, ExecError> {
        let stack_len = self.label_stack.len();
        let frame_idx = stack_len.checked_sub(1 + depth as usize).ok_or_else(|| {
            ExecError::Custom(format!(
                "branch depth {depth} exceeds label stack depth {stack_len}"
            ))
        })?;
        let frame = self.label_stack[frame_idx].clone();
        let is_loop = matches!(frame.kind, LabelKind::Loop);
        let arity = frame.arity as usize;
        let stack_depth = self.stack.len();
        let results: Vec<WasmValue> = if stack_depth >= arity {
            self.stack.drain(stack_depth - arity..).collect()
        } else {
            return Err(ExecError::StackUnderflow);
        };
        self.stack.truncate(frame.stack_base);
        self.stack.extend(results);
        self.label_stack.truncate(frame_idx + 1);
        if !is_loop {
            self.label_stack.pop();
        }
        Ok(frame.branch_target)
    }

    /// Full PC-based dispatch loop for a function body.
    ///
    /// Handles all control-flow instructions; delegates arithmetic/memory
    /// instructions to `execute_one`.
    pub fn execute_function(
        &mut self,
        module: &WasmModule,
        body: &[WasmInstruction],
    ) -> Result<(), ExecError> {
        const MAX_CALL_DEPTH: usize = 256;
        let mut pc: usize = 0;
        loop {
            if pc >= body.len() {
                break;
            }
            let instr = &body[pc];
            match instr {
                // ── Structured control flow ─────────────────────────────
                WasmInstruction::Block { ty } => {
                    let arity: u32 = if ty.is_some() { 1 } else { 0 };
                    let end_pc = Self::find_matching_end(body, pc)
                        .ok_or_else(|| ExecError::Custom(format!("unmatched Block at pc {pc}")))?;
                    self.label_stack.push(LabelFrame {
                        kind: LabelKind::Block,
                        arity,
                        branch_target: end_pc + 1,
                        stack_base: self.stack.len(),
                    });
                    pc += 1;
                }
                WasmInstruction::Loop { ty } => {
                    let arity: u32 = if ty.is_some() { 1 } else { 0 };
                    self.label_stack.push(LabelFrame {
                        kind: LabelKind::Loop,
                        arity,
                        branch_target: pc,
                        stack_base: self.stack.len(),
                    });
                    pc += 1;
                }
                WasmInstruction::If { ty } => {
                    let arity: u32 = if ty.is_some() { 1 } else { 0 };
                    let cond = self.pop_i32()?;
                    let else_pc = Self::find_matching_else(body, pc);
                    let end_pc = Self::find_matching_end(body, pc)
                        .ok_or_else(|| ExecError::Custom(format!("unmatched If at pc {pc}")))?;
                    if cond != 0 {
                        self.label_stack.push(LabelFrame {
                            kind: LabelKind::IfThen { else_pc },
                            arity,
                            branch_target: end_pc + 1,
                            stack_base: self.stack.len(),
                        });
                        pc += 1;
                    } else {
                        match else_pc {
                            Some(ep) => {
                                self.label_stack.push(LabelFrame {
                                    kind: LabelKind::IfThen { else_pc: Some(ep) },
                                    arity,
                                    branch_target: end_pc + 1,
                                    stack_base: self.stack.len(),
                                });
                                pc = ep + 1;
                            }
                            None => {
                                pc = end_pc + 1;
                            }
                        }
                    }
                }
                WasmInstruction::Else => {
                    let frame = self.label_stack.last().ok_or_else(|| {
                        ExecError::Custom("Else with empty label stack".to_string())
                    })?;
                    let end_target = frame.branch_target;
                    let arity = frame.arity as usize;
                    let stack_depth = self.stack.len();
                    let results: Vec<WasmValue> = if stack_depth >= arity {
                        self.stack.drain(stack_depth - arity..).collect()
                    } else {
                        return Err(ExecError::StackUnderflow);
                    };
                    let base = frame.stack_base;
                    self.stack.truncate(base);
                    self.stack.extend(results);
                    self.label_stack.pop();
                    pc = end_target;
                }
                WasmInstruction::End => {
                    if let Some(frame) = self.label_stack.pop() {
                        let arity = frame.arity as usize;
                        let stack_depth = self.stack.len();
                        let results: Vec<WasmValue> = if stack_depth >= arity {
                            self.stack.drain(stack_depth - arity..).collect()
                        } else {
                            return Err(ExecError::StackUnderflow);
                        };
                        self.stack.truncate(frame.stack_base);
                        self.stack.extend(results);
                    }
                    pc += 1;
                }
                WasmInstruction::Br(depth) => {
                    pc = self.branch(*depth)?;
                }
                WasmInstruction::BrIf(depth) => {
                    let cond = self.pop_i32()?;
                    if cond != 0 {
                        pc = self.branch(*depth)?;
                    } else {
                        pc += 1;
                    }
                }
                WasmInstruction::BrTable { targets, default } => {
                    let idx = self.pop_i32()? as usize;
                    let depth = if idx < targets.len() {
                        targets[idx]
                    } else {
                        *default
                    };
                    pc = self.branch(depth)?;
                }
                WasmInstruction::Return => {
                    let depth = self.label_stack.len() as u32;
                    if depth > 0 {
                        let stack_base = self.label_stack[0].stack_base;
                        let results: Vec<WasmValue> = self.stack.drain(stack_base..).collect();
                        self.stack.truncate(stack_base);
                        self.stack.extend(results);
                        self.label_stack.clear();
                    }
                    pc = body.len();
                }
                // ── Function calls ───────────────────────────────────────
                WasmInstruction::Call(func_idx) => {
                    let func_name = module
                        .table
                        .get(*func_idx as usize)
                        .ok_or_else(|| {
                            ExecError::Custom(format!(
                                "Call: no function at table index {func_idx}"
                            ))
                        })?
                        .to_owned();
                    if self.call_stack.len() >= MAX_CALL_DEPTH {
                        return Err(ExecError::CallStackOverflow);
                    }
                    let callee = module.functions.get(&func_name).ok_or_else(|| {
                        ExecError::Custom(format!("Call: function `{func_name}` not in module"))
                    })?;
                    let param_count = callee.locals.len();
                    let stack_len = self.stack.len();
                    if stack_len < param_count {
                        return Err(ExecError::StackUnderflow);
                    }
                    let mut callee_locals: Vec<WasmValue> =
                        self.stack.drain(stack_len - param_count..).collect();
                    for ty in callee.locals.iter().skip(param_count) {
                        callee_locals.push(ty.default_value());
                    }
                    let callee_body = callee.body.clone();
                    self.call_stack.push(CallFrame {
                        saved_locals: std::mem::replace(&mut self.locals, callee_locals),
                        return_pc: pc + 1,
                        label_base: self.label_stack.len(),
                        value_base: self.stack.len(),
                        func_name: func_name.clone(),
                    });
                    self.execute_function(module, &callee_body)?;
                    // Restore caller state after callee returns.
                    if let Some(frame) = self.call_stack.pop() {
                        self.locals = frame.saved_locals;
                        self.label_stack.truncate(frame.label_base);
                    }
                    pc += 1;
                }
                WasmInstruction::CallIndirect { table_idx, .. } => {
                    let runtime_idx = self.pop_i32()? as usize;
                    let func_name = module
                        .table
                        .get(runtime_idx)
                        .ok_or_else(|| {
                            ExecError::Custom(format!(
                                "CallIndirect: no function at table index {runtime_idx}"
                            ))
                        })?
                        .to_owned();
                    let _ = table_idx;
                    if self.call_stack.len() >= MAX_CALL_DEPTH {
                        return Err(ExecError::CallStackOverflow);
                    }
                    let callee = module.functions.get(&func_name).ok_or_else(|| {
                        ExecError::Custom(format!(
                            "CallIndirect: function `{func_name}` not in module"
                        ))
                    })?;
                    let param_count = callee.locals.len();
                    let stack_len = self.stack.len();
                    if stack_len < param_count {
                        return Err(ExecError::StackUnderflow);
                    }
                    let mut callee_locals: Vec<WasmValue> =
                        self.stack.drain(stack_len - param_count..).collect();
                    for ty in callee.locals.iter().skip(param_count) {
                        callee_locals.push(ty.default_value());
                    }
                    let callee_body = callee.body.clone();
                    self.call_stack.push(CallFrame {
                        saved_locals: std::mem::replace(&mut self.locals, callee_locals),
                        return_pc: pc + 1,
                        label_base: self.label_stack.len(),
                        value_base: self.stack.len(),
                        func_name: func_name.clone(),
                    });
                    self.execute_function(module, &callee_body)?;
                    if let Some(frame) = self.call_stack.pop() {
                        self.locals = frame.saved_locals;
                        self.label_stack.truncate(frame.label_base);
                    }
                    pc += 1;
                }
                // ── All other instructions: delegate to execute_one ──────
                other => {
                    self.execute_one(other)?;
                    pc += 1;
                }
            }
        }
        Ok(())
    }
}

/// Helper for tests: build a `WasmModule` with a single registered function.
///
/// `param_types` describes the parameter types (which become locals[0..n]).
/// The function is registered both under `name` and placed in `table[0]`
/// for index-based `Call(0)` tests.
#[cfg(test)]
pub(crate) fn make_module_with(
    name: &str,
    param_types: Vec<super::defs::WasmType>,
    body: Vec<WasmInstruction>,
) -> WasmModule {
    use super::defs::WasmType;
    let mut module = WasmModule::new("test", 1);
    let mut func = WasmFunction::new(name, 0);
    for ty in &param_types {
        func.add_local(ty.clone());
    }
    func.body = body;
    module.functions.insert(name.to_string(), func);
    // Also place at table slot 0 for Call(0) style tests.
    module.table.set(0, name);
    module
}

#[cfg(test)]
mod tests {
    use super::super::defs::{
        ExecError, StackMachine, WasmFunction, WasmInstruction, WasmModule, WasmType, WasmValue,
    };
    use super::*;

    // ─── D1.a: WasmModule.functions registry ────────────────────────────────

    #[test]
    fn test_register_and_lookup_function() {
        let mut module = WasmModule::new("m", 1);
        let mut func = WasmFunction::new("add", 0);
        func.add_instruction(WasmInstruction::I32Const(42));
        let body_snapshot = func.body.clone();
        module.register_function("add".to_string(), func);
        let found = module.functions.get("add").expect("function should exist");
        assert_eq!(found.body, body_snapshot);
    }

    #[test]
    fn test_call_function_with_body() {
        // Register `const42` that ignores args and returns 42.
        let module = make_module_with("const42", vec![], vec![WasmInstruction::I32Const(42)]);
        let result = module
            .call_function("const42", &[])
            .expect("call should succeed");
        assert_eq!(result, vec![WasmValue::I32(42)]);
    }

    // ─── D1.b: Block / Loop / If / Else / End ───────────────────────────────

    #[test]
    fn test_block_leaves_result() {
        // block (result i32)
        //   i32.const 7
        // end
        let module = make_module_with(
            "f",
            vec![],
            vec![
                WasmInstruction::Block {
                    ty: Some(WasmType::I32),
                },
                WasmInstruction::I32Const(7),
                WasmInstruction::End,
            ],
        );
        let result = module.call_function("f", &[]).expect("call ok");
        assert_eq!(result, vec![WasmValue::I32(7)]);
    }

    #[test]
    fn test_nested_block_stack_depth() {
        // Outer block (result i32) contains inner block (result i32).
        // Inner block pushes 3 and ends. Outer block also has result i32.
        // After inner End, 3 is on stack. Outer End keeps 3 (arity=1).
        let module = make_module_with(
            "f",
            vec![],
            vec![
                WasmInstruction::Block {
                    ty: Some(WasmType::I32),
                }, // outer: result i32
                WasmInstruction::Block {
                    ty: Some(WasmType::I32),
                }, // inner: result i32
                WasmInstruction::I32Const(3),
                WasmInstruction::End, // end inner: leaves 3 on stack
                WasmInstruction::End, // end outer: keeps 3 (arity=1)
            ],
        );
        let result = module.call_function("f", &[]).expect("call ok");
        assert_eq!(result, vec![WasmValue::I32(3)]);
    }

    #[test]
    fn test_if_then_branch_taken() {
        // if (condition=1) → push 10; else → push 20; end
        let module = make_module_with(
            "f",
            vec![],
            vec![
                WasmInstruction::I32Const(1),
                WasmInstruction::If {
                    ty: Some(WasmType::I32),
                },
                WasmInstruction::I32Const(10),
                WasmInstruction::Else,
                WasmInstruction::I32Const(20),
                WasmInstruction::End,
            ],
        );
        let result = module.call_function("f", &[]).expect("call ok");
        assert_eq!(result, vec![WasmValue::I32(10)]);
    }

    #[test]
    fn test_if_else_branch_taken() {
        // if (condition=0) → push 10; else → push 20; end
        let module = make_module_with(
            "f",
            vec![],
            vec![
                WasmInstruction::I32Const(0),
                WasmInstruction::If {
                    ty: Some(WasmType::I32),
                },
                WasmInstruction::I32Const(10),
                WasmInstruction::Else,
                WasmInstruction::I32Const(20),
                WasmInstruction::End,
            ],
        );
        let result = module.call_function("f", &[]).expect("call ok");
        assert_eq!(result, vec![WasmValue::I32(20)]);
    }

    #[test]
    fn test_if_no_else_falsy() {
        // if (condition=0, no else) → nothing pushed
        let module = make_module_with(
            "f",
            vec![],
            vec![
                WasmInstruction::I32Const(0),
                WasmInstruction::If { ty: None },
                WasmInstruction::I32Const(99),
                WasmInstruction::End,
                WasmInstruction::I32Const(5),
            ],
        );
        let result = module.call_function("f", &[]).expect("call ok");
        assert_eq!(result, vec![WasmValue::I32(5)]);
    }

    #[test]
    fn test_loop_with_counter() {
        // Counts from 0 to 3 using a loop, then leaves 3 on stack.
        // local[0] = counter (starts at 0)
        // loop:
        //   local[0] += 1
        //   if local[0] < 3: br 0 (loop back)
        // end
        // push local[0]
        let module = make_module_with(
            "f",
            vec![WasmType::I32],
            vec![
                // local.set 0 = 0 (already 0 by default)
                WasmInstruction::Loop { ty: None },
                WasmInstruction::LocalGet(0),
                WasmInstruction::I32Const(1),
                WasmInstruction::I32Add,
                WasmInstruction::LocalSet(0),
                WasmInstruction::LocalGet(0),
                WasmInstruction::I32Const(3),
                WasmInstruction::I32LtS,
                WasmInstruction::BrIf(0),
                WasmInstruction::End,
                WasmInstruction::LocalGet(0),
            ],
        );
        let result = module
            .call_function("f", &[WasmValue::I32(0)])
            .expect("call ok");
        assert_eq!(result, vec![WasmValue::I32(3)]);
    }

    // ─── D1.c: Br / BrIf / BrTable / Return ────────────────────────────────

    #[test]
    fn test_br_breaks_out_of_block() {
        // block
        //   i32.const 1
        //   br 0         ;; break out of block, leaving 1 on stack
        //   i32.const 2  ;; should not execute
        // end
        let module = make_module_with(
            "f",
            vec![],
            vec![
                WasmInstruction::Block {
                    ty: Some(WasmType::I32),
                },
                WasmInstruction::I32Const(1),
                WasmInstruction::Br(0),
                WasmInstruction::I32Const(2),
                WasmInstruction::End,
            ],
        );
        let result = module.call_function("f", &[]).expect("call ok");
        assert_eq!(result, vec![WasmValue::I32(1)]);
    }

    #[test]
    fn test_brif_skips_on_zero() {
        // block
        //   i32.const 5
        //   i32.const 0  ;; condition = false
        //   br_if 0      ;; should NOT branch
        //   i32.const 2  ;; should replace 5
        // end
        // Note: after br_if consumes condition, 5 is still on stack.
        // Then 2 is pushed. Block result pops to arity=0, leaving [5, 2].
        // We just check the top.
        let module = make_module_with(
            "f",
            vec![],
            vec![
                WasmInstruction::Block { ty: None },
                WasmInstruction::I32Const(0),
                WasmInstruction::BrIf(0),
                WasmInstruction::End,
                WasmInstruction::I32Const(99),
            ],
        );
        let result = module.call_function("f", &[]).expect("call ok");
        // br_if did not branch; 99 was pushed after block
        assert_eq!(result, vec![WasmValue::I32(99)]);
    }

    #[test]
    fn test_brif_branches_on_nonzero() {
        // br_if with condition=1 should branch, leaving value before End.
        let module = make_module_with(
            "f",
            vec![],
            vec![
                WasmInstruction::Block { ty: None },
                WasmInstruction::I32Const(1),
                WasmInstruction::BrIf(0),
                WasmInstruction::Unreachable,
                WasmInstruction::End,
                WasmInstruction::I32Const(42),
            ],
        );
        let result = module.call_function("f", &[]).expect("call ok");
        assert_eq!(result, vec![WasmValue::I32(42)]);
    }

    #[test]
    fn test_brtable_selects_target() {
        // br_table [label0, label1] default=label0
        // With index 1 → branch to label1 (outer block).
        let module = make_module_with(
            "f",
            vec![],
            vec![
                WasmInstruction::Block {
                    ty: Some(WasmType::I32),
                }, // label1 (depth 0 from inside inner)
                WasmInstruction::Block {
                    ty: Some(WasmType::I32),
                }, // label0 (depth 0 from br_table)
                WasmInstruction::I32Const(10),
                WasmInstruction::I32Const(1), // index = 1
                WasmInstruction::BrTable {
                    targets: vec![0, 1],
                    default: 0,
                },
                WasmInstruction::End,          // end label0
                WasmInstruction::I32Const(99), // should not execute
                WasmInstruction::End,          // end label1
            ],
        );
        let result = module.call_function("f", &[]).expect("call ok");
        // index=1 → br to depth 1 (outer block) → leaves 10 on stack
        assert_eq!(result, vec![WasmValue::I32(10)]);
    }

    #[test]
    fn test_return_exits_early() {
        // return before a second push → only first value on stack
        let module = make_module_with(
            "f",
            vec![],
            vec![
                WasmInstruction::I32Const(7),
                WasmInstruction::Return,
                WasmInstruction::I32Const(99), // dead code
            ],
        );
        let result = module.call_function("f", &[]).expect("call ok");
        assert_eq!(result, vec![WasmValue::I32(7)]);
    }

    // ─── D1.d: Call / CallIndirect / WasmModule::call_function ─────────────

    #[test]
    fn test_call_simple_add() {
        // Function `add` takes two i32 locals and returns local[0] + local[1].
        let mut module = WasmModule::new("m", 1);
        let mut add = WasmFunction::new("add", 0);
        add.add_local(WasmType::I32);
        add.add_local(WasmType::I32);
        add.body = vec![
            WasmInstruction::LocalGet(0),
            WasmInstruction::LocalGet(1),
            WasmInstruction::I32Add,
        ];
        module.functions.insert("add".to_string(), add);
        // Direct call via call_function API.
        let result = module
            .call_function("add", &[WasmValue::I32(3), WasmValue::I32(4)])
            .expect("call ok");
        assert_eq!(result, vec![WasmValue::I32(7)]);
    }

    #[test]
    fn test_call_function_out_of_bounds_error() {
        let module = WasmModule::new("m", 1);
        let err = module.call_function("nonexistent", &[]);
        assert!(err.is_err());
        let msg = err.unwrap_err();
        assert!(msg.contains("nonexistent"));
    }

    #[test]
    fn test_call_factorial_recursive() {
        // factorial(n):
        //   local[0] = n
        //   if n == 0: return 1
        //   else: return n * factorial(n-1)
        //
        // WASM body:
        //   local.get 0
        //   i32.const 0
        //   i32.eq
        //   if (result i32)
        //     i32.const 1
        //   else
        //     local.get 0
        //     local.get 0
        //     i32.const 1
        //     i32.sub
        //     call 0         ;; table[0] = "factorial"
        //     i32.mul
        //   end
        let mut module = WasmModule::new("m", 1);
        let mut fact = WasmFunction::new("factorial", 0);
        fact.add_local(WasmType::I32);
        fact.body = vec![
            WasmInstruction::LocalGet(0),
            WasmInstruction::I32Const(0),
            WasmInstruction::I32Eq,
            WasmInstruction::If {
                ty: Some(WasmType::I32),
            },
            WasmInstruction::I32Const(1),
            WasmInstruction::Else,
            WasmInstruction::LocalGet(0),
            WasmInstruction::LocalGet(0),
            WasmInstruction::I32Const(1),
            WasmInstruction::I32Sub,
            WasmInstruction::Call(0), // factorial(n-1)
            WasmInstruction::I32Mul,
            WasmInstruction::End,
        ];
        module.functions.insert("factorial".to_string(), fact);
        module.table.set(0, "factorial");

        let result = module
            .call_function("factorial", &[WasmValue::I32(5)])
            .expect("factorial(5) should succeed");
        assert_eq!(result, vec![WasmValue::I32(120)]);
    }

    // ─── D1.e: Memory round-trip ─────────────────────────────────────────────

    #[test]
    fn test_memory_store_and_load_round_trip() {
        // Store 12345 at address 0, then load it back.
        let module = make_module_with(
            "f",
            vec![],
            vec![
                WasmInstruction::I32Const(0),
                WasmInstruction::I32Const(12345),
                WasmInstruction::I32Store {
                    align: 2,
                    offset: 0,
                },
                WasmInstruction::I32Const(0),
                WasmInstruction::I32Load {
                    align: 2,
                    offset: 0,
                },
            ],
        );
        let result = module.call_function("f", &[]).expect("call ok");
        assert_eq!(result, vec![WasmValue::I32(12345)]);
    }
}
