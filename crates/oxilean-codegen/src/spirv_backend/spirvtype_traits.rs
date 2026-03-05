//! # SpirVType - Trait Implementations
//!
//! This module contains trait implementations for `SpirVType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SpirVType;
use std::fmt;

impl fmt::Display for SpirVType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpirVType::Void => write!(f, "void"),
            SpirVType::Bool => write!(f, "bool"),
            SpirVType::Int { width, signed } => {
                let sign = if *signed { "i" } else { "u" };
                write!(f, "{}{}", sign, width)
            }
            SpirVType::Float { width } => write!(f, "f{}", width),
            SpirVType::Vector { element, count } => {
                write!(f, "vec{}<{}>", count, element)
            }
            SpirVType::Matrix {
                column_type,
                column_count,
            } => {
                write!(f, "mat{}x<{}>", column_count, column_type)
            }
            SpirVType::Array { element, length } => {
                write!(f, "[{}; {}]", element, length)
            }
            SpirVType::RuntimeArray(elem) => write!(f, "[{}]", elem),
            SpirVType::Struct(members) => {
                write!(f, "struct {{")?;
                for (i, m) in members.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", m)?;
                }
                write!(f, "}}")
            }
            SpirVType::Pointer {
                storage_class,
                pointee,
            } => {
                write!(f, "*{} {}", storage_class, pointee)
            }
            SpirVType::Function {
                return_type,
                param_types,
            } => {
                write!(f, "fn(")?;
                for (i, p) in param_types.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", p)?;
                }
                write!(f, ") -> {}", return_type)
            }
            SpirVType::Image { dim, .. } => write!(f, "image<{:?}>", dim),
            SpirVType::Sampler => write!(f, "sampler"),
            SpirVType::SampledImage(img) => write!(f, "sampled_image<{}>", img),
        }
    }
}
