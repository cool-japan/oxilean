//! Auto-generated module structure

pub mod functions;
pub mod patterncompiler_analyze_usefulness_group;
pub mod patterncompiler_bound_var_set_group;
pub mod patterncompiler_check_methods;
pub mod patterncompiler_collect_bound_names_group;
pub mod patterncompiler_collect_ctors_from_pattern_group;
pub mod patterncompiler_collect_literal_set_group;
pub mod patterncompiler_collect_literals_group;
pub mod patterncompiler_collect_pattern_ctors_group;
pub mod patterncompiler_count_bindings_group;
pub mod patterncompiler_flatten_or_pattern_group;
pub mod patterncompiler_fresh_var_group;
pub mod patterncompiler_max_pattern_depth_group;
pub mod patterncompiler_pattern_to_string_group;
pub mod patterncompiler_patterns_equivalent_group;
pub mod patterncompiler_predicates;
pub mod patterncompiler_queries;
pub mod patterncompiler_select_column_group;
pub mod patterncompiler_traits;
pub mod patterncompiler_type;
pub mod patterncoverageext_traits;
pub mod patternrenamer_traits;
pub mod types;

// Re-export all types
pub use functions::*;
pub use patterncompiler_type::*;
pub use types::*;
