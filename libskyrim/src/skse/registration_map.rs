use alloc::collections::{BTreeMap, BTreeSet};
use alloc::string::String;
use alloc::vec;
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

fn release_handle_map<Filter>(regs: &BTreeMap<Filter, BTreeSet<VMHandle>>) {
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

pub trait RegistrationFilter: Clone + Ord {
    fn save_filter(&self, serialization: &SerializationInterface) -> bool;
    fn load_filter(serialization: &SerializationInterface) -> Option<Self>;
}

macro_rules! impl_registration_filter_pod {
    ($($ty:ty),* $(,)?) => {
        $(
            impl RegistrationFilter for $ty {
                #[inline(always)]
                fn save_filter(&self, serialization: &SerializationInterface) -> bool {
                    serialization.write_record_data(
                        (self as *const $ty).cast(),
                        core::mem::size_of::<$ty>() as u32,
                    )
                }

                #[inline(always)]
                fn load_filter(serialization: &SerializationInterface) -> Option<Self> {
                    let mut value = core::mem::MaybeUninit::<$ty>::uninit();
                    serialization.read_record_data(
                        value.as_mut_ptr().cast(),
                        core::mem::size_of::<$ty>() as u32,
                    );
                    Some(unsafe { value.assume_init() })
                }
            }
        )*
    };
}

impl_registration_filter_pod!(u8, u16, u32, u64, i8, i16, i32, i64, usize, isize, bool);

impl RegistrationFilter for String {
    fn save_filter(&self, serialization: &SerializationInterface) -> bool {
        let length = self.len() + 1;
        if !serialization.write_record_data(
            (&length as *const usize).cast(),
            core::mem::size_of::<usize>() as u32,
        ) {
            return false;
        }

        let mut bytes = vec![0_u8; length];
        bytes[..self.len()].copy_from_slice(self.as_bytes());
        serialization.write_record_data(bytes.as_ptr().cast(), bytes.len() as u32)
    }

    fn load_filter(serialization: &SerializationInterface) -> Option<Self> {
        let mut length = 0_usize;
        serialization.read_record_data(
            (&mut length as *mut usize).cast(),
            core::mem::size_of::<usize>() as u32,
        );

        let mut bytes = vec![0_u8; length];
        serialization.read_record_data(bytes.as_mut_ptr().cast(), bytes.len() as u32);

        if bytes.last().copied() == Some(0) {
            bytes.pop();
        }

        String::from_utf8(bytes).ok()
    }
}

/// Source-backed Rust port of `SKSE::Impl::EventFilter<Filter>::RegistrationMapBase`.
///
/// Rust uses explicit `&mut self` mutation instead of the C++ internal mutex.
pub struct RegistrationMapBase<Filter: RegistrationFilter> {
    regs: BTreeMap<Filter, BTreeSet<VMHandle>>,
    event_name: String,
}

impl<Filter: RegistrationFilter> Clone for RegistrationMapBase<Filter> {
    fn clone(&self) -> Self {
        let cloned = Self {
            regs: self.regs.clone(),
            event_name: self.event_name.clone(),
        };

        let policy = get_handle_policy();
        if !policy.is_null() {
            for handles in cloned.regs.values() {
                for &handle in handles {
                    unsafe {
                        (*policy).persist_handle(handle);
                    }
                }
            }
        }

        cloned
    }
}

impl<Filter: RegistrationFilter> Drop for RegistrationMapBase<Filter> {
    fn drop(&mut self) {
        release_handle_map(&self.regs);
    }
}

impl<Filter: RegistrationFilter> RegistrationMapBase<Filter> {
    #[inline(always)]
    pub fn new(event_name: impl Into<String>) -> Self {
        Self {
            regs: BTreeMap::new(),
            event_name: event_name.into(),
        }
    }

    #[inline(always)]
    pub fn regs(&self) -> &BTreeMap<Filter, BTreeSet<VMHandle>> {
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
    pub fn register_form<'a>(
        &mut self,
        form: impl Into<GameRef<'a, TESForm>>,
        filter: Filter,
    ) -> bool {
        let form = form.into();
        if form.is_null() {
            return false;
        }

        let form = form.as_ptr().cast_const();
        let type_id = unsafe { (*form).get_form_type() as VMTypeID };
        self.register_object(form.cast(), filter, type_id)
    }

    #[inline(always)]
    pub fn register_alias<'a>(
        &mut self,
        alias: impl Into<GameRef<'a, BGSBaseAlias>>,
        filter: Filter,
    ) -> bool {
        let alias = alias.into();
        if alias.is_null() {
            return false;
        }

        let alias = alias.as_ptr().cast_const();
        self.register_object(alias.cast(), filter, BGSBaseAlias::VM_TYPE_ID)
    }

    #[inline(always)]
    pub fn register_active_effect<'a>(
        &mut self,
        effect: impl Into<GameRef<'a, ActiveEffect>>,
        filter: Filter,
    ) -> bool {
        let effect = effect.into();
        if effect.is_null() {
            return false;
        }

        let effect = effect.as_ptr().cast_const();
        self.register_object(effect.cast(), filter, ActiveEffect::VM_TYPE_ID)
    }

    #[inline(always)]
    pub fn unregister_form<'a>(
        &mut self,
        form: impl Into<GameRef<'a, TESForm>>,
        filter: Filter,
    ) -> bool {
        let form = form.into();
        if form.is_null() {
            return false;
        }

        let form = form.as_ptr().cast_const();
        let type_id = unsafe { (*form).get_form_type() as VMTypeID };
        self.unregister_object(form.cast(), filter, type_id)
    }

    #[inline(always)]
    pub fn unregister_alias<'a>(
        &mut self,
        alias: impl Into<GameRef<'a, BGSBaseAlias>>,
        filter: Filter,
    ) -> bool {
        let alias = alias.into();
        if alias.is_null() {
            return false;
        }

        let alias = alias.as_ptr().cast_const();
        self.unregister_object(alias.cast(), filter, BGSBaseAlias::VM_TYPE_ID)
    }

    #[inline(always)]
    pub fn unregister_active_effect<'a>(
        &mut self,
        effect: impl Into<GameRef<'a, ActiveEffect>>,
        filter: Filter,
    ) -> bool {
        let effect = effect.into();
        if effect.is_null() {
            return false;
        }

        let effect = effect.as_ptr().cast_const();
        self.unregister_object(effect.cast(), filter, ActiveEffect::VM_TYPE_ID)
    }

    #[inline(always)]
    pub fn unregister_all_form<'a>(&mut self, form: impl Into<GameRef<'a, TESForm>>) {
        let form = form.into();
        if form.is_null() {
            return;
        }

        let form = form.as_ptr().cast_const();
        let type_id = unsafe { (*form).get_form_type() as VMTypeID };
        self.unregister_all_object(form.cast(), type_id);
    }

    #[inline(always)]
    pub fn unregister_all_alias<'a>(&mut self, alias: impl Into<GameRef<'a, BGSBaseAlias>>) {
        let alias = alias.into();
        if alias.is_null() {
            return;
        }

        let alias = alias.as_ptr().cast_const();
        self.unregister_all_object(alias.cast(), BGSBaseAlias::VM_TYPE_ID);
    }

    #[inline(always)]
    pub fn unregister_all_active_effect<'a>(
        &mut self,
        effect: impl Into<GameRef<'a, ActiveEffect>>,
    ) {
        let effect = effect.into();
        if effect.is_null() {
            return;
        }

        let effect = effect.as_ptr().cast_const();
        self.unregister_all_object(effect.cast(), ActiveEffect::VM_TYPE_ID);
    }

    pub fn unregister_all_handle(&mut self, handle: VMHandle) {
        let policy = get_handle_policy();
        if policy.is_null() {
            return;
        }

        for handles in self.regs.values_mut() {
            if handles.remove(&handle) {
                unsafe {
                    (*policy).release_handle(handle);
                }
            }
        }
    }

    pub fn clear(&mut self) {
        release_handle_map(&self.regs);
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
        let num_regs = self.regs.len();
        if !serialization.write_record_data(
            (&num_regs as *const usize).cast(),
            core::mem::size_of::<usize>() as u32,
        ) {
            return false;
        }

        for (filter, handles) in &self.regs {
            if !filter.save_filter(serialization) {
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
        let mut num_regs = 0_usize;
        serialization.read_record_data(
            (&mut num_regs as *mut usize).cast(),
            core::mem::size_of::<usize>() as u32,
        );

        self.regs.clear();

        for _ in 0..num_regs {
            let Some(filter) = Filter::load_filter(serialization) else {
                return false;
            };

            let mut num_handles = 0_usize;
            serialization.read_record_data(
                (&mut num_handles as *mut usize).cast(),
                core::mem::size_of::<usize>() as u32,
            );

            let handles = self.regs.entry(filter).or_default();
            for _ in 0..num_handles {
                let mut handle = 0 as VMHandle;
                serialization.read_record_data(
                    (&mut handle as *mut VMHandle).cast(),
                    core::mem::size_of::<VMHandle>() as u32,
                );

                let mut resolved = handle;
                if serialization.resolve_handle(handle, &mut resolved) {
                    handles.insert(resolved);
                }
            }
        }

        true
    }

    #[inline(always)]
    pub fn revert(&mut self, _serialization: Option<&SerializationInterface>) {
        self.clear();
    }

    fn register_object(
        &mut self,
        object: *const c_void,
        filter: Filter,
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

        let inserted = self.regs.entry(filter).or_default().insert(handle);
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
        filter: Filter,
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

        if let Some(handles) = self.regs.get_mut(&filter) {
            if handles.remove(&handle) {
                unsafe {
                    (*policy).release_handle(handle);
                }
                return true;
            }
        }

        false
    }

    fn unregister_all_object(&mut self, object: *const c_void, type_id: VMTypeID) {
        let policy = get_handle_policy();
        if policy.is_null() {
            return;
        }

        let invalid = unsafe { (*policy).empty_handle() };
        let handle = unsafe { (*policy).get_handle_for_object(type_id, object) };
        if handle == invalid {
            return;
        }

        for handles in self.regs.values_mut() {
            if handles.remove(&handle) {
                unsafe {
                    (*policy).release_handle(handle);
                }
            }
        }
    }
}

/// Rust-side port of `SKSE::RegistrationMap<Filter, Args...>`.
///
/// TODO: Add typed `send_event(...)` / `queue_event(...)` parity once libskyrim
/// exposes source-backed `MakeFunctionArguments` / `VMArg` construction for
/// arbitrary Papyrus-convertible argument packs. CommonLib creates a fresh
/// argument object per handle, so a shared raw-pointer helper would be dishonest.
pub struct RegistrationMap<Filter: RegistrationFilter, Args = ()> {
    base: RegistrationMapBase<Filter>,
    _marker: PhantomData<fn() -> Args>,
}

impl<Filter: RegistrationFilter, Args> Clone for RegistrationMap<Filter, Args> {
    fn clone(&self) -> Self {
        Self {
            base: self.base.clone(),
            _marker: PhantomData,
        }
    }
}

impl<Filter: RegistrationFilter, Args> RegistrationMap<Filter, Args> {
    #[inline(always)]
    pub fn new(event_name: impl Into<String>) -> Self {
        Self {
            base: RegistrationMapBase::new(event_name),
            _marker: PhantomData,
        }
    }

    #[inline(always)]
    pub fn base(&self) -> &RegistrationMapBase<Filter> {
        &self.base
    }

    #[inline(always)]
    pub fn base_mut(&mut self) -> &mut RegistrationMapBase<Filter> {
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
    pub fn register_form<'a>(
        &mut self,
        form: impl Into<GameRef<'a, TESForm>>,
        filter: Filter,
    ) -> bool {
        self.base.register_form(form, filter)
    }

    #[inline(always)]
    pub fn register_alias<'a>(
        &mut self,
        alias: impl Into<GameRef<'a, BGSBaseAlias>>,
        filter: Filter,
    ) -> bool {
        self.base.register_alias(alias, filter)
    }

    #[inline(always)]
    pub fn register_active_effect<'a>(
        &mut self,
        effect: impl Into<GameRef<'a, ActiveEffect>>,
        filter: Filter,
    ) -> bool {
        self.base.register_active_effect(effect, filter)
    }

    #[inline(always)]
    pub fn unregister_form<'a>(
        &mut self,
        form: impl Into<GameRef<'a, TESForm>>,
        filter: Filter,
    ) -> bool {
        self.base.unregister_form(form, filter)
    }

    #[inline(always)]
    pub fn unregister_alias<'a>(
        &mut self,
        alias: impl Into<GameRef<'a, BGSBaseAlias>>,
        filter: Filter,
    ) -> bool {
        self.base.unregister_alias(alias, filter)
    }

    #[inline(always)]
    pub fn unregister_active_effect<'a>(
        &mut self,
        effect: impl Into<GameRef<'a, ActiveEffect>>,
        filter: Filter,
    ) -> bool {
        self.base.unregister_active_effect(effect, filter)
    }

    #[inline(always)]
    pub fn unregister_all_form<'a>(&mut self, form: impl Into<GameRef<'a, TESForm>>) {
        self.base.unregister_all_form(form);
    }

    #[inline(always)]
    pub fn unregister_all_alias<'a>(&mut self, alias: impl Into<GameRef<'a, BGSBaseAlias>>) {
        self.base.unregister_all_alias(alias);
    }

    #[inline(always)]
    pub fn unregister_all_active_effect<'a>(
        &mut self,
        effect: impl Into<GameRef<'a, ActiveEffect>>,
    ) {
        self.base.unregister_all_active_effect(effect);
    }

    #[inline(always)]
    pub fn unregister_all_handle(&mut self, handle: VMHandle) {
        self.base.unregister_all_handle(handle);
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
    pub fn for_each_handle(&self, filter: &Filter, mut f: impl FnMut(VMHandle)) {
        if let Some(handles) = self.base.regs().get(filter) {
            for &handle in handles {
                f(handle);
            }
        }
    }
}
