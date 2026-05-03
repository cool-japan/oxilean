//! Algebraic combinatorics: Young tableaux, symmetric functions, crystal graphs,
//! Robinson-Schensted correspondence, and combinatorial sequences.

pub mod functions;
pub mod functions_2;
pub mod types;
pub mod types_2;

// Re-export all types from original modules.
pub use functions::*;
pub use types::*;

// Re-export from extended modules (explicit to avoid ambiguous glob re-exports).
pub use functions_2::{
    ballot_sequence_count, beta_set_from_diagram, catalan_number, conjugate_diagram,
    coxeter_from_word, coxeter_length, descent_set, hook_length, hook_length_formula,
    inversion_number, kostka_number, lehmer_code, major_index, motzkin_number, narayana_number,
    number_of_syt, plethysm_multiplicity, rs_insertion, schur_polynomial_evaluation,
    young_diagram_from_partition,
};
pub use types_2::{
    BetaSet, CoxeterElement, DescentSet, Permutation, RobinsonSchensted, SchurPolynomial,
    SemistandardTableau, StandardTableau,
};

// Re-export YoungDiagram from types_2 as YoungDiagram2 to avoid ambiguity with
// the original YoungDiagram (which uses `parts` field, not `rows`).
pub use types_2::YoungDiagram as YoungDiagram2;
