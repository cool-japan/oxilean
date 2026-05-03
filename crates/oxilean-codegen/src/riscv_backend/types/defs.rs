//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::impls1::*;
use super::impls2::*;
use std::collections::{HashMap, HashSet, VecDeque};

/// Pass registry for RISCVExt.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct RISCVExtPassRegistry {
    pub(crate) configs: Vec<RISCVExtPassConfig>,
    pub(crate) stats: Vec<RISCVExtPassStats>,
}
impl RISCVExtPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn register(&mut self, c: RISCVExtPassConfig) {
        self.stats.push(RISCVExtPassStats::new());
        self.configs.push(c);
    }
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.configs.len()
    }
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.configs.is_empty()
    }
    #[allow(dead_code)]
    pub fn get(&self, i: usize) -> Option<&RISCVExtPassConfig> {
        self.configs.get(i)
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, i: usize) -> Option<&RISCVExtPassStats> {
        self.stats.get(i)
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&RISCVExtPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn passes_in_phase(&self, ph: &RISCVExtPassPhase) -> Vec<&RISCVExtPassConfig> {
        self.configs
            .iter()
            .filter(|c| c.enabled && &c.phase == ph)
            .collect()
    }
    #[allow(dead_code)]
    pub fn total_nodes_visited(&self) -> usize {
        self.stats.iter().map(|s| s.nodes_visited).sum()
    }
    #[allow(dead_code)]
    pub fn any_changed(&self) -> bool {
        self.stats.iter().any(|s| s.changed)
    }
}
/// Dependency graph for RISCVX2.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RISCVX2DepGraph {
    pub(crate) n: usize,
    pub(crate) adj: Vec<Vec<usize>>,
    pub(crate) rev: Vec<Vec<usize>>,
    pub(crate) edge_count: usize,
}
impl RISCVX2DepGraph {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        Self {
            n,
            adj: vec![Vec::new(); n],
            rev: vec![Vec::new(); n],
            edge_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_edge(&mut self, from: usize, to: usize) {
        if from < self.n && to < self.n {
            if !self.adj[from].contains(&to) {
                self.adj[from].push(to);
                self.rev[to].push(from);
                self.edge_count += 1;
            }
        }
    }
    #[allow(dead_code)]
    pub fn succs(&self, n: usize) -> &[usize] {
        self.adj.get(n).map(|v| v.as_slice()).unwrap_or(&[])
    }
    #[allow(dead_code)]
    pub fn preds(&self, n: usize) -> &[usize] {
        self.rev.get(n).map(|v| v.as_slice()).unwrap_or(&[])
    }
    #[allow(dead_code)]
    pub fn topo_sort(&self) -> Option<Vec<usize>> {
        let mut deg: Vec<usize> = (0..self.n).map(|i| self.rev[i].len()).collect();
        let mut q: std::collections::VecDeque<usize> =
            (0..self.n).filter(|&i| deg[i] == 0).collect();
        let mut out = Vec::with_capacity(self.n);
        while let Some(u) = q.pop_front() {
            out.push(u);
            for &v in &self.adj[u] {
                deg[v] -= 1;
                if deg[v] == 0 {
                    q.push_back(v);
                }
            }
        }
        if out.len() == self.n {
            Some(out)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn has_cycle(&self) -> bool {
        self.topo_sort().is_none()
    }
    #[allow(dead_code)]
    pub fn reachable(&self, start: usize) -> Vec<usize> {
        let mut vis = vec![false; self.n];
        let mut stk = vec![start];
        let mut out = Vec::new();
        while let Some(u) = stk.pop() {
            if u < self.n && !vis[u] {
                vis[u] = true;
                out.push(u);
                for &v in &self.adj[u] {
                    if !vis[v] {
                        stk.push(v);
                    }
                }
            }
        }
        out
    }
    #[allow(dead_code)]
    pub fn scc(&self) -> Vec<Vec<usize>> {
        let mut visited = vec![false; self.n];
        let mut order = Vec::new();
        for i in 0..self.n {
            if !visited[i] {
                let mut stk = vec![(i, 0usize)];
                while let Some((u, idx)) = stk.last_mut() {
                    if !visited[*u] {
                        visited[*u] = true;
                    }
                    if *idx < self.adj[*u].len() {
                        let v = self.adj[*u][*idx];
                        *idx += 1;
                        if !visited[v] {
                            stk.push((v, 0));
                        }
                    } else {
                        order.push(*u);
                        stk.pop();
                    }
                }
            }
        }
        let mut comp = vec![usize::MAX; self.n];
        let mut components: Vec<Vec<usize>> = Vec::new();
        for &start in order.iter().rev() {
            if comp[start] == usize::MAX {
                let cid = components.len();
                let mut component = Vec::new();
                let mut stk = vec![start];
                while let Some(u) = stk.pop() {
                    if comp[u] == usize::MAX {
                        comp[u] = cid;
                        component.push(u);
                        for &v in &self.rev[u] {
                            if comp[v] == usize::MAX {
                                stk.push(v);
                            }
                        }
                    }
                }
                components.push(component);
            }
        }
        components
    }
    #[allow(dead_code)]
    pub fn node_count(&self) -> usize {
        self.n
    }
    #[allow(dead_code)]
    pub fn edge_count(&self) -> usize {
        self.edge_count
    }
}
/// Worklist for RISCVX2.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RISCVX2Worklist {
    pub(crate) items: std::collections::VecDeque<usize>,
    pub(crate) present: Vec<bool>,
}
impl RISCVX2Worklist {
    #[allow(dead_code)]
    pub fn new(capacity: usize) -> Self {
        Self {
            items: std::collections::VecDeque::new(),
            present: vec![false; capacity],
        }
    }
    #[allow(dead_code)]
    pub fn push(&mut self, id: usize) {
        if id < self.present.len() && !self.present[id] {
            self.present[id] = true;
            self.items.push_back(id);
        }
    }
    #[allow(dead_code)]
    pub fn push_front(&mut self, id: usize) {
        if id < self.present.len() && !self.present[id] {
            self.present[id] = true;
            self.items.push_front(id);
        }
    }
    #[allow(dead_code)]
    pub fn pop(&mut self) -> Option<usize> {
        let id = self.items.pop_front()?;
        if id < self.present.len() {
            self.present[id] = false;
        }
        Some(id)
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
    pub fn contains(&self, id: usize) -> bool {
        id < self.present.len() && self.present[id]
    }
    #[allow(dead_code)]
    pub fn drain_all(&mut self) -> Vec<usize> {
        let v: Vec<usize> = self.items.drain(..).collect();
        for &id in &v {
            if id < self.present.len() {
                self.present[id] = false;
            }
        }
        v
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RvDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl RvDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        RvDominatorTree {
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
/// RISC-V code-generation backend.
///
/// Supports both RV32I (`is_64bit = false`) and RV64I (`is_64bit = true`).
#[derive(Debug, Clone)]
pub struct RiscVBackend {
    pub is_64bit: bool,
}
impl RiscVBackend {
    pub fn new(is_64bit: bool) -> Self {
        Self { is_64bit }
    }
    /// Emit a single instruction as a text line (GNU assembler syntax).
    pub fn emit_instr(&self, instr: &RiscVInstr) -> String {
        match instr {
            RiscVInstr::LUI(rd, imm) => format!("    lui     {}, {}", rd.name(), imm),
            RiscVInstr::AUIPC(rd, imm) => format!("    auipc   {}, {}", rd.name(), imm),
            RiscVInstr::JAL(rd, off) => format!("    jal     {}, {}", rd.name(), off),
            RiscVInstr::JALR(rd, rs1, off) => {
                format!("    jalr    {}, {}({})", rd.name(), off, rs1.name())
            }
            RiscVInstr::BEQ(rs1, rs2, off) => {
                format!("    beq     {}, {}, {}", rs1.name(), rs2.name(), off)
            }
            RiscVInstr::BNE(rs1, rs2, off) => {
                format!("    bne     {}, {}, {}", rs1.name(), rs2.name(), off)
            }
            RiscVInstr::BLT(rs1, rs2, off) => {
                format!("    blt     {}, {}, {}", rs1.name(), rs2.name(), off)
            }
            RiscVInstr::BGE(rs1, rs2, off) => {
                format!("    bge     {}, {}, {}", rs1.name(), rs2.name(), off)
            }
            RiscVInstr::BLTU(rs1, rs2, off) => {
                format!("    bltu    {}, {}, {}", rs1.name(), rs2.name(), off)
            }
            RiscVInstr::BGEU(rs1, rs2, off) => {
                format!("    bgeu    {}, {}, {}", rs1.name(), rs2.name(), off)
            }
            RiscVInstr::LB(rd, rs1, off) => {
                format!("    lb      {}, {}({})", rd.name(), off, rs1.name())
            }
            RiscVInstr::LH(rd, rs1, off) => {
                format!("    lh      {}, {}({})", rd.name(), off, rs1.name())
            }
            RiscVInstr::LW(rd, rs1, off) => {
                format!("    lw      {}, {}({})", rd.name(), off, rs1.name())
            }
            RiscVInstr::LBU(rd, rs1, off) => {
                format!("    lbu     {}, {}({})", rd.name(), off, rs1.name())
            }
            RiscVInstr::LHU(rd, rs1, off) => {
                format!("    lhu     {}, {}({})", rd.name(), off, rs1.name())
            }
            RiscVInstr::LD(rd, rs1, off) => {
                format!("    ld      {}, {}({})", rd.name(), off, rs1.name())
            }
            RiscVInstr::LWU(rd, rs1, off) => {
                format!("    lwu     {}, {}({})", rd.name(), off, rs1.name())
            }
            RiscVInstr::SB(rs2, rs1, off) => {
                format!("    sb      {}, {}({})", rs2.name(), off, rs1.name())
            }
            RiscVInstr::SH(rs2, rs1, off) => {
                format!("    sh      {}, {}({})", rs2.name(), off, rs1.name())
            }
            RiscVInstr::SW(rs2, rs1, off) => {
                format!("    sw      {}, {}({})", rs2.name(), off, rs1.name())
            }
            RiscVInstr::SD(rs2, rs1, off) => {
                format!("    sd      {}, {}({})", rs2.name(), off, rs1.name())
            }
            RiscVInstr::ADDI(rd, rs1, imm) => {
                format!("    addi    {}, {}, {}", rd.name(), rs1.name(), imm)
            }
            RiscVInstr::SLTI(rd, rs1, imm) => {
                format!("    slti    {}, {}, {}", rd.name(), rs1.name(), imm)
            }
            RiscVInstr::SLTIU(rd, rs1, imm) => {
                format!("    sltiu   {}, {}, {}", rd.name(), rs1.name(), imm)
            }
            RiscVInstr::XORI(rd, rs1, imm) => {
                format!("    xori    {}, {}, {}", rd.name(), rs1.name(), imm)
            }
            RiscVInstr::ORI(rd, rs1, imm) => {
                format!("    ori     {}, {}, {}", rd.name(), rs1.name(), imm)
            }
            RiscVInstr::ANDI(rd, rs1, imm) => {
                format!("    andi    {}, {}, {}", rd.name(), rs1.name(), imm)
            }
            RiscVInstr::SLLI(rd, rs1, shamt) => {
                format!("    slli    {}, {}, {}", rd.name(), rs1.name(), shamt)
            }
            RiscVInstr::SRLI(rd, rs1, shamt) => {
                format!("    srli    {}, {}, {}", rd.name(), rs1.name(), shamt)
            }
            RiscVInstr::SRAI(rd, rs1, shamt) => {
                format!("    srai    {}, {}, {}", rd.name(), rs1.name(), shamt)
            }
            RiscVInstr::ADDIW(rd, rs1, imm) => {
                format!("    addiw   {}, {}, {}", rd.name(), rs1.name(), imm)
            }
            RiscVInstr::ADD(rd, rs1, rs2) => {
                format!("    add     {}, {}, {}", rd.name(), rs1.name(), rs2.name())
            }
            RiscVInstr::SUB(rd, rs1, rs2) => {
                format!("    sub     {}, {}, {}", rd.name(), rs1.name(), rs2.name())
            }
            RiscVInstr::SLL(rd, rs1, rs2) => {
                format!("    sll     {}, {}, {}", rd.name(), rs1.name(), rs2.name())
            }
            RiscVInstr::SLT(rd, rs1, rs2) => {
                format!("    slt     {}, {}, {}", rd.name(), rs1.name(), rs2.name())
            }
            RiscVInstr::SLTU(rd, rs1, rs2) => {
                format!("    sltu    {}, {}, {}", rd.name(), rs1.name(), rs2.name())
            }
            RiscVInstr::XOR(rd, rs1, rs2) => {
                format!("    xor     {}, {}, {}", rd.name(), rs1.name(), rs2.name())
            }
            RiscVInstr::SRL(rd, rs1, rs2) => {
                format!("    srl     {}, {}, {}", rd.name(), rs1.name(), rs2.name())
            }
            RiscVInstr::SRA(rd, rs1, rs2) => {
                format!("    sra     {}, {}, {}", rd.name(), rs1.name(), rs2.name())
            }
            RiscVInstr::OR(rd, rs1, rs2) => {
                format!("    or      {}, {}, {}", rd.name(), rs1.name(), rs2.name())
            }
            RiscVInstr::AND(rd, rs1, rs2) => {
                format!("    and     {}, {}, {}", rd.name(), rs1.name(), rs2.name())
            }
            RiscVInstr::MUL(rd, rs1, rs2) => {
                format!("    mul     {}, {}, {}", rd.name(), rs1.name(), rs2.name())
            }
            RiscVInstr::MULH(rd, rs1, rs2) => {
                format!("    mulh    {}, {}, {}", rd.name(), rs1.name(), rs2.name())
            }
            RiscVInstr::DIV(rd, rs1, rs2) => {
                format!("    div     {}, {}, {}", rd.name(), rs1.name(), rs2.name())
            }
            RiscVInstr::REM(rd, rs1, rs2) => {
                format!("    rem     {}, {}, {}", rd.name(), rs1.name(), rs2.name())
            }
            RiscVInstr::ECALL => "    ecall".to_string(),
            RiscVInstr::EBREAK => "    ebreak".to_string(),
            RiscVInstr::LI(rd, imm) => format!("    li      {}, {}", rd.name(), imm),
            RiscVInstr::MV(rd, rs) => format!("    mv      {}, {}", rd.name(), rs.name()),
            RiscVInstr::NOP => "    nop".to_string(),
            RiscVInstr::RET => "    ret".to_string(),
            RiscVInstr::CALL(sym) => format!("    call    {}", sym),
            RiscVInstr::Label(lbl) => format!("{}:", lbl),
            RiscVInstr::Directive(d, arg) => {
                if arg.is_empty() {
                    format!("    .{}", d)
                } else {
                    format!("    .{} {}", d, arg)
                }
            }
        }
    }
    /// Emit a complete function (with .globl / .type directives and label).
    pub fn emit_function(&self, func: &RiscVFunction) -> String {
        let mut out = String::new();
        out.push_str(&format!("    .globl {}\n", func.name));
        out.push_str(&format!("    .type  {}, @function\n", func.name));
        out.push_str(&format!("{}:\n", func.name));
        for instr in &func.instrs {
            out.push_str(&self.emit_instr(instr));
            out.push('\n');
        }
        out.push_str(&format!("    .size  {}, .-{}\n", func.name, func.name));
        out
    }
    /// Standard RISC-V function prologue.
    ///
    /// Saves `ra` and `s0` (frame pointer), sets up a stack frame of
    /// `frame_size` bytes (rounded up to 16-byte alignment).
    pub fn prologue(&self, frame_size: u32) -> Vec<RiscVInstr> {
        let aligned = frame_size.div_ceil(16) * 16;
        let neg = -(aligned as i16);
        let (store_ra, store_fp, set_fp) = if self.is_64bit {
            (
                RiscVInstr::SD(RiscVReg::Ra, RiscVReg::Sp, (aligned as i16) - 8),
                RiscVInstr::SD(RiscVReg::S0, RiscVReg::Sp, (aligned as i16) - 16),
                RiscVInstr::ADDI(RiscVReg::S0, RiscVReg::Sp, aligned as i16),
            )
        } else {
            (
                RiscVInstr::SW(RiscVReg::Ra, RiscVReg::Sp, (aligned as i16) - 4),
                RiscVInstr::SW(RiscVReg::S0, RiscVReg::Sp, (aligned as i16) - 8),
                RiscVInstr::ADDI(RiscVReg::S0, RiscVReg::Sp, aligned as i16),
            )
        };
        vec![
            RiscVInstr::ADDI(RiscVReg::Sp, RiscVReg::Sp, neg),
            store_ra,
            store_fp,
            set_fp,
        ]
    }
    /// Standard RISC-V function epilogue (restores ra/s0, deallocates frame).
    pub fn epilogue(&self) -> Vec<RiscVInstr> {
        let (load_ra, load_fp) = if self.is_64bit {
            (
                RiscVInstr::LD(RiscVReg::Ra, RiscVReg::S0, -8),
                RiscVInstr::LD(RiscVReg::S0, RiscVReg::S0, -16),
            )
        } else {
            (
                RiscVInstr::LW(RiscVReg::Ra, RiscVReg::S0, -4),
                RiscVInstr::LW(RiscVReg::S0, RiscVReg::S0, -8),
            )
        };
        vec![
            load_ra,
            load_fp,
            RiscVInstr::MV(RiscVReg::Sp, RiscVReg::S0),
            RiscVInstr::RET,
        ]
    }
    /// Argument registers per the RISC-V calling convention (a0 .. a7).
    pub fn calling_convention_args() -> Vec<RiscVReg> {
        vec![
            RiscVReg::A0,
            RiscVReg::A1,
            RiscVReg::A2,
            RiscVReg::A3,
            RiscVReg::A4,
            RiscVReg::A5,
            RiscVReg::A6,
            RiscVReg::A7,
        ]
    }
    /// Caller-saved (temporary) registers: t0 .. t6.
    pub fn caller_saved() -> Vec<RiscVReg> {
        vec![
            RiscVReg::T0,
            RiscVReg::T1,
            RiscVReg::T2,
            RiscVReg::T3,
            RiscVReg::T4,
            RiscVReg::T5,
            RiscVReg::T6,
        ]
    }
    /// Callee-saved registers: s0 .. s11.
    pub fn callee_saved() -> Vec<RiscVReg> {
        vec![
            RiscVReg::S0,
            RiscVReg::S1,
            RiscVReg::S2,
            RiscVReg::S3,
            RiscVReg::S4,
            RiscVReg::S5,
            RiscVReg::S6,
            RiscVReg::S7,
            RiscVReg::S8,
            RiscVReg::S9,
            RiscVReg::S10,
            RiscVReg::S11,
        ]
    }
}
/// Liveness analysis for RISCVExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct RISCVExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
impl RISCVExtLiveness {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        Self {
            live_in: vec![Vec::new(); n],
            live_out: vec![Vec::new(); n],
            defs: vec![Vec::new(); n],
            uses: vec![Vec::new(); n],
        }
    }
    #[allow(dead_code)]
    pub fn live_in(&self, b: usize, v: usize) -> bool {
        self.live_in.get(b).map(|s| s.contains(&v)).unwrap_or(false)
    }
    #[allow(dead_code)]
    pub fn live_out(&self, b: usize, v: usize) -> bool {
        self.live_out
            .get(b)
            .map(|s| s.contains(&v))
            .unwrap_or(false)
    }
    #[allow(dead_code)]
    pub fn add_def(&mut self, b: usize, v: usize) {
        if let Some(s) = self.defs.get_mut(b) {
            if !s.contains(&v) {
                s.push(v);
            }
        }
    }
    #[allow(dead_code)]
    pub fn add_use(&mut self, b: usize, v: usize) {
        if let Some(s) = self.uses.get_mut(b) {
            if !s.contains(&v) {
                s.push(v);
            }
        }
    }
    #[allow(dead_code)]
    pub fn var_is_used_in_block(&self, b: usize, v: usize) -> bool {
        self.uses.get(b).map(|s| s.contains(&v)).unwrap_or(false)
    }
    #[allow(dead_code)]
    pub fn var_is_def_in_block(&self, b: usize, v: usize) -> bool {
        self.defs.get(b).map(|s| s.contains(&v)).unwrap_or(false)
    }
}
/// Analysis cache for RISCVExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct RISCVExtCache {
    pub(crate) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(crate) cap: usize,
    pub(crate) total_hits: u64,
    pub(crate) total_misses: u64,
}
impl RISCVExtCache {
    #[allow(dead_code)]
    pub fn new(cap: usize) -> Self {
        Self {
            entries: Vec::new(),
            cap,
            total_hits: 0,
            total_misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: u64) -> Option<&[u8]> {
        for e in self.entries.iter_mut() {
            if e.0 == key && e.2 {
                e.3 += 1;
                self.total_hits += 1;
                return Some(&e.1);
            }
        }
        self.total_misses += 1;
        None
    }
    #[allow(dead_code)]
    pub fn put(&mut self, key: u64, data: Vec<u8>) {
        if self.entries.len() >= self.cap {
            self.entries.retain(|e| e.2);
            if self.entries.len() >= self.cap {
                self.entries.remove(0);
            }
        }
        self.entries.push((key, data, true, 0));
    }
    #[allow(dead_code)]
    pub fn invalidate(&mut self) {
        for e in self.entries.iter_mut() {
            e.2 = false;
        }
    }
    #[allow(dead_code)]
    pub fn hit_rate(&self) -> f64 {
        let t = self.total_hits + self.total_misses;
        if t == 0 {
            0.0
        } else {
            self.total_hits as f64 / t as f64
        }
    }
    #[allow(dead_code)]
    pub fn live_count(&self) -> usize {
        self.entries.iter().filter(|e| e.2).count()
    }
}
/// RISC-V register set (integer registers, ABI names).
#[derive(Debug, Clone, PartialEq)]
pub enum RiscVReg {
    Zero,
    Ra,
    Sp,
    Gp,
    Tp,
    T0,
    T1,
    T2,
    S0,
    S1,
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    S9,
    S10,
    S11,
    T3,
    T4,
    T5,
    T6,
}
impl RiscVReg {
    pub fn name(&self) -> &str {
        match self {
            RiscVReg::Zero => "zero",
            RiscVReg::Ra => "ra",
            RiscVReg::Sp => "sp",
            RiscVReg::Gp => "gp",
            RiscVReg::Tp => "tp",
            RiscVReg::T0 => "t0",
            RiscVReg::T1 => "t1",
            RiscVReg::T2 => "t2",
            RiscVReg::S0 => "s0",
            RiscVReg::S1 => "s1",
            RiscVReg::A0 => "a0",
            RiscVReg::A1 => "a1",
            RiscVReg::A2 => "a2",
            RiscVReg::A3 => "a3",
            RiscVReg::A4 => "a4",
            RiscVReg::A5 => "a5",
            RiscVReg::A6 => "a6",
            RiscVReg::A7 => "a7",
            RiscVReg::S2 => "s2",
            RiscVReg::S3 => "s3",
            RiscVReg::S4 => "s4",
            RiscVReg::S5 => "s5",
            RiscVReg::S6 => "s6",
            RiscVReg::S7 => "s7",
            RiscVReg::S8 => "s8",
            RiscVReg::S9 => "s9",
            RiscVReg::S10 => "s10",
            RiscVReg::S11 => "s11",
            RiscVReg::T3 => "t3",
            RiscVReg::T4 => "t4",
            RiscVReg::T5 => "t5",
            RiscVReg::T6 => "t6",
        }
    }
    /// x-register index (x0 .. x31).
    pub fn index(&self) -> u8 {
        match self {
            RiscVReg::Zero => 0,
            RiscVReg::Ra => 1,
            RiscVReg::Sp => 2,
            RiscVReg::Gp => 3,
            RiscVReg::Tp => 4,
            RiscVReg::T0 => 5,
            RiscVReg::T1 => 6,
            RiscVReg::T2 => 7,
            RiscVReg::S0 => 8,
            RiscVReg::S1 => 9,
            RiscVReg::A0 => 10,
            RiscVReg::A1 => 11,
            RiscVReg::A2 => 12,
            RiscVReg::A3 => 13,
            RiscVReg::A4 => 14,
            RiscVReg::A5 => 15,
            RiscVReg::A6 => 16,
            RiscVReg::A7 => 17,
            RiscVReg::S2 => 18,
            RiscVReg::S3 => 19,
            RiscVReg::S4 => 20,
            RiscVReg::S5 => 21,
            RiscVReg::S6 => 22,
            RiscVReg::S7 => 23,
            RiscVReg::S8 => 24,
            RiscVReg::S9 => 25,
            RiscVReg::S10 => 26,
            RiscVReg::S11 => 27,
            RiscVReg::T3 => 28,
            RiscVReg::T4 => 29,
            RiscVReg::T5 => 30,
            RiscVReg::T6 => 31,
        }
    }
}
/// Constant folding helper for RISCVExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct RISCVExtConstFolder {
    pub(crate) folds: usize,
    pub(crate) failures: usize,
    pub(crate) enabled: bool,
}
