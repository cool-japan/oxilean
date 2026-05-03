//! # OxiLean Build System & Package Manager
//!
//! This crate implements the build system and package manager for OxiLean,
//! providing incremental compilation, dependency resolution, parallel build
//! execution, and package registry integration.
//!
//! ## Modules
//!
//! - manifest: Package manifest parsing and metadata
//! - resolver: PubGrub-style dependency resolution
//! - incremental: Incremental compilation with fingerprinting
//! - executor: DAG-based parallel build scheduling
//! - registry: Package registry integration
//! - scripts: Custom build scripts and hooks

#![allow(dead_code)]
#![warn(clippy::all)]
#![allow(clippy::result_large_err)]
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
#![allow(clippy::len_without_is_empty)]
#![allow(clippy::type_complexity)]
#![allow(clippy::manual_saturating_arithmetic)]
#![allow(clippy::if_same_then_else)]
#![allow(clippy::manual_is_variant_and)]
#![allow(clippy::implicit_saturating_sub)]
#![allow(clippy::incompatible_msrv)]
#![allow(clippy::int_plus_one)]
#![allow(clippy::manual_map)]
#![allow(clippy::needless_bool)]
#![allow(clippy::needless_else)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::manual_find)]
#![allow(clippy::for_kv_map)]
#![allow(clippy::manual_range_contains)]
#![allow(clippy::double_ended_iterator_last)]
#![allow(clippy::len_zero)]

pub mod analytics;
pub mod cache_eviction;
pub mod cache_invalidation;
pub mod dep_analysis;
pub mod distributed;
pub mod executor;
pub mod file_watcher;
pub mod incremental;
pub mod manifest;
pub mod opt_incremental;
pub mod plan_optimizer;
pub mod registry;
pub mod remote_cache;
pub mod resolver;
pub mod scripts;

pub mod core_types;
pub use core_types::*;
