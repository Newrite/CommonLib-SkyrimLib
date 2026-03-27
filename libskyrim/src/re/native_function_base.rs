use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BSScript__NF_util__NativeFunctionBase;
use crate::offsets::offsets_vtable::VTABLE_BSScript__NF_util__NativeFunctionBase;
use crate::re::bs_core_types::VMStackID;
use crate::re::bs_fixed_string::BSFixedString;
use crate::re::bst_smart_pointer::BSTSmartPointer;
use crate::re::error_logger::ErrorLogger;
use crate::re::ifunction::IFunction;
use crate::re::stack::Stack;
use crate::re::stack_frame::StackFrame;
use crate::re::type_info::TypeInfo;
use crate::re::v_desc_table::VDescTable;
use crate::re::variable::Variable;
use crate::re::virtual_machine::VirtualMachine;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::BSScript::NF_util::NativeFunctionBase`
#[repr(C)]
pub struct NativeFunctionBase {
    pub base: IFunction,                // 00
    pub name: BSFixedString,            // 10
    pub obj_name: BSFixedString,        // 18
    pub state_name: BSFixedString,      // 20
    pub ret_type: TypeInfo,             // 28
    pub desc_table: VDescTable,         // 30
    pub is_static: bool,                // 40
    pub is_callable_from_tasklet: bool, // 41
    pub is_latent: bool,                // 42
    pub pad43: u8,                      // 43
    pub user_flags: u32,                // 44
    pub doc_string: BSFixedString,      // 48
}

const _: () = assert!(core::mem::size_of::<NativeFunctionBase>() == 0x50);
const _: () = assert!(core::mem::offset_of!(NativeFunctionBase, base) == 0x0);
const _: () = assert!(core::mem::offset_of!(NativeFunctionBase, name) == 0x10);
const _: () = assert!(core::mem::offset_of!(NativeFunctionBase, obj_name) == 0x18);
const _: () = assert!(core::mem::offset_of!(NativeFunctionBase, state_name) == 0x20);
const _: () = assert!(core::mem::offset_of!(NativeFunctionBase, ret_type) == 0x28);
const _: () = assert!(core::mem::offset_of!(NativeFunctionBase, desc_table) == 0x30);
const _: () = assert!(core::mem::offset_of!(NativeFunctionBase, is_static) == 0x40);
const _: () = assert!(core::mem::offset_of!(NativeFunctionBase, is_callable_from_tasklet) == 0x41);
const _: () = assert!(core::mem::offset_of!(NativeFunctionBase, is_latent) == 0x42);
const _: () = assert!(core::mem::offset_of!(NativeFunctionBase, user_flags) == 0x44);
const _: () = assert!(core::mem::offset_of!(NativeFunctionBase, doc_string) == 0x48);

inherit!(NativeFunctionBase : IFunction);

impl RttiType for NativeFunctionBase {
    const RTTI: VariantID = RTTI_BSScript__NF_util__NativeFunctionBase;
}

impl NativeFunctionBase {
    pub const RTTI: VariantID = RTTI_BSScript__NF_util__NativeFunctionBase;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSScript__NF_util__NativeFunctionBase;

    crate::virtual_method! {
        pub const VFUNC_HAS_STUB: usize = 0x15;
        pub fn has_stub() -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_MARSHALL_AND_DISPATCH: usize = 0x16;
        pub fn marshall_and_dispatch(
            base_value: &mut Variable,
            vm: &mut VirtualMachine,
            stack_id: VMStackID,
            result_value: &mut Variable,
            frame: &StackFrame
        ) -> bool
    }

    #[inline(always)]
    pub fn get_is_latent(&self) -> bool {
        self.is_latent
    }

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

    #[inline(always)]
    pub fn get_param_count_value(&self) -> u32 {
        self.desc_table.total_entries as u32
    }

    #[inline(always)]
    pub fn get_stack_frame_size_value(&self) -> u32 {
        self.desc_table.total_entries as u32
    }

    #[inline(always)]
    pub fn get_is_native_value(&self) -> bool {
        true
    }

    #[inline(always)]
    pub fn get_is_static_value(&self) -> bool {
        self.is_static
    }

    #[inline(always)]
    pub fn get_is_empty_value(&self) -> bool {
        false
    }

    #[inline(always)]
    pub fn get_user_flags_value(&self) -> u32 {
        self.user_flags
    }

    #[inline(always)]
    pub fn get_doc_string_ref(&self) -> &BSFixedString {
        &self.doc_string
    }

    #[inline(always)]
    pub fn insert_locals_value(&mut self, _frame: *mut StackFrame) {}

    #[inline(always)]
    pub fn get_param_value(&self, idx: u32, name_out: &mut BSFixedString, type_out: &mut TypeInfo) {
        let entries = self.desc_table.entries.as_slice();
        if idx < self.desc_table.param_count as u32 {
            let entry = &entries[idx as usize];
            *name_out = entry.first.clone();
            *type_out = entry.second;
        } else {
            *name_out = BSFixedString::empty();
            type_out.set_type(crate::re::type_info::RawType::None);
        }
    }

    #[inline(always)]
    pub fn get_var_name_for_stack_index_value(
        &self,
        idx: u32,
        name_out: &mut BSFixedString,
    ) -> bool {
        let entries = self.desc_table.entries.as_slice();
        if idx < self.desc_table.total_entries as u32 {
            *name_out = entries[idx as usize].first.clone();
            true
        } else {
            *name_out = BSFixedString::empty();
            false
        }
    }

    #[inline(always)]
    pub fn can_be_called_from_tasklets_value(&self) -> bool {
        self.is_callable_from_tasklet
    }

    #[inline(always)]
    pub fn set_callable_from_tasklets_value(&mut self, callable: bool) {
        self.is_callable_from_tasklet = callable;
    }

    #[inline(always)]
    pub fn call_value(
        &mut self,
        stack: &BSTSmartPointer<Stack>,
        logger: *mut ErrorLogger,
        vm: *mut VirtualMachine,
        arg4: bool,
    ) -> crate::re::ifunction::CallResult {
        IFunction::call(self, stack, logger, vm, arg4)
    }
}
