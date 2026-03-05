//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::functions::*;

/// `Array.prefix_sum_correct : ∀ {n}, Array Nat n → Prop`
///
/// Correctness of prefix sums: for all valid ranges [l, r],
/// the range sum equals prefix[r] - prefix[l-1].
#[allow(dead_code)]
pub fn arr_ext_prefix_sum_correct_ty() -> Expr {
    implicit_pi(
        "n",
        nat_ty(),
        default_pi("arr", array_of(nat_ty(), Expr::BVar(0)), prop()),
    )
}
/// `Array.diff_array : {α : Type} → {n : Nat} → Array α n → Array α n`
///
/// Difference array: for an array a, define d where d[0] = a[0],
/// d[i] = a[i] - a[i-1]. Supports O(1) range update, O(n) reconstruction.
#[allow(dead_code)]
pub fn arr_ext_diff_array_ty() -> Expr {
    implicit_pi(
        "α",
        type1(),
        implicit_pi(
            "n",
            nat_ty(),
            default_pi(
                "arr",
                array_of(Expr::BVar(1), Expr::BVar(0)),
                array_of(Expr::BVar(2), Expr::BVar(1)),
            ),
        ),
    )
}
/// `Array.sparse_table : {α : Type} → {n : Nat} → [Ord α] → Array α n → Type`
///
/// Sparse table for range-minimum queries (RMQ): precomputes answers for all
/// power-of-2 length intervals in O(n log n) time, enabling O(1) RMQ.
#[allow(dead_code)]
pub fn arr_ext_sparse_table_ty() -> Expr {
    implicit_pi(
        "α",
        type1(),
        implicit_pi(
            "n",
            nat_ty(),
            inst_pi(
                "inst",
                ord_of(Expr::BVar(1)),
                default_pi("arr", array_of(Expr::BVar(2), Expr::BVar(1)), type1()),
            ),
        ),
    )
}
/// `Array.rmq_correct : ∀ {α n}, [Ord α] → Array α n → Prop`
///
/// Range minimum query correctness: the sparse table answers RMQ queries
/// correctly, returning the minimum element in the queried range.
#[allow(dead_code)]
pub fn arr_ext_rmq_correct_ty() -> Expr {
    implicit_pi(
        "α",
        type1(),
        implicit_pi(
            "n",
            nat_ty(),
            inst_pi(
                "inst",
                ord_of(Expr::BVar(1)),
                default_pi("arr", array_of(Expr::BVar(2), Expr::BVar(1)), prop()),
            ),
        ),
    )
}
/// `Array2D : Type → Nat → Nat → Type`
///
/// A two-dimensional array of `rows × cols` elements of type `α`.
#[allow(dead_code)]
pub fn arr_ext_array2d_ty() -> Expr {
    default_pi(
        "α",
        type1(),
        default_pi("rows", nat_ty(), default_pi("cols", nat_ty(), type1())),
    )
}
/// `Array2D.transpose : {α : Type} → {r c : Nat} → Array2D α r c → Array2D α c r`
///
/// Matrix transposition: swaps rows and columns. `transpose (transpose M) = M`.
#[allow(dead_code)]
pub fn arr_ext_transpose_ty() -> Expr {
    implicit_pi(
        "α",
        type1(),
        implicit_pi(
            "r",
            nat_ty(),
            implicit_pi(
                "c",
                nat_ty(),
                default_pi(
                    "m",
                    app3(
                        Expr::Const(Name::str("Array2D"), vec![]),
                        Expr::BVar(2),
                        Expr::BVar(1),
                        Expr::BVar(0),
                    ),
                    type1(),
                ),
            ),
        ),
    )
}
/// `Array.rotate_left : {α : Type} → {n : Nat} → Nat → Array α n → Array α n`
///
/// Left rotation by k positions: moves the first k elements to the end.
/// `rotate_left 0 a = a`, `rotate_left n a = a`.
#[allow(dead_code)]
pub fn arr_ext_rotate_left_ty() -> Expr {
    implicit_pi(
        "α",
        type1(),
        implicit_pi(
            "n",
            nat_ty(),
            default_pi(
                "k",
                nat_ty(),
                default_pi(
                    "arr",
                    array_of(Expr::BVar(2), Expr::BVar(1)),
                    array_of(Expr::BVar(3), Expr::BVar(2)),
                ),
            ),
        ),
    )
}
/// `Array.rotate_right : {α : Type} → {n : Nat} → Nat → Array α n → Array α n`
///
/// Right rotation by k positions: moves the last k elements to the front.
/// Right and left rotations are inverses.
#[allow(dead_code)]
pub fn arr_ext_rotate_right_ty() -> Expr {
    implicit_pi(
        "α",
        type1(),
        implicit_pi(
            "n",
            nat_ty(),
            default_pi(
                "k",
                nat_ty(),
                default_pi(
                    "arr",
                    array_of(Expr::BVar(2), Expr::BVar(1)),
                    array_of(Expr::BVar(3), Expr::BVar(2)),
                ),
            ),
        ),
    )
}
/// `Array.rotate_inverse : ∀ {α n k}, Array α n → Prop`
///
/// Rotation inverse law: `rotate_right k (rotate_left k a) = a` and
/// `rotate_left k (rotate_right k a) = a`.
#[allow(dead_code)]
pub fn arr_ext_rotate_inverse_ty() -> Expr {
    implicit_pi(
        "α",
        type1(),
        implicit_pi(
            "n",
            nat_ty(),
            implicit_pi(
                "k",
                nat_ty(),
                default_pi("arr", array_of(Expr::BVar(2), Expr::BVar(1)), prop()),
            ),
        ),
    )
}
/// `Array.suffix_array : {n : Nat} → Array Nat n → Array Nat n`
///
/// Suffix array construction: given a string (represented as an array of
/// character codes), compute the lexicographically sorted array of suffix start
/// indices. Used in O(n log n) string matching algorithms.
#[allow(dead_code)]
pub fn arr_ext_suffix_array_ty() -> Expr {
    implicit_pi(
        "n",
        nat_ty(),
        default_pi(
            "str",
            array_of(nat_ty(), Expr::BVar(0)),
            array_of(nat_ty(), Expr::BVar(1)),
        ),
    )
}
/// `Array.suffix_array_correct : ∀ {n}, Array Nat n → Prop`
///
/// Suffix array correctness: the suffix array SA is a permutation of [0..n)
/// such that the suffixes str[SA[i]..] are in strictly increasing lexicographic order.
#[allow(dead_code)]
pub fn arr_ext_suffix_array_correct_ty() -> Expr {
    implicit_pi(
        "n",
        nat_ty(),
        default_pi("str", array_of(nat_ty(), Expr::BVar(0)), prop()),
    )
}
/// `Array.convolution : {n : Nat} → Array Nat n → Array Nat n → Array Nat n`
///
/// Discrete convolution of two length-n sequences. The FFT-based algorithm
/// computes this in O(n log n) time versus the naive O(n²).
#[allow(dead_code)]
pub fn arr_ext_convolution_ty() -> Expr {
    implicit_pi(
        "n",
        nat_ty(),
        default_pi(
            "a",
            array_of(nat_ty(), Expr::BVar(0)),
            default_pi(
                "b",
                array_of(nat_ty(), Expr::BVar(1)),
                array_of(nat_ty(), Expr::BVar(2)),
            ),
        ),
    )
}
/// `Array.fft_conv_correct : ∀ {n}, Array Nat n → Array Nat n → Prop`
///
/// FFT convolution correctness: the fast convolution result equals the
/// naive pointwise convolution result.
#[allow(dead_code)]
pub fn arr_ext_fft_conv_correct_ty() -> Expr {
    implicit_pi(
        "n",
        nat_ty(),
        default_pi(
            "a",
            array_of(nat_ty(), Expr::BVar(0)),
            default_pi("b", array_of(nat_ty(), Expr::BVar(1)), prop()),
        ),
    )
}
/// `PersistentArray : Type → Nat → Type`
///
/// A persistent (purely functional) array: all versions of the array are
/// accessible after updates. Implemented via balanced binary trees or path
/// copying, supporting O(log n) access and update.
#[allow(dead_code)]
pub fn arr_ext_persistent_array_ty() -> Expr {
    default_pi("α", type1(), default_pi("n", nat_ty(), type1()))
}
/// `PersistentArray.get : {α : Type} → {n : Nat} → PersistentArray α n → Fin n → α`
///
/// Persistent array lookup: O(log n) access without mutation.
#[allow(dead_code)]
pub fn arr_ext_persistent_get_ty() -> Expr {
    implicit_pi(
        "α",
        type1(),
        implicit_pi(
            "n",
            nat_ty(),
            default_pi(
                "pa",
                app2(
                    Expr::Const(Name::str("PersistentArray"), vec![]),
                    Expr::BVar(1),
                    Expr::BVar(0),
                ),
                default_pi("i", fin_of(Expr::BVar(1)), Expr::BVar(3)),
            ),
        ),
    )
}
/// `PersistentArray.set : {α : Type} → {n : Nat} → PersistentArray α n → Fin n → α → PersistentArray α n`
///
/// Persistent array update: returns a new version with updated element at
/// index i; original version is unaffected (path copying).
#[allow(dead_code)]
pub fn arr_ext_persistent_set_ty() -> Expr {
    implicit_pi(
        "α",
        type1(),
        implicit_pi(
            "n",
            nat_ty(),
            default_pi(
                "pa",
                app2(
                    Expr::Const(Name::str("PersistentArray"), vec![]),
                    Expr::BVar(1),
                    Expr::BVar(0),
                ),
                default_pi(
                    "i",
                    fin_of(Expr::BVar(1)),
                    default_pi(
                        "v",
                        Expr::BVar(3),
                        app2(
                            Expr::Const(Name::str("PersistentArray"), vec![]),
                            Expr::BVar(4),
                            Expr::BVar(3),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `Array.chunksOf : {α : Type} → {n : Nat} → Nat → Array α n → List (List α)`
///
/// Split an array into consecutive chunks of size k. The last chunk may be
/// smaller if n is not divisible by k. Used in SIMD/vectorization.
#[allow(dead_code)]
pub fn arr_ext_chunks_of_ty() -> Expr {
    implicit_pi(
        "α",
        type1(),
        implicit_pi(
            "n",
            nat_ty(),
            default_pi(
                "k",
                nat_ty(),
                default_pi(
                    "arr",
                    array_of(Expr::BVar(2), Expr::BVar(1)),
                    list_of(list_of(Expr::BVar(3))),
                ),
            ),
        ),
    )
}
/// `Array.chunks_cover : ∀ {α n k}, Array α n → Prop`
///
/// Coverage property of chunking: concatenating all chunks yields the original
/// array. `concat (chunksOf k a) = toList a`.
#[allow(dead_code)]
pub fn arr_ext_chunks_cover_ty() -> Expr {
    implicit_pi(
        "α",
        type1(),
        implicit_pi(
            "n",
            nat_ty(),
            implicit_pi(
                "k",
                nat_ty(),
                default_pi("arr", array_of(Expr::BVar(2), Expr::BVar(1)), prop()),
            ),
        ),
    )
}
/// `Array.par_map : {α β : Type} → {n : Nat} → (α → β) → Array α n → Array β n`
///
/// Parallel array map: conceptually applies f to each element concurrently.
/// The result is identical to sequential map; only execution is parallel.
#[allow(dead_code)]
pub fn arr_ext_par_map_ty() -> Expr {
    implicit_pi(
        "α",
        type1(),
        implicit_pi(
            "β",
            type1(),
            implicit_pi(
                "n",
                nat_ty(),
                default_pi(
                    "f",
                    arrow(Expr::BVar(2), Expr::BVar(1)),
                    default_pi(
                        "arr",
                        array_of(Expr::BVar(3), Expr::BVar(1)),
                        array_of(Expr::BVar(3), Expr::BVar(2)),
                    ),
                ),
            ),
        ),
    )
}
/// `Array.par_map_correct : ∀ {α β n}, (α → β) → Array α n → Prop`
///
/// Correctness of parallel map: `par_map f a = map f a`.
/// The parallel execution produces identical results to sequential mapping.
#[allow(dead_code)]
pub fn arr_ext_par_map_correct_ty() -> Expr {
    implicit_pi(
        "α",
        type1(),
        implicit_pi(
            "β",
            type1(),
            implicit_pi(
                "n",
                nat_ty(),
                default_pi(
                    "f",
                    arrow(Expr::BVar(2), Expr::BVar(1)),
                    default_pi("arr", array_of(Expr::BVar(3), Expr::BVar(1)), prop()),
                ),
            ),
        ),
    )
}
/// `Array.par_reduce : {α : Type} → {n : Nat} → (α → α → α) → α → Array α n → α`
///
/// Parallel reduction: applies an associative binary operator to reduce an
/// array to a single value in O(log n) parallel time.
#[allow(dead_code)]
pub fn arr_ext_par_reduce_ty() -> Expr {
    implicit_pi(
        "α",
        type1(),
        implicit_pi(
            "n",
            nat_ty(),
            default_pi(
                "op",
                arrow(Expr::BVar(1), arrow(Expr::BVar(2), Expr::BVar(2))),
                default_pi(
                    "init",
                    Expr::BVar(2),
                    default_pi("arr", array_of(Expr::BVar(3), Expr::BVar(2)), Expr::BVar(4)),
                ),
            ),
        ),
    )
}
/// `Array.scan_left : {α β : Type} → {n : Nat} → (β → α → β) → β → Array α n → Array β (n+1)`
///
/// Left scan (prefix scan): produces an array of prefix reductions.
/// `scan_left f z [a0, a1, ..., an-1] = [z, f z a0, f (f z a0) a1, ...]`.
#[allow(dead_code)]
pub fn arr_ext_scan_left_ty() -> Expr {
    implicit_pi(
        "α",
        type1(),
        implicit_pi(
            "β",
            type1(),
            implicit_pi(
                "n",
                nat_ty(),
                default_pi(
                    "f",
                    arrow(Expr::BVar(1), arrow(Expr::BVar(3), Expr::BVar(2))),
                    default_pi(
                        "init",
                        Expr::BVar(2),
                        default_pi(
                            "arr",
                            array_of(Expr::BVar(4), Expr::BVar(2)),
                            array_of(Expr::BVar(4), nat_succ(Expr::BVar(3))),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `Array.map_fuse : ∀ {α β γ n}, (β → γ) → (α → β) → Array α n → Prop`
///
/// Map fusion law: two consecutive maps can be fused into one pass.
/// `map g (map f a) = map (g ∘ f) a`. This enables loop fusion optimization.
#[allow(dead_code)]
pub fn arr_ext_map_fuse_ty() -> Expr {
    implicit_pi(
        "α",
        type1(),
        implicit_pi(
            "β",
            type1(),
            implicit_pi(
                "γ",
                type1(),
                implicit_pi(
                    "n",
                    nat_ty(),
                    default_pi(
                        "g",
                        arrow(Expr::BVar(2), Expr::BVar(1)),
                        default_pi(
                            "f",
                            arrow(Expr::BVar(4), Expr::BVar(3)),
                            default_pi("arr", array_of(Expr::BVar(5), Expr::BVar(2)), prop()),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `Array.filter_map_fuse : ∀ {α β n}, (α → Option β) → Array α n → Prop`
///
/// filterMap fusion: applying filterMap (a combined filter and map) is equivalent
/// to first mapping then filtering, but in a single pass.
#[allow(dead_code)]
pub fn arr_ext_filter_map_fuse_ty() -> Expr {
    implicit_pi(
        "α",
        type1(),
        implicit_pi(
            "β",
            type1(),
            implicit_pi(
                "n",
                nat_ty(),
                default_pi(
                    "f",
                    arrow(Expr::BVar(2), option_of(Expr::BVar(1))),
                    default_pi("arr", array_of(Expr::BVar(3), Expr::BVar(1)), prop()),
                ),
            ),
        ),
    )
}
/// `Array.fold_map_fuse : ∀ {α β γ n}, (β → γ → γ) → γ → (α → β) → Array α n → Prop`
///
/// Fold-map fusion: `foldl g z (map f a) = foldl (λacc x, g acc (f x)) z a`.
/// Avoids constructing the intermediate mapped array.
#[allow(dead_code)]
pub fn arr_ext_fold_map_fuse_ty() -> Expr {
    implicit_pi(
        "α",
        type1(),
        implicit_pi(
            "β",
            type1(),
            implicit_pi(
                "γ",
                type1(),
                implicit_pi(
                    "n",
                    nat_ty(),
                    default_pi(
                        "g",
                        arrow(Expr::BVar(2), arrow(Expr::BVar(2), Expr::BVar(2))),
                        default_pi(
                            "z",
                            Expr::BVar(3),
                            default_pi(
                                "f",
                                arrow(Expr::BVar(5), Expr::BVar(4)),
                                default_pi("arr", array_of(Expr::BVar(6), Expr::BVar(3)), prop()),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// Register all extended array axioms into the kernel environment.
///
/// This adds axioms for: functor/monad laws, sort stability, reverse involution,
/// append laws, slices, prefix sums, 2D arrays, rotation, suffix arrays,
/// convolution, persistent arrays, chunking, parallel map, and fusion laws.
pub fn register_array_extended(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        ("Array.map_id", arr_ext_map_id_ty()),
        ("Array.map_comp", arr_ext_map_comp_ty()),
        ("Array.pure_map_size", arr_ext_pure_map_size_ty()),
        ("Array.bind_assoc", arr_ext_bind_assoc_ty()),
        ("Array.mergesort", arr_ext_mergesort_ty()),
        ("Array.sort_stable", arr_ext_sort_stable_ty()),
        ("Array.sort_perm", arr_ext_sort_perm_ty()),
        ("Array.sort_sorted", arr_ext_sort_sorted_ty()),
        ("Array.qsort_average_case", arr_ext_qsort_avg_ty()),
        ("Array.reverse_involution", arr_ext_reverse_involution_ty()),
        ("Array.reverse_size", arr_ext_reverse_size_ty()),
        ("Array.append_assoc", arr_ext_append_assoc_ty()),
        ("Array.append_empty_left", arr_ext_append_empty_left_ty()),
        ("Array.append_empty_right", arr_ext_append_empty_right_ty()),
        ("Array.append_size", arr_ext_append_size_ty()),
        ("Array.slice", arr_ext_slice_ty()),
        ("Array.prefix_sum", arr_ext_prefix_sum_ty()),
        ("Array.prefix_sum_correct", arr_ext_prefix_sum_correct_ty()),
        ("Array.diff_array", arr_ext_diff_array_ty()),
        ("Array.sparse_table", arr_ext_sparse_table_ty()),
        ("Array.rmq_correct", arr_ext_rmq_correct_ty()),
        ("Array2D", arr_ext_array2d_ty()),
        ("Array2D.transpose", arr_ext_transpose_ty()),
        ("Array.rotate_left", arr_ext_rotate_left_ty()),
        ("Array.rotate_right", arr_ext_rotate_right_ty()),
        ("Array.rotate_inverse", arr_ext_rotate_inverse_ty()),
        ("Array.suffix_array", arr_ext_suffix_array_ty()),
        (
            "Array.suffix_array_correct",
            arr_ext_suffix_array_correct_ty(),
        ),
        ("Array.convolution", arr_ext_convolution_ty()),
        ("Array.fft_conv_correct", arr_ext_fft_conv_correct_ty()),
        ("PersistentArray", arr_ext_persistent_array_ty()),
        ("PersistentArray.get", arr_ext_persistent_get_ty()),
        ("PersistentArray.set", arr_ext_persistent_set_ty()),
        ("Array.chunksOf", arr_ext_chunks_of_ty()),
        ("Array.chunks_cover", arr_ext_chunks_cover_ty()),
        ("Array.par_map", arr_ext_par_map_ty()),
        ("Array.par_map_correct", arr_ext_par_map_correct_ty()),
        ("Array.par_reduce", arr_ext_par_reduce_ty()),
        ("Array.scan_left", arr_ext_scan_left_ty()),
        ("Array.map_fuse", arr_ext_map_fuse_ty()),
        ("Array.filter_map_fuse", arr_ext_filter_map_fuse_ty()),
        ("Array.fold_map_fuse", arr_ext_fold_map_fuse_ty()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    Ok(())
}
/// Merge sort implementation on a mutable slice.
///
/// Returns a sorted copy of the input using stable merge sort.
///
/// # Example
/// ```
/// # use oxilean_std::array::merge_sort;
/// let sorted = merge_sort(&[3, 1, 4, 1, 5, 9, 2, 6]);
/// assert_eq!(sorted, vec![1, 1, 2, 3, 4, 5, 6, 9]);
/// ```
#[allow(dead_code)]
pub fn merge_sort(data: &[i64]) -> Vec<i64> {
    if data.len() <= 1 {
        return data.to_vec();
    }
    let mid = data.len() / 2;
    let left = merge_sort(&data[..mid]);
    let right = merge_sort(&data[mid..]);
    let mut result = Vec::with_capacity(data.len());
    let (mut i, mut j) = (0, 0);
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result.push(left[i]);
            i += 1;
        } else {
            result.push(right[j]);
            j += 1;
        }
    }
    result.extend_from_slice(&left[i..]);
    result.extend_from_slice(&right[j..]);
    result
}
