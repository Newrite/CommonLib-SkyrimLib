//! Runtime registration and callback glue for plugin serialization models.
//!
//! This layer bridges the declarative [`super::Schema`] / [`super::Model`]
//! surface to the raw `SKSE::SerializationInterface` callbacks. Most plugin
//! code should interact with it through:
//!
//! - [`register_model`] during bootstrap;
//! - [`with_registered_model`] or [`with_registered_model_mut`] during runtime;
//! - [`last_runtime_error`] or [`take_last_runtime_error`] when surfacing
//!   save/load diagnostics.
//!
//! Typical registered-model flow:
//!
//! 1. implement [`super::Model`] for one plugin-owned state type;
//! 2. call [`register_model`] once during bootstrap;
//! 3. let SKSE drive save/load/revert/form-delete callbacks automatically;
//! 4. borrow that same model later through [`with_registered_model`] or
//!    [`with_registered_model_mut`];
//! 5. inspect [`last_runtime_error`] / [`take_last_runtime_error`] when
//!    debugging failed save/load passes.
//!
//! Sketch:
//!
//! ```rust,ignore
//! use libskyrim::sdk::plugin::serialization::{self, Model, Schema};
//!
//! #[derive(Default)]
//! struct SaveState {
//!     counter: u32,
//! }
//!
//! impl Model for SaveState {
//!     const UNIQUE_ID: serialization::UniqueId = serialization::unique_id!("EXMP");
//!
//!     fn schema(schema: &mut Schema<Self>) {
//!         schema.value(serialization::record_id!("CNT1"), 1, |s| &s.counter, |s, v| s.counter = v);
//!     }
//! }
//!
//! fn install_serialization() {
//!     serialization::register_model::<SaveState>().unwrap();
//! }
//!
//! fn increment_counter() {
//!     serialization::with_registered_model_mut::<SaveState, _>(|state| {
//!         state.counter += 1;
//!     })
//!     .unwrap();
//! }
//! ```

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::any::Any;

use spin::Mutex;

use crate::re::bs_core_types::VMHandle;
use crate::skse::{SerializationInterface, get_serialization_interface};

use super::schema::{BuiltSchema, Schema};
use super::types::{Model, ModelAccessError, RegistrationError, RuntimeError, SchemaBuildError};
use super::{LoadContext, LoadedRecord, OwnedRecord, UniqueId, read_next_record};

trait RegisteredDriver: Send {
    fn unique_id(&self) -> UniqueId;
    fn save(&mut self, serialization: &SerializationInterface) -> Result<(), RuntimeError>;
    fn load(&mut self, serialization: &SerializationInterface) -> Result<(), RuntimeError>;
    fn revert(&mut self);
    fn form_delete(&mut self, handle: VMHandle);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub(super) struct ModelDriver<T: Model> {
    pub(super) state: T,
    schema: BuiltSchema<T>,
    pub(super) unknown_records: Vec<LoadedRecord>,
}

impl<T: Model> ModelDriver<T> {
    pub(super) fn new() -> Result<Self, SchemaBuildError> {
        let mut schema = Schema::<T>::new();
        T::schema(&mut schema);
        Ok(Self {
            state: T::default(),
            schema: schema.build()?,
            unknown_records: Vec::new(),
        })
    }

    pub(super) fn build_save_records(&self) -> Result<Vec<OwnedRecord>, RuntimeError> {
        let mut records = self.schema.save(&self.state)?;

        for record in &self.unknown_records {
            // TODO: COMPAT - schema-owned record IDs currently win over passthrough future-version
            // records with the same ID, so unknown newer payloads are not re-emitted here. Preserve
            // those records once the schema layer can distinguish "current owned record" from
            // "unknown future-version record" without duplicating conflicting same-ID output.
            if self.schema.contains_id(record.header().id()) {
                continue;
            }

            let owned = OwnedRecord::new(
                record.header().id(),
                record.header().version(),
                record.bytes().to_vec(),
            )
            .map_err(|source| RuntimeError::SaveRecord {
                id: record.header().id(),
                version: record.header().version(),
                source,
            })?;
            records.push(owned);
        }

        Ok(records)
    }

    pub(super) fn load_records<I>(
        &mut self,
        records: I,
        context: LoadContext<'_>,
    ) -> Result<(), RuntimeError>
    where
        I: IntoIterator<Item = LoadedRecord>,
    {
        let mut next_state = T::default();
        let mut next_unknown_records = Vec::new();

        for record in records {
            if !self.schema.try_load(&mut next_state, &record, context)? {
                next_unknown_records.push(record);
            }
        }

        self.state = next_state;
        self.unknown_records = next_unknown_records;
        Ok(())
    }
}

impl<T: Model> RegisteredDriver for ModelDriver<T> {
    #[inline(always)]
    fn unique_id(&self) -> UniqueId {
        T::UNIQUE_ID
    }

    fn save(&mut self, serialization: &SerializationInterface) -> Result<(), RuntimeError> {
        let records = self.build_save_records()?;
        for record in &records {
            let header = record.header();
            record
                .write_to(serialization)
                .map_err(|source| RuntimeError::SaveRecord {
                    id: header.id(),
                    version: header.version(),
                    source,
                })?;
        }

        Ok(())
    }

    fn load(&mut self, serialization: &SerializationInterface) -> Result<(), RuntimeError> {
        let context = LoadContext::new(serialization);
        let mut records = Vec::new();

        while let Some(record) = read_next_record(serialization)
            .map_err(|source| RuntimeError::ReadNextRecord { source })?
        {
            records.push(record);
        }

        self.load_records(records, context)
    }

    fn revert(&mut self) {
        self.state = T::default();
        self.unknown_records.clear();
        self.state.on_revert();
    }

    fn form_delete(&mut self, handle: VMHandle) {
        self.state.on_form_delete(handle);
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

struct SerializationState {
    driver: Box<dyn RegisteredDriver>,
    runtime_error: Option<RuntimeError>,
}

impl SerializationState {
    #[inline(always)]
    fn new(driver: Box<dyn RegisteredDriver>) -> Self {
        Self {
            driver,
            runtime_error: None,
        }
    }

    #[inline(always)]
    fn set_runtime_error(&mut self, error: RuntimeError) {
        self.runtime_error = Some(error);
    }

    #[inline(always)]
    fn clear_runtime_error(&mut self) {
        self.runtime_error = None;
    }
}

static SERIALIZATION_STATE: Mutex<Option<SerializationState>> = Mutex::new(None);

/// Whether a serialization model is currently registered for this plugin.
///
/// This is mostly useful for diagnostics or defensive install code; most
/// plugins simply call [`register_model`] once during bootstrap.
#[inline(always)]
pub fn has_registered_model() -> bool {
    SERIALIZATION_STATE.lock().is_some()
}

/// Installs a plugin-owned serialization [`Model`].
///
/// Call this once during bootstrap, typically around `PostLoad` or
/// `DataLoaded`, before any code expects the model to be available through
/// [`with_registered_model`] or [`with_registered_model_mut`].
///
/// On success this function:
///
/// - builds the model schema;
/// - registers the plugin unique ID with SKSE;
/// - installs save/load/revert/form-delete callbacks;
/// - stores one live model instance for later access.
pub fn register_model<T: Model>() -> Result<(), RegistrationError> {
    let serialization_ptr = get_serialization_interface();
    let serialization =
        unsafe { serialization_ptr.as_ref() }.ok_or(RegistrationError::InterfaceUnavailable)?;

    let mut state = SERIALIZATION_STATE.lock();
    if state.is_some() {
        return Err(RegistrationError::AlreadyRegistered);
    }

    let driver = ModelDriver::<T>::new().map_err(RegistrationError::Schema)?;
    serialization.set_unique_id(T::UNIQUE_ID.raw());
    serialization.set_save_callback(Some(save_callback));
    serialization.set_load_callback(Some(load_callback));
    serialization.set_revert_callback(Some(revert_callback));
    serialization.set_form_delete_callback(Some(form_delete_callback));

    *state = Some(SerializationState::new(Box::new(driver)));
    Ok(())
}

/// Removes the registered model and clears installed SKSE callbacks.
///
/// This is rarely needed in ordinary plugin code, but it is useful for tests,
/// reload-like flows, or explicit teardown.
///
/// In normal plugins, registration usually happens once and remains installed
/// for the plugin's whole lifetime.
pub fn unregister_model() {
    let serialization_ptr = get_serialization_interface();
    if let Some(serialization) = unsafe { serialization_ptr.as_ref() } {
        serialization.set_save_callback(None);
        serialization.set_load_callback(None);
        serialization.set_revert_callback(None);
        serialization.set_form_delete_callback(None);
    }

    *SERIALIZATION_STATE.lock() = None;
}

/// Return the unique ID of the currently registered model, if any.
///
/// This is mainly diagnostic glue when a plugin wants to expose or validate
/// which registered model currently owns the serialization callbacks.
///
/// Ordinary runtime code usually does not need this because
/// [`with_registered_model`] / [`with_registered_model_mut`] already route
/// through the installed state directly.
pub fn registered_model_unique_id() -> Option<UniqueId> {
    SERIALIZATION_STATE
        .lock()
        .as_ref()
        .map(|state| state.driver.unique_id())
}

/// Borrow the last runtime save/load error without clearing it.
///
/// Use this when the plugin wants to log or inspect the last failure while
/// leaving it available for later diagnostics.
pub fn last_runtime_error() -> Option<RuntimeError> {
    SERIALIZATION_STATE
        .lock()
        .as_ref()
        .and_then(|state| state.runtime_error.clone())
}

/// Take and clear the last runtime save/load error.
///
/// Use this when the plugin consumes serialization failures as one-shot
/// diagnostics and does not want them reported repeatedly.
pub fn take_last_runtime_error() -> Option<RuntimeError> {
    SERIALIZATION_STATE
        .lock()
        .as_mut()
        .and_then(|state| state.runtime_error.take())
}

fn with_model_driver<T: Model, R>(
    f: impl FnOnce(&ModelDriver<T>) -> R,
) -> Result<R, ModelAccessError> {
    let state = &mut *SERIALIZATION_STATE.lock();
    let state = state.as_mut().ok_or(ModelAccessError::NotRegistered)?;
    let driver = state
        .driver
        .as_any()
        .downcast_ref::<ModelDriver<T>>()
        .ok_or(ModelAccessError::WrongModelType)?;
    Ok(f(driver))
}

fn with_model_driver_mut<T: Model, R>(
    f: impl FnOnce(&mut ModelDriver<T>) -> R,
) -> Result<R, ModelAccessError> {
    let state = &mut *SERIALIZATION_STATE.lock();
    let state = state.as_mut().ok_or(ModelAccessError::NotRegistered)?;
    let driver = state
        .driver
        .as_any_mut()
        .downcast_mut::<ModelDriver<T>>()
        .ok_or(ModelAccessError::WrongModelType)?;
    Ok(f(driver))
}

/// Borrows the currently registered model immutably.
///
/// This is the normal read-only entry point for runtime code that wants access
/// to plugin-owned persistent state.
///
/// Prefer this over storing your own global pointer or lock to the model:
/// it keeps all access routed through the same installed serialization state.
pub fn with_registered_model<T: Model, R>(f: impl FnOnce(&T) -> R) -> Result<R, ModelAccessError> {
    with_model_driver(|driver| f(&driver.state))
}

/// Borrows the currently registered model mutably.
///
/// This is the normal mutation entry point for runtime code that wants to
/// update plugin-owned persistent state between save/load callbacks.
///
/// Keep callback bodies lightweight: borrow, mutate the model, and hand heavier
/// gameplay or UI work off through `sdk::plugin::task` when appropriate.
pub fn with_registered_model_mut<T: Model, R>(
    f: impl FnOnce(&mut T) -> R,
) -> Result<R, ModelAccessError> {
    with_model_driver_mut(|driver| f(&mut driver.state))
}

unsafe extern "system" fn save_callback(serialization: *mut SerializationInterface) {
    crate::skse::crash::guard("SKSE serialization save callback", || {
        let Some(serialization) = (unsafe { serialization.as_ref() }) else {
            return;
        };

        let state = &mut *SERIALIZATION_STATE.lock();
        let Some(state) = state.as_mut() else {
            return;
        };

        state.clear_runtime_error();
        if let Err(error) = state.driver.save(serialization) {
            state.set_runtime_error(error);
        }
    });
}

unsafe extern "system" fn load_callback(serialization: *mut SerializationInterface) {
    crate::skse::crash::guard("SKSE serialization load callback", || {
        let Some(serialization) = (unsafe { serialization.as_ref() }) else {
            return;
        };

        let state = &mut *SERIALIZATION_STATE.lock();
        let Some(state) = state.as_mut() else {
            return;
        };

        state.clear_runtime_error();
        if let Err(error) = state.driver.load(serialization) {
            state.set_runtime_error(error);
        }
    });
}

unsafe extern "system" fn revert_callback(_serialization: *mut SerializationInterface) {
    crate::skse::crash::guard("SKSE serialization revert callback", || {
        let state = &mut *SERIALIZATION_STATE.lock();
        let Some(state) = state.as_mut() else {
            return;
        };

        state.clear_runtime_error();
        state.driver.revert();
    });
}

unsafe extern "system" fn form_delete_callback(handle: VMHandle) {
    crate::skse::crash::guard("SKSE serialization form-delete callback", || {
        let state = &mut *SERIALIZATION_STATE.lock();
        let Some(state) = state.as_mut() else {
            return;
        };

        state.driver.form_delete(handle);
    });
}
