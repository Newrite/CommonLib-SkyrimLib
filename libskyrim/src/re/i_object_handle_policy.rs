use crate::offsets::offsets_rtti::RTTI_BSScript__IObjectHandlePolicy;
use crate::offsets::offsets_vtable::VTABLE_BSScript__IObjectHandlePolicy;
use crate::re::bs_core_types::{VMHandle, VMTypeID};
use crate::re::bs_fixed_string::BSFixedString;
use crate::re::form_type::FormType;
use crate::re::tes_form::TESForm;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::BSScript::IObjectHandlePolicy`
#[repr(C)]
pub struct IObjectHandlePolicy {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<IObjectHandlePolicy>() == 0x8);

impl RttiType for IObjectHandlePolicy {
    const RTTI: VariantID = RTTI_BSScript__IObjectHandlePolicy;
}

impl IObjectHandlePolicy {
    pub const RTTI: VariantID = RTTI_BSScript__IObjectHandlePolicy;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSScript__IObjectHandlePolicy;

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    virtual_method! {
        pub const VFUNC_HANDLE_IS_TYPE: usize = 0x01;
        pub fn handle_is_type(type_id: VMTypeID, handle: VMHandle) -> bool
    }

    virtual_method! {
        pub const VFUNC_IS_HANDLE_OBJECT_AVAILABLE: usize = 0x02;
        pub fn is_handle_object_available(handle: VMHandle) -> bool
    }

    virtual_method! {
        pub const VFUNC_EMPTY_HANDLE: usize = 0x03;
        pub fn empty_handle() -> VMHandle
    }

    virtual_method! {
        pub const VFUNC_GET_HANDLE_FOR_OBJECT: usize = 0x04;
        pub fn get_handle_for_object(type_id: VMTypeID, src_data: *const core::ffi::c_void) -> VMHandle
    }

    virtual_method! {
        pub const VFUNC_HAS_PARENT: usize = 0x05;
        pub fn has_parent(handle: VMHandle) -> bool
    }

    virtual_method! {
        pub const VFUNC_GET_PARENT_HANDLE: usize = 0x06;
        pub fn get_parent_handle(handle: VMHandle) -> VMHandle
    }

    virtual_method! {
        pub const VFUNC_GET_HANDLE_SCRIPTS_MOVED_FROM: usize = 0x07;
        pub fn get_handle_scripts_moved_from(handle: VMHandle) -> VMHandle
    }

    virtual_method! {
        pub const VFUNC_GET_OBJECT_FOR_HANDLE: usize = 0x08;
        pub fn get_object_for_handle(type_id: VMTypeID, handle: VMHandle) -> *mut core::ffi::c_void
    }

    virtual_method! {
        pub const VFUNC_PERSIST_HANDLE: usize = 0x09;
        pub fn persist_handle(handle: VMHandle)
    }

    virtual_method! {
        pub const VFUNC_RELEASE_HANDLE: usize = 0x0A;
        pub fn release_handle(handle: VMHandle)
    }

    virtual_method! {
        pub const VFUNC_CONVERT_HANDLE_TO_STRING: usize = 0x0B;
        pub fn convert_handle_to_string(handle: VMHandle, out: &mut BSFixedString)
    }

    #[inline(always)]
    pub fn handle_is_type_form(&self, type_id: FormType, handle: VMHandle) -> bool {
        self.handle_is_type(type_id as VMTypeID, handle)
    }

    #[inline(always)]
    pub fn get_handle_for_form(&self, type_id: FormType, src_data: *const TESForm) -> VMHandle {
        self.get_handle_for_object(type_id as VMTypeID, src_data.cast())
    }

    #[inline(always)]
    pub fn get_form_for_handle(&self, type_id: FormType, handle: VMHandle) -> *mut TESForm {
        self.get_object_for_handle(type_id as VMTypeID, handle)
            .cast()
    }
}
