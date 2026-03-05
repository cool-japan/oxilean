//! # SolidityExpr - Trait Implementations
//!
//! This module contains trait implementations for `SolidityExpr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SolidityExpr;
use std::fmt;

impl fmt::Display for SolidityExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SolidityExpr::IntLit(n) => write!(f, "{}", n),
            SolidityExpr::BoolLit(b) => write!(f, "{}", b),
            SolidityExpr::StrLit(s) => write!(f, "\"{}\"", s),
            SolidityExpr::AddressLit(a) => write!(f, "{}", a),
            SolidityExpr::HexLit(h) => write!(f, "{}", h),
            SolidityExpr::Var(v) => write!(f, "{}", v),
            SolidityExpr::This => write!(f, "this"),
            SolidityExpr::MsgSender => write!(f, "msg.sender"),
            SolidityExpr::MsgValue => write!(f, "msg.value"),
            SolidityExpr::MsgData => write!(f, "msg.data"),
            SolidityExpr::BlockTimestamp => write!(f, "block.timestamp"),
            SolidityExpr::BlockNumber => write!(f, "block.number"),
            SolidityExpr::BlockBasefee => write!(f, "block.basefee"),
            SolidityExpr::TxOrigin => write!(f, "tx.origin"),
            SolidityExpr::GasLeft => write!(f, "gasleft()"),
            SolidityExpr::FieldAccess(e, field) => write!(f, "{}.{}", e, field),
            SolidityExpr::Index(e, idx) => write!(f, "{}[{}]", e, idx),
            SolidityExpr::Call(func, args) => {
                write!(f, "{}(", func)?;
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", a)?;
                }
                write!(f, ")")
            }
            SolidityExpr::NamedCall(func, args) => {
                write!(f, "{}({{", func)?;
                for (i, (k, v)) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", k, v)?;
                }
                write!(f, "}})")
            }
            SolidityExpr::Cast(ty, expr) => write!(f, "{}({})", ty, expr),
            SolidityExpr::AbiEncode(args) => {
                write!(f, "abi.encode(")?;
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", a)?;
                }
                write!(f, ")")
            }
            SolidityExpr::AbiEncodePacked(args) => {
                write!(f, "abi.encodePacked(")?;
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", a)?;
                }
                write!(f, ")")
            }
            SolidityExpr::AbiEncodeWithSelector(sel, args) => {
                write!(f, "abi.encodeWithSelector({}", sel)?;
                for a in args.iter() {
                    write!(f, ", {}", a)?;
                }
                write!(f, ")")
            }
            SolidityExpr::Keccak256(data) => write!(f, "keccak256({})", data),
            SolidityExpr::Sha256(data) => write!(f, "sha256({})", data),
            SolidityExpr::Ecrecover(h, v, r, s) => {
                write!(f, "ecrecover({}, {}, {}, {})", h, v, r, s)
            }
            SolidityExpr::BinOp(op, lhs, rhs) => write!(f, "({} {} {})", lhs, op, rhs),
            SolidityExpr::UnaryOp(op, expr) => write!(f, "{}({})", op, expr),
            SolidityExpr::Ternary(cond, then_, else_) => {
                write!(f, "({} ? {} : {})", cond, then_, else_)
            }
            SolidityExpr::New(ty, args) => {
                write!(f, "new {}(", ty)?;
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", a)?;
                }
                write!(f, ")")
            }
            SolidityExpr::Delete(expr) => write!(f, "delete {}", expr),
            SolidityExpr::ArrayLit(elems) => {
                write!(f, "[")?;
                for (i, e) in elems.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", e)?;
                }
                write!(f, "]")
            }
            SolidityExpr::TupleLit(elems) => {
                write!(f, "(")?;
                for (i, e) in elems.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", e)?;
                }
                write!(f, ")")
            }
            SolidityExpr::TypeMax(ty) => write!(f, "type({}).max", ty),
            SolidityExpr::TypeMin(ty) => write!(f, "type({}).min", ty),
            SolidityExpr::Payable(addr) => write!(f, "payable({})", addr),
        }
    }
}
