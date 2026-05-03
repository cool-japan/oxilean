//! Hygienic macro expansion for OxiLean.
//!
//! This module implements scope-aware macro expansion that prevents variable
//! capture.  The core algorithm tags every introduced name with a fresh
//! `ScopeId` and performs alpha-renaming so that names from a macro body
//! cannot accidentally collide with names from the call site, and vice-versa.
//!
//! # Quick example
//!
//! ```ignore
//! use oxilean_parse::hygienic_macro::{HygieneCtx, MacroDef, MacroCall, expand_macro, ScopeId};
//!
//! let mut ctx = HygieneCtx::new();
//! let def = MacroDef { name: "double".into(), params: vec!["n".into()],
//!                       body: "n + n".into(), def_scope: ScopeId(0) };
//! let call = MacroCall { name: "double".into(), args: vec!["5".into()],
//!                         call_scope: ctx.current_scope };
//! let result = expand_macro(&def, &call, &mut ctx);
//! assert!(result.expanded.contains("5"));
//! ```

pub mod functions;
pub mod types;

pub use functions::*;
pub use types::*;
