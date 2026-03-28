//! Batch installation helpers for SDK event registrations.

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::fmt;

use super::bus::{Bus, SubscriberPriority};
use super::input::InputEvents;
use super::skse::dispatchers::DispatcherEvent;
use super::skse::messages::{MessageKind, MessageRef};
use super::source::{self, EventInstallError, EventSourceRef, IntoEventFlow};
use super::{game, input, ui};
use crate::re::{BSTEventSource, ScriptEventSourceHolderEvent, UIEventSourceEvent};
use crate::sdk::core::{GameLifecyclePhase, LifecyclePhase, PluginLifecyclePhase};

trait EventKeepAlive {}

impl<T> EventKeepAlive for T {}

struct InstalledEvent<'a> {
    name: &'static str,
    _keepalive: Option<Box<dyn EventKeepAlive + 'a>>,
}

impl InstalledEvent<'_> {
    #[inline(always)]
    fn name(&self) -> &'static str {
        self.name
    }
}

/// Error reported when one event installation inside a batch fails.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EventBatchError {
    event_name: &'static str,
    error: EventInstallError,
}

impl EventBatchError {
    #[inline(always)]
    pub const fn new(event_name: &'static str, error: EventInstallError) -> Self {
        Self { event_name, error }
    }

    #[inline(always)]
    pub const fn event_name(self) -> &'static str {
        self.event_name
    }

    #[inline(always)]
    pub const fn error(self) -> EventInstallError {
        self.error
    }

    #[inline(always)]
    pub fn install_or_fatal(self) -> ! {
        self.error.install_or_fatal(self.event_name)
    }
}

impl fmt::Display for EventBatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "failed to install event `{}`: {:?}",
            self.event_name, self.error
        )
    }
}

impl core::error::Error for EventBatchError {}

pub type EventInstallFn = for<'a> fn(&mut EventBatch<'a>) -> EventBatchInstallResult;
pub type EventBatchInstallResult = Result<(), EventBatchError>;

#[derive(Clone, Copy)]
pub struct EventInstaller {
    name: &'static str,
    install: EventInstallFn,
}

impl EventInstaller {
    #[inline(always)]
    pub const fn new(name: &'static str, install: EventInstallFn) -> Self {
        Self { name, install }
    }

    #[inline(always)]
    pub const fn name(self) -> &'static str {
        self.name
    }

    #[inline(always)]
    pub const fn install_fn(self) -> EventInstallFn {
        self.install
    }

    #[inline(always)]
    pub fn try_install(self, batch: &mut EventBatch<'_>) -> EventBatchInstallResult {
        (self.install)(batch)
    }

    #[inline(always)]
    pub fn install_or_fatal(self, batch: &mut EventBatch<'_>) {
        if let Err(error) = self.try_install(batch) {
            error.install_or_fatal();
        }
    }
}

/// Owner container for installed event subscriptions.
///
/// Dropping the batch drops all retained RAII subscriptions and removes the
/// corresponding engine or local-bus listeners.
#[derive(Default)]
pub struct EventBatch<'a> {
    installed: Vec<InstalledEvent<'a>>,
}

impl<'a> EventBatch<'a> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            installed: Vec::new(),
        }
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.installed.len()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.installed.is_empty()
    }

    pub fn names(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.installed.iter().map(InstalledEvent::name)
    }

    #[inline(always)]
    pub fn message<F>(&mut self, name: &'static str, kind: MessageKind, callback: F) -> &mut Self
    where
        F: for<'b> FnMut(MessageRef<'b>) + 'static,
    {
        let _ = super::skse::messages::on(kind, callback);
        self.installed.push(InstalledEvent {
            name,
            _keepalive: None,
        });
        self
    }

    pub fn message_filtered<P, F>(
        &mut self,
        name: &'static str,
        kind: MessageKind,
        predicate: P,
        callback: F,
    ) -> &mut Self
    where
        P: for<'b> Fn(MessageRef<'b>) -> bool + 'static,
        F: for<'b> FnMut(MessageRef<'b>) + 'static,
    {
        let _ = super::skse::messages::on_filtered(kind, predicate, callback);
        self.installed.push(InstalledEvent {
            name,
            _keepalive: None,
        });
        self
    }

    #[inline(always)]
    pub fn message_sender_str<F>(
        &mut self,
        name: &'static str,
        kind: MessageKind,
        sender: &'static str,
        callback: F,
    ) -> &mut Self
    where
        F: for<'b> FnMut(MessageRef<'b>) + 'static,
    {
        let _ = super::skse::messages::on_sender_str(kind, sender, callback);
        self.installed.push(InstalledEvent {
            name,
            _keepalive: None,
        });
        self
    }

    pub fn message_plugin_phase<F>(
        &mut self,
        name: &'static str,
        phase: PluginLifecyclePhase,
        callback: F,
    ) -> &mut Self
    where
        F: for<'b> FnMut(MessageRef<'b>) + 'static,
    {
        let _ = super::skse::messages::on_plugin_phase(phase, callback);
        self.installed.push(InstalledEvent {
            name,
            _keepalive: None,
        });
        self
    }

    #[inline(always)]
    pub fn message_plugin_phase_sender_str<F>(
        &mut self,
        name: &'static str,
        phase: PluginLifecyclePhase,
        sender: &'static str,
        callback: F,
    ) -> &mut Self
    where
        F: for<'b> FnMut(MessageRef<'b>) + 'static,
    {
        let _ = super::skse::messages::on_plugin_phase_sender_str(phase, sender, callback);
        self.installed.push(InstalledEvent {
            name,
            _keepalive: None,
        });
        self
    }

    pub fn message_game_lifecycle<F>(
        &mut self,
        name: &'static str,
        phase: GameLifecyclePhase,
        callback: F,
    ) -> &mut Self
    where
        F: for<'b> FnMut(MessageRef<'b>) + 'static,
    {
        let _ = super::skse::messages::on_game_lifecycle(phase, callback);
        self.installed.push(InstalledEvent {
            name,
            _keepalive: None,
        });
        self
    }

    #[inline(always)]
    pub fn message_game_lifecycle_sender_str<F>(
        &mut self,
        name: &'static str,
        phase: GameLifecyclePhase,
        sender: &'static str,
        callback: F,
    ) -> &mut Self
    where
        F: for<'b> FnMut(MessageRef<'b>) + 'static,
    {
        let _ = super::skse::messages::on_game_lifecycle_sender_str(phase, sender, callback);
        self.installed.push(InstalledEvent {
            name,
            _keepalive: None,
        });
        self
    }

    pub fn message_lifecycle<F>(
        &mut self,
        name: &'static str,
        phase: LifecyclePhase,
        callback: F,
    ) -> &mut Self
    where
        F: for<'b> FnMut(MessageRef<'b>) + 'static,
    {
        let _ = super::skse::messages::on_lifecycle(phase, callback);
        self.installed.push(InstalledEvent {
            name,
            _keepalive: None,
        });
        self
    }

    #[inline(always)]
    pub fn message_lifecycle_sender_str<F>(
        &mut self,
        name: &'static str,
        phase: LifecyclePhase,
        sender: &'static str,
        callback: F,
    ) -> &mut Self
    where
        F: for<'b> FnMut(MessageRef<'b>) + 'static,
    {
        let _ = super::skse::messages::on_lifecycle_sender_str(phase, sender, callback);
        self.installed.push(InstalledEvent {
            name,
            _keepalive: None,
        });
        self
    }

    pub fn game<E, F, R>(
        &mut self,
        name: &'static str,
        callback: F,
    ) -> Result<&mut Self, EventBatchError>
    where
        E: ScriptEventSourceHolderEvent + 'static,
        F: FnMut(Option<&E>) -> R + 'static,
        R: IntoEventFlow,
    {
        let subscription = game::subscribe::<E, _, _>(callback)
            .map_err(|error| EventBatchError::new(name, error))?;
        Ok(self.push_keepalive(name, subscription))
    }

    pub fn prepend_game<E, F, R>(
        &mut self,
        name: &'static str,
        callback: F,
    ) -> Result<&mut Self, EventBatchError>
    where
        E: ScriptEventSourceHolderEvent + 'static,
        F: FnMut(Option<&E>) -> R + 'static,
        R: IntoEventFlow,
    {
        let subscription = game::prepend::<E, _, _>(callback)
            .map_err(|error| EventBatchError::new(name, error))?;
        Ok(self.push_keepalive(name, subscription))
    }

    pub fn ui<E, F, R>(
        &mut self,
        name: &'static str,
        callback: F,
    ) -> Result<&mut Self, EventBatchError>
    where
        E: UIEventSourceEvent + 'static,
        F: FnMut(Option<&E>) -> R + 'static,
        R: IntoEventFlow,
    {
        let subscription = ui::subscribe::<E, _, _>(callback)
            .map_err(|error| EventBatchError::new(name, error))?;
        Ok(self.push_keepalive(name, subscription))
    }

    pub fn prepend_ui<E, F, R>(
        &mut self,
        name: &'static str,
        callback: F,
    ) -> Result<&mut Self, EventBatchError>
    where
        E: UIEventSourceEvent + 'static,
        F: FnMut(Option<&E>) -> R + 'static,
        R: IntoEventFlow,
    {
        let subscription =
            ui::prepend::<E, _, _>(callback).map_err(|error| EventBatchError::new(name, error))?;
        Ok(self.push_keepalive(name, subscription))
    }

    pub fn dispatcher<E, F, R>(
        &mut self,
        name: &'static str,
        callback: F,
    ) -> Result<&mut Self, EventBatchError>
    where
        E: DispatcherEvent + 'static,
        F: FnMut(Option<&E>) -> R + 'static,
        R: IntoEventFlow,
    {
        let subscription = super::skse::dispatchers::subscribe::<E, _, _>(callback)
            .map_err(|error| EventBatchError::new(name, error))?;
        Ok(self.push_keepalive(name, subscription))
    }

    pub fn prepend_dispatcher<E, F, R>(
        &mut self,
        name: &'static str,
        callback: F,
    ) -> Result<&mut Self, EventBatchError>
    where
        E: DispatcherEvent + 'static,
        F: FnMut(Option<&E>) -> R + 'static,
        R: IntoEventFlow,
    {
        let subscription = super::skse::dispatchers::prepend::<E, _, _>(callback)
            .map_err(|error| EventBatchError::new(name, error))?;
        Ok(self.push_keepalive(name, subscription))
    }

    pub fn input<F, R>(
        &mut self,
        name: &'static str,
        callback: F,
    ) -> Result<&mut Self, EventBatchError>
    where
        F: for<'b> FnMut(InputEvents<'b>) -> R + 'static,
        R: IntoEventFlow,
    {
        let subscription =
            input::subscribe(callback).map_err(|error| EventBatchError::new(name, error))?;
        Ok(self.push_keepalive(name, subscription))
    }

    pub fn prepend_input<F, R>(
        &mut self,
        name: &'static str,
        callback: F,
    ) -> Result<&mut Self, EventBatchError>
    where
        F: for<'b> FnMut(InputEvents<'b>) -> R + 'static,
        R: IntoEventFlow,
    {
        let subscription =
            input::prepend(callback).map_err(|error| EventBatchError::new(name, error))?;
        Ok(self.push_keepalive(name, subscription))
    }

    /// # Safety
    /// `source_ptr` must remain valid until this batch is dropped.
    pub unsafe fn source<T, F, R>(
        &mut self,
        name: &'static str,
        source_ptr: *mut BSTEventSource<T>,
        callback: F,
    ) -> Result<&mut Self, EventBatchError>
    where
        T: 'a,
        F: FnMut(Option<&T>) -> R + 'static,
        R: IntoEventFlow,
    {
        let subscription = unsafe { source::subscribe::<'a, _, _, _>(source_ptr, callback) }
            .map_err(|error| EventBatchError::new(name, error))?;
        Ok(self.push_keepalive(name, subscription))
    }

    /// # Safety
    /// `source_ptr` must remain valid until this batch is dropped.
    pub unsafe fn prepend_source<T, F, R>(
        &mut self,
        name: &'static str,
        source_ptr: *mut BSTEventSource<T>,
        callback: F,
    ) -> Result<&mut Self, EventBatchError>
    where
        T: 'a,
        F: FnMut(Option<&T>) -> R + 'static,
        R: IntoEventFlow,
    {
        let subscription = unsafe { source::prepend::<'a, _, _, _>(source_ptr, callback) }
            .map_err(|error| EventBatchError::new(name, error))?;
        Ok(self.push_keepalive(name, subscription))
    }

    pub fn source_ref<T, F, R>(
        &mut self,
        name: &'static str,
        source_ref: EventSourceRef<'a, T>,
        callback: F,
    ) -> Result<&mut Self, EventBatchError>
    where
        T: 'a,
        F: FnMut(Option<&T>) -> R + 'static,
        R: IntoEventFlow,
    {
        let subscription = source_ref
            .subscribe(callback)
            .map_err(|error| EventBatchError::new(name, error))?;
        Ok(self.push_keepalive(name, subscription))
    }

    pub fn prepend_source_ref<T, F, R>(
        &mut self,
        name: &'static str,
        source_ref: EventSourceRef<'a, T>,
        callback: F,
    ) -> Result<&mut Self, EventBatchError>
    where
        T: 'a,
        F: FnMut(Option<&T>) -> R + 'static,
        R: IntoEventFlow,
    {
        let subscription = source_ref
            .prepend(callback)
            .map_err(|error| EventBatchError::new(name, error))?;
        Ok(self.push_keepalive(name, subscription))
    }

    pub fn bus<T, F, R>(
        &mut self,
        name: &'static str,
        bus: &'a Bus<T>,
        callback: F,
    ) -> Result<&mut Self, EventBatchError>
    where
        T: 'a,
        F: FnMut(&mut T) -> R + 'static,
        R: IntoEventFlow,
    {
        let subscription = bus.subscribe(callback);
        Ok(self.push_keepalive(name, subscription))
    }

    pub fn bus_with_priority<T, F, R>(
        &mut self,
        name: &'static str,
        bus: &'a Bus<T>,
        priority: SubscriberPriority,
        callback: F,
    ) -> Result<&mut Self, EventBatchError>
    where
        T: 'a,
        F: FnMut(&mut T) -> R + 'static,
        R: IntoEventFlow,
    {
        let subscription = bus.subscribe_with_priority(priority, callback);
        Ok(self.push_keepalive(name, subscription))
    }

    #[inline(always)]
    pub fn bus_first<T, F, R>(
        &mut self,
        name: &'static str,
        bus: &'a Bus<T>,
        callback: F,
    ) -> Result<&mut Self, EventBatchError>
    where
        T: 'a,
        F: FnMut(&mut T) -> R + 'static,
        R: IntoEventFlow,
    {
        self.bus_with_priority(name, bus, SubscriberPriority::FIRST, callback)
    }

    #[inline(always)]
    pub fn bus_early<T, F, R>(
        &mut self,
        name: &'static str,
        bus: &'a Bus<T>,
        callback: F,
    ) -> Result<&mut Self, EventBatchError>
    where
        T: 'a,
        F: FnMut(&mut T) -> R + 'static,
        R: IntoEventFlow,
    {
        self.bus_with_priority(name, bus, SubscriberPriority::EARLY, callback)
    }

    #[inline(always)]
    pub fn bus_late<T, F, R>(
        &mut self,
        name: &'static str,
        bus: &'a Bus<T>,
        callback: F,
    ) -> Result<&mut Self, EventBatchError>
    where
        T: 'a,
        F: FnMut(&mut T) -> R + 'static,
        R: IntoEventFlow,
    {
        self.bus_with_priority(name, bus, SubscriberPriority::LATE, callback)
    }

    #[inline(always)]
    pub fn bus_last<T, F, R>(
        &mut self,
        name: &'static str,
        bus: &'a Bus<T>,
        callback: F,
    ) -> Result<&mut Self, EventBatchError>
    where
        T: 'a,
        F: FnMut(&mut T) -> R + 'static,
        R: IntoEventFlow,
    {
        self.bus_with_priority(name, bus, SubscriberPriority::LAST, callback)
    }

    fn push_keepalive<H>(&mut self, name: &'static str, handle: H) -> &mut Self
    where
        H: 'a,
    {
        self.installed.push(InstalledEvent {
            name,
            _keepalive: Some(Box::new(handle)),
        });
        self
    }
}

pub fn try_install_all(
    batch: &mut EventBatch<'_>,
    installers: &[EventInstaller],
) -> EventBatchInstallResult {
    for installer in installers {
        installer.try_install(batch)?;
    }

    Ok(())
}

pub fn install_batch_or_fatal(batch: &mut EventBatch<'_>, installers: &[EventInstaller]) {
    if let Err(error) = try_install_all(batch, installers) {
        error.install_or_fatal();
    }
}

#[cfg(test)]
mod tests {
    use alloc::rc::Rc;
    use core::cell::Cell;

    use crate::re::BSTEventSource;

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
                        crate::sdk::events::EventFlow::Continue
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
                    crate::sdk::events::EventFlow::Continue
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
}
