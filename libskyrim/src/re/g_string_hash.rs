#![allow(non_camel_case_types)]

use core::marker::PhantomData;

use crate::re::GAllocatorGH;

/// Minimal source-backed partial of C++ `RE::GStringHash<U, Allocator>`.
#[repr(C)]
pub struct GStringHash<U, Allocator = GAllocatorGH<U>> {
    // TODO: `GStringHash.h` derives from `GHash<...>` and stores a `Container`
    // member at offset 0. The current translation only needs the concrete
    // 8-byte storage footprint used by `GASStringManager`. Restore the typed
    // `GHash`/`GHashSet` chain once that reusable container surface is
    // translated.
    pub hash: *mut core::ffi::c_void, // 00
    pub _marker: PhantomData<fn() -> (U, Allocator)>,
}

const _: () = assert!(core::mem::size_of::<GStringHash<*mut core::ffi::c_void>>() == 0x8);
const _: () = assert!(core::mem::offset_of!(GStringHash<*mut core::ffi::c_void>, hash) == 0x0);
