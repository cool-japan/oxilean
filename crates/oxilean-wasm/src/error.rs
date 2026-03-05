//! Error types for the WASM API
#![allow(dead_code)]

use std::fmt;

#[derive(Debug, Clone)]
pub enum WasmError {
    ParseError(String),
    ElabError(String),
    KernelError(String),
    IoError(String),
    Internal(String),
}

impl fmt::Display for WasmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WasmError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            WasmError::ElabError(msg) => write!(f, "Elaboration error: {}", msg),
            WasmError::KernelError(msg) => write!(f, "Kernel error: {}", msg),
            WasmError::IoError(msg) => write!(f, "IO error: {}", msg),
            WasmError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for WasmError {}

pub type WasmResult<T> = Result<T, WasmError>;
