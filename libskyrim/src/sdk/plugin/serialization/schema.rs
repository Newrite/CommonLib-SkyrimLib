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

#[inline(always)]
pub fn write_value_record<T: CosaveEncode>(
    id: RecordId,
    version: u32,
    value: &T,
) -> Result<OwnedRecord, SaveError> {
    OwnedRecord::from_value(id, version, value)
}
