//! # StripNullFields - Trait Implementations
//!
//! This module contains trait implementations for `StripNullFields`.
//!
//! ## Implemented Traits
//!
//! - `JsonValueTransform`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::JsonValueTransform;
use super::types::{JsonValue, StripNullFields};
use std::fmt;

#[allow(dead_code)]
impl JsonValueTransform for StripNullFields {
    fn transform(&self, value: JsonValue) -> JsonValue {
        match value {
            JsonValue::Object(map) => {
                let map: Vec<(String, JsonValue)> = map
                    .into_iter()
                    .filter(|(_, v)| !matches!(v, JsonValue::Null))
                    .map(|(k, v)| (k, self.transform(v)))
                    .collect();
                JsonValue::Object(map)
            }
            JsonValue::Array(arr) => {
                JsonValue::Array(arr.into_iter().map(|v| self.transform(v)).collect())
            }
            other => other,
        }
    }
}
