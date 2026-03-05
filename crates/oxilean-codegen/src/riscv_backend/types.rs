//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet, VecDeque};

/// Pass registry for RISCVExt.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct RISCVExtPassRegistry {
    pub(super) configs: Vec<RISCVExtPassConfig>,
    pub(super) stats: Vec<RISCVExtPassStats>,
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
    pub(super) n: usize,
    pub(super) adj: Vec<Vec<usize>>,
    pub(super) rev: Vec<Vec<usize>>,
    pub(super) edge_count: usize,
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
    pub(super) items: std::collections::VecDeque<usize>,
    pub(super) present: Vec<bool>,
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
    pub(super) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(super) cap: usize,
    pub(super) total_hits: u64,
    pub(super) total_misses: u64,
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
    pub(super) folds: usize,
    pub(super) failures: usize,
    pub(super) enabled: bool,
}
impl RISCVExtConstFolder {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            folds: 0,
            failures: 0,
            enabled: true,
        }
    }
    #[allow(dead_code)]
    pub fn add_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        self.folds += 1;
        a.checked_add(b)
    }
    #[allow(dead_code)]
    pub fn sub_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        self.folds += 1;
        a.checked_sub(b)
    }
    #[allow(dead_code)]
    pub fn mul_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        self.folds += 1;
        a.checked_mul(b)
    }
    #[allow(dead_code)]
    pub fn div_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        if b == 0 {
            self.failures += 1;
            None
        } else {
            self.folds += 1;
            a.checked_div(b)
        }
    }
    #[allow(dead_code)]
    pub fn rem_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        if b == 0 {
            self.failures += 1;
            None
        } else {
            self.folds += 1;
            a.checked_rem(b)
        }
    }
    #[allow(dead_code)]
    pub fn neg_i64(&mut self, a: i64) -> Option<i64> {
        self.folds += 1;
        a.checked_neg()
    }
    #[allow(dead_code)]
    pub fn shl_i64(&mut self, a: i64, s: u32) -> Option<i64> {
        if s >= 64 {
            self.failures += 1;
            None
        } else {
            self.folds += 1;
            a.checked_shl(s)
        }
    }
    #[allow(dead_code)]
    pub fn shr_i64(&mut self, a: i64, s: u32) -> Option<i64> {
        if s >= 64 {
            self.failures += 1;
            None
        } else {
            self.folds += 1;
            a.checked_shr(s)
        }
    }
    #[allow(dead_code)]
    pub fn and_i64(&mut self, a: i64, b: i64) -> i64 {
        self.folds += 1;
        a & b
    }
    #[allow(dead_code)]
    pub fn or_i64(&mut self, a: i64, b: i64) -> i64 {
        self.folds += 1;
        a | b
    }
    #[allow(dead_code)]
    pub fn xor_i64(&mut self, a: i64, b: i64) -> i64 {
        self.folds += 1;
        a ^ b
    }
    #[allow(dead_code)]
    pub fn not_i64(&mut self, a: i64) -> i64 {
        self.folds += 1;
        !a
    }
    #[allow(dead_code)]
    pub fn fold_count(&self) -> usize {
        self.folds
    }
    #[allow(dead_code)]
    pub fn failure_count(&self) -> usize {
        self.failures
    }
    #[allow(dead_code)]
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    #[allow(dead_code)]
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    #[allow(dead_code)]
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RvDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl RvDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        RvDepGraph {
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
#[derive(Debug, Clone)]
pub struct RvAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, RvCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl RvAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        RvAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&RvCacheEntry> {
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
            RvCacheEntry {
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
/// Constant folding helper for RISCVX2.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct RISCVX2ConstFolder {
    pub(super) folds: usize,
    pub(super) failures: usize,
    pub(super) enabled: bool,
}
impl RISCVX2ConstFolder {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            folds: 0,
            failures: 0,
            enabled: true,
        }
    }
    #[allow(dead_code)]
    pub fn add_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        self.folds += 1;
        a.checked_add(b)
    }
    #[allow(dead_code)]
    pub fn sub_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        self.folds += 1;
        a.checked_sub(b)
    }
    #[allow(dead_code)]
    pub fn mul_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        self.folds += 1;
        a.checked_mul(b)
    }
    #[allow(dead_code)]
    pub fn div_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        if b == 0 {
            self.failures += 1;
            None
        } else {
            self.folds += 1;
            a.checked_div(b)
        }
    }
    #[allow(dead_code)]
    pub fn rem_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        if b == 0 {
            self.failures += 1;
            None
        } else {
            self.folds += 1;
            a.checked_rem(b)
        }
    }
    #[allow(dead_code)]
    pub fn neg_i64(&mut self, a: i64) -> Option<i64> {
        self.folds += 1;
        a.checked_neg()
    }
    #[allow(dead_code)]
    pub fn shl_i64(&mut self, a: i64, s: u32) -> Option<i64> {
        if s >= 64 {
            self.failures += 1;
            None
        } else {
            self.folds += 1;
            a.checked_shl(s)
        }
    }
    #[allow(dead_code)]
    pub fn shr_i64(&mut self, a: i64, s: u32) -> Option<i64> {
        if s >= 64 {
            self.failures += 1;
            None
        } else {
            self.folds += 1;
            a.checked_shr(s)
        }
    }
    #[allow(dead_code)]
    pub fn and_i64(&mut self, a: i64, b: i64) -> i64 {
        self.folds += 1;
        a & b
    }
    #[allow(dead_code)]
    pub fn or_i64(&mut self, a: i64, b: i64) -> i64 {
        self.folds += 1;
        a | b
    }
    #[allow(dead_code)]
    pub fn xor_i64(&mut self, a: i64, b: i64) -> i64 {
        self.folds += 1;
        a ^ b
    }
    #[allow(dead_code)]
    pub fn not_i64(&mut self, a: i64) -> i64 {
        self.folds += 1;
        !a
    }
    #[allow(dead_code)]
    pub fn fold_count(&self) -> usize {
        self.folds
    }
    #[allow(dead_code)]
    pub fn failure_count(&self) -> usize {
        self.failures
    }
    #[allow(dead_code)]
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    #[allow(dead_code)]
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    #[allow(dead_code)]
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}
/// Statistics for RISCVExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct RISCVExtPassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}
impl RISCVExtPassStats {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn visit(&mut self) {
        self.nodes_visited += 1;
    }
    #[allow(dead_code)]
    pub fn modify(&mut self) {
        self.nodes_modified += 1;
        self.changed = true;
    }
    #[allow(dead_code)]
    pub fn iterate(&mut self) {
        self.iterations += 1;
    }
    #[allow(dead_code)]
    pub fn error(&mut self) {
        self.errors += 1;
    }
    #[allow(dead_code)]
    pub fn efficiency(&self) -> f64 {
        if self.nodes_visited == 0 {
            0.0
        } else {
            self.nodes_modified as f64 / self.nodes_visited as f64
        }
    }
    #[allow(dead_code)]
    pub fn merge(&mut self, o: &RISCVExtPassStats) {
        self.iterations += o.iterations;
        self.changed |= o.changed;
        self.nodes_visited += o.nodes_visited;
        self.nodes_modified += o.nodes_modified;
        self.time_ms += o.time_ms;
        self.memory_bytes = self.memory_bytes.max(o.memory_bytes);
        self.errors += o.errors;
    }
}
/// RISC-V instructions (RV32I/RV64I base subset).
#[derive(Debug, Clone)]
pub enum RiscVInstr {
    LUI(RiscVReg, u32),
    AUIPC(RiscVReg, u32),
    JAL(RiscVReg, i32),
    JALR(RiscVReg, RiscVReg, i16),
    BEQ(RiscVReg, RiscVReg, i16),
    BNE(RiscVReg, RiscVReg, i16),
    BLT(RiscVReg, RiscVReg, i16),
    BGE(RiscVReg, RiscVReg, i16),
    BLTU(RiscVReg, RiscVReg, i16),
    BGEU(RiscVReg, RiscVReg, i16),
    LB(RiscVReg, RiscVReg, i16),
    LH(RiscVReg, RiscVReg, i16),
    LW(RiscVReg, RiscVReg, i16),
    LBU(RiscVReg, RiscVReg, i16),
    LHU(RiscVReg, RiscVReg, i16),
    LD(RiscVReg, RiscVReg, i16),
    LWU(RiscVReg, RiscVReg, i16),
    SB(RiscVReg, RiscVReg, i16),
    SH(RiscVReg, RiscVReg, i16),
    SW(RiscVReg, RiscVReg, i16),
    SD(RiscVReg, RiscVReg, i16),
    ADDI(RiscVReg, RiscVReg, i16),
    SLTI(RiscVReg, RiscVReg, i16),
    SLTIU(RiscVReg, RiscVReg, i16),
    XORI(RiscVReg, RiscVReg, i16),
    ORI(RiscVReg, RiscVReg, i16),
    ANDI(RiscVReg, RiscVReg, i16),
    SLLI(RiscVReg, RiscVReg, u8),
    SRLI(RiscVReg, RiscVReg, u8),
    SRAI(RiscVReg, RiscVReg, u8),
    ADDIW(RiscVReg, RiscVReg, i16),
    ADD(RiscVReg, RiscVReg, RiscVReg),
    SUB(RiscVReg, RiscVReg, RiscVReg),
    SLL(RiscVReg, RiscVReg, RiscVReg),
    SLT(RiscVReg, RiscVReg, RiscVReg),
    SLTU(RiscVReg, RiscVReg, RiscVReg),
    XOR(RiscVReg, RiscVReg, RiscVReg),
    SRL(RiscVReg, RiscVReg, RiscVReg),
    SRA(RiscVReg, RiscVReg, RiscVReg),
    OR(RiscVReg, RiscVReg, RiscVReg),
    AND(RiscVReg, RiscVReg, RiscVReg),
    MUL(RiscVReg, RiscVReg, RiscVReg),
    MULH(RiscVReg, RiscVReg, RiscVReg),
    DIV(RiscVReg, RiscVReg, RiscVReg),
    REM(RiscVReg, RiscVReg, RiscVReg),
    ECALL,
    EBREAK,
    Label(String),
    Directive(String, String),
    /// Pseudo-instruction: load immediate (expanded by assembler).
    LI(RiscVReg, i64),
    /// Pseudo-instruction: move.
    MV(RiscVReg, RiscVReg),
    /// Pseudo-instruction: no-op.
    NOP,
    /// Pseudo-instruction: return (jalr zero, ra, 0).
    RET,
    /// Pseudo-instruction: call function by name.
    CALL(String),
}
/// Pass registry for RISCVX2.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct RISCVX2PassRegistry {
    pub(super) configs: Vec<RISCVX2PassConfig>,
    pub(super) stats: Vec<RISCVX2PassStats>,
}
impl RISCVX2PassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn register(&mut self, c: RISCVX2PassConfig) {
        self.stats.push(RISCVX2PassStats::new());
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
    pub fn get(&self, i: usize) -> Option<&RISCVX2PassConfig> {
        self.configs.get(i)
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, i: usize) -> Option<&RISCVX2PassStats> {
        self.stats.get(i)
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&RISCVX2PassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn passes_in_phase(&self, ph: &RISCVX2PassPhase) -> Vec<&RISCVX2PassConfig> {
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
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum RvPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl RvPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            RvPassPhase::Analysis => "analysis",
            RvPassPhase::Transformation => "transformation",
            RvPassPhase::Verification => "verification",
            RvPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, RvPassPhase::Transformation | RvPassPhase::Cleanup)
    }
}
/// Pass execution phase for RISCVX2.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RISCVX2PassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
impl RISCVX2PassPhase {
    #[allow(dead_code)]
    pub fn is_early(&self) -> bool {
        matches!(self, Self::Early)
    }
    #[allow(dead_code)]
    pub fn is_middle(&self) -> bool {
        matches!(self, Self::Middle)
    }
    #[allow(dead_code)]
    pub fn is_late(&self) -> bool {
        matches!(self, Self::Late)
    }
    #[allow(dead_code)]
    pub fn is_finalize(&self) -> bool {
        matches!(self, Self::Finalize)
    }
    #[allow(dead_code)]
    pub fn order(&self) -> u32 {
        match self {
            Self::Early => 0,
            Self::Middle => 1,
            Self::Late => 2,
            Self::Finalize => 3,
        }
    }
    #[allow(dead_code)]
    pub fn from_order(n: u32) -> Option<Self> {
        match n {
            0 => Some(Self::Early),
            1 => Some(Self::Middle),
            2 => Some(Self::Late),
            3 => Some(Self::Finalize),
            _ => None,
        }
    }
}
/// Dominator tree for RISCVX2.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RISCVX2DomTree {
    pub(super) idom: Vec<Option<usize>>,
    pub(super) children: Vec<Vec<usize>>,
    pub(super) depth: Vec<usize>,
}
impl RISCVX2DomTree {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        Self {
            idom: vec![None; n],
            children: vec![Vec::new(); n],
            depth: vec![0; n],
        }
    }
    #[allow(dead_code)]
    pub fn set_idom(&mut self, node: usize, dom: usize) {
        if node < self.idom.len() {
            self.idom[node] = Some(dom);
            if dom < self.children.len() {
                self.children[dom].push(node);
            }
            self.depth[node] = if dom < self.depth.len() {
                self.depth[dom] + 1
            } else {
                1
            };
        }
    }
    #[allow(dead_code)]
    pub fn dominates(&self, a: usize, mut b: usize) -> bool {
        if a == b {
            return true;
        }
        let n = self.idom.len();
        for _ in 0..n {
            match self.idom.get(b).copied().flatten() {
                None => return false,
                Some(p) if p == a => return true,
                Some(p) if p == b => return false,
                Some(p) => b = p,
            }
        }
        false
    }
    #[allow(dead_code)]
    pub fn children_of(&self, n: usize) -> &[usize] {
        self.children.get(n).map(|v| v.as_slice()).unwrap_or(&[])
    }
    #[allow(dead_code)]
    pub fn depth_of(&self, n: usize) -> usize {
        self.depth.get(n).copied().unwrap_or(0)
    }
    #[allow(dead_code)]
    pub fn lca(&self, mut a: usize, mut b: usize) -> usize {
        let n = self.idom.len();
        for _ in 0..(2 * n) {
            if a == b {
                return a;
            }
            if self.depth_of(a) > self.depth_of(b) {
                a = self.idom.get(a).and_then(|x| *x).unwrap_or(a);
            } else {
                b = self.idom.get(b).and_then(|x| *x).unwrap_or(b);
            }
        }
        0
    }
}
/// Configuration for RISCVExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RISCVExtPassConfig {
    pub name: String,
    pub phase: RISCVExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
impl RISCVExtPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            phase: RISCVExtPassPhase::Middle,
            enabled: true,
            max_iterations: 100,
            debug: 0,
            timeout_ms: None,
        }
    }
    #[allow(dead_code)]
    pub fn with_phase(mut self, phase: RISCVExtPassPhase) -> Self {
        self.phase = phase;
        self
    }
    #[allow(dead_code)]
    pub fn with_max_iter(mut self, n: usize) -> Self {
        self.max_iterations = n;
        self
    }
    #[allow(dead_code)]
    pub fn with_debug(mut self, d: u32) -> Self {
        self.debug = d;
        self
    }
    #[allow(dead_code)]
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
    #[allow(dead_code)]
    pub fn with_timeout(mut self, ms: u64) -> Self {
        self.timeout_ms = Some(ms);
        self
    }
    #[allow(dead_code)]
    pub fn is_debug_enabled(&self) -> bool {
        self.debug > 0
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RvPassConfig {
    pub phase: RvPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl RvPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: RvPassPhase) -> Self {
        RvPassConfig {
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
/// Worklist for RISCVExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RISCVExtWorklist {
    pub(super) items: std::collections::VecDeque<usize>,
    pub(super) present: Vec<bool>,
}
impl RISCVExtWorklist {
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
/// Dominator tree for RISCVExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RISCVExtDomTree {
    pub(super) idom: Vec<Option<usize>>,
    pub(super) children: Vec<Vec<usize>>,
    pub(super) depth: Vec<usize>,
}
impl RISCVExtDomTree {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        Self {
            idom: vec![None; n],
            children: vec![Vec::new(); n],
            depth: vec![0; n],
        }
    }
    #[allow(dead_code)]
    pub fn set_idom(&mut self, node: usize, dom: usize) {
        if node < self.idom.len() {
            self.idom[node] = Some(dom);
            if dom < self.children.len() {
                self.children[dom].push(node);
            }
            self.depth[node] = if dom < self.depth.len() {
                self.depth[dom] + 1
            } else {
                1
            };
        }
    }
    #[allow(dead_code)]
    pub fn dominates(&self, a: usize, mut b: usize) -> bool {
        if a == b {
            return true;
        }
        let n = self.idom.len();
        for _ in 0..n {
            match self.idom.get(b).copied().flatten() {
                None => return false,
                Some(p) if p == a => return true,
                Some(p) if p == b => return false,
                Some(p) => b = p,
            }
        }
        false
    }
    #[allow(dead_code)]
    pub fn children_of(&self, n: usize) -> &[usize] {
        self.children.get(n).map(|v| v.as_slice()).unwrap_or(&[])
    }
    #[allow(dead_code)]
    pub fn depth_of(&self, n: usize) -> usize {
        self.depth.get(n).copied().unwrap_or(0)
    }
    #[allow(dead_code)]
    pub fn lca(&self, mut a: usize, mut b: usize) -> usize {
        let n = self.idom.len();
        for _ in 0..(2 * n) {
            if a == b {
                return a;
            }
            if self.depth_of(a) > self.depth_of(b) {
                a = self.idom.get(a).and_then(|x| *x).unwrap_or(a);
            } else {
                b = self.idom.get(b).and_then(|x| *x).unwrap_or(b);
            }
        }
        0
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RvWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl RvWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        RvWorklist {
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
#[derive(Debug, Clone, Default)]
pub struct RvPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl RvPassStats {
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
/// Pass execution phase for RISCVExt.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RISCVExtPassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
impl RISCVExtPassPhase {
    #[allow(dead_code)]
    pub fn is_early(&self) -> bool {
        matches!(self, Self::Early)
    }
    #[allow(dead_code)]
    pub fn is_middle(&self) -> bool {
        matches!(self, Self::Middle)
    }
    #[allow(dead_code)]
    pub fn is_late(&self) -> bool {
        matches!(self, Self::Late)
    }
    #[allow(dead_code)]
    pub fn is_finalize(&self) -> bool {
        matches!(self, Self::Finalize)
    }
    #[allow(dead_code)]
    pub fn order(&self) -> u32 {
        match self {
            Self::Early => 0,
            Self::Middle => 1,
            Self::Late => 2,
            Self::Finalize => 3,
        }
    }
    #[allow(dead_code)]
    pub fn from_order(n: u32) -> Option<Self> {
        match n {
            0 => Some(Self::Early),
            1 => Some(Self::Middle),
            2 => Some(Self::Late),
            3 => Some(Self::Finalize),
            _ => None,
        }
    }
}
/// Statistics for RISCVX2 passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct RISCVX2PassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}
impl RISCVX2PassStats {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn visit(&mut self) {
        self.nodes_visited += 1;
    }
    #[allow(dead_code)]
    pub fn modify(&mut self) {
        self.nodes_modified += 1;
        self.changed = true;
    }
    #[allow(dead_code)]
    pub fn iterate(&mut self) {
        self.iterations += 1;
    }
    #[allow(dead_code)]
    pub fn error(&mut self) {
        self.errors += 1;
    }
    #[allow(dead_code)]
    pub fn efficiency(&self) -> f64 {
        if self.nodes_visited == 0 {
            0.0
        } else {
            self.nodes_modified as f64 / self.nodes_visited as f64
        }
    }
    #[allow(dead_code)]
    pub fn merge(&mut self, o: &RISCVX2PassStats) {
        self.iterations += o.iterations;
        self.changed |= o.changed;
        self.nodes_visited += o.nodes_visited;
        self.nodes_modified += o.nodes_modified;
        self.time_ms += o.time_ms;
        self.memory_bytes = self.memory_bytes.max(o.memory_bytes);
        self.errors += o.errors;
    }
}
/// A named RISC-V function / code section.
#[derive(Debug, Clone)]
pub struct RiscVFunction {
    pub name: String,
    pub instrs: Vec<RiscVInstr>,
}
impl RiscVFunction {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            instrs: Vec::new(),
        }
    }
    pub fn push(&mut self, instr: RiscVInstr) {
        self.instrs.push(instr);
    }
}
/// Configuration for RISCVX2 passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RISCVX2PassConfig {
    pub name: String,
    pub phase: RISCVX2PassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
impl RISCVX2PassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            phase: RISCVX2PassPhase::Middle,
            enabled: true,
            max_iterations: 100,
            debug: 0,
            timeout_ms: None,
        }
    }
    #[allow(dead_code)]
    pub fn with_phase(mut self, phase: RISCVX2PassPhase) -> Self {
        self.phase = phase;
        self
    }
    #[allow(dead_code)]
    pub fn with_max_iter(mut self, n: usize) -> Self {
        self.max_iterations = n;
        self
    }
    #[allow(dead_code)]
    pub fn with_debug(mut self, d: u32) -> Self {
        self.debug = d;
        self
    }
    #[allow(dead_code)]
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
    #[allow(dead_code)]
    pub fn with_timeout(mut self, ms: u64) -> Self {
        self.timeout_ms = Some(ms);
        self
    }
    #[allow(dead_code)]
    pub fn is_debug_enabled(&self) -> bool {
        self.debug > 0
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RvCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
#[allow(dead_code)]
pub struct RvConstantFoldingHelper;
impl RvConstantFoldingHelper {
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
/// Analysis cache for RISCVX2.
#[allow(dead_code)]
#[derive(Debug)]
pub struct RISCVX2Cache {
    pub(super) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(super) cap: usize,
    pub(super) total_hits: u64,
    pub(super) total_misses: u64,
}
impl RISCVX2Cache {
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
/// Dependency graph for RISCVExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RISCVExtDepGraph {
    pub(super) n: usize,
    pub(super) adj: Vec<Vec<usize>>,
    pub(super) rev: Vec<Vec<usize>>,
    pub(super) edge_count: usize,
}
impl RISCVExtDepGraph {
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
#[allow(dead_code)]
pub struct RvPassRegistry {
    pub(super) configs: Vec<RvPassConfig>,
    pub(super) stats: std::collections::HashMap<String, RvPassStats>,
}
impl RvPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        RvPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: RvPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), RvPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&RvPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&RvPassStats> {
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
#[derive(Debug, Clone)]
pub struct RvLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl RvLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        RvLivenessInfo {
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
/// Liveness analysis for RISCVX2.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct RISCVX2Liveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
impl RISCVX2Liveness {
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
