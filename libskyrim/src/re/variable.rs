use core::mem::ManuallyDrop;

use crate::re::{Array, BSFixedString, BSTSmartPointer, Object, TypeInfo};

/// C++ `RE::BSScript::Variable::Value`
#[repr(C)]
pub union Value {
    pub i: i32,                                     // 00
    pub u: u32,                                     // 00
    pub f: f32,                                     // 00
    pub b: bool,                                    // 00
    pub p: *mut core::ffi::c_void,                  // 00
    pub arr: ManuallyDrop<BSTSmartPointer<Array>>,  // 00
    pub obj: ManuallyDrop<BSTSmartPointer<Object>>, // 00
    pub str_: ManuallyDrop<BSFixedString>,          // 00
}

const _: () = assert!(core::mem::size_of::<Value>() == 0x8);

/// C++ `RE::BSScript::Variable`
#[repr(C)]
pub struct Variable {
    pub var_type: TypeInfo, // 00
    pub value: Value,       // 08
}

const _: () = assert!(core::mem::size_of::<Variable>() == 0x10);
const _: () = assert!(core::mem::offset_of!(Variable, var_type) == 0x00);
const _: () = assert!(core::mem::offset_of!(Variable, value) == 0x08);

impl Default for Variable {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for Variable {
    fn clone(&self) -> Self {
        let mut out = Self::new();
        out.assign(self);
        out
    }
}

impl Drop for Variable {
    fn drop(&mut self) {
        self.cleanup();
        unsafe {
            core::ptr::write_bytes(self as *mut Self, 0, 1);
        }
    }
}

impl Variable {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            var_type: TypeInfo::new(crate::re::type_info::RawType::None),
            value: Value {
                p: core::ptr::null_mut(),
            },
        }
    }

    #[inline(always)]
    pub fn with_type(type_: TypeInfo) -> Self {
        Self {
            var_type: type_,
            value: Value {
                p: core::ptr::null_mut(),
            },
        }
    }

    #[inline(always)]
    pub fn is_type(&self, type_info: TypeInfo) -> bool {
        self.var_type == type_info
    }

    #[inline(always)]
    pub fn is_array(&self) -> bool {
        self.var_type.is_array()
    }

    #[inline(always)]
    pub fn is_bool(&self) -> bool {
        self.var_type.is_bool()
    }

    #[inline(always)]
    pub fn is_float(&self) -> bool {
        self.var_type.is_float()
    }

    #[inline(always)]
    pub fn is_int(&self) -> bool {
        self.var_type.is_int()
    }

    #[inline(always)]
    pub fn is_literal_array(&self) -> bool {
        self.var_type.is_literal_array()
    }

    #[inline(always)]
    pub fn is_none_array(&self) -> bool {
        self.var_type.is_none_array()
    }

    #[inline(always)]
    pub fn is_none_object(&self) -> bool {
        self.var_type.is_none_object()
    }

    #[inline(always)]
    pub fn is_object(&self) -> bool {
        self.var_type.is_object()
    }

    #[inline(always)]
    pub fn is_object_array(&self) -> bool {
        self.var_type.is_object_array()
    }

    #[inline(always)]
    pub fn is_string(&self) -> bool {
        self.var_type.is_string()
    }

    #[inline(always)]
    pub fn get_type(&self) -> TypeInfo {
        self.var_type
    }

    #[inline(always)]
    pub fn get_sint(&self) -> i32 {
        debug_assert!(self.is_int());
        unsafe { self.value.i }
    }

    #[inline(always)]
    pub fn get_uint(&self) -> u32 {
        debug_assert!(self.is_int());
        unsafe { self.value.u }
    }

    #[inline(always)]
    pub fn get_float(&self) -> f32 {
        debug_assert!(self.is_float());
        unsafe { self.value.f }
    }

    #[inline(always)]
    pub fn get_bool(&self) -> bool {
        debug_assert!(self.is_bool());
        unsafe { self.value.b }
    }

    #[inline(always)]
    pub fn get_array(&self) -> BSTSmartPointer<Array> {
        debug_assert!(self.is_array() || self.is_none_array());
        unsafe {
            (&*(core::ptr::addr_of!(self.value.arr) as *const BSTSmartPointer<Array>)).clone()
        }
    }

    #[inline(always)]
    pub fn get_object(&self) -> BSTSmartPointer<Object> {
        debug_assert!(self.is_object() || self.is_none_object());
        unsafe {
            (&*(core::ptr::addr_of!(self.value.obj) as *const BSTSmartPointer<Object>)).clone()
        }
    }

    #[inline(always)]
    pub fn get_string(&self) -> BSFixedString {
        debug_assert!(self.is_string());
        unsafe { (&*(core::ptr::addr_of!(self.value.str_) as *const BSFixedString)).clone() }
    }

    #[inline(always)]
    pub fn set_none(&mut self) {
        self.change_type(crate::re::type_info::RawType::None);
    }

    #[inline(always)]
    pub fn set_sint(&mut self, value: i32) {
        self.change_type(crate::re::type_info::RawType::Int);
        self.value = Value { i: value };
    }

    #[inline(always)]
    pub fn set_uint(&mut self, value: u32) {
        self.change_type(crate::re::type_info::RawType::Int);
        self.value = Value { u: value };
    }

    #[inline(always)]
    pub fn set_float(&mut self, value: f32) {
        self.change_type(crate::re::type_info::RawType::Float);
        self.value = Value { f: value };
    }

    #[inline(always)]
    pub fn set_bool(&mut self, value: bool) {
        self.change_type(crate::re::type_info::RawType::Bool);
        self.value = Value { b: value };
    }

    #[inline(always)]
    pub fn set_array(&mut self, value: BSTSmartPointer<Array>) {
        debug_assert!(!value.is_null());
        let type_ = unsafe { (*value.get()).type_() };
        self.change_type_set(type_);
        self.value = Value {
            arr: ManuallyDrop::new(value),
        };
    }

    #[inline(always)]
    pub fn set_object(&mut self, value: BSTSmartPointer<Object>) {
        debug_assert!(!value.is_null());
        let type_ = unsafe { (*(*value.get()).get_type_info()).get_raw_type() };
        self.change_type_set(type_);
        self.value = Value {
            obj: ManuallyDrop::new(value),
        };
    }

    #[inline(always)]
    pub fn set_object_with_type(
        &mut self,
        value: BSTSmartPointer<Object>,
        type_: crate::re::type_info::RawType,
    ) {
        debug_assert!(!value.is_null());
        self.change_type(type_);
        self.value = Value {
            obj: ManuallyDrop::new(value),
        };
    }

    #[inline(always)]
    pub fn set_string(&mut self, value: &str) {
        self.change_type(crate::re::type_info::RawType::String);
        self.value = Value {
            str_: ManuallyDrop::new(BSFixedString::from_str(value)),
        };
    }

    #[inline(always)]
    fn change_type(&mut self, type_: crate::re::type_info::RawType) {
        self.cleanup();
        self.var_type = TypeInfo::new(type_);
        self.value = Value {
            p: core::ptr::null_mut(),
        };
    }

    #[inline(always)]
    fn change_type_set(&mut self, type_: core_util::EnumSet<crate::re::type_info::RawType, usize>) {
        self.cleanup();
        self.var_type = TypeInfo { raw_type: type_ };
        self.value = Value {
            p: core::ptr::null_mut(),
        };
    }

    #[inline(always)]
    fn cleanup(&mut self) {
        use crate::re::type_info::RawType;

        match self.var_type.get_unmangled_raw_type() {
            RawType::Object => unsafe { ManuallyDrop::drop(&mut self.value.obj) },
            RawType::String => unsafe { ManuallyDrop::drop(&mut self.value.str_) },
            RawType::None | RawType::Int | RawType::Float | RawType::Bool => {}
            RawType::NoneArray => {}
            RawType::ObjectArray
            | RawType::StringArray
            | RawType::IntArray
            | RawType::FloatArray
            | RawType::BoolArray => unsafe { ManuallyDrop::drop(&mut self.value.arr) },
            RawType::ArraysEnd => {}
        }
    }

    #[inline(always)]
    fn assign(&mut self, rhs: &Variable) {
        use crate::re::type_info::RawType;

        self.var_type = rhs.var_type;
        match rhs.var_type.get_unmangled_raw_type() {
            RawType::Object => {
                self.value = Value {
                    obj: ManuallyDrop::new(rhs.get_object()),
                };
            }
            RawType::String => {
                self.value = Value {
                    str_: ManuallyDrop::new(rhs.get_string()),
                };
            }
            RawType::None | RawType::Int | RawType::Float | RawType::Bool | RawType::NoneArray => unsafe {
                self.value = Value { p: rhs.value.p };
            },
            RawType::ObjectArray
            | RawType::StringArray
            | RawType::IntArray
            | RawType::FloatArray
            | RawType::BoolArray => {
                self.value = Value {
                    arr: ManuallyDrop::new(rhs.get_array()),
                };
            }
            RawType::ArraysEnd => {
                self.value = Value {
                    p: core::ptr::null_mut(),
                };
            }
        }
    }
}
