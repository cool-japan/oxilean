//! Enhanced pretty-printer for OxiLean kernel terms.
//!
//! This module provides [`PrettyPrinter`] — a fully configurable,
//! layout-aware pretty-printer that supersedes the simpler
//! `prettyprint::ExprPrinter`.
//!
//! # Key improvements over `prettyprint`
//!
//! - Configurable line width (for future Wadler-Lindig layout)
//! - Full `ConstantInfo` / declaration printing via `pp_decl`
//! - Explicit `pp_type` entry point (same output, clearer intent)
//! - [`IndentMode`]: spaces or tabs, configurable depth
//! - `max_depth` guard to avoid pathological prints
//! - Universe annotations shown/hidden via config flag
//! - Proof bodies shown/hidden independently
//!
//! # Quick start
//!
//! ```
//! use oxilean_kernel::pretty_print::{PrettyPrinter, pp_expr, pp_type};
//! use oxilean_kernel::{Expr, Literal};
//!
//! let e = Expr::Lit(Literal::Nat(42));
//! assert_eq!(pp_expr(&e), "42");
//! assert_eq!(pp_type(&e), "42");
//! assert_eq!(PrettyPrinter::new().pp_expr(&e), "42");
//! ```
//!
//! # Configuration
//!
//! ```
//! use oxilean_kernel::pretty_print::{PrettyPrinter, PrettyConfig, IndentMode};
//!
//! let pp = PrettyPrinter::with_config(PrettyConfig {
//!     unicode: false,
//!     show_universes: true,
//!     width: 80,
//!     indent: IndentMode::Spaces(4),
//!     ..Default::default()
//! });
//! ```

pub mod functions;
pub mod types;

pub use functions::*;
pub use types::*;
