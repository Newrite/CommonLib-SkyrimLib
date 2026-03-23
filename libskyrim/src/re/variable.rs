use core::ffi::c_void;

use crate::re::type_info::TypeInfo;

/// C++ `RE::BSScript::Variable::Value`
#[repr(C)]
#[derive(Clone, Copy)]
pub union Value {
    pub i: i32,         // 00
    pub u: u32,         // 00
    pub f: f32,         // 00
    pub b: bool,        // 00
    pub p: *mut c_void, // 00
    pub bits: u64,      // 00
}

const _: () = assert!(core::mem::size_of::<Value>() == 0x8);

/// C++ `RE::BSScript::Variable`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Variable {
    pub var_type: TypeInfo, // 00
    pub value: Value,       // 08
}

const _: () = assert!(core::mem::size_of::<Variable>() == 0x10);

impl Default for Variable {
    #[inline(always)]
    fn default() -> Self {
        Self {
            var_type: TypeInfo::default(),
            value: Value { bits: 0 },
        }
    }
}
