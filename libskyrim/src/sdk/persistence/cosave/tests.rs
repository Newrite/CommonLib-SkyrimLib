use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use crate::re::bs_core_types::{FormID, VMHandle};
use crate::skse::SerializationInterface;

use super::*;

const TEST_PLUGIN_HANDLE_RAW: u32 = 1;

unsafe extern "system" fn test_set_unique_id(_plugin: crate::skse::PluginHandle, _uid: u32) {}

unsafe extern "system" fn test_set_event_callback(
    _plugin: crate::skse::PluginHandle,
    _callback: Option<crate::skse::SerializationEventCallback>,
) {
}

unsafe extern "system" fn test_set_form_delete_callback(
    _plugin: crate::skse::PluginHandle,
    _callback: Option<crate::skse::FormDeleteCallback>,
) {
}

unsafe extern "system" fn test_write_record(
    _ty: u32,
    _version: u32,
    _buf: *const core::ffi::c_void,
    _length: u32,
) -> bool {
    false
}

unsafe extern "system" fn test_open_record(_ty: u32, _version: u32) -> bool {
    false
}

unsafe extern "system" fn test_write_record_data(
    _buf: *const core::ffi::c_void,
    _length: u32,
) -> bool {
    false
}

unsafe extern "system" fn test_get_next_record_info(
    _ty: *mut u32,
    _version: *mut u32,
    _length: *mut u32,
) -> bool {
    false
}

unsafe extern "system" fn test_read_record_data(_buf: *mut core::ffi::c_void, _length: u32) -> u32 {
    0
}

unsafe extern "system" fn test_resolve_handle(
    old_handle: VMHandle,
    new_handle: *mut VMHandle,
) -> bool {
    if old_handle == 0 {
        return false;
    }

    unsafe {
        *new_handle = old_handle.wrapping_add(TEST_PLUGIN_HANDLE_RAW as u64);
    }
    true
}

unsafe extern "system" fn test_resolve_form_id(
    old_form_id: FormID,
    new_form_id: *mut FormID,
) -> bool {
    if old_form_id == 0 {
        return false;
    }

    unsafe {
        *new_form_id = old_form_id.wrapping_add(TEST_PLUGIN_HANDLE_RAW);
    }
    true
}

fn test_serialization_interface() -> SerializationInterface {
    SerializationInterface {
        interface_version: 4,
        set_unique_id: test_set_unique_id,
        set_revert_callback: test_set_event_callback,
        set_save_callback: test_set_event_callback,
        set_load_callback: test_set_event_callback,
        set_form_delete_callback: test_set_form_delete_callback,
        write_record: test_write_record,
        open_record: test_open_record,
        write_record_data: test_write_record_data,
        get_next_record_info: test_get_next_record_info,
        read_record_data: test_read_record_data,
        resolve_handle: test_resolve_handle,
        resolve_form_id: test_resolve_form_id,
    }
}

#[test]
fn vec_round_trip() {
    let value = vec![1_u32, 2, 3, 5, 8];
    let mut writer = RecordWriter::new();
    value.encode(&mut writer).unwrap();

    let mut reader = RecordReader::new(writer.bytes(), LoadContext::empty());
    let decoded = Vec::<u32>::decode(&mut reader).unwrap();
    reader.finish().unwrap();

    assert_eq!(decoded, value);
}

#[test]
fn bounded_vec_round_trip() {
    let mut value = BoundedVec::<u32, 4>::new();
    value.push(10).unwrap();
    value.push(20).unwrap();

    let mut writer = RecordWriter::new();
    value.encode(&mut writer).unwrap();

    let mut reader = RecordReader::new(writer.bytes(), LoadContext::empty());
    let decoded = BoundedVec::<u32, 4>::decode(&mut reader).unwrap();
    reader.finish().unwrap();

    assert_eq!(decoded.as_slice(), value.as_slice());
}

#[test]
fn bounded_vec_rejects_oversized_payloads() {
    let mut writer = RecordWriter::new();
    writer.write_u32(3);
    writer.write_u32(1);
    writer.write_u32(2);
    writer.write_u32(3);

    let mut reader = RecordReader::new(writer.bytes(), LoadContext::empty());
    let error = BoundedVec::<u32, 2>::decode(&mut reader).unwrap_err();
    assert_eq!(error, LoadError::VectorTooLong { len: 3, max: 2 });
}

#[test]
fn btree_map_round_trip() {
    let mut value = BTreeMap::new();
    value.insert(String::from("health"), 5_u32);
    value.insert(String::from("stamina"), 7_u32);

    let mut writer = RecordWriter::new();
    value.encode(&mut writer).unwrap();

    let mut reader = RecordReader::new(writer.bytes(), LoadContext::empty());
    let decoded = BTreeMap::<String, u32>::decode(&mut reader).unwrap();
    reader.finish().unwrap();

    assert_eq!(decoded, value);
}

#[test]
fn fourcc_literals_are_const_checked() {
    const UNIQUE: UniqueId = unique_id!("TFNG");
    const RECORD: RecordId = record_id!("CDAD");

    assert_eq!(UNIQUE.to_bytes(), *b"TFNG");
    assert_eq!(RECORD.to_bytes(), *b"CDAD");
}

#[test]
fn resolved_form_id_decodes_through_context() {
    let interface = test_serialization_interface();
    let context = LoadContext::new(&interface);

    let mut writer = RecordWriter::new();
    writer.write_form_id(0x1000_0042);

    let mut reader = RecordReader::new(writer.bytes(), context);
    let decoded = ResolvedFormId::decode(&mut reader).unwrap();
    reader.finish().unwrap();

    assert_eq!(decoded.get(), 0x1000_0043);
}

#[test]
fn resolved_vm_handle_decodes_through_context() {
    let interface = test_serialization_interface();
    let context = LoadContext::new(&interface);

    let mut writer = RecordWriter::new();
    writer.write_vm_handle(0x0000_0000_0000_1000);

    let mut reader = RecordReader::new(writer.bytes(), context);
    let decoded = ResolvedVmHandle::decode(&mut reader).unwrap();
    reader.finish().unwrap();

    assert_eq!(decoded.get(), 0x0000_0000_0000_1001);
}

#[test]
fn option_codec_round_trips_some_and_none() {
    let some = Some(String::from("flask"));
    let none: Option<u32> = None;

    let mut writer = RecordWriter::new();
    some.encode(&mut writer).unwrap();
    none.encode(&mut writer).unwrap();

    let mut reader = RecordReader::new(writer.bytes(), LoadContext::empty());
    let decoded_some = Option::<String>::decode(&mut reader).unwrap();
    let decoded_none = Option::<u32>::decode(&mut reader).unwrap();
    reader.finish().unwrap();

    assert_eq!(decoded_some, some);
    assert_eq!(decoded_none, none);
}

#[test]
fn fixed_array_codec_round_trips() {
    let values = [3_u32, 5, 8, 13];

    let mut writer = RecordWriter::new();
    values.encode(&mut writer).unwrap();

    let mut reader = RecordReader::new(writer.bytes(), LoadContext::empty());
    let decoded = <[u32; 4]>::decode(&mut reader).unwrap();
    reader.finish().unwrap();

    assert_eq!(decoded, values);
}

#[test]
fn vec_decode_reports_failing_element_index() {
    let mut writer = RecordWriter::new();
    writer.write_len(2).unwrap();
    writer.write_bool(true);
    writer.write_u8(2);

    let mut reader = RecordReader::new(writer.bytes(), LoadContext::empty());
    let error = Vec::<bool>::decode(&mut reader).unwrap_err();
    assert_eq!(
        error,
        LoadError::SequenceElement {
            index: 1,
            source: Box::new(LoadError::InvalidBool(2)),
        }
    );
}

#[test]
fn btreemap_decode_reports_failing_key_and_value_entries() {
    let mut key_writer = RecordWriter::new();
    key_writer.write_len(1).unwrap();
    key_writer.write_u8(2);
    key_writer.write_u32(10);

    let mut key_reader = RecordReader::new(key_writer.bytes(), LoadContext::empty());
    let key_error = BTreeMap::<bool, u32>::decode(&mut key_reader).unwrap_err();
    assert_eq!(
        key_error,
        LoadError::MapKey {
            entry: 0,
            source: Box::new(LoadError::InvalidBool(2)),
        }
    );

    let mut value_writer = RecordWriter::new();
    value_writer.write_len(1).unwrap();
    value_writer.write_bool(true);
    value_writer.write_u8(2);

    let mut value_reader = RecordReader::new(value_writer.bytes(), LoadContext::empty());
    let value_error = BTreeMap::<bool, bool>::decode(&mut value_reader).unwrap_err();
    assert_eq!(
        value_error,
        LoadError::MapValue {
            entry: 0,
            source: Box::new(LoadError::InvalidBool(2)),
        }
    );
}
