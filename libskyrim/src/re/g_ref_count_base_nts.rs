#![allow(non_camel_case_types)]

use core::marker::PhantomData;

use core_util::inherit;

use crate::re::{GRefCountBaseStatImpl, GRefCountNTSImpl};

/// C++ `RE::GRefCountBaseNTS<C, Stat>`
#[repr(C)]
pub struct GRefCountBaseNTS<C, const STAT: u32> {
    pub base: GRefCountBaseStatImpl<GRefCountNTSImpl, STAT>, // 00
    pub _marker: PhantomData<fn() -> C>,
}

const _: () = assert!(core::mem::size_of::<GRefCountBaseNTS<(), 0>>() == 0x10);
const _: () = assert!(core::mem::offset_of!(GRefCountBaseNTS<(), 0>, base) == 0x0);

inherit!(for[C, const STAT: u32] GRefCountBaseNTS<C, STAT> : GRefCountBaseStatImpl<GRefCountNTSImpl, STAT>, base);

impl<C, const STAT: u32> GRefCountBaseNTS<C, STAT> {
    pub const STAT_TYPE: u32 = STAT;
}
