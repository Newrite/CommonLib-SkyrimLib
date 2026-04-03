use alloc::rc::Rc;
use core::cell::Cell;

use crate::re::BSTEventSource;

use super::*;

#[test]
fn null_source_is_rejected() {
    let result = unsafe { subscribe::<u32, _, _>(core::ptr::null_mut(), |_| EventFlow::Continue) };
    assert!(matches!(result, Err(EventInstallError::SourceUnavailable)));
}

#[test]
fn subscription_receives_events_and_unsubscribes_on_drop() {
    let mut source = BSTEventSource::<u32>::new();
    let seen = Rc::new(Cell::new(0u32));
    let last = Rc::new(Cell::new(0u32));

    {
        let seen_ref = Rc::clone(&seen);
        let last_ref = Rc::clone(&last);
        let _subscription = unsafe {
            subscribe(&mut source, move |event| {
                let Some(event) = event else {
                    return EventFlow::Continue;
                };

                seen_ref.set(seen_ref.get() + 1);
                last_ref.set(*event);
                EventFlow::Continue
            })
        }
        .expect("subscription should install");

        let value = 42u32;
        unsafe {
            source.send_event(&value);
        }
    }

    assert_eq!(seen.get(), 1);
    assert_eq!(last.get(), 42);
    assert!(unsafe { source.sinks.as_slice() }.is_empty());
}

#[test]
fn source_ref_installs_without_raw_pointer_plumbing() {
    let mut source = BSTEventSource::<u32>::new();
    let seen = Rc::new(Cell::new(0u32));

    {
        let source_ref = EventSourceRef::new(&mut source);
        let source_ptr = source_ref.as_ptr();
        let seen_ref = Rc::clone(&seen);
        let _subscription = source_ref
            .subscribe(move |event| {
                if let Some(event) = event {
                    seen_ref.set(*event);
                }
            })
            .expect("source_ref should install");

        let value = 7u32;
        unsafe {
            (*source_ptr).send_event(&value);
        }
    }

    assert_eq!(seen.get(), 7);
    assert!(unsafe { source.sinks.as_slice() }.is_empty());
}

#[test]
fn event_source_ext_subscribes_through_owner_borrow() {
    let mut source = BSTEventSource::<u32>::new();
    let seen = Rc::new(Cell::new(0u32));

    {
        let source_ptr = core::ptr::from_mut(&mut source);
        let seen_ref = Rc::clone(&seen);
        let _subscription = source
            .subscribe_sdk(move |event| {
                if let Some(event) = event {
                    seen_ref.set(*event);
                }
            })
            .expect("extension trait should install");

        let value = 11u32;
        unsafe {
            (*source_ptr).send_event(&value);
        }
    }

    assert_eq!(seen.get(), 11);
    assert!(unsafe { source.sinks.as_slice() }.is_empty());
}
