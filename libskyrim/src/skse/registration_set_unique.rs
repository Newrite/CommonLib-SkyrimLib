use alloc::collections::{BTreeMap, BTreeSet};
use alloc::string::String;
use core::ffi::c_void;
use core::marker::PhantomData;

use crate::re::{
    ActiveEffect, BGSRefAlias, BSFixedString, IObjectHandlePolicy, SkyrimVM, TESForm, VMHandle,
    VMTypeID,
};
use crate::sdk::core::GameRef;

use super::SerializationInterface;

fn get_handle_policy() -> *mut IObjectHandlePolicy {
    let skyrim_vm = SkyrimVM::get_singleton();
    if skyrim_vm.is_null() {
        return core::ptr::null_mut();
    }

    let vm = unsafe { (*skyrim_vm).get_virtual_machine() };
    if vm.is_null() {
        return core::ptr::null_mut();
    }

    unsafe { (*vm).get_object_handle_policy() }
}

fn persist_unique_regs(regs: &BTreeMap<u32, BTreeSet<VMHandle>>) {
    let policy = get_handle_policy();
    if policy.is_null() {
        return;
    }

    for handles in regs.values() {
        for &handle in handles {
            unsafe {
                (*policy).persist_handle(handle);
            }
        }
    }
}

fn release_unique_regs(regs: &BTreeMap<u32, BTreeSet<VMHandle>>) {
    let policy = get_handle_policy();
    if policy.is_null() {
        return;
    }

    for handles in regs.values() {
        for &handle in handles {
            unsafe {
                (*policy).release_handle(handle);
            }
        }
    }
}

pub struct RegistrationSetUniqueBase {
    regs: BTreeMap<u32, BTreeSet<VMHandle>>,
    event_name: String,
}

impl Clone for RegistrationSetUniqueBase {
    fn clone(&self) -> Self {
        let cloned = Self {
            regs: self.regs.clone(),
            event_name: self.event_name.clone(),
        };
        persist_unique_regs(&cloned.regs);
        cloned
    }
}

impl Drop for RegistrationSetUniqueBase {
    fn drop(&mut self) {
        release_unique_regs(&self.regs);
    }
}

impl RegistrationSetUniqueBase {
    #[inline(always)]
    pub fn new(event_name: impl Into<String>) -> Self {
        Self {
            regs: BTreeMap::new(),
            event_name: event_name.into(),
        }
    }

    #[inline(always)]
    pub fn regs(&self) -> &BTreeMap<u32, BTreeSet<VMHandle>> {
        &self.regs
    }

    #[inline(always)]
    pub fn event_name(&self) -> &str {
        &self.event_name
    }

    #[inline(always)]
    pub fn event_name_fixed(&self) -> BSFixedString {
        BSFixedString::from_str(&self.event_name)
    }

    #[inline(always)]
    pub fn register_form<'a>(&mut self, form: impl Into<GameRef<'a, TESForm>>) -> bool {
        let form = form.into();
        if form.is_null() {
            return false;
        }

        let form = form.as_ptr();
        let reference = unsafe { (*form).as_reference() };
        if reference.is_null() {
            return false;
        }

        let form_id = unsafe { (*reference).get_form_id() };
        if form_id == 0 {
            return false;
        }

        let type_id = unsafe { (*form).get_form_type() as VMTypeID };
        self.register_object(form.cast(), form_id, type_id)
    }

    #[inline(always)]
    pub fn register_alias<'a>(&mut self, alias: impl Into<GameRef<'a, BGSRefAlias>>) -> bool {
        let alias = alias.into();
        if alias.is_null() {
            return false;
        }

        let alias = alias.as_ptr();
        let target = unsafe { (*alias).get_actor_reference() };
        if target.is_null() {
            return false;
        }

        let form_id = unsafe { (*target).get_form_id() };
        if form_id == 0 {
            return false;
        }

        self.register_object(alias.cast(), form_id, BGSRefAlias::VM_TYPE_ID)
    }

    #[inline(always)]
    pub fn register_active_effect<'a>(
        &mut self,
        effect: impl Into<GameRef<'a, ActiveEffect>>,
    ) -> bool {
        let effect = effect.into();
        if effect.is_null() {
            return false;
        }

        let effect = effect.as_ptr();
        let target = unsafe { (*effect).get_target_actor() };
        if target.is_null() {
            return false;
        }

        let form_id = unsafe { (*target).get_form_id() };
        if form_id == 0 {
            return false;
        }

        self.register_object(effect.cast(), form_id, ActiveEffect::VM_TYPE_ID)
    }

    #[inline(always)]
    pub fn unregister_form<'a>(&mut self, form: impl Into<GameRef<'a, TESForm>>) -> bool {
        let form = form.into();
        if form.is_null() {
            return false;
        }

        let form = form.as_ptr();
        let reference = unsafe { (*form).as_reference() };
        if reference.is_null() {
            return false;
        }

        let form_id = unsafe { (*reference).get_form_id() };
        if form_id == 0 {
            return false;
        }

        let type_id = unsafe { (*form).get_form_type() as VMTypeID };
        self.unregister_object(form.cast(), form_id, type_id)
    }

    #[inline(always)]
    pub fn unregister_alias<'a>(&mut self, alias: impl Into<GameRef<'a, BGSRefAlias>>) -> bool {
        let alias = alias.into();
        if alias.is_null() {
            return false;
        }

        let alias = alias.as_ptr();
        let target = unsafe { (*alias).get_actor_reference() };
        if target.is_null() {
            return false;
        }

        let form_id = unsafe { (*target).get_form_id() };
        if form_id == 0 {
            return false;
        }

        self.unregister_object(alias.cast(), form_id, BGSRefAlias::VM_TYPE_ID)
    }

    #[inline(always)]
    pub fn unregister_active_effect<'a>(
        &mut self,
        effect: impl Into<GameRef<'a, ActiveEffect>>,
    ) -> bool {
        let effect = effect.into();
        if effect.is_null() {
            return false;
        }

        let effect = effect.as_ptr();
        let target = unsafe { (*effect).get_target_actor() };
        if target.is_null() {
            return false;
        }

        let form_id = unsafe { (*target).get_form_id() };
        if form_id == 0 {
            return false;
        }

        self.unregister_object(effect.cast(), form_id, ActiveEffect::VM_TYPE_ID)
    }

    pub fn unregister_handle(&mut self, handle: VMHandle) -> bool {
        let policy = get_handle_policy();
        if policy.is_null() {
            return false;
        }

        for handles in self.regs.values_mut() {
            if handles.remove(&handle) {
                unsafe {
                    (*policy).release_handle(handle);
                }
                return true;
            }
        }

        false
    }

    pub fn unregister_unique_id(&mut self, unique_id: u32) -> bool {
        let policy = get_handle_policy();
        if policy.is_null() {
            return false;
        }

        if let Some(handles) = self.regs.remove(&unique_id) {
            for handle in handles {
                unsafe {
                    (*policy).release_handle(handle);
                }
            }
            true
        } else {
            false
        }
    }

    pub fn clear(&mut self) {
        release_unique_regs(&self.regs);
        self.regs.clear();
    }

    pub fn save_record(
        &self,
        serialization: &SerializationInterface,
        ty: u32,
        version: u32,
    ) -> bool {
        if !serialization.open_record(ty, version) {
            return false;
        }

        self.save(serialization)
    }

    pub fn save(&self, serialization: &SerializationInterface) -> bool {
        let num_unique_handles = self.regs.len();
        if !serialization.write_record_data(
            (&num_unique_handles as *const usize).cast(),
            core::mem::size_of::<usize>() as u32,
        ) {
            return false;
        }

        for (form_id, handles) in &self.regs {
            if !serialization.write_record_data(
                (form_id as *const u32).cast(),
                core::mem::size_of::<u32>() as u32,
            ) {
                return false;
            }

            let num_handles = handles.len();
            if !serialization.write_record_data(
                (&num_handles as *const usize).cast(),
                core::mem::size_of::<usize>() as u32,
            ) {
                return false;
            }

            for handle in handles {
                if !serialization.write_record_data(
                    (handle as *const VMHandle).cast(),
                    core::mem::size_of::<VMHandle>() as u32,
                ) {
                    return false;
                }
            }
        }

        true
    }

    pub fn load(&mut self, serialization: &SerializationInterface) -> bool {
        let mut num_unique_handles = 0_usize;
        serialization.read_record_data(
            (&mut num_unique_handles as *mut usize).cast(),
            core::mem::size_of::<usize>() as u32,
        );

        self.regs.clear();

        for _ in 0..num_unique_handles {
            let mut form_id = 0_u32;
            serialization.read_record_data(
                (&mut form_id as *mut u32).cast(),
                core::mem::size_of::<u32>() as u32,
            );

            let mut resolved_form_id = form_id;
            let resolved = serialization.resolve_form_id(form_id, &mut resolved_form_id);

            let mut num_handles = 0_usize;
            serialization.read_record_data(
                (&mut num_handles as *mut usize).cast(),
                core::mem::size_of::<usize>() as u32,
            );

            for _ in 0..num_handles {
                let mut handle = 0 as VMHandle;
                serialization.read_record_data(
                    (&mut handle as *mut VMHandle).cast(),
                    core::mem::size_of::<VMHandle>() as u32,
                );

                let mut resolved_handle = handle;
                if resolved && serialization.resolve_handle(handle, &mut resolved_handle) {
                    self.regs
                        .entry(resolved_form_id)
                        .or_default()
                        .insert(resolved_handle);
                }
            }
        }

        true
    }

    #[inline(always)]
    pub fn revert(&mut self, _serialization: Option<&SerializationInterface>) {
        self.clear();
    }

    fn register_object(&mut self, object: *const c_void, form_id: u32, type_id: VMTypeID) -> bool {
        let policy = get_handle_policy();
        if policy.is_null() {
            return false;
        }

        let invalid = unsafe { (*policy).empty_handle() };
        let handle = unsafe { (*policy).get_handle_for_object(type_id, object) };
        if handle == invalid {
            return false;
        }

        let inserted = self.regs.entry(form_id).or_default().insert(handle);
        if inserted {
            unsafe {
                (*policy).persist_handle(handle);
            }
        }

        inserted
    }

    fn unregister_object(
        &mut self,
        object: *const c_void,
        form_id: u32,
        type_id: VMTypeID,
    ) -> bool {
        let policy = get_handle_policy();
        if policy.is_null() {
            return false;
        }

        let invalid = unsafe { (*policy).empty_handle() };
        let handle = unsafe { (*policy).get_handle_for_object(type_id, object) };
        if handle == invalid {
            return false;
        }

        if let Some(handles) = self.regs.get_mut(&form_id) {
            if handles.remove(&handle) {
                unsafe {
                    (*policy).release_handle(handle);
                }
                return true;
            }
        }

        false
    }
}

/// Rust-side port of `SKSE::RegistrationSetUnique<Args...>`.
///
/// TODO: Add typed `send_event(...)` / `queue_event(...)` parity once libskyrim
/// exposes source-backed `MakeFunctionArguments` / `VMArg` construction for
/// Papyrus-convertible argument packs. CommonLib builds a fresh argument object
/// per handle, so a shared raw-pointer shortcut here would be dishonest.
pub struct RegistrationSetUnique<Args = ()> {
    base: RegistrationSetUniqueBase,
    _marker: PhantomData<fn() -> Args>,
}

impl<Args> Clone for RegistrationSetUnique<Args> {
    fn clone(&self) -> Self {
        Self {
            base: self.base.clone(),
            _marker: PhantomData,
        }
    }
}

impl<Args> RegistrationSetUnique<Args> {
    #[inline(always)]
    pub fn new(event_name: impl Into<String>) -> Self {
        Self {
            base: RegistrationSetUniqueBase::new(event_name),
            _marker: PhantomData,
        }
    }

    #[inline(always)]
    pub fn base(&self) -> &RegistrationSetUniqueBase {
        &self.base
    }

    #[inline(always)]
    pub fn base_mut(&mut self) -> &mut RegistrationSetUniqueBase {
        &mut self.base
    }

    #[inline(always)]
    pub fn event_name(&self) -> &str {
        self.base.event_name()
    }

    #[inline(always)]
    pub fn event_name_fixed(&self) -> BSFixedString {
        self.base.event_name_fixed()
    }

    #[inline(always)]
    pub fn register_form<'a>(&mut self, form: impl Into<GameRef<'a, TESForm>>) -> bool {
        self.base.register_form(form)
    }

    #[inline(always)]
    pub fn register_alias<'a>(&mut self, alias: impl Into<GameRef<'a, BGSRefAlias>>) -> bool {
        self.base.register_alias(alias)
    }

    #[inline(always)]
    pub fn register_active_effect<'a>(
        &mut self,
        effect: impl Into<GameRef<'a, ActiveEffect>>,
    ) -> bool {
        self.base.register_active_effect(effect)
    }

    #[inline(always)]
    pub fn unregister_form<'a>(&mut self, form: impl Into<GameRef<'a, TESForm>>) -> bool {
        self.base.unregister_form(form)
    }

    #[inline(always)]
    pub fn unregister_alias<'a>(&mut self, alias: impl Into<GameRef<'a, BGSRefAlias>>) -> bool {
        self.base.unregister_alias(alias)
    }

    #[inline(always)]
    pub fn unregister_active_effect<'a>(
        &mut self,
        effect: impl Into<GameRef<'a, ActiveEffect>>,
    ) -> bool {
        self.base.unregister_active_effect(effect)
    }

    #[inline(always)]
    pub fn unregister_handle(&mut self, handle: VMHandle) -> bool {
        self.base.unregister_handle(handle)
    }

    #[inline(always)]
    pub fn unregister_unique_id(&mut self, unique_id: u32) -> bool {
        self.base.unregister_unique_id(unique_id)
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.base.clear();
    }

    #[inline(always)]
    pub fn save_record(
        &self,
        serialization: &SerializationInterface,
        ty: u32,
        version: u32,
    ) -> bool {
        self.base.save_record(serialization, ty, version)
    }

    #[inline(always)]
    pub fn save(&self, serialization: &SerializationInterface) -> bool {
        self.base.save(serialization)
    }

    #[inline(always)]
    pub fn load(&mut self, serialization: &SerializationInterface) -> bool {
        self.base.load(serialization)
    }

    #[inline(always)]
    pub fn revert(&mut self, serialization: Option<&SerializationInterface>) {
        self.base.revert(serialization);
    }

    #[inline(always)]
    pub fn for_each_handle(&self, unique_id: u32, mut f: impl FnMut(VMHandle)) {
        if let Some(handles) = self.base.regs().get(&unique_id) {
            for &handle in handles {
                f(handle);
            }
        }
    }
}
