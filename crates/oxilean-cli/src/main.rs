//! OxiLean CLI entry point.

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::field_reassign_with_default)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::single_match)]
#![allow(clippy::needless_ifs)]
#![allow(clippy::useless_format)]
#![allow(clippy::new_without_default)]
#![allow(clippy::manual_strip)]
#![allow(clippy::needless_borrows_for_generic_args)]
#![allow(clippy::manual_saturating_arithmetic)]
#![allow(clippy::collapsible_str_replace)]
#![allow(clippy::bool_comparison)]
#![allow(clippy::type_complexity)]
#![allow(clippy::if_same_then_else)]
#![allow(clippy::manual_is_variant_and)]
#![allow(clippy::unnecessary_map_or)]
#![allow(clippy::len_without_is_empty)]
#![allow(clippy::len_zero)]
#![allow(clippy::implicit_saturating_sub)]
#![allow(clippy::to_string_in_format_args)]
#![allow(clippy::incompatible_msrv)]
#![allow(clippy::int_plus_one)]
#![allow(clippy::manual_map)]
#![allow(clippy::needless_bool)]
#![allow(clippy::needless_else)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::inherent_to_string)]
#![allow(clippy::manual_find)]
#![allow(clippy::double_ended_iterator_last)]
#![allow(clippy::for_kv_map)]
#![allow(clippy::needless_splitn)]
#![allow(clippy::trim_split_whitespace)]
#![allow(clippy::useless_vec)]
#![allow(clippy::cloned_ref_to_slice_refs)]
#![allow(non_snake_case)]
#![allow(clippy::enum_variant_names)]
#![allow(clippy::manual_range_contains)]

mod agda_export;
mod bench;
mod build;
mod commands;
mod completions;
mod config;
mod diff;
mod docgen;
mod error_display;
mod format;
mod interactive;
mod json_export;
mod latex_export;
mod lsp;
mod migrate;
mod progress;
mod project;
mod proof;
mod repl;
mod watcher;

#[path = "main/mod.rs"]
mod cli_main_module;

fn main() {
    cli_main_module::cli_main();
}
