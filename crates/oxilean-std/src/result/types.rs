//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

/// A lazily-evaluated Result cell (thunk wrapper).
#[allow(dead_code)]
pub struct LazyResultCell<T, E> {
    thunk: Box<dyn Fn() -> std::result::Result<T, E>>,
    evaluated: bool,
    label: String,
}
/// A simple error accumulator for collecting multiple Result errors.
///
/// Useful when you want to collect all errors before reporting them.
#[derive(Clone, Debug, Default)]
pub struct ErrorAccumulator {
    errors: Vec<String>,
}
impl ErrorAccumulator {
    /// Create a new empty accumulator.
    pub fn new() -> Self {
        Self::default()
    }
    /// Try a Result and accumulate any error.
    ///
    /// Returns the value if Ok, or records the error and returns None.
    pub fn try_add<T>(&mut self, result: std::result::Result<T, String>) -> Option<T> {
        match result {
            Ok(v) => Some(v),
            Err(e) => {
                self.errors.push(e);
                None
            }
        }
    }
    /// Check if any errors have been accumulated.
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    /// Return the accumulated errors.
    pub fn errors(&self) -> &[String] {
        &self.errors
    }
    /// Convert accumulated errors into a single Result.
    ///
    /// Returns `Ok(())` if there are no errors, or `Err(combined_message)` otherwise.
    pub fn into_result(self) -> std::result::Result<(), String> {
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.join("; "))
        }
    }
    /// Return the number of accumulated errors.
    pub fn len(&self) -> usize {
        self.errors.len()
    }
    /// Check if there are no errors.
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }
    /// Clear all accumulated errors.
    pub fn clear(&mut self) {
        self.errors.clear();
    }
}
/// A registry of Result-related definitions in an environment.
#[derive(Clone, Debug, Default)]
pub struct ResultRegistry {
    /// Names of all Result definitions known to be present.
    registered: Vec<String>,
}
impl ResultRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self::default()
    }
    /// Register a new definition name.
    pub fn register(&mut self, name: impl Into<String>) {
        let n = name.into();
        if !self.registered.contains(&n) {
            self.registered.push(n);
        }
    }
    /// Check if a definition is registered.
    pub fn contains(&self, name: &str) -> bool {
        self.registered.iter().any(|n| n == name)
    }
    /// Build from an environment by scanning for `Result.*` declarations.
    pub fn from_env(env: &Environment) -> Self {
        let names = [
            "Result",
            "Result.ok",
            "Result.err",
            "Result.isOk",
            "Result.isErr",
            "Result.map",
            "Result.andThen",
            "Result.mapErr",
            "Result.getOrElse",
            "Result.ok_isOk",
            "Result.err_isErr",
        ];
        let mut reg = Self::new();
        for name in &names {
            if env.get(&Name::str(*name)).is_some() {
                reg.register(*name);
            }
        }
        reg
    }
    /// Return all registered names.
    pub fn all_names(&self) -> &[String] {
        &self.registered
    }
    /// Number of registered definitions.
    pub fn len(&self) -> usize {
        self.registered.len()
    }
    /// Whether the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.registered.is_empty()
    }
}
/// A chain of Result-producing computations (monadic pipeline).
#[allow(dead_code)]
pub struct ResultChain<T, E> {
    steps: Vec<Box<dyn Fn(T) -> std::result::Result<T, E>>>,
    label: String,
}
/// Collects multiple Results, accumulating all errors (Validation-style).
#[allow(dead_code)]
pub struct ValidationCollector<T, E> {
    successes: Vec<T>,
    failures: Vec<E>,
    capacity: usize,
}
impl<T, E> ValidationCollector<T, E> {
    /// Create with a capacity hint.
    #[allow(dead_code)]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            successes: Vec::with_capacity(capacity),
            failures: Vec::new(),
            capacity,
        }
    }
    /// Push a Result into the collector.
    #[allow(dead_code)]
    pub fn push(&mut self, result: std::result::Result<T, E>) {
        match result {
            Ok(v) => self.successes.push(v),
            Err(e) => self.failures.push(e),
        }
    }
    /// True if there were no failures.
    #[allow(dead_code)]
    pub fn is_valid(&self) -> bool {
        self.failures.is_empty()
    }
    /// Consume into a Result of all successes, or all failures.
    #[allow(dead_code)]
    pub fn finish(self) -> std::result::Result<Vec<T>, Vec<E>> {
        if self.failures.is_empty() {
            Ok(self.successes)
        } else {
            Err(self.failures)
        }
    }
}
/// Registry tracking which Result extended axioms have been registered.
#[allow(dead_code)]
pub struct ResultAxiomRegistry {
    names: Vec<String>,
    version: u32,
    checksum: u64,
}
impl ResultAxiomRegistry {
    /// Create a new empty registry.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            names: Vec::new(),
            version: 1,
            checksum: 0,
        }
    }
    /// Register a name and update the checksum.
    #[allow(dead_code)]
    pub fn register(&mut self, name: impl Into<String>) {
        let n = name.into();
        self.checksum = self.checksum.wrapping_add(n.len() as u64);
        self.names.push(n);
    }
    /// Count registered axioms.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.names.len()
    }
    /// Check if empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }
}
/// Bridge between Result<T,E> and Either<E,T> representations.
#[allow(dead_code)]
pub struct ResultEitherBridge {
    pub convention: &'static str,
    pub flip_convention: bool,
    pub tag: u8,
}
impl ResultEitherBridge {
    /// Create a standard Right-is-Ok bridge.
    #[allow(dead_code)]
    pub fn standard() -> Self {
        Self {
            convention: "Right=Ok, Left=Err",
            flip_convention: false,
            tag: 0,
        }
    }
    /// Create a flipped Left-is-Ok bridge.
    #[allow(dead_code)]
    pub fn flipped() -> Self {
        Self {
            convention: "Left=Ok, Right=Err",
            flip_convention: true,
            tag: 1,
        }
    }
    /// Describe the current convention.
    #[allow(dead_code)]
    pub fn describe(&self) -> &'static str {
        self.convention
    }
}
