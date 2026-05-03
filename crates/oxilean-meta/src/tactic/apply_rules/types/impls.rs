//! Impl blocks for apply_rules

use std::collections::HashMap;

use super::defs::*;

#[allow(dead_code)]
impl ApplyRulesUtil1 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        ApplyRulesUtil1 {
            id,
            name: name.to_string(),
            value,
            enabled: true,
            tags: Vec::new(),
        }
    }
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn is_active(&self) -> bool {
        self.enabled
    }
    pub fn score(&self) -> i64 {
        if self.enabled {
            self.value
        } else {
            0
        }
    }
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}

#[allow(dead_code)]
impl TacticApplyRulesConfigValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            TacticApplyRulesConfigValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            TacticApplyRulesConfigValue::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            TacticApplyRulesConfigValue::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            TacticApplyRulesConfigValue::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&[String]> {
        match self {
            TacticApplyRulesConfigValue::List(v) => Some(v),
            _ => None,
        }
    }
    pub fn type_name(&self) -> &'static str {
        match self {
            TacticApplyRulesConfigValue::Bool(_) => "bool",
            TacticApplyRulesConfigValue::Int(_) => "int",
            TacticApplyRulesConfigValue::Float(_) => "float",
            TacticApplyRulesConfigValue::Str(_) => "str",
            TacticApplyRulesConfigValue::List(_) => "list",
        }
    }
}

#[allow(dead_code)]
impl ApplyRulesRegistry {
    pub fn new(capacity: usize) -> Self {
        ApplyRulesRegistry {
            entries: Vec::new(),
            capacity,
        }
    }
    pub fn register(&mut self, entry: ApplyRulesUtil0) -> bool {
        if self.entries.len() >= self.capacity {
            return false;
        }
        self.entries.push(entry);
        true
    }
    pub fn lookup(&self, id: usize) -> Option<&ApplyRulesUtil0> {
        self.entries.iter().find(|e| e.id == id)
    }
    pub fn remove(&mut self, id: usize) -> bool {
        let before = self.entries.len();
        self.entries.retain(|e| e.id != id);
        self.entries.len() < before
    }
    pub fn active_entries(&self) -> Vec<&ApplyRulesUtil0> {
        self.entries.iter().filter(|e| e.is_active()).collect()
    }
    pub fn total_score(&self) -> i64 {
        self.entries.iter().map(|e| e.score()).sum()
    }
    pub fn count(&self) -> usize {
        self.entries.len()
    }
    pub fn is_full(&self) -> bool {
        self.entries.len() >= self.capacity
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

#[allow(dead_code)]
impl ApplyRulesUtil5 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        ApplyRulesUtil5 {
            id,
            name: name.to_string(),
            value,
            enabled: true,
            tags: Vec::new(),
        }
    }
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn is_active(&self) -> bool {
        self.enabled
    }
    pub fn score(&self) -> i64 {
        if self.enabled {
            self.value
        } else {
            0
        }
    }
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}

#[allow(dead_code)]
impl TacticApplyRulesDiff {
    pub fn new() -> Self {
        TacticApplyRulesDiff {
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

#[allow(dead_code)]
impl ApplyRulesStats {
    pub fn new() -> Self {
        ApplyRulesStats::default()
    }
    pub fn record_success(&mut self, time_ns: u64) {
        self.total_ops += 1;
        self.successful_ops += 1;
        self.total_time_ns += time_ns;
        if time_ns > self.max_time_ns {
            self.max_time_ns = time_ns;
        }
    }
    pub fn record_failure(&mut self) {
        self.total_ops += 1;
        self.failed_ops += 1;
    }
    pub fn success_rate(&self) -> f64 {
        if self.total_ops == 0 {
            0.0
        } else {
            self.successful_ops as f64 / self.total_ops as f64
        }
    }
    pub fn avg_time_ns(&self) -> f64 {
        if self.successful_ops == 0 {
            0.0
        } else {
            self.total_time_ns as f64 / self.successful_ops as f64
        }
    }
    pub fn merge(&mut self, other: &Self) {
        self.total_ops += other.total_ops;
        self.successful_ops += other.successful_ops;
        self.failed_ops += other.failed_ops;
        self.total_time_ns += other.total_time_ns;
        if other.max_time_ns > self.max_time_ns {
            self.max_time_ns = other.max_time_ns;
        }
    }
}

impl ApplyRulesConfig {
    /// Create a configuration for safe backward reasoning only.
    pub fn safe_backward(max_depth: usize) -> Self {
        Self {
            max_depth,
            safe_only: true,
            ..Default::default()
        }
    }
    /// Create a configuration for forward reasoning.
    pub fn forward(max_depth: usize) -> Self {
        Self {
            max_depth,
            mode: ReasoningMode::Forward,
            ..Default::default()
        }
    }
    /// Create a configuration for both reasoning modes.
    pub fn both(max_depth: usize) -> Self {
        Self {
            max_depth,
            mode: ReasoningMode::Both,
            ..Default::default()
        }
    }
    /// Add a tag filter.
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tag_filter
            .get_or_insert_with(Vec::new)
            .push(tag.into());
        self
    }
}

impl RuleApplication {
    /// Return `true` if this application closed the goal.
    pub fn closed_goal(&self) -> bool {
        self.subgoals_after.is_empty()
    }
}

impl RuleSet {
    /// Create an empty rule set.
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            by_tag: HashMap::new(),
        }
    }
    /// Create a rule set from a list of rule names with default priorities.
    pub fn from_names(names: &[&str]) -> Self {
        let mut set = Self::new();
        for (i, name) in names.iter().enumerate() {
            set.add(RuleEntry::new(*name, (i as u32) * 100 + 1000));
        }
        set
    }
    /// Create a rule set with standard order/logic rules.
    pub fn with_defaults() -> Self {
        let mut set = Self::new();
        set.add(
            RuleEntry::new("Nat.le_refl", 100)
                .with_shape(RuleShape::Closing)
                .with_conclusion("<=")
                .with_tag("order"),
        );
        set.add(
            RuleEntry::new("Nat.le_trans", 500)
                .with_shape(RuleShape::MultiSubgoal(2))
                .with_params(3)
                .with_conclusion("<=")
                .with_tag("order")
                .unsafe_rule(),
        );
        set.add(
            RuleEntry::new("Nat.le_succ", 200)
                .with_shape(RuleShape::Closing)
                .with_conclusion("<=")
                .with_tag("order"),
        );
        set.add(
            RuleEntry::new("Nat.zero_le", 150)
                .with_shape(RuleShape::Closing)
                .with_conclusion("<=")
                .with_tag("order"),
        );
        set.add(
            RuleEntry::new("And.intro", 300)
                .with_shape(RuleShape::MultiSubgoal(2))
                .with_params(2)
                .with_conclusion("And")
                .with_tag("logic"),
        );
        set.add(
            RuleEntry::new("Or.inl", 400)
                .with_shape(RuleShape::SingleSubgoal)
                .with_params(1)
                .with_conclusion("Or")
                .with_tag("logic")
                .unsafe_rule(),
        );
        set.add(
            RuleEntry::new("Or.inr", 400)
                .with_shape(RuleShape::SingleSubgoal)
                .with_params(1)
                .with_conclusion("Or")
                .with_tag("logic")
                .unsafe_rule(),
        );
        set.add(
            RuleEntry::new("Iff.intro", 350)
                .with_shape(RuleShape::MultiSubgoal(2))
                .with_params(2)
                .with_conclusion("Iff")
                .with_tag("logic"),
        );
        set.add(
            RuleEntry::new("Eq.refl", 50)
                .with_shape(RuleShape::Closing)
                .with_conclusion("=")
                .with_tag("eq"),
        );
        set.add(
            RuleEntry::new("Nat.add_le_add", 250)
                .with_shape(RuleShape::MultiSubgoal(2))
                .with_params(4)
                .with_conclusion("<=")
                .with_tag("order"),
        );
        set
    }
    /// Add a rule to the set.
    pub fn add(&mut self, rule: RuleEntry) {
        let idx = self.rules.len();
        if let Some(ref tag) = rule.tag {
            self.by_tag.entry(tag.clone()).or_default().push(idx);
        }
        self.rules.push(rule);
        self.rules.sort_by_key(|r| r.priority);
        self.rebuild_tag_index();
    }
    /// Rebuild the tag index after sorting.
    fn rebuild_tag_index(&mut self) {
        self.by_tag.clear();
        for (idx, rule) in self.rules.iter().enumerate() {
            if let Some(ref tag) = rule.tag {
                self.by_tag.entry(tag.clone()).or_default().push(idx);
            }
        }
    }
    /// Return all rules, sorted by priority.
    pub fn all_rules(&self) -> &[RuleEntry] {
        &self.rules
    }
    /// Return rules matching a given tag.
    pub fn rules_by_tag(&self, tag: &str) -> Vec<&RuleEntry> {
        self.by_tag
            .get(tag)
            .map(|indices| indices.iter().map(|&i| &self.rules[i]).collect())
            .unwrap_or_default()
    }
    /// Return the number of rules.
    pub fn len(&self) -> usize {
        self.rules.len()
    }
    /// Return `true` if the set is empty.
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
    /// Return rules that match a given goal string.
    pub fn matching_rules(&self, goal: &str) -> Vec<&RuleEntry> {
        self.rules.iter().filter(|r| r.matches_goal(goal)).collect()
    }
    /// Return only safe rules.
    pub fn safe_rules(&self) -> Vec<&RuleEntry> {
        self.rules.iter().filter(|r| r.safe).collect()
    }
    /// Return all distinct tags.
    pub fn tags(&self) -> Vec<&str> {
        self.by_tag.keys().map(|s| s.as_str()).collect()
    }
}

impl ApplyRulesResult {
    /// Create a success result.
    pub fn success(remaining: Vec<String>, trace: Vec<RuleApplication>) -> Self {
        let n = trace.len();
        let msg = if remaining.is_empty() {
            format!("apply_rules closed the goal with {} application(s)", n)
        } else {
            format!(
                "apply_rules made {} application(s), {} goal(s) remaining",
                n,
                remaining.len()
            )
        };
        Self {
            success: true,
            remaining_goals: remaining,
            trace,
            message: msg,
            num_applications: n,
        }
    }
    /// Create a failure result.
    pub fn failure(message: impl Into<String>) -> Self {
        Self {
            success: false,
            remaining_goals: Vec::new(),
            trace: Vec::new(),
            message: message.into(),
            num_applications: 0,
        }
    }
    /// Return `true` if the goal was completely closed.
    pub fn is_closed(&self) -> bool {
        self.success && self.remaining_goals.is_empty()
    }
    /// Return the names of all rules that were applied.
    pub fn applied_rule_names(&self) -> Vec<&str> {
        self.trace.iter().map(|a| a.rule_name.as_str()).collect()
    }
    /// Return the number of distinct rules applied.
    pub fn distinct_rules_applied(&self) -> usize {
        let mut names: Vec<&str> = self.applied_rule_names();
        names.sort();
        names.dedup();
        names.len()
    }
}

#[allow(dead_code)]
impl ApplyRulesUtil11 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        ApplyRulesUtil11 {
            id,
            name: name.to_string(),
            value,
            enabled: true,
            tags: Vec::new(),
        }
    }
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn is_active(&self) -> bool {
        self.enabled
    }
    pub fn score(&self) -> i64 {
        if self.enabled {
            self.value
        } else {
            0
        }
    }
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}

#[allow(dead_code)]
impl ApplyRulesUtil0 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        ApplyRulesUtil0 {
            id,
            name: name.to_string(),
            value,
            enabled: true,
            tags: Vec::new(),
        }
    }
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn is_active(&self) -> bool {
        self.enabled
    }
    pub fn score(&self) -> i64 {
        if self.enabled {
            self.value
        } else {
            0
        }
    }
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}

impl ApplyRulesTactic {
    /// Create a new `ApplyRulesTactic` with default configuration and rules.
    pub fn new() -> Self {
        Self {
            config: ApplyRulesConfig::default(),
            rules: RuleSet::with_defaults(),
        }
    }
    /// Create with a custom config and default rules.
    pub fn with_config(config: ApplyRulesConfig) -> Self {
        Self {
            config,
            rules: RuleSet::with_defaults(),
        }
    }
    /// Create with a custom rule set and default config.
    pub fn with_rules(rules: RuleSet) -> Self {
        Self {
            config: ApplyRulesConfig::default(),
            rules,
        }
    }
    /// Create with both custom config and rules.
    pub fn with_config_and_rules(config: ApplyRulesConfig, rules: RuleSet) -> Self {
        Self { config, rules }
    }
    /// Apply rules to a goal string (backward reasoning only).
    pub fn apply(&self, goal: &str) -> ApplyRulesResult {
        let mut state = SubgoalState::new(goal.to_string());
        self.solve_backward(&mut state);
        ApplyRulesResult::success(state.goals, state.trace)
    }
    /// Apply rules to a goal string with hypotheses (supports forward reasoning).
    pub fn apply_with_hyps(&self, goal: &str, hyps: &[&str]) -> ApplyRulesResult {
        let hyp_strings: Vec<String> = hyps.iter().map(|h| h.to_string()).collect();
        let mut state = SubgoalState::with_hyps(goal.to_string(), hyp_strings);
        if self.config.mode.allows_forward() {
            self.solve_forward(&mut state);
        }
        if self.config.mode.allows_backward() {
            self.solve_backward(&mut state);
        }
        ApplyRulesResult::success(state.goals, state.trace)
    }
    /// Attempt backward reasoning: try each rule against the first open goal.
    fn solve_backward(&self, state: &mut SubgoalState) {
        let max_iterations = self.config.max_depth * 3;
        let mut iterations = 0;
        while !state.is_complete() && iterations < max_iterations {
            iterations += 1;
            let current_goal = match state.goals.first() {
                Some(g) => g.clone(),
                None => break,
            };
            let applicable = self.find_applicable_rules(&current_goal);
            if applicable.is_empty() {
                break;
            }
            let mut applied = false;
            for rule in &applicable {
                if self.config.safe_only && !rule.safe {
                    continue;
                }
                if let Some(ref tags) = self.config.tag_filter {
                    if let Some(ref rule_tag) = rule.tag {
                        if !tags.iter().any(|t| t == rule_tag) {
                            continue;
                        }
                    } else {
                        continue;
                    }
                }
                let subgoals = self.simulate_apply(rule, &current_goal);
                let app = RuleApplication {
                    rule_name: rule.name.clone(),
                    goal_before: current_goal.clone(),
                    subgoals_after: subgoals.clone(),
                    depth: state.depth,
                    forward: false,
                };
                state.goals.remove(0);
                for sg in subgoals.into_iter().rev() {
                    state.goals.insert(0, sg);
                }
                state.record(app);
                state.depth += 1;
                applied = true;
                if !self.config.exhaustive {
                    return;
                }
                break;
            }
            if !applied {
                break;
            }
        }
    }
    /// Attempt forward reasoning: try each rule against hypotheses.
    fn solve_forward(&self, state: &mut SubgoalState) {
        if state.hypotheses.is_empty() {
            return;
        }
        for hyp in state.hypotheses.clone() {
            for rule in self.rules.all_rules() {
                if self.config.safe_only && !rule.safe {
                    continue;
                }
                if !rule.matches_hypothesis(&hyp) {
                    continue;
                }
                let new_fact = self.simulate_forward(rule, &hyp);
                if let Some(fact) = new_fact {
                    let app = RuleApplication {
                        rule_name: rule.name.clone(),
                        goal_before: hyp.clone(),
                        subgoals_after: vec![fact.clone()],
                        depth: state.depth,
                        forward: true,
                    };
                    state.record(app);
                    state.hypotheses.push(fact.clone());
                    state.goals.retain(|g| g != &fact);
                }
            }
        }
    }
    /// Find rules applicable to a goal, sorted by priority.
    fn find_applicable_rules<'a>(&'a self, goal: &str) -> Vec<&'a RuleEntry> {
        self.rules.matching_rules(goal)
    }
    /// Simulate applying a rule backward, returning sub-goals.
    ///
    /// This is a simplified simulation based on the rule shape.
    fn simulate_apply(&self, rule: &RuleEntry, _goal: &str) -> Vec<String> {
        match &rule.shape {
            RuleShape::Closing => vec![],
            RuleShape::SingleSubgoal => vec![format!("subgoal from {}", rule.name)],
            RuleShape::MultiSubgoal(n) => (0..*n)
                .map(|i| format!("subgoal {} from {}", i + 1, rule.name))
                .collect(),
            RuleShape::Unknown => vec![],
        }
    }
    /// Simulate applying a rule forward, returning a derived fact.
    fn simulate_forward(&self, rule: &RuleEntry, hyp: &str) -> Option<String> {
        if rule.matches_hypothesis(hyp) {
            Some(format!("derived from {} via {}", hyp, rule.name))
        } else {
            None
        }
    }
    /// Get a reference to the rule set.
    pub fn rules(&self) -> &RuleSet {
        &self.rules
    }
    /// Get a mutable reference to the rule set.
    pub fn rules_mut(&mut self) -> &mut RuleSet {
        &mut self.rules
    }
}

#[allow(dead_code)]
impl ApplyRulesUtil6 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        ApplyRulesUtil6 {
            id,
            name: name.to_string(),
            value,
            enabled: true,
            tags: Vec::new(),
        }
    }
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn is_active(&self) -> bool {
        self.enabled
    }
    pub fn score(&self) -> i64 {
        if self.enabled {
            self.value
        } else {
            0
        }
    }
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}

#[allow(dead_code)]
impl TacticApplyRulesResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, TacticApplyRulesResult::Ok(_))
    }
    pub fn is_err(&self) -> bool {
        matches!(self, TacticApplyRulesResult::Err(_))
    }
    pub fn is_partial(&self) -> bool {
        matches!(self, TacticApplyRulesResult::Partial { .. })
    }
    pub fn is_skipped(&self) -> bool {
        matches!(self, TacticApplyRulesResult::Skipped)
    }
    pub fn ok_msg(&self) -> Option<&str> {
        match self {
            TacticApplyRulesResult::Ok(s) => Some(s),
            _ => None,
        }
    }
    pub fn err_msg(&self) -> Option<&str> {
        match self {
            TacticApplyRulesResult::Err(s) => Some(s),
            _ => None,
        }
    }
    pub fn progress(&self) -> f64 {
        match self {
            TacticApplyRulesResult::Ok(_) => 1.0,
            TacticApplyRulesResult::Err(_) => 0.0,
            TacticApplyRulesResult::Skipped => 0.0,
            TacticApplyRulesResult::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
        }
    }
}

#[allow(dead_code)]
impl ApplyRulesUtil8 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        ApplyRulesUtil8 {
            id,
            name: name.to_string(),
            value,
            enabled: true,
            tags: Vec::new(),
        }
    }
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn is_active(&self) -> bool {
        self.enabled
    }
    pub fn score(&self) -> i64 {
        if self.enabled {
            self.value
        } else {
            0
        }
    }
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}

#[allow(dead_code)]
impl TacticApplyRulesPipeline {
    pub fn new(name: &str) -> Self {
        TacticApplyRulesPipeline {
            passes: Vec::new(),
            name: name.to_string(),
            total_inputs_processed: 0,
        }
    }
    pub fn add_pass(&mut self, pass: TacticApplyRulesAnalysisPass) {
        self.passes.push(pass);
    }
    pub fn run_all(&mut self, input: &str) -> Vec<TacticApplyRulesResult> {
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
impl TacticApplyRulesConfig {
    pub fn new() -> Self {
        TacticApplyRulesConfig {
            values: std::collections::HashMap::new(),
            read_only: false,
        }
    }
    pub fn set(&mut self, key: &str, value: TacticApplyRulesConfigValue) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    pub fn get(&self, key: &str) -> Option<&TacticApplyRulesConfigValue> {
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
        self.set(key, TacticApplyRulesConfigValue::Bool(v))
    }
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, TacticApplyRulesConfigValue::Int(v))
    }
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, TacticApplyRulesConfigValue::Str(v.to_string()))
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
impl TacticApplyRulesDiagnostics {
    pub fn new(max_errors: usize) -> Self {
        TacticApplyRulesDiagnostics {
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

impl ApplyRulesExtPass3400 {
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
    pub fn run(&mut self, input: &str) -> ApplyRulesExtResult3400 {
        if !self.enabled {
            return ApplyRulesExtResult3400::Skipped;
        }
        self.total_runs += 1;
        let result = if input.is_empty() {
            self.errors += 1;
            ApplyRulesExtResult3400::Err(format!("empty input in pass '{}'", self.name))
        } else {
            self.successes += 1;
            ApplyRulesExtResult3400::Ok(format!(
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

impl RuleEntry {
    /// Create a basic rule with a name and priority.
    pub fn new(name: impl Into<String>, priority: u32) -> Self {
        Self {
            name: name.into(),
            tag: None,
            priority,
            shape: RuleShape::Unknown,
            num_params: 0,
            safe: true,
            conclusion_pattern: None,
            hypothesis_patterns: Vec::new(),
        }
    }
    /// Set the tag.
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tag = Some(tag.into());
        self
    }
    /// Set the shape.
    pub fn with_shape(mut self, shape: RuleShape) -> Self {
        self.shape = shape;
        self
    }
    /// Set the number of parameters.
    pub fn with_params(mut self, n: usize) -> Self {
        self.num_params = n;
        self
    }
    /// Mark as unsafe (may produce unprovable sub-goals).
    pub fn unsafe_rule(mut self) -> Self {
        self.safe = false;
        self
    }
    /// Set the conclusion pattern.
    pub fn with_conclusion(mut self, pattern: impl Into<String>) -> Self {
        self.conclusion_pattern = Some(pattern.into());
        self
    }
    /// Add a hypothesis pattern for forward reasoning.
    pub fn with_hyp_pattern(mut self, pattern: impl Into<String>) -> Self {
        self.hypothesis_patterns.push(pattern.into());
        self
    }
    /// Return `true` if this rule can potentially close a goal.
    pub fn can_close(&self) -> bool {
        matches!(self.shape, RuleShape::Closing)
    }
    /// Return `true` if this rule matches a given goal string (simple substring match).
    pub fn matches_goal(&self, goal: &str) -> bool {
        match &self.conclusion_pattern {
            Some(pat) => goal.contains(pat.as_str()),
            None => true,
        }
    }
    /// Return `true` if this rule matches a given hypothesis (simple substring match).
    pub fn matches_hypothesis(&self, hyp: &str) -> bool {
        if self.hypothesis_patterns.is_empty() {
            return false;
        }
        self.hypothesis_patterns
            .iter()
            .any(|pat| hyp.contains(pat.as_str()))
    }
}

#[allow(dead_code)]
impl ApplyRulesUtil2 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        ApplyRulesUtil2 {
            id,
            name: name.to_string(),
            value,
            enabled: true,
            tags: Vec::new(),
        }
    }
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn is_active(&self) -> bool {
        self.enabled
    }
    pub fn score(&self) -> i64 {
        if self.enabled {
            self.value
        } else {
            0
        }
    }
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}

#[allow(dead_code)]
impl ApplyRulesUtil13 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        ApplyRulesUtil13 {
            id,
            name: name.to_string(),
            value,
            enabled: true,
            tags: Vec::new(),
        }
    }
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn is_active(&self) -> bool {
        self.enabled
    }
    pub fn score(&self) -> i64 {
        if self.enabled {
            self.value
        } else {
            0
        }
    }
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}

impl ApplyRulesExtDiff3400 {
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

#[allow(dead_code)]
impl ApplyRulesUtil4 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        ApplyRulesUtil4 {
            id,
            name: name.to_string(),
            value,
            enabled: true,
            tags: Vec::new(),
        }
    }
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn is_active(&self) -> bool {
        self.enabled
    }
    pub fn score(&self) -> i64 {
        if self.enabled {
            self.value
        } else {
            0
        }
    }
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}

#[allow(dead_code)]
impl ApplyRulesUtil10 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        ApplyRulesUtil10 {
            id,
            name: name.to_string(),
            value,
            enabled: true,
            tags: Vec::new(),
        }
    }
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn is_active(&self) -> bool {
        self.enabled
    }
    pub fn score(&self) -> i64 {
        if self.enabled {
            self.value
        } else {
            0
        }
    }
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}

#[allow(dead_code)]
impl ApplyRulesLogger {
    pub fn new(max_entries: usize) -> Self {
        ApplyRulesLogger {
            entries: Vec::new(),
            max_entries,
            verbose: false,
        }
    }
    pub fn log(&mut self, msg: &str) {
        if self.entries.len() < self.max_entries {
            self.entries.push(msg.to_string());
        }
    }
    pub fn verbose(&mut self, msg: &str) {
        if self.verbose {
            self.log(msg);
        }
    }
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    pub fn count(&self) -> usize {
        self.entries.len()
    }
    pub fn last(&self) -> Option<&str> {
        self.entries.last().map(|s| s.as_str())
    }
    pub fn enable_verbose(&mut self) {
        self.verbose = true;
    }
    pub fn disable_verbose(&mut self) {
        self.verbose = false;
    }
}

impl ApplyRulesExtResult3400 {
    #[allow(dead_code)]
    pub fn is_ok(&self) -> bool {
        matches!(self, ApplyRulesExtResult3400::Ok(_))
    }
    #[allow(dead_code)]
    pub fn is_err(&self) -> bool {
        matches!(self, ApplyRulesExtResult3400::Err(_))
    }
    #[allow(dead_code)]
    pub fn is_partial(&self) -> bool {
        matches!(self, ApplyRulesExtResult3400::Partial { .. })
    }
    #[allow(dead_code)]
    pub fn is_skipped(&self) -> bool {
        matches!(self, ApplyRulesExtResult3400::Skipped)
    }
    #[allow(dead_code)]
    pub fn ok_msg(&self) -> Option<&str> {
        if let ApplyRulesExtResult3400::Ok(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn err_msg(&self) -> Option<&str> {
        if let ApplyRulesExtResult3400::Err(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn progress(&self) -> f64 {
        match self {
            ApplyRulesExtResult3400::Ok(_) => 1.0,
            ApplyRulesExtResult3400::Err(_) => 0.0,
            ApplyRulesExtResult3400::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
            ApplyRulesExtResult3400::Skipped => 0.5,
        }
    }
}

#[allow(dead_code)]
impl ApplyRulesPriorityQueue {
    pub fn new() -> Self {
        ApplyRulesPriorityQueue { items: Vec::new() }
    }
    pub fn push(&mut self, item: ApplyRulesUtil0, priority: i64) {
        self.items.push((item, priority));
        self.items.sort_by_key(|(_, p)| -p);
    }
    pub fn pop(&mut self) -> Option<(ApplyRulesUtil0, i64)> {
        if self.items.is_empty() {
            None
        } else {
            Some(self.items.remove(0))
        }
    }
    pub fn peek(&self) -> Option<&(ApplyRulesUtil0, i64)> {
        self.items.first()
    }
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    pub fn len(&self) -> usize {
        self.items.len()
    }
}

impl ApplyRulesExtPipeline3400 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            passes: Vec::new(),
            run_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_pass(&mut self, pass: ApplyRulesExtPass3400) {
        self.passes.push(pass);
    }
    #[allow(dead_code)]
    pub fn run_all(&mut self, input: &str) -> Vec<ApplyRulesExtResult3400> {
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

#[allow(dead_code)]
impl ApplyRulesCache {
    pub fn new() -> Self {
        ApplyRulesCache {
            data: std::collections::HashMap::new(),
            hits: 0,
            misses: 0,
        }
    }
    pub fn get(&mut self, key: &str) -> Option<i64> {
        if let Some(&v) = self.data.get(key) {
            self.hits += 1;
            Some(v)
        } else {
            self.misses += 1;
            None
        }
    }
    pub fn insert(&mut self, key: &str, value: i64) {
        self.data.insert(key.to_string(), value);
    }
    pub fn hit_rate(&self) -> f64 {
        let t = self.hits + self.misses;
        if t == 0 {
            0.0
        } else {
            self.hits as f64 / t as f64
        }
    }
    pub fn size(&self) -> usize {
        self.data.len()
    }
    pub fn clear(&mut self) {
        self.data.clear();
        self.hits = 0;
        self.misses = 0;
    }
}

#[allow(dead_code)]
impl ApplyRulesUtil3 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        ApplyRulesUtil3 {
            id,
            name: name.to_string(),
            value,
            enabled: true,
            tags: Vec::new(),
        }
    }
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn is_active(&self) -> bool {
        self.enabled
    }
    pub fn score(&self) -> i64 {
        if self.enabled {
            self.value
        } else {
            0
        }
    }
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}

#[allow(dead_code)]
impl ApplyRulesUtil7 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        ApplyRulesUtil7 {
            id,
            name: name.to_string(),
            value,
            enabled: true,
            tags: Vec::new(),
        }
    }
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn is_active(&self) -> bool {
        self.enabled
    }
    pub fn score(&self) -> i64 {
        if self.enabled {
            self.value
        } else {
            0
        }
    }
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}

#[allow(dead_code)]
impl ApplyRulesUtil14 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        ApplyRulesUtil14 {
            id,
            name: name.to_string(),
            value,
            enabled: true,
            tags: Vec::new(),
        }
    }
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn is_active(&self) -> bool {
        self.enabled
    }
    pub fn score(&self) -> i64 {
        if self.enabled {
            self.value
        } else {
            0
        }
    }
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}

#[allow(dead_code)]
impl TacticApplyRulesAnalysisPass {
    pub fn new(name: &str) -> Self {
        TacticApplyRulesAnalysisPass {
            name: name.to_string(),
            enabled: true,
            results: Vec::new(),
            total_runs: 0,
        }
    }
    pub fn run(&mut self, input: &str) -> TacticApplyRulesResult {
        self.total_runs += 1;
        let result = if input.is_empty() {
            TacticApplyRulesResult::Err("empty input".to_string())
        } else {
            TacticApplyRulesResult::Ok(format!("processed: {}", input))
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

impl SubgoalState {
    /// Create a new state with a single goal.
    fn new(goal: String) -> Self {
        Self {
            goals: vec![goal],
            hypotheses: Vec::new(),
            depth: 0,
            trace: Vec::new(),
        }
    }
    /// Create a new state with a goal and hypotheses.
    fn with_hyps(goal: String, hyps: Vec<String>) -> Self {
        Self {
            goals: vec![goal],
            hypotheses: hyps,
            depth: 0,
            trace: Vec::new(),
        }
    }
    /// Return `true` if all goals are solved.
    fn is_complete(&self) -> bool {
        self.goals.is_empty()
    }
    /// Add a trace entry.
    fn record(&mut self, app: RuleApplication) {
        self.trace.push(app);
    }
}

#[allow(dead_code)]
impl ApplyRulesUtil12 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        ApplyRulesUtil12 {
            id,
            name: name.to_string(),
            value,
            enabled: true,
            tags: Vec::new(),
        }
    }
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn is_active(&self) -> bool {
        self.enabled
    }
    pub fn score(&self) -> i64 {
        if self.enabled {
            self.value
        } else {
            0
        }
    }
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}

impl ApplyRulesExtConfig3400 {
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
    pub fn set(&mut self, key: &str, value: ApplyRulesExtConfigVal3400) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&ApplyRulesExtConfigVal3400> {
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
        self.set(key, ApplyRulesExtConfigVal3400::Bool(v))
    }
    #[allow(dead_code)]
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, ApplyRulesExtConfigVal3400::Int(v))
    }
    #[allow(dead_code)]
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, ApplyRulesExtConfigVal3400::Str(v.to_string()))
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

impl ApplyRulesExtDiag3400 {
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

impl ReasoningMode {
    /// Return `true` if backward reasoning is enabled.
    pub fn allows_backward(&self) -> bool {
        matches!(self, ReasoningMode::Backward | ReasoningMode::Both)
    }
    /// Return `true` if forward reasoning is enabled.
    pub fn allows_forward(&self) -> bool {
        matches!(self, ReasoningMode::Forward | ReasoningMode::Both)
    }
}

impl ApplyRulesExtConfigVal3400 {
    #[allow(dead_code)]
    pub fn as_bool(&self) -> Option<bool> {
        if let ApplyRulesExtConfigVal3400::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_int(&self) -> Option<i64> {
        if let ApplyRulesExtConfigVal3400::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_float(&self) -> Option<f64> {
        if let ApplyRulesExtConfigVal3400::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_str(&self) -> Option<&str> {
        if let ApplyRulesExtConfigVal3400::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_list(&self) -> Option<&[String]> {
        if let ApplyRulesExtConfigVal3400::List(l) = self {
            Some(l)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn type_name(&self) -> &'static str {
        match self {
            ApplyRulesExtConfigVal3400::Bool(_) => "bool",
            ApplyRulesExtConfigVal3400::Int(_) => "int",
            ApplyRulesExtConfigVal3400::Float(_) => "float",
            ApplyRulesExtConfigVal3400::Str(_) => "str",
            ApplyRulesExtConfigVal3400::List(_) => "list",
        }
    }
}

#[allow(dead_code)]
impl ApplyRulesUtil9 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        ApplyRulesUtil9 {
            id,
            name: name.to_string(),
            value,
            enabled: true,
            tags: Vec::new(),
        }
    }
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn is_active(&self) -> bool {
        self.enabled
    }
    pub fn score(&self) -> i64 {
        if self.enabled {
            self.value
        } else {
            0
        }
    }
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}
