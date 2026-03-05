//! # WasmInstr - Trait Implementations
//!
//! This module contains trait implementations for `WasmInstr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::WasmInstr;
use std::fmt;

impl fmt::Display for WasmInstr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WasmInstr::LocalGet(name) => write!(f, "local.get ${}", name),
            WasmInstr::LocalSet(name) => write!(f, "local.set ${}", name),
            WasmInstr::I32Const(n) => write!(f, "i32.const {}", n),
            WasmInstr::I64Const(n) => write!(f, "i64.const {}", n),
            WasmInstr::F64Const(n) => write!(f, "f64.const {}", n),
            WasmInstr::Call(name) => write!(f, "call ${}", name),
            WasmInstr::CallIndirect => write!(f, "call_indirect"),
            WasmInstr::Return => write!(f, "return"),
            WasmInstr::Drop => write!(f, "drop"),
            WasmInstr::BrIf(label) => write!(f, "br_if {}", label),
            WasmInstr::Block => write!(f, "block"),
            WasmInstr::Loop => write!(f, "loop"),
            WasmInstr::End => write!(f, "end"),
            WasmInstr::Nop => write!(f, "nop"),
            WasmInstr::Unreachable => write!(f, "unreachable"),
            WasmInstr::Select => write!(f, "select"),
            WasmInstr::I32Add => write!(f, "i32.add"),
            WasmInstr::I32Sub => write!(f, "i32.sub"),
            WasmInstr::I32Mul => write!(f, "i32.mul"),
            WasmInstr::I32DivS => write!(f, "i32.div_s"),
            WasmInstr::I64Add => write!(f, "i64.add"),
            WasmInstr::I64Mul => write!(f, "i64.mul"),
            WasmInstr::F64Add => write!(f, "f64.add"),
            WasmInstr::F64Mul => write!(f, "f64.mul"),
            WasmInstr::F64Div => write!(f, "f64.div"),
            WasmInstr::F64Sqrt => write!(f, "f64.sqrt"),
            WasmInstr::MemLoad(align) => write!(f, "i32.load align={}", align),
            WasmInstr::MemStore(align) => write!(f, "i32.store align={}", align),
            WasmInstr::I32Eqz => write!(f, "i32.eqz"),
            WasmInstr::I32Eq => write!(f, "i32.eq"),
            WasmInstr::I32Ne => write!(f, "i32.ne"),
            WasmInstr::I32LtS => write!(f, "i32.lt_s"),
            WasmInstr::I32GtS => write!(f, "i32.gt_s"),
            WasmInstr::I32LeS => write!(f, "i32.le_s"),
            WasmInstr::I32GeS => write!(f, "i32.ge_s"),
            WasmInstr::RefNull => write!(f, "ref.null funcref"),
            WasmInstr::RefIsNull => write!(f, "ref.is_null"),
            WasmInstr::TableGet(idx) => write!(f, "table.get {}", idx),
            WasmInstr::TableSet(idx) => write!(f, "table.set {}", idx),
        }
    }
}
