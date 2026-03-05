//! # VyperType - Trait Implementations
//!
//! This module contains trait implementations for `VyperType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;

use super::types::VyperType;
use std::fmt;

impl fmt::Display for VyperType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VyperType::Uint256 => write!(f, "uint256"),
            VyperType::Uint128 => write!(f, "uint128"),
            VyperType::Uint64 => write!(f, "uint64"),
            VyperType::Uint32 => write!(f, "uint32"),
            VyperType::Uint8 => write!(f, "uint8"),
            VyperType::Int256 => write!(f, "int256"),
            VyperType::Int128 => write!(f, "int128"),
            VyperType::Int64 => write!(f, "int64"),
            VyperType::Int32 => write!(f, "int32"),
            VyperType::Int8 => write!(f, "int8"),
            VyperType::Address => write!(f, "address"),
            VyperType::Bool => write!(f, "bool"),
            VyperType::Bytes32 => write!(f, "bytes32"),
            VyperType::Bytes4 => write!(f, "bytes4"),
            VyperType::Bytes(n) => write!(f, "Bytes[{}]", n),
            VyperType::StringTy(n) => write!(f, "String[{}]", n),
            VyperType::DynArray(elem, n) => write!(f, "DynArray[{}, {}]", elem, n),
            VyperType::FixedArray(elem, n) => write!(f, "{}[{}]", elem, n),
            VyperType::HashMap(k, v) => write!(f, "HashMap[{}, {}]", k, v),
            VyperType::Struct(name) => write!(f, "{}", name),
            VyperType::Decimal => write!(f, "decimal"),
            VyperType::Flag(name) => write!(f, "{}", name),
        }
    }
}
