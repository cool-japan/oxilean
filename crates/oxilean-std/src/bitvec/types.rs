//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// Bit manipulation helper utilities.
#[allow(dead_code)]
pub struct BitManipulator {
    mask: u64,
    width: usize,
}
/// Fixed-width bitvector backed by a const generic width parameter.
#[allow(dead_code)]
pub struct BitVecFixed<const N: usize> {
    pub data: u64,
}
/// SMT-LIB BitVec representation for solver integration.
#[allow(dead_code)]
pub struct BitVecSMT {
    name: String,
    width: usize,
}
/// Arithmetic utilities for bitvector operations.
#[allow(dead_code)]
pub struct BitVecArithmetic {
    width: usize,
    signed: bool,
}
/// Precomputed population count lookup table.
#[allow(dead_code)]
pub struct PopcountTable {
    table: Vec<u8>,
}
