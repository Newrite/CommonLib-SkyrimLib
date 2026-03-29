#![allow(non_camel_case_types)]

use core_util::inherit;

use crate::re::{GRefCountNTSImpl, GWeakPtrProxy};

/// C++ `RE::GRefCountWeakSupportImpl`
#[repr(C)]
pub struct GRefCountWeakSupportImpl {
    pub base: GRefCountNTSImpl,         // 00
    pub weak_proxy: *mut GWeakPtrProxy, // 10
}

const _: () = assert!(core::mem::size_of::<GRefCountWeakSupportImpl>() == 0x18);
const _: () = assert!(core::mem::offset_of!(GRefCountWeakSupportImpl, base) == 0x0);
const _: () = assert!(core::mem::offset_of!(GRefCountWeakSupportImpl, weak_proxy) == 0x10);

inherit!(GRefCountWeakSupportImpl : GRefCountNTSImpl, base);
