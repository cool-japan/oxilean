//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

/// Append the §5 axioms into the environment.
///
/// Call after `register_repr_extended` to add stack-protection, DWARF,
/// register file, memory model, and ABI axioms.
pub fn register_repr_extended2(env: &mut Environment) -> Result<(), String> {
    let decls: &[(&str, Expr)] = &[
        ("StackCanary", axiom_stack_canary_ty()),
        ("CanaryValue", axiom_canary_value_ty()),
        ("CanaryValid", axiom_canary_valid_ty()),
        ("ShadowStack", axiom_shadow_stack_ty()),
        ("ShadowStackPush", axiom_shadow_stack_push_ty()),
        ("ShadowStackPop", axiom_shadow_stack_pop_ty()),
        ("DwarfEntry", axiom_dwarf_entry_ty()),
        ("DwarfTag", axiom_dwarf_tag_ty()),
        ("UnwindTable", axiom_unwind_table_ty()),
        ("UnwindRule", axiom_unwind_rule_ty()),
        ("RegisterFile", axiom_register_file_ty()),
        ("RegValue", axiom_reg_value_ty()),
        ("RegWrite", axiom_reg_write_ty()),
        ("RegReadAfterWrite", axiom_reg_read_after_write_ty()),
        ("NumRegisters", axiom_num_registers_ty()),
        ("MemoryModel", axiom_memory_model_ty()),
        ("IsSequentiallyConsistent", axiom_is_sc_ty()),
        ("IsTotalStoreOrder", axiom_is_tso_ty()),
        ("IsRelaxed", axiom_is_relaxed_ty()),
        ("MemoryFence", axiom_memory_fence_ty()),
        ("ABI", axiom_abi_ty()),
        ("ABIIntSize", axiom_abi_int_size_ty()),
        ("ABILongSize", axiom_abi_long_size_ty()),
        ("ABIPtrSize", axiom_abi_ptr_size_ty()),
        ("ABIStackAlignment", axiom_abi_stack_alignment_ty()),
        ("ABIArgRegCount", axiom_abi_arg_reg_count_ty()),
    ];
    for (name, ty) in decls {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}
