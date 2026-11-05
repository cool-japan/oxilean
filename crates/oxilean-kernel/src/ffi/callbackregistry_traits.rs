//! # CallbackRegistry — runtime sibling of `ExternRegistry`
//!
//! `ExternRegistry` (see `types.rs`, 1469–1539) stores compile-time
//! metadata for `@[extern]` declarations: the `(lib_name,
//! symbol_name)` pair, the parameter / return canonical-ABI byte
//! layouts, etc. Nothing in `ExternRegistry` can actually *execute*
//! the foreign function.
//!
//! `CallbackRegistry` closes that gap. It maps the same
//! `(lib_name, symbol_name)` key to a `Box<dyn Fn(&[u8]) ->
//! Result<Vec<u8>, ExternCallError>>` closure, supplied by the
//! embedder (e.g. `leo4-oxilean`'s `OxiLeanInvoker`). The runtime
//! evaluator's `Const` reduction will, in a follow-up commit
//! (OX8.3b), look up the callback by `(lib, symbol)` and dispatch
//! into it whenever it encounters an `@[extern]` declaration.
//!
//! The kernel-side surface area is intentionally tiny: a type
//! alias, an error enum, a small struct with `new` / `register` /
//! `invoke`. No reduction logic — that lives in
//! `oxilean-runtime`. No metadata — that stays in
//! `ExternRegistry`. This crate-level split mirrors the existing
//! `Definition.val` (kernel-side AST) vs. `ReductionStrategy`
//! (runtime-side dispatch) boundary.
//!
//! See `docs/ox8-3-callback-hook-design.md` in the leo4 repo for
//! the full rationale, evaluator integration sketch, and
//! upstream-PR viability analysis.

use std::collections::HashMap;
use std::fmt;

/// A boxed closure invoked by the OxiLean evaluator when it
/// encounters an `@[extern]` declaration during `Const`
/// reduction.
///
/// The closure receives the canonical-ABI encoded argument bytes
/// (concatenated per the leo4 ABI rules) and returns either the
/// canonical-ABI encoded return value or an [`ExternCallError`].
///
/// `Send + Sync` so future parallel evaluators can dispatch
/// without holding a `&mut` to the registry.
pub type ExternCallback =
    Box<dyn Fn(&[u8]) -> Result<Vec<u8>, ExternCallError> + Send + Sync>;

/// Error returned by [`CallbackRegistry::invoke`] and surfaced
/// from the evaluator when an `@[extern]` dispatch fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExternCallError {
    /// No callback was registered for the given `(lib, symbol)`
    /// key. Either the embedder forgot to call
    /// `CallbackRegistry::register`, or the metadata in
    /// `ExternRegistry` references a symbol the embedder doesn't
    /// know about.
    NotRegistered {
        /// The `lib_name` looked up.
        lib: String,
        /// The `symbol_name` looked up.
        symbol: String,
    },
    /// The registered callback was invoked, but returned an
    /// `Err`. The wrapped string is the callback-provided error
    /// message (free-form — typically a `Display`'d cdylib /
    /// libloading error).
    CallbackFailed(String),
}

impl fmt::Display for ExternCallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExternCallError::NotRegistered { lib, symbol } => write!(
                f,
                "no callback registered for extern symbol `{lib}::{symbol}`"
            ),
            ExternCallError::CallbackFailed(msg) => {
                write!(f, "extern callback failed: {msg}")
            }
        }
    }
}

impl std::error::Error for ExternCallError {}

/// Runtime sibling of [`ExternRegistry`](super::types::ExternRegistry).
///
/// Stores callbacks keyed by `(lib_name, symbol_name)`. The key
/// shape matches `ExternRegistry`'s internal `decls` map 1:1, so a
/// metadata entry + a callback entry for the same extern always
/// share a key.
pub struct CallbackRegistry {
    callbacks: HashMap<(String, String), ExternCallback>,
}

impl CallbackRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        CallbackRegistry {
            callbacks: HashMap::new(),
        }
    }

    /// Register a callback for the given `(lib, symbol)` key.
    ///
    /// If a callback was already registered for this key, the
    /// previous one is **replaced**. This is intentional —
    /// embedders may want to swap a stub for the real callback
    /// once a cdylib has been `dlopen`'d, and forcing them to
    /// build a fresh registry would be hostile.
    pub fn register(
        &mut self,
        lib: impl Into<String>,
        symbol: impl Into<String>,
        cb: ExternCallback,
    ) {
        self.callbacks.insert((lib.into(), symbol.into()), cb);
    }

    /// Invoke the callback for `(lib, symbol)` with the given
    /// canonical-ABI argument bytes.
    ///
    /// Returns:
    /// - `Ok(bytes)` — callback ran, here are its canonical-ABI
    ///   encoded return bytes.
    /// - `Err(ExternCallError::NotRegistered { .. })` — no
    ///   callback for this key.
    /// - `Err(ExternCallError::CallbackFailed(msg))` — callback
    ///   ran but returned `Err`.
    pub fn invoke(
        &self,
        lib: &str,
        symbol: &str,
        args: &[u8],
    ) -> Result<Vec<u8>, ExternCallError> {
        // HashMap lookup with borrowed-tuple-key is awkward
        // because `&(String, String)` ≠ `(&str, &str)`. Build a
        // small owned key here — extern dispatch is not a hot
        // path (it crosses a Rust↔Lean boundary every call), so
        // the allocation is negligible compared to the canonical
        // ABI encode/decode that brackets it.
        let key = (lib.to_string(), symbol.to_string());
        match self.callbacks.get(&key) {
            Some(cb) => cb(args),
            None => Err(ExternCallError::NotRegistered {
                lib: lib.to_string(),
                symbol: symbol.to_string(),
            }),
        }
    }

    /// Number of callbacks currently registered.
    pub fn len(&self) -> usize {
        self.callbacks.len()
    }

    /// `true` iff no callbacks are registered.
    pub fn is_empty(&self) -> bool {
        self.callbacks.is_empty()
    }
}

impl Default for CallbackRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Sanity: register a closure, invoke it, get the expected
    /// bytes back. Closure observes the args it was called with.
    #[test]
    fn register_and_invoke_round_trip() {
        let mut reg = CallbackRegistry::new();
        reg.register(
            "leo4-rust-bridge",
            "leo4_add_u64",
            Box::new(|args: &[u8]| {
                // Echo the args back with a known prefix so the
                // assertion can prove (a) the closure ran and
                // (b) it saw the right args.
                let mut out = vec![0xAB];
                out.extend_from_slice(args);
                Ok(out)
            }),
        );

        let result = reg
            .invoke("leo4-rust-bridge", "leo4_add_u64", &[0x01, 0x02, 0x03])
            .expect("registered callback should succeed");
        assert_eq!(result, vec![0xAB, 0x01, 0x02, 0x03]);
        assert_eq!(reg.len(), 1);
    }

    /// Looking up an unregistered symbol returns `NotRegistered`
    /// carrying the offending `(lib, symbol)` pair — not a panic,
    /// not a generic error.
    #[test]
    fn invoke_missing_symbol_returns_not_registered() {
        let reg = CallbackRegistry::new();
        let err = reg
            .invoke("leo4-rust-bridge", "nonexistent", &[])
            .expect_err("missing symbol must error");
        match err {
            ExternCallError::NotRegistered { lib, symbol } => {
                assert_eq!(lib, "leo4-rust-bridge");
                assert_eq!(symbol, "nonexistent");
            }
            other => panic!("expected NotRegistered, got {other:?}"),
        }
        assert!(reg.is_empty());
    }

    /// A callback that returns `Err` propagates the message
    /// verbatim through `ExternCallError::CallbackFailed`.
    #[test]
    fn invoke_callback_can_propagate_error() {
        let mut reg = CallbackRegistry::new();
        reg.register(
            "leo4-rust-bridge",
            "leo4_failing",
            Box::new(|_args: &[u8]| {
                Err(ExternCallError::CallbackFailed(
                    "dlsym returned null".to_string(),
                ))
            }),
        );

        let err = reg
            .invoke("leo4-rust-bridge", "leo4_failing", &[])
            .expect_err("callback returned Err");
        match err {
            ExternCallError::CallbackFailed(msg) => {
                assert_eq!(msg, "dlsym returned null");
            }
            other => panic!("expected CallbackFailed, got {other:?}"),
        }
    }

    /// Re-registering the same `(lib, symbol)` key replaces the
    /// previous callback. The new closure must be the one that
    /// runs.
    #[test]
    fn register_overwrite_replaces_previous() {
        let mut reg = CallbackRegistry::new();
        reg.register(
            "leo4-rust-bridge",
            "leo4_swap_me",
            Box::new(|_args: &[u8]| Ok(vec![0x01])),
        );
        reg.register(
            "leo4-rust-bridge",
            "leo4_swap_me",
            Box::new(|_args: &[u8]| Ok(vec![0x02])),
        );

        let result = reg
            .invoke("leo4-rust-bridge", "leo4_swap_me", &[])
            .expect("second callback registered");
        assert_eq!(result, vec![0x02]);
        // Still exactly one entry — overwrite, not append.
        assert_eq!(reg.len(), 1);
    }

    /// Compile-time check: `ExternCallback`'s `Send + Sync`
    /// bounds hold and a `CallbackRegistry` can therefore be
    /// shared across threads behind `Arc`. Not a runtime
    /// assertion — the test passing means the bounds compile.
    #[test]
    fn callback_registry_is_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<CallbackRegistry>();
        assert_send_sync::<ExternCallback>();
        assert_send_sync::<ExternCallError>();
    }

    /// `Display` impl produces the expected human-readable form
    /// for both error variants. Lets downstream code use
    /// `?`/`format!` without redoing this string assembly.
    #[test]
    fn extern_call_error_display() {
        let not_reg = ExternCallError::NotRegistered {
            lib: "leo4-rust-bridge".to_string(),
            symbol: "missing".to_string(),
        };
        assert_eq!(
            not_reg.to_string(),
            "no callback registered for extern symbol `leo4-rust-bridge::missing`"
        );

        let failed = ExternCallError::CallbackFailed("boom".to_string());
        assert_eq!(failed.to_string(), "extern callback failed: boom");
    }
}
