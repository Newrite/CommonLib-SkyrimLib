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

#[inline(always)]
pub fn has_registered_model() -> bool {
    SERIALIZATION_STATE.lock().is_some()
}

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

pub fn registered_model_unique_id() -> Option<UniqueId> {
    SERIALIZATION_STATE
        .lock()
        .as_ref()
        .map(|state| state.driver.unique_id())
}

pub fn last_runtime_error() -> Option<RuntimeError> {
    SERIALIZATION_STATE
        .lock()
        .as_ref()
        .and_then(|state| state.runtime_error.clone())
}

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

pub fn with_registered_model<T: Model, R>(f: impl FnOnce(&T) -> R) -> Result<R, ModelAccessError> {
    with_model_driver(|driver| f(&driver.state))
}

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
