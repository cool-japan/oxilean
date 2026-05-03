//! Implementation blocks (part 1)

use super::defs::*;
use std::collections::{HashMap, HashSet, VecDeque};

impl X86DepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        X86DepGraph {
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
impl X86ExtDomTree {
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
impl X86Backend {
    pub fn new() -> Self {
        Self
    }
    /// Emit a single instruction as AT&T syntax text.
    pub fn emit_instr(&self, instr: &X86Instr) -> String {
        match instr {
            X86Instr::Mov(dst, src) => {
                format!("    movq    {}, {}", src.name_att(), dst.name_att())
            }
            X86Instr::MovImm(dst, imm) => {
                format!("    movq    ${}, {}", imm, dst.name_att())
            }
            X86Instr::MovLoad(dst, mem) => {
                format!("    movq    {}, {}", mem.fmt_att(), dst.name_att())
            }
            X86Instr::MovStore(mem, src) => {
                format!("    movq    {}, {}", src.name_att(), mem.fmt_att())
            }
            X86Instr::MovImmStore(mem, imm) => {
                format!("    movq    ${}, {}", imm, mem.fmt_att())
            }
            X86Instr::Lea(dst, mem) => {
                format!("    leaq    {}, {}", mem.fmt_att(), dst.name_att())
            }
            X86Instr::Push(reg) => format!("    pushq   {}", reg.name_att()),
            X86Instr::Pop(reg) => format!("    popq    {}", reg.name_att()),
            X86Instr::Add(dst, src) => {
                format!("    addq    {}, {}", src.name_att(), dst.name_att())
            }
            X86Instr::AddImm(dst, imm) => {
                format!("    addq    ${}, {}", imm, dst.name_att())
            }
            X86Instr::Sub(dst, src) => {
                format!("    subq    {}, {}", src.name_att(), dst.name_att())
            }
            X86Instr::SubImm(dst, imm) => {
                format!("    subq    ${}, {}", imm, dst.name_att())
            }
            X86Instr::IMul(dst, src) => {
                format!("    imulq   {}, {}", src.name_att(), dst.name_att())
            }
            X86Instr::IDiv(src) => format!("    idivq   {}", src.name_att()),
            X86Instr::Neg(reg) => format!("    negq    {}", reg.name_att()),
            X86Instr::And(dst, src) => {
                format!("    andq    {}, {}", src.name_att(), dst.name_att())
            }
            X86Instr::AndImm(dst, imm) => {
                format!("    andq    ${}, {}", imm, dst.name_att())
            }
            X86Instr::Or(dst, src) => {
                format!("    orq     {}, {}", src.name_att(), dst.name_att())
            }
            X86Instr::OrImm(dst, imm) => {
                format!("    orq     ${}, {}", imm, dst.name_att())
            }
            X86Instr::Xor(dst, src) => {
                format!("    xorq    {}, {}", src.name_att(), dst.name_att())
            }
            X86Instr::XorImm(dst, imm) => {
                format!("    xorq    ${}, {}", imm, dst.name_att())
            }
            X86Instr::Not(reg) => format!("    notq    {}", reg.name_att()),
            X86Instr::Shl(reg, n) => format!("    shlq    ${}, {}", n, reg.name_att()),
            X86Instr::Shr(reg, n) => format!("    shrq    ${}, {}", n, reg.name_att()),
            X86Instr::Sar(reg, n) => format!("    sarq    ${}, {}", n, reg.name_att()),
            X86Instr::Cmp(a, b) => {
                format!("    cmpq    {}, {}", b.name_att(), a.name_att())
            }
            X86Instr::CmpImm(reg, imm) => {
                format!("    cmpq    ${}, {}", imm, reg.name_att())
            }
            X86Instr::Test(a, b) => {
                format!("    testq   {}, {}", b.name_att(), a.name_att())
            }
            X86Instr::SetE(reg) => format!("    sete    {}", reg.name_att()),
            X86Instr::SetNe(reg) => format!("    setne   {}", reg.name_att()),
            X86Instr::SetL(reg) => format!("    setl    {}", reg.name_att()),
            X86Instr::SetG(reg) => format!("    setg    {}", reg.name_att()),
            X86Instr::Jmp(lbl) => format!("    jmp     {}", lbl),
            X86Instr::Je(lbl) => format!("    je      {}", lbl),
            X86Instr::Jne(lbl) => format!("    jne     {}", lbl),
            X86Instr::Jl(lbl) => format!("    jl      {}", lbl),
            X86Instr::Jg(lbl) => format!("    jg      {}", lbl),
            X86Instr::Jle(lbl) => format!("    jle     {}", lbl),
            X86Instr::Jge(lbl) => format!("    jge     {}", lbl),
            X86Instr::Call(sym) => format!("    call    {}", sym),
            X86Instr::CallReg(reg) => format!("    call    *{}", reg.name_att()),
            X86Instr::Ret => "    ret".to_string(),
            X86Instr::Cqo => "    cqo".to_string(),
            X86Instr::MovsdLoad(dst, mem) => {
                format!("    movsd   {}, {}", mem.fmt_att(), dst.name_att())
            }
            X86Instr::MovsdStore(mem, src) => {
                format!("    movsd   {}, {}", src.name_att(), mem.fmt_att())
            }
            X86Instr::AddsdReg(dst, src) => {
                format!("    addsd   {}, {}", src.name_att(), dst.name_att())
            }
            X86Instr::SubsdReg(dst, src) => {
                format!("    subsd   {}, {}", src.name_att(), dst.name_att())
            }
            X86Instr::MulsdReg(dst, src) => {
                format!("    mulsd   {}, {}", src.name_att(), dst.name_att())
            }
            X86Instr::DivsdReg(dst, src) => {
                format!("    divsd   {}, {}", src.name_att(), dst.name_att())
            }
            X86Instr::Label(lbl) => format!("{}:", lbl),
            X86Instr::Directive(d, arg) => {
                if arg.is_empty() {
                    format!("    .{}", d)
                } else {
                    format!("    .{} {}", d, arg)
                }
            }
            X86Instr::Raw(text) => text.clone(),
        }
    }
    /// Emit a complete function with `.globl` / `.type` directives.
    pub fn emit_function(&self, func: &X86Function) -> String {
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
    /// Standard System V AMD64 ABI function prologue.
    ///
    /// Saves `%rbp`, sets up frame pointer, allocates `frame_size` bytes on
    /// the stack (rounded up to 16-byte alignment).
    pub fn prologue(&self, frame_size: u32) -> Vec<X86Instr> {
        let aligned = frame_size.div_ceil(16) * 16;
        vec![
            X86Instr::Push(X86Reg::RBP),
            X86Instr::Mov(X86Reg::RBP, X86Reg::RSP),
            X86Instr::SubImm(X86Reg::RSP, aligned as i32),
        ]
    }
    /// Standard System V AMD64 ABI function epilogue.
    pub fn epilogue(&self) -> Vec<X86Instr> {
        vec![
            X86Instr::Mov(X86Reg::RSP, X86Reg::RBP),
            X86Instr::Pop(X86Reg::RBP),
            X86Instr::Ret,
        ]
    }
    /// System V AMD64 integer argument registers (rdi, rsi, rdx, rcx, r8, r9).
    pub fn calling_convention_args() -> Vec<X86Reg> {
        vec![
            X86Reg::RDI,
            X86Reg::RSI,
            X86Reg::RDX,
            X86Reg::RCX,
            X86Reg::R8,
            X86Reg::R9,
        ]
    }
    /// Caller-saved (volatile) registers per SysV ABI.
    pub fn caller_saved() -> Vec<X86Reg> {
        vec![
            X86Reg::RAX,
            X86Reg::RCX,
            X86Reg::RDX,
            X86Reg::RSI,
            X86Reg::RDI,
            X86Reg::R8,
            X86Reg::R9,
            X86Reg::R10,
            X86Reg::R11,
        ]
    }
    /// Callee-saved (non-volatile) registers per SysV ABI.
    pub fn callee_saved() -> Vec<X86Reg> {
        vec![
            X86Reg::RBX,
            X86Reg::R12,
            X86Reg::R13,
            X86Reg::R14,
            X86Reg::R15,
            X86Reg::RBP,
        ]
    }
}
impl X86ExtPassStats {
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
    pub fn merge(&mut self, o: &X86ExtPassStats) {
        self.iterations += o.iterations;
        self.changed |= o.changed;
        self.nodes_visited += o.nodes_visited;
        self.nodes_modified += o.nodes_modified;
        self.time_ms += o.time_ms;
        self.memory_bytes = self.memory_bytes.max(o.memory_bytes);
        self.errors += o.errors;
    }
}
impl X86PassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            X86PassPhase::Analysis => "analysis",
            X86PassPhase::Transformation => "transformation",
            X86PassPhase::Verification => "verification",
            X86PassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, X86PassPhase::Transformation | X86PassPhase::Cleanup)
    }
}
impl X86Function {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            instrs: Vec::new(),
        }
    }
    pub fn push(&mut self, instr: X86Instr) {
        self.instrs.push(instr);
    }
}
impl X86PassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: X86PassPhase) -> Self {
        X86PassConfig {
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
impl X86ExtLiveness {
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
impl X86X2Cache {
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
impl X86PassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        X86PassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: X86PassConfig) {
        self.stats
            .insert(config.pass_name.clone(), X86PassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&X86PassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&X86PassStats> {
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
impl X86DominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        X86DominatorTree {
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
impl X86ExtDepGraph {
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
impl X86PassStats {
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
impl X86AnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        X86AnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&X86CacheEntry> {
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
            X86CacheEntry {
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
impl X86LivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        X86LivenessInfo {
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
impl X86X2Worklist {
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
impl X86ExtCache {
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
