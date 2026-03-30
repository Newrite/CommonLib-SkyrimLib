#![allow(non_camel_case_types)]

use core::marker::PhantomData;

use core_util::inherit;

use crate::re::{GRefCountBaseStatImpl, GRefCountImpl};

/// C++ `RE::GRefCountBase<T, STAT>`
#[repr(C)]
pub struct GRefCountBase<T, const STAT: u32> {
    pub base: GRefCountBaseStatImpl<GRefCountImpl, STAT>, // 00
    pub _marker: PhantomData<fn() -> T>,
}

const _: () = assert!(core::mem::size_of::<GRefCountBase<(), 0>>() == 0x10);
const _: () = assert!(core::mem::offset_of!(GRefCountBase<(), 0>, base) == 0x0);

inherit!(for[T, const STAT: u32] GRefCountBase<T, STAT> : GRefCountBaseStatImpl<GRefCountImpl, STAT>, base);

impl<T, const STAT: u32> GRefCountBase<T, STAT> {
    pub const STAT_TYPE: u32 = STAT;

    // TODO: `GRefCountBase.h` inherits the `GRefCountBaseStatImpl` heap
    // override semantics. Rust intentionally keeps only the honest layout and
    // stat constant here until there is a source-backed allocation/delete path
    // for direct construction of these Scaleform refcount bases.
}
