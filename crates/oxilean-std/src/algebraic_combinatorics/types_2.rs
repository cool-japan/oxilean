//! Extended algebraic combinatorics types:
//! Young diagrams, tableaux, permutations, and related structures.

/// A Young diagram (integer partition).  `rows\[i\]` is the length of row `i`,
/// with `rows\[0\] >= rows\[1\] >= ... >= rows[k-1] > 0`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct YoungDiagram {
    pub rows: Vec<usize>,
}

impl YoungDiagram {
    /// Total number of cells.
    pub fn size(&self) -> usize {
        self.rows.iter().sum()
    }

    /// Number of rows (parts).
    pub fn num_rows(&self) -> usize {
        self.rows.len()
    }

    /// Number of columns (first-part length).
    pub fn num_cols(&self) -> usize {
        self.rows.first().copied().unwrap_or(0)
    }

    /// Verify the partition invariants.
    pub fn is_valid(&self) -> bool {
        self.rows.windows(2).all(|w| w[0] >= w[1]) && self.rows.iter().all(|&r| r > 0)
    }
}

/// A Standard Young Tableau (SYT): entries are a permutation of `1..=n`
/// filling the shape such that rows and columns are strictly increasing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StandardTableau {
    pub shape: YoungDiagram,
    /// `entries[row][col]` — 1-indexed values.
    pub entries: Vec<Vec<usize>>,
}

/// A Semistandard Young Tableau (SSYT): entries are weakly increasing
/// along rows and strictly increasing down columns.  Values are positive
/// integers (no upper bound by default).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemistandardTableau {
    pub shape: YoungDiagram,
    /// `entries[row][col]` — positive integer values.
    pub entries: Vec<Vec<usize>>,
}

/// A permutation of `{0, 1, ..., n-1}` stored as a vector where
/// `sigma\[i\]` is the image of `i`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Permutation {
    pub sigma: Vec<usize>,
}

impl Permutation {
    /// Identity permutation on `n` elements.
    pub fn identity(n: usize) -> Self {
        Self {
            sigma: (0..n).collect(),
        }
    }

    /// Length (number of inversions).
    pub fn length(&self) -> usize {
        let n = self.sigma.len();
        let mut count = 0;
        for i in 0..n {
            for j in (i + 1)..n {
                if self.sigma[i] > self.sigma[j] {
                    count += 1;
                }
            }
        }
        count
    }

    /// Permutation size.
    pub fn n(&self) -> usize {
        self.sigma.len()
    }
}

/// The descent set of a permutation: positions `i` such that `σ(i) > σ(i+1)`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DescentSet {
    pub positions: Vec<usize>,
}

/// Beta-set (diagram) representation of an integer partition.
/// The beta-set for a partition `λ` with `k` parts is
/// `{ λ_i + k - i : 1 ≤ i ≤ k }`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BetaSet {
    pub values: Vec<usize>,
}

/// A Schur polynomial in `variables` variables associated to a Young diagram.
/// (Symbolic structure; evaluation uses SSYT.)
#[derive(Debug, Clone)]
pub struct SchurPolynomial {
    pub diagram: YoungDiagram,
    pub variables: usize,
}

/// An element of a Coxeter group specified by a reduced word in the generators
/// `s_0, s_1, ..., s_{n-1}` (0-indexed).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoxeterElement {
    pub reduced_word: Vec<usize>,
}

/// The Robinson-Schensted correspondence output: a pair of standard tableaux
/// (insertion tableau P, recording tableau Q) of the same shape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RobinsonSchensted {
    pub insertion: StandardTableau,
    pub recording: StandardTableau,
}
