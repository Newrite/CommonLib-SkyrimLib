use alloc::boxed::Box;
use alloc::vec::Vec;

use super::types::{EventBatchError, InstalledEvent};
use crate::re::{BSTEventSource, ScriptEventSourceHolderEvent, UIEventSourceEvent};
use crate::sdk::core::{GameLifecyclePhase, LifecyclePhase, PluginLifecyclePhase};
use crate::sdk::events::bus::{Bus, SubscriberPriority};
use crate::sdk::events::input::InputEvents;
use crate::sdk::events::skse::dispatchers::DispatcherEvent;
use crate::sdk::events::skse::messages::{MessageKind, MessageRef};
use crate::sdk::events::source::{self, EventSourceRef, IntoEventFlow};
use crate::sdk::events::{game, input, ui};

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
        let _ = crate::sdk::events::skse::messages::on(kind, callback);
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
        let _ = crate::sdk::events::skse::messages::on_filtered(kind, predicate, callback);
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
        let _ = crate::sdk::events::skse::messages::on_sender_str(kind, sender, callback);
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
        let _ = crate::sdk::events::skse::messages::on_plugin_phase(phase, callback);
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
        let _ =
            crate::sdk::events::skse::messages::on_plugin_phase_sender_str(phase, sender, callback);
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
        let _ = crate::sdk::events::skse::messages::on_game_lifecycle(phase, callback);
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
        let _ = crate::sdk::events::skse::messages::on_game_lifecycle_sender_str(
            phase, sender, callback,
        );
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
        let _ = crate::sdk::events::skse::messages::on_lifecycle(phase, callback);
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
        let _ =
            crate::sdk::events::skse::messages::on_lifecycle_sender_str(phase, sender, callback);
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
        let subscription = crate::sdk::events::skse::dispatchers::subscribe::<E, _, _>(callback)
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
        let subscription = crate::sdk::events::skse::dispatchers::prepend::<E, _, _>(callback)
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
