//! Types for user-defined notation elaboration.

/// Associativity of an infix operator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Assoc {
    /// Left-associative: `a + b + c` = `(a + b) + c`.
    Left,
    /// Right-associative: `a → b → c` = `a → (b → c)`.
    Right,
    /// Non-associative: `a = b = c` is a parse error.
    None,
}

impl std::fmt::Display for Assoc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Assoc::Left => write!(f, "left"),
            Assoc::Right => write!(f, "right"),
            Assoc::None => write!(f, "none"),
        }
    }
}

/// A part of a mixfix notation pattern.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MixfixPart {
    /// A literal keyword or punctuation token.
    Literal(String),
    /// A placeholder for a sub-expression, with its binding precedence.
    Placeholder(u32),
}

impl std::fmt::Display for MixfixPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MixfixPart::Literal(s) => write!(f, "{}", s),
            MixfixPart::Placeholder(p) => write!(f, "_{}", p),
        }
    }
}

/// How a notation is positioned relative to its operands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotationKind {
    /// Prefix operator: appears *before* its operand.
    Prefix {
        /// Binding power of the operand.
        prec: u32,
    },
    /// Postfix operator: appears *after* its operand.
    Postfix {
        /// Binding power of the operand.
        prec: u32,
    },
    /// Infix operator: appears *between* two operands.
    Infix {
        /// Binding power.
        prec: u32,
        /// Associativity.
        assoc: Assoc,
    },
    /// General mixfix notation with an explicit pattern.
    Mixfix {
        /// The ordered list of literals and placeholders.
        parts: Vec<MixfixPart>,
    },
}

impl std::fmt::Display for NotationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NotationKind::Prefix { prec } => write!(f, "prefix({})", prec),
            NotationKind::Postfix { prec } => write!(f, "postfix({})", prec),
            NotationKind::Infix { prec, assoc } => write!(f, "infix({}, {})", prec, assoc),
            NotationKind::Mixfix { parts } => {
                let s: Vec<String> = parts.iter().map(|p| p.to_string()).collect();
                write!(f, "mixfix({})", s.join(" "))
            }
        }
    }
}

/// A registered notation definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotationDef {
    /// The operator symbol (e.g. `∧`, `+`, `if_then_else`).
    pub symbol: String,
    /// How the notation is positioned.
    pub kind: NotationKind,
    /// The translation template (e.g. `And $0 $1`).
    pub translation: String,
    /// Priority used for conflict resolution; higher priority wins.
    pub priority: i32,
}

impl std::fmt::Display for NotationDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "notation {} {} => {}  [priority={}]",
            self.kind, self.symbol, self.translation, self.priority
        )
    }
}

/// Database of all registered notations, sorted by priority (descending).
#[derive(Debug, Clone, Default)]
pub struct NotationDB {
    /// Registered notations.
    pub notations: Vec<NotationDef>,
}

/// A conflict detected when trying to add a notation that clashes with an existing one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotationConflict {
    /// The symbol that caused the conflict.
    pub symbol: String,
    /// The existing notation definition.
    pub existing: NotationDef,
    /// The new definition that conflicts.
    pub new_: NotationDef,
}

impl std::fmt::Display for NotationConflict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "notation conflict for '{}': existing={} vs new={}",
            self.symbol, self.existing, self.new_
        )
    }
}

/// The result of elaborating notations in a source string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElabNotationResult {
    /// The original (un-elaborated) source.
    pub original: String,
    /// The elaborated source (with notations replaced by their translations).
    pub elaborated: String,
    /// Names of notations that were applied, in order.
    pub applied_notations: Vec<String>,
}
