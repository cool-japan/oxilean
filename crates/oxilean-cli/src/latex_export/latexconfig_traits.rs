//! # LatexConfig - Trait Implementations
//!
//! This module contains trait implementations for `LatexConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;
use std::fmt;

use super::types::LatexConfig;

impl Default for LatexConfig {
    fn default() -> Self {
        Self {
            document_class: "article".to_string(),
            packages: vec![
                "amsthm".to_string(),
                "amsmath".to_string(),
                "amssymb".to_string(),
                "mathtools".to_string(),
            ],
            use_ams: true,
            use_hyperref: true,
            include_proofs: true,
            number_theorems: true,
            custom_preamble: String::new(),
            macros: HashMap::new(),
            title: None,
            author: None,
            full_document: true,
            font_size: "11pt".to_string(),
            encoding: "utf8".to_string(),
            paper_size: "a4paper".to_string(),
        }
    }
}
