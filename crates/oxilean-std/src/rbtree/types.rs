//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// AVL tree rotation data.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum AvlRotation {
    LeftLeft,
    RightRight,
    LeftRight,
    RightLeft,
}
#[allow(dead_code)]
impl AvlRotation {
    /// Number of single rotations needed.
    pub fn num_rotations(&self) -> usize {
        match self {
            Self::LeftLeft | Self::RightRight => 1,
            Self::LeftRight | Self::RightLeft => 2,
        }
    }
    /// Description.
    pub fn description(&self) -> &str {
        match self {
            Self::LeftLeft => "single right rotation",
            Self::RightRight => "single left rotation",
            Self::LeftRight => "left-right double rotation",
            Self::RightLeft => "right-left double rotation",
        }
    }
}
/// Treap (randomized BST) node.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TreapNode {
    pub key: i64,
    pub priority: u64,
    pub size: usize,
}
#[allow(dead_code)]
impl TreapNode {
    /// Create a treap node with a random priority.
    pub fn new(key: i64, priority: u64) -> Self {
        Self {
            key,
            priority,
            size: 1,
        }
    }
    /// Expected height of a treap is O(log n).
    pub fn expected_height_description(n: usize) -> String {
        let h = (n as f64).log2() * 2.0;
        format!("Expected height ~{:.1} for n={}", h, n)
    }
}
/// Splay tree amortized analysis.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SplayAnalysis {
    pub sequence_length: usize,
    pub access_sequence: Vec<i64>,
    pub amortized_cost: f64,
}
#[allow(dead_code)]
impl SplayAnalysis {
    /// Splay tree analysis for a sequence.
    pub fn new(seq: Vec<i64>) -> Self {
        let n = seq.len();
        let cost = if n == 0 {
            0.0
        } else {
            (n as f64) * (n as f64).log2()
        };
        Self {
            sequence_length: n,
            access_sequence: seq,
            amortized_cost: cost,
        }
    }
    /// Total amortized cost.
    pub fn total_cost(&self) -> f64 {
        self.amortized_cost
    }
    /// Average cost per operation.
    pub fn avg_cost(&self) -> f64 {
        if self.sequence_length == 0 {
            0.0
        } else {
            self.amortized_cost / self.sequence_length as f64
        }
    }
}
/// Order statistics tree data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OrderStatisticsTree {
    pub size: usize,
    pub keys: Vec<i64>,
}
#[allow(dead_code)]
impl OrderStatisticsTree {
    /// Create an order statistics tree.
    pub fn new(mut keys: Vec<i64>) -> Self {
        keys.sort_unstable();
        let size = keys.len();
        Self { size, keys }
    }
    /// k-th smallest element (1-indexed).
    pub fn kth_smallest(&self, k: usize) -> Option<i64> {
        if k == 0 || k > self.size {
            None
        } else {
            Some(self.keys[k - 1])
        }
    }
    /// Rank of a key (number of elements <= key).
    pub fn rank(&self, key: i64) -> usize {
        self.keys.iter().filter(|&&k| k <= key).count()
    }
}
/// B-tree node data (for database/storage contexts).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BTreeNodeData {
    pub order: usize,
    pub keys: Vec<i64>,
    pub is_leaf: bool,
    pub num_children: usize,
}
#[allow(dead_code)]
impl BTreeNodeData {
    /// Create a B-tree leaf node.
    pub fn leaf(order: usize, keys: Vec<i64>) -> Self {
        Self {
            order,
            keys: keys.clone(),
            is_leaf: true,
            num_children: 0,
        }
    }
    /// Create an internal B-tree node.
    pub fn internal(order: usize, keys: Vec<i64>, num_children: usize) -> Self {
        Self {
            order,
            keys,
            is_leaf: false,
            num_children,
        }
    }
    /// Is this node overfull (needs splitting)?
    pub fn needs_split(&self) -> bool {
        self.keys.len() >= 2 * self.order - 1
    }
    /// Is this node underfull (needs merging)?
    pub fn needs_merge(&self) -> bool {
        self.keys.len() < self.order - 1
    }
}
