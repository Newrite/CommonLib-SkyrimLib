#![allow(non_camel_case_types)]

use core::marker::PhantomData;

/// C++ `RE::GNewOverrideBase<Stat>`
#[repr(C)]
pub struct GNewOverrideBase<const STAT: u32> {
    pub _marker: PhantomData<fn() -> ()>,
}

const _: () = assert!(core::mem::size_of::<GNewOverrideBase<0>>() == 0x0);

impl<const STAT: u32> GNewOverrideBase<STAT> {
    pub const kStatType: u32 = STAT;

    // TODO: `GNewOverrideBase.h` exists specifically to override C++ `new` /
    // `delete` through the Scaleform heap macros. Rust keeps only the
    // zero-sized layout and stat constant here because this repo still lacks a
    // source-backed Rust-side construction/deletion surface for those macro
    // families. Keep consumers on relocated engine factories or ABI bridges
    // instead of inventing direct Rust allocation for these bases.
}
