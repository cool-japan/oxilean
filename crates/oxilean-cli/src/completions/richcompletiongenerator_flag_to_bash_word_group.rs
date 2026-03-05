//! # RichCompletionGenerator - flag_to_bash_word_group Methods
//!
//! This module contains method implementations for `RichCompletionGenerator`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CompletionSpec;

use super::functions::*;
use super::richcompletiongenerator_type::RichCompletionGenerator;

impl<'a> RichCompletionGenerator<'a> {
    pub fn flag_to_bash_word(flag: &CompletionSpec) -> String {
        let mut s = flag.long.clone();
        if let Some(ref short) = flag.short {
            s.push(' ');
            s.push_str(short);
        }
        s
    }
}
