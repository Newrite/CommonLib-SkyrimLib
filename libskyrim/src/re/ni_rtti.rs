use core::ffi::c_char;

/// C++ `RE::NiRTTI`
#[repr(C)]
pub struct NiRTTI {
    pub name: *const c_char,      // 00
    pub base_rtti: *const NiRTTI, // 08
}

const _: () = assert!(core::mem::size_of::<NiRTTI>() == 0x10);
const _: () = assert!(core::mem::offset_of!(NiRTTI, name) == 0x00);
const _: () = assert!(core::mem::offset_of!(NiRTTI, base_rtti) == 0x08);

impl NiRTTI {
    #[inline(always)]
    pub const fn get_name(&self) -> *const c_char {
        self.name
    }

    #[inline(always)]
    pub const fn get_base_rtti(&self) -> *const NiRTTI {
        self.base_rtti
    }

    #[inline]
    pub fn is_kind_of(&self, rtti: *const NiRTTI) -> bool {
        let mut iter = self as *const Self;
        while !iter.is_null() {
            if iter == rtti {
                return true;
            }
            unsafe {
                iter = (*iter).get_base_rtti();
            }
        }
        false
    }
}
