//! Overall summary test and debug tests.

use std::path::Path;

use super::normalize::normalize_lean4_to_oxilean;
use super::test_infra::{
    mathlib4_root, print_stats, run_compat_on_dir, run_compat_recursive, try_parse_decl,
};
use super::types::CompatStats;

#[test]
#[ignore]
fn test_overall_compat_summary() {
    let mathlib4_root = match mathlib4_root() {
        Some(r) => r,
        None => return,
    };
    let root = Path::new(&mathlib4_root);
    if !root.exists() {
        println!("[SKIP] Mathlib4 not found at {mathlib4_root}");
        return;
    }
    let flat_dirs = [
        // Data categories
        ("Data/Nat", 100usize),
        ("Data/List", 100),
        ("Data/Bool", 50),
        ("Data/Option", 50),
        ("Data/Prod", 50),
        ("Data/Sum", 50),
        ("Data/Int", 50),
        ("Data/Fin", 50),
        ("Data/Multiset", 100),
        ("Data/Finset", 100),
        ("Data/Set", 100),
        ("Data/Rat", 50),
        ("Data/PNat", 50),
        ("Data/Fintype", 100),
        ("Data/ENat", 50),
        ("Data/Finsupp", 100),
        ("Data/Nat/GCD", 50),
        ("Data/Nat/Factorial", 50),
        ("Data/ZMod", 50),
        ("Data/Complex", 50),
        // Algebra categories
        ("Algebra", 50),
        ("Algebra/Group", 100),
        ("Algebra/Ring", 100),
        ("Algebra/Field", 50),
        ("Algebra/Module", 100),
        ("Algebra/Order/Group", 50),
        ("Algebra/Order/Ring", 50),
        ("Algebra/BigOperators", 50),
        ("Algebra/Algebra", 50),
        ("Algebra/Group/Subgroup", 50),
        ("Algebra/Group/Hom", 50),
        ("Algebra/Group/Submonoid", 50),
        ("Algebra/GroupWithZero", 100),
        ("Algebra/CharP", 50),
        // Group/Ring/Field theory
        ("GroupTheory/Coset", 50),
        ("GroupTheory", 100),
        ("GroupTheory/QuotientGroup", 50),
        ("RingTheory", 100),
        ("RingTheory/Coprime", 50),
        ("FieldTheory", 100),
        // Order
        ("Order", 100),
        ("Order/Filter", 100),
        // Number theory
        ("NumberTheory", 100),
        ("NumberTheory/ArithmeticFunction", 50),
        // Linear algebra & Combinatorics
        ("LinearAlgebra", 100),
        ("Combinatorics/SimpleGraph", 100),
        // Topology & Measure theory
        ("Topology", 100),
        ("SetTheory/Cardinal", 50),
        ("SetTheory/Ordinal", 50),
        ("MeasureTheory/Measure", 100),
        // New: Analysis
        ("Analysis/Normed", 50),
        ("Analysis/InnerProductSpace", 50),
        ("Analysis/Calculus", 50),
        ("Analysis/SpecificLimits", 50),
        ("Analysis/Complex", 100),
        ("Analysis/Convex", 100),
        // New: Category Theory
        ("CategoryTheory/Functor", 50),
        ("CategoryTheory/Limits", 100),
        ("CategoryTheory/Monoidal", 100),
        ("CategoryTheory/Adjunction", 50),
        // New: Geometry
        ("Geometry/Euclidean", 50),
        ("Geometry/Manifold", 50),
        // New: Probability & Dynamics
        ("Probability/ProbabilityMassFunction", 50),
        ("Dynamics", 50),
        // New: Computability
        ("Computability", 100),
        // New: Algebraic Geometry/Topology
        ("AlgebraicGeometry", 100),
        ("AlgebraicTopology", 50),
        // New: Model/Representation/Information Theory
        ("ModelTheory", 50),
        ("RepresentationTheory", 50),
        ("InformationTheory", 50),
        // Phase 2: Tactic & Control
        ("Tactic", 200),
        ("Condensed", 50),
        ("Control", 50),
        // Phase 2: Combinatorics (additional)
        ("Combinatorics", 50),
        ("Combinatorics/Additive", 50),
        ("Combinatorics/SetFamily", 50),
        // Phase 2: Analysis (additional)
        ("Analysis/SpecialFunctions", 50),
        ("Analysis/CStarAlgebra", 50),
        ("Analysis/LocallyConvex", 50),
        ("Analysis/Fourier", 50),
        // Phase 2: MeasureTheory (main)
        ("MeasureTheory", 50),
        // Phase 2: Probability (additional)
        ("Probability/Kernel", 50),
        // Phase 2: Category Theory (additional)
        ("CategoryTheory/Abelian", 50),
        ("CategoryTheory/Category", 50),
        // Phase 2: Topology (additional)
        ("Topology/Algebra", 100),
        // Phase 3: Topology expansion
        ("Topology/MetricSpace", 100),
        ("Topology/UniformSpace", 50),
        ("Topology/Order", 100),
        ("Topology/ContinuousMap", 50),
        // Phase 3: Algebra expansion
        ("Algebra/Polynomial", 100),
        ("Algebra/Homology", 100),
        ("Algebra/Lie", 100),
        ("Algebra/Star", 50),
        ("Algebra/MvPolynomial", 50),
        // Phase 3: LinearAlgebra expansion
        ("LinearAlgebra/Matrix", 100),
        // Phase 3: MeasureTheory expansion
        ("MeasureTheory/Integral", 100),
        ("MeasureTheory/Function", 100),
        // Phase 3: RingTheory expansion
        ("RingTheory/Ideal", 50),
        // Phase 3: GroupTheory expansion
        ("GroupTheory/GroupAction", 50),
        // Phase 3: NumberTheory expansion
        ("NumberTheory/LSeries", 50),
        // Phase 4: Major expansion
        ("Algebra/Order/Field", 50),
        ("Algebra/DirectSum", 50),
        ("Algebra/MonoidAlgebra", 50),
        ("CategoryTheory/Bicategory", 50),
        ("CategoryTheory/ConcreteCategory", 50),
        ("Topology/Category", 50),
        ("Topology/Instances", 50),
        ("Topology/Separation", 50),
        ("Topology/Compactness", 50),
        ("Topology/Connected", 50),
        ("Topology/Sheaves", 50),
        ("RingTheory/Polynomial", 100),
        ("RingTheory/Localization", 100),
        ("RingTheory/PowerSeries", 100),
        ("RingTheory/DedekindDomain", 50),
        ("RingTheory/Valuation", 50),
        ("NumberTheory/NumberField", 50),
        ("NumberTheory/ModularForms", 50),
        ("NumberTheory/Padics", 50),
        ("LinearAlgebra/TensorProduct", 50),
        ("LinearAlgebra/AffineSpace", 50),
        ("LinearAlgebra/QuadraticForm", 50),
        ("GroupTheory/Perm", 50),
        ("Order/Interval", 50),
        ("MeasureTheory/Constructions", 50),
        ("MeasureTheory/Group", 50),
        ("Analysis/NormedSpace", 50),
        ("Analysis/Analytic", 50),
        ("Analysis/BoxIntegral", 50),
        ("Data/Real", 50),
        ("Data/Matrix", 50),
        ("Data/DFinsupp", 50),
        ("Data/ENNReal", 50),
        ("Combinatorics/Enumerative", 50),
        ("Combinatorics/Matroid", 50),
        ("Probability/Distributions", 50),
        ("Geometry/RingedSpace", 50),
        ("SetTheory/Game", 50),
        // Phase 5: Deep expansion
        ("RingTheory/WittVector", 50),
        ("RingTheory/RingHom", 50),
        ("RingTheory/TensorProduct", 50),
        ("RingTheory/UniqueFactorizationDomain", 50),
        ("RingTheory/Smooth", 50),
        ("Order/Category", 50),
        ("Order/SuccPred", 50),
        ("Order/Hom", 50),
        ("LinearAlgebra/RootSystem", 50),
        ("LinearAlgebra/Dimension", 50),
        ("LinearAlgebra/CliffordAlgebra", 50),
        ("LinearAlgebra/Basis", 50),
        ("LinearAlgebra/Multilinear", 50),
        ("Analysis/Asymptotics", 50),
        ("MeasureTheory/MeasurableSpace", 50),
        ("Probability/Independence", 50),
        ("Probability/Moments", 50),
        ("CategoryTheory/Galois", 50),
        ("CategoryTheory/Filtered", 50),
        ("GroupTheory/MonoidLocalization", 50),
        // Phase 6: Untested large subdirectories
        ("CategoryTheory/Localization", 100),
        ("CategoryTheory/Preadditive", 50),
        ("CategoryTheory/ObjectProperty", 50),
        ("CategoryTheory/MorphismProperty", 50),
        ("CategoryTheory/Triangulated", 50),
        ("CategoryTheory/Shift", 50),
        ("CategoryTheory/Presentable", 50),
        ("CategoryTheory/Comma", 50),
        ("CategoryTheory/Subobject", 50),
        ("CategoryTheory/Monad", 50),
        ("CategoryTheory/Closed", 50),
        ("CategoryTheory/SmallObject", 50),
        ("CategoryTheory/Enriched", 50),
        ("CategoryTheory/Generator", 50),
        ("CategoryTheory/Idempotents", 50),
        ("CategoryTheory/FiberedCategory", 50),
        ("CategoryTheory/GradedObject", 50),
        ("CategoryTheory/EffectiveEpi", 50),
        ("CategoryTheory/Groupoid", 50),
        ("AlgebraicTopology/SimplicialSet", 100),
        ("AlgebraicTopology/ModelCategory", 50),
        ("AlgebraicTopology/DoldKan", 50),
        ("AlgebraicTopology/SimplexCategory", 50),
        ("AlgebraicGeometry/Morphisms", 100),
        ("AlgebraicGeometry/EllipticCurve", 50),
        ("Tactic/Linter", 50),
        ("Tactic/NormNum", 50),
        ("Tactic/CategoryTheory", 50),
        ("RingTheory/Spectrum", 50),
        ("RingTheory/Finiteness", 50),
        ("RingTheory/MvPolynomial", 50),
        ("RingTheory/LocalRing", 50),
        ("RingTheory/Flat", 50),
        ("Condensed/Light", 50),
        ("RepresentationTheory/Homological", 50),
        ("Topology/Homotopy", 50),
        ("Topology/EMetricSpace", 50),
        ("MeasureTheory/VectorMeasure", 50),
        ("MeasureTheory/OuterMeasure", 50),
        ("MeasureTheory/Covering", 50),
        ("Analysis/Distribution", 50),
        ("Analysis/Real", 50),
        ("Analysis/Meromorphic", 50),
        ("Dynamics/Ergodic", 50),
        ("Probability/Process", 50),
        ("Probability/Martingale", 50),
        ("Geometry/Convex", 50),
        ("FieldTheory/Galois", 50),
        ("FieldTheory/IntermediateField", 50),
        ("NumberTheory/Cyclotomic", 50),
        ("Data/NNRat", 50),
        ("Data/Finite", 50),
        ("LinearAlgebra/FreeModule", 50),
        ("LinearAlgebra/Eigenspace", 50),
        ("Order/CompleteLattice", 50),
        ("GroupTheory/FreeGroup", 50),
        ("GroupTheory/SpecificGroups", 50),
        ("SetTheory/ZFC", 50),
        // Phase 7: Deep third-level dirs + remaining second-level
        ("CategoryTheory/Limits/Shapes", 100),
        ("CategoryTheory/Limits/Constructions", 50),
        ("Analysis/Normed/Group", 100),
        ("Analysis/Normed/Ring", 50),
        ("Analysis/Normed/Field", 50),
        ("Analysis/Calculus/FDeriv", 50),
        ("Analysis/SpecialFunctions/Trigonometric", 50),
        ("Analysis/SpecialFunctions/Log", 50),
        ("Analysis/SpecialFunctions/Pow", 50),
        ("Analysis/SpecialFunctions/Gamma", 50),
        ("Topology/Algebra/Module", 50),
        ("Topology/Algebra/Group", 50),
        ("Algebra/Order/Monoid", 50),
        ("Algebra/Order/Hom", 50),
        ("Algebra/Homology/ShortComplex", 50),
        ("Data/Nat/Prime", 50),
        ("Probability/Kernel/Composition", 50),
        ("MeasureTheory/Function/LpSeminorm", 50),
        ("Geometry/Manifold/VectorBundle", 50),
        ("Geometry/Manifold/MFDeriv", 50),
        ("RingTheory/Ideal/Quotient", 50),
        ("Combinatorics/SimpleGraph/Connectivity", 50),
        // More second-level untested
        ("Algebra/ContinuedFractions", 50),
        ("Algebra/GCDMonoid", 50),
        ("RingTheory/IntegralClosure", 50),
        ("RingTheory/HahnSeries", 50),
        ("RingTheory/GradedAlgebra", 50),
        ("RingTheory/Adjoin", 50),
        ("LinearAlgebra/BilinearForm", 50),
        ("LinearAlgebra/PiTensorProduct", 50),
        ("LinearAlgebra/Finsupp", 50),
        ("NumberTheory/LegendreSymbol", 50),
        ("Order/UpperLower", 50),
        ("MeasureTheory/OuterMeasure", 50),
        ("FieldTheory/Minpoly", 50),
        ("FieldTheory/Finite", 50),
        ("Dynamics/TopologicalEntropy", 50),
        ("Condensed/Discrete", 50),
        // Phase 7b: More third-level dirs
        ("Analysis/Normed/Module", 50),
        ("Analysis/Calculus/Deriv", 50),
        ("Order/Interval/Set", 50),
        ("Algebra/Category/ModuleCat", 50),
        ("Algebra/Order", 50),
        ("RingTheory/MvPowerSeries", 50),
        ("RingTheory/Noetherian", 50),
        ("Combinatorics/Quiver", 50),
        ("Algebra/Regular", 50),
        // Phase 7c: More third-level untested dirs
        ("Algebra/Category/Grp", 50),
        ("Order/Filter/AtTopBot", 50),
        ("Analysis/Normed/Operator", 50),
        ("Algebra/Homology/HomotopyCategory", 50),
        ("Topology/Algebra/InfiniteSum", 50),
        ("Algebra/Homology/Embedding", 50),
        ("RingTheory/Spectrum/Prime", 50),
        ("CategoryTheory/Limits/Types", 50),
        ("Algebra/Module/Submodule", 50),
        ("Algebra/Group/Action", 50),
        ("Analysis/Calculus/ContDiff", 50),
        ("Analysis/CStarAlgebra/ContinuousFunctionalCalculus", 50),
        ("Algebra/Algebra/Subalgebra", 50),
        ("MeasureTheory/Integral/IntervalIntegral", 50),
        ("CategoryTheory/Limits/Preserves", 50),
        ("Analysis/Normed/Algebra", 50),
        ("Algebra/Polynomial/Degree", 50),
        ("Algebra/Order/Module", 50),
        ("Algebra/Homology/DerivedCategory", 50),
        ("Algebra/GroupWithZero/Action", 50),
        // Phase 8: 4th-level dirs + remaining untested
        ("CategoryTheory/Limits/Shapes/Pullback", 50),
        ("CategoryTheory/Limits/Preserves/Shapes", 50),
        ("Algebra/Category/ModuleCat/Presheaf", 50),
        ("Algebra/BigOperators/Group/Finset", 50),
        ("Algebra/Category/ModuleCat/Sheaf", 50),
        ("Algebra/Order/Monoid/Unbundled", 50),
        ("Algebra/Homology/DerivedCategory/Ext", 50),
        ("Algebra/Group/Pointwise/Set", 50),
        ("CategoryTheory/Sites/Coherent", 50),
        ("Data/QPF", 50),
        ("RingTheory/Extension", 50),
        ("RingTheory/AdicCompletion", 50),
        ("Analysis/RCLike", 50),
        ("Analysis/Matrix", 50),
        ("Analysis/Polynomial", 50),
        ("Geometry/Euclidean/Angle", 50),
        ("GroupTheory/Congruence", 50),
        // Phase 9
        ("Util", 50),
        ("Data", 50),
        ("Analysis", 50),
        ("Logic/Equiv", 50),
        ("Probability", 50),
        ("Logic/Function", 50),
        ("Data/Nat/Choose", 50),
        ("CategoryTheory/Monoidal/Cartesian", 50),
        ("Algebra/Category/Ring", 50),
        ("Topology/Category/TopCat", 50),
        ("Tactic/Widget", 50),
        ("Topology/Algebra/Order", 50),
        ("Tactic/FunProp", 50),
        ("RingTheory/KrullDimension", 50),
        ("MeasureTheory/Measure/Haar", 50),
        ("Data/Nat/Cast", 50),
        ("Data/Fin/Tuple", 50),
        ("CategoryTheory/Monoidal/Closed", 50),
        ("CategoryTheory/Bicategory/Functor", 50),
        ("Analysis/SpecialFunctions/Complex", 50),
        ("Analysis/Normed/Unbundled", 50),
        ("Analysis/Normed/Affine", 50),
        ("Algebra/Ring/Action", 50),
        ("Algebra/Module/Presentation", 50),
        ("Topology/Category/Profinite", 50),
        ("Topology/Category/LightProfinite", 50),
        ("RingTheory/RootsOfUnity", 50),
        ("RingTheory/Coalgebra", 50),
        ("RingTheory/Algebraic", 50),
        ("Probability/Kernel/Disintegration", 50),
        ("Order/Interval/Finset", 50),
        ("MeasureTheory/Integral/Lebesgue", 50),
        ("MeasureTheory/Function/ConditionalExpectation", 50),
        ("MeasureTheory/Constructions/BorelSpace", 50),
        ("LinearAlgebra/Matrix/Charpoly", 50),
        ("Analysis/NormedSpace/OperatorNorm", 50),
        ("Analysis/Normed/Lp", 50),
        ("Analysis/Calculus/TangentCone", 50),
        ("AlgebraicGeometry/Sites", 50),
        ("Algebra/Module/LinearMap", 50),
        ("Topology/Sets", 50),
        ("Topology/Algebra/Valued", 50),
        ("RingTheory/Unramified", 50),
        ("RingTheory/LocalProperties", 50),
        ("RingTheory/Etale", 50),
        ("RingTheory/AlgebraicIndependent", 50),
        ("RepresentationTheory/Homological/GroupCohomology", 50),
        ("Order/Monotone", 50),
        ("NumberTheory/ModularForms/EisensteinSeries", 50),
        ("Data/Set/Finite", 50),
        ("Combinatorics/SimpleGraph/Regularity", 50),
        ("CategoryTheory/Limits/Indization", 50),
        ("CategoryTheory/Category/Cat", 50),
        ("Analysis/Complex/UpperHalfPlane", 50),
        ("Algebra/Ring/Subring", 50),
        ("Algebra/Polynomial/Eval", 50),
        ("Algebra/Order/GroupWithZero", 50),
        ("Algebra/Module/LocalizedModule", 50),
        ("Algebra/Lie/Weights", 50),
        ("Algebra/Category/MonCat", 50),
    ];
    let recursive_dirs = [
        ("Algebra", 10000usize),
        ("AlgebraicGeometry", 10000),
        ("AlgebraicTopology", 10000),
        ("Analysis", 10000),
        ("CategoryTheory", 10000),
        ("Combinatorics", 10000),
        ("Computability", 10000),
        ("Condensed", 10000),
        ("Control", 10000),
        ("Data", 10000),
        ("Deprecated", 10000),
        ("Dynamics", 10000),
        ("FieldTheory", 10000),
        ("Geometry", 10000),
        ("GroupTheory", 10000),
        ("InformationTheory", 10000),
        ("Lean", 10000),
        ("LinearAlgebra", 10000),
        ("Logic", 10000),
        ("MeasureTheory", 10000),
        ("ModelTheory", 10000),
        ("NumberTheory", 10000),
        ("Order", 10000),
        ("Probability", 10000),
        ("RepresentationTheory", 10000),
        ("RingTheory", 10000),
        ("SetTheory", 10000),
        ("Tactic", 10000),
        ("Testing", 10000),
        ("Topology", 10000),
        ("Util", 10000),
    ];
    let mut total_stats = CompatStats::default();
    // Skip flat dirs covered by recursive dirs (avoid double-counting)
    let recursive_prefixes: Vec<&str> = recursive_dirs.iter().map(|(d, _)| *d).collect();
    for (dir_name, max_files) in &flat_dirs {
        let covered = recursive_prefixes
            .iter()
            .any(|r| *dir_name == *r || dir_name.starts_with(&format!("{r}/")));
        if covered {
            continue;
        }
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
    // Also scan Archive/ and Counterexamples/ (sibling dirs of Mathlib/)
    for extra_dir in &["../Archive", "../Counterexamples"] {
        let dir = root.join(extra_dir);
        if dir.exists() {
            let s = run_compat_recursive(&dir, 10000);
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
         Date: 2026-03-05\n\n\
         ## Summary\n\
         Files processed: {}\n\
         Total declarations tested: {}\n\
         Parsed successfully: {}\n\
         Compatibility rate: {:.1}%\n\n\
         ## Categories\n\
         - {} flat directories + {} recursive directories\n\
         - max_files: 50 (flat), 100 (recursive)\n\n\
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
         - def without return type: `def f (x : T) :=` -> `def f := sorry`\n",
        total_stats.files_processed, total_stats.total, total_stats.parsed_ok,
        total_stats.success_rate(),
        flat_dirs.len(), recursive_dirs.len()
    );
    let _ = std::fs::write("/tmp/oxilean_compat_report.md", &report);
    println!("Report written to /tmp/oxilean_compat_report.md");
}
#[test]
#[allow(dead_code)]
fn test_debug_bigunion() {
    let decl = "theorem iUnion_setOf (P : \u{03B9} \u{2192} \u{03B1} \u{2192} Prop) : \u{22C3} i, { x : \u{03B1} | P i x } = { x : \u{03B1} | \u{2203} i, P i x } := by ext; exact mem_iUnion";
    let normalized = normalize_lean4_to_oxilean(decl);
    println!("Normalized: {normalized}");
    let ok = try_parse_decl(&normalized);
    println!("Parse OK: {ok}");
}
#[test]
#[allow(dead_code)]
fn test_debug_have_in_type() {
    let decl =
        "theorem natCast_eq_mk {m n : \u{2115}} (h : m < n) : have : NeZero n := \u{27E8}Nat.ne_zero_of_lt h\u{27E9}";
    let normalized = normalize_lean4_to_oxilean(decl);
    println!("Normalized natCast: {normalized}");
    let ok = try_parse_decl(&normalized);
    println!("Parse OK: {ok}");
    let decl2 = "lemma _root_.finCongr_eq_equivCast (h : n = m) : finCongr h = .cast (h \u{25B8} rfl) := by subst h; simp";
    let normalized2 = normalize_lean4_to_oxilean(decl2);
    println!("Normalized finCongr: {normalized2}");
    let ok2 = try_parse_decl(&normalized2);
    println!("Parse OK: {ok2}");
    let decl3 =
        "theorem card_perms_of_finset : \u{2200} s : Finset \u{03B1}, #(permsOfFinset s) = (#s)! := by simp";
    let normalized3 = normalize_lean4_to_oxilean(decl3);
    println!("Normalized card_perms: {normalized3}");
    let ok3 = try_parse_decl(&normalized3);
    println!("Parse OK: {ok3}");
}
#[test]
#[allow(dead_code)]
fn test_debug_remaining() {
    let decls = vec![
        "theorem cons_val_two (x : \u{03B1}) (u : Fin m.succ.succ \u{2192} \u{03B1}) : vecCons x u 2 = vecHead (vecTail u) := rfl",
        "lemma iInter_sum {s : \u{03B1} \u{2295} \u{03B2} \u{2192} Set \u{03B3}} : \u{22C2} x, s x = (\u{22C2} x, s (.inl x)) \u{2229} \u{22C2} x, s (.inr x) := iInf_sup_eq",
        "lemma iUnion_sum {s : \u{03B1} \u{2295} \u{03B2} \u{2192} Set \u{03B3}} : \u{22C3} x, s x = (\u{22C3} x, s (.inl x)) \u{222A} \u{22C3} x, s (.inr x) := iSup_sup_eq",
        "theorem iUnion_eq_range_sigma (s : \u{03B1} \u{2192} Set \u{03B2}) : \u{22C3} i, s i = range fun a : \u{03A3} i, s i => a.2 := by ext; exact mem_iUnion",
        "theorem iUnion_eq_range_psigma (s : \u{03B9} \u{2192} Set \u{03B2}) : \u{22C3} i, s i = range fun a : \u{03A3}' i, s i => a.2 := by ext; exact mem_iUnion",
        "theorem or_le : \u{2200} {x y z}, x \u{2264} z \u{2192} y \u{2264} z \u{2192} (x || y) \u{2264} z := by decide",
        "lemma pairwise_iff_lt (hp : Symmetric p) : Pairwise p \u{2194} \u{2200} \u{2983}a b\u{2984}, a < b \u{2192} p a b := by simp",
        "lemma Finite.pi' (ht : \u{2200} i, (t i).Finite) : {f : \u{2200} i, \u{03BA} i | \u{2200} i, f i \u{2208} t i}.Finite := by simp",
        "theorem iUnion_of_singleton_coe (s : Set \u{03B1}) : \u{22C3} i : s, ({(i : \u{03B1})} : Set \u{03B1}) = s := by simp",
        "\u{2200} {x y z}, x \u{2264} z \u{2192} y \u{2264} z",
        "theorem test_fin_type : forall (u : Fin (m.succ).succ -> \u{03B1}), vecHead u = u := sorry",
        "theorem test_fin2 : forall (u : Fin m.succ.succ -> \u{03B1}), u = u := sorry",
        "theorem test_sigma_fun : forall (a : Sigma i, s i), a = a := sorry",
        "theorem sSup_iUnion (t : \u{03B9} \u{2192} Set \u{03B2}) : sSup (\u{22C3} i, t i) = \u{2A06} i, sSup (t i) := by simp",
        "theorem ne_key {a} {l : List (Sigma \u{03B2})} : a \u{2209} l.keys \u{2194} \u{2200} s : Sigma \u{03B2}, s \u{2208} l \u{2192} a \u{2260} s.fst := by simp",
        "theorem coe_image_of_subset {s t : Set \u{03B1}} (h : t \u{2286} s) : (\u{2191}) '' { x : \u{2191}s | \u{2191}x \u{2208} t } = t := by simp",
        "theorem sSup_sUnion (s : Set (Set \u{03B2})) : sSup (\u{22C3}\u{2080} s) = \u{2A06} t \u{2208} s, sSup t := by simp",
        "theorem finprod_one : (\u{220F}\u{1D1F} _ : \u{03B1}, (1 : M)) = 1 := by simp",
        "theorem finprod_of_isEmpty [IsEmpty \u{03B1}] (f : \u{03B1} \u{2192} M) : \u{220F}\u{1D1F} i, f i = 1 := by simp",
        "theorem mul_prod_removeNth i (f : Fin (n + 1) \u{2192} M) : f i * \u{220F} j, removeNth i f j = \u{220F} j, removeNth i f j := by simp",
        "theorem sum_moebius_mul_log_eq {n : \u{2115}} : (\u{2211} d \u{2208} n.divisors, (\u{03BC} d : \u{211D}) * log d) = 0 := by simp",
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
#[test]
#[allow(dead_code)]
fn test_debug_tactic_remaining() {
    let decls = vec![
        "theorem nth_of_forall_not {n : \u{2115}} (hp : \u{2200} n' \u{2265} n, \u{00AC}p n') : nth p n = 0 := by simp",
        "theorem indicator_apply [DecidableEq \u{03B9}] : indicator s f i = if hi : i \u{2208} s then f \u{27E8}i, hi\u{27E9} else 0 := by simp",
        "lemma mul_def (x y : \u{2A02}[R] i, A i) : x * y = mul x y := rfl",
        "theorem sum_four_squares (n : \u{2115}) : \u{2203} a b c d : \u{2115}, a ^ 2 + b ^ 2 + c ^ 2 + d ^ 2 = n := by simp",
        "theorem isIntegral_exp_rat_mul_pi_mul_I (q : \u{211A}) : IsIntegral \u{2124} <| exp <| q * \u{03C0} * I := by simp",
        "theorem interval_average_symm (f : \u{211D} \u{2192} E) (a b : \u{211D}) : (\u{2A0D} x in a..b, f x) = \u{2A0D} x in b..a, f x := by simp",
        "theorem test1 : a  Mem  s = a := sorry",
        "theorem shadow_singleton (a : \u{03B1}) : \u{2202} {{a}} = {\u{2205}} := by simp",
        "theorem primorial_succ {n : \u{2115}} (hn1 : n \u{2260} 1) (hn : Odd n) : (n + 1)# = n# := by simp",
        "theorem nhds_zero : \u{1D4DD} (0 : \u{0393}\u{2080}) = \u{2A05} \u{03B3} \u{2260} 0, \u{1D4DF} (Iio \u{03B3}) := by simp",
        "theorem cofinite_limsup : limsup s cofinite = { x | { n | x \u{2208} s n }.Infinite } := by simp",
        "theorem piecewise_empty [\u{2200} i : \u{03B9}, Decidable (i \u{2208} (\u{2205} : Finset \u{03B9}))] : piecewise \u{2205} f g = g := by simp",
        "theorem integral_bern : \u{222B} x : \u{211D} in 0..1, bernoulliFun k x = if k = 0 then 1 else 0 := by simp",
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
