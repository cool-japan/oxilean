//! # OxiLean
//!
//! A Pure Rust theorem prover and dependent type checker inspired by Lean 4.
//!
//! This crate is a facade that re-exports all OxiLean library components.
//! Use feature flags to select which components to include.
//!
//! ## Features
//!
//! - **`kernel`** (default) - The trusted computing base for type checking
//! - **`parse`** (default) - Concrete syntax to abstract syntax parser
//! - **`elab`** (default) - Surface syntax to kernel terms elaborator
//! - **`meta`** (default) - Metavar-aware WHNF, unification, type class synthesis, and tactics
//! - **`codegen`** - LCNF-based compilation and optimization
//! - **`runtime`** - Runtime system with GC, closures, and bytecode interpretation
//! - **`std-lib`** - Standard library (Nat, Bool, List, etc.)
//! - **`lint`** - Static analysis and lint rules
//! - **`build-sys`** - Build system with incremental compilation
//! - **`wasm`** - WebAssembly support
//! - **`full`** - All components except `wasm`
//!
//! ## Quick Start
//!
//! ```rust
//! // With default features (kernel, parse, elab, meta):
//! use oxilean::kernel;
//! use oxilean::parse;
//! ```

#![deny(unsafe_code)]
#![warn(missing_docs)]

/// The trusted computing base for type checking.
#[cfg(feature = "kernel")]
pub use oxilean_kernel as kernel;

/// Metavar-aware WHNF, unification, type class synthesis, and tactics.
#[cfg(feature = "meta")]
pub use oxilean_meta as meta;

/// Concrete syntax to abstract syntax parser.
#[cfg(feature = "parse")]
pub use oxilean_parse as parse;

/// Surface syntax to kernel terms elaborator.
#[cfg(feature = "elab")]
pub use oxilean_elab as elab;

/// LCNF-based compilation and optimization.
#[cfg(feature = "codegen")]
pub use oxilean_codegen as codegen;

/// Runtime system with GC, closures, and bytecode interpretation.
#[cfg(feature = "runtime")]
pub use oxilean_runtime as runtime;

/// Standard library (Nat, Bool, List, etc.).
#[cfg(feature = "std-lib")]
pub use oxilean_std as std_lib;

/// Static analysis and lint rules.
#[cfg(feature = "lint")]
pub use oxilean_lint as lint;

/// Build system with incremental compilation.
#[cfg(feature = "build-sys")]
pub use oxilean_build as build_sys;

/// WebAssembly support.
#[cfg(feature = "wasm")]
pub use oxilean_wasm as wasm;
