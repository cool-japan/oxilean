//! # MetalType - Trait Implementations
//!
//! This module contains trait implementations for `MetalType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MetalType;
use std::fmt;

impl fmt::Display for MetalType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MetalType::Bool => write!(f, "bool"),
            MetalType::Half => write!(f, "half"),
            MetalType::Float => write!(f, "float"),
            MetalType::Int => write!(f, "int"),
            MetalType::Uint => write!(f, "uint"),
            MetalType::Short => write!(f, "short"),
            MetalType::Ushort => write!(f, "ushort"),
            MetalType::Char => write!(f, "char"),
            MetalType::Uchar => write!(f, "uchar"),
            MetalType::Long => write!(f, "long"),
            MetalType::Ulong => write!(f, "ulong"),
            MetalType::Float2 => write!(f, "float2"),
            MetalType::Float3 => write!(f, "float3"),
            MetalType::Float4 => write!(f, "float4"),
            MetalType::Half2 => write!(f, "half2"),
            MetalType::Half3 => write!(f, "half3"),
            MetalType::Half4 => write!(f, "half4"),
            MetalType::Int2 => write!(f, "int2"),
            MetalType::Int3 => write!(f, "int3"),
            MetalType::Int4 => write!(f, "int4"),
            MetalType::Uint2 => write!(f, "uint2"),
            MetalType::Uint3 => write!(f, "uint3"),
            MetalType::Uint4 => write!(f, "uint4"),
            MetalType::Float2x2 => write!(f, "float2x2"),
            MetalType::Float3x3 => write!(f, "float3x3"),
            MetalType::Float4x4 => write!(f, "float4x4"),
            MetalType::Array(elem, size) => write!(f, "{}[{}]", elem, size),
            MetalType::Struct(name) => write!(f, "{}", name),
            MetalType::Texture(elem) => write!(f, "texture2d<{}>", elem),
            MetalType::Sampler => write!(f, "sampler"),
            MetalType::Pointer(inner, space) => write!(f, "{} {}*", space, inner),
            MetalType::Void => write!(f, "void"),
        }
    }
}
