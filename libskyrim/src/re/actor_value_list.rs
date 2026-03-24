use core::ffi::c_char;

use crate::re::actor_value_info::ActorValueInfo;
use crate::re::actor_values::ActorValue;
use crate::relocation::RelocationID;

const ACTOR_VALUE_COUNT: usize = ActorValue::Total as usize;

/// C++ `RE::ActorValueList`
#[repr(C)]
pub struct ActorValueList {
    pub unk00: u32,                                             // 00
    pub pad04: u32,                                             // 04
    pub actor_values: [*mut ActorValueInfo; ACTOR_VALUE_COUNT], // 08
}

const _: () = assert!(core::mem::size_of::<ActorValueList>() == 0x528);
const _: () = assert!(core::mem::offset_of!(ActorValueList, actor_values) == 0x08);

impl ActorValueList {
    crate::relocation_variable! {
        fn singleton_ptr() -> &'static *mut ActorValueList => RelocationID::new(514139, 400267)
    }

    #[inline(always)]
    pub fn get_singleton() -> *mut ActorValueList {
        *Self::singleton_ptr()
    }

    crate::relocation_func! {
        pub fn get_actor_value_info(actor_value: ActorValue) -> *mut ActorValueInfo => RelocationID::new(26569, 27202)
    }

    crate::relocation_func! {
        pub fn lookup_actor_value_by_name(enum_name: *const c_char) -> ActorValue => RelocationID::new(26570, 27203)
    }

    crate::relocation_func! {
        pub fn get_actor_value_name(actor_value: ActorValue) -> *const c_char => RelocationID::new(26561, 27192)
    }

    #[inline]
    pub fn get_actor_value_name_as_str(actor_value: ActorValue) -> &'static str {
        let name = Self::get_actor_value_name(actor_value);
        if name.is_null() {
            "None"
        } else {
            core_util::ptr_to_str(name)
        }
    }
}

#[inline]
pub fn actor_value_to_string(actor_value: ActorValue) -> &'static str {
    ActorValueList::get_actor_value_name_as_str(actor_value)
}
