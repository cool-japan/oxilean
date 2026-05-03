//! Elaboration hooks — callbacks that run at key points during elaboration.
//!
//! This module provides a `HookRegistry` for registering named hooks keyed by
//! `ElabHookKind`, utilities for firing hooks against a `HookEvent`, and a
//! `HookTrace` that records every hook execution for debugging and testing.

pub mod functions;
pub mod types;

pub use functions::{fire_hooks, merge_results, trace_to_string};
pub use types::{ElabHook, ElabHookKind, HookEvent, HookRegistry, HookResult, HookTrace};
