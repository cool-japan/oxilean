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
//! ## Walker shape coverage (as of 2026-05-29)
//!
//! The IO action walker recognises the following expression
//! shapes, with everything else surfacing
//! [`DriverError::NotYetImplemented`] + a debug repr of the
//! offending sub-expression so callers can pinpoint the gap:
//!
//! - `Expr::Const("IO.pure", _)` and the underscore /
//!   `Pure.pure` spellings — nullary terminal, action
//!   complete.
//! - `Expr::App(IO.pure, x)` — terminal with the result
//!   discarded (`main : IO Unit` is the v0 target; non-Unit
//!   α stops at this level without re-encoding `x`).
//!   `EStateM.pure` / `EIO.pure` / `ExceptT.pure` /
//!   `StateT.pure` / `ReaderT.pure` (and their underscore-
//!   mangled forms) all accepted as the same terminal —
//!   Lean's `def IO := EIO IO.Error` unfolds through the
//!   monad-transformer family, and any rung can surface
//!   depending on the elaborator's unfold aggressiveness.
//! - `IO.bind α β m k` (arity-4 with implicits inserted by
//!   the elaborator) and `Bind.bind m k` (arity-2 after
//!   implicit erasure) — the trailing-two-args heuristic
//!   picks `(m, k)` correctly in both cases. Same
//!   monad-transformer-family lowerings recognised
//!   (`EStateM.bind`, `EIO.bind`, `ExceptT.bind`,
//!   `StateT.bind`, `ReaderT.bind`). The walker walks `m`
//!   then walks `k`; beta-application of `k` to `m`'s
//!   concrete result is a follow-up.
//! - `Expr::Const(name, _)` (and App-chains to it) where
//!   `name` resolves to an `@[extern]`-attributed
//!   declaration — `dispatch_extern_const(env, registry,
//!   resolver, name, &[])` fires the effect; `Resolved`
//!   advances the action, `NoResolverInstalled` surfaces a
//!   clean diagnostic, `Failed(e)` becomes
//!   [`DriverError::ExternFailed`].
//!
//! Still open (`NotYetImplemented` arms):
//! - `EStateM Error IO.RealWorld α` lowerings (Lean stdlib
//!   funnels IO through this monad).
//! - Beta-application of `k` in `IO.bind m k` with the
//!   concrete result feed from `m`.
//! - Walker-side canonical-ABI encoding of `head_args` into
//!   the resolver's args buffer (the empty buffer suffices
//!   for nullary callbacks; embedder-side re-pack lands
//!   alongside the leo4 adapter's full wiring).
//!
//! ## Upstream-PR viability
//!
//! Designed to land cleanly in cool-japan/oxilean — the
//! module name + signatures don't reference any leo4
//! concept. The API shape (`run_main` / `run_main_with_args`
//! / `DriverError` arms / `extern_registry` parameter) is
//! posted at <https://github.com/cool-japan/oxilean/issues/2>
//! for maintainer review. Once that discussion settles, the
//! body (continuing to expand the recognised-shape set in
//! this file) lands as a follow-up PR.
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
    ffi::ExternRegistry,
    instantiate::instantiate_one,
    Expr, Name,
};

use crate::extern_resolver::{dispatch_extern_const, ExternDispatch, SharedExternResolver};

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
    /// The walker doesn't yet recognise the reduced
    /// expression shape. v0 covers `IO.pure`, `IO.bind`
    /// (arity-4 + arity-2 / `Bind.bind`), and `@[extern]`
    /// Const dispatch; everything else surfaces this arm
    /// with a debug repr of the offending sub-expression
    /// so callers can pinpoint the missing shape.
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
                    "driver: IO walker doesn't yet cover this reduced \
                     expression shape ({reason}). v0 covers `IO.pure`, \
                     `IO.bind` (arity-4 + arity-2), and `@[extern]` Const \
                     dispatch; expand walker coverage incrementally as new \
                     shapes surface."
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
    extern_registry: &ExternRegistry,
    resolver: SharedExternResolver,
    main_name: &Name,
) -> Result<(), DriverError> {
    run_main_with_args(env, extern_registry, resolver, main_name, &[])
}

/// Drive `main : List String → IO α` (the longer form) with
/// the supplied program arguments. Empty `args` defers to
/// the no-arg `main : IO α` shape.
///
/// The `extern_registry` is the canonical OxiLean test for
/// "is this `Const` reduction `@[extern]`-backed?"
/// — see `oxilean-elab`'s `@[extern]` attribute handler.
/// The walker consults it before forwarding to `resolver`.
///
/// # Errors
/// See [`DriverError`].
#[allow(clippy::needless_pass_by_value)] // resolver intentionally moved in
pub fn run_main_with_args(
    env: &Environment,
    extern_registry: &ExternRegistry,
    resolver: SharedExternResolver,
    main_name: &Name,
    args: &[&str],
) -> Result<(), DriverError> {
    let decl = env.get(main_name).ok_or_else(|| DriverError::NotFound {
        name: main_name.to_string(),
    })?;

    let (_ty, val) = match decl {
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

    let _ = args; // program-args slot reserved for `main :
                  // List String → IO α`; v0 walker accepts
                  // only `main : IO α` (no-arg).

    let mut ctx = WalkCtx {
        env,
        extern_registry,
        resolver: &resolver,
    };
    let _final_result = walk_io_action(&val, &mut ctx, /*depth=*/ 0)?;
    // We discard the action's final pure result at the
    // driver level — `main : IO α` is driven for its
    // effects, not its return value. Embedders that
    // want the return can layer on top of `walk_io_action`
    // directly.
    Ok(())
}

/// Walker-internal context. Bundles the env + extern
/// registry + resolver so every recursion site has the
/// full toolkit available without dragging four arguments
/// through every helper.
struct WalkCtx<'a> {
    env: &'a Environment,
    extern_registry: &'a ExternRegistry,
    resolver: &'a SharedExternResolver,
}

/// IO action walker. Recognises a small but non-empty set
/// of reductions (`IO.pure` / `IO.bind` / `@[extern]` Const
/// dispatch) and surfaces explicit
/// [`DriverError::NotYetImplemented`] for anything outside
/// that set so downstream callers can distinguish "walker
/// doesn't know this shape yet" from real failures.
///
/// Full coverage list lives in the module-level docs at
/// the top of this file; this docstring focuses on the
/// internals of the recursion itself.
///
/// The shapes that still need wiring (in order of
/// motivating use cases):
///   - `Expr::App(Expr::Const("IO.bind", _), [α, β, m, k])`:
///     monadic sequence. Walk `m`, apply `k` to its result,
///     recurse.
///   - `Expr::App(Expr::Const(name, _), args)` where `name`
///     resolves to an `@[extern]` declaration: decode args
///     via canonical-ABI, dispatch through `resolver`,
///     encode the return, continue.
///   - The `EStateM Error IO.RealWorld α` reductions Lean
///     emits when an IO action goes through the standard
///     library (`println`, `IO.FS.*`, etc.) — these need
///     either OxiLean's builtin-dispatch surface
///     (`FunctionEntry::builtin`) wired into this walker,
///     or an explicit recognition layer per builtin name.
///
/// Recursion is depth-limited (`MAX_WALK_DEPTH`) to keep a
/// degenerate `IO.bind` chain from blowing the stack
/// before it becomes obvious that the walker isn't
/// progressing.
const MAX_WALK_DEPTH: usize = 1024;

/// Walker outcome: action complete + the pure result the
/// action evaluated to, when statically known. `None`
/// means the result is opaque (e.g. an `@[extern]` call
/// returning bytes the walker doesn't decode) — beta-
/// application of a continuation against an opaque result
/// can't make progress, so the walker falls back to walking
/// the continuation as another opaque IO action.
type WalkResult = Result<Option<Expr>, DriverError>;

fn walk_io_action(
    expr: &Expr,
    ctx: &mut WalkCtx<'_>,
    depth: usize,
) -> WalkResult {
    if depth > MAX_WALK_DEPTH {
        return Err(DriverError::NotYetImplemented {
            reason: format!(
                "walker recursion exceeded {MAX_WALK_DEPTH}; \
                 likely a non-terminating `IO.bind` chain or \
                 a shape the walker doesn't yet recognise"
            ),
        });
    }
    // Decompose left-leaning App chain into (head, args)
    // once at the top so each shape handler can pattern-
    // match on the head + arity without re-walking.
    let (head, head_args) = decompose_app(expr);

    if let Expr::Const(name, _) = head {
        // ── `IO.pure` / `Pure.pure` — terminal ─────────
        if is_io_pure_name(name) {
            // The pure value (last App arg) is the
            // action's static result. Surface it so
            // `IO.bind m k` can beta-apply `k` to it.
            // Nullary const form (zero App args) has no
            // explicit value; report `None` (opaque
            // Unit-ish).
            let result = head_args.last().map(|a| (*a).clone());
            return Ok(result);
        }

        // ── `IO.bind m k` — monadic sequence ───────────
        // Lean spells this `IO.bind {α β : Type} (m : IO α)
        // (k : α → IO β) : IO β`. The two implicit type
        // arguments are typically inserted by the elaborator;
        // the kernel form may have arity 4 (α, β, m, k) or
        // arity 2 (m, k) depending on whether implicit
        // erasure ran. We accept both: skip leading
        // `Sort` / `Const(...)` (likely a type) until we hit
        // the `m` value.
        if is_io_bind_name(name) {
            // Find m + k. Heuristically: the trailing two
            // args of any arity ≥ 2 are (m, k).
            if head_args.len() >= 2 {
                let m = head_args[head_args.len() - 2];
                let k = head_args[head_args.len() - 1];
                // Walk m. When it returns a concrete
                // result (e.g. `m = IO.pure x`), the
                // continuation `k` typically has shape
                // `Lam(_, _, _, body)`; beta-apply by
                // instantiating `BVar(0)` in body with
                // x, then walk the result. Otherwise
                // fall back to walking `k` as another
                // opaque IO action (the v0 behaviour
                // from `d357a01`).
                let m_result = walk_io_action(m, ctx, depth + 1)?;
                let k_result = match (m_result, k) {
                    (Some(x), Expr::Lam(_, _, _, body)) => {
                        // Concrete result + lambda continuation —
                        // beta-reduce by instantiating BVar(0)
                        // with x, then walk the substituted body.
                        let inst = instantiate_one(body, &x);
                        walk_io_action(&inst, ctx, depth + 1)?
                    }
                    (None, Expr::Lam(_, _, _, body)) => {
                        // Opaque result + lambda continuation —
                        // we don't have a concrete `x` to feed
                        // in. Substitute a placeholder
                        // (`Unit.unit`) for `BVar(0)` so any
                        // dangling de-Bruijn index in the body
                        // resolves to a valid Expr, then walk.
                        // If the body actually depends on the
                        // (opaque) result value, the walker will
                        // surface an unrecognised-shape error
                        // deep in `body`'s reduction — exactly
                        // the "this shape's gap is visible per-
                        // shape" contract.
                        let placeholder = Expr::Const(Name::str("Unit.unit"), Vec::new());
                        let inst = instantiate_one(body, &placeholder);
                        walk_io_action(&inst, ctx, depth + 1)?
                    }
                    (_, other) => walk_io_action(other, ctx, depth + 1)?,
                };
                return Ok(k_result);
            }
        }

        // ── `@[extern]`-attributed Const ───────────────
        // Reduce to the resolver-supplied bytes when
        // `dispatch_extern_const` recognises the name.
        // Currently the walker forwards an empty arg
        // buffer because the IO walker doesn't yet
        // canonical-ABI-encode `head_args` for the
        // resolver; that encoding lives at the
        // canonical-ABI layer one level up (leo4-side).
        // The empty buffer is enough to fire callbacks
        // that take no arguments (e.g. `IO.getStdin`-
        // style nullary).
        match dispatch_extern_const(ctx.env, ctx.extern_registry, Some(ctx.resolver), name, &[]) {
            ExternDispatch::Resolved(_bytes) => {
                // Effect fired. The result bytes don't
                // decode into a Lean Expr at the
                // walker level — the canonical-ABI
                // boundary lives one layer up
                // (leo4-side). Surface `None` so any
                // enclosing `IO.bind m k` sees an
                // opaque result and walks `k` as an
                // opaque IO action without trying to
                // beta-apply.
                return Ok(None);
            }
            ExternDispatch::NotExtern => {
                // Not `@[extern]` — fall through to the
                // "unrecognised shape" error so the gap
                // is visible.
            }
            ExternDispatch::NoResolverInstalled => {
                return Err(DriverError::NotYetImplemented {
                    reason: format!(
                        "walker hit `@[extern]` const `{name}` but the \
                         resolver dispatched to `NoResolverInstalled`. \
                         Check that the embedder's `SharedExternResolver` \
                         is wired before driving the walker."
                    ),
                });
            }
            ExternDispatch::Failed(e) => return Err(DriverError::ExternFailed(e)),
        }

        // ── `callback_id` outbound dispatch placeholder ─
        // (P0b #75 step 3 + IO walker integration). When
        // the IO action's reduced form mentions a Const
        // that decodes to an outbound callback_id (Lean
        // dereferenced a Rust closure passed via
        // `leo4::import!`), the walker forwards to the
        // leo4-side `OxiLeanInvoker::invoke_outbound`. The
        // shape-recognition step is leo4-specific (the
        // wire schema lives in the leo4 IDL, not the
        // OxiLean kernel), so today the walker only
        // surfaces a clear "this shape needs the leo4
        // adapter" diagnostic; the actual invocation
        // happens through the adapter-side hook.
    }

    // Fallthrough — shape not yet recognised. Surface
    // exactly what we saw so downstream knows the gap.
    Err(DriverError::NotYetImplemented {
        reason: format!(
            "walker can't reduce expression yet. \
             Head: {head:?}, arity: {arity}",
            arity = head_args.len()
        ),
    })
}

/// Decompose a left-leaning `App` chain into the ultimate
/// head expression + an ordered list of argument
/// references. `f a b c` becomes `(f, [a, b, c])`.
fn decompose_app(expr: &Expr) -> (&Expr, Vec<&Expr>) {
    let mut args: Vec<&Expr> = Vec::new();
    let mut cursor = expr;
    while let Expr::App(h, a) = cursor {
        args.push(a);
        cursor = h;
    }
    args.reverse();
    (cursor, args)
}

/// Recognise the `IO.pure` / `Pure.pure` const name in any
/// of the spellings OxiLean's elaborator can produce. The
/// canonical Lean name is `IO.pure`; the prelude lifts
/// some calls through `Pure.pure` with `Monad IO` dictionary
/// resolution. v0 walker accepts both as the same terminal.
fn is_io_pure_name(name: &Name) -> bool {
    let s = name.to_string();
    matches!(
        s.as_str(),
        "IO.pure"
            | "Pure.pure"
            | "IO_pure"
            | "Pure_pure"
            | "pure"
            // EStateM / EIO lowerings (2026-06-01 walker
            // shape grow). Lean's standard library defines
            // `def EIO (ε : Type) : Type → Type := EStateM ε
            // IO.RealWorld` and `def IO := EIO IO.Error`, so
            // depending on how aggressively the elaborator
            // unfolds the alias chain, an `IO.pure` lift can
            // surface at any rung of the alias ladder.
            // Accept every spelling as the same terminal.
            | "EIO.pure"
            | "EStateM.pure"
            | "EIO_pure"
            | "EStateM_pure"
            // ExceptT / StateT / ReaderT analogues — same
            // monad-transformer family Lean uses to define
            // IO under the hood. Treated as terminals here
            // since their effects (state mutation, error
            // propagation) are flattened back into IO at
            // the surface; the walker's job is to drive
            // the IO action, not to interpret the inner
            // monad's pure values.
            | "ExceptT.pure"
            | "StateT.pure"
            | "ReaderT.pure"
            | "ExceptT_pure"
            | "StateT_pure"
            | "ReaderT_pure"
    )
}

/// Recognise the `IO.bind` / `Bind.bind` / `Monad.bind`
/// const name in any of the spellings OxiLean's elaborator
/// can produce. Same accept-set strategy as
/// [`is_io_pure_name`] — the elaborator picks the
/// projection target based on the available `Monad IO`
/// instance, so we accept the dotted and underscore
/// forms uniformly.
fn is_io_bind_name(name: &Name) -> bool {
    let s = name.to_string();
    matches!(
        s.as_str(),
        "IO.bind"
            | "Bind.bind"
            | "Monad.bind"
            | "IO_bind"
            | "Bind_bind"
            | "Monad_bind"
            | "bind"
            // EStateM / EIO lowerings — same coverage
            // rationale as `is_io_pure_name`.
            | "EIO.bind"
            | "EStateM.bind"
            | "EIO_bind"
            | "EStateM_bind"
            // Monad transformer family. The walker
            // sequences them the same way it sequences
            // raw `IO.bind`; the inner monad's specific
            // semantics don't affect IO-effect ordering.
            | "ExceptT.bind"
            | "StateT.bind"
            | "ReaderT.bind"
            | "ExceptT_bind"
            | "StateT_bind"
            | "ReaderT_bind"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxilean_kernel::{env::Environment, Expr, Name};

    fn empty_env() -> Environment {
        Environment::new()
    }

    fn empty_extern_registry() -> ExternRegistry {
        ExternRegistry::new()
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
        let err = run_main(&env, &empty_extern_registry(), resolver, &name).unwrap_err();
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
        let err = run_main(&env, &empty_extern_registry(), resolver, &main).unwrap_err();
        match err {
            DriverError::NotADefinition { name, kind } => {
                assert!(name.contains("main"));
                assert_eq!(kind, "Axiom");
            }
            other => panic!("expected NotADefinition, got: {other:?}"),
        }
    }

    #[test]
    fn run_main_io_pure_const_succeeds() {
        // `def main : IO Unit := IO.pure ()` lowered to the
        // bare `IO.pure` const (the kernel form when no
        // explicit application is present). Walker accepts.
        let mut env = empty_env();
        let main = Name::str("main");
        let io_unit_ty = Expr::App(
            Box::new(Expr::Const(Name::str("IO"), Vec::new())),
            Box::new(Expr::Const(Name::str("Unit"), Vec::new())),
        );
        let body = Expr::Const(Name::str("IO.pure"), Vec::new());
        env.add(Declaration::Definition {
            name: main.clone(),
            univ_params: Vec::new(),
            ty: io_unit_ty,
            val: body,
            hint: oxilean_kernel::ReducibilityHint::Regular(0),
        })
        .unwrap();
        let resolver = make_resolver();
        run_main(&env, &empty_extern_registry(), resolver, &main)
            .expect("IO.pure const should walk to completion");
    }

    #[test]
    fn run_main_io_pure_app_succeeds() {
        // `def main : IO Unit := IO.pure ()` lowered to
        // `App(IO.pure, Unit.unit)`. Walker unwraps the
        // App-head chain to find `IO.pure`, accepts.
        let mut env = empty_env();
        let main = Name::str("main");
        let io_unit_ty = Expr::App(
            Box::new(Expr::Const(Name::str("IO"), Vec::new())),
            Box::new(Expr::Const(Name::str("Unit"), Vec::new())),
        );
        let body = Expr::App(
            Box::new(Expr::Const(Name::str("IO.pure"), Vec::new())),
            Box::new(Expr::Const(Name::str("Unit.unit"), Vec::new())),
        );
        env.add(Declaration::Definition {
            name: main.clone(),
            univ_params: Vec::new(),
            ty: io_unit_ty,
            val: body,
            hint: oxilean_kernel::ReducibilityHint::Regular(0),
        })
        .unwrap();
        let resolver = make_resolver();
        run_main(&env, &empty_extern_registry(), resolver, &main)
            .expect("IO.pure app should walk to completion");
    }

    #[test]
    fn run_main_io_bind_arity_4_succeeds() {
        // `IO.bind α β m k` form with the elaborator-
        // inserted type implicits left in place. Both
        // m and k are nullary `IO.pure` (terminal); walker
        // should walk both halves and return Ok.
        let mut env = empty_env();
        let main = Name::str("main");
        let io_unit_ty = Expr::App(
            Box::new(Expr::Const(Name::str("IO"), Vec::new())),
            Box::new(Expr::Const(Name::str("Unit"), Vec::new())),
        );
        // App-chain: ((((IO.bind α) β) m) k)
        let alpha = Expr::Const(Name::str("Unit"), Vec::new());
        let beta = Expr::Const(Name::str("Unit"), Vec::new());
        let m = Expr::Const(Name::str("IO.pure"), Vec::new());
        let k = Expr::Const(Name::str("IO.pure"), Vec::new());
        let body = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("IO.bind"), Vec::new())),
                        Box::new(alpha),
                    )),
                    Box::new(beta),
                )),
                Box::new(m),
            )),
            Box::new(k),
        );
        env.add(Declaration::Definition {
            name: main.clone(),
            univ_params: Vec::new(),
            ty: io_unit_ty,
            val: body,
            hint: oxilean_kernel::ReducibilityHint::Regular(0),
        })
        .unwrap();
        let resolver = make_resolver();
        run_main(&env, &empty_extern_registry(), resolver, &main)
            .expect("IO.bind arity-4 chain of IO.pure terminals should walk");
    }

    #[test]
    fn run_main_io_bind_arity_2_succeeds() {
        // `IO.bind m k` form after implicit erasure — only
        // two arguments survive at the kernel level.
        // Walker uses the "trailing two args" heuristic so
        // arity 2 picks (m, k) correctly.
        let mut env = empty_env();
        let main = Name::str("main");
        let io_unit_ty = Expr::App(
            Box::new(Expr::Const(Name::str("IO"), Vec::new())),
            Box::new(Expr::Const(Name::str("Unit"), Vec::new())),
        );
        let m = Expr::Const(Name::str("IO.pure"), Vec::new());
        let k = Expr::Const(Name::str("IO.pure"), Vec::new());
        let body = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Bind.bind"), Vec::new())),
                Box::new(m),
            )),
            Box::new(k),
        );
        env.add(Declaration::Definition {
            name: main.clone(),
            univ_params: Vec::new(),
            ty: io_unit_ty,
            val: body,
            hint: oxilean_kernel::ReducibilityHint::Regular(0),
        })
        .unwrap();
        let resolver = make_resolver();
        run_main(&env, &empty_extern_registry(), resolver, &main)
            .expect("Bind.bind arity-2 form should walk");
    }

    #[test]
    fn run_main_estatem_pure_terminal_succeeds() {
        // `def main : IO Unit := EStateM.pure ()` — the
        // form the elaborator can produce when it unfolds
        // `IO.pure` through the `EIO` / `EStateM` alias
        // chain. Walker accepts as the same terminal.
        let mut env = empty_env();
        let main = Name::str("main");
        let io_unit_ty = Expr::App(
            Box::new(Expr::Const(Name::str("IO"), Vec::new())),
            Box::new(Expr::Const(Name::str("Unit"), Vec::new())),
        );
        let body = Expr::App(
            Box::new(Expr::Const(Name::str("EStateM.pure"), Vec::new())),
            Box::new(Expr::Const(Name::str("Unit.unit"), Vec::new())),
        );
        env.add(Declaration::Definition {
            name: main.clone(),
            univ_params: Vec::new(),
            ty: io_unit_ty,
            val: body,
            hint: oxilean_kernel::ReducibilityHint::Regular(0),
        })
        .unwrap();
        let resolver = make_resolver();
        run_main(&env, &empty_extern_registry(), resolver, &main)
            .expect("EStateM.pure should walk to completion");
    }

    #[test]
    fn run_main_eio_bind_arity_2_succeeds() {
        // `def main : IO Unit := EIO.bind (EIO.pure ()) (fun _ => EIO.pure ())`
        // — chained through the `EIO` rung of the monad-
        // transformer family. Walker treats it identically
        // to the `Bind.bind` / `IO.bind` cases.
        let mut env = empty_env();
        let main = Name::str("main");
        let io_unit_ty = Expr::App(
            Box::new(Expr::Const(Name::str("IO"), Vec::new())),
            Box::new(Expr::Const(Name::str("Unit"), Vec::new())),
        );
        let m = Expr::Const(Name::str("EIO.pure"), Vec::new());
        let k = Expr::Const(Name::str("EIO.pure"), Vec::new());
        let body = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("EIO.bind"), Vec::new())),
                Box::new(m),
            )),
            Box::new(k),
        );
        env.add(Declaration::Definition {
            name: main.clone(),
            univ_params: Vec::new(),
            ty: io_unit_ty,
            val: body,
            hint: oxilean_kernel::ReducibilityHint::Regular(0),
        })
        .unwrap();
        let resolver = make_resolver();
        run_main(&env, &empty_extern_registry(), resolver, &main)
            .expect("EIO.bind arity-2 chain should walk");
    }

    #[test]
    fn run_main_statet_pure_terminal_succeeds() {
        // Spot-check one of the other monad transformer
        // lowerings — `StateT.pure`. Same handling as
        // `EStateM.pure` / `IO.pure`.
        let mut env = empty_env();
        let main = Name::str("main");
        let io_unit_ty = Expr::App(
            Box::new(Expr::Const(Name::str("IO"), Vec::new())),
            Box::new(Expr::Const(Name::str("Unit"), Vec::new())),
        );
        let body = Expr::Const(Name::str("StateT.pure"), Vec::new());
        env.add(Declaration::Definition {
            name: main.clone(),
            univ_params: Vec::new(),
            ty: io_unit_ty,
            val: body,
            hint: oxilean_kernel::ReducibilityHint::Regular(0),
        })
        .unwrap();
        let resolver = make_resolver();
        run_main(&env, &empty_extern_registry(), resolver, &main)
            .expect("StateT.pure should walk to completion");
    }

    #[test]
    fn run_main_io_bind_beta_applies_k_to_m_result() {
        // `def main : IO Unit := IO.bind (IO.pure Unit.unit) (fun x => IO.pure x)`.
        // The walker should:
        //   1. Walk m = `App(IO.pure, Unit.unit)` → `Some(Unit.unit)`.
        //   2. Beta-apply k = `Lam(_, "x", _, App(IO.pure, BVar 0))`
        //      by instantiating BVar(0) with Unit.unit →
        //      `App(IO.pure, Unit.unit)`.
        //   3. Walk the substituted body → terminal.
        //
        // No extern dispatch needed; the test exercises
        // the beta path itself.
        let mut env = empty_env();
        let main = Name::str("main");
        let io_unit_ty = Expr::App(
            Box::new(Expr::Const(Name::str("IO"), Vec::new())),
            Box::new(Expr::Const(Name::str("Unit"), Vec::new())),
        );
        let unit = Expr::Const(Name::str("Unit.unit"), Vec::new());
        let m = Expr::App(
            Box::new(Expr::Const(Name::str("IO.pure"), Vec::new())),
            Box::new(unit.clone()),
        );
        // k = `fun x : Unit => IO.pure (BVar 0)`.
        let unit_ty = Expr::Const(Name::str("Unit"), Vec::new());
        let k = Expr::Lam(
            oxilean_kernel::BinderInfo::Default,
            Name::str("x"),
            Box::new(unit_ty),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("IO.pure"), Vec::new())),
                Box::new(Expr::BVar(0)),
            )),
        );
        let body = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Bind.bind"), Vec::new())),
                Box::new(m),
            )),
            Box::new(k),
        );
        env.add(Declaration::Definition {
            name: main.clone(),
            univ_params: Vec::new(),
            ty: io_unit_ty,
            val: body,
            hint: oxilean_kernel::ReducibilityHint::Regular(0),
        })
        .unwrap();
        let resolver = make_resolver();
        run_main(&env, &empty_extern_registry(), resolver, &main)
            .expect("IO.bind (IO.pure x) (fun x => IO.pure x) should walk");
    }

    #[test]
    fn run_main_io_bind_opaque_extern_falls_back_to_opaque_k_walk() {
        // `def main : IO Unit := IO.bind myExtern (fun _ => IO.pure ())`.
        // myExtern is `@[extern]` so the walker dispatches
        // through the resolver and gets opaque bytes (we
        // don't know the Lean Expr). The walker then walks
        // k as an opaque action — k's body discards the
        // result (`_` binder) so the action still walks
        // to completion.
        use crate::extern_resolver::ExternResolver;
        use oxilean_kernel::ffi::{
            CallingConvention, ExternDecl, FfiSafety, FfiSignature, FfiType,
        };

        struct OkResolver;
        impl ExternResolver for OkResolver {
            fn resolve(
                &self,
                _decl_name: &Name,
                _args: &[u8],
            ) -> Result<Vec<u8>, oxilean_kernel::ffi::ExternCallError> {
                Ok(Vec::new())
            }
        }

        let mut env = empty_env();
        let main = Name::str("main");
        let extern_name = Name::str("myExtern");

        env.add(Declaration::Axiom {
            name: extern_name.clone(),
            univ_params: Vec::new(),
            ty: Expr::App(
                Box::new(Expr::Const(Name::str("IO"), Vec::new())),
                Box::new(Expr::Const(Name::str("Unit"), Vec::new())),
            ),
        })
        .unwrap();

        let m = Expr::Const(extern_name.clone(), Vec::new());
        // k = `fun _ : Unit => IO.pure ()` — body ignores
        // BVar(0), so even though the walker passes "opaque"
        // through, the body walks to completion.
        let k = Expr::Lam(
            oxilean_kernel::BinderInfo::Default,
            Name::str("_"),
            Box::new(Expr::Const(Name::str("Unit"), Vec::new())),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("IO.pure"), Vec::new())),
                Box::new(Expr::Const(Name::str("Unit.unit"), Vec::new())),
            )),
        );
        let body = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Bind.bind"), Vec::new())),
                Box::new(m),
            )),
            Box::new(k),
        );

        env.add(Declaration::Definition {
            name: main.clone(),
            univ_params: Vec::new(),
            ty: Expr::App(
                Box::new(Expr::Const(Name::str("IO"), Vec::new())),
                Box::new(Expr::Const(Name::str("Unit"), Vec::new())),
            ),
            val: body,
            hint: oxilean_kernel::ReducibilityHint::Regular(0),
        })
        .unwrap();

        let mut registry = ExternRegistry::new();
        registry
            .register(ExternDecl::new(
                extern_name.clone(),
                Expr::Const(Name::str("ByteArray"), Vec::new()),
                "leo4-rust-bridge".to_string(),
                "myExtern".to_string(),
                FfiSafety::Safe,
                CallingConvention::Rust,
                FfiSignature::new(vec![FfiType::ByteArray], Box::new(FfiType::ByteArray)),
            ))
            .unwrap();

        let resolver: SharedExternResolver = Arc::new(OkResolver);
        run_main(&env, &registry, resolver, &main)
            .expect("opaque-extern IO.bind should walk with discarded k binder");
    }

    #[test]
    fn run_main_extern_const_dispatches_through_resolver() {
        use crate::extern_resolver::ExternResolver;
        use oxilean_kernel::ffi::{
            CallingConvention, ExternDecl, FfiSafety, FfiSignature, FfiType,
        };

        // Tracking resolver — records whether it was
        // consulted + returns Ok(empty).
        struct TrackingResolver {
            fired: std::sync::Mutex<bool>,
        }
        impl ExternResolver for TrackingResolver {
            fn resolve(
                &self,
                _decl_name: &Name,
                _args: &[u8],
            ) -> Result<Vec<u8>, oxilean_kernel::ffi::ExternCallError> {
                *self.fired.lock().unwrap() = true;
                Ok(Vec::new())
            }
        }

        let mut env = empty_env();
        let main = Name::str("main");
        let extern_name = Name::str("myExtern");

        // env carries the extern as an Axiom (canonical
        // shape for `@[extern] opaque myExtern : IO Unit`).
        env.add(Declaration::Axiom {
            name: extern_name.clone(),
            univ_params: Vec::new(),
            ty: Expr::App(
                Box::new(Expr::Const(Name::str("IO"), Vec::new())),
                Box::new(Expr::Const(Name::str("Unit"), Vec::new())),
            ),
        })
        .unwrap();

        // main := myExtern
        let body = Expr::Const(extern_name.clone(), Vec::new());
        env.add(Declaration::Definition {
            name: main.clone(),
            univ_params: Vec::new(),
            ty: Expr::App(
                Box::new(Expr::Const(Name::str("IO"), Vec::new())),
                Box::new(Expr::Const(Name::str("Unit"), Vec::new())),
            ),
            val: body,
            hint: oxilean_kernel::ReducibilityHint::Regular(0),
        })
        .unwrap();

        let mut registry = ExternRegistry::new();
        registry
            .register(ExternDecl::new(
                extern_name.clone(),
                Expr::Const(Name::str("ByteArray"), Vec::new()),
                "leo4-rust-bridge".to_string(),
                "myExtern".to_string(),
                FfiSafety::Safe,
                CallingConvention::Rust,
                FfiSignature::new(vec![FfiType::ByteArray], Box::new(FfiType::ByteArray)),
            ))
            .unwrap();

        let tracker = Arc::new(TrackingResolver {
            fired: std::sync::Mutex::new(false),
        });
        let resolver: SharedExternResolver = tracker.clone();

        run_main(&env, &registry, resolver, &main)
            .expect("extern const should dispatch through resolver");

        assert!(
            *tracker.fired.lock().unwrap(),
            "resolver should have been consulted on the @[extern] const reduction"
        );
    }

    #[test]
    fn run_main_unsupported_shape_surfaces_not_yet_implemented() {
        // Non-IO-pure body (e.g. bare Unit.unit, or any
        // shape outside the v0 walker's recognised set)
        // surfaces an explicit NotYetImplemented so
        // downstream knows the gap is "this shape", not
        // "everything".
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
        let err = run_main(&env, &empty_extern_registry(), resolver, &main).unwrap_err();
        match err {
            DriverError::NotYetImplemented { reason } => {
                assert!(
                    reason.contains("walker"),
                    "reason should mention the walker, got: {reason}"
                );
            }
            other => panic!("expected NotYetImplemented, got: {other:?}"),
        }
    }
}
