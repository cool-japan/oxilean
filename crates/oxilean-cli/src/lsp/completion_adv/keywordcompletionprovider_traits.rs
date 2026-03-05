//! # KeywordCompletionProvider - Trait Implementations
//!
//! This module contains trait implementations for `KeywordCompletionProvider`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `CompletionProvider`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::CompletionProvider;
use super::types::{
    AdvancedCompletionItem, CompletionList, KeywordCompletionProvider, LspCompletionContext,
};
use std::fmt;

impl Default for KeywordCompletionProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl CompletionProvider for KeywordCompletionProvider {
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
            .keywords
            .iter()
            .map(|(kw, doc)| {
                AdvancedCompletionItem::new(*kw)
                    .with_kind(14)
                    .with_documentation(*doc)
            })
            .collect();
        CompletionList::complete(items)
    }
}
