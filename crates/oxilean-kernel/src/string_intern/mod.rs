//! Thread-safe string interning pool for `Name` construction.
//!
//! The kernel frequently creates `Name::Str` values from string literals.
//! Interning them saves allocation: each unique string is stored only once,
//! and callers receive a cheap `Copy` handle (`InternedStr`) that can be
//! resolved back to a `&'static str` at any time.
//!
//! # Design
//!
//! - Global singleton via `std::sync::OnceLock<Arc<Mutex<InternPool>>>`.
//! - `InternedStr` is a 32-bit index — fits in a register, implements `Copy`.
//! - `resolve()` returns `&'static str` by leaking pool-owned strings exactly
//!   once per unique value (the leaked allocation lives for the program
//!   lifetime, matching the static lifetime contract).
//!
//! # Example
//!
//! ```
//! use oxilean_kernel::string_intern::{intern, resolve};
//!
//! let h1 = intern("hello");
//! let h2 = intern("hello");
//! assert_eq!(h1, h2);
//! assert_eq!(resolve(h1), "hello");
//! ```

pub mod interner;
pub mod pool;

pub use interner::{intern, resolve, InternedStr, StringInterner};
pub use pool::InternPool;
