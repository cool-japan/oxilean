//! # PythonType - Trait Implementations
//!
//! This module contains trait implementations for `PythonType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::PythonType;
use std::fmt;

impl fmt::Display for PythonType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PythonType::Int => write!(f, "int"),
            PythonType::Float => write!(f, "float"),
            PythonType::Str => write!(f, "str"),
            PythonType::Bool => write!(f, "bool"),
            PythonType::None_ => write!(f, "None"),
            PythonType::List(t) => write!(f, "list[{}]", t),
            PythonType::Dict(k, v) => write!(f, "dict[{}, {}]", k, v),
            PythonType::Tuple(ts) => {
                write!(f, "tuple[")?;
                for (i, t) in ts.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", t)?;
                }
                write!(f, "]")
            }
            PythonType::Optional(t) => write!(f, "{} | None", t),
            PythonType::Union(ts) => {
                for (i, t) in ts.iter().enumerate() {
                    if i > 0 {
                        write!(f, " | ")?;
                    }
                    write!(f, "{}", t)?;
                }
                Ok(())
            }
            PythonType::Custom(s) => write!(f, "{}", s),
            PythonType::Any => write!(f, "Any"),
            PythonType::Callable => write!(f, "Callable"),
            PythonType::Set(t) => write!(f, "set[{}]", t),
            PythonType::FrozenSet(t) => write!(f, "frozenset[{}]", t),
            PythonType::Generator(y, s, r) => write!(f, "Generator[{}, {}, {}]", y, s, r),
            PythonType::AsyncGenerator(y, s) => write!(f, "AsyncGenerator[{}, {}]", y, s),
            PythonType::Iterator(t) => write!(f, "Iterator[{}]", t),
            PythonType::Iterable(t) => write!(f, "Iterable[{}]", t),
            PythonType::Sequence(t) => write!(f, "Sequence[{}]", t),
            PythonType::Mapping(k, v) => write!(f, "Mapping[{}, {}]", k, v),
            PythonType::ClassVar(t) => write!(f, "ClassVar[{}]", t),
            PythonType::Final(t) => write!(f, "Final[{}]", t),
            PythonType::Type(t) => write!(f, "type[{}]", t),
        }
    }
}
