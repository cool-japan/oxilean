//! # NotationRegistry - Trait Implementations
//!
//! This module contains trait implementations for `NotationRegistry`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Expr, Level, Name};
use std::fmt;

use super::types::{Notation, NotationExpansion, NotationKind, NotationPart, NotationRegistry};

impl Default for NotationRegistry {
    fn default() -> Self {
        let mut registry = Self::new();
        registry.register(Notation {
            name: Name::str("list_nil"),
            kind: NotationKind::Notation,
            pattern: "[]".to_string(),
            parts: vec![NotationPart::Literal("[]".to_string())],
            expansion: NotationExpansion::Simple(Expr::Const(Name::str("List.nil"), vec![])),
            priority: 0,
            scope: None,
            is_builtin: false,
        });
        registry.register(Notation {
            name: Name::str("list_cons"),
            kind: NotationKind::Infixr { precedence: 67 },
            pattern: "::".to_string(),
            parts: vec![
                NotationPart::Placeholder("h".to_string(), 67),
                NotationPart::Literal("::".to_string()),
                NotationPart::Placeholder("t".to_string(), 67),
            ],
            expansion: NotationExpansion::Simple(Expr::Const(Name::str("List.cons"), vec![])),
            priority: 67,
            scope: None,
            is_builtin: false,
        });
        registry
    }
}
