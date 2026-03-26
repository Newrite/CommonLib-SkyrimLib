use core::ffi::c_void;

use crate::re::TESObjectREFR;
use crate::relocation::RelocationID;

/// C++ `RE::BGSEntryPointFunction::ENTRY_POINT_FUNCTION`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BGSEntryPointFunctionEntryPointFunction {
    NullFunction = 0,
    SetValue = 1,
    AddValue = 2,
    MultiplyValue = 3,
    AddRangeToValue = 4,
    AddActorValueMult = 5,
    AbsoluteValue = 6,
    NegativeAbsoluteValue = 7,
    AddLeveledList = 8,
    AddActivateChoice = 9,
    SelectSpell = 10,
    SelectText = 11,
    SetToActorValueMult = 12,
    MultiplyActorValueMult = 13,
    MultiplyOnePlusActorValueMult = 14,
    SetText = 15,
    Total = 16,
}

/// C++ `RE::BGSEntryPointFunction::ENTRY_POINT_FUNCTION_TYPE`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BGSEntryPointFunctionType {
    Value = 0,
    AddLeveledList = 1,
    AddActivateChoice = 2,
    Null = 3,
    SelectSpell = 4,
    SelectText = 5,
    SetText = 6,
    Total = 7,
}

/// C++ `RE::BGSEntryPointFunction::EntryPointFunction`
#[repr(C)]
pub struct BGSEntryPointFunctionInfo {
    pub name: *const i8,                          // 00
    pub function_type: BGSEntryPointFunctionType, // 08
    pub pad0c: u32,                               // 0C
    pub function: Option<
        unsafe extern "C" fn(
            *mut TESObjectREFR,
            BGSEntryPointFunctionType,
            u8,
            *mut *mut c_void,
            *mut c_void,
        ),
    >, // 10
}

const _: () = assert!(core::mem::size_of::<BGSEntryPointFunctionInfo>() == 0x18);
const _: () = assert!(core::mem::offset_of!(BGSEntryPointFunctionInfo, function_type) == 0x08);
const _: () = assert!(core::mem::offset_of!(BGSEntryPointFunctionInfo, function) == 0x10);

/// C++ `RE::BGSEntryPointFunction`
pub struct BGSEntryPointFunction;

impl BGSEntryPointFunction {
    crate::relocation_variable! {
        fn entry_point_function_type_argument_count()
            -> *mut u32
            => RelocationID::new(502187, 369210), is_ptr
    }

    crate::relocation_variable! {
        fn entry_point_functions()
            -> *mut BGSEntryPointFunctionInfo
            => RelocationID::new(675799, 369178), is_ptr
    }

    #[inline(always)]
    pub fn get_argument_count(function_type: BGSEntryPointFunctionType) -> u32 {
        let index = function_type as usize;
        if index < BGSEntryPointFunctionType::Total as usize {
            unsafe { *Self::entry_point_function_type_argument_count().add(index) }
        } else {
            0
        }
    }

    #[inline(always)]
    pub fn get_entry_point_function(
        function: BGSEntryPointFunctionEntryPointFunction,
    ) -> *mut BGSEntryPointFunctionInfo {
        let index = function as usize;
        if index < BGSEntryPointFunctionEntryPointFunction::Total as usize {
            unsafe { Self::entry_point_functions().add(index) }
        } else {
            core::ptr::null_mut()
        }
    }
}
