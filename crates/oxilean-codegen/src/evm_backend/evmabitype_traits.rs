//! # EvmAbiType - Trait Implementations
//!
//! This module contains trait implementations for `EvmAbiType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::EvmAbiType;
use std::fmt;

impl std::fmt::Display for EvmAbiType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvmAbiType::Uint(n) => write!(f, "uint{}", n),
            EvmAbiType::Int(n) => write!(f, "int{}", n),
            EvmAbiType::Address => write!(f, "address"),
            EvmAbiType::Bool => write!(f, "bool"),
            EvmAbiType::Bytes(n) => write!(f, "bytes{}", n),
            EvmAbiType::BytesDyn => write!(f, "bytes"),
            EvmAbiType::StringDyn => write!(f, "string"),
            EvmAbiType::Tuple(ts) => {
                let ss: Vec<String> = ts.iter().map(|t| t.to_string()).collect();
                write!(f, "({})", ss.join(","))
            }
            EvmAbiType::Array(t, None) => write!(f, "{}[]", t),
            EvmAbiType::Array(t, Some(n)) => write!(f, "{}[{}]", t, n),
        }
    }
}
