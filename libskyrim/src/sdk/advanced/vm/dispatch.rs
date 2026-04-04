use crate::re::{
    Awaitable, BSTSmartPointer, Object, PapyrusReturn, PapyrusReturnConvertible, TESForm, VMHandle,
    VMStackID, Variable,
};
use crate::sdk::core::GamePtr;

use super::access::{form_handle, is_empty_handle, vm_interface};
use super::shared::{fixed, with_vm_arguments};
use super::types::{VmArguments, VmDispatchOutcome};

#[inline]
pub fn send_event<Args>(handle: VMHandle, event_name: &str, args: Args) -> bool
where
    Args: VmArguments,
{
    if is_empty_handle(handle) {
        return false;
    }

    let event_name = fixed(event_name);
    with_vm_arguments(args, |vm, args_ptr| {
        vm.base.send_event(handle, &event_name, args_ptr);
        true
    })
    .unwrap_or(false)
}

#[inline]
pub fn send_event_all<Args>(event_name: &str, args: Args) -> bool
where
    Args: VmArguments,
{
    let event_name = fixed(event_name);
    with_vm_arguments(args, |vm, args_ptr| {
        vm.base.send_event_all(&event_name, args_ptr);
        true
    })
    .unwrap_or(false)
}

#[inline]
pub fn send_form_event<Args>(
    form: impl Into<GamePtr<TESForm>>,
    event_name: &str,
    args: Args,
) -> bool
where
    Args: VmArguments,
{
    let Some(handle) = form_handle(form) else {
        return false;
    };
    send_event(handle, event_name, args)
}

#[inline]
pub fn dispatch_static_call<Args>(class_name: &str, fn_name: &str, args: Args) -> VmDispatchOutcome
where
    Args: VmArguments,
{
    let class_name = fixed(class_name);
    let fn_name = fixed(fn_name);
    with_vm_arguments(args, |vm, args_ptr| {
        let mut callback = BSTSmartPointer::default();
        let dispatched =
            vm.base
                .dispatch_static_call(&class_name, &fn_name, args_ptr, &mut callback);
        VmDispatchOutcome::new(dispatched, callback)
    })
    .unwrap_or_default()
}

#[inline]
pub fn dispatch_object_method_call<Args>(
    object: &BSTSmartPointer<Object>,
    fn_name: &str,
    args: Args,
) -> VmDispatchOutcome
where
    Args: VmArguments,
{
    if object.is_null() {
        return VmDispatchOutcome::default();
    }

    let fn_name = fixed(fn_name);
    with_vm_arguments(args, |vm, args_ptr| {
        let mut object = object.clone();
        let mut callback = BSTSmartPointer::default();
        let dispatched =
            vm.base
                .dispatch_method_call_object(&mut object, &fn_name, args_ptr, &mut callback);
        VmDispatchOutcome::new(dispatched, callback)
    })
    .unwrap_or_default()
}

#[inline]
pub fn dispatch_handle_method_call<Args>(
    handle: VMHandle,
    class_name: &str,
    fn_name: &str,
    args: Args,
) -> VmDispatchOutcome
where
    Args: VmArguments,
{
    if is_empty_handle(handle) {
        return VmDispatchOutcome::default();
    }

    let class_name = fixed(class_name);
    let fn_name = fixed(fn_name);
    with_vm_arguments(args, |vm, args_ptr| {
        let mut callback = BSTSmartPointer::default();
        let dispatched = vm.base.dispatch_method_call_handle(
            handle,
            &class_name,
            &fn_name,
            args_ptr,
            &mut callback,
        );
        VmDispatchOutcome::new(dispatched, callback)
    })
    .unwrap_or_default()
}

#[inline]
pub fn dispatch_form_method_call<Args>(
    form: impl Into<GamePtr<TESForm>>,
    class_name: &str,
    fn_name: &str,
    args: Args,
) -> VmDispatchOutcome
where
    Args: VmArguments,
{
    let Some(handle) = form_handle(form) else {
        return VmDispatchOutcome::default();
    };
    dispatch_handle_method_call(handle, class_name, fn_name, args)
}

#[inline]
pub fn await_static_call<Args>(class_name: &str, fn_name: &str, args: Args) -> Awaitable
where
    Args: VmArguments,
{
    let class_name = fixed(class_name);
    let fn_name = fixed(fn_name);
    with_vm_arguments(args, |vm, args_ptr| {
        vm.base
            .adispatch_static_call(&class_name, &fn_name, args_ptr)
    })
    .unwrap_or_default()
}

#[inline]
pub fn await_object_method_call<Args>(
    object: &BSTSmartPointer<Object>,
    fn_name: &str,
    args: Args,
) -> Awaitable
where
    Args: VmArguments,
{
    if object.is_null() {
        return Awaitable::default();
    }

    let fn_name = fixed(fn_name);
    with_vm_arguments(args, |vm, args_ptr| {
        let mut object = object.clone();
        vm.base
            .adispatch_method_call_object(&mut object, &fn_name, args_ptr)
    })
    .unwrap_or_default()
}

#[inline]
pub fn await_handle_method_call<Args>(
    handle: VMHandle,
    class_name: &str,
    fn_name: &str,
    args: Args,
) -> Awaitable
where
    Args: VmArguments,
{
    if is_empty_handle(handle) {
        return Awaitable::default();
    }

    let class_name = fixed(class_name);
    let fn_name = fixed(fn_name);
    with_vm_arguments(args, |vm, args_ptr| {
        vm.base
            .adispatch_method_call_handle(handle, &class_name, &fn_name, args_ptr)
    })
    .unwrap_or_default()
}

#[inline]
pub fn await_form_method_call<Args>(
    form: impl Into<GamePtr<TESForm>>,
    class_name: &str,
    fn_name: &str,
    args: Args,
) -> Awaitable
where
    Args: VmArguments,
{
    let Some(handle) = form_handle(form) else {
        return Awaitable::default();
    };
    await_handle_method_call(handle, class_name, fn_name, args)
}

#[inline(always)]
pub fn return_latent_result<R>(stack_id: VMStackID, result: R) -> bool
where
    R: PapyrusReturn + PapyrusReturnConvertible,
{
    vm_interface().return_latent_result(stack_id, result)
}

#[inline(always)]
pub fn return_latent_variable(stack_id: VMStackID, result: &Variable) {
    vm_interface().return_latent_variable(stack_id, result);
}
