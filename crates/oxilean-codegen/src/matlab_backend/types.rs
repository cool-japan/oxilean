//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};
use std::fmt::Write as FmtWrite;

/// A simple constant-folding optimizer for MATLAB expressions.
#[allow(dead_code)]
pub struct MatlabOptimizer {
    /// Number of rewrites performed.
    pub rewrites: usize,
}
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
/// MATLAB classdef property.
#[derive(Debug, Clone, PartialEq)]
pub struct MatlabProperty {
    pub name: String,
    pub ty: Option<MatlabType>,
    pub default: Option<MatlabExpr>,
    pub access: PropAccess,
    pub is_constant: bool,
    pub is_dependent: bool,
}
/// MATLAB type representation.
#[derive(Debug, Clone, PartialEq)]
pub enum MatlabType {
    /// `double` — 64-bit float (default numeric type)
    Double,
    /// `single` — 32-bit float
    Single,
    /// `int8`
    Int8,
    /// `int16`
    Int16,
    /// `int32`
    Int32,
    /// `int64`
    Int64,
    /// `uint8`
    Uint8,
    /// `uint16`
    Uint16,
    /// `uint32`
    Uint32,
    /// `uint64`
    Uint64,
    /// `logical`
    Logical,
    /// `char` — character array / string (pre-R2016b)
    Char,
    /// `string` — string array (R2016b+)
    StringArray,
    /// `cell` — cell array
    Cell,
    /// Named struct type
    StructType(String),
    /// `function_handle` — `@func`
    FunctionHandle,
    /// `sparse` — sparse matrix
    Sparse,
    /// N-D array of a base type
    Array(Box<MatlabType>, Vec<Option<usize>>),
    /// Class instance
    Class(String),
    /// Any / unspecified
    Any,
}
/// Property access level.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropAccess {
    Public,
    Protected,
    Private,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MatlabCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
#[allow(dead_code)]
pub struct MatlabConstantFoldingHelper;
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
/// MATLAB expression.
#[derive(Debug, Clone, PartialEq)]
pub enum MatlabExpr {
    /// Literal value
    Lit(MatlabLiteral),
    /// Variable reference: `x`
    Var(String),
    /// Matrix literal: `[1 2; 3 4]`
    MatrixLit(Vec<Vec<MatlabExpr>>),
    /// Cell array literal: `{1, 'a', true}`
    CellLit(Vec<Vec<MatlabExpr>>),
    /// Colon range: `start:step:end` or `start:end`
    ColonRange {
        start: Box<MatlabExpr>,
        step: Option<Box<MatlabExpr>>,
        end: Box<MatlabExpr>,
    },
    /// Function call: `f(a, b)`
    Call(Box<MatlabExpr>, Vec<MatlabExpr>),
    /// Indexing: `A(i, j)` or `A{i}` (cell)
    Index {
        obj: Box<MatlabExpr>,
        indices: Vec<MatlabExpr>,
        cell_index: bool,
    },
    /// Struct field access: `s.field`
    FieldAccess(Box<MatlabExpr>, String),
    /// Binary operator: `a + b`, `a .* b`, `a & b`
    BinaryOp(String, Box<MatlabExpr>, Box<MatlabExpr>),
    /// Unary operator: `-x`, `~x`, `x'`, `x.'`
    UnaryOp(String, Box<MatlabExpr>, bool),
    /// Ternary-style if expression (MATLAB doesn't have this — emitted as inline)
    IfExpr(Box<MatlabExpr>, Box<MatlabExpr>, Box<MatlabExpr>),
    /// Anonymous function: `@(x, y) x + y`
    AnonFunc(Vec<String>, Box<MatlabExpr>),
    /// End keyword (for indexing)
    End,
    /// Colon alone (`:`) for all-elements indexing
    Colon,
    /// Nargin / nargout
    Nargin,
    Nargout,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MatlabDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
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
/// Configuration for the MATLAB code generator.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MatlabGenConfig {
    /// Emit statement suppression (`;`) by default.
    pub suppress_output: bool,
    /// Emit `%% section` markers.
    pub emit_section_markers: bool,
    /// Target Octave compatibility (avoid newer MATLAB features).
    pub octave_compat: bool,
    /// Indent string.
    pub indent: String,
    /// Whether to emit function-end `end` keywords (MATLAB 2016b+).
    pub emit_function_end: bool,
    /// Whether to use `@(x) ...` anonymous function syntax.
    pub prefer_anon_functions: bool,
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MatlabDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MatlabWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
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
/// Literal values in MATLAB.
#[derive(Debug, Clone, PartialEq)]
pub enum MatlabLiteral {
    /// `42` or `42.0`
    Double(f64),
    /// `42` integer literal (will cast)
    Integer(i64),
    /// `true` / `false`
    Logical(bool),
    /// `'hello'` char array
    Char(String),
    /// `"hello"` string (R2016b+)
    Str(String),
    /// `[]` empty array / matrix
    Empty,
    /// `NaN`
    NaN,
    /// `Inf` / `-Inf`
    Inf(bool),
    /// `pi`
    Pi,
    /// `eps`
    Eps,
}
/// A MATLAB function documentation annotation.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct MatlabAnnotation {
    /// Short summary line.
    pub summary: String,
    /// Long description.
    pub description: Option<String>,
    /// Input parameter descriptions `(name, description)`.
    pub inputs: Vec<(String, String)>,
    /// Output descriptions `(name, description)`.
    pub outputs: Vec<(String, String)>,
    /// Example lines.
    pub examples: Vec<String>,
    /// See-also references.
    pub see_also: Vec<String>,
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
/// A basic type-consistency checker for MATLAB expressions.
#[allow(dead_code)]
pub struct MatlabTypeChecker {
    /// Variable type environment.
    pub env: HashMap<String, MatlabType>,
    /// Type errors collected.
    pub errors: Vec<String>,
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
#[allow(dead_code)]
pub struct MatlabPassRegistry {
    pub(super) configs: Vec<MatlabPassConfig>,
    pub(super) stats: std::collections::HashMap<String, MatlabPassStats>,
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MatlabAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, MatlabCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MatlabPassConfig {
    pub phase: MatlabPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
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
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum MatlabPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
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
/// A MATLAB struct literal.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct MatlabStructLiteral {
    /// Fields.
    pub fields: Vec<MatlabStructField>,
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
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MatlabPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
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
/// A MATLAB matrix literal.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct MatlabMatrix {
    /// Rows of the matrix (each row is a list of expressions).
    pub rows: Vec<Vec<MatlabExpr>>,
}
impl MatlabMatrix {
    /// Create a new matrix.
    #[allow(dead_code)]
    pub fn new() -> Self {
        MatlabMatrix { rows: Vec::new() }
    }
    /// Add a row to the matrix.
    #[allow(dead_code)]
    pub fn add_row(mut self, row: Vec<MatlabExpr>) -> Self {
        self.rows.push(row);
        self
    }
    /// Number of rows.
    #[allow(dead_code)]
    pub fn num_rows(&self) -> usize {
        self.rows.len()
    }
    /// Number of columns (from the first row).
    #[allow(dead_code)]
    pub fn num_cols(&self) -> usize {
        self.rows.first().map(|r| r.len()).unwrap_or(0)
    }
    /// Emit as a MATLAB matrix literal.
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        let rows: Vec<String> = self
            .rows
            .iter()
            .map(|r| {
                r.iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            })
            .collect();
        format!("[{}]", rows.join("; "))
    }
    /// Create an identity matrix of size `n`.
    #[allow(dead_code)]
    pub fn identity(n: usize) -> MatlabExpr {
        MatlabExpr::Call(
            Box::new(MatlabExpr::Var("eye".to_string())),
            vec![MatlabExpr::Lit(MatlabLiteral::Integer(n as i64))],
        )
    }
    /// Create a zeros matrix of shape `(m, n)`.
    #[allow(dead_code)]
    pub fn zeros(m: usize, n: usize) -> MatlabExpr {
        MatlabExpr::Call(
            Box::new(MatlabExpr::Var("zeros".to_string())),
            vec![
                MatlabExpr::Lit(MatlabLiteral::Integer(m as i64)),
                MatlabExpr::Lit(MatlabLiteral::Integer(n as i64)),
            ],
        )
    }
    /// Create an ones matrix of shape `(m, n)`.
    #[allow(dead_code)]
    pub fn ones(m: usize, n: usize) -> MatlabExpr {
        MatlabExpr::Call(
            Box::new(MatlabExpr::Var("ones".to_string())),
            vec![
                MatlabExpr::Lit(MatlabLiteral::Integer(m as i64)),
                MatlabExpr::Lit(MatlabLiteral::Integer(n as i64)),
            ],
        )
    }
}
/// Statistics about a generated MATLAB module.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MatlabStats {
    /// Number of functions.
    pub num_functions: usize,
    /// Number of classes.
    pub num_classes: usize,
    /// Total number of statements.
    pub total_stmts: usize,
    /// Number of matrix operations.
    pub matrix_ops: usize,
}
impl MatlabStats {
    /// Compute stats from a module builder.
    #[allow(dead_code)]
    pub fn from_module(module: &MatlabModuleBuilder) -> Self {
        let total_stmts = module.functions.iter().map(|f| f.body.len()).sum::<usize>();
        MatlabStats {
            num_functions: module.functions.len(),
            num_classes: module.classes.len(),
            total_stmts,
            matrix_ops: 0,
        }
    }
    /// Merge another stats record.
    #[allow(dead_code)]
    pub fn merge(&mut self, other: &MatlabStats) {
        self.num_functions += other.num_functions;
        self.num_classes += other.num_classes;
        self.total_stmts += other.total_stmts;
        self.matrix_ops += other.matrix_ops;
    }
}
/// MATLAB function parameter with optional validation.
#[derive(Debug, Clone, PartialEq)]
pub struct MatlabParam {
    pub name: String,
    pub default_value: Option<MatlabExpr>,
    pub validator: Option<MatlabType>,
}
impl MatlabParam {
    pub fn required(name: &str) -> Self {
        MatlabParam {
            name: name.to_string(),
            default_value: None,
            validator: None,
        }
    }
    pub fn with_default(name: &str, default: MatlabExpr) -> Self {
        MatlabParam {
            name: name.to_string(),
            default_value: Some(default),
            validator: None,
        }
    }
    pub fn typed(name: &str, ty: MatlabType) -> Self {
        MatlabParam {
            name: name.to_string(),
            default_value: None,
            validator: Some(ty),
        }
    }
}
/// MATLAB classdef definition.
#[derive(Debug, Clone, PartialEq)]
pub struct MatlabClassdef {
    /// Class name
    pub name: String,
    /// Superclass names
    pub superclasses: Vec<String>,
    /// Properties blocks (grouped by access level)
    pub properties: Vec<MatlabProperty>,
    /// Methods
    pub methods: Vec<MatlabFunction>,
    /// Events
    pub events: Vec<String>,
    /// Enumeration members (for enumeration classes)
    pub enumerations: Vec<(String, Vec<MatlabExpr>)>,
}
impl MatlabClassdef {
    pub fn new(name: &str) -> Self {
        MatlabClassdef {
            name: name.to_string(),
            superclasses: Vec::new(),
            properties: Vec::new(),
            methods: Vec::new(),
            events: Vec::new(),
            enumerations: Vec::new(),
        }
    }
    pub fn inherits(mut self, parent: &str) -> Self {
        self.superclasses.push(parent.to_string());
        self
    }
}
/// Top-level MATLAB file structure.
#[derive(Debug, Clone)]
pub struct MatlabFile {
    /// Top-level functions (first is the main function)
    pub functions: Vec<MatlabFunction>,
    /// Script statements (for script files — no functions)
    pub scripts: Vec<MatlabStmt>,
    /// Class definition (for classdef files)
    pub classdef: Option<MatlabClassdef>,
    /// File-level comment block
    pub header_comment: Option<String>,
    /// Whether this is a script file (no function wrapper)
    pub is_script: bool,
}
impl MatlabFile {
    pub fn new() -> Self {
        MatlabFile {
            functions: Vec::new(),
            scripts: Vec::new(),
            classdef: None,
            header_comment: None,
            is_script: false,
        }
    }
    pub fn script() -> Self {
        MatlabFile {
            is_script: true,
            ..Self::new()
        }
    }
    pub fn add_function(&mut self, fun: MatlabFunction) {
        self.functions.push(fun);
    }
    pub fn add_script_stmt(&mut self, stmt: MatlabStmt) {
        self.scripts.push(stmt);
    }
    pub fn with_classdef(mut self, cls: MatlabClassdef) -> Self {
        self.classdef = Some(cls);
        self
    }
    pub fn with_header(mut self, comment: &str) -> Self {
        self.header_comment = Some(comment.to_string());
        self
    }
}
/// A MATLAB script (executed top-to-bottom, no function signature).
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct MatlabScript {
    /// Script file name (without `.m`).
    pub name: String,
    /// Header comment lines.
    pub header_comments: Vec<String>,
    /// Body statements.
    pub statements: Vec<MatlabStmt>,
}
impl MatlabScript {
    /// Create a new script.
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        MatlabScript {
            name: name.into(),
            header_comments: Vec::new(),
            statements: Vec::new(),
        }
    }
    /// Add a header comment.
    #[allow(dead_code)]
    pub fn add_comment(mut self, comment: impl Into<String>) -> Self {
        self.header_comments.push(comment.into());
        self
    }
    /// Add a statement.
    #[allow(dead_code)]
    pub fn add_stmt(mut self, stmt: MatlabStmt) -> Self {
        self.statements.push(stmt);
        self
    }
    /// Emit the full script source.
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        let mut backend = MatlabBackend::new();
        for comment in &self.header_comments {
            backend.emit_stmt(&MatlabStmt::Comment(comment.clone()));
        }
        for stmt in &self.statements {
            backend.emit_stmt(stmt);
        }
        backend.take_output()
    }
}
/// A MATLAB plot specification.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct MatlabPlot {
    /// Figure title.
    pub title: String,
    /// X-axis label.
    pub xlabel: String,
    /// Y-axis label.
    pub ylabel: String,
    /// Data series (each is a variable name + style string).
    pub series: Vec<(String, String)>,
    /// Whether to use a grid.
    pub grid: bool,
    /// Whether to use a legend.
    pub legend: bool,
    /// Figure size `[width, height]` in points.
    pub figure_size: Option<[f64; 2]>,
}
impl MatlabPlot {
    /// Create a new plot with defaults.
    #[allow(dead_code)]
    pub fn new(title: impl Into<String>) -> Self {
        MatlabPlot {
            title: title.into(),
            xlabel: String::new(),
            ylabel: String::new(),
            series: Vec::new(),
            grid: true,
            legend: false,
            figure_size: None,
        }
    }
    /// Add a data series.
    #[allow(dead_code)]
    pub fn add_series(mut self, var: impl Into<String>, style: impl Into<String>) -> Self {
        self.series.push((var.into(), style.into()));
        self
    }
    /// Set axis labels.
    #[allow(dead_code)]
    pub fn labels(mut self, xlabel: impl Into<String>, ylabel: impl Into<String>) -> Self {
        self.xlabel = xlabel.into();
        self.ylabel = ylabel.into();
        self
    }
    /// Enable legend.
    #[allow(dead_code)]
    pub fn with_legend(mut self) -> Self {
        self.legend = true;
        self
    }
    /// Emit MATLAB plotting code.
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        let mut out = String::new();
        out.push_str("figure;\n");
        if let Some([w, h]) = self.figure_size {
            out.push_str(&format!(
                "set(gcf, 'Position', [100, 100, {}, {}]);\n",
                w, h
            ));
        }
        for (i, (var, style)) in self.series.iter().enumerate() {
            if i == 0 {
                out.push_str(&format!("plot({}, '{}');\n", var, style));
            } else {
                out.push_str("hold on;\n");
                out.push_str(&format!("plot({}, '{}');\n", var, style));
            }
        }
        if !self.series.is_empty() {
            out.push_str("hold off;\n");
        }
        if !self.title.is_empty() {
            out.push_str(&format!("title('{}');\n", self.title));
        }
        if !self.xlabel.is_empty() {
            out.push_str(&format!("xlabel('{}');\n", self.xlabel));
        }
        if !self.ylabel.is_empty() {
            out.push_str(&format!("ylabel('{}');\n", self.ylabel));
        }
        if self.grid {
            out.push_str("grid on;\n");
        }
        if self.legend {
            let labels: Vec<_> = self
                .series
                .iter()
                .map(|(v, _)| format!("'{}'", v))
                .collect();
            out.push_str(&format!("legend({});\n", labels.join(", ")));
        }
        out
    }
}
/// A MATLAB struct field value.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct MatlabStructField {
    /// Field name.
    pub name: String,
    /// Field value.
    pub value: MatlabExpr,
}
impl MatlabStructField {
    /// Create a new struct field.
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, value: MatlabExpr) -> Self {
        MatlabStructField {
            name: name.into(),
            value,
        }
    }
}
/// MATLAB statement.
#[derive(Debug, Clone, PartialEq)]
pub enum MatlabStmt {
    /// Assignment: `[a, b] = f(x)` or `a = expr`
    Assign {
        lhs: Vec<String>,
        rhs: MatlabExpr,
        suppress: bool,
    },
    /// Complex left-hand side: `A(i,j) = expr`
    AssignIndex {
        obj: MatlabExpr,
        indices: Vec<MatlabExpr>,
        cell_index: bool,
        rhs: MatlabExpr,
        suppress: bool,
    },
    /// Struct field assignment: `s.field = expr`
    AssignField {
        obj: String,
        field: String,
        rhs: MatlabExpr,
        suppress: bool,
    },
    /// `for var = range; body; end`
    ForLoop {
        var: String,
        range: MatlabExpr,
        body: Vec<MatlabStmt>,
    },
    /// `while cond; body; end`
    WhileLoop {
        cond: MatlabExpr,
        body: Vec<MatlabStmt>,
    },
    /// `if cond; ... elseif ...; else ...; end`
    IfElseIf {
        cond: MatlabExpr,
        then_body: Vec<MatlabStmt>,
        elseif_branches: Vec<(MatlabExpr, Vec<MatlabStmt>)>,
        else_body: Option<Vec<MatlabStmt>>,
    },
    /// `switch expr; case val; ...; otherwise; ...; end`
    SwitchCase {
        expr: MatlabExpr,
        cases: Vec<(MatlabExpr, Vec<MatlabStmt>)>,
        otherwise: Option<Vec<MatlabStmt>>,
    },
    /// `return`
    Return,
    /// `break`
    Break,
    /// `continue`
    Continue,
    /// `error(msg, args...)`
    Error(MatlabExpr, Vec<MatlabExpr>),
    /// `warning(msg, args...)`
    Warning(MatlabExpr, Vec<MatlabExpr>),
    /// `disp(expr)` or `fprintf(...)`
    Disp(MatlabExpr),
    /// Function definition block
    FunctionDef(MatlabFunction),
    /// `try; ...; catch e; ...; end`
    TryCatch {
        body: Vec<MatlabStmt>,
        catch_var: Option<String>,
        catch_body: Vec<MatlabStmt>,
    },
    /// Class property validation
    ValidateProp(String, MatlabExpr),
    /// Expression statement (with or without semicolon suppression)
    Expr(MatlabExpr, bool),
    /// Comment: `% text`
    Comment(String),
    /// `global x y z`
    Global(Vec<String>),
    /// `persistent x`
    Persistent(Vec<String>),
    /// `classdef` inner block statement
    ClassdefStmt(String),
}
/// Argument validation block entry.
#[derive(Debug, Clone, PartialEq)]
pub struct MatlabArgValidation {
    pub name: String,
    pub size: Option<Vec<Option<usize>>>,
    pub class: Option<MatlabType>,
    pub validators: Vec<String>,
    pub default: Option<MatlabExpr>,
}
/// Helpers for generating MATLAB input validation statements.
#[allow(dead_code)]
pub struct MatlabValidation;
impl MatlabValidation {
    /// Emit `validateattributes(x, {'class'}, {attrs...})`.
    #[allow(dead_code)]
    pub fn validate_attributes(var: &str, class: &str, attributes: &[&str]) -> MatlabStmt {
        let attrs_str = attributes
            .iter()
            .map(|a| format!("'{}'", a))
            .collect::<Vec<_>>()
            .join(", ");
        MatlabStmt::Expr(
            MatlabExpr::Call(
                Box::new(MatlabExpr::Var("validateattributes".to_string())),
                vec![
                    MatlabExpr::Var(var.to_string()),
                    MatlabExpr::Lit(MatlabLiteral::Char(format!("{{{{'{}'}}}}", class))),
                    MatlabExpr::Lit(MatlabLiteral::Char(format!("{{{{{}}}}}", attrs_str))),
                ],
            ),
            true,
        )
    }
    /// Emit `narginchk(min, max)`.
    #[allow(dead_code)]
    pub fn narginchk(min: i64, max: i64) -> MatlabStmt {
        MatlabStmt::Expr(
            MatlabExpr::Call(
                Box::new(MatlabExpr::Var("narginchk".to_string())),
                vec![
                    MatlabExpr::Lit(MatlabLiteral::Integer(min)),
                    MatlabExpr::Lit(MatlabLiteral::Integer(max)),
                ],
            ),
            true,
        )
    }
    /// Emit `nargoutchk(min, max)`.
    #[allow(dead_code)]
    pub fn nargoutchk(min: i64, max: i64) -> MatlabStmt {
        MatlabStmt::Expr(
            MatlabExpr::Call(
                Box::new(MatlabExpr::Var("nargoutchk".to_string())),
                vec![
                    MatlabExpr::Lit(MatlabLiteral::Integer(min)),
                    MatlabExpr::Lit(MatlabLiteral::Integer(max)),
                ],
            ),
            true,
        )
    }
}
/// A MATLAB cell array literal.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct MatlabCellArray {
    /// Elements of the cell array.
    pub elements: Vec<MatlabExpr>,
}
impl MatlabCellArray {
    /// Create a new empty cell array.
    #[allow(dead_code)]
    pub fn new() -> Self {
        MatlabCellArray {
            elements: Vec::new(),
        }
    }
    /// Add an element.
    #[allow(dead_code)]
    pub fn add(mut self, elem: MatlabExpr) -> Self {
        self.elements.push(elem);
        self
    }
    /// Emit as a MATLAB cell array literal `{...}`.
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        let elems: Vec<_> = self.elements.iter().map(|e| e.to_string()).collect();
        format!("{{{}}}", elems.join(", "))
    }
}
/// Backend state for emitting MATLAB source code.
pub struct MatlabBackend {
    /// Accumulated output buffer
    pub(super) output: String,
    /// Current indentation level
    pub(super) indent: usize,
    /// Indentation string (default: two spaces)
    pub(super) indent_str: String,
    /// Known class definitions
    pub(super) classes: HashMap<String, MatlabClassdef>,
    /// Whether to emit Octave-compatible output (no `end` keywords)
    pub(super) octave_compat: bool,
}
impl MatlabBackend {
    /// Create a new MATLAB backend.
    pub fn new() -> Self {
        MatlabBackend {
            output: String::new(),
            indent: 0,
            indent_str: "  ".to_string(),
            classes: HashMap::new(),
            octave_compat: false,
        }
    }
    /// Create a backend configured for Octave compatibility.
    pub fn octave() -> Self {
        MatlabBackend {
            octave_compat: true,
            ..Self::new()
        }
    }
    /// Take the accumulated output, resetting the buffer.
    pub fn take_output(&mut self) -> String {
        std::mem::take(&mut self.output)
    }
    /// Register a known class definition.
    pub fn register_class(&mut self, cls: MatlabClassdef) {
        self.classes.insert(cls.name.clone(), cls);
    }
    pub(super) fn current_indent(&self) -> String {
        self.indent_str.repeat(self.indent)
    }
    pub(super) fn emit_line(&mut self, line: &str) {
        let indent = self.current_indent();
        let _ = writeln!(self.output, "{}{}", indent, line);
    }
    pub(super) fn emit_raw(&mut self, s: &str) {
        self.output.push_str(s);
    }
    pub(super) fn indent_up(&mut self) {
        self.indent += 1;
    }
    pub(super) fn indent_down(&mut self) {
        if self.indent > 0 {
            self.indent -= 1;
        }
    }
    /// Emit a complete MATLAB file.
    pub fn emit_file(&mut self, file: &MatlabFile) {
        if let Some(header) = &file.header_comment {
            for line in header.lines() {
                self.emit_line(&format!("% {}", line));
            }
            self.emit_line("");
        }
        if let Some(cls) = &file.classdef.clone() {
            self.emit_classdef(cls);
            return;
        }
        if file.is_script {
            for stmt in &file.scripts.clone() {
                self.emit_stmt(stmt);
            }
            return;
        }
        for (idx, fun) in file.functions.iter().enumerate() {
            if idx > 0 {
                self.emit_line("");
            }
            self.emit_function(fun);
        }
    }
    /// Emit a MATLAB function definition.
    pub fn emit_function(&mut self, fun: &MatlabFunction) {
        let outputs_str = match fun.outputs.len() {
            0 => String::new(),
            1 => format!("{} = ", fun.outputs[0]),
            _ => format!("[{}] = ", fun.outputs.join(", ")),
        };
        let inputs_str: Vec<String> = fun.inputs.iter().map(|p| p.name.clone()).collect();
        self.emit_line(&format!(
            "function {}{}({})",
            outputs_str,
            fun.name,
            inputs_str.join(", ")
        ));
        self.indent_up();
        if let Some(help) = &fun.help_text {
            for line in help.lines() {
                self.emit_line(&format!("% {}", line));
            }
        }
        if !fun.argument_validation.is_empty() {
            self.emit_line("arguments");
            self.indent_up();
            for av in &fun.argument_validation {
                self.emit_arg_validation(av);
            }
            self.indent_down();
            self.emit_line("end");
        }
        for stmt in &fun.body {
            self.emit_stmt(stmt);
        }
        self.indent_down();
        self.emit_line("end");
    }
    pub(super) fn emit_arg_validation(&mut self, av: &MatlabArgValidation) {
        let size_str = if let Some(sizes) = &av.size {
            let s: Vec<String> = sizes
                .iter()
                .map(|d| d.map(|n| n.to_string()).unwrap_or_else(|| ":".to_string()))
                .collect();
            format!("({}) ", s.join(","))
        } else {
            String::new()
        };
        let class_str = if let Some(cls) = &av.class {
            format!("{} ", cls)
        } else {
            String::new()
        };
        let validators_str = if !av.validators.is_empty() {
            format!(" {{{}}}", av.validators.join(", "))
        } else {
            String::new()
        };
        let default_str = if let Some(def) = &av.default {
            format!(" = {}", self.emit_expr(def))
        } else {
            String::new()
        };
        self.emit_line(&format!(
            "{}{}{}{}{}",
            av.name, size_str, class_str, validators_str, default_str
        ));
    }
    /// Emit a MATLAB classdef.
    pub fn emit_classdef(&mut self, cls: &MatlabClassdef) {
        let inherits_str = if cls.superclasses.is_empty() {
            String::new()
        } else {
            format!(" < {}", cls.superclasses.join(" & "))
        };
        self.emit_line(&format!("classdef {}{}", cls.name, inherits_str));
        self.indent_up();
        let pub_props: Vec<&MatlabProperty> = cls
            .properties
            .iter()
            .filter(|p| p.access == PropAccess::Public)
            .collect();
        let prot_props: Vec<&MatlabProperty> = cls
            .properties
            .iter()
            .filter(|p| p.access == PropAccess::Protected)
            .collect();
        let priv_props: Vec<&MatlabProperty> = cls
            .properties
            .iter()
            .filter(|p| p.access == PropAccess::Private)
            .collect();
        if !pub_props.is_empty() {
            self.emit_line("properties");
            self.indent_up();
            for prop in pub_props {
                self.emit_property(prop);
            }
            self.indent_down();
            self.emit_line("end");
        }
        if !prot_props.is_empty() {
            self.emit_line("properties (Access = protected)");
            self.indent_up();
            for prop in prot_props {
                self.emit_property(prop);
            }
            self.indent_down();
            self.emit_line("end");
        }
        if !priv_props.is_empty() {
            self.emit_line("properties (Access = private)");
            self.indent_up();
            for prop in priv_props {
                self.emit_property(prop);
            }
            self.indent_down();
            self.emit_line("end");
        }
        if !cls.events.is_empty() {
            self.emit_line("events");
            self.indent_up();
            for ev in &cls.events {
                self.emit_line(ev);
            }
            self.indent_down();
            self.emit_line("end");
        }
        if !cls.enumerations.is_empty() {
            self.emit_line("enumeration");
            self.indent_up();
            for (name, args) in &cls.enumerations {
                let args_str: Vec<String> = args.iter().map(|a| self.emit_expr(a)).collect();
                if args_str.is_empty() {
                    self.emit_line(name);
                } else {
                    self.emit_line(&format!("{}({})", name, args_str.join(", ")));
                }
            }
            self.indent_down();
            self.emit_line("end");
        }
        if !cls.methods.is_empty() {
            self.emit_line("methods");
            self.indent_up();
            for method in &cls.methods.clone() {
                self.emit_function(method);
                self.emit_line("");
            }
            self.indent_down();
            self.emit_line("end");
        }
        self.indent_down();
        self.emit_line("end");
    }
    pub(super) fn emit_property(&mut self, prop: &MatlabProperty) {
        let ty_str = if let Some(ty) = &prop.ty {
            format!(" ({})", ty)
        } else {
            String::new()
        };
        if let Some(default) = &prop.default {
            let default_str = self.emit_expr(default);
            self.emit_line(&format!("{}{} = {}", prop.name, ty_str, default_str));
        } else {
            self.emit_line(&format!("{}{}", prop.name, ty_str));
        }
    }
    /// Emit a single MATLAB statement.
    pub fn emit_stmt(&mut self, stmt: &MatlabStmt) {
        match stmt {
            MatlabStmt::Assign { lhs, rhs, suppress } => {
                let rhs_str = self.emit_expr(rhs);
                let semi = if *suppress { ";" } else { "" };
                match lhs.len() {
                    0 => self.emit_line(&format!("{}{}", rhs_str, semi)),
                    1 => self.emit_line(&format!("{} = {}{}", lhs[0], rhs_str, semi)),
                    _ => self.emit_line(&format!("[{}] = {}{}", lhs.join(", "), rhs_str, semi)),
                }
            }
            MatlabStmt::AssignIndex {
                obj,
                indices,
                cell_index,
                rhs,
                suppress,
            } => {
                let obj_str = self.emit_expr(obj);
                let idx_str: Vec<String> = indices.iter().map(|i| self.emit_expr(i)).collect();
                let rhs_str = self.emit_expr(rhs);
                let semi = if *suppress { ";" } else { "" };
                let (open, close) = if *cell_index { ("{", "}") } else { ("(", ")") };
                self.emit_line(&format!(
                    "{}{}{}{}{}){} = {}{}",
                    obj_str,
                    open,
                    idx_str.join(", "),
                    close,
                    "",
                    "",
                    rhs_str,
                    semi
                ));
                if let Some(bad) = self.output.lines().last().map(|l| l.to_string()) {
                    let len_to_remove = bad.len() + 1;
                    let new_len = self.output.len().saturating_sub(len_to_remove);
                    self.output.truncate(new_len);
                }
                let indent = self.current_indent();
                let _ = writeln!(
                    self.output,
                    "{}{}{}{}{}{}{}",
                    indent,
                    obj_str,
                    open,
                    idx_str.join(", "),
                    close,
                    format_args!(" = {}", rhs_str),
                    semi
                );
            }
            MatlabStmt::AssignField {
                obj,
                field,
                rhs,
                suppress,
            } => {
                let rhs_str = self.emit_expr(rhs);
                let semi = if *suppress { ";" } else { "" };
                self.emit_line(&format!("{}.{} = {}{}", obj, field, rhs_str, semi));
            }
            MatlabStmt::ForLoop { var, range, body } => {
                let range_str = self.emit_expr(range);
                self.emit_line(&format!("for {} = {}", var, range_str));
                self.indent_up();
                for s in body {
                    self.emit_stmt(s);
                }
                self.indent_down();
                self.emit_line("end");
            }
            MatlabStmt::WhileLoop { cond, body } => {
                let cond_str = self.emit_expr(cond);
                self.emit_line(&format!("while {}", cond_str));
                self.indent_up();
                for s in body {
                    self.emit_stmt(s);
                }
                self.indent_down();
                self.emit_line("end");
            }
            MatlabStmt::IfElseIf {
                cond,
                then_body,
                elseif_branches,
                else_body,
            } => {
                let cond_str = self.emit_expr(cond);
                self.emit_line(&format!("if {}", cond_str));
                self.indent_up();
                for s in then_body {
                    self.emit_stmt(s);
                }
                self.indent_down();
                for (elif_cond, elif_body) in elseif_branches {
                    let elif_str = self.emit_expr(elif_cond);
                    self.emit_line(&format!("elseif {}", elif_str));
                    self.indent_up();
                    for s in elif_body {
                        self.emit_stmt(s);
                    }
                    self.indent_down();
                }
                if let Some(else_stmts) = else_body {
                    self.emit_line("else");
                    self.indent_up();
                    for s in else_stmts {
                        self.emit_stmt(s);
                    }
                    self.indent_down();
                }
                self.emit_line("end");
            }
            MatlabStmt::SwitchCase {
                expr,
                cases,
                otherwise,
            } => {
                let expr_str = self.emit_expr(expr);
                self.emit_line(&format!("switch {}", expr_str));
                self.indent_up();
                for (val, body) in cases {
                    let val_str = self.emit_expr(val);
                    self.emit_line(&format!("case {}", val_str));
                    self.indent_up();
                    for s in body {
                        self.emit_stmt(s);
                    }
                    self.indent_down();
                }
                if let Some(other_stmts) = otherwise {
                    self.emit_line("otherwise");
                    self.indent_up();
                    for s in other_stmts {
                        self.emit_stmt(s);
                    }
                    self.indent_down();
                }
                self.indent_down();
                self.emit_line("end");
            }
            MatlabStmt::Return => self.emit_line("return;"),
            MatlabStmt::Break => self.emit_line("break;"),
            MatlabStmt::Continue => self.emit_line("continue;"),
            MatlabStmt::Error(fmt_expr, args) => {
                let fmt_str = self.emit_expr(fmt_expr);
                if args.is_empty() {
                    self.emit_line(&format!("error({});", fmt_str));
                } else {
                    let args_str: Vec<String> = args.iter().map(|a| self.emit_expr(a)).collect();
                    self.emit_line(&format!("error({}, {});", fmt_str, args_str.join(", ")));
                }
            }
            MatlabStmt::Warning(fmt_expr, args) => {
                let fmt_str = self.emit_expr(fmt_expr);
                if args.is_empty() {
                    self.emit_line(&format!("warning({});", fmt_str));
                } else {
                    let args_str: Vec<String> = args.iter().map(|a| self.emit_expr(a)).collect();
                    self.emit_line(&format!("warning({}, {});", fmt_str, args_str.join(", ")));
                }
            }
            MatlabStmt::Disp(expr) => {
                let e_str = self.emit_expr(expr);
                self.emit_line(&format!("disp({});", e_str));
            }
            MatlabStmt::FunctionDef(fun) => {
                self.emit_function(fun);
            }
            MatlabStmt::TryCatch {
                body,
                catch_var,
                catch_body,
            } => {
                self.emit_line("try");
                self.indent_up();
                for s in body {
                    self.emit_stmt(s);
                }
                self.indent_down();
                if let Some(var) = catch_var {
                    self.emit_line(&format!("catch {}", var));
                } else {
                    self.emit_line("catch");
                }
                self.indent_up();
                for s in catch_body {
                    self.emit_stmt(s);
                }
                self.indent_down();
                self.emit_line("end");
            }
            MatlabStmt::ValidateProp(name, expr) => {
                let e_str = self.emit_expr(expr);
                self.emit_line(&format!("validateattributes({}, {});", name, e_str));
            }
            MatlabStmt::Expr(expr, suppress) => {
                let e_str = self.emit_expr(expr);
                let semi = if *suppress { ";" } else { "" };
                self.emit_line(&format!("{}{}", e_str, semi));
            }
            MatlabStmt::Comment(text) => {
                for line in text.lines() {
                    self.emit_line(&format!("% {}", line));
                }
            }
            MatlabStmt::Global(names) => {
                self.emit_line(&format!("global {}", names.join(" ")));
            }
            MatlabStmt::Persistent(names) => {
                self.emit_line(&format!("persistent {}", names.join(" ")));
            }
            MatlabStmt::ClassdefStmt(s) => {
                self.emit_line(s);
            }
        }
    }
    /// Emit a MATLAB expression to a string.
    pub fn emit_expr(&mut self, expr: &MatlabExpr) -> String {
        self.emit_expr_pure(expr)
    }
    /// Emit a MATLAB expression to a string (pure).
    pub fn emit_expr_pure(&self, expr: &MatlabExpr) -> String {
        match expr {
            MatlabExpr::Lit(lit) => self.emit_literal(lit),
            MatlabExpr::Var(name) => name.clone(),
            MatlabExpr::MatrixLit(rows) => {
                let rows_str: Vec<String> = rows
                    .iter()
                    .map(|row| {
                        let elems: Vec<String> =
                            row.iter().map(|e| self.emit_expr_pure(e)).collect();
                        elems.join(", ")
                    })
                    .collect();
                format!("[{}]", rows_str.join("; "))
            }
            MatlabExpr::CellLit(rows) => {
                let rows_str: Vec<String> = rows
                    .iter()
                    .map(|row| {
                        let elems: Vec<String> =
                            row.iter().map(|e| self.emit_expr_pure(e)).collect();
                        elems.join(", ")
                    })
                    .collect();
                format!("{{{}}}", rows_str.join("; "))
            }
            MatlabExpr::ColonRange { start, step, end } => {
                let start_str = self.emit_expr_pure(start);
                let end_str = self.emit_expr_pure(end);
                if let Some(step_expr) = step {
                    let step_str = self.emit_expr_pure(step_expr);
                    format!("{}:{}:{}", start_str, step_str, end_str)
                } else {
                    format!("{}:{}", start_str, end_str)
                }
            }
            MatlabExpr::Call(func, args) => {
                let func_str = self.emit_expr_pure(func);
                let args_str: Vec<String> = args.iter().map(|a| self.emit_expr_pure(a)).collect();
                format!("{}({})", func_str, args_str.join(", "))
            }
            MatlabExpr::Index {
                obj,
                indices,
                cell_index,
            } => {
                let obj_str = self.emit_expr_pure(obj);
                let idx_str: Vec<String> = indices.iter().map(|i| self.emit_expr_pure(i)).collect();
                let (open, close) = if *cell_index { ("{", "}") } else { ("(", ")") };
                format!("{}{}{}{}", obj_str, open, idx_str.join(", "), close)
            }
            MatlabExpr::FieldAccess(obj, field) => {
                let obj_str = self.emit_expr_pure(obj);
                format!("{}.{}", obj_str, field)
            }
            MatlabExpr::BinaryOp(op, lhs, rhs) => {
                let lhs_str = self.emit_expr_pure(lhs);
                let rhs_str = self.emit_expr_pure(rhs);
                format!("{} {} {}", lhs_str, op, rhs_str)
            }
            MatlabExpr::UnaryOp(op, operand, postfix) => {
                let operand_str = self.emit_expr_pure(operand);
                if *postfix {
                    format!("{}{}", operand_str, op)
                } else {
                    format!("{}{}", op, operand_str)
                }
            }
            MatlabExpr::IfExpr(cond, then_expr, else_expr) => {
                let cond_str = self.emit_expr_pure(cond);
                let then_str = self.emit_expr_pure(then_expr);
                let else_str = self.emit_expr_pure(else_expr);
                format!("({{{}; {}}}{{{}+1}})", else_str, then_str, cond_str)
            }
            MatlabExpr::AnonFunc(params, body) => {
                let params_str = params.join(", ");
                let body_str = self.emit_expr_pure(body);
                format!("@({}) {}", params_str, body_str)
            }
            MatlabExpr::End => "end".to_string(),
            MatlabExpr::Colon => ":".to_string(),
            MatlabExpr::Nargin => "nargin".to_string(),
            MatlabExpr::Nargout => "nargout".to_string(),
        }
    }
    pub(super) fn emit_literal(&self, lit: &MatlabLiteral) -> String {
        match lit {
            MatlabLiteral::Double(f) => {
                if f.fract() == 0.0 && f.abs() < 1e15 {
                    format!("{}", *f as i64)
                } else {
                    format!("{}", f)
                }
            }
            MatlabLiteral::Integer(n) => format!("{}", n),
            MatlabLiteral::Logical(b) => {
                if *b {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            MatlabLiteral::Char(s) => format!("'{}'", s.replace('\'', "''")),
            MatlabLiteral::Str(s) => format!("\"{}\"", s.replace('"', "\"\"")),
            MatlabLiteral::Empty => "[]".to_string(),
            MatlabLiteral::NaN => "NaN".to_string(),
            MatlabLiteral::Inf(neg) => {
                if *neg {
                    "-Inf".to_string()
                } else {
                    "Inf".to_string()
                }
            }
            MatlabLiteral::Pi => "pi".to_string(),
            MatlabLiteral::Eps => "eps".to_string(),
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MatlabLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl MatlabLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        MatlabLivenessInfo {
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
/// Helpers for constructing common MATLAB numeric operations.
#[allow(dead_code)]
pub struct MatlabNumericOps;
impl MatlabNumericOps {
    /// Element-wise multiply `a .* b`.
    #[allow(dead_code)]
    pub fn elem_mul(a: MatlabExpr, b: MatlabExpr) -> MatlabExpr {
        MatlabExpr::BinaryOp(".*".to_string(), Box::new(a), Box::new(b))
    }
    /// Element-wise divide `a ./ b`.
    #[allow(dead_code)]
    pub fn elem_div(a: MatlabExpr, b: MatlabExpr) -> MatlabExpr {
        MatlabExpr::BinaryOp("./".to_string(), Box::new(a), Box::new(b))
    }
    /// Matrix multiply `a * b`.
    #[allow(dead_code)]
    pub fn mat_mul(a: MatlabExpr, b: MatlabExpr) -> MatlabExpr {
        MatlabExpr::BinaryOp("*".to_string(), Box::new(a), Box::new(b))
    }
    /// Matrix power `a ^ n`.
    #[allow(dead_code)]
    pub fn mat_pow(a: MatlabExpr, n: MatlabExpr) -> MatlabExpr {
        MatlabExpr::BinaryOp("^".to_string(), Box::new(a), Box::new(n))
    }
    /// Element-wise power `a .^ n`.
    #[allow(dead_code)]
    pub fn elem_pow(a: MatlabExpr, n: MatlabExpr) -> MatlabExpr {
        MatlabExpr::BinaryOp(".^".to_string(), Box::new(a), Box::new(n))
    }
    /// Colon range `start:stop`.
    #[allow(dead_code)]
    pub fn range(start: MatlabExpr, stop: MatlabExpr) -> MatlabExpr {
        MatlabExpr::ColonRange {
            start: Box::new(start),
            step: None,
            end: Box::new(stop),
        }
    }
    /// Colon range with step `start:step:stop`.
    #[allow(dead_code)]
    pub fn range_step(start: MatlabExpr, step: MatlabExpr, stop: MatlabExpr) -> MatlabExpr {
        MatlabExpr::ColonRange {
            start: Box::new(start),
            step: Some(Box::new(step)),
            end: Box::new(stop),
        }
    }
    /// `abs(x)`.
    #[allow(dead_code)]
    pub fn abs(x: MatlabExpr) -> MatlabExpr {
        MatlabExpr::Call(Box::new(MatlabExpr::Var("abs".to_string())), vec![x])
    }
    /// `sum(x)`.
    #[allow(dead_code)]
    pub fn sum(x: MatlabExpr) -> MatlabExpr {
        MatlabExpr::Call(Box::new(MatlabExpr::Var("sum".to_string())), vec![x])
    }
    /// `prod(x)`.
    #[allow(dead_code)]
    pub fn prod(x: MatlabExpr) -> MatlabExpr {
        MatlabExpr::Call(Box::new(MatlabExpr::Var("prod".to_string())), vec![x])
    }
    /// `min(x)`.
    #[allow(dead_code)]
    pub fn min(x: MatlabExpr) -> MatlabExpr {
        MatlabExpr::Call(Box::new(MatlabExpr::Var("min".to_string())), vec![x])
    }
    /// `max(x)`.
    #[allow(dead_code)]
    pub fn max(x: MatlabExpr) -> MatlabExpr {
        MatlabExpr::Call(Box::new(MatlabExpr::Var("max".to_string())), vec![x])
    }
    /// `mean(x)`.
    #[allow(dead_code)]
    pub fn mean(x: MatlabExpr) -> MatlabExpr {
        MatlabExpr::Call(Box::new(MatlabExpr::Var("mean".to_string())), vec![x])
    }
    /// `std(x)`.
    #[allow(dead_code)]
    pub fn std(x: MatlabExpr) -> MatlabExpr {
        MatlabExpr::Call(Box::new(MatlabExpr::Var("std".to_string())), vec![x])
    }
    /// `sqrt(x)`.
    #[allow(dead_code)]
    pub fn sqrt(x: MatlabExpr) -> MatlabExpr {
        MatlabExpr::Call(Box::new(MatlabExpr::Var("sqrt".to_string())), vec![x])
    }
    /// `norm(x)`.
    #[allow(dead_code)]
    pub fn norm(x: MatlabExpr) -> MatlabExpr {
        MatlabExpr::Call(Box::new(MatlabExpr::Var("norm".to_string())), vec![x])
    }
    /// `det(A)`.
    #[allow(dead_code)]
    pub fn det(a: MatlabExpr) -> MatlabExpr {
        MatlabExpr::Call(Box::new(MatlabExpr::Var("det".to_string())), vec![a])
    }
    /// `inv(A)`.
    #[allow(dead_code)]
    pub fn inv(a: MatlabExpr) -> MatlabExpr {
        MatlabExpr::Call(Box::new(MatlabExpr::Var("inv".to_string())), vec![a])
    }
    /// `eig(A)`.
    #[allow(dead_code)]
    pub fn eig(a: MatlabExpr) -> MatlabExpr {
        MatlabExpr::Call(Box::new(MatlabExpr::Var("eig".to_string())), vec![a])
    }
    /// `svd(A)`.
    #[allow(dead_code)]
    pub fn svd(a: MatlabExpr) -> MatlabExpr {
        MatlabExpr::Call(Box::new(MatlabExpr::Var("svd".to_string())), vec![a])
    }
    /// `linspace(a, b, n)`.
    #[allow(dead_code)]
    pub fn linspace(a: MatlabExpr, b: MatlabExpr, n: MatlabExpr) -> MatlabExpr {
        MatlabExpr::Call(
            Box::new(MatlabExpr::Var("linspace".to_string())),
            vec![a, b, n],
        )
    }
    /// `mod(a, m)`.
    #[allow(dead_code)]
    pub fn matlab_mod(a: MatlabExpr, m: MatlabExpr) -> MatlabExpr {
        MatlabExpr::Call(Box::new(MatlabExpr::Var("mod".to_string())), vec![a, m])
    }
    /// `floor(x)`.
    #[allow(dead_code)]
    pub fn floor(x: MatlabExpr) -> MatlabExpr {
        MatlabExpr::Call(Box::new(MatlabExpr::Var("floor".to_string())), vec![x])
    }
    /// `ceil(x)`.
    #[allow(dead_code)]
    pub fn ceil(x: MatlabExpr) -> MatlabExpr {
        MatlabExpr::Call(Box::new(MatlabExpr::Var("ceil".to_string())), vec![x])
    }
    /// `round(x)`.
    #[allow(dead_code)]
    pub fn round(x: MatlabExpr) -> MatlabExpr {
        MatlabExpr::Call(Box::new(MatlabExpr::Var("round".to_string())), vec![x])
    }
    /// `fix(x)` — truncate toward zero.
    #[allow(dead_code)]
    pub fn fix(x: MatlabExpr) -> MatlabExpr {
        MatlabExpr::Call(Box::new(MatlabExpr::Var("fix".to_string())), vec![x])
    }
    /// `rem(a, m)` — remainder (sign matches dividend).
    #[allow(dead_code)]
    pub fn rem(a: MatlabExpr, m: MatlabExpr) -> MatlabExpr {
        MatlabExpr::Call(Box::new(MatlabExpr::Var("rem".to_string())), vec![a, m])
    }
}
/// A MATLAB function definition.
#[derive(Debug, Clone, PartialEq)]
pub struct MatlabFunction {
    /// Function name
    pub name: String,
    /// Input parameter names
    pub inputs: Vec<MatlabParam>,
    /// Output parameter names
    pub outputs: Vec<String>,
    /// Function body
    pub body: Vec<MatlabStmt>,
    /// Whether this is a nested function
    pub is_nested: bool,
    /// Whether this is a local function (appears after main function)
    pub is_local: bool,
    /// Help text (first comment block)
    pub help_text: Option<String>,
    /// Validation blocks (arguments ... end)
    pub argument_validation: Vec<MatlabArgValidation>,
}
impl MatlabFunction {
    pub fn new(
        name: &str,
        inputs: Vec<MatlabParam>,
        outputs: Vec<String>,
        body: Vec<MatlabStmt>,
    ) -> Self {
        MatlabFunction {
            name: name.to_string(),
            inputs,
            outputs,
            body,
            is_nested: false,
            is_local: false,
            help_text: None,
            argument_validation: Vec::new(),
        }
    }
    pub fn nested(mut self) -> Self {
        self.is_nested = true;
        self
    }
    pub fn local(mut self) -> Self {
        self.is_local = true;
        self
    }
    pub fn with_help(mut self, help: &str) -> Self {
        self.help_text = Some(help.to_string());
        self
    }
}
/// A high-level builder for MATLAB modules (collections of functions).
#[allow(dead_code)]
pub struct MatlabModuleBuilder {
    /// Module name.
    pub name: String,
    /// Functions in declaration order.
    pub functions: Vec<MatlabFunction>,
    /// Classes in declaration order.
    pub classes: Vec<MatlabClassdef>,
    /// Scripts (stand-alone statements).
    pub scripts: Vec<MatlabScript>,
    /// Global variable declarations.
    pub globals: Vec<String>,
    /// Configuration.
    pub config: MatlabGenConfig,
}
impl MatlabModuleBuilder {
    /// Create a new module builder.
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        MatlabModuleBuilder {
            name: name.into(),
            functions: Vec::new(),
            classes: Vec::new(),
            scripts: Vec::new(),
            globals: Vec::new(),
            config: MatlabGenConfig::default(),
        }
    }
    /// Add a function.
    #[allow(dead_code)]
    pub fn add_function(mut self, func: MatlabFunction) -> Self {
        self.functions.push(func);
        self
    }
    /// Add a class.
    #[allow(dead_code)]
    pub fn add_class(mut self, cls: MatlabClassdef) -> Self {
        self.classes.push(cls);
        self
    }
    /// Add a script.
    #[allow(dead_code)]
    pub fn add_script(mut self, script: MatlabScript) -> Self {
        self.scripts.push(script);
        self
    }
    /// Declare a global variable.
    #[allow(dead_code)]
    pub fn declare_global(mut self, name: impl Into<String>) -> Self {
        self.globals.push(name.into());
        self
    }
    /// Emit the entire module.
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        let mut backend = MatlabBackend::new();
        if !self.globals.is_empty() {
            let globals = self.globals.join(" ");
            backend.emit_stmt(&MatlabStmt::Comment(format!("globals: {}", globals)));
        }
        for func in &self.functions {
            backend.emit_function(func);
        }
        for cls in &self.classes {
            backend.emit_classdef(cls);
        }
        backend.take_output()
    }
    /// Number of items in the module.
    #[allow(dead_code)]
    pub fn total_items(&self) -> usize {
        self.functions.len() + self.classes.len() + self.scripts.len()
    }
}
