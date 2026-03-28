//! Rust-local plugin event bus primitives.
//!
//! This domain is intentionally plugin-local and does not attempt to replace
//! engine-backed `BSTEventSource<T>` or SKSE messaging. It is intended for
//! intra-plugin orchestration, especially when one system such as a hook or
//! gameplay callback wants to publish a mutable event payload to multiple Rust
//! subscribers.

use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::RefCell;
use core::fmt;

use spin::Mutex;

use super::source::{EventFlow, IntoEventFlow};

type DynBusCallback<T> = dyn FnMut(&mut T) -> EventFlow + 'static;

/// Subscriber execution priority for a plugin-local event bus.
///
/// Higher values run earlier. Equal priorities keep subscription order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct SubscriberPriority(i32);

impl SubscriberPriority {
    pub const FIRST: Self = Self(100);
    pub const EARLY: Self = Self(50);
    pub const NORMAL: Self = Self(0);
    pub const LATE: Self = Self(-50);
    pub const LAST: Self = Self(-100);

    #[inline(always)]
    pub const fn new(value: i32) -> Self {
        Self(value)
    }

    #[inline(always)]
    pub const fn value(self) -> i32 {
        self.0
    }
}

struct BusSubscriber<T> {
    id: u64,
    priority: SubscriberPriority,
    order: u64,
    callback: Rc<RefCell<Box<DynBusCallback<T>>>>,
}

impl<T> Clone for BusSubscriber<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            priority: self.priority,
            order: self.order,
            callback: Rc::clone(&self.callback),
        }
    }
}

#[derive(Default)]
struct BusState<T> {
    subscribers: Vec<BusSubscriber<T>>,
    pending_adds: Vec<BusSubscriber<T>>,
    pending_removes: Vec<u64>,
    dispatch_depth: usize,
    next_id: u64,
}

impl<T> BusState<T> {
    fn subscribe<F>(&mut self, priority: SubscriberPriority, callback: F) -> u64
    where
        F: FnMut(&mut T) -> EventFlow + 'static,
    {
        let id = self.next_id;
        self.next_id += 1;

        let subscriber = BusSubscriber {
            id,
            priority,
            order: id,
            callback: Rc::new(RefCell::new(Box::new(callback))),
        };

        if self.dispatch_depth == 0 {
            self.insert_subscriber(subscriber);
        } else {
            self.pending_adds.push(subscriber);
        }

        id
    }

    fn remove(&mut self, id: u64) -> bool {
        if self.dispatch_depth == 0 {
            self.remove_now(id)
        } else if self.remove_pending_add(id) {
            true
        } else if self.pending_removes.contains(&id) {
            false
        } else {
            self.pending_removes.push(id);
            true
        }
    }

    fn remove_pending_add(&mut self, id: u64) -> bool {
        let Some(index) = self
            .pending_adds
            .iter()
            .position(|subscriber| subscriber.id == id)
        else {
            return false;
        };

        self.pending_adds.remove(index);
        true
    }

    fn remove_now(&mut self, id: u64) -> bool {
        let Some(index) = self
            .subscribers
            .iter()
            .position(|subscriber| subscriber.id == id)
        else {
            return false;
        };

        self.subscribers.remove(index);
        true
    }

    fn insert_subscriber(&mut self, subscriber: BusSubscriber<T>) {
        let index = self
            .subscribers
            .iter()
            .position(|existing| subscriber_precedes(&subscriber, existing))
            .unwrap_or(self.subscribers.len());
        self.subscribers.insert(index, subscriber);
    }

    fn contains_pending_remove(&self, id: u64) -> bool {
        self.pending_removes.contains(&id)
    }

    fn begin_dispatch(&mut self) -> Vec<BusSubscriber<T>> {
        self.dispatch_depth += 1;
        self.subscribers.clone()
    }

    fn end_dispatch(&mut self) {
        self.dispatch_depth -= 1;
        if self.dispatch_depth == 0 {
            let pending_removes = core::mem::take(&mut self.pending_removes);
            for id in pending_removes {
                let _ = self.remove_now(id);
            }

            let pending_adds = core::mem::take(&mut self.pending_adds);
            for subscriber in pending_adds {
                self.insert_subscriber(subscriber);
            }
        }
    }
}

#[inline(always)]
fn subscriber_precedes<T>(left: &BusSubscriber<T>, right: &BusSubscriber<T>) -> bool {
    left.priority > right.priority || (left.priority == right.priority && left.order < right.order)
}

/// Synchronous Rust-local event bus with mutable payload dispatch.
///
/// Subscribers are invoked in descending priority order. Equal priorities keep
/// subscription order. If one subscriber triggers a nested publish while it is
/// already running, that subscriber is skipped in the nested pass instead of
/// being re-entered recursively.
pub struct Bus<T> {
    state: Mutex<BusState<T>>,
}

struct DispatchGuard<'a, T> {
    bus: &'a Bus<T>,
}

impl<T> Drop for DispatchGuard<'_, T> {
    fn drop(&mut self) {
        self.bus.state.lock().end_dispatch();
    }
}

/// Result of publishing an owned payload through a [`Bus`].
pub struct PublishResult<T> {
    event: T,
    flow: EventFlow,
}

impl<T> PublishResult<T> {
    #[inline(always)]
    pub const fn flow(&self) -> EventFlow {
        self.flow
    }

    #[inline(always)]
    pub fn event(&self) -> &T {
        &self.event
    }

    #[inline(always)]
    pub fn event_mut(&mut self) -> &mut T {
        &mut self.event
    }

    #[inline(always)]
    pub fn into_event(self) -> T {
        self.event
    }

    #[inline(always)]
    pub fn into_parts(self) -> (T, EventFlow) {
        (self.event, self.flow)
    }
}

impl<T> Default for Bus<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Bus<T> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            state: Mutex::new(BusState {
                subscribers: Vec::new(),
                pending_adds: Vec::new(),
                pending_removes: Vec::new(),
                dispatch_depth: 0,
                next_id: 0,
            }),
        }
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.state.lock().subscribers.is_empty()
    }

    #[inline(always)]
    pub fn subscribe<F, R>(&self, callback: F) -> BusSubscription<'_, T>
    where
        F: FnMut(&mut T) -> R + 'static,
        R: IntoEventFlow,
    {
        self.subscribe_with_priority(SubscriberPriority::NORMAL, callback)
    }

    pub fn subscribe_with_priority<F, R>(
        &self,
        priority: SubscriberPriority,
        callback: F,
    ) -> BusSubscription<'_, T>
    where
        F: FnMut(&mut T) -> R + 'static,
        R: IntoEventFlow,
    {
        let mut callback = callback;
        let id = self
            .state
            .lock()
            .subscribe(priority, move |event| callback(event).into_event_flow());
        BusSubscription {
            bus: self,
            id,
            active: true,
        }
    }

    #[inline(always)]
    pub fn subscribe_first<F, R>(&self, callback: F) -> BusSubscription<'_, T>
    where
        F: FnMut(&mut T) -> R + 'static,
        R: IntoEventFlow,
    {
        self.subscribe_with_priority(SubscriberPriority::FIRST, callback)
    }

    #[inline(always)]
    pub fn subscribe_early<F, R>(&self, callback: F) -> BusSubscription<'_, T>
    where
        F: FnMut(&mut T) -> R + 'static,
        R: IntoEventFlow,
    {
        self.subscribe_with_priority(SubscriberPriority::EARLY, callback)
    }

    #[inline(always)]
    pub fn subscribe_late<F, R>(&self, callback: F) -> BusSubscription<'_, T>
    where
        F: FnMut(&mut T) -> R + 'static,
        R: IntoEventFlow,
    {
        self.subscribe_with_priority(SubscriberPriority::LATE, callback)
    }

    #[inline(always)]
    pub fn subscribe_last<F, R>(&self, callback: F) -> BusSubscription<'_, T>
    where
        F: FnMut(&mut T) -> R + 'static,
        R: IntoEventFlow,
    {
        self.subscribe_with_priority(SubscriberPriority::LAST, callback)
    }

    /// Publish one mutable payload to all currently visible subscribers.
    ///
    /// Subscribers added during this dispatch become visible on the next
    /// publish. Subscribers removed during this dispatch are skipped for the
    /// remaining tail of the current publish.
    pub fn publish(&self, event: &mut T) -> EventFlow {
        let snapshot = self.state.lock().begin_dispatch();
        let _guard = DispatchGuard { bus: self };
        let mut result = EventFlow::Continue;

        for subscriber in snapshot {
            {
                let state = self.state.lock();
                if state.contains_pending_remove(subscriber.id) {
                    continue;
                }
            }

            let Ok(mut callback) = subscriber.callback.try_borrow_mut() else {
                continue;
            };

            result = (*callback)(event);
            if result.is_stop() {
                break;
            }
        }
        result
    }

    #[inline(always)]
    pub fn publish_owned(&self, mut event: T) -> PublishResult<T> {
        let flow = self.publish(&mut event);
        PublishResult { event, flow }
    }

    #[inline(always)]
    pub fn publish_default(&self) -> PublishResult<T>
    where
        T: Default,
    {
        self.publish_owned(T::default())
    }

    #[inline(always)]
    pub fn publish_with<F>(&self, init: F) -> PublishResult<T>
    where
        T: Default,
        F: FnOnce(&mut T),
    {
        let mut event = T::default();
        init(&mut event);
        self.publish_owned(event)
    }

    fn remove_subscription(&self, id: u64) -> bool {
        self.state.lock().remove(id)
    }
}

impl<T> fmt::Debug for Bus<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = self.state.lock();
        f.debug_struct("Bus")
            .field("subscribers", &state.subscribers.len())
            .field("pending_adds", &state.pending_adds.len())
            .field("pending_removes", &state.pending_removes.len())
            .field("dispatch_depth", &state.dispatch_depth)
            .finish()
    }
}

/// RAII subscription handle for [`Bus`].
pub struct BusSubscription<'a, T> {
    bus: &'a Bus<T>,
    id: u64,
    active: bool,
}

impl<T> BusSubscription<'_, T> {
    #[inline(always)]
    pub const fn id(&self) -> u64 {
        self.id
    }

    #[inline(always)]
    pub const fn is_active(&self) -> bool {
        self.active
    }

    pub fn unsubscribe(&mut self) -> bool {
        if !self.active {
            return false;
        }

        let removed = self.bus.remove_subscription(self.id);
        self.active = false;
        removed
    }
}

impl<T> Drop for BusSubscription<'_, T> {
    fn drop(&mut self) {
        let _ = self.unsubscribe();
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use alloc::rc::Rc;
    use alloc::vec;
    use core::cell::{Cell, RefCell};
    use std::panic::{AssertUnwindSafe, catch_unwind};

    use super::*;

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
                assert_eq!(*value, 4);
                EventFlow::Continue
            });

            let mut payload = 1;
            assert_eq!(bus.publish(&mut payload), EventFlow::Continue);
            assert_eq!(payload, 4);
        }

        assert_eq!(*order.borrow(), vec!["early", "late", "normal"]);
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
}
