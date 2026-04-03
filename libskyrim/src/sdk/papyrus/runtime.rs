//! Runtime installer for SDK Papyrus event registries.
//!
//! This layer owns the SKSE serialization callbacks for a plugin's Papyrus
//! event registries and provides typed access back to the installed registry
//! set after registration.

use alloc::boxed::Box;
use core::any::Any;

use spin::Mutex;

use crate::re::bs_core_types::VMHandle;
use crate::skse::{SerializationInterface, get_serialization_interface};

use super::events::{PapyrusEventCollection, PapyrusEventLoadStatus};

/// Unique SKSE serialization ID used by a plugin's Papyrus event set.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PapyrusEventUniqueId(u32);

impl PapyrusEventUniqueId {
    #[inline(always)]
    pub const fn new(raw: u32) -> Self {
        Self(raw)
    }

    #[inline(always)]
    pub const fn raw(self) -> u32 {
        self.0
    }
}

/// Installed Papyrus event-set definition.
pub struct PapyrusEventSet<T> {
    unique_id: PapyrusEventUniqueId,
    events: T,
}

impl<T> PapyrusEventSet<T> {
    #[inline(always)]
    pub const fn new(unique_id: PapyrusEventUniqueId, events: T) -> Self {
        Self { unique_id, events }
    }

    #[inline(always)]
    pub const fn unique_id(&self) -> PapyrusEventUniqueId {
        self.unique_id
    }

    #[inline(always)]
    pub const fn events(&self) -> &T {
        &self.events
    }

    #[inline(always)]
    pub fn events_mut(&mut self) -> &mut T {
        &mut self.events
    }

    #[inline(always)]
    pub fn into_events(self) -> T {
        self.events
    }
}

/// Error returned when registering the global Papyrus event runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PapyrusEventRegistrationError {
    InterfaceUnavailable,
    AlreadyRegistered,
}

/// Error returned when accessing the installed Papyrus event set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PapyrusEventAccessError {
    NotRegistered,
    WrongEventSetType,
}

/// Runtime callback failure surfaced by the Papyrus event installer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PapyrusEventRuntimeError {
    SaveFailed,
    LoadFailed { ty: u32, version: u32 },
}

trait RegisteredEventSet: Send {
    fn unique_id(&self) -> PapyrusEventUniqueId;
    fn save(&self, serialization: &SerializationInterface) -> Result<(), PapyrusEventRuntimeError>;
    fn load(
        &mut self,
        serialization: &SerializationInterface,
    ) -> Result<(), PapyrusEventRuntimeError>;
    fn revert(&mut self, serialization: Option<&SerializationInterface>);
    fn form_delete(&mut self, handle: VMHandle) -> usize;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T> RegisteredEventSet for PapyrusEventSet<T>
where
    T: PapyrusEventCollection + Send + 'static,
{
    #[inline(always)]
    fn unique_id(&self) -> PapyrusEventUniqueId {
        self.unique_id
    }

    fn save(&self, serialization: &SerializationInterface) -> Result<(), PapyrusEventRuntimeError> {
        if self.events.save_events(serialization) {
            Ok(())
        } else {
            Err(PapyrusEventRuntimeError::SaveFailed)
        }
    }

    fn load(
        &mut self,
        serialization: &SerializationInterface,
    ) -> Result<(), PapyrusEventRuntimeError> {
        let mut ty = 0_u32;
        let mut version = 0_u32;
        let mut length = 0_u32;

        while serialization.get_next_record_info(&mut ty, &mut version, &mut length) {
            match self.events.load_event_record(ty, serialization) {
                PapyrusEventLoadStatus::Loaded => {}
                PapyrusEventLoadStatus::Ignored => {
                    if !skip_record_data(serialization, length) {
                        return Err(PapyrusEventRuntimeError::LoadFailed { ty, version });
                    }
                }
                PapyrusEventLoadStatus::Failed => {
                    return Err(PapyrusEventRuntimeError::LoadFailed { ty, version });
                }
            }
        }

        Ok(())
    }

    #[inline(always)]
    fn revert(&mut self, serialization: Option<&SerializationInterface>) {
        self.events.revert_events(serialization);
    }

    #[inline(always)]
    fn form_delete(&mut self, handle: VMHandle) -> usize {
        self.events.form_delete_events(handle)
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    #[inline(always)]
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

struct RuntimeState {
    set: Box<dyn RegisteredEventSet>,
    last_error: Option<PapyrusEventRuntimeError>,
}

impl RuntimeState {
    #[inline(always)]
    fn new(set: Box<dyn RegisteredEventSet>) -> Self {
        Self {
            set,
            last_error: None,
        }
    }

    #[inline(always)]
    fn clear_last_error(&mut self) {
        self.last_error = None;
    }

    #[inline(always)]
    fn set_last_error(&mut self, error: PapyrusEventRuntimeError) {
        self.last_error = Some(error);
    }
}

static PAPYRUS_EVENT_RUNTIME: Mutex<Option<RuntimeState>> = Mutex::new(None);

fn skip_record_data(serialization: &SerializationInterface, mut length: u32) -> bool {
    let mut scratch = [0_u8; 64];

    while length != 0 {
        let chunk = length.min(scratch.len() as u32);
        let read = serialization.read_record_data(scratch.as_mut_ptr().cast(), chunk);
        if read == 0 {
            return false;
        }
        length = length.saturating_sub(read);
    }

    true
}

#[inline(always)]
pub fn is_registered() -> bool {
    PAPYRUS_EVENT_RUNTIME.lock().is_some()
}

pub fn register_event_set<T>(set: PapyrusEventSet<T>) -> Result<(), PapyrusEventRegistrationError>
where
    T: PapyrusEventCollection + Send + 'static,
{
    let serialization_ptr = get_serialization_interface();
    let serialization = unsafe { serialization_ptr.as_ref() }
        .ok_or(PapyrusEventRegistrationError::InterfaceUnavailable)?;

    let mut state = PAPYRUS_EVENT_RUNTIME.lock();
    if state.is_some() {
        return Err(PapyrusEventRegistrationError::AlreadyRegistered);
    }

    serialization.set_unique_id(set.unique_id().raw());
    serialization.set_save_callback(Some(save_callback));
    serialization.set_load_callback(Some(load_callback));
    serialization.set_revert_callback(Some(revert_callback));
    serialization.set_form_delete_callback(Some(form_delete_callback));

    *state = Some(RuntimeState::new(Box::new(set)));
    Ok(())
}

pub fn unregister_event_set() {
    let serialization_ptr = get_serialization_interface();
    if let Some(serialization) = unsafe { serialization_ptr.as_ref() } {
        serialization.set_save_callback(None);
        serialization.set_load_callback(None);
        serialization.set_revert_callback(None);
        serialization.set_form_delete_callback(None);
    }

    *PAPYRUS_EVENT_RUNTIME.lock() = None;
}

#[inline(always)]
pub fn registered_unique_id() -> Option<PapyrusEventUniqueId> {
    PAPYRUS_EVENT_RUNTIME
        .lock()
        .as_ref()
        .map(|state| state.set.unique_id())
}

#[inline(always)]
pub fn last_error() -> Option<PapyrusEventRuntimeError> {
    PAPYRUS_EVENT_RUNTIME
        .lock()
        .as_ref()
        .and_then(|state| state.last_error)
}

#[inline(always)]
pub fn take_last_error() -> Option<PapyrusEventRuntimeError> {
    PAPYRUS_EVENT_RUNTIME
        .lock()
        .as_mut()
        .and_then(|state| state.last_error.take())
}

fn with_registered_set<T, R>(
    f: impl FnOnce(&PapyrusEventSet<T>) -> R,
) -> Result<R, PapyrusEventAccessError>
where
    T: PapyrusEventCollection + Send + 'static,
{
    let state = &mut *PAPYRUS_EVENT_RUNTIME.lock();
    let state = state
        .as_mut()
        .ok_or(PapyrusEventAccessError::NotRegistered)?;
    let set = state
        .set
        .as_any()
        .downcast_ref::<PapyrusEventSet<T>>()
        .ok_or(PapyrusEventAccessError::WrongEventSetType)?;
    Ok(f(set))
}

fn with_registered_set_mut<T, R>(
    f: impl FnOnce(&mut PapyrusEventSet<T>) -> R,
) -> Result<R, PapyrusEventAccessError>
where
    T: PapyrusEventCollection + Send + 'static,
{
    let state = &mut *PAPYRUS_EVENT_RUNTIME.lock();
    let state = state
        .as_mut()
        .ok_or(PapyrusEventAccessError::NotRegistered)?;
    let set = state
        .set
        .as_any_mut()
        .downcast_mut::<PapyrusEventSet<T>>()
        .ok_or(PapyrusEventAccessError::WrongEventSetType)?;
    Ok(f(set))
}

#[inline(always)]
pub fn with_event_set<T, R>(
    f: impl FnOnce(&PapyrusEventSet<T>) -> R,
) -> Result<R, PapyrusEventAccessError>
where
    T: PapyrusEventCollection + Send + 'static,
{
    with_registered_set(f)
}

#[inline(always)]
pub fn with_event_set_mut<T, R>(
    f: impl FnOnce(&mut PapyrusEventSet<T>) -> R,
) -> Result<R, PapyrusEventAccessError>
where
    T: PapyrusEventCollection + Send + 'static,
{
    with_registered_set_mut(f)
}

#[inline(always)]
pub fn with_events<T, R>(f: impl FnOnce(&T) -> R) -> Result<R, PapyrusEventAccessError>
where
    T: PapyrusEventCollection + Send + 'static,
{
    with_registered_set(|set| f(set.events()))
}

#[inline(always)]
pub fn with_events_mut<T, R>(f: impl FnOnce(&mut T) -> R) -> Result<R, PapyrusEventAccessError>
where
    T: PapyrusEventCollection + Send + 'static,
{
    with_registered_set_mut(|set| f(set.events_mut()))
}

unsafe extern "system" fn save_callback(serialization: *mut SerializationInterface) {
    crate::skse::crash::guard("SKSE papyrus event save callback", || {
        let Some(serialization) = (unsafe { serialization.as_ref() }) else {
            return;
        };

        let state = &mut *PAPYRUS_EVENT_RUNTIME.lock();
        let Some(state) = state.as_mut() else {
            return;
        };

        state.clear_last_error();
        if let Err(error) = state.set.save(serialization) {
            state.set_last_error(error);
        }
    });
}

unsafe extern "system" fn load_callback(serialization: *mut SerializationInterface) {
    crate::skse::crash::guard("SKSE papyrus event load callback", || {
        let Some(serialization) = (unsafe { serialization.as_ref() }) else {
            return;
        };

        let state = &mut *PAPYRUS_EVENT_RUNTIME.lock();
        let Some(state) = state.as_mut() else {
            return;
        };

        state.clear_last_error();
        if let Err(error) = state.set.load(serialization) {
            state.set_last_error(error);
        }
    });
}

unsafe extern "system" fn revert_callback(serialization: *mut SerializationInterface) {
    crate::skse::crash::guard("SKSE papyrus event revert callback", || {
        let state = &mut *PAPYRUS_EVENT_RUNTIME.lock();
        let Some(state) = state.as_mut() else {
            return;
        };

        state.clear_last_error();
        state.set.revert(unsafe { serialization.as_ref() });
    });
}

unsafe extern "system" fn form_delete_callback(handle: VMHandle) {
    crate::skse::crash::guard("SKSE papyrus event form-delete callback", || {
        let state = &mut *PAPYRUS_EVENT_RUNTIME.lock();
        let Some(state) = state.as_mut() else {
            return;
        };

        let _ = state.set.form_delete(handle);
    });
}
