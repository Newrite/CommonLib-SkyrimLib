#![allow(non_camel_case_types)]

use core::marker::PhantomData;

/// C++ `RE::GFxLogBase<Derived>`
#[repr(C)]
pub struct GFxLogBase<Derived> {
    pub vtable: *const usize, // 00
    pub _marker: PhantomData<fn() -> Derived>,
}

const _: () = assert!(core::mem::size_of::<GFxLogBase<()>>() == 0x8);
const _: () = assert!(core::mem::offset_of!(GFxLogBase<()>, vtable) == 0x0);

impl<Derived> GFxLogBase<Derived> {
    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    crate::virtual_method! {
        pub const VFUNC_IS_VERBOSE_ACTION_ERRORS: usize = 0x01;
        pub fn is_verbose_action_errors() -> bool
    }
}
