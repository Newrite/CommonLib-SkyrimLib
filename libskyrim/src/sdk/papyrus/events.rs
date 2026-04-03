//! High-level Papyrus event registration helpers.
//!
//! This module builds a more domain-oriented SDK surface on top of
//! `skse::RegistrationSet*`:
//!
//! - event-name-first registries for form / alias / active-effect listeners
//! - optional persistent record metadata for save/load/revert flows
//! - grouped helper functions for plugin serialization callbacks
//! - queued dispatch helpers for "register now, run later" event delivery

use alloc::string::String;

use crate::re::{ActiveEffect, BGSBaseAlias, BGSRefAlias, TESForm, TESObjectREFR, VMHandle};
use crate::sdk::core::GamePtr;
use crate::skse::{RegistrationSet, RegistrationSetUnique, SerializationInterface};

pub use crate::skse::RegistrationEventArgs;

/// Persistent record metadata for a Papyrus event registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PapyrusEventRecord {
    pub ty: u32,
    pub version: u32,
}

impl PapyrusEventRecord {
    #[inline(always)]
    pub const fn new(ty: u32, version: u32) -> Self {
        Self { ty, version }
    }
}

/// Result of trying to load a serialized Papyrus event record.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PapyrusEventLoadStatus {
    Loaded,
    Ignored,
    Failed,
}

/// Object-safe persistence surface shared by SDK Papyrus event registries.
///
/// Each registry should own a distinct `PapyrusEventRecord::ty` when used with
/// grouped `save_registries(...)` / `load_record(...)` helpers.
pub trait PapyrusPersistentEventRegistry {
    fn event_name(&self) -> &str;
    fn record(&self) -> Option<PapyrusEventRecord>;
    fn save_record(&self, serialization: &SerializationInterface) -> bool;
    fn load_record(
        &mut self,
        ty: u32,
        serialization: &SerializationInterface,
    ) -> PapyrusEventLoadStatus;
    fn revert(&mut self, serialization: Option<&SerializationInterface>);
    fn form_delete(&mut self, handle: VMHandle) -> bool;
}

/// Grouped collection of one or more persistent Papyrus event registries.
///
/// This trait is the bridge between per-event registries and runtime callback
/// installers that need to save/load/revert/form-delete a whole registry set.
pub trait PapyrusEventCollection {
    fn save_events(&self, serialization: &SerializationInterface) -> bool;
    fn load_event_record(
        &mut self,
        ty: u32,
        serialization: &SerializationInterface,
    ) -> PapyrusEventLoadStatus;
    fn revert_events(&mut self, serialization: Option<&SerializationInterface>);
    fn form_delete_events(&mut self, handle: VMHandle) -> usize;
}

impl PapyrusEventCollection for () {
    #[inline(always)]
    fn save_events(&self, _serialization: &SerializationInterface) -> bool {
        true
    }

    #[inline(always)]
    fn load_event_record(
        &mut self,
        _ty: u32,
        _serialization: &SerializationInterface,
    ) -> PapyrusEventLoadStatus {
        PapyrusEventLoadStatus::Ignored
    }

    #[inline(always)]
    fn revert_events(&mut self, _serialization: Option<&SerializationInterface>) {}

    #[inline(always)]
    fn form_delete_events(&mut self, _handle: VMHandle) -> usize {
        0
    }
}

impl<T> PapyrusEventCollection for T
where
    T: PapyrusPersistentEventRegistry,
{
    #[inline(always)]
    fn save_events(&self, serialization: &SerializationInterface) -> bool {
        self.save_record(serialization)
    }

    #[inline(always)]
    fn load_event_record(
        &mut self,
        ty: u32,
        serialization: &SerializationInterface,
    ) -> PapyrusEventLoadStatus {
        self.load_record(ty, serialization)
    }

    #[inline(always)]
    fn revert_events(&mut self, serialization: Option<&SerializationInterface>) {
        self.revert(serialization);
    }

    #[inline(always)]
    fn form_delete_events(&mut self, handle: VMHandle) -> usize {
        usize::from(self.form_delete(handle))
    }
}

macro_rules! impl_papyrus_event_collection_tuple {
    ($count:expr; $(($index:tt, $ty:ident)),+ $(,)?) => {
        impl<$($ty),+> PapyrusEventCollection for ($($ty,)+)
        where
            $($ty: PapyrusPersistentEventRegistry,)+
        {
            #[inline(always)]
            fn save_events(&self, serialization: &SerializationInterface) -> bool {
                let registries: [&dyn PapyrusPersistentEventRegistry; $count] = [
                    $(&self.$index),+
                ];
                save_registries(serialization, &registries)
            }

            #[inline(always)]
            fn load_event_record(
                &mut self,
                ty: u32,
                serialization: &SerializationInterface,
            ) -> PapyrusEventLoadStatus {
                let mut registries: [&mut dyn PapyrusPersistentEventRegistry; $count] = [
                    $(&mut self.$index),+
                ];
                load_record(ty, serialization, &mut registries)
            }

            #[inline(always)]
            fn revert_events(&mut self, serialization: Option<&SerializationInterface>) {
                let mut registries: [&mut dyn PapyrusPersistentEventRegistry; $count] = [
                    $(&mut self.$index),+
                ];
                revert_registries(serialization, &mut registries);
            }

            #[inline(always)]
            fn form_delete_events(&mut self, handle: VMHandle) -> usize {
                let mut registries: [&mut dyn PapyrusPersistentEventRegistry; $count] = [
                    $(&mut self.$index),+
                ];
                form_delete_registries(handle, &mut registries)
            }
        }
    };
}

impl_papyrus_event_collection_tuple!(2; (0, A0), (1, A1));
impl_papyrus_event_collection_tuple!(3; (0, A0), (1, A1), (2, A2));
impl_papyrus_event_collection_tuple!(4; (0, A0), (1, A1), (2, A2), (3, A3));
impl_papyrus_event_collection_tuple!(5; (0, A0), (1, A1), (2, A2), (3, A3), (4, A4));
impl_papyrus_event_collection_tuple!(6; (0, A0), (1, A1), (2, A2), (3, A3), (4, A4), (5, A5));
impl_papyrus_event_collection_tuple!(7; (0, A0), (1, A1), (2, A2), (3, A3), (4, A4), (5, A5), (6, A6));
impl_papyrus_event_collection_tuple!(8; (0, A0), (1, A1), (2, A2), (3, A3), (4, A4), (5, A5), (6, A6), (7, A7));

/// Save all persistent Papyrus event registries.
///
/// Returns `false` if any registry is missing record metadata or fails to save.
#[inline(always)]
pub fn save_registries(
    serialization: &SerializationInterface,
    registries: &[&dyn PapyrusPersistentEventRegistry],
) -> bool {
    registries
        .iter()
        .copied()
        .all(|registry| registry.save_record(serialization))
}

/// Load a single Papyrus event record into the first registry that owns `ty`.
#[inline(always)]
pub fn load_record(
    ty: u32,
    serialization: &SerializationInterface,
    registries: &mut [&mut dyn PapyrusPersistentEventRegistry],
) -> PapyrusEventLoadStatus {
    for registry in registries {
        let status = registry.load_record(ty, serialization);
        if status != PapyrusEventLoadStatus::Ignored {
            return status;
        }
    }

    PapyrusEventLoadStatus::Ignored
}

/// Revert all Papyrus event registries.
#[inline(always)]
pub fn revert_registries(
    serialization: Option<&SerializationInterface>,
    registries: &mut [&mut dyn PapyrusPersistentEventRegistry],
) {
    for registry in registries {
        registry.revert(serialization);
    }
}

/// Route a form-delete handle through all Papyrus event registries.
#[inline(always)]
pub fn form_delete_registries(
    handle: VMHandle,
    registries: &mut [&mut dyn PapyrusPersistentEventRegistry],
) -> usize {
    let mut removed = 0;
    for registry in registries {
        if registry.form_delete(handle) {
            removed += 1;
        }
    }
    removed
}

/// SDK-facing wrapper over `skse::RegistrationSet`.
pub struct PapyrusEventRegistry<Args = ()> {
    inner: RegistrationSet<Args>,
    record: Option<PapyrusEventRecord>,
}

impl<Args> PapyrusEventRegistry<Args> {
    #[inline(always)]
    pub fn new(event_name: impl Into<String>) -> Self {
        Self {
            inner: RegistrationSet::new(event_name),
            record: None,
        }
    }

    #[inline(always)]
    pub fn persistent(event_name: impl Into<String>, record: PapyrusEventRecord) -> Self {
        Self::new(event_name).with_record(record)
    }

    #[inline(always)]
    pub fn with_record(mut self, record: PapyrusEventRecord) -> Self {
        self.record = Some(record);
        self
    }

    #[inline(always)]
    pub fn inner(&self) -> &RegistrationSet<Args> {
        &self.inner
    }

    #[inline(always)]
    pub fn inner_mut(&mut self) -> &mut RegistrationSet<Args> {
        &mut self.inner
    }

    #[inline(always)]
    pub fn event_name(&self) -> &str {
        self.inner.event_name()
    }

    #[inline(always)]
    pub const fn record(&self) -> Option<PapyrusEventRecord> {
        self.record
    }

    #[inline(always)]
    pub fn set_record(&mut self, record: PapyrusEventRecord) {
        self.record = Some(record);
    }

    #[inline(always)]
    pub fn clear_record(&mut self) {
        self.record = None;
    }

    #[inline(always)]
    pub fn register_form(&mut self, form: impl Into<GamePtr<TESForm>>) -> bool {
        self.inner.register_form(form.into())
    }

    #[inline(always)]
    pub fn register_alias(&mut self, alias: impl Into<GamePtr<BGSBaseAlias>>) -> bool {
        self.inner.register_alias(alias.into())
    }

    #[inline(always)]
    pub fn register_ref_alias(&mut self, alias: impl Into<GamePtr<BGSRefAlias>>) -> bool {
        self.inner.register_alias(alias.into().base())
    }

    #[inline(always)]
    pub fn register_active_effect(&mut self, effect: impl Into<GamePtr<ActiveEffect>>) -> bool {
        self.inner.register_active_effect(effect.into())
    }

    #[inline(always)]
    pub fn unregister_form(&mut self, form: impl Into<GamePtr<TESForm>>) -> bool {
        self.inner.unregister_form(form.into())
    }

    #[inline(always)]
    pub fn unregister_alias(&mut self, alias: impl Into<GamePtr<BGSBaseAlias>>) -> bool {
        self.inner.unregister_alias(alias.into())
    }

    #[inline(always)]
    pub fn unregister_ref_alias(&mut self, alias: impl Into<GamePtr<BGSRefAlias>>) -> bool {
        self.inner.unregister_alias(alias.into().base())
    }

    #[inline(always)]
    pub fn unregister_active_effect(&mut self, effect: impl Into<GamePtr<ActiveEffect>>) -> bool {
        self.inner.unregister_active_effect(effect.into())
    }

    #[inline(always)]
    pub fn unregister_handle(&mut self, handle: VMHandle) -> bool {
        self.inner.unregister_handle(handle)
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    #[inline(always)]
    pub fn for_each_handle(&self, f: impl FnMut(VMHandle)) {
        self.inner.for_each_handle(f);
    }

    #[inline(always)]
    pub fn save_record(&self, serialization: &SerializationInterface) -> bool {
        match self.record {
            Some(record) => self
                .inner
                .save_record(serialization, record.ty, record.version),
            None => false,
        }
    }

    #[inline(always)]
    pub fn load_record(
        &mut self,
        ty: u32,
        serialization: &SerializationInterface,
    ) -> PapyrusEventLoadStatus {
        let Some(record) = self.record else {
            return PapyrusEventLoadStatus::Ignored;
        };
        if record.ty != ty {
            return PapyrusEventLoadStatus::Ignored;
        }

        if self.inner.load(serialization) {
            PapyrusEventLoadStatus::Loaded
        } else {
            PapyrusEventLoadStatus::Failed
        }
    }

    #[inline(always)]
    pub fn revert(&mut self, serialization: Option<&SerializationInterface>) {
        self.inner.revert(serialization);
    }

    #[inline(always)]
    pub fn form_delete(&mut self, handle: VMHandle) -> bool {
        self.inner.unregister_handle(handle)
    }
}

impl<Args> PapyrusEventRegistry<Args>
where
    Args: RegistrationEventArgs,
{
    #[inline(always)]
    pub fn send(&self, args: Args) {
        self.inner.send_event(args);
    }
}

impl<Args> PapyrusEventRegistry<Args>
where
    Args: RegistrationEventArgs + Send + 'static,
{
    #[inline(always)]
    pub fn queue_send(&'static self, args: Args) {
        self.inner.queue_event(args);
    }
}

impl<Args> PapyrusPersistentEventRegistry for PapyrusEventRegistry<Args> {
    #[inline(always)]
    fn event_name(&self) -> &str {
        PapyrusEventRegistry::event_name(self)
    }

    #[inline(always)]
    fn record(&self) -> Option<PapyrusEventRecord> {
        PapyrusEventRegistry::record(self)
    }

    #[inline(always)]
    fn save_record(&self, serialization: &SerializationInterface) -> bool {
        PapyrusEventRegistry::save_record(self, serialization)
    }

    #[inline(always)]
    fn load_record(
        &mut self,
        ty: u32,
        serialization: &SerializationInterface,
    ) -> PapyrusEventLoadStatus {
        PapyrusEventRegistry::load_record(self, ty, serialization)
    }

    #[inline(always)]
    fn revert(&mut self, serialization: Option<&SerializationInterface>) {
        PapyrusEventRegistry::revert(self, serialization);
    }

    #[inline(always)]
    fn form_delete(&mut self, handle: VMHandle) -> bool {
        PapyrusEventRegistry::form_delete(self, handle)
    }
}

/// SDK-facing wrapper over `skse::RegistrationSetUnique`.
pub struct PapyrusTargetedEventRegistry<Args = ()> {
    inner: RegistrationSetUnique<Args>,
    record: Option<PapyrusEventRecord>,
}

impl<Args> PapyrusTargetedEventRegistry<Args> {
    #[inline(always)]
    pub fn new(event_name: impl Into<String>) -> Self {
        Self {
            inner: RegistrationSetUnique::new(event_name),
            record: None,
        }
    }

    #[inline(always)]
    pub fn persistent(event_name: impl Into<String>, record: PapyrusEventRecord) -> Self {
        Self::new(event_name).with_record(record)
    }

    #[inline(always)]
    pub fn with_record(mut self, record: PapyrusEventRecord) -> Self {
        self.record = Some(record);
        self
    }

    #[inline(always)]
    pub fn inner(&self) -> &RegistrationSetUnique<Args> {
        &self.inner
    }

    #[inline(always)]
    pub fn inner_mut(&mut self) -> &mut RegistrationSetUnique<Args> {
        &mut self.inner
    }

    #[inline(always)]
    pub fn event_name(&self) -> &str {
        self.inner.event_name()
    }

    #[inline(always)]
    pub const fn record(&self) -> Option<PapyrusEventRecord> {
        self.record
    }

    #[inline(always)]
    pub fn set_record(&mut self, record: PapyrusEventRecord) {
        self.record = Some(record);
    }

    #[inline(always)]
    pub fn clear_record(&mut self) {
        self.record = None;
    }

    #[inline(always)]
    pub fn register_form(&mut self, form: impl Into<GamePtr<TESForm>>) -> bool {
        self.inner.register_form(form.into())
    }

    #[inline(always)]
    pub fn register_alias(&mut self, alias: impl Into<GamePtr<BGSRefAlias>>) -> bool {
        self.inner.register_alias(alias.into())
    }

    #[inline(always)]
    pub fn register_ref_alias(&mut self, alias: impl Into<GamePtr<BGSRefAlias>>) -> bool {
        self.register_alias(alias)
    }

    #[inline(always)]
    pub fn register_active_effect(&mut self, effect: impl Into<GamePtr<ActiveEffect>>) -> bool {
        self.inner.register_active_effect(effect.into())
    }

    #[inline(always)]
    pub fn unregister_form(&mut self, form: impl Into<GamePtr<TESForm>>) -> bool {
        self.inner.unregister_form(form.into())
    }

    #[inline(always)]
    pub fn unregister_alias(&mut self, alias: impl Into<GamePtr<BGSRefAlias>>) -> bool {
        self.inner.unregister_alias(alias.into())
    }

    #[inline(always)]
    pub fn unregister_ref_alias(&mut self, alias: impl Into<GamePtr<BGSRefAlias>>) -> bool {
        self.unregister_alias(alias)
    }

    #[inline(always)]
    pub fn unregister_active_effect(&mut self, effect: impl Into<GamePtr<ActiveEffect>>) -> bool {
        self.inner.unregister_active_effect(effect.into())
    }

    #[inline(always)]
    pub fn unregister_handle(&mut self, handle: VMHandle) -> bool {
        self.inner.unregister_handle(handle)
    }

    #[inline(always)]
    pub fn unregister_target(&mut self, target_id: u32) -> bool {
        self.inner.unregister_unique_id(target_id)
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    #[inline(always)]
    pub fn for_each_target_handle(&self, target_id: u32, f: impl FnMut(VMHandle)) {
        self.inner.for_each_handle(target_id, f);
    }

    #[inline(always)]
    pub fn save_record(&self, serialization: &SerializationInterface) -> bool {
        match self.record {
            Some(record) => self
                .inner
                .save_record(serialization, record.ty, record.version),
            None => false,
        }
    }

    #[inline(always)]
    pub fn load_record(
        &mut self,
        ty: u32,
        serialization: &SerializationInterface,
    ) -> PapyrusEventLoadStatus {
        let Some(record) = self.record else {
            return PapyrusEventLoadStatus::Ignored;
        };
        if record.ty != ty {
            return PapyrusEventLoadStatus::Ignored;
        }

        if self.inner.load(serialization) {
            PapyrusEventLoadStatus::Loaded
        } else {
            PapyrusEventLoadStatus::Failed
        }
    }

    #[inline(always)]
    pub fn revert(&mut self, serialization: Option<&SerializationInterface>) {
        self.inner.revert(serialization);
    }

    #[inline(always)]
    pub fn form_delete(&mut self, handle: VMHandle) -> bool {
        self.inner.unregister_handle(handle)
    }
}

impl<Args> PapyrusTargetedEventRegistry<Args>
where
    Args: RegistrationEventArgs,
{
    #[inline(always)]
    pub fn send_to(&self, target: impl Into<GamePtr<TESObjectREFR>>, args: Args) {
        self.inner.send_event(target.into(), args);
    }
}

impl<Args> PapyrusTargetedEventRegistry<Args>
where
    Args: RegistrationEventArgs + Send + 'static,
{
    #[inline(always)]
    pub fn queue_send_to(&'static self, target: impl Into<GamePtr<TESObjectREFR>>, args: Args) {
        self.inner.queue_event(target.into(), args);
    }
}

impl<Args> PapyrusPersistentEventRegistry for PapyrusTargetedEventRegistry<Args> {
    #[inline(always)]
    fn event_name(&self) -> &str {
        PapyrusTargetedEventRegistry::event_name(self)
    }

    #[inline(always)]
    fn record(&self) -> Option<PapyrusEventRecord> {
        PapyrusTargetedEventRegistry::record(self)
    }

    #[inline(always)]
    fn save_record(&self, serialization: &SerializationInterface) -> bool {
        PapyrusTargetedEventRegistry::save_record(self, serialization)
    }

    #[inline(always)]
    fn load_record(
        &mut self,
        ty: u32,
        serialization: &SerializationInterface,
    ) -> PapyrusEventLoadStatus {
        PapyrusTargetedEventRegistry::load_record(self, ty, serialization)
    }

    #[inline(always)]
    fn revert(&mut self, serialization: Option<&SerializationInterface>) {
        PapyrusTargetedEventRegistry::revert(self, serialization);
    }

    #[inline(always)]
    fn form_delete(&mut self, handle: VMHandle) -> bool {
        PapyrusTargetedEventRegistry::form_delete(self, handle)
    }
}

/// Compatibility alias for callers who think in terms of
/// `RegistrationSetUnique`.
pub type PapyrusUniqueEventRegistry<Args = ()> = PapyrusTargetedEventRegistry<Args>;

#[cfg(test)]
mod tests {
    extern crate std;

    use alloc::vec::Vec;
    use core::ffi::c_void;
    use core::mem::size_of;
    use std::cell::RefCell;

    use super::{
        PapyrusEventLoadStatus, PapyrusEventRecord, PapyrusEventRegistry,
        PapyrusPersistentEventRegistry, PapyrusTargetedEventRegistry, load_record,
        revert_registries, save_registries,
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
        let mut first = PapyrusEventRegistry::<()>::persistent(
            "OnFirst",
            PapyrusEventRecord::new(0x1111_1111, 1),
        );
        let mut second = PapyrusEventRegistry::<()>::persistent(
            "OnSecond",
            PapyrusEventRecord::new(0x2222_2222, 1),
        );

        let mut read_bytes = Vec::new();
        push_usize(&mut read_bytes, 2);
        push_vm_handle(&mut read_bytes, 10);
        push_vm_handle(&mut read_bytes, 20);
        reset_serialization_state(read_bytes);

        let mut registries: [&mut dyn PapyrusPersistentEventRegistry; 2] =
            [&mut first, &mut second];
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
}
