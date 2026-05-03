//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::defs::*;
use super::impls1::*;
use crate::lcnf::{LcnfArg, LcnfExpr, LcnfFunDecl, LcnfLetValue, LcnfVarId};
use std::collections::{HashMap, HashSet};

use super::super::functions::*;
use std::collections::VecDeque;

/// Detects potential data races using a simplified Lamport happened-before model.
///
/// We conservatively flag any pair of accesses to the same variable where at
/// least one is a write and there is no ordering edge between them.
pub(crate) struct RaceDetector {
    /// Known ordering edges: `(a, b)` means `a` happens-before `b`.
    pub(crate) happens_before: HashSet<(LcnfVarId, LcnfVarId)>,
}
impl RaceDetector {
    pub(crate) fn new() -> Self {
        RaceDetector {
            happens_before: HashSet::new(),
        }
    }
    pub(crate) fn add_ordering(&mut self, before: LcnfVarId, after: LcnfVarId) {
        self.happens_before.insert((before, after));
    }
    pub(crate) fn may_race(
        &self,
        a: LcnfVarId,
        b: LcnfVarId,
        a_is_write: bool,
        b_is_write: bool,
    ) -> bool {
        if !a_is_write && !b_is_write {
            return false;
        }
        !self.happens_before.contains(&(a, b)) && !self.happens_before.contains(&(b, a))
    }
    pub(crate) fn analyse_decl(&self, decl: &LcnfFunDecl) -> ThreadSafetyInfo {
        let mut races = Vec::new();
        let mut atomics = Vec::new();
        let accesses = Self::collect_var_accesses(&decl.body);
        let writes: Vec<_> = accesses
            .iter()
            .filter(|(_, is_write, _)| *is_write)
            .collect();
        let reads: Vec<_> = accesses
            .iter()
            .filter(|(_, is_write, _)| !*is_write)
            .collect();
        for (wid, _, wname) in &writes {
            for (rid, _, rname) in &reads {
                if wid != rid && self.may_race(*wid, *rid, true, false) {
                    races.push((wname.clone(), rname.clone()));
                }
            }
            for (wid2, _, wname2) in &writes {
                if wid != wid2 && self.may_race(*wid, *wid2, true, true) {
                    atomics.push(wname.clone());
                    atomics.push(wname2.clone());
                }
            }
        }
        atomics.sort();
        atomics.dedup();
        ThreadSafetyInfo {
            is_thread_safe: races.is_empty() && atomics.is_empty(),
            race_conditions: races,
            atomic_ops_needed: atomics,
        }
    }
    pub(crate) fn collect_var_accesses(expr: &LcnfExpr) -> Vec<(LcnfVarId, bool, String)> {
        let mut out = Vec::new();
        Self::collect_inner(expr, &mut out);
        out
    }
    pub(crate) fn collect_inner(expr: &LcnfExpr, out: &mut Vec<(LcnfVarId, bool, String)>) {
        match expr {
            LcnfExpr::Let {
                id,
                name,
                value,
                body,
                ..
            } => {
                let is_write = matches!(
                    value, LcnfLetValue::Ctor(n, _, _) if n.contains("write") || n
                    .contains("store")
                );
                out.push((*id, is_write, name.clone()));
                Self::collect_inner(body, out);
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts {
                    Self::collect_inner(&alt.body, out);
                }
                if let Some(d) = default {
                    Self::collect_inner(d, out);
                }
            }
            _ => {}
        }
    }
}
/// A generic key-value configuration store for OPar.
#[derive(Debug, Clone, Default)]
pub struct OParConfig {
    pub(crate) entries: std::collections::HashMap<String, String>,
}
impl OParConfig {
    pub fn new() -> Self {
        OParConfig::default()
    }
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.entries.insert(key.into(), value.into());
    }
    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries.get(key).map(|s| s.as_str())
    }
    pub fn get_bool(&self, key: &str) -> bool {
        matches!(self.get(key), Some("true") | Some("1") | Some("yes"))
    }
    pub fn get_int(&self, key: &str) -> Option<i64> {
        self.get(key)?.parse().ok()
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ParAnalysisCache {
    pub(crate) entries: std::collections::HashMap<String, ParCacheEntry>,
    pub(crate) max_size: usize,
    pub(crate) hits: u64,
    pub(crate) misses: u64,
}
impl ParAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        ParAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&ParCacheEntry> {
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
            ParCacheEntry {
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
/// Dependency graph for OParExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OParExtDepGraph {
    pub(crate) n: usize,
    pub(crate) adj: Vec<Vec<usize>>,
    pub(crate) rev: Vec<Vec<usize>>,
    pub(crate) edge_count: usize,
}
impl OParExtDepGraph {
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
/// Emission statistics for OPar.
#[derive(Debug, Clone, Default)]
pub struct OParEmitStats {
    pub bytes_emitted: usize,
    pub items_emitted: usize,
    pub errors: usize,
    pub warnings: usize,
    pub elapsed_ms: u64,
}
impl OParEmitStats {
    pub fn new() -> Self {
        OParEmitStats::default()
    }
    pub fn throughput_bps(&self) -> f64 {
        if self.elapsed_ms == 0 {
            0.0
        } else {
            self.bytes_emitted as f64 / (self.elapsed_ms as f64 / 1000.0)
        }
    }
    pub fn is_clean(&self) -> bool {
        self.errors == 0
    }
}
/// A version tag for OPar output artifacts.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OParVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub pre: Option<String>,
}
impl OParVersion {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        OParVersion {
            major,
            minor,
            patch,
            pre: None,
        }
    }
    pub fn with_pre(mut self, pre: impl Into<String>) -> Self {
        self.pre = Some(pre.into());
        self
    }
    pub fn is_stable(&self) -> bool {
        self.pre.is_none()
    }
    pub fn is_compatible_with(&self, other: &OParVersion) -> bool {
        self.major == other.major && self.minor >= other.minor
    }
}
/// Tracks declared names for OPar scope analysis.
#[derive(Debug, Default)]
pub struct OParNameScope {
    pub(crate) declared: std::collections::HashSet<String>,
    pub(crate) depth: usize,
    pub(crate) parent: Option<Box<OParNameScope>>,
}
impl OParNameScope {
    pub fn new() -> Self {
        OParNameScope::default()
    }
    pub fn declare(&mut self, name: impl Into<String>) -> bool {
        self.declared.insert(name.into())
    }
    pub fn is_declared(&self, name: &str) -> bool {
        self.declared.contains(name)
    }
    pub fn push_scope(self) -> Self {
        OParNameScope {
            declared: std::collections::HashSet::new(),
            depth: self.depth + 1,
            parent: Some(Box::new(self)),
        }
    }
    pub fn pop_scope(self) -> Self {
        *self.parent.unwrap_or_default()
    }
    pub fn depth(&self) -> usize {
        self.depth
    }
    pub fn len(&self) -> usize {
        self.declared.len()
    }
}
/// Pass-timing record for OPar profiler.
#[derive(Debug, Clone)]
pub struct OParPassTiming {
    pub pass_name: String,
    pub elapsed_us: u64,
    pub items_processed: usize,
    pub bytes_before: usize,
    pub bytes_after: usize,
}
impl OParPassTiming {
    pub fn new(
        pass_name: impl Into<String>,
        elapsed_us: u64,
        items: usize,
        before: usize,
        after: usize,
    ) -> Self {
        OParPassTiming {
            pass_name: pass_name.into(),
            elapsed_us,
            items_processed: items,
            bytes_before: before,
            bytes_after: after,
        }
    }
    pub fn throughput_mps(&self) -> f64 {
        if self.elapsed_us == 0 {
            0.0
        } else {
            self.items_processed as f64 / (self.elapsed_us as f64 / 1_000_000.0)
        }
    }
    pub fn size_ratio(&self) -> f64 {
        if self.bytes_before == 0 {
            1.0
        } else {
            self.bytes_after as f64 / self.bytes_before as f64
        }
    }
    pub fn is_profitable(&self) -> bool {
        self.size_ratio() <= 1.05
    }
}
/// Pipeline profiler for OPar.
#[derive(Debug, Default)]
pub struct OParProfiler {
    pub(crate) timings: Vec<OParPassTiming>,
}
impl OParProfiler {
    pub fn new() -> Self {
        OParProfiler::default()
    }
    pub fn record(&mut self, t: OParPassTiming) {
        self.timings.push(t);
    }
    pub fn total_elapsed_us(&self) -> u64 {
        self.timings.iter().map(|t| t.elapsed_us).sum()
    }
    pub fn slowest_pass(&self) -> Option<&OParPassTiming> {
        self.timings.iter().max_by_key(|t| t.elapsed_us)
    }
    pub fn num_passes(&self) -> usize {
        self.timings.len()
    }
    pub fn profitable_passes(&self) -> Vec<&OParPassTiming> {
        self.timings.iter().filter(|t| t.is_profitable()).collect()
    }
}
/// High-level classification of a parallel execution model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParallelKind {
    /// Independent iterations can run simultaneously (SIMD / OpenMP parallel-for).
    DataParallel,
    /// Independent sub-computations (futures / async tasks).
    TaskParallel,
    /// Producer–consumer stages overlap in time.
    PipelineParallel,
    /// Evaluate multiple branches speculatively and discard losers.
    SpeculativeParallel,
}
/// Liveness analysis for OParExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct OParExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
impl OParExtLiveness {
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
#[allow(dead_code)]
pub struct ParConstantFoldingHelper;
impl ParConstantFoldingHelper {
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
/// Heuristic detector for common parallel patterns over LCNF.
///
/// We walk the function body looking for structural clues:
/// - A tail-recursive function whose accumulator is updated unconditionally
///   is a `Reduce`.
/// - A tail-recursive function that writes an output array at index `i` and
///   reads only from the input at `i` is a `Map`.
/// - Otherwise, if the loop body is side-effect-free we classify as `ParallelFor`.
pub struct PatternDetector<'a> {
    pub(crate) decl: &'a LcnfFunDecl,
}
impl<'a> PatternDetector<'a> {
    pub(crate) fn new(decl: &'a LcnfFunDecl) -> Self {
        PatternDetector { decl }
    }
    pub(crate) fn detect(&self) -> Option<ParallelPattern> {
        if !self.decl.is_recursive {
            return None;
        }
        let reads = self.collect_reads(&self.decl.body);
        let writes = self.collect_writes(&self.decl.body);
        let has_accumulator = self.has_accumulator_update(&self.decl.body);
        let has_index_write = self.has_index_write(&self.decl.body);
        let has_index_read = !reads.is_empty();
        if has_accumulator && !has_index_write {
            return Some(ParallelPattern::Reduce);
        }
        if has_index_write && has_index_read {
            let write_bases: HashSet<&str> = writes.iter().map(|s| s.as_str()).collect();
            let read_bases: HashSet<&str> = reads.iter().map(|s| s.as_str()).collect();
            if write_bases == read_bases {
                return Some(ParallelPattern::Stencil);
            }
            return Some(ParallelPattern::Map);
        }
        if has_index_write && !has_index_read {
            return Some(ParallelPattern::Scatter);
        }
        if !has_index_write && has_index_read {
            return Some(ParallelPattern::Gather);
        }
        if self.has_filter_pattern(&self.decl.body) {
            return Some(ParallelPattern::Filter);
        }
        if self.has_scan_pattern(&self.decl.body) {
            return Some(ParallelPattern::Scan);
        }
        Some(ParallelPattern::ParallelFor)
    }
    pub(crate) fn collect_reads(&self, expr: &LcnfExpr) -> Vec<String> {
        let mut out = Vec::new();
        self.collect_reads_inner(expr, &mut out);
        out
    }
    pub(crate) fn collect_reads_inner(&self, expr: &LcnfExpr, out: &mut Vec<String>) {
        match expr {
            LcnfExpr::Let { value, body, .. } => {
                if let LcnfLetValue::App(LcnfArg::Var(id), _args) = value {
                    out.push(format!("read_{}", id.0));
                }
                self.collect_reads_inner(body, out);
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts {
                    self.collect_reads_inner(&alt.body, out);
                }
                if let Some(d) = default {
                    self.collect_reads_inner(d, out);
                }
            }
            LcnfExpr::Return(_) | LcnfExpr::TailCall(_, _) | LcnfExpr::Unreachable => {}
        }
    }
    pub(crate) fn collect_writes(&self, expr: &LcnfExpr) -> Vec<String> {
        let mut out = Vec::new();
        self.collect_writes_inner(expr, &mut out);
        out
    }
    pub(crate) fn collect_writes_inner(&self, expr: &LcnfExpr, out: &mut Vec<String>) {
        match expr {
            LcnfExpr::Let {
                value, body, name, ..
            } => {
                if let LcnfLetValue::Ctor(ctor_name, _, _) = value {
                    if ctor_name.contains("write") || ctor_name.contains("store") {
                        out.push(name.clone());
                    }
                }
                self.collect_writes_inner(body, out);
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts {
                    self.collect_writes_inner(&alt.body, out);
                }
                if let Some(d) = default {
                    self.collect_writes_inner(d, out);
                }
            }
            LcnfExpr::Return(_) | LcnfExpr::TailCall(_, _) | LcnfExpr::Unreachable => {}
        }
    }
    pub(crate) fn has_accumulator_update(&self, expr: &LcnfExpr) -> bool {
        match expr {
            LcnfExpr::Let { value, body, .. } => {
                if let LcnfLetValue::App(LcnfArg::Var(fid), args) = value {
                    let is_binop = args.len() == 2;
                    let is_arith = fid.0 < 16;
                    if is_binop && is_arith {
                        return true;
                    }
                }
                self.has_accumulator_update(body)
            }
            LcnfExpr::Case { alts, default, .. } => {
                alts.iter().any(|a| self.has_accumulator_update(&a.body))
                    || default
                        .as_ref()
                        .map(|d| self.has_accumulator_update(d))
                        .unwrap_or(false)
            }
            _ => false,
        }
    }
    pub(crate) fn has_index_write(&self, expr: &LcnfExpr) -> bool {
        match expr {
            LcnfExpr::Let { value, body, .. } => {
                matches!(
                    value, LcnfLetValue::Ctor(n, _, _) if n.contains("write") || n
                    .contains("store")
                ) || self.has_index_write(body)
            }
            LcnfExpr::Case { alts, default, .. } => {
                alts.iter().any(|a| self.has_index_write(&a.body))
                    || default
                        .as_ref()
                        .map(|d| self.has_index_write(d))
                        .unwrap_or(false)
            }
            _ => false,
        }
    }
    pub(crate) fn has_filter_pattern(&self, expr: &LcnfExpr) -> bool {
        matches!(expr, LcnfExpr::Case { alts, .. } if alts.len() == 2)
            || match expr {
                LcnfExpr::Let { body, .. } => self.has_filter_pattern(body),
                _ => false,
            }
    }
    pub(crate) fn has_scan_pattern(&self, expr: &LcnfExpr) -> bool {
        self.has_accumulator_update(expr) && self.has_index_write(expr)
    }
}
