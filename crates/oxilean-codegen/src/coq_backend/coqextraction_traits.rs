//! # CoqExtraction - Trait Implementations
//!
//! This module contains trait implementations for `CoqExtraction`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CoqExtraction;
use std::fmt;

impl std::fmt::Display for CoqExtraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoqExtraction::Language(l) => write!(f, "Extraction Language {}.", l),
            CoqExtraction::Constant(c, t) => {
                write!(f, "Extract Constant {} => \"{}\".", c, t)
            }
            CoqExtraction::Inductive(c, t) => {
                write!(f, "Extract Inductive {} => \"{}\" [\"\"].", c, t)
            }
            CoqExtraction::Inline(ns) => write!(f, "Extraction Inline {}.", ns.join(" ")),
            CoqExtraction::NoInline(ns) => {
                write!(f, "Extraction NoInline {}.", ns.join(" "))
            }
            CoqExtraction::RecursiveExtraction(n) => {
                write!(f, "Recursive Extraction {}.", n)
            }
            CoqExtraction::Extraction(n, file) => {
                write!(f, "Extraction \"{}\" {}.", file, n)
            }
            CoqExtraction::ExtractionLibrary(n, file) => {
                write!(f, "Extraction Library {} \"{}\".", n, file)
            }
        }
    }
}
