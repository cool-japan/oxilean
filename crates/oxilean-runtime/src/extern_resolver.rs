//! # `ExternResolver` — runtime-side dispatch hook for `@[extern]` declarations
//!
//! OxiLean's [`ExternRegistry`](oxilean_kernel::ffi::ExternRegistry)
//! stores compile-time metadata for every `@[extern]` declaration:
//! the `(lib_name, symbol_name)` pair, the parameter / return
//! `FfiType`s, the calling convention, the FFI-safety verdict.
//! Nothing in `ExternRegistry` can actually *execute* the foreign
//! function — at runtime, the evaluator's `Const` reduction stops
//! short of any dispatch and either unfolds a `Definition.val` or
//! treats `Axiom` / opaque entries as irreducible.
//!
//! `@[extern]` declarations lower to an `Axiom` in the current
//! kernel (no `val` is available because the implementation lives
//! in a separately-compiled cdylib / shared object). The
//! `ExternResolver` trait closes the gap: an evaluator instance
//! that has an `ExternResolver` installed will, when it encounters
//! an `@[extern]`-backed `Const`, hand off the encoded argument
//! bytes to the resolver and decode the resulting bytes back into
//! a value.
//!
//! ## Boundary
//!
//! - The kernel keeps owning `Declaration::Axiom` plus the
//!   metadata side ([`ExternRegistry`]). Kernel code stays free
//!   of any runtime / FFI execution concerns.
//! - The runtime owns *execution* — that's this module plus the
//!   matching [`CallbackRegistry`](oxilean_kernel::ffi::CallbackRegistry)
//!   that lives in `oxilean-kernel/src/ffi/callbackregistry_traits.rs`
//!   for kernel-runtime co-location reasons.
//! - The embedder (e.g. `leo4-oxilean`'s `OxiLeanInvoker`)
//!   implements `ExternResolver` and is what actually decides how
//!   to dispatch (in-process Rust closure, `dlsym`'d cdylib call,
//!   IPC out-of-process call — all behind the same trait).
//!
//! This split mirrors the existing `Definition.val` (kernel-side
//! AST) vs. `ReductionStrategy` (runtime-side dispatch) boundary
//! that OxiLean already uses.
//!
//! See `docs/ox8-3-callback-hook-design.md` in the leo4 repo for
//! the full rationale and the upstream-PR viability analysis.

use std::sync::Arc;

use oxilean_kernel::env::Declaration;
use oxilean_kernel::ffi::{ExternCallError, ExternRegistry};
use oxilean_kernel::{Environment, Name};

/// Runtime-side hook the evaluator calls when it encounters an
/// `@[extern]`-backed `Const` during reduction.
///
/// Implementors receive the *fully-qualified* declaration name
/// (the kernel-level [`Name`], not the mangled symbol — translating
/// to the `(lib, symbol)` pair is the implementor's responsibility,
/// usually via a lookup in [`ExternRegistry`]). The `args` slice
/// carries the canonical-ABI encoded argument bytes — the exact
/// concatenation produced by leo4's `LeanMarshal::canonical_encode`
/// for the forward-direction path.
///
/// The return is the canonical-ABI encoded return value as an
/// owned `Vec<u8>`, or an [`ExternCallError`] when the resolver
/// cannot dispatch (no registered callback, callback returned
/// `Err`, encoding mismatch, etc.).
///
/// `Send + Sync` so future parallel evaluators can dispatch
/// without holding a `&mut` to the resolver. The current evaluator
/// runs sequentially but the bound is cheap to require up-front.
///
/// One consequence worth stating: `Name` is `Rc<NameKind>` since
/// the kernel's structural-sharing work, so it is `!Send` and an
/// implementor **cannot store the `decl_name` it is handed**.
/// Keep a `String` (or an interned id of your own) instead. The
/// `&Name` in the signature is fine — only ownership is barred.
pub trait ExternResolver: Send + Sync {
    /// Resolve and execute the `@[extern]` declaration named
    /// `decl_name` against the encoded argument bytes `args`.
    fn resolve(&self, decl_name: &Name, args: &[u8]) -> Result<Vec<u8>, ExternCallError>;
}

/// Boxed `dyn ExternResolver` — convenience alias for the
/// `Option<Arc<dyn ExternResolver>>` slot evaluators carry. The
/// `Arc` lets the same resolver be shared across evaluator
/// instances (re-entrancy, parallel reduction lanes); the `Option`
/// makes the resolver-less default (and thus the v0.1.2 backward-
/// compatible path) the natural one.
pub type SharedExternResolver = Arc<dyn ExternResolver>;

/// Decision returned by [`dispatch_extern_const`] — telling the
/// caller whether the `Const` reduction step reduced to extern
/// bytes, was left alone (no resolver, or not an extern), or
/// failed.
#[derive(Debug)]
pub enum ExternDispatch {
    /// The declaration is `@[extern]`-backed *and* a resolver is
    /// installed *and* the resolver succeeded. Wraps the
    /// canonical-ABI encoded return bytes.
    Resolved(Vec<u8>),
    /// The `Const` does not name an `@[extern]` declaration (or
    /// the declaration is not in `ExternRegistry`). Caller should
    /// fall through to its normal reduction path (`Definition`
    /// unfold, `Axiom` opaque, etc.).
    NotExtern,
    /// The declaration *is* `@[extern]` but no resolver is
    /// installed on the evaluator. Caller treats the `Const` as
    /// opaque — semantically identical to an `Axiom`, which is
    /// the v0.1.2 behaviour. **No error**: this is the graceful
    /// degradation path for evaluator clients that don't need
    /// extern dispatch.
    NoResolverInstalled,
    /// Dispatch was attempted but the resolver returned `Err`.
    /// Carries the error verbatim for the caller to surface.
    Failed(ExternCallError),
}

/// Inspect a `Const(name, _)` reduction site and, if it names an
/// `@[extern]` declaration and a resolver is installed, dispatch
/// into the resolver. Otherwise return [`ExternDispatch::NotExtern`]
/// or [`ExternDispatch::NoResolverInstalled`] so the caller can
/// continue its normal reduction.
///
/// This is the single entry point new `Const`-reduction code paths
/// should call before any other unfolding heuristic. It encodes
/// the spec's evaluator-integration pseudo-code (see
/// `docs/ox8-3-callback-hook-design.md` §"Evaluator integration")
/// verbatim.
///
/// # Arguments
///
/// - `env`: the global environment, used to look up the
///   declaration backing `decl_name`. If the name is not in the
///   environment the result is `NotExtern` (the caller's normal
///   "unknown const" handling kicks in — we don't synthesise an
///   error here).
/// - `registry`: OxiLean's `ExternRegistry`. A name is treated as
///   `@[extern]` iff it has a metadata entry in this registry.
///   This is the canonical OxiLean test for "is this declaration
///   `@[extern]`-backed?" — see `oxilean-elab`'s `@[extern]`
///   attribute handler which populates the registry on every
///   `@[extern]` it processes.
/// - `resolver`: the optional embedder-supplied dispatch hook.
///   `None` produces `NoResolverInstalled` (graceful no-op).
/// - `args`: canonical-ABI encoded argument bytes the evaluator
///   has assembled at this `Const` reduction site. Forwarded
///   verbatim to the resolver.
pub fn dispatch_extern_const(
    env: &Environment,
    registry: &ExternRegistry,
    resolver: Option<&SharedExternResolver>,
    decl_name: &Name,
    args: &[u8],
) -> ExternDispatch {
    // Step 1 — does the env carry a declaration for this name at
    // all? If not the caller's normal "unknown const" path takes
    // over.
    if env.find(decl_name).is_none() {
        return ExternDispatch::NotExtern;
    }

    // Step 2 — is it an @[extern]-backed declaration? The
    // canonical test is "is there an ExternRegistry entry under
    // this name?" — `Declaration::Axiom` with no extern metadata
    // would not appear in the registry.
    if registry.lookup(decl_name).is_err() {
        return ExternDispatch::NotExtern;
    }

    // Step 3 — graceful no-op when no resolver is installed. This
    // is the v0.1.2 behaviour: an `@[extern]` `Const` stays
    // opaque, indistinguishable from a regular `Axiom`. No
    // error, no panic — embedders that don't need dispatch get
    // the existing semantics.
    let Some(resolver) = resolver else {
        return ExternDispatch::NoResolverInstalled;
    };

    // Step 4 — actually dispatch.
    match resolver.resolve(decl_name, args) {
        Ok(bytes) => ExternDispatch::Resolved(bytes),
        Err(e) => ExternDispatch::Failed(e),
    }
}

/// Convenience: same as [`dispatch_extern_const`] but specifically
/// for the "look-up-only-by-declaration" path where the caller
/// has already confirmed the declaration exists in the environment
/// (e.g. it has the `Declaration` in hand). Skips the
/// `env.find(...)` step.
pub fn dispatch_extern_decl(
    decl: &Declaration,
    registry: &ExternRegistry,
    resolver: Option<&SharedExternResolver>,
    args: &[u8],
) -> ExternDispatch {
    let decl_name = decl.name();
    if registry.lookup(decl_name).is_err() {
        return ExternDispatch::NotExtern;
    }
    let Some(resolver) = resolver else {
        return ExternDispatch::NoResolverInstalled;
    };
    match resolver.resolve(decl_name, args) {
        Ok(bytes) => ExternDispatch::Resolved(bytes),
        Err(e) => ExternDispatch::Failed(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxilean_kernel::env::Declaration;
    use oxilean_kernel::expr::Expr;
    use oxilean_kernel::ffi::{CallingConvention, ExternDecl, FfiSafety, FfiSignature, FfiType};
    use std::sync::Mutex;

    /// Build a `Declaration::Axiom` named `name` with a `ByteArray`-
    /// typed payload. Mirrors the shape `oxilean-elab` produces for
    /// `@[extern] axiom foo : T` declarations.
    fn axiom(name: &str) -> Declaration {
        Declaration::Axiom {
            name: Name::str(name),
            univ_params: vec![],
            ty: Expr::Const(Name::str("ByteArray"), vec![]),
        }
    }

    /// Build a minimal `ExternDecl` for `(lib, symbol) =
    /// ("leo4-rust-bridge", name)`.
    fn extern_decl(name: &str) -> ExternDecl {
        ExternDecl::new(
            Name::str(name),
            Expr::Const(Name::str("ByteArray"), vec![]),
            "leo4-rust-bridge".to_string(),
            name.to_string(),
            FfiSafety::Safe,
            CallingConvention::Rust,
            FfiSignature::new(vec![FfiType::ByteArray], Box::new(FfiType::ByteArray)),
        )
    }

    /// Mock resolver that records the calls it received and
    /// returns canned bytes. Keeps the recorded calls under a
    /// `Mutex` so the trait's `&self` (not `&mut self`) works.
    ///
    /// Records the decl name as a `String`, not a `Name`: since
    /// oxilean-kernel's structural-sharing work `Name` is
    /// `Rc<NameKind>` and therefore `!Send`, so a `Send + Sync`
    /// resolver cannot own one. See [`ExternResolver`]'s note.
    struct MockResolver {
        calls: Mutex<Vec<(String, Vec<u8>)>>,
        result: Result<Vec<u8>, ExternCallError>,
    }

    impl MockResolver {
        fn new(result: Result<Vec<u8>, ExternCallError>) -> Self {
            MockResolver {
                calls: Mutex::new(Vec::new()),
                result,
            }
        }
        fn call_count(&self) -> usize {
            self.calls.lock().unwrap().len()
        }
    }

    impl ExternResolver for MockResolver {
        fn resolve(&self, decl_name: &Name, args: &[u8]) -> Result<Vec<u8>, ExternCallError> {
            self.calls
                .lock()
                .unwrap()
                .push((decl_name.to_string(), args.to_vec()));
            self.result.clone()
        }
    }

    /// Acceptance criterion #1: `@[extern]` decl + installed
    /// resolver → resolver runs with the right args + the bytes
    /// flow back through `ExternDispatch::Resolved`.
    #[test]
    fn resolver_called_for_extern_decl() {
        let mut env = Environment::new();
        env.add(axiom("leo4_add_u64")).unwrap();

        let mut registry = ExternRegistry::new();
        registry.register(extern_decl("leo4_add_u64")).unwrap();

        // Hold a typed handle to the mock so we can observe its
        // recorded calls after dispatch. The `SharedExternResolver`
        // we pass to `dispatch_extern_const` is the same Arc
        // up-cast to `dyn ExternResolver`.
        let mock = Arc::new(MockResolver::new(Ok(vec![0xCA, 0xFE])));
        let resolver: SharedExternResolver = mock.clone();

        let result = dispatch_extern_const(
            &env,
            &registry,
            Some(&resolver),
            &Name::str("leo4_add_u64"),
            &[0x01, 0x02, 0x03, 0x04],
        );

        match result {
            ExternDispatch::Resolved(bytes) => {
                assert_eq!(bytes, vec![0xCA, 0xFE]);
            }
            other => panic!("expected Resolved, got {other:?}"),
        }

        // Mock observed the call with the right args.
        assert_eq!(mock.call_count(), 1);
        let calls = mock.calls.lock().unwrap();
        assert_eq!(calls[0].0, Name::str("leo4_add_u64").to_string());
        assert_eq!(calls[0].1, vec![0x01, 0x02, 0x03, 0x04]);
    }

    /// Acceptance criterion #2: `@[extern]` decl + *no* resolver
    /// installed → `NoResolverInstalled` (graceful — no panic, no
    /// error variant from the resolver, callers degrade to the
    /// existing `Axiom`-opaque behaviour).
    #[test]
    fn no_resolver_installed_means_extern_decl_stays_opaque() {
        let mut env = Environment::new();
        env.add(axiom("leo4_unhooked")).unwrap();

        let mut registry = ExternRegistry::new();
        registry.register(extern_decl("leo4_unhooked")).unwrap();

        let result = dispatch_extern_const(&env, &registry, None, &Name::str("leo4_unhooked"), &[]);

        match result {
            ExternDispatch::NoResolverInstalled => {}
            other => panic!("expected NoResolverInstalled, got {other:?}"),
        }
    }

    /// Acceptance criterion #3: resolver returns `Err(...)` →
    /// `ExternDispatch::Failed` carrying the same `ExternCallError`
    /// verbatim. Embedders + caller wrappers (e.g. `leo4-oxilean`)
    /// can then map it to their domain error.
    #[test]
    fn resolver_propagates_error() {
        let mut env = Environment::new();
        env.add(axiom("leo4_explodes")).unwrap();

        let mut registry = ExternRegistry::new();
        registry.register(extern_decl("leo4_explodes")).unwrap();

        let resolver: SharedExternResolver = Arc::new(MockResolver::new(Err(
            ExternCallError::CallbackFailed("simulated failure".to_string()),
        )));

        let result = dispatch_extern_const(
            &env,
            &registry,
            Some(&resolver),
            &Name::str("leo4_explodes"),
            &[],
        );

        match result {
            ExternDispatch::Failed(ExternCallError::CallbackFailed(msg)) => {
                assert_eq!(msg, "simulated failure");
            }
            other => panic!("expected Failed(CallbackFailed), got {other:?}"),
        }
    }

    /// A `Const` whose name is *not* in `ExternRegistry` (a normal
    /// `Definition` / `Axiom`) returns `NotExtern` regardless of
    /// whether a resolver is installed. The caller's normal
    /// reduction path takes over.
    #[test]
    fn non_extern_decl_returns_not_extern() {
        let mut env = Environment::new();
        env.add(axiom("regular_axiom")).unwrap();

        let registry = ExternRegistry::new(); // empty

        let resolver: SharedExternResolver = Arc::new(MockResolver::new(Ok(vec![])));

        let result = dispatch_extern_const(
            &env,
            &registry,
            Some(&resolver),
            &Name::str("regular_axiom"),
            &[],
        );

        match result {
            ExternDispatch::NotExtern => {}
            other => panic!("expected NotExtern, got {other:?}"),
        }
    }

    /// A `Const` referring to a name the environment has no
    /// declaration for returns `NotExtern` — the caller's
    /// "unknown const" handling (typically an error) takes over.
    /// We don't synthesise a dispatch error for unknown names.
    #[test]
    fn unknown_name_returns_not_extern() {
        let env = Environment::new();
        let registry = ExternRegistry::new();
        let resolver: SharedExternResolver = Arc::new(MockResolver::new(Ok(vec![])));

        let result = dispatch_extern_const(
            &env,
            &registry,
            Some(&resolver),
            &Name::str("does_not_exist"),
            &[],
        );

        match result {
            ExternDispatch::NotExtern => {}
            other => panic!("expected NotExtern, got {other:?}"),
        }
    }

    /// The decl-handle entry point `dispatch_extern_decl` skips
    /// the env lookup but otherwise behaves identically. Useful
    /// for evaluator code paths that already have a `Declaration`
    /// in hand.
    #[test]
    fn dispatch_by_decl_handle_works() {
        let decl = axiom("leo4_decl_handle");
        let mut registry = ExternRegistry::new();
        registry.register(extern_decl("leo4_decl_handle")).unwrap();

        let resolver: SharedExternResolver = Arc::new(MockResolver::new(Ok(vec![0x42])));

        let result = dispatch_extern_decl(&decl, &registry, Some(&resolver), &[]);

        match result {
            ExternDispatch::Resolved(bytes) => assert_eq!(bytes, vec![0x42]),
            other => panic!("expected Resolved, got {other:?}"),
        }
    }

    /// `ExternResolver` must be `Send + Sync` so `Arc<dyn
    /// ExternResolver>` can cross threads. Compile-time check.
    #[test]
    fn extern_resolver_trait_is_send_and_sync() {
        fn assert_send_sync<T: Send + Sync + ?Sized>() {}
        assert_send_sync::<dyn ExternResolver>();
        assert_send_sync::<SharedExternResolver>();
    }
}
