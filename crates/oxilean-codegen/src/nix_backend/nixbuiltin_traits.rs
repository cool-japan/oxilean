//! # NixBuiltin - Trait Implementations
//!
//! This module contains trait implementations for `NixBuiltin`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::NixBuiltin;
use std::fmt;

impl std::fmt::Display for NixBuiltin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            NixBuiltin::IsInt => "isInt",
            NixBuiltin::IsFloat => "isFloat",
            NixBuiltin::IsBool => "isBool",
            NixBuiltin::IsString => "isString",
            NixBuiltin::IsPath => "isPath",
            NixBuiltin::IsNull => "isNull",
            NixBuiltin::IsList => "isList",
            NixBuiltin::IsAttrs => "isAttrs",
            NixBuiltin::IsFunction => "isFunction",
            NixBuiltin::StringLength => "stringLength",
            NixBuiltin::SubString => "substring",
            NixBuiltin::Concat => "concatLists",
            NixBuiltin::ConcatStringsSep => "concatStringsSep",
            NixBuiltin::ToString => "toString",
            NixBuiltin::ParseInt => "parseInt",
            NixBuiltin::ParseFloat => "parseFloat",
            NixBuiltin::ToLower => "toLower",
            NixBuiltin::ToUpper => "toUpper",
            NixBuiltin::HasSuffix => "hasSuffix",
            NixBuiltin::HasPrefix => "hasPrefix",
            NixBuiltin::StringSplit => "splitString",
            NixBuiltin::ReplaceStrings => "replaceStrings",
            NixBuiltin::Length => "length",
            NixBuiltin::Head => "head",
            NixBuiltin::Tail => "tail",
            NixBuiltin::Filter => "filter",
            NixBuiltin::Map => "map",
            NixBuiltin::FoldLeft => "foldl'",
            NixBuiltin::FoldRight => "foldr",
            NixBuiltin::Concatmap => "concatMap",
            NixBuiltin::Elem => "elem",
            NixBuiltin::ElemAt => "elemAt",
            NixBuiltin::Flatten => "flatten",
            NixBuiltin::Sort => "sort",
            NixBuiltin::Partition => "partition",
            NixBuiltin::GroupBy => "groupBy",
            NixBuiltin::ZipAttrsWith => "zipAttrsWith",
            NixBuiltin::Unique => "unique",
            NixBuiltin::Reversal => "reverseList",
            NixBuiltin::Intersect => "intersectLists",
            NixBuiltin::SubtractLists => "subtractLists",
            NixBuiltin::ListToAttrs => "listToAttrs",
            NixBuiltin::AttrNames => "attrNames",
            NixBuiltin::AttrValues => "attrValues",
            NixBuiltin::HasAttr => "hasAttr",
            NixBuiltin::GetAttr => "getAttr",
            NixBuiltin::Intersect2 => "intersectAttrs",
            NixBuiltin::RemoveAttrs => "removeAttrs",
            NixBuiltin::MapAttrs => "mapAttrs",
            NixBuiltin::FilterAttrs => "filterAttrs",
            NixBuiltin::Foldl2 => "foldlAttrs",
            NixBuiltin::ToJSON => "toJSON",
            NixBuiltin::FromJSON => "fromJSON",
            NixBuiltin::ToTOML => "toTOML",
            NixBuiltin::ReadFile => "readFile",
            NixBuiltin::ReadDir => "readDir",
            NixBuiltin::PathExists => "pathExists",
            NixBuiltin::BaseName => "baseNameOf",
            NixBuiltin::DirOf => "dirOf",
            NixBuiltin::ToPath => "toPath",
            NixBuiltin::StorePath => "storePath",
            NixBuiltin::DerivationStrict => "derivationStrict",
            NixBuiltin::PlaceholderOf => "placeholder",
            NixBuiltin::HashString => "hashString",
            NixBuiltin::HashFile => "hashFile",
            NixBuiltin::TypeOf => "typeOf",
            NixBuiltin::Seq => "seq",
            NixBuiltin::DeepSeq => "deepSeq",
            NixBuiltin::Trace => "trace",
            NixBuiltin::Abort => "abort",
            NixBuiltin::Throw => "throw",
            NixBuiltin::CurrentSystem => "currentSystem",
            NixBuiltin::CurrentTime => "currentTime",
            NixBuiltin::NixVersion => "nixVersion",
        };
        write!(f, "builtins.{}", name)
    }
}
