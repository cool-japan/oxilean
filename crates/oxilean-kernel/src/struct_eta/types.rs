//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use crate::{Environment, Expr, Level, Name};

/// A set of projection rewrite rules for a kernel.
#[allow(dead_code)]
pub struct ProjectionRewriteSet {
    rules: Vec<ProjectionRewrite>,
}
#[allow(dead_code)]
impl ProjectionRewriteSet {
    /// Create an empty set.
    pub fn new() -> Self {
        ProjectionRewriteSet { rules: Vec::new() }
    }
    /// Add a rule.
    pub fn add(&mut self, rule: ProjectionRewrite) {
        self.rules.push(rule);
    }
    /// Look up a rule by projector name.
    pub fn find_by_projector(&self, projector: &str) -> Option<&ProjectionRewrite> {
        self.rules.iter().find(|r| r.projector_name == projector)
    }
    /// Look up all rules for a constructor.
    pub fn rules_for_ctor(&self, ctor: &str) -> Vec<&ProjectionRewrite> {
        self.rules.iter().filter(|r| r.ctor_name == ctor).collect()
    }
    /// Return the number of rules.
    pub fn len(&self) -> usize {
        self.rules.len()
    }
    /// Return whether there are no rules.
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
    /// Return all projector names.
    pub fn projector_names(&self) -> Vec<&str> {
        self.rules
            .iter()
            .map(|r| r.projector_name.as_str())
            .collect()
    }
}
/// Eta-expanded form: a record constructed from its own projections.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EtaExpanded {
    /// The structure constructor name.
    pub ctor: String,
    /// The expression being eta-expanded.
    pub expr_id: u64,
    /// The field values (as expression ids).
    pub field_ids: Vec<u64>,
}
#[allow(dead_code)]
impl EtaExpanded {
    /// Build an eta-expanded form given ctor name and projection ids.
    pub fn new(ctor: impl Into<String>, expr_id: u64, field_ids: Vec<u64>) -> Self {
        EtaExpanded {
            ctor: ctor.into(),
            expr_id,
            field_ids,
        }
    }
    /// Return the arity (number of fields).
    pub fn arity(&self) -> usize {
        self.field_ids.len()
    }
}
/// Collects all eta-redexes found during a traversal.
#[allow(dead_code)]
pub struct EtaRedexCollector {
    redexes: Vec<EtaRedex>,
    max_depth: usize,
}
#[allow(dead_code)]
impl EtaRedexCollector {
    /// Create a collector with an unlimited depth.
    pub fn new() -> Self {
        EtaRedexCollector {
            redexes: Vec::new(),
            max_depth: usize::MAX,
        }
    }
    /// Create a collector that only collects redexes up to a given depth.
    pub fn with_max_depth(max_depth: usize) -> Self {
        EtaRedexCollector {
            redexes: Vec::new(),
            max_depth,
        }
    }
    /// Add a found redex.
    pub fn add(&mut self, redex: EtaRedex) {
        if redex.depth() <= self.max_depth {
            self.redexes.push(redex);
        }
    }
    /// Return all collected redexes.
    pub fn redexes(&self) -> &[EtaRedex] {
        &self.redexes
    }
    /// Return the number of redexes found.
    pub fn count(&self) -> usize {
        self.redexes.len()
    }
    /// Return whether any top-level redex was found.
    pub fn has_top_level(&self) -> bool {
        self.redexes.iter().any(|r| r.is_top_level())
    }
    /// Return redexes sorted by depth (shallowest first).
    pub fn sorted_by_depth(&self) -> Vec<&EtaRedex> {
        let mut v: Vec<&EtaRedex> = self.redexes.iter().collect();
        v.sort_by_key(|r| r.depth());
        v
    }
}
/// Performs K-like reduction on singleton types.
///
/// A *singleton type* (in OxiLean's sense) is an inductive type with
/// - exactly 1 constructor, and
/// - 0 non-parameter fields.
///
/// All elements of a singleton type are definitionally equal to the unique
/// constructor (applied only to its uniform parameters).  This enables a
/// restricted form of the K axiom: pattern matching on self-equality can be
/// simplified because any proof of `a = a` must equal `rfl`.
pub struct SingletonKReducer<'a> {
    env: &'a Environment,
}
impl<'a> SingletonKReducer<'a> {
    /// Create a new `SingletonKReducer` bound to the given environment.
    pub fn new(env: &'a Environment) -> Self {
        SingletonKReducer { env }
    }
    /// Return `true` when `ty` (looked up as a `Const` head) has exactly 1
    /// constructor with 0 non-parameter fields.
    pub fn is_singleton_type(&self, ty: &Expr) -> bool {
        let name = match head_const(ty) {
            Some(n) => n,
            None => return false,
        };
        let iv = match self.env.get_inductive_val(name) {
            Some(v) => v,
            None => return false,
        };
        if iv.ctors.len() != 1 {
            return false;
        }
        let ctor_name = &iv.ctors[0];
        match self.env.get_constructor_val(ctor_name) {
            Some(cv) => cv.num_fields == 0,
            None => false,
        }
    }
    /// Return the unique constructor name for `type_name` if it is a singleton.
    pub fn get_unique_ctor(&self, type_name: &Name) -> Option<Name> {
        let iv = self.env.get_inductive_val(type_name)?;
        if iv.ctors.len() != 1 {
            return None;
        }
        let ctor_name = iv.ctors[0].clone();
        let cv = self.env.get_constructor_val(&ctor_name)?;
        if cv.num_fields == 0 {
            Some(ctor_name)
        } else {
            None
        }
    }
    /// K-reduce `expr : ty`.
    ///
    /// For a singleton type the canonical element is the unique zero-field
    /// constructor constant.  Returns `Some(canonical)` when applicable,
    /// `None` otherwise.
    pub fn k_reduce(&self, _expr: &Expr, ty: &Expr) -> Option<Expr> {
        if !self.is_singleton_type(ty) {
            return None;
        }
        let type_name = head_const(ty)?;
        let ctor_name = self.get_unique_ctor(type_name)?;
        Some(Expr::Const(ctor_name, vec![]))
    }
    /// Attempt to simplify `Rec(motive, proof)` on a singleton type.
    ///
    /// When both `motive` and `proof` involve a singleton type, the match can
    /// be eliminated because `proof` must equal the canonical constructor.
    /// Returns `Some(simplified)` when a simplification is found.
    pub fn apply_k_reduction(motive: &Expr, proof: &Expr) -> Option<Expr> {
        let simplified = Expr::App(Box::new(motive.clone()), Box::new(proof.clone()));
        Some(simplified)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StructureDef {
    pub name: String,
    pub ctor_name: String,
    pub fields: Vec<FieldDescriptor>,
}
/// Counts eta-expansion and reduction events for instrumentation.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct EtaStats {
    pub expansions: u64,
    pub reductions: u64,
    pub failed_expansions: u64,
    pub failed_reductions: u64,
}
#[allow(dead_code)]
impl EtaStats {
    /// Create zeroed stats.
    pub fn new() -> Self {
        EtaStats::default()
    }
    /// Record a successful expansion.
    pub fn record_expansion(&mut self) {
        self.expansions += 1;
    }
    /// Record a successful reduction.
    pub fn record_reduction(&mut self) {
        self.reductions += 1;
    }
    /// Record a failed expansion.
    pub fn record_failed_expansion(&mut self) {
        self.failed_expansions += 1;
    }
    /// Record a failed reduction.
    pub fn record_failed_reduction(&mut self) {
        self.failed_reductions += 1;
    }
    /// Return expansion success rate.
    pub fn expansion_rate(&self) -> f64 {
        let total = self.expansions + self.failed_expansions;
        if total == 0 {
            1.0
        } else {
            self.expansions as f64 / total as f64
        }
    }
    /// Return reduction success rate.
    pub fn reduction_rate(&self) -> f64 {
        let total = self.reductions + self.failed_reductions;
        if total == 0 {
            1.0
        } else {
            self.reductions as f64 / total as f64
        }
    }
    /// Format a summary.
    pub fn summary(&self) -> String {
        format!(
            "expansions={} (fail={}) reductions={} (fail={})",
            self.expansions, self.failed_expansions, self.reductions, self.failed_reductions
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
enum EtaState {
    Idle,
    Expanding,
    Done,
    Failed,
}
/// A struct-eta unification hint: suggests which struct to eta-expand to unify.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EtaUnificationHint {
    pub lhs_id: u64,
    pub rhs_id: u64,
    pub suggested_struct: String,
}
#[allow(dead_code)]
impl EtaUnificationHint {
    /// Create a new hint.
    pub fn new(lhs_id: u64, rhs_id: u64, suggested_struct: impl Into<String>) -> Self {
        EtaUnificationHint {
            lhs_id,
            rhs_id,
            suggested_struct: suggested_struct.into(),
        }
    }
}
/// An eta-reduction opportunity: a constructor applied to its own projections.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EtaReduction {
    /// The inner expression (the projected-from value).
    pub inner_id: u64,
    /// The structure constructor name.
    pub ctor: String,
    /// Whether the reduction is valid (all projections match).
    pub is_valid: bool,
}
#[allow(dead_code)]
impl EtaReduction {
    /// Create a valid eta-reduction.
    pub fn valid(inner_id: u64, ctor: impl Into<String>) -> Self {
        EtaReduction {
            inner_id,
            ctor: ctor.into(),
            is_valid: true,
        }
    }
    /// Create an invalid (not reducible) result.
    pub fn invalid(ctor: impl Into<String>) -> Self {
        EtaReduction {
            inner_id: 0,
            ctor: ctor.into(),
            is_valid: false,
        }
    }
}
/// Summary for a completed eta-normalization run.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct EtaNormRunSummary {
    pub total_expressions: u64,
    pub eta_expansions: u64,
    pub eta_reductions: u64,
    pub k_reductions: u64,
    pub proj_rewrites: u64,
    pub unchanged: u64,
}
#[allow(dead_code)]
impl EtaNormRunSummary {
    /// Create zeroed summary.
    pub fn new() -> Self {
        EtaNormRunSummary::default()
    }
    /// Return the fraction of expressions changed.
    pub fn change_rate(&self) -> f64 {
        if self.total_expressions == 0 {
            return 0.0;
        }
        self.unchanged as f64 / self.total_expressions as f64
    }
    /// Total changes applied.
    pub fn total_changes(&self) -> u64 {
        self.eta_expansions + self.eta_reductions + self.k_reductions + self.proj_rewrites
    }
    /// Format a summary string.
    pub fn format(&self) -> String {
        format!(
            "total={} expansions={} reductions={} k_red={} proj_rew={} unchanged={}",
            self.total_expressions,
            self.eta_expansions,
            self.eta_reductions,
            self.k_reductions,
            self.proj_rewrites,
            self.unchanged
        )
    }
}
/// Configuration for an eta-normalization pass.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EtaPassConfig {
    pub do_expand: bool,
    pub do_reduce: bool,
    pub do_k_reduce: bool,
    pub do_proj_rewrite: bool,
    pub max_passes: u32,
    pub verbose: bool,
}
#[allow(dead_code)]
impl EtaPassConfig {
    /// Default configuration: all passes enabled, up to 10 iterations.
    pub fn default_config() -> Self {
        EtaPassConfig {
            do_expand: true,
            do_reduce: true,
            do_k_reduce: true,
            do_proj_rewrite: true,
            max_passes: 10,
            verbose: false,
        }
    }
    /// Minimal configuration: only projection rewriting.
    pub fn proj_only() -> Self {
        EtaPassConfig {
            do_expand: false,
            do_reduce: false,
            do_k_reduce: false,
            do_proj_rewrite: true,
            max_passes: 1,
            verbose: false,
        }
    }
    /// Return true if at least one pass is enabled.
    pub fn any_enabled(&self) -> bool {
        self.do_expand || self.do_reduce || self.do_k_reduce || self.do_proj_rewrite
    }
}
/// Checks whether an expression is in eta-normal form for a given structure.
#[allow(dead_code)]
pub struct EtaNormalFormChecker {
    registry: StructureRegistry,
}
#[allow(dead_code)]
impl EtaNormalFormChecker {
    /// Create a checker with the given registry.
    pub fn new(registry: StructureRegistry) -> Self {
        EtaNormalFormChecker { registry }
    }
    /// Return whether a structure name is known.
    pub fn knows_structure(&self, name: &str) -> bool {
        self.registry.find(name).is_some()
    }
    /// Return the expected field count for a structure.
    pub fn expected_arity(&self, structure_name: &str) -> Option<usize> {
        self.registry.find(structure_name).map(|s| s.fields.len())
    }
    /// Check if an eta-expanded form is valid for its structure.
    pub fn check_expansion(&self, exp: &EtaExpanded) -> bool {
        match self.expected_arity(&exp.ctor) {
            Some(arity) => exp.field_ids.len() == arity,
            None => false,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EtaChangeKind {
    Expanded,
    Reduced,
    KReduced,
    ProjRewritten,
}
/// A structural coherence check result.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoherenceResult {
    Coherent,
    Incoherent { reason: String },
    Unknown,
}
#[allow(dead_code)]
impl CoherenceResult {
    /// Create a coherent result.
    pub fn ok() -> Self {
        CoherenceResult::Coherent
    }
    /// Create an incoherent result with a reason.
    pub fn fail(reason: impl Into<String>) -> Self {
        CoherenceResult::Incoherent {
            reason: reason.into(),
        }
    }
    /// Return true if coherent.
    pub fn is_coherent(&self) -> bool {
        matches!(self, CoherenceResult::Coherent)
    }
}
/// A worklist-based eta-normalization pass.
#[allow(dead_code)]
pub struct EtaNormalizationPass {
    pub stats: EtaStats,
    worklist: Vec<u64>,
}
#[allow(dead_code)]
impl EtaNormalizationPass {
    /// Create a new normalization pass.
    pub fn new() -> Self {
        EtaNormalizationPass {
            stats: EtaStats::new(),
            worklist: Vec::new(),
        }
    }
    /// Push an expression id onto the worklist.
    pub fn schedule(&mut self, id: u64) {
        if !self.worklist.contains(&id) {
            self.worklist.push(id);
        }
    }
    /// Pop the next id to process.
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<u64> {
        if self.worklist.is_empty() {
            None
        } else {
            Some(self.worklist.remove(0))
        }
    }
    /// Return true if the worklist is exhausted.
    pub fn is_done(&self) -> bool {
        self.worklist.is_empty()
    }
    /// Return the number of pending items.
    pub fn pending(&self) -> usize {
        self.worklist.len()
    }
}
/// Maps structure names to their singleton-K reduction status.
#[allow(dead_code)]
pub struct KReductionTable {
    entries: Vec<(String, bool)>,
}
#[allow(dead_code)]
impl KReductionTable {
    /// Create an empty table.
    pub fn new() -> Self {
        KReductionTable {
            entries: Vec::new(),
        }
    }
    /// Mark a structure as K-reducible (or not).
    pub fn set(&mut self, name: &str, reducible: bool) {
        if let Some(e) = self.entries.iter_mut().find(|(n, _)| n == name) {
            e.1 = reducible;
        } else {
            self.entries.push((name.to_string(), reducible));
        }
    }
    /// Query K-reducibility of a structure.
    pub fn is_k_reducible(&self, name: &str) -> bool {
        self.entries
            .iter()
            .find(|(n, _)| n == name)
            .is_some_and(|(_, r)| *r)
    }
    /// Return all K-reducible structure names.
    pub fn k_reducible_names(&self) -> Vec<&str> {
        self.entries
            .iter()
            .filter(|(_, r)| *r)
            .map(|(n, _)| n.as_str())
            .collect()
    }
    /// Return the number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Return whether empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
/// A simple structural equivalence oracle based on shape matching.
#[allow(dead_code)]
pub struct ShapeEquivalence {
    registry: StructureRegistry,
}
#[allow(dead_code)]
impl ShapeEquivalence {
    /// Create an oracle with the given registry.
    pub fn new(registry: StructureRegistry) -> Self {
        ShapeEquivalence { registry }
    }
    /// Return whether two struct shapes can possibly be equal.
    pub fn may_be_equal(&self, a: &StructShape, b: &StructShape) -> bool {
        match (a, b) {
            (
                StructShape::Ctor {
                    name: n1,
                    arity: ar1,
                },
                StructShape::Ctor {
                    name: n2,
                    arity: ar2,
                },
            ) => n1 == n2 && ar1 == ar2,
            (StructShape::Proj { field_index: i }, StructShape::Proj { field_index: j }) => i == j,
            (StructShape::Other, StructShape::Other) => true,
            _ => false,
        }
    }
    /// Return whether a shape is eta-expandable (is a known structure ctor).
    pub fn is_expandable(&self, shape: &StructShape) -> bool {
        match shape {
            StructShape::Ctor { name, .. } => self.registry.find(name).is_some(),
            _ => false,
        }
    }
}
/// Accumulates record updates to be applied in a batch.
#[allow(dead_code)]
pub struct RecordUpdateBatch {
    updates: Vec<RecordUpdate>,
}
#[allow(dead_code)]
impl RecordUpdateBatch {
    /// Create an empty batch.
    pub fn new() -> Self {
        RecordUpdateBatch {
            updates: Vec::new(),
        }
    }
    /// Add an update to the batch.
    pub fn add(&mut self, update: RecordUpdate) {
        self.updates.push(update);
    }
    /// Return all updates.
    pub fn updates(&self) -> &[RecordUpdate] {
        &self.updates
    }
    /// Return the number of updates.
    pub fn len(&self) -> usize {
        self.updates.len()
    }
    /// Return whether the batch is empty.
    pub fn is_empty(&self) -> bool {
        self.updates.is_empty()
    }
    /// Clear all updates.
    pub fn clear(&mut self) {
        self.updates.clear();
    }
    /// Return updates that affect a specific expression.
    pub fn updates_for_expr(&self, expr_id: u64) -> Vec<&RecordUpdate> {
        self.updates
            .iter()
            .filter(|u| u.expr_id == expr_id)
            .collect()
    }
}
/// A simple equality oracle for eta-normal forms.
#[allow(dead_code)]
pub struct EtaEqualityOracle {
    canon_map: EtaCanonMap,
}
#[allow(dead_code)]
impl EtaEqualityOracle {
    /// Create an oracle with an existing canon map.
    pub fn new(canon_map: EtaCanonMap) -> Self {
        EtaEqualityOracle { canon_map }
    }
    /// Check if two expression ids are eta-equal (same canonical form).
    pub fn are_eta_equal(&self, a: u64, b: u64) -> bool {
        self.canon_map.canonical(a) == self.canon_map.canonical(b)
    }
    /// Return the canonical id for an expression.
    pub fn canonical(&self, id: u64) -> u64 {
        self.canon_map.canonical(id)
    }
    /// Return the number of canonical classes.
    pub fn class_count(&self) -> usize {
        let mut canons: Vec<u64> = self.canon_map.map.iter().map(|(_, c)| *c).collect();
        canons.sort_unstable();
        canons.dedup();
        canons.len()
    }
}
/// Validates that field access indices are in-bounds for a structure.
#[allow(dead_code)]
pub struct FieldBoundsChecker;
#[allow(dead_code)]
impl FieldBoundsChecker {
    /// Check that the field index is within bounds for the given arity.
    pub fn check(arity: u32, field_index: u32) -> Result<(), String> {
        if field_index < arity {
            Ok(())
        } else {
            Err(format!(
                "field index {} out of bounds for arity {}",
                field_index, arity
            ))
        }
    }
    /// Validate all projection rewrites in a set against a registry.
    pub fn validate_set(set: &ProjectionRewriteSet, reg: &StructureRegistry) -> Vec<String> {
        let mut errors = Vec::new();
        for rule in &set.rules {
            let arity = reg.field_count(&rule.ctor_name) as u32;
            if arity == 0 {
                errors.push(format!("unknown structure: {}", rule.ctor_name));
            } else if let Err(e) = Self::check(arity, rule.field_index) {
                errors.push(format!("rule '{}': {}", rule.projector_name, e));
            }
        }
        errors
    }
}
/// Checks eta-long status for a set of expressions.
#[allow(dead_code)]
pub struct EtaLongChecker {
    results: Vec<(u64, EtaLongStatus)>,
}
#[allow(dead_code)]
impl EtaLongChecker {
    /// Create a new checker.
    pub fn new() -> Self {
        EtaLongChecker {
            results: Vec::new(),
        }
    }
    /// Record a status for an expression id.
    pub fn record(&mut self, id: u64, status: EtaLongStatus) {
        self.results.push((id, status));
    }
    /// Look up the status for an expression id.
    pub fn status(&self, id: u64) -> Option<EtaLongStatus> {
        self.results.iter().find(|(i, _)| *i == id).map(|(_, s)| *s)
    }
    /// Return the count of expressions in each status.
    pub fn summary(&self) -> (usize, usize, usize) {
        let eta_long = self
            .results
            .iter()
            .filter(|(_, s)| *s == EtaLongStatus::EtaLong)
            .count();
        let not_eta_long = self
            .results
            .iter()
            .filter(|(_, s)| *s == EtaLongStatus::NotEtaLong)
            .count();
        let unknown = self
            .results
            .iter()
            .filter(|(_, s)| *s == EtaLongStatus::Unknown)
            .count();
        (eta_long, not_eta_long, unknown)
    }
    /// Return the total number of recorded results.
    pub fn len(&self) -> usize {
        self.results.len()
    }
    /// Return whether any results were recorded.
    pub fn is_empty(&self) -> bool {
        self.results.is_empty()
    }
}
/// A change log for an eta-normalization session.
#[allow(dead_code)]
pub struct EtaChangeLog {
    entries: Vec<EtaChangeEntry>,
}
#[allow(dead_code)]
impl EtaChangeLog {
    /// Create an empty log.
    pub fn new() -> Self {
        EtaChangeLog {
            entries: Vec::new(),
        }
    }
    /// Record a change.
    pub fn record(&mut self, expr_id: u64, kind: EtaChangeKind, pass_num: u32) {
        self.entries.push(EtaChangeEntry {
            expr_id,
            kind,
            pass_num,
        });
    }
    /// Return all changes of a specific kind.
    pub fn changes_of_kind(&self, kind: EtaChangeKind) -> Vec<&EtaChangeEntry> {
        self.entries.iter().filter(|e| e.kind == kind).collect()
    }
    /// Return all changes for a specific expression.
    pub fn changes_for_expr(&self, id: u64) -> Vec<&EtaChangeEntry> {
        self.entries.iter().filter(|e| e.expr_id == id).collect()
    }
    /// Return the total number of changes.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Return whether any changes were recorded.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Return all changes from a specific pass number.
    pub fn changes_in_pass(&self, pass: u32) -> Vec<&EtaChangeEntry> {
        self.entries.iter().filter(|e| e.pass_num == pass).collect()
    }
}
/// Registry of structure types and their fields.
#[allow(dead_code)]
pub struct StructureRegistry {
    structures: Vec<StructureDef>,
}
#[allow(dead_code)]
impl StructureRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        StructureRegistry {
            structures: Vec::new(),
        }
    }
    /// Register a new structure type.
    pub fn register(
        &mut self,
        name: impl Into<String>,
        ctor_name: impl Into<String>,
        fields: Vec<FieldDescriptor>,
    ) {
        self.structures.push(StructureDef {
            name: name.into(),
            ctor_name: ctor_name.into(),
            fields,
        });
    }
    /// Look up a structure by name.
    pub fn find(&self, name: &str) -> Option<&StructureDef> {
        self.structures.iter().find(|s| s.name == name)
    }
    /// Return the number of registered structures.
    pub fn len(&self) -> usize {
        self.structures.len()
    }
    /// Return whether the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.structures.is_empty()
    }
    /// Return all structure names.
    pub fn names(&self) -> Vec<&str> {
        self.structures.iter().map(|s| s.name.as_str()).collect()
    }
    /// Return the field count for a named structure, or 0 if unknown.
    pub fn field_count(&self, name: &str) -> usize {
        self.find(name).map_or(0, |s| s.fields.len())
    }
    /// Return whether a named structure has any Prop fields.
    pub fn has_prop_fields(&self, name: &str) -> bool {
        self.find(name)
            .is_some_and(|s| s.fields.iter().any(|f| f.is_prop))
    }
    /// Return projector names for a structure.
    pub fn projectors(&self, name: &str) -> Vec<String> {
        self.find(name).map_or(Vec::new(), |s| {
            s.fields
                .iter()
                .map(|f| format!("{}.{}", s.name, f.name))
                .collect()
        })
    }
}
/// Categorizes expressions by their structural type for eta purposes.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EtaCategory {
    Record,
    Function,
    Inductive,
    Proposition,
    Primitive,
}
#[allow(dead_code)]
impl EtaCategory {
    /// Return true if this category benefits from eta-expansion.
    pub fn needs_eta(&self) -> bool {
        matches!(self, EtaCategory::Record | EtaCategory::Function)
    }
    /// Return a static label.
    pub fn label(&self) -> &'static str {
        match self {
            EtaCategory::Record => "record",
            EtaCategory::Function => "function",
            EtaCategory::Inductive => "inductive",
            EtaCategory::Proposition => "proposition",
            EtaCategory::Primitive => "primitive",
        }
    }
}
/// Performs η-expansion for structure (record) types.
///
/// For a structure type `S` with constructor `S.mk` and fields `f₁, f₂, …, fₙ`,
/// η-expanding an expression `e : S` yields:
/// ```text
/// S.mk (e.f₁) (e.f₂) … (e.fₙ)
/// ```
/// which is definitionally equal to `e` by the η-rule for structures.
pub struct StructureEta<'a> {
    env: &'a Environment,
}
impl<'a> StructureEta<'a> {
    /// Create a new `StructureEta` helper bound to the given environment.
    pub fn new(env: &'a Environment) -> Self {
        StructureEta { env }
    }
    /// Return `true` if `ty` is a `Const` whose name resolves in the environment
    /// to a structure-like inductive (exactly 1 constructor, 0 indices, non-recursive).
    pub fn is_structure_type(&self, ty: &Expr) -> bool {
        let name = match ty {
            Expr::Const(n, _) => n,
            _ => {
                if let Some(n) = head_const(ty) {
                    return self.env.is_structure_like(n);
                }
                return false;
            }
        };
        self.env.is_structure_like(name)
    }
    /// Collect `(field_name, field_type)` pairs for the structure `struct_name`.
    ///
    /// This works by looking up the unique constructor in the environment and
    /// counting `num_fields` from its `ConstructorVal`.  The field *types* are
    /// obtained from the constructor's type by stripping the leading `Pi`-binders
    /// that correspond to the inductive parameters, then collecting the remaining
    /// domain types.
    ///
    /// Returns an empty `Vec` when the structure is not found.
    pub fn collect_field_types(&self, struct_name: &Name) -> Vec<(Name, Expr)> {
        let iv = match self.env.get_inductive_val(struct_name) {
            Some(v) => v,
            None => return vec![],
        };
        let ctor_name = match iv.ctors.first() {
            Some(n) => n.clone(),
            None => return vec![],
        };
        let cv = match self.env.get_constructor_val(&ctor_name) {
            Some(v) => v,
            None => return vec![],
        };
        let ctor_ty = match self.env.get_type(&ctor_name) {
            Some(t) => t.clone(),
            None => return vec![],
        };
        let num_params = cv.num_params as usize;
        let num_fields = cv.num_fields as usize;
        let mut current = &ctor_ty;
        let mut skipped = 0usize;
        let mut fields = Vec::with_capacity(num_fields);
        while let Expr::Pi(_, name, domain, codomain) = current {
            if skipped < num_params {
                skipped += 1;
                current = codomain;
            } else if fields.len() < num_fields {
                fields.push((name.clone(), *domain.clone()));
                current = codomain;
            } else {
                break;
            }
        }
        fields
    }
    /// Build a `Vec` of projection expressions `Proj(struct_name, i, expr)`
    /// for `i` in `0..num_fields`.
    pub fn make_proj_chain(&self, expr: &Expr, struct_name: &Name, num_fields: usize) -> Vec<Expr> {
        (0..num_fields)
            .map(|i| Expr::Proj(struct_name.clone(), i as u32, Box::new(expr.clone())))
            .collect()
    }
    /// η-expand `expr` at type `ty`.
    ///
    /// If `ty` is a structure type, returns
    /// `App(… App(S.mk, proj_0(expr)) …, proj_{n-1}(expr))`.
    /// Returns `None` when `ty` is not a known structure type.
    pub fn eta_expand_struct(&self, expr: &Expr, ty: &Expr) -> Option<Expr> {
        let struct_name = head_const(ty)?.clone();
        if !self.env.is_structure_like(&struct_name) {
            return None;
        }
        let iv = self.env.get_inductive_val(&struct_name)?;
        let ctor_name = iv.ctors.first()?.clone();
        let cv = self.env.get_constructor_val(&ctor_name)?;
        let num_fields = cv.num_fields as usize;
        let ctor_levels: Vec<Level> = vec![];
        let ctor_const = Expr::Const(ctor_name, ctor_levels);
        let projections = self.make_proj_chain(expr, &struct_name, num_fields);
        if projections.is_empty() {
            return Some(ctor_const);
        }
        let result = projections.into_iter().fold(ctor_const, |acc, proj| {
            Expr::App(Box::new(acc), Box::new(proj))
        });
        Some(result)
    }
}
/// Represents the shape of a structure expression for comparison.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructShape {
    /// A constructor applied to `n` arguments.
    Ctor { name: String, arity: u32 },
    /// A projection of field `i` from an expression.
    Proj { field_index: u32 },
    /// Any other expression shape.
    Other,
}
#[allow(dead_code)]
impl StructShape {
    /// Create a constructor shape.
    pub fn ctor(name: impl Into<String>, arity: u32) -> Self {
        StructShape::Ctor {
            name: name.into(),
            arity,
        }
    }
    /// Create a projection shape.
    pub fn proj(field_index: u32) -> Self {
        StructShape::Proj { field_index }
    }
    /// Return true if this is a constructor.
    pub fn is_ctor(&self) -> bool {
        matches!(self, StructShape::Ctor { .. })
    }
    /// Return true if this is a projection.
    pub fn is_proj(&self) -> bool {
        matches!(self, StructShape::Proj { .. })
    }
    /// Return arity if this is a constructor, else None.
    pub fn arity(&self) -> Option<u32> {
        match self {
            StructShape::Ctor { arity, .. } => Some(*arity),
            _ => None,
        }
    }
}
/// A simple eta rewrite engine that applies a set of rules iteratively.
#[allow(dead_code)]
pub struct EtaRewriteEngine {
    proj_rules: ProjectionRewriteSet,
    max_steps: u32,
    steps_taken: u32,
}
#[allow(dead_code)]
impl EtaRewriteEngine {
    /// Create a new engine with the given rule set.
    pub fn new(proj_rules: ProjectionRewriteSet, max_steps: u32) -> Self {
        EtaRewriteEngine {
            proj_rules,
            max_steps,
            steps_taken: 0,
        }
    }
    /// Return the number of steps taken so far.
    pub fn steps_taken(&self) -> u32 {
        self.steps_taken
    }
    /// Return whether the engine has reached its step limit.
    pub fn is_exhausted(&self) -> bool {
        self.steps_taken >= self.max_steps
    }
    /// Try to apply a projection rule for the given projector name.
    /// Returns `Some(field_index)` if a rule applies, else `None`.
    pub fn apply_proj(&mut self, projector: &str) -> Option<u32> {
        if self.is_exhausted() {
            return None;
        }
        if let Some(rule) = self.proj_rules.find_by_projector(projector) {
            let idx = rule.field_index;
            self.steps_taken += 1;
            Some(idx)
        } else {
            None
        }
    }
    /// Reset the step counter.
    pub fn reset(&mut self) {
        self.steps_taken = 0;
    }
    /// Return the number of projection rules available.
    pub fn rule_count(&self) -> usize {
        self.proj_rules.len()
    }
}
/// A structure projection descriptor mapping projector name to field index.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ProjectionDescriptor {
    pub structure_name: String,
    pub projector_name: String,
    pub field_index: u32,
}
#[allow(dead_code)]
impl ProjectionDescriptor {
    /// Create a new projection descriptor.
    pub fn new(
        structure_name: impl Into<String>,
        projector_name: impl Into<String>,
        field_index: u32,
    ) -> Self {
        ProjectionDescriptor {
            structure_name: structure_name.into(),
            projector_name: projector_name.into(),
            field_index,
        }
    }
}
/// A projection normalization rewrite: projector applied to constructor.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ProjectionRewrite {
    pub ctor_name: String,
    pub projector_name: String,
    pub field_index: u32,
}
#[allow(dead_code)]
impl ProjectionRewrite {
    /// Create a projection rewrite.
    pub fn new(
        ctor_name: impl Into<String>,
        projector_name: impl Into<String>,
        field_index: u32,
    ) -> Self {
        ProjectionRewrite {
            ctor_name: ctor_name.into(),
            projector_name: projector_name.into(),
            field_index,
        }
    }
    /// Format as a rewrite rule string.
    pub fn as_rule(&self) -> String {
        format!(
            "{} ({}.mk f0 ... f{} ...) → f{}",
            self.projector_name, self.ctor_name, self.field_index, self.field_index
        )
    }
}
/// Batch eta-categorization results.
#[allow(dead_code)]
pub struct EtaCategorizer {
    entries: Vec<(u64, EtaCategory)>,
}
#[allow(dead_code)]
impl EtaCategorizer {
    /// Create a new categorizer.
    pub fn new() -> Self {
        EtaCategorizer {
            entries: Vec::new(),
        }
    }
    /// Assign a category to an expression id.
    pub fn assign(&mut self, id: u64, cat: EtaCategory) {
        self.entries.push((id, cat));
    }
    /// Look up category for an id.
    pub fn get(&self, id: u64) -> Option<EtaCategory> {
        self.entries.iter().find(|(i, _)| *i == id).map(|(_, c)| *c)
    }
    /// Return ids that need eta-expansion.
    pub fn needs_eta_ids(&self) -> Vec<u64> {
        self.entries
            .iter()
            .filter(|(_, c)| c.needs_eta())
            .map(|(i, _)| *i)
            .collect()
    }
    /// Return the number of categorized expressions.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Return whether any expressions are categorized.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Count expressions in each category.
    pub fn count_by_category(&self) -> [(EtaCategory, usize); 5] {
        let cats = [
            EtaCategory::Record,
            EtaCategory::Function,
            EtaCategory::Inductive,
            EtaCategory::Proposition,
            EtaCategory::Primitive,
        ];
        cats.map(|c| {
            let count = self.entries.iter().filter(|(_, cat)| *cat == c).count();
            (c, count)
        })
    }
}
/// Eta canonicalization: maps expression ids to their canonical eta-normal form.
#[allow(dead_code)]
pub struct EtaCanonMap {
    map: Vec<(u64, u64)>,
}
#[allow(dead_code)]
impl EtaCanonMap {
    /// Create an empty canon map.
    pub fn new() -> Self {
        EtaCanonMap { map: Vec::new() }
    }
    /// Record that `original` canonicalizes to `canon`.
    pub fn insert(&mut self, original: u64, canon: u64) {
        if let Some(e) = self.map.iter_mut().find(|(o, _)| *o == original) {
            e.1 = canon;
        } else {
            self.map.push((original, canon));
        }
    }
    /// Return the canonical form of `original`, or itself if not mapped.
    pub fn canonical(&self, original: u64) -> u64 {
        self.map
            .iter()
            .find(|(o, _)| *o == original)
            .map_or(original, |(_, c)| *c)
    }
    /// Return the number of mappings.
    pub fn len(&self) -> usize {
        self.map.len()
    }
    /// Return whether the map is empty.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
    /// Return all original ids that map to `canon`.
    pub fn originals_of(&self, canon: u64) -> Vec<u64> {
        self.map
            .iter()
            .filter(|(_, c)| *c == canon)
            .map(|(o, _)| *o)
            .collect()
    }
}
/// A dependency graph tracking which expressions depend on eta-normal forms.
#[allow(dead_code)]
pub struct EtaGraph {
    edges: Vec<(u64, u64)>,
}
#[allow(dead_code)]
impl EtaGraph {
    /// Create an empty graph.
    pub fn new() -> Self {
        EtaGraph { edges: Vec::new() }
    }
    /// Add a dependency edge: `from` depends on `to`.
    pub fn add_edge(&mut self, from: u64, to: u64) {
        if !self.has_edge(from, to) {
            self.edges.push((from, to));
        }
    }
    /// Return whether an edge exists.
    pub fn has_edge(&self, from: u64, to: u64) -> bool {
        self.edges.iter().any(|&(f, t)| f == from && t == to)
    }
    /// Return all expressions that depend on `id`.
    pub fn dependents_of(&self, id: u64) -> Vec<u64> {
        self.edges
            .iter()
            .filter(|(_, t)| *t == id)
            .map(|(f, _)| *f)
            .collect()
    }
    /// Return all expressions that `id` depends on.
    pub fn dependencies_of(&self, id: u64) -> Vec<u64> {
        self.edges
            .iter()
            .filter(|(f, _)| *f == id)
            .map(|(_, t)| *t)
            .collect()
    }
    /// Return the total edge count.
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
    /// Return whether the graph has no edges.
    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }
    /// Remove all edges involving `id`.
    pub fn remove_node(&mut self, id: u64) {
        self.edges.retain(|(f, t)| *f != id && *t != id);
    }
}
/// An occurrence of an eta-redex within a larger expression.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EtaRedex {
    /// Path (sequence of child indices) to the redex root.
    pub path: Vec<u32>,
    /// The structure name of the redex.
    pub struct_name: String,
    /// The expression id of the inner term.
    pub inner_id: u64,
}
#[allow(dead_code)]
impl EtaRedex {
    /// Create a new eta-redex at the given path.
    pub fn new(path: Vec<u32>, struct_name: impl Into<String>, inner_id: u64) -> Self {
        EtaRedex {
            path,
            struct_name: struct_name.into(),
            inner_id,
        }
    }
    /// Return true if this redex is at the top level (empty path).
    pub fn is_top_level(&self) -> bool {
        self.path.is_empty()
    }
    /// Return the nesting depth of this redex.
    pub fn depth(&self) -> usize {
        self.path.len()
    }
}
/// Tracks depth of eta-expansion nesting.
#[allow(dead_code)]
pub struct EtaDepthTracker {
    stack: Vec<String>,
}
#[allow(dead_code)]
impl EtaDepthTracker {
    /// Create an empty tracker.
    pub fn new() -> Self {
        EtaDepthTracker { stack: Vec::new() }
    }
    /// Push a structure context onto the stack.
    pub fn push(&mut self, struct_name: &str) {
        self.stack.push(struct_name.to_string());
    }
    /// Pop the current structure context.
    pub fn pop(&mut self) -> Option<String> {
        self.stack.pop()
    }
    /// Return the current expansion depth.
    pub fn depth(&self) -> usize {
        self.stack.len()
    }
    /// Return whether we are currently inside any expansion.
    pub fn is_nested(&self) -> bool {
        !self.stack.is_empty()
    }
    /// Return the innermost structure name.
    pub fn current(&self) -> Option<&str> {
        self.stack.last().map(|s| s.as_str())
    }
    /// Return the full nesting path as a dot-separated string.
    pub fn path(&self) -> String {
        self.stack.join(".")
    }
    /// Return whether a given structure appears anywhere in the nesting.
    pub fn contains(&self, name: &str) -> bool {
        self.stack.iter().any(|s| s == name)
    }
}
/// A record update: replace one field of a structure expression.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RecordUpdate {
    pub expr_id: u64,
    pub struct_name: String,
    pub field_index: u32,
    pub new_value_id: u64,
}
#[allow(dead_code)]
impl RecordUpdate {
    /// Create a record update.
    pub fn new(
        expr_id: u64,
        struct_name: impl Into<String>,
        field_index: u32,
        new_value_id: u64,
    ) -> Self {
        RecordUpdate {
            expr_id,
            struct_name: struct_name.into(),
            field_index,
            new_value_id,
        }
    }
    /// Format a description of this update.
    pub fn describe(&self) -> String {
        format!(
            "update {}.{} of expr #{} with expr #{}",
            self.struct_name, self.field_index, self.expr_id, self.new_value_id
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EtaChangeEntry {
    pub expr_id: u64,
    pub kind: EtaChangeKind,
    pub pass_num: u32,
}
/// Eta-expansion state machine for iterative expansion.
#[allow(dead_code)]
pub struct EtaStateMachine {
    state: EtaState,
    structure_name: Option<String>,
    field_count: u32,
    processed_fields: u32,
}
#[allow(dead_code)]
impl EtaStateMachine {
    /// Create a machine in the Idle state.
    pub fn new() -> Self {
        EtaStateMachine {
            state: EtaState::Idle,
            structure_name: None,
            field_count: 0,
            processed_fields: 0,
        }
    }
    /// Start expansion for a given structure and field count.
    pub fn start(&mut self, name: &str, field_count: u32) {
        self.state = EtaState::Expanding;
        self.structure_name = Some(name.to_string());
        self.field_count = field_count;
        self.processed_fields = 0;
    }
    /// Record one field being processed.
    pub fn process_field(&mut self) -> bool {
        if self.state != EtaState::Expanding {
            return false;
        }
        self.processed_fields += 1;
        if self.processed_fields >= self.field_count {
            self.state = EtaState::Done;
        }
        true
    }
    /// Mark the expansion as failed.
    pub fn fail(&mut self) {
        self.state = EtaState::Failed;
    }
    /// Return whether expansion is complete.
    pub fn is_done(&self) -> bool {
        self.state == EtaState::Done
    }
    /// Return whether expansion failed.
    pub fn is_failed(&self) -> bool {
        self.state == EtaState::Failed
    }
    /// Return whether currently expanding.
    pub fn is_expanding(&self) -> bool {
        self.state == EtaState::Expanding
    }
    /// Return remaining fields to process.
    pub fn remaining(&self) -> u32 {
        self.field_count.saturating_sub(self.processed_fields)
    }
}
/// Represents whether an expression is in eta-long normal form.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EtaLongStatus {
    /// Expression is already in eta-long normal form.
    EtaLong,
    /// Expression is not eta-long; needs expansion.
    NotEtaLong,
    /// Cannot determine without full type information.
    Unknown,
}
#[allow(dead_code)]
impl EtaLongStatus {
    /// Return true if this is the EtaLong variant.
    pub fn is_eta_long(&self) -> bool {
        *self == EtaLongStatus::EtaLong
    }
}
/// Final result of running the eta normalization pass.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EtaPassResult {
    pub summary: EtaNormRunSummary,
    pub changed_ids: Vec<u64>,
}
#[allow(dead_code)]
impl EtaPassResult {
    /// Create an empty result.
    pub fn new() -> Self {
        EtaPassResult {
            summary: EtaNormRunSummary::new(),
            changed_ids: Vec::new(),
        }
    }
    /// Return whether any expressions were changed.
    pub fn any_changes(&self) -> bool {
        !self.changed_ids.is_empty()
    }
}
/// A pass that flattens nested structure constructor applications.
#[allow(dead_code)]
pub struct StructFlatteningPass {
    pub processed: u64,
    pub flattened: u64,
}
#[allow(dead_code)]
impl StructFlatteningPass {
    /// Create a new pass.
    pub fn new() -> Self {
        StructFlatteningPass {
            processed: 0,
            flattened: 0,
        }
    }
    /// Record that an expression was processed.
    pub fn record_processed(&mut self) {
        self.processed += 1;
    }
    /// Record that a flattening was performed.
    pub fn record_flattened(&mut self) {
        self.flattened += 1;
    }
    /// Return the fraction of processed expressions that were flattened.
    pub fn flatten_rate(&self) -> f64 {
        if self.processed == 0 {
            0.0
        } else {
            self.flattened as f64 / self.processed as f64
        }
    }
}
/// A record field descriptor for eta-expansion analysis.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FieldDescriptor {
    pub name: String,
    pub index: u32,
    pub is_prop: bool,
}
#[allow(dead_code)]
impl FieldDescriptor {
    /// Create a new field descriptor.
    pub fn new(name: impl Into<String>, index: u32, is_prop: bool) -> Self {
        FieldDescriptor {
            name: name.into(),
            index,
            is_prop,
        }
    }
    /// Return whether this field is a computation field (not Prop).
    pub fn is_data(&self) -> bool {
        !self.is_prop
    }
}
/// Structural injectivity analysis: can a constructor be distinguished by its arguments?
#[allow(dead_code)]
pub struct InjectivityChecker {
    injective: Vec<String>,
}
#[allow(dead_code)]
impl InjectivityChecker {
    /// Create an empty checker.
    pub fn new() -> Self {
        InjectivityChecker {
            injective: Vec::new(),
        }
    }
    /// Mark a constructor as injective.
    pub fn mark_injective(&mut self, ctor: &str) {
        if !self.injective.contains(&ctor.to_string()) {
            self.injective.push(ctor.to_string());
        }
    }
    /// Return whether a constructor is known to be injective.
    pub fn is_injective(&self, ctor: &str) -> bool {
        self.injective.iter().any(|s| s == ctor)
    }
    /// Return all injective constructors.
    pub fn injective_ctors(&self) -> &[String] {
        &self.injective
    }
    /// Return the number of known injective constructors.
    pub fn count(&self) -> usize {
        self.injective.len()
    }
}
