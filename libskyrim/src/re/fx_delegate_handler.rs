use crate::offsets::offsets_rtti::{
    RTTI_FxDelegateHandler, RTTI_FxDelegateHandler__CallbackProcessor,
};
use crate::offsets::offsets_vtable::{
    VTABLE_FxDelegateHandler, VTABLE_FxDelegateHandler__CallbackProcessor,
};
use core_util::inherit;

use crate::re::{FxDelegateArgs, GPtrTarget, GRefCountBase, GStatGroups, GString};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::FxDelegateHandler::CallbackFn`
pub type FxDelegateHandlerCallbackFn = unsafe extern "C" fn(params: *const FxDelegateArgs);

/// C++ `RE::FxDelegateHandler::CallbackProcessor`
#[repr(C)]
pub struct FxDelegateHandlerCallbackProcessor {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<FxDelegateHandlerCallbackProcessor>() == 0x8);
const _: () = assert!(core::mem::offset_of!(FxDelegateHandlerCallbackProcessor, vtable) == 0x0);

impl RttiType for FxDelegateHandlerCallbackProcessor {
    const RTTI: VariantID = RTTI_FxDelegateHandler__CallbackProcessor;
}

impl FxDelegateHandlerCallbackProcessor {
    pub const RTTI: VariantID = RTTI_FxDelegateHandler__CallbackProcessor;
    pub const VTABLE: &'static [VariantID] = &VTABLE_FxDelegateHandler__CallbackProcessor;

    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_PROCESS: usize = 0x01;
        pub fn process(&mut self, method_name: &GString, method: Option<FxDelegateHandlerCallbackFn>)
    }
}

/// C++ `RE::FxDelegateHandler`
#[repr(C)]
pub struct FxDelegateHandler {
    pub base: GRefCountBase<FxDelegateHandler, { GStatGroups::kGStat_Default_Mem as u32 }>, // 00
}

const _: () = assert!(core::mem::size_of::<FxDelegateHandler>() == 0x10);
const _: () = assert!(core::mem::offset_of!(FxDelegateHandler, base) == 0x0);

impl RttiType for FxDelegateHandler {
    const RTTI: VariantID = RTTI_FxDelegateHandler;
}

inherit!(FxDelegateHandler : GRefCountBase<FxDelegateHandler, { GStatGroups::kGStat_Default_Mem as u32 }>, base);

impl FxDelegateHandler {
    pub const RTTI: VariantID = RTTI_FxDelegateHandler;
    pub const VTABLE: &'static [VariantID] = &VTABLE_FxDelegateHandler;

    // override (GRefCountImpl)
    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_ACCEPT: usize = 0x01;
        pub fn accept(&mut self, cb_reg: *mut FxDelegateHandlerCallbackProcessor)
    }
}

impl GPtrTarget for FxDelegateHandler {
    #[inline(always)]
    fn gptr_add_ref(&self) {
        unsafe {
            (*core::ptr::from_ref(self).cast_mut()).base.add_ref();
        }
    }

    #[inline(always)]
    fn gptr_release(&self) {
        unsafe {
            (*core::ptr::from_ref(self).cast_mut()).base.release();
        }
    }
}
