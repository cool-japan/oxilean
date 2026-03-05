//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{BTreeSet, HashMap};

use super::functions::{estimate_max_stack_depth, opcode_kind};

/// Statistics about a bytecode chunk.
#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
pub struct ChunkStats {
    /// Total number of instructions.
    pub total_instructions: usize,
    /// Number of branch instructions.
    pub branch_count: usize,
    /// Number of arithmetic instructions.
    pub arith_count: usize,
    /// Number of load/store instructions.
    pub mem_count: usize,
    /// Number of call/return instructions.
    pub call_count: usize,
    /// Estimated max stack depth.
    pub max_stack_depth: usize,
}
impl ChunkStats {
    /// Compute statistics for a chunk.
    pub fn compute(chunk: &BytecodeChunk) -> Self {
        let mut stats = ChunkStats::default();
        stats.total_instructions = chunk.opcodes.len();
        stats.max_stack_depth = estimate_max_stack_depth(chunk);
        for op in &chunk.opcodes {
            match op {
                Opcode::Jump(_) | Opcode::JumpIf(_) | Opcode::JumpIfNot(_) => {
                    stats.branch_count += 1;
                }
                Opcode::Add
                | Opcode::Sub
                | Opcode::Mul
                | Opcode::Div
                | Opcode::Mod
                | Opcode::Eq
                | Opcode::Lt
                | Opcode::Le
                | Opcode::Not
                | Opcode::And
                | Opcode::Or => {
                    stats.arith_count += 1;
                }
                Opcode::Load(_) | Opcode::Store(_) | Opcode::LoadGlobal(_) => {
                    stats.mem_count += 1;
                }
                Opcode::Call(_) | Opcode::Return => {
                    stats.call_count += 1;
                }
                _ => {}
            }
        }
        stats
    }
}
/// A saved call frame for nested function calls.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CallFrame {
    /// The return instruction pointer.
    pub return_ip: usize,
    /// The base of the local variable array for this frame.
    pub locals_base: usize,
    /// Name of the function being called (for diagnostics).
    pub fn_name: String,
}
impl CallFrame {
    /// Create a new call frame.
    pub fn new(return_ip: usize, locals_base: usize, fn_name: impl Into<String>) -> Self {
        CallFrame {
            return_ip,
            locals_base,
            fn_name: fn_name.into(),
        }
    }
}
/// A simple inline cache entry for a call site.
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct InlineCacheEntry {
    /// Opcode index of the call site.
    pub call_ip: usize,
    /// Number of times this call site has been observed.
    pub hit_count: u64,
    /// The most recently observed target function position.
    pub last_target: Option<u32>,
    /// Whether this entry is considered monomorphic.
    pub is_monomorphic: bool,
}
impl InlineCacheEntry {
    /// Create a new IC entry for a given call site.
    pub fn new(call_ip: usize) -> Self {
        InlineCacheEntry {
            call_ip,
            hit_count: 0,
            last_target: None,
            is_monomorphic: true,
        }
    }
    /// Record a call to `target_fn`.
    pub fn record_call(&mut self, target_fn: u32) {
        self.hit_count += 1;
        match self.last_target {
            None => {
                self.last_target = Some(target_fn);
            }
            Some(prev) if prev != target_fn => {
                self.is_monomorphic = false;
            }
            _ => {}
        }
    }
}
/// A DSL-style builder for constructing bytecode chunks.
///
/// Allows fluent construction of bytecode sequences without manually
/// managing jump offsets.
pub struct ChunkBuilder {
    chunk: BytecodeChunk,
}
impl ChunkBuilder {
    /// Create a new builder for a chunk with the given name.
    pub fn new(name: &str) -> Self {
        ChunkBuilder {
            chunk: BytecodeChunk::new(name),
        }
    }
    /// Emit a `Push(n)` instruction.
    pub fn push_nat(mut self, n: u64) -> Self {
        self.chunk.push_op(Opcode::Push(n));
        self
    }
    /// Emit a `PushBool(b)` instruction.
    pub fn push_bool(mut self, b: bool) -> Self {
        self.chunk.push_op(Opcode::PushBool(b));
        self
    }
    /// Emit a `PushStr(s)` instruction.
    pub fn push_str(mut self, s: &str) -> Self {
        self.chunk.push_op(Opcode::PushStr(s.to_string()));
        self
    }
    /// Emit `PushNil`.
    pub fn push_nil(mut self) -> Self {
        self.chunk.push_op(Opcode::PushNil);
        self
    }
    /// Emit `Add`.
    pub fn add(mut self) -> Self {
        self.chunk.push_op(Opcode::Add);
        self
    }
    /// Emit `Sub`.
    pub fn sub(mut self) -> Self {
        self.chunk.push_op(Opcode::Sub);
        self
    }
    /// Emit `Mul`.
    pub fn mul(mut self) -> Self {
        self.chunk.push_op(Opcode::Mul);
        self
    }
    /// Emit `Div`.
    pub fn div(mut self) -> Self {
        self.chunk.push_op(Opcode::Div);
        self
    }
    /// Emit `Mod`.
    pub fn modulo(mut self) -> Self {
        self.chunk.push_op(Opcode::Mod);
        self
    }
    /// Emit `Eq`.
    pub fn eq(mut self) -> Self {
        self.chunk.push_op(Opcode::Eq);
        self
    }
    /// Emit `Lt`.
    pub fn lt(mut self) -> Self {
        self.chunk.push_op(Opcode::Lt);
        self
    }
    /// Emit `Le`.
    pub fn le(mut self) -> Self {
        self.chunk.push_op(Opcode::Le);
        self
    }
    /// Emit `Not`.
    pub fn not(mut self) -> Self {
        self.chunk.push_op(Opcode::Not);
        self
    }
    /// Emit `And`.
    pub fn and(mut self) -> Self {
        self.chunk.push_op(Opcode::And);
        self
    }
    /// Emit `Or`.
    pub fn or(mut self) -> Self {
        self.chunk.push_op(Opcode::Or);
        self
    }
    /// Emit `Dup`.
    pub fn dup(mut self) -> Self {
        self.chunk.push_op(Opcode::Dup);
        self
    }
    /// Emit `Pop`.
    pub fn pop(mut self) -> Self {
        self.chunk.push_op(Opcode::Pop);
        self
    }
    /// Emit `Swap`.
    pub fn swap(mut self) -> Self {
        self.chunk.push_op(Opcode::Swap);
        self
    }
    /// Emit `Load(idx)`.
    pub fn load(mut self, idx: u32) -> Self {
        self.chunk.push_op(Opcode::Load(idx));
        self
    }
    /// Emit `Store(idx)`.
    pub fn store(mut self, idx: u32) -> Self {
        self.chunk.push_op(Opcode::Store(idx));
        self
    }
    /// Emit `LoadGlobal(name)`.
    pub fn load_global(mut self, name: &str) -> Self {
        self.chunk.push_op(Opcode::LoadGlobal(name.to_string()));
        self
    }
    /// Emit `Jump(offset)`.
    pub fn jump(mut self, offset: i32) -> Self {
        self.chunk.push_op(Opcode::Jump(offset));
        self
    }
    /// Emit `JumpIf(offset)`.
    pub fn jump_if(mut self, offset: i32) -> Self {
        self.chunk.push_op(Opcode::JumpIf(offset));
        self
    }
    /// Emit `JumpIfNot(offset)`.
    pub fn jump_if_not(mut self, offset: i32) -> Self {
        self.chunk.push_op(Opcode::JumpIfNot(offset));
        self
    }
    /// Emit `Call(pos)`.
    pub fn call(mut self, pos: u32) -> Self {
        self.chunk.push_op(Opcode::Call(pos));
        self
    }
    /// Emit `Return`.
    pub fn ret(mut self) -> Self {
        self.chunk.push_op(Opcode::Return);
        self
    }
    /// Emit `MakeClosure(n)`.
    pub fn make_closure(mut self, n: u32) -> Self {
        self.chunk.push_op(Opcode::MakeClosure(n));
        self
    }
    /// Emit `Apply`.
    pub fn apply(mut self) -> Self {
        self.chunk.push_op(Opcode::Apply);
        self
    }
    /// Emit `Halt`.
    pub fn halt(mut self) -> Self {
        self.chunk.push_op(Opcode::Halt);
        self
    }
    /// Finalize and return the built chunk.
    pub fn build(self) -> BytecodeChunk {
        self.chunk
    }
    /// Current number of opcodes in the builder.
    pub fn current_ip(&self) -> usize {
        self.chunk.opcodes.len()
    }
}
/// Result of applying a call convention.
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum CallResult {
    /// Exact application: `n == arity`.
    Exact { fn_pos: u32, args: Vec<StackValue> },
    /// Under-application: returns a new PAP-like closure.
    Partial {
        captured: Vec<StackValue>,
        remaining_arity: u32,
    },
    /// Over-application: apply to arity args, then apply result to rest.
    Over {
        fn_pos: u32,
        first_args: Vec<StackValue>,
        rest_args: Vec<StackValue>,
    },
}
/// Remove instructions that can never be executed.
///
/// This simple pass detects instructions immediately following an unconditional
/// `Halt` or `Jump` that are not targeted by any branch.
pub struct DeadCodeEliminator;
impl DeadCodeEliminator {
    /// Remove dead instructions from a chunk.
    pub fn eliminate(chunk: &BytecodeChunk) -> BytecodeChunk {
        let n = chunk.opcodes.len();
        let mut reachable = vec![false; n];
        let mut worklist = vec![0usize];
        while let Some(ip) = worklist.pop() {
            if ip >= n || reachable[ip] {
                continue;
            }
            reachable[ip] = true;
            let op = &chunk.opcodes[ip];
            match op {
                Opcode::Halt | Opcode::Return => {}
                Opcode::Jump(off) => {
                    let target = (ip as i64 + 1 + *off as i64) as usize;
                    if target < n {
                        worklist.push(target);
                    }
                }
                Opcode::JumpIf(off) | Opcode::JumpIfNot(off) => {
                    let target = (ip as i64 + 1 + *off as i64) as usize;
                    if target < n {
                        worklist.push(target);
                    }
                    worklist.push(ip + 1);
                }
                _ => {
                    worklist.push(ip + 1);
                }
            }
        }
        let mut out = BytecodeChunk::new(&chunk.name);
        for (i, op) in chunk.opcodes.iter().enumerate() {
            if reachable[i] {
                out.push_op(op.clone());
            }
        }
        out
    }
}
/// A per-chunk inline cache.
#[derive(Default, Debug)]
#[allow(dead_code)]
pub struct InlineCache {
    entries: std::collections::HashMap<usize, InlineCacheEntry>,
}
impl InlineCache {
    /// Create a new inline cache.
    pub fn new() -> Self {
        InlineCache::default()
    }
    /// Record a call at `call_ip` to `target_fn`.
    pub fn record(&mut self, call_ip: usize, target_fn: u32) {
        self.entries
            .entry(call_ip)
            .or_insert_with(|| InlineCacheEntry::new(call_ip))
            .record_call(target_fn);
    }
    /// Get the IC entry for `call_ip`, if any.
    pub fn entry(&self, call_ip: usize) -> Option<&InlineCacheEntry> {
        self.entries.get(&call_ip)
    }
    /// Number of recorded call sites.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Whether the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Return all monomorphic call sites.
    pub fn monomorphic_sites(&self) -> Vec<usize> {
        self.entries
            .values()
            .filter(|e| e.is_monomorphic && e.last_target.is_some())
            .map(|e| e.call_ip)
            .collect()
    }
}
/// Counts how many times each opcode type was executed.
#[derive(Default, Clone, Debug)]
#[allow(dead_code)]
pub struct OpcodeProfile {
    /// Raw count per opcode variant (indexed by discriminant).
    counts: std::collections::HashMap<String, u64>,
    /// Total instructions executed.
    pub total_executed: u64,
}
impl OpcodeProfile {
    /// Create a new empty profile.
    pub fn new() -> Self {
        OpcodeProfile::default()
    }
    /// Record execution of an opcode.
    pub fn record(&mut self, op: &Opcode) {
        let key = opcode_kind(op);
        *self.counts.entry(key).or_insert(0) += 1;
        self.total_executed += 1;
    }
    /// Get the count for a given opcode kind string (e.g., `"Add"`).
    pub fn count(&self, kind: &str) -> u64 {
        self.counts.get(kind).copied().unwrap_or(0)
    }
    /// Return a sorted (descending by count) list of `(kind, count)` pairs.
    pub fn top_opcodes(&self) -> Vec<(String, u64)> {
        let mut v: Vec<_> = self.counts.iter().map(|(k, v)| (k.clone(), *v)).collect();
        v.sort_by(|a, b| b.1.cmp(&a.1));
        v
    }
    /// Reset all counts.
    pub fn reset(&mut self) {
        self.counts.clear();
        self.total_executed = 0;
    }
}
/// An interpreter that manages a proper call frame stack.
pub struct FramedInterpreter {
    /// The operand stack.
    pub stack: Vec<StackValue>,
    /// All local variables across all frames (flat layout).
    pub locals: Vec<StackValue>,
    /// Frame stack (innermost at the back).
    pub frames: Vec<CallFrame>,
    /// Current instruction pointer.
    pub ip: usize,
    /// Maximum frame depth (0 = unlimited).
    pub max_depth: usize,
}
impl FramedInterpreter {
    /// Create a new framed interpreter.
    pub fn new() -> Self {
        FramedInterpreter {
            stack: Vec::new(),
            locals: Vec::new(),
            frames: Vec::new(),
            ip: 0,
            max_depth: 256,
        }
    }
    /// Set the maximum call depth.
    pub fn with_max_depth(mut self, d: usize) -> Self {
        self.max_depth = d;
        self
    }
    /// Push a new call frame, saving `return_ip` and recording `locals_base`.
    pub fn push_frame(&mut self, return_ip: usize, fn_name: &str) -> Result<(), String> {
        if self.max_depth > 0 && self.frames.len() >= self.max_depth {
            return Err(format!(
                "call stack overflow (max depth {})",
                self.max_depth
            ));
        }
        let base = self.locals.len();
        self.frames.push(CallFrame::new(return_ip, base, fn_name));
        Ok(())
    }
    /// Pop the top call frame and restore the instruction pointer.
    pub fn pop_frame(&mut self) -> Option<usize> {
        let frame = self.frames.pop()?;
        self.locals.truncate(frame.locals_base);
        Some(frame.return_ip)
    }
    /// Current call depth.
    pub fn depth(&self) -> usize {
        self.frames.len()
    }
    /// Get a frame by depth (0 = innermost).
    pub fn frame(&self, depth: usize) -> Option<&CallFrame> {
        self.frames.get(self.frames.len().saturating_sub(1 + depth))
    }
    /// Pretty-print the current call stack trace.
    pub fn stack_trace(&self) -> String {
        let mut out = String::new();
        for (i, frame) in self.frames.iter().rev().enumerate() {
            out.push_str(&format!(
                "  #{}: {} (return @{})\n",
                i, frame.fn_name, frame.return_ip
            ));
        }
        out
    }
    /// Reset the interpreter to a clean state.
    pub fn reset(&mut self) {
        self.stack.clear();
        self.locals.clear();
        self.frames.clear();
        self.ip = 0;
    }
}
/// A named sequence of opcodes.
pub struct BytecodeChunk {
    pub opcodes: Vec<Opcode>,
    pub name: String,
}
impl BytecodeChunk {
    /// Create a new empty chunk with the given name.
    pub fn new(name: &str) -> Self {
        BytecodeChunk {
            opcodes: Vec::new(),
            name: name.to_string(),
        }
    }
    /// Append an opcode to the chunk.
    pub fn push_op(&mut self, op: Opcode) {
        self.opcodes.push(op);
    }
    /// Number of opcodes in the chunk.
    pub fn len(&self) -> usize {
        self.opcodes.len()
    }
    /// Whether the chunk is empty.
    pub fn is_empty(&self) -> bool {
        self.opcodes.is_empty()
    }
}
/// A dispatch table mapping opcode tags to handler descriptions.
///
/// Used for debugging, introspection, and code generation tooling.
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct OpcodeInfo {
    /// Opcode tag byte (as used in binary encoding).
    pub tag: u8,
    /// Human-readable mnemonic.
    pub mnemonic: &'static str,
    /// Number of bytes used by the instruction (including the tag byte).
    pub byte_size: usize,
    /// Whether the instruction affects control flow.
    pub is_branch: bool,
    /// Whether the instruction terminates a basic block.
    pub is_terminator: bool,
}
/// A basic block: a maximal sequence of non-branching instructions.
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct BasicBlock {
    /// Start index in the chunk's opcode array.
    pub start: usize,
    /// End index (exclusive).
    pub end: usize,
    /// Successor block indices (for control flow graph).
    pub successors: Vec<usize>,
}
impl BasicBlock {
    /// Number of instructions in this block.
    pub fn len(&self) -> usize {
        self.end - self.start
    }
    /// Whether this block is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
/// A more comprehensive constant folding pass that folds sequences of constant
/// push + arithmetic/logic instructions into a single Push.
pub struct ConstantFolder;
impl ConstantFolder {
    /// Fold all constant expressions in a chunk.
    pub fn fold(chunk: &BytecodeChunk) -> BytecodeChunk {
        let folded = Self::fold_ops(&chunk.opcodes);
        let mut out = BytecodeChunk::new(&chunk.name);
        for op in folded {
            out.push_op(op);
        }
        out
    }
    fn fold_ops(ops: &[Opcode]) -> Vec<Opcode> {
        let mut result: Vec<Opcode> = Vec::new();
        let mut const_stack: Vec<Option<StackValue>> = Vec::new();
        for op in ops {
            match op {
                Opcode::Push(n) => {
                    result.push(op.clone());
                    const_stack.push(Some(StackValue::Nat(*n)));
                }
                Opcode::PushBool(b) => {
                    result.push(op.clone());
                    const_stack.push(Some(StackValue::Bool(*b)));
                }
                Opcode::PushStr(s) => {
                    result.push(op.clone());
                    const_stack.push(Some(StackValue::Str(s.clone())));
                }
                Opcode::PushNil => {
                    result.push(op.clone());
                    const_stack.push(Some(StackValue::Nil));
                }
                Opcode::Add => {
                    let b = const_stack.pop().flatten();
                    let a = const_stack.pop().flatten();
                    if let (Some(StackValue::Nat(av)), Some(StackValue::Nat(bv))) = (a, b) {
                        let len = result.len();
                        if len >= 2 {
                            result.truncate(len - 2);
                        }
                        let folded = av.wrapping_add(bv);
                        result.push(Opcode::Push(folded));
                        const_stack.push(Some(StackValue::Nat(folded)));
                    } else {
                        result.push(op.clone());
                        const_stack.push(None);
                    }
                }
                Opcode::Mul => {
                    let b = const_stack.pop().flatten();
                    let a = const_stack.pop().flatten();
                    if let (Some(StackValue::Nat(av)), Some(StackValue::Nat(bv))) = (a, b) {
                        let len = result.len();
                        if len >= 2 {
                            result.truncate(len - 2);
                        }
                        let folded = av.wrapping_mul(bv);
                        result.push(Opcode::Push(folded));
                        const_stack.push(Some(StackValue::Nat(folded)));
                    } else {
                        result.push(op.clone());
                        const_stack.push(None);
                    }
                }
                Opcode::Sub => {
                    let b = const_stack.pop().flatten();
                    let a = const_stack.pop().flatten();
                    if let (Some(StackValue::Nat(av)), Some(StackValue::Nat(bv))) = (a, b) {
                        let len = result.len();
                        if len >= 2 {
                            result.truncate(len - 2);
                        }
                        let folded = av.wrapping_sub(bv);
                        result.push(Opcode::Push(folded));
                        const_stack.push(Some(StackValue::Nat(folded)));
                    } else {
                        result.push(op.clone());
                        const_stack.push(None);
                    }
                }
                Opcode::Not => {
                    let a = const_stack.pop().flatten();
                    if let Some(StackValue::Bool(bv)) = a {
                        let len = result.len();
                        if len >= 1 {
                            result.truncate(len - 1);
                        }
                        result.push(Opcode::PushBool(!bv));
                        const_stack.push(Some(StackValue::Bool(!bv)));
                    } else {
                        result.push(op.clone());
                        const_stack.push(None);
                    }
                }
                Opcode::Pop => {
                    const_stack.pop();
                    result.push(op.clone());
                }
                Opcode::Dup => {
                    let top = const_stack.last().cloned().flatten();
                    result.push(op.clone());
                    const_stack.push(top.map(Some).unwrap_or(None));
                }
                Opcode::Swap => {
                    let len = const_stack.len();
                    if len >= 2 {
                        const_stack.swap(len - 1, len - 2);
                    }
                    result.push(op.clone());
                }
                _ => {
                    result.push(op.clone());
                    const_stack.push(None);
                }
            }
        }
        result
    }
}
/// Result of liveness analysis for a single chunk.
#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
pub struct LivenessInfo {
    /// For each instruction index, the set of local variable indices that are
    /// live (potentially used later) before that instruction executes.
    pub live_before: Vec<std::collections::BTreeSet<u32>>,
}
/// An interpreter variant that tracks opcode profiling information.
pub struct ProfilingInterpreter {
    /// Underlying interpreter.
    pub interp: Interpreter,
    /// Opcode execution profile.
    pub profile: OpcodeProfile,
}
impl ProfilingInterpreter {
    /// Create a new profiling interpreter.
    pub fn new() -> Self {
        ProfilingInterpreter {
            interp: Interpreter::new(),
            profile: OpcodeProfile::new(),
        }
    }
    /// Execute a chunk, collecting profiling data.
    pub fn execute_chunk(&mut self, chunk: &BytecodeChunk) -> Result<StackValue, String> {
        self.interp.ip = 0;
        loop {
            if self.interp.ip >= chunk.opcodes.len() {
                break;
            }
            let op = chunk.opcodes[self.interp.ip].clone();
            self.interp.ip += 1;
            self.profile.record(&op);
            let cont = self.interp.execute_op(&op, chunk)?;
            if !cont {
                break;
            }
        }
        self.interp
            .pop()
            .ok_or_else(|| "stack underflow at end of chunk".to_string())
    }
}
/// A simple peephole optimizer for bytecode chunks.
///
/// Applies a fixed set of local rewriting rules to reduce unnecessary
/// instructions.
pub struct PeepholeOptimizer {
    /// Number of optimization passes to make.
    pub passes: usize,
}
impl PeepholeOptimizer {
    /// Create a new peephole optimizer with `passes` iterations.
    pub fn new(passes: usize) -> Self {
        PeepholeOptimizer {
            passes: passes.max(1),
        }
    }
    /// Optimize the given chunk, returning a new optimized chunk.
    pub fn optimize(&self, chunk: &BytecodeChunk) -> BytecodeChunk {
        let mut ops = chunk.opcodes.clone();
        for _ in 0..self.passes {
            ops = Self::run_pass(&ops);
        }
        let mut out = BytecodeChunk::new(&chunk.name);
        for op in ops {
            out.push_op(op);
        }
        out
    }
    fn run_pass(ops: &[Opcode]) -> Vec<Opcode> {
        let mut result = Vec::with_capacity(ops.len());
        let mut i = 0;
        while i < ops.len() {
            if i + 1 < ops.len() {
                if let (Opcode::Push(_), Opcode::Pop) = (&ops[i], &ops[i + 1]) {
                    i += 2;
                    continue;
                }
            }
            if i + 2 < ops.len() {
                if let (Opcode::PushBool(a), Opcode::PushBool(b), Opcode::And) =
                    (&ops[i], &ops[i + 1], &ops[i + 2])
                {
                    result.push(Opcode::PushBool(*a && *b));
                    i += 3;
                    continue;
                }
            }
            if i + 2 < ops.len() {
                if let (Opcode::PushBool(a), Opcode::PushBool(b), Opcode::Or) =
                    (&ops[i], &ops[i + 1], &ops[i + 2])
                {
                    result.push(Opcode::PushBool(*a || *b));
                    i += 3;
                    continue;
                }
            }
            if i + 1 < ops.len() {
                if let (Opcode::PushBool(b), Opcode::Not) = (&ops[i], &ops[i + 1]) {
                    result.push(Opcode::PushBool(!*b));
                    i += 2;
                    continue;
                }
            }
            if i + 2 < ops.len() {
                if let (Opcode::Push(a), Opcode::Push(b), Opcode::Add) =
                    (&ops[i], &ops[i + 1], &ops[i + 2])
                {
                    result.push(Opcode::Push(a.wrapping_add(*b)));
                    i += 3;
                    continue;
                }
            }
            if i + 2 < ops.len() {
                if let (Opcode::Push(a), Opcode::Push(b), Opcode::Mul) =
                    (&ops[i], &ops[i + 1], &ops[i + 2])
                {
                    result.push(Opcode::Push(a.wrapping_mul(*b)));
                    i += 3;
                    continue;
                }
            }
            if i + 1 < ops.len() {
                if let (Opcode::Dup, Opcode::Pop) = (&ops[i], &ops[i + 1]) {
                    i += 2;
                    continue;
                }
            }
            result.push(ops[i].clone());
            i += 1;
        }
        result
    }
}
/// A simple compiler from common expression patterns to bytecode chunks.
pub struct BytecodeCompiler;
impl BytecodeCompiler {
    /// Create a new BytecodeCompiler.
    pub fn new() -> Self {
        BytecodeCompiler
    }
    /// Compile a chunk that simply pushes a natural number and halts.
    pub fn compile_nat(n: u64) -> BytecodeChunk {
        let mut chunk = BytecodeChunk::new("nat");
        chunk.push_op(Opcode::Push(n));
        chunk.push_op(Opcode::Halt);
        chunk
    }
    /// Compile a chunk that adds two naturals and halts.
    pub fn compile_add(a: u64, b: u64) -> BytecodeChunk {
        let mut chunk = BytecodeChunk::new("add");
        chunk.push_op(Opcode::Push(a));
        chunk.push_op(Opcode::Push(b));
        chunk.push_op(Opcode::Add);
        chunk.push_op(Opcode::Halt);
        chunk
    }
    /// Compile a conditional: if `cond` then `then_val` else `else_val`.
    pub fn compile_if(cond: bool, then_val: u64, else_val: u64) -> BytecodeChunk {
        let mut chunk = BytecodeChunk::new("if");
        chunk.push_op(Opcode::PushBool(cond));
        chunk.push_op(Opcode::JumpIfNot(2));
        chunk.push_op(Opcode::Push(then_val));
        chunk.push_op(Opcode::Jump(1));
        chunk.push_op(Opcode::Push(else_val));
        chunk.push_op(Opcode::Halt);
        chunk
    }
}
/// A value on the interpreter stack.
#[derive(Debug, Clone, PartialEq)]
pub enum StackValue {
    /// Natural number.
    Nat(u64),
    /// Signed integer.
    Int(i64),
    /// Boolean.
    Bool(bool),
    /// String.
    Str(String),
    /// Closure value with code and captured environment.
    Closure {
        code: Vec<Opcode>,
        env: Vec<StackValue>,
    },
    /// Nil / unit.
    Nil,
}
/// A single bytecode instruction.
#[derive(Debug, Clone, PartialEq)]
pub enum Opcode {
    /// Push a natural number onto the stack.
    Push(u64),
    /// Push a boolean onto the stack.
    PushBool(bool),
    /// Push a string onto the stack.
    PushStr(String),
    /// Push nil (unit) onto the stack.
    PushNil,
    /// Discard the top of the stack.
    Pop,
    /// Duplicate the top of the stack.
    Dup,
    /// Swap the top two stack elements.
    Swap,
    /// Add the top two naturals.
    Add,
    /// Subtract (second - top).
    Sub,
    /// Multiply the top two naturals.
    Mul,
    /// Divide (second / top). Errors on division by zero.
    Div,
    /// Modulo (second % top). Errors on division by zero.
    Mod,
    /// Equality comparison; pushes Bool.
    Eq,
    /// Less-than comparison; pushes Bool.
    Lt,
    /// Less-than-or-equal comparison; pushes Bool.
    Le,
    /// Boolean NOT.
    Not,
    /// Boolean AND.
    And,
    /// Boolean OR.
    Or,
    /// Unconditional jump by a signed offset from the *next* instruction.
    Jump(i32),
    /// Jump if the top of the stack is Bool(true) (pops the condition).
    JumpIf(i32),
    /// Jump if the top of the stack is Bool(false) (pops the condition).
    JumpIfNot(i32),
    /// Call a function at position `u32` in the chunk (simplified model).
    Call(u32),
    /// Return from the current function.
    Return,
    /// Load local variable at index.
    Load(u32),
    /// Store top-of-stack into local variable at index (does not pop).
    Store(u32),
    /// Load a named global value.
    LoadGlobal(String),
    /// Create a closure over `u32` captured values from the stack.
    MakeClosure(u32),
    /// Apply a closure on the stack to the argument below it.
    Apply,
    /// Stop execution.
    Halt,
}
/// A simple stack-based bytecode interpreter.
pub struct Interpreter {
    pub stack: Vec<StackValue>,
    pub locals: Vec<StackValue>,
    pub ip: usize,
    pub call_stack: Vec<usize>,
}
impl Interpreter {
    /// Create a new empty interpreter.
    pub fn new() -> Self {
        Interpreter {
            stack: Vec::new(),
            locals: Vec::with_capacity(64),
            ip: 0,
            call_stack: Vec::new(),
        }
    }
    /// Push a value onto the operand stack.
    pub fn push(&mut self, v: StackValue) {
        self.stack.push(v);
    }
    /// Pop a value from the operand stack.
    pub fn pop(&mut self) -> Option<StackValue> {
        self.stack.pop()
    }
    /// Peek at the top of the operand stack without removing it.
    pub fn peek(&self) -> Option<&StackValue> {
        self.stack.last()
    }
    /// Execute all opcodes in `chunk` and return the top-of-stack value.
    pub fn execute_chunk(&mut self, chunk: &BytecodeChunk) -> Result<StackValue, String> {
        self.ip = 0;
        loop {
            if self.ip >= chunk.opcodes.len() {
                break;
            }
            let op = chunk.opcodes[self.ip].clone();
            self.ip += 1;
            let cont = self.execute_op(&op, chunk)?;
            if !cont {
                break;
            }
        }
        self.pop()
            .ok_or_else(|| "stack underflow at end of chunk".to_string())
    }
    /// Execute a single opcode.
    ///
    /// Returns `Ok(true)` to continue, `Ok(false)` to halt.
    pub fn execute_op(&mut self, op: &Opcode, _chunk: &BytecodeChunk) -> Result<bool, String> {
        match op {
            Opcode::Push(n) => {
                self.stack.push(StackValue::Nat(*n));
            }
            Opcode::PushBool(b) => {
                self.stack.push(StackValue::Bool(*b));
            }
            Opcode::PushStr(s) => {
                self.stack.push(StackValue::Str(s.clone()));
            }
            Opcode::PushNil => {
                self.stack.push(StackValue::Nil);
            }
            Opcode::Pop => {
                self.stack.pop();
            }
            Opcode::Dup => {
                let top = self.stack.last().ok_or("Dup: stack underflow")?.clone();
                self.stack.push(top);
            }
            Opcode::Swap => {
                let len = self.stack.len();
                if len < 2 {
                    return Err("Swap: stack underflow".to_string());
                }
                self.stack.swap(len - 1, len - 2);
            }
            Opcode::Add => {
                let b = self.pop_nat("Add")?;
                let a = self.pop_nat("Add")?;
                self.stack.push(StackValue::Nat(a.wrapping_add(b)));
            }
            Opcode::Sub => {
                let b = self.pop_nat("Sub")?;
                let a = self.pop_nat("Sub")?;
                self.stack.push(StackValue::Nat(a.wrapping_sub(b)));
            }
            Opcode::Mul => {
                let b = self.pop_nat("Mul")?;
                let a = self.pop_nat("Mul")?;
                self.stack.push(StackValue::Nat(a.wrapping_mul(b)));
            }
            Opcode::Div => {
                let b = self.pop_nat("Div")?;
                let a = self.pop_nat("Div")?;
                if b == 0 {
                    return Err("division by zero".to_string());
                }
                self.stack.push(StackValue::Nat(a / b));
            }
            Opcode::Mod => {
                let b = self.pop_nat("Mod")?;
                let a = self.pop_nat("Mod")?;
                if b == 0 {
                    return Err("modulo by zero".to_string());
                }
                self.stack.push(StackValue::Nat(a % b));
            }
            Opcode::Eq => {
                let b = self.pop().ok_or("Eq: stack underflow")?;
                let a = self.pop().ok_or("Eq: stack underflow")?;
                self.stack.push(StackValue::Bool(a == b));
            }
            Opcode::Lt => {
                let b = self.pop_nat("Lt")?;
                let a = self.pop_nat("Lt")?;
                self.stack.push(StackValue::Bool(a < b));
            }
            Opcode::Le => {
                let b = self.pop_nat("Le")?;
                let a = self.pop_nat("Le")?;
                self.stack.push(StackValue::Bool(a <= b));
            }
            Opcode::Not => {
                let v = self.pop_bool("Not")?;
                self.stack.push(StackValue::Bool(!v));
            }
            Opcode::And => {
                let b = self.pop_bool("And")?;
                let a = self.pop_bool("And")?;
                self.stack.push(StackValue::Bool(a && b));
            }
            Opcode::Or => {
                let b = self.pop_bool("Or")?;
                let a = self.pop_bool("Or")?;
                self.stack.push(StackValue::Bool(a || b));
            }
            Opcode::Jump(offset) => {
                let new_ip = self.ip as i64 + *offset as i64;
                if new_ip < 0 {
                    return Err(format!("Jump: negative IP {}", new_ip));
                }
                self.ip = new_ip as usize;
            }
            Opcode::JumpIf(offset) => {
                let cond = self.pop_bool("JumpIf")?;
                if cond {
                    let new_ip = self.ip as i64 + *offset as i64;
                    if new_ip < 0 {
                        return Err(format!("JumpIf: negative IP {}", new_ip));
                    }
                    self.ip = new_ip as usize;
                }
            }
            Opcode::JumpIfNot(offset) => {
                let cond = self.pop_bool("JumpIfNot")?;
                if !cond {
                    let new_ip = self.ip as i64 + *offset as i64;
                    if new_ip < 0 {
                        return Err(format!("JumpIfNot: negative IP {}", new_ip));
                    }
                    self.ip = new_ip as usize;
                }
            }
            Opcode::Call(pos) => {
                self.call_stack.push(self.ip);
                self.ip = *pos as usize;
            }
            Opcode::Return => {
                if let Some(ret_addr) = self.call_stack.pop() {
                    self.ip = ret_addr;
                } else {
                    return Ok(false);
                }
            }
            Opcode::Load(idx) => {
                let idx = *idx as usize;
                let v = self
                    .locals
                    .get(idx)
                    .ok_or_else(|| format!("Load: local {} not found", idx))?
                    .clone();
                self.stack.push(v);
            }
            Opcode::Store(idx) => {
                let idx = *idx as usize;
                let v = self.stack.last().ok_or("Store: stack underflow")?.clone();
                while self.locals.len() <= idx {
                    self.locals.push(StackValue::Nil);
                }
                self.locals[idx] = v;
            }
            Opcode::LoadGlobal(name) => {
                self.stack
                    .push(StackValue::Str(format!("<global:{}>", name)));
            }
            Opcode::MakeClosure(n_captured) => {
                let n = *n_captured as usize;
                if self.stack.len() < n {
                    return Err(format!(
                        "MakeClosure: need {} captures, have {}",
                        n,
                        self.stack.len()
                    ));
                }
                let mut env = Vec::with_capacity(n);
                for _ in 0..n {
                    env.push(self.pop().expect(
                        "stack has at least n elements as verified by the length check above",
                    ));
                }
                env.reverse();
                self.stack.push(StackValue::Closure {
                    code: Vec::new(),
                    env,
                });
            }
            Opcode::Apply => {
                let _arg = self.pop().ok_or("Apply: stack underflow (arg)")?;
                let _closure = self.pop().ok_or("Apply: stack underflow (closure)")?;
                self.stack.push(StackValue::Nil);
            }
            Opcode::Halt => {
                return Ok(false);
            }
        }
        Ok(true)
    }
    fn pop_nat(&mut self, ctx: &str) -> Result<u64, String> {
        match self.pop() {
            Some(StackValue::Nat(n)) => Ok(n),
            Some(other) => Err(format!("{}: expected Nat, got {:?}", ctx, other)),
            None => Err(format!("{}: stack underflow", ctx)),
        }
    }
    fn pop_bool(&mut self, ctx: &str) -> Result<bool, String> {
        match self.pop() {
            Some(StackValue::Bool(b)) => Ok(b),
            Some(other) => Err(format!("{}: expected Bool, got {:?}", ctx, other)),
            None => Err(format!("{}: stack underflow", ctx)),
        }
    }
}
/// Compact binary encoding of a single opcode.
///
/// Format (variable-width):
/// - 1 byte: opcode tag
/// - optional operand bytes depending on the opcode
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EncodedInstruction {
    /// Raw bytes of the encoded instruction.
    pub bytes: Vec<u8>,
}
impl EncodedInstruction {
    /// Encode a single [`Opcode`] to bytes.
    pub fn encode(op: &Opcode) -> Self {
        let mut bytes = Vec::new();
        match op {
            Opcode::Push(n) => {
                bytes.push(0x01);
                bytes.extend_from_slice(&n.to_le_bytes());
            }
            Opcode::PushBool(b) => {
                bytes.push(0x02);
                bytes.push(*b as u8);
            }
            Opcode::PushStr(s) => {
                bytes.push(0x03);
                let len = s.len() as u32;
                bytes.extend_from_slice(&len.to_le_bytes());
                bytes.extend_from_slice(s.as_bytes());
            }
            Opcode::PushNil => bytes.push(0x04),
            Opcode::Pop => bytes.push(0x10),
            Opcode::Dup => bytes.push(0x11),
            Opcode::Swap => bytes.push(0x12),
            Opcode::Add => bytes.push(0x20),
            Opcode::Sub => bytes.push(0x21),
            Opcode::Mul => bytes.push(0x22),
            Opcode::Div => bytes.push(0x23),
            Opcode::Mod => bytes.push(0x24),
            Opcode::Eq => bytes.push(0x30),
            Opcode::Lt => bytes.push(0x31),
            Opcode::Le => bytes.push(0x32),
            Opcode::Not => bytes.push(0x40),
            Opcode::And => bytes.push(0x41),
            Opcode::Or => bytes.push(0x42),
            Opcode::Jump(off) => {
                bytes.push(0x50);
                bytes.extend_from_slice(&off.to_le_bytes());
            }
            Opcode::JumpIf(off) => {
                bytes.push(0x51);
                bytes.extend_from_slice(&off.to_le_bytes());
            }
            Opcode::JumpIfNot(off) => {
                bytes.push(0x52);
                bytes.extend_from_slice(&off.to_le_bytes());
            }
            Opcode::Call(pos) => {
                bytes.push(0x60);
                bytes.extend_from_slice(&pos.to_le_bytes());
            }
            Opcode::Return => bytes.push(0x61),
            Opcode::Load(idx) => {
                bytes.push(0x70);
                bytes.extend_from_slice(&idx.to_le_bytes());
            }
            Opcode::Store(idx) => {
                bytes.push(0x71);
                bytes.extend_from_slice(&idx.to_le_bytes());
            }
            Opcode::LoadGlobal(name) => {
                bytes.push(0x72);
                let len = name.len() as u32;
                bytes.extend_from_slice(&len.to_le_bytes());
                bytes.extend_from_slice(name.as_bytes());
            }
            Opcode::MakeClosure(n) => {
                bytes.push(0x80);
                bytes.extend_from_slice(&n.to_le_bytes());
            }
            Opcode::Apply => bytes.push(0x81),
            Opcode::Halt => bytes.push(0xFF),
        }
        EncodedInstruction { bytes }
    }
    /// Attempt to decode the first instruction from `data`.
    /// Returns `(opcode, bytes_consumed)` or `None` if data is malformed.
    pub fn decode(data: &[u8]) -> Option<(Opcode, usize)> {
        if data.is_empty() {
            return None;
        }
        let tag = data[0];
        match tag {
            0x01 => {
                if data.len() < 9 {
                    return None;
                }
                let n = u64::from_le_bytes(data[1..9].try_into().ok()?);
                Some((Opcode::Push(n), 9))
            }
            0x02 => {
                if data.len() < 2 {
                    return None;
                }
                Some((Opcode::PushBool(data[1] != 0), 2))
            }
            0x03 => {
                if data.len() < 5 {
                    return None;
                }
                let len = u32::from_le_bytes(data[1..5].try_into().ok()?) as usize;
                if data.len() < 5 + len {
                    return None;
                }
                let s = std::str::from_utf8(&data[5..5 + len]).ok()?.to_string();
                Some((Opcode::PushStr(s), 5 + len))
            }
            0x04 => Some((Opcode::PushNil, 1)),
            0x10 => Some((Opcode::Pop, 1)),
            0x11 => Some((Opcode::Dup, 1)),
            0x12 => Some((Opcode::Swap, 1)),
            0x20 => Some((Opcode::Add, 1)),
            0x21 => Some((Opcode::Sub, 1)),
            0x22 => Some((Opcode::Mul, 1)),
            0x23 => Some((Opcode::Div, 1)),
            0x24 => Some((Opcode::Mod, 1)),
            0x30 => Some((Opcode::Eq, 1)),
            0x31 => Some((Opcode::Lt, 1)),
            0x32 => Some((Opcode::Le, 1)),
            0x40 => Some((Opcode::Not, 1)),
            0x41 => Some((Opcode::And, 1)),
            0x42 => Some((Opcode::Or, 1)),
            0x50 => {
                if data.len() < 5 {
                    return None;
                }
                let off = i32::from_le_bytes(data[1..5].try_into().ok()?);
                Some((Opcode::Jump(off), 5))
            }
            0x51 => {
                if data.len() < 5 {
                    return None;
                }
                let off = i32::from_le_bytes(data[1..5].try_into().ok()?);
                Some((Opcode::JumpIf(off), 5))
            }
            0x52 => {
                if data.len() < 5 {
                    return None;
                }
                let off = i32::from_le_bytes(data[1..5].try_into().ok()?);
                Some((Opcode::JumpIfNot(off), 5))
            }
            0x60 => {
                if data.len() < 5 {
                    return None;
                }
                let pos = u32::from_le_bytes(data[1..5].try_into().ok()?);
                Some((Opcode::Call(pos), 5))
            }
            0x61 => Some((Opcode::Return, 1)),
            0x70 => {
                if data.len() < 5 {
                    return None;
                }
                let idx = u32::from_le_bytes(data[1..5].try_into().ok()?);
                Some((Opcode::Load(idx), 5))
            }
            0x71 => {
                if data.len() < 5 {
                    return None;
                }
                let idx = u32::from_le_bytes(data[1..5].try_into().ok()?);
                Some((Opcode::Store(idx), 5))
            }
            0x72 => {
                if data.len() < 5 {
                    return None;
                }
                let len = u32::from_le_bytes(data[1..5].try_into().ok()?) as usize;
                if data.len() < 5 + len {
                    return None;
                }
                let s = std::str::from_utf8(&data[5..5 + len]).ok()?.to_string();
                Some((Opcode::LoadGlobal(s), 5 + len))
            }
            0x80 => {
                if data.len() < 5 {
                    return None;
                }
                let n = u32::from_le_bytes(data[1..5].try_into().ok()?);
                Some((Opcode::MakeClosure(n), 5))
            }
            0x81 => Some((Opcode::Apply, 1)),
            0xFF => Some((Opcode::Halt, 1)),
            _ => None,
        }
    }
}
