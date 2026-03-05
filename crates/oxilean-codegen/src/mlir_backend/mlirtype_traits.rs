//! # MlirType - Trait Implementations
//!
//! This module contains trait implementations for `MlirType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{AffineMap, MlirType};
use std::fmt;

impl fmt::Display for MlirType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MlirType::Integer(bits, signed) => {
                if *signed {
                    write!(f, "si{}", bits)
                } else {
                    write!(f, "i{}", bits)
                }
            }
            MlirType::Float(bits) => match bits {
                16 => write!(f, "f16"),
                32 => write!(f, "f32"),
                64 => write!(f, "f64"),
                80 => write!(f, "f80"),
                128 => write!(f, "f128"),
                _ => write!(f, "bf{}", bits),
            },
            MlirType::Index => write!(f, "index"),
            MlirType::MemRef(elem, dims, amap) => {
                write!(f, "memref<")?;
                for d in dims {
                    if *d < 0 {
                        write!(f, "?x")?;
                    } else {
                        write!(f, "{}x", d)?;
                    }
                }
                write!(f, "{}", elem)?;
                if let AffineMap::Custom(s) = amap {
                    write!(f, ", {}>", s)?;
                } else {
                    write!(f, ">")?;
                }
                Ok(())
            }
            MlirType::Tensor(dims, elem) => {
                write!(f, "tensor<")?;
                for d in dims {
                    if *d < 0 {
                        write!(f, "?x")?;
                    } else {
                        write!(f, "{}x", d)?;
                    }
                }
                write!(f, "{}>", elem)
            }
            MlirType::Vector(dims, elem) => {
                write!(f, "vector<")?;
                for d in dims {
                    write!(f, "{}x", d)?;
                }
                write!(f, "{}>", elem)
            }
            MlirType::Tuple(types) => {
                write!(f, "tuple<")?;
                for (i, t) in types.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", t)?;
                }
                write!(f, ">")
            }
            MlirType::NoneType => write!(f, "none"),
            MlirType::Custom(s) => write!(f, "!{}", s),
            MlirType::FuncType(inputs, outputs) => {
                write!(f, "(")?;
                for (i, t) in inputs.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", t)?;
                }
                write!(f, ") -> ")?;
                if outputs.len() == 1 {
                    write!(f, "{}", outputs[0])
                } else {
                    write!(f, "(")?;
                    for (i, t) in outputs.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", t)?;
                    }
                    write!(f, ")")
                }
            }
            MlirType::Complex(inner) => write!(f, "complex<{}>", inner),
            MlirType::UnrankedMemRef(elem) => write!(f, "memref<*x{}>", elem),
        }
    }
}
