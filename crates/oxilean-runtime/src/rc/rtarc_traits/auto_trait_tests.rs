//! `RtArc<T>` is `Send` and `Sync` exactly when `T: Send + Sync`.

use crate::rc::RtArc;
use std::marker::PhantomData;

/// `is_send` / `is_sync` resolve to the inherent methods (answering `true`)
/// when `T` has the trait, and to the traits' defaults (answering `false`)
/// otherwise.
struct Probe<T: ?Sized>(PhantomData<T>);
trait NotSend {
    fn is_send(&self) -> bool {
        false
    }
}
trait NotSync {
    fn is_sync(&self) -> bool {
        false
    }
}
impl<T: ?Sized> NotSend for Probe<T> {}
impl<T: ?Sized> NotSync for Probe<T> {}
impl<T: ?Sized + Send> Probe<T> {
    fn is_send(&self) -> bool {
        true
    }
}
impl<T: ?Sized + Sync> Probe<T> {
    fn is_sync(&self) -> bool {
        true
    }
}

macro_rules! send_sync {
    ($t:ty) => {
        (
            Probe::<$t>(PhantomData).is_send(),
            Probe::<$t>(PhantomData).is_sync(),
        )
    };
}

#[test]
fn rtarc_is_send_and_sync_exactly_when_its_content_is() {
    assert_eq!(send_sync!(RtArc<u64>), (true, true));
    assert_eq!(send_sync!(RtArc<String>), (true, true));
    assert_eq!(send_sync!(RtArc<std::sync::Mutex<Vec<u8>>>), (true, true));
    // `Send` but not `Sync`, `Sync` but not `Send`, and neither.
    assert_eq!(send_sync!(std::cell::Cell<u8>), (true, false));
    assert_eq!(send_sync!(RtArc<std::cell::Cell<u8>>), (false, false));
    assert_eq!(
        send_sync!(std::sync::MutexGuard<'static, u8>),
        (false, true)
    );
    assert_eq!(
        send_sync!(RtArc<std::sync::MutexGuard<'static, u8>>),
        (false, false)
    );
    assert_eq!(send_sync!(RtArc<std::rc::Rc<u8>>), (false, false));
    assert_eq!(send_sync!(RtArc<*const u8>), (false, false));
}
