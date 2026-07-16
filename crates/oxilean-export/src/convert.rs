//! Typed field-extraction helpers over [`JsonValue`], with three-bucket error
//! reporting.
//!
//! These keep the record dispatch in [`crate::reader`] terse and make every
//! "missing field / wrong type" case a [`ExportError::Malformed`] with a precise
//! position.

use crate::error::{ExportError, ExportResult, Position};
use crate::json::JsonValue;

/// A view over a JSON object with helpers that report the source position on
/// failure.
pub(crate) struct Obj<'a> {
    map: &'a std::collections::BTreeMap<String, JsonValue>,
    pos: Position,
}

impl<'a> Obj<'a> {
    /// Wrap a value expected to be an object.
    pub(crate) fn new(value: &'a JsonValue, pos: Position) -> ExportResult<Self> {
        match value.as_object() {
            Some(map) => Ok(Self { map, pos }),
            None => Err(ExportError::malformed(pos, "expected a JSON object")),
        }
    }

    /// Look up a required field.
    pub(crate) fn get(&self, key: &str) -> ExportResult<&'a JsonValue> {
        self.map
            .get(key)
            .ok_or_else(|| ExportError::malformed(self.pos, format!("missing field {key:?}")))
    }

    /// Look up an optional field.
    pub(crate) fn get_opt(&self, key: &str) -> Option<&'a JsonValue> {
        self.map.get(key)
    }

    /// Read a required string field.
    pub(crate) fn str(&self, key: &str) -> ExportResult<&'a str> {
        self.get(key)?.as_str().ok_or_else(|| {
            ExportError::malformed(self.pos, format!("field {key:?} must be a string"))
        })
    }

    /// Read a required boolean field.
    pub(crate) fn bool(&self, key: &str) -> ExportResult<bool> {
        self.get(key)?.as_bool().ok_or_else(|| {
            ExportError::malformed(self.pos, format!("field {key:?} must be a boolean"))
        })
    }

    /// Read a required non-negative integer index field as `usize`.
    pub(crate) fn index(&self, key: &str) -> ExportResult<usize> {
        parse_index(self.get(key)?, self.pos, key)
    }

    /// Read a required non-negative integer field as `u32`.
    pub(crate) fn u32(&self, key: &str) -> ExportResult<u32> {
        let s = self.get(key)?.as_int_str().ok_or_else(|| {
            ExportError::malformed(self.pos, format!("field {key:?} must be an integer"))
        })?;
        s.parse::<u32>().map_err(|_| {
            ExportError::malformed(self.pos, format!("field {key:?} out of u32 range: {s}"))
        })
    }

    /// Read a required non-negative integer field as `u64`.
    pub(crate) fn u64(&self, key: &str) -> ExportResult<u64> {
        let s = self.get(key)?.as_int_str().ok_or_else(|| {
            ExportError::malformed(self.pos, format!("field {key:?} must be an integer"))
        })?;
        s.parse::<u64>().map_err(|_| {
            ExportError::malformed(self.pos, format!("field {key:?} out of u64 range: {s}"))
        })
    }

    /// Read a required array field of index integers.
    pub(crate) fn index_array(&self, key: &str) -> ExportResult<Vec<usize>> {
        let arr = self.get(key)?.as_array().ok_or_else(|| {
            ExportError::malformed(self.pos, format!("field {key:?} must be an array"))
        })?;
        arr.iter().map(|v| parse_index(v, self.pos, key)).collect()
    }

    /// Read a required array field of arbitrary values.
    pub(crate) fn array(&self, key: &str) -> ExportResult<&'a [JsonValue]> {
        self.get(key)?.as_array().ok_or_else(|| {
            ExportError::malformed(self.pos, format!("field {key:?} must be an array"))
        })
    }
}

/// Parse a JSON integer as a non-negative table index (`usize`).
pub(crate) fn parse_index(value: &JsonValue, pos: Position, ctx: &str) -> ExportResult<usize> {
    let s = value
        .as_int_str()
        .ok_or_else(|| ExportError::malformed(pos, format!("{ctx} must be an integer index")))?;
    if s.starts_with('-') {
        return Err(ExportError::malformed(
            pos,
            format!("{ctx} index must be non-negative: {s}"),
        ));
    }
    s.parse::<usize>()
        .map_err(|_| ExportError::malformed(pos, format!("{ctx} index out of range: {s}")))
}
