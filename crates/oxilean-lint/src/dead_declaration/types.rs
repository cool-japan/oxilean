//! Types for dead-declaration lint rules.

/// A reference to a declaration name at a specific location.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeclRef {
    /// The referenced name.
    pub name: String,
    /// (line, column) of the reference; 1-based.
    pub location: (u32, u32),
}

impl DeclRef {
    /// Construct a new declaration reference.
    pub fn new(name: impl Into<String>, location: (u32, u32)) -> Self {
        Self {
            name: name.into(),
            location,
        }
    }
}

/// Usage information for a single declaration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeclUsage {
    /// The declaration name.
    pub name: String,
    /// Where the declaration was defined; 1-based (line, column).
    pub defined_at: (u32, u32),
    /// All locations where this declaration is referenced.
    pub used_at: Vec<(u32, u32)>,
    /// Whether this declaration is publicly visible outside the module.
    pub is_exported: bool,
    /// Whether this declaration is an `axiom`.
    pub is_axiom: bool,
}

impl DeclUsage {
    /// Construct a new declaration usage record.
    pub fn new(
        name: impl Into<String>,
        defined_at: (u32, u32),
        is_exported: bool,
        is_axiom: bool,
    ) -> Self {
        Self {
            name: name.into(),
            defined_at,
            used_at: Vec::new(),
            is_exported,
            is_axiom,
        }
    }

    /// Return `true` when the declaration has at least one use site.
    pub fn is_used(&self) -> bool {
        !self.used_at.is_empty()
    }
}

/// The kind of dead-declaration problem detected.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeadDeclKind {
    /// Completely unused.
    Unused,
    /// Only referenced from within `#[test]` or `#[cfg(test)]` blocks.
    OnlyUsedInTests,
    /// Shadowed by a newer declaration with the same name.
    Shadowed {
        /// Name of the shadowing declaration (may equal the original).
        by: String,
    },
    /// An `axiom` that could be derived from existing declarations.
    RedundantAxiom,
    /// A private declaration that is never used.
    PrivateUnused,
}

impl std::fmt::Display for DeadDeclKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unused => write!(f, "unused"),
            Self::OnlyUsedInTests => write!(f, "only_used_in_tests"),
            Self::Shadowed { by } => write!(f, "shadowed_by({})", by),
            Self::RedundantAxiom => write!(f, "redundant_axiom"),
            Self::PrivateUnused => write!(f, "private_unused"),
        }
    }
}

/// A single dead-declaration finding.
#[derive(Clone, Debug)]
pub struct DeadDeclIssue {
    /// Declaration name.
    pub name: String,
    /// Why this declaration is considered dead.
    pub kind: DeadDeclKind,
    /// (line, column) of the declaration; 1-based.
    pub location: (u32, u32),
}

impl DeadDeclIssue {
    /// Construct a new dead-declaration issue.
    pub fn new(name: impl Into<String>, kind: DeadDeclKind, location: (u32, u32)) -> Self {
        Self {
            name: name.into(),
            kind,
            location,
        }
    }
}

impl std::fmt::Display for DeadDeclIssue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] `{}` at {}:{}",
            self.kind, self.name, self.location.0, self.location.1
        )
    }
}

/// Configuration for the dead-declaration lint pass.
#[derive(Clone, Debug)]
pub struct DeadDeclConfig {
    /// Warn about private declarations that are never used.
    pub warn_private: bool,
    /// Warn about declarations that are only referenced from test code.
    pub warn_tests_only: bool,
    /// Glob-style name patterns to ignore (e.g. `"_*"` or `"test_*"`).
    pub ignore_patterns: Vec<String>,
}

impl Default for DeadDeclConfig {
    fn default() -> Self {
        Self {
            warn_private: true,
            warn_tests_only: false,
            ignore_patterns: vec!["_".to_owned()],
        }
    }
}

impl DeadDeclConfig {
    /// Return `true` when `name` matches one of the ignore patterns.
    pub fn is_ignored(&self, name: &str) -> bool {
        for pat in &self.ignore_patterns {
            if pat.ends_with('*') {
                let prefix = &pat[..pat.len() - 1];
                if name.starts_with(prefix) {
                    return true;
                }
            } else if name == pat.as_str() {
                return true;
            } else if name.starts_with(pat.as_str()) && pat == "_" {
                return true;
            }
        }
        false
    }
}

/// Summary report for dead-declaration analysis.
#[derive(Clone, Debug, Default)]
pub struct DeadDeclReport {
    /// All dead-declaration issues found.
    pub issues: Vec<DeadDeclIssue>,
    /// Total number of declarations analysed.
    pub total_decls: usize,
    /// Number of dead declarations found.
    pub dead_count: usize,
}

impl DeadDeclReport {
    /// Return `true` when no dead declarations were found.
    pub fn is_clean(&self) -> bool {
        self.issues.is_empty()
    }

    /// Percentage of declarations that are dead, or 0.0 when total is 0.
    pub fn dead_ratio(&self) -> f64 {
        if self.total_decls == 0 {
            return 0.0;
        }
        self.dead_count as f64 / self.total_decls as f64
    }
}
