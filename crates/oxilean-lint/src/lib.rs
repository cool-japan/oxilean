//! # OxiLean Lint -- Static Analysis and Lint Rules
//!
//! This crate provides a lint engine and a collection of lint rules for
//! analyzing OxiLean source code.

#![allow(dead_code)]
#![warn(clippy::all)]
#![allow(unused_imports)]
#![allow(clippy::field_reassign_with_default)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::single_match)]
#![allow(clippy::needless_ifs)]
#![allow(clippy::len_without_is_empty)]
#![allow(clippy::new_without_default)]
#![allow(clippy::inherent_to_string_shadow_display)]
#![allow(clippy::type_complexity)]
#![allow(clippy::manual_strip)]
#![allow(clippy::bool_comparison)]
#![allow(clippy::if_same_then_else)]
#![allow(clippy::implicit_saturating_sub)]
#![allow(clippy::int_plus_one)]
#![allow(clippy::manual_map)]
#![allow(clippy::needless_bool)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::manual_find)]
#![allow(clippy::for_kv_map)]
#![allow(clippy::enum_variant_names)]
#![allow(clippy::manual_range_contains)]
#![allow(clippy::to_string_in_format_args)]

pub mod autofix;
pub mod framework;
pub mod ide_integration;
pub mod mutation_testing;
pub mod plugin;
pub mod rules;

pub use framework::{
    AutoFix, LintConfig, LintContext, LintDiagnostic, LintEngine, LintId, LintRegistry, LintRule,
    LintSuppression, Severity,
};
pub use rules::{
    DeadCodeRule, DeprecatedApiRule, DeprecatedTacticRule, LongProofRule, MissingDocRule,
    MissingDocstringRule, NamingConventionRule, RedundantAssumptionRule, RedundantPatternRule,
    SimplifiableExprRule, StyleRule, UnreachableCodeRule, UnusedHypothesisRule, UnusedImportRule,
    UnusedVariableRule,
};

pub mod core_types;
pub use core_types::*;

pub mod dead_declaration;
pub mod proof_quality;
