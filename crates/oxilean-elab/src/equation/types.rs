//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{Expr, Name};

#[allow(dead_code)]
pub struct EquationNormalizer {
    desugar_or_patterns: bool,
    flatten_as_patterns: bool,
    sort_by_specificity: bool,
}
#[allow(dead_code, clippy::new_without_default)]
impl EquationNormalizer {
    pub fn new() -> Self {
        EquationNormalizer {
            desugar_or_patterns: true,
            flatten_as_patterns: false,
            sort_by_specificity: true,
        }
    }
    pub fn with_desugar_or(mut self, v: bool) -> Self {
        self.desugar_or_patterns = v;
        self
    }
    pub fn with_flatten_as(mut self, v: bool) -> Self {
        self.flatten_as_patterns = v;
        self
    }
    pub fn normalize(&self, equations: &[Equation]) -> Vec<Equation> {
        let mut result = Vec::new();
        for eq in equations {
            if self.desugar_or_patterns && Self::has_or_pattern(eq) {
                let desugared = Self::desugar_or(eq);
                result.extend(desugared);
            } else {
                result.push(eq.clone());
            }
        }
        if self.sort_by_specificity {
            result.sort_by(|a, b| {
                let ca: u32 = a.patterns.iter().map(pattern_complexity).sum();
                let cb: u32 = b.patterns.iter().map(pattern_complexity).sum();
                cb.cmp(&ca)
            });
        }
        result
    }
    fn has_or_pattern(eq: &Equation) -> bool {
        eq.patterns.iter().any(|p| matches!(p, Pattern::Or(_, _)))
    }
    fn desugar_or(eq: &Equation) -> Vec<Equation> {
        for (i, p) in eq.patterns.iter().enumerate() {
            if let Pattern::Or(left, right) = p {
                let mut eq_left = eq.clone();
                let mut eq_right = eq.clone();
                eq_left.patterns[i] = *left.clone();
                eq_right.patterns[i] = *right.clone();
                let mut result = Self::desugar_or(&eq_left);
                result.extend(Self::desugar_or(&eq_right));
                return result;
            }
        }
        vec![eq.clone()]
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExhaustCheck {
    Exhaustive,
    NonExhaustive(Vec<String>),
    Unknown,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct EquationCompilerStats {
    pub equations_processed: usize,
    pub redundant_equations: usize,
    pub or_patterns_desugared: usize,
    pub as_patterns_encountered: usize,
    pub max_matrix_depth: usize,
    pub columns_selected: Vec<usize>,
}
#[allow(dead_code)]
impl EquationCompilerStats {
    pub fn new() -> Self {
        EquationCompilerStats::default()
    }
    pub fn record_processed(&mut self, n: usize) {
        self.equations_processed += n;
    }
    pub fn record_redundant(&mut self, n: usize) {
        self.redundant_equations += n;
    }
    pub fn record_or_desugar(&mut self) {
        self.or_patterns_desugared += 1;
    }
    pub fn record_as_pattern(&mut self) {
        self.as_patterns_encountered += 1;
    }
    pub fn record_depth(&mut self, depth: usize) {
        if depth > self.max_matrix_depth {
            self.max_matrix_depth = depth;
        }
    }
    pub fn record_column_selection(&mut self, col: usize) {
        self.columns_selected.push(col);
    }
    pub fn redundancy_rate(&self) -> f64 {
        if self.equations_processed == 0 {
            return 0.0;
        }
        self.redundant_equations as f64 / self.equations_processed as f64
    }
    pub fn summary(&self) -> String {
        format!(
            "processed={} redundant={} or_desugar={} as_patterns={} max_depth={}",
            self.equations_processed,
            self.redundant_equations,
            self.or_patterns_desugared,
            self.as_patterns_encountered,
            self.max_matrix_depth,
        )
    }
}
/// Result of an exhaustiveness check.
#[derive(Debug, Clone, PartialEq)]
pub enum ExhaustivenessResult {
    /// All cases are covered.
    Exhaustive,
    /// Some patterns are missing.
    Missing(Vec<Pattern>),
    /// Some patterns are redundant.
    Redundant(Vec<usize>),
}
/// Equation compiler.
pub struct EquationCompiler {
    /// Equations to compile
    pub(crate) equations: Vec<Equation>,
}
impl EquationCompiler {
    /// Create a new equation compiler.
    pub fn new(equations: Vec<Equation>) -> Self {
        Self { equations }
    }
    /// Compile equations to a decision tree.
    pub fn compile(&self) -> Result<DecisionTree, String> {
        if self.equations.is_empty() {
            return Ok(DecisionTree::Fail);
        }
        self.compile_equations(&self.equations, 0)
    }
    fn compile_equations(&self, eqs: &[Equation], depth: usize) -> Result<DecisionTree, String> {
        if eqs.is_empty() {
            return Ok(DecisionTree::Fail);
        }
        if eqs.iter().all(|eq| eq.patterns.is_empty()) {
            return Ok(DecisionTree::Leaf(eqs[0].rhs.clone()));
        }
        let mut ctor_groups: Vec<(Name, Vec<Equation>)> = Vec::new();
        let mut wild_eqs: Vec<Equation> = Vec::new();
        for eq in eqs {
            match eq.patterns.first() {
                Some(Pattern::Ctor(name, _)) => {
                    if let Some((_, group)) = ctor_groups.iter_mut().find(|(n, _)| n == name) {
                        group.push(eq.clone());
                    } else {
                        ctor_groups.push((name.clone(), vec![eq.clone()]));
                    }
                }
                _ => {
                    wild_eqs.push(Equation {
                        patterns: eq.patterns.get(1..).unwrap_or(&[]).to_vec(),
                        rhs: eq.rhs.clone(),
                        guard: eq.guard.clone(),
                        source_loc: eq.source_loc,
                    });
                }
            }
        }
        if !ctor_groups.is_empty() {
            let mut cases = Vec::new();
            for (name, group) in &ctor_groups {
                let mut specialized = specialize_for_ctor(group, name);
                specialized.extend_from_slice(&wild_eqs);
                let subtree = self.compile_equations(&specialized, depth + 1)?;
                cases.push((name.clone(), subtree));
            }
            let default = if !wild_eqs.is_empty() {
                Some(Box::new(self.compile_equations(&wild_eqs, depth + 1)?))
            } else {
                None
            };
            Ok(DecisionTree::Switch {
                var: depth,
                cases,
                default,
            })
        } else if !wild_eqs.is_empty() {
            self.compile_equations(&wild_eqs, depth + 1)
        } else {
            Ok(DecisionTree::Fail)
        }
    }
    /// Check if equations are exhaustive.
    ///
    /// Returns `true` if every possible value is matched by at least one equation.
    /// Uses the pattern usefulness algorithm: equations are exhaustive when no
    /// additional wildcard pattern would be "useful" (i.e. every row is already covered).
    ///
    /// Without a full type environment we conservatively check:
    /// 1. Empty equations → not exhaustive.
    /// 2. Any fully-irrefutable row → exhaustive (wildcard covers all cases).
    /// 3. A default (wildcard/variable) in the first column → exhaustive.
    /// 4. Otherwise, every constructor that appears in the first column must have
    ///    an exhaustive specialised sub-matrix, AND there must be no gaps.
    ///    Since we cannot enumerate all constructors of a type without the
    ///    environment, step 4 reports exhaustive only when a wildcard/default row
    ///    is present (conservative but sound).
    pub fn check_exhaustive(&self) -> bool {
        if self.equations.is_empty() {
            return false;
        }
        Self::is_exhaustive_matrix(&self.equations)
    }
    /// Recursive helper: check whether a slice of equations covers all cases.
    fn is_exhaustive_matrix(eqs: &[Equation]) -> bool {
        if eqs.is_empty() {
            return false;
        }
        if eqs[0].patterns.iter().all(|p| p.is_irrefutable()) {
            return true;
        }
        if eqs
            .iter()
            .any(|eq| eq.patterns.iter().all(|p| p.is_irrefutable()))
        {
            return true;
        }
        let has_default_col0 = eqs.iter().any(|eq| {
            eq.patterns
                .first()
                .map(|p| p.is_irrefutable())
                .unwrap_or(true)
        });
        if has_default_col0 {
            let default_eqs: Vec<Equation> = eqs
                .iter()
                .filter(|eq| {
                    eq.patterns
                        .first()
                        .map(|p| p.is_irrefutable())
                        .unwrap_or(true)
                })
                .map(|eq| Equation {
                    patterns: eq.patterns.get(1..).unwrap_or(&[]).to_vec(),
                    rhs: eq.rhs.clone(),
                    guard: eq.guard.clone(),
                    source_loc: eq.source_loc,
                })
                .collect();
            return Self::is_exhaustive_matrix(&default_eqs)
                || default_eqs.iter().all(|e| e.patterns.is_empty());
        }
        false
    }
    /// Check for redundant equations.
    ///
    /// An equation is redundant if every value it could match is already matched
    /// by some earlier equation.  We detect two cases:
    ///
    /// 1. A catch-all (fully-irrefutable) row makes every subsequent row redundant.
    /// 2. A row whose first-column constructor already has a fully-irrefutable
    ///    earlier row in the same constructor group is redundant.
    /// 3. An exact duplicate constructor at the same arity with identical patterns
    ///    is redundant.
    pub fn check_redundant(&self) -> Vec<usize> {
        find_redundant_equations(&self.equations)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FullPatternMatrix {
    /// Each row is one equation: a list of patterns + optional guard + rhs expr
    rows: Vec<FullPatternRow>,
    /// Number of columns (scrutinee arity)
    arity: usize,
}
#[allow(dead_code)]
impl FullPatternMatrix {
    pub fn new(arity: usize) -> Self {
        FullPatternMatrix {
            rows: Vec::new(),
            arity,
        }
    }
    pub fn from_equations(equations: &[Equation]) -> Option<Self> {
        let arity = equations.first()?.patterns.len();
        let mut matrix = FullPatternMatrix::new(arity);
        for (i, eq) in equations.iter().enumerate() {
            matrix.rows.push(FullPatternRow {
                patterns: eq.patterns.clone(),
                guard: eq.guard.clone(),
                rhs: eq.rhs.clone(),
                origin: i,
            });
        }
        Some(matrix)
    }
    pub fn num_rows(&self) -> usize {
        self.rows.len()
    }
    pub fn arity(&self) -> usize {
        self.arity
    }
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
    /// Get the first column of patterns
    pub fn first_column(&self) -> Vec<&Pattern> {
        self.rows.iter().map(|r| &r.patterns[0]).collect()
    }
    /// Swap columns i and j in every row
    pub fn swap_columns(&mut self, i: usize, j: usize) {
        for row in &mut self.rows {
            row.patterns.swap(i, j);
        }
    }
    /// Specialize: keep only rows where first pattern matches constructor `ctor` with arity `k`
    /// and expand those k sub-patterns into the front
    pub fn specialize(&self, ctor: &Name, k: usize) -> FullPatternMatrix {
        let mut result = FullPatternMatrix::new(self.arity - 1 + k);
        for row in &self.rows {
            match &row.patterns[0] {
                Pattern::Ctor(name, args) if name == ctor => {
                    let mut new_patterns = args.clone();
                    new_patterns.extend_from_slice(&row.patterns[1..]);
                    result.rows.push(FullPatternRow {
                        patterns: new_patterns,
                        guard: row.guard.clone(),
                        rhs: row.rhs.clone(),
                        origin: row.origin,
                    });
                }
                Pattern::Wild | Pattern::Var(_) => {
                    let mut new_patterns = vec![Pattern::Wild; k];
                    new_patterns.extend_from_slice(&row.patterns[1..]);
                    result.rows.push(FullPatternRow {
                        patterns: new_patterns,
                        guard: row.guard.clone(),
                        rhs: row.rhs.clone(),
                        origin: row.origin,
                    });
                }
                Pattern::As(_, inner) => {
                    if let Pattern::Ctor(name, args) = inner.as_ref() {
                        if name == ctor {
                            let mut new_patterns = args.clone();
                            new_patterns.extend_from_slice(&row.patterns[1..]);
                            result.rows.push(FullPatternRow {
                                patterns: new_patterns,
                                guard: row.guard.clone(),
                                rhs: row.rhs.clone(),
                                origin: row.origin,
                            });
                        }
                    }
                }
                _ => {}
            }
        }
        result
    }
    /// Default matrix: keep only rows where first pattern is a wildcard/variable
    pub fn default_matrix(&self) -> FullPatternMatrix {
        let new_arity = if self.arity > 0 { self.arity - 1 } else { 0 };
        let mut result = FullPatternMatrix::new(new_arity);
        for row in &self.rows {
            match &row.patterns[0] {
                Pattern::Wild | Pattern::Var(_) => {
                    let new_patterns = row.patterns[1..].to_vec();
                    result.rows.push(FullPatternRow {
                        patterns: new_patterns,
                        guard: row.guard.clone(),
                        rhs: row.rhs.clone(),
                        origin: row.origin,
                    });
                }
                _ => {}
            }
        }
        result
    }
    /// Collect all constructor names appearing in the first column
    pub fn head_constructors(&self) -> Vec<Name> {
        let mut ctors = Vec::new();
        for row in &self.rows {
            if let Pattern::Ctor(name, _) = &row.patterns[0] {
                if !ctors.contains(name) {
                    ctors.push(name.clone());
                }
            }
        }
        ctors
    }
}
#[allow(dead_code)]
#[derive(Default)]
pub struct OverlapChecker;
#[allow(dead_code)]
impl OverlapChecker {
    pub fn new() -> Self {
        OverlapChecker
    }
    /// Check whether pattern `a` is subsumed by pattern `b` (b covers everything a covers)
    pub fn subsumes(b: &Pattern, a: &Pattern) -> bool {
        match (b, a) {
            (Pattern::Wild, _) | (Pattern::Var(_), _) => true,
            (Pattern::Ctor(nb, argsb), Pattern::Ctor(na, argsa)) => {
                nb == na
                    && argsb.len() == argsa.len()
                    && argsb
                        .iter()
                        .zip(argsa.iter())
                        .all(|(pb, pa)| Self::subsumes(pb, pa))
            }
            (Pattern::Lit(lb), Pattern::Lit(la)) => lb == la,
            (Pattern::Or(b1, b2), a) => Self::subsumes(b1, a) || Self::subsumes(b2, a),
            (b, Pattern::Or(a1, a2)) => Self::subsumes(b, a1) && Self::subsumes(b, a2),
            (Pattern::As(_, inner), a) => Self::subsumes(inner, a),
            _ => false,
        }
    }
    /// Find pairs of equations where the later one is fully subsumed by the earlier one
    pub fn find_redundant_pairs(&self, equations: &[Equation]) -> Vec<OverlapPair> {
        let mut pairs = Vec::new();
        for i in 0..equations.len() {
            for j in (i + 1)..equations.len() {
                let ei = &equations[i];
                let ej = &equations[j];
                if ei.patterns.len() != ej.patterns.len() {
                    continue;
                }
                let all_subsumed = ei
                    .patterns
                    .iter()
                    .zip(ej.patterns.iter())
                    .all(|(pi, pj)| Self::subsumes(pi, pj));
                if all_subsumed {
                    pairs.push(OverlapPair {
                        first: i,
                        second: j,
                        column: 0,
                    });
                }
            }
        }
        pairs
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PatternAnnotation {
    pub kind: PatternAnnotationKind,
    pub note: Option<String>,
}
#[allow(dead_code)]
impl PatternAnnotation {
    pub fn new(kind: PatternAnnotationKind) -> Self {
        PatternAnnotation { kind, note: None }
    }
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }
    pub fn is_irrefutable(&self) -> bool {
        self.kind == PatternAnnotationKind::Irrefutable
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FullPatternRow {
    pub patterns: Vec<Pattern>,
    pub guard: Option<Expr>,
    pub rhs: Expr,
    pub origin: usize,
}
#[allow(dead_code)]
pub struct EquationCompilationReport {
    pub equation_count: usize,
    pub redundant_indices: Vec<usize>,
    pub missing_cases: Vec<Name>,
    pub tree_stats: DecisionTreeStats,
    pub warnings: Vec<String>,
}
#[allow(dead_code)]
impl EquationCompilationReport {
    pub fn new(equation_count: usize) -> Self {
        EquationCompilationReport {
            equation_count,
            redundant_indices: Vec::new(),
            missing_cases: Vec::new(),
            tree_stats: DecisionTreeStats::new(),
            warnings: Vec::new(),
        }
    }
    pub fn has_redundancy(&self) -> bool {
        !self.redundant_indices.is_empty()
    }
    pub fn is_exhaustive(&self) -> bool {
        self.missing_cases.is_empty()
    }
    pub fn add_warning(&mut self, w: impl Into<String>) {
        self.warnings.push(w.into());
    }
    pub fn warning_count(&self) -> usize {
        self.warnings.len()
    }
}
/// A compiled equation from pattern matching.
#[derive(Debug, Clone)]
pub struct Equation {
    /// Patterns for this equation
    pub patterns: Vec<Pattern>,
    /// Right-hand side expression
    pub rhs: Expr,
    /// Optional guard condition
    pub guard: Option<Expr>,
    /// Source line number (for diagnostics), if known
    pub source_loc: Option<u32>,
}
impl Equation {
    /// Create a new equation with given patterns and RHS.
    pub fn new(patterns: Vec<Pattern>, rhs: Expr) -> Self {
        Self {
            patterns,
            rhs,
            guard: None,
            source_loc: None,
        }
    }
    /// Attach a guard condition.
    pub fn with_guard(mut self, guard: Expr) -> Self {
        self.guard = Some(guard);
        self
    }
    /// Mark this equation with a source line number.
    pub fn at_line(mut self, line: u32) -> Self {
        self.source_loc = Some(line);
        self
    }
    /// Consume the first pattern, returning the rest of the equation.
    ///
    /// Returns `None` if there are no patterns.
    pub fn consume_first(&self) -> Option<(Pattern, Equation)> {
        if self.patterns.is_empty() {
            return None;
        }
        let first = self.patterns[0].clone();
        let rest = Equation {
            patterns: self.patterns[1..].to_vec(),
            rhs: self.rhs.clone(),
            guard: self.guard.clone(),
            source_loc: self.source_loc,
        };
        Some((first, rest))
    }
    /// Check if this equation has a guard.
    pub fn has_guard(&self) -> bool {
        self.guard.is_some()
    }
    /// Return the number of patterns.
    pub fn arity(&self) -> usize {
        self.patterns.len()
    }
}
/// A matrix of patterns used during compilation.
///
/// Each row is an equation, each column is a scrutinee variable.
#[derive(Debug, Clone, Default)]
pub struct PatternMatrix {
    /// Rows of the matrix (equations).
    rows: Vec<Equation>,
}
impl PatternMatrix {
    /// Create a new empty pattern matrix.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a row (equation) to the matrix.
    pub fn add_row(&mut self, eq: Equation) {
        self.rows.push(eq);
    }
    /// Return the number of rows.
    pub fn num_rows(&self) -> usize {
        self.rows.len()
    }
    /// Return the number of columns (arity of first row).
    pub fn num_cols(&self) -> usize {
        self.rows.first().map(|r| r.arity()).unwrap_or(0)
    }
    /// Check if the matrix is empty.
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
    /// Return an iterator over the rows.
    pub fn rows(&self) -> &[Equation] {
        &self.rows
    }
    /// Select a column (the i-th pattern from each row).
    pub fn column(&self, i: usize) -> Vec<Option<&Pattern>> {
        self.rows.iter().map(|row| row.patterns.get(i)).collect()
    }
    /// Find the first column containing a constructor pattern.
    ///
    /// Used for the "heuristic column selection" strategy.
    pub fn first_ctor_column(&self) -> Option<usize> {
        let ncols = self.num_cols();
        for col in 0..ncols {
            let has_ctor = self.rows.iter().any(|row| {
                row.patterns
                    .get(col)
                    .map(|p| matches!(p, Pattern::Ctor(_, _)))
                    .unwrap_or(false)
            });
            if has_ctor {
                return Some(col);
            }
        }
        None
    }
}
/// A pattern matrix row: patterns plus the RHS expression.
///
/// Used internally by the equation compiler when flattening
/// nested match constructs into a 2-D matrix.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PatternRow {
    /// Patterns for each column.
    pub patterns: Vec<Pattern>,
    /// Right-hand side.
    pub rhs: Expr,
    /// Optional guard.
    pub guard: Option<Expr>,
    /// Row index (for diagnostics).
    pub row_id: usize,
}
impl PatternRow {
    /// Create a new pattern row.
    #[allow(dead_code)]
    pub fn new(patterns: Vec<Pattern>, rhs: Expr) -> Self {
        Self {
            patterns,
            rhs,
            guard: None,
            row_id: 0,
        }
    }
    /// Set a guard on this row.
    #[allow(dead_code)]
    pub fn with_guard(mut self, guard: Expr) -> Self {
        self.guard = Some(guard);
        self
    }
    /// Number of columns in this row.
    #[allow(dead_code)]
    pub fn width(&self) -> usize {
        self.patterns.len()
    }
    /// Check if all patterns are wildcards or variables (irrefutable).
    #[allow(dead_code)]
    pub fn is_irrefutable(&self) -> bool {
        self.patterns
            .iter()
            .all(|p| matches!(p, Pattern::Wild | Pattern::Var(_)))
    }
    /// Return the first non-wild pattern index, if any.
    #[allow(dead_code)]
    pub fn first_ctor_column(&self) -> Option<usize> {
        self.patterns
            .iter()
            .position(|p| matches!(p, Pattern::Ctor(_, _) | Pattern::Lit(_)))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EquationCompilerConfig {
    pub check_exhaustiveness: bool,
    pub check_redundancy: bool,
    pub sort_by_specificity: bool,
    pub max_depth: usize,
    pub allow_non_exhaustive: bool,
}
#[allow(dead_code)]
impl EquationCompilerConfig {
    pub fn new() -> Self {
        EquationCompilerConfig {
            check_exhaustiveness: true,
            check_redundancy: true,
            sort_by_specificity: true,
            max_depth: 100,
            allow_non_exhaustive: false,
        }
    }
    pub fn permissive(mut self) -> Self {
        self.allow_non_exhaustive = true;
        self.check_redundancy = false;
        self
    }
    pub fn strict(mut self) -> Self {
        self.allow_non_exhaustive = false;
        self.check_redundancy = true;
        self
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OverlapPair {
    pub first: usize,
    pub second: usize,
    pub column: usize,
}
#[allow(dead_code)]
pub struct EquationExtensionMarker;
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColumnHeuristic {
    /// Always pick column 0 (left-to-right)
    LeftToRight,
    /// Pick the column with the most constructors (most informative)
    MostConstructors,
    /// Pick the column with the fewest wildcards (fewest "don't cares")
    FewestWildcards,
    /// Pick the column with the smallest number of distinct ctor arities
    SmallestArity,
}
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct DecisionTreeStats {
    pub leaf_count: usize,
    pub switch_count: usize,
    pub guard_count: usize,
    pub fail_count: usize,
    pub max_depth: usize,
}
#[allow(dead_code)]
impl DecisionTreeStats {
    pub fn new() -> Self {
        DecisionTreeStats::default()
    }
    pub fn analyze(tree: &DecisionTree) -> Self {
        let mut stats = DecisionTreeStats::new();
        stats.count_node(tree, 0);
        stats
    }
    fn count_node(&mut self, tree: &DecisionTree, depth: usize) {
        if depth > self.max_depth {
            self.max_depth = depth;
        }
        match tree {
            DecisionTree::Leaf(_) => self.leaf_count += 1,
            DecisionTree::Fail => self.fail_count += 1,
            DecisionTree::Guard {
                then_branch,
                else_branch,
                ..
            } => {
                self.guard_count += 1;
                self.count_node(then_branch, depth + 1);
                self.count_node(else_branch, depth + 1);
            }
            DecisionTree::Switch { cases, default, .. } => {
                self.switch_count += 1;
                for (_, sub) in cases {
                    self.count_node(sub, depth + 1);
                }
                if let Some(d) = default {
                    self.count_node(d, depth + 1);
                }
            }
        }
    }
    pub fn total_nodes(&self) -> usize {
        self.leaf_count + self.switch_count + self.guard_count + self.fail_count
    }
    pub fn is_simple(&self) -> bool {
        self.switch_count == 0 && self.guard_count == 0
    }
}
/// Pattern in an equation.
#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    /// Wildcard pattern _
    Wild,
    /// Variable pattern
    Var(Name),
    /// Constructor pattern
    Ctor(Name, Vec<Pattern>),
    /// Literal pattern
    Lit(oxilean_kernel::Literal),
    /// As-pattern (x @ p)
    As(Name, Box<Pattern>),
    /// Or-pattern (p1 | p2)
    Or(Box<Pattern>, Box<Pattern>),
}
impl Pattern {
    /// Check if pattern is irrefutable (always matches).
    pub fn is_irrefutable(&self) -> bool {
        matches!(self, Pattern::Wild | Pattern::Var(_))
    }
    /// Get all variables bound by this pattern.
    pub fn bound_vars(&self) -> Vec<Name> {
        match self {
            Pattern::Wild => Vec::new(),
            Pattern::Var(n) => vec![n.clone()],
            Pattern::Ctor(_, pats) => pats.iter().flat_map(|p| p.bound_vars()).collect(),
            Pattern::Lit(_) => Vec::new(),
            Pattern::As(n, p) => {
                let mut vars = vec![n.clone()];
                vars.extend(p.bound_vars());
                vars
            }
            Pattern::Or(p1, p2) => {
                let mut vars = p1.bound_vars();
                vars.extend(p2.bound_vars());
                vars
            }
        }
    }
}
impl Pattern {
    /// Compute the nesting depth of this pattern.
    ///
    /// A wildcard or variable has depth 0. A constructor has depth 1 + max child depth.
    pub fn depth(&self) -> usize {
        match self {
            Pattern::Wild | Pattern::Var(_) | Pattern::Lit(_) => 0,
            Pattern::Ctor(_, pats) => 1 + pats.iter().map(|p| p.depth()).max().unwrap_or(0),
            Pattern::As(_, inner) => inner.depth(),
            Pattern::Or(p1, p2) => p1.depth().max(p2.depth()),
        }
    }
    /// Count the number of constructor occurrences in this pattern.
    pub fn count_ctors(&self) -> usize {
        match self {
            Pattern::Ctor(_, pats) => 1 + pats.iter().map(|p| p.count_ctors()).sum::<usize>(),
            Pattern::As(_, inner) => inner.count_ctors(),
            Pattern::Or(p1, p2) => p1.count_ctors() + p2.count_ctors(),
            _ => 0,
        }
    }
    /// Check if this pattern matches the given constructor name.
    pub fn matches_ctor(&self, name: &Name) -> bool {
        match self {
            Pattern::Ctor(n, _) => n == name,
            Pattern::Or(p1, p2) => p1.matches_ctor(name) || p2.matches_ctor(name),
            Pattern::As(_, inner) => inner.matches_ctor(name),
            _ => false,
        }
    }
    /// Strip an as-pattern, returning the inner pattern.
    pub fn strip_as(&self) -> &Pattern {
        match self {
            Pattern::As(_, inner) => inner.strip_as(),
            other => other,
        }
    }
    /// Check if all branch variables of an or-pattern bind the same names.
    ///
    /// OxiLean requires that `p1 | p2` bind the same variables.
    pub fn or_branches_consistent(&self) -> bool {
        match self {
            Pattern::Or(p1, p2) => {
                let mut v1 = p1.bound_vars();
                let mut v2 = p2.bound_vars();
                v1.sort_by_key(|n| n.to_string());
                v2.sort_by_key(|n| n.to_string());
                v1 == v2
            }
            _ => true,
        }
    }
    /// Replace all wildcards with fresh variable patterns.
    ///
    /// Useful for converting patterns to a canonical form for comparison.
    pub fn wildcards_to_vars(&self, counter: &mut u32) -> Pattern {
        match self {
            Pattern::Wild => {
                let n = Name::str(format!("_w{}", *counter));
                *counter += 1;
                Pattern::Var(n)
            }
            Pattern::Ctor(name, pats) => {
                let new_pats = pats.iter().map(|p| p.wildcards_to_vars(counter)).collect();
                Pattern::Ctor(name.clone(), new_pats)
            }
            Pattern::As(n, inner) => {
                Pattern::As(n.clone(), Box::new(inner.wildcards_to_vars(counter)))
            }
            Pattern::Or(p1, p2) => Pattern::Or(
                Box::new(p1.wildcards_to_vars(counter)),
                Box::new(p2.wildcards_to_vars(counter)),
            ),
            other => other.clone(),
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatternAnnotationKind {
    Irrefutable,
    Refutable,
    Nested,
    HasGuard,
    Redundant,
}
/// Decision tree for efficient pattern matching.
#[derive(Debug, Clone)]
pub enum DecisionTree {
    /// Leaf node - return this expression
    Leaf(Expr),
    /// Switch on a constructor
    Switch {
        /// Variable to scrutinize
        var: usize,
        /// Cases for each constructor
        cases: Vec<(Name, DecisionTree)>,
        /// Default case (if any)
        default: Option<Box<DecisionTree>>,
    },
    /// Guard check
    Guard {
        /// Guard condition
        condition: Expr,
        /// Tree if guard succeeds
        then_branch: Box<DecisionTree>,
        /// Tree if guard fails
        else_branch: Box<DecisionTree>,
    },
    /// Match failure
    Fail,
}
impl DecisionTree {
    /// Check if this decision tree is always exhaustive.
    ///
    /// A tree is exhaustive if it has no `Fail` leaves.
    pub fn is_exhaustive(&self) -> bool {
        match self {
            DecisionTree::Fail => false,
            DecisionTree::Leaf(_) => true,
            DecisionTree::Switch { cases, default, .. } => {
                let cases_ok = cases.iter().all(|(_, t)| t.is_exhaustive());
                let default_ok = default.as_ref().map(|t| t.is_exhaustive()).unwrap_or(false);
                cases_ok || default_ok
            }
            DecisionTree::Guard {
                then_branch,
                else_branch,
                ..
            } => then_branch.is_exhaustive() && else_branch.is_exhaustive(),
        }
    }
    /// Count the number of leaf nodes in this decision tree.
    pub fn num_leaves(&self) -> usize {
        match self {
            DecisionTree::Fail => 1,
            DecisionTree::Leaf(_) => 1,
            DecisionTree::Switch { cases, default, .. } => {
                let c = cases.iter().map(|(_, t)| t.num_leaves()).sum::<usize>();
                let d = default.as_ref().map(|t| t.num_leaves()).unwrap_or(0);
                c + d
            }
            DecisionTree::Guard {
                then_branch,
                else_branch,
                ..
            } => then_branch.num_leaves() + else_branch.num_leaves(),
        }
    }
    /// Compute the depth of this decision tree.
    ///
    /// The depth is the longest path from root to any leaf.
    pub fn depth(&self) -> usize {
        match self {
            DecisionTree::Fail | DecisionTree::Leaf(_) => 0,
            DecisionTree::Switch { cases, default, .. } => {
                let c = cases.iter().map(|(_, t)| t.depth()).max().unwrap_or(0);
                let d = default.as_ref().map(|t| t.depth()).unwrap_or(0);
                1 + c.max(d)
            }
            DecisionTree::Guard {
                then_branch,
                else_branch,
                ..
            } => 1 + then_branch.depth().max(else_branch.depth()),
        }
    }
    /// Collect all leaf expressions from this decision tree.
    pub fn collect_leaves(&self) -> Vec<&Expr> {
        match self {
            DecisionTree::Fail => vec![],
            DecisionTree::Leaf(e) => vec![e],
            DecisionTree::Switch { cases, default, .. } => {
                let mut leaves: Vec<&Expr> =
                    cases.iter().flat_map(|(_, t)| t.collect_leaves()).collect();
                if let Some(d) = default {
                    leaves.extend(d.collect_leaves());
                }
                leaves
            }
            DecisionTree::Guard {
                then_branch,
                else_branch,
                ..
            } => {
                let mut leaves = then_branch.collect_leaves();
                leaves.extend(else_branch.collect_leaves());
                leaves
            }
        }
    }
}
/// A pattern matrix: the 2-D structure used in pattern matching compilation.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PatternMatrix2 {
    /// Rows of the matrix (each is a list of patterns + RHS).
    pub rows: Vec<PatternRow>,
    /// Number of scrutinee columns.
    pub num_columns: usize,
}
impl PatternMatrix2 {
    /// Create a new empty pattern matrix with a given column count.
    #[allow(dead_code)]
    pub fn new(num_columns: usize) -> Self {
        Self {
            rows: Vec::new(),
            num_columns,
        }
    }
    /// Add a row to the matrix.
    #[allow(dead_code)]
    pub fn add_row(&mut self, row: PatternRow) {
        self.rows.push(row);
    }
    /// Number of rows.
    #[allow(dead_code)]
    pub fn height(&self) -> usize {
        self.rows.len()
    }
    /// Check if the matrix is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
    /// Check if the first row is irrefutable (acts as a catchall).
    #[allow(dead_code)]
    pub fn first_row_is_irrefutable(&self) -> bool {
        self.rows.first().is_some_and(|r| r.is_irrefutable())
    }
    /// Return which columns contain any constructor patterns.
    #[allow(dead_code)]
    pub fn ctor_columns(&self) -> Vec<usize> {
        (0..self.num_columns)
            .filter(|&col| {
                self.rows.iter().any(|row| {
                    matches!(
                        row.patterns.get(col),
                        Some(Pattern::Ctor(_, _)) | Some(Pattern::Lit(_))
                    )
                })
            })
            .collect()
    }
    /// Collect all constructor names appearing in column `col`.
    #[allow(dead_code)]
    pub fn ctors_in_column(&self, col: usize) -> Vec<Name> {
        let mut seen: Vec<Name> = Vec::new();
        for row in &self.rows {
            if let Some(Pattern::Ctor(n, _)) = row.patterns.get(col) {
                if !seen.contains(n) {
                    seen.push(n.clone());
                }
            }
        }
        seen
    }
    /// Build a `PatternMatrix` from a list of `Equation`s.
    #[allow(dead_code)]
    pub fn from_equations(eqs: &[Equation]) -> Self {
        let num_cols = eqs.first().map_or(0, |e| e.patterns.len());
        let mut m = Self::new(num_cols);
        for (i, eq) in eqs.iter().enumerate() {
            let mut row = PatternRow::new(eq.patterns.clone(), eq.rhs.clone());
            row.guard = eq.guard.clone();
            row.row_id = i;
            m.add_row(row);
        }
        m
    }
}
#[allow(dead_code)]
pub struct EquationSet {
    equations: Vec<Equation>,
}
#[allow(dead_code)]
impl EquationSet {
    pub fn new(equations: Vec<Equation>) -> Self {
        EquationSet { equations }
    }
    pub fn count(&self) -> usize {
        self.equations.len()
    }
    pub fn arity(&self) -> Option<usize> {
        self.equations.first().map(|e| e.patterns.len())
    }
    pub fn has_wildcards_only(&self) -> bool {
        self.equations.iter().all(|e| {
            e.patterns
                .iter()
                .all(|p| matches!(p, Pattern::Wild | Pattern::Var(_)))
        })
    }
    pub fn has_guards(&self) -> bool {
        self.equations.iter().any(|e| e.guard.is_some())
    }
    pub fn max_depth(&self) -> usize {
        self.equations
            .iter()
            .flat_map(|e| e.patterns.iter())
            .map(pattern_depth)
            .max()
            .unwrap_or(0)
    }
    pub fn equations(&self) -> &[Equation] {
        &self.equations
    }
    pub fn first_equation(&self) -> Option<&Equation> {
        self.equations.first()
    }
    pub fn is_empty(&self) -> bool {
        self.equations.is_empty()
    }
    pub fn push(&mut self, eq: Equation) {
        self.equations.push(eq);
    }
}
#[allow(dead_code)]
pub struct ColumnSelector {
    heuristic: ColumnHeuristic,
}
#[allow(dead_code)]
impl ColumnSelector {
    pub fn new(heuristic: ColumnHeuristic) -> Self {
        ColumnSelector { heuristic }
    }
    pub fn select(&self, matrix: &FullPatternMatrix) -> usize {
        match self.heuristic {
            ColumnHeuristic::LeftToRight => 0,
            ColumnHeuristic::MostConstructors => self.most_constructors(matrix),
            ColumnHeuristic::FewestWildcards => self.fewest_wildcards(matrix),
            ColumnHeuristic::SmallestArity => self.smallest_arity(matrix),
        }
    }
    fn most_constructors(&self, matrix: &FullPatternMatrix) -> usize {
        let mut best_col = 0;
        let mut best_count = 0;
        for col in 0..matrix.arity() {
            let mut ctors = std::collections::HashSet::new();
            for row in &matrix.rows {
                if let Pattern::Ctor(name, _) = &row.patterns[col] {
                    ctors.insert(format!("{:?}", name));
                }
            }
            if ctors.len() > best_count {
                best_count = ctors.len();
                best_col = col;
            }
        }
        best_col
    }
    fn fewest_wildcards(&self, matrix: &FullPatternMatrix) -> usize {
        let mut best_col = 0;
        let mut fewest = usize::MAX;
        for col in 0..matrix.arity() {
            let count = matrix
                .rows
                .iter()
                .filter(|r| matches!(r.patterns[col], Pattern::Wild | Pattern::Var(_)))
                .count();
            if count < fewest {
                fewest = count;
                best_col = col;
            }
        }
        best_col
    }
    fn smallest_arity(&self, matrix: &FullPatternMatrix) -> usize {
        let mut best_col = 0;
        let mut smallest = usize::MAX;
        for col in 0..matrix.arity() {
            let mut max_arity = 0;
            for row in &matrix.rows {
                if let Pattern::Ctor(_, args) = &row.patterns[col] {
                    max_arity = max_arity.max(args.len());
                }
            }
            if max_arity < smallest {
                smallest = max_arity;
                best_col = col;
            }
        }
        best_col
    }
}
#[allow(dead_code)]
pub struct ExhaustivenessAnalyzer {
    max_depth: usize,
}
#[allow(dead_code)]
impl ExhaustivenessAnalyzer {
    pub fn new(max_depth: usize) -> Self {
        ExhaustivenessAnalyzer { max_depth }
    }
    /// Simple exhaustiveness: if any row is all wildcards/vars, it's exhaustive
    pub fn check_simple(&self, equations: &[Equation]) -> ExhaustCheck {
        for eq in equations {
            if eq
                .patterns
                .iter()
                .all(|p| matches!(p, Pattern::Wild | Pattern::Var(_)))
            {
                return ExhaustCheck::Exhaustive;
            }
        }
        ExhaustCheck::NonExhaustive(vec!["_".to_string()])
    }
    /// Check exhaustiveness via the pattern matrix default method
    pub fn check_matrix(&self, matrix: &FullPatternMatrix) -> ExhaustCheck {
        if matrix.is_empty() {
            if matrix.arity() == 0 {
                return ExhaustCheck::Exhaustive;
            }
            return ExhaustCheck::NonExhaustive(vec!["_".to_string()]);
        }
        for row in &matrix.rows {
            if matches!(row.patterns[0], Pattern::Wild | Pattern::Var(_)) {
                let rest_matrix = matrix.default_matrix();
                if rest_matrix.arity() == 0 {
                    return ExhaustCheck::Exhaustive;
                }
                return self.check_matrix(&rest_matrix);
            }
        }
        ExhaustCheck::Unknown
    }
}
