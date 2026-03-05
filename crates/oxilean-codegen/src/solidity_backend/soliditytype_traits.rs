//! # SolidityType - Trait Implementations
//!
//! This module contains trait implementations for `SolidityType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SolidityType;
use std::fmt;

impl fmt::Display for SolidityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SolidityType::Uint256 => write!(f, "uint256"),
            SolidityType::Uint128 => write!(f, "uint128"),
            SolidityType::Uint64 => write!(f, "uint64"),
            SolidityType::Uint32 => write!(f, "uint32"),
            SolidityType::Uint8 => write!(f, "uint8"),
            SolidityType::Int256 => write!(f, "int256"),
            SolidityType::Int128 => write!(f, "int128"),
            SolidityType::Int64 => write!(f, "int64"),
            SolidityType::Int32 => write!(f, "int32"),
            SolidityType::Int8 => write!(f, "int8"),
            SolidityType::Address => write!(f, "address"),
            SolidityType::AddressPayable => write!(f, "address payable"),
            SolidityType::Bool => write!(f, "bool"),
            SolidityType::Bytes => write!(f, "bytes"),
            SolidityType::Bytes32 => write!(f, "bytes32"),
            SolidityType::Bytes16 => write!(f, "bytes16"),
            SolidityType::Bytes4 => write!(f, "bytes4"),
            SolidityType::StringTy => write!(f, "string"),
            SolidityType::Mapping(k, v) => write!(f, "mapping({} => {})", k, v),
            SolidityType::DynArray(elem) => write!(f, "{}[]", elem),
            SolidityType::FixedArray(elem, n) => write!(f, "{}[{}]", elem, n),
            SolidityType::Named(name) => write!(f, "{}", name),
            SolidityType::Tuple(elems) => {
                write!(f, "tuple(")?;
                for (i, t) in elems.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", t)?;
                }
                write!(f, ")")
            }
        }
    }
}
