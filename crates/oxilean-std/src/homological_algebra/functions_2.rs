//! Spec-required functions for homological algebra.
//!
//! Smith normal form, chain complex homology, Betti numbers,
//! Euler characteristic, exactness checks, and standard complexes.

use super::types::{
    AbelianGroup, CochainComplex, HomologyGroup, LongExactSequence, SpecChainComplex, SpecChainMap,
    SpecExtGroup, SpecShortExactSequence, TorGroup,
};

// ── Smith normal form ────────────────────────────────────────────────────────

/// Compute the Smith normal form of an integer matrix.
///
/// Returns `(P, D, Q)` such that `P * matrix * Q = D` where `D` is diagonal
/// with non-negative diagonal entries d_1 | d_2 | … | d_r (divisibility chain).
/// P and Q are invertible integer matrices (unimodular).
///
/// Algorithm: column and row reduction with elementary integer operations.
pub fn smith_normal_form(matrix: &[Vec<i64>]) -> (Vec<Vec<i64>>, Vec<Vec<i64>>, Vec<Vec<i64>>) {
    if matrix.is_empty() {
        return (vec![], vec![], vec![]);
    }
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut d: Vec<Vec<i64>> = matrix.to_vec();
    let mut p: Vec<Vec<i64>> = identity_matrix(rows);
    let mut q: Vec<Vec<i64>> = identity_matrix(cols);

    let mut pivot_row = 0usize;
    let mut pivot_col = 0usize;

    while pivot_row < rows && pivot_col < cols {
        // Find the non-zero entry with smallest absolute value.
        let found = find_pivot(&d, pivot_row, pivot_col, rows, cols);
        let (min_r, min_c) = match found {
            None => break,
            Some(pos) => pos,
        };

        // Swap pivot to (pivot_row, pivot_col).
        if min_r != pivot_row {
            d.swap(min_r, pivot_row);
            p.swap(min_r, pivot_row);
        }
        if min_c != pivot_col {
            for row in &mut d {
                row.swap(min_c, pivot_col);
            }
            for row in &mut q {
                row.swap(min_c, pivot_col);
            }
        }

        // Ensure pivot is positive.
        if d[pivot_row][pivot_col] < 0 {
            for j in 0..cols {
                d[pivot_row][j] = -d[pivot_row][j];
            }
            for j in 0..cols {
                p[pivot_row][j] = -p[pivot_row][j];
            }
        }

        // Eliminate the pivot row and column using a convergent GCD-based algorithm.
        // We iterate until the pivot entry divides all other entries in its row and column,
        // and also divides every other entry in the submatrix (SNF divisibility condition).
        loop {
            let mut made_progress = false;

            // Eliminate column: for each other row r, subtract floor(d[r][pc]/pivot) * pivot_row.
            for r in 0..rows {
                if r == pivot_row {
                    continue;
                }
                let entry = d[r][pivot_col];
                if entry == 0 {
                    continue;
                }
                let piv = d[pivot_row][pivot_col];
                if piv == 0 {
                    break;
                }
                let q_val = entry / piv;
                if q_val == 0 {
                    // entry is non-zero but |entry| < |piv|: swap rows to get a smaller pivot.
                    d.swap(r, pivot_row);
                    p.swap(r, pivot_row);
                    made_progress = true;
                    continue;
                }
                for c in 0..cols {
                    let sub = q_val * d[pivot_row][c];
                    d[r][c] -= sub;
                    let sub_p = q_val * p[pivot_row][c];
                    p[r][c] -= sub_p;
                }
                made_progress = true;
            }

            // Eliminate row: for each other column c, subtract floor(d[pr][c]/pivot) * pivot_col.
            for c in 0..cols {
                if c == pivot_col {
                    continue;
                }
                let entry = d[pivot_row][c];
                if entry == 0 {
                    continue;
                }
                let piv = d[pivot_row][pivot_col];
                if piv == 0 {
                    break;
                }
                let q_val = entry / piv;
                if q_val == 0 {
                    // entry is non-zero but |entry| < |piv|: swap columns.
                    for row in &mut d {
                        row.swap(c, pivot_col);
                    }
                    for row in &mut q {
                        row.swap(c, pivot_col);
                    }
                    made_progress = true;
                    continue;
                }
                for r in 0..rows {
                    let sub = q_val * d[r][pivot_col];
                    d[r][c] -= sub;
                    let sub_q = q_val * q[r][pivot_col];
                    q[r][c] -= sub_q;
                }
                made_progress = true;
            }

            // Ensure pivot is positive after swaps/negations.
            let piv = d[pivot_row][pivot_col];
            if piv < 0 {
                for j in 0..cols {
                    d[pivot_row][j] = -d[pivot_row][j];
                    p[pivot_row][j] = -p[pivot_row][j];
                }
            }

            // Check whether row and column are fully zeroed except at pivot.
            let col_clear = (0..rows).all(|r| r == pivot_row || d[r][pivot_col] == 0);
            let row_clear = (0..cols).all(|c| c == pivot_col || d[pivot_row][c] == 0);

            if !col_clear || !row_clear {
                // Not done: loop back to eliminate remaining entries.
                continue;
            }

            // Row and column are clear. Check SNF divisibility: pivot must divide all
            // remaining submatrix entries. If not, absorb a non-divisible entry by adding
            // its row to the pivot row and continue.
            let piv = d[pivot_row][pivot_col];
            if piv == 0 {
                break;
            }
            let mut found_non_div = false;
            'div_check: for r in (pivot_row + 1)..rows {
                for c in (pivot_col + 1)..cols {
                    if d[r][c] % piv != 0 {
                        // Add row r to pivot_row to mix the gcd.
                        for c2 in 0..cols {
                            let add = d[r][c2];
                            d[pivot_row][c2] += add;
                            let add_p = p[r][c2];
                            p[pivot_row][c2] += add_p;
                        }
                        found_non_div = true;
                        break 'div_check;
                    }
                }
            }

            if !found_non_div && !made_progress {
                // Converged: row, column clear and divisibility holds.
                break;
            }
            if !found_non_div && made_progress {
                // Made progress this round; check again cleanly.
                continue;
            }
            // found_non_div: loop again to re-eliminate with new pivot row content.
        }

        pivot_row += 1;
        pivot_col += 1;
    }

    // Ensure diagonal entries are non-negative.
    for i in 0..rows.min(cols) {
        if d[i][i] < 0 {
            for j in 0..cols {
                d[i][j] = -d[i][j];
            }
            for j in 0..cols {
                p[i][j] = -p[i][j];
            }
        }
    }

    (p, d, q)
}

fn identity_matrix(n: usize) -> Vec<Vec<i64>> {
    (0..n)
        .map(|i| (0..n).map(|j| if i == j { 1 } else { 0 }).collect())
        .collect()
}

fn find_pivot(
    d: &[Vec<i64>],
    from_row: usize,
    from_col: usize,
    rows: usize,
    cols: usize,
) -> Option<(usize, usize)> {
    let mut best: Option<(i64, usize, usize)> = None;
    for r in from_row..rows {
        for c in from_col..cols {
            let v = d[r][c].unsigned_abs() as i64;
            if v > 0 {
                match best {
                    None => best = Some((v, r, c)),
                    Some((bv, _, _)) if v < bv => best = Some((v, r, c)),
                    _ => {}
                }
            }
        }
    }
    best.map(|(_, r, c)| (r, c))
}

/// Extract the rank of the image from the Smith diagonal.
fn image_rank_from_snf(diagonal: &[Vec<i64>]) -> usize {
    diagonal
        .iter()
        .enumerate()
        .filter(|&(i, row)| i < row.len() && row[i] != 0)
        .count()
}

/// Extract torsion coefficients (diagonal entries > 1) from SNF diagonal.
fn torsion_from_snf(diagonal: &[Vec<i64>]) -> Vec<u64> {
    diagonal
        .iter()
        .enumerate()
        .filter_map(|(i, row)| {
            if i < row.len() {
                let v = row[i];
                if v > 1 {
                    Some(v as u64)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

// ── Matrix arithmetic helpers ────────────────────────────────────────────────

/// Matrix multiplication A (m×k) * B (k×n) → C (m×n).
fn mat_mul(a: &[Vec<i64>], b: &[Vec<i64>]) -> Vec<Vec<i64>> {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let m = a.len();
    let k = b.len();
    let n = b[0].len();
    let mut c = vec![vec![0i64; n]; m];
    for i in 0..m {
        for j in 0..n {
            for l in 0..k {
                c[i][j] += a[i][l] * b[l][j];
            }
        }
    }
    c
}

/// Number of rows of a matrix.
fn nrows(m: &[Vec<i64>]) -> usize {
    m.len()
}

/// Number of columns of a matrix (0 if empty).
fn ncols(m: &[Vec<i64>]) -> usize {
    m.first().map_or(0, |r| r.len())
}

// ── Homology computation ─────────────────────────────────────────────────────

/// Compute H_n of a chain complex at degree `degree`.
///
/// H_n = ker(d_n) / im(d_{n+1})
///
/// We use Smith normal form to compute the ranks:
///   rank ker d_n = dim C_n − rank d_n
///   rank im d_{n+1} = rank d_{n+1}
pub fn compute_homology(complex: &SpecChainComplex, degree: usize) -> HomologyGroup {
    if degree >= complex.groups.len() {
        return HomologyGroup::zero(degree as i32);
    }

    let c_n_rank = complex.groups[degree].rank();

    // Rank of d_n (differential out of C_n, i.e., differentials[degree-1] if degree > 0).
    let rank_dn = if degree == 0 {
        0
    } else if degree - 1 < complex.differentials.len() {
        let (_, d, _) = smith_normal_form(&complex.differentials[degree - 1]);
        image_rank_from_snf(&d)
    } else {
        0
    };

    // Rank of d_{n+1} (differential into C_n from C_{n+1}).
    let (rank_dn1, torsion) = if degree < complex.differentials.len() {
        let (_, d, _) = smith_normal_form(&complex.differentials[degree]);
        (image_rank_from_snf(&d), torsion_from_snf(&d))
    } else {
        (0, Vec::new())
    };

    let ker_rank = c_n_rank.saturating_sub(rank_dn);
    let free_rank = ker_rank.saturating_sub(rank_dn1);

    HomologyGroup::new(degree as i32, free_rank, torsion)
}

/// Compute all homology groups of the complex.
pub fn compute_all_homology(complex: &SpecChainComplex) -> Vec<HomologyGroup> {
    (0..complex.groups.len())
        .map(|n| compute_homology(complex, n))
        .collect()
}

// ── Exactness & splitting ─────────────────────────────────────────────────────

/// Check whether im(f: A → B) = ker(g: B → C) in a short exact sequence.
///
/// We verify:
/// 1. g∘f = 0  (im f ⊆ ker g).
/// 2. rank(im f) = rank(ker g) = rank(B) − rank(g).
pub fn is_exact_sequence(ses: &SpecShortExactSequence) -> bool {
    let f = &ses.maps[0];
    let g = &ses.maps[1];

    // g ∘ f must be the zero matrix.
    if !nrows(f).eq(&0) && !ncols(g).eq(&0) {
        let gf = mat_mul(g, f);
        for row in &gf {
            for &v in row {
                if v != 0 {
                    return false;
                }
            }
        }
    }

    // rank im(f) = rank f.
    let (_, df, _) = smith_normal_form(f);
    let rank_f = image_rank_from_snf(&df);

    // rank ker(g) = cols(g) − rank(g).
    let (_, dg, _) = smith_normal_form(g);
    let rank_g = image_rank_from_snf(&dg);
    let b_rank = ses.groups[1].rank();
    let ker_g_rank = b_rank.saturating_sub(rank_g);

    rank_f == ker_g_rank
}

/// Splitting lemma: the short exact sequence 0 → A → B → C → 0 splits
/// (i.e., B ≅ A ⊕ C) iff the sequence is exact and B has rank = rank(A) + rank(C).
///
/// For torsion-free abelian groups the condition is purely rank-based.
pub fn splitting_lemma(ses: &SpecShortExactSequence) -> bool {
    if !is_exact_sequence(ses) {
        return false;
    }
    let rank_a = ses.groups[0].rank();
    let rank_b = ses.groups[1].rank();
    let rank_c = ses.groups[2].rank();
    rank_b == rank_a + rank_c
}

// ── Chain map verification ───────────────────────────────────────────────────

/// Check whether `map` is a chain map between `source` and `target` complexes,
/// i.e., d_D ∘ f_n = f_{n-1} ∘ d_C for all n.
pub fn check_chain_map(
    map: &SpecChainMap,
    source: &SpecChainComplex,
    target: &SpecChainComplex,
) -> bool {
    let len = map.maps.len();
    if len == 0 {
        return true;
    }

    for n in 1..len {
        let fn_map = &map.maps[n];
        let fn1_map = &map.maps[n - 1];

        // d_D^{n-1}: target.differentials[n-2] if n>=2, else trivial.
        // d_C^n: source.differentials[n-1].
        let has_src_diff = n - 1 < source.differentials.len();
        let has_tgt_diff = n - 1 < target.differentials.len();

        if !has_src_diff && !has_tgt_diff {
            continue;
        }

        // lhs = f_{n-1} ∘ d_C^n
        let lhs = if has_src_diff {
            let dc = &source.differentials[n - 1];
            if nrows(fn1_map) == 0 || ncols(dc) == 0 {
                vec![]
            } else {
                mat_mul(fn1_map, dc)
            }
        } else {
            vec![]
        };

        // rhs = d_D^{n-1} ∘ f_n
        let rhs = if has_tgt_diff {
            let dd = &target.differentials[n - 1];
            if nrows(dd) == 0 || ncols(fn_map) == 0 {
                vec![]
            } else {
                mat_mul(dd, fn_map)
            }
        } else {
            vec![]
        };

        if lhs != rhs {
            return false;
        }
    }
    true
}

// ── Euler characteristic & Betti numbers ────────────────────────────────────

/// Compute the Euler characteristic χ = Σ (-1)^n β_n where β_n = free_rank(H_n).
pub fn euler_characteristic(complex: &SpecChainComplex) -> i64 {
    compute_all_homology(complex)
        .iter()
        .enumerate()
        .map(|(n, h)| {
            let sign: i64 = if n % 2 == 0 { 1 } else { -1 };
            sign * h.free_rank as i64
        })
        .sum()
}

/// Compute the Betti numbers β_n = free_rank(H_n) for each degree.
pub fn betti_numbers(complex: &SpecChainComplex) -> Vec<usize> {
    compute_all_homology(complex)
        .iter()
        .map(|h| h.free_rank)
        .collect()
}

// ── Standard complexes ───────────────────────────────────────────────────────

/// The chain complex of S¹ (circle):
///   C_1 = Z (generated by the 1-cell), C_0 = Z (generated by the vertex)
///   d_1 = 0 (the boundary of the 1-cell is vertex − vertex = 0).
///
/// H_0 = Z, H_1 = Z.
pub fn circle_complex() -> SpecChainComplex {
    let c0 = AbelianGroup::free(1); // Z
    let c1 = AbelianGroup::free(1); // Z
                                    // d_1: C_1 → C_0 is the zero map (1×1 zero matrix).
    let d1 = vec![vec![0i64]];
    SpecChainComplex::new(vec![c0, c1], vec![d1])
}

/// The chain complex of S^n (n-sphere):
///   C_n = Z, C_0 = Z, all others 0, all differentials 0.
///
/// H_0 = Z, H_n = Z, H_k = 0 for 0 < k < n.
pub fn sphere_complex(n: u32) -> SpecChainComplex {
    let n = n as usize;
    let groups: Vec<AbelianGroup> = (0..=n)
        .map(|k| {
            if k == 0 || k == n {
                AbelianGroup::free(1)
            } else {
                AbelianGroup::free(0)
            }
        })
        .collect();

    // Differentials: all zero.
    let mut differentials: Vec<Vec<Vec<i64>>> = Vec::new();
    for k in 0..n {
        let src_rank = groups[k + 1].rank();
        let tgt_rank = groups[k].rank();
        let zero = vec![vec![0i64; src_rank]; tgt_rank];
        differentials.push(zero);
    }

    SpecChainComplex::new(groups, differentials)
}

/// The chain complex of T² (torus):
///   C_2 = Z, C_1 = Z², C_0 = Z
///   d_2: C_2 → C_1 is the zero map (2×1 zero matrix).
///   d_1: C_1 → C_0 is the zero map (1×2 zero matrix).
///
/// H_0 = Z, H_1 = Z², H_2 = Z.
pub fn torus_complex() -> SpecChainComplex {
    let c0 = AbelianGroup::free(1);
    let c1 = AbelianGroup::free(2);
    let c2 = AbelianGroup::free(1);
    // d_2: C_2(rank 1) → C_1(rank 2): 2×1 zero matrix.
    let d2 = vec![vec![0i64], vec![0i64]];
    // d_1: C_1(rank 2) → C_0(rank 1): 1×2 zero matrix.
    let d1 = vec![vec![0i64, 0i64]];
    SpecChainComplex::new(vec![c0, c1, c2], vec![d1, d2])
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests_spec_homological {
    use super::*;

    // ── Smith normal form tests ──────────────────────────────────────────────

    #[test]
    fn test_snf_identity() {
        let m = vec![vec![1, 0], vec![0, 1]];
        let (_, d, _) = smith_normal_form(&m);
        assert_eq!(d[0][0], 1);
        assert_eq!(d[1][1], 1);
    }

    #[test]
    fn test_snf_zero_matrix() {
        let m = vec![vec![0, 0], vec![0, 0]];
        let (_, d, _) = smith_normal_form(&m);
        assert_eq!(d[0][0], 0);
        assert_eq!(d[1][1], 0);
    }

    #[test]
    fn test_snf_single_entry() {
        let m = vec![vec![6i64]];
        let (_, d, _) = smith_normal_form(&m);
        assert_eq!(d[0][0], 6);
    }

    #[test]
    fn test_snf_image_rank() {
        // 2×2 matrix of rank 1.
        let m = vec![vec![2, 4], vec![1, 2]];
        let (_, d, _) = smith_normal_form(&m);
        assert_eq!(image_rank_from_snf(&d), 1);
    }

    #[test]
    fn test_snf_empty() {
        let m: Vec<Vec<i64>> = vec![];
        let (p, d, q) = smith_normal_form(&m);
        assert!(p.is_empty());
        assert!(d.is_empty());
        assert!(q.is_empty());
    }

    #[test]
    fn test_snf_diagonal_positive() {
        let m = vec![vec![3, 0], vec![0, 6]];
        let (_, d, _) = smith_normal_form(&m);
        assert!(d[0][0] >= 0);
        assert!(d[1][1] >= 0);
    }

    // ── Homology tests ───────────────────────────────────────────────────────

    #[test]
    fn test_circle_homology_h0() {
        let c = circle_complex();
        let h0 = compute_homology(&c, 0);
        assert_eq!(h0.free_rank, 1, "H_0(S^1) = Z");
    }

    #[test]
    fn test_circle_homology_h1() {
        let c = circle_complex();
        let h1 = compute_homology(&c, 1);
        assert_eq!(h1.free_rank, 1, "H_1(S^1) = Z");
    }

    #[test]
    fn test_torus_homology_h0() {
        let c = torus_complex();
        let h0 = compute_homology(&c, 0);
        assert_eq!(h0.free_rank, 1, "H_0(T^2) = Z");
    }

    #[test]
    fn test_torus_homology_h1() {
        let c = torus_complex();
        let h1 = compute_homology(&c, 1);
        assert_eq!(h1.free_rank, 2, "H_1(T^2) = Z^2");
    }

    #[test]
    fn test_torus_homology_h2() {
        let c = torus_complex();
        let h2 = compute_homology(&c, 2);
        assert_eq!(h2.free_rank, 1, "H_2(T^2) = Z");
    }

    #[test]
    fn test_sphere_2_h0_h2() {
        let c = sphere_complex(2);
        let h0 = compute_homology(&c, 0);
        let h2 = compute_homology(&c, 2);
        assert_eq!(h0.free_rank, 1, "H_0(S^2) = Z");
        assert_eq!(h2.free_rank, 1, "H_2(S^2) = Z");
    }

    #[test]
    fn test_sphere_2_h1_trivial() {
        let c = sphere_complex(2);
        let h1 = compute_homology(&c, 1);
        assert_eq!(h1.free_rank, 0, "H_1(S^2) = 0");
    }

    #[test]
    fn test_compute_all_homology_circle() {
        let c = circle_complex();
        let all = compute_all_homology(&c);
        assert_eq!(all.len(), 2);
        assert_eq!(all[0].free_rank, 1);
        assert_eq!(all[1].free_rank, 1);
    }

    #[test]
    fn test_compute_all_homology_torus() {
        let c = torus_complex();
        let all = compute_all_homology(&c);
        assert_eq!(all.len(), 3);
        assert_eq!(all[0].free_rank, 1);
        assert_eq!(all[1].free_rank, 2);
        assert_eq!(all[2].free_rank, 1);
    }

    // ── Euler characteristic and Betti numbers ───────────────────────────────

    #[test]
    fn test_euler_circle() {
        let c = circle_complex();
        assert_eq!(euler_characteristic(&c), 0, "χ(S^1) = 0");
    }

    #[test]
    fn test_euler_torus() {
        let c = torus_complex();
        assert_eq!(euler_characteristic(&c), 0, "χ(T^2) = 0");
    }

    #[test]
    fn test_euler_sphere_2() {
        let c = sphere_complex(2);
        assert_eq!(euler_characteristic(&c), 2, "χ(S^2) = 2");
    }

    #[test]
    fn test_betti_circle() {
        let c = circle_complex();
        let b = betti_numbers(&c);
        assert_eq!(b, vec![1, 1]);
    }

    #[test]
    fn test_betti_torus() {
        let c = torus_complex();
        let b = betti_numbers(&c);
        assert_eq!(b, vec![1, 2, 1]);
    }

    // ── Exactness & splitting ────────────────────────────────────────────────

    #[test]
    fn test_exact_sequence_split_trivial() {
        // 0 → Z → Z → 0: f = id, g = zero.
        let a = AbelianGroup::free(1);
        let b = AbelianGroup::free(1);
        let c = AbelianGroup::free(0);
        let f = vec![vec![1i64]];
        let g: Vec<Vec<i64>> = vec![vec![]];
        let ses = SpecShortExactSequence::new(a, b, c, f, g);
        // g∘f should be zero (empty product).
        assert!(is_exact_sequence(&ses));
    }

    #[test]
    fn test_splitting_lemma_split() {
        // 0 → Z → Z² → Z → 0 splits.
        let a = AbelianGroup::free(1);
        let b = AbelianGroup::free(2);
        let c = AbelianGroup::free(1);
        // f: Z → Z²  [1,0]ᵀ,  g: Z² → Z  [0,1].
        let f = vec![vec![1i64], vec![0]];
        let g = vec![vec![0i64, 1]];
        let ses = SpecShortExactSequence::new(a, b, c, f, g);
        assert!(splitting_lemma(&ses));
    }

    // ── Chain map tests ──────────────────────────────────────────────────────

    #[test]
    fn test_check_chain_map_identity() {
        let c = circle_complex();
        // Identity chain map.
        let maps = vec![vec![vec![1i64]], vec![vec![1i64]]];
        let map = SpecChainMap::new(0, 0, maps);
        assert!(check_chain_map(&map, &c, &c));
    }

    #[test]
    fn test_check_chain_map_empty() {
        let c = circle_complex();
        let map = SpecChainMap::new(0, 0, vec![]);
        assert!(check_chain_map(&map, &c, &c));
    }

    // ── Type construction tests ──────────────────────────────────────────────

    #[test]
    fn test_abelian_group_free() {
        let g = AbelianGroup::free(3);
        assert_eq!(g.rank(), 3);
        assert!(g.relations.is_empty());
    }

    #[test]
    fn test_abelian_group_cyclic() {
        let g = AbelianGroup::cyclic(5);
        assert_eq!(g.generators.len(), 1);
        assert_eq!(g.relations[0][0], 5);
    }

    #[test]
    fn test_homology_group_zero() {
        let h = HomologyGroup::zero(3);
        assert!(h.is_zero());
        assert_eq!(h.degree, 3);
    }

    #[test]
    fn test_homology_group_not_zero() {
        let h = HomologyGroup::new(1, 2, vec![3]);
        assert!(!h.is_zero());
    }

    #[test]
    fn test_cochain_complex_new() {
        let g = AbelianGroup::free(1);
        let cc = CochainComplex::new(vec![g], vec![]);
        assert_eq!(cc.groups.len(), 1);
    }

    #[test]
    fn test_long_exact_sequence_new() {
        let les = LongExactSequence::new(vec![AbelianGroup::free(1)], vec![]);
        assert_eq!(les.groups.len(), 1);
    }

    #[test]
    fn test_spec_ext_group() {
        let h = HomologyGroup::new(0, 1, vec![]);
        let ext = SpecExtGroup::new(1, 2, h);
        assert_eq!(ext.p, 1);
        assert_eq!(ext.q, 2);
        assert_eq!(ext.group.free_rank, 1);
    }

    #[test]
    fn test_tor_group() {
        let h = HomologyGroup::new(0, 0, vec![2]);
        let tor = TorGroup::new(0, 1, h);
        assert_eq!(tor.p, 0);
        assert_eq!(tor.q, 1);
        assert_eq!(tor.group.torsion, vec![2]);
    }

    #[test]
    fn test_spec_chain_map_new() {
        let m = SpecChainMap::new(0, 1, vec![vec![vec![1]]]);
        assert_eq!(m.source, 0);
        assert_eq!(m.target, 1);
        assert_eq!(m.maps.len(), 1);
    }

    #[test]
    fn test_sphere_complex_0() {
        let c = sphere_complex(0);
        // S^0 has C_0 = Z; H_0 = Z.
        assert_eq!(c.groups.len(), 1);
        let h0 = compute_homology(&c, 0);
        assert_eq!(h0.free_rank, 1);
    }

    #[test]
    fn test_sphere_complex_3() {
        let c = sphere_complex(3);
        let h3 = compute_homology(&c, 3);
        assert_eq!(h3.free_rank, 1, "H_3(S^3) = Z");
    }
}
