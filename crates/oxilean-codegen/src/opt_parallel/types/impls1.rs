//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::defs::*;
use super::impls2::*;
use crate::lcnf::{LcnfArg, LcnfExpr, LcnfFunDecl, LcnfLetValue, LcnfVarId};
use std::collections::{HashMap, HashSet};

use super::super::functions::*;
use std::collections::VecDeque;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ParDepGraph {
    pub(crate) nodes: Vec<u32>,
    pub(crate) edges: Vec<(u32, u32)>,
}
impl ParDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        ParDepGraph {
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
/// A text buffer for building OPar output source code.
#[derive(Debug, Default)]
pub struct OParSourceBuffer {
    pub(crate) buf: String,
    pub(crate) indent_level: usize,
    pub(crate) indent_str: String,
}
impl OParSourceBuffer {
    pub fn new() -> Self {
        OParSourceBuffer {
            buf: String::new(),
            indent_level: 0,
            indent_str: "    ".to_string(),
        }
    }
    pub fn with_indent(mut self, indent: impl Into<String>) -> Self {
        self.indent_str = indent.into();
        self
    }
    pub fn push_line(&mut self, line: &str) {
        for _ in 0..self.indent_level {
            self.buf.push_str(&self.indent_str);
        }
        self.buf.push_str(line);
        self.buf.push('\n');
    }
    pub fn push_raw(&mut self, s: &str) {
        self.buf.push_str(s);
    }
    pub fn indent(&mut self) {
        self.indent_level += 1;
    }
    pub fn dedent(&mut self) {
        self.indent_level = self.indent_level.saturating_sub(1);
    }
    pub fn as_str(&self) -> &str {
        &self.buf
    }
    pub fn len(&self) -> usize {
        self.buf.len()
    }
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }
    pub fn line_count(&self) -> usize {
        self.buf.lines().count()
    }
    pub fn into_string(self) -> String {
        self.buf
    }
    pub fn reset(&mut self) {
        self.buf.clear();
        self.indent_level = 0;
    }
}
/// A fixed-capacity ring buffer of strings (for recent-event logging in OPar).
#[derive(Debug)]
pub struct OParEventLog {
    pub(crate) entries: std::collections::VecDeque<String>,
    pub(crate) capacity: usize,
}
impl OParEventLog {
    pub fn new(capacity: usize) -> Self {
        OParEventLog {
            entries: std::collections::VecDeque::with_capacity(capacity),
            capacity,
        }
    }
    pub fn push(&mut self, event: impl Into<String>) {
        if self.entries.len() >= self.capacity {
            self.entries.pop_front();
        }
        self.entries.push_back(event.into());
    }
    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.entries.iter()
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
/// Analysis cache for OParExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct OParExtCache {
    pub(crate) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(crate) cap: usize,
    pub(crate) total_hits: u64,
    pub(crate) total_misses: u64,
}
impl OParExtCache {
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
/// Analyses loop-carried dependences for a single LCNF function.
pub struct DependenceAnalyser<'a> {
    pub(crate) decl: &'a LcnfFunDecl,
}
impl<'a> DependenceAnalyser<'a> {
    pub(crate) fn new(decl: &'a LcnfFunDecl) -> Self {
        DependenceAnalyser { decl }
    }
    pub(crate) fn analyse(&self) -> DependenceInfo {
        let accesses = self.collect_accesses(&self.decl.body);
        let mut info = DependenceInfo::default();
        for i in 0..accesses.len() {
            for j in (i + 1)..accesses.len() {
                let a = &accesses[i];
                let b = &accesses[j];
                if a.independent_from(b) {
                    continue;
                }
                let edge = DepEdge {
                    from: format!("{}{}", a.base, a.offset),
                    to: format!("{}{}", b.base, b.offset),
                    distance: (b.offset - a.offset).abs(),
                };
                let is_loop_carried = edge.distance > 0;
                if a.is_write && !b.is_write {
                    info.true_deps.push(edge.clone());
                } else if !a.is_write && b.is_write {
                    info.anti_deps.push(edge.clone());
                } else if a.is_write && b.is_write {
                    info.output_deps.push(edge.clone());
                }
                if is_loop_carried {
                    info.loop_carried_deps.push(edge);
                }
            }
        }
        info
    }
    pub(crate) fn collect_accesses(&self, expr: &LcnfExpr) -> Vec<AffineAccess> {
        let mut out = Vec::new();
        self.collect_accesses_inner(expr, &mut out);
        out
    }
    pub(crate) fn collect_accesses_inner(&self, expr: &LcnfExpr, out: &mut Vec<AffineAccess>) {
        match expr {
            LcnfExpr::Let {
                value, body, name, ..
            } => {
                match value {
                    LcnfLetValue::App(LcnfArg::Var(fid), args) => {
                        let coeff = if args.len() > 1 { 1 } else { 0 };
                        out.push(AffineAccess {
                            base: name.clone(),
                            coeff,
                            offset: fid.0 as i64,
                            is_write: false,
                        });
                    }
                    LcnfLetValue::Ctor(ctor_name, tag, _) => {
                        out.push(AffineAccess {
                            base: ctor_name.clone(),
                            coeff: 1,
                            offset: *tag as i64,
                            is_write: true,
                        });
                    }
                    _ => {}
                }
                self.collect_accesses_inner(body, out);
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts {
                    self.collect_accesses_inner(&alt.body, out);
                }
                if let Some(d) = default {
                    self.collect_accesses_inner(d, out);
                }
            }
            _ => {}
        }
    }
}
/// Heuristic freshness key for OPar incremental compilation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OParIncrKey {
    pub content_hash: u64,
    pub config_hash: u64,
}
impl OParIncrKey {
    pub fn new(content: u64, config: u64) -> Self {
        OParIncrKey {
            content_hash: content,
            config_hash: config,
        }
    }
    pub fn combined_hash(&self) -> u64 {
        self.content_hash.wrapping_mul(0x9e3779b97f4a7c15) ^ self.config_hash
    }
    pub fn matches(&self, other: &OParIncrKey) -> bool {
        self.content_hash == other.content_hash && self.config_hash == other.config_hash
    }
}
/// A feature flag set for OPar capabilities.
#[derive(Debug, Clone, Default)]
pub struct OParFeatures {
    pub(crate) flags: std::collections::HashSet<String>,
}
impl OParFeatures {
    pub fn new() -> Self {
        OParFeatures::default()
    }
    pub fn enable(&mut self, flag: impl Into<String>) {
        self.flags.insert(flag.into());
    }
    pub fn disable(&mut self, flag: &str) {
        self.flags.remove(flag);
    }
    pub fn is_enabled(&self, flag: &str) -> bool {
        self.flags.contains(flag)
    }
    pub fn len(&self) -> usize {
        self.flags.len()
    }
    pub fn is_empty(&self) -> bool {
        self.flags.is_empty()
    }
    pub fn union(&self, other: &OParFeatures) -> OParFeatures {
        OParFeatures {
            flags: self.flags.union(&other.flags).cloned().collect(),
        }
    }
    pub fn intersection(&self, other: &OParFeatures) -> OParFeatures {
        OParFeatures {
            flags: self.flags.intersection(&other.flags).cloned().collect(),
        }
    }
}
/// A monotonically increasing ID generator for OPar.
#[derive(Debug, Default)]
pub struct OParIdGen {
    pub(crate) next: u32,
}
impl OParIdGen {
    pub fn new() -> Self {
        OParIdGen::default()
    }
    pub fn next_id(&mut self) -> u32 {
        let id = self.next;
        self.next += 1;
        id
    }
    pub fn peek_next(&self) -> u32 {
        self.next
    }
    pub fn reset(&mut self) {
        self.next = 0;
    }
    pub fn skip(&mut self, n: u32) {
        self.next += n;
    }
}
#[allow(dead_code)]
pub struct ParPassRegistry {
    pub(crate) configs: Vec<ParPassConfig>,
    pub(crate) stats: std::collections::HashMap<String, ParPassStats>,
}
impl ParPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        ParPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: ParPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), ParPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&ParPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&ParPassStats> {
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
pub struct ParDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl ParDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        ParDominatorTree {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ParWorklist {
    pub(crate) items: std::collections::VecDeque<u32>,
    pub(crate) in_worklist: std::collections::HashSet<u32>,
}
impl ParWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        ParWorklist {
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
/// Configuration for OParExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OParExtPassConfig {
    pub name: String,
    pub phase: OParExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
impl OParExtPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            phase: OParExtPassPhase::Middle,
            enabled: true,
            max_iterations: 100,
            debug: 0,
            timeout_ms: None,
        }
    }
    #[allow(dead_code)]
    pub fn with_phase(mut self, phase: OParExtPassPhase) -> Self {
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
#[derive(Debug, Clone, PartialEq)]
pub enum ParPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl ParPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            ParPassPhase::Analysis => "analysis",
            ParPassPhase::Transformation => "transformation",
            ParPassPhase::Verification => "verification",
            ParPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, ParPassPhase::Transformation | ParPassPhase::Cleanup)
    }
}
/// A single affine access of the form `base + coeff * loop_var + offset`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AffineAccess {
    /// Base array / variable name.
    pub base: String,
    /// Coefficient of the loop induction variable.
    pub coeff: i64,
    /// Constant offset.
    pub offset: i64,
    /// Whether this is a write access.
    pub is_write: bool,
}
impl AffineAccess {
    /// Two accesses are independent under Bernstein's conditions when their
    /// access sets are provably disjoint, i.e., there is no integer `i`, `j`
    /// (with `i != j` for loop-carried deps) satisfying both access functions.
    ///
    /// For 1-D affine accesses `base + c1*i + o1` vs `base + c2*j + o2` we
    /// require `c1 == c2` (same stride) to get a GCD test: `gcd(c1,c2) | (o1-o2)`.
    pub fn independent_from(&self, other: &AffineAccess) -> bool {
        if self.base != other.base {
            return true;
        }
        if !self.is_write && !other.is_write {
            return true;
        }
        let g = gcd(self.coeff.unsigned_abs(), other.coeff.unsigned_abs()) as i64;
        if g == 0 {
            return self.offset != other.offset;
        }
        (self.offset - other.offset) % g != 0
    }
}
/// The concrete algorithmic pattern recognised in a parallel region.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParallelPattern {
    /// `out[i] = f(in[i])` — embarrassingly parallel.
    Map,
    /// `out = [x for x in xs if p(x)]` — stream compaction.
    Filter,
    /// `acc = fold(f, init, xs)` — can use tree-reduction.
    Reduce,
    /// `out[i] = prefix_op(out[0..i])` — parallel-prefix.
    Scan,
    /// `out[i] = f(neighbours(in, i))` — finite-difference / convolution.
    Stencil,
    /// Generic counted loop with independent iterations.
    ParallelFor,
    /// Indexed scatter: `out[idx[i]] = val[i]`.
    Scatter,
    /// Indexed gather: `out[i] = in[idx[i]]`.
    Gather,
}
/// Constant folding helper for OParExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct OParExtConstFolder {
    pub(crate) folds: usize,
    pub(crate) failures: usize,
    pub(crate) enabled: bool,
}
impl OParExtConstFolder {
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
