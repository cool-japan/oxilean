//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use crate::context::ElabContext;
use oxilean_kernel::{Environment, Expr, FVarId, Level, Name};
use oxilean_parse::{Located, MatchArm, Pattern, SurfaceExpr};
use std::collections::HashMap;

/// Decision tree for compiled pattern matching.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DecisionTree {
    /// Root node of the decision tree.
    root: DecisionNode,
}
impl DecisionTree {
    /// Create a new decision tree from match arms.
    pub fn new(arms: &[(Located<Pattern>, Located<SurfaceExpr>)]) -> Result<Self, String> {
        if arms.is_empty() {
            return Ok(Self {
                root: DecisionNode::Leaf(Expr::BVar(0)),
            });
        }
        let mut node = DecisionNode::Fail;
        for (pat, rhs_surf) in arms.iter().rev() {
            let rhs = surface_to_placeholder(&rhs_surf.value);
            node = build_node_from_pattern(&pat.value, rhs, node);
        }
        Ok(Self { root: node })
    }
    /// Create a decision tree from elaborated match equations.
    #[allow(dead_code)]
    pub fn from_equations(equations: &[MatchEquation]) -> Result<Self, String> {
        if equations.is_empty() {
            return Ok(Self {
                root: DecisionNode::Fail,
            });
        }
        let mut node = DecisionNode::Fail;
        for eq in equations.iter().rev() {
            if eq.patterns.is_empty() {
                node = DecisionNode::Leaf(eq.rhs.clone());
            } else {
                node = build_node_from_elab_pattern(&eq.patterns[0], eq.rhs.clone(), node);
            }
        }
        Ok(Self { root: node })
    }
    /// Compile the decision tree to a kernel expression.
    pub fn compile(&self) -> Expr {
        compile_node(&self.root)
    }
}
/// High-level elaboration of a `match` expression.
#[allow(dead_code)]
pub struct MatchElaborator {
    /// Fresh variable counter.
    next_var: u64,
    /// Whether to perform exhaustiveness checking.
    pub check_exhaustive: bool,
    /// Whether to perform redundancy checking.
    pub check_redundant: bool,
}
#[allow(dead_code)]
impl MatchElaborator {
    /// Create a new match elaborator.
    pub fn new() -> Self {
        Self {
            next_var: 10000,
            check_exhaustive: true,
            check_redundant: true,
        }
    }
    /// Generate a fresh free variable ID.
    pub fn fresh_fvar(&mut self) -> FVarId {
        let id = FVarId(self.next_var);
        self.next_var += 1;
        id
    }
    /// Elaborate a pattern and return the `ElabPattern` plus variable bindings.
    #[allow(clippy::type_complexity)]
    pub fn elab_pattern(
        &mut self,
        pat: &Pattern,
        ty: &Expr,
    ) -> Result<(ElabPattern, Vec<(FVarId, Name, Expr)>), String> {
        elab_pattern_with_counter(pat, ty, &mut self.next_var)
    }
    /// Run exhaustiveness checking on a set of surface patterns.
    pub fn check_exhaust(&self, patterns: &[Located<Pattern>]) -> ExhaustivenessResult {
        check_exhaustive_full(patterns)
    }
    /// Run redundancy checking on a set of surface patterns.
    pub fn check_redundant_arms(&self, patterns: &[Located<Pattern>]) -> Vec<usize> {
        check_redundant_full(patterns)
    }
}
/// Substitutes free variables within an elaborated pattern.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct PatternSubstitution {
    /// Mapping from `FVarId` to replacement expression.
    subst: HashMap<FVarId, Expr>,
}
#[allow(dead_code)]
impl PatternSubstitution {
    /// Create an empty substitution.
    pub fn new() -> Self {
        Self::default()
    }
    /// Bind a free variable to an expression.
    pub fn bind(&mut self, fvar: FVarId, expr: Expr) {
        self.subst.insert(fvar, expr);
    }
    /// Look up the replacement for a free variable.
    pub fn get(&self, fvar: &FVarId) -> Option<&Expr> {
        self.subst.get(fvar)
    }
    /// Return the number of bindings.
    pub fn len(&self) -> usize {
        self.subst.len()
    }
    /// Return true if there are no bindings.
    pub fn is_empty(&self) -> bool {
        self.subst.is_empty()
    }
    /// Apply the substitution to an expression (identity placeholder).
    pub fn apply_to_expr(&self, expr: &Expr) -> Expr {
        expr.clone()
    }
    /// Merge another substitution into this one (later bindings override).
    pub fn merge(&mut self, other: &PatternSubstitution) {
        for (k, v) in &other.subst {
            self.subst.insert(*k, v.clone());
        }
    }
}
/// Heuristics for selecting the best column to split in a pattern matrix.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColumnHeuristic {
    /// Always choose the leftmost column.
    LeftMost,
    /// Choose the column with the most constructor patterns.
    MostConstructors,
    /// Choose the column with the smallest number of distinct patterns.
    SmallestBranching,
}
#[allow(dead_code)]
impl ColumnHeuristic {
    /// Apply this heuristic to select a column from the matrix.
    pub fn select_column(&self, matrix: &PatternMatrix) -> usize {
        match self {
            ColumnHeuristic::LeftMost => 0,
            ColumnHeuristic::MostConstructors => matrix.best_column(),
            ColumnHeuristic::SmallestBranching => matrix.best_column(),
        }
    }
}
/// An elaborated pattern with full type information.
#[derive(Clone, Debug, PartialEq)]
pub enum ElabPattern {
    /// Wildcard: matches anything.
    Wild,
    /// Variable binding: captures the matched value.
    Var(FVarId, Name, Expr),
    /// Constructor application: matches a specific constructor.
    Ctor(Name, Vec<ElabPattern>, Expr),
    /// Literal pattern.
    Lit(oxilean_kernel::Literal),
    /// Or pattern: matches if either sub-pattern matches.
    Or(Box<ElabPattern>, Box<ElabPattern>),
    /// As-pattern: binds a variable while matching a sub-pattern.
    As(FVarId, Name, Box<ElabPattern>),
    /// Inaccessible pattern: not used for matching, only for type-checking.
    Inaccessible(Expr),
}
/// Statistics about a collection of patterns.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct PatternStats {
    /// Total number of patterns.
    pub total: usize,
    /// Number of wildcard patterns.
    pub wilds: usize,
    /// Number of variable bindings.
    pub variables: usize,
    /// Number of constructor patterns.
    pub constructors: usize,
    /// Number of literal patterns.
    pub literals: usize,
    /// Number of or-patterns.
    pub ors: usize,
    /// Number of as-patterns.
    pub as_pats: usize,
    /// Number of inaccessible patterns.
    pub inaccessibles: usize,
    /// Maximum nesting depth.
    pub max_depth: usize,
}
#[allow(dead_code)]
impl PatternStats {
    /// Compute statistics from a slice of elaborated patterns.
    pub fn from_patterns(patterns: &[ElabPattern]) -> Self {
        let mut stats = Self {
            total: patterns.len(),
            ..Self::default()
        };
        for pat in patterns {
            stats.count_pattern(pat);
            let depth = pattern_depth(pat);
            if depth > stats.max_depth {
                stats.max_depth = depth;
            }
        }
        stats
    }
    fn count_pattern(&mut self, pat: &ElabPattern) {
        match pat {
            ElabPattern::Wild => self.wilds += 1,
            ElabPattern::Var(_, _, _) => self.variables += 1,
            ElabPattern::Ctor(_, sub, _) => {
                self.constructors += 1;
                for s in sub {
                    self.count_pattern(s);
                }
            }
            ElabPattern::Lit(_) => self.literals += 1,
            ElabPattern::Or(a, b) => {
                self.ors += 1;
                self.count_pattern(a);
                self.count_pattern(b);
            }
            ElabPattern::As(_, _, inner) => {
                self.as_pats += 1;
                self.count_pattern(inner);
            }
            ElabPattern::Inaccessible(_) => self.inaccessibles += 1,
        }
    }
}
/// Elaboration result for pattern matching.
#[derive(Debug, Clone)]
pub struct MatchResult {
    /// The compiled match expression.
    pub expr: Expr,
    /// Generated auxiliary definitions.
    pub defs: Vec<(Name, Expr)>,
    /// The expanded equations.
    pub equations: Vec<MatchEquation>,
    /// Missing patterns (for incomplete matches).
    pub missing_patterns: Vec<MissingPattern>,
    /// Indices of redundant arms.
    pub redundant_arms: Vec<usize>,
}
/// A match equation: a row in the pattern matrix.
#[derive(Clone, Debug)]
pub struct MatchEquation {
    /// The patterns for this row (one per scrutinee in multi-match).
    pub patterns: Vec<ElabPattern>,
    /// The right-hand side expression.
    pub rhs: Expr,
    /// Index of the original arm this equation came from.
    pub arm_idx: usize,
}
/// A pattern matrix used during compilation.
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct PatternMatrix {
    /// Rows of the matrix: each row has patterns and a rhs index.
    pub(super) rows: Vec<MatrixRow>,
    /// Number of columns.
    pub(super) num_cols: usize,
}
#[allow(dead_code)]
impl PatternMatrix {
    /// Create a new pattern matrix from equations.
    pub(crate) fn from_equations(equations: &[MatchEquation]) -> Self {
        let num_cols = equations.first().map(|e| e.patterns.len()).unwrap_or(0);
        let rows = equations
            .iter()
            .map(|eq| MatrixRow {
                patterns: eq.patterns.clone(),
                rhs_idx: eq.arm_idx,
            })
            .collect();
        Self { rows, num_cols }
    }
    /// Check if the matrix is empty.
    pub(crate) fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
    /// Pick the best column to split on (heuristic: most constructors).
    pub(crate) fn best_column(&self) -> usize {
        if self.num_cols == 0 {
            return 0;
        }
        let mut best_col = 0;
        let mut best_score = 0;
        for col in 0..self.num_cols {
            let mut ctors = Vec::new();
            for row in &self.rows {
                if col < row.patterns.len() {
                    if let ElabPattern::Ctor(name, _, _) = &row.patterns[col] {
                        if !ctors.contains(name) {
                            ctors.push(name.clone());
                        }
                    }
                }
            }
            if ctors.len() > best_score {
                best_score = ctors.len();
                best_col = col;
            }
        }
        best_col
    }
    /// Specialize the matrix for a given constructor.
    ///
    /// Keeps only rows where column `col` matches `ctor_name`, and expands
    /// the constructor's sub-patterns into new columns.
    pub(super) fn specialize(
        &self,
        col: usize,
        ctor_name: &Name,
        ctor_arity: usize,
    ) -> PatternMatrix {
        let mut new_rows = Vec::new();
        for row in &self.rows {
            if col >= row.patterns.len() {
                continue;
            }
            match &row.patterns[col] {
                ElabPattern::Ctor(name, sub_pats, _) if name == ctor_name => {
                    let mut new_pats = Vec::new();
                    for sp in sub_pats {
                        new_pats.push(sp.clone());
                    }
                    while new_pats.len() < ctor_arity {
                        new_pats.push(ElabPattern::Wild);
                    }
                    for (j, p) in row.patterns.iter().enumerate() {
                        if j != col {
                            new_pats.push(p.clone());
                        }
                    }
                    new_rows.push(MatrixRow {
                        patterns: new_pats,
                        rhs_idx: row.rhs_idx,
                    });
                }
                ElabPattern::Wild | ElabPattern::Var(_, _, _) => {
                    let mut new_pats = Vec::new();
                    for _ in 0..ctor_arity {
                        new_pats.push(ElabPattern::Wild);
                    }
                    for (j, p) in row.patterns.iter().enumerate() {
                        if j != col {
                            new_pats.push(p.clone());
                        }
                    }
                    new_rows.push(MatrixRow {
                        patterns: new_pats,
                        rhs_idx: row.rhs_idx,
                    });
                }
                _ => {}
            }
        }
        let num_cols = if self.num_cols > 0 {
            self.num_cols - 1 + ctor_arity
        } else {
            ctor_arity
        };
        PatternMatrix {
            rows: new_rows,
            num_cols,
        }
    }
    /// Default matrix: rows where column `col` is a wildcard/variable.
    pub(super) fn default_matrix(&self, col: usize) -> PatternMatrix {
        let mut new_rows = Vec::new();
        for row in &self.rows {
            if col >= row.patterns.len() {
                continue;
            }
            match &row.patterns[col] {
                ElabPattern::Wild | ElabPattern::Var(_, _, _) => {
                    let new_pats: Vec<ElabPattern> = row
                        .patterns
                        .iter()
                        .enumerate()
                        .filter(|(j, _)| *j != col)
                        .map(|(_, p)| p.clone())
                        .collect();
                    new_rows.push(MatrixRow {
                        patterns: new_pats,
                        rhs_idx: row.rhs_idx,
                    });
                }
                _ => {}
            }
        }
        let num_cols = if self.num_cols > 0 {
            self.num_cols - 1
        } else {
            0
        };
        PatternMatrix {
            rows: new_rows,
            num_cols,
        }
    }
}
/// Normalizes patterns by removing redundant wrappers.
#[allow(dead_code)]
pub struct PatternNormalizer;
#[allow(dead_code)]
impl PatternNormalizer {
    /// Create a new normalizer.
    pub fn new() -> Self {
        Self
    }
    /// Normalize an elaborated pattern.
    pub fn normalize(&self, pat: &ElabPattern) -> ElabPattern {
        match pat {
            ElabPattern::As(_, _, inner) if is_irrefutable(inner) => self.normalize(inner),
            ElabPattern::Or(a, b) if a == b => self.normalize(a),
            ElabPattern::Ctor(name, sub, ty) => {
                let normalized_sub: Vec<ElabPattern> =
                    sub.iter().map(|s| self.normalize(s)).collect();
                ElabPattern::Ctor(name.clone(), normalized_sub, ty.clone())
            }
            ElabPattern::Or(a, b) => {
                ElabPattern::Or(Box::new(self.normalize(a)), Box::new(self.normalize(b)))
            }
            ElabPattern::As(fv, n, inner) => {
                ElabPattern::As(*fv, n.clone(), Box::new(self.normalize(inner)))
            }
            other => other.clone(),
        }
    }
    /// Normalize a list of patterns in place.
    pub fn normalize_all(&self, patterns: &[ElabPattern]) -> Vec<ElabPattern> {
        patterns.iter().map(|p| self.normalize(p)).collect()
    }
}
/// Analyzes the structure of a compiled decision tree.
#[allow(dead_code)]
pub struct DecisionTreeAnalyzer;
#[allow(dead_code)]
impl DecisionTreeAnalyzer {
    /// Estimate the depth of the decision tree.
    pub fn estimate_depth(tree: &DecisionTree) -> usize {
        Self::node_depth(&tree.root)
    }
    /// Estimate the total number of nodes in the tree.
    pub fn estimate_nodes(tree: &DecisionTree) -> usize {
        Self::count_nodes(&tree.root)
    }
    /// Return true if the tree has a default (wildcard) arm.
    pub fn has_default_arm(tree: &DecisionTree) -> bool {
        Self::node_has_default(&tree.root)
    }
    /// Return the number of arms (leaf nodes) in the tree.
    pub fn num_arms(tree: &DecisionTree) -> usize {
        Self::count_leaves(&tree.root)
    }
    fn node_depth(node: &DecisionNode) -> usize {
        match node {
            DecisionNode::Leaf(_) | DecisionNode::Fail => 1,
            DecisionNode::Switch {
                branches, default, ..
            } => {
                let branch_max = branches
                    .iter()
                    .map(|(_, b)| Self::node_depth(b))
                    .max()
                    .unwrap_or(0);
                let def_depth = default.as_ref().map_or(0, |d| Self::node_depth(d));
                1 + branch_max.max(def_depth)
            }
            DecisionNode::Guard { then, else_, .. } => {
                1 + Self::node_depth(then).max(Self::node_depth(else_))
            }
        }
    }
    fn count_nodes(node: &DecisionNode) -> usize {
        match node {
            DecisionNode::Leaf(_) | DecisionNode::Fail => 1,
            DecisionNode::Switch {
                branches, default, ..
            } => {
                let sum: usize = branches.iter().map(|(_, b)| Self::count_nodes(b)).sum();
                1 + sum + default.as_ref().map_or(0, |d| Self::count_nodes(d))
            }
            DecisionNode::Guard { then, else_, .. } => {
                1 + Self::count_nodes(then) + Self::count_nodes(else_)
            }
        }
    }
    fn node_has_default(node: &DecisionNode) -> bool {
        match node {
            DecisionNode::Leaf(_) | DecisionNode::Fail => false,
            DecisionNode::Switch {
                branches, default, ..
            } => {
                if default.is_some() {
                    return true;
                }
                branches.iter().any(|(_, b)| Self::node_has_default(b))
            }
            DecisionNode::Guard { then, else_, .. } => {
                Self::node_has_default(then) || Self::node_has_default(else_)
            }
        }
    }
    fn count_leaves(node: &DecisionNode) -> usize {
        match node {
            DecisionNode::Leaf(_) => 1,
            DecisionNode::Fail => 0,
            DecisionNode::Switch {
                branches, default, ..
            } => {
                let sum: usize = branches.iter().map(|(_, b)| Self::count_leaves(b)).sum();
                sum + default.as_ref().map_or(0, |d| Self::count_leaves(d))
            }
            DecisionNode::Guard { then, else_, .. } => {
                Self::count_leaves(then) + Self::count_leaves(else_)
            }
        }
    }
}
/// A node in the decision tree.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum DecisionNode {
    /// Leaf node: produce this expression.
    Leaf(Expr),
    /// Switch on constructor of a variable.
    Switch {
        /// Variable to switch on.
        var: Name,
        /// Branches for each constructor.
        branches: Vec<(Name, Box<DecisionNode>)>,
        /// Default branch (for wildcard / variable patterns).
        default: Option<Box<DecisionNode>>,
    },
    /// Guard check.
    Guard {
        /// Guard condition expression.
        condition: Expr,
        /// Branch if guard evaluates to true.
        then: Box<DecisionNode>,
        /// Branch if guard evaluates to false.
        else_: Box<DecisionNode>,
    },
    /// Failure: no pattern matched (unreachable if exhaustive).
    Fail,
}
/// Tracks which literal values appear in a set of patterns.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LiteralSet {
    /// Natural number literals seen.
    pub nats: Vec<u64>,
    /// String literals seen.
    pub strings: Vec<String>,
    /// Whether a wildcard has been seen (closes the literal set).
    pub has_wildcard: bool,
}
#[allow(dead_code)]
impl LiteralSet {
    /// Create an empty literal set.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a literal to the set.
    pub fn add_literal(&mut self, lit: &oxilean_kernel::Literal) {
        match lit {
            oxilean_kernel::Literal::Nat(n) => {
                if !self.nats.contains(n) {
                    self.nats.push(*n);
                }
            }
            oxilean_kernel::Literal::Str(s) => {
                if !self.strings.contains(s) {
                    self.strings.push(s.clone());
                }
            }
        }
    }
    /// Mark that a wildcard was seen.
    pub fn add_wildcard(&mut self) {
        self.has_wildcard = true;
    }
    /// Check if a specific natural number literal is covered.
    pub fn covers_nat(&self, n: u64) -> bool {
        self.has_wildcard || self.nats.contains(&n)
    }
    /// Return the total number of specific literals seen.
    pub fn num_specific(&self) -> usize {
        self.nats.len() + self.strings.len()
    }
}
/// Classification of an elaborated pattern.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PatternKind {
    /// Wildcard pattern.
    Wild,
    /// Variable binding.
    Variable,
    /// Constructor application.
    Constructor,
    /// Literal value.
    Literal,
    /// Or-pattern.
    Or,
    /// As-pattern.
    As,
    /// Inaccessible (dot) pattern.
    Inaccessible,
}
#[allow(dead_code)]
impl PatternKind {
    /// Classify an elaborated pattern.
    pub fn of(pat: &ElabPattern) -> Self {
        match pat {
            ElabPattern::Wild => PatternKind::Wild,
            ElabPattern::Var(_, _, _) => PatternKind::Variable,
            ElabPattern::Ctor(_, _, _) => PatternKind::Constructor,
            ElabPattern::Lit(_) => PatternKind::Literal,
            ElabPattern::Or(_, _) => PatternKind::Or,
            ElabPattern::As(_, _, _) => PatternKind::As,
            ElabPattern::Inaccessible(_) => PatternKind::Inaccessible,
        }
    }
    /// Return true if this pattern kind can bind variables.
    pub fn can_bind(&self) -> bool {
        matches!(self, PatternKind::Variable | PatternKind::As)
    }
    /// Return true if this pattern kind is a "catch-all" (always matches).
    pub fn is_catch_all(&self) -> bool {
        matches!(self, PatternKind::Wild | PatternKind::Variable)
    }
}
/// Result of exhaustiveness checking.
#[derive(Clone, Debug)]
pub struct ExhaustivenessResult {
    /// Whether the patterns are exhaustive.
    pub is_exhaustive: bool,
    /// Missing constructors/patterns if not exhaustive.
    pub missing: Vec<MissingPattern>,
}
/// A single row in the pattern matrix.
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct MatrixRow {
    pub patterns: Vec<ElabPattern>,
    pub rhs_idx: usize,
}
/// Tracks which match arms have been covered by test inputs.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MatchCoverage {
    /// Number of arms in the match.
    pub num_arms: usize,
    /// Which arms have been hit.
    pub hit: Vec<bool>,
}
#[allow(dead_code)]
impl MatchCoverage {
    /// Create a coverage tracker for a match with `n` arms.
    pub fn new(n: usize) -> Self {
        Self {
            num_arms: n,
            hit: vec![false; n],
        }
    }
    /// Record that arm `i` was hit.
    pub fn record_hit(&mut self, i: usize) {
        if i < self.num_arms {
            self.hit[i] = true;
        }
    }
    /// Return the number of arms hit.
    pub fn arms_hit(&self) -> usize {
        self.hit.iter().filter(|&&h| h).count()
    }
    /// Return true if all arms have been hit.
    pub fn is_full_coverage(&self) -> bool {
        self.hit.iter().all(|&h| h)
    }
    /// Return the indices of arms that have NOT been hit.
    pub fn uncovered_arms(&self) -> Vec<usize> {
        self.hit
            .iter()
            .enumerate()
            .filter(|(_, &h)| !h)
            .map(|(i, _)| i)
            .collect()
    }
    /// Coverage percentage (0.0 to 1.0).
    pub fn coverage_pct(&self) -> f64 {
        if self.num_arms == 0 {
            1.0
        } else {
            self.arms_hit() as f64 / self.num_arms as f64
        }
    }
}
/// Pretty-printer for elaborated patterns.
#[allow(dead_code)]
pub struct PatternPrinter {
    /// Indent width for nested patterns.
    pub indent: usize,
}
#[allow(dead_code)]
impl PatternPrinter {
    /// Create a new printer with default indent of 2.
    pub fn new() -> Self {
        Self { indent: 2 }
    }
    /// Create a printer with a custom indent.
    pub fn with_indent(indent: usize) -> Self {
        Self { indent }
    }
    /// Print an elaborated pattern to a string.
    pub fn print(&self, pat: &ElabPattern) -> String {
        self.print_inner(pat, 0)
    }
    fn print_inner(&self, pat: &ElabPattern, _depth: usize) -> String {
        match pat {
            ElabPattern::Wild => "_".to_string(),
            ElabPattern::Var(_, name, _) => format!("{}", name),
            ElabPattern::Lit(lit) => match lit {
                oxilean_kernel::Literal::Nat(n) => format!("{}", n),
                oxilean_kernel::Literal::Str(s) => format!("\"{}\"", s),
            },
            ElabPattern::Ctor(name, sub, _) => {
                if sub.is_empty() {
                    format!("{}", name)
                } else {
                    let args: Vec<String> = sub
                        .iter()
                        .map(|s| self.print_inner(s, _depth + 1))
                        .collect();
                    format!("({} {})", name, args.join(" "))
                }
            }
            ElabPattern::Or(a, b) => {
                format!(
                    "({} | {})",
                    self.print_inner(a, _depth),
                    self.print_inner(b, _depth)
                )
            }
            ElabPattern::As(_, name, inner) => {
                format!("({} @ {})", self.print_inner(inner, _depth), name)
            }
            ElabPattern::Inaccessible(e) => format!(".({})", e),
        }
    }
    /// Print a list of patterns separated by newlines.
    pub fn print_all(&self, patterns: &[ElabPattern]) -> String {
        patterns
            .iter()
            .enumerate()
            .map(|(i, p)| format!("  arm {}: {}", i, self.print(p)))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
/// Pattern match compiler.
///
/// Compiles surface-level patterns into decision trees for efficient matching.
pub struct PatternCompiler {
    /// Fresh variable counter.
    pub(crate) next_var: u32,
}
impl PatternCompiler {
    /// Create a new pattern compiler.
    pub fn new() -> Self {
        Self { next_var: 0 }
    }
    /// Generate a fresh variable name.
    pub fn fresh_var(&mut self) -> Name {
        let n = self.next_var;
        self.next_var += 1;
        Name::str(format!("_pat{}", n))
    }
    /// Compile surface-level patterns to a decision tree.
    pub fn compile(
        &mut self,
        patterns: &[(Located<Pattern>, Located<SurfaceExpr>)],
    ) -> Result<DecisionTree, String> {
        DecisionTree::new(patterns)
    }
    /// Compile from elaborated match equations.
    #[allow(dead_code)]
    pub fn compile_equations(
        &mut self,
        equations: &[MatchEquation],
    ) -> Result<DecisionTree, String> {
        DecisionTree::from_equations(equations)
    }
    /// Compile from match arms (with guards).
    #[allow(dead_code)]
    pub fn from_match_arms(
        &mut self,
        ctx: &mut ElabContext,
        _scrutinee_ty: &Expr,
        arms: &[MatchArm],
    ) -> Result<(DecisionTree, MatchResult), String> {
        let mut equations = Vec::new();
        for (i, arm) in arms.iter().enumerate() {
            let expected_ty = Expr::Sort(Level::succ(Level::zero()));
            let (elab_pat, _bindings) = elaborate_pattern(ctx, &arm.pattern.value, &expected_ty)?;
            let rhs = surface_to_placeholder(&arm.rhs.value);
            let guard_expr = arm.guard.as_ref().map(|g| surface_to_placeholder(&g.value));
            let _ = guard_expr;
            equations.push(MatchEquation {
                patterns: vec![elab_pat],
                rhs,
                arm_idx: i,
            });
        }
        let tree = DecisionTree::from_equations(&equations)?;
        let expr = tree.compile();
        let patterns_located: Vec<Located<Pattern>> =
            arms.iter().map(|a| a.pattern.clone()).collect();
        let exhaustiveness = check_exhaustive_full(&patterns_located);
        let redundant = check_redundant_full(&patterns_located);
        let result = MatchResult {
            expr,
            defs: Vec::new(),
            equations,
            missing_patterns: exhaustiveness.missing,
            redundant_arms: redundant,
        };
        Ok((tree, result))
    }
}
/// A missing pattern discovered during exhaustiveness checking.
#[derive(Clone, Debug)]
pub struct MissingPattern {
    /// Constructor name (or "wildcard" for general missing).
    pub ctor_name: Name,
    /// Sub-patterns within the constructor.
    pub sub_patterns: Vec<ElabPattern>,
}
/// Simulates runtime pattern matching against an elaborated pattern.
///
/// This is used for testing and verification, not for actual codegen.
#[allow(dead_code)]
pub struct PatternMatcher {
    /// Whether to allow partial matches (for debugging).
    pub partial_ok: bool,
}
#[allow(dead_code)]
impl PatternMatcher {
    /// Create a new pattern matcher.
    pub fn new() -> Self {
        Self { partial_ok: false }
    }
    /// Check if a pattern would match a given literal expression.
    pub fn matches_literal(&self, pat: &ElabPattern, lit: &oxilean_kernel::Literal) -> bool {
        match pat {
            ElabPattern::Wild | ElabPattern::Var(_, _, _) => true,
            ElabPattern::Lit(l) => l == lit,
            ElabPattern::Or(a, b) => self.matches_literal(a, lit) || self.matches_literal(b, lit),
            ElabPattern::As(_, _, inner) => self.matches_literal(inner, lit),
            ElabPattern::Ctor(_, _, _) | ElabPattern::Inaccessible(_) => false,
        }
    }
    /// Find the first arm index that matches a given literal.
    pub fn first_match_idx(
        &self,
        arms: &[ElabPattern],
        lit: &oxilean_kernel::Literal,
    ) -> Option<usize> {
        arms.iter().position(|p| self.matches_literal(p, lit))
    }
    /// Return all arms that match a given literal.
    pub fn all_match_idxs(
        &self,
        arms: &[ElabPattern],
        lit: &oxilean_kernel::Literal,
    ) -> Vec<usize> {
        arms.iter()
            .enumerate()
            .filter(|(_, p)| self.matches_literal(p, lit))
            .map(|(i, _)| i)
            .collect()
    }
}
