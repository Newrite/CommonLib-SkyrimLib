use crate::re::{
    BSFixedString, IObjectHandlePolicy, IVirtualMachine, ObjectBindPolicy, SkyrimVM, TESForm,
    VMHandle, VirtualMachine,
};
use crate::sdk::core::{GamePtr, GameRef};

#[inline(always)]
pub fn skyrim_vm() -> GameRef<SkyrimVM> {
    unsafe {
        GameRef::try_from_raw(SkyrimVM::get_singleton())
            .expect("SkyrimVM singleton must be available")
    }
}

#[inline(always)]
pub fn virtual_machine() -> GameRef<VirtualMachine> {
    unsafe {
        GameRef::try_from_raw(VirtualMachine::get_singleton())
            .expect("BSScript virtual machine singleton must be available")
    }
}

#[inline(always)]
pub fn vm_interface() -> GameRef<IVirtualMachine> {
    unsafe {
        GameRef::try_from_raw(skyrim_vm().get_virtual_machine())
            .expect("BSScript IVirtualMachine must be available")
    }
}

#[inline(always)]
pub fn handle_policy() -> GamePtr<IObjectHandlePolicy> {
    unsafe { GamePtr::from_raw(vm_interface().get_object_handle_policy()) }
}

#[inline(always)]
pub fn object_bind_policy() -> GamePtr<ObjectBindPolicy> {
    unsafe { GamePtr::from_raw(vm_interface().get_object_bind_policy()) }
}

#[inline(always)]
pub fn is_empty_handle(handle: VMHandle) -> bool {
    let Some(policy) = handle_policy().into_option() else {
        return handle == 0;
    };
    handle == policy.empty_handle()
}

#[inline(always)]
pub fn is_handle_available(handle: VMHandle) -> bool {
    let Some(policy) = handle_policy().into_option() else {
        return false;
    };
    !is_empty_handle(handle) && policy.is_handle_object_available(handle)
}

#[inline]
pub fn handle_display_string(handle: VMHandle) -> BSFixedString {
    let Some(policy) = handle_policy().into_option() else {
        return BSFixedString::default();
    };
    if handle == policy.empty_handle() {
        return BSFixedString::default();
    }

    let mut out = BSFixedString::default();
    policy.convert_handle_to_string(handle, &mut out);
    out
}

#[inline]
pub fn form_handle(form: impl Into<GamePtr<TESForm>>) -> Option<VMHandle> {
    let form = form.into();
    let form = form.into_option()?;
    let policy = handle_policy().into_option()?;
    let handle = policy.get_handle_for_form(form.get_form_type(), form.as_ptr());
    if handle == policy.empty_handle() {
        None
    } else {
        Some(handle)
    }
}
