//! # PatternCompiler - pattern_to_string_group Methods
//!
//! This module contains method implementations for `PatternCompiler`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::Pattern;

use super::patterncompiler_type::PatternCompiler;

impl PatternCompiler {
    /// Pretty-print a pattern to a string.
    #[allow(dead_code)]
    pub fn pattern_to_string(&self, pattern: &Pattern) -> String {
        match pattern {
            Pattern::Wild => "_".to_string(),
            Pattern::Var(name) => name.clone(),
            Pattern::Ctor(name, args) => {
                if args.is_empty() {
                    name.clone()
                } else {
                    let args_str: Vec<String> = args
                        .iter()
                        .map(|a| {
                            let s = self.pattern_to_string(&a.value);
                            if matches!(
                                & a.value, Pattern::Ctor(_, ref sub) if ! sub.is_empty()
                            ) {
                                format!("({})", s)
                            } else {
                                s
                            }
                        })
                        .collect();
                    format!("{} {}", name, args_str.join(" "))
                }
            }
            Pattern::Lit(lit) => format!("{}", lit),
            Pattern::Or(left, right) => {
                let ls = self.pattern_to_string(&left.value);
                let rs = self.pattern_to_string(&right.value);
                format!("{} | {}", ls, rs)
            }
        }
    }
    /// Convert patterns to a canonical form for comparison.
    pub fn canonicalize(&self, pattern: &Pattern) -> String {
        self.pattern_to_string(pattern)
    }
}
