//! # PatternCompiler - queries Methods
//!
//! This module contains method implementations for `PatternCompiler`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::{Located, MatchArm, Pattern, Span, SurfaceExpr};
use std::collections::HashSet;

use super::types::{CaseBranch, CaseTree, MatchClause, PatternRow, TypeConstructors};

use super::patterncompiler_type::PatternCompiler;

impl PatternCompiler {
    /// Create a new pattern compiler.
    pub fn new() -> Self {
        Self { next_var: 0 }
    }
    /// Compile a pattern match to a case tree.
    ///
    /// Builds a `SurfaceExpr::Match` over the given scrutinee with one
    /// arm per clause.  All clauses are included in textual order; the
    /// elaborator handles the actual case-tree construction.
    pub fn compile_match(
        &mut self,
        scrutinee: &SurfaceExpr,
        clauses: &[MatchClause],
    ) -> Result<SurfaceExpr, String> {
        if clauses.is_empty() {
            return Err("Match with no clauses".to_string());
        }
        let dummy = Span::new(0, 0, 0, 0);
        let arms: Vec<MatchArm> = clauses
            .iter()
            .map(|clause| MatchArm {
                pattern: Located::new(clause.pattern.clone(), dummy.clone()),
                guard: None,
                rhs: Located::new(clause.body.clone(), dummy.clone()),
            })
            .collect();
        Ok(SurfaceExpr::Match(
            Box::new(Located::new(scrutinee.clone(), dummy)),
            arms,
        ))
    }
    /// Return the indices of redundant (unreachable) patterns.
    ///
    /// A pattern at index `i` is redundant when some earlier pattern in the
    /// list is irrefutable (wildcard or variable), meaning it would always
    /// match before the pattern at `i` is reached.
    pub fn check_redundant(&self, patterns: &[Pattern]) -> Vec<usize> {
        let mut redundant = Vec::new();
        let mut has_irrefutable = false;
        for (i, pattern) in patterns.iter().enumerate() {
            if has_irrefutable {
                redundant.push(i);
            }
            if matches!(pattern, Pattern::Wild | Pattern::Var(_)) {
                has_irrefutable = true;
            }
        }
        redundant
    }
    /// Compile a pattern matrix into a case tree.
    ///
    /// This implements the standard pattern matrix compilation algorithm.
    /// Each row contains a vector of patterns (one per column) and a body.
    /// The result is a decision tree that tests columns in an efficient order.
    #[allow(dead_code)]
    pub fn compile_matrix(&mut self, rows: &[PatternRow], num_cols: usize) -> CaseTree {
        if rows.is_empty() {
            return CaseTree::Failure;
        }
        if num_cols == 0 {
            return CaseTree::Leaf { body_idx: 0 };
        }
        let first_row = &rows[0];
        let all_wild = first_row.patterns.iter().all(|p| self.is_irrefutable(p));
        if all_wild && first_row.guard.is_none() {
            return CaseTree::Leaf { body_idx: 0 };
        }
        let col = self.select_column(rows, num_cols);
        let ctors = self.collect_constructors(rows, col);
        if ctors.is_empty() {
            let defaults = self.default_rows(rows, col);
            let new_cols = num_cols.saturating_sub(1);
            return self.compile_matrix(&defaults, new_cols);
        }
        let mut branches = Vec::new();
        for (ctor_name, arity) in &ctors {
            let specialized = self.specialize(rows, col, ctor_name, *arity);
            let new_cols = num_cols - 1 + arity;
            let subtree = self.compile_matrix(&specialized, new_cols);
            branches.push(CaseBranch {
                ctor: ctor_name.clone(),
                num_fields: *arity,
                subtree,
            });
        }
        let defaults = self.default_rows(rows, col);
        let default = if defaults.is_empty() {
            None
        } else {
            let new_cols = num_cols.saturating_sub(1);
            Some(Box::new(self.compile_matrix(&defaults, new_cols)))
        };
        CaseTree::Switch {
            scrutinee: col,
            branches,
            default,
        }
    }
    /// Specialize the pattern matrix for a particular constructor.
    ///
    /// For each row where column `col` matches constructor `ctor`:
    ///   - If the pattern is `Ctor(ctor, args)`, replace column `col` with `args`
    ///   - If the pattern is a wildcard/variable, expand it to `arity` wildcards
    ///
    /// Rows that have a different constructor in column `col` are removed.
    #[allow(dead_code)]
    pub fn specialize(
        &self,
        rows: &[PatternRow],
        col: usize,
        ctor: &str,
        arity: usize,
    ) -> Vec<PatternRow> {
        let mut result = Vec::new();
        for row in rows {
            if col >= row.patterns.len() {
                continue;
            }
            match &row.patterns[col] {
                Pattern::Ctor(name, args) => {
                    if name == ctor {
                        let mut new_patterns = Vec::new();
                        for (i, p) in row.patterns.iter().enumerate() {
                            if i == col {
                                for arg in args {
                                    new_patterns.push(arg.value.clone());
                                }
                            } else {
                                new_patterns.push(p.clone());
                            }
                        }
                        result.push(PatternRow {
                            patterns: new_patterns,
                            body: row.body.clone(),
                            guard: row.guard.clone(),
                        });
                    }
                }
                Pattern::Wild | Pattern::Var(_) => {
                    let mut new_patterns = Vec::new();
                    for (i, p) in row.patterns.iter().enumerate() {
                        if i == col {
                            for _ in 0..arity {
                                new_patterns.push(Pattern::Wild);
                            }
                        } else {
                            new_patterns.push(p.clone());
                        }
                    }
                    result.push(PatternRow {
                        patterns: new_patterns,
                        body: row.body.clone(),
                        guard: row.guard.clone(),
                    });
                }
                Pattern::Lit(_) => {}
                Pattern::Or(left, right) => {
                    let mut left_row = row.clone();
                    left_row.patterns[col] = left.value.clone();
                    let mut right_row = row.clone();
                    right_row.patterns[col] = right.value.clone();
                    let left_spec = self.specialize(&[left_row], col, ctor, arity);
                    let right_spec = self.specialize(&[right_row], col, ctor, arity);
                    result.extend(left_spec);
                    result.extend(right_spec);
                }
            }
        }
        result
    }
    /// Compute the default matrix.
    ///
    /// Rows where column `col` is a wildcard or variable are kept (with the
    /// column removed). Rows with a specific constructor are dropped.
    #[allow(dead_code)]
    pub fn default_rows(&self, rows: &[PatternRow], col: usize) -> Vec<PatternRow> {
        let mut result = Vec::new();
        for row in rows {
            if col >= row.patterns.len() {
                continue;
            }
            match &row.patterns[col] {
                Pattern::Wild | Pattern::Var(_) => {
                    let new_patterns: Vec<Pattern> = row
                        .patterns
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| *i != col)
                        .map(|(_, p)| p.clone())
                        .collect();
                    result.push(PatternRow {
                        patterns: new_patterns,
                        body: row.body.clone(),
                        guard: row.guard.clone(),
                    });
                }
                Pattern::Or(left, right)
                    if (self.is_irrefutable(&left.value) || self.is_irrefutable(&right.value)) =>
                {
                    let new_patterns: Vec<Pattern> = row
                        .patterns
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| *i != col)
                        .map(|(_, p)| p.clone())
                        .collect();
                    result.push(PatternRow {
                        patterns: new_patterns,
                        body: row.body.clone(),
                        guard: row.guard.clone(),
                    });
                }
                _ => {}
            }
        }
        result
    }
    /// Collect constructors appearing in a particular column.
    ///
    /// Returns a deduplicated list of `(constructor_name, arity)` pairs.
    #[allow(dead_code)]
    pub fn collect_constructors(&self, rows: &[PatternRow], col: usize) -> Vec<(String, usize)> {
        let mut seen = Vec::new();
        for row in rows {
            if col >= row.patterns.len() {
                continue;
            }
            self.collect_ctors_from_pattern(&row.patterns[col], &mut seen);
        }
        seen
    }
    /// Check exhaustiveness given known type constructors.
    ///
    /// Returns `Ok(())` if the patterns are exhaustive, or `Err(missing)` with
    /// a list of constructor names that are not covered.
    #[allow(dead_code)]
    pub fn check_exhaustive_with_ctors(
        &self,
        patterns: &[Pattern],
        ctors: &TypeConstructors,
    ) -> Result<(), Vec<String>> {
        for pat in patterns {
            if self.is_irrefutable(pat) {
                return Ok(());
            }
        }
        let mut covered = Vec::new();
        for pat in patterns {
            self.collect_pattern_ctors(pat, &mut covered);
        }
        let mut missing = Vec::new();
        for ctor_info in &ctors.constructors {
            if !covered.contains(&ctor_info.name) {
                missing.push(ctor_info.name.clone());
            }
        }
        if missing.is_empty() {
            Ok(())
        } else {
            Err(missing)
        }
    }
    /// Check exhaustiveness for multi-column pattern matching.
    ///
    /// Each inner `Vec<Pattern>` represents a row of patterns (one per scrutinee).
    /// `ctors` provides constructor info for each column's type.
    ///
    /// Returns `Ok(())` if exhaustive, or `Err` with missing pattern combinations.
    #[allow(dead_code)]
    pub fn check_nested_exhaustive(
        &self,
        patterns: &[Vec<Pattern>],
        ctors: &[TypeConstructors],
    ) -> Result<(), Vec<Vec<String>>> {
        if patterns.is_empty() {
            if ctors.is_empty() {
                return Ok(());
            }
            let missing: Vec<Vec<String>> = ctors[0]
                .constructors
                .iter()
                .map(|c| vec![c.name.clone()])
                .collect();
            return if missing.is_empty() {
                Ok(())
            } else {
                Err(missing)
            };
        }
        if ctors.is_empty() {
            return Ok(());
        }
        let mut all_missing: Vec<Vec<String>> = Vec::new();
        for (col_idx, type_ctors) in ctors.iter().enumerate() {
            let col_patterns: Vec<Pattern> = patterns
                .iter()
                .filter_map(|row| row.get(col_idx).cloned())
                .collect();
            if let Err(missing) = self.check_exhaustive_with_ctors(&col_patterns, type_ctors) {
                for m in &missing {
                    let mut combo = vec!["_".to_string(); ctors.len()];
                    combo[col_idx] = m.clone();
                    all_missing.push(combo);
                }
            }
        }
        if all_missing.is_empty() {
            Ok(())
        } else {
            Err(all_missing)
        }
    }
    /// Simplify a pattern by flattening nested or-patterns.
    ///
    /// Transforms `(a | b) | c` into a flat structure and removes
    /// redundant wildcards in or-patterns.
    #[allow(dead_code)]
    pub fn simplify_pattern(&self, pattern: &Pattern) -> Pattern {
        match pattern {
            Pattern::Or(left, right) => {
                let sl = self.simplify_pattern(&left.value);
                let sr = self.simplify_pattern(&right.value);
                if self.is_irrefutable(&sl) || self.is_irrefutable(&sr) {
                    return Pattern::Wild;
                }
                Pattern::Or(
                    Box::new(crate::Located::new(sl, left.span.clone())),
                    Box::new(crate::Located::new(sr, right.span.clone())),
                )
            }
            Pattern::Ctor(name, args) => {
                let simplified_args: Vec<crate::Located<Pattern>> = args
                    .iter()
                    .map(|a| crate::Located::new(self.simplify_pattern(&a.value), a.span.clone()))
                    .collect();
                Pattern::Ctor(name.clone(), simplified_args)
            }
            Pattern::Wild => Pattern::Wild,
            Pattern::Var(v) => Pattern::Var(v.clone()),
            Pattern::Lit(l) => Pattern::Lit(l.clone()),
        }
    }
    /// Extract literal values from a pattern for range analysis.
    pub fn extract_literal_range(&self, patterns: &[Pattern]) -> Option<(i64, i64)> {
        let mut values = Vec::new();
        for pat in patterns {
            self.collect_literals(pat, &mut values);
        }
        if values.is_empty() {
            return None;
        }
        values.sort();
        Some((values[0], values[values.len() - 1]))
    }
    /// Check if patterns cover all values in a given range.
    pub fn check_range_coverage(&self, patterns: &[Pattern], min: i64, max: i64) -> bool {
        let mut covered = HashSet::new();
        for pat in patterns {
            self.collect_literal_set(pat, &mut covered);
        }
        for pat in patterns {
            if self.is_irrefutable(pat) {
                return true;
            }
        }
        for i in min..=max {
            if !covered.contains(&(i as u64)) {
                return false;
            }
        }
        true
    }
    /// Analyze pattern matrix for dead code.
    pub fn find_dead_patterns(&self, rows: &[PatternRow]) -> Vec<usize> {
        let mut dead = Vec::new();
        for (i, _row) in rows.iter().enumerate() {
            if i > 0 && self.all_irrefutable(&rows[..i]) {
                dead.push(i);
            }
        }
        dead
    }
    /// Extract all bound variable names from a pattern.
    #[allow(dead_code)]
    pub fn extract_bound_names(&self, pattern: &Pattern) -> Vec<String> {
        let mut names = Vec::new();
        self.collect_bound_names(pattern, &mut names);
        names
    }
}
