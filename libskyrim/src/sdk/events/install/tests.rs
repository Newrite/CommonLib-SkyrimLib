use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::Cell;

use crate::re::BSTEventSource;
use crate::sdk::events::{Bus, EventFlow};

use super::*;

#[test]
fn raw_source_subscription_lives_until_batch_drop() {
    let seen = Rc::new(Cell::new(0u32));
    let mut source = BSTEventSource::<u32>::new();

    {
        let mut batch = EventBatch::new();
        let seen_ref = Rc::clone(&seen);
        unsafe {
            batch
                .source("raw", &mut source, move |event| {
                    if let Some(event) = event {
                        seen_ref.set(seen_ref.get() + *event);
                    }
                    EventFlow::Continue
                })
                .expect("raw source should install");
        }

        let value = 2u32;
        unsafe { source.send_event(&value) };
        assert_eq!(seen.get(), 2);
        assert_eq!(batch.len(), 1);
        assert_eq!(batch.names().collect::<Vec<_>>(), ["raw"]);
    }

    let value = 3u32;
    unsafe { source.send_event(&value) };
    assert_eq!(seen.get(), 2);
}

#[test]
fn bus_subscription_lives_until_batch_drop() {
    let bus = Bus::<u32>::new();
    let seen = Rc::new(Cell::new(0u32));

    {
        let mut batch = EventBatch::new();
        let seen_ref = Rc::clone(&seen);
        batch
            .bus("local", &bus, move |value| {
                seen_ref.set(seen_ref.get() + *value);
                EventFlow::Continue
            })
            .expect("bus subscription should install");

        let mut payload = 5;
        let _ = bus.publish(&mut payload);
        assert_eq!(seen.get(), 5);
    }

    let mut payload = 7;
    let _ = bus.publish(&mut payload);
    assert_eq!(seen.get(), 5);
}
