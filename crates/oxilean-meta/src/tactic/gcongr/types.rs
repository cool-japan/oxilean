//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct GcongrExtDiff1000 {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
impl GcongrExtDiff1000 {
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
/// A builder pattern for GCongrTac.
#[allow(dead_code)]
pub struct GCongrTacBuilder {
    pub name: String,
    pub items: Vec<String>,
    pub config: std::collections::HashMap<String, String>,
}
#[allow(dead_code)]
impl GCongrTacBuilder {
    pub fn new(name: &str) -> Self {
        GCongrTacBuilder {
            name: name.to_string(),
            items: Vec::new(),
            config: std::collections::HashMap::new(),
        }
    }
    pub fn add_item(mut self, item: &str) -> Self {
        self.items.push(item.to_string());
        self
    }
    pub fn set_config(mut self, key: &str, value: &str) -> Self {
        self.config.insert(key.to_string(), value.to_string());
        self
    }
    pub fn item_count(&self) -> usize {
        self.items.len()
    }
    pub fn has_config(&self, key: &str) -> bool {
        self.config.contains_key(key)
    }
    pub fn get_config(&self, key: &str) -> Option<&str> {
        self.config.get(key).map(|s| s.as_str())
    }
    pub fn build_summary(&self) -> String {
        format!(
            "{}: {} items, {} config keys",
            self.name,
            self.items.len(),
            self.config.len()
        )
    }
}
/// A state machine for GCongrTac.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum GCongrTacState {
    Initial,
    Running,
    Paused,
    Complete,
    Failed(String),
}
#[allow(dead_code)]
impl GCongrTacState {
    pub fn is_terminal(&self) -> bool {
        matches!(self, GCongrTacState::Complete | GCongrTacState::Failed(_))
    }
    pub fn can_run(&self) -> bool {
        matches!(self, GCongrTacState::Initial | GCongrTacState::Paused)
    }
    pub fn is_running(&self) -> bool {
        matches!(self, GCongrTacState::Running)
    }
    pub fn error_msg(&self) -> Option<&str> {
        match self {
            GCongrTacState::Failed(s) => Some(s),
            _ => None,
        }
    }
}
/// A diff for TacticGcongr analysis results.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TacticGcongrDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
#[allow(dead_code)]
impl TacticGcongrDiff {
    pub fn new() -> Self {
        TacticGcongrDiff {
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
/// The result of applying the `gcongr` tactic.
#[derive(Clone, Debug)]
pub struct GCongrResult {
    /// Whether the tactic succeeded.
    pub success: bool,
    /// The sub-goals produced (empty if the goal was closed).
    pub subgoals: Vec<GCongrSubgoal>,
    /// The monotone entry that was used (if any).
    pub used_entry: Option<String>,
    /// The depth at which the tactic was applied.
    pub depth: usize,
    /// Diagnostic message.
    pub message: String,
}
impl GCongrResult {
    /// Create a success result with sub-goals.
    pub fn success(subgoals: Vec<GCongrSubgoal>, entry_name: Option<String>) -> Self {
        let msg = if subgoals.is_empty() {
            "gcongr closed the goal".to_string()
        } else {
            format!("gcongr produced {} sub-goal(s)", subgoals.len())
        };
        Self {
            success: true,
            subgoals,
            used_entry: entry_name,
            depth: 0,
            message: msg,
        }
    }
    /// Create a failure result.
    pub fn failure(message: impl Into<String>) -> Self {
        Self {
            success: false,
            subgoals: Vec::new(),
            used_entry: None,
            depth: 0,
            message: message.into(),
        }
    }
    /// Return `true` if all sub-goals were closed.
    pub fn is_closed(&self) -> bool {
        self.success && self.subgoals.is_empty()
    }
    /// Return the number of remaining sub-goals.
    pub fn remaining_goals(&self) -> usize {
        self.subgoals.len()
    }
}
#[allow(dead_code)]
pub struct GcongrExtConfig1000 {
    pub(super) values: std::collections::HashMap<String, GcongrExtConfigVal1000>,
    pub(super) read_only: bool,
    pub(super) name: String,
}
impl GcongrExtConfig1000 {
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
    pub fn set(&mut self, key: &str, value: GcongrExtConfigVal1000) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&GcongrExtConfigVal1000> {
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
        self.set(key, GcongrExtConfigVal1000::Bool(v))
    }
    #[allow(dead_code)]
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, GcongrExtConfigVal1000::Int(v))
    }
    #[allow(dead_code)]
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, GcongrExtConfigVal1000::Str(v.to_string()))
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
/// A diagnostic reporter for TacticGcongr.
#[allow(dead_code)]
pub struct TacticGcongrDiagnostics {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
#[allow(dead_code)]
impl TacticGcongrDiagnostics {
    pub fn new(max_errors: usize) -> Self {
        TacticGcongrDiagnostics {
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
/// An analysis pass for TacticGcongr.
#[allow(dead_code)]
pub struct TacticGcongrAnalysisPass {
    pub name: String,
    pub enabled: bool,
    pub results: Vec<TacticGcongrResult>,
    pub total_runs: usize,
}
#[allow(dead_code)]
impl TacticGcongrAnalysisPass {
    pub fn new(name: &str) -> Self {
        TacticGcongrAnalysisPass {
            name: name.to_string(),
            enabled: true,
            results: Vec::new(),
            total_runs: 0,
        }
    }
    pub fn run(&mut self, input: &str) -> TacticGcongrResult {
        self.total_runs += 1;
        let result = if input.is_empty() {
            TacticGcongrResult::Err("empty input".to_string())
        } else {
            TacticGcongrResult::Ok(format!("processed: {}", input))
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
/// A counter map for GCongrTac frequency analysis.
#[allow(dead_code)]
pub struct GCongrTacCounterMap {
    pub counts: std::collections::HashMap<String, usize>,
    pub total: usize,
}
#[allow(dead_code)]
impl GCongrTacCounterMap {
    pub fn new() -> Self {
        GCongrTacCounterMap {
            counts: std::collections::HashMap::new(),
            total: 0,
        }
    }
    pub fn increment(&mut self, key: &str) {
        *self.counts.entry(key.to_string()).or_insert(0) += 1;
        self.total += 1;
    }
    pub fn count(&self, key: &str) -> usize {
        *self.counts.get(key).unwrap_or(&0)
    }
    pub fn frequency(&self, key: &str) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            self.count(key) as f64 / self.total as f64
        }
    }
    pub fn most_common(&self) -> Option<(&String, usize)> {
        self.counts
            .iter()
            .max_by_key(|(_, &v)| v)
            .map(|(k, &v)| (k, v))
    }
    pub fn num_unique(&self) -> usize {
        self.counts.len()
    }
    pub fn is_empty(&self) -> bool {
        self.counts.is_empty()
    }
}
/// The generalized congruence tactic.
#[derive(Clone, Debug)]
pub struct GCongrTactic {
    pub(super) config: GCongrConfig,
    pub(super) registry: MonotoneRegistry,
}
impl GCongrTactic {
    /// Create a new `GCongrTactic` with default configuration and default registry.
    pub fn new() -> Self {
        Self {
            config: GCongrConfig::default(),
            registry: MonotoneRegistry::with_defaults(),
        }
    }
    /// Create a new `GCongrTactic` with custom config and default registry.
    pub fn with_config(config: GCongrConfig) -> Self {
        Self {
            config,
            registry: MonotoneRegistry::with_defaults(),
        }
    }
    /// Create a new `GCongrTactic` with both custom config and registry.
    pub fn with_config_and_registry(config: GCongrConfig, registry: MonotoneRegistry) -> Self {
        Self { config, registry }
    }
    /// Decompose a goal string into a `GCongrGoal`.
    ///
    /// Expects goals like `f a b <= f c d`, `Nat.add x y >= Nat.add z w`, etc.
    pub fn decompose_goal(&self, goal: &str) -> Option<GCongrGoal> {
        let trimmed = goal.trim();
        let relation_ops: &[(&str, GCongrRelation)] = &[
            (" <= ", GCongrRelation::Le),
            (" < ", GCongrRelation::Lt),
            (" >= ", GCongrRelation::Ge),
            (" > ", GCongrRelation::Gt),
            (" = ", GCongrRelation::Eq),
            (" ⊆ ", GCongrRelation::Subset),
        ];
        for &(op_str, ref rel) in relation_ops {
            if let Some(pos) = find_op_at_depth0(trimmed, op_str) {
                if let Some(ref filter) = self.config.relation_filter {
                    if !filter.is_compatible(rel) {
                        continue;
                    }
                }
                let lhs = trimmed[..pos].trim().to_string();
                let rhs = trimmed[pos + op_str.len()..].trim().to_string();
                let lhs_parts = split_application(&lhs);
                let rhs_parts = split_application(&rhs);
                let lhs_head = lhs_parts.first().map(|s| s.to_string());
                let rhs_head = rhs_parts.first().map(|s| s.to_string());
                let lhs_args: Vec<String> =
                    lhs_parts.iter().skip(1).map(|s| s.to_string()).collect();
                let rhs_args: Vec<String> =
                    rhs_parts.iter().skip(1).map(|s| s.to_string()).collect();
                return Some(GCongrGoal {
                    lhs,
                    rhs,
                    relation: rel.clone(),
                    lhs_head,
                    rhs_head,
                    lhs_args,
                    rhs_args,
                });
            }
        }
        None
    }
    /// Apply `gcongr` to a goal string, producing sub-goals.
    pub fn apply(&self, goal: &str) -> GCongrResult {
        self.apply_at_depth(goal, 0)
    }
    /// Apply `gcongr` at a given recursion depth.
    fn apply_at_depth(&self, goal: &str, depth: usize) -> GCongrResult {
        if depth >= self.config.max_depth {
            return GCongrResult::failure("gcongr: max depth exceeded");
        }
        let decomposed = match self.decompose_goal(goal) {
            Some(g) => g,
            None => return GCongrResult::failure("gcongr: could not decompose goal"),
        };
        if !decomposed.heads_match() {
            return GCongrResult::failure(format!(
                "gcongr: head functions do not match: {:?} vs {:?}",
                decomposed.lhs_head, decomposed.rhs_head
            ));
        }
        if !decomposed.arities_match() {
            return GCongrResult::failure("gcongr: argument arities do not match");
        }
        let head = match &decomposed.lhs_head {
            Some(h) => h.clone(),
            None => return GCongrResult::failure("gcongr: no head function found"),
        };
        let entries = self.registry.lookup(&head);
        if let Some(entry) = entries.first() {
            let subgoals = self.generate_subgoals_from_entry(entry, &decomposed);
            let mut result = GCongrResult::success(subgoals, Some(entry.function.clone()));
            result.depth = depth;
            if self.config.try_refl {
                result.subgoals.retain(|sg| !sg.is_reflexive());
            }
            if self.config.recursive && depth + 1 < self.config.max_depth {
                let mut new_subgoals = Vec::new();
                for sg in &result.subgoals {
                    let sg_goal = sg.to_goal_string();
                    let sub_result = self.apply_at_depth(&sg_goal, depth + 1);
                    if sub_result.success && !sub_result.subgoals.is_empty() {
                        new_subgoals.extend(sub_result.subgoals);
                    } else {
                        new_subgoals.push(sg.clone());
                    }
                }
                result.subgoals = new_subgoals;
            }
            result
        } else {
            let subgoals = self.structural_decompose(&decomposed);
            let mut result = GCongrResult::success(subgoals, None);
            result.depth = depth;
            if self.config.try_refl {
                result.subgoals.retain(|sg| !sg.is_reflexive());
            }
            result
        }
    }
    /// Generate sub-goals using a monotone registry entry.
    fn generate_subgoals_from_entry(
        &self,
        entry: &MonotoneEntry,
        goal: &GCongrGoal,
    ) -> Vec<GCongrSubgoal> {
        let mut subgoals = Vec::new();
        let n = goal
            .lhs_args
            .len()
            .min(goal.rhs_args.len())
            .min(entry.arity);
        let mut premise_idx = 0;
        for i in 0..n {
            let kind = entry.arg_kinds.get(i).unwrap_or(&ArgMonotonicity::Monotone);
            match kind {
                ArgMonotonicity::Monotone => {
                    let rel = entry
                        .premise_relations
                        .get(premise_idx)
                        .cloned()
                        .unwrap_or_else(|| goal.relation.clone());
                    subgoals.push(GCongrSubgoal {
                        lhs: goal.lhs_args[i].clone(),
                        rhs: goal.rhs_args[i].clone(),
                        relation: rel,
                        arg_index: i,
                        monotonicity: ArgMonotonicity::Monotone,
                    });
                    premise_idx += 1;
                }
                ArgMonotonicity::Antitone => {
                    let rel = entry
                        .premise_relations
                        .get(premise_idx)
                        .cloned()
                        .unwrap_or_else(|| goal.relation.flip());
                    subgoals.push(GCongrSubgoal {
                        lhs: goal.rhs_args[i].clone(),
                        rhs: goal.lhs_args[i].clone(),
                        relation: rel,
                        arg_index: i,
                        monotonicity: ArgMonotonicity::Antitone,
                    });
                    premise_idx += 1;
                }
                ArgMonotonicity::Congruent => {
                    subgoals.push(GCongrSubgoal {
                        lhs: goal.lhs_args[i].clone(),
                        rhs: goal.rhs_args[i].clone(),
                        relation: GCongrRelation::Eq,
                        arg_index: i,
                        monotonicity: ArgMonotonicity::Congruent,
                    });
                }
                ArgMonotonicity::Ignore => {}
            }
        }
        subgoals
    }
    /// Structural decomposition: assume each argument is monotone.
    fn structural_decompose(&self, goal: &GCongrGoal) -> Vec<GCongrSubgoal> {
        let n = goal.lhs_args.len().min(goal.rhs_args.len());
        let mut subgoals = Vec::new();
        for i in 0..n {
            subgoals.push(GCongrSubgoal {
                lhs: goal.lhs_args[i].clone(),
                rhs: goal.rhs_args[i].clone(),
                relation: goal.relation.clone(),
                arg_index: i,
                monotonicity: ArgMonotonicity::Monotone,
            });
        }
        subgoals
    }
    /// Get a reference to the registry.
    pub fn registry(&self) -> &MonotoneRegistry {
        &self.registry
    }
    /// Get a mutable reference to the registry for adding custom entries.
    pub fn registry_mut(&mut self) -> &mut MonotoneRegistry {
        &mut self.registry
    }
}
/// Ordering / relation kinds supported by `gcongr`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum GCongrRelation {
    /// `<=`
    Le,
    /// `<`
    Lt,
    /// `>=`
    Ge,
    /// `>`
    Gt,
    /// `=`
    Eq,
    /// Subset: `subseteq`
    Subset,
    /// Divisibility
    Dvd,
    /// A named custom relation.
    Custom(String),
}
impl GCongrRelation {
    /// Parse a relation from its string representation.
    pub fn parse(s: &str) -> Option<GCongrRelation> {
        match s.trim() {
            "<=" | "≤" | "LE.le" => Some(GCongrRelation::Le),
            "<" | "LT.lt" => Some(GCongrRelation::Lt),
            ">=" | "≥" | "GE.ge" => Some(GCongrRelation::Ge),
            ">" | "GT.gt" => Some(GCongrRelation::Gt),
            "=" | "Eq" => Some(GCongrRelation::Eq),
            "⊆" | "HasSubset.subset" => Some(GCongrRelation::Subset),
            "∣" | "Dvd.dvd" => Some(GCongrRelation::Dvd),
            _ => {
                if s.trim().is_empty() {
                    None
                } else {
                    Some(GCongrRelation::Custom(s.trim().to_string()))
                }
            }
        }
    }
    /// Return the symbol for display.
    pub fn symbol(&self) -> &str {
        match self {
            GCongrRelation::Le => "<=",
            GCongrRelation::Lt => "<",
            GCongrRelation::Ge => ">=",
            GCongrRelation::Gt => ">",
            GCongrRelation::Eq => "=",
            GCongrRelation::Subset => "⊆",
            GCongrRelation::Dvd => "∣",
            GCongrRelation::Custom(s) => s,
        }
    }
    /// Whether this is a non-strict ordering (Le, Ge, Eq, Subset).
    pub fn is_non_strict(&self) -> bool {
        matches!(
            self,
            GCongrRelation::Le | GCongrRelation::Ge | GCongrRelation::Eq | GCongrRelation::Subset
        )
    }
    /// Whether this is a strict ordering (Lt, Gt).
    pub fn is_strict(&self) -> bool {
        matches!(self, GCongrRelation::Lt | GCongrRelation::Gt)
    }
    /// Flip the direction: Le <-> Ge, Lt <-> Gt.
    pub fn flip(&self) -> GCongrRelation {
        match self {
            GCongrRelation::Le => GCongrRelation::Ge,
            GCongrRelation::Lt => GCongrRelation::Gt,
            GCongrRelation::Ge => GCongrRelation::Le,
            GCongrRelation::Gt => GCongrRelation::Lt,
            other => other.clone(),
        }
    }
    /// Whether two relations are compatible (can chain transitively).
    pub fn is_compatible(&self, other: &GCongrRelation) -> bool {
        match (self, other) {
            (GCongrRelation::Le, GCongrRelation::Le)
            | (GCongrRelation::Le, GCongrRelation::Lt)
            | (GCongrRelation::Lt, GCongrRelation::Le)
            | (GCongrRelation::Lt, GCongrRelation::Lt) => true,
            (GCongrRelation::Ge, GCongrRelation::Ge)
            | (GCongrRelation::Ge, GCongrRelation::Gt)
            | (GCongrRelation::Gt, GCongrRelation::Ge)
            | (GCongrRelation::Gt, GCongrRelation::Gt) => true,
            (GCongrRelation::Eq, _) | (_, GCongrRelation::Eq) => true,
            (GCongrRelation::Subset, GCongrRelation::Subset) => true,
            (GCongrRelation::Dvd, GCongrRelation::Dvd) => true,
            (GCongrRelation::Custom(a), GCongrRelation::Custom(b)) => a == b,
            _ => false,
        }
    }
    /// The weakest relation that contains both `self` and `other`.
    pub fn join(&self, other: &GCongrRelation) -> Option<GCongrRelation> {
        if self == other {
            return Some(self.clone());
        }
        match (self, other) {
            (GCongrRelation::Eq, r) | (r, GCongrRelation::Eq) => Some(r.clone()),
            (GCongrRelation::Le, GCongrRelation::Lt) | (GCongrRelation::Lt, GCongrRelation::Le) => {
                Some(GCongrRelation::Le)
            }
            (GCongrRelation::Ge, GCongrRelation::Gt) | (GCongrRelation::Gt, GCongrRelation::Ge) => {
                Some(GCongrRelation::Ge)
            }
            _ => None,
        }
    }
}
/// A decomposed relational goal: `lhs R rhs`.
#[derive(Clone, Debug)]
pub struct GCongrGoal {
    /// The full LHS expression string.
    pub lhs: String,
    /// The full RHS expression string.
    pub rhs: String,
    /// The relation between LHS and RHS.
    pub relation: GCongrRelation,
    /// The head function on the LHS (if applicable).
    pub lhs_head: Option<String>,
    /// The head function on the RHS (if applicable).
    pub rhs_head: Option<String>,
    /// Arguments of the LHS head function.
    pub lhs_args: Vec<String>,
    /// Arguments of the RHS head function.
    pub rhs_args: Vec<String>,
}
impl GCongrGoal {
    /// Return `true` if both sides have the same head function.
    pub fn heads_match(&self) -> bool {
        match (&self.lhs_head, &self.rhs_head) {
            (Some(a), Some(b)) => a == b,
            _ => false,
        }
    }
    /// Return `true` if the arities match.
    pub fn arities_match(&self) -> bool {
        self.lhs_args.len() == self.rhs_args.len()
    }
}
/// A sub-goal produced by `gcongr`.
#[derive(Clone, Debug)]
pub struct GCongrSubgoal {
    /// The LHS of the sub-goal.
    pub lhs: String,
    /// The RHS of the sub-goal.
    pub rhs: String,
    /// The relation for this sub-goal.
    pub relation: GCongrRelation,
    /// Which argument position (0-indexed) this sub-goal corresponds to.
    pub arg_index: usize,
    /// The monotonicity kind for this argument.
    pub monotonicity: ArgMonotonicity,
}
impl GCongrSubgoal {
    /// Format as a goal string like `a <= b`.
    pub fn to_goal_string(&self) -> String {
        format!("{} {} {}", self.lhs, self.relation.symbol(), self.rhs)
    }
    /// Return `true` if the sub-goal is trivially reflexive (lhs == rhs).
    pub fn is_reflexive(&self) -> bool {
        self.lhs == self.rhs
    }
}
/// A typed slot for TacticGcongr configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum TacticGcongrConfigValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
#[allow(dead_code)]
impl TacticGcongrConfigValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            TacticGcongrConfigValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            TacticGcongrConfigValue::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            TacticGcongrConfigValue::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            TacticGcongrConfigValue::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&[String]> {
        match self {
            TacticGcongrConfigValue::List(v) => Some(v),
            _ => None,
        }
    }
    pub fn type_name(&self) -> &'static str {
        match self {
            TacticGcongrConfigValue::Bool(_) => "bool",
            TacticGcongrConfigValue::Int(_) => "int",
            TacticGcongrConfigValue::Float(_) => "float",
            TacticGcongrConfigValue::Str(_) => "str",
            TacticGcongrConfigValue::List(_) => "list",
        }
    }
}
/// An extended utility type for GCongrTac.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct GCongrTacExt {
    /// A tag for identifying this utility instance.
    pub tag: u32,
    /// An optional description string.
    pub description: Option<String>,
}
#[allow(dead_code)]
impl GCongrTacExt {
    /// Creates a new default instance.
    pub fn new() -> Self {
        Self {
            tag: 0,
            description: None,
        }
    }
    /// Sets the tag.
    pub fn with_tag(mut self, tag: u32) -> Self {
        self.tag = tag;
        self
    }
    /// Sets the description.
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
    /// Returns `true` if the description is set.
    pub fn has_description(&self) -> bool {
        self.description.is_some()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum GcongrExtResult1000 {
    /// Operation completed successfully.
    Ok(String),
    /// Operation encountered an error.
    Err(String),
    /// Operation partially completed.
    Partial { done: usize, total: usize },
    /// Operation was skipped.
    Skipped,
}
impl GcongrExtResult1000 {
    #[allow(dead_code)]
    pub fn is_ok(&self) -> bool {
        matches!(self, GcongrExtResult1000::Ok(_))
    }
    #[allow(dead_code)]
    pub fn is_err(&self) -> bool {
        matches!(self, GcongrExtResult1000::Err(_))
    }
    #[allow(dead_code)]
    pub fn is_partial(&self) -> bool {
        matches!(self, GcongrExtResult1000::Partial { .. })
    }
    #[allow(dead_code)]
    pub fn is_skipped(&self) -> bool {
        matches!(self, GcongrExtResult1000::Skipped)
    }
    #[allow(dead_code)]
    pub fn ok_msg(&self) -> Option<&str> {
        if let GcongrExtResult1000::Ok(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn err_msg(&self) -> Option<&str> {
        if let GcongrExtResult1000::Err(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn progress(&self) -> f64 {
        match self {
            GcongrExtResult1000::Ok(_) => 1.0,
            GcongrExtResult1000::Err(_) => 0.0,
            GcongrExtResult1000::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
            GcongrExtResult1000::Skipped => 0.5,
        }
    }
}
/// A registered monotone function entry.
#[derive(Clone, Debug)]
pub struct MonotoneEntry {
    /// The function name (e.g. "Nat.add", "List.length").
    pub function: String,
    /// The number of explicit arguments.
    pub arity: usize,
    /// The monotonicity kind of each argument position.
    pub arg_kinds: Vec<ArgMonotonicity>,
    /// The relation in the conclusion when all monotone args satisfy their relation.
    pub conclusion_relation: GCongrRelation,
    /// The relations required for monotone argument positions.
    pub premise_relations: Vec<GCongrRelation>,
    /// Priority (lower = tried first).
    pub priority: u32,
}
impl MonotoneEntry {
    /// Create a new entry where every argument is monotone in the same relation.
    pub fn uniform(function: impl Into<String>, arity: usize, relation: GCongrRelation) -> Self {
        let arg_kinds = vec![ArgMonotonicity::Monotone; arity];
        let premise_relations = vec![relation.clone(); arity];
        Self {
            function: function.into(),
            arity,
            arg_kinds,
            conclusion_relation: relation,
            premise_relations,
            priority: 1000,
        }
    }
    /// Create a new entry with explicit argument kinds.
    pub fn with_args(
        function: impl Into<String>,
        arg_kinds: Vec<ArgMonotonicity>,
        conclusion_relation: GCongrRelation,
        premise_relations: Vec<GCongrRelation>,
    ) -> Self {
        let arity = arg_kinds.len();
        Self {
            function: function.into(),
            arity,
            arg_kinds,
            conclusion_relation,
            premise_relations,
            priority: 1000,
        }
    }
    /// Set the priority.
    pub fn with_priority(mut self, priority: u32) -> Self {
        self.priority = priority;
        self
    }
    /// Return the number of monotone argument positions (i.e. non-Ignore, non-Congruent).
    pub fn monotone_arg_count(&self) -> usize {
        self.arg_kinds
            .iter()
            .filter(|k| matches!(k, ArgMonotonicity::Monotone | ArgMonotonicity::Antitone))
            .count()
    }
    /// Return `true` if this entry matches the given function name.
    pub fn matches_function(&self, name: &str) -> bool {
        self.function == name
    }
}
/// A collection of known monotone functions for the `gcongr` tactic.
#[derive(Clone, Debug)]
pub struct MonotoneRegistry {
    /// Entries indexed by function name.
    pub(super) entries: HashMap<String, Vec<MonotoneEntry>>,
    /// Total number of entries.
    pub(super) count: usize,
}
impl MonotoneRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            count: 0,
        }
    }
    /// Create a registry pre-populated with standard monotone functions.
    pub fn with_defaults() -> Self {
        let mut reg = Self::new();
        reg.register_defaults();
        reg
    }
    /// Register a monotone entry.
    pub fn register(&mut self, entry: MonotoneEntry) {
        self.entries
            .entry(entry.function.clone())
            .or_default()
            .push(entry);
        self.count += 1;
    }
    /// Look up entries for a function by name.
    pub fn lookup(&self, function: &str) -> &[MonotoneEntry] {
        self.entries
            .get(function)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }
    /// Return the total number of registered entries.
    pub fn len(&self) -> usize {
        self.count
    }
    /// Return `true` if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
    /// Return all registered function names.
    pub fn function_names(&self) -> Vec<&str> {
        self.entries.keys().map(|s| s.as_str()).collect()
    }
    /// Register the default built-in monotone functions.
    fn register_defaults(&mut self) {
        self.register(MonotoneEntry::uniform("Nat.add", 2, GCongrRelation::Le));
        self.register(MonotoneEntry::uniform("Nat.mul", 2, GCongrRelation::Le));
        self.register(MonotoneEntry::uniform("Int.add", 2, GCongrRelation::Le));
        self.register(MonotoneEntry::uniform("Int.mul", 2, GCongrRelation::Le));
        self.register(MonotoneEntry::uniform("HAdd.hAdd", 2, GCongrRelation::Le));
        self.register(MonotoneEntry::uniform("HMul.hMul", 2, GCongrRelation::Le));
        self.register(MonotoneEntry::uniform("Nat.succ", 1, GCongrRelation::Le));
        self.register(MonotoneEntry::uniform("max", 2, GCongrRelation::Le));
        self.register(MonotoneEntry::uniform("min", 2, GCongrRelation::Le));
        self.register(MonotoneEntry::uniform(
            "Set.union",
            2,
            GCongrRelation::Subset,
        ));
        self.register(MonotoneEntry::uniform(
            "Set.inter",
            2,
            GCongrRelation::Subset,
        ));
        self.register(MonotoneEntry::with_args(
            "Nat.pow",
            vec![ArgMonotonicity::Monotone, ArgMonotonicity::Congruent],
            GCongrRelation::Le,
            vec![GCongrRelation::Le],
        ));
        self.register(
            MonotoneEntry::uniform("List.length", 1, GCongrRelation::Le).with_priority(500),
        );
        self.register(
            MonotoneEntry::uniform("Finset.card", 1, GCongrRelation::Le).with_priority(500),
        );
        self.register(MonotoneEntry::with_args(
            "Neg.neg",
            vec![ArgMonotonicity::Antitone],
            GCongrRelation::Ge,
            vec![GCongrRelation::Le],
        ));
        self.register(MonotoneEntry::with_args(
            "HSub.hSub",
            vec![ArgMonotonicity::Monotone, ArgMonotonicity::Antitone],
            GCongrRelation::Le,
            vec![GCongrRelation::Le, GCongrRelation::Ge],
        ));
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum GcongrExtConfigVal1000 {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
impl GcongrExtConfigVal1000 {
    #[allow(dead_code)]
    pub fn as_bool(&self) -> Option<bool> {
        if let GcongrExtConfigVal1000::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_int(&self) -> Option<i64> {
        if let GcongrExtConfigVal1000::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_float(&self) -> Option<f64> {
        if let GcongrExtConfigVal1000::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_str(&self) -> Option<&str> {
        if let GcongrExtConfigVal1000::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_list(&self) -> Option<&[String]> {
        if let GcongrExtConfigVal1000::List(l) = self {
            Some(l)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn type_name(&self) -> &'static str {
        match self {
            GcongrExtConfigVal1000::Bool(_) => "bool",
            GcongrExtConfigVal1000::Int(_) => "int",
            GcongrExtConfigVal1000::Float(_) => "float",
            GcongrExtConfigVal1000::Str(_) => "str",
            GcongrExtConfigVal1000::List(_) => "list",
        }
    }
}
/// A configuration store for TacticGcongr.
#[allow(dead_code)]
pub struct TacticGcongrConfig {
    pub values: std::collections::HashMap<String, TacticGcongrConfigValue>,
    pub read_only: bool,
}
#[allow(dead_code)]
impl TacticGcongrConfig {
    pub fn new() -> Self {
        TacticGcongrConfig {
            values: std::collections::HashMap::new(),
            read_only: false,
        }
    }
    pub fn set(&mut self, key: &str, value: TacticGcongrConfigValue) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    pub fn get(&self, key: &str) -> Option<&TacticGcongrConfigValue> {
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
        self.set(key, TacticGcongrConfigValue::Bool(v))
    }
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, TacticGcongrConfigValue::Int(v))
    }
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, TacticGcongrConfigValue::Str(v.to_string()))
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
#[allow(dead_code)]
pub struct GcongrExtDiag1000 {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
impl GcongrExtDiag1000 {
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
pub struct GCongrTacExtUtil {
    pub key: String,
    pub data: Vec<i64>,
    pub active: bool,
    pub flags: u32,
}
#[allow(dead_code)]
impl GCongrTacExtUtil {
    pub fn new(key: &str) -> Self {
        GCongrTacExtUtil {
            key: key.to_string(),
            data: Vec::new(),
            active: true,
            flags: 0,
        }
    }
    pub fn push(&mut self, v: i64) {
        self.data.push(v);
    }
    pub fn pop(&mut self) -> Option<i64> {
        self.data.pop()
    }
    pub fn sum(&self) -> i64 {
        self.data.iter().sum()
    }
    pub fn min_val(&self) -> Option<i64> {
        self.data.iter().copied().reduce(i64::min)
    }
    pub fn max_val(&self) -> Option<i64> {
        self.data.iter().copied().reduce(i64::max)
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn clear(&mut self) {
        self.data.clear();
    }
    pub fn set_flag(&mut self, bit: u32) {
        self.flags |= 1 << bit;
    }
    pub fn has_flag(&self, bit: u32) -> bool {
        self.flags & (1 << bit) != 0
    }
    pub fn deactivate(&mut self) {
        self.active = false;
    }
    pub fn activate(&mut self) {
        self.active = true;
    }
}
/// An extended map for GCongrTac keys to values.
#[allow(dead_code)]
pub struct GCongrTacExtMap<V> {
    pub data: std::collections::HashMap<String, V>,
    pub default_key: Option<String>,
}
#[allow(dead_code)]
impl<V: Clone + Default> GCongrTacExtMap<V> {
    pub fn new() -> Self {
        GCongrTacExtMap {
            data: std::collections::HashMap::new(),
            default_key: None,
        }
    }
    pub fn insert(&mut self, key: &str, value: V) {
        self.data.insert(key.to_string(), value);
    }
    pub fn get(&self, key: &str) -> Option<&V> {
        self.data.get(key)
    }
    pub fn get_or_default(&self, key: &str) -> V {
        self.data.get(key).cloned().unwrap_or_default()
    }
    pub fn contains(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }
    pub fn remove(&mut self, key: &str) -> Option<V> {
        self.data.remove(key)
    }
    pub fn size(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn set_default(&mut self, key: &str) {
        self.default_key = Some(key.to_string());
    }
    pub fn keys_sorted(&self) -> Vec<&String> {
        let mut keys: Vec<&String> = self.data.keys().collect();
        keys.sort();
        keys
    }
}
/// A work queue for GCongrTac items.
#[allow(dead_code)]
pub struct GCongrTacWorkQueue {
    pub pending: std::collections::VecDeque<String>,
    pub processed: Vec<String>,
    pub capacity: usize,
}
#[allow(dead_code)]
impl GCongrTacWorkQueue {
    pub fn new(capacity: usize) -> Self {
        GCongrTacWorkQueue {
            pending: std::collections::VecDeque::new(),
            processed: Vec::new(),
            capacity,
        }
    }
    pub fn enqueue(&mut self, item: String) -> bool {
        if self.pending.len() >= self.capacity {
            return false;
        }
        self.pending.push_back(item);
        true
    }
    pub fn dequeue(&mut self) -> Option<String> {
        let item = self.pending.pop_front()?;
        self.processed.push(item.clone());
        Some(item)
    }
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }
    pub fn processed_count(&self) -> usize {
        self.processed.len()
    }
    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }
    pub fn is_full(&self) -> bool {
        self.pending.len() >= self.capacity
    }
    pub fn total_processed(&self) -> usize {
        self.processed.len()
    }
}
#[allow(dead_code)]
pub struct GcongrExtPass1000 {
    pub name: String,
    pub total_runs: usize,
    pub successes: usize,
    pub errors: usize,
    pub enabled: bool,
    pub results: Vec<GcongrExtResult1000>,
}
impl GcongrExtPass1000 {
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
    pub fn run(&mut self, input: &str) -> GcongrExtResult1000 {
        if !self.enabled {
            return GcongrExtResult1000::Skipped;
        }
        self.total_runs += 1;
        let result = if input.is_empty() {
            self.errors += 1;
            GcongrExtResult1000::Err(format!("empty input in pass '{}'", self.name))
        } else {
            self.successes += 1;
            GcongrExtResult1000::Ok(format!(
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
/// A state machine controller for GCongrTac.
#[allow(dead_code)]
pub struct GCongrTacStateMachine {
    pub state: GCongrTacState,
    pub transitions: usize,
    pub history: Vec<String>,
}
#[allow(dead_code)]
impl GCongrTacStateMachine {
    pub fn new() -> Self {
        GCongrTacStateMachine {
            state: GCongrTacState::Initial,
            transitions: 0,
            history: Vec::new(),
        }
    }
    pub fn transition_to(&mut self, new_state: GCongrTacState) -> bool {
        if self.state.is_terminal() {
            return false;
        }
        let desc = format!("{:?} -> {:?}", self.state, new_state);
        self.state = new_state;
        self.transitions += 1;
        self.history.push(desc);
        true
    }
    pub fn start(&mut self) -> bool {
        self.transition_to(GCongrTacState::Running)
    }
    pub fn pause(&mut self) -> bool {
        self.transition_to(GCongrTacState::Paused)
    }
    pub fn complete(&mut self) -> bool {
        self.transition_to(GCongrTacState::Complete)
    }
    pub fn fail(&mut self, msg: &str) -> bool {
        self.transition_to(GCongrTacState::Failed(msg.to_string()))
    }
    pub fn num_transitions(&self) -> usize {
        self.transitions
    }
}
/// Configuration for the `gcongr` tactic.
#[derive(Clone, Debug)]
pub struct GCongrConfig {
    /// Maximum recursion depth.
    pub max_depth: usize,
    /// If set, only consider this specific relation.
    pub relation_filter: Option<GCongrRelation>,
    /// Whether to try reflexivity to close trivial sub-goals.
    pub try_refl: bool,
    /// Whether to use hypotheses from the local context.
    pub use_hyps: bool,
    /// Whether to recursively apply gcongr on sub-goals.
    pub recursive: bool,
}
impl GCongrConfig {
    /// Create a config that only decomposes `<=` goals.
    pub fn le_only() -> Self {
        Self {
            relation_filter: Some(GCongrRelation::Le),
            ..Default::default()
        }
    }
    /// Create a config with recursive decomposition enabled.
    pub fn recursive(max_depth: usize) -> Self {
        Self {
            max_depth,
            recursive: true,
            ..Default::default()
        }
    }
}
#[allow(dead_code)]
pub struct GcongrExtPipeline1000 {
    pub name: String,
    pub passes: Vec<GcongrExtPass1000>,
    pub run_count: usize,
}
impl GcongrExtPipeline1000 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            passes: Vec::new(),
            run_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_pass(&mut self, pass: GcongrExtPass1000) {
        self.passes.push(pass);
    }
    #[allow(dead_code)]
    pub fn run_all(&mut self, input: &str) -> Vec<GcongrExtResult1000> {
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
/// A result type for TacticGcongr analysis.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TacticGcongrResult {
    Ok(String),
    Err(String),
    Partial { done: usize, total: usize },
    Skipped,
}
#[allow(dead_code)]
impl TacticGcongrResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, TacticGcongrResult::Ok(_))
    }
    pub fn is_err(&self) -> bool {
        matches!(self, TacticGcongrResult::Err(_))
    }
    pub fn is_partial(&self) -> bool {
        matches!(self, TacticGcongrResult::Partial { .. })
    }
    pub fn is_skipped(&self) -> bool {
        matches!(self, TacticGcongrResult::Skipped)
    }
    pub fn ok_msg(&self) -> Option<&str> {
        match self {
            TacticGcongrResult::Ok(s) => Some(s),
            _ => None,
        }
    }
    pub fn err_msg(&self) -> Option<&str> {
        match self {
            TacticGcongrResult::Err(s) => Some(s),
            _ => None,
        }
    }
    pub fn progress(&self) -> f64 {
        match self {
            TacticGcongrResult::Ok(_) => 1.0,
            TacticGcongrResult::Err(_) => 0.0,
            TacticGcongrResult::Skipped => 0.0,
            TacticGcongrResult::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
        }
    }
}
/// A sliding window accumulator for GCongrTac.
#[allow(dead_code)]
pub struct GCongrTacWindow {
    pub buffer: std::collections::VecDeque<f64>,
    pub capacity: usize,
    pub running_sum: f64,
}
#[allow(dead_code)]
impl GCongrTacWindow {
    pub fn new(capacity: usize) -> Self {
        GCongrTacWindow {
            buffer: std::collections::VecDeque::new(),
            capacity,
            running_sum: 0.0,
        }
    }
    pub fn push(&mut self, v: f64) {
        if self.buffer.len() >= self.capacity {
            if let Some(old) = self.buffer.pop_front() {
                self.running_sum -= old;
            }
        }
        self.buffer.push_back(v);
        self.running_sum += v;
    }
    pub fn mean(&self) -> f64 {
        if self.buffer.is_empty() {
            0.0
        } else {
            self.running_sum / self.buffer.len() as f64
        }
    }
    pub fn variance(&self) -> f64 {
        if self.buffer.len() < 2 {
            return 0.0;
        }
        let m = self.mean();
        self.buffer.iter().map(|&x| (x - m).powi(2)).sum::<f64>() / self.buffer.len() as f64
    }
    pub fn std_dev(&self) -> f64 {
        self.variance().sqrt()
    }
    pub fn len(&self) -> usize {
        self.buffer.len()
    }
    pub fn is_full(&self) -> bool {
        self.buffer.len() >= self.capacity
    }
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}
/// A pipeline of TacticGcongr analysis passes.
#[allow(dead_code)]
pub struct TacticGcongrPipeline {
    pub passes: Vec<TacticGcongrAnalysisPass>,
    pub name: String,
    pub total_inputs_processed: usize,
}
#[allow(dead_code)]
impl TacticGcongrPipeline {
    pub fn new(name: &str) -> Self {
        TacticGcongrPipeline {
            passes: Vec::new(),
            name: name.to_string(),
            total_inputs_processed: 0,
        }
    }
    pub fn add_pass(&mut self, pass: TacticGcongrAnalysisPass) {
        self.passes.push(pass);
    }
    pub fn run_all(&mut self, input: &str) -> Vec<TacticGcongrResult> {
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
/// Describes how a function is monotone in each argument position.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArgMonotonicity {
    /// The function is monotone (order-preserving) in this argument.
    Monotone,
    /// The function is antitone (order-reversing) in this argument.
    Antitone,
    /// The argument position is "congruent" (must be equal on both sides).
    Congruent,
    /// The argument does not participate (e.g. type arguments).
    Ignore,
}
