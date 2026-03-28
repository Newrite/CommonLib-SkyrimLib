//! High-level serialization registration layered over SKSE's low-level cosave
//! interface.
//!
//! This module owns the callback-registration side of serialization. The raw
//! binary format helpers live in `sdk::persistence::cosave`; this layer builds
//! a model/schema API on top so plugin authors can register one persistent
//! model and let the framework drive save/load/revert callbacks.

/// Expand a list of top-level `Schema::value(...)` declarations using field
/// names instead of repetitive getter/setter closures.
#[macro_export]
macro_rules! schema_fields {
    ($schema:ident, { $($record_id:expr => $version:expr => $field:ident),+ $(,)? }) => {{
        let __sdk_schema = &mut $schema;
        $(
            __sdk_schema.value(
                $record_id,
                $version,
                |state| &state.$field,
                |state, value| state.$field = value,
            );
        )+
    }};
    ($schema:expr, { $($record_id:expr => $version:expr => $field:ident),+ $(,)? }) => {{
        let __sdk_schema: &mut _ = $schema;
        $(
            __sdk_schema.value(
                $record_id,
                $version,
                |state| &state.$field,
                |state, value| state.$field = value,
            );
        )+
    }};
}

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::any::Any;
use core::fmt;
use core::marker::PhantomData;

use spin::Mutex;

use crate::re::bs_core_types::VMHandle;
pub use crate::schema_fields;
pub use crate::sdk::persistence::cosave::{
    BoundedVec, BoundedVecError, CosaveDecode, CosaveEncode, LoadContext, LoadError, LoadedRecord,
    OwnedRecord, RecordHeader, RecordId, RecordReader, RecordWriter, ResolvedFormId,
    ResolvedVmHandle, SaveError, UniqueId, fourcc_from_str, read_next_record, record_id, unique_id,
};
use crate::skse::{SerializationInterface, get_serialization_interface};
pub use libskyrim_macros::Cosave;

pub use crate::sdk::persistence::cosave;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadStatus {
    Handled,
    Unhandled,
}

pub trait Model: Default + Send + 'static {
    const UNIQUE_ID: UniqueId;

    fn schema(schema: &mut Schema<Self>);

    fn on_revert(&mut self) {}

    fn on_form_delete(&mut self, _handle: VMHandle) {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchemaBuildError {
    DuplicateRecordId(RecordId),
    DuplicateRecordVersion { id: RecordId, version: u32 },
}

impl fmt::Display for SchemaBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateRecordId(id) => write!(f, "duplicate serialization record id {}", id),
            Self::DuplicateRecordVersion { id, version } => write!(
                f,
                "duplicate serialization loader version {} for record {}",
                version, id
            ),
        }
    }
}

impl core::error::Error for SchemaBuildError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistrationError {
    InterfaceUnavailable,
    AlreadyRegistered,
    Schema(SchemaBuildError),
}

impl fmt::Display for RegistrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InterfaceUnavailable => write!(f, "SKSE serialization interface is unavailable"),
            Self::AlreadyRegistered => write!(f, "a serialization model is already registered"),
            Self::Schema(error) => write!(f, "invalid serialization schema: {}", error),
        }
    }
}

impl core::error::Error for RegistrationError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelAccessError {
    NotRegistered,
    WrongModelType,
}

impl fmt::Display for ModelAccessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotRegistered => write!(f, "no serialization model is registered"),
            Self::WrongModelType => {
                write!(
                    f,
                    "registered serialization model type does not match request"
                )
            }
        }
    }
}

impl core::error::Error for ModelAccessError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeError {
    SaveRecord {
        id: RecordId,
        version: u32,
        source: SaveError,
    },
    ReadNextRecord {
        source: LoadError,
    },
    LoadRecord {
        header: RecordHeader,
        source: LoadError,
    },
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SaveRecord {
                id,
                version,
                source,
            } => write!(
                f,
                "failed to save serialization record {} version {}: {}",
                id, version, source
            ),
            Self::ReadNextRecord { source } => {
                write!(
                    f,
                    "failed to read the next serialization record: {}",
                    source
                )
            }
            Self::LoadRecord { header, source } => write!(
                f,
                "failed to load serialization record {} version {}: {}",
                header.id(),
                header.version(),
                source
            ),
        }
    }
}

impl core::error::Error for RuntimeError {}

type ValueGetter<T, V> = fn(&T) -> &V;
type ValueSetter<T, V> = fn(&mut T, V);
type CustomSaveFn<T> = fn(&T, &mut RecordWriter) -> Result<(), SaveError>;
type CustomLoadFn<T> = fn(&mut T, &LoadedRecord, LoadContext<'_>) -> Result<LoadStatus, LoadError>;

trait RecordCodec<T>: Send {
    fn id(&self) -> RecordId;
    fn save_version(&self) -> u32;
    fn encode_record(&self, state: &T) -> Result<OwnedRecord, SaveError>;
    fn try_load(
        &self,
        state: &mut T,
        record: &LoadedRecord,
        context: LoadContext<'_>,
    ) -> Result<LoadStatus, LoadError>;
}

struct ValueRecord<T, V> {
    id: RecordId,
    version: u32,
    get: ValueGetter<T, V>,
    set: ValueSetter<T, V>,
    _marker: PhantomData<V>,
}

impl<T, V> ValueRecord<T, V> {
    #[inline(always)]
    const fn new(
        id: RecordId,
        version: u32,
        get: ValueGetter<T, V>,
        set: ValueSetter<T, V>,
    ) -> Self {
        Self {
            id,
            version,
            get,
            set,
            _marker: PhantomData,
        }
    }
}

impl<T, V> RecordCodec<T> for ValueRecord<T, V>
where
    T: Send + 'static,
    V: CosaveEncode + CosaveDecode + Send + 'static,
{
    #[inline(always)]
    fn id(&self) -> RecordId {
        self.id
    }

    #[inline(always)]
    fn save_version(&self) -> u32 {
        self.version
    }

    fn encode_record(&self, state: &T) -> Result<OwnedRecord, SaveError> {
        OwnedRecord::from_value(self.id, self.version, (self.get)(state))
    }

    fn try_load(
        &self,
        state: &mut T,
        record: &LoadedRecord,
        context: LoadContext<'_>,
    ) -> Result<LoadStatus, LoadError> {
        if record.header().version() != self.version {
            return Ok(LoadStatus::Unhandled);
        }

        let value = record.decode::<V>(context)?;
        (self.set)(state, value);
        Ok(LoadStatus::Handled)
    }
}

struct CustomRecord<T> {
    id: RecordId,
    save_version: u32,
    save: CustomSaveFn<T>,
    load: CustomLoadFn<T>,
}

impl<T> CustomRecord<T> {
    #[inline(always)]
    const fn new(
        id: RecordId,
        save_version: u32,
        save: CustomSaveFn<T>,
        load: CustomLoadFn<T>,
    ) -> Self {
        Self {
            id,
            save_version,
            save,
            load,
        }
    }
}

impl<T> RecordCodec<T> for CustomRecord<T>
where
    T: Send + 'static,
{
    #[inline(always)]
    fn id(&self) -> RecordId {
        self.id
    }

    #[inline(always)]
    fn save_version(&self) -> u32 {
        self.save_version
    }

    fn encode_record(&self, state: &T) -> Result<OwnedRecord, SaveError> {
        let mut writer = RecordWriter::new();
        (self.save)(state, &mut writer)?;
        writer.into_record(self.id, self.save_version)
    }

    fn try_load(
        &self,
        state: &mut T,
        record: &LoadedRecord,
        context: LoadContext<'_>,
    ) -> Result<LoadStatus, LoadError> {
        (self.load)(state, record, context)
    }
}

struct VersionedLoad<T> {
    version: u32,
    load: CustomLoadFn<T>,
}

impl<T> VersionedLoad<T> {
    #[inline(always)]
    const fn new(version: u32, load: CustomLoadFn<T>) -> Self {
        Self { version, load }
    }
}

struct MigratingRecord<T> {
    id: RecordId,
    save_version: u32,
    save: CustomSaveFn<T>,
    loads: Vec<VersionedLoad<T>>,
}

impl<T> MigratingRecord<T> {
    #[inline(always)]
    fn new(id: RecordId, save_version: u32, save: CustomSaveFn<T>) -> Self {
        Self {
            id,
            save_version,
            save,
            loads: Vec::new(),
        }
    }

    fn push_load(&mut self, version: u32, load: CustomLoadFn<T>) -> Result<(), SchemaBuildError> {
        if self
            .loads
            .iter()
            .any(|candidate| candidate.version == version)
        {
            return Err(SchemaBuildError::DuplicateRecordVersion {
                id: self.id,
                version,
            });
        }

        self.loads.push(VersionedLoad::new(version, load));
        self.loads.sort_by_key(|candidate| candidate.version);
        Ok(())
    }
}

impl<T> RecordCodec<T> for MigratingRecord<T>
where
    T: Send + 'static,
{
    #[inline(always)]
    fn id(&self) -> RecordId {
        self.id
    }

    #[inline(always)]
    fn save_version(&self) -> u32 {
        self.save_version
    }

    fn encode_record(&self, state: &T) -> Result<OwnedRecord, SaveError> {
        let mut writer = RecordWriter::new();
        (self.save)(state, &mut writer)?;
        writer.into_record(self.id, self.save_version)
    }

    fn try_load(
        &self,
        state: &mut T,
        record: &LoadedRecord,
        context: LoadContext<'_>,
    ) -> Result<LoadStatus, LoadError> {
        let version = record.header().version();
        let Some(candidate) = self
            .loads
            .iter()
            .find(|candidate| candidate.version == version)
        else {
            return Ok(LoadStatus::Unhandled);
        };

        (candidate.load)(state, record, context)
    }
}

pub struct MigratingRecordBuilder<'a, T: Send + 'static> {
    schema: Option<&'a mut Schema<T>>,
    record: Option<MigratingRecord<T>>,
}

impl<'a, T> MigratingRecordBuilder<'a, T>
where
    T: Send + 'static,
{
    #[inline(always)]
    fn new(
        schema: &'a mut Schema<T>,
        id: RecordId,
        save_version: u32,
        save: CustomSaveFn<T>,
    ) -> Self {
        Self {
            schema: Some(schema),
            record: Some(MigratingRecord::new(id, save_version, save)),
        }
    }

    pub fn load(mut self, version: u32, load: CustomLoadFn<T>) -> Self {
        if let Some(record) = self.record.as_mut() {
            if let Err(error) = record.push_load(version, load) {
                if let Some(schema) = self.schema.as_deref_mut() {
                    if schema.error.is_none() {
                        schema.error = Some(error);
                    }
                }
            }
        }

        self
    }

    pub fn finish(mut self) -> &'a mut Schema<T> {
        let schema = self
            .schema
            .take()
            .expect("migrating record builder should still own its schema");
        if let Some(record) = self.record.take() {
            schema.push(Box::new(record));
        }
        schema
    }
}

impl<T> Drop for MigratingRecordBuilder<'_, T>
where
    T: Send + 'static,
{
    fn drop(&mut self) {
        let Some(record) = self.record.take() else {
            return;
        };
        let Some(schema) = self.schema.as_deref_mut() else {
            return;
        };
        schema.push(Box::new(record));
    }
}

pub struct Schema<T> {
    records: Vec<Box<dyn RecordCodec<T> + Send>>,
    error: Option<SchemaBuildError>,
}

impl<T> Schema<T> {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
            error: None,
        }
    }

    pub fn value<V>(
        &mut self,
        id: RecordId,
        version: u32,
        get: ValueGetter<T, V>,
        set: ValueSetter<T, V>,
    ) -> &mut Self
    where
        T: Send + 'static,
        V: CosaveEncode + CosaveDecode + Send + 'static,
    {
        self.push(Box::new(ValueRecord::new(id, version, get, set)));
        self
    }

    pub fn record(
        &mut self,
        id: RecordId,
        save_version: u32,
        save: CustomSaveFn<T>,
        load: CustomLoadFn<T>,
    ) -> &mut Self
    where
        T: Send + 'static,
    {
        self.push(Box::new(CustomRecord::new(id, save_version, save, load)));
        self
    }

    pub fn migrating_record(
        &mut self,
        id: RecordId,
        save_version: u32,
        save: CustomSaveFn<T>,
    ) -> MigratingRecordBuilder<'_, T>
    where
        T: Send + 'static,
    {
        MigratingRecordBuilder::new(self, id, save_version, save)
    }

    fn push(&mut self, record: Box<dyn RecordCodec<T> + Send>) {
        if self.error.is_some() {
            return;
        }

        let id = record.id();
        if self.records.iter().any(|existing| existing.id() == id) {
            self.error = Some(SchemaBuildError::DuplicateRecordId(id));
            return;
        }

        self.records.push(record);
    }

    fn build(mut self) -> Result<BuiltSchema<T>, SchemaBuildError> {
        if let Some(error) = self.error.take() {
            return Err(error);
        }

        self.records.sort_by_key(|record| record.id().raw());
        Ok(BuiltSchema {
            records: self.records,
        })
    }
}

impl<T> Default for Schema<T> {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

struct BuiltSchema<T> {
    records: Vec<Box<dyn RecordCodec<T> + Send>>,
}

impl<T> BuiltSchema<T> {
    fn save(&self, state: &T) -> Result<Vec<OwnedRecord>, RuntimeError> {
        let mut records = Vec::with_capacity(self.records.len());
        for record in &self.records {
            let version = record.save_version();
            let encoded =
                record
                    .encode_record(state)
                    .map_err(|source| RuntimeError::SaveRecord {
                        id: record.id(),
                        version,
                        source,
                    })?;
            records.push(encoded);
        }
        Ok(records)
    }

    fn try_load(
        &self,
        state: &mut T,
        record: &LoadedRecord,
        context: LoadContext<'_>,
    ) -> Result<bool, RuntimeError> {
        for candidate in &self.records {
            if candidate.id() != record.header().id() {
                continue;
            }

            let handled = candidate
                .try_load(state, record, context)
                .map_err(|source| RuntimeError::LoadRecord {
                    header: record.header(),
                    source,
                })?;
            return Ok(handled == LoadStatus::Handled);
        }

        Ok(false)
    }

    #[inline(always)]
    fn contains_id(&self, id: RecordId) -> bool {
        self.records.iter().any(|record| record.id() == id)
    }
}

trait RegisteredDriver: Send {
    fn unique_id(&self) -> UniqueId;
    fn save(&mut self, serialization: &SerializationInterface) -> Result<(), RuntimeError>;
    fn load(&mut self, serialization: &SerializationInterface) -> Result<(), RuntimeError>;
    fn revert(&mut self);
    fn form_delete(&mut self, handle: VMHandle);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

struct ModelDriver<T: Model> {
    state: T,
    schema: BuiltSchema<T>,
    unknown_records: Vec<LoadedRecord>,
}

impl<T: Model> ModelDriver<T> {
    fn new() -> Result<Self, SchemaBuildError> {
        let mut schema = Schema::<T>::new();
        T::schema(&mut schema);
        Ok(Self {
            state: T::default(),
            schema: schema.build()?,
            unknown_records: Vec::new(),
        })
    }

    fn build_save_records(&self) -> Result<Vec<OwnedRecord>, RuntimeError> {
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

    fn load_records<I>(&mut self, records: I, context: LoadContext<'_>) -> Result<(), RuntimeError>
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
    last_error: Option<RuntimeError>,
}

impl SerializationState {
    #[inline(always)]
    fn new(driver: Box<dyn RegisteredDriver>) -> Self {
        Self {
            driver,
            last_error: None,
        }
    }

    #[inline(always)]
    fn set_last_error(&mut self, error: RuntimeError) {
        self.last_error = Some(error);
    }

    #[inline(always)]
    fn clear_last_error(&mut self) {
        self.last_error = None;
    }
}

static SERIALIZATION_STATE: Mutex<Option<SerializationState>> = Mutex::new(None);

#[inline(always)]
pub fn is_registered() -> bool {
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

pub fn registered_unique_id() -> Option<UniqueId> {
    SERIALIZATION_STATE
        .lock()
        .as_ref()
        .map(|state| state.driver.unique_id())
}

pub fn last_error() -> Option<RuntimeError> {
    SERIALIZATION_STATE
        .lock()
        .as_ref()
        .and_then(|state| state.last_error.clone())
}

pub fn take_last_error() -> Option<RuntimeError> {
    SERIALIZATION_STATE
        .lock()
        .as_mut()
        .and_then(|state| state.last_error.take())
}

pub fn with_model<T: Model, R>(f: impl FnOnce(&T) -> R) -> Result<R, ModelAccessError> {
    let state = &mut *SERIALIZATION_STATE.lock();
    let state = state.as_mut().ok_or(ModelAccessError::NotRegistered)?;
    let driver = state
        .driver
        .as_any()
        .downcast_ref::<ModelDriver<T>>()
        .ok_or(ModelAccessError::WrongModelType)?;
    Ok(f(&driver.state))
}

pub fn with_model_mut<T: Model, R>(f: impl FnOnce(&mut T) -> R) -> Result<R, ModelAccessError> {
    let state = &mut *SERIALIZATION_STATE.lock();
    let state = state.as_mut().ok_or(ModelAccessError::NotRegistered)?;
    let driver = state
        .driver
        .as_any_mut()
        .downcast_mut::<ModelDriver<T>>()
        .ok_or(ModelAccessError::WrongModelType)?;
    Ok(f(&mut driver.state))
}

#[inline(always)]
pub fn write_value_record<T: CosaveEncode>(
    id: RecordId,
    version: u32,
    value: &T,
) -> Result<OwnedRecord, SaveError> {
    OwnedRecord::from_value(id, version, value)
}

unsafe extern "system" fn save_callback(serialization: *mut SerializationInterface) {
    let Some(serialization) = (unsafe { serialization.as_ref() }) else {
        return;
    };

    let state = &mut *SERIALIZATION_STATE.lock();
    let Some(state) = state.as_mut() else {
        return;
    };

    state.clear_last_error();
    if let Err(error) = state.driver.save(serialization) {
        state.set_last_error(error);
    }
}

unsafe extern "system" fn load_callback(serialization: *mut SerializationInterface) {
    let Some(serialization) = (unsafe { serialization.as_ref() }) else {
        return;
    };

    let state = &mut *SERIALIZATION_STATE.lock();
    let Some(state) = state.as_mut() else {
        return;
    };

    state.clear_last_error();
    if let Err(error) = state.driver.load(serialization) {
        state.set_last_error(error);
    }
}

unsafe extern "system" fn revert_callback(_serialization: *mut SerializationInterface) {
    let state = &mut *SERIALIZATION_STATE.lock();
    let Some(state) = state.as_mut() else {
        return;
    };

    state.clear_last_error();
    state.driver.revert();
}

unsafe extern "system" fn form_delete_callback(handle: VMHandle) {
    let state = &mut *SERIALIZATION_STATE.lock();
    let Some(state) = state.as_mut() else {
        return;
    };

    state.driver.form_delete(handle);
}

#[cfg(test)]
mod tests {
    use alloc::collections::BTreeMap;
    use alloc::string::String;
    use alloc::vec;
    use alloc::vec::Vec;

    use super::*;

    #[derive(Cosave, Default, Debug, PartialEq, Eq)]
    struct DerivedEntry {
        label: String,
        values: BoundedVec<u32, 4>,
    }

    #[derive(Cosave, Default, Debug, PartialEq, Eq)]
    struct DerivedPayload {
        count: u32,
        entries: Vec<DerivedEntry>,
    }

    mod offset_u32_codec {
        use super::{LoadError, RecordReader, RecordWriter, SaveError};

        pub fn encode(value: &u32, writer: &mut RecordWriter) -> Result<(), SaveError> {
            writer.write_u32(*value + 10);
            Ok(())
        }

        pub fn decode(reader: &mut RecordReader<'_>) -> Result<u32, LoadError> {
            Ok(reader.read_u32()? - 10)
        }
    }

    mod failing_codec {
        use super::{LoadError, RecordReader, RecordWriter, SaveError};

        pub fn encode(_value: &u32, _writer: &mut RecordWriter) -> Result<(), SaveError> {
            Err(SaveError::LengthOverflow(123))
        }

        pub fn decode(_reader: &mut RecordReader<'_>) -> Result<u32, LoadError> {
            Err(LoadError::InvalidBool(2))
        }
    }

    #[derive(Cosave, Default, Debug, PartialEq, Eq)]
    struct DerivedAttrPayload {
        id: u32,
        #[cosave(skip)]
        transient: u32,
        #[cosave(with = offset_u32_codec)]
        encoded: u32,
    }

    #[derive(Cosave, Default, Debug, PartialEq, Eq)]
    struct DerivedDefaultPayload {
        id: u32,
        #[cosave(default)]
        title: String,
        #[cosave(default)]
        counts: [u32; 2],
    }

    #[derive(Cosave, Default, Debug, PartialEq, Eq)]
    struct DerivedFailPayload {
        #[cosave(with = failing_codec)]
        value: u32,
    }

    #[derive(Default)]
    struct TestState {
        count: u32,
        cache: BTreeMap<String, BoundedVec<u32, 4>>,
    }

    #[derive(Default)]
    struct DriverTestState {
        count: u32,
        title: String,
    }

    impl Model for DriverTestState {
        const UNIQUE_ID: UniqueId = unique_id!("DRVR");

        fn schema(schema: &mut Schema<Self>) {
            schema.value(
                record_id!("CNT1"),
                1,
                |state| &state.count,
                |state, value| {
                    state.count = value;
                },
            );
            schema.value(
                record_id!("TTL1"),
                1,
                |state| &state.title,
                |state, value| {
                    state.title = value;
                },
            );
        }
    }

    fn get_count(state: &TestState) -> &u32 {
        &state.count
    }

    fn set_count(state: &mut TestState, value: u32) {
        state.count = value;
    }

    fn get_cache(state: &TestState) -> &BTreeMap<String, BoundedVec<u32, 4>> {
        &state.cache
    }

    fn set_cache(state: &mut TestState, value: BTreeMap<String, BoundedVec<u32, 4>>) {
        state.cache = value;
    }

    fn save_double_count(state: &TestState, writer: &mut RecordWriter) -> Result<(), SaveError> {
        writer.write_u32(state.count * 2);
        Ok(())
    }

    fn load_double_count(
        state: &mut TestState,
        record: &LoadedRecord,
        context: LoadContext<'_>,
    ) -> Result<LoadStatus, LoadError> {
        if record.header().version() != 7 {
            return Ok(LoadStatus::Unhandled);
        }

        let mut reader = record.reader(context);
        state.count = reader.read_u32()? / 2;
        reader.finish()?;
        Ok(LoadStatus::Handled)
    }

    fn save_migrated_count(state: &TestState, writer: &mut RecordWriter) -> Result<(), SaveError> {
        writer.write_u32(state.count + 100);
        Ok(())
    }

    fn load_migrated_count_v1(
        state: &mut TestState,
        record: &LoadedRecord,
        context: LoadContext<'_>,
    ) -> Result<LoadStatus, LoadError> {
        let mut reader = record.reader(context);
        state.count = reader.read_u32()?;
        reader.finish()?;
        Ok(LoadStatus::Handled)
    }

    fn load_migrated_count_v2(
        state: &mut TestState,
        record: &LoadedRecord,
        context: LoadContext<'_>,
    ) -> Result<LoadStatus, LoadError> {
        let mut reader = record.reader(context);
        state.count = reader.read_u32()? - 100;
        reader.finish()?;
        Ok(LoadStatus::Handled)
    }

    #[test]
    fn schema_value_records_round_trip_nested_containers() {
        let mut schema = Schema::<TestState>::new();
        schema.value(record_id!("CNT1"), 1, get_count, set_count);
        schema.value(record_id!("MAP1"), 1, get_cache, set_cache);
        let built = schema.build().unwrap();

        let mut slots = BoundedVec::<u32, 4>::new();
        slots.push(3).unwrap();
        slots.push(5).unwrap();

        let mut state = TestState {
            count: 42,
            cache: BTreeMap::new(),
        };
        state.cache.insert(String::from("health"), slots);

        let records = built.save(&state).unwrap();
        assert_eq!(records.len(), 2);

        let mut loaded = TestState::default();
        for record in &records {
            let loaded_record = LoadedRecord::from_owned(record.clone());
            assert!(
                built
                    .try_load(&mut loaded, &loaded_record, LoadContext::empty())
                    .unwrap()
            );
        }

        assert_eq!(loaded.count, 42);
        assert_eq!(loaded.cache.get("health").unwrap().as_slice(), &[3, 5]);
    }

    #[test]
    fn schema_custom_records_receive_full_loaded_record() {
        let mut schema = Schema::<TestState>::new();
        schema.record(record_id!("DBL1"), 7, save_double_count, load_double_count);
        let built = schema.build().unwrap();

        let state = TestState {
            count: 21,
            cache: BTreeMap::new(),
        };

        let records = built.save(&state).unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].header().version(), 7);

        let loaded_record = LoadedRecord::from_owned(records[0].clone());
        let mut loaded = TestState::default();
        assert!(
            built
                .try_load(&mut loaded, &loaded_record, LoadContext::empty())
                .unwrap()
        );
        assert_eq!(loaded.count, 21);
    }

    #[test]
    fn duplicate_record_ids_are_rejected() {
        let mut schema = Schema::<TestState>::new();
        schema.value(record_id!("DUP1"), 1, get_count, set_count);
        schema.value(record_id!("DUP1"), 2, get_count, set_count);

        let error = match schema.build() {
            Ok(_) => panic!("duplicate record ids should fail schema construction"),
            Err(error) => error,
        };
        assert_eq!(
            error,
            SchemaBuildError::DuplicateRecordId(record_id!("DUP1"))
        );
    }

    #[test]
    fn migrating_records_dispatch_loaders_by_version() {
        let mut schema = Schema::<TestState>::new();
        schema
            .migrating_record(record_id!("MIG1"), 2, save_migrated_count)
            .load(1, load_migrated_count_v1)
            .load(2, load_migrated_count_v2);
        let built = schema.build().unwrap();

        let state = TestState {
            count: 42,
            cache: BTreeMap::new(),
        };
        let saved = built.save(&state).unwrap();
        assert_eq!(saved.len(), 1);
        assert_eq!(saved[0].header().version(), 2);

        let mut loaded_current = TestState::default();
        let saved_record = LoadedRecord::from_owned(saved[0].clone());
        assert!(
            built
                .try_load(&mut loaded_current, &saved_record, LoadContext::empty())
                .unwrap()
        );
        assert_eq!(loaded_current.count, 42);

        let mut legacy_writer = RecordWriter::new();
        legacy_writer.write_u32(17);
        let legacy_record = legacy_writer
            .into_record(record_id!("MIG1"), 1)
            .unwrap()
            .into_loaded();

        let mut loaded_legacy = TestState::default();
        assert!(
            built
                .try_load(&mut loaded_legacy, &legacy_record, LoadContext::empty())
                .unwrap()
        );
        assert_eq!(loaded_legacy.count, 17);
    }

    #[test]
    fn duplicate_migration_versions_are_rejected() {
        let mut schema = Schema::<TestState>::new();
        schema
            .migrating_record(record_id!("MIG2"), 2, save_migrated_count)
            .load(1, load_migrated_count_v1)
            .load(1, load_migrated_count_v1);

        let error = match schema.build() {
            Ok(_) => panic!("duplicate migration versions should fail schema construction"),
            Err(error) => error,
        };
        assert_eq!(
            error,
            SchemaBuildError::DuplicateRecordVersion {
                id: record_id!("MIG2"),
                version: 1,
            }
        );
    }

    #[test]
    fn derive_cosave_round_trips_nested_structs() {
        let mut bounded = BoundedVec::<u32, 4>::new();
        bounded.push(7).unwrap();
        bounded.push(9).unwrap();

        let payload = DerivedPayload {
            count: 2,
            entries: vec![DerivedEntry {
                label: String::from("health"),
                values: bounded,
            }],
        };

        let mut writer = RecordWriter::new();
        payload.encode(&mut writer).unwrap();

        let mut reader = RecordReader::new(writer.bytes(), LoadContext::empty());
        let decoded = DerivedPayload::decode(&mut reader).unwrap();
        reader.finish().unwrap();

        assert_eq!(decoded, payload);
    }

    #[test]
    fn derive_cosave_supports_skip_and_with_attributes() {
        let payload = DerivedAttrPayload {
            id: 7,
            transient: 99,
            encoded: 42,
        };

        let mut writer = RecordWriter::new();
        payload.encode(&mut writer).unwrap();

        let mut reader = RecordReader::new(writer.bytes(), LoadContext::empty());
        let decoded = DerivedAttrPayload::decode(&mut reader).unwrap();
        reader.finish().unwrap();

        assert_eq!(
            decoded,
            DerivedAttrPayload {
                id: 7,
                transient: 0,
                encoded: 42,
            }
        );
    }

    #[test]
    fn derive_cosave_default_attribute_fills_missing_trailing_fields() {
        let mut writer = RecordWriter::new();
        11_u32.encode(&mut writer).unwrap();

        let mut reader = RecordReader::new(writer.bytes(), LoadContext::empty());
        let decoded = DerivedDefaultPayload::decode(&mut reader).unwrap();
        reader.finish().unwrap();

        assert_eq!(
            decoded,
            DerivedDefaultPayload {
                id: 11,
                title: String::new(),
                counts: [0, 0],
            }
        );
    }

    #[test]
    fn derive_cosave_wraps_field_errors() {
        let payload = DerivedFailPayload { value: 3 };
        let mut writer = RecordWriter::new();
        let save_error = payload.encode(&mut writer).unwrap_err();
        assert_eq!(
            save_error,
            SaveError::Field {
                field: "value",
                source: alloc::boxed::Box::new(SaveError::LengthOverflow(123)),
            }
        );

        let mut reader = RecordReader::new(&[], LoadContext::empty());
        let load_error = DerivedFailPayload::decode(&mut reader).unwrap_err();
        assert_eq!(
            load_error,
            LoadError::Field {
                field: "value",
                source: alloc::boxed::Box::new(LoadError::InvalidBool(2)),
            }
        );
    }

    #[test]
    fn schema_fields_expands_value_records() {
        let mut schema = Schema::<TestState>::new();
        crate::schema_fields!(schema, {
            record_id!("CNT1") => 1 => count,
            record_id!("MAP1") => 1 => cache,
        });

        let built = schema.build().unwrap();
        assert!(built.contains_id(record_id!("CNT1")));
        assert!(built.contains_id(record_id!("MAP1")));
    }

    #[test]
    fn model_driver_load_is_transactional_on_failure() {
        let mut driver = ModelDriver::<DriverTestState>::new().unwrap();
        driver.state.count = 77;
        driver.state.title = String::from("before");

        let unknown = OwnedRecord::new(record_id!("UNKN"), 9, vec![0xAA, 0xBB])
            .unwrap()
            .into_loaded();
        driver.unknown_records.push(unknown.clone());

        let count_record = OwnedRecord::from_value(record_id!("CNT1"), 1, &12_u32)
            .unwrap()
            .into_loaded();

        let mut invalid_title_writer = RecordWriter::new();
        invalid_title_writer.write_u32(1);
        invalid_title_writer.write_u8(0xFF);
        let invalid_title_record = invalid_title_writer
            .into_record(record_id!("TTL1"), 1)
            .unwrap()
            .into_loaded();

        let error = driver
            .load_records(
                vec![count_record, invalid_title_record],
                LoadContext::empty(),
            )
            .unwrap_err();

        assert!(matches!(
            error,
            RuntimeError::LoadRecord { header, .. } if header.id() == record_id!("TTL1")
        ));
        assert_eq!(driver.state.count, 77);
        assert_eq!(driver.state.title, "before");
        assert_eq!(driver.unknown_records, vec![unknown]);
    }

    #[test]
    fn model_driver_save_preserves_unknown_passthrough_records() {
        let mut driver = ModelDriver::<DriverTestState>::new().unwrap();
        driver.state.count = 5;
        driver.state.title = String::from("persisted");

        let unknown = OwnedRecord::new(record_id!("UNKN"), 9, vec![0x10, 0x20, 0x30])
            .unwrap()
            .into_loaded();
        driver.unknown_records.push(unknown.clone());

        let records = driver.build_save_records().unwrap();
        assert_eq!(
            records
                .iter()
                .map(|record| record.header().id())
                .collect::<Vec<_>>(),
            vec![record_id!("CNT1"), record_id!("TTL1"), record_id!("UNKN")]
        );

        let passthrough = records
            .iter()
            .find(|record| record.header().id() == record_id!("UNKN"))
            .unwrap();
        assert_eq!(passthrough.header().version(), 9);
        assert_eq!(passthrough.bytes(), unknown.bytes());
    }
}
