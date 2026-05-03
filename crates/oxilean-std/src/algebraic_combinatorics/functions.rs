//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
#![allow(clippy::items_after_test_module)]

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    CharacterTable, CrystalGraph, CrystalNode, DyckPath, EhrhartPolynomial, HVector,
    LittlewoodRichardsonRule, NonCrossingPartition, ParkingFunction, Polytope, RSKCorrespondence,
    SchurFunction, SemistandardYoungTableau, StandardYoungTableau, StanleyDecomposition,
    SymFunctionHopf, SymmetricFunction, TamariLattice, TensorProductCrystal, YoungDiagram,
};

pub fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
pub fn app2(f: Expr, a: Expr, b: Expr) -> Expr {
    app(app(f, a), b)
}
pub fn app3(f: Expr, a: Expr, b: Expr, c: Expr) -> Expr {
    app(app2(f, a, b), c)
}
pub fn cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}
pub fn prop() -> Expr {
    Expr::Sort(Level::zero())
}
pub fn type0() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
pub fn pi(bi: BinderInfo, name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(bi, Name::str(name), Box::new(dom), Box::new(body))
}
pub fn arrow(a: Expr, b: Expr) -> Expr {
    pi(BinderInfo::Default, "_", a, b)
}
pub fn bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
pub fn nat_ty() -> Expr {
    cst("Nat")
}
pub fn int_ty() -> Expr {
    cst("Int")
}
pub fn real_ty() -> Expr {
    cst("Real")
}
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn list_ty(elem: Expr) -> Expr {
    app(cst("List"), elem)
}
/// `YoungDiagram : Type` — partition λ = (λ₁ ≥ λ₂ ≥ … ≥ λ_k > 0).
pub fn young_diagram_ty() -> Expr {
    type0()
}
/// `StandardYoungTableau : YoungDiagram → Type`
/// Filling of a Young diagram with integers 1..n, rows and columns strictly
/// increasing.
pub fn standard_young_tableau_ty() -> Expr {
    arrow(cst("YoungDiagram"), type0())
}
/// `SemistandardYoungTableau : YoungDiagram → Nat → Type`
/// Filling with alphabet {1..n}: rows weakly increasing, columns strictly.
pub fn semistandard_young_tableau_ty() -> Expr {
    arrow(cst("YoungDiagram"), arrow(nat_ty(), type0()))
}
/// `ConjugatePartition : YoungDiagram → YoungDiagram` — transpose of λ.
pub fn conjugate_partition_ty() -> Expr {
    arrow(cst("YoungDiagram"), cst("YoungDiagram"))
}
/// `HookLength : YoungDiagram → Nat → Nat → Nat`
/// Hook length at cell (i, j) = λ_i − j + λ'_j − i + 1.
pub fn hook_length_ty() -> Expr {
    arrow(
        cst("YoungDiagram"),
        arrow(nat_ty(), arrow(nat_ty(), nat_ty())),
    )
}
/// `NumSYT : YoungDiagram → Nat`
/// Number of standard Young tableaux of shape λ (hook-length formula: n!/∏h).
pub fn num_syt_ty() -> Expr {
    arrow(cst("YoungDiagram"), nat_ty())
}
/// `RSKCorrespondence : List Nat → YoungDiagram × YoungDiagram`
/// Robinson-Schensted-Knuth bijection from permutations to pairs of SYT.
pub fn rsk_correspondence_ty() -> Expr {
    arrow(
        list_ty(nat_ty()),
        app(app(cst("Prod"), cst("YoungDiagram")), cst("YoungDiagram")),
    )
}
/// Hook-Length Formula: `num_syt λ = n! / ∏_{(i,j)∈λ} hook(i,j)`.
pub fn hook_length_formula_ty() -> Expr {
    pi(BinderInfo::Default, "λ", cst("YoungDiagram"), prop())
}
/// `SymmetricFunction : Type` — element of the ring Λ of symmetric functions.
pub fn symmetric_function_ty() -> Expr {
    type0()
}
/// `SchurFunction : YoungDiagram → SymmetricFunction` — Schur polynomial s_λ.
pub fn schur_function_ty() -> Expr {
    arrow(cst("YoungDiagram"), cst("SymmetricFunction"))
}
/// `ElementarySymmetric : Nat → SymmetricFunction` — e_k = ∑_{i₁<…<i_k} x_{i₁}…x_{i_k}.
pub fn elementary_symmetric_ty() -> Expr {
    arrow(nat_ty(), cst("SymmetricFunction"))
}
/// `HomogeneousSymmetric : Nat → SymmetricFunction` — h_k = ∑_{i₁≤…≤i_k} x_{i₁}…x_{i_k}.
pub fn homogeneous_symmetric_ty() -> Expr {
    arrow(nat_ty(), cst("SymmetricFunction"))
}
/// `PowerSymmetric : Nat → SymmetricFunction` — p_k = ∑ x_i^k.
pub fn power_symmetric_ty() -> Expr {
    arrow(nat_ty(), cst("SymmetricFunction"))
}
/// `JackPolynomial : YoungDiagram → Real → SymmetricFunction`
/// Jack polynomial P_λ(α) parametrised by α > 0.
pub fn jack_polynomial_ty() -> Expr {
    arrow(
        cst("YoungDiagram"),
        arrow(real_ty(), cst("SymmetricFunction")),
    )
}
/// `LRCoefficient : YoungDiagram → YoungDiagram → YoungDiagram → Nat`
/// Littlewood-Richardson coefficient c^λ_{μν}.
pub fn lr_coefficient_ty() -> Expr {
    arrow(
        cst("YoungDiagram"),
        arrow(cst("YoungDiagram"), arrow(cst("YoungDiagram"), nat_ty())),
    )
}
/// `PlethysticSubstitution : SymmetricFunction → SymmetricFunction → SymmetricFunction`
/// f\[g\] plethystic substitution.
pub fn plethystic_substitution_ty() -> Expr {
    arrow(
        cst("SymmetricFunction"),
        arrow(cst("SymmetricFunction"), cst("SymmetricFunction")),
    )
}
/// Pieri Rule: multiplying s_λ by h_k gives ∑ s_μ over all μ obtained by
/// adding k boxes to λ with no two in the same column.
pub fn pieri_rule_ty() -> Expr {
    prop()
}
/// Dual basis: the Schur functions {s_λ} and {s_λ} are dual under the Hall
/// inner product.
pub fn schur_dual_basis_ty() -> Expr {
    prop()
}
/// `IrreducibleRepSn : YoungDiagram → Type`
/// Specht module S^λ — the irreducible complex S_n-representation indexed by λ.
pub fn irreducible_rep_sn_ty() -> Expr {
    arrow(cst("YoungDiagram"), type0())
}
/// `CharacterValue : YoungDiagram → YoungDiagram → Int`
/// χ^λ(μ) — character of S^λ evaluated on the conjugacy class indexed by μ.
pub fn character_value_ty() -> Expr {
    arrow(cst("YoungDiagram"), arrow(cst("YoungDiagram"), int_ty()))
}
/// `CharacterTable : Nat → Type`
/// Full character table of S_n: rows are irreps (partitions of n), columns
/// are conjugacy classes.
pub fn character_table_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `FrobeniusCharacterMap : SymmetricFunction → SymmetricFunction`
/// Characteristic map ch: Z\[S_n\] → Λ_n sending S^λ to s_λ.
pub fn frobenius_character_map_ty() -> Expr {
    arrow(cst("SymmetricFunction"), cst("SymmetricFunction"))
}
/// Murnaghan-Nakayama Rule: recursive formula for χ^λ(μ) by removing rim
/// hooks from λ.
pub fn murnaghan_nakayama_rule_ty() -> Expr {
    prop()
}
/// Frobenius Formula: the Frobenius characteristic map sends the irreducible
/// character χ^λ to the Schur function s_λ.
pub fn frobenius_formula_ty() -> Expr {
    pi(BinderInfo::Default, "λ", cst("YoungDiagram"), prop())
}
/// `CrystalGraph : Type` — directed graph coloured by simple roots with
/// weight function.
pub fn crystal_graph_ty() -> Expr {
    type0()
}
/// `CrystalNode : CrystalGraph → Type`
/// Node in a crystal graph carrying weight ∈ weight lattice.
pub fn crystal_node_ty() -> Expr {
    arrow(cst("CrystalGraph"), type0())
}
/// `TensorProductCrystal : CrystalGraph → CrystalGraph → CrystalGraph`
/// Tensor product B₁ ⊗ B₂ with Kashiwara tensor product rule.
pub fn tensor_product_crystal_ty() -> Expr {
    arrow(
        cst("CrystalGraph"),
        arrow(cst("CrystalGraph"), cst("CrystalGraph")),
    )
}
/// `HighestWeightCrystal : List Nat → CrystalGraph`
/// B(λ) — the unique (up to iso) highest-weight crystal for dominant weight λ.
pub fn highest_weight_crystal_ty() -> Expr {
    arrow(list_ty(nat_ty()), cst("CrystalGraph"))
}
/// `CrystalOperatorE : Nat → CrystalGraph → CrystalGraph → Prop`
/// e_i operator on a crystal graph node.
pub fn crystal_operator_e_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(cst("CrystalGraph"), arrow(cst("CrystalGraph"), prop())),
    )
}
/// `CrystalOperatorF : Nat → CrystalGraph → CrystalGraph → Prop`
/// f_i operator on a crystal graph node.
pub fn crystal_operator_f_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(cst("CrystalGraph"), arrow(cst("CrystalGraph"), prop())),
    )
}
/// Crystal axiom: e_i and f_i are partial inverses on the crystal.
pub fn crystal_inverse_axiom_ty() -> Expr {
    prop()
}
/// `ParkingFunction : Nat → Type`
/// A sequence (a₁,…,a_n) with 1 ≤ a_i ≤ n and a_i ≤ i after sorting.
pub fn parking_function_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `DyckPath : Nat → Type`
/// Lattice path of length 2n with n up-steps (1,1) and n down-steps (1,-1)
/// never going below the x-axis.
pub fn dyck_path_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `NonCrossingPartition : Nat → Type`
/// Set partition of {1,…,n} where no two blocks "cross".
pub fn non_crossing_partition_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `TamariLattice : Nat → Type`
/// Partial order on full binary trees with n+1 leaves (or equivalently Dyck
/// paths of semilength n).
pub fn tamari_lattice_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `CountParkingFunctions : Nat → Nat`
/// Number of parking functions of length n equals (n+1)^{n-1}.
pub fn count_parking_functions_ty() -> Expr {
    arrow(nat_ty(), nat_ty())
}
/// `CatalanNumber : Nat → Nat` — C_n = Binomial(2n,n)/(n+1).
pub fn catalan_number_ty() -> Expr {
    arrow(nat_ty(), nat_ty())
}
/// `QAnalogCatalan : Nat → Nat → Nat`
/// q-analogue of the Catalan number C_n(q) = Gaussian-binomial(2n,n)_q / [n+1]_q.
pub fn q_analog_catalan_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), nat_ty()))
}
/// Parking function count theorem: |PF_n| = (n+1)^{n-1}.
pub fn parking_function_count_ty() -> Expr {
    pi(BinderInfo::Default, "n", nat_ty(), prop())
}
/// Catalan bijections: DyckPath n, NonCrossingPartition n, ParkingFunction n,
/// and TamariLattice n are all in bijection (counted by C_n).
pub fn catalan_bijection_ty() -> Expr {
    prop()
}
/// `Polytope : Nat → Type`
/// Convex hull of finitely many vertices in ℝ^n.
pub fn polytope_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `EhrhartPolynomial : Polytope n → Nat → Nat` (where n is implicit)
/// L(P, t) = |tP ∩ ℤ^n| — Ehrhart polynomial evaluated at integer t.
pub fn ehrhart_polynomial_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), nat_ty()))
}
/// `HVector : Nat → Type`
/// h-vector of a simplicial complex of dimension d: (h₀, h₁, …, h_d).
pub fn h_vector_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `StanleyDecomposition : Type`
/// Decomposition of a simplicial complex into Boolean intervals (Stanley
/// decomposition / Cohen-Macaulay criterion).
pub fn stanley_decomposition_ty() -> Expr {
    type0()
}
/// `NormalizedVolume : Nat → Nat`
/// Normalized volume of a lattice polytope: n! · Vol(P).
pub fn normalized_volume_ty() -> Expr {
    arrow(nat_ty(), nat_ty())
}
/// `FVector : Nat → List Nat`
/// f-vector (f₀, f₁, …, f_{d-1}): f_i = number of i-dimensional faces.
pub fn f_vector_ty() -> Expr {
    arrow(nat_ty(), list_ty(nat_ty()))
}
/// Ehrhart Reciprocity: L(P, -t) = (-1)^n L(int P, t) for a lattice polytope.
pub fn ehrhart_reciprocity_ty() -> Expr {
    prop()
}
/// Stanley non-negativity: the h-vector of a Cohen-Macaulay simplicial complex
/// has non-negative entries.
pub fn stanley_nonnegativity_ty() -> Expr {
    prop()
}
/// Populate an `Environment` with all algebraic-combinatorics axioms.
pub fn build_algebraic_combinatorics_env(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        ("YoungDiagram", young_diagram_ty()),
        ("StandardYoungTableau", standard_young_tableau_ty()),
        ("SemistandardYoungTableau", semistandard_young_tableau_ty()),
        ("ConjugatePartition", conjugate_partition_ty()),
        ("HookLength", hook_length_ty()),
        ("NumSYT", num_syt_ty()),
        ("RSKCorrespondence", rsk_correspondence_ty()),
        ("HookLengthFormula", hook_length_formula_ty()),
        ("SymmetricFunction", symmetric_function_ty()),
        ("SchurFunction", schur_function_ty()),
        ("ElementarySymmetric", elementary_symmetric_ty()),
        ("HomogeneousSymmetric", homogeneous_symmetric_ty()),
        ("PowerSymmetric", power_symmetric_ty()),
        ("JackPolynomial", jack_polynomial_ty()),
        ("LRCoefficient", lr_coefficient_ty()),
        ("PlethysticSubstitution", plethystic_substitution_ty()),
        ("PieriRule", pieri_rule_ty()),
        ("SchurDualBasis", schur_dual_basis_ty()),
        ("IrreducibleRepSn", irreducible_rep_sn_ty()),
        ("CharacterValue", character_value_ty()),
        ("CharacterTable", character_table_ty()),
        ("FrobeniusCharacterMap", frobenius_character_map_ty()),
        ("MurnaghanNakayamaRule", murnaghan_nakayama_rule_ty()),
        ("FrobeniusFormula", frobenius_formula_ty()),
        ("CrystalGraph", crystal_graph_ty()),
        ("CrystalNode", crystal_node_ty()),
        ("TensorProductCrystal", tensor_product_crystal_ty()),
        ("HighestWeightCrystal", highest_weight_crystal_ty()),
        ("CrystalOperatorE", crystal_operator_e_ty()),
        ("CrystalOperatorF", crystal_operator_f_ty()),
        ("CrystalInverseAxiom", crystal_inverse_axiom_ty()),
        ("ParkingFunction", parking_function_ty()),
        ("DyckPath", dyck_path_ty()),
        ("NonCrossingPartition", non_crossing_partition_ty()),
        ("TamariLattice", tamari_lattice_ty()),
        ("CountParkingFunctions", count_parking_functions_ty()),
        ("CatalanNumber", catalan_number_ty()),
        ("QAnalogCatalan", q_analog_catalan_ty()),
        ("ParkingFunctionCount", parking_function_count_ty()),
        ("CatalanBijection", catalan_bijection_ty()),
        ("Polytope", polytope_ty()),
        ("EhrhartPolynomial", ehrhart_polynomial_ty()),
        ("HVector", h_vector_ty()),
        ("StanleyDecomposition", stanley_decomposition_ty()),
        ("NormalizedVolume", normalized_volume_ty()),
        ("FVector", f_vector_ty()),
        ("EhrhartReciprocity", ehrhart_reciprocity_ty()),
        ("StanleyNonnegativity", stanley_nonnegativity_ty()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .map_err(|e| format!("Failed to add '{}': {:?}", name, e))?;
    }
    Ok(())
}
/// Count semistandard Young tableaux of given shape and content via backtracking.
pub fn count_ssyt(shape: &YoungDiagram, content: &[usize]) -> usize {
    let n = shape.size();
    if n == 0 {
        return 1;
    }
    let num_rows = shape.parts.len();
    let max_col = *shape.parts.first().unwrap_or(&0);
    let mut cells: Vec<(usize, usize)> = Vec::with_capacity(n);
    for col in 0..max_col {
        for row in 0..num_rows {
            if col < shape.parts[row] {
                cells.push((row, col));
            }
        }
    }
    let mut grid = vec![vec![0usize; max_col + 1]; num_rows];
    let mut remaining = content.to_vec();
    count_ssyt_backtrack(&cells, 0, &mut grid, &mut remaining, shape)
}
pub fn count_ssyt_backtrack(
    cells: &[(usize, usize)],
    idx: usize,
    grid: &mut Vec<Vec<usize>>,
    remaining: &mut Vec<usize>,
    shape: &YoungDiagram,
) -> usize {
    if idx == cells.len() {
        return 1;
    }
    let (r, c) = cells[idx];
    let mut total = 0;
    for letter in 1..=remaining.len() {
        if remaining[letter - 1] == 0 {
            continue;
        }
        if c > 0 && grid[r][c - 1] > letter {
            continue;
        }
        if r > 0 && c < shape.parts[r - 1] && grid[r - 1][c] >= letter {
            continue;
        }
        grid[r][c] = letter;
        remaining[letter - 1] -= 1;
        total += count_ssyt_backtrack(cells, idx + 1, grid, remaining, shape);
        remaining[letter - 1] += 1;
        grid[r][c] = 0;
    }
    total
}
/// Count LR tableaux of skew shape λ/μ with content ν.
pub fn count_lr_tableaux(lambda: &YoungDiagram, mu: &YoungDiagram, nu: &YoungDiagram) -> usize {
    let mut skew_cells: Vec<(usize, usize)> = Vec::new();
    for i in 0..lambda.parts.len() {
        let start = if i < mu.parts.len() { mu.parts[i] } else { 0 };
        let end = lambda.parts[i];
        for j in start..end {
            skew_cells.push((i, j));
        }
    }
    if skew_cells.len() != nu.size() {
        return 0;
    }
    let max_col = lambda.parts.first().copied().unwrap_or(0);
    let num_rows = lambda.parts.len();
    let mut grid = vec![vec![0usize; max_col + 1]; num_rows];
    let mut content = nu.parts.clone();
    lr_backtrack(&skew_cells, 0, &mut grid, &mut content, lambda, mu)
}
pub fn lr_backtrack(
    cells: &[(usize, usize)],
    idx: usize,
    grid: &mut Vec<Vec<usize>>,
    remaining: &mut Vec<usize>,
    lambda: &YoungDiagram,
    mu: &YoungDiagram,
) -> usize {
    if idx == cells.len() {
        let rev_word = reading_word_reverse(grid, lambda, mu);
        if is_lattice_word(&rev_word, remaining.len()) {
            return 1;
        }
        return 0;
    }
    let (r, c) = cells[idx];
    let mut total = 0;
    for letter in 1..=remaining.len() {
        if remaining[letter - 1] == 0 {
            continue;
        }
        let left_val = if c > 0 { grid[r][c - 1] } else { 0 };
        if left_val > letter {
            continue;
        }
        let above_val = if r > 0 { grid[r - 1][c] } else { 0 };
        if above_val >= letter && above_val != 0 {
            continue;
        }
        grid[r][c] = letter;
        remaining[letter - 1] -= 1;
        total += lr_backtrack(cells, idx + 1, grid, remaining, lambda, mu);
        remaining[letter - 1] += 1;
        grid[r][c] = 0;
    }
    total
}
/// Read the grid in reverse row-major order (bottom-right to top-left),
/// skipping cells belonging to μ (grid value 0).
pub fn reading_word_reverse(
    grid: &[Vec<usize>],
    lambda: &YoungDiagram,
    mu: &YoungDiagram,
) -> Vec<usize> {
    let mut word = Vec::new();
    for i in (0..lambda.parts.len()).rev() {
        let start = if i < mu.parts.len() { mu.parts[i] } else { 0 };
        for j in (start..lambda.parts[i]).rev() {
            if grid[i][j] > 0 {
                word.push(grid[i][j]);
            }
        }
    }
    word
}
/// Check that a word is a lattice word: every prefix has at least as many i's
/// as (i+1)'s for every i.
pub fn is_lattice_word(word: &[usize], max_letter: usize) -> bool {
    let mut counts = vec![0i64; max_letter + 1];
    for &c in word {
        if c == 0 || c > max_letter {
            return false;
        }
        counts[c] += 1;
        for i in 1..max_letter {
            if counts[i] < counts[i + 1] {
                return false;
            }
        }
    }
    true
}
/// Generate all partitions of n in lexicographic order.
pub fn partitions_of(n: usize) -> Vec<Vec<usize>> {
    let mut result = Vec::new();
    let mut current = Vec::new();
    gen_partitions(n, n, &mut current, &mut result);
    result
}
pub fn gen_partitions(
    remaining: usize,
    max_part: usize,
    current: &mut Vec<usize>,
    result: &mut Vec<Vec<usize>>,
) {
    if remaining == 0 {
        result.push(current.clone());
        return;
    }
    let upper = remaining.min(max_part);
    for part in (1..=upper).rev() {
        current.push(part);
        gen_partitions(remaining - part, part, current, result);
        current.pop();
    }
}
/// Compute χ^λ(μ) via the Murnaghan-Nakayama rule.
/// λ is the irrep partition, μ is the cycle type of the conjugacy class.
pub fn murnaghan_nakayama(lambda: &[usize], mu: &[usize]) -> i64 {
    if mu.is_empty() {
        let yd = YoungDiagram::new(lambda.to_vec());
        return yd.num_syt() as i64;
    }
    let k = mu[0];
    let mut total = 0i64;
    let yd = YoungDiagram::new(lambda.to_vec());
    for (mu_hook, sign) in rim_hooks(&yd, k) {
        let sub = murnaghan_nakayama(&mu_hook.parts, &mu[1..]);
        total += sign * sub;
    }
    total
}
/// Enumerate all ways to remove a rim hook of size k from a Young diagram,
/// returning (resulting_diagram, sign) pairs.  sign = (-1)^{height-1}.
pub fn rim_hooks(yd: &YoungDiagram, k: usize) -> Vec<(YoungDiagram, i64)> {
    let mut results = Vec::new();
    let n = yd.parts.len();
    for start in 0..n {
        let mut hook_size = 0usize;
        for end in start..n {
            let lambda_end_plus1 = if end + 1 < n { yd.parts[end + 1] } else { 0 };
            hook_size += yd.parts[end] - lambda_end_plus1;
            if hook_size == k {
                let mut new_parts = yd.parts.clone();
                for i in start..=end {
                    new_parts[i] = lambda_end_plus1;
                }
                let result_yd = YoungDiagram::new(new_parts.clone());
                let height = end - start + 1;
                let sign = if height % 2 == 0 { -1i64 } else { 1i64 };
                results.push((result_yd, sign));
                break;
            }
        }
    }
    results
}
/// Weight in a root system (vector of integers).
pub type Weight = Vec<i32>;
/// Generate all SSYT of given shape with alphabet {1..alphabet}.
pub fn gen_all_ssyt(shape: &YoungDiagram, alphabet: usize) -> Vec<Vec<Vec<usize>>> {
    let num_rows = shape.parts.len();
    if num_rows == 0 || shape.size() == 0 {
        return vec![vec![]];
    }
    let max_col = *shape
        .parts
        .first()
        .expect("parts is non-empty: checked by num_rows > 0 guard");
    let mut cells: Vec<(usize, usize)> = Vec::new();
    for r in 0..num_rows {
        for c in 0..shape.parts[r] {
            cells.push((r, c));
        }
    }
    let mut results = Vec::new();
    let mut grid = vec![vec![0usize; max_col]; num_rows];
    gen_ssyt_backtrack(&cells, 0, &mut grid, alphabet, shape, &mut results);
    results
}
pub fn gen_ssyt_backtrack(
    cells: &[(usize, usize)],
    idx: usize,
    grid: &mut Vec<Vec<usize>>,
    alphabet: usize,
    shape: &YoungDiagram,
    results: &mut Vec<Vec<Vec<usize>>>,
) {
    if idx == cells.len() {
        results.push(grid.clone());
        return;
    }
    let (r, c) = cells[idx];
    let min_val = if c > 0 { grid[r][c - 1] } else { 1 };
    let above_val = if r > 0 && c < shape.parts[r - 1] {
        grid[r - 1][c] + 1
    } else {
        1
    };
    let start = min_val.max(above_val);
    for v in start..=alphabet {
        grid[r][c] = v;
        gen_ssyt_backtrack(cells, idx + 1, grid, alphabet, shape, results);
    }
    grid[r][c] = 0;
}
/// Apply crystal operator f_i (lower crystal operator) to an SSYT.
/// Uses the signature rule: scan reading word for i, i+1 pattern.
pub fn crystal_f(tab: &[Vec<usize>], i: usize, shape: &YoungDiagram) -> Option<Vec<Vec<usize>>> {
    let word: Vec<usize> = tab.iter().flat_map(|r| r.iter().copied()).collect();
    let mut sig = Vec::new();
    for &c in &word {
        if c == i {
            sig.push(0u8);
        } else if c == i + 1 {
            if sig.last() == Some(&0) {
                sig.pop();
            } else {
                sig.push(1u8);
            }
        }
    }
    let mut depth = 0i32;
    let _last_plus_pos: Option<usize> = None;
    let _sig_idx = 0usize;
    let mut i_count = 0i32;
    let mut ip1_count = 0i32;
    let mut last_i_pos: Option<usize> = None;
    for (pos, &c) in word.iter().enumerate() {
        if c == i {
            i_count += 1;
            depth += 1;
            if depth > 0 {
                last_i_pos = Some(pos);
            }
        } else if c == i + 1 {
            ip1_count += 1;
            depth -= 1;
        }
    }
    let _ = i_count;
    let _ = ip1_count;
    if depth <= 0 || last_i_pos.is_none() {
        return None;
    }
    let change_pos = last_i_pos.expect("last_i_pos is Some: checked by is_none guard above");
    let mut new_word = word.clone();
    new_word[change_pos] = i + 1;
    let mut new_tab = tab.to_vec();
    let mut pos = 0;
    'outer: for r in 0..new_tab.len() {
        for c in 0..new_tab[r].len() {
            if pos == change_pos {
                new_tab[r][c] = new_word[change_pos];
                break 'outer;
            }
            pos += 1;
        }
    }
    let ssyt = SemistandardYoungTableau {
        rows: new_tab.clone(),
        shape: shape.clone(),
        alphabet_size: i + 1,
    };
    if ssyt.is_valid() {
        Some(new_tab)
    } else {
        None
    }
}
pub fn gen_dyck(steps: &mut Vec<bool>, ups_left: i32, downs_left: i32, result: &mut Vec<DyckPath>) {
    if ups_left == 0 && downs_left == 0 {
        let n = steps.len() / 2;
        result.push(DyckPath {
            steps: steps.clone(),
            semilength: n,
        });
        return;
    }
    let cur_height: i32 = steps.iter().map(|&s| if s { 1 } else { -1 }).sum();
    if ups_left > 0 {
        steps.push(true);
        gen_dyck(steps, ups_left - 1, downs_left, result);
        steps.pop();
    }
    if downs_left > ups_left && cur_height > 0 {
        steps.push(false);
        gen_dyck(steps, ups_left, downs_left - 1, result);
        steps.pop();
    }
}
/// Integer determinant of a d×d matrix (in-place LU via row operations).
pub fn det(mat: &mut Vec<Vec<i64>>) -> i64 {
    let n = mat.len();
    if n == 0 {
        return 1;
    }
    if n == 1 {
        return mat[0][0];
    }
    if n == 2 {
        return mat[0][0] * mat[1][1] - mat[0][1] * mat[1][0];
    }
    let mut result = 0i64;
    for col in 0..n {
        let mut minor: Vec<Vec<i64>> = (1..n)
            .map(|i| (0..n).filter(|&j| j != col).map(|j| mat[i][j]).collect())
            .collect();
        let sign = if col % 2 == 0 { 1i64 } else { -1i64 };
        result += sign * mat[0][col] * det(&mut minor);
    }
    result
}
/// Binomial coefficient C(n, k).
pub fn choose(n: usize, k: usize) -> usize {
    if k > n {
        return 0;
    }
    if k == 0 || k == n {
        return 1;
    }
    let k = k.min(n - k);
    let mut result = 1usize;
    for i in 0..k {
        result = result * (n - i) / (i + 1);
    }
    result
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_young_diagram_conjugate() {
        let yd = YoungDiagram::new(vec![3, 2, 1]);
        let conj = yd.conjugate_partition();
        assert_eq!(conj.parts, vec![3, 2, 1]);
        let yd2 = YoungDiagram::new(vec![4, 2]);
        let conj2 = yd2.conjugate_partition();
        assert_eq!(conj2.parts, vec![2, 2, 1, 1]);
    }
    #[test]
    fn test_hook_length() {
        let yd = YoungDiagram::new(vec![3, 1]);
        assert_eq!(yd.hook_length(0, 0), Some(4));
        assert_eq!(yd.hook_length(0, 1), Some(2));
        assert_eq!(yd.hook_length(0, 2), Some(1));
        assert_eq!(yd.hook_length(1, 0), Some(1));
        assert_eq!(yd.hook_length(2, 0), None);
    }
    #[test]
    fn test_num_syt() {
        let yd = YoungDiagram::new(vec![2, 1]);
        assert_eq!(yd.num_syt(), 2);
        let yd2 = YoungDiagram::new(vec![3]);
        assert_eq!(yd2.num_syt(), 1);
        let yd3 = YoungDiagram::new(vec![2, 2]);
        assert_eq!(yd3.num_syt(), 2);
    }
    #[test]
    fn test_rsk() {
        let rsk = RSKCorrespondence::from_word(&[1, 2, 3]);
        assert_eq!(rsk.shape.parts, vec![3]);
        let rsk2 = RSKCorrespondence::from_word(&[2, 1, 3]);
        assert_eq!(rsk2.shape.size(), 3);
    }
    #[test]
    fn test_syt_validity() {
        let shape = YoungDiagram::new(vec![2, 1]);
        let syt = StandardYoungTableau {
            rows: vec![vec![1, 2], vec![3]],
            shape: shape.clone(),
        };
        assert!(syt.is_valid());
        let bad = StandardYoungTableau {
            rows: vec![vec![1, 3], vec![2]],
            shape,
        };
        assert!(bad.is_valid());
    }
    #[test]
    fn test_schur_kostka() {
        let sf = SchurFunction::new(YoungDiagram::new(vec![2, 1]));
        assert_eq!(sf.kostka_number(&[2, 1]), 1);
        assert_eq!(sf.kostka_number(&[1, 1, 1]), 2);
    }
    #[test]
    fn test_lr_coefficient() {
        let lr = LittlewoodRichardsonRule::new(
            YoungDiagram::new(vec![3]),
            YoungDiagram::new(vec![2]),
            YoungDiagram::new(vec![1]),
        );
        assert_eq!(lr.coefficient(), 1);
    }
    #[test]
    fn test_partitions_of() {
        let p3 = partitions_of(3);
        assert_eq!(p3.len(), 3);
        let p4 = partitions_of(4);
        assert_eq!(p4.len(), 5);
    }
    #[test]
    fn test_character_table_s3() {
        let ct = CharacterTable::build(3);
        assert_eq!(ct.n, 3);
        assert_eq!(ct.partitions.len(), 3);
    }
    #[test]
    fn test_parking_function() {
        assert!(ParkingFunction::is_valid(&[1, 1, 2]));
        assert!(ParkingFunction::is_valid(&[1, 2, 1]));
        assert!(!ParkingFunction::is_valid(&[2, 2, 2]));
        assert_eq!(ParkingFunction::count(3), 16);
        assert_eq!(ParkingFunction::count(1), 1);
    }
    #[test]
    fn test_dyck_path_catalan() {
        assert_eq!(DyckPath::catalan(0), 1);
        assert_eq!(DyckPath::catalan(1), 1);
        assert_eq!(DyckPath::catalan(2), 2);
        assert_eq!(DyckPath::catalan(3), 5);
        assert_eq!(DyckPath::catalan(4), 14);
        let paths = DyckPath::all_dyck_paths(3);
        assert_eq!(paths.len(), 5);
    }
    #[test]
    fn test_non_crossing_partition() {
        assert_eq!(NonCrossingPartition::count(3), 5);
        assert_eq!(NonCrossingPartition::count(4), 14);
    }
    #[test]
    fn test_polytope_normalized_volume() {
        let p = Polytope::new(2, vec![vec![0, 0], vec![1, 0], vec![0, 1]]);
        assert_eq!(p.normalized_volume(), 1);
    }
    #[test]
    fn test_ehrhart() {
        let p = Polytope::new(2, vec![vec![0, 0], vec![1, 0], vec![0, 1]]);
        let ep = EhrhartPolynomial::new(p);
        assert_eq!(ep.evaluate(1), 3);
        assert_eq!(ep.evaluate(2), 6);
        assert_eq!(ep.evaluate(3), 10);
    }
    #[test]
    fn test_build_algebraic_combinatorics_env() {
        let mut env = oxilean_kernel::Environment::new();
        let result = build_algebraic_combinatorics_env(&mut env);
        assert!(
            result.is_ok(),
            "build_algebraic_combinatorics_env failed: {:?}",
            result.err()
        );
    }
}
/// `MonomialSymmetric : YoungDiagram → SymmetricFunction`
/// Monomial symmetric function m_λ = sum of all distinct monomials of shape λ.
pub fn alg_comb_ext_monomial_sym_ty() -> Expr {
    arrow(cst("YoungDiagram"), cst("SymmetricFunction"))
}
/// `PowerSumBasis : Nat → SymmetricFunction`
/// Power sum symmetric function p_k as a basis element.
pub fn alg_comb_ext_power_sum_basis_ty() -> Expr {
    arrow(nat_ty(), cst("SymmetricFunction"))
}
/// `OmegaInvolution : SymmetricFunction → SymmetricFunction`
/// The omega involution ω : Λ → Λ sending e_k to h_k and s_λ to s_{λ'}.
pub fn alg_comb_ext_omega_involution_ty() -> Expr {
    arrow(cst("SymmetricFunction"), cst("SymmetricFunction"))
}
/// `HallInnerProduct : SymmetricFunction → SymmetricFunction → Real`
/// Hall inner product ⟨f, g⟩ making the Schur functions orthonormal.
pub fn alg_comb_ext_hall_inner_product_ty() -> Expr {
    arrow(
        cst("SymmetricFunction"),
        arrow(cst("SymmetricFunction"), real_ty()),
    )
}
/// `SymmetricFunctionRing : Type`
/// The graded ring Λ of symmetric functions over ℤ.
pub fn alg_comb_ext_sym_function_ring_ty() -> Expr {
    type0()
}
/// `PlethysmOfSchur : YoungDiagram → YoungDiagram → SymmetricFunction`
/// Plethysm s_λ\[s_μ\] of two Schur functions.
pub fn alg_comb_ext_plethysm_of_schur_ty() -> Expr {
    arrow(
        cst("YoungDiagram"),
        arrow(cst("YoungDiagram"), cst("SymmetricFunction")),
    )
}
/// `LambdaRingOperation : SymmetricFunction → Nat → SymmetricFunction`
/// λ-ring operations λ^k on a symmetric function.
pub fn alg_comb_ext_lambda_ring_op_ty() -> Expr {
    arrow(
        cst("SymmetricFunction"),
        arrow(nat_ty(), cst("SymmetricFunction")),
    )
}
/// `AdamsOperation : Nat → SymmetricFunction → SymmetricFunction`
/// Adams (power) operations ψ^k in the λ-ring structure.
pub fn alg_comb_ext_adams_operation_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(cst("SymmetricFunction"), cst("SymmetricFunction")),
    )
}
/// `RSKRecordingTableau : List Nat → SemistandardYoungTableau`
/// The recording tableau Q in the RSK correspondence.
pub fn alg_comb_ext_rsk_recording_tableau_ty() -> Expr {
    arrow(list_ty(nat_ty()), cst("SemistandardYoungTableau"))
}
/// `JeuDeTaquin : SemistandardYoungTableau → SemistandardYoungTableau`
/// Jeu de taquin: straightening of skew tableaux via forward and backward slides.
pub fn alg_comb_ext_jeu_de_taquin_ty() -> Expr {
    arrow(
        cst("SemistandardYoungTableau"),
        cst("SemistandardYoungTableau"),
    )
}
/// `EvacuationMap : StandardYoungTableau → StandardYoungTableau`
/// Schützenberger's evacuation involution on standard Young tableaux.
pub fn alg_comb_ext_evacuation_map_ty() -> Expr {
    arrow(cst("StandardYoungTableau"), cst("StandardYoungTableau"))
}
/// `PromotionMap : StandardYoungTableau → StandardYoungTableau`
/// Promotion operator on standard Young tableaux.
pub fn alg_comb_ext_promotion_map_ty() -> Expr {
    arrow(cst("StandardYoungTableau"), cst("StandardYoungTableau"))
}
/// `RowInsertionBumpPath : YoungDiagram → List Nat → List Nat`
/// The bump path produced during RSK row-insertion.
pub fn alg_comb_ext_row_insertion_bump_ty() -> Expr {
    arrow(
        cst("YoungDiagram"),
        arrow(list_ty(nat_ty()), list_ty(nat_ty())),
    )
}
/// `KazhdanLusztigPolynomial : Nat → Nat → SymmetricFunction`
/// Kazhdan-Lusztig polynomial P_{y,w}(q) for elements y ≤ w in a Coxeter group.
pub fn alg_comb_ext_kl_polynomial_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), cst("SymmetricFunction")))
}
/// `HallPolynomial : YoungDiagram → YoungDiagram → YoungDiagram → Nat → Nat`
/// Hall polynomial g^λ_{μν}(q) counting subgroups of type μ in type λ with quotient ν.
pub fn alg_comb_ext_hall_polynomial_ty() -> Expr {
    arrow(
        cst("YoungDiagram"),
        arrow(
            cst("YoungDiagram"),
            arrow(cst("YoungDiagram"), arrow(nat_ty(), nat_ty())),
        ),
    )
}
/// `InverseKLPolynomial : Nat → Nat → SymmetricFunction`
/// Inverse Kazhdan-Lusztig polynomial Q_{y,w}(q).
pub fn alg_comb_ext_inv_kl_polynomial_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), cst("SymmetricFunction")))
}
/// `KLBasisElement : Nat → SymmetricFunction`
/// The Kazhdan-Lusztig basis element C'_w in the Hecke algebra.
pub fn alg_comb_ext_kl_basis_element_ty() -> Expr {
    arrow(nat_ty(), cst("SymmetricFunction"))
}
/// `QuasisymmetricFunction : Type`
/// Element of QSym: the ring of quasisymmetric functions.
pub fn alg_comb_ext_quasisymmetric_ty() -> Expr {
    type0()
}
/// `FundamentalQSym : List Bool → QuasisymmetricFunction`
/// Gessel's fundamental quasisymmetric function F_α associated to a composition.
pub fn alg_comb_ext_fundamental_qsym_ty() -> Expr {
    arrow(list_ty(bool_ty()), cst("QuasisymmetricFunction"))
}
/// `MonomiialQSym : List Nat → QuasisymmetricFunction`
/// Monomial quasisymmetric function M_α for composition α.
pub fn alg_comb_ext_monomial_qsym_ty() -> Expr {
    arrow(list_ty(nat_ty()), cst("QuasisymmetricFunction"))
}
/// `NoncommSymmetricFunction : Type`
/// Element of NSym: the ring of noncommutative symmetric functions.
pub fn alg_comb_ext_noncomm_sym_ty() -> Expr {
    type0()
}
/// `RibbonSchurFunction : List Nat → NoncommSymmetricFunction`
/// Ribbon Schur function r_α indexed by composition α.
pub fn alg_comb_ext_ribbon_schur_ty() -> Expr {
    arrow(list_ty(nat_ty()), cst("NoncommSymmetricFunction"))
}
/// `DescentSet : StandardYoungTableau → List Nat`
/// Descent set Des(T) = {i : i+1 appears in a lower row than i in T}.
pub fn alg_comb_ext_descent_set_ty() -> Expr {
    arrow(cst("StandardYoungTableau"), list_ty(nat_ty()))
}
/// `CrystalMorphism : CrystalGraph → CrystalGraph → Type`
/// A morphism of crystal graphs preserving operators e_i, f_i, wt.
pub fn alg_comb_ext_crystal_morphism_ty() -> Expr {
    arrow(cst("CrystalGraph"), arrow(cst("CrystalGraph"), type0()))
}
/// `CrystalEmbedding : CrystalGraph → CrystalGraph → Type`
/// An injective crystal morphism (embedding).
pub fn alg_comb_ext_crystal_embedding_ty() -> Expr {
    arrow(cst("CrystalGraph"), arrow(cst("CrystalGraph"), type0()))
}
/// `NakajimaMonomial : List Int → CrystalGraph`
/// Nakajima monomial crystal model for a highest-weight module.
pub fn alg_comb_ext_nakajima_monomial_ty() -> Expr {
    arrow(list_ty(int_ty()), cst("CrystalGraph"))
}
/// `StringParametrization : CrystalNode → List Nat`
/// Lusztig string parametrization of a crystal node.
pub fn alg_comb_ext_string_param_ty() -> Expr {
    arrow(cst("CrystalNode"), list_ty(nat_ty()))
}
/// `FlagVariety : Nat → Type`
/// Complete flag variety Fl(n): the space of complete flags in ℂ^n.
pub fn alg_comb_ext_flag_variety_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `SchubertVariety : Nat → YoungDiagram → Type`
/// Schubert variety X_λ inside the Grassmannian Gr(k,n).
pub fn alg_comb_ext_schubert_variety_ty() -> Expr {
    arrow(nat_ty(), arrow(cst("YoungDiagram"), type0()))
}
/// `SchubertClass : YoungDiagram → SymmetricFunction`
/// Schubert class σ_λ as a cohomology class in H*(Gr(k,n)).
pub fn alg_comb_ext_schubert_class_ty() -> Expr {
    arrow(cst("YoungDiagram"), cst("SymmetricFunction"))
}
/// `GrassmannianIntersection : YoungDiagram → YoungDiagram → List YoungDiagram`
/// Intersection product σ_μ · σ_ν = ∑ c^λ_{μν} σ_λ in the Grassmannian.
pub fn alg_comb_ext_grassmannian_intersection_ty() -> Expr {
    arrow(
        cst("YoungDiagram"),
        arrow(cst("YoungDiagram"), list_ty(cst("YoungDiagram"))),
    )
}
/// `DoubleSchubertPolynomial : Nat → SymmetricFunction`
/// Double Schubert polynomial S_w(x;y) for a permutation w.
pub fn alg_comb_ext_double_schubert_poly_ty() -> Expr {
    arrow(nat_ty(), cst("SymmetricFunction"))
}
/// `CyclicSievingPhenomenon : Type → Nat → Nat → Prop`
/// The cyclic sieving phenomenon: |X^{C_n}| = f(ω^k) for all k.
pub fn alg_comb_ext_cyclic_sieving_ty() -> Expr {
    arrow(type0(), arrow(nat_ty(), arrow(nat_ty(), prop())))
}
/// `HomomesyStatistic : Type → SymmetricFunction → Prop`
/// A statistic f is homomesic under a cyclic action if its average is constant.
pub fn alg_comb_ext_homomesy_ty() -> Expr {
    arrow(type0(), arrow(cst("SymmetricFunction"), prop()))
}
/// `ToggleInvolution : Type → Type → Prop`
/// Toggle involution on a poset (Striker-Williams toggling).
pub fn alg_comb_ext_toggle_involution_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// `RowmotionOperator : Type → Type → Type`
/// Rowmotion operator on order ideals of a poset.
pub fn alg_comb_ext_rowmotion_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// `HopfAlgebra : Type → Type`
/// A Hopf algebra: algebra + coalgebra + antipode satisfying compatibility.
pub fn alg_comb_ext_hopf_algebra_ty() -> Expr {
    arrow(type0(), type0())
}
/// `SymFunctionHopf : HopfAlgebra SymmetricFunction`
/// The Hopf algebra structure on the ring of symmetric functions.
pub fn alg_comb_ext_sym_function_hopf_ty() -> Expr {
    app(cst("HopfAlgebra"), cst("SymmetricFunction"))
}
/// `CoproductMap : SymmetricFunction → List (SymmetricFunction × SymmetricFunction)`
/// The coproduct Δ : Λ → Λ ⊗ Λ, e.g. Δ(h_n) = ∑_{i+j=n} h_i ⊗ h_j.
pub fn alg_comb_ext_coproduct_map_ty() -> Expr {
    arrow(
        cst("SymmetricFunction"),
        list_ty(app(
            app(cst("Prod"), cst("SymmetricFunction")),
            cst("SymmetricFunction"),
        )),
    )
}
/// `AntipodeMap : SymmetricFunction → SymmetricFunction`
/// The antipode S : Λ → Λ in the Hopf algebra of symmetric functions.
pub fn alg_comb_ext_antipode_map_ty() -> Expr {
    arrow(cst("SymmetricFunction"), cst("SymmetricFunction"))
}
/// `TabWordReading : SemistandardYoungTableau → List Nat`
/// The column reading word of a semistandard Young tableau.
pub fn alg_comb_ext_tab_word_reading_ty() -> Expr {
    arrow(cst("SemistandardYoungTableau"), list_ty(nat_ty()))
}
/// `ContentVector : SemistandardYoungTableau → List Nat`
/// Content μ_i = number of cells with entry i in a tableau.
pub fn alg_comb_ext_content_vector_ty() -> Expr {
    arrow(cst("SemistandardYoungTableau"), list_ty(nat_ty()))
}
/// `KostkaNumber : YoungDiagram → YoungDiagram → Nat`
/// Kostka number K_{λμ} = number of SSYT of shape λ and content μ.
pub fn alg_comb_ext_kostka_number_ty() -> Expr {
    arrow(cst("YoungDiagram"), arrow(cst("YoungDiagram"), nat_ty()))
}
/// `InversionNumber : StandardYoungTableau → Nat`
/// Inversion number of a standard Young tableau (co-charge statistic).
pub fn alg_comb_ext_inversion_number_ty() -> Expr {
    arrow(cst("StandardYoungTableau"), nat_ty())
}
/// `MajorIndex : List Nat → Nat`
/// Major index of a permutation: sum of its descents.
pub fn alg_comb_ext_major_index_ty() -> Expr {
    arrow(list_ty(nat_ty()), nat_ty())
}
/// Register all extended algebraic combinatorics axioms into the environment.
pub fn register_algebraic_combinatorics_extended(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        ("MonomialSymmetric", alg_comb_ext_monomial_sym_ty()),
        ("PowerSumBasis", alg_comb_ext_power_sum_basis_ty()),
        ("OmegaInvolution", alg_comb_ext_omega_involution_ty()),
        ("HallInnerProduct", alg_comb_ext_hall_inner_product_ty()),
        ("SymmetricFunctionRing", alg_comb_ext_sym_function_ring_ty()),
        ("PlethysmOfSchur", alg_comb_ext_plethysm_of_schur_ty()),
        ("LambdaRingOperation", alg_comb_ext_lambda_ring_op_ty()),
        ("AdamsOperation", alg_comb_ext_adams_operation_ty()),
        (
            "RSKRecordingTableau",
            alg_comb_ext_rsk_recording_tableau_ty(),
        ),
        ("JeuDeTaquin", alg_comb_ext_jeu_de_taquin_ty()),
        ("EvacuationMap", alg_comb_ext_evacuation_map_ty()),
        ("PromotionMap", alg_comb_ext_promotion_map_ty()),
        ("RowInsertionBumpPath", alg_comb_ext_row_insertion_bump_ty()),
        ("KazhdanLusztigPolynomial", alg_comb_ext_kl_polynomial_ty()),
        ("HallPolynomial", alg_comb_ext_hall_polynomial_ty()),
        ("InverseKLPolynomial", alg_comb_ext_inv_kl_polynomial_ty()),
        ("KLBasisElement", alg_comb_ext_kl_basis_element_ty()),
        ("QuasisymmetricFunction", alg_comb_ext_quasisymmetric_ty()),
        ("FundamentalQSym", alg_comb_ext_fundamental_qsym_ty()),
        ("MonomialQSym", alg_comb_ext_monomial_qsym_ty()),
        ("NoncommSymmetricFunction", alg_comb_ext_noncomm_sym_ty()),
        ("RibbonSchurFunction", alg_comb_ext_ribbon_schur_ty()),
        ("DescentSet", alg_comb_ext_descent_set_ty()),
        ("CrystalMorphism", alg_comb_ext_crystal_morphism_ty()),
        ("CrystalEmbedding", alg_comb_ext_crystal_embedding_ty()),
        ("NakajimaMonomial", alg_comb_ext_nakajima_monomial_ty()),
        ("StringParametrization", alg_comb_ext_string_param_ty()),
        ("FlagVariety", alg_comb_ext_flag_variety_ty()),
        ("SchubertVariety", alg_comb_ext_schubert_variety_ty()),
        ("SchubertClass", alg_comb_ext_schubert_class_ty()),
        (
            "GrassmannianIntersection",
            alg_comb_ext_grassmannian_intersection_ty(),
        ),
        (
            "DoubleSchubertPolynomial",
            alg_comb_ext_double_schubert_poly_ty(),
        ),
        ("CyclicSievingPhenomenon", alg_comb_ext_cyclic_sieving_ty()),
        ("HomomesyStatistic", alg_comb_ext_homomesy_ty()),
        ("ToggleInvolution", alg_comb_ext_toggle_involution_ty()),
        ("RowmotionOperator", alg_comb_ext_rowmotion_ty()),
        ("HopfAlgebra", alg_comb_ext_hopf_algebra_ty()),
        ("SymFunctionHopf", alg_comb_ext_sym_function_hopf_ty()),
        ("CoproductMap", alg_comb_ext_coproduct_map_ty()),
        ("AntipodeMap", alg_comb_ext_antipode_map_ty()),
        ("TabWordReading", alg_comb_ext_tab_word_reading_ty()),
        ("ContentVector", alg_comb_ext_content_vector_ty()),
        ("KostkaNumber", alg_comb_ext_kostka_number_ty()),
        ("InversionNumber", alg_comb_ext_inversion_number_ty()),
        ("MajorIndex", alg_comb_ext_major_index_ty()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .map_err(|e| format!("Failed to add '{}': {:?}", name, e))?;
    }
    Ok(())
}
/// Convert bitmask to composition of given degree.
pub fn mask_to_composition(mask: u32, degree: usize) -> Vec<usize> {
    let mut parts = Vec::new();
    let mut current = 0usize;
    for i in 0..degree {
        current += 1;
        if i + 1 < degree && (mask >> i) & 1 == 1 {
            parts.push(current);
            current = 0;
        }
    }
    if current > 0 {
        parts.push(current);
    }
    parts
}
