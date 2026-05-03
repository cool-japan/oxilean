//! Implementation blocks (part 1)

use super::defs::*;
use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};
use std::fmt::Write as FmtWrite;

impl MatlabOptimizer {
    /// Create a new optimizer.
    #[allow(dead_code)]
    pub fn new() -> Self {
        MatlabOptimizer { rewrites: 0 }
    }
    /// Simplify a MATLAB expression.
    #[allow(dead_code)]
    pub fn simplify(&mut self, expr: MatlabExpr) -> MatlabExpr {
        match expr {
            MatlabExpr::BinaryOp(op, lhs, rhs) => {
                let lhs = self.simplify(*lhs);
                let rhs = self.simplify(*rhs);
                if let (
                    MatlabExpr::Lit(MatlabLiteral::Integer(a)),
                    MatlabExpr::Lit(MatlabLiteral::Integer(b)),
                ) = (&lhs, &rhs)
                {
                    match op.as_str() {
                        "+" => {
                            self.rewrites += 1;
                            return MatlabExpr::Lit(MatlabLiteral::Integer(a + b));
                        }
                        "-" => {
                            self.rewrites += 1;
                            return MatlabExpr::Lit(MatlabLiteral::Integer(a - b));
                        }
                        "*" => {
                            self.rewrites += 1;
                            return MatlabExpr::Lit(MatlabLiteral::Integer(a * b));
                        }
                        _ => {}
                    }
                }
                MatlabExpr::BinaryOp(op, Box::new(lhs), Box::new(rhs))
            }
            MatlabExpr::UnaryOp(op, operand, postfix) => {
                let operand = self.simplify(*operand);
                if op == "-" && !postfix {
                    if let MatlabExpr::Lit(MatlabLiteral::Integer(n)) = &operand {
                        self.rewrites += 1;
                        return MatlabExpr::Lit(MatlabLiteral::Integer(-n));
                    }
                }
                MatlabExpr::UnaryOp(op, Box::new(operand), postfix)
            }
            other => other,
        }
    }
}
impl MatlabConstantFoldingHelper {
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
impl MatlabDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        MatlabDominatorTree {
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
impl MatlabGenConfig {
    /// Create a config for Octave-compatible output.
    #[allow(dead_code)]
    pub fn octave() -> Self {
        MatlabGenConfig {
            octave_compat: true,
            ..Default::default()
        }
    }
    /// Create a config for MATLAB R2022a and newer.
    #[allow(dead_code)]
    pub fn matlab_r2022a() -> Self {
        MatlabGenConfig {
            emit_section_markers: true,
            ..Default::default()
        }
    }
}
impl MatlabDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        MatlabDepGraph {
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
impl MatlabWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        MatlabWorklist {
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
impl MatlabAnnotation {
    /// Create a new annotation with just a summary.
    #[allow(dead_code)]
    pub fn new(summary: impl Into<String>) -> Self {
        MatlabAnnotation {
            summary: summary.into(),
            description: None,
            inputs: Vec::new(),
            outputs: Vec::new(),
            examples: Vec::new(),
            see_also: Vec::new(),
        }
    }
    /// Add an input description.
    #[allow(dead_code)]
    pub fn input(mut self, name: impl Into<String>, desc: impl Into<String>) -> Self {
        self.inputs.push((name.into(), desc.into()));
        self
    }
    /// Add an output description.
    #[allow(dead_code)]
    pub fn output(mut self, name: impl Into<String>, desc: impl Into<String>) -> Self {
        self.outputs.push((name.into(), desc.into()));
        self
    }
    /// Add an example.
    #[allow(dead_code)]
    pub fn example(mut self, code: impl Into<String>) -> Self {
        self.examples.push(code.into());
        self
    }
    /// Emit as MATLAB `%` comment block.
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        let mut lines = vec![format!("%{}", self.summary)];
        if let Some(desc) = &self.description {
            lines.push("%".to_string());
            for line in desc.lines() {
                lines.push(format!("%  {}", line));
            }
        }
        if !self.inputs.is_empty() {
            lines.push("%".to_string());
            lines.push("% Inputs:".to_string());
            for (name, desc) in &self.inputs {
                lines.push(format!("%   {} - {}", name, desc));
            }
        }
        if !self.outputs.is_empty() {
            lines.push("%".to_string());
            lines.push("% Outputs:".to_string());
            for (name, desc) in &self.outputs {
                lines.push(format!("%   {} - {}", name, desc));
            }
        }
        if !self.examples.is_empty() {
            lines.push("%".to_string());
            lines.push("% Examples:".to_string());
            for ex in &self.examples {
                lines.push(format!("%   {}", ex));
            }
        }
        if !self.see_also.is_empty() {
            lines.push("%".to_string());
            lines.push(format!("% See also: {}", self.see_also.join(", ")));
        }
        lines.join("\n")
    }
}
impl MatlabTypeChecker {
    /// Create a new checker.
    #[allow(dead_code)]
    pub fn new() -> Self {
        MatlabTypeChecker {
            env: HashMap::new(),
            errors: Vec::new(),
        }
    }
    /// Declare a variable with a type.
    #[allow(dead_code)]
    pub fn declare(&mut self, name: impl Into<String>, ty: MatlabType) {
        self.env.insert(name.into(), ty);
    }
    /// Infer the type of a MATLAB expression.
    #[allow(dead_code)]
    pub fn infer(&self, expr: &MatlabExpr) -> MatlabType {
        match expr {
            MatlabExpr::Lit(MatlabLiteral::Integer(_)) => MatlabType::Int64,
            MatlabExpr::Lit(MatlabLiteral::Double(_)) => MatlabType::Double,
            MatlabExpr::Lit(MatlabLiteral::Logical(_)) => MatlabType::Logical,
            MatlabExpr::Lit(MatlabLiteral::Char(_)) => MatlabType::Char,
            MatlabExpr::Var(name) => self.env.get(name).cloned().unwrap_or(MatlabType::Any),
            MatlabExpr::BinaryOp(_, lhs, rhs) => {
                let lt = self.infer(lhs);
                let rt = self.infer(rhs);
                self.numeric_promote(lt, rt)
            }
            MatlabExpr::UnaryOp(op, inner, postfix) if (op == "'" || op == ".'") && *postfix => {
                self.infer(inner)
            }
            _ => MatlabType::Any,
        }
    }
    pub(super) fn numeric_promote(&self, a: MatlabType, b: MatlabType) -> MatlabType {
        match (&a, &b) {
            (MatlabType::Double, _) | (_, MatlabType::Double) => MatlabType::Double,
            (MatlabType::Single, _) | (_, MatlabType::Single) => MatlabType::Single,
            (MatlabType::Int64, _) | (_, MatlabType::Int64) => MatlabType::Int64,
            _ => MatlabType::Any,
        }
    }
    /// Check a statement for type consistency.
    #[allow(dead_code)]
    pub fn check_stmt(&mut self, stmt: &MatlabStmt) {
        match stmt {
            MatlabStmt::Assign { lhs, rhs, .. } => {
                let _rhs_ty = self.infer(rhs);
                for name in lhs {
                    if !self.env.contains_key(name) {
                        self.env.insert(name.clone(), MatlabType::Any);
                    }
                }
            }
            _ => {}
        }
    }
    /// Whether any errors were found.
    #[allow(dead_code)]
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}
impl MatlabPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        MatlabPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: MatlabPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), MatlabPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&MatlabPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&MatlabPassStats> {
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
impl MatlabAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        MatlabAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&MatlabCacheEntry> {
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
            MatlabCacheEntry {
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
impl MatlabPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: MatlabPassPhase) -> Self {
        MatlabPassConfig {
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
impl MatlabPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            MatlabPassPhase::Analysis => "analysis",
            MatlabPassPhase::Transformation => "transformation",
            MatlabPassPhase::Verification => "verification",
            MatlabPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(
            self,
            MatlabPassPhase::Transformation | MatlabPassPhase::Cleanup
        )
    }
}
impl MatlabStructLiteral {
    /// Create a new empty struct literal.
    #[allow(dead_code)]
    pub fn new() -> Self {
        MatlabStructLiteral { fields: Vec::new() }
    }
    /// Add a field.
    #[allow(dead_code)]
    pub fn field(mut self, name: impl Into<String>, value: MatlabExpr) -> Self {
        self.fields.push(MatlabStructField::new(name, value));
        self
    }
    /// Emit as a MATLAB `struct(...)` call.
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        if self.fields.is_empty() {
            return "struct()".to_string();
        }
        let args: Vec<_> = self
            .fields
            .iter()
            .flat_map(|f| vec![format!("'{}'", f.name), format!("{{{}}}", f.value)])
            .collect();
        format!("struct({})", args.join(", "))
    }
}
impl MatlabPassStats {
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
