use alloc::collections::BTreeSet;
use alloc::string::String;
use core::ffi::c_void;
use core::marker::PhantomData;

use crate::re::{
    ActiveEffect, BGSBaseAlias, BSFixedString, IObjectHandlePolicy, SkyrimVM, TESForm, VMHandle,
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

fn release_handle_set(handles: &BTreeSet<VMHandle>) {
    let policy = get_handle_policy();
    if policy.is_null() {
        return;
    }

    for &handle in handles {
        unsafe {
            (*policy).release_handle(handle);
        }
    }
}

/// Source-backed Rust port of `SKSE::Impl::RegistrationSetBase`.
///
/// Rust uses explicit `&mut self` mutation instead of the C++ internal mutex.
pub struct RegistrationSetBase {
    handles: BTreeSet<VMHandle>,
    event_name: String,
}

impl Clone for RegistrationSetBase {
    fn clone(&self) -> Self {
        let cloned = Self {
            handles: self.handles.clone(),
            event_name: self.event_name.clone(),
        };

        let policy = get_handle_policy();
        if !policy.is_null() {
            for &handle in &cloned.handles {
                unsafe {
                    (*policy).persist_handle(handle);
                }
            }
        }

        cloned
    }
}

impl Drop for RegistrationSetBase {
    fn drop(&mut self) {
        release_handle_set(&self.handles);
    }
}

impl RegistrationSetBase {
    #[inline(always)]
    pub fn new(event_name: impl Into<String>) -> Self {
        Self {
            handles: BTreeSet::new(),
            event_name: event_name.into(),
        }
    }

    #[inline(always)]
    pub fn handles(&self) -> &BTreeSet<VMHandle> {
        &self.handles
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

        let form = form.as_ptr().cast_const();
        let type_id = unsafe { (*form).get_form_type() as VMTypeID };
        self.register_object(form.cast(), type_id)
    }

    #[inline(always)]
    pub fn register_alias<'a>(&mut self, alias: impl Into<GameRef<'a, BGSBaseAlias>>) -> bool {
        let alias = alias.into();
        if alias.is_null() {
            return false;
        }

        let alias = alias.as_ptr().cast_const();
        self.register_object(alias.cast(), BGSBaseAlias::VM_TYPE_ID)
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

        let effect = effect.as_ptr().cast_const();
        self.register_object(effect.cast(), ActiveEffect::VM_TYPE_ID)
    }

    #[inline(always)]
    pub fn unregister_form<'a>(&mut self, form: impl Into<GameRef<'a, TESForm>>) -> bool {
        let form = form.into();
        if form.is_null() {
            return false;
        }

        let form = form.as_ptr().cast_const();
        let type_id = unsafe { (*form).get_form_type() as VMTypeID };
        self.unregister_object(form.cast(), type_id)
    }

    #[inline(always)]
    pub fn unregister_alias<'a>(&mut self, alias: impl Into<GameRef<'a, BGSBaseAlias>>) -> bool {
        let alias = alias.into();
        if alias.is_null() {
            return false;
        }

        let alias = alias.as_ptr().cast_const();
        self.unregister_object(alias.cast(), BGSBaseAlias::VM_TYPE_ID)
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

        let effect = effect.as_ptr().cast_const();
        self.unregister_object(effect.cast(), ActiveEffect::VM_TYPE_ID)
    }

    pub fn unregister_handle(&mut self, handle: VMHandle) -> bool {
        let policy = get_handle_policy();
        if policy.is_null() {
            return false;
        }

        if self.handles.remove(&handle) {
            unsafe {
                (*policy).release_handle(handle);
            }
            true
        } else {
            false
        }
    }

    pub fn clear(&mut self) {
        release_handle_set(&self.handles);
        self.handles.clear();
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
        let num_regs = self.handles.len();
        if !serialization.write_record_data(
            (&num_regs as *const usize).cast(),
            core::mem::size_of::<usize>() as u32,
        ) {
            return false;
        }

        for handle in &self.handles {
            if !serialization.write_record_data(
                (handle as *const VMHandle).cast(),
                core::mem::size_of::<VMHandle>() as u32,
            ) {
                return false;
            }
        }

        true
    }

    pub fn load(&mut self, serialization: &SerializationInterface) -> bool {
        let mut num_regs = 0_usize;
        serialization.read_record_data(
            (&mut num_regs as *mut usize).cast(),
            core::mem::size_of::<usize>() as u32,
        );

        self.handles.clear();

        for _ in 0..num_regs {
            let mut handle = 0 as VMHandle;
            serialization.read_record_data(
                (&mut handle as *mut VMHandle).cast(),
                core::mem::size_of::<VMHandle>() as u32,
            );

            let mut resolved = handle;
            if serialization.resolve_handle(handle, &mut resolved) {
                self.handles.insert(resolved);
            }
        }

        true
    }

    #[inline(always)]
    pub fn revert(&mut self, _serialization: Option<&SerializationInterface>) {
        self.clear();
    }

    fn register_object(&mut self, object: *const c_void, type_id: VMTypeID) -> bool {
        let policy = get_handle_policy();
        if policy.is_null() {
            return false;
        }

        let invalid = unsafe { (*policy).empty_handle() };
        let handle = unsafe { (*policy).get_handle_for_object(type_id, object) };
        if handle == invalid {
            return false;
        }

        if self.handles.insert(handle) {
            unsafe {
                (*policy).persist_handle(handle);
            }
            true
        } else {
            false
        }
    }

    fn unregister_object(&mut self, object: *const c_void, type_id: VMTypeID) -> bool {
        let policy = get_handle_policy();
        if policy.is_null() {
            return false;
        }

        let invalid = unsafe { (*policy).empty_handle() };
        let handle = unsafe { (*policy).get_handle_for_object(type_id, object) };
        if handle == invalid {
            return false;
        }

        self.unregister_handle(handle)
    }
}

/// Rust-side port of `SKSE::RegistrationSet<Args...>`.
///
/// TODO: Add typed `send_event(...)` / `queue_event(...)` parity once libskyrim
/// exposes source-backed `MakeFunctionArguments` / `VMArg` construction for
/// arbitrary Papyrus-convertible argument packs. CommonLib creates a fresh
/// argument object per handle, so a shared raw-pointer helper would be dishonest.
pub struct RegistrationSet<Args = ()> {
    base: RegistrationSetBase,
    _marker: PhantomData<fn() -> Args>,
}

impl<Args> Clone for RegistrationSet<Args> {
    fn clone(&self) -> Self {
        Self {
            base: self.base.clone(),
            _marker: PhantomData,
        }
    }
}

impl<Args> RegistrationSet<Args> {
    #[inline(always)]
    pub fn new(event_name: impl Into<String>) -> Self {
        Self {
            base: RegistrationSetBase::new(event_name),
            _marker: PhantomData,
        }
    }

    #[inline(always)]
    pub fn base(&self) -> &RegistrationSetBase {
        &self.base
    }

    #[inline(always)]
    pub fn base_mut(&mut self) -> &mut RegistrationSetBase {
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
    pub fn register_alias<'a>(&mut self, alias: impl Into<GameRef<'a, BGSBaseAlias>>) -> bool {
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
    pub fn unregister_alias<'a>(&mut self, alias: impl Into<GameRef<'a, BGSBaseAlias>>) -> bool {
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
    pub fn for_each_handle(&self, mut f: impl FnMut(VMHandle)) {
        for &handle in self.base.handles() {
            f(handle);
        }
    }
}
