use crate::lcnf::*;
use std::collections::{HashMap, HashSet, VecDeque};

use super::defs::*;

impl LLVMLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        LLVMLivenessInfo {
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

impl LLVMExtPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn register(&mut self, c: LLVMExtPassConfig) {
        self.stats.push(LLVMExtPassStats::new());
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
    pub fn get(&self, i: usize) -> Option<&LLVMExtPassConfig> {
        self.configs.get(i)
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, i: usize) -> Option<&LLVMExtPassStats> {
        self.stats.get(i)
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&LLVMExtPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn passes_in_phase(&self, ph: &LLVMExtPassPhase) -> Vec<&LLVMExtPassConfig> {
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

impl LLVMExtLiveness {
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

impl LLVMPassStats {
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

impl LLVMExtWorklist {
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

impl LlvmModule {
    /// Emit the full LLVM IR text for this module.
    pub fn emit(&self) -> String {
        let mut out = String::new();
        if !self.source_filename.is_empty() {
            out.push_str(&format!("source_filename = \"{}\"\n", self.source_filename));
        }
        if !self.target_triple.is_empty() {
            out.push_str(&format!("target triple = \"{}\"\n", self.target_triple));
        }
        if !self.data_layout.is_empty() {
            out.push_str(&format!("target datalayout = \"{}\"\n", self.data_layout));
        }
        if !self.source_filename.is_empty()
            || !self.target_triple.is_empty()
            || !self.data_layout.is_empty()
        {
            out.push('\n');
        }
        if !self.type_aliases.is_empty() {
            for alias in &self.type_aliases {
                out.push_str(&format!("{}\n", alias));
            }
            out.push('\n');
        }
        if !self.globals.is_empty() {
            for global in &self.globals {
                out.push_str(&format!("{}\n", global));
            }
            out.push('\n');
        }
        for func in &self.functions {
            out.push_str(&format!("{}\n", func));
        }
        if !self.metadata.is_empty() {
            out.push('\n');
            for (name, val) in &self.metadata {
                out.push_str(&format!("!{} = !{{{}}}\n", name, val));
            }
        }
        out
    }
}

impl LLVMExtConstFolder {
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

impl LLVMDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        LLVMDepGraph {
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

impl LLVMExtPassPhase {
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

impl LLVMExtPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            phase: LLVMExtPassPhase::Middle,
            enabled: true,
            max_iterations: 100,
            debug: 0,
            timeout_ms: None,
        }
    }
    #[allow(dead_code)]
    pub fn with_phase(mut self, phase: LLVMExtPassPhase) -> Self {
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

impl LLVMAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        LLVMAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&LLVMCacheEntry> {
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
            LLVMCacheEntry {
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

impl LLVMExtCache {
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

impl LLVMExtDepGraph {
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

impl LlvmBackend {
    /// Create a new `LlvmBackend`.
    pub fn new() -> Self {
        LlvmBackend { reg_counter: 0 }
    }
    /// Generate a fresh register name: `_r0`, `_r1`, ...
    pub fn fresh_reg(&mut self) -> String {
        let r = format!("_r{}", self.reg_counter);
        self.reg_counter += 1;
        r
    }
    /// Mangle a name for LLVM IR: replace `.`, `-`, spaces, and other
    /// non-alphanumeric characters (except `_`) with underscores.
    pub fn mangle_name(name: &str) -> String {
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
    /// Map an LCNF type to an LLVM type.
    pub fn llvm_type_for(ty: &LcnfType) -> LlvmType {
        match ty {
            LcnfType::Erased => LlvmType::I64,
            LcnfType::Var(_) => LlvmType::Ptr,
            LcnfType::Fun(_, _) => LlvmType::Ptr,
            LcnfType::Ctor(_, _) => LlvmType::Ptr,
            LcnfType::Object => LlvmType::Ptr,
            LcnfType::Nat => LlvmType::I64,
            LcnfType::LcnfString => LlvmType::Ptr,
            LcnfType::Unit => LlvmType::I64,
            LcnfType::Irrelevant => LlvmType::I64,
        }
    }
    /// Compile an LCNF function declaration to an LLVM function.
    pub fn compile_decl(&mut self, decl: &LcnfFunDecl) -> LlvmFunc {
        let mangled_name = Self::mangle_name(&decl.name);
        let ret_ty = Self::llvm_type_for(&decl.ret_type);
        let params: Vec<(LlvmType, String)> = decl
            .params
            .iter()
            .map(|p| {
                let ty = Self::llvm_type_for(&p.ty);
                let name = format!("p{}", p.id.0);
                (ty, name)
            })
            .collect();
        let mut body: Vec<LlvmInstr> = Vec::new();
        self.compile_expr(&decl.body, &ret_ty, &mut body);
        LlvmFunc {
            name: mangled_name,
            ret_ty,
            params,
            body,
            linkage: LlvmLinkage::External,
            attrs: vec![LlvmAttr::NoUnwind],
            is_declare: false,
        }
    }
    /// Recursively compile an LCNF expression into LLVM instructions.
    pub(crate) fn compile_expr(
        &mut self,
        expr: &LcnfExpr,
        ret_ty: &LlvmType,
        body: &mut Vec<LlvmInstr>,
    ) {
        match expr {
            LcnfExpr::Return(arg) => {
                let val = self.compile_arg(arg);
                if *ret_ty == LlvmType::Void {
                    body.push(LlvmInstr::Ret(None));
                } else {
                    body.push(LlvmInstr::Ret(Some((ret_ty.clone(), val))));
                }
            }
            LcnfExpr::Unreachable => {
                body.push(LlvmInstr::Unreachable);
            }
            LcnfExpr::Let {
                id,
                value,
                body: rest,
                ..
            } => {
                let reg = format!("x{}", id.0);
                self.compile_let_value(value, &reg, body);
                self.compile_expr(rest, ret_ty, body);
            }
            LcnfExpr::Case {
                scrutinee,
                alts,
                default,
                ..
            } => {
                let scrut_val = LlvmValue::LocalRef(format!("x{}", scrutinee.0));
                let merge_label = format!("merge_{}", self.fresh_reg());
                if alts.is_empty() {
                    if let Some(def) = default {
                        self.compile_expr(def, ret_ty, body);
                    } else {
                        body.push(LlvmInstr::Unreachable);
                    }
                    return;
                }
                let alt_labels: Vec<String> = alts
                    .iter()
                    .enumerate()
                    .map(|(i, a)| Self::mangle_name(&format!("case_{}_{}", a.ctor_name, i)))
                    .collect();
                let default_label = format!("default_{}", self.fresh_reg());
                for (i, (alt, label)) in alts.iter().zip(alt_labels.iter()).enumerate() {
                    let next_label = if i + 1 < alt_labels.len() {
                        alt_labels[i + 1].clone()
                    } else {
                        default_label.clone()
                    };
                    let tag_reg = self.fresh_reg();
                    body.push(LlvmInstr::ICmp {
                        result: tag_reg.clone(),
                        pred: IcmpPred::Eq,
                        lhs: scrut_val.clone(),
                        rhs: LlvmValue::Const(i as i64),
                    });
                    body.push(LlvmInstr::CondBr {
                        cond: LlvmValue::LocalRef(tag_reg),
                        true_: label.clone(),
                        false_: next_label,
                    });
                    body.push(LlvmInstr::Label(label.clone()));
                    self.compile_expr(&alt.body, ret_ty, body);
                    body.push(LlvmInstr::Br(merge_label.clone()));
                }
                body.push(LlvmInstr::Label(default_label));
                if let Some(def) = default {
                    self.compile_expr(def, ret_ty, body);
                } else {
                    body.push(LlvmInstr::Unreachable);
                }
                body.push(LlvmInstr::Br(merge_label.clone()));
                body.push(LlvmInstr::Label(merge_label));
            }
            LcnfExpr::TailCall(func_arg, args) => {
                let func_name = match func_arg {
                    LcnfArg::Var(id) => format!("x{}", id.0),
                    _ => "unknown".to_string(),
                };
                let mangled = Self::mangle_name(&func_name);
                let compiled_args: Vec<(LlvmType, LlvmValue)> = args
                    .iter()
                    .map(|a| (LlvmType::I64, self.compile_arg(a)))
                    .collect();
                let result_reg = if *ret_ty == LlvmType::Void {
                    None
                } else {
                    Some(self.fresh_reg())
                };
                let result_clone = result_reg.clone();
                body.push(LlvmInstr::Call {
                    result: result_reg,
                    ret_ty: ret_ty.clone(),
                    func: mangled,
                    args: compiled_args,
                });
                if let Some(r) = result_clone {
                    body.push(LlvmInstr::Ret(Some((
                        ret_ty.clone(),
                        LlvmValue::LocalRef(r),
                    ))));
                } else {
                    body.push(LlvmInstr::Ret(None));
                }
            }
        }
    }
    /// Compile an LCNF let-value into a named register.
    pub(crate) fn compile_let_value(
        &mut self,
        val: &LcnfLetValue,
        reg: &str,
        body: &mut Vec<LlvmInstr>,
    ) {
        match val {
            LcnfLetValue::Lit(lit) => {
                let const_val = match lit {
                    LcnfLit::Nat(n) => LlvmValue::Const(*n as i64),
                    LcnfLit::Str(_) => LlvmValue::Null,
                };
                body.push(LlvmInstr::Add {
                    result: reg.to_string(),
                    lhs: const_val,
                    rhs: LlvmValue::Const(0),
                });
            }
            LcnfLetValue::FVar(id) => {
                body.push(LlvmInstr::Add {
                    result: reg.to_string(),
                    lhs: LlvmValue::LocalRef(format!("x{}", id.0)),
                    rhs: LlvmValue::Const(0),
                });
            }
            LcnfLetValue::App(func_arg, args) => {
                let func_name = match func_arg {
                    LcnfArg::Var(id) => format!("x{}", id.0),
                    _ => "unknown".to_string(),
                };
                let mangled = Self::mangle_name(&func_name);
                let compiled_args: Vec<(LlvmType, LlvmValue)> = args
                    .iter()
                    .map(|a| (LlvmType::I64, self.compile_arg(a)))
                    .collect();
                body.push(LlvmInstr::Call {
                    result: Some(reg.to_string()),
                    ret_ty: LlvmType::I64,
                    func: mangled,
                    args: compiled_args,
                });
            }
            LcnfLetValue::Ctor(name, _tag, args) => {
                let mangled = format!("ctor_{}", Self::mangle_name(name));
                let compiled_args: Vec<(LlvmType, LlvmValue)> = args
                    .iter()
                    .map(|a| (LlvmType::I64, self.compile_arg(a)))
                    .collect();
                body.push(LlvmInstr::Call {
                    result: Some(reg.to_string()),
                    ret_ty: LlvmType::I64,
                    func: mangled,
                    args: compiled_args,
                });
            }
            LcnfLetValue::Proj(_name, idx, base) => {
                let gep_reg = self.fresh_reg();
                body.push(LlvmInstr::GetElementPtr {
                    result: gep_reg.clone(),
                    base_ty: LlvmType::I64,
                    ptr: LlvmValue::LocalRef(format!("x{}", base.0)),
                    indices: vec![
                        (LlvmType::I32, LlvmValue::Const(0)),
                        (LlvmType::I32, LlvmValue::Const(*idx as i64)),
                    ],
                });
                body.push(LlvmInstr::Load {
                    result: reg.to_string(),
                    ty: LlvmType::I64,
                    ptr: LlvmValue::LocalRef(gep_reg),
                    align: Some(8),
                });
            }
            LcnfLetValue::Reset(var) => {
                body.push(LlvmInstr::Add {
                    result: reg.to_string(),
                    lhs: LlvmValue::LocalRef(format!("x{}", var.0)),
                    rhs: LlvmValue::Const(0),
                });
            }
            LcnfLetValue::Reuse(_slot, name, _tag, args) => {
                let mangled = format!("ctor_{}", Self::mangle_name(name));
                let compiled_args: Vec<(LlvmType, LlvmValue)> = args
                    .iter()
                    .map(|a| (LlvmType::I64, self.compile_arg(a)))
                    .collect();
                body.push(LlvmInstr::Call {
                    result: Some(reg.to_string()),
                    ret_ty: LlvmType::I64,
                    func: mangled,
                    args: compiled_args,
                });
            }
            LcnfLetValue::Erased => {
                body.push(LlvmInstr::Add {
                    result: reg.to_string(),
                    lhs: LlvmValue::Const(0),
                    rhs: LlvmValue::Const(0),
                });
            }
        }
    }
    /// Compile an LCNF argument to an LLVM value.
    pub(crate) fn compile_arg(&self, arg: &LcnfArg) -> LlvmValue {
        match arg {
            LcnfArg::Var(id) => LlvmValue::LocalRef(format!("x{}", id.0)),
            LcnfArg::Lit(LcnfLit::Nat(n)) => LlvmValue::Const(*n as i64),
            LcnfArg::Lit(LcnfLit::Str(_)) => LlvmValue::Null,
            LcnfArg::Erased => LlvmValue::Const(0),
            LcnfArg::Type(_) => LlvmValue::Const(0),
        }
    }
    /// Compile all LCNF declarations and emit an LLVM IR module as text.
    pub fn emit_module(&mut self, decls: &[LcnfFunDecl]) -> Result<String, String> {
        let mut module = LlvmModule {
            source_filename: "oxilean_output.ll".to_string(),
            target_triple: "x86_64-unknown-linux-gnu".to_string(),
            data_layout: "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
                .to_string(),
            ..Default::default()
        };
        for decl in decls {
            let func = self.compile_decl(decl);
            module.functions.push(func);
        }
        Ok(module.emit())
    }
}

impl LLVMPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        LLVMPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: LLVMPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), LLVMPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&LLVMPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&LLVMPassStats> {
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

impl LLVMWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        LLVMWorklist {
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

impl LLVMConstantFoldingHelper {
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

impl LLVMExtDomTree {
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

impl LLVMExtPassStats {
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
    pub fn merge(&mut self, o: &LLVMExtPassStats) {
        self.iterations += o.iterations;
        self.changed |= o.changed;
        self.nodes_visited += o.nodes_visited;
        self.nodes_modified += o.nodes_modified;
        self.time_ms += o.time_ms;
        self.memory_bytes = self.memory_bytes.max(o.memory_bytes);
        self.errors += o.errors;
    }
}

impl LLVMPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            LLVMPassPhase::Analysis => "analysis",
            LLVMPassPhase::Transformation => "transformation",
            LLVMPassPhase::Verification => "verification",
            LLVMPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, LLVMPassPhase::Transformation | LLVMPassPhase::Cleanup)
    }
}

impl LLVMPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: LLVMPassPhase) -> Self {
        LLVMPassConfig {
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

impl LLVMDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        LLVMDominatorTree {
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
