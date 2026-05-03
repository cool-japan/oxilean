//! # OxiLean Code Generation
//!
//! Multi-target code generation backend for the OxiLean theorem prover.
//!
//! This crate compiles type-checked OxiLean kernel expressions into executable
//! code for a wide range of target platforms. Expressions are first lowered to
//! a Lambda-Case Normal Form (LCNF) intermediate representation, then run
//! through optimization passes, and finally emitted by a target-specific
//! backend.
//!
//! ## Pipeline
//!
//! 1. Kernel `Expr` is lowered via [`to_lcnf`] into the [`lcnf`] IR.
//! 2. Optimization passes in [`opt_passes`] orchestrate dead-code elimination
//!    ([`opt_dce`]), copy propagation and inlining ([`opt_copy_prop`],
//!    [`opt_inline`]), join-point analysis ([`opt_join`]), reference-count
//!    reuse ([`opt_reuse`]), monomorphization ([`opt_specialize`]),
//!    common-subexpression elimination ([`opt_cse`]), loop-invariant code
//!    motion ([`opt_licm`]), vectorization ([`opt_vectorize`]), and many more.
//! 3. [`closure_convert`] flattens closures.
//! 4. A backend module emits the target-specific source or bytecode.
//!
//! ## Backends
//!
//! Native and systems targets include [`native_backend`] (Rust), [`c_backend`],
//! [`llvm_backend`], [`cranelift_backend`], [`x86_64_backend`], [`riscv_backend`],
//! [`mlir_backend`], [`spirv_backend`], [`wasm_backend`], and
//! [`wasm_component_backend`]. GPU and shader targets include [`cuda_backend`],
//! [`metal_backend`], [`glsl_backend`], and [`wgsl_backend`]. Functional and
//! proof-assistant targets include [`agda_backend`], [`coq_backend`],
//! [`lean4_backend`], [`haskell_backend`], [`idris_backend`], [`ocaml_backend`],
//! [`fsharp_backend`], and [`dhall_backend`]. Mainstream language targets
//! include [`js_backend`], [`typescript_backend`], [`python_backend`],
//! [`java_backend`], [`jvm_backend`], [`kotlin_backend`], [`scala_backend`],
//! [`csharp_backend`], [`cil_backend`], [`go_backend`], [`swift_backend`],
//! [`dart_backend`], [`ruby_backend`], [`php_backend`], [`lua_backend`],
//! [`elixir_backend`], [`beam_backend`], [`bash_backend`], [`r_backend`],
//! [`julia_backend`], [`matlab_backend`], [`fortran_backend`], [`zig_backend`],
//! [`nix_backend`], [`prolog_backend`], [`chapel_backend`], [`futhark_backend`],
//! [`chisel_backend`], [`verilog_backend`], [`sql_backend`],
//! [`graphql_backend`], and the smart-contract targets [`evm_backend`],
//! [`solidity_backend`], and [`vyper_backend`]. C FFI headers are produced by
//! [`ffi_bridge`].
//!
//! ## Usage
//!
//! See the README for a configuration example using `CodegenConfig` and
//! `CodegenTarget`, and the [`pipeline`] module for the end-to-end driver.

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(clippy::new_without_default)]
#![allow(non_snake_case)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::approx_constant)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::single_match)]
#![allow(clippy::field_reassign_with_default)]
#![allow(clippy::len_without_is_empty)]
#![allow(clippy::useless_format)]
#![allow(clippy::type_complexity)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::result_large_err)]
#![allow(clippy::vec_init_then_push)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::write_with_newline)]
#![allow(clippy::absurd_extreme_comparisons)]
#![allow(clippy::unnecessary_map_or)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::if_same_then_else)]
#![allow(clippy::only_used_in_recursion)]
#![allow(clippy::module_inception)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::manual_map)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::while_let_loop)]
#![allow(clippy::unnecessary_map_on_constructor)]
#![allow(clippy::iter_kv_map)]
#![allow(clippy::collapsible_str_replace)]
#![allow(clippy::default_constructed_unit_structs)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::needless_borrow)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::len_zero)]
#![allow(clippy::get_first)]
#![allow(clippy::manual_find)]
#![allow(clippy::float_cmp)]
#![allow(clippy::excessive_precision)]
#![allow(clippy::inconsistent_digit_grouping)]
#![allow(clippy::needless_ifs)]
#![allow(clippy::needless_borrows_for_generic_args)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::unnecessary_unwrap)]
#![allow(clippy::for_kv_map)]
#![allow(unused_comparisons)]
#![allow(private_interfaces)]
#![allow(clippy::incompatible_msrv)]

//! Code Generation Backend for Oxilean
//!
//! This crate provides compilation from Oxilean kernel expressions
// to various target languages: Rust, C, LLVM IR, and interpretation.
//
// The pipeline consists of:
// 1. Expression to Intermediate Representation (ExprToIr)
// 2. IR optimization (Optimizer)
// 3. Backend-specific code emission (IrToRust, IrToC)

pub mod agda_backend;
pub mod bash_backend;
pub mod beam_backend;
pub mod c_backend;
pub mod chapel_backend;
pub mod chisel_backend;
pub mod cil_backend;
pub mod closure_convert;
pub mod coq_backend;
pub mod cranelift_backend;
pub mod csharp_backend;
pub mod cuda_backend;
pub mod dart_backend;
pub mod dhall_backend;
pub mod elixir_backend;
pub mod evm_backend;
pub mod ffi_bridge;
pub mod fortran_backend;
pub mod fsharp_backend;
pub mod futhark_backend;
pub mod glsl_backend;
pub mod go_backend;
pub mod graphql_backend;
pub mod haskell_backend;
pub mod idris_backend;
pub mod ir_serialize;
pub mod java_backend;
pub mod js_backend;
pub mod julia_backend;
pub mod jvm_backend;
pub mod kotlin_backend;
pub mod lcnf;
pub mod lean4_backend;
pub mod llvm_backend;
pub mod llvm_ir_text;
pub mod lua_backend;
pub mod matlab_backend;
pub mod metal_backend;
pub mod mlir_backend;
pub mod native_backend;
pub mod nix_backend;
pub mod ocaml_backend;
pub mod opt_algebraic;
pub mod opt_alias;
pub mod opt_beta_eta;
pub mod opt_cache;
pub mod opt_copy_prop;
pub mod opt_cse;
pub mod opt_ctfe;
pub mod opt_dce;
pub mod opt_dead_branch;
pub mod opt_dse;
pub mod opt_escape;
pub mod opt_gvn;
pub mod opt_inline;
pub mod opt_join;
pub mod opt_licm;
pub mod opt_loop_unroll;
pub mod opt_mem2reg;
pub mod opt_parallel;
pub mod opt_partial_eval;
pub mod opt_passes;
pub mod opt_peephole;
pub mod opt_regalloc;
pub mod opt_reuse;
pub mod opt_specialize;
pub mod opt_specialize_types;
pub mod opt_strength;
pub mod opt_tail_recursion;
pub mod opt_vectorize;
pub mod pgo;
pub mod php_backend;
pub mod pipeline;
pub mod prolog_backend;
pub mod python_backend;
pub mod r_backend;
pub mod riscv_backend;
pub mod ruby_backend;
pub mod runtime_codegen;
pub mod rust_target_backend;
pub mod scala_backend;
pub mod solidity_backend;
pub mod spirv_backend;
pub mod sql_backend;
pub mod swift_backend;
pub mod to_lcnf;
pub mod typescript_backend;
pub mod verilog_backend;
pub mod vyper_backend;
pub mod wasm_backend;
pub mod wasm_component_backend;
pub mod wgsl_backend;
pub mod x86_64_backend;
pub mod zig_backend;

pub mod core_types;
pub use core_types::*;
