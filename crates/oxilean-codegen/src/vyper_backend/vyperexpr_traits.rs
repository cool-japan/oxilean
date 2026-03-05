//! # VyperExpr - Trait Implementations
//!
//! This module contains trait implementations for `VyperExpr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::VyperExpr;
use std::fmt;

impl fmt::Display for VyperExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VyperExpr::IntLit(n) => write!(f, "{}", n),
            VyperExpr::BoolLit(b) => write!(f, "{}", if *b { "True" } else { "False" }),
            VyperExpr::StrLit(s) => write!(f, "\"{}\"", s),
            VyperExpr::HexLit(h) => write!(f, "{}", h),
            VyperExpr::Var(v) => write!(f, "{}", v),
            VyperExpr::SelfField(field) => write!(f, "self.{}", field),
            VyperExpr::MsgSender => write!(f, "msg.sender"),
            VyperExpr::MsgValue => write!(f, "msg.value"),
            VyperExpr::BlockTimestamp => write!(f, "block.timestamp"),
            VyperExpr::BlockNumber => write!(f, "block.number"),
            VyperExpr::ChainId => write!(f, "chain.id"),
            VyperExpr::TxOrigin => write!(f, "tx.origin"),
            VyperExpr::Len(expr) => write!(f, "len({})", expr),
            VyperExpr::Convert(expr, ty) => write!(f, "convert({}, {})", expr, ty),
            VyperExpr::Concat(args) => {
                let strs: Vec<String> = args.iter().map(|a| a.to_string()).collect();
                write!(f, "concat({})", strs.join(", "))
            }
            VyperExpr::Keccak256(expr) => write!(f, "keccak256({})", expr),
            VyperExpr::Sha256(expr) => write!(f, "sha256({})", expr),
            VyperExpr::Ecrecover(h, v, r, s) => {
                write!(f, "ecrecover({}, {}, {}, {})", h, v, r, s)
            }
            VyperExpr::Extract32(expr, start) => {
                write!(f, "extract32({}, {})", expr, start)
            }
            VyperExpr::FieldAccess(expr, field) => write!(f, "{}.{}", expr, field),
            VyperExpr::Index(expr, idx) => write!(f, "{}[{}]", expr, idx),
            VyperExpr::Call(name, args) => {
                let strs: Vec<String> = args.iter().map(|a| a.to_string()).collect();
                write!(f, "{}({})", name, strs.join(", "))
            }
            VyperExpr::ExtCall(iface, addr, method, args) => {
                let strs: Vec<String> = args.iter().map(|a| a.to_string()).collect();
                write!(f, "{}({}).{}({})", iface, addr, method, strs.join(", "))
            }
            VyperExpr::BinOp(op, lhs, rhs) => write!(f, "({} {} {})", lhs, op, rhs),
            VyperExpr::UnaryOp(op, expr) => {
                if op == "not" {
                    write!(f, "not {}", expr)
                } else {
                    write!(f, "{}{}", op, expr)
                }
            }
            VyperExpr::IfExpr(val, cond, default) => {
                write!(f, "{} if {} else {}", val, cond, default)
            }
            VyperExpr::StructLit(name, fields) => {
                let field_strs: Vec<String> = fields
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect();
                write!(f, "{}({{{}}})", name, field_strs.join(", "))
            }
            VyperExpr::ListLit(elems) => {
                let strs: Vec<String> = elems.iter().map(|e| e.to_string()).collect();
                write!(f, "[{}]", strs.join(", "))
            }
            VyperExpr::Empty(ty) => write!(f, "empty({})", ty),
            VyperExpr::MaxValue(ty) => write!(f, "max_value({})", ty),
            VyperExpr::MinValue(ty) => write!(f, "min_value({})", ty),
            VyperExpr::Isqrt(expr) => write!(f, "isqrt({})", expr),
            VyperExpr::Uint2Str(expr) => write!(f, "uint2str({})", expr),
            VyperExpr::RawCall {
                addr,
                data,
                value,
                gas,
            } => {
                write!(f, "raw_call({}, {}", addr, data)?;
                if let Some(v) = value {
                    write!(f, ", value={}", v)?;
                }
                if let Some(g) = gas {
                    write!(f, ", gas={}", g)?;
                }
                write!(f, ")")
            }
            VyperExpr::CreateMinimalProxy(target) => {
                write!(f, "create_minimal_proxy_to({})", target)
            }
            VyperExpr::CreateCopyOf(target) => write!(f, "create_copy_of({})", target),
            VyperExpr::CreateFromBlueprint(bp, args) => {
                let strs: Vec<String> = args.iter().map(|a| a.to_string()).collect();
                if strs.is_empty() {
                    write!(f, "create_from_blueprint({})", bp)
                } else {
                    write!(f, "create_from_blueprint({}, {})", bp, strs.join(", "))
                }
            }
        }
    }
}
