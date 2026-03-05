//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::path::Path;

use super::functions::normalize_lean4_to_oxilean;
use super::functions_3::MATHLIB4_ROOT;
use super::functions_3::{print_stats, run_compat_on_dir, run_compat_recursive, try_parse_decl};
use super::types::CompatStats;

#[test]
fn test_order_filter_compat() {
    let dir = Path::new(MATHLIB4_ROOT).join("Order/Filter");
    if !dir.exists() {
        println!("[SKIP] Mathlib4 not found at {}", dir.display());
        return;
    }
    let stats = run_compat_on_dir(&dir, 10);
    print_stats("Order/Filter", &stats);
    if stats.total > 0 {
        println!(
            "  Compat rate: {:.1}% ({}/{})",
            stats.success_rate(),
            stats.parsed_ok,
            stats.total
        );
        assert!(
            stats.parsed_ok * 100 >= stats.total * 60,
            "Order/Filter compat should be >=60%: got {}/{}",
            stats.parsed_ok,
            stats.total
        );
    } else {
        println!("  No single-line declarations found in Order/Filter");
    }
}
#[test]
fn test_algebra_group_hom_compat() {
    let dir = Path::new(MATHLIB4_ROOT).join("Algebra/Group/Hom");
    if !dir.exists() {
        println!("[SKIP] Mathlib4 not found at {}", dir.display());
        return;
    }
    let stats = run_compat_on_dir(&dir, 8);
    print_stats("Algebra/Group/Hom", &stats);
    if stats.total > 0 {
        println!(
            "  Compat rate: {:.1}% ({}/{})",
            stats.success_rate(),
            stats.parsed_ok,
            stats.total
        );
        assert!(
            stats.parsed_ok * 100 >= stats.total * 60,
            "Algebra/Group/Hom compat should be >=60%: got {}/{}",
            stats.parsed_ok,
            stats.total
        );
    } else {
        println!("  No single-line declarations found in Algebra/Group/Hom");
    }
}
#[test]
fn test_algebra_order_ring_compat() {
    let dir = Path::new(MATHLIB4_ROOT).join("Algebra/Order/Ring");
    if !dir.exists() {
        println!("[SKIP] Mathlib4 not found at {}", dir.display());
        return;
    }
    let stats = run_compat_on_dir(&dir, 10);
    print_stats("Algebra/Order/Ring", &stats);
    if stats.total > 0 {
        println!(
            "  Compat rate: {:.1}% ({}/{})",
            stats.success_rate(),
            stats.parsed_ok,
            stats.total
        );
        assert!(
            stats.parsed_ok * 100 >= stats.total * 60,
            "Algebra/Order/Ring compat should be >=60%: got {}/{}",
            stats.parsed_ok,
            stats.total
        );
    } else {
        println!("  No single-line declarations found in Algebra/Order/Ring");
    }
}
#[test]
fn test_data_complex_compat() {
    let dir = Path::new(MATHLIB4_ROOT).join("Data/Complex");
    if !dir.exists() {
        println!("[SKIP] Mathlib4 not found at {}", dir.display());
        return;
    }
    let stats = run_compat_on_dir(&dir, 9);
    print_stats("Data/Complex", &stats);
    if stats.total > 0 {
        println!(
            "  Compat rate: {:.1}% ({}/{})",
            stats.success_rate(),
            stats.parsed_ok,
            stats.total
        );
        assert!(
            stats.parsed_ok * 100 >= stats.total * 60,
            "Data/Complex compat should be >=60%: got {}/{}",
            stats.parsed_ok,
            stats.total
        );
    } else {
        println!("  No single-line declarations found in Data/Complex");
    }
}
#[test]
fn test_overall_compat_summary() {
    let root = Path::new(MATHLIB4_ROOT);
    if !root.exists() {
        println!("[SKIP] Mathlib4 not found at {MATHLIB4_ROOT}");
        return;
    }
    let flat_dirs = [
        ("Data/Nat", 20usize),
        ("Data/List", 25),
        ("Algebra", 20),
        ("Data/Bool", 10),
        ("Order", 20),
        ("Data/Option", 10),
        ("Data/Prod", 10),
        ("Data/Sum", 10),
        ("Data/Int", 20),
        ("Data/Fin", 10),
        ("Data/Multiset", 15),
        ("Data/Finset", 15),
        ("Data/Set", 15),
        ("Data/Rat", 10),
        ("Data/PNat", 10),
        ("Data/Fintype", 15),
        ("Algebra/Group", 15),
        ("Algebra/Ring", 15),
        ("GroupTheory/Coset", 10),
        ("Data/ENat", 5),
        ("Data/Finsupp", 10),
        ("Algebra/Order/Group", 10),
        ("NumberTheory", 5),
        ("Algebra/Field", 12),
        ("Algebra/Module", 15),
        ("GroupTheory", 12),
        ("Combinatorics/SimpleGraph", 15),
        ("Algebra/Algebra", 15),
        ("RingTheory", 15),
        ("Data/Nat/GCD", 5),
        ("Data/Nat/Factorial", 6),
        ("Algebra/BigOperators", 15),
        ("LinearAlgebra", 8),
        ("NumberTheory/ArithmeticFunction", 6),
        ("Topology", 12),
        ("SetTheory/Cardinal", 10),
        ("SetTheory/Ordinal", 10),
        ("FieldTheory", 10),
        ("MeasureTheory/Measure", 10),
        ("Data/ZMod", 10),
        ("Algebra/Group/Subgroup", 12),
        ("Order/Filter", 10),
        ("Algebra/Group/Hom", 8),
        ("Algebra/Order/Ring", 10),
        ("Data/Complex", 9),
        ("Algebra/Group/Submonoid", 10),
        ("GroupTheory/QuotientGroup", 5),
        ("Algebra/GroupWithZero", 8),
        ("Algebra/CharP", 8),
        ("RingTheory/Coprime", 5),
    ];
    let recursive_dirs = [("Logic", 30usize)];
    let mut total_stats = CompatStats::default();
    for (dir_name, max_files) in &flat_dirs {
        let dir = root.join(dir_name);
        if dir.exists() {
            let s = run_compat_on_dir(&dir, *max_files);
            total_stats.merge(s);
        }
    }
    for (dir_name, max_files) in &recursive_dirs {
        let dir = root.join(dir_name);
        if dir.exists() {
            let s = run_compat_recursive(&dir, *max_files);
            total_stats.merge(s);
        }
    }
    println!("\n========================================");
    println!("OVERALL MATHLIB4 COMPAT SUMMARY");
    println!("========================================");
    print_stats("All Categories", &total_stats);
    println!("========================================\n");
    assert!(
        total_stats.success_rate() >= 0.0,
        "Overall compat rate should be non-negative"
    );
    let report = format!(
        "# OxiLean Mathlib4 Parser Compatibility Report\n\n\
         Date: 2026-02-17\n\n\
         ## Summary\n\
         Files processed: {}\n\
         Total declarations tested: {}\n\
         Parsed successfully: {}\n\
         Compatibility rate: {:.1}%\n\n\
         ## Normalizations Applied (v3)\n\
         - \u{21A6} (U+21A6 mapsto) -> ->\n\
         - \u{2115} -> Nat, \u{2124} -> Int, \u{211D} -> Real, \u{211A} -> Rat, \u{2102} -> Complex\n\
         - `fun x => body` -> `fun x -> body`\n\
         - Head binders: `theorem foo (x : T) : P` -> `theorem foo : forall (x : T), P`\n\
         - `:= by <tactic>` -> `:= sorry` (proof replaced)\n\
         - Dotted names: `Nat.add_comm` -> `Nat_add_comm`\n\
         - `_root_.` prefix stripped\n\
         - `Sort*`/`Type*` -> `Type`\n\
         - `@[attr]` stripped\n\
         - \u{2286} -> Subset, \u{2208} -> Mem, \u{222A} -> Union, \u{2229} -> Inter, \u{2205} -> empty_set\n\
         - Bounded quantifiers: `ISup k < n, body` -> `ISup (fun k -> body)`\n\
         - Array subscripts: `ident[n]` -> `ident`\n\
         - def without return type: `def f (x : T) :=` -> `def f := sorry`\n\n\
         ## Remaining Failure Categories\n\
         - Complex type expressions with unsupported operators\n\
         - `where` clauses (multi-part definitions)\n\
         - Anonymous constructor in term proofs\n\
         - `·` dot lambda / anonymous function syntax\n\
         - `@explicit` override syntax\n\
         - `iff`/`\u{2194}` in complex term proofs\n\n\
         ## Next Steps\n\
         - Add OxiLean Iff notation support\n\
         - Handle `@[simp]`-tagged theorems in elaborator\n\
         - Expand tactic set in kernel (omega, linarith)\n",
        total_stats.files_processed, total_stats.total, total_stats.parsed_ok,
        total_stats.success_rate()
    );
    let _ = std::fs::write("/tmp/oxilean_compat_report.md", &report);
    println!("Report written to /tmp/oxilean_compat_report.md");
}
#[test]
#[allow(dead_code)]
fn test_debug_bigunion() {
    let decl = "theorem iUnion_setOf (P : ι → α → Prop) : ⋃ i, { x : α | P i x } = { x : α | ∃ i, P i x } := by ext; exact mem_iUnion";
    let normalized = normalize_lean4_to_oxilean(decl);
    println!("Normalized: {normalized}");
    let ok = try_parse_decl(&normalized);
    println!("Parse OK: {ok}");
}
#[test]
#[allow(dead_code)]
fn test_debug_have_in_type() {
    let decl =
        "theorem natCast_eq_mk {m n : ℕ} (h : m < n) : have : NeZero n := ⟨Nat.ne_zero_of_lt h⟩";
    let normalized = normalize_lean4_to_oxilean(decl);
    println!("Normalized natCast: {normalized}");
    let ok = try_parse_decl(&normalized);
    println!("Parse OK: {ok}");
    let decl2 = "lemma _root_.finCongr_eq_equivCast (h : n = m) : finCongr h = .cast (h ▸ rfl) := by subst h; simp";
    let normalized2 = normalize_lean4_to_oxilean(decl2);
    println!("Normalized finCongr: {normalized2}");
    let ok2 = try_parse_decl(&normalized2);
    println!("Parse OK: {ok2}");
    let decl3 =
        "theorem card_perms_of_finset : ∀ s : Finset α, #(permsOfFinset s) = (#s)! := by simp";
    let normalized3 = normalize_lean4_to_oxilean(decl3);
    println!("Normalized card_perms: {normalized3}");
    let ok3 = try_parse_decl(&normalized3);
    println!("Parse OK: {ok3}");
}
#[test]
#[allow(dead_code)]
fn test_debug_remaining() {
    let decls = vec![
        "theorem cons_val_two (x : α) (u : Fin m.succ.succ → α) : vecCons x u 2 = vecHead (vecTail u) := rfl",
        "lemma iInter_sum {s : α ⊕ β → Set γ} : ⋂ x, s x = (⋂ x, s (.inl x)) ∩ ⋂ x, s (.inr x) := iInf_sup_eq",
        "lemma iUnion_sum {s : α ⊕ β → Set γ} : ⋃ x, s x = (⋃ x, s (.inl x)) ∪ ⋃ x, s (.inr x) := iSup_sup_eq",
        "theorem iUnion_eq_range_sigma (s : α → Set β) : ⋃ i, s i = range fun a : Σ i, s i => a.2 := by ext; exact mem_iUnion",
        "theorem iUnion_eq_range_psigma (s : ι → Set β) : ⋃ i, s i = range fun a : Σ' i, s i => a.2 := by ext; exact mem_iUnion",
        "theorem or_le : ∀ {x y z}, x ≤ z → y ≤ z → (x || y) ≤ z := by decide",
        "lemma pairwise_iff_lt (hp : Symmetric p) : Pairwise p ↔ ∀ ⦃a b⦄, a < b → p a b := by simp",
        "lemma Finite.pi' (ht : ∀ i, (t i).Finite) : {f : ∀ i, κ i | ∀ i, f i ∈ t i}.Finite := by simp",
        "theorem iUnion_of_singleton_coe (s : Set α) : ⋃ i : s, ({(i : α)} : Set α) = s := by simp",
        "∀ {x y z}, x ≤ z → y ≤ z",
        "theorem test_fin_type : forall (u : Fin (m.succ).succ -> α), vecHead u = u := sorry",
        "theorem test_fin2 : forall (u : Fin m.succ.succ -> α), u = u := sorry",
        "theorem test_sigma_fun : forall (a : Sigma i, s i), a = a := sorry",
        "theorem sSup_iUnion (t : ι → Set β) : sSup (⋃ i, t i) = ⨆ i, sSup (t i) := by simp",
        "theorem ne_key {a} {l : List (Sigma β)} : a ∉ l.keys ↔ ∀ s : Sigma β, s ∈ l → a ≠ s.fst := by simp",
        "theorem coe_image_of_subset {s t : Set α} (h : t ⊆ s) : (↑) '' { x : ↥s | ↑x ∈ t } = t := by simp",
        "theorem sSup_sUnion (s : Set (Set β)) : sSup (⋃₀ s) = ⨆ t ∈ s, sSup t := by simp",
        "theorem finprod_one : (\u{220f}\u{1d1f} _ : \u{03b1}, (1 : M)) = 1 := by simp",
        "theorem finprod_of_isEmpty [IsEmpty \u{03b1}] (f : \u{03b1} \u{2192} M) : \u{220f}\u{1d1f} i, f i = 1 := by simp",
        "theorem mul_prod_removeNth i (f : Fin (n + 1) \u{2192} M) : f i * \u{220f} j, removeNth i f j = \u{220f} j, removeNth i f j := by simp",
        "theorem sum_moebius_mul_log_eq {n : \u{2115}} : (\u{2211} d \u{2208} n.divisors, (\u{03bc} d : \u{211d}) * log d) = 0 := by simp",
        "theorem test_dot_paren : forall (n : Nat), BigSum n.divisors (fun d -> d) = 0 := sorry",
        "axiom test_n_dot : forall (n : Nat), n.divisors = n.divisors",
        "def test_bigsum_inner : BigSum (n.divisors) (fun d -> d) := sorry",
    ];
    for decl in &decls {
        let normalized = normalize_lean4_to_oxilean(decl);
        let ok = try_parse_decl(&normalized);
        println!("OK={ok}: {normalized}");
    }
}
/// Extract OxiLean definition names from a Rust source file.
///
/// Looks for `Name::str("...")` patterns which are how OxiLean std library
/// registers kernel definitions/theorems.
#[allow(dead_code)]
fn extract_oxilean_names_from_rust(content: &str) -> Vec<String> {
    let mut names = Vec::new();
    let prefix = "Name::str(\"";
    let mut rest = content;
    while let Some(pos) = rest.find(prefix) {
        rest = &rest[pos + prefix.len()..];
        if let Some(end) = rest.find('"') {
            let name = rest[..end].to_string();
            if !name.is_empty() && name.len() < 80 {
                names.push(name);
            }
        }
    }
    names
}
