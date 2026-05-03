//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use crate::match_basic::{MetaMatchArm, MetaPattern};
use oxilean_kernel::{Expr, Name};

/// Analysis of constructor coverage in a match expression.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CoverageAnalysis {
    /// Constructors covered by at least one arm.
    pub covered: Vec<String>,
    /// Constructors not covered by any arm.
    pub uncovered: Vec<String>,
    /// Constructors covered by multiple arms (overlap).
    pub overlapping: Vec<String>,
}
#[allow(dead_code)]
impl CoverageAnalysis {
    /// Check if coverage is complete for the given set of constructors.
    pub fn is_complete(&self, all_ctors: &[&str]) -> bool {
        all_ctors
            .iter()
            .all(|c| self.covered.contains(&c.to_string()))
    }
    /// Check if there are overlapping cases.
    pub fn has_overlap(&self) -> bool {
        !self.overlapping.is_empty()
    }
    /// Summary string.
    pub fn summary(&self) -> String {
        format!(
            "covered=[{}], uncovered=[{}], overlap=[{}]",
            self.covered.join(","),
            self.uncovered.join(","),
            self.overlapping.join(","),
        )
    }
}
/// A DAG of decision tree nodes.
#[allow(dead_code)]
pub struct DecisionDag {
    /// Arena of all nodes.
    pub nodes: Vec<DagNode>,
    /// Root node index.
    pub root: usize,
}
impl DecisionDag {
    /// Create an empty DAG.
    #[allow(dead_code)]
    pub fn new() -> Self {
        DecisionDag {
            nodes: Vec::new(),
            root: 0,
        }
    }
    /// Convert a `DecisionTree` to a DAG by flattening.
    #[allow(dead_code)]
    pub fn from_tree(tree: &DecisionTree) -> Self {
        let mut dag = DecisionDag::new();
        let root = dag.insert_tree(tree);
        dag.root = root;
        dag
    }
    fn insert_tree(&mut self, tree: &DecisionTree) -> usize {
        match tree {
            DecisionTree::Leaf { arm_idx, .. } => {
                let idx = self.nodes.len();
                self.nodes.push(DagNode::Leaf { arm_idx: *arm_idx });
                idx
            }
            DecisionTree::Failure => {
                let idx = self.nodes.len();
                self.nodes.push(DagNode::Failure);
                idx
            }
            DecisionTree::Switch {
                column,
                branches,
                default,
            } => {
                let dag_branches: Vec<DagBranch> = branches
                    .iter()
                    .map(|b| DagBranch {
                        ctor_name: b.ctor_name.clone(),
                        num_fields: b.num_fields,
                        child_idx: self.insert_tree(&b.subtree),
                    })
                    .collect();
                let default_idx = default.as_ref().map(|d| self.insert_tree(d));
                let idx = self.nodes.len();
                self.nodes.push(DagNode::Switch {
                    column: *column,
                    branches: dag_branches,
                    default: default_idx,
                });
                idx
            }
        }
    }
    /// Return the total number of nodes.
    #[allow(dead_code)]
    pub fn num_nodes(&self) -> usize {
        self.nodes.len()
    }
    /// Return the number of leaf nodes.
    #[allow(dead_code)]
    pub fn num_leaves(&self) -> usize {
        self.nodes
            .iter()
            .filter(|n| matches!(n, DagNode::Leaf { .. }))
            .count()
    }
    /// Return the number of failure nodes.
    #[allow(dead_code)]
    pub fn num_failures(&self) -> usize {
        self.nodes
            .iter()
            .filter(|n| matches!(n, DagNode::Failure))
            .count()
    }
}
/// Scoring heuristics for column selection.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColumnHeuristic {
    /// Pick column with the most distinct constructors.
    MostDistinct,
    /// Pick the leftmost non-wildcard column.
    Leftmost,
    /// Pick column with fewest wildcard rows.
    FewestWildcards,
    /// Combined: most distinct constructors, break ties by fewest wildcards.
    Combined,
}
impl ColumnHeuristic {
    /// Return a human-readable label.
    #[allow(dead_code)]
    pub fn label(&self) -> &'static str {
        match self {
            ColumnHeuristic::MostDistinct => "most-distinct",
            ColumnHeuristic::Leftmost => "leftmost",
            ColumnHeuristic::FewestWildcards => "fewest-wildcards",
            ColumnHeuristic::Combined => "combined",
        }
    }
}
/// A guard condition associated with a match arm.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct GuardedArm {
    /// The arm index.
    pub arm_idx: usize,
    /// The guard expression (None = always match).
    pub guard: Option<oxilean_kernel::Expr>,
    /// The RHS expression.
    pub rhs: oxilean_kernel::Expr,
}
/// A compiled decision tree for a match expression.
#[derive(Clone, Debug)]
pub enum DecisionTree {
    /// Leaf: execute arm at given index.
    Leaf {
        /// The arm index to execute.
        arm_idx: usize,
        /// Bindings from pattern variables to expressions.
        bindings: Vec<(Name, Expr)>,
    },
    /// Switch on a constructor at the given column.
    Switch {
        /// Which discriminant to test.
        column: usize,
        /// Branches by constructor name.
        branches: Vec<DecisionBranch>,
        /// Default branch (for wildcards).
        default: Option<Box<DecisionTree>>,
    },
    /// Failure: no match (should not happen if exhaustive).
    Failure,
}
/// A pattern matrix row, tracking original arm index.
#[derive(Clone, Debug)]
pub(super) struct PatRow {
    pub(super) patterns: Vec<MetaPattern>,
    pub(super) arm_idx: usize,
    pub(super) rhs: Expr,
}
/// A branch in a decision tree switch node.
#[derive(Clone, Debug)]
pub struct DecisionBranch {
    /// Constructor name.
    pub ctor_name: Name,
    /// Number of fields.
    pub num_fields: u32,
    /// Field variable names.
    pub field_names: Vec<Name>,
    /// Sub-decision tree.
    pub subtree: DecisionTree,
}
/// Information about pattern exhaustiveness for one column.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ExhaustivenessInfo {
    /// Column index.
    pub column: usize,
    /// Known constructors seen in patterns.
    pub seen_ctors: Vec<Name>,
    /// Whether there is at least one wildcard row.
    pub has_wildcard: bool,
    /// Whether the column is exhaustive for known types.
    pub is_exhaustive: bool,
}
impl ExhaustivenessInfo {
    /// Compute exhaustiveness info for a given column from match arms.
    #[allow(dead_code)]
    pub fn from_arms(arms: &[MetaMatchArm], col: usize) -> Self {
        let mut seen: Vec<Name> = Vec::new();
        let mut has_wildcard = false;
        for arm in arms {
            match arm.patterns.get(col) {
                Some(crate::match_basic::MetaPattern::Constructor(name, _))
                    if !seen.contains(name) =>
                {
                    seen.push(name.clone());
                }
                Some(crate::match_basic::MetaPattern::Wildcard)
                | Some(crate::match_basic::MetaPattern::Var(_)) => {
                    has_wildcard = true;
                }
                _ => {}
            }
        }
        let is_exhaustive = has_wildcard
            || (seen.iter().any(|n| n.to_string().contains("zero"))
                && seen.iter().any(|n| n.to_string().contains("succ")))
            || (seen.iter().any(|n| n.to_string().contains("true"))
                && seen.iter().any(|n| n.to_string().contains("false")));
        ExhaustivenessInfo {
            column: col,
            seen_ctors: seen,
            has_wildcard,
            is_exhaustive,
        }
    }
    /// Propagate exhaustiveness from child subtrees.
    #[allow(dead_code)]
    pub fn propagate(branch_exhaustive: &[bool], has_default: bool) -> bool {
        has_default || branch_exhaustive.iter().all(|&e| e)
    }
}
/// Column scoring information for heuristic selection.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ColumnScore {
    /// Column index.
    pub column: usize,
    /// Number of distinct constructors in this column.
    pub distinct_ctors: usize,
    /// Number of wildcard/variable rows in this column.
    pub wildcard_rows: usize,
    /// Overall score (higher = better).
    pub score: i64,
}
/// A compiled decision tree with associated metadata.
#[allow(dead_code)]
pub struct CompiledMatch {
    /// The root decision tree.
    pub tree: DecisionTree,
    /// Number of arms in the original match.
    pub num_arms: usize,
    /// Number of discriminant columns.
    pub num_columns: usize,
    /// Whether the match is exhaustive.
    pub is_exhaustive: bool,
}
#[allow(dead_code)]
impl CompiledMatch {
    /// Create a new compiled match.
    pub fn new(tree: DecisionTree, num_arms: usize, num_columns: usize) -> Self {
        let is_exhaustive = is_exhaustive_tree(&tree);
        Self {
            tree,
            num_arms,
            num_columns,
            is_exhaustive,
        }
    }
    /// Compile a match from arms.
    pub fn compile(arms: &[MetaMatchArm], num_columns: usize) -> Self {
        let tree = build_decision_tree(arms, num_columns);
        Self::new(tree, arms.len(), num_columns)
    }
    /// Get the tree depth.
    pub fn depth(&self) -> usize {
        tree_depth(&self.tree)
    }
    /// Count the leaves.
    pub fn num_leaves(&self) -> usize {
        count_leaves(&self.tree)
    }
    /// Summarize for debugging.
    pub fn summary(&self) -> String {
        format!(
            "CompiledMatch(arms={}, cols={}, exhaustive={}, depth={}, leaves={})",
            self.num_arms,
            self.num_columns,
            self.is_exhaustive,
            self.depth(),
            self.num_leaves(),
        )
    }
}
/// Statistics about a compiled decision tree.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct TreeStats {
    /// Total number of nodes.
    pub total_nodes: usize,
    /// Number of leaf nodes.
    pub leaf_nodes: usize,
    /// Number of failure nodes.
    pub failure_nodes: usize,
    /// Number of switch nodes.
    pub switch_nodes: usize,
    /// Maximum depth.
    pub max_depth: usize,
    /// Number of reachable arms.
    pub reachable_arm_count: usize,
}
#[allow(dead_code)]
impl TreeStats {
    /// Collect stats from a tree.
    pub fn from_tree(tree: &DecisionTree, num_arms: usize) -> Self {
        let mut stats = Self::default();
        collect_tree_stats(tree, &mut stats, 0);
        stats.reachable_arm_count = reachable_arms(tree).len().min(num_arms);
        stats
    }
}
/// Exhaustiveness report with reasons for incompleteness.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ExhaustivenessReport {
    /// Whether the match is exhaustive.
    pub is_exhaustive: bool,
    /// Missing constructor patterns, if any.
    pub missing_patterns: Vec<String>,
}
#[allow(dead_code)]
impl ExhaustivenessReport {
    /// Create a report for an exhaustive match.
    pub fn exhaustive() -> Self {
        Self {
            is_exhaustive: true,
            missing_patterns: Vec::new(),
        }
    }
    /// Create a report for an incomplete match.
    pub fn incomplete(missing: Vec<String>) -> Self {
        Self {
            is_exhaustive: false,
            missing_patterns: missing,
        }
    }
    /// Format the report.
    pub fn format(&self) -> String {
        if self.is_exhaustive {
            "exhaustive".to_string()
        } else {
            format!(
                "non-exhaustive (missing: [{}])",
                self.missing_patterns.join(", ")
            )
        }
    }
}
/// An equation generated from a match arm, used for
/// definitional unfolding of the match expression.
#[derive(Clone, Debug)]
pub struct MatchEquation {
    /// Left-hand side patterns.
    pub lhs_patterns: Vec<MetaPattern>,
    /// Right-hand side expression.
    pub rhs: Expr,
    /// Arm index.
    pub arm_idx: usize,
}
/// A node in a DAG-shared decision tree.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum DagNode {
    /// Leaf: execute arm at this index.
    Leaf {
        /// The arm index to execute.
        arm_idx: usize,
    },
    /// Failure node.
    Failure,
    /// Switch: test a constructor, branch to child node indices.
    Switch {
        /// The column to test.
        column: usize,
        /// The branches for each constructor.
        branches: Vec<DagBranch>,
        /// The default branch if no constructor matches.
        default: Option<usize>,
    },
}
/// Column selection strategy for the decision tree algorithm.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColumnStrategy {
    /// Pick the leftmost non-wildcard column.
    Leftmost,
    /// Pick the column with the most distinct constructors.
    MostDiscriminating,
    /// Pick the column with the fewest rows.
    SmallestColumn,
}
#[allow(dead_code)]
impl ColumnStrategy {
    /// Get the default strategy.
    pub fn default_strategy() -> Self {
        Self::MostDiscriminating
    }
    /// Get a description of the strategy.
    pub fn description(&self) -> &str {
        match self {
            ColumnStrategy::Leftmost => "leftmost non-wildcard",
            ColumnStrategy::MostDiscriminating => "most discriminating",
            ColumnStrategy::SmallestColumn => "smallest column",
        }
    }
}
/// A branch in a DAG switch node.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct DagBranch {
    /// Constructor name.
    pub ctor_name: Name,
    /// Number of fields.
    pub num_fields: u32,
    /// Index of the child node in the arena.
    pub child_idx: usize,
}
/// A jump table entry: maps a constructor to an arm index.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JumpTableEntry {
    /// The constructor name.
    pub ctor_name: Name,
    /// Index of the constructor in the type's constructor list.
    pub ctor_idx: usize,
    /// The arm index to jump to.
    pub arm_idx: usize,
}
/// Statistics about a compiled decision tree.
#[derive(Debug, Clone)]
pub struct DecisionTreeStats {
    /// Total number of nodes in the tree.
    pub num_nodes: usize,
    /// Maximum depth of the tree.
    pub max_depth: usize,
    /// Number of leaf nodes (successful match arms).
    pub num_leaves: usize,
    /// Number of failure nodes.
    pub num_failures: usize,
    /// Number of distinct arm indices referenced.
    pub num_arms_referenced: usize,
}
/// A compiled jump table for a single switch node.
#[allow(dead_code)]
pub struct JumpTable {
    /// The discriminant column.
    pub column: usize,
    /// Entries sorted by constructor index.
    pub entries: Vec<JumpTableEntry>,
    /// Default arm (for wildcards), if any.
    pub default_arm: Option<usize>,
}
impl JumpTable {
    /// Look up the arm for a given constructor name.
    #[allow(dead_code)]
    pub fn lookup(&self, ctor_name: &Name) -> Option<usize> {
        for entry in &self.entries {
            if &entry.ctor_name == ctor_name {
                return Some(entry.arm_idx);
            }
        }
        self.default_arm
    }
    /// Return the number of entries.
    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.entries.len()
    }
    /// Check if all entries jump to the same arm.
    #[allow(dead_code)]
    pub fn is_uniform(&self) -> bool {
        if self.entries.is_empty() {
            return true;
        }
        let first = self.entries[0].arm_idx;
        self.entries.iter().all(|e| e.arm_idx == first)
            && self.default_arm.map_or(true, |d| d == first)
    }
}
/// Extended size statistics for a decision tree.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ExtendedTreeStats {
    /// Total switch nodes.
    pub num_switches: usize,
    /// Total leaf nodes.
    pub num_leaves: usize,
    /// Total failure nodes.
    pub num_failures: usize,
    /// Total branch edges.
    pub total_branches: usize,
    /// Maximum branching factor.
    pub max_branching_factor: usize,
    pub(super) sum_branching: usize,
    /// Average branching factor.
    pub avg_branching_factor: f64,
    /// Tree depth.
    pub tree_depth: usize,
}
impl ExtendedTreeStats {
    pub(super) fn finalize(&mut self) {
        if self.num_switches > 0 {
            self.avg_branching_factor = self.sum_branching as f64 / self.num_switches as f64;
        }
    }
}
