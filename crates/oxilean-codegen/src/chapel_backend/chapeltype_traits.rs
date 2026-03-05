//! # ChapelType - Trait Implementations
//!
//! This module contains trait implementations for `ChapelType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ChapelType;
use std::fmt;

impl fmt::Display for ChapelType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChapelType::Int(None) => write!(f, "int"),
            ChapelType::Int(Some(w)) => write!(f, "int({w})"),
            ChapelType::UInt(None) => write!(f, "uint"),
            ChapelType::UInt(Some(w)) => write!(f, "uint({w})"),
            ChapelType::Real(None) => write!(f, "real"),
            ChapelType::Real(Some(w)) => write!(f, "real({w})"),
            ChapelType::Imag(None) => write!(f, "imag"),
            ChapelType::Imag(Some(w)) => write!(f, "imag({w})"),
            ChapelType::Complex(None) => write!(f, "complex"),
            ChapelType::Complex(Some(w)) => write!(f, "complex({w})"),
            ChapelType::Bool => write!(f, "bool"),
            ChapelType::String => write!(f, "string"),
            ChapelType::Bytes => write!(f, "bytes"),
            ChapelType::Range(t) => write!(f, "range({t})"),
            ChapelType::Domain(rank, t) => write!(f, "domain({rank}, {t})"),
            ChapelType::Array(dom, elt) => write!(f, "[{dom}] {elt}"),
            ChapelType::Record(name) => write!(f, "{name}"),
            ChapelType::Class(name) => write!(f, "{name}"),
            ChapelType::Union(name) => write!(f, "{name}"),
            ChapelType::EnumType(name) => write!(f, "{name}"),
            ChapelType::ProcType(args, ret) => {
                write!(f, "proc(")?;
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{a}")?;
                }
                write!(f, "): {ret}")
            }
            ChapelType::Tuple(ts) => {
                write!(f, "(")?;
                for (i, t) in ts.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{t}")?;
                }
                write!(f, ")")
            }
            ChapelType::Named(name) => write!(f, "{name}"),
            ChapelType::Void => write!(f, "void"),
            ChapelType::TypeVar(name) => write!(f, "?{name}"),
            ChapelType::Atomic(t) => write!(f, "atomic {t}"),
            ChapelType::Sync(t) => write!(f, "sync {t}"),
            ChapelType::Single(t) => write!(f, "single {t}"),
            ChapelType::Owned(t) => write!(f, "owned {t}"),
            ChapelType::Shared(t) => write!(f, "shared {t}"),
            ChapelType::Unmanaged(t) => write!(f, "unmanaged {t}"),
        }
    }
}
