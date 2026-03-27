use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BSScript__IFunction;
use crate::offsets::offsets_vtable::VTABLE_BSScript__IFunction;
use crate::re::bs_fixed_string::BSFixedString;
use crate::re::bs_intrusive_ref_counted::BSIntrusiveRefCounted;
use crate::re::bst_smart_pointer::{BSTSmartPointer, BSTSmartPointerIntrusiveRefCountable};
use crate::re::error_logger::ErrorLogger;
use crate::re::stack::Stack;
use crate::re::stack_frame::StackFrame;
use crate::re::type_info::TypeInfo;
use crate::re::virtual_machine::VirtualMachine;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::BSScript::IFunction::FunctionType`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FunctionType {
    Normal = 0,
    Getter = 1,
    Setter = 2,
}

core_util::impl_enumset_type!(FunctionType => u16);

/// C++ `RE::BSScript::IFunction::CallResult`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CallResult {
    Completed = 0,
    SetupForVM = 1,
    InProgress = 2,
    FailedRetry = 3,
    FailedAbort = 4,
}

/// C++ `RE::BSScript::IFunction`
#[repr(C)]
pub struct IFunction {
    pub vtable: *const usize,        // 00
    pub base: BSIntrusiveRefCounted, // 08
    pub pad0c: u32,                  // 0C
}

const _: () = assert!(core::mem::size_of::<IFunction>() == 0x10);
const _: () = assert!(core::mem::offset_of!(IFunction, base) == 0x08);
const _: () = assert!(core::mem::offset_of!(IFunction, pad0c) == 0x0C);

inherit!(IFunction : BSIntrusiveRefCounted);

impl RttiType for IFunction {
    const RTTI: VariantID = RTTI_BSScript__IFunction;
}

impl BSTSmartPointerIntrusiveRefCountable for IFunction {
    #[inline(always)]
    fn bst_inc_ref(&self) {
        self.base.inc_ref();
    }

    #[inline(always)]
    fn bst_dec_ref(&self) -> u32 {
        self.base.dec_ref()
    }

    #[inline(always)]
    unsafe fn bst_delete(&self) {
        unsafe {
            ((self as *const Self).cast_mut())
                .as_mut()
                .unwrap_unchecked()
                .dtor()
        };
    }
}

impl IFunction {
    pub const RTTI: VariantID = RTTI_BSScript__IFunction;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSScript__IFunction;

    pub const VFUNC_GET_NAME: usize = 0x01;
    pub const VFUNC_GET_OBJECT_TYPE_NAME: usize = 0x02;
    pub const VFUNC_GET_STATE_NAME: usize = 0x03;
    pub const VFUNC_GET_DOC_STRING: usize = 0x0D;
    pub const VFUNC_GET_SOURCE_FILENAME: usize = 0x10;

    crate::virtual_method! { pub const VFUNC_DTOR: usize = 0x00; pub fn dtor(&mut self) }
    crate::virtual_method! { pub const VFUNC_GET_RETURN_TYPE: usize = 0x04; pub fn get_return_type() -> TypeInfo }
    crate::virtual_method! { pub const VFUNC_GET_PARAM_COUNT: usize = 0x05; pub fn get_param_count() -> u32 }
    crate::virtual_method! { pub const VFUNC_GET_PARAM: usize = 0x06; pub fn get_param(idx: u32, name_out: &mut BSFixedString, type_out: &mut TypeInfo) }
    crate::virtual_method! { pub const VFUNC_GET_STACK_FRAME_SIZE: usize = 0x07; pub fn get_stack_frame_size() -> u32 }
    crate::virtual_method! { pub const VFUNC_GET_IS_NATIVE: usize = 0x08; pub fn get_is_native() -> bool }
    crate::virtual_method! { pub const VFUNC_GET_IS_STATIC: usize = 0x09; pub fn get_is_static() -> bool }
    crate::virtual_method! { pub const VFUNC_GET_IS_EMPTY: usize = 0x0A; pub fn get_is_empty() -> bool }
    crate::virtual_method! { pub const VFUNC_GET_FUNCTION_TYPE: usize = 0x0B; pub fn get_function_type() -> FunctionType }
    crate::virtual_method! { pub const VFUNC_GET_USER_FLAGS: usize = 0x0C; pub fn get_user_flags() -> u32 }
    crate::virtual_method! { pub const VFUNC_INSERT_LOCALS: usize = 0x0E; pub fn insert_locals(&mut self, frame: *mut StackFrame) }
    crate::virtual_method! { pub const VFUNC_CALL: usize = 0x0F; pub fn call(&mut self, stack: &BSTSmartPointer<Stack>, logger: *mut ErrorLogger, vm: *mut VirtualMachine, arg4: bool) -> CallResult }
    crate::virtual_method! { pub const VFUNC_TRANSLATE_IP_TO_LINE_NUMBER: usize = 0x11; pub fn translate_ip_to_line_number(index_ptr: u32, line_number_out: &mut u32) -> bool }
    crate::virtual_method! { pub const VFUNC_GET_VAR_NAME_FOR_STACK_INDEX: usize = 0x12; pub fn get_var_name_for_stack_index(idx: u32, name_out: &mut BSFixedString) -> bool }
    crate::virtual_method! { pub const VFUNC_CAN_BE_CALLED_FROM_TASKLETS: usize = 0x13; pub fn can_be_called_from_tasklets() -> bool }
    crate::virtual_method! { pub const VFUNC_SET_CALLABLE_FROM_TASKLETS: usize = 0x14; pub fn set_callable_from_tasklets(&mut self, callable: bool) }

    #[inline(always)]
    pub fn get_name(&self) -> &BSFixedString {
        let func: extern "C" fn(*const Self) -> *const BSFixedString = unsafe {
            crate::relocation::virtual_function(self as *const Self, Self::VFUNC_GET_NAME)
        };
        unsafe { &*func(self) }
    }

    #[inline(always)]
    pub fn get_object_type_name(&self) -> &BSFixedString {
        let func: extern "C" fn(*const Self) -> *const BSFixedString = unsafe {
            crate::relocation::virtual_function(
                self as *const Self,
                Self::VFUNC_GET_OBJECT_TYPE_NAME,
            )
        };
        unsafe { &*func(self) }
    }

    #[inline(always)]
    pub fn get_state_name(&self) -> &BSFixedString {
        let func: extern "C" fn(*const Self) -> *const BSFixedString = unsafe {
            crate::relocation::virtual_function(self as *const Self, Self::VFUNC_GET_STATE_NAME)
        };
        unsafe { &*func(self) }
    }

    #[inline(always)]
    pub fn get_doc_string(&self) -> &BSFixedString {
        let func: extern "C" fn(*const Self) -> *const BSFixedString = unsafe {
            crate::relocation::virtual_function(self as *const Self, Self::VFUNC_GET_DOC_STRING)
        };
        unsafe { &*func(self) }
    }

    #[inline(always)]
    pub fn get_source_filename(&self) -> &BSFixedString {
        let func: extern "C" fn(*const Self) -> *const BSFixedString = unsafe {
            crate::relocation::virtual_function(
                self as *const Self,
                Self::VFUNC_GET_SOURCE_FILENAME,
            )
        };
        unsafe { &*func(self) }
    }
}
