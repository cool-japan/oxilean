//! # LlvmInstr - Trait Implementations
//!
//! This module contains trait implementations for `LlvmInstr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::LlvmInstr;
use std::fmt;

impl fmt::Display for LlvmInstr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LlvmInstr::Alloca { result, ty, align } => {
                write!(f, "  %{} = alloca {}", result, ty)?;
                if let Some(a) = align {
                    write!(f, ", align {}", a)?;
                }
                Ok(())
            }
            LlvmInstr::Load {
                result,
                ty,
                ptr,
                align,
            } => {
                write!(f, "  %{} = load {}, ptr {}", result, ty, ptr)?;
                if let Some(a) = align {
                    write!(f, ", align {}", a)?;
                }
                Ok(())
            }
            LlvmInstr::Store {
                val,
                ty,
                ptr,
                align,
            } => {
                write!(f, "  store {} {}, ptr {}", ty, val, ptr)?;
                if let Some(a) = align {
                    write!(f, ", align {}", a)?;
                }
                Ok(())
            }
            LlvmInstr::Add { result, lhs, rhs } => {
                write!(f, "  %{} = add i64 {}, {}", result, lhs, rhs)
            }
            LlvmInstr::Sub { result, lhs, rhs } => {
                write!(f, "  %{} = sub i64 {}, {}", result, lhs, rhs)
            }
            LlvmInstr::Mul { result, lhs, rhs } => {
                write!(f, "  %{} = mul i64 {}, {}", result, lhs, rhs)
            }
            LlvmInstr::SDiv { result, lhs, rhs } => {
                write!(f, "  %{} = sdiv i64 {}, {}", result, lhs, rhs)
            }
            LlvmInstr::SRem { result, lhs, rhs } => {
                write!(f, "  %{} = srem i64 {}, {}", result, lhs, rhs)
            }
            LlvmInstr::FAdd { result, lhs, rhs } => {
                write!(f, "  %{} = fadd double {}, {}", result, lhs, rhs)
            }
            LlvmInstr::FSub { result, lhs, rhs } => {
                write!(f, "  %{} = fsub double {}, {}", result, lhs, rhs)
            }
            LlvmInstr::FMul { result, lhs, rhs } => {
                write!(f, "  %{} = fmul double {}, {}", result, lhs, rhs)
            }
            LlvmInstr::FDiv { result, lhs, rhs } => {
                write!(f, "  %{} = fdiv double {}, {}", result, lhs, rhs)
            }
            LlvmInstr::And { result, lhs, rhs } => {
                write!(f, "  %{} = and i64 {}, {}", result, lhs, rhs)
            }
            LlvmInstr::Or { result, lhs, rhs } => {
                write!(f, "  %{} = or i64 {}, {}", result, lhs, rhs)
            }
            LlvmInstr::Xor { result, lhs, rhs } => {
                write!(f, "  %{} = xor i64 {}, {}", result, lhs, rhs)
            }
            LlvmInstr::Shl { result, lhs, rhs } => {
                write!(f, "  %{} = shl i64 {}, {}", result, lhs, rhs)
            }
            LlvmInstr::LShr { result, lhs, rhs } => {
                write!(f, "  %{} = lshr i64 {}, {}", result, lhs, rhs)
            }
            LlvmInstr::AShr { result, lhs, rhs } => {
                write!(f, "  %{} = ashr i64 {}, {}", result, lhs, rhs)
            }
            LlvmInstr::ICmp {
                result,
                pred,
                lhs,
                rhs,
            } => {
                write!(f, "  %{} = icmp {} i64 {}, {}", result, pred, lhs, rhs)
            }
            LlvmInstr::FCmp {
                result,
                pred,
                lhs,
                rhs,
            } => {
                write!(f, "  %{} = fcmp {} double {}, {}", result, pred, lhs, rhs)
            }
            LlvmInstr::Br(label) => write!(f, "  br label %{}", label),
            LlvmInstr::CondBr {
                cond,
                true_,
                false_,
            } => {
                write!(f, "  br i1 {}, label %{}, label %{}", cond, true_, false_)
            }
            LlvmInstr::Ret(None) => write!(f, "  ret void"),
            LlvmInstr::Ret(Some((ty, val))) => write!(f, "  ret {} {}", ty, val),
            LlvmInstr::Unreachable => write!(f, "  unreachable"),
            LlvmInstr::Label(name) => write!(f, "{}:", name),
            LlvmInstr::Call {
                result,
                ret_ty,
                func,
                args,
            } => {
                if let Some(r) = result {
                    write!(f, "  %{} = call {} @{}(", r, ret_ty, func)?;
                } else {
                    write!(f, "  call {} @{}(", ret_ty, func)?;
                }
                for (i, (ty, val)) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{} {}", ty, val)?;
                }
                write!(f, ")")
            }
            LlvmInstr::GetElementPtr {
                result,
                base_ty,
                ptr,
                indices,
            } => {
                write!(
                    f,
                    "  %{} = getelementptr inbounds {}, ptr {}",
                    result, base_ty, ptr
                )?;
                for (ty, val) in indices {
                    write!(f, ", {} {}", ty, val)?;
                }
                Ok(())
            }
            LlvmInstr::BitCast {
                result,
                val,
                from_ty,
                to_ty,
            } => {
                write!(
                    f,
                    "  %{} = bitcast {} {} to {}",
                    result, from_ty, val, to_ty
                )
            }
            LlvmInstr::Phi {
                result,
                ty,
                incoming,
            } => {
                write!(f, "  %{} = phi {}", result, ty)?;
                for (i, (val, label)) in incoming.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, " [ {}, %{} ]", val, label)?;
                }
                Ok(())
            }
            LlvmInstr::Select {
                result,
                cond,
                true_val,
                false_val,
                ty,
            } => {
                write!(
                    f,
                    "  %{} = select i1 {}, {} {}, {} {}",
                    result, cond, ty, true_val, ty, false_val
                )
            }
            LlvmInstr::ExtractValue {
                result,
                agg,
                agg_ty,
                indices,
            } => {
                write!(f, "  %{} = extractvalue {} {}", result, agg_ty, agg)?;
                for idx in indices {
                    write!(f, ", {}", idx)?;
                }
                Ok(())
            }
            LlvmInstr::InsertValue {
                result,
                agg,
                agg_ty,
                val,
                val_ty,
                indices,
            } => {
                write!(
                    f,
                    "  %{} = insertvalue {} {}, {} {}",
                    result, agg_ty, agg, val_ty, val
                )?;
                for idx in indices {
                    write!(f, ", {}", idx)?;
                }
                Ok(())
            }
            LlvmInstr::ZExt {
                result,
                val,
                from_ty,
                to_ty,
            } => {
                write!(f, "  %{} = zext {} {} to {}", result, from_ty, val, to_ty)
            }
            LlvmInstr::SExt {
                result,
                val,
                from_ty,
                to_ty,
            } => {
                write!(f, "  %{} = sext {} {} to {}", result, from_ty, val, to_ty)
            }
            LlvmInstr::Trunc {
                result,
                val,
                from_ty,
                to_ty,
            } => {
                write!(f, "  %{} = trunc {} {} to {}", result, from_ty, val, to_ty)
            }
            LlvmInstr::FpToSI {
                result,
                val,
                from_ty,
                to_ty,
            } => {
                write!(f, "  %{} = fptosi {} {} to {}", result, from_ty, val, to_ty)
            }
            LlvmInstr::SIToFp {
                result,
                val,
                from_ty,
                to_ty,
            } => {
                write!(f, "  %{} = sitofp {} {} to {}", result, from_ty, val, to_ty)
            }
            LlvmInstr::FpExt {
                result,
                val,
                from_ty,
                to_ty,
            } => {
                write!(f, "  %{} = fpext {} {} to {}", result, from_ty, val, to_ty)
            }
            LlvmInstr::FpTrunc {
                result,
                val,
                from_ty,
                to_ty,
            } => {
                write!(
                    f,
                    "  %{} = fptrunc {} {} to {}",
                    result, from_ty, val, to_ty
                )
            }
        }
    }
}
