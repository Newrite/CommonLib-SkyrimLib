#![allow(non_camel_case_types)]

use core::ffi::c_void;

use core_util::inherit;

use crate::re::{
    GArray, GArrayConstPolicy, GAtomicInt, GLock, GNewOverrideBase, GPtrTarget, GRefCountBase,
    GStatGroups,
};

pub type GWaitableWaitHandler = unsafe extern "C" fn(*mut c_void);

/// C++ `RE::GWaitable::HandlerStruct`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct GWaitableHandlerStruct {
    pub handler: Option<GWaitableWaitHandler>, // 00
    pub user_data: *mut c_void,                // 08
}

const _: () = assert!(core::mem::size_of::<GWaitableHandlerStruct>() == 0x10);

pub type GWaitableHandlerArraySizePolicy = GArrayConstPolicy<0, 16, true>;
pub type GWaitableHandlerArrayType = GArray<
    GWaitableHandlerStruct,
    { GStatGroups::DEFAULT_MEM as u32 },
    GWaitableHandlerArraySizePolicy,
>;

/// C++ `RE::GWaitable::HandlerArray`
#[repr(C)]
pub struct GWaitableHandlerArray {
    pub base: GNewOverrideBase<{ GStatGroups::DEFAULT_MEM as u32 }>, // 00
    pub ref_count: GAtomicInt<i32>,                                  // 00
    pub pad04: u32,                                                  // 04
    pub handlers: GWaitableHandlerArrayType,                         // 08
    pub handlers_lock: GLock,                                        // 20
}

const _: () = assert!(core::mem::size_of::<GWaitableHandlerArray>() == 0x48);
const _: () = assert!(core::mem::offset_of!(GWaitableHandlerArray, ref_count) == 0x0);
const _: () = assert!(core::mem::offset_of!(GWaitableHandlerArray, handlers) == 0x8);
const _: () = assert!(core::mem::offset_of!(GWaitableHandlerArray, handlers_lock) == 0x20);

inherit!(GWaitableHandlerArray => GNewOverrideBase<{ GStatGroups::DEFAULT_MEM as u32 }>, base);

/// C++ `RE::GWaitable`
#[repr(C)]
pub struct GWaitable {
    pub base: GRefCountBase<GWaitable, { GStatGroups::DEFAULT_MEM as u32 }>, // 00
    pub handlers: *mut GWaitableHandlerArray,                                // 10
}

const _: () = assert!(core::mem::size_of::<GWaitable>() == 0x18);
const _: () = assert!(core::mem::offset_of!(GWaitable, base) == 0x0);
const _: () = assert!(core::mem::offset_of!(GWaitable, handlers) == 0x10);

inherit!(GWaitable : GRefCountBase<GWaitable, { GStatGroups::DEFAULT_MEM as u32 }>, base);

impl GWaitable {
    // override (GRefCountImpl)
    crate::virtual_method! { pub const VFUNC_DTOR: usize = 0x00; pub fn dtor() }
}

impl GPtrTarget for GWaitable {
    #[inline(always)]
    fn gptr_add_ref(&self) {
        unsafe {
            (*(core::ptr::from_ref(self).cast_mut())).base.add_ref();
        }
    }

    #[inline(always)]
    fn gptr_release(&self) {
        unsafe {
            (*(core::ptr::from_ref(self).cast_mut())).base.release();
        }
    }
}
