//! # JuliaType - Trait Implementations
//!
//! This module contains trait implementations for `JuliaType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::{emit_expr, emit_stmt_inline};
use super::types::JuliaType;
use std::fmt;

impl fmt::Display for JuliaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JuliaType::Int8 => write!(f, "Int8"),
            JuliaType::Int16 => write!(f, "Int16"),
            JuliaType::Int32 => write!(f, "Int32"),
            JuliaType::Int64 => write!(f, "Int64"),
            JuliaType::Int128 => write!(f, "Int128"),
            JuliaType::UInt8 => write!(f, "UInt8"),
            JuliaType::UInt16 => write!(f, "UInt16"),
            JuliaType::UInt32 => write!(f, "UInt32"),
            JuliaType::UInt64 => write!(f, "UInt64"),
            JuliaType::UInt128 => write!(f, "UInt128"),
            JuliaType::Float32 => write!(f, "Float32"),
            JuliaType::Float64 => write!(f, "Float64"),
            JuliaType::Bool => write!(f, "Bool"),
            JuliaType::String => write!(f, "String"),
            JuliaType::Char => write!(f, "Char"),
            JuliaType::Nothing => write!(f, "Nothing"),
            JuliaType::Any => write!(f, "Any"),
            JuliaType::Vector(t) => write!(f, "Vector{{{}}}", t),
            JuliaType::Matrix(t) => write!(f, "Matrix{{{}}}", t),
            JuliaType::Array(t, n) => write!(f, "Array{{{}, {}}}", t, n),
            JuliaType::Tuple(ts) => {
                write!(f, "Tuple{{")?;
                for (i, t) in ts.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", t)?;
                }
                write!(f, "}}")
            }
            JuliaType::NamedTuple(fields) => {
                write!(f, "NamedTuple{{(")?;
                for (i, (name, _)) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, ":{}", name)?;
                }
                write!(f, "), Tuple{{")?;
                for (i, (_, ty)) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", ty)?;
                }
                write!(f, "}}}}")
            }
            JuliaType::Union(ts) => {
                write!(f, "Union{{")?;
                for (i, t) in ts.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", t)?;
                }
                write!(f, "}}")
            }
            JuliaType::Abstract(name) => write!(f, "{}", name),
            JuliaType::Parametric(name, params) => {
                write!(f, "{}", name)?;
                if !params.is_empty() {
                    write!(f, "{{")?;
                    for (i, p) in params.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", p)?;
                    }
                    write!(f, "}}")?;
                }
                Ok(())
            }
            JuliaType::TypeVar(name) => write!(f, "{}", name),
            JuliaType::Function => write!(f, "Function"),
            JuliaType::Dict(k, v) => write!(f, "Dict{{{}, {}}}", k, v),
            JuliaType::Set(t) => write!(f, "Set{{{}}}", t),
            JuliaType::Ref(t) => write!(f, "Ref{{{}}}", t),
            JuliaType::Named(name) => write!(f, "{}", name),
        }
    }
}
