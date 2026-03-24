use core::ffi::c_char;

use crate::re::NiObject;

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

/// RTTI-backed downcast helper mirroring CommonLib's `netimmerse_cast` intent.
///
/// # Safety
/// `from` must be null or a valid pointer to a live `NiObject`-derived instance.
/// `to_rtti` must be null or point to a valid NetImmerse RTTI object.
#[inline]
pub unsafe fn netimmerse_cast<To>(from: *const NiObject, to_rtti: *const NiRTTI) -> *mut To {
    if from.is_null() || to_rtti.is_null() {
        return core::ptr::null_mut();
    }

    let from_rtti = unsafe { (*from).get_rtti() };
    if from_rtti.is_null() || unsafe { !(*from_rtti).is_kind_of(to_rtti) } {
        core::ptr::null_mut()
    } else {
        from.cast_mut().cast::<To>()
    }
}

/// Const-preserving RTTI-backed cast helper mirroring CommonLib's const overload.
///
/// # Safety
/// `from` must be null or a valid pointer to a live `NiObject`-derived instance.
/// `to_rtti` must be null or point to a valid NetImmerse RTTI object.
#[inline]
pub unsafe fn netimmerse_cast_const<To>(
    from: *const NiObject,
    to_rtti: *const NiRTTI,
) -> *const To {
    unsafe { netimmerse_cast::<To>(from, to_rtti).cast_const() }
}
