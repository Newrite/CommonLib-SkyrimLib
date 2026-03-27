use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BSScript__Internal__ScriptFunction;
use crate::offsets::offsets_vtable::VTABLE_BSScript__Internal__ScriptFunction;
use crate::re::{
    BSFixedString, FunctionType, IFunction, PackedInstructionStream, TypeInfo, VDescTable,
};
use crate::relocation::{RttiType, VariantID};

#[repr(C)]
pub struct ScriptFunction {
    pub base: IFunction,                                      // 00
    pub name: BSFixedString,                                  // 10
    pub obj_name: BSFixedString,                              // 18
    pub state_name: BSFixedString,                            // 20
    pub ret_type: TypeInfo,                                   // 28
    pub desc_table: VDescTable,                               // 30
    pub user_flags: u32,                                      // 40
    pub pad44: u32,                                           // 44
    pub instructions: PackedInstructionStream,                // 48
    pub function_type: core_util::EnumSet<FunctionType, u16>, // 58
    pub is_static: bool,                                      // 5A
    pub pad5b: u8,                                            // 5B
    pub pad5c: u32,                                           // 5C
    pub doc_string: BSFixedString,                            // 60
    pub source_file_name: BSFixedString,                      // 68
    pub line_number_count: u32,                               // 70
    pub pad74: u32,                                           // 74
    pub line_numbers: *mut u16,                               // 78
}

const _: () = assert!(core::mem::size_of::<ScriptFunction>() == 0x80);

inherit!(ScriptFunction : IFunction);

impl RttiType for ScriptFunction {
    const RTTI: VariantID = RTTI_BSScript__Internal__ScriptFunction;
}

impl ScriptFunction {
    pub const RTTI: VariantID = RTTI_BSScript__Internal__ScriptFunction;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSScript__Internal__ScriptFunction;

    #[inline(always)]
    pub fn get_name_ref(&self) -> &BSFixedString {
        &self.name
    }

    #[inline(always)]
    pub fn get_object_type_name_ref(&self) -> &BSFixedString {
        &self.obj_name
    }

    #[inline(always)]
    pub fn get_state_name_ref(&self) -> &BSFixedString {
        &self.state_name
    }

    #[inline(always)]
    pub fn get_return_type_value(&self) -> TypeInfo {
        self.ret_type
    }
}
