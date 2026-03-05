//! # OleanError - Trait Implementations
//!
//! This module contains trait implementations for `OleanError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//! - `Error`
//! - `From`
//! - `From`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use super::types::OleanError;
use std::fmt;

impl fmt::Display for OleanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OleanError::InvalidMagic => {
                write!(f, "invalid OleanC magic bytes (expected 'OLNC')")
            }
            OleanError::UnsupportedVersion(v) => {
                write!(f, "unsupported OleanC version: {v} (supported: {VERSION})")
            }
            OleanError::UnexpectedEof => {
                write!(f, "unexpected end of file while reading OleanC data")
            }
            OleanError::InvalidUtf8(e) => {
                write!(f, "invalid UTF-8 in OleanC string: {e}")
            }
            OleanError::InvalidDeclKind(k) => {
                write!(f, "invalid declaration kind tag: {k}")
            }
            OleanError::IoError(e) => write!(f, "I/O error: {e}"),
        }
    }
}

impl std::error::Error for OleanError {}

impl From<std::io::Error> for OleanError {
    fn from(e: std::io::Error) -> Self {
        OleanError::IoError(e)
    }
}

impl From<std::string::FromUtf8Error> for OleanError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        OleanError::InvalidUtf8(e)
    }
}
