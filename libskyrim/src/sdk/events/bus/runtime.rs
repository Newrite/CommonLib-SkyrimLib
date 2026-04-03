use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::RefCell;
use core::fmt;

use spin::Mutex;

use super::types::{PublishResult, SubscriberPriority};
use crate::sdk::events::source::{EventFlow, IntoEventFlow};

type DynBusCallback<T> = dyn FnMut(&mut T) -> EventFlow + 'static;

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
        PublishResult::new(event, flow)
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
