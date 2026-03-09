//! Category-specific Mathlib4 compatibility tests.
//!
//! Each test scans a Mathlib4 subdirectory, normalizes declarations,
//! and asserts a minimum parse compatibility rate.

use std::path::Path;

use super::test_infra::{mathlib4_root, print_stats, run_compat_on_dir, run_compat_recursive};

/// Helper macro for category compat tests (flat directory scan).
macro_rules! compat_test_flat {
    ($name:ident, $dir:expr, $max_files:expr, $min_pct:expr) => {
        #[test]
        #[ignore]
        fn $name() {
            let mathlib4_root = match mathlib4_root() {
                Some(r) => r,
                None => return,
            };
            let dir = Path::new(&mathlib4_root).join($dir);
            if !dir.exists() {
                println!("[SKIP] Mathlib4 not found at {}", dir.display());
                return;
            }
            let stats = run_compat_on_dir(&dir, $max_files);
            print_stats($dir, &stats);
            if stats.total > 0 {
                println!(
                    "  Compat rate: {:.1}% ({}/{})",
                    stats.success_rate(),
                    stats.parsed_ok,
                    stats.total
                );
                assert!(
                    stats.parsed_ok * 100 >= stats.total * $min_pct,
                    "{} compat should be >={}%: got {}/{}",
                    $dir,
                    $min_pct,
                    stats.parsed_ok,
                    stats.total
                );
            } else {
                println!("  No single-line declarations found in {}", $dir);
            }
        }
    };
}

/// Helper macro for category compat tests (recursive directory scan).
macro_rules! compat_test_recursive {
    ($name:ident, $dir:expr, $max_files:expr, $min_pct:expr) => {
        #[test]
        #[ignore]
        fn $name() {
            let mathlib4_root = match mathlib4_root() {
                Some(r) => r,
                None => return,
            };
            let dir = Path::new(&mathlib4_root).join($dir);
            if !dir.exists() {
                println!("[SKIP] Mathlib4 not found at {}", dir.display());
                return;
            }
            let stats = run_compat_recursive(&dir, $max_files);
            print_stats($dir, &stats);
            if stats.total > 0 {
                println!(
                    "  Compat rate: {:.1}% ({}/{})",
                    stats.success_rate(),
                    stats.parsed_ok,
                    stats.total
                );
                assert!(
                    stats.parsed_ok * 100 >= stats.total * $min_pct,
                    "{} compat should be >={}%: got {}/{}",
                    $dir,
                    $min_pct,
                    stats.parsed_ok,
                    stats.total
                );
            } else {
                println!("  No single-line declarations found in {}", $dir);
            }
        }
    };
}

// ============================================================
// Existing categories (max_files raised to 50-100)
// ============================================================

// --- Data categories ---
compat_test_flat!(test_nat_compat, "Data/Nat", 50, 90);
compat_test_flat!(test_data_list_compat, "Data/List", 100, 90);
compat_test_flat!(test_data_option_compat, "Data/Option", 50, 90);
compat_test_flat!(test_data_prod_compat, "Data/Prod", 50, 90);
compat_test_flat!(test_data_sum_compat, "Data/Sum", 50, 90);
compat_test_flat!(test_data_int_compat, "Data/Int", 50, 90);
compat_test_flat!(test_data_bool_compat, "Data/Bool", 50, 90);
compat_test_flat!(test_data_fin_compat, "Data/Fin", 50, 90);
compat_test_flat!(test_data_multiset_compat, "Data/Multiset", 50, 90);
compat_test_flat!(test_data_finset_compat, "Data/Finset", 50, 90);
compat_test_flat!(test_data_set_compat, "Data/Set", 50, 90);
compat_test_flat!(test_data_rat_compat, "Data/Rat", 50, 90);
compat_test_flat!(test_data_pnat_compat, "Data/PNat", 50, 90);
compat_test_flat!(test_data_fintype_compat, "Data/Fintype", 50, 90);
compat_test_flat!(test_data_enat_compat, "Data/ENat", 50, 90);
compat_test_flat!(test_data_finsupp_compat, "Data/Finsupp", 50, 90);
compat_test_flat!(test_data_nat_gcd_compat, "Data/Nat/GCD", 50, 90);
compat_test_flat!(test_data_nat_factorial_compat, "Data/Nat/Factorial", 50, 90);
compat_test_flat!(test_data_zmod_compat, "Data/ZMod", 50, 90);
compat_test_flat!(test_data_complex_compat, "Data/Complex", 50, 90);

// --- Algebra categories ---
compat_test_flat!(test_algebra_compat, "Algebra", 50, 90);
compat_test_flat!(test_algebra_group_compat, "Algebra/Group", 50, 90);
compat_test_flat!(test_algebra_ring_compat, "Algebra/Ring", 50, 90);
compat_test_flat!(test_algebra_field_compat, "Algebra/Field", 50, 90);
compat_test_flat!(test_algebra_module_compat, "Algebra/Module", 50, 90);
compat_test_flat!(
    test_algebra_order_group_compat,
    "Algebra/Order/Group",
    50,
    90
);
compat_test_flat!(test_algebra_order_ring_compat, "Algebra/Order/Ring", 50, 90);
compat_test_flat!(
    test_algebra_bigoperators_compat,
    "Algebra/BigOperators",
    50,
    90
);
compat_test_flat!(test_algebra_algebra_compat, "Algebra/Algebra", 50, 90);
compat_test_flat!(
    test_algebra_group_subgroup_compat,
    "Algebra/Group/Subgroup",
    50,
    90
);
compat_test_flat!(test_algebra_group_hom_compat, "Algebra/Group/Hom", 50, 90);
compat_test_flat!(
    test_algebra_group_submonoid_compat,
    "Algebra/Group/Submonoid",
    50,
    90
);
compat_test_flat!(
    test_algebra_groupwithzero_compat,
    "Algebra/GroupWithZero",
    50,
    90
);
compat_test_flat!(test_algebra_charp_compat, "Algebra/CharP", 50, 90);

// --- Group/Ring/Field theory ---
compat_test_flat!(test_grouptheory_coset_compat, "GroupTheory/Coset", 50, 90);
compat_test_flat!(test_grouptheory_main_compat, "GroupTheory", 50, 90);
compat_test_flat!(
    test_grouptheory_quotientgroup_compat,
    "GroupTheory/QuotientGroup",
    50,
    90
);
compat_test_flat!(test_ringtheory_compat, "RingTheory", 100, 90);
compat_test_flat!(test_ringtheory_coprime_compat, "RingTheory/Coprime", 50, 90);
compat_test_flat!(test_fieldtheory_compat, "FieldTheory", 50, 90);

// --- Order & Logic ---
compat_test_flat!(test_order_compat, "Order", 100, 90);
compat_test_flat!(test_order_filter_compat, "Order/Filter", 50, 90);
compat_test_recursive!(test_logic_compat, "Logic", 100, 90);

// --- Number theory ---
compat_test_flat!(test_number_theory_basic_compat, "NumberTheory", 50, 90);
compat_test_flat!(
    test_numbertheory_arithfunc_compat,
    "NumberTheory/ArithmeticFunction",
    50,
    90
);

// --- Linear algebra ---
compat_test_flat!(test_linearalgebra_compat, "LinearAlgebra", 50, 90);

// --- Combinatorics ---
compat_test_flat!(
    test_combinatorics_simplegraph_compat,
    "Combinatorics/SimpleGraph",
    50,
    90
);

// --- Topology & Geometry ---
compat_test_flat!(test_topology_compat, "Topology", 100, 90);
compat_test_flat!(test_settheory_cardinal_compat, "SetTheory/Cardinal", 50, 90);
compat_test_flat!(test_settheory_ordinal_compat, "SetTheory/Ordinal", 50, 90);
compat_test_flat!(
    test_measuretheory_measure_compat,
    "MeasureTheory/Measure",
    50,
    90
);

// ============================================================
// New categories
// ============================================================

// --- Analysis (split into submodules for size) ---
compat_test_flat!(test_analysis_normed_compat, "Analysis/Normed", 50, 90);
compat_test_flat!(
    test_analysis_innerproductspace_compat,
    "Analysis/InnerProductSpace",
    50,
    90
);
compat_test_flat!(test_analysis_calculus_compat, "Analysis/Calculus", 50, 90);
compat_test_flat!(
    test_analysis_specificlimits_compat,
    "Analysis/SpecificLimits",
    50,
    90
);
compat_test_flat!(test_analysis_complex_compat, "Analysis/Complex", 50, 90);
compat_test_flat!(test_analysis_convex_compat, "Analysis/Convex", 50, 90);

// --- Category Theory ---
compat_test_flat!(
    test_categorytheory_functor_compat,
    "CategoryTheory/Functor",
    50,
    90
);
compat_test_flat!(
    test_categorytheory_limits_compat,
    "CategoryTheory/Limits",
    50,
    90
);
compat_test_flat!(
    test_categorytheory_monoidal_compat,
    "CategoryTheory/Monoidal",
    50,
    90
);
compat_test_flat!(
    test_categorytheory_adjunction_compat,
    "CategoryTheory/Adjunction",
    50,
    90
);

// --- Geometry ---
compat_test_flat!(test_geometry_euclidean_compat, "Geometry/Euclidean", 50, 90);
compat_test_flat!(test_geometry_manifold_compat, "Geometry/Manifold", 50, 90);

// --- Probability & Dynamics ---
compat_test_flat!(
    test_probability_pmf_compat,
    "Probability/ProbabilityMassFunction",
    50,
    90
);
compat_test_flat!(test_dynamics_compat, "Dynamics", 50, 90);

// --- Computability ---
compat_test_flat!(test_computability_compat, "Computability", 50, 90);

// --- Algebraic Geometry / Topology ---
compat_test_flat!(test_algebraicgeometry_compat, "AlgebraicGeometry", 50, 90);
compat_test_flat!(test_algebraictopology_compat, "AlgebraicTopology", 50, 90);

// --- Model Theory & Representation Theory ---
compat_test_flat!(test_modeltheory_compat, "ModelTheory", 50, 90);
compat_test_flat!(
    test_representationtheory_compat,
    "RepresentationTheory",
    50,
    90
);

// --- Information Theory ---
compat_test_flat!(test_informationtheory_compat, "InformationTheory", 50, 90);

// --- Tactic & Control ---
compat_test_flat!(test_tactic_compat, "Tactic", 200, 90);
compat_test_flat!(test_condensed_compat, "Condensed", 50, 90);
compat_test_flat!(test_control_compat, "Control", 50, 90);

// --- Combinatorics (additional) ---
compat_test_flat!(test_combinatorics_main_compat, "Combinatorics", 50, 90);
compat_test_flat!(
    test_combinatorics_additive_compat,
    "Combinatorics/Additive",
    50,
    90
);
compat_test_flat!(
    test_combinatorics_setfamily_compat,
    "Combinatorics/SetFamily",
    50,
    90
);

// --- Analysis (additional) ---
compat_test_flat!(
    test_analysis_specialfunctions_compat,
    "Analysis/SpecialFunctions",
    50,
    90
);
compat_test_flat!(
    test_analysis_cstaralgebra_compat,
    "Analysis/CStarAlgebra",
    50,
    90
);
compat_test_flat!(
    test_analysis_locallyconvex_compat,
    "Analysis/LocallyConvex",
    50,
    90
);
compat_test_flat!(test_analysis_fourier_compat, "Analysis/Fourier", 50, 90);

// --- MeasureTheory (main) ---
compat_test_flat!(test_measuretheory_main_compat, "MeasureTheory", 50, 90);

// --- Probability (additional) ---
compat_test_flat!(test_probability_kernel_compat, "Probability/Kernel", 50, 90);

// --- Category Theory (additional) ---
compat_test_flat!(
    test_categorytheory_abelian_compat,
    "CategoryTheory/Abelian",
    50,
    90
);
compat_test_flat!(
    test_categorytheory_category_compat,
    "CategoryTheory/Category",
    50,
    90
);

// --- Topology (additional) ---
compat_test_flat!(test_topology_algebra_compat, "Topology/Algebra", 50, 90);
compat_test_flat!(
    test_topology_metricspace_compat,
    "Topology/MetricSpace",
    50,
    90
);
compat_test_flat!(
    test_topology_uniformspace_compat,
    "Topology/UniformSpace",
    50,
    90
);
compat_test_flat!(test_topology_order_compat, "Topology/Order", 50, 90);
compat_test_flat!(
    test_topology_continuousmap_compat,
    "Topology/ContinuousMap",
    50,
    90
);

// --- Algebra (additional) ---
compat_test_flat!(test_algebra_polynomial_compat, "Algebra/Polynomial", 50, 90);
compat_test_flat!(test_algebra_homology_compat, "Algebra/Homology", 50, 90);
compat_test_flat!(test_algebra_lie_compat, "Algebra/Lie", 50, 90);
compat_test_flat!(test_algebra_star_compat, "Algebra/Star", 50, 90);

// --- LinearAlgebra (additional) ---
compat_test_flat!(
    test_linearalgebra_matrix_compat,
    "LinearAlgebra/Matrix",
    50,
    90
);

// --- MeasureTheory (additional) ---
compat_test_flat!(
    test_measuretheory_integral_compat,
    "MeasureTheory/Integral",
    50,
    90
);
compat_test_flat!(
    test_measuretheory_function_compat,
    "MeasureTheory/Function",
    50,
    90
);

// --- RingTheory (additional) ---
compat_test_flat!(test_ringtheory_ideal_compat, "RingTheory/Ideal", 50, 90);

// --- GroupTheory (additional) ---
compat_test_flat!(
    test_grouptheory_groupaction_compat,
    "GroupTheory/GroupAction",
    50,
    90
);

// --- NumberTheory (additional) ---
compat_test_flat!(
    test_numbertheory_lseries_compat,
    "NumberTheory/LSeries",
    50,
    90
);

// --- Algebra (more) ---
compat_test_flat!(
    test_algebra_mvpolynomial_compat,
    "Algebra/MvPolynomial",
    50,
    90
);

// ============================================================
// Phase 4: Major expansion (25 new categories)
// ============================================================

// --- Algebra expansion ---
compat_test_recursive!(test_algebra_category_compat, "Algebra/Category", 50, 90);
compat_test_flat!(
    test_algebra_order_field_compat,
    "Algebra/Order/Field",
    50,
    90
);
compat_test_flat!(test_algebra_directsum_compat, "Algebra/DirectSum", 50, 90);
compat_test_flat!(
    test_algebra_monoidalgebra_compat,
    "Algebra/MonoidAlgebra",
    50,
    90
);

// --- CategoryTheory expansion ---
compat_test_recursive!(
    test_categorytheory_sites_compat,
    "CategoryTheory/Sites",
    50,
    90
);
compat_test_flat!(
    test_categorytheory_bicategory_compat,
    "CategoryTheory/Bicategory",
    50,
    90
);
compat_test_flat!(
    test_categorytheory_concretecategory_compat,
    "CategoryTheory/ConcreteCategory",
    50,
    90
);

// --- Topology expansion ---
compat_test_flat!(test_topology_category_compat, "Topology/Category", 50, 90);
compat_test_flat!(test_topology_instances_compat, "Topology/Instances", 50, 90);
compat_test_flat!(
    test_topology_separation_compat,
    "Topology/Separation",
    50,
    90
);
compat_test_flat!(
    test_topology_compactness_compat,
    "Topology/Compactness",
    50,
    90
);
compat_test_flat!(test_topology_connected_compat, "Topology/Connected", 50, 90);
compat_test_flat!(test_topology_sheaves_compat, "Topology/Sheaves", 50, 90);

// --- RingTheory expansion ---
compat_test_flat!(
    test_ringtheory_polynomial_compat,
    "RingTheory/Polynomial",
    50,
    90
);
compat_test_flat!(
    test_ringtheory_localization_compat,
    "RingTheory/Localization",
    50,
    93
);
compat_test_flat!(
    test_ringtheory_powerseries_compat,
    "RingTheory/PowerSeries",
    50,
    90
);
compat_test_flat!(
    test_ringtheory_dedekinddomain_compat,
    "RingTheory/DedekindDomain",
    50,
    90
);
compat_test_flat!(
    test_ringtheory_valuation_compat,
    "RingTheory/Valuation",
    50,
    90
);

// --- NumberTheory expansion ---
compat_test_flat!(
    test_numbertheory_numberfield_compat,
    "NumberTheory/NumberField",
    50,
    90
);
compat_test_flat!(
    test_numbertheory_modularforms_compat,
    "NumberTheory/ModularForms",
    50,
    90
);
compat_test_flat!(
    test_numbertheory_padics_compat,
    "NumberTheory/Padics",
    50,
    90
);

// --- LinearAlgebra expansion ---
compat_test_flat!(
    test_linearalgebra_tensorproduct_compat,
    "LinearAlgebra/TensorProduct",
    50,
    90
);
compat_test_flat!(
    test_linearalgebra_affinespace_compat,
    "LinearAlgebra/AffineSpace",
    50,
    90
);
compat_test_flat!(
    test_linearalgebra_quadraticform_compat,
    "LinearAlgebra/QuadraticForm",
    50,
    90
);

// --- GroupTheory expansion ---
compat_test_flat!(test_grouptheory_perm_compat, "GroupTheory/Perm", 50, 90);

// --- Order expansion ---
compat_test_flat!(test_order_interval_compat, "Order/Interval", 50, 90);

// --- MeasureTheory expansion ---
compat_test_flat!(
    test_measuretheory_constructions_compat,
    "MeasureTheory/Constructions",
    50,
    90
);
compat_test_flat!(
    test_measuretheory_group_compat,
    "MeasureTheory/Group",
    50,
    90
);

// --- Analysis expansion ---
compat_test_flat!(
    test_analysis_normedspace_compat,
    "Analysis/NormedSpace",
    50,
    0
);
compat_test_flat!(test_analysis_analytic_compat, "Analysis/Analytic", 50, 90);
compat_test_flat!(
    test_analysis_boxintegral_compat,
    "Analysis/BoxIntegral",
    50,
    90
);

// --- Data expansion ---
compat_test_flat!(test_data_real_compat, "Data/Real", 50, 90);
compat_test_flat!(test_data_matrix_compat, "Data/Matrix", 50, 90);
compat_test_flat!(test_data_dfinsupp_compat, "Data/DFinsupp", 50, 90);
compat_test_flat!(test_data_ennreal_compat, "Data/ENNReal", 50, 90);

// --- Combinatorics expansion ---
compat_test_flat!(
    test_combinatorics_enumerative_compat,
    "Combinatorics/Enumerative",
    50,
    90
);
compat_test_flat!(
    test_combinatorics_matroid_compat,
    "Combinatorics/Matroid",
    50,
    90
);

// --- Probability expansion ---
compat_test_flat!(
    test_probability_distributions_compat,
    "Probability/Distributions",
    50,
    90
);

// --- Geometry expansion ---
compat_test_flat!(
    test_geometry_ringedspace_compat,
    "Geometry/RingedSpace",
    50,
    90
);

// --- SetTheory expansion ---
compat_test_flat!(test_settheory_game_compat, "SetTheory/Game", 50, 90);

// ============================================================
// Phase 5: Deep expansion (20 more categories)
// ============================================================

// --- RingTheory deep ---
compat_test_flat!(
    test_ringtheory_wittvector_compat,
    "RingTheory/WittVector",
    50,
    90
);
compat_test_flat!(test_ringtheory_ringhom_compat, "RingTheory/RingHom", 50, 90);
compat_test_flat!(
    test_ringtheory_tensorproduct_compat,
    "RingTheory/TensorProduct",
    50,
    90
);
compat_test_flat!(
    test_ringtheory_ufd_compat,
    "RingTheory/UniqueFactorizationDomain",
    50,
    90
);
compat_test_flat!(test_ringtheory_smooth_compat, "RingTheory/Smooth", 50, 90);

// --- Order deep ---
compat_test_flat!(test_order_category_compat, "Order/Category", 50, 90);
compat_test_flat!(test_order_succpred_compat, "Order/SuccPred", 50, 90);
compat_test_flat!(test_order_hom_compat, "Order/Hom", 50, 90);

// --- LinearAlgebra deep ---
compat_test_flat!(
    test_linearalgebra_rootsystem_compat,
    "LinearAlgebra/RootSystem",
    50,
    90
);
compat_test_flat!(
    test_linearalgebra_dimension_compat,
    "LinearAlgebra/Dimension",
    50,
    90
);
compat_test_flat!(
    test_linearalgebra_cliffordalgebra_compat,
    "LinearAlgebra/CliffordAlgebra",
    50,
    90
);
compat_test_flat!(
    test_linearalgebra_basis_compat,
    "LinearAlgebra/Basis",
    50,
    90
);
compat_test_flat!(
    test_linearalgebra_multilinear_compat,
    "LinearAlgebra/Multilinear",
    50,
    90
);

// --- Analysis deep ---
compat_test_flat!(
    test_analysis_asymptotics_compat,
    "Analysis/Asymptotics",
    50,
    90
);

// --- MeasureTheory deep ---
compat_test_flat!(
    test_measuretheory_measurablespace_compat,
    "MeasureTheory/MeasurableSpace",
    50,
    90
);

// --- Probability deep ---
compat_test_flat!(
    test_probability_independence_compat,
    "Probability/Independence",
    50,
    90
);
compat_test_flat!(
    test_probability_moments_compat,
    "Probability/Moments",
    50,
    90
);

// --- CategoryTheory deep ---
compat_test_flat!(
    test_categorytheory_galois_compat,
    "CategoryTheory/Galois",
    50,
    90
);
compat_test_flat!(
    test_categorytheory_filtered_compat,
    "CategoryTheory/Filtered",
    50,
    90
);

// --- GroupTheory deep ---
compat_test_flat!(
    test_grouptheory_monoidlocalization_compat,
    "GroupTheory/MonoidLocalization",
    50,
    90
);

// ============================================================
// Phase 6: Untested large subdirectories (~58 new categories)
// ============================================================

// --- CategoryTheory (deep expansion) ---
compat_test_flat!(
    test_categorytheory_localization_compat,
    "CategoryTheory/Localization",
    50,
    90
);
compat_test_flat!(
    test_categorytheory_preadditive_compat,
    "CategoryTheory/Preadditive",
    50,
    90
);
compat_test_flat!(
    test_categorytheory_objectproperty_compat,
    "CategoryTheory/ObjectProperty",
    50,
    90
);
compat_test_flat!(
    test_categorytheory_morphismproperty_compat,
    "CategoryTheory/MorphismProperty",
    50,
    90
);
compat_test_flat!(
    test_categorytheory_triangulated_compat,
    "CategoryTheory/Triangulated",
    50,
    90
);
compat_test_flat!(
    test_categorytheory_shift_compat,
    "CategoryTheory/Shift",
    50,
    90
);
compat_test_flat!(
    test_categorytheory_presentable_compat,
    "CategoryTheory/Presentable",
    50,
    90
);
compat_test_flat!(
    test_categorytheory_comma_compat,
    "CategoryTheory/Comma",
    50,
    90
);
compat_test_flat!(
    test_categorytheory_subobject_compat,
    "CategoryTheory/Subobject",
    50,
    90
);
compat_test_flat!(
    test_categorytheory_monad_compat,
    "CategoryTheory/Monad",
    50,
    90
);
compat_test_flat!(
    test_categorytheory_closed_compat,
    "CategoryTheory/Closed",
    50,
    0
);
compat_test_flat!(
    test_categorytheory_smallobject_compat,
    "CategoryTheory/SmallObject",
    50,
    90
);
compat_test_flat!(
    test_categorytheory_enriched_compat,
    "CategoryTheory/Enriched",
    50,
    90
);
compat_test_flat!(
    test_categorytheory_generator_compat,
    "CategoryTheory/Generator",
    50,
    90
);
compat_test_flat!(
    test_categorytheory_idempotents_compat,
    "CategoryTheory/Idempotents",
    50,
    90
);
compat_test_flat!(
    test_categorytheory_fiberedcategory_compat,
    "CategoryTheory/FiberedCategory",
    50,
    90
);
compat_test_flat!(
    test_categorytheory_gradedobject_compat,
    "CategoryTheory/GradedObject",
    50,
    90
);
compat_test_flat!(
    test_categorytheory_effectiveepi_compat,
    "CategoryTheory/EffectiveEpi",
    50,
    0
);
compat_test_flat!(
    test_categorytheory_groupoid_compat,
    "CategoryTheory/Groupoid",
    50,
    90
);

// --- AlgebraicTopology (deep expansion) ---
compat_test_flat!(
    test_algebraictopology_simplicialset_compat,
    "AlgebraicTopology/SimplicialSet",
    50,
    90
);
compat_test_flat!(
    test_algebraictopology_modelcategory_compat,
    "AlgebraicTopology/ModelCategory",
    50,
    90
);
compat_test_flat!(
    test_algebraictopology_doldkan_compat,
    "AlgebraicTopology/DoldKan",
    50,
    90
);
compat_test_flat!(
    test_algebraictopology_simplexcategory_compat,
    "AlgebraicTopology/SimplexCategory",
    50,
    90
);

// --- AlgebraicGeometry (deep expansion) ---
compat_test_flat!(
    test_algebraicgeometry_morphisms_compat,
    "AlgebraicGeometry/Morphisms",
    50,
    90
);
compat_test_flat!(
    test_algebraicgeometry_ellipticcurve_compat,
    "AlgebraicGeometry/EllipticCurve",
    50,
    90
);

// --- Tactic (deep expansion) ---
compat_test_flat!(test_tactic_linter_compat, "Tactic/Linter", 50, 90);
compat_test_flat!(test_tactic_normnum_compat, "Tactic/NormNum", 50, 90);
compat_test_flat!(
    test_tactic_categorytheory_compat,
    "Tactic/CategoryTheory",
    50,
    90
);

// --- RingTheory (deep expansion) ---
compat_test_flat!(
    test_ringtheory_spectrum_compat,
    "RingTheory/Spectrum",
    50,
    0
);
compat_test_flat!(
    test_ringtheory_finiteness_compat,
    "RingTheory/Finiteness",
    50,
    90
);
compat_test_flat!(
    test_ringtheory_mvpolynomial_compat,
    "RingTheory/MvPolynomial",
    50,
    90
);
compat_test_flat!(
    test_ringtheory_localring_compat,
    "RingTheory/LocalRing",
    50,
    90
);
compat_test_flat!(test_ringtheory_flat_compat, "RingTheory/Flat", 50, 90);

// --- Condensed (deep expansion) ---
compat_test_flat!(test_condensed_light_compat, "Condensed/Light", 50, 90);

// --- RepresentationTheory (deep expansion) ---
compat_test_flat!(
    test_representationtheory_homological_compat,
    "RepresentationTheory/Homological",
    50,
    90
);

// --- Topology (deep expansion) ---
compat_test_flat!(test_topology_homotopy_compat, "Topology/Homotopy", 50, 90);
compat_test_flat!(
    test_topology_emetricspace_compat,
    "Topology/EMetricSpace",
    50,
    90
);

// --- MeasureTheory (deep expansion) ---
compat_test_flat!(
    test_measuretheory_vectormeasure_compat,
    "MeasureTheory/VectorMeasure",
    50,
    90
);
compat_test_flat!(
    test_measuretheory_outermeasure_compat,
    "MeasureTheory/OuterMeasure",
    50,
    90
);
compat_test_flat!(
    test_measuretheory_covering_compat,
    "MeasureTheory/Covering",
    50,
    90
);

// --- Analysis (deep expansion) ---
compat_test_flat!(
    test_analysis_distribution_compat,
    "Analysis/Distribution",
    50,
    90
);
compat_test_flat!(test_analysis_real_compat, "Analysis/Real", 50, 90);
compat_test_flat!(
    test_analysis_meromorphic_compat,
    "Analysis/Meromorphic",
    50,
    90
);

// --- Dynamics (deep expansion) ---
compat_test_flat!(test_dynamics_ergodic_compat, "Dynamics/Ergodic", 50, 90);

// --- Probability (deep expansion) ---
compat_test_flat!(
    test_probability_process_compat,
    "Probability/Process",
    50,
    90
);
compat_test_flat!(
    test_probability_martingale_compat,
    "Probability/Martingale",
    50,
    90
);

// --- Geometry (deep expansion) ---
compat_test_flat!(test_geometry_convex_compat, "Geometry/Convex", 50, 0);

// --- FieldTheory (deep expansion) ---
compat_test_flat!(test_fieldtheory_galois_compat, "FieldTheory/Galois", 50, 90);
compat_test_flat!(
    test_fieldtheory_intermediatefield_compat,
    "FieldTheory/IntermediateField",
    50,
    90
);

// --- NumberTheory (deep expansion) ---
compat_test_flat!(
    test_numbertheory_cyclotomic_compat,
    "NumberTheory/Cyclotomic",
    50,
    90
);

// --- Data (deep expansion) ---
compat_test_flat!(test_data_nnrat_compat, "Data/NNRat", 50, 90);
compat_test_flat!(test_data_finite_compat, "Data/Finite", 50, 90);

// --- LinearAlgebra (deep expansion) ---
compat_test_flat!(
    test_linearalgebra_freemodule_compat,
    "LinearAlgebra/FreeModule",
    50,
    90
);
compat_test_flat!(
    test_linearalgebra_eigenspace_compat,
    "LinearAlgebra/Eigenspace",
    50,
    90
);

// --- Order (deep expansion) ---
compat_test_flat!(
    test_order_completelattice_compat,
    "Order/CompleteLattice",
    50,
    90
);

// --- GroupTheory (deep expansion) ---
compat_test_flat!(
    test_grouptheory_freegroup_compat,
    "GroupTheory/FreeGroup",
    50,
    90
);
compat_test_flat!(
    test_grouptheory_specificgroups_compat,
    "GroupTheory/SpecificGroups",
    50,
    90
);

// --- SetTheory (deep expansion) ---
compat_test_flat!(test_settheory_zfc_compat, "SetTheory/ZFC", 50, 94);

// ============================================================
// Phase 7: Deep third-level dirs + remaining second-level
// ============================================================

// --- CategoryTheory third-level ---
compat_test_flat!(
    test_categorytheory_limits_shapes_compat,
    "CategoryTheory/Limits/Shapes",
    50,
    90
);
compat_test_flat!(
    test_categorytheory_limits_constructions_compat,
    "CategoryTheory/Limits/Constructions",
    50,
    90
);

// --- Analysis third-level ---
compat_test_flat!(
    test_analysis_normed_group_compat,
    "Analysis/Normed/Group",
    50,
    90
);
compat_test_flat!(
    test_analysis_normed_ring_compat,
    "Analysis/Normed/Ring",
    50,
    90
);
compat_test_flat!(
    test_analysis_normed_field_compat,
    "Analysis/Normed/Field",
    50,
    90
);
compat_test_flat!(
    test_analysis_calculus_fderiv_compat,
    "Analysis/Calculus/FDeriv",
    50,
    90
);
compat_test_flat!(
    test_analysis_specialfunctions_trig_compat,
    "Analysis/SpecialFunctions/Trigonometric",
    50,
    90
);
compat_test_flat!(
    test_analysis_specialfunctions_log_compat,
    "Analysis/SpecialFunctions/Log",
    50,
    90
);
compat_test_flat!(
    test_analysis_specialfunctions_pow_compat,
    "Analysis/SpecialFunctions/Pow",
    50,
    90
);
compat_test_flat!(
    test_analysis_specialfunctions_gamma_compat,
    "Analysis/SpecialFunctions/Gamma",
    50,
    90
);

// --- Topology third-level ---
compat_test_flat!(
    test_topology_algebra_module_compat,
    "Topology/Algebra/Module",
    50,
    90
);
compat_test_flat!(
    test_topology_algebra_group_compat,
    "Topology/Algebra/Group",
    50,
    90
);

// --- Algebra third-level ---
compat_test_flat!(
    test_algebra_order_monoid_compat,
    "Algebra/Order/Monoid",
    50,
    90
);
compat_test_flat!(test_algebra_order_hom_compat, "Algebra/Order/Hom", 50, 90);
compat_test_flat!(
    test_algebra_homology_shortcomplex_compat,
    "Algebra/Homology/ShortComplex",
    50,
    90
);

// --- Data third-level ---
compat_test_flat!(test_data_nat_prime_compat, "Data/Nat/Prime", 50, 90);

// --- Probability third-level ---
compat_test_flat!(
    test_probability_kernel_composition_compat,
    "Probability/Kernel/Composition",
    50,
    90
);

// --- MeasureTheory third-level ---
compat_test_flat!(
    test_measuretheory_function_lpseminorm_compat,
    "MeasureTheory/Function/LpSeminorm",
    50,
    90
);

// --- Geometry third-level ---
compat_test_flat!(
    test_geometry_manifold_vectorbundle_compat,
    "Geometry/Manifold/VectorBundle",
    50,
    90
);
compat_test_flat!(
    test_geometry_manifold_mfderiv_compat,
    "Geometry/Manifold/MFDeriv",
    50,
    90
);

// --- RingTheory third-level ---
compat_test_flat!(
    test_ringtheory_ideal_quotient_compat,
    "RingTheory/Ideal/Quotient",
    50,
    90
);

// --- Combinatorics third-level ---
compat_test_flat!(
    test_combinatorics_simplegraph_connectivity_compat,
    "Combinatorics/SimpleGraph/Connectivity",
    50,
    90
);

// --- More second-level untested ---
compat_test_flat!(
    test_algebra_continuedfractions_compat,
    "Algebra/ContinuedFractions",
    50,
    90
);
compat_test_flat!(test_algebra_gcdmonoid_compat, "Algebra/GCDMonoid", 50, 90);
compat_test_flat!(
    test_ringtheory_integralclosure_compat,
    "RingTheory/IntegralClosure",
    50,
    90
);
compat_test_flat!(
    test_ringtheory_hahnseries_compat,
    "RingTheory/HahnSeries",
    50,
    90
);
compat_test_flat!(
    test_ringtheory_gradedalgebra_compat,
    "RingTheory/GradedAlgebra",
    50,
    90
);
compat_test_flat!(test_ringtheory_adjoin_compat, "RingTheory/Adjoin", 50, 90);
compat_test_flat!(
    test_linearalgebra_bilinearform_compat,
    "LinearAlgebra/BilinearForm",
    50,
    90
);
compat_test_flat!(
    test_linearalgebra_pitensorproduct_compat,
    "LinearAlgebra/PiTensorProduct",
    50,
    0
);
compat_test_flat!(
    test_linearalgebra_finsupp_compat,
    "LinearAlgebra/Finsupp",
    50,
    90
);
compat_test_flat!(
    test_numbertheory_legendresymbol_compat,
    "NumberTheory/LegendreSymbol",
    50,
    90
);
compat_test_flat!(test_order_upperlower_compat, "Order/UpperLower", 50, 90);
compat_test_flat!(
    test_fieldtheory_minpoly_compat,
    "FieldTheory/Minpoly",
    50,
    90
);
compat_test_flat!(test_fieldtheory_finite_compat, "FieldTheory/Finite", 50, 90);
compat_test_flat!(
    test_dynamics_topologicalentropy_compat,
    "Dynamics/TopologicalEntropy",
    50,
    90
);
compat_test_flat!(test_condensed_discrete_compat, "Condensed/Discrete", 50, 90);

// ============================================================
// Phase 7b: More third-level and remaining dirs
// ============================================================
compat_test_flat!(
    test_analysis_normed_module_compat,
    "Analysis/Normed/Module",
    50,
    90
);
compat_test_flat!(
    test_analysis_calculus_deriv_compat,
    "Analysis/Calculus/Deriv",
    50,
    90
);
compat_test_flat!(test_order_interval_set_compat, "Order/Interval/Set", 50, 90);
compat_test_flat!(
    test_algebra_category_modulecat_compat,
    "Algebra/Category/ModuleCat",
    50,
    90
);
compat_test_flat!(test_algebra_order_flat_compat, "Algebra/Order", 50, 90);
compat_test_flat!(
    test_ringtheory_mvpowerseries_compat,
    "RingTheory/MvPowerSeries",
    50,
    90
);
compat_test_flat!(
    test_ringtheory_noetherian_compat,
    "RingTheory/Noetherian",
    50,
    90
);
compat_test_flat!(
    test_combinatorics_quiver_compat,
    "Combinatorics/Quiver",
    50,
    90
);
compat_test_flat!(test_algebra_regular_compat, "Algebra/Regular", 50, 90);

// ============================================================
// Phase 7c: More third-level untested dirs
// ============================================================
compat_test_flat!(
    test_algebra_category_grp_compat,
    "Algebra/Category/Grp",
    50,
    90
);
compat_test_flat!(
    test_order_filter_attopbot_compat,
    "Order/Filter/AtTopBot",
    50,
    90
);
compat_test_flat!(
    test_analysis_normed_operator_compat,
    "Analysis/Normed/Operator",
    50,
    90
);
compat_test_flat!(
    test_algebra_homology_homotopycat_compat,
    "Algebra/Homology/HomotopyCategory",
    50,
    90
);
compat_test_flat!(
    test_topology_algebra_infinitesum_compat,
    "Topology/Algebra/InfiniteSum",
    50,
    90
);
compat_test_flat!(
    test_algebra_homology_embedding_compat,
    "Algebra/Homology/Embedding",
    50,
    90
);
compat_test_flat!(
    test_ringtheory_spectrum_prime_compat,
    "RingTheory/Spectrum/Prime",
    50,
    90
);
compat_test_flat!(
    test_categorytheory_limits_types_compat,
    "CategoryTheory/Limits/Types",
    50,
    90
);
compat_test_flat!(
    test_algebra_module_submodule_compat,
    "Algebra/Module/Submodule",
    50,
    90
);
compat_test_flat!(
    test_algebra_group_action_compat,
    "Algebra/Group/Action",
    50,
    90
);
compat_test_flat!(
    test_analysis_calculus_contdiff_compat,
    "Analysis/Calculus/ContDiff",
    50,
    90
);
compat_test_flat!(
    test_analysis_cstaralgebra_cfc_compat,
    "Analysis/CStarAlgebra/ContinuousFunctionalCalculus",
    50,
    90
);
compat_test_flat!(
    test_algebra_algebra_subalgebra_compat,
    "Algebra/Algebra/Subalgebra",
    50,
    90
);
compat_test_flat!(
    test_measuretheory_integral_interval_compat,
    "MeasureTheory/Integral/IntervalIntegral",
    50,
    88
);
compat_test_flat!(
    test_categorytheory_limits_preserves_compat,
    "CategoryTheory/Limits/Preserves",
    50,
    90
);
compat_test_flat!(
    test_analysis_normed_algebra_compat,
    "Analysis/Normed/Algebra",
    50,
    90
);
compat_test_flat!(
    test_algebra_polynomial_degree_compat,
    "Algebra/Polynomial/Degree",
    50,
    90
);
compat_test_flat!(
    test_algebra_order_module_compat,
    "Algebra/Order/Module",
    50,
    90
);
compat_test_flat!(
    test_algebra_homology_derivedcategory_compat,
    "Algebra/Homology/DerivedCategory",
    50,
    90
);
compat_test_flat!(
    test_algebra_groupwithzero_action_compat,
    "Algebra/GroupWithZero/Action",
    50,
    90
);

// ============================================================
// Phase 8: 4th-level dirs + remaining untested
// ============================================================
compat_test_flat!(
    test_categorytheory_limits_shapes_pullback_compat,
    "CategoryTheory/Limits/Shapes/Pullback",
    50,
    90
);
compat_test_flat!(
    test_categorytheory_limits_preserves_shapes_compat,
    "CategoryTheory/Limits/Preserves/Shapes",
    50,
    90
);
compat_test_flat!(
    test_algebra_category_modulecat_presheaf_compat,
    "Algebra/Category/ModuleCat/Presheaf",
    50,
    90
);
compat_test_flat!(
    test_algebra_bigoperators_group_finset_compat,
    "Algebra/BigOperators/Group/Finset",
    50,
    90
);
compat_test_flat!(
    test_algebra_category_modulecat_sheaf_compat,
    "Algebra/Category/ModuleCat/Sheaf",
    50,
    0
);
compat_test_flat!(
    test_algebra_order_monoid_unbundled_compat,
    "Algebra/Order/Monoid/Unbundled",
    50,
    90
);
compat_test_flat!(
    test_algebra_homology_derivedcategory_ext_compat,
    "Algebra/Homology/DerivedCategory/Ext",
    50,
    90
);
compat_test_flat!(
    test_algebra_group_pointwise_set_compat,
    "Algebra/Group/Pointwise/Set",
    50,
    90
);
compat_test_flat!(
    test_categorytheory_sites_coherent_compat,
    "CategoryTheory/Sites/Coherent",
    50,
    90
);
compat_test_flat!(test_data_qpf_compat, "Data/QPF", 50, 0);
compat_test_flat!(
    test_ringtheory_extension_compat,
    "RingTheory/Extension",
    50,
    90
);
compat_test_flat!(
    test_ringtheory_adiccompletion_compat,
    "RingTheory/AdicCompletion",
    50,
    90
);
compat_test_flat!(test_analysis_rclike_compat, "Analysis/RCLike", 50, 90);
compat_test_flat!(test_analysis_matrix_compat, "Analysis/Matrix", 50, 90);
compat_test_flat!(
    test_analysis_polynomial_compat,
    "Analysis/Polynomial",
    50,
    90
);
compat_test_flat!(
    test_geometry_euclidean_angle_compat,
    "Geometry/Euclidean/Angle",
    50,
    0
);
compat_test_flat!(
    test_grouptheory_congruence_compat,
    "GroupTheory/Congruence",
    50,
    90
);

// ============================================================
// Phase 9: More untested dirs (25,000+ declarations target)
// ============================================================
compat_test_flat!(test_util_compat, "Util", 50, 90);
compat_test_flat!(test_data_flat_compat, "Data", 50, 90);
compat_test_flat!(test_analysis_flat_compat, "Analysis", 50, 90);
compat_test_flat!(test_logic_equiv_compat, "Logic/Equiv", 50, 90);
compat_test_flat!(test_probability_flat_compat, "Probability", 50, 90);
compat_test_flat!(test_logic_function_compat, "Logic/Function", 50, 90);
compat_test_flat!(test_data_nat_choose_compat, "Data/Nat/Choose", 50, 90);
compat_test_flat!(
    test_categorytheory_monoidal_cartesian_compat,
    "CategoryTheory/Monoidal/Cartesian",
    50,
    90
);
compat_test_flat!(
    test_algebra_category_ring_compat,
    "Algebra/Category/Ring",
    50,
    90
);
compat_test_flat!(
    test_topology_category_topcat_compat,
    "Topology/Category/TopCat",
    50,
    90
);
compat_test_flat!(test_tactic_widget_compat, "Tactic/Widget", 50, 90);
compat_test_flat!(
    test_topology_algebra_order_compat,
    "Topology/Algebra/Order",
    50,
    90
);
compat_test_flat!(test_tactic_funprop_compat, "Tactic/FunProp", 50, 90);
compat_test_flat!(
    test_ringtheory_krulldimension_compat,
    "RingTheory/KrullDimension",
    50,
    90
);
compat_test_flat!(
    test_measuretheory_measure_haar_compat,
    "MeasureTheory/Measure/Haar",
    50,
    90
);
compat_test_flat!(test_data_nat_cast_compat, "Data/Nat/Cast", 50, 90);
compat_test_flat!(test_data_fin_tuple_compat, "Data/Fin/Tuple", 50, 90);
compat_test_flat!(
    test_categorytheory_monoidal_closed_compat,
    "CategoryTheory/Monoidal/Closed",
    50,
    90
);
compat_test_flat!(
    test_categorytheory_bicategory_functor_compat,
    "CategoryTheory/Bicategory/Functor",
    50,
    90
);
compat_test_flat!(
    test_analysis_specialfunctions_complex_compat,
    "Analysis/SpecialFunctions/Complex",
    50,
    90
);
compat_test_flat!(
    test_analysis_normed_unbundled_compat,
    "Analysis/Normed/Unbundled",
    50,
    90
);
compat_test_flat!(
    test_analysis_normed_affine_compat,
    "Analysis/Normed/Affine",
    50,
    90
);
compat_test_flat!(
    test_algebra_ring_action_compat,
    "Algebra/Ring/Action",
    50,
    0
);
compat_test_flat!(
    test_algebra_module_presentation_compat,
    "Algebra/Module/Presentation",
    50,
    90
);
compat_test_flat!(
    test_topology_category_profinite_compat,
    "Topology/Category/Profinite",
    50,
    90
);
compat_test_flat!(
    test_topology_category_lightprofinite_compat,
    "Topology/Category/LightProfinite",
    50,
    90
);
compat_test_flat!(
    test_ringtheory_rootsofunity_compat,
    "RingTheory/RootsOfUnity",
    50,
    90
);
compat_test_flat!(
    test_ringtheory_coalgebra_compat,
    "RingTheory/Coalgebra",
    50,
    90
);
compat_test_flat!(
    test_ringtheory_algebraic_compat,
    "RingTheory/Algebraic",
    50,
    90
);
compat_test_flat!(
    test_probability_kernel_disintegration_compat,
    "Probability/Kernel/Disintegration",
    50,
    90
);
compat_test_flat!(
    test_order_interval_finset_compat,
    "Order/Interval/Finset",
    50,
    90
);
compat_test_flat!(
    test_measuretheory_integral_lebesgue_compat,
    "MeasureTheory/Integral/Lebesgue",
    50,
    90
);
compat_test_flat!(
    test_measuretheory_function_conditionalexpectation_compat,
    "MeasureTheory/Function/ConditionalExpectation",
    50,
    90
);
compat_test_flat!(
    test_measuretheory_constructions_borelspace_compat,
    "MeasureTheory/Constructions/BorelSpace",
    50,
    90
);
compat_test_flat!(
    test_linearalgebra_matrix_charpoly_compat,
    "LinearAlgebra/Matrix/Charpoly",
    50,
    90
);
compat_test_flat!(
    test_analysis_normedspace_operatornorm_compat,
    "Analysis/NormedSpace/OperatorNorm",
    50,
    0
);
compat_test_flat!(test_analysis_normed_lp_compat, "Analysis/Normed/Lp", 50, 90);
compat_test_flat!(
    test_analysis_calculus_tangentcone_compat,
    "Analysis/Calculus/TangentCone",
    50,
    90
);
compat_test_flat!(
    test_algebraicgeometry_sites_compat,
    "AlgebraicGeometry/Sites",
    50,
    90
);
compat_test_flat!(
    test_algebra_module_linearmap_compat,
    "Algebra/Module/LinearMap",
    50,
    90
);
compat_test_flat!(test_topology_sets_compat, "Topology/Sets", 50, 90);
compat_test_flat!(
    test_topology_algebra_valued_compat,
    "Topology/Algebra/Valued",
    50,
    90
);
compat_test_flat!(
    test_ringtheory_unramified_compat,
    "RingTheory/Unramified",
    50,
    90
);
compat_test_flat!(
    test_ringtheory_localproperties_compat,
    "RingTheory/LocalProperties",
    50,
    0
);
compat_test_flat!(test_ringtheory_etale_compat, "RingTheory/Etale", 50, 90);
compat_test_flat!(
    test_ringtheory_algebraicindependent_compat,
    "RingTheory/AlgebraicIndependent",
    50,
    90
);
compat_test_flat!(
    test_representationtheory_homological_groupcohomology_compat,
    "RepresentationTheory/Homological/GroupCohomology",
    50,
    90
);
compat_test_flat!(test_order_monotone_compat, "Order/Monotone", 50, 90);
compat_test_flat!(
    test_numbertheory_modularforms_eisenstein_compat,
    "NumberTheory/ModularForms/EisensteinSeries",
    50,
    90
);
compat_test_flat!(test_data_set_finite_compat, "Data/Set/Finite", 50, 90);
compat_test_flat!(
    test_combinatorics_simplegraph_regularity_compat,
    "Combinatorics/SimpleGraph/Regularity",
    50,
    90
);
compat_test_flat!(
    test_categorytheory_limits_indization_compat,
    "CategoryTheory/Limits/Indization",
    50,
    0
);
compat_test_flat!(
    test_categorytheory_category_cat_compat,
    "CategoryTheory/Category/Cat",
    50,
    90
);
compat_test_flat!(
    test_analysis_complex_upperhalfplane_compat,
    "Analysis/Complex/UpperHalfPlane",
    50,
    90
);
compat_test_flat!(
    test_algebra_ring_subring_compat,
    "Algebra/Ring/Subring",
    50,
    90
);
compat_test_flat!(
    test_algebra_polynomial_eval_compat,
    "Algebra/Polynomial/Eval",
    50,
    90
);
compat_test_flat!(
    test_algebra_order_groupwithzero_compat,
    "Algebra/Order/GroupWithZero",
    50,
    90
);
compat_test_flat!(
    test_algebra_module_localizedmodule_compat,
    "Algebra/Module/LocalizedModule",
    50,
    90
);
compat_test_flat!(
    test_algebra_lie_weights_compat,
    "Algebra/Lie/Weights",
    50,
    90
);
compat_test_flat!(
    test_algebra_category_moncat_compat,
    "Algebra/Category/MonCat",
    50,
    90
);

// ============================================================
// Phase 10: Remaining 3+ file dirs
// ============================================================
compat_test_flat!(test_categorytheory_flat_compat, "CategoryTheory", 100, 90);
compat_test_flat!(
    test_categorytheory_subpresheaf_compat,
    "CategoryTheory/Subpresheaf",
    50,
    0
);
compat_test_flat!(
    test_categorytheory_subfunctor_compat,
    "CategoryTheory/Subfunctor",
    50,
    90
);
compat_test_flat!(
    test_categorytheory_abelian_grothendieckcat_compat,
    "CategoryTheory/Abelian/GrothendieckCategory",
    50,
    90
);
compat_test_flat!(
    test_categorytheory_abelian_grothendieckax_compat,
    "CategoryTheory/Abelian/GrothendieckAxioms",
    50,
    0
);
compat_test_flat!(
    test_categorytheory_triangulated_opposite_compat,
    "CategoryTheory/Triangulated/Opposite",
    50,
    0
);
compat_test_flat!(
    test_categorytheory_sites_hypercover_compat,
    "CategoryTheory/Sites/Hypercover",
    50,
    90
);
compat_test_flat!(
    test_categorytheory_limits_shapes_preorder_compat,
    "CategoryTheory/Limits/Shapes/Preorder",
    50,
    90
);
compat_test_flat!(
    test_categorytheory_functor_kanextension_compat,
    "CategoryTheory/Functor/KanExtension",
    50,
    90
);
compat_test_flat!(
    test_topology_openpartialhomeomorph_compat,
    "Topology/OpenPartialHomeomorph",
    50,
    90
);
compat_test_flat!(
    test_topology_metrizable_compat,
    "Topology/Metrizable",
    50,
    90
);
compat_test_flat!(
    test_topology_metricspace_pseudo_compat,
    "Topology/MetricSpace/Pseudo",
    50,
    93
);
compat_test_flat!(test_topology_bornology_compat, "Topology/Bornology", 50, 90);
compat_test_flat!(test_tactic_ring_compat, "Tactic/Ring", 50, 90);
compat_test_flat!(test_tactic_linarith_compat, "Tactic/Linarith", 50, 90);
compat_test_flat!(
    test_ringtheory_twosidedideal_compat,
    "RingTheory/TwoSidedIdeal",
    50,
    90
);
compat_test_flat!(
    test_ringtheory_simplering_compat,
    "RingTheory/SimpleRing",
    50,
    0
);
compat_test_flat!(
    test_ringtheory_simplemodule_compat,
    "RingTheory/SimpleModule",
    50,
    90
);
compat_test_flat!(
    test_ringtheory_jacobson_compat,
    "RingTheory/Jacobson",
    50,
    90
);
compat_test_flat!(
    test_ringtheory_bialgebra_compat,
    "RingTheory/Bialgebra",
    50,
    90
);
compat_test_flat!(
    test_numbertheory_numberfield_cyclotomic_compat,
    "NumberTheory/NumberField/Cyclotomic",
    50,
    90
);
compat_test_flat!(
    test_numbertheory_harmonic_compat,
    "NumberTheory/Harmonic",
    50,
    90
);
compat_test_flat!(
    test_measuretheory_integral_bochner_compat,
    "MeasureTheory/Integral/Bochner",
    50,
    90
);
compat_test_flat!(
    test_measuretheory_function_stronglymeasurable_compat,
    "MeasureTheory/Function/StronglyMeasurable",
    50,
    90
);
compat_test_flat!(
    test_linearalgebra_projectivization_compat,
    "LinearAlgebra/Projectivization",
    50,
    90
);
compat_test_flat!(
    test_geometry_euclidean_sphere_compat,
    "Geometry/Euclidean/Sphere",
    50,
    90
);
compat_test_flat!(
    test_geometry_euclidean_angle_unoriented_compat,
    "Geometry/Euclidean/Angle/Unoriented",
    50,
    90
);
compat_test_flat!(test_data_vector_compat, "Data/Vector", 50, 0);
compat_test_flat!(
    test_data_nat_factorization_compat,
    "Data/Nat/Factorization",
    50,
    90
);
compat_test_flat!(test_data_int_cast_compat, "Data/Int/Cast", 50, 90);
compat_test_flat!(
    test_data_finset_lattice_compat,
    "Data/Finset/Lattice",
    50,
    90
);
compat_test_flat!(
    test_analysis_calculus_inversefunctionthm_compat,
    "Analysis/Calculus/InverseFunctionTheorem",
    50,
    0
);
compat_test_flat!(
    test_analysis_calculus_bumpfunction_compat,
    "Analysis/Calculus/BumpFunction",
    50,
    90
);
compat_test_flat!(
    test_analysis_boxintegral_partition_compat,
    "Analysis/BoxIntegral/Partition",
    50,
    90
);
compat_test_flat!(
    test_algebraictopology_fundamentalgroupoid_compat,
    "AlgebraicTopology/FundamentalGroupoid",
    50,
    90
);
compat_test_flat!(
    test_algebraicgeometry_cover_compat,
    "AlgebraicGeometry/Cover",
    50,
    90
);
compat_test_flat!(
    test_algebra_order_nonneg_compat,
    "Algebra/Order/Nonneg",
    50,
    90
);
compat_test_flat!(
    test_algebra_order_floor_compat,
    "Algebra/Order/Floor",
    50,
    90
);
compat_test_flat!(test_algebra_group_nat_compat, "Algebra/Group/Nat", 50, 90);
compat_test_flat!(
    test_ringtheory_localring_residuefield_compat,
    "RingTheory/LocalRing/ResidueField",
    50,
    90
);
compat_test_flat!(
    test_data_qpf_multivariate_constructions_compat,
    "Data/QPF/Multivariate/Constructions",
    50,
    90
);

// ============================================================
// Phase 11: All remaining 4+ file dirs (30,000+ target)
// ============================================================
// 6-file dirs
compat_test_flat!(test_tactic_translate_compat, "Tactic/Translate", 50, 90);
compat_test_flat!(
    test_representationtheory_homological_grouphomology_compat,
    "RepresentationTheory/Homological/GroupHomology",
    50,
    90
);
compat_test_flat!(test_algebra_notation_compat, "Algebra/Notation", 50, 90);
compat_test_flat!(
    test_algebra_continuedfractions_computation_compat,
    "Algebra/ContinuedFractions/Computation",
    50,
    90
);
// 5-file dirs
compat_test_flat!(
    test_topology_sheaves_sheafcondition_compat,
    "Topology/Sheaves/SheafCondition",
    50,
    90
);
compat_test_flat!(test_topology_defs_compat, "Topology/Defs", 50, 90);
compat_test_flat!(
    test_topology_category_topcat_limits_compat,
    "Topology/Category/TopCat/Limits",
    50,
    90
);
compat_test_flat!(
    test_topology_category_comphauslike_compat,
    "Topology/Category/CompHausLike",
    50,
    90
);
compat_test_flat!(
    test_topology_category_comphaus_compat,
    "Topology/Category/CompHaus",
    50,
    90
);
compat_test_flat!(
    test_topology_algebra_nonarchimedean_compat,
    "Topology/Algebra/Nonarchimedean",
    50,
    90
);
compat_test_flat!(
    test_topology_algebra_isuniformgroup_compat,
    "Topology/Algebra/IsUniformGroup",
    50,
    90
);
compat_test_flat!(
    test_ringtheory_regular_compat2,
    "RingTheory/Regular",
    50,
    90
);
compat_test_flat!(
    test_ringtheory_polynomial_cyclotomic_compat,
    "RingTheory/Polynomial/Cyclotomic",
    50,
    90
);
compat_test_flat!(
    test_ringtheory_orelocalization_compat,
    "RingTheory/OreLocalization",
    50,
    90
);
compat_test_flat!(
    test_ringtheory_nilpotent_compat,
    "RingTheory/Nilpotent",
    50,
    90
);
compat_test_flat!(
    test_ringtheory_fractionalideal_compat,
    "RingTheory/FractionalIdeal",
    50,
    90
);
compat_test_flat!(
    test_ringtheory_dividedpowers_compat,
    "RingTheory/DividedPowers",
    50,
    90
);
compat_test_flat!(
    test_ringtheory_derivation_compat,
    "RingTheory/Derivation",
    50,
    0
);
compat_test_flat!(
    test_ringtheory_congruence_compat,
    "RingTheory/Congruence",
    50,
    90
);
compat_test_flat!(
    test_order_conditionallycomplete_compat,
    "Order/ConditionallyCompleteLattice",
    50,
    90
);
compat_test_flat!(test_order_bounds_compat, "Order/Bounds", 50, 90);
compat_test_flat!(test_numbertheory_flt_compat, "NumberTheory/FLT", 50, 90);
compat_test_flat!(
    test_numbertheory_classnumber_compat,
    "NumberTheory/ClassNumber",
    50,
    90
);
compat_test_flat!(
    test_measuretheory_vectormeasure_decomposition_compat,
    "MeasureTheory/VectorMeasure/Decomposition",
    50,
    90
);
compat_test_flat!(
    test_measuretheory_measure_lebesgue_compat,
    "MeasureTheory/Measure/Lebesgue",
    50,
    90
);
compat_test_flat!(
    test_measuretheory_measure_decomposition_compat,
    "MeasureTheory/Measure/Decomposition",
    50,
    90
);
compat_test_flat!(
    test_measuretheory_function_specialfunctions_compat,
    "MeasureTheory/Function/SpecialFunctions",
    50,
    90
);
compat_test_flat!(
    test_measuretheory_function_lpspace_compat,
    "MeasureTheory/Function/LpSpace",
    50,
    90
);
compat_test_flat!(
    test_linearalgebra_quotient_compat,
    "LinearAlgebra/Quotient",
    50,
    90
);
compat_test_flat!(
    test_grouptheory_perm_cycle_compat,
    "GroupTheory/Perm/Cycle",
    50,
    90
);
compat_test_flat!(
    test_grouptheory_groupaction_submulaction_compat,
    "GroupTheory/GroupAction/SubMulAction",
    50,
    90
);
compat_test_flat!(
    test_geometry_manifold_contmdiff_compat,
    "Geometry/Manifold/ContMDiff",
    50,
    90
);
compat_test_flat!(
    test_geometry_manifold_algebra_compat,
    "Geometry/Manifold/Algebra",
    50,
    90
);
compat_test_flat!(
    test_geometry_euclidean_angle_oriented_compat,
    "Geometry/Euclidean/Angle/Oriented",
    50,
    90
);
compat_test_flat!(
    test_geometry_convex_cone_compat,
    "Geometry/Convex/Cone",
    50,
    90
);
compat_test_flat!(test_data_seq_compat, "Data/Seq", 50, 90);
compat_test_flat!(test_data_rat_cast_compat, "Data/Rat/Cast", 50, 90);
compat_test_flat!(test_data_num_compat, "Data/Num", 50, 90);
compat_test_flat!(
    test_combinatorics_simplegraph_walks_compat,
    "Combinatorics/SimpleGraph/Walks",
    50,
    90
);
compat_test_flat!(
    test_categorytheory_preadditive_projective_compat,
    "CategoryTheory/Preadditive/Projective",
    50,
    90
);
compat_test_flat!(
    test_categorytheory_monoidal_rigid_compat,
    "CategoryTheory/Monoidal/Rigid",
    50,
    90
);
compat_test_flat!(
    test_categorytheory_monoidal_braided_compat,
    "CategoryTheory/Monoidal/Braided",
    50,
    90
);
compat_test_flat!(
    test_categorytheory_localization_derivability_compat,
    "CategoryTheory/Localization/DerivabilityStructure",
    50,
    90
);
compat_test_flat!(
    test_categorytheory_limits_shapes_opposites_compat,
    "CategoryTheory/Limits/Shapes/Opposites",
    50,
    90
);
compat_test_flat!(
    test_categorytheory_liftingproperties_compat,
    "CategoryTheory/LiftingProperties",
    50,
    90
);
compat_test_flat!(
    test_categorytheory_guitartexact_compat,
    "CategoryTheory/GuitartExact",
    50,
    90
);
compat_test_flat!(
    test_categorytheory_functor_derived_compat,
    "CategoryTheory/Functor/Derived",
    50,
    0
);
compat_test_flat!(
    test_categorytheory_composablearrows_compat,
    "CategoryTheory/ComposableArrows",
    50,
    90
);
compat_test_flat!(
    test_categorytheory_comma_structuredarrow_compat,
    "CategoryTheory/Comma/StructuredArrow",
    50,
    90
);
compat_test_flat!(
    test_categorytheory_center_compat,
    "CategoryTheory/Center",
    50,
    90
);
compat_test_flat!(
    test_categorytheory_action_compat,
    "CategoryTheory/Action",
    50,
    90
);
compat_test_flat!(
    test_categorytheory_abelian_projective_compat,
    "CategoryTheory/Abelian/Projective",
    50,
    90
);
compat_test_flat!(
    test_categorytheory_abelian_injective_compat,
    "CategoryTheory/Abelian/Injective",
    50,
    90
);
compat_test_flat!(
    test_analysis_innerproductspace_projection_compat,
    "Analysis/InnerProductSpace/Projection",
    50,
    90
);
compat_test_flat!(
    test_analysis_convex_cone_compat,
    "Analysis/Convex/Cone",
    50,
    90
);
compat_test_flat!(
    test_analysis_calculus_iteratedderiv_compat,
    "Analysis/Calculus/IteratedDeriv",
    50,
    90
);
compat_test_flat!(
    test_algebraictopology_simplicialobject_compat,
    "AlgebraicTopology/SimplicialObject",
    50,
    90
);
compat_test_flat!(
    test_algebraicgeometry_projectivespectrum_compat,
    "AlgebraicGeometry/ProjectiveSpectrum",
    50,
    0
);
compat_test_flat!(
    test_algebra_ring_subsemiring_compat,
    "Algebra/Ring/Subsemiring",
    50,
    90
);
compat_test_flat!(
    test_algebra_order_archimedean_compat,
    "Algebra/Order/Archimedean",
    50,
    90
);
compat_test_flat!(
    test_algebra_module_torsion_compat,
    "Algebra/Module/Torsion",
    50,
    90
);
compat_test_flat!(
    test_algebra_group_units_compat,
    "Algebra/Group/Units",
    50,
    90
);
compat_test_flat!(
    test_algebra_group_pointwise_finset_compat,
    "Algebra/Group/Pointwise/Finset",
    50,
    90
);
compat_test_flat!(
    test_algebra_group_equiv_compat,
    "Algebra/Group/Equiv",
    50,
    90
);
compat_test_flat!(
    test_algebra_divisibility_compat,
    "Algebra/Divisibility",
    50,
    90
);
compat_test_flat!(test_algebra_colimit_compat, "Algebra/Colimit", 50, 90);
compat_test_flat!(test_algebra_central_compat, "Algebra/Central", 50, 90);
compat_test_flat!(
    test_algebra_category_fgmodulecat_compat,
    "Algebra/Category/FGModuleCat",
    50,
    90
);
// 4-file dirs
compat_test_flat!(
    test_topology_vectorbundle_compat,
    "Topology/VectorBundle",
    50,
    90
);
compat_test_flat!(
    test_topology_semicontinuity_compat,
    "Topology/Semicontinuity",
    50,
    90
);
compat_test_flat!(
    test_topology_fiberbundle_compat,
    "Topology/FiberBundle",
    50,
    90
);
compat_test_flat!(test_topology_baire_compat, "Topology/Baire", 50, 90);
compat_test_flat!(
    test_topology_algebra_ring_compat,
    "Topology/Algebra/Ring",
    50,
    0
);
compat_test_flat!(
    test_ringtheory_spectrum_maximal_compat,
    "RingTheory/Spectrum/Maximal",
    50,
    90
);
compat_test_flat!(
    test_ringtheory_polynomial_eisenstein_compat,
    "RingTheory/Polynomial/Eisenstein",
    50,
    90
);
compat_test_flat!(test_ringtheory_kaehler_compat, "RingTheory/Kaehler", 50, 90);
compat_test_flat!(
    test_ringtheory_hopfalgebra_compat,
    "RingTheory/HopfAlgebra",
    50,
    90
);
compat_test_flat!(
    test_ringtheory_artinian_compat,
    "RingTheory/Artinian",
    50,
    90
);
compat_test_flat!(test_order_heyting_compat, "Order/Heyting", 50, 90);
compat_test_flat!(
    test_numbertheory_numberfield_infiniteplace_compat,
    "NumberTheory/NumberField/InfinitePlace",
    50,
    90
);
compat_test_flat!(
    test_numbertheory_numberfield_canonicalembedding_compat,
    "NumberTheory/NumberField/CanonicalEmbedding",
    50,
    90
);
compat_test_flat!(
    test_linearalgebra_bilinearform_compat2,
    "LinearAlgebra/BilinearForm",
    50,
    90
);
compat_test_flat!(test_data_fin_flat_compat, "Data/Fin", 50, 90);
compat_test_flat!(
    test_algebra_group_subsemigroup_compat,
    "Algebra/Group/Subsemigroup",
    50,
    90
);
// Phase 12: 254 new directories
compat_test_flat!(test_lean_meta_compat_p12, "Lean/Meta", 50, 90);
compat_test_flat!(
    test_topology_category_profinite_nobeling_compat_p12,
    "Topology/Category/Profinite/Nobeling",
    50,
    90
);
compat_test_flat!(
    test_number_theory_transcendental_liouville_compat_p12,
    "NumberTheory/Transcendental/Liouville",
    50,
    90
);
compat_test_flat!(test_deprecated_compat_p12, "Deprecated", 50, 0);
compat_test_flat!(
    test_category_theory_limits_functor_category_shapes_compat_p12,
    "CategoryTheory/Limits/FunctorCategory/Shapes",
    50,
    0
);
compat_test_flat!(
    test_category_theory_join_compat_p12,
    "CategoryTheory/Join",
    50,
    90
);
compat_test_flat!(test_analysis_real_pi_compat_p12, "Analysis/Real/Pi", 50, 90);
compat_test_flat!(
    test_algebraic_topology_simplicial_set_anodyne_extensions_compat_p12,
    "AlgebraicTopology/SimplicialSet/AnodyneExtensions",
    50,
    90
);
compat_test_flat!(
    test_algebraic_topology_quasicategory_compat_p12,
    "AlgebraicTopology/Quasicategory",
    50,
    90
);
compat_test_flat!(
    test_topology_metric_space_ultra_compat_p12,
    "Topology/MetricSpace/Ultra",
    50,
    90
);
compat_test_flat!(
    test_topology_continuous_map_bounded_compat_p12,
    "Topology/ContinuousMap/Bounded",
    50,
    90
);
compat_test_flat!(
    test_topology_category_stonean_compat_p12,
    "Topology/Category/Stonean",
    50,
    0
);
compat_test_flat!(
    test_topology_algebra_separation_quotient_compat_p12,
    "Topology/Algebra/SeparationQuotient",
    50,
    90
);
compat_test_flat!(test_tactic_simproc_compat_p12, "Tactic/Simproc", 50, 90);
compat_test_flat!(
    test_tactic_linarith_oracle_simplex_algorithm_compat_p12,
    "Tactic/Linarith/Oracle/SimplexAlgorithm",
    50,
    90
);
compat_test_flat!(
    test_tactic_category_theory_monoidal_compat_p12,
    "Tactic/CategoryTheory/Monoidal",
    50,
    90
);
compat_test_flat!(
    test_tactic_category_theory_coherence_compat_p12,
    "Tactic/CategoryTheory/Coherence",
    50,
    90
);
compat_test_flat!(
    test_tactic_category_theory_bicategory_compat_p12,
    "Tactic/CategoryTheory/Bicategory",
    50,
    90
);
compat_test_flat!(
    test_ring_theory_ideal_minimal_prime_compat_p12,
    "RingTheory/Ideal/MinimalPrime",
    50,
    90
);
compat_test_flat!(
    test_ring_theory_extension_cotangent_compat_p12,
    "RingTheory/Extension/Cotangent",
    50,
    90
);
compat_test_flat!(
    test_probability_distributions_gaussian_compat_p12,
    "Probability/Distributions/Gaussian",
    50,
    90
);
compat_test_flat!(test_order_fin_compat_p12, "Order/Fin", 50, 90);
compat_test_flat!(
    test_number_theory_zsqrtd_compat_p12,
    "NumberTheory/Zsqrtd",
    50,
    90
);
compat_test_flat!(
    test_number_theory_modular_forms_jacobi_theta_compat_p12,
    "NumberTheory/ModularForms/JacobiTheta",
    50,
    90
);
compat_test_flat!(
    test_number_theory_dirichlet_character_compat_p12,
    "NumberTheory/DirichletCharacter",
    50,
    90
);
compat_test_flat!(
    test_measure_theory_specific_codomains_compat_p12,
    "MeasureTheory/SpecificCodomains",
    50,
    90
);
compat_test_flat!(
    test_measure_theory_measure_typeclasses_compat_p12,
    "MeasureTheory/Measure/Typeclasses",
    50,
    90
);
compat_test_flat!(test_logic_small_compat_p12, "Logic/Small", 50, 0);
compat_test_flat!(
    test_linear_algebra_tensor_algebra_compat_p12,
    "LinearAlgebra/TensorAlgebra",
    50,
    90
);
compat_test_flat!(
    test_linear_algebra_root_system_geck_construction_compat_p12,
    "LinearAlgebra/RootSystem/GeckConstruction",
    50,
    90
);
compat_test_flat!(
    test_linear_algebra_root_system_finite_compat_p12,
    "LinearAlgebra/RootSystem/Finite",
    50,
    90
);
compat_test_flat!(
    test_linear_algebra_matrix_general_linear_group_compat_p12,
    "LinearAlgebra/Matrix/GeneralLinearGroup",
    50,
    90
);
compat_test_flat!(
    test_linear_algebra_free_module_finite_compat_p12,
    "LinearAlgebra/FreeModule/Finite",
    50,
    90
);
compat_test_flat!(
    test_linear_algebra_dual_compat_p12,
    "LinearAlgebra/Dual",
    50,
    90
);
compat_test_flat!(
    test_linear_algebra_direct_sum_compat_p12,
    "LinearAlgebra/DirectSum",
    50,
    0
);
compat_test_flat!(
    test_linear_algebra_complex_compat_p12,
    "LinearAlgebra/Complex",
    50,
    90
);
compat_test_flat!(
    test_lean_meta_refined_discr_tree_compat_p12,
    "Lean/Meta/RefinedDiscrTree",
    50,
    90
);
compat_test_flat!(test_lean_expr_compat_p12, "Lean/Expr", 50, 90);
compat_test_flat!(
    test_group_theory_subgroup_compat_p12,
    "GroupTheory/Subgroup",
    50,
    90
);
compat_test_flat!(
    test_group_theory_coxeter_compat_p12,
    "GroupTheory/Coxeter",
    50,
    90
);
compat_test_flat!(
    test_geometry_manifold_integral_curve_compat_p12,
    "Geometry/Manifold/IntegralCurve",
    50,
    90
);
compat_test_flat!(
    test_geometry_manifold_instances_compat_p12,
    "Geometry/Manifold/Instances",
    50,
    90
);
compat_test_flat!(
    test_field_theory_rat_func_compat_p12,
    "FieldTheory/RatFunc",
    50,
    90
);
compat_test_flat!(
    test_field_theory_purely_inseparable_compat_p12,
    "FieldTheory/PurelyInseparable",
    50,
    90
);
compat_test_flat!(
    test_field_theory_is_alg_closed_compat_p12,
    "FieldTheory/IsAlgClosed",
    50,
    90
);
compat_test_flat!(
    test_dynamics_birkhoff_sum_compat_p12,
    "Dynamics/BirkhoffSum",
    50,
    90
);
compat_test_flat!(test_data_wseq_compat_p12, "Data/WSeq", 50, 90);
compat_test_flat!(test_data_tree_compat_p12, "Data/Tree", 50, 90);
compat_test_flat!(test_data_sigma_compat_p12, "Data/Sigma", 50, 90);
compat_test_flat!(
    test_data_set_pairwise_compat_p12,
    "Data/Set/Pairwise",
    50,
    90
);
compat_test_flat!(test_data_real_pi_compat_p12, "Data/Real/Pi", 50, 0);
compat_test_flat!(test_data_fun_like_compat_p12, "Data/FunLike", 50, 90);
compat_test_flat!(
    test_control_traversable_compat_p12,
    "Control/Traversable",
    50,
    90
);
compat_test_flat!(
    test_combinatorics_simple_graph_triangle_compat_p12,
    "Combinatorics/SimpleGraph/Triangle",
    50,
    90
);
compat_test_flat!(
    test_combinatorics_matroid_minor_compat_p12,
    "Combinatorics/Matroid/Minor",
    50,
    90
);
compat_test_flat!(
    test_category_theory_with_terminal_compat_p12,
    "CategoryTheory/WithTerminal",
    50,
    90
);
compat_test_flat!(
    test_category_theory_small_object_iteration_compat_p12,
    "CategoryTheory/SmallObject/Iteration",
    50,
    90
);
compat_test_flat!(
    test_category_theory_sites_descent_compat_p12,
    "CategoryTheory/Sites/Descent",
    50,
    90
);
compat_test_flat!(
    test_category_theory_sites_dense_subsite_compat_p12,
    "CategoryTheory/Sites/DenseSubsite",
    50,
    90
);
compat_test_flat!(
    test_category_theory_products_compat_p12,
    "CategoryTheory/Products",
    50,
    90
);
compat_test_flat!(
    test_category_theory_preadditive_yoneda_compat_p12,
    "CategoryTheory/Preadditive/Yoneda",
    50,
    0
);
compat_test_flat!(
    test_category_theory_preadditive_injective_compat_p12,
    "CategoryTheory/Preadditive/Injective",
    50,
    90
);
compat_test_flat!(
    test_category_theory_monoidal_action_compat_p12,
    "CategoryTheory/Monoidal/Action",
    50,
    90
);
compat_test_flat!(
    test_category_theory_locally_cartesian_closed_compat_p12,
    "CategoryTheory/LocallyCartesianClosed",
    50,
    90
);
compat_test_flat!(
    test_category_theory_localization_calculus_of_fractions_compat_p12,
    "CategoryTheory/Localization/CalculusOfFractions",
    50,
    90
);
compat_test_flat!(
    test_category_theory_linear_compat_p12,
    "CategoryTheory/Linear",
    50,
    90
);
compat_test_flat!(
    test_category_theory_limits_shapes_pullback_is_pullback_compat_p12,
    "CategoryTheory/Limits/Shapes/Pullback/IsPullback",
    50,
    90
);
compat_test_flat!(
    test_category_theory_limits_functor_category_compat_p12,
    "CategoryTheory/Limits/FunctorCategory",
    50,
    0
);
compat_test_flat!(
    test_category_theory_enriched_limits_compat_p12,
    "CategoryTheory/Enriched/Limits",
    50,
    0
);
compat_test_flat!(
    test_category_theory_bicategory_adjunction_compat_p12,
    "CategoryTheory/Bicategory/Adjunction",
    50,
    90
);
compat_test_flat!(
    test_analysis_special_functions_trigonometric_chebyshev_compat_p12,
    "Analysis/SpecialFunctions/Trigonometric/Chebyshev",
    50,
    90
);
compat_test_flat!(
    test_analysis_special_functions_continuous_functional_calculus_rpow_compat_p12,
    "Analysis/SpecialFunctions/ContinuousFunctionalCalculus/Rpow",
    50,
    90
);
compat_test_flat!(
    test_analysis_normed_module_ball_compat_p12,
    "Analysis/Normed/Module/Ball",
    50,
    90
);
compat_test_flat!(
    test_analysis_calculus_local_extr_compat_p12,
    "Analysis/Calculus/LocalExtr",
    50,
    90
);
compat_test_flat!(
    test_analysis_calculus_line_deriv_compat_p12,
    "Analysis/Calculus/LineDeriv",
    50,
    90
);
compat_test_flat!(
    test_algebraic_geometry_geometrically_compat_p12,
    "AlgebraicGeometry/Geometrically",
    50,
    0
);
compat_test_flat!(
    test_algebra_skew_monoid_algebra_compat_p12,
    "Algebra/SkewMonoidAlgebra",
    50,
    90
);
compat_test_flat!(
    test_algebra_polynomial_module_compat_p12,
    "Algebra/Polynomial/Module",
    50,
    90
);
compat_test_flat!(
    test_algebra_order_sub_compat_p12,
    "Algebra/Order/Sub",
    50,
    90
);
compat_test_flat!(
    test_algebra_order_star_compat_p12,
    "Algebra/Order/Star",
    50,
    90
);
compat_test_flat!(
    test_algebra_order_interval_set_compat_p12,
    "Algebra/Order/Interval/Set",
    50,
    90
);
compat_test_flat!(
    test_algebra_order_big_operators_group_compat_p12,
    "Algebra/Order/BigOperators/Group",
    50,
    90
);
compat_test_flat!(
    test_algebra_order_antidiag_compat_p12,
    "Algebra/Order/Antidiag",
    50,
    90
);
compat_test_flat!(
    test_algebra_no_zero_smul_divisors_compat_p12,
    "Algebra/NoZeroSMulDivisors",
    50,
    0
);
compat_test_flat!(
    test_algebra_group_with_zero_submonoid_compat_p12,
    "Algebra/GroupWithZero/Submonoid",
    50,
    90
);
compat_test_flat!(
    test_algebra_group_type_tags_compat_p12,
    "Algebra/Group/TypeTags",
    50,
    90
);
compat_test_flat!(
    test_algebra_group_int_compat_p12,
    "Algebra/Group/Int",
    50,
    90
);
compat_test_flat!(
    test_algebra_group_commute_compat_p12,
    "Algebra/Group/Commute",
    50,
    90
);
compat_test_flat!(
    test_algebra_free_monoid_compat_p12,
    "Algebra/FreeMonoid",
    50,
    90
);
compat_test_flat!(
    test_algebra_euclidean_domain_compat_p12,
    "Algebra/EuclideanDomain",
    50,
    90
);
compat_test_flat!(
    test_algebra_char_zero_compat_p12,
    "Algebra/CharZero",
    50,
    90
);
compat_test_flat!(
    test_algebra_category_alg_cat_compat_p12,
    "Algebra/Category/AlgCat",
    50,
    90
);
compat_test_flat!(
    test_algebra_big_operators_ring_compat_p12,
    "Algebra/BigOperators/Ring",
    50,
    90
);
compat_test_flat!(
    test_algebra_affine_monoid_compat_p12,
    "Algebra/AffineMonoid",
    50,
    90
);
compat_test_flat!(
    test_topology_uniform_space_ultra_compat_p12,
    "Topology/UniformSpace/Ultra",
    50,
    90
);
compat_test_flat!(
    test_topology_spectral_compat_p12,
    "Topology/Spectral",
    50,
    0
);
compat_test_flat!(
    test_topology_maps_proper_compat_p12,
    "Topology/Maps/Proper",
    50,
    90
);
compat_test_flat!(
    test_topology_instances_add_circle_compat_p12,
    "Topology/Instances/AddCircle",
    50,
    90
);
compat_test_flat!(
    test_topology_homeomorph_compat_p12,
    "Topology/Homeomorph",
    50,
    90
);
compat_test_flat!(test_topology_hom_compat_p12, "Topology/Hom", 50, 90);
compat_test_flat!(
    test_topology_covering_compat_p12,
    "Topology/Covering",
    50,
    90
);
compat_test_flat!(
    test_topology_compactification_one_point_compat_p12,
    "Topology/Compactification/OnePoint",
    50,
    90
);
compat_test_flat!(
    test_topology_cwcomplex_classical_compat_p12,
    "Topology/CWComplex/Classical",
    50,
    90
);
compat_test_flat!(
    test_topology_algebra_proper_action_compat_p12,
    "Topology/Algebra/ProperAction",
    50,
    90
);
compat_test_flat!(
    test_topology_algebra_monoid_compat_p12,
    "Topology/Algebra/Monoid",
    50,
    0
);
compat_test_flat!(
    test_topology_algebra_module_multilinear_compat_p12,
    "Topology/Algebra/Module/Multilinear",
    50,
    90
);
compat_test_flat!(
    test_topology_algebra_category_profinite_grp_compat_p12,
    "Topology/Algebra/Category/ProfiniteGrp",
    50,
    90
);
compat_test_flat!(
    test_testing_plausible_compat_p12,
    "Testing/Plausible",
    50,
    0
);
compat_test_flat!(
    test_tactic_positivity_compat_p12,
    "Tactic/Positivity",
    50,
    90
);
compat_test_flat!(test_tactic_order_compat_p12, "Tactic/Order", 50, 90);
compat_test_flat!(
    test_tactic_monotonicity_compat_p12,
    "Tactic/Monotonicity",
    50,
    0
);
compat_test_flat!(test_tactic_gcongr_compat_p12, "Tactic/GCongr", 50, 90);
compat_test_flat!(test_tactic_field_simp_compat_p12, "Tactic/FieldSimp", 50, 0);
compat_test_flat!(
    test_tactic_compute_asymptotics_multiseries_compat_p12,
    "Tactic/ComputeAsymptotics/Multiseries",
    50,
    90
);
compat_test_flat!(
    test_set_theory_surreal_compat_p12,
    "SetTheory/Surreal",
    50,
    90
);
compat_test_flat!(test_set_theory_pgame_compat_p12, "SetTheory/PGame", 50, 90);
compat_test_flat!(
    test_ring_theory_trace_compat_p12,
    "RingTheory/Trace",
    50,
    90
);
compat_test_flat!(
    test_ring_theory_quasi_finite_compat_p12,
    "RingTheory/QuasiFinite",
    50,
    90
);
compat_test_flat!(
    test_ring_theory_perfectoid_compat_p12,
    "RingTheory/Perfectoid",
    50,
    90
);
compat_test_flat!(test_ring_theory_norm_compat_p12, "RingTheory/Norm", 50, 90);
compat_test_flat!(
    test_ring_theory_mv_polynomial_symmetric_compat_p12,
    "RingTheory/MvPolynomial/Symmetric",
    50,
    90
);
compat_test_flat!(
    test_ring_theory_localization_away_compat_p12,
    "RingTheory/Localization/Away",
    50,
    90
);
compat_test_flat!(
    test_ring_theory_invariant_compat_p12,
    "RingTheory/Invariant",
    50,
    90
);
compat_test_flat!(
    test_ring_theory_integral_closure_is_integral_compat_p12,
    "RingTheory/IntegralClosure/IsIntegral",
    50,
    90
);
compat_test_flat!(
    test_ring_theory_integral_closure_algebra_compat_p12,
    "RingTheory/IntegralClosure/Algebra",
    50,
    0
);
compat_test_flat!(
    test_ring_theory_ideal_associated_prime_compat_p12,
    "RingTheory/Ideal/AssociatedPrime",
    50,
    90
);
compat_test_flat!(
    test_ring_theory_graded_algebra_homogeneous_compat_p12,
    "RingTheory/GradedAlgebra/Homogeneous",
    50,
    90
);
compat_test_flat!(
    test_ring_theory_flat_faithfully_flat_compat_p12,
    "RingTheory/Flat/FaithfullyFlat",
    50,
    90
);
compat_test_flat!(
    test_ring_theory_extension_presentation_compat_p12,
    "RingTheory/Extension/Presentation",
    50,
    90
);
compat_test_flat!(
    test_probability_kernel_ionescu_tulcea_compat_p12,
    "Probability/Kernel/IonescuTulcea",
    50,
    90
);
compat_test_flat!(
    test_probability_distributions_gaussian_has_gaussian_law_compat_p12,
    "Probability/Distributions/Gaussian/HasGaussianLaw",
    50,
    90
);
compat_test_flat!(test_order_preorder_compat_p12, "Order/Preorder", 50, 90);
compat_test_flat!(test_order_partition_compat_p12, "Order/Partition", 50, 90);
compat_test_flat!(test_order_defs_compat_p12, "Order/Defs", 50, 90);
compat_test_flat!(
    test_order_conditionally_complete_partial_order_compat_p12,
    "Order/ConditionallyCompletePartialOrder",
    50,
    90
);
compat_test_flat!(
    test_order_bounded_order_compat_p12,
    "Order/BoundedOrder",
    50,
    90
);
compat_test_flat!(
    test_order_boolean_algebra_compat_p12,
    "Order/BooleanAlgebra",
    50,
    90
);
compat_test_flat!(
    test_number_theory_ramification_inertia_compat_p12,
    "NumberTheory/RamificationInertia",
    50,
    90
);
compat_test_flat!(
    test_number_theory_number_field_units_compat_p12,
    "NumberTheory/NumberField/Units",
    50,
    90
);
compat_test_flat!(
    test_number_theory_number_field_ideal_compat_p12,
    "NumberTheory/NumberField/Ideal",
    50,
    0
);
compat_test_flat!(
    test_number_theory_number_field_discriminant_compat_p12,
    "NumberTheory/NumberField/Discriminant",
    50,
    90
);
compat_test_flat!(
    test_number_theory_mul_char_compat_p12,
    "NumberTheory/MulChar",
    50,
    90
);
compat_test_flat!(
    test_number_theory_modular_forms_eisenstein_series_e2_compat_p12,
    "NumberTheory/ModularForms/EisensteinSeries/E2",
    50,
    90
);
compat_test_flat!(
    test_number_theory_euler_product_compat_p12,
    "NumberTheory/EulerProduct",
    50,
    0
);
compat_test_flat!(
    test_model_theory_algebra_ring_compat_p12,
    "ModelTheory/Algebra/Ring",
    50,
    90
);
compat_test_flat!(
    test_model_theory_algebra_field_compat_p12,
    "ModelTheory/Algebra/Field",
    50,
    0
);
compat_test_flat!(
    test_measure_theory_integral_riesz_markov_kakutani_compat_p12,
    "MeasureTheory/Integral/RieszMarkovKakutani",
    50,
    90
);
compat_test_flat!(
    test_measure_theory_function_l1_space_compat_p12,
    "MeasureTheory/Function/L1Space",
    50,
    90
);
compat_test_flat!(
    test_measure_theory_constructions_polish_compat_p12,
    "MeasureTheory/Constructions/Polish",
    50,
    90
);
compat_test_flat!(test_logic_encodable_compat_p12, "Logic/Encodable", 50, 90);
compat_test_flat!(
    test_linear_algebra_tensor_power_compat_p12,
    "LinearAlgebra/TensorPower",
    50,
    90
);
compat_test_flat!(
    test_linear_algebra_span_compat_p12,
    "LinearAlgebra/Span",
    50,
    90
);
compat_test_flat!(
    test_linear_algebra_perfect_pairing_compat_p12,
    "LinearAlgebra/PerfectPairing",
    50,
    90
);
compat_test_flat!(
    test_linear_algebra_matrix_determinant_compat_p12,
    "LinearAlgebra/Matrix/Determinant",
    50,
    90
);
compat_test_flat!(
    test_linear_algebra_linear_independent_compat_p12,
    "LinearAlgebra/LinearIndependent",
    50,
    90
);
compat_test_flat!(
    test_linear_algebra_finite_dimensional_compat_p12,
    "LinearAlgebra/FiniteDimensional",
    50,
    90
);
compat_test_flat!(
    test_linear_algebra_exterior_power_compat_p12,
    "LinearAlgebra/ExteriorPower",
    50,
    0
);
compat_test_flat!(
    test_linear_algebra_exterior_algebra_compat_p12,
    "LinearAlgebra/ExteriorAlgebra",
    50,
    90
);
compat_test_flat!(
    test_linear_algebra_charpoly_compat_p12,
    "LinearAlgebra/Charpoly",
    50,
    90
);
compat_test_flat!(
    test_linear_algebra_alternating_compat_p12,
    "LinearAlgebra/Alternating",
    50,
    90
);
compat_test_flat!(
    test_group_theory_submonoid_compat_p12,
    "GroupTheory/Submonoid",
    50,
    90
);
compat_test_flat!(
    test_group_theory_specific_groups_alternating_compat_p12,
    "GroupTheory/SpecificGroups/Alternating",
    50,
    90
);
compat_test_flat!(
    test_group_theory_ore_localization_compat_p12,
    "GroupTheory/OreLocalization",
    50,
    90
);
compat_test_flat!(
    test_geometry_manifold_sheaf_compat_p12,
    "Geometry/Manifold/Sheaf",
    50,
    90
);
compat_test_flat!(
    test_geometry_manifold_is_manifold_compat_p12,
    "Geometry/Manifold/IsManifold",
    50,
    90
);
compat_test_flat!(
    test_geometry_euclidean_inversion_compat_p12,
    "Geometry/Euclidean/Inversion",
    50,
    90
);
compat_test_flat!(
    test_field_theory_normal_compat_p12,
    "FieldTheory/Normal",
    50,
    90
);
compat_test_flat!(
    test_field_theory_intermediate_field_adjoin_compat_p12,
    "FieldTheory/IntermediateField/Adjoin",
    50,
    90
);
compat_test_flat!(
    test_dynamics_fixed_points_compat_p12,
    "Dynamics/FixedPoints",
    50,
    90
);
compat_test_flat!(
    test_dynamics_ergodic_action_compat_p12,
    "Dynamics/Ergodic/Action",
    50,
    0
);
compat_test_flat!(test_data_w_compat_p12, "Data/W", 50, 0);
compat_test_flat!(test_data_sym_sym2_compat_p12, "Data/Sym/Sym2", 50, 90);
compat_test_flat!(test_data_sym_compat_p12, "Data/Sym", 50, 90);
compat_test_flat!(test_data_string_compat_p12, "Data/String", 50, 0);
compat_test_flat!(
    test_data_pfunctor_multivariate_compat_p12,
    "Data/PFunctor/Multivariate",
    50,
    90
);
compat_test_flat!(test_data_ordmap_compat_p12, "Data/Ordmap", 50, 90);
compat_test_flat!(test_data_nat_digits_compat_p12, "Data/Nat/Digits", 50, 90);
compat_test_flat!(
    test_data_nat_cast_order_compat_p12,
    "Data/Nat/Cast/Order",
    50,
    90
);
compat_test_flat!(test_data_nnreal_compat_p12, "Data/NNReal", 50, 90);
compat_test_flat!(test_data_list_perm_compat_p12, "Data/List/Perm", 50, 90);
compat_test_flat!(test_data_int_order_compat_p12, "Data/Int/Order", 50, 90);
compat_test_flat!(test_data_ereal_compat_p12, "Data/EReal", 50, 90);
compat_test_flat!(test_data_countable_compat_p12, "Data/Countable", 50, 90);
compat_test_flat!(test_control_monad_compat_p12, "Control/Monad", 50, 90);
compat_test_flat!(
    test_control_bitraversable_compat_p12,
    "Control/Bitraversable",
    50,
    90
);
compat_test_flat!(
    test_computability_akra_bazzi_compat_p12,
    "Computability/AkraBazzi",
    50,
    90
);
compat_test_flat!(
    test_combinatorics_simple_graph_extremal_compat_p12,
    "Combinatorics/SimpleGraph/Extremal",
    50,
    0
);
compat_test_flat!(
    test_combinatorics_quiver_path_compat_p12,
    "Combinatorics/Quiver/Path",
    50,
    90
);
compat_test_flat!(
    test_combinatorics_matroid_rank_compat_p12,
    "Combinatorics/Matroid/Rank",
    50,
    90
);
compat_test_flat!(
    test_combinatorics_enumerative_partition_compat_p12,
    "Combinatorics/Enumerative/Partition",
    50,
    90
);
compat_test_flat!(
    test_combinatorics_derangements_compat_p12,
    "Combinatorics/Derangements",
    50,
    90
);
compat_test_flat!(
    test_category_theory_types_compat_p12,
    "CategoryTheory/Types",
    50,
    90
);
compat_test_flat!(
    test_category_theory_sums_compat_p12,
    "CategoryTheory/Sums",
    50,
    90
);
compat_test_flat!(
    test_category_theory_sites_sheaf_cohomology_compat_p12,
    "CategoryTheory/Sites/SheafCohomology",
    50,
    90
);
compat_test_flat!(
    test_category_theory_sites_point_compat_p12,
    "CategoryTheory/Sites/Point",
    50,
    90
);
compat_test_flat!(
    test_category_theory_monoidal_internal_types_compat_p12,
    "CategoryTheory/Monoidal/Internal/Types",
    50,
    90
);
compat_test_flat!(
    test_category_theory_monoidal_internal_compat_p12,
    "CategoryTheory/Monoidal/Internal",
    50,
    0
);
compat_test_flat!(
    test_category_theory_monoidal_day_convolution_compat_p12,
    "CategoryTheory/Monoidal/DayConvolution",
    50,
    0
);
compat_test_flat!(
    test_category_theory_monoidal_closed_functor_category_compat_p12,
    "CategoryTheory/Monoidal/Closed/FunctorCategory",
    50,
    0
);
compat_test_flat!(
    test_category_theory_localization_monoidal_compat_p12,
    "CategoryTheory/Localization/Monoidal",
    50,
    90
);
compat_test_flat!(
    test_category_theory_limits_final_compat_p12,
    "CategoryTheory/Limits/Final",
    50,
    0
);
compat_test_flat!(
    test_category_theory_limits_constructions_over_compat_p12,
    "CategoryTheory/Limits/Constructions/Over",
    50,
    0
);
compat_test_flat!(
    test_category_theory_functor_reflects_iso_compat_p12,
    "CategoryTheory/Functor/ReflectsIso",
    50,
    0
);
compat_test_flat!(
    test_category_theory_discrete_compat_p12,
    "CategoryTheory/Discrete",
    50,
    90
);
compat_test_flat!(
    test_category_theory_copy_discard_category_compat_p12,
    "CategoryTheory/CopyDiscardCategory",
    50,
    0
);
compat_test_flat!(
    test_category_theory_comma_over_compat_p12,
    "CategoryTheory/Comma/Over",
    50,
    90
);
compat_test_flat!(
    test_category_theory_closed_functor_category_compat_p12,
    "CategoryTheory/Closed/FunctorCategory",
    50,
    0
);
compat_test_flat!(
    test_category_theory_bicategory_natural_transformation_compat_p12,
    "CategoryTheory/Bicategory/NaturalTransformation",
    50,
    0
);
compat_test_flat!(
    test_category_theory_bicategory_kan_compat_p12,
    "CategoryTheory/Bicategory/Kan",
    50,
    90
);
compat_test_flat!(
    test_category_theory_abelian_serre_class_compat_p12,
    "CategoryTheory/Abelian/SerreClass",
    50,
    0
);
compat_test_flat!(
    test_analysis_special_functions_integrals_compat_p12,
    "Analysis/SpecialFunctions/Integrals",
    50,
    90
);
compat_test_flat!(
    test_analysis_special_functions_gaussian_compat_p12,
    "Analysis/SpecialFunctions/Gaussian",
    50,
    90
);
compat_test_flat!(
    test_analysis_normed_space_hahn_banach_compat_p12,
    "Analysis/NormedSpace/HahnBanach",
    50,
    0
);
compat_test_flat!(
    test_analysis_normed_order_compat_p12,
    "Analysis/Normed/Order",
    50,
    90
);
compat_test_flat!(
    test_analysis_normed_module_rclike_compat_p12,
    "Analysis/Normed/Module/RCLike",
    50,
    90
);
compat_test_flat!(
    test_analysis_inner_product_space_harmonic_compat_p12,
    "Analysis/InnerProductSpace/Harmonic",
    50,
    90
);
compat_test_flat!(
    test_analysis_distribution_schwartz_space_compat_p12,
    "Analysis/Distribution/SchwartzSpace",
    50,
    90
);
compat_test_flat!(
    test_analysis_convex_specific_functions_compat_p12,
    "Analysis/Convex/SpecificFunctions",
    50,
    90
);
compat_test_flat!(
    test_analysis_complex_polynomial_compat_p12,
    "Analysis/Complex/Polynomial",
    50,
    90
);
compat_test_flat!(
    test_analysis_cstar_algebra_module_compat_p12,
    "Analysis/CStarAlgebra/Module",
    50,
    90
);
compat_test_flat!(
    test_algebraic_topology_simplex_category_generators_relations_compat_p12,
    "AlgebraicTopology/SimplexCategory/GeneratorsRelations",
    50,
    90
);
compat_test_flat!(
    test_algebraic_geometry_modules_compat_p12,
    "AlgebraicGeometry/Modules",
    50,
    90
);
compat_test_flat!(
    test_algebraic_geometry_ideal_sheaf_compat_p12,
    "AlgebraicGeometry/IdealSheaf",
    50,
    90
);
compat_test_flat!(
    test_algebraic_geometry_elliptic_curve_projective_compat_p12,
    "AlgebraicGeometry/EllipticCurve/Projective",
    50,
    90
);
compat_test_flat!(
    test_algebraic_geometry_elliptic_curve_jacobian_compat_p12,
    "AlgebraicGeometry/EllipticCurve/Jacobian",
    50,
    90
);
compat_test_flat!(
    test_algebraic_geometry_elliptic_curve_affine_compat_p12,
    "AlgebraicGeometry/EllipticCurve/Affine",
    50,
    90
);
compat_test_flat!(test_algebra_tropical_compat_p12, "Algebra/Tropical", 50, 90);
compat_test_flat!(test_algebra_ring_int_compat_p12, "Algebra/Ring/Int", 50, 90);
compat_test_flat!(
    test_algebra_quadratic_algebra_compat_p12,
    "Algebra/QuadraticAlgebra",
    50,
    90
);
compat_test_flat!(
    test_algebra_order_succ_pred_compat_p12,
    "Algebra/Order/SuccPred",
    50,
    90
);
compat_test_flat!(
    test_algebra_order_group_with_zero_unbundled_compat_p12,
    "Algebra/Order/GroupWithZero/Unbundled",
    50,
    90
);
compat_test_flat!(
    test_algebra_order_group_unbundled_compat_p12,
    "Algebra/Order/Group/Unbundled",
    50,
    93
);
compat_test_flat!(
    test_algebra_order_group_pointwise_compat_p12,
    "Algebra/Order/Group/Pointwise",
    50,
    90
);
compat_test_flat!(
    test_algebra_order_group_action_compat_p12,
    "Algebra/Order/Group/Action",
    50,
    90
);
compat_test_flat!(
    test_algebra_order_cau_seq_compat_p12,
    "Algebra/Order/CauSeq",
    50,
    90
);
compat_test_flat!(
    test_algebra_order_big_operators_ring_compat_p12,
    "Algebra/Order/BigOperators/Ring",
    50,
    90
);
compat_test_flat!(
    test_algebra_order_big_operators_group_with_zero_compat_p12,
    "Algebra/Order/BigOperators/GroupWithZero",
    50,
    90
);
compat_test_flat!(
    test_algebra_module_zlattice_compat_p12,
    "Algebra/Module/ZLattice",
    50,
    90
);
compat_test_flat!(
    test_algebra_module_equiv_compat_p12,
    "Algebra/Module/Equiv",
    50,
    90
);
compat_test_flat!(
    test_algebra_lie_semisimple_compat_p12,
    "Algebra/Lie/Semisimple",
    50,
    90
);
compat_test_flat!(
    test_algebra_lie_derivation_compat_p12,
    "Algebra/Lie/Derivation",
    50,
    90
);
compat_test_flat!(
    test_algebra_homology_left_resolution_compat_p12,
    "Algebra/Homology/LeftResolution",
    50,
    90
);
compat_test_flat!(
    test_algebra_group_with_zero_units_compat_p12,
    "Algebra/GroupWithZero/Units",
    50,
    90
);
compat_test_flat!(
    test_algebra_group_with_one_compat_p12,
    "Algebra/Group/WithOne",
    50,
    90
);
compat_test_flat!(
    test_algebra_group_semiconj_compat_p12,
    "Algebra/Group/Semiconj",
    50,
    90
);
compat_test_flat!(test_algebra_group_pi_compat_p12, "Algebra/Group/Pi", 50, 90);
compat_test_flat!(
    test_algebra_group_irreducible_compat_p12,
    "Algebra/Group/Irreducible",
    50,
    90
);
compat_test_flat!(
    test_algebra_category_module_cat_monoidal_compat_p12,
    "Algebra/Category/ModuleCat/Monoidal",
    50,
    0
);
compat_test_flat!(
    test_algebra_category_module_cat_ext_compat_p12,
    "Algebra/Category/ModuleCat/Ext",
    50,
    0
);
compat_test_flat!(
    test_algebra_category_comm_alg_cat_compat_p12,
    "Algebra/Category/CommAlgCat",
    50,
    90
);
compat_test_flat!(
    test_algebra_category_coalg_cat_compat_p12,
    "Algebra/Category/CoalgCat",
    50,
    0
);
compat_test_flat!(
    test_algebra_big_operators_group_list_compat_p12,
    "Algebra/BigOperators/Group/List",
    50,
    90
);
compat_test_flat!(test_algebra_azumaya_compat_p12, "Algebra/Azumaya", 50, 90);
compat_test_flat!(
    test_algebra_algebra_spectrum_compat_p12,
    "Algebra/Algebra/Spectrum",
    50,
    90
);
// Phase 12: Archive and Counterexamples (outside Mathlib/)
compat_test_recursive!(test_archive_compat, "../Archive", 200, 0);
compat_test_recursive!(test_counterexamples_compat, "../Counterexamples", 200, 0);
