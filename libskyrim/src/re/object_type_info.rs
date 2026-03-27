use core_util::inherit;

use crate::re::{
    BSFixedString, BSIntrusiveRefCounted, BSTSmartPointer, BSTSmartPointerIntrusiveRefCountable,
    IFunction, PropertyTypeInfo, TypeInfo, Variable,
};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LinkValidState {
    NotLinked = 0,
    CurrentlyLinking = 1,
    LinkedInvalid = 2,
    LinkedValid = 3,
}

#[repr(C)]
pub struct UnlinkedNativeFunction {
    pub next: *mut UnlinkedNativeFunction, // 00
    pub func: BSTSmartPointer<IFunction>,  // 08
}

const _: () = assert!(core::mem::size_of::<UnlinkedNativeFunction>() == 0x10);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct UserFlagInfo {
    pub data: usize, // 00
}

const _: () = assert!(core::mem::size_of::<UserFlagInfo>() == 0x8);

impl UserFlagInfo {
    pub const SET_ON_OBJECT: usize = 1 << 0;

    #[inline(always)]
    pub fn get_user_flag(&self) -> BSFixedString {
        let sanitized = self.data & !Self::SET_ON_OBJECT;
        unsafe { core::mem::transmute(sanitized) }
    }
}

#[repr(C)]
pub struct VariableInfo {
    pub name: BSFixedString, // 00
    pub type_: TypeInfo,     // 08
}

const _: () = assert!(core::mem::size_of::<VariableInfo>() == 0x10);

#[repr(C)]
pub struct InitialValueInfo {
    pub variable_index: u32,     // 00
    pub pad04: u32,              // 04
    pub initial_value: Variable, // 08
}

const _: () = assert!(core::mem::size_of::<InitialValueInfo>() == 0x18);

#[repr(C)]
pub struct PropertyInfo {
    pub name: BSFixedString,    // 00
    pub info: PropertyTypeInfo, // 08
}

const _: () = assert!(core::mem::size_of::<PropertyInfo>() == 0x48);

#[repr(C)]
pub struct GlobalFuncInfo {
    pub func: BSTSmartPointer<IFunction>, // 00
}

const _: () = assert!(core::mem::size_of::<GlobalFuncInfo>() == 0x8);

#[repr(C)]
pub struct MemberFuncInfo {
    pub func: BSTSmartPointer<IFunction>, // 00
}

const _: () = assert!(core::mem::size_of::<MemberFuncInfo>() == 0x8);

#[repr(C)]
pub struct NamedStateFunc {
    pub func: BSTSmartPointer<IFunction>, // 00
}

const _: () = assert!(core::mem::size_of::<NamedStateFunc>() == 0x8);

#[repr(C)]
pub struct NamedStateInfo {
    pub name: BSFixedString,       // 00
    pub member_function_data: u32, // 08
    pub pad0c: u32,                // 0C
}

const _: () = assert!(core::mem::size_of::<NamedStateInfo>() == 0x10);

impl NamedStateInfo {
    pub const FUNC_COUNT_MASK: u32 = 0x1FF;
    pub const FUNC_OFFSET_SHIFT: u32 = 9;

    #[inline(always)]
    pub const fn get_num_funcs(&self) -> u32 {
        self.member_function_data & Self::FUNC_COUNT_MASK
    }

    #[inline(always)]
    pub const fn get_member_function_offset(&self) -> u32 {
        (self.member_function_data >> Self::FUNC_OFFSET_SHIFT) & Self::FUNC_COUNT_MASK
    }

    #[inline(always)]
    pub fn get_func_iter(&self) -> *mut NamedStateFunc {
        (self as *const Self as usize + self.get_member_function_offset() as usize)
            as *mut NamedStateFunc
    }
}

#[repr(C)]
pub struct ObjectTypeInfo {
    pub base: BSIntrusiveRefCounted,                       // 00
    pub name: BSFixedString,                               // 08
    pub parent_type_info: BSTSmartPointer<ObjectTypeInfo>, // 10
    pub doc_string: BSFixedString,                         // 18
    pub counts20: u32,                                     // 20
    pub counts24: u32,                                     // 24
    pub counts28: u32,                                     // 28
    pub pad2c: u32,                                        // 2C
    pub data: *mut core::ffi::c_void,                      // 30
}

const _: () = assert!(core::mem::size_of::<ObjectTypeInfo>() == 0x38);
const _: () = assert!(core::mem::offset_of!(ObjectTypeInfo, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(ObjectTypeInfo, name) == 0x08);
const _: () = assert!(core::mem::offset_of!(ObjectTypeInfo, parent_type_info) == 0x10);
const _: () = assert!(core::mem::offset_of!(ObjectTypeInfo, doc_string) == 0x18);
const _: () = assert!(core::mem::offset_of!(ObjectTypeInfo, counts20) == 0x20);
const _: () = assert!(core::mem::offset_of!(ObjectTypeInfo, counts24) == 0x24);
const _: () = assert!(core::mem::offset_of!(ObjectTypeInfo, counts28) == 0x28);
const _: () = assert!(core::mem::offset_of!(ObjectTypeInfo, data) == 0x30);

inherit!(ObjectTypeInfo : BSIntrusiveRefCounted);

impl BSTSmartPointerIntrusiveRefCountable for ObjectTypeInfo {
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
        let this = (self as *const Self).cast_mut();
        unsafe {
            (*this).release_data();
            crate::ffi::commonlib_free(this.cast());
        }
    }
}

impl ObjectTypeInfo {
    #[inline(always)]
    pub const fn get_linked_valid_state(&self) -> LinkValidState {
        match self.counts20 & 0x3 {
            0 => LinkValidState::NotLinked,
            1 => LinkValidState::CurrentlyLinking,
            2 => LinkValidState::LinkedInvalid,
            _ => LinkValidState::LinkedValid,
        }
    }

    #[inline(always)]
    pub const fn get_num_user_flags(&self) -> u32 {
        (self.counts20 >> 2) & 0x3F
    }

    #[inline(always)]
    pub const fn get_num_variables(&self) -> u32 {
        (self.counts20 >> 8) & 0x3FF
    }

    #[inline(always)]
    pub const fn get_num_initial_values(&self) -> u32 {
        self.counts24 & 0x3FF
    }

    #[inline(always)]
    pub const fn get_num_properties(&self) -> u32 {
        (self.counts24 >> 10) & 0x3FF
    }

    #[inline(always)]
    pub const fn get_num_global_funcs(&self) -> u32 {
        (self.counts24 >> 20) & 0x1FF
    }

    #[inline(always)]
    pub const fn get_num_member_funcs(&self) -> u32 {
        self.counts28 & 0x7FF
    }

    #[inline(always)]
    pub const fn get_num_named_states(&self) -> u32 {
        (self.counts28 >> 11) & 0x7F
    }

    #[inline(always)]
    pub fn is_linked(&self) -> bool {
        matches!(
            self.get_linked_valid_state(),
            LinkValidState::LinkedInvalid | LinkValidState::LinkedValid
        )
    }

    #[inline(always)]
    pub fn get_parent(&self) -> *mut ObjectTypeInfo {
        self.parent_type_info.get()
    }

    #[inline(always)]
    pub fn get_raw_type(&self) -> core_util::EnumSet<crate::re::type_info::RawType, usize> {
        core_util::EnumSet::from_underlying(self as *const Self as usize)
    }

    #[inline(always)]
    pub fn get_unlinked_function_iter(&self) -> *mut UnlinkedNativeFunction {
        self.data.cast()
    }

    #[inline(always)]
    pub fn get_user_flag_iter(&self) -> *mut UserFlagInfo {
        self.data.cast()
    }

    #[inline(always)]
    pub fn get_total_num_variables(&self) -> u32 {
        let mut vars = self.get_num_variables();
        let mut iter = self.get_parent();
        while !iter.is_null() {
            unsafe {
                vars = vars.wrapping_add((*iter).get_num_variables());
                iter = (*iter).get_parent();
            }
        }
        vars
    }

    #[inline(always)]
    pub fn get_variable_iter(&self) -> *mut VariableInfo {
        unsafe {
            self.get_user_flag_iter()
                .add(self.get_num_user_flags() as usize)
                .cast()
        }
    }

    #[inline(always)]
    pub fn get_initial_value_iter(&self) -> *mut InitialValueInfo {
        unsafe {
            self.get_variable_iter()
                .add(self.get_num_variables() as usize)
                .cast()
        }
    }

    #[inline(always)]
    pub fn get_property_iter(&self) -> *mut PropertyInfo {
        unsafe {
            self.get_initial_value_iter()
                .add(self.get_num_initial_values() as usize)
                .cast()
        }
    }

    #[inline(always)]
    pub fn get_global_func_iter(&self) -> *mut GlobalFuncInfo {
        unsafe {
            self.get_property_iter()
                .add(self.get_num_properties() as usize)
                .cast()
        }
    }

    #[inline(always)]
    pub fn get_member_func_iter(&self) -> *mut MemberFuncInfo {
        unsafe {
            self.get_global_func_iter()
                .add(self.get_num_global_funcs() as usize)
                .cast()
        }
    }

    #[inline(always)]
    pub fn get_named_state_iter(&self) -> *mut NamedStateInfo {
        unsafe {
            self.get_member_func_iter()
                .add(self.get_num_member_funcs() as usize)
                .cast()
        }
    }

    #[inline(always)]
    pub fn get_property_index(&self, name: &BSFixedString) -> u32 {
        let props = self.get_property_iter();
        if !props.is_null() {
            for i in 0..self.get_num_properties() {
                let prop = unsafe { &*props.add(i as usize) };
                if prop.name == *name {
                    return prop.info.auto_var_index;
                }
            }
        }
        u32::MAX
    }

    crate::relocation_func! {
        pub fn release_data(&mut self) => crate::relocation::ID::new(97538)
    }
}
