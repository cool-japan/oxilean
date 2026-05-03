//! Type definitions

use std::collections::{HashMap, HashSet, VecDeque};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct X86DepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
/// Dominator tree for X86Ext.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct X86ExtDomTree {
    pub(super) idom: Vec<Option<usize>>,
    pub(super) children: Vec<Vec<usize>>,
    pub(super) depth: Vec<usize>,
}
/// x86-64 code-generation backend (AT&T syntax).
#[derive(Debug, Clone, Default)]
pub struct X86Backend;
/// Statistics for X86Ext passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct X86ExtPassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum X86PassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
/// A named x86-64 function / text section.
#[derive(Debug, Clone)]
pub struct X86Function {
    pub name: String,
    pub instrs: Vec<X86Instr>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct X86PassConfig {
    pub phase: X86PassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
/// Liveness analysis for X86Ext.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct X86ExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
/// Analysis cache for X86X2.
#[allow(dead_code)]
#[derive(Debug)]
pub struct X86X2Cache {
    pub(super) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(super) cap: usize,
    pub(super) total_hits: u64,
    pub(super) total_misses: u64,
}
#[allow(dead_code)]
pub struct X86PassRegistry {
    pub(super) configs: Vec<X86PassConfig>,
    pub(super) stats: std::collections::HashMap<String, X86PassStats>,
}
/// x86-64 instruction set (AT&T syntax output).
#[derive(Debug, Clone)]
pub enum X86Instr {
    Mov(X86Reg, X86Reg),
    MovImm(X86Reg, i64),
    MovLoad(X86Reg, MemOp),
    MovStore(MemOp, X86Reg),
    MovImmStore(MemOp, i32),
    Lea(X86Reg, MemOp),
    Push(X86Reg),
    Pop(X86Reg),
    Add(X86Reg, X86Reg),
    AddImm(X86Reg, i32),
    Sub(X86Reg, X86Reg),
    SubImm(X86Reg, i32),
    IMul(X86Reg, X86Reg),
    IDiv(X86Reg),
    Neg(X86Reg),
    And(X86Reg, X86Reg),
    AndImm(X86Reg, i32),
    Or(X86Reg, X86Reg),
    OrImm(X86Reg, i32),
    Xor(X86Reg, X86Reg),
    XorImm(X86Reg, i32),
    Not(X86Reg),
    Shl(X86Reg, u8),
    Shr(X86Reg, u8),
    Sar(X86Reg, u8),
    Cmp(X86Reg, X86Reg),
    CmpImm(X86Reg, i32),
    Test(X86Reg, X86Reg),
    SetE(X86Reg),
    SetNe(X86Reg),
    SetL(X86Reg),
    SetG(X86Reg),
    Jmp(String),
    Je(String),
    Jne(String),
    Jl(String),
    Jg(String),
    Jle(String),
    Jge(String),
    Call(String),
    CallReg(X86Reg),
    Ret,
    Cqo,
    MovsdLoad(X86Reg, MemOp),
    MovsdStore(MemOp, X86Reg),
    AddsdReg(X86Reg, X86Reg),
    SubsdReg(X86Reg, X86Reg),
    MulsdReg(X86Reg, X86Reg),
    DivsdReg(X86Reg, X86Reg),
    Label(String),
    Directive(String, String),
    /// Raw text (escape hatch).
    Raw(String),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct X86DominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
/// Dependency graph for X86Ext.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct X86ExtDepGraph {
    pub(super) n: usize,
    pub(super) adj: Vec<Vec<usize>>,
    pub(super) rev: Vec<Vec<usize>>,
    pub(super) edge_count: usize,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct X86PassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct X86AnalysisCache {
    pub(crate) entries: std::collections::HashMap<String, X86CacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct X86LivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
/// Worklist for X86X2.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct X86X2Worklist {
    pub(super) items: std::collections::VecDeque<usize>,
    pub(super) present: Vec<bool>,
}
/// Analysis cache for X86Ext.
#[allow(dead_code)]
#[derive(Debug)]
pub struct X86ExtCache {
    pub(super) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(super) cap: usize,
    pub(super) total_hits: u64,
    pub(super) total_misses: u64,
}
/// Constant folding helper for X86Ext.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct X86ExtConstFolder {
    pub(super) folds: usize,
    pub(super) failures: usize,
    pub(super) enabled: bool,
}
/// Liveness analysis for X86X2.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct X86X2Liveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
/// Dependency graph for X86X2.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct X86X2DepGraph {
    pub(super) n: usize,
    pub(super) adj: Vec<Vec<usize>>,
    pub(super) rev: Vec<Vec<usize>>,
    pub(super) edge_count: usize,
}
/// Pass registry for X86X2.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct X86X2PassRegistry {
    pub(super) configs: Vec<X86X2PassConfig>,
    pub(super) stats: Vec<X86X2PassStats>,
}
/// Pass execution phase for X86X2.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum X86X2PassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct X86CacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
/// Constant folding helper for X86X2.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct X86X2ConstFolder {
    pub(super) folds: usize,
    pub(super) failures: usize,
    pub(super) enabled: bool,
}
/// Pass registry for X86Ext.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct X86ExtPassRegistry {
    pub(super) configs: Vec<X86ExtPassConfig>,
    pub(super) stats: Vec<X86ExtPassStats>,
}
#[allow(dead_code)]
pub struct X86ConstantFoldingHelper;
/// Configuration for X86Ext passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct X86ExtPassConfig {
    pub name: String,
    pub phase: X86ExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct X86Worklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
/// Worklist for X86Ext.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct X86ExtWorklist {
    pub(super) items: std::collections::VecDeque<usize>,
    pub(super) present: Vec<bool>,
}
/// Pass execution phase for X86Ext.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum X86ExtPassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
/// Configuration for X86X2 passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct X86X2PassConfig {
    pub name: String,
    pub phase: X86X2PassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
/// Memory operand: base + optional displacement.
#[derive(Debug, Clone)]
pub struct MemOp {
    pub base: X86Reg,
    pub disp: i64,
}
/// Statistics for X86X2 passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct X86X2PassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}
/// Dominator tree for X86X2.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct X86X2DomTree {
    pub(super) idom: Vec<Option<usize>>,
    pub(super) children: Vec<Vec<usize>>,
    pub(super) depth: Vec<usize>,
}
/// x86-64 register set (integer + lower XMM).
#[derive(Debug, Clone, PartialEq)]
pub enum X86Reg {
    RAX,
    RBX,
    RCX,
    RDX,
    RSP,
    RBP,
    RSI,
    RDI,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    EAX,
    EBX,
    ECX,
    EDX,
    ESI,
    EDI,
    R8D,
    R9D,
    XMM0,
    XMM1,
    XMM2,
    XMM3,
    XMM4,
    XMM5,
    XMM6,
    XMM7,
}
