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
//! ## Walker shape coverage (as of 2026-05-31)
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
//!   resolver, name, encode_leaf_args_for_extern(args, env))`
//!   fires the effect. The arg encoder covers the leaf
//!   shapes the walker can statically lower (Nat / String
//!   literals, `Bool.true` / `Bool.false`, `Unit.unit`);
//!   anything richer falls back to the empty buffer so
//!   nullary callbacks keep firing while composite
//!   encoding lands incrementally.
//! - **`IO.bind m k` beta-application** — when `m` walks
//!   to `Some(x)` (statically known result, e.g.
//!   `m = IO.pure x`) and `k` is a `Lam(_, _, _, body)`,
//!   the walker `instantiate_one`'s `body` with `x` and
//!   walks the substituted body. Opaque `m`-results
//!   substitute a `Unit.unit` placeholder for `BVar(0)`
//!   so `fun _ => …`-style continuations still walk
//!   cleanly.
//! - **`@[extern]` arg encoding — sized integer /
//!   signed / float / char path**. Lean wraps these as
//!   `OfNat.ofNat n` / `Neg.neg x` / `Char.ofNat n`
//!   typeclass-projection Apps after elaboration.
//!   `encode_typeclass_projection` recognises the
//!   sized-integer width by inspecting the type-class
//!   Type arg (UInt8..128, USize, Int8..128, ISize,
//!   Char) and writes the matching canonical-ABI byte
//!   width LE. Float literals stay constant-folded by
//!   OxiLean's reducer before the walker sees them.
//! - **`@[extern]` arg encoding — composite ctor
//!   path**. `Prod.mk a b` / `Subtype.mk x p` /
//!   `Option.some x` / `Sum.inl/inr x` / `Option.none`
//!   each recurse through `encode_one_arg` and emit
//!   the SPEC §10/§11 record / variant wire bytes.
//!   User-defined records + inductives stay out of
//!   scope (embedder territory — needs the user-side
//!   IDL).
//! - **Stdlib IO builtin dispatch**. `IO.println` /
//!   `IO.eprintln` / `IO.print` / `IO.eprint` (and
//!   their underscore-mangled spellings) fire their
//!   stdout / stderr effect directly in the walker via
//!   `try_dispatch_io_builtin`, ahead of the regular
//!   `@[extern]` resolver dispatch. Embedders that
//!   want to intercept these still can — the walker
//!   only handles the case where the arg decodes
//!   through a `Lit(Str)` or trivial `toString` wrap;
//!   richer shapes fall through to the resolver path.
//! - **Stdlib `IO.FS.*` file-system dispatch**.
//!   `IO.FS.readFile` / `writeFile` / `appendFile` /
//!   `removeFile` / `createDir` / `createDirAll` /
//!   `removeDir` / `removeDirAll` / `rename` (and the
//!   underscore-mangled spellings) fire their effect
//!   directly via `try_dispatch_io_fs_builtin` against
//!   the host `std::fs`. `readFile` surfaces the
//!   contents as `Ok(Some(Lit(Str)))` so an enclosing
//!   `IO.bind m k` can beta-apply `k` against the
//!   string. IO errors wrap into
//!   `DriverError::ExternFailed` via
//!   `ExternCallError::CallbackFailed`.
//! - **User-defined record / inductive ctor encoding**.
//!   `encode_user_defined_ctor` walks the ctor's
//!   `ConstantInfo::Constructor` metadata in `env` to
//!   skip type-param leading args, then emits SPEC §10
//!   variant-tag discriminant (multi-ctor inductives
//!   only) + each field encoded via `encode_one_arg`.
//!   This covers both anonymous user records
//!   (single-ctor structures) and sum-type inductives
//!   without the walker needing the user-side IDL on
//!   hand.
//!
//! Out of scope by design (single fallthrough
//! `NotYetImplemented` arm covers everything below — by
//! intent, not by neglect):
//!
//! - **Non-IO monad-class run projections**
//!   (`StateT.run` / `ReaderT.run` / `ExceptT.run`,
//!   etc.). These belong at the LCNF / bytecode
//!   interpreter layer, *below* the kernel-name walker.
//!   The walker fires only on `IO` actions; transformer-
//!   stack `runX` projections should reduce away inside
//!   `bytecode_interp` / `lazy_eval` / `tco` before the
//!   walker sees the resulting `IO α`. Wiring them at
//!   the walker layer would double-implement reduction
//!   that the lower layers already do.
//! - **`IO.FS.Handle.*` family** (handle-based
//!   readers/writers). Needs a host-side `File` lifetime
//!   tied to a Lean value, which the walker doesn't
//!   model. Embedders can intercept via the resolver.
//! - **`dbg_trace` / `panic!` / `unreachable!`**.
//!   Compile-time elaboration hooks, not IO-level —
//!   OxiLean's elaborator handles them before the
//!   walker sees the body.
//! - **Float-literal lowering** (`Float.ofBinaryScientific`).
//!   Constant-folded by OxiLean's reducer; not the
//!   walker's responsibility.
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
    declaration::ConstantInfo,
    env::{Declaration, Environment},
    ffi::ExternRegistry,
    instantiate::instantiate_one,
    Expr, Literal, Name,
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
    /// The walker hit a reduced expression shape that
    /// belongs to the "out of scope by design" set
    /// documented in the module docstring (non-IO monad
    /// transformer projections, `IO.FS.Handle.*`,
    /// compile-time hooks like `dbg_trace`, …). The
    /// embedder can either pre-reduce the body at the
    /// LCNF / bytecode interpreter layer before driving
    /// `run_main`, or intercept via the `@[extern]`
    /// resolver path.
    ///
    /// The `reason` carries a debug repr of the offending
    /// sub-expression so callers can pinpoint exactly
    /// which shape escaped recognition.
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
                    "driver: IO walker doesn't cover this reduced \
                     expression shape ({reason}). The recognised set \
                     covers `IO.pure`, `IO.bind` (arity-4 + arity-2) + \
                     monad-transformer-family lowerings, `@[extern]` \
                     Const dispatch (with canonical-ABI arg encoding \
                     including user-defined record / inductive ctors via \
                     env-lookup), and the stdlib `IO.println` family + \
                     `IO.FS.*` file-system builtins. Shapes outside that \
                     set are out-of-scope by design — either pre-reduce \
                     at the LCNF / bytecode interpreter layer or \
                     intercept via the resolver."
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

        // ── Stdlib IO builtins handled directly ─────
        // `IO.println` / `IO.eprintln` / `IO.print` and
        // their underscore-mangled spellings fire their
        // effect directly here so embedders don't have
        // to layer a resolver for the common stdout /
        // stderr write path. When the arg is a `String`
        // literal (or decodes through the leaf encoder),
        // the walker writes it + returns `Ok(None)`.
        // Anything richer falls through to the regular
        // `dispatch_extern_const` arm so embedders can
        // still customise.
        if let Some(()) = try_dispatch_io_builtin(name, &head_args) {
            return Ok(None);
        }

        // ── Stdlib IO.FS file-system builtins ─────────
        // Same shape as `try_dispatch_io_builtin` but for
        // file-system effects (`IO.FS.readFile` /
        // `writeFile` / `appendFile` / `removeFile` /
        // `createDir` / `removeDir`). `readFile` surfaces
        // the file contents as `Ok(Some(Lit(Str)))` so a
        // surrounding `IO.bind m k` can beta-apply `k`
        // against the contents; the write/remove/create
        // variants return `Ok(None)`. std::fs errors
        // surface as `DriverError::ExternFailed` via
        // `ExternCallError::CallbackFailed`.
        if let Some(result) = try_dispatch_io_fs_builtin(name, &head_args) {
            return result;
        }

        // ── `@[extern]`-attributed Const ───────────────
        // Reduce to the resolver-supplied bytes when
        // `dispatch_extern_const` recognises the name.
        // Args are canonical-ABI-encoded by
        // `encode_leaf_args_for_extern` — this covers
        // the leaf shapes (Lit / Bool ctor / Unit /
        // BVar-placeholder-substituted const) the walker
        // can reduce statically. When *any* arg falls
        // outside the recognised set, the walker
        // forwards `&[]` for backward compatibility
        // with the v0 (nullary-callback-only) path —
        // the resolver still fires; embedders that need
        // richer encoding can layer on top.
        let encoded_args = encode_leaf_args_for_extern(&head_args, ctx.env).unwrap_or_default();
        match dispatch_extern_const(ctx.env, ctx.extern_registry, Some(ctx.resolver), name, &encoded_args) {
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

/// Canonical-ABI encode the walker-visible leaf shapes of
/// an `@[extern]` Const's arg list. Returns `Some(bytes)`
/// when *every* arg lowers to a recognised leaf, `None`
/// otherwise — the caller falls back to forwarding an
/// empty buffer for backward compatibility with the
/// pre-encoding walker (`d357a01` `IO.bind` + extern
/// dispatch).
///
/// Recognised leaf shapes + their wire format (matching
/// `SPEC/canonical-abi.md` §6/§7/§10/§11):
///
/// - `Expr::Lit(Literal::Nat(n))` — 8 bytes, u64 LE.
/// - `Expr::Lit(Literal::Str(s))` — 4 bytes u32 LE
///   length prefix + UTF-8 bytes.
/// - `Expr::Const("Bool.true", _)` / `Bool.false` —
///   1 byte (0x01 / 0x00).
/// - `Expr::Const("Unit.unit", _)` — zero bytes (unit
///   type has no payload).
/// - `OfNat.ofNat <type> <Lit(Nat)> _` — sized integer
///   (UInt8..128, USize, Int8..128, ISize, Char) at the
///   matching canonical-ABI byte width LE.
/// - `Neg.neg <type> _ <inner>` — signed integer with
///   the inner Nat negated under two's complement.
/// - `Char.ofNat <Lit(Nat)>` — u32 LE Unicode code
///   point.
/// - `Prod.mk` / `Subtype.mk` / `Option.some` /
///   `Option.none` / `Sum.inl` / `Sum.inr` — canonical
///   record / variant wire bytes by recursing into the
///   payload.
/// - **Any user-defined `Constructor` registered in
///   `env`** — `encode_user_defined_ctor` walks the
///   ctor's `ConstantInfo::Constructor` metadata to skip
///   leading type-param args, emit the SPEC §10
///   variant-tag discriminant (multi-ctor inductives
///   only), and recursively encode each field. Covers
///   anonymous user records (single-ctor structures)
///   and sum-type inductives without leo4 IDL
///   threading.
///
/// Anything else — Lam, Pi, Let, Proj, FVar, BVar
/// (post-instantiate), or a `Const` / App that doesn't
/// resolve to a Constructor in `env` — returns `None`.
/// The walker then falls back to the empty-arg-buffer
/// dispatch path so callbacks that don't read their args
/// still fire.
///
/// Out of scope (each surfaces as `None`):
///
/// - `Float32.ofBinaryScientific` /
///   `Float.ofBinaryScientific` — Lean's float literal
///   shape. Constant-folded by OxiLean's reducer before
///   the walker sees them; the walker doesn't replicate
///   arithmetic at the kernel layer.
fn encode_leaf_args_for_extern(args: &[&Expr], env: &Environment) -> Option<Vec<u8>> {
    let mut out = Vec::new();
    for arg in args {
        encode_one_arg(arg, &mut out, env)?;
    }
    Some(out)
}

/// Encode a single leaf arg. Split out from
/// `encode_leaf_args_for_extern` so the typeclass-
/// projection arms (`OfNat.ofNat`, `Neg.neg`,
/// `Char.ofNat`) can recurse on their inner value
/// expressions.
fn encode_one_arg(arg: &Expr, out: &mut Vec<u8>, env: &Environment) -> Option<()> {
    match arg {
        Expr::Lit(Literal::Nat(n)) => {
            out.extend_from_slice(&n.to_le_bytes());
        }
        Expr::Lit(Literal::Str(s)) => {
            let bytes = s.as_bytes();
            let len = u32::try_from(bytes.len()).ok()?;
            out.extend_from_slice(&len.to_le_bytes());
            out.extend_from_slice(bytes);
        }
        Expr::Const(name, _) => {
            let s = name.to_string();
            match s.as_str() {
                "Bool.true" | "Bool_true" | "true" => out.push(0x01),
                "Bool.false" | "Bool_false" | "false" => out.push(0x00),
                "Unit.unit" | "Unit_unit" => { /* zero-byte payload */ }
                // Nullary variant ctors at the bare-Const
                // level (no App-chain) — same wire format
                // as the App-headed `encode_typeclass_projection`
                // arms but with zero payload.
                "Option.none" | "Option_none" => {
                    out.extend_from_slice(&0u32.to_le_bytes());
                }
                _ => {
                    // Fall back to env-lookup: a bare
                    // `Const(ctor)` with no App args is a
                    // nullary user-defined ctor like
                    // `Color.Red`. The encoder writes the
                    // SPEC §10 variant-tag discriminant
                    // when the parent inductive has > 1
                    // ctor; single-ctor inductives (unit-
                    // shaped values) emit nothing.
                    encode_user_defined_ctor(name, &[], out, env)?;
                }
            }
        }
        Expr::App(_, _) => {
            // App-headed args — typically a typeclass
            // projection like `OfNat.ofNat`, `Neg.neg`, or
            // `Char.ofNat`, or a user-defined ctor App-
            // chain like `Foo.mk a b`. Decompose +
            // recognise the head's named pattern; the
            // user-defined-ctor fallback lives in
            // `encode_typeclass_projection`'s `_` arm.
            let (head, head_args) = decompose_app(arg);
            encode_typeclass_projection(head, &head_args, out, env)?;
        }
        _ => return None,
    }
    Some(())
}

/// Recognise the small subset of Lean stdlib IO
/// `@[extern]` functions whose effects the walker fires
/// directly: stdout / stderr writes. When the head
/// matches and the trailing string-shaped arg decodes
/// cleanly, this writes the value + returns `Some(())`
/// to signal "effect handled". Anything else returns
/// `None` so the caller continues through the regular
/// `dispatch_extern_const` arm.
///
/// Recognised heads (all share the `String → IO Unit`
/// shape; arg decode reads a `Lit(Str)` or a constant-
/// time-foldable string-typed expression at the
/// trailing arg position):
///
/// - `IO.println` / `IO_println` — writes `s\n` to
///   stdout.
/// - `IO.eprintln` / `IO_eprintln` — writes `s\n` to
///   stderr.
/// - `IO.print` / `IO_print` — writes `s` (no newline)
///   to stdout.
/// - `IO.eprint` / `IO_eprint` — writes `s` (no newline)
///   to stderr.
///
/// Doesn't yet recognise: `IO.FS.*` (file I/O — needs
/// path + handle plumbing), `IO.getLine` / `IO.getEnv`
/// (read-side primitives — need a result feed back into
/// the walker), `dbg_trace` / `panic!` (compile-time
/// elaboration, not IO-level).
fn try_dispatch_io_builtin(name: &Name, args: &[&Expr]) -> Option<()> {
    let name_str = name.to_string();
    let (handler, has_newline, to_stderr): (&str, bool, bool) =
        match name_str.as_str() {
            "IO.println" | "IO_println" => ("println", true, false),
            "IO.eprintln" | "IO_eprintln" => ("eprintln", true, true),
            "IO.print" | "IO_print" => ("print", false, false),
            "IO.eprint" | "IO_eprint" => ("eprint", false, true),
            _ => return None,
        };
    let _ = handler;
    // Last arg is the string payload.
    let payload = args.last()?;
    let s = decode_string_arg(payload)?;
    if to_stderr {
        if has_newline {
            eprintln!("{s}");
        } else {
            eprint!("{s}");
        }
    } else if has_newline {
        println!("{s}");
    } else {
        print!("{s}");
    }
    Some(())
}

/// Recognise the Lean stdlib `IO.FS.*` file-system
/// `@[extern]` declarations + fire their effect against
/// the host file system. Returns `Some(WalkResult)` when
/// the head matches (regardless of whether the dispatch
/// succeeded — IO errors surface as `Err(ExternFailed)`),
/// `None` so the caller falls through to the regular
/// resolver dispatch path.
///
/// Recognised heads + their host calls (Lean signatures
/// abbreviated — `System.FilePath` reduces to `String` on
/// the wire; the walker takes the trailing arg as the
/// path via `decode_string_arg`):
///
/// - `IO.FS.readFile : FilePath → IO String` →
///   `std::fs::read_to_string` → `Ok(Some(Lit(Str)))`.
/// - `IO.FS.writeFile : FilePath → String → IO Unit` →
///   `std::fs::write` → `Ok(None)`.
/// - `IO.FS.appendFile : FilePath → String → IO Unit` →
///   `std::fs::OpenOptions::append` → `Ok(None)`.
/// - `IO.FS.removeFile : FilePath → IO Unit` →
///   `std::fs::remove_file` → `Ok(None)`.
/// - `IO.FS.createDir : FilePath → IO Unit` →
///   `std::fs::create_dir` → `Ok(None)`.
/// - `IO.FS.createDirAll : FilePath → IO Unit` →
///   `std::fs::create_dir_all` → `Ok(None)`.
/// - `IO.FS.removeDir : FilePath → IO Unit` →
///   `std::fs::remove_dir` → `Ok(None)`.
/// - `IO.FS.removeDirAll : FilePath → IO Unit` →
///   `std::fs::remove_dir_all` → `Ok(None)`.
/// - `IO.FS.rename : FilePath → FilePath → IO Unit` →
///   `std::fs::rename` → `Ok(None)`.
///
/// `IO.FS.Handle.*` (handle-based readers/writers) and
/// `IO.FS.DirEntry.*` are deliberately out of scope — they
/// need handle plumbing (Rust `File` lifetime tied to a
/// Lean value) that the walker doesn't model. Embedders
/// can intercept those via the resolver.
fn try_dispatch_io_fs_builtin(name: &Name, args: &[&Expr]) -> Option<WalkResult> {
    let name_str = name.to_string();
    match name_str.as_str() {
        "IO.FS.readFile" | "IO_FS_readFile" => {
            let path = decode_string_arg(args.last()?)?;
            Some(match std::fs::read_to_string(&path) {
                Ok(contents) => Ok(Some(Expr::Lit(Literal::Str(contents)))),
                Err(e) => Err(extern_fs_error("IO.FS.readFile", &path, e)),
            })
        }
        "IO.FS.writeFile" | "IO_FS_writeFile" => {
            if args.len() < 2 {
                return None;
            }
            let path = decode_string_arg(args[args.len() - 2])?;
            let contents = decode_string_arg(args.last()?)?;
            Some(match std::fs::write(&path, &contents) {
                Ok(()) => Ok(None),
                Err(e) => Err(extern_fs_error("IO.FS.writeFile", &path, e)),
            })
        }
        "IO.FS.appendFile" | "IO_FS_appendFile" => {
            if args.len() < 2 {
                return None;
            }
            let path = decode_string_arg(args[args.len() - 2])?;
            let contents = decode_string_arg(args.last()?)?;
            use std::io::Write;
            Some(
                match std::fs::OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(&path)
                    .and_then(|mut f| f.write_all(contents.as_bytes()))
                {
                    Ok(()) => Ok(None),
                    Err(e) => Err(extern_fs_error("IO.FS.appendFile", &path, e)),
                },
            )
        }
        "IO.FS.removeFile" | "IO_FS_removeFile" => {
            let path = decode_string_arg(args.last()?)?;
            Some(match std::fs::remove_file(&path) {
                Ok(()) => Ok(None),
                Err(e) => Err(extern_fs_error("IO.FS.removeFile", &path, e)),
            })
        }
        "IO.FS.createDir" | "IO_FS_createDir" => {
            let path = decode_string_arg(args.last()?)?;
            Some(match std::fs::create_dir(&path) {
                Ok(()) => Ok(None),
                Err(e) => Err(extern_fs_error("IO.FS.createDir", &path, e)),
            })
        }
        "IO.FS.createDirAll" | "IO_FS_createDirAll" => {
            let path = decode_string_arg(args.last()?)?;
            Some(match std::fs::create_dir_all(&path) {
                Ok(()) => Ok(None),
                Err(e) => Err(extern_fs_error("IO.FS.createDirAll", &path, e)),
            })
        }
        "IO.FS.removeDir" | "IO_FS_removeDir" => {
            let path = decode_string_arg(args.last()?)?;
            Some(match std::fs::remove_dir(&path) {
                Ok(()) => Ok(None),
                Err(e) => Err(extern_fs_error("IO.FS.removeDir", &path, e)),
            })
        }
        "IO.FS.removeDirAll" | "IO_FS_removeDirAll" => {
            let path = decode_string_arg(args.last()?)?;
            Some(match std::fs::remove_dir_all(&path) {
                Ok(()) => Ok(None),
                Err(e) => Err(extern_fs_error("IO.FS.removeDirAll", &path, e)),
            })
        }
        "IO.FS.rename" | "IO_FS_rename" => {
            if args.len() < 2 {
                return None;
            }
            let from = decode_string_arg(args[args.len() - 2])?;
            let to = decode_string_arg(args.last()?)?;
            Some(match std::fs::rename(&from, &to) {
                Ok(()) => Ok(None),
                Err(e) => {
                    Err(extern_fs_error("IO.FS.rename", &format!("{from} -> {to}"), e))
                }
            })
        }
        _ => None,
    }
}

/// Wrap a `std::io::Error` into a `DriverError::ExternFailed`
/// with a `CallbackFailed` payload describing the failed
/// FS call. Matches the framing the resolver-side
/// dispatch path uses for cdylib errors, so embedders that
/// observe `ExternFailed` don't need to special-case
/// builtin failures separately.
fn extern_fs_error(
    op: &str,
    arg: &str,
    err: std::io::Error,
) -> DriverError {
    DriverError::ExternFailed(oxilean_kernel::ffi::ExternCallError::CallbackFailed(
        format!("{op}({arg}): {err}"),
    ))
}

/// Extract a Rust `String` from an Expr shape the walker
/// can statically lower. Covers:
///
/// - `Expr::Lit(Literal::Str(s))` — direct string literal.
/// - `Expr::App(Const("String.mk", _), <inner>)` — the
///   String-constructor wrap Lean inserts when an
///   elaborator round-trip goes through `List Char`. v0
///   walker doesn't synthesise from `List Char` yet;
///   returns `None`.
/// - `Expr::App(Const("toString", _), <inner>)` — the
///   `ToString` projection. v0 only handles the case
///   where `inner` is itself a `Lit(Str)`.
///
/// Anything richer (composite types, user-defined
/// `ToString` instances) returns `None` so the caller
/// falls through to the regular resolver dispatch path.
fn decode_string_arg(arg: &Expr) -> Option<String> {
    match arg {
        Expr::Lit(Literal::Str(s)) => Some(s.clone()),
        Expr::App(head, inner) => {
            let head_name = if let Expr::Const(n, _) = head.as_ref() {
                n.to_string()
            } else {
                return None;
            };
            match head_name.as_str() {
                "toString" | "ToString.toString" => decode_string_arg(inner),
                _ => None,
            }
        }
        _ => None,
    }
}

/// Encode the App-chain shape Lean's elaborator produces
/// when it inserts a typeclass-projection method around a
/// kernel-level leaf. The shapes the walker recognises:
///
/// **`@OfNat.ofNat <type> <Lit(Nat)> <instance>`** — Lean
/// elaborates `(42 : UInt64)` as
/// `@OfNat.ofNat UInt64 42 <instOfNatUInt64>`. The arity
/// after decomposition is 3 (type, value, instance) with
/// `head_args[0] = Const("<type>", _)`, `head_args[1] =
/// Lit(Nat n)`, `head_args[2]` = the instance Expr (ignored
/// — we read the size from the type). Wire form follows
/// `SPEC/canonical-abi.md` §6: u8/16/32/64/128 (and i*) as
/// little-endian 1/2/4/8/16 bytes, signed types via two's
/// complement of the same byte width.
///
/// **`@Neg.neg <type> <inst> <inner>`** — `(-42 : Int64)`
/// elaborates as `@Neg.neg Int64 _inst (@OfNat.ofNat Int64
/// 42 _inst2)`. Decompose `inner` recursively; if it's an
/// `OfNat.ofNat` over the same type, encode the **negated**
/// value with the right signed width.
///
/// **`@Char.ofNat <Lit(Nat n)>`** — Lean elaborates `'A'`
/// as `Char.ofNat 65`. Wire form is u32 LE Unicode code
/// point. Arity 1 after decomposition.
///
/// **`@Float32.ofBinaryScientific …`** /
/// **`@Float.ofBinaryScientific …`** — Lean's
/// float-literal lowering. v0 walker doesn't statically
/// fold these (constant-time arithmetic at the kernel
/// layer is OxiLean's job, not the walker's), so they
/// return `None` and the caller falls back to the empty
/// buffer. A fixture surfacing this gap moves the
/// boundary forward.
fn encode_typeclass_projection(
    head: &Expr,
    head_args: &[&Expr],
    out: &mut Vec<u8>,
    env: &Environment,
) -> Option<()> {
    let Expr::Const(name, _) = head else {
        return None;
    };
    let name_str = name.to_string();
    match name_str.as_str() {
        "OfNat.ofNat" | "OfNat_ofNat" => {
            // arity 3: type, value, instance (instance
            // ignored). Some elaborator paths leave only
            // arity 2 (no instance) — accept both.
            if head_args.len() < 2 {
                return None;
            }
            let ty_expr = head_args[0];
            let value_expr = head_args[1];
            let n = match value_expr {
                Expr::Lit(Literal::Nat(n)) => *n,
                _ => return None,
            };
            encode_sized_integer(ty_expr, n, /*negate=*/ false, out)?;
        }
        "Neg.neg" | "Neg_neg" => {
            // arity 3: type, instance, inner. inner is
            // expected to be `@OfNat.ofNat <type>
            // <Lit(Nat)> _`.
            if head_args.len() < 3 {
                return None;
            }
            let ty_expr = head_args[0];
            let inner = head_args[2];
            let (inner_head, inner_args) = decompose_app(inner);
            let Expr::Const(inner_name, _) = inner_head else {
                return None;
            };
            let inner_name_str = inner_name.to_string();
            if !matches!(inner_name_str.as_str(), "OfNat.ofNat" | "OfNat_ofNat") {
                return None;
            }
            if inner_args.len() < 2 {
                return None;
            }
            let n = match inner_args[1] {
                Expr::Lit(Literal::Nat(n)) => *n,
                _ => return None,
            };
            encode_sized_integer(ty_expr, n, /*negate=*/ true, out)?;
        }
        "Char.ofNat" | "Char_ofNat" => {
            // arity 1: a Nat literal that's the Unicode
            // code point. Wire form is u32 LE.
            if head_args.is_empty() {
                return None;
            }
            let n = match head_args[head_args.len() - 1] {
                Expr::Lit(Literal::Nat(n)) => *n,
                _ => return None,
            };
            let cp = u32::try_from(n).ok()?;
            out.extend_from_slice(&cp.to_le_bytes());
        }
        // Canonical record / variant ctors. Walker only
        // recognises the small fixed set defined here; user-
        // defined structures + inductives require IDL-side
        // knowledge of which fields cross the boundary and
        // are therefore left to the embedder.
        "Prod.mk" | "Prod_mk" => {
            // `@Prod.mk α β a b` — wire format per
            // SPEC/canonical-abi.md §7 (tuple): `a ‖ b`,
            // no framing. Decomposed arity is 4 (type α,
            // type β, value a, value b); take the trailing
            // two as the encodable values.
            if head_args.len() < 2 {
                return None;
            }
            let a = head_args[head_args.len() - 2];
            let b = head_args[head_args.len() - 1];
            encode_one_arg(a, out, env)?;
            encode_one_arg(b, out, env)?;
        }
        "Subtype.mk" | "Subtype_mk" => {
            // `@Subtype.mk α p val proof` — proof is
            // erased on the wire; `val` is the only
            // observable payload. SPEC/canonical-abi.md
            // §11.2 (proof-carrying record): `val` alone.
            // Decomposed arity 4: type, predicate, value,
            // proof. Take the second-from-last as the
            // value; last is the proof (ignored).
            if head_args.len() < 2 {
                return None;
            }
            let val = head_args[head_args.len() - 2];
            encode_one_arg(val, out, env)?;
        }
        "Option.none" | "Option_none" => {
            // SPEC/canonical-abi.md §10 (variant tag):
            // 4 bytes u32 LE = 0. `Option.none` is the
            // null arm. App-chain may carry one type
            // implicit arg; ignored.
            out.extend_from_slice(&0u32.to_le_bytes());
        }
        "Option.some" | "Option_some" => {
            // 4 bytes u32 LE = 1, then the payload
            // value. Decomposed arity is 2 (type α,
            // value); take the trailing one.
            if head_args.is_empty() {
                return None;
            }
            out.extend_from_slice(&1u32.to_le_bytes());
            let val = head_args[head_args.len() - 1];
            encode_one_arg(val, out, env)?;
        }
        "Sum.inl" | "Sum_inl" => {
            // 4 bytes u32 LE = 0, then the payload.
            // Decomposed arity is 3 (type α, type β,
            // value); take the trailing one.
            if head_args.is_empty() {
                return None;
            }
            out.extend_from_slice(&0u32.to_le_bytes());
            let val = head_args[head_args.len() - 1];
            encode_one_arg(val, out, env)?;
        }
        "Sum.inr" | "Sum_inr" => {
            if head_args.is_empty() {
                return None;
            }
            out.extend_from_slice(&1u32.to_le_bytes());
            let val = head_args[head_args.len() - 1];
            encode_one_arg(val, out, env)?;
        }
        _ => {
            // Fall back to user-defined ctor encoding via
            // env lookup. When the name resolves to a
            // `ConstantInfo::Constructor`, the encoder
            // emits SPEC §10/§11 record / variant wire
            // bytes against the parent inductive's ctor
            // count (multi-ctor → u32 LE discriminant
            // prefix + payload; single-ctor record → just
            // the payload). Type-instantiation args at
            // the head of the App-chain are skipped using
            // the ctor's `num_params` count.
            encode_user_defined_ctor(name, head_args, out, env)?;
        }
    }
    Some(())
}

/// Encode a user-defined record / inductive ctor by
/// walking its `ConstantInfo::Constructor` metadata in
/// `env`. Recognises any ctor name registered as a
/// `Constructor` decl (the lake plugin emits these when
/// `@[leo4_export]` discovers user-defined types; OxiLean
/// also synthesises them during inductive elaboration).
///
/// Wire format (matches SPEC/canonical-abi.md §10/§11):
///
/// - **Multi-ctor inductive** (`iv.ctors.len() > 1`,
///   `Sum`-like): u32 LE discriminant (= `cv.cidx`) +
///   each field encoded in order via `encode_one_arg`.
/// - **Single-ctor inductive** (`iv.ctors.len() == 1`,
///   structure / record): no discriminant; each field
///   encoded in order via `encode_one_arg`. Matches
///   `Prod.mk` / `Subtype.mk` shape — those are explicit
///   arms above for symmetry with the SPEC's named
///   record types, but anonymous user records share the
///   wire format.
///
/// Skips the leading `cv.num_params` args (these are
/// the inductive's type parameters, instantiated by
/// the elaborator). The remaining `cv.num_fields` args
/// are the actual field values.
///
/// Returns `None` (caller falls through to empty-buffer
/// dispatch) when:
///
/// - `ctor_name` doesn't resolve to a `Constructor` in
///   `env` (could be a Definition / Axiom / typeclass
///   projection name — the caller's existing arms cover
///   those).
/// - The parent inductive isn't in `env` (env was
///   incompletely populated).
/// - Any value-arg fails to lower via `encode_one_arg`
///   (recursive composition).
/// - The arity check fails (too few args supplied;
///   under-applied ctor App-chain).
fn encode_user_defined_ctor(
    ctor_name: &Name,
    head_args: &[&Expr],
    out: &mut Vec<u8>,
    env: &Environment,
) -> Option<()> {
    let info = env.find(ctor_name)?;
    let cv = match info {
        ConstantInfo::Constructor(cv) => cv,
        _ => return None,
    };
    let parent = env.find(&cv.induct)?;
    let iv = match parent {
        ConstantInfo::Inductive(iv) => iv,
        _ => return None,
    };
    // Multi-ctor inductive: prefix with discriminant.
    if iv.ctors.len() > 1 {
        out.extend_from_slice(&cv.cidx.to_le_bytes());
    }
    let nparams = cv.num_params as usize;
    if head_args.len() < nparams {
        return None;
    }
    let value_args = &head_args[nparams..];
    for a in value_args {
        encode_one_arg(a, out, env)?;
    }
    Some(())
}

/// Encode a sized-integer value against the type Expr.
/// `negate = true` means the wire bytes carry `-n` in two's
/// complement at the type's width. Recognised types:
/// UInt8/16/32/64/128, Int8/16/32/64/128, USize/ISize
/// (host width).
fn encode_sized_integer(
    ty_expr: &Expr,
    n: u64,
    negate: bool,
    out: &mut Vec<u8>,
) -> Option<()> {
    let Expr::Const(ty_name, _) = ty_expr else {
        return None;
    };
    let ty_str = ty_name.to_string();
    match ty_str.as_str() {
        // Unsigned sized integers.
        "UInt8" => {
            if negate {
                return None; // Neg.neg over an unsigned type doesn't elaborate.
            }
            let v = u8::try_from(n).ok()?;
            out.push(v);
        }
        "UInt16" => {
            if negate {
                return None;
            }
            let v = u16::try_from(n).ok()?;
            out.extend_from_slice(&v.to_le_bytes());
        }
        "UInt32" => {
            if negate {
                return None;
            }
            let v = u32::try_from(n).ok()?;
            out.extend_from_slice(&v.to_le_bytes());
        }
        "UInt64" => {
            if negate {
                return None;
            }
            out.extend_from_slice(&n.to_le_bytes());
        }
        "UInt128" => {
            if negate {
                return None;
            }
            let v = u128::from(n);
            out.extend_from_slice(&v.to_le_bytes());
        }
        "USize" => {
            if negate {
                return None;
            }
            let v = usize::try_from(n).ok()?;
            out.extend_from_slice(&(v as u64).to_le_bytes());
        }
        // Signed sized integers. `negate = true` flips the
        // sign; `negate = false` accepts the natural-number
        // representation `(n : Int*)`.
        "Int8" => {
            let v = i8::try_from(n).ok()?;
            let signed = if negate { v.checked_neg()? } else { v };
            out.extend_from_slice(&signed.to_le_bytes());
        }
        "Int16" => {
            let v = i16::try_from(n).ok()?;
            let signed = if negate { v.checked_neg()? } else { v };
            out.extend_from_slice(&signed.to_le_bytes());
        }
        "Int32" => {
            let v = i32::try_from(n).ok()?;
            let signed = if negate { v.checked_neg()? } else { v };
            out.extend_from_slice(&signed.to_le_bytes());
        }
        "Int64" => {
            let v = i64::try_from(n).ok()?;
            let signed = if negate { v.checked_neg()? } else { v };
            out.extend_from_slice(&signed.to_le_bytes());
        }
        "Int128" => {
            let v = i128::try_from(n).ok()?;
            let signed = if negate { v.checked_neg()? } else { v };
            out.extend_from_slice(&signed.to_le_bytes());
        }
        "ISize" => {
            let v = isize::try_from(n).ok()?;
            let signed = if negate { v.checked_neg()? } else { v };
            out.extend_from_slice(&(signed as i64).to_le_bytes());
        }
        _ => return None,
    }
    Some(())
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

    // ─── encode_leaf_args_for_extern unit tests ───────

    #[test]
    fn encode_nat_literal_is_u64_le() {
        let arg = Expr::Lit(Literal::Nat(0xDEAD_BEEF));
        let out = encode_leaf_args_for_extern(&[&arg], &empty_env()).unwrap();
        assert_eq!(out, vec![0xEF, 0xBE, 0xAD, 0xDE, 0, 0, 0, 0]);
    }

    #[test]
    fn encode_string_literal_is_len_prefix_plus_utf8() {
        let arg = Expr::Lit(Literal::Str("hi".to_string()));
        let out = encode_leaf_args_for_extern(&[&arg], &empty_env()).unwrap();
        // u32 LE length (2) + "hi" bytes.
        assert_eq!(out, vec![2, 0, 0, 0, b'h', b'i']);
    }

    #[test]
    fn encode_bool_ctors_are_one_byte_each() {
        let bt = Expr::Const(Name::str("Bool.true"), Vec::new());
        let bf = Expr::Const(Name::str("Bool.false"), Vec::new());
        let out = encode_leaf_args_for_extern(&[&bt, &bf], &empty_env()).unwrap();
        assert_eq!(out, vec![0x01, 0x00]);
    }

    #[test]
    fn encode_unit_is_zero_bytes() {
        let u = Expr::Const(Name::str("Unit.unit"), Vec::new());
        let out = encode_leaf_args_for_extern(&[&u], &empty_env()).unwrap();
        assert!(out.is_empty());
    }

    #[test]
    fn encode_unknown_const_returns_none() {
        // `Foo.bar` isn't in the recognised leaf set —
        // walker falls back to `&[]` and the resolver
        // still fires.
        let arg = Expr::Const(Name::str("Foo.bar"), Vec::new());
        assert!(encode_leaf_args_for_extern(&[&arg], &empty_env()).is_none());
    }

    #[test]
    fn encode_app_headed_arg_returns_none() {
        // `(IO.pure x)` is App-headed; encoder bails so
        // the walker forwards `&[]`.
        let arg = Expr::App(
            Box::new(Expr::Const(Name::str("IO.pure"), Vec::new())),
            Box::new(Expr::Const(Name::str("Unit.unit"), Vec::new())),
        );
        assert!(encode_leaf_args_for_extern(&[&arg], &empty_env()).is_none());
    }

    #[test]
    fn encode_concatenates_multiple_args_in_order() {
        let a = Expr::Lit(Literal::Nat(1));
        let b = Expr::Const(Name::str("Bool.true"), Vec::new());
        let c = Expr::Lit(Literal::Str("x".to_string()));
        let out = encode_leaf_args_for_extern(&[&a, &b, &c], &empty_env()).unwrap();
        // u64 LE 1 ‖ 0x01 ‖ u32 LE 1 ‖ 'x'
        let mut expected = Vec::new();
        expected.extend_from_slice(&1u64.to_le_bytes());
        expected.push(0x01);
        expected.extend_from_slice(&1u32.to_le_bytes());
        expected.push(b'x');
        assert_eq!(out, expected);
    }

    // ─── OfNat.ofNat / Neg.neg / Char.ofNat encoder ────

    fn ofnat_of(ty: &str, n: u64) -> Expr {
        // `@OfNat.ofNat <ty> <n> <instance>` — instance
        // is opaque to the encoder; use a sentinel Const.
        Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("OfNat.ofNat"), Vec::new())),
                    Box::new(Expr::Const(Name::str(ty), Vec::new())),
                )),
                Box::new(Expr::Lit(Literal::Nat(n))),
            )),
            Box::new(Expr::Const(Name::str("_inst"), Vec::new())),
        )
    }

    fn neg_of(ty: &str, inner: Expr) -> Expr {
        // `@Neg.neg <ty> <inst> <inner>`.
        Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Neg.neg"), Vec::new())),
                    Box::new(Expr::Const(Name::str(ty), Vec::new())),
                )),
                Box::new(Expr::Const(Name::str("_inst"), Vec::new())),
            )),
            Box::new(inner),
        )
    }

    #[test]
    fn encode_ofnat_uint8_uint16_uint32_uint64_uint128_usize() {
        let cases: Vec<(&str, u64, Vec<u8>)> = vec![
            ("UInt8", 0x2A, vec![0x2A]),
            ("UInt16", 0xBEEF, vec![0xEF, 0xBE]),
            ("UInt32", 0xDEAD_BEEF, vec![0xEF, 0xBE, 0xAD, 0xDE]),
            (
                "UInt64",
                0x0102_0304_0506_0708,
                vec![0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01],
            ),
            (
                "UInt128",
                7,
                vec![7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            ),
            (
                "USize",
                42,
                vec![42, 0, 0, 0, 0, 0, 0, 0],
            ),
        ];
        for (ty, n, expected) in cases {
            let arg = ofnat_of(ty, n);
            let out = encode_leaf_args_for_extern(&[&arg], &empty_env())
                .unwrap_or_else(|| panic!("{ty} should encode"));
            assert_eq!(out, expected, "{ty} value {n}");
        }
    }

    #[test]
    fn encode_ofnat_int_positive_sizes() {
        // `(42 : Int8)` etc. without Neg.neg wrapping —
        // signed types accept the positive natural-number
        // OfNat path. Wire form is the same byte width as
        // the matching unsigned.
        let cases: Vec<(&str, u64, Vec<u8>)> = vec![
            ("Int8", 42, vec![42]),
            ("Int16", 0x07F0, vec![0xF0, 0x07]),
            (
                "Int32",
                0x0102_0304,
                vec![0x04, 0x03, 0x02, 0x01],
            ),
            (
                "Int64",
                0x1122_3344_5566_7788,
                vec![0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11],
            ),
            (
                "Int128",
                1,
                vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            ),
            (
                "ISize",
                100,
                vec![100, 0, 0, 0, 0, 0, 0, 0],
            ),
        ];
        for (ty, n, expected) in cases {
            let arg = ofnat_of(ty, n);
            let out = encode_leaf_args_for_extern(&[&arg], &empty_env())
                .unwrap_or_else(|| panic!("{ty} should encode"));
            assert_eq!(out, expected, "{ty} value {n}");
        }
    }

    #[test]
    fn encode_neg_negates_signed_integers_at_correct_width() {
        // `(-42 : Int8)` → wire byte `0xD6` (two's
        // complement of 42 at 8 bits).
        let arg = neg_of("Int8", ofnat_of("Int8", 42));
        let out = encode_leaf_args_for_extern(&[&arg], &empty_env()).unwrap();
        assert_eq!(out, vec![0xD6]);

        // `(-1 : Int32)` → `[0xFF, 0xFF, 0xFF, 0xFF]`.
        let arg = neg_of("Int32", ofnat_of("Int32", 1));
        let out = encode_leaf_args_for_extern(&[&arg], &empty_env()).unwrap();
        assert_eq!(out, vec![0xFF, 0xFF, 0xFF, 0xFF]);

        // `(-1 : Int64)` → eight 0xFFs.
        let arg = neg_of("Int64", ofnat_of("Int64", 1));
        let out = encode_leaf_args_for_extern(&[&arg], &empty_env()).unwrap();
        assert_eq!(out, vec![0xFF; 8]);
    }

    #[test]
    fn encode_neg_rejects_unsigned_types() {
        // `Neg.neg` over `UInt32` doesn't elaborate (no
        // `Neg UInt32` instance); the encoder bails so the
        // walker falls back to `&[]`.
        let arg = neg_of("UInt32", ofnat_of("UInt32", 5));
        assert!(encode_leaf_args_for_extern(&[&arg], &empty_env()).is_none());
    }

    #[test]
    fn encode_char_ofnat_writes_u32_le_code_point() {
        // `'A'` = `Char.ofNat 65` → 4 bytes LE.
        let arg = Expr::App(
            Box::new(Expr::Const(Name::str("Char.ofNat"), Vec::new())),
            Box::new(Expr::Lit(Literal::Nat(65))),
        );
        let out = encode_leaf_args_for_extern(&[&arg], &empty_env()).unwrap();
        assert_eq!(out, vec![65, 0, 0, 0]);

        // BMP-side code point check.
        let arg = Expr::App(
            Box::new(Expr::Const(Name::str("Char.ofNat"), Vec::new())),
            Box::new(Expr::Lit(Literal::Nat(0x4E2D))), // 中
        );
        let out = encode_leaf_args_for_extern(&[&arg], &empty_env()).unwrap();
        assert_eq!(out, vec![0x2D, 0x4E, 0, 0]);
    }

    #[test]
    fn encode_ofnat_overflowing_value_rejected() {
        // `(256 : UInt8)` overflows — encoder bails so
        // the walker falls back rather than wrapping
        // silently.
        let arg = ofnat_of("UInt8", 256);
        assert!(encode_leaf_args_for_extern(&[&arg], &empty_env()).is_none());
    }

    #[test]
    fn encode_mixed_leaf_and_ofnat_concatenates_correctly() {
        // resolver receives `[Bool.true, (42 : UInt32), "x"]`
        let bool_t = Expr::Const(Name::str("Bool.true"), Vec::new());
        let u32_42 = ofnat_of("UInt32", 42);
        let str_x = Expr::Lit(Literal::Str("x".to_string()));
        let out = encode_leaf_args_for_extern(&[&bool_t, &u32_42, &str_x], &empty_env()).unwrap();
        let mut expected = Vec::new();
        expected.push(0x01);
        expected.extend_from_slice(&42u32.to_le_bytes());
        expected.extend_from_slice(&1u32.to_le_bytes());
        expected.push(b'x');
        assert_eq!(out, expected);
    }

    // ─── Composite-type ctors ──────────────────────────

    fn prod_mk(a: Expr, b: Expr) -> Expr {
        // `@Prod.mk α β a b` — type implicits use sentinel
        // Const placeholders that the encoder ignores.
        let alpha = Expr::Const(Name::str("_α"), Vec::new());
        let beta = Expr::Const(Name::str("_β"), Vec::new());
        Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Prod.mk"), Vec::new())),
                        Box::new(alpha),
                    )),
                    Box::new(beta),
                )),
                Box::new(a),
            )),
            Box::new(b),
        )
    }

    #[test]
    fn encode_prod_mk_concatenates_fields() {
        // `(true, 0x2A : Bool × UInt8)` → 0x01 ‖ 0x2A.
        let arg = prod_mk(
            Expr::Const(Name::str("Bool.true"), Vec::new()),
            ofnat_of("UInt8", 0x2A),
        );
        let out = encode_leaf_args_for_extern(&[&arg], &empty_env()).unwrap();
        assert_eq!(out, vec![0x01, 0x2A]);
    }

    #[test]
    fn encode_prod_mk_nested_recurses_correctly() {
        // `((1, 2), 3) : (UInt8 × UInt8) × UInt8` → 1 ‖ 2 ‖ 3.
        let inner = prod_mk(ofnat_of("UInt8", 1), ofnat_of("UInt8", 2));
        let outer = prod_mk(inner, ofnat_of("UInt8", 3));
        let out = encode_leaf_args_for_extern(&[&outer], &empty_env()).unwrap();
        assert_eq!(out, vec![1, 2, 3]);
    }

    #[test]
    fn encode_subtype_mk_writes_only_val_dropping_proof() {
        // `@Subtype.mk Nat _pred 42 _proof` — proof
        // erased; val = 42 wrapped via OfNat.ofNat over
        // Int8 for a determinate byte width.
        let val = ofnat_of("Int8", 42);
        let arg = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Subtype.mk"), Vec::new())),
                        Box::new(Expr::Const(Name::str("_α"), Vec::new())),
                    )),
                    Box::new(Expr::Const(Name::str("_pred"), Vec::new())),
                )),
                Box::new(val),
            )),
            Box::new(Expr::Const(Name::str("_proof"), Vec::new())),
        );
        let out = encode_leaf_args_for_extern(&[&arg], &empty_env()).unwrap();
        assert_eq!(out, vec![42]);
    }

    #[test]
    fn encode_option_none_writes_zero_tag() {
        let arg = Expr::Const(Name::str("Option.none"), Vec::new());
        let out = encode_leaf_args_for_extern(&[&arg], &empty_env()).unwrap();
        assert_eq!(out, vec![0, 0, 0, 0]);
    }

    #[test]
    fn encode_option_some_writes_tag_then_payload() {
        // `Option.some (42 : UInt8)` → tag 1 ‖ 0x2A.
        let arg = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Option.some"), Vec::new())),
                Box::new(Expr::Const(Name::str("_α"), Vec::new())),
            )),
            Box::new(ofnat_of("UInt8", 0x2A)),
        );
        let out = encode_leaf_args_for_extern(&[&arg], &empty_env()).unwrap();
        assert_eq!(out, vec![1, 0, 0, 0, 0x2A]);
    }

    #[test]
    fn encode_sum_inl_inr_distinguish_tags() {
        // `Sum.inl (0xFF : UInt8) : Sum UInt8 UInt16` →
        // tag 0 ‖ 0xFF.
        let inl = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Sum.inl"), Vec::new())),
                    Box::new(Expr::Const(Name::str("_α"), Vec::new())),
                )),
                Box::new(Expr::Const(Name::str("_β"), Vec::new())),
            )),
            Box::new(ofnat_of("UInt8", 0xFF)),
        );
        let out = encode_leaf_args_for_extern(&[&inl], &empty_env()).unwrap();
        assert_eq!(out, vec![0, 0, 0, 0, 0xFF]);

        // `Sum.inr (0x4242 : UInt16) : Sum UInt8 UInt16`
        // → tag 1 ‖ 0x42 0x42 (little-endian).
        let inr = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Sum.inr"), Vec::new())),
                    Box::new(Expr::Const(Name::str("_α"), Vec::new())),
                )),
                Box::new(Expr::Const(Name::str("_β"), Vec::new())),
            )),
            Box::new(ofnat_of("UInt16", 0x4242)),
        );
        let out = encode_leaf_args_for_extern(&[&inr], &empty_env()).unwrap();
        assert_eq!(out, vec![1, 0, 0, 0, 0x42, 0x42]);
    }

    #[test]
    fn encode_user_defined_struct_ctor_falls_back_to_none() {
        // `Foo.mk a b` for an unknown `Foo` returns None;
        // the walker forwards `&[]` so embedder-side IDL-
        // aware encoding can layer on top.
        let arg = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Foo.mk"), Vec::new())),
                Box::new(Expr::Lit(Literal::Nat(1))),
            )),
            Box::new(Expr::Lit(Literal::Nat(2))),
        );
        assert!(encode_leaf_args_for_extern(&[&arg], &empty_env()).is_none());
    }

    // ─── IO builtin dispatch ───────────────────────────

    #[test]
    fn try_dispatch_io_builtin_recognises_println_with_lit_str() {
        let name = Name::str("IO.println");
        let s = Expr::Lit(Literal::Str("hello".to_string()));
        // We don't actually want to assert on stdout
        // capture here — `println!` is wired through
        // process stdout. Verify the predicate returns
        // `Some(())` (i.e. the walker would short-
        // circuit) rather than the contents.
        assert!(try_dispatch_io_builtin(&name, &[&s]).is_some());
    }

    #[test]
    fn try_dispatch_io_builtin_recognises_eprintln_eprint_print() {
        for builtin in [
            "IO.println",
            "IO.eprintln",
            "IO.print",
            "IO.eprint",
        ] {
            let n = Name::str(builtin);
            let s = Expr::Lit(Literal::Str("x".to_string()));
            assert!(
                try_dispatch_io_builtin(&n, &[&s]).is_some(),
                "{builtin} should dispatch"
            );
        }
    }

    #[test]
    fn try_dispatch_io_builtin_rejects_non_string_arg() {
        let name = Name::str("IO.println");
        let arg = Expr::Lit(Literal::Nat(42));
        assert!(try_dispatch_io_builtin(&name, &[&arg]).is_none());
    }

    #[test]
    fn try_dispatch_io_builtin_rejects_unknown_name() {
        let name = Name::str("Foo.bar");
        let s = Expr::Lit(Literal::Str("hi".to_string()));
        assert!(try_dispatch_io_builtin(&name, &[&s]).is_none());
    }

    #[test]
    fn try_dispatch_io_builtin_walks_to_string_wrap() {
        // `IO.println (toString s)` — the elaborator
        // inserts the projection. Walker decodes through
        // it.
        let name = Name::str("IO.println");
        let inner = Expr::Lit(Literal::Str("via toString".to_string()));
        let to_string = Expr::App(
            Box::new(Expr::Const(Name::str("toString"), Vec::new())),
            Box::new(inner),
        );
        assert!(try_dispatch_io_builtin(&name, &[&to_string]).is_some());
    }

    #[test]
    fn run_main_io_println_dispatches_without_resolver() {
        // `def main : IO Unit := IO.println "hi"`
        // — no resolver-side hook needed; the walker
        // itself handles the effect.
        let mut env = empty_env();
        let main = Name::str("main");
        let io_unit_ty = Expr::App(
            Box::new(Expr::Const(Name::str("IO"), Vec::new())),
            Box::new(Expr::Const(Name::str("Unit"), Vec::new())),
        );
        // Have to declare the extern axiom so env.find
        // can resolve it, but the walker's IO-builtin arm
        // fires *before* `dispatch_extern_const` so no
        // matching ExternRegistry entry is needed.
        env.add(Declaration::Axiom {
            name: Name::str("IO.println"),
            univ_params: Vec::new(),
            ty: io_unit_ty.clone(),
        })
        .unwrap();
        let body = Expr::App(
            Box::new(Expr::Const(Name::str("IO.println"), Vec::new())),
            Box::new(Expr::Lit(Literal::Str(
                "[driver test] IO.println dispatch".to_string(),
            ))),
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
        // `empty_extern_registry()` deliberately has no
        // entry for IO.println — the builtin arm runs
        // first.
        run_main(&env, &empty_extern_registry(), resolver, &main)
            .expect("IO.println should dispatch via the builtin arm");
    }

    // ─── IO.FS builtin dispatch ─────────────────────────

    /// Build a unique temp file path under the OS temp
    /// dir. Combines pid + thread id + a counter so
    /// parallel tests don't collide.
    fn unique_tempfile_path(tag: &str) -> std::path::PathBuf {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let n = COUNTER.fetch_add(1, Ordering::SeqCst);
        let pid = std::process::id();
        let tid = format!("{:?}", std::thread::current().id());
        let tid_sanitized: String = tid
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        std::env::temp_dir().join(format!(
            "oxilean-driver-test-{tag}-{pid}-{tid_sanitized}-{n}"
        ))
    }

    #[test]
    fn try_dispatch_io_fs_writefile_then_readfile_roundtrips() {
        let path = unique_tempfile_path("rw.txt");
        let path_str = path.to_string_lossy().into_owned();
        let payload = "hello, IO.FS";

        // writeFile path "hello, IO.FS"
        let path_e = Expr::Lit(Literal::Str(path_str.clone()));
        let payload_e = Expr::Lit(Literal::Str(payload.to_string()));
        let name = Name::str("IO.FS.writeFile");
        let r = try_dispatch_io_fs_builtin(&name, &[&path_e, &payload_e])
            .expect("writeFile head should match");
        assert!(matches!(r, Ok(None)), "writeFile returns Ok(None)");

        // readFile path
        let name = Name::str("IO.FS.readFile");
        let r = try_dispatch_io_fs_builtin(&name, &[&path_e])
            .expect("readFile head should match");
        match r {
            Ok(Some(Expr::Lit(Literal::Str(s)))) => {
                assert_eq!(s, payload, "round-tripped contents must match");
            }
            other => panic!("readFile returned unexpected: {other:?}"),
        }

        // Cleanup.
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn try_dispatch_io_fs_removefile_returns_ok_none() {
        let path = unique_tempfile_path("remove.txt");
        std::fs::write(&path, "scratch").unwrap();
        assert!(path.exists());

        let path_e = Expr::Lit(Literal::Str(path.to_string_lossy().into_owned()));
        let name = Name::str("IO.FS.removeFile");
        let r = try_dispatch_io_fs_builtin(&name, &[&path_e])
            .expect("removeFile head should match");
        assert!(matches!(r, Ok(None)));
        assert!(!path.exists(), "file must be gone after removeFile");
    }

    #[test]
    fn try_dispatch_io_fs_createdir_then_removedir_roundtrips() {
        let dir = unique_tempfile_path("dir");
        let dir_e = Expr::Lit(Literal::Str(dir.to_string_lossy().into_owned()));

        let name = Name::str("IO.FS.createDir");
        let r = try_dispatch_io_fs_builtin(&name, &[&dir_e])
            .expect("createDir head should match");
        assert!(matches!(r, Ok(None)));
        assert!(dir.is_dir(), "directory must exist after createDir");

        let name = Name::str("IO.FS.removeDir");
        let r = try_dispatch_io_fs_builtin(&name, &[&dir_e])
            .expect("removeDir head should match");
        assert!(matches!(r, Ok(None)));
        assert!(!dir.exists(), "directory must be gone after removeDir");
    }

    #[test]
    fn try_dispatch_io_fs_appendfile_appends_to_existing() {
        let path = unique_tempfile_path("append.txt");
        std::fs::write(&path, "first\n").unwrap();
        let path_e = Expr::Lit(Literal::Str(path.to_string_lossy().into_owned()));
        let payload_e = Expr::Lit(Literal::Str("second\n".to_string()));

        let name = Name::str("IO.FS.appendFile");
        let r = try_dispatch_io_fs_builtin(&name, &[&path_e, &payload_e])
            .expect("appendFile head should match");
        assert!(matches!(r, Ok(None)));

        let contents = std::fs::read_to_string(&path).unwrap();
        assert_eq!(contents, "first\nsecond\n");
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn try_dispatch_io_fs_readfile_missing_path_returns_extern_failed() {
        let path = unique_tempfile_path("does-not-exist.txt");
        let path_e = Expr::Lit(Literal::Str(path.to_string_lossy().into_owned()));
        let name = Name::str("IO.FS.readFile");
        let r = try_dispatch_io_fs_builtin(&name, &[&path_e])
            .expect("readFile head should match");
        match r {
            Err(DriverError::ExternFailed(e)) => {
                let msg = e.to_string();
                assert!(
                    msg.contains("IO.FS.readFile") && msg.contains("does-not-exist"),
                    "error message should reference op + path; got: {msg}"
                );
            }
            other => panic!("expected ExternFailed, got: {other:?}"),
        }
    }

    #[test]
    fn try_dispatch_io_fs_rejects_unknown_head() {
        let path_e = Expr::Lit(Literal::Str("anything".to_string()));
        let name = Name::str("IO.FS.notABuiltin");
        assert!(try_dispatch_io_fs_builtin(&name, &[&path_e]).is_none());
    }

    // ─── User-defined ctor encoding ─────────────────────

    /// Build an env populated with a multi-ctor enum-like
    /// inductive `Color` with three nullary ctors, plus a
    /// single-ctor record `Point` carrying a single `Nat`
    /// field. Mirrors the shape leo4 plugin emits for
    /// `@[leo4_export]`-discovered types.
    fn env_with_user_ctors() -> Environment {
        use oxilean_kernel::declaration::{
            ConstantInfo, ConstantVal, ConstructorVal, InductiveVal,
        };
        let mut env = Environment::new();
        let color_ty = Expr::Const(Name::str("Color"), Vec::new());
        let red = ConstantInfo::Constructor(ConstructorVal {
            common: ConstantVal {
                name: Name::str("Color.Red"),
                level_params: vec![],
                ty: color_ty.clone(),
            },
            induct: Name::str("Color"),
            cidx: 0,
            num_params: 0,
            num_fields: 0,
            is_unsafe: false,
        });
        let green = ConstantInfo::Constructor(ConstructorVal {
            common: ConstantVal {
                name: Name::str("Color.Green"),
                level_params: vec![],
                ty: color_ty.clone(),
            },
            induct: Name::str("Color"),
            cidx: 1,
            num_params: 0,
            num_fields: 0,
            is_unsafe: false,
        });
        let blue = ConstantInfo::Constructor(ConstructorVal {
            common: ConstantVal {
                name: Name::str("Color.Blue"),
                level_params: vec![],
                ty: color_ty,
            },
            induct: Name::str("Color"),
            cidx: 2,
            num_params: 0,
            num_fields: 0,
            is_unsafe: false,
        });
        let color_ind = ConstantInfo::Inductive(InductiveVal {
            common: ConstantVal {
                name: Name::str("Color"),
                level_params: vec![],
                ty: Expr::Sort(oxilean_kernel::Level::succ(
                    oxilean_kernel::Level::zero(),
                )),
            },
            num_params: 0,
            num_indices: 0,
            all: vec![Name::str("Color")],
            ctors: vec![
                Name::str("Color.Red"),
                Name::str("Color.Green"),
                Name::str("Color.Blue"),
            ],
            num_nested: 0,
            is_rec: false,
            is_unsafe: false,
            is_reflexive: false,
            is_prop: false,
        });
        env.add_constant(color_ind).unwrap();
        env.add_constant(red).unwrap();
        env.add_constant(green).unwrap();
        env.add_constant(blue).unwrap();

        // Single-ctor record `Point` with one `Nat` field.
        let point_ty = Expr::Const(Name::str("Point"), Vec::new());
        let point_mk = ConstantInfo::Constructor(ConstructorVal {
            common: ConstantVal {
                name: Name::str("Point.mk"),
                level_params: vec![],
                ty: point_ty.clone(),
            },
            induct: Name::str("Point"),
            cidx: 0,
            num_params: 0,
            num_fields: 1,
            is_unsafe: false,
        });
        let point_ind = ConstantInfo::Inductive(InductiveVal {
            common: ConstantVal {
                name: Name::str("Point"),
                level_params: vec![],
                ty: Expr::Sort(oxilean_kernel::Level::succ(
                    oxilean_kernel::Level::zero(),
                )),
            },
            num_params: 0,
            num_indices: 0,
            all: vec![Name::str("Point")],
            ctors: vec![Name::str("Point.mk")],
            num_nested: 0,
            is_rec: false,
            is_unsafe: false,
            is_reflexive: false,
            is_prop: false,
        });
        env.add_constant(point_ind).unwrap();
        env.add_constant(point_mk).unwrap();
        env
    }

    #[test]
    fn encode_user_defined_ctor_multi_writes_discriminant() {
        let env = env_with_user_ctors();
        // `Color.Red` is cidx=0 in a 3-ctor inductive →
        // 4-byte u32 LE = 0.
        let arg = Expr::Const(Name::str("Color.Red"), Vec::new());
        let out = encode_leaf_args_for_extern(&[&arg], &env).unwrap();
        assert_eq!(out, vec![0, 0, 0, 0]);

        let arg = Expr::Const(Name::str("Color.Green"), Vec::new());
        let out = encode_leaf_args_for_extern(&[&arg], &env).unwrap();
        assert_eq!(out, vec![1, 0, 0, 0]);

        let arg = Expr::Const(Name::str("Color.Blue"), Vec::new());
        let out = encode_leaf_args_for_extern(&[&arg], &env).unwrap();
        assert_eq!(out, vec![2, 0, 0, 0]);
    }

    #[test]
    fn encode_user_defined_ctor_single_record_writes_just_payload() {
        let env = env_with_user_ctors();
        // `Point.mk 42` — single-ctor record → no
        // discriminant prefix; payload is the Nat → 8
        // bytes u64 LE.
        let arg = Expr::App(
            Box::new(Expr::Const(Name::str("Point.mk"), Vec::new())),
            Box::new(Expr::Lit(Literal::Nat(42))),
        );
        let out = encode_leaf_args_for_extern(&[&arg], &env).unwrap();
        assert_eq!(out, vec![42, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn encode_user_defined_ctor_unknown_ctor_returns_none() {
        let env = env_with_user_ctors();
        let arg = Expr::Const(Name::str("Unknown.Ctor"), Vec::new());
        // No corresponding entry in env → falls through to
        // returning None from encode_leaf_args_for_extern.
        assert!(encode_leaf_args_for_extern(&[&arg], &env).is_none());
    }

    #[test]
    fn run_main_extern_const_with_encodable_args_passes_them_through() {
        use crate::extern_resolver::ExternResolver;
        use oxilean_kernel::ffi::{
            CallingConvention, ExternDecl, FfiSafety, FfiSignature, FfiType,
        };

        struct ArgTracker {
            last_args: std::sync::Mutex<Vec<u8>>,
        }
        impl ExternResolver for ArgTracker {
            fn resolve(
                &self,
                _decl_name: &Name,
                args: &[u8],
            ) -> Result<Vec<u8>, oxilean_kernel::ffi::ExternCallError> {
                *self.last_args.lock().unwrap() = args.to_vec();
                Ok(Vec::new())
            }
        }

        let mut env = empty_env();
        let main = Name::str("main");
        let extern_name = Name::str("nat_extern");

        env.add(Declaration::Axiom {
            name: extern_name.clone(),
            univ_params: Vec::new(),
            ty: Expr::App(
                Box::new(Expr::Const(Name::str("IO"), Vec::new())),
                Box::new(Expr::Const(Name::str("Unit"), Vec::new())),
            ),
        })
        .unwrap();

        // `main := nat_extern 42 Bool.true`
        let body = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(extern_name.clone(), Vec::new())),
                Box::new(Expr::Lit(Literal::Nat(42))),
            )),
            Box::new(Expr::Const(Name::str("Bool.true"), Vec::new())),
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
                "nat_extern".to_string(),
                FfiSafety::Safe,
                CallingConvention::Rust,
                FfiSignature::new(vec![FfiType::ByteArray], Box::new(FfiType::ByteArray)),
            ))
            .unwrap();

        let tracker = Arc::new(ArgTracker {
            last_args: std::sync::Mutex::new(Vec::new()),
        });
        let resolver: SharedExternResolver = tracker.clone();

        run_main(&env, &registry, resolver, &main)
            .expect("encodable-args extern call should dispatch");
        let got = tracker.last_args.lock().unwrap().clone();
        let mut expected = Vec::new();
        expected.extend_from_slice(&42u64.to_le_bytes());
        expected.push(0x01);
        assert_eq!(got, expected, "resolver should receive concatenated u64+bool bytes");
    }
}
