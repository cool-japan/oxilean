//! WebAssembly bindings for OxiLean.
//!
//! This crate provides a WASM-compatible API for the OxiLean type checker,
//! allowing use from JavaScript/TypeScript in web browsers or Node.js.
#![allow(dead_code)]
#![allow(unused_imports)]

pub mod api;
pub mod error;
pub mod incremental;
pub mod types;

#[cfg(feature = "wasm")]
pub mod wasm_api;
