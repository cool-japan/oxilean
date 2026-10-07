//! Linear-scan register allocation (`RegisterAllocator`) and the two-way case
//! lowering of `NativeBackend`.

use super::*;
use crate::lcnf::LcnfLit;

fn v(n: u32) -> Register {
    Register::virt(n)
}

fn load(dst: u32, value: i64) -> NativeInst {
    NativeInst::LoadImm {
        dst: v(dst),
        ty: NativeType::I64,
        value,
    }
}

fn add(dst: u32, lhs: u32, rhs: u32) -> NativeInst {
    NativeInst::Add {
        dst: v(dst),
        ty: NativeType::I64,
        lhs: NativeValue::Reg(v(lhs)),
        rhs: NativeValue::Reg(v(rhs)),
    }
}

fn func_with(instructions: Vec<NativeInst>) -> NativeFunc {
    let mut func = NativeFunc::new("f", Vec::new(), NativeType::Void);
    let mut block = BasicBlock::new(BlockId(0));
    for inst in instructions {
        block.push_inst(inst);
    }
    func.blocks.push(block);
    func
}

/// v0 lives over positions 0..=3, v1 over 1..=2, v2 over 2..=3, v3 at 3.
fn nested_lifetimes() -> NativeFunc {
    func_with(vec![load(0, 1), load(1, 2), add(2, 1, 1), add(3, 0, 2)])
}

#[test]
fn enough_registers_means_no_spill() {
    let func = nested_lifetimes();
    let mut allocator = RegisterAllocator::new(8);
    let assignment = allocator.allocate(&func);
    assert_eq!(allocator.spill_count(), 0);
    for n in 0..4 {
        match assignment.get(&v(n)) {
            Some(phys) => assert!(phys.is_physical()),
            None => panic!("v{n} must be in a register"),
        }
    }
}

#[test]
fn without_registers_every_interval_is_spilled() {
    let func = nested_lifetimes();
    let mut allocator = RegisterAllocator::new(0);
    let assignment = allocator.allocate(&func);
    assert!(assignment.is_empty());
    assert_eq!(allocator.spill_count(), 4);
}

#[test]
fn an_outer_interval_that_ends_later_is_spilled_for_a_nested_one() {
    let func = nested_lifetimes();
    let mut allocator = RegisterAllocator::new(1);
    let assignment = allocator.allocate(&func);
    // v0 takes the only register first; v1 (1..=2) ends before it, so v0 is
    // evicted and v1 takes its register. v2 (2..=3) ends after v1 and is
    // spilled itself. v1 has ended when v3 (3..=3) starts, so v3 reuses the
    // register.
    assert_eq!(assignment.get(&v(1)), Some(&Register::phys(0)));
    assert_eq!(assignment.get(&v(3)), Some(&Register::phys(0)));
    assert_eq!(assignment.get(&v(0)), None);
    assert_eq!(assignment.get(&v(2)), None);
    assert_eq!(allocator.spill_count(), 2);
    let mut spilled: Vec<Register> = allocator
        .intervals
        .iter()
        .filter(|i| i.spill_slot.is_some())
        .map(|i| i.vreg)
        .collect();
    spilled.sort();
    assert_eq!(spilled, vec![v(0), v(2)]);
}

#[test]
fn a_register_is_reused_once_its_interval_has_ended() {
    // v0 lives over 0..=1 and v2 over 2..=3: v2 starts after v0 has ended and
    // takes its register. v1 (1..=1) and v3 (3..=3) start while the register is
    // held by an interval that ends no later than they do, and are spilled.
    let func = func_with(vec![load(0, 1), add(1, 0, 0), load(2, 3), add(3, 2, 2)]);
    let mut allocator = RegisterAllocator::new(1);
    let assignment = allocator.allocate(&func);
    assert_eq!(assignment.get(&v(0)), Some(&Register::phys(0)));
    assert_eq!(assignment.get(&v(2)), Some(&Register::phys(0)));
    assert_eq!(assignment.get(&v(1)), None);
    assert_eq!(assignment.get(&v(3)), None);
    assert_eq!(allocator.spill_count(), 2);
}

fn module_with_body(body: LcnfExpr) -> LcnfModule {
    LcnfModule {
        fun_decls: vec![LcnfFunDecl {
            name: "f".to_string(),
            original_name: None,
            params: vec![LcnfParam {
                id: LcnfVarId(0),
                name: "n".to_string(),
                ty: LcnfType::Nat,
                erased: false,
                borrowed: false,
            }],
            ret_type: LcnfType::Nat,
            body,
            is_recursive: false,
            is_lifted: false,
            inline_cost: 1,
        }],
        extern_decls: vec![],
        name: "m".to_string(),
        metadata: Default::default(),
    }
}

fn ret(n: u64) -> LcnfExpr {
    LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(n)))
}

fn alt(tag: u32, n: u64) -> LcnfAlt {
    LcnfAlt {
        ctor_name: format!("C{tag}"),
        ctor_tag: tag,
        params: Vec::new(),
        body: ret(n),
    }
}

fn compile_case(alts: Vec<LcnfAlt>, default: Option<LcnfExpr>) -> NativeFunc {
    let module = module_with_body(LcnfExpr::Case {
        scrutinee: LcnfVarId(0),
        scrutinee_ty: LcnfType::Object,
        alts,
        default: default.map(Box::new),
    });
    let mut backend = NativeBackend::default_backend();
    let mut native = backend.compile_module(&module);
    match native.functions.pop() {
        Some(func) => func,
        None => panic!("one function was compiled"),
    }
}

fn has_switch(func: &NativeFunc) -> bool {
    func.blocks.iter().any(|b| {
        b.instructions
            .iter()
            .chain(b.terminator.iter())
            .any(|i| matches!(i, NativeInst::Switch { .. }))
    })
}

fn has_cond_br(func: &NativeFunc) -> bool {
    func.blocks.iter().any(|b| {
        b.instructions
            .iter()
            .chain(b.terminator.iter())
            .any(|i| matches!(i, NativeInst::CondBr { .. }))
    })
}

#[test]
fn one_alternative_with_a_default_is_a_two_way_branch() {
    let func = compile_case(vec![alt(3, 1)], Some(ret(2)));
    assert!(has_cond_br(&func));
    assert!(!has_switch(&func));
    // entry, then, else and merge.
    assert_eq!(func.blocks.len(), 4);
}

#[test]
fn one_alternative_without_a_default_is_a_switch() {
    let func = compile_case(vec![alt(3, 1)], None);
    assert!(has_switch(&func));
    assert!(!has_cond_br(&func));
}

#[test]
fn several_alternatives_with_a_default_are_a_switch() {
    let func = compile_case(vec![alt(0, 1), alt(1, 2)], Some(ret(3)));
    assert!(has_switch(&func));
    assert!(!has_cond_br(&func));
}

#[test]
fn no_alternatives_return_the_default_or_nothing() {
    let with_default = compile_case(vec![], Some(ret(5)));
    assert!(!has_switch(&with_default) && !has_cond_br(&with_default));
    let without_default = compile_case(vec![], None);
    assert!(!has_switch(&without_default) && !has_cond_br(&without_default));
}
