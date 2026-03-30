#![allow(non_camel_case_types)]

use core::marker::PhantomData;

use crate::re::GNewOverrideBase;

/// C++ `RE::GRefCountBaseGC<STAT>`
#[repr(C)]
pub struct GRefCountBaseGC<const STAT: u32> {
    pub base: GNewOverrideBase<STAT>,  // 00
    pub vtable: *const usize,          // 00
    pub unk08: *mut core::ffi::c_void, // 08
    pub ref_count: u32,                // 10
    pub pad14: u32,                    // 14
    pub unk18: u64,                    // 18
    pub _marker: PhantomData<[u8; 0]>,
}

const _: () = assert!(core::mem::size_of::<GRefCountBaseGC<0>>() == 0x20);
const _: () = assert!(core::mem::offset_of!(GRefCountBaseGC<0>, vtable) == 0x0);
const _: () = assert!(core::mem::offset_of!(GRefCountBaseGC<0>, unk08) == 0x8);
const _: () = assert!(core::mem::offset_of!(GRefCountBaseGC<0>, ref_count) == 0x10);

core_util::inherit!(for[const STAT: u32] GRefCountBaseGC<STAT> => GNewOverrideBase<STAT>, base);

impl<const STAT: u32> GRefCountBaseGC<STAT> {
    pub const STAT_TYPE: u32 = STAT;

    crate::virtual_method! { pub const VFUNC_UNK_00: usize = 0x00; pub fn unk_00() }
    crate::virtual_method! { pub const VFUNC_UNK_01: usize = 0x01; pub fn unk_01() }

    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x02;
        pub fn dtor()
    }
}
