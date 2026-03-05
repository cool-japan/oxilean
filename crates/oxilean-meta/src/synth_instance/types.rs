//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::basic::{MVarId, MetaContext};
use crate::def_eq::{MetaDefEq, UnificationResult};
use crate::discr_tree::DiscrTree;
use oxilean_kernel::{BinderInfo, Expr, Level, Name};
use std::collections::{HashMap, HashSet};

use super::functions::{
    collect_unassigned_mvars, extract_class_name, goals_structurally_similar, InstancePriority,
    DEFAULT_PRIORITY,
};

#[allow(dead_code)]
pub struct SynthInstanceExtDiff1500 {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
impl SynthInstanceExtDiff1500 {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            added: Vec::new(),
            removed: Vec::new(),
            unchanged: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn add(&mut self, s: &str) {
        self.added.push(s.to_string());
    }
    #[allow(dead_code)]
    pub fn remove(&mut self, s: &str) {
        self.removed.push(s.to_string());
    }
    #[allow(dead_code)]
    pub fn keep(&mut self, s: &str) {
        self.unchanged.push(s.to_string());
    }
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty()
    }
    #[allow(dead_code)]
    pub fn total_changes(&self) -> usize {
        self.added.len() + self.removed.len()
    }
    #[allow(dead_code)]
    pub fn net_additions(&self) -> i64 {
        self.added.len() as i64 - self.removed.len() as i64
    }
    #[allow(dead_code)]
    pub fn summary(&self) -> String {
        format!(
            "+{} -{} =={}",
            self.added.len(),
            self.removed.len(),
            self.unchanged.len()
        )
    }
}
/// A typed slot for SynthInstance configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum SynthInstanceConfigValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
#[allow(dead_code)]
impl SynthInstanceConfigValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            SynthInstanceConfigValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            SynthInstanceConfigValue::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            SynthInstanceConfigValue::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            SynthInstanceConfigValue::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&[String]> {
        match self {
            SynthInstanceConfigValue::List(v) => Some(v),
            _ => None,
        }
    }
    pub fn type_name(&self) -> &'static str {
        match self {
            SynthInstanceConfigValue::Bool(_) => "bool",
            SynthInstanceConfigValue::Int(_) => "int",
            SynthInstanceConfigValue::Float(_) => "float",
            SynthInstanceConfigValue::Str(_) => "str",
            SynthInstanceConfigValue::List(_) => "list",
        }
    }
}
/// Choice point for backtracking.
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub(crate) struct ChoicePoint {
    /// Goal at this choice point.
    pub(super) goal: Expr,
    /// Depth at this choice point.
    pub(super) depth: u32,
    /// Remaining candidates to try.
    pub(super) candidates_remaining: usize,
}
/// A pipeline of SynthInstance analysis passes.
#[allow(dead_code)]
pub struct SynthInstancePipeline {
    pub passes: Vec<SynthInstanceAnalysisPass>,
    pub name: String,
    pub total_inputs_processed: usize,
}
#[allow(dead_code)]
impl SynthInstancePipeline {
    pub fn new(name: &str) -> Self {
        SynthInstancePipeline {
            passes: Vec::new(),
            name: name.to_string(),
            total_inputs_processed: 0,
        }
    }
    pub fn add_pass(&mut self, pass: SynthInstanceAnalysisPass) {
        self.passes.push(pass);
    }
    pub fn run_all(&mut self, input: &str) -> Vec<SynthInstanceResult> {
        self.total_inputs_processed += 1;
        self.passes
            .iter_mut()
            .filter(|p| p.enabled)
            .map(|p| p.run(input))
            .collect()
    }
    pub fn num_passes(&self) -> usize {
        self.passes.len()
    }
    pub fn num_enabled_passes(&self) -> usize {
        self.passes.iter().filter(|p| p.enabled).count()
    }
    pub fn total_success_rate(&self) -> f64 {
        if self.passes.is_empty() {
            0.0
        } else {
            let total_rate: f64 = self.passes.iter().map(|p| p.success_rate()).sum();
            total_rate / self.passes.len() as f64
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum SynthInstanceExtConfigVal1500 {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
impl SynthInstanceExtConfigVal1500 {
    #[allow(dead_code)]
    pub fn as_bool(&self) -> Option<bool> {
        if let SynthInstanceExtConfigVal1500::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_int(&self) -> Option<i64> {
        if let SynthInstanceExtConfigVal1500::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_float(&self) -> Option<f64> {
        if let SynthInstanceExtConfigVal1500::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_str(&self) -> Option<&str> {
        if let SynthInstanceExtConfigVal1500::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_list(&self) -> Option<&[String]> {
        if let SynthInstanceExtConfigVal1500::List(l) = self {
            Some(l)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn type_name(&self) -> &'static str {
        match self {
            SynthInstanceExtConfigVal1500::Bool(_) => "bool",
            SynthInstanceExtConfigVal1500::Int(_) => "int",
            SynthInstanceExtConfigVal1500::Float(_) => "float",
            SynthInstanceExtConfigVal1500::Str(_) => "str",
            SynthInstanceExtConfigVal1500::List(_) => "list",
        }
    }
}
/// A registered type class instance.
#[derive(Clone, Debug)]
pub struct InstanceEntry {
    /// Name of the instance declaration.
    pub name: Name,
    /// Type of the instance (the class application it satisfies).
    pub ty: Expr,
    /// Priority (lower = tried first).
    pub priority: InstancePriority,
    /// Whether this is a local instance (from the context).
    pub is_local: bool,
    /// Whether this instance is preferred (used for tie-breaking).
    pub preferred: bool,
}
impl InstanceEntry {
    /// Create a new instance entry with defaults.
    pub fn new(name: Name, ty: Expr) -> Self {
        Self {
            name,
            ty,
            priority: DEFAULT_PRIORITY,
            is_local: false,
            preferred: false,
        }
    }
    /// Set the priority of this instance.
    pub fn with_priority(mut self, priority: InstancePriority) -> Self {
        self.priority = priority;
        self
    }
    /// Mark this instance as local.
    pub fn with_local(mut self, is_local: bool) -> Self {
        self.is_local = is_local;
        self
    }
    /// Mark this instance as preferred.
    pub fn with_preferred(mut self, preferred: bool) -> Self {
        self.preferred = preferred;
        self
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum SynthInstanceExtResult1500 {
    /// Operation completed successfully.
    Ok(String),
    /// Operation encountered an error.
    Err(String),
    /// Operation partially completed.
    Partial { done: usize, total: usize },
    /// Operation was skipped.
    Skipped,
}
impl SynthInstanceExtResult1500 {
    #[allow(dead_code)]
    pub fn is_ok(&self) -> bool {
        matches!(self, SynthInstanceExtResult1500::Ok(_))
    }
    #[allow(dead_code)]
    pub fn is_err(&self) -> bool {
        matches!(self, SynthInstanceExtResult1500::Err(_))
    }
    #[allow(dead_code)]
    pub fn is_partial(&self) -> bool {
        matches!(self, SynthInstanceExtResult1500::Partial { .. })
    }
    #[allow(dead_code)]
    pub fn is_skipped(&self) -> bool {
        matches!(self, SynthInstanceExtResult1500::Skipped)
    }
    #[allow(dead_code)]
    pub fn ok_msg(&self) -> Option<&str> {
        if let SynthInstanceExtResult1500::Ok(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn err_msg(&self) -> Option<&str> {
        if let SynthInstanceExtResult1500::Err(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn progress(&self) -> f64 {
        match self {
            SynthInstanceExtResult1500::Ok(_) => 1.0,
            SynthInstanceExtResult1500::Err(_) => 0.0,
            SynthInstanceExtResult1500::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
            SynthInstanceExtResult1500::Skipped => 0.5,
        }
    }
}
/// Type class instance synthesizer with advanced resolution.
///
/// Uses tabled resolution with generator/consumer nodes
/// for efficient instance search and sophisticated search strategies.
pub struct InstanceSynthesizer {
    /// Global instances indexed by discrimination tree.
    pub(super) global_instances: DiscrTree<InstanceEntry>,
    /// Instance entries by class name.
    pub(super) instances_by_class: HashMap<Name, Vec<InstanceEntry>>,
    /// Cache of successful synthesis results.
    pub(super) cache: HashMap<Expr, Expr>,
    /// Cache of failed synthesis attempts.
    pub(super) failure_cache: HashMap<Expr, FailureReason>,
    /// Maximum search depth.
    pub(super) max_depth: u32,
    /// Maximum number of heartbeats (work units).
    pub(super) max_heartbeats: u64,
    /// Current heartbeat count.
    pub(super) heartbeats: u64,
    /// Trail of goals being explored (for loop detection).
    pub(super) trail: Vec<Expr>,
    /// Resolution nodes for tabled resolution.
    pub(super) resolution_nodes: Vec<ResolutionNode>,
    /// Choice points for backtracking.
    pub(super) choice_points: Vec<ChoicePoint>,
    /// Statistics for current synthesis.
    pub(super) current_stats: SearchStats,
    /// Diagnostics for last synthesis.
    pub(super) last_diagnostics: Option<SynthDiagnostics>,
}
impl InstanceSynthesizer {
    /// Create a new instance synthesizer.
    pub fn new() -> Self {
        Self {
            global_instances: DiscrTree::new(),
            instances_by_class: HashMap::new(),
            cache: HashMap::new(),
            failure_cache: HashMap::new(),
            max_depth: 32,
            max_heartbeats: 20_000,
            heartbeats: 0,
            trail: Vec::new(),
            resolution_nodes: Vec::new(),
            choice_points: Vec::new(),
            current_stats: SearchStats::default(),
            last_diagnostics: None,
        }
    }
    /// Register a global instance.
    pub fn add_instance(&mut self, entry: InstanceEntry) {
        let class_name = extract_class_name(&entry.ty);
        self.global_instances.insert(&entry.ty, entry.clone());
        self.instances_by_class
            .entry(class_name)
            .or_default()
            .push(entry);
    }
    /// Add a local instance from the current context.
    pub fn add_local_instance(&mut self, entry: InstanceEntry) {
        let mut local_entry = entry;
        local_entry.is_local = true;
        self.add_instance(local_entry);
    }
    /// Get all instances for a class.
    pub fn get_instances(&self, class_name: &Name) -> &[InstanceEntry] {
        self.instances_by_class
            .get(class_name)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }
    /// Get the number of registered instances.
    pub fn num_instances(&self) -> usize {
        self.global_instances.num_entries()
    }
    /// Check for instance overlap (coherence check).
    ///
    /// Returns true if there are instances with the same head that
    /// could potentially match the same goal.
    pub fn check_overlap(&self, class_name: &Name) -> bool {
        let instances = self.get_instances(class_name);
        if instances.len() < 2 {
            return false;
        }
        let mut seen = HashSet::new();
        for inst in instances {
            let key = (inst.priority, inst.preferred);
            if seen.contains(&key) {
                return true;
            }
            seen.insert(key);
        }
        false
    }
    /// Clear all cached results.
    pub fn clear_cache(&mut self) {
        self.cache.clear();
        self.failure_cache.clear();
    }
    /// Synthesize an instance for the given type class goal.
    ///
    /// The `goal` should be a type class application, e.g., `Add Nat`.
    pub fn synthesize(&mut self, goal: &Expr, ctx: &mut MetaContext) -> SynthResult {
        self.heartbeats = 0;
        self.current_stats = SearchStats::default();
        self.trail.clear();
        self.resolution_nodes.clear();
        self.choice_points.clear();
        let goal_inst = ctx.instantiate_mvars(goal);
        if let Some(cached) = self.cache.get(&goal_inst) {
            self.current_stats.cache_hits = 1;
            return SynthResult::Success(cached.clone());
        }
        if let Some(reason) = self.failure_cache.get(&goal_inst) {
            let diag = SynthDiagnostics {
                failure_reason: reason.clone(),
                stats: self.current_stats.clone(),
                tried_candidates: vec![],
            };
            self.last_diagnostics = Some(diag);
            return SynthResult::Failure;
        }
        let result = self.synthesize_internal(&goal_inst, ctx);
        match &result {
            SynthResult::Success(expr) => {
                self.cache.insert(goal_inst.clone(), expr.clone());
            }
            SynthResult::Failure => {
                self.failure_cache
                    .insert(goal_inst, FailureReason::NoInstances);
            }
            _ => {}
        }
        result
    }
    /// Internal synthesis implementation with backtracking.
    fn synthesize_internal(&mut self, goal: &Expr, ctx: &mut MetaContext) -> SynthResult {
        self.trail.push(goal.clone());
        let class_name = extract_class_name(goal);
        let candidates = self.get_sorted_candidates(&class_name, goal);
        if candidates.is_empty() {
            self.trail.pop();
            let diag = SynthDiagnostics {
                failure_reason: FailureReason::NoInstances,
                stats: self.current_stats.clone(),
                tried_candidates: vec![],
            };
            self.last_diagnostics = Some(diag);
            return SynthResult::Failure;
        }
        let mut tried = vec![];
        for candidate in &candidates {
            self.heartbeats += 1;
            self.current_stats.instances_examined += 1;
            if self.heartbeats > self.max_heartbeats {
                self.trail.pop();
                let diag = SynthDiagnostics {
                    failure_reason: FailureReason::Timeout,
                    stats: self.current_stats.clone(),
                    tried_candidates: tried,
                };
                self.last_diagnostics = Some(diag);
                return SynthResult::Failure;
            }
            let state = ctx.save_state();
            match self.try_candidate(candidate, goal, ctx, 0) {
                SynthResult::Success(result) => {
                    self.trail.pop();
                    self.current_stats.successful_unifications += 1;
                    return SynthResult::Success(result);
                }
                SynthResult::Stuck(id) => {
                    ctx.restore_state(state);
                    self.trail.pop();
                    return SynthResult::Stuck(id);
                }
                SynthResult::Failure => {
                    ctx.restore_state(state);
                    self.current_stats.failed_unifications += 1;
                    tried.push((candidate.name.clone(), FailureReason::UnificationFailed));
                    continue;
                }
            }
        }
        self.trail.pop();
        let diag = SynthDiagnostics {
            failure_reason: FailureReason::UnificationFailed,
            stats: self.current_stats.clone(),
            tried_candidates: tried,
        };
        self.last_diagnostics = Some(diag);
        SynthResult::Failure
    }
    /// Get candidates sorted by priority with tie-breaking.
    pub(crate) fn get_sorted_candidates(
        &self,
        class_name: &Name,
        _goal: &Expr,
    ) -> Vec<InstanceEntry> {
        let mut candidates: Vec<InstanceEntry> = self.get_instances(class_name).to_vec();
        candidates.sort_by(|a, b| match a.priority.cmp(&b.priority) {
            std::cmp::Ordering::Equal => match b.preferred.cmp(&a.preferred) {
                std::cmp::Ordering::Equal => b.is_local.cmp(&a.is_local),
                other => other,
            },
            other => other,
        });
        candidates
    }
    /// Try a single candidate instance with unification.
    fn try_candidate(
        &mut self,
        candidate: &InstanceEntry,
        goal: &Expr,
        ctx: &mut MetaContext,
        depth: u32,
    ) -> SynthResult {
        if depth > self.max_depth {
            self.current_stats.max_depth_reached = depth;
            return SynthResult::Failure;
        }
        if self
            .trail
            .iter()
            .take(self.trail.len() - 1)
            .any(|g| goals_structurally_similar(g, goal))
        {
            return SynthResult::Failure;
        }
        let (instance_expr, instance_ty) = self.instantiate_instance(candidate, ctx);
        let mut deq = MetaDefEq::new();
        let result = deq.is_def_eq(&instance_ty, goal, ctx);
        match result {
            UnificationResult::Equal => {
                self.current_stats.recursive_calls += 1;
                let inst_expr = ctx.instantiate_mvars(&instance_expr);
                if ctx.has_unassigned_mvars(&inst_expr) {
                    self.solve_subgoals(&inst_expr, ctx, depth + 1)
                } else {
                    SynthResult::Success(inst_expr)
                }
            }
            UnificationResult::Postponed => {
                if let Some(constraints) = ctx.postponed_constraints().last() {
                    if let Some(id) = MetaContext::is_mvar_expr(&constraints.lhs) {
                        return SynthResult::Stuck(id);
                    }
                }
                SynthResult::Failure
            }
            UnificationResult::NotEqual => SynthResult::Failure,
        }
    }
    /// Create a fresh instance of an instance entry with metavariables
    /// for all its parameters (higher-order unification).
    fn instantiate_instance(
        &self,
        candidate: &InstanceEntry,
        ctx: &mut MetaContext,
    ) -> (Expr, Expr) {
        let (level_params, raw_ty) = match ctx.find_const(&candidate.name) {
            Some(ci) => (ci.level_params().to_vec(), ci.ty().clone()),
            None => {
                return (
                    Expr::Const(candidate.name.clone(), vec![]),
                    candidate.ty.clone(),
                );
            }
        };
        let level_mvars: Vec<Level> = level_params
            .iter()
            .map(|_| ctx.mk_fresh_level_mvar())
            .collect();
        let inst_ty =
            oxilean_kernel::instantiate_level_params(&raw_ty, &level_params, &level_mvars);
        let mut instance_expr = Expr::Const(candidate.name.clone(), level_mvars);
        let mut cur_ty = inst_ty;
        while let Expr::Pi(
            BinderInfo::Implicit | BinderInfo::StrictImplicit | BinderInfo::InstImplicit,
            _,
            domain,
            body,
        ) = &cur_ty.clone()
        {
            let (_, mvar) =
                ctx.mk_fresh_expr_mvar((**domain).clone(), crate::basic::MetavarKind::Natural);
            instance_expr = Expr::App(Box::new(instance_expr), Box::new(mvar.clone()));
            cur_ty = oxilean_kernel::instantiate(body, &mvar);
        }
        (instance_expr, cur_ty)
    }
    /// Try to solve remaining subgoals in an instance expression (recursive instance search).
    fn solve_subgoals(&mut self, expr: &Expr, ctx: &mut MetaContext, depth: u32) -> SynthResult {
        if depth > self.max_depth {
            self.current_stats.max_depth_reached = depth;
            return SynthResult::Failure;
        }
        let unassigned = collect_unassigned_mvars(expr, ctx);
        for mvar_id in unassigned {
            if let Some(mvar_ty) = ctx.get_mvar_type(mvar_id) {
                let goal = mvar_ty.clone();
                let goal_inst = ctx.instantiate_mvars(&goal);
                let class_name = extract_class_name(&goal_inst);
                if !self.instances_by_class.contains_key(&class_name) {
                    continue;
                }
                if self.trail.contains(&goal_inst) {
                    return SynthResult::Failure;
                }
                self.trail.push(goal_inst.clone());
                match self.synthesize(&goal_inst, ctx) {
                    SynthResult::Success(result) => {
                        ctx.reassign_mvar(mvar_id, result);
                    }
                    SynthResult::Stuck(id) => {
                        self.trail.pop();
                        return SynthResult::Stuck(id);
                    }
                    SynthResult::Failure => {
                        self.trail.pop();
                        return SynthResult::Failure;
                    }
                }
                self.trail.pop();
            }
        }
        let final_expr = ctx.instantiate_mvars(expr);
        if ctx.has_unassigned_mvars(&final_expr) {
            SynthResult::Failure
        } else {
            SynthResult::Success(final_expr)
        }
    }
    /// Set the maximum search depth.
    pub fn set_max_depth(&mut self, depth: u32) {
        self.max_depth = depth;
    }
    /// Set the maximum number of heartbeats.
    pub fn set_max_heartbeats(&mut self, heartbeats: u64) {
        self.max_heartbeats = heartbeats;
    }
    /// Get the number of heartbeats used in the last synthesis.
    pub fn last_heartbeats(&self) -> u64 {
        self.heartbeats
    }
    /// Get diagnostics from the last synthesis attempt.
    pub fn last_diagnostics(&self) -> Option<&SynthDiagnostics> {
        self.last_diagnostics.as_ref()
    }
    /// Get current search statistics.
    pub fn current_stats(&self) -> &SearchStats {
        &self.current_stats
    }
    /// Rank candidate instances for a goal (for diagnostics).
    pub fn rank_candidates(&self, goal: &Expr) -> Vec<(Name, u32)> {
        let class_name = extract_class_name(goal);
        let candidates = self.get_sorted_candidates(&class_name, goal);
        candidates
            .iter()
            .enumerate()
            .map(|(rank, inst)| (inst.name.clone(), rank as u32))
            .collect()
    }
}
/// A diff for SynthInstance analysis results.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SynthInstanceDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
#[allow(dead_code)]
impl SynthInstanceDiff {
    pub fn new() -> Self {
        SynthInstanceDiff {
            added: Vec::new(),
            removed: Vec::new(),
            unchanged: Vec::new(),
        }
    }
    pub fn add(&mut self, s: &str) {
        self.added.push(s.to_string());
    }
    pub fn remove(&mut self, s: &str) {
        self.removed.push(s.to_string());
    }
    pub fn keep(&mut self, s: &str) {
        self.unchanged.push(s.to_string());
    }
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty()
    }
    pub fn total_changes(&self) -> usize {
        self.added.len() + self.removed.len()
    }
    pub fn net_additions(&self) -> i64 {
        self.added.len() as i64 - self.removed.len() as i64
    }
    pub fn summary(&self) -> String {
        format!(
            "+{} -{} =={}",
            self.added.len(),
            self.removed.len(),
            self.unchanged.len()
        )
    }
}
/// A result type for SynthInstance analysis.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum SynthInstanceResult {
    Ok(String),
    Err(String),
    Partial { done: usize, total: usize },
    Skipped,
}
#[allow(dead_code)]
impl SynthInstanceResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, SynthInstanceResult::Ok(_))
    }
    pub fn is_err(&self) -> bool {
        matches!(self, SynthInstanceResult::Err(_))
    }
    pub fn is_partial(&self) -> bool {
        matches!(self, SynthInstanceResult::Partial { .. })
    }
    pub fn is_skipped(&self) -> bool {
        matches!(self, SynthInstanceResult::Skipped)
    }
    pub fn ok_msg(&self) -> Option<&str> {
        match self {
            SynthInstanceResult::Ok(s) => Some(s),
            _ => None,
        }
    }
    pub fn err_msg(&self) -> Option<&str> {
        match self {
            SynthInstanceResult::Err(s) => Some(s),
            _ => None,
        }
    }
    pub fn progress(&self) -> f64 {
        match self {
            SynthInstanceResult::Ok(_) => 1.0,
            SynthInstanceResult::Err(_) => 0.0,
            SynthInstanceResult::Skipped => 0.0,
            SynthInstanceResult::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
        }
    }
}
/// A diagnostic reporter for SynthInstance.
#[allow(dead_code)]
pub struct SynthInstanceDiagnostics {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
#[allow(dead_code)]
impl SynthInstanceDiagnostics {
    pub fn new(max_errors: usize) -> Self {
        SynthInstanceDiagnostics {
            errors: Vec::new(),
            warnings: Vec::new(),
            notes: Vec::new(),
            max_errors,
        }
    }
    pub fn error(&mut self, msg: &str) {
        if self.errors.len() < self.max_errors {
            self.errors.push(msg.to_string());
        }
    }
    pub fn warning(&mut self, msg: &str) {
        self.warnings.push(msg.to_string());
    }
    pub fn note(&mut self, msg: &str) {
        self.notes.push(msg.to_string());
    }
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    pub fn num_errors(&self) -> usize {
        self.errors.len()
    }
    pub fn num_warnings(&self) -> usize {
        self.warnings.len()
    }
    pub fn is_clean(&self) -> bool {
        self.errors.is_empty() && self.warnings.is_empty()
    }
    pub fn at_error_limit(&self) -> bool {
        self.errors.len() >= self.max_errors
    }
    pub fn clear(&mut self) {
        self.errors.clear();
        self.warnings.clear();
        self.notes.clear();
    }
    pub fn summary(&self) -> String {
        format!(
            "{} error(s), {} warning(s)",
            self.errors.len(),
            self.warnings.len()
        )
    }
}
#[allow(dead_code)]
pub struct SynthInstanceExtPipeline1500 {
    pub name: String,
    pub passes: Vec<SynthInstanceExtPass1500>,
    pub run_count: usize,
}
impl SynthInstanceExtPipeline1500 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            passes: Vec::new(),
            run_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_pass(&mut self, pass: SynthInstanceExtPass1500) {
        self.passes.push(pass);
    }
    #[allow(dead_code)]
    pub fn run_all(&mut self, input: &str) -> Vec<SynthInstanceExtResult1500> {
        self.run_count += 1;
        self.passes
            .iter_mut()
            .filter(|p| p.enabled)
            .map(|p| p.run(input))
            .collect()
    }
    #[allow(dead_code)]
    pub fn num_passes(&self) -> usize {
        self.passes.len()
    }
    #[allow(dead_code)]
    pub fn num_enabled_passes(&self) -> usize {
        self.passes.iter().filter(|p| p.enabled).count()
    }
    #[allow(dead_code)]
    pub fn total_success_rate(&self) -> f64 {
        let total: usize = self.passes.iter().map(|p| p.total_runs).sum();
        let ok: usize = self.passes.iter().map(|p| p.successes).sum();
        if total == 0 {
            0.0
        } else {
            ok as f64 / total as f64
        }
    }
}
/// Reason for instance search failure.
#[derive(Clone, Debug)]
pub enum FailureReason {
    /// No matching instances found for this class.
    NoInstances,
    /// Unification with instance failed.
    UnificationFailed,
    /// Recursive loop detected in search.
    LoopDetected,
    /// Maximum depth exceeded.
    MaxDepthExceeded,
    /// Search timeout (heartbeat limit).
    Timeout,
    /// Unresolved postponed constraints.
    PostponedConstraints,
}
/// Diagnostics for a failed synthesis attempt.
#[derive(Clone, Debug)]
pub struct SynthDiagnostics {
    /// Reason for failure.
    pub failure_reason: FailureReason,
    /// Statistics about the search.
    pub stats: SearchStats,
    /// Candidate instances that were tried.
    pub tried_candidates: Vec<(Name, FailureReason)>,
}
/// An analysis pass for SynthInstance.
#[allow(dead_code)]
pub struct SynthInstanceAnalysisPass {
    pub name: String,
    pub enabled: bool,
    pub results: Vec<SynthInstanceResult>,
    pub total_runs: usize,
}
#[allow(dead_code)]
impl SynthInstanceAnalysisPass {
    pub fn new(name: &str) -> Self {
        SynthInstanceAnalysisPass {
            name: name.to_string(),
            enabled: true,
            results: Vec::new(),
            total_runs: 0,
        }
    }
    pub fn run(&mut self, input: &str) -> SynthInstanceResult {
        self.total_runs += 1;
        let result = if input.is_empty() {
            SynthInstanceResult::Err("empty input".to_string())
        } else {
            SynthInstanceResult::Ok(format!("processed: {}", input))
        };
        self.results.push(result.clone());
        result
    }
    pub fn success_count(&self) -> usize {
        self.results.iter().filter(|r| r.is_ok()).count()
    }
    pub fn error_count(&self) -> usize {
        self.results.iter().filter(|r| r.is_err()).count()
    }
    pub fn success_rate(&self) -> f64 {
        if self.total_runs == 0 {
            0.0
        } else {
            self.success_count() as f64 / self.total_runs as f64
        }
    }
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    pub fn clear_results(&mut self) {
        self.results.clear();
    }
}
#[allow(dead_code)]
pub struct SynthInstanceExtConfig1500 {
    pub(super) values: std::collections::HashMap<String, SynthInstanceExtConfigVal1500>,
    pub(super) read_only: bool,
    pub(super) name: String,
}
impl SynthInstanceExtConfig1500 {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            values: std::collections::HashMap::new(),
            read_only: false,
            name: String::new(),
        }
    }
    #[allow(dead_code)]
    pub fn named(name: &str) -> Self {
        Self {
            values: std::collections::HashMap::new(),
            read_only: false,
            name: name.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn set(&mut self, key: &str, value: SynthInstanceExtConfigVal1500) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&SynthInstanceExtConfigVal1500> {
        self.values.get(key)
    }
    #[allow(dead_code)]
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.get(key)?.as_bool()
    }
    #[allow(dead_code)]
    pub fn get_int(&self, key: &str) -> Option<i64> {
        self.get(key)?.as_int()
    }
    #[allow(dead_code)]
    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.get(key)?.as_str()
    }
    #[allow(dead_code)]
    pub fn set_bool(&mut self, key: &str, v: bool) -> bool {
        self.set(key, SynthInstanceExtConfigVal1500::Bool(v))
    }
    #[allow(dead_code)]
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, SynthInstanceExtConfigVal1500::Int(v))
    }
    #[allow(dead_code)]
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, SynthInstanceExtConfigVal1500::Str(v.to_string()))
    }
    #[allow(dead_code)]
    pub fn lock(&mut self) {
        self.read_only = true;
    }
    #[allow(dead_code)]
    pub fn unlock(&mut self) {
        self.read_only = false;
    }
    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.values.len()
    }
    #[allow(dead_code)]
    pub fn has(&self, key: &str) -> bool {
        self.values.contains_key(key)
    }
    #[allow(dead_code)]
    pub fn remove(&mut self, key: &str) -> bool {
        self.values.remove(key).is_some()
    }
}
/// Result of instance synthesis.
#[derive(Clone, Debug)]
pub enum SynthResult {
    /// Successfully synthesized an instance.
    Success(Expr),
    /// Synthesis failed (no matching instance found).
    Failure,
    /// Synthesis is stuck (waiting for metavar assignment).
    Stuck(MVarId),
}
impl SynthResult {
    /// Check if synthesis succeeded.
    pub fn is_success(&self) -> bool {
        matches!(self, SynthResult::Success(_))
    }
    /// Get the synthesized expression, if successful.
    pub fn expr(&self) -> Option<&Expr> {
        match self {
            SynthResult::Success(e) => Some(e),
            _ => None,
        }
    }
}
/// A generator/consumer node in the tabled resolution search.
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub(crate) struct ResolutionNode {
    /// Goal being solved.
    pub(super) goal: Expr,
    /// Depth of this node in search tree.
    pub(super) depth: u32,
    /// Candidates to try for this goal.
    pub(super) candidates: Vec<InstanceEntry>,
    /// Current candidate index.
    pub(super) current_index: usize,
    /// Whether this node has been completed.
    pub(super) completed: bool,
    /// Cached result, if any.
    pub(super) cached_result: Option<Expr>,
}
/// A configuration store for SynthInstance.
#[allow(dead_code)]
pub struct SynthInstanceConfig {
    pub values: std::collections::HashMap<String, SynthInstanceConfigValue>,
    pub read_only: bool,
}
#[allow(dead_code)]
impl SynthInstanceConfig {
    pub fn new() -> Self {
        SynthInstanceConfig {
            values: std::collections::HashMap::new(),
            read_only: false,
        }
    }
    pub fn set(&mut self, key: &str, value: SynthInstanceConfigValue) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    pub fn get(&self, key: &str) -> Option<&SynthInstanceConfigValue> {
        self.values.get(key)
    }
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.get(key)?.as_bool()
    }
    pub fn get_int(&self, key: &str) -> Option<i64> {
        self.get(key)?.as_int()
    }
    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.get(key)?.as_str()
    }
    pub fn set_bool(&mut self, key: &str, v: bool) -> bool {
        self.set(key, SynthInstanceConfigValue::Bool(v))
    }
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, SynthInstanceConfigValue::Int(v))
    }
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, SynthInstanceConfigValue::Str(v.to_string()))
    }
    pub fn lock(&mut self) {
        self.read_only = true;
    }
    pub fn unlock(&mut self) {
        self.read_only = false;
    }
    pub fn size(&self) -> usize {
        self.values.len()
    }
    pub fn has(&self, key: &str) -> bool {
        self.values.contains_key(key)
    }
    pub fn remove(&mut self, key: &str) -> bool {
        self.values.remove(key).is_some()
    }
}
/// Statistics about a synthesis search.
#[derive(Clone, Debug, Default)]
pub struct SearchStats {
    /// Total instances examined.
    pub instances_examined: u32,
    /// Successful unifications.
    pub successful_unifications: u32,
    /// Failed unifications.
    pub failed_unifications: u32,
    /// Recursive calls made.
    pub recursive_calls: u32,
    /// Cache hits.
    pub cache_hits: u32,
    /// Heartbeats used.
    pub heartbeats_used: u64,
    /// Maximum depth reached.
    pub max_depth_reached: u32,
}
#[allow(dead_code)]
pub struct SynthInstanceExtPass1500 {
    pub name: String,
    pub total_runs: usize,
    pub successes: usize,
    pub errors: usize,
    pub enabled: bool,
    pub results: Vec<SynthInstanceExtResult1500>,
}
impl SynthInstanceExtPass1500 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            total_runs: 0,
            successes: 0,
            errors: 0,
            enabled: true,
            results: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn run(&mut self, input: &str) -> SynthInstanceExtResult1500 {
        if !self.enabled {
            return SynthInstanceExtResult1500::Skipped;
        }
        self.total_runs += 1;
        let result = if input.is_empty() {
            self.errors += 1;
            SynthInstanceExtResult1500::Err(format!("empty input in pass '{}'", self.name))
        } else {
            self.successes += 1;
            SynthInstanceExtResult1500::Ok(format!(
                "processed {} chars in pass '{}'",
                input.len(),
                self.name
            ))
        };
        self.results.push(result.clone());
        result
    }
    #[allow(dead_code)]
    pub fn success_count(&self) -> usize {
        self.successes
    }
    #[allow(dead_code)]
    pub fn error_count(&self) -> usize {
        self.errors
    }
    #[allow(dead_code)]
    pub fn success_rate(&self) -> f64 {
        if self.total_runs == 0 {
            0.0
        } else {
            self.successes as f64 / self.total_runs as f64
        }
    }
    #[allow(dead_code)]
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    #[allow(dead_code)]
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    #[allow(dead_code)]
    pub fn clear_results(&mut self) {
        self.results.clear();
    }
}
#[allow(dead_code)]
pub struct SynthInstanceExtDiag1500 {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
impl SynthInstanceExtDiag1500 {
    #[allow(dead_code)]
    pub fn new(max_errors: usize) -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
            notes: Vec::new(),
            max_errors,
        }
    }
    #[allow(dead_code)]
    pub fn error(&mut self, msg: &str) {
        if self.errors.len() < self.max_errors {
            self.errors.push(msg.to_string());
        }
    }
    #[allow(dead_code)]
    pub fn warning(&mut self, msg: &str) {
        self.warnings.push(msg.to_string());
    }
    #[allow(dead_code)]
    pub fn note(&mut self, msg: &str) {
        self.notes.push(msg.to_string());
    }
    #[allow(dead_code)]
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    #[allow(dead_code)]
    pub fn num_errors(&self) -> usize {
        self.errors.len()
    }
    #[allow(dead_code)]
    pub fn num_warnings(&self) -> usize {
        self.warnings.len()
    }
    #[allow(dead_code)]
    pub fn is_clean(&self) -> bool {
        self.errors.is_empty() && self.warnings.is_empty()
    }
    #[allow(dead_code)]
    pub fn at_error_limit(&self) -> bool {
        self.errors.len() >= self.max_errors
    }
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.errors.clear();
        self.warnings.clear();
        self.notes.clear();
    }
    #[allow(dead_code)]
    pub fn summary(&self) -> String {
        format!(
            "{} error(s), {} warning(s)",
            self.errors.len(),
            self.warnings.len()
        )
    }
}
