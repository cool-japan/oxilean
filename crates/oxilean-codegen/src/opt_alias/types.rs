//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use crate::lcnf::{LcnfArg, LcnfExpr, LcnfFunDecl, LcnfLetValue, LcnfType, LcnfVarId};
use std::collections::{HashMap, HashSet, VecDeque};

/// Global alias summary for a function
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FuncAliasSummary {
    pub func_name: String,
    pub mem_effect: MemEffect,
    pub modifies: Vec<u32>,
    pub reads: Vec<u32>,
    pub return_aliases: Vec<u32>,
}
/// Whole-program alias summary
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct WholeProgramAlias {
    pub func_summaries: std::collections::HashMap<String, FuncAliasSummary>,
    pub global_points_to: std::collections::HashMap<u32, PointsToSet>,
}
#[allow(dead_code)]
impl WholeProgramAlias {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_func_summary(&mut self, summary: FuncAliasSummary) {
        self.func_summaries
            .insert(summary.func_name.clone(), summary);
    }
    pub fn get_func_summary(&self, func: &str) -> Option<&FuncAliasSummary> {
        self.func_summaries.get(func)
    }
    pub fn add_global_points_to(&mut self, var: u32, pts: PointsToSet) {
        self.global_points_to
            .entry(var)
            .or_default()
            .union_with(&pts);
    }
    pub fn func_count(&self) -> usize {
        self.func_summaries.len()
    }
}
/// Which level / flavor of alias analysis to run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AliasAnalysisLevel {
    /// Disable alias analysis (assume everything MayAlias).
    NoAlias,
    /// Basic alias analysis: trivial same-variable queries only.
    BasicAA,
    /// Type-based alias analysis (TBAA): use LCNF types to rule out aliases.
    TypeBasedAA,
    /// Scoped no-alias: use structural scoping to prove NoAlias.
    ScopedNoAliasAA,
    /// Global variables analysis: treat globals as separate alias class.
    GlobalsAA,
    /// CFL Andersen: context-free-language reachability, inclusion-based.
    #[default]
    CFLAndersen,
    /// CFL Steensgaard: context-free-language reachability, unification-based.
    CFLSteensgaard,
}
impl AliasAnalysisLevel {
    /// A human-readable description of this level.
    pub fn description(&self) -> &'static str {
        match self {
            AliasAnalysisLevel::NoAlias => "No alias analysis (conservative MayAlias)",
            AliasAnalysisLevel::BasicAA => "Basic AA (identity only)",
            AliasAnalysisLevel::TypeBasedAA => "Type-based AA (TBAA)",
            AliasAnalysisLevel::ScopedNoAliasAA => "Scoped no-alias AA",
            AliasAnalysisLevel::GlobalsAA => "Globals AA",
            AliasAnalysisLevel::CFLAndersen => "CFL Andersen (inclusion-based)",
            AliasAnalysisLevel::CFLSteensgaard => "CFL Steensgaard (unification-based)",
        }
    }
    /// Returns `true` if this level uses points-to analysis.
    pub fn uses_points_to(&self) -> bool {
        matches!(
            self,
            AliasAnalysisLevel::CFLAndersen | AliasAnalysisLevel::CFLSteensgaard
        )
    }
}
/// Memory effect annotation
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemEffect {
    ReadNone,
    ReadOnly,
    WriteOnly,
    ReadWrite,
    ArgMemOnly,
    InaccessibleMemOnly,
}
/// A node in the points-to graph representing an allocation site or variable.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PointsToNode {
    /// The variable ID this node represents.
    pub var: LcnfVarId,
    /// A human-readable label for debugging.
    pub label: String,
    /// The kind of node.
    pub kind: NodeKind,
}
impl PointsToNode {
    /// Create a new local node.
    pub fn local(var: LcnfVarId, label: impl Into<String>) -> Self {
        PointsToNode {
            var,
            label: label.into(),
            kind: NodeKind::Local,
        }
    }
    /// Create a new heap node.
    pub fn heap(var: LcnfVarId, label: impl Into<String>) -> Self {
        PointsToNode {
            var,
            label: label.into(),
            kind: NodeKind::Heap,
        }
    }
    /// Create a new parameter node.
    pub fn parameter(var: LcnfVarId, label: impl Into<String>) -> Self {
        PointsToNode {
            var,
            label: label.into(),
            kind: NodeKind::Parameter,
        }
    }
    /// Returns `true` if this node is a heap allocation site.
    pub fn is_heap(&self) -> bool {
        matches!(self.kind, NodeKind::Heap)
    }
    /// Returns `true` if this node is a local/stack variable.
    pub fn is_local(&self) -> bool {
        matches!(self.kind, NodeKind::Local)
    }
}
/// Alias analysis pass summary
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AliasPassSummary {
    pub pass_name: String,
    pub functions_analyzed: usize,
    pub queries_answered: usize,
    pub must_alias_rate: f64,
    pub no_alias_rate: f64,
    pub duration_us: u64,
}
/// Alias query batch
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct AliasQueryBatch {
    pub queries: Vec<(u32, u32)>,
    pub results: Vec<AliasResultExt>,
}
#[allow(dead_code)]
impl AliasQueryBatch {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add(&mut self, a: u32, b: u32) {
        self.queries.push((a, b));
    }
    pub fn record_result(&mut self, r: AliasResultExt) {
        self.results.push(r);
    }
    pub fn must_alias_count(&self) -> usize {
        self.results
            .iter()
            .filter(|r| **r == AliasResultExt::MustAlias)
            .count()
    }
    pub fn no_alias_count(&self) -> usize {
        self.results
            .iter()
            .filter(|r| **r == AliasResultExt::NoAlias)
            .count()
    }
}
/// TBAA tree (type hierarchy for alias analysis)
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct TbaaTree {
    pub nodes: std::collections::HashMap<String, TbaaTypeNode>,
}
#[allow(dead_code)]
impl TbaaTree {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_root(&mut self, name: &str) {
        self.nodes.insert(
            name.to_string(),
            TbaaTypeNode {
                name: name.to_string(),
                parent: None,
                is_const: false,
            },
        );
    }
    pub fn add_child(&mut self, name: &str, parent: &str, is_const: bool) {
        self.nodes.insert(
            name.to_string(),
            TbaaTypeNode {
                name: name.to_string(),
                parent: Some(parent.to_string()),
                is_const,
            },
        );
    }
    pub fn is_ancestor(&self, potential_ancestor: &str, node: &str) -> bool {
        let mut current = node;
        loop {
            if current == potential_ancestor {
                return true;
            }
            match self.nodes.get(current) {
                Some(n) => {
                    if let Some(p) = &n.parent {
                        current = p.as_str();
                    } else {
                        return false;
                    }
                }
                None => return false,
            }
        }
    }
    pub fn may_alias_tbaa(&self, t1: &str, t2: &str) -> bool {
        if t1 == t2 {
            return true;
        }
        self.is_ancestor(t1, t2) || self.is_ancestor(t2, t1)
    }
}
/// Inclusion-based (Andersen) points-to solver.
///
/// Processes constraints until a fixed point is reached.
/// Optionally field-sensitive.
#[derive(Debug, Clone, Default)]
pub struct AndersenSolver {
    /// Accumulated constraints.
    pub constraints: Vec<AndersenConstraint>,
    /// Current points-to graph.
    pub graph: PointsToGraph,
    /// Whether to use field-sensitive analysis.
    pub field_sensitive: bool,
    /// Number of solver iterations performed.
    pub iterations: u32,
}
impl AndersenSolver {
    /// Create a new solver.
    pub fn new() -> Self {
        AndersenSolver::default()
    }
    /// Create a field-sensitive solver.
    pub fn field_sensitive() -> Self {
        AndersenSolver {
            field_sensitive: true,
            ..AndersenSolver::default()
        }
    }
    /// Add an AddressOf constraint: `lhs ⊇ {&addr_of}`.
    pub fn add_address_of(&mut self, lhs: LcnfVarId, addr_of: LcnfVarId) {
        self.constraints
            .push(AndersenConstraint::AddressOf { lhs, addr_of });
    }
    /// Add a Copy constraint: `pts(lhs) ⊇ pts(rhs)`.
    pub fn add_copy(&mut self, lhs: LcnfVarId, rhs: LcnfVarId) {
        self.constraints.push(AndersenConstraint::Copy { lhs, rhs });
    }
    /// Add a Load constraint: `lhs = *ptr`.
    pub fn add_load(&mut self, lhs: LcnfVarId, ptr: LcnfVarId) {
        self.constraints.push(AndersenConstraint::Load { lhs, ptr });
    }
    /// Add a Store constraint: `*ptr = rhs`.
    pub fn add_store(&mut self, ptr: LcnfVarId, rhs: LcnfVarId) {
        self.constraints
            .push(AndersenConstraint::Store { ptr, rhs });
    }
    /// Register a variable in the underlying points-to graph.
    pub fn register_var(&mut self, node: PointsToNode) {
        self.graph.add_node(node);
    }
    /// Solve constraints to a fixed point.
    ///
    /// Uses a worklist algorithm for efficiency.
    pub fn solve(&mut self) {
        let mut changed = true;
        while changed {
            changed = false;
            self.iterations += 1;
            for constraint in self.constraints.clone() {
                match constraint {
                    AndersenConstraint::AddressOf { lhs, addr_of } => {
                        let pts = self.graph.pts.entry(lhs).or_default();
                        changed |= pts.insert(addr_of);
                    }
                    AndersenConstraint::Copy { lhs, rhs } => {
                        let pts_rhs: HashSet<LcnfVarId> = self.graph.pts_of(rhs).clone();
                        let pts_lhs = self.graph.pts.entry(lhs).or_default();
                        for v in &pts_rhs {
                            changed |= pts_lhs.insert(*v);
                        }
                    }
                    AndersenConstraint::Load { lhs, ptr } => {
                        let ptrs: Vec<LcnfVarId> =
                            self.graph.pts_of(ptr).clone().into_iter().collect();
                        for p in ptrs {
                            let pts_p: HashSet<LcnfVarId> = self.graph.pts_of(p).clone();
                            let pts_lhs = self.graph.pts.entry(lhs).or_default();
                            for v in &pts_p {
                                changed |= pts_lhs.insert(*v);
                            }
                        }
                    }
                    AndersenConstraint::Store { ptr, rhs } => {
                        let ptrs: Vec<LcnfVarId> =
                            self.graph.pts_of(ptr).clone().into_iter().collect();
                        let pts_rhs: HashSet<LcnfVarId> = self.graph.pts_of(rhs).clone();
                        for p in ptrs {
                            let pts_p = self.graph.pts.entry(p).or_default();
                            for v in &pts_rhs {
                                changed |= pts_p.insert(*v);
                            }
                        }
                    }
                }
            }
        }
    }
    /// Query alias relationship between two variables after solving.
    pub fn query(&self, a: LcnfVarId, b: LcnfVarId) -> AliasResult {
        if a == b {
            return AliasResult::MustAlias;
        }
        let pa = self.graph.pts_of(a);
        let pb = self.graph.pts_of(b);
        if pa.is_empty() || pb.is_empty() {
            return AliasResult::NoAlias;
        }
        let overlap: bool = pa.iter().any(|x| pb.contains(x));
        if overlap {
            if pa.len() == 1 && pb.len() == 1 {
                AliasResult::MustAlias
            } else {
                AliasResult::MayAlias
            }
        } else {
            AliasResult::NoAlias
        }
    }
}
/// Steensgaard-style union-find for alias analysis
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct SteensgaardUnionFind {
    pub parent: Vec<u32>,
    pub rank: Vec<u32>,
    pub points_to: Vec<Option<u32>>,
}
#[allow(dead_code)]
impl SteensgaardUnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            parent: (0..size as u32).collect(),
            rank: vec![0; size],
            points_to: vec![None; size],
        }
    }
    pub fn find(&mut self, x: u32) -> u32 {
        if self.parent[x as usize] != x {
            let root = self.find(self.parent[x as usize]);
            self.parent[x as usize] = root;
        }
        self.parent[x as usize]
    }
    pub fn union(&mut self, x: u32, y: u32) -> u32 {
        let rx = self.find(x);
        let ry = self.find(y);
        if rx == ry {
            return rx;
        }
        if self.rank[rx as usize] < self.rank[ry as usize] {
            self.parent[rx as usize] = ry;
            ry
        } else if self.rank[rx as usize] > self.rank[ry as usize] {
            self.parent[ry as usize] = rx;
            rx
        } else {
            self.parent[ry as usize] = rx;
            self.rank[rx as usize] += 1;
            rx
        }
    }
    pub fn same_class(&mut self, x: u32, y: u32) -> bool {
        self.find(x) == self.find(y)
    }
}
/// Alias analysis source buffer
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct AliasExtSourceBuffer {
    pub content: String,
}
#[allow(dead_code)]
impl AliasExtSourceBuffer {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn write(&mut self, s: &str) {
        self.content.push_str(s);
    }
    pub fn writeln(&mut self, s: &str) {
        self.content.push_str(s);
        self.content.push('\n');
    }
    pub fn finish(self) -> String {
        self.content
    }
}
/// Alias cache (for repeated queries)
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct AliasCache {
    pub cache: std::collections::HashMap<(u32, u32), AliasResultExt>,
    pub hits: u64,
    pub misses: u64,
}
#[allow(dead_code)]
impl AliasCache {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn get(&mut self, a: u32, b: u32) -> Option<AliasResultExt> {
        let key = if a <= b { (a, b) } else { (b, a) };
        if let Some(r) = self.cache.get(&key) {
            self.hits += 1;
            Some(r.clone())
        } else {
            self.misses += 1;
            None
        }
    }
    pub fn insert(&mut self, a: u32, b: u32, result: AliasResultExt) {
        let key = if a <= b { (a, b) } else { (b, a) };
        self.cache.insert(key, result);
    }
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }
    pub fn invalidate(&mut self) {
        self.cache.clear();
    }
}
/// Heap allocation summary
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HeapAllocSummary {
    pub alloc_id: u32,
    pub site: String,
    pub may_escape: bool,
    pub is_unique: bool,
    pub estimated_size: Option<u64>,
}
/// Alias analysis inline hint
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AliasInlineHint {
    pub call_site: u32,
    pub callee: String,
    pub benefit: f64,
    pub cost: f64,
}
impl AliasInlineHint {
    #[allow(dead_code)]
    pub fn should_inline(&self, threshold: f64) -> bool {
        self.benefit / self.cost.max(1.0) >= threshold
    }
}
/// The result of querying whether two values may alias.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AliasResult {
    /// The two values definitely refer to the same memory location.
    MustAlias,
    /// The two values may or may not refer to the same location.
    MayAlias,
    /// The two values definitely do not refer to the same location.
    NoAlias,
}
impl AliasResult {
    /// Returns `true` if aliasing is possible (MustAlias or MayAlias).
    pub fn may_alias(self) -> bool {
        matches!(self, AliasResult::MustAlias | AliasResult::MayAlias)
    }
    /// Returns `true` if aliasing is certain.
    pub fn must_alias(self) -> bool {
        matches!(self, AliasResult::MustAlias)
    }
    /// Returns `true` if no alias is possible.
    pub fn no_alias(self) -> bool {
        matches!(self, AliasResult::NoAlias)
    }
    /// Merge two alias results conservatively.
    pub fn merge(self, other: AliasResult) -> AliasResult {
        match (self, other) {
            (AliasResult::MustAlias, AliasResult::MustAlias) => AliasResult::MustAlias,
            (AliasResult::NoAlias, AliasResult::NoAlias) => AliasResult::NoAlias,
            _ => AliasResult::MayAlias,
        }
    }
}
/// Value numbering for alias analysis
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct AliasValueNumbering {
    pub map: std::collections::HashMap<String, u32>,
    pub next: u32,
}
#[allow(dead_code)]
impl AliasValueNumbering {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn get_or_insert(&mut self, key: &str) -> u32 {
        if let Some(&n) = self.map.get(key) {
            n
        } else {
            let n = self.next;
            self.next += 1;
            self.map.insert(key.to_string(), n);
            n
        }
    }
    pub fn lookup(&self, key: &str) -> Option<u32> {
        self.map.get(key).copied()
    }
}
/// CFL-Reachability alias analysis
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct CflReachabilityAnalysis {
    pub graph: Vec<Vec<(u32, String)>>,
    pub num_nodes: usize,
}
#[allow(dead_code)]
impl CflReachabilityAnalysis {
    pub fn new(n: usize) -> Self {
        Self {
            graph: vec![Vec::new(); n],
            num_nodes: n,
        }
    }
    pub fn add_edge(&mut self, from: u32, to: u32, label: &str) {
        if (from as usize) < self.num_nodes {
            self.graph[from as usize].push((to, label.to_string()));
        }
    }
    pub fn reachable(&self, src: u32, dst: u32) -> bool {
        if src == dst {
            return true;
        }
        let mut visited = vec![false; self.num_nodes];
        let mut stack = vec![src];
        while let Some(n) = stack.pop() {
            if n == dst {
                return true;
            }
            if n as usize >= self.num_nodes {
                continue;
            }
            if visited[n as usize] {
                continue;
            }
            visited[n as usize] = true;
            for (next, _) in &self.graph[n as usize] {
                stack.push(*next);
            }
        }
        false
    }
}
/// Alias analysis code stats
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct AliasCodeStats {
    pub functions_analyzed: usize,
    pub constraints_solved: usize,
    pub must_aliases_found: usize,
    pub no_aliases_found: usize,
    pub escape_candidates: usize,
    pub stack_promotions: usize,
}
/// Type-based alias analysis (TBAA) type descriptor
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TbaaTypeNode {
    pub name: String,
    pub parent: Option<String>,
    pub is_const: bool,
}
/// Alias analysis optimizer state
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct AliasOptimizerState {
    pub pipeline: AliasAnalysisPipeline,
    pub escape: EscapeAnalysis,
    pub hints: AliasHintCollector,
    pub refmod: RefModTable,
    pub stats: AliasCodeStats,
}
#[allow(dead_code)]
impl AliasOptimizerState {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn query(&mut self, a: u32, b: u32) -> AliasResultExt {
        self.pipeline.query(a, b)
    }
    pub fn report(&self) -> String {
        format!("{}", self.stats)
    }
}
/// Alias query result (extended)
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AliasResultExt {
    NoAlias,
    MayAlias,
    PartialAlias,
    MustAlias,
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MemSsaKind {
    MemDef,
    MemPhi,
    MemUse,
    LiveOnEntry,
}
/// Alias analysis feature flags
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct AliasFeatureFlags {
    pub enable_tbaa: bool,
    pub enable_field_sensitivity: bool,
    pub enable_flow_sensitivity: bool,
    pub enable_context_sensitivity: bool,
    pub enable_escape_analysis: bool,
    pub enable_cfl_reachability: bool,
}
/// Memory SSA (a simplified model for alias analysis)
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MemSsaDef {
    pub id: u32,
    pub version: u32,
    pub kind: MemSsaKind,
}
/// Tracks in-flight loads for forwarding.
#[derive(Debug, Clone, Default)]
pub struct LoadStoreForwarder {
    /// Maps (base_var, offset) → last-written value variable.
    pub store_cache: HashMap<(LcnfVarId, Option<i64>), LcnfVarId>,
    /// Number of forwarding substitutions performed.
    pub forwards_applied: usize,
}
impl LoadStoreForwarder {
    /// Create a new forwarder.
    pub fn new() -> Self {
        LoadStoreForwarder::default()
    }
    /// Record a store: `*base[offset] = val`.
    pub fn record_store(&mut self, base: LcnfVarId, offset: Option<i64>, val: LcnfVarId) {
        self.store_cache.insert((base, offset), val);
    }
    /// Attempt to forward a load from `base[offset]`.
    ///
    /// Returns the variable holding the cached value if available.
    pub fn forward_load(&self, base: LcnfVarId, offset: Option<i64>) -> Option<LcnfVarId> {
        self.store_cache.get(&(base, offset)).copied()
    }
    /// Invalidate all cache entries that may alias `base`.
    pub fn invalidate(&mut self, base: LcnfVarId) {
        self.store_cache.retain(|(b, _), _| *b != base);
    }
    /// Clear the entire cache (conservative: on unknown calls).
    pub fn clear(&mut self) {
        self.store_cache.clear();
    }
}
/// Abstract memory location for alias analysis
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MemLocation {
    Stack(u32),
    Heap(u32),
    Global(String),
    Field(Box<MemLocation>, String),
    Index(Box<MemLocation>, u32),
    Unknown,
}
/// Mod-Ref analysis result
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModRefResult {
    NoModRef,
    Ref,
    Mod,
    ModRef,
}
/// Describes a single memory access for alias comparison.
#[derive(Debug, Clone)]
pub struct MemoryAccessInfo {
    /// The base pointer variable being accessed.
    pub base: LcnfVarId,
    /// Byte offset from the base pointer (None = unknown).
    pub offset: Option<i64>,
    /// Size of the access in bytes (None = unknown).
    pub size: Option<u64>,
    /// Whether this is a volatile access.
    pub is_volatile: bool,
    /// The LCNF type of the accessed value.
    pub access_type: LcnfType,
    /// Whether this is a read or a write.
    pub is_write: bool,
}
impl MemoryAccessInfo {
    /// Create a simple read access with unknown offset/size.
    pub fn read(base: LcnfVarId, ty: LcnfType) -> Self {
        MemoryAccessInfo {
            base,
            offset: None,
            size: None,
            is_volatile: false,
            access_type: ty,
            is_write: false,
        }
    }
    /// Create a simple write access with unknown offset/size.
    pub fn write(base: LcnfVarId, ty: LcnfType) -> Self {
        MemoryAccessInfo {
            base,
            offset: None,
            size: None,
            is_volatile: false,
            access_type: ty,
            is_write: true,
        }
    }
    /// Returns `true` if offsets are known and do not overlap.
    pub fn definitely_disjoint(&self, other: &MemoryAccessInfo) -> bool {
        match (self.offset, self.size, other.offset, other.size) {
            (Some(o1), Some(s1), Some(o2), Some(s2)) => {
                let end1 = o1 + s1 as i64;
                let end2 = o2 + s2 as i64;
                end1 <= o2 || end2 <= o1
            }
            _ => false,
        }
    }
}
/// Andersen constraint solver
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct AndersenSolver2 {
    pub constraints: Vec<AndersenConstraint2>,
    pub points_to: std::collections::HashMap<u32, PointsToSet>,
    pub worklist: Vec<u32>,
}
#[allow(dead_code)]
impl AndersenSolver2 {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_constraint(&mut self, c: AndersenConstraint2) {
        self.constraints.push(c);
    }
    pub fn get_points_to(&self, var: u32) -> PointsToSet {
        self.points_to.get(&var).cloned().unwrap_or_default()
    }
    pub fn add_to_points_to(&mut self, var: u32, loc: MemLocation) -> bool {
        let set = self.points_to.entry(var).or_default();
        if set.locations.contains(&loc) {
            false
        } else {
            set.locations.insert(loc);
            true
        }
    }
    pub fn solve(&mut self) {
        let addrs: Vec<_> = self
            .constraints
            .iter()
            .filter_map(|c| {
                if let AndersenConstraint2::AddressOf { dest, src } = c {
                    Some((*dest, src.clone()))
                } else {
                    None
                }
            })
            .collect();
        for (dest, src) in addrs {
            self.add_to_points_to(dest, src);
            self.worklist.push(dest);
        }
        let copies: Vec<_> = self
            .constraints
            .iter()
            .filter_map(|c| {
                if let AndersenConstraint2::Copy { dest, src } = c {
                    Some((*dest, *src))
                } else {
                    None
                }
            })
            .collect();
        for _ in 0..10 {
            for (dest, src) in &copies {
                let src_set = self.get_points_to(*src);
                let locs: Vec<_> = src_set.locations.into_iter().collect();
                for loc in locs {
                    self.add_to_points_to(*dest, loc);
                }
            }
        }
    }
}
/// A constraint in Andersen-style points-to analysis.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AndersenConstraint {
    /// AddressOf: `x` contains the address of `y` (x ⊇ {&y}).
    AddressOf { lhs: LcnfVarId, addr_of: LcnfVarId },
    /// Copy: `x = y` — pts(x) ⊇ pts(y).
    Copy { lhs: LcnfVarId, rhs: LcnfVarId },
    /// Load: `x = *y` — for each p ∈ pts(y): pts(x) ⊇ pts(p).
    Load { lhs: LcnfVarId, ptr: LcnfVarId },
    /// Store: `*x = y` — for each p ∈ pts(x): pts(p) ⊇ pts(y).
    Store { ptr: LcnfVarId, rhs: LcnfVarId },
}
/// Alias analysis emit stats
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct AliasExtEmitStats {
    pub bytes_written: usize,
    pub items_emitted: usize,
    pub errors: usize,
    pub warnings: usize,
}
/// Alias analysis statistics (extended)
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct AliasStatsExt {
    pub queries_total: usize,
    pub must_alias_count: usize,
    pub no_alias_count: usize,
    pub may_alias_count: usize,
    pub partial_alias_count: usize,
    pub constraints_generated: usize,
    pub constraint_solving_iters: usize,
    pub points_to_sets_computed: usize,
}
/// Alias analysis version info
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AliasVersionInfo {
    pub pass_version: u32,
    pub supports_tbaa: bool,
    pub supports_field_sensitivity: bool,
    pub supports_context_sensitivity: bool,
    pub default_level: String,
}
/// Ref/Mod table (per instruction)
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct RefModTable {
    pub entries: std::collections::HashMap<u32, ModRefResult>,
}
#[allow(dead_code)]
impl RefModTable {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set(&mut self, inst: u32, mr: ModRefResult) {
        self.entries.insert(inst, mr);
    }
    pub fn get(&self, inst: u32) -> &ModRefResult {
        self.entries.get(&inst).unwrap_or(&ModRefResult::NoModRef)
    }
    pub fn reads_count(&self) -> usize {
        self.entries
            .values()
            .filter(|m| matches!(m, ModRefResult::Ref | ModRefResult::ModRef))
            .count()
    }
    pub fn writes_count(&self) -> usize {
        self.entries
            .values()
            .filter(|m| matches!(m, ModRefResult::Mod | ModRefResult::ModRef))
            .count()
    }
}
/// Alias-aware code motion hint
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AliasCodeMotionHint {
    pub inst_id: u32,
    pub target_block: u32,
    pub alias_safe: bool,
    pub estimated_savings: i32,
}
/// Andersen-style points-to constraint
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum AndersenConstraint2 {
    AddressOf { dest: u32, src: MemLocation },
    Copy { dest: u32, src: u32 },
    Load { dest: u32, ptr: u32 },
    Store { ptr: u32, src: u32 },
    Call { ret: u32, func: u32, args: Vec<u32> },
}
/// Escape analysis result
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct EscapeAnalysis {
    pub allocs: Vec<HeapAllocSummary>,
    pub escaping: std::collections::HashSet<u32>,
}
#[allow(dead_code)]
impl EscapeAnalysis {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_alloc(&mut self, summary: HeapAllocSummary) {
        if summary.may_escape {
            self.escaping.insert(summary.alloc_id);
        }
        self.allocs.push(summary);
    }
    pub fn does_escape(&self, alloc_id: u32) -> bool {
        self.escaping.contains(&alloc_id)
    }
    pub fn non_escaping_count(&self) -> usize {
        self.allocs.iter().filter(|a| !a.may_escape).count()
    }
}
/// A points-to graph mapping each variable to the set of nodes it may point to.
///
/// This implements a simplified Andersen / Steensgaard style analysis.
#[derive(Debug, Clone, Default)]
pub struct PointsToGraph {
    /// All known nodes in the graph.
    pub nodes: HashMap<LcnfVarId, PointsToNode>,
    /// `pts[x]` = set of node IDs that variable `x` may point to.
    pub pts: HashMap<LcnfVarId, HashSet<LcnfVarId>>,
    /// Whether this graph was built with Steensgaard (union-find) or Andersen style.
    pub is_steensgaard: bool,
}
impl PointsToGraph {
    /// Create a new empty points-to graph.
    pub fn new() -> Self {
        PointsToGraph::default()
    }
    /// Register a node in the graph.
    pub fn add_node(&mut self, node: PointsToNode) {
        let id = node.var;
        self.nodes.insert(id, node);
        self.pts.entry(id).or_default();
    }
    /// Add a points-to edge: `src` may point to `tgt`.
    pub fn add_pts(&mut self, src: LcnfVarId, tgt: LcnfVarId) {
        self.pts.entry(src).or_default().insert(tgt);
    }
    /// Retrieve the points-to set for `var`.
    pub fn pts_of(&self, var: LcnfVarId) -> &HashSet<LcnfVarId> {
        static EMPTY: std::sync::OnceLock<HashSet<LcnfVarId>> = std::sync::OnceLock::new();
        self.pts
            .get(&var)
            .unwrap_or_else(|| EMPTY.get_or_init(HashSet::new))
    }
    /// Check whether two variables share a common points-to target.
    pub fn intersects(&self, a: LcnfVarId, b: LcnfVarId) -> bool {
        let pa = self.pts_of(a);
        let pb = self.pts_of(b);
        pa.iter().any(|x| pb.contains(x))
    }
    /// Propagate points-to sets: if `from` ⊆ `to`, propagate pts(from) → pts(to).
    pub fn propagate(&mut self, from: LcnfVarId, to: LcnfVarId) {
        let pts_from: HashSet<LcnfVarId> = self.pts_of(from).clone();
        self.pts.entry(to).or_default().extend(pts_from);
    }
}
/// Alias analysis hint collector
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct AliasHintCollector {
    pub dead_stores: Vec<DeadStoreHint>,
    pub forwarded_loads: Vec<LoadForwardHint>,
}
#[allow(dead_code)]
impl AliasHintCollector {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_dead_store(&mut self, hint: DeadStoreHint) {
        self.dead_stores.push(hint);
    }
    pub fn add_load_forward(&mut self, hint: LoadForwardHint) {
        self.forwarded_loads.push(hint);
    }
    pub fn dead_store_count(&self) -> usize {
        self.dead_stores.len()
    }
    pub fn load_forward_count(&self) -> usize {
        self.forwarded_loads.len()
    }
}
/// TBAA metadata tag (for IR annotations)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TbaaTag {
    pub base_type: TbaaTypeNode,
    pub access_type: TbaaTypeNode,
    pub offset: u64,
    pub is_const: bool,
}
/// A set of variable IDs that may alias each other.
///
/// All variables in an alias set may point to the same memory location.
#[derive(Debug, Clone, Default)]
pub struct AliasSet {
    /// The representative (canonical) variable of this set.
    pub representative: Option<LcnfVarId>,
    /// All variables in this alias set.
    pub members: HashSet<LcnfVarId>,
    /// Whether this set contains any heap-allocated variables.
    pub has_heap: bool,
    /// Whether this set contains any stack-allocated variables.
    pub has_stack: bool,
    /// Whether this set contains any global variables.
    pub has_global: bool,
}
impl AliasSet {
    /// Create a new singleton alias set.
    pub fn singleton(var: LcnfVarId) -> Self {
        let mut members = HashSet::new();
        members.insert(var);
        AliasSet {
            representative: Some(var),
            members,
            has_heap: false,
            has_stack: false,
            has_global: false,
        }
    }
    /// Create an empty alias set.
    pub fn new() -> Self {
        AliasSet::default()
    }
    /// Insert a variable into this alias set.
    pub fn insert(&mut self, var: LcnfVarId) {
        if self.representative.is_none() {
            self.representative = Some(var);
        }
        self.members.insert(var);
    }
    /// Merge another alias set into this one.
    pub fn merge_with(&mut self, other: &AliasSet) {
        self.members.extend(other.members.iter().copied());
        self.has_heap |= other.has_heap;
        self.has_stack |= other.has_stack;
        self.has_global |= other.has_global;
        if self.representative.is_none() {
            self.representative = other.representative;
        }
    }
    /// Returns `true` if `var` is in this set.
    pub fn contains(&self, var: LcnfVarId) -> bool {
        self.members.contains(&var)
    }
    /// Number of members.
    pub fn len(&self) -> usize {
        self.members.len()
    }
    /// Returns `true` if no members.
    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }
}
/// Alias analysis profiler
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct AliasExtProfiler {
    pub timings: Vec<(String, u64)>,
}
#[allow(dead_code)]
impl AliasExtProfiler {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn record(&mut self, pass: &str, us: u64) {
        self.timings.push((pass.to_string(), us));
    }
    pub fn total_us(&self) -> u64 {
        self.timings.iter().map(|(_, t)| *t).sum()
    }
}
/// Classification of a points-to node.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NodeKind {
    /// A local (stack-allocated) variable.
    Local,
    /// A heap-allocated object (constructor application, closure).
    Heap,
    /// A global or top-level constant.
    Global,
    /// A function parameter.
    Parameter,
    /// A return value placeholder.
    Return,
    /// Unknown / conservative.
    Unknown,
}
/// Summary statistics from alias analysis.
#[derive(Debug, Clone, Default)]
pub struct AliasReport {
    /// Total number of alias pairs analyzed.
    pub pairs_analyzed: usize,
    /// Number of pairs that are MustAlias.
    pub must_alias: usize,
    /// Number of pairs that are MayAlias.
    pub may_alias: usize,
    /// Number of pairs that are NoAlias.
    pub no_alias: usize,
    /// The analysis level used.
    pub analysis_level: AliasAnalysisLevel,
    /// Number of load-store forwards performed.
    pub load_store_forwards: usize,
    /// Number of solver iterations (Andersen only).
    pub solver_iterations: u32,
}
impl AliasReport {
    /// Percentage of pairs proven NoAlias (precision metric).
    pub fn no_alias_ratio(&self) -> f64 {
        if self.pairs_analyzed == 0 {
            0.0
        } else {
            self.no_alias as f64 / self.pairs_analyzed as f64
        }
    }
    /// Percentage of pairs proven MustAlias.
    pub fn must_alias_ratio(&self) -> f64 {
        if self.pairs_analyzed == 0 {
            0.0
        } else {
            self.must_alias as f64 / self.pairs_analyzed as f64
        }
    }
}
/// Alias analysis id generator
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct AliasExtIdGen {
    pub(super) counter: u32,
}
#[allow(dead_code)]
impl AliasExtIdGen {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn next(&mut self) -> u32 {
        let id = self.counter;
        self.counter += 1;
        id
    }
}
/// The alias analysis pass.
///
/// Collects variables from LCNF declarations, builds constraints,
/// solves the points-to graph, and answers alias queries.
#[derive(Debug)]
pub struct AliasPass {
    /// The analysis level to use.
    pub level: AliasAnalysisLevel,
    /// The Andersen solver (used for CFL levels).
    pub solver: AndersenSolver,
    /// Type-based alias map: (var, type) pairs.
    pub type_map: HashMap<LcnfVarId, LcnfType>,
    /// Load-store forwarder.
    pub forwarder: LoadStoreForwarder,
    /// Accumulated report.
    pub(super) report: AliasReport,
    /// All queried pairs and their results.
    pub(super) query_cache: HashMap<(LcnfVarId, LcnfVarId), AliasResult>,
}
impl AliasPass {
    /// Create a new alias pass with default (CFLAndersen) level.
    pub fn new() -> Self {
        AliasPass {
            level: AliasAnalysisLevel::CFLAndersen,
            solver: AndersenSolver::new(),
            type_map: HashMap::new(),
            forwarder: LoadStoreForwarder::new(),
            report: AliasReport {
                analysis_level: AliasAnalysisLevel::CFLAndersen,
                ..Default::default()
            },
            query_cache: HashMap::new(),
        }
    }
    /// Create a pass with a specific analysis level.
    pub fn with_level(level: AliasAnalysisLevel) -> Self {
        AliasPass {
            level,
            solver: AndersenSolver::new(),
            type_map: HashMap::new(),
            forwarder: LoadStoreForwarder::new(),
            report: AliasReport {
                analysis_level: level,
                ..Default::default()
            },
            query_cache: HashMap::new(),
        }
    }
    /// Run alias analysis over all function declarations.
    pub fn run(&mut self, decls: &mut [LcnfFunDecl]) {
        for decl in decls.iter() {
            self.collect_decl(decl);
        }
        if self.level.uses_points_to() {
            self.solver.solve();
            self.report.solver_iterations = self.solver.iterations;
        }
    }
    /// Collect variables and constraints from a single function declaration.
    pub(super) fn collect_decl(&mut self, decl: &LcnfFunDecl) {
        for param in &decl.params {
            let node = PointsToNode::parameter(param.id, param.name.clone());
            self.solver.register_var(node);
            self.type_map.insert(param.id, param.ty.clone());
        }
        self.collect_expr(&decl.body);
    }
    /// Walk an LCNF expression and gather constraints.
    pub(super) fn collect_expr(&mut self, expr: &LcnfExpr) {
        match expr {
            LcnfExpr::Let {
                id,
                name,
                ty,
                value,
                body,
            } => {
                let node = PointsToNode::local(*id, name.clone());
                self.solver.register_var(node);
                self.type_map.insert(*id, ty.clone());
                self.collect_let_value(*id, value);
                self.collect_expr(body);
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts {
                    for param in &alt.params {
                        let node = PointsToNode::local(param.id, param.name.clone());
                        self.solver.register_var(node);
                        self.type_map.insert(param.id, param.ty.clone());
                    }
                    self.collect_expr(&alt.body);
                }
                if let Some(def) = default {
                    self.collect_expr(def);
                }
            }
            LcnfExpr::Return(_) | LcnfExpr::TailCall(_, _) | LcnfExpr::Unreachable => {}
        }
    }
    /// Gather constraints from a let-binding value.
    pub(super) fn collect_let_value(&mut self, lhs: LcnfVarId, value: &LcnfLetValue) {
        match value {
            LcnfLetValue::App(func_arg, _args) => {
                if let LcnfArg::Var(f) = func_arg {
                    self.solver.add_copy(lhs, *f);
                }
            }
            LcnfLetValue::Ctor(_, _, _args) => {
                self.solver.add_address_of(lhs, lhs);
            }
            LcnfLetValue::Reuse(slot, _, _, _args) => {
                self.solver.add_address_of(lhs, *slot);
            }
            LcnfLetValue::FVar(src) => {
                self.solver.add_copy(lhs, *src);
            }
            LcnfLetValue::Proj(_, _, src) => {
                self.solver.add_load(lhs, *src);
            }
            LcnfLetValue::Reset(src) => {
                self.solver.add_copy(lhs, *src);
            }
            LcnfLetValue::Lit(_) | LcnfLetValue::Erased => {}
        }
    }
    /// Query whether two variables alias after analysis.
    pub fn query(&mut self, a: LcnfVarId, b: LcnfVarId) -> AliasResult {
        let key = if a.0 <= b.0 { (a, b) } else { (b, a) };
        if let Some(&cached) = self.query_cache.get(&key) {
            return cached;
        }
        let result = self.compute_alias(a, b);
        self.query_cache.insert(key, result);
        self.report.pairs_analyzed += 1;
        match result {
            AliasResult::MustAlias => self.report.must_alias += 1,
            AliasResult::MayAlias => self.report.may_alias += 1,
            AliasResult::NoAlias => self.report.no_alias += 1,
        }
        result
    }
    /// Internal alias computation (uncached).
    pub(super) fn compute_alias(&self, a: LcnfVarId, b: LcnfVarId) -> AliasResult {
        match self.level {
            AliasAnalysisLevel::NoAlias => AliasResult::MayAlias,
            AliasAnalysisLevel::BasicAA => {
                if a == b {
                    AliasResult::MustAlias
                } else {
                    AliasResult::MayAlias
                }
            }
            AliasAnalysisLevel::TypeBasedAA => {
                if a == b {
                    return AliasResult::MustAlias;
                }
                if let (Some(ta), Some(tb)) = (self.type_map.get(&a), self.type_map.get(&b)) {
                    if tbaa_no_alias(ta, tb) {
                        return AliasResult::NoAlias;
                    }
                }
                AliasResult::MayAlias
            }
            AliasAnalysisLevel::ScopedNoAliasAA => {
                if a == b {
                    return AliasResult::MustAlias;
                }
                let a_is_alloc = self.solver.graph.pts_of(a).contains(&a);
                let b_is_alloc = self.solver.graph.pts_of(b).contains(&b);
                if a_is_alloc && b_is_alloc {
                    AliasResult::NoAlias
                } else {
                    AliasResult::MayAlias
                }
            }
            AliasAnalysisLevel::GlobalsAA => {
                if a == b {
                    return AliasResult::MustAlias;
                }
                let a_global = self
                    .solver
                    .graph
                    .nodes
                    .get(&a)
                    .map(|n| matches!(n.kind, NodeKind::Global))
                    .unwrap_or(false);
                let b_global = self
                    .solver
                    .graph
                    .nodes
                    .get(&b)
                    .map(|n| matches!(n.kind, NodeKind::Global))
                    .unwrap_or(false);
                if a_global != b_global {
                    AliasResult::NoAlias
                } else {
                    AliasResult::MayAlias
                }
            }
            AliasAnalysisLevel::CFLAndersen | AliasAnalysisLevel::CFLSteensgaard => {
                self.solver.query(a, b)
            }
        }
    }
    /// Return a copy of the accumulated report.
    pub fn report(&self) -> AliasReport {
        self.report.clone()
    }
    /// Apply load-store forwarding to all declarations (using alias info).
    pub fn apply_load_store_forwarding(&mut self, decls: &mut [LcnfFunDecl]) {
        for decl in decls.iter_mut() {
            apply_forwarding_to_expr(&mut decl.body, &mut self.forwarder);
        }
        self.report.load_store_forwards = self.forwarder.forwards_applied;
    }
}
/// Points-to set (abstract)
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct PointsToSet {
    pub locations: std::collections::HashSet<MemLocation>,
    pub may_alias_unknown: bool,
}
#[allow(dead_code)]
impl PointsToSet {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn top() -> Self {
        Self {
            locations: std::collections::HashSet::new(),
            may_alias_unknown: true,
        }
    }
    pub fn singleton(loc: MemLocation) -> Self {
        let mut s = Self::new();
        s.locations.insert(loc);
        s
    }
    pub fn add(&mut self, loc: MemLocation) {
        self.locations.insert(loc);
    }
    pub fn union_with(&mut self, other: &PointsToSet) {
        self.may_alias_unknown |= other.may_alias_unknown;
        for loc in &other.locations {
            self.locations.insert(loc.clone());
        }
    }
    pub fn may_alias(&self, other: &PointsToSet) -> bool {
        if self.may_alias_unknown || other.may_alias_unknown {
            return true;
        }
        self.locations
            .iter()
            .any(|loc| other.locations.contains(loc))
    }
    pub fn must_alias(&self, other: &PointsToSet) -> bool {
        if self.may_alias_unknown || other.may_alias_unknown {
            return false;
        }
        self.locations.len() == 1 && other.locations.len() == 1 && self.locations == other.locations
    }
    pub fn is_empty(&self) -> bool {
        self.locations.is_empty() && !self.may_alias_unknown
    }
    pub fn size(&self) -> usize {
        self.locations.len()
    }
}
/// Alias analysis pipeline (compose multiple analyses)
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct AliasAnalysisPipeline {
    pub passes: Vec<String>,
    pub cache: AliasCache,
    pub stats: AliasStatsExt,
    pub tbaa: TbaaTree,
}
#[allow(dead_code)]
impl AliasAnalysisPipeline {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_pass(&mut self, name: &str) {
        self.passes.push(name.to_string());
    }
    pub fn query(&mut self, a: u32, b: u32) -> AliasResultExt {
        self.stats.queries_total += 1;
        if let Some(cached) = self.cache.get(a, b) {
            return cached;
        }
        let result = AliasResultExt::MayAlias;
        self.stats.may_alias_count += 1;
        self.cache.insert(a, b, result.clone());
        result
    }
    pub fn query_modref(&self, _func: u32, _loc: u32) -> ModRefResult {
        ModRefResult::ModRef
    }
}
/// Alias analysis pass configuration (extended)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AliasConfigExt {
    pub level: String,
    pub max_iterations: usize,
    pub max_points_to_size: usize,
    pub enable_field_sensitivity: bool,
    pub enable_flow_sensitivity: bool,
    pub enable_context_sensitivity: bool,
    pub track_heap: bool,
    pub track_globals: bool,
}
/// Alias-based dead store elimination hint
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DeadStoreHint {
    pub store_id: u32,
    pub overwritten_by: u32,
    pub alias_result: AliasResultExt,
}
/// Load-store forwarding hint
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LoadForwardHint {
    pub load_id: u32,
    pub from_store: u32,
    pub is_exact: bool,
}
