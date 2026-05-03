//! Types for Abstract Rewriting Systems (ARS) and term rewriting.

use std::collections::HashMap;

/// A conditional rewrite rule over a term type `T`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RewriteRule<T: Clone + Eq> {
    /// Left-hand side pattern.
    pub lhs: T,
    /// Right-hand side result.
    pub rhs: T,
    /// Human-readable name for the rule.
    pub name: String,
    /// Conditions that must hold (as terms that must reduce to True/normal form).
    pub conditions: Vec<T>,
}

impl<T: Clone + Eq> RewriteRule<T> {
    /// Creates a new unconditional rewrite rule.
    pub fn new(name: impl Into<String>, lhs: T, rhs: T) -> Self {
        RewriteRule {
            lhs,
            rhs,
            name: name.into(),
            conditions: Vec::new(),
        }
    }

    /// Creates a new conditional rewrite rule.
    pub fn conditional(name: impl Into<String>, lhs: T, rhs: T, conditions: Vec<T>) -> Self {
        RewriteRule {
            lhs,
            rhs,
            name: name.into(),
            conditions,
        }
    }

    /// Returns whether this rule has conditions.
    pub fn is_conditional(&self) -> bool {
        !self.conditions.is_empty()
    }
}

/// The strategy used when applying rewrite rules.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RewriteStrategy {
    /// Reduce innermost redexes first (call-by-value).
    Innermost,
    /// Reduce outermost redexes first (call-by-name / lazy).
    Outermost,
    /// Leftmost-innermost: standard reduction order for functional languages.
    LeftmostInnermost,
    /// Leftmost-outermost: used in lambda calculus normal order.
    LeftmostOutermost,
    /// Reduce all redexes simultaneously.
    Parallel,
}

/// A term rewriting system with a collection of rules and a reduction strategy.
#[derive(Debug, Clone)]
pub struct RewriteSystem<T: Clone + Eq> {
    /// The set of rewrite rules.
    pub rules: Vec<RewriteRule<T>>,
    /// The reduction strategy to use.
    pub strategy: RewriteStrategy,
}

impl<T: Clone + Eq> RewriteSystem<T> {
    /// Creates a new rewrite system with the given strategy.
    pub fn new(strategy: RewriteStrategy) -> Self {
        RewriteSystem {
            rules: Vec::new(),
            strategy,
        }
    }

    /// Adds a rule to the system.
    pub fn add_rule(&mut self, rule: RewriteRule<T>) {
        self.rules.push(rule);
    }
}

/// The result of a rewriting computation.
#[derive(Debug, Clone)]
pub struct RewriteResult<T> {
    /// The final (possibly normal) term.
    pub term: T,
    /// Trace of (rule_name, resulting_term) pairs.
    pub steps: Vec<(String, T)>,
    /// Whether the term reached a normal form (no more rules apply).
    pub converged: bool,
}

impl<T> RewriteResult<T> {
    /// Returns the number of rewrite steps taken.
    pub fn num_steps(&self) -> usize {
        self.steps.len()
    }
}

/// A concrete tree-structured term.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TermTree {
    /// A leaf node (constant or variable). Variables are uppercase by convention.
    Leaf(String),
    /// A node with a symbol and children (function application).
    Node {
        /// The function symbol.
        symbol: String,
        /// The children (arguments).
        children: Vec<TermTree>,
    },
}

impl TermTree {
    /// Convenience constructor for a leaf node.
    pub fn leaf(s: impl Into<String>) -> Self {
        TermTree::Leaf(s.into())
    }

    /// Convenience constructor for a node.
    pub fn node(symbol: impl Into<String>, children: Vec<TermTree>) -> Self {
        TermTree::Node {
            symbol: symbol.into(),
            children,
        }
    }

    /// Returns true if this term is a variable (leaf with uppercase first char).
    pub fn is_variable(&self) -> bool {
        match self {
            TermTree::Leaf(s) => s.starts_with(|c: char| c.is_uppercase()),
            TermTree::Node { .. } => false,
        }
    }

    /// Returns true if this is a leaf node.
    pub fn is_leaf(&self) -> bool {
        matches!(self, TermTree::Leaf(_))
    }

    /// Returns the root symbol, if any.
    pub fn root_symbol(&self) -> Option<&str> {
        match self {
            TermTree::Leaf(s) => Some(s),
            TermTree::Node { symbol, .. } => Some(symbol),
        }
    }
}

impl std::fmt::Display for TermTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TermTree::Leaf(s) => write!(f, "{s}"),
            TermTree::Node { symbol, children } => {
                if children.is_empty() {
                    write!(f, "{symbol}")
                } else {
                    let args: Vec<String> = children.iter().map(|c| c.to_string()).collect();
                    write!(f, "{symbol}({})", args.join(", "))
                }
            }
        }
    }
}

/// A critical pair representing a potential confluence failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CriticalPair {
    /// Name of the first rule involved.
    pub rule1: String,
    /// Name of the second rule involved.
    pub rule2: String,
    /// The term where the overlap occurs.
    pub overlap: TermTree,
    /// Result after applying rule1.
    pub result1: TermTree,
    /// Result after applying rule2.
    pub result2: TermTree,
}

impl CriticalPair {
    /// Returns true if this critical pair is trivial (results are equal).
    pub fn is_trivial(&self) -> bool {
        self.result1 == self.result2
    }
}

/// The result of a confluence analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfluenceResult {
    /// The system is confluent (all critical pairs are joinable).
    Confluent,
    /// The system is not confluent; the given critical pair witnesses this.
    NotConfluent(CriticalPair),
    /// Confluence could not be determined (e.g., non-terminating system).
    Unknown,
}

/// The normalization order used when comparing term weights.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NormalizationOrder {
    /// Compare subterms left to right.
    LeftToRight,
    /// Prefer terms with greater weight first.
    GreaterFirst,
    /// Weighted path order using symbol weights.
    WeightedPath {
        /// Weight map from symbol names to numeric weights.
        weights: HashMap<String, u32>,
    },
}
