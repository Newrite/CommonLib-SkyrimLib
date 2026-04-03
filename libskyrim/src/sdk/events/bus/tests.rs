extern crate std;

use alloc::rc::Rc;
use alloc::vec;
use alloc::vec::Vec;
use core::cell::{Cell, RefCell};
use std::panic::{AssertUnwindSafe, catch_unwind};

use super::*;
use crate::sdk::events::EventFlow;

#[test]
fn priorities_order_mutating_subscribers() {
    let bus = Bus::<i32>::new();
    let order = Rc::new(RefCell::new(Vec::new()));

    {
        let order_ref = Rc::clone(&order);
        let _late = bus.subscribe_with_priority(SubscriberPriority::LATE, move |value| {
            order_ref.borrow_mut().push("late");
            assert_eq!(*value, 2);
            *value *= 2;
            EventFlow::Continue
        });

        let order_ref = Rc::clone(&order);
        let _early = bus.subscribe_with_priority(SubscriberPriority::EARLY, move |value| {
            order_ref.borrow_mut().push("early");
            *value += 1;
            EventFlow::Continue
        });

        let order_ref = Rc::clone(&order);
        let _normal = bus.subscribe(move |value| {
            order_ref.borrow_mut().push("normal");
            assert_eq!(*value, 2);
            EventFlow::Continue
        });

        let mut payload = 1;
        assert_eq!(bus.publish(&mut payload), EventFlow::Continue);
        assert_eq!(payload, 4);
    }

    assert_eq!(*order.borrow(), vec!["early", "normal", "late"]);
}

#[test]
fn stop_skips_later_subscribers() {
    let bus = Bus::<u32>::new();
    let seen = Rc::new(Cell::new(0usize));

    let seen_ref = Rc::clone(&seen);
    let _first = bus.subscribe(move |_| {
        seen_ref.set(seen_ref.get() + 1);
        EventFlow::Stop
    });

    let seen_ref = Rc::clone(&seen);
    let _second = bus.subscribe(move |_| {
        seen_ref.set(seen_ref.get() + 1);
        EventFlow::Continue
    });

    let mut payload = 0;
    assert_eq!(bus.publish(&mut payload), EventFlow::Stop);
    assert_eq!(seen.get(), 1);
}

#[test]
fn dropped_subscription_is_removed() {
    let bus = Bus::<u32>::new();
    let seen = Rc::new(Cell::new(0usize));

    {
        let seen_ref = Rc::clone(&seen);
        let _subscription = bus.subscribe(move |_| {
            seen_ref.set(seen_ref.get() + 1);
            EventFlow::Continue
        });

        let mut payload = 0;
        let _ = bus.publish(&mut payload);
    }

    let mut payload = 0;
    let _ = bus.publish(&mut payload);
    assert_eq!(seen.get(), 1);
    assert!(bus.is_empty());
}

#[test]
fn publish_owned_returns_mutated_payload_and_flow() {
    let bus = Bus::<u32>::new();
    let _first = bus.subscribe_first(|value| {
        *value += 2;
    });
    let _last = bus.subscribe_last(|value| {
        *value *= 3;
        EventFlow::Stop
    });

    let published = bus.publish_owned(4);
    assert_eq!(published.flow(), EventFlow::Stop);
    assert_eq!(published.event(), &18);
    assert_eq!(published.into_parts(), (18, EventFlow::Stop));
}

#[test]
fn publish_with_builds_default_payload_before_dispatch() {
    let bus = Bus::<u32>::new();
    let _subscription = bus.subscribe(|value| {
        *value *= 2;
    });

    let published = bus.publish_with(|value| {
        *value = 6;
    });
    assert_eq!(published.into_parts(), (12, EventFlow::Continue));
}

#[test]
fn panic_during_dispatch_does_not_poison_future_publishes() {
    let bus = Bus::<u32>::new();
    let seen = Rc::new(Cell::new(0usize));

    let mut panic_subscription = bus.subscribe(|_| -> EventFlow {
        panic!("expected test panic");
    });

    let seen_ref = Rc::clone(&seen);
    let _after = bus.subscribe(move |_| {
        seen_ref.set(seen_ref.get() + 1);
        EventFlow::Continue
    });

    let mut payload = 0;
    let panic_result = catch_unwind(AssertUnwindSafe(|| {
        let _ = bus.publish(&mut payload);
    }));
    assert!(panic_result.is_err());

    assert!(panic_subscription.unsubscribe());

    let mut payload = 0;
    let _ = bus.publish(&mut payload);
    assert_eq!(seen.get(), 1);
}
