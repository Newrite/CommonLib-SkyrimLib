#![allow(non_camel_case_types)]

use core::marker::PhantomData;

use crate::re::{GRefCountBaseStatImpl, GRefCountWeakSupportImpl};

/// C++ `RE::GRefCountBaseWeakSupport<T, STAT>`
#[repr(C)]
pub struct GRefCountBaseWeakSupport<T, const STAT: u32> {
    pub base: GRefCountBaseStatImpl<GRefCountWeakSupportImpl, STAT>, // 00
    pub _marker: PhantomData<fn() -> T>,
}

const _: () = assert!(core::mem::size_of::<GRefCountBaseWeakSupport<(), 0>>() == 0x18);
const _: () = assert!(core::mem::offset_of!(GRefCountBaseWeakSupport<(), 0>, base) == 0x0);

core_util::inherit!(for[T, const STAT: u32] GRefCountBaseWeakSupport<T, STAT> : GRefCountBaseStatImpl<GRefCountWeakSupportImpl, STAT>, base);

impl<T, const STAT: u32> GRefCountBaseWeakSupport<T, STAT> {
    pub const STAT_TYPE: u32 = STAT;
}
