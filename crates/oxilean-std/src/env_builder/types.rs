//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name, ReducibilityHint};

/// A thin wrapper around `Environment` providing convenience methods for
/// registering axioms and definitions during standard library initialisation.
pub struct EnvBuilder {
    env: Environment,
    errors: Vec<String>,
}
impl EnvBuilder {
    /// Create a new builder from an existing environment.
    pub fn new(env: Environment) -> Self {
        Self {
            env,
            errors: Vec::new(),
        }
    }
    /// Create a builder around a fresh `Environment::new()`.
    pub fn fresh() -> Self {
        Self::new(Environment::new())
    }
    /// Add an axiom with the given name and type.
    pub fn add_axiom(&mut self, name: Name, ty: Expr) {
        let result = self.env.add(Declaration::Axiom {
            name,
            univ_params: vec![],
            ty,
        });
        if let Err(e) = result {
            self.errors.push(e.to_string());
        }
    }
    /// Add an axiom using a `&str` name.
    pub fn axiom(&mut self, name: &str, ty: Expr) -> &mut Self {
        self.add_axiom(Name::str(name), ty);
        self
    }
    /// Add a definition with the given name, type, and value.
    pub fn add_definition(&mut self, name: Name, ty: Expr, value: Expr) {
        let result = self.env.add(Declaration::Definition {
            name,
            univ_params: vec![],
            ty,
            val: value,
            hint: ReducibilityHint::Regular(0),
        });
        if let Err(e) = result {
            self.errors.push(e.to_string());
        }
    }
    /// Add a definition using a `&str` name (builder-style).
    pub fn def(&mut self, name: &str, ty: Expr, value: Expr) -> &mut Self {
        self.add_definition(Name::str(name), ty, value);
        self
    }
    /// Add a theorem (opaque definition) using a `&str` name.
    pub fn theorem(&mut self, name: &str, ty: Expr, proof: Expr) -> &mut Self {
        let result = self.env.add(Declaration::Theorem {
            name: Name::str(name),
            univ_params: vec![],
            ty,
            val: proof,
        });
        if let Err(e) = result {
            self.errors.push(e.to_string());
        }
        self
    }
    /// Add a `Sorry` opaque axiom — useful for placeholder proofs.
    pub fn sorry(&mut self, name: &str, ty: Expr) -> &mut Self {
        self.axiom(name, ty)
    }
    /// Finish building and return the environment, or an error string.
    pub fn finish(self) -> Result<Environment, String> {
        if self.errors.is_empty() {
            Ok(self.env)
        } else {
            Err(self.errors.join("; "))
        }
    }
    /// Finish building, panicking on any error.  Useful in tests.
    pub fn finish_or_panic(self) -> Environment {
        match self.finish() {
            Ok(env) => env,
            Err(e) => panic!("EnvBuilder errors: {}", e),
        }
    }
    /// Get a reference to the underlying environment.
    pub fn env(&self) -> &Environment {
        &self.env
    }
    /// `true` if no errors have been accumulated.
    pub fn is_ok(&self) -> bool {
        self.errors.is_empty()
    }
    /// Return the accumulated error messages.
    pub fn errors(&self) -> &[String] {
        &self.errors
    }
    /// Check whether `name` has been registered.
    pub fn contains(&self, name: &str) -> bool {
        self.env.get(&Name::str(name)).is_some()
    }
}
