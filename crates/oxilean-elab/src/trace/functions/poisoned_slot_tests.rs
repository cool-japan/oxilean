//! Tests for emptying a tracer slot whose lock a panicking holder poisoned.
//! They use a local slot, never the global one, so no other test sees the
//! poisoned lock.

use super::super::types::{ElabTracer, TraceLevel};
use super::clear_tracer_slot;
use std::sync::{Arc, Mutex};

#[test]
fn clearing_a_slot_empties_it() {
    let slot = Mutex::new(Some(ElabTracer::new()));
    clear_tracer_slot(&slot);
    let guard = slot.lock().expect("this slot was never poisoned");
    assert!(guard.is_none());
}

#[test]
fn clearing_a_poisoned_slot_empties_it_without_panicking() {
    let slot = Arc::new(Mutex::new(Some({
        let mut tracer = ElabTracer::new();
        tracer.set_level(TraceLevel::Info);
        tracer
    })));
    let holder = Arc::clone(&slot);
    let joined = std::thread::spawn(move || {
        let _guard = holder.lock().expect("the first lock succeeds");
        panic!("holder panics while holding the tracer lock");
    })
    .join();
    assert!(joined.is_err());
    assert!(slot.is_poisoned());
    clear_tracer_slot(&slot);
    let guard = slot
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    assert!(guard.is_none());
}
