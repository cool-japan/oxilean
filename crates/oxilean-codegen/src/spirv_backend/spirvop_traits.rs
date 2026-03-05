//! # SpirVOp - Trait Implementations
//!
//! This module contains trait implementations for `SpirVOp`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{ExecutionMode, MemoryModel, SpirVOp};
use std::fmt;

impl fmt::Display for SpirVOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpirVOp::Variable(sc) => write!(f, "OpVariable({})", sc),
            SpirVOp::Load => write!(f, "OpLoad"),
            SpirVOp::Store => write!(f, "OpStore"),
            SpirVOp::AccessChain => write!(f, "OpAccessChain"),
            SpirVOp::CopyObject => write!(f, "OpCopyObject"),
            SpirVOp::IAdd => write!(f, "OpIAdd"),
            SpirVOp::ISub => write!(f, "OpISub"),
            SpirVOp::IMul => write!(f, "OpIMul"),
            SpirVOp::SDiv => write!(f, "OpSDiv"),
            SpirVOp::UDiv => write!(f, "OpUDiv"),
            SpirVOp::SMod => write!(f, "OpSMod"),
            SpirVOp::UMod => write!(f, "OpUMod"),
            SpirVOp::SNegate => write!(f, "OpSNegate"),
            SpirVOp::FAdd => write!(f, "OpFAdd"),
            SpirVOp::FSub => write!(f, "OpFSub"),
            SpirVOp::FMul => write!(f, "OpFMul"),
            SpirVOp::FDiv => write!(f, "OpFDiv"),
            SpirVOp::FMod => write!(f, "OpFMod"),
            SpirVOp::FNegate => write!(f, "OpFNegate"),
            SpirVOp::FRem => write!(f, "OpFRem"),
            SpirVOp::IEqual => write!(f, "OpIEqual"),
            SpirVOp::INotEqual => write!(f, "OpINotEqual"),
            SpirVOp::SLessThan => write!(f, "OpSLessThan"),
            SpirVOp::SLessThanEqual => write!(f, "OpSLessThanEqual"),
            SpirVOp::SGreaterThan => write!(f, "OpSGreaterThan"),
            SpirVOp::ULessThan => write!(f, "OpULessThan"),
            SpirVOp::FOrdEqual => write!(f, "OpFOrdEqual"),
            SpirVOp::FOrdLessThan => write!(f, "OpFOrdLessThan"),
            SpirVOp::FOrdGreaterThan => write!(f, "OpFOrdGreaterThan"),
            SpirVOp::LogicalAnd => write!(f, "OpLogicalAnd"),
            SpirVOp::LogicalOr => write!(f, "OpLogicalOr"),
            SpirVOp::LogicalNot => write!(f, "OpLogicalNot"),
            SpirVOp::LogicalEqual => write!(f, "OpLogicalEqual"),
            SpirVOp::BitwiseAnd => write!(f, "OpBitwiseAnd"),
            SpirVOp::BitwiseOr => write!(f, "OpBitwiseOr"),
            SpirVOp::BitwiseXor => write!(f, "OpBitwiseXor"),
            SpirVOp::Not => write!(f, "OpNot"),
            SpirVOp::ShiftLeftLogical => write!(f, "OpShiftLeftLogical"),
            SpirVOp::ShiftRightLogical => write!(f, "OpShiftRightLogical"),
            SpirVOp::ShiftRightArithmetic => write!(f, "OpShiftRightArithmetic"),
            SpirVOp::ConvertFToS => write!(f, "OpConvertFToS"),
            SpirVOp::ConvertFToU => write!(f, "OpConvertFToU"),
            SpirVOp::ConvertSToF => write!(f, "OpConvertSToF"),
            SpirVOp::ConvertUToF => write!(f, "OpConvertUToF"),
            SpirVOp::FConvert => write!(f, "OpFConvert"),
            SpirVOp::SConvert => write!(f, "OpSConvert"),
            SpirVOp::UConvert => write!(f, "OpUConvert"),
            SpirVOp::Bitcast => write!(f, "OpBitcast"),
            SpirVOp::CompositeConstruct => write!(f, "OpCompositeConstruct"),
            SpirVOp::CompositeExtract => write!(f, "OpCompositeExtract"),
            SpirVOp::CompositeInsert => write!(f, "OpCompositeInsert"),
            SpirVOp::VectorShuffle => write!(f, "OpVectorShuffle"),
            SpirVOp::VectorExtractDynamic => write!(f, "OpVectorExtractDynamic"),
            SpirVOp::VectorInsertDynamic => write!(f, "OpVectorInsertDynamic"),
            SpirVOp::MatrixTimesVector => write!(f, "OpMatrixTimesVector"),
            SpirVOp::VectorTimesMatrix => write!(f, "OpVectorTimesMatrix"),
            SpirVOp::MatrixTimesMatrix => write!(f, "OpMatrixTimesMatrix"),
            SpirVOp::MatrixTimesScalar => write!(f, "OpMatrixTimesScalar"),
            SpirVOp::Dot => write!(f, "OpDot"),
            SpirVOp::OuterProduct => write!(f, "OpOuterProduct"),
            SpirVOp::Transpose => write!(f, "OpTranspose"),
            SpirVOp::Label => write!(f, "OpLabel"),
            SpirVOp::Branch => write!(f, "OpBranch"),
            SpirVOp::BranchConditional => write!(f, "OpBranchConditional"),
            SpirVOp::Switch => write!(f, "OpSwitch"),
            SpirVOp::Return => write!(f, "OpReturn"),
            SpirVOp::ReturnValue => write!(f, "OpReturnValue"),
            SpirVOp::Unreachable => write!(f, "OpUnreachable"),
            SpirVOp::Phi => write!(f, "OpPhi"),
            SpirVOp::LoopMerge => write!(f, "OpLoopMerge"),
            SpirVOp::SelectionMerge => write!(f, "OpSelectionMerge"),
            SpirVOp::Function => write!(f, "OpFunction"),
            SpirVOp::FunctionParameter => write!(f, "OpFunctionParameter"),
            SpirVOp::FunctionEnd => write!(f, "OpFunctionEnd"),
            SpirVOp::FunctionCall => write!(f, "OpFunctionCall"),
            SpirVOp::ImageSampleImplicitLod => write!(f, "OpImageSampleImplicitLod"),
            SpirVOp::ImageSampleExplicitLod => write!(f, "OpImageSampleExplicitLod"),
            SpirVOp::ImageLoad => write!(f, "OpImageLoad"),
            SpirVOp::ImageStore => write!(f, "OpImageStore"),
            SpirVOp::AtomicLoad => write!(f, "OpAtomicLoad"),
            SpirVOp::AtomicStore => write!(f, "OpAtomicStore"),
            SpirVOp::AtomicIAdd => write!(f, "OpAtomicIAdd"),
            SpirVOp::AtomicISub => write!(f, "OpAtomicISub"),
            SpirVOp::AtomicCompareExchange => write!(f, "OpAtomicCompareExchange"),
            SpirVOp::ExtInstGlsl(op) => write!(f, "OpExtInstGlsl({:?})", op),
            SpirVOp::Capability(cap) => write!(f, "OpCapability({:?})", cap),
            SpirVOp::Extension(ext) => write!(f, "OpExtension(\"{}\")", ext),
            SpirVOp::ExtInstImport(set) => write!(f, "OpExtInstImport(\"{}\")", set),
            SpirVOp::MemoryModel(addr, mem) => {
                write!(f, "OpMemoryModel({:?}, {:?})", addr, mem)
            }
            SpirVOp::EntryPoint(model, name) => {
                write!(f, "OpEntryPoint({:?}, \"{}\")", model, name)
            }
            SpirVOp::ExecutionMode(mode) => write!(f, "OpExecutionMode({:?})", mode),
            SpirVOp::Decorate(deco) => write!(f, "OpDecorate({:?})", deco),
            SpirVOp::MemberDecorate(idx, deco) => {
                write!(f, "OpMemberDecorate({}, {:?})", idx, deco)
            }
            SpirVOp::Name(n) => write!(f, "OpName(\"{}\")", n),
            SpirVOp::Constant(v) => write!(f, "OpConstant({})", v),
            SpirVOp::ConstantComposite => write!(f, "OpConstantComposite"),
            SpirVOp::ConstantTrue => write!(f, "OpConstantTrue"),
            SpirVOp::ConstantFalse => write!(f, "OpConstantFalse"),
            SpirVOp::TypeForwardPointer(sc) => write!(f, "OpTypeForwardPointer({})", sc),
            SpirVOp::ControlBarrier => write!(f, "OpControlBarrier"),
            SpirVOp::MemoryBarrier => write!(f, "OpMemoryBarrier"),
        }
    }
}
