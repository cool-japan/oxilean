//! # SyntaxItem - Trait Implementations
//!
//! This module contains trait implementations for `SyntaxItem`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::fmt;

use super::types::SyntaxItem;

impl fmt::Display for SyntaxItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SyntaxItem::Token(tk) => write!(f, "{}", tk),
            SyntaxItem::Category(cat) => write!(f, "{}", cat),
            SyntaxItem::Optional(item) => write!(f, "({})?", item),
            SyntaxItem::Many(item) => write!(f, "({})*", item),
            SyntaxItem::Group(items) => {
                write!(f, "(")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, ")")
            }
        }
    }
}
