//! # SnippetCompletionProvider - Trait Implementations
//!
//! This module contains trait implementations for `SnippetCompletionProvider`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `CompletionProvider`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::CompletionProvider;
use super::types::{
    AdvancedCompletionItem, CompletionList, LspCompletionContext, SnippetCompletionProvider,
};
use std::fmt;

impl Default for SnippetCompletionProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl CompletionProvider for SnippetCompletionProvider {
    fn triggers(&self) -> &[char] {
        &[]
    }
    fn completions(
        &self,
        _uri: &str,
        _line: u32,
        _character: u32,
        _context: &LspCompletionContext,
    ) -> CompletionList {
        let items: Vec<AdvancedCompletionItem> = self
            .snippets
            .iter()
            .map(|s| {
                AdvancedCompletionItem::new(s.prefix.clone())
                    .with_kind(15)
                    .with_documentation(s.description.clone())
                    .with_snippet(s.body.clone())
            })
            .collect();
        CompletionList::complete(items)
    }
}
