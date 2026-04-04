extern crate std;

use alloc::vec::Vec;
use core::ffi::c_void;
use core::mem::size_of;
use std::cell::RefCell;

use super::{
    PapyrusEventLoadStatus, PapyrusEventRecord, PapyrusEventRegistry,
    PapyrusPersistentEventRegistry, PapyrusTargetedEventRegistry, load_record, revert_registries,
    save_registries,
};
use crate::re::bs_core_types::FormID;
use crate::re::bs_core_types::VMHandle;
use crate::skse::SerializationInterface;

#[derive(Default)]
struct TestRecord {
    ty: u32,
    version: u32,
    data: Vec<u8>,
}

#[derive(Default)]
struct TestSerializationState {
    read_bytes: Vec<u8>,
    read_offset: usize,
    current_record: Option<TestRecord>,
    written_records: Vec<TestRecord>,
}

std::thread_local! {
    static TEST_SERIALIZATION_STATE: RefCell<Option<TestSerializationState>> =
        const { RefCell::new(None) };
}

fn with_test_state<R>(f: impl FnOnce(&mut TestSerializationState) -> R) -> R {
    TEST_SERIALIZATION_STATE.with(|state| {
        let mut state = state.borrow_mut();
        let state = state.as_mut().expect("missing serialization test state");
        f(state)
    })
}

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
    ty: u32,
    version: u32,
    buf: *const c_void,
    length: u32,
) -> bool {
    let bytes = unsafe { core::slice::from_raw_parts(buf.cast::<u8>(), length as usize) };
    with_test_state(|state| {
        state.written_records.push(TestRecord {
            ty,
            version,
            data: bytes.to_vec(),
        });
        true
    })
}

unsafe extern "system" fn test_open_record(ty: u32, version: u32) -> bool {
    with_test_state(|state| {
        if let Some(record) = state.current_record.take() {
            state.written_records.push(record);
        }
        state.current_record = Some(TestRecord {
            ty,
            version,
            data: Vec::new(),
        });
        true
    })
}

unsafe extern "system" fn test_write_record_data(buf: *const c_void, length: u32) -> bool {
    let bytes = unsafe { core::slice::from_raw_parts(buf.cast::<u8>(), length as usize) };
    with_test_state(|state| {
        let Some(record) = state.current_record.as_mut() else {
            return false;
        };
        record.data.extend_from_slice(bytes);
        true
    })
}

unsafe extern "system" fn test_get_next_record_info(
    _ty: *mut u32,
    _version: *mut u32,
    _length: *mut u32,
) -> bool {
    false
}

unsafe extern "system" fn test_read_record_data(buf: *mut c_void, length: u32) -> u32 {
    with_test_state(|state| {
        let start = state.read_offset;
        let end = start + length as usize;
        let bytes = &state.read_bytes[start..end];
        unsafe {
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), buf.cast::<u8>(), bytes.len());
        }
        state.read_offset = end;
        length
    })
}

unsafe extern "system" fn test_resolve_handle(
    old_handle: VMHandle,
    new_handle: *mut VMHandle,
) -> bool {
    if old_handle == 0 {
        return false;
    }

    unsafe {
        *new_handle = old_handle.wrapping_add(1);
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
        *new_form_id = old_form_id.wrapping_add(2);
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

fn reset_serialization_state(read_bytes: Vec<u8>) {
    TEST_SERIALIZATION_STATE.with(|state| {
        *state.borrow_mut() = Some(TestSerializationState {
            read_bytes,
            ..TestSerializationState::default()
        });
    });
}

fn take_written_records() -> Vec<(u32, u32, Vec<u8>)> {
    with_test_state(|state| {
        if let Some(record) = state.current_record.take() {
            state.written_records.push(record);
        }

        state
            .written_records
            .drain(..)
            .map(|record| (record.ty, record.version, record.data))
            .collect()
    })
}

fn push_usize(bytes: &mut Vec<u8>, value: usize) {
    bytes.extend_from_slice(&value.to_ne_bytes());
}

fn push_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_ne_bytes());
}

fn push_vm_handle(bytes: &mut Vec<u8>, value: VMHandle) {
    bytes.extend_from_slice(&value.to_ne_bytes());
}

fn snapshot_handles(registry: &PapyrusEventRegistry<()>) -> Vec<VMHandle> {
    let mut handles = Vec::new();
    registry.for_each_handle(|handle| handles.push(handle));
    handles
}

fn snapshot_target_handles(
    registry: &PapyrusTargetedEventRegistry<()>,
    target_id: u32,
) -> Vec<VMHandle> {
    let mut handles = Vec::new();
    registry.for_each_target_handle(target_id, |handle| handles.push(handle));
    handles
}

#[test]
fn grouped_load_routes_to_matching_registry() {
    let serialization = test_serialization_interface();
    let mut first =
        PapyrusEventRegistry::<()>::persistent("OnFirst", PapyrusEventRecord::new(0x1111_1111, 1));
    let mut second =
        PapyrusEventRegistry::<()>::persistent("OnSecond", PapyrusEventRecord::new(0x2222_2222, 1));

    let mut read_bytes = Vec::new();
    push_usize(&mut read_bytes, 2);
    push_vm_handle(&mut read_bytes, 10);
    push_vm_handle(&mut read_bytes, 20);
    reset_serialization_state(read_bytes);

    let mut registries: [&mut dyn PapyrusPersistentEventRegistry; 2] = [&mut first, &mut second];
    let status = load_record(0x2222_2222, &serialization, &mut registries);

    assert_eq!(status, PapyrusEventLoadStatus::Loaded);
    assert!(snapshot_handles(&first).is_empty());
    assert_eq!(snapshot_handles(&second), [11, 21]);
}

#[test]
fn grouped_save_writes_record_headers() {
    let serialization = test_serialization_interface();
    let mut regular = PapyrusEventRegistry::<()>::persistent(
        "OnRegular",
        PapyrusEventRecord::new(0xAAAA_0001, 7),
    );
    let mut targeted = PapyrusTargetedEventRegistry::<()>::persistent(
        "OnTargeted",
        PapyrusEventRecord::new(0xBBBB_0002, 9),
    );

    let mut regular_bytes = Vec::new();
    push_usize(&mut regular_bytes, 1);
    push_vm_handle(&mut regular_bytes, 40);
    reset_serialization_state(regular_bytes);
    assert_eq!(
        regular.load_record(0xAAAA_0001, &serialization),
        PapyrusEventLoadStatus::Loaded
    );

    let mut targeted_bytes = Vec::new();
    push_usize(&mut targeted_bytes, 1);
    push_u32(&mut targeted_bytes, 0x1234);
    push_usize(&mut targeted_bytes, 1);
    push_vm_handle(&mut targeted_bytes, 80);
    reset_serialization_state(targeted_bytes);
    assert_eq!(
        targeted.load_record(0xBBBB_0002, &serialization),
        PapyrusEventLoadStatus::Loaded
    );

    reset_serialization_state(Vec::new());
    let registries: [&dyn PapyrusPersistentEventRegistry; 2] = [&regular, &targeted];
    assert!(save_registries(&serialization, &registries));

    let written = take_written_records();
    assert_eq!(written.len(), 2);
    assert_eq!(written[0].0, 0xAAAA_0001);
    assert_eq!(written[0].1, 7);
    assert_eq!(written[1].0, 0xBBBB_0002);
    assert_eq!(written[1].1, 9);
    assert_eq!(
        written[0].2.len(),
        size_of::<usize>() + size_of::<VMHandle>()
    );
    assert_eq!(
        written[1].2.len(),
        size_of::<usize>() + size_of::<u32>() + size_of::<usize>() + size_of::<VMHandle>()
    );
}

#[test]
fn revert_clears_loaded_registrations() {
    let serialization = test_serialization_interface();
    let mut regular = PapyrusEventRegistry::<()>::persistent(
        "OnRegular",
        PapyrusEventRecord::new(0xA0A0_A0A0, 1),
    );
    let mut targeted = PapyrusTargetedEventRegistry::<()>::persistent(
        "OnTargeted",
        PapyrusEventRecord::new(0xB0B0_B0B0, 1),
    );

    let mut regular_bytes = Vec::new();
    push_usize(&mut regular_bytes, 1);
    push_vm_handle(&mut regular_bytes, 5);
    reset_serialization_state(regular_bytes);
    assert_eq!(
        regular.load_record(0xA0A0_A0A0, &serialization),
        PapyrusEventLoadStatus::Loaded
    );

    let mut targeted_bytes = Vec::new();
    push_usize(&mut targeted_bytes, 1);
    push_u32(&mut targeted_bytes, 0x0100);
    push_usize(&mut targeted_bytes, 2);
    push_vm_handle(&mut targeted_bytes, 7);
    push_vm_handle(&mut targeted_bytes, 8);
    reset_serialization_state(targeted_bytes);
    assert_eq!(
        targeted.load_record(0xB0B0_B0B0, &serialization),
        PapyrusEventLoadStatus::Loaded
    );

    let mut registries: [&mut dyn PapyrusPersistentEventRegistry; 2] =
        [&mut regular, &mut targeted];
    revert_registries(None, &mut registries);

    assert!(snapshot_handles(&regular).is_empty());
    assert!(snapshot_target_handles(&targeted, 0x0102).is_empty());
}
