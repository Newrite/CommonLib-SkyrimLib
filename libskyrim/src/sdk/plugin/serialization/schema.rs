//! Declarative schema builders for plugin serialization models.
//!
//! This module sits between low-level `cosave` records and the registered-model
//! runtime in [`super::runtime`]. It answers one question: "how does this
//! plugin-owned model map onto named records and versions?"
//!
//! Decision guide:
//!
//! - use [`Schema::value`] when one model field already implements
//!   [`CosaveEncode`] / [`CosaveDecode`] and can live in its own record;
//! - use [`Schema::record`] when save/load logic needs a custom binary layout or
//!   direct access to the whole [`LoadedRecord`];
//! - use [`Schema::migrating_record`] when one stable record ID must continue to
//!   accept older on-disk versions during load;
//! - use [`write_value_record`] for tests or ad-hoc helpers that want one typed
//!   record without defining a full [`Schema`].
//!
//! Typical schema sketch:
//!
//! ```rust,ignore
//! use libskyrim::sdk::plugin::serialization::{
//!     self, LoadContext, LoadStatus, LoadedRecord, RecordWriter, Schema,
//! };
//!
//! #[derive(Default)]
//! struct SaveState {
//!     counter: u32,
//!     tags: Vec<String>,
//! }
//!
//! fn build_schema(schema: &mut Schema<SaveState>) {
//!     schema.value(serialization::record_id!("CNT1"), 1, |s| &s.counter, |s, v| s.counter = v);
//!
//!     schema
//!         .migrating_record(serialization::record_id!("TAGS"), 2, |state, writer| {
//!             writer.write_value(&state.tags)
//!         })
//!         .load(1, |state, record, context| {
//!             state.tags = record.decode::<Vec<String>>(context)?;
//!             Ok(LoadStatus::Handled)
//!         })
//!         .load(2, |state, record, context| {
//!             state.tags = record.decode::<Vec<String>>(context)?;
//!             Ok(LoadStatus::Handled)
//!         })
//!         .finish();
//! }
//! ```

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::marker::PhantomData;

use super::types::RuntimeError;
use super::{
    CosaveDecode, CosaveEncode, LoadContext, LoadError, LoadStatus, LoadedRecord, OwnedRecord,
    RecordId, RecordWriter, SaveError, SchemaBuildError,
};

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

/// Builder returned by [`Schema::migrating_record`] for one record ID that
/// supports multiple historical load versions.
///
/// This is the right tool when a plugin wants to keep one stable record ID but
/// accept several older on-disk versions during load.
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

    /// Adds a loader for one historical record version.
    ///
    /// Register loaders in the versions you still want to accept on disk. The
    /// current save path always uses the `save_version` passed to
    /// [`Schema::migrating_record`].
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

    /// Finalizes the builder and returns the parent schema.
    ///
    /// Calling this is optional because the builder also auto-finalizes on
    /// drop, but an explicit `finish()` can make longer schema definitions read
    /// more clearly.
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

/// Declarative serialization schema for a plugin [`Model`](super::Model).
///
/// Use [`Schema::value`] for plain encoded fields, [`Schema::record`] for
/// custom binary records, and [`Schema::migrating_record`] when a record ID
/// needs multiple historical loaders.
pub struct Schema<T> {
    records: Vec<Box<dyn RecordCodec<T> + Send>>,
    error: Option<SchemaBuildError>,
}

impl<T> Schema<T> {
    /// Construct an empty schema builder.
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
            error: None,
        }
    }

    /// Registers a simple field-backed record.
    ///
    /// This is the most common entrypoint for plain value-like model fields
    /// that already implement [`CosaveEncode`] and [`CosaveDecode`].
    ///
    /// ```rust,ignore
    /// # use libskyrim::sdk::plugin::serialization::{RecordId, Schema};
    /// #[derive(Default)]
    /// struct SaveState {
    ///     counter: u32,
    /// }
    ///
    /// let mut schema = Schema::<SaveState>::new();
    /// schema.value(RecordId::from_bytes(*b"CNT1"), 1, |s| &s.counter, |s, v| s.counter = v);
    /// ```
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

    /// Registers a custom save/load record pair.
    ///
    /// Use this when a record's binary layout is not a simple one-field value
    /// or when loading needs direct access to the full [`LoadedRecord`].
    ///
    /// ```rust,ignore
    /// # use libskyrim::sdk::plugin::serialization::{
    /// #     LoadContext, LoadStatus, LoadedRecord, RecordId, RecordWriter, Schema,
    /// # };
    /// #[derive(Default)]
    /// struct SaveState {
    ///     first: u32,
    ///     second: u32,
    /// }
    ///
    /// let mut schema = Schema::<SaveState>::new();
    /// schema.record(
    ///     RecordId::from_bytes(*b"PAIR"),
    ///     1,
    ///     |state, writer| {
    ///         writer.write_u32(state.first);
    ///         writer.write_u32(state.second);
    ///         Ok(())
    ///     },
    ///     |state, record, _context| {
    ///         let mut reader = record.reader(LoadContext::empty());
    ///         state.first = reader.read_u32()?;
    ///         state.second = reader.read_u32()?;
    ///         reader.finish()?;
    ///         Ok(LoadStatus::Handled)
    ///     },
    /// );
    /// ```
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

    /// Begins building a versioned custom record with historical loaders.
    ///
    /// This is the normal entry point for compatibility-preserving migrations:
    /// keep one stable record ID for current saves, but accept several historic
    /// versions during load through [`MigratingRecordBuilder::load`].
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

    pub(super) fn build(mut self) -> Result<BuiltSchema<T>, SchemaBuildError> {
        if let Some(error) = self.error.take() {
            return Err(error);
        }

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

pub(super) struct BuiltSchema<T> {
    records: Vec<Box<dyn RecordCodec<T> + Send>>,
}

impl<T> BuiltSchema<T> {
    pub(super) fn save(&self, state: &T) -> Result<Vec<OwnedRecord>, RuntimeError> {
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

    pub(super) fn try_load(
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
    pub(super) fn contains_id(&self, id: RecordId) -> bool {
        self.records.iter().any(|record| record.id() == id)
    }
}

/// Encode a single value as a standalone record without building a full
/// [`Schema`].
///
/// This is useful for tests, ad-hoc utilities, or helper code that wants one
/// typed record but does not need a full plugin [`Model`](super::Model).
///
/// ```rust,ignore
/// # use libskyrim::sdk::plugin::serialization::{RecordId, write_value_record};
/// let record = write_value_record(RecordId::from_bytes(*b"CNT1"), 1, &42_u32)?;
/// assert_eq!(record.header().version(), 1);
/// # Ok::<(), libskyrim::sdk::plugin::serialization::SaveError>(())
/// ```
#[inline(always)]
pub fn write_value_record<T: CosaveEncode>(
    id: RecordId,
    version: u32,
    value: &T,
) -> Result<OwnedRecord, SaveError> {
    OwnedRecord::from_value(id, version, value)
}
