//! # OxiLean Standard Library — Core Data Types & Theorems
//!
//! The standard library provides essential types, type classes, and theorems for OxiLean programs.
//! It is written in OxiLean itself and loaded into the kernel environment at startup.
//!
//! ## Architecture Overview
//!
//! The standard library is organized into logical categories:
//! - Primitive Types: `nat`, `bool`, `char`, `int`, `string`, `list`, `array`, `option`, `result`
//! - Type Classes: `Eq`, `Ord`, `Show`, `Repr`, `Functor`, `Monad`, `Monoid`, `Decidable`
//! - Core Theorems: equality, logic, arithmetic, order
//! - Utilities: tactic lemmas, decision procedures, parsing combinators
//!
//! ## Environment Building
//!
//! Loading happens via `build_*_env` functions:
//!
//! ```ignore
//! let env = Environment::new();
//! let env = build_nat_env(env)?;
//! let env = build_list_env(env)?;
//! let env = build_eq_env(env)?;
//! ```
//!
//! See `registry` module for module metadata and dependency graph.

#![allow(missing_docs)]
#![warn(clippy::all)]
#![allow(clippy::module_inception)]
#![allow(mixed_script_confusables)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::map_unwrap_or)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_else_if)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::vec_box)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::redundant_field_names)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::int_plus_one)]
#![allow(clippy::implicit_saturating_sub)]
#![allow(clippy::assign_op_pattern)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::useless_let_if_seq)]
#![allow(clippy::identity_op)]
#![allow(clippy::if_same_then_else)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::manual_range_contains)]
#![allow(clippy::option_if_let_else)]
#![allow(clippy::unnecessary_unwrap)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::return_self_not_must_use)]
#![allow(clippy::items_after_statements)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::similar_names)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::float_cmp)]
#![allow(clippy::match_same_arms)]
#![allow(clippy::single_match_else)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::unused_self)]
#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::use_self)]
#![allow(clippy::default_trait_access)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::vec_init_then_push)]
#![allow(clippy::box_collection)]
#![allow(clippy::iter_nth_zero)]
#![allow(clippy::new_without_default)]
#![allow(clippy::useless_conversion)]
#![allow(suspicious_double_ref_op)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::inline_always)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(clippy::unnecessary_map_or)]
#![allow(clippy::extra_unused_lifetimes)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::borrow_as_ptr)]
#![allow(clippy::used_underscore_binding)]
#![allow(clippy::manual_let_else)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::useless_format)]
#![allow(clippy::option_option)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::enum_variant_names)]
#![allow(clippy::single_char_lifetime_names)]
#![allow(clippy::needless_update)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::missing_fields_in_debug)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::explicit_deref_methods)]
#![allow(clippy::suspicious_arithmetic_impl)]
#![allow(clippy::suspicious_op_assign_impl)]
#![allow(clippy::duplicated_attributes)]
#![allow(clippy::match_like_matches_macro)]
#![allow(clippy::manual_clamp)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::borrowed_box)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::option_map_or_none)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::map_clone)]
#![allow(clippy::unnecessary_cast)]
#![allow(unused_imports)]

// ── Standard library modules ──────────────────────────────────────────────────

pub mod abstract_algebra_adv;
pub mod abstract_interpretation;
pub mod abstract_rewriting;
pub mod algebra;
pub mod algebraic_combinatorics;
pub mod algebraic_effects;
pub mod algebraic_geometry;
pub mod algebraic_k_theory;
pub mod algebraic_number_theory;
pub mod algebraic_topology;
pub mod analytic_number_theory;
pub mod arithmetic_geometry;
pub mod array;
pub mod automata_theory;
pub mod bitvec;
pub mod bool;
pub mod category_theory;
pub mod category_theory_ext;
pub mod certified_algorithms;
pub mod chaos_theory;
pub mod char;
pub mod chromatic_homotopy;
pub mod coding_theory;
pub mod coinduction;
pub mod combinatorial_game_theory;
pub mod combinatorial_optimization;
pub mod combinatorics;
pub mod combinatory_logic;
pub mod complex;
pub mod complexity;
pub mod computability_theory;
pub mod computational_geometry;
pub mod concurrency_theory;
pub mod constructive_mathematics;
pub mod control_theory;
pub mod convex_geometry;
pub mod convex_optimization;
pub mod cryptographic_protocols;
pub mod cryptography;
pub mod data_structures;
pub mod decidable;
pub mod denotational_semantics;
pub mod dependent_type_theory;
pub mod derived_algebraic_geometry;
pub mod descriptive_set_theory;
pub mod differential_equations;
pub mod differential_geometry;
pub mod diophantine_geometry;
pub mod domain_theory;
pub mod either;
pub mod elliptic_curves;
pub mod env_builder;
pub mod eq;
pub mod ergodic_theory;
pub mod fin;
pub mod forcing_theory;
pub mod formal_languages;
pub mod formal_verification;
pub mod functional_analysis;
pub mod functional_calculus;
pub mod functional_programming;
pub mod functor;
pub mod fuzzy_logic;
pub mod game_theory;
pub mod game_theory_ext;
pub mod geometric_group_theory;
pub mod geometric_topology;
pub mod graph;
pub mod graph_algorithms;
pub mod groebner_bases;
pub mod group_theory;
pub mod hashmap;
pub mod hashset;
pub mod higher_category_theory;
pub mod homological_algebra;
pub mod homological_computations;
pub mod homotopy_theory;
pub mod homotopy_type_theory;
pub mod hybrid_systems;
pub mod information_geometry;
pub mod information_theory;
pub mod information_theory_ext;
pub mod int;
pub mod io;
pub mod iwasawa_theory;
pub mod k_theory;
pub mod lattice_theory;
pub mod lazy;
pub mod lie_theory;
pub mod linear_algebra;
pub mod linear_logic;
pub mod linear_programming;
pub mod list;
pub mod logic;
pub mod machine_learning;
pub mod mathematical_physics;
pub mod measure_theory;
pub mod model_checking;
pub mod model_theory;
pub mod monad;
pub mod motivic_cohomology;
pub mod nat;
pub mod noncommutative_geometry;
pub mod nonlinear_dynamics;
pub mod number_theory;
pub mod numerical_analysis;
pub mod numerical_linear_algebra;
pub mod operations_research;
pub mod operator_algebras;
pub mod optimal_transport;
pub mod option;
pub mod ord;
pub mod order;
pub mod order_topology;
pub mod ordering;
pub mod padic_analysis;
pub mod padic_hodge_theory;
pub mod parametricity;
pub mod parsec;
pub mod pde_theory;
pub mod persistent_data_structures;
pub mod point_set_topology;
pub mod probability;
pub mod prod;
pub mod proof_mining;
pub mod proof_theory;
pub mod prop;
pub mod quantum_computing;
pub mod quantum_field_theory;
pub mod quantum_information;
pub mod quotient_types;
pub mod ramsey_theory;
pub mod random_matrix_theory;
pub mod range;
pub mod rbtree;
pub mod real;
pub mod repr;
pub mod representation_theory;
pub mod result;
pub mod reverse_mathematics;
pub mod set;
pub mod set_theory_zfc;
pub mod show;
pub mod sigma;
pub mod social_choice_theory;
pub mod spectral_graph_theory;
pub mod spectral_theory;
pub mod statistical_learning;
pub mod statistical_mechanics;
pub mod stochastic_control;
pub mod stochastic_processes;
pub mod stream;
pub mod string;
pub mod sum;
pub mod symplectic_geometry;
pub mod tactic_lemmas;
pub mod tauberian_theory;
pub mod term_rewriting;
pub mod thunk;
pub mod topological_data_analysis;
pub mod topology;
pub mod topology_ext;
pub mod topos_theory;
pub mod tropical_geometry;
pub mod type_theory;
pub mod universal_algebra;
pub mod variational_calculus;
pub mod vec;
pub mod wellfounded;

// Extended modules
pub mod abstract_algebra_advanced;
pub mod approximation_algorithms;
pub mod bayesian_networks;
pub mod bioinformatics;
pub mod birational_geometry;
pub mod categorical_logic;
pub mod coalgebra_theory;
pub mod compiler_theory;
pub mod convex_analysis;
pub mod cubical_type_theory;
pub mod differential_privacy;
pub mod distributed_systems_theory;
pub mod finite_element_method;
pub mod formal_argumentation;
pub mod formal_epistemology;
pub mod geometric_measure_theory;
pub mod harmonic_analysis;
pub mod information_theoretic_security;
pub mod intersection_theory;
pub mod knot_theory;
pub mod lambda_calculus;
pub mod linear_temporal_logic;
pub mod logic_programming;
pub mod low_dimensional_topology;
pub mod mathematical_ecology;
pub mod mechanism_design;
pub mod modal_logic;
pub mod modular_forms;
pub mod network_theory;
pub mod nonstandard_analysis;
pub mod observational_type_theory;
pub mod optimization_theory;
pub mod parameterized_complexity;
pub mod point_free_topology;
pub mod post_quantum_crypto;
pub mod probabilistic_programming;
pub mod program_extraction;
pub mod program_logics;
pub mod program_synthesis;
pub mod quantum_error_correction;
pub mod rough_set_theory;
pub mod session_types;
pub mod sheaf_theory;
pub mod spectral_methods;
pub mod static_analysis;
pub mod string_theory_basics;
pub mod surreal_numbers;
pub mod systems_biology;
pub mod temporal_logic;
pub mod topological_quantum_computation;
pub mod type_directed_search;
pub mod type_inference_algorithms;
pub mod type_theory_advanced;
pub mod variational_analysis;
pub mod zero_knowledge_proofs;

// ── Registry and utilities sub-modules ───────────────────────────────────────
pub mod registry;
pub mod std_utilities;

// ── Re-exports of build_*_env functions ──────────────────────────────────────

pub use algebra::build_algebra_env;
pub use array::build_array_env;
pub use bitvec::build_bitvec_env;
pub use bool::build_bool_env;
pub use category_theory::build_category_theory_env;
pub use char::build_char_env;
pub use decidable::build_decidable_env;
pub use either::build_either_env;
pub use eq::build_eq_env;
pub use fin::build_fin_env;
pub use functor::build_functor_env;
pub use group_theory::build_group_theory_env;
pub use hashmap::build_hashmap_env;
pub use hashset::build_hashset_env;
pub use int::build_int_env;
pub use io::build_io_env;
pub use lazy::build_lazy_env;
pub use list::build_list_env;
pub use logic::build_logic_env;
pub use monad::build_monad_env;
pub use nat::build_nat_env;
pub use option::build_option_env;
pub use ord::build_ord_env;
pub use order::build_order_env;
pub use ordering::build_ordering_env;
pub use parsec::build_parsec_env;
pub use prod::build_prod_env;
pub use prop::build_prop_env;
pub use quotient_types::build_quotient_types_env;
pub use range::build_range_env;
pub use rbtree::build_rbtree_env;
pub use real::build_real_env;
pub use repr::build_repr_env;
pub use result::build_result_env;
pub use set::build_set_env;
pub use show::build_show_env;
pub use sigma::build_sigma_env;
pub use stream::build_stream_env;
pub use string::build_string_env;
pub use sum::build_sum_env;
pub use tactic_lemmas::build_tactic_lemmas_env;
pub use thunk::build_thunk_env;
pub use topology::build_topology_env;
pub use vec::build_vec_env;
pub use wellfounded::build_wellfounded_env;

// ── Re-exports from registry ─────────────────────────────────────────────────

pub use registry::{
    all_modules, all_theorems, count_default_modules, count_total_modules, default_modules,
    dependents_of, direct_deps, entries_in_module, find_module, lookup_std_entry, module_category,
    modules_for_phase, topological_sort_modules, BuildPhase, ModuleDep, StdCategory, StdEntry,
    StdFeatures, StdLibStats, StdModuleEntry, StdVersion, CORE_DEPS, STD_KNOWN_ENTRIES,
    STD_MODULE_REGISTRY, STD_VERSION,
};

// ── Re-exports from std_utilities ────────────────────────────────────────────

pub use std_utilities::{
    BitSet64, Counter, Diagnostic, DiagnosticBag, DiagnosticLevel, DirectedGraph, FreshNameGen,
    Located, MinHeap, MultiMap, NameTable, ScopeTable, Span, StringSet, Trie,
};
