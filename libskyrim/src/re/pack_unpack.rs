use core::ffi::c_void;

use crate::re::{
    BSTSmartPointer, IObjectHandlePolicy, Object, ObjectTypeInfo, RawType, TypeInfo, VMTypeID,
    Variable, VirtualMachine,
};

/// Source-backed Rust translation of the runtime helper layer from
/// `RE/P/PackUnpack.h` and `PackUnpack.cpp`.
#[inline(always)]
pub fn get_raw_type_from_vm_type(type_id: VMTypeID) -> Option<core_util::EnumSet<RawType, usize>> {
    let vm = VirtualMachine::get_singleton();
    if vm.is_null() {
        return None;
    }

    get_raw_type_from_vm_type_vm(unsafe { &*vm }, type_id)
}

#[inline(always)]
pub fn get_raw_type_from_vm_type_vm(
    vm: &crate::re::IVirtualMachine,
    type_id: VMTypeID,
) -> Option<core_util::EnumSet<RawType, usize>> {
    let mut class_ptr = BSTSmartPointer::<ObjectTypeInfo>::default();
    if vm.get_script_object_type_by_id(type_id, &mut class_ptr) && !class_ptr.is_null() {
        Some(unsafe { (*class_ptr.get()).get_raw_type() })
    } else {
        None
    }
}

#[inline(always)]
pub fn get_type_info_from_vm_type_vm(
    vm: &crate::re::IVirtualMachine,
    type_id: VMTypeID,
) -> Option<TypeInfo> {
    Some(TypeInfo {
        raw_type: get_raw_type_from_vm_type_vm(vm, type_id)?,
    })
}

#[inline(always)]
pub fn bind_id(object_ptr: &mut BSTSmartPointer<Object>, src: *const c_void, type_id: VMTypeID) {
    let vm = VirtualMachine::get_singleton();
    if vm.is_null() || object_ptr.is_null() {
        return;
    }

    let vm = unsafe { &mut *vm };
    let type_info = unsafe { (*object_ptr.get()).get_type_info() };
    if type_info.is_null() {
        return;
    }

    let mut id = 0;
    if !vm.get_type_id_for_script_object(unsafe { &(*type_info).name }, &mut id) {
        return;
    }

    let handle_policy = vm.get_object_handle_policy();
    if handle_policy.is_null() {
        return;
    }

    let handle_policy = unsafe { &*handle_policy.cast::<IObjectHandlePolicy>() };
    let handle = handle_policy.get_handle_for_object(type_id, src);
    if !(handle_policy.handle_is_type(id, handle)
        && handle_policy.is_handle_object_available(handle))
    {
        return;
    }

    let bind_policy = vm.get_object_bind_policy();
    if bind_policy.is_null() {
        return;
    }

    unsafe {
        (*bind_policy).bind_object(object_ptr, handle);
    }
}

#[inline(always)]
pub fn pack_handle(dst: &mut Variable, src: *const c_void, type_id: VMTypeID) {
    dst.set_none();
    if src.is_null() {
        return;
    }

    let vm = VirtualMachine::get_singleton();
    if vm.is_null() {
        return;
    }

    let vm = unsafe { &mut *vm };
    let mut class_ptr = BSTSmartPointer::<ObjectTypeInfo>::default();
    if !vm.get_script_object_type_by_id(type_id, &mut class_ptr) || class_ptr.is_null() {
        return;
    }

    let policy = vm.get_object_handle_policy();
    if policy.is_null() {
        return;
    }

    let policy = unsafe { &*policy.cast::<IObjectHandlePolicy>() };
    let handle = policy.get_handle_for_object(type_id, src);
    let Some(class_name) = (unsafe { (*class_ptr.get()).name.as_c_str() }) else {
        return;
    };

    let mut object_ptr = BSTSmartPointer::<Object>::default();
    if !vm.find_bound_object(handle, class_name.as_ptr(), &mut object_ptr) {
        if vm.create_object(unsafe { &(*class_ptr.get()).name }, &mut object_ptr)
            && !object_ptr.is_null()
        {
            bind_id(&mut object_ptr, src, type_id);
        }
    }

    if !object_ptr.is_null() {
        dst.set_object(object_ptr);
    }
}

#[inline(always)]
pub fn unpack_handle(src: &Variable, type_id: VMTypeID) -> *mut c_void {
    let object = src.get_object();
    if object.is_null() {
        core::ptr::null_mut()
    } else {
        unsafe { (*object.get()).resolve(type_id) }
    }
}
