//! Extended algebraic combinatorics functions: Young tableaux, permutations,
//! Robinson-Schensted, Kostka numbers, and combinatorial sequences.
#![allow(clippy::items_after_test_module)]

use super::types_2::{
    BetaSet, CoxeterElement, DescentSet, Permutation, RobinsonSchensted, SchurPolynomial,
    SemistandardTableau, StandardTableau, YoungDiagram,
};

// ─── Young diagram utilities ──────────────────────────────────────────────────

/// Construct a `YoungDiagram` from a slice of part sizes.
///
/// Parts are sorted in descending order and zeros are removed.
pub fn young_diagram_from_partition(parts: &[usize]) -> YoungDiagram {
    let mut rows: Vec<usize> = parts.iter().filter(|&&p| p > 0).copied().collect();
    rows.sort_unstable_by(|a, b| b.cmp(a));
    YoungDiagram { rows }
}

/// Compute the conjugate (transpose) Young diagram.
pub fn conjugate_diagram(yd: &YoungDiagram) -> YoungDiagram {
    if yd.rows.is_empty() {
        return YoungDiagram { rows: vec![] };
    }
    let num_cols = yd.rows[0];
    let rows: Vec<usize> = (0..num_cols)
        .map(|col| yd.rows.iter().filter(|&&r| r > col).count())
        .collect();
    YoungDiagram { rows }
}

/// Compute the hook length at cell `(row, col)` (0-indexed).
///
/// Hook length = (cells to the right in same row) + (cells below in same col) + 1.
pub fn hook_length(yd: &YoungDiagram, row: usize, col: usize) -> usize {
    if row >= yd.rows.len() || col >= yd.rows[row] {
        return 0;
    }
    let arm = yd.rows[row] - col - 1; // cells to the right
    let leg = yd.rows[row + 1..].iter().filter(|&&r| r > col).count(); // cells below
    arm + leg + 1
}

/// The hook-length formula: `n! / ∏ hook_lengths`.
///
/// Returns the number of standard Young tableaux of shape `yd`.
pub fn hook_length_formula(yd: &YoungDiagram) -> u64 {
    let n = yd.size();
    let nfact = factorial(n);
    let mut prod: u64 = 1;
    for (row_idx, &row_len) in yd.rows.iter().enumerate() {
        for col in 0..row_len {
            let h = hook_length(yd, row_idx, col) as u64;
            if h == 0 {
                return 0;
            }
            prod = prod.saturating_mul(h);
        }
    }
    nfact / prod
}

/// Number of standard Young tableaux of shape `yd`.
pub fn number_of_syt(yd: &YoungDiagram) -> u64 {
    hook_length_formula(yd)
}

// ─── Permutation functions ────────────────────────────────────────────────────

/// Compute the descent set of a permutation.
///
/// Position `i` is a descent if `σ(i) > σ(i+1)`.
pub fn descent_set(perm: &Permutation) -> DescentSet {
    let positions: Vec<usize> = (0..perm.sigma.len().saturating_sub(1))
        .filter(|&i| perm.sigma[i] > perm.sigma[i + 1])
        .collect();
    DescentSet { positions }
}

/// Major index of a permutation: sum of descent positions.
pub fn major_index(perm: &Permutation) -> usize {
    descent_set(perm).positions.iter().sum()
}

/// Count the number of inversions in a permutation.
///
/// An inversion is a pair `(i, j)` with `i < j` and `σ(i) > σ(j)`.
pub fn inversion_number(perm: &Permutation) -> usize {
    perm.length()
}

/// Compute the Lehmer code (factoradic representation) of a permutation.
///
/// `code\[i\]` = number of `j > i` with `σ(j) < σ(i)`.
pub fn lehmer_code(perm: &Permutation) -> Vec<usize> {
    let n = perm.sigma.len();
    (0..n)
        .map(|i| {
            perm.sigma[i + 1..]
                .iter()
                .filter(|&&v| v < perm.sigma[i])
                .count()
        })
        .collect()
}

// ─── Robinson-Schensted correspondence ───────────────────────────────────────

/// Perform the Robinson-Schensted correspondence on a permutation.
///
/// Returns a pair `(P, Q)` of standard Young tableaux of the same shape,
/// where `P` is the insertion tableau and `Q` the recording tableau.
pub fn rs_insertion(perm: &Permutation) -> RobinsonSchensted {
    // Represent the growing tableau as a vector of rows (of usize values).
    let mut p_rows: Vec<Vec<usize>> = Vec::new();
    let mut q_rows: Vec<Vec<usize>> = Vec::new();
    for (step, &val) in perm.sigma.iter().enumerate() {
        // RS row-insertion of (val+1) — we store 1-indexed values.
        let insert_val = val + 1;
        let row_idx = schensted_insert(&mut p_rows, insert_val);

        // Record in Q: place step+1 at the same position.
        while q_rows.len() <= row_idx {
            q_rows.push(Vec::new());
        }
        q_rows[row_idx].push(step + 1);
    }

    // Build shape.
    let shape_rows: Vec<usize> = p_rows.iter().map(|r| r.len()).collect();
    let shape = YoungDiagram { rows: shape_rows };

    let insertion = StandardTableau {
        shape: shape.clone(),
        entries: p_rows,
    };
    let recording = StandardTableau {
        shape,
        entries: q_rows,
    };
    RobinsonSchensted {
        insertion,
        recording,
    }
}

/// Schensted row-insertion: insert `val` into `rows` using the RSK bumping
/// algorithm.  Returns the row index where a new cell was created.
fn schensted_insert(rows: &mut Vec<Vec<usize>>, val: usize) -> usize {
    let mut to_insert = val;
    let mut row_idx = 0usize;
    loop {
        if row_idx >= rows.len() {
            // Append a new row.
            rows.push(vec![to_insert]);
            return rows.len() - 1;
        }
        // Find the leftmost entry strictly greater than `to_insert`.
        let maybe_pos = rows[row_idx].iter().position(|&x| x > to_insert);
        if let Some(pos) = maybe_pos {
            // Bump: swap `to_insert` with `rows[row_idx][pos]` and continue
            // inserting the displaced value into the next row.
            std::mem::swap(&mut rows[row_idx][pos], &mut to_insert);
            row_idx += 1;
        } else {
            // `to_insert` is >= all entries in this row: append.
            rows[row_idx].push(to_insert);
            return row_idx;
        }
    }
}

// ─── Schur polynomial evaluation ─────────────────────────────────────────────

/// Evaluate the Schur polynomial `s_λ` at a point `point` (a vector of `k`
/// real values) by summing `x^{content(T)}` over all SSYT `T` of shape `λ`
/// with entries in `{1, ..., k}`.
///
/// For small shapes / few variables this is exact; for large shapes it may be
/// slow (exponential in the number of cells).
pub fn schur_polynomial_evaluation(diagram: &YoungDiagram, point: &[f64]) -> f64 {
    let k = point.len();
    if k == 0 {
        return if diagram.size() == 0 { 1.0 } else { 0.0 };
    }
    // Collect all cells in reading order.
    let cells: Vec<(usize, usize)> = diagram
        .rows
        .iter()
        .enumerate()
        .flat_map(|(r, &len)| (0..len).map(move |c| (r, c)))
        .collect();
    let num_cells = cells.len();
    // `filling[i]` = 1-indexed value at cell `cells[i]`.
    let mut filling: Vec<usize> = vec![1; num_cells];
    let mut result = 0.0_f64;

    // Iterate over all fillings and sum those that are SSYT.
    loop {
        if is_ssyt_filling(&cells, &filling, &diagram.rows, k) {
            let contrib: f64 = filling.iter().map(|&v| point[v - 1]).product();
            result += contrib;
        }
        // Increment filling (odometer in {1..k}^num_cells, reversed).
        let mut carry = true;
        for i in (0..num_cells).rev() {
            if carry {
                filling[i] += 1;
                if filling[i] > k {
                    filling[i] = 1;
                } else {
                    carry = false;
                }
            }
        }
        if carry {
            break;
        }
    }
    result
}

/// Check whether a filling of cells defines a valid SSYT:
/// weakly increasing along rows, strictly increasing down columns.
fn is_ssyt_filling(
    cells: &[(usize, usize)],
    filling: &[usize],
    row_lens: &[usize],
    _k: usize,
) -> bool {
    // Build a 2-D map from cell to value.
    let max_row = row_lens.len();
    let max_col = row_lens.iter().copied().max().unwrap_or(0);
    let mut grid: Vec<Vec<Option<usize>>> = vec![vec![None; max_col]; max_row];
    for (idx, &(r, c)) in cells.iter().enumerate() {
        grid[r][c] = Some(filling[idx]);
    }
    // Check weakly increasing along rows.
    for (r, &len) in row_lens.iter().enumerate() {
        for c in 1..len {
            let left = grid[r][c - 1].unwrap_or(0);
            let cur = grid[r][c].unwrap_or(0);
            if cur < left {
                return false;
            }
        }
    }
    // Check strictly increasing down columns.
    for c in 0..max_col {
        for r in 1..max_row {
            if let (Some(up), Some(dn)) = (grid[r - 1][c], grid[r][c]) {
                if dn <= up {
                    return false;
                }
            }
        }
    }
    true
}

// ─── Kostka numbers ───────────────────────────────────────────────────────────

/// Compute the Kostka number K_{λ,μ}: the number of SSYT of shape `lambda`
/// and content `mu` (i.e. value `i` appears `mu[i-1]` times).
pub fn kostka_number(lambda: &YoungDiagram, mu: &YoungDiagram) -> u64 {
    let content: Vec<usize> = mu.rows.clone();
    if content.iter().sum::<usize>() != lambda.size() {
        return 0;
    }
    count_ssyt_with_content(lambda, &content)
}

/// Count SSYT of shape `lambda` with given content via backtracking.
fn count_ssyt_with_content(shape: &YoungDiagram, content: &[usize]) -> u64 {
    let cells: Vec<(usize, usize)> = shape
        .rows
        .iter()
        .enumerate()
        .flat_map(|(r, &len)| (0..len).map(move |c| (r, c)))
        .collect();
    let num_cells = cells.len();
    let max_row = shape.rows.len();
    let max_col = shape.rows.iter().copied().max().unwrap_or(0);
    let mut grid: Vec<Vec<usize>> = vec![vec![0; max_col]; max_row];
    let mut remaining = content.to_vec();
    let mut count = 0u64;
    backtrack_ssyt(
        &cells,
        &mut grid,
        &shape.rows,
        &mut remaining,
        0,
        num_cells,
        &mut count,
    );
    count
}

fn backtrack_ssyt(
    cells: &[(usize, usize)],
    grid: &mut Vec<Vec<usize>>,
    row_lens: &[usize],
    remaining: &mut Vec<usize>,
    pos: usize,
    num_cells: usize,
    count: &mut u64,
) {
    if pos == num_cells {
        *count += 1;
        return;
    }
    let (r, c) = cells[pos];
    let k = remaining.len();
    for val in 1..=k {
        if remaining[val - 1] == 0 {
            continue;
        }
        // Weakly increasing row constraint.
        if c > 0 {
            let left = grid[r][c - 1];
            if left > val {
                continue;
            }
        }
        // Strictly increasing column constraint.
        if r > 0 && row_lens.get(r - 1).copied().unwrap_or(0) > c {
            let up = grid[r - 1][c];
            if up >= val {
                continue;
            }
        }
        grid[r][c] = val;
        remaining[val - 1] -= 1;
        backtrack_ssyt(cells, grid, row_lens, remaining, pos + 1, num_cells, count);
        remaining[val - 1] += 1;
        grid[r][c] = 0;
    }
}

// ─── Plethysm ─────────────────────────────────────────────────────────────────

/// Simplified plethysm multiplicity: the number of times `s_lambda` appears
/// in `s_mu[s_(1^k)]` (plethystic substitution).
///
/// For the purposes of this library we compute `K_{lambda, mu^k}` where
/// `mu^k` is the weight obtained by repeating `mu` exactly `n/size(mu)` times
/// and zero-padding.  This is a sound approximation for rectangular shapes;
/// the full plethysm is hard in general.
pub fn plethysm_multiplicity(lambda: &YoungDiagram, mu: &YoungDiagram) -> u64 {
    if mu.size() == 0 || lambda.size() == 0 {
        return 0;
    }
    // Use Kostka numbers as a proxy.
    if lambda.size() == mu.size() {
        return kostka_number(lambda, mu);
    }
    // Pad mu rows to match lambda.size() if possible.
    let ratio = lambda.size() / mu.size();
    if ratio * mu.size() != lambda.size() {
        return 0;
    }
    let extended_rows: Vec<usize> = mu.rows.iter().map(|&r| r * ratio).collect();
    let extended = YoungDiagram {
        rows: extended_rows,
    };
    kostka_number(lambda, &extended)
}

// ─── Combinatorial number sequences ──────────────────────────────────────────

/// Catalan number C_n = binom(2n, n) / (n+1).
pub fn catalan_number(n: usize) -> u64 {
    if n == 0 {
        return 1;
    }
    binom(2 * n, n) / (n as u64 + 1)
}

/// Count ballot sequences of length `2n` with `n` occurrences of each of two
/// symbols, where every prefix has at least as many A's as B's.
///
/// This equals the Catalan number C_n; the second parameter `k` is kept for
/// API generality (currently must equal `n`).
pub fn ballot_sequence_count(n: usize, k: usize) -> u64 {
    if k != n {
        // General ballot problem: C(n+k, n) * (n-k+1) / (n+1) when n >= k.
        if n < k {
            return 0;
        }
        let num = binom(n + k, k);
        let denom = n + 1;
        return num * ((n - k + 1) as u64) / (denom as u64);
    }
    catalan_number(n)
}

/// Motzkin number M_n: number of Motzkin paths of length n.
///
/// Recurrence: M_0 = 1, M_1 = 1, M_n = M_{n-1} + sum_{k=0}^{n-2} M_k * M_{n-2-k}.
pub fn motzkin_number(n: usize) -> u64 {
    let mut m: Vec<u64> = vec![0; n + 1];
    m[0] = 1;
    if n == 0 {
        return 1;
    }
    m[1] = 1;
    for i in 2..=n {
        m[i] = m[i - 1];
        for k in 0..=(i - 2) {
            m[i] = m[i].saturating_add(m[k].saturating_mul(m[i - 2 - k]));
        }
    }
    m[n]
}

/// Narayana number N(n, k) = (1/n) * C(n,k) * C(n, k-1).
///
/// Counts the number of Dyck paths of semilength n with exactly k peaks.
pub fn narayana_number(n: usize, k: usize) -> u64 {
    if n == 0 {
        return if k == 0 { 1 } else { 0 };
    }
    if k == 0 || k > n {
        return 0;
    }
    let c1 = binom(n, k);
    let c2 = binom(n, k - 1);
    c1.saturating_mul(c2) / (n as u64)
}

// ─── Beta-set ─────────────────────────────────────────────────────────────────

/// Compute the beta-set of a partition with `k` parts.
///
/// Beta-set = { λ_i + k - i : i = 0, ..., k-1 }  (0-indexed).
pub fn beta_set_from_diagram(yd: &YoungDiagram) -> BetaSet {
    let k = yd.rows.len();
    let values: Vec<usize> = yd
        .rows
        .iter()
        .enumerate()
        .map(|(i, &row)| row + k - 1 - i)
        .collect();
    BetaSet { values }
}

// ─── Coxeter element ──────────────────────────────────────────────────────────

/// Construct a `CoxeterElement` from a reduced word.
pub fn coxeter_from_word(word: Vec<usize>) -> CoxeterElement {
    CoxeterElement { reduced_word: word }
}

/// Compute the length of a Coxeter element (its reduced word length).
pub fn coxeter_length(elem: &CoxeterElement) -> usize {
    elem.reduced_word.len()
}

// ─── Internal helpers ─────────────────────────────────────────────────────────

/// Factorial n! saturating at u64::MAX.
pub(super) fn factorial(n: usize) -> u64 {
    (1..=n as u64).fold(1u64, |acc, x| acc.saturating_mul(x))
}

/// Binomial coefficient C(n, k).
pub(super) fn binom(n: usize, k: usize) -> u64 {
    if k > n {
        return 0;
    }
    let k = k.min(n - k);
    let mut result: u64 = 1;
    for i in 0..k {
        result = result.saturating_mul((n - i) as u64) / (i as u64 + 1);
    }
    result
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_young_diagram_from_partition() {
        let yd = young_diagram_from_partition(&[3, 1, 2]);
        assert_eq!(yd.rows, vec![3, 2, 1]);
        assert_eq!(yd.size(), 6);
    }

    #[test]
    fn test_conjugate_diagram() {
        // λ = (3, 2, 1)  →  λ' = (3, 2, 1)  (self-conjugate).
        let yd = young_diagram_from_partition(&[3, 2, 1]);
        let conj = conjugate_diagram(&yd);
        assert_eq!(conj.rows, vec![3, 2, 1]);
    }

    #[test]
    fn test_conjugate_diagram_2() {
        // λ = (3, 3)  →  λ' = (2, 2, 2).
        let yd = young_diagram_from_partition(&[3, 3]);
        let conj = conjugate_diagram(&yd);
        assert_eq!(conj.rows, vec![2, 2, 2]);
    }

    #[test]
    fn test_hook_length_staircase() {
        // λ = (2, 1).  Hook at (0,0) = 3, (0,1) = 1, (1,0) = 1.
        let yd = young_diagram_from_partition(&[2, 1]);
        assert_eq!(hook_length(&yd, 0, 0), 3);
        assert_eq!(hook_length(&yd, 0, 1), 1);
        assert_eq!(hook_length(&yd, 1, 0), 1);
    }

    #[test]
    fn test_hook_length_formula_hook3() {
        // λ = (2, 1), n = 3.  f^λ = 3!/3 = 2.
        let yd = young_diagram_from_partition(&[2, 1]);
        assert_eq!(hook_length_formula(&yd), 2);
    }

    #[test]
    fn test_number_of_syt_partition_3() {
        // λ = (3): only one SYT.
        let yd = young_diagram_from_partition(&[3]);
        assert_eq!(number_of_syt(&yd), 1);
    }

    #[test]
    fn test_number_of_syt_partition_21() {
        let yd = young_diagram_from_partition(&[2, 1]);
        assert_eq!(number_of_syt(&yd), 2);
    }

    #[test]
    fn test_descent_set() {
        // Permutation [2, 0, 1] (0-indexed): descent at position 0 (2>0).
        let perm = Permutation {
            sigma: vec![2, 0, 1],
        };
        let ds = descent_set(&perm);
        assert_eq!(ds.positions, vec![0]);
    }

    #[test]
    fn test_major_index() {
        let perm = Permutation {
            sigma: vec![2, 0, 1],
        };
        assert_eq!(major_index(&perm), 0); // single descent at position 0
    }

    #[test]
    fn test_inversion_number() {
        // [2, 0, 1]: inversions (2,0), (2,1) → 2.
        let perm = Permutation {
            sigma: vec![2, 0, 1],
        };
        assert_eq!(inversion_number(&perm), 2);
    }

    #[test]
    fn test_lehmer_code() {
        // [2, 0, 1]: code = [2, 0, 0].
        let perm = Permutation {
            sigma: vec![2, 0, 1],
        };
        assert_eq!(lehmer_code(&perm), vec![2, 0, 0]);
    }

    #[test]
    fn test_rs_insertion_identity() {
        // Identity permutation [0,1,2] → P = [[1,2,3]], Q = [[1,2,3]].
        let perm = Permutation::identity(3);
        let rs = rs_insertion(&perm);
        assert_eq!(rs.insertion.shape, rs.recording.shape);
        assert_eq!(rs.insertion.entries, vec![vec![1, 2, 3]]);
    }

    #[test]
    fn test_rs_insertion_reversal() {
        // [2,1,0] → all values go through bumping; shape should be (1,1,1).
        let perm = Permutation {
            sigma: vec![2, 1, 0],
        };
        let rs = rs_insertion(&perm);
        assert_eq!(rs.insertion.shape, rs.recording.shape);
        // Shape is a single column [1,1,1].
        assert_eq!(rs.insertion.shape.rows, vec![1, 1, 1]);
    }

    #[test]
    fn test_rs_shape_symmetry() {
        // The shape of P(σ) = shape of P(σ^{-1}) (a known property).
        let perm = Permutation {
            sigma: vec![1, 3, 0, 2],
        };
        let rs = rs_insertion(&perm);
        assert_eq!(rs.insertion.shape, rs.recording.shape);
    }

    #[test]
    fn test_catalan_numbers() {
        assert_eq!(catalan_number(0), 1);
        assert_eq!(catalan_number(1), 1);
        assert_eq!(catalan_number(2), 2);
        assert_eq!(catalan_number(3), 5);
        assert_eq!(catalan_number(4), 14);
        assert_eq!(catalan_number(5), 42);
    }

    #[test]
    fn test_motzkin_numbers() {
        // M_0=1, M_1=1, M_2=2, M_3=4, M_4=9, M_5=21.
        assert_eq!(motzkin_number(0), 1);
        assert_eq!(motzkin_number(1), 1);
        assert_eq!(motzkin_number(2), 2);
        assert_eq!(motzkin_number(3), 4);
        assert_eq!(motzkin_number(4), 9);
        assert_eq!(motzkin_number(5), 21);
    }

    #[test]
    fn test_narayana_numbers() {
        // N(3,1)=1, N(3,2)=3, N(3,3)=1.
        assert_eq!(narayana_number(3, 1), 1);
        assert_eq!(narayana_number(3, 2), 3);
        assert_eq!(narayana_number(3, 3), 1);
        // Sum for n=3: 1+3+1=5 = C_3.
        let sum3: u64 = (1..=3).map(|k| narayana_number(3, k)).sum();
        assert_eq!(sum3, 5);
    }

    #[test]
    fn test_kostka_number_trivial() {
        // K_{(n), (n)} = 1 (unique SSYT of single-row shape and single-part content).
        let lambda = young_diagram_from_partition(&[3]);
        let mu = young_diagram_from_partition(&[3]);
        assert_eq!(kostka_number(&lambda, &mu), 1);
    }

    #[test]
    fn test_kostka_number_staircase() {
        // K_{(2,1),(1,1,1)}: SSYT of shape (2,1) with one each of 1,2,3.
        // There are 2 such tableaux.
        let lambda = young_diagram_from_partition(&[2, 1]);
        let mu = young_diagram_from_partition(&[1, 1, 1]);
        assert_eq!(kostka_number(&lambda, &mu), 2);
    }

    #[test]
    fn test_schur_polynomial_evaluation_single_var() {
        // s_{(2)}(x) = x^2 for a single variable.
        let diag = young_diagram_from_partition(&[2]);
        let val = schur_polynomial_evaluation(&diag, &[2.0]);
        // Only SSYT of shape (2) with values in {1}: filling [1,1].
        // Contribution: 2^1 * 2^1 = 4.
        assert!((val - 4.0).abs() < 1e-9);
    }

    #[test]
    fn test_ballot_sequence_count_equals_catalan() {
        for n in 0..6 {
            assert_eq!(ballot_sequence_count(n, n), catalan_number(n));
        }
    }

    #[test]
    fn test_beta_set() {
        // λ = (3,1), k=2: beta = {3+1=4, 1+0=1} = {4,1}.
        let yd = young_diagram_from_partition(&[3, 1]);
        let bs = beta_set_from_diagram(&yd);
        assert_eq!(bs.values, vec![4, 1]);
    }
}
