use alloc::collections::{BTreeMap, BTreeSet};
use alloc::string::String;
use core::ffi::c_void;
use core::marker::PhantomData;

use crate::re::{
    ActiveEffect, BGSRefAlias, BSFixedString, IObjectHandlePolicy, SkyrimVM, TESForm,
    TESObjectREFR, VMHandle, VMTypeID,
};
use crate::sdk::core::GamePtr;

use super::registration_arguments::{RegistrationEventArgs, with_vm};
use super::{RegistrationFilter, SerializationInterface};

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

type UniqueEventFilter<Filter> = (Filter, bool);
type UniqueEventFilterHandleMap<Filter> = BTreeMap<UniqueEventFilter<Filter>, BTreeSet<VMHandle>>;

fn persist_unique_filter_regs<Filter: RegistrationFilter>(
    regs: &BTreeMap<u32, UniqueEventFilterHandleMap<Filter>>,
) {
    let policy = get_handle_policy();
    if policy.is_null() {
        return;
    }

    for handles_by_filter in regs.values() {
        for handles in handles_by_filter.values() {
            for &handle in handles {
                unsafe {
                    (*policy).persist_handle(handle);
                }
            }
        }
    }
}

fn release_unique_filter_regs<Filter: RegistrationFilter>(
    regs: &BTreeMap<u32, UniqueEventFilterHandleMap<Filter>>,
) {
    let policy = get_handle_policy();
    if policy.is_null() {
        return;
    }

    for handles_by_filter in regs.values() {
        for handles in handles_by_filter.values() {
            for &handle in handles {
                unsafe {
                    (*policy).release_handle(handle);
                }
            }
        }
    }
}

pub struct RegistrationMapUniqueBase<Filter: RegistrationFilter> {
    regs: BTreeMap<u32, UniqueEventFilterHandleMap<Filter>>,
    event_name: String,
}

impl<Filter: RegistrationFilter> Clone for RegistrationMapUniqueBase<Filter> {
    fn clone(&self) -> Self {
        let cloned = Self {
            regs: self.regs.clone(),
            event_name: self.event_name.clone(),
        };
        persist_unique_filter_regs(&cloned.regs);
        cloned
    }
}

impl<Filter: RegistrationFilter> Drop for RegistrationMapUniqueBase<Filter> {
    fn drop(&mut self) {
        release_unique_filter_regs(&self.regs);
    }
}

impl<Filter: RegistrationFilter> RegistrationMapUniqueBase<Filter> {
    #[inline(always)]
    pub fn new(event_name: impl Into<String>) -> Self {
        Self {
            regs: BTreeMap::new(),
            event_name: event_name.into(),
        }
    }

    #[inline(always)]
    pub fn regs(&self) -> &BTreeMap<u32, UniqueEventFilterHandleMap<Filter>> {
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
        form: impl Into<GamePtr<TESForm>>,
        filter: Filter,
        match_filter: bool,
    ) -> bool {
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
        self.register_object(form.cast(), form_id, (filter, match_filter), type_id)
    }

    #[inline(always)]
    pub fn register_active_effect<'a>(
        &mut self,
        effect: impl Into<GamePtr<ActiveEffect>>,
        filter: Filter,
        match_filter: bool,
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

        self.register_object(
            effect.cast(),
            form_id,
            (filter, match_filter),
            ActiveEffect::VM_TYPE_ID,
        )
    }

    #[inline(always)]
    pub fn register_alias<'a>(
        &mut self,
        alias: impl Into<GamePtr<BGSRefAlias>>,
        filter: Filter,
        match_filter: bool,
    ) -> bool {
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

        self.register_object(
            alias.cast(),
            form_id,
            (filter, match_filter),
            BGSRefAlias::VM_TYPE_ID,
        )
    }

    #[inline(always)]
    pub fn unregister_form<'a>(
        &mut self,
        form: impl Into<GamePtr<TESForm>>,
        filter: Filter,
        match_filter: bool,
    ) -> bool {
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
        self.unregister_object(form.cast(), form_id, (filter, match_filter), type_id)
    }

    #[inline(always)]
    pub fn unregister_active_effect<'a>(
        &mut self,
        effect: impl Into<GamePtr<ActiveEffect>>,
        filter: Filter,
        match_filter: bool,
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

        self.unregister_object(
            effect.cast(),
            form_id,
            (filter, match_filter),
            ActiveEffect::VM_TYPE_ID,
        )
    }

    #[inline(always)]
    pub fn unregister_alias<'a>(
        &mut self,
        alias: impl Into<GamePtr<BGSRefAlias>>,
        filter: Filter,
        match_filter: bool,
    ) -> bool {
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

        self.unregister_object(
            alias.cast(),
            form_id,
            (filter, match_filter),
            BGSRefAlias::VM_TYPE_ID,
        )
    }

    #[inline(always)]
    pub fn unregister_all_form(&mut self, form: impl Into<GamePtr<TESForm>>) {
        let form = form.into();
        if form.is_null() {
            return;
        }

        let form = form.as_ptr().cast_const();
        let reference = unsafe { (*(form as *mut TESForm)).as_reference() };
        if reference.is_null() {
            return;
        }

        let form_id = unsafe { (*reference).get_form_id() };
        if form_id == 0 {
            return;
        }

        let type_id = unsafe { (*form).get_form_type() as VMTypeID };
        self.unregister_all_object(form.cast(), form_id, type_id);
    }

    #[inline(always)]
    pub fn unregister_all_active_effect<'a>(&mut self, effect: impl Into<GamePtr<ActiveEffect>>) {
        let effect = effect.into();
        if effect.is_null() {
            return;
        }

        let effect = effect.as_ptr();
        let target = unsafe { (*effect).get_target_actor() };
        if target.is_null() {
            return;
        }

        let form_id = unsafe { (*target).get_form_id() };
        if form_id == 0 {
            return;
        }

        self.unregister_all_object(effect.cast(), form_id, ActiveEffect::VM_TYPE_ID);
    }

    #[inline(always)]
    pub fn unregister_all_alias(&mut self, alias: impl Into<GamePtr<BGSRefAlias>>) {
        let alias = alias.into();
        if alias.is_null() {
            return;
        }

        let alias = alias.as_ptr();
        let target = unsafe { (*alias).get_actor_reference() };
        if target.is_null() {
            return;
        }

        let form_id = unsafe { (*target).get_form_id() };
        if form_id == 0 {
            return;
        }

        self.unregister_all_object(alias.cast(), form_id, BGSRefAlias::VM_TYPE_ID);
    }

    pub fn unregister_all_handle(&mut self, handle: VMHandle) {
        let policy = get_handle_policy();
        if policy.is_null() {
            return;
        }

        for handles_by_filter in self.regs.values_mut() {
            for handles in handles_by_filter.values_mut() {
                if handles.remove(&handle) {
                    unsafe {
                        (*policy).release_handle(handle);
                    }
                }
            }
        }
    }

    pub fn unregister_unique_id(&mut self, unique_id: u32) {
        let policy = get_handle_policy();
        if policy.is_null() {
            return;
        }

        if let Some(handles_by_filter) = self.regs.remove(&unique_id) {
            for handles in handles_by_filter.into_values() {
                for handle in handles {
                    unsafe {
                        (*policy).release_handle(handle);
                    }
                }
            }
        }
    }

    pub fn clear(&mut self) {
        release_unique_filter_regs(&self.regs);
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

        for (form_id, handles_by_filter) in &self.regs {
            if !serialization.write_record_data(
                (form_id as *const u32).cast(),
                core::mem::size_of::<u32>() as u32,
            ) {
                return false;
            }

            let num_filters = handles_by_filter.len();
            if !serialization.write_record_data(
                (&num_filters as *const usize).cast(),
                core::mem::size_of::<usize>() as u32,
            ) {
                return false;
            }

            for ((filter, match_filter), handles) in handles_by_filter {
                if !filter.save_filter(serialization) {
                    return false;
                }

                if !serialization.write_record_data(
                    (match_filter as *const bool).cast(),
                    core::mem::size_of::<bool>() as u32,
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
            let mut form_id = 0_u32;
            serialization.read_record_data(
                (&mut form_id as *mut u32).cast(),
                core::mem::size_of::<u32>() as u32,
            );

            let mut resolved_form_id = form_id;
            let resolved = serialization.resolve_form_id(form_id, &mut resolved_form_id);

            let mut num_filters = 0_usize;
            serialization.read_record_data(
                (&mut num_filters as *mut usize).cast(),
                core::mem::size_of::<usize>() as u32,
            );

            for _ in 0..num_filters {
                let mut filter = Filter::default();
                let loaded_filter = filter.load_filter(serialization);

                let mut match_filter = false;
                serialization.read_record_data(
                    (&mut match_filter as *mut bool).cast(),
                    core::mem::size_of::<bool>() as u32,
                );

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
                    if resolved
                        && loaded_filter
                        && serialization.resolve_handle(handle, &mut resolved_handle)
                    {
                        self.regs
                            .entry(resolved_form_id)
                            .or_default()
                            .entry((filter.clone(), match_filter))
                            .or_default()
                            .insert(resolved_handle);
                    }
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
        form_id: u32,
        filter: UniqueEventFilter<Filter>,
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

        let inserted = self
            .regs
            .entry(form_id)
            .or_default()
            .entry(filter)
            .or_default()
            .insert(handle);
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
        filter: UniqueEventFilter<Filter>,
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

        if let Some(handles_by_filter) = self.regs.get_mut(&form_id) {
            if let Some(handles) = handles_by_filter.get_mut(&filter) {
                if handles.remove(&handle) {
                    unsafe {
                        (*policy).release_handle(handle);
                    }
                    return true;
                }
            }
        }

        false
    }

    fn unregister_all_object(&mut self, object: *const c_void, form_id: u32, type_id: VMTypeID) {
        let policy = get_handle_policy();
        if policy.is_null() {
            return;
        }

        let invalid = unsafe { (*policy).empty_handle() };
        let handle = unsafe { (*policy).get_handle_for_object(type_id, object) };
        if handle == invalid {
            return;
        }

        if let Some(handles_by_filter) = self.regs.get_mut(&form_id) {
            for handles in handles_by_filter.values_mut() {
                if handles.remove(&handle) {
                    unsafe {
                        (*policy).release_handle(handle);
                    }
                }
            }
        }
    }
}

/// Rust-side port of `SKSE::RegistrationMapUnique<Filter, Args...>`.
pub struct RegistrationMapUnique<Filter: RegistrationFilter, Args = ()> {
    base: RegistrationMapUniqueBase<Filter>,
    _marker: PhantomData<fn() -> Args>,
}

impl<Filter: RegistrationFilter, Args> Clone for RegistrationMapUnique<Filter, Args> {
    fn clone(&self) -> Self {
        Self {
            base: self.base.clone(),
            _marker: PhantomData,
        }
    }
}

impl<Filter: RegistrationFilter, Args> RegistrationMapUnique<Filter, Args> {
    #[inline(always)]
    fn send_event_for_target_id(
        &self,
        target_id: u32,
        mut pass_filter: impl FnMut(&Filter, bool) -> bool,
        args: Args,
    ) where
        Args: RegistrationEventArgs,
    {
        let Some(handles_by_filter) = self.base.regs().get(&target_id) else {
            return;
        };

        let event_name = self.base.event_name_fixed();
        let _ = with_vm(|vm| {
            for ((filter, match_filter), handles) in handles_by_filter {
                if !pass_filter(filter, *match_filter) {
                    continue;
                }

                for &handle in handles {
                    let Some(arguments) = args.to_function_arguments(vm) else {
                        continue;
                    };
                    vm.base.send_event(handle, &event_name, arguments.as_ptr());
                }
            }
        });
    }

    #[inline(always)]
    pub fn new(event_name: impl Into<String>) -> Self {
        Self {
            base: RegistrationMapUniqueBase::new(event_name),
            _marker: PhantomData,
        }
    }

    #[inline(always)]
    pub fn base(&self) -> &RegistrationMapUniqueBase<Filter> {
        &self.base
    }

    #[inline(always)]
    pub fn base_mut(&mut self) -> &mut RegistrationMapUniqueBase<Filter> {
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
        form: impl Into<GamePtr<TESForm>>,
        filter: Filter,
        match_filter: bool,
    ) -> bool {
        self.base.register_form(form, filter, match_filter)
    }

    #[inline(always)]
    pub fn register_active_effect<'a>(
        &mut self,
        effect: impl Into<GamePtr<ActiveEffect>>,
        filter: Filter,
        match_filter: bool,
    ) -> bool {
        self.base
            .register_active_effect(effect, filter, match_filter)
    }

    #[inline(always)]
    pub fn register_alias<'a>(
        &mut self,
        alias: impl Into<GamePtr<BGSRefAlias>>,
        filter: Filter,
        match_filter: bool,
    ) -> bool {
        self.base.register_alias(alias, filter, match_filter)
    }

    #[inline(always)]
    pub fn unregister_form<'a>(
        &mut self,
        form: impl Into<GamePtr<TESForm>>,
        filter: Filter,
        match_filter: bool,
    ) -> bool {
        self.base.unregister_form(form, filter, match_filter)
    }

    #[inline(always)]
    pub fn unregister_active_effect<'a>(
        &mut self,
        effect: impl Into<GamePtr<ActiveEffect>>,
        filter: Filter,
        match_filter: bool,
    ) -> bool {
        self.base
            .unregister_active_effect(effect, filter, match_filter)
    }

    #[inline(always)]
    pub fn unregister_alias<'a>(
        &mut self,
        alias: impl Into<GamePtr<BGSRefAlias>>,
        filter: Filter,
        match_filter: bool,
    ) -> bool {
        self.base.unregister_alias(alias, filter, match_filter)
    }

    #[inline(always)]
    pub fn unregister_all_form(&mut self, form: impl Into<GamePtr<TESForm>>) {
        self.base.unregister_all_form(form);
    }

    #[inline(always)]
    pub fn unregister_all_active_effect<'a>(&mut self, effect: impl Into<GamePtr<ActiveEffect>>) {
        self.base.unregister_all_active_effect(effect);
    }

    #[inline(always)]
    pub fn unregister_all_alias(&mut self, alias: impl Into<GamePtr<BGSRefAlias>>) {
        self.base.unregister_all_alias(alias);
    }

    #[inline(always)]
    pub fn unregister_all_handle(&mut self, handle: VMHandle) {
        self.base.unregister_all_handle(handle);
    }

    #[inline(always)]
    pub fn unregister_unique_id(&mut self, unique_id: u32) {
        self.base.unregister_unique_id(unique_id);
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
    pub fn for_each_handle(
        &self,
        unique_id: u32,
        mut pass_filter: impl FnMut(&Filter, bool) -> bool,
        mut f: impl FnMut(VMHandle),
    ) {
        if let Some(handles_by_filter) = self.base.regs().get(&unique_id) {
            for ((filter, match_filter), handles) in handles_by_filter {
                if pass_filter(filter, *match_filter) {
                    for &handle in handles {
                        f(handle);
                    }
                }
            }
        }
    }

    /// Sends a Papyrus event to all handles currently registered under
    /// `target` whose stored filter passes `pass_filter`.
    ///
    /// `Args` is modeled as a Rust tuple pack. For example:
    /// `RegistrationMapUnique<MyFilter, (i32, bool)>` expects
    /// `send_event(target, pass_filter, (42, true))`.
    #[inline(always)]
    pub fn send_event<'a>(
        &self,
        target: impl Into<GamePtr<TESObjectREFR>>,
        pass_filter: impl FnMut(&Filter, bool) -> bool,
        args: Args,
    ) where
        Args: RegistrationEventArgs,
    {
        let target = target.into();
        let Some(target) = target.as_ref() else {
            return;
        };
        self.send_event_for_target_id(target.get_form_id(), pass_filter, args);
    }

    /// Queues a Papyrus event dispatch on the SKSE task interface.
    ///
    /// The task queue may outlive the current stack frame, so this requires a
    /// long-lived registration container reference.
    #[inline(always)]
    pub fn queue_event<'a, PassFilter>(
        &'static self,
        target: impl Into<GamePtr<TESObjectREFR>>,
        pass_filter: PassFilter,
        args: Args,
    ) where
        Filter: Send + Sync + 'static,
        Args: RegistrationEventArgs + Send + 'static,
        PassFilter: FnMut(&Filter, bool) -> bool + Send + 'static,
    {
        let target = target.into();
        let target_id = match target.as_ref() {
            Some(target) => target.get_form_id(),
            None => return,
        };
        super::task::add_task(move || {
            self.send_event_for_target_id(target_id, pass_filter, args);
        });
    }
}
