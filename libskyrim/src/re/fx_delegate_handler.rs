use crate::ffi::{commonlib_fx_delegate_handler_add_ref, commonlib_fx_delegate_handler_release};
use crate::offsets::offsets_rtti::{
    RTTI_FxDelegateHandler, RTTI_FxDelegateHandler__CallbackProcessor,
};
use crate::offsets::offsets_vtable::{
    VTABLE_FxDelegateHandler, VTABLE_FxDelegateHandler__CallbackProcessor,
};
use crate::re::GPtrTarget;
use crate::relocation::{RttiType, VariantID};

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

    // TODO: `CallbackProcessor::Process(const GString&, CallbackFn*)` depends on
    // source-backed `GString` / `FxDelegateArgs` translation. Keep this nested
    // type as the honest ABI layout until that Scaleform dependency chain lands.
}

/// C++ `RE::FxDelegateHandler`
#[repr(C)]
pub struct FxDelegateHandler {
    pub vtable: *const usize, // 00
    pub ref_count: u32,       // 08
    pub pad0c: u32,           // 0C
}

const _: () = assert!(core::mem::size_of::<FxDelegateHandler>() == 0x10);
const _: () = assert!(core::mem::offset_of!(FxDelegateHandler, vtable) == 0x0);
const _: () = assert!(core::mem::offset_of!(FxDelegateHandler, ref_count) == 0x8);
const _: () = assert!(core::mem::offset_of!(FxDelegateHandler, pad0c) == 0xC);

impl RttiType for FxDelegateHandler {
    const RTTI: VariantID = RTTI_FxDelegateHandler;
}

impl FxDelegateHandler {
    pub const RTTI: VariantID = RTTI_FxDelegateHandler;
    pub const VTABLE: &'static [VariantID] = &VTABLE_FxDelegateHandler;

    // TODO: `FxDelegateHandler::Accept(CallbackProcessor*)` is source-backed,
    // but a faithful Rust signature needs the translated Scaleform callback
    // chain. Keep the exact ABI layout here for `IMenu` inheritance first.
}

impl GPtrTarget for FxDelegateHandler {
    #[inline(always)]
    fn gptr_add_ref(&self) {
        unsafe {
            commonlib_fx_delegate_handler_add_ref(core::ptr::from_ref(self).cast_mut().cast());
        }
    }

    #[inline(always)]
    fn gptr_release(&self) {
        unsafe {
            commonlib_fx_delegate_handler_release(core::ptr::from_ref(self).cast_mut().cast());
        }
    }
}
