use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use super::runtime::ModelDriver;
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
