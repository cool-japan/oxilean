//! # OxiLean Standard Library — Core Data Types & Theorems
//!
//! The standard library provides essential types, type classes, and theorems for OxiLean programs.
//! It is written in OxiLean itself and loaded into the kernel environment at startup.
//!
//! ## Quick Start
//!
//! ### Using Standard Types
//!
//! ```ignore
//! theorem two_plus_three : 2 + 3 = 5 := by
//!   norm_num
//!
//! def my_list : List Nat := [1, 2, 3]
//!
//! instance : Eq Nat := { ... }
//! ```
//!
//! ## Architecture Overview
//!
//! The standard library is organized into logical categories:
//!
//! ```text
//! Standard Library Organization
//! ├─ Primitive Types
//! │  ├─ nat, bool, char, int
//! │  ├─ string, list, array
//! │  ├─ option, result, sum
//! │  └─ prod, sigma
//! │
//! ├─ Type Classes & Traits
//! │  ├─ Eq, Ord, Show, Repr
//! │  ├─ Functor, Monad, Applicative
//! │  ├─ Monoid, Semigroup
//! │  └─ Decidable
//! │
//! ├─ Core Theorems & Lemmas
//! │  ├─ Equality (eq.rs)
//! │  ├─ Logic (logic.rs, prop.rs)
//! │  ├─ Arithmetic (nat.rs, int.rs)
//! │  └─ Order (ord.rs, order.rs)
//! │
//! └─ Utilities & Automation
//!    ├─ Tactic lemmas
//!    ├─ Decision procedures
//!    └─ Parsing combinators
//! ```
//!
//! ## Key Concepts & Terminology
//!
//! ### Inductive Types
//!
//! Core types are defined inductively:
//!
//! ```text
//! Nat : Type 0
//!   | zero : Nat
//!   | succ : Nat → Nat
//!
//! Bool : Prop
//!   | true : Bool
//!   | false : Bool
//!
//! List α : Type 0
//!   | nil : List α
//!   | cons : α → List α → List α
//! ```
//!
//! Each inductive type comes with:
//! - **Constructors**: Ways to create values
//! - **Recursor**: Principle for inductive recursion
//! - **Induction principle**: Structural induction proof technique
//!
//! ### Type Classes
//!
//! Type classes enable ad-hoc polymorphism:
//!
//! ```text
//! class Eq (α : Type u) : Prop where
//!   eq : α → α → Prop
//!   refl : ∀ x, eq x x
//!   symm : ∀ x y, eq x y → eq y x
//!   trans : ∀ x y z, eq x y → eq y z → eq x z
//! ```
//!
//! Instances provide implementations for specific types:
//!
//! ```text
//! instance Eq Nat : Eq Nat where
//!   eq := Nat.eq
//!   refl := Nat.eq_refl
//!   ...
//! ```
//!
//! ### Recursion Schemes
//!
//! Each inductive type gets a **recursor** for primitive recursion:
//!
//! ```text
//! Nat.rec :
//!   {motive : Nat → Sort u}
//! → motive 0
//! → (∀ n, motive n → motive (n + 1))
//! → ∀ n, motive n
//! ```
//!
//! Higher-order recursion patterns (fold, map, etc.) are defined in terms of recs.
//!
//! ### Decidability
//!
//! For computable propositions, we can decide them algorithmically:
//!
//! ```text
//! class Decidable (p : Prop) : Type 0 where
//!   decide : Bool
//!   correct : decide = true ↔ p
//! ```
//!
//! Examples: `2 + 3 = 5` is decidable (computable), but `∀ n, P n` usually isn't.
//!
//! ## Module Organization
//!
//! ### Primitive Types & Constructors
//!
//! | Module | Types | Purpose |
//! |--------|-------|---------|
//! | `nat` | `Nat` | Natural numbers (zero, successor) |
//! | `bool` | `Bool` | Booleans (true, false) |
//! | `int` | `Int` | Signed integers |
//! | `char` | `Char` | Unicode characters |
//! | `string` | `String` | Text strings |
//!
//! ### Collection Types
//!
//! | Module | Types | Purpose |
//! |--------|-------|---------|
//! | `list` | `List α` | Linked lists |
//! | `array` | `Array α` | Fixed-size arrays |
//! | `vec` | `Vec α` | Dynamic vectors (like Lean 4) |
//! | `set` | `Set α` | Mathematical sets |
//! | `fin` | `Fin n` | Finite sets (0..n-1) |
//!
//! ### Option & Alternative Types
//!
//! | Module | Types | Purpose |
//! |--------|-------|---------|
//! | `option` | `Option α` | Nullable value |
//! | `result` | `Result α ε` | Value or error |
//! | `sum` | `α ⊕ β` | Tagged union |
//! | `either` | `Either α β` | Alternative representation |
//! | `prod` | `α × β` | Pairs/products |
//! | `sigma` | `Σ x, P x` | Dependent pairs |
//!
//! ### Type Classes
//!
//! | Module | Classes | Purpose |
//! |--------|---------|---------|
//! | `eq` | `Eq` | Equality relation |
//! | `ord` | `Ord` | Total ordering |
//! | `show` | `Show` | String representation |
//! | `repr` | `Repr` | Debug representation |
//! | `decidable` | `Decidable` | Computable propositions |
//!
//! ### Higher-Order Abstractions
//!
//! | Module | Classes | Purpose |
//! |--------|---------|---------|
//! | `functor` | `Functor` | Map over container |
//! | `applicative` | `Applicative` | Apply pure functions |
//! | `monad` | `Monad` | Sequencing computations |
//! | `algebra` | `Monoid`, `Semigroup` | Algebraic structures |
//!
//! ### Logic & Propositions
//!
//! | Module | Purpose |
//! |--------|---------|
//! | `logic` | Classical logic (excluded middle, etc.) |
//! | `prop` | Propositional theorems |
//! | `eq` | Equality and substitution |
//!
//! ### Arithmetic & Order
//!
//! | Module | Purpose |
//! |--------|---------|
//! | `nat` | Natural number theorems |
//! | `int` | Integer theorems |
//! | `order` | Partial orders |
//! | `ordering` | Compare result type |
//! | `range` | Integer ranges |
//!
//! ### Advanced Structures
//!
//! | Module | Purpose |
//! |--------|---------|
//! | `rbtree` | Red-black trees (sorted maps) |
//! | `hashmap` | Hash maps |
//! | `hashset` | Hash sets |
//! | `stream` | Lazy sequences |
//! | `thunk` | Delayed computation |
//! | `lazy` | Lazy values |
//!
//! ### Utilities
//!
//! | Module | Purpose |
//! |--------|---------|
//! | `wellfounded` | Well-founded relations |
//! | `quotient_types` | Quotient type operations |
//! | `tactic_lemmas` | Lemmas useful for automation |
//! | `parsec` | Parsing combinators |
//! | `io` | I/O operations |
//!
//! ## Type Hierarchy
//!
//! The standard library respects the universe hierarchy:
//!
//! ```text
//! Data types (Nat, Bool, List) : Type 0
//! Type classes (Eq, Show) : Type 0 → Prop or Type 1
//! Propositions (Nat.add_comm) : Prop
//! Proofs (rfl, trans) : Prop
//! ```
//!
//! ## Usage Examples
//!
//! ### Example 1: Working with Natural Numbers
//!
//! ```ignore
//! theorem add_comm : ∀ n m : Nat, n + m = m + n := by
//!   intro n m
//!   induction n with
//!   | zero => simp
//!   | succ n ih => simp [ih]
//! ```
//!
//! ### Example 2: List Operations
//!
//! ```ignore
//! def append {α : Type u} : List α → List α → List α
//!   | [], ys => ys
//!   | x :: xs, ys => x :: append xs ys
//!
//! theorem append_nil : ∀ xs : List α, append xs [] = xs := by
//!   intro xs
//!   induction xs with
//!   | nil => rfl
//!   | cons x xs ih => simp [ih]
//! ```
//!
//! ### Example 3: Type Class Instance
//!
//! ```ignore
//! instance : Eq (List Nat) where
//!   eq xs ys := (append xs [] = append ys [])
//!   refl xs := rfl
//!   ...
//! ```
//!
//! ### Example 4: Pattern Matching
//!
//! ```ignore
//! def is_zero : Nat → Bool
//!   | 0 => true
//!   | _ + 1 => false
//!
//! def head {α : Type u} : List α → Option α
//!   | [] => none
//!   | x :: _ => some x
//! ```
//!
//! ## Environment Building
//!
//! The standard library is built in phases:
//!
//! 1. **Phase 1: Primitives** — Nat, Bool, basic operations
//! 2. **Phase 2: Collections** — List, Array, Set
//! 3. **Phase 3: Type Classes** — Eq, Show, etc.
//! 4. **Phase 4: Theorems** — Induction principles, lemmas
//! 5. **Phase 5: Automation** — Tactic lemmas, decision procedures
//!
//! Loading happens in `oxilean-std` via `build_*_env` functions:
//!
//! ```ignore
//! let env = Environment::new();
//! let env = build_nat_env(env)?;
//! let env = build_list_env(env)?;
//! let env = build_eq_env(env)?;
//! // etc...
//! ```
//!
//! ## Integration with Elaborator & Tactics
//!
//! ### Decidability
//!
//! The `norm_num` and `decide` tactics use decidable instances:
//! - `2 + 3 = 5` via `Decidable` instance on Nat equality
//! - `decide` extracts this decision to reduce proofs automatically
//!
//! ### Simplification
//!
//! The `simp` tactic uses lemmas from the standard library:
//! - Reflexivity: `x = x`
//! - Commutativity: `x + y = y + x`
//! - Associativity: `(x + y) + z = x + (y + z)`
//!
//! ### Induction
//!
//! The `induction` tactic uses recursors and induction principles:
//! - For `Nat`: Decompose into zero/successor cases
//! - For `List`: Decompose into nil/cons cases
//!
//! ## Performance & Memory
//!
//! - **Lazy evaluation**: Some computations deferred via `Thunk`
//! - **Persistent data structures**: Lists/sets use structural sharing
//! - **Universe polymorphism**: Generic types work across universe levels
//!
//! ## Further Reading
//!
//! - [ARCHITECTURE.md](../../ARCHITECTURE.md) — System architecture
//! - [BLUEPRINT.md](../../BLUEPRINT.md) — Formal specifications
//! - Module documentation for specific type definitions

#![allow(missing_docs)]
#![warn(clippy::all)]
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
#![allow(clippy::map_unwrap_or)]
#![allow(clippy::map_unwrap_or)]
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
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::duplicated_attributes)]
#![allow(clippy::match_like_matches_macro)]
#![allow(clippy::manual_clamp)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::borrowed_box)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::manual_range_contains)]
#![allow(clippy::if_same_then_else)]
#![allow(clippy::option_map_or_none)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::map_clone)]
#![allow(clippy::unnecessary_cast)]
#![allow(unused_imports)]
#![allow(dead_code)]
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

pub mod abstract_algebra_adv;
pub mod abstract_interpretation;
pub mod algebra;
pub mod algebraic_combinatorics;
pub mod algebraic_effects;
pub mod algebraic_geometry;
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
pub mod combinatorial_game_theory;
pub mod combinatorial_optimization;
pub mod combinatorics;
pub mod complex;
pub mod complexity;
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
pub mod graph;
pub mod groebner_bases;
pub mod group_theory;
pub mod hashmap;
pub mod hashset;
pub mod higher_category_theory;
pub mod homological_algebra;
pub mod homological_computations;
pub mod homotopy_theory;
pub mod homotopy_type_theory;
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
pub mod number_theory;
pub mod numerical_analysis;
pub mod numerical_linear_algebra;
pub mod operations_research;
pub mod operator_algebras;
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

// ============================================================================
// Environment Builder Registry & Orchestration
// ============================================================================

/// Represents a phase in the standard library build process.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuildPhase {
    /// Phase 1: Primitive types (Nat, Bool, Char, etc.)
    Primitives,
    /// Phase 2: Collection types (List, Array, Set, etc.)
    Collections,
    /// Phase 3: Type class definitions (Eq, Ord, Show, etc.)
    TypeClasses,
    /// Phase 4: Core theorems and lemmas.
    Theorems,
    /// Phase 5: Automation (tactic lemmas, decision procedures).
    Automation,
}

impl BuildPhase {
    /// Returns all phases in build order.
    #[allow(dead_code)]
    pub fn all_in_order() -> &'static [BuildPhase] {
        &[
            BuildPhase::Primitives,
            BuildPhase::Collections,
            BuildPhase::TypeClasses,
            BuildPhase::Theorems,
            BuildPhase::Automation,
        ]
    }

    /// Returns a human-readable name for this phase.
    #[allow(dead_code)]
    pub fn name(&self) -> &'static str {
        match self {
            BuildPhase::Primitives => "primitives",
            BuildPhase::Collections => "collections",
            BuildPhase::TypeClasses => "type_classes",
            BuildPhase::Theorems => "theorems",
            BuildPhase::Automation => "automation",
        }
    }
}

/// A registry entry describing one standard library module.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StdModuleEntry {
    /// Fully qualified module name.
    pub qualified_name: &'static str,
    /// Build phase this module belongs to.
    pub phase: BuildPhase,
    /// Whether this module is loaded by default.
    pub default_load: bool,
    /// Brief description of module purpose.
    pub description: &'static str,
}

/// Inventory of all standard library modules.
#[allow(dead_code)]
pub static STD_MODULE_REGISTRY: &[StdModuleEntry] = &[
    StdModuleEntry {
        qualified_name: "Std.Nat",
        phase: BuildPhase::Primitives,
        default_load: true,
        description: "Natural number type",
    },
    StdModuleEntry {
        qualified_name: "Std.Bool",
        phase: BuildPhase::Primitives,
        default_load: true,
        description: "Boolean type",
    },
    StdModuleEntry {
        qualified_name: "Std.Char",
        phase: BuildPhase::Primitives,
        default_load: true,
        description: "Unicode character type",
    },
    StdModuleEntry {
        qualified_name: "Std.Int",
        phase: BuildPhase::Primitives,
        default_load: true,
        description: "Signed integer type",
    },
    StdModuleEntry {
        qualified_name: "Std.String",
        phase: BuildPhase::Primitives,
        default_load: true,
        description: "Immutable string type",
    },
    StdModuleEntry {
        qualified_name: "Std.List",
        phase: BuildPhase::Collections,
        default_load: true,
        description: "Linked list type",
    },
    StdModuleEntry {
        qualified_name: "Std.Array",
        phase: BuildPhase::Collections,
        default_load: true,
        description: "Fixed-size arrays",
    },
    StdModuleEntry {
        qualified_name: "Std.Set",
        phase: BuildPhase::Collections,
        default_load: false,
        description: "Mathematical sets",
    },
    StdModuleEntry {
        qualified_name: "Std.Fin",
        phase: BuildPhase::Collections,
        default_load: true,
        description: "Finite sets",
    },
    StdModuleEntry {
        qualified_name: "Std.Vec",
        phase: BuildPhase::Collections,
        default_load: false,
        description: "Length-indexed vectors",
    },
    StdModuleEntry {
        qualified_name: "Std.Option",
        phase: BuildPhase::Collections,
        default_load: true,
        description: "Optional value type",
    },
    StdModuleEntry {
        qualified_name: "Std.Result",
        phase: BuildPhase::Collections,
        default_load: true,
        description: "Result type",
    },
    StdModuleEntry {
        qualified_name: "Std.Prod",
        phase: BuildPhase::Collections,
        default_load: true,
        description: "Product type",
    },
    StdModuleEntry {
        qualified_name: "Std.Sum",
        phase: BuildPhase::Collections,
        default_load: true,
        description: "Sum type",
    },
    StdModuleEntry {
        qualified_name: "Std.Sigma",
        phase: BuildPhase::Collections,
        default_load: false,
        description: "Dependent pair type",
    },
    StdModuleEntry {
        qualified_name: "Std.Eq",
        phase: BuildPhase::TypeClasses,
        default_load: true,
        description: "Equality type class",
    },
    StdModuleEntry {
        qualified_name: "Std.Ord",
        phase: BuildPhase::TypeClasses,
        default_load: true,
        description: "Total ordering type class",
    },
    StdModuleEntry {
        qualified_name: "Std.Show",
        phase: BuildPhase::TypeClasses,
        default_load: false,
        description: "String representation",
    },
    StdModuleEntry {
        qualified_name: "Std.Functor",
        phase: BuildPhase::TypeClasses,
        default_load: true,
        description: "Functor type class",
    },
    StdModuleEntry {
        qualified_name: "Std.Monad",
        phase: BuildPhase::TypeClasses,
        default_load: false,
        description: "Monad type class",
    },
    StdModuleEntry {
        qualified_name: "Std.Decidable",
        phase: BuildPhase::TypeClasses,
        default_load: true,
        description: "Decidable propositions",
    },
    StdModuleEntry {
        qualified_name: "Std.Algebra",
        phase: BuildPhase::TypeClasses,
        default_load: false,
        description: "Algebraic structures",
    },
    StdModuleEntry {
        qualified_name: "Std.Logic",
        phase: BuildPhase::Theorems,
        default_load: true,
        description: "Classical logic",
    },
    StdModuleEntry {
        qualified_name: "Std.Prop",
        phase: BuildPhase::Theorems,
        default_load: true,
        description: "Propositional theorems",
    },
    StdModuleEntry {
        qualified_name: "Std.Order",
        phase: BuildPhase::Theorems,
        default_load: false,
        description: "Order theorems",
    },
    StdModuleEntry {
        qualified_name: "Std.TacticLemmas",
        phase: BuildPhase::Automation,
        default_load: true,
        description: "Tactic lemmas",
    },
    StdModuleEntry {
        qualified_name: "Std.WellFounded",
        phase: BuildPhase::Automation,
        default_load: false,
        description: "Well-founded relations",
    },
];

/// Get all module entries for a specific build phase.
#[allow(dead_code)]
pub fn modules_for_phase(phase: BuildPhase) -> Vec<&'static StdModuleEntry> {
    STD_MODULE_REGISTRY
        .iter()
        .filter(|e| e.phase == phase)
        .collect()
}

/// Get all default-loaded modules.
#[allow(dead_code)]
pub fn default_modules() -> Vec<&'static StdModuleEntry> {
    STD_MODULE_REGISTRY
        .iter()
        .filter(|e| e.default_load)
        .collect()
}

/// Get all modules.
#[allow(dead_code)]
pub fn all_modules() -> &'static [StdModuleEntry] {
    STD_MODULE_REGISTRY
}

/// Count how many modules are loaded by default.
#[allow(dead_code)]
pub fn count_default_modules() -> usize {
    STD_MODULE_REGISTRY
        .iter()
        .filter(|e| e.default_load)
        .count()
}

/// Count total number of registered standard library modules.
#[allow(dead_code)]
pub fn count_total_modules() -> usize {
    STD_MODULE_REGISTRY.len()
}

/// Look up a module entry by its qualified name.
#[allow(dead_code)]
pub fn find_module(qualified_name: &str) -> Option<&'static StdModuleEntry> {
    STD_MODULE_REGISTRY
        .iter()
        .find(|e| e.qualified_name == qualified_name)
}

/// A dependency pair: (dependent, dependency).
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct ModuleDep {
    /// The module that depends on another.
    pub dependent: &'static str,
    /// The module that must be built first.
    pub dependency: &'static str,
}

/// Minimal dependency graph for core standard library modules.
#[allow(dead_code)]
pub static CORE_DEPS: &[ModuleDep] = &[
    ModuleDep {
        dependent: "Std.List",
        dependency: "Std.Nat",
    },
    ModuleDep {
        dependent: "Std.Array",
        dependency: "Std.Nat",
    },
    ModuleDep {
        dependent: "Std.Fin",
        dependency: "Std.Nat",
    },
    ModuleDep {
        dependent: "Std.Vec",
        dependency: "Std.List",
    },
    ModuleDep {
        dependent: "Std.Set",
        dependency: "Std.List",
    },
    ModuleDep {
        dependent: "Std.Ord",
        dependency: "Std.Eq",
    },
    ModuleDep {
        dependent: "Std.Functor",
        dependency: "Std.Eq",
    },
    ModuleDep {
        dependent: "Std.Monad",
        dependency: "Std.Functor",
    },
    ModuleDep {
        dependent: "Std.Algebra",
        dependency: "Std.Eq",
    },
    ModuleDep {
        dependent: "Std.Logic",
        dependency: "Std.Bool",
    },
    ModuleDep {
        dependent: "Std.Prop",
        dependency: "Std.Logic",
    },
    ModuleDep {
        dependent: "Std.Order",
        dependency: "Std.Ord",
    },
    ModuleDep {
        dependent: "Std.TacticLemmas",
        dependency: "Std.Nat",
    },
    ModuleDep {
        dependent: "Std.WellFounded",
        dependency: "Std.Nat",
    },
];

/// Get all dependencies of a named module (direct only).
#[allow(dead_code)]
pub fn direct_deps(module: &str) -> Vec<&'static str> {
    CORE_DEPS
        .iter()
        .filter(|d| d.dependent == module)
        .map(|d| d.dependency)
        .collect()
}

/// Get all modules that depend on the given module.
#[allow(dead_code)]
pub fn dependents_of(module: &str) -> Vec<&'static str> {
    CORE_DEPS
        .iter()
        .filter(|d| d.dependency == module)
        .map(|d| d.dependent)
        .collect()
}

/// OxiLean standard library version string.
#[allow(dead_code)]
pub const STD_VERSION: &str = "0.1.1";

/// Feature flags for optional standard library components.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct StdFeatures {
    /// Enable classical logic axioms (excluded middle, choice).
    pub classical: bool,
    /// Enable propext (propositional extensionality).
    pub propext: bool,
    /// Enable funext (function extensionality).
    pub funext: bool,
    /// Enable quotient types.
    pub quotient: bool,
    /// Enable experimental category theory module.
    pub category_theory: bool,
    /// Enable topology module.
    pub topology: bool,
    /// Enable real number support.
    pub reals: bool,
}

impl StdFeatures {
    /// Create the default feature set (classical logic enabled by default).
    #[allow(dead_code)]
    pub fn default_features() -> Self {
        Self {
            classical: true,
            propext: true,
            funext: true,
            quotient: false,
            category_theory: false,
            topology: false,
            reals: false,
        }
    }

    /// Create the full feature set.
    #[allow(dead_code)]
    pub fn full() -> Self {
        Self {
            classical: true,
            propext: true,
            funext: true,
            quotient: true,
            category_theory: true,
            topology: true,
            reals: true,
        }
    }

    /// Count how many features are enabled.
    #[allow(dead_code)]
    pub fn count_enabled(&self) -> usize {
        [
            self.classical,
            self.propext,
            self.funext,
            self.quotient,
            self.category_theory,
            self.topology,
            self.reals,
        ]
        .iter()
        .filter(|&&v| v)
        .count()
    }
}

/// Standard library module statistics.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct StdLibStats {
    /// Total modules registered.
    pub total_modules: usize,
    /// Modules enabled by default.
    pub default_modules: usize,
    /// Modules per build phase.
    pub per_phase: [usize; 5],
}

impl StdLibStats {
    /// Compute statistics from the registry.
    #[allow(dead_code)]
    pub fn compute() -> Self {
        let total = count_total_modules();
        let defaults = count_default_modules();
        let phases = [
            modules_for_phase(BuildPhase::Primitives).len(),
            modules_for_phase(BuildPhase::Collections).len(),
            modules_for_phase(BuildPhase::TypeClasses).len(),
            modules_for_phase(BuildPhase::Theorems).len(),
            modules_for_phase(BuildPhase::Automation).len(),
        ];
        Self {
            total_modules: total,
            default_modules: defaults,
            per_phase: phases,
        }
    }

    /// Check if all phases have at least one module.
    #[allow(dead_code)]
    pub fn all_phases_populated(&self) -> bool {
        self.per_phase.iter().all(|&n| n > 0)
    }

    /// Get total modules across all phases.
    #[allow(dead_code)]
    pub fn phase_total(&self) -> usize {
        self.per_phase.iter().sum()
    }
}

#[cfg(test)]
mod std_lib_tests {
    use super::*;

    #[test]
    fn test_module_registry_not_empty() {
        assert!(!STD_MODULE_REGISTRY.is_empty());
        assert!(count_total_modules() > 10);
    }

    #[test]
    fn test_default_modules_subset() {
        let defaults = default_modules();
        assert!(!defaults.is_empty());
        assert!(defaults.len() <= count_total_modules());
    }

    #[test]
    fn test_modules_for_phase() {
        let primitives = modules_for_phase(BuildPhase::Primitives);
        assert!(!primitives.is_empty());
        for m in &primitives {
            assert_eq!(m.phase, BuildPhase::Primitives);
        }
    }

    #[test]
    fn test_find_module_existing() {
        let m = find_module("Std.Nat");
        assert!(m.is_some());
        assert_eq!(m.expect("m should be valid").phase, BuildPhase::Primitives);
    }

    #[test]
    fn test_find_module_nonexistent() {
        assert!(find_module("Std.DoesNotExist").is_none());
    }

    #[test]
    fn test_direct_deps() {
        let deps = direct_deps("Std.List");
        assert!(deps.contains(&"Std.Nat"));
    }

    #[test]
    fn test_dependents_of() {
        let deps = dependents_of("Std.Nat");
        assert!(deps.contains(&"Std.List"));
    }

    #[test]
    fn test_build_phase_order() {
        let phases = BuildPhase::all_in_order();
        assert_eq!(phases.len(), 5);
        assert_eq!(phases[0], BuildPhase::Primitives);
        assert_eq!(phases[4], BuildPhase::Automation);
    }

    #[test]
    fn test_std_features_default() {
        let f = StdFeatures::default_features();
        assert!(f.classical);
        assert!(f.propext);
        assert!(!f.topology);
    }

    #[test]
    fn test_std_features_full() {
        let f = StdFeatures::full();
        assert!(f.classical && f.topology && f.reals && f.quotient);
        assert_eq!(f.count_enabled(), 7);
    }

    #[test]
    fn test_build_phase_names() {
        assert_eq!(BuildPhase::Primitives.name(), "primitives");
        assert_eq!(BuildPhase::Automation.name(), "automation");
    }

    #[test]
    fn test_count_default_modules() {
        let count = count_default_modules();
        assert!(count > 0);
        assert!(count <= count_total_modules());
    }

    #[test]
    fn test_std_lib_stats() {
        let stats = StdLibStats::compute();
        assert!(stats.total_modules > 0);
        assert!(stats.all_phases_populated());
    }

    #[test]
    fn test_all_modules_count() {
        assert_eq!(all_modules().len(), STD_MODULE_REGISTRY.len());
    }

    #[test]
    fn test_std_version_nonempty() {
        assert!(!STD_VERSION.is_empty());
    }
}

#[cfg(test)]
mod std_stats_tests {
    use super::*;

    #[test]
    fn test_std_lib_stats_compute() {
        let s = StdLibStats::compute();
        assert_eq!(s.total_modules, count_total_modules());
        assert!(s.all_phases_populated());
        assert!(s.phase_total() > 0);
    }

    #[test]
    fn test_std_lib_stats_phase_total() {
        let s = StdLibStats::compute();
        assert_eq!(s.phase_total(), s.total_modules);
    }

    #[test]
    fn test_module_descriptions_not_empty() {
        for m in STD_MODULE_REGISTRY {
            assert!(
                !m.description.is_empty(),
                "Module {} has empty description",
                m.qualified_name
            );
        }
    }
}

// ============================================================
// Topological sort for module dependency resolution
// ============================================================

/// Compute a topological ordering of modules based on `CORE_DEPS`.
///
/// Returns `None` if there is a cycle.
#[allow(dead_code)]
pub fn topological_sort_modules() -> Option<Vec<&'static str>> {
    let mut in_degree: std::collections::HashMap<&'static str, usize> =
        std::collections::HashMap::new();
    let mut adjacency: std::collections::HashMap<&'static str, Vec<&'static str>> =
        std::collections::HashMap::new();

    // Initialize all modules
    for entry in STD_MODULE_REGISTRY {
        in_degree.entry(entry.qualified_name).or_insert(0);
        adjacency.entry(entry.qualified_name).or_default();
    }

    // Build graph from dependencies
    for dep in CORE_DEPS {
        *in_degree.entry(dep.dependent).or_insert(0) += 1;
        adjacency
            .entry(dep.dependency)
            .or_default()
            .push(dep.dependent);
    }

    // Kahn's algorithm
    let mut queue: std::collections::VecDeque<&'static str> = in_degree
        .iter()
        .filter(|(_, &d)| d == 0)
        .map(|(&n, _)| n)
        .collect();

    let mut sorted = Vec::new();
    while let Some(node) = queue.pop_front() {
        sorted.push(node);
        if let Some(neighbors) = adjacency.get(node) {
            for &neighbor in neighbors {
                let deg = in_degree.entry(neighbor).or_insert(0);
                *deg -= 1;
                if *deg == 0 {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    if sorted.len() == in_degree.len() {
        Some(sorted)
    } else {
        None // cycle detected
    }
}

#[cfg(test)]
mod topo_sort_tests {
    use super::*;

    #[test]
    fn test_topological_sort_acyclic() {
        let result = topological_sort_modules();
        assert!(result.is_some(), "Dependency graph should have no cycles");
        let sorted = result.expect("result should be valid");
        assert!(!sorted.is_empty());
    }

    #[test]
    fn test_nat_before_list() {
        let sorted = topological_sort_modules().expect("operation should succeed");
        let nat_pos = sorted.iter().position(|&s| s == "Std.Nat");
        let list_pos = sorted.iter().position(|&s| s == "Std.List");
        if let (Some(np), Some(lp)) = (nat_pos, list_pos) {
            assert!(np < lp, "Nat must appear before List");
        }
    }

    #[test]
    fn test_nat_before_array() {
        let sorted = topological_sort_modules().expect("operation should succeed");
        let nat_pos = sorted.iter().position(|&s| s == "Std.Nat");
        let arr_pos = sorted.iter().position(|&s| s == "Std.Array");
        if let (Some(np), Some(ap)) = (nat_pos, arr_pos) {
            assert!(np < ap);
        }
    }

    #[test]
    fn test_all_modules_in_sorted() {
        let sorted = topological_sort_modules().expect("operation should succeed");
        for entry in STD_MODULE_REGISTRY {
            assert!(
                sorted.contains(&entry.qualified_name),
                "Module {} missing from topological sort",
                entry.qualified_name
            );
        }
    }

    #[test]
    fn test_std_features_default_count() {
        let f = StdFeatures::default_features();
        // classical, propext, funext are true → 3 enabled
        assert_eq!(f.count_enabled(), 3);
    }

    #[test]
    fn test_module_dep_dependent_in_registry() {
        // Every dependent in CORE_DEPS should be in the registry
        let names: Vec<_> = STD_MODULE_REGISTRY
            .iter()
            .map(|e| e.qualified_name)
            .collect();
        for dep in CORE_DEPS {
            assert!(
                names.contains(&dep.dependent),
                "Dependent {} not in registry",
                dep.dependent
            );
        }
    }

    #[test]
    fn test_module_dep_dependency_in_registry() {
        let names: Vec<_> = STD_MODULE_REGISTRY
            .iter()
            .map(|e| e.qualified_name)
            .collect();
        for dep in CORE_DEPS {
            assert!(
                names.contains(&dep.dependency),
                "Dependency {} not in registry",
                dep.dependency
            );
        }
    }

    #[test]
    fn test_std_lib_stats_phase_count() {
        let s = StdLibStats::compute();
        assert_eq!(s.per_phase.len(), 5);
    }

    #[test]
    fn test_direct_deps_non_empty() {
        let deps = direct_deps("Std.Monad");
        assert!(!deps.is_empty());
    }
}

// ============================================================
// Extended standard library utilities
// ============================================================

/// A record of a single OxiLean standard library theorem or definition
/// that the elaborator knows about.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StdEntry {
    /// Qualified name (e.g., `Nat.add_comm`).
    pub name: &'static str,
    /// Which module this entry belongs to.
    pub module: &'static str,
    /// Human-readable description.
    pub description: &'static str,
    /// Whether this is a theorem (vs. a definition).
    pub is_theorem: bool,
}

/// A representative sample of well-known standard library entries.
#[allow(dead_code)]
pub const STD_KNOWN_ENTRIES: &[StdEntry] = &[
    StdEntry {
        name: "Nat.zero_add",
        module: "Std.Nat",
        description: "0 + n = n",
        is_theorem: true,
    },
    StdEntry {
        name: "Nat.add_zero",
        module: "Std.Nat",
        description: "n + 0 = n",
        is_theorem: true,
    },
    StdEntry {
        name: "Nat.add_comm",
        module: "Std.Nat",
        description: "Commutativity of natural number addition",
        is_theorem: true,
    },
    StdEntry {
        name: "Nat.add_assoc",
        module: "Std.Nat",
        description: "Associativity of natural number addition",
        is_theorem: true,
    },
    StdEntry {
        name: "Nat.mul_comm",
        module: "Std.Nat",
        description: "Commutativity of natural number multiplication",
        is_theorem: true,
    },
    StdEntry {
        name: "List.length_nil",
        module: "Std.List",
        description: "Length of the empty list is 0",
        is_theorem: true,
    },
    StdEntry {
        name: "List.length_cons",
        module: "Std.List",
        description: "Length of cons is 1 + length of tail",
        is_theorem: true,
    },
    StdEntry {
        name: "Bool.not_not",
        module: "Std.Bool",
        description: "Double negation elimination for Bool",
        is_theorem: true,
    },
    StdEntry {
        name: "Bool.and_comm",
        module: "Std.Bool",
        description: "Commutativity of boolean and",
        is_theorem: true,
    },
    StdEntry {
        name: "Option.some_ne_none",
        module: "Std.Option",
        description: "Some x is never None",
        is_theorem: true,
    },
];

/// Look up a standard library entry by its qualified name.
#[allow(dead_code)]
pub fn lookup_std_entry(name: &str) -> Option<&'static StdEntry> {
    STD_KNOWN_ENTRIES.iter().find(|e| e.name == name)
}

/// Return all entries from a given module.
#[allow(dead_code)]
pub fn entries_in_module(module: &str) -> Vec<&'static StdEntry> {
    STD_KNOWN_ENTRIES
        .iter()
        .filter(|e| e.module == module)
        .collect()
}

/// Return all theorems (non-definitions) in the standard library sample.
#[allow(dead_code)]
pub fn all_theorems() -> Vec<&'static StdEntry> {
    STD_KNOWN_ENTRIES.iter().filter(|e| e.is_theorem).collect()
}

/// A category tag for standard library modules.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StdCategory {
    /// Core arithmetic (Nat, Int).
    Arithmetic,
    /// Logic (Prop, And, Or, Not, Iff).
    Logic,
    /// Data structures (List, Array, Option, etc.).
    Data,
    /// Type classes (Eq, Ord, Functor, etc.).
    TypeClass,
    /// IO and system operations.
    IO,
    /// String operations.
    String,
    /// Tactics and proof automation.
    Tactic,
    /// Universe polymorphism.
    Universe,
}

impl StdCategory {
    /// Human-readable label.
    #[allow(dead_code)]
    pub fn label(self) -> &'static str {
        match self {
            Self::Arithmetic => "Arithmetic",
            Self::Logic => "Logic",
            Self::Data => "Data",
            Self::TypeClass => "TypeClass",
            Self::IO => "IO",
            Self::String => "String",
            Self::Tactic => "Tactic",
            Self::Universe => "Universe",
        }
    }
}

/// Map a module name to its `StdCategory`.
#[allow(dead_code)]
pub fn module_category(module: &str) -> StdCategory {
    if module.contains("Nat") || module.contains("Int") {
        StdCategory::Arithmetic
    } else if module.contains("Logic") || module.contains("Prop") {
        StdCategory::Logic
    } else if module.contains("List") || module.contains("Option") || module.contains("Array") {
        StdCategory::Data
    } else if module.contains("Functor") || module.contains("Monad") || module.contains("Eq") {
        StdCategory::TypeClass
    } else if module.contains("IO") {
        StdCategory::IO
    } else if module.contains("String") || module.contains("Char") {
        StdCategory::String
    } else if module.contains("Tactic") {
        StdCategory::Tactic
    } else {
        StdCategory::Universe
    }
}

/// Version information for the OxiLean standard library.
#[allow(dead_code)]
pub struct StdVersion {
    /// Major version number.
    pub major: u32,
    /// Minor version number.
    pub minor: u32,
    /// Patch version number.
    pub patch: u32,
    /// Pre-release label (empty for stable).
    pub pre: &'static str,
}

impl StdVersion {
    /// The current standard library version.
    #[allow(dead_code)]
    pub const CURRENT: StdVersion = StdVersion {
        major: 0,
        minor: 1,
        patch: 0,
        pre: "alpha",
    };

    /// Format as a semver string.
    #[allow(dead_code)]
    pub fn format_version(&self) -> String {
        self.to_string()
    }

    /// Check if this is a stable release.
    #[allow(dead_code)]
    pub fn is_stable(&self) -> bool {
        self.pre.is_empty()
    }
}
impl std::fmt::Display for StdVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.pre.is_empty() {
            write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
        } else {
            write!(
                f,
                "{}.{}.{}-{}",
                self.major, self.minor, self.patch, self.pre
            )
        }
    }
}

#[cfg(test)]
mod extra_std_tests {
    use super::*;

    #[test]
    fn test_lookup_std_entry_found() {
        let e = lookup_std_entry("Nat.add_comm");
        assert!(e.is_some());
        assert!(e.expect("e should be valid").is_theorem);
    }

    #[test]
    fn test_lookup_std_entry_not_found() {
        assert!(lookup_std_entry("Nonexistent.theorem").is_none());
    }

    #[test]
    fn test_entries_in_module() {
        let nat_entries = entries_in_module("Std.Nat");
        assert!(!nat_entries.is_empty());
        assert!(nat_entries.iter().all(|e| e.module == "Std.Nat"));
    }

    #[test]
    fn test_all_theorems_nonempty() {
        let thms = all_theorems();
        assert!(!thms.is_empty());
        assert!(thms.iter().all(|e| e.is_theorem));
    }

    #[test]
    fn test_module_category_nat() {
        assert_eq!(module_category("Std.Nat"), StdCategory::Arithmetic);
    }

    #[test]
    fn test_module_category_list() {
        assert_eq!(module_category("Std.List"), StdCategory::Data);
    }

    #[test]
    fn test_module_category_io() {
        assert_eq!(module_category("Std.IO"), StdCategory::IO);
    }

    #[test]
    fn test_std_version_to_string() {
        let v = StdVersion {
            major: 1,
            minor: 2,
            patch: 3,
            pre: "",
        };
        assert_eq!(v.to_string(), "1.2.3");
    }

    #[test]
    fn test_std_version_prerelease_to_string() {
        let v = StdVersion {
            major: 0,
            minor: 1,
            patch: 0,
            pre: "alpha",
        };
        assert_eq!(v.to_string(), "0.1.0-alpha");
    }

    #[test]
    fn test_std_version_is_stable_false() {
        assert!(!StdVersion::CURRENT.is_stable());
    }

    #[test]
    fn test_std_category_label() {
        assert_eq!(StdCategory::Arithmetic.label(), "Arithmetic");
        assert_eq!(StdCategory::Logic.label(), "Logic");
        assert_eq!(StdCategory::Data.label(), "Data");
    }

    #[test]
    fn test_std_known_entries_nonempty() {
        assert!(!STD_KNOWN_ENTRIES.is_empty());
    }

    #[test]
    fn test_all_entries_have_module() {
        for e in STD_KNOWN_ENTRIES {
            assert!(!e.module.is_empty(), "Entry {} has empty module", e.name);
        }
    }
}
pub mod abstract_algebra_advanced;
pub mod approximation_algorithms;
pub mod bayesian_networks;
pub mod bioinformatics;
pub mod birational_geometry;
pub mod coalgebra_theory;
pub mod compiler_theory;
pub mod convex_analysis;
pub mod cubical_type_theory;
pub mod differential_privacy;
pub mod distributed_systems_theory;
pub mod finite_element_method;
pub mod geometric_measure_theory;
pub mod harmonic_analysis;
pub mod information_theoretic_security;
pub mod intersection_theory;
pub mod knot_theory;
pub mod lambda_calculus;
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
pub mod type_inference_algorithms;
pub mod variational_analysis;

pub mod categorical_logic;
pub mod type_theory_advanced;

// ── Extended standard library utilities ──────────────────────────────────────

/// Utility type for carrying source-location metadata.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    /// Byte offset of the first character.
    pub start: usize,
    /// Byte offset one past the last character.
    pub end: usize,
    /// 1-based line number of the start.
    pub line: u32,
    /// 1-based column number of the start.
    pub column: u32,
}

impl Span {
    /// Create a new span.
    #[allow(dead_code)]
    pub fn new(start: usize, end: usize, line: u32, column: u32) -> Self {
        Self {
            start,
            end,
            line,
            column,
        }
    }

    /// Create a dummy span (all zeros).
    #[allow(dead_code)]
    pub fn dummy() -> Self {
        Self {
            start: 0,
            end: 0,
            line: 0,
            column: 0,
        }
    }

    /// Return the length in bytes.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }

    /// Return `true` if the span covers zero bytes.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.start >= self.end
    }

    /// Merge two spans: from the earlier start to the later end.
    #[allow(dead_code)]
    pub fn merge(&self, other: &Span) -> Span {
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
            line: self.line.min(other.line),
            column: self.column,
        }
    }
}

/// A value annotated with a `Span`.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Located<T> {
    /// The wrapped value.
    pub value: T,
    /// The source span.
    pub span: Span,
}

impl<T> Located<T> {
    /// Wrap `value` with a `span`.
    #[allow(dead_code)]
    pub fn new(value: T, span: Span) -> Self {
        Self { value, span }
    }

    /// Wrap `value` with a dummy span.
    #[allow(dead_code)]
    pub fn dummy(value: T) -> Self {
        Self {
            value,
            span: Span::dummy(),
        }
    }

    /// Map over the inner value.
    #[allow(dead_code)]
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Located<U> {
        Located {
            value: f(self.value),
            span: self.span,
        }
    }

    /// Return a reference to the inner value.
    #[allow(dead_code)]
    pub fn as_ref(&self) -> Located<&T> {
        Located {
            value: &self.value,
            span: self.span.clone(),
        }
    }
}

// ── Simple name-interning table ───────────────────────────────────────────────

/// A simple string-interning table backed by a `Vec`.
///
/// Useful for giving cheap `usize` IDs to string names during elaboration.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct NameTable {
    names: Vec<String>,
}

impl NameTable {
    /// Create an empty table.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Intern `name` and return its ID.  If already present, returns the
    /// existing ID without inserting a duplicate.
    #[allow(dead_code)]
    pub fn intern(&mut self, name: &str) -> usize {
        if let Some(pos) = self.names.iter().position(|n| n == name) {
            return pos;
        }
        let id = self.names.len();
        self.names.push(name.to_owned());
        id
    }

    /// Look up the string for an ID.
    #[allow(dead_code)]
    pub fn lookup(&self, id: usize) -> Option<&str> {
        self.names.get(id).map(String::as_str)
    }

    /// Return the number of interned names.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.names.len()
    }

    /// Return `true` if the table is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }

    /// Clear all entries.
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.names.clear();
    }

    /// Return an iterator over `(id, name)` pairs.
    #[allow(dead_code)]
    pub fn iter(&self) -> impl Iterator<Item = (usize, &str)> {
        self.names.iter().enumerate().map(|(i, s)| (i, s.as_str()))
    }
}

// ── DiagnosticLevel ──────────────────────────────────────────────────────────

/// Severity levels for compiler diagnostics.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiagnosticLevel {
    /// Informational note; does not prevent compilation.
    Note,
    /// Warning; compilation continues.
    Warning,
    /// Error; compilation should stop.
    Error,
    /// Internal compiler error (ICE).
    Bug,
}

impl DiagnosticLevel {
    /// Return a short label string.
    #[allow(dead_code)]
    pub fn label(self) -> &'static str {
        match self {
            Self::Note => "note",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Bug => "internal compiler error",
        }
    }

    /// Return `true` if this level prevents a successful build.
    #[allow(dead_code)]
    pub fn is_fatal(self) -> bool {
        matches!(self, Self::Error | Self::Bug)
    }
}

/// A single compiler diagnostic.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Diagnostic {
    /// Severity level.
    pub level: DiagnosticLevel,
    /// Human-readable message.
    pub message: String,
    /// Optional source span.
    pub span: Option<Span>,
    /// Optional help/hint text.
    pub help: Option<String>,
}

impl Diagnostic {
    /// Construct an error diagnostic.
    #[allow(dead_code)]
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            level: DiagnosticLevel::Error,
            message: message.into(),
            span: None,
            help: None,
        }
    }

    /// Construct a warning.
    #[allow(dead_code)]
    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            level: DiagnosticLevel::Warning,
            message: message.into(),
            span: None,
            help: None,
        }
    }

    /// Construct a note.
    #[allow(dead_code)]
    pub fn note(message: impl Into<String>) -> Self {
        Self {
            level: DiagnosticLevel::Note,
            message: message.into(),
            span: None,
            help: None,
        }
    }

    /// Attach a source span.
    #[allow(dead_code)]
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = Some(span);
        self
    }

    /// Attach a help string.
    #[allow(dead_code)]
    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }

    /// Return `true` if this diagnostic is fatal.
    #[allow(dead_code)]
    pub fn is_fatal(&self) -> bool {
        self.level.is_fatal()
    }
}

/// Accumulator for multiple diagnostics.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct DiagnosticBag {
    items: Vec<Diagnostic>,
}

impl DiagnosticBag {
    /// Create an empty bag.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Push a diagnostic.
    #[allow(dead_code)]
    pub fn push(&mut self, diag: Diagnostic) {
        self.items.push(diag);
    }

    /// Return `true` if there are any fatal diagnostics.
    #[allow(dead_code)]
    pub fn has_errors(&self) -> bool {
        self.items.iter().any(|d| d.is_fatal())
    }

    /// Return the number of accumulated diagnostics.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Return `true` if the bag is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Drain all diagnostics, returning them in order.
    #[allow(dead_code)]
    pub fn drain(&mut self) -> Vec<Diagnostic> {
        std::mem::take(&mut self.items)
    }

    /// Iterate over diagnostics.
    #[allow(dead_code)]
    pub fn iter(&self) -> impl Iterator<Item = &Diagnostic> {
        self.items.iter()
    }
}

// ── Simple scoped symbol table ────────────────────────────────────────────────

/// A scoped symbol table supporting nested scopes.
///
/// Each `push_scope` / `pop_scope` pair delimits a lexical scope.  Lookups
/// search from the innermost scope outward.
#[allow(dead_code)]
#[derive(Debug)]
pub struct ScopeTable<K, V> {
    scopes: Vec<Vec<(K, V)>>,
}

impl<K: Eq, V: Clone> ScopeTable<K, V> {
    /// Create a table with a single (global) scope.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            scopes: vec![Vec::new()],
        }
    }

    /// Push a new nested scope.
    #[allow(dead_code)]
    pub fn push_scope(&mut self) {
        self.scopes.push(Vec::new());
    }

    /// Pop the innermost scope, discarding its bindings.
    /// Panics if called on the root scope.
    #[allow(dead_code)]
    pub fn pop_scope(&mut self) {
        assert!(self.scopes.len() > 1, "cannot pop root scope");
        self.scopes.pop();
    }

    /// Bind `key` → `value` in the current (innermost) scope.
    #[allow(dead_code)]
    pub fn define(&mut self, key: K, value: V) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.push((key, value));
        }
    }

    /// Look up `key`, searching from innermost to outermost scope.
    #[allow(dead_code)]
    pub fn lookup(&self, key: &K) -> Option<&V> {
        for scope in self.scopes.iter().rev() {
            for (k, v) in scope.iter().rev() {
                if k == key {
                    return Some(v);
                }
            }
        }
        None
    }

    /// Return `true` if `key` is defined in the current scope only.
    #[allow(dead_code)]
    pub fn defined_locally(&self, key: &K) -> bool {
        if let Some(scope) = self.scopes.last() {
            scope.iter().any(|(k, _)| k == key)
        } else {
            false
        }
    }

    /// Current depth (1 = global scope only).
    #[allow(dead_code)]
    pub fn depth(&self) -> usize {
        self.scopes.len()
    }
}

impl<K: Eq, V: Clone> Default for ScopeTable<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

// ── Counter utilities ─────────────────────────────────────────────────────────

/// A monotonically increasing counter, useful for generating fresh variable IDs.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct Counter {
    next: u64,
}

impl Counter {
    /// Create a counter starting at zero.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a counter starting at `start`.
    #[allow(dead_code)]
    pub fn starting_at(start: u64) -> Self {
        Self { next: start }
    }

    /// Return the next value and advance the counter.
    #[allow(dead_code)]
    pub fn next(&mut self) -> u64 {
        let val = self.next;
        self.next += 1;
        val
    }

    /// Peek at the current value without advancing.
    #[allow(dead_code)]
    pub fn peek(&self) -> u64 {
        self.next
    }

    /// Reset the counter to zero.
    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.next = 0;
    }
}

// ── FreshName generator ───────────────────────────────────────────────────────

/// Generates fresh name strings of the form `prefix_N`.
#[allow(dead_code)]
#[derive(Debug)]
pub struct FreshNameGen {
    prefix: String,
    counter: Counter,
}

impl FreshNameGen {
    /// Create a generator with the given prefix.
    #[allow(dead_code)]
    pub fn new(prefix: impl Into<String>) -> Self {
        Self {
            prefix: prefix.into(),
            counter: Counter::new(),
        }
    }

    /// Return the next fresh name.
    #[allow(dead_code)]
    pub fn fresh(&mut self) -> String {
        let n = self.counter.next();
        format!("{}_{}", self.prefix, n)
    }

    /// Return the next fresh name as a `&'static str`-compatible owned `String`.
    #[allow(dead_code)]
    pub fn fresh_str(&mut self) -> String {
        self.fresh()
    }

    /// Reset the counter (reuse names — use with caution).
    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.counter.reset();
    }
}

// ── StringSet (ordered, for deterministic output) ────────────────────────────

/// A set of `String` values backed by a sorted `Vec` for deterministic output.
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct StringSet {
    items: Vec<String>,
}

impl StringSet {
    /// Create an empty set.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert `item`.  No-op if already present.  Returns `true` if new.
    #[allow(dead_code)]
    pub fn insert(&mut self, item: impl Into<String>) -> bool {
        let s = item.into();
        match self.items.binary_search(&s) {
            Ok(_) => false,
            Err(pos) => {
                self.items.insert(pos, s);
                true
            }
        }
    }

    /// Return `true` if `item` is in the set.
    #[allow(dead_code)]
    pub fn contains(&self, item: &str) -> bool {
        self.items
            .binary_search_by_key(&item, String::as_str)
            .is_ok()
    }

    /// Remove `item`.  Returns `true` if it was present.
    #[allow(dead_code)]
    pub fn remove(&mut self, item: &str) -> bool {
        match self.items.binary_search_by_key(&item, String::as_str) {
            Ok(pos) => {
                self.items.remove(pos);
                true
            }
            Err(_) => false,
        }
    }

    /// Return the number of elements.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Return `true` if empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Iterate over items in sorted order.
    #[allow(dead_code)]
    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.items.iter().map(String::as_str)
    }

    /// Compute the union of `self` and `other`.
    #[allow(dead_code)]
    pub fn union(&self, other: &StringSet) -> StringSet {
        let mut result = self.clone();
        for item in other.iter() {
            result.insert(item);
        }
        result
    }

    /// Compute the intersection of `self` and `other`.
    #[allow(dead_code)]
    pub fn intersection(&self, other: &StringSet) -> StringSet {
        let mut result = StringSet::new();
        for item in self.iter() {
            if other.contains(item) {
                result.insert(item);
            }
        }
        result
    }

    /// Compute the difference `self \ other`.
    #[allow(dead_code)]
    pub fn difference(&self, other: &StringSet) -> StringSet {
        let mut result = StringSet::new();
        for item in self.iter() {
            if !other.contains(item) {
                result.insert(item);
            }
        }
        result
    }
}

// ── Multi-map ─────────────────────────────────────────────────────────────────

/// A simple multi-map: each key may map to multiple values.
#[allow(dead_code)]
#[derive(Debug)]
pub struct MultiMap<K, V> {
    inner: Vec<(K, Vec<V>)>,
}

impl<K, V> Default for MultiMap<K, V> {
    fn default() -> Self {
        Self { inner: Vec::new() }
    }
}

impl<K: Eq, V> MultiMap<K, V> {
    /// Create an empty multi-map.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a `(key, value)` pair.
    #[allow(dead_code)]
    pub fn insert(&mut self, key: K, value: V) {
        for (k, vs) in &mut self.inner {
            if k == &key {
                vs.push(value);
                return;
            }
        }
        self.inner.push((key, vec![value]));
    }

    /// Return all values associated with `key`.
    #[allow(dead_code)]
    pub fn get(&self, key: &K) -> &[V] {
        for (k, vs) in &self.inner {
            if k == key {
                return vs;
            }
        }
        &[]
    }

    /// Return `true` if `key` has at least one associated value.
    #[allow(dead_code)]
    pub fn contains_key(&self, key: &K) -> bool {
        self.inner.iter().any(|(k, _)| k == key)
    }

    /// Return the number of distinct keys.
    #[allow(dead_code)]
    pub fn key_count(&self) -> usize {
        self.inner.len()
    }

    /// Remove all entries for `key`.  Returns the removed values.
    #[allow(dead_code)]
    pub fn remove(&mut self, key: &K) -> Vec<V> {
        let mut result = Vec::new();
        let mut i = 0;
        while i < self.inner.len() {
            if &self.inner[i].0 == key {
                let (_, vs) = self.inner.remove(i);
                result = vs;
            } else {
                i += 1;
            }
        }
        result
    }

    /// Iterate over `(key, values)` pairs.
    #[allow(dead_code)]
    pub fn iter(&self) -> impl Iterator<Item = (&K, &[V])> {
        self.inner.iter().map(|(k, vs)| (k, vs.as_slice()))
    }
}

// ── Trie (prefix tree) ────────────────────────────────────────────────────────

/// A simple trie mapping byte strings to values.
#[allow(dead_code)]
#[derive(Debug)]
pub struct Trie<V> {
    children: Vec<(u8, Trie<V>)>,
    value: Option<V>,
}

impl<V> Trie<V> {
    /// Create an empty trie node.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            value: None,
        }
    }

    /// Insert `key` → `value`.
    #[allow(dead_code)]
    pub fn insert(&mut self, key: &[u8], value: V) {
        if let Some((first, rest)) = key.split_first() {
            let child = self.get_or_create_child(*first);
            child.insert(rest, value);
        } else {
            self.value = Some(value);
        }
    }

    /// Look up `key` and return a reference to the associated value, if any.
    #[allow(dead_code)]
    pub fn get(&self, key: &[u8]) -> Option<&V> {
        if let Some((first, rest)) = key.split_first() {
            for (b, child) in &self.children {
                if *b == *first {
                    return child.get(rest);
                }
            }
            None
        } else {
            self.value.as_ref()
        }
    }

    /// Return `true` if `key` is present.
    #[allow(dead_code)]
    pub fn contains(&self, key: &[u8]) -> bool {
        self.get(key).is_some()
    }

    /// Return all keys with the given `prefix`.
    #[allow(dead_code)]
    pub fn keys_with_prefix(&self, prefix: &[u8]) -> Vec<Vec<u8>> {
        match prefix.split_first() {
            Some((first, rest)) => {
                for (b, child) in &self.children {
                    if *b == *first {
                        return child
                            .keys_with_prefix(rest)
                            .into_iter()
                            .map(|mut k| {
                                k.insert(0, *first);
                                k
                            })
                            .collect();
                    }
                }
                Vec::new()
            }
            None => self.collect_all(Vec::new()),
        }
    }

    fn get_or_create_child(&mut self, byte: u8) -> &mut Trie<V> {
        for i in 0..self.children.len() {
            if self.children[i].0 == byte {
                return &mut self.children[i].1;
            }
        }
        self.children.push((byte, Trie::new()));
        let last = self.children.len() - 1;
        &mut self.children[last].1
    }

    fn collect_all(&self, prefix: Vec<u8>) -> Vec<Vec<u8>> {
        let mut result = Vec::new();
        if self.value.is_some() {
            result.push(prefix.clone());
        }
        for (b, child) in &self.children {
            let mut p = prefix.clone();
            p.push(*b);
            result.extend(child.collect_all(p));
        }
        result
    }
}

impl<V> Default for Trie<V> {
    fn default() -> Self {
        Self::new()
    }
}

// ── BitSet (fixed-width 64-bit) ───────────────────────────────────────────────

/// A fixed-size bit set backed by a single `u64`.  Supports positions 0..63.
#[allow(dead_code)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BitSet64(u64);

impl BitSet64 {
    /// Empty set.
    #[allow(dead_code)]
    pub const fn empty() -> Self {
        Self(0)
    }

    /// Full set (all 64 bits set).
    #[allow(dead_code)]
    pub const fn full() -> Self {
        Self(u64::MAX)
    }

    /// Set the bit at `pos`.
    #[allow(dead_code)]
    pub fn set(&mut self, pos: u8) {
        debug_assert!(pos < 64);
        self.0 |= 1u64 << pos;
    }

    /// Clear the bit at `pos`.
    #[allow(dead_code)]
    pub fn clear(&mut self, pos: u8) {
        debug_assert!(pos < 64);
        self.0 &= !(1u64 << pos);
    }

    /// Test whether the bit at `pos` is set.
    #[allow(dead_code)]
    pub fn test(&self, pos: u8) -> bool {
        debug_assert!(pos < 64);
        (self.0 >> pos) & 1 == 1
    }

    /// Return the number of set bits.
    #[allow(dead_code)]
    pub fn count(&self) -> u32 {
        self.0.count_ones()
    }

    /// Return `true` if no bits are set.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Compute bitwise AND.
    #[allow(dead_code)]
    pub fn and(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    /// Compute bitwise OR.
    #[allow(dead_code)]
    pub fn or(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Compute bitwise XOR.
    #[allow(dead_code)]
    pub fn xor(self, other: Self) -> Self {
        Self(self.0 ^ other.0)
    }

    /// Compute bitwise NOT.
    #[allow(dead_code)]
    pub fn not(self) -> Self {
        Self(!self.0)
    }

    /// Iterate over set bit positions.
    #[allow(dead_code)]
    pub fn iter_ones(self) -> impl Iterator<Item = u8> {
        (0u8..64).filter(move |&i| self.test(i))
    }
}

// ── PriorityQueue ─────────────────────────────────────────────────────────────

/// A min-heap priority queue.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct MinHeap<P: Ord, V> {
    heap: Vec<(P, V)>,
}

impl<P: Ord, V> MinHeap<P, V> {
    /// Create an empty heap.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { heap: Vec::new() }
    }

    /// Push `(priority, value)` onto the heap.
    #[allow(dead_code)]
    pub fn push(&mut self, priority: P, value: V) {
        self.heap.push((priority, value));
        let mut i = self.heap.len() - 1;
        while i > 0 {
            let parent = (i - 1) / 2;
            if self.heap[parent].0 > self.heap[i].0 {
                self.heap.swap(parent, i);
                i = parent;
            } else {
                break;
            }
        }
    }

    /// Pop the minimum-priority element.
    #[allow(dead_code)]
    pub fn pop(&mut self) -> Option<(P, V)> {
        if self.heap.is_empty() {
            return None;
        }
        let n = self.heap.len();
        self.heap.swap(0, n - 1);
        let min = self.heap.pop();
        self.sift_down(0);
        min
    }

    /// Peek at the minimum-priority element without removing it.
    #[allow(dead_code)]
    pub fn peek(&self) -> Option<(&P, &V)> {
        self.heap.first().map(|(p, v)| (p, v))
    }

    /// Return the number of elements.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.heap.len()
    }

    /// Return `true` if empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    fn sift_down(&mut self, mut i: usize) {
        let n = self.heap.len();
        loop {
            let left = 2 * i + 1;
            let right = 2 * i + 2;
            let mut smallest = i;
            if left < n && self.heap[left].0 < self.heap[smallest].0 {
                smallest = left;
            }
            if right < n && self.heap[right].0 < self.heap[smallest].0 {
                smallest = right;
            }
            if smallest == i {
                break;
            }
            self.heap.swap(i, smallest);
            i = smallest;
        }
    }
}

// ── Graph (adjacency list) ────────────────────────────────────────────────────

/// A directed graph with `n` nodes, represented as an adjacency list.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DirectedGraph {
    adj: Vec<Vec<usize>>,
}

impl DirectedGraph {
    /// Create a graph with `n` nodes and no edges.
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        Self {
            adj: vec![Vec::new(); n],
        }
    }

    /// Add a directed edge `u → v`.
    #[allow(dead_code)]
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.adj[u].push(v);
    }

    /// Return the number of nodes.
    #[allow(dead_code)]
    pub fn node_count(&self) -> usize {
        self.adj.len()
    }

    /// Return the out-degree of node `u`.
    #[allow(dead_code)]
    pub fn out_degree(&self, u: usize) -> usize {
        self.adj[u].len()
    }

    /// Iterate over the successors of `u`.
    #[allow(dead_code)]
    pub fn successors(&self, u: usize) -> &[usize] {
        &self.adj[u]
    }

    /// Compute a topological ordering using Kahn's algorithm.
    /// Returns `None` if the graph contains a cycle.
    #[allow(dead_code)]
    pub fn topological_sort(&self) -> Option<Vec<usize>> {
        let n = self.adj.len();
        let mut in_deg = vec![0usize; n];
        for u in 0..n {
            for &v in &self.adj[u] {
                in_deg[v] += 1;
            }
        }
        let mut queue: std::collections::VecDeque<usize> =
            (0..n).filter(|&u| in_deg[u] == 0).collect();
        let mut order = Vec::new();
        while let Some(u) = queue.pop_front() {
            order.push(u);
            for &v in &self.adj[u] {
                in_deg[v] -= 1;
                if in_deg[v] == 0 {
                    queue.push_back(v);
                }
            }
        }
        if order.len() == n {
            Some(order)
        } else {
            None
        }
    }

    /// Compute strongly connected components using Kosaraju's algorithm.
    #[allow(dead_code)]
    pub fn strongly_connected_components(&self) -> Vec<Vec<usize>> {
        let n = self.adj.len();
        // Pass 1: finish-time order
        let mut visited = vec![false; n];
        let mut finish_order = Vec::new();
        for start in 0..n {
            if !visited[start] {
                self.dfs_finish(start, &mut visited, &mut finish_order);
            }
        }
        // Build reverse graph
        let mut rev = vec![Vec::new(); n];
        for u in 0..n {
            for &v in &self.adj[u] {
                rev[v].push(u);
            }
        }
        // Pass 2: assign SCCs in reverse finish order
        let mut comp = vec![usize::MAX; n];
        let mut scc_id = 0;
        for &start in finish_order.iter().rev() {
            if comp[start] == usize::MAX {
                let mut stack = vec![start];
                while let Some(u) = stack.pop() {
                    if comp[u] != usize::MAX {
                        continue;
                    }
                    comp[u] = scc_id;
                    for &v in &rev[u] {
                        if comp[v] == usize::MAX {
                            stack.push(v);
                        }
                    }
                }
                scc_id += 1;
            }
        }
        let mut sccs: Vec<Vec<usize>> = vec![Vec::new(); scc_id];
        for u in 0..n {
            sccs[comp[u]].push(u);
        }
        sccs
    }

    fn dfs_finish(&self, u: usize, visited: &mut Vec<bool>, order: &mut Vec<usize>) {
        let mut stack = vec![(u, 0usize)];
        while let Some((node, idx)) = stack.last_mut() {
            let _node = *node;
            if !visited[_node] {
                visited[_node] = true;
            }
            if *idx < self.adj[_node].len() {
                let next = self.adj[_node][*idx];
                *idx += 1;
                if !visited[next] {
                    stack.push((next, 0));
                }
            } else {
                order.push(_node);
                stack.pop();
            }
        }
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod lib_extended_tests {
    use super::*;

    #[test]
    fn test_span_merge() {
        let a = Span::new(0, 5, 1, 1);
        let b = Span::new(3, 10, 1, 4);
        let m = a.merge(&b);
        assert_eq!(m.start, 0);
        assert_eq!(m.end, 10);
    }

    #[test]
    fn test_located_map() {
        let l = Located::dummy(42u32);
        let l2 = l.map(|x| x * 2);
        assert_eq!(l2.value, 84);
    }

    #[test]
    fn test_name_table() {
        let mut t = NameTable::new();
        let id_a = t.intern("alpha");
        let id_b = t.intern("beta");
        let id_a2 = t.intern("alpha");
        assert_eq!(id_a, id_a2);
        assert_ne!(id_a, id_b);
        assert_eq!(t.lookup(id_a), Some("alpha"));
        assert_eq!(t.len(), 2);
    }

    #[test]
    fn test_diagnostic_bag() {
        let mut bag = DiagnosticBag::new();
        assert!(!bag.has_errors());
        bag.push(Diagnostic::warning("minor issue"));
        assert!(!bag.has_errors());
        bag.push(Diagnostic::error("fatal problem"));
        assert!(bag.has_errors());
        assert_eq!(bag.len(), 2);
        let drained = bag.drain();
        assert_eq!(drained.len(), 2);
        assert!(bag.is_empty());
    }

    #[test]
    fn test_scope_table() {
        let mut s: ScopeTable<&str, u32> = ScopeTable::new();
        s.define("x", 1);
        s.push_scope();
        s.define("x", 2);
        assert_eq!(s.lookup(&"x"), Some(&2));
        s.pop_scope();
        assert_eq!(s.lookup(&"x"), Some(&1));
    }

    #[test]
    fn test_counter_and_fresh_name() {
        let mut c = Counter::new();
        assert_eq!(c.next(), 0);
        assert_eq!(c.next(), 1);
        assert_eq!(c.peek(), 2);
        c.reset();
        assert_eq!(c.next(), 0);

        let mut gen = FreshNameGen::new("var");
        let n0 = gen.fresh();
        let n1 = gen.fresh();
        assert_eq!(n0, "var_0");
        assert_eq!(n1, "var_1");
    }

    #[test]
    fn test_string_set_operations() {
        let mut s = StringSet::new();
        assert!(s.insert("banana"));
        assert!(s.insert("apple"));
        assert!(!s.insert("apple")); // duplicate
        assert!(s.contains("apple"));
        assert!(!s.contains("cherry"));
        assert_eq!(s.len(), 2);
        assert!(s.remove("apple"));
        assert!(!s.contains("apple"));
        let mut t = StringSet::new();
        t.insert("cherry");
        t.insert("banana");
        let u = s.union(&t);
        assert!(u.contains("banana"));
        assert!(u.contains("cherry"));
    }

    #[test]
    fn test_multi_map() {
        let mut m: MultiMap<&str, u32> = MultiMap::new();
        m.insert("key", 1);
        m.insert("key", 2);
        m.insert("other", 3);
        assert_eq!(m.get(&"key"), &[1, 2]);
        assert_eq!(m.key_count(), 2);
        let removed = m.remove(&"key");
        assert_eq!(removed, vec![1, 2]);
        assert!(!m.contains_key(&"key"));
    }

    #[test]
    fn test_trie() {
        let mut t: Trie<u32> = Trie::new();
        t.insert(b"hello", 1);
        t.insert(b"help", 2);
        t.insert(b"world", 3);
        assert_eq!(t.get(b"hello"), Some(&1));
        assert_eq!(t.get(b"help"), Some(&2));
        assert!(t.get(b"helo").is_none());
        assert!(t.contains(b"world"));
        let pfx = t.keys_with_prefix(b"hel");
        assert_eq!(pfx.len(), 2);
    }

    #[test]
    fn test_bitset64() {
        let mut bs = BitSet64::empty();
        assert!(bs.is_empty());
        bs.set(5);
        bs.set(10);
        assert!(bs.test(5));
        assert!(bs.test(10));
        assert!(!bs.test(0));
        assert_eq!(bs.count(), 2);
        bs.clear(5);
        assert!(!bs.test(5));
        let ones: Vec<u8> = bs.iter_ones().collect();
        assert_eq!(ones, vec![10]);
    }

    #[test]
    fn test_min_heap() {
        let mut heap: MinHeap<u32, &str> = MinHeap::new();
        heap.push(5, "five");
        heap.push(1, "one");
        heap.push(3, "three");
        assert_eq!(heap.len(), 3);
        let (p, v) = heap.pop().expect("pop should succeed");
        assert_eq!(p, 1);
        assert_eq!(v, "one");
        let (p2, _) = heap.pop().expect("pop should succeed");
        assert_eq!(p2, 3);
    }

    #[test]
    fn test_directed_graph_topo_sort() {
        let mut g = DirectedGraph::new(4);
        g.add_edge(0, 1);
        g.add_edge(0, 2);
        g.add_edge(1, 3);
        g.add_edge(2, 3);
        let order = g.topological_sort().expect("should be a DAG");
        assert_eq!(order.len(), 4);
        // 0 must come before 1,2; 1 and 2 before 3
        let pos: Vec<usize> = {
            let mut p = vec![0usize; 4];
            for (i, &node) in order.iter().enumerate() {
                p[node] = i;
            }
            p
        };
        assert!(pos[0] < pos[1]);
        assert!(pos[0] < pos[2]);
        assert!(pos[1] < pos[3]);
        assert!(pos[2] < pos[3]);
    }

    #[test]
    fn test_directed_graph_scc() {
        // 0 → 1 → 2 → 0, 3 (separate)
        let mut g = DirectedGraph::new(4);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 0);
        // node 3 is isolated
        let sccs = g.strongly_connected_components();
        // Should have 2 SCCs: {0,1,2} and {3}
        assert_eq!(sccs.len(), 2);
    }

    #[test]
    fn test_diagnostic_level_ordering() {
        assert!(DiagnosticLevel::Note < DiagnosticLevel::Warning);
        assert!(DiagnosticLevel::Warning < DiagnosticLevel::Error);
        assert!(DiagnosticLevel::Error < DiagnosticLevel::Bug);
        assert!(DiagnosticLevel::Error.is_fatal());
        assert!(!DiagnosticLevel::Warning.is_fatal());
    }
}
