#![allow(missing_docs)]
//! Tactic framework for interactive proof construction.
//!
//! Provides the core tactic infrastructure and a library of basic tactics
//! that mirror LEAN 4's tactic mode.
//!
//! ## Architecture
//!
//! Tactics operate on a `TacticState` which manages a list of goals
//! (metavariables to be filled). Each tactic transforms the state by
//! assigning some metavariables and potentially creating new ones.

// --- Batch 4.3: Core Tactics ---
pub mod cases;
pub mod constructor;
pub mod core;
pub mod rewrite;
pub mod state;
pub mod structural;

// --- Batch 4.4: Advanced Tactics ---
pub mod calc;
pub mod omega;
pub mod simp;

// --- Batch 4.5: Specialized Tactics ---
pub mod ext;
pub mod norm_num;
pub mod ring;
pub mod solve_by_elim;

// --- Batch 4.6: Proof Search Tactics ---
pub mod aesop;
pub mod library_search;

// --- Batch 4.7: Recursive Destructuring Tactics ---
pub mod rcases;

// --- Batch 4.8: Advanced Induction Tactics ---
pub mod induction_adv;

// --- Batch 4.10: E-matching / Congruence Closure Prover ---
pub mod grind;

// --- Batch 4.9: Bit-Vector Decision Procedure ---
pub mod bvdecide;

// --- Phase 12: Additional Tactics ---
pub mod conv_mode;
pub mod injection;
pub mod monotonicity;
pub mod norm_cast;
pub mod smt;

// --- Phase 13: Decidability, Congruence, Positivity, Linear Combination ---
pub mod congr;
pub mod decide;
pub mod linear_combination;
pub mod positivity;

// --- Phase 14: Fun_prop, Polyrith, Tauto ---
pub mod apply_rules;
pub mod fun_prop;
pub mod gcongr;
pub mod polyrith;
pub mod tauto;

// --- Phase 15: New Tactics (field_simp, abel, group, slim_check, convert) ---
pub mod abel;
pub mod convert;
pub mod field_simp;
pub mod group;
pub mod slim_check;

// --- Extracted submodules (splitrs) ---
pub mod functions;
pub mod functions_2;
pub mod types;

// --- Re-exports: Batch 4.3 ---
pub use cases::{tac_cases, tac_induction, CasesResult, InductionResult};
pub use constructor::{tac_constructor, tac_existsi, tac_left, tac_right};
pub use core::{
    tac_apply, tac_assumption, tac_exact, tac_intro, tac_intros, tac_refine, tac_trivial,
};
pub use rewrite::{tac_rewrite, RewriteDirection};
pub use state::{GoalView, TacticError, TacticResult, TacticState};
pub use structural::{tac_clear, tac_revert, tac_subst};

// --- Re-exports: Batch 4.4 ---
pub use calc::{enter_conv, tac_calc, CalcProof, CalcStep, ConvSide};
pub use omega::{
    gcd, is_satisfiable, is_unsatisfiable, lcm, solve_omega, solve_omega_with_config, tac_omega,
    LinearConstraint, LinearExpr, LinearTerm, OmegaConfig, OmegaProof, OmegaResult, OmegaSolver,
    OmegaStep,
};
pub use simp::discharge::DischargeStrategy;
pub use simp::types::{default_simp_lemmas, SimpConfig, SimpLemma, SimpResult, SimpTheorems};

// --- Re-exports: Batch 4.5 ---
pub use ext::{
    tac_ext, tac_ext_with_config, tac_funext, tac_propext, ExtConfig, ExtLemma, ExtLemmaRegistry,
    ExtResult, RegistrySummary,
};
pub use norm_num::{tac_norm_num, ComparisonOp, NumericValue};
pub use ring::{tac_ring, Monomial, Polynomial};
pub use solve_by_elim::{
    solve_by_elim_with_stats, tac_solve_by_elim, tac_solve_by_elim_with_config, BacktrackState,
    CandidateSource, SearchStats, SolveByElimConfig, SolveByElimResult,
};

// --- Re-exports: Batch 4.6 ---
pub use aesop::{
    tac_aesop, tac_aesop_with_config, tac_aesop_with_rules, AesopConfig, AesopResult, AesopRule,
    AesopRuleKind, AesopRuleSafety, AesopRuleSet, AesopSearchNode, AesopSearchState, AesopStats,
};
pub use library_search::{
    tac_exact_question, tac_library_search, CacheLookup, LemmaCandidate, LemmaIndex,
    LibrarySearchConfig, ScoringCriteria, SearchCache, SearchResult,
};

// --- Re-exports: Batch 4.7 ---
pub use rcases::{
    parse_rcases_pattern, tac_obtain, tac_rcases, tac_rcases_many, tac_rintro, ObtainResult,
    RcasesConfig, RcasesPattern, RcasesResult,
};

// --- Re-exports: Batch 4.8 ---
pub use induction_adv::{
    check_recursor_compatibility, infer_induction_scheme, tac_generalize, tac_induction_adv,
    tac_mutual_induction, tac_well_founded_induction, GeneralizationResult, InductionConfig,
    InductionScheme, MinorPremise, MutualInductionConfig, WellFoundedConfig,
};

// --- Re-exports: Batch 4.9 ---
pub use bvdecide::{
    bv_decide_with_stats, tac_bv_decide, tac_bv_decide_with_config, BitVec, BitWidth,
    BvDecideConfig, BvDecideStats, BvExpr, CdclSolver, CnfFormula, SatResult,
};

// --- Re-exports: Batch 4.10 ---
pub use grind::{
    check_nat_le_by_transitivity, extract_nat_constraints, grind_check_eq, grind_eq, grind_on_goal,
    grind_with_la, grind_with_stats, tac_grind, tac_grind_aggressive, tac_grind_with_config,
    try_parse_nat_constraint, CaseSplitter, CongruenceClosure, EClass, EClassId, EMatchCompiler,
    ENode, ENodeId, EPattern, EPatternNode, EqualityStep, GrindConfig, GrindProof, GrindResult,
    GrindState, GrindStats, MergeReason, NatConstraint, NatRelKind, ProofStep, SignatureTable,
    Substitution, TermIndex, UnionFind,
};

// --- Re-exports: Phase 12 ---
pub use conv_mode::{
    conv_arg, conv_ext, conv_lhs, conv_norm_num, conv_rhs, conv_ring, conv_rw, conv_simp,
    enter_conv as enter_conv_mode, exit_conv, run_conv_session, ConvConfig, ConvDirection,
    ConvEntrySide, ConvOperation, ConvPath, ConvResult, ConvState, ConvStats, ConvTarget,
};
pub use injection::{
    build_injection_proof, build_no_confusion_proof, decompose_constructor_eq, tac_injection,
    tac_injection_with, tac_no_confusion, InjectionConfig, InjectionResult, InjectionStats,
    NoConfusionResult,
};
pub use monotonicity::{
    combine_relations, count_rules, decompose_relation, generate_mono_goals,
    is_monotone_in_ruleset, monotone_functions_for_relation, structurally_compatible, tac_mono,
    tac_mono_with_config, tac_mono_with_rules, MonoChain, MonoConclusion, MonoConfig, MonoPremise,
    MonoRelation, MonoResult, MonoRule, MonoRuleSet, MonoStats,
};
pub use norm_cast::{
    find_cast_chain, tac_exact_mod_cast, tac_norm_cast, tac_pull_cast, tac_push_cast, CastConfig,
    CastDirection, CastLemma, CastLemmaSet, CastResult, CastStats, CastStep,
};

// --- Re-exports: Phase 13 (tac_* entry-point wrappers) ---
pub use decide::tac_decide;
pub use positivity::tac_positivity;

// --- Re-exports: Phase 14 (tac_* entry-point wrappers) ---
pub use fun_prop::{tac_continuity, tac_measurability};
pub use gcongr::tac_gcongr;
pub use polyrith::tac_polyrith;
pub use tauto::tac_tauto;

// --- Re-exports: Phase 15 ---
pub use abel::{
    abel_forms_equal, abel_to_expr, expr_to_abel, normalize_abel_term, tac_abel,
    tac_abel_with_config, AbelConfig, AbelNormalForm, AbelTerm,
};
pub use convert::{
    find_mismatches, tac_convert, tac_convert_with_config, ConvertConfig, ConvertResult,
};
pub use field_simp::{
    clear_denominator, field_simp_expr, find_division_patterns, normalize_fractions,
    tac_field_simp, tac_field_simp_with_config, DivisionPattern, FieldSimpConfig, FieldSimpResult,
};
pub use group::{
    expr_to_group_word, invert_word, reduce_word, reduce_word_with_config, tac_group,
    tac_group_with_config, words_equal, GroupConfig, GroupLetter, GroupWord,
};
pub use simp::simp_rw::{apply_rw_rules, tac_simp_rw, tac_simp_rw_with_iters, RwRule};
pub use slim_check::{
    extract_forall_vars, gen_bool, gen_int, gen_nat, lcg_rand, tac_slim_check,
    tac_slim_check_with_config, try_find_counterexample, Counterexample, ForallVar,
    SlimCheckConfig, SlimCheckOutcome, SlimCheckResult,
};

// --- Re-exports: extracted submodules ---
pub use functions::{
    block_has_sorry, closing_tactics, common_next_tactics, count_tactics_in_block, is_backward_rw,
    is_finishing_tactic, is_known_tactic, is_structural_tactic, lookup_tactic, most_used_tactic,
    registered_tactic_count, simp_only_lemmas, sort_by_priority, split_tactic_block,
    splitting_tactics, suggest_tactics, tactic_argument, tactic_block_stats, tactic_completions,
    tactic_default_priority, tactic_docs, tactic_invocation_counts, tactic_name, tactic_skeleton,
    validate_tactic_block, validate_tactic_brackets, TACTIC_REGISTRY, TACTIC_SEQUENCES,
};
pub use types::{
    ModExtConfig2700, ModExtConfigVal2700, ModExtDiag2700, ModExtDiff2700, ModExtPass2700,
    ModExtPipeline2700, ModExtResult2700, ProofStateSummary, TacModBuilder, TacModCounterMap,
    TacModExt, TacModExt2, TacModExtMap, TacModExtUtil, TacModState, TacModStateMachine,
    TacModWindow, TacModWorkQueue, TacticBlockStats, TacticEntry, TacticHint,
    TacticModAnalysisPass, TacticModConfig, TacticModConfigValue, TacticModDiagnostics,
    TacticModDiff, TacticModPipeline, TacticModResult, TacticOutcome, TacticPriority,
    TacticProfile, TacticSequence, TacticTiming,
};
