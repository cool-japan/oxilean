//! Tests pinning what `MakeClosure` captures and what it leaves on the stack.

use super::super::types::{BytecodeChunk, Interpreter, Opcode, StackValue};

fn nat(n: u64) -> StackValue {
    StackValue::Nat(n)
}

#[test]
fn captures_the_top_values_in_stack_order() {
    let mut interp = Interpreter::new();
    let chunk = BytecodeChunk::new("make_closure");
    for n in 1..=5 {
        interp.push(nat(n));
    }
    let cont = interp
        .execute_op(&Opcode::MakeClosure(3), &chunk)
        .expect("five values cover three captures");
    assert!(cont);
    assert_eq!(interp.stack.len(), 3);
    assert!(matches!(interp.stack[0], StackValue::Nat(1)));
    assert!(matches!(interp.stack[1], StackValue::Nat(2)));
    match &interp.stack[2] {
        StackValue::Closure { code, env } => {
            assert!(code.is_empty());
            let captured: Vec<u64> = env
                .iter()
                .map(|v| match v {
                    StackValue::Nat(n) => *n,
                    other => panic!("unexpected capture {:?}", other),
                })
                .collect();
            assert_eq!(captured, vec![3, 4, 5]);
        }
        other => panic!("expected a closure, got {:?}", other),
    }
}

#[test]
fn captures_nothing_and_everything() {
    let mut interp = Interpreter::new();
    let chunk = BytecodeChunk::new("make_closure");
    interp
        .execute_op(&Opcode::MakeClosure(0), &chunk)
        .expect("zero captures need no values");
    assert!(matches!(&interp.stack[0], StackValue::Closure { env, .. } if env.is_empty()));
    interp.stack.clear();
    interp.push(nat(8));
    interp.push(nat(9));
    interp
        .execute_op(&Opcode::MakeClosure(2), &chunk)
        .expect("two values cover two captures");
    assert_eq!(interp.stack.len(), 1);
    assert!(matches!(&interp.stack[0], StackValue::Closure { env, .. } if env.len() == 2));
}

#[test]
fn too_few_values_is_an_error_that_leaves_the_stack_alone() {
    let mut interp = Interpreter::new();
    let chunk = BytecodeChunk::new("make_closure");
    interp.push(nat(1));
    interp.push(nat(2));
    let err = interp
        .execute_op(&Opcode::MakeClosure(3), &chunk)
        .expect_err("two values cannot cover three captures");
    assert_eq!(err, "MakeClosure: need 3 captures, have 2");
    assert_eq!(interp.stack.len(), 2);
}
