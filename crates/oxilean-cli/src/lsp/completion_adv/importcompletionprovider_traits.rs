//! # ImportCompletionProvider - Trait Implementations
//!
//! This module contains trait implementations for `ImportCompletionProvider`.
//!
//! ## Implemented Traits
//!
//! - `CompletionProvider`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::CompletionProvider;
use super::types::{
    AdvancedCompletionItem, CompletionList, ImportCompletionProvider, LspCompletionContext,
};
use std::fmt;

impl CompletionProvider for ImportCompletionProvider {
    fn triggers(&self) -> &[char] {
        &[' ', '.']
    }
    fn completions(
        &self,
        _uri: &str,
        _line: u32,
        _character: u32,
        _context: &LspCompletionContext,
    ) -> CompletionList {
        let items: Vec<AdvancedCompletionItem> = self
            .known_modules
            .iter()
            .map(|m| {
                AdvancedCompletionItem::new(m.clone())
                    .with_kind(9)
                    .with_documentation(format!("Module: {}", m))
            })
            .collect();
        CompletionList::complete(items)
    }
}
